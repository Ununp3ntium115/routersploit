# PyRouterSploit - Cross-Platform Architecture

## Vision
Transform RouterSploit into a framework-agnostic, cross-platform security testing platform with:
- **Rust Core**: High-performance, memory-safe bedrock
- **redb**: Embedded Rust database for persistent storage
- **Node-RED**: Visual workflow automation
- **Svelte**: Modern, reactive UI
- **QKD Encryption**: Quantum-resistant security layer
- **MCP Servers**: LLM-accessible programmatic interface

## Architecture Layers

```
┌─────────────────────────────────────────────────────────────┐
│                    Svelte UI Frontend                        │
│              (Web-based Dashboard & Controls)                │
└────────────────────────┬────────────────────────────────────┘
                         │ REST/WebSocket API
┌────────────────────────┴────────────────────────────────────┐
│                  Node-RED Integration Layer                  │
│         (Visual Workflow, Automation, Orchestration)         │
└────────────────────────┬────────────────────────────────────┘
                         │ gRPC/JSON-RPC
┌────────────────────────┴────────────────────────────────────┐
│                    Rust Core Engine                          │
│  ┌──────────────────────────────────────────────────────┐   │
│  │  PyRouterSploit Business Logic                       │   │
│  │  - Exploit Engine (Rust reimplementation)            │   │
│  │  - Scanner Engine                                    │   │
│  │  - Credential Testing                                │   │
│  │  - Payload Generation                                │   │
│  └──────────────────────────────────────────────────────┘   │
│  ┌──────────────────────────────────────────────────────┐   │
│  │  QKD Encryption Module                               │   │
│  │  - Multi-algorithm hashing (SHA-2/3, BLAKE, etc.)    │   │
│  │  - Quantum-resistant key distribution                │   │
│  │  - Post-quantum cryptography (Kyber, Dilithium)      │   │
│  └──────────────────────────────────────────────────────┘   │
│  ┌──────────────────────────────────────────────────────┐   │
│  │  redb Persistence Layer                              │   │
│  │  - Exploit metadata & results                        │   │
│  │  - Cryptex dictionary (function→branding mapping)    │   │
│  │  - Configuration & wordlists                         │   │
│  │  - Training model data                               │   │
│  └──────────────────────────────────────────────────────┘   │
└────────────────────────┬────────────────────────────────────┘
                         │ FFI Bridge
┌────────────────────────┴────────────────────────────────────┐
│              Python Compatibility Layer                      │
│         (Legacy module support via PyO3/ctypes)              │
└─────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────┐
│                    MCP Server Layer                          │
│  ┌──────────────────────┐  ┌─────────────────────────────┐  │
│  │ RouterSploit MCP     │  │ PYRO Platform MCP           │  │
│  │ - Cryptex Dictionary │  │ - Cross-platform tools      │  │
│  │ - Exploit API        │  │ - Integration endpoints     │  │
│  │ - Scan Operations    │  │ - Training model access     │  │
│  └──────────────────────┘  └─────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

## Technology Stack

### Core Layer (Rust)
- **Language**: Rust (2021 edition)
- **Database**: redb (embedded, ACID-compliant)
- **Async Runtime**: tokio
- **Serialization**: serde, serde_json, bincode
- **Networking**: reqwest, tokio-tungstenite
- **Cryptography**:
  - ring, rust-crypto, sha2, sha3, blake3
  - Post-quantum: pqcrypto, oqs (Open Quantum Safe)
- **FFI**: PyO3 (Python interop)

### Integration Layer (Node-RED)
- **Custom Nodes**:
  - pyroutersploit-scanner
  - pyroutersploit-exploit
  - pyroutersploit-creds
  - pyroutersploit-qkd-encrypt
  - pyroutersploit-db-query

### Frontend Layer (Svelte)
- **Framework**: SvelteKit
- **UI Components**: Carbon Components Svelte
- **State**: Svelte stores + IndexedDB
- **API Client**: Generated from OpenAPI spec
- **Real-time**: WebSocket for live scan results

### MCP Servers
- **RouterSploit MCP**: Exposes exploit/scan/creds operations
- **PYRO Platform MCP**: Integration hub for cross-platform tools
- **Protocol**: JSON-RPC 2.0 over stdio/HTTP

## Key Components

### 1. Cryptex Dictionary
Branding and function mapping system stored in redb:

```rust
struct CryptexEntry {
    function_name: String,        // e.g., "exploit_dlink_rce"
    branding_name: String,        // e.g., "pyroutersploit_dlink_pwn"
    pseudo_code: String,          // Human-readable description
    rust_impl: Option<String>,    // Rust implementation path
    python_impl: Option<String>,  // Legacy Python module path
    metadata: HashMap<String, String>,
}
```

### 2. QKD Encryption Engine
Comprehensive hashing and quantum-resistant encryption:

**Hash Algorithms**:
- SHA-2 family (SHA-224, SHA-256, SHA-384, SHA-512)
- SHA-3 family (SHA3-224, SHA3-256, SHA3-384, SHA3-512, SHAKE)
- BLAKE2b, BLAKE2s, BLAKE3
- Whirlpool, RIPEMD-160
- MD5, SHA-1 (legacy/compatibility only)

**Post-Quantum Crypto**:
- CRYSTALS-Kyber (key encapsulation)
- CRYSTALS-Dilithium (digital signatures)
- SPHINCS+ (stateless hash-based signatures)
- Falcon (lattice-based signatures)

**QKD Simulation**:
- BB84 protocol implementation
- Decoy state QKD
- Continuous-variable QKD

### 3. Rust Core Modules

```
rust/
├── Cargo.toml
├── src/
│   ├── main.rs                  # Entry point
│   ├── lib.rs                   # Library exports
│   ├── core/
│   │   ├── exploit.rs           # Exploit engine
│   │   ├── scanner.rs           # Scanner engine
│   │   ├── payload.rs           # Payload generation
│   │   └── session.rs           # Session management
│   ├── crypto/
│   │   ├── qkd.rs              # QKD implementation
│   │   ├── hashing.rs          # Multi-algorithm hashing
│   │   └── pqc.rs              # Post-quantum crypto
│   ├── db/
│   │   ├── redb_client.rs      # redb wrapper
│   │   ├── cryptex.rs          # Cryptex dictionary
│   │   └── models.rs           # Data models
│   ├── api/
│   │   ├── rest.rs             # REST API (axum)
│   │   ├── websocket.rs        # WebSocket server
│   │   └── grpc.rs             # gRPC services
│   ├── mcp/
│   │   ├── server.rs           # MCP server implementation
│   │   ├── handlers.rs         # Request handlers
│   │   └── schema.rs           # MCP protocol types
│   ├── nodered/
│   │   └── bridge.rs           # Node-RED integration
│   └── python_compat/
│       ├── ffi.rs              # PyO3 bindings
│       └── module_loader.rs    # Legacy module support
```

### 4. Node-RED Nodes

```
nodered/
├── package.json
└── nodes/
    ├── scanner.js
    ├── scanner.html
    ├── exploit.js
    ├── exploit.html
    ├── qkd-encrypt.js
    ├── qkd-encrypt.html
    └── db-query.js
