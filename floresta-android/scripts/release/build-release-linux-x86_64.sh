#!/bin/bash
set -euo pipefail

if [ -z "$ANDROID_NDK_ROOT" ]; then
    echo "Error: ANDROID_NDK_ROOT is not defined in your environment"
    exit 1
fi

PATH="$ANDROID_NDK_ROOT/toolchains/llvm/prebuilt/linux-x86_64/bin:$PATH"
CFLAGS="-D__ANDROID_MIN_SDK_VERSION__=24"
AR="llvm-ar"
export ANDROID_NDK_HOME="$ANDROID_NDK_ROOT"
ln -sf /usr/include/boost "$ANDROID_NDK_ROOT/toolchains/llvm/prebuilt/linux-x86_64/sysroot/usr/include/boost"
LIB_NAME="libflorestad_ffi.so"
COMPILATION_TARGET_ARM64_V8A="aarch64-linux-android"
COMPILATION_TARGET_X86_64="x86_64-linux-android"
RESOURCE_DIR_ARM64_V8A="arm64-v8a"
RESOURCE_DIR_X86_64="x86_64"
NDK_LIB_DIR="$ANDROID_NDK_ROOT/toolchains/llvm/prebuilt/linux-x86_64/sysroot/usr/lib"
READELF="${READELF:-llvm-readelf}"

if ! command -v "$READELF" >/dev/null 2>&1; then
    READELF="readelf"
fi

if ! command -v "$READELF" >/dev/null 2>&1; then
    echo "Error: llvm-readelf or readelf is required to verify native dependencies"
    exit 1
fi

verify_native_dependencies() {
    local lib_path="$1"
    local dynamic_section

    if ! dynamic_section=$("$READELF" -d "$lib_path"); then
        echo "Error: failed to inspect native dependencies for $lib_path"
        exit 1
    fi

    if grep -q "libc++_shared.so" <<< "$dynamic_section"; then
        echo "Error: $lib_path depends on libc++_shared.so"
        echo "Static libc++ linkage failed; do not package this library without libc++_shared.so."
        exit 1
    fi

    if ! grep -q "Shared library: \[libc.so\]" <<< "$dynamic_section"; then
        echo "Error: $lib_path does not dynamically link libc.so"
        echo "The NDK static libc.a may have been linked into the shared library."
        exit 1
    fi

    if ! grep -q "Shared library: \[libm.so\]" <<< "$dynamic_section"; then
        echo "Error: $lib_path does not dynamically link libm.so"
        echo "The NDK static libm.a may have been linked into the shared library."
        exit 1
    fi
}

export Boost_DIR=/usr/lib/x86_64-linux-gnu/cmake/Boost-1.83.0
export CMAKE_PREFIX_PATH=/usr/lib/x86_64-linux-gnu/cmake
export CARGO_TARGET_AARCH64_LINUX_ANDROID_RUSTFLAGS="-L native=$NDK_LIB_DIR/$COMPILATION_TARGET_ARM64_V8A/24 ${CARGO_TARGET_AARCH64_LINUX_ANDROID_RUSTFLAGS:-}"
export CARGO_TARGET_X86_64_LINUX_ANDROID_RUSTFLAGS="-L native=$NDK_LIB_DIR/$COMPILATION_TARGET_X86_64/24 ${CARGO_TARGET_X86_64_LINUX_ANDROID_RUSTFLAGS:-}"

cd ../floresta-ffi/ || exit
rustup target add $COMPILATION_TARGET_ARM64_V8A $COMPILATION_TARGET_X86_64

CC="aarch64-linux-android24-clang" CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER="aarch64-linux-android24-clang" cargo build --lib --profile release-smaller --target $COMPILATION_TARGET_ARM64_V8A
CC="x86_64-linux-android24-clang" CARGO_TARGET_X86_64_LINUX_ANDROID_LINKER="x86_64-linux-android24-clang" cargo build --lib --profile release-smaller --target $COMPILATION_TARGET_X86_64

verify_native_dependencies "./target/$COMPILATION_TARGET_ARM64_V8A/release-smaller/$LIB_NAME"
verify_native_dependencies "./target/$COMPILATION_TARGET_X86_64/release-smaller/$LIB_NAME"

mkdir -p ../floresta-android/lib/src/main/jniLibs/$RESOURCE_DIR_ARM64_V8A/
mkdir -p ../floresta-android/lib/src/main/jniLibs/$RESOURCE_DIR_X86_64/
rm -f ../floresta-android/lib/src/main/jniLibs/$RESOURCE_DIR_ARM64_V8A/libc++_shared.so
rm -f ../floresta-android/lib/src/main/jniLibs/$RESOURCE_DIR_X86_64/libc++_shared.so
cp ./target/$COMPILATION_TARGET_ARM64_V8A/release-smaller/$LIB_NAME ../floresta-android/lib/src/main/jniLibs/$RESOURCE_DIR_ARM64_V8A/
cp ./target/$COMPILATION_TARGET_X86_64/release-smaller/$LIB_NAME ../floresta-android/lib/src/main/jniLibs/$RESOURCE_DIR_X86_64/

cargo run --bin uniffi-bindgen generate --library ./target/$COMPILATION_TARGET_ARM64_V8A/release-smaller/$LIB_NAME --language kotlin --out-dir ../floresta-android/lib/src/main/kotlin/ --no-format
