@echo off

rem Specify the paths to the folders you want to delete
set "RESULTS_FOLDER=./results"
set "ZARR_FILES_FOLDER=./zarr-files"
set "NT_FILES=./nt-files"

rem Delete the results folder and its contents
rmdir /s /q "%RESULTS_FOLDER%"

rem Delete the zarr-files folder and its contents
rmdir /s /q "%ZARR_FILES_FOLDER%"

rem Delete the nt-files folder and its contents
rmdir /s /q "%NT_FILES%"
