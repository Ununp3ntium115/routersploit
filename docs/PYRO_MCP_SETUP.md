# PYRO Platform Ignition MCP Server

## ✅ Setup Complete!

The **PYRO Platform Ignition MCP server** has been created and is now available!

**Repository:** https://github.com/Ununp3ntium115/PYRO_Platform_Ignition

This MCP server provides comprehensive platform specifications for integrating PyRouterSploit with PYRO Fire Marshal.

## What It Provides

The PYRO Platform Ignition MCP server exposes tools to query:

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

## Installation

### 1. Clone and Build PYRO Platform Ignition

```bash
# Clone the repository
git clone https://github.com/Ununp3ntium115/PYRO_Platform_Ignition.git
cd PYRO_Platform_Ignition

# Install dependencies
npm install

# Build TypeScript
npm run build
```

### 2. Add to Claude Desktop Configuration

**File:** `~/.config/claude/claude_desktop_config.json`

```json
{
  "mcpServers": {
    "pyro-platform-ignition": {
      "command": "node",
      "args": ["/path/to/PYRO_Platform_Ignition/dist/index.js"],
      "env": {
        "NODE_ENV": "production"
      }
    }
  }
}
```

**Replace `/path/to/PYRO_Platform_Ignition/` with the actual path.**

### 3. Restart Claude Desktop

After updating the configuration, restart Claude Desktop to load the MCP server.

## Available MCP Tools

The PYRO Platform Ignition MCP server provides 8 tools:

1. **`get_platform_specs`** - Platform specifications (auth, database, UI, services, security)
2. **`get_api_schema`** - Complete API schema with all endpoints
3. **`get_integration_requirements`** - Integration requirements (data, UI, auth, config, monitoring)
4. **`get_webhook_config`** - Webhook configuration
5. **`get_training_model_schema`** - Training model data schema
6. **`get_vulnerability_api_endpoints`** - Vulnerability scanning API endpoints
7. **`get_ui_integration_options`** - UI integration modes (embedded/standalone/hybrid)
8. **`get_deployment_config`** - Deployment configuration

## Usage Examples

### Query Authentication Specs

```typescript
// Use the get_platform_specs tool
{
  "name": "get_platform_specs",
  "arguments": {
    "section": "authentication"
  }
}

// Returns:
{
  "type": "jwt",
  "issuer": "https://pyro-platform.local",
  "algorithm": "RS256",
  "token_lifetime": 3600,
  "refresh_token_lifetime": 86400,
  "jwks_url": "https://pyro-platform.local/.well-known/jwks.json"
}
```

### Get Vulnerability API Endpoints

```typescript
// Use the get_vulnerability_api_endpoints tool
{
  "name": "get_vulnerability_api_endpoints"
}

// Returns all /api/v1/vulnscan/* endpoints with full details
```

### Check UI Integration Options

```typescript
// Use the get_ui_integration_options tool
{
  "name": "get_ui_integration_options",
  "arguments": {
    "mode": "embedded"  // or "standalone", "hybrid", "all"
  }
}
```

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

## Platform Specifications Summary

The PYRO Platform Ignition MCP server provides these confirmed specifications:

### Authentication
- **Type:** JWT with RS256 algorithm
- **Issuer:** https://pyro-platform.local
- **Token Lifetime:** 3600 seconds (1 hour)
- **Refresh Token:** 86400 seconds (24 hours)
- **SSO:** OAuth2 integration

### Database
- **Type:** PostgreSQL 15.x
- **Multi-tenancy:** Schema-per-tenant strategy
- **Shared Tables:** users, tenants, audit_logs

### UI Framework
- **Framework:** Svelte 4.x + SvelteKit
- **Component Library:** Carbon Components Svelte
- **Styling:** SCSS with CSS variables theming
- **Build Tool:** Vite

### API Structure
- **Base URL:** https://pyro-platform.local/api/v1
- **PyRouterSploit Namespace:** `/api/v1/vulnscan/*`
- **Response Format:** Standardized JSON with `{status, data, meta}` structure

### Integration
- **Event Streaming:** WebSocket (wss://pyro-platform.local/ws/v1)
- **Webhooks:** HMAC-SHA256 authentication with exponential backoff
- **Training Models:** JSON format ingestion to `/api/v1/models/ingest`

## Next Steps for PyRouterSploit

Now that we have the PYRO platform specifications, the next steps are:

1. ✅ **PYRO MCP Server Created**
2. 🚧 **Update PyRouterSploit API** to match PYRO response format
3. 🚧 **Implement JWT Authentication** with RS256 validation
4. 🚧 **Build UI Components** compatible with Carbon Components Svelte
5. 🚧 **Configure Database** for PostgreSQL shared schema access
6. 🚧 **Implement WebSocket** event streaming
7. 🚧 **Add Training Model** data export pipeline

---

**Status:** ✅ PYRO Platform Ignition MCP Server operational
**Repository:** https://github.com/Ununp3ntium115/PYRO_Platform_Ignition
