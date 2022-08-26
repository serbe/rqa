use bytes::Bytes;
use netc::Response;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use url::Url;

use crate::{error::Error, Client};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse {
    pub result: String,
    pub arguments: Option<Value>,
    // pub tag: Option<i64>,
}

// #[derive(Debug, Serialize, Deserialize)]
// pub struct PortTest {
//     #[serde(rename = "port-is-open")]
//     pub port_is_open: bool,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct BlocklistUpdate {
//     #[serde(rename = "blocklist-size")]
//     pub blocklist_size: i64,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct FreeSpace {
//     pub path: String,
//     #[serde(rename = "size-bytes")]
//     pub size_bytes: i64,
// }

// pub fn value_from_response(response: RpcResponse) -> Result<Value, Error> {
//     if &response.result == "success" {
//         Ok(response.arguments.map_or(Err(Error::NoArguments), Ok)?)
//     } else {
//         Err(Error::BadResponse(response.result))
//     }
// }

impl Client {
    pub(crate) async fn get_response(&self, method: &str, body: &Bytes) -> Result<Response, Error> {
        let cb = netc::Client::builder();
        let options = Url::options();
        let base_url = options.base_url(Some(&self.url));
        let url = base_url.parse(method)?;
        let mut client = cb
            .post(&url)
            .header("Cache-Control", "no-cache")
            .header("Pragma", "no-cache")
            .header("Cookie", &self.cookie)
            .content_type("application/x-www-form-urlencoded; charset=utf-8")
            .origin(&self.url.origin().ascii_serialization())
            .body(body.clone())
            .build()
            .await?;
        Ok(client.send().await?)
    }
}

pub(crate) fn check_default_status<T>(response: &Response, value: T) -> Result<T, Error> {
    match response.status_code().as_u16() {
        200 => Ok(value),
        _ => Err(Error::WrongStatusCode),
    }
}
