# 05 — Plans Module

## Overview

Customers can view their current plan details, browse available plans, and request plan upgrades/downgrades. Plan data comes from the backend API.

---

## Screen Layout

### Current Plan Screen
```
┌─────────────────────────────────┐
│  ← My Plan                📋    │
├─────────────────────────────────┤
│                                 │
│  ┌───────────────────────────┐  │
│  │  🌟 AeroXe 100           │  │
│  │                           │  │
│  │  ⬇️ 100 Mbps  ⬆️ 50 Mbps  │  │
│  │  Unlimited Data           │  │
│  │                           │  │
│  │  ₹699/month               │  │
│  │  Next billing: Jul 15     │  │
│  └───────────────────────────┘  │
│                                 │
│  ── Plan Features ──────────    │
│                                 │
│  ✅ Unlimited Data              │
│  ✅ Free Installation           │
│  ✅ 24/7 Support               │
│  ✅ Dual Band WiFi Router      │
│  ✅ Priority Support            │
│                                 │
│  ── Billing Info ───────────    │
│                                 │
│  Start Date:    Jul 15, 2025   │
│  Billing Cycle: Monthly         │
│  Auto-Renew:    Yes             │
│  Status:        Active          │
│                                 │
│  ┌─────────────────────────┐   │
│  │     Change Plan →       │   │
│  └─────────────────────────┘   │
└─────────────────────────────────┘
```

### Browse Plans Screen
```
┌─────────────────────────────────┐
│  ← Available Plans              │
├─────────────────────────────────┤
│                                 │
│  ┌───────────────────────────┐  │
│  │  AeroXe 50       ₹400/mo │  │
│  │  ⬇️ 50 Mbps  ⬆️ 25 Mbps  │  │
│  │  Unlimited Data           │  │
│  │  ✅ Free Router (12mo)    │  │
│  │  [Select Plan]            │  │
│  └───────────────────────────┘  │
│                                 │
│  ┌───────────────────────────┐  │
│  │  ⭐ AeroXe 100    ₹699/mo │  │
│  │  ⬇️ 100 Mbps ⬆️ 50 Mbps  │  │
│  │  Unlimited Data           │  │
│  │  ✅ Priority Support      │  │
│  │  [Current Plan]           │  │
│  └───────────────────────────┘  │
│                                 │
│  ┌───────────────────────────┐  │
│  │  AeroXe 150    ₹999/mo   │  │
│  │  ⬇️ 150 Mbps ⬆️ 75 Mbps  │  │
│  │  Unlimited Data           │  │
│  │  ✅ Business Grade        │  │
│  │  [Select Plan]            │  │
│  └───────────────────────────┘  │
│                                 │
│  ┌───────────────────────────┐  │
│  │  AeroXe 200   ₹1499/mo   │  │
│  │  ⬇️ 200 Mbps ⬆️ 100 Mbps │  │
│  │  Unlimited Data           │  │
│  │  ✅ Dedicated Support     │  │
│  │  [Select Plan]            │  │
│  └───────────────────────────┘  │
│                                 │
└─────────────────────────────────┘
```

### Plan Change Confirmation
```
┌─────────────────────────────────┐
│  ← Confirm Plan Change          │
├─────────────────────────────────┤
│                                 │
│  ┌───────────────────────────┐  │
│  │  AeroXe 100 → AeroXe 150 │  │
│  │                           │  │
│  │  ⬇️ 100 → 150 Mbps       │  │
│  │  ⬆️ 50 → 75 Mbps         │  │
│  │                           │  │
│  │  ₹699 → ₹999/month       │  │
│  │  Difference: +₹300/month  │  │
│  │                           │  │
│  │  Effective: Immediately   │  │
│  │  Pro-rata: ₹150 for 15 days│ │
│  └───────────────────────────┘  │
│                                 │
│  ⚠️ Pro-rata billing applies.  │
│  You will be charged ₹150 for  │
│  the remaining 15 days of your │
│  current billing cycle.         │
│                                 │
│  ┌─────────────────────────┐   │
│  │    Confirm Change →     │   │
│  └─────────────────────────┘   │
│                                 │
│        Cancel                   │
└─────────────────────────────────┘
```

---

## API Endpoints

> **API Convention:** Protobuf-first. See `docs/backend/API-CONVENTIONS.md`.

### Get Current Subscription
```
POST /api/v1/customer/subscription/get

Response 200:
{
  "subscription": {
    "id": "sub_abc123",
    "status": "active",
    "plan": {
      "id": "plan_100",
      "name": "AeroXe 100",
      "speed_download_mbps": 100,
      "speed_upload_mbps": 50,
      "monthly_price": 699,
      "data_cap_gb": 200,
      "features": [
        "Unlimited Data",
        "Free Installation",
        "24/7 Support",
        "Dual Band WiFi Router",
        "Priority Support"
      ]
    },
    "start_date": "2025-07-15",
    "next_billing_date": "2026-07-15",
    "auto_renew": true,
    "billing_cycle": "monthly"
  }
}
```

### Browse Plans
```
POST /api/v1/plans/list

Response 200:
{
  "plans": [
    {
      "id": "plan_50",
      "name": "AeroXe 50",
      "speed_download_mbps": 50,
      "speed_upload_mbps": 25,
      "monthly_price": 400,
      "data_cap_gb": 200,
      "features": [
        "Unlimited Data",
        "Free Installation",
        "24/7 Support"
      ],
      "free_router": true,
      "router_conditions": "12-month plans only",
      "is_popular": false
    },
    {
      "id": "plan_100",
      "name": "AeroXe 100",
      "speed_download_mbps": 100,
      "speed_upload_mbps": 50,
      "monthly_price": 699,
      "data_cap_gb": 200,
      "features": [
        "Unlimited Data",
        "Free Installation",
        "24/7 Support",
        "Dual Band WiFi Router",
        "Priority Support"
      ],
      "free_router": true,
      "router_conditions": "12-month plans only",
      "is_popular": true
    }
    // ... more plans
  ]
}
```

