# 08 — Notifications Module

## Overview

Push notifications via FCM (Android) and APNs (iOS), with an in-app notification center showing notification history. Both platforms share identical notification payloads and API contracts.

---

## Screen Layout

### Notification Center Screen
```
┌─────────────────────────────────┐
│  ← Notifications          🔔    │
│                           Mark all read │
├─────────────────────────────────┤
│                                 │
│  ── Today ──────────────────    │
│                                 │
│  ┌───────────────────────────┐  │
│  │  🔴 Bill Due Tomorrow     │  │
│  │  Your invoice of ₹824.82  │  │
│  │  is due on Jul 15.        │  │
│  │  [Pay Now]                │  │
│  │  2 hours ago              │  │
│  └───────────────────────────┘  │
│                                 │
│  ┌───────────────────────────┐  │
│  │  🟡 Speed Alert           │  │
│  │  Your download speed has  │  │
│  │  dropped below 50 Mbps.   │  │
│  │  5 hours ago              │  │
│  └───────────────────────────┘  │
│                                 │
│  ── Yesterday ──────────────    │
│                                 │
│  ┌───────────────────────────┐  │
│  │  🟢 Payment Received      │  │
│  │  Payment of ₹699 received │  │
│  │  for INV-2026-0011.       │  │
│  │  Yesterday, 3:45 PM       │  │
│  └───────────────────────────┘  │
│                                 │
│  ┌───────────────────────────┐  │
│  │  🔵 Plan Upgrade          │  │
│  │  Your plan has been       │  │
│  │  upgraded to AeroXe 100.  │  │
│  │  Yesterday, 10:20 AM      │  │
│  └───────────────────────────┘  │
│                                 │
└─────────────────────────────────┘
```

### Notification Badge
```
Dashboard header shows unread count:
┌──────────────────────┐
│  Good evening, Rahul! │  🔔3  │
└──────────────────────┘
```

---

## Notification Types

| Type | Title Template | Body Template | Action |
|------|---------------|---------------|--------|
| `bill_due` | Bill Due Tomorrow | Your invoice of ₹{amount} is due on {date}. | Pay Now |
| `bill_overdue` | Payment Overdue | Your invoice of ₹{amount} is {days} days overdue. | Pay Now |
| `payment_received` | Payment Received | Payment of ₹{amount} received for {invoice}. | View |
| `plan_changed` | Plan Updated | Your plan has been changed to {plan_name}. | View |
| `speed_alert` | Speed Alert | Your download speed has dropped below {speed} Mbps. | View |
| `usage_warning` | Data Usage Warning | You've used {percent}% of your data cap. | Upgrade |
| `ticket_update` | Ticket Update | {agent_name} replied to ticket #{number}. | View |
| `ticket_resolved` | Ticket Resolved | Ticket #{number} has been resolved. | View |
| `installation_scheduled` | Installation Scheduled | Your installation is scheduled for {date}. | View |
| `installation_complete` | Installation Complete | Your installation is complete. Welcome! | View |
| `maintenance` | Maintenance Notice | Scheduled maintenance on {date} from {start} to {end}. | View |

---

## API Endpoints

### Get Notifications
```
POST /api/v1/customer/notifications/list

Response 200:
{
  "notifications": [
    {
      "id": "notif_abc123",
      "type": "bill_due",
      "title": "Bill Due Tomorrow",
      "body": "Your invoice of ₹824.82 is due on Jul 15.",
      "data": {
        "invoice_id": "inv_abc123",
        "amount": 824.82,
        "due_date": "2026-07-15"
      },
      "is_read": false,
      "created_at": "2026-07-08T10:00:00Z"
    }
  ],
  "unread_count": 3,
  "pagination": { "page": 1, "limit": 20, "total": 15 }
}
```

### Mark as Read
```
PATCH /api/v1/customer/notifications/:id/read

Response 200: { "success": true }
```

### Mark All Read
```
PATCH /api/v1/customer/notifications/read-all

Response 200: { "success": true, "count": 3 }
```

### Register Device Token
```
POST /api/v1/customer/devices/fcm  (Android)
POST /api/v1/customer/devices/apns (iOS)

Request:
{
  "token": "fcm_token_xxx",
  "platform": "android",
  "device_name": "Pixel 8",
  "os_version": "14"
}

Response 200: { "success": true, "device_id": "dev_abc123" }
```

---

## Android Implementation

