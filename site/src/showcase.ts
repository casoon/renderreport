import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { url } from '@casoon/pages-theme/lib/url.ts';
import type { ShowcaseExample } from '@casoon/pages-theme/showcase';

// Everything shown here comes from examples/: each Rust example or JSON request sits in the
// repository next to the PDF renderreport made from it. examples/regenerate.sh rebuilds them.
const sources = import.meta.glob<string>(['../../examples/*.rs', '../../examples/requests/*.json'], {
  query: '?raw',
  import: 'default',
  eager: true,
});

function source(file: string): string {
  const found = sources[`../../${file}`];
  if (found === undefined) throw new Error(`Missing fixture: ${file}`);
  return found;
}

/** The repository root; `astro build` and `astro dev` run in site/. */
const repoDir = resolve(process.cwd(), '..');

/** Published name (/examples/<name>.pdf) → PDF in the repository. */
const pdfs: Record<string, string> = {
  'health-check': 'examples/requests/health-check.pdf',
  'health-check-themed': 'examples/requests/health-check-themed.pdf',
  'simple-report': 'examples/output/simple_report.pdf',
  'dark-report': 'examples/output/dark_report.pdf',
  'word-search': 'examples/output/wordsearch_puzzle.pdf',
  'component-catalog': 'examples/output/component_catalog.pdf',
};

export const pdfNames = Object.keys(pdfs);

export function readPdf(name: string): Buffer {
  const file = pdfs[name];
  if (file === undefined) throw new Error(`Unknown PDF: ${name}`);
  return readFileSync(resolve(repoDir, file));
}

/** Inline PDF preview with open and download links; the PDF is served at /examples/<name>.pdf. */
export function pdfViewer(name: string, label: string): string {
  const href = url(`examples/${name}.pdf`);
  const size = (readPdf(name).length / 1024).toFixed(0);
  return `<object data="${href}" type="application/pdf" title="${label}" style="display:block;width:100%;aspect-ratio:210/297;border:1px solid var(--border);border-radius:8px">
  <p style="margin:0;padding:16px">This browser does not show PDFs inline. Use the links below.</p>
</object>
<p style="margin:12px 0 0;font-size:14px"><a href="${href}">Open ${name}.pdf</a> · <a href="${href}" download>Download ${name}.pdf</a> (${size} KiB)</p>`;
}

const entries = [
  {
    slug: 'health-check',
    title: 'Website health check from JSON',
    file: 'examples/requests/health-check.json',
    lang: 'json',
    tags: ['CLI', 'JSON', 'score cards', 'findings'],
    description:
      'A render request as JSON: title page, score cards in a grid, progress bars, findings with recommendations and a callout. Rendered with renderreport render health-check.json.',
  },
  {
    slug: 'health-check-themed',
    title: 'Same request, brand theme',
    file: 'examples/requests/health-check-themed.json',
    lang: 'json',
    tags: ['CLI', 'theme tokens'],
    description:
      'The health check request plus a theme block that overrides seven tokens: primary and status colours, and the page margin. No component data changed.',
  },
  {
    slug: 'simple-report',
    title: 'Project status report',
    file: 'examples/simple_report.rs',
    lang: 'rust',
    tags: ['Rust', 'grid', 'chart', 'gauge'],
    description:
      'The beginner example of the crate: summary box, score cards, progress bars, a chart, gauges, findings and a key-value list, built with the Rust builders.',
  },
  {
    slug: 'dark-report',
    title: 'Dark theme',
    file: 'examples/custom_theme.rs',
    lang: 'rust',
    tags: ['Rust', 'custom theme'],
    description:
      'custom_theme.rs builds two themes from scratch, Corporate Blue and Dark Mode, and renders a full report with each. This is the dark one.',
  },
  {
    slug: 'word-search',
    title: 'Word search puzzle',
    file: 'examples/wordsearch_demo.rs',
    lang: 'rust',
    tags: ['Rust', 'word-search', 'seeded'],
    description:
      'A 14 × 14 grid generated from ten words with seed 42, so it is the same on every run. Arrows show the allowed reading directions.',
  },
  {
    slug: 'component-catalog',
    title: 'Component catalog',
    file: 'examples/component_catalog.rs',
    lang: 'rust',
    tags: ['Rust', 'all components'],
    description: 'Every built-in component with sample data, grouped by category.',
  },
];

export const examples: ShowcaseExample[] = entries.map(({ slug, title, file, lang, tags, description }) => ({
  slug,
  title,
  description,
  file,
  tags,
  input: { code: source(file), lang },
  output: { html: pdfViewer(slug, `${slug}.pdf, rendered by renderreport`), kind: 'panel' as const },
}));
