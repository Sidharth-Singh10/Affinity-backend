#!/bin/bash

# Check if a file name is provided as an argument
if [ -z "$1" ]; then
  echo "Usage: $0 <filename>"
  exit 1
fi

# Check if the file exists
if [ ! -f "$1" ]; then
  echo "File not found!"
  exit 1
fi

# Read the content of the file
file_content=$(cat "$1")

# Escape double quotes in the content
escaped_content=$(echo "$file_content" | sed 's/"/\\"/g')

# Create the JSON payload
json_payload=$(cat <<EOF
{
  "language": "python",
  "stdin" :"5",
  "files": [{
    "name": "main.py",
    "content": $file_content 
  }]
}
EOF
)

# Run the Docker command
echo "$json_payload" | docker run --rm -i --read-only --tmpfs /tmp:rw,noexec,nosuid,size=65536k --tmpfs /home/glot:rw,exec,nosuid,uid=1000,gid=1000,size=131072k -u glot -w /home/glot glot/python:latest
