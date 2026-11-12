//! Typst compilation wrapper

use std::sync::Arc;

use typst::diag::{SourceDiagnostic, Tracepoint};
use typst_layout::PagedDocument;

use crate::engine::world::{FontCache, ReportWorld};
use crate::error::{Error, RenderError};
#[cfg(any(feature = "png", feature = "svg"))]
use crate::render::PageFormat;
use crate::render::RenderRequest;

/// Compile Typst source to PDF
pub fn compile_to_pdf(
    source: &str,
    font_cache: &Arc<FontCache>,
    request: &RenderRequest,
) -> crate::Result<Vec<u8>> {
    compile_with_files(source, font_cache, request, std::iter::empty())
}

/// [`compile_to_pdf`] with `files` (path, bytes) added to the virtual
/// filesystem before the request's own assets, which win on a clash.
pub(crate) fn compile_with_files<'a>(
    source: &str,
    font_cache: &Arc<FontCache>,
    request: &RenderRequest,
    files: impl IntoIterator<Item = (&'a str, &'a [u8])>,
) -> crate::Result<Vec<u8>> {
    let document = compile_document(source, font_cache, request, files)?;

    // Render to PDF. `tagged` is already `true` by default (Typst 0.14+
    // writes a baseline structure tree for every PDF), but PDF/UA-1
    // conformance additionally requires enforcing it as a `standards`
    // validator — this makes the compiler hard-fail (with an actionable
    // Typst diagnostic) on any document missing a title, language,
    // outline, or image alt text, instead of silently emitting a
    // non-conformant PDF.
    let standards = typst_pdf::PdfStandards::new(&[typst_pdf::PdfStandard::Ua_1])
        .map_err(|e| Error::Render(RenderError::TypstCompilation(e.message().to_string())))?;
    let options = typst_pdf::PdfOptions {
        standards,
        ..Default::default()
    };
    typst_pdf::pdf(&document, &options).map_err(|errors| {
        Error::Render(RenderError::TypstCompilation(describe_diagnostics(&errors)))
    })
}

/// Compile Typst source to one image per page, in the given format.
#[cfg(any(feature = "png", feature = "svg"))]
pub(crate) fn compile_pages_with_files<'a>(
    source: &str,
    font_cache: &Arc<FontCache>,
    request: &RenderRequest,
    files: impl IntoIterator<Item = (&'a str, &'a [u8])>,
    format: PageFormat,
) -> crate::Result<Vec<Vec<u8>>> {
    let document = compile_document(source, font_cache, request, files)?;
    document
        .pages()
        .iter()
        .map(|page| render_page(page, format))
        .collect()
}

#[cfg(any(feature = "png", feature = "svg"))]
fn render_page(page: &typst_layout::Page, format: PageFormat) -> crate::Result<Vec<u8>> {
    match format {
        #[cfg(feature = "png")]
        PageFormat::Png { ppi } => {
            // Typst measures in points, 72 per inch.
            let options = typst_render::RenderOptions {
                pixel_per_pt: (f64::from(ppi) / 72.0).into(),
                ..Default::default()
            };
            typst_render::render(page, &options)
                .encode_png()
                .map_err(|e| Error::Render(RenderError::TypstCompilation(e.to_string())))
        }
        #[cfg(feature = "svg")]
        PageFormat::Svg => Ok(typst_svg::svg(page, &Default::default()).into_bytes()),
    }
}

/// Compile Typst source to a laid-out document.
fn compile_document<'a>(
    source: &str,
    font_cache: &Arc<FontCache>,
    request: &RenderRequest,
    files: impl IntoIterator<Item = (&'a str, &'a [u8])>,
) -> crate::Result<PagedDocument> {
    // Create Typst world
    let world = ReportWorld::new(source.to_string(), font_cache.clone());

    for (name, bytes) in files {
        world.add_file(name, bytes);
    }

    // Add assets to virtual filesystem
    for (name, path) in &request.assets {
        if let Ok(bytes) = std::fs::read(path) {
            world.add_file(name, bytes);
        }
    }

    typst::compile::<PagedDocument>(&world)
        .output
        .map_err(|errors| {
            Error::Render(RenderError::TypstCompilation(describe_diagnostics(&errors)))
        })
}

/// One line per diagnostic: the message, the component whose template raised
/// it, and Typst's hints — instead of the diagnostic's debug form, whose span
/// ids mean nothing outside this process.
fn describe_diagnostics(diagnostics: &[SourceDiagnostic]) -> String {
    diagnostics
        .iter()
        .map(|diagnostic| {
            let mut line = diagnostic.message.to_string();
            if let Some(component) = failing_component(diagnostic) {
                line.push_str(&format!(" (in component `{component}`)"));
            }
            for hint in &diagnostic.hints {
                line.push_str(&format!("; hint: {}", hint.v));
            }
            line
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// The outermost function call in the diagnostic's trace. The report content
/// calls one component function per component, so for an error inside a
/// template that call names the component.
fn failing_component(diagnostic: &SourceDiagnostic) -> Option<&str> {
    let name = diagnostic
        .trace
        .iter()
        .rev()
        .find_map(|point| match &point.v {
            Tracepoint::Call(Some(name)) => Some(name.as_str()),
            _ => None,
        })?;
    // `image` is emitted as `report-image` to avoid Typst's built-in.
    Some(if name == "report-image" {
        "image"
    } else {
        name
    })
}
