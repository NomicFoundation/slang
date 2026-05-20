use std::net::{SocketAddr, TcpStream};
use std::process::Stdio;
use std::time::{Duration, Instant};
use std::{fs, thread};

use anyhow::{bail, Context, Result};
use clap::Parser;
use infra_utils::cargo::{CargoWorkspace, UserFacingV1Crate};
use infra_utils::commands::Command;
use serde::Deserialize;
use strum::IntoEnumIterator;

use crate::utils::DryRun;

const LOCAL_REGISTRY_NAME: &str = "local";

#[derive(Clone, Debug, Parser)]
pub struct CargoController {
    #[command(flatten)]
    dry_run: DryRun,

    /// Smoke-test the per-crate publish flow end-to-end against a local
    /// `cargo-http-registry` instead of crates.io. Used in CI's `review` job
    /// to catch flag-set drift and sequential-flow issues that the existing
    /// batched `--dry-run` doesn't exercise. Mutually exclusive with --dry-run.
    #[arg(long, conflicts_with = "dry_run")]
    local_smoke: bool,
}

impl CargoController {
    pub fn execute(&self) -> Result<()> {
        // The local-smoke registry starts empty, so every user-facing crate
        // gets published to it regardless of what's on crates.io — we're
        // exercising the publish *path*, not bringing crates.io up to date.
        let crates_to_run: Vec<String> = if self.local_smoke {
            UserFacingV1Crate::iter().map(|c| c.to_string()).collect()
        } else {
            let mut v = vec![];
            for c in UserFacingV1Crate::iter() {
                let name = c.to_string();
                if needs_publish(&name)? {
                    v.push(name);
                }
            }
            v
        };

        if crates_to_run.is_empty() {
            println!("No crates to publish.");
            return Ok(());
        }

        if self.local_smoke {
            run_local_smoke(&crates_to_run)
        } else if self.dry_run.get() {
            // Per-crate dry-runs against crates.io fail because the bumped
            // versions of internal deps aren't on crates.io yet. Batch them
            // into one invocation so cargo resolves internal path-deps locally.
            run_batched_dry_run(&crates_to_run);
            Ok(())
        } else {
            // `--no-verify` skips the verification compile so no `build.rs` or
            // proc-macros from the workspace dep graph run alongside the
            // OIDC-exchanged crates.io token. cargo still does dep resolution
            // + tarball creation, but those are cargo's own code paths.
            for crate_name in &crates_to_run {
                run_cargo_publish(crate_name);
            }
            Ok(())
        }
    }
}

fn needs_publish(crate_name: &str) -> Result<bool> {
    if let Ok(published_version) = CargoWorkspace::published_version(crate_name) {
        println!("Published version of {crate_name}: {published_version}");
        let local_version = CargoWorkspace::local_version()?;
        println!("Local version: {local_version}");
        if local_version == published_version {
            println!("Skipping crate {crate_name}, since the local version is already published.");
            return Ok(false);
        }
    } else {
        println!("No published version found for crate {crate_name}.");
    }
    Ok(true)
}

fn run_cargo_publish(crate_name: &str) {
    Command::new("cargo")
        .arg("publish")
        .flag("--no-verify")
        .flag("--all-features")
        .property("--package", crate_name)
        .run();
}

fn run_batched_dry_run(crate_names: &[String]) {
    let mut command = Command::new("cargo")
        .arg("publish")
        .flag("--all-features")
        .flag("--dry-run");
    for crate_name in crate_names {
        command = command.property("--package", crate_name);
    }
    command.run();
}

