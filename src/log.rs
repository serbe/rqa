/// Log
///
/// All Log API methods are under "log", e.g.: /api/v2/log/methodName.
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::{
    request::{ApiRequest, Arguments, Method},
    response::check_default_status,
    Client, Error,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct GetLog {
    /// Include normal messages (default: true)
    pub normal: bool,
    /// Include info messages (default: true)
    pub info: bool,
    /// Include warning messages (default: true)
    pub warning: bool,
    /// Include critical messages (default: true)
    pub critical: bool,
    /// Exclude messages with "message id" <= last_known_id (default: -1)
    pub last_known_id: i64,
}

impl Default for GetLog {
    fn default() -> Self {
        Self {
            normal: true,
            info: true,
            warning: true,
            critical: true,
            last_known_id: -1,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogEntry {
    /// ID of the message
    pub id: i64,
    /// Text of the message
    pub message: String,
    /// Milliseconds since epoch
    pub timestamp: i64,
    /// Type of the message: Log::NORMAL: 1, Log::INFO: 2, Log::WARNING: 4, Log::CRITICAL: 8
    #[serde(rename = "type")]
    pub kind: LogType,
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum LogType {
    NORMAL = 1,
    INFO = 2,
    WARNING = 4,
    CRITICAL = 8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetPeerLog {
    /// Exclude messages with "message id" <= last_known_id (default: -1)
    pub last_known_id: i64,
}

impl Default for GetPeerLog {
    fn default() -> Self {
        Self { last_known_id: -1 }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogPeerEntry {
    /// ID of the peer
    pub id: i64,
    /// IP of the peer
    pub ip: String,
    /// Milliseconds since epoch
    pub timestamp: i64,
    /// Whether or not the peer was blocked
    pub blocked: bool,
    /// Reason of the block
    pub reason: String,
}

impl Client {
    /// Get log
    ///
    /// Name: main
    ///
    /// Parameters:
    /// Parameter  Type  Description
    /// normal  bool  Include normal messages (default: true)
    /// info  bool  Include info messages (default: true)
    /// warning  bool  Include warning messages (default: true)
    /// critical  bool  Include critical messages (default: true)
    /// last_known_id  integer  Exclude messages with "message id" <= last_known_id (default: -1)
    ///
    /// Example:
    ///
    /// /api/v2/log/main?normal=true&info=true&warning=true&critical=true&last_known_id=-1
    ///
    /// Returns:
    /// HTTP Status Code  Scenario
    /// 200  All scenarios- see JSON below
    ///
    /// Vec<LoeEntry>
    ///
    /// The response is a JSON array in which each element is an entry of the log.
    ///
    pub async fn get_log(&mut self, values: GetLog) -> Result<Vec<LogEntry>, Error> {
        let arguments = Arguments::Json(json!(values));
        let request = ApiRequest {
            method: Method::Main,
            arguments: Some(arguments),
        };
        let response = self.send_request(&request).await?;
        check_default_status(
            &response,
            serde_json::from_reader(response.body().as_ref())?,
        )
    }

    /// Get peer log
    ///
    /// Name: peers
    ///
    /// Parameters:
    /// Parameter  Type  Description
    /// last_known_id  integer  Exclude messages with "message id" <= last_known_id (default: -1)
    ///
    /// Returns:
    /// HTTP Status Code Scenario
    /// 200 All scenarios- see JSON below
    ///
    /// Vec<LogPeerEntry>
    ///
    /// The response a JSON array. Each element of the array of objects (each object is the information relative to a peer) containing the following fields
    ///
    pub async fn get_peer_log(&mut self, values: GetPeerLog) -> Result<Vec<LogPeerEntry>, Error> {
        let arguments = Arguments::Json(json!(values));
        let request = ApiRequest {
            method: Method::Peers,
            arguments: Some(arguments),
        };
        let response = self.send_request(&request).await?;
        check_default_status(
            &response,
            serde_json::from_reader(response.body().as_ref())?,
        )
    }
}
