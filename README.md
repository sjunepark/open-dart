## Todos

- [ ] Review api test logic and mock logic
- [ ] Use mocks to reduce actual api calls
- [ ] Add `assert_impl_all_commons` to types
- [ ] Finish implementing types

## Limitations

### Doesn't support XML responses

The library doesn't support XML responses.
This is because serde isn't fully compatible with XML and would have to
implement separate structs with different attributes from when parsing JSON.

> From [quick-xml](https://docs.rs/quick-xml/latest/quick_xml/de/):
>
> Due to the complexity of the XML standard and the fact that Serde was developed with JSON in mind, not all Serde
> concepts apply smoothly to XML. This leads to that fact that some XML concepts are inexpressible in terms of Serde
> derives and may require manual deserialization.

## Development Guidelines

This crate leverages type-driven development to ensure that the API is used correctly. This means that the types

### Newtypes

The crate leverages the use of newtypes to provide a more type-safe interface.
The inner type is always private, and it will be a primitive type or an enum in most cases.

### Enum patterns

If a certain type could be of multiple values (so-called, enum or union type), they should be represented as below.

Numeric values are implemented in the same way
[`http::status::StatusCode`](https://docs.rs/http/latest/http/status/struct.StatusCode.html) is implemented.

```rust
struct StatusCode(uint);

impl StatusCode {
    const OK: Self = Self(200);
    const CREATED: Self = Self(201);
    // ...
}
```

String values are implemented in the same way
[`http::method::Method`](https://docs.rs/http/latest/http/method/struct.Method.html) is implemented.

### Optional fields

When a field or parameter is optional, wrap the type in an Option when specifying the field type ,
not the inner type of the newtype.

```rust
// Yes
pub struct SomeRequestParams {
    pub api_key: Option<ApiKey>,
}

struct ApiKey(String);
```

```rust
// No
pub struct SomeRequestParams {
    pub api_key: ApiKey,
}

struct ApiKey(Option<String>);
```

e