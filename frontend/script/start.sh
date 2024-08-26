#!/bin/sh
# start.sh

# Replace runtime env vars and start next server
bash script/replace.sh &&
node server.js