# export PLAYONLINUX=/usr/share/playonlinux4
# source "$PLAYONLINUX/lib/sources"
export WINEPREFIX="/home/david/.PlayOnLinux//wineprefix/Anno1602KE"
# export WINEDEBUG="-all"
cd "/home/david/.PlayOnLinux//wineprefix/Anno1602KE/drive_c/./Program Files/Anno 1602 Königs-Edition"
wine 1602.exe "$@"
