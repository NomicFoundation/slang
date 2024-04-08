#!/bin/bash

# These are environment variables used during the build process
# Hence you should be running this scrit with `source` command
# Example: `source ./dev_build.sh` or `. ./dev_build.sh`
# Don't run this script with `./dev_build.sh` as it will not set 
# the environment variables in the current shell!
export REPO_ROOT=$(pwd)
export RUST_NIGHTLY_VERSION=1.76
export RUST_STABLE_VERSION=1.76.0


RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color


command_exists() {
    command -v "$1" >/dev/null 2>&1
}


print_message() {
    echo -e "\n${YELLOW}================= $1 ==================${NC}\n"
}


if ! command_exists rustup; then
    echo -e "${RED}Rust not installed. Install Rust and try again.${NC}"
    exit 1
fi


if ! command_exists npm; then
    echo -e "${RED}Node.js and npm not installed. Install Node.js and try again.${NC}"
    exit 1
fi


OS=""
case "$(uname -s)" in
    Linux*)     OS="Linux";;
    Darwin*)    OS="Mac";;
    *)          echo -e "${RED}Unsupported OS${NC}"; exit 1;;
esac


echo -e "\n${GREEN}Detected OS: $OS${NC}"


print_message "Checking pkg-config installation"
if command_exists pkg-config; then
    echo -e "${GREEN}pkg-config is already installed.${NC}"  
else
    echo -e "${YELLOW}pkg-config not found. Installing...${NC}"

    if [ "$OS" = "Linux" ]; then
        sudo apt-get install --yes pkg-config || {
            echo -e "${RED}Failed to install pkg-config.${NC}"; exit 1;
        }
    elif [ "$OS" = "Mac" ]; then
        brew install pkg-config || {
            echo -e "${RED}Failed to install pkg-config.${NC}"; exit 1;
        }
    fi

    echo -e "${GREEN}pkg-config installation completed.${NC}"
fi


print_message "Setting Rust versions"
rustup default $RUST_STABLE_VERSION || {
    echo -e "${RED}Failed to set Rust default version.${NC}"; exit 1;
}
rustup toolchain install nightly $RUST_NIGHTLY_VERSION || {
    echo -e "${RED}Failed to install Rust nightly toolchain.${NC}"; exit 1;
}


print_message "Setting npm prettier globally"
npm install -g prettier || {
    echo -e "${RED}Failed to install prettier globally.${NC}"; exit 1;
}


print_message "Installing project dependencies"
npm install || {
    echo -e "${RED}Failed to install project dependencies.${NC}"; exit 1;
}
npm fund || {
    echo -e "${RED}Failed to display funding information.${NC}"; exit 1;
}


print_message "Now compiling the repo"
cargo build || {
    echo -e "${RED}Failed to build the project.${NC}"; exit 1;
}


echo -e "\n${GREEN} Repo built successfully!${NC}\n"
