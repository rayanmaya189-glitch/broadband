# 06 — Billing & Payments Module

## Overview

Invoice management, payment processing via Razorpay SDK, payment history, and receipt downloads. Both platforms use identical API contracts and Razorpay integration.

---

## Screen Layout

### Invoices List Screen
```
┌─────────────────────────────────┐
│  ← Invoices & Payments          │
├─────────────────────────────────┤
│                                 │
│  ┌───────────────────────────┐  │
│  │  💰 Amount Due: ₹699     │  │
│  │  Due: Jul 15, 2026       │  │
│  │  [Pay Now →]              │  │
│  └───────────────────────────┘  │
│                                 │
│  ── History ────────────────    │
│                                 │
│  ┌───────────────────────────┐  │
│  │  🧾 INV-2026-0012        │  │
│  │  Jul 15, 2026 • ₹699     │  │
│  │  Status: ⏳ Pending       │  │
│  │  [Pay Now] [View →]       │  │
│  └───────────────────────────┘  │
│                                 │
│  ┌───────────────────────────┐  │
│  │  🧾 INV-2026-0011        │  │
│  │  Jun 15, 2026 • ₹699     │  │
│  │  Status: ✅ Paid          │  │
│  │  [View →] [PDF ↓]         │  │
│  └───────────────────────────┘  │
│                                 │
│  ┌───────────────────────────┐  │
│  │  🧾 INV-2026-0010        │  │
│  │  May 15, 2026 • ₹699     │  │
│  │  Status: ✅ Paid          │  │
│  │  [View →] [PDF ↓]         │  │
│  └───────────────────────────┘  │
│                                 │
└─────────────────────────────────┘
```

### Invoice Detail Screen
```
┌─────────────────────────────────┐
│  ← Invoice Detail        📄    │
├─────────────────────────────────┤
│                                 │
│  ┌───────────────────────────┐  │
│  │  AEROXE BROADBAND        │  │
│  │  Invoice: INV-2026-0012  │  │
│  │  Date: Jul 15, 2026      │  │
│  │                           │  │
│  │  Bill To:                 │  │
│  │  Rahul Patil              │  │
│  │  +91 98765 43210          │  │
│  │  123 Main St, Jalgaon     │  │
│  │                           │  │
│  │  ─────────────────────    │  │
│  │  Description     Amount   │  │
│  │  ─────────────────────    │  │
│  │  AeroXe 100 Plan  ₹699   │  │
│  │  (Jul 15 - Aug 14)       │  │
│  │  ─────────────────────    │  │
│  │  Subtotal        ₹699    │  │
│  │  GST (18%)       ₹125.82 │  │
│  │  ─────────────────────    │  │
│  │  Total           ₹824.82 │  │
│  │  ─────────────────────    │  │
│  │                           │  │
│  │  Status: ⏳ Pending       │  │
│  └───────────────────────────┘  │
│                                 │
│  ┌─────────────────────────┐   │
│  │     Pay ₹824.82 →       │   │
│  └─────────────────────────┘   │
│                                 │
│  [Download PDF 📄]              │
└─────────────────────────────────┘
```

### Payment Processing (Razorpay)
```
┌─────────────────────────────────┐
│  ← Pay Invoice                  │
├─────────────────────────────────┤
│                                 │
│  Amount: ₹824.82                │
│  Invoice: INV-2026-0012         │
│                                 │
│  ── Payment Method ─────────    │
│                                 │
│  ○ UPI (Google Pay/PhonePe)     │
│  ○ Credit/Debit Card            │
│  ○ Net Banking                  │
│  ○ Wallets                      │
│                                 │
│  ┌─────────────────────────┐   │
│  │    Pay with Razorpay →   │   │
│  └─────────────────────────┘   │
│                                 │
│  🔒 Secured by Razorpay        │
└─────────────────────────────────┘
```

