description = "Node.js® is a JavaScript runtime built on Chrome's V8 JavaScript engine."
test = "node --version"
binaries = ["bin/*"]
source = "https://nodejs.org/dist/v${version}/node-v${version}-${os}-x64.tar.xz"
strip = 1
env = {
  "NPM_CONFIG_PREFIX": "${HERMIT_ENV}/.hermit/node",
  "PATH": "${HERMIT_ENV}/node_modules/.bin:${NPM_CONFIG_PREFIX}/bin:${PATH}",
}

version "12.18.3" "12.22.6" "12.22.7" "12.22.12" "14.16.0" "14.17.0" "14.17.3"
        "14.18.0" "14.19.0" "16.9.0" "15.10.0" {
}

// # arm64 only available for >=16
version "16.1.0" "16.2.0" "16.3.0" "16.4.0" "16.5.0" "16.6.0" "16.6.1" "16.6.2"
        "16.7.0" "16.8.0" "16.9.1" "16.10.0" "16.11.0" "17.0.0" "17.0.1" "16.13.0" "17.1.0"
        "16.13.1" "17.3.0" "17.3.1" "17.4.0" "16.14.0" "17.5.0" "17.6.0" "17.7.0" "17.7.1"
        "16.14.1" "17.7.2" "17.8.0" "17.9.0" "18.0.0" "16.15.0" "18.1.0" "14.19.2" "12.22.9"
        "18.2.0" {
  darwin {
    arch = "arm64"
    source = "https://nodejs.org/dist/v${version}/node-v${version}-${os}-arm64.tar.gz"
  }

  linux {
    source = "https://nodejs.org/dist/v${version}/node-v${version}-${os}-${arch}.tar.gz"
  }

  auto-version {
    github-release = "nodejs/node"
  }
}

channel "lts" {
  version = "16.*"
  update = "168h"
}

channel "current" {
  version = "18.*"
  update = "24h"
}