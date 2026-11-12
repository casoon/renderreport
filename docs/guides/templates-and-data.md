---
title: Templates and data
description: Describe a report once as a template and fill it with different data on every run.
order: 7
---

A render request can be a template: it describes the report once, the data comes with each run.

```sh
renderreport render template.json --data march.json -o march.pdf
renderreport render template.json --data april.json -o april.pdf
```

The template is an ordinary [render request](../../reference/render-request/) with three
additions. They are resolved before the request is read, as a plain JSON-to-JSON step; the
engine does not know about them. The complete example is in
[`examples/requests/monthly-report/`](https://github.com/casoon/renderreport/tree/main/examples/requests/monthly-report).

## Placeholders

`{{ path.to.value }}` in any string value is replaced by the value from the data. Path segments
are separated by dots; array elements are addressed by index, `{{ findings.0.title }}`.

```json
{ "title": "{{ client.name }} Monthly Report" }
```

A string that consists of one placeholder only takes the JSON type of the value, so numbers,
arrays and objects reach the component unchanged: `"score": "{{ s.value }}"` becomes the number
`78`. Inside longer text a value is written as text; arrays and objects cannot be, and `null`
becomes empty text. Object keys are not replaced, only values.

Three filters can be chained with `|`:

| Filter | Effect |
| --- | --- |
| `upper`, `lower` | Changes the case of a string |
| `round` | Rounds a number to an integer |

```json
{ "title": "Findings for {{ client.name | upper }}", "score": "{{ s.value | round }}" }
```

There is no arithmetic and no other expression; calculate sums, counts and groups before the
run and pass them in the data.

## Repetition: `for`

As an element of an array, `{"for": "item in list", "component": …}` is replaced by one
`component` per element of `list`. `item` can be used in placeholders inside it.

```json
{
  "for": "f in findings",
  "component": { "type": "finding", "data": { "title": "{{ f.title }}", "severity": "{{ f.severity }}" } }
}
```

`list` must be an array; an empty array produces nothing. The entry works in every array, not
only in `components`, for example in the `items` of a `grid-component`.

## Condition: `if`

`{"if": "path", "component": …}` keeps `component` only if the path exists in the data and is
not `null`, `false`, `0`, `""`, `[]` or `{}`; otherwise it is left out. A missing path is not an
error here, that is the point of the condition. `for` and `if` nest: put one in the `component`
of the other. Use one of them per entry, not both.

## Data files

`--data` reads JSON, or a `.csv` file. CSV is turned into `{"rows": [ … ]}` with one object per
line, keyed by the header line, so a template loops with `"r in rows"`. Fields may be quoted with
`"`; the separator is a comma. A cell that reads back unchanged as a number (`42`, `3.5`) becomes
a number, every other cell, such as `007` or `1.50`, stays text.

In Wasm, `render_with_data(request_json, data_json)` does the same with JSON data, see
[Wasm](../wasm/). In Rust, `renderreport::render::bind::bind(&template, &data)` returns the
resolved request as a `serde_json::Value`.

## Errors and checking

Errors name the place in the template and the data path that is missing:

```text
Error: failed to bind data to the request: components[1].data.items[0].for: data path `scores` not found: no `scores` in the data
```

`renderreport validate template.json --data march.json` binds first and then checks the
resolved request, so it also reports data that does not fit a component. The schema from
`renderreport schema` accepts `for`/`if` entries in `components`, but it describes the resolved
request: a placeholder in a number field, such as `"score": "{{ s.value }}"`, is flagged by an
editor although it resolves correctly.
