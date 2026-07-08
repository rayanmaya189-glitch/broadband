# 17 — Referral Program Module

## Overview

Customers can share their unique referral code, track referral status, view rewards earned, and redeem benefits. The referral program is managed by admins — customers interact with the active program. Benefits include account credits, free days, plan upgrades, and discounts.

---

## Screen Layout

### Referral Home Screen
```
┌─────────────────────────────────┐
│  ← Referral Program       🎁    │
├─────────────────────────────────┤
│                                 │
│  ┌───────────────────────────┐  │
│  │  🎁 Refer & Earn          │  │
│  │                           │  │
│  │  Share your code and      │  │
│  │  earn ₹200 for each       │  │
│  │  friend who subscribes!   │  │
│  │                           │  │
│  │  Your Code:               │  │
│  │  ┌─────────────────────┐  │  │
│  │  │  RAHU2485      📋  │  │  │
│  │  └─────────────────────┘  │  │
│  │                           │  │
│  │  [📤 Share Code]          │  │
│  └───────────────────────────┘  │
│                                 │
│  ── Your Stats ─────────────    │
│                                 │
│  ┌──────┐ ┌──────┐ ┌──────┐    │
│  │  23  │ │   8  │ │₹1,600│    │
│  │Shared│ │Active│ │Earned│    │
│  └──────┘ └──────┘ └──────┘    │
│                                 │
│  ── How It Works ───────────    │
│                                 │
│  1️⃣  Share your code            │
│  2️⃣  Friend signs up            │
│  3️⃣  Friend subscribes          │
│  4️⃣  You both get rewards! 🎉   │
│                                 │
│  ── Reward Benefits ────────    │
│                                 │
│  👤 Referrer Gets:              │
│     • ₹200 account credit       │
│     • Per successful referral   │
│                                 │
│  👥 Friend Gets:                │
│     • 10% off first month       │
│     • Welcome bonus             │
│                                 │
│  ── Your Referrals ─────────    │
│                                 │
│  ┌───────────────────────────┐  │
│  │  🟢 Amit Deshmukh         │  │
│  │  +9198765 43210            │  │
│  │  Status: Activated ✅     │  │
│  │  Reward: ₹200 credited    │  │
│  └───────────────────────────┘  │
│                                 │
│  ┌───────────────────────────┐  │
│  │  🟡 Priya Sharma          │  │
│  │  +9198765 43211            │  │
│  │  Status: Registered ⏳    │  │
│  │  Waiting for subscription │  │
│  └───────────────────────────┘  │
│                                 │
│  ┌───────────────────────────┐  │
│  │  🔵 Vikram Joshi          │  │
│  │  +9198765 43212            │  │
│  │  Status: Pending 📤      │  │
│  │  Code shared, not yet     │  │
│  │  signed up                │  │
│  └───────────────────────────┘  │
│                                 │
│  [View All Referrals →]         │
│                                 │
└─────────────────────────────────┘
```

### Share Code Screen
```
┌─────────────────────────────────┐
│  ← Share Your Code              │
├─────────────────────────────────┤
│                                 │
│  ┌───────────────────────────┐  │
│  │                           │  │
│  │     🎁 RAHU2485           │  │
│  │                           │  │
│  │  Share with friends and   │  │
│  │  earn ₹200 for each       │  │
│  │  successful referral!     │  │
│  │                           │  │
│  └───────────────────────────┘  │
│                                 │
│  ── Share Via ──────────────    │
│                                 │
│  ┌──────┐ ┌──────┐ ┌──────┐    │
│  │ 📱   │ │ 💬   │ │ 📧   │    │
│  │WhatsApp│ │SMS  │ │Email │    │
│  └──────┘ └──────┘ └──────┘    │
│  ┌──────┐ ┌──────┐ ┌──────┐    │
│  │ 📋   │ │ 📲   │ │ •••  │    │
│  │ Copy │ │Telegram│ │More │    │
│  └──────┘ └──────┘ └──────┘    │
│                                 │
│  ── Or Send Directly ───────    │
│                                 │
│  Enter phone number:            │
│  ┌───────────────────────────┐  │
│  │ +91  [_______________]    │  │
│  └───────────────────────────┘  │
│  [Send SMS Invite →]            │
│                                 │
└─────────────────────────────────┘
```

