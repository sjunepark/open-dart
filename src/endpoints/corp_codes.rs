use crate::client::OpenDartApi;
use crate::error::{OpenDartError, UnexpectedZipContentError, ValidationError};
use crate::types::{CorpCode, CorpName, Date, StockCode};
use bytes::Bytes;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::io::{BufRead, BufReader, Cursor};
use std::str::FromStr;
use zip::ZipArchive;

impl OpenDartApi {
    #[tracing::instrument(skip(self))]
    pub async fn get_corp_codes(&self) -> Result<CorpInfos, OpenDartError> {
        let url = self.url("/api/corpCode.xml");
        let bytes = self.get_zip(url).await?;
        let cursor = Cursor::new(bytes);
        let mut zip = ZipArchive::new(cursor)?;
        CorpInfos::from_zip(&mut zip)
    }
}

#[derive(
    std::fmt::Debug,
    Clone,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    // derive_more
    derive_more::AsRef,
    derive_more::Display,
    derive_more::From,
    derive_more::Into,
    // serde
    serde::Serialize,
    serde::Deserialize,
)]
#[display("{self:?}")]
pub struct CorpInfo {
    corp_code: CorpCode,
    corp_name: CorpName,
    stock_code: StockCode,
    modify_date: Date,
}

#[derive(Default)]
struct CorpInfoRaw {
    corp_code: String,
    corp_name: String,
    stock_code: String,
    modify_date: String,
}

impl TryFrom<CorpInfoRaw> for CorpInfo {
    type Error = OpenDartError;

    fn try_from(value: CorpInfoRaw) -> Result<Self, Self::Error> {
        Ok(CorpInfo {
            corp_code: CorpCode::try_new(&value.corp_code)?,
            corp_name: CorpName::try_new(&value.corp_name)?,
            stock_code: StockCode::try_new(&value.stock_code)?,
            modify_date: Date::from_str(&value.modify_date)?,
        })
    }
}

#[derive(
    std::fmt::Debug,
    Clone,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    // derive_more
    derive_more::AsRef,
    derive_more::Display,
    derive_more::From,
    derive_more::Into,
    // serde
    serde::Serialize,
    serde::Deserialize,
)]
#[cfg_attr(
    feature = "diesel_newtype",
    derive(diesel_derive_newtype::DieselNewType)
)]
#[display("{self:?}")]
pub struct CorpInfos(Vec<CorpInfo>);

impl CorpInfos {
    #[tracing::instrument(skip(zip))]
    fn from_zip(zip: &mut ZipArchive<Cursor<Bytes>>) -> Result<Self, OpenDartError> {
        if zip.len() != 1 {
            let mut files = Vec::new();

            for i in 0..zip.len() {
                let file = zip.by_index(i)?;
                files.push(file.name().to_string());
            }

            Err(UnexpectedZipContentError { files })?;
        }

        let zip_file = zip.by_index(0)?;
        if !zip_file.name().ends_with(".xml") {
            Err(UnexpectedZipContentError {
                files: vec![zip_file.name().to_string()],
            })?;
        }

        let buf = BufReader::new(zip_file);
        let corp_infos = CorpInfos::from_reader(buf)?;
        Ok(corp_infos)
    }

