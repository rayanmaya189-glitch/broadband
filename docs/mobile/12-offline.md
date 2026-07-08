# 12 — Offline Support Module

## Overview

Both apps support offline viewing via local databases (Room on Android, SwiftData on iOS) with background synchronization using WorkManager (Android) and BackgroundTasks (iOS). Cached data provides a degraded but functional experience when connectivity is lost.

---

## Cache Strategy

| Data Type | Cache Duration | Sync Trigger | Priority |
|-----------|---------------|--------------|----------|
| Plan Details | 24 hours | App open, background refresh | High |
| Subscription | 24 hours | App open, background refresh | High |
| Invoice List | 1 hour | Pull-to-refresh, background refresh | Medium |
| Usage Data | 5 minutes | Polling, WebSocket, background refresh | High |
| Profile | 24 hours | App open, background refresh | Medium |
| Notifications | 15 minutes | FCM, background refresh | Medium |
| Settings | 24 hours | App open, background refresh | Low |
| Tickets | 30 minutes | Pull-to-refresh, background refresh | Medium |

---

## Android Implementation

### Room Database

#### AppDatabase.kt
```kotlin
@Database(
    entities = [
        CachedPlan::class,
        CachedSubscription::class,
        CachedInvoice::class,
        CachedUsage::class,
        CachedProfile::class,
        CachedNotification::class,
        CachedTicket::class,
        SyncMetadata::class
    ],
    version = 1,
    exportSchema = true
)
@TypeConverters(Converters::class)
abstract class AppDatabase : RoomDatabase() {
    abstract fun planDao(): PlanDao
    abstract fun subscriptionDao(): SubscriptionDao
    abstract fun invoiceDao(): InvoiceDao
    abstract fun usageDao(): UsageDao
    abstract fun profileDao(): ProfileDao
    abstract fun notificationDao(): NotificationDao
    abstract fun ticketDao(): TicketDao
    abstract fun syncMetadataDao(): SyncMetadataDao
}
```

#### CachedSubscription.kt
```kotlin
@Entity(tableName = "cached_subscriptions")
data class CachedSubscription(
    @PrimaryKey val id: String,
    val planId: String,
    val planName: String,
    val speedDownloadMbps: Int,
    val speedUploadMbps: Int,
    val monthlyPrice: Double,
    val status: String,
    val startDate: String,
    val nextBillingDate: String,
    val autoRenew: Boolean,
    val cachedAt: Long = System.currentTimeMillis()
)

@Entity(tableName = "sync_metadata")
data class SyncMetadata(
    @PrimaryKey val key: String,
    val lastSyncedAt: Long,
    val expiryMs: Long
)
```

#### PlanDao.kt
```kotlin
@Dao
interface PlanDao {
    @Query("SELECT * FROM cached_plans ORDER BY monthly_price ASC")
    fun getAllPlans(): Flow<List<CachedPlan>>
    
    @Insert(onConflict = OnConflictStrategy.REPLACE)
    suspend fun insertAll(plans: List<CachedPlan>)
    
    @Query("DELETE FROM cached_plans")
    suspend fun clearAll()
}

@Dao
interface SubscriptionDao {
    @Query("SELECT * FROM cached_subscriptions LIMIT 1")
    fun getCurrentSubscription(): Flow<CachedSubscription?>
    
    @Insert(onConflict = OnConflictStrategy.REPLACE)
    suspend fun insert(subscription: CachedSubscription)
    
    @Query("DELETE FROM cached_subscriptions")
    suspend fun clear()
}
```

#### SyncMetadataDao.kt
```kotlin
@Dao
interface SyncMetadataDao {
    @Query("SELECT * FROM sync_metadata WHERE key = :key")
    suspend fun get(key: String): SyncMetadata?
    
    @Insert(onConflict = OnConflictStrategy.REPLACE)
    suspend fun upsert(metadata: SyncMetadata)
    
    @Query("DELETE FROM sync_metadata WHERE key = :key")
    suspend fun delete(key: String)
}
```

### Offline Repository Pattern

#### CachedRepository.kt
```kotlin
abstract class CachedRepository<T>(
    private val database: AppDatabase,
    private val cacheKey: String,
    private val cacheDurationMs: Long
) {
    protected abstract suspend fun fetchFromNetwork(): Result<T>
    protected abstract suspend fun saveToCache(data: T)
    protected abstract suspend fun getFromCache(): Flow<T>
    protected abstract suspend fun isCacheValid(): Boolean
    
    suspend fun getData(forceRefresh: Boolean = false): Flow<T> {
        return flow {
            // Emit cached data first
            val cached = getFromCache()
            cached.collect { data ->
                emit(data)
            }
            
            // If cache is stale or force refresh, fetch from network
            if (forceRefresh || !isCacheValid()) {
                fetchFromNetwork()
                    .onSuccess { data ->
                        saveToCache(data)
                        database.syncMetadataDao().upsert(
                            SyncMetadata(
                                key = cacheKey,
                                lastSyncedAt = System.currentTimeMillis(),
                                expiryMs = cacheDurationMs
                            )
                        )
                    }
            }
        }
    }
    
    private suspend fun isCacheValid(): Boolean {
        val metadata = database.syncMetadataDao().get(cacheKey) ?: return false
        return System.currentTimeMillis() - metadata.lastSyncedAt < metadata.expiryMs
    }
}
```

### WorkManager Background Sync

#### SyncWorker.kt
```kotlin
@HiltWorker
class SyncWorker @AssistedInject constructor(
    @Assisted appContext: Context,
    @Assisted workerParams: WorkerParameters,
    private val dashboardRepository: DashboardRepository,
    private val notificationsRepository: NotificationsRepository,
    private val subscriptionRepository: SubscriptionRepository
) : CoroutineWorker(appContext, workerParams) {
    
    override suspend fun doWork(): Result {
        return try {
            // Sync dashboard data
            dashboardRepository.refreshCache()
            
            // Sync notifications
            notificationsRepository.refreshCache()
            
            // Sync subscription
            subscriptionRepository.refreshCache()
            
            Result.success()
        } catch (e: Exception) {
            if (runAttemptCount < 3) {
                Result.retry()
            } else {
                Result.failure()
            }
        }
    }
}
```

#### SyncScheduler.kt
```kotlin
class SyncScheduler @Inject constructor(
    private val workManager: WorkManager
) {
    fun schedulePeriodicSync() {
        val constraints = Constraints.Builder()
            .setRequiredNetworkType(NetworkType.CONNECTED)
            .build()
        
        val syncRequest = PeriodicWorkRequestBuilder<SyncWorker>(
            15, TimeUnit.MINUTES
        )
            .setConstraints(constraints)
            .setBackoffCriteria(
                BackoffPolicy.EXPONENTIAL,
                WorkRequest.MIN_BACKOFF_MILLIS,
                TimeUnit.MILLISECONDS
            )
            .build()
        
        workManager.enqueueUniquePeriodicWork(
            "aeroxe_sync",
            ExistingPeriodicWorkPolicy.KEEP,
            syncRequest
        )
    }
    
    fun scheduleImmediateSync() {
        val constraints = Constraints.Builder()
            .setRequiredNetworkType(NetworkType.CONNECTED)
            .build()
        
        val syncRequest = OneTimeWorkRequestBuilder<SyncWorker>()
            .setConstraints(constraints)
            .build()
        
        workManager.enqueue(syncRequest)
    }
}
```

### Connectivity Monitor

#### ConnectivityObserver.kt
```kotlin
class ConnectivityObserver @Inject constructor(
    @ApplicationContext private val context: Context
) {
    private val connectivityManager = context.getSystemService<ConnectivityManager>()
    
    val isConnected: StateFlow<Boolean> = callbackFlow {
        val callback = object : ConnectivityManager.NetworkCallback() {
            override fun onAvailable(network: Network) {
                trySend(true)
            }
            override fun onLost(network: Network) {
                trySend(false)
            }
        }
        
        val request = NetworkRequest.Builder()
            .addCapability(NetworkCapabilities.NET_CAPABILITY_INTERNET)
            .build()
        
        connectivityManager?.registerNetworkCallback(request, callback)
        
        // Initial state
        trySend(isCurrentlyConnected())
        
        awaitClose { connectivityManager?.unregisterNetworkCallback(callback) }
    }.stateIn(
        scope = CoroutineScope(Dispatchers.Default),
        started = SharingStarted.WhileSubscribed(5000),
        initialValue = isCurrentlyConnected()
    )
    
    private fun isCurrentlyConnected(): Boolean {
        val network = connectivityManager?.activeNetwork ?: return false
        val capabilities = connectivityManager.getNetworkCapabilities(network) ?: return false
        return capabilities.hasCapability(NetworkCapabilities.NET_CAPABILITY_INTERNET)
    }
}
```

---

## iOS Implementation

### SwiftData Models

#### CachedModels.swift
```swift
import SwiftData

@Model
class CachedSubscriptionModel {
    var id: String
    var planId: String
    var planName: String
    var speedDownloadMbps: Int
    var speedUploadMbps: Int
    var monthlyPrice: Double
    var status: String
    var startDate: String
    var nextBillingDate: String
    var autoRenew: Bool
    var cachedAt: Date
    
    init(from subscription: Subscription) {
        self.id = subscription.id
        self.planId = subscription.plan.id
        self.planName = subscription.plan.name
        self.speedDownloadMbps = subscription.plan.speedDownloadMbps
        self.speedUploadMbps = subscription.plan.speedUploadMbps
        self.monthlyPrice = subscription.plan.monthlyPrice
        self.status = subscription.status
        self.startDate = subscription.startDate
        self.nextBillingDate = subscription.nextBillingDate
        self.autoRenew = subscription.autoRenew
        self.cachedAt = Date()
    }
}

@Model
class SyncMetadataModel {
    var key: String
    var lastSyncedAt: Date
    var expiryMs: Int64
    
    init(key: String, expiryMs: Int64) {
        self.key = key
        self.lastSyncedAt = Date()
        self.expiryMs = expiryMs
    }
    
    var isExpired: Bool {
        Date().timeIntervalSince(lastSyncedAt) * 1000 > Double(expiryMs)
    }
}
```

