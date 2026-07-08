# 13 — Navigation & Deep Links Module

## Overview

Both apps use a bottom tab bar as the primary navigation, with stack navigation within each tab. Deep links allow navigation from push notifications, QR codes, and external URLs.

---

## Navigation Structure

### Bottom Tabs

```
┌─────────────────────────────────┐
│                                 │
│         Screen Content          │
│                                 │
│                                 │
│                                 │
├─────────────────────────────────┤
│  🏠     💰     🎫     👤     ⚙️  │
│  Home   Bills  Help   Me   More │
└─────────────────────────────────┘
```

| Tab | Label | Screen | Badge |
|-----|-------|--------|-------|
| `home` | Home | Dashboard | Notification count |
| `bills` | Bills | Invoices List | Pending count |
| `help` | Help | Tickets List | Open ticket count |
| `me` | Me | Profile | — |
| `more` | More | Settings | — |

---

## Android Navigation

### NavGraph.kt
```kotlin
@Composable
fun AeroXeNavGraph(
    navController: NavHostController = rememberNavController(),
    startDestination: String = "splash"
) {
    NavHost(navController = navController, startDestination = startDestination) {
        
        // Auth flow
        composable("splash") { SplashScreen(navController) }
        composable("auth/phone") { PhoneInputScreen(navController) }
        composable("auth/otp/{phone}") { backStackEntry ->
            val phone = backStackEntry.arguments?.getString("phone") ?: ""
            OtpVerificationScreen(navController, phone)
        }
        
        // Main tabs
        navigation(startDestination = "home", route = "main") {
            // Home tab
            composable("home") {
                DashboardScreen(
                    onNavigateToUsage = { navController.navigate("usage") },
                    onNavigateToPlans = { navController.navigate("plans") },
                    onNavigateToTickets = { navController.navigate("help") },
                    onNavigateToInvoices = { navController.navigate("bills") },
                    onNavigateToProfile = { navController.navigate("me") },
                    onNavigateToSettings = { navController.navigate("settings") }
                )
            }
            
            // Bills tab
            composable("bills") {
                InvoicesScreen(
                    onInvoiceClick = { id -> navController.navigate("invoice/$id") },
                    onPayClick = { id -> navController.navigate("payment/$id") }
                )
            }
            
            // Help tab
            composable("help") {
                TicketsScreen(
                    onTicketClick = { id -> navController.navigate("ticket/$id") },
                    onCreateTicket = { navController.navigate("ticket/create") }
                )
            }
            
            // Me tab
            composable("me") {
                ProfileScreen(
                    onEditProfile = { navController.navigate("profile/edit") },
                    onKycUpload = { navController.navigate("kyc/upload") }
                )
            }
            
            // More tab
            composable("more") {
                SettingsScreen(
                    onNavigateToSecurity = { navController.navigate("settings/security") },
                    onNavigateToHelp = { navController.navigate("help") }
                )
            }
        }
        
        // Detail screens (stack within tabs)
        composable("usage") { UsageScreen(navController) }
        composable("plans") { PlansScreen(navController) }
        composable("plans/compare") { PlanComparisonScreen(navController) }
        composable("plans/change/{planId}") { backStackEntry ->
            PlanChangeScreen(navController, backStackEntry.arguments?.getString("planId") ?: "")
        }
        composable("invoice/{id}") { backStackEntry ->
            InvoiceDetailScreen(navController, backStackEntry.arguments?.getString("id") ?: "")
        }
        composable("payment/{invoiceId}") { backStackEntry ->
            PaymentScreen(navController, backStackEntry.arguments?.getString("invoiceId") ?: "")
        }
        composable("ticket/{id}") { backStackEntry ->
            TicketDetailScreen(navController, backStackEntry.arguments?.getString("id") ?: "")
        }
        composable("ticket/create") { CreateTicketScreen(navController) }
        composable("profile/edit") { EditProfileScreen(navController) }
        composable("profile/change-password") { ChangePasswordScreen(navController) }
        composable("kyc/upload") { KycUploadScreen(navController) }
        composable("settings/security") { SecuritySettingsScreen(navController) }
        composable("settings/notifications") { NotificationSettingsScreen(navController) }
        
        // Notification center
        composable("notifications") {
            NotificationCenterScreen(
                onNotificationClick = { type, data ->
                    handleNotificationNavigation(navController, type, data)
                }
            )
        }
        
        // 404
        composable("not_found") { NotFoundScreen(navController) }
    }
}

private fun handleNotificationNavigation(
    navController: NavHostController,
    type: String,
    data: Map<String, String>
) {
    when (type) {
        "bill_due", "bill_overdue" -> navController.navigate("invoice/${data["invoice_id"]}")
        "ticket_update" -> navController.navigate("ticket/${data["ticket_id"]}")
        "plan_changed" -> navController.navigate("plans")
        "usage_warning" -> navController.navigate("usage")
        else -> navController.navigate("home")
    }
}
```