    #[tracing::instrument(skip(reader))]
    fn from_reader<R: BufRead>(reader: R) -> Result<Self, OpenDartError> {
        let mut xml_reader = Reader::from_reader(reader);
        xml_reader.config_mut().trim_text(true);

        let mut items = Vec::new();
        let mut buf = Vec::new();
        let mut current_item = CorpInfoRaw::default();
        let mut current_field = String::new();

        loop {
            match xml_reader.read_event_into(&mut buf) {
                Ok(Event::Decl(_)) => {
                    tracing::trace!("Skipping XML declaration tag.");
                }
                Ok(Event::Empty(e)) => {
                    let field = String::from_utf8_lossy(e.name().as_ref()).to_string();
                    match field.as_str() {
                        "corp_code" => current_item.corp_code = "".to_string(),
                        "corp_name" => current_item.corp_name = "".to_string(),
                        "stock_code" => current_item.stock_code = "".to_string(),
                        "modify_date" => current_item.modify_date = "".to_string(),
                        field => {
                            Err(ValidationError {
                                value: field.to_string(),
                                message: "Unexpected field while parsing xml.".to_string(),
                            })?;
                        }
                    }
                }
                Ok(Event::Start(ref e)) => {
                    current_field = String::from_utf8_lossy(e.name().as_ref()).to_string();
                }
                Ok(Event::Text(e)) => {
                    let text = e.unescape()?.to_string();
                    match current_field.as_str() {
                        "corp_code" => current_item.corp_code = text,
                        "corp_name" => current_item.corp_name = text,
                        "stock_code" => current_item.stock_code = text,
                        "modify_date" => current_item.modify_date = text,
                        field => {
                            Err(ValidationError {
                                value: field.to_string(),
                                message: "Unexpected field while parsing xml.".to_string(),
                            })?;
                        }
                    }
                }
                Ok(Event::End(ref e)) => {
                    if String::from_utf8_lossy(e.name().as_ref()) == "list" {
                        let corp_info = CorpInfo::try_from(std::mem::take(&mut current_item))?;
                        items.push(corp_info);
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => Err(e)?,
                e => Err(ValidationError {
                    value: format!("{:?}", e),
                    message: "Unexpected event while parsing xml.".to_string(),
                })?,
            }
            buf.clear();
        }

        Ok(CorpInfos(items))
    }

    pub fn iter(&self) -> std::slice::Iter<CorpInfo> {
        self.0.iter()
    }
}

impl IntoIterator for CorpInfos {
    type Item = CorpInfo;
    type IntoIter = std::vec::IntoIter<CorpInfo>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::test_context;
    use crate::test_utils::tracing_setup::subscribe_tracing_with_span;
    use goldrust::Content;
    use serde::Serialize;
    use std::io::Write;
    use zip::write::FileOptions;
    use zip::ZipWriter;

    const XML_CONTENT: &str = r#"<?xml version="1.0" encoding="UTF-8"?>

<result>
    <list>
        <corp_code>00126380</corp_code>
        <corp_name>삼성전자(주)</corp_name>
        <stock_code>005930</stock_code>
        <modify_date>20210531</modify_date>
    </list>
    <list>
        <corp_code>00164779</corp_code>
        <corp_name>삼성전자서비스(주)</corp_name>
        <stock_code>012057</stock_code>
        <modify_date>20210531</modify_date>
    </list>
</result>"#;

    fn create_mock_zip_bytes(file_count: u64) -> Bytes {
        let mut buf = Vec::new();
        let mut writer = ZipWriter::new(Cursor::new(&mut buf));
        for i in 0..file_count {
            writer
                .start_file::<&str, ()>(format!("test{}.xml", i).as_str(), FileOptions::default())
                .unwrap();
            writer.write_all(XML_CONTENT.as_bytes()).unwrap();
        }
        writer.finish().unwrap();

        buf.into()
    }

    #[test]
    fn read_xml_from_zip_works() {
        let zip = create_mock_zip_bytes(1);

        let cursor = Cursor::new(zip);
        let mut zip = ZipArchive::new(cursor).unwrap();
        let corp_infos = CorpInfos::from_zip(&mut zip).unwrap();

        assert_eq!(
            corp_infos,
            CorpInfos(vec![
                CorpInfo {
                    corp_code: CorpCode::try_new("00126380").unwrap(),
                    corp_name: CorpName::try_new("삼성전자(주)").unwrap(),
                    stock_code: StockCode::try_new("005930").unwrap(),
                    modify_date: Date::from_str("20210531").unwrap(),
                },
                CorpInfo {
                    corp_code: CorpCode::try_new("00164779").unwrap(),
                    corp_name: CorpName::try_new("삼성전자서비스(주)").unwrap(),
                    stock_code: StockCode::try_new("012057").unwrap(),
                    modify_date: Date::from_str("20210531").unwrap(),
                }
            ])
        );
    }

    #[test]
    fn read_xml_should_error_when_there_are_more_than_one_file() {
        let zip = create_mock_zip_bytes(2);

        let cursor = Cursor::new(zip);
        let mut zip = ZipArchive::new(cursor).unwrap();

        let corp_infos = CorpInfos::from_zip(&mut zip);
        assert!(corp_infos.is_err());
    }

    // todo: add assertion
    #[tokio::test]
    async fn get_corp_codes_works() {
        subscribe_tracing_with_span!("test");
        let mut ctx = test_context!("zip").await;
        ctx.arrange_test_endpoint_zip("/api/corpCode.xml").await;
        let corp_infos = ctx.api.get_corp_codes().await.unwrap();

        // Use only the first 10 items for testing
        // since the actual data is too large to be saved in the test context.
        let corp_infos_first_10 = &corp_infos.0.into_iter().take(10).collect::<Vec<_>>();
        tracing::debug!(?corp_infos_first_10);
        assert!(!corp_infos_first_10.is_empty());

        // region: Serialize to XML
        #[derive(Serialize)]
        #[serde(rename = "result")]
        struct Root {
            list: Vec<CorpInfo>,
        }
        let root = Root {
            list: corp_infos_first_10.to_owned(),
        };
        let xml = quick_xml::se::to_string(&root).unwrap();
        tracing::debug!(?xml);
        // endregion: Serialize to XML

        // region: Save to a golden file
        let xml = xml.as_bytes();

        let mut buf = Vec::new();
        let mut writer = ZipWriter::new(Cursor::new(&mut buf));
        writer
            .start_file::<&str, ()>("corp_infos.xml", FileOptions::default())
            .unwrap();
        writer.write_all(xml).unwrap();
        writer.finish().unwrap();

        ctx.goldrust
            .save(Content::Zip(buf))
            .expect("failed to save corp_infos");
        // endregion: Save to a golden file
    }
}