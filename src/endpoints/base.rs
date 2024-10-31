use crate::error::MessageError;
use crate::statics::assert_impl_commons_without_default;
use derive_more::{Display, From, Into};
use reqwest::header::HeaderMap;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

// region: Crate response

#[derive(Debug)]
pub struct OpenDartResponse<B>
where
    B: Serialize + ResponseCheck,
{
    status: StatusCode,
    _header_map: HeaderMap,
    pub body: Option<B>,
}

impl<B> OpenDartResponse<ResponseBody<B>>
where
    B: Serialize + ResponseCheck,
{
    pub fn new(status: StatusCode, header_map: HeaderMap, body: Option<ResponseBody<B>>) -> Self {
        Self {
            status,
            _header_map: header_map,
            body,
        }
    }

    pub fn status(&self) -> StatusCode {
        self.status
    }
}

// endregion: Crate response

// region: OpenDart external api response type

/// This is the body of an OpenDart response.
///
/// When a successful response (code 000) is received,
/// the message content will be flattened into the body.
///
/// ```json
/// {
///     "status": "000",
///     "message": "정상",
///     "some_body": {
///         // ...
///     }
/// }
/// ```
///
/// When an unsuccessful response (code not 000) is received,
/// the message will be received as a separate field.
///
/// ```json
/// {
///    "message": {
///         "status": "010",
///         "message": "등록되지 않은 키입니다."
///    }
/// }
/// ```
#[derive(
    Debug,
    Clone,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    // derive_more
    Display,
    // serde
    Serialize,
    Deserialize,
)]
#[serde(deny_unknown_fields)]
pub enum ResponseBody<B: Serialize + ResponseCheck> {
    #[serde(rename = "message")]
    Message(Message),
    #[serde(untagged)]
    Body(B),
    #[serde(untagged)]
    UnTaggedMessage(Message),
}

impl<B> ResponseCheck for ResponseBody<B>
where
    B: Serialize + ResponseCheck,
{
    fn is_success(&self) -> Result<(), MessageError> {
        match self {
            ResponseBody::Message(message) => message.is_success(),
            ResponseBody::UnTaggedMessage(message) => message.is_success(),
            ResponseBody::Body(body) => body.is_success(),
        }
    }
}

// endregion: OpenDart external api response type

// region: Message

assert_impl_commons_without_default!(Message);

/// This is the message returned from an OpenDart 200 response.
#[derive(
    Debug,
    Clone,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    // derive_more
    Display,
    From,
    Into,
    // serde
    Serialize,
    Deserialize,
)]
#[display("{self:?}")]
#[serde(deny_unknown_fields)]
pub struct Message {
    /// ### 에러 및 정보 코드
    /// (※메시지 설명 참조)
    pub status: String,

    /// ### 에러 및 정보 메시지
    /// (※메시지 설명 참조)
    pub message: String,
}

impl ResponseCheck for Message {
    fn is_success(&self) -> Result<(), MessageError> {
        is_success(&self.status)
    }
}

pub(crate) fn is_success(status: &str) -> Result<(), MessageError> {
    if status == "000" {
        Ok(())
    } else {
        Err(MessageError {
            message: Message {
                status: status.to_string(),
                message: "".to_string(),
            },
        })
    }
}

// endregion: Message

pub trait ResponseCheck {
    fn is_success(&self) -> Result<(), MessageError>;
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use serde_json::json;

    #[test]
    fn untagged_enums_should_deserialize_as_expected() {
        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        #[serde(deny_unknown_fields)]
        struct Message {
            status: String,
            message: String,
        }

        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        #[serde(deny_unknown_fields)]
        struct WithMessage {
            status: String,
            message: String,
            content: String,
        }

        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        #[serde(deny_unknown_fields)]
        enum ResponseBody {
            #[serde(rename = "message")]
            Message(Message),
            #[serde(untagged)]
            UnTaggedMessage(Message),
            #[serde(untagged)]
            WithMessage(WithMessage),
        }

        let tagged = json!({
            "message": {
                "status": "010",
                "message": "등록되지 않은 키입니다."
            }
        });

        let untagged = json!({
            "status": "010",
            "message": "등록되지 않은 키입니다."
        });

        let with_message = json!({
            "status": "010",
            "message": "등록되지 않은 키입니다.",
            "content": "This is the content"
        });

        let tagged_deserialized: ResponseBody = serde_json::from_value(tagged).unwrap();
        let untagged_deserialized: ResponseBody = serde_json::from_value(untagged).unwrap();
        let with_message_deserialized: ResponseBody = serde_json::from_value(with_message).unwrap();

        assert_eq!(
            tagged_deserialized,
            ResponseBody::Message(Message {
                status: "010".to_string(),
                message: "등록되지 않은 키입니다.".to_string(),
            })
        );

        assert_eq!(
            untagged_deserialized,
            ResponseBody::UnTaggedMessage(Message {
                status: "010".to_string(),
                message: "등록되지 않은 키입니다.".to_string(),
            })
        );

        assert_eq!(
            with_message_deserialized,
            ResponseBody::WithMessage(WithMessage {
                status: "010".to_string(),
                message: "등록되지 않은 키입니다.".to_string(),
                content: "This is the content".to_string(),
            })
        );
    }
}
