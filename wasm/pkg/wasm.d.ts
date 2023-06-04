/* tslint:disable */
/* eslint-disable */
/**
* @param {string} input
* @returns {string}
*/
export function decode(input: string): string;
/**
* @param {string} input
* @param {string} secret
* @returns {string}
*/
export function encode(input: string, secret: string): string;
/**
* @param {string} input
* @param {Uint8Array} secret
* @returns {string}
*/
export function encode_binary(input: string, secret: Uint8Array): string;
/**
* @param {string} input
* @returns {Uint8Array}
*/
export function decode_binary(input: string): Uint8Array;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly decode: (a: number) => number;
  readonly encode: (a: number, b: number) => number;
  readonly encode_binary: (a: number, b: number) => number;
  readonly decode_binary: (a: number) => number;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
