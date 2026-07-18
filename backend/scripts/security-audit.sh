#!/usr/bin/env bash
# AeroXe Backend Security Audit Script
# Runs cargo audit, clippy, and security checks
set -euo pipefail

echo "🔒 AeroXe Backend Security Audit"
echo "================================"

# 1. Cargo Audit - Check for known vulnerabilities
echo ""
echo "📦 Running cargo audit..."
if command -v cargo-audit &> /dev/null; then
    cargo audit
    echo "✅ No known vulnerabilities found"
else
    echo "⚠️  cargo-audit not installed. Installing..."
    cargo install cargo-audit
    cargo audit
fi

# 2. Clippy with security-focused lints
echo ""
echo "🔍 Running clippy security lints..."
cargo clippy -- \
    -D clippy::unwrap_used \
    -D clippy::expect_used \
    -D clippy::panic \
    -D clippy::print_stdout \
    -D clippy::print_stderr \
    -W clippy::all \
    -W clippy::pedantic

# 3. Check for hardcoded secrets
echo ""
echo "🔑 Checking for hardcoded secrets..."
SECRETS_FOUND=0
PATTERNS=(
    'password\s*=\s*"[^"]*"'
    'secret\s*=\s*"[^"]*"'
    'api_key\s*=\s*"[^"]*"'
    'PRIVATE KEY'
    'BEGIN RSA'
    'BEGIN EC PRIVATE'
)

for pattern in "${PATTERNS[@]}"; do
    if grep -rn "$pattern" --include="*.rs" --include="*.toml" --include="*.env" . 2>/dev/null | grep -v "test" | grep -v "example"; then
        echo "❌ Potential secret found: $pattern"
        SECRETS_FOUND=1
    fi
done

if [ $SECRETS_FOUND -eq 0 ]; then
    echo "✅ No hardcoded secrets detected"
fi

# 4. Check for debug statements in production code
echo ""
echo "🐛 Checking for debug statements..."
DEBUG_FOUND=0
if grep -rn "eprintln\!\|println\!\|dbg\!\|debug\!" --include="*.rs" src/ 2>/dev/null | grep -v "test" | grep -v "#\[cfg(test)\]"; then
    echo "⚠️  Debug statements found in production code"
    DEBUG_FOUND=1
else
    echo "✅ No debug statements in production code"
fi

# 5. Format check
echo ""
echo "📝 Checking formatting..."
cargo fmt --check

# 6. License audit
echo ""
echo "📄 Checking dependencies..."
if command -v cargo-deny &> /dev/null; then
    cargo deny check
else
    echo "ℹ️  cargo-deny not installed (optional)"
fi

echo ""
echo "🏁 Security audit complete"
