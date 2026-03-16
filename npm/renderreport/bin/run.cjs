#!/usr/bin/env node

/**
 * Thin wrapper that executes the platform-specific renderreport binary.
 */

const { execFileSync } = require("child_process");
const path = require("path");
const fs = require("fs");

const binaryName =
  process.platform === "win32" ? "renderreport.exe" : "renderreport";
const binaryPath = path.join(__dirname, binaryName);

if (!fs.existsSync(binaryPath)) {
  console.error(
    `Error: ${binaryName} not found at ${binaryPath}\n` +
      "Run 'npm rebuild @casoon/renderreport' or reinstall the package."
  );
  process.exit(2);
}

try {
  execFileSync(binaryPath, process.argv.slice(2), {
    stdio: "inherit",
  });
} catch (err) {
  process.exit(err.status ?? 2);
}
