# PyRouterSploit Integration Strategy

## Dual-Mode Architecture

PyRouterSploit operates in two modes:

### 1. **Standalone Mode**
Independent vulnerability testing tool with full CLI, UI, and API

### 2. **PYRO Fire Marshal Integration Mode**
Vulnerability testing engine integrated into the PYRO platform

---

## Business Logic: Vulnerability Testing

### Core Capabilities

**Primary Function:** Automated vulnerability discovery and exploitation testing for embedded devices

**Target Devices:**
- Routers (D-Link, Linksys, Netgear, Cisco, etc.)
- IP Cameras
- IoT Devices
- Network Appliances
- Printers
- Industrial Control Systems

**Testing Workflow:**
```
1. Discovery → 2. Scanning → 3. Vulnerability Detection → 4. Exploitation → 5. Reporting
```

---

## Architecture: Dual-Mode Design

### Standalone Architecture

```
┌─────────────────────────────────────────┐
│         PyRouterSploit Standalone        │
│                                          │
│  ┌────────────┐  ┌─────────────────┐   │
│  │  Svelte UI │  │  CLI Interface  │   │
│  └─────┬──────┘  └────────┬────────┘   │
│        │                  │             │
│        └──────────┬───────┘             │
│                   │                     │
│        ┌──────────▼──────────┐         │
│        │   REST/WS API       │         │
│        └──────────┬──────────┘         │
│                   │                     │
│        ┌──────────▼──────────┐         │
│        │   Rust Core Engine  │         │
│        │  - Vulnerability DB │         │
│        │  - Exploit Engine   │         │
│        │  - Scanner          │         │
│        │  - QKD Crypto       │         │
│        └─────────────────────┘         │
└─────────────────────────────────────────┘
```

### PYRO Fire Marshal Integration Architecture

```
┌───────────────────────────────────────────────────────────────┐
│                 PYRO Fire Marshal Platform                     │
│                                                                 │
│  ┌─────────────┐  ┌──────────────┐  ┌──────────────────┐     │
│  │  Monitoring │  │   Training   │  │   Compliance     │     │
│  │   Module    │  │   Models     │  │    Reporting     │     │
│  └──────┬──────┘  └──────┬───────┘  └────────┬─────────┘     │
│         │                 │                    │               │
│         └─────────────────┴────────────────────┘               │
│                           │                                    │
│                ┌──────────▼──────────┐                        │
│                │  PYRO Integration   │                        │
│                │       Layer         │                        │
│                └──────────┬──────────┘                        │
│                           │                                    │
│         ┌─────────────────┴─────────────────┐                │
│         │                                    │                │
│  ┌──────▼────────┐              ┌───────────▼──────────┐    │
│  │ PyRouterSploit│              │   Other Security     │    │
│  │ Vuln Testing  │              │      Modules         │    │
│  │    Engine     │              └──────────────────────┘    │
│  │               │                                           │
│  │ - REST API    │                                           │
│  │ - MCP Server  │                                           │
│  │ - Scan Engine │                                           │
│  │ - Results DB  │                                           │
│  └───────────────┘                                           │
└───────────────────────────────────────────────────────────────┘
```

---

## Integration Points

### 1. REST API Integration (Primary)

**Endpoint Structure:**
```
/api/v1/
├── /scan           # Initiate vulnerability scan
├── /exploits       # List/execute exploits
├── /results        # Fetch scan results
├── /status         # Get scan status
└── /config         # Configuration management
```

**Example Integration:**
```rust
// PYRO Fire Marshal calls PyRouterSploit API
POST /api/v1/scan
{
  "targets": ["192.168.1.0/24"],
  "scan_type": "comprehensive",
  "callback_url": "https://pyro-platform/webhooks/scan-complete"
}

Response:
{
  "scan_id": "550e8400-e29b-41d4-a716-446655440000",
  "status": "running",
  "estimated_completion": "2025-01-22T14:30:00Z"
}
```

### 2. MCP Server Integration (AI/Automation)

**PYRO Fire Marshal MCP Tools:**
```json
{
  "name": "pyro_vulnerability_scan",
  "description": "Scan network for vulnerabilities using PyRouterSploit",
  "input_schema": {
    "target": "string",
    "depth": "enum[quick, standard, comprehensive]"
  }
}
```

**LLM-Driven Workflow:**
```
User: "Scan our production network for router vulnerabilities"
  ↓
PYRO Fire Marshal LLM
  ↓
Calls pyro_vulnerability_scan MCP tool
  ↓
PyRouterSploit executes scan
  ↓
Results → PYRO Training Models
  ↓
Risk assessment & recommendations
```

### 3. Database Integration

**Shared Data Model:**
```rust
// PyRouterSploit exports scan results
pub struct VulnerabilityScanResult {
    pub scan_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub target: String,
    pub vulnerabilities: Vec<Vulnerability>,
    pub risk_score: f64,
    pub metadata: HashMap<String, String>,
}

// PYRO ingests into training models
impl From<VulnerabilityScanResult> for PyroTrainingData {
    fn from(result: VulnerabilityScanResult) -> Self {
        // Convert to PYRO format
    }
}
```

