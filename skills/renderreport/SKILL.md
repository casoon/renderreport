---
name: renderreport
description: Write a PDF report (audit, status report, findings, KPI summary, table report) as a JSON render request and build it with the renderreport CLI. Use when the user asks for a PDF report or document from data, or wants an existing Markdown, CSV, Lighthouse, SARIF or JUnit file turned into a PDF. Not for editing existing PDFs and not for interactive dashboards.
---

# renderreport

renderreport compiles a JSON render request into a PDF (or PNG/SVG pages). You write the
request; the CLI validates and renders it. Do not guess component names or field names: the
CLI tells you, and `validate` catches what you get wrong.

## When to use

- The user wants a PDF report, summary, audit or findings document.
- Data exists (values, a CSV, a tool report) and should become a readable, paged document.
- The same report layout is needed again with different data (template plus `--data`).

Not for: a single table or a few numbers (answer in text), editing existing PDFs, web pages.

## Route

1. **Plan the report** in one message: audience, sections, which numbers go where. Ask only
   when data is missing.
2. **Read the catalog**: `renderreport list-components` (add `--category` to narrow it).
   Pick the components that fit; prefer a few well-chosen ones over many.
3. **Get the schema of each component you use**: `renderreport schema --component <id>`.
   Read the field names, types and required fields from it.
4. **Write the request** as JSON (shape below).
5. **Validate until it passes**: `renderreport validate report.json`. Each error names the
   component index and the field; fix that field and run it again.
6. **Render**: `renderreport render report.json -o report.pdf`.
7. **Look at it** when layout matters: `renderreport render report.json --format png --pages 2 -o p2.png`
   and open the image. `validate` cannot see an empty page, a bad page break or missing
   content; the preview can.
8. **Deliver** the request JSON and the PDF, and say what is in it.

## Request shape

```json
{
  "template_id": "default",
  "title": "Quarterly Service Review",
  "subtitle": "Q3 2026",
  "metadata": { "author": "Platform Team", "date": "2026-10-05", "lang": "en" },
  "components": [
    { "type": "section", "data": { "title": "Summary", "level": 1 } },
    { "type": "textblock", "data": { "content": "Availability stayed above target." } }
  ]
}
```

- Each component is `{ "type": "<id>", "data": { ... } }`. Fields belong inside `data`.
- With a `title` the report opens with a title page. `metadata.lang` sets the document
  language (default `en`).
- `components` render in order, top to bottom.

## Structure of a good report

1. **Title page**: `title`, `subtitle`, `metadata.author` and `metadata.date`.
2. **Summary first**: a `section` heading, one `textblock` with the conclusion in two or
   three sentences, then a `metric-strip` with the key numbers. A reader who stops here
   should have the answer.
3. **Findings**: one `finding` per item (`severity`: `critical`, `high`, `medium`, `low`,
   `info`), most severe first, each with a `recommendation`.
4. **Details and appendix**: `table` for full data, `list` for enumerations. Put bulk data
   last.

Use `section` for every part, with `level` 1 for parts and 2 or 3 below them. A `section` is
only the heading: put its components after it in the same `components` array. Content nested
in a section's `content` is not rendered. Group side by side with `grid-component`.

Page breaks: start a long part on a new page with `{ "type": "page-break", "data": null }`.
Components marked `keep-together` in the catalog move to the next page as a whole instead of
splitting; `breakable` ones (tables, lists) flow across pages.

## Template and data

For a layout you reuse, write the request once with `{{ placeholders }}` and pass the data:

```json
{ "template_id": "default", "title": "{{ client }} Review", "components": [
  { "type": "metric-strip", "data": { "items": [
    { "for": "m in metrics", "component": { "label": "{{ m.name }}", "value": "{{ m.value }}" } } ] } }
] }
```

```sh
renderreport validate template.json --data data.json
renderreport render template.json --data data.json -o review.pdf
```

`--data` takes JSON or CSV. `validate --data` checks the resolved request.

## From an existing file

`renderreport from <format> <file>` builds a report without a hand-written request. Formats:
`markdown`, `csv`, `lighthouse`, `sarif`, `junit`.

```sh
renderreport from csv sales.csv --title "Sales Q3" -o sales.pdf
renderreport from markdown notes.md --emit-request > request.json
```

`--emit-request` prints the generated request so you can edit it and render it with
`renderreport render`. Use it when the plain conversion is close but not right.

## Typical mistakes

| Symptom (`validate` message) | Cause and fix |
| --- | --- |
| `missing field `content`` | A field name was guessed (`text` instead of `content`). Read `schema --component`. |
| `component `x` has no `data`` | Fields were put next to `type`. Wrap them in `"data": { ... }`. |
| `invalid type: integer `3`, expected a string` | `metric-strip` values are strings: `"3"`, not `3`. Check each type in the schema. |
| `unknown variant `urgent`, expected one of ...` | An enum value outside the list; the message lists the allowed ones. |
| `expected unit struct PageBreak` | Components without fields take `"data": null`, not `{}`. |
| `unknown component type` | Wrong id, or the component belongs to a pack. Pack components exist only with `pack_id` in the request and `--pack-path <dir>` if the pack is not in `./packs`. |
| Valid, but content is missing in the PDF | Content was nested in `section.content`. Move it after the section. |

`validate` reports one error per component at a time; fix, run again, repeat until
`Request is valid`.

## Never

- Invent figures, findings or sources. Use only data the user gave you; mark gaps.
- Hand-edit the PDF or the generated Typst. Change the request and render again.
- Skip `validate`, or call the report done without having rendered it.

## References

- Documentation: <https://casoon.github.io/renderreport/>
- Component list: `renderreport list-components --markdown`
- Request fields: `renderreport schema`
