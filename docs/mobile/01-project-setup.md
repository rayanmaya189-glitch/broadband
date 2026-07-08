# 01 — Project Setup

## Android Project Setup (§17.1)

### build.gradle.kts (Project Level)
```kotlin
plugins {
    id("com.android.application") version "8.5.0" apply false
    id("org.jetbrains.kotlin.android") version "2.0.0" apply false
    id("com.google.dagger.hilt.android") version "2.51.1" apply false
    id("com.google.gms.google-services") version "4.4.2" apply false
    id("com.google.firebase.crashlytics") version "3.0.2" apply false
}
```

### build.gradle.kts (App Level)
```kotlin
plugins {
    id("com.android.application")
    id("org.jetbrains.kotlin.android")
    id("com.google.dagger.hilt.android")
    id("com.google.gms.google-services")
    id("com.google.firebase.crashlytics")
    id("kotlin-parcelize")
    id("kotlinx-serialization")
}

android {
    namespace = "com.aeroxebroadband.customer"
    compileSdk = 34
    
    defaultConfig {
        applicationId = "com.aeroxebroadband.customer"
        minSdk = 26
        targetSdk = 34
        versionCode = 1
        versionName = "1.0.0"
        
        testInstrumentationRunner = "androidx.test.runner.AndroidJUnitRunner"
        
        // Build flavors for environments
        buildConfigField("String", "BASE_URL", "\"https://api.aeroxebroadband.com\"")
    }
    
    buildTypes {
        debug {
            applicationIdSuffix = ".debug"
            isDebuggable = true
            buildConfigField("String", "BASE_URL", "\"http://10.0.2.2:8080\"")
        }
        release {
            isMinifyEnabled = true
            isShrinkResources = true
            proguardFiles(
                getDefaultProguardFile("proguard-android-optimize.txt"),
                "proguard-rules.pro"
            )
        }
    }
    
    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }
    
    kotlinOptions {
        jvmTarget = "17"
    }
    
    buildFeatures {
        compose = true
        buildConfig = true
    }
    
    composeOptions {
        kotlinCompilerExtensionVersion = "1.5.14"
    }
}

dependencies {
    // Core
    implementation("androidx.core:core-ktx:1.13.1")
    implementation("androidx.lifecycle:lifecycle-runtime-ktx:2.8.4")
    implementation("androidx.activity:activity-compose:1.9.1")
    
    // Compose BOM
    implementation(platform("androidx.compose:compose-bom:2024.06.00"))
    implementation("androidx.compose.ui:ui")
    implementation("androidx.compose.ui:ui-graphics")
    implementation("androidx.compose.ui:ui-tooling-preview")
    implementation("androidx.compose.material3:material3")
    implementation("androidx.compose.material:material-icons-extended")
    
    // Navigation
    implementation("androidx.navigation:navigation-compose:2.7.7")
    
    // Hilt
    implementation("com.google.dagger:hilt-android:2.51.1")
    ksp("com.google.dagger:hilt-android-compiler:2.51.1")
    implementation("androidx.hilt:hilt-navigation-compose:1.2.0")
    
    // Retrofit + OkHttp
    implementation("com.squareup.retrofit2:retrofit:2.11.0")
    implementation("com.squareup.retrofit2:converter-gson:2.11.0")
    implementation("com.squareup.okhttp3:okhttp:4.12.0")
    implementation("com.squareup.okhttp3:logging-interceptor:4.12.0")
    implementation("com.squareup.okhttp3:okhttp-sse:4.12.0")
    
    // Room
    implementation("androidx.room:room-runtime:2.6.1")
    implementation("androidx.room:room-ktx:2.6.1")
    ksp("androidx.room:room-compiler:2.6.1")
    
    // WorkManager
    implementation("androidx.work:work-runtime-ktx:2.9.1")
    implementation("androidx.hilt:hilt-work:1.2.0")
    ksp("androidx.hilt:hilt-compiler:1.2.0")
    
    // Firebase
    implementation(platform("com.google.firebase:firebase-bom:33.1.2"))
    implementation("com.google.firebase:firebase-messaging-ktx")
    implementation("com.google.firebase:firebase-analytics-ktx")
    implementation("com.google.firebase:firebase-crashlytics-ktx")
    
    // Coil (Image Loading)
    implementation("io.coil-kt:coil-compose:2.6.0")
    
    // Razorpay
    implementation("com.razorpay:checkout:1.6.27")
    
    // Biometric
    implementation("androidx.biometric:biometric:1.2.0-alpha05")
    
    // DataStore
    implementation("androidx.datastore:datastore-preferences:1.1.1")
    
    // Kotlin Serialization
    implementation("org.jetbrains.kotlinx:kotlinx-serialization-json:1.7.1")
    
    // Testing
    testImplementation("junit:junit:4.13.2")
    testImplementation("org.jetbrains.kotlinx:kotlinx-coroutines-test:1.8.1")
    testImplementation("io.mockk:mockk:1.13.11")
    testImplementation("app.cash.turbine:turbine:1.1.0")
    androidTestImplementation("androidx.test.ext:junit:1.2.1")
    androidTestImplementation("androidx.compose.ui:ui-test-junit4")
}
```