/// Spin up a local `cargo-http-registry`, run the same per-crate sequential
/// `cargo publish --no-verify` we'll run in production, then tear it down.
/// Catches:
/// - Flag-set drift between dry-run and real publish.
/// - Sequential-flow issues (each crate's upload must propagate before the
///   next crate's dep resolution).
/// - Cargo+slang interaction with the real publish code path.
///
/// Does NOT catch crates.io-specific server rejection (badge slug whitelist,
/// SPDX license validation, name conflicts) — the local registry accepts
/// anything cargo sends.
fn run_local_smoke(crate_names: &[String]) -> Result<()> {
    CargoWorkspace::install_binary("cargo-http-registry")?;

    let storage = tempfile::tempdir().context("Failed to create temp registry dir")?;
    println!("Local registry root: {}", storage.path().display());

    // `--addr 127.0.0.1:0` asks the kernel for an ephemeral port; cargo-http-registry
    // writes the assigned port to `<root>/config.json` once it has bound. Ephemeral
    // is safer than a pinned port for parallel CI jobs or local dev machines.
    let mut server = std::process::Command::new("cargo-http-registry")
        .arg(storage.path())
        .args(["--addr", "127.0.0.1:0"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .context("Failed to spawn cargo-http-registry")?;

    // Run the smoke test with explicit Result-based control flow; the inner
    // function's errors propagate up, then we *always* kill the server below.
    // `Drop` won't help because the underlying `Command::run` for cargo publish
    // calls `process::exit` on failure, bypassing destructors.
    let result = run_smoke_inner(storage.path(), crate_names);

    println!("Stopping local registry...");
    let _ = server.kill();
    let _ = server.wait();

    result
}

fn run_smoke_inner(storage_root: &std::path::Path, crate_names: &[String]) -> Result<()> {
    let api_url = read_registry_api_url(storage_root)?;
    wait_for_server(&api_url)?;
    let index_url = format!("{api_url}/git");
    println!("Local registry ready (api={api_url}, index={index_url})");

    let registry_upper = LOCAL_REGISTRY_NAME.to_uppercase();
    let index_env = format!("CARGO_REGISTRIES_{registry_upper}_INDEX");
    let token_env = format!("CARGO_REGISTRIES_{registry_upper}_TOKEN");

    for name in crate_names {
        println!("--- Publishing {name} to local registry ---");
        // Use std::process::Command directly so failures propagate as Err
        // instead of `process::exit`, letting us tear down the server before
        // surfacing the error.
        let status = std::process::Command::new("cargo")
            .arg("publish")
            .args(["--no-verify", "--all-features"])
            // `--allow-dirty` is safe here: this runs against a throwaway
            // registry, and locally the working tree is often dirty from
            // in-progress edits. In CI the workspace is freshly checked out,
            // so the flag is a no-op there.
            .arg("--allow-dirty")
            .args(["--registry", LOCAL_REGISTRY_NAME])
            .args(["--package", name])
            .env(&index_env, &index_url)
            // cargo-http-registry documents that it doesn't validate tokens
            // (accepts any string), but cargo refuses to publish without one.
            // Set per-registry so it doesn't leak into other commands.
            .env(&token_env, "fake-token-local-registry-ignores-it")
            // cargo-http-registry serves the index over plain HTTP git;
            // cargo's built-in libgit2 won't talk plain HTTP, so shell out to
            // the system git binary (present on all CI runners).
            .env("CARGO_NET_GIT_FETCH_WITH_CLI", "true")
            .status()
            .with_context(|| format!("Failed to invoke cargo publish for {name}"))?;
        if !status.success() {
            bail!(
                "cargo publish to local registry failed for {name}: exit code {}",
                status.code().map_or("?".to_owned(), |c| c.to_string()),
            );
        }
    }

    println!(
        "Local smoke published {} crates successfully.",
        crate_names.len()
    );
    Ok(())
}

/// `cargo-http-registry` writes its assigned address to `<root>/config.json`
/// once it has bound the socket. With `--addr 127.0.0.1:0` we need to read
/// that file to learn which port the kernel handed us.
fn read_registry_api_url(root: &std::path::Path) -> Result<String> {
    #[derive(Deserialize)]
    struct RegistryConfig {
        api: String,
    }
    let config_path = root.join("config.json");
    let deadline = Instant::now() + Duration::from_secs(15);
    let mut last_err = None;
    while Instant::now() < deadline {
        if config_path.exists() {
            match fs::read_to_string(&config_path) {
                Ok(contents) => match serde_json::from_str::<RegistryConfig>(&contents) {
                    Ok(cfg) => return Ok(cfg.api),
                    Err(e) => last_err = Some(format!("parse error: {e}")),
                },
                Err(e) => last_err = Some(format!("read error: {e}")),
            }
        }
        thread::sleep(Duration::from_millis(100));
    }
    bail!(
        "Timed out waiting for cargo-http-registry to write {config_path:?}{}",
        last_err
            .map(|e| format!(" (last: {e})"))
            .unwrap_or_default(),
    );
}

/// Verify the server is actually serving on its advertised address (not just
/// that the file appeared on disk).
fn wait_for_server(api_url: &str) -> Result<()> {
    let host_and_port = api_url
        .trim_start_matches("http://")
        .trim_start_matches("https://");
    let socket: SocketAddr = host_and_port
        .parse()
        .with_context(|| format!("Invalid registry api URL: {api_url}"))?;
    let deadline = Instant::now() + Duration::from_secs(15);
    while Instant::now() < deadline {
        if TcpStream::connect_timeout(&socket, Duration::from_millis(200)).is_ok() {
            thread::sleep(Duration::from_millis(100));
            return Ok(());
        }
        thread::sleep(Duration::from_millis(100));
    }
    bail!("cargo-http-registry did not accept TCP connections at {socket} within 15s");
}
