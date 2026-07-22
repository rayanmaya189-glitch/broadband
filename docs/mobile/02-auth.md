# 02 — Authentication Module

## Overview

Phone + SMS-based OTP authentication with optional biometric unlock. Both platforms share identical API contracts and user flows.

---

## User Flow

```
┌──────────────┐     ┌──────────────┐     ┌──────────────┐     ┌──────────────┐
│  Splash       │────▶│  Phone Input  │────▶│  OTP Verify  │────▶│  Dashboard   │
│  (auto-login) │     │  Screen       │     │  Screen      │     │  Screen      │
└──────────────┘     └──────────────┘     └──────────────┘     └──────────────┘
       │                    │                    │
       │ (has valid token)  │                    │
       ▼                    ▼                    ▼
  ┌──────────┐       ┌──────────┐        ┌──────────────┐
  │ Dashboard │       │ Send OTP │        │ Biometric    │
  │ (skip auth)│       │ API call │        │ Setup Prompt │
  └──────────┘       └──────────┘        └──────────────┘
```

---

## Screens

### Phone Input Screen
```
┌─────────────────────────────┐
│         AeroXe              │
│    🌐 Broadband             │
│                             │
│  Welcome! Sign in to        │
│  manage your connection.    │
│                             │
│  ┌─ Country Code ─┬────────┐│
│  │ +91            │ Phone  ││
│  └────────────────┴────────┘│
│                             │
│  ┌─────────────────────────┐│
│  │     Send OTP →          ││
│  └─────────────────────────┘│
│                             │
│  By continuing, you agree   │
│  to our Terms & Privacy     │
│  Policy                     │
└─────────────────────────────┘
```

**Validations:**
- Phone number: 10 digits, starting with 6-9
- Country code: +91 (India) default
- Disable button while loading
- Show error for invalid numbers

### OTP Verification Screen
```
┌─────────────────────────────┐
│         AeroXe              │
│                             │
│  Enter the 6-digit code     │
│  sent to +91 98765 43210   │
│                             │
│  ┌───┬───┬───┬───┬───┬───┐ │
│  │ 1 │ 2 │ 3 │ _ │ _ │ _ │ │
│  └───┴───┴───┴───┴───┴───┘ │
│                             │
│  Resend OTP in 00:45        │
│  (or) Resend OTP            │
│                             │
│  ┌─────────────────────────┐│
│  │     Verify →            ││
│  └─────────────────────────┘│
│                             │
│  ← Change phone number      │
└─────────────────────────────┘
```

**Behavior:**
- Auto-focus first input field
- Auto-submit when 6 digits entered
- Resend countdown: 60 seconds
- OTP expiry: 5 minutes (server-side)
- Show "Invalid OTP" error with shake animation
- Max 3 attempts → lock for 5 minutes

### Biometric Setup Prompt (Optional)
```
┌─────────────────────────────┐
│                             │
│         🔐                  │
│                             │
│  Enable Biometric Login?    │
│                             │
│  Use fingerprint or face    │
│  to sign in quickly next    │
│  time.                      │
│                             │
│  ┌─────────────────────────┐│
│  │     Enable →            ││
│  └─────────────────────────┘│
│                             │
│       Skip for now          │
└─────────────────────────────┘
```

---

## API Integration

### Send OTP
```kotlin
// Request
POST /api/v1/auth/otp/send
{
    "phone": "+919876543210"
}

// Response 200
{
    "success": true,
    "message": "OTP sent successfully",
    "expires_in": 300  // seconds
}

// Response 429 (rate limited)
{
    "success": false,
    "error": "Too many requests",
    "retry_after": 60
}
```

### Verify OTP
```kotlin
// Request
POST /api/v1/auth/otp/verify
{
    "phone": "+919876543210",
    "otp": "123456"
}

// Response 200
{
    "success": true,
    "data": {
        "access_token": "eyJ...",
        "refresh_token": "eyJ...",
        "expires_in": 900,
        "customer": {
            "id": "cust_abc123",
            "name": "Rahul Patil",
            "phone": "+919876543210",
            "status": "active",
            "kyc_status": "verified",
            "subscription_status": "active"
        }
    }
}

// Response 401
{
    "success": false,
    "error": "Invalid OTP",
    "attempts_remaining": 2
}
```

### Refresh Token
```kotlin
// Request
POST /api/v1/auth/refresh
{
    "refresh_token": "eyJ..."
}

// Response 200
{
    "success": true,
    "data": {
        "access_token": "eyJ...",
        "expires_in": 900
    }
}
```

### Logout
```kotlin
// Request
POST /api/v1/auth/logout
Headers: Authorization: Bearer {access_token}

// Response 200
{
    "success": true,
    "message": "Logged out successfully"
}
```

