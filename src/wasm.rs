//! Wasm bindings for running the engine inside a JS host (e.g. a Cloudflare Worker).
//!
//! No filesystem/system fonts are available in that environment, so the
//! engine is configured to rely solely on the embedded fallback fonts
//! (see `engine::world::FontCache`).

use serde::Deserialize;
use serde_json::json;
use wasm_bindgen::prelude::*;

use crate::components::{Component, WordSearch};
use crate::engine::{Engine, EngineConfig};
use crate::render::RenderRequest;

fn wasm_engine() -> Result<Engine, JsValue> {
    let config = EngineConfig {
        pack_paths: vec![],
        font_paths: vec![],
        use_embedded_fonts: true,
        use_system_fonts: false,
        cache_dir: None,
    };
    Engine::with_config(config).map_err(|e| JsValue::from_str(&format!("engine init failed: {e}")))
}

/// Render an arbitrary [`RenderRequest`] (as JSON) to PDF bytes.
#[wasm_bindgen]
pub fn render(request_json: &str) -> Result<Vec<u8>, JsValue> {
    let request: RenderRequest = serde_json::from_str(request_json)
        .map_err(|e| JsValue::from_str(&format!("invalid RenderRequest JSON: {e}")))?;

    wasm_engine()?
        .render_pdf(&request)
        .map_err(|e| JsValue::from_str(&format!("render failed: {e}")))
}

/// Input parameters for [`render_wordsearch`].
#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
struct WordSearchRequest {
    title: String,
    #[serde(default)]
    description: Option<String>,
    words: Vec<String>,
    /// Optional translation per word, matched by index. Entries that are
    /// `None`/absent stay monolingual — bilingual mode is opt-in per word.
    #[serde(default)]
    translations: Vec<Option<String>>,
    #[serde(default = "default_size")]
    width: usize,
    #[serde(default = "default_size")]
    height: usize,
    #[serde(default = "default_true")]
    allow_diagonal: bool,
    #[serde(default)]
    allow_reverse: bool,
    #[serde(default = "default_seed")]
    seed: u64,
    #[serde(default = "default_columns")]
    columns_word_list: usize,
    /// UI/instruction language: "de", "en", "fr", or "es". Unrecognized codes
    /// fall back to German.
    #[serde(default = "default_language")]
    language: String,
}

fn default_size() -> usize {
    12
}

fn default_true() -> bool {
    true
}

fn default_seed() -> u64 {
    42
}

fn default_columns() -> usize {
    3
}

fn default_language() -> String {
    "de".into()
}

/// Localized copy for the title/explanation page and footer. Unrecognized
/// language codes fall back to German.
struct Copy {
    explanation: &'static str,
    about_title: &'static str,
    about_content: &'static str,
    solution_prefix: &'static str,
    solution_description: &'static str,
    default_description: &'static str,
    footer_prefix: &'static str,
    footer_tagline: &'static str,
}

fn copy_for(language: &str) -> Copy {
    match language {
        "en" => Copy {
            explanation: "How it works: find all the listed words in the letter grid — they can be hidden horizontally, vertically, or diagonally. The solution with highlighted words is on the last page.",
            about_title: "About this PDF",
            about_content: "This puzzle was generated with renderreport — a PDF engine written in Rust with Typst as its rendering core (https://renderreport.casoon.de).",
            solution_prefix: "Solution: ",
            solution_description: "All hidden words are highlighted.",
            default_description: "Find all the hidden words in the letter grid!",
            footer_prefix: "Created with",
            footer_tagline: "Word Search Generator",
        },
        "fr" => Copy {
            explanation: "Comment jouer : trouvez tous les mots dans la grille de lettres – ils peuvent être cachés horizontalement, verticalement ou en diagonale. La solution avec les mots surlignés se trouve sur la dernière page.",
            about_title: "À propos de ce PDF",
            about_content: "Ce puzzle a été généré avec renderreport – un moteur PDF écrit en Rust avec Typst comme noyau de rendu (https://renderreport.casoon.de).",
            solution_prefix: "Solution : ",
            solution_description: "Tous les mots cachés sont surlignés.",
            default_description: "Trouvez tous les mots cachés dans la grille de lettres !",
            footer_prefix: "Créé avec",
            footer_tagline: "Générateur de mots cachés",
        },
        "es" => Copy {
            explanation: "Cómo funciona: encuentra todas las palabras en la cuadrícula de letras – pueden estar ocultas en horizontal, vertical o diagonal. La solución con las palabras resaltadas está en la última página.",
            about_title: "Sobre este PDF",
            about_content: "Este rompecabezas fue generado con renderreport – un motor de PDF escrito en Rust con Typst como núcleo de renderizado (https://renderreport.casoon.de).",
            solution_prefix: "Solución: ",
            solution_description: "Todas las palabras ocultas están resaltadas.",
            default_description: "¡Encuentra todas las palabras ocultas en la cuadrícula de letras!",
            footer_prefix: "Creado con",
            footer_tagline: "Generador de sopa de letras",
        },
        _ => Copy {
            explanation: "So funktioniert's: Finde alle gesuchten Wörter im Buchstabengitter – sie können waagerecht, senkrecht oder diagonal versteckt sein. Die Lösung mit farbig hervorgehobenen Wörtern findest du auf der letzten Seite.",
            about_title: "Über dieses PDF",
            about_content: "Dieses Rätsel wurde mit renderreport erzeugt – einer in Rust geschriebenen PDF-Engine mit Typst als Rendering-Kern (https://renderreport.casoon.de).",
            solution_prefix: "Lösung: ",
            solution_description: "Alle versteckten Wörter sind hervorgehoben.",
            default_description: "Finde alle versteckten Wörter im Buchstabengitter!",
            footer_prefix: "Erstellt mit",
            footer_tagline: "Wortsuchrätsel-Generator",
        },
    }
}

