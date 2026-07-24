# 11 — Documents Module

## Overview

Secure document upload via presigned URLs to MinIO. Used for KYC documents, support ticket attachments, payment proofs, and installation feedback. Both platforms share identical upload logic and validation rules.

---

## Upload Flow

```
┌────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│  Select File    │───▶│  Validate Locally │───▶│ Request Presigned│
│  (Camera/Gallery│    │  (type, size,     │    │  Upload URL      │
│   / File Picker)│    │   magic bytes)    │    │  (POST API)      │
└────────────────┘    └──────────────────┘    └─────────────────┘
                                                      │
                                                      ▼
                                            ┌──────────────────┐
                                            │ Upload to MinIO   │
                                            │ (HTTP PUT)        │
                                            │ Progress tracking │
                                            └──────────────────┘
                                                      │
                                                      ▼
                                            ┌──────────────────┐
                                            │ Confirm Upload    │
                                            │ (POST API)        │
                                            └──────────────────┘
                                                      │
                                                      ▼
                                            ┌──────────────────┐
                                            │ Show Success      │
                                            │ Update UI         │
                                            └──────────────────┘
```

---

## Validation Rules

### File Type Validation
| Category | Allowed Extensions | Allowed MIME Types |
|----------|-------------------|-------------------|
| Images | `.jpg`, `.jpeg`, `.png`, `.webp` | `image/jpeg`, `image/png`, `image/webp` |
| Documents | `.pdf` | `application/pdf` |
| ❌ Not Allowed | `.mp4`, `.zip`, `.rar`, `.doc`, `.docx` | — |

### Size Limits
| Context | Per File | Files per Request | Total per Entity |
|---------|----------|-------------------|------------------|
| Customer (Mobile) | 10 MB | 5 | 50 MB |
| Customer (KYC) | 10 MB | 2 | 20 MB |
| Staff (Admin) | 50 MB | 10 | 200 MB |

### Magic Bytes Validation
```kotlin
// Validate file header matches extension
fun validateMagicBytes(bytes: ByteArray, expectedType: String): Boolean {
    return when (expectedType) {
        "image/jpeg" -> bytes.startsWith(byteArrayOf(0xFF.toByte(), 0xD8.toByte()))
        "image/png" -> bytes.startsWith(byteArrayOf(0x89.toByte(), 0x50, 0x4E, 0x47))
        "image/webp" -> bytes.size >= 12 && 
            String(bytes.copyOfRange(8, 12)) == "WEBP"
        "application/pdf" -> bytes.startsWith("%PDF".toByteArray())
        else -> false
    }
}
```

---

## API Endpoints

> **API Convention:** Protobuf-first. See `docs/backend/API-CONVENTIONS.md`.

### Request Presigned URL
```
POST /api/v1/customer/documents/upload-url

Request:
{
  "document_type": "kyc_aadhaar_front",
  "file_name": "aadhaar_front.jpg",
  "file_size": 1024000,
  "content_type": "image/jpeg"
}

Response 200:
{
  "document_id": "doc_abc123",
  "upload_url": "https://minio.aeroxebroadband.com/uploads/doc_abc123?X-Amz-Algorithm=...",
  "expires_in": 300,
  "minio_key": "uploads/doc_abc123"
}
```

### Confirm Upload
```
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
    "type": "kyc_aadhaar_front",
    "filename": "aadhaar_front.jpg",
    "size": 1024000,
    "status": "uploaded",
    "uploaded_at": "2026-07-08T10:00:00Z"
  }
}
```

### List Documents
```
POST /api/v1/customer/documents/list

Response 200:
{
  "documents": [
    {
      "id": "doc_abc123",
      "type": "kyc_aadhaar_front",
      "filename": "aadhaar_front.jpg",
      "size": 1024000,
      "status": "verified",
      "uploaded_at": "2026-07-01T10:00:00Z"
    }
  ]
}
```

### Download Document
```
POST /api/v1/customer/documents/download

Response 200:
{
  "download_url": "https://minio.aeroxebroadband.com/uploads/doc_abc123?X-Amz-Signature=...",
  "expires_in": 300
}
```

