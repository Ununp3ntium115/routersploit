# PyRouterSploit → PYRO Fire Marshal API Integration Map

## Where PyRouterSploit Sits in PYRO Ecosystem

### PYRO Fire Marshal Platform Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                    PYRO Fire Marshal Platform                    │
│                                                                   │
│  ┌────────────────────────────────────────────────────────────┐ │
│  │              PYRO Main UI (Svelte/React)                   │ │
│  │                                                             │ │
│  │  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────────┐ │ │
│  │  │Dashboard │ │Monitoring│ │Compliance│ │Vuln Testing  │ │ │
│  │  │          │ │          │ │          │ │(PyRouter UI) │ │ │
│  │  └──────────┘ └──────────┘ └──────────┘ └──────────────┘ │ │
│  └────────────────────────────────────────────────────────────┘ │
│                            │                                     │
│  ┌─────────────────────────▼──────────────────────────────────┐ │
│  │              PYRO API Gateway / Router                      │ │
│  │              (Handles auth, routing, rate limiting)         │ │
│  └─────────────────────────┬──────────────────────────────────┘ │
│                            │                                     │
│         ┌──────────────────┼──────────────────┐                 │
│         │                  │                  │                 │
│  ┌──────▼──────┐   ┌──────▼──────┐   ┌──────▼──────────┐      │
│  │  Monitoring │   │  Training   │   │  PyRouterSploit │      │
│  │   Service   │   │   Models    │   │  Vuln Engine    │      │
│  │             │   │   Service   │   │                 │      │
│  │  /monitor/* │   │  /models/*  │   │  /vulnscan/*    │      │
│  └─────────────┘   └─────────────┘   └─────────────────┘      │
│                                              │                   │
│                                    ┌─────────▼─────────┐        │
│                                    │ Shared PostgreSQL │        │
│                                    │   or redb cluster │        │
│                                    └───────────────────┘        │
└─────────────────────────────────────────────────────────────────┘
```

---

## PyRouterSploit API Endpoints in PYRO Context

### PYRO API Gateway Routes

All PYRO services are accessed through a unified gateway:

```
https://pyro-platform.local/api/v1/
```

### PyRouterSploit Namespace in PYRO

```
PYRO Gateway → /api/v1/vulnscan/* → PyRouterSploit Service
```

**Full Path Structure:**
```
https://pyro-platform.local/api/v1/vulnscan/
├── /scan                    # Initiate vulnerability scan
├── /scans                   # List all scans
├── /scans/{id}             # Get specific scan details
├── /scans/{id}/status      # Get scan status
├── /scans/{id}/results     # Get scan results
├── /scans/{id}/cancel      # Cancel running scan
├── /exploits               # List available exploits
├── /exploits/{id}          # Get exploit details
├── /exploits/run           # Execute specific exploit
├── /cryptex                # Cryptex dictionary operations
├── /cryptex/search         # Search cryptex
├── /crypto/hash            # Hashing operations
├── /crypto/qkd/encrypt     # QKD encryption
└── /health                 # Service health check
```

---

## PYRO-Compatible API Specification

### 1. Authentication Flow

**PYRO Gateway handles all authentication:**

```http
# User authenticates with PYRO
POST https://pyro-platform.local/api/v1/auth/login
{
  "username": "security_analyst",
  "password": "***"
}

Response:
{
  "access_token": "eyJhbGc...",
  "refresh_token": "...",
  "token_type": "Bearer"
}

# All subsequent requests include token
GET https://pyro-platform.local/api/v1/vulnscan/scans
Authorization: Bearer eyJhbGc...
```

**PyRouterSploit receives pre-authenticated requests:**

```rust
// PYRO gateway validates JWT and forwards with user context
#[derive(Debug, Deserialize)]
pub struct PyroUserContext {
    pub user_id: String,
    pub username: String,
    pub roles: Vec<String>,
    pub tenant_id: Option<String>,
}

// PyRouterSploit receives this in headers
X-PYRO-User-ID: abc123
X-PYRO-Username: security_analyst
X-PYRO-Roles: admin,scanner
X-PYRO-Tenant-ID: tenant-001
```

### 2. Scan Initiation (PYRO Format)

**Endpoint:** `POST /api/v1/vulnscan/scan`

**PYRO Gateway Request:**
```http
POST https://pyro-platform.local/api/v1/vulnscan/scan
Authorization: Bearer {pyro_token}
Content-Type: application/json

{
  "scan_config": {
    "targets": [
      "192.168.1.0/24",
      "10.0.0.1"
    ],
    "scan_type": "comprehensive",
    "verify_exploits": true,
    "max_threads": 50,
    "timeout": 300,
    "scan_profiles": ["router", "camera"],
    "exclude_targets": ["192.168.1.100"]
  },
  "metadata": {
    "environment": "production",
    "compliance_framework": "NIST",
    "requester_name": "John Doe",
    "department": "Security Operations",
    "tags": ["quarterly-scan", "router-audit"]
  },
  "callbacks": {
    "on_complete": "/api/v1/webhooks/scan-complete",
    "on_vulnerability_found": "/api/v1/webhooks/vulnerability-alert",
    "on_critical_finding": "/api/v1/webhooks/critical-alert"
  },
  "integrations": {
    "training_model": true,
    "compliance_report": true,
    "slack_notification": true
  }
}
```

**PyRouterSploit Response (PYRO Format):**
```json
{
  "status": "success",
  "data": {
    "scan_id": "550e8400-e29b-41d4-a716-446655440000",
    "status": "initiated",
    "estimated_duration_seconds": 1800,
    "estimated_completion": "2025-01-22T16:30:00Z",
    "targets_count": 254,
    "scan_type": "comprehensive"
  },
  "meta": {
    "request_id": "req_abc123",
    "timestamp": "2025-01-22T14:00:00Z",
    "service": "pyroutersploit",
    "version": "4.0.0"
  }
}
```

### 3. Results Retrieval (PYRO Format)

**Endpoint:** `GET /api/v1/vulnscan/scans/{id}/results`

**PYRO-Compatible Response:**
```json
{
  "status": "success",
  "data": {
    "scan_id": "550e8400-e29b-41d4-a716-446655440000",
    "scan_status": "completed",
    "started_at": "2025-01-22T14:00:00Z",
    "completed_at": "2025-01-22T14:45:00Z",
    "duration_seconds": 2700,
    "summary": {
      "targets_scanned": 254,
      "vulnerabilities_found": 47,
      "by_severity": {
        "critical": 5,
        "high": 12,
        "medium": 20,
        "low": 10
      },
      "risk_score": 78.5,
      "compliance_status": "non_compliant"
    },
    "vulnerabilities": [
      {
        "id": "vuln_001",
        "target": "192.168.1.1",
        "device_info": {
          "type": "router",
          "vendor": "D-Link",
          "model": "DIR-850L",
          "firmware_version": "1.14B07"
        },
        "vulnerability": {
          "cve_id": "CVE-2024-XXXX",
          "name": "D-Link HNAP Authentication Bypass",
          "severity": "critical",
          "cvss_score": 9.8,
          "description": "Remote code execution via HNAP authentication bypass",
          "exploit_available": true,
          "exploit_public": true,
          "verified": true,
          "proof_of_concept": "Successfully executed arbitrary command",
          "affected_versions": ["1.14B07", "1.13B01"],
          "remediation": "Update to firmware version 1.15 or higher",
          "references": [
            "https://nvd.nist.gov/vuln/detail/CVE-2024-XXXX",
            "https://www.dlink.com/security-advisory"
          ]
        },
        "attack_details": {
          "attack_vector": "network",
          "attack_complexity": "low",
          "privileges_required": "none",
          "user_interaction": "none",
          "scope": "unchanged"
        },
        "discovered_at": "2025-01-22T14:15:00Z"
      }
    ],
    "metadata": {
      "environment": "production",
      "compliance_framework": "NIST",
      "requester_name": "John Doe"
    }
  },
  "meta": {
    "request_id": "req_xyz789",
    "timestamp": "2025-01-22T15:00:00Z",
    "service": "pyroutersploit",
    "version": "4.0.0"
  }
}
```

---

## Svelte UI Integration Architecture

### Option 1: Embedded in PYRO UI (RECOMMENDED)

**Architecture:**
```
PYRO Main UI (Svelte/React)
├── Dashboard Module
├── Monitoring Module
├── Compliance Module
└── Vulnerability Testing Module ← PyRouterSploit Svelte Components
    ├── Scan Dashboard
    ├── Exploit Browser
    ├── Results Viewer
    └── Cryptex Dictionary
```

**Implementation:**
```typescript
// PYRO main app imports PyRouterSploit components
import { ScanDashboard, ExploitBrowser } from '@pyroutersploit/ui-components';

// Mount in PYRO route
<Route path="/vulnerability-testing">
  <ScanDashboard apiUrl={PYRO_API_URL} />
</Route>
```

**Build as NPM Package:**
```json
// ui/package.json
{
  "name": "@pyroutersploit/ui-components",
  "version": "4.0.0",
  "main": "dist/index.js",
  "exports": {
    "./ScanDashboard": "./dist/ScanDashboard.svelte",
    "./ExploitBrowser": "./dist/ExploitBrowser.svelte",
    "./ResultsViewer": "./dist/ResultsViewer.svelte",
    "./CryptexDictionary": "./dist/CryptexDictionary.svelte"
  }
}
```

### Option 2: Standalone with PYRO Theming

**Architecture:**
```
PyRouterSploit Standalone UI (Port 5173)
  ↓ (iframed or linked from)
PYRO Main UI
  → "Vulnerability Testing" tab opens PyRouterSploit UI
```

**Implementation:**
```html
<!-- PYRO UI embeds PyRouterSploit -->
<iframe
  src="http://pyroutersploit:5173"
  sandbox="allow-same-origin allow-scripts"
  style="width: 100%; height: 100%;">
</iframe>
```

### Option 3: Hybrid (RECOMMENDED FOR MVP)

**Architecture:**
```
PYRO Main UI
├── Embedded: Scan Dashboard (quick view)
└── Link: Full PyRouterSploit UI (detailed analysis)
```

**Key Components to Embed:**
1. **Scan Initiator** - Form to start scans from PYRO
2. **Results Summary** - Table of recent scans
3. **Vulnerability Alerts** - Critical findings widget

**Standalone Components:**
1. **Full Scan Dashboard** - Detailed scan management
2. **Exploit Browser** - Browse/test exploits
3. **Cryptex Dictionary** - Manage function mappings
4. **QKD Crypto Console** - Encryption tools

---

## API Client Configuration

### Svelte UI API Client (PYRO-Aware)

```typescript
// ui/src/lib/api/pyro-client.ts
import axios from 'axios';

export class PyroApiClient {
  private baseUrl: string;
  private token: string | null = null;

  constructor(config: { baseUrl?: string; token?: string } = {}) {
    // Auto-detect if running in PYRO context
    this.baseUrl = config.baseUrl || this.detectPyroUrl();
    this.token = config.token || this.getPyroToken();
  }

  private detectPyroUrl(): string {
    // If embedded in PYRO, use PYRO's API gateway
    if (window.location.hostname.includes('pyro-platform')) {
      return `${window.location.origin}/api/v1/vulnscan`;
    }
    // Otherwise, standalone mode
    return 'http://localhost:8080/api/v1';
  }

  private getPyroToken(): string | null {
    // If in PYRO context, token is provided by parent frame
    if (window.parent !== window) {
      return window.parent.localStorage.getItem('pyro_access_token');
    }
    // Standalone mode - local auth
    return localStorage.getItem('pyroutersploit_token');
  }

  async inititateScan(config: ScanConfig) {
    return axios.post(`${this.baseUrl}/scan`, config, {
      headers: {
        'Authorization': `Bearer ${this.token}`,
        'X-PYRO-Integration': 'true',
      }
    });
  }
}
```

---

## WebSocket Integration

### PYRO WebSocket Gateway

**PYRO WebSocket URL:**
```
wss://pyro-platform.local/ws/v1/vulnscan/scans/{scan_id}
```

**PyRouterSploit Events → PYRO:**
```typescript
// PyRouterSploit streams events
enum ScanEvent {
  STARTED = "scan.started",
  TARGET_SCANNED = "scan.target_scanned",
  VULNERABILITY_FOUND = "scan.vulnerability_found",
  COMPLETED = "scan.completed",
  ERROR = "scan.error"
}

// PYRO subscribes and broadcasts to all connected clients
ws.on('message', (event) => {
  // Forward to PYRO dashboard
  pyroDashboard.broadcast({
    type: event.type,
    service: 'pyroutersploit',
    data: event.data
  });

  // Update training models
  if (event.type === 'VULNERABILITY_FOUND') {
    trainingModelService.ingest(event.data);
  }

  // Trigger compliance checks
  if (event.type === 'COMPLETED') {
    complianceService.evaluate(event.data);
  }
});
```

---

## PYRO Integration Checklist

### Phase 1: API Alignment
- [ ] Adopt PYRO API response format (status, data, meta)
- [ ] Use PYRO authentication headers (X-PYRO-User-ID, etc.)
- [ ] Implement PYRO error format
- [ ] Add PYRO-specific metadata to responses
- [ ] Support PYRO webhook callbacks

### Phase 2: UI Integration
- [ ] Build Svelte components as NPM package
- [ ] Support PYRO theming (CSS variables)
- [ ] Detect PYRO context in API client
- [ ] Add parent-frame communication
- [ ] Implement PYRO SSO token handling

### Phase 3: Data Integration
- [ ] Export scan results in PYRO training format
- [ ] Send events to PYRO WebSocket gateway
- [ ] Support PYRO's PostgreSQL (in addition to redb)
- [ ] Implement PYRO tenant isolation
- [ ] Add PYRO compliance tags

### Phase 4: Operations
- [ ] Health check endpoint for PYRO monitoring
- [ ] Metrics endpoint (Prometheus format)
- [ ] Logging to PYRO's centralized logging
- [ ] Support PYRO's service discovery
- [ ] Graceful shutdown on PYRO signals

---

## Environment Configuration

### Standalone Mode

```bash
# .env.standalone
MODE=standalone
API_HOST=0.0.0.0
API_PORT=8080
DATABASE_PATH=./data/pyroutersploit.redb
UI_ENABLED=true
UI_PORT=5173
AUTH_ENABLED=true
```

### PYRO Integration Mode

```bash
# .env.pyro
MODE=pyro_integrated
API_HOST=0.0.0.0
API_PORT=8080
PYRO_GATEWAY_URL=https://pyro-platform.local/api/v1
PYRO_TENANT_ID=tenant-001
DATABASE_TYPE=postgres
DATABASE_URL=$PYRO_DB_URL
UI_ENABLED=false  # PYRO provides UI
AUTH_ENABLED=false  # PYRO gateway handles auth
PYRO_WEBHOOK_SECRET=$PYRO_WEBHOOK_SECRET
```

---

## Implementation Priority

### Week 1: API Format Alignment
1. Update all API responses to PYRO format
2. Add PYRO authentication header support
3. Implement webhook callbacks
4. Add health/metrics endpoints

### Week 2: Svelte UI Package
1. Extract Svelte components to @pyroutersploit/ui-components
2. Add PYRO theme support
3. Build API client with context detection
4. Publish NPM package

### Week 3: Integration Testing
1. Test with mock PYRO gateway
2. Test WebSocket event forwarding
3. Test training data export
4. End-to-end integration test

### Week 4: Documentation
1. PYRO integration guide
2. API contract documentation
3. UI component documentation
4. Deployment guide

**Total Effort:** ~80 hours

---

## Next Steps

1. **Confirm PYRO API specifications** - Get actual PYRO gateway API docs
2. **Choose UI integration approach** - Embedded vs Standalone vs Hybrid
3. **Implement API format changes** - Align responses with PYRO
4. **Build UI component package** - Extract Svelte components
5. **Test integration** - Mock PYRO environment

**Questions to Resolve:**
- Does PYRO use Svelte or React for main UI?
- What's PYRO's authentication mechanism (JWT, OAuth, custom)?
- Does PYRO have existing API response format standards?
- What's the PYRO database choice (PostgreSQL, MongoDB, other)?
- How does PYRO handle multi-tenancy?
