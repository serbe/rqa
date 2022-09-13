use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::{
    request::{ApiRequest, Arguments, Method},
    response::check_default_status,
    Client, Error,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct TransferInfo {
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
}

// In addition to the above in partial data requests (see Get partial data for more info):

// Property	Type	Description
// queueing	bool	True if torrent queueing is enabled
// use_alt_speed_limits	bool	True if alternative speed limits are enabled
// refresh_interval	integer	Transfer list refresh interval (milliseconds)
// Possible values of connection_status:

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ConnectionStatus {
    Connected,
    Firewalled,
    Disconnected,
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum AltSpeedState {
    Disabled = 0,
    Enabled = 1,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Limit {
    limit: i64,
}

impl Client {
    /// Get global transfer info
    /// This method returns info you usually see in qBt status bar.
    ///
    /// Name: info
    ///
    /// Parameters:
    ///
    /// None
    ///
    /// Returns:
    ///
    /// HTTP Status Code	Scenario
    /// 200	All scenarios- see JSON below
    ///
    /// TransferInfo
    ///
    pub async fn get_transfer_info(&mut self) -> Result<TransferInfo, Error> {
        let request = ApiRequest {
            method: Method::Info,
            arguments: None,
        };
        let response = self.send_request(&request).await?;
        check_default_status(
            &response,
            serde_json::from_reader(response.body().as_ref())?,
        )
    }

    /// Get alternative speed limits state
    /// Name: speedLimitsMode
    ///
    /// Parameters:
    ///
    /// None
    ///
    /// Returns:
    ///
    /// HTTP Status Code	Scenario
    /// 200	All scenarios
    ///
    /// The response is 1 if alternative speed limits are enabled, 0 otherwise.
    ///
    pub async fn get_alt_speed_state(&mut self) -> Result<AltSpeedState, Error> {
        let request = ApiRequest {
            method: Method::SpeedLimitsMode,
            arguments: None,
        };
        let response = self.send_request(&request).await?;
        check_default_status(
            &response,
            serde_json::from_reader(response.body().as_ref())?,
        )
    }

    /// Toggle alternative speed limits
    /// Name: toggleSpeedLimitsMode
    ///
    /// Parameters:
    ///
    /// None
    ///
    /// Returns:
    ///
    /// HTTP Status Code	Scenario
    /// 200	All scenarios
    ///
    pub async fn toggle_alt_speed(&mut self) -> Result<(), Error> {
        let request = ApiRequest {
            method: Method::ToggleSpeedLimitsMode,
            arguments: None,
        };
        let response = self.send_request(&request).await?;
        check_default_status(&response, ())
    }

    /// Get global download limit
    /// Name: downloadLimit
    ///
    /// Parameters:
    ///
    /// None
    ///
    /// Returns:
    ///
    /// HTTP Status Code	Scenario
    /// 200	All scenarios
    /// The response is the value of current global download speed limit in bytes/second; this value will be zero if no limit is applied.
    ///
    pub async fn get_download_limit(&mut self) -> Result<i64, Error> {
        let request = ApiRequest {
            method: Method::DownloadLimit,
            arguments: None,
        };
        let response = self.send_request(&request).await?;
        check_default_status(
            &response,
            String::from_utf8(response.body().to_vec())?.parse()?,
        )
    }

    /// Set global download limit
    /// Name: setDownloadLimit
    ///
    /// Parameters:
    ///
    /// Parameter	Type	Description
    /// limit	integer	The global download speed limit to set in bytes/second
    /// Returns:
    ///
    /// HTTP Status Code	Scenario
    /// 200	All scenarios
    ///
    pub async fn set_download_limit(&mut self, limit: i64) -> Result<(), Error> {
        let request = ApiRequest {
            method: Method::SetDownloadLimit,
            arguments: Some(Arguments::Form(format!("limit={}", limit))),
        };
        let response = self.send_request(&request).await?;
        check_default_status(&response, ())
    }

    /// Get global upload limit
    /// Name: uploadLimit
    ///
    /// Parameters:
    ///
    /// None
    ///
    /// Returns:
    ///
    /// HTTP Status Code	Scenario
    /// 200	All scenarios
    /// The response is the value of current global upload speed limit in bytes/second; this value will be zero if no limit is applied.
    ///
    pub async fn get_upload_limit(&mut self) -> Result<i64, Error> {
        let request = ApiRequest {
            method: Method::UploadLimit,
            arguments: None,
        };
        let response = self.send_request(&request).await?;
        check_default_status(
            &response,
            String::from_utf8(response.body().to_vec())?.parse()?,
        )
    }

    /// Set global upload limit
    /// Name: setUploadLimit
    ///
    /// Parameters:
    ///
    /// Parameter	Type	Description
    /// limit	integer	The global upload speed limit to set in bytes/second
    /// Returns:
    ///
    /// HTTP Status Code	Scenario
    /// 200	All scenarios
    pub async fn set_upload_limit(&mut self, limit: i64) -> Result<(), Error> {
        let request = ApiRequest {
            method: Method::SetUploadLimit,
            arguments: Some(Arguments::Form(format!("limit={}", limit))),
        };
        let response = self.send_request(&request).await?;
        check_default_status(&response, ())
    }

    /// Ban peers
    /// Name: banPeers
    ///
    /// Parameters:
    ///
    /// Parameter	Type	Description
    /// peers	string	The peer to ban, or multiple peers separated by a pipe |. Each peer is a colon-separated host:port
    /// Returns:
    ///
    /// HTTP Status Code	Scenario
    /// 200	All scenarios
    ///
    pub async fn ban_peers(&mut self, value: String) -> Result<String, Error> {
        let request = ApiRequest {
            method: Method::BanPeers,
            arguments: Some(Arguments::Form(value)),
        };
        let response = self.send_request(&request).await?;
        check_default_status(&response, String::from_utf8(response.body().to_vec())?)
    }
}
