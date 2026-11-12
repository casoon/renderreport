---
title: Reports from files
description: Turn a Markdown, CSV, Lighthouse, SARIF or JUnit file into a PDF with one command.
order: 8
---

`renderreport from <format> <file>` builds a report from a file you already have, without
writing a render request first. The command reads the file, assembles a request from the
[components](../../concepts/components/) that fit its content and renders it like
[`renderreport render`](../../reference/cli/) does.

```sh
renderreport from lighthouse report.json -o lighthouse.pdf
renderreport from csv sales.csv --title "Sales Q3" --lang de -o sales.pdf
```

Common options:

| Option | Effect |
| --- | --- |
| `-o, --output` | Output path, default `report.pdf` |
| `--title` | Report title instead of the one the format provides |
| `--lang en\|de` | Language of the labels the report adds (default `en`); the content of the file is never translated |
| `--emit-request` | Write the generated request JSON instead of rendering (to stdout without `-o`) |
| `--format pdf\|png\|svg`, `--pages`, `--ppi` | Page images instead of a PDF, as for `render` |

`--emit-request` is the way to customise a report: take the JSON as a starting point, edit it
and render it with `renderreport render`. Every request the adapters build passes
`renderreport validate`.

An input that is not what the format expects is reported with the format and the missing part,
for example `lighthouse: missing `categories` (is this `lighthouse --output json`?)`.

## Markdown

```sh
renderreport from markdown notes.md -o notes.pdf
```

A fixed subset of Markdown is read; the adapter has no Markdown dependency.

| Markdown | Component |
| --- | --- |
| Front matter between `---` lines: `title`, `subtitle`, `author`, `date`, `lang` | Title page, header and footer, document language |
| `#`, `##`, `###` headings | `section` (levels 1 to 3; up to `######` are accepted) |
| Paragraphs | `textblock` |
| `-`, `*`, `+` and `1.` lists, one level of nesting by indentation | `list` |
| `> ` quotes | `quote-block` |
| Tables with `\|` and a `---` separator line (`:--`, `--:`, `:-:` set the alignment) | `table` |
| Fenced code blocks | `textblock` in a smaller size (plain text, no highlighting) |
| `![alt](path)` on a line of its own | `image`, the path is relative to the Markdown file; URLs are not fetched |
| `---` on a line of its own | `divider` |

Inline marks are reduced to their text: `**bold**` and `` `code` `` lose their marks and
`[text](url)` becomes `text (url)`, because the text components print their content as plain
text. Without a `title` in the front matter (or `--title`) the file name is the title.

## CSV

```sh
renderreport from csv sales.csv --title "Sales Q3" -o sales.pdf
```

The first line is the header; quoting follows the rules of the
[`--data` CSV reader](../templates-and-data/). The report has

- the number of rows and columns,
- for each numeric column (the first six) sum, mean, minimum and maximum, as a `metric-strip`,
- a horizontal bar chart of the first numeric column by the first text column, at most the first
  12 rows, and
- the whole table, numeric columns right-aligned.

A column is numeric when all its non-empty cells read back unchanged as a number; `007` or
`1.50` stay text.

## Lighthouse

```sh
lighthouse https://example.com --output json --output-path report.json
renderreport from lighthouse report.json -o lighthouse.pdf
```

- A `score-card` per category (performance, accessibility, best practices, SEO).
- Core Web Vitals as `metric-card`s: first contentful paint, largest contentful paint, total
  blocking time, cumulative layout shift and speed index, coloured by Lighthouse's score bands.
- A `finding` for every audit with a score below 0.9 that counts towards a category (weight
  above 0), worst first, at most 25. Severity follows the score: below 0.5 `high`, below 0.75
  `medium`, otherwise `low`.

The final URL is the subtitle, the fetch date the report date.

## SARIF

```sh
renderreport from sarif results.sarif -o findings.pdf
```

Static-analysis results in SARIF 2.1.0 (CodeQL, Semgrep, clippy, trivy and others). All runs of
the file are combined.

- A `severity-overview` with the results counted by `level`: error, warning, note, other. A
  result without a level uses the rule's default level, else `warning`.
- A `finding` per result, errors first: rule ID as title, the message, and `file:line` of the
  first location. Errors are shown as critical, warnings as high, notes as medium.
- At most the first 100 findings; a note says how many results were left out.

The tool name and version are the title and subtitle.

## JUnit

```sh
renderreport from junit results.xml -o tests.pdf
```

- A summary line, and the figures tests, passed, failed (failures and errors), skipped and the
  duration.
- A `finding` for each failed test with the message and the start of the stack trace; a
  `failure` is high, an `error` critical. At most 100.
- A table of all test suites with their counts and time.

The XML is read by a minimal scanner for `testsuite`, `testcase`, `failure`, `error` and
`skipped`; nested suites are listed individually. When a file has no `testcase` elements, the
counts come from the `tests`, `failures`, `errors` and `skipped` attributes of the suites.
