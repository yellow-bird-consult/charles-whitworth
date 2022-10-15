#!/usr/bin/env bash

# navigate to directory
SCRIPTPATH="$( cd "$(dirname "$0")" ; pwd -P )"
cd $SCRIPTPATH

cd ..

# clean up service builds
sh ~/yb_tools/services.sh wipe

# build what's needed for local testing
sh ~/yb_tools/services.sh service add librarian development