### FCM Setup
```kotlin
@AndroidEntryPoint
class FCMService : FirebaseMessagingService() {
    
    @Inject lateinit var notificationHelper: NotificationHelper
    @Inject lateinit var deviceRepository: DeviceRepository
    
    override fun onNewToken(token: String) {
        super.onNewToken(token)
        // Register new token with backend
        CoroutineScope(Dispatchers.IO).launch {
            deviceRepository.registerDevice(
                token = token,
                platform = "android",
                deviceName = Build.MODEL,
                osVersion = Build.VERSION.RELEASE
            )
        }
    }
    
    override fun onMessageReceived(message: RemoteMessage) {
        super.onMessageReceived(message)
        
        val data = message.data
        val title = message.notification?.title ?: data["title"] ?: ""
        val body = message.notification?.body ?: data["body"] ?: ""
        val type = data["type"] ?: "general"
        
        // Show local notification
        notificationHelper.showNotification(
            title = title,
            body = body,
            type = type,
            data = data
        )
    }
}
```

### NotificationHelper.kt
```kotlin
class NotificationHelper @Inject constructor(
    @ApplicationContext private val context: Context
) {
    private val notificationManager = context.getSystemService<NotificationManager>()
    
    init {
        createNotificationChannels()
    }
    
    private fun createNotificationChannels() {
        val channels = listOf(
            NotificationChannel("billing", "Billing", NotificationManager.IMPORTANCE_HIGH),
            NotificationChannel("usage", "Usage Alerts", NotificationManager.IMPORTANCE_DEFAULT),
            NotificationChannel("tickets", "Support Tickets", NotificationManager.IMPORTANCE_HIGH),
            NotificationChannel("general", "General", NotificationManager.IMPORTANCE_LOW)
        )
        notificationManager?.createNotificationChannels(channels)
    }
    
    fun showNotification(
        title: String,
        body: String,
        type: String,
        data: Map<String, String>
    ) {
        val channelId = when (type) {
            "bill_due", "bill_overdue", "payment_received" -> "billing"
            "speed_alert", "usage_warning" -> "usage"
            "ticket_update", "ticket_resolved" -> "tickets"
            else -> "general"
        }
        
        // Create deep link intent
        val intent = createDeepLinkIntent(type, data)
        
        val notification = NotificationCompat.Builder(context, channelId)
            .setSmallIcon(R.drawable.ic_notification)
            .setContentTitle(title)
            .setContentText(body)
            .setPriority(NotificationCompat.PRIORITY_HIGH)
            .setContentIntent(PendingIntent.getActivity(
                context, 0, intent,
                PendingIntent.FLAG_UPDATE_CURRENT or PendingIntent.FLAG_IMMUTABLE
            ))
            .setAutoCancel(true)
            .build()
        
        notificationManager?.notify(System.currentTimeMillis().toInt(), notification)
    }
    
    private fun createDeepLinkIntent(type: String, data: Map<String, String>): Intent {
        val navController = // Get NavController
        val deepLink = when (type) {
            "bill_due", "bill_overdue" -> "aeroxe://invoices/${data["invoice_id"]}"
            "ticket_update" -> "aeroxe://tickets/${data["ticket_id"]}"
            "usage_warning" -> "aeroxe://usage"
            else -> "aeroxe://dashboard"
        }
        return Intent(Intent.ACTION_VIEW, Uri.parse(deepLink))
    }
}
```

### NotificationCenterViewModel.kt
```kotlin
@HiltViewModel
class NotificationCenterViewModel @Inject constructor(
    private val notificationsRepository: NotificationsRepository
) : ViewModel() {
    
    private val _uiState = MutableStateFlow(NotificationCenterUiState())
    val uiState: StateFlow<NotificationCenterUiState> = _uiState.asStateFlow()
    
    init {
        loadNotifications()
    }
    
    private fun loadNotifications() {
        viewModelScope.launch {
            notificationsRepository.getNotifications()
                .onSuccess { result ->
                    _uiState.update {
                        it.copy(
                            notifications = result.notifications,
                            unreadCount = result.unreadCount
                        )
                    }
                }
        }
    }
    
    fun markAsRead(notificationId: String) {
        viewModelScope.launch {
            notificationsRepository.markAsRead(notificationId)
            _uiState.update {
                it.copy(
                    notifications = it.notifications.map { n ->
                        if (n.id == notificationId) n.copy(isRead = true) else n
                    },
                    unreadCount = (it.unreadCount - 1).coerceAtLeast(0)
                )
            }
        }
    }
    
    fun markAllAsRead() {
        viewModelScope.launch {
            notificationsRepository.markAllAsRead()
            _uiState.update {
                it.copy(
                    notifications = it.notifications.map { n -> n.copy(isRead = true) },
                    unreadCount = 0
                )
            }
        }
    }
}

data class NotificationCenterUiState(
    val notifications: List<Notification> = emptyList(),
    val unreadCount: Int = 0,
    val isLoading: Boolean = false,
    val error: String? = null
)
```

---

## iOS Implementation

### Push Notification Setup (AppDelegate)
```swift
import FirebaseMessaging

class AppDelegate: NSObject, UIApplicationDelegate, MessagingDelegate, UNUserNotificationCenterDelegate {
    
    func application(
        _ application: UIApplication,
        didFinishLaunchingWithOptions launchOptions: [UIApplication.LaunchOptionsKey: Any]? = nil
    ) -> Bool {
        // Request push notification permission
        let center = UNUserNotificationCenter.current()
        center.delegate = self
        center.requestAuthorization(options: [.alert, .badge, .sound]) { granted, error in
            if granted {
                DispatchQueue.main.async {
                    application.registerForRemoteNotifications()
                }
            }
        }
        
        // Setup Firebase Messaging
        Messaging.messaging().delegate = self
        
        return true
    }
    
    func application(
        _ application: UIApplication,
        didRegisterForRemoteNotificationsWithDeviceToken deviceToken: Data
    ) {
        Messaging.messaging().apnsToken = deviceToken
    }
    
    func messaging(
        _ messaging: Messaging,
        didReceiveRegistrationToken fcmToken: String?
    ) {
        guard let token = fcmToken else { return }
        // Register with backend
        Task {
            try? await deviceRepository.registerDevice(
                token: token,
                platform: "ios",
                deviceName: UIDevice.current.name,
                osVersion: UIDevice.current.systemVersion
            )
        }
    }
    
    func userNotificationCenter(
        _ center: UNUserNotificationCenter,
        willPresent notification: UNNotification,
        withCompletionHandler completionHandler: @escaping (UNNotificationPresentationOptions) -> Void
    ) {
        // Show notification even when app is in foreground
        completionHandler([.banner, .badge, .sound])
    }
    
    func userNotificationCenter(
        _ center: UNUserNotificationCenter,
        didReceive response: UNNotificationResponse,
        withCompletionHandler completionHandler: @escaping () -> Void
    ) {
        // Handle notification tap → deep link
        let data = response.notification.request.content.userInfo
        handleNotificationTap(data: data as? [String: Any] ?? [:])
        completionHandler()
    }
    
    private func handleNotificationTap(data: [String: Any]) {
        guard let type = data["type"] as? String else { return }
        // Navigate based on type
        switch type {
        case "bill_due", "bill_overdue":
            NavigationManager.shared.navigate(to: .invoiceDetail(id: data["invoice_id"] as? String ?? ""))
        case "ticket_update":
            NavigationManager.shared.navigate(to: .ticketDetail(id: data["ticket_id"] as? String ?? ""))
        default:
            NavigationManager.shared.navigate(to: .dashboard)
        }
    }
}
```

### NotificationCenterViewModel.swift
```swift
@Observable
class NotificationCenterViewModel {
    var notifications: [NotificationItem] = []
    var unreadCount: Int = 0
    var isLoading: Bool = false
    var error: String?
    
    private let notificationsRepository: NotificationsRepositoryProtocol
    
    init(notificationsRepository: NotificationsRepositoryProtocol = NotificationsRepository()) {
        self.notificationsRepository = notificationsRepository
        Task { await loadNotifications() }
    }
    
    @MainActor
    func loadNotifications() async {
        isLoading = true
        defer { isLoading = false }
        
        do {
            let result = try await notificationsRepository.getNotifications()
            notifications = result.notifications
            unreadCount = result.unreadCount
        } catch {
            self.error = error.localizedDescription
        }
    }
    
    func markAsRead(id: String) async {
        try? await notificationsRepository.markAsRead(id: id)
        if let index = notifications.firstIndex(where: { $0.id == id }) {
            notifications[index].isRead = true
            unreadCount = max(unreadCount - 1, 0)
        }
    }
    
    func markAllAsRead() async {
        try? await notificationsRepository.markAllAsRead()
        notifications = notifications.map { $0.copy(isRead: true) }
        unreadCount = 0
    }
}
```

---

## Badge Count Management

| Platform | Implementation |
|----------|---------------|
| Android | `NotificationManagerCompat.from(context).cancelAll()` on app open |
| iOS | `UIApplication.shared.applicationIconBadgeNumber = unreadCount` |
| Both | Update badge on every notification fetch |

---

## Foreground Notification Handling

When the app is in the foreground:
- **Android**: Show notification via `NotificationManager`
- **iOS**: Show banner via `UNUserNotificationCenter` with `.banner` option
- Both platforms update the in-app notification list in real-time
