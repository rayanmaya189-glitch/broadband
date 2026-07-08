# 16 — Build, Signing & Release Process

## Overview

Complete build, code signing, versioning, and release process for both Android (Google Play) and iOS (App Store) apps.

---

## Versioning Strategy

| Component | Format | Example |
|-----------|--------|---------|
| Version Name | `MAJOR.MINOR.PATCH` | `1.2.0` |
| Version Code (Android) | Incrementing integer | `42` |
| Build Number (iOS) | Incrementing integer | `42` |
| Bundle Version | `MAJOR.MINOR.PATCH.BUILD` | `1.2.0.42` |

### Version Bump Rules

| Change Type | Version Bump | Example |
|-------------|-------------|---------|
| Bug fix | PATCH | 1.0.0 → 1.0.1 |
| New feature | MINOR | 1.0.0 → 1.1.0 |
| Breaking change | MAJOR | 1.0.0 → 2.0.0 |
| Hotfix | PATCH + build | 1.0.1 → 1.0.2 |

---

## Android Signing

### Keystore Setup

#### Debug Keystore (Auto-generated)
```properties
# gradle.properties
android.debug.keystore.path=debug.keystore
android.debug.keystore.alias=androiddebugkey
android.debug.keystore.password=android
```

#### Release Keystore
```bash
# Generate release keystore
keytool -genkeypair \
  -v \
  -keystore aeroxe-release.keystore \
  -alias aeroxe \
  -keyalg RSA \
  -keysize 2048 \
  -validity 10000

# Store securely (never commit to git)
# Use Google Play App Signing for production
```

### build.gradle.kts (Signing)
```kotlin
android {
    signingConfigs {
        create("release") {
            storeFile = file(System.getenv("KEYSTORE_PATH") ?: "aeroxe-release.keystore")
            storePassword = System.getenv("KEYSTORE_PASSWORD") ?: ""
            keyAlias = System.getenv("KEY_ALIAS") ?: "aeroxe"
            keyPassword = System.getenv("KEY_PASSWORD") ?: ""
        }
    }
    
    buildTypes {
        release {
            signingConfig = signingConfigs.getByName("release")
            isMinifyEnabled = true
            isShrinkResources = true
            proguardFiles(
                getDefaultProguardFile("proguard-android-optimize.txt"),
                "proguard-rules.pro"
            )
        }
    }
}
```

### ProGuard Rules
```proguard
# Keep Razorpay
-keep class com.razorpay.** { *; }
-keepclassmembers class * {
    @com.razorpay.annotation.* <methods>;
}

# Keep Hilt
-keep class dagger.hilt.** { *; }
-keep class javax.inject.** { *; }

# Keep Room entities
-keep class * extends androidx.room.RoomDatabase
-keep @androidx.room.Entity class *
-dontwarn androidx.room.paging.**

# Keep Kotlin Serialization
-keepattributes *Annotation*, InnerClasses
-dontnote kotlinx.serialization.AnnotationsKt
-keepclassmembers class kotlinx.serialization.json.** { *** Companion; }
```

---

## iOS Signing

### Xcode Configuration

#### Export Options (exportOptions.plist)
```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>method</key>
    <string>app-store</string>
    <key>teamId</key>
    <string>YOUR_TEAM_ID</string>
    <key>uploadBitcode</key>
    <false/>
    <key>uploadSymbols</key>
    <true/>
    <key>compileBitcode</key>
    <false/>
</dict>
</plist>
```

### Keychain Setup (CI/CD)
```bash
# Create keychain
security create-keychain -p "" build.keychain
security default-keychain -s build.keychain
security unlock-keychain -p "" build.keychain

# Import certificates
security import certificate.cer -k build.keychain -P "" -T /usr/bin/codesign
security import distribution.p12 -k build.keychain -P "" -T /usr/bin/codesign

# Set partition list
security set-key-partition-list -S apple-tool:,apple: -s -k "" build.keychain
```

---

## CI/CD Pipeline

### GitHub Actions

#### .github/workflows/android-release.yml
```yaml
name: Android Release

on:
  push:
    tags:
      - 'v*'

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Set up JDK 17
        uses: actions/setup-java@v4
        with:
          java-version: '17'
          distribution: 'temurin'
      
      - name: Setup Gradle
        uses: gradle/actions/setup-gradle@v3
      
      - name: Decode Keystore
        run: echo "${{ secrets.KEYSTORE_BASE64 }}" | base64 -d > aeroxe-release.keystore
      
      - name: Build Release APK
        run: ./gradlew assembleRelease
        env:
          KEYSTORE_PASSWORD: ${{ secrets.KEYSTORE_PASSWORD }}
          KEY_ALIAS: ${{ secrets.KEY_ALIAS }}
          KEY_PASSWORD: ${{ secrets.KEY_PASSWORD }}
      
      - name: Build Release AAB
        run: ./gradlew bundleRelease
      
      - name: Sign AAB
        run: |
          jarsigner -verbose -sigalg SHA256withRSA -digestalg SHA-256 \
            -keystore aeroxe-release.keystore \
            app/build/outputs/bundle/release/app-release.aab \
            ${{ secrets.KEY_ALIAS }}
      
      - name: Upload to Google Play
        uses: r0adkll/upload-google-play@v1
        with:
          serviceAccountJsonPlainText: ${{ secrets.GOOGLE_PLAY_SERVICE_ACCOUNT }}
          packageName: com.aeroxebroadband.customer
          releaseFiles: app/build/outputs/bundle/release/app-release.aab
          track: internal
          status: completed
      
      - name: Upload Artifact
        uses: actions/upload-artifact@v4
        with:
          name: android-release
          path: |
            app/build/outputs/apk/release/*.apk
            app/build/outputs/bundle/release/*.aab
```

