@echo off

rem Set the path to the generate_RDF.sh script
set "GENERATE_RDF_SCRIPT=./generate_RDF.bat"
rem Set the path to the generate directory
set "GENERATE_DIR=./generate"

rem Check if the script and directory exist
if exist "%GENERATE_RDF_SCRIPT%" if exist "%GENERATE_DIR%" (
    rem Enter the generate directory
    pushd "%GENERATE_DIR%" || exit /b

    rem Specify the four parameters you want to pass to generate_RDF.sh
    set "PARAM1=%1"
    set "PARAM2=%2"
    set "PARAM3=%3"
    set "PARAM4=%4"

    rem Execute the generate_RDF.sh script with parameters
    call "%GENERATE_RDF_SCRIPT%" "%PARAM1%" "%PARAM2%" "%PARAM3%" "%PARAM4%"

    rem Return to the original directory
    popd || exit /b
) else (
    echo Error: %GENERATE_RDF_SCRIPT% not found, or %GENERATE_DIR% does not exist.
)
