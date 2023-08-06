#!/usr/bin/env bash

set -ex

adb install -r android/app/build/outputs/apk/release/app-release.apk

adb shell am start -n 'com.mahjong_rust/com.mahjong_rust.MainActivity'