---

## Android Implementation

### AuthRepository.kt
```kotlin
interface AuthRepository {
    suspend fun sendOtp(phone: String): Result<OtpResponse>
    suspend fun verifyOtp(phone: String, otp: String): Result<AuthTokens>
    suspend fun refreshToken(): Result<TokenResponse>
    suspend fun logout(): Result<Unit>
    suspend fun getStoredTokens(): AuthTokens?
    suspend fun clearTokens()
}
```

### AuthViewModel.kt (MVI)
```kotlin
// State
data class AuthUiState(
    val phone: String = "",
    val otp: String = "",
    val step: AuthStep = AuthStep.Phone,
    val isLoading: Boolean = false,
    val error: String? = null,
    val otpExpiry: Int = 0,
    val resendAvailable: Boolean = false,
    val biometricAvailable: Boolean = false,
    val biometricEnabled: Boolean = false
)

// Events
sealed class AuthEvent {
    data class PhoneChanged(val phone: String) : AuthEvent()
    data class OtpChanged(val otp: String) : AuthEvent()
    object SendOtp : AuthEvent()
    object VerifyOtp : AuthEvent()
    object ResendOtp : AuthEvent()
    object EnableBiometric : AuthEvent()
    object SkipBiometric : AuthEvent()
    object ClearError : AuthEvent()
}
```

### Biometric Auth (Android)
```kotlin
class BiometricManager(private val context: Context) {
    
    private val biometricPrompt by lazy {
        val executor = ContextCompat.getMainExecutor(context)
        
        BiometricPrompt(
            context as FragmentActivity,
            executor,
            object : BiometricPrompt.AuthenticationCallback() {
                override fun onAuthenticationSucceeded(result: AuthenticationResult) {
                    // Navigate to dashboard
                }
                override fun onAuthenticationError(errorCode: Int, errString: CharSequence) {
                    // Handle error
                }
                override fun onAuthenticationFailed() {
                    // Show "Try again" message
                }
            }
        )
    }
    
    fun isBiometricAvailable(): Boolean {
        val manager = context.getSystemService(BiometricManager::class.java)
        return manager.canAuthenticate(
            BiometricManager.Authenticators.BIOMETRIC_STRONG
        ) == BiometricManager.BIOMETRIC_SUCCESS
    }
    
    fun authenticate() {
        val promptInfo = BiometricPrompt.PromptInfo.Builder()
            .setTitle("AeroXe Sign In")
            .setSubtitle("Verify your identity")
            .setNegativeButtonText("Use OTP instead")
            .build()
        
        biometricPrompt.authenticate(promptInfo)
    }
}
```

### Secure Token Storage (Android)
```kotlin
class SecureTokenStorage @Inject constructor(
    private val dataStore: DataStore<Preferences>
) {
    companion object {
        val ACCESS_TOKEN = stringPreferencesKey("access_token")
        val REFRESH_TOKEN = stringPreferencesKey("refresh_token")
        val PHONE = stringPreferencesKey("phone")
        val BIOMETRIC_ENABLED = booleanPreferencesKey("biometric_enabled")
    }
    
    suspend fun saveTokens(tokens: AuthTokens) {
        dataStore.edit { prefs ->
            prefs[ACCESS_TOKEN] = tokens.accessToken
            prefs[REFRESH_TOKEN] = tokens.refreshToken
        }
    }
    
    suspend fun getAccessToken(): String? {
        return dataStore.data.first()[ACCESS_TOKEN]
    }
    
    suspend fun clear() {
        dataStore.edit { prefs ->
            prefs.clear()
        }
    }
}
```

---

## iOS Implementation

### AuthRepository.swift
```swift
protocol AuthRepositoryProtocol {
    func sendOtp(phone: String) async throws -> OtpResponse
    func verifyOtp(phone: String, otp: String) async throws -> AuthTokens
    func refreshToken() async throws -> TokenResponse
    func logout() async throws
    func getStoredTokens() -> AuthTokens?
    func clearTokens()
}
```

### AuthViewModel.swift
```swift
@Observable
class AuthViewModel {
    var phone: String = ""
    var otp: String = ""
    var step: AuthStep = .phone
    var isLoading: Bool = false
    var error: String?
    var otpExpiry: Int = 0
    var resendAvailable: Bool = false
    
    enum AuthStep {
        case phone, otp, biometricSetup, done
    }
}
```

