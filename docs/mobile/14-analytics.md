# 14 — Analytics & Crash Reporting Module

## Overview

Firebase Analytics for event tracking and user behavior analysis, plus Firebase Crashlytics for crash reporting. Both platforms use identical event schemas and custom dimensions.

---

## Firebase Setup

### Both Platforms
1. Create Firebase project: `aeroxe-customer-app`
2. Register apps (Android: `com.aeroxebroadband.customer`, iOS: `com.aeroxebroadband.customer`)
3. Download config files:
   - Android: `google-services.json` → `app/`
   - iOS: `GoogleService-Info.plist` → Xcode project root
4. Enable Analytics and Crashlytics in Firebase Console

---

## Custom Events

### Authentication Events
| Event Name | Parameters | Description |
|------------|-----------|-------------|
| `login_started` | `method: "otp"`, `phone_hash` | User started login |
| `otp_sent` | `phone_hash`, `delivery_time_ms` | OTP sent successfully |
| `otp_verified` | `phone_hash`, `attempts`, `time_to_verify_ms` | OTP verified |
| `otp_failed` | `phone_hash`, `reason`, `attempts` | OTP verification failed |
| `login_completed` | `phone_hash`, `method`, `biometric_enabled` | Login completed |
| `biometric_enabled` | `biometric_type` | User enabled biometric |
| `biometric_auth_success` | `biometric_type` | Biometric auth successful |
| `biometric_auth_failed` | `biometric_type`, `reason` | Biometric auth failed |
| `logout` | `session_duration_ms` | User logged out |

### Dashboard Events
| Event Name | Parameters | Description |
|------------|-----------|-------------|
| `dashboard_viewed` | `load_time_ms`, `from_cache: bool` | Dashboard screen viewed |
| `dashboard_refreshed` | `refresh_time_ms` | User pulled to refresh |
| `quick_action_tapped` | `action: "pay_bill" \| "ticket" \| "usage"` | Quick action tapped |
| `notification_badge_tapped` | `unread_count` | Tapped notification badge |

### Usage Events
| Event Name | Parameters | Description |
|------------|-----------|-------------|
| `usage_screen_viewed` | `load_time_ms` | Usage screen viewed |
| `usage_chart_toggled` | `period: "daily" \| "monthly"` | Chart period toggled |
| `daily_breakdown_viewed` | `date` | Viewed hourly breakdown |
| `speed_indicator_viewed` | `download_speed`, `upload_speed` | Speed indicator viewed |

### Plan Events
| Event Name | Parameters | Description |
|------------|-----------|-------------|
| `plans_screen_viewed` | `plan_count` | Plans screen viewed |
| `plan_selected` | `plan_id`, `plan_name`, `price` | Plan selected |
| `plan_change_initiated` | `from_plan`, `to_plan`, `price_diff` | Plan change started |
| `plan_change_confirmed` | `plan_id`, `effective_date` | Plan change confirmed |
| `plan_change_failed` | `plan_id`, `error` | Plan change failed |

### Billing Events
| Event Name | Parameters | Description |
|------------|-----------|-------------|
| `invoices_screen_viewed` | `pending_count`, `paid_count` | Invoices screen viewed |
| `invoice_viewed` | `invoice_id`, `amount`, `status` | Invoice detail viewed |
| `payment_initiated` | `invoice_id`, `amount`, `method` | Payment started |
| `payment_completed` | `invoice_id`, `amount`, `method`, `payment_id`, `duration_ms` | Payment succeeded |
| `payment_failed` | `invoice_id`, `amount`, `method`, `error_code`, `error_message` | Payment failed |
| `invoice_pdf_downloaded` | `invoice_id` | Invoice PDF downloaded |

### Support Events
| Event Name | Parameters | Description |
|------------|-----------|-------------|
| `tickets_screen_viewed` | `open_count`, `closed_count` | Tickets screen viewed |
| `ticket_created` | `ticket_id`, `category`, `priority` | Ticket created |
| `ticket_viewed` | `ticket_id`, `status` | Ticket detail viewed |
| `ticket_message_sent` | `ticket_id`, `has_attachments` | Message sent |
| `ticket_closed` | `ticket_id`, `resolution_time_hours` | Ticket closed |

### Notification Events
| Event Name | Parameters | Description |
|------------|-----------|-------------|
| `notification_received` | `type`, `title` | Push notification received |
| `notification_tapped` | `type`, `notification_id` | Push notification tapped |
| `notifications_screen_viewed` | `unread_count` | Notification center viewed |
| `notification_read` | `notification_id`, `type` | Notification marked as read |

