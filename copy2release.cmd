@echo off

set "to=target\release"
mkdir %to%\config
mkdir %to%\drv
copy /y config\* %to%\config\*
copy /y exe\* %to%\

pause