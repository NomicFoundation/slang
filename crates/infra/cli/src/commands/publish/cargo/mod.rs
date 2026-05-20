use std::net::{SocketAddr, TcpStream};
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
    /// Tests the verify-compile of each packaged crate, which the local smoke test can't cover.
    #[command(flatten)]
    dry_run: DryRun,

    /// Publish to a local `cargo-http-registry` instead of crates.io,
    /// exercising the same per-crate sequential path production uses.
    /// Mutually exclusive with --dry-run.
    #[arg(long, conflicts_with = "dry_run")]
    local_smoke: bool,
}

impl CargoController {
    pub fn execute(&self) -> Result<()> {
        // Local registry starts empty; publish every user-facing crate
        // regardless of crates.io state.
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

/// Spin up a local `cargo-http-registry`, run the same per-crate
/// `cargo publish --no-verify` production uses, tear it down.
fn run_local_smoke(crate_names: &[String]) -> Result<()> {
    CargoWorkspace::install_binary("cargo-http-registry")?;

    let storage = tempfile::tempdir().context("Failed to create temp registry dir")?;
    println!("Local registry root: {}", storage.path().display());

    // Ephemeral port (`:0`): cargo-http-registry writes the assigned port to
    // `<root>/config.json` once bound; we read it back below.
    let stdout_log = storage.path().join("cargo-http-registry.stdout");
    let stderr_log = storage.path().join("cargo-http-registry.stderr");
    let mut server = std::process::Command::new("cargo-http-registry")
        .arg(storage.path())
        .args(["--addr", "127.0.0.1:0"])
        .stdout(
            fs::File::create(&stdout_log)
                .with_context(|| format!("Failed to create {stdout_log:?}"))?,
        )
        .stderr(
            fs::File::create(&stderr_log)
                .with_context(|| format!("Failed to create {stderr_log:?}"))?,
        )
        .spawn()
        .context("Failed to spawn cargo-http-registry")?;

    let result = run_smoke_inner(storage.path(), crate_names);

    println!("Stopping local registry...");
    let _ = server.kill();
    let _ = server.wait();

    if result.is_err() {
        print_log_if_present("cargo-http-registry stdout", &stdout_log);
        print_log_if_present("cargo-http-registry stderr", &stderr_log);
    }

    result
}

fn print_log_if_present(label: &str, path: &std::path::Path) {
    if let Ok(contents) = fs::read_to_string(path) {
        if !contents.trim().is_empty() {
            eprintln!("--- {label} ---\n{contents}");
        }
    }
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
        // std::process::Command instead of the project's Command so failures
        // return Err and let the caller kill the server before exiting.
        let status = std::process::Command::new("cargo")
            .arg("publish")
            .args(["--no-verify", "--all-features"])
            // Throwaway registry — local dev workspaces are often dirty.
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
            // TCP bind can return before warp's accept loop is ready to serve HTTP.
            thread::sleep(Duration::from_millis(100));
            return Ok(());
        }
        thread::sleep(Duration::from_millis(100));
    }
    bail!("cargo-http-registry did not accept TCP connections at {socket} within 15s");
}
