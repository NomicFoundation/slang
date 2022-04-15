use crate::{
    builds::{download_build, BuildInfo},
    utils::{download_file, with_progress},
};
use reqwest::Url;
use semver::Version;
use serde::Deserialize;
use std::{env, fs, path::PathBuf, time::Duration};

#[derive(Deserialize)]
struct MirrorResponse {
    builds: Vec<BuildEntry>,
}

#[derive(Deserialize)]
struct BuildEntry {
    path: String,
    version: String,
    build: String,
}

pub fn fetch_builds(binaries_dir: &PathBuf) -> Vec<BuildInfo> {
    if !binaries_dir.exists() {
        std::fs::create_dir_all(&binaries_dir).unwrap();
    }

    println!("Fetching list of available builds...");
    let builds = fetch_response(binaries_dir);

    let versions = builds
        .iter()
        .map(|b| b.version.to_string())
        .collect::<Vec<String>>();

    println!("Found {} builds:\n\n{:?}\n", builds.len(), versions);
    with_progress("Binaries", &builds, |build| download_build(build));

    return builds;
}

fn fetch_response(binaries_dir: &PathBuf) -> Vec<BuildInfo> {
    let mirror_url = match env::consts::OS {
        "linux" => Url::parse("https://binaries.soliditylang.org/linux-amd64/").unwrap(),
        "macos" => Url::parse("https://binaries.soliditylang.org/macosx-amd64/").unwrap(),
        _ => panic!("Platform not supported: {}", env::consts::OS),
    };

    let list_path = binaries_dir.join("list.json");
    let list_url = mirror_url.join("list.json").unwrap();

    if !list_path.exists() {
        download_file(&list_url, &list_path);
    } else {
        // Fetch again if it is older than one hour
        let metadata = fs::metadata(&list_path).unwrap();
        let one_hour = Duration::from_secs(60 * 60);
        if metadata.modified().unwrap().elapsed().unwrap() > one_hour {
            download_file(&list_url, &list_path);
        }
    }

    return parse_response(&list_path, &binaries_dir, &mirror_url);
}

fn parse_response(list_path: &PathBuf, binaries_dir: &PathBuf, mirror_url: &Url) -> Vec<BuildInfo> {
    let json = fs::read_to_string(&list_path).unwrap();
    let response: MirrorResponse = serde_json::from_str(&json).unwrap();

    let builds = response
        .builds
        .iter()
        .map(|build| {
            return BuildInfo {
                version: Version::parse(&build.version).unwrap(),
                commit: build.build.clone(),
                local_path: binaries_dir.join(&build.version),
                remote_url: mirror_url.join(&build.path).unwrap(),
            };
        })
        .collect::<Vec<BuildInfo>>();

    for i in 1..builds.len() {
        // Make sure the list is sorted
        let previous = &builds.get(i - 1).expect("exists").version;
        let current = &builds.get(i).expect("exists").version;
        assert!(previous.lt(&current));
    }

    return builds;
}