```

### 5. Svelte Frontend

```
ui/
├── package.json
├── svelte.config.js
└── src/
    ├── routes/
    │   ├── +page.svelte         # Dashboard
    │   ├── exploits/
    │   │   └── +page.svelte     # Exploit browser
    │   ├── scanners/
    │   │   └── +page.svelte     # Scanner interface
    │   ├── cryptex/
    │   │   └── +page.svelte     # Cryptex dictionary UI
    │   └── qkd/
    │       └── +page.svelte     # QKD encryption console
    ├── lib/
    │   ├── components/          # Reusable components
    │   ├── stores/              # State management
    │   └── api/                 # API client
    └── app.html
```

## Data Flow Examples

### Exploit Execution
1. User selects exploit in Svelte UI
2. WebSocket message → Rust API server
3. Rust core loads exploit from redb/Python compat
4. Execution via Rust engine or Python FFI
5. Results streamed back via WebSocket
6. UI updates in real-time

### Node-RED Workflow
1. User creates flow: Scanner → Filter → Exploit → Report
2. Scanner node → Rust gRPC API
3. Results flow through Node-RED nodes
4. Exploit node executes vulnerable targets
5. Report node saves to redb + exports

### MCP Server Query
1. LLM requests exploit info via MCP
2. MCP server queries Cryptex dictionary (redb)
3. Returns function name, branding, pseudo-code
4. LLM can trigger execution via MCP tools

## Migration Strategy

### Phase 1: Foundation (Current)
- ✅ Analyze existing Python codebase
- 🔄 Create Rust project structure
- 🔄 Implement redb schema
- 🔄 Build MCP servers

### Phase 2: Core Rewrite
- Reimplement exploit engine in Rust
- Port critical modules (scanners, creds)
- Build Python FFI bridge for legacy modules
- Implement QKD encryption module

### Phase 3: Integration
- Create Node-RED custom nodes
- Build REST/WebSocket API
- Develop Svelte UI
- Connect all layers

### Phase 4: Enhancement
- Training model integration
- Advanced QKD features
- Performance optimization
- Documentation

## API Endpoints

### REST API (Rust/axum)
- `GET /api/exploits` - List all exploits
- `POST /api/exploits/{id}/run` - Execute exploit
- `GET /api/scanners` - List scanners
- `POST /api/scan` - Start scan
- `GET /api/cryptex` - Query cryptex dictionary
- `POST /api/cryptex` - Add cryptex entry
- `POST /api/qkd/encrypt` - Encrypt with QKD
- `POST /api/qkd/hash` - Multi-algorithm hash

### WebSocket API
- `/ws/scans/{id}` - Real-time scan results
- `/ws/exploits/{id}` - Real-time exploit output

### gRPC Services
- `ExploitService` - Exploit operations
- `ScannerService` - Scanning operations
- `CryptexService` - Dictionary management
- `QKDService` - Encryption/hashing

### MCP Tools (RouterSploit)
- `list_exploits` - Get available exploits
- `get_exploit_info` - Get exploit details
- `run_exploit` - Execute exploit
- `scan_target` - Scan for vulnerabilities
- `query_cryptex` - Search cryptex dictionary
- `add_cryptex_entry` - Add to cryptex

## Framework Agnostic Design

### Principle: Core business logic in Rust
All critical functionality lives in Rust, exposing clean interfaces:
- REST API (any client can consume)
- gRPC (language-agnostic)
- MCP (LLM-accessible)
- FFI (Python, Node.js, etc.)

### Alternative Frontends
- CLI (Rust binary)
- TUI (ratatui)
- Web (Svelte)
- Mobile (React Native consuming REST API)
- Desktop (Tauri)

### Alternative Integration
- Node-RED (current)
- Apache NiFi
- n8n
- Zapier/Make (via REST API)

## Security Considerations

1. **Isolation**: Rust core runs exploits in sandboxed environments
2. **Encryption**: All stored credentials encrypted with QKD-derived keys
3. **Audit**: All operations logged to redb with cryptographic signatures
4. **Access Control**: Role-based access via API tokens
5. **Network**: TLS 1.3 for all external communication

## Performance Targets

- Exploit execution: < 50ms overhead vs Python
- Scan throughput: 1000+ targets/minute
- Database queries: < 1ms for cryptex lookups
- UI responsiveness: < 100ms for all interactions
- WebSocket latency: < 10ms for local connections

## Branding: PyRouterSploit

While the core is Rust-based, we maintain "PyRouterSploit" branding to:
1. Honor the Python origins
2. Maintain compatibility
3. Emphasize cross-platform nature (Py = Platform-agnostic, Python-compatible)
4. Leverage existing community recognition

Function naming convention:
- Internal: `pyroutersploit_{vendor}_{attack_type}`
- External: User-friendly aliases via cryptex dictionary
- Legacy: Original Python module names supported

---

**Next Steps**: Implement Rust core, MCP servers, and begin migration.
