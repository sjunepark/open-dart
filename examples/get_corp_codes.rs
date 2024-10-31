use open_dart::client::OpenDartApi;

#[tokio::main]
async fn main() {
    let api = OpenDartApi::default();
    let corp_codes = api
        .get_corp_codes()
        .await
        .expect("Failed to get corp codes");
    println!("Got {} corp codes", corp_codes.iter().len());

    let first_10_corp_codes = corp_codes.iter().take(10).collect::<Vec<_>>();
    println!("First 10 corp codes: {:#?}", first_10_corp_codes);
}
