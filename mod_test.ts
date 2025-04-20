import { assertEquals } from "@std/assert"
import { parse, prettyPrint, stringify } from "./lib/json_map.js"

Deno.test("idempotency", () => {
  assertEquals(
    parse(stringify({
      ".": 1,
      "$": 2,
      "1": 3,
    })),
    new Map([
      [".", 1],
      ["$", 2],
      ["1", 3],
    ]),
  )
})

Deno.test("parse() preserves order", () => {
  assertEquals(
    parse(`{ ".": 1, "$": 2, "1": 3 }`),
    new Map([
      [".", 1],
      ["$", 2],
      ["1", 3],
    ]),
  )
})

const obj = `{".":1,"$":2,"1":3}`
Deno.test("stringify() preserves order", () => {
  assertEquals(
    stringify(parse(obj)),
    obj,
  )
})
Deno.test("prettyPrint() preserves order", () => {
  assertEquals(
    prettyPrint(parse(obj)),
    `{
  ".": 1,\n  "$": 2,\n  "1": 3
}`,
  )
})

Deno.test("stringify() works for regular JSON values", () => {
  assertEquals(stringify(1), "1")
  assertEquals(stringify("1"), '"1"')
  assertEquals(stringify(true), "true")
  assertEquals(stringify(false), "false")
  assertEquals(stringify(null), "null")
  assertEquals(stringify([1, 2, 3]), "[1,2,3]")
})
