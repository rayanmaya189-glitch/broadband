# AeroXe Backend Security Audit Script (Windows)
# Runs cargo audit, clippy, and security checks

Write-Host "🔒 AeroXe Backend Security Audit" -ForegroundColor Cyan
Write-Host "================================"

# 1. Cargo Audit - Check for known vulnerabilities
Write-Host ""
Write-Host "📦 Running cargo audit..." -ForegroundColor Yellow
try {
    cargo audit
    Write-Host "✅ No known vulnerabilities found" -ForegroundColor Green
} catch {
    Write-Host "⚠️  cargo-audit not installed. Run: cargo install cargo-audit" -ForegroundColor Yellow
}

# 2. Clippy with security-focused lints
Write-Host ""
Write-Host "🔍 Running clippy security lints..." -ForegroundColor Yellow
cargo clippy -- -D clippy::unwrap_used -D clippy::expect_used -D clippy::panic -D clippy::print_stdout -D clippy::print_stderr -W clippy::all -W clippy::pedantic

# 3. Check for hardcoded secrets
Write-Host ""
Write-Host "🔑 Checking for hardcoded secrets..." -ForegroundColor Yellow
$patterns = @('password\s*=\s*"[^"]*"', 'secret\s*=\s*"[^"]*"', 'api_key\s*=\s*"[^"]*"', 'PRIVATE KEY', 'BEGIN RSA')
$secretsFound = $false

foreach ($pattern in $patterns) {
    $matches = Get-ChildItem -Path . -Recurse -Include *.rs,*.toml,*.env | Select-String -Pattern $pattern | Where-Object { $_.Path -notmatch "test|example" }
    if ($matches) {
        Write-Host "❌ Potential secret found: $pattern" -ForegroundColor Red
        $secretsFound = $true
    }
}

if (-not $secretsFound) {
    Write-Host "✅ No hardcoded secrets detected" -ForegroundColor Green
}

# 4. Check for debug statements in production code
Write-Host ""
Write-Host "🐛 Checking for debug statements..." -ForegroundColor Yellow
$debugMatches = Get-ChildItem -Path src -Recurse -Include *.rs | Select-String -Pattern "eprintln!|println!|dbg!|debug!" | Where-Object { $_.Path -notmatch "test" -and $_.Line -notmatch "#\[cfg\(test\)\]" }

if ($debugMatches) {
    Write-Host "⚠️  Debug statements found in production code" -ForegroundColor Yellow
    $debugFound = $true
} else {
    Write-Host "✅ No debug statements in production code" -ForegroundColor Green
    $debugFound = $false
}

# 5. Format check
Write-Host ""
Write-Host "📝 Checking formatting..." -ForegroundColor Yellow
cargo fmt --check
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Code formatting issues found" -ForegroundColor Red
    exit 1
}

# Exit with error if any checks failed
if ($secretsFound -or $debugFound) {
    Write-Host ""
    Write-Host "❌ Security audit failed — issues detected" -ForegroundColor Red
    exit 1
}

Write-Host ""
Write-Host "✅ Security audit passed — no issues found" -ForegroundColor Green
