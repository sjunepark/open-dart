## Enum Patterns

Enum patterns were originally implemented in the way
[`http::method::Method`](https://docs.rs/http/latest/http/method/struct.Method.html) is implemented.

One of the reasons that I followed this pattern was to use `nutype`, which doesn't support enums. However, as we decided
not to use nutype, the nested enum complexity seemed unnecessary.

## Using `nutype` for creating newtypes

This reduces boilerplate related to creating newtypes. However, manual implementation is not that cumbersome. On the
other hand, using `nutype` creates a hard lock-in, because `nutype` doesn't support other attributes to be attached to
the types. This is annoying when using `serde` or `diesel`, etc.

## `#[serde(flatten)]`

Serde's [struct flattening](https://serde.rs/attr-flatten.html#struct-flattening) feature is useful to reuse common
fields.
However, it doesn't throw an error when deserialization fails.
This is a bug.
See [issue](https://github.com/serde-rs/serde/issues/2793).
As so, #[serde(flatten)] should be used sparingly.

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