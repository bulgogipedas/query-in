/* tslint:disable */
/* eslint-disable */

export class QueryEngine {
    free(): void;
    [Symbol.dispose](): void;
    execute(sql: string): any;
    constructor();
    register_csv(name: string, data: Uint8Array): any;
    table_count(): number;
}

export function infer_schema(data: Uint8Array): any;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly __wbg_queryengine_free: (a: number, b: number) => void;
    readonly infer_schema: (a: number, b: number) => [number, number, number];
    readonly queryengine_execute: (a: number, b: number, c: number) => [number, number, number];
    readonly queryengine_new: () => number;
    readonly queryengine_register_csv: (a: number, b: number, c: number, d: number, e: number) => [number, number, number];
    readonly queryengine_table_count: (a: number) => number;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __externref_table_dealloc: (a: number) => void;
    readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