### Referral Detail Screen
```
┌─────────────────────────────────┐
│  ← Referral #REF-2026-0045      │
├─────────────────────────────────┤
│                                 │
│  ┌───────────────────────────┐  │
│  │  👤 Amit Deshmukh         │  │
│  │  +9198765 43210            │  │
│  │                           │  │
│  │  Status: ✅ Activated     │  │
│  │  Program: Launch 2026     │  │
│  │                           │  │
│  │  ── Timeline ──────────   │  │
│  │                           │  │
│  │  📤 Shared: Mar 15, 10AM  │  │
│  │  │                        │  │
│  │  📝 Registered: Mar 15, 2PM│ │
│  │  │                        │  │
│  │  ✅ Activated: Mar 20, 3PM│  │
│  │  │                        │  │
│  │  🎁 Rewarded: Mar 20, 3PM│  │
│  │                           │  │
│  └───────────────────────────┘  │
│                                 │
│  ── Rewards ────────────────    │
│                                 │
│  ┌───────────────────────────┐  │
│  │  👤 Your Reward            │  │
│  │  Type: Account Credit      │  │
│  │  Amount: ₹200             │  │
│  │  Status: ✅ Credited      │  │
│  │  Credited: Mar 20, 2026  │  │
│  └───────────────────────────┘  │
│                                 │
│  ┌───────────────────────────┐  │
│  │  👥 Friend's Reward        │  │
│  │  Type: Discount            │  │
│  │  Amount: 10% off           │  │
│  │  Status: ✅ Applied        │  │
│  │  Applied: Mar 20, 2026    │  │
│  └───────────────────────────┘  │
│                                 │
└─────────────────────────────────┘
```

### Rewards History Screen
```
┌─────────────────────────────────┐
│  ← Reward History         💰    │
├─────────────────────────────────┤
│                                 │
│  ┌───────────────────────────┐  │
│  │  Total Earned: ₹1,600    │  │
│  │  Available: ₹1,200       │  │
│  │  Used: ₹400              │  │
│  └───────────────────────────┘  │
│                                 │
│  ── Recent Rewards ─────────    │
│                                 │
│  ┌───────────────────────────┐  │
│  │  🎁 ₹200 Credit          │  │
│  │  Referral: Amit Deshmukh  │  │
│  │  Mar 20, 2026            │  │
│  │  Status: ✅ Applied       │  │
│  └───────────────────────────┘  │
│                                 │
│  ┌───────────────────────────┐  │
│  │  🎁 ₹200 Credit          │  │
│  │  Referral: Priya Sharma   │  │
│  │  Mar 25, 2026            │  │
│  │  Status: ✅ Applied       │  │
│  └───────────────────────────┘  │
│                                 │
│  ┌───────────────────────────┐  │
│  │  🎁 ₹200 Credit          │  │
│  │  Referral: Vikram Joshi   │  │
│  │  Apr 1, 2026             │  │
│  │  Status: ⏳ Pending       │  │
│  └───────────────────────────┘  │
│                                 │
│  ── Benefits Used ──────────    │
│                                 │
│  ┌───────────────────────────┐  │
│  │  💳 ₹200 credit applied   │  │
│  │  to invoice INV-2026-0008│  │
│  │  Mar 28, 2026            │  │
│  └───────────────────────────┘  │
│                                 │
└─────────────────────────────────┘
```

---

## API Endpoints

### Get My Referral Code
```
GET /api/v1/customer/referrals/my-code

Response 200:
{
  "referral_code": "RAHU2485",
  "program": {
    "id": "prog_abc123",
    "name": "Launch 2026",
    "referrer_reward_type": "credit",
    "referrer_reward_value": 200,
    "referee_reward_type": "discount",
    "referee_reward_value": 10
  },
  "stats": {
    "total_shared": 23,
    "total_registered": 15,
    "total_activated": 8,
    "total_rewards_earned": 1600,
    "available_balance": 1200,
    "used_balance": 400
  }
}
```

