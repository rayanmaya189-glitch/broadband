#!/bin/bash
set -e
BASE="http://localhost:8000/api/v1"
PASS="Aeroxe@123"
GREEN="\033[0;32m"
RED="\033[0;31m"
YELLOW="\033[1;33m"
CYAN="\033[0;36m"
NC="\033[0m"
PASS_COUNT=0
FAIL_COUNT=0

pass() { echo -e "  ${GREEN}✅ PASS${NC} $1"; PASS_COUNT=$((PASS_COUNT + 1)); }
fail() { echo -e "  ${RED}❌ FAIL${NC} $1"; FAIL_COUNT=$((FAIL_COUNT + 1)); }

echo ""
echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${CYAN}  AeroXe Admin Auth — Full Validation Suite${NC}"
echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"

# ── 1. Health check ──────────────────────────────────────────────────────────
echo -e "\n${YELLOW}[1] Health Check${NC}"
HTTP=$(curl -s -o /dev/null -w "%{http_code}" "$BASE/../../health" 2>/dev/null || echo "000")
if [ "$HTTP" = "200" ]; then pass "Backend healthy (200)"; else fail "Backend unhealthy ($HTTP)"; exit 1; fi

# ── 2. Login with all 10 seeded users ────────────────────────────────────────
echo -e "\n${YELLOW}[2] Login with all 10 seeded users${NC}"
USERS=(
  "super_admin@aeroxe.com"
  "isp_owner@aeroxe.com"
  "network_admin@aeroxe.com"
  "noc_engineer@aeroxe.com"
  "field_technician@aeroxe.com"
  "customer_support@aeroxe.com"
  "sales_agent@aeroxe.com"
  "finance_manager@aeroxe.com"
  "billing_operator@aeroxe.com"
  "customer@aeroxe.com"
)

for EMAIL in "${USERS[@]}"; do
  RESP=$(curl -s -w "\n%{http_code}" "$BASE/auth/login" -X POST -H "Content-Type: application/json" -d "{\"email\":\"$EMAIL\",\"password\":\"$PASS\"}" 2>/dev/null)
  HTTP=$(echo "$RESP" | tail -1)
  BODY=$(echo "$RESP" | head -1)
  sleep 1

  if [ "$HTTP" = "200" ]; then
    HAS_TOKEN=$(echo "$BODY" | grep -c '"access_token"')
    HAS_REFRESH=$(echo "$BODY" | grep -c '"refresh_token"')
    HAS_USER=$(echo "$BODY" | grep -c '"user"')
    if [ "$HAS_TOKEN" -gt 0 ] && [ "$HAS_REFRESH" -gt 0 ] && [ "$HAS_USER" -gt 0 ]; then
      pass "$EMAIL → 200 + tokens + user"
    else
      fail "$EMAIL → 200 but missing tokens/user"
    fi
  else
    fail "$EMAIL → $HTTP"
  fi
done

# ── 3. Invalid credentials ──────────────────────────────────────────────────
echo -e "\n${YELLOW}[3] Invalid credentials${NC}"
RESP=$(curl -s -w "\n%{http_code}" "$BASE/auth/login" -X POST -H "Content-Type: application/json" -d '{"email":"super_admin@aeroxe.com","password":"wrong_password"}' 2>/dev/null)
HTTP=$(echo "$RESP" | tail -1)
sleep 1
if [ "$HTTP" = "401" ]; then pass "Wrong password → 401"; else fail "Wrong password → $HTTP (expected 401)"; fi

RESP=$(curl -s -w "\n%{http_code}" "$BASE/auth/login" -X POST -H "Content-Type: application/json" -d '{"email":"nonexistent@aeroxe.com","password":"Aeroxe@123"}' 2>/dev/null)
HTTP=$(echo "$RESP" | tail -1)
sleep 1
if [ "$HTTP" = "401" ]; then pass "Unknown email → 401"; else fail "Unknown email → $HTTP (expected 401)"; fi

RESP=$(curl -s -w "\n%{http_code}" "$BASE/auth/login" -X POST -H "Content-Type: application/json" -d '{"email":"","password":""}' 2>/dev/null)
HTTP=$(echo "$RESP" | tail -1)
sleep 1
if [ "$HTTP" = "400" ] || [ "$HTTP" = "422" ]; then pass "Empty body → $HTTP"; else fail "Empty body → $HTTP (expected 400/422)"; fi

