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

Teruskan server MCP secara langsung dalam opsi `mcpServers`. Contoh ini memulai server MCP filesystem lokal untuk `/Users/me/projects`. Ganti jalur tersebut dengan direktori di mesin Anda:

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

Buat file `.mcp.json` di root proyek Anda. File ini diambil ketika sumber pengaturan `project` diaktifkan, yang merupakan default untuk opsi `query()`. Jika Anda menetapkan `settingSources` secara eksplisit, sertakan `"project"` agar file ini dimuat. Ganti `/Users/me/projects` dengan direktori di mesin Anda:

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

<h2 id="connection-timing">
  Waktu koneksi
</h2>

Claude Code mendaftarkan server yang Anda berikan dalam `options.mcpServers` saat startup dan mengirimkan [pesan init](#error-handling) setelah penundaan putaran pertama, jika ada, terselesaikan. Apakah setiap server `options.mcpServers` menunda putaran pertama, dan kapan server tersebut terhubung, tergantung pada jenisnya:

| Jenis server                                                                                          | Menunda putaran pertama?                                | Batas waktu tunggu putaran pertama                                                                              |
| :---------------------------------------------------------------------------------------------------- | :------------------------------------------------------ | :-------------------------------------------------------------------------------------------------------------- |
| Server stdio, atau server HTTP/SSE tanpa daftar alat yang di-cache                                    | Ya, sampai terhubung                                    | [`MCP_TIMEOUT`](/docs/id/env-vars), 30 detik secara default; koneksi gagal pada batas waktu tersebut                 |
| Server jarak jauh dengan daftar alat yang di-cache, disimpan oleh Claude Code dari koneksi sebelumnya | Tidak; alat yang di-cache tersedia dari putaran pertama | Tidak ada; terhubung pada panggilan alat pertamanya, dan koneksi tertunda tersebut memiliki batas waktu sendiri |
| Server [SDK](#sdk-mcp-servers) dalam proses                                                           | Ya, sampai terhubung dan mencantumkan alatnya           | Tidak ada; permintaan koneksi dan pencantuman alat masing-masing memiliki batas waktu sendiri                   |

Server yang dimuat dari [file pengaturan](#from-a-config-file) seperti `.mcp.json` atau dari plugin biasanya menunjukkan `pending` dalam pesan init. Ketika `options.mcpServers` menyimpan server stdio, HTTP, atau SSE, putaran pertama menunggu server yang tertunda ini juga, hingga `MCP_TIMEOUT`. Ketika `options.mcpServers` kosong atau hanya menyimpan server SDK, putaran pertama menunggu hingga 2 detik sebagai gantinya:

* **Dengan [pencarian alat](/docs/id/agent-sdk/tool-search), default**: penundaan mencakup server yang masih tertunda yang dikonfigurasi dengan [`alwaysLoad: true`](/docs/id/mcp#exempt-a-server-from-deferral) dan bukan sisanya. Sisanya terus terhubung di latar belakang. [Ketersediaan alat](/docs/id/mcp#tool-availability) menjelaskan bagaimana Claude mencapai alat mereka setelah terhubung.
* **Tanpa pencarian alat**: penundaan mencakup setiap server yang tertunda. [Konfigurasi pencarian alat](/docs/id/agent-sdk/tool-search#configure-tool-search) mencakup apa yang mematikan pencarian alat. Jika Anda mengecualikan alat `ToolSearch` dari sesi, misalnya melalui `disallowedTools`, sesi juga berjalan tanpa pencarian alat.

Jika Anda menetapkan `permissionPromptToolName`, putaran pertama juga menunggu server alat tersebut dalam setiap kasus, hingga `MCP_TIMEOUT`.

Untuk menetapkan penundaan putaran pertama sendiri, tambahkan `CLAUDE_CODE_MCP_STARTUP_WAIT_MS` ke [opsi `env`](/docs/id/agent-sdk/configuration#set-environment-variables), misalnya `CLAUDE_CODE_MCP_STARTUP_WAIT_MS: "5000"`. Putaran pertama kemudian menunggu hingga banyak milidetik untuk setiap server yang tertunda, terlepas dari apakah pencarian alat tersedia. Batas waktu ini juga menggantikan penundaan putaran pertama `MCP_TIMEOUT` untuk server stdio, HTTP, dan SSE dalam `options.mcpServers`. `CLAUDE_CODE_MCP_STARTUP_WAIT_MS` memerlukan Claude Code v2.1.274 atau lebih baru.

Server yang masih tertunda ketika penundaan berakhir terus terhubung di latar belakang. Atur variabel ke `0` untuk melewati penundaan. Server `permissionPromptToolName` mempertahankan penundaan `MCP_TIMEOUT` sendiri terlepas dari nilainya.

Untuk memblokir startup itu sendiri pada fase terpisah yang lebih awal daripada penundaan putaran pertama, sebelum pesan init dikirim:

* Atur [`MCP_CONNECTION_NONBLOCKING`](/docs/id/env-vars) ke `0` untuk memblokir seluruh batch koneksi. Claude Code membatasi penundaan tersebut pada 5 detik secara default. Sesuaikan batas dengan variabel lingkungan [`MCP_CONNECT_TIMEOUT_MS`](/docs/id/env-vars), dalam milidetik. Server yang masih tertunda pada batas waktu tersebut terus terhubung di latar belakang.
* Atur `alwaysLoad: true` pada konfigurasi server untuk membuat alatnya tersedia pada skema lengkap mereka pada putaran pertama, [dikecualikan dari penundaan pencarian alat](/docs/id/mcp#exempt-a-server-from-deferral). Claude Code menunggu saat startup untuk alat server tersebut, dibatasi pada batas waktu yang sama, sementara server lain terus terhubung di latar belakang; server jarak jauh dengan daftar alat yang di-cache menyediakannya tanpa terhubung, sesuai tabel di atas.

Pesan `system` dengan subtipe `init` melaporkan status setiap server pada saat pesan tersebut dikirim; lihat [Penanganan kesalahan](#error-handling) untuk membaca status tersebut.

<h2 id="allow-mcp-tools">
  Izinkan alat MCP
</h2>

Alat MCP memerlukan izin eksplisit sebelum Claude dapat menggunakannya. Tanpa izin, Claude akan melihat bahwa alat tersedia tetapi tidak akan dapat memanggilnya.

<h3 id="tool-naming-convention">
  Konvensi penamaan alat
</h3>

Alat MCP mengikuti pola penamaan `mcp__<server-name>__<tool-name>`. Misalnya, server GitHub bernama `"github"` dengan alat `list_issues` menjadi `mcp__github__list_issues`.

<h3 id="auto-approve-with-allowedtools">
  Auto-approve dengan allowedTools
</h3>

Gunakan `allowedTools` untuk auto-approve alat MCP tertentu sehingga Claude dapat menggunakannya tanpa prompt izin:

<CodeGroup>
  ```typescript TypeScript hidelines={1,-1} theme={null}
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

  ```python Python theme={null}
  options = ClaudeAgentOptions(
      mcp_servers={
          # your servers
      },
      allowed_tools=[
          "mcp__github__*",  # All tools from the github server
          "mcp__db__query",  # Only the query tool from db server
          "mcp__slack__send_message",  # Only send_message from slack server
      ],
  )
  ```
</CodeGroup>

Wildcard (`*`) memungkinkan Anda untuk mengizinkan semua alat dari server tanpa mencantumkan masing-masing secara individual.

<Note>
  **Lebih suka `allowedTools` daripada mode izin untuk akses MCP.** `permissionMode: "acceptEdits"` tidak auto-approve alat MCP (hanya edit file dan perintah Bash filesystem). `permissionMode: "bypassPermissions"` melakukan auto-approve alat MCP tetapi juga menonaktifkan sebagian besar prompt keamanan lainnya, yang lebih luas dari yang diperlukan; lihat [Bagaimana izin dievaluasi](/docs/id/agent-sdk/permissions#how-permissions-are-evaluated) untuk prompt yang tetap ada. Wildcard dalam `allowedTools` memberikan akses ke server MCP yang Anda inginkan dan tidak lebih. Lihat [Mode izin](/docs/id/agent-sdk/permissions#permission-modes) untuk perbandingan lengkap.
</Note>

<h3 id="discover-available-tools">
  Temukan alat yang tersedia
</h3>

Untuk melihat alat apa yang disediakan server MCP, periksa dokumentasi server atau inspeksi array `tools` dalam pesan init `system`. Nama alat MCP dimulai dengan `mcp__`.

Claude Code memancarkan pesan init setelah [penundaan koneksi giliran pertama](#connection-timing) untuk server yang dilewatkan dalam `options.mcpServers`, jadi array `tools` mencantumkan alat `mcp__` dari setiap server yang telah terhubung pada saat itu, ditambah alat dari server dengan [daftar alat yang di-cache](#connection-timing), yang terhubung pada penggunaan pertama. Alat dari server lain yang belum terhubung tidak ada; lihat [Penanganan kesalahan](#error-handling) untuk membaca status setiap server.

Filter ini mencetak nama alat MCP:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const options = {
    mcpServers: {
      // your servers
    },
  };

  for await (const message of query({ prompt: "...", options })) {
    if (message.type === "system" && message.subtype === "init") {
      const mcpTools = message.tools.filter((name) => name.startsWith("mcp__"));
      console.log("Available MCP tools:", mcpTools);
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, SystemMessage


  async def main():
      options = ClaudeAgentOptions(
          mcp_servers={
              # your servers
          },
      )
      async for message in query(prompt="...", options=options):
          if isinstance(message, SystemMessage) and message.subtype == "init":
              mcp_tools = [t for t in message.data.get("tools", []) if t.startswith("mcp__")]
              print("Available MCP tools:", mcp_tools)


  asyncio.run(main())
  ```
</CodeGroup>

Anda juga dapat meminta Claude untuk mencantumkan alat yang tersedia dari server.

<h2 id="transport-types">
  Jenis transport
</h2>

Server MCP berkomunikasi dengan agen Anda menggunakan protokol transport yang berbeda. Periksa dokumentasi server untuk melihat transport mana yang didukungnya:

* Jika dokumen memberi Anda **perintah untuk dijalankan** (seperti `npx @modelcontextprotocol/server-filesystem`), gunakan stdio
* Jika dokumen memberi Anda **URL**, gunakan HTTP atau SSE
* Jika Anda membangun alat Anda sendiri dalam kode, gunakan server MCP SDK

<h3 id="stdio-servers">
  Server stdio
</h3>

Proses lokal yang berkomunikasi melalui stdin/stdout. Gunakan ini untuk server MCP yang Anda jalankan di mesin yang sama. Untuk bentuk `.mcp.json`, gunakan bidang yang sama seperti yang ditunjukkan di [Dari file konfigurasi](#from-a-config-file). Dalam kode, teruskan perintah dan argumennya. Ganti `/Users/me/projects` dengan direktori di mesin Anda:

<CodeGroup>
  ```typescript TypeScript hidelines={1,-1} theme={null}
  const _ = {
    options: {
      mcpServers: {
        filesystem: {
          command: "npx",
          args: ["-y", "@modelcontextprotocol/server-filesystem", "/Users/me/projects"]
        }
      },
      allowedTools: ["mcp__filesystem__read_file", "mcp__filesystem__list_directory"]
    }
  };
  ```

  ```python Python theme={null}
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
      allowed_tools=["mcp__filesystem__read_file", "mcp__filesystem__list_directory"],
  )
  ```
</CodeGroup>

<h3 id="http/sse-servers">
  Server HTTP/SSE
</h3>

Gunakan HTTP atau SSE untuk server MCP yang dihosting di cloud dan API jarak jauh. Untuk bentuk `.mcp.json`, gunakan bidang yang sama seperti contoh di [Header HTTP untuk server jarak jauh](#http-headers-for-remote-servers), dengan `"type": "sse"` untuk server SSE. Dalam kode, teruskan URL server:

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

Untuk transport HTTP yang dapat dialirkan, gunakan `"type": "http"` sebagai gantinya. Dalam file konfigurasi `.mcp.json` dan JSON lainnya, `"streamable-http"` diterima sebagai alias untuk `"http"`. Tipe `McpHttpServerConfig` SDK hanya mendeklarasikan `"http"`, jadi gunakan `"http"` untuk server yang Anda teruskan dalam kode.

<h3 id="sdk-mcp-servers">
  Server MCP SDK
</h3>

Tentukan alat khusus langsung dalam kode aplikasi Anda alih-alih menjalankan proses server terpisah. Lihat [panduan alat khusus](/docs/id/agent-sdk/custom-tools) untuk detail implementasi.

Server MCP SDK yang didaftarkan oleh [permintaan kontrol `initialize`](/docs/id/agent-sdk/typescript#sdkcontrolinitializeresponse) mulai terhubung segera setelah Claude Code memproses permintaan.

<h2 id="mcp-tool-search">
  Pencarian tool MCP
</h2>

Ketika Anda memiliki banyak tool MCP yang dikonfigurasi, definisi tool dapat mengonsumsi sebagian signifikan dari jendela konteks Anda. Pencarian tool mengatasi ini dengan menahan definisi tool dari konteks dan memuat hanya yang Claude butuhkan untuk setiap giliran.

Pencarian tool diaktifkan secara default. Lihat [Pencarian tool](/docs/id/agent-sdk/tool-search) untuk opsi konfigurasi, praktik terbaik, dan menggunakan pencarian tool dengan tool SDK kustom.

<h2 id="authentication">
  Autentikasi
</h2>

Sebagian besar server MCP memerlukan autentikasi untuk mengakses layanan eksternal. Teruskan kredensial melalui variabel lingkungan dalam konfigurasi server.

<h3 id="pass-credentials-via-environment-variables">
  Teruskan kredensial melalui variabel lingkungan
</h3>

Gunakan field `env` untuk meneruskan kunci API, token, dan kredensial lainnya ke server MCP:

<Tabs>
  <Tab title="Dalam kode">
    <CodeGroup>
      ```typescript TypeScript hidelines={1,-1} theme={null}
      const _ = {
        options: {
          mcpServers: {
            "api-server": {
              command: "npx",
              args: ["-y", "@your-org/api-mcp-server"],
              env: {
                API_KEY: process.env.API_KEY
              }
            }
          },
          allowedTools: ["mcp__api-server__*"]
        }
      };
      ```

      ```python Python theme={null}
      options = ClaudeAgentOptions(
          mcp_servers={
              "api-server": {
                  "command": "npx",
                  "args": ["-y", "@your-org/api-mcp-server"],
                  "env": {"API_KEY": os.environ["API_KEY"]},
              }
          },
          allowed_tools=["mcp__api-server__*"],
      )
      ```
    </CodeGroup>
  </Tab>

  <Tab title=".mcp.json">
    ```json theme={null}
    {
      "mcpServers": {
        "api-server": {
          "command": "npx",
          "args": ["-y", "@your-org/api-mcp-server"],
          "env": {
            "API_KEY": "${API_KEY}"
          }
        }
      }
    }
    ```

    Sintaks `${API_KEY}` memperluas variabel lingkungan saat runtime.
  </Tab>
</Tabs>

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

Untuk contoh kerja lengkap dari server jarak jauh yang diautentikasi dengan header, lihat [Daftar masalah dari repositori](#list-issues-from-a-repository).

<h3 id="oauth2-authentication">
  Autentikasi OAuth2
</h3>

[Spesifikasi MCP mendukung OAuth 2.1](https://modelcontextprotocol.io/specification/2025-03-26/basic/authorization) untuk otorisasi. SDK tidak membuka browser atau menjalankan alur OAuth interaktif. Ketika server yang dikonfigurasi mengembalikan tantangan otorisasi dan tidak ada token yang disimpan tersedia, jalankan agen berlanjut tanpa alat server tersebut, dan server melaporkan status `needs-auth`. Array `mcp_servers` dari [pesan inisialisasi sistem](/docs/id/agent-sdk/typescript#sdksystemmessage) mungkin masih menunjukkan `pending` untuk server tersebut saat dipancarkan. Untuk mengonfirmasi apakah server memerlukan kredensial, polling `mcpServerStatus()` dalam SDK TypeScript atau [`get_mcp_status()`](/docs/id/agent-sdk/python#methods) dalam Python.

Untuk menyediakan kredensial, selesaikan alur OAuth dalam aplikasi Anda sendiri dan teruskan token akses yang dihasilkan dalam `headers` server:

<CodeGroup>
  ```typescript TypeScript theme={null}
  // Setelah menyelesaikan alur OAuth dalam aplikasi Anda.
  // Implementasikan getAccessTokenFromOAuthFlow untuk penyedia OAuth Anda.
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
  # Setelah menyelesaikan alur OAuth dalam aplikasi Anda.
  # Implementasikan get_access_token_from_oauth_flow untuk penyedia OAuth Anda.
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

Contoh ini terhubung ke [server MCP GitHub](https://github.com/github/github-mcp-server) jarak jauh untuk mencantumkan masalah terbaru. Contoh ini mencakup logging debug untuk memverifikasi koneksi MCP dan panggilan alat.

Sebelum menjalankan, buat [token akses pribadi GitHub](https://github.com/settings/personal-access-tokens) dengan akses baca ke repositori yang ingin Anda kueri dan atur sebagai variabel lingkungan:

```bash theme={null}
export GITHUB_TOKEN=YOUR_GITHUB_PAT
```

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "List the 3 most recent issues in anthropics/claude-code",
    options: {
      mcpServers: {
        github: {
          type: "http",
          url: "https://api.githubcopilot.com/mcp/",
          headers: {
            Authorization: `Bearer ${process.env.GITHUB_TOKEN}`
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
                  "type": "http",
                  "url": "https://api.githubcopilot.com/mcp/",
                  "headers": {"Authorization": f"Bearer {os.environ['GITHUB_TOKEN']}"},
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

Pada baris `MCP servers:`, `status` sebesar `connected` untuk `github` mengkonfirmasi token berfungsi. Jika Claude Code memiliki [daftar alat yang di-cache](#connection-timing) untuk server, status dapat membaca `pending` sebagai gantinya dan server terhubung pada panggilan alat pertamanya. Jika statusnya adalah `failed` atau `needs-auth`, lihat [Penanganan kesalahan](#error-handling) sebelum mempercayai hasilnya, karena Claude dapat kembali ke alat bawaan ketika server tidak tersedia.

<h3 id="query-a-database">
  Kueri basis data
</h3>

Contoh ini menggunakan [DBHub](https://github.com/bytebase/dbhub) untuk mengueri basis data Postgres. Agen secara otomatis menemukan skema basis data, menulis kueri SQL, dan mengembalikan hasilnya.

Alat `execute_sql` DBHub menjalankan SQL apa pun yang dikeluarkan agen, termasuk penulisan, kecuali Anda membatasinya. Mengatur `readonly = true` dalam [file konfigurasi DBHub](https://dbhub.ai/config/toml) membuat DBHub menolak pernyataan `INSERT`, `UPDATE`, `DELETE`, dan DDL, sehingga contoh tidak dapat memodifikasi data Anda bahkan jika agen mengeluarkan penulisan. DBHub menyelesaikan `${DATABASE_URL}` dari lingkungan proses ketika memuat konfigurasi, sehingga string koneksi tetap keluar dari file. Buat `dbhub.toml` ini di sebelah skrip Anda:

```toml dbhub.toml theme={null}
[[sources]]
id = "production"
dsn = "${DATABASE_URL}"

[[tools]]
name = "execute_sql"
source = "production"
readonly = true
```

Skrip kemudian menunjukkan DBHub ke file konfigurasi alih-alih melewatkan string koneksi secara langsung. Sebelum menjalankan, atur variabel lingkungan `DATABASE_URL` ke string koneksi Anda. Ganti nilai placeholder dengan detail basis data Anda sendiri:

```bash theme={null}
export DATABASE_URL=postgresql://user:password@localhost:5432/mydb
```

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    // Natural language query - Claude writes the SQL
    prompt: "How many users signed up last week? Break it down by day.",
    options: {
      mcpServers: {
        postgres: {
          command: "npx",
          // dbhub.toml sets readonly = true, so execute_sql rejects writes
          args: ["-y", "@bytebase/dbhub", "--config", "dbhub.toml"]
        }
      },
      allowedTools: ["mcp__postgres__execute_sql"]
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
              "postgres": {
                  "command": "npx",
                  # dbhub.toml sets readonly = true, so execute_sql rejects writes
                  "args": [
                      "-y",
                      "@bytebase/dbhub",
                      "--config",
                      "dbhub.toml",
                  ],
              }
          },
          allowed_tools=["mcp__postgres__execute_sql"],
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

Claude Code mengirimkan pesan `system` dengan subtype `init` di awal setiap kueri. Pesan ini mencakup status koneksi untuk setiap server MCP. Bidang `status` dapat berupa `"pending"`, `"connected"`, `"failed"`, `"needs-auth"`, atau `"disabled"`. Claude Code mengirimkan pesan init setelah [waktu tunggu koneksi putaran pertama](#connection-timing) untuk server yang dilewatkan dalam `options.mcpServers`, jadi server seperti itu yang terhubung dalam waktu tunggu menunjukkan `"connected"`.

Dalam pesan init, jangan perlakukan `"pending"` sebagai kegagalan dengan sendirinya. Ini dapat berarti salah satu dari ini:

* Server belum terhubung. Lihat [berapa lama Claude Code menunggu sebelum putaran pertama](#connection-timing)
* Daftar alat server [disajikan dari cache](#connection-timing), dengan koneksi yang dibuat pada penggunaan pertama
* Batas waktu koneksi telah kedaluwarsa. Server seperti itu melaporkan `"pending"` atau `"failed"` tergantung pada waktu

Periksa `"failed"` atau `"needs-auth"` untuk mendeteksi server yang tidak akan dapat digunakan:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  try {
    for await (const message of query({
      prompt: "Process data",
      options: {
        mcpServers: {
          // Replace dataServer with your server configuration
          "data-processor": dataServer
        }
      }
    })) {
      if (message.type === "system" && message.subtype === "init") {
        const unavailableServers = message.mcp_servers.filter(
          (s) => s.status === "failed" || s.status === "needs-auth"
        );

        if (unavailableServers.length > 0) {
          console.warn("Unavailable MCP servers:", unavailableServers);
        }
      }

      if (message.type === "result" && message.subtype === "error_during_execution") {
        console.error("Execution failed");
      }
    }
  } catch (error) {
    // A single-shot query() throws after yielding an error result. If the
    // failure was an error result, the error subtype branch above has
    // already run; a failure to start or reach the Claude Code process
    // yields no result message. MCP servers that fail to connect don't
    // throw: use the status check above, and note that servers still
    // "pending" at init need a later status check.
    console.log(`Session ended with an error: ${error}`);
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, SystemMessage, ResultMessage


  async def main():
      # Replace data_server with your server configuration
      options = ClaudeAgentOptions(mcp_servers={"data-processor": data_server})

      try:
          async for message in query(prompt="Process data", options=options):
              if isinstance(message, SystemMessage) and message.subtype == "init":
                  unavailable_servers = [
                      s
                      for s in message.data.get("mcp_servers", [])
                      if s.get("status") in ("failed", "needs-auth")
                  ]

                  if unavailable_servers:
                      print(f"Unavailable MCP servers: {unavailable_servers}")

              if (
                  isinstance(message, ResultMessage)
                  and message.subtype == "error_during_execution"
              ):
                  print("Execution failed")
      except Exception as error:
          # A single-shot query() raises after yielding an error result. If the
          # failure was an error result, the error subtype branch above has
          # already run; a failure to start or reach the Claude Code process
          # yields no result message. MCP servers that fail to connect don't
          # raise: use the status check above, and note that servers still
          # "pending" at init need a later status check.
          print(f"Session ended with an error: {error}")


  asyncio.run(main())
  ```
</CodeGroup>

Status server jarak jauh juga dapat berubah setelah melaporkan `"connected"`. Ketika koneksi ke server itu terputus di tengah sesi, Claude Code memindahkan server kembali ke `"pending"` sambil [menghubungkan kembali](/docs/id/mcp#automatic-reconnection). Panggilan `mcpServerStatus()` yang lebih baru dalam TypeScript, atau [`ClaudeSDKClient.get_mcp_status()`](/docs/id/agent-sdk/python#methods) dalam Python, kemudian dapat melaporkan `"pending"` untuk server yang Anda lihat terhubung sebelumnya, tanpa perubahan konfigurasi di pihak Anda.

Setelah lima upaya penghubungan kembali gagal, server melaporkan `"failed"`, atau `"needs-auth"` ketika perlu diotorisasi lagi. Untuk mencoba lagi secara manual, panggil [`reconnectMcpServer()`](/docs/id/agent-sdk/typescript#methods) dalam TypeScript atau [`ClaudeSDKClient.reconnect_mcp_server()`](/docs/id/agent-sdk/python#methods) dalam Python.

<h2 id="troubleshooting">
  Troubleshooting
</h2>

<h3 id="server-shows-failed-status">
  Server menunjukkan status "failed"
</h3>

Periksa pesan `init` untuk melihat server mana yang gagal terhubung:

<CodeGroup>
  ```typescript TypeScript theme={null}
  if (message.type === "system" && message.subtype === "init") {
    for (const server of message.mcp_servers) {
      if (server.status === "failed") {
        console.error(`Server ${server.name} failed to connect`);
      }
    }
  }
  ```

  ```python Python theme={null}
  if isinstance(message, SystemMessage) and message.subtype == "init":
      for server in message.data.get("mcp_servers", []):
          if server.get("status") == "failed":
              print(f"Server {server['name']} failed to connect")
  ```
</CodeGroup>

Status `"pending"` tidak berarti server gagal. Lihat [Error handling](#error-handling) untuk kasus-kasus yang dicakupnya saat init. Untuk mendapatkan status yang diperbarui nanti dalam sesi, panggil metode `mcpServerStatus()` query di TypeScript SDK, atau [`ClaudeSDKClient.get_mcp_status()`](/docs/id/agent-sdk/python#methods) di Python.

Penyebab umum:

* **Variabel lingkungan yang hilang**: Pastikan token dan kredensial yang diperlukan telah diatur. Untuk server stdio, periksa bahwa field `env` cocok dengan apa yang diharapkan server.
* **Server tidak terinstal**: Untuk perintah `npx`, verifikasi bahwa paket ada dan Node.js berada di PATH Anda.
* **String koneksi tidak valid**: Untuk server database, verifikasi format string koneksi dan bahwa database dapat diakses.
* **Masalah jaringan**: Untuk server HTTP/SSE jarak jauh, periksa bahwa URL dapat dijangkau dan firewall apa pun memungkinkan koneksi.

<h3 id="tools-not-being-called">
  Tools tidak dipanggil
</h3>

Jika Claude melihat tools tetapi tidak menggunakannya, periksa bahwa Anda telah memberikan izin dengan `allowedTools`:

<CodeGroup>
  ```typescript TypeScript hidelines={1,-1} theme={null}
  const _ = {
    options: {
      mcpServers: {
        // your servers
      },
      allowedTools: ["mcp__servername__*"] // Auto-approve calls from this server
    }
  };
  ```

  ```python Python theme={null}
  options = ClaudeAgentOptions(
      mcp_servers={
          # your servers
      },
      allowed_tools=["mcp__servername__*"],  # Auto-approve calls from this server
  )
  ```
</CodeGroup>

<h3 id="connection-timeouts">
  Connection timeouts
</h3>

Koneksi server MCP habis waktu setelah 30 detik secara default. Untuk mengubah berapa lama panggilan tool yang sedang berjalan dapat memakan waktu, atur [`MCP_TOOL_TIMEOUT`](/docs/id/env-vars). Jika server Anda membutuhkan waktu lebih lama untuk memulai, koneksi gagal. Naikkan batas koneksi dengan variabel lingkungan [`MCP_TIMEOUT`](/docs/id/env-vars), dalam milidetik. Untuk server yang membutuhkan lebih banyak waktu startup, pertimbangkan juga:

* Menggunakan server yang lebih ringan jika tersedia
* Pre-warming server sebelum memulai agent Anda
* Memeriksa log server untuk penyebab inisialisasi yang lambat

Di TypeScript, Anda dapat mengatur batas panggilan tool untuk [SDK MCP server](#sdk-mcp-servers) tunggal dengan melewatkan [`timeout` ke `createSdkMcpServer()`](/docs/id/agent-sdk/typescript#createsdkmcpserver).

<h3 id="tool-output-exceeds-maximum-allowed-tokens">
  Tool output melebihi token maksimal yang diizinkan
</h3>

SDK menerapkan batas output MCP yang sama dengan Claude Code. Ketika hasil tool tanpa konten gambar lebih besar dari 25.000 token, Claude Code menyimpan output ke file dan mengganti hasil tool dengan pesan kesalahan yang menyebutkan jalur file, sehingga agent dapat membaca output kembali dalam porsi.

Naikkan batas dengan variabel lingkungan [`MAX_MCP_OUTPUT_TOKENS`](/docs/id/env-vars). Lihat [MCP output limits and warnings](/docs/id/mcp#mcp-output-limits-and-warnings) untuk perilaku lengkap, termasuk bagaimana server dapat mendeklarasikan batas per-tool yang lebih tinggi dengan anotasi `anthropic/maxResultSizeChars`.

<h2 id="related-resources">
  Sumber daya terkait
</h2>

* **[Panduan alat kustom](/docs/id/agent-sdk/custom-tools)**: Bangun server MCP Anda sendiri yang berjalan dalam proses dengan aplikasi SDK Anda
* **[Izin](/docs/id/agent-sdk/permissions)**: Kontrol alat MCP mana yang dapat digunakan agen Anda dengan `allowedTools` dan `disallowedTools`
* **[Referensi TypeScript SDK](/docs/id/agent-sdk/typescript)**: Referensi API lengkap termasuk opsi konfigurasi MCP
* **[Referensi Python SDK](/docs/id/agent-sdk/python)**: Referensi API lengkap termasuk opsi konfigurasi MCP
* **[Direktori server MCP](https://github.com/modelcontextprotocol/servers)**: Jelajahi server MCP yang tersedia untuk database, API, dan lainnya
