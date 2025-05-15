#!/bin/bash

# Script to automatically fix common markdown linting issues
# Usage: ./scripts/fix_markdown.sh <file_path>

set -e

if [ -z "$1" ]; then
  echo "Usage: $0 <file_path>"
  exit 1
fi

FILE="$1"

if [ ! -f "$FILE" ]; then
  echo "Error: File not found: $FILE"
  exit 1
fi

echo "Processing file: $FILE"

# Create a temporary file
TEMP_FILE=$(mktemp)

# Function to check if we're inside a code block
in_code_block=0
function is_code_block() {
  local line="$1"
  if [[ "$line" =~ ^(\s*\`\`\`|\s*~~~) ]]; then
    if [ $in_code_block -eq 0 ]; then
      in_code_block=1
    else
      in_code_block=0
    fi
  fi
}

# Function to fix ordered list numbering
current_list_level=0
list_counter=0
function fix_ordered_list_numbering() {
  local line="$1"
  # Check if this is an ordered list item
  if [[ "$line" =~ ^(\s*)([0-9]+)(\.\s.*)$ ]]; then
    local indent="${BASH_REMATCH[1]}"
    local number="${BASH_REMATCH[2]}"
    local content="${BASH_REMATCH[3]}"
    
    # Calculate indentation level (each level is typically 2 or 4 spaces)
    local indent_level=$((${#indent} / 2))
    
    # If we're at a new nesting level or first item
    if [ $indent_level -ne $current_list_level ]; then
      current_list_level=$indent_level
      list_counter=1
    fi
    
    # Replace the number with the correct sequence
    echo "${indent}${list_counter}${content}"
    list_counter=$((list_counter + 1))
  else
    # Reset counter if not in a list
    if ! [[ "$line" =~ ^(\s*)([-*+]|\d+\.)(\s.*)$ ]]; then
      current_list_level=0
      list_counter=0
    fi
    echo "$line"
  fi
}

# Process the file line by line
while IFS= read -r line || [ -n "$line" ]; do  # Handle last line without newline
  # Check if we're entering or leaving a code block
  is_code_block "$line"
  
  # Only process lines outside of code blocks and not starting with HTML-like tags
  if [ $in_code_block -eq 0 ] && ! [[ "$line" =~ ^\s*\<[a-zA-Z] ]]; then
    # Fix MD009: Trailing spaces - more aggressive approach
    line="${line%"${line##*[![:space:]]}"}"  # Remove all trailing whitespace
    
    # Fix MD034: Bare URLs - Wrap URLs in angle brackets
    line=$(echo "$line" | sed -E 's/(^|[^(<])((http|https|ftp):\/\/[^ )]*)([^)>]|$)/\1<\2>\4/g')
    
    # Fix MD029: Ordered list prefix
    line=$(fix_ordered_list_numbering "$line")
    
    # Fix MD033: Inline HTML - Replace common elements with markdown equivalents
    # Convert <strong> tags to ** markdown
    line=$(echo "$line" | sed -E 's/<strong>(.*)<\/strong>/\*\*\1\*\*/g')
    # Convert <N> to N (common in projections file)
    line=$(echo "$line" | sed -E 's/<([A-Za-z])>/\1/g')
    
    # Fix MD013: Long lines - wrap at 100 chars, preserve indentation
    if [ ${#line} -gt 100 ]; then
      # Get current indentation
      indent=$(echo "$line" | sed -E 's/^([[:space:]]*).*$/\1/')
      indent_length=${#indent}
      
      # Split long lines using fold, preserving indentation
      folded=$(echo "$line" | fold -s -w $((100 - indent_length)) | sed -e "2,\$s/^/$indent/")
      echo "$folded" >> "$TEMP_FILE"
    else
      echo "$line" >> "$TEMP_FILE"
    fi
  else
    # Inside code blocks, preserve content exactly
    echo "$line" >> "$TEMP_FILE"
  fi
done < "$FILE"

# Replace original file with fixed version
mv "$TEMP_FILE" "$FILE"

echo "Processing complete for $FILE"