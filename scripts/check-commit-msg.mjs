import { readFileSync } from "node:fs";

const COMMIT_PATTERN =
  /^(feat|fix|docs|style|refactor|perf|test|build|ci|chore|revert)(\([a-z0-9-]+\))?!?: .{1,100}$/;

function verifyCommitMessage() {
  const messageFile = process.argv[2];
  if (!messageFile) {
    process.stderr.write("Usage: node scripts/check-commit-msg.mjs <commit-msg-file>\n");
    process.exit(1);
  }

  const rawMessage = readFileSync(messageFile, "utf8").trim();
  const firstLine = rawMessage.split(/\r?\n/)[0] ?? "";

  if (!COMMIT_PATTERN.test(firstLine)) {
    process.stderr.write(
      `[ELMAS.md] Message de commit invalide : "${firstLine}"\nAttendu : <type>(<scope optionnel>): <description> (ex: feat(schema): add entity validation)\n`,
    );
    process.exit(1);
  }
}

verifyCommitMessage();
