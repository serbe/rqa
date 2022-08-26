// Sync
//
// Sync API implements requests for obtaining changes since the last request. All Sync API methods are under "sync", e.g.: /api/v2/sync/methodName.

use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    request::{ApiRequest, Arguments, Method},
    response::check_default_status,
    Client, Error,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct GetMainData {
    /// Exclude messages with "message id" <= last_known_id (default: -1)
    pub rid: i64,
}

impl Default for GetMainData {
    fn default() -> Self {
        Self { rid: 0 }
    }
}

// #[derive(Debug, Serialize, Deserialize)]
// pub struct MainData {
//     /// Response ID
//     pub rid 	:i64 	,
//     /// Whether the response contains all the data or partial data
//     pub full_update :	bool 	,
//     /// Property: torrent hash, value: same as torrent list
//     pub torrents :	Vec<Torrent> 	,
//     /// List of hashes of torrents removed since last request
//     pub torrents_removed 	array 	,
//     /// Info for categories added since last request
//     pub categories 	object 	,
//     /// List of categories removed since last request
//     pub categories_removed 	array 	,
//     /// List of tags added since last request
//     pub tags 	array 	,
//     /// List of tags removed since last request
//     pub tags_removed 	array 	,
//     /// Global transfer info
//     pub server_state 	object 	,
// }

impl Client {
    /// Get main data
    ///
    /// Name: maindata
    ///
    /// Parameters:
    /// Parameter 	Type 	Description
    /// rid 	integer 	Response ID. If not provided, rid=0 will be assumed. If the given rid is different from the one of last server reply, full_update will be true (see the server reply details for more info)
    ///
    /// Example:
    ///
    /// /api/v2/sync/maindata?rid=14
    ///
    /// Returns:
    /// HTTP Status Code	Scenario
    /// 200	All scenarios- see JSON below
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
    pub async fn get_main_data(&mut self, values: GetMainData) -> Result<String, Error> {
        let arguments = Arguments::Json(json!(values));
        let request = ApiRequest {
            method: Method::MainData,
            arguments: Some(arguments),
        };
        let response = self.send_request(&request).await?;
        check_default_status(&response, String::from_utf8(response.body().to_vec())?)
    }

    /// Get torrent peers data
    ///
    /// Name: torrentPeers
    ///
    /// Parameters:
    /// Parameter 	Type 	Description
    /// hash 	string 	Torrent hash
    /// rid 	integer 	Response ID. If not provided, rid=0 will be assumed. If the given rid is different from the one of last server reply, full_update will be true (see the server reply details for more info)
    ///
    /// Example:
    ///
    /// /api/v2/sync/torrentPeers?hash=8c212779b4abde7c6bc608063a0d008b7e40ce32?rid=14
    ///
    /// Returns:
    /// HTTP Status Code 	Scenario
    /// 404 	Torrent hash was not found
    /// 200 	All other scenarios- see JSON below
    ///
    /// String
    ///
    /// The response is TODO
    pub async fn get_peer_data(&mut self, values: GetMainData) -> Result<String, Error> {
        let arguments = Arguments::Json(json!(values));
        let request = ApiRequest {
            method: Method::MainData,
            arguments: Some(arguments),
        };
        let response = self.send_request(&request).await?;
        check_default_status(&response, String::from_utf8(response.body().to_vec())?)
    }
}
