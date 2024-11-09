//! Example for calling the `list` endpoint.

use open_dart::client::OpenDartApi;
use open_dart::endpoints::fnltt_singl_acnt_all;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_test_writer()
        .with_env_filter(EnvFilter::from_default_env())
        .pretty()
        .init();

    let api = OpenDartApi::default();

    let params = fnltt_singl_acnt_all::ParamsBuilder::default()
        .corp_code("00126380")
        .bsns_year("2023")
        .reprt_code("11011")
        .fs_div("CFS")
        .build()
        .expect("Failed to build FnlttSinglAcntAllRequestParams");
    let response = api
        .get_fnltt_singl_acnt_all(params)
        .await
        .expect("Failed to get list");

    println!("{:#?}", response);
}
