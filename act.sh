#!/bin/sh

 # Load the .env file
source .env
echo $S3_BUCKET
# Check if at least one argument is provided
if [ "$#" -lt 1 ]; then
  echo "Usage: $0 <command>"
  exit 1
fi

# Join all arguments into a single string to form the command
COMMAND="$*"

# Run the workflow with the provided command
act --var COMMAND=$COMMAND --var S3_BUCKET=$S3_BUCKET --var AWS_REGION=$AWS_REGION -s AWS_ACCESS_KEY_ID=$AWS_ACCESS_KEY_ID -s AWS_SECRET_ACCESS_KEY=$AWS_SECRET_ACCESS_KEY -P ubuntu-20.04=abrandec/siwe-dev:act-20.04 -W .github/workflows/local_build.yml