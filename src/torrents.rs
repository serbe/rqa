use serde::{Deserialize, Serialize};

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
