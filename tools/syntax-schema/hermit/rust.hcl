description = "A language empowering everyone to build reliable and efficient software."
binaries = ["bin/*"]
strip = 2
env = {
  "CARGO_HOME": "${HERMIT_ENV}/.hermit/rust",
  "PATH": "${HERMIT_ENV}/.hermit/rust/bin:${PATH}",
}

darwin {
  source = "https://static.rust-lang.org/dist/rust-${version}-${xarch}-apple-darwin.tar.xz"
}

linux {
  source = "https://static.rust-lang.org/dist/rust-${version}-${xarch}-unknown-linux-gnu.tar.xz"
}

channel "nightly" {
  update = "24h"

  darwin {
    source = "https://static.rust-lang.org/dist/rust-nightly-${xarch}-apple-darwin.tar.xz"
  }

  linux {
    source = "https://static.rust-lang.org/dist/rust-nightly-${xarch}-unknown-linux-gnu.tar.xz"
  }
}

version "1.51.0" "1.52.1" "1.53.0" "1.54.0" "1.55.0" "1.56.0" "1.57.0" "1.58.0"
        "1.58.1" "1.59.0" "1.60.0" {
  auto-version {
    github-release = "rust-lang/rust"
  }
}
