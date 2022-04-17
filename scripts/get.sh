#!/bin/sh

KEY=$1

if [[ -z $KEY ]]; then
  echo "Usage:"
  echo ""
  echo "./get.sh [KEY]"
  exit 1
fi

echo "Getting localhost:8000/get/${KEY}"

curl "localhost:8000/get/${KEY}"
