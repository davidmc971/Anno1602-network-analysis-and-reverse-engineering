#!/bin/sh

# on Arch, wine is now by default in WoW64 mode and 32-bit apps can be run in 64-bit mode
# export WINEARCH=win32

mkdir -p ./client_install

WINEPREFIX=$(pwd)/client_install winetricks winxp sound=pulse
WINEPREFIX=$(pwd)/client_install winetricks directplay
WINEPREFIX=$(pwd)/client_install wine $1

# cp -r ./client_install ./host_install
