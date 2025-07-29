class IstanbulSourcemapGo < Formula
  desc "High-performance Istanbul coverage source map transformer"
  homepage "https://github.com/canyon-project/rust-istanbul-sourcemap"
  version "0.1.0"
  
  if OS.mac?
    if Hardware::CPU.arm?
      url "https://github.com/canyon-project/rust-istanbul-sourcemap/releases/download/v#{version}/istanbul-sourcemap-go_#{version}_darwin_arm64.tar.gz"
      sha256 "YOUR_ARM64_SHA256_HERE"
    else
      url "https://github.com/canyon-project/rust-istanbul-sourcemap/releases/download/v#{version}/istanbul-sourcemap-go_#{version}_darwin_amd64.tar.gz"
      sha256 "YOUR_AMD64_SHA256_HERE"
    end
  elsif OS.linux?
    if Hardware::CPU.arm?
      url "https://github.com/canyon-project/rust-istanbul-sourcemap/releases/download/v#{version}/istanbul-sourcemap-go_#{version}_linux_arm64.tar.gz"
      sha256 "YOUR_LINUX_ARM64_SHA256_HERE"
    else
      url "https://github.com/canyon-project/rust-istanbul-sourcemap/releases/download/v#{version}/istanbul-sourcemap-go_#{version}_linux_amd64.tar.gz"
      sha256 "YOUR_LINUX_AMD64_SHA256_HERE"
    end
  end

  def install
    bin.install "istanbul-sourcemap-go"
  end

  test do
    system "#{bin}/istanbul-sourcemap-go", "-version"
  end
end