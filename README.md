# `@scarf005/json-map`

JSON `parse` and `stringify` that uses [`Map`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Map) instead of `Object`. Can be used in a niche situation when you need to preserve the order of keys in JSON objects for some reason.

Implemented as a thin wrapper around [serde_json](https://crates.io/crates/serde_json/1.0.1/dependencies).
