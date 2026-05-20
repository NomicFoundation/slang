use std::net::TcpStream;
use std::path::{Path, PathBuf};
use std::process::Child;
use std::time::{Duration, Instant};
use std::{fs, thread};

use anyhow::{anyhow, bail, Context, Result};
use infra_utils::cargo::CargoWorkspace;
use serde::Deserialize;
use tempfile::TempDir;
use url::Url;

const LOCAL_REGISTRY_NAME: &str = "local";

#[derive(Deserialize)]
struct RegistryConfig {
    api: String,
}

/// Entrypoint for `infra publish cargo --local-smoke`. Spins up a local
/// `cargo-http-registry`, runs the same per-crate `cargo publish --no-verify`
/// production uses, then tears it down.
pub(super) fn run(crate_names: &[String]) -> Result<()> {
    let registry = LocalRegistry::spawn()?;

    let result = registry.publish_each(crate_names);
    if result.is_err() {
        registry.surface_logs();
    }
    result
    // `registry` drops here: the child gets killed + reaped automatically.
}

/// A running `cargo-http-registry` instance. Owns its child process and the
/// tempdir it serves from; `Drop` kills + reaps the child so the server can't
/// outlive this value.
struct LocalRegistry {
    server: Child,
    // Held so the storage dir survives until Drop.
    _storage: TempDir,
    api_url: String,
    stdout_log: PathBuf,
    stderr_log: PathBuf,
}

impl LocalRegistry {
    /// Install `cargo-http-registry`, spawn it on an ephemeral port, and wait
    /// until it's accepting connections.
    fn spawn() -> Result<Self> {
        CargoWorkspace::install_binary("cargo-http-registry")?;

        let storage = tempfile::tempdir().context("Failed to create temp registry dir")?;
        println!("Local registry root: {}", storage.path().display());

        let stdout_log = storage.path().join("cargo-http-registry.stdout");
        let stderr_log = storage.path().join("cargo-http-registry.stderr");
        // Ephemeral port (`:0`): cargo-http-registry writes the assigned port
        // to `<root>/config.json` once bound; we read it back below.
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

        let api_url = match wait_for_ready(storage.path()) {
            Ok(url) => url,
            Err(e) => {
                // Readiness failed; we own a running child and need to clean
                // it up before returning. Drop won't fire because we never
                // constructed Self.
                surface_logs_from(&stdout_log, &stderr_log);
                let _ = server.kill();
                let _ = server.wait();
                return Err(e);
            }
        };

        Ok(Self {
            server,
            _storage: storage,
            api_url,
            stdout_log,
            stderr_log,
        })
    }

    fn publish_each(&self, crate_names: &[String]) -> Result<()> {
        let index_url = format!("{api}/git", api = self.api_url);
        println!(
            "Local registry ready (api={api}, index={index_url})",
            api = self.api_url,
        );

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

    /// Print captured stdout/stderr from the server. Called only on failure;
    /// success runs don't need the noise.
    fn surface_logs(&self) {
        surface_logs_from(&self.stdout_log, &self.stderr_log);
    }
}

impl Drop for LocalRegistry {
    fn drop(&mut self) {
        println!("Stopping local registry...");
        let _ = self.server.kill();
        let _ = self.server.wait();
    }
}

fn surface_logs_from(stdout_log: &Path, stderr_log: &Path) {
    for (label, path) in [("stdout", stdout_log), ("stderr", stderr_log)] {
        print_log_if_present(&format!("cargo-http-registry {label}"), path);
    }
}

fn print_log_if_present(label: &str, path: &Path) {
    if let Ok(contents) = fs::read_to_string(path) {
        if !contents.trim().is_empty() {
            eprintln!("--- {label} ---\n{contents}");
        }
    }
}

/// Wait until cargo-http-registry has written `<root>/config.json` *and* the
/// advertised api endpoint accepts TCP connections.
fn wait_for_ready(root: &Path) -> Result<String> {
    let api_url = read_registry_api_url(root)?;
    wait_for_server(&api_url)?;
    Ok(api_url)
}

/// Read the API URL cargo-http-registry writes to `<root>/config.json` after
/// binding, polling until the file appears.
fn read_registry_api_url(root: &Path) -> Result<String> {
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
