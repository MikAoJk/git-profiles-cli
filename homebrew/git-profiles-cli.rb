class GitProfilesCli < Formula
  desc "A CLI tool to switch between private and work user profiles"
  homepage "https://github.com/MikAoJk/git-profiles-cli"
  version "1.0.1"
  license "MIT"

  if Hardware::CPU.arm?
    url "https://github.com/MikAoJk/git-profiles-cli/releases/download/v1.0.1/git-profiles-cli-1.0.1-aarch64-apple-darwin.tar.gz"
    sha256 "REPLACE_WITH_ARM64_SHA256"
  else
    url "https://github.com/MikAoJk/git-profiles-cli/releases/download/v1.0.1/git-profiles-cli-1.0.1-x86_64-apple-darwin.tar.gz"
    sha256 "REPLACE_WITH_X86_64_SHA256"
  end

  def install
    bin.install "git-profiles-cli"
  end

  test do
    system "#{bin}/git-profiles-cli", "--help"
  end
end
