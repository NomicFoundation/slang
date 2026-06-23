description = "Static checker for GitHub Actions workflow files"
test = "actionlint --version"
binaries = ["actionlint"]

version "1.7.12" {
  source = "https://github.com/rhysd/actionlint/releases/download/v${version}/actionlint_${version}_${os}_${arch}.tar.gz"

  auto-version {
    github-release = "rhysd/actionlint/"
  }
}

sha256sums = {
  "https://github.com/rhysd/actionlint/releases/download/v1.7.12/actionlint_1.7.12_linux_arm64.tar.gz": "325e971b6ba9bfa504672e29be93c24981eeb1c07576d730e9f7c8805afff0c6",
  "https://github.com/rhysd/actionlint/releases/download/v1.7.12/actionlint_1.7.12_linux_amd64.tar.gz": "8aca8db96f1b94770f1b0d72b6dddcb1ebb8123cb3712530b08cc387b349a3d8",
  "https://github.com/rhysd/actionlint/releases/download/v1.7.12/actionlint_1.7.12_darwin_amd64.tar.gz": "5b44c3bc2255115c9b69e30efc0fecdf498fdb63c5d58e17084fd5f16324c644",
  "https://github.com/rhysd/actionlint/releases/download/v1.7.12/actionlint_1.7.12_darwin_arm64.tar.gz": "aba9ced2dee8d27fecca3dc7feb1a7f9a52caefa1eb46f3271ea66b6e0e6953f",
}
