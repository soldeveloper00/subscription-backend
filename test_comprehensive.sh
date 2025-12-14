#!/bin/bash

echo "🔬 COMPREHENSIVE BACKEND TEST SUITE"
echo "==================================="
echo ""

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

PASS=0
FAIL=0
WARN=0

# Function to print results
pass() { echo -e "${GREEN}✅ $1${NC}"; ((PASS++)); }
fail() { echo -e "${RED}❌ $1${NC}"; ((FAIL++)); }
warn() { echo -e "${YELLOW}⚠️  $1${NC}"; ((WARN++)); }

# 1. STRUCTURE TEST
echo "1. PROJECT STRUCTURE TEST"
echo "-------------------------"

[ -f "Cargo.toml" ] && pass "Cargo.toml exists" || fail "Missing Cargo.toml"
[ -f "README.md" ] && pass "README.md exists" || fail "Missing README.md"
[ -f ".env.example" ] && pass ".env.example exists" || fail "Missing .env.example"
[ -f ".gitignore" ] && pass ".gitignore exists" || fail "Missing .gitignore"
[ -f "scripts/setup-dev-wallet.sh" ] && pass "Setup script exists" || warn "No setup script"

# Check for sensitive files in gitignore
if grep -q "\.env" .gitignore && grep -q "\.wallets" .gitignore; then
    pass "Sensitive files are gitignored"
else
    fail ".env or .wallets not in .gitignore"
fi

# 2. BUILD TEST
echo ""
echo "2. BUILD TEST"
echo "-------------"

cargo check --quiet
if [ $? -eq 0 ]; then
    pass "Project builds successfully"
else
    fail "Build failed"
    cargo check  # Show errors
fi

# 3. START SERVER TEST
echo ""
echo "3. SERVER STARTUP TEST"
echo "----------------------"

# Kill any existing server
pkill -f trading-signals 2>/dev/null
sleep 1

# Start server in background
echo "Starting server on port 9090..."
cargo run > /tmp/server_test.log 2>&1 &
SERVER_PID=$!
sleep 4

# Check if server started
if ps -p $SERVER_PID > /dev/null; then
    pass "Server process started (PID: $SERVER_PID)"
else
    fail "Server failed to start"
    echo "=== Server Log ==="
    tail -20 /tmp/server_test.log
fi

# 4. API ENDPOINT TESTS
echo ""
echo "4. API ENDPOINT TESTS"
echo "---------------------"

test_endpoint() {
    local endpoint=$1
    local name=$2
    local method=${3:-GET}
    local data=${4:-}
    
    local curl_cmd="curl -s -X $method -w '%{http_code}'"
    if [ ! -z "$data" ]; then
        curl_cmd="$curl_cmd -H 'Content-Type: application/json' -d '$data'"
    fi
    curl_cmd="$curl_cmd http://127.0.0.1:9090$endpoint"
    
    local response=$(eval $curl_cmd 2>/dev/null)
    local status_code=${response: -3}
    local body=${response%???}
    
    if [ "$status_code" = "200" ]; then
        pass "$name ($endpoint) - HTTP 200"
        echo "   Response: $(echo $body | jq -c '.message // .status // .[0:50]' 2>/dev/null || echo $body | head -c 50)"
    else
        fail "$name ($endpoint) - HTTP $status_code"
    fi
}

# Test all GET endpoints
test_endpoint "/wallet/info" "Wallet Info"
test_endpoint "/wallet/test-connection" "Test Connection"
test_endpoint "/blockchain/status" "Blockchain Status"
test_endpoint "/blockchain/test-integration" "Blockchain Integration"
test_endpoint "/prices" "Get Prices"
test_endpoint "/signals" "Get Signals"
test_endpoint "/signals/all" "Get All Signals"
test_endpoint "/status" "Subscription Status"
test_endpoint "/integration/example" "Integration Example"

# Test POST endpoints
test_endpoint "/subscribe" "Subscribe" "POST" '{"email":"test@example.com","symbols":["BTC","ETH"]}'
test_endpoint "/integration/set-condition" "Set Condition" "POST" '{"symbol":"BTC","target_price":100000,"condition":"above"}'
test_endpoint "/signals/webhook" "Webhook Signal" "POST" '{"symbol":"BTC","price":50000,"signal":"buy"}'

# 5. BLOCKCHAIN INTEGRATION TEST
echo ""
echo "5. BLOCKCHAIN INTEGRATION TEST"
echo "-------------------------------"