### Background Tasks (iOS)

#### BackgroundSyncManager.swift
```swift
import BackgroundTasks

class BackgroundSyncManager {
    static let shared = BackgroundSyncManager()
    
    private let taskIdentifier = "com.aeroxebroadband.sync"
    
    func register() {
        BGTaskScheduler.shared.register(
            forTaskWithIdentifier: taskIdentifier,
            using: nil
        ) { task in
            self.handleSync(task: task as! BGAppRefreshTask)
        }
    }
    
    func schedule() {
        let request = BGAppRefreshTaskRequest(identifier: taskIdentifier)
        request.earliestBeginDate = Date(timeIntervalSinceNow: 15 * 60) // 15 minutes
        try? BGTaskScheduler.shared.submit(request)
    }
    
    private func handleSync(task: BGAppRefreshTask) {
        let operation = SyncOperation()
        
        task.expirationHandler = {
            operation.cancel()
        }
        
        operation.completionBlock = {
            task.setTaskCompleted(success: !operation.isCancelled)
            self.schedule() // Reschedule
        }
        
        OperationQueue.main.addOperation(operation)
    }
}

class SyncOperation: Operation {
    override func main() {
        guard !isCancelled else { return }
        
        Task {
            // Sync data in background
            try? await DashboardRepository.shared.refreshCache()
            try? await NotificationsRepository.shared.refreshCache()
            try? await SubscriptionRepository.shared.refreshCache()
        }
    }
}
```

### Connectivity Monitor (iOS)

#### NetworkMonitor.swift
```swift
import Network

class NetworkMonitor {
    static let shared = NetworkMonitor()
    
    private let monitor = NWPathMonitor()
    private let queue = DispatchQueue(label: "NetworkMonitor")
    
    @Published var isConnected: Bool = true
    @Published var connectionType: ConnectionType = .unknown
    
    enum ConnectionType {
        case wifi, cellular, ethernet, unknown
    }
    
    func startMonitoring() {
        monitor.start(queue: queue) { path in
            DispatchQueue.main.async {
                self.isConnected = path.status == .satisfied
                self.connectionType = self.getConnectionType(path)
            }
        }
    }
    
    func stopMonitoring() {
        monitor.cancel()
    }
    
    private func getConnectionType(_ path: NWPath) -> ConnectionType {
        if path.usesInterfaceType(.wifi) { return .wifi }
        if path.usesInterfaceType(.cellular) { return .cellular }
        if path.usesInterfaceType(.wiredEthernet) { return .ethernet }
        return .unknown
    }
}
```

---

## Offline UI Indicators

### No Connection Banner
```
┌─────────────────────────────────┐
│  ⚠️ No internet connection      │
│  Showing cached data            │
├─────────────────────────────────┤
│  ... content ...                │
└─────────────────────────────────┘
```

### Android
```kotlin
@Composable
fun OfflineBanner(isConnected: Boolean) {
    AnimatedVisibility(visible = !isConnected) {
        Surface(
            color = MaterialTheme.colorScheme.errorContainer,
            modifier = Modifier.fillMaxWidth()
        ) {
            Row(
                modifier = Modifier.padding(12.dp),
                verticalAlignment = Alignment.CenterVertically
            ) {
                Icon(Icons.Default.Warning, contentDescription = null)
                Spacer(Modifier.width(8.dp))
                Text("No internet connection. Showing cached data.")
            }
        }
    }
}
```

### iOS
```swift
struct OfflineBanner: View {
    let isConnected: Bool
    
    var body: some View {
        if !isConnected {
            HStack {
                Image(systemName: "wifi.slash")
                Text("No internet connection. Showing cached data.")
            }
            .padding(12)
            .frame(maxWidth: .infinity)
            .background(.red.opacity(0.1))
            .transition(.move(edge: .top))
        }
    }
}
```

---

## Cache Invalidation

| Event | Action |
|-------|--------|
| App launch | Check cache freshness, sync if stale |
| Pull-to-refresh | Force refresh all visible data |
| WebSocket reconnect | Sync real-time data |
| Login | Full cache sync |
| Logout | Clear all cache |
| Plan change | Invalidate subscription + invoice cache |
| Payment | Invalidate invoice cache |
| Profile update | Invalidate profile cache |
