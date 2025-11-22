# PyRouterSploit Quick Start Guide

Get up and running with PyRouterSploit in 5 minutes!

## ⚡ Quick Install

```bash
# 1. Clone repository
git clone https://github.com/Ununp3ntium115/routersploit.git
cd routersploit

# 2. Build Rust core
cd rust && cargo build --release && cd ..

# 3. Initialize database
./rust/target/release/pyroutersploit init --populate-cryptex

# 4. You're ready!
./rust/target/release/pyroutersploit --help
```

## 🎯 Basic Usage

### 1. Query the Cryptex Dictionary

The Cryptex Dictionary maps function names to branding names:

```bash
# Search for exploits
./rust/target/release/pyroutersploit cryptex --search dlink

# List all entries
./rust/target/release/pyroutersploit cryptex --list-all
```

**Example Output:**
```
📚 Cryptex Dictionary (12 entries):

  • exploit_dlink_rce_hnap → pyroutersploit_dlink_hnap_pwn
    Category: Exploit
    Execute remote code on D-Link routers via HNAP vulnerability

  • scanner_router_autopwn → pyroutersploit_autopwn_scanner
    Category: Scanner
    Automated vulnerability scanner for routers across all protocols
```

### 2. Hash Data with Multiple Algorithms

```bash
# Hash with all algorithms
./rust/target/release/pyroutersploit hash --data "test" --all

# Hash with specific algorithm
./rust/target/release/pyroutersploit hash --data "test" --algorithm SHA256
```

**Output:**
```
🔐 Hashing 'test' with all algorithms:

  SHA256          : 9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08
  SHA512          : ee26b0dd4af7e749aa1a8ee3c10ae9923f618980772e473f8819a5d4940e0db2...
  BLAKE3          : 4878ca0425c739fa427f7eda20fe845f6b2e46ba5fe2a14df5b1e32f50603215
  SHA3_256        : 36f028580bb02cc8272a9a020f4200e346e276ae664e45ee80745574e2f5ab80
```

### 3. QKD Encryption

```bash
# Generate a quantum-derived key
./rust/target/release/pyroutersploit qkd generate-key --size 32

# Encrypt data
./rust/target/release/pyroutersploit qkd encrypt --data "secret message"
```

**Output:**
```
🔐 Encrypted (hex): a3f5b2c1d4e6...
Session ID: 550e8400-e29b-41d4-a716-446655440000
✓ Session saved to database
```

### 4. Start the API Server

```bash
./rust/target/release/pyroutersploit serve --host 127.0.0.1 --port 8080
```

**Test the API:**
```bash
# Health check
curl http://localhost:8080/health

# List exploits (when implemented)
curl http://localhost:8080/api/exploits
```

### 5. Use the MCP Server

```bash
# Start MCP server
./rust/target/release/pyroutersploit mcp --transport stdio
```

**Configure in Claude Desktop** (`.config/claude/claude_desktop_config.json`):
```json
{
  "mcpServers": {
    "pyroutersploit": {
      "command": "/path/to/routersploit/rust/target/release/pyroutersploit",
      "args": ["mcp", "--transport", "stdio"],
      "env": {
        "PYROUTERSPLOIT_DB_PATH": "/path/to/routersploit/data/pyroutersploit.redb"
      }
    }
  }
}
```

**Query from Claude:**
```
"Can you query the cryptex dictionary for D-Link exploits?"
```

Claude will use the MCP server to:
1. Call `cryptex_query` tool
2. Search for "dlink"
3. Return matching entries

## 🖥️ Web UI

```bash
# Install dependencies
cd ui
npm install

# Start development server
npm run dev

# Visit http://localhost:5173
```

**Features:**
- Dashboard with statistics
- Exploit browser
- Scanner interface
- Cryptex dictionary viewer
- QKD encryption console

## 🔄 Node-RED Integration

### Install Nodes

```bash
cd nodered
npm install
npm link

# In Node-RED directory
cd ~/.node-red
npm link @pyroutersploit/node-red-contrib-pyroutersploit
```

