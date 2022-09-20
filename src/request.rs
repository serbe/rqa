use std::convert::From;
use std::fmt;

use bytes::Bytes;
use netc::{Response, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::client::Client;
use crate::error::Error;

// use crate::response::{BlocklistUpdate, FreeSpace, PortTest, RpcResponse};

pub struct ApiRequest {
    pub method: Method,
    pub arguments: Option<Arguments>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub tag: Option<Ids>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Arguments {
    Json(Value),
    Form(String),
}

// #[derive(Debug, Serialize, Deserialize)]
// #[serde(untagged)]
// pub enum Id {
//     Id(i64),
//     Hash(String),
// }

// impl From<i64> for Id {
//     fn from(id: i64) -> Self {
//         Id::Id(id)
//     }
// }

// impl From<&str> for Id {
//     fn from(hash: &str) -> Self {
//         Id::Hash(hash.to_string())
//     }
// }

// impl From<String> for Id {
//     fn from(hash: String) -> Self {
//         Id::Hash(hash)
//     }
// }

// #[derive(Debug, Deserialize)]
// pub enum Ids {
//     Id(i64),
//     Array(Vec<Id>),
//     RecentlyActive,
// }

// impl Default for Ids {
//     fn default() -> Self {
//         Ids::RecentlyActive
//     }
// }

// impl From<i64> for Ids {
//     fn from(id: i64) -> Self {
//         Ids::Id(id)
//     }
// }

// impl From<Vec<Id>> for Ids {
//     fn from(values: Vec<Id>) -> Self {
//         let mut arr = Vec::new();
//         for value in values {
//             arr.push(value);
//         }
//         Ids::Array(arr)
//     }
// }

// impl Serialize for Ids {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         match self {
//             Ids::Id(id) => serializer.serialize_i64(*id),
//             Ids::Array(values) => values.serialize(serializer),
//             Ids::RecentlyActive => serializer.serialize_str("recently-active"),
//         }
//     }
// }

// pub fn value_from_response(response: RpcResponse) -> Result<Value, Error> {
//     if &response.result == "success" {
//         Ok(response.arguments.map_or(Err(Error::NoArguments), Ok)?)
//     } else {
//         Err(Error::BadResponse(response.result))
//     }
// }

#[derive(Eq, PartialEq)]
pub enum Method {
    Login,
    Logout,
    Version,
    WebapiVersion,
    BuildInfo,
    Shutdown,
    Preferences,
    SetPreferences,
    DefaultSavePath,
    Main,
    Peers,
    MainData,
    TorrentPeers,
    TransferInfo,
    SpeedLimitsMode,
    ToggleSpeedLimitsMode,
    DownloadLimit,
    SetDownloadLimit,
    UploadLimit,
    SetUploadLimit,
    BanPeers,
    TorrentsInfo,
    Properties,
    Trackers,
    Webseeds,
    Files,
    PieceStates,
    PieceHashes,
    Pause,
    Resume,
    Delete,
    Recheck,
    Reannounce,
    Add,
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Method::Login => write!(f, "auth/login"),
            Method::Logout => write!(f, "auth/logout"),
            Method::Version => write!(f, "app/version"),
            Method::WebapiVersion => write!(f, "app/webapiVersion"),
            Method::BuildInfo => write!(f, "app/buildInfo"),
            Method::Shutdown => write!(f, "app/shutdown"),
            Method::Preferences => write!(f, "app/preferences"),
            Method::SetPreferences => write!(f, "app/setPreferences"),
            Method::DefaultSavePath => write!(f, "app/defaultSavePath"),
            Method::Main => write!(f, "log/main"),
            Method::Peers => write!(f, "log/peers"),
            Method::MainData => write!(f, "sync/maindata"),
            Method::TorrentPeers => write!(f, "sync/torrentPeers"),
            Method::TransferInfo => write!(f, "transfer/info"),
            Method::SpeedLimitsMode => write!(f, "transfer/speedLimitsMode"),
            Method::ToggleSpeedLimitsMode => write!(f, "transfer/toggleSpeedLimitsMode"),
            Method::DownloadLimit => write!(f, "transfer/downloadLimit"),
            Method::SetDownloadLimit => write!(f, "transfer/setDownloadLimit"),
            Method::UploadLimit => write!(f, "transfer/uploadLimit"),
            Method::SetUploadLimit => write!(f, "transfer/setUploadLimit"),
            Method::BanPeers => write!(f, "transfer/banPeers"),
            Method::TorrentsInfo => write!(f, "torrents/info"),
            Method::Properties => write!(f, "torrents/properties"),
            Method::Trackers => write!(f, "torrents/trackers"),
            Method::Webseeds => write!(f, "torrents/webseeds"),
            Method::Files => write!(f, "torrents/files"),
            Method::PieceStates => write!(f, "torrents/pieceStates"),
            Method::PieceHashes => write!(f, "torrents/pieceHashes"),
            Method::Pause => write!(f, "torrents/pause"),
            Method::Resume => write!(f, "torrents/resume"),
            Method::Delete => write!(f, "torrents/delete"),
            Method::Recheck => write!(f, "torrents/recheck"),
            Method::Reannounce => write!(f, "torrents/reannounce"),
            Method::Add => write!(f, "torrents/add"),
        }
    }
}

