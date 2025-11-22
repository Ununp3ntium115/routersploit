# PyRouterSploit Node-RED Nodes

Custom Node-RED nodes for integrating PyRouterSploit security testing framework into visual workflows.

## Installation

```bash
cd nodered
npm install
npm link
```

Then in your Node-RED user directory:
```bash
npm link @pyroutersploit/node-red-contrib-pyroutersploit
```

## Available Nodes

### 1. PyRouterSploit Scanner
Scans targets for security vulnerabilities.

**Input:**
- `payload.target` - IP address or hostname
- `payload.scanType` - Type of scan (autopwn, http, ssh, telnet, all)
- `payload.threads` - Number of threads

**Output:**
- Scan results with vulnerabilities found

### 2. PyRouterSploit Exploit
Executes exploits against targets.

**Input:**
- `payload.target` - Target IP/hostname
- `payload.exploit` - Exploit name or branding name
- `payload.options` - Additional exploit options

**Output:**
- Exploit execution results

### 3. Cryptex Dictionary
Query the cryptex function-to-branding mapping.

**Input:**
- `payload.query` - Search term
- `payload.type` - Query type (function, branding, search, category)

**Output:**
- Matching cryptex entries

### 4. QKD Encryption
Encrypt data using Quantum Key Distribution.

**Input:**
- `payload.data` - Data to encrypt
- `payload.keySize` - Key size in bytes (default: 32)

**Output:**
- Encrypted ciphertext and session ID

### 5. Multi-Hash
Hash data with multiple algorithms.

**Input:**
- `payload.data` - Data to hash
- `payload.algorithm` - Specific algorithm or 'all'

**Output:**
- Hash results for specified algorithm(s)

### 6. Database Query
Query PyRouterSploit redb database.

**Input:**
- `payload.collection` - Collection to query (exploits, scans, cryptex, etc.)
- `payload.filter` - Query filter

**Output:**
- Query results

## Example Workflow

```json
[Scanner] → [Filter Vulnerable] → [Exploit] → [Report]
    ↓
[QKD Encrypt] → [Store Results]
```

## API Server

Ensure the PyRouterSploit API server is running:

```bash
cd rust
cargo build --release
./target/release/pyroutersploit serve --host 127.0.0.1 --port 8080
```

## License

MIT