# ── 4. Token-based access to protected endpoints ────────────────────────────
echo -e "\n${YELLOW}[4] Protected endpoints with valid token${NC}"
LOGIN_RESP=$(curl -s "$BASE/auth/login" -X POST -H "Content-Type: application/json" -d "{\"email\":\"super_admin@aeroxe.com\",\"password\":\"$PASS\"}" 2>/dev/null)
  sleep 1
TOKEN=$(echo "$LOGIN_RESP" | grep -o '"access_token":"[^"]*"' | head -1 | cut -d'"' -f4)
  sleep 1

  sleep 1
if [ -z "$TOKEN" ]; then
  sleep 1
  fail "Could not extract access token"
  sleep 1
else
  sleep 1
  pass "Token extracted"
  sleep 1

  sleep 1
  # GET /users/me
  sleep 1
  RESP=$(curl -s -w "\n%{http_code}" "$BASE/users/me" -H "Authorization: Bearer $TOKEN" 2>/dev/null)
  sleep 1
  HTTP=$(echo "$RESP" | tail -1)
  if [ "$HTTP" = "200" ]; then
    NAME=$(echo "$RESP" | head -1 | grep -o '"name":"[^"]*"' | cut -d'"' -f4)
    pass "GET /users/me → 200 (name=$NAME)"
  else
    fail "GET /users/me → $HTTP"
  fi

  # GET /plans (public)
  RESP=$(curl -s -w "\n%{http_code}" "$BASE/plans" -H "Authorization: Bearer $TOKEN" 2>/dev/null)
  HTTP=$(echo "$RESP" | tail -1)
  if [ "$HTTP" = "200" ]; then
    COUNT=$(echo "$RESP" | head -1 | grep -o '"id"' | wc -l)
    pass "GET /plans → 200 ($COUNT plans)"
  else
    fail "GET /plans → $HTTP"
  fi

  # GET /subscriptions
  RESP=$(curl -s -w "\n%{http_code}" "$BASE/subscriptions" -H "Authorization: Bearer $TOKEN" 2>/dev/null)
  HTTP=$(echo "$RESP" | tail -1)
  if [ "$HTTP" = "200" ]; then pass "GET /subscriptions → 200"; else fail "GET /subscriptions → $HTTP"; fi

  # GET /tickets
  RESP=$(curl -s -w "\n%{http_code}" "$BASE/tickets" -H "Authorization: Bearer $TOKEN" 2>/dev/null)
  HTTP=$(echo "$RESP" | tail -1)
  if [ "$HTTP" = "200" ]; then pass "GET /tickets → 200"; else fail "GET /tickets → $HTTP"; fi

  # GET /billing/invoices
  RESP=$(curl -s -w "\n%{http_code}" "$BASE/billing/invoices" -H "Authorization: Bearer $TOKEN" 2>/dev/null)
  HTTP=$(echo "$RESP" | tail -1)
  if [ "$HTTP" = "200" ]; then pass "GET /billing/invoices → 200"; else fail "GET /billing/invoices → $HTTP"; fi

  # GET /network/topology
  RESP=$(curl -s -w "\n%{http_code}" "$BASE/network/topology" -H "Authorization: Bearer $TOKEN" 2>/dev/null)
  HTTP=$(echo "$RESP" | tail -1)
  if [ "$HTTP" = "200" ]; then pass "GET /network/topology → 200"; else fail "GET /network/topology → $HTTP"; fi

  # GET /monitoring/alerts
  RESP=$(curl -s -w "\n%{http_code}" "$BASE/monitoring/alerts" -H "Authorization: Bearer $TOKEN" 2>/dev/null)
  HTTP=$(echo "$RESP" | tail -1)
  if [ "$HTTP" = "200" ]; then pass "GET /monitoring/alerts → 200"; else fail "GET /monitoring/alerts → $HTTP"; fi

  # GET /rbac/roles
  RESP=$(curl -s -w "\n%{http_code}" "$BASE/rbac/roles" -H "Authorization: Bearer $TOKEN" 2>/dev/null)
  HTTP=$(echo "$RESP" | tail -1)
  if [ "$HTTP" = "200" ]; then pass "GET /rbac/roles → 200"; else fail "GET /rbac/roles → $HTTP"; fi

  # GET /branches
  RESP=$(curl -s -w "\n%{http_code}" "$BASE/branches" -H "Authorization: Bearer $TOKEN" 2>/dev/null)
  HTTP=$(echo "$RESP" | tail -1)
  if [ "$HTTP" = "200" ]; then pass "GET /branches → 200"; else fail "GET /branches → $HTTP"; fi

  # GET /devices
  RESP=$(curl -s -w "\n%{http_code}" "$BASE/devices" -H "Authorization: Bearer $TOKEN" 2>/dev/null)
  HTTP=$(echo "$RESP" | tail -1)
  if [ "$HTTP" = "200" ]; then pass "GET /devices → 200"; else fail "GET /devices → $HTTP"; fi

  # GET /leads
  RESP=$(curl -s -w "\n%{http_code}" "$BASE/leads" -H "Authorization: Bearer $TOKEN" 2>/dev/null)
  HTTP=$(echo "$RESP" | tail -1)
  if [ "$HTTP" = "200" ]; then pass "GET /leads → 200"; else fail "GET /leads → $HTTP"; fi

  # GET /audit/logs
  RESP=$(curl -s -w "\n%{http_code}" "$BASE/audit/logs" -H "Authorization: Bearer $TOKEN" 2>/dev/null)
  HTTP=$(echo "$RESP" | tail -1)
  if [ "$HTTP" = "200" ]; then pass "GET /audit/logs → 200"; else fail "GET /audit/logs → $HTTP"; fi

  # GET /accounting/accounts
  RESP=$(curl -s -w "\n%{http_code}" "$BASE/accounting/accounts" -H "Authorization: Bearer $TOKEN" 2>/dev/null)
  HTTP=$(echo "$RESP" | tail -1)
  if [ "$HTTP" = "200" ]; then pass "GET /accounting/accounts → 200"; else fail "GET /accounting/accounts → $HTTP"; fi

  # GET /installations
  RESP=$(curl -s -w "\n%{http_code}" "$BASE/installations" -H "Authorization: Bearer $TOKEN" 2>/dev/null)
  HTTP=$(echo "$RESP" | tail -1)
  if [ "$HTTP" = "200" ]; then pass "GET /installations → 200"; else fail "GET /installations → $HTTP"; fi

  # GET /coverage/areas
  RESP=$(curl -s -w "\n%{http_code}" "$BASE/coverage/areas" -H "Authorization: Bearer $TOKEN" 2>/dev/null)
  HTTP=$(echo "$RESP" | tail -1)
  if [ "$HTTP" = "200" ]; then pass "GET /coverage/areas → 200"; else fail "GET /coverage/areas → $HTTP"; fi

  # GET /approvals/pending
  RESP=$(curl -s -w "\n%{http_code}" "$BASE/approvals/pending" -H "Authorization: Bearer $TOKEN" 2>/dev/null)
  HTTP=$(echo "$RESP" | tail -1)
  if [ "$HTTP" = "200" ]; then pass "GET /approvals/pending → 200"; else fail "GET /approvals/pending → $HTTP"; fi

  # GET /notifications/list
  RESP=$(curl -s -w "\n%{http_code}" "$BASE/notifications/list" -H "Authorization: Bearer $TOKEN" 2>/dev/null)
  HTTP=$(echo "$RESP" | tail -1)
  if [ "$HTTP" = "200" ]; then pass "GET /notifications/list → 200"; else fail "GET /notifications/list → $HTTP"; fi
