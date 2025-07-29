@echo off

echo Installing Istanbul Sourcemap Go library...

REM Download the native library
echo Downloading native library...
go run download_lib.go

if %ERRORLEVEL% neq 0 (
    echo Failed to download library
    exit /b 1
)

echo Installation completed successfully!
echo.
echo You can now use the library in your Go code:
echo.
echo import "github.com/canyon-project/rust-istanbul-sourcemap"
echo.
echo ism := istanbul_sourcemap.New()
echo result, err := ism.TransformCoverage(coverageData)