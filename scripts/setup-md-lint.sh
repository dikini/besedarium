#!/bin/bash

# Script to set up markdownlint-cli2 for the project
# Usage: ./scripts/setup-md-lint.sh

set -e

echo "Setting up markdownlint-cli2..."

# Check if npm is installed
if ! command -v npm &> /dev/null; then
  echo "Error: npm is not installed. Please install Node.js and npm first."
  exit 1
fi

# Install markdownlint-cli2 locally
echo "Installing markdownlint-cli2..."
npm install --no-save markdownlint-cli2

# Create config file if it doesn't exist
if [ ! -f .markdownlint-cli2.yaml ]; then
  echo "Creating default .markdownlint-cli2.yaml configuration..."
  cat > .markdownlint-cli2.yaml << 'EOL'
config:
  # First, set the default
  default: true

  # Per-rule settings in alphabetical order
  code-block-style:                 # MD046
    style: "fenced"
  # Temporarily disable MD014 until we fix problems
  commands-show-output: false       # MD014
  emphasis-style: false             # MD049
  header-style:                     # MD003
    style: "atx"
  hr-style:                         # MD035
    style: "---"
  line-length:                      # MD013
    code_blocks: false
    tables: false
    headings: true
    heading_line_length: 100
    line_length: 800
  no-duplicate-heading:             # MD024
    siblings_only: true
  no-emphasis-as-heading: false     # MD036
  no-inline-html: false             # MD033
  no-trailing-punctuation:          # MD026
    punctuation: ".,;:!。，；：！？"
  no-trailing-spaces: false         # MD009
  reference-links-images: false     # MD052
  ul-style:                         # MD004
    style: "dash"

  # Disable this rule in the CLI project, because 1. 1. 1. 1. numbering
  # is rendered verbatim in CLI output
  ol-prefix: false                  # MD029
EOL
else
  echo ".markdownlint-cli2.yaml already exists."
fi

# Make scripts executable
chmod +x ./scripts/md-lint.sh
chmod +x ./scripts/fix_markdown.sh

echo "Setup complete! You can now run markdown linting with:"
echo "  ./scripts/md-lint.sh"
echo "Or fix issues automatically with:"
echo "  ./scripts/md-lint.sh --fix"