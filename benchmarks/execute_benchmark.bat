@echo off

rem Check if Cargo is installed
where cargo >nul 2>&1
if %errorlevel% neq 0 (
    echo Error: Cargo is not installed. Please install Rust and Cargo before running this script.
    exit /b 1
)

rem Assign arguments to variables
set arg1=%1
set arg2=%2

rem Run the cargo command
cargo run --release -- -i %arg1% -f ..\zarr-files\*.zarr --context %arg2%
