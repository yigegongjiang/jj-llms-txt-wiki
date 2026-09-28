> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Subagents dalam SDK

> Tentukan dan panggil subagents untuk mengisolasi konteks, menjalankan tugas secara paralel, dan menerapkan instruksi khusus dalam aplikasi Claude Agent SDK Anda.

Subagents adalah instans agen terpisah yang dapat dihasilkan oleh agen utama Anda untuk menangani subtask yang terfokus.
Gunakan mereka untuk mengisolasi konteks, menjalankan beberapa analisis secara paralel, dan menerapkan instruksi khusus tanpa menambah prompt agen utama Anda.

<h2 id="overview">
  Ikhtisar
</h2>

Anda dapat membuat subagen dalam tiga cara:

* **Secara Programatik**: gunakan parameter `agents` dalam opsi `query()` Anda. Lihat referensi [TypeScript](/docs/id/agent-sdk/typescript#agentdefinition) dan [Python](/docs/id/agent-sdk/python#agentdefinition)
* **Berbasis Sistem File**: tentukan agen sebagai file markdown di direktori `.claude/agents/`. Lihat [mendefinisikan subagen sebagai file](/docs/id/sub-agents)
* **Tujuan Umum Bawaan**: Claude dapat memanggil subagen `general-purpose` bawaan kapan saja melalui alat Agent tanpa Anda perlu mendefinisikan apa pun

Panduan ini berfokus pada pendekatan programatik, yang direkomendasikan untuk aplikasi SDK.

<h2 id="benefits-of-using-subagents">
  Manfaat menggunakan subagents
</h2>

Karena subagents adalah instance agent terpisah, mendelegasikan pekerjaan kepada mereka memberikan Anda empat manfaat:

* **Isolasi konteks**: setiap subagent berjalan dalam percakapannya sendiri, yang dimulai segar kecuali subagent adalah [fork](/docs/id/sub-agents#fork-the-current-conversation). Bagaimanapun, panggilan alat perantara dan hasil tetap berada di dalam subagent; hanya pesan finalnya yang kembali ke parent. Subagent `research-assistant` dapat menjelajahi puluhan file tanpa konten apa pun yang terakumulasi dalam percakapan utama. Parent menerima ringkasan ringkas, bukan setiap file yang dibaca subagent. Lihat [What subagents inherit](#what-subagents-inherit) untuk mengetahui dengan tepat apa yang ada dalam konteks subagent.
* **Paralelisasi**: beberapa subagents dapat berjalan secara bersamaan, sehingga subtask independen selesai dalam waktu yang paling lambat daripada jumlah semua dari mereka. Selama tinjauan kode, Anda dapat menjalankan subagents `style-checker`, `security-scanner`, dan `test-coverage` secara bersamaan daripada secara berurutan.
* **Instruksi dan pengetahuan khusus**: setiap subagent dapat memiliki system prompt yang disesuaikan dengan keahlian spesifik, praktik terbaik, dan batasan. Subagent `database-migration` dapat memiliki pengetahuan terperinci tentang praktik terbaik SQL, strategi rollback, dan pemeriksaan integritas data yang akan menjadi kebisingan yang tidak perlu dalam instruksi agent utama.
* **Pembatasan alat**: subagents dapat dibatasi pada alat tertentu, mengurangi risiko tindakan yang tidak diinginkan. Subagent `doc-reviewer` mungkin hanya memiliki akses ke alat Read dan Grep, memastikan bahwa ia dapat menganalisis tetapi tidak pernah secara tidak sengaja memodifikasi file dokumentasi Anda.

<h2 id="create-subagents">
  Buat subagen
</h2>

<h3 id="programmatic-definition-recommended">
  Definisi programatik (direkomendasikan)
</h3>

Tentukan subagen langsung dalam kode Anda menggunakan parameter `agents`. Claude menjalankan subagen melalui tool `Agent`.

Sebagian besar contoh di halaman ini hanya mencetak hasil akhir. Untuk memastikan bahwa Claude mendelegasikan ke subagen daripada menjawab secara langsung, lihat [Deteksi invokasi subagen](#detect-subagent-invocation).

Contoh ini membuat dua subagen: pengulas kode dengan akses baca-saja dan pelari tes yang dapat menjalankan perintah.

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, AgentDefinition


  async def main():
      async for message in query(
          prompt="Review the authentication module for security issues",
          options=ClaudeAgentOptions(
              # Auto-approve these tools
              allowed_tools=["Read", "Grep", "Glob", "Agent"],
              agents={
                  "code-reviewer": AgentDefinition(
                      # description tells Claude when to use this subagent
                      description="Expert code review specialist. Use for quality, security, and maintainability reviews.",
                      # prompt defines the subagent's behavior and expertise
                      prompt="""You are a code review specialist with expertise in security, performance, and best practices.

  When reviewing code:
  - Identify security vulnerabilities
  - Check for performance issues
  - Verify adherence to coding standards
  - Suggest specific improvements

  Be thorough but concise in your feedback.""",
                      # tools restricts what the subagent can do (read-only here)
                      tools=["Read", "Grep", "Glob"],
                      # model overrides the default model for this subagent
                      model="sonnet",
                  ),
                  "test-runner": AgentDefinition(
                      description="Runs and analyzes test suites. Use for test execution and coverage analysis.",
                      prompt="""You are a test execution specialist. Run tests and provide clear analysis of results.

  Focus on:
  - Running test commands
  - Analyzing test output
  - Identifying failing tests
  - Suggesting fixes for failures""",
                      # Bash access lets this subagent run test commands
                      tools=["Bash", "Read", "Grep"],
                  ),
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
    prompt: "Review the authentication module for security issues",
    options: {
      // Auto-approve these tools
      allowedTools: ["Read", "Grep", "Glob", "Agent"],
      agents: {
        "code-reviewer": {
          // description tells Claude when to use this subagent
          description:
            "Expert code review specialist. Use for quality, security, and maintainability reviews.",
          // prompt defines the subagent's behavior and expertise
          prompt: `You are a code review specialist with expertise in security, performance, and best practices.

  When reviewing code:
  - Identify security vulnerabilities
  - Check for performance issues
  - Verify adherence to coding standards
  - Suggest specific improvements

  Be thorough but concise in your feedback.`,
          // tools restricts what the subagent can do (read-only here)
          tools: ["Read", "Grep", "Glob"],
          // model overrides the default model for this subagent
          model: "sonnet"
        },
        "test-runner": {
          description:
            "Runs and analyzes test suites. Use for test execution and coverage analysis.",
          prompt: `You are a test execution specialist. Run tests and provide clear analysis of results.

  Focus on:
  - Running test commands
  - Analyzing test output
  - Identifying failing tests
  - Suggesting fixes for failures`,
          // Bash access lets this subagent run test commands
          tools: ["Bash", "Read", "Grep"]
        }
      }
    }
  })) {
    if ("result" in message) console.log(message.result);
  }
  ```
</CodeGroup>

<h3 id="agentdefinition-configuration">
  Konfigurasi AgentDefinition
</h3>

| Field             | Type                                                        | Required | Description                                                                                                                                                                                                                                                                                                                                                      |
| :---------------- | :---------------------------------------------------------- | :------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `description`     | `string`                                                    | Ya       | Deskripsi bahasa alami tentang kapan menggunakan agen ini                                                                                                                                                                                                                                                                                                        |
| `prompt`          | `string`                                                    | Ya       | Prompt sistem agen yang mendefinisikan peran dan perilakunya                                                                                                                                                                                                                                                                                                     |
| `tools`           | `string[]`                                                  | Tidak    | Array nama tool yang diizinkan. Jika dihilangkan, mewarisi setiap [tool yang tersedia untuk subagen](/docs/id/sub-agents#available-tools)                                                                                                                                                                                                                             |
| `disallowedTools` | `string[]`                                                  | Tidak    | Array nama tool yang akan dihapus dari set tool agen. Pola tingkat server MCP juga diterima: `mcp__server` atau `mcp__server__*` menghapus setiap tool dari server tersebut, dan `mcp__*` menghapus setiap tool MCP dari server apa pun                                                                                                                          |
| `model`           | `string`                                                    | Tidak    | Penggantian model untuk agen ini. Menerima alias seperti `'fable'`, `'opus'`, `'sonnet'`, `'haiku'`, `'inherit'`, atau ID model lengkap. `'inherit'` menggunakan model utama. Ketika Anda menghilangkannya, Claude Code memilih model dalam [urutan model subagen](/docs/id/sub-agents#choose-a-model)                                                                |
| `skills`          | `string[]`                                                  | Tidak    | Daftar nama skill yang dimuat sebelumnya ke dalam konteks agen saat startup. Skill yang tidak terdaftar tetap dapat dipanggil melalui tool Skill                                                                                                                                                                                                                 |
| `memory`          | `'user' \| 'project' \| 'local'`                            | Tidak    | Sumber memori untuk agen ini                                                                                                                                                                                                                                                                                                                                     |
| `mcpServers`      | `(string \| object)[]`                                      | Tidak    | Server MCP yang tersedia untuk agen ini, berdasarkan nama atau konfigurasi inline                                                                                                                                                                                                                                                                                |
| `initialPrompt`   | `string`                                                    | Tidak    | Otomatis dikirimkan sebagai giliran pengguna pertama ketika agen ini berjalan sebagai agen thread utama. Diabaikan ketika agen dipanggil sebagai subagen                                                                                                                                                                                                         |
| `maxTurns`        | `number`                                                    | Tidak    | Jumlah maksimum giliran agentic sebelum agen berhenti. Ketika agen mencapai batas, Claude Code mengembalikan output-nya yang ditandai sebagai parsial, dan Anda dapat [melanjutkan agen](#resume-subagents) untuk terus berlanjut. Penandaan parsial memerlukan Claude Code v2.1.246 atau lebih baru                                                             |
| `background`      | `boolean`                                                   | Tidak    | Jalankan agen ini sebagai tugas latar belakang non-blocking ketika dipanggil                                                                                                                                                                                                                                                                                     |
| `omitClaudeMd`    | `boolean`                                                   | Tidak    | Jalankan agen ini tanpa file CLAUDE.md pengguna, proyek, dan lokal ketika berjalan sebagai subagen; file kebijakan yang dikelola tetap dimuat. Diabaikan ketika agen berjalan sebagai agen thread utama. Memerlukan TypeScript Agent SDK v0.3.271 atau lebih baru. Python SDK [`AgentDefinition`](/docs/id/agent-sdk/python#agentdefinition) tidak memiliki field ini |
| `effort`          | `'low' \| 'medium' \| 'high' \| 'xhigh' \| 'max' \| number` | Tidak    | Tingkat upaya penalaran untuk agen ini                                                                                                                                                                                                                                                                                                                           |
| `permissionMode`  | `PermissionMode`                                            | Tidak    | Mode izin untuk eksekusi tool dalam agen ini. [Aturan pewarisan subagen](/docs/id/agent-sdk/permissions#available-modes) menentukan kapan itu berlaku                                                                                                                                                                                                                 |

Dalam Python SDK, nama field multi-kata seperti `disallowedTools` dan `mcpServers` mempertahankan ejaan camelCase mereka untuk mencocokkan format wire daripada mengikuti konvensi snake\_case Python. Lihat referensi [`AgentDefinition`](/docs/id/agent-sdk/python#agentdefinition) untuk detail.

Subagen berjalan di latar belakang secara default. Panggilan tool Agent yang menghilangkan input [`run_in_background`](/docs/id/sub-agents#run-subagents-in-foreground-or-background) meluncurkan subagen latar belakang, dan Claude menetapkan `run_in_background: false` ketika memerlukan hasil sebelum melanjutkan. Atur field `background` ke `true` untuk memaksa eksekusi latar belakang untuk agen tertentu terlepas dari apa yang diminta Claude. Sebelum Claude Code v2.1.198, default latar belakang sedang diluncurkan secara bertahap, dan panggilan tool Agent yang menghilangkan `run_in_background` dapat menjalankan subagen secara sinkron.

Subagen juga dapat menghasilkan subagen mereka sendiri. Untuk membatasi seberapa dalam nesting itu, berapa banyak subagen yang berjalan sekaligus, dan berapa banyak kueri yang dihabiskan, lihat [Batasi kedalaman, konkurensi, dan pengeluaran subagen](#cap-subagent-depth-concurrency-and-spend).

<h3 id="filesystem-based-definition-alternative">
  Definisi berbasis filesystem (alternatif)
</h3>

Anda juga dapat mendefinisikan subagen sebagai file markdown di direktori `.claude/agents/`. Lihat [dokumentasi subagen Claude Code](/docs/id/sub-agents) untuk detail tentang pendekatan ini. Agen yang didefinisikan secara programatik memiliki prioritas lebih tinggi daripada agen berbasis filesystem dengan nama yang sama.

<Note>
  Ketika Claude memanggil tool Agent tanpa `subagent_type`, ia mendapatkan subagen `general-purpose` bawaan, yang dapat Claude hasilkan bahkan ketika Anda tidak mendefinisikan agen apa pun sendiri. Menetapkan [`CLAUDE_AGENT_SDK_DISABLE_BUILTIN_AGENTS=1`](/docs/id/env-vars) menghapus default tersebut, dan panggilan seperti itu gagal dengan [`subagent_type is required`](/docs/id/errors#subagent-type-is-required).
</Note>

<h2 id="what-subagents-inherit">
  Apa yang diwariskan subagent
</h2>

Kecuali subagent adalah [fork](/docs/id/sub-agents#fork-the-current-conversation), jendela konteksnya dimulai segar, tanpa percakapan induk, tetapi tidak kosong. Satu-satunya konten yang Anda teruskan dari induk ke subagent adalah string prompt alat Agent, jadi sertakan jalur file, pesan kesalahan, atau keputusan apa pun yang dibutuhkan subagent langsung dalam prompt tersebut.

Subagent yang memiliki alat [`SendMessage`](/docs/id/tools-reference) dimulai dengan daftar agen bernama lainnya yang berjalan dalam sesi, sehingga mengetahui nama mana yang dapat dikirim pesan. Claude Code menambahkan daftar ke giliran pertama subagent secara otomatis. [Fork](/docs/id/sub-agents#fork-the-current-conversation) tidak mendapatkan daftar karena mewarisi percakapan induk sebagai gantinya.

Subagent juga mewarisi konfigurasi pemikiran yang diperluas dari sesi utama.

Tabel di bawah mencantumkan apa yang diterima konteks subagent non-fork dan apa yang ditinggalkannya.

| Subagent menerima                                                                                                                                                                                                   | Subagent tidak menerima                                                               |
| :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | :------------------------------------------------------------------------------------ |
| Prompt sistem miliknya sendiri (`AgentDefinition.prompt`) dan prompt alat Agent                                                                                                                                     | Riwayat percakapan induk atau hasil alat                                              |
| Project CLAUDE.md (dimuat melalui [`settingSources`](/docs/id/agent-sdk/claude-code-features#control-filesystem-settings-with-settingsources)), kecuali agen menetapkan [`omitClaudeMd`](#agentdefinition-configuration) | Konten skill yang dimuat sebelumnya, kecuali tercantum dalam `AgentDefinition.skills` |
| Definisi alat (diwariskan dari induk atau subset dalam `tools`, [disaring untuk background runs](/docs/id/sub-agents#available-tools))                                                                                   | Prompt sistem induk                                                                   |

<Note>
  Induk menerima pesan terakhir subagent sebagai hasil alat Agent, tetapi dapat merangkumnya dalam respons miliknya sendiri. Untuk mempertahankan output subagent verbatim dalam respons yang menghadap pengguna, sertakan instruksi untuk melakukannya dalam prompt atau opsi `systemPrompt` yang Anda teruskan ke panggilan `query()` utama.

  Dalam v2.1.210 dan yang lebih baru, Claude Code [memindai pesan terakhir untuk pola berbentuk instruksi](/docs/id/sub-agents#subagent-output-scanning) sebelum induk membacanya. Pemindaian memperlakukan tiga jenis pola secara berbeda:

  * **Peniruan tag kontrol**: Claude Code menetralkan tag yang hanya dipancarkan harness, seperti blok `<system-reminder>`, di tempat. Ini menyisipkan garis miring terbalik setelah kurung sudut pembuka dan tidak menghapus apa pun.
  * **Penyebutan konfigurasi izin**: Claude Code menyimpan referensi ke konfigurasi izin, seperti `.claude/settings.json`, `bypassPermissions`, atau `--dangerously-skip-permissions`, seperti yang ditulis.
  * **Penanda giliran**: baris yang dimulai dengan `Human:` atau `Assistant:` mendapatkan garis miring terbalik sebelum titik dua, sehingga pesan tidak dapat meniru batas giliran percakapan.

  Untuk kecocokan tag kontrol atau konfigurasi izin, Claude Code menambahkan baris penanda `[harness: ...]` yang menamai pola yang cocok; kecocokan penanda giliran tidak menambahkan baris penanda. Itu adalah satu-satunya modifikasi yang dilakukan pemindaian: tidak pernah menghapus atau mengubah kata-kata subagent.
</Note>

Kesalahan API yang mengakhiri subagent lebih awal, seperti batas laju, tidak pernah dikirimkan sebagai hasilnya. Lihat [Kesalahan API dalam subagent](/docs/id/sub-agents#api-errors-in-subagents) untuk perilaku foreground dan background.

<h2 id="invoke-subagents">
  Panggil subagen
</h2>

<h3 id="automatic-invocation">
  Pemanggilan otomatis
</h3>

Claude secara otomatis memutuskan kapan harus memanggil subagen berdasarkan tugas dan `description` setiap subagen. Misalnya, jika Anda mendefinisikan subagen `performance-optimizer` dengan deskripsi "Performance optimization specialist for query tuning", Claude akan memanggilnya ketika prompt Anda menyebutkan optimasi kueri.

Tulis deskripsi yang jelas dan spesifik sehingga Claude dapat mencocokkan tugas dengan subagen yang tepat.

<h3 id="explicit-invocation">
  Pemanggilan eksplisit
</h3>

Untuk menjamin Claude menggunakan subagen tertentu, sebutkan namanya dalam prompt Anda:

```text theme={null}
"Use the code-reviewer agent to check the authentication module"
```

Ini melewati pencocokan otomatis dan secara langsung memanggil subagen yang dinamai.

<h3 id="dynamic-agent-configuration">
  Konfigurasi agen dinamis
</h3>

Anda dapat membuat definisi agen secara dinamis berdasarkan kondisi runtime. Contoh ini membuat reviewer keamanan dengan tingkat ketat yang berbeda, menggunakan model yang lebih mampu untuk review ketat.

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, AgentDefinition


  # Factory function that returns an AgentDefinition
  # This pattern lets you customize agents based on runtime conditions
  def create_security_agent(security_level: str) -> AgentDefinition:
      is_strict = security_level == "strict"
      return AgentDefinition(
          description="Security code reviewer",
          # Customize the prompt based on strictness level
          prompt=f"You are a {'strict' if is_strict else 'balanced'} security reviewer...",
          tools=["Read", "Grep", "Glob"],
          # Key insight: use a more capable model for high-stakes reviews
          model="opus" if is_strict else "sonnet",
      )


  async def main():
      # The agent is created at query time, so each request can use different settings
      async for message in query(
          prompt="Review this PR for security issues",
          options=ClaudeAgentOptions(
              allowed_tools=["Read", "Grep", "Glob", "Agent"],
              agents={
                  # Call the factory with your desired configuration
                  "security-reviewer": create_security_agent("strict")
              },
          ),
      ):
          if hasattr(message, "result"):
              print(message.result)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query, type AgentDefinition } from "@anthropic-ai/claude-agent-sdk";

  // Factory function that returns an AgentDefinition
  // This pattern lets you customize agents based on runtime conditions
  function createSecurityAgent(securityLevel: "basic" | "strict"): AgentDefinition {
    const isStrict = securityLevel === "strict";
    return {
      description: "Security code reviewer",
      // Customize the prompt based on strictness level
      prompt: `You are a ${isStrict ? "strict" : "balanced"} security reviewer...`,
      tools: ["Read", "Grep", "Glob"],
      // Key insight: use a more capable model for high-stakes reviews
      model: isStrict ? "opus" : "sonnet"
    };
  }

  // The agent is created at query time, so each request can use different settings
  for await (const message of query({
    prompt: "Review this PR for security issues",
    options: {
      allowedTools: ["Read", "Grep", "Glob", "Agent"],
      agents: {
        // Call the factory with your desired configuration
        "security-reviewer": createSecurityAgent("strict")
      }
    }
  })) {
    if ("result" in message) console.log(message.result);
  }
  ```
</CodeGroup>

<h2 id="detect-subagent-invocation">
  Deteksi pemanggilan subagent
</h2>

Claude memanggil subagent melalui tool Agent. Untuk mendeteksi ketika subagent dipanggil, periksa blok `tool_use` di mana `name` adalah `"Agent"`. Pesan dari dalam konteks subagent mencakup field `parent_tool_use_id`.

<Note>
  Tool muncul sebagai `"Agent"` dalam blok `tool_use` tetapi sebagai `"Task"` dalam daftar tool `system:init`. Sebelum Claude Code v2.1.63, blok `tool_use` juga menamakannya `"Task"`. Untuk menjaga deteksi tetap berfungsi di seluruh versi SDK, cocokkan kedua nilai dalam `block.name`.
</Note>

Struktur pesan berbeda antara SDK. Di Python, Anda mengakses blok konten secara langsung melalui `message.content`. Di TypeScript, `SDKAssistantMessage` membungkus pesan API Claude, jadi Anda mengakses konten melalui `message.message.content`.

Contoh ini melakukan iterasi melalui pesan yang di-stream, mencatat ketika subagent dipanggil dan ketika pesan berikutnya berasal dari dalam konteks eksekusi subagent tersebut.

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, AgentDefinition, ToolUseBlock


  async def main():
      async for message in query(
          prompt="Use the code-reviewer agent to review this codebase",
          options=ClaudeAgentOptions(
              allowed_tools=["Read", "Glob", "Grep", "Agent"],
              agents={
                  "code-reviewer": AgentDefinition(
                      description="Expert code reviewer.",
                      prompt="Analyze code quality and suggest improvements.",
                      tools=["Read", "Glob", "Grep"],
                  )
              },
          ),
      ):
          # Check for subagent invocation. Match both names: older SDK
          # versions emitted "Task", current versions emit "Agent".
          if hasattr(message, "content") and message.content:
              for block in message.content:
                  if isinstance(block, ToolUseBlock) and block.name in (
                      "Task",
                      "Agent",
                  ):
                      print(f"Subagent invoked: {block.input.get('subagent_type')}")

          # Check if this message is from within a subagent's context
          if hasattr(message, "parent_tool_use_id") and message.parent_tool_use_id:
              print("  (running inside subagent)")

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
          description: "Expert code reviewer.",
          prompt: "Analyze code quality and suggest improvements.",
          tools: ["Read", "Glob", "Grep"]
        }
      }
    }
  })) {
    const msg = message as any;

    // Check for subagent invocation. Match both names: older SDK versions
    // emitted "Task", current versions emit "Agent".
    for (const block of msg.message?.content ?? []) {
      if (block.type === "tool_use" && (block.name === "Task" || block.name === "Agent")) {
        console.log(`Subagent invoked: ${block.input.subagent_type}`);
      }
    }

    // Check if this message is from within a subagent's context
    if (msg.parent_tool_use_id) {
      console.log("  (running inside subagent)");
    }

    if ("result" in message) {
      console.log(message.result);
    }
  }
  ```
</CodeGroup>

<h2 id="resume-subagents">
  Melanjutkan subagents
</h2>

Anda dapat melanjutkan subagent untuk meneruskan dari tempat ia berhenti daripada memulai dari awal. Subagent yang dilanjutkan mempertahankan riwayat percakapan lengkapnya, termasuk semua panggilan alat sebelumnya, hasil, dan penalaran.

Ketika subagent berhenti pada batas [`maxTurns`](#agentdefinition-configuration) nya, Claude Code menandai output dalam hasil alat Agent sebagai parsial, sehingga Claude mengetahui bahwa jalannya belum selesai.

Ketika subagent selesai, hasil alat Agent mencakup blok teks yang berisi `agentId: <id>`. Agent [`Explore` dan `Plan`](/docs/id/sub-agents#built-in-subagents) bawaan adalah one-shot dan tidak mengembalikan `agentId`, jadi gunakan agent kustom atau `general-purpose` ketika Anda perlu melanjutkan. Untuk melanjutkan subagent secara terprogram:

1. **Tangkap ID sesi**: ekstrak `session_id` dari pesan selama kueri pertama
2. **Ekstrak ID agent**: parse `agentId` dari teks hasil alat Agent
3. **Lanjutkan sesi**: teruskan `resume: sessionId` dalam opsi kueri kedua, dan sertakan ID agent dalam prompt Anda. Setiap panggilan `query()` memulai sesi baru secara default, dan Anda harus melanjutkan sesi yang sama untuk mengakses transkrip subagent.

<Note>
  Saat menggunakan agent kustom, teruskan definisi agent yang sama dalam parameter `agents` untuk kedua kueri.
</Note>

Contoh di bawah ini mendefinisikan agent `endpoint-finder` kustom. Kueri pertama menjalankannya dan menangkap ID sesi dan ID agent dari hasil alat Agent, kemudian kueri kedua melanjutkan sesi untuk mengajukan pertanyaan lanjutan yang memerlukan konteks dari analisis pertama.

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  import re
  from claude_agent_sdk import query, ClaudeAgentOptions, AgentDefinition, ToolResultBlock

  AGENTS = {
      "endpoint-finder": AgentDefinition(
          description="Locates and catalogs API endpoints in a codebase.",
          prompt="You find and document API endpoints. Report each endpoint's path, method, and handler.",
          tools=["Read", "Grep", "Glob"],
      )
  }


  def extract_agent_id(block: ToolResultBlock) -> str | None:
      """Extract agentId from an Agent tool result's text content."""
      parts = block.content if isinstance(block.content, list) else [{"text": block.content}]
      for part in parts:
          if match := re.search(r"agentId:\s*([\w-]+)", part.get("text") or ""):
              return match.group(1)
      return None


  async def main():
      agent_id = None
      session_id = None

      # First invocation - run the endpoint-finder subagent
      try:
          async for message in query(
              prompt="Use the endpoint-finder agent to find all API endpoints in this codebase",
              options=ClaudeAgentOptions(allowed_tools=["Read", "Grep", "Glob", "Agent"], agents=AGENTS),
          ):
              # Capture session_id from ResultMessage (needed to resume this session)
              if hasattr(message, "session_id"):
                  session_id = message.session_id
              # Search tool results for the agentId trailer
              for block in getattr(message, "content", None) or []:
                  if isinstance(block, ToolResultBlock):
                      agent_id = extract_agent_id(block) or agent_id
              # Print the final result
              if hasattr(message, "result"):
                  print(message.result)
      except Exception as error:
          # A single-shot query() raises after yielding an error result,
          # so session_id and agent_id have already been captured by the loop above.
          print(f"Session ended with an error: {error}")

      # Second invocation - resume and ask follow-up
      if agent_id and session_id:
          async for message in query(
              prompt=f"Resume agent {agent_id} and list the top 3 most complex endpoints",
              options=ClaudeAgentOptions(
                  allowed_tools=["Read", "Grep", "Glob", "Agent"], agents=AGENTS, resume=session_id
              ),
          ):
              if hasattr(message, "result"):
                  print(message.result)
      else:
          print("No agentId found in the first query, so there is no subagent to resume.")


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query, type SDKMessage } from "@anthropic-ai/claude-agent-sdk";

  const agents = {
    "endpoint-finder": {
      description: "Locates and catalogs API endpoints in a codebase.",
      prompt: "You find and document API endpoints. Report each endpoint's path, method, and handler.",
      tools: ["Read", "Grep", "Glob"]
    }
  };

  // Stringify content to search for agentId without traversing nested block types
  function extractAgentId(message: SDKMessage): string | undefined {
    if (message.type !== "assistant" && message.type !== "user") return undefined;
    const content = JSON.stringify(message.message.content);
    const match = content.match(/agentId:\s*([\w-]+)/);
    return match?.[1];
  }

  let agentId: string | undefined;
  let sessionId: string | undefined;

  // First invocation - run the endpoint-finder subagent
  try {
    for await (const message of query({
      prompt: "Use the endpoint-finder agent to find all API endpoints in this codebase",
      options: { allowedTools: ["Read", "Grep", "Glob", "Agent"], agents }
    })) {
      // Capture session_id from ResultMessage (needed to resume this session)
      if ("session_id" in message) sessionId = message.session_id;
      // Search message content for the agentId (appears in Agent tool results)
      const extractedId = extractAgentId(message);
      if (extractedId) agentId = extractedId;
      // Print the final result
      if ("result" in message) console.log(message.result);
    }
  } catch (error) {
    // A single-shot query() throws after yielding an error result,
    // so sessionId and agentId have already been captured by the loop above.
    console.error(`Session ended with an error: ${error}`);
  }

  // Second invocation - resume and ask follow-up
  if (agentId && sessionId) {
    for await (const message of query({
      prompt: `Resume agent ${agentId} and list the top 3 most complex endpoints`,
      options: { allowedTools: ["Read", "Grep", "Glob", "Agent"], agents, resume: sessionId }
    })) {
      if ("result" in message) console.log(message.result);
    }
  } else {
    console.log("No agentId found in the first query, so there is no subagent to resume.");
  }
  ```
