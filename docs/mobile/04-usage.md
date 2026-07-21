# 04 — Internet Usage Module

## Overview

Real-time and historical internet usage monitoring with speed tests, daily/monthly charts, and data breakdown by direction (download/upload).

---

## Screen Layout

### Usage Overview Screen
```
┌─────────────────────────────────┐
│  ← Internet Usage         📊    │
├─────────────────────────────────┤
│                                 │
│  ┌───────────────────────────┐  │
│  │  ⬇️ 45.2 Mbps             │  │
│  │  Current Download Speed   │  │
│  │  ████████████░░░░░░░░░░   │  │
│  │  45% of 100 Mbps plan    │  │
│  └───────────────────────────┘  │
│                                 │
│  ┌───────────────────────────┐  │
│  │  ⬆️ 12.1 Mbps             │  │
│  │  Current Upload Speed     │  │
│  │  █████░░░░░░░░░░░░░░░░░   │  │
│  │  24% of 50 Mbps plan     │  │
│  └───────────────────────────┘  │
│                                 │
│  ── This Month ─────────────    │
│                                 │
│  ┌───────────────────────────┐  │
│  │  📊 Usage Chart           │  │
│  │                           │  │
│  │  ▏                        │  │
│  │  ▏ ▏ ▏ ▏                  │  │
│  │  ▏ ▏ ▏ ▏ ▏ ▏ ▏            │  │
│  │  ▏ ▏ ▏ ▏ ▏ ▏ ▏ ▏ ▏        │  │
│  │  ─────────────────────    │  │
│  │  Mon Tue Wed Thu Fri      │  │
│  └───────────────────────────┘  │
│                                 │
│  ┌──────┐  ┌──────┐            │
│  │ ⬇️   │  │ ⬆️   │            │
│  │118.2 │  │ 5.2  │            │
│  │ GB   │  │ GB   │            │
│  └──────┘  └──────┘            │
│                                 │
│  ┌───────────────────────────┐  │
│  │  📱 Data Cap: 123.4/200GB │  │
│  │  ████████████████░░░░░░  │  │
│  │  7 days remaining         │  │
│  └───────────────────────────┘  │
│                                 │
└─────────────────────────────────┘
```

### Daily Breakdown Screen
```
┌─────────────────────────────────┐
│  ← Daily Breakdown      Jul 8   │
├─────────────────────────────────┤
│                                 │
│  ┌───────────────────────────┐  │
│  │  📅 July 8, 2026         │  │
│  │                           │  │
│  │  ⬇️ 8.2 GB  ⬆️ 0.4 GB    │  │
│  │  Total: 8.6 GB            │  │
│  └───────────────────────────┘  │
│                                 │
│  Hourly Breakdown:              │
│  ┌───────────────────────────┐  │
│  │  12AM ░░░░░░░░░░ 0.1 GB  │  │
│  │   3AM ░░░░░░░░░░ 0.1 GB  │  │
│  │   6AM ▓▓▓▓▓▓░░░░ 2.1 GB  │  │
│  │   9AM ▓▓▓▓▓░░░░░ 1.8 GB  │  │
│  │  12PM ▓▓▓░░░░░░░ 1.2 GB  │  │
│  │   3PM ▓▓▓▓░░░░░░ 1.5 GB  │  │
│  │   6PM ▓▓▓▓▓░░░░░ 1.8 GB  │  │
│  │   9PM ░░░░░░░░░░ ---     │  │
│  └───────────────────────────┘  │
│                                 │
└─────────────────────────────────┘
```

---

## API Endpoints

### Usage Statistics
```
POST /api/v1/customer/usage

Response 200:
{
  "current_speed": {
    "download_mbps": 45.2,
    "upload_mbps": 12.1,
    "latency_ms": 12,
    "timestamp": "2026-07-08T22:00:00Z"
  },
  "plan_limits": {
    "download_mbps": 100,
    "upload_mbps": 50,
    "data_cap_gb": 200
  },
  "monthly_summary": {
    "download_gb": 118.2,
    "upload_gb": 5.2,
    "total_gb": 123.4,
    "limit_gb": 200,
    "percentage_used": 61.7,
    "days_remaining": 7,
    "avg_daily_gb": 15.4
  },
  "daily_usage": [
    {
      "date": "2026-07-08",
      "download_gb": 8.2,
      "upload_gb": 0.4,
      "total_gb": 8.6
    },
    {
      "date": "2026-07-07",
      "download_gb": 12.1,
      "upload_gb": 0.8,
      "total_gb": 12.9
    }
    // ... more days
  ]
}
```

### Hourly Breakdown
```
POST /api/v1/customer/usage/hourly

Response 200:
{
  "date": "2026-07-08",
  "hourly": [
    { "hour": 0, "download_gb": 0.1, "upload_gb": 0.01 },
    { "hour": 1, "download_gb": 0.05, "upload_gb": 0.005 },
    { "hour": 2, "download_gb": 0.05, "upload_gb": 0.005 },
    { "hour": 3, "download_gb": 0.05, "upload_gb": 0.005 },
    { "hour": 4, "download_gb": 0.1, "upload_gb": 0.01 },
    { "hour": 5, "download_gb": 0.2, "upload_gb": 0.02 },
    { "hour": 6, "download_gb": 2.1, "upload_gb": 0.15 },
    { "hour": 7, "download_gb": 1.8, "upload_gb": 0.12 },
    { "hour": 8, "download_gb": 1.2, "upload_gb": 0.08 },
    { "hour": 9, "download_gb": 1.5, "upload_gb": 0.1 },
    { "hour": 10, "download_gb": 0.8, "upload_gb": 0.05 },
    { "hour": 11, "download_gb": 0.5, "upload_gb": 0.03 },
    { "hour": 12, "download_gb": 0.3, "upload_gb": 0.02 },
    { "hour": 13, "download_gb": 0.2, "upload_gb": 0.01 }
  ]
}
```

### Real-time Speed (WebSocket)
```
WebSocket: wss://api.aeroxebroadband.com/ws/usage?token={access_token}

Subscribe:
{ "type": "subscribe", "channels": ["usage"] }

Server push (every 5s):
{
  "type": "usage_update",
  "data": {
    "download_mbps": 45.2,
    "upload_mbps": 12.1,
    "latency_ms": 12,
    "data_used_today_gb": 8.2,
    "timestamp": "2026-07-08T22:00:05Z"
  }
}
```

---

## Android Implementation

### UsageViewModel.kt
```kotlin
@HiltViewModel
class UsageViewModel @Inject constructor(
    private val usageRepository: UsageRepository,
    private val webSocketManager: WebSocketManager
) : ViewModel() {
    
    private val _uiState = MutableStateFlow(UsageUiState())
    val uiState: StateFlow<UsageUiState> = _uiState.asStateFlow()
    
    init {
        loadUsage()
        observeRealtimeSpeed()
    }
    
    private fun loadUsage() {
        viewModelScope.launch {
            _uiState.update { it.copy(isLoading = true) }
            usageRepository.getMonthlyUsage()
                .onSuccess { data ->
                    _uiState.update {
                        it.copy(
                            isLoading = false,
                            currentSpeed = data.currentSpeed,
                            planLimits = data.planLimits,
                            monthlySummary = data.monthlySummary,
                            dailyUsage = data.dailyUsage
                        )
                    }
                }
                .onFailure { error ->
                    _uiState.update { it.copy(isLoading = false, error = error.message) }
                }
        }
    }
    
    fun loadHourlyBreakdown(date: String) {
        viewModelScope.launch {
            usageRepository.getHourlyBreakdown(date)
                .onSuccess { data ->
                    _uiState.update { it.copy(hourlyBreakdown = data.hourly) }
                }
        }
    }
    
    private fun observeRealtimeSpeed() {
        viewModelScope.launch {
            webSocketManager.usageUpdates.collect { update ->
                _uiState.update {
                    it.copy(
                        currentSpeed = SpeedInfo(
                            download = update.downloadMbps,
                            upload = update.uploadMbps,
                            latency = update.latencyMs
                        )
                    )
                }
            }
        }
    }
}

data class UsageUiState(
    val isLoading: Boolean = true,
    val currentSpeed: SpeedInfo = SpeedInfo(),
    val planLimits: PlanLimits = PlanLimits(),
    val monthlySummary: MonthlySummary = MonthlySummary(),
    val dailyUsage: List<DailyUsage> = emptyList(),
    val hourlyBreakdown: List<HourlyUsage> = emptyList(),
    val selectedDate: String = "",
    val error: String? = null
)
```

### UsageScreen.kt (Chart)
```kotlin
@Composable
fun UsageChart(
    dailyUsage: List<DailyUsage>,
    modifier: Modifier = Modifier
) {
    val chartData = dailyUsage.map { daily ->
        BarEntry(
            x = daily.date.toFloat(),
            y = floatArrayOf(daily.downloadGb.toFloat(), daily.uploadGb.toFloat())
        )
    }
    
    // Using a custom Compose chart implementation
    // Or MPAndroidChart via AndroidView
    BarChart(
        data = chartData,
        colors = listOf(
            MaterialTheme.colorScheme.primary,
            MaterialTheme.colorScheme.secondary
        ),
        labels = dailyUsage.map { it.date.takeLast(5) }
    )
}
```

