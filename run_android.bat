@echo off

title Android Runner
echo Compiling and Running Project...

cargo apk run

@REM rem Clear the screen once compilation is done
cls

echo Opening Logcat...

rem Get the PID
FOR /F %%i IN ('adb shell pidof rust.mediacodec') DO set pid=%%i

echo PID: %pid%

adb logcat --pid=%pid%

@REM adb 
