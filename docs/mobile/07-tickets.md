# 07 — Support Tickets Module

## Overview

Customers can create, view, update, and close support tickets. Tickets follow a lifecycle: Open → In Progress → Resolved → Closed.

---

## Screen Layout

### Tickets List Screen
```
┌─────────────────────────────────┐
│  ← Support Tickets         🎫   │
├─────────────────────────────────┤
│                                 │
│  [All] [Open] [In Progress] [Closed] │
│                                 │
│  ┌───────────────────────────┐  │
│  │  🟡 Slow Internet Speed   │  │
│  │  #TKT-2026-0045          │  │
│  │  Opened: Jul 8, 2026     │  │
│  │  Status: 🟡 Open         │  │
│  │  Priority: Medium         │  │
│  │  [View →]                 │  │
│  └───────────────────────────┘  │
│                                 │
│  ┌───────────────────────────┐  │
│  │  🔵 Router Not Working    │  │
│  │  #TKT-2026-0042          │  │
│  │  Opened: Jul 5, 2026     │  │
│  │  Status: 🔵 In Progress  │  │
│  │  Priority: High           │  │
│  │  [View →]                 │  │
│  └───────────────────────────┘  │
│                                 │
│  ┌───────────────────────────┐  │
│  │  ✅ Bill Dispute          │  │
│  │  #TKT-2026-0038          │  │
│  │  Opened: Jun 28, 2026    │  │
│  │  Status: ✅ Resolved     │  │
│  │  [View →]                 │  │
│  └───────────────────────────┘  │
│                                 │
│  ┌─────────────────────────┐   │
│  │    + Create New Ticket   │   │
│  └─────────────────────────┘   │
└─────────────────────────────────┘
```

### Ticket Detail Screen
```
┌─────────────────────────────────┐
│  ← Ticket #TKT-2026-0045       │
├─────────────────────────────────┤
│                                 │
│  Slow Internet Speed            │
│  Status: 🟡 Open                │
│  Priority: Medium               │
│  Category: Speed Issue          │
│  Created: Jul 8, 2026          │
│                                 │
│  ── Conversation ───────────    │
│                                 │
│  ┌───────────────────────────┐  │
│  │  👤 You                   │  │
│  │  Jul 8, 10:30 AM          │  │
│  │                           │  │
│  │  My internet speed has    │  │
│  │  been very slow since     │  │
│  │  morning. Getting only    │  │
│  │  5-10 Mbps instead of     │  │
│  │  100 Mbps.                │  │
│  └───────────────────────────┘  │
│                                 │
│  ┌───────────────────────────┐  │
│  │  🛠️ Support Agent         │  │
│  │  Jul 8, 11:15 AM          │  │
│  │                           │  │
│  │  Hi Rahul, we've checked  │  │
│  │  your connection and      │  │
│  │  found a temporary issue. │  │
│  │  We're working on it.     │  │
│  └───────────────────────────┘  │
│                                 │
│  ┌───────────────────────────┐  │
│  │  📎 photo.jpg             │  │
│  │  [View Attachment]        │  │
│  └───────────────────────────┘  │
│                                 │
│  ── Add Message ────────────    │
│                                 │
│  ┌───────────────────────────┐  │
│  │  Type your message...     │  │
│  │                           │  │
│  └───────────────────────────┘  │
│  [📎] [📷] [Send →]            │
│                                 │
└─────────────────────────────────┘
```

### Create Ticket Screen
```
┌─────────────────────────────────┐
│  ← New Support Ticket           │
├─────────────────────────────────┤
│                                 │
│  Category:                      │
│  [Select Category ▼]            │
│  ─ Speed Issue                  │
│  ─ Connection Problem           │
│  ─ Billing Dispute              │
│  ─ Router Issue                 │
│  ─ Installation                 │
│  ─ Other                        │
│                                 │
│  Priority:                      │
│  ○ Low  ○ Medium  ● High       │
│                                 │
│  Subject:                       │
│  ┌───────────────────────────┐  │
│  │ Slow internet speed       │  │
│  └───────────────────────────┘  │
│                                 │
│  Description:                   │
│  ┌───────────────────────────┐  │
│  │ My internet speed has     │  │
│  │ been very slow since      │  │
│  │ morning...                │  │
│  │                           │  │
│  └───────────────────────────┘  │
│                                 │
│  Attachments:                   │
│  [📎 Add Photo/Document]        │
│  (Max 5 files, 10MB each)      │
│                                 │
│  ┌─────────────────────────┐   │
│  │    Submit Ticket →       │   │
│  └─────────────────────────┘   │
└─────────────────────────────────┘
```

---

## API Endpoints

### List Tickets
```
POST /api/v1/customer/tickets/list

Response 200:
{
  "tickets": [
    {
      "id": "tkt_abc123",
      "ticket_number": "TKT-2026-0045",
      "subject": "Slow Internet Speed",
      "category": "speed_issue",
      "priority": "medium",
      "status": "open",
      "created_at": "2026-07-08T10:30:00Z",
      "last_reply_at": "2026-07-08T11:15:00Z",
      "message_count": 2,
      "has_attachments": false
    }
  ],
  "pagination": { "page": 1, "limit": 20, "total": 5 }
}
```

### Create Ticket
```
POST /api/v1/customer/tickets

Request (multipart/form-data):
{
  "subject": "Slow Internet Speed",
  "category": "speed_issue",
  "priority": "medium",
  "description": "My internet speed has been very slow...",
  "attachments[]": [file1.jpg, file2.png]
}

Response 201:
{
  "ticket": {
    "id": "tkt_abc123",
    "ticket_number": "TKT-2026-0045",
    "status": "open",
    "created_at": "2026-07-08T10:30:00Z"
  }
}
```

### Get Ticket Detail
```
POST /api/v1/customer/tickets/get

Response 200:
{
  "ticket": {
    "id": "tkt_abc123",
    "ticket_number": "TKT-2026-0045",
    "subject": "Slow Internet Speed",
    "category": "speed_issue",
    "priority": "medium",
    "status": "open",
    "messages": [
      {
        "id": "msg_001",
        "sender": {
          "name": "Rahul Patil",
          "role": "customer"
        },
        "content": "My internet speed has been very slow since morning...",
        "attachments": [],
        "created_at": "2026-07-08T10:30:00Z"
      },
      {
        "id": "msg_002",
        "sender": {
          "name": "Agent Priya",
          "role": "support_agent"
        },
        "content": "Hi Rahul, we've checked your connection...",
        "attachments": [
          {
            "id": "att_001",
            "filename": "photo.jpg",
            "url": "https://minio.aeroxebroadband.com/tickets/att_001.jpg",
            "size": 1024000
          }
        ],
        "created_at": "2026-07-08T11:15:00Z"
      }
    ]
  }
}
```

### Add Message
```
POST /api/v1/customer/tickets/:id/messages

Request (multipart/form-data):
{
  "content": "Thank you for the quick response!",
  "attachments[]": [screenshot.png]
}

Response 201:
{
  "message": {
    "id": "msg_003",
    "created_at": "2026-07-08T12:00:00Z"
  }
}
```

### Close Ticket
```
POST /api/v1/customer/tickets/:id/close

Response 200:
{
  "success": true,
  "ticket": {
    "status": "closed",
    "closed_at": "2026-07-08T12:30:00Z"
  }
}
```

---

## Android Implementation

### TicketsViewModel.kt
```kotlin
@HiltViewModel
class TicketsViewModel @Inject constructor(
    private val ticketsRepository: TicketsRepository
) : ViewModel() {
    
    private val _uiState = MutableStateFlow(TicketsUiState())
    val uiState: StateFlow<TicketsUiState> = _uiState.asStateFlow()
    
    private var currentPage = 1
    
    init {
        loadTickets()
    }
    
    private fun loadTickets(reset: Boolean = false) {
        viewModelScope.launch {
            if (reset) currentPage = 1
            _uiState.update { it.copy(isLoading = true) }
            
            ticketsRepository.getTickets(page = currentPage, status = _uiState.value.filter)
                .onSuccess { result ->
                    _uiState.update {
                        it.copy(
                            isLoading = false,
                            tickets = if (reset) result.tickets else it.tickets + result.tickets,
                            hasMore = result.pagination.currentPage < result.pagination.totalPages
                        )
                    }
                    currentPage++
                }
        }
    }
    
    fun filterByStatus(status: String?) {
        _uiState.update { it.copy(filter = status, tickets = emptyList()) }
        loadTickets(reset = true)
    }
    
    fun loadMore() {
        if (!_uiState.value.isLoading && _uiState.value.hasMore) {
            loadTickets()
        }
    }
}

data class TicketsUiState(
    val isLoading: Boolean = true,
    val tickets: List<Ticket> = emptyList(),
    val filter: String? = null,
    val hasMore: Boolean = false,
    val error: String? = null
)
```

