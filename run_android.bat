@echo off

title Android Runner
echo Compiling and Running Project...

cargo apk run

if %errorlevel% neq 0 exit /b %errorlevel%

@REM rem Clear the screen once compilation is done
cls

timeout 2 > nul

echo Opening Logcat...

rem Get the PID
FOR /F %%i IN ('adb shell pidof rust.mediacodec') DO set pid=%%i

echo PID: %pid%

adb logcat --pid=%pid%

@REM adb 
