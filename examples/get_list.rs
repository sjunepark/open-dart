//! Example for calling the `list` endpoint.

use chrono::NaiveDate;
use open_dart::client::OpenDartApi;
use open_dart::endpoints::list;
use open_dart::types::BgnDe;

#[tokio::main]
async fn main() {
    let api = OpenDartApi::default();

    let begin_date = NaiveDate::from_ymd_opt(2024, 10, 1).expect("Failed to create NaiveDate");

    let list_params = list::ParamsBuilder::default()
        .bgn_de(BgnDe::from(begin_date))
        .build()
        .expect("Failed to build ListRequestParams");

    let list = api.get_list(list_params).await.expect("Failed to get list");

    println!("{:#?}", list);
}
