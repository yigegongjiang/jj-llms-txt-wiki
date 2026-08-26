> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Gunakan fitur Claude Code di SDK

> Muat instruksi proyek, skills, hooks, dan fitur Claude Code lainnya ke dalam agen SDK Anda.

Agent SDK dibangun di atas fondasi yang sama dengan Claude Code, yang berarti agen SDK Anda memiliki akses ke fitur berbasis filesystem yang sama: instruksi proyek (`CLAUDE.md` dan rules), skills, hooks, dan lainnya.

Ketika Anda menghilangkan `settingSources`, `query()` membaca pengaturan filesystem yang sama dengan Claude Code CLI: pengaturan pengguna, proyek, dan lokal, file `CLAUDE.md`, dan skills, agen, dan perintah di `.claude/`. Untuk menjalankan tanpa ini, teruskan `settingSources: []`, yang membatasi agen hanya pada apa yang Anda konfigurasi secara terprogram. Pengaturan kebijakan terkelola dan konfigurasi global `~/.claude.json` dibaca terlepas dari opsi ini. Lihat [Apa yang tidak dikontrol settingSources](#what-settingsources-does-not-control).

Untuk gambaran konseptual tentang apa yang dilakukan setiap fitur dan kapan menggunakannya, lihat [Perluas Claude Code](/docs/id/features-overview).

<h2 id="control-filesystem-settings-with-settingsources">
  Kontrol pengaturan filesystem dengan settingSources
</h2>

Opsi sumber pengaturan ([`setting_sources`](/docs/id/agent-sdk/python#claudeagentoptions) di Python, [`settingSources`](/docs/id/agent-sdk/typescript#settingsource) di TypeScript) mengontrol pengaturan berbasis filesystem mana yang dimuat SDK. Teruskan daftar eksplisit untuk memilih sumber tertentu, atau teruskan array kosong untuk menonaktifkan pengaturan pengguna, proyek, dan lokal.

Contoh ini memuat pengaturan tingkat pengguna dan tingkat proyek dengan menetapkan `settingSources` ke `["user", "project"]`:

<CodeGroup>
  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions, AssistantMessage, ResultMessage

  async for message in query(
      prompt="Help me refactor the auth module",
      options=ClaudeAgentOptions(
          # "user" loads from ~/.claude/, "project" loads from ./.claude/ in cwd.
          # Together they give the agent access to CLAUDE.md, skills, hooks, and
          # permissions from both locations.
          setting_sources=["user", "project"],
          allowed_tools=["Read", "Edit", "Bash"],
      ),
  ):
      if isinstance(message, AssistantMessage):
          for block in message.content:
              if hasattr(block, "text"):
                  print(block.text)
      if isinstance(message, ResultMessage) and message.subtype == "success":
          print(f"\nResult: {message.result}")
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Help me refactor the auth module",
    options: {
      // "user" loads from ~/.claude/, "project" loads from ./.claude/ in cwd.
      // Together they give the agent access to CLAUDE.md, skills, hooks, and
      // permissions from both locations.
      settingSources: ["user", "project"],
      allowedTools: ["Read", "Edit", "Bash"]
    }
  })) {
    if (message.type === "assistant") {
      for (const block of message.message.content) {
        if (block.type === "text") console.log(block.text);
      }
    }
    if (message.type === "result" && message.subtype === "success") {
      console.log(`\nResult: ${message.result}`);
    }
  }
  ```
</CodeGroup>

Setiap sumber memuat pengaturan dari lokasi tertentu, di mana `<cwd>` adalah direktori kerja yang Anda teruskan melalui opsi `cwd`, atau direktori saat ini proses jika tidak diatur. Untuk definisi tipe lengkap, lihat [`SettingSource`](/docs/id/agent-sdk/typescript#settingsource) (TypeScript) atau [`SettingSource`](/docs/id/agent-sdk/python#settingsource) (Python).

| Sumber      | Apa yang dimuat                                                                             | Lokasi                                                                                                                                                                                 |
| :---------- | :------------------------------------------------------------------------------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `"project"` | CLAUDE.md proyek, `.claude/rules/*.md`, skills proyek, hooks proyek, `settings.json` proyek | `<cwd>/.claude/` untuk `settings.json` dan hooks; `<cwd>` dan setiap direktori induk untuk CLAUDE.md dan rules; `<cwd>` dan setiap direktori induk hingga akar repositori untuk skills |
| `"user"`    | CLAUDE.md pengguna, `~/.claude/rules/*.md`, skills pengguna, pengaturan pengguna            | `~/.claude/`                                                                                                                                                                           |
| `"local"`   | CLAUDE.local.md, `.claude/settings.local.json`                                              | `<cwd>/.claude/` untuk `settings.local.json`; `<cwd>` dan setiap direktori induk untuk CLAUDE.local.md                                                                                 |

Menghilangkan `settingSources` setara dengan `["user", "project", "local"]`.

Opsi `cwd` menentukan di mana SDK mencari input tingkat proyek. CLAUDE.md dan rules dimuat dari `<cwd>` dan dari setiap direktori induk. Skills dimuat dari `<cwd>` dan dari setiap direktori induk hingga akar repositori. `settings.json` proyek dan hooks dimuat hanya dari `<cwd>/.claude/` tanpa fallback direktori induk.

<h3 id="what-settingsources-does-not-control">
  Apa yang tidak dikontrol settingSources
</h3>

`settingSources` mencakup pengaturan pengguna, proyek, dan lokal. Beberapa input dibaca terlepas dari nilainya:

| Input                                                                 | Perilaku                                                                                                                                                                                                                                                                                                                                                                                                    | Untuk menonaktifkan                                                                                                                                                                                         |
| :-------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Pengaturan kebijakan terkelola                                        | Kebijakan yang dikelola endpoint, baik plist MDM, kebijakan registri, atau file pengaturan terkelola, dimuat dari host. [Pengaturan yang dikelola server](/docs/id/server-managed-settings) diambil pada [konfigurasi yang memenuhi syarat](/docs/id/server-managed-settings#platform-availability) ketika sesi melakukan autentikasi dengan login OAuth organisasi atau kunci API yang dikonfigurasi secara langsung | Kebijakan endpoint: hapus file pengaturan terkelola, plist, atau kebijakan registri dari host. Pengaturan yang dikelola server: dikendalikan oleh admin organisasi Anda; tidak dapat dinonaktifkan dari SDK |
| Konfigurasi global `~/.claude.json`                                   | Selalu dibaca                                                                                                                                                                                                                                                                                                                                                                                               | Pindahkan dengan `CLAUDE_CONFIG_DIR` di `env`                                                                                                                                                               |
| Memori otomatis di `~/.claude/projects/<project>/memory/`             | Dimuat ke dalam system prompt pada awal sesi. Agen menulis memori baru di sana dengan tools `Write` dan `Edit` standar daripada tool memori khusus, jadi tools tersebut harus diaktifkan agar agen dapat menyimpan memori                                                                                                                                                                                   | Atur `autoMemoryEnabled: false` di pengaturan, atau `CLAUDE_CODE_DISABLE_AUTO_MEMORY=1` di `env`                                                                                                            |
| [Konektor MCP dari claude.ai](/docs/id/mcp#use-mcp-servers-from-claude-ai) | Dimuat ketika metode autentikasi aktif adalah langganan claude.ai. Melewatkan `mcpServers: {}` tidak menekannya                                                                                                                                                                                                                                                                                             | Atur `strictMcpConfig: true`, [`disableClaudeAiConnectors: true`](/docs/id/mcp#disable-claude-ai-connectors) di pengaturan, atau `ENABLE_CLAUDEAI_MCP_SERVERS=false` di `env`                                    |

<Warning>
  Jangan andalkan opsi `query()` default untuk isolasi multi-tenant. Karena input di atas dibaca terlepas dari `settingSources`, proses SDK dapat mengambil konfigurasi tingkat host dan memori per-direktori. Untuk deployment multi-tenant, jalankan setiap tenant di filesystem-nya sendiri dan atur `settingSources: []` ditambah `CLAUDE_CODE_DISABLE_AUTO_MEMORY=1` di `env`. [Pengaturan yang dikelola server](/docs/id/server-managed-settings) diambil ketika proses melakukan autentikasi dengan kredensial organisasi; isolasi filesystem tidak menghapusnya. Lihat [Secure deployment](/docs/id/agent-sdk/secure-deployment).
</Warning>

<h2 id="project-instructions-claude-md-and-rules">
  Instruksi proyek (CLAUDE.md dan rules)
</h2>

File `CLAUDE.md` dan file `.claude/rules/*.md` memberikan agen Anda konteks persisten tentang proyek Anda: konvensi pengkodean, perintah build, keputusan arsitektur, dan instruksi. Ketika `settingSources` mencakup `"project"` (seperti dalam contoh di atas), SDK memuat file ini ke dalam konteks pada awal sesi. Agen kemudian mengikuti konvensi proyek Anda tanpa Anda mengulanginya di setiap prompt.

<h3 id="claude-md-load-locations">
  Lokasi pemuatan CLAUDE.md
</h3>

| Level                    | Lokasi                                                                        | Kapan dimuat                                                                                            |
| :----------------------- | :---------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------ |
| Proyek (root)            | `<cwd>/CLAUDE.md` atau `<cwd>/.claude/CLAUDE.md`                              | `settingSources` mencakup `"project"`                                                                   |
| Rules proyek             | `<cwd>/.claude/rules/*.md` dan `.claude/rules/*.md` di setiap direktori induk | `settingSources` mencakup `"project"`                                                                   |
| Proyek (direktori induk) | File `CLAUDE.md` di direktori di atas `cwd`                                   | `settingSources` mencakup `"project"`, dimuat pada awal sesi                                            |
| Proyek (direktori anak)  | File `CLAUDE.md` di subdirektori `cwd`                                        | `settingSources` mencakup `"project"`, dimuat sesuai permintaan ketika agen membaca file di subtree itu |
| Lokal                    | `<cwd>/CLAUDE.local.md` dan `CLAUDE.local.md` di setiap direktori induk       | `settingSources` mencakup `"local"`                                                                     |
| Pengguna                 | `~/.claude/CLAUDE.md`                                                         | `settingSources` mencakup `"user"`                                                                      |
| Rules pengguna           | `~/.claude/rules/*.md`                                                        | `settingSources` mencakup `"user"`                                                                      |

Semua level bersifat aditif: jika file `CLAUDE.md` proyek dan pengguna keduanya ada, agen melihat keduanya. Tidak ada aturan preseden keras antara level; jika instruksi bertentangan, hasilnya tergantung pada bagaimana Claude menafsirkannya. Tulis aturan yang tidak bertentangan, atau nyatakan preseden secara eksplisit di file yang lebih spesifik ("Instruksi proyek ini menggantikan default tingkat pengguna yang bertentangan").

<Tip>
  Anda juga dapat menyuntikkan konteks secara langsung melalui `systemPrompt` tanpa menggunakan file `CLAUDE.md`. Lihat [Ubah system prompts](/docs/id/agent-sdk/modifying-system-prompts). Gunakan `CLAUDE.md` ketika Anda ingin konteks yang sama dibagikan antara sesi Claude Code interaktif dan agen SDK Anda.
</Tip>

Untuk cara menyusun dan mengorganisir konten `CLAUDE.md`, lihat [Kelola memori Claude](/docs/id/memory).

<h2 id="skills">
  Skills
</h2>

Skills adalah file markdown yang memberikan agen Anda pengetahuan khusus dan alur kerja yang dapat dipanggil. Tidak seperti `CLAUDE.md` (yang dimuat setiap sesi), skills dimuat sesuai permintaan. Agen menerima deskripsi skill pada startup dan memuat konten lengkap ketika relevan.

Skills ditemukan dari filesystem melalui `settingSources`. Ketika opsi `skills` pada `query()` dihilangkan, skills pengguna dan proyek yang ditemukan diaktifkan dan tool Skill tersedia, sesuai dengan perilaku CLI. Untuk mengontrol skills mana yang diaktifkan, berikan `skills` sebagai `"all"`, daftar nama skill, atau `[]` untuk menonaktifkan semua. Ketika `skills` diatur, SDK secara otomatis menambahkan tool Skill ke `allowedTools`. Jika Anda juga meneruskan daftar `tools` eksplisit, sertakan `"Skill"` dalam daftar tersebut sehingga Claude dapat memanggil skills.

<CodeGroup>
  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage

  # Skills in .claude/skills/ are discovered automatically
  # when settingSources includes "project"
  async for message in query(
      prompt="Review this PR using our code review checklist",
      options=ClaudeAgentOptions(
          setting_sources=["user", "project"],
          skills="all",
          allowed_tools=["Read", "Grep", "Glob"],
      ),
  ):
      if isinstance(message, ResultMessage) and message.subtype == "success":
          print(message.result)
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // Skills in .claude/skills/ are discovered automatically
  // when settingSources includes "project"
  for await (const message of query({
    prompt: "Review this PR using our code review checklist",
    options: {
      settingSources: ["user", "project"],
      skills: "all",
      allowedTools: ["Read", "Grep", "Glob"]
    }
  })) {
    if (message.type === "result" && message.subtype === "success") {
      console.log(message.result);
    }
  }
  ```
</CodeGroup>

<Note>
  Skills harus dibuat sebagai artifact filesystem (`.claude/skills/<name>/SKILL.md`). SDK tidak memiliki API terprogram untuk mendaftarkan skills. Lihat [Agent Skills di SDK](/docs/id/agent-sdk/skills) untuk detail lengkap.
</Note>

Untuk lebih lanjut tentang membuat dan menggunakan skills, lihat [Agent Skills di SDK](/docs/id/agent-sdk/skills).

<h2 id="hooks">
  Hooks
</h2>

SDK mendukung dua cara untuk mendefinisikan hooks, dan mereka berjalan beriringan:

* **Filesystem hooks:** perintah shell yang didefinisikan di `settings.json`, dimuat ketika `settingSources` mencakup sumber yang relevan. Ini adalah hooks yang sama yang akan Anda konfigurasi untuk [sesi Claude Code interaktif](/docs/id/hooks-guide).
* **Programmatic hooks:** fungsi callback yang diteruskan langsung ke `query()`. Ini berjalan dalam proses aplikasi Anda dan dapat mengembalikan keputusan terstruktur. Lihat [Kontrol eksekusi dengan hooks](/docs/id/agent-sdk/hooks).

Kedua tipe berjalan selama siklus hidup hook yang sama. Jika Anda sudah memiliki hooks di `.claude/settings.json` proyek Anda dan Anda menetapkan `settingSources: ["project"]`, hooks tersebut berjalan secara otomatis di SDK tanpa konfigurasi tambahan.

Callback hook menerima input tool dan mengembalikan dict keputusan. Mengembalikan `{}` berarti izinkan tool untuk melanjutkan. Untuk memblokir eksekusi, kembalikan objek `hookSpecificOutput` dengan `permissionDecision: "deny"` dan `permissionDecisionReason`. Alasan dikirim ke Claude sebagai hasil tool. Bidang tingkat atas `decision` dan `reason` sudah usang untuk `PreToolUse`. Lihat [panduan hooks](/docs/id/agent-sdk/hooks) untuk tanda tangan callback lengkap dan tipe pengembalian.

<CodeGroup>
  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions, HookMatcher, ResultMessage


  # PreToolUse hook callback. Positional args:
  #   input_data: HookInput dict with tool_name, tool_input, hook_event_name
  #   tool_use_id: str | None, the ID of the tool call being intercepted
  #   context: HookContext, carries session metadata
  async def audit_bash(input_data, tool_use_id, context):
      command = input_data.get("tool_input", {}).get("command", "")
      if "rm -rf" in command:
          return {
              "hookSpecificOutput": {
                  "hookEventName": "PreToolUse",
                  "permissionDecision": "deny",
                  "permissionDecisionReason": "Destructive command blocked",
              }
          }
      return {}  # Empty dict: allow the tool to proceed


  # Filesystem hooks from .claude/settings.json run automatically
  # when settingSources loads them. You can also add programmatic hooks:
  async for message in query(
      prompt="Refactor the auth module",
      options=ClaudeAgentOptions(
          setting_sources=["project"],  # Loads hooks from .claude/settings.json
          hooks={
              "PreToolUse": [
                  HookMatcher(matcher="Bash", hooks=[audit_bash]),
              ]
          },
      ),
  ):
      if isinstance(message, ResultMessage) and message.subtype == "success":
          print(message.result)
  ```

  ```typescript TypeScript theme={null}
  import { query, type HookInput, type HookJSONOutput } from "@anthropic-ai/claude-agent-sdk";

  // PreToolUse hook callback. HookInput is a discriminated union on
  // hook_event_name, so narrowing on it gives TypeScript the right
  // tool_input shape for this event.
  const auditBash = async (input: HookInput): Promise<HookJSONOutput> => {
    if (input.hook_event_name !== "PreToolUse") return {};
    const toolInput = input.tool_input as { command?: string };
    if (toolInput.command?.includes("rm -rf")) {
      return {
        hookSpecificOutput: {
          hookEventName: "PreToolUse",
          permissionDecision: "deny",
          permissionDecisionReason: "Destructive command blocked",
        },
      };
    }
    return {}; // Empty object: allow the tool to proceed
  };

  // Filesystem hooks from .claude/settings.json run automatically
  // when settingSources loads them. You can also add programmatic hooks:
  for await (const message of query({
    prompt: "Refactor the auth module",
    options: {
      settingSources: ["project"], // Loads hooks from .claude/settings.json
      hooks: {
        PreToolUse: [{ matcher: "Bash", hooks: [auditBash] }]
      }
    }
  })) {
    if (message.type === "result" && message.subtype === "success") {
      console.log(message.result);
    }
  }
  ```
</CodeGroup>

<h3 id="when-to-use-which-hook-type">
  Kapan menggunakan tipe hook mana
</h3>

| Tipe hook                                 | Terbaik untuk                                                                                                                                                                                                                                                                                                          |
| :---------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Filesystem** (`settings.json`)          | Berbagi hooks antara sesi CLI dan SDK. Mendukung `"command"` (skrip shell), `"http"` (POST ke endpoint), `"mcp_tool"` (panggil tool server MCP yang terhubung), `"prompt"` (LLM mengevaluasi prompt), dan `"agent"` (menghasilkan agen verifier). Ini dijalankan di agen utama dan subagen apa pun yang dihasilkannya. |
| **Programmatic** (callbacks di `query()`) | Logika khusus aplikasi, keputusan terstruktur, dan integrasi dalam proses. Ini juga dijalankan di dalam subagen. Callback menerima `agent_id` dan `agent_type` untuk membedakan.                                                                                                                                       |

<Note>
  SDK TypeScript mendukung event hook tambahan di luar Python, termasuk `SessionStart`, `SessionEnd`, `TeammateIdle`, dan `TaskCompleted`. Lihat [panduan hooks](/docs/id/agent-sdk/hooks) untuk tabel kompatibilitas event lengkap.
</Note>

Untuk detail lengkap tentang hooks terprogram, lihat [Kontrol eksekusi dengan hooks](/docs/id/agent-sdk/hooks). Untuk sintaks hook filesystem, lihat [Hooks](/docs/id/hooks).

<h2 id="choose-the-right-feature">
  Pilih fitur yang tepat
</h2>

Agent SDK memberi Anda akses ke beberapa cara untuk memperluas perilaku agen Anda. Jika Anda tidak yakin mana yang digunakan, tabel ini memetakan tujuan umum ke pendekatan yang tepat.

| Anda ingin...                                                                                        | Gunakan                                       | Permukaan SDK                                                                                                                                                                              |
| :--------------------------------------------------------------------------------------------------- | :-------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Atur konvensi proyek yang selalu diikuti agen Anda                                                   | [CLAUDE.md](/docs/id/memory)                       | `settingSources: ["project"]` memuat secara otomatis                                                                                                                                       |
| Berikan agen materi referensi yang dimuat ketika relevan                                             | [Skills](/docs/id/agent-sdk/skills)                | `settingSources` + `skills` option                                                                                                                                                         |
| Jalankan alur kerja yang dapat digunakan kembali (deploy, review, release)                           | [User-invocable skills](/docs/id/agent-sdk/skills) | `settingSources` + `skills` option                                                                                                                                                         |
| Delegasikan subtask terisolasi ke konteks segar (research, review)                                   | [Subagents](/docs/id/agent-sdk/subagents)          | Parameter `agents` + `allowedTools: ["Agent"]`                                                                                                                                             |
| Koordinasikan beberapa instans Claude Code dengan daftar tugas bersama dan pesan inter-agen langsung | [Agent teams](/docs/id/agent-teams)                | Tidak dikonfigurasi langsung melalui opsi SDK. Agent teams adalah fitur CLI di mana satu sesi bertindak sebagai pemimpin tim, mengoordinasikan pekerjaan di seluruh rekan kerja independen |
| Jalankan logika deterministik pada tool calls (audit, block, transform)                              | [Hooks](/docs/id/agent-sdk/hooks)                  | Parameter `hooks` dengan callbacks, atau skrip shell dimuat melalui `settingSources`                                                                                                       |
| Berikan Claude akses tool terstruktur ke layanan eksternal                                           | [MCP](/docs/id/agent-sdk/mcp)                      | Parameter `mcpServers`                                                                                                                                                                     |

<Tip>
  **Subagents versus agent teams:** Subagents bersifat ephemeral dan terisolasi: percakapan segar, satu tugas, ringkasan dikembalikan ke induk. Agent teams mengoordinasikan beberapa instans Claude Code independen yang berbagi daftar tugas dan saling mengirim pesan langsung. Agent teams adalah fitur CLI. Lihat [Apa yang diwarisi subagents](/docs/id/agent-sdk/subagents#what-subagents-inherit) dan [perbandingan agent teams](/docs/id/agent-teams#compare-with-subagents) untuk detail.
</Tip>

Setiap fitur yang Anda aktifkan menambah jendela konteks agen Anda. Untuk biaya per-fitur dan bagaimana fitur ini berlapis bersama, lihat [Perluas Claude Code](/docs/id/features-overview#understand-context-costs).

<h2 id="related-resources">
  Sumber daya terkait
</h2>

* [Perluas Claude Code](/docs/id/features-overview): Gambaran konseptual semua fitur ekstensi, dengan tabel perbandingan dan analisis biaya konteks
* [Skills di SDK](/docs/id/agent-sdk/skills): Panduan lengkap menggunakan skills secara terprogram
* [Subagents](/docs/id/agent-sdk/subagents): Tentukan dan panggil subagents untuk subtask terisolasi
* [Hooks](/docs/id/agent-sdk/hooks): Intersep dan kontrol perilaku agen di titik eksekusi kunci
* [Permissions](/docs/id/agent-sdk/permissions): Kontrol akses tool dengan mode, rules, dan callbacks
* [System prompts](/docs/id/agent-sdk/modifying-system-prompts): Suntikkan konteks tanpa file CLAUDE.md
