use url::Url;

use crate::error::Error;

#[derive(Clone, Debug)]
pub struct Client {
    pub(crate) url: Url,
    pub(crate) cookie: String,
}

impl Client {
    pub fn new(uri: &str) -> Result<Client, Error> {
        let options = Url::options();
        let api = Url::parse(uri)?;
        let base_url = options.base_url(Some(&api));
        let url = base_url.parse("api/v2/")?;

        Ok(Client {
            url,
            cookie: String::new(),
        })
    }
}
