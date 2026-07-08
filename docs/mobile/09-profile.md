# 09 — Profile & KYC Module

## Overview

Customer profile management, KYC document upload, personal information editing, and KYC verification status tracking.

---

## Screen Layout

### Profile Screen
```
┌─────────────────────────────────┐
│  ← My Profile            👤     │
├─────────────────────────────────┤
│                                 │
│         ┌──────────┐            │
│         │    RP    │            │
│         │  (avatar)│            │
│         └──────────┘            │
│         Rahul Patil             │
│         +91 98765 43210         │
│                                 │
│  ── KYC Status ─────────────    │
│                                 │
│  ┌───────────────────────────┐  │
│  │  ✅ KYC Verified          │  │
│  │  Verified on: Jul 1, 2026 │  │
│  └───────────────────────────┘  │
│                                 │
│  (or if pending)                │
│  ┌───────────────────────────┐  │
│  │  ⏳ KYC Pending           │  │
│  │  [Upload Documents →]     │  │
│  └───────────────────────────┘  │
│                                 │
│  ── Personal Info ───────────   │
│                                 │
│  Name          Rahul Patil  >  │
│  Phone         +91 98765  >   │
│  Email         rahul@email >  │
│  DOB           15/03/1995  >  │
│  Gender        Male        >  │
│                                 │
│  ── Address ────────────────    │
│                                 │
│  123 Main Street               │
│  Jalgaon, Maharashtra 425001   │
│  [Edit Address →]              │
│                                 │
│  ── Documents ──────────────    │
│                                 │
│  Aadhaar Card     ✅ Verified  │
│  PAN Card         ✅ Verified  │
│  [View Documents →]            │
│                                 │
└─────────────────────────────────┘
```

### Edit Profile Screen
```
┌─────────────────────────────────┐
│  ← Edit Profile           ✓     │
├─────────────────────────────────┤
│                                 │
│  Full Name:                     │
│  ┌───────────────────────────┐  │
│  │ Rahul Patil               │  │
│  └───────────────────────────┘  │
│                                 │
│  Email:                         │
│  ┌───────────────────────────┐  │
│  │ rahul@email.com           │  │
│  └───────────────────────────┘  │
│                                 │
│  Date of Birth:                 │
│  ┌───────────────────────────┐  │
│  │ 15/03/1995                │  │
│  └───────────────────────────┘  │
│                                 │
│  Gender:                        │
│  [Male ▼]                       │
│                                 │
│  ── Address ────────────────    │
│                                 │
│  Street Address:                │
│  ┌───────────────────────────┐  │
│  │ 123 Main Street           │  │
│  └───────────────────────────┘  │
│                                 │
│  City:                          │
│  ┌───────────────────────────┐  │
│  │ Jalgaon                   │  │
│  └───────────────────────────┘  │
│                                 │
│  State:    [Maharashtra ▼]      │
│  Pincode:  ┌─────────────────┐  │
│            │ 425001           │  │
│            └─────────────────┘  │
│                                 │
│  ┌─────────────────────────┐   │
│  │    Save Changes →        │   │
│  └─────────────────────────┘   │
└─────────────────────────────────┘
```

### KYC Upload Screen
```
┌─────────────────────────────────┐
│  ← KYC Verification             │
├─────────────────────────────────┤
│                                 │
│  Upload your identity documents │
│  to complete verification.      │
│                                 │
│  ── Required Documents ─────    │
│                                 │
│  ┌───────────────────────────┐  │
│  │  📄 Aadhaar Card          │  │
│  │  Status: ✅ Uploaded      │  │
│  │  [Re-upload] [View]       │  │
│  └───────────────────────────┘  │
│                                 │
│  ┌───────────────────────────┐  │
│  │  📄 PAN Card              │  │
│  │  Status: ⏳ Pending       │  │
│  │  [Upload →]               │  │
│  └───────────────────────────┘  │
│                                 │
│  ── Upload Guidelines ──────    │
│                                 │
│  • Clear, readable photos       │
│  • All four corners visible     │
│  • No blur or glare             │
│  • Accepted: JPG, PNG, PDF      │
│  • Max size: 10 MB per file     │
│                                 │
│  ┌─────────────────────────┐   │
│  │  Submit for Verification │   │
│  └─────────────────────────┘   │
└─────────────────────────────────┘
```

---

## API Endpoints

### Get Profile
```
GET /api/v1/customer/profile

Response 200:
{
  "profile": {
    "id": "cust_abc123",
    "name": "Rahul Patil",
    "phone": "+919876543210",
    "email": "rahul@email.com",
    "date_of_birth": "1995-03-15",
    "gender": "male",
    "address": {
      "street": "123 Main Street",
      "city": "Jalgaon",
      "state": "Maharashtra",
      "pincode": "425001"
    },
    "kyc_status": "verified",
    "kyc_verified_at": "2026-07-01T10:00:00Z",
    "documents": [
      {
        "type": "aadhaar",
        "status": "verified",
        "uploaded_at": "2026-06-28T10:00:00Z"
      },
      {
        "type": "pan",
        "status": "verified",
        "uploaded_at": "2026-06-28T10:05:00Z"
      }
    ],
    "created_at": "2026-06-15T10:00:00Z"
  }
}
```

