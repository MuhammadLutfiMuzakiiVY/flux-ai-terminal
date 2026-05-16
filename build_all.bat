@echo off
echo 🚀 FLUX AI TERMINAL - PROFESSIONAL BUILD SYSTEM
echo 🦀 Building Rust Core...
cd core
cargo build --release
if %errorlevel% neq 0 (
    echo ❌ Rust Build Failed!
    exit /b 1
)
cd ..

echo 🤖 Building Android App...
cd android-app
call gradlew.bat assembleDebug
if %errorlevel% neq 0 (
    echo ❌ Android Build Failed!
    exit /b 1
)

echo ✅ SUCCESS!
echo 📦 APK Location: flux\android-app\app\build\outputs\apk\debug\app-debug.apk
pause
