/// All Authentication API methods are under "auth", e.g.: /api/v2/auth/methodName.
/// qBittorrent uses cookie-based authentication.
use crate::{
    request::{ApiRequest, Arguments, Method},
    response::check_default_status,
    Client, Error,
};

impl Client {
    /// Login
    ///
    /// Name: login
    ///
    /// Parameters:
    /// Parameter  Type  Description
    /// username  string  Username used to access the WebUI
    /// password  string  Password used to access the WebUI
    ///
    /// Returns:
    /// HTTP Status Code  Scenario
    /// 403  User's IP is banned for too many failed login attempts
    /// 200  All other scenarios
    ///
    /// Upon success, the response will contain a cookie with your SID. You must supply the cookie whenever you want to perform an operation that requires authentication.
    ///
    /// Example showing how to login and execute a command that requires authentication using curl:
    ///
    /// $ curl -i --header 'Referer: http://localhost:8080' --data 'username=admin&password=adminadmin' http://localhost:8080/api/v2/auth/login
    /// HTTP/1.1 200 OK
    /// Content-Encoding:
    /// Content-Length: 3
    /// Content-Type: text/plain; charset=UTF-8
    /// Set-Cookie: SID=hBc7TxF76ERhvIw0jQQ4LZ7Z1jQUV0tQ; path=/
    /// $ curl http://localhost:8080/api/v2/torrents/info --cookie "SID=hBc7TxF76ERhvIw0jQQ4LZ7Z1jQUV0tQ"
    ///
    /// Note: Set Referer or Origin header to the exact same domain and port as used in the HTTP query Host header.
    ///
    pub async fn login(&mut self, username: &str, password: &str) -> Result<(), Error> {
        let request = ApiRequest {
            method: Method::Login,
            arguments: Some(Arguments::Form(format!(
                "username={}&password={}",
                username, password
            ))),
        };
        let response = self.send_request(&request).await?;
        match response.status_code().as_u16() {
            200 => Ok(()),
            403 => Err(Error::Banned),
            _ => Err(Error::WrongStatusCode),
        }
    }

    /// Logout
    ///
    /// Name: logout
    ///
    /// Parameters:
    ///
    /// None
    ///
    /// Returns:
    /// HTTP Status Code Scenario
    /// 200 All scenarios
    ///
    /// None
    ///
    pub async fn logout(&mut self) -> Result<(), Error> {
        let request = ApiRequest {
            method: Method::Logout,
            arguments: None,
        };
        let response = self.send_request(&request).await?;
        self.cookie = String::new();
        check_default_status(&response, ())
    }
}