### Payment Success
```
┌─────────────────────────────────┐
│                                 │
│         ✅                      │
│                                 │
│    Payment Successful!          │
│                                 │
│  Amount Paid: ₹824.82          │
│  Payment ID: pay_xyz789         │
│  Date: Jul 8, 2026             │
│                                 │
│  ┌─────────────────────────┐   │
│  │   View Receipt →         │   │
│  └─────────────────────────┘   │
│                                 │
│  [Back to Invoices]             │
└─────────────────────────────────┘
```

---

## API Endpoints

> **API Convention:** Protobuf-first. See `docs/backend/API-CONVENTIONS.md`.

### Get Invoices
```
POST /api/v1/customer/invoices/list

Response 200:
{
  "invoices": [
    {
      "id": "inv_abc123",
      "invoice_number": "INV-2026-0012",
      "amount": 699.00,
      "gst_amount": 125.82,
      "total_amount": 824.82,
      "status": "pending",
      "due_date": "2026-07-15",
      "billing_period": {
        "from": "2026-07-15",
        "to": "2026-08-14"
      },
      "plan_name": "AeroXe 100",
      "created_at": "2026-07-01T00:00:00Z"
    }
  ],
  "pagination": {
    "page": 1,
    "limit": 20,
    "total": 12,
    "total_pages": 1
  }
}
```

### Get Invoice Detail
```
POST /api/v1/customer/invoices/get

Response 200:
{
  "invoice": {
    "id": "inv_abc123",
    "invoice_number": "INV-2026-0012",
    "customer": {
      "name": "Rahul Patil",
      "phone": "+919876543210",
      "address": "123 Main St, Jalgaon"
    },
    "line_items": [
      {
        "description": "AeroXe 100 Plan (Jul 15 - Aug 14)",
        "amount": 699.00
      }
    ],
    "subtotal": 699.00,
    "gst_rate": 18,
    "gst_amount": 125.82,
    "total_amount": 824.82,
    "status": "pending",
    "due_date": "2026-07-15",
    "payment_history": []
  }
}
```

### Initiate Payment
```
POST /api/v1/customer/payments

Request:
{
  "invoice_id": "inv_abc123",
  "payment_method": "upi",
  "amount": 824.82
}

Response 200:
{
  "payment": {
    "id": "pay_xyz789",
    "razorpay_order_id": "order_abc123",
    "amount": 824.82,
    "currency": "INR",
    "status": "created"
  }
}
```

### Verify Payment
```
POST /api/v1/customer/payments/verify

Request:
{
  "razorpay_order_id": "order_abc123",
  "razorpay_payment_id": "pay_xyz789",
  "razorpay_signature": "abc123..."
}

Response 200:
{
  "success": true,
  "payment": {
    "id": "pay_xyz789",
    "status": "completed",
    "paid_at": "2026-07-08T22:15:00Z",
    "invoice_status": "paid"
  }
}
```

### Download Invoice PDF
```
POST /api/v1/customer/invoices/pdf

Response 200: application/pdf (binary)
```

---

## Razorpay Integration

### Android (Razorpay SDK)
```kotlin
class PaymentManager @Inject constructor(
    private val paymentRepository: PaymentRepository
) {
    private var activity: Activity? = null
    
    fun startPayment(
        activity: Activity,
        orderId: String,
        amount: Double,
        customerName: String,
        customerPhone: String,
        onSuccess: (PaymentResult) -> Unit,
        onError: (String) -> Unit
    ) {
        this.activity = activity
        
        val checkout = Checkout()
        checkout.setKeyID("rzp_live_xxxxx") // From BuildConfig
        
        try {
            val options = JSONObject().apply {
                put("name", "AeroXe Broadband")
                put("description", "Invoice Payment")
                put("order_id", orderId)
                put("amount", (amount * 100).toInt()) // In paise
                put("currency", "INR")
                put("prefill", JSONObject().apply {
                    put("contact", customerPhone)
                    put("name", customerName)
                })
                put("theme", JSONObject().apply {
                    put("color", "#1565C0") // Brand color
                })
            }
            
            checkout.open(activity, options)
        } catch (e: Exception) {
            onError(e.message ?: "Payment failed")
        }
    }
    
    fun handlePaymentResult(
        requestCode: Int,
        resultCode: Int,
        data: Intent?,
        onSuccess: (PaymentResult) -> Unit,
        onError: (String) -> Unit
    ) {
        // Razorpay handles via ActivityResult
        // This is called from the Activity's onActivityResult
    }
}
```

