#! /bin/bash
# Linux install script.
# --------------------
# Untested and unfinished.

PROJECT_NAME="dim-bonfire"
INSTALL_VER="1.0.0"

COLOR_RESET="\033[39;49m"
COLOR_RED="\033[31m"
COLOR_GREEN="\033[32m"
COLOR_GRAY="\033[90m"
COLOR_WHITE="\033[37m"

COLOR_BLUE="\033[94m"
COLOR_YELLOW="\e[93m"
# COLOR_RED="\e[31m"

STYLE_BOLD="\033[1m"
STYLE_RESET="\033[0m"

print_buffer="None"

PROJECT_NAME="bonfire"

# Ideal cases: 
# - CentOS
# - Debian
# - Fedora Server
# Container: Fedora CoreOS

function info {
    printf "$COLOR_BLUE$STYLE_BOLD     INFO$STYLE_RESET $o\n"
}

function warn {
    printf "$COLOR_YELLOW$STYLE_BOLD     WARN$STYLE_RESET $o\n"
}

function error {
    printf "$COLOR_RED$STYLE_BOLD    ERROR$STYLE_RESET $o\n"
}

function ask { 
    printf "$COLOR_GREEN$STYLE_BOLD      ASK$STYLE_RESET $o"
    read -p "" i
}

# $i = input buffer
# $o = output buffer

# if [ "$USER" != "root" ]
# then
#     o="The bonfire install script needs to be run with root permissions."
#     error
#     o="If you do not trust this script for some reason, "
#     info
#     o="please follow our manual install guide."
#     info
#     exit 2
# fi

DATA_LOCATION="/srv/$PROJECT_NAME/"
# we also write to /srv/https/bonfire/

o="Data will be stored in $COLOR_WHITE$DATA_LOCATION"
info
# ew
#read -p "Is this okay? $COLOR_GRAY($COLOR_GREEN Y $COLOR_GRAY / $COLOR_RED n $COLOR_GRAY): $COLOR_WHITE" confirm && [[ $confirm == [yY] || $confirm == [yY][eE][sS] ]] || exit 1
o="Is this okay? (Y/n): "
ask

if [ $i != "y" ] && [ $i != "Y" ]
then
    o="Exiting ..."
    info

    exit 1
fi

# if path_okay=false: 
#     read -p " Then what path would you like?: " DATA_LOCATION
# fi

o="Creating Directories"
info

mkdir /srv/$PROJECT_NAME/
mkdir /srv/$PROJECT_NAME/app/
mkdir /srv/$PROJECT_NAME/data/
mkdir /srv/$PROJECT_NAME/https/
mkdir /srv/$PROJECT_NAME/old/

o="Creating \"automated\" user group"
info

groupadd automated

o="Creating User"
info

useradd -g automated bonfire_io

o="Setting Permissions"
info

# Set the MINIMUM permissions these files/directories need to work for security

# r-x --- --- /srv/bonfire/MANAGE_INSTALL.sh (update, downdate, uhhh... ) 
# r-x r-x --- /srv/bonfire/STARTUP.sh

# Default permissions: 
# r-- r-- ---
chmod 440 /srv/$PROJECT_NAME/ 

# Backend processing 
# rwx r-x ---
chmod 750 /srv/$PROJECT_NAME/app/

# Database
# --- rwx ---
chmod 070 /srv/$PROJECT_NAME/data/

# What is served to the user directly (i.e. webapp)
# rw- r-- r--
chmod 644 /srv/$PROJECT_NAME/https/

# Old read-only versions of the software to easily revert. 
# r-- r-- ---
chmod 440 /srv/$PROJECT_NAME/old/

# mkdir $DATA_LOCATION/data/
# mkdir $DATA_LOCATION/app/

# chmod $DATA_LOCATION/data

# Creat ADMIN user and group
# Add permisisons with CHMOD
# Download and add the files where they need to go 
# Run to confim everything is working