### Type-Safe Navigation (Android)
```kotlin
// Define routes as sealed class
sealed class Screen(val route: String) {
    data object Splash : Screen("splash")
    data object PhoneInput : Screen("auth/phone")
    data class OtpVerification(val phone: String) : Screen("auth/otp/{phone}")
    data object Dashboard : Screen("home")
    data object Invoices : Screen("bills")
    data class InvoiceDetail(val id: String) : Screen("invoice/{id}")
    data class Payment(val invoiceId: String) : Screen("payment/{invoiceId}")
    data object Tickets : Screen("help")
    data class TicketDetail(val id: String) : Screen("ticket/{id}")
    data object CreateTicket : Screen("ticket/create")
    data object Profile : Screen("me")
    data object EditProfile : Screen("profile/edit")
    data object Settings : Screen("settings")
    data object SecuritySettings : Screen("settings/security")
    data object Usage : Screen("usage")
    data object Plans : Screen("plans")
    data object Notifications : Screen("notifications")
}
```

---

## iOS Navigation

### NavigationStack.swift
```swift
enum AppScreen: Hashable {
    case splash
    case phoneInput
    case otpVerification(phone: String)
    case dashboard
    case invoices
    case invoiceDetail(id: String)
    case payment(invoiceId: String)
    case tickets
    case ticketDetail(id: String)
    case createTicket
    case profile
    case editProfile
    case settings
    case securitySettings
    case notificationSettings
    case usage
    case plans
    case notifications
}

struct ContentView: View {
    @State private var navigationPath = NavigationPath()
    @State private var selectedTab: Tab = .home
    @State private var isAuthenticated = false
    
    enum Tab {
        case home, bills, help, me, more
    }
    
    var body: some View {
        if !isAuthenticated {
            NavigationStack(path: $navigationPath) {
                PhoneInputView()
                    .navigationDestination(for: AppScreen.self) { screen in
                        destinationView(for: screen)
                    }
            }
        } else {
            TabView(selection: $selectedTab) {
                NavigationStack(path: $navigationPath) {
                    DashboardView()
                        .navigationDestination(for: AppScreen.self) { screen in
                            destinationView(for: screen)
                        }
                }
                .tabItem {
                    Label("Home", systemImage: "house")
                }
                .tag(Tab.home)
                
                NavigationStack(path: $navigationPath) {
                    InvoicesView()
                        .navigationDestination(for: AppScreen.self) { screen in
                            destinationView(for: screen)
                        }
                }
                .tabItem {
                    Label("Bills", systemImage: "doc.text")
                }
                .tag(Tab.bills)
                
                // ... other tabs
            }
        }
    }
    
    @ViewBuilder
    private func destinationView(for screen: AppScreen) -> some View {
        switch screen {
        case .dashboard:
            DashboardView()
        case .invoiceDetail(let id):
            InvoiceDetailView(invoiceId: id)
        case .ticketDetail(let id):
            TicketDetailView(ticketId: id)
        case .usage:
            UsageView()
        case .plans:
            PlansView()
        case .notifications:
            NotificationCenterView()
        // ... other cases
        default:
            EmptyView()
        }
    }
}
```

---

## Deep Links

### URL Scheme
```
aeroxe://dashboard
aeroxe://invoices/{id}
aeroxe://tickets/{id}
aeroxe://usage
aeroxe://plans
aeroxe://plans/compare
aeroxe://profile
aeroxe://settings
aeroxe://notifications
```

### Universal Links (HTTPS)
```
https://aeroxebroadband.com/app/dashboard
https://aeroxebroadband.com/app/invoices/{id}
https://aeroxebroadband.com/app/tickets/{id}
https://aeroxebroadband.com/app/usage
https://aeroxebroadband.com/app/plans
https://aeroxebroadband.com/app/profile
https://aeroxebroadband.com/app/settings
```

