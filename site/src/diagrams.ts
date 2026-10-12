import { renderChart } from '@casoon/chartlet';

type Spec = Record<string, unknown>;

// Diagrams as chartlet specifications in site/diagrams/, rendered to HTML at build time.
const specs = import.meta.glob<Spec>('../diagrams/*.json', { import: 'default', eager: true });

/**
 * Each diagram is rendered twice — with chartlet's light and its dark theme — and CSS shows the
 * one matching the page: flow charts fill their nodes from role colours, so overriding a few
 * colour variables would leave light text on light nodes.
 */
const SURFACE_STYLE = `<style>.cl-surface{padding:16px;border-radius:10px;margin-block:1.5rem}.cl-surface details{overflow-x:auto}.cl-light{background:#fff;color:#172033;color-scheme:light}.cl-dark{display:none;background:#0e131c;color:#e8edf6;color-scheme:dark}:root[data-theme=dark] .cl-light{display:none}:root[data-theme=dark] .cl-dark{display:block}@media (prefers-color-scheme:dark){:root:not([data-theme=light]) .cl-light{display:none}:root:not([data-theme=light]) .cl-dark{display:block}}</style>`;

/** A diagram from site/diagrams/<name>.json. */
export function diagram(name: string): string {
  const spec = specs[`../diagrams/${name}.json`];
  if (!spec) throw new Error(`unknown diagram: ${name}`);
  const variant = (theme: 'light' | 'dark') => {
    const { content, warnings } = renderChart(
      { ...spec, theme },
      { format: 'html', table: 'details', idPrefix: `${name}-${theme}` },
    );
    for (const warning of warnings) console.warn(`[diagram ${name}, ${theme}] ${warning.trim()}`);
    return `<div class="cl-surface cl-${theme}">${content}</div>`;
  };
  return `${SURFACE_STYLE}${variant('light')}${variant('dark')}`;
}