### CreateTicketViewModel.kt
```kotlin
@HiltViewModel
class CreateTicketViewModel @Inject constructor(
    private val ticketsRepository: TicketsRepository
) : ViewModel() {
    
    private val _uiState = MutableStateFlow(CreateTicketUiState())
    val uiState: StateFlow<CreateTicketUiState> = _uiState.asStateFlow()
    
    fun updateSubject(subject: String) {
        _uiState.update { it.copy(subject = subject) }
    }
    
    fun updateCategory(category: String) {
        _uiState.update { it.copy(category = category) }
    }
    
    fun updatePriority(priority: String) {
        _uiState.update { it.copy(priority = priority) }
    }
    
    fun updateDescription(description: String) {
        _uiState.update { it.copy(description = description) }
    }
    
    fun addAttachment(uri: Uri) {
        _uiState.update { it.copy(attachments = it.attachments + uri) }
    }
    
    fun removeAttachment(index: Int) {
        _uiState.update {
            it.copy(attachments = it.attachments.toMutableList().apply { removeAt(index) })
        }
    }
    
    fun submitTicket() {
        viewModelScope.launch {
            _uiState.update { it.copy(isSubmitting = true) }
            
            ticketsRepository.createTicket(
                subject = _uiState.value.subject,
                category = _uiState.value.category,
                priority = _uiState.value.priority,
                description = _uiState.value.description,
                attachments = _uiState.value.attachments
            ).onSuccess { ticket ->
                _uiState.update { it.copy(isSubmitting = false, createdTicket = ticket) }
            }.onFailure { error ->
                _uiState.update { it.copy(isSubmitting = false, error = error.message) }
            }
        }
    }
}
```

---

## iOS Implementation

### TicketsViewModel.swift
```swift
@Observable
class TicketsViewModel {
    var isLoading: Bool = true
    var tickets: [Ticket] = []
    var filter: String?
    var hasMore: Bool = false
    var error: String?
    
    private let ticketsRepository: TicketsRepositoryProtocol
    private var currentPage: Int = 1
    
    init(ticketsRepository: TicketsRepositoryProtocol = TicketsRepository()) {
        self.ticketsRepository = ticketsRepository
        Task { await loadTickets(reset: true) }
    }
    
    @MainActor
    func loadTickets(reset: Bool = false) async {
        if reset { currentPage = 1 }
        isLoading = true
        defer { isLoading = false }
        
        do {
            let result = try await ticketsRepository.getTickets(
                page: currentPage,
                status: filter
            )
            if reset {
                tickets = result.tickets
            } else {
                tickets.append(contentsOf: result.tickets)
            }
            hasMore = result.pagination.currentPage < result.pagination.totalPages
            currentPage += 1
        } catch {
            self.error = error.localizedDescription
        }
    }
}
```

### TicketDetailView.swift
```swift
struct TicketDetailView: View {
    let ticketId: String
    @State private var viewModel: TicketDetailViewModel
    @State private var messageText: String = ""
    
    init(ticketId: String) {
        self.ticketId = ticketId
        _viewModel = State(initialValue: TicketDetailViewModel(ticketId: ticketId))
    }
    
    var body: some View {
        VStack(spacing: 0) {
            // Ticket header
            TicketHeader(ticket: viewModel.ticket)
            
            Divider()
            
            // Messages
            ScrollViewReader { proxy in
                ScrollView {
                    LazyVStack(spacing: 12) {
                        ForEach(viewModel.messages) { message in
                            MessageBubble(message: message)
                                .id(message.id)
                        }
                    }
                    .padding()
                }
                .onChange(of: viewModel.messages.count) {
                    withAnimation {
                        proxy.scrollTo(viewModel.messages.last?.id, anchor: .bottom)
                    }
                }
            }
            
            Divider()
            
            // Message input
            MessageInputBar(
                text: $messageText,
                onSend: {
                    Task {
                        await viewModel.sendMessage(content: messageText)
                        messageText = ""
                    }
                },
                onAttach: { /* Handle attachment */ }
            )
        }
        .navigationTitle("Ticket #\(viewModel.ticket?.ticketNumber ?? "")")
        .navigationBarTitleDisplayMode(.inline)
    }
}
```

---

## Ticket Categories

| Category | Icon | Description |
|----------|------|-------------|
| `speed_issue` | 🐌 | Slow speed, not meeting plan |
| `connection_problem` | 📡 | No internet, intermittent |
| `billing_dispute` | 💰 | Incorrect bill, charges |
| `router_issue` | 📶 | WiFi router problems |
| `installation` | 🔧 | Installation issues |
| `other` | 📝 | General inquiries |

---

## Real-time Updates

Both apps receive real-time ticket updates via WebSocket:

```
WebSocket message:
{
  "type": "ticket_update",
  "data": {
    "ticket_id": "tkt_abc123",
    "event": "new_message",
    "message": {
      "id": "msg_002",
      "sender": { "name": "Agent Priya", "role": "support_agent" },
      "content": "Hi Rahul, we've checked your connection...",
      "created_at": "2026-07-08T11:15:00Z"
    }
  }
}
```
