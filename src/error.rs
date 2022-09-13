#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Error in parse URL")]
    UrlError(#[from] url::ParseError),
    #[error("Response no contains set-cookie header")]
    NoSetCookie,
    #[error("Header set-cookie no contains SID")]
    NoSID,
    #[error("User's IP is banned for too many failed login attempts")]
    Banned,
    #[error("Wrong response status code")]
    WrongStatusCode,
    #[error("Error convert bytes to string")]
    BytesToString(#[from] std::string::FromUtf8Error),
    #[error("Torrent hash was not found")]
    NoTorrentHash,
    #[error("Error convert string to i64")]
    StringToInt(#[from] std::num::ParseIntError),

    #[error("NC error")]
    Nc(#[from] netc::error::Error),
    #[error("json error")]
    Json(#[from] serde_json::Error),
    #[error("dotenv error")]
    DotEnv(#[from] dotenv::Error),
    #[error("response not success: {0}")]
    BadResponse(String),
    #[error("response no contain arguments")]
    NoArguments,
    #[error("unmutable fields in session-set")]
    WrongSessionSetFields,
    #[error("io error")]
    Io(#[from] std::io::Error),
    #[error("not auth")]
    NotAuth,
    #[error("TorrentAdd args have both filename and metadata")]
    BothFileMeta,
    #[error("TorrentAdd args no have filename or metadata")]
    NoFileMeta,
    #[error("Unknown torrent fields")]
    UnknownTorrentFields,
}
