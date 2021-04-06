/* tslint:disable */
/* eslint-disable */
/**
* @param {string} xprv
* @param {number} idx
* @returns {any}
*/
export function to_wif(xprv: string, idx: number): any;
/**
* @param {string} xprv
* @param {number} words_number
* @param {number} idx
* @returns {any}
*/
export function to_mnemonic(xprv: string, words_number: number, idx: number): any;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly to_wif: (a: number, b: number, c: number) => number;
  readonly rustsecp256k1_v0_2_0_context_preallocated_destroy: (a: number) => void;
  readonly to_mnemonic: (a: number, b: number, c: number, d: number) => number;
  readonly rustsecp256k1_v0_2_0_ec_pubkey_create: (a: number, b: number, c: number) => number;
  readonly rustsecp256k1_v0_2_0_ec_pubkey_serialize: (a: number, b: number, c: number, d: number, e: number) => number;
  readonly rustsecp256k1_v0_2_0_ec_seckey_verify: (a: number, b: number) => number;
  readonly rustsecp256k1_v0_2_0_ec_seckey_tweak_add: (a: number, b: number, c: number) => number;
  readonly rustsecp256k1_v0_2_0_ec_pubkey_parse: (a: number, b: number, c: number, d: number) => number;
  readonly rustsecp256k1_v0_2_0_ecdsa_recoverable_signature_serialize_compact: (a: number, b: number, c: number, d: number) => number;
  readonly rustsecp256k1_v0_2_0_ecdsa_recoverable_signature_parse_compact: (a: number, b: number, c: number, d: number) => number;
  readonly rustsecp256k1_v0_2_0_context_preallocated_size: (a: number) => number;
  readonly rustsecp256k1_v0_2_0_context_preallocated_create: (a: number, b: number) => number;
  readonly rustsecp256k1_v0_2_0_ecdsa_signature_serialize_der: (a: number, b: number, c: number, d: number) => number;
  readonly rustsecp256k1_v0_2_0_ecdsa_signature_parse_der: (a: number, b: number, c: number, d: number) => number;
  readonly rustsecp256k1_v0_2_0_ecdsa_signature_parse_compact: (a: number, b: number, c: number) => number;
  readonly rustsecp256k1_v0_2_0_ecdsa_signature_normalize: (a: number, b: number, c: number) => number;
  readonly rustsecp256k1_v0_2_0_ec_pubkey_combine: (a: number, b: number, c: number, d: number) => number;
  readonly rustsecp256k1_v0_2_0_context_create: (a: number) => number;
  readonly rustsecp256k1_v0_2_0_context_destroy: (a: number) => void;
  readonly rustsecp256k1_v0_2_0_default_illegal_callback_fn: (a: number, b: number) => void;
  readonly rustsecp256k1_v0_2_0_default_error_callback_fn: (a: number, b: number) => void;
  readonly rustsecp256k1_v0_2_0_context_preallocated_clone_size: (a: number) => number;
  readonly rustsecp256k1_v0_2_0_context_preallocated_clone: (a: number, b: number) => number;
  readonly rustsecp256k1_v0_2_0_context_set_illegal_callback: (a: number, b: number, c: number) => void;
  readonly rustsecp256k1_v0_2_0_context_set_error_callback: (a: number, b: number, c: number) => void;
  readonly rustsecp256k1_v0_2_0_ecdsa_signature_serialize_compact: (a: number, b: number, c: number) => number;
  readonly rustsecp256k1_v0_2_0_ecdsa_verify: (a: number, b: number, c: number, d: number) => number;
  readonly rustsecp256k1_v0_2_0_ecdsa_sign: (a: number, b: number, c: number, d: number, e: number, f: number) => number;
  readonly rustsecp256k1_v0_2_0_ec_seckey_negate: (a: number, b: number) => number;
  readonly rustsecp256k1_v0_2_0_ec_privkey_negate: (a: number, b: number) => number;
  readonly rustsecp256k1_v0_2_0_ec_pubkey_negate: (a: number, b: number) => number;
  readonly rustsecp256k1_v0_2_0_ec_privkey_tweak_add: (a: number, b: number, c: number) => number;
  readonly rustsecp256k1_v0_2_0_ec_pubkey_tweak_add: (a: number, b: number, c: number) => number;
  readonly rustsecp256k1_v0_2_0_ec_seckey_tweak_mul: (a: number, b: number, c: number) => number;
  readonly rustsecp256k1_v0_2_0_ec_privkey_tweak_mul: (a: number, b: number, c: number) => number;
  readonly rustsecp256k1_v0_2_0_ec_pubkey_tweak_mul: (a: number, b: number, c: number) => number;
  readonly rustsecp256k1_v0_2_0_context_randomize: (a: number, b: number) => number;
  readonly rustsecp256k1_v0_2_0_ecdh: (a: number, b: number, c: number, d: number, e: number, f: number) => number;
  readonly rustsecp256k1_v0_2_0_ecdsa_recoverable_signature_convert: (a: number, b: number, c: number) => number;
  readonly rustsecp256k1_v0_2_0_ecdsa_sign_recoverable: (a: number, b: number, c: number, d: number, e: number, f: number) => number;
  readonly rustsecp256k1_v0_2_0_ecdsa_recover: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
