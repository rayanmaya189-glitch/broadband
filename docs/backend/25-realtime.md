# AeroXe Backend — Realtime Module

> **Req Ref:** §10 Realtime System

---

## 1. Overview

WebSocket-based real-time system for the NOC dashboard and customer portal. Provides live updates for device status, customer sessions, ticket escalations, and network health. Uses Redis Pub/Sub for message distribution.

## 2. Architecture

```
Client (Browser)
    ↓ WebSocket Connection
Axum WebSocket Handler
    ↓
Redis Pub/Sub
    ↓
Message Router
    ├── NOC Dashboard (device status, sessions, alerts)
    ├── Customer Portal (invoice, ticket updates)
    └── Admin Portal (real-time metrics)
```

## 3. WebSocket Channels

| Channel | Subscribers | Data |
|---------|-------------|------|
| `ws:customer:{id}` | Customer | Invoice, ticket, subscription updates |
| `ws:branch:{id}` | Branch staff | Branch-wide alerts, ticket escalations |
| `ws:noc:alerts` | NOC engineers | Device alerts, SLA breaches, outages |
| `ws:noc:devices` | NOC engineers | Device status changes, health scores |
| `ws:noc:sessions` | NOC engineers | Customer online/offline status |
| `ws:noc:discovery` | NOC engineers | New device discoveries |
| `ws:admin:metrics` | Admins | Real-time KPIs, revenue, subscriber count |

## 4. WebSocket Handler

```rust
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<SharedState>,
    user: UserContext,
) -> Response {
    ws.on_upgrade(move |socket| handle_socket(socket, state, user))
}

async fn handle_socket(socket: WebSocket, state: SharedState, user: UserContext) {
    let (mut sender, mut receiver) = socket.split();

    // Determine channels based on user role and branch
    let channels = resolve_channels(&user);

    // Subscribe to Redis Pub/Sub channels
    let mut pubsub = state.redis.get_async_subscriber().await?;
    for channel in &channels {
        pubsub.subscribe(channel).await?;
    }

    // Spawn task to forward Redis messages to WebSocket
    tokio::spawn(async move {
        while let Some(msg) = pubsub.on_message().next().await {
            let payload = msg.get_payload::<String>().unwrap();
            sender.send(Message::Text(payload)).await.ok();
        }
    });

    // Handle incoming WebSocket messages (client → server)
    while let Some(Ok(msg)) = receiver.next().await {
        match msg {
            Message::Text(text) => handle_client_message(&state, &user, &text).await,
            Message::Close(_) => break,
            _ => {}
        }
    }
}
```

## 5. Message Format

```json
{
  "type": "device.status.changed",
  "channel": "ws:noc:devices",
  "data": {
    "device_id": 42,
    "device_name": "Jalgaon-CityCenter-OLT-01",
    "old_status": "online",
    "new_status": "degraded",
    "health_score": 45,
    "timestamp": "2026-07-08T14:30:00Z"
  }
}
```

## 6. Redis Pub/Sub Integration

```rust
pub struct RealtimeBroadcaster {
    redis: redis::Client,
}

impl RealtimeBroadcaster {
    pub async fn broadcast(&self, channel: &str, message: &str) -> Result<()> {
        let mut conn = self.redis.get_async_connection().await?;
        conn.publish(channel, message).await?;
        Ok(())
    }

    pub async fn broadcast_to_branch(&self, branch_id: i64, message: &str) -> Result<()> {
        self.broadcast(&format!("ws:branch:{}", branch_id), message).await
    }

    pub async fn broadcast_to_noc(&self, channel: &str, message: &str) -> Result<()> {
        self.broadcast(&format!("ws:noc:{}", channel), message).await
    }
}
```

## 7. Heartbeat & Reconnection

```
Client sends ping every 30s
    ↓
Server responds with pong
    ↓
If no ping for 60s → close connection
    ↓
Client reconnects with exponential backoff
    ↓
Resubscribe to channels
```

## 8. API Endpoints

| Method | Path | Auth | Description |
|--------|------|------|-------------|
| GET | `/ws` | WebSocket | WebSocket upgrade endpoint |
| GET | `/api/v1/realtime/health` | Yes | WebSocket health check |
| GET | `/api/v1/realtime/channels` | Yes | List available channels |

## 9. Security

- WebSocket connections require valid JWT token (passed as query param or first message)
- Channel access is role-based (NOC channels only for noc_engineer+)
- Rate limit: 100 messages/minute per connection
- Max connections per user: 3

## 10. RBAC Permissions

```
realtime.connect
realtime.noc.alerts
realtime.noc.devices
realtime.noc.sessions
realtime.noc.discovery
realtime.admin.metrics
realtime.customer.updates
```