### Profile Events
| Event Name | Parameters | Description |
|------------|-----------|-------------|
| `profile_viewed` | `kyc_status` | Profile screen viewed |
| `profile_updated` | `fields_changed: ["name", "email"]` | Profile updated |
| `kyc_document_uploaded` | `document_type`, `file_size` | KYC document uploaded |
| `kyc_submitted` | `documents_count` | KYC submitted for verification |

### Settings Events
| Event Name | Parameters | Description |
|------------|-----------|-------------|
| `theme_changed` | `from_theme`, `to_theme` | Theme changed |
| `language_changed` | `from_lang`, `to_lang` | Language changed |
| `notification_setting_changed` | `setting`, `enabled` | Notification pref changed |
| `cache_cleared` | `cache_size_bytes` | Cache cleared |

---

## Android Implementation

### AnalyticsManager.kt
```kotlin
class AnalyticsManager @Inject constructor(
    private val firebaseAnalytics: FirebaseAnalytics,
    private val crashlytics: FirebaseCrashlytics
) {
    // --- Authentication ---
    fun logLoginStarted(method: String, phoneHash: String) {
        firebaseAnalytics.logEvent("login_started") {
            param("method", method)
            param("phone_hash", phoneHash)
        }
    }
    
    fun logOtpSent(phoneHash: String, deliveryTimeMs: Long) {
        firebaseAnalytics.logEvent("otp_sent") {
            param("phone_hash", phoneHash)
            param("delivery_time_ms", deliveryTimeMs)
        }
    }
    
    fun logOtpVerified(phoneHash: String, attempts: Int, timeToVerifyMs: Long) {
        firebaseAnalytics.logEvent("otp_verified") {
            param("phone_hash", phoneHash)
            param("attempts", attempts.toLong())
            param("time_to_verify_ms", timeToVerifyMs)
        }
    }
    
    fun logLoginCompleted(phoneHash: String, method: String, biometricEnabled: Boolean) {
        firebaseAnalytics.logEvent("login_completed") {
            param("phone_hash", phoneHash)
            param("method", method)
            param("biometric_enabled", biometricEnabled)
        }
    }
    
    // --- Billing ---
    fun logPaymentInitiated(invoiceId: String, amount: Double, method: String) {
        firebaseAnalytics.logEvent("payment_initiated") {
            param("invoice_id", invoiceId)
            param("amount", amount)
            param("method", method)
        }
    }
    
    fun logPaymentCompleted(invoiceId: String, amount: Double, method: String, paymentId: String, durationMs: Long) {
        firebaseAnalytics.logEvent("payment_completed") {
            param("invoice_id", invoiceId)
            param("amount", amount)
            param("method", method)
            param("payment_id", paymentId)
            param("duration_ms", durationMs)
        }
    }
    
    fun logPaymentFailed(invoiceId: String, amount: Double, method: String, errorCode: String, errorMessage: String) {
        firebaseAnalytics.logEvent("payment_failed") {
            param("invoice_id", invoiceId)
            param("amount", amount)
            param("method", method)
            param("error_code", errorCode)
            param("error_message", errorMessage)
        }
    }
    
    // --- Crashlytics ---
    fun setUserId(userId: String) {
        crashlytics.setUserId(userId)
    }
    
    fun logError throwable: Throwable, message: String) {
        crashlytics.recordException(throwable)
        crashlytics.log(message)
    }
    
    fun setCustomKey(key: String, value: String) {
        crashlytics.setCustomKey(key, value)
    }
}

// Extension for easier event logging
fun FirebaseAnalytics.logEvent(name: String, block: AnalyticsParameterBuilder.() -> Unit) {
    val builder = AnalyticsParameterBuilder()
    builder.block()
    logEvent(name, builder.bundle)
}

class AnalyticsParameterBuilder {
    val bundle = Bundle()
    
    fun param(key: String, value: String) { bundle.putString(key, value) }
    fun param(key: String, value: Long) { bundle.putLong(key, value) }
    fun param(key: String, value: Double) { bundle.putDouble(key, value) }
    fun param(key: String, value: Boolean) { bundle.putBoolean(key, value) }
}
```

---

## iOS Implementation