### 4. Event Stream Integration

**WebSocket Events:**
```rust
// PyRouterSploit streams real-time scan events
enum ScanEvent {
    Started { scan_id: Uuid, targets: Vec<String> },
    TargetScanned { target: String, vulnerabilities: usize },
    VulnerabilityFound { target: String, severity: Severity, cve: Option<String> },
    Completed { scan_id: Uuid, total_vulns: usize },
    Error { scan_id: Uuid, error: String },
}

// PYRO subscribes to WebSocket
ws://pyroutersploit:8080/ws/scans/{scan_id}
```

### 5. Node-RED Workflow Integration

**PYRO Fire Marshal Workflow:**
```
[Scheduled Scan] → [PyRouterSploit Scanner] → [Filter Critical]
     ↓
[Alert PYRO Dashboard] → [Update Training Model] → [Generate Report]
     ↓
[Compliance Check] → [Archive Results] → [QKD Encrypt Sensitive Data]
```

---

## Configuration Modes

### Standalone Configuration

**File:** `pyroutersploit.toml`
```toml
[mode]
type = "standalone"

[api]
host = "0.0.0.0"
port = 8080
enable_auth = true

[database]
path = "./data/pyroutersploit.redb"

[ui]
enabled = true
port = 5173
```

### PYRO Integration Configuration

**File:** `pyroutersploit.toml`
```toml
[mode]
type = "pyro_integrated"

[pyro]
platform_url = "https://pyro-fire-marshal.local"
api_key_env = "PYRO_API_KEY"
report_endpoint = "/api/v1/vulnerability-reports"

[api]
host = "127.0.0.1"  # Only accessible to PYRO
port = 8080
enable_auth = false  # PYRO handles auth

[database]
# Use PYRO's shared database
type = "postgres"
connection_string_env = "PYRO_DB_URL"

[ui]
enabled = false  # PYRO provides UI
```

---

## Business Logic Implementation

### Core Vulnerability Testing Workflow

```rust
// PyRouterSploit business logic entry point
pub async fn run_vulnerability_scan(config: ScanConfig) -> Result<ScanResult> {
    // 1. Discovery Phase
    let targets = discover_targets(&config.target_range).await?;

    // 2. Port Scanning
    let open_services = scan_ports(&targets, &config.ports).await?;

    // 3. Service Identification
    let identified_services = identify_services(&open_services).await?;

    // 4. Vulnerability Matching
    let potential_vulns = match_vulnerabilities(&identified_services).await?;

    // 5. Exploitation Testing (if enabled)
    let confirmed_vulns = if config.verify_exploits {
        test_exploits(&potential_vulns).await?
    } else {
        potential_vulns
    };

    // 6. Risk Scoring
    let risk_score = calculate_risk_score(&confirmed_vulns);

    // 7. Generate Report
    let report = generate_report(ScanResult {
        scan_id: Uuid::new_v4(),
        timestamp: Utc::now(),
        targets,
        vulnerabilities: confirmed_vulns,
        risk_score,
        metadata: config.metadata,
    });

    // 8. Store Results
    db::scans::insert(&report).await?;

    // 9. If integrated with PYRO, send results
    if let Some(pyro_config) = config.pyro_integration {
        send_to_pyro(&report, &pyro_config).await?;
    }

    Ok(report)
}
```

### Vulnerability Database Schema

```rust
pub struct Vulnerability {
    pub id: Uuid,
    pub cve_id: Option<String>,
    pub name: String,
    pub description: String,
    pub severity: Severity,
    pub affected_devices: Vec<String>,
    pub affected_versions: Vec<String>,
    pub exploit_available: bool,
    pub exploit_public: bool,
    pub proof_of_concept: Option<String>,
    pub remediation: String,
    pub references: Vec<String>,
    pub discovered_date: DateTime<Utc>,
    pub disclosed_date: Option<DateTime<Utc>>,
    pub cvss_score: Option<f64>,
    pub attack_vector: AttackVector,
    pub attack_complexity: AttackComplexity,
}

pub enum AttackVector {
    Network,
    Adjacent,
    Local,
    Physical,
}

pub enum AttackComplexity {
    Low,
    High,
}
```

---

## PYRO Fire Marshal Integration API

### Scan Initiation

```http
POST /api/v1/scan
Content-Type: application/json
X-PYRO-API-Key: <api_key>

{
  "target_range": "10.0.0.0/24",
  "scan_type": "comprehensive",
  "verify_exploits": true,
  "max_threads": 50,
  "timeout_per_target": 300,
  "callbacks": {
    "on_complete": "https://pyro/webhooks/scan-complete",
    "on_vulnerability": "https://pyro/webhooks/vulnerability-found"
  },
  "metadata": {
    "environment": "production",
    "compliance_framework": "NIST",
    "requester": "pyro-fire-marshal"
  }
}
```

