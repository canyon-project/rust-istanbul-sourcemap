#!/bin/bash

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}Istanbul Sourcemap Go - Publishing Script${NC}"
echo "========================================"

# Check if version is provided
if [ -z "$1" ]; then
    echo -e "${RED}Error: Version not provided${NC}"
    echo "Usage: $0 <version>"
    echo "Example: $0 v0.1.0"
    exit 1
fi

VERSION=$1

# Validate version format
if [[ ! $VERSION =~ ^v[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    echo -e "${RED}Error: Invalid version format${NC}"
    echo "Version should be in format: v0.1.0"
    exit 1
fi

echo -e "${YELLOW}Publishing version: $VERSION${NC}"

# Check if we're on main branch
CURRENT_BRANCH=$(git branch --show-current)
if [ "$CURRENT_BRANCH" != "main" ] && [ "$CURRENT_BRANCH" != "master" ]; then
    echo -e "${RED}Error: Not on main/master branch${NC}"
    echo "Current branch: $CURRENT_BRANCH"
    exit 1
fi

# Check if working directory is clean
if [ -n "$(git status --porcelain)" ]; then
    echo -e "${RED}Error: Working directory is not clean${NC}"
    echo "Please commit or stash your changes first"
    exit 1
fi

# Update version in go.mod if needed
echo -e "${YELLOW}Updating go.mod...${NC}"
go mod tidy

# Run tests
echo -e "${YELLOW}Running tests...${NC}"
if [ -f "lib/libistanbul_sourcemap.so" ] || [ -f "lib/libistanbul_sourcemap.dylib" ] || [ -f "lib/istanbul_sourcemap.dll" ]; then
    go test -v
else
    echo -e "${YELLOW}Warning: Native library not found, skipping CGO tests${NC}"
fi

# Create and push tag
echo -e "${YELLOW}Creating and pushing tag...${NC}"
git tag -a "$VERSION" -m "Release $VERSION"
git push origin "$VERSION"

echo -e "${GREEN}Successfully published version $VERSION!${NC}"
echo ""
echo "Next steps:"
echo "1. GitHub Actions will automatically build and release the binaries"
echo "2. Users can install with: go get github.com/canyon-project/rust-istanbul-sourcemap@$VERSION"
echo "3. Monitor the GitHub Actions workflow for any issues"