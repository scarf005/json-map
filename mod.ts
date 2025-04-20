/**
 * @module
 *
 * JSON `parse` and `stringify` that uses [`Map`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Map) instead of `Object`. Can be used in a niche situation when you need to preserve the order of keys in JSON objects for some reason.
 *
 * Implemented as a thin wrapper around serde-json.
 *
 * ## Example
 *
 * ```
 * import { parse } from "jsr:@scarf/json-map"
 *
 * parse(`{ ".": 1, "$": 2, "1": 3 }`)
 * ```
 */
export {
  parse,
  prettyPrint,
  type RecursiveJsonMap,
  stringify,
} from "./lib/json_map.js"
