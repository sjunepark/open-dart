## Enum Patterns

Enum patterns were originally implemented in the way
[`http::method::Method`](https://docs.rs/http/latest/http/method/struct.Method.html) is implemented.

One of the reasons that I followed this pattern was to use `nutype`, which doesn't support enums. However, as we decided
not to use nutype, the nested enum complexity seemed unnecessary.

## Using `nutype` for creating newtypes

This reduces boilerplate related to creating newtypes. However, manual implementation is not that cumbersome. On the
other hand, using `nutype` creates a hard lock-in, because `nutype` doesn't support other attributes to be attached to
the types. This is annoying when using `serde` or `diesel`, etc.