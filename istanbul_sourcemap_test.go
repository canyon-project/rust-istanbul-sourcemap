package istanbul_sourcemap

import (
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func TestNew(t *testing.T) {
	ism := New()
	assert.NotNil(t, ism)
}

func TestGetVersion(t *testing.T) {
	ism := New()
	version := ism.GetVersion()
	assert.NotEmpty(t, version)
	t.Logf("Library version: %s", version)
}

func TestGetPlatform(t *testing.T) {
	ism := New()
	platform := ism.GetPlatform()
	assert.NotEmpty(t, platform)
	t.Logf("Platform: %s", platform)
}

func TestTransformCoverage(t *testing.T) {
	ism := New()

	// Test with valid Istanbul coverage data
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

	result, err := ism.TransformCoverage(coverageData)
	require.NoError(t, err)
	assert.NotEmpty(t, result)
	t.Logf("Transformed result: %s", result)
}

func TestTransformCoverageInvalidInput(t *testing.T) {
	ism := New()

	// Test with invalid JSON
	result, err := ism.TransformCoverage("invalid json")
	assert.Error(t, err)
	assert.Empty(t, result)
}

func TestPackageLevelFunctions(t *testing.T) {
	// Test package-level convenience functions
	version := GetVersion()
	assert.NotEmpty(t, version)

	coverageData := `{
		"test.js": {
			"path": "test.js",
			"statementMap": {},
			"fnMap": {},
			"branchMap": {},
			"s": {},
			"f": {},
			"b": {}
		}
	}`

	result, err := TransformCoverage(coverageData)
	require.NoError(t, err)
	assert.NotEmpty(t, result)
}
