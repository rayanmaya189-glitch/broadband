# 15 — Testing Strategy

## Overview

Comprehensive testing approach covering unit tests, integration tests, UI tests, and end-to-end tests for both Android and iOS platforms.

---

## Testing Pyramid

```
                    ┌───────────┐
                    │    E2E    │  ← 5% (Manual + Appium)
                    │  (3-5 per │
                    │   flow)   │
                  ┌─┴───────────┴─┐
                  │   UI Tests     │  ← 15% (Compose UI / XCUITest)
                  │  (1-2 per      │
                  │   screen)      │
                ┌─┴───────────────┴─┐
                │ Integration Tests  │  ← 30% (Repository + API)
                │ (1-2 per           │
                │   repository)      │
              ┌─┴───────────────────┴─┐
              │    Unit Tests          │  ← 50% (ViewModel, UseCase, Utils)
              │  (3-5 per class)       │
              └───────────────────────┘
```

---

## Unit Tests

### Android (JUnit + MockK + Turbine)

#### AuthViewModelTest.kt
```kotlin
class AuthViewModelTest {
    
    @get:Rule
    val mainDispatcherRule = MainDispatcherRule()
    
    private lateinit var viewModel: AuthViewModel
    private lateinit var authRepository: AuthRepository
    
    @Before
    fun setup() {
        authRepository = mockk()
        viewModel = AuthViewModel(authRepository)
    }
    
    @Test
    fun `sendOtp should update state to OTP step on success`() = runTest {
        // Given
        coEvery { authRepository.sendOtp("+919876543210") } returns Result.success(
            OtpResponse(expiresIn = 300)
        )
        
        // When
        viewModel.onEvent(AuthEvent.PhoneChanged("+919876543210"))
        viewModel.onEvent(AuthEvent.SendOtp)
        
        // Then
        val state = viewModel.uiState.value
        assertFalse(state.isLoading)
        assertEquals(AuthStep.Otp, state.step)
        assertEquals(300, state.otpExpiry)
    }
    
    @Test
    fun `verifyOtp should navigate to dashboard on success`() = runTest {
        // Given
        coEvery { authRepository.verifyOtp("+919876543210", "123456") } returns Result.success(
            AuthTokens(accessToken = "token", refreshToken = "refresh")
        )
        
        // When
        viewModel.onEvent(AuthEvent.PhoneChanged("+919876543210"))
        viewModel.onEvent(AuthEvent.OtpChanged("123456"))
        viewModel.onEvent(AuthEvent.VerifyOtp)
        
        // Then
        val state = viewModel.uiState.value
        assertFalse(state.isLoading)
        assertNull(state.error)
    }
    
    @Test
    fun `verifyOtp should show error on invalid OTP`() = runTest {
        // Given
        coEvery { authRepository.verifyOtp("+919876543210", "000000") } returns Result.failure(
            Exception("Invalid OTP")
        )
        
        // When
        viewModel.onEvent(AuthEvent.PhoneChanged("+919876543210"))
        viewModel.onEvent(AuthEvent.OtpChanged("000000"))
        viewModel.onEvent(AuthEvent.VerifyOtp)
        
        // Then
        val state = viewModel.uiState.value
        assertFalse(state.isLoading)
        assertEquals("Invalid OTP", state.error)
    }
}
```

#### DashboardViewModelTest.kt
```kotlin
class DashboardViewModelTest {
    
    @get:Rule
    val mainDispatcherRule = MainDispatcherRule()
    
    private lateinit var viewModel: DashboardViewModel
    private lateinit var dashboardRepository: DashboardRepository
    private lateinit var webSocketManager: WebSocketManager
    
    @Before
    fun setup() {
        dashboardRepository = mockk()
        webSocketManager = mockk(relaxed = true)
        viewModel = DashboardViewModel(dashboardRepository, webSocketManager)
    }
    
    @Test
    fun `loadDashboard should populate state on success`() = runTest {
        // Given
        val dashboardData = DashboardData(
            customer = CustomerInfo(name = "Rahul", greeting = "Good evening"),
            connection = ConnectionInfo(status = "online", downloadMbps = 45.2, uploadMbps = 12.1),
            usage = UsageInfo(totalGb = 123.4, limitGb = 200.0),
            subscription = SubscriptionInfo(planName = "AeroXe 100", monthlyPrice = 699)
        )
        coEvery { dashboardRepository.getDashboard() } returns Result.success(dashboardData)
        
        // When - ViewModel loads in init
        advanceUntilIdle()
        
        // Then
        val state = viewModel.uiState.value
        assertFalse(state.isLoading)
        assertEquals("Rahul", state.customerName)
        assertEquals("online", state.connectionStatus)
        assertEquals(45.2, state.currentSpeed.download, 0.1)
    }
    
    @Test
    fun `loadDashboard should show error on failure`() = runTest {
        // Given
        coEvery { dashboardRepository.getDashboard() } returns Result.failure(
            Exception("Network error")
        )
        
        // When
        advanceUntilIdle()
        
        // Then
        val state = viewModel.uiState.value
        assertFalse(state.isLoading)
        assertEquals("Network error", state.error)
    }
}
```

#### UsageViewModelTest.kt
```kotlin
class UsageViewModelTest {
    
    @get:Rule
    val mainDispatcherRule = MainDispatcherRule()
    
    private lateinit var viewModel: UsageViewModel
    private lateinit var usageRepository: UsageRepository
    private lateinit var webSocketManager: WebSocketManager
    
    @Before
    fun setup() {
        usageRepository = mockk()
        webSocketManager = mockk(relaxed = true)
        viewModel = UsageViewModel(usageRepository, webSocketManager)
    }
    
    @Test
    fun `loadUsage should populate monthly summary`() = runTest {
        // Given
        val usageData = UsageData(
            currentSpeed = SpeedInfo(download = 45.2, upload = 12.1),
            planLimits = PlanLimits(downloadMbps = 100, uploadMbps = 50, dataCapGb = 200),
            monthlySummary = MonthlySummary(totalGb = 123.4, limitGb = 200.0, percentageUsed = 61.7),
            dailyUsage = emptyList()
        )
        coEvery { usageRepository.getMonthlyUsage() } returns Result.success(usageData)
        
        // When
        advanceUntilIdle()
        
        // Then
        val state = viewModel.uiState.value
        assertFalse(state.isLoading)
        assertEquals(123.4, state.monthlySummary.totalGb, 0.1)
        assertEquals(61.7, state.monthlySummary.percentageUsed, 0.1)
    }
}
```

### iOS (XCTest + XCTestCombine)

```swift
class AuthViewModelTests: XCTestCase {
    
    var viewModel: AuthViewModel!
    var mockRepository: MockAuthRepository!
    
    override func setUp() {
        super.setUp()
        mockRepository = MockAuthRepository()
        viewModel = AuthViewModel(authRepository: mockRepository)
    }
    
    func testSendOtpSuccess() async {
        // Given
        mockRepository.sendOtpResult = .success(OtpResponse(expiresIn: 300))
        
        // When
        viewModel.phone = "+919876543210"
        await viewModel.sendOtp()
        
        // Then
        XCTAssertFalse(viewModel.isLoading)
        XCTAssertEqual(viewModel.step, .otp)
        XCTAssertEqual(viewModel.otpExpiry, 300)
    }
    
    func testVerifyOtpSuccess() async {
        // Given
        mockRepository.verifyOtpResult = .success(
            AuthTokens(accessToken: "token", refreshToken: "refresh")
        )
        
        // When
        viewModel.phone = "+919876543210"
        viewModel.otp = "123456"
        await viewModel.verifyOtp()
        
        // Then
        XCTAssertFalse(viewModel.isLoading)
        XCTAssertNil(viewModel.error)
    }
    
    func testVerifyOtpFailure() async {
        // Given
        mockRepository.verifyOtpResult = .failure(AuthError.invalidOTP)
        
        // When
        viewModel.phone = "+919876543210"
        viewModel.otp = "000000"
        await viewModel.verifyOtp()
        
        // Then
        XCTAssertFalse(viewModel.isLoading)
        XCTAssertNotNil(viewModel.error)
    }
}

// Mock Repository
class MockAuthRepository: AuthRepositoryProtocol {
    var sendOtpResult: Result<OtpResponse, Error> = .failure(NSError(domain: "", code: 0))
    var verifyOtpResult: Result<AuthTokens, Error> = .failure(NSError(domain: "", code: 0))
    
    func sendOtp(phone: String) async throws -> OtpResponse {
        try sendOtpResult.get()
    }
    
    func verifyOtp(phone: String, otp: String) async throws -> AuthTokens {
        try verifyOtpResult.get()
    }
}
```

---

## UI Tests

### Android (Compose Testing)
```kotlin
class DashboardScreenTest {
    
    @get:Rule
    val composeRule = createAndroidComposeRule<MainActivity>()
    
    @Test
    fun dashboardScreen_displaysGreeting() {
        // Given
        val state = DashboardUiState(
            isLoading = false,
            customerName = "Rahul",
            greeting = "Good evening"
        )
        
        // When
        composeRule.setContent {
            AeroXeTheme {
                DashboardContent(state = state, onRefresh = {})
            }
        }
        
        // Then
        composeRule.onNodeWithText("Good evening, Rahul!").assertIsDisplayed()
    }
    
    @Test
    fun dashboardScreen_displaysLoadingSkeleton() {
        // Given
        val state = DashboardUiState(isLoading = true)
        
        // When
        composeRule.setContent {
            AeroXeTheme {
                DashboardContent(state = state, onRefresh = {})
            }
        }
        
        // Then
        composeRule.onNodeWithTag("loading_skeleton").assertIsDisplayed()
    }
    
    @Test
    fun dashboardScreen_displaysConnectionStatus() {
        // Given
        val state = DashboardUiState(
            isLoading = false,
            connectionStatus = "online"
        )
        
        // When
        composeRule.setContent {
            AeroXeTheme {
                DashboardContent(state = state, onRefresh = {})
            }
        }
        
        // Then
        composeRule.onNodeWithText("Online").assertIsDisplayed()
    }
}
```

