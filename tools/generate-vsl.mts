import { execFile } from "node:child_process";
import { mkdtemp, readFile, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import path from "node:path";
import { promisify } from "node:util";
import { buildVirtualStorageLayout } from "./vsl/virtualStorageLayout";
import type { SoliditySourceUnitAst, VirtualStorageLayoutRecord } from "./vsl/types";

const exec = promisify(execFile);

type ForgeArtifact = { ast?: SoliditySourceUnitAst };
type GeneratedRecord = VirtualStorageLayoutRecord & {
  virtualPath: string;
  parentVirtualPath: string | null;
  diamondName: null;
};

async function main(): Promise<void> {
  const [sourceArgument, contractName, ...options] = process.argv.slice(2);
  if (!sourceArgument || !contractName) {
    throw new Error("Usage: generate-vsl.mts <source.sol> <contract-name> [--out <vsl.json>]");
  }

  const outputIndex = options.indexOf("--out");
  const outputPath = outputIndex === -1
    ? path.join(path.dirname(path.resolve(sourceArgument)), "vsl.json")
    : path.resolve(options[outputIndex + 1] ?? "");
  if (!outputPath) throw new Error("--out requires a path");

  const sourcePath = path.resolve(sourceArgument);
  const outputDirectory = await mkdtemp(path.join(tmpdir(), "compose-vsl-"));
  try {
    const forge = process.env.FOUNDRY_FORGE ?? "forge";
    await exec(forge, ["build", path.basename(sourcePath), "--ast", "--force", "--out", outputDirectory], {
      cwd: path.dirname(sourcePath),
    });

    const artifactPath = path.join(outputDirectory, path.basename(sourcePath), `${contractName}.json`);
    const artifact = JSON.parse(await readFile(artifactPath, "utf8")) as ForgeArtifact;
    if (!artifact.ast) throw new Error(`Foundry artifact has no AST: ${artifactPath}`);

    const layout = buildVirtualStorageLayout([{
      sourceName: artifact.ast.absolutePath ?? sourcePath,
      ast: artifact.ast,
    }], [contractName]);
    const records = await Promise.all(layout.records.map(async (record) => ({
      ...record,
      id: await keccak(record.id),
      virtualPath: record.id,
      parentVirtualPath: record.parentPath ?? null,
      diamondName: null,
    })));

    await writeFile(outputPath, `${JSON.stringify({ records }, null, 2)}\n`);
    console.log(`Generated ${records.length} VSL record(s): ${outputPath}`);
    for (const warning of layout.warnings) console.warn(`warning: ${warning.message}`);
    for (const collision of layout.collisions) console.warn(`collision: ${collision.reason} (${collision.id})`);
  } finally {
    await rm(outputDirectory, { recursive: true, force: true });
  }
}

async function keccak(value: string): Promise<string> {
  const cast = process.env.FOUNDRY_CAST ?? "cast";
  const { stdout } = await exec(cast, ["keccak", value]);
  return stdout.trim();
}

void main().catch((error: unknown) => {
  console.error(error instanceof Error ? error.message : error);
  process.exitCode = 1;
});
