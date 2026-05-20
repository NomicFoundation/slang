use std::net::TcpStream;
use std::path::PathBuf;
use std::process::Child;
use std::time::{Duration, Instant};
use std::{fs, thread};

use anyhow::{anyhow, bail, Context, Result};
use clap::Parser;
use infra_utils::cargo::{CargoWorkspace, UserFacingV1Crate};
use infra_utils::commands::Command;
use semver::Version;
use serde::Deserialize;
use strum::IntoEnumIterator;
use tempfile::TempDir;
use url::Url;

use crate::utils::DryRun;

const LOCAL_REGISTRY_NAME: &str = "local";

#[derive(Deserialize)]
struct RegistryConfig {
    api: String,
}

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
        let local_version = CargoWorkspace::local_version()?;
        let mut crates_to_run: Vec<String> =
            UserFacingV1Crate::iter().map(|c| c.to_string()).collect();
        // Local smoke publishes every crate to a fresh empty registry,
        // so the crates.io version check doesn't apply.
        if !self.local_smoke {
            crates_to_run.retain(|name| needs_publish(name, &local_version));
        }

        if crates_to_run.is_empty() {
            println!("No crates to publish.");
            return Ok(());
        }

        if self.local_smoke {
            run_local_smoke(&crates_to_run)
        } else if self.dry_run.get() {
            run_batched_dry_run(&crates_to_run);
            Ok(())
        } else {
            for crate_name in &crates_to_run {
                run_cargo_publish(crate_name);
            }
            Ok(())
        }
    }
}

fn needs_publish(crate_name: &str, local_version: &Version) -> bool {
    if let Ok(published_version) = CargoWorkspace::published_version(crate_name) {
        println!("Published version of {crate_name}: {published_version}");
        println!("Local version: {local_version}");
        if *local_version == published_version {
            println!("Skipping crate {crate_name}, since the local version is already published.");
            return false;
        }
    } else {
        println!("No published version found for crate {crate_name}.");
    }
    true
}

/// `--no-verify` skips the verification compile — no workspace
/// `build.rs` or proc-macros run alongside the OIDC crates.io token.
/// cargo's own code (dep resolution, packaging) still runs.
fn run_cargo_publish(crate_name: &str) {
    Command::new("cargo")
        .arg("publish")
        .flag("--no-verify")
        .flag("--all-features")
        .property("--package", crate_name)
        .run();
}

/// Per-crate dry-runs against crates.io fail because the bumped versions
/// of internal deps aren't on crates.io yet. Batch them into one invocation
/// so cargo resolves internal path-deps locally.
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
    let registry = LocalRegistry::spawn()?;

    let result = publish_each(crate_names, registry.api_url());
    if result.is_err() {
        registry.surface_logs();
    }
    result
    // `registry` drops here: the child gets killed + reaped automatically.
}

fn publish_each(crate_names: &[String], api_url: &str) -> Result<()> {
    let index_url = format!("{api_url}/git");
    println!("Local registry ready (api={api_url}, index={index_url})");

    let registry_upper = LOCAL_REGISTRY_NAME.to_uppercase();
    let index_env = format!("CARGO_REGISTRIES_{registry_upper}_INDEX");
    let token_env = format!("CARGO_REGISTRIES_{registry_upper}_TOKEN");

    for name in crate_names {
        println!("--- Publishing {name} to local registry ---");
        // Bypass `infra_utils::commands::Command` here — its `run()` calls
        // `process::exit` on non-zero status, which would skip the registry
        // teardown by the surrounding `LocalRegistry` drop.
        let status = std::process::Command::new("cargo")
            .arg("publish")
            .args(["--no-verify", "--all-features"])
            // Throwaway registry — local dev workspaces are often dirty.
            .arg("--allow-dirty")
            .args(["--registry", LOCAL_REGISTRY_NAME])
            .args(["--package", name])
            .env(&index_env, &index_url)
            .env(&token_env, "fake-token-local-registry-ignores-it")
            // cargo-http-registry's index is plain HTTP git; libgit2 can't
            // fetch that, so this flag falls back to the system git binary.
            .env("CARGO_NET_GIT_FETCH_WITH_CLI", "true")
            .status()
            .with_context(|| format!("Failed to spawn cargo publish for {name}"))?;
        if !status.success() {
            bail!(
                "cargo publish to local registry failed for {name}: exit code {:?}",
                status.code(),
            );
        }
    }

    println!(
        "Local smoke published {} crates successfully.",
        crate_names.len()
    );
    Ok(())
}

/// A running `cargo-http-registry` instance. Owns its child process and the
/// tempdir it serves from; `Drop` kills + reaps the child so the server can't
/// outlive this value.
struct LocalRegistry {
    server: Child,
    // Held so the storage dir survives until Drop.
    storage: TempDir,
    api_url: String,
    stdout_log: PathBuf,
    stderr_log: PathBuf,
}

impl LocalRegistry {
    /// Spawn on an ephemeral port and wait until accepting connections.
    /// Caller is responsible for `install_binary("cargo-http-registry")` first.
    fn spawn() -> Result<Self> {
        let storage = tempfile::tempdir().context("Failed to create temp registry dir")?;
        println!("Local registry root: {}", storage.path().display());

        let stdout_log = storage.path().join("cargo-http-registry.stdout");
        let stderr_log = storage.path().join("cargo-http-registry.stderr");
        // Ephemeral port (`:0`): cargo-http-registry writes the assigned port
        // to `<root>/config.json` once bound; we read it back below.
        let server = std::process::Command::new("cargo-http-registry")
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

        let mut registry = Self {
            server,
            storage,
            api_url: String::new(),
            stdout_log,
            stderr_log,
        };

        // Drop fires on Err — child gets reaped — but logs would be lost
        // without an explicit surface here.
        if let Err(e) = registry.wait_until_ready() {
            registry.surface_logs();
            return Err(e);
        }
        Ok(registry)
    }

    fn wait_until_ready(&mut self) -> Result<()> {
        let api_url = read_registry_api_url(self.storage.path())?;
        wait_for_server(&api_url)?;
        self.api_url = api_url;
        Ok(())
    }

    fn api_url(&self) -> &str {
        &self.api_url
    }

    /// Print captured stdout/stderr from the server. Called only on failure;
    /// success runs don't need the noise.
    fn surface_logs(&self) {
        for (label, path) in [("stdout", &self.stdout_log), ("stderr", &self.stderr_log)] {
            print_log_if_present(&format!("cargo-http-registry {label}"), path);
        }
    }
}

impl Drop for LocalRegistry {
    fn drop(&mut self) {
        println!("Stopping local registry...");
        let _ = self.server.kill();
        let _ = self.server.wait();
    }
}

fn print_log_if_present(label: &str, path: &std::path::Path) {
    if let Ok(contents) = fs::read_to_string(path) {
        if !contents.trim().is_empty() {
            eprintln!("--- {label} ---\n{contents}");
        }
    }
}

/// Read the API URL cargo-http-registry writes to `<root>/config.json` after
/// binding, polling until the file appears.
fn read_registry_api_url(root: &std::path::Path) -> Result<String> {
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
    let socket = Url::parse(api_url)
        .with_context(|| format!("Invalid registry api URL: {api_url}"))?
        .socket_addrs(|| None)
        .with_context(|| format!("Could not resolve {api_url}"))?
        .into_iter()
        .next()
        .ok_or_else(|| anyhow!("{api_url} resolved to no addresses"))?;
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
