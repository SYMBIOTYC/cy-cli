class Cy < Formula
  desc "CY-CLI: advanced CLI built on Codex CLI foundation"
  homepage "https://github.com/vladleopold/cy-cli"
  version "0.0.0"  # bumped by CI
  license "Apache-2.0"

  on_macos do
    if Hardware::CPU.intel?
      url "https://github.com/vladleopold/cy-cli/releases/download/v#{version}/cy-x86_64-apple-darwin.tar.gz"
      sha256 "<sha256-from-release>"
    else
      url "https://github.com/vladleopold/cy-cli/releases/download/v#{version}/cy-aarch64-apple-darwin.tar.gz"
      sha256 "<sha256-from-release>"
    end
  end

  on_linux do
    if Hardware::CPU.intel?
      url "https://github.com/vladleopold/cy-cli/releases/download/v#{version}/cy-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "<sha256-from-release>"
    else
      url "https://github.com/vladleopold/cy-cli/releases/download/v#{version}/cy-aarch64-unknown-linux-gnu.tar.gz"
      sha256 "<sha256-from-release>"
    end
  end

  def install
    bin.install "cy"
  end

  test do
    system "#{bin}/cy", "--version"
  end
end