### Get My Referrals
```
GET /api/v1/customer/referrals/my-referrals?page=1&limit=20

Response 200:
{
  "referrals": [
    {
      "id": "ref_xyz789",
      "referral_code": "RAHU2485",
      "referee_name": "Amit Deshmukh",
      "referee_phone": "+919876543210",
      "status": "activated",
      "referrer_reward": {
        "type": "credit",
        "amount": 200,
        "status": "credited",
        "credited_at": "2026-03-20T15:00:00Z"
      },
      "referee_reward": {
        "type": "discount",
        "amount": 10,
        "status": "applied",
        "applied_at": "2026-03-20T15:00:00Z"
      },
      "shared_at": "2026-03-15T10:00:00Z",
      "registered_at": "2026-03-15T14:00:00Z",
      "activated_at": "2026-03-20T15:00:00Z",
      "rewarded_at": "2026-03-20T15:00:00Z"
    },
    {
      "id": "ref_abc123",
      "referee_name": "Priya Sharma",
      "referee_phone": "+919876543211",
      "status": "registered",
      "referrer_reward": null,
      "referee_reward": null,
      "shared_at": "2026-03-18T09:00:00Z",
      "registered_at": "2026-03-18T11:00:00Z",
      "activated_at": null,
      "rewarded_at": null
    }
  ],
  "pagination": { "page": 1, "limit": 20, "total": 23 }
}
```

### Share Referral Code
```
POST /api/v1/customer/referrals/share

Request:
{
  "method": "sms",
  "phone": "+919876543212",
  "message": "Join AeroXe Broadband! Use my code RAHU2485 for 10% off your first month!"
}

Response 200:
{
  "success": true,
  "referral": {
    "id": "ref_new123",
    "referee_phone": "+919876543212",
    "status": "pending",
    "shared_at": "2026-07-08T10:30:00Z"
  }
}
```

### Get Referral Stats
```
GET /api/v1/customer/referrals/stats

Response 200:
{
  "stats": {
    "total_shared": 23,
    "total_registered": 15,
    "total_activated": 8,
    "conversion_rate": 34.8,
    "total_rewards_earned": 1600,
    "available_balance": 1200,
    "used_balance": 400,
    "rewards_history": [
      {
        "id": "rew_001",
        "type": "credit",
        "amount": 200,
        "referral_id": "ref_xyz789",
        "referee_name": "Amit Deshmukh",
        "status": "applied",
        "earned_at": "2026-03-20T15:00:00Z",
        "applied_to": "INV-2026-0008",
        "applied_at": "2026-03-28T10:00:00Z"
      }
    ]
  }
}
```

### Get Active Program
```
GET /api/v1/customer/referrals/program

Response 200:
{
  "program": {
    "id": "prog_abc123",
    "name": "Launch 2026",
    "status": "active",
    "referrer_reward": {
      "type": "credit",
      "amount": 200,
      "description": "₹200 account credit per successful referral"
    },
    "referee_reward": {
      "type": "discount",
      "amount": 10,
      "description": "10% off your first month"
    },
    "max_referrals": 10,
    "end_date": "2026-12-31",
    "terms": [
      "Friend must subscribe to a plan to qualify",
      "Maximum 10 referrals per customer",
      "Credit is applied to your next invoice",
      "Discount applies to friend's first invoice only"
    ]
  }
}
```

---

## Android Implementation

### ReferralsViewModel.kt
```kotlin
@HiltViewModel
class ReferralsViewModel @Inject constructor(
    private val referralsRepository: ReferralsRepository
) : ViewModel() {
    
    private val _uiState = MutableStateFlow(ReferralsUiState())
    val uiState: StateFlow<ReferralsUiState> = _uiState.asStateFlow()
    
    init {
        loadReferralData()
    }
    
    private fun loadReferralData() {
        viewModelScope.launch {
            _uiState.update { it.copy(isLoading = true) }
            
            val codeDeferred = async { referralsRepository.getMyCode() }
            val referralsDeferred = async { referralsRepository.getMyReferrals() }
            val programDeferred = async { referralsRepository.getActiveProgram() }
            
            val code = codeDeferred.await()
            val referrals = referralsDeferred.await()
            val program = programDeferred.await()
            
            code.onSuccess { codeData ->
                referrals.onSuccess { referralData ->
                    program.onSuccess { programData ->
                        _uiState.update {
                            it.copy(
                                isLoading = false,
                                referralCode = codeData.referralCode,
                                stats = codeData.stats,
                                referrals = referralData.referrals,
                                program = programData.program
                            )
                        }
                    }
                }
            }
        }
    }
    
    fun shareCode(method: String, phone: String? = null) {
        viewModelScope.launch {
            _uiState.update { it.copy(isSharing = true) }
            referralsRepository.shareReferral(
                method = method,
                phone = phone
            ).onSuccess { result ->
                _uiState.update {
                    it.copy(
                        isSharing = false,
                        lastSharedReferral = result.referral,
                        showShareSuccess = true
                    )
                }
                loadReferralData() // Refresh stats
            }.onFailure { error ->
                _uiState.update { it.copy(isSharing = false, error = error.message) }
            }
        }
    }
    
    fun copyCode() {
        _uiState.update { it.copy(codeCopied = true) }
    }
    
    fun dismissShareSuccess() {
        _uiState.update { it.copy(showShareSuccess = false) }
    }
}

data class ReferralsUiState(
    val isLoading: Boolean = true,
    val referralCode: String = "",
    val stats: ReferralStats = ReferralStats(),
    val referrals: List<Referral> = emptyList(),
    val program: ReferralProgram? = null,
    val isSharing: Boolean = false,
    val lastSharedReferral: Referral? = null,
    val showShareSuccess: Boolean = false,
    val codeCopied: Boolean = false,
    val error: String? = null
)
```

### ReferralShareSheet.kt
```kotlin
@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun ReferralShareSheet(
    referralCode: String,
    onShare: (String, String?) -> Unit,
    onDismiss: () -> Unit
) {
    ModalBottomSheet(
        onDismissRequest = onDismiss,
        sheetState = rememberModalBottomSheetState()
    ) {
        Column(
            modifier = Modifier
                .fillMaxWidth()
                .padding(16.dp)
        ) {
            Text(
                "Share Your Code",
                style = MaterialTheme.typography.headlineSmall,
                modifier = Modifier.padding(bottom = 16.dp)
            )
            
            // Referral code display
            Card(
                modifier = Modifier.fillMaxWidth(),
                colors = CardDefaults.cardColors(
                    containerColor = MaterialTheme.colorScheme.primaryContainer
                )
            ) {
                Column(
                    modifier = Modifier.padding(16.dp),
                    horizontalAlignment = Alignment.CenterHorizontally
                ) {
                    Text(
                        referralCode,
                        style = MaterialTheme.typography.headlineLarge,
                        fontWeight = FontWeight.Bold
                    )
                    Spacer(Modifier.height(8.dp))
                    Text(
                        "Share with friends and earn rewards!",
                        style = MaterialTheme.typography.bodyMedium
                    )
                }
            }
            
            Spacer(Modifier.height(24.dp))
            
            // Share options grid
            LazyVerticalGrid(
                columns = GridCells.Fixed(3),
                horizontalArrangement = Arrangement.spacedBy(12.dp),
                verticalArrangement = Arrangement.spacedBy(12.dp)
            ) {
                item {
                    ShareOption(
                        icon = Icons.Default.Share,
                        label = "WhatsApp",
                        onClick = { onShare("whatsapp", null) }
                    )
                }
                item {
                    ShareOption(
                        icon = Icons.Default.Sms,
                        label = "SMS",
                        onClick = { onShare("sms", null) }
                    )
                }
                item {
                    ShareOption(
                        icon = Icons.Default.Email,
                        label = "Email",
                        onClick = { onShare("email", null) }
                    )
                }
                item {
                    ShareOption(
                        icon = Icons.Default.ContentCopy,
                        label = "Copy",
                        onClick = { onShare("copy", null) }
                    )
                }
                item {
                    ShareOption(
                        icon = Icons.Default.Telegram,
                        label = "Telegram",
                        onClick = { onShare("telegram", null) }
                    )
                }
                item {
                    ShareOption(
                        icon = Icons.Default.MoreVert,
                        label = "More",
                        onClick = { onShare("system", null) }
                    )
                }
            }
            
            Spacer(Modifier.height(24.dp))
        }
    }
}

@Composable
fun ShareOption(
    icon: ImageVector,
    label: String,
    onClick: () -> Unit
) {
    Column(
        horizontalAlignment = Alignment.CenterHorizontally,
        modifier = Modifier
            .clip(RoundedCornerShape(12.dp))
            .clickable(onClick = onClick)
            .padding(12.dp)
    ) {
        Icon(
            icon,
            contentDescription = label,
            modifier = Modifier.size(32.dp),
            tint = MaterialTheme.colorScheme.primary
        )
        Spacer(Modifier.height(4.dp))
        Text(
            label,
            style = MaterialTheme.typography.bodySmall
        )
    }
}
```

