# AeroXe Backend Security Audit Script (Windows)
# Runs cargo audit, clippy, and security checks

Write-Host "AeroXe Backend Security Audit" -ForegroundColor Cyan
Write-Host "================================"

$failed = $false

# 1. Cargo Audit - Check for known vulnerabilities
Write-Host ""
Write-Host "Running cargo audit..." -ForegroundColor Yellow
if (Get-Command cargo-audit -ErrorAction SilentlyContinue) {
    cargo audit
    if ($LASTEXITCODE -ne 0) {
        Write-Host "Cargo audit reported vulnerabilities" -ForegroundColor Red
        $failed = $true
    } else {
        Write-Host "No known vulnerabilities found" -ForegroundColor Green
    }
} else {
    Write-Host "cargo-audit not installed. Run: cargo install cargo-audit" -ForegroundColor Yellow
}

# 2. Clippy - use the same gate as CI (fmt + clippy -D warnings)
Write-Host ""
Write-Host "Running clippy..." -ForegroundColor Yellow
cargo clippy --all-targets --all-features -- -D warnings
if ($LASTEXITCODE -ne 0) {
    Write-Host "Clippy reported warnings" -ForegroundColor Red
    $failed = $true
} else {
    Write-Host "Clippy clean" -ForegroundColor Green
}

# 3. Check for hardcoded secrets
Write-Host ""
Write-Host "Checking for hardcoded secrets..." -ForegroundColor Yellow
$patterns = @('password\s*=\s*"[^"]*"', 'secret\s*=\s*"[^"]*"', 'api_key\s*=\s*"[^"]*"', 'PRIVATE KEY', 'BEGIN RSA')
$secretsFound = $false

foreach ($pattern in $patterns) {
    $matches = Get-ChildItem -Path . -Recurse -Include *.rs,*.toml,*.env | Select-String -Pattern $pattern | Where-Object { $_.Path -notmatch "test|example" }
    if ($matches) {
        Write-Host "Potential secret found: $pattern" -ForegroundColor Red
        $secretsFound = $true
    }
}

if (-not $secretsFound) {
    Write-Host "No hardcoded secrets detected" -ForegroundColor Green
}

# 4. Check for raw debug/panic macros in production code
# NOTE: tracing::debug!/trace! are the sanctioned logging macros and are NOT
# treated as debug leftovers.
Write-Host ""
Write-Host "Checking for debug statements..." -ForegroundColor Yellow
$debugMatches = Get-ChildItem -Path src -Recurse -Include *.rs |
    Select-String -Pattern '\b(db|eprintln|println|print)!' |
    Where-Object { $_.Path -notmatch "test" }

if ($debugMatches) {
    Write-Host "Raw print/dbg macros found in production code" -ForegroundColor Yellow
    $debugFound = $true
} else {
    Write-Host "No raw debug statements in production code" -ForegroundColor Green
    $debugFound = $false
}

# 5. Format check
Write-Host ""
Write-Host "Checking formatting..." -ForegroundColor Yellow
cargo fmt --check
if ($LASTEXITCODE -ne 0) {
    Write-Host "Code formatting issues found" -ForegroundColor Red
    $failed = $true
} else {
    Write-Host "Formatting clean" -ForegroundColor Green
}

# Exit with error if any checks failed
if ($secretsFound -or $debugFound -or $failed) {
    Write-Host ""
    Write-Host "Security audit failed - issues detected" -ForegroundColor Red
    exit 1
}

Write-Host ""
Write-Host "Security audit passed - no issues found" -ForegroundColor Green
