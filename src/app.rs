use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::{
    request::{ApiRequest, Arguments, Method},
    response::check_default_status,
    Client, Error,
};

/// All Application API methods are under "app", e.g.: /api/v2/app/methodName

#[derive(Debug, Serialize, Deserialize)]
pub struct BuildInfo {
    /// QT version
    pub qt: String,
    /// libtorrent version
    pub libtorrent: String,
    /// Boost version
    pub boost: String,
    /// OpenSSL version
    pub openssl: String,
    /// Application bitness (e.g. 64-bit)
    pub bitness: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Preferences {
    /// Currently selected language (e.g. en_GB for English)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    /// True if a subfolder should be created when adding a torrent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_subfolder_enabled: Option<bool>,
    /// True if torrents should be added in a Paused state
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_paused_enabled: Option<bool>,
    /// TODO
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_delete_mode: Option<i64>,
    /// True if disk space should be pre-allocated for all files
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preallocate_all: Option<bool>,
    /// True if ".!qB" should be appended to incomplete files
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incomplete_files_ext: Option<bool>,
    /// True if Automatic Torrent Management is enabled by default
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_tmm_enabled: Option<bool>,
    /// True if torrent should be relocated when its Category changes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub torrent_changed_tmm_enabled: Option<bool>,
    /// True if torrent should be relocated when the default save path changes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub save_path_changed_tmm_enabled: Option<bool>,
    /// True if torrent should be relocated when its Category's save path changes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_changed_tmm_enabled: Option<bool>,
    /// Default save path for torrents, separated by slashes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub save_path: Option<String>,
    /// True if folder for incomplete torrents is enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temp_path_enabled: Option<bool>,
    /// Path for incomplete torrents, separated by slashes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temp_path: Option<String>,
    /// Property: directory to watch for torrent files, value: where torrents loaded from this directory should be downloaded to (see list of possible values below). Slashes are used as path separators; multiple key/value pairs can be specified
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_dirs: Option<HashMap<String, ScanDir>>,
    /// Path to directory to copy .torrent files to. Slashes are used as path separators
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_dir: Option<String>,
    /// Path to directory to copy .torrent files of completed downloads to. Slashes are used as path separators
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_dir_fin: Option<String>,
    /// True if e-mail notification should be enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mail_notification_enabled: Option<bool>,
    /// e-mail where notifications should originate from
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mail_notification_sender: Option<String>,
    /// e-mail to send notifications to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mail_notification_email: Option<String>,
    /// smtp server for e-mail notifications
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mail_notification_smtp: Option<String>,
    /// True if smtp server requires SSL connection
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mail_notification_ssl_enabled: Option<bool>,
    /// True if smtp server requires authentication
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mail_notification_auth_enabled: Option<bool>,
    /// Username for smtp authentication
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mail_notification_username: Option<String>,
    /// Password for smtp authentication
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mail_notification_password: Option<String>,
    /// True if external program should be run after torrent has finished downloading
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autorun_enabled: Option<bool>,
    /// Program path/name/arguments to run if autorun_enabled is enabled; path is separated by slashes; you can use %f and %n arguments, which will be expanded by qBittorent as path_to_torrent_file and torrent_name (from the GUI; not the .torrent file name) respectively
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autorun_program: Option<String>,
    /// True if torrent queuing is enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queueing_enabled: Option<bool>,
    /// Maximum number of active simultaneous downloads
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_active_downloads: Option<i64>,
    /// Maximum number of active simultaneous downloads and uploads
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_active_torrents: Option<i64>,
    /// Maximum number of active simultaneous uploads
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_active_uploads: Option<i64>,
    /// If true torrents w/o any activity (stalled ones) will not be counted towards max_active_* limits; see dont_count_slow_torrents for more information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dont_count_slow_torrents: Option<bool>,
    /// Download rate in KiB/s for a torrent to be considered "slow"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slow_torrent_dl_rate_threshold: Option<i64>,
    /// Upload rate in KiB/s for a torrent to be considered "slow"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slow_torrent_ul_rate_threshold: Option<i64>,
    /// Seconds a torrent should be inactive before considered "slow"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slow_torrent_inactive_timer: Option<i64>,
    /// True if share ratio limit is enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_ratio_enabled: Option<bool>,
    /// Get the global share ratio limit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_ratio: Option<f64>,
    /// Action performed when a torrent reaches the maximum share ratio. See list of possible values here below.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_ratio_act: Option<MaxRatioAct>,
    /// Port for incoming connections
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listen_port: Option<i64>,
    /// True if UPnP/NAT-PMP is enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upnp: Option<bool>,
    /// True if the port is randomly selected
    #[serde(skip_serializing_if = "Option::is_none")]
    pub random_port: Option<bool>,
    /// Global download speed limit in KiB/s; -1 means no limit is applied
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dl_limit: Option<i64>,
    /// Global upload speed limit in KiB/s; -1 means no limit is applied
    #[serde(skip_serializing_if = "Option::is_none")]
    pub up_limit: Option<i64>,
    /// Maximum global number of simultaneous connections
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_connec: Option<i64>,
    /// Maximum number of simultaneous connections per torrent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_connec_per_torrent: Option<i64>,
    /// Maximum number of upload slots
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_uploads: Option<i64>,
    /// Maximum number of upload slots per torrent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_uploads_per_torrent: Option<i64>,
    /// Timeout in seconds for a stopped announce request to trackers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_tracker_timeout: Option<i64>,
    /// True if the advanced libtorrent option piece_extent_affinity is enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_piece_extent_affinity: Option<bool>,
    /// Bittorrent Protocol to use (see list of possible values below)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bittorrent_protocol: Option<BittorrentProtocol>,
    /// True if [du]l_limit should be applied to uTP connections; this option is only available in qBittorent built against libtorrent version 0.16.X and higher
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_utp_rate: Option<bool>,
    /// True if [du]l_limit should be applied to estimated TCP overhead (service data: e.g. packet headers)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_tcp_overhead: Option<bool>,
    /// True if [du]l_limit should be applied to peers on the LAN
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_lan_peers: Option<bool>,
    /// Alternative global download speed limit in KiB/s
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt_dl_limit: Option<i64>,
    /// Alternative global upload speed limit in KiB/s
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt_up_limit: Option<i64>,
    /// True if alternative limits should be applied according to schedule
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduler_enabled: Option<bool>,
    /// Scheduler starting hour
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_from_hour: Option<i64>,
    /// Scheduler starting minute
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_from_min: Option<i64>,
    /// Scheduler ending hour
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_to_hour: Option<i64>,
    /// Scheduler ending minute
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_to_min: Option<i64>,
    /// Scheduler days. See possible values here below
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduler_days: Option<SchedulerDays>,
    /// True if DHT is enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dht: Option<bool>,
    /// True if PeX is enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pex: Option<bool>,
    /// True if LSD is enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lsd: Option<bool>,
    /// See list of possible values here below
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<Encryption>,
    /// If true anonymous mode will be enabled; read more here; this option is only available in qBittorent built against libtorrent version 0.16.X and higher
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anonymous_mode: Option<bool>,
    /// See list of possible values here below
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_type: Option<ProxyType>,
    /// Proxy IP address or domain name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_ip: Option<String>,
    /// Proxy port
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_port: Option<i64>,
    /// True if peer and web seed connections should be proxified; this option will have any effect only in qBittorent built against libtorrent version 0.16.X and higher
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_peer_connections: Option<bool>,
    /// True proxy requires authentication; doesn't apply to SOCKS4 proxies
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_auth_enabled: Option<bool>,
    /// Username for proxy authentication
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_username: Option<String>,
    /// Password for proxy authentication
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_password: Option<String>,
    /// True if proxy is only used for torrents
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_torrents_only: Option<bool>,
    /// True if external IP filter should be enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_filter_enabled: Option<bool>,
    /// Path to IP filter file (.dat, .p2p, .p2b files are supported); path is separated by slashes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_filter_path: Option<String>,
    /// True if IP filters are applied to trackers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_filter_trackers: Option<bool>,
    /// Comma-separated list of domains to accept when performing Host header validation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_ui_domain_list: Option<String>,
    /// IP address to use for the WebUI
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_ui_address: Option<String>,
    /// WebUI port
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_ui_port: Option<i64>,
    /// True if UPnP is used for the WebUI port
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_ui_upnp: Option<bool>,
    /// WebUI username
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_ui_username: Option<String>,
    /// For API ≥ v2.3.0: Plaintext WebUI password, not readable, write-only. For API < v2.3.0: MD5 hash of WebUI password, hash is generated from the following string: username:Web UI Access:plain_text_web_ui_password
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_ui_password: Option<String>,
    /// True if WebUI CSRF protection is enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_ui_csrf_protection_enabled: Option<bool>,
    /// True if WebUI clickjacking protection is enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_ui_clickjacking_protection_enabled: Option<bool>,
    /// True if WebUI cookie Secure flag is enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_ui_secure_cookie_enabled: Option<bool>,
    /// Maximum number of authentication failures before WebUI access ban
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_ui_max_auth_fail_count: Option<i64>,
    /// WebUI access ban duration in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_ui_ban_duration: Option<i64>,
    /// Seconds until WebUI is automatically signed off
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_ui_session_timeout: Option<i64>,
    /// True if WebUI host header validation is enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_ui_host_header_validation_enabled: Option<bool>,
    /// True if authentication challenge for loopback address (127.0.0.1) should be disabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bypass_local_auth: Option<bool>,
    /// True if webui authentication should be bypassed for clients whose ip resides within (at least) one of the subnets on the whitelist
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bypass_auth_subnet_whitelist_enabled: Option<bool>,
    /// (White)list of ipv4/ipv6 subnets for which webui authentication should be bypassed; list entries are separated by commas
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bypass_auth_subnet_whitelist: Option<String>,
    /// True if an alternative WebUI should be used
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alternative_webui_enabled: Option<bool>,
    /// File path to the alternative WebUI
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alternative_webui_path: Option<String>,
    /// True if WebUI HTTPS access is enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_https: Option<bool>,
    /// For API < v2.0.1: SSL keyfile contents (this is a not a path)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_key: Option<String>,
    /// For API < v2.0.1: SSL certificate contents (this is a not a path)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_cert: Option<String>,
    /// For API ≥ v2.0.1: Path to SSL keyfile
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_ui_https_key_path: Option<String>,
    /// For API ≥ v2.0.1: Path to SSL certificate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_ui_https_cert_path: Option<String>,
    /// True if server DNS should be updated dynamically
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dyndns_enabled: Option<bool>,
    /// See list of possible values here below
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dyndns_service: Option<DyndnsService>,
    /// Username for DDNS service
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dyndns_username: Option<String>,
    /// Password for DDNS service
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dyndns_password: Option<String>,
    /// Your DDNS domain name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dyndns_domain: Option<String>,
    /// RSS refresh interval
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rss_refresh_interval: Option<i64>,
    /// Max stored articles per RSS feed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rss_max_articles_per_feed: Option<i64>,
    /// Enable processing of RSS feeds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rss_processing_enabled: Option<bool>,
    /// Enable auto-downloading of torrents from the RSS feeds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rss_auto_downloading_enabled: Option<bool>,
    /// For API ≥ v2.5.1: Enable downloading of repack/proper Episodes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rss_download_repack_proper_episodes: Option<bool>,
    /// For API ≥ v2.5.1: List of RSS Smart Episode Filters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rss_smart_episode_filters: Option<String>,
    /// Enable automatic adding of trackers to new torrents
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_trackers_enabled: Option<bool>,
    /// List of trackers to add to new torrent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_trackers: Option<String>,
    /// For API ≥ v2.5.1: Enable custom http headers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_ui_use_custom_http_headers_enabled: Option<bool>,
    /// For API ≥ v2.5.1: List of custom http headers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_ui_custom_http_headers: Option<String>,
    /// True enables max seeding time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_seeding_time_enabled: Option<bool>,
    /// Number of minutes to seed a torrent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_seeding_time: Option<i64>,
    /// TODO
    #[serde(skip_serializing_if = "Option::is_none")]
    pub announce_ip: Option<String>,
    /// True always announce to all tiers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub announce_to_all_tiers: Option<bool>,
    /// True always announce to all trackers in a tier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub announce_to_all_trackers: Option<bool>,
    /// Number of asynchronous I/O threads
    #[serde(skip_serializing_if = "Option::is_none")]
    pub async_io_threads: Option<i64>,
    /// List of banned IPs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banned_ips: Option<String>,
    /// Outstanding memory when checking torrents in MiB
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checking_memory_use: Option<i64>,
    /// IP Address to bind to. Empty String means All addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_interface_address: Option<String>,
    /// Network Interface used
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_network_interface: Option<String>,
    /// Disk cache used in MiB
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_cache: Option<i64>,
    /// Disk cache expiry interval in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_cache_ttl: Option<i64>,
    /// Port used for embedded tracker
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedded_tracker_port: Option<i64>,
    /// True enables coalesce reads & writes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_coalesce_read_write: Option<bool>,
    /// True enables embedded tracker
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_embedded_tracker: Option<bool>,
    /// True allows multiple connections from the same IP address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_multi_connections_from_same_ip: Option<bool>,
    /// True enables os cache
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_os_cache: Option<bool>,
    /// True enables sending of upload piece suggestions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_upload_suggestions: Option<bool>,
    /// File pool size
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_pool_size: Option<i64>,
    /// Maximal outgoing port (0: Disabled)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outgoing_ports_max: Option<i64>,
    /// Minimal outgoing port (0: Disabled)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outgoing_ports_min: Option<i64>,
    /// True rechecks torrents on completion
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recheck_completed_torrents: Option<bool>,
    /// True resolves peer countries
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolve_peer_countries: Option<bool>,
    /// Save resume data interval in min
    #[serde(skip_serializing_if = "Option::is_none")]
    pub save_resume_data_interval: Option<i64>,
    /// Send buffer low watermark in KiB
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_buffer_low_watermark: Option<i64>,
    /// Send buffer watermark in KiB
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_buffer_watermark: Option<i64>,
    /// Send buffer watermark factor in percent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_buffer_watermark_factor: Option<i64>,
    /// Socket backlog size
    #[serde(skip_serializing_if = "Option::is_none")]
    pub socket_backlog_size: Option<i64>,
    /// Upload choking algorithm used (see list of possible values below)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_choking_algorithm: Option<UploadChokingAlgorithm>,
    /// Upload slots behavior used (see list of possible values below)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_slots_behavior: Option<UploadSlotsBehavior>,
    /// UPnP lease duration (0: Permanent lease)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upnp_lease_duration: Option<i64>,
    /// μTP-TCP mixed mode algorithm (see list of possible values below)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utp_tcp_mixed_mode: Option<UtpTcpMixedMode>,
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ScanDir {
    /// Download to the monitored folder
    Zero = 0,
    /// Download to the default save path
    One = 1,
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum SchedulerDays {
    EveryDay = 0,
    EveryWeekday = 1,
    EveryWeekend = 2,
    EveryMonday = 3,
    EveryTuesday = 4,
    EveryWednesday = 5,
    EveryThursday = 6,
    EveryFriday = 7,
    EverySaturday = 8,
    EverySunday = 9,
}

///     NB: the first options allows you to use both encrypted and unencrypted connections (this is the default); other options are mutually exclusive: e.g. by forcing encryption on you won't be able to use unencrypted connections and vice versa.
#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum Encryption {
    PreferEncryption = 0,
    ForceEncryptionOn = 1,
    ForceEncryptionOff = 2,
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(i8)]
pub enum ProxyType {
    /// Proxy is disabled
    Disabled = 0,
    /// HTTP proxy without authentication
    HttpNoAuth = 1,
    /// SOCKS5 proxy without authentication
    Socks5NoAuth = 2,
    /// HTTP proxy with authentication
    HttpAuth = 3,
    /// SOCKS5 proxy with authentication
    Socks5Auth = 4,
    /// SOCKS4 proxy without authentication
    Socks4NoAuth = 5,
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum DyndnsService {
    DyDNS = 0,
    NOIP = 1,
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum MaxRatioAct {
    Pause = 0,
    Remove = 1,
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum BittorrentProtocol {
    Both = 0,
    TCP = 1,
    UTP = 2,
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum UploadChokingAlgorithm {
    RoundRobin = 0,
    FastestUpload = 1,
    AntiLeech = 2,
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum UploadSlotsBehavior {
    FixedSlots = 0,
    UploadRateBased = 1,
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum UtpTcpMixedMode {
    PreferTCP = 0,
    PeerProportional = 1,
}

impl Client {
    /// Get application version
    ///
    /// Name: version
    ///
    /// Parameters:
    ///
    /// None
    ///
    /// Returns:
    /// HTTP Status Code	Scenario
    /// 200	All scenarios
    ///
    /// String
    ///
    /// The response is a string with the application version, e.g. v4.1.3
    ///
    pub async fn get_version(&mut self) -> Result<String, Error> {
        let request = ApiRequest {
            method: Method::Version,
            arguments: None,
        };
        let response = self.send_request(&request).await?;
        check_default_status(&response, String::from_utf8(response.body().to_vec())?)
    }

    /// Get API version
    ///
    /// Name: webapiVersion
    ///
    /// Parameters:
    ///
    /// None
    ///
    /// Returns:
    /// HTTP Status Code	Scenario
    /// 200	All scenarios
    ///
    /// String
    ///
    /// The response is a string with the WebAPI version, e.g. 2.0
    ///
    pub async fn get_api_version(&mut self) -> Result<String, Error> {
        let request = ApiRequest {
            method: Method::WebapiVersion,
            arguments: None,
        };
        let response = self.send_request(&request).await?;
        check_default_status(&response, String::from_utf8(response.body().to_vec())?)
    }

    /// Get build info
    ///
    /// Name: buildInfo
    ///
    /// Parameters:
    ///
    /// None
    ///
    /// Returns:
    /// HTTP Status Code	Scenario
    /// 200	All scenarios- see JSON below
    ///
    /// BuildInfo
    ///
    /// The response is a JSON object containing the following fields
    ///
    pub async fn get_build_info(&mut self) -> Result<BuildInfo, Error> {
        let request = ApiRequest {
            method: Method::BuildInfo,
            arguments: None,
        };
        let response = self.send_request(&request).await?;
        check_default_status(
            &response,
            serde_json::from_reader(response.body().as_ref())?,
        )
    }

    /// Shutdown application
    ///
    /// Name: shutdown
    ///
    /// Parameters:
    ///
    /// None
    ///
    /// Returns:
    /// HTTP Status Code	Scenario
    /// 200	All scenarios
    ///
    /// None
    ///
    pub async fn shutdown(&mut self) -> Result<(), Error> {
        let request = ApiRequest {
            method: Method::Shutdown,
            arguments: None,
        };
        let response = self.send_request(&request).await?;
        check_default_status(&response, ())
    }

    /// Get application preferences
    ///
    /// Name: preferences
    ///
    /// Parameters:
    ///
    /// None
    ///
    /// Returns:
    /// HTTP Status Code	Scenario
    /// 200	All scenarios- see JSON below
    ///
    /// Preferences
    ///
    /// The response is a JSON object with several fields (key-value) pairs representing the application's settings. The contents may vary depending on which settings are present in qBittorrent.ini.
    ///
    pub async fn get_preferences(&mut self) -> Result<Preferences, Error> {
        let request = ApiRequest {
            method: Method::Preferences,
            arguments: None,
        };
        let response = self.send_request(&request).await?;
        check_default_status(
            &response,
            serde_json::from_reader(response.body().as_ref())?,
        )
    }

    /// Set application preferences
    ///
    /// Name: setPreferences
    ///
    /// Parameters:
    ///
    /// A json object with key-value pairs of the settings you want to change and their new values.
    ///
    /// Example:
    ///
    /// json={"save_path":"C:/Users/Dayman/Downloads","queueing_enabled":false,"scan_dirs":{"C:/Games": 0,"D:/Downloads": 1}}
    ///
    /// Returns:
    /// HTTP Status Code	Scenario
    /// 200	All scenarios
    ///
    /// None
    ///
    /// Notes:
    ///
    ///     There is no need to pass all possible preferences' token:value pairs if you only want to change one option
    ///     Paths in scan_dirs must exist, otherwise this option will have no effect
    ///     String values must be quoted; integer and boolean values must never be quoted
    ///
    /// For a list of possible preference options see Get application preferences
    ///
    pub async fn set_preferences(&mut self, values: Preferences) -> Result<(), Error> {
        let arguments = Arguments::Json(json!(values));
        let request = ApiRequest {
            method: Method::SetPreferences,
            arguments: Some(arguments),
        };
        let response = self.send_request(&request).await?;
        check_default_status(&response, ())
    }

    /// Get default save path
    ///
    /// Name: defaultSavePath
    ///
    /// Parameters:
    ///
    /// None
    ///
    /// Returns:
    /// HTTP Status Code	Scenario
    /// 200	All scenarios
    ///
    /// String
    ///
    /// The response is a string with the default save path, e.g. C:/Users/Dayman/Downloads.
    ///
    pub async fn get_default_save_path(&mut self) -> Result<String, Error> {
        let request = ApiRequest {
            method: Method::DefaultSavePath,
            arguments: None,
        };
        let response = self.send_request(&request).await?;
        check_default_status(&response, String::from_utf8(response.body().to_vec())?)
    }
}