impl Client {
    pub async fn send_request(&mut self, input: &ApiRequest) -> Result<Response, Error> {
        let body = match &input.arguments {
            Some(Arguments::Json(value)) => {
                let mut buf = vec![];
                serde_json::to_writer(&mut buf, value)?;
                buf.into()
            }
            Some(Arguments::Form(value)) => {
                let body = value.clone();
                body.into()
            }
            None => Bytes::new(),
        };
        let response = self.get_response(&input.method.to_string(), &body).await?;
        if input.method == Method::Login && response.status_code() == StatusCode::from(200) {
            let set_cookie = response
                .headers
                .get("set-cookie")
                .ok_or(Error::NoSetCookie)?;
            let cookie = set_cookie.split(';').next().ok_or(Error::NoSID)?;
            self.cookie = cookie.to_string();
        }
        Ok(response)
    }
    //     pub async fn blocklist_update(&mut self) -> Result<BlocklistUpdate, Error> {
    //         let request = RpcRequest {
    //             method: Method::BlocklistUpdate,
    //             arguments: None,
    //             tag: None,
    //         };
    //         let response = self.send_msg(&request).await?;
    //         let parsed_value = serde_json::from_value(value_from_response(response)?)?;
    //         Ok(parsed_value)
    //     }

    //     pub async fn port_test(&mut self) -> Result<PortTest, Error> {
    //         let request = RpcRequest {
    //             method: Method::PortTest,
    //             arguments: None,
    //             tag: None,
    //         };
    //         let response = self.send_msg(&request).await?;
    //         let parsed_value = serde_json::from_value(value_from_response(response)?)?;
    //         Ok(parsed_value)
    //     }

    //     pub async fn free_space(&mut self, path: &str) -> Result<FreeSpace, Error> {
    //         let request = RpcRequest {
    //             method: Method::FreeSpace,
    //             arguments: Some(json!({"path": path.to_string()})),
    //             tag: None,
    //         };
    //         let response = self.send_msg(&request).await?;
    //         let parsed_value = serde_json::from_value(value_from_response(response)?)?;
    //         Ok(parsed_value)
    //     }

    //     pub async fn queue_move_top(&mut self, args: Option<Ids>) -> Result<(), Error> {
    //         let value = args.map(|args| json!(args));
    //         let request = RpcRequest {
    //             method: Method::QueueMoveTop,
    //             arguments: value,
    //             tag: None,
    //         };
    //         let response = self.send_msg(&request).await?;
    //         let _ = value_from_response(response)?;
    //         Ok(())
    //     }

    //     pub async fn queue_move_up(&mut self, args: Option<Ids>) -> Result<(), Error> {
    //         let value = args.map(|args| json!(args));
    //         let request = RpcRequest {
    //             method: Method::QueueMoveUp,
    //             arguments: value,
    //             tag: None,
    //         };
    //         let response = self.send_msg(&request).await?;
    //         let _ = value_from_response(response)?;
    //         Ok(())
    //     }

    //     pub async fn queue_move_down(&mut self, args: Option<Ids>) -> Result<(), Error> {
    //         let value = args.map(|args| json!(args));
    //         let request = RpcRequest {
    //             method: Method::QueueMoveDown,
    //             arguments: value,
    //             tag: None,
    //         };
    //         let response = self.send_msg(&request).await?;
    //         let _ = value_from_response(response)?;
    //         Ok(())
    //     }

    //     pub async fn queue_move_bottom(&mut self, args: Option<Ids>) -> Result<(), Error> {
    //         let value = args.map(|args| json!(args));
    //         let request = RpcRequest {
    //             method: Method::QueueMoveBottom,
    //             arguments: value,
    //             tag: None,
    //         };
    //         let response = self.send_msg(&request).await?;
    //         let _ = value_from_response(response)?;
    //         Ok(())
    //     }
}