### Example Flow

1. Drag **PyRouterSploit Scanner** node
2. Configure target: `192.168.1.1`
3. Connect to **Debug** node
4. Deploy and inject

```
[Scanner] → [Debug]
```

### Advanced Workflow

```
[Inject] → [Scanner] → [Function: Filter Vulnerable] → [Exploit] → [QKD Encrypt] → [File]
```

## 🤖 MCP Tools Reference

### cryptex_query
Search the cryptex dictionary:

```json
{
  "function_name": "exploit_dlink_rce_hnap",
  // or
  "branding_name": "pyroutersploit_dlink_hnap_pwn",
  // or
  "search": "dlink",
  // or
  "category": "Exploit"
}
```

### multi_hash
Hash data:

```json
{
  "data": "test",
  "algorithm": "SHA256",
  // or
  "all_algorithms": true
}
```

### qkd_encrypt
Encrypt with QKD:

```json
{
  "data": "secret message",
  "key_size": 32
}
```

## 📊 Project Status

| Component | Status | Notes |
|-----------|--------|-------|
| Rust Core | ✅ Complete | Database, crypto, MCP server |
| Cryptex Dictionary | ✅ Complete | Full CRUD operations |
| QKD Encryption | ✅ Complete | BB84 simulation + PQC |
| Multi-Hash | ✅ Complete | 17 algorithms |
| MCP Server | ✅ Complete | 7 tools available |
| REST API | 🚧 In Progress | Basic routes |
| Exploit Engine | 🚧 In Progress | Framework ready |
| Scanner Engine | 🚧 In Progress | Autopwn planned |
| Python Compat | 📋 Planned | PyO3 FFI bridge |
| Node-RED Nodes | ✅ Complete | 6 custom nodes |
| Svelte UI | ✅ Complete | Dashboard + routes |

## 🎯 Next Steps

### Phase 1: Core (Current)
- ✅ Database layer (redb)
- ✅ Cryptex dictionary
- ✅ QKD encryption
- ✅ MCP servers
- ✅ CLI interface

### Phase 2: Integration (Next)
- 🚧 REST API completion
- 🚧 WebSocket for real-time updates
- 🚧 Python FFI bridge
- 📋 Exploit engine implementation
- 📋 Scanner engine

### Phase 3: Production (Future)
- 📋 Training model integration
- 📋 Advanced QKD features
- 📋 Performance optimization
- 📋 Docker containers
- 📋 Kubernetes deployment

## 🐛 Troubleshooting

### Build Errors

**Problem:** `cargo build` fails with dependency errors

**Solution:**
```bash
rustup update
cargo clean
cargo build --release
```

### Database Not Found

**Problem:** `Database not initialized`

**Solution:**
```bash
mkdir -p data
./rust/target/release/pyroutersploit init --populate-cryptex
```

### MCP Server Not Connecting

**Problem:** Claude can't connect to MCP server

**Solution:**
1. Check absolute paths in config
2. Ensure binary is executable: `chmod +x rust/target/release/pyroutersploit`
3. Test manually: `./rust/target/release/pyroutersploit mcp`

## 💡 Tips

1. **Use Cryptex Search**: Find exploits by vendor, type, or keyword
2. **Try All Hash Algorithms**: Use `--all` to see all hash outputs
3. **Save QKD Sessions**: Session IDs are stored in database for later decryption
4. **Explore MCP Tools**: Each tool has detailed input schema
5. **Chain Node-RED Nodes**: Build complex automation workflows

## 📚 Further Reading

- [Full Architecture](./ARCHITECTURE.md)
- [MCP Server API](./docs/MCP.md)
- [Node-RED Integration](./nodered/README.md)
- [Svelte UI Guide](./ui/README.md)

## 🆘 Getting Help

- **GitHub Issues**: Bug reports and feature requests
- **Discussions**: Q&A and community support
- **Documentation**: Complete API reference

---

**Happy Hacking! 🚀**
