//! Markdown: a fixed subset, read line by line.
//!
//! Front matter (`---` with `title`, `subtitle`, `author`, `date`, `lang`),
//! `#`/`##`/`###` headings, paragraphs, `-`/`*`/`+` and `1.` lists (one level
//! of nesting), `>` quotes, `|` tables, fenced code blocks (as plain text),
//! `---` rules and images on a line of their own. Inline marks (`**`, `` ` ``,
//! links) are reduced to their text, everything else passes through.

use serde_json::{json, Value};

use super::{component, plain, section, text_block, Options, Report, Source};

pub(super) fn build(source: &Source, _options: &Options) -> Result<Report, String> {
    let mut report = Report::default();
    let body = front_matter(source.text, &mut report)?;
    if report.title.is_empty() {
        report.title = source.stem.to_string();
    }

    let lines: Vec<&str> = body.lines().collect();
    let mut i = 0;
    while i < lines.len() {
        let line = lines[i];
        let trimmed = line.trim();
        if trimmed.is_empty() {
            i += 1;
        } else if trimmed.starts_with("```") {
            let start = i + 1;
            let end = lines[start..]
                .iter()
                .position(|l| l.trim_start().starts_with("```"))
                .map(|n| start + n)
                .ok_or_else(|| {
                    format!(
                        "markdown: code block opened on line {} is never closed",
                        i + 1
                    )
                })?;
            let code = lines[start..end].join("\n");
            if !code.trim().is_empty() {
                report.components.push(component(
                    "textblock",
                    json!({ "content": code, "font_size": "9pt" }),
                ));
            }
            i = end + 1;
        } else if let Some((level, title)) = heading(trimmed) {
            report.components.push(section(&plain(title), level));
            i += 1;
        } else if is_rule(trimmed) {
            report.components.push(component("divider", json!({})));
            i += 1;
        } else if trimmed.starts_with('>') {
            let end = lines[i..]
                .iter()
                .position(|l| !l.trim_start().starts_with('>'))
                .map_or(lines.len(), |n| i + n);
            let quote: Vec<&str> = lines[i..end]
                .iter()
                .map(|l| l.trim_start().trim_start_matches('>').trim())
                .collect();
            report.components.push(component(
                "quote-block",
                json!({ "quote": plain(&quote.join(" ")) }),
            ));
            i = end;
        } else if is_table_start(&lines, i) {
            let end = lines[i + 2..]
                .iter()
                .position(|l| !l.contains('|'))
                .map_or(lines.len(), |n| i + 2 + n);
            report.components.push(table(&lines[i..end]));
            i = end;
        } else if list_item(line).is_some() {
            let end = lines[i..]
                .iter()
                .position(|l| l.trim().is_empty() || (list_item(l).is_none() && !is_indented(l)))
                .map_or(lines.len(), |n| i + n);
            report.components.push(list(&lines[i..end]));
            i = end;
        } else if let Some((alt, path)) = image_line(trimmed) {
            let name = add_image(&mut report, source, path, i + 1)?;
            report.components.push(component(
                "image",
                json!({ "src": name, "alt": alt, "caption": alt }),
            ));
            i += 1;
        } else {
            let end = lines[i + 1..]
                .iter()
                .position(|l| ends_paragraph(l))
                .map_or(lines.len(), |n| i + 1 + n);
            let paragraph: Vec<&str> = lines[i..end].iter().map(|l| l.trim()).collect();
            report
                .components
                .push(text_block(&plain(&paragraph.join(" "))));
            i = end;
        }
    }
    if report.components.is_empty() {
        return Err("markdown: the file has no content".into());
    }
    Ok(report)
}

/// Reads the front matter into `report` and returns the text after it.
fn front_matter<'a>(text: &'a str, report: &mut Report) -> Result<&'a str, String> {
    let text = text.strip_prefix('\u{feff}').unwrap_or(text);
    let Some(rest) = text
        .strip_prefix("---\n")
        .or_else(|| text.strip_prefix("---\r\n"))
    else {
        return Ok(text);
    };
    let mut offset = 0;
    for line in rest.split_inclusive('\n') {
        if line.trim_end() == "---" {
            return Ok(&rest[offset + line.len()..]);
        }
        if let Some((key, value)) = line.split_once(':') {
            let value = value
                .trim()
                .trim_matches(|c| c == '"' || c == '\'')
                .to_string();
            match key.trim() {
                "title" => report.title = value,
                "subtitle" => report.subtitle = Some(value),
                "author" | "date" | "lang" => {
                    report.metadata.insert(key.trim().into(), value.into());
                }
                _ => {}
            }
        }
        offset += line.len();
    }
    Err("markdown: front matter starts with `---` but is never closed".into())
}

fn heading(line: &str) -> Option<(u32, &str)> {
    let hashes = line.chars().take_while(|&c| c == '#').count();
    let title = line[hashes..].strip_prefix(' ')?;
    (1..=6)
        .contains(&hashes)
        .then(|| (hashes as u32, title.trim()))
}

