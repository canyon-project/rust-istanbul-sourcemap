//go:build ignore

package main

import (
	"archive/tar"
	"compress/gzip"
	"fmt"
	"io"
	"net/http"
	"os"
	"path/filepath"
	"runtime"
	"strings"
)

const (
	githubRepo    = "canyon-project/rust-istanbul-sourcemap"
	latestRelease = "https://api.github.com/repos/" + githubRepo + "/releases/latest"
)

func main() {
	if err := downloadLibrary(); err != nil {
		fmt.Fprintf(os.Stderr, "Error downloading library: %v\n", err)
		os.Exit(1)
	}
}

func downloadLibrary() error {
	// Create lib directory
	libDir := "lib"
	if err := os.MkdirAll(libDir, 0755); err != nil {
		return fmt.Errorf("failed to create lib directory: %w", err)
	}

	// Determine platform-specific library name
	var libName string
	switch runtime.GOOS {
	case "linux":
		switch runtime.GOARCH {
		case "amd64":
			libName = "libistanbul_sourcemap-linux-amd64.so"
		case "arm64":
			libName = "libistanbul_sourcemap-linux-arm64.so"
		default:
			return fmt.Errorf("unsupported architecture: %s", runtime.GOARCH)
		}
	case "darwin":
		switch runtime.GOARCH {
		case "amd64":
			libName = "libistanbul_sourcemap-darwin-amd64.dylib"
		case "arm64":
			libName = "libistanbul_sourcemap-darwin-arm64.dylib"
		default:
			return fmt.Errorf("unsupported architecture: %s", runtime.GOARCH)
		}
	case "windows":
		if runtime.GOARCH == "amd64" {
			libName = "istanbul_sourcemap-windows-amd64.dll"
		} else {
			return fmt.Errorf("unsupported architecture: %s", runtime.GOARCH)
		}
	default:
		return fmt.Errorf("unsupported operating system: %s", runtime.GOOS)
	}

	// Download URL (you'll need to update this with your actual GitHub repo)
	downloadURL := fmt.Sprintf("https://github.com/%s/releases/latest/download/%s", githubRepo, libName)
	
	// Determine local library name
	var localLibName string
	switch runtime.GOOS {
	case "linux", "darwin":
		localLibName = "libistanbul_sourcemap.so"
	case "windows":
		localLibName = "istanbul_sourcemap.dll"
	}

	localPath := filepath.Join(libDir, localLibName)

	// Check if library already exists
	if _, err := os.Stat(localPath); err == nil {
		fmt.Printf("Library already exists at %s\n", localPath)
		return nil
	}

	fmt.Printf("Downloading %s to %s...\n", libName, localPath)

	// Download the file
	resp, err := http.Get(downloadURL)
	if err != nil {
		return fmt.Errorf("failed to download library: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		return fmt.Errorf("failed to download library: HTTP %d", resp.StatusCode)
	}

	// Create the local file
	out, err := os.Create(localPath)
	if err != nil {
		return fmt.Errorf("failed to create local file: %w", err)
	}
	defer out.Close()

	// Copy the downloaded content
	_, err = io.Copy(out, resp.Body)
	if err != nil {
		return fmt.Errorf("failed to save library: %w", err)
	}

	// Make executable on Unix systems
	if runtime.GOOS != "windows" {
		if err := os.Chmod(localPath, 0755); err != nil {
			return fmt.Errorf("failed to make library executable: %w", err)
		}
	}

	fmt.Printf("Successfully downloaded library to %s\n", localPath)
	return nil
}