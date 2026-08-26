> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Gambaran Umum Agent SDK

> Bangun agen AI produksi dengan Claude Code sebagai perpustakaan

Bangun agen AI yang secara mandiri membaca file, menjalankan perintah, mencari web, mengedit kode, dan banyak lagi. Agent SDK memberi Anda alat yang sama, loop agen, dan manajemen konteks yang mendukung Claude Code, dapat diprogram dalam Python dan TypeScript. Untuk pemikiran di balik desain harness agen, lihat [A harness for every task: dynamic workflows in Claude Code](https://claude.com/blog/a-harness-for-every-task-dynamic-workflows-in-claude-code) di blog.

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions


  async def main():
      async for message in query(
          prompt="Find and fix the bug in auth.py",
          options=ClaudeAgentOptions(allowed_tools=["Read", "Edit", "Bash"]),
      ):
          print(message)  # Claude reads the file, finds the bug, edits it


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Find and fix the bug in auth.ts",
    options: { allowedTools: ["Read", "Edit", "Bash"] }
  })) {
    console.log(message); // Claude reads the file, finds the bug, edits it
  }
  ```
</CodeGroup>

Agent SDK mencakup alat bawaan untuk membaca file, menjalankan perintah, dan mengedit kode, sehingga agen Anda dapat mulai bekerja segera tanpa Anda perlu mengimplementasikan eksekusi alat. Selami panduan cepat atau jelajahi agen nyata yang dibangun dengan SDK:

<CardGroup cols={2}>
  <Card title="Panduan Cepat" icon="play" href="/docs/id/agent-sdk/quickstart">
    Bangun agen perbaikan bug dalam hitungan menit
  </Card>

  <Card title="Agen contoh" icon="star" href="https://github.com/anthropics/claude-agent-sdk-demos">
    Asisten email, agen penelitian, dan banyak lagi
  </Card>
</CardGroup>

<h2 id="get-started">
  Memulai
</h2>

<Steps>
  <Step title="Instal SDK">
    <Tabs>
      <Tab title="TypeScript">
        ```bash theme={null}
        npm install @anthropic-ai/claude-agent-sdk
        ```
      </Tab>

      <Tab title="Python (uv)">
        [uv](https://docs.astral.sh/uv/) adalah manajer paket Python yang cepat yang menangani lingkungan virtual secara otomatis:

        ```bash theme={null}
        uv init
        uv add claude-agent-sdk
        ```
      </Tab>

      <Tab title="Python (pip)">
        Buat dan aktifkan lingkungan virtual, kemudian instal paket. Menginstal ke dalam lingkungan virtual menghindari kegagalan `error: externally-managed-environment` yang dikembalikan oleh Python sistem pada instalasi Debian, Ubuntu, dan Homebrew terbaru untuk `pip install` di luar venv.

        Di macOS atau Linux:

        ```bash theme={null}
        python3 -m venv .venv
        source .venv/bin/activate
        pip install claude-agent-sdk
        ```

        Di Windows:

        ```powershell theme={null}
        py -m venv .venv
        .venv\Scripts\Activate.ps1
        pip install claude-agent-sdk
        ```

        Jika PowerShell memblokir `Activate.ps1` dengan kesalahan kebijakan eksekusi, jalankan `Set-ExecutionPolicy -Scope Process RemoteSigned` terlebih dahulu.

        Paket Python memerlukan Python 3.10 atau lebih baru. Jika pip melaporkan `No matching distribution found for claude-agent-sdk`, interpreter Anda lebih lama dari 3.10. Jalankan `python3 --version` di macOS atau Linux, atau `py --version` di Windows, untuk memeriksa.
      </Tab>
    </Tabs>

    <Note>
      TypeScript SDK menggabungkan biner Claude Code asli untuk platform Anda sebagai dependensi opsional, jadi Anda tidak perlu menginstal Claude Code secara terpisah.
    </Note>
  </Step>

  <Step title="Atur kunci API Anda">
    Dapatkan kunci API dari [Konsol](https://platform.claude.com/), kemudian atur sebagai variabel lingkungan.

    Di macOS atau Linux:

    ```bash theme={null}
    export ANTHROPIC_API_KEY=sk-ant-xxxxx
    ```

    Di Windows PowerShell:

    ```powershell theme={null}
    $env:ANTHROPIC_API_KEY = "sk-ant-xxxxx"
    ```

    SDK juga mendukung autentikasi melalui penyedia API pihak ketiga:

    * **Amazon Bedrock**: atur variabel lingkungan `CLAUDE_CODE_USE_BEDROCK=1` dan konfigurasi kredensial AWS
    * **Claude Platform on AWS**: atur `CLAUDE_CODE_USE_ANTHROPIC_AWS=1` dan `ANTHROPIC_AWS_WORKSPACE_ID`, kemudian konfigurasi kredensial AWS
    * **Google Cloud's Agent Platform**: atur variabel lingkungan `CLAUDE_CODE_USE_VERTEX=1` dan konfigurasi kredensial Google Cloud
    * **Microsoft Azure**: atur variabel lingkungan `CLAUDE_CODE_USE_FOUNDRY=1` dan konfigurasi kredensial Azure

    Lihat panduan penyiapan untuk [Amazon Bedrock](/docs/id/amazon-bedrock), [Claude Platform on AWS](/docs/id/claude-platform-on-aws), [Google Cloud's Agent Platform](/docs/id/google-vertex-ai), atau [Microsoft Foundry](/docs/id/microsoft-foundry) untuk detail.

    <Note>
      Kecuali sebelumnya disetujui, Anthropic tidak mengizinkan pengembang pihak ketiga untuk menawarkan login claude.ai atau batas laju untuk produk mereka, termasuk agen yang dibangun di Claude Agent SDK. Silakan gunakan metode autentikasi kunci API yang dijelaskan dalam dokumen ini.
    </Note>
  </Step>

  <Step title="Jalankan agen pertama Anda">
    Contoh ini membuat agen yang mencantumkan file di direktori saat ini menggunakan alat bawaan.

    <CodeGroup>
      ```python Python theme={null}
      import asyncio
      from claude_agent_sdk import query, ClaudeAgentOptions


      async def main():
          async for message in query(
              prompt="What files are in this directory?",
              options=ClaudeAgentOptions(allowed_tools=["Bash", "Glob"]),
          ):
              if hasattr(message, "result"):
                  print(message.result)


      asyncio.run(main())
      ```

      ```typescript TypeScript theme={null}
      import { query } from "@anthropic-ai/claude-agent-sdk";

      for await (const message of query({
        prompt: "What files are in this directory?",
        options: { allowedTools: ["Bash", "Glob"] }
      })) {
        if ("result" in message) console.log(message.result);
      }
      ```
    </CodeGroup>
  </Step>
</Steps>

**Siap untuk membangun?** Ikuti [Panduan Cepat](/docs/id/agent-sdk/quickstart) untuk membuat agen yang menemukan dan memperbaiki bug dalam hitungan menit.

<h2 id="capabilities">
  Kemampuan
</h2>

Semua yang membuat Claude Code kuat tersedia di SDK:

<Tabs>
  <Tab title="Alat bawaan">
    Agen Anda dapat membaca file, menjalankan perintah, dan mencari basis kode langsung dari kotak. Alat kunci meliputi:

    | Alat                                                                        | Apa yang dilakukannya                                                               |
    | --------------------------------------------------------------------------- | ----------------------------------------------------------------------------------- |
    | **Read**                                                                    | Baca file apa pun di direktori kerja                                                |
    | **Write**                                                                   | Buat file baru                                                                      |
    | **Edit**                                                                    | Buat pengeditan presisi pada file yang ada                                          |
    | **Bash**                                                                    | Jalankan perintah terminal, skrip, operasi git                                      |
    | **Monitor**                                                                 | Pantau skrip latar belakang dan bereaksi terhadap setiap baris output sebagai acara |
    | **Glob**                                                                    | Temukan file berdasarkan pola (`**/*.ts`, `src/**/*.py`)                            |
    | **Grep**                                                                    | Cari konten file dengan regex                                                       |
    | **WebSearch**                                                               | Cari web untuk informasi terkini                                                    |
    | **WebFetch**                                                                | Ambil dan parsing konten halaman web                                                |
    | **[AskUserQuestion](/docs/id/agent-sdk/user-input#handle-clarifying-questions)** | Tanyakan pertanyaan klarifikasi kepada pengguna dengan opsi pilihan ganda           |

    Contoh ini membuat agen yang mencari basis kode Anda untuk komentar TODO:

    <CodeGroup>
      ```python Python theme={null}
      import asyncio
      from claude_agent_sdk import query, ClaudeAgentOptions


      async def main():
          async for message in query(
              prompt="Find all TODO comments and create a summary",
              options=ClaudeAgentOptions(allowed_tools=["Read", "Glob", "Grep"]),
          ):
              if hasattr(message, "result"):
                  print(message.result)


      asyncio.run(main())
      ```

      ```typescript TypeScript theme={null}
      import { query } from "@anthropic-ai/claude-agent-sdk";

      for await (const message of query({
        prompt: "Find all TODO comments and create a summary",
        options: { allowedTools: ["Read", "Glob", "Grep"] }
      })) {
        if ("result" in message) console.log(message.result);
      }
      ```
    </CodeGroup>
  </Tab>

  <Tab title="Hooks">
    Jalankan kode khusus pada titik-titik kunci dalam siklus hidup agen. SDK hooks menggunakan fungsi callback untuk memvalidasi, mencatat, memblokir, atau mengubah perilaku agen.

    **Hooks yang tersedia:** `PreToolUse`, `PostToolUse`, `Stop`, `SessionStart`, `SessionEnd`, `UserPromptSubmit`, dan banyak lagi.

    Contoh ini mencatat semua perubahan file ke file audit:

    <CodeGroup>
      ```python Python theme={null}
      import asyncio
      from datetime import datetime
      from claude_agent_sdk import query, ClaudeAgentOptions, HookMatcher


      async def log_file_change(input_data, tool_use_id, context):
          file_path = input_data.get("tool_input", {}).get("file_path", "unknown")
          with open("./audit.log", "a") as f:
              f.write(f"{datetime.now()}: modified {file_path}\n")
          return {}


      async def main():
          async for message in query(
              prompt="Refactor utils.py to improve readability",
              options=ClaudeAgentOptions(
                  permission_mode="acceptEdits",
                  hooks={
                      "PostToolUse": [
                          HookMatcher(matcher="Edit|Write", hooks=[log_file_change])
                      ]
                  },
              ),
          ):
              if hasattr(message, "result"):
                  print(message.result)


      asyncio.run(main())
      ```

      ```typescript TypeScript theme={null}
      import { query, HookCallback } from "@anthropic-ai/claude-agent-sdk";
      import { appendFile } from "fs/promises";

      const logFileChange: HookCallback = async (input) => {
        const filePath = (input as any).tool_input?.file_path ?? "unknown";
        await appendFile("./audit.log", `${new Date().toISOString()}: modified ${filePath}\n`);
        return {};
      };

      for await (const message of query({
        prompt: "Refactor utils.py to improve readability",
        options: {
          permissionMode: "acceptEdits",
          hooks: {
            PostToolUse: [{ matcher: "Edit|Write", hooks: [logFileChange] }]
          }
        }
      })) {
        if ("result" in message) console.log(message.result);
      }
      ```
    </CodeGroup>

    [Pelajari lebih lanjut tentang hooks →](/docs/id/agent-sdk/hooks)
  </Tab>

  <Tab title="Subagents">
    Spawn agen khusus untuk menangani subtask yang terfokus. Agen utama Anda mendelegasikan pekerjaan, dan subagen melaporkan kembali dengan hasil.

    Tentukan agen khusus dengan instruksi khusus. Subagen dipanggil melalui alat Agent, jadi sertakan `Agent` dalam `allowedTools` untuk auto-approve invokasi tersebut:

    <CodeGroup>
      ```python Python theme={null}
      import asyncio
      from claude_agent_sdk import query, ClaudeAgentOptions, AgentDefinition


      async def main():
          async for message in query(
              prompt="Use the code-reviewer agent to review this codebase",
              options=ClaudeAgentOptions(
                  allowed_tools=["Read", "Glob", "Grep", "Agent"],
                  agents={
                      "code-reviewer": AgentDefinition(
                          description="Expert code reviewer for quality and security reviews.",
                          prompt="Analyze code quality and suggest improvements.",
                          tools=["Read", "Glob", "Grep"],
                      )
                  },
              ),
          ):
              if hasattr(message, "result"):
                  print(message.result)


      asyncio.run(main())
      ```

      ```typescript TypeScript theme={null}
      import { query } from "@anthropic-ai/claude-agent-sdk";

      for await (const message of query({
        prompt: "Use the code-reviewer agent to review this codebase",
        options: {
          allowedTools: ["Read", "Glob", "Grep", "Agent"],
          agents: {
            "code-reviewer": {
              description: "Expert code reviewer for quality and security reviews.",
              prompt: "Analyze code quality and suggest improvements.",
              tools: ["Read", "Glob", "Grep"]
            }
          }
        }
      })) {
        if ("result" in message) console.log(message.result);
      }
      ```
    </CodeGroup>

    Pesan dari dalam konteks subagen mencakup bidang `parent_tool_use_id`, memungkinkan Anda melacak pesan mana yang termasuk dalam eksekusi subagen mana.

    [Pelajari lebih lanjut tentang subagents →](/docs/id/agent-sdk/subagents)
  </Tab>

  <Tab title="MCP">
    Terhubung ke sistem eksternal melalui Model Context Protocol: database, browser, API, dan [ratusan lainnya](https://github.com/modelcontextprotocol/servers).

    Contoh ini menghubungkan [server Playwright MCP](https://github.com/microsoft/playwright-mcp) untuk memberikan agen Anda kemampuan otomasi browser:

    <CodeGroup>
      ```python Python theme={null}
      import asyncio
      from claude_agent_sdk import query, ClaudeAgentOptions


      async def main():
          async for message in query(
              prompt="Open example.com and describe what you see",
              options=ClaudeAgentOptions(
                  mcp_servers={
                      "playwright": {"command": "npx", "args": ["@playwright/mcp@latest"]}
                  }
              ),
          ):
              if hasattr(message, "result"):
                  print(message.result)


      asyncio.run(main())
      ```

      ```typescript TypeScript theme={null}
      import { query } from "@anthropic-ai/claude-agent-sdk";

      for await (const message of query({
        prompt: "Open example.com and describe what you see",
        options: {
          mcpServers: {
            playwright: { command: "npx", args: ["@playwright/mcp@latest"] }
          }
        }
      })) {
        if ("result" in message) console.log(message.result);
      }
      ```
    </CodeGroup>

    [Pelajari lebih lanjut tentang MCP →](/docs/id/agent-sdk/mcp)
  </Tab>

  <Tab title="Izin">
    Kontrol dengan tepat alat mana yang dapat digunakan agen Anda. Izinkan operasi yang aman, blokir yang berbahaya, atau minta persetujuan untuk tindakan sensitif.

    <Note>
      Untuk prompt persetujuan interaktif dan alat `AskUserQuestion`, lihat [Tangani persetujuan dan input pengguna](/docs/id/agent-sdk/user-input).
    </Note>

    Contoh ini membuat agen read-only yang dapat menganalisis tetapi tidak memodifikasi kode. `allowed_tools` pra-menyetujui `Read`, `Glob`, dan `Grep`.

    <CodeGroup>
      ```python Python theme={null}
      import asyncio
      from claude_agent_sdk import query, ClaudeAgentOptions


      async def main():
          async for message in query(
              prompt="Review this code for best practices",
              options=ClaudeAgentOptions(
                  allowed_tools=["Read", "Glob", "Grep"],
              ),
          ):
              if hasattr(message, "result"):
                  print(message.result)


      asyncio.run(main())
      ```

      ```typescript TypeScript theme={null}
      import { query } from "@anthropic-ai/claude-agent-sdk";

      for await (const message of query({
        prompt: "Review this code for best practices",
        options: {
          allowedTools: ["Read", "Glob", "Grep"]
        }
      })) {
        if ("result" in message) console.log(message.result);
      }
      ```
    </CodeGroup>

    [Pelajari lebih lanjut tentang izin →](/docs/id/agent-sdk/permissions)
  </Tab>

  <Tab title="Sesi">
    Pertahankan konteks di seluruh pertukaran berganda. Claude mengingat file yang dibaca, analisis yang dilakukan, dan riwayat percakapan. Lanjutkan sesi nanti, atau fork mereka untuk menjelajahi pendekatan berbeda.

    Contoh ini menangkap ID sesi dari kueri pertama, kemudian melanjutkan untuk terus dengan konteks penuh:

    <CodeGroup>
      ```python Python theme={null}
      import asyncio
      from claude_agent_sdk import query, ClaudeAgentOptions, SystemMessage, ResultMessage


      async def main():
          session_id = None

          # First query: capture the session ID
          async for message in query(
              prompt="Read the authentication module",
              options=ClaudeAgentOptions(allowed_tools=["Read", "Glob"]),
          ):
              if isinstance(message, SystemMessage) and message.subtype == "init":
                  session_id = message.data["session_id"]

          # Resume with full context from the first query
          async for message in query(
              prompt="Now find all places that call it",  # "it" = auth module
              options=ClaudeAgentOptions(resume=session_id),
          ):
              if isinstance(message, ResultMessage):
                  print(message.result)


      asyncio.run(main())
      ```

      ```typescript TypeScript theme={null}
      import { query } from "@anthropic-ai/claude-agent-sdk";

      let sessionId: string | undefined;

      // First query: capture the session ID
      for await (const message of query({
        prompt: "Read the authentication module",
        options: { allowedTools: ["Read", "Glob"] }
      })) {
        if (message.type === "system" && message.subtype === "init") {
          sessionId = message.session_id;
        }
      }

      // Resume with full context from the first query
      for await (const message of query({
        prompt: "Now find all places that call it", // "it" = auth module
        options: { resume: sessionId }
      })) {
        if ("result" in message) console.log(message.result);
      }
      ```
    </CodeGroup>

    [Pelajari lebih lanjut tentang sesi →](/docs/id/agent-sdk/sessions)
  </Tab>
</Tabs>

<h3 id="claude-code-features">
  Fitur Claude Code
</h3>

SDK juga mendukung konfigurasi berbasis filesystem Claude Code. Dengan opsi default, SDK memuat ini dari `.claude/` di direktori kerja Anda dan `~/.claude/`. Untuk membatasi sumber mana yang dimuat, atur `setting_sources` (Python) atau `settingSources` (TypeScript) dalam opsi Anda.

| Fitur                                            | Deskripsi                                                                               | Lokasi                               |
| ------------------------------------------------ | --------------------------------------------------------------------------------------- | ------------------------------------ |
| [Skills](/docs/id/agent-sdk/skills)                   | Kemampuan khusus yang digunakan Claude secara otomatis atau Anda panggil dengan `/name` | `.claude/skills/*/SKILL.md`          |
| [Commands](/docs/id/agent-sdk/slash-commands)         | Perintah khusus dalam format legacy. Gunakan skills untuk perintah khusus baru          | `.claude/commands/*.md`              |
| [Memory](/docs/id/agent-sdk/modifying-system-prompts) | Konteks proyek dan instruksi                                                            | `CLAUDE.md` atau `.claude/CLAUDE.md` |
| [Plugins](/docs/id/agent-sdk/plugins)                 | Perluas dengan skills, agen, hooks, dan server MCP                                      | Programmatic via `plugins` option    |

<h2 id="compare-the-agent-sdk-to-other-claude-tools">
  Bandingkan Agent SDK dengan alat Claude lainnya
</h2>

Platform Claude menawarkan berbagai cara untuk membangun dengan Claude. Berikut cara Agent SDK cocok:

<Tabs>
  <Tab title="Agent SDK vs Client SDK">
    [Anthropic Client SDK](https://platform.claude.com/docs/id/api/client-sdks) memberi Anda akses API langsung: Anda mengirim prompt dan mengimplementasikan eksekusi alat sendiri. **Agent SDK** memberi Anda Claude dengan eksekusi alat bawaan.

    Dengan Client SDK, Anda mengimplementasikan loop alat. Dengan Agent SDK, Claude menanganinya:

    <CodeGroup>
      ```python Python theme={null}
      # Client SDK: You implement the tool loop
      response = client.messages.create(...)
      while response.stop_reason == "tool_use":
          result = your_tool_executor(response.tool_use)
          response = client.messages.create(tool_result=result, **params)

      # Agent SDK: Claude handles tools autonomously
      async for message in query(prompt="Fix the bug in auth.py"):
          print(message)
      ```

      ```typescript TypeScript theme={null}
      // Client SDK: You implement the tool loop
      let response = await client.messages.create({ ...params });
      while (response.stop_reason === "tool_use") {
        const result = yourToolExecutor(response.tool_use);
        response = await client.messages.create({ tool_result: result, ...params });
      }

      // Agent SDK: Claude handles tools autonomously
      for await (const message of query({ prompt: "Fix the bug in auth.ts" })) {
        console.log(message);
      }
      ```
    </CodeGroup>
  </Tab>

  <Tab title="Agent SDK vs Claude Code CLI">
    Kemampuan yang sama, antarmuka berbeda:

    | Kasus penggunaan        | Pilihan terbaik |
    | ----------------------- | --------------- |
    | Pengembangan interaktif | CLI             |
    | Pipeline CI/CD          | SDK             |
    | Aplikasi khusus         | SDK             |
    | Tugas sekali jalan      | CLI             |
    | Otomasi produksi        | SDK             |

    Banyak tim menggunakan keduanya: CLI untuk pengembangan harian, SDK untuk produksi. Alur kerja diterjemahkan langsung di antara keduanya.
  </Tab>

  <Tab title="Agent SDK vs Managed Agents">
    [Managed Agents](https://platform.claude.com/docs/id/managed-agents/overview) adalah REST API yang dihosting: Anthropic menjalankan agen dan sandbox, dan aplikasi Anda mengirim acara dan streaming kembali hasil. **Agent SDK** adalah perpustakaan yang menjalankan loop agen di dalam proses Anda sendiri.

    |                       | Agent SDK                                                                       | Managed Agents                                                                                           |
    | --------------------- | ------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------- |
    | **Berjalan di**       | Proses Anda, infrastruktur Anda                                                 | Infrastruktur yang dikelola Anthropic                                                                    |
    | **Antarmuka**         | Perpustakaan Python atau TypeScript                                             | REST API                                                                                                 |
    | **Agen bekerja pada** | File di infrastruktur Anda                                                      | Sandbox yang dikelola per sesi                                                                           |
    | **Status sesi**       | JSONL di sistem file Anda                                                       | Log acara yang dihosting Anthropic                                                                       |
    | **Alat khusus**       | Fungsi Python atau TypeScript dalam proses                                      | Claude memicu alat; Anda menjalankan dan mengembalikan hasil                                             |
    | **Terbaik untuk**     | Prototyping lokal, agen yang bekerja langsung pada sistem file dan layanan Anda | Agen produksi tanpa mengoperasikan infrastruktur sandbox atau sesi, sesi yang berjalan lama dan asinkron |

    Jalur umum adalah membuat prototipe dengan Agent SDK secara lokal, kemudian pindah ke Managed Agents untuk produksi.
  </Tab>
</Tabs>

<h2 id="changelog">
  Changelog
</h2>

Lihat changelog lengkap untuk pembaruan SDK, perbaikan bug, dan fitur baru:

* **TypeScript SDK**: [lihat CHANGELOG.md](https://github.com/anthropics/claude-agent-sdk-typescript/blob/main/CHANGELOG.md)
* **Python SDK**: [lihat CHANGELOG.md](https://github.com/anthropics/claude-agent-sdk-python/blob/main/CHANGELOG.md)

<h2 id="reporting-bugs">
  Melaporkan bug
</h2>

Jika Anda mengalami bug atau masalah dengan Agent SDK:

* **TypeScript SDK**: [laporkan masalah di GitHub](https://github.com/anthropics/claude-agent-sdk-typescript/issues)
* **Python SDK**: [laporkan masalah di GitHub](https://github.com/anthropics/claude-agent-sdk-python/issues)

<h2 id="branding-guidelines">
  Pedoman branding
</h2>

Untuk mitra yang mengintegrasikan Claude Agent SDK, penggunaan branding Claude bersifat opsional. Saat mereferensikan Claude dalam produk Anda:

**Diizinkan:**

* "Claude Agent" (lebih disukai untuk menu dropdown)
* "Claude" (ketika sudah dalam menu berlabel "Agents")
* "{YourAgentName} Powered by Claude" (jika Anda memiliki nama agen yang ada)

**Tidak diizinkan:**

* "Claude Code" atau "Claude Code Agent"
* Elemen visual atau ASCII art bermerek Claude Code yang meniru Claude Code

Produk Anda harus mempertahankan branding sendiri dan tidak boleh terlihat seperti Claude Code atau produk Anthropic apa pun. Untuk pertanyaan tentang kepatuhan branding, hubungi [tim penjualan](https://www.anthropic.com/contact-sales) Anthropic.

<h2 id="license-and-terms">
  Lisensi dan persyaratan
</h2>

Penggunaan Claude Agent SDK diatur oleh [Persyaratan Layanan Komersial Anthropic](https://www.anthropic.com/legal/commercial-terms), termasuk ketika Anda menggunakannya untuk memberdayakan produk dan layanan yang Anda buat tersedia untuk pelanggan dan pengguna akhir Anda sendiri, kecuali sejauh komponen atau dependensi tertentu dicakup oleh lisensi berbeda seperti yang ditunjukkan dalam file LICENSE komponen tersebut.

<h2 id="next-steps">
  Langkah berikutnya
</h2>

<CardGroup cols={2}>
  <Card title="Panduan Cepat" icon="play" href="/docs/id/agent-sdk/quickstart">
    Bangun agen yang menemukan dan memperbaiki bug dalam hitungan menit
  </Card>

  <Card title="Agen contoh" icon="star" href="https://github.com/anthropics/claude-agent-sdk-demos">
    Asisten email, agen penelitian, dan banyak lagi
  </Card>

  <Card title="TypeScript SDK" icon="code" href="/docs/id/agent-sdk/typescript">
    Referensi API TypeScript lengkap dan contoh
  </Card>

  <Card title="Python SDK" icon="code" href="/docs/id/agent-sdk/python">
    Referensi API Python lengkap dan contoh
  </Card>
</CardGroup>
