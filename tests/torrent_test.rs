// use rqa::{
//     torrent::{TorrentAddArgs, TorrentGetArgs},
//     Client,
// };

// const MAGNET: &str = "magnet:?xt=urn:btih:6a0a9282c65fc6a1324e6e1605fe9bb9746c3aa8&dn=test%20dir";

// #[tokio::test]
// async fn test_torrent_add_meta() {
//     let uri = dotenv::var("TRPC_TARGET").expect("not set TRPC_TARGET");

//     let mut client = Client::new(&uri);
//     let add_args = TorrentAddArgs::from_meta("tests\\test dir.torrent").unwrap();
//     let body = client.torrent_add(add_args).await.unwrap();
//     dbg!(body);
// }

// #[tokio::test]
// async fn test_torrent_add_uri() {
//     let uri = dotenv::var("TRPC_TARGET").expect("not set TRPC_TARGET");

//     let mut client = Client::new(&uri);
//     let add_args = TorrentAddArgs::from_file(MAGNET).unwrap();
//     let body = client.torrent_add(add_args).await.unwrap();
//     dbg!(&body);
//     assert!(body.torrent_duplicate.is_some());
// }

// #[tokio::test]
// async fn test_torrent_add_with_file_and_meta() {
//     let uri = dotenv::var("TRPC_TARGET").expect("not set TRPC_TARGET");

//     let mut client = Client::new(&uri);
//     let mut add_args = TorrentAddArgs::from_meta("tests\\test dir.torrent").unwrap();
//     add_args.metainfo = Some(MAGNET.to_string());
//     let body = client.torrent_add(add_args).await;
//     assert!(body.is_err());
// }

// #[tokio::test]
// async fn test_torrent_add_without_file_and_meta() {
//     let uri = dotenv::var("TRPC_TARGET").expect("not set TRPC_TARGET");

//     let mut client = Client::new(&uri);
//     let mut add_args = TorrentAddArgs::from_meta("tests\\test dir.torrent").unwrap();
//     add_args.metainfo = None;
//     let body = client.torrent_add(add_args).await;
//     assert!(body.is_err());
// }

// #[tokio::test]
// async fn test_get_torrent() {
//     let uri = dotenv::var("TRPC_TARGET").expect("not set TRPC_TARGET");

//     let mut client = Client::new(&uri);
//     let get_args = TorrentGetArgs {
//         ids: Some(vec!["6a0a9282c65fc6a1324e6e1605fe9bb9746c3aa8".into()].into()),
//         fields: vec!["id".try_into().unwrap(), "hashstring".try_into().unwrap()],
//     };
//     let body = client.torrent_get(get_args).await;
//     assert!(body.is_ok());
//     let torrents = body.unwrap().torrents;
//     assert!(!torrents.is_empty());
//     let torrent = &torrents[0];
//     assert!(torrent.id.is_some());
//     assert!(torrent.hash_string.is_some());
//     assert!(torrent.name.is_none());
// }
