use open_dart::client::OpenDartApi;
use open_dart::endpoints::company;

#[tokio::main]
async fn main() {
    let api = OpenDartApi::default();

    let company_params = company::ParamsBuilder::default()
        .corp_code("00126380")
        .build()
        .expect("Failed to build CompanyRequestParams");

    let company = api
        .get_company(company_params)
        .await
        .expect("Failed to get company");

    println!("{:#?}", company);
}
