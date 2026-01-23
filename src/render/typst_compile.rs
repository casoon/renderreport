//! Typst compilation wrapper

use crate::engine::world::ReportWorld;
use crate::engine::EngineConfig;
use crate::error::{Error, RenderError};
use crate::render::RenderRequest;

/// Compile Typst source to PDF
pub fn compile_to_pdf(
    source: &str,
    config: &EngineConfig,
    request: &RenderRequest,
) -> crate::Result<Vec<u8>> {
    // Create Typst world
    let world = ReportWorld::new(source.to_string(), config);

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
            // Render to PDF
            let pdf_result = typst_pdf::pdf(&document, &typst_pdf::PdfOptions::default());
            match pdf_result {
                Ok(pdf_bytes) => Ok(pdf_bytes),
                Err(errors) => {
                    let error_messages: Vec<String> =
                        errors.iter().map(|e| format!("{:?}", e)).collect();
                    Err(Error::Render(RenderError::TypstCompilation(
                        error_messages.join("\n"),
                    )))
                }
            }
        }
        Err(errors) => {
            let error_messages: Vec<String> = errors
                .iter()
                .map(|e| format!("{:?}: {}", e.span, e.message))
                .collect();

            Err(Error::Render(RenderError::TypstCompilation(
                error_messages.join("\n"),
            )))
        }
    }
}
