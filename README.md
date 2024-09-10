## Todos

- [ ] Use mocks to reduce actual api calls
- [ ] Add `assert_impl_all_commons` to types
- [ ] Finish implementing types

## Limitations

### Doesn't support xml responses

The library doesn't support xml responses. This is because serde isn't fully compatible with xml, and would have to
implement separate structs with different attributes from when parsing json.

> From [quick-xml](https://docs.rs/quick-xml/latest/quick_xml/de/):
>
> Due to the complexity of the XML standard and the fact that Serde was developed with JSON in mind, not all Serde
> concepts apply smoothly to XML. This leads to that fact that some XML concepts are inexpressible in terms of Serde
> derives and may require manual deserialization.

