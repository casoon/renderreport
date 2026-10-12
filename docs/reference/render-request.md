---
title: Render request
description: Every field of RenderRequest, the input of the library, the CLI and the Wasm build.
order: 1
---

`RenderRequest` is the one input format. The Rust builder produces it, the CLI and the Wasm
`render` function read it as JSON.

| Field | Type | Default | Purpose |
| --- | --- | --- | --- |
| `template_id` | string | – | Report template; `default` for the built-in one |
| `components` | array | – | `{ "type": "<id>", "data": { … } }` per component, in document order |
| `title` | string | none | Title page and PDF document title |
| `subtitle` | string | none | Line under the title on the title page |
| `pack_id` | string | none | Loaded pack whose default theme is merged in |
| `theme` | object | none | `{ "id", "name", "tokens": { … } }`, see [themes](../../concepts/themes/) |
| `metadata` | object | `{}` | String keys and values, see below |
| `assets` | object | `{}` | Name → file path; files are added to the Typst file system |
| `page_setup` | object | see below | Paper, orientation, margins, header and footer |

With a `title`, the report opens with a title page. Without one, the PDF document title falls
back to the first `cover-page` component's title, then the first `section`'s. PDF/UA-1 requires a
document title, so a report with neither fails to render.

## `metadata`

| Key | Used for |
| --- | --- |
| `author` | Title page and running footer |
| `date` | Title page and running header |
| `lang` | Document language (default `en`); PDF/UA-1 requires one |
| `footer_prefix`, `footer_tagline`, `footer_link_url` | Running footer text and link |

Other keys are kept in the request but not used by the default template.

## `page_setup`

| Field | Default | Values |
| --- | --- | --- |
| `paper` | `a4` | `a4`, `a3`, `letter`, `legal` |
| `orientation` | `portrait` | `portrait`, `landscape` |
| `margin` | theme | A raw Typst value such as `"20pt"` or `"(top: 24pt)"`; overrides the `page.margin*` tokens |
| `show_header` | `true` | Running header on content pages |
| `show_footer` | `true` | Running footer on content pages |
| `header_logo` | none | Asset name of a logo for the header |
