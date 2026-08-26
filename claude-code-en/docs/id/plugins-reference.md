> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Referensi Plugins

> Referensi teknis lengkap untuk sistem plugin Claude Code, termasuk skema, perintah CLI, dan spesifikasi komponen.

<Tip>
  Mencari cara memasang plugins? Lihat [Temukan dan pasang plugins](/docs/id/discover-plugins). Untuk membuat plugins, lihat [Plugins](/docs/id/plugins). Untuk mendistribusikan plugins, lihat [Plugin marketplaces](/docs/id/plugin-marketplaces).
</Tip>

Referensi ini menyediakan spesifikasi teknis lengkap untuk sistem plugin Claude Code, termasuk skema komponen, perintah CLI, dan alat pengembangan.

Sebuah **plugin** adalah direktori yang mandiri berisi komponen yang memperluas Claude Code dengan fungsionalitas khusus. Komponen plugin mencakup skills, agents, hooks, MCP servers, LSP servers, dan monitors.

<h2 id="plugin-components-reference">
  Referensi komponen plugin
</h2>

<h3 id="skills">
  Skills
</h3>

Plugins menambahkan skills ke Claude Code, membuat pintasan `/name` yang dapat Anda atau Claude panggil.

**Lokasi**: Direktori `skills/` atau `commands/` di root plugin, atau file `SKILL.md` tunggal di root plugin

**Format file**: Skills adalah direktori dengan `SKILL.md`; commands adalah file markdown sederhana

**Struktur skill**:

```text theme={null}
skills/
├── pdf-processor/
│   ├── SKILL.md
│   ├── reference.md (opsional)
│   └── scripts/ (opsional)
└── code-reviewer/
    └── SKILL.md
```

**Perilaku integrasi**:

* Skills dan commands secara otomatis ditemukan saat plugin dipasang
* Claude dapat memanggilnya secara otomatis berdasarkan konteks tugas
* Skills dapat menyertakan file pendukung di samping SKILL.md

Jika plugin tidak memiliki direktori `skills/` dan tidak memiliki field manifest `skills`, file `SKILL.md` di root plugin dimuat sebagai skill tunggal. Atur field frontmatter `name` untuk mengontrol nama invokasi skill. Tanpanya, Claude Code kembali ke nama direktori instalasi, yang untuk plugins yang dipasang dari marketplace adalah string versi yang berubah pada setiap update. Untuk plugins yang mengirimkan lebih dari satu skill, gunakan tata letak direktori `skills/` yang ditunjukkan di atas.

Untuk detail lengkap, lihat [Skills](/docs/id/skills).

<h3 id="agents">
  Agents
</h3>

Plugins dapat menyediakan subagents khusus untuk tugas-tugas tertentu yang dapat Claude panggil secara otomatis jika sesuai.

**Lokasi**: Direktori `agents/` di root plugin

**Format file**: File markdown yang menjelaskan kemampuan agent

**Struktur agent**:

```markdown theme={null}
---
name: agent-name
description: Apa yang agent ini spesialisasikan dan kapan Claude harus memanggilnya
model: sonnet
effort: medium
maxTurns: 20
disallowedTools: Write, Edit
---

Prompt sistem terperinci untuk agent yang menjelaskan peran, keahlian, dan perilakunya.
```

Plugin agents mendukung field frontmatter `name`, `description`, `model`, `effort`, `maxTurns`, `tools`, `disallowedTools`, `skills`, `memory`, `background`, dan `isolation`. Satu-satunya nilai `isolation` yang valid adalah `"worktree"`. Untuk alasan keamanan, `hooks`, `mcpServers`, dan `permissionMode` tidak didukung untuk agents yang dikirim plugin.

**Titik integrasi**:

