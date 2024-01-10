# Development

## Dev Containers

To make the developer experience as seamless and consistent as possible, we recommend using the VS Code [devcontainer](https://github.com/NomicFoundation/slang/tree/main/.devcontainer) included in this repository. It is a light image that has the minimum required tools to build this project. If you are not familiar with containerized development, I recommend taking a look at [the official VS Code guide](https://code.visualstudio.com/docs/remote/containers). Using a devcontainer allows us to quickly setup/teardown the environment, and install/setup different dependencies for different projects, without polluting the local environment. In the future, it will enable us to include Windows and Mac OS specific images for cross-platform testing.

If you would like to still develop outside a container, this should still be possible, as the CI will guarantee that your changes are safe. We intend to keep the images to a bare minimum, and install most dependencies through scripts you can run locally. However, using a common development container means sharing and standardizing useful settings and extensions for the editor (VS Code), the terminal (zsh), and any other tools.

In the future, if we decide to enable code spaces, we can have a 1-click button to create and warm up a powerful dev machine to use in seconds, and running completely remote in a browser tab. It will make it trivial to switch between different versions and branches, or even use and debug multiple ones at the same time from different tabs.

## Hermit

To install language-specific binaries and packages, we use [Hermit](https://cashapp.github.io/hermit/), which installs all tools only when it is first needed/invoked, so you can quickly setup and build different projects quickly. It also takes care of updating your `$PATH` as you `cd` in and out of different projects, to make sure you are using the right tools every time. Follow [this guide](https://cashapp.github.io/hermit/usage/get-started/) to install it locally to your machine, or simply build any included project, and it will bootstrap itself if it is missing.

## Infra CLI

To ensure consistency, and a good experience for first-time developers, all build/test/run/debug commands should be written, versioned, and documented inside the `infra_cli` crate. This means that any dev instructions are well documented, versioned, and verified/executed with every build. It also means that we can minimize any manual setup or teardown steps during development, and just rely on that cli.

You can access all such commands (from the hermit environment) by just running the `infra` script, which just refers to `$REPO_ROOT/scripts/bin/infra`. If this is your first time contributing, we recommend starting with `infra --help` to familiarize yourself with its capabilities.

## Versioning and Publishing

We manage versioning through [changesets](https://github.com/changesets/changesets). Each pull request can describe what user facing changes it is introducing, and include this information as a "changeset" markdown file along with source changes. These changeset files are analyzed and used to create another pull request to bump the repository version and update dependencies. Once the version bump is merged, artifacts are built and released to all registries.
