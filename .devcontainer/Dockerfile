############################################################################################
#                                                                                          #
#                                                                                          #
#     !!!!!!                KEEP THIS IMAGE AS MINIMAL AS POSSIBLE              !!!!!!     #
#                                                                                          #
#                                                                                          #
#     This image should be kept as minimal as possible; only installing the bare           #
#     essentials needed to run the repository, or fetch other tools.                       #
#                                                                                          #
#     Any dependency that is used to build/test this project should be installed and       #
#     configured from within the project that needs it (using Hermit).                     #
#                                                                                          #
#     1) Smaller container size means we can fetch and create it quicker.                  #
#     2) Different projects can require different environments/dependencies.               #
#     3) Maximum portability, for when we start supporting Windows/MacOS builds.           #
#     4) Makes it easier to work on this repository outside the container if needed.       #
#                                                                                          #
#                                                                                          #
############################################################################################

# Base Container

FROM ubuntu:22.04
RUN apt-get update

# Shell

RUN apt-get install --assume-yes \
  "bash=5.*" \
  "curl=7.*" \
  "git=1:2.*" \
  "zsh=5.*"

COPY "./scripts/*" "$HOME/"
RUN curl -fsSL "https://github.com/cashapp/hermit/releases/download/v0.26.1/install.sh" | bash

# Rust

RUN apt-get install --assume-yes \
  "build-essential=12.*" \
  "pkg-config=0.*" \
  "libssl-dev=3.*"