### Android Manifest
```xml
<activity
    android:name=".MainActivity"
    android:exported="true"
    android:launchMode="singleTask">
    
    <intent-filter android:autoVerify="true">
        <action android:name="android.intent.action.VIEW" />
        <category android:name="android.intent.category.DEFAULT" />
        <category android:name="android.intent.category.BROWSABLE" />
        <data
            android:scheme="https"
            android:host="aeroxebroadband.com"
            android:pathPrefix="/app" />
    </intent-filter>
    
    <intent-filter>
        <action android:name="android.intent.action.VIEW" />
        <category android:name="android.intent.category.DEFAULT" />
        <category android:name="android.intent.category.BROWSABLE" />
        <data android:scheme="aeroxe" />
    </intent-filter>
</activity>
```

### Android Deep Link Handler
```kotlin
class DeepLinkHandler @Inject constructor(
    private val navController: NavController
) {
    fun handleDeepLink(intent: Intent) {
        val data = intent.data ?: return
        val path = data.pathSegments
        
        when {
            data.scheme == "aeroxe" || data.host == "aeroxebroadband.com" -> {
                when {
                    path?.firstOrNull() == "dashboard" -> {
                        navController.navigate("home")
                    }
                    path?.firstOrNull() == "invoices" && path.size > 1 -> {
                        navController.navigate("invoice/${path[1]}")
                    }
                    path?.firstOrNull() == "tickets" && path.size > 1 -> {
                        navController.navigate("ticket/${path[1]}")
                    }
                    path?.firstOrNull() == "usage" -> {
                        navController.navigate("usage")
                    }
                    path?.firstOrNull() == "plans" -> {
                        navController.navigate("plans")
                    }
                    path?.firstOrNull() == "profile" -> {
                        navController.navigate("me")
                    }
                    path?.firstOrNull() == "settings" -> {
                        navController.navigate("settings")
                    }
                    else -> {
                        navController.navigate("not_found")
                    }
                }
            }
        }
    }
}
```

### iOS Universal Links (Associated Domains)
```
// apple-app-site-association
{
  "applinks": {
    "apps": [],
    "details": [
      {
        "appIDs": ["TEAMID.com.aeroxebroadband.customer"],
        "paths": [
          "/app/dashboard",
          "/app/invoices/*",
          "/app/tickets/*",
          "/app/usage",
          "/app/plans",
          "/app/profile",
          "/app/settings",
          "/app/notifications"
        ]
      }
    ]
  }
}
```

### iOS Deep Link Handler
```swift
class DeepLinkHandler {
    static func handle(url: URL) {
        guard let components = URLComponents(url: url, resolvingAgainstBaseURL: true) else { return }
        
        let pathComponents = components.path.split(separator: "/").map(String.init)
        
        switch pathComponents.first {
        case "dashboard":
            NavigationManager.shared.navigate(to: .dashboard)
        case "invoices":
            if let id = pathComponents.dropFirst().first {
                NavigationManager.shared.navigate(to: .invoiceDetail(id: id))
            }
        case "tickets":
            if let id = pathComponents.dropFirst().first {
                NavigationManager.shared.navigate(to: .ticketDetail(id: id))
            }
        case "usage":
            NavigationManager.shared.navigate(to: .usage)
        case "plans":
            NavigationManager.shared.navigate(to: .plans)
        case "profile":
            NavigationManager.shared.navigate(to: .profile)
        case "settings":
            NavigationManager.shared.navigate(to: .settings)
        default:
            break
        }
    }
}
```

---

## Navigation Manager (Shared Concept)

### Android
```kotlin
class NavigationManager @Inject constructor() {
    private var navController: NavController? = null
    
    fun setNavController(controller: NavController) {
        navController = controller
    }
    
    fun navigate(screen: Screen) {
        navController?.navigate(screen.route)
    }
    
    fun navigateBack() {
        navController?.popBackStack()
    }
}
```

### iOS
```swift
@Observable
class NavigationManager {
    static let shared = NavigationManager()
    
    var path = NavigationPath()
    
    func navigate(to screen: AppScreen) {
        path.append(screen)
    }
    
    func popToRoot() {
        path = NavigationPath()
    }
    
    func goBack() {
        guard !path.isEmpty else { return }
        path.removeLast()
    }
}
```
