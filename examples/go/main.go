package main

/*
#cgo LDFLAGS: -L. -listanbul_sourcemap
#include <stdlib.h>

// Function declarations
char* transform_coverage_ffi(const char* input);
void free_string(char* ptr);
const char* get_version();
*/
import "C"
import (
	"fmt"
	"unsafe"
)

// TransformCoverage transforms Istanbul coverage data using the Rust library
func TransformCoverage(input string) (string, error) {
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
func GetVersion() string {
	cVersion := C.get_version()
	return C.GoString(cVersion)
}

func main() {
	fmt.Printf("Istanbul Sourcemap Library Version: %s\n", GetVersion())

	// Example Istanbul coverage data
	coverageData := `{
		"path/to/file.js": {
			"path": "path/to/file.js",
			"statementMap": {
				"0": {"start": {"line": 1, "column": 0}, "end": {"line": 1, "column": 10}}
			},
			"fnMap": {
				"0": {
					"name": "testFunction",
					"decl": {"start": {"line": 1, "column": 0}, "end": {"line": 1, "column": 10}},
					"loc": {"start": {"line": 1, "column": 0}, "end": {"line": 3, "column": 1}}
				}
			},
			"branchMap": {},
			"s": {"0": 1},
			"f": {"0": 1},
			"b": {}
		}
	}`

	transformed, err := TransformCoverage(coverageData)
	if err != nil {
		fmt.Printf("Error: %v\n", err)
		return
	}

	fmt.Printf("Transformed coverage data:\n%s\n", transformed)
}
