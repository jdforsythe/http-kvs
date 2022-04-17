#!/bin/sh

KEY=$1
VAL=$2

if [[ -z $KEY || -z $VAL ]]; then
  echo "Usage:"
  echo ""
  echo "./set.sh [KEY] [VALUE]"
  exit 1
fi

curl --location --request POST "localhost:8000/set/${KEY}" --data-raw "${VAL}"