/// Render a complete word search PDF (title/explanation page, puzzle page,
/// solution page) from simple parameters — the puzzle grid itself is always
/// computed by the real [`WordSearch`] engine, never supplied by the caller.
#[wasm_bindgen]
pub fn render_wordsearch(request_json: &str) -> Result<Vec<u8>, JsValue> {
    let req: WordSearchRequest = serde_json::from_str(request_json)
        .map_err(|e| JsValue::from_str(&format!("invalid word search request JSON: {e}")))?;

    let copy = copy_for(&req.language);

    let puzzle = WordSearch::builder()
        .title(req.title.clone())
        .description(
            req.description
                .clone()
                .unwrap_or_else(|| copy.default_description.into()),
        )
        .words(req.words.clone())
        .translations(req.translations.clone())
        .language(req.language.clone())
        .grid_size(req.width, req.height)
        .allow_diagonal(req.allow_diagonal)
        .allow_reverse(req.allow_reverse)
        .seed(req.seed)
        .show_solution(false)
        .columns_word_list(req.columns_word_list)
        .build()
        .map_err(|e| JsValue::from_str(&format!("invalid puzzle parameters: {e}")))?;

    let solution = WordSearch::builder()
        .title(format!("{}{}", copy.solution_prefix, req.title))
        .description(copy.solution_description)
        .words(req.words)
        .translations(req.translations)
        .language(req.language.clone())
        .grid_size(req.width, req.height)
        .allow_diagonal(req.allow_diagonal)
        .allow_reverse(req.allow_reverse)
        .seed(req.seed)
        .show_solution(true)
        .columns_word_list(req.columns_word_list)
        .build()
        .map_err(|e| JsValue::from_str(&format!("invalid solution parameters: {e}")))?;

    let request = RenderRequest {
        template_id: "default".into(),
        pack_id: None,
        title: None,
        subtitle: None,
        theme: None,
        components: vec![
            json!({
                "type": "section",
                "data": { "title": req.title, "level": 2, "content": [] }
            }),
            json!({
                "type": "textblock",
                "data": { "content": copy.explanation }
            }),
            json!({
                "type": "callout",
                "data": {
                    "content": copy.about_content,
                    "callout_type": "info",
                    "title": copy.about_title,
                }
            }),
            json!({ "type": "page-break", "data": {} }),
            json!({ "type": "word-search", "data": puzzle.to_data() }),
            json!({ "type": "page-break", "data": {} }),
            json!({ "type": "word-search", "data": solution.to_data() }),
        ],
        assets: Default::default(),
        metadata: [
            ("footer_prefix".to_string(), copy.footer_prefix.to_string()),
            ("author".to_string(), "renderreport".to_string()),
            (
                "footer_link_url".to_string(),
                "https://renderreport.casoon.de".to_string(),
            ),
            ("footer_tagline".to_string(), copy.footer_tagline.to_string()),
        ]
        .into_iter()
        .collect(),
        page_setup: Default::default(),
    };

    wasm_engine()?
        .render_pdf(&request)
        .map_err(|e| JsValue::from_str(&format!("render failed: {e}")))
}
