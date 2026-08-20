# AeroXe Load Testing

## Prerequisites

Install k6:
```bash
# macOS
brew install k6

# Linux (Debian/Ubuntu)
sudo gpg -k
sudo gpg --no-default-keyring --keyring /usr/share/keyrings/k6-archive-keyring.gpg --keyserver hkp://keyserver.ubuntu.com:80 --recv-keys C5AD17C747E3415A3642D57D77C6C491D6AC1D68
echo "deb [signed-by=/usr/share/keyrings/k6-archive-keyring.gpg] https://dl.k6.io/deb stable main" | sudo tee /etc/apt/sources.list.d/k6.list
sudo apt-get update
sudo apt-get install k6

# Windows
choco install k6
```

## Running Tests

### Basic load test
```bash
k6 run loadtest/k6-load-test.js
```

### Custom VUs and duration
```bash
k6 run --vus 50 --duration 2m loadtest/k6-load-test.js
```

### With custom target
```bash
BASE_URL=https://api.aeroxebroadband.com k6 run loadtest/k6-load-test.js
```

### With authentication
```bash
AUTH_USERNAME=admin@aeroxe.com AUTH_PASSWORD=yourpassword k6 run loadtest/k6-load-test.js
```

### Output to InfluxDB (for Grafana dashboards)
```bash
k6 run --out influxdb=http://localhost:8086/k6 loadtest/k6-load-test.js
```

## Thresholds

The default test configuration enforces:
- **95th percentile response time**: < 2000ms
- **99th percentile response time**: < 5000ms
- **Error rate**: < 10%
- **Failed requests**: < 10%

## Metrics

Custom metrics tracked:
- `errors` — Rate of requests with status >= 400
- `request_duration` — Request duration trend
- `auth_failures` — Counter of failed login attempts
