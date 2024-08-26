#!/bin/bash
# replace.sh

# Define a list of environment variables to check and replace
VARIABLES=("API_BASE_URL")

# Check if each variable is set
for VAR in "${VARIABLES[@]}"; do
    if [ -z "${!VAR}" ]; then
        echo "$VAR is not set. Please set it and rerun the script."
        exit 1
    fi
done

# Find and replace BAKED values with real values
find .next -type f -name "*.js" |
while read file; do
    for VAR in "${VARIABLES[@]}"; do
        sed -i "s|BAKED_$VAR|${!VAR}|g" "$file"
    done
done
