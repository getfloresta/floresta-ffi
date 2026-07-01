#!/bin/bash
set -euo pipefail

if [ -z "$ANDROID_NDK_ROOT" ]; then
    echo "Error: ANDROID_NDK_ROOT is not defined in your environment"
    exit 1
fi

ARCH=$(uname -m)
if [ "$ARCH" = "x86_64" ]; then
    NDK_HOST="linux-x86_64"
elif [ "$ARCH" = "aarch64" ]; then
    NDK_HOST="linux-aarch64"
else
    echo "Error: unsupported host architecture: $ARCH"
    exit 1
fi

PATH="$ANDROID_NDK_ROOT/toolchains/llvm/prebuilt/$NDK_HOST/bin:$PATH"
CFLAGS="-D__ANDROID_MIN_SDK_VERSION__=24"
AR="llvm-ar"
export ANDROID_NDK_HOME="$ANDROID_NDK_ROOT"
ln -sf /usr/include/boost "$ANDROID_NDK_ROOT/toolchains/llvm/prebuilt/$NDK_HOST/sysroot/usr/include/boost"
LIB_NAME="libflorestad_ffi.so"
COMPILATION_TARGET_ARM64_V8A="aarch64-linux-android"
RESOURCE_DIR_ARM64_V8A="arm64-v8a"
NDK_LIB_DIR="$ANDROID_NDK_ROOT/toolchains/llvm/prebuilt/$NDK_HOST/sysroot/usr/lib"

export CARGO_TARGET_AARCH64_LINUX_ANDROID_RUSTFLAGS="-L native=$NDK_LIB_DIR/$COMPILATION_TARGET_ARM64_V8A/24 ${CARGO_TARGET_AARCH64_LINUX_ANDROID_RUSTFLAGS:-}"

cd ../floresta-ffi/ || exit
rustup target add $COMPILATION_TARGET_ARM64_V8A

CC="aarch64-linux-android24-clang" CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER="aarch64-linux-android24-clang" cargo build --lib --target $COMPILATION_TARGET_ARM64_V8A

mkdir -p ../floresta-android/lib/src/main/jniLibs/$RESOURCE_DIR_ARM64_V8A/
rm -f ../floresta-android/lib/src/main/jniLibs/$RESOURCE_DIR_ARM64_V8A/libc++_shared.so
cp ./target/$COMPILATION_TARGET_ARM64_V8A/debug/$LIB_NAME ../floresta-android/lib/src/main/jniLibs/$RESOURCE_DIR_ARM64_V8A/

cargo run --bin uniffi-bindgen generate --library ./target/$COMPILATION_TARGET_ARM64_V8A/debug/$LIB_NAME --language kotlin --out-dir ../floresta-android/lib/src/main/kotlin/ --no-format
