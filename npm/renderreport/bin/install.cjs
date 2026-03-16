#!/usr/bin/env node

/**
 * Post-install script: downloads the prebuilt renderreport binary
 * for the current platform from GitHub Releases.
 */

const { execSync } = require("child_process");
const crypto = require("crypto");
const fs = require("fs");
const path = require("path");
const https = require("https");

const PACKAGE = "renderreport";
const VERSION = require("../package.json").version;
const REPO = "casoon/renderreport";

function getPlatformTarget() {
  const platform = process.platform;
  const arch = process.arch;

  const targets = {
    "darwin-x64": "x86_64-apple-darwin",
    "darwin-arm64": "aarch64-apple-darwin",
    "linux-x64": "x86_64-unknown-linux-gnu",
    "linux-arm64": "aarch64-unknown-linux-gnu",
    "win32-x64": "x86_64-pc-windows-msvc",
  };

  const key = `${platform}-${arch}`;
  const target = targets[key];

  if (!target) {
    console.error(`Unsupported platform: ${key}`);
    console.error(`Supported platforms: ${Object.keys(targets).join(", ")}`);
    process.exit(1);
  }

  return target;
}

function getBinaryName() {
  return process.platform === "win32" ? `${PACKAGE}.exe` : PACKAGE;
}

function getDownloadUrl(target) {
  const ext = process.platform === "win32" ? ".zip" : ".tar.gz";
  return `https://github.com/${REPO}/releases/download/v${VERSION}/${PACKAGE}-v${VERSION}-${target}${ext}`;
}

async function download(url, dest) {
  return new Promise((resolve, reject) => {
    const follow = (url, redirects = 0) => {
      if (redirects > 5) {
        reject(new Error("Too many redirects"));
        return;
      }

      https
        .get(url, (res) => {
          if (
            res.statusCode >= 300 &&
            res.statusCode < 400 &&
            res.headers.location
          ) {
            follow(res.headers.location, redirects + 1);
            return;
          }

          if (res.statusCode !== 200) {
            reject(
              new Error(`Download failed: HTTP ${res.statusCode} from ${url}`)
            );
            return;
          }

          const file = fs.createWriteStream(dest);
          res.pipe(file);
          file.on("finish", () => {
            file.close(resolve);
          });
        })
        .on("error", reject);
    };

    follow(url);
  });
}

async function downloadToBuffer(url) {
  return new Promise((resolve, reject) => {
    const follow = (url, redirects = 0) => {
      if (redirects > 5) {
        reject(new Error("Too many redirects"));
        return;
      }

      https
        .get(url, (res) => {
          if (
            res.statusCode >= 300 &&
            res.statusCode < 400 &&
            res.headers.location
          ) {
            follow(res.headers.location, redirects + 1);
            return;
          }

          if (res.statusCode !== 200) {
            reject(
              new Error(`Download failed: HTTP ${res.statusCode} from ${url}`)
            );
            return;
          }

          const chunks = [];
          res.on("data", (chunk) => chunks.push(chunk));
          res.on("end", () => resolve(Buffer.concat(chunks)));
        })
        .on("error", reject);
    };

    follow(url);
  });
}

function verifySha256(filePath, checksumContent, archiveFilename) {
  const fileBuffer = fs.readFileSync(filePath);
  const actualHash = crypto
    .createHash("sha256")
    .update(fileBuffer)
    .digest("hex");

  const lines = checksumContent.toString("utf-8").trim().split("\n");
  for (const line of lines) {
    const match = line.match(/^([0-9a-f]{64})\s+(.+)$/);
    if (match && match[2].trim() === archiveFilename) {
      const expectedHash = match[1];
      if (actualHash !== expectedHash) {
        throw new Error(
          `SHA256 mismatch for ${archiveFilename}!\n` +
            `  Expected: ${expectedHash}\n` +
            `  Actual:   ${actualHash}\n` +
            "The downloaded binary may have been tampered with. Aborting install."
        );
      }
      console.log(`  SHA256 verified: ${actualHash}`);
      return;
    }
  }

  console.warn(
    `  Warning: SHA256 checksum not found for ${archiveFilename} — skipping verification.`
  );
}

async function main() {
  const target = getPlatformTarget();
  const binaryName = getBinaryName();
  const binDir = path.join(__dirname);
  const binaryPath = path.join(binDir, binaryName);

  // Skip if binary already exists and matches version
  if (fs.existsSync(binaryPath)) {
    try {
      const out = execSync(`"${binaryPath}" --version`, {
        encoding: "utf-8",
        timeout: 5000,
      }).trim();
      const installedVersion = out.split(/\s+/).pop();
      if (installedVersion === VERSION) {
        console.log(
          `${PACKAGE} v${VERSION} already installed, skipping download.`
        );
        return;
      }
      console.log(
        `${PACKAGE} version mismatch: installed ${installedVersion}, expected ${VERSION}. Re-downloading...`
      );
    } catch {
      console.log(
        `${PACKAGE} binary exists but version check failed. Re-downloading...`
      );
    }
  }

  const url = getDownloadUrl(target);
  const archiveExt = process.platform === "win32" ? ".zip" : ".tar.gz";
  const archiveFilename = `${PACKAGE}-v${VERSION}-${target}${archiveExt}`;
  const archivePath = path.join(binDir, `download${archiveExt}`);

  console.log(`Downloading ${PACKAGE} v${VERSION} for ${target}...`);
  console.log(`  URL: ${url}`);

  try {
    await download(url, archivePath);

    // Verify SHA256 checksum
    const checksumUrl = `${url}.sha256`;
    try {
      const checksumData = await downloadToBuffer(checksumUrl);
      verifySha256(archivePath, checksumData, archiveFilename);
    } catch (checksumErr) {
      if (checksumErr.message.includes("SHA256 mismatch")) {
        fs.unlinkSync(archivePath);
        throw checksumErr;
      }
      console.warn(
        `  Warning: Could not download checksum file: ${checksumErr.message}`
      );
      console.warn("  Skipping SHA256 verification.");
    }

    // Extract
    if (process.platform === "win32") {
      execSync(
        `powershell -Command "Expand-Archive -Path '${archivePath}' -DestinationPath '${binDir}' -Force"`,
        { stdio: "inherit" }
      );
    } else {
      execSync(`tar -xzf "${archivePath}" -C "${binDir}"`, {
        stdio: "inherit",
      });
    }

    // Ensure binary is executable
    if (process.platform !== "win32") {
      fs.chmodSync(binaryPath, 0o755);
    }

    // Cleanup archive
    fs.unlinkSync(archivePath);

    console.log(`${PACKAGE} v${VERSION} installed successfully.`);
  } catch (err) {
    console.warn(`Failed to download ${PACKAGE}: ${err.message}`);
    console.warn(
      "You can install it manually from: " +
        `https://github.com/${REPO}/releases`
    );
  }
}

main();
