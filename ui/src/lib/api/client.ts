// PyRouterSploit API Client

import axios, { type AxiosInstance } from 'axios';

const API_BASE_URL = import.meta.env.VITE_API_URL || 'http://localhost:8080';

class PyRouterSploitClient {
  private client: AxiosInstance;

  constructor() {
    this.client = axios.create({
      baseURL: API_BASE_URL,
      timeout: 30000,
      headers: {
        'Content-Type': 'application/json'
      }
    });
  }

  // Cryptex operations
  async queryCryptex(params: {
    function_name?: string;
    branding_name?: string;
    search?: string;
    category?: string;
  }) {
    return this.client.get('/api/cryptex', { params });
  }

  async addCryptexEntry(entry: {
    function_name: string;
    branding_name: string;
    pseudo_code: string;
    category: string;
    rust_impl?: string;
    python_impl?: string;
  }) {
    return this.client.post('/api/cryptex', entry);
  }

  // Exploit operations
  async listExploits() {
    return this.client.get('/api/exploits');
  }

  async getExploit(id: string) {
    return this.client.get(`/api/exploits/${id}`);
  }

  async runExploit(params: {
    exploit_id?: string;
    branding_name?: string;
    target: string;
    options?: any;
  }) {
    return this.client.post('/api/exploits/run', params);
  }

  // Scanner operations
  async startScan(params: {
    target: string;
    scan_type?: string;
    threads?: number;
  }) {
    return this.client.post('/api/scan', params);
  }

  async getScanResults(scanId: string) {
    return this.client.get(`/api/scans/${scanId}`);
  }

  // Hashing operations
  async hash(params: {
    data: string;
    algorithm?: string;
    all_algorithms?: boolean;
  }) {
    return this.client.post('/api/hash', params);
  }

  // QKD operations
  async qkdEncrypt(params: {
    data: string;
    key_size?: number;
  }) {
    return this.client.post('/api/qkd/encrypt', params);
  }

  async qkdDecrypt(params: {
    ciphertext: string;
    session_id: string;
  }) {
    return this.client.post('/api/qkd/decrypt', params);
  }

  // Stats
  async getStats() {
    return this.client.get('/api/stats');
  }
}

export const apiClient = new PyRouterSploitClient();
