@echo off

rem Assign arguments to variables
set command_name=%1
set arg1=%2
set arg2=%3
set arg3=%4

rem Run the cargo command
cargo run --release %command_name% %arg1% %arg2% %arg3%
