/* tslint:disable */
/* eslint-disable */
export class WasmTimeSlices {
  free(): void;
  /**
   *
   *     * Critically, we need to parse the time slices from the explainer page and store them
   *     * for later use. The timeline module does not care about the plain text values.
   *     
   */
  constructor(page_string: string);
  get_times(): Int32Array;
}
export class WasmTimeline {
  free(): void;
  get_balances(who: string): any;
  get_limb_state(who: string): any;
  get_afflictions(who: string): any;
  get_current_time(): number;
  set_timeline_time(slices: WasmTimeSlices, time: number): string[];
  constructor(me: string);
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_wasmtimeline_free: (a: number, b: number) => void;
  readonly __wbg_wasmtimeslices_free: (a: number, b: number) => void;
  readonly wasmtimeline_get_afflictions: (a: number, b: number, c: number) => any;
  readonly wasmtimeline_get_balances: (a: number, b: number, c: number) => any;
  readonly wasmtimeline_get_current_time: (a: number) => number;
  readonly wasmtimeline_get_limb_state: (a: number, b: number, c: number) => any;
  readonly wasmtimeline_new: (a: number, b: number) => number;
  readonly wasmtimeline_set_timeline_time: (a: number, b: number, c: number) => [number, number];
  readonly wasmtimeslices_get_times: (a: number) => [number, number];
  readonly wasmtimeslices_new: (a: number, b: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_export_3: WebAssembly.Table;
  readonly __externref_drop_slice: (a: number, b: number) => void;
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
