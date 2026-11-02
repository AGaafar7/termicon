class Termicon < Formula
  desc "Blazing-fast Nerd Font and Unicode file icons for macOS shells"
  homepage "https://github.com/your-username/termicon"
  version "0.1.0"
  license "MIT"

  # We use the universal binary built for macOS
  if OS.mac?
    url "https://github.com/your-username/termicon/releases/download/v#{version}/termicon-macOS-universal.tar.gz"
    # Replace this with the real sha256 from: shasum -a 256 termicon-macOS-universal.tar.gz
    sha256 "PLACEHOLDER_SHA256"
  end

  def install
    bin.install "termicon"
  end

  test do
    assert_match "termicon v#{version}", shell_output("#{bin}/termicon --version")
  end
end
