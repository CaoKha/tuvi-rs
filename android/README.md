# Thiên Hạc for Android

The Android app runs the `tuvi-web` server on the phone and shows it in a
WebView. It works offline and does not use Render.

- `build-rust.sh` builds `tuvi-web` for arm64 Android. It puts the binary and
  its asset bundle in `build/rust`.
- The APK ships the binary as `libtuvi_web.so`. Android unpacks it to the
  native library folder, which is the only folder an app can run a program
  from.
- `MainActivity` copies the asset bundle to app storage and starts the server
  on `127.0.0.1:47613` with `TUVI_ASSETS` set. Then it opens the page.
- The Rust code is the same as the web app's. The only Android-specific part
  is `TUVI_ASSETS` in `crates/tuvi-web/src/app.rs`.

## Install on your phone

On every push to `main`, CI publishes the newest APK as the
[`android-latest` release](https://github.com/CaoKha/tuvi-rs/releases/tag/android-latest).

1. On the phone, open that page and download `thien-hac.apk`.
2. Open the file. Allow your browser to install unknown apps when Android asks.

The app needs an arm64 phone, Android 8 or newer.

## Signing

Android installs an update only when it has the same signing key as the
installed app. Without a key, CI signs each build with a new throwaway key,
and you must uninstall the app before each update. Uninstalling deletes your
favorites and settings.

To keep one key, create it once and store it in GitHub secrets:

```bash
keytool -genkeypair -v -keystore release.jks -alias thienhac \
  -keyalg RSA -keysize 4096 -validity 36500
base64 -w0 release.jks          # the value of ANDROID_KEYSTORE_BASE64
```

| Secret | Value |
| --- | --- |
| `ANDROID_KEYSTORE_BASE64` | the base64 output |
| `ANDROID_KEYSTORE_PASSWORD` | the keystore password |
| `ANDROID_KEY_ALIAS` | `thienhac` |
| `ANDROID_KEY_PASSWORD` | the key password (the keystore password if you did not set one) |

Keep `release.jks` and its password safe, out of git. If you lose it, you must
uninstall the app to install a new build.

## Build locally

You need the Topcoat CLI, Gradle 8.11, JDK 17, the Android SDK and the NDK.

```bash
rustup target add aarch64-linux-android
ANDROID_NDK_HOME=~/Android/Sdk/ndk/<version> android/build-rust.sh
cd android && gradle assembleRelease
adb install -r build/outputs/apk/release/*.apk
```

The server logs to logcat with the tag `ThienHac`.
