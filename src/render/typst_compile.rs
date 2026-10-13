//! Typst compilation wrapper

use std::sync::Arc;

use typst::diag::{SourceDiagnostic, Tracepoint};

use crate::engine::world::{FontCache, ReportWorld};
use crate::error::{Error, RenderError};
use crate::render::RenderRequest;

/// Compile Typst source to PDF
pub fn compile_to_pdf(
    source: &str,
    font_cache: &Arc<FontCache>,
    request: &RenderRequest,
) -> crate::Result<Vec<u8>> {
    // Create Typst world
    let world = ReportWorld::new(source.to_string(), font_cache.clone());

    // Add assets to virtual filesystem
    for (name, path) in &request.assets {
        if let Ok(bytes) = std::fs::read(path) {
            world.add_file(name, bytes);
        }
    }

    // Compile
    let result = typst::compile(&world);

    match result.output {
        Ok(document) => {
            // Render to PDF. `tagged` is already `true` by default (Typst 0.14+
            // writes a baseline structure tree for every PDF), but PDF/UA-1
            // conformance additionally requires enforcing it as a `standards`
            // validator — this makes the compiler hard-fail (with an actionable
            // Typst diagnostic) on any document missing a title, language,
            // outline, or image alt text, instead of silently emitting a
            // non-conformant PDF.
            let standards =
                typst_pdf::PdfStandards::new(&[typst_pdf::PdfStandard::Ua_1]).map_err(|e| {
                    Error::Render(RenderError::TypstCompilation(e.message().to_string()))
                })?;
            let options = typst_pdf::PdfOptions {
                standards,
                ..Default::default()
            };
            let pdf_result = typst_pdf::pdf(&document, &options);
            match pdf_result {
                Ok(pdf_bytes) => Ok(pdf_bytes),
                Err(errors) => Err(Error::Render(RenderError::TypstCompilation(
                    describe_diagnostics(&errors),
                ))),
            }
        }
        Err(errors) => Err(Error::Render(RenderError::TypstCompilation(
            describe_diagnostics(&errors),
        ))),
    }
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