---

## Android Implementation

### DocumentUploadService.kt
```kotlin
class DocumentUploadService @Inject constructor(
    private val documentApi: DocumentApi,
    private val httpClient: OkHttpClient
) {
    suspend fun uploadDocument(
        documentType: String,
        fileName: String,
        fileUri: Uri,
        context: Context,
        onProgress: (Int) -> Unit = {}
    ): Result<Document> {
        return try {
            // 1. Validate file
            val inputStream = context.contentResolver.openInputStream(fileUri)
                ?: return Result.failure(Exception("Cannot read file"))
            val fileBytes = inputStream.readBytes()
            inputStream.close()
            
            val mimeType = context.contentResolver.getType(fileUri) ?: ""
            if (!isValidFile(fileBytes, mimeType)) {
                return Result.failure(Exception("Invalid file type"))
            }
            
            if (fileBytes.size > 10 * 1024 * 1024) {
                return Result.failure(Exception("File too large (max 10 MB)"))
            }
            
            // 2. Request presigned URL
            val presigned = documentApi.getUploadUrl(
                UploadUrlRequest(
                    documentType = documentType,
                    fileName = fileName,
                    fileSize = fileBytes.size.toLong(),
                    contentType = mimeType
                )
            )
            
            // 3. Upload to MinIO with progress
            val requestBody = fileBytes.toRequestBody(mimeType.toMediaTypeOrNull())
            val request = Request.Builder()
                .url(presigned.uploadUrl)
                .put(requestBody)
                .build()
            
            val response = httpClient.newCall(request).execute()
            if (!response.isSuccessful) {
                return Result.failure(Exception("Upload failed: ${response.code}"))
            }
            
            // 4. Confirm upload
            val confirmed = documentApi.confirmUpload(
                ConfirmUploadRequest(presigned.documentId)
            )
            
            onProgress(100)
            Result.success(confirmed.document)
        } catch (e: Exception) {
            Result.failure(e)
        }
    }
    
    private fun isValidFile(bytes: ByteArray, mimeType: String): Boolean {
        val allowedMimes = listOf("image/jpeg", "image/png", "image/webp", "application/pdf")
        if (mimeType !in allowedMimes) return false
        
        // Magic bytes check
        return when (mimeType) {
            "image/jpeg" -> bytes.size >= 2 && bytes[0] == 0xFF.toByte() && bytes[1] == 0xD8.toByte()
            "image/png" -> bytes.size >= 4 && 
                bytes[0] == 0x89.toByte() && bytes[1] == 0x50.toByte() && 
                bytes[2] == 0x4E.toByte() && bytes[3] == 0x47.toByte()
            "application/pdf" -> bytes.size >= 4 && 
                String(bytes.copyOfRange(0, 4)) == "%PDF"
            else -> true
        }
    }
}
```

### Upload Progress Tracking
```kotlin
class ProgressRequestBody(
    private val delegate: RequestBody,
    private val onProgress: (Int) -> Unit
) : RequestBody() {
    
    override fun contentType() = delegate.contentType()
    
    override fun contentLength() = delegate.contentLength()
    
    override fun writeTo(sink: BufferedSink) {
        val totalBytes = contentLength()
        var uploadedBytes = 0L
        
        val countingSink = object : ForwardingSink(sink) {
            override fun write(source: Buffer, byteCount: Long) {
                super.write(source, byteCount)
                uploadedBytes += byteCount
                val progress = ((uploadedBytes * 100) / totalBytes).toInt()
                onProgress(progress)
            }
        }
        
        val bufferedSink = countingSink.buffer()
        delegate.writeTo(bufferedSink)
        bufferedSink.flush()
    }
}
```

---

## iOS Implementation