---

## iOS Implementation

### ReferralsViewModel.swift
```swift
@Observable
class ReferralsViewModel {
    var isLoading: Bool = true
    var referralCode: String = ""
    var stats: ReferralStats = ReferralStats()
    var referrals: [Referral] = []
    var program: ReferralProgram?
    var isSharing: Bool = false
    var showShareSheet: Bool = false
    var codeCopied: Bool = false
    var error: String?
    
    private let referralsRepository: ReferralsRepositoryProtocol
    
    init(referralsRepository: ReferralsRepositoryProtocol = ReferralsRepository()) {
        self.referralsRepository = referralsRepository
        Task { await loadReferralData() }
    }
    
    @MainActor
    func loadReferralData() async {
        isLoading = true
        defer { isLoading = false }
        
        do {
            async let code = referralsRepository.getMyCode()
            async let myReferrals = referralsRepository.getMyReferrals()
            async let activeProgram = referralsRepository.getActiveProgram()
            
            let codeData = try await code
            let referralData = try await myReferrals
            let programData = try await activeProgram
            
            referralCode = codeData.referralCode
            stats = codeData.stats
            referrals = referralData.referrals
            program = programData.program
        } catch {
            self.error = error.localizedDescription
        }
    }
    
    func shareCode(method: String, phone: String? = nil) async {
        isSharing = true
        defer { isSharing = false }
        
        do {
            let result = try await referralsRepository.shareReferral(method: method, phone: phone)
            await loadReferralData()
        } catch {
            self.error = error.localizedDescription
        }
    }
    
    func copyCode() {
        UIPasteboard.general.string = referralCode
        codeCopied = true
    }
}
```

### ReferralShareSheet.swift
```swift
struct ReferralShareSheet: View {
    let referralCode: String
    let onShare: (String, String?) -> Void
    @Environment(\.dismiss) private var dismiss
    
    var body: some View {
        NavigationStack {
            VStack(spacing: 20) {
                // Referral code card
                VStack(spacing: 8) {
                    Text("Share Your Code")
                        .font(.headline)
                    
                    Text(referralCode)
                        .font(.largeTitle)
                        .fontWeight(.bold)
                        .foregroundStyle(.primary)
                    
                    Text("Share with friends and earn rewards!")
                        .font(.subheadline)
                        .foregroundStyle(.secondary)
                }
                .padding()
                .frame(maxWidth: .infinity)
                .background(.ultraThinMaterial)
                .clipShape(RoundedRectangle(cornerRadius: 16))
                
                // Share options
                LazyVGrid(columns: [
                    GridItem(.flexible()),
                    GridItem(.flexible()),
                    GridItem(.flexible())
                ], spacing: 16) {
                    ShareOptionButton(icon: "message", label: "WhatsApp") {
                        onShare("whatsapp", nil)
                    }
                    ShareOptionButton(icon: "text.bubble", label: "SMS") {
                        onShare("sms", nil)
                    }
                    ShareOptionButton(icon: "envelope", label: "Email") {
                        onShare("email", nil)
                    }
                    ShareOptionButton(icon: "doc.on.doc", label: "Copy") {
                        onShare("copy", nil)
                    }
                    ShareOptionButton(icon: "paperplane", label: "Telegram") {
                        onShare("telegram", nil)
                    }
                    ShareOptionButton(icon: "ellipsis", label: "More") {
                        onShare("system", nil)
                    }
                }
                .padding(.horizontal)
            }
            .padding()
            .navigationTitle("Refer a Friend")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .topBarTrailing) {
                    Button("Done") { dismiss() }
                }
            }
        }
    }
}

struct ShareOptionButton: View {
    let icon: String
    let label: String
    let action: () -> Void
    
    var body: some View {
        Button(action: action) {
            VStack(spacing: 8) {
                Image(systemName: icon)
                    .font(.title2)
                    .foregroundStyle(.blue)
                Text(label)
                    .font(.caption)
                    .foregroundStyle(.primary)
            }
            .frame(maxWidth: .infinity)
            .padding(.vertical, 16)
            .background(.ultraThinMaterial)
            .clipShape(RoundedRectangle(cornerRadius: 12))
        }
        .buttonStyle(.plain)
    }
}
```

