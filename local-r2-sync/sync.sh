#!/usr/bin/env bash
SCRIPT_DIR=$(realpath "$(dirname "${BASH_SOURCE[0]}")")

if [ $# -ne 2 ]; then
  echo "Usage: $0 <directory> <base URL>"
  exit 1
fi

# Check if the directory argument is provided
if [ -z "$1" ]; then
  echo "Please provide a directory path as the first argument"
  exit 1
fi

# Check if the directory exists
if [ ! -d "$1" ]; then
  echo "The specified directory does not exist"
  exit 1
fi

# Check if the base URL argument is provided
if [ -z "$2" ]; then
  echo "Please provide a base HTTP URL as the second argument"
  exit 1
fi

REPO_BASE=$(realpath "$1")
BASE_URL=$(echo "$2" | sed 's#/$##')

cd "$SCRIPT_DIR" || exit 1

wrangler dev --persist-to=./bucket &

while true; do
  if curl --output /dev/null --silent "$BASE_URL"; then
    echo "Server is up!"
    break
  else
    echo "Server is down, retrying in 1 seconds..."
    sleep 1
  fi
done

# Use find to upload each file to the specified URL
find "$REPO_BASE" -type f | while read file; do
  URL="$BASE_URL/${file#$REPO_BASE/}"
  CONTENT_TYPE="application/octet-stream"

  curl --silent -X PUT -H "Content-Type: $CONTENT_TYPE" --data-binary "@$file" "$URL"
  echo
done

kill %1
wait %1