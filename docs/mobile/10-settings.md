# 10 — Settings Module

## Overview

Customer app preferences including notification settings, theme selection, language, biometric toggle, and account actions.

---

## Screen Layout

### Settings Screen
```
┌─────────────────────────────────┐
│  ← Settings                ⚙️    │
├─────────────────────────────────┤
│                                 │
│  ── Account ────────────────    │
│                                 │
│  👤 My Profile              >  │
│  🔐 Security & Privacy     >  │
│                                 │
│  ── Notifications ──────────    │
│                                 │
│  Push Notifications     [●]    │
│  Bill Reminders         [●]    │
│  Usage Alerts           [○]    │
│  Ticket Updates         [●]    │
│  Promotions             [○]    │
│                                 │
│  ── Appearance ────────────    │
│                                 │
│  Theme              [System ▼] │
│  Language            [English ▼]│
│                                 │
│  ── Security ──────────────    │
│                                 │
│  Biometric Login       [●]    │
│  Change PIN            >       │
│                                 │
│  ── Support ───────────────    │
│                                 │
│  Help Center               >  │
│  Contact Us                 >  │
│  Rate the App            >  │
│  Terms & Privacy         >  │
│                                 │
│  ── About ─────────────────    │
│                                 │
│  App Version: 1.0.0            │
│  Build: 42                     │
│                                 │
│  ── Account Actions ───────    │
│                                 │
│        Log Out                  │
│                                 │
│        Delete Account           │
│                                 │
└─────────────────────────────────┘
```

### Security & Privacy Screen
```
┌─────────────────────────────────┐
│  ← Security & Privacy           │
├─────────────────────────────────┤
│                                 │
│  ── Authentication ─────────    │
│                                 │
│  Biometric Login       [●]    │
│  Require for Payments  [●]    │
│                                 │
│  ── Data & Privacy ────────    │
│                                 │
│  Clear App Cache        >     │
│  Download My Data       >     │
│                                 │
│  ── Session ───────────────    │
│                                 │
│  Active Sessions: 2            │
│  [Manage Sessions →]           │
│                                 │
└─────────────────────────────────┘
```

---

## API Endpoints

### Get Settings
```
GET /api/v1/customer/settings

Response 200:
{
  "settings": {
    "notifications": {
      "push_enabled": true,
      "bill_reminders": true,
      "usage_alerts": false,
      "ticket_updates": true,
      "promotions": false
    },
    "appearance": {
      "theme": "system",
      "language": "en"
    },
    "security": {
      "biometric_enabled": true,
      "require_biometric_for_payment": true
    }
  }
}
```

### Update Settings
```
PATCH /api/v1/customer/settings

Request:
{
  "notifications": {
    "push_enabled": true,
    "bill_reminders": true,
    "usage_alerts": true
  }
}

Response 200: { "success": true }
```

---

## Android Implementation

### SettingsViewModel.kt
```kotlin
@HiltViewModel
class SettingsViewModel @Inject constructor(
    private val settingsRepository: SettingsRepository,
    private val authRepository: AuthRepository,
    @ApplicationContext private val context: Context
) : ViewModel() {
    
    private val _uiState = MutableStateFlow(SettingsUiState())
    val uiState: StateFlow<SettingsUiState> = _uiState.asStateFlow()
    
    init {
        loadSettings()
    }
    
    private fun loadSettings() {
        viewModelScope.launch {
            settingsRepository.getSettings()
                .onSuccess { settings ->
                    _uiState.update { it.copy(settings = settings, isLoading = false) }
                }
        }
    }
    
    fun updateNotificationSetting(key: String, enabled: Boolean) {
        viewModelScope.launch {
            val current = _uiState.value.settings ?: return@launch
            val updated = current.copy(
                notifications = current.notifications.toMutableMap().apply {
                    put(key, enabled)
                }
            )
            settingsRepository.updateSettings(updated)
            _uiState.update { it.copy(settings = updated) }
        }
    }
    
    fun updateTheme(theme: String) {
        viewModelScope.launch {
            val current = _uiState.value.settings ?: return@launch
            val updated = current.copy(
                appearance = current.appearance.copy(theme = theme)
            )
            settingsRepository.updateSettings(updated)
            _uiState.update { it.copy(settings = updated) }
        }
    }
    
    fun updateLanguage(language: String) {
        viewModelScope.launch {
            val current = _uiState.value.settings ?: return@launch
            val updated = current.copy(
                appearance = current.appearance.copy(language = language)
            )
            settingsRepository.updateSettings(updated)
            _uiState.update { it.copy(settings = updated) }
        }
    }
    
    fun toggleBiometric(enabled: Boolean) {
        viewModelScope.launch {
            val current = _uiState.value.settings ?: return@launch
            val updated = current.copy(
                security = current.security.copy(biometricEnabled = enabled)
            )
            settingsRepository.updateSettings(updated)
            _uiState.update { it.copy(settings = updated) }
        }
    }
    
    fun clearCache() {
        viewModelScope.launch {
            // Clear Room cache
            settingsRepository.clearCache()
            // Clear image cache
            Coil.getImageLoader(context).memoryCache?.clear()
            _uiState.update { it.copy(cacheCleared = true) }
        }
    }
    
    fun logout() {
        viewModelScope.launch {
            authRepository.logout()
            authRepository.clearTokens()
            _uiState.update { it.copy(loggedOut = true) }
        }
    }
}

data class SettingsUiState(
    val isLoading: Boolean = true,
    val settings: AppSettings? = null,
    val cacheCleared: Boolean = false,
    val loggedOut: Boolean = false,
    val error: String? = null
)
```

### Theme Toggle (Android)
```kotlin
@Composable
fun ThemeToggle(
    currentTheme: String,
    onThemeChanged: (String) -> Unit
) {
    var expanded by remember { mutableStateOf(false) }
    
    Row(
        modifier = Modifier.fillMaxWidth(),
        horizontalArrangement = Arrangement.SpaceBetween,
        verticalAlignment = Alignment.CenterVertically
    ) {
        Text("Theme")
        
        ExposedDropdownMenuBox(
            expanded = expanded,
            onExpandedChange = { expanded = it }
        ) {
            Text(
                text = when (currentTheme) {
                    "light" -> "Light"
                    "dark" -> "Dark"
                    else -> "System"
                }
            )
            ExposedDropdownMenu(
                expanded = expanded,
                onDismissRequest = { expanded = false }
            ) {
                DropdownMenuItem(
                    text = { Text("System") },
                    onClick = { onThemeChanged("system"); expanded = false }
                )
                DropdownMenuItem(
                    text = { Text("Light") },
                    onClick = { onThemeChanged("light"); expanded = false }
                )
                DropdownMenuItem(
                    text = { Text("Dark") },
                    onClick = { onThemeChanged("dark"); expanded = false }
                )
            }
        }
    }
}
```

---

## iOS Implementation

### SettingsViewModel.swift
```swift
@Observable
class SettingsViewModel {
    var isLoading: Bool = true
    var settings: AppSettings?
    var cacheCleared: Bool = false
    var loggedOut: Bool = false
    var error: String?
    
    private let settingsRepository: SettingsRepositoryProtocol
    private let authRepository: AuthRepositoryProtocol
    
    init(
        settingsRepository: SettingsRepositoryProtocol = SettingsRepository(),
        authRepository: AuthRepositoryProtocol = AuthRepository()
    ) {
        self.settingsRepository = settingsRepository
        self.authRepository = authRepository
        Task { await loadSettings() }
    }
    
    @MainActor
    func loadSettings() async {
        isLoading = true
        defer { isLoading = false }
        
        do {
            settings = try await settingsRepository.getSettings()
        } catch {
            self.error = error.localizedDescription
        }
    }
    
    func updateNotification(key: String, enabled: Bool) async {
        guard var current = settings else { return }
        current.notifications[key] = enabled
        try? await settingsRepository.updateSettings(current)
        settings = current
    }
    
    func updateTheme(_ theme: String) async {
        guard var current = settings else { return }
        current.appearance.theme = theme
        try? await settingsRepository.updateSettings(current)
        settings = current
    }
    
    func logout() async {
        try? await authRepository.logout()
        authRepository.clearTokens()
        loggedOut = true
    }
}
```

### SettingsView.swift
```swift
struct SettingsView: View {
    @State private var viewModel = SettingsViewModel()
    
    var body: some View {
        List {
            // Account
            Section("Account") {
                NavigationLink("My Profile", destination: ProfileView())
                NavigationLink("Security & Privacy", destination: SecurityView())
            }
            
            // Notifications
            Section("Notifications") {
                if let settings = viewModel.settings {
                    Toggle("Push Notifications", isOn: binding(key: "push_enabled"))
                    Toggle("Bill Reminders", isOn: binding(key: "bill_reminders"))
                    Toggle("Usage Alerts", isOn: binding(key: "usage_alerts"))
                    Toggle("Ticket Updates", isOn: binding(key: "ticket_updates"))
                    Toggle("Promotions", isOn: binding(key: "promotions"))
                }
            }
            
            // Appearance
            Section("Appearance") {
                Picker("Theme", selection: themeBinding) {
                    Text("System").tag("system")
                    Text("Light").tag("light")
                    Text("Dark").tag("dark")
                }
                
                Picker("Language", selection: languageBinding) {
                    Text("English").tag("en")
                    Text("Hindi").tag("hi")
                    Text("Marathi").tag("mr")
                }
            }
            
            // Support
            Section("Support") {
                Link("Help Center", destination: URL(string: "https://aeroxebroadband.com/help")!)
                Link("Contact Us", destination: URL(string: "tel:+919876543210")!)
                Link("Terms & Privacy", destination: URL(string: "https://aeroxebroadband.com/terms")!)
            }
            
            // About
            Section("About") {
                HStack {
                    Text("App Version")
                    Spacer()
                    Text("1.0.0 (42)")
                        .foregroundStyle(.secondary)
                }
            }
            
            // Account Actions
            Section {
                Button("Log Out", role: .destructive) {
                    Task { await viewModel.logout() }
                }
            }
        }
        .navigationTitle("Settings")
    }
}
```

---

## Theme Application

| Platform | Light Mode | Dark Mode | System |
|----------|-----------|-----------|--------|
| Android | `MaterialTheme(colorScheme = lightColorScheme())` | `MaterialTheme(colorScheme = darkColorScheme())` | Follows system setting |
| iOS | `preferredColorScheme(.light)` | `preferredColorScheme(.dark)` | No modifier (follows system) |

---

## Language Support

| Language | Code | UI Strings |
|----------|------|------------|
| English | `en` | Default |
| Hindi | `hi` | Full translation |
| Marathi | `mr` | Full translation |

### Android: strings.xml structure
```
res/
├── values/
│   └── strings.xml          (English - default)
├── values-hi/
│   └── strings.xml          (Hindi)
└── values-mr/
    └── strings.xml          (Marathi)
```

### iOS: Localizable.strings structure
```
├── en.lproj/
│   └── Localizable.strings
├── hi.lproj/
│   └── Localizable.strings
└── mr.lproj/
    └── Localizable.strings
```
