#!/bin/bash

echo "=== OAuth Session Persistence Test ==="
echo

# Set test environment variables
export GITHUB_CLIENT_ID="test_client_id"
export GITHUB_CLIENT_SECRET="test_client_secret"
export OAUTH_REDIRECT_URL="http://localhost:3000/auth/callback"
export SESSION_SECURE="false"
export SESSION_MAX_AGE_DAYS="30"

echo "Environment variables set:"
echo "GITHUB_CLIENT_ID=$GITHUB_CLIENT_ID"
echo "OAUTH_REDIRECT_URL=$OAUTH_REDIRECT_URL"
echo "SESSION_SECURE=$SESSION_SECURE"
echo

echo "Starting web server..."
echo "The server should start successfully and show:"
echo "- Session cookie configuration with SameSite=Lax"
echo "- Debug logging for session IDs"
echo "- OAuth endpoints at /auth/login, /auth/callback, /auth/logout"
echo

# Start server in background
cargo run --bin stage10_web --features web &
SERVER_PID=$!

# Wait a moment for server to start
sleep 3

echo "Testing server startup..."
if curl -s http://localhost:3000/ > /dev/null; then
    echo "✓ Server is responding on http://localhost:3000"
    
    echo
    echo "Available endpoints:"
    echo "- http://localhost:3000/ (root with login/logout)"
    echo "- http://localhost:3000/auth/login (GitHub OAuth)"
    echo "- http://localhost:3000/debug/session (session inspector)"
    echo
    
    echo "Test the OAuth flow:"
    echo "1. Open http://localhost:3000/ in browser"
    echo "2. Click 'Login with GitHub'"
    echo "3. Check terminal for DEBUG messages"
    echo "4. Verify session persistence after redirect"
    
else
    echo "✗ Server failed to start"
fi

echo
echo "Press Ctrl+C to stop the server"

# Wait for user interrupt
wait $SERVER_PID
