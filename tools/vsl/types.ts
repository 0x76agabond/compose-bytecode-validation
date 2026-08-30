/** Minimal AST and VSL types copied from Compose CLI for the standalone generator. */
export type SoliditySourceUnitAst = {
  id: number;
  nodeType: "SourceUnit";
  src: string;
  absolutePath?: string;
  nodes?: unknown[];
  [key: string]: unknown;
};

export type SolidityAstSource = {
  sourceName: string;
  ast: SoliditySourceUnitAst;
};

export type VirtualStorageLayoutKind = "normal" | "immutable";
export type VirtualStorageLayoutSource =
  | "erc8042"
  | "erc7201"
  | "slot-assignment"
  | "implicit-state";

export type VirtualStorageLayoutRecord = {
  id: string;
  /** Tool-only provenance. It distinguishes a generated child from a root named `*.2`. */
  parentPath?: string;
  kind: VirtualStorageLayoutKind;
  codeWidth: 1;
  layout: string[];
  serializedLayout: string[];
  slots: number[][];
  source: VirtualStorageLayoutSource;
  sourceName: string;
  contractName: string;
  structName: string | null;
};

export type VirtualStorageLayoutWarning = { sourceName: string; message: string };
export type VirtualStorageLayoutCollision = {
  id: string;
  reason: string;
  records: VirtualStorageLayoutRecord[];
};

export type VirtualStorageLayoutResult = {
  records: VirtualStorageLayoutRecord[];
  warnings: VirtualStorageLayoutWarning[];
  collisions: VirtualStorageLayoutCollision[];
};