* Agents muncul di typeahead [@-mention](/docs/id/sub-agents#invoke-subagents-explicitly) dengan nama yang diberi scope, seperti `my-plugin:code-reviewer`, setelah plugin diaktifkan
* Claude dapat memanggil agents secara otomatis berdasarkan konteks tugas
* Agents dapat dipanggil secara manual oleh pengguna
* Plugin agents bekerja bersama agents Claude bawaan

Untuk detail lengkap, lihat [Subagents](/docs/id/sub-agents).

<h3 id="hooks">
  Hooks
</h3>

Plugins dapat menyediakan event handlers yang merespons peristiwa Claude Code secara otomatis.

**Lokasi**: `hooks/hooks.json` di root plugin, atau inline di plugin.json

**Format**: Konfigurasi JSON dengan event matchers dan actions

**Konfigurasi hook**:

```json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Write|Edit",
        "hooks": [
          {
            "type": "command",
            "command": "\"${CLAUDE_PLUGIN_ROOT}\"/scripts/format-code.sh"
          }
        ]
      }
    ]
  }
}
```

Plugin hooks merespons peristiwa lifecycle yang sama seperti [hooks yang ditentukan pengguna](/docs/id/hooks):

| Event                 | When it fires                                                                                                                                                                                                                                         |
| :-------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `SessionStart`        | When a session begins or resumes                                                                                                                                                                                                                      |
| `Setup`               | When you start Claude Code with `--init-only`, or with `--init` or `--maintenance` in `-p` mode. For one-time preparation in CI or scripts                                                                                                            |
| `UserPromptSubmit`    | When you submit a prompt, before Claude processes it                                                                                                                                                                                                  |
| `UserPromptExpansion` | When a user-typed command expands into a prompt, before it reaches Claude. Can block the expansion                                                                                                                                                    |
| `PreToolUse`          | Before a tool call executes. Can block it                                                                                                                                                                                                             |
| `PermissionRequest`   | When a tool call needs a permission decision                                                                                                                                                                                                          |
| `PermissionDenied`    | When auto mode denies a tool call, including denials without a classifier verdict. Use JSON `hookSpecificOutput.retry: true` to tell the model it may retry the denied tool call. Claude Code ignores `retry` when the classifier produced no verdict |
| `PostToolUse`         | After a tool call succeeds                                                                                                                                                                                                                            |
| `PostToolUseFailure`  | After a tool call fails                                                                                                                                                                                                                               |
| `PostToolBatch`       | After a full batch of parallel tool calls resolves, before the next model call                                                                                                                                                                        |
| `Notification`        | When Claude Code sends a notification                                                                                                                                                                                                                 |
| `MessageDisplay`      | While assistant message text is displayed                                                                                                                                                                                                             |
| `SubagentStart`       | When a subagent is spawned                                                                                                                                                                                                                            |
| `SubagentStop`        | When a subagent finishes                                                                                                                                                                                                                              |
| `TaskCreated`         | When a task is being created via `TaskCreate`                                                                                                                                                                                                         |
| `TaskCompleted`       | When a task is being marked as completed                                                                                                                                                                                                              |
| `Stop`                | When Claude finishes responding                                                                                                                                                                                                                       |
| `StopFailure`         | When the turn ends due to an API error                                                                                                                                                                                                                |
| `TeammateIdle`        | When an [agent team](/docs/en/agent-teams) teammate is about to go idle                                                                                                                                                                                    |
| `InstructionsLoaded`  | When a CLAUDE.md or `.claude/rules/*.md` file is loaded into context. Fires at session start and when files are lazily loaded during a session                                                                                                        |
| `ConfigChange`        | When a configuration file changes during a session                                                                                                                                                                                                    |
| `CwdChanged`          | When the working directory changes, for example when Claude executes a `cd` command. Useful for reactive environment management with tools like direnv                                                                                                |
| `DirectoryAdded`      | When a working directory is added mid-session via `/add-dir` or the SDK `register_repo_root` control request                                                                                                                                          |
| `FileChanged`         | When a watched file changes on disk. The `matcher` field specifies which filenames to watch                                                                                                                                                           |
| `WorktreeCreate`      | When a worktree is being created via `--worktree`, `isolation: "worktree"`, or for a background session. Replaces default git behavior                                                                                                                |
| `WorktreeRemove`      | When a worktree is being removed at session exit, when a subagent finishes, or when you delete a background session                                                                                                                                   |
| `PreCompact`          | Before context compaction                                                                                                                                                                                                                             |
| `PostCompact`         | After context compaction completes                                                                                                                                                                                                                    |
| `Elicitation`         | When an MCP server requests user input during a tool call                                                                                                                                                                                             |
| `ElicitationResult`   | After a user responds to an MCP elicitation, before the response is sent back to the server                                                                                                                                                           |
| `SessionEnd`          | When a session terminates                                                                                                                                                                                                                             |

**Tipe hook**:

* `command`: jalankan perintah shell atau scripts
* `http`: kirim JSON event sebagai POST request ke URL
* `mcp_tool`: panggil tool pada [MCP server](/docs/id/mcp) yang dikonfigurasi
* `prompt`: evaluasi prompt dengan LLM (menggunakan placeholder `$ARGUMENTS` untuk konteks)
* `agent`: jalankan verifier agentic dengan tools untuk tugas verifikasi kompleks

Hooks yang menargetkan [MCP server bundel](/docs/id/mcp#plugin-provided-mcp-servers) plugin itu sendiri harus menggunakan nama yang diberi scope. Tool matchers dan field `if` mengambil nama tool yang diberi scope `mcp__plugin_<plugin-name>_<server-name>__<tool>`, dan field `server` hook `mcp_tool` mengambil `plugin:<plugin-name>:<server-name>`. Matcher yang ditulis terhadap kunci server bare tidak pernah aktif. Lihat [Match MCP tools](/docs/id/hooks#match-mcp-tools) dan [Plugin-provided MCP servers](/docs/id/mcp#plugin-provided-mcp-servers).

<h3 id="mcp-servers">
  MCP servers
</h3>

Plugins dapat menggabungkan Model Context Protocol (MCP) servers untuk menghubungkan Claude Code dengan alat dan layanan eksternal.

**Lokasi**: `.mcp.json` di root plugin, atau inline di plugin.json

**Format**: Konfigurasi MCP server standar

**Konfigurasi MCP server**:

```json theme={null}
{
  "mcpServers": {
    "plugin-database": {
      "command": "${CLAUDE_PLUGIN_ROOT}/servers/db-server",
      "args": ["--config", "${CLAUDE_PLUGIN_ROOT}/config.json"],
      "env": {
        "DB_PATH": "${CLAUDE_PLUGIN_ROOT}/data"
      }
    },
    "plugin-api-client": {
      "command": "npx",
      "args": ["@company/mcp-server", "--plugin-mode"]
    }
  }
}
```

**Perilaku integrasi**:

* Plugin MCP servers dimulai secara otomatis saat plugin diaktifkan
* Servers muncul sebagai alat MCP standar di toolkit Claude
* Kemampuan server terintegrasi dengan mulus dengan alat Claude yang ada
* Plugin servers dapat dikonfigurasi secara independen dari MCP servers pengguna

<h3 id="lsp-servers">
  LSP servers
</h3>

<Tip>
  Mencari cara menggunakan LSP plugins? Pasang dari marketplace resmi: cari "lsp" di tab Discover `/plugin`. Bagian ini mendokumentasikan cara membuat LSP plugins untuk bahasa yang tidak tercakup oleh marketplace resmi.
</Tip>

Plugins dapat menyediakan server [Language Server Protocol](https://microsoft.github.io/language-server-protocol/) (LSP) untuk memberikan Claude intelijen kode real-time saat bekerja pada codebase Anda.

Integrasi LSP menyediakan:

* **Diagnostik instan**: Claude melihat kesalahan dan peringatan segera setelah setiap edit
* **Navigasi kode**: buka definisi, temukan referensi, dan informasi hover
* **Kesadaran bahasa**: informasi tipe dan dokumentasi untuk simbol kode

**Lokasi**: `.lsp.json` di root plugin, atau inline di `plugin.json`

**Format**: Konfigurasi JSON yang memetakan nama language server ke konfigurasinya

**Format file `.lsp.json`**:

```json theme={null}
{
  "go": {
    "command": "gopls",
    "args": ["serve"],
    "extensionToLanguage": {
      ".go": "go"
    }
  }
}
```

**Inline di `plugin.json`**:

```json theme={null}
{
  "name": "my-plugin",
  "lspServers": {
    "go": {
      "command": "gopls",
      "args": ["serve"],
      "extensionToLanguage": {
        ".go": "go"
      }
    }
  }
}
```

**Field yang diperlukan:**

| Field                 | Deskripsi                                          |
| :-------------------- | :------------------------------------------------- |
| `command`             | Biner LSP yang akan dijalankan (harus ada di PATH) |
| `extensionToLanguage` | Memetakan ekstensi file ke pengenal bahasa         |

**Field opsional:**

| Field                   | Deskripsi                                                                                                                                                                           |
| :---------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `args`                  | Argumen baris perintah untuk LSP server                                                                                                                                             |
| `transport`             | Transport komunikasi: `stdio` (default) atau `socket`                                                                                                                               |
| `env`                   | Variabel lingkungan yang diatur saat memulai server                                                                                                                                 |
| `initializationOptions` | Opsi yang diteruskan ke server selama inisialisasi                                                                                                                                  |
| `settings`              | Pengaturan yang diteruskan melalui `workspace/didChangeConfiguration`                                                                                                               |
| `workspaceFolder`       | Jalur folder workspace untuk server                                                                                                                                                 |
| `startupTimeout`        | Waktu maksimal untuk menunggu startup server (milidetik)                                                                                                                            |
| `shutdownTimeout`       | Waktu maksimal untuk menunggu shutdown yang elegan (milidetik). Ketika timeout berlalu, Claude Code menghentikan proses server. Ketika tidak diatur, tidak ada timeout yang berlaku |
| `restartOnCrash`        | Apakah memulai ulang server setelah crash. Default ke `true`. Atur ke `false` untuk membiarkan server yang crash tetap berhenti daripada memulai ulang                              |
| `maxRestarts`           | Jumlah maksimal upaya restart sebelum menyerah                                                                                                                                      |
| `diagnostics`           | Apakah mendorong diagnostik ke dalam konteks Claude setelah edits (default `true`). Atur ke `false` untuk mempertahankan navigasi kode tetapi menekan injeksi diagnostik otomatis.  |

`restartOnCrash` dan `shutdownTimeout` memerlukan Claude Code v2.1.205 atau lebih baru. Sebelum v2.1.205, skema config menerima kedua opsi tetapi mengatur salah satu menyebabkan Claude Code melewati LSP server itu sepenuhnya saat startup, dengan alasan hanya terlihat di output `claude --debug`.

**Multiple servers untuk ekstensi yang sama**: ketika lebih dari satu LSP server yang diaktifkan mendeklarasikan ekstensi file yang sama di `extensionToLanguage`, apakah servers berasal dari satu plugin atau dari plugin yang berbeda, server pertama yang terdaftar menangani file dengan ekstensi itu dan yang lain tidak pernah dimulai. Interface `/plugin` menampilkan peringatan yang menamai plugin yang servernya aktif.

**Servers yang gagal menginisialisasi**: Claude Code melewati server yang konfigurasinya tidak valid, misalnya yang hilang `command` atau `extensionToLanguage`, dan server yang dikonfigurasi lainnya masih dimulai. Jalankan `claude --debug` untuk melihat mengapa server dilewati.

Server yang dilewati tidak mengklaim ekstensi filenya, jadi server valid lain yang mendeklarasikan ekstensi yang sama, dari plugin yang sama atau berbeda, masih menangani file tersebut. Sebelum v2.1.205, server yang gagal menginisialisasi masih mengklaim ekstensinya dan memblokir server valid lain untuk ekstensi yang sama.

<Warning>
  **Anda harus memasang biner language server secara terpisah.** LSP plugins mengonfigurasi cara Claude Code terhubung ke language server, tetapi mereka tidak menyertakan server itu sendiri. Jika Anda melihat `Executable not found in $PATH` di tab Errors `/plugin`, pasang biner yang diperlukan untuk bahasa Anda.
</Warning>

**LSP plugins yang tersedia:**

| Plugin              | Language server            | Perintah instalasi                                                                        |
| :------------------ | :------------------------- | :---------------------------------------------------------------------------------------- |
| `pyright-lsp`       | Pyright (Python)           | `pip install pyright` atau `npm install -g pyright`                                       |
| `typescript-lsp`    | TypeScript Language Server | `npm install -g typescript-language-server typescript`                                    |
| `rust-analyzer-lsp` | rust-analyzer              | [Lihat instalasi rust-analyzer](https://rust-analyzer.github.io/manual.html#installation) |

Pasang language server terlebih dahulu, kemudian pasang plugin dari marketplace.

<h3 id="monitors">
  Monitors
</h3>

Plugins dapat mendeklarasikan monitors latar belakang yang Claude Code mulai secara otomatis saat plugin aktif. Setiap monitor menjalankan perintah shell untuk seumur hidup sesi dan mengirimkan setiap baris stdout ke Claude sebagai notifikasi, sehingga Claude dapat bereaksi terhadap entri log, perubahan status, atau peristiwa yang dipolling tanpa diminta untuk memulai watch itu sendiri.

Plugin monitors menggunakan mekanisme yang sama seperti [Monitor tool](/docs/id/tools-reference#monitor-tool) dan berbagi batasan ketersediaannya. Mereka hanya berjalan dalam sesi CLI interaktif, berjalan tanpa sandbox pada tingkat kepercayaan yang sama seperti [hooks](#hooks), dan dilewati pada host di mana Monitor tool tidak tersedia.

**Lokasi**: `monitors/monitors.json` di root plugin, atau inline di `plugin.json`

**Format**: Array JSON dari entri monitor

`monitors/monitors.json` berikut memantau endpoint status deployment dan log error lokal:

```json theme={null}
[
  {
    "name": "deploy-status",
    "command": "\"${CLAUDE_PLUGIN_ROOT}\"/scripts/poll-deploy.sh",
    "description": "Deployment status changes"
  },
  {
    "name": "error-log",
    "command": "tail -F ./logs/error.log",
    "description": "Application error log",
    "when": "on-skill-invoke:debug"
  }
]
```

Untuk mendeklarasikan monitors inline, atur `experimental.monitors` di `plugin.json` ke array yang sama. Untuk memuat dari jalur non-default, atur `experimental.monitors` ke string jalur relatif seperti `"./config/monitors.json"`. Monitors adalah [komponen eksperimental](#experimental-components).

**Field yang diperlukan:**

| Field         | Deskripsi                                                                                                     |
| :------------ | :------------------------------------------------------------------------------------------------------------ |
| `name`        | Pengenal unik dalam plugin. Mencegah proses duplikat saat plugin dimuat ulang atau skill dipanggil lagi       |
| `command`     | Perintah shell yang dijalankan sebagai proses latar belakang persisten dalam direktori kerja sesi             |
| `description` | Ringkasan singkat tentang apa yang sedang dipantau. Ditampilkan di panel tugas dan dalam ringkasan notifikasi |

**Field opsional:**

| Field  | Deskripsi                                                                                                                                                                                                                |
| :----- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `when` | Mengontrol kapan monitor dimulai. `"always"` memulainya saat startup sesi dan pada reload plugin, dan merupakan default. `"on-skill-invoke:<skill-name>"` memulainya pertama kali skill bernama dalam plugin ini dikirim |

Nilai `command` mendukung [substitusi variabel](#environment-variables) `${CLAUDE_PLUGIN_ROOT}`, `${CLAUDE_PLUGIN_DATA}`, dan `${CLAUDE_PROJECT_DIR}`, plus `${ENV_VAR}` apa pun dari lingkungan. Awali perintah dengan `cd "${CLAUDE_PLUGIN_ROOT}" && ` jika script perlu berjalan dari direktori plugin itu sendiri.

Perintah `command` monitor tidak dapat mereferensikan nilai [`${user_config.*}`](#user-configuration). Perintah berjalan melalui shell, jadi Claude Code menolak monitor dengan [error](/docs/id/errors#plugin-command-references-user-config) daripada mengganti nilai. Proses monitor tidak menerima variabel lingkungan `CLAUDE_PLUGIN_OPTION_<KEY>`, jadi biarkan script monitor membaca nilai dari file config yang dimilikinya. Sebelum v2.1.207, perintah monitor mengganti nilai `${user_config.*}`.

Menonaktifkan plugin di tengah sesi tidak menghentikan monitors yang sudah berjalan. Mereka berhenti saat sesi berakhir.

<h3 id="themes">
  Themes
</h3>

Plugins dapat mengirimkan color themes yang muncul di `/theme` bersama preset bawaan dan themes lokal pengguna. Sebuah theme adalah file JSON di `themes/` dengan preset `base` dan peta `overrides` yang sparse dari color tokens. Themes adalah [komponen eksperimental](#experimental-components).

```json theme={null}
{
  "name": "Dracula",
  "base": "dark",
  "overrides": {
    "claude": "#bd93f9",
    "error": "#ff5555",
    "success": "#50fa7b"
  }
}
```

Memilih plugin theme menyimpan `custom:<plugin-name>:<slug>` di config pengguna. Plugin themes bersifat read-only; menekan `Ctrl+E` pada salah satu di `/theme` menyalinnya ke `~/.claude/themes/` sehingga pengguna dapat mengedit salinannya.

***

<h2 id="plugin-installation-scopes">
  Cakupan instalasi plugin
</h2>

Saat Anda memasang plugin, Anda memilih **cakupan** yang menentukan di mana plugin tersedia dan siapa lagi yang dapat menggunakannya:

| Cakupan   | File pengaturan                                     | Kasus penggunaan                                  |
| :-------- | :-------------------------------------------------- | :------------------------------------------------ |
| `user`    | `~/.claude/settings.json`                           | Plugin pribadi tersedia di semua proyek (default) |
| `project` | `.claude/settings.json`                             | Plugin tim yang dibagikan melalui version control |
| `local`   | `.claude/settings.local.json`                       | Plugin khusus proyek, gitignored                  |
| `managed` | [Pengaturan terkelola](/docs/id/settings#settings-files) | Plugin terkelola (read-only, hanya update)        |

Plugins menggunakan sistem cakupan yang sama dengan konfigurasi Claude Code lainnya. Untuk instruksi instalasi dan flag cakupan, lihat [Pasang plugins](/docs/id/discover-plugins#install-plugins). Untuk penjelasan lengkap tentang cakupan, lihat [Configuration scopes](/docs/id/settings#configuration-scopes).

***

<h2 id="skills-directory-plugins">
  Skills-directory plugins
</h2>

Folder apa pun di bawah direktori skills yang berisi manifest `.claude-plugin/plugin.json` dimuat sebagai plugin bernama `<name>@skills-dir` pada sesi berikutnya, tanpa marketplace dan tanpa langkah instalasi. Scaffold satu dengan [`plugin init`](#plugin-init). Tidak seperti instalasi marketplace, plugin ditemukan di tempat daripada disalin ke cache plugin.

Pohon direktori skills mendukung tiga hal yang berbeda:

| Apa yang Anda miliki                          | Apa itu                                                                                               |
| :-------------------------------------------- | :---------------------------------------------------------------------------------------------------- |
| `<skills-dir>/foo/SKILL.md` tanpa manifest    | Sebuah [skill](/docs/id/skills) biasa bernama `foo`                                                        |
| `<skills-dir>/foo/.claude-plugin/plugin.json` | Plugin `foo@skills-dir`, yang dapat menggabungkan skills, agents, hooks, dan lainnya miliknya sendiri |
| `<plugin>/skills/bar/SKILL.md`                | Skill `bar` yang dikemas di dalam plugin                                                              |

<h3 id="choose-where-the-plugin-loads-from">
  Pilih di mana plugin dimuat
</h3>

| Direktori skills        | Cakupan  | Dimuat                                                                              |
| :---------------------- | :------- | :---------------------------------------------------------------------------------- |
| `~/.claude/skills/`     | personal | Di setiap proyek, karena lokasi hanya milik Anda                                    |
| `<cwd>/.claude/skills/` | project  | Hanya setelah Anda menerima dialog [trust](/docs/id/settings) workspace untuk folder itu |

Plugin cakupan proyek diperiksa ke dalam repositori dan mencapai setiap kolaborator yang mengklonnya. Karena konten itu berasal dari repositori daripada dari Anda, itu dimuat hanya setelah gerbang kepercayaan yang sama yang mengatur `.claude/settings.json`, dan komponen yang menjalankan kode dibatasi lebih lanjut:

* MCP servers yang dideklarasikannya melalui [persetujuan per-server yang sama](/docs/id/mcp) seperti `.mcp.json` proyek
* LSP servers dimulai hanya setelah Anda mempercayai workspace
* [Background monitors](#monitors) tidak dimuat

Plugin cakupan personal tidak memiliki batasan ini.

<Warning>
  Plugin `@skills-dir` cakupan proyek dimuat hanya dari `.claude/skills/` direktori tempat Anda memulai Claude Code. Mereka tidak [berjalan ke root repositori](/docs/id/skills#automatic-discovery-from-parent-and-nested-directories) seperti yang dilakukan skills dan commands biasa, jadi meluncurkan dari subdirektori melewatkan plugin yang tinggal di root repo. Luncurkan dari root repositori, atau jalankan `/reload-plugins` setelah mengubah direktori.
</Warning>

<h3 id="edit-reload-and-disable-a-skills-directory-plugin">
  Edit, reload, dan disable skills-directory plugin
</h3>

Perubahan yang Anda buat pada `SKILL.md` skill berlaku segera dalam sesi saat ini. Perubahan pada komponen plugin lainnya, seperti `hooks/`, `.mcp.json`, `agents/`, dan `output-styles/`, tidak. Jalankan `/reload-plugins` atau restart Claude Code untuk mengambilnya. Lihat [Live change detection](/docs/id/skills#live-change-detection).

Untuk menghentikan loading skills-directory plugin, hapus foldernya atau nonaktifkan berdasarkan nama. Tidak ada langkah `uninstall` karena tidak ada yang dipasang dari marketplace.

```bash theme={null}
claude plugin disable my-tool@skills-dir
```

***

<h2 id="plugin-manifest-schema">
  Skema manifest plugin
</h2>

File `.claude-plugin/plugin.json` mendefinisikan metadata dan konfigurasi plugin Anda. Bagian ini mendokumentasikan semua field dan opsi yang didukung.

Manifest bersifat opsional. Jika dihilangkan, Claude Code secara otomatis menemukan komponen di [lokasi default](#file-locations-reference) dan menurunkan nama plugin dari nama direktori. Gunakan manifest saat Anda perlu memberikan metadata atau jalur komponen khusus.

<h3 id="complete-schema">
  Skema lengkap
</h3>

```json theme={null}
{
  "name": "plugin-name",
  "displayName": "Plugin Name",
  "version": "1.2.0",
  "description": "Brief plugin description",
  "author": {
    "name": "Author Name",
    "email": "author@example.com",
    "url": "https://github.com/author"
  },
  "homepage": "https://docs.example.com/plugin",
  "repository": "https://github.com/author/plugin",
  "license": "MIT",
  "keywords": ["keyword1", "keyword2"],
  "skills": "./custom/skills/",
  "commands": ["./custom/commands/special.md"],
  "agents": ["./custom/agents/reviewer.md"],
  "hooks": "./config/hooks.json",
  "mcpServers": "./mcp-config.json",
  "outputStyles": "./styles/",
  "lspServers": "./.lsp.json",
  "experimental": {
    "themes": "./themes/",
    "monitors": "./monitors.json"
  },
  "dependencies": [
    "helper-lib",
    { "name": "secrets-vault", "version": "~2.1.0" }
  ]
}
```

<h3 id="required-fields">
  Field yang diperlukan
</h3>

Jika Anda menyertakan manifest, `name` adalah satu-satunya field yang diperlukan.

| Field  | Tipe   | Deskripsi                                                                                                                                                                                                                                 | Contoh               |
| :----- | :----- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------- |
| `name` | string | Pengenal unik (kebab-case, tanpa spasi). Saat [entri marketplace](/docs/id/plugin-marketplaces#plugin-entries) mencantumkan plugin dengan nama berbeda, nama entri marketplace adalah yang digunakan oleh kunci `enabledPlugins` dan `/plugin` | `"deployment-tools"` |

Nama ini digunakan untuk namespacing komponen. Misalnya, di UI, agent `agent-creator` untuk plugin dengan nama `plugin-dev` akan muncul sebagai `plugin-dev:agent-creator`.

<h3 id="unrecognized-fields">
  Field yang tidak dikenali
</h3>

Claude Code mengabaikan field tingkat atas yang tidak dikenalinya. Anda dapat menyimpan metadata dari ekosistem lain di `plugin.json` dan plugin masih dimuat. Ini membuat praktis untuk mempertahankan satu manifest yang berfungsi ganda sebagai manifest ekstensi VS Code atau Cursor, `package.json` npm, atau manifest bundle MCPB/DXT.

`claude plugin validate` melaporkan field yang tidak dikenali sebagai peringatan, bukan kesalahan. Jika field adalah satu atau dua karakter dari yang dikenali, peringatan menyarankan nama yang mungkin dimaksudkan. Plugin dengan hanya peringatan field yang tidak dikenali masih lulus validasi dan dimuat saat runtime.

Field dengan tipe yang salah masih gagal. Misalnya, nilai `keywords` yang merupakan string daripada array adalah kesalahan load, dan `claude plugin validate` melaporkannya sebagai satu.

Teruskan `--strict` untuk memperlakukan peringatan sebagai kesalahan. Gunakan di CI untuk menangkap nama field yang salah eja atau field yang tersisa dari manifest tool lain sebelum menerbitkan, meskipun plugin akan dimuat saat runtime.

```bash theme={null}
claude plugin validate ./my-plugin --strict
```

<h3 id="metadata-fields">
  Field metadata
</h3>

| Field            | Tipe    | Deskripsi                                                                                                                                                                                                                                                                                                                                                                       | Contoh                                                            |
| :--------------- | :------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | :---------------------------------------------------------------- |
| `$schema`        | string  | URL JSON Schema untuk autocomplete dan validasi editor. Claude Code mengabaikan field ini saat waktu load.                                                                                                                                                                                                                                                                      | `"https://json.schemastore.org/claude-code-plugin-manifest.json"` |
| `displayName`    | string  | Nama yang dapat dibaca manusia ditampilkan di picker `/plugin` dan permukaan UI lainnya. Kembali ke `name` saat dihilangkan. Tidak seperti `name`, dapat berisi spasi dan casing apa pun. Tidak digunakan untuk namespacing atau lookup. Memerlukan Claude Code v2.1.143 atau lebih baru.                                                                                       | `"Deployment Tools"`                                              |
| `version`        | string  | Opsional. Versi semantik. Mengatur ini mengikat plugin ke string versi tersebut, sehingga pengguna hanya menerima update saat Anda menaikkannya. Jika dihilangkan, Claude Code kembali ke SHA commit git, sehingga setiap commit diperlakukan sebagai versi baru. Jika juga diatur di entri marketplace, `plugin.json` menang. Lihat [Version management](#version-management). | `"2.1.0"`                                                         |
| `description`    | string  | Penjelasan singkat tentang tujuan plugin                                                                                                                                                                                                                                                                                                                                        | `"Deployment automation tools"`                                   |
| `author`         | object  | Informasi penulis                                                                                                                                                                                                                                                                                                                                                               | `{"name": "Dev Team", "email": "dev@company.com"}`                |
| `homepage`       | string  | URL dokumentasi                                                                                                                                                                                                                                                                                                                                                                 | `"https://docs.example.com"`                                      |
| `repository`     | string  | URL kode sumber                                                                                                                                                                                                                                                                                                                                                                 | `"https://github.com/user/plugin"`                                |
| `license`        | string  | Pengenal lisensi                                                                                                                                                                                                                                                                                                                                                                | `"MIT"`, `"Apache-2.0"`                                           |
| `keywords`       | array   | Tag penemuan                                                                                                                                                                                                                                                                                                                                                                    | `["deployment", "ci-cd"]`                                         |
| `defaultEnabled` | boolean | Apakah plugin dimulai dalam keadaan diaktifkan saat pengguna belum menetapkan satu. Default ke `true`. Lihat [Default enablement](#default-enablement). Memerlukan Claude Code v2.1.154 atau lebih baru.                                                                                                                                                                        | `false`                                                           |

<h3 id="default-enablement">
  Default enablement
</h3>

Atur `defaultEnabled: false` di `plugin.json` untuk mengirimkan plugin yang dipasang dalam keadaan dinonaktifkan. Pengguna mengaktifkannya dengan `claude plugin enable <plugin>` atau antarmuka `/plugin`. Gunakan ini untuk plugins yang menambah biaya atau cakupan yang harus pengguna pilih, seperti yang menghubungkan ke layanan eksternal. Ini memerlukan Claude Code v2.1.154 atau lebih baru. Versi sebelumnya mengabaikan field dan mengaktifkan plugin saat instalasi.

`defaultEnabled` adalah fallback saat tidak ada yang lain telah memutuskan status plugin. Dua hal mengambil alih:

* **Pengaturan pengguna**: entri untuk plugin di `enabledPlugins` pada cakupan pengaturan apa pun. Setelah ditulis, itu bertahan di seluruh update dan reinstall plugin, jadi mengubah `defaultEnabled` dalam rilis kemudian tidak membalik pengguna yang ada.
* **Persyaratan dependensi**: ketika plugin diperlukan oleh yang lain yang aktif, Claude Code menulis `true` untuk itu saat waktu instalasi atau enable. Itu memberikannya pengaturan eksplisit, jadi defaultnya tidak lagi berlaku. Lihat [Enable or disable a plugin with dependencies](/docs/id/plugin-dependencies#enable-or-disable-a-plugin-with-dependencies).

Field yang sama dapat muncul dalam entri marketplace plugin, di mana itu mengambil alih nilai di `plugin.json`. Lihat [Optional plugin fields](/docs/id/plugin-marketplaces#optional-plugin-fields).

<h3 id="component-path-fields">
  Field jalur komponen
</h3>

| Field                   | Tipe                  | Deskripsi                                                                                                                                                                    | Contoh                                               |
| :---------------------- | :-------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------- |
| `skills`                | string\|array         | Direktori skill khusus yang berisi `<name>/SKILL.md`. Menambah scan default `skills/`. Lihat [Path behavior rules](#path-behavior-rules) untuk pengecualian marketplace-root | `"./custom/skills/"`                                 |
| `commands`              | string\|array         | File skill `.md` datar atau direktori khusus (menggantikan default `commands/`)                                                                                              | `"./custom/cmd.md"` atau `["./cmd1.md"]`             |
| `agents`                | string\|array         | File agent khusus (menggantikan default `agents/`)                                                                                                                           | `"./custom/agents/reviewer.md"`                      |
| `hooks`                 | string\|array\|object | Jalur konfigurasi hook atau konfigurasi inline                                                                                                                               | `"./my-extra-hooks.json"`                            |
| `mcpServers`            | string\|array\|object | Jalur konfigurasi MCP atau konfigurasi inline                                                                                                                                | `"./my-extra-mcp-config.json"`                       |
| `outputStyles`          | string\|array         | File/direktori gaya output khusus (menggantikan default `output-styles/`)                                                                                                    | `"./styles/"`                                        |
| `lspServers`            | string\|array\|object | Konfigurasi [Language Server Protocol](https://microsoft.github.io/language-server-protocol/) untuk intelijen kode (buka definisi, temukan referensi, dll.)                  | `"./.lsp.json"`                                      |
| `experimental.themes`   | string\|array         | File/direktori tema warna (menggantikan default `themes/`). Lihat [Themes](#themes)                                                                                          | `"./themes/"`                                        |
| `experimental.monitors` | string\|array         | Konfigurasi [Monitor](/docs/id/tools-reference#monitor-tool) latar belakang yang dimulai secara otomatis saat plugin aktif. Lihat [Monitors](#monitors)                           | `"./monitors.json"`                                  |
| `userConfig`            | object                | Nilai yang dapat dikonfigurasi pengguna yang diminta saat enable. Lihat [User configuration](#user-configuration)                                                            | Lihat di bawah                                       |
| `channels`              | array                 | Deklarasi channel untuk message injection (Telegram, Slack, Discord style). Lihat [Channels](#channels)                                                                      | Lihat di bawah                                       |
| `dependencies`          | array                 | Plugin lain yang diperlukan plugin ini, secara opsional dengan batasan versi semver. Lihat [Constrain plugin dependency versions](/docs/id/plugin-dependencies)                   | `[{ "name": "secrets-vault", "version": "~2.1.0" }]` |

<h3 id="experimental-components">
  Komponen eksperimental
</h3>

Komponen di bawah kunci `experimental`, `themes` dan `monitors`, memiliki skema manifest yang mungkin berubah antar rilis saat mereka stabil. Di mana Anda mendeklarasikannya adalah migrasi terpisah: tingkat atas masih berfungsi, `claude plugin validate` memperingatkan, dan rilis mendatang akan memerlukan `experimental.*`.

<h3 id="user-configuration">
  User configuration
</h3>

Field `userConfig` mendeklarasikan nilai yang Claude Code minta dari pengguna saat plugin diaktifkan. Gunakan ini daripada memerlukan pengguna untuk mengedit `settings.json` secara manual.

```json theme={null}
{
  "userConfig": {
    "api_endpoint": {
      "type": "string",
      "title": "API endpoint",
      "description": "Endpoint API tim Anda"
    },
    "api_token": {
      "type": "string",
      "title": "API token",
      "description": "Token autentikasi API",
      "sensitive": true
    }
  }
}
```

Kunci harus berupa pengenal yang valid. Setiap opsi mendukung field berikut:

| Field         | Diperlukan | Deskripsi                                                                                             |
| :------------ | :--------- | :---------------------------------------------------------------------------------------------------- |
| `type`        | Ya         | Salah satu dari `string`, `number`, `boolean`, `directory`, atau `file`                               |
| `title`       | Ya         | Label yang ditampilkan dalam dialog konfigurasi                                                       |
| `description` | Ya         | Teks bantuan yang ditampilkan di bawah field                                                          |
| `sensitive`   | Tidak      | Jika `true`, menyembunyikan input dan menyimpan nilai dalam penyimpanan aman daripada `settings.json` |
| `required`    | Tidak      | Jika `true`, validasi gagal saat field kosong                                                         |
| `default`     | Tidak      | Nilai yang digunakan saat pengguna tidak memberikan apa pun                                           |
| `multiple`    | Tidak      | Untuk tipe `string`, izinkan array string                                                             |
| `min` / `max` | Tidak      | Batas untuk tipe `number`                                                                             |

Setiap nilai tersedia untuk substitusi sebagai `${user_config.KEY}` di konfigurasi MCP dan LSP server dan perintah hook. Nilai non-sensitif juga dapat disubstitusi dalam konten skill dan agent. Semua nilai diekspor ke proses hook sebagai variabel lingkungan `CLAUDE_PLUGIN_OPTION_<KEY>`, di mana `<KEY>` adalah kunci opsi yang dikapitalisasi.

Field yang berjalan dalam shell menolak `${user_config.*}`: mensubstitusi nilai yang dikonfigurasi ke dalam perintah shell akan membiarkan shell menjalankan apa pun yang nilai itu berisi, jadi komponen gagal dengan [error](/docs/id/errors#plugin-command-references-user-config) sebagai gantinya. Setiap field yang ditolak memiliki cara alternatif untuk melewatkan nilai:

| Field yang ditolak                                                           | Cara melewatkan nilai                                                                                                              |
| :--------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------- |
| Perintah hook bentuk shell                                                   | Gunakan [exec form](/docs/id/hooks#exec-form-and-shell-form) dengan `args`, atau baca `CLAUDE_PLUGIN_OPTION_<KEY>` dari lingkungan hook |
| Perintah [Monitor](#monitors)                                                | Baca nilai dari file konfigurasi dalam script                                                                                      |
| MCP [`headersHelper`](/docs/id/mcp#use-dynamic-headers-for-custom-authentication) | Baca nilai dari file konfigurasi dalam script                                                                                      |

Sebelum v2.1.207, field ini mensubstitusi nilai `${user_config.KEY}`; perbarui plugins yang mengandalkan ini.

Nilai non-sensitif disimpan di bawah kunci [`pluginConfigs`](/docs/id/settings#pluginconfigs) di `settings.json` sebagai `pluginConfigs[<plugin-id>].options`. Claude Code menulis kunci ke pengaturan pengguna dan membacanya kembali dari pengaturan pengguna, flag `--settings`, dan pengaturan yang dikelola saja; entri di `.claude/settings.json` atau `.claude/settings.local.json` proyek diabaikan. Sebelum v2.1.207, Claude Code juga membaca pengaturan proyek dan lokal.

Nilai sensitif masuk ke Keychain macOS, atau ke `~/.claude/.credentials.json` di platform di mana keychain yang didukung tidak tersedia. Penyimpanan keychain dibagikan dengan token OAuth dan memiliki batas total sekitar 2 KB, jadi jaga nilai sensitif tetap kecil.

<h3 id="channels">
  Channels
</h3>

Field `channels` memungkinkan plugin mendeklarasikan satu atau lebih message channels yang menyuntikkan konten ke dalam percakapan. Setiap channel mengikat ke MCP server yang disediakan plugin.

```json theme={null}
{
  "channels": [
    {
      "server": "telegram",
      "userConfig": {
        "bot_token": {
          "type": "string",
          "title": "Bot token",
          "description": "Token bot Telegram",
          "sensitive": true
        },
        "owner_id": {
          "type": "string",
          "title": "Owner ID",
          "description": "ID pengguna Telegram Anda"
        }
      }
    }
  ]
}
```

Field `server` diperlukan dan harus cocok dengan kunci di `mcpServers` plugin. Field `userConfig` per-channel opsional menggunakan skema yang sama dengan field tingkat atas, memungkinkan plugin meminta token bot atau ID pemilik saat plugin diaktifkan.

<h3 id="path-behavior-rules">
  Aturan perilaku jalur
</h3>

Apakah jalur khusus menggantikan atau memperluas direktori default plugin tergantung pada field:

* **Menggantikan default**: `commands`, `agents`, `outputStyles`, `experimental.themes`, `experimental.monitors`. Misalnya, saat manifest menentukan `commands`, direktori default `commands/` tidak dipindai. Untuk menyimpan default dan menambahkan lebih banyak, sertakan secara eksplisit: `"commands": ["./commands/", "./extras/"]`
* **Menambah default**: `skills`. Direktori default `skills/` selalu dipindai, dan direktori yang tercantum di `skills` dimuat bersama dengannya. Pengecualian: untuk [entri marketplace yang `source`-nya diselesaikan ke root marketplace](/docs/id/plugin-marketplaces#advanced-plugin-entries), mendeklarasikan subdirektori khusus menggantikan scan default `skills/`
* **Aturan penggabungan sendiri**: [hooks](#hooks), [MCP servers](#mcp-servers), dan [LSP servers](#lsp-servers). Lihat setiap bagian untuk cara beberapa sumber digabungkan

Saat plugin memiliki folder default dan kunci manifest yang cocok, Claude Code v2.1.140 dan yang lebih baru menandai folder yang diabaikan di `claude plugin list` dan tampilan detail `/plugin`. Plugin masih dimuat menggunakan jalur manifest. Tidak ada peringatan yang ditampilkan saat kunci manifest menunjuk ke folder default, misalnya `"commands": ["./commands/deploy.md"]`, karena folder ditangani secara eksplisit dalam hal itu.

Untuk semua field jalur:

* Semua jalur harus relatif terhadap root plugin dan dimulai dengan `./`
* Komponen dari jalur khusus menggunakan aturan penamaan dan namespacing yang sama
* Beberapa jalur dapat ditentukan sebagai array
* Saat jalur skill menunjuk ke direktori yang berisi `SKILL.md` secara langsung, misalnya `"skills": ["./"]` menunjuk ke root plugin, field frontmatter `name` di `SKILL.md` menentukan nama invokasi skill. Ini memberikan nama stabil terlepas dari direktori instalasi. Jika `name` tidak diatur di frontmatter, basename direktori digunakan sebagai fallback.

Plugin yang memiliki `SKILL.md` di root-nya, tidak ada subdirektori `skills/`, dan tidak ada field manifest `skills` secara otomatis dimuat sebagai plugin single-skill di Claude Code v2.1.142 dan yang lebih baru. Anda tidak perlu mengatur `"skills": ["./"]` di `plugin.json` untuk layout ini. Nama invokasi skill mengikuti aturan yang sama seperti di atas: field frontmatter `name`, atau basename direktori sebagai fallback.

**Contoh jalur**:

```json theme={null}
{
  "commands": [
    "./specialized/deploy.md",
    "./utilities/batch-process.md"
  ],
  "agents": [
    "./custom-agents/reviewer.md",
    "./custom-agents/tester.md"
  ]
}
```

<h3 id="environment-variables">
  Variabel lingkungan
</h3>

Claude Code menyediakan tiga variabel untuk mereferensikan jalur:

| Variabel                | Diselesaikan ke                                                                                                       | Gunakan untuk                                                                                                      |
| :---------------------- | :-------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------- |
| `${CLAUDE_PLUGIN_ROOT}` | Jalur absolut ke direktori instalasi plugin                                                                           | Scripts, binaries, dan file konfigurasi yang disertakan dengan plugin                                              |
| `${CLAUDE_PLUGIN_DATA}` | [Direktori persisten](#persistent-data-directory) yang bertahan setelah plugin updates, dibuat pada referensi pertama | Dependensi yang dipasang seperti `node_modules` atau Python virtual environments, kode yang dihasilkan, dan caches |
| `${CLAUDE_PROJECT_DIR}` | Root proyek                                                                                                           | Scripts dan file konfigurasi lokal proyek                                                                          |

Ketiga variabel diekspor sebagai variabel lingkungan ke proses hook dan ke subprocess MCP dan LSP server. Field mana yang mensubstitusi mereka inline tergantung pada komponen plugin:

| Komponen plugin                | Field di mana placeholder diselesaikan      |
| :----------------------------- | :------------------------------------------ |
| Konten skill dan agent         | Di mana pun placeholder muncul              |
| Perintah hook dan monitor      | Di mana pun placeholder muncul              |
| Server MCP `stdio`             | `command`, `args`, `env`                    |
| Server MCP `http`, `sse`, `ws` | `url`, `headers`, `headersHelper`           |
| Server LSP                     | `command`, `args`, `env`, `workspaceFolder` |

Dalam perintah hook, gunakan [exec form](/docs/id/hooks#exec-form-and-shell-form) dengan `args` sehingga setiap jalur dilewatkan sebagai satu argumen tanpa quoting. Dalam hook bentuk shell dan perintah monitor, bungkus variabel dalam tanda kutip ganda, seperti `"${CLAUDE_PROJECT_DIR}/scripts/server.sh"`. Hook bentuk shell ini menjalankan script yang disertakan dengan plugin:

```json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "\"${CLAUDE_PLUGIN_ROOT}\"/scripts/process.sh"
          }
        ]
      }
    ]
  }
}
```

`${CLAUDE_PLUGIN_ROOT}` berubah saat plugin diperbarui. Direktori versi sebelumnya tetap berada di disk selama sekitar tujuh hari setelah update sebelum pembersihan, tetapi perlakukan sebagai ephemeral dan jangan tulis state di sana.

Saat plugin diperbarui di tengah sesi, perintah hook, monitor, MCP server, dan LSP server terus menggunakan jalur versi sebelumnya. Jalankan `/reload-plugins` untuk mengalihkan hooks, MCP server, dan LSP server ke jalur baru; monitor memerlukan restart sesi.

MCP server juga dapat memanggil permintaan `roots/list` untuk membaca direktori kerja sesi saat runtime. Lihat [apa yang dikembalikan `roots/list` dan kapan Claude Code memberi tahu server tentang perubahan](/docs/id/mcp#option-3-add-a-local-stdio-server).

<h4 id="persistent-data-directory">
  Direktori data persisten
</h4>

Direktori `${CLAUDE_PLUGIN_DATA}` diselesaikan ke `~/.claude/plugins/data/{id}/`, di mana `{id}` adalah pengenal plugin dengan karakter di luar `a-z`, `A-Z`, `0-9`, `_`, dan `-` diganti dengan `-`. Untuk plugin yang dipasang sebagai `formatter@my-marketplace`, direktorinya adalah `~/.claude/plugins/data/formatter-my-marketplace/`.

Penggunaan umum adalah memasang dependensi bahasa sekali dan menggunakannya kembali di seluruh sesi dan update plugin. Karena direktori data bertahan lebih lama dari versi plugin tunggal, pemeriksaan keberadaan direktori saja tidak dapat mendeteksi saat update mengubah manifest dependensi plugin. Pola yang direkomendasikan membandingkan manifest yang disertakan terhadap salinan di direktori data dan memasang ulang saat mereka berbeda.

Hook `SessionStart` ini memasang `node_modules` pada run pertama dan lagi kapan pun update plugin menyertakan `package.json` yang berubah:

```json theme={null}
{
  "hooks": {
    "SessionStart": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "diff -q \"${CLAUDE_PLUGIN_ROOT}/package.json\" \"${CLAUDE_PLUGIN_DATA}/package.json\" >/dev/null 2>&1 || (cd \"${CLAUDE_PLUGIN_DATA}\" && cp \"${CLAUDE_PLUGIN_ROOT}/package.json\" . && npm install) || rm -f \"${CLAUDE_PLUGIN_DATA}/package.json\""
          }
        ]
      }
    ]
  }
}
```

`diff` keluar nonzero saat salinan yang disimpan hilang atau berbeda dari yang disertakan, mencakup run pertama dan updates yang mengubah dependensi. Jika `npm install` gagal, trailing `rm` menghapus manifest yang disalin sehingga sesi berikutnya mencoba lagi.

Scripts yang disertakan di `${CLAUDE_PLUGIN_ROOT}` kemudian dapat berjalan terhadap `node_modules` yang persisten:

```json theme={null}
{
  "mcpServers": {
    "routines": {
      "command": "node",
      "args": ["${CLAUDE_PLUGIN_ROOT}/server.js"],
      "env": {
        "NODE_PATH": "${CLAUDE_PLUGIN_DATA}/node_modules"
      }
    }
  }
}
```

Direktori data dihapus secara otomatis saat Anda menghapus plugin dari cakupan terakhir di mana itu dipasang. Antarmuka `/plugin` menunjukkan ukuran direktori dan meminta sebelum menghapus. CLI menghapus secara default; teruskan [`--keep-data`](#plugin-uninstall) untuk mempertahankannya.

***

<h2 id="plugin-caching-and-file-resolution">
  Plugin caching dan resolusi file
</h2>

Plugins ditentukan dalam salah satu dari dua cara:

* Melalui `claude --plugin-dir` atau `claude --plugin-url`, untuk durasi sesi.
* Melalui marketplace, dipasang untuk sesi mendatang.

Untuk tujuan keamanan dan verifikasi, Claude Code menyalin plugin *marketplace* ke **plugin cache** lokal pengguna (`~/.claude/plugins/cache`) daripada menggunakannya di tempat. Memahami perilaku ini penting saat mengembangkan plugins yang mereferensikan file eksternal.

Setiap versi yang dipasang adalah direktori terpisah dalam cache. Saat Anda memperbarui atau menghapus plugin, direktori versi sebelumnya ditandai sebagai orphaned dan dihapus secara otomatis 7 hari kemudian. Periode grace memungkinkan sesi Claude Code bersamaan yang sudah memuat versi lama untuk terus berjalan tanpa kesalahan.

Tools Glob dan Grep Claude melewati direktori versi orphaned selama pencarian, jadi hasil file tidak menyertakan kode plugin yang ketinggalan zaman.

<h3 id="path-traversal-limitations">
  Batasan path traversal
</h3>

Plugin yang dipasang tidak dapat mereferensikan file di luar direktorinya. Jalur yang melintasi di luar root plugin (seperti `../shared-utils`) tidak akan berfungsi setelah instalasi karena file eksternal tersebut tidak disalin ke cache.

<h3 id="share-files-within-a-marketplace-with-symlinks">
  Bagikan file dalam marketplace dengan symlinks
</h3>

Jika plugin Anda perlu berbagi file dengan bagian lain dari marketplace yang sama, Anda dapat membuat symbolic links di dalam direktori plugin Anda. Cara symlink ditangani saat plugin disalin ke cache tergantung pada di mana targetnya diselesaikan:

* **Dalam direktori plugin itu sendiri:** symlink dipertahankan sebagai symlink relatif dalam cache, sehingga terus diselesaikan ke target yang disalin saat runtime.
* **Di tempat lain dalam marketplace yang sama:** symlink didereferensikan. Konten target disalin ke cache di tempatnya. Ini memungkinkan direktori `skills/` meta-plugin untuk menghubungkan ke skills yang ditentukan oleh plugins lain dalam marketplace.
* **Di luar marketplace:** symlink dilewati untuk keamanan. Ini mencegah plugins dari menarik file host arbitrer seperti jalur sistem ke dalam cache.

Untuk plugins yang dipasang dengan `--plugin-dir` atau dari jalur lokal, hanya symlinks yang diselesaikan dalam direktori plugin itu sendiri yang dipertahankan. Semua yang lain dilewati.

Perintah berikut membuat link dari dalam plugin marketplace ke skill bersama yang ditentukan oleh plugin sibling. Di Windows, gunakan `mklink /D` dari Command Prompt yang ditingkatkan atau aktifkan Developer Mode:

```bash theme={null}
ln -s ../../shared-plugin/skills/foo ./skills/foo
```

Ini memberikan fleksibilitas sambil mempertahankan manfaat keamanan dari sistem caching.

***

<h2 id="plugin-directory-structure">
  Struktur direktori plugin
</h2>

<h3 id="standard-plugin-layout">
  Tata letak plugin standar
</h3>

Plugin lengkap mengikuti struktur ini:

```text theme={null}
enterprise-plugin/
├── .claude-plugin/           # Direktori metadata (opsional)
│   └── plugin.json             # plugin manifest
├── skills/                   # Skills
│   ├── code-reviewer/
│   │   └── SKILL.md
│   └── pdf-processor/
│       ├── SKILL.md
│       └── scripts/
├── commands/                 # Skills sebagai file .md datar
│   ├── status.md
│   └── logs.md
├── agents/                   # Definisi subagent
│   ├── security-reviewer.md
│   ├── performance-tester.md
│   └── compliance-checker.md
├── output-styles/            # Definisi gaya output
│   └── terse.md
├── themes/                   # Definisi tema warna
│   └── dracula.json
├── monitors/                 # Konfigurasi monitor latar belakang
│   └── monitors.json
├── hooks/                    # Konfigurasi hooks
│   ├── hooks.json           # Konfigurasi hook utama
│   └── security-hooks.json  # Hook tambahan
├── bin/                      # Plugin executables ditambahkan ke PATH
│   └── my-tool               # Dapat dipanggil sebagai perintah bare di Bash tool
├── settings.json            # Pengaturan default untuk plugin
├── .mcp.json                # Definisi MCP server
├── .lsp.json                # Konfigurasi LSP server
├── scripts/                 # Hook dan utility scripts
│   ├── security-scan.sh
│   ├── format-code.py
│   └── deploy.js
├── LICENSE                  # File lisensi
└── CHANGELOG.md             # Riwayat versi
```

<Warning>
  Direktori `.claude-plugin/` berisi file `plugin.json`. Semua direktori lainnya (commands/, agents/, skills/, output-styles/, themes/, monitors/, hooks/) harus berada di root plugin, bukan di dalam `.claude-plugin/`.
</Warning>

File `CLAUDE.md` di root plugin tidak dimuat sebagai konteks proyek. Plugin berkontribusi konteks melalui skills, agents, dan hooks daripada CLAUDE.md. Untuk mengirimkan instruksi yang dimuat ke dalam konteks Claude, letakkan mereka dalam sebuah [skill](#skills).

<h3 id="file-locations-reference">
  Referensi lokasi file
</h3>

| Komponen          | Lokasi Default               | Tujuan                                                                                                                                                                                    |
| :---------------- | :--------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Manifest**      | `.claude-plugin/plugin.json` | Metadata dan konfigurasi plugin (opsional)                                                                                                                                                |
| **Skills**        | `skills/`                    | Skills dengan struktur `<name>/SKILL.md`                                                                                                                                                  |
| **Commands**      | `commands/`                  | Skills sebagai file Markdown datar. Gunakan `skills/` untuk plugin baru                                                                                                                   |
| **Agents**        | `agents/`                    | File Markdown Subagent                                                                                                                                                                    |
| **Output styles** | `output-styles/`             | Definisi gaya output                                                                                                                                                                      |
| **Themes**        | `themes/`                    | Definisi tema warna                                                                                                                                                                       |
| **Hooks**         | `hooks/hooks.json`           | Konfigurasi hook                                                                                                                                                                          |
| **MCP servers**   | `.mcp.json`                  | Definisi MCP server                                                                                                                                                                       |
| **LSP servers**   | `.lsp.json`                  | Konfigurasi language server                                                                                                                                                               |
| **Monitors**      | `monitors/monitors.json`     | Konfigurasi monitor latar belakang                                                                                                                                                        |
| **Executables**   | `bin/`                       | Executables ditambahkan ke `PATH` Bash tool. File di sini dapat dipanggil sebagai perintah bare di panggilan Bash tool apa pun saat plugin diaktifkan                                     |
| **Settings**      | `settings.json`              | Konfigurasi default yang diterapkan saat plugin diaktifkan. Saat ini hanya kunci [`agent`](/docs/id/sub-agents) dan [`subagentStatusLine`](/docs/id/statusline#subagent-status-lines) yang didukung |

***

<h2 id="cli-commands-reference">
  Referensi perintah CLI
</h2>

Claude Code menyediakan perintah CLI untuk manajemen plugin non-interaktif, berguna untuk scripting dan otomasi.

<h3 id="plugin-init">
  plugin init
</h3>

Scaffold plugin baru di `~/.claude/skills/<name>/`. Pada sesi Claude Code berikutnya itu dimuat secara otomatis sebagai `<name>@skills-dir` dan muncul di `/plugin` dan `claude plugin list` tanpa langkah instalasi.

Lihat [Skills-directory plugins](#skills-directory-plugins) untuk persyaratan cakupan dan kepercayaan.

```bash theme={null}
claude plugin init <name> [options]
```

**Argumen:**

* `<name>`: Nama plugin. Menjadi namespace skill dan nama direktori di bawah `~/.claude/skills/`, jadi tidak dapat berisi spasi atau pemisah jalur.

**Opsi:**

| Opsi                     | Deskripsi                                                                                                             | Default                 |
| :----------------------- | :-------------------------------------------------------------------------------------------------------------------- | :---------------------- |
| `--description <text>`   | Deskripsi manifest                                                                                                    |                         |
| `--author <name>`        | Nama penulis                                                                                                          | `git config user.name`  |
| `--author-email <email>` | Email penulis                                                                                                         | `git config user.email` |
| `--with <components...>` | Juga scaffold folder komponen. Nilai yang valid: `skills`, `agents`, `hooks`, `mcp`, `lsp`, `output-style`, `channel` |                         |
| `-f, --force`            | Timpa `.claude-plugin/` yang ada di target                                                                            |                         |
| `-h, --help`             | Tampilkan bantuan untuk perintah                                                                                      |                         |

**Alias:** `new`

Setiap nilai `--with` menambahkan file starter untuk komponen itu, siap untuk diedit:

| Komponen       | Apa yang di-scaffold                                                                                  |
| :------------- | :---------------------------------------------------------------------------------------------------- |
| `skills`       | Skill `<name>:example` bernama namespace ekstra bersama yang default                                  |
| `agents`       | Definisi subagent `agents/`                                                                           |
| `hooks`        | `hooks/hooks.json` dengan event handler sampel                                                        |
| `mcp`          | `.mcp.json` dengan contoh server HTTP dan stdio                                                       |
| `lsp`          | Contoh `.lsp.json` language-server                                                                    |
| `output-style` | `output-styles/<name>.md` yang berlaku otomatis saat plugin diaktifkan                                |
| `channel`      | [Channel](/docs/id/channels) berbasis MCP: server stdio (`server.ts`), `.mcp.json`-nya, dan `package.json` |

Plugin yang di-scaffold menggunakan sumber `@skills-dir` daripada marketplace. Admin dapat memblokir sumber ini dengan `strictKnownMarketplaces` atau dengan menambahkan `{"source": "skills-dir"}` ke `blockedMarketplaces` dalam [managed settings](/docs/id/plugin-marketplaces#managed-marketplace-restrictions). Saat diblokir, `plugin init` gagal sebelum menulis.

**Contoh:**

```bash theme={null}
# Scaffold plugin minimal
claude plugin init my-helper

# Scaffold dengan folder skill dan hook
claude plugin init my-helper --with skills hooks

# Timpa scaffold yang ada
claude plugin init my-helper --force
```

<h3 id="plugin-install">
  plugin install
</h3>

Pasang plugin dari marketplace yang tersedia.

```bash theme={null}
claude plugin install <plugin> [options]
```

**Argumen:**

* `<plugin>`: Nama plugin atau `plugin-name@marketplace-name` untuk marketplace tertentu

**Opsi:**

| Opsi                  | Deskripsi                                          | Default |
| :-------------------- | :------------------------------------------------- | :------ |
| `-s, --scope <scope>` | Cakupan instalasi: `user`, `project`, atau `local` | `user`  |
| `-h, --help`          | Tampilkan bantuan untuk perintah                   |         |

Cakupan menentukan file pengaturan mana yang ditambahkan plugin yang dipasang. Misalnya, `--scope project` menulis ke `enabledPlugins` di .claude/settings.json, membuat plugin tersedia untuk semua orang yang mengkloning repositori proyek.

**Contoh:**

```bash theme={null}
# Pasang ke cakupan user (default)
claude plugin install formatter@my-marketplace

# Pasang ke cakupan project (dibagikan dengan tim)
claude plugin install formatter@my-marketplace --scope project

# Pasang ke cakupan local (gitignored)
claude plugin install formatter@my-marketplace --scope local
```

<h3 id="plugin-uninstall">
  plugin uninstall
</h3>

Hapus plugin yang dipasang.

```bash theme={null}
claude plugin uninstall <plugin> [options]
```

**Argumen:**

* `<plugin>`: Nama plugin atau `plugin-name@marketplace-name`

**Opsi:**

| Opsi                  | Deskripsi                                                                                                           | Default |
| :-------------------- | :------------------------------------------------------------------------------------------------------------------ | :------ |
| `-s, --scope <scope>` | Hapus dari cakupan: `user`, `project`, atau `local`                                                                 | `user`  |
| `--keep-data`         | Pertahankan [direktori data persisten](#persistent-data-directory) plugin                                           |         |
| `--prune`             | Juga hapus dependensi yang dipasang otomatis yang tidak diperlukan plugin lain. Lihat [plugin prune](#plugin-prune) |         |
| `-y, --yes`           | Lewati prompt konfirmasi `--prune`. Diperlukan ketika stdin atau stdout bukan TTY                                   |         |
| `-h, --help`          | Tampilkan bantuan untuk perintah                                                                                    |         |

**Alias:** `remove`, `rm`

Secara default, menghapus dari cakupan terakhir yang tersisa juga menghapus direktori `${CLAUDE_PLUGIN_DATA}` plugin. Gunakan `--keep-data` untuk mempertahankannya, misalnya saat memasang ulang setelah menguji versi baru.

<h3 id="plugin-prune">
  plugin prune
</h3>

Hapus dependensi plugin yang dipasang otomatis yang tidak lagi diperlukan oleh plugin yang dipasang. Dependensi yang Claude Code tarik untuk memenuhi bidang [`dependencies`](/docs/id/plugin-dependencies) plugin lain dihapus; plugin yang Anda pasang secara langsung tidak pernah disentuh.

```bash theme={null}
claude plugin prune [options]
```

**Opsi:**

| Opsi                  | Deskripsi                                                               | Default |
| :-------------------- | :---------------------------------------------------------------------- | :------ |
| `-s, --scope <scope>` | Prune pada cakupan: `user`, `project`, atau `local`                     | `user`  |
| `--dry-run`           | Daftar apa yang akan dihapus tanpa menghapus apa pun                    |         |
| `-y, --yes`           | Lewati prompt konfirmasi. Diperlukan ketika stdin atau stdout bukan TTY |         |
| `-h, --help`          | Tampilkan bantuan untuk perintah                                        |         |

**Alias:** `autoremove`

Perintah ini mencantumkan dependensi yatim piatu dan meminta konfirmasi sebelum menghapusnya. Untuk menghapus plugin dan membersihkan dependensinya dalam satu langkah, jalankan `claude plugin uninstall <plugin> --prune`.

<Note>
  `claude plugin prune` memerlukan Claude Code v2.1.121 atau lebih baru.
</Note>

<h3 id="plugin-enable">
  plugin enable
</h3>

Aktifkan plugin yang dinonaktifkan. Jika plugin mendeklarasikan [dependencies](/docs/id/plugin-dependencies), Claude Code mengaktifkannya secara transitif pada cakupan yang sama, dan perintah gagal ketika dependensi tidak dipasang.

```bash theme={null}
claude plugin enable <plugin> [options]
```

**Argumen:**

* `<plugin>`: Nama plugin atau `plugin-name@marketplace-name`

**Opsi:**

| Opsi                  | Deskripsi                                                 | Default |
| :-------------------- | :-------------------------------------------------------- | :------ |
| `-s, --scope <scope>` | Cakupan untuk diaktifkan: `user`, `project`, atau `local` | `user`  |
| `-h, --help`          | Tampilkan bantuan untuk perintah                          |         |

<h3 id="plugin-disable">
  plugin disable
</h3>

Nonaktifkan plugin tanpa menghapusnya. Gagal ketika plugin yang diaktifkan lain [bergantung pada](/docs/id/plugin-dependencies#enable-or-disable-a-plugin-with-dependencies) target. Pesan kesalahan mencakup perintah berantai yang menonaktifkan setiap dependensi terlebih dahulu.

```bash theme={null}
claude plugin disable <plugin> [options]
```

**Argumen:**

* `<plugin>`: Nama plugin atau `plugin-name@marketplace-name`

**Opsi:**

| Opsi                  | Deskripsi                                                    | Default |
| :-------------------- | :----------------------------------------------------------- | :------ |
| `-s, --scope <scope>` | Cakupan untuk dinonaktifkan: `user`, `project`, atau `local` | `user`  |
| `-h, --help`          | Tampilkan bantuan untuk perintah                             |         |

<h3 id="plugin-update">
  plugin update
</h3>

Perbarui plugin ke versi terbaru.

```bash theme={null}
claude plugin update <plugin> [options]
```

**Argumen:**

* `<plugin>`: Nama plugin atau `plugin-name@marketplace-name`

**Opsi:**

| Opsi                  | Deskripsi                                                            | Default |
| :-------------------- | :------------------------------------------------------------------- | :------ |
| `-s, --scope <scope>` | Cakupan untuk diperbarui: `user`, `project`, `local`, atau `managed` | `user`  |
| `-h, --help`          | Tampilkan bantuan untuk perintah                                     |         |

***

<h3 id="plugin-list">
  plugin list
</h3>

Daftar plugin yang dipasang dengan versi, marketplace sumber, dan status enable mereka.

```bash theme={null}
claude plugin list [options]
```

**Opsi:**

| Opsi          | Deskripsi                                                           | Default |
| :------------ | :------------------------------------------------------------------ | :------ |
| `--json`      | Output sebagai JSON                                                 |         |
| `--available` | Sertakan plugin yang tersedia dari marketplace. Memerlukan `--json` |         |
| `-h, --help`  | Tampilkan bantuan untuk perintah                                    |         |

Dalam sesi interaktif, `/plugin list` mencetak daftar yang sama secara inline. Bentuk interaktif menerima `--enabled` atau `--disabled` untuk menampilkan hanya plugin dalam status itu, dan `ls` sebagai singkatan untuk `list`.

<h3 id="plugin-details">
  plugin details
</h3>

Tampilkan inventaris komponen plugin dan perkiraan biaya token yang diproyeksikan. Output mencantumkan semua komponen yang disumbangkan plugin, dikelompokkan sebagai Skills, Agents, Hooks, server MCP, dan server LSP, bersama dengan perkiraan berapa banyak token yang ditambahkannya ke setiap sesi. Grup Skills mencakup entri `skills/` dan `commands/`.

```bash theme={null}
claude plugin details <name>
```

**Argumen:**

* `<name>`: Nama plugin atau `plugin-name@marketplace-name`

**Opsi:**

| Opsi         | Deskripsi                        | Default |
| :----------- | :------------------------------- | :------ |
| `-h, --help` | Tampilkan bantuan untuk perintah |         |

Output menampilkan dua angka biaya untuk setiap komponen:

* **Always-on:** token yang ditambahkan ke setiap sesi oleh teks daftar plugin, seperti deskripsi skill, deskripsi agent, dan nama perintah, terlepas dari apakah ada komponen yang diaktifkan.
* **On-invoke:** token yang dihabiskan komponen saat diaktifkan. Ditampilkan per komponen, bukan sebagai total plugin, karena sesi khas hanya mengaktifkan subset komponen.

Contoh ini menunjukkan seperti apa output untuk plugin dengan dua skill:

```
dependency-guard 1.2.0
  Dependency analysis for Claude Code sessions
  Source: dependency-guard@example-marketplace

Component inventory
  Skills (2)  scan-dependencies, review-changes
  Agents (0)
  Hooks (1)  (harness-only — no model context cost)
  MCP servers (0)
  LSP servers (0)

Projected token cost
  Always-on:   ~180 tok   added to every session

Per-component (rounded)
  component            always-on  on-invoke
  scan-dependencies        ~100      ~2400
  review-changes            ~80      ~1800

  On-invoke cost is paid each time a skill or agent fires.
  Token counts are estimates and may differ from actual usage.
```

Total always-on dihitung melalui API `count_tokens` untuk model aktif Anda. Angka per-komponen diskalakan secara proporsional dari total tersebut. Jika API tidak dapat dijangkau, perintah kembali ke perkiraan berbasis karakter.

<h3 id="plugin-tag">
  plugin tag
</h3>

Buat tag rilis git untuk plugin di direktori saat ini. Jalankan dari dalam folder plugin. Lihat [Tag plugin releases](/docs/id/plugin-dependencies#tag-plugin-releases-for-version-resolution).

```bash theme={null}
claude plugin tag [options]
```

**Opsi:**

| Opsi          | Deskripsi                                                 | Default |
| :------------ | :-------------------------------------------------------- | :------ |
| `--push`      | Dorong tag ke remote setelah membuatnya                   |         |
| `--dry-run`   | Cetak apa yang akan diberi tag tanpa membuat tag          |         |
| `-f, --force` | Buat tag bahkan jika pohon kerja kotor atau tag sudah ada |         |
| `-h, --help`  | Tampilkan bantuan untuk perintah                          |         |

***

<h2 id="debugging-and-development-tools">
  Alat debugging dan pengembangan
</h2>

<h3 id="debugging-commands">
  Perintah debugging
</h3>

Gunakan `claude --debug` untuk melihat detail loading plugin:

Ini menunjukkan:

* Plugin mana yang sedang dimuat
* Kesalahan apa pun dalam manifest plugin
* Registrasi skill, agent, dan hook
* Inisialisasi MCP server

<h3 id="common-issues">
  Masalah umum
</h3>

| Masalah                             | Penyebab                       | Solusi                                                                                                                                                                             |
| :---------------------------------- | :----------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Plugin tidak dimuat                 | `plugin.json` tidak valid      | Jalankan `claude plugin validate` atau `/plugin validate` untuk memeriksa `plugin.json`, frontmatter skill/agent/command, dan `hooks/hooks.json` untuk kesalahan sintaks dan skema |
| Skills tidak muncul                 | Struktur direktori salah       | Pastikan `skills/` atau `commands/` di root plugin, bukan di `.claude-plugin/`                                                                                                     |
| Hooks tidak aktif                   | Script tidak dapat dieksekusi  | Jalankan `chmod +x script.sh`                                                                                                                                                      |
| MCP server gagal                    | `${CLAUDE_PLUGIN_ROOT}` hilang | Gunakan variabel untuk semua jalur plugin                                                                                                                                          |
| Kesalahan jalur                     | Jalur absolut digunakan        | Semua jalur harus relatif dan dimulai dengan `./`                                                                                                                                  |
| LSP `Executable not found in $PATH` | Language server tidak dipasang | Pasang biner (misalnya, `npm install -g typescript-language-server typescript`)                                                                                                    |

<h3 id="example-error-messages">
  Contoh pesan kesalahan
</h3>

**Kesalahan validasi manifest**:

* `Invalid JSON syntax: Unexpected token } in JSON at position 142`: periksa koma yang hilang, koma ekstra, atau string yang tidak dikutip
* `Plugin has an invalid manifest file at .claude-plugin/plugin.json. Validation errors: name: Required`: field yang diperlukan hilang
* `Plugin has a corrupt manifest file at .claude-plugin/plugin.json. JSON parse error: ...`: kesalahan sintaks JSON

**Kesalahan loading plugin**:

* `Warning: No commands found in plugin my-plugin custom directory: ./cmds. Expected .md files or SKILL.md in subdirectories.`: jalur command ada tetapi tidak berisi file command yang valid
* `Plugin directory not found at path: ./plugins/my-plugin. Check that the marketplace entry has the correct path.`: jalur `source` di marketplace.json menunjuk ke direktori yang tidak ada
* `Plugin my-plugin has conflicting manifests: both plugin.json and marketplace entry specify components.`: hapus definisi komponen duplikat atau hapus `strict: false` di entri marketplace

<h3 id="hook-troubleshooting">
  Troubleshooting hook
</h3>

**Hook script tidak dieksekusi**:

1. Periksa script dapat dieksekusi: `chmod +x ./scripts/your-script.sh`
2. Verifikasi baris shebang: Baris pertama harus `#!/bin/bash` atau `#!/usr/bin/env bash`
3. Periksa jalur menggunakan `${CLAUDE_PLUGIN_ROOT}`: `"command": "\"${CLAUDE_PLUGIN_ROOT}\"/scripts/your-script.sh"`
4. Uji script secara manual: `./scripts/your-script.sh`

**Hook tidak dipicu pada event yang diharapkan**:

1. Verifikasi nama event benar (case-sensitive): `PostToolUse`, bukan `postToolUse`
2. Periksa pola matcher cocok dengan alat Anda: `"matcher": "Write|Edit"` untuk operasi file
3. Konfirmkan tipe hook valid: `command`, `http`, `mcp_tool`, `prompt`, atau `agent`

<h3 id="mcp-server-troubleshooting">
  Troubleshooting MCP server
</h3>

**Server tidak dimulai**:

1. Periksa command ada dan dapat dieksekusi
2. Verifikasi semua jalur menggunakan variabel `${CLAUDE_PLUGIN_ROOT}`
3. Periksa log MCP server: `claude --debug` menunjukkan kesalahan inisialisasi
4. Uji server secara manual di luar Claude Code

**Alat server tidak muncul**:

1. Pastikan server dikonfigurasi dengan benar di `.mcp.json` atau `plugin.json`
2. Verifikasi server mengimplementasikan protokol MCP dengan benar
3. Periksa timeout koneksi di output debug

<h3 id="directory-structure-mistakes">
  Kesalahan struktur direktori
</h3>

**Gejala**: Plugin dimuat tetapi komponen (skills, agents, hooks) hilang.

**Struktur yang benar**: Komponen harus berada di root plugin, bukan di dalam `.claude-plugin/`. Hanya `plugin.json` yang termasuk di `.claude-plugin/`.

```text theme={null}
my-plugin/
├── .claude-plugin/
│   └── plugin.json      ← Hanya manifest di sini
├── commands/            ← Di level root
├── agents/              ← Di level root
└── hooks/               ← Di level root
```

Jika komponen Anda berada di dalam `.claude-plugin/`, pindahkan ke root plugin.

**Daftar periksa debug**:

1. Jalankan `claude --debug` dan cari pesan "loading plugin"
2. Periksa bahwa setiap direktori komponen terdaftar di output debug
3. Verifikasi izin file memungkinkan membaca file plugin

***

<h2 id="distribution-and-versioning-reference">
  Referensi distribusi dan versioning
</h2>

<h3 id="version-management">
  Manajemen versi
</h3>

Claude Code menggunakan versi plugin sebagai cache key yang menentukan apakah pembaruan tersedia. Ketika Anda menjalankan `/plugin update` atau auto-update dipicu, Claude Code menghitung versi saat ini dan melewati pembaruan jika cocok dengan apa yang sudah terpasang.

Versi diselesaikan dari yang pertama dari ini yang diatur:

1. Field `version` dalam `plugin.json` plugin
2. Field `version` dalam entri marketplace plugin dalam `marketplace.json`
3. Git commit SHA dari sumber plugin, untuk sumber `github`, `url`, `git-subdir`, dan relative-path dalam marketplace yang dihosting git
4. `unknown`, untuk sumber `npm` atau direktori lokal yang tidak berada dalam repositori git

Ini memberi Anda dua cara untuk memberi versi pada plugin:

| Pendekatan           | Cara                                                         | Perilaku pembaruan                                                                                                                                                                                 | Terbaik untuk                                         |
| :------------------- | :----------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------- |
| **Versi eksplisit**  | Atur `"version": "2.1.0"` dalam `plugin.json`                | Pengguna mendapatkan pembaruan hanya ketika Anda menaikkan field ini. Mendorong commit baru tanpa menaikkannya tidak berpengaruh, dan `/plugin update` melaporkan "already at the latest version". | Plugin yang dipublikasikan dengan siklus rilis stabil |
| **Versi commit-SHA** | Hilangkan `version` dari `plugin.json` dan entri marketplace | Pengguna mendapatkan pembaruan pada setiap commit baru ke sumber git plugin                                                                                                                        | Plugin internal atau tim di bawah pengembangan aktif  |

<Warning>
  Jika Anda mengatur `version` dalam `plugin.json`, Anda harus menaikkannya setiap kali Anda ingin pengguna menerima perubahan. Mendorong commit baru saja tidak cukup, karena Claude Code melihat string versi yang sama dan menyimpan salinan yang di-cache. Jika Anda melakukan iterasi dengan cepat, biarkan `version` tidak diatur sehingga git commit SHA digunakan sebagai gantinya.
</Warning>

Jika Anda menggunakan versi eksplisit, ikuti [semantic versioning](https://semver.org) (`MAJOR.MINOR.PATCH`): naikkan MAJOR untuk perubahan breaking, MINOR untuk fitur baru, PATCH untuk perbaikan bug. Dokumentasikan perubahan dalam `CHANGELOG.md`.

***

<h2 id="see-also">
  Lihat juga
</h2>

* [Plugins](/docs/id/plugins) - Tutorial dan penggunaan praktis
* [Plugin marketplaces](/docs/id/plugin-marketplaces) - Membuat dan mengelola marketplace
* [Skills](/docs/id/skills) - Detail pengembangan skill
* [Subagents](/docs/id/sub-agents) - Konfigurasi dan kemampuan agent
* [Hooks](/docs/id/hooks) - Penanganan event dan otomasi
* [MCP](/docs/id/mcp) - Integrasi alat eksternal
* [Settings](/docs/id/settings) - Opsi konfigurasi untuk plugins