fn is_rule(line: &str) -> bool {
    let compact: String = line.chars().filter(|c| !c.is_whitespace()).collect();
    compact.len() >= 3
        && ['-', '*', '_']
            .iter()
            .any(|&m| compact.chars().all(|c| c == m))
}

fn is_indented(line: &str) -> bool {
    line.starts_with("  ") || line.starts_with('\t')
}

/// `(indent, numbered, text)` of a list item line.
fn list_item(line: &str) -> Option<(usize, bool, &str)> {
    let indent = line.len() - line.trim_start().len();
    let rest = line.trim_start();
    for marker in ["- ", "* ", "+ "] {
        if let Some(text) = rest.strip_prefix(marker) {
            return (!is_rule(rest)).then_some((indent, false, text.trim()));
        }
    }
    let digits = rest.chars().take_while(char::is_ascii_digit).count();
    let text = rest[digits..]
        .strip_prefix(". ")
        .or_else(|| rest[digits..].strip_prefix(") "))?;
    (digits > 0).then_some((indent, true, text.trim()))
}

fn list(lines: &[&str]) -> Value {
    let (base, numbered) = list_item(lines[0]).map_or((0, false), |(i, n, _)| (i, n));
    let mut items: Vec<Value> = Vec::new();
    for line in lines {
        let item = |text: &str| json!({ "content": plain(text), "children": [] });
        match list_item(line) {
            Some((indent, _, text)) if indent > base && !items.is_empty() => {
                items.last_mut().expect("checked non-empty")["children"]
                    .as_array_mut()
                    .expect("children is an array")
                    .push(item(text));
            }
            Some((_, _, text)) => items.push(item(text)),
            // A wrapped line continues the item before it.
            None => {
                if let Some(last) = items.last_mut() {
                    let has_child = last["children"].as_array().is_some_and(|c| !c.is_empty());
                    let target = if has_child {
                        last["children"]
                            .as_array_mut()
                            .and_then(|c| c.last_mut())
                            .expect("has a child")
                    } else {
                        last
                    };
                    let joined = format!(
                        "{} {}",
                        target["content"].as_str().unwrap_or(""),
                        plain(line.trim())
                    );
                    target["content"] = joined.into();
                }
            }
        }
    }
    component("list", json!({ "items": items, "numbered": numbered }))
}

fn is_table_start(lines: &[&str], i: usize) -> bool {
    let Some(separator) = lines.get(i + 1) else {
        return false;
    };
    let separator = separator.trim();
    lines[i].contains('|')
        && separator.contains('-')
        && separator.contains('|')
        && separator
            .chars()
            .all(|c| matches!(c, '|' | '-' | ':' | ' '))
}

fn split_row(line: &str) -> Vec<String> {
    let line = line.trim();
    let line = line.strip_prefix('|').unwrap_or(line);
    let line = line.strip_suffix('|').unwrap_or(line);
    line.replace("\\|", "\u{0}")
        .split('|')
        .map(|cell| plain(cell.trim()).replace('\u{0}', "|"))
        .collect()
}

fn table(lines: &[&str]) -> Value {
    let header = split_row(lines[0]);
    let align = split_row(lines[1]);
    let columns: Vec<Value> = header
        .iter()
        .enumerate()
        .map(|(n, header)| {
            let marks = align.get(n).map_or("", String::as_str);
            match (marks.starts_with(':'), marks.ends_with(':')) {
                (true, true) => json!({ "header": header, "align": "center" }),
                (false, true) => json!({ "header": header, "align": "right" }),
                _ => json!({ "header": header }),
            }
        })
        .collect();
    let rows: Vec<Vec<String>> = lines[2..]
        .iter()
        .map(|line| {
            let mut cells = split_row(line);
            cells.resize(header.len(), String::new());
            cells
        })
        .collect();
    component("table", json!({ "columns": columns, "rows": rows }))
}

/// `![alt](path)` and nothing else on the line.
fn image_line(line: &str) -> Option<(&str, &str)> {
    let rest = line.strip_prefix("![")?;
    let (alt, tail) = rest.split_once("](")?;
    let path = tail.strip_suffix(')')?;
    (!path.contains(')')).then_some((alt, path.trim()))
}

/// Registers the image file as an asset and returns the name to use as `src`.
fn add_image(
    report: &mut Report,
    source: &Source,
    path: &str,
    line: usize,
) -> Result<String, String> {
    if path.contains("://") {
        return Err(format!(
            "markdown: line {line}: image `{path}` is a URL, only local files are supported"
        ));
    }
    let file = source.dir.join(path);
    if !file.is_file() {
        return Err(format!(
            "markdown: line {line}: image `{path}` not found (looked for {})",
            file.display()
        ));
    }
    let ext = file.extension().and_then(|e| e.to_str()).unwrap_or("png");
    let name = format!("image-{}.{ext}", report.assets.len() + 1);
    report
        .assets
        .insert(name.clone(), file.to_string_lossy().into_owned().into());
    Ok(name)
}

fn ends_paragraph(line: &str) -> bool {
    let trimmed = line.trim();
    trimmed.is_empty()
        || trimmed.starts_with("```")
        || trimmed.starts_with('>')
        || heading(trimmed).is_some()
        || list_item(line).is_some()
}