#### .github/workflows/ios-release.yml
```yaml
name: iOS Release

on:
  push:
    tags:
      - 'v*'

jobs:
  build:
    runs-on: macos-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Setup Ruby
        uses: ruby/setup-ruby@v1
        with:
          ruby-version: '3.2'
      
      - name: Install Fastlane
        run: gem install fastlane
      
      - name: Setup Keychain
        run: |
          security create-keychain -p "" build.keychain
          security default-keychain -s build.keychain
          security unlock-keychain -p "" build.keychain
          echo "${{ secrets.IOS_CERTIFICATE_BASE64 }}" | base64 -d > cert.p12
          security import cert.p12 -k build.keychain -P "${{ secrets.IOS_CERTIFICATE_PASSWORD }}" -T /usr/bin/codesign
          security set-key-partition-list -S apple-tool:,apple: -s -k "" build.keychain
      
      - name: Install Provisioning Profile
        run: |
          echo "${{ secrets.IOS_PROVISION_PROFILE_BASE64 }}" | base64 -d > profile.mobileprovision
          mkdir -p ~/Library/MobileDevice/Provisioning\ Profiles
          UUID=$(security cms -D -i profile.mobileprovision | plutil -extract UUID raw -)
          cp profile.mobileprovision ~/Library/MobileDevice/Provisioning\ Profiles/$UUID.mobileprovision
      
      - name: Build & Archive
        run: |
          xcodebuild archive \
            -project AeroXe.xcodeproj \
            -scheme AeroXe \
            -archivePath build/AeroXe.xcarchive \
            -destination 'generic/platform=iOS' \
            CODE_SIGN_IDENTITY="${{ secrets.IOS_SIGNING_IDENTITY }}" \
            PROVISIONING_PROFILE_SPECIFIER="${{ secrets.IOS_PROVISIONING_PROFILE }}" \
            DEVELOPMENT_TEAM="${{ secrets.IOS_TEAM_ID }}"
      
      - name: Export IPA
        run: |
          xcodebuild -exportArchive \
            -archivePath build/AeroXe.xcarchive \
            -exportOptionsPlist exportOptions.plist \
            -exportPath build/output
      
      - name: Upload to TestFlight
        run: |
          fastlane ios upload_to_testflight \
            api_key_id:${{ secrets.APP_STORE_CONNECT_KEY_ID }} \
            api_key_issuer_id:${{ secrets.APP_STORE_CONNECT_ISSUER_ID }} \
            api_key:${{ secrets.APP_STORE_CONNECT_API_KEY }}
      
      - name: Upload Artifact
        uses: actions/upload-artifact@v4
        with:
          name: ios-release
          path: build/output/*.ipa
```

### Fastlane (iOS)

#### Fastfile
```ruby
default_platform(:ios)

platform :ios do
  desc "Push to TestFlight"
  lane :beta do
    increment_build_number(xcodeproj: "AeroXe.xcodeproj")
    build_app(
      scheme: "AeroXe",
      export_method: "app-store",
      output_directory: "./build"
    )
    upload_to_testflight(skip_waiting_for_build_processing: true)
  end
  
  desc "Push to App Store"
  lane :release do
    build_app(
      scheme: "AeroXe",
      export_method: "app-store",
      output_directory: "./build"
    )
    upload_to_app_store(
      force: true,
      skip_screenshots: true,
      skip_metadata: false
    )
  end
  
  desc "Capture screenshots"
  lane :screenshots do
    capture_screenshots(
      scheme: "AeroXeUITests",
      devices: ["iPhone 15 Pro Max", "iPhone 15", "iPad Pro 12.9-inch (6th generation)"]
    )
  end
end
```

---

## Release Checklist

### Pre-Release
- [ ] All unit tests pass
- [ ] All UI tests pass
- [ ] Code coverage meets target (75%)
- [ ] Lint checks pass (no errors)
- [ ] QA testing complete
- [ ] Staging build verified
- [ ] Version number bumped
- [ ] Changelog updated
- [ ] Screenshots updated (if UI changed)

### Android Release
- [ ] Keystore configured
- [ ] ProGuard rules verified
- [ ] AAB built and signed
- [ ] Uploaded to Internal Testing
- [ ] Internal testing approved
- [ ] Promoted to Production
- [ ] Google Play listing updated

### iOS Release
- [ ] Certificates valid
- [ ] Provisioning profile active
- [ ] Archive built successfully
- [ ] Uploaded to TestFlight
- [ ] TestFlight testing approved
- [ ] App Store review submitted
- [ ] App Store listing updated

### Post-Release
- [ ] Monitor crash reports (24-48 hours)
- [ ] Check user reviews
- [ ] Verify analytics events firing
- [ ] Monitor API error rates
- [ ] Check performance metrics

---

## Environment Promotion

```
┌──────────┐     ┌──────────┐     ┌──────────┐
│  Dev      │────▶│ Staging  │────▶│Production │
│          │     │          │     │          │
│ localhost │     │ staging- │     │ api.aero │
│ :8080    │     │ api.aero │     │xebroadband│
│          │     │band.com  │     │.com      │
└──────────┘     └──────────┘     └──────────┘
     │                │                │
     ▼                ▼                ▼
  Debug APK      Staging APK      Release AAB
  Debug IPA      TestFlight        App Store
```

---

## Hotfix Process

1. Create hotfix branch from `main`
2. Fix the bug
3. Bump PATCH version
4. Run tests
5. Create PR → merge to `main`
6. Tag: `v1.0.2-hotfix`
7. CI/CD builds and deploys
8. Monitor crash reports