### AnalyticsManager.swift
```swift
import FirebaseAnalytics
import FirebaseCrashlytics

class AnalyticsManager {
    static let shared = AnalyticsManager()
    
    // --- Authentication ---
    func logLoginStarted(method: String, phoneHash: String) {
        Analytics.logEvent("login_started", parameters: [
            "method": method,
            "phone_hash": phoneHash
        ])
    }
    
    func logOtpSent(phoneHash: String, deliveryTimeMs: Int64) {
        Analytics.logEvent("otp_sent", parameters: [
            "phone_hash": phoneHash,
            "delivery_time_ms": deliveryTimeMs
        ])
    }
    
    func logOtpVerified(phoneHash: String, attempts: Int, timeToVerifyMs: Int64) {
        Analytics.logEvent("otp_verified", parameters: [
            "phone_hash": phoneHash,
            "attempts": attempts,
            "time_to_verify_ms": timeToVerifyMs
        ])
    }
    
    func logLoginCompleted(phoneHash: String, method: String, biometricEnabled: Bool) {
        Analytics.logEvent("login_completed", parameters: [
            "phone_hash": phoneHash,
            "method": method,
            "biometric_enabled": biometricEnabled
        ])
    }
    
    // --- Billing ---
    func logPaymentInitiated(invoiceId: String, amount: Double, method: String) {
        Analytics.logEvent("payment_initiated", parameters: [
            "invoice_id": invoiceId,
            "amount": amount,
            "method": method
        ])
    }
    
    func logPaymentCompleted(invoiceId: String, amount: Double, method: String, paymentId: String, durationMs: Int64) {
        Analytics.logEvent("payment_completed", parameters: [
            "invoice_id": invoiceId,
            "amount": amount,
            "method": method,
            "payment_id": paymentId,
            "duration_ms": durationMs
        ])
    }
    
    // --- Crashlytics ---
    func setUserId(_ userId: String) {
        Crashlytics.crashlytics().setUserID(userId)
    }
    
    func logError(_ error: Error, message: String) {
        Crashlytics.crashlytics().record(error: error)
        Crashlytics.crashlytics().log(message)
    }
    
    func setCustomValue(_ value: String, forKey key: String) {
        Crashlytics.crashlytics().setCustomValue(value, forKey: key)
    }
}
```

---

## Screen Tracking

### Android (Compose)
```kotlin
@Composable
fun TrackScreen(name: String) {
    val analytics = LocalContext.current.analyticsManager
    
    LaunchedEffect(name) {
        analytics.logScreenViewed(name)
    }
}

// Usage
@Composable
fun DashboardScreen() {
    TrackScreen("dashboard")
    // ... rest of screen
}
```

### iOS (SwiftUI)
```swift
struct ScreenTracker: ViewModifier {
    let screenName: String
    
    func body(content: Content) -> content {
        content.onAppear {
            AnalyticsManager.shared.logScreenViewed(screenName)
        }
    }
}

extension View {
    func trackScreen(_ name: String) -> some View {
        modifier(ScreenTracker(screenName: name))
    }
}

// Usage
struct DashboardView: View {
    var body: some View {
        // ... content
    }
    .trackScreen("dashboard")
}
```

---

## User Properties

| Property | Type | Description |
|----------|------|-------------|
| `user_id` | String | Customer ID |
| `plan_name` | String | Current plan name |
| `plan_price` | Double | Current plan price |
| `subscription_status` | String | active/suspended/terminated |
| `kyc_status` | String | verified/pending/not_started |
| `account_age_days` | Int | Days since registration |
| `app_version` | String | Current app version |
| `os_version` | String | Device OS version |
| `device_model` | String | Device model |

### Setting User Properties
```kotlin
// Android
firebaseAnalytics.setUserProperty("plan_name", "AeroXe 100")
firebaseAnalytics.setUserProperty("subscription_status", "active")
```

```swift
// iOS
Analytics.setUserProperty("AeroXe 100", forName: "plan_name")
Analytics.setUserProperty("active", forName: "subscription_status")
```

---

## Crash Reporting Best Practices

| Practice | Description |
|----------|-------------|
| Non-fatals | Log non-fatal errors via `recordException` |
| Breadcrumbs | Log key events before crash for context |
| Custom keys | Add user context (plan, subscription status) |
| Symbol upload | Upload dSYMs (iOS) and mapping files (Android) on build |
| Alert rules | Set up Firebase alerts for crash rate spikes |

### Breadcrumbs
```kotlin
// Before risky operation
crashlytics.log("Starting payment for invoice $invoiceId")
crashlytics.log("Razorpay checkout opened")

// If crash occurs, breadcrumbs help understand the context
```

```swift
// Before risky operation
Crashlytics.crashlytics().log("Starting payment for invoice \(invoiceId)")
Crashlytics.crashlytics().log("Razorpay checkout opened")
```
