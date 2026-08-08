"use strict";

const fs = require("fs");
const path = require("path");
const os = require("os");
const https = require("https");
const http = require("http");
const { execFileSync } = require("child_process");
const { resolveTarget } = require("./platform");

const OWNER = "Wolfe-Jam";
const REPO = "mcp-better";
const APP = "mcp-better";

/**
 * Cache dir for downloaded binaries (versioned + triple).
 * @param {string} version
 * @param {string} triple
 */
function cacheDir(version, triple) {
  const base =
    process.env.MCP_BETTER_CACHE_DIR ||
    path.join(os.homedir(), ".cache", "mcp-better");
  return path.join(base, version, triple);
}

/**
 * Follow redirects (GitHub release assets).
 * @param {string} url
 * @param {string} dest
 * @param {number} redirects
 */
function downloadFile(url, dest, redirects = 0) {
  if (redirects > 10) {
    throw new Error(`mcp-better: too many redirects fetching ${url}`);
  }
  return new Promise((resolve, reject) => {
    const lib = url.startsWith("https:") ? https : http;
    const req = lib.get(
      url,
      {
        headers: {
          "User-Agent": "mcp-better-npm-shim",
          Accept: "application/octet-stream",
        },
      },
      (res) => {
        if (
          res.statusCode &&
          res.statusCode >= 300 &&
          res.statusCode < 400 &&
          res.headers.location
        ) {
          res.resume();
          downloadFile(res.headers.location, dest, redirects + 1)
            .then(resolve)
            .catch(reject);
          return;
        }
        if (res.statusCode !== 200) {
          res.resume();
          reject(
            new Error(
              `mcp-better: download failed HTTP ${res.statusCode} for ${url}`
            )
          );
          return;
        }
        const tmp = `${dest}.partial`;
        const out = fs.createWriteStream(tmp);
        res.pipe(out);
        out.on("finish", () => {
          out.close(() => {
            fs.renameSync(tmp, dest);
            resolve();
          });
        });
        out.on("error", (err) => {
          try {
            fs.unlinkSync(tmp);
          } catch {
            /* ignore */
          }
          reject(err);
        });
      }
    );
    req.on("error", reject);
  });
}

/**
 * Extract archive to dir. Prefer system tar; fall back to PowerShell Expand-Archive on Windows zip.
 * @param {string} archivePath
 * @param {string} destDir
 * @param {boolean} isWindows
 */
function extractArchive(archivePath, destDir, isWindows) {
  fs.mkdirSync(destDir, { recursive: true });
  if (archivePath.endsWith(".zip")) {
    if (isWindows) {
      execFileSync(
        "powershell.exe",
        [
          "-NoProfile",
          "-Command",
          `Expand-Archive -Path '${archivePath.replace(/'/g, "''")}' -DestinationPath '${destDir.replace(/'/g, "''")}' -Force`,
        ],
        { stdio: "inherit" }
      );
    } else {
      execFileSync("unzip", ["-o", archivePath, "-d", destDir], {
        stdio: "inherit",
      });
    }
    return;
  }
  // .tar.gz / .tar.xz
  execFileSync("tar", ["-xf", archivePath, "-C", destDir], {
    stdio: "inherit",
  });
}

/**
 * Find binary under extracted tree (cargo-dist may nest by triple).
 * @param {string} root
 * @param {string} binaryName
 */
function findBinary(root, binaryName) {
  const direct = path.join(root, binaryName);
  if (fs.existsSync(direct) && fs.statSync(direct).isFile()) {
    return direct;
  }
  /** @type {string[]} */
  const stack = [root];
  while (stack.length) {
    const dir = stack.pop();
    let entries;
    try {
      entries = fs.readdirSync(dir, { withFileTypes: true });
    } catch {
      continue;
    }
    for (const ent of entries) {
      const full = path.join(dir, ent.name);
      if (ent.isDirectory()) {
        stack.push(full);
      } else if (ent.name === binaryName) {
        return full;
      }
    }
  }
  return null;
}

/**
 * Ensure native binary is on disk; return absolute path.
 * @param {string} version package version (must match GH release tag vX.Y.Z)
 */
async function ensureBinary(version) {
  const { triple, isWindows, binaryName } = resolveTarget();
  const dir = cacheDir(version, triple);
  const binPath = path.join(dir, binaryName);

  if (fs.existsSync(binPath)) {
    return binPath;
  }

  // cargo-dist default artifact names (no version infix):
  //   mcp-better-aarch64-apple-darwin.tar.xz
  //   mcp-better-x86_64-pc-windows-msvc.zip
  const ext = isWindows ? "zip" : "tar.xz";
  const asset = `${APP}-${triple}.${ext}`;
  // Also try .tar.gz (hand-rolled releases)
  const candidates = isWindows
    ? [asset]
    : [asset, `${APP}-${triple}.tar.gz`, `${APP}-v${version}-${triple}.tar.gz`];

  fs.mkdirSync(dir, { recursive: true });
  const extractRoot = path.join(dir, "_extract");
  fs.rmSync(extractRoot, { recursive: true, force: true });
  fs.mkdirSync(extractRoot, { recursive: true });

  let lastErr = null;
  for (const name of candidates) {
    const url = `https://github.com/${OWNER}/${REPO}/releases/download/v${version}/${name}`;
    const archivePath = path.join(dir, name);
    try {
      process.stderr.write(`mcp-better: downloading ${name}…\n`);
      await downloadFile(url, archivePath);
      extractArchive(archivePath, extractRoot, isWindows);
      const found = findBinary(extractRoot, binaryName);
      if (!found) {
        throw new Error(`archive ${name} did not contain ${binaryName}`);
      }
      fs.copyFileSync(found, binPath);
      if (!isWindows) {
        fs.chmodSync(binPath, 0o755);
      }
      // tidy
      try {
        fs.unlinkSync(archivePath);
      } catch {
        /* ignore */
      }
      fs.rmSync(extractRoot, { recursive: true, force: true });
      return binPath;
    } catch (err) {
      lastErr = err;
      try {
        fs.unlinkSync(archivePath);
      } catch {
        /* ignore */
      }
    }
  }

  throw new Error(
    `mcp-better: could not download a binary for ${triple} (v${version}). ` +
      `Install Rust and run: cargo install mcp-better --version ${version}\n` +
      `Last error: ${lastErr && lastErr.message ? lastErr.message : lastErr}`
  );
}

module.exports = { ensureBinary, cacheDir };
