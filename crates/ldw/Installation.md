# Installation

There are three main methods to install and set up the Language Design Workbench:

## 1. Using Devbox with direnv (Recommended)

If you have [devbox](https://www.jetpack.io/devbox/), [Nix](https://nixos.org/),
and [direnv](https://direnv.net/) installed on your system:

1. Clone this repository:
    ```
    git clone <repository-url>
    cd language-design-workbench
    ```
2. Allow direnv to load the environment:
    ```
    direnv allow
    ```
3. Install dependencies:
    ```
    npm install
    ```

Note: We recommend installing the [direnv extension for
VSCode](https://marketplace.visualstudio.com/items?itemName=mkhl.direnv) for
seamless integration. The project already includes a `.envrc` file for direnv
configuration.

## 2. Manual Installation

If you prefer to use your local Node.js and npm installation:

1. Ensure you have [Node.js](https://nodejs.org/) and [npm](https://www.npmjs.com/) installed on your system.
2. Clone this repository:
    ```
    git clone <repository-url>
    cd language-design-workbench
    ```
3. Install dependencies:
    ```
    npm install
    ```

## 3. Using a Devcontainer

If you're using Visual Studio Code with the Remote - Containers extension:

1. Use the "Clone Repository in Container Volume" option when cloning the
   repository. This approach is recommended over checking out the source and then
   reopening in the container, as it avoids potential performance issues associated
   with mounting local volumes.

2. VSCode will automatically set up the development environment and install
   dependencies.

Note: Be aware that using a local volume mount with devcontainers can have
performance implications, especially on non-Linux hosts. The "Clone Repository
in Container Volume" approach helps mitigate these issues.
