// The Thiên Hạc Android app: a WebView over the tuvi-web server, which runs
// on the phone. Run `./build-rust.sh` first; it stages the server and its
// assets in build/rust.
plugins {
    id("com.android.application") version "8.7.3"
}

// CI sets these. Each run gets a higher versionCode, so a new APK installs
// over the old one.
val runNumber = System.getenv("GITHUB_RUN_NUMBER")?.toIntOrNull() ?: 1
val keystore = System.getenv("ANDROID_KEYSTORE_FILE")?.let(::file)?.takeIf { it.isFile }

android {
    namespace = "io.github.caokha.thienhac"
    compileSdk = 35

    defaultConfig {
        applicationId = "io.github.caokha.thienhac"
        minSdk = 26
        targetSdk = 35
        versionCode = runNumber
        versionName = "0.1.$runNumber"
        ndk { abiFilters += "arm64-v8a" }
    }

    signingConfigs {
        if (keystore != null) {
            create("release") {
                storeFile = keystore
                storePassword = System.getenv("ANDROID_KEYSTORE_PASSWORD")
                keyAlias = System.getenv("ANDROID_KEY_ALIAS")
                keyPassword = System.getenv("ANDROID_KEY_PASSWORD")
            }
        }
    }

    buildTypes {
        release {
            isMinifyEnabled = false
            // Without a keystore the APK still installs, but each build has a
            // new key, so you must uninstall the old app first.
            signingConfig = signingConfigs.findByName("release") ?: signingConfigs.getByName("debug")
        }
    }

    sourceSets["main"].apply {
        jniLibs.srcDir("build/rust/jniLibs")
        assets.srcDir("build/rust/assets")
    }

    packaging {
        // Unpack the server to the native library folder, the one place an
        // app may run a program from.
        jniLibs.useLegacyPackaging = true
    }

    androidResources {
        // Keep every bundled file, even names that start with `.` or `_`.
        ignoreAssetsPattern = "!.svn:!.git"
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }
}
