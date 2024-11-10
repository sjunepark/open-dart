use crate::client::OpenDartApi;
use crate::endpoints::macros::derive_common;
use crate::error::{OpenDartError, UnexpectedZipContentError};
use crate::utils::derive_newtype;
use bytes::Bytes;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::borrow::Cow;
use std::io::{BufRead, BufReader, Cursor};
use validator::ValidationError;
use zip::ZipArchive;

impl OpenDartApi {
    #[tracing::instrument(skip(self))]
    pub async fn get_corp_codes(&self) -> Result<CorpMetas, OpenDartError> {
        let url = self.url("/api/corpCode.xml");
        let bytes = self.get_zip(url).await?;
        let cursor = Cursor::new(bytes);
        let mut zip = ZipArchive::new(cursor)?;
        CorpMetas::from_zip(&mut zip)
    }
}

derive_newtype! {
    /// ## 기업 코드 정보
    #[display("{_0:?}")]
    pub struct CorpMetas(Vec<CorpCodeMeta>);
}

derive_common!(CorpCodeMeta {
    corp_code: String,
    corp_name: String,
    stock_code: String,
    modify_date: String
});

impl IntoIterator for CorpMetas {
    type Item = CorpCodeMeta;
    type IntoIter = std::vec::IntoIter<CorpCodeMeta>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[derive(Default)]
struct CorpCodeMetaOptional {
    corp_code: Option<String>,
    corp_name: Option<String>,
    stock_code: Option<String>,
    modify_date: Option<String>,
}

impl TryFrom<CorpCodeMetaOptional> for CorpCodeMeta {
    type Error = OpenDartError;

    fn try_from(optional: CorpCodeMetaOptional) -> Result<Self, Self::Error> {
        let validation_error = |field: &str| {
            let mut err = validator::ValidationError::new("empty_value");
            err.add_param(Cow::from("field"), &field);
            OpenDartError::from(err)
        };

        Ok(Self {
            corp_code: optional.corp_code.ok_or(validation_error("corp_code"))?,
            corp_name: optional.corp_name.ok_or(validation_error("corp_name"))?,
            // stock_code is optional
            stock_code: optional.stock_code.unwrap_or_default(),
            modify_date: optional
                .modify_date
                .ok_or(validation_error("modify_date"))?,
        })
    }
}

impl CorpMetas {
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
        let corp_infos = CorpMetas::from_reader(buf)?;
        Ok(corp_infos)
    }

    #[tracing::instrument(skip(reader))]
    fn from_reader<R: BufRead>(reader: R) -> Result<Self, OpenDartError> {
        let mut xml_reader = Reader::from_reader(reader);
        xml_reader.config_mut().trim_text(true);

        let mut items = Vec::new();
        let mut buf = Vec::new();
        let mut current_item = CorpCodeMetaOptional::default();
        let mut current_field = String::new();

        loop {
            match xml_reader.read_event_into(&mut buf) {
                Ok(Event::Decl(_)) => {
                    tracing::trace!("Skipping XML declaration tag.");
                }
                Ok(Event::Empty(e)) => {
                    let field = String::from_utf8_lossy(e.name().as_ref()).to_string();
                    match field.as_str() {
                        "corp_code" => current_item.corp_code = Some("".to_string()),
                        "corp_name" => current_item.corp_name = Some("".to_string()),
                        "stock_code" => current_item.stock_code = Some("".to_string()),
                        "modify_date" => current_item.modify_date = Some("".to_string()),
                        field => {
                            let mut err = ValidationError::new("unexpected_field");
                            err.add_param(Cow::from("field"), &field);
                            Err(err)?;
                        }
                    }
                }
                Ok(Event::Start(ref e)) => {
                    current_field = String::from_utf8_lossy(e.name().as_ref()).to_string();
                }
                Ok(Event::Text(e)) => {
                    let text = e.unescape()?.to_string();
                    match current_field.as_str() {
                        "corp_code" => current_item.corp_code = Some(text),
                        "corp_name" => current_item.corp_name = Some(text),
                        "stock_code" => current_item.stock_code = Some(text),
                        "modify_date" => current_item.modify_date = Some(text),
                        field => {
                            let mut err = ValidationError::new("unexpected_field");
                            err.add_param(Cow::from("field"), &field);
                            Err(err)?;
                        }
                    }
                }
                Ok(Event::End(ref e)) => {
                    if String::from_utf8_lossy(e.name().as_ref()) == "list" {
                        let corp_info = CorpCodeMeta::try_from(std::mem::take(&mut current_item))?;
                        items.push(corp_info);
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => Err(e)?,
                e => {
                    let mut err = ValidationError::new("unexpected_event");
                    err.add_param(Cow::from("event"), &format!("{:?}", e));
                    Err(err)?;
                }
            }
            buf.clear();
        }

        Ok(CorpMetas(items))
    }

    pub fn iter(&self) -> std::slice::Iter<CorpCodeMeta> {
        self.0.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::test_context;
    use crate::test_utils::tracing::subscribe_tracing_with_span;
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
        let corp_infos = CorpMetas::from_zip(&mut zip).unwrap();

        assert_eq!(
            corp_infos,
            CorpMetas(vec![
                CorpCodeMeta {
                    corp_code: "00126380".to_string(),
                    corp_name: "삼성전자(주)".to_string(),
                    stock_code: "005930".to_string(),
                    modify_date: "20210531".to_string(),
                },
                CorpCodeMeta {
                    corp_code: "00164779".to_string(),
                    corp_name: "삼성전자서비스(주)".to_string(),
                    stock_code: "012057".to_string(),
                    modify_date: "20210531".to_string(),
                }
            ])
        );
    }

    #[test]
    fn read_xml_should_error_when_there_are_more_than_one_file() {
        let zip = create_mock_zip_bytes(2);

        let cursor = Cursor::new(zip);
        let mut zip = ZipArchive::new(cursor).unwrap();

        let corp_infos = CorpMetas::from_zip(&mut zip);
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
            list: Vec<CorpCodeMeta>,
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
