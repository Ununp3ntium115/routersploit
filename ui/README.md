# PyRouterSploit Svelte UI

Modern, reactive web interface for PyRouterSploit security testing framework.

## Features

- 📊 **Dashboard** - Real-time statistics and overview
- 🔍 **Exploit Browser** - Browse and search available exploits
- 🎯 **Scanner Interface** - Configure and run security scans
- 📚 **Cryptex Dictionary** - Manage function-to-branding mappings
- 🔐 **QKD Crypto Console** - Quantum key distribution encryption
- 📈 **Results Visualization** - Charts and graphs for scan results

## Tech Stack

- **Svelte/SvelteKit** - Reactive frontend framework
- **Carbon Components** - IBM Design System
- **TypeScript** - Type-safe development
- **Vite** - Fast build tool
- **Axios** - HTTP client

## Development

### Prerequisites

- Node.js 18+
- PyRouterSploit API server running on `localhost:8080`

### Setup

```bash
cd ui
npm install
```

### Run Development Server

```bash
npm run dev
```

Visit `http://localhost:5173`

### Build for Production

```bash
npm run build
```

### Preview Production Build

```bash
npm run preview
```

## Project Structure

```
ui/
├── src/
│   ├── routes/
│   │   ├── +page.svelte          # Dashboard
│   │   ├── exploits/
│   │   │   └── +page.svelte      # Exploit browser
│   │   ├── scanners/
│   │   │   └── +page.svelte      # Scanner interface
│   │   ├── cryptex/
│   │   │   └── +page.svelte      # Cryptex dictionary
│   │   └── qkd/
│   │       └── +page.svelte      # QKD encryption console
│   ├── lib/
│   │   ├── components/           # Reusable components
│   │   ├── stores/               # Svelte stores
│   │   └── api/
│   │       └── client.ts         # API client
│   └── app.html                  # HTML template
├── package.json
└── svelte.config.js
```

## API Integration

The UI communicates with the PyRouterSploit Rust backend via REST API:

- `GET /api/exploits` - List exploits
- `POST /api/exploits/run` - Execute exploit
- `POST /api/scan` - Start scan
- `GET /api/cryptex` - Query cryptex
- `POST /api/qkd/encrypt` - QKD encryption
- `POST /api/hash` - Multi-algorithm hashing

WebSocket connection for real-time scan updates:
- `ws://localhost:8080/ws/scans/{id}`

## Environment Variables

Create `.env` file:

```env
VITE_API_URL=http://localhost:8080
```

## Components

### Dashboard
- Statistics tiles
- Quick action buttons
- Recent activity feed

### Exploit Browser
- Search and filter exploits
- View exploit details
- Execute against targets

### Scanner
- Target configuration
- Scan type selection
- Real-time progress updates
- Results table

### Cryptex
- Search dictionary entries
- Add new mappings
- View function/branding relationships

### QKD Crypto
- Encrypt/decrypt data
- Generate quantum keys
- View session history

## Deployment

### Static Hosting

Build and deploy to any static host:

```bash
npm run build
# Deploy contents of build/ directory
```

### Docker

```dockerfile
FROM node:18-alpine AS builder
WORKDIR /app
COPY package*.json ./
RUN npm ci
COPY . .
RUN npm run build

FROM nginx:alpine
COPY --from=builder /app/build /usr/share/nginx/html
EXPOSE 80
```

### Tauri (Desktop App)

Convert to desktop application:

```bash
npm install --save-dev @tauri-apps/cli
npm run tauri build
```

## License

MIT
