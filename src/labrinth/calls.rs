//! his file contains abstractions of some of the calls supported by the Labrinth API

use super::structs::*;
use bytes::Bytes;
use reqwest::StatusCode;
use reqwest::{Client, Response};

/// Return the contents of `version`'s JAR file as bytes
pub async fn download_version(client: &Client, version: &Version) -> Bytes {
    request(client, version.files[0].url.clone())
        .await
        .bytes()
        .await
        .unwrap()
}

/// Checks if a mod exists. If it does, then the mod is returned, else None is returned
pub async fn does_exist(client: &Client, mod_id: &ID) -> Option<Mod> {
    let response = request_rel(client, format!("/mod/{}", mod_id)).await;
    match response.status() {
        StatusCode::OK => response.json().await.unwrap(),
        _ => Option::None,
    }
}

/// Returns the versions of `mod_id`'s mod sorted in chronologically descending order
pub async fn get_versions(client: &Client, mod_id: &str) -> Vec<Version> {
    request_rel(client, format!("/mod/{}/version", mod_id))
        .await
        .json()
        .await
        .unwrap()
}

/// Get a mod using the `mod_slug`, which can also be the mod ID
pub async fn get_mod(client: &Client, mod_slug: &str) -> Mod {
    request_rel(client, format!("/mod/{}", mod_slug))
        .await
        .json()
        .await
        .unwrap()
}

/// Send a request to `url` with `client` and return response. Labrinth's base URL will be prepended to `url`
pub async fn request_rel(client: &Client, url: String) -> Response {
    request(
        client,
        format!("https://api.modrinth.com/api/v1{}", url).into(),
    )
    .await
}

/// Send a request to `url` with `client` and return response
pub async fn request(client: &Client, url: String) -> Response {
    let response = client.get(url).send().await.unwrap();
    if response.status().is_success() {
        response
    } else {
        panic!("HTTP request failed with error code {}", response.status());
    }
}
