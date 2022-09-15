use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::{
    client::Client,
    error::Error,
    request::{ApiRequest, Arguments, Method},
    response::check_default_status,
};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetTorrentList {
    /// Filter torrent list by state. Allowed state filters: all, downloading, seeding, completed, paused, active, inactive, resumed, stalled, stalled_uploading, stalled_downloading, errored
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
    /// Get torrents with the given category (empty string means "without category"; no "category" parameter means "any category" <- broken until #11748 is resolved). Remember to URL-encode the category name. For example, My category becomes My%20category
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// Get torrents with the given tag (empty string means "without tag"; no "tag" parameter means "any tag". Remember to URL-encode the category name. For example, My tag becomes My%20tag
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// Sort torrents by given key. They can be sorted using any field of the response's JSON array (which are documented below) as the sort key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    /// Enable reverse sorting. Defaults to false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverse: Option<bool>,
    /// Limit the number of torrents returned
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Set offset (if less than 0, offset from end)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// Filter by hashes. Can contain multiple hashes separated by |
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hashes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Torrent {
    /// Time (Unix Epoch) when the torrent was added to the client
    pub added_on: i64,
    /// Amount of data left to download (bytes)
    pub amount_left: i64,
    /// Whether this torrent is managed by Automatic Torrent Management
    pub auto_tmm: bool,
    /// Percentage of file pieces currently available
    pub availability: Option<f64>,
    /// Category of the torrent
    pub category: String,
    /// Amount of transfer data completed (bytes)
    pub completed: i64,
    /// Time (Unix Epoch) when the torrent completed
    pub completion_on: i64,
    /// Torrent download speed limit (bytes/s). -1 if ulimited.
    pub dl_limit: i64,
    /// Torrent download speed (bytes/s)
    pub dlspeed: i64,
    /// Amount of data downloaded
    pub downloaded: i64,
    /// Amount of data downloaded this session
    pub downloaded_session: i64,
    /// Torrent ETA (seconds)
    pub eta: i64,
    /// True if first last piece are prioritized
    pub f_l_piece_prio: bool,
    /// True if force start is enabled for this torrent
    pub force_start: bool,
    /// Torrent hash
    pub hash: Option<String>,
    /// Last time (Unix Epoch) when a chunk was downloaded/uploaded
    pub last_activity: i64,
    /// Magnet URI corresponding to this torrent
    pub magnet_uri: String,
    /// Maximum share ratio until torrent is stopped from seeding/uploading
    pub max_ratio: f64,
    /// Maximum seeding time (seconds) until torrent is stopped from seeding
    pub max_seeding_time: i64,
    /// Torrent name
    pub name: String,
    /// Number of seeds in the swarm
    pub num_complete: i64,
    /// Number of leechers in the swarm
    pub num_incomplete: i64,
    /// Number of leechers connected to
    pub num_leechs: i64,
    /// Number of seeds connected to
    pub num_seeds: i64,
    /// Torrent priority. Returns -1 if queuing is disabled or torrent is in seed mode
    pub priority: i64,
    /// Torrent progress (percentage/100)
    pub progress: f64,
    /// Torrent share ratio. Max ratio value: 9999.
    pub ratio: f64,
    /// TODO (what is different from max_ratio?)
    pub ratio_limit: f64,
    /// Path where this torrent's data is stored
    pub save_path: String,
    /// TODO (what is different from max_seeding_time?)
    pub seeding_time_limit: i64,
    /// Time (Unix Epoch) when this torrent was last seen complete
    pub seen_complete: i64,
    /// True if sequential download is enabled
    pub seq_dl: bool,
    /// Total size (bytes) of files selected for download
    pub size: i64,
    /// Torrent state. See table here below for the possible values
    pub state: String,
    /// True if super seeding is enabled
    pub super_seeding: bool,
    /// Comma-concatenated tag list of the torrent
    pub tags: String,
    /// Total active time (seconds)
    pub time_active: i64,
    /// Total size (bytes) of all file in this torrent (including unselected ones)
    pub total_size: i64,
    /// The first tracker with working status. Returns empty : String, if no tracker is working.
    pub tracker: String,
    /// Torrent upload speed limit (bytes/s). -1 if ulimited.
    pub up_limit: i64,
    /// Amount of data uploaded
    pub uploaded: i64,
    /// Amount of data uploaded this session
    pub uploaded_session: i64,
    /// Torrent upload speed (bytes/s)
    pub upspeed: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum State {
    /// Some error occurred, applies to paused torrents
    Error,
    /// Torrent data files is missing
    MissingFiles,
    /// Torrent is being seeded and data is being transferred
    Uploading,
    /// Torrent is paused and has finished downloading
    PausedUP,
    /// Queuing is enabled and torrent is queued for upload
    QueuedUP,
    /// Torrent is being seeded, but no connection were made
    StalledUP,
    /// Torrent has finished downloading and is being checked
    CheckingUP,
    /// Torrent is forced to uploading and ignore queue limit
    ForcedUP,
    /// Torrent is allocating disk space for download
    Allocating,
    /// Torrent is being downloaded and data is being transferred
    Downloading,
    /// Torrent has just started downloading and is fetching metadata
    MetaDL,
    /// Torrent is paused and has NOT finished downloading
    PausedDL,
    /// Queuing is enabled and torrent is queued for download
    QueuedDL,
    /// Torrent is being downloaded, but no connection were made
    StalledDL,
    /// Same as checkingUP, but torrent has NOT finished downloading
    CheckingDL,
    /// Torrent is forced to downloading to ignore queue limit
    ForceDL,
    /// Checking resume data on qBt startup
    CheckingResumeData,
    /// Torrent is moving to another location
    Moving,
    /// Unknown status
    Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TorrentProperties {
    /// Torrent save path
    pub save_path: String,
    /// Torrent creation date (Unix timestamp)
    pub creation_date: i64,
    /// Torrent piece size (bytes)
    pub piece_size: i64,
    /// Torrent comment
    pub comment: String,
    /// Total data wasted for torrent (bytes)
    pub total_wasted: i64,
    /// Total data uploaded for torrent (bytes)
    pub total_uploaded: i64,
    /// Total data uploaded this session (bytes)
    pub total_uploaded_session: i64,
    /// Total data downloaded for torrent (bytes)
    pub total_downloaded: i64,
    /// Total data downloaded this session (bytes)
    pub total_downloaded_session: i64,
    /// Torrent upload limit (bytes/s)
    pub up_limit: i64,
    /// Torrent download limit (bytes/s)
    pub dl_limit: i64,
    /// Torrent elapsed time (seconds)
    pub time_elapsed: i64,
    /// Torrent elapsed time while complete (seconds)
    pub seeding_time: i64,
    /// Torrent connection count
    pub nb_connections: i64,
    /// Torrent connection count limit
    pub nb_connections_limit: i64,
    /// Torrent share ratio
    pub share_ratio: f64,
    /// When this torrent was added (unix timestamp)
    pub addition_date: i64,
    /// Torrent completion date (unix timestamp)
    pub completion_date: i64,
    /// Torrent creator
    pub created_by: String,
    /// Torrent average download speed (bytes/second)
    pub dl_speed_avg: i64,
    /// Torrent download speed (bytes/second)
    pub dl_speed: i64,
    /// Torrent ETA (seconds)
    pub eta: i64,
    /// Last seen complete date (unix timestamp)
    pub last_seen: i64,
    /// Number of peers connected to
    pub peers: i64,
    /// Number of peers in the swarm
    pub peers_total: i64,
    /// Number of pieces owned
    pub pieces_have: i64,
    /// Number of pieces of the torrent
    pub pieces_num: i64,
    /// Number of seconds until the next announce
    pub reannounce: i64,
    /// Number of seeds connected to
    pub seeds: i64,
    /// Number of seeds in the swarm
    pub seeds_total: i64,
    /// Torrent total size (bytes)
    pub total_size: i64,
    /// Torrent average upload speed (bytes/second)
    pub up_speed_avg: i64,
    /// Torrent upload speed (bytes/second)
    pub up_speed: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tracker {
    /// Tracker url
    pub url: String,
    /// Tracker status. See the table below for possible values
    pub status: i64,
    /// Tracker priority tier. Lower tier trackers are tried before higher tiers. Tier numbers are valid when >= 0, < 0 is used as placeholder when tier does not exist for special entries (such as DHT).
    pub tier: Tier,
    /// Number of peers for current torrent, as reported by the tracker
    pub num_peers: i64,
    /// Number of seeds for current torrent, asreported by the tracker
    pub num_seeds: i64,
    /// Number of leeches for current torrent, as reported by the tracker
    pub num_leeches: i64,
    /// Number of completed downlods for current torrent, as reported by the tracker
    pub num_downloaded: i64,
    /// Tracker message (there is no way of knowing what this message is - it's up to tracker admins)
    pub msg: String,
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum TrackerStatus {
    /// Tracker is disabled (used for DHT, PeX, and LSD)
    Disabled = 0,
    /// Tracker has not been contacted yet
    NotContacted = 1,
    /// Tracker has been contacted and is working
    Working = 2,
    /// Tracker is updating
    Updating = 3,
    /// Tracker has been contacted, but it is not working (or doesn't send proper replies)
    NotWorking = 4,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Tier {
    None(String),
    Priority(i64),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Webseed {
    /// URL of the web seed
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct File {
    /// File index
    pub index: Option<i64>,
    /// File name (including relative path)
    pub name: String,
    /// File size (bytes)
    pub size: i64,
    /// File progress (percentage/100)
    pub progress: f64,
    /// File priority. See possible values here below
    pub priority: i64,
    /// True if file is seeding/complete
    pub is_seed: Option<bool>,
    /// The first number is the starting piece index and the second number is the ending piece index (inclusive)
    pub piece_range: Vec<i64>,
    /// Percentage of file pieces currently available (percentage/100)
    pub availability: f64,
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum Priority {
    /// Do not download
    Skip = 0,
    /// Normal priority
    Normal = 1,
    /// High priority
    High = 6,
    /// Maximal priority
    Maximum = 7,
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum PieceState {
    /// Not downloaded yet
    NotDownloadedYet = 0,
    /// Now downloading
    NowDownloading = 1,
    /// Already downloaded
    AlreadyDownloaded = 2,
}

impl Client {
    /// Get torrent list
    /// Name: info
    ///
    /// Parameters:
    ///
    /// GetTorrentList
    ///
    /// Example:
    ///
    /// /api/v2/torrents/info?filter=downloading&category=sample%20category&sort=ratio
    /// Returns:
    ///
    /// HTTP Status Code	Scenario
    /// 200	All scenarios- see JSON below
    ///
    /// array of Torrent
    ///
    pub async fn get_torrent_list(
        &mut self,
        values: GetTorrentList,
    ) -> Result<Vec<Torrent>, Error> {
        let arguments = Arguments::Json(json!(values));
        let request = ApiRequest {
            method: Method::TorrentsInfo,
            arguments: Some(arguments),
        };
        let response = self.send_request(&request).await?;
        check_default_status(
            &response,
            serde_json::from_reader(response.body().as_ref())?,
        )
    }

    /// Get torrent generic properties
    /// Requires knowing the torrent hash. You can get it from torrent list.
    ///
    /// Name: properties
    ///
    /// Parameters:
    ///
    /// Parameter	Type	Description
    /// hash	string	The hash of the torrent you want to get the generic properties of
    /// Returns:
    ///
    /// HTTP Status Code	Scenario
    /// 404	Torrent hash was not found
    /// 200	All other scenarios- see JSON below
    /// The response is:
    ///
    /// empty, if the torrent hash is invalid
    /// otherwise, TorrentProperties
    ///
    pub async fn get_torrent_properties(
        &mut self,
        hash: String,
    ) -> Result<Option<TorrentProperties>, Error> {
        let arguments = Arguments::Form(format!("hash={}", hash));
        let request = ApiRequest {
            method: Method::Properties,
            arguments: Some(arguments),
        };
        let response = self.send_request(&request).await?;
        match response.status_code().as_u16() {
            200 => Ok(serde_json::from_reader(response.body().as_ref())?),
            404 => Err(Error::NoTorrentHash),
            _ => Err(Error::WrongStatusCode),
        }
    }

    /// Get torrent trackers
    /// Requires knowing the torrent hash. You can get it from torrent list.
    ///
    /// Name: trackers
    ///
    /// Parameters:
    ///
    /// Parameter	Type	Description
    /// hash	string	The hash of the torrent you want to get the trackers of
    /// Returns:
    ///
    /// HTTP Status Code	Scenario
    /// 404	Torrent hash was not found
    /// 200	All other scenarios- see JSON below
    ///
    pub async fn get_torrent_trackers(&mut self, hash: &str) -> Result<Vec<Tracker>, Error> {
        let arguments = Arguments::Form(format!("hash={}", hash));
        let request = ApiRequest {
            method: Method::Trackers,
            arguments: Some(arguments),
        };
        let response = self.send_request(&request).await?;
        match response.status_code().as_u16() {
            200 => Ok(serde_json::from_reader(response.body().as_ref())?),
            404 => Err(Error::NoTorrentHash),
            _ => Err(Error::WrongStatusCode),
        }
    }

    /// Get torrent web seeds
    /// Requires knowing the torrent hash. You can get it from torrent list.
    ///
    /// Name: webseeds
    ///
    /// Parameters:
    ///
    /// Parameter	Type	Description
    /// hash	string	The hash of the torrent you want to get the webseeds of
    /// Returns:
    ///
    /// HTTP Status Code	Scenario
    /// 404	Torrent hash was not found
    /// 200	All other scenarios- see JSON below
    ///
    /// Webseed
    pub async fn get_torrent_seeds(&mut self, hash: &str) -> Result<Vec<Webseed>, Error> {
        let arguments = Arguments::Form(format!("hash={}", hash));
        let request = ApiRequest {
            method: Method::Webseeds,
            arguments: Some(arguments),
        };
        let response = self.send_request(&request).await?;
        match response.status_code().as_u16() {
            200 => Ok(serde_json::from_reader(response.body().as_ref())?),
            404 => Err(Error::NoTorrentHash),
            _ => Err(Error::WrongStatusCode),
        }
    }

    /// Get torrent contents
    /// Requires knowing the torrent hash. You can get it from torrent list.
    ///
    /// Name: files
    ///
    /// Parameters:
    ///
    /// Parameter	Type	Description
    /// hash	string	The hash of the torrent you want to get the contents of
    /// indexes optional since 2.8.2	string	The indexes of the files you want to retrieve. indexes can contain multiple values separated by |.
    /// Returns:
    ///
    /// HTTP Status Code	Scenario
    /// 404	Torrent hash was not found
    /// 200	All other scenarios- see JSON below
    /// The response is:
    ///
    /// empty, if the torrent hash is invalid
    /// otherwise, Vec<File>
    ///
    pub async fn get_torrent_contents(
        &mut self,
        hash: &str,
        indexes: &str,
    ) -> Result<Vec<File>, Error> {
        let arguments = Arguments::Form(format!("hash={}&indexes={}", hash, indexes));
        let request = ApiRequest {
            method: Method::Files,
            arguments: Some(arguments),
        };
        let response = self.send_request(&request).await?;
        match dbg!(response.status_code().as_u16()) {
            200 => Ok(serde_json::from_reader(response.body().as_ref())?),
            404 => Err(Error::NoTorrentHash),
            _ => Err(Error::WrongStatusCode),
        }
    }

    /// Get torrent pieces' states
    /// Requires knowing the torrent hash. You can get it from torrent list.
    ///
    /// Name: pieceStates
    ///
    /// Parameters:
    ///
    /// Parameter	Type	Description
    /// hash string The hash of the torrent you want to get the pieces' states of
    ///
    /// Returns:
    ///
    /// HTTP Status Code	Scenario
    /// 404	Torrent hash was not found
    /// 200	All other scenarios- see JSON below
    /// The response is:
    ///
    /// empty, if the torrent hash is invalid
    /// otherwise, Vec<PieceState>
    ///
    pub async fn get_torrent_states(&mut self, hash: &str) -> Result<Vec<PieceState>, Error> {
        let request = ApiRequest {
            method: Method::PieceStates,
            arguments: Some(Arguments::Form(format!("hash={}", hash))),
        };
        let response = self.send_request(&request).await?;
        match dbg!(response.status_code().as_u16()) {
            200 => Ok(serde_json::from_reader(response.body().as_ref())?),
            404 => Err(Error::NoTorrentHash),
            _ => Err(Error::WrongStatusCode),
        }
    }

    /// Get torrent pieces' hashes
    /// Requires knowing the torrent hash. You can get it from torrent list.
    ///
    /// Name: pieceHashes
    ///
    /// Parameters:
    ///
    /// Parameter	Type	Description
    /// hash string The hash of the torrent you want to get the pieces' hashes of
    ///
    /// Returns:
    ///
    /// HTTP Status Code	Scenario
    /// 404	Torrent hash was not found
    /// 200	All other scenarios- see JSON below
    /// The response is:
    ///
    /// empty, if the torrent hash is invalid
    /// otherwise, Vec<String>.
    ///
    pub async fn get_torrent_hashes(&mut self, hash: &str) -> Result<Vec<String>, Error> {
        let request = ApiRequest {
            method: Method::PieceHashes,
            arguments: Some(Arguments::Form(format!("hash={}", hash))),
        };
        let response = self.send_request(&request).await?;
        check_default_status(
            &response,
            serde_json::from_reader(response.body().as_ref())?,
        )
    }

    /// Pause torrents
    /// Requires knowing the torrent hashes. You can get it from torrent list.
    /// 
    /// Name: pause
    /// 
    /// Parameters:
    /// 
    /// Parameter	Type	Description
    /// hashes string The hashes of the torrents you want to pause. hashes can contain multiple hashes separated by |, to pause multiple torrents, or set to all, to pause all torrents.
    /// Example:
    /// 
    /// /api/v2/torrents/pause?hashes=8c212779b4abde7c6bc608063a0d008b7e40ce32|54eddd830a5b58480a6143d616a97e3a6c23c439
    /// Returns:
    /// 
    /// HTTP Status Code	Scenario
    /// 200	All scenarios


    /// Resume torrents
    /// Requires knowing the torrent hashes. You can get it from torrent list.

    // Name: resume

    // Parameters:

    // Parameter	Type	Description
    //     /// The hashes of the torrents you want to resume. hashes can contain multiple hashes separated by |, to resume multiple torrents, or set to all, to resume all torrents.
    //    pub hashes: String,
    // Example:

    // /api/v2/torrents/resume?hashes=8c212779b4abde7c6bc608063a0d008b7e40ce32|54eddd830a5b58480a6143d616a97e3a6c23c439
    // Returns:

    // HTTP Status Code	Scenario
    // 200	All scenarios
    // Delete torrents
    // Requires knowing the torrent hashes. You can get it from torrent list.

    // Name: delete

    // Parameters:

    // Parameter	Type	Description
    //     /// The hashes of the torrents you want to delete. hashes can contain multiple hashes separated by |, to delete multiple torrents, or set to all, to delete all torrents.
    //    pub hashes: String,
    // deleteFiles	If set to true, the downloaded data will also be deleted, otherwise has no effect.
    // Example:

    // /api/v2/torrents/delete?hashes=8c212779b4abde7c6bc608063a0d008b7e40ce32&deleteFiles=false
    // Returns:

    // HTTP Status Code	Scenario
    // 200	All scenarios
    // Recheck torrents
    // Requires knowing the torrent hashes. You can get it from torrent list.

    // Name: recheck

    // Parameters:

    // Parameter	Type	Description
    //     /// The hashes of the torrents you want to recheck. hashes can contain multiple hashes separated by |, to recheck multiple torrents, or set to all, to recheck all torrents.
    //    pub hashes: String,
    // Example:

    // /api/v2/torrents/recheck?hashes=8c212779b4abde7c6bc608063a0d008b7e40ce32|54eddd830a5b58480a6143d616a97e3a6c23c439
    // Returns:

    // HTTP Status Code	Scenario
    // 200	All scenarios
    // Reannounce torrents
    // Requires knowing the torrent hashes. You can get it from torrent list.

    // Name: reannounce

    // Parameters:

    // Parameter	Type	Description
    //     /// The hashes of the torrents you want to reannounce. hashes can contain multiple hashes separated by |, to reannounce multiple torrents, or set to all, to reannounce all torrents.
    //    pub hashes: String,
    // Example:

    // /api/v2/torrents/reannounce?hashes=8c212779b4abde7c6bc608063a0d008b7e40ce32|54eddd830a5b58480a6143d616a97e3a6c23c439
    // Returns:

    // HTTP Status Code	Scenario
    // 200	All scenarios
    // Add new torrent
    // This method can add torrents from server local file or from URLs. http://, https://, magnet: and bc://bt/ links are supported.

    // Add torrent from URLs example:

    // POST /api/v2/torrents/add HTTP/1.1
    // User-Agent: Fiddler
    // Host: 127.0.0.1
    // Cookie: SID=your_sid
    // Content-Type: multipart/form-data; boundary=---------------------------6688794727912
    // Content-Length: length

    // -----------------------------6688794727912
    // Content-Disposition: form-data; name="urls"

    // https://torcache.net/torrent/3B1A1469C180F447B77021074DBBCCAEF62611E7.torrent
    // https://torcache.net/torrent/3B1A1469C180F447B77021074DBBCCAEF62611E8.torrent
    // -----------------------------6688794727912
    // Content-Disposition: form-data; name="savepath"

    // C:/Users/qBit/Downloads
    // -----------------------------6688794727912
    // Content-Disposition: form-data; name="cookie"

    // ui=28979218048197
    // -----------------------------6688794727912
    // Content-Disposition: form-data; name="category"

    // movies
    // -----------------------------6688794727912
    // Content-Disposition: form-data; name="skip_checking"

    // true
    // -----------------------------6688794727912
    // Content-Disposition: form-data; name="paused"

    // true
    // -----------------------------6688794727912
    // Content-Disposition: form-data; name="root_folder"

    // true
    // -----------------------------6688794727912--
    // Add torrents from files example:

    // POST /api/v2/torrents/add HTTP/1.1
    // Content-Type: multipart/form-data; boundary=-------------------------acebdf13572468
    // User-Agent: Fiddler
    // Host: 127.0.0.1
    // Cookie: SID=your_sid
    // Content-Length: length

    // ---------------------------acebdf13572468
    // Content-Disposition: form-data; name="torrents"; filename="8f18036b7a205c9347cb84a253975e12f7adddf2.torrent"
    // Content-Type: application/x-bittorrent

    // file_binary_data_goes_here
    // ---------------------------acebdf13572468
    // Content-Disposition: form-data; name="torrents"; filename="UFS.torrent"
    // Content-Type: application/x-bittorrent

    // file_binary_data_goes_here
    // ---------------------------acebdf13572468--
    // The above example will add two torrent files. file_binary_data_goes_here represents raw data of torrent file (basically a byte array).

    // Property	Type	Description
    //     /// URLs separated with newlines
    //    pub urls: String,
    // torrents	raw	Raw data of torrent file. torrents can be presented multiple times.
    //     /// Download folder
    //    pub savepath : Option<String>,
    //     /// Cookie sent to download the .torrent file
    //    pub cookie : Option<String>,
    //     /// Category for the torrent
    //    pub category : Option<String>,
    //     /// Tags for the torrent, split by ','
    //    pub tags : Option<String>,
    //     /// Skip hash checking. Possible values are true, false (default)
    //    pub skip_checking : Option<String>,
    //     /// Add torrents in the paused state. Possible values are true, false (default)
    //    pub paused : Option<String>,
    //     /// Create the root folder. Possible values are true, false, unset (default)
    //    pub root_folder : Option<String>,
    //     /// Rename torrent
    //    pub rename : Option<String>,
    //     /// Set torrent upload speed limit. Unit in bytes/second
    //    pub upLimit : Option<i64>,
    //     /// Set torrent download speed limit. Unit in bytes/second
    //    pub dlLimit : Option<i64>,
    //     /// Set torrent share ratio limit
    //    pub ratioLimit : Option<f64>,
    //     /// Set torrent seeding time limit. Unit in seconds
    //    pub seedingTimeLimit optional since 2.8.1: i64,
    //     /// Whether Automatic Torrent Management should be used
    //    pub autoTMM : Option<bool>,
    //     /// Enable sequential download. Possible values are true, false (default)
    //    pub sequentialDownload : Option<String>,
    //     /// Prioritize download first last piece. Possible values are true, false (default)
    //    pub firstLastPiecePrio : Option<String>,
    // Returns:

    // HTTP Status Code	Scenario
    // 415	Torrent file is not valid
    // 200	All other scenarios
    // Add trackers to torrent
    // Requires knowing the torrent hash. You can get it from torrent list.

    // POST /api/v2/torrents/addTrackers HTTP/1.1
    // User-Agent: Fiddler
    // Host: 127.0.0.1
    // Cookie: SID=your_sid
    // Content-Type: application/x-www-form-urlencoded
    // Content-Length: length

    // hash=8c212779b4abde7c6bc608063a0d008b7e40ce32&urls=http://192.168.0.1/announce%0Audp://192.168.0.1:3333/dummyAnnounce
    // This adds two trackers to torrent with hash 8c212779b4abde7c6bc608063a0d008b7e40ce32. Note %0A (aka LF newline) between trackers. Ampersand in tracker urls MUST be escaped.

    // Returns:

    // HTTP Status Code	Scenario
    // 404	Torrent hash was not found
    // 200	All other scenarios
    // Edit trackers
    // Name: editTracker

    // Parameters:

    // Parameter	Type	Description
    //     /// The hash of the torrent
    //    pub hash: String,
    //     /// The tracker URL you want to edit
    //    pub origUrl: String,
    //     /// The new URL to replace the origUrl
    //    pub newUrl: String,
    // Returns:

    // HTTP Status Code	Scenario
    // 400	newUrl is not a valid URL
    // 404	Torrent hash was not found
    // 409	newUrl already exists for the torrent
    // 409	origUrl was not found
    // 200	All other scenarios
    // Remove trackers
    // Name: removeTrackers

    // Parameters:

    // Parameter	Type	Description
    //     /// The hash of the torrent
    //    pub hash: String,
    //     /// URLs to remove, separated by |
    //    pub urls: String,
    // Returns:

    // HTTP Status Code	Scenario
    // 404	Torrent hash was not found
    // 409	All urls were not found
    // 200	All other scenarios
    // Add peers
    // Name: addPeers

    // Parameters:

    // Parameter	Type	Description
    //     /// The hash of the torrent, or multiple hashes separated by a pipe |
    //    pub hashes: String,
    //     /// The peer to add, or multiple peers separated by a pipe |. Each peer is a colon-separated host:port
    //    pub peers: String,
    // Returns:

    // HTTP Status Code	Scenario
    // 400	None of the supplied peers are valid
    // 200	All other scenarios
    // Increase torrent priority
    // Requires knowing the torrent hash. You can get it from torrent list.

    // Name: increasePrio

    // Parameters:

    // Parameter	Type	Description
    //     /// The hashes of the torrents you want to increase the priority of. hashes can contain multiple hashes separated by |, to increase the priority of multiple torrents, or set to all, to increase the priority of all torrents.
    //    pub hashes: String,
    // Example:

    // /api/v2/torrents/increasePrio?hashes=8c212779b4abde7c6bc608063a0d008b7e40ce32|54eddd830a5b58480a6143d616a97e3a6c23c439
    // Returns:

    // HTTP Status Code	Scenario
    // 409	Torrent queueing is not enabled
    // 200	All other scenarios
    // Decrease torrent priority
    // Requires knowing the torrent hash. You can get it from torrent list.

    // Name: decreasePrio

    // Parameters:

    // Parameter	Type	Description
    //     /// The hashes of the torrents you want to decrease the priority of. hashes can contain multiple hashes separated by |, to decrease the priority of multiple torrents, or set to all, to decrease the priority of all torrents.
    //    pub hashes: String,
    // Example:

    // /api/v2/torrents/decreasePrio?hashes=8c212779b4abde7c6bc608063a0d008b7e40ce32|54eddd830a5b58480a6143d616a97e3a6c23c439
    // Returns:

    // HTTP Status Code	Scenario
    // 409	Torrent queueing is not enabled
    // 200	All other scenarios
    // Maximal torrent priority
    // Requires knowing the torrent hash. You can get it from torrent list.

    // Name: topPrio

    // Parameters:

    // Parameter	Type	Description
    //     /// The hashes of the torrents you want to set to the maximum priority. hashes can contain multiple hashes separated by |, to set multiple torrents to the maximum priority, or set to all, to set all torrents to the maximum priority.
    //    pub hashes: String,
    // Example:

    // /api/v2/torrents/topPrio?hashes=8c212779b4abde7c6bc608063a0d008b7e40ce32|54eddd830a5b58480a6143d616a97e3a6c23c439
    // Returns:

    // HTTP Status Code	Scenario
    // 409	Torrent queueing is not enabled
    // 200	All other scenarios
    // Minimal torrent priority
    // Requires knowing the torrent hash. You can get it from torrent list.

    // Name: bottomPrio

    // Parameters:

    // Parameter	Type	Description
    //     /// The hashes of the torrents you want to set to the minimum priority. hashes can contain multiple hashes separated by |, to set multiple torrents to the minimum priority, or set to all, to set all torrents to the minimum priority.
    //    pub hashes: String,
    // Example:

    // /api/v2/torrents/bottomPrio?hashes=8c212779b4abde7c6bc608063a0d008b7e40ce32|54eddd830a5b58480a6143d616a97e3a6c23c439
    // Returns:

    // HTTP Status Code	Scenario
    // 409	Torrent queueing is not enabled
    // 200	All other scenarios
    // Set file priority
    // Name: filePrio

    // Parameters:

    // Parameter	Type	Description
    //     /// The hash of the torrent
    //    pub hash: String,
    //     /// File ids, separated by |
    //    pub id: String,
    // priority	number	File priority to set (consult torrent contents API for possible values)
    // id values correspond to file position inside the array returned by torrent contents API, e.g. id=0 for first file, id=1 for second file, etc.

    // Since 2.8.2 it is reccomended to use index field returned by torrent contents API (since the files can be filtered and the index value may differ from the position inside the response array).

    // Returns:

    // HTTP Status Code	Scenario
    // 400	Priority is invalid
    // 400	At least one file id is not a valid integer
    // 404	Torrent hash was not found
    // 409	Torrent metadata hasn't downloaded yet
    // 409	At least one file id was not found
    // 200	All other scenarios
    // Get torrent download limit
    // Requires knowing the torrent hash. You can get it from torrent list.

    // POST /api/v2/torrents/downloadLimit HTTP/1.1
    // User-Agent: Fiddler
    // Host: 127.0.0.1
    // Cookie: SID=your_sid
    // Content-Type: application/x-www-form-urlencoded
    // Content-Length: length

    // hashes=8c212779b4abde7c6bc608063a0d008b7e40ce32|284b83c9c7935002391129fd97f43db5d7cc2ba0
    // hashes can contain multiple hashes separated by | or set to all

    // Server reply (example):

    // HTTP/1.1 200 OK
    // content-type: application/json
    // content-length: length

    // {"8c212779b4abde7c6bc608063a0d008b7e40ce32":338944,"284b83c9c7935002391129fd97f43db5d7cc2ba0":123}
    // 8c212779b4abde7c6bc608063a0d008b7e40ce32 is the hash of the torrent and 338944 its download speed limit in bytes per second; this value will be zero if no limit is applied.

    // Set torrent download limit
    // Requires knowing the torrent hash. You can get it from torrent list.

    // POST /api/v2/torrents/setDownloadLimit HTTP/1.1
    // User-Agent: Fiddler
    // Host: 127.0.0.1
    // Cookie: SID=your_sid
    // Content-Type: application/x-www-form-urlencoded
    // Content-Length: length

    // hashes=8c212779b4abde7c6bc608063a0d008b7e40ce32|284b83c9c7935002391129fd97f43db5d7cc2ba0&limit=131072
    // hashes can contain multiple hashes separated by | or set to all limit is the download speed limit in bytes per second you want to set.

    // Returns:

    // HTTP Status Code	Scenario
    // 200	All scenarios
    // Set torrent share limit
    // Requires knowing the torrent hash. You can get it from torrent list.

    // POST /api/v2/torrents/setShareLimits HTTP/1.1
    // User-Agent: Fiddler
    // Host: 127.0.0.1
    // Cookie: SID=your_sid
    // Content-Type: application/x-www-form-urlencoded
    // Content-Length: length

    // hashes=8c212779b4abde7c6bc608063a0d008b7e40ce32|284b83c9c7935002391129fd97f43db5d7cc2ba0&ratioLimit=1.0&seedingTimeLimit=60
    // hashes can contain multiple hashes separated by | or set to all ratioLimit is the max ratio the torrent should be seeded until. -2 means the global limit should be used, -1 means no limit. seedingTimeLimit is the max amount of time the torrent should be seeded. -2 means the global limit should be used, -1 means no limit.

    // Returns:

    // HTTP Status Code	Scenario
    // 200	All scenarios
    // Get torrent upload limit
    // Requires knowing the torrent hash. You can get it from torrent list.

    // POST /api/v2/torrents/uploadLimit HTTP/1.1
    // User-Agent: Fiddler
    // Host: 127.0.0.1
    // Cookie: SID=your_sid
    // Content-Type: application/x-www-form-urlencoded
    // Content-Length: length

    // hashes=8c212779b4abde7c6bc608063a0d008b7e40ce32|284b83c9c7935002391129fd97f43db5d7cc2ba0
    // hashes can contain multiple hashes separated by | or set to all

    // Server reply (example):

    // HTTP/1.1 200 OK
    // content-type: application/json
    // content-length: length

    // {"8c212779b4abde7c6bc608063a0d008b7e40ce32":338944,"284b83c9c7935002391129fd97f43db5d7cc2ba0":123}
    // 8c212779b4abde7c6bc608063a0d008b7e40ce32 is the hash of the torrent in the request and 338944 its upload speed limit in bytes per second; this value will be zero if no limit is applied.

    // Set torrent upload limit
    // Requires knowing the torrent hash. You can get it from torrent list.

    // POST /api/v2/torrents/setUploadLimit HTTP/1.1
    // User-Agent: Fiddler
    // Host: 127.0.0.1
    // Cookie: SID=your_sid
    // Content-Type: application/x-www-form-urlencoded
    // Content-Length: length

    // hashes=8c212779b4abde7c6bc608063a0d008b7e40ce32|284b83c9c7935002391129fd97f43db5d7cc2ba0&limit=131072
    // hashes can contain multiple hashes separated by | or set to all limit is the upload speed limit in bytes per second you want to set.

    // Returns:

    // HTTP Status Code	Scenario
    // 200	All scenarios
    // Set torrent location
    // Requires knowing the torrent hash. You can get it from torrent list.

    // POST /api/v2/torrents/setLocation HTTP/1.1
    // User-Agent: Fiddler
    // Host: 127.0.0.1
    // Cookie: SID=your_sid
    // Content-Type: application/x-www-form-urlencoded
    // Content-Length: length

    // hashes=8c212779b4abde7c6bc608063a0d008b7e40ce32|284b83c9c7935002391129fd97f43db5d7cc2ba0&location=/mnt/nfs/media
    // hashes can contain multiple hashes separated by | or set to all location is the location to download the torrent to. If the location doesn't exist, the torrent's location is unchanged.

    // Returns:

    // HTTP Status Code	Scenario
    // 400	Save path is empty
    // 403	User does not have write access to directory
    // 409	Unable to create save path directory
    // 200	All other scenarios
    // Set torrent name
    // Requires knowing the torrent hash. You can get it from torrent list.

    // POST /api/v2/torrents/rename HTTP/1.1
    // User-Agent: Fiddler
    // Host: 127.0.0.1
    // Cookie: SID=your_sid
    // Content-Type: application/x-www-form-urlencoded
    // Content-Length: length

    // hash=8c212779b4abde7c6bc608063a0d008b7e40ce32&name=This%20is%20a%20test
    // Returns:

    // HTTP Status Code	Scenario
    // 404	Torrent hash is invalid
    // 409	Torrent name is empty
    // 200	All other scenarios
    // Set torrent category
    // Requires knowing the torrent hash. You can get it from torrent list.

    // POST /api/v2/torrents/setCategory HTTP/1.1
    // User-Agent: Fiddler
    // Host: 127.0.0.1
    // Cookie: SID=your_sid
    // Content-Type: application/x-www-form-urlencoded
    // Content-Length: length

    // hashes=8c212779b4abde7c6bc608063a0d008b7e40ce32|284b83c9c7935002391129fd97f43db5d7cc2ba0&category=CategoryName
    // hashes can contain multiple hashes separated by | or set to all

    // category is the torrent category you want to set.

    // Returns:

    // HTTP Status Code	Scenario
    // 409	Category name does not exist
    // 200	All other scenarios
    // Get all categories
    // Name: categories

    // Parameters:

    // None

    // Returns all categories in JSON format, e.g.:

    // {
    //     "Video": {
    //         "name": "Video",
    //         "savePath": "/home/user/torrents/video/"
    //     },
    //     "eBooks": {
    //         "name": "eBooks",
    //         "savePath": "/home/user/torrents/eBooks/"
    //     }
    // }
    // Returns:

    // HTTP Status Code	Scenario
    // 200	All scenarios
    // Add new category
    // POST /api/v2/torrents/createCategory HTTP/1.1
    // User-Agent: Fiddler
    // Host: 127.0.0.1
    // Cookie: SID=your_sid
    // Content-Type: application/x-www-form-urlencoded
    // Content-Length: length

    // category=CategoryName&savePath=/path/to/dir
    // category is the category you want to create.

    // Returns:

    // HTTP Status Code	Scenario
    // 400	Category name is empty
    // 409	Category name is invalid
    // 200	All other scenarios
    // Edit category
    // POST /api/v2/torrents/editCategory HTTP/1.1
    // User-Agent: Fiddler
    // Host: 127.0.0.1
    // Cookie: SID=your_sid
    // Content-Type: application/x-www-form-urlencoded
    // Content-Length: length

    // category=CategoryName&savePath=/path/to/save/torrents/to
    // Returns:

    // HTTP Status Code	Scenario
    // 400	Category name is empty
    // 409	Category editing failed
    // 200	All other scenarios
    // Remove categories
    // POST /api/v2/torrents/removeCategories HTTP/1.1
    // User-Agent: Fiddler
    // Host: 127.0.0.1
    // Cookie: SID=your_sid
    // Content-Type: application/x-www-form-urlencoded
    // Content-Length: length

    // categories=Category1%0ACategory2
    // categories can contain multiple cateogies separated by \n (%0A urlencoded)

    // Returns:

    // HTTP Status Code	Scenario
    // 200	All scenarios
    // Add torrent tags
    // Requires knowing the torrent hash. You can get it from torrent list.

    // POST /api/v2/torrents/addTags HTTP/1.1
    // User-Agent: Fiddler
    // Host: 127.0.0.1
    // Cookie: SID=your_sid
    // Content-Type: application/x-www-form-urlencoded
    // Content-Length: length

    // hashes=8c212779b4abde7c6bc608063a0d008b7e40ce32|284b83c9c7935002391129fd97f43db5d7cc2ba0&tags=TagName1,TagName2
    // hashes can contain multiple hashes separated by | or set to all

    // tags is the list of tags you want to add to passed torrents.

    // Returns:

    // HTTP Status Code	Scenario
    // 200	All scenarios
    // Remove torrent tags
    // Requires knowing the torrent hash. You can get it from torrent list.

    // POST /api/v2/torrents/removeTags HTTP/1.1
    // User-Agent: Fiddler
    // Host: 127.0.0.1
    // Cookie: SID=your_sid
    // Content-Type: application/x-www-form-urlencoded
    // Content-Length: length

    // hashes=8c212779b4abde7c6bc608063a0d008b7e40ce32|284b83c9c7935002391129fd97f43db5d7cc2ba0&tags=TagName1,TagName2
    // hashes can contain multiple hashes separated by | or set to all

    // tags is the list of tags you want to remove from passed torrents. Empty list removes all tags from relevant torrents.

    // Returns:

    // HTTP Status Code	Scenario
    // 200	All scenarios
    // Get all tags
    // Name: tags

    // Parameters:

    // None

    // Returns all tags in JSON format, e.g.:

    // [
    //     "Tag 1",
    //     "Tag 2"
    // ]
    // Returns:

    // HTTP Status Code	Scenario
    // 200	All scenarios
    // Create tags
    // POST /api/v2/torrents/createTags HTTP/1.1
    // User-Agent: Fiddler
    // Host: 127.0.0.1
    // Cookie: SID=your_sid
    // Content-Type: application/x-www-form-urlencoded
    // Content-Length: length

    // tags=TagName1,TagName2
    // tags is a list of tags you want to create. Can contain multiple tags separated by ,.

    // Returns:

    // HTTP Status Code	Scenario
    // 200	All scenarios
    // Delete tags
    // POST /api/v2/torrents/deleteTags HTTP/1.1
    // User-Agent: Fiddler
    // Host: 127.0.0.1
    // Cookie: SID=your_sid
    // Content-Type: application/x-www-form-urlencoded
    // Content-Length: length

    // tags=TagName1,TagName2
    // tags is a list of tags you want to delete. Can contain multiple tags separated by ,.

    // Returns:

    // HTTP Status Code	Scenario
    // 200	All scenarios
    // Set automatic torrent management
    // Requires knowing the torrent hash. You can get it from torrent list.

    // POST /api/v2/torrents/setAutoManagement HTTP/1.1
    // User-Agent: Fiddler
    // Host: 127.0.0.1
    // Cookie: SID=your_sid
    // Content-Type: application/x-www-form-urlencoded
    // Content-Length: length

    // hashes=8c212779b4abde7c6bc608063a0d008b7e40ce32|284b83c9c7935002391129fd97f43db5d7cc2ba0&enable=true
    // hashes can contain multiple hashes separated by | or set to all enable is a boolean, affects the torrents listed in hashes, default is false

    // Returns:

    // HTTP Status Code	Scenario
    // 200	All scenarios
    // Toggle sequential download
    // Requires knowing the torrent hash. You can get it from torrent list.

    // Name: toggleSequentialDownload

    // Parameters:

    // Parameter	Type	Description
    //     /// The hashes of the torrents you want to toggle sequential download for. hashes can contain multiple hashes separated by |, to toggle sequential download for multiple torrents, or set to all, to toggle sequential download for all torrents.
    //    pub hashes: String,
    // Example:

    // /api/v2/torrents/toggleSequentialDownload?hashes=8c212779b4abde7c6bc608063a0d008b7e40ce32|54eddd830a5b58480a6143d616a97e3a6c23c439
    // Returns:

    // HTTP Status Code	Scenario
    // 200	All scenarios
    // Set first/last piece priority
    // Requires knowing the torrent hash. You can get it from torrent list.

    // Name: toggleFirstLastPiecePrio

    // Parameters:

    // Parameter	Type	Description
    //     /// The hashes of the torrents you want to toggle the first/last piece priority for. hashes can contain multiple hashes separated by |, to toggle the first/last piece priority for multiple torrents, or set to all, to toggle the first/last piece priority for all torrents.
    //    pub hashes: String,
    // Example:

    // /api/v2/torrents/toggleFirstLastPiecePrio?hashes=8c212779b4abde7c6bc608063a0d008b7e40ce32|54eddd830a5b58480a6143d616a97e3a6c23c439
    // Returns:

    // HTTP Status Code	Scenario
    // 200	All scenarios
    // Set force start
    // Requires knowing the torrent hash. You can get it from torrent list.

    // POST /api/v2/torrents/setForceStart HTTP/1.1
    // User-Agent: Fiddler
    // Host: 127.0.0.1
    // Cookie: SID=your_sid
    // Content-Type: application/x-www-form-urlencoded
    // Content-Length: length

    // hashes=8c212779b4abde7c6bc608063a0d008b7e40ce32?value=true
    // hashes can contain multiple hashes separated by | or set to all value is a boolean, affects the torrents listed in hashes, default is false

    // Returns:

    // HTTP Status Code	Scenario
    // 200	All scenarios
    // Set super seeding
    // Requires knowing the torrent hash. You can get it from torrent list.

    // POST /api/v2/torrents/setSuperSeeding HTTP/1.1
    // User-Agent: Fiddler
    // Host: 127.0.0.1
    // Cookie: SID=your_sid
    // Content-Type: application/x-www-form-urlencoded
    // Content-Length: length

    // hashes=8c212779b4abde7c6bc608063a0d008b7e40ce32?value=true
    // hashes can contain multiple hashes separated by | or set to all value is a boolean, affects the torrents listed in hashes, default is false

    // Returns:

    // HTTP Status Code	Scenario
    // 200	All scenarios
    // Rename file
    // Name: renameFile

    // Parameters:

    // Parameter	Type	Description
    //     /// The hash of the torrent
    //    pub hash: String,
    //     /// The old path of the torrent
    //    pub oldPath: String,
    //     /// The new path to use for the file
    //    pub newPath: String,
    // Returns:

    // HTTP Status Code	Scenario
    // 400	Missing newPath parameter
    // 409	Invalid newPath or oldPath, or newPath already in use
    // 200	All other scenarios
    // Rename folder
    // Name: renameFolder

    // Parameters:

    // Parameter	Type	Description
    //     /// The hash of the torrent
    //    pub hash: String,
    //     /// The old path of the torrent
    //    pub oldPath: String,
    //     /// The new path to use for the file
    //    pub newPath: String,
    // Returns:

    // HTTP Status Code	Scenario
    // 400	Missing newPath parameter
    // 409	Invalid newPath or oldPath, or newPath already in use
    // 200	All other scenarios
}
