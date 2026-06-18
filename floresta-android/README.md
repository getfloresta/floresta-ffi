# floresta-android

This project builds an `.aar` package for the Android platform that provides Kotlin language bindings for the [Floresta](https://github.com/getfloresta/Floresta) Bitcoin Utreexo full node. The Kotlin language bindings are created by the [`floresta-ffi`](https://github.com/getfloresta/floresta-ffi) project which is included in the same repository.

## How to Use

To use the Kotlin language bindings for Floresta in your Android project add the following to your gradle dependencies:

```kotlin
repositories {
    mavenCentral()
}

dependencies {
    implementation("org.getfloresta:floresta-android:<version>")
}
```

## How to Build

1. Clone this repository:
   ```bash
   git clone https://github.com/getfloresta/floresta-ffi
   ```

2. Install Android SDK and Build-Tools for API level 30+

3. Setup `ANDROID_SDK_ROOT` and `ANDROID_NDK_ROOT` path variables. NDK version 27.2.12479018 or above is recommended. For example:
   ```bash
   # Linux
   export ANDROID_SDK_ROOT=/usr/local/lib/android/sdk
   export ANDROID_NDK_ROOT=$ANDROID_SDK_ROOT/ndk/27.2.12479018

   # macOS
   export ANDROID_SDK_ROOT=~/Library/Android/sdk
   export ANDROID_NDK_ROOT=$ANDROID_SDK_ROOT/ndk/27.2.12479018
   ```

4. Build the Rust library and generate Kotlin bindings:
   ```bash
   cd floresta-android
   bash ./scripts/release/build-release-linux-x86_64.sh
   ```

5. Build the AAR:
   ```bash
   ./gradlew assembleRelease
   ```

6. Start an Android emulator and run tests:
   ```bash
   ./gradlew connectedAndroidTest
   ```

## How to Publish to Your Local Maven Repo

```bash
just publish-local
```

## Known Issues

### JNA dependency

Depending on the JVM version you use, you might not have the JNA dependency on your classpath. The exception thrown will be:
```
class file for com.sun.jna.Pointer not found
```

The solution is to add JNA as a dependency like so:
```kotlin
dependencies {
    implementation("net.java.dev.jna:jna:5.14.0")
}
```

### x86 emulators

For some older versions of macOS, Android Studio will recommend users install the x86 version of the emulator by default. This will not work with the floresta-android library, as we do not support 32-bit x86 architectures. Make sure you install an x86_64 emulator.