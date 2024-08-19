#!/bin/bash

# Check if the correct number of arguments is provided
if [ "$#" -ne 2 ]; then
  echo "Usage: $0 <python-file> <output-file>"
  exit 1
fi

# Input Python file and output JSON file
python_file="$1"
output_file="$2"

# Check if the Python file exists
if [ ! -f "$python_file" ]; then
  echo "File not found: $python_file"
  exit 1
fi

# Read the content of the Python file
python_code=$(<"$python_file")

# Escape special characters for JSON
escaped_code=$(printf '%s' "$python_code" | jq -Rs .)

# Create the JSON output
json_output=$(printf '%s' "$escaped_code")

# Write the JSON output to the specified file
echo "$json_output" > "$output_file"

# Inform the user
echo "JSON output written to $output_file"
