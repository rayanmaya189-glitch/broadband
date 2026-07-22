# 00 — Mobile Architecture Overview

> **API Convention:** Protobuf-first. See `docs/backend/API-CONVENTIONS.md`.

## Overview

Both the Customer Android (§17) and Customer iOS (§18) applications share a feature-for-feature identical design, using the same API endpoints and data models. The only differences are platform-specific implementations.

## Shared Principles

| Principle | Description |
|-----------|-------------|
| Clean Architecture | Strict separation: Data → Domain → Presentation |
| Feature-first modules | Each screen/feature is a self-contained module |
| Single source of truth | Remote API is primary; local DB is cache |
| Unidirectional data flow | State flows down, events flow up (MVI pattern) |
| API contract sharing | Both apps consume identical REST endpoints |

---

## API Endpoints (Shared)

All mobile features consume these backend API endpoints:

### Authentication
| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | `/api/v1/auth/otp/send` | Send OTP to phone |
| POST | `/api/v1/auth/otp/verify` | Verify OTP, return JWT |
| POST | `/api/v1/auth/refresh` | Refresh access token |
| POST | `/api/v1/auth/logout` | Invalidate session |

### Dashboard & Usage
| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | `/api/v1/customer/dashboard` | Dashboard summary |
| POST | `/api/v1/customer/usage` | Usage stats & charts |
| POST | `/api/v1/customer/usage/realtime` | WebSocket: real-time bandwidth |

### Plans & Subscriptions
| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | `/api/v1/customer/subscription/get` | Current plan details |
| POST | `/api/v1/plans/list` | Browse available plans |
| POST | `/api/v1/customer/subscription/upgrade` | Request plan change |

### Billing & Payments
| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | `/api/v1/customer/invoices/list` | Invoice list |
| POST | `/api/v1/customer/invoices/get` | Invoice detail |
| POST | `/api/v1/customer/invoices/pdf` | Download PDF |
| POST | `/api/v1/customer/payments` | Initiate payment (Razorpay) |
| POST | `/api/v1/customer/payments/verify` | Verify payment callback |

### Support Tickets
| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | `/api/v1/customer/tickets/list` | List tickets |
| POST | `/api/v1/customer/tickets/create` | Create ticket |
| POST | `/api/v1/customer/tickets/get` | Ticket detail |
| POST | `/api/v1/customer/tickets/messages` | Add message |
| POST | `/api/v1/customer/tickets/close` | Close ticket |

### Notifications
| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | `/api/v1/customer/notifications/list` | Notification list |
| POST | `/api/v1/customer/notifications/read` | Mark as read |
| POST | `/api/v1/customer/notifications/read-all` | Mark all read |
| POST | `/api/v1/customer/devices/fcm` | Register FCM token |
| POST | `/api/v1/customer/devices/apns` | Register APNs token |

### Profile & Documents
| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | `/api/v1/customer/profile/get` | Get profile |
| PATCH | `/api/v1/customer/profile/update` | Update profile |
| POST | `/api/v1/customer/profile/kyc-status` | KYC verification status |
| POST | `/api/v1/customer/documents/upload-url` | Get presigned upload URL |
| POST | `/api/v1/customer/documents/confirm-upload` | Confirm upload complete |
| POST | `/api/v1/customer/documents/list` | List uploaded documents |

### Settings
| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | `/api/v1/customer/settings/get` | Get preferences |
| PATCH | `/api/v1/customer/settings/update` | Update preferences |

---

## Authentication Flow (Shared)

```
┌─────────────┐    ┌──────────────┐    ┌─────────────┐    ┌──────────────┐
│  Enter Phone │───▶│  Send OTP    │───▶│  Verify OTP │───▶│  Dashboard   │
└─────────────┘    └──────────────┘    └─────────────┘    └──────────────┘
                                              │
                                              ▼
                                    ┌──────────────────┐
                                    │  Store JWT pair   │
                                    │  (access + refresh)│
                                    └──────────────────┘
                                              │
                                              ▼
                                    ┌──────────────────┐
                                    │  Enable biometric │
                                    │  (optional)       │
                                    └──────────────────┘
```

### JWT Token Management
- **Access Token**: 15-minute expiry, RS256 signed
- **Refresh Token**: 7-day expiry, stored in secure storage
- **Token Refresh**: Automatic refresh 60 seconds before expiry
- **Logout**: Invalidate refresh token on server

### Biometric Authentication
- **Android**: `BiometricPrompt` API (fingerprint/face)
- **iOS**: `LAContext` (Face ID / Touch ID)
- Fallback to phone+OTP if biometric fails 3 times

---

## Error Handling (Shared)

| HTTP Status | Handling |
|-------------|----------|
| 200 | Success |
| 400 | Show validation errors inline |
| 401 | Attempt token refresh; if fails → re-authenticate |
| 403 | Show permission denied message |
| 404 | Show not found / empty state |
| 429 | Show rate limit message, retry after delay |
| 500+ | Show generic error, allow retry |

