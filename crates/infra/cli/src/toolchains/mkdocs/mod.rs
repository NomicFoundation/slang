use std::path::Path;

use anyhow::Result;
use infra_utils::cargo::CargoWorkspace;
use infra_utils::commands::Command;
use infra_utils::paths::PathExtensions;

use crate::toolchains::pipenv::PipEnv;
use crate::utils::DryRun;

pub struct Mkdocs;

impl Mkdocs {
    pub fn check() {
        mkdocs().arg("build").flag("--clean").flag("--strict").run();
    }

    pub fn watch() {
        // _MKDOCS_WATCH_PORT_ | keep in sync with the port number defined in "$REPO_ROOT/.devcontainer/devcontainer.json"
        const PORT: usize = 5353;

        mkdocs()
            .arg("serve")
            .flag("--clean")
            .flag("--watch-theme")
            .property("--dev-addr", format!("localhost:{PORT}"))
            .run();
    }

    pub fn publish_main_branch(dry_run: DryRun) {
        fetch_latest_remote();

        let mut command = mike().args(["deploy", "main"]);

        if !dry_run.get() {
            command = command.flag("--push");
        }

        command.run();
    }

    pub fn publish_latest_release(dry_run: DryRun) -> Result<()> {
        fetch_latest_remote();

        let version = CargoWorkspace::local_version()?.to_string();

        if mike().args(["list", &version]).evaluate().is_ok() {
            println!("Version '{version}' is already published.");
            return Ok(());
        }

        let mut command = mike()
            .args(["deploy", &version, "latest"])
            .flag("--update-aliases");

        if !dry_run.get() {
            command = command.flag("--push");
        }

        command.run();

        Ok(())
    }
}

fn fetch_latest_remote() {
    Command::new("git")
        .args(["fetch", "origin", "gh-pages"])
        .property("--depth", "1")
        .run();
}

fn mkdocs() -> Command {
    PipEnv::run("mkdocs").current_dir(Path::repo_path("documentation"))
}

fn mike() -> Command {
    PipEnv::run("mike").current_dir(Path::repo_path("documentation"))
}
