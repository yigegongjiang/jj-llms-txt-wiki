> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Subagents dalam SDK

> Tentukan dan panggil subagents untuk mengisolasi konteks, menjalankan tugas secara paralel, dan menerapkan instruksi khusus dalam aplikasi Claude Agent SDK Anda.

Subagents adalah instans agen terpisah yang dapat dihasilkan oleh agen utama Anda untuk menangani subtask yang terfokus.
Gunakan subagents untuk mengisolasi konteks, menjalankan beberapa analisis secara paralel, dan menerapkan instruksi khusus tanpa menambah prompt agen utama.

Panduan ini menjelaskan cara mendefinisikan dan menggunakan subagents dalam SDK menggunakan parameter `agents`.

<h2 id="overview">
  Ikhtisar
</h2>

Anda dapat membuat subagents dengan tiga cara:

* **Secara programatis**: gunakan parameter `agents` dalam opsi `query()` Anda. Lihat referensi [TypeScript](/docs/id/agent-sdk/typescript#agentdefinition) dan [Python](/docs/id/agent-sdk/python#agentdefinition)
* **Berbasis sistem file**: tentukan agents sebagai file markdown di direktori `.claude/agents/`. Lihat [mendefinisikan subagents sebagai file](/docs/id/sub-agents)
* **Tujuan umum bawaan**: Claude dapat memanggil subagent `general-purpose` bawaan kapan saja melalui alat Agent tanpa Anda mendefinisikan apa pun

Panduan ini berfokus pada pendekatan programatis, yang direkomendasikan untuk aplikasi SDK.

Ketika Anda mendefinisikan subagents, Claude menentukan apakah akan memanggil mereka berdasarkan field `description` setiap subagent. Tulis deskripsi yang jelas yang menjelaskan kapan menggunakan subagent, dan Claude akan secara otomatis mendelegasikan tugas yang sesuai. Anda juga dapat secara eksplisit meminta subagent berdasarkan nama dalam prompt Anda, misalnya "Gunakan agen code-reviewer untuk...".

<h2 id="benefits-of-using-subagents">
  Manfaat menggunakan subagents
</h2>

<h3 id="context-isolation">
  Isolasi konteks
</h3>

Setiap subagent berjalan dalam percakapan segar mereka sendiri. Panggilan alat perantara dan hasil tetap berada di dalam subagent; hanya pesan finalnya yang kembali ke parent. Lihat [Apa yang diwarisi subagents](#what-subagents-inherit) untuk mengetahui dengan tepat apa yang ada dalam konteks subagent.

**Contoh:** subagent `research-assistant` dapat menjelajahi puluhan file tanpa konten apa pun yang terakumulasi dalam percakapan utama. Parent menerima ringkasan ringkas, bukan setiap file yang dibaca subagent.

<h3 id="parallelization">
  Paralelisasi
</h3>

Beberapa subagents dapat berjalan secara bersamaan, sehingga subtask independen selesai dalam waktu yang paling lambat daripada jumlah dari semuanya.

**Contoh:** selama tinjauan kode, Anda dapat menjalankan subagents `style-checker`, `security-scanner`, dan `test-coverage` secara bersamaan, bukan secara berurutan.

<h3 id="specialized-instructions-and-knowledge">
  Instruksi dan pengetahuan khusus
</h3>

Setiap subagent dapat memiliki prompt sistem yang disesuaikan dengan keahlian spesifik, praktik terbaik, dan batasan.

**Contoh:** subagent `database-migration` dapat memiliki pengetahuan terperinci tentang praktik terbaik SQL, strategi rollback, dan pemeriksaan integritas data yang akan menjadi kebisingan yang tidak perlu dalam instruksi agen utama.

<h3 id="tool-restrictions">
  Pembatasan alat
</h3>

Subagents dapat dibatasi pada alat tertentu, mengurangi risiko tindakan yang tidak diinginkan.

**Contoh:** subagent `doc-reviewer` mungkin hanya memiliki akses ke alat Read dan Grep, memastikan dapat menganalisis tetapi tidak pernah secara tidak sengaja memodifikasi file dokumentasi Anda.

<h2 id="create-subagents">
  Membuat subagents
</h2>

<h3 id="programmatic-definition-recommended">
  Definisi programatis (direkomendasikan)
</h3>

Tentukan subagents langsung dalam kode Anda menggunakan parameter `agents`. Claude memanggil subagents melalui alat `Agent`, jadi sertakan `Agent` dalam `allowedTools` untuk auto-approve invokasi subagent tanpa prompt izin.

Sebagian besar contoh di halaman ini hanya mencetak hasil akhir. Untuk mengonfirmasi bahwa Claude mendelegasikan ke subagent daripada menjawab secara langsung, lihat [Mendeteksi invokasi subagent](#detect-subagent-invocation).

Contoh ini membuat dua subagents: peninjau kode dengan akses read-only dan runner test yang dapat menjalankan perintah.

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, AgentDefinition


  async def main():
      async for message in query(
          prompt="Review the authentication module for security issues",
          options=ClaudeAgentOptions(
              # Auto-approve these tools, including Agent for subagent invocation
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
      // Auto-approve these tools, including Agent for subagent invocation
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

| Field             | Type                                                        | Required | Description                                                                                                                                                                                                                             |
| :---------------- | :---------------------------------------------------------- | :------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `description`     | `string`                                                    | Yes      | Deskripsi bahasa alami tentang kapan menggunakan agen ini                                                                                                                                                                               |
| `prompt`          | `string`                                                    | Yes      | Prompt sistem agen yang mendefinisikan peran dan perilakunya                                                                                                                                                                            |
| `tools`           | `string[]`                                                  | No       | Array nama alat yang diizinkan. Jika dihilangkan, mewarisi semua alat                                                                                                                                                                   |
| `disallowedTools` | `string[]`                                                  | No       | Array nama alat yang akan dihapus dari set alat agen. Pola tingkat server MCP juga diterima: `mcp__server` atau `mcp__server__*` menghapus setiap alat dari server tersebut, dan `mcp__*` menghapus setiap alat MCP dari server apa pun |
| `model`           | `string`                                                    | No       | Penggantian model untuk agen ini. Menerima alias seperti `'fable'`, `'opus'`, `'sonnet'`, `'haiku'`, `'inherit'`, atau ID model lengkap. Default ke model utama jika dihilangkan                                                        |
| `skills`          | `string[]`                                                  | No       | Daftar nama skill untuk dimuat sebelumnya ke dalam konteks agen saat startup. Skill yang tidak terdaftar tetap dapat dipanggil melalui alat Skill                                                                                       |
| `memory`          | `'user' \| 'project' \| 'local'`                            | No       | Sumber memori untuk agen ini                                                                                                                                                                                                            |
| `mcpServers`      | `(string \| object)[]`                                      | No       | Server MCP yang tersedia untuk agen ini, berdasarkan nama atau konfigurasi inline                                                                                                                                                       |
| `initialPrompt`   | `string`                                                    | No       | Auto-submitted sebagai putaran pengguna pertama saat agen ini berjalan sebagai agen thread utama. Diabaikan saat agen dipanggil sebagai subagent                                                                                        |
| `maxTurns`        | `number`                                                    | No       | Jumlah maksimum putaran agentic sebelum agen berhenti                                                                                                                                                                                   |
| `background`      | `boolean`                                                   | No       | Jalankan agen ini sebagai tugas latar belakang non-blocking saat dipanggil                                                                                                                                                              |
| `effort`          | `'low' \| 'medium' \| 'high' \| 'xhigh' \| 'max' \| number` | No       | Tingkat upaya penalaran untuk agen ini                                                                                                                                                                                                  |
| `permissionMode`  | `PermissionMode`                                            | No       | Mode izin untuk eksekusi alat dalam agen ini                                                                                                                                                                                            |

Dalam Python SDK, nama field multi-kata seperti `disallowedTools` dan `mcpServers` mempertahankan ejaan camelCase mereka untuk mencocokkan format wire daripada mengikuti konvensi snake\_case Python. Lihat referensi [`AgentDefinition`](/docs/id/agent-sdk/python#agentdefinition) untuk detail.

Dua perilaku subagent berubah dalam Claude Code v2.1.198:

* Subagents berjalan di latar belakang secara default. Panggilan alat Agent yang menghilangkan input [`run_in_background`](/docs/id/agent-sdk/typescript) meluncurkan subagent latar belakang, dan Claude menetapkan `run_in_background: false` ketika memerlukan hasil sebelum melanjutkan. Sebelum v2.1.198, menghilangkan `run_in_background` menjalankan subagent secara sinkron. Atur field `background` ke `true` untuk memaksa eksekusi latar belakang untuk agen tertentu terlepas dari apa yang diminta Claude.
* Subagent mewarisi konfigurasi extended thinking sesi utama. Pada versi sebelumnya, extended thinking dinonaktifkan di dalam subagents terlepas dari pengaturan sesi utama.

<Note>
  Sejak Claude Code v2.1.172, subagents dapat menghasilkan subagents mereka sendiri. Subagent lima level di bawah agen utama tidak dapat menghasilkan subagents lebih lanjut, terlepas dari apakah itu berjalan di foreground atau background. Untuk mencegah subagent menghasilkan yang lain, hilangkan `Agent` dari array `tools` atau tambahkan ke `disallowedTools`. Lihat [subagents bersarang](/docs/id/sub-agents#spawn-nested-subagents) untuk aturan kedalaman lengkap.
</Note>

<h3 id="filesystem-based-definition-alternative">
  Definisi berbasis sistem file (alternatif)
</h3>

Anda juga dapat mendefinisikan subagents sebagai file markdown di direktori `.claude/agents/`. Lihat [dokumentasi subagents Claude Code](/docs/id/sub-agents) untuk detail tentang pendekatan ini. Agen yang didefinisikan secara programatis memiliki prioritas lebih tinggi daripada agen berbasis sistem file dengan nama yang sama.

<Note>
  Bahkan tanpa mendefinisikan subagents khusus, Claude dapat menghasilkan subagent `general-purpose` bawaan. Ini berguna untuk mendelegasikan tugas penelitian atau eksplorasi tanpa membuat agen khusus. Sertakan `Agent` dalam `allowedTools` sehingga invokasi ini auto-approve tanpa prompt izin.
</Note>

<h2 id="what-subagents-inherit">
  Apa yang diwarisi subagents
</h2>

Jendela konteks subagent dimulai segar, tanpa percakapan parent, tetapi tidak kosong. Satu-satunya konten yang Anda teruskan dari parent ke subagent adalah string prompt alat Agent, jadi sertakan jalur file, pesan kesalahan, atau keputusan apa pun yang dibutuhkan subagent langsung dalam prompt itu.

Subagent yang memiliki alat [`SendMessage`](/docs/id/tools-reference) dimulai dengan daftar agen bernama lainnya yang berjalan dalam sesi, sehingga mengetahui nama mana yang dapat dikirim pesan. Claude Code menambahkan daftar ke giliran pertama subagent secara otomatis. [Fork](/docs/id/sub-agents#fork-the-current-conversation) tidak mendapatkan daftar karena mewarisi percakapan parent sebagai gantinya. Daftar memerlukan Claude Code v2.1.206 atau lebih baru.

| Subagent menerima                                                                                                                         | Subagent tidak menerima                                                               |
| :---------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------ |
| Prompt sistem sendiri (`AgentDefinition.prompt`) dan prompt alat Agent                                                                    | Riwayat percakapan parent atau hasil alat                                             |
| Project CLAUDE.md (dimuat melalui [`settingSources`](/docs/id/agent-sdk/claude-code-features#control-filesystem-settings-with-settingsources)) | Konten skill yang dimuat sebelumnya, kecuali terdaftar dalam `AgentDefinition.skills` |
| Definisi alat (diwarisi dari parent, atau subset dalam `tools`)                                                                           | Prompt sistem parent                                                                  |

<Note>
  Parent menerima pesan final subagent verbatim sebagai hasil alat Agent, tetapi dapat merangkumnya dalam respons sendiri. Untuk mempertahankan output subagent verbatim dalam respons yang menghadap pengguna, sertakan instruksi untuk melakukannya dalam prompt atau opsi `systemPrompt` yang Anda berikan ke panggilan `query()` utama.
</Note>

Kesalahan API yang mengakhiri subagent lebih awal, seperti batas laju, tidak pernah disampaikan sebagai hasilnya. Jika batas laju, kelebihan beban, atau kesalahan server memotong subagent foreground yang sudah menghasilkan output teks, alat Agent mengembalikan output parsial itu dengan catatan bahwa subagent tidak selesai. Subagent yang tidak menghasilkan apa pun, atau yang output-nya hanya berupa panggilan alat tanpa teks, gagal dengan pesan kesalahan, `Agent terminated early due to an API error`, diikuti oleh detail kesalahan. Lihat [API errors in subagents](/docs/id/sub-agents#api-errors-in-subagents) untuk perilaku foreground dan background.

Penanganan output parsial ini memerlukan Claude Code v2.1.199 atau lebih baru. Dalam v2.1.199, batas laju, kelebihan beban, atau kesalahan server membiarkan bentuk tool-calls-only dengan hasil parsial kosong yang hanya berisi catatan cutoff.

<h2 id="invoke-subagents">
  Memanggil subagents
</h2>

<h3 id="automatic-invocation">
  Invokasi otomatis
</h3>

Claude secara otomatis memutuskan kapan akan memanggil subagents berdasarkan tugas dan `description` setiap subagent. Misalnya, jika Anda mendefinisikan subagent `performance-optimizer` dengan deskripsi "Performance optimization specialist for query tuning", Claude akan memanggilnya ketika prompt Anda menyebutkan optimasi query.

Tulis deskripsi yang jelas dan spesifik sehingga Claude dapat mencocokkan tugas ke subagent yang tepat.

<h3 id="explicit-invocation">
  Invokasi eksplisit
</h3>

Untuk menjamin Claude menggunakan subagent tertentu, sebutkan berdasarkan nama dalam prompt Anda:

```text theme={null}
"Use the code-reviewer agent to check the authentication module"
```

Ini melewati pencocokan otomatis dan langsung memanggil subagent bernama.

<h3 id="dynamic-agent-configuration">
  Konfigurasi agen dinamis
</h3>

Anda dapat membuat definisi agen secara dinamis berdasarkan kondisi runtime. Contoh ini membuat peninjau keamanan dengan tingkat ketat yang berbeda, menggunakan model yang lebih kuat untuk tinjauan ketat.

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
  Mendeteksi invokasi subagent
</h2>

Claude menginvokasi subagents melalui alat Agent. Untuk mendeteksi ketika subagent diinvokasi, periksa blok `tool_use` di mana `name` adalah `"Agent"`. Pesan dari dalam konteks subagent mencakup field `parent_tool_use_id`.

<Note>
  Nama alat diubah dari `"Task"` menjadi `"Agent"` dalam Claude Code v2.1.63. Rilis SDK saat ini memancarkan `"Agent"` dalam blok `tool_use` tetapi masih menggunakan `"Task"` dalam daftar alat `system:init` dan dalam `result.permission_denials[].tool_name`. Memeriksa kedua nilai dalam `block.name` memastikan kompatibilitas di seluruh versi SDK.
</Note>

Struktur pesan berbeda antara SDK. Dalam Python, blok konten diakses langsung melalui `message.content`. Dalam TypeScript, `SDKAssistantMessage` membungkus pesan Claude API, jadi konten diakses melalui `message.message.content`.

Contoh ini mengulangi pesan yang dialirkan, mencatat ketika subagent dipanggil dan ketika pesan berikutnya berasal dari dalam konteks eksekusi subagent itu.

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

Anda dapat melanjutkan subagent untuk terus dari mana ia berhenti daripada memulai dari awal. Subagent yang dilanjutkan mempertahankan riwayat percakapan lengkapnya, termasuk semua panggilan alat sebelumnya, hasil, dan penalaran.

Ketika subagent selesai, hasil alat Agent mencakup blok teks yang berisi `agentId: <id>`. Agen bawaan [`Explore` dan `Plan`](/docs/id/sub-agents#built-in-subagents) adalah one-shot dan tidak mengembalikan `agentId`, jadi gunakan agen khusus atau `general-purpose` ketika Anda perlu melanjutkan. Untuk melanjutkan subagent secara programatis:

1. **Tangkap ID sesi**: ekstrak `session_id` dari pesan selama query pertama
2. **Ekstrak ID agen**: parse `agentId` dari teks hasil alat Agent
3. **Lanjutkan sesi**: berikan `resume: sessionId` dalam opsi query kedua, dan sertakan ID agen dalam prompt Anda

<Note>
  Anda harus melanjutkan sesi yang sama untuk mengakses transkrip subagent. Setiap panggilan `query()` memulai sesi baru secara default, jadi berikan `resume: sessionId` untuk melanjutkan dalam sesi yang sama.

  Ketika menggunakan agen khusus, berikan definisi agen yang sama dalam parameter `agents` untuk kedua query.
</Note>

Contoh di bawah mendefinisikan agen khusus `endpoint-finder`. Query pertama menjalankannya dan menangkap ID sesi dan ID agen dari hasil alat Agent, kemudian query kedua melanjutkan sesi untuk mengajukan pertanyaan tindak lanjut yang memerlukan konteks dari analisis pertama.

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

Transkrip subagent bertahan secara independen dari percakapan utama:

* **Pemadatan percakapan utama**: ketika percakapan utama dipadatkan, transkrip subagent tidak terpengaruh. Mereka disimpan dalam file terpisah.
* **Persistensi sesi**: transkrip subagent bertahan dalam sesi mereka. Anda dapat melanjutkan subagent setelah memulai ulang Claude Code dengan melanjutkan sesi yang sama.
* **Pembersihan otomatis**: transkrip dibersihkan berdasarkan pengaturan `cleanupPeriodDays`, yang secara default adalah 30 hari.

<h2 id="tool-restrictions-2">
  Pembatasan alat
</h2>

Subagents dapat memiliki akses alat terbatas melalui field `tools`:

* **Hilangkan field**: agen mewarisi semua alat yang tersedia (default)
* **Tentukan alat**: agen hanya dapat menggunakan alat yang terdaftar

Contoh ini membuat agen analisis read-only yang dapat memeriksa kode tetapi tidak dapat memodifikasi file atau menjalankan perintah.

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

| Kasus penggunaan   | Alat                                    | Deskripsi                                                       |
| :----------------- | :-------------------------------------- | :-------------------------------------------------------------- |
| Analisis read-only | `Read`, `Grep`, `Glob`                  | Dapat memeriksa kode tetapi tidak memodifikasi atau menjalankan |
| Eksekusi test      | `Bash`, `Read`, `Grep`                  | Dapat menjalankan perintah dan menganalisis output              |
| Modifikasi kode    | `Read`, `Edit`, `Write`, `Grep`, `Glob` | Akses read/write penuh tanpa eksekusi perintah                  |
| Akses penuh        | Semua alat                              | Mewarisi semua alat dari parent (hilangkan field `tools`)       |

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

* **Periksa bahwa invokasi Agent disetujui**: sertakan `Agent` dalam `allowedTools` untuk auto-approve panggilan subagent. Tanpa itu, invokasi Agent jatuh ke callback `canUseTool` Anda atau, dalam mode `dontAsk`, ditolak
* **Gunakan prompting eksplisit**: sebutkan subagent berdasarkan nama dalam prompt Anda, misalnya "Gunakan agen code-reviewer untuk..."
* **Tulis deskripsi yang jelas**: jelaskan dengan tepat kapan menggunakan subagent sehingga Claude dapat mencocokkan tugas dengan tepat

<h3 id="filesystem-based-agents-not-loading">
  Agen berbasis filesystem tidak dimuat
</h3>

Claude Code memantau `~/.claude/agents/` dan `.claude/agents/` dan mengambil file agen baru atau yang telah diedit dalam beberapa detik, tanpa perlu restart. Jika definisi tidak pernah muncul, kerjakan melalui penyebab-penyebab ini:

* **Direktori `agents` baru**: pemantau hanya mencakup direktori yang ada saat sesi dimulai, jadi file pertama di direktori baru memerlukan restart sesi. Ini adalah penyebab paling umum.
* **Frontmatter tidak valid atau `name` duplikat**: periksa YAML file, dan apakah agen yang ada sudah menggunakan `name` tersebut.
* **`--disable-slash-commands`**: sesi yang dimulai dengan flag ini tidak memantau direktori-direktori ini dan selalu memerlukan restart untuk memuat file baru.
* **Agen programatik dengan nama yang sama**: `agents` yang dilewatkan ke `query()` menimpa agen filesystem dengan nama yang sama.

Untuk format file, lihat [cara menulis file subagent](/docs/id/sub-agents#write-subagent-files).

<h3 id="long-prompt-failures-on-windows">
  Kegagalan prompt panjang di Windows
</h3>

Di Windows, subagents dengan prompt yang sangat panjang mungkin gagal karena batasan panjang baris perintah sebesar 8191 karakter. Jaga prompt tetap ringkas atau gunakan agen berbasis filesystem untuk instruksi kompleks.

<h2 id="related-documentation">
  Dokumentasi terkait
</h2>

* [Subagents Claude Code](/docs/id/sub-agents): dokumentasi subagent komprehensif termasuk definisi berbasis sistem file
* [Alur kerja dinamis](/docs/id/workflows): orkestrasi banyak subagents dari skrip untuk pekerjaan yang terlalu besar untuk satu percakapan
* [Ikhtisar SDK](/docs/id/agent-sdk/overview): memulai dengan Claude Agent SDK
