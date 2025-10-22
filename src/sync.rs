// Sync
//
// Sync API implements requests for obtaining changes since the last request. All Sync API methods are under "sync", e.g.: /api/v2/sync/methodName.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    Client, Error,
    request::{ApiRequest, Arguments, Method},
    response::check_default_status,
    torrents::Torrent,
    transfer::ConnectionStatus,
};

#[derive(Debug, Serialize, Default, Deserialize)]
pub struct GetMainData {
    /// Exclude messages with "message id" <= last_known_id (default: -1)
    pub rid: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetPeersData {
    /// Torrent hash
    pub hash: String,
    /// Response ID. If not provided, rid=0 will be assumed. If the given rid is different from the one of last server reply, full_update will be true (see the server reply details for more info)
    pub rid: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MainData {
    /// Response ID
    pub rid: i64,
    /// Whether the response contains all the data or partial data
    pub full_update: bool,
    /// Property: torrent hash, value: same as torrent list
    pub torrents: HashMap<String, Torrent>,
    /// List of hashes of torrents removed since last request
    pub torrents_removed: Option<Vec<String>>,
    /// Info for categories added since last request
    pub categories: HashMap<String, Category>,
    /// List of categories removed since last request
    pub categories_removed: Option<Vec<String>>,
    /// List of tags added since last request
    pub tags: Option<Vec<String>>,
    /// List of tags removed since last request
    pub tags_removed: Option<Vec<String>>,
    /// Global transfer info
    pub server_state: ServerState,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    /// Category name
    pub name: String,
    /// Save torrent to the given directory
    pub save_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerState {
    /// Global download rate (bytes/s)
    pub dl_info_speed: i64,
    /// Data downloaded this session (bytes)
    pub dl_info_data: i64,
    /// Global upload rate (bytes/s)
    pub up_info_speed: i64,
    /// Data uploaded this session (bytes)
    pub up_info_data: i64,
    /// Download rate limit (bytes/s)
    pub dl_rate_limit: i64,
    /// Upload rate limit (bytes/s)
    pub up_rate_limit: i64,
    /// DHT nodes connected to
    pub dht_nodes: i64,
    /// Connection status. See possible values here below
    pub connection_status: ConnectionStatus,
    /// True if torrent queueing is enabled
    pub queueing: bool,
    /// True if alternative speed limits are enabled
    pub use_alt_speed_limits: bool,
    /// Transfer list refresh interval (milliseconds)
    pub refresh_interval: i64,
}

impl Client {
    /// Get main data
    ///
    /// Name: maindata
    ///
    /// Parameters:
    /// Parameter  Type  Description
    /// rid  integer  Response ID. If not provided, rid=0 will be assumed. If the given rid is different from the one of last server reply, full_update will be true (see the server reply details for more info)
    ///
    /// Example:
    ///
    /// /api/v2/sync/maindata?rid=14
    ///
    /// Returns:
    /// HTTP Status Code Scenario
    /// 200 All scenarios- see JSON below
    ///
    /// MainData
    ///
    /// Example:
    ///
    /// {
    ///     "rid":15,
    ///     "torrents":
    ///     {
    ///         "8c212779b4abde7c6bc608063a0d008b7e40ce32":
    ///         {
    ///             "state":"pausedUP"
    ///         }
    ///     }
    /// }
    ///
    pub async fn get_main_data(&mut self, values: GetMainData) -> Result<MainData, Error> {
        let arguments = Arguments::Json(json!(values));
        let request = ApiRequest {
            method: Method::MainData,
            arguments: Some(arguments),
        };
        let response = self.send_request(&request).await?;
        check_default_status(
            &response,
            serde_json::from_reader(response.body().as_ref())?,
        )
    }

    /// Get torrent peers data
    ///
    /// Name: torrentPeers
    ///
    /// Parameters:
    /// Parameter  Type  Description
    /// hash  string  Torrent hash
    /// rid  integer  Response ID. If not provided, rid=0 will be assumed. If the given rid is different from the one of last server reply, full_update will be true (see the server reply details for more info)
    ///
    /// Example:
    ///
    /// /api/v2/sync/torrentPeers?hash=8c212779b4abde7c6bc608063a0d008b7e40ce32?rid=14
    ///
    /// Returns:
    /// HTTP Status Code  Scenario
    /// 404  Torrent hash was not found
    /// 200  All other scenarios- see JSON below
    ///
    /// String
    ///
    /// The response is TODO
    pub async fn get_peers_data(&mut self, values: GetPeersData) -> Result<String, Error> {
        let arguments = Arguments::Json(json!(values));
        let request = ApiRequest {
            method: Method::TorrentPeers,
            arguments: Some(arguments),
        };
        let response = self.send_request(&request).await?;
        match response.status_code().as_u16() {
            200 => Ok(String::from_utf8(response.body().to_vec())?),
            404 => Err(Error::NoTorrentHash),
            _ => Err(Error::WrongStatusCode),
        }
    }
}
