#!/usr/bin/env bash
# Builds the tuvi-web server for Android (arm64) and stages it for Gradle:
#
#   build/rust/jniLibs/arm64-v8a/libtuvi_web.so   the server, run as a program
#   build/rust/assets/web/                         its asset bundle
#
# Needs the Topcoat CLI, the `aarch64-linux-android` Rust target and the
# Android NDK (set ANDROID_NDK_HOME).
set -euo pipefail

here=$(cd "$(dirname "$0")" && pwd)
root=$(dirname "$here")
ndk=${ANDROID_NDK_HOME:?set ANDROID_NDK_HOME to the Android NDK folder}

target=aarch64-linux-android
api=24
host="$(uname -s | tr '[:upper:]' '[:lower:]')-x86_64"
toolchain="$ndk/toolchains/llvm/prebuilt/$host/bin"

# The Topcoat CLI runs `cargo build`, which reads the target from here.
export CARGO_BUILD_TARGET=$target
export CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER="$toolchain/$target$api-clang"
export CC_aarch64_linux_android="$toolchain/$target$api-clang"
export AR_aarch64_linux_android="$toolchain/llvm-ar"

cd "$root"
topcoat asset bundle --release -p tuvi-web

release="$root/target/$target/release"
out="$here/build/rust"
rm -rf "$out"
mkdir -p "$out/jniLibs/arm64-v8a" "$out/assets"
# Android only installs files named lib*.so from jniLibs.
cp "$release/tuvi-web" "$out/jniLibs/arm64-v8a/libtuvi_web.so"
cp -r "$release/assets" "$out/assets/web"
echo "staged the server and its assets in $out"
