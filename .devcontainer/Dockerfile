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

# OS Packages

RUN apt-get install --assume-yes \
  "bash=5.*" \
  "build-essential=12.*" \
  "curl=7.*" \
  "git=1:2.*" \
  "libssl-dev=3.*" \
  "pkg-config=0.*" \
  "zsh=5.*"

# Local Scripts

COPY ./scripts/ /root/
