import { readFileSync, readdirSync, statSync } from "node:fs";
import { extname, join, relative } from "node:path";

const MAX_LINES = 350;
const ROOT_DIR = process.cwd();
const IGNORED_DIRS = new Set([".git", "node_modules", "target", "dist", ".turbo"]);
const CHECKED_EXTENSIONS = new Set([
  ".rs",
  ".ts",
  ".tsx",
  ".js",
  ".mjs",
  ".cjs",
  ".css",
  ".toml",
  ".yml",
  ".yaml",
]);

function collectSourceFiles(currentDir) {
  const entries = readdirSync(currentDir);
  const collected = [];

  for (const entry of entries) {
    if (IGNORED_DIRS.has(entry)) {
      continue;
    }
    const fullPath = join(currentDir, entry);
    const stats = statSync(fullPath);

    if (stats.isDirectory()) {
      collected.push(...collectSourceFiles(fullPath));
      continue;
    }

    if (CHECKED_EXTENSIONS.has(extname(entry)) && entry !== "pnpm-lock.yaml") {
      collected.push(fullPath);
    }
  }

  return collected;
}

function countFileLines(filePath) {
  const content = readFileSync(filePath, "utf8");
  if (content.length === 0) {
    return 0;
  }
  return content.split(/\r?\n/).length;
}

function verifyMaxLineCount() {
  const files = collectSourceFiles(ROOT_DIR);
  const violations = [];

  for (const filePath of files) {
    const lineCount = countFileLines(filePath);
    if (lineCount > MAX_LINES) {
      violations.push({
        file: relative(ROOT_DIR, filePath),
        lines: lineCount,
      });
    }
  }

  if (violations.length > 0) {
    process.stderr.write(
      `[ELMAS.md] Échec : ${violations.length} fichier(s) dépassent la limite de ${MAX_LINES} lignes :\n`,
    );
    for (const violation of violations) {
      process.stderr.write(`  - ${violation.file} (${violation.lines} lignes)\n`);
    }
    process.exit(1);
  }

  process.stdout.write(
    `[ELMAS.md] OK : ${files.length} fichiers vérifiés (<= ${MAX_LINES} lignes).\n`,
  );
}

verifyMaxLineCount();
