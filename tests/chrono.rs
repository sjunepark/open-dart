// use chrono::NaiveDate;
//
// #[test]
// fn naive_date_serialization() -> anyhow::Result<()> {
//     let date = NaiveDate::from_ymd_opt(2021, 1, 1);
//     let serialized = serde_json::to_string(&date)?;
//     assert_eq!(serialized, "\"20210101\"");
//     Ok(())
// }
