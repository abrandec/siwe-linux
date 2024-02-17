#!/bin/sh

# Check if at least one argument is provided
if [ "$#" -lt 1 ]; then
  echo "Usage: $0 <command>"
  exit 1
fi

# Join all arguments into a single string to form the command
COMMAND="$*"

# Run the workflow with the provided command
act --var COMMAND=$COMMAND -P ubuntu-20.04=abrandec/siwe-dev:act-20.04 -W .github/workflows/local_build.yml