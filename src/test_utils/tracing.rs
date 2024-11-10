pub(crate) fn subscribe() {
    let subscriber = tracing_subscriber::fmt()
        .with_test_writer()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .pretty()
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber");
}

macro_rules! subscribe_tracing_with_span {
    ($span_name:expr) => {
        $crate::test_utils::tracing::subscribe();

        let function_id = $crate::test_utils::function_id!();
        let _span = tracing::info_span!($span_name, ?function_id).entered();
    };
}
pub(crate) use subscribe_tracing_with_span;