### DocumentUploadService.swift
```swift
class DocumentUploadService {
    private let documentAPI: DocumentAPIProtocol
    private let session: URLSession
    
    init(
        documentAPI: DocumentAPIProtocol = DocumentAPI(),
        session: URLSession = .shared
    ) {
        self.documentAPI = documentAPI
        self.session = session
    }
    
    func uploadDocument(
        type: String,
        fileName: String,
        fileURL: URL,
        onProgress: @escaping (Double) -> Void = { _ in }
    ) async throws -> Document {
        // 1. Validate file
        let fileData = try Data(contentsOf: fileURL)
        let mimeType = fileURL.mimeType
        
        guard isValidFile(data: fileData, mimeType: mimeType) else {
            throw DocumentError.invalidFileType
        }
        
        guard fileData.count <= 10 * 1024 * 1024 else {
            throw DocumentError.fileTooLarge
        }
        
        // 2. Request presigned URL
        let presigned = try await documentAPI.getUploadUrl(
            documentType: type,
            fileName: fileName,
            fileSize: fileData.count,
            contentType: mimeType
        )
        
        // 3. Upload to MinIO
        var request = URLRequest(url: URL(string: presigned.uploadUrl)!)
        request.httpMethod = "PUT"
        request.setValue(mimeType, forHTTPHeaderField: "Content-Type")
        
        let delegate = UploadDelegate(onProgress: onProgress)
        let (_, response) = try await session.upload(for: request, from: fileData, delegate: delegate)
        
        guard let httpResponse = response as? HTTPURLResponse,
              (200...299).contains(httpResponse.statusCode) else {
            throw DocumentError.uploadFailed
        }
        
        // 4. Confirm upload
        let confirmed = try await documentAPI.confirmUpload(documentId: presigned.documentId)
        
        onProgress(1.0)
        return confirmed.document
    }
    
    private func isValidFile(data: Data, mimeType: String) -> Bool {
        let allowedTypes = ["image/jpeg", "image/png", "image/webp", "application/pdf"]
        guard allowedTypes.contains(mimeType) else { return false }
        
        // Magic bytes check
        guard data.count >= 4 else { return false }
        let header = [UInt8](data.prefix(4))
        
        return switch mimeType {
        case "image/jpeg": header[0] == 0xFF && header[1] == 0xD8
        case "image/png": header[0] == 0x89 && header[1] == 0x50 && header[2] == 0x4E && header[3] == 0x47
        case "application/pdf": header[0] == 0x25 && header[1] == 0x50 && header[2] == 0x44 && header[3] == 0x46
        default: true
        }
    }
}

class UploadDelegate: NSObject, URLSessionTaskDelegate {
    let onProgress: (Double) -> Void
    
    init(onProgress: @escaping (Double) -> Void) {
        self.onProgress = onProgress
    }
    
    func urlSession(
        _ session: URLSession,
        task: URLSessionTask,
        didSendBodyData bytesSent: Int64,
        totalBytesSent: Int64,
        totalBytesExpectedToSend: Int64
    ) {
        let progress = Double(totalBytesSent) / Double(totalBytesExpectedToSend)
        onProgress(progress)
    }
}

enum DocumentError: LocalizedError {
    case invalidFileType
    case fileTooLarge
    case uploadFailed
    
    var errorDescription: String? {
        switch self {
        case .invalidFileType: return "File type not allowed. Use JPG, PNG, WebP, or PDF."
        case .fileTooLarge: return "File too large. Maximum size is 10 MB."
        case .uploadFailed: return "Upload failed. Please try again."
        }
    }
}
```

---

## Upload Progress UI

```
┌─────────────────────────────┐
│  📄 Uploading aadhaar.jpg   │
│                             │
│  ████████████████░░░░░░ 78% │
│  7.8 MB / 10.0 MB           │
│                             │
│  [Cancel]                    │
└─────────────────────────────┘
```

### Android Progress
```kotlin
LinearProgressIndicator(
    progress = { uploadProgress / 100f },
    modifier = Modifier.fillMaxWidth()
)
Text("${uploadProgress}% • ${formatFileSize(bytesUploaded)} / ${formatFileSize(totalBytes)}")
```

### iOS Progress
```swift
ProgressView(value: uploadProgress)
    .progressViewStyle(.linear)
Text("\(Int(uploadProgress * 100))% • \(formatFileSize(bytesUploaded)) / \(formatFileSize(totalBytes))")
```
