> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Cara kerja agent loop

> Pahami lifecycle pesan, eksekusi tool, context window, dan arsitektur yang menggerakkan agent SDK Anda.

Agent SDK memungkinkan Anda untuk menyematkan autonomous agent loop Claude Code dalam aplikasi Anda sendiri. SDK adalah paket standalone yang memberikan Anda kontrol programatik atas tools, permissions, cost limits, dan output. Anda tidak perlu menginstal Claude Code CLI untuk menggunakannya.

Ketika Anda memulai agent, SDK menjalankan [execution loop yang sama yang menggerakkan Claude Code](/docs/id/how-claude-code-works#the-agentic-loop): Claude mengevaluasi prompt Anda, memanggil tools untuk mengambil tindakan, menerima hasilnya, dan mengulangi sampai tugas selesai. Halaman ini menjelaskan apa yang terjadi di dalam loop tersebut sehingga Anda dapat membangun, debug, dan mengoptimalkan agent Anda secara efektif.

<h2 id="the-loop-at-a-glance">
  Loop sekilas
</h2>

Setiap sesi agent mengikuti siklus yang sama:

<img src="https://mintcdn.com/claude-code/ikqp3_70mqIahteV/images/agent-loop-diagram.svg?fit=max&auto=format&n=ikqp3_70mqIahteV&q=85&s=1c6e8f28d80dba14a7287419656f1237" alt="Diagram agent loop: prompt Anda memasuki agentic loop, di mana Claude mengevaluasi dan baik meminta tool calls, yang hasilnya umpan balik ke evaluasi lain, atau mengembalikan jawaban final" width="720" height="212" data-path="images/agent-loop-diagram.svg" />

1. **Terima prompt.** Claude menerima prompt Anda, bersama dengan system prompt, tool definitions, dan conversation history. SDK menghasilkan [`SystemMessage`](#message-types) dengan subtype `"init"` yang berisi session metadata.
2. **Evaluasi dan respons.** Claude mengevaluasi state saat ini dan menentukan cara melanjutkan. Ini dapat merespons dengan teks, meminta satu atau lebih tool calls, atau keduanya. SDK menghasilkan [`AssistantMessage`](#message-types) yang berisi teks dan permintaan tool call apa pun.
3. **Eksekusi tools.** SDK menjalankan setiap tool yang diminta dan mengumpulkan hasilnya. Setiap set hasil tool umpan balik ke Claude untuk keputusan berikutnya. Anda dapat menggunakan [hooks](/docs/id/agent-sdk/hooks) untuk mengintersepsi, memodifikasi, atau memblokir tool calls sebelum dijalankan.
4. **Ulangi.** Langkah 2 dan 3 berulang sebagai siklus. Setiap siklus penuh adalah satu turn. Claude terus memanggil tools dan memproses hasil sampai menghasilkan respons tanpa tool calls.
5. **Kembalikan hasil.** SDK menghasilkan [`AssistantMessage`](#message-types) final dengan respons teks (tanpa tool calls), diikuti oleh [`ResultMessage`](#message-types) dengan teks final, token usage, cost, dan session ID.

Pertanyaan cepat ("file apa yang ada di sini?") mungkin membutuhkan satu atau dua turn memanggil `Glob` dan merespons dengan hasilnya. Tugas kompleks ("refactor modul auth dan perbarui tests") dapat merantai puluhan tool calls di banyak turn, membaca file, mengedit kode, dan menjalankan tests, dengan Claude menyesuaikan pendekatannya berdasarkan setiap hasil.

<h2 id="turns-and-messages">
  Turns dan messages
</h2>

Turn adalah satu round trip di dalam loop: Claude menghasilkan output yang mencakup tool calls, SDK menjalankan tools tersebut, dan hasilnya umpan balik ke Claude secara otomatis. Ini terjadi tanpa menghasilkan kontrol kembali ke kode Anda. Turns berlanjut sampai Claude menghasilkan output tanpa tool calls, di mana titik loop berakhir dan hasil final dikirimkan.

Pertimbangkan seperti apa sesi penuh untuk prompt "Fix the failing tests in auth.ts".

Pertama, SDK mengirim prompt Anda ke Claude dan menghasilkan [`SystemMessage`](#message-types) dengan session metadata. Kemudian loop dimulai:

1. **Turn 1:** Claude memanggil `Bash` untuk menjalankan `npm test`. SDK menghasilkan [`AssistantMessage`](#message-types) dengan tool call, menjalankan perintah, kemudian menghasilkan [`UserMessage`](#message-types) dengan output (tiga kegagalan).
2. **Turn 2:** Claude memanggil `Read` pada `auth.ts` dan `auth.test.ts`. SDK mengembalikan konten file dan menghasilkan `AssistantMessage`.
3. **Turn 3:** Claude memanggil `Edit` untuk memperbaiki `auth.ts`, kemudian memanggil `Bash` untuk menjalankan kembali `npm test`. Ketiga tests lulus. SDK menghasilkan `AssistantMessage`.
4. **Turn final:** Claude menghasilkan respons hanya teks tanpa tool calls: "Fixed the auth bug, all three tests pass now." SDK menghasilkan `AssistantMessage` final dengan teks ini, kemudian [`ResultMessage`](#message-types) dengan teks yang sama ditambah cost dan usage.

Itu adalah empat turns: tiga dengan tool calls, satu respons hanya teks final.

Anda dapat membatasi loop dengan `max_turns` / `maxTurns`, yang menghitung tool-use turns saja. Misalnya, `max_turns=2` dalam loop di atas akan berhenti sebelum langkah edit. Anda juga dapat menggunakan `max_budget_usd` / `maxBudgetUsd` untuk membatasi turns berdasarkan threshold pengeluaran.

Tanpa batas, loop berjalan sampai Claude selesai sendiri, yang baik untuk tugas yang well-scoped tetapi dapat berjalan lama pada prompts open-ended ("improve this codebase"). Menetapkan budget adalah default yang baik untuk production agents. Lihat [Turns dan budget](#turns-and-budget) di bawah untuk referensi opsi.

<h2 id="message-types">
  Tipe Message
</h2>

Saat loop berjalan, SDK menghasilkan aliran messages. Setiap message membawa tipe yang memberi tahu Anda tahap loop mana yang berasal darinya. Lima tipe inti adalah:

* **`SystemMessage`:** session lifecycle events. Field `subtype` membedakannya:

  * `"init"`: session metadata untuk run. Ketika hook `SessionStart` atau `Setup` berjalan selama session startup, [hook lifecycle messages](/docs/id/agent-sdk/typescript#sdkhookstartedmessage) tiba sebelum message `init`
  * `"compact_boundary"`: menyala setelah [compaction](#automatic-compaction)
  * `"informational"`: plain-text status banners dari loop
  * `"worker_shutting_down"`: loop akan berakhir setelah turn saat ini karena host sedang keluar atau Remote Control terputus

  Di TypeScript, setiap subtype selain `"init"` adalah tipenya sendiri dalam union [`SDKMessage`](/docs/id/agent-sdk/typescript#sdkmessage) daripada subtype dari `SDKSystemMessage`.
* **`AssistantMessage`:** dipancarkan setelah setiap respons Claude, termasuk yang hanya teks final. Berisi text content blocks dan tool call blocks dari turn itu.
* **`UserMessage`:** dipancarkan setelah setiap eksekusi tool dengan tool result content yang dikirim kembali ke Claude. Juga dipancarkan untuk input pengguna apa pun yang Anda stream mid-loop.
* **`StreamEvent`:** hanya dipancarkan ketika partial messages diaktifkan. Berisi raw API streaming events (text deltas, tool input chunks). Lihat [Stream responses](/docs/id/agent-sdk/streaming-output).
* **`ResultMessage`:** menandai akhir dari agent loop. Berisi hasil teks final, token usage, cost, dan session ID. Periksa field `subtype` untuk menentukan apakah tugas berhasil atau mencapai batas. Sejumlah kecil trailing system events, seperti `prompt_suggestion`, dapat tiba setelahnya, jadi iterasi stream hingga selesai daripada break pada hasil. Lihat [Handle the result](#handle-the-result).

Lima tipe ini mencakup lifecycle agent loop penuh di kedua SDK. TypeScript SDK juga menghasilkan additional observability events (hook events, tool progress, rate limits, task notifications) yang memberikan detail ekstra tetapi tidak diperlukan untuk menjalankan loop. Lihat [Python message types reference](/docs/id/agent-sdk/python#message-types) dan [TypeScript message types reference](/docs/id/agent-sdk/typescript#message-types) untuk daftar lengkap.

<h3 id="handle-messages">
  Handle messages
</h3>

Messages mana yang Anda handle tergantung pada apa yang Anda bangun:

* **Final results only:** handle `ResultMessage` untuk mendapatkan output, cost, dan apakah tugas berhasil atau mencapai batas.
* **Progress updates:** handle `AssistantMessage` untuk melihat apa yang dilakukan Claude setiap turn, termasuk tools mana yang dipanggilnya.
* **Live streaming:** aktifkan partial messages (`include_partial_messages` di Python, `includePartialMessages` di TypeScript) untuk mendapatkan `StreamEvent` messages secara real time. Lihat [Stream responses in real-time](/docs/id/agent-sdk/streaming-output).

Cara Anda memeriksa message types tergantung pada SDK:

* **Python:** periksa message types dengan `isinstance()` terhadap classes yang diimpor dari `claude_agent_sdk` (misalnya, `isinstance(message, ResultMessage)`).
* **TypeScript:** periksa field string `type` (misalnya, `message.type === "result"`). `AssistantMessage` dan `UserMessage` membungkus raw API message dalam field `.message`, jadi content blocks berada di `message.message.content`, bukan `message.content`.

<Accordion title="Contoh: Periksa message types dan handle results">
  <CodeGroup>
    ```python Python theme={null}
    import asyncio
    from claude_agent_sdk import query, AssistantMessage, ResultMessage


    async def main():
        try:
            async for message in query(prompt="Summarize this project"):
                if isinstance(message, AssistantMessage):
                    print(f"Turn completed: {len(message.content)} content blocks")
                if isinstance(message, ResultMessage):
                    if message.subtype == "success":
                        print(message.result)
                    else:
                        print(f"Stopped: {message.subtype}")
        except Exception as error:
            # A single-shot query() raises after yielding an error result. If the
            # failure was an error result, the error subtype branches above have
            # already run; connection or process failures yield no result message.
            print(f"Session ended with an error: {error}")


    asyncio.run(main())
    ```

    ```typescript TypeScript theme={null}
    import { query } from "@anthropic-ai/claude-agent-sdk";

    try {
      for await (const message of query({ prompt: "Summarize this project" })) {
        if (message.type === "assistant") {
          console.log(`Turn completed: ${message.message.content.length} content blocks`);
        }
        if (message.type === "result") {
          if (message.subtype === "success") {
            console.log(message.result);
          } else {
            console.log(`Stopped: ${message.subtype}`);
          }
        }
      }
    } catch (error) {
      // A single-shot query() throws after yielding an error result. If the
      // failure was an error result, the error subtype branches above have
      // already run; connection or process failures yield no result message.
      console.log(`Session ended with an error: ${error}`);
    }
    ```
  </CodeGroup>
</Accordion>

<h2 id="tool-execution">
  Eksekusi tool
</h2>

Tools memberikan agent Anda kemampuan untuk mengambil tindakan. Tanpa tools, Claude hanya dapat merespons dengan teks. Dengan tools, Claude dapat membaca file, menjalankan perintah, mencari kode, dan berinteraksi dengan layanan eksternal.

<h3 id="built-in-tools">
  Built-in tools
</h3>

SDK mencakup tools yang sama yang menggerakkan Claude Code:

| Kategori            | Tools                                                           | Apa yang mereka lakukan                                                      |
| :------------------ | :-------------------------------------------------------------- | :--------------------------------------------------------------------------- |
| **File operations** | `Read`, `Edit`, `Write`                                         | Baca, modifikasi, dan buat file                                              |
| **Search**          | `Glob`, `Grep`                                                  | Temukan file berdasarkan pola, cari konten dengan regex                      |
| **Execution**       | `Bash`                                                          | Jalankan shell commands, scripts, git operations                             |
| **Web**             | `WebSearch`, `WebFetch`                                         | Cari web, ambil dan parse halaman                                            |
| **Discovery**       | `ToolSearch`                                                    | Temukan dan muat tools secara dinamis on-demand daripada preloading semuanya |
| **Orchestration**   | `Agent`, `Skill`, `AskUserQuestion`, `TaskCreate`, `TaskUpdate` | Spawn subagents, invoke skills, tanya pengguna, track tasks                  |

Melampaui built-in tools, Anda dapat:

* **Hubungkan layanan eksternal** dengan [MCP servers](/docs/id/agent-sdk/mcp) (databases, browsers, APIs)
* **Tentukan custom tools** dengan [custom tool handlers](/docs/id/agent-sdk/custom-tools)
* **Muat project skills** melalui [setting sources](/docs/id/agent-sdk/claude-code-features) untuk reusable workflows

<h3 id="tool-permissions">
  Tool permissions
</h3>

Claude menentukan tools mana yang akan dipanggil berdasarkan tugas, tetapi Anda mengontrol apakah panggilan tersebut diizinkan untuk dieksekusi. Anda dapat auto-approve tools spesifik, memblokir yang lain sepenuhnya, atau memerlukan approval untuk semuanya. Tiga opsi bekerja bersama untuk menentukan apa yang berjalan:

* **`allowed_tools` / `allowedTools`** auto-approves tools yang terdaftar. Agent read-only dengan `["Read", "Glob", "Grep"]` dalam daftar allowed tools-nya menjalankan tools tersebut tanpa prompting. Tools yang tidak terdaftar masih tersedia tetapi memerlukan permission.
* **`disallowed_tools` / `disallowedTools`** memblokir tools yang terdaftar, terlepas dari pengaturan lainnya. Lihat [Permissions](/docs/id/agent-sdk/permissions) untuk urutan aturan yang diperiksa sebelum tool berjalan.
* **`permission_mode` / `permissionMode`** mengontrol apa yang terjadi pada tools yang tidak tercakup oleh allow atau deny rules. Lihat [Permission mode](#permission-mode) untuk mode yang tersedia.

Anda juga dapat scope individual tools dengan rules seperti `"Bash(npm *)"` untuk mengizinkan hanya perintah spesifik. Lihat [Permissions](/docs/id/agent-sdk/permissions) untuk full rule syntax.

Ketika tool ditolak, Claude menerima rejection message sebagai tool result dan biasanya mencoba pendekatan berbeda atau melaporkan bahwa tidak dapat melanjutkan.

<h3 id="parallel-tool-execution">
  Parallel tool execution
</h3>

Ketika Claude meminta multiple tool calls dalam satu turn, kedua SDK dapat menjalankannya secara concurrent atau sequential tergantung pada tool. Read-only tools (seperti `Read`, `Glob`, `Grep`, dan MCP tools yang ditandai sebagai read-only) dapat berjalan secara concurrent. Tools yang memodifikasi state (seperti `Edit`, `Write`, dan `Bash`) berjalan secara sequential untuk menghindari conflicts.

Custom tools default ke sequential execution. Untuk mengaktifkan parallel execution untuk custom tool, set `readOnlyHint` dalam annotationsnya. Kedua [TypeScript](/docs/id/agent-sdk/typescript#tool) dan [Python](/docs/id/agent-sdk/python#tool) SDKs menggunakan field name ini dari MCP SDK.

<h2 id="control-how-the-loop-runs">
  Kontrol cara loop berjalan
</h2>

Anda dapat membatasi berapa banyak turns yang diambil loop, berapa banyak biayanya, seberapa dalam Claude bernalar, dan apakah tools memerlukan approval sebelum berjalan. Semua ini adalah fields pada [`ClaudeAgentOptions`](/docs/id/agent-sdk/python#claudeagentoptions) (Python) / [`Options`](/docs/id/agent-sdk/typescript#options) (TypeScript).

<h3 id="turns-and-budget">
  Turns dan budget
</h3>

| Opsi                                           | Apa yang dikontrolnya         | Default         |
| :--------------------------------------------- | :---------------------------- | :-------------- |
| Max turns (`max_turns` / `maxTurns`)           | Maximum tool-use round trips  | Tidak ada batas |
| Max budget (`max_budget_usd` / `maxBudgetUsd`) | Maximum cost sebelum berhenti | Tidak ada batas |

Ketika salah satu batas tercapai, SDK mengembalikan `ResultMessage` dengan error subtype yang sesuai (`error_max_turns` atau `error_max_budget_usd`). Lihat [Handle the result](#handle-the-result) untuk cara memeriksa subtypes ini dan [`ClaudeAgentOptions`](/docs/id/agent-sdk/python#claudeagentoptions) / [`Options`](/docs/id/agent-sdk/typescript#options) untuk syntax.

Dengan [streaming input](/docs/id/agent-sdk/streaming-vs-single-mode), pesan yang Anda kirim saat turn masih berjalan tetap antri ketika turn itu berakhir pada batas max-turns, dan itu memulai turn-nya sendiri dengan batas max-turns-nya sendiri. Sebelum v2.1.205, pesan yang tiba pada iterasi final turn bisa dikonsumsi ke dalam ending turn dan hilang tanpa pernah mencapai model.

<h3 id="effort-level">
  Effort level
</h3>

Opsi `effort` mengontrol berapa banyak reasoning yang diterapkan Claude. Lower effort levels menggunakan fewer tokens per turn dan mengurangi cost. Tidak semua models mendukung effort parameter. Lihat [Effort](https://platform.claude.com/docs/en/build-with-claude/effort) untuk models mana yang mendukungnya.

| Level      | Behavior                          | Baik untuk                                                                  |
| :--------- | :-------------------------------- | :-------------------------------------------------------------------------- |
| `"low"`    | Minimal reasoning, fast responses | File lookups, listing directories                                           |
| `"medium"` | Balanced reasoning                | Routine edits, standard tasks                                               |
| `"high"`   | Thorough analysis                 | Refactors, debugging                                                        |
| `"xhigh"`  | Extended reasoning depth          | Coding dan agentic tasks; recommended pada Fable 5, Opus 4.7+, dan Sonnet 5 |
| `"max"`    | Maximum reasoning depth           | Multi-step problems memerlukan deep analysis                                |

Jika Anda tidak set `effort`, kedua SDKs membiarkan parameter unset dan menunda ke model's default behavior.

<Note>
  `effort` trades latency dan token cost untuk reasoning depth dalam setiap respons. [Extended thinking](https://platform.claude.com/docs/en/build-with-claude/extended-thinking) adalah fitur terpisah yang menghasilkan visible chain-of-thought blocks dalam output. Mereka independen: Anda dapat set `effort: "low"` dengan extended thinking diaktifkan, atau `effort: "max"` tanpanya.
</Note>

Gunakan lower effort untuk agents yang melakukan simple, well-scoped tasks (seperti listing files atau menjalankan single grep) untuk mengurangi cost dan latency. Set `effort` dalam top-level `query()` options untuk seluruh sesi, atau per subagent dengan field `effort` pada [`AgentDefinition`](/docs/id/agent-sdk/subagents#agentdefinition-configuration) untuk override session level.

<h3 id="permission-mode">
  Permission mode
</h3>

Opsi permission mode (`permission_mode` di Python, `permissionMode` di TypeScript) mengontrol apakah agent meminta approval sebelum menggunakan tools:

| Mode                  | Behavior                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| :-------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `"default"`           | Tools yang tidak tercakup oleh allow rules memicu approval callback Anda; tidak ada callback berarti deny                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `"acceptEdits"`       | Auto-approves file edits dan common filesystem commands (`mkdir`, `touch`, `mv`, `cp`, dll.); Bash commands lainnya mengikuti default rules                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `"plan"`              | Claude mengeksplorasi dan merencanakan tanpa mengedit source files Anda; file edits tidak pernah auto-approved dan prompt melalui `canUseTool` callback Anda                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `"dontAsk"`           | Tidak pernah prompt. Tools pre-approved oleh [permission rules](/docs/id/settings#permission-settings) berjalan; semuanya lainnya ditolak. `AskUserQuestion`, connector tools [organisasi Anda set ke `ask`](/docs/id/mcp#organization-controls-on-connector-tools), dan MCP tools yang ditandai [`requiresUserInteraction`](/docs/id/mcp#require-approval-for-a-specific-tool) ditolak bahkan jika Anda telah mengizinkannya                                                                                                                                                                                   |
| `"auto"`              | Menggunakan model classifier untuk approve atau deny setiap tool call. Lihat [Auto mode](/docs/id/permission-modes#eliminate-prompts-with-auto-mode) untuk availability dan behavior                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `"bypassPermissions"` | Menjalankan semua allowed tools tanpa bertanya, kecuali tools yang cocok dengan explicit [`ask` rule](/docs/id/settings#permission-settings), connector tools [organisasi Anda set ke `ask`](/docs/id/mcp#organization-controls-on-connector-tools), dan tools yang memerlukan user interaction; lihat [How permissions are evaluated](/docs/id/agent-sdk/permissions#how-permissions-are-evaluated) untuk urutan precedence. Tidak dapat digunakan saat berjalan sebagai root pada Unix. Gunakan hanya dalam isolated environments di mana tindakan agent tidak dapat mempengaruhi systems yang Anda pedulikan |

Untuk interactive applications, gunakan `"default"` dengan tool approval callback untuk surface approval prompts. Untuk autonomous agents pada dev machine, `"acceptEdits"` auto-approves file edits dan common filesystem commands (`mkdir`, `touch`, `mv`, `cp`, dll.) sambil masih gating `Bash` commands lainnya di belakang allow rules. Reserve `"bypassPermissions"` untuk CI, containers, atau isolated environments lainnya. Lihat [Permissions](/docs/id/agent-sdk/permissions) untuk full details.

<h3 id="model">
  Model
</h3>

Jika Anda tidak set `model`, SDK menggunakan Claude Code's default, yang tergantung pada authentication method dan subscription Anda. Set secara eksplisit (misalnya, `model="claude-sonnet-5"`) untuk pin model spesifik atau untuk menggunakan smaller model untuk faster, cheaper agents. Lihat [models](https://platform.claude.com/docs/en/about-claude/models) untuk available IDs.

<h2 id="the-context-window">
  Jendela konteks
</h2>

Jendela konteks adalah total jumlah informasi yang tersedia untuk Claude selama sesi. Ini tidak reset antara turns dalam sesi. Semuanya terakumulasi: system prompt, tool definitions, conversation history, tool inputs, dan tool outputs. Konten yang tetap sama di seluruh turns (system prompt, tool definitions, CLAUDE.md) secara otomatis [prompt cached](https://platform.claude.com/docs/id/build-with-claude/prompt-caching), yang mengurangi cost dan latency untuk repeated prefixes.

<h3 id="what-consumes-context">
  Apa yang mengkonsumsi konteks
</h3>

Berikut adalah cara setiap komponen mempengaruhi konteks dalam SDK:

| Sumber                   | Ketika dimuat                                                                 | Dampak                                                                                                                                                                                                                                                                                                                                           |
| :----------------------- | :---------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **System prompt**        | Setiap request                                                                | Small fixed cost, selalu present                                                                                                                                                                                                                                                                                                                 |
| **CLAUDE.md files**      | Session start, melalui [`settingSources`](/docs/id/agent-sdk/claude-code-features) | Full content dalam setiap request (tetapi prompt-cached, jadi hanya request pertama yang membayar full cost)                                                                                                                                                                                                                                     |
| **Tool definitions**     | Setiap request; MCP schemas deferred secara default                           | Built-in tool schemas dimuat setiap request. [Tool search](/docs/id/agent-sdk/mcp#mcp-tool-search) menunda MCP tool schemas secara default, kembali ke upfront loading pada Google Cloud's Agent Platform atau non-first-party `ANTHROPIC_BASE_URL`. Lihat [Configure tool search](/docs/id/agent-sdk/tool-search#configure-tool-search) untuk full matrix |
| **Conversation history** | Terakumulasi di seluruh turns                                                 | Tumbuh dengan setiap turn: prompts, responses, tool inputs, tool outputs                                                                                                                                                                                                                                                                         |
| **Skill descriptions**   | Session start, melalui setting sources                                        | Short summaries; full content dimuat hanya ketika invoked                                                                                                                                                                                                                                                                                        |

Large tool outputs mengkonsumsi significant context. Membaca file besar atau menjalankan command dengan verbose output dapat menggunakan ribuan tokens dalam satu turn. Konteks terakumulasi di seluruh turns, jadi longer sessions dengan banyak tool calls membangun significantly lebih banyak konteks daripada short ones.

<h3 id="automatic-compaction">
  Automatic compaction
</h3>

Ketika jendela konteks mendekati limitnya, SDK secara otomatis compacts conversation: ini merangkum older history untuk membebaskan space, menjaga most recent exchanges dan key decisions Anda tetap intact. SDK memancarkan message dengan `type: "system"` dan `subtype: "compact_boundary"` dalam stream ketika ini terjadi (di Python ini adalah `SystemMessage`; di TypeScript ini adalah tipe `SDKCompactBoundaryMessage` terpisah).

Compaction menggantikan older messages dengan summary, jadi specific instructions dari early dalam conversation mungkin tidak dipertahankan. Persistent rules milik CLAUDE.md (dimuat melalui [`settingSources`](/docs/id/agent-sdk/claude-code-features)) daripada dalam initial prompt, karena CLAUDE.md content di-re-inject pada setiap request.

Anda dapat customize compaction behavior dalam beberapa cara:

* **Summarization instructions dalam CLAUDE.md:** Compactor membaca CLAUDE.md Anda seperti context lainnya, jadi Anda dapat menyertakan section yang memberi tahu apa yang dipertahankan saat merangkum. Section header adalah free-form (bukan magic string); compactor matches pada intent.
* **`PreCompact` hook:** Jalankan custom logic sebelum compaction terjadi, misalnya untuk archive full transcript. Hook menerima field `trigger` (`manual` atau `auto`). Lihat [hooks](/docs/id/agent-sdk/hooks).
* **Manual compaction:** Kirim `/compact` sebagai prompt string untuk trigger compaction on demand. Commands yang dikirim dengan cara ini adalah SDK inputs, bukan CLI-only shortcuts. Lihat [commands dalam SDK](/docs/id/agent-sdk/slash-commands).

<Accordion title="Contoh: Summarization instructions dalam CLAUDE.md">
  Tambahkan section ke CLAUDE.md proyek Anda yang memberi tahu compactor apa yang dipertahankan. Nama header tidak special; gunakan label yang jelas apa pun.

  ```markdown CLAUDE.md theme={null}
  # Summary instructions

  When summarizing this conversation, always preserve:
  - The current task objective and acceptance criteria
  - File paths that have been read or modified
  - Test results and error messages
  - Decisions made and the reasoning behind them
  ```
</Accordion>

<h3 id="keep-context-efficient">
  Jaga konteks tetap efisien
</h3>

Beberapa strategi untuk long-running agents:

* **Gunakan subagents untuk subtasks.** Setiap subagent dimulai dengan fresh conversation (tidak ada prior message history, meskipun dimuat system prompt dan project-level context seperti CLAUDE.md sendiri). Ini tidak melihat parent's turns, dan hanya final responsenya kembali ke parent sebagai tool result. Main agent's context tumbuh oleh summary itu, bukan oleh full subtask transcript. Lihat [What subagents inherit](/docs/id/agent-sdk/subagents#what-subagents-inherit) untuk details.
* **Jadilah selective dengan tools.** Setiap tool definition mengambil context space. Gunakan field `tools` pada [`AgentDefinition`](/docs/id/agent-sdk/subagents#agentdefinition-configuration) untuk scope subagents ke minimum set yang mereka butuhkan.
* **Perhatikan MCP server costs.** [MCP tool search](/docs/id/agent-sdk/mcp#mcp-tool-search) menunda MCP tool schemas secara default dan memuat mereka on demand. Ketika tool search dimatikan, pada Google Cloud's Agent Platform, atau di belakang non-first-party `ANTHROPIC_BASE_URL`, setiap MCP server menambahkan semua tool schemas-nya ke setiap request, jadi beberapa servers dengan banyak tools dapat mengkonsumsi significant context sebelum agent melakukan pekerjaan apa pun.
* **Gunakan lower effort untuk routine tasks.** Set [effort](#effort-level) ke `"low"` untuk agents yang hanya perlu membaca files atau list directories. Ini mengurangi token usage dan cost.

Untuk detailed breakdown dari per-feature context costs, lihat [Understand context costs](/docs/id/features-overview#understand-context-costs).

<h2 id="sessions-and-continuity">
  Sessions dan continuity
</h2>

Setiap interaksi dengan SDK membuat atau melanjutkan sesi. Capture session ID dari `ResultMessage.session_id` (tersedia di kedua SDKs) untuk resume nanti. TypeScript SDK juga mengeksposnya sebagai direct field pada init `SystemMessage`; di Python ini nested dalam `SystemMessage.data`.

Ketika Anda resume, full context dari previous turns dipulihkan: files yang dibaca, analysis yang dilakukan, dan actions yang diambil. Anda juga dapat fork sesi untuk branch ke pendekatan berbeda tanpa memodifikasi original.

Lihat [Session management](/docs/id/agent-sdk/sessions) untuk full guide pada resume, continue, dan fork patterns.

<Note>
  Di Python, `ClaudeSDKClient` menangani session IDs secara otomatis di seluruh multiple calls. Lihat [Python SDK reference](/docs/id/agent-sdk/python#choosing-between-query-and-claudesdkclient) untuk details.
</Note>

<h2 id="handle-the-result">
  Handle the result
</h2>

Ketika loop berakhir, `ResultMessage` memberi tahu Anda apa yang terjadi dan memberikan output. Field `subtype` (tersedia di kedua SDKs) adalah cara utama untuk memeriksa termination state.

| Result subtype                        | Apa yang terjadi                                                                                                                                                                             | Field `result` tersedia? |
| :------------------------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :----------------------: |
| `success`                             | Claude menyelesaikan tugas secara normal                                                                                                                                                     |            Ya            |
| `error_max_turns`                     | Mencapai batas `maxTurns` sebelum selesai                                                                                                                                                    |           Tidak          |
| `error_max_budget_usd`                | Mencapai batas `maxBudgetUsd` sebelum selesai                                                                                                                                                |           Tidak          |
| `error_during_execution`              | Error mengganggu loop (misalnya, API failure atau cancelled request)                                                                                                                         |           Tidak          |
| `error_max_structured_output_retries` | Tidak ada structured output yang valid diproduksi dalam configured retry limit: setiap upaya gagal validation, atau model fallback mencabut output yang sudah selesai tanpa successful retry |           Tidak          |

Field `result` (final text output) hanya present pada variant `success`, jadi selalu periksa subtype sebelum membacanya. Semua result subtypes membawa `total_cost_usd`, `usage`, `num_turns`, dan `session_id` sehingga Anda dapat track cost dan resume bahkan setelah errors. Di Python, `total_cost_usd` dan `usage` diketik sebagai optional dan mungkin `None` pada beberapa error paths, jadi guard sebelum formatting mereka. Lihat [Tracking costs dan usage](/docs/id/agent-sdk/cost-tracking) untuk details tentang interpreting `usage` fields.

<Note>
  Ketika query berakhir pada error result:

  * Sebuah single-shot `query()` call menghasilkan final result message, kemudian raises error yang mencakup failure text, seperti `Reached maximum number of turns`. Raise adalah intentional — bungkus loop dalam try block jika kode Anda perlu melanjutkan melewatinya. Underlying Claude Code process juga exits dengan nonzero code.
  * Sebuah streaming input session tetap alive, dan Anda dapat terus mengirim messages.
</Note>

Hasil juga mencakup field `stop_reason` (`string | null` di TypeScript, `str | None` di Python) yang menunjukkan mengapa model berhenti generating pada final turn-nya. Common values adalah `end_turn` (model selesai secara normal), `max_tokens` (mencapai output token limit), dan `refusal` (model menolak request). Pada error result subtypes, `stop_reason` membawa value dari last assistant response sebelum loop berakhir. Untuk mendeteksi refusals, periksa `stop_reason === "refusal"` (TypeScript) atau `stop_reason == "refusal"` (Python). Lihat [`SDKResultMessage`](/docs/id/agent-sdk/typescript#sdkresultmessage) (TypeScript) atau [`ResultMessage`](/docs/id/agent-sdk/python#resultmessage) (Python) untuk full type.

<h2 id="hooks">
  Hooks
</h2>

[Hooks](/docs/id/agent-sdk/hooks) adalah callbacks yang menyala pada titik-titik spesifik dalam loop: sebelum tool berjalan, setelah dikembalikan, ketika agent selesai, dan sebagainya. Beberapa hooks yang umum digunakan adalah:

| Hook                             | Ketika menyala                      | Penggunaan umum                              |
| :------------------------------- | :---------------------------------- | :------------------------------------------- |
| `PreToolUse`                     | Sebelum tool dieksekusi             | Validasi input, blokir perintah berbahaya    |
| `PostToolUse`                    | Setelah tool dikembalikan           | Audit output, picu efek samping              |
| `UserPromptSubmit`               | Ketika prompt dikirim               | Injeksi konteks tambahan ke dalam prompt     |
| `Stop`                           | Ketika agent selesai                | Validasi hasil, simpan status sesi           |
| `SubagentStart` / `SubagentStop` | Ketika subagent muncul atau selesai | Lacak dan agregasi hasil tugas paralel       |
| `PreCompact`                     | Sebelum pemadatan konteks           | Arsipkan transkrip lengkap sebelum merangkum |

Hooks berjalan dalam proses aplikasi Anda, bukan di dalam jendela konteks agent, jadi mereka tidak mengkonsumsi konteks. Hooks juga dapat memutus loop: hook `PreToolUse` yang menolak panggilan tool mencegahnya dari eksekusi, dan Claude menerima pesan penolakan sebagai gantinya.

Kedua SDK mendukung semua peristiwa di atas. SDK TypeScript mencakup peristiwa tambahan yang Python belum dukung. Lihat [Kontrol eksekusi dengan hooks](/docs/id/agent-sdk/hooks) untuk daftar peristiwa lengkap, ketersediaan per-SDK, dan API callback lengkap.

<h2 id="put-it-all-together">
  Satukan semuanya
</h2>

Contoh ini menggabungkan konsep-konsep kunci dari halaman ini ke dalam satu agent yang memperbaiki tes yang gagal. Ini mengkonfigurasi agent dengan tools yang diizinkan (auto-approved sehingga agent berjalan secara otonom), pengaturan proyek, dan batas keamanan pada turns dan effort reasoning. Saat loop berjalan, ini menangkap session ID untuk potensi resumption, menangani hasil akhir, dan mencetak total biaya.

Karena panggilan `query()` single-shot menaikkan error setelah menghasilkan hasil error, loop dibungkus dalam blok try sehingga skrip keluar dengan bersih ketika batas tercapai.

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage


  async def run_agent():
      session_id = None

      try:
          async for message in query(
              prompt="Find and fix the bug causing test failures in the auth module",
              options=ClaudeAgentOptions(
                  allowed_tools=[
                      "Read",
                      "Edit",
                      "Bash",
                      "Glob",
                      "Grep",
                  ],  # Listing tools here auto-approves them (no prompting)
                  setting_sources=[
                      "project"
                  ],  # Load CLAUDE.md, skills, hooks from current directory
                  max_turns=30,  # Prevent runaway sessions
                  effort="high",  # Thorough reasoning for complex debugging
              ),
          ):
              # Handle the final result
              if isinstance(message, ResultMessage):
                  session_id = message.session_id  # Save for potential resumption

                  if message.subtype == "success":
                      print(f"Done: {message.result}")
                  elif message.subtype == "error_max_turns":
                      # Agent ran out of turns. Resume with a higher limit.
                      print(f"Hit turn limit. Resume session {session_id} to continue.")
                  elif message.subtype == "error_max_budget_usd":
                      print("Hit budget limit.")
                  else:
                      print(f"Stopped: {message.subtype}")
                  if message.total_cost_usd is not None:
                      print(f"Cost: ${message.total_cost_usd:.4f}")
      except Exception as error:
          # A single-shot query() raises after yielding an error result. If the
          # failure was an error result, the error subtype branches above have
          # already run; connection or process failures yield no result message.
          print(f"Session ended with an error: {error}")


  asyncio.run(run_agent())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  let sessionId: string | undefined;

  try {
    for await (const message of query({
      prompt: "Find and fix the bug causing test failures in the auth module",
      options: {
        allowedTools: ["Read", "Edit", "Bash", "Glob", "Grep"], // Listing tools here auto-approves them (no prompting)
        settingSources: ["project"], // Load CLAUDE.md, skills, hooks from current directory
        maxTurns: 30, // Prevent runaway sessions
        effort: "high" // Thorough reasoning for complex debugging
      }
    })) {
      // Save the session ID to resume later if needed
      if (message.type === "system" && message.subtype === "init") {
        sessionId = message.session_id;
      }

      // Handle the final result
      if (message.type === "result") {
        if (message.subtype === "success") {
          console.log(`Done: ${message.result}`);
        } else if (message.subtype === "error_max_turns") {
          // Agent ran out of turns. Resume with a higher limit.
          console.log(`Hit turn limit. Resume session ${sessionId} to continue.`);
        } else if (message.subtype === "error_max_budget_usd") {
          console.log("Hit budget limit.");
        } else {
          console.log(`Stopped: ${message.subtype}`);
        }
        console.log(`Cost: $${message.total_cost_usd.toFixed(4)}`);
      }
    }
  } catch (error) {
    // A single-shot query() throws after yielding an error result. If the
    // failure was an error result, the error subtype branches above have
    // already run; connection or process failures yield no result message.
    console.log(`Session ended with an error: ${error}`);
  }
  ```
</CodeGroup>

<h2 id="next-steps">
  Langkah selanjutnya
</h2>

Sekarang Anda memahami loop, berikut adalah tempat untuk pergi tergantung pada apa yang Anda bangun:

* **Belum menjalankan agent?** Mulai dengan [quickstart](/docs/id/agent-sdk/quickstart) untuk mendapatkan SDK terinstal dan lihat contoh lengkap berjalan end to end.
* **Siap untuk hook ke proyek Anda?** [Load CLAUDE.md, skills, dan filesystem hooks](/docs/id/agent-sdk/claude-code-features) sehingga agent mengikuti project conventions Anda secara otomatis.
* **Membangun interactive UI?** Aktifkan [streaming](/docs/id/agent-sdk/streaming-output) untuk menampilkan live text dan tool calls saat loop berjalan.
* **Butuh tighter control atas apa yang dapat dilakukan agent?** Lock down tool access dengan [permissions](/docs/id/agent-sdk/permissions), dan gunakan [hooks](/docs/id/agent-sdk/hooks) untuk audit, block, atau transform tool calls sebelum dieksekusi.
* **Menjalankan long atau expensive tasks?** Offload isolated work ke [subagents](/docs/id/agent-sdk/subagents) untuk keep main context Anda lean.

Untuk broader conceptual picture dari agentic loop (bukan SDK-specific), lihat [How Claude Code works](/docs/id/how-claude-code-works). Untuk panduan praktis dalam merancang loops di Claude Code, dari turn-based hingga goal-based dan proactive loops, lihat [Loop engineering: getting started with loops](https://claude.com/blog/getting-started-with-loops) di blog.