### Biometric Auth (iOS)
```swift
import LocalAuthentication

class BiometricManager {
    func isBiometricAvailable() -> Bool {
        let context = LAContext()
        var error: NSError?
        return context.canEvaluatePolicy(
            .deviceOwnerAuthenticationWithBiometrics,
            error: &error
        )
    }
    
    func authenticate() async throws -> Bool {
        let context = LAContext()
        context.localizedCancelTitle = "Use OTP"
        
        let reason = "Sign in to AeroXe Broadband"
        
        return try await context.evaluatePolicy(
            .deviceOwnerAuthenticationWithBiometrics,
            localizedReason: reason
        )
    }
    
    func biometricType() -> LABiometryType {
        let context = LAContext()
        var error: NSError?
        guard context.canEvaluatePolicy(
            .deviceOwnerAuthenticationWithBiometrics,
            error: &error
        ) else {
            return .none
        }
        return context.biometryType
    }
}
```

### Keychain Storage (iOS)
```swift
class KeychainManager {
    func saveTokens(_ tokens: AuthTokens) throws {
        try KeychainSwift().set(
            tokens.accessToken,
            forKey: "access_token",
            withAccess: .whenUnlockedThisDeviceOnly
        )
        try KeychainSwift().set(
            tokens.refreshToken,
            forKey: "refresh_token",
            withAccess: .whenUnlockedThisDeviceOnly
        )
    }
    
    func getAccessToken() -> String? {
        KeychainSwift().get("access_token")
    }
    
    func clear() {
        KeychainSwift().delete("access_token")
        KeychainSwift().delete("refresh_token")
    }
}
```

---

## Token Refresh Interceptor

### Android (OkHttp Interceptor)
```kotlin
class AuthInterceptor @Inject constructor(
    private val tokenStorage: SecureTokenStorage,
    private val authApi: AuthApi
) : Interceptor {
    
    override fun intercept(chain: Interceptor.Chain): Response {
        val original = chain.request()
        
        // Skip auth for login endpoints
        if (original.url.encodedPath.startsWith("/api/v1/auth/")) {
            return chain.proceed(original)
        }
        
        // Add access token
        val token = runBlocking { tokenStorage.getAccessToken() }
        val request = original.newBuilder()
            .header("Authorization", "Bearer $token")
            .build()
        
        val response = chain.proceed(request)
        
        // If 401, try refresh
        if (response.code == 401) {
            val newToken = runBlocking { 
                authApi.refreshToken(
                    RefreshRequest(tokenStorage.getRefreshToken()!!)
                )
            }
            if (newToken.isSuccessful) {
                val refreshedRequest = original.newBuilder()
                    .header("Authorization", "Bearer ${newToken.body()!!.accessToken}")
                    .build()
                return chain.proceed(refreshedRequest)
            } else {
                // Clear tokens → navigate to login
                runBlocking { tokenStorage.clear() }
            }
        }
        
        return response
    }
}
```

### iOS (URLProtocol)
```swift
class AuthInterceptor: AsyncURLProtocol {
    override func sendAsync(for client: URLSessionClient) async throws {
        var request = self.request
        
        // Add access token
        if let token = KeychainManager().getAccessToken() {
            request.setValue("Bearer \(token)", forHTTPHeaderField: "Authorization")
        }
        
        let (data, response) = try await URLSession.shared.data(for: request)
        
        // If 401, refresh token
        if let httpResponse = response as? HTTPURLResponse,
           httpResponse.statusCode == 401 {
            let newToken = try await authAPI.refreshToken()
            request.setValue("Bearer \(newToken.accessToken)", forHTTPHeaderField: "Authorization")
            let (newData, newResponse) = try await URLSession.shared.data(for: request)
            client.receive(newData, response: newResponse)
        } else {
            client.receive(data, response: response)
        }
    }
}
```

---

## State Machine

```
                    ┌─────────────┐
                    │  Splash      │
                    │  (check      │
                    │   token)     │
                    └──────┬──────┘
                           │
              ┌────────────┴────────────┐
              │                         │
         Has Token                No Token
              │                         │
              ▼                         ▼
    ┌──────────────────┐     ┌──────────────────┐
    │ Validate Token   │     │ Phone Input      │
    │ (POST /profile/get)   │     │ Screen           │
    └────────┬─────────┘     └────────┬─────────┘
             │                        │
     ┌───────┴───────┐               │
     │               │               │
  Valid          Invalid              │
     │               │               │
     ▼               ▼               ▼
  Dashboard    Phone Input    Send OTP API
              (clear tokens)       │
                                   ▼
                          ┌──────────────────┐
                          │ OTP Verification │
                          │ Screen           │
                          └────────┬─────────┘
                                   │
                         ┌─────────┴─────────┐
                         │                   │
                    Verified            Failed
                         │                   │
                         ▼                   ▼
                  Biometric Prompt    Show Error
                         │            (retry)
              ┌──────────┴──────────┐
              │                     │
          Enabled               Skipped
              │                     │
              ▼                     ▼
          Dashboard             Dashboard
```
