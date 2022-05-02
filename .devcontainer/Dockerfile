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

FROM ubuntu:21.10
RUN apt-get update

# Shell

ENV HOME="${HOME:-"/root"}"
ENV PATH="$HOME/bin:$PATH"
COPY "./scripts" "$HOME"

RUN apt-get install --assume-yes \
  "bash=5.1-3ubuntu2" \
  "curl=7.74.0-1.3ubuntu2.1" \
  "git=1:2.32.0-1ubuntu1" \
  "zsh=5.8-6ubuntu0.1"

RUN curl -fsSL "https://github.com/cashapp/hermit/releases/download/v0.18.3/install.sh" | bash

# Rust

RUN apt-get install --assume-yes \
  "build-essential=12.9ubuntu2" \
  "pkg-config=0.29.2-1ubuntu1" \
  "libssl-dev=1.1.1l-1ubuntu1.2"