### Razorpay Checkout Activity (AndroidManifest.xml)
```xml
<activity
    name="com.razorpay.CheckoutActivity"
    android:configChanges="keyboard|keyboardHidden|orientation|screenSize"
    android:theme="@style/CheckoutTheme" />
```

### iOS (Razorpay SDK)
```swift
import Razorpay

class PaymentManager: NSObject, RazorpayPaymentCompletionProtocol {
    private var razorpay: RazorpayCheckout?
    private var completion: ((Result<PaymentResult, Error>) -> Void)?
    
    func startPayment(
        orderId: String,
        amount: Double,
        customerName: String,
        customerPhone: String,
        completion: @escaping (Result<PaymentResult, Error>) -> Void
    ) {
        self.completion = completion
        razorpay = RazorpayCheckout.initWithKey(
            "rzp_live_xxxxx",
            delegate: self
        )
        
        let options: [String: Any] = [
            "amount": Int(amount * 100), // In paise
            "currency": "INR",
            "order_id": orderId,
            "name": "AeroXe Broadband",
            "description": "Invoice Payment",
            "prefill": [
                "contact": customerPhone,
                "name": customerName
            ],
            "theme": [
                "color": "#1565C0"
            ]
        ]
        
        razorpay?.open(options)
    }
    
    func onPaymentSuccess(paymentId: String) {
        completion?(.success(PaymentResult(paymentId: paymentId)))
    }
    
    func onPaymentError(code: Int, description: String) {
        completion?(.failure(PaymentError(code: code, message: description)))
    }
}
```

---

## Payment Flow State Machine

```
┌──────────────┐     ┌──────────────┐     ┌──────────────┐
│  Select       │────▶│  Create       │────▶│  Razorpay    │
│  Invoice      │     │  Order        │     │  Checkout    │
└──────────────┘     └──────────────┘     └──────┬───────┘
                                                  │
                                    ┌─────────────┴─────────────┐
                                    │                           │
                              Payment                      Payment
                              Success                      Failed
                                    │                           │
                                    ▼                           ▼
                            ┌──────────────┐           ┌──────────────┐
                            │  Verify       │           │  Show Error   │
                            │  Signature    │           │  Retry / Cancel│
                            └──────┬───────┘           └──────────────┘
                                   │
                           ┌───────┴───────┐
                           │               │
                       Verified        Invalid
                           │               │
                           ▼               ▼
                   ┌──────────────┐  ┌──────────────┐
                   │  Update       │  │  Show Error   │
                   │  Invoice      │  │  Contact      │
                   │  Status       │  │  Support      │
                   └──────────────┘  └──────────────┘
```

---

## Dunning Notifications

When payment fails or is overdue:

| Day | Action |
|-----|--------|
| Due date | Push notification: "Your invoice of ₹824.82 is due today" |
| Day 1 overdue | Push + SMS: "Payment overdue. Pay now to avoid service interruption" |
| Day 3 overdue | Push + SMS + Email: "Urgent: Pay within 24 hours" |
| Day 7 overdue | Service suspension warning |
| Day 14 overdue | Service suspended |

---

## Offline Handling

| Scenario | Behavior |
|----------|----------|
| No internet | Queue payment verification, retry when online |
| Payment timeout | Check payment status via API after 30s |
| Network drop mid-payment | Razorpay handles retries internally |
| Server error | Show error, allow retry |
