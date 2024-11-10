//! Example for calling the `list` endpoint.

use open_dart::client::OpenDartApi;
use open_dart::endpoints::list;

#[tokio::main]
async fn main() {
    let api = OpenDartApi::default();

    let list_params = list::ParamsBuilder::default()
        .bgn_de("20241001".to_string())
        .build()
        .expect("Failed to build ListRequestParams");

    let list = api.get_list(list_params).await.expect("Failed to get list");

    println!("{:#?}", list);
}
