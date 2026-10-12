import { docsSchema } from '@casoon/pages-theme/content';
import { defineCollection } from 'astro:content';
import { glob } from 'astro/loaders';

// Sources: ../docs (docs/ in the project repository). The upper-case Markdown files at its root
// (API.md, CONVENTIONS.md, …) are repository docs linked from the README, not site pages.
export const collections = {
  docs: defineCollection({
    loader: glob({ pattern: ['**/*.{md,mdx}', '![A-Z]*.md'], base: '../docs' }),
    schema: docsSchema,
  }),
};
