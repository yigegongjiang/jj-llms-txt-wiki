> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Hubungkan ke alat eksternal dengan MCP

> Konfigurasi server MCP untuk memperluas agen Anda dengan alat eksternal. Mencakup jenis transport, pencarian alat untuk set alat besar, autentikasi, dan penanganan kesalahan.

[Model Context Protocol (MCP)](https://modelcontextprotocol.io/docs/getting-started/intro) adalah standar terbuka untuk menghubungkan agen AI ke alat eksternal dan sumber data. Dengan MCP, agen Anda dapat menanyakan database, mengintegrasikan dengan API seperti Slack dan GitHub, dan terhubung ke layanan lain tanpa menulis implementasi alat khusus.

Server MCP dapat berjalan sebagai proses lokal, terhubung melalui HTTP, atau dieksekusi langsung dalam aplikasi SDK Anda.

<Note>
  Halaman ini mencakup konfigurasi MCP untuk Agent SDK. Untuk menambahkan server MCP ke Claude Code CLI sehingga dimuat di setiap proyek, lihat [Cakupan instalasi MCP](/docs/id/mcp#mcp-installation-scopes).
</Note>

<h2 id="quickstart">
  Quickstart
</h2>

Contoh ini terhubung ke server MCP [dokumentasi Claude Code](https://code.claude.com/docs) menggunakan [transport HTTP](#http%2Fsse-servers) dan menggunakan [`allowedTools`](#allow-mcp-tools) dengan wildcard untuk mengizinkan semua alat dari server.

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Use the docs MCP server to explain what hooks are in Claude Code",
    options: {
      mcpServers: {
        "claude-code-docs": {
          type: "http",
          url: "https://code.claude.com/docs/mcp"
        }
      },
      allowedTools: ["mcp__claude-code-docs__*"]
    }
  })) {
    if (message.type === "result" && message.subtype === "success") {
      console.log(message.result);
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage


  async def main():
      options = ClaudeAgentOptions(
          mcp_servers={
              "claude-code-docs": {
                  "type": "http",
                  "url": "https://code.claude.com/docs/mcp",
              }
          },
          allowed_tools=["mcp__claude-code-docs__*"],
      )

      async for message in query(
          prompt="Use the docs MCP server to explain what hooks are in Claude Code",
          options=options,
      ):
          if isinstance(message, ResultMessage) and message.subtype == "success":
              print(message.result)


  asyncio.run(main())
  ```
</CodeGroup>

Agen terhubung ke server dokumentasi, mencari informasi tentang hooks, dan mengembalikan hasilnya.

<h2 id="add-an-mcp-server">
  Tambahkan server MCP
</h2>

Anda dapat mengonfigurasi server MCP dalam kode saat memanggil `query()`, atau dalam file `.mcp.json` yang dimuat melalui [`settingSources`](#from-a-config-file).

<h3 id="in-code">
  Dalam kode
</h3>

Teruskan server MCP langsung dalam opsi `mcpServers`:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "List files in my project",
    options: {
      mcpServers: {
        filesystem: {
          command: "npx",
          args: ["-y", "@modelcontextprotocol/server-filesystem", "/Users/me/projects"]
        }
      },
      allowedTools: ["mcp__filesystem__*"]
    }
  })) {
    if (message.type === "result" && message.subtype === "success") {
      console.log(message.result);
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage


  async def main():
      options = ClaudeAgentOptions(
          mcp_servers={
              "filesystem": {
                  "command": "npx",
                  "args": [
                      "-y",
                      "@modelcontextprotocol/server-filesystem",
                      "/Users/me/projects",
                  ],
              }
          },
          allowed_tools=["mcp__filesystem__*"],
      )

      async for message in query(prompt="List files in my project", options=options):
          if isinstance(message, ResultMessage) and message.subtype == "success":
              print(message.result)


  asyncio.run(main())
  ```
</CodeGroup>

<h3 id="from-a-config-file">
  Dari file konfigurasi
</h3>

Buat file `.mcp.json` di root proyek Anda. File ini diambil ketika sumber pengaturan `project` diaktifkan, yang merupakan default untuk opsi `query()`. Jika Anda menetapkan `settingSources` secara eksplisit, sertakan `"project"` agar file ini dimuat:

```json theme={null}
{
  "mcpServers": {
    "filesystem": {
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-filesystem", "/Users/me/projects"]
    }
  }
}
```

<h2 id="allow-mcp-tools">
  Izinkan alat MCP
</h2>

Alat MCP memerlukan izin eksplisit sebelum Claude dapat menggunakannya. Tanpa izin, Claude akan melihat bahwa alat tersedia tetapi tidak akan dapat memanggilnya.

<h3 id="tool-naming-convention">
  Konvensi penamaan alat
</h3>

Alat MCP mengikuti pola penamaan `mcp__<server-name>__<tool-name>`. Misalnya, server GitHub bernama `"github"` dengan alat `list_issues` menjadi `mcp__github__list_issues`.

<h3 id="auto-approve-with-allowedtools">
  Persetujuan otomatis dengan allowedTools
</h3>

Gunakan `allowedTools` untuk secara otomatis menyetujui alat MCP tertentu sehingga Claude dapat menggunakannya tanpa permintaan izin:

```typescript hidelines={1,-1} theme={null}
const _ = {
  options: {
    mcpServers: {
      // your servers
    },
    allowedTools: [
      "mcp__github__*", // All tools from the github server
      "mcp__db__query", // Only the query tool from db server
      "mcp__slack__send_message" // Only send_message from slack server
    ]
  }
};
```

Wildcard (`*`) memungkinkan Anda mengizinkan semua alat dari server tanpa mencantumkan masing-masing secara individual.

<Note>
  **Lebih suka `allowedTools` daripada mode izin untuk akses MCP.** `permissionMode: "acceptEdits"` tidak secara otomatis menyetujui alat MCP (hanya edit file dan perintah Bash filesystem). `permissionMode: "bypassPermissions"` secara otomatis menyetujui alat MCP tetapi juga menonaktifkan sebagian besar prompt keamanan lainnya, yang lebih luas dari yang diperlukan; lihat [Bagaimana izin dievaluasi](/docs/id/agent-sdk/permissions#how-permissions-are-evaluated) untuk prompt yang tetap ada. Wildcard dalam `allowedTools` memberikan akses ke server MCP yang Anda inginkan dan tidak lebih. Lihat [Mode izin](/docs/id/agent-sdk/permissions#permission-modes) untuk perbandingan lengkap.
</Note>

<h3 id="discover-available-tools">
  Temukan alat yang tersedia
</h3>

Untuk melihat alat apa yang disediakan server MCP, periksa dokumentasi server atau terhubung ke server dan periksa pesan init `system`:

<CodeGroup>
  ```typescript TypeScript theme={null}
  for await (const message of query({ prompt: "...", options })) {
    if (message.type === "system" && message.subtype === "init") {
      console.log("Available MCP tools:", message.mcp_servers);
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, SystemMessage


  async def main():
      async for message in query(prompt="...", options=options):
          if isinstance(message, SystemMessage) and message.subtype == "init":
              print("Available MCP tools:", message.data["mcp_servers"])


  asyncio.run(main())
  ```
</CodeGroup>

<h2 id="transport-types">
  Jenis transport
</h2>

Server MCP berkomunikasi dengan agen Anda menggunakan protokol transport yang berbeda. Periksa dokumentasi server untuk melihat transport mana yang didukungnya:

* Jika dokumen memberi Anda **perintah untuk dijalankan** (seperti `npx @modelcontextprotocol/server-github`), gunakan stdio
* Jika dokumen memberi Anda **URL**, gunakan HTTP atau SSE
* Jika Anda membangun alat Anda sendiri dalam kode, gunakan server MCP SDK

<h3 id="stdio-servers">
  Server stdio
</h3>

Proses lokal yang berkomunikasi melalui stdin/stdout. Gunakan ini untuk server MCP yang Anda jalankan di mesin yang sama:

<Tabs>
  <Tab title="Dalam kode">
    <CodeGroup>
      ```typescript TypeScript hidelines={1,-1} theme={null}
      const _ = {
        options: {
          mcpServers: {
            github: {
              command: "npx",
              args: ["-y", "@modelcontextprotocol/server-github"],
              env: {
                GITHUB_TOKEN: process.env.GITHUB_TOKEN
              }
            }
          },
          allowedTools: ["mcp__github__list_issues", "mcp__github__search_issues"]
        }
      };
      ```

      ```python Python theme={null}
      options = ClaudeAgentOptions(
          mcp_servers={
              "github": {
                  "command": "npx",
                  "args": ["-y", "@modelcontextprotocol/server-github"],
                  "env": {"GITHUB_TOKEN": os.environ["GITHUB_TOKEN"]},
              }
          },
          allowed_tools=["mcp__github__list_issues", "mcp__github__search_issues"],
      )
      ```
    </CodeGroup>
  </Tab>

  <Tab title=".mcp.json">
    ```json theme={null}
    {
      "mcpServers": {
        "github": {
          "command": "npx",
          "args": ["-y", "@modelcontextprotocol/server-github"],
          "env": {
            "GITHUB_TOKEN": "${GITHUB_TOKEN}"
          }
        }
      }
    }
    ```
  </Tab>
</Tabs>

<h3 id="http/sse-servers">
  Server HTTP/SSE
</h3>

Gunakan HTTP atau SSE untuk server MCP yang dihosting cloud dan API jarak jauh:

<Tabs>
  <Tab title="Dalam kode">
    <CodeGroup>
      ```typescript TypeScript hidelines={1,-1} theme={null}
      const _ = {
        options: {
          mcpServers: {
            "remote-api": {
              type: "sse",
              url: "https://api.example.com/mcp/sse",
              headers: {
                Authorization: `Bearer ${process.env.API_TOKEN}`
              }
            }
          },
          allowedTools: ["mcp__remote-api__*"]
        }
      };
      ```

      ```python Python theme={null}
      options = ClaudeAgentOptions(
          mcp_servers={
              "remote-api": {
                  "type": "sse",
                  "url": "https://api.example.com/mcp/sse",
                  "headers": {"Authorization": f"Bearer {os.environ['API_TOKEN']}"},
              }
          },
          allowed_tools=["mcp__remote-api__*"],
      )
      ```
    </CodeGroup>
  </Tab>

  <Tab title=".mcp.json">
    ```json theme={null}
    {
      "mcpServers": {
        "remote-api": {
          "type": "sse",
          "url": "https://api.example.com/mcp/sse",
          "headers": {
            "Authorization": "Bearer ${API_TOKEN}"
          }
        }
      }
    }
    ```
  </Tab>
</Tabs>

Untuk transport HTTP yang dapat dialirkan, gunakan `"type": "http"` sebagai gantinya. Dalam file konfigurasi `.mcp.json` dan JSON lainnya, `"streamable-http"` diterima sebagai alias untuk `"http"`. Opsi `mcpServers` pemrograman hanya menerima `"http"`.

<h3 id="sdk-mcp-servers">
  Server MCP SDK
</h3>

Tentukan alat khusus langsung dalam kode aplikasi Anda daripada menjalankan proses server terpisah. Lihat [panduan alat khusus](/docs/id/agent-sdk/custom-tools) untuk detail implementasi.

<h2 id="mcp-tool-search">
  Pencarian alat MCP
</h2>

Ketika Anda memiliki banyak alat MCP yang dikonfigurasi, definisi alat dapat mengonsumsi bagian signifikan dari jendela konteks Anda. Pencarian alat mengatasi ini dengan menahan definisi alat dari konteks dan memuat hanya yang Claude butuhkan untuk setiap giliran.

Pencarian alat diaktifkan secara default. Lihat [Pencarian alat](/docs/id/agent-sdk/tool-search) untuk opsi konfigurasi dan detail.

Untuk detail lebih lanjut, termasuk praktik terbaik dan menggunakan pencarian alat dengan alat SDK khusus, lihat [panduan pencarian alat](/docs/id/agent-sdk/tool-search).

<h2 id="authentication">
  Autentikasi
</h2>

Sebagian besar server MCP memerlukan autentikasi untuk mengakses layanan eksternal. Teruskan kredensial melalui variabel lingkungan dalam konfigurasi server.

<h3 id="pass-credentials-via-environment-variables">
  Teruskan kredensial melalui variabel lingkungan
</h3>

Gunakan bidang `env` untuk meneruskan kunci API, token, dan kredensial lainnya ke server MCP:

<Tabs>
  <Tab title="Dalam kode">
    <CodeGroup>
      ```typescript TypeScript hidelines={1,-1} theme={null}
      const _ = {
        options: {
          mcpServers: {
            github: {
              command: "npx",
              args: ["-y", "@modelcontextprotocol/server-github"],
              env: {
                GITHUB_TOKEN: process.env.GITHUB_TOKEN
              }
            }
          },
          allowedTools: ["mcp__github__list_issues"]
        }
      };
      ```

      ```python Python theme={null}
      options = ClaudeAgentOptions(
          mcp_servers={
              "github": {
                  "command": "npx",
                  "args": ["-y", "@modelcontextprotocol/server-github"],
                  "env": {"GITHUB_TOKEN": os.environ["GITHUB_TOKEN"]},
              }
          },
          allowed_tools=["mcp__github__list_issues"],
      )
      ```
    </CodeGroup>
  </Tab>

  <Tab title=".mcp.json">
    ```json theme={null}
    {
      "mcpServers": {
        "github": {
          "command": "npx",
          "args": ["-y", "@modelcontextprotocol/server-github"],
          "env": {
            "GITHUB_TOKEN": "${GITHUB_TOKEN}"
          }
        }
      }
    }
    ```

    Sintaks `${GITHUB_TOKEN}` memperluas variabel lingkungan saat runtime.
  </Tab>
</Tabs>

Lihat [Daftar masalah dari repositori](#list-issues-from-a-repository) untuk contoh kerja lengkap dengan logging debug.

<h3 id="http-headers-for-remote-servers">
  Header HTTP untuk server jarak jauh
</h3>

Untuk server HTTP dan SSE, teruskan header autentikasi langsung dalam konfigurasi server:

<Tabs>
  <Tab title="Dalam kode">
    <CodeGroup>
      ```typescript TypeScript hidelines={1,-1} theme={null}
      const _ = {
        options: {
          mcpServers: {
            "secure-api": {
              type: "http",
              url: "https://api.example.com/mcp",
              headers: {
                Authorization: `Bearer ${process.env.API_TOKEN}`
              }
            }
          },
          allowedTools: ["mcp__secure-api__*"]
        }
      };
      ```

      ```python Python theme={null}
      options = ClaudeAgentOptions(
          mcp_servers={
              "secure-api": {
                  "type": "http",
                  "url": "https://api.example.com/mcp",
                  "headers": {"Authorization": f"Bearer {os.environ['API_TOKEN']}"},
              }
          },
          allowed_tools=["mcp__secure-api__*"],
      )
      ```
    </CodeGroup>
  </Tab>

  <Tab title=".mcp.json">
    ```json theme={null}
    {
      "mcpServers": {
        "secure-api": {
          "type": "http",
          "url": "https://api.example.com/mcp",
          "headers": {
            "Authorization": "Bearer ${API_TOKEN}"
          }
        }
      }
    }
    ```

    Sintaks `${API_TOKEN}` memperluas variabel lingkungan saat runtime.
  </Tab>
</Tabs>

<h3 id="oauth2-authentication">
  Autentikasi OAuth2
</h3>

[Spesifikasi MCP mendukung OAuth 2.1](https://modelcontextprotocol.io/specification/2025-03-26/basic/authorization) untuk otorisasi. SDK tidak membuka browser atau menjalankan alur OAuth interaktif. Ketika server yang dikonfigurasi mengembalikan tantangan otorisasi dan tidak ada token yang disimpan tersedia, jalankan agen berlanjut tanpa alat server tersebut, dan server dilaporkan dengan status `needs-auth` dalam array `mcp_servers` dari [pesan inisialisasi sistem](/docs/id/agent-sdk/typescript#sdksystemmessage). Periksa array tersebut saat startup jika agen Anda bergantung pada server tertentu yang terhubung.

Untuk menyediakan kredensial, selesaikan alur OAuth dalam aplikasi Anda sendiri dan teruskan token akses yang dihasilkan dalam `headers` server:

<CodeGroup>
  ```typescript TypeScript theme={null}
  // After completing OAuth flow in your app
  const accessToken = await getAccessTokenFromOAuthFlow();

  const options = {
    mcpServers: {
      "oauth-api": {
        type: "http",
        url: "https://api.example.com/mcp",
        headers: {
          Authorization: `Bearer ${accessToken}`
        }
      }
    },
    allowedTools: ["mcp__oauth-api__*"]
  };
  ```

  ```python Python theme={null}
  # After completing OAuth flow in your app
  access_token = await get_access_token_from_oauth_flow()

  options = ClaudeAgentOptions(
      mcp_servers={
          "oauth-api": {
              "type": "http",
              "url": "https://api.example.com/mcp",
              "headers": {"Authorization": f"Bearer {access_token}"},
          }
      },
      allowed_tools=["mcp__oauth-api__*"],
  )
  ```
</CodeGroup>

<h2 id="examples">
  Contoh
</h2>

<h3 id="list-issues-from-a-repository">
  Daftar masalah dari repositori
</h3>

Contoh ini terhubung ke [server MCP GitHub](https://github.com/modelcontextprotocol/servers/tree/main/src/github) untuk mencantumkan masalah terbaru. Contoh ini mencakup logging debug untuk memverifikasi koneksi MCP dan panggilan alat.

Sebelum menjalankan, buat [token akses pribadi GitHub](https://github.com/settings/tokens) dengan cakupan `repo` dan atur sebagai variabel lingkungan:

```bash theme={null}
export GITHUB_TOKEN=ghp_xxxxxxxxxxxxxxxxxxxx
```

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "List the 3 most recent issues in anthropics/claude-code",
    options: {
      mcpServers: {
        github: {
          command: "npx",
          args: ["-y", "@modelcontextprotocol/server-github"],
          env: {
            GITHUB_TOKEN: process.env.GITHUB_TOKEN
          }
        }
      },
      allowedTools: ["mcp__github__list_issues"]
    }
  })) {
    // Verify MCP server connected successfully
    if (message.type === "system" && message.subtype === "init") {
      console.log("MCP servers:", message.mcp_servers);
    }

    // Log when Claude calls an MCP tool
    if (message.type === "assistant") {
      for (const block of message.message.content) {
        if (block.type === "tool_use" && block.name.startsWith("mcp__")) {
          console.log("MCP tool called:", block.name);
        }
      }
    }

    // Print the final result
    if (message.type === "result" && message.subtype === "success") {
      console.log(message.result);
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  import os
  from claude_agent_sdk import (
      query,
      ClaudeAgentOptions,
      ResultMessage,
      SystemMessage,
      AssistantMessage,
  )


  async def main():
      options = ClaudeAgentOptions(
          mcp_servers={
              "github": {
                  "command": "npx",
                  "args": ["-y", "@modelcontextprotocol/server-github"],
                  "env": {"GITHUB_TOKEN": os.environ["GITHUB_TOKEN"]},
              }
          },
          allowed_tools=["mcp__github__list_issues"],
      )

      async for message in query(
          prompt="List the 3 most recent issues in anthropics/claude-code",
          options=options,
      ):
          # Verify MCP server connected successfully
          if isinstance(message, SystemMessage) and message.subtype == "init":
              print("MCP servers:", message.data.get("mcp_servers"))

          # Log when Claude calls an MCP tool
          if isinstance(message, AssistantMessage):
              for block in message.content:
                  if hasattr(block, "name") and block.name.startswith("mcp__"):
                      print("MCP tool called:", block.name)

          # Print the final result
          if isinstance(message, ResultMessage) and message.subtype == "success":
              print(message.result)


  asyncio.run(main())
  ```
</CodeGroup>

<h3 id="query-a-database">
  Tanyakan database
</h3>

Contoh ini menggunakan [server MCP Postgres](https://github.com/modelcontextprotocol/servers/tree/main/src/postgres) untuk menanyakan database. String koneksi diteruskan sebagai argumen ke server. Agen secara otomatis menemukan skema database, menulis kueri SQL, dan mengembalikan hasilnya:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // Connection string from environment variable
  const connectionString = process.env.DATABASE_URL;

  for await (const message of query({
    // Natural language query - Claude writes the SQL
    prompt: "How many users signed up last week? Break it down by day.",
    options: {
      mcpServers: {
        postgres: {
          command: "npx",
          // Pass connection string as argument to the server
          args: ["-y", "@modelcontextprotocol/server-postgres", connectionString]
        }
      },
      // Allow only read queries, not writes
      allowedTools: ["mcp__postgres__query"]
    }
  })) {
    if (message.type === "result" && message.subtype === "success") {
      console.log(message.result);
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  import os
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage


  async def main():
      # Connection string from environment variable
      connection_string = os.environ["DATABASE_URL"]

      options = ClaudeAgentOptions(
          mcp_servers={
              "postgres": {
                  "command": "npx",
                  # Pass connection string as argument to the server
                  "args": [
                      "-y",
                      "@modelcontextprotocol/server-postgres",
                      connection_string,
                  ],
              }
          },
          # Allow only read queries, not writes
          allowed_tools=["mcp__postgres__query"],
      )

      # Natural language query - Claude writes the SQL
      async for message in query(
          prompt="How many users signed up last week? Break it down by day.",
          options=options,
      ):
          if isinstance(message, ResultMessage) and message.subtype == "success":
              print(message.result)


  asyncio.run(main())
  ```
</CodeGroup>

<h2 id="error-handling">
  Penanganan kesalahan
</h2>

Server MCP dapat gagal terhubung karena berbagai alasan: proses server mungkin tidak terinstal, kredensial mungkin tidak valid, atau server jarak jauh mungkin tidak dapat dijangkau.

SDK mengirimkan pesan `system` dengan subtype `init` di awal setiap kueri. Pesan ini mencakup status koneksi untuk setiap server MCP. Periksa bidang `status` untuk mendeteksi kegagalan koneksi sebelum agen mulai bekerja:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Process data",
    options: {
      mcpServers: {
        "data-processor": dataServer
      }
    }
  })) {
    if (message.type === "system" && message.subtype === "init") {
      const failedServers = message.mcp_servers.filter((s) => s.status !== "connected");

      if (failedServers.length > 0) {
        console.warn("Failed to connect:", failedServers);
      }
    }

    if (message.type === "result" && message.subtype === "error_during_execution") {
      console.error("Execution failed");
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, SystemMessage, ResultMessage


  async def main():
      options = ClaudeAgentOptions(mcp_servers={"data-processor": data_server})

      async for message in query(prompt="Process data", options=options):
          if isinstance(message, SystemMessage) and message.subtype == "init":
              failed_servers = [
                  s
                  for s in message.data.get("mcp_servers", [])
                  if s.get("status") != "connected"
              ]

              if failed_servers:
                  print(f"Failed to connect: {failed_servers}")

          if (
              isinstance(message, ResultMessage)
              and message.subtype == "error_during_execution"
          ):
              print("Execution failed")


  asyncio.run(main())
  ```
</CodeGroup>

<h2 id="troubleshooting">
  Troubleshooting
</h2>

<h3 id="server-shows-failed-status">
  Server menunjukkan status "failed"
</h3>

Periksa pesan `init` untuk melihat server mana yang gagal terhubung:

```typescript theme={null}
if (message.type === "system" && message.subtype === "init") {
  for (const server of message.mcp_servers) {
    if (server.status === "failed") {
      console.error(`Server ${server.name} failed to connect`);
    }
  }
}
```

Penyebab umum:

* **Variabel lingkungan yang hilang**: Pastikan token dan kredensial yang diperlukan diatur. Untuk server stdio, periksa bidang `env` cocok dengan apa yang diharapkan server.
* **Server tidak terinstal**: Untuk perintah `npx`, verifikasi paket ada dan Node.js ada di PATH Anda.
* **String koneksi tidak valid**: Untuk server database, verifikasi format string koneksi dan bahwa database dapat diakses.
* **Masalah jaringan**: Untuk server HTTP/SSE jarak jauh, periksa URL dapat dijangkau dan firewall apa pun memungkinkan koneksi.

<h3 id="tools-not-being-called">
  Alat tidak dipanggil
</h3>

Jika Claude melihat alat tetapi tidak menggunakannya, periksa bahwa Anda telah memberikan izin dengan `allowedTools`:

```typescript hidelines={1,-1} theme={null}
const _ = {
  options: {
    mcpServers: {
      // your servers
    },
    allowedTools: ["mcp__servername__*"] // Auto-approve calls from this server
  }
};
```

<h3 id="connection-timeouts">
  Timeout koneksi
</h3>

Koneksi server MCP mengalami timeout setelah 30 detik secara default. Jika server Anda membutuhkan waktu lebih lama untuk memulai, koneksi akan gagal. Naikkan batasnya dengan variabel lingkungan [`MCP_TIMEOUT`](/docs/id/env-vars), dalam milidetik. Untuk server yang memerlukan waktu startup lebih lama, pertimbangkan juga:

* Menggunakan server yang lebih ringan jika tersedia
* Pre-warming server sebelum memulai agen Anda
* Memeriksa log server untuk penyebab inisialisasi lambat

<h3 id="tool-output-exceeds-maximum-allowed-tokens">
  Output alat melebihi token maksimal yang diizinkan
</h3>

SDK menerapkan batas output MCP yang sama dengan Claude Code. Ketika hasil alat lebih besar dari 25.000 token, output lengkap disimpan ke file dan hasil alat diganti dengan pesan kesalahan yang menyebutkan jalur file, sehingga agen dapat membaca output kembali dalam porsi. Naikkan batasnya dengan variabel lingkungan [`MAX_MCP_OUTPUT_TOKENS`](/docs/id/env-vars). Lihat [MCP output limits and warnings](/docs/id/mcp#mcp-output-limits-and-warnings) untuk perilaku lengkap, termasuk bagaimana server dapat mendeklarasikan batas per-alat yang lebih tinggi.

<h2 id="related-resources">
  Sumber daya terkait
</h2>

* **[Panduan alat khusus](/docs/id/agent-sdk/custom-tools)**: Bangun server MCP Anda sendiri yang berjalan in-process dengan aplikasi SDK Anda
* **[Izin](/docs/id/agent-sdk/permissions)**: Kontrol alat MCP mana yang dapat digunakan agen Anda dengan `allowedTools` dan `disallowedTools`
* **[Batas output MCP dan peringatan](/docs/id/mcp#mcp-output-limits-and-warnings)**: Bagaimana SDK menangani hasil alat yang melebihi `MAX_MCP_OUTPUT_TOKENS`, termasuk fallback persist-to-disk dan anotasi per-alat `anthropic/maxResultSizeChars`
* **[Referensi SDK TypeScript](/docs/id/agent-sdk/typescript)**: Referensi API lengkap termasuk opsi konfigurasi MCP
* **[Referensi SDK Python](/docs/id/agent-sdk/python)**: Referensi API lengkap termasuk opsi konfigurasi MCP
* **[Direktori server MCP](https://github.com/modelcontextprotocol/servers)**: Jelajahi server MCP yang tersedia untuk database, API, dan lainnya
