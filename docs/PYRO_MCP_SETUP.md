# Accessing PYRO Platform Ignition MCP Server

## Current Situation

To properly integrate PyRouterSploit with PYRO Fire Marshal, I need access to the **PYRO Platform Ignition MCP server** to query:

1. **API Specifications**
   - Authentication mechanism
   - Request/response formats
   - Endpoint structure
   - Error handling

2. **Platform Architecture**
   - UI framework (Svelte/React/other)
   - Database type (PostgreSQL/MongoDB/other)
   - Multi-tenancy design
   - Service discovery

3. **Integration Requirements**
   - Webhook formats
   - Event streaming protocols
   - Training model data format
   - Compliance reporting structure

## How to Connect to PYRO Platform MCP

### Option 1: Add PYRO MCP Server to Claude Config

If the PYRO Platform Ignition MCP server is available, add it to Claude's configuration:

**File:** `~/.config/claude/claude_desktop_config.json`
```json
{
  "mcpServers": {
    "pyro-platform-ignition": {
      "command": "/path/to/pyro-platform-ignition/mcp-server",
      "args": ["--mode", "stdio"],
      "env": {
        "PYRO_CONFIG": "/path/to/pyro/config.json"
      }
    }
  }
}
```

Then I can query it with MCP tools like:
```
- get_platform_specs
- get_api_schema
- get_integration_requirements
- get_ui_framework_info
```

### Option 2: Provide PYRO Repository Access

If the PYRO Platform Ignition repository is available:

```bash
git clone https://github.com/Ununp3ntium115/PYRO_Platform_Ignition.git
cd PYRO_Platform_Ignition
# I can then analyze the codebase
```

### Option 3: Provide PYRO Documentation

Share any of the following:
- API documentation (OpenAPI/Swagger specs)
- Architecture diagrams
- Integration guides
- Database schemas
- Authentication flow documentation

### Option 4: Manual Specification

Provide answers to these questions:

1. **Authentication:**
   - What auth mechanism? (JWT, OAuth2, API Key, Custom)
   - Token format and claims?
   - Refresh token strategy?

2. **API Format:**
   - Standard response structure?
   - Error response format?
   - Pagination approach?
   - Versioning strategy?

3. **UI Framework:**
   - Svelte, React, Vue, or other?
   - Component library in use?
   - Theming approach?

4. **Database:**
   - PostgreSQL, MongoDB, MySQL, or other?
   - Schema for shared tables?
   - Multi-tenancy approach?

5. **Integration Points:**
   - Webhook URL format?
   - Event streaming protocol (WebSocket, SSE, Kafka)?
   - Training model data format?

## What I'll Query from PYRO MCP

Once connected, I'll use the PYRO MCP server to get:

### 1. Platform Specifications
```typescript
// MCP Tool: get_platform_specs
{
  "platform_name": "PYRO Fire Marshal",
  "version": "x.x.x",
  "api_version": "v1",
  "authentication": {
    "type": "jwt",
    "issuer": "https://pyro-platform.local",
    "algorithm": "RS256"
  },
  "database": {
    "type": "postgresql",
    "version": "15.x",
    "schema_version": "1.2.0"
  },
  "ui_framework": {
    "framework": "svelte",
    "version": "4.x",
    "component_library": "carbon-components-svelte"
  }
}
```

### 2. API Schema
```typescript
// MCP Tool: get_api_schema
{
  "gateway_url": "https://pyro-platform.local/api/v1",
  "services": [
    {
      "name": "vulnerability-scanning",
      "path": "/vulnscan",
      "provider": "pyroutersploit"
    }
  ],
  "response_format": {
    "success": {
      "status": "success",
      "data": {},
      "meta": {}
    },
    "error": {
      "status": "error",
      "error": {
        "code": "string",
        "message": "string"
      }
    }
  }
}
```

### 3. Integration Requirements
```typescript
// MCP Tool: get_integration_requirements
{
  "webhooks": {
    "format": "json",
    "authentication": "hmac-sha256",
    "retry_policy": "exponential_backoff"
  },
  "events": {
    "protocol": "websocket",
    "url": "wss://pyro-platform.local/ws/v1"
  },
  "training_models": {
    "input_format": "json",
    "schema_version": "1.0",
    "endpoint": "/api/v1/models/ingest"
  }
}
```

## Temporary Workaround

Until I have access to the PYRO MCP server, I've created:

1. **Assumed specifications** in [PYRO_API_MAPPING.md](./PYRO_API_MAPPING.md)
2. **Placeholder integration** that can be updated once specs are known
3. **Flexible architecture** that can adapt to actual PYRO requirements

## Next Steps

Please provide **one of the following**:

1. ✅ **PYRO MCP server connection details**
2. ✅ **PYRO Platform Ignition repository URL**
3. ✅ **PYRO API documentation**
4. ✅ **Manual answers to the questions above**

Once I have access, I can:
- Update API implementation to match actual PYRO format
- Configure authentication properly
- Build UI components compatible with PYRO's framework
- Implement exact integration requirements
- Generate accurate documentation

---

**Current Status:** Waiting for PYRO platform specifications to finalize integration.
