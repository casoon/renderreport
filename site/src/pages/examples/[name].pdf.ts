import type { APIRoute, GetStaticPaths } from 'astro';
import { pdfNames, readPdf } from '../../showcase';

// Serves the committed PDFs from examples/ at /examples/<name>.pdf, next to their sources.
export const getStaticPaths = (() => pdfNames.map((name) => ({ params: { name } }))) satisfies GetStaticPaths;

export const GET: APIRoute = ({ params }) =>
  new Response(new Uint8Array(readPdf(String(params.name))), {
    headers: { 'Content-Type': 'application/pdf' },
  });
