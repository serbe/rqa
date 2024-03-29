// use std::collections::HashMap;

// use serde::{Deserialize, Serialize};
// use serde_json::json;
// use crate::sync::GetPeersData;
use tokio::runtime::Runtime;
use tokio::time::{sleep, Duration};

use client::Client;
use error::Error;

// use torrent::{TorrentAddArgs, TorrentGetArgs, TorrentRemoveArgs};

pub mod app;
pub mod auth;
pub mod client;
pub mod error;
pub mod log;
pub mod request;
pub mod response;
pub mod sync;
pub mod torrents;
pub mod transfer;

async fn run() -> Result<(), Error> {
    let uri = dotenv::var("QAPI_TARGET").expect("not set QAPI_TARGET");
    let mut client = Client::new(&uri)?;

    let username = dotenv::var("QAPI_USERNAME").expect("not set QAPI_USERNAME");
    let password = dotenv::var("QAPI_PASSWORD").expect("not set QAPI_PASSWORD");

    client.login(&username, &password).await?;

    dbg!(client.get_version().await?);
    dbg!(client.get_api_version().await?);

    let urls = "magnet:?xt=urn:btih:dc05fd2481d6ca52f767183c70ac383e831f4ed1&dn=rutor.info_The+Sims+4%3A+Deluxe+Edition+%5Bv+1.91.186.1030+%2F+1.91.186.1530+%2B+DLCs%5D+%282014%29+PC+%7C+RePack+от+Chovka&tr=udp://opentor.net:6969&tr=http://retracker.local/announce".to_string();
    let category = Some("games".to_string());

    let v: crate::torrents::AddTorrent = torrents::AddTorrent {
        urls,
        category,
        ..Default::default()
    };

    dbg!(client.add_torrent(v).await?);

    // dbg!(
    //     client
    //         .reannounce_torrent(vec!["8658006eaac03dbd7bf6901b4288c22c578a4836"])
    //         .await?
    // );

    // dbg!(client.toggle_alt_speed().await?);
    // dbg!(client.get_download_limit().await?);
    // dbg!(client.set_download_limit(512000).await?);
    // dbg!(client.get_download_limit().await?);
    // dbg!(client.toggle_alt_speed().await?);

    // let req = crate::sync::GetPeersData {
    //     rid: 45,
    //     hash: "14061948332125cc81b0c7466d2bd33ee0f26f46".to_string(),
    // };
    // dbg!(client.get_peers_data(req).await?);

    // dbg!(client);

    // dbg!(client);
    // let get_args = TorrentGetArgs {
    //     ids: Some(vec!["6a0a9282c65fc6a1324e6e1605fe9bb9746c3aa8".into()].into()),
    //     fields: vec!["id".try_into().unwrap()],
    // };

    // let body = client.torrent_get(get_args).await.unwrap();
    // dbg!(body);

    // let add_args = TorrentAddArgs::from_meta("tests\\test dir.torrent").unwrap();

    // let body = client.torrent_add(add_args).await.unwrap();
    // dbg!(body);

    // let del_args = TorrentRemoveArgs {
    //     ids: Ids::Array(vec![Id::Hash(
    //         "6a0a9282c65fc6a1324e6e1605fe9bb9746c3aa8".to_string(),
    //     )]),
    //     delete_local_data: true,
    // };

    // let body = client.torrent_remove(del_args).await.unwrap();
    // dbg!(body);

    sleep(Duration::from_millis(2000)).await;
    Ok(())
}

fn main() {
    dotenv::dotenv().ok().unwrap();
    env_logger::init();

    let rt = Runtime::new().unwrap();

    rt.block_on(async {
        if let Err(err) = run().await {
            eprintln!("{err:?}");
        }
    });
}