---

## Offline Strategy (Shared)

| Data Type | Cache Duration | Sync Strategy |
|-----------|---------------|---------------|
| Plan Details | 24 hours | Background sync on app open |
| Invoice List | 1 hour | Pull-to-refresh |
| Usage Data | 5 minutes | Polling + WebSocket |
| Profile | 24 hours | Background sync |
| Notifications | 15 minutes | FCM + background sync |
| Settings | 24 hours | Background sync |

---

## Project Structure Comparison

```
Android (Kotlin)                    iOS (Swift)
─────────────────                   ────────────
app/src/main/java/                  AeroXe/
├── data/                           ├── Data/
│   ├── remote/                     │   ├── Network/
│   ├── local/                      │   ├── Storage/
│   └── repository/                 │   └── Repository/
├── domain/                         ├── Domain/
│   ├── model/                      │   ├── Models/
│   ├── repository/                 │   ├── Repository/
│   └── usecase/                    │   └── UseCases/
├── presentation/                   ├── Presentation/
│   ├── theme/                      │   ├── Theme/
│   ├── navigation/                 │   ├── Navigation/
│   ├── auth/                       │   ├── Auth/
│   ├── dashboard/                  │   ├── Dashboard/
│   ├── usage/                      │   ├── Usage/
│   ├── plans/                      │   ├── Plans/
│   ├── billing/                    │   ├── Billing/
│   ├── tickets/                    │   ├── Tickets/
│   ├── notifications/              │   ├── Notifications/
│   ├── profile/                    │   ├── Profile/
│   └── settings/                   │   └── Settings/
├── di/                             ├── Core/
│                                   │   ├── DI/
│                                   │   └── Utils/
└── utils/                          └── Helpers/
```

---

## Dependencies Comparison

| Concern | Android | iOS |
|---------|---------|-----|
| UI Framework | Jetpack Compose | SwiftUI |
| DI | Hilt | Factory / Manual |
| Networking | Retrofit + OkHttp | URLSession + async/await |
| Local DB | Room | SwiftData / CoreData |
| Navigation | Navigation Compose | NavigationStack |
| State | StateFlow + MVI | @Observable / Combine |
| Image Loading | Coil | AsyncImage |
| Push Notifications | FCM | APNs |
| Background Tasks | WorkManager | BackgroundTasks |
| Biometric | BiometricPrompt | LAContext |
| Payment | Razorpay SDK | Razorpay SDK |
| Analytics | Firebase Analytics | Firebase Analytics |
| Crash Reporting | Firebase Crashlytics | Firebase Crashlytics |

---

## WebSocket Protocol (Shared)

Both apps connect to the same WebSocket for real-time data:

```
Connection: wss://api.aeroxebroadband.com/ws/usage?token={access_token}

Messages (Server → Client):
{
  "type": "usage_update",
  "data": {
    "download_speed_mbps": 45.2,
    "upload_speed_mbps": 12.1,
    "data_used_gb": 123.4,
    "timestamp": "2026-07-08T10:30:00Z"
  }
}

Messages (Client → Server):
{
  "type": "subscribe",
  "channels": ["usage", "notifications"]
}

Heartbeat (every 30s):
{
  "type": "ping"
}
Response:
{
  "type": "pong"
}
```

---

## Document Upload Flow (Shared)

```
┌────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│  Select File    │───▶│  Validate Locally │───▶│ Request Presigned│
│  (Camera/Gallery│    │  (type, size)     │    │  Upload URL      │
│   / File Picker)│    └──────────────────┘    └─────────────────┘
└────────────────┘                                    │
                                                      ▼
                                            ┌──────────────────┐
                                            │ Upload to MinIO   │
                                            │ (direct PUT)      │
                                            └──────────────────┘
                                                      │
                                                      ▼
                                            ┌──────────────────┐
                                            │ Confirm Upload    │
                                            │ (POST to backend) │
                                            └──────────────────┘
```

### Customer Upload Limits
- **Allowed formats**: `.jpg`, `.jpeg`, `.png`, `.webp`, `.pdf`
- **Max file size**: 10 MB per file
- **Max files per request**: 5
- **Max total per entity**: 50 MB
- **EXIF data**: Stripped from images
- **Virus scan**: Optional ClamAV integration

---

## Implementation Priority

| Phase | Modules | Timeline |
|-------|---------|----------|
| **Phase 1** | Auth, Dashboard, Profile | Weeks 1-3 |
| **Phase 2** | Plans, Billing, Invoices | Weeks 3-5 |
| **Phase 3** | Usage, Notifications, Settings | Weeks 5-7 |
| **Phase 4** | Tickets, Documents, Offline | Weeks 7-9 |
| **Phase 5** | Testing, Polish, Release | Weeks 9-10 |