---

## iOS Project Setup (§18.1)

### Project Configuration
- **Minimum Deployment Target**: iOS 17.0
- **Swift Version**: 5.9
- **Xcode**: 15.0+

### SPM Dependencies (Package.swift)
```swift
dependencies: [
    // Networking
    .package(url: "https://github.com/Alamofire/Alamofire.git", from: "5.9.0"),
    
    // DI
    .package(url: "https://github.com/hmlongco/Factory.git", from: "2.3.0"),
    
    // Charts
    .package(url: "https://github.com/ChartsOrg/Charts.git", from: "5.0.0"),
    
    // Keychain
    .package(url: "https://github.com/evgenyneu/keychain-swift.git", from: "23.0.0"),
    
    // Firebase
    .package(url: "https://github.com/firebase/firebase-ios-sdk.git", from: "11.0.0"),
    
    // Razorpay
    .package(url: "https://github.com/razorpay/razorpay-ios-sdk.git", from: "1.2.0"),
]
```

### Info.plist Keys
```xml
<!-- Camera & Photo Library (KYC documents) -->
<key>NSCameraUsageDescription</key>
<string>Take a photo for document verification</string>
<key>NSPhotoLibraryUsageDescription</key>
<string>Select a photo for document verification</string>

<!-- Biometric -->
<key>NSFaceIDUsageDescription</key>
<string>Use Face ID for quick sign-in</string>

<!-- Push Notifications -->
<key>UIBackgroundModes</key>
<array>
    <string>fetch</string>
    <string>remote-notification</string>
</array>
```

---

## Firebase Configuration

### Both Platforms
1. Create Firebase project: `aeroxe-customer-app`
2. Register Android app: `com.aeroxebroadband.customer`
3. Register iOS app: `com.aeroxebroadband.customer`
4. Download config files:
   - Android: `google-services.json` → `app/`
   - iOS: `GoogleService-Info.plist` → Xcode project root
5. Enable Cloud Messaging for both platforms

### FCM/APNs Setup
- **Android**: FCM token sent to `POST /api/v1/customer/devices/fcm`
- **iOS**: APNs device token → converted to FCM token → sent to `POST /api/v1/customer/devices/apns`

---

## Environment Configuration

| Environment | Base URL | Notes |
|-------------|----------|-------|
| Development | `http://10.0.2.2:8080` (Android) / `http://localhost:8080` (iOS) | Local backend |
| Staging | `https://staging-api.aeroxebroadband.com` | Staging server |
| Production | `https://api.aeroxebroadband.com` | Live server |

### Android Build Flavors
```kotlin
productFlavors {
    create("dev") {
        dimension = "environment"
        buildConfigField("String", "BASE_URL", "\"http://10.0.2.2:8080\"")
    }
    create("staging") {
        dimension = "environment"
        buildConfigField("String", "BASE_URL", "\"https://staging-api.aeroxebroadband.com\"")
    }
    create("prod") {
        dimension = "environment"
        buildConfigField("String", "BASE_URL", "\"https://api.aeroxebroadband.com\"")
    }
}
```

### iOS Build Configurations
- **Debug**: Uses `Dev.xcconfig` → `http://localhost:8080`
- **Staging**: Uses `Staging.xcconfig` → `https://staging-api.aeroxebroadband.com`
- **Release**: Uses `Release.xcconfig` → `https://api.aeroxebroadband.com`

---

## Code Generation & Tools

| Tool | Android | iOS |
|------|---------|-----|
| DI | Hilt (KSP) | Factory |
| Serialization | Kotlin Serialization | Codable |
| DB | Room (KSP) | SwiftData |
| Navigation | Compose Navigation (Type-safe) | NavigationStack (Type-safe) |
| Lint | Detekt + Compose Lint | SwiftLint |
| Formatting | Ktlint | SwiftFormat |
