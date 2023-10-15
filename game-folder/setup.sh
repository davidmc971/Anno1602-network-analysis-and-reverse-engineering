#!/bin/sh

export WINEARCH=win32

mkdir -p ./client_install

WINEPREFIX=$(pwd)/client_install winetricks winxp sound=pulse
WINEPREFIX=$(pwd)/client_install winetricks directplay
WINEPREFIX=$(pwd)/client_install wine $1

# cp -r ./client_install ./host_install
