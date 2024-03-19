@echo off

rem Call reset.bat 
call reset.bat

rem Call generate_RDF.bat with appropriate arguments
call generate_RDF.bat moreNodes-n1000-p10-t1000000 1000 10 1000000

rem Call execute_benchmark.bat with appropriate arguments
call execute_benchmark.bat 5 local
