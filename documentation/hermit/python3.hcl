description = "Python is a programming language that lets you work quickly and integrate systems more effectively."
strip = 1
binaries = ["install/bin/pip3", "install/bin/python3"]
test = "python3 -m pip install flake8"

env = {
  PYTHONPYCACHEPREFIX: "${HERMIT_ENV}/.hermit/python/cache",
  PYTHONUSERBASE: "${HERMIT_ENV}/.hermit/python",
  PATH: "${HERMIT_ENV}/.hermit/python/bin:${PATH}",
}

platform darwin {
  source = "https://github.com/indygreg/python-build-standalone/releases/download/${release_date}/cpython-${version}+${release_date}-${xarch}-apple-darwin-pgo+lto-full.tar.zst"
}

platform linux arm64 {
  source = "https://github.com/indygreg/python-build-standalone/releases/download/${release_date}/cpython-${version}+${release_date}-${xarch}-unknown-linux-gnu-lto-full.tar.zst"
}

platform linux amd64 {
  source = "https://github.com/indygreg/python-build-standalone/releases/download/${release_date}/cpython-${version}+${release_date}-${xarch}-unknown-linux-gnu-pgo+lto-full.tar.zst"
}

// Older releases had a slightly different URL template and no arm64 builds on Mac.
version "3.8.10" "3.9.5" {
  platform darwin {
    source = "https://github.com/indygreg/python-build-standalone/releases/download/20210506/cpython-${version}-x86_64-apple-darwin-pgo+lto-20210506T0943.tar.zst"
  }

  platform linux {
    source = "https://github.com/indygreg/python-build-standalone/releases/download/20210506/cpython-${version}-${xarch}-unknown-linux-gnu-pgo+lto-20210506T0943.tar.zst"
  }

}

version "3.8.12" "3.9.10" "3.10.2" {
  vars = {
    release_date: "20220227"
  }
}

version "3.8.13" "3.9.11" "3.10.3" {
  vars = {
    release_date: "20220318",
  }
}

version "3.10.4" {
  vars = {
    release_date: "20220502",
  }
}