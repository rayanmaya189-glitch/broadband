# 03 — Dashboard Module

## Overview

The dashboard is the primary screen after login, providing a quick overview of the customer's internet service status, usage, plan details, and quick actions.

---

## Screen Layout

```
┌─────────────────────────────────┐
│  Good evening, Rahul!     🔔 👤 │
│  Last updated: 2 min ago   ↻    │
├─────────────────────────────────┤
│                                 │
│  ┌───────────────────────────┐  │
│  │  🟢 Online                │  │
│  │  Connected since 6:30 AM  │  │
│  │  Uptime: 15h 32m          │  │
│  └───────────────────────────┘  │
│                                 │
│  ┌───────────────────────────┐  │
│  │  ⬇️ 45.2 Mbps    ⬆️ 12.1 Mbps │
│  │  Current Speed (↓ / ↑)    │  │
│  └───────────────────────────┘  │
│                                 │
│  ┌───────────────────────────┐  │
│  │  📊 Usage This Month      │  │
│  │                           │  │
│  │  ████████████░░░░░ 62%    │  │
│  │  123.4 GB / 200 GB used   │  │
│  │                           │  │
│  │  ⬇️ 118.2 GB  ⬆️ 5.2 GB  │  │
│  └───────────────────────────┘  │
│                                 │
│  ┌───────────────────────────┐  │
│  │  📋 My Plan: AeroXe 100  │  │
│  │  ₹699/month • 100 Mbps   │  │
│  │  Renews: Jul 15, 2026    │  │
│  │  [View Details →]         │  │
│  └───────────────────────────┘  │
│                                 │
│  ── Quick Actions ──────────    │
│                                 │
│  ┌──────┐ ┌──────┐ ┌──────┐    │
│  │ 💰   │ │ 🎫   │ │ 📞   │    │
│  │ Pay  │ │Ticket│ │Help  │    │
│  │ Bill │ │      │ │      │    │
│  └──────┘ └──────┘ └──────┘    │
│  ┌──────┐ ┌──────┐ ┌──────┐    │
│  │ 📄   │ │ 📊   │ │ ⚙️   │    │
│  │Invoice│ │Usage │ │More  │    │
│  │      │ │Graph │ │      │    │
│  └──────┘ └──────┘ └──────┘    │
│                                 │
├─────────────────────────────────┤
│  🏠    💰    🎫    👤    ⚙️     │
│  Home  Bill  Help  Me   More   │
└─────────────────────────────────┘
```

---

## API Endpoints

### Dashboard Summary
```
GET /api/v1/customer/dashboard

Response 200:
{
  "customer": {
    "name": "Rahul Patil",
    "greeting": "Good evening"
  },
  "connection": {
    "status": "online",
    "connected_since": "2026-07-08T06:30:00Z",
    "uptime_hours": 15.53,
    "current_download_mbps": 45.2,
    "current_upload_mbps": 12.1
  },
  "usage": {
    "current_month": {
      "download_gb": 118.2,
      "upload_gb": 5.2,
      "total_gb": 123.4,
      "limit_gb": 200,
      "percentage_used": 61.7,
      "days_remaining": 7
    }
  },
  "subscription": {
    "plan_name": "AeroXe 100",
    "speed_mbps": 100,
    "monthly_price": 699,
    "next_billing_date": "2026-07-15",
    "status": "active"
  },
  "notifications": {
    "unread_count": 3
  }
}
```

### Connection Status (WebSocket)
```
WebSocket: wss://api.aeroxebroadband.com/ws/status?token={access_token}

Server push (every 5s when active):
{
  "type": "status_update",
  "data": {
    "status": "online",
    "download_mbps": 45.2,
    "upload_mbps": 12.1,
    "latency_ms": 12,
    "timestamp": "2026-07-08T22:00:00Z"
  }
}
```

---

## Android Implementation

### DashboardViewModel.kt
```kotlin
@HiltViewModel
class DashboardViewModel @Inject constructor(
    private val dashboardRepository: DashboardRepository,
    private val webSocketManager: WebSocketManager
) : ViewModel() {
    
    private val _uiState = MutableStateFlow(DashboardUiState())
    val uiState: StateFlow<DashboardUiState> = _uiState.asStateFlow()
    
    init {
        loadDashboard()
        observeRealtimeStatus()
    }
    
    private fun loadDashboard() {
        viewModelScope.launch {
            _uiState.update { it.copy(isLoading = true) }
            dashboardRepository.getDashboard()
                .onSuccess { data ->
                    _uiState.update {
                        it.copy(
                            isLoading = false,
                            customerName = data.customer.name,
                            greeting = data.customer.greeting,
                            connectionStatus = data.connection.status,
                            currentSpeed = data.connection,
                            usage = data.usage,
                            subscription = data.subscription,
                            unreadNotifications = data.notifications.unreadCount
                        )
                    }
                }
                .onFailure { error ->
                    _uiState.update { it.copy(isLoading = false, error = error.message) }
                }
        }
    }
    
    private fun observeRealtimeStatus() {
        viewModelScope.launch {
            webSocketManager.connectionStatus.collect { status ->
                _uiState.update {
                    it.copy(
                        connectionStatus = status.status,
                        currentSpeed = SpeedInfo(
                            download = status.downloadMgps,
                            upload = status.uploadMgps
                        ),
                        lastUpdated = status.timestamp
                    )
                }
            }
        }
    }
    
    fun refresh() {
        loadDashboard()
    }
    
    fun clearError() {
        _uiState.update { it.copy(error = null) }
    }
}

data class DashboardUiState(
    val isLoading: Boolean = true,
    val customerName: String = "",
    val greeting: String = "Hello",
    val connectionStatus: String = "offline",
    val currentSpeed: SpeedInfo = SpeedInfo(),
    val usage: UsageInfo = UsageInfo(),
    val subscription: SubscriptionInfo = SubscriptionInfo(),
    val unreadNotifications: Int = 0,
    val lastUpdated: Instant? = null,
    val error: String? = null
)

data class SpeedInfo(
    val download: Double = 0.0,
    val upload: Double = 0.0
)

data class UsageInfo(
    val downloadGb: Double = 0.0,
    val uploadGb: Double = 0.0,
    val totalGb: Double = 0.0,
    val limitGb: Double = 0.0,
    val percentageUsed: Double = 0.0,
    val daysRemaining: Int = 0
)

data class SubscriptionInfo(
    val planName: String = "",
    val speedMbps: Int = 0,
    val monthlyPrice: Int = 0,
    val nextBillingDate: String = "",
    val status: String = ""
)
```

### DashboardScreen.kt
```kotlin
@Composable
fun DashboardScreen(
    viewModel: DashboardViewModel = hiltViewModel(),
    onNavigateToUsage: () -> Unit,
    onNavigateToPlans: () -> Unit,
    onNavigateToTickets: () -> Unit,
    onNavigateToInvoices: () -> Unit,
    onNavigateToProfile: () -> Unit,
    onNavigateToSettings: () -> Unit
) {
    val uiState by viewModel.uiState.collectAsStateWithLifecycle()
    
    if (uiState.isLoading) {
        DashboardSkeleton()
        return
    }
    
    LazyColumn(
        modifier = Modifier.fillMaxSize(),
        contentPadding = PaddingValues(16.dp)
    ) {
        // Greeting header
        item {
            DashboardHeader(
                name = uiState.customerName,
                greeting = uiState.greeting,
                unreadCount = uiState.unreadNotifications,
                onNotificationClick = { /* Navigate to notifications */ }
            )
            Spacer(Modifier.height(16.dp))
        }
        
        // Connection status card
        item {
            ConnectionStatusCard(
                status = uiState.connectionStatus,
                speed = uiState.currentSpeed,
                lastUpdated = uiState.lastUpdated
            )
            Spacer(Modifier.height(12.dp))
        }
        
        // Usage card
        item {
            UsageCard(
                usage = uiState.usage,
                onClick = onNavigateToUsage
            )
            Spacer(Modifier.height(12.dp))
        }
        
        // Plan card
        item {
            PlanCard(
                subscription = uiState.subscription,
                onClick = onNavigateToPlans
            )
            Spacer(Modifier.height(16.dp))
        }
        
        // Quick actions grid
        item {
            QuickActionsGrid(
                onPayBill = { /* Navigate to billing */ },
                onTicket = onNavigateToTickets,
                onHelp = { /* Navigate to support */ },
                onInvoice = onNavigateToInvoices,
                onUsageGraph = onNavigateToUsage,
                onMore = onNavigateToSettings
            )
        }
    }
    
    // Pull to refresh
    PullToRefreshBox(
        isRefreshing = uiState.isLoading,
        onRefresh = viewModel::refresh
    ) { /* content */ }
}
```

