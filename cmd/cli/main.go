package main

import (
	"flag"
	"fmt"
	"io"
	"os"

	istanbul "github.com/canyon-project/rust-istanbul-sourcemap"
)

var (
	version = flag.Bool("version", false, "Show version information")
	input   = flag.String("input", "", "Input file path (default: stdin)")
	output  = flag.String("output", "", "Output file path (default: stdout)")
	help    = flag.Bool("help", false, "Show help information")
)

func main() {
	flag.Parse()

	if *help {
		showHelp()
		return
	}

	if *version {
		showVersion()
		return
	}

	if err := run(); err != nil {
		fmt.Fprintf(os.Stderr, "Error: %v\n", err)
		os.Exit(1)
	}
}

func showHelp() {
	fmt.Printf(`Istanbul Sourcemap Transformer

USAGE:
    istanbul-sourcemap-go [OPTIONS]

OPTIONS:
    -input <file>     Input file path (default: stdin)
    -output <file>    Output file path (default: stdout)
    -version          Show version information
    -help             Show this help message

EXAMPLES:
    # Transform from stdin to stdout
    cat coverage.json | istanbul-sourcemap-go

    # Transform from file to file
    istanbul-sourcemap-go -input coverage.json -output transformed.json

    # Show version
    istanbul-sourcemap-go -version
`)
}

func showVersion() {
	ism := istanbul.New()
	fmt.Printf("Istanbul Sourcemap Go CLI\n")
	fmt.Printf("Library Version: %s\n", ism.GetVersion())
	fmt.Printf("Platform: %s\n", ism.GetPlatform())
}

func run() error {
	// Read input
	var inputData []byte
	var err error

	if *input == "" {
		inputData, err = io.ReadAll(os.Stdin)
	} else {
		inputData, err = os.ReadFile(*input)
	}
	if err != nil {
		return fmt.Errorf("failed to read input: %w", err)
	}

	// Transform coverage data
	ism := istanbul.New()
	result, err := ism.TransformCoverage(string(inputData))
	if err != nil {
		return fmt.Errorf("failed to transform coverage: %w", err)
	}

	// Write output
	if *output == "" {
		fmt.Print(result)
	} else {
		err = os.WriteFile(*output, []byte(result), 0644)
		if err != nil {
			return fmt.Errorf("failed to write output: %w", err)
		}
	}

	return nil
}