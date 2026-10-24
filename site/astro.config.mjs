// @ts-check
import casoonPages from '@casoon/pages-theme';
import { defineConfig } from 'astro/config';

// Project page: https://casoon.github.io/renderreport/ — `base` is the GitHub Pages path.
export default defineConfig({
  site: 'https://casoon.github.io/renderreport',
  base: '/renderreport/',
  integrations: [
    casoonPages({
      name: 'renderreport',
      tagline: 'PDF reports from data',
      website: 'https://renderreport.casoon.de',
      description:
        'Data-driven PDF reports from components and theme tokens, rendered by an embedded Typst engine. Rust library, CLI, npm package and Wasm.',
      repo: 'casoon/renderreport',
      version: '0.6.2',
      license: 'MIT',
      mdxComponents: './src/mdx-components.ts',
      packages: [
        { label: 'crates.io', href: 'https://crates.io/crates/renderreport' },
        { label: 'npm', href: 'https://www.npmjs.com/package/@casoon/renderreport' },
        { label: 'docs.rs', href: 'https://docs.rs/renderreport' },
      ],
      docsGroups: {
        'getting-started': 'Getting started',
        concepts: 'Concepts',
        guides: 'Guides',
        reference: 'Reference',
      },
    }),
  ],
});