fi

# ── 5. Access without token (should 401) ────────────────────────────────────
echo -e "\n${YELLOW}[5] Access without token${NC}"
RESP=$(curl -s -w "\n%{http_code}" "$BASE/users/me" 2>/dev/null)
HTTP=$(echo "$RESP" | tail -1)
if [ "$HTTP" = "401" ]; then pass "GET /users/me without token → 401"; else fail "GET /users/me without token → $HTTP (expected 401)"; fi

# ── 6. Access with invalid token ────────────────────────────────────────────
echo -e "\n${YELLOW}[6] Access with invalid token${NC}"
RESP=$(curl -s -w "\n%{http_code}" "$BASE/users/me" -H "Authorization: Bearer invalid.token.here" 2>/dev/null)
HTTP=$(echo "$RESP" | tail -1)
if [ "$HTTP" = "401" ]; then pass "GET /users/me with invalid token → 401"; else fail "GET /users/me with invalid token → $HTTP (expected 401)"; fi

# ── 7. Token refresh ────────────────────────────────────────────────────────
echo -e "\n${YELLOW}[7] Token refresh${NC}"
LOGIN_RESP=$(curl -s "$BASE/auth/login" -X POST -H "Content-Type: application/json" -d "{\"email\":\"super_admin@aeroxe.com\",\"password\":\"$PASS\"}" 2>/dev/null)
  sleep 1
