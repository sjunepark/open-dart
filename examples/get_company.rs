use open_dart::client::OpenDartApi;
use open_dart::endpoints::company;
use open_dart::types::CorpCode;

#[tokio::main]
async fn main() {
    let api = OpenDartApi::default();

    let company_params = company::ParamsBuilder::default()
        .corp_code(CorpCode::try_new("00126380").expect("Failed to create CorpCode"))
        .build()
        .expect("Failed to build CompanyRequestParams");

    let company = api
        .get_company(company_params)
        .await
        .expect("Failed to get company");

    println!("{:#?}", company);
}
