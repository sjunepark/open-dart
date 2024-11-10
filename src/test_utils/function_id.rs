macro_rules! function_id {
    () => {{
        fn f() {}
        fn type_name_of_val<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let mut name = type_name_of_val(f).strip_suffix("::f").unwrap_or("");
        while let Some(rest) = name.strip_suffix("::{{closure}}") {
            name = rest;
        }
        &name.replace("::", "-")
    }};
}
pub(crate) use function_id;

mod tests {
    use super::*;
    use crate::test_utils::test_context;

    #[tokio::test]
    async fn test_context_function_id_should_be_as_expected() {
        let ctx = test_context!("json").await;
        let expected = function_id!();
        assert_eq!(
            ctx.function_id, *expected,
            "function_id should be as expected"
        );
    }
}