### iOS (XCUITest)
```swift
class DashboardUITests: XCTestCase {
    
    var app: XCUIApplication!
    
    override func setUp() {
        super.setUp()
        app = XCUIApplication()
        app.launch()
    }
    
    func testDashboard_displaysGreeting() {
        // When
        // (Assumes user is logged in)
        
        // Then
        XCTAssertTrue(app.staticTexts["Good evening, Rahul!"].exists)
    }
    
    func testDashboard_displaysConnectionStatus() {
        // Then
        XCTAssertTrue(app.staticTexts["Online"].exists)
    }
    
    func testDashboard_showsQuickActions() {
        // Then
        XCTAssertTrue(app.buttons["Pay Bill"].exists)
        XCTAssertTrue(app.buttons["Ticket"].exists)
        XCTAssertTrue(app.buttons["Usage"].exists)
    }
}
```

---

## Integration Tests

### Android (Retrofit Mock)
```kotlin
class AuthApiIntegrationTest {
    
    private lateinit var mockWebServer: MockWebServer
    private lateinit var authApi: AuthApi
    
    @Before
    fun setup() {
        mockWebServer = MockWebServer()
        mockWebServer.start()
        
        val retrofit = Retrofit.Builder()
            .baseUrl(mockWebServer.url("/"))
            .addConverterFactory(GsonConverterFactory.create())
            .build()
        
        authApi = retrofit.create(AuthApi::class.java)
    }
    
    @After
    fun teardown() {
        mockWebServer.shutdown()
    }
    
    @Test
    fun `sendOtp returns success response`() = runTest {
        // Given
        val response = """
            {
                "success": true,
                "message": "OTP sent successfully",
                "expires_in": 300
            }
        """.trimIndent()
        mockWebServer.enqueue(MockResponse().setBody(response).setResponseCode(200))
        
        // When
        val result = authApi.sendOtp(SendOtpRequest(phone = "+919876543210"))
        
        // Then
        assertTrue(result.isSuccessful)
        assertEquals(300, result.body()?.expiresIn)
    }
    
    @Test
    fun `verifyOtp returns auth tokens on success`() = runTest {
        // Given
        val response = """
            {
                "success": true,
                "data": {
                    "access_token": "eyJ...",
                    "refresh_token": "eyJ...",
                    "expires_in": 900
                }
            }
        """.trimIndent()
        mockWebServer.enqueue(MockResponse().setBody(response).setResponseCode(200))
        
        // When
        val result = authApi.verifyOtp(VerifyOtpRequest(phone = "+919876543210", otp = "123456"))
        
        // Then
        assertTrue(result.isSuccessful)
        assertNotNull(result.body()?.data?.accessToken)
    }
}
```

---

## Test Commands

### Android
```bash
# Unit tests
./gradlew test

# UI tests
./gradlew connectedAndroidTest

# Code coverage
./gradlew jacocoTestReport

# Lint
./gradlew lint
./gradlew detekt
```

### iOS
```bash
# Unit tests
xcodebuild test -scheme AeroXe -destination 'platform=iOS Simulator,name=iPhone 15'

# UI tests
xcodebuild test -scheme AeroXeUITests -destination 'platform=iOS Simulator,name=iPhone 15'

# Code coverage
xcodebuild test -scheme AeroXe -enableCodeCoverage YES

# SwiftLint
swiftlint lint
```

---

## Coverage Targets

| Layer | Target | Tool |
|-------|--------|------|
| Unit Tests | 80% | JaCoCo (Android) / Xcode Coverage (iOS) |
| Integration Tests | 70% | JaCoCo / Xcode |
| UI Tests | 50% (critical paths) | Compose Testing / XCUITest |
| Overall | 75% | Combined |

---

## Test Data Management

### Android Fixtures
```kotlin
object TestData {
    val sampleCustomer = Customer(
        id = "cust_abc123",
        name = "Rahul Patil",
        phone = "+919876543210",
        email = "rahul@email.com"
    )
    
    val sampleSubscription = Subscription(
        id = "sub_abc123",
        plan = Plan(id = "plan_100", name = "AeroXe 100", monthlyPrice = 699.0),
        status = "active",
        startDate = "2025-07-15",
        nextBillingDate = "2026-07-15"
    )
    
    val sampleInvoice = Invoice(
        id = "inv_abc123",
        invoiceNumber = "INV-2026-0012",
        totalAmount = 824.82,
        status = "pending",
        dueDate = "2026-07-15"
    )
}
```

### iOS Fixtures
```swift
enum TestData {
    static let sampleCustomer = Customer(
        id: "cust_abc123",
        name: "Rahul Patil",
        phone: "+919876543210",
        email: "rahul@email.com"
    )
    
    static let sampleSubscription = Subscription(
        id: "sub_abc123",
        plan: Plan(id: "plan_100", name: "AeroXe 100", monthlyPrice: 699.0),
        status: "active",
        startDate: "2025-07-15",
        nextBillingDate: "2026-07-15"
    )
}
```
