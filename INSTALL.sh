#! /bin/bash
# Linux install script.
# --------------------
# Untested and unfinished.

if [ "$USER" != "root" ]
then
    echo "The bonfire install script needs to be run with root permissions."
    echo "If you do not trust this script for some reason, please follow our manual install guide."
    exit 2
fi

PROJECT_NAME = "ocp-bonfire"
INSTALL_VER = "1.0.0"

mkdir /var/$PROJECT_NAME/data/
mkdir /srv/$PROJECT_NAME/app/

# Creat ADMIN user and group
# Add permisisons with CHMOD
# Download and add the files where they need to go 
# Run to confim everything is working