---

## Referral Status Flow

```
┌──────────┐     ┌──────────────┐     ┌──────────────┐     ┌──────────┐
│  Pending  │────▶│  Registered  │────▶│  Activated   │────▶│ Rewarded │
│  (shared) │     │  (signed up) │     │  (subscribed)│     │          │
└──────────┘     └──────────────┘     └──────────────┘     └──────────┘
```

### Status Indicators

| Status | Icon | Color | Description |
|--------|------|-------|-------------|
| `pending` | 📤 | Blue | Code shared, friend hasn't signed up yet |
| `registered` | ⏳ | Yellow | Friend signed up but hasn't subscribed |
| `activated` | ✅ | Green | Friend subscribed to a plan |
| `rewarded` | 🎁 | Green | Both parties received rewards |

---

## Share Methods

| Method | Implementation | Platforms |
|--------|---------------|-----------|
| `whatsapp` | Deep link: `whatsapp://send?text=...` | Android + iOS |
| `sms` | `sms:` URL scheme with pre-filled message | Android + iOS |
| `email` | `mailto:` URL scheme | Android + iOS |
| `copy` | ClipboardManager (Android) / UIPasteboard (iOS) | Both |
| `telegram` | Deep link: `tg://msg?text=...` | Android + iOS |
| `system` | Native share sheet | Both |

### Android Share Intent
```kotlin
fun shareViaSystem(context: Context, code: String, message: String) {
    val intent = Intent(Intent.ACTION_SEND).apply {
        type = "text/plain"
        putExtra(Intent.EXTRA_TEXT, "$message\n\nUse code: $code")
        putExtra(Intent.EXTRA_SUBJECT, "Join AeroXe Broadband!")
    }
    context.startActivity(Intent.createChooser(intent, "Share via"))
}
```

### iOS Share Sheet
```swift
func shareViaSystem(code: String, message: String) {
    let items = ["\(message)\n\nUse code: \(code)"]
    let activityVC = UIActivityViewController(
        activityItems: items,
        applicationActivities: nil
    )
    // Present from root view controller
}
```

---

## Reward Application

When a referral is activated, rewards are automatically applied:

| Reward Type | Referrer | Referee | Application |
|-------------|----------|---------|-------------|
| `credit` | ₹200 added to wallet | — | Auto-applied to next invoice |
| `free_days` | 7 free days added | — | Extended subscription end date |
| `plan_upgrade` | Upgrade for 1 month | — | Temp plan upgrade |
| `discount` | — | 10% off first invoice | Applied to first invoice |

### Wallet Balance Display
```
┌─────────────────────────────────┐
│  💰 Wallet Balance              │
│                                 │
│  Available: ₹1,200             │
│  Used: ₹400                    │
│  Total Earned: ₹1,600          │
│                                 │
│  Balance is auto-applied to     │
│  your next invoice.             │
└─────────────────────────────────┘
```

---

## Deep Link Handling

When a user clicks a referral link:

```
aeroxe://referral?code=RAHU2485
https://aeroxebroadband.com/app/referral?code=RAHU2485
```

### Android
```kotlin
// In NavGraph
composable("referral/{code}") { backStackEntry ->
    val code = backStackEntry.arguments?.getString("code") ?: ""
    ReferralSignupScreen(referralCode = code, navController)
}
```

### iOS
```swift
// In ContentView
case .referral(let code):
    ReferralSignupView(referralCode: code)
```

---

## Notification Events

| Event | Notification | Trigger |
|-------|-------------|---------|
| `referral.activated` | "🎉 Your friend {name} just subscribed! ₹{amount} credited to your account." | Friend activates service |
| `referral.reward_applied` | "💰 Your ₹{amount} referral reward has been applied to invoice {number}." | Reward applied to invoice |
| `referral.program_update` | "🎁 New referral program: {name}! Share now and earn {reward}." | Admin creates new program |
