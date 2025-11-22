<script lang="ts">
  import { onMount } from 'svelte';
  import {
    Header,
    HeaderUtilities,
    HeaderAction,
    Content,
    Grid,
    Row,
    Column,
    Tile,
    Button
  } from 'carbon-components-svelte';
  import { Security, Tool, DataVis_3, Chemistry } from 'carbon-icons-svelte';

  let stats = {
    exploits: 0,
    scans: 0,
    cryptexEntries: 0,
    qkdSessions: 0
  };

  onMount(async () => {
    // Fetch stats from API
    try {
      const response = await fetch('http://localhost:8080/api/stats');
      if (response.ok) {
        stats = await response.json();
      }
    } catch (error) {
      console.error('Failed to load stats:', error);
    }
  });
</script>

<Header company="PyRouterSploit" platformName="Security Framework">
  <HeaderUtilities>
    <HeaderAction icon={Security} text="Exploits" href="/exploits" />
    <HeaderAction icon={Tool} text="Scanners" href="/scanners" />
    <HeaderAction icon={DataVis_3} text="Cryptex" href="/cryptex" />
    <HeaderAction icon={Chemistry} text="QKD Crypto" href="/qkd" />
  </HeaderUtilities>
</Header>

<Content>
  <Grid>
    <Row>
      <Column>
        <h1>PyRouterSploit Dashboard</h1>
        <p class="subtitle">Cross-Platform Security Exploitation Framework</p>
      </Column>
    </Row>

    <Row>
      <Column lg={4} md={4} sm={4}>
        <Tile>
          <div class="stat-card">
            <Security size={32} />
            <h3>{stats.exploits}</h3>
            <p>Available Exploits</p>
            <Button href="/exploits">Browse →</Button>
          </div>
        </Tile>
      </Column>

      <Column lg={4} md={4} sm={4}>
        <Tile>
          <div class="stat-card">
            <Tool size={32} />
            <h3>{stats.scans}</h3>
            <p>Completed Scans</p>
            <Button href="/scanners">Scan Now →</Button>
          </div>
        </Tile>
      </Column>

      <Column lg={4} md={4} sm={4}>
        <Tile>
          <div class="stat-card">
            <DataVis_3 size={32} />
            <h3>{stats.cryptexEntries}</h3>
            <p>Cryptex Entries</p>
            <Button href="/cryptex">Explore →</Button>
          </div>
        </Tile>
      </Column>
    </Row>

    <Row>
      <Column lg={4} md={4} sm={4}>
        <Tile>
          <div class="stat-card">
            <Chemistry size={32} />
            <h3>{stats.qkdSessions}</h3>
            <p>QKD Encryption Sessions</p>
            <Button href="/qkd">Encrypt →</Button>
          </div>
        </Tile>
      </Column>
    </Row>

    <Row>
      <Column>
        <Tile class="feature-tile">
          <h2>🚀 Features</h2>
          <ul>
            <li><strong>Rust Core:</strong> High-performance, memory-safe bedrock</li>
            <li><strong>redb Database:</strong> Embedded, ACID-compliant storage</li>
            <li><strong>Node-RED Integration:</strong> Visual workflow automation</li>
            <li><strong>QKD Encryption:</strong> Quantum Key Distribution with post-quantum crypto</li>
            <li><strong>Cryptex Dictionary:</strong> Function-to-branding name mapping</li>
            <li><strong>MCP Servers:</strong> LLM-accessible programmatic interface</li>
            <li><strong>Multi-Algorithm Hashing:</strong> SHA-2/3, BLAKE2/3, and more</li>
            <li><strong>Cross-Platform:</strong> Works anywhere - Linux, macOS, Windows</li>
          </ul>
        </Tile>
      </Column>
    </Row>
  </Grid>
</Content>

<style>
  .subtitle {
    font-size: 1.2rem;
    color: #666;
    margin-bottom: 2rem;
  }

  .stat-card {
    text-align: center;
    padding: 1rem;
  }

  .stat-card h3 {
    font-size: 2.5rem;
    margin: 1rem 0;
  }

  .stat-card p {
    color: #666;
    margin-bottom: 1rem;
  }

  :global(.feature-tile) {
    padding: 2rem;
  }

  :global(.feature-tile h2) {
    margin-bottom: 1rem;
  }

  :global(.feature-tile ul) {
    list-style-type: none;
    padding-left: 0;
  }

  :global(.feature-tile li) {
    padding: 0.5rem 0;
    border-bottom: 1px solid #e0e0e0;
  }

  :global(.feature-tile li:last-child) {
    border-bottom: none;
  }
</style>
