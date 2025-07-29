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
	"os"
	"os/exec"
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
		// Check if library already exists
		libPath := filepath.Join("lib", getLibraryName())
		if _, err := os.Stat(libPath); err == nil {
			return // Library exists
		}

		// Try to download the library
		fmt.Println("Downloading native library...")
		cmd := exec.Command("go", "run", "github.com/canyon-project/rust-istanbul-sourcemap/download_lib.go")
		if err := cmd.Run(); err != nil {
			initErr = fmt.Errorf("failed to download native library: %w", err)
			return
		}
		fmt.Println("Native library downloaded successfully")
	})
	return initErr
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
