@echo off

mkdir publish
mkdir publish\config
mkdir publish\drv
copy /y target\release\signingcheck.exe publish\
copy /y config\* publish\config\*
copy /y exe\* publish\
copy /y how-to-use.txt publish\
copy /y releasenote.txt publish\

pause