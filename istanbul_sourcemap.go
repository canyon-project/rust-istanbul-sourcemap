package istanbul_sourcemap

/*
#cgo LDFLAGS: -L${SRCDIR}/lib -L${SRCDIR}/target/release -listanbul_sourcemap
#cgo linux LDFLAGS: -ldl
#cgo darwin LDFLAGS: -ldl
#include <stdlib.h>

// Function declarations
char* transform_coverage_ffi(const char* input);
void free_string(char* ptr);
const char* get_version();
*/
import "C"
import (
	"fmt"
	"io"
	"net/http"
	"os"
	"path/filepath"
	"runtime"
	"sync"
	"unsafe"
)

var (
	initOnce sync.Once
	initErr  error
)

// IstanbulSourceMap provides methods to transform Istanbul coverage data
type IstanbulSourceMap struct{}

// ensureLibrary ensures the native library is available
func ensureLibrary() error {
	initOnce.Do(func() {
		// Check if library already exists in multiple possible locations
		possiblePaths := []string{
			filepath.Join("lib", getLibraryName()),
			filepath.Join("target", "release", getLibraryName()),
			getLibraryName(), // Current directory
		}
		
		for _, libPath := range possiblePaths {
			if _, err := os.Stat(libPath); err == nil {
				return // Library exists
			}
		}

		// Try to download the library
		fmt.Println("Native library not found, attempting to download...")
		if err := downloadLibrary(); err != nil {
			initErr = fmt.Errorf("failed to download native library: %w", err)
			return
		}
		fmt.Println("Native library downloaded successfully")
	})
	return initErr
}

// downloadLibrary downloads the native library from GitHub releases
func downloadLibrary() error {
	// Create lib directory
	libDir := "lib"
	if err := os.MkdirAll(libDir, 0755); err != nil {
		return fmt.Errorf("failed to create lib directory: %w", err)
	}

	// Determine platform-specific library name for download
	var downloadName string
	switch runtime.GOOS {
	case "linux":
		switch runtime.GOARCH {
		case "amd64":
			downloadName = "libistanbul_sourcemap-linux-amd64.so"
		case "arm64":
			downloadName = "libistanbul_sourcemap-linux-arm64.so"
		default:
			return fmt.Errorf("unsupported architecture: %s", runtime.GOARCH)
		}
	case "darwin":
		switch runtime.GOARCH {
		case "amd64":
			downloadName = "libistanbul_sourcemap-darwin-amd64.dylib"
		case "arm64":
			downloadName = "libistanbul_sourcemap-darwin-arm64.dylib"
		default:
			return fmt.Errorf("unsupported architecture: %s", runtime.GOARCH)
		}
	case "windows":
		if runtime.GOARCH == "amd64" {
			downloadName = "istanbul_sourcemap-windows-amd64.dll"
		} else {
			return fmt.Errorf("unsupported architecture: %s", runtime.GOARCH)
		}
	default:
		return fmt.Errorf("unsupported operating system: %s", runtime.GOOS)
	}

	// Download URL
	downloadURL := fmt.Sprintf("https://github.com/canyon-project/rust-istanbul-sourcemap/releases/latest/download/%s", downloadName)
	localPath := filepath.Join(libDir, getLibraryName())

	fmt.Printf("Downloading %s...\n", downloadURL)

	// Try to download
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

	return nil
}

// getLibraryName returns the platform-specific library name
func getLibraryName() string {
	switch runtime.GOOS {
	case "linux", "darwin":
		return "libistanbul_sourcemap.so"
	case "windows":
		return "istanbul_sourcemap.dll"
	default:
		return "libistanbul_sourcemap.so"
	}
}

// New creates a new IstanbulSourceMap instance
func New() *IstanbulSourceMap {
	if err := ensureLibrary(); err != nil {
		fmt.Printf("Warning: %v\n", err)
	}
	return &IstanbulSourceMap{}
}

// TransformCoverage transforms Istanbul coverage data using the Rust library
func (ism *IstanbulSourceMap) TransformCoverage(input string) (string, error) {
	cInput := C.CString(input)
	defer C.free(unsafe.Pointer(cInput))

	cResult := C.transform_coverage_ffi(cInput)
	if cResult == nil {
		return "", fmt.Errorf("failed to transform coverage data")
	}
	defer C.free_string(cResult)

	result := C.GoString(cResult)
	return result, nil
}

// GetVersion returns the library version
func (ism *IstanbulSourceMap) GetVersion() string {
	cVersion := C.get_version()
	return C.GoString(cVersion)
}

// GetPlatform returns the current platform information
func (ism *IstanbulSourceMap) GetPlatform() string {
	return fmt.Sprintf("%s/%s", runtime.GOOS, runtime.GOARCH)
}

// Package-level convenience functions

// TransformCoverage is a convenience function that creates a new instance and transforms coverage data
func TransformCoverage(input string) (string, error) {
	ism := New()
	return ism.TransformCoverage(input)
}

// GetVersion is a convenience function that returns the library version
func GetVersion() string {
	ism := New()
	return ism.GetVersion()
}
