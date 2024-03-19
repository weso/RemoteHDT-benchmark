@echo off

rem Set the path to the execute_benchmark.sh script
set "EXECUTE_BENCHMARK_SCRIPT=./execute_benchmark.bat"
rem Set the path to the benchmarks directory
set "BENCHMARKS_DIR=./benchmarks"

rem Check if the script and directory exist
if exist "%EXECUTE_BENCHMARK_SCRIPT%" if exist "%BENCHMARKS_DIR%" (
    rem Enter the benchmarks directory
    pushd "%BENCHMARKS_DIR%" || exit /b

    rem Execute the execute_benchmark.sh script with the parameter
    call "%EXECUTE_BENCHMARK_SCRIPT%" "%~1" "%~2"

    rem Return to the original directory
    popd || exit /b
) else (
    echo Error: %EXECUTE_BENCHMARK_SCRIPT% not found, or %BENCHMARKS_DIR% does not exist.
)