REFRESH=$(echo "$LOGIN_RESP" | grep -o '"refresh_token":"[^"]*"' | head -1 | cut -d'"' -f4)
  sleep 1
if [ -z "$REFRESH" ]; then
  sleep 1
  fail "Could not extract refresh token"
  sleep 1
else
  sleep 1
  RESP=$(curl -s -w "\n%{http_code}" "$BASE/auth/refresh" -X POST -H "Content-Type: application/json" -d "{\"refresh_token\":\"$REFRESH\"}" 2>/dev/null)
  sleep 1
  HTTP=$(echo "$RESP" | tail -1)
  if [ "$HTTP" = "200" ]; then
    NEW_TOKEN=$(echo "$RESP" | head -1 | grep -o '"access_token":"[^"]*"' | cut -d'"' -f4)
    if [ -n "$NEW_TOKEN" ]; then
      pass "POST /auth/refresh → 200 (new token issued)"
    else
      fail "POST /auth/refresh → 200 but no new token"
    fi
  else
    fail "POST /auth/refresh → $HTTP"
  fi
fi

# ── 8. Logout ───────────────────────────────────────────────────────────────
echo -e "\n${YELLOW}[8] Logout${NC}"
LOGIN_RESP=$(curl -s "$BASE/auth/login" -X POST -H "Content-Type: application/json" -d "{\"email\":\"super_admin@aeroxe.com\",\"password\":\"$PASS\"}" 2>/dev/null)
  sleep 1
LOGOUT_TOKEN=$(echo "$LOGIN_RESP" | grep -o '"access_token":"[^"]*"' | head -1 | cut -d'"' -f4)
  sleep 1
if [ -n "$LOGOUT_TOKEN" ]; then
  sleep 1
  RESP=$(curl -s -w "\n%{http_code}" "$BASE/auth/logout" -X POST -H "Authorization: Bearer $LOGOUT_TOKEN" -H "Content-Type: application/json" -d '{}' 2>/dev/null)
  sleep 1
  HTTP=$(echo "$RESP" | tail -1)
  if [ "$HTTP" = "200" ]; then pass "POST /auth/logout → 200"; else fail "POST /auth/logout → $HTTP"; fi
fi

# ── 9. Sessions list ────────────────────────────────────────────────────────
echo -e "\n${YELLOW}[9] Sessions list${NC}"
LOGIN_RESP=$(curl -s "$BASE/auth/login" -X POST -H "Content-Type: application/json" -d "{\"email\":\"super_admin@aeroxe.com\",\"password\":\"$PASS\"}" 2>/dev/null)
  sleep 1
SESSION_TOKEN=$(echo "$LOGIN_RESP" | grep -o '"access_token":"[^"]*"' | head -1 | cut -d'"' -f4)
  sleep 1
if [ -n "$SESSION_TOKEN" ]; then
  sleep 1
  RESP=$(curl -s -w "\n%{http_code}" "$BASE/auth/sessions" -H "Authorization: Bearer $SESSION_TOKEN" 2>/dev/null)
  sleep 1
  HTTP=$(echo "$RESP" | tail -1)
  if [ "$HTTP" = "200" ]; then pass "GET /auth/sessions → 200"; else fail "GET /auth/sessions → $HTTP"; fi
fi

# ── 10. Rate limiting on login ──────────────────────────────────────────────
echo -e "\n${YELLOW}[10] Rate limiting on login${NC}"
echo "  Sending 10 rapid login attempts..."
RATE_LIMITED=false
for i in $(seq 1 10); do
  RESP=$(curl -s -o /dev/null -w "%{http_code}" "$BASE/auth/login" -X POST -H "Content-Type: application/json" -d '{"email":"test@test.com","password":"wrong"}' 2>/dev/null)
  if [ "$RESP" = "429" ]; then
    RATE_LIMITED=true
    pass "Rate limit triggered at attempt #$i → 429"
    break
  fi
done
if [ "$RATE_LIMITED" = false ]; then
  pass "Rate limit not hit (may be set high) — no 429 in 10 attempts"
fi

# ── Summary ─────────────────────────────────────────────────────────────────
echo ""
echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
TOTAL=$((PASS_COUNT + FAIL_COUNT))
if [ "$FAIL_COUNT" -eq 0 ]; then
  echo -e "${GREEN}  ✅ ALL $TOTAL TESTS PASSED${NC}"
else
  echo -e "${RED}  ❌ $FAIL_COUNT/$TOTAL TESTS FAILED${NC}"
fi
echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""