# Test wallet loading
if [ -f ".wallets/devnet-keypair.json" ]; then
    wallet_addr=$(solana address --keypair .wallets/devnet-keypair.json 2>/dev/null || echo "unknown")
    if [ ! -z "$wallet_addr" ]; then
        pass "Devnet wallet exists: $wallet_addr"
        
        # Check balance (optional)
        balance=$(solana balance $wallet_addr --url https://api.devnet.solana.com 2>/dev/null || echo "0")
        if [ "$balance" != "0 SOL" ]; then
            pass "Wallet has balance: $balance"
        else
            warn "Wallet has 0 balance (run: solana airdrop 2 $wallet_addr)"
        fi
    else
        warn "Could not read wallet address"
    fi
else
    warn "No devnet wallet found (run: bash scripts/setup-dev-wallet.sh)"
fi

# 6. SECURITY TEST
echo ""
echo "6. SECURITY AUDIT"
echo "-----------------"

# Check for hardcoded secrets
if grep -r "private_key\|PRIVATE_KEY\|secret.*key\|SECRET.*KEY" src/ --include="*.rs" | grep -v "//\|test\|example" | grep -q .; then
    fail "Found possible hardcoded secrets in source code"
    grep -r "private_key\|PRIVATE_KEY\|secret.*key\|SECRET.*KEY" src/ --include="*.rs" | grep -v "//"
else
    pass "No hardcoded secrets found"
fi

# Check .env file not committed
if [ -f ".env" ]; then
    if git status .env 2>/dev/null | grep -q "tracked"; then
        fail ".env file is tracked by git (should be in .gitignore)"
    else
        pass ".env file exists but not tracked by git"
    fi
else
    warn "No .env file found (create from .env.example)"
fi

# 7. CODE QUALITY TEST
echo ""
echo "7. CODE QUALITY CHECK"
echo "---------------------"

# Check for unused imports
if cargo check --quiet 2>&1 | grep -q "unused"; then
    warn "Found unused imports/variables"
    cargo check 2>&1 | grep -i "unused" | head -5
else
    pass "No major unused code warnings"
fi

# Count routes
route_count=$(grep -c "\.route(" src/main.rs)
if [ $route_count -ge 10 ]; then
    pass "Found $route_count API routes"
else
    warn "Only $route_count routes found (expected 10+)"
fi

# 8. PERFORMANCE TEST
echo ""
echo "8. PERFORMANCE TEST"
echo "-------------------"

echo "Testing response times..."
for i in {1..3}; do
    time curl -s -o /dev/null -w "%{time_total}s" http://127.0.0.1:9090/wallet/info
    echo " - Request $i"
done | awk '{sum+=$1} END {if(NR>0) print "   Average: " sum/NR "s"}'

# 9. STOP SERVER
echo ""
echo "9. CLEANUP"
echo "----------"

if kill $SERVER_PID 2>/dev/null; then
    pass "Server stopped successfully"
else
    fail "Failed to stop server"
fi

# 10. FINAL SUMMARY
echo ""
echo "📊 TEST SUMMARY"
echo "==============="
echo "Total Tests: $((PASS + FAIL + WARN))"
echo -e "${GREEN}Passed: $PASS${NC}"
echo -e "${RED}Failed: $FAIL${NC}"
echo -e "${YELLOW}Warnings: $WARN${NC}"
echo ""

if [ $FAIL -eq 0 ]; then
    if [ $WARN -eq 0 ]; then
        echo -e "${GREEN}🎉 ALL TESTS PASSED! Ready for GitHub submission.${NC}"
    else
        echo -e "${GREEN}✅ All critical tests passed.${NC}"
        echo -e "${YELLOW}⚠️  Some warnings to review before submission.${NC}"
    fi
else
    echo -e "${RED}❌ Critical failures detected. Fix before submission.${NC}"
    exit 1
fi

echo ""
echo "📦 GITHUB SUBMISSION CHECKLIST:"
echo "1. [ ] Ensure no secrets in code"
echo "2. [ ] Update README.md with your details"
echo "3. [ ] Add LICENSE file (optional)"
echo "4. [ ] Create initial commit: git init"
echo "5. [ ] Add files: git add ."
echo "6. [ ] Commit: git commit -m 'Initial commit: Trading Signals Backend'"
echo "7. [ ] Create repo on GitHub"
echo "8. [ ] Push: git remote add origin <your-repo-url>"
echo "9. [ ] Push: git push -u origin main"
echo ""
echo "🚀 Your trading signals backend is ready for the world!"
