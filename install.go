//go:build ignore

package main

import (
	"fmt"
	"io"
	"net/http"
	"os"
	"path/filepath"
	"runtime"
)

func main() {
	fmt.Println("🔧 Installing Istanbul Sourcemap native library...")
	
	if err := installLibrary(); err != nil {
		fmt.Fprintf(os.Stderr, "❌ Installation failed: %v\n", err)
		os.Exit(1)
	}
	
	fmt.Println("✅ Installation completed successfully!")
	fmt.Println("")
	fmt.Println("You can now use the library in your Go code:")
	fmt.Println(`import istanbul "github.com/canyon-project/rust-istanbul-sourcemap"`)
}

func installLibrary() error {
	// 创建lib目录
	libDir := "lib"
	if err := os.MkdirAll(libDir, 0755); err != nil {
		return fmt.Errorf("failed to create lib directory: %w", err)
	}

	// 确定平台特定的库名称
	var downloadName, localName string
	switch runtime.GOOS {
	case "linux":
		localName = "libistanbul_sourcemap.so"
		switch runtime.GOARCH {
		case "amd64":
			downloadName = "libistanbul_sourcemap-linux-amd64.so"
		case "arm64":
			downloadName = "libistanbul_sourcemap-linux-arm64.so"
		default:
			return fmt.Errorf("unsupported Linux architecture: %s", runtime.GOARCH)
		}
	case "darwin":
		localName = "libistanbul_sourcemap.so"
		switch runtime.GOARCH {
		case "amd64":
			downloadName = "libistanbul_sourcemap-darwin-amd64.dylib"
		case "arm64":
			downloadName = "libistanbul_sourcemap-darwin-arm64.dylib"
		default:
			return fmt.Errorf("unsupported macOS architecture: %s", runtime.GOARCH)
		}
	case "windows":
		localName = "istanbul_sourcemap.dll"
		if runtime.GOARCH == "amd64" {
			downloadName = "istanbul_sourcemap-windows-amd64.dll"
		} else {
			return fmt.Errorf("unsupported Windows architecture: %s", runtime.GOARCH)
		}
	default:
		return fmt.Errorf("unsupported operating system: %s", runtime.GOOS)
	}

	localPath := filepath.Join(libDir, localName)

	// 检查是否已经存在
	if _, err := os.Stat(localPath); err == nil {
		fmt.Printf("📚 Library already exists at %s\n", localPath)
		return nil
	}

	// 下载URL
	downloadURL := fmt.Sprintf("https://github.com/canyon-project/rust-istanbul-sourcemap/releases/latest/download/%s", downloadName)
	
	fmt.Printf("⬇️  Downloading %s...\n", downloadURL)

	// 下载文件
	resp, err := http.Get(downloadURL)
	if err != nil {
		return fmt.Errorf("failed to download library: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		return fmt.Errorf("failed to download library: HTTP %d", resp.StatusCode)
	}

	// 创建本地文件
	out, err := os.Create(localPath)
	if err != nil {
		return fmt.Errorf("failed to create local file: %w", err)
	}
	defer out.Close()

	// 复制内容
	_, err = io.Copy(out, resp.Body)
	if err != nil {
		return fmt.Errorf("failed to save library: %w", err)
	}

	// 在Unix系统上设置执行权限
	if runtime.GOOS != "windows" {
		if err := os.Chmod(localPath, 0755); err != nil {
			return fmt.Errorf("failed to make library executable: %w", err)
		}
	}

	fmt.Printf("📦 Successfully downloaded library to %s\n", localPath)
	return nil
}