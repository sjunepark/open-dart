## Enum Patterns

Enum patterns were originally implemented in the way
[`http::method::Method`](https://docs.rs/http/latest/http/method/struct.Method.html) is implemented.

One of the reasons that I followed this pattern was to use `nutype`, which doesn't support enums. However, as we decided
not to use nutype, the nested enum complexity seemed unnecessary.

## Using `nutype` for creating newtypes

This reduces boilerplate related to creating newtypes. However, manual implementation is not that cumbersome. On the
other hand, using `nutype` creates a hard lock-in, because `nutype` doesn't support other attributes to be attached to
the types. This is annoying when using `serde` or `diesel`, etc.

## Avoid using `#[serde(flatten)]`

Serde's [struct flattening](https://serde.rs/attr-flatten.html#struct-flattening) feature is useful
to reuse common fields.

### Silent deserialization failure

When deserializing a struct with #[serde(flatten)], it doesn't throw an error when deserialization fails.

See <https://github.com/serde-rs/serde/issues/2793>.

The original response was generic as below, but it was refactored to be implemented manually for each struct.

```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct OpenDartResponseBody<R> {
    #[serde(flatten)]
    pub message: Message,

    #[serde(flatten)]
    pub content: Option<R>,
}
```

The `content` field resulted as `None` when deserialization fails, without returning any errors.

### `#[serde(flatten)]` doesn't work with `#[serde(deny_unknown_fields)]`

See <https://serde.rs/container-attrs.html#deny_unknown_fields>.

## Use `#[serde(deny_unknown_fields)]` when possible to prevent silent deserialization failure

By default, serde ignores unknown fields when deserializing a struct.
This can lead to silent success when there are unexpected fields in the JSON.

For example, the OpenDart external api can return both of the following responses:
And these are the corresponding Rust representations:

```rust
use serde::{Deserialize, Serialize};
use serde_json::json;

fn deserialize() {
    let without_content = json!({
        "status": "000",
        "message": "success"
    });

    let with_content = json!({
        "status": "000",
        "message": "success",
        "content": "some content"
    });

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(untagged)]
    enum Response {
        Message(Message),
        WithContent(WithContent),
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct Message {
        status: String,
        message: String,
    }

    struct WithContent {
        status: String,
        message: String,
        content: String,
    }

    let without_content: Response = serde_json::from_value(without_content).unwrap();
    let with_content: Response = serde_json::from_value(with_content).unwrap();
}
```

In the case above, both of the JSON can be deserialized as the `Message` variant,
since serde doesn't care if there are additional unknown fields(`content`).

This behavior is not what we want.
We want `with_content` to be deserialized as `WithContent` by making it fail when trying to be deserialized as
`Message`.
