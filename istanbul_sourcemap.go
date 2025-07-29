package istanbul_sourcemap

/*
#cgo LDFLAGS: -L${SRCDIR}/lib -L${SRCDIR}/target/release -listanbul_sourcemap
#cgo linux LDFLAGS: -ldl
#cgo darwin LDFLAGS: -ldl
#cgo windows LDFLAGS: -lstdc++
#include <stdlib.h>

// Function declarations
char* transform_coverage_ffi(const char* input);
void free_string(char* ptr);
const char* get_version();
*/
import "C"
import (
	"fmt"
	"runtime"
	"unsafe"
)

// IstanbulSourceMap provides methods to transform Istanbul coverage data
type IstanbulSourceMap struct{}

// New creates a new IstanbulSourceMap instance
func New() *IstanbulSourceMap {
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