### Results Retrieval

```http
GET /api/v1/results/{scan_id}
X-PYRO-API-Key: <api_key>

Response:
{
  "scan_id": "550e8400-e29b-41d4-a716-446655440000",
  "status": "completed",
  "started_at": "2025-01-22T14:00:00Z",
  "completed_at": "2025-01-22T14:45:00Z",
  "targets_scanned": 254,
  "vulnerabilities_found": 47,
  "critical": 5,
  "high": 12,
  "medium": 20,
  "low": 10,
  "risk_score": 78.5,
  "details": [
    {
      "target": "10.0.0.1",
      "device_type": "D-Link Router",
      "vulnerabilities": [
        {
          "cve_id": "CVE-2024-XXXX",
          "severity": "critical",
          "exploit_available": true,
          "verified": true
        }
      ]
    }
  ]
}
```

---

## Training Model Integration

### Feeding Data to PYRO Training Models

```rust
// PyRouterSploit → PYRO Training Pipeline
pub async fn export_training_data(scan_result: &ScanResult) -> Result<TrainingDataset> {
    TrainingDataset {
        features: extract_features(scan_result),
        labels: classify_vulnerabilities(scan_result),
        metadata: {
            "scan_id": scan_result.scan_id,
            "timestamp": scan_result.timestamp,
            "target_count": scan_result.targets.len(),
        },
    }
}

// Features for ML model
fn extract_features(result: &ScanResult) -> Vec<Feature> {
    vec![
        Feature::DeviceType(result.device_info.type),
        Feature::FirmwareVersion(result.device_info.firmware),
        Feature::OpenPorts(result.open_ports),
        Feature::ServiceVersions(result.services),
        Feature::ResponsePatterns(result.fingerprints),
    ]
}
```

---

## Deployment Scenarios

### Scenario 1: Standalone Deployment

```bash
# Build and run standalone
./scripts/build.sh
./rust/target/release/pyroutersploit serve --mode standalone

# Access UI
http://localhost:5173

# Access API
http://localhost:8080/api/v1
```

### Scenario 2: PYRO Fire Marshal Integration

```bash
# Deploy as PYRO module
docker-compose -f docker-compose.pyro.yml up -d

# PYRO accesses via internal network
http://pyroutersploit:8080/api/v1
```

### Scenario 3: Hybrid Deployment

```bash
# Run both modes simultaneously
./rust/target/release/pyroutersploit serve \
  --mode hybrid \
  --standalone-port 8080 \
  --pyro-integration-port 8081
```

---

## Security Considerations

### Authentication in Integration Mode

```rust
// PYRO-provided JWT validation
pub async fn validate_pyro_token(token: &str) -> Result<PyroClaims> {
    let jwks_url = env::var("PYRO_JWKS_URL")?;
    let jwks = fetch_jwks(&jwks_url).await?;

    let claims: PyroClaims = jsonwebtoken::decode(
        token,
        &jwks,
        &Validation::default(),
    )?.claims;

    Ok(claims)
}
```

### Data Encryption in Transit

```rust
// Use QKD encryption for sensitive results sent to PYRO
pub async fn send_to_pyro_encrypted(
    result: &ScanResult,
    pyro_config: &PyroConfig,
) -> Result<()> {
    let qkd = QKDEncryption::new_session(32)?;
    let encrypted = qkd.encrypt(&serde_json::to_vec(result)?)?;

    reqwest::Client::new()
        .post(&pyro_config.endpoint)
        .header("X-QKD-Session", qkd.session_id.to_string())
        .body(encrypted)
        .send()
        .await?;

    Ok(())
}
```

---

## API Versioning Strategy

```
/api/v1/  - Current stable API for PYRO integration
/api/v2/  - Next generation (when available)
/api/internal/  - Internal-only endpoints (health, metrics)
```

---

## Next Steps for Integration

1. **Define PYRO Fire Marshal API Contract** (2h)
   - Finalize webhook endpoints
   - Define data schemas
   - Authentication mechanism

2. **Implement REST API Endpoints** (12h)
   - All scan operations
   - Results retrieval
   - Configuration management

3. **Build PYRO Integration Layer** (8h)
   - JWT validation
   - Webhook delivery
   - Shared database connector

4. **Create Integration Tests** (6h)
   - Mock PYRO platform
   - End-to-end workflows
   - Error handling

5. **Documentation** (4h)
   - Integration guide
   - API reference
   - Example workflows

**Total Integration Effort:** ~32 hours

---

## Summary

PyRouterSploit serves dual purposes:
1. **Standalone:** Full-featured vulnerability testing tool
2. **Integrated:** Vulnerability testing engine for PYRO Fire Marshal

The architecture supports both modes through:
- Configuration-driven mode selection
- RESTful API for platform integration
- MCP servers for LLM-driven automation
- WebSocket for real-time updates
- Shared data models for training
- QKD encryption for secure data transfer

The business logic focuses on **automated vulnerability discovery and verification** for embedded devices, feeding results to PYRO's broader security platform.