### Update Profile
```
PATCH /api/v1/customer/profile

Request:
{
  "name": "Rahul Patil",
  "email": "rahul.new@email.com",
  "date_of_birth": "1995-03-15",
  "gender": "male",
  "address": {
    "street": "123 Main Street",
    "city": "Jalgaon",
    "state": "Maharashtra",
    "pincode": "425001"
  }
}

Response 200:
{
  "success": true,
  "profile": { ... }
}
```

### Get KYC Status
```
GET /api/v1/customer/profile/kyc-status

Response 200:
{
  "kyc_status": "pending",
  "required_documents": [
    { "type": "aadhaar", "status": "uploaded", "verified": false },
    { "type": "pan", "status": "pending", "verified": false }
  ],
  "verification_notes": null
}
```

### Upload KYC Document
```
POST /api/v1/customer/documents/upload-url

Request:
{
  "document_type": "aadhaar",
  "file_name": "aadhaar_front.jpg",
  "file_size": 1024000,
  "content_type": "image/jpeg"
}

Response 200:
{
  "upload_url": "https://minio.aeroxebroadband.com/kyc/abc123?X-Amz-Signature=...",
  "document_id": "doc_abc123",
  "expires_in": 300
}

# Then upload directly to MinIO
PUT {upload_url}
Body: <binary file data>

# Then confirm
POST /api/v1/customer/documents/confirm-upload

Request:
{
  "document_id": "doc_abc123"
}

Response 200:
{
  "success": true,
  "document": {
    "id": "doc_abc123",
    "type": "aadhaar",
    "status": "pending_verification"
  }
}
```

---

## Android Implementation

### ProfileViewModel.kt
```kotlin
@HiltViewModel
class ProfileViewModel @Inject constructor(
    private val profileRepository: ProfileRepository,
    private val documentRepository: DocumentRepository
) : ViewModel() {
    
    private val _uiState = MutableStateFlow(ProfileUiState())
    val uiState: StateFlow<ProfileUiState> = _uiState.asStateFlow()
    
    init {
        loadProfile()
    }
    
    private fun loadProfile() {
        viewModelScope.launch {
            _uiState.update { it.copy(isLoading = true) }
            profileRepository.getProfile()
                .onSuccess { profile ->
                    _uiState.update {
                        it.copy(
                            isLoading = false,
                            profile = profile,
                            kycStatus = profile.kycStatus,
                            documents = profile.documents
                        )
                    }
                }
        }
    }
    
    fun updateProfile(name: String, email: String, dob: String, gender: String, address: Address) {
        viewModelScope.launch {
            _uiState.update { it.copy(isSaving = true) }
            profileRepository.updateProfile(
                name = name, email = email, dateOfBirth = dob,
                gender = gender, address = address
            ).onSuccess { profile ->
                _uiState.update { it.copy(isSaving = false, profile = profile, showSuccess = true) }
            }.onFailure { error ->
                _uiState.update { it.copy(isSaving = false, error = error.message) }
            }
        }
    }
    
    fun uploadDocument(type: String, uri: Uri) {
        viewModelScope.launch {
            _uiState.update { it.copy(isUploading = true, uploadingType = type) }
            documentRepository.uploadDocument(type, uri)
                .onSuccess { doc ->
                    _uiState.update {
                        it.copy(
                            isUploading = false,
                            uploadingType = null,
                            documents = it.documents.map { d ->
                                if (d.type == type) doc else d
                            }
                        )
                    }
                }
                .onFailure { error ->
                    _uiState.update { it.copy(isUploading = false, error = error.message) }
                }
        }
    }
}

data class ProfileUiState(
    val isLoading: Boolean = true,
    val profile: Profile? = null,
    val kycStatus: String = "pending",
    val documents: List<Document> = emptyList(),
    val isSaving: Boolean = false,
    val isUploading: Boolean = false,
    val uploadingType: String? = null,
    val showSuccess: Boolean = false,
    val error: String? = null
)
```

