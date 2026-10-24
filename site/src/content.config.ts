import { docsSchema } from '@casoon/pages-theme/content';
import { defineCollection } from 'astro:content';
import { glob } from 'astro/loaders';

// Source: ../docs (docs/ in the project repository).
export const collections = {
  docs: defineCollection({
    loader: glob({ pattern: '**/*.{md,mdx}', base: '../docs' }),
    schema: docsSchema,
  }),
};
