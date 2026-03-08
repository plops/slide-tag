## 2. Nginx Reverse Proxy Konfiguration
Dein Rust Axum-Webserver wird lokal auf Port `3000` laufen. Da Nginx bereits auf Port `80/443` lauscht und den Traffic an Python (Port `5001`) weiterleitet, fügen wir für Rust eine spezifische Route (z.B. `/app/` oder `/api/`) hinzu.

Füge diesen Block in deine bestehende `nginx.conf` innerhalb des `server { listen 443 ... }` Blocks ein (vorzugsweise *vor* dem generischen `location /` Block):

```nginx
    # Route traffic for the Rust Web App to Axum (Port 3000)
    location /app/ {
        proxy_pass http://127.0.0.1:3000/; # Trailing slash is important to strip /app/
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        
        # WebSocket support if needed by Axum later
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
    }
```