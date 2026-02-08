# Homebrew formula for datacell
# Documentation: https://docs.brew.sh/Formula-Cookbook
#                https://rubydoc.brew.sh/Formula

class Datacell < Formula
  desc "CLI tool for spreadsheet manipulation with pandas-style operations"
  homepage "https://github.com/yingkitw/datacell"

  # For stable releases from git tags (uncomment when you have releases)
  # url "https://github.com/yingkitw/datacell/archive/refs/tags/v0.1.5.tar.gz"
  # sha256 ""  # Run `brew install --build-from-source` and copy the checksum

  # Install from git (development version)
  url "https://github.com/yingkitw/datacell.git",
    branch: "main",
    using: :git

  head "https://github.com/yingkitw/datacell.git",
    branch: "main"

  depends_on "rust" => :build

  def install
    system "cargo", "install", "--locked", "--root", prefix, "--path", "."
  end

  test do
    system "#{bin}/datacell", "--version"
    assert_match "datacell", shell_output("#{bin}/datacell --help")
  end
end
