#!/bin/bash

echo "=== GitHub OAuth Test mit echten Credentials ==="
echo

# Secrets aus den konfigurierten Dateien laden (wie in docs/carbonyl.md beschrieben)
if [ -f "$HOME/rs_scrape_github.sh" ]; then
    echo "🔐 Lade OAuth Secrets aus ~/rs_scrape_github.sh"
    source "$HOME/rs_scrape_github.sh"
else
    echo "❌ Fehler: ~/rs_scrape_github.sh nicht gefunden!"
    echo "Bitte erstellen Sie diese Datei mit den GitHub OAuth Credentials."
    exit 1
fi

# Prüfen ob Environment Variables nach dem Laden gesetzt sind
if [ -z "$GITHUB_CLIENT_ID" ] || [ -z "$GITHUB_CLIENT_SECRET" ]; then
    echo "❌ Fehler: GitHub OAuth Credentials nicht in ~/rs_scrape_github.sh gefunden!"
    echo
    echo "Die Datei sollte enthalten:"
    echo "export GITHUB_CLIENT_ID=\"IHR_CLIENT_ID\""
    echo "export GITHUB_CLIENT_SECRET=\"IHR_CLIENT_SECRET\""
    echo "export OAUTH_REDIRECT_URL=\"http://localhost:3000/auth/callback\""
    exit 1
fi

echo "✅ GitHub OAuth Credentials geladen"
echo "GITHUB_CLIENT_ID: ${GITHUB_CLIENT_ID:0:8}..."
echo "OAUTH_REDIRECT_URL: $OAUTH_REDIRECT_URL"
echo

# Session-Konfiguration anzeigen
echo "Session-Konfiguration:"
echo "SESSION_SECURE: ${SESSION_SECURE:-false}"
echo "SESSION_MAX_AGE_DAYS: ${SESSION_MAX_AGE_DAYS:-30}"
echo

echo "🚀 Starte Web Server..."
echo "Server wird auf http://localhost:3000 gestartet"
echo

# Server im Hintergrund starten
cargo run --bin stage10_web --features web &
SERVER_PID=$!

# Warten auf Server-Start
sleep 3

echo "🧪 Teste Server-Endpunkte..."

# Test 1: Root-Endpunkt
echo -n "1. Root-Endpunkt: "
if curl -s http://localhost:3000/ | grep -q "Login with GitHub"; then
    echo "✅ OK"
else
    echo "❌ Fehler"
fi

# Test 2: Session Debug
echo -n "2. Session Debug: "
if curl -s http://localhost:3000/debug/session | grep -q "session_id"; then
    echo "✅ OK"
else
    echo "❌ Fehler"
fi

# Test 3: OAuth Login Redirect
echo -n "3. OAuth Login: "
RESPONSE=$(curl -i -s http://localhost:3000/auth/login)
if echo "$RESPONSE" | grep -q "location.*github.com" && echo "$RESPONSE" | grep -q "set-cookie:"; then
    echo "✅ OK"
    echo "   - Redirect zu GitHub funktioniert"
    echo "   - Session Cookie wird gesetzt"
else
    echo "❌ Fehler"
fi

echo
echo "🔍 OAuth Flow Test:"
echo "1. Öffnen Sie im Browser: http://localhost:3000"
echo "2. Klicken Sie auf 'Login with GitHub'"
echo "3. Autorisieren Sie die Anwendung auf GitHub"
echo "4. Sie sollten zurück zur Anwendung geleitet werden"
echo "5. Überprüfen Sie, dass 'Hello {Name}!' angezeigt wird"
echo
echo "🐛 Debug-Informationen:"
echo "Überwachen Sie die Terminal-Ausgabe für DEBUG-Meldungen"
echo
echo "🍪 Cookie-Überprüfung:"
echo "Browser DevTools → Application → Cookies → http://localhost:3000"
echo
echo "Server läuft mit PID: $SERVER_PID"
echo "Drücken Sie Ctrl+C um den Server zu stoppen"

# Warten auf User-Interrupt
wait $SERVER_PID