### SpeedIndicator.kt
```kotlin
@Composable
fun SpeedIndicator(
    speed: Double,
    maxSpeed: Double,
    label: String,
    icon: ImageVector,
    modifier: Modifier = Modifier
) {
    val progress by animateFloatAsState(
        targetValue = (speed / maxSpeed).toFloat().coerceIn(0f, 1f),
        animationSpec = tween(1000)
    )
    
    Column(modifier = modifier) {
        Row(verticalAlignment = Alignment.CenterVertically) {
            Icon(icon, contentDescription = label)
            Spacer(Modifier.width(8.dp))
            Text(
                text = "%.1f Mbps".format(speed),
                style = MaterialTheme.typography.headlineMedium,
                fontWeight = FontWeight.Bold
            )
        }
        Spacer(Modifier.height(4.dp))
        Text(
            text = label,
            style = MaterialTheme.typography.bodyMedium,
            color = MaterialTheme.colorScheme.onSurfaceVariant
        )
        Spacer(Modifier.height(8.dp))
        LinearProgressIndicator(
            progress = { progress },
            modifier = Modifier
                .fillMaxWidth()
                .height(8.dp)
                .clip(RoundedCornerShape(4.dp)),
            color = when {
                progress > 0.8f -> MaterialTheme.colorScheme.error
                progress > 0.5f -> MaterialTheme.colorScheme.tertiary
                else -> MaterialTheme.colorScheme.primary
            }
        )
        Spacer(Modifier.height(4.dp))
        Text(
            text = "%.0f%% of %d Mbps plan".format(progress * 100, maxSpeed.toInt()),
            style = MaterialTheme.typography.bodySmall,
            color = MaterialTheme.colorScheme.onSurfaceVariant
        )
    }
}
```

---

## iOS Implementation

### UsageViewModel.swift
```swift
@Observable
class UsageViewModel {
    var isLoading: Bool = true
    var currentSpeed: SpeedInfo = SpeedInfo()
    var planLimits: PlanLimits = PlanLimits()
    var monthlySummary: MonthlySummary = MonthlySummary()
    var dailyUsage: [DailyUsage] = []
    var hourlyBreakdown: [HourlyUsage] = []
    var selectedDate: String = ""
    var error: String?
    
    private let usageRepository: UsageRepositoryProtocol
    private let webSocketManager: WebSocketManager
    
    init(
        usageRepository: UsageRepositoryProtocol = UsageRepository(),
        webSocketManager: WebSocketManager = .shared
    ) {
        self.usageRepository = usageRepository
        self.webSocketManager = webSocketManager
        
        Task {
            await loadUsage()
            observeRealtimeSpeed()
        }
    }
    
    @MainActor
    func loadUsage() async {
        isLoading = true
        defer { isLoading = false }
        
        do {
            let data = try await usageRepository.getMonthlyUsage()
            currentSpeed = data.currentSpeed
            planLimits = data.planLimits
            monthlySummary = data.monthlySummary
            dailyUsage = data.dailyUsage
        } catch {
            self.error = error.localizedDescription
        }
    }
    
    private func observeRealtimeSpeed() {
        webSocketManager.onUsageUpdate = { [weak self] update in
            Task { @MainActor in
                self?.currentSpeed = SpeedInfo(
                    download: update.downloadMbps,
                    upload: update.uploadMbps,
                    latency: update.latencyMs
                )
            }
        }
    }
}
```

### UsageView.swift (Charts)
```swift
import Charts

struct UsageChartView: View {
    let dailyUsage: [DailyUsage]
    
    var body: some View {
        Chart {
            ForEach(dailyUsage) { day in
                BarMark(
                    x: .value("Date", day.date, unit: .day),
                    y: .value("Download (GB)", day.downloadGb),
                    stacking: .standard
                )
                .foregroundStyle(by: .value("Type", "Download"))
                
                BarMark(
                    x: .value("Date", day.date, unit: .day),
                    y: .value("Upload (GB)", day.uploadGb),
                    stacking: .standard
                )
                .foregroundStyle(by: .value("Type", "Upload"))
            }
        }
        .chartForegroundStyleScale([
            "Download": .blue,
            "Upload": .green
        ])
        .frame(height: 200)
    }
}
```

### SpeedIndicator.swift
```swift
struct SpeedIndicator: View {
    let speed: Double
    let maxSpeed: Double
    let label: String
    let icon: String
    
    private var progress: Double {
        min(speed / maxSpeed, 1.0)
    }
    
    var body: some View {
        VStack(alignment: .leading, spacing: 8) {
            HStack {
                Image(systemName: icon)
                Text(String(format: "%.1f Mbps", speed))
                    .font(.title2)
                    .fontWeight(.bold)
            }
            
            Text(label)
                .font(.subheadline)
                .foregroundStyle(.secondary)
            
            ProgressView(value: progress)
                .tint(progressColor)
                .frame(height: 8)
                .clipShape(Capsule())
            
            Text(String(format: "%.0f%% of %d Mbps plan", progress * 100, Int(maxSpeed)))
                .font(.caption)
                .foregroundStyle(.secondary)
        }
    }
    
    private var progressColor: Color {
        if progress > 0.8 { return .red }
        if progress > 0.5 { return .orange }
        return .green
    }
}
```

---

## Data Cap Warning

When usage exceeds 80% of data cap:

```
┌─────────────────────────────┐
│  ⚠️ Data Cap Warning        │
│                             │
│  You've used 82% of your   │
│  200 GB monthly data cap.  │
│                             │
│  Consider upgrading your   │
│  plan for unlimited data.  │
│                             │
│  [Upgrade Plan →]           │
└─────────────────────────────┘
```

Trigger: `monthlySummary.percentageUsed >= 80`
