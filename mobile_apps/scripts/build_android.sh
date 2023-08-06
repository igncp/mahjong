#!/usr/bin/env bash

set -e

npx react-native bundle \
  --platform android \
  --dev false \
  --entry-file index.js \
  --bundle-output android/app/src/main/assets/index.android.bundle \
  --assets-dest android/app/src/main/res

rm -rf android/app/src/main/res/drawable-*
rm -rf android/app/src/main/res/raw

cd android

./gradlew clean
./gradlew --stop
./gradlew assembleRelease --stacktrace > /tmp/android_build.log 2>&1 || (echo "Build failed, check /tmp/android_build.log" && exit 1)
