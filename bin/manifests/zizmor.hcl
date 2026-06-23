description = "Static analysis for GitHub Actions, finding security issues in workflows and actions."
test = "zizmor --version"
binaries = ["zizmor"]

version "1.26.1" {
  platform "linux" {
    source = "https://github.com/zizmorcore/zizmor/releases/download/v${version}/zizmor-${xarch}-unknown-linux-gnu.tar.gz"
  }

  platform "darwin" {
    source = "https://github.com/zizmorcore/zizmor/releases/download/v${version}/zizmor-${xarch}-apple-darwin.tar.gz"
  }

  auto-version {
    github-release = "zizmorcore/zizmor"
  }
}

sha256sums = {
  "https://github.com/zizmorcore/zizmor/releases/download/v1.26.1/zizmor-x86_64-unknown-linux-gnu.tar.gz": "8556289a64e7aaf2400cd516f61a471aa91c5902cc56ad96a82fd12f90c2ef73",
  "https://github.com/zizmorcore/zizmor/releases/download/v1.26.1/zizmor-x86_64-apple-darwin.tar.gz": "2967414a561f8c1264121e8f723c3b5abcf3d1bf7ce5063114df99985dd75801",
  "https://github.com/zizmorcore/zizmor/releases/download/v1.26.1/zizmor-aarch64-apple-darwin.tar.gz": "68ab2b37836bbd44f6cfffcc102b9ffffbc20c5d67d84293dafb63bd2775a1da",
  "https://github.com/zizmorcore/zizmor/releases/download/v1.26.1/zizmor-aarch64-unknown-linux-gnu.tar.gz": "711f5af366b299128f9a04b1470e37d990b41fbd21f14a1a4148d25004a83762",
}