---

## iOS Implementation

### DashboardViewModel.swift
```swift
@Observable
class DashboardViewModel {
    var isLoading: Bool = true
    var customerName: String = ""
    var greeting: String = "Hello"
    var connectionStatus: String = "offline"
    var currentSpeed: SpeedInfo = SpeedInfo()
    var usage: UsageInfo = UsageInfo()
    var subscription: SubscriptionInfo = SubscriptionInfo()
    var unreadNotifications: Int = 0
    var lastUpdated: Date?
    var error: String?
    
    private let dashboardRepository: DashboardRepositoryProtocol
    private let webSocketManager: WebSocketManager
    
    init(
        dashboardRepository: DashboardRepositoryProtocol = DashboardRepository(),
        webSocketManager: WebSocketManager = .shared
    ) {
        self.dashboardRepository = dashboardRepository
        self.webSocketManager = webSocketManager
        
        Task {
            await loadDashboard()
            observeRealtimeStatus()
        }
    }
    
    @MainActor
    func loadDashboard() async {
        isLoading = true
        defer { isLoading = false }
        
        do {
            let data = try await dashboardRepository.getDashboard()
            customerName = data.customer.name
            greeting = data.customer.greeting
            connectionStatus = data.connection.status
            currentSpeed = SpeedInfo(
                download: data.connection.currentDownloadMgps,
                upload: data.connection.currentUploadMgps
            )
            usage = UsageInfo(from: data.usage)
            subscription = SubscriptionInfo(from: data.subscription)
            unreadNotifications = data.notifications.unreadCount
        } catch {
            self.error = error.localizedDescription
        }
    }
    
    private func observeRealtimeStatus() {
        webSocketManager.onStatusUpdate = { [weak self] status in
            Task { @MainActor in
                self?.connectionStatus = status.status
                self?.currentSpeed = SpeedInfo(
                    download: status.downloadMgps,
                    upload: status.uploadMgps
                )
                self?.lastUpdated = status.timestamp
            }
        }
    }
}
```

### DashboardView.swift
```swift
struct DashboardView: View {
    @State private var viewModel = DashboardViewModel()
    
    var body: some View {
        NavigationStack {
            ScrollView {
                VStack(spacing: 16) {
                    // Greeting header
                    DashboardHeader(
                        name: viewModel.customerName,
                        greeting: viewModel.greeting,
                        unreadCount: viewModel.unreadNotifications
                    )
                    
                    // Connection status
                    ConnectionStatusCard(
                        status: viewModel.connectionStatus,
                        speed: viewModel.currentSpeed,
                        lastUpdated: viewModel.lastUpdated
                    )
                    
                    // Usage card
                    UsageCard(usage: viewModel.usage)
                    
                    // Plan card
                    PlanCard(subscription: viewModel.subscription)
                    
                    // Quick actions
                    QuickActionsGrid()
                }
                .padding()
            }
            .refreshable {
                await viewModel.loadDashboard()
            }
        }
    }
}
```

---

## Pull-to-Refresh

| Platform | Implementation |
|----------|---------------|
| Android | `PullToRefreshBox` (Material 3) |
| iOS | `.refreshable` modifier on `ScrollView` |

---

## Loading Skeleton

Both platforms show shimmer skeleton placeholders while data loads:

```
┌─────────────────────────────┐
│  ████░░░░░  ████░░░░░       │
│                             │
│  ┌───────────────────────┐  │
│  │  ████████████████████ │  │
│  │  ████████████████████ │  │
│  └───────────────────────┘  │
│                             │
│  ┌───────────────────────┐  │
│  │  ████████████████████ │  │
│  │  ████████████████████ │  │
│  └───────────────────────┘  │
└─────────────────────────────┘
```