</CodeGroup>

Transkrip subagent disimpan dalam file terpisah dan bertahan secara independen dari percakapan utama. Lihat [melanjutkan subagents di Claude Code](/docs/id/sub-agents#resume-subagents) untuk perilaku pemadatan dan periode pembersihan `cleanupPeriodDays`.

<h2 id="tool-restrictions">
  Pembatasan alat
</h2>

Gunakan field `tools` untuk membatasi apa yang dapat dilakukan subagen:

* **Abaikan `tools`**: subagen mendapatkan setiap [alat yang tersedia untuk subagen](/docs/id/sub-agents#available-tools)
* **Daftar alat**: subagen hanya mendapatkan yang tersebut. Misalnya, pengulas kode yang tidak boleh mengedit file mendapatkan `["Read", "Grep", "Glob"]`

Alat yang Anda tinggalkan tidak ada dalam sesi subagen sama sekali: Claude bekerja tanpanya, tanpa permintaan izin atau kesalahan.

Contoh ini membuat agen analisis baca-saja yang dapat memeriksa kode tetapi tidak dapat memodifikasi file atau menjalankan perintah.

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, AgentDefinition


  async def main():
      async for message in query(
          prompt="Analyze the architecture of this codebase",
          options=ClaudeAgentOptions(
              allowed_tools=["Read", "Grep", "Glob", "Agent"],
              agents={
                  "code-analyzer": AgentDefinition(
                      description="Static code analysis and architecture review",
                      prompt="""You are a code architecture analyst. Analyze code structure,
  identify patterns, and suggest improvements without making changes.""",
                      # Read-only tools: no Edit, Write, or Bash access
                      tools=["Read", "Grep", "Glob"],
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
    prompt: "Analyze the architecture of this codebase",
    options: {
      allowedTools: ["Read", "Grep", "Glob", "Agent"],
      agents: {
        "code-analyzer": {
          description: "Static code analysis and architecture review",
          prompt: `You are a code architecture analyst. Analyze code structure,
  identify patterns, and suggest improvements without making changes.`,
          // Read-only tools: no Edit, Write, or Bash access
          tools: ["Read", "Grep", "Glob"]
        }
      }
    }
  })) {
    if ("result" in message) console.log(message.result);
  }
  ```
</CodeGroup>

<h3 id="common-tool-combinations">
  Kombinasi alat umum
</h3>

| Kasus penggunaan   | Alat                                    | Deskripsi                                                         |
| :----------------- | :-------------------------------------- | :---------------------------------------------------------------- |
| Analisis baca-saja | `Read`, `Grep`, `Glob`                  | Dapat memeriksa kode tetapi tidak memodifikasi atau menjalankan   |
| Eksekusi pengujian | `Bash`, `Read`, `Grep`                  | Dapat menjalankan perintah dan menganalisis output                |
| Modifikasi kode    | `Read`, `Edit`, `Write`, `Grep`, `Glob` | Akses baca/tulis penuh tanpa eksekusi perintah                    |
| Akses penuh        | Semua alat                              | Mewarisi alat yang tersedia untuk subagen (abaikan field `tools`) |

<h2 id="cap-subagent-depth-concurrency-and-spend">
  Batasi kedalaman, konkurensi, dan pengeluaran subagen
</h2>

<Note>
  Bagian ini menjelaskan TypeScript SDK v0.3.219 dan Python SDK v0.2.127 dan yang lebih baru, rilis yang menggabungkan Claude Code v2.1.219 atau yang lebih baru. Pada rilis sebelumnya, beberapa batas ini hilang atau default berbeda, jadi tingkatkan sebelum Anda mengandalkannya untuk membatasi jalankan. [Referensi variabel lingkungan](/docs/id/env-vars) dan [giliran dan anggaran](/docs/id/agent-sdk/agent-loop#turns-and-budget) mencatat versi Claude Code yang menambahkan setiap variabel dan penegakan batas pengeluaran subagen.
</Note>

Claude memutuskan sendiri kapan akan menspawn subagen dan berapa banyak yang akan dispawn. Setiap subagen membuat permintaan API-nya sendiri, yang dihitung terhadap `total_cost_usd` kueri, dan subagen dapat menspawn subagen mereka sendiri, jadi satu prompt dapat berkembang menjadi pohon agen.

Anda dapat membatasi pertumbuhan itu dengan tiga cara: seberapa dalam subagen bersarang, berapa banyak yang berjalan sekaligus, dan berapa banyak seluruh kueri menghabiskan. Atur batas kedalaman dan konkurensi sebagai variabel lingkungan melalui opsi [`env`](/docs/id/agent-sdk/typescript#options), dan batas pengeluaran sebagai opsi kueri:

| Batas       | Atur dengan                                              | Default                                                                                                        | Apa yang Claude Code lakukan pada batas                                                                                                                                                                                                                                                                                                                                |
| :---------- | :------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Kedalaman   | [`CLAUDE_CODE_MAX_SUBAGENT_SPAWN_DEPTH`](/docs/id/env-vars)   | `3` lapisan subagen di bawah agen utama Anda. `1` menghentikan subagen Anda dari menspawn milik mereka sendiri | Meninggalkan subagen di lapisan bawah tidak dapat menspawn, jadi ia melakukan pekerjaan yang didelegasikan sendiri. Lihat [subagen bersarang](/docs/id/sub-agents#let-subagents-spawn-their-own-subagents)                                                                                                                                                                  |
| Konkurensi  | [`CLAUDE_CODE_MAX_CONCURRENT_SUBAGENTS`](/docs/id/env-vars)   | `20` subagen berjalan sekaligus, menghitung setiap subagen yang Claude spawnkan dengan alat Agent              | Menolak untuk menspawn subagen lain, mengembalikan `Concurrent subagent limit reached`, sampai jumlah yang berjalan turun di bawah batas. Sesi dengan [ultracode](/docs/id/model-config#adjust-effort-level) aktif tidak pernah ditolak. Lihat [batas subagen bersamaan](/docs/id/sub-agents#concurrent-subagent-limit)                                                          |
| Pengeluaran | `maxBudgetUsd` di TypeScript, `max_budget_usd` di Python | Tidak ada batas. Menghitung pengeluaran panggilan sendiri, permintaan subagen disertakan                       | Menegakkan batas dengan tiga cara: menolak untuk menspawn lebih banyak subagen, mengembalikan `Budget limit reached`, menghentikan subagen latar belakang yang masih berjalan, dan mengakhiri kueri dengan subtipe hasil `error_max_budget_usd`. Untuk cara batas berperilaku di seluruh sesi, lihat [giliran dan anggaran](/docs/id/agent-sdk/agent-loop#turns-and-budget) |

Kedua SDK memperlakukan opsi `env` secara berbeda: SDK TypeScript mengganti lingkungan subprocess dengannya, jadi sebarkan `process.env` ke dalamnya untuk menjaga variabel seperti `PATH`, sementara SDK Python menggabungkannya ke dalam lingkungan yang diwariskan. Contoh ini mematikan penyarangan, memungkinkan paling banyak lima subagen sekaligus, dan menghentikan kueri setelah pengeluaran yang diperkirakan mencapai \$5:

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage


  async def main():
      try:
          async for message in query(
              prompt="Audit every service in this repo for unhandled promise rejections",
              options=ClaudeAgentOptions(
                  allowed_tools=["Read", "Grep", "Glob", "Agent"],
                  # env is merged on top of the inherited environment
                  env={
                      "CLAUDE_CODE_MAX_SUBAGENT_SPAWN_DEPTH": "1",
                      "CLAUDE_CODE_MAX_CONCURRENT_SUBAGENTS": "5",
                  },
                  max_budget_usd=5.0,
              ),
          ):
              if isinstance(message, ResultMessage):
                  print(f"{message.subtype}: ${message.total_cost_usd}")
      except Exception as error:
          # A single-shot query() raises after yielding an error result,
          # so the budget-capped result has already been printed above.
          print(f"Session ended with an error: {error}")


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  try {
    for await (const message of query({
      prompt: "Audit every service in this repo for unhandled promise rejections",
      options: {
        allowedTools: ["Read", "Grep", "Glob", "Agent"],
        // env replaces the subprocess environment, so spread process.env to keep PATH
        env: {
          ...process.env,
          CLAUDE_CODE_MAX_SUBAGENT_SPAWN_DEPTH: "1",
          CLAUDE_CODE_MAX_CONCURRENT_SUBAGENTS: "5",
        },
        maxBudgetUsd: 5,
      },
    })) {
      if (message.type === "result") {
        console.log(`${message.subtype}: $${message.total_cost_usd}`);
      }
    }
  } catch (error) {
    // A single-shot query() throws after yielding an error result,
    // so the budget-capped result has already been logged above.
    console.error(`Session ended with an error: ${error}`);
  }
  ```
</CodeGroup>

Apa yang Anda lihat tergantung pada batas mana, jika ada, yang dicapai kueri:

* **Di bawah batas pengeluaran**: Anda melihat `success` dan biaya yang diperkirakan.
* **Pada batas pengeluaran**: Anda melihat `error_max_budget_usd` dengan biaya pada atau di atas `5`, dan kemudian penanganan kesalahan Anda berjalan.
* **Pada batas konkurensi**: Anda melihat blok `tool_result` dalam aliran pesan yang membawa `Concurrent subagent limit reached`. Claude menerima blok yang sama sebagai hasil alat Agent.

<h3 id="run-opus-5-with-subagents">
  Jalankan Opus 5 dengan subagen
</h3>

Claude Opus 5 mendelegasikan ke subagen lebih mudah daripada model sebelumnya, jadi [batas kedalaman, konkurensi, dan pengeluaran](#cap-subagent-depth-concurrency-and-spend) paling penting pada kueri yang menjalankan Opus 5. [Panduan prompting Opus 5](https://platform.claude.com/docs/en/build-with-claude/prompt-engineering/prompting-claude-opus-5#controlling-subagent-spawning) memiliki instruksi delegasi yang dapat Anda tambahkan ke prompt apa pun. Apakah Claude Code menambahkan instruksi sendiri tergantung pada [prompt sistem](/docs/id/agent-sdk/modifying-system-prompts#how-system-prompts-work) mana yang Anda gunakan:

* **Preset `claude_code`**: ketika modelnya adalah Opus 5, Claude Code menambahkan baris ke prompt sistemnya yang memberi tahu Claude untuk tidak memanggil alat Agent kecuali diminta. Alat Agent tetap tersedia.
* **Prompt kustom, atau tidak ada `systemPrompt`**: Claude Code tidak membangun prompt sistemnya, jadi baris itu tidak ada. Tambahkan instruksi delegasi dari panduan prompting ke prompt Anda sendiri.

Setiap instruksi hanya mengarahkan Claude, jadi atur batasnya juga. Claude Code menegakkannya bagaimanapun Claude memutuskan untuk mendelegasikan.

<h2 id="scale-up-with-dynamic-workflows">
  Skalakan dengan alur kerja dinamis
</h2>

Subagents bekerja dengan baik untuk beberapa tugas yang didelegasikan per putaran. Untuk menjalankan yang mengoordinasikan puluhan hingga ratusan agen, gunakan alat `Workflow`, yang memindahkan orkestrasi ke dalam skrip yang dijalankan runtime di luar konteks percakapan. Lihat [alur kerja dinamis](/docs/id/workflows) untuk cara alur kerja berbeda dari delegasi subagent putaran demi putaran.

Alat `Workflow` tersedia dalam TypeScript Agent SDK v0.3.149 dan yang lebih baru. Sertakan `Workflow` dalam `allowedTools` untuk auto-approve jalankan alur kerja. Skema input dan output alat tercantum dalam [referensi TypeScript](/docs/id/agent-sdk/typescript#workflow).

<h2 id="troubleshooting">
  Troubleshooting
</h2>

<h3 id="claude-not-delegating-to-subagents">
  Claude tidak mendelegasikan ke subagents
</h3>

Jika Claude menyelesaikan tugas secara langsung daripada mendelegasikan ke subagent Anda:

* **Gunakan prompting eksplisit**: sebutkan subagent berdasarkan nama dalam prompt Anda, misalnya "Gunakan agen code-reviewer untuk memeriksa modul autentikasi"
* **Tulis deskripsi yang jelas**: jelaskan dengan tepat kapan menggunakan subagent sehingga Claude dapat mencocokkan tugas dengan tepat

<h3 id="filesystem-based-agents-not-loading">
  Agen berbasis filesystem tidak dimuat
</h3>

Claude Code memantau `~/.claude/agents/` dan `.claude/agents/` dan mengambil file agen baru atau yang telah diedit dalam beberapa detik, tanpa perlu restart. Jika definisi tidak pernah muncul, kerjakan melalui penyebab-penyebab ini:

* **Direktori `agents` baru**: pemantau hanya mencakup direktori yang ada saat sesi dimulai, jadi file pertama di direktori baru memerlukan restart sesi. Ini adalah penyebab paling umum.
* **Frontmatter tidak valid atau `name` duplikat**: periksa YAML file, dan apakah agen yang ada sudah menggunakan `name` tersebut.
* **`--disable-slash-commands`**: sesi yang dimulai dengan flag ini tidak memantau direktori-direktori ini dan selalu memerlukan restart untuk memuat file baru.
* **File di bawah direktori yang ditambahkan**: Claude Code memuat `.claude/agents/` dari direktori yang ditambahkan dengan opsi `add_dirs` (Python) atau `additionalDirectories` (TypeScript), atau CLI `--add-dir` atau `/add-dir`, tetapi tidak memantaunya, jadi file baru atau yang telah diedit di sana memerlukan restart sesi.
* **Agen programatik dengan nama yang sama**: `agents` yang dilewatkan ke `query()` menimpa agen filesystem dengan nama yang sama.

Untuk format file, lihat [cara menulis file subagent](/docs/id/sub-agents#write-subagent-files).

<h2 id="related-documentation">
  Dokumentasi terkait
</h2>

* [Subagents Claude Code](/docs/id/sub-agents): dokumentasi subagent komprehensif termasuk definisi berbasis sistem file
* [Alur kerja dinamis](/docs/id/workflows): orkestrasi banyak subagents dari skrip untuk pekerjaan yang terlalu besar untuk satu percakapan
* [Ikhtisar SDK](/docs/id/agent-sdk/overview): memulai dengan Claude Agent SDK
