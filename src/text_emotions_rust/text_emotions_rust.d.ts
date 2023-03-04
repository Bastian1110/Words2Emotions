/* tslint:disable */
/* eslint-disable */
/**
 */
export function greet(): void;
/**
 */
export function model_from_file(): void;
/**
 */
export class EmotionRecognition {
	free(): void;
	/**
	 * @returns {EmotionRecognition}
	 */
	static new(): EmotionRecognition;
	/**
	 * @param {string} input
	 * @returns {string}
	 */
	predict(input: string): string;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
	readonly memory: WebAssembly.Memory;
	readonly model_from_file: () => void;
	readonly __wbg_emotionrecognition_free: (a: number) => void;
	readonly emotionrecognition_new: () => number;
	readonly emotionrecognition_predict: (a: number, b: number, c: number, d: number) => void;
	readonly greet: () => void;
	readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
	readonly __wbindgen_malloc: (a: number) => number;
	readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
	readonly __wbindgen_free: (a: number, b: number) => void;
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
export default function init(module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