### DocumentUploadManager.kt
```kotlin
class DocumentUploadManager @Inject constructor(
    private val documentApi: DocumentApi,
    private val minioClient: MinioClient
) {
    suspend fun uploadDocument(type: String, uri: Uri, context: Context): Result<Document> {
        return try {
            // 1. Get presigned URL
            val fileName = getFileName(context, uri)
            val fileSize = getFileSize(context, uri)
            val contentType = getContentType(context, uri)
            
            val presignedResponse = documentApi.getUploadUrl(
                UploadUrlRequest(type, fileName, fileSize, contentType)
            )
            
            // 2. Upload to MinIO
            val fileBytes = context.contentResolver.openInputStream(uri)?.readBytes()
                ?: throw Exception("Cannot read file")
            
            minioClient.putObject(
                bucket = "kyc",
                key = presignedResponse.documentId,
                data = fileBytes,
                contentType = contentType
            )
            
            // 3. Confirm upload
            val confirmResponse = documentApi.confirmUpload(
                ConfirmUploadRequest(presignedResponse.documentId)
            )
            
            Result.success(confirmResponse.document)
        } catch (e: Exception) {
            Result.failure(e)
        }
    }
    
    fun validateFile(uri: Uri, context: Context): ValidationResult {
        val mimeType = context.contentResolver.getType(uri) ?: ""
        val fileSize = getFileSize(context, uri)
        
        val allowedTypes = listOf("image/jpeg", "image/png", "image/webp", "application/pdf")
        val maxSize = 10 * 1024 * 1024 // 10 MB
        
        if (mimeType !in allowedTypes) {
            return ValidationResult(false, "File type not allowed. Use JPG, PNG, WebP, or PDF.")
        }
        if (fileSize > maxSize) {
            return ValidationResult(false, "File too large. Maximum size is 10 MB.")
        }
        
        return ValidationResult(true)
    }
}

data class ValidationResult(val isValid: Boolean, val error: String? = null)
```

---

## iOS Implementation

### ProfileViewModel.swift
```swift
@Observable
class ProfileViewModel {
    var isLoading: Bool = true
    var profile: Profile?
    var kycStatus: String = "pending"
    var documents: [Document] = []
    var isSaving: Bool = false
    var isUploading: Bool = false
    var uploadingType: String?
    var showSuccess: Bool = false
    var error: String?
    
    private let profileRepository: ProfileRepositoryProtocol
    private let documentRepository: DocumentRepositoryProtocol
    
    init(
        profileRepository: ProfileRepositoryProtocol = ProfileRepository(),
        documentRepository: DocumentRepositoryProtocol = DocumentRepository()
    ) {
        self.profileRepository = profileRepository
        self.documentRepository = documentRepository
        Task { await loadProfile() }
    }
    
    @MainActor
    func loadProfile() async {
        isLoading = true
        defer { isLoading = false }
        
        do {
            profile = try await profileRepository.getProfile()
            kycStatus = profile?.kycStatus ?? "pending"
            documents = profile?.documents ?? []
        } catch {
            self.error = error.localizedDescription
        }
    }
    
    func uploadDocument(type: String, url: URL) async {
        isUploading = true
        uploadingType = type
        defer { isUploading = false; uploadingType = nil }
        
        do {
            let doc = try await documentRepository.uploadDocument(type: type, fileURL: url)
            if let index = documents.firstIndex(where: { $0.type == type }) {
                documents[index] = doc
            }
        } catch {
            self.error = error.localizedDescription
        }
    }
}
```

### DocumentPicker.swift
```swift
struct DocumentPicker: UIViewControllerRepresentable {
    let onPick: (URL) -> Void
    
    func makeUIViewController(context: Context) -> some UIViewController {
        let picker = UIDocumentPickerViewController(
            forOpeningContentTypes: [
                .image, .pdf
            ]
        )
        picker.delegate = context.coordinator
        picker.allowsMultipleSelection = false
        return picker
    }
    
    func updateUIViewController(_ uiViewController: UIViewControllerType, context: Context) {}
    
    func makeCoordinator() -> Coordinator {
        Coordinator(onPick: onPick)
    }
    
    class Coordinator: NSObject, UIDocumentPickerDelegate {
        let onPick: (URL) -> Void
        
        init(onPick: @escaping (URL) -> Void) {
            self.onPick = onPick
        }
        
        func documentPicker(_ controller: UIDocumentPickerViewController, didPickDocumentsAt urls: [URL]) {
            guard let url = urls.first else { return }
            onPick(url)
        }
    }
}
```

---

## KYC Status States

```
┌──────────────┐     ┌──────────────┐     ┌──────────────┐
│  Not Started  │────▶│  Documents   │────▶│  Pending     │
│  (no docs)    │     │  Uploaded    │     │  Verification│
└──────────────┘     └──────────────┘     └──────┬───────┘
                                                  │
                                    ┌─────────────┴─────────────┐
                                    │                           │
                               Verified                    Rejected
                                    │                           │
                                    ▼                           ▼
                            ┌──────────────┐           ┌──────────────┐
                            │  ✅ Verified  │           │  ❌ Rejected  │
                            └──────────────┘           │  (re-upload)  │
                                                       └──────────────┘
```