### Request Plan Change
```
POST /api/v1/customer/subscription/upgrade

Request:
{
  "new_plan_id": "plan_150",
  "effective_date": "immediate"  // or "next_billing_cycle"
}

Response 200:
{
  "success": true,
  "change": {
    "current_plan": "AeroXe 100",
    "new_plan": "AeroXe 150",
    "effective_date": "2026-07-08",
    "prorated_charge": 150.00,
    "new_monthly_price": 999,
    "next_billing_date": "2026-07-15",
    "message": "Plan changed successfully. Pro-rata charge of ₹150 applied."
  }
}
```

---

## Android Implementation

### PlansViewModel.kt
```kotlin
@HiltViewModel
class PlansViewModel @Inject constructor(
    private val plansRepository: PlansRepository
) : ViewModel() {
    
    private val _uiState = MutableStateFlow(PlansUiState())
    val uiState: StateFlow<PlansUiState> = _uiState.asStateFlow()
    
    init {
        loadPlans()
    }
    
    private fun loadPlans() {
        viewModelScope.launch {
            _uiState.update { it.copy(isLoading = true) }
            
            // Load both current subscription and available plans
            val subscriptionDeferred = async { plansRepository.getCurrentSubscription() }
            val plansDeferred = async { plansRepository.getAvailablePlans() }
            
            val subscription = subscriptionDeferred.await()
            val plans = plansDeferred.await()
            
            subscription.onSuccess { sub ->
                plans.onSuccess { availablePlans ->
                    _uiState.update {
                        it.copy(
                            isLoading = false,
                            currentSubscription = sub,
                            availablePlans = availablePlans
                        )
                    }
                }
            }
        }
    }
    
    fun selectPlan(plan: Plan) {
        _uiState.update { it.copy(selectedPlan = plan, showConfirmation = true) }
    }
    
    fun confirmPlanChange(effectiveDate: String) {
        viewModelScope.launch {
            val plan = _uiState.value.selectedPlan ?: return@launch
            _uiState.update { it.copy(isChangingPlan = true) }
            
            plansRepository.requestPlanChange(plan.id, effectiveDate)
                .onSuccess { result ->
                    _uiState.update {
                        it.copy(
                            isChangingPlan = false,
                            showConfirmation = false,
                            changeResult = result,
                            showSuccess = true
                        )
                    }
                    loadPlans() // Refresh
                }
                .onFailure { error ->
                    _uiState.update {
                        it.copy(isChangingPlan = false, error = error.message)
                    }
                }
        }
    }
    
    fun dismissConfirmation() {
        _uiState.update { it.copy(showConfirmation = false, selectedPlan = null) }
    }
}

data class PlansUiState(
    val isLoading: Boolean = true,
    val currentSubscription: Subscription? = null,
    val availablePlans: List<Plan> = emptyList(),
    val selectedPlan: Plan? = null,
    val showConfirmation: Boolean = false,
    val isChangingPlan: Boolean = false,
    val changeResult: PlanChangeResult? = null,
    val showSuccess: Boolean = false,
    val error: String? = null
)
```

---

## iOS Implementation

### PlansViewModel.swift
```swift
@Observable
class PlansViewModel {
    var isLoading: Bool = true
    var currentSubscription: Subscription?
    var availablePlans: [Plan] = []
    var selectedPlan: Plan?
    var showConfirmation: Bool = false
    var isChangingPlan: Bool = false
    var changeResult: PlanChangeResult?
    var showSuccess: Bool = false
    var error: String?
    
    private let plansRepository: PlansRepositoryProtocol
    
    init(plansRepository: PlansRepositoryProtocol = PlansRepository()) {
        self.plansRepository = plansRepository
        Task { await loadPlans() }
    }
    
    @MainActor
    func loadPlans() async {
        isLoading = true
        defer { isLoading = false }
        
        do {
            async let subscription = plansRepository.getCurrentSubscription()
            async let plans = plansRepository.getAvailablePlans()
            
            currentSubscription = try await subscription
            availablePlans = try await plans
        } catch {
            self.error = error.localizedDescription
        }
    }
    
    func selectPlan(_ plan: Plan) {
        selectedPlan = plan
        showConfirmation = true
    }
    
    func confirmPlanChange(effectiveDate: String) async {
        guard let plan = selectedPlan else { return }
        isChangingPlan = true
        defer { isChangingPlan = false }
        
        do {
            changeResult = try await plansRepository.requestPlanChange(
                planId: plan.id,
                effectiveDate: effectiveDate
            )
            showConfirmation = false
            showSuccess = true
            await loadPlans()
        } catch {
            self.error = error.localizedDescription
        }
    }
}
```

---

## Plan Comparison

When customer taps "Compare Plans":

```
┌─────────────────────────────────┐
│  ← Plan Comparison              │
├─────────────────────────────────┤
│                                 │
│  Feature        50    100   150 │
│  ─────────────────────────────  │
│  Price      ₹400  ₹699  ₹999  │
│  Download    50   100    150   │
│  Upload      25    50     75   │
│  Data      200GB 200GB  200GB  │
│  Router       ✓     ✓      ✓   │
│  Priority     ✗     ✓      ✓   │
│  Business     ✗     ✗      ✓   │
│  ─────────────────────────────  │
│                                 │
│  [Select AeroXe 150 →]         │
└─────────────────────────────────┘
```
