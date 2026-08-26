> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Referensi hooks

> Referensi untuk event hook Claude Code, skema konfigurasi, format JSON input/output, kode keluar, hooks asinkron, hooks HTTP, prompt hooks, dan MCP tool hooks.

<Tip>
  Untuk panduan quickstart dengan contoh, lihat [Otomatisasi alur kerja dengan hooks](/docs/id/hooks-guide).
</Tip>

Hooks adalah perintah shell yang ditentukan pengguna, endpoint HTTP, atau prompt LLM yang dijalankan secara otomatis pada titik-titik tertentu dalam siklus hidup Claude Code. Gunakan referensi ini untuk mencari skema event, opsi konfigurasi, format JSON input/output, dan fitur lanjutan seperti async hooks, HTTP hooks, dan MCP tool hooks. Jika Anda menyiapkan hooks untuk pertama kalinya, mulai dengan [panduan](/docs/id/hooks-guide) sebagai gantinya.

<h2 id="hook-lifecycle">
  Siklus hidup hook
</h2>

Hooks dijalankan pada titik-titik tertentu selama sesi Claude Code. Ketika event dijalankan dan matcher cocok, Claude Code meneruskan konteks JSON tentang event ke handler hook Anda. Untuk command hooks, input tiba di stdin. Untuk HTTP hooks, input tiba sebagai badan permintaan POST. Handler Anda kemudian dapat memeriksa input, mengambil tindakan, dan secara opsional mengembalikan keputusan.

Events jatuh ke dalam tiga cadence:

* sekali per sesi: `SessionStart` dan `SessionEnd`
* sekali per turn: `UserPromptSubmit`, `Stop`, dan `StopFailure`
* pada setiap pemanggilan tool di dalam loop agentic: `PreToolUse` dan `PostToolUse`

<div style={{maxWidth: "500px", margin: "0 auto"}}>
  <Frame>
    <img src="https://mintcdn.com/claude-code/jhXrDR5TrSZ5hgXM/images/hooks-lifecycle.svg?fit=max&auto=format&n=jhXrDR5TrSZ5hgXM&q=85&s=3ca47113d5956460e6e4611b8dbc63b7" alt="Diagram siklus hidup hook menunjukkan Setup opsional yang mengalir ke SessionStart, kemudian loop per-turn yang berisi UserPromptSubmit, UserPromptExpansion untuk slash commands, loop agentic bersarang (PreToolUse, PermissionRequest, PostToolUse, PostToolUseFailure, PostToolBatch, SubagentStart/Stop, TaskCreated, TaskCompleted), dan Stop atau StopFailure, diikuti TeammateIdle, PreCompact, PostCompact, dan SessionEnd, dengan Elicitation dan ElicitationResult bersarang di dalam eksekusi MCP tool, PermissionDenied sebagai cabang samping dari PermissionRequest untuk penolakan mode otomatis, WorktreeCreate, WorktreeRemove, Notification, ConfigChange, InstructionsLoaded, CwdChanged, dan FileChanged sebagai event asinkron mandiri, dan MessageDisplay sebagai event display-only yang berjalan saat teks pesan asisten streaming" width="520" height="1228" data-path="images/hooks-lifecycle.svg" />
  </Frame>
</div>

Tabel di bawah merangkum kapan setiap event dijalankan. Bagian [Hook events](#hook-events) mendokumentasikan skema input lengkap dan opsi kontrol keputusan untuk masing-masing.

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

<h3 id="how-a-hook-resolves">
  Bagaimana hook diselesaikan
</h3>

Untuk melihat bagaimana potongan-potongan ini cocok bersama, pertimbangkan hook `PreToolUse` ini yang memblokir perintah shell yang merusak. `matcher` mempersempit ke pemanggilan tool Bash dan kondisi `if` mempersempit lebih lanjut ke perintah Bash yang cocok dengan `rm *`, jadi `block-rm.sh` hanya spawn ketika kedua filter cocok:

```json theme={null}
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "Bash",
        "hooks": [
          {
            "type": "command",
            "if": "Bash(rm *)",
            "command": "${CLAUDE_PROJECT_DIR}/.claude/hooks/block-rm.sh",
            "args": []
          }
        ]
      }
    ]
  }
}
```

Skrip membaca input JSON dari stdin, mengekstrak perintah, dan mengembalikan `permissionDecision` dari `"deny"` jika berisi `rm -rf`:

```bash theme={null}
#!/bin/bash
# .claude/hooks/block-rm.sh
COMMAND=$(jq -r '.tool_input.command')

if echo "$COMMAND" | grep -q 'rm -rf'; then
  jq -n '{
    hookSpecificOutput: {
      hookEventName: "PreToolUse",
      permissionDecision: "deny",
      permissionDecisionReason: "Destructive command blocked by hook"
    }
  }'
else
  exit 0  # no decision; normal permission flow applies
fi
```

Sekarang anggaplah Claude Code memutuskan untuk menjalankan `Bash "rm -rf /tmp/build"`. Inilah yang terjadi:

<Frame>
  <img src="https://mintcdn.com/claude-code/ikqp3_70mqIahteV/images/hook-resolution.svg?fit=max&auto=format&n=ikqp3_70mqIahteV&q=85&s=be0bf3053550c26de5f54cd64674c197" alt="Diagram resolusi hook: event PreToolUse dijalankan, matcher memeriksa kecocokan Bash, kemudian kondisi if memeriksa kecocokan Bash(rm *). Jika keduanya cocok, perintah hook dijalankan dan mengembalikan permissionDecision deny, jadi pemanggilan tool diblokir dan Claude Code melanjutkan. Jika salah satu pemeriksaan gagal cocok, hook dilewati dan pemanggilan tool diizinkan untuk melanjutkan." width="930" height="270" data-path="images/hook-resolution.svg" />
</Frame>

<Steps>
  <Step title="Event dijalankan">
    Event `PreToolUse` dijalankan. Claude Code mengirimkan input tool sebagai JSON di stdin ke hook:

    ```json theme={null}
    { "tool_name": "Bash", "tool_input": { "command": "rm -rf /tmp/build" }, ... }
    ```
  </Step>

  <Step title="Matcher memeriksa">
    Matcher `"Bash"` cocok dengan nama tool, jadi grup hook ini diaktifkan. Jika Anda menghilangkan matcher atau menggunakan `"*"`, grup diaktifkan pada setiap kemunculan event.
  </Step>

  <Step title="Kondisi if memeriksa">
    Kondisi `if` `"Bash(rm *)"` cocok karena `rm -rf /tmp/build` adalah subperintah yang cocok dengan `rm *`, jadi handler ini spawn. Jika perintah telah `npm test`, pemeriksaan `if` akan gagal dan `block-rm.sh` tidak akan pernah dijalankan, menghindari overhead spawn proses. Bidang `if` bersifat opsional; tanpanya, setiap handler dalam grup yang cocok dijalankan.
  </Step>

  <Step title="Handler hook dijalankan">
    Skrip memeriksa perintah lengkap dan menemukan `rm -rf`, jadi itu mencetak keputusan ke stdout:

    ```json theme={null}
    {
      "hookSpecificOutput": {
        "hookEventName": "PreToolUse",
        "permissionDecision": "deny",
        "permissionDecisionReason": "Destructive command blocked by hook"
      }
    }
    ```

    Jika perintah telah menjadi varian `rm` yang lebih aman seperti `rm file.txt`, skrip akan mencapai `exit 0` sebagai gantinya. Kode keluar 0 tanpa output berarti hook tidak memiliki keputusan untuk dilaporkan, jadi pemanggilan tool berlanjut melalui [alur izin](/docs/id/permissions) normal. Hook dapat menolak pemanggilan, tetapi tetap diam tidak menyetujuinya.
  </Step>

  <Step title="Claude Code bertindak atas hasil">
    Claude Code membaca keputusan JSON, memblokir pemanggilan tool, dan menunjukkan Claude alasannya.
  </Step>
</Steps>

Bagian [Configuration](#configuration) di bawah mendokumentasikan skema lengkap, dan setiap bagian [hook event](#hook-events) mendokumentasikan input apa yang diterima perintah Anda dan output apa yang dapat dikembalikan.

<h2 id="configuration">
  Konfigurasi
</h2>

Hooks didefinisikan dalam file pengaturan JSON. Konfigurasi memiliki tiga tingkat nesting:

1. Pilih [hook event](#hook-events) untuk merespons, seperti `PreToolUse` atau `Stop`
2. Tambahkan [matcher group](#matcher-patterns) untuk memfilter kapan dijalankan, seperti "hanya untuk tool Bash"
3. Tentukan satu atau lebih [hook handlers](#hook-handler-fields) untuk dijalankan saat cocok

Lihat [Bagaimana hook diselesaikan](#how-a-hook-resolves) di atas untuk panduan lengkap dengan contoh beranotasi.

<Note>
  Halaman ini menggunakan istilah spesifik untuk setiap tingkat: **hook event** untuk titik siklus hidup, **matcher group** untuk filter, dan **hook handler** untuk perintah shell, endpoint HTTP, tool MCP, prompt, atau agent yang dijalankan. "Hook" sendiri merujuk pada fitur umum.
</Note>

<h3 id="hook-locations">
  Lokasi hook
</h3>

Tempat Anda mendefinisikan hook menentukan cakupannya:

| Lokasi                                                       | Cakupan                  | Dapat Dibagikan                               |
| :----------------------------------------------------------- | :----------------------- | :-------------------------------------------- |
| `~/.claude/settings.json`                                    | Semua proyek Anda        | Tidak, lokal ke mesin Anda                    |
| `.claude/settings.json`                                      | Proyek tunggal           | Ya, dapat dikomit ke repo                     |
| `.claude/settings.local.json`                                | Proyek tunggal           | Tidak, gitignored saat Claude Code membuatnya |
| Pengaturan kebijakan terkelola                               | Seluruh organisasi       | Ya, dikendalikan admin                        |
| [Plugin](/docs/id/plugins) `hooks/hooks.json`                     | Ketika plugin diaktifkan | Ya, dibundel dengan plugin                    |
| [Skill](/docs/id/skills) atau [agent](/docs/id/sub-agents) frontmatter | Saat komponen aktif      | Ya, didefinisikan dalam file komponen         |

Untuk detail tentang resolusi file pengaturan, lihat [settings](/docs/id/settings). Administrator enterprise dapat menggunakan `allowManagedHooksOnly` untuk memblokir hooks pengguna, proyek, dan plugin. Hooks dari plugins yang dipaksa-aktifkan dalam pengaturan terkelola `enabledPlugins` dikecualikan, jadi administrator dapat mendistribusikan hooks yang telah diverifikasi melalui marketplace organisasi. Lihat [Hook configuration](/docs/id/settings#hook-configuration).

<h3 id="matcher-patterns">
  Pola matcher
</h3>

Bidang `matcher` memfilter kapan hooks dijalankan. Bagaimana matcher dievaluasi tergantung pada karakter yang dikandungnya:

| Nilai matcher                                      | Dievaluasi sebagai                                                                                                 | Contoh                                                                                                                                                                          |
| :------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `"*"`, `""`, atau dihilangkan                      | Cocokkan semua                                                                                                     | dijalankan pada setiap kemunculan event                                                                                                                                         |
| Hanya huruf, digit, `_`, `-`, spasi, `,`, dan `\|` | String yang tepat, atau daftar string yang tepat dipisahkan `\|` atau `,` dengan whitespace opsional di sekitarnya | `Bash` cocok hanya dengan tool Bash; `Edit\|Write` dan `Edit, Write` masing-masing cocok dengan salah satu tool dengan tepat; `code-reviewer` cocok hanya dengan tipe agent itu |
| Berisi karakter lain apa pun                       | Ekspresi reguler JavaScript, tidak berlabuh                                                                        | `^Notebook` cocok dengan tool apa pun yang dimulai dengan Notebook; `mcp__memory__.*` cocok dengan setiap tool dari server `memory`                                             |

Matcher pada jalur ekspresi reguler diuji dengan `RegExp.prototype.test` JavaScript, yang berhasil pada kecocokan di mana pun dalam nilai. `Edit.*` cocok dengan `Edit` dan `NotebookEdit`; bungkus pola dalam `^` dan `$`, seperti `^Edit$`, ketika Anda memerlukan kecocokan seluruh string.

Pemisah koma dan toleransi whitespace di sekitarnya memerlukan Claude Code v2.1.191 atau lebih baru.

Tanda hubung dalam set exact-match memerlukan Claude Code v2.1.195 atau lebih baru. Pada versi sebelumnya, nama dengan tanda hubung seperti `code-reviewer` dievaluasi sebagai ekspresi reguler yang tidak berlabuh, jadi juga dijalankan untuk `senior-code-reviewer`; labuhnya sebagai `^code-reviewer$` pada versi tersebut untuk mencocokkan hanya nama itu.

`FileChanged` dan `StopFailure` menggunakan set exact-match yang lebih sempit dari huruf, digit, `_`, dan `|` saja. Tanda hubung, spasi, atau koma dalam matcher untuk dua event itu membuat tetap pada jalur ekspresi reguler, dan hanya `|` yang memisahkan alternatif. Setiap event lain dengan dukungan matcher dalam tabel yang mengikuti menerima `|` atau `,`.

Event `FileChanged` tidak mengikuti aturan ini saat membangun daftar watch-nya. Lihat [FileChanged](#filechanged).

Setiap tipe event cocok pada bidang yang berbeda:

| Event                                                                                                                                             | Apa yang difilter matcher                                            | Contoh nilai matcher                                                                                                                                                                |
| :------------------------------------------------------------------------------------------------------------------------------------------------ | :------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `PreToolUse`, `PostToolUse`, `PostToolUseFailure`, `PermissionRequest`, `PermissionDenied`                                                        | nama tool                                                            | `Bash`, `Edit\|Write`, `mcp__.*`                                                                                                                                                    |
| `SessionStart`                                                                                                                                    | bagaimana sesi dimulai                                               | `startup`, `resume`, `clear`, `compact`                                                                                                                                             |
| `Setup`                                                                                                                                           | flag CLI mana yang memicu setup                                      | `init`, `maintenance`                                                                                                                                                               |
| `SessionEnd`                                                                                                                                      | mengapa sesi berakhir                                                | `clear`, `resume`, `logout`, `prompt_input_exit`, `bypass_permissions_disabled`, `other`                                                                                            |
| `Notification`                                                                                                                                    | tipe notifikasi                                                      | `permission_prompt`, `idle_prompt`, `auth_success`, `elicitation_dialog`, `elicitation_complete`, `elicitation_response`, `agent_needs_input`, `agent_completed`                    |
| `SubagentStart`                                                                                                                                   | tipe agent                                                           | `general-purpose`, `Explore`, `Plan`, nama agent kustom, atau nama dengan cakupan plugin seperti `^my-plugin:reviewer$`                                                             |
| `PreCompact`, `PostCompact`                                                                                                                       | apa yang memicu compaction                                           | `manual`, `auto`                                                                                                                                                                    |
| `SubagentStop`                                                                                                                                    | tipe agent                                                           | nilai yang sama seperti `SubagentStart`                                                                                                                                             |
| `ConfigChange`                                                                                                                                    | sumber konfigurasi                                                   | `user_settings`, `project_settings`, `local_settings`, `policy_settings`, `skills`                                                                                                  |
| `CwdChanged`                                                                                                                                      | tidak ada dukungan matcher                                           | selalu dijalankan pada setiap perubahan direktori                                                                                                                                   |
| `FileChanged`                                                                                                                                     | nama file literal untuk ditonton (lihat [FileChanged](#filechanged)) | `.envrc\|.env`                                                                                                                                                                      |
| `StopFailure`                                                                                                                                     | tipe kesalahan                                                       | `rate_limit`, `overloaded`, `authentication_failed`, `oauth_org_not_allowed`, `billing_error`, `invalid_request`, `model_not_found`, `server_error`, `max_output_tokens`, `unknown` |
| `InstructionsLoaded`                                                                                                                              | alasan load                                                          | `session_start`, `nested_traversal`, `path_glob_match`, `include`, `compact`                                                                                                        |
| `UserPromptExpansion`                                                                                                                             | nama command                                                         | nama skill atau command Anda                                                                                                                                                        |
| `Elicitation`                                                                                                                                     | nama server MCP                                                      | nama server MCP yang dikonfigurasi Anda                                                                                                                                             |
| `ElicitationResult`                                                                                                                               | nama server MCP                                                      | nilai yang sama seperti `Elicitation`                                                                                                                                               |
| `UserPromptSubmit`, `PostToolBatch`, `Stop`, `TeammateIdle`, `TaskCreated`, `TaskCompleted`, `WorktreeCreate`, `WorktreeRemove`, `MessageDisplay` | tidak ada dukungan matcher                                           | selalu dijalankan pada setiap kemunculan                                                                                                                                            |

Matcher dijalankan terhadap bidang dari [JSON input](#hook-input-and-output) yang Claude Code kirimkan ke hook Anda di stdin. Untuk tool events, bidang itu adalah `tool_name`. Setiap bagian [hook event](#hook-events) mencantumkan set lengkap nilai matcher dan skema input untuk event itu.

Contoh ini menjalankan skrip linting hanya ketika Claude menulis atau mengedit file:

```json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Edit|Write",
        "hooks": [
          {
            "type": "command",
            "command": "/path/to/lint-check.sh"
          }
        ]
      }
    ]
  }
}
```

`UserPromptSubmit`, `PostToolBatch`, `Stop`, `TeammateIdle`, `TaskCreated`, `TaskCompleted`, `WorktreeCreate`, `WorktreeRemove`, `MessageDisplay`, dan `CwdChanged` tidak mendukung matchers dan selalu dijalankan pada setiap kemunculan. Jika Anda menambahkan bidang `matcher` ke event ini, itu akan diabaikan secara diam-diam.

Untuk tool events, Anda dapat memfilter lebih sempit dengan menetapkan bidang [`if`](#common-fields) pada handler hook individual. `if` menggunakan [sintaks aturan izin](/docs/id/permissions) untuk mencocokkan terhadap nama tool dan argumen bersama-sama, jadi `"Bash(git *)"` dijalankan ketika subperintah apa pun dari input Bash cocok dengan `git *` dan `"Edit(*.ts)"` dijalankan hanya untuk file TypeScript.

<h4 id="match-mcp-tools">
  Cocokkan MCP tools
</h4>

Tool server [MCP](/docs/id/mcp) muncul sebagai tool reguler dalam tool events (`PreToolUse`, `PostToolUse`, `PostToolUseFailure`, `PermissionRequest`, `PermissionDenied`), jadi Anda dapat mencocokkannya dengan cara yang sama seperti Anda mencocokkan nama tool lainnya.

MCP tools mengikuti pola penamaan `mcp__<server>__<tool>`, misalnya:

* `mcp__memory__create_entities`: tool create entities dari Memory server
* `mcp__filesystem__read_file`: tool read file dari Filesystem server
* `mcp__github__search_repositories`: tool search dari GitHub server

Untuk mencocokkan setiap tool dari server, tambahkan `.*` ke awalan server. `.*` diperlukan: matcher seperti `mcp__memory` atau `mcp__brave-search` hanya berisi karakter exact-match, jadi dibandingkan sebagai string yang tepat dan tidak cocok dengan tool apa pun.

* `mcp__memory__.*` cocok dengan semua tools dari server `memory`
* `mcp__brave-search__.*` cocok dengan semua tools dari server yang namanya berisi tanda hubung
* `mcp__.*__write.*` cocok dengan tool apa pun yang namanya dimulai dengan `write` dari server apa pun

Tanda hubung dalam set exact-match memerlukan Claude Code v2.1.195 atau lebih baru. Pada versi sebelumnya, awalan bare dengan tanda hubung seperti `mcp__brave-search` dievaluasi sebagai ekspresi reguler yang tidak berlabuh dan cocok dengan setiap tool dari server itu. Bentuk `mcp__brave-search__.*` bekerja pada setiap versi.

Tools dari [plugin-bundled MCP server](/docs/id/mcp#plugin-provided-mcp-servers) menggunakan segmen server yang dibatasi yang mencakup nama plugin: `mcp__plugin_<plugin-name>_<server-name>__<tool>`. Matcher yang ditulis terhadap kunci server bare tidak pernah dijalankan untuk tools ini. Untuk plugin bernama `my-plugin` yang membundel server di bawah kunci `db`, tool `query` muncul sebagai `mcp__plugin_my-plugin_db__query`, jadi matcher untuk setiap tool dari server itu adalah `mcp__plugin_my-plugin_db__.*`. Gunakan nama tool yang dibatasi yang sama dalam bidang [`if`](#common-fields) handler. Lihat [Plugin-provided MCP servers](/docs/id/mcp#plugin-provided-mcp-servers) untuk bagaimana nama yang dibatasi dibangun.

Contoh ini mencatat semua operasi memory server dan memvalidasi operasi write dari server MCP apa pun:

```json theme={null}
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "mcp__memory__.*",
        "hooks": [
          {
            "type": "command",
            "command": "echo 'Memory operation initiated' >> ~/mcp-operations.log"
          }
        ]
      },
      {
        "matcher": "mcp__.*__write.*",
        "hooks": [
          {
            "type": "command",
            "command": "/home/user/scripts/validate-mcp-write.py"
          }
        ]
      }
    ]
  }
}
```

<h3 id="hook-handler-fields">
  Bidang hook handler
</h3>

Setiap objek dalam array `hooks` inner adalah hook handler: perintah shell, endpoint HTTP, tool MCP, prompt LLM, atau agent yang dijalankan saat matcher cocok. Ada lima tipe:

* **[Command hooks](#command-hook-fields)** (`type: "command"`): jalankan perintah shell. Skrip Anda menerima [JSON input](#hook-input-and-output) event di stdin dan mengkomunikasikan hasil kembali melalui kode keluar dan stdout.
* **[HTTP hooks](#http-hook-fields)** (`type: "http"`): kirimkan JSON input event sebagai permintaan HTTP POST ke URL. Endpoint mengkomunikasikan hasil kembali melalui badan respons menggunakan [format JSON output](#json-output) yang sama seperti command hooks.
* **[MCP tool hooks](#mcp-tool-hook-fields)** (`type: "mcp_tool"`): panggil tool pada [MCP server](/docs/id/mcp) yang sudah terhubung. Output teks tool diperlakukan seperti command-hook stdout.
* **[Prompt hooks](#prompt-and-agent-hook-fields)** (`type: "prompt"`): kirimkan prompt ke model Claude untuk evaluasi single-turn. Model mengembalikan keputusan yes/no sebagai JSON. Lihat [Prompt-based hooks](#prompt-based-hooks).
* **[Agent hooks](#prompt-and-agent-hook-fields)** (`type: "agent"`): spawn subagent yang dapat menggunakan tools seperti Read, Grep, dan Glob untuk memverifikasi kondisi sebelum mengembalikan keputusan. Agent hooks adalah eksperimental dan mungkin berubah. Lihat [Agent-based hooks](#agent-based-hooks).

Semua matching hooks dijalankan secara paralel, dan handler identik dideduplikasi secara otomatis. Command hooks dideduplikasi berdasarkan string perintah dan `args`, dan HTTP hooks dideduplikasi berdasarkan URL.

Handlers dijalankan di direktori saat ini dengan lingkungan Claude Code. Variabel lingkungan `$CLAUDE_CODE_REMOTE` diatur ke `"true"` di lingkungan web jarak jauh dan tidak diatur di CLI lokal. Mulai dari v2.1.199, [`$CLAUDE_CODE_BRIDGE_SESSION_ID`](/docs/id/env-vars) diatur ke [Remote Control](/docs/id/remote-control) session ID saat sesi lokal memiliki koneksi Remote Control yang aktif.

<h4 id="common-fields">
  Bidang umum
</h4>

Bidang-bidang ini berlaku untuk semua tipe hook:

| Bidang          | Diperlukan | Deskripsi                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| :-------------- | :--------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `type`          | ya         | `"command"`, `"http"`, `"mcp_tool"`, `"prompt"`, atau `"agent"`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `if`            | tidak      | Sintaks aturan izin untuk memfilter kapan hook ini dijalankan, seperti `"Bash(git *)"` atau `"Edit(*.ts)"`. Hook command hanya dijalankan jika pemanggilan tool cocok dengan pola. Lihat tabel [Bash matching](#bash-if-matching) di bawah untuk bagaimana pola Bash dievaluasi terhadap subperintah, `$()`, dan backticks. Hanya dievaluasi pada tool events: `PreToolUse`, `PostToolUse`, `PostToolUseFailure`, `PermissionRequest`, dan `PermissionDenied`. Pada event lain, hook dengan `if` yang ditetapkan tidak akan pernah dijalankan. Menggunakan sintaks yang sama seperti [aturan izin](/docs/id/permissions) |
| `timeout`       | tidak      | Detik sebelum membatalkan. Default: 600 untuk `command`, `http`, dan `mcp_tool`; 30 untuk `prompt`; 60 untuk `agent`. [`UserPromptSubmit`](#userpromptsubmit) menurunkan default `command`, `http`, dan `mcp_tool` menjadi 30, dan [`MessageDisplay`](#messagedisplay) menurunkannya menjadi 10                                                                                                                                                                                                                                                                                                                     |
| `statusMessage` | tidak      | Pesan spinner kustom ditampilkan saat hook dijalankan                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `once`          | tidak      | Jika `true`, dijalankan hanya sekali per sesi kemudian dihapus. Hanya dihormati untuk hooks yang dideklarasikan dalam [skill frontmatter](#hooks-in-skills-and-agents); diabaikan dalam file pengaturan dan agent frontmatter                                                                                                                                                                                                                                                                                                                                                                                       |

Bidang `if` menyimpan tepat satu aturan izin. Tidak ada sintaks `&&`, `||`, atau list untuk menggabungkan aturan; untuk menerapkan beberapa kondisi, tentukan handler hook terpisah untuk masing-masing.

<span id="bash-if-matching" />Untuk pola Bash, apakah hook command Anda dijalankan tergantung pada bentuk pola dan perintah Bash yang Claude panggil. Penugasan `VAR=value` terkemuka dihapus sebelum pencocokan.

| Pola `if`          | Perintah Bash          | Hook dijalankan? | Mengapa                                                                                                        |
| :----------------- | :--------------------- | :--------------- | :------------------------------------------------------------------------------------------------------------- |
| `Bash(git *)`      | `FOO=bar git push`     | ya               | penugasan terkemuka dihapus; `git push` cocok                                                                  |
| `Bash(git *)`      | `npm test && git push` | ya               | setiap subperintah diperiksa; `git push` cocok                                                                 |
| `Bash(rm *)`       | `echo $(rm -rf /)`     | ya               | perintah di dalam `$()` dan backticks diperiksa; `rm -rf /` cocok                                              |
| `Bash(rm *)`       | `echo $(date)`         | tidak            | tidak ada subperintah yang cocok dengan `rm *`                                                                 |
| `Bash(git push *)` | `echo $(date)`         | ya               | pola yang menentukan lebih dari nama perintah menjalankan hook bagaimanapun pada `$()`, backticks, atau `$VAR` |

Filter juga gagal terbuka, menjalankan hook Anda terlepas dari pola, ketika perintah Bash tidak dapat diurai. Karena filter `if` adalah best-effort, gunakan [sistem izin](/docs/id/permissions) daripada hook untuk memberlakukan allow atau deny yang keras.

<h4 id="command-hook-fields">
  Bidang command hook
</h4>

Selain [bidang umum](#common-fields), command hooks menerima bidang-bidang ini:

| Bidang        | Diperlukan | Deskripsi                                                                                                                                                                                                                                                                                                                                                                          |
| :------------ | :--------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `command`     | ya         | Perintah shell untuk dijalankan. Dengan `args`, executable untuk spawn secara langsung. Lihat [Exec form dan shell form](#exec-form-and-shell-form)                                                                                                                                                                                                                                |
| `args`        | tidak      | Daftar argumen. Ketika ada, `command` diselesaikan sebagai executable dan di-spawn secara langsung dengan `args` sebagai vektor argumen, tanpa shell yang terlibat. Lihat [Exec form dan shell form](#exec-form-and-shell-form)                                                                                                                                                    |
| `async`       | tidak      | Jika `true`, dijalankan di latar belakang tanpa memblokir. Lihat [Run hooks in the background](#run-hooks-in-the-background)                                                                                                                                                                                                                                                       |
| `asyncRewake` | tidak      | Jika `true`, dijalankan di latar belakang dan membangunkan Claude pada kode keluar 2. Menyiratkan `async`. stderr hook, atau stdout jika stderr kosong, ditampilkan ke Claude sebagai pengingat sistem sehingga dapat bereaksi terhadap kegagalan latar belakang yang berjalan lama                                                                                                |
| `shell`       | tidak      | Shell untuk digunakan untuk hook ini. Menerima `"bash"` atau `"powershell"`. Default ke `"bash"`, atau ke `"powershell"` di Windows ketika Git Bash tidak diinstal. Menetapkan `"powershell"` menjalankan perintah melalui PowerShell di Windows. Tidak memerlukan `CLAUDE_CODE_USE_POWERSHELL_TOOL` karena hooks spawn PowerShell secara langsung. Diabaikan ketika `args` diatur |

<a id="exec-form-and-shell-form" />

<h5 id="exec-form-and-shell-form">
  Exec form dan shell form
</h5>

Hook command dijalankan sebagai exec form ketika `args` diatur, dan shell form ketika `args` dihilangkan. Atur `args` setiap kali hook mereferensikan [path placeholder](#reference-scripts-by-path), karena setiap elemen dilewatkan sebagai satu argumen tanpa quoting. Hilangkan `args` ketika Anda memerlukan fitur shell seperti pipes atau `&&`, atau ketika tidak ada kekhawatiran yang berlaku.

**Exec form** dijalankan ketika `args` ada. Claude Code menyelesaikan `command` sebagai executable di `PATH` dan spawn-nya secara langsung dengan `args` sebagai vektor argumen. Tidak ada shell, jadi setiap elemen `args` adalah satu argumen persis seperti yang ditulis, dan path placeholders seperti `${CLAUDE_PLUGIN_ROOT}` disubstitusi ke dalam `command` dan ke dalam setiap elemen `args` sebagai string biasa. Karakter khusus seperti apostrophe, `$`, dan backticks melewati verbatim karena tidak ada shell untuk menginterpretasinya. Tidak ada tokenisasi shell yang terjadi di platform apa pun.

**Shell form** dijalankan ketika `args` tidak ada. String `command` dilewatkan ke shell: `sh -c` di macOS dan Linux, Git Bash di Windows, atau PowerShell ketika Git Bash tidak diinstal. Atur bidang `shell` untuk memilih secara eksplisit. Shell melakukan tokenisasi string, memperluas variabel, dan menginterpretasi pipes, `&&`, redirects, dan globs.

<Note>
  Di Windows, exec form memerlukan `command` untuk diselesaikan ke executable nyata seperti `.exe`. Shim `.cmd` dan `.bat` yang npm, npx, eslint, dan tools lainnya instal di `node_modules/.bin` bukan executables dan tidak dapat di-spawn tanpa shell. Untuk menjalankannya dalam exec form, panggil skrip yang mendasar dengan `node` secara langsung, misalnya `"command": "node", "args": ["${CLAUDE_PLUGIN_ROOT}/node_modules/eslint/bin/eslint.js"]`. Pola `node` plus script-path bekerja di setiap platform karena `node.exe` adalah binary nyata. Untuk menjalankan shim `.cmd` atau `.bat` berdasarkan nama, gunakan shell form.
</Note>

Contoh ini menjalankan skrip Node yang dibundel dengan plugin. Exec form melewatkan path skrip yang diselesaikan sebagai satu argumen tanpa quoting:

```json theme={null}
{
  "type": "command",
  "command": "node",
  "args": ["${CLAUDE_PLUGIN_ROOT}/scripts/format.js", "--fix"]
}
```

Shell form yang setara memerlukan quoting untuk menangani paths dengan spasi atau karakter khusus:

```json theme={null}
{
  "type": "command",
  "command": "node \"${CLAUDE_PLUGIN_ROOT}\"/scripts/format.js --fix"
}
```

Kedua form mendukung [path placeholders](#reference-scripts-by-path) yang sama, dan keduanya mengekspornya sebagai variabel lingkungan `CLAUDE_PROJECT_DIR`, `CLAUDE_PLUGIN_ROOT`, dan `CLAUDE_PLUGIN_DATA` pada proses yang di-spawn, jadi skrip dapat membaca `process.env.CLAUDE_PLUGIN_ROOT` terlepas dari bagaimana itu diluncurkan.

Plugin hooks juga mensubstitusi nilai [`${user_config.*}`](/docs/id/plugins-reference#user-configuration), dalam exec form saja: nilai disubstitusi ke dalam `command` dan ke dalam setiap elemen `args` sebagai string biasa, jadi tidak ada shell yang mem-parse ulangnya.

Hook plugin bentuk shell yang `command`-nya mereferensikan `${user_config.*}` gagal dengan [error](/docs/id/errors#plugin-command-references-user-config) daripada menjalankan. Untuk menggunakan nilai opsi dari hook bentuk shell, baca variabel lingkungan `$CLAUDE_PLUGIN_OPTION_<KEY>`, seperti `$CLAUDE_PLUGIN_OPTION_WEBHOOK_URL` untuk opsi `webhook_url`, atau atur `args` untuk beralih hook ke exec form. Sebelum v2.1.207, hook plugin bentuk shell juga mensubstitusi `${user_config.*}`.

<Note>
  Dalam exec form, `command` adalah nama executable atau path saja. Jika `command` adalah nama bare tanpa path separator dan berisi whitespace bersama `args`, Claude Code mencatat warning karena spawn akan gagal: tidak ada executable bernama `node script.js`. Pindahkan token ekstra ke dalam `args`. Path absolut dengan spasi, seperti `C:\Program Files\nodejs\node.exe`, adalah executable tunggal yang valid dan tidak memicu warning.
</Note>

<h4 id="http-hook-fields">
  Bidang HTTP hook
</h4>

Selain [bidang umum](#common-fields), HTTP hooks menerima bidang-bidang ini:

| Bidang           | Diperlukan | Deskripsi                                                                                                                                                                                                                     |
| :--------------- | :--------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `url`            | ya         | URL untuk mengirimkan permintaan POST ke                                                                                                                                                                                      |
| `headers`        | tidak      | Header HTTP tambahan sebagai pasangan kunci-nilai. Nilai mendukung interpolasi variabel lingkungan menggunakan sintaks `$VAR_NAME` atau `${VAR_NAME}`. Hanya variabel yang tercantum dalam `allowedEnvVars` yang diselesaikan |
| `allowedEnvVars` | tidak      | Daftar nama variabel lingkungan yang dapat diinterpolasi ke nilai header. Referensi ke variabel yang tidak tercantum diganti dengan string kosong. Diperlukan untuk interpolasi variabel env apa pun untuk bekerja            |

Claude Code mengirimkan [JSON input](#hook-input-and-output) hook sebagai badan permintaan POST dengan `Content-Type: application/json`. Badan respons menggunakan [format JSON output](#json-output) yang sama seperti command hooks.

Penanganan kesalahan berbeda dari command hooks: respons non-2xx, kegagalan koneksi, dan timeout semuanya menghasilkan kesalahan non-blocking yang memungkinkan eksekusi berlanjut. Untuk memblokir pemanggilan tool atau menolak izin, kembalikan respons 2xx dengan badan JSON yang berisi `decision: "block"` atau `hookSpecificOutput` dengan `permissionDecision: "deny"`.

Contoh ini mengirimkan event `PreToolUse` ke layanan validasi lokal, mengautentikasi dengan token dari variabel lingkungan `MY_TOKEN`:

```json theme={null}
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "Bash",
        "hooks": [
          {
            "type": "http",
            "url": "http://localhost:8080/hooks/pre-tool-use",
            "timeout": 30,
            "headers": {
              "Authorization": "Bearer $MY_TOKEN"
            },
            "allowedEnvVars": ["MY_TOKEN"]
          }
        ]
      }
    ]
  }
}
```

<h4 id="mcp-tool-hook-fields">
  Bidang MCP tool hook
</h4>

Selain [bidang umum](#common-fields), MCP tool hooks menerima bidang-bidang ini:

| Bidang   | Diperlukan | Deskripsi                                                                                                                                                                                                                                                                                                          |
| :------- | :--------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `server` | ya         | Nama server MCP yang dikonfigurasi. Untuk [plugin-bundled server](/docs/id/mcp#plugin-provided-mcp-servers), ini adalah nama yang dibatasi `plugin:<plugin-name>:<server-name>`, seperti `plugin:my-plugin:db`, bukan kunci server bare. Server harus sudah terhubung; hook tidak pernah memicu alur OAuth atau koneksi |
| `tool`   | ya         | Nama tool untuk dipanggil di server itu                                                                                                                                                                                                                                                                            |
| `input`  | tidak      | Argumen yang dilewatkan ke tool. Nilai string mendukung substitusi `${path}` dari [JSON input](#hook-input-and-output) hook, seperti `"${tool_input.file_path}"`                                                                                                                                                   |

Output teks tool diperlakukan seperti command-hook stdout: jika itu diurai sebagai [JSON output](#json-output) yang valid, itu diproses sebagai keputusan, jika tidak, itu ditampilkan sebagai teks biasa. Jika server bernama tidak terhubung, atau tool mengembalikan `isError: true`, hook menghasilkan kesalahan non-blocking dan eksekusi berlanjut.

MCP tool hooks tersedia pada setiap hook event setelah Claude Code terhubung ke server MCP Anda. `SessionStart` dan `Setup` biasanya dijalankan sebelum server selesai terhubung, jadi hooks pada event tersebut harus mengharapkan kesalahan "not connected" pada run pertama.

Contoh ini memanggil tool `security_scan` pada server MCP `my_server` setelah setiap `Write` atau `Edit`, melewatkan path file yang diedit:

```json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Write|Edit",
        "hooks": [
          {
            "type": "mcp_tool",
            "server": "my_server",
            "tool": "security_scan",
            "input": { "file_path": "${tool_input.file_path}" }
          }
        ]
      }
    ]
  }
}
```

<h4 id="prompt-and-agent-hook-fields">
  Bidang prompt dan agent hook
</h4>

Selain [bidang umum](#common-fields), prompt dan agent hooks menerima bidang-bidang ini:

| Bidang   | Diperlukan | Deskripsi                                                                                                                                                                                     |
| :------- | :--------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `prompt` | ya         | Teks prompt untuk dikirim ke model. Gunakan `$ARGUMENTS` sebagai placeholder untuk JSON input hook. Escape dengan backslash untuk menyertakan teks literal: `\$1.00` dirender sebagai `$1.00` |
| `model`  | tidak      | Model untuk digunakan untuk evaluasi. Default ke model cepat                                                                                                                                  |

<h3 id="reference-scripts-by-path">
  Referensi skrip berdasarkan path
</h3>

Gunakan placeholders ini untuk mereferensikan skrip hook relatif terhadap akar proyek atau plugin, terlepas dari direktori kerja saat hook dijalankan:

* `${CLAUDE_PROJECT_DIR}`: akar proyek. Claude Code juga menetapkan variabel ini dalam lingkungan [stdio MCP servers](/docs/id/mcp#option-3-add-a-local-stdio-server) dan plugin LSP servers.
* `${CLAUDE_PLUGIN_ROOT}`: direktori instalasi plugin, untuk skrip yang dibundel dengan [plugin](/docs/id/plugins). Berubah pada setiap pembaruan plugin.
* `${CLAUDE_PLUGIN_DATA}`: [direktori data persisten](/docs/id/plugins-reference#persistent-data-directory) plugin, untuk dependensi dan status yang harus bertahan pembaruan plugin.

Lebih suka [exec form](#exec-form-and-shell-form) untuk hook apa pun yang mereferensikan path placeholder. Exec form melewatkan setiap elemen `args` sebagai satu argumen tanpa tokenisasi shell, jadi paths dengan spasi atau karakter khusus tidak memerlukan quoting. Dalam shell form, bungkus setiap placeholder dalam tanda kutip ganda.

<Tabs>
  <Tab title="Skrip proyek">
    Contoh ini menggunakan `${CLAUDE_PROJECT_DIR}` untuk menjalankan pemeriksa gaya dari direktori `.claude/hooks/` proyek setelah pemanggilan tool `Write` atau `Edit` apa pun:

    ```json theme={null}
    {
      "hooks": {
        "PostToolUse": [
          {
            "matcher": "Write|Edit",
            "hooks": [
              {
                "type": "command",
                "command": "${CLAUDE_PROJECT_DIR}/.claude/hooks/check-style.sh",
                "args": []
              }
            ]
          }
        ]
      }
    }
    ```
  </Tab>

  <Tab title="Skrip plugin">
    Tentukan plugin hooks dalam `hooks/hooks.json` dengan bidang `description` tingkat atas opsional. Ketika plugin diaktifkan, hooks-nya bergabung dengan hooks pengguna dan proyek Anda.

    Contoh ini menjalankan skrip pemformatan yang dibundel dengan plugin:

    ```json theme={null}
    {
      "description": "Automatic code formatting",
      "hooks": {
        "PostToolUse": [
          {
            "matcher": "Write|Edit",
            "hooks": [
              {
                "type": "command",
                "command": "${CLAUDE_PLUGIN_ROOT}/scripts/format.sh",
                "args": [],
                "timeout": 30
              }
            ]
          }
        ]
      }
    }
    ```

    Lihat [plugin components reference](/docs/id/plugins-reference#hooks) untuk detail tentang membuat plugin hooks.
  </Tab>
</Tabs>

<h3 id="hooks-in-skills-and-agents">
  Hooks dalam skills dan agents
</h3>

Selain file pengaturan dan plugin, hooks dapat didefinisikan langsung dalam [skills](/docs/id/skills) dan [subagents](/docs/id/sub-agents) menggunakan frontmatter. Hooks ini dibatasi pada siklus hidup komponen dan hanya dijalankan ketika komponen itu aktif.

Semua hook events didukung. Untuk subagents, `Stop` hooks secara otomatis dikonversi ke `SubagentStop` karena itu adalah event yang dijalankan ketika subagent selesai.

Hooks menggunakan format konfigurasi yang sama seperti hooks berbasis pengaturan tetapi dibatasi pada masa hidup komponen dan dibersihkan saat selesai.

Skill ini mendefinisikan hook `PreToolUse` yang menjalankan skrip validasi keamanan sebelum setiap perintah `Bash`:

```yaml theme={null}
---
name: secure-operations
description: Perform operations with security checks
hooks:
  PreToolUse:
    - matcher: "Bash"
      hooks:
        - type: command
          command: "./scripts/security-check.sh"
---
```

Agents menggunakan format yang sama dalam frontmatter YAML mereka.

<h3 id="the-/hooks-menu">
  Menu `/hooks`
</h3>

Ketik `/hooks` di Claude Code untuk membuka browser hooks read-only. Menu menampilkan setiap hook event dengan jumlah hooks yang dikonfigurasi, memungkinkan Anda menggali ke dalam matchers, dan menampilkan detail lengkap setiap hook handler. Gunakan untuk memverifikasi konfigurasi, memeriksa file pengaturan mana hook berasal, atau memeriksa perintah, prompt, atau URL hook.

Menu menampilkan semua lima tipe hook: `command`, `prompt`, `agent`, `http`, dan `mcp_tool`. Setiap hook diberi label dengan awalan `[type]` dan sumber menunjukkan di mana itu didefinisikan:

* `User`: dari `~/.claude/settings.json`
* `Project`: dari `.claude/settings.json`
* `Local`: dari `.claude/settings.local.json`
* `Plugin`: dari `hooks/hooks.json` plugin
* `Session`: terdaftar dalam memori untuk sesi saat ini
* `Built-in`: terdaftar secara internal oleh Claude Code

Memilih hook membuka tampilan detail menampilkan event, matcher, tipe, file sumber, dan perintah lengkap, prompt, atau URL. Menu adalah read-only: untuk menambah, memodifikasi, atau menghapus hooks, edit JSON pengaturan secara langsung atau minta Claude membuat perubahan.

<h3 id="disable-or-remove-hooks">
  Nonaktifkan atau hapus hooks
</h3>

Untuk menghapus hook, hapus entrinya dari file JSON pengaturan.

Untuk menonaktifkan semua hooks sementara tanpa menghapusnya, atur `"disableAllHooks": true` dalam file pengaturan Anda. Tidak ada cara untuk menonaktifkan hook individual sambil menyimpannya dalam konfigurasi.

Pengaturan `disableAllHooks` menghormati hierarki pengaturan terkelola. Jika administrator telah mengonfigurasi hooks melalui pengaturan kebijakan terkelola, `disableAllHooks` yang diatur dalam pengaturan pengguna, proyek, atau lokal tidak dapat menonaktifkan hooks terkelola tersebut. Hanya `disableAllHooks` yang diatur pada tingkat pengaturan terkelola yang dapat menonaktifkan hooks terkelola.

Pengeditan langsung ke hooks dalam file pengaturan biasanya diambil secara otomatis oleh file watcher.

<h2 id="hook-input-and-output">
  Input dan output hook
</h2>

Command hooks menerima data JSON melalui stdin dan mengkomunikasikan hasil melalui kode keluar, stdout, dan stderr. HTTP hooks menerima JSON yang sama sebagai badan permintaan POST dan mengkomunikasikan hasil melalui badan respons HTTP. Bagian ini mencakup bidang dan perilaku yang umum untuk semua events. Setiap bagian event di bawah [Hook events](#hook-events) mencakup skema input spesifiknya dan opsi kontrol keputusan.

Pada macOS dan Linux, command hooks berjalan dalam sesi mereka sendiri tanpa terminal pengontrol sejak v2.1.139. Proses hook dan proses anak apa pun tidak dapat membuka `/dev/tty` atau mengirim urutan escape langsung ke antarmuka Claude Code. Windows tidak memiliki `/dev/tty`. Untuk menampilkan pesan kepada pengguna di platform apa pun, kembalikan [`systemMessage`](#json-output) dalam output JSON. Untuk memicu notifikasi desktop, atur judul jendela, atau bunyikan bel, kembalikan [`terminalSequence`](#emit-terminal-notifications) sebagai gantinya.

<h3 id="common-input-fields">
  Bidang input umum
</h3>

Hook events menerima bidang-bidang ini sebagai JSON, selain bidang spesifik event yang didokumentasikan dalam setiap bagian [hook event](#hook-events). Untuk command hooks, JSON ini tiba melalui stdin. Untuk HTTP hooks, itu tiba sebagai badan permintaan POST.

| Bidang            | Deskripsi                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| :---------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `session_id`      | Pengenal sesi saat ini                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `prompt_id`       | UUID yang mengidentifikasi prompt pengguna yang sedang diproses. Cocok dengan atribut [`prompt.id` pada events OpenTelemetry](/docs/id/monitoring-usage#event-correlation-attributes), sehingga Anda dapat menghubungkan output hook dengan telemetri untuk satu prompt. Tidak ada sampai input pengguna pertama. Memerlukan Claude Code v2.1.196 atau lebih baru                                                                                                                                                                                                                                                                                                                                                                                                              |
| `transcript_path` | Path ke JSON percakapan. File transkrip ditulis secara asinkron dan mungkin tertinggal dari percakapan dalam memori, jadi mungkin belum menyertakan pesan terbaru giliran saat ini ketika hook dijalankan. Hooks yang memerlukan teks asisten akhir dari giliran saat ini harus menggunakan `last_assistant_message` pada [Stop](#stop) dan [SubagentStop](#subagentstop) alih-alih membaca transkrip                                                                                                                                                                                                                                                                                                                                                                     |
| `cwd`             | Direktori kerja saat hook dipanggil                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `permission_mode` | [Mode izin](/docs/id/permissions#permission-modes) saat ini: `"default"`, `"plan"`, `"acceptEdits"`, `"auto"`, `"dontAsk"`, atau `"bypassPermissions"`. Mode yang diberi label **Manual** tiba sebagai `"default"`, tidak pernah sebagai `"manual"`, jadi skrip yang cocok dengan `"default"` terus bekerja. Tidak semua events menerima bidang ini. Periksa contoh JSON di setiap bagian [hook event](#hook-events)                                                                                                                                                                                                                                                                                                                                                           |
| `effort`          | Objek dengan bidang `level` yang menyimpan [tingkat effort](/docs/id/model-config#adjust-effort-level) aktif untuk giliran: `"low"`, `"medium"`, `"high"`, `"xhigh"`, atau `"max"`. Jika effort yang diminta melebihi apa yang didukung model saat ini, ini adalah tingkat yang diturunkan yang sebenarnya digunakan model. Ultracode bukan tingkat yang berbeda dan dilaporkan sebagai `"xhigh"`. Objek cocok dengan bidang `effort` [status line](/docs/id/statusline#available-data). Hadir untuk events yang dijalankan dalam konteks penggunaan tool, seperti `PreToolUse`, `PostToolUse`, `Stop`, dan `SubagentStop`, ketika model saat ini mendukung parameter effort. Tingkat juga tersedia untuk perintah hook dan tool Bash sebagai variabel lingkungan `$CLAUDE_EFFORT`. |
| `hook_event_name` | Nama event yang dijalankan                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |

Saat berjalan dengan `--agent` atau di dalam subagent, dua bidang tambahan disertakan:

| Bidang       | Deskripsi                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| :----------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `agent_id`   | Pengenal unik untuk subagent. Hadir hanya ketika hook dijalankan di dalam pemanggilan subagent. Gunakan ini untuk membedakan pemanggilan hook subagent dari pemanggilan thread utama.                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `agent_type` | Nama agent (misalnya, `"Explore"` atau `"security-reviewer"`). Hadir ketika sesi menggunakan `--agent` atau hook dijalankan di dalam subagent. Untuk subagents, tipe subagent mengambil alih nilai `--agent` sesi. Untuk [custom subagents](/docs/id/sub-agents), ini adalah bidang `name` dari frontmatter agent, bukan nama file. Untuk subagents yang dikirim oleh [plugin](/docs/id/plugins), ini adalah pengenal yang dibatasi plugin seperti `my-plugin:reviewer`, bukan nama frontmatter telanjang. Lihat [SubagentStart](#subagentstart) untuk cara menulis matcher terhadap nama yang dibatasi plugin. |

Hanya hooks [`SessionStart`](#sessionstart) yang dapat menerima bidang `model`, dan tidak dijamin ada. Tidak ada variabel lingkungan `$CLAUDE_MODEL`. Proses hook mewarisi lingkungan induk, jadi dapat membaca `$ANTHROPIC_MODEL` jika Anda menetapkannya di shell Anda, tetapi nilai itu tidak berubah ketika Anda beralih model dengan `/model` selama sesi. Satu set variabel tidak diwariskan: Claude Code [menghapus variabel exporter `OTEL_*` dari setiap subprocess yang dijalankannya](/docs/id/monitoring-usage#administrator-configuration), termasuk hooks.

Misalnya, hook `PreToolUse` untuk perintah Bash menerima ini di stdin:

```json theme={null}
{
  "session_id": "abc123",
  "prompt_id": "550e8400-e29b-41d4-a716-446655440000",
  "transcript_path": "/home/user/.claude/projects/.../transcript.jsonl",
  "cwd": "/home/user/my-project",
  "permission_mode": "default",
  "hook_event_name": "PreToolUse",
  "tool_name": "Bash",
  "tool_input": {
    "command": "npm test"
  }
}
```

Bidang `tool_name` dan `tool_input` spesifik untuk event. Setiap bagian [hook event](#hook-events) mendokumentasikan bidang tambahan untuk event itu.

<h3 id="exit-code-output">
  Output kode keluar
</h3>

Kode keluar dari perintah hook Anda memberitahu Claude Code apakah tindakan harus dilanjutkan, diblokir, atau diabaikan.

**Exit 0** berarti sukses. Claude Code mengurai stdout untuk [bidang output JSON](#json-output). Output JSON hanya diproses pada exit 0. Untuk sebagian besar events, stdout ditulis ke debug log tetapi tidak ditampilkan dalam transkrip. Pengecualiannya adalah `UserPromptSubmit`, `UserPromptExpansion`, dan `SessionStart`, di mana stdout ditambahkan sebagai konteks yang dapat dilihat dan ditindaklanjuti Claude.

**Exit 2** berarti kesalahan blocking. Claude Code mengabaikan stdout dan JSON apa pun di dalamnya. Sebagai gantinya, teks stderr diumpankan kembali ke Claude sebagai pesan kesalahan. Efeknya tergantung pada event: `PreToolUse` memblokir pemanggilan tool, `UserPromptSubmit` menolak prompt, dan sebagainya. Lihat [perilaku kode keluar 2](#exit-code-2-behavior-per-event) untuk daftar lengkap.

**Kode keluar lainnya** adalah kesalahan non-blocking untuk sebagian besar hook events. Transkrip menampilkan pemberitahuan `<hook name> hook error` diikuti oleh baris pertama stderr, jadi Anda dapat mengidentifikasi penyebabnya tanpa `--debug`. Eksekusi berlanjut dan stderr lengkap ditulis ke debug log.

Misalnya, skrip perintah hook yang memblokir perintah Bash berbahaya:

```bash theme={null}
#!/bin/bash
# Membaca input JSON dari stdin, memeriksa perintah
command=$(jq -r '.tool_input.command' < /dev/stdin)

if [[ "$command" == rm* ]]; then
  echo "Blocked: rm commands are not allowed" >&2
  exit 2  # Blocking error: tool call is prevented
fi

exit 0  # No decision: the normal permission flow applies
```

<Warning>
  Untuk sebagian besar hook events, hanya kode keluar 2 yang memblokir tindakan. Claude Code memperlakukan kode keluar 1 sebagai kesalahan non-blocking dan melanjutkan dengan tindakan, meskipun 1 adalah kode kegagalan Unix konvensional. Jika hook Anda dimaksudkan untuk menegakkan kebijakan, gunakan `exit 2`. Pengecualiannya adalah `WorktreeCreate`, di mana kode keluar non-zero apa pun membatalkan pembuatan worktree.
</Warning>

<h4 id="exit-code-2-behavior-per-event">
  Perilaku kode keluar 2 per event
</h4>

Kode keluar 2 adalah cara hook menandakan "berhenti, jangan lakukan ini." Efeknya tergantung pada event, karena beberapa event mewakili tindakan yang dapat diblokir (seperti pemanggilan tool yang belum terjadi) dan yang lain mewakili hal-hal yang sudah terjadi atau tidak dapat dicegah.

| Hook event            | Dapat diblokir? | Apa yang terjadi pada exit 2                                                                                                                                  |
| :-------------------- | :-------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `PreToolUse`          | Ya              | Memblokir pemanggilan tool                                                                                                                                    |
| `PermissionRequest`   | Ya              | Menolak izin                                                                                                                                                  |
| `UserPromptSubmit`    | Ya              | Memblokir pemrosesan prompt dan menghapus prompt                                                                                                              |
| `UserPromptExpansion` | Ya              | Memblokir ekspansi                                                                                                                                            |
| `Stop`                | Ya              | Mencegah Claude berhenti, melanjutkan percakapan                                                                                                              |
| `SubagentStop`        | Ya              | Mencegah subagent berhenti                                                                                                                                    |
| `TeammateIdle`        | Ya              | Mencegah teammate menjadi idle, jadi terus bekerja                                                                                                            |
| `TaskCreated`         | Ya              | Membatalkan pembuatan tugas                                                                                                                                   |
| `TaskCompleted`       | Ya              | Mencegah tugas ditandai sebagai selesai                                                                                                                       |
| `ConfigChange`        | Ya              | Memblokir perubahan konfigurasi dari berlaku (kecuali `policy_settings`)                                                                                      |
| `StopFailure`         | Tidak           | Output dan kode keluar diabaikan                                                                                                                              |
| `PostToolUse`         | Tidak           | Menampilkan stderr ke Claude; tool sudah dijalankan                                                                                                           |
| `PostToolUseFailure`  | Tidak           | Menampilkan stderr ke Claude; tool sudah gagal                                                                                                                |
| `PostToolBatch`       | Ya              | Menghentikan loop agentic sebelum pemanggilan model berikutnya                                                                                                |
| `PermissionDenied`    | Tidak           | Kode keluar dan stderr diabaikan karena penolakan sudah terjadi. Gunakan JSON `hookSpecificOutput.retry: true` untuk memberitahu model itu dapat mencoba lagi |
| `Notification`        | Tidak           | Menampilkan stderr ke pengguna saja                                                                                                                           |
| `SubagentStart`       | Tidak           | Menampilkan stderr ke pengguna saja                                                                                                                           |
| `SessionStart`        | Tidak           | Menampilkan stderr ke pengguna saja                                                                                                                           |
| `Setup`               | Tidak           | Menampilkan stderr ke pengguna saja                                                                                                                           |
| `SessionEnd`          | Tidak           | Menampilkan stderr ke pengguna saja                                                                                                                           |
| `CwdChanged`          | Tidak           | Menampilkan stderr ke pengguna saja                                                                                                                           |
| `FileChanged`         | Tidak           | Menampilkan stderr ke pengguna saja                                                                                                                           |
| `PreCompact`          | Ya              | Memblokir compaction                                                                                                                                          |
| `PostCompact`         | Tidak           | Menampilkan stderr ke pengguna saja                                                                                                                           |
| `Elicitation`         | Ya              | Menolak elicitation                                                                                                                                           |
| `ElicitationResult`   | Ya              | Memblokir respons (tindakan menjadi decline)                                                                                                                  |
| `WorktreeCreate`      | Ya              | Kode keluar non-zero apa pun menyebabkan pembuatan worktree gagal                                                                                             |
| `WorktreeRemove`      | Tidak           | Kegagalan dicatat dalam mode debug saja                                                                                                                       |
| `InstructionsLoaded`  | Tidak           | Kode keluar diabaikan                                                                                                                                         |
| `MessageDisplay`      | Tidak           | Teks asli ditampilkan                                                                                                                                         |

Untuk `SessionStart`, `Setup`, dan `SubagentStart`, stderr kode keluar 2 dirender dalam transkrip sebagai pemberitahuan `<hook name> hook error`, dengan cara yang sama seperti [kesalahan non-blocking](#exit-code-output). Claude tidak melihatnya, dan sesi atau subagent berlanjut. Untuk `SubagentStart`, pemberitahuan muncul dalam transkrip subagent itu sendiri, bukan dalam percakapan induk.

Sejak Claude Code v2.1.199, `SessionStart`, `Setup`, dan `SubagentStart` menampilkan stderr kode keluar 2 dalam transkrip. Versi sebelumnya menulisnya ke debug log saja.

<h3 id="http-response-handling">
  Penanganan respons HTTP
</h3>

HTTP hooks menggunakan kode status HTTP dan badan respons sebagai pengganti kode keluar dan stdout:

* **2xx dengan badan kosong**: sukses, setara dengan kode keluar 0 tanpa output
* **2xx dengan badan teks biasa**: sukses, teks ditambahkan sebagai konteks
* **2xx dengan badan JSON**: sukses, diurai menggunakan skema [JSON output](#json-output) yang sama seperti command hooks
* **Status non-2xx**: kesalahan non-blocking, eksekusi berlanjut
* **Kegagalan koneksi atau timeout**: kesalahan non-blocking, eksekusi berlanjut

Tidak seperti command hooks, HTTP hooks tidak dapat menandakan kesalahan blocking hanya melalui kode status. Untuk memblokir pemanggilan tool atau menolak izin, kembalikan respons 2xx dengan badan JSON yang berisi bidang keputusan yang sesuai.

<h3 id="json-output">
  Output JSON
</h3>

Kode keluar memungkinkan Anda memblokir atau tetap diam, tetapi output JSON memberikan kontrol yang lebih halus. Alih-alih keluar dengan kode 2 untuk memblokir, keluar 0 dan cetak objek JSON ke stdout. Claude Code membaca bidang tertentu dari JSON itu untuk mengontrol perilaku, termasuk [decision control](#decision-control) untuk memblokir, mengizinkan, atau meningkatkan ke pengguna.

<Note>
  Anda harus memilih satu pendekatan per hook, bukan keduanya: gunakan kode keluar saja untuk signaling, atau keluar 0 dan cetak JSON untuk kontrol terstruktur. Claude Code hanya memproses JSON pada exit 0. Jika Anda keluar 2, JSON apa pun diabaikan.
</Note>

Stdout hook Anda harus berisi hanya objek JSON. Jika profil shell Anda mencetak teks saat startup, itu dapat mengganggu parsing JSON. Lihat [JSON validation failed](/docs/id/hooks-guide#json-validation-failed) dalam panduan troubleshooting.

String output hook, termasuk `additionalContext`, `systemMessage`, dan plain stdout, dibatasi pada 10.000 karakter. Output yang melebihi batas ini disimpan ke file dan diganti dengan pratinjau dan path file, dengan cara yang sama seperti hasil tool besar ditangani.

Objek JSON mendukung tiga jenis bidang:

* **Bidang universal** seperti `continue` bekerja di semua events. Ini tercantum dalam tabel di bawah.
* **Top-level `decision` dan `reason`** digunakan oleh beberapa events untuk memblokir atau memberikan umpan balik.
* **`hookSpecificOutput`** adalah objek bersarang untuk events yang memerlukan kontrol yang lebih kaya. Ini memerlukan bidang `hookEventName` yang diatur ke nama event.

| Bidang             | Default   | Deskripsi                                                                                                                                                                                                                                                                                                                                |
| :----------------- | :-------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `continue`         | `true`    | Jika `false`, Claude berhenti memproses sepenuhnya setelah hook dijalankan. Mengambil alih bidang keputusan spesifik event apa pun                                                                                                                                                                                                       |
| `stopReason`       | tidak ada | Pesan ditampilkan ke pengguna saat `continue` adalah `false`. Tidak ditampilkan ke Claude                                                                                                                                                                                                                                                |
| `suppressOutput`   | `false`   | Jika `true`, menyembunyikan stdout dari hook dari transkrip. Stdout masih muncul dalam debug log                                                                                                                                                                                                                                         |
| `systemMessage`    | tidak ada | Pesan peringatan ditampilkan ke pengguna                                                                                                                                                                                                                                                                                                 |
| `terminalSequence` | tidak ada | Urutan escape terminal untuk Claude Code yang akan dipancarkan atas nama Anda, seperti notifikasi desktop, judul jendela, atau bel. Dibatasi pada OSC `0`/`1`/`2`/`9`/`99`/`777` dan BEL. Jika nilai berisi apa pun di luar daftar putih, bidang diabaikan. Gunakan ini alih-alih menulis ke `/dev/tty`, yang tidak tersedia untuk hooks |

Untuk menghentikan Claude sepenuhnya terlepas dari tipe event:

```json theme={null}
{ "continue": false, "stopReason": "Build failed, fix errors before continuing" }
```

<h4 id="emit-terminal-notifications">
  Emit terminal notifications
</h4>

Bidang `terminalSequence` memerlukan Claude Code v2.1.141 atau lebih baru.

Hooks berjalan tanpa terminal pengontrol, jadi menulis urutan escape langsung ke `/dev/tty` gagal. Sebagai gantinya, kembalikan urutan escape dalam bidang `terminalSequence` dan Claude Code memancarkannya untuk Anda melalui jalur penulisan terminal miliknya sendiri. Ini bebas race, bekerja di dalam tmux dan GNU screen, dan bekerja di Windows di mana tidak ada `/dev/tty`.

Bidang menerima string dari satu atau lebih urutan escape yang diizinkan:

* OSC `0`, `1`, `2`: judul jendela dan ikon
* OSC `9`: notifikasi iTerm2, ConEmu, Windows Terminal, dan WezTerm, termasuk `9;4` kemajuan taskbar
* OSC `99`: notifikasi Kitty
* OSC `777`: notifikasi urxvt, Ghostty, dan Warp
* BEL telanjang

Urutan dapat diakhiri dengan BEL atau dengan ST. Apa pun di luar daftar putih, termasuk urutan kursor dan warna CSI, urutan palet OSC, hyperlink OSC 8, penulisan clipboard OSC 52, dan OSC 1337, ditolak dan bidang diabaikan.

Contoh di bawah menjalankan notifikasi desktop dari hook `Notification`. Urutan escape dibangun dengan `printf` octal escapes sehingga byte kontrol tidak pernah muncul di baris perintah shell, dan `jq -n --arg` membangun output JSON sehingga tanda kutip, backslash, dan newline dalam pesan notifikasi diloloskan dengan benar:

```bash theme={null}
#!/bin/bash
# Notification hook: ping desktop ketika Claude Code membutuhkan perhatian.
input=$(cat)
title="Claude Code'
body=$(jq -r '.message // 'Needs your attention"' <<<"$input")
seq=$(printf '\033]777;notify;%s;%s\007' "$title" "$body")
jq -nc --arg seq "$seq" '{terminalSequence: $seq}'
```

Bentuk `{ "terminalSequence": "..." }` sama dari shell atau bahasa apa pun. Di Windows, bangun string escape di PowerShell atau skrip dan pancarkan objek JSON yang sama.

<Note>
  `terminalSequence` adalah pengganti yang didukung untuk hooks yang sebelumnya menulis urutan escape langsung ke `/dev/tty`. Daftar putih dibatasi pada urutan yang tidak dapat memindahkan kursor atau mengubah warna, jadi hook tidak pernah dapat merusak prompt di layar.
</Note>

<h4 id="add-context-for-claude">
  Tambahkan konteks untuk Claude
</h4>

Bidang `additionalContext` meneruskan string dari hook Anda ke jendela konteks Claude. Claude Code membungkus string dalam pengingat sistem dan menyisipkannya ke dalam percakapan pada titik di mana hook dijalankan. Claude membaca pengingat pada permintaan model berikutnya, tetapi itu tidak muncul sebagai pesan chat dalam antarmuka.

Kembalikan `additionalContext` di dalam `hookSpecificOutput` bersama nama event:

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PostToolUse",
    "additionalContext": "This file is generated. Edit src/schema.ts and run `bun generate` instead."
  }
}
```

Di mana pengingat muncul tergantung pada event:

* [SessionStart](#sessionstart), [Setup](#setup), dan [SubagentStart](#subagentstart): di awal percakapan, sebelum prompt pertama
* [UserPromptSubmit](#userpromptsubmit) dan [UserPromptExpansion](#userpromptexpansion): bersama prompt yang dikirimkan
* [PreToolUse](#pretooluse), [PostToolUse](#posttooluse), [PostToolUseFailure](#posttoolusefailure), dan [PostToolBatch](#posttoolbatch): di sebelah hasil tool
* [Stop](#stop) dan [SubagentStop](#subagentstop): di akhir giliran. Percakapan berlanjut sehingga Claude dapat bertindak atas umpan balik. Lihat [Stop decision control](#stop-decision-control)

Ketika beberapa hooks mengembalikan `additionalContext` untuk event yang sama, Claude menerima semua nilai. Jika nilai melebihi 10.000 karakter, Claude Code menulis teks lengkap ke file di direktori sesi dan meneruskan Claude path file dengan pratinjau singkat sebagai gantinya.

Gunakan `additionalContext` untuk informasi yang harus diketahui Claude tentang keadaan saat ini lingkungan Anda atau operasi yang baru saja dijalankan:

* **Keadaan lingkungan**: branch saat ini, target deployment, atau flag fitur aktif
* **Aturan proyek bersyarat**: perintah test mana yang berlaku untuk file yang baru diedit, direktori mana yang read-only di worktree ini
* **Data eksternal**: masalah terbuka yang ditugaskan kepada Anda, hasil CI terbaru, konten yang diambil dari layanan internal

Untuk instruksi yang tidak pernah berubah, lebih suka [CLAUDE.md](/docs/id/memory). Itu dimuat tanpa menjalankan skrip dan merupakan tempat standar untuk konvensi proyek statis.

Tulis teks sebagai pernyataan faktual daripada instruksi sistem imperatif. Frasa seperti "Target deployment adalah production" atau "Repo ini menggunakan `bun test`" dibaca sebagai informasi proyek. Teks yang dibingkai sebagai perintah sistem out-of-band dapat memicu pertahanan injeksi prompt Claude, yang menyebabkan Claude menampilkan teks kepada Anda alih-alih memperlakukannya sebagai konteks.

Setelah disuntikkan, teks disimpan dalam transkrip sesi. Untuk events mid-session seperti `PostToolUse` atau `UserPromptSubmit`, melanjutkan dengan `--continue` atau `--resume` memutar ulang teks yang disimpan daripada menjalankan kembali hook untuk giliran masa lalu, jadi nilai seperti timestamp atau commit SHA menjadi usang pada resume. Hook `SessionStart` dijalankan lagi pada resume dengan `source` diatur ke `"resume"`, jadi mereka dapat menyegarkan konteks mereka.

<h4 id="decision-control">
  Kontrol keputusan
</h4>

Tidak setiap event mendukung pemblokiran atau kontrol perilaku melalui JSON. Events yang melakukannya masing-masing menggunakan set bidang yang berbeda untuk mengekspresikan keputusan itu. Gunakan tabel ini sebagai referensi cepat sebelum menulis hook:

| Events                                                                                                                              | Pola keputusan                     | Bidang kunci                                                                                                                                                                                                                                           |
| :---------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| UserPromptSubmit, UserPromptExpansion, PostToolUse, PostToolUseFailure, PostToolBatch, Stop, SubagentStop, ConfigChange, PreCompact | Top-level `decision`               | `decision: "block"`, `reason`. Stop dan SubagentStop juga menerima `hookSpecificOutput.additionalContext` untuk [umpan balik non-error yang melanjutkan percakapan](#stop-decision-control)                                                            |
| TeammateIdle, TaskCreated, TaskCompleted                                                                                            | Kode keluar atau `continue: false` | Kode keluar 2 memblokir tindakan dengan umpan balik stderr. JSON `{"continue": false, "stopReason": "..."}` juga menghentikan teammate sepenuhnya, mencocokkan perilaku hook `Stop`                                                                    |
| PreToolUse                                                                                                                          | `hookSpecificOutput`               | `permissionDecision` (allow/deny/ask/defer), `permissionDecisionReason`                                                                                                                                                                                |
| PermissionRequest                                                                                                                   | `hookSpecificOutput`               | `decision.behavior` (allow/deny)                                                                                                                                                                                                                       |
| PermissionDenied                                                                                                                    | `hookSpecificOutput`               | `retry: true` memberitahu model itu dapat mencoba lagi pemanggilan tool yang ditolak                                                                                                                                                                   |
| WorktreeCreate                                                                                                                      | path return                        | Command hook mencetak path di stdout; HTTP hook mengembalikan `hookSpecificOutput.worktreePath`. Kegagalan hook atau path yang hilang gagal membuat                                                                                                    |
| Elicitation                                                                                                                         | `hookSpecificOutput`               | `action` (accept/decline/cancel), `content` (nilai field form untuk accept)                                                                                                                                                                            |
| ElicitationResult                                                                                                                   | `hookSpecificOutput`               | `action` (accept/decline/cancel), `content` (nilai field form override)                                                                                                                                                                                |
| MessageDisplay                                                                                                                      | `hookSpecificOutput`               | `displayContent` menggantikan teks yang ditampilkan di layar. Display-only: transkrip dan apa yang Claude lihat tetap mempertahankan asli                                                                                                              |
| SessionStart, Setup, SubagentStart                                                                                                  | Context only                       | `hookSpecificOutput.additionalContext` menambahkan konteks untuk Claude. SessionStart juga menerima [`initialUserMessage`, `watchPaths`, `sessionTitle`, dan `reloadSkills`](#sessionstart-decision-control). Tidak ada kontrol blocking atau decision |
| WorktreeRemove, Notification, SessionEnd, PostCompact, InstructionsLoaded, StopFailure, CwdChanged, FileChanged                     | Tidak ada                          | Tidak ada kontrol keputusan. Digunakan untuk efek samping seperti logging atau cleanup                                                                                                                                                                 |

Beberapa events juga dapat menulis ulang konten daripada hanya mengizinkan atau memblokir:

* `PreToolUse`: `updatedInput` langsung di bawah `hookSpecificOutput` menggantikan argumen tool sebelum dijalankan. Lihat [PreToolUse decision control](#pretooluse-decision-control) untuk set lengkap opsi.
* `PermissionRequest`: `updatedInput` di dalam objek `decision`. Lihat [PermissionRequest decision control](#permissionrequest-decision-control) untuk set lengkap opsi.
* `PostToolUse`: `updatedToolOutput` menggantikan hasil tool. Lihat [PostToolUse decision control](#posttooluse-decision-control) untuk set lengkap opsi.
* `UserPromptSubmit`: tidak dapat mengganti prompt; hanya menyuntikkan `additionalContext` di sampingnya

Untuk kasus penggunaan redaksi atau transformasi, intersep di `PreToolUse` untuk input tool keluar dan `PostToolUse` untuk hasil tool masuk.

Berikut adalah contoh setiap pola dalam aksi:

<Tabs>
  <Tab title="Top-level decision">
    Digunakan oleh `UserPromptSubmit`, `UserPromptExpansion`, `PostToolUse`, `PostToolUseFailure`, `PostToolBatch`, `Stop`, `SubagentStop`, `ConfigChange`, dan `PreCompact`. Satu-satunya nilai adalah `"block"`. Untuk mengizinkan tindakan dilanjutkan, hilangkan `decision` dari JSON Anda, atau keluar 0 tanpa JSON apa pun:

    ```json theme={null}
    {
      "decision": "block",
      "reason": "Test suite must pass before proceeding"
    }
    ```
  </Tab>

  <Tab title="PreToolUse">
    Menggunakan `hookSpecificOutput` untuk kontrol yang lebih kaya: izinkan, tolak, atau eskalasi ke pengguna. Anda juga dapat memodifikasi input tool sebelum dijalankan atau menyuntikkan konteks tambahan untuk Claude. Lihat [PreToolUse decision control](#pretooluse-decision-control) untuk set lengkap opsi.

    ```json theme={null}
    {
      "hookSpecificOutput": {
        "hookEventName": "PreToolUse",
        "permissionDecision": "deny",
        "permissionDecisionReason": "Database writes are not allowed"
      }
    }
    ```
  </Tab>

  <Tab title="PermissionRequest">
    Menggunakan `hookSpecificOutput` untuk mengizinkan atau menolak permintaan izin atas nama pengguna. Saat mengizinkan, Anda juga dapat memodifikasi input tool atau menerapkan aturan izin sehingga pengguna tidak diminta lagi. Lihat [PermissionRequest decision control](#permissionrequest-decision-control) untuk set lengkap opsi.

    ```json theme={null}
    {
      "hookSpecificOutput": {
        "hookEventName": "PermissionRequest",
        "decision": {
          "behavior": "allow",
          "updatedInput": {
            "command": "npm run lint"
          }
        }
      }
    }
    ```
  </Tab>
</Tabs>

Untuk contoh yang diperluas termasuk validasi perintah Bash, pemfilteran prompt, dan skrip persetujuan otomatis, lihat [What you can automate](/docs/id/hooks-guide#what-you-can-automate) dalam panduan dan [Bash command validator reference implementation](https://github.com/anthropics/claude-code/blob/main/examples/hooks/bash_command_validator_example.py).

<h2 id="hook-events">
  Hook events
</h2>

Setiap event sesuai dengan titik dalam siklus hidup Claude Code di mana hooks dapat dijalankan. Bagian-bagian di bawah diurutkan untuk mencocokkan siklus hidup: dari pengaturan sesi melalui loop agentic ke akhir sesi. Setiap bagian menjelaskan kapan event dijalankan, matcher apa yang didukungnya, JSON input yang diterima, dan cara mengontrol perilaku melalui output.

<h3 id="sessionstart">
  SessionStart
</h3>

Dijalankan ketika Claude Code memulai sesi baru atau melanjutkan sesi yang ada. Berguna untuk memuat konteks pengembangan seperti masalah yang ada atau perubahan terbaru pada codebase Anda, atau menyiapkan variabel lingkungan. Untuk konteks statis yang tidak memerlukan skrip, gunakan [CLAUDE.md](/docs/id/memory) sebagai gantinya.

SessionStart dijalankan pada setiap sesi, jadi jaga hooks ini tetap cepat. Hanya hooks `type: "command"` dan `type: "mcp_tool"` yang didukung.

Nilai matcher sesuai dengan cara sesi dimulai:

| Matcher   | Kapan dijalankan                         |
| :-------- | :--------------------------------------- |
| `startup` | Sesi baru                                |
| `resume`  | `--resume`, `--continue`, atau `/resume` |
| `clear`   | `/clear`                                 |
| `compact` | Compaction otomatis atau manual          |

<h4 id="sessionstart-input">
  SessionStart input
</h4>

Selain [bidang input umum](#common-input-fields), SessionStart hooks menerima `source` dan secara opsional `model`, `agent_type`, dan `session_title`:

| Bidang          | Deskripsi                                                                                                                                                                                                                                           |
| :-------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `source`        | Bagaimana sesi dimulai: `"startup"` untuk sesi baru, `"resume"` untuk sesi yang dilanjutkan, `"clear"` setelah `/clear`, atau `"compact"` setelah compaction                                                                                        |
| `model`         | Pengenal model aktif. Itu dapat dihilangkan, misalnya setelah `/clear` atau ketika sesi dipulihkan melalui conversation recovery, jadi periksa bidang sebelum membacanya                                                                            |
| `agent_type`    | Nama agent, hadir ketika Anda memulai Claude Code dengan `claude --agent <name>`                                                                                                                                                                    |
| `session_title` | Judul sesi saat ini jika sudah ditetapkan, misalnya melalui `--name` atau `/rename`. Hook yang memancarkan `sessionTitle` dapat memeriksa `session_title` terlebih dahulu untuk menghindari menimpa judul yang ditetapkan pengguna secara eksplisit |

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "SessionStart",
  "source": "startup",
  "model": "claude-sonnet-5"
}
```

<h4 id="sessionstart-decision-control">
  SessionStart decision control
</h4>

Teks apa pun yang dicetak skrip hook ke stdout ditambahkan sebagai konteks untuk Claude. Selain [bidang output JSON](#json-output) yang tersedia untuk semua hooks, Anda dapat mengembalikan bidang spesifik event ini:

| Bidang               | Deskripsi                                                                                                                                                                                                                                                                                                                                                                 |
| :------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `additionalContext`  | String ditambahkan ke konteks Claude pada awal percakapan, sebelum prompt pertama. Lihat [Tambahkan konteks untuk Claude](#add-context-for-claude) untuk cara teks disampaikan dan apa yang harus dimasukkan                                                                                                                                                              |
| `initialUserMessage` | String digunakan sebagai pesan pengguna pertama sesi. Berlaku dalam [mode non-interaktif](/docs/id/headless) dengan flag `-p`, di mana itu menjadi giliran pertama bahkan jika tidak ada prompt yang disediakan. Jika prompt disediakan, itu mengikuti sebagai giliran berikutnya. Tidak seperti `additionalContext`, yang menempel pada giliran yang ada, ini membuat giliran |
| `sessionTitle`       | Menetapkan judul sesi, dengan efek yang sama seperti `/rename`. Gunakan untuk memberi nama sesi secara otomatis dari folder peluncuran, cabang git, atau nama worktree. Berlaku hanya ketika `source` adalah `"startup"` atau `"resume"`; diabaikan pada `"clear"` dan `"compact"`                                                                                        |
| `watchPaths`         | Array path absolut untuk menonton untuk event [FileChanged](#filechanged) selama sesi ini                                                                                                                                                                                                                                                                                 |
| `reloadSkills`       | Boolean. Ketika `true`, Claude Code memindai ulang direktori [skill](/docs/id/skills) dan command setelah SessionStart hooks selesai, jadi skills yang diinstal hook tersedia dalam sesi yang sama, dimulai dengan prompt pertama                                                                                                                                              |

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "SessionStart",
    "additionalContext": "Current branch: feat/auth-refactor\nUncommitted changes: src/auth.ts, src/login.tsx\nActive issue: #4211 Migrate to OAuth2",
    "sessionTitle": "auth-refactor"
  }
}
```

Karena plain stdout sudah mencapai Claude untuk event ini, hook yang hanya memuat konteks dapat mencetak ke stdout secara langsung tanpa membangun JSON. Gunakan bentuk JSON ketika Anda perlu menggabungkan konteks dengan bidang lain seperti `suppressOutput` atau `sessionTitle`.

Gunakan `reloadSkills` ketika SessionStart hook menginstal atau memperbarui skills. Penemuan skill biasanya berjalan sebelum SessionStart hooks selesai, jadi file yang ditulis hook ke `~/.claude/skills/` atau `.claude/skills/` tidak akan muncul sampai sesi berikutnya. Contoh ini menyinkronkan repositori skills bersama dan meminta pemindaian ulang:

```bash theme={null}
#!/bin/bash

git -C ~/.claude/skills/team-skills pull --quiet 2>/dev/null || \
  git clone --quiet https://git.example.com/your-org/team-skills.git ~/.claude/skills/team-skills

echo '{"hookSpecificOutput": {"hookEventName": "SessionStart", "reloadSkills": true}}'
```

<h4 id="persist-environment-variables">
  Persist environment variables
</h4>

SessionStart hooks memiliki akses ke variabel lingkungan `CLAUDE_ENV_FILE`, yang menyediakan path file di mana Anda dapat mempertahankan variabel lingkungan untuk perintah Bash berikutnya.

Untuk menetapkan variabel lingkungan individual, tulis pernyataan `export` ke `CLAUDE_ENV_FILE`. Gunakan append (`>>`) untuk mempertahankan variabel yang ditetapkan oleh hooks lain:

```bash theme={null}
#!/bin/bash

if [ -n "$CLAUDE_ENV_FILE" ]; then
  echo 'export NODE_ENV=production' >> "$CLAUDE_ENV_FILE"
  echo 'export DEBUG_LOG=true' >> "$CLAUDE_ENV_FILE"
  echo 'export PATH="$PATH:./node_modules/.bin"' >> "$CLAUDE_ENV_FILE"
fi

exit 0
```

Untuk menangkap semua perubahan lingkungan dari perintah setup, bandingkan variabel yang diekspor sebelum dan sesudah:

```bash theme={null}
#!/bin/bash

ENV_BEFORE=$(export -p | sort)

# Jalankan perintah setup Anda yang memodifikasi lingkungan
source ~/.nvm/nvm.sh
nvm use 20

if [ -n "$CLAUDE_ENV_FILE" ]; then
  ENV_AFTER=$(export -p | sort)
  comm -13 <(echo "$ENV_BEFORE") <(echo "$ENV_AFTER") >> "$CLAUDE_ENV_FILE"
fi

exit 0
```

Variabel apa pun yang ditulis ke file ini akan tersedia dalam semua perintah Bash berikutnya yang dijalankan Claude Code selama sesi.

<Note>
  `CLAUDE_ENV_FILE` tersedia untuk SessionStart, [Setup](#setup), [CwdChanged](#cwdchanged), dan [FileChanged](#filechanged) hooks. Tipe hook lainnya tidak memiliki akses ke variabel ini.
</Note>

<h3 id="setup">
  Setup
</h3>

Dijalankan hanya ketika Anda meluncurkan Claude Code dengan `--init-only`, atau dengan `--init` atau `--maintenance` dalam [mode non-interaktif](/docs/id/headless) dengan flag `-p`. Itu tidak dijalankan pada startup normal. Gunakan untuk instalasi dependensi satu kali atau pembersihan terjadwal yang Anda picu secara eksplisit dari CI atau skrip, terpisah dari startup sesi normal. Untuk inisialisasi per-sesi, gunakan [SessionStart](#sessionstart) sebagai gantinya.

Nilai matcher sesuai dengan flag CLI yang memicu hook:

| Matcher       | Kapan dijalankan                             |
| :------------ | :------------------------------------------- |
| `init`        | `claude --init-only` atau `claude -p --init` |
| `maintenance` | `claude -p --maintenance`                    |

`--init-only` menjalankan Setup hooks dan SessionStart hooks dengan matcher `startup`, kemudian keluar tanpa memulai percakapan. `--init` dan `--maintenance` menjalankan Setup hooks hanya ketika digabungkan dengan `-p`; dalam sesi interaktif dua flag itu saat ini tidak menjalankan Setup hooks.

Karena Setup tidak dijalankan pada setiap peluncuran, plugin yang memerlukan dependensi yang diinstal tidak dapat mengandalkan Setup saja. Pola praktis adalah memeriksa dependensi pada penggunaan pertama dan menginstal jika tidak ada, misalnya hook atau skill yang menguji `${CLAUDE_PLUGIN_DATA}/node_modules` dan menjalankan `npm install` jika tidak ada. Lihat [direktori data persisten](/docs/id/plugins-reference#persistent-data-directory) untuk tempat menyimpan dependensi yang diinstal.

<h4 id="setup-input">
  Setup input
</h4>

Selain [bidang input umum](#common-input-fields), Setup hooks menerima bidang `trigger` yang diatur ke `"init"` atau `"maintenance"`:

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "Setup",
  "trigger": "init"
}
```

<h4 id="setup-decision-control">
  Setup decision control
</h4>

Setup hooks tidak dapat memblokir. Kode keluar non-nol apa pun, termasuk 2, menampilkan stderr ke pengguna sebagai pemberitahuan `<hook name> hook error`, dan eksekusi berlanjut. Dalam [mode non-interaktif](/docs/id/headless), output hook muncul hanya ketika Anda meluncurkan dengan `--verbose`.

Untuk meneruskan informasi ke konteks Claude, kembalikan `additionalContext` dalam output JSON; plain stdout ditulis ke debug log saja. Selain [bidang output JSON](#json-output) yang tersedia untuk semua hooks, Anda dapat mengembalikan bidang spesifik event ini:

| Bidang              | Deskripsi                                                                   |
| :------------------ | :-------------------------------------------------------------------------- |
| `additionalContext` | String ditambahkan ke konteks Claude. Nilai dari beberapa hooks digabungkan |

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "Setup",
    "additionalContext": "Dependencies installed: node_modules, .venv"
  }
}
```

Setup hooks memiliki akses ke `CLAUDE_ENV_FILE`. Variabel yang ditulis ke file itu bertahan ke perintah Bash berikutnya untuk sesi, sama seperti dalam [SessionStart hooks](#persist-environment-variables). Hanya hooks `type: "command"` dan `type: "mcp_tool"` yang didukung.

<h3 id="instructionsloaded">
  InstructionsLoaded
</h3>

Dijalankan ketika file `CLAUDE.md` atau `.claude/rules/*.md` dimuat ke dalam konteks. Event ini dijalankan saat startup sesi untuk file yang dimuat dengan eager dan lagi nanti ketika file dimuat dengan lazy, misalnya ketika Claude mengakses subdirektori yang berisi `CLAUDE.md` bersarang atau ketika aturan bersyarat dengan frontmatter `paths:` cocok. Hook tidak mendukung pemblokiran atau kontrol keputusan. Itu dijalankan secara asinkron untuk tujuan observabilitas.

Matcher dijalankan terhadap `load_reason`. Misalnya, gunakan `"matcher": "session_start"` untuk dijalankan hanya untuk file yang dimuat saat startup sesi, atau `"matcher": "path_glob_match|nested_traversal"` untuk dijalankan hanya untuk lazy loads.

<h4 id="instructionsloaded-input">
  InstructionsLoaded input
</h4>

Selain [bidang input umum](#common-input-fields), InstructionsLoaded hooks menerima bidang-bidang ini:

| Bidang              | Deskripsi                                                                                                                                                                                                  |
| :------------------ | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `file_path`         | Path absolut ke file instruksi yang dimuat                                                                                                                                                                 |
| `memory_type`       | Cakupan file: `"User"`, `"Project"`, `"Local"`, atau `"Managed"`                                                                                                                                           |
| `load_reason`       | Mengapa file dimuat: `"session_start"`, `"nested_traversal"`, `"path_glob_match"`, `"include"`, atau `"compact"`. Nilai `"compact"` dijalankan ketika file instruksi dimuat ulang setelah event compaction |
| `globs`             | Pola glob path dari frontmatter `paths:` file, jika ada. Hadir hanya untuk load `path_glob_match`                                                                                                          |
| `trigger_file_path` | Path ke file yang akses memicu load ini, untuk lazy loads                                                                                                                                                  |
| `parent_file_path`  | Path ke file instruksi induk yang menyertakan ini, untuk load `include`                                                                                                                                    |

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../transcript.jsonl",
  "cwd": "/Users/my-project",
  "hook_event_name": "InstructionsLoaded",
  "file_path": "/Users/my-project/CLAUDE.md",
  "memory_type": "Project",
  "load_reason": "session_start"
}
```

<h4 id="instructionsloaded-decision-control">
  InstructionsLoaded decision control
</h4>

InstructionsLoaded hooks tidak memiliki kontrol keputusan. Mereka tidak dapat memblokir atau memodifikasi pemuatan instruksi. Gunakan event ini untuk audit logging, compliance tracking, atau observabilitas.

<h3 id="userpromptsubmit">
  UserPromptSubmit
</h3>

Dijalankan ketika pengguna mengirimkan prompt, sebelum Claude memproses. Ini memungkinkan Anda menambahkan konteks tambahan berdasarkan prompt/percakapan, memvalidasi prompts, atau memblokir jenis prompts tertentu.

Hooks `UserPromptSubmit` memiliki timeout default 30 detik untuk tipe `command`, `http`, dan `mcp_tool`, lebih pendek dari default 600 detik untuk tipe tersebut pada event lain. Karena hook ini dijalankan sebelum setiap prompt dan memblokir pemrosesan model sampai selesai, hook yang macet menghentikan sesi. Jika hook Anda memerlukan lebih banyak waktu, atur bidang `timeout` dalam entri hook.

Hook `UserPromptSubmit` command, HTTP, atau MCP tool yang mencapai timeout-nya dibatalkan dan output-nya, termasuk `additionalContext` apa pun, dibuang. Prompt masih mencapai Claude tanpa konteks itu. Mulai dari v2.1.196, transkrip menampilkan pemberitahuan yang menamai hook, timeout yang dijalankan, dan bahwa output dibuang. Versi sebelumnya membatalkan hook tanpa pemberitahuan.

Hook callback [Agent SDK](/docs/id/agent-sdk/hooks) pada `UserPromptSubmit` yang mencapai timeout-nya memblokir prompt dengan pesan yang menamai hook dan timeout, karena callback di sana dapat bertindak sebagai policy gate yang tidak boleh gagal terbuka. Sesi berlanjut. Sebelum v2.1.208, timeout callback pada event itu mengakhiri giliran dengan kesalahan eksekusi.

<h4 id="userpromptsubmit-input">
  UserPromptSubmit input
</h4>

Selain [bidang input umum](#common-input-fields), UserPromptSubmit hooks menerima bidang `prompt` yang berisi teks yang dikirimkan pengguna.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "UserPromptSubmit",
  "prompt": "Write a function to calculate the factorial of a number"
}
```

<h4 id="userpromptsubmit-decision-control">
  UserPromptSubmit decision control
</h4>

Hooks `UserPromptSubmit` dapat mengontrol apakah prompt pengguna diproses dan menambahkan konteks. Semua [bidang output JSON](#json-output) tersedia.

Ada dua cara untuk menambahkan konteks ke percakapan pada kode keluar 0:

* **Plain text stdout**: teks non-JSON apa pun yang ditulis ke stdout ditambahkan sebagai konteks
* **JSON dengan `additionalContext`**: gunakan format JSON di bawah untuk kontrol lebih. Bidang `additionalContext` ditambahkan sebagai konteks

Plain stdout ditampilkan sebagai output hook dalam transkrip. Nilai `additionalContext` disuntikkan sebagai pengingat sistem yang dibaca Claude tanpa entri transkrip yang terlihat.

Untuk memblokir prompt, kembalikan objek JSON dengan `decision` diatur ke `"block"`:

| Bidang                   | Deskripsi                                                                                                                            |
| :----------------------- | :----------------------------------------------------------------------------------------------------------------------------------- |
| `decision`               | `"block"` mencegah prompt diproses dan menghapusnya dari konteks. Hilangkan untuk mengizinkan prompt dilanjutkan                     |
| `reason`                 | Ditampilkan ke pengguna saat `decision` adalah `"block"`. Tidak ditambahkan ke konteks                                               |
| `additionalContext`      | String ditambahkan ke konteks Claude bersama prompt yang dikirimkan. Lihat [Tambahkan konteks untuk Claude](#add-context-for-claude) |
| `sessionTitle`           | Menetapkan judul sesi. Gunakan untuk memberi nama sesi secara otomatis berdasarkan konten prompt                                     |
| `suppressOriginalPrompt` | Jika `true` saat `decision` adalah `"block"`, menghilangkan teks prompt asli dari pesan blok yang ditampilkan ke pengguna            |

```json theme={null}
{
  "decision": "block",
  "reason": "Explanation for decision",
  "hookSpecificOutput": {
    "hookEventName": "UserPromptSubmit",
    "additionalContext": "My additional context here",
    "sessionTitle": "My session title"
  }
}
```

<h3 id="userpromptexpansion">
  UserPromptExpansion
</h3>

Dijalankan ketika perintah slash yang diketik pengguna berkembang menjadi prompt sebelum mencapai Claude. Gunakan ini untuk memblokir perintah tertentu dari invokasi langsung, menyuntikkan konteks untuk skill tertentu, atau mencatat perintah mana yang diinvokasi pengguna. Misalnya, hook yang cocok dengan `deploy` dapat memblokir `/deploy` kecuali file persetujuan ada, atau hook yang cocok dengan skill review dapat menambahkan checklist review tim sebagai `additionalContext`.

Event ini mencakup path yang `PreToolUse` tidak: hook `PreToolUse` yang cocok dengan tool `Skill` hanya dijalankan ketika Claude memanggil tool, tetapi mengetik `/skillname` secara langsung melewati `PreToolUse`. `UserPromptExpansion` dijalankan pada path langsung itu.

Cocok pada `command_name`. Biarkan matcher kosong untuk dijalankan pada setiap slash command prompt-type.

<h4 id="userpromptexpansion-input">
  UserPromptExpansion input
</h4>

Selain [bidang input umum](#common-input-fields), UserPromptExpansion hooks menerima `expansion_type`, `command_name`, `command_args`, `command_source`, dan string `prompt` asli. Bidang `expansion_type` adalah `slash_command` untuk skill dan custom commands, atau `mcp_prompt` untuk MCP server prompts.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../00893aaf.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "UserPromptExpansion",
  "expansion_type": "slash_command",
  "command_name": "example-skill",
  "command_args": "arg1 arg2",
  "command_source": "plugin",
  "prompt": "/example-skill arg1 arg2"
}
```

<h4 id="userpromptexpansion-decision-control">
  UserPromptExpansion decision control
</h4>

Hooks `UserPromptExpansion` dapat memblokir ekspansi atau menambahkan konteks. Semua [bidang output JSON](#json-output) tersedia.

| Bidang              | Deskripsi                                                                                                                            |
| :------------------ | :----------------------------------------------------------------------------------------------------------------------------------- |
| `decision`          | `"block"` mencegah slash command berkembang. Hilangkan untuk mengizinkannya dilanjutkan                                              |
| `reason`            | Ditampilkan ke pengguna saat `decision` adalah `"block"`                                                                             |
| `additionalContext` | String ditambahkan ke konteks Claude bersama prompt yang berkembang. Lihat [Tambahkan konteks untuk Claude](#add-context-for-claude) |

```json theme={null}
{
  "decision": "block",
  "reason": "This slash command is not available",
  "hookSpecificOutput": {
    "hookEventName": "UserPromptExpansion",
    "additionalContext": "Additional context for this expansion"
  }
}
```

<h3 id="messagedisplay">
  MessageDisplay
</h3>

Dijalankan saat pesan asisten mengalir ke layar. Claude Code menampilkan pesan dalam kenaikan: setiap kali batch baris yang baru selesai siap untuk dirender, hook dijalankan sekali dengan baris-baris itu dan Claude Code merender teks pengganti hook di tempatnya. Pesan panjang menghasilkan beberapa panggilan; pesan pendek mungkin hanya menghasilkan satu.

Gunakan MessageDisplay untuk:

* menghapus markdown untuk tampilan minimal
* mengubah teks yang ditampilkan aplikasi Agent SDK kepada penggunanya
* menyensor API keys atau hostname internal dari respons Claude

Claude Code menahan setiap batch sampai hook Anda kembali, jadi jaga hook tetap cepat. Jika hook gagal atau timeout, Claude Code menampilkan teks asli. Timeout default untuk event ini adalah 10 detik; jika hook Anda memerlukan lebih banyak waktu, atur bidang `timeout` dalam entri hook.

MessageDisplay hanya untuk tampilan: teks pengganti hanya mengubah apa yang dirender di layar. Transkrip dan apa yang dilihat Claude menyimpan teks asli, jadi Claude tidak pernah melihat pengganti, dan mode verbose menampilkan asli. Hook menerima teks pesan asisten saja, jadi hasil tool dan teks yang Anda ketik dirender tanpa perubahan.

MessageDisplay tidak mendukung matchers dan dijalankan untuk setiap pesan asisten yang mengalir teks; pesan tanpa teks, seperti respons hanya pemanggilan tool, tidak memicunya.

Dalam run non-interaktif, termasuk kueri Agent SDK dan `claude -p`, MessageDisplay dijalankan sekali per pesan asisten alih-alih sekali per batch baris. Panggilan tunggal tiba setelah pesan selesai dan membawa teks pesan lengkap: `index` adalah `0`, `final` adalah `true`, dan `delta` menyimpan seluruh pesan. Hook yang mengumpulkan teks `delta` untuk setiap pesan menerima teks total yang sama dalam kedua mode.

<h4 id="messagedisplay-input">
  MessageDisplay input
</h4>

Selain [bidang input umum](#common-input-fields), MessageDisplay hooks menerima pengenal untuk giliran dan pesan, posisi panggilan ini dalam pesan, dan teks baru dalam `delta`. Batas batch tergantung pada bagaimana teks mengalir, jadi gunakan `index` dan `final` untuk melacak kemajuan melalui pesan daripada mengharapkan baris dikelompokkan dengan cara tertentu.

| Bidang       | Deskripsi                                                                                                                                                                                                                                                                                                                                                                                                    |
| :----------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `turn_id`    | UUID dari giliran saat ini                                                                                                                                                                                                                                                                                                                                                                                   |
| `message_id` | UUID dari pesan asisten yang ditampilkan. Stabil di setiap batch dari pesan yang sama. Ini bukan API `msg_…` id, jadi tidak dapat dikorelasikan dengan id pesan transkrip                                                                                                                                                                                                                                    |
| `index`      | Indeks berbasis nol dari batch ini dalam pesan                                                                                                                                                                                                                                                                                                                                                               |
| `final`      | `true` pada batch terakhir pesan. Setiap pesan memiliki tepat satu batch final                                                                                                                                                                                                                                                                                                                               |
| `delta`      | Baris yang baru selesai sejak batch sebelumnya, termasuk newline yang mengakhiri. Selalu baris utuh, kecuali batch final yang mungkin berakhir di tengah baris. Dalam run interaktif, delta batch final kosong ketika pesan berakhir pada newline, jadi perlakukan `final`, bukan delta non-kosong, sebagai sinyal akhir pesan. Dalam run Agent SDK dan `claude -p`, panggilan tunggal membawa seluruh pesan |

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../transcript.jsonl",
  "cwd": "/Users/my-project",
  "hook_event_name": "MessageDisplay",
  "turn_id": "0c9e6a2f-7d41-4f4e-9a15-3f4f7c2b8d10",
  "message_id": "5b2a9c8e-1f63-4d8a-b7c4-9e0d2a6f1c3b",
  "index": 0,
  "final": false,
  "delta": "Here is the plan:\n"
}
```

<h4 id="messagedisplay-output">
  MessageDisplay output
</h4>

Selain [bidang output JSON](#json-output) yang tersedia untuk semua hooks, MessageDisplay hooks dapat mengembalikan `displayContent` untuk mengganti delta di layar:

| Bidang           | Deskripsi                                                                  |
| :--------------- | :------------------------------------------------------------------------- |
| `displayContent` | Teks ditampilkan sebagai pengganti delta. Hilangkan untuk menampilkan asli |

MessageDisplay hooks tidak memiliki kontrol keputusan. Mereka tidak dapat memblokir pesan atau mengubah apa yang disimpan dalam transkrip atau dikirim ke Claude.

Contoh ini menghapus pemformatan markdown dari respons Claude untuk tampilan teks biasa. Skrip membaca setiap batch dari stdin, menghapus penanda bold dan backtick kode inline dari `delta`, dan mengembalikan hasilnya sebagai `displayContent`.

<Tabs>
  <Tab title="macOS/Linux">
    Daftarkan command hook untuk event dalam file pengaturan Anda:

    ```json theme={null}
    {
      "hooks": {
        "MessageDisplay": [
          {
            "hooks": [
              {
                "type": "command",
                "command": "${CLAUDE_PROJECT_DIR}/.claude/hooks/plain-display.sh",
                "args": []
              }
            ]
          }
        ]
      }
    }
    ```

    Simpan skrip ini ke `.claude/hooks/plain-display.sh` dalam proyek Anda dan buat dapat dieksekusi dengan `chmod +x`:

    ```bash theme={null}
    #!/bin/bash
    jq '{hookSpecificOutput: {hookEventName: "MessageDisplay", displayContent: (.delta | gsub("\\*\\*"; "") | gsub("`"; ""))}}'
    ```

    Skrip memerlukan `jq` di `PATH` Anda.
  </Tab>

  <Tab title="Windows (PowerShell)">
    Daftarkan command hook yang menjalankan skrip melalui PowerShell:

    ```json theme={null}
    {
      "hooks": {
        "MessageDisplay": [
          {
            "hooks": [
              {
                "type": "command",
                "command": "powershell.exe",
                "args": [
                  "-NoProfile",
                  "-ExecutionPolicy",
                  "Bypass",
                  "-File",
                  "${CLAUDE_PROJECT_DIR}/.claude/hooks/plain-display.ps1"
                ]
              }
            ]
          }
        ]
      }
    }
    ```

    Flag `-NoProfile` melewati pemuatan profil PowerShell Anda sehingga hook dimulai cepat, dan `-ExecutionPolicy Bypass` memungkinkan PowerShell menjalankan file skrip lokal.

    Simpan skrip ini ke `.claude/hooks/plain-display.ps1` dalam proyek Anda:

    ```powershell theme={null}
    $batch = [Console]::In.ReadToEnd() | ConvertFrom-Json
    $text = $batch.delta -replace '\*\*', '' -replace '`', ''
    @{
      hookSpecificOutput = @{
        hookEventName = "MessageDisplay"
        displayContent = $text
      }
    } | ConvertTo-Json
    ```
  </Tab>
</Tabs>

Batch tanpa markdown melewati tanpa perubahan. Jika skrip gagal, misalnya karena `jq` hilang, Claude Code menampilkan teks asli dan mencatat kegagalan hanya dalam [output debug](#debug-hooks), bukan dalam sesi.

<h3 id="pretooluse">
  PreToolUse
</h3>

Dijalankan setelah Claude membuat parameter tool dan sebelum memproses pemanggilan tool. Cocok pada nama tool: `Bash`, `Edit`, `Write`, `Read`, `Glob`, `Grep`, `Agent`, `WebFetch`, `WebSearch`, `AskUserQuestion`, `ExitPlanMode`, dan nama [MCP tool](#match-mcp-tools) apa pun.

<Warning>
  PreToolUse hanya dijalankan ketika Claude memanggil tool. File yang Anda [referensikan dengan `@` dalam prompt Anda](/docs/id/common-workflows#reference-files-and-directories) ditambahkan tanpa pemanggilan tool apa pun: Claude Code menyisipkan konten mereka saat membangun prompt, jadi tidak ada hook PreToolUse yang dijalankan untuk mereka, termasuk hooks yang cocok dengan `Read`. Untuk memblokir path tertentu dari referensi `@`, gunakan [aturan deny `Read`](/docs/id/permissions#read-and-edit) sebagai gantinya.
</Warning>

Gunakan [PreToolUse decision control](#pretooluse-decision-control) untuk mengizinkan, menolak, menanyakan, atau menunda pemanggilan tool.

<h4 id="pretooluse-input">
  PreToolUse input
</h4>

Selain [bidang input umum](#common-input-fields), PreToolUse hooks menerima `tool_name`, `tool_input`, dan `tool_use_id`. Bidang `tool_input` tergantung pada tool:

<h5 id="bash">
  Bash
</h5>

Menjalankan perintah shell.

| Bidang              | Tipe    | Contoh             | Deskripsi                                                                                                                                 |
| :------------------ | :------ | :----------------- | :---------------------------------------------------------------------------------------------------------------------------------------- |
| `command`           | string  | `"npm test"`       | Perintah shell untuk dijalankan                                                                                                           |
| `description`       | string  | `"Run test suite"` | Deskripsi opsional tentang apa yang dilakukan perintah                                                                                    |
| `timeout`           | number  | `120000`           | Timeout opsional dalam milidetik. Nilai di atas [maksimum](/docs/id/tools-reference#bash-tool-behavior) dikurangi ke maksimum daripada ditolak |
| `run_in_background` | boolean | `false`            | Apakah menjalankan perintah di latar belakang                                                                                             |

<h5 id="write">
  Write
</h5>

Membuat atau menimpa file.

| Bidang      | Tipe   | Contoh                | Deskripsi                          |
| :---------- | :----- | :-------------------- | :--------------------------------- |
| `file_path` | string | `"/path/to/file.txt"` | Path absolut ke file untuk ditulis |
| `content`   | string | `"file content"`      | Konten untuk ditulis ke file       |

<h5 id="edit">
  Edit
</h5>

Mengganti string dalam file yang ada.

| Bidang        | Tipe    | Contoh                | Deskripsi                         |
| :------------ | :------ | :-------------------- | :-------------------------------- |
| `file_path`   | string  | `"/path/to/file.txt"` | Path absolut ke file untuk diedit |
| `old_string`  | string  | `"original text"`     | Teks untuk dicari dan diganti     |
| `new_string`  | string  | `"replacement text"`  | Teks pengganti                    |
| `replace_all` | boolean | `false`               | Apakah mengganti semua kemunculan |

<h5 id="read">
  Read
</h5>

Membaca konten file.

| Bidang      | Tipe   | Contoh                | Deskripsi                                     |
| :---------- | :----- | :-------------------- | :-------------------------------------------- |
| `file_path` | string | `"/path/to/file.txt"` | Path absolut ke file untuk dibaca             |
| `offset`    | number | `10`                  | Nomor baris opsional untuk mulai membaca dari |
| `limit`     | number | `50`                  | Jumlah baris opsional untuk dibaca            |

<h5 id="glob">
  Glob
</h5>

Menemukan file yang cocok dengan pola glob.

| Bidang    | Tipe   | Contoh           | Deskripsi                                                            |
| :-------- | :----- | :--------------- | :------------------------------------------------------------------- |
| `pattern` | string | `"**/*.ts"`      | Pola glob untuk mencocokkan file terhadap                            |
| `path`    | string | `"/path/to/dir"` | Direktori opsional untuk dicari. Default ke direktori kerja saat ini |

<h5 id="grep">
  Grep
</h5>

Mencari konten file dengan ekspresi reguler.

| Bidang        | Tipe    | Contoh           | Deskripsi                                                                              |
| :------------ | :------ | :--------------- | :------------------------------------------------------------------------------------- |
| `pattern`     | string  | `"TODO.*fix"`    | Pola ekspresi reguler untuk dicari                                                     |
| `path`        | string  | `"/path/to/dir"` | File atau direktori opsional untuk dicari                                              |
| `glob`        | string  | `"*.ts"`         | Pola glob opsional untuk memfilter file                                                |
| `output_mode` | string  | `"content"`      | `"content"`, `"files_with_matches"`, atau `"count"`. Default ke `"files_with_matches"` |
| `-i`          | boolean | `true`           | Pencarian case insensitive                                                             |
| `multiline`   | boolean | `false`          | Aktifkan pencocokan multiline                                                          |

<h5 id="webfetch">
  WebFetch
</h5>

Mengambil dan memproses konten web.

| Bidang   | Tipe   | Contoh                        | Deskripsi                                        |
| :------- | :----- | :---------------------------- | :----------------------------------------------- |
| `url`    | string | `"https://example.com/api"`   | URL untuk mengambil konten dari                  |
| `prompt` | string | `"Extract the API endpoints"` | Prompt untuk dijalankan pada konten yang diambil |

<h5 id="websearch">
  WebSearch
</h5>

Mencari web.

| Bidang            | Tipe   | Contoh                         | Deskripsi                                      |
| :---------------- | :----- | :----------------------------- | :--------------------------------------------- |
| `query`           | string | `"react hooks best practices"` | Query pencarian                                |
| `allowed_domains` | array  | `["docs.example.com"]`         | Opsional: hanya sertakan hasil dari domain ini |
| `blocked_domains` | array  | `["spam.example.com"]`         | Opsional: kecualikan hasil dari domain ini     |

<h5 id="agent">
  Agent
</h5>

Spawn [subagent](/docs/id/sub-agents).

| Bidang          | Tipe   | Contoh                     | Deskripsi                                  |
| :-------------- | :----- | :------------------------- | :----------------------------------------- |
| `prompt`        | string | `"Find all API endpoints"` | Tugas untuk agent lakukan                  |
| `description`   | string | `"Find API endpoints"`     | Deskripsi singkat tugas                    |
| `subagent_type` | string | `"Explore"`                | Tipe agent khusus untuk digunakan          |
| `model`         | string | `"sonnet"`                 | Alias model opsional untuk menimpa default |

Dalam `PostToolUse`, `tool_response` untuk panggilan Agent yang selesai membawa teks akhir subagent bersama dengan telemetri penggunaan. Baca bidang-bidang ini untuk mencatat biaya per-subagent dari hook:

| Bidang              | Tipe   | Contoh                                                | Deskripsi                                                                                                                                                                                                                                                   |
| :------------------ | :----- | :---------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `status`            | string | `"completed"`                                         | `"completed"` untuk panggilan foreground subagents, `"async_launched"` untuk background subagents. Mulai dari v2.1.198, subagents berjalan di latar belakang secara default, jadi `run_in_background` yang dihilangkan juga menghasilkan `"async_launched"` |
| `agentId`           | string | `"a4d2c8f1e0b3a297"`                                  | Pengenal untuk run subagent                                                                                                                                                                                                                                 |
| `content`           | array  | `[{"type": "text", "text": "Found 12 endpoints..."}]` | Blok teks akhir subagent                                                                                                                                                                                                                                    |
| `resolvedModel`     | string | `"claude-sonnet-4-5"`                                 | Model yang dijalankan subagent, yang mungkin berbeda dari model yang diminta. Memerlukan Claude Code v2.1.174 atau lebih baru                                                                                                                               |
| `totalTokens`       | number | `12450`                                               | Total tokens yang ditagih di seluruh giliran subagent                                                                                                                                                                                                       |
| `totalDurationMs`   | number | `48211`                                               | Durasi wall-clock dari run subagent                                                                                                                                                                                                                         |
| `totalToolUseCount` | number | `7`                                                   | Jumlah pemanggilan tool yang dibuat subagent                                                                                                                                                                                                                |
| `usage`             | object | `{"input_tokens": 8320, ...}`                         | Breakdown token per-tipe: `input_tokens`, `output_tokens`, `cache_creation_input_tokens`, `cache_read_input_tokens`                                                                                                                                         |

Untuk background subagents, tool mengembalikan segera setelah meluncurkan, jadi `tool_response` tidak membawa bidang penggunaan. Itu memiliki `status: "async_launched"`, `agentId`, `description`, `prompt`, `outputFile`, dan `resolvedModel`.

Bidang `resolvedModel` menamai model yang sebenarnya dijalankan subagent, yang dapat berbeda dari nilai `model` dalam `tool_input`. Itu memerlukan Claude Code v2.1.174 atau lebih baru.

<a id="askuserquestion" />

<h5 id="askuserquestion">
  AskUserQuestion
</h5>

Mengajukan pertanyaan multiple-choice satu hingga empat kepada pengguna.

| Bidang      | Tipe   | Contoh                                                                                                             | Deskripsi                                                                                                                                                                                                                   |
| :---------- | :----- | :----------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `questions` | array  | `[{"question": "Which framework?", "header": "Framework", "options": [{"label": "React"}], "multiSelect": false}]` | Pertanyaan untuk disajikan, masing-masing dengan string `question`, `header` pendek, array `options`, dan flag `multiSelect` opsional                                                                                       |
| `answers`   | object | `{"Which framework?": "React"}`                                                                                    | Opsional. Memetakan teks pertanyaan ke label opsi yang dipilih. Jawaban multi-select menggabungkan label dengan koma. Claude tidak menetapkan bidang ini; sediakan melalui `updatedInput` untuk menjawab secara programatis |

<h5 id="exitplanmode">
  ExitPlanMode
</h5>

Menyajikan rencana dan meminta pengguna untuk menyetujuinya sebelum Claude meninggalkan [plan mode](/docs/id/permission-modes#analyze-before-you-edit-with-plan-mode). Claude menulis rencana ke file di disk sebelum memanggil tool, jadi `tool_input` literal dari model hanya membawa `allowedPrompts`. Claude Code menyuntikkan konten rencana dan path file sebelum meneruskan input ke hooks.

| Bidang           | Tipe   | Contoh                                      | Deskripsi                                                                                                                                                          |
| :--------------- | :----- | :------------------------------------------ | :----------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `plan`           | string | `"## Refactor auth\n1. Extract..."`         | Konten rencana dalam Markdown. Disuntikkan dari file rencana di disk                                                                                               |
| `planFilePath`   | string | `"/Users/.../plans/refactor-auth.md"`       | Path ke file rencana. Disuntikkan                                                                                                                                  |
| `allowedPrompts` | array  | `[{"tool": "Bash", "prompt": "run tests"}]` | Usang. Claude Code menerima bidang tetapi mengabaikannya. Sebelum v2.1.205, itu membawa izin berbasis prompt yang diminta Claude untuk mengimplementasikan rencana |

Dalam `PostToolUse`, `tool_response` adalah objek dengan bidang `plan` dan `filePath` yang menyimpan rencana yang disetujui, ditambah flag status internal. Baca `tool_response.plan` untuk konten rencana daripada membaca ulang file dari disk.

<h4 id="pretooluse-decision-control">
  PreToolUse decision control
</h4>

Hooks `PreToolUse` dapat mengontrol apakah pemanggilan tool dilanjutkan. Tidak seperti hooks lain yang menggunakan bidang `decision` tingkat atas, PreToolUse mengembalikan keputusannya di dalam objek `hookSpecificOutput`. Ini memberikannya kontrol yang lebih kaya: empat hasil (izinkan, tolak, tanya, atau tunda) ditambah kemampuan untuk memodifikasi input tool sebelum eksekusi.

| Bidang                     | Deskripsi                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| :------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `permissionDecision`       | `"allow"` melewati prompt izin, kecuali untuk [tools yang memerlukan interaksi pengguna](#pretooluse-decision-control) dan connector tools [organisasi Anda atur ke `ask`](/docs/id/mcp#organization-controls-on-connector-tools). `"deny"` mencegah pemanggilan tool. `"ask"` meminta pengguna untuk mengkonfirmasi. `"defer"` keluar dengan baik sehingga tool dapat dilanjutkan nanti. [Deny and ask rules](/docs/id/permissions#manage-permissions) masih dievaluasi terlepas dari apa yang dikembalikan hook |
| `permissionDecisionReason` | Untuk `"allow"` dan `"ask"`, ditampilkan ke pengguna tetapi bukan Claude. Untuk `"deny"`, ditampilkan ke Claude. Untuk `"defer"`, diabaikan                                                                                                                                                                                                                                                                                                                                                             |
| `updatedInput`             | Memodifikasi parameter input tool sebelum eksekusi. Menggantikan seluruh objek input, jadi sertakan bidang yang tidak berubah bersama yang dimodifikasi. Gabungkan dengan `"allow"` untuk persetujuan otomatis, atau `"ask"` untuk menampilkan input yang dimodifikasi ke pengguna. Untuk `"defer"`, diabaikan                                                                                                                                                                                          |
| `additionalContext`        | String ditambahkan ke konteks Claude bersama hasil tool. Diabaikan ketika `permissionDecision` adalah `"defer"`. Lihat [Tambahkan konteks untuk Claude](#add-context-for-claude)                                                                                                                                                                                                                                                                                                                        |

Ketika beberapa PreToolUse hooks mengembalikan keputusan berbeda, prioritas adalah `deny` > `defer` > `ask` > `allow`.

Ketika hook mengembalikan `"ask"`, dialog izin yang ditampilkan kepada pengguna mencakup label yang mengidentifikasi dari mana hook berasal: misalnya, `[User]`, `[Project]`, `[Plugin]`, atau `[Local]`. Ini membantu pengguna memahami sumber konfigurasi mana yang meminta konfirmasi.

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PreToolUse",
    "permissionDecision": "allow",
    "permissionDecisionReason": "My reason here",
    "updatedInput": {
      "field_to_modify": "new value"
    },
    "additionalContext": "Current environment: production. Proceed with caution."
  }
}
```

`AskUserQuestion` dan `ExitPlanMode` memerlukan interaksi pengguna dan biasanya memblokir dalam [mode non-interaktif](/docs/id/headless) dengan flag `-p`. Mengembalikan `permissionDecision: "allow"` bersama dengan `updatedInput` memenuhi persyaratan itu: hook membaca input tool dari stdin, mengumpulkan jawaban melalui UI Anda sendiri, dan mengembalikannya dalam `updatedInput` sehingga tool dijalankan tanpa meminta. Mengembalikan `"allow"` saja tidak cukup untuk tools ini. Untuk `AskUserQuestion`, kembalikan array `questions` asli dan tambahkan objek [`answers`](#askuserquestion) yang memetakan teks setiap pertanyaan ke jawaban yang dipilih.

Connector tools [organisasi Anda atur ke `ask`](/docs/id/mcp#organization-controls-on-connector-tools) meminta bahkan ketika hook mengembalikan `"allow"`.

Mulai dari v2.1.199, tool MCP yang server-nya menandainya dengan [`_meta["anthropic/requiresUserInteraction"]`](/docs/id/mcp#require-approval-for-a-specific-tool) lebih ketat: hook tidak dapat melewati prompt persetujuannya dengan `"allow"`, dengan atau tanpa `updatedInput`, karena Claude Code tidak dapat mengkonfirmasi hook mengumpulkan interaksi yang dibutuhkan tool.

<Note>
  PreToolUse sebelumnya menggunakan bidang `decision` dan `reason` tingkat atas, tetapi ini sudah usang untuk event ini. Gunakan `hookSpecificOutput.permissionDecision` dan `hookSpecificOutput.permissionDecisionReason` sebagai gantinya. Nilai usang `"approve"` dan `"block"` memetakan ke `"allow"` dan `"deny"` masing-masing. Events lain seperti PostToolUse dan Stop terus menggunakan `decision` dan `reason` tingkat atas sebagai format saat ini mereka.
</Note>

<h4 id="defer-a-tool-call-for-later">
  Defer a tool call for later
</h4>

`"defer"` adalah untuk integrasi yang menjalankan `claude -p` sebagai subprocess dan membaca output JSON-nya, seperti aplikasi Agent SDK atau UI kustom yang dibangun di atas Claude Code. Ini memungkinkan proses pemanggil itu menjeda Claude pada pemanggilan tool, mengumpulkan input melalui antarmuka miliknya sendiri, dan melanjutkan di mana ia berhenti. Claude Code menghormati nilai ini hanya dalam [mode non-interaktif](/docs/id/headless) dengan flag `-p`. Dalam sesi interaktif itu mencatat peringatan dan mengabaikan hasil hook.

Tool `AskUserQuestion` adalah kasus tipikal: Claude ingin menanyakan sesuatu kepada pengguna, tetapi tidak ada terminal untuk menjawab. Perjalanan bolak-balik bekerja seperti ini:

1. Claude memanggil `AskUserQuestion`. Hook `PreToolUse` dijalankan.
2. Hook mengembalikan `permissionDecision: "defer"`. Tool tidak dijalankan. Proses keluar dengan `stop_reason: "tool_deferred"` dan pemanggilan tool yang tertunda dipertahankan dalam transkrip.
3. Proses pemanggil membaca `deferred_tool_use` dari hasil SDK, menampilkan pertanyaan di UI miliknya sendiri, dan menunggu jawaban.
4. Proses pemanggil menjalankan `claude -p --resume <session-id>`. Pemanggilan tool yang sama menjalankan `PreToolUse` lagi.
5. Hook mengembalikan `permissionDecision: "allow"` dengan jawaban dalam `updatedInput`. Tool dijalankan dan Claude melanjutkan.

Bidang `deferred_tool_use` membawa `id`, `name`, dan `input` tool. `input` adalah parameter yang Claude hasilkan untuk pemanggilan tool, ditangkap sebelum eksekusi:

```json theme={null}
{
  "type": "result",
  "subtype": "success",
  "stop_reason": "tool_deferred",
  "session_id": "abc123",
  "deferred_tool_use": {
    "id": "toolu_01abc",
    "name": "AskUserQuestion",
    "input": { "questions": [{ "question": "Which framework?", "header": "Framework", "options": [{"label": "React"}, {"label": "Vue"}], "multiSelect": false }] }
  }
}
```

Tidak ada timeout atau batas retry. Sesi tetap di disk sampai Anda melanjutkannya, tunduk pada penyapuan retensi [`cleanupPeriodDays`](/docs/id/settings#available-settings) yang menghapus file sesi setelah 30 hari secara default. Jika jawaban tidak siap saat Anda melanjutkan, hook dapat mengembalikan `"defer"` lagi dan proses keluar dengan cara yang sama. Proses pemanggil mengontrol kapan harus memecah loop dengan akhirnya mengembalikan `"allow"` atau `"deny"` dari hook.

`"defer"` hanya bekerja ketika Claude membuat satu pemanggilan tool dalam giliran. Jika Claude membuat beberapa pemanggilan tool sekaligus, `"defer"` diabaikan dengan peringatan dan tool melanjutkan melalui alur izin normal. Batasan ada karena resume hanya dapat menjalankan kembali satu tool: tidak ada cara untuk menunda satu pemanggilan dari batch tanpa meninggalkan yang lain tidak terselesaikan.

Jika tool yang ditunda tidak lagi tersedia saat Anda melanjutkan, proses keluar dengan `stop_reason: "tool_deferred_unavailable"` dan `is_error: true` sebelum hook dijalankan. Ini terjadi ketika server MCP yang menyediakan tool tidak terhubung untuk sesi yang dilanjutkan. Payload `deferred_tool_use` masih disertakan sehingga Anda dapat mengidentifikasi tool mana yang hilang.

<Note>
  `--resume` memulihkan mode izin yang aktif saat tool ditunda, jadi Anda tidak perlu meneruskan `--permission-mode` lagi. Pengecualiannya adalah `plan` dan `bypassPermissions`, yang tidak pernah dibawa. Meneruskan `--permission-mode` secara eksplisit pada resume menimpa nilai yang dipulihkan.
</Note>

<h3 id="permissionrequest">
  PermissionRequest
</h3>

Dijalankan ketika pengguna ditampilkan dialog izin.
Gunakan [PermissionRequest decision control](#permissionrequest-decision-control) untuk mengizinkan atau menolak atas nama pengguna.

Cocok pada nama tool, nilai yang sama seperti PreToolUse.

<h4 id="permissionrequest-input">
  PermissionRequest input
</h4>

PermissionRequest hooks menerima bidang `tool_name` dan `tool_input` seperti PreToolUse hooks, tetapi tanpa `tool_use_id`. Array `permission_suggestions` opsional berisi opsi "selalu izinkan" yang biasanya dilihat pengguna dalam dialog izin. Perbedaannya adalah kapan hook dijalankan: PermissionRequest hooks dijalankan ketika dialog izin akan ditampilkan ke pengguna, sementara PreToolUse hooks dijalankan sebelum eksekusi tool terlepas dari status izin.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "PermissionRequest",
  "tool_name": "Bash",
  "tool_input": {
    "command": "rm -rf node_modules",
    "description": "Remove node_modules directory"
  },
  "permission_suggestions": [
    {
      "type": "addRules",
      "rules": [{ "toolName": "Bash", "ruleContent": "rm -rf node_modules" }],
      "behavior": "allow",
      "destination": "localSettings"
    }
  ]
}
```

<h4 id="permissionrequest-decision-control">
  PermissionRequest decision control
</h4>

Hooks `PermissionRequest` dapat mengizinkan atau menolak permintaan izin. Selain [bidang output JSON](#json-output) yang tersedia untuk semua hooks, skrip hook Anda dapat mengembalikan objek `decision` dengan bidang spesifik event ini:

| Bidang               | Deskripsi                                                                                                                                                                                                                                            |
| :------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `behavior`           | `"allow"` memberikan izin, `"deny"` menolaknya. [Deny and ask rules](/docs/id/permissions#manage-permissions) masih dievaluasi, jadi hook yang mengembalikan `"allow"` tidak menimpa aturan deny yang cocok                                               |
| `updatedInput`       | Untuk `"allow"` saja: memodifikasi parameter input tool sebelum eksekusi. Menggantikan seluruh objek input, jadi sertakan bidang yang tidak berubah bersama yang dimodifikasi. Input yang dimodifikasi dievaluasi ulang terhadap aturan deny dan ask |
| `updatedPermissions` | Untuk `"allow"` saja: array dari [permission update entries](#permission-update-entries) untuk diterapkan, seperti menambahkan aturan allow atau mengubah mode izin sesi                                                                             |
| `message`            | Untuk `"deny"` saja: memberitahu Claude mengapa izin ditolak                                                                                                                                                                                         |
| `interrupt`          | Untuk `"deny"` saja: jika `true`, menghentikan Claude                                                                                                                                                                                                |

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PermissionRequest",
    "decision": {
      "behavior": "allow",
      "updatedInput": {
        "command": "npm run lint"
      }
    }
  }
}
```

<h4 id="permission-update-entries">
  Permission update entries
</h4>

Bidang output `updatedPermissions` dan bidang input [`permission_suggestions`](#permissionrequest-input) keduanya menggunakan array objek entry yang sama. Setiap entry memiliki `type` yang menentukan bidang lainnya, dan `destination` yang mengontrol di mana perubahan ditulis.

| `type`              | Bidang                             | Efek                                                                                                                                                                                                                            |
| :------------------ | :--------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `addRules`          | `rules`, `behavior`, `destination` | Menambahkan aturan izin. `rules` adalah array dari objek `{toolName, ruleContent?}`. Hilangkan `ruleContent` untuk mencocokkan seluruh tool. `behavior` adalah `"allow"`, `"deny"`, atau `"ask"`                                |
| `replaceRules`      | `rules`, `behavior`, `destination` | Mengganti semua aturan dari `behavior` yang diberikan di `destination` dengan `rules` yang disediakan                                                                                                                           |
| `removeRules`       | `rules`, `behavior`, `destination` | Menghapus aturan yang cocok dari `behavior` yang diberikan                                                                                                                                                                      |
| `setMode`           | `mode`, `destination`              | Mengubah mode izin. Mode yang valid adalah `default`, `auto`, `acceptEdits`, `dontAsk`, `bypassPermissions`, `plan`, dan `manual` sebagai alias untuk `default`. Alias `manual` memerlukan Claude Code v2.1.200 atau lebih baru |
| `addDirectories`    | `directories`, `destination`       | Menambahkan direktori kerja. `directories` adalah array dari string path                                                                                                                                                        |
| `removeDirectories` | `directories`, `destination`       | Menghapus direktori kerja                                                                                                                                                                                                       |

<Note>
  `setMode` dengan `bypassPermissions` hanya berlaku jika sesi diluncurkan dengan mode bypass sudah tersedia: `--dangerously-skip-permissions`, `--permission-mode bypassPermissions`, `--allow-dangerously-skip-permissions`, atau `permissions.defaultMode: "bypassPermissions"` dalam pengaturan, dan mode tidak dinonaktifkan oleh [`permissions.disableBypassPermissionsMode`](/docs/id/permissions#managed-settings). Jika tidak, update adalah no-op. `bypassPermissions` tidak pernah dipertahankan sebagai `defaultMode` terlepas dari `destination`.
</Note>

Bidang `destination` pada setiap entry menentukan apakah perubahan tetap dalam memori atau persisten ke file pengaturan.

| `destination`     | Menulis ke                                       |
| :---------------- | :----------------------------------------------- |
| `session`         | hanya dalam memori, dibuang ketika sesi berakhir |
| `localSettings`   | `.claude/settings.local.json`                    |
| `projectSettings` | `.claude/settings.json`                          |
| `userSettings`    | `~/.claude/settings.json`                        |

Hook dapat mengembalikan salah satu dari `permission_suggestions` yang diterima sebagai output `updatedPermissions` miliknya sendiri, yang setara dengan pengguna memilih opsi "selalu izinkan" itu dalam dialog.

<h3 id="posttooluse">
  PostToolUse
</h3>

Dijalankan segera setelah tool selesai dengan sukses.

Cocok pada nama tool, nilai yang sama seperti PreToolUse.

<h4 id="posttooluse-input">
  PostToolUse input
</h4>

Hooks `PostToolUse` dijalankan setelah tool sudah dijalankan dengan sukses. Input mencakup `tool_input`, argumen yang dikirim ke tool, dan `tool_response`, hasil yang dikembalikan. Skema yang tepat untuk keduanya tergantung pada tool.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "PostToolUse",
  "tool_name": "Write",
  "tool_input": {
    "file_path": "/path/to/file.txt",
    "content": "file content"
  },
  "tool_response": {
    "filePath": "/path/to/file.txt",
    "success": true
  },
  "tool_use_id": "toolu_01ABC123...",
  "duration_ms": 12
}
```

| Bidang        | Deskripsi                                                                                                                 |
| :------------ | :------------------------------------------------------------------------------------------------------------------------ |
| `duration_ms` | Opsional. Waktu eksekusi tool dalam milidetik. Mengecualikan waktu yang dihabiskan dalam prompt izin dan PreToolUse hooks |

<h4 id="posttooluse-decision-control">
  PostToolUse decision control
</h4>

Hooks `PostToolUse` dapat memberikan umpan balik ke Claude setelah eksekusi tool. Selain [bidang output JSON](#json-output) yang tersedia untuk semua hooks, skrip hook Anda dapat mengembalikan bidang spesifik event ini:

| Bidang                 | Deskripsi                                                                                                                               |
| :--------------------- | :-------------------------------------------------------------------------------------------------------------------------------------- |
| `decision`             | `"block"` menambahkan `reason` di sebelah hasil tool. Claude masih melihat output asli; untuk menggantinya, gunakan `updatedToolOutput` |
| `reason`               | Penjelasan ditampilkan ke Claude saat `decision` adalah `"block"`                                                                       |
| `additionalContext`    | String ditambahkan ke konteks Claude bersama hasil tool. Lihat [Tambahkan konteks untuk Claude](#add-context-for-claude)                |
| `updatedToolOutput`    | Mengganti output tool dengan nilai yang disediakan sebelum dikirim ke Claude. Nilai harus cocok dengan bentuk output tool               |
| `updatedMCPToolOutput` | Mengganti output untuk [MCP tools](#match-mcp-tools) saja. Lebih suka `updatedToolOutput`, yang bekerja untuk semua tools               |

Contoh di bawah mengganti output pemanggilan `Bash`. Nilai pengganti cocok dengan bentuk output tool `Bash`:

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PostToolUse",
    "additionalContext": "Additional information for Claude",
    "updatedToolOutput": {
      "stdout": "[redacted]",
      "stderr": "",
      "interrupted": false,
      "isImage": false
    }
  }
}
```

<Warning>
  `updatedToolOutput` hanya mengubah apa yang dilihat Claude. Tool sudah dijalankan pada saat hook dijalankan, jadi file apa pun yang ditulis, perintah yang dijalankan, atau permintaan jaringan yang dikirim sudah berlaku. Telemetri seperti span tool OpenTelemetry dan acara analitik juga menangkap output asli sebelum hook dijalankan. Untuk mencegah atau memodifikasi pemanggilan tool sebelum dijalankan, gunakan hook [PreToolUse](#pretooluse) sebagai gantinya.

  Nilai pengganti harus cocok dengan bentuk output tool. Tools bawaan mengembalikan objek terstruktur daripada string biasa. Misalnya, `Bash` mengembalikan objek dengan bidang `stdout`, `stderr`, `interrupted`, dan `isImage`. Untuk tools bawaan, nilai yang tidak cocok dengan skema output tool diabaikan dan output asli digunakan. Output tool MCP dilewatkan tanpa validasi skema. Menghapus detail kesalahan yang Claude butuhkan dapat menyebabkannya melanjutkan dengan asumsi yang salah.
</Warning>

<h3 id="posttoolusefailure">
  PostToolUseFailure
</h3>

Dijalankan ketika tool yang mulai dijalankan gagal: tool melempar kesalahan, atau tool MCP mengembalikan hasil kesalahan. Gunakan ini untuk mencatat kegagalan, mengirim alert, atau memberikan umpan balik korektif ke Claude.

Cocok pada nama tool, nilai yang sama seperti PreToolUse.

<Note>
  Event ini tidak dijalankan untuk pemanggilan tool yang ditolak sebelum eksekusi: nama tool yang tidak dikenal, input yang gagal validasi skema atau tool-spesifik, atau penolakan izin. Penolakan validasi dikembalikan sebagai hasil `tool_use_error` dan terjadi sebelum hooks dijalankan, jadi mereka tidak menjalankan `PreToolUse` atau event ini. Penolakan izin menjalankan `PreToolUse` tetapi bukan event ini; lihat [PermissionDenied](#permissiondenied).
</Note>

<h4 id="posttoolusefailure-input">
  PostToolUseFailure input
</h4>

PostToolUseFailure hooks menerima bidang `tool_name` dan `tool_input` yang sama seperti PostToolUse, bersama dengan informasi kesalahan sebagai bidang tingkat atas:

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "PostToolUseFailure",
  "tool_name": "Bash",
  "tool_input": {
    "command": "npm test",
    "description": "Run test suite"
  },
  "tool_use_id": "toolu_01ABC123...",
  "error": "Command exited with non-zero status code 1",
  "is_interrupt": false,
  "duration_ms": 4187
}
```

| Bidang         | Deskripsi                                                                                                                 |
| :------------- | :------------------------------------------------------------------------------------------------------------------------ |
| `error`        | String menjelaskan apa yang salah                                                                                         |
| `is_interrupt` | Boolean opsional menunjukkan apakah kegagalan disebabkan oleh interupsi pengguna                                          |
| `duration_ms`  | Opsional. Waktu eksekusi tool dalam milidetik. Mengecualikan waktu yang dihabiskan dalam prompt izin dan PreToolUse hooks |

<h4 id="posttoolusefailure-decision-control">
  PostToolUseFailure decision control
</h4>

Hooks `PostToolUseFailure` dapat memberikan konteks ke Claude setelah kegagalan tool. Selain [bidang output JSON](#json-output) yang tersedia untuk semua hooks, skrip hook Anda dapat mengembalikan bidang spesifik event ini:

| Bidang              | Deskripsi                                                                                                               |
| :------------------ | :---------------------------------------------------------------------------------------------------------------------- |
| `additionalContext` | String ditambahkan ke konteks Claude bersama kesalahan. Lihat [Tambahkan konteks untuk Claude](#add-context-for-claude) |

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PostToolUseFailure",
    "additionalContext": "Additional information about the failure for Claude"
  }
}
```

<h3 id="posttoolbatch">
  PostToolBatch
</h3>

Dijalankan sekali setelah setiap tool call dalam batch telah terselesaikan, sebelum Claude Code mengirimkan permintaan berikutnya ke model. `PostToolUse` dijalankan sekali per tool, yang berarti dijalankan secara bersamaan ketika Claude membuat pemanggilan tool paralel. `PostToolBatch` dijalankan tepat sekali dengan batch lengkap, jadi ini adalah tempat yang tepat untuk menyuntikkan konteks yang bergantung pada set tools yang dijalankan daripada pada tool tunggal. Tidak ada matcher untuk event ini.

<h4 id="posttoolbatch-input">
  PostToolBatch input
</h4>

Selain [bidang input umum](#common-input-fields), PostToolBatch hooks menerima `tool_calls`, array yang menjelaskan setiap pemanggilan tool dalam batch:

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "PostToolBatch",
  "tool_calls": [
    {
      "tool_name": "Read",
      "tool_input": {"file_path": "/.../ledger/accounts.py"},
      "tool_use_id": "toolu_01...",
      "tool_response": "     1\tfrom __future__ import annotations\n     2\t..."
    },
    {
      "tool_name": "Read",
      "tool_input": {"file_path": "/.../ledger/transactions.py"},
      "tool_use_id": "toolu_02...",
      "tool_response": "     1\tfrom __future__ import annotations\n     2\t..."
    }
  ]
}
```

`tool_response` berisi konten yang sama yang diterima model dalam blok `tool_result` yang sesuai. Nilainya adalah string yang diserialisasi atau array blok konten, persis seperti yang dikeluarkan tool. Untuk `Read`, itu berarti teks dengan awalan nomor baris daripada konten file mentah. Respons dapat besar, jadi hanya parse bidang yang Anda butuhkan.

<Note>
  Bentuk `tool_response` berbeda dari `PostToolUse`. `PostToolUse` meneruskan objek `Output` terstruktur tool, seperti `{filePath: "...", success: true}` untuk `Write`; `PostToolBatch` meneruskan konten `tool_result` yang diserialisasi yang dilihat model.
</Note>

<h4 id="posttoolbatch-decision-control">
  PostToolBatch decision control
</h4>

Hooks `PostToolBatch` dapat menyuntikkan konteks untuk Claude. Selain [bidang output JSON](#json-output) yang tersedia untuk semua hooks, skrip hook Anda dapat mengembalikan bidang spesifik event ini:

| Bidang              | Deskripsi                                                                                                                                                                                                                                               |
| :------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `additionalContext` | String konteks yang disuntikkan sekali sebelum panggilan model berikutnya. Lihat [Tambahkan konteks untuk Claude](#add-context-for-claude) untuk detail pengiriman, apa yang harus dimasukkan, dan cara sesi yang dilanjutkan menangani nilai masa lalu |

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PostToolBatch",
    "additionalContext": "These files are part of the ledger module. Run pytest before marking the task complete."
  }
}
```

Mengembalikan `decision: "block"` atau `continue: false` menghentikan loop agentic sebelum panggilan model berikutnya.

<h3 id="permissiondenied">
  PermissionDenied
</h3>

Dijalankan ketika pengklasifikasi [mode otomatis](/docs/id/permission-modes#eliminate-prompts-with-auto-mode) menolak pemanggilan tool. Hook ini hanya dijalankan dalam mode otomatis: itu tidak dijalankan ketika Anda secara manual menolak dialog izin, ketika hook `PreToolUse` memblokir pemanggilan, atau ketika aturan `deny` cocok. Gunakan untuk mencatat penolakan pengklasifikasi, menyesuaikan konfigurasi, atau memberitahu model itu dapat mencoba lagi pemanggilan tool.

Cocok pada nama tool, nilai yang sama seperti PreToolUse.

<h4 id="permissiondenied-input">
  PermissionDenied input
</h4>

Selain [bidang input umum](#common-input-fields), PermissionDenied hooks menerima `tool_name`, `tool_input`, `tool_use_id`, dan `reason`.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "auto",
  "hook_event_name": "PermissionDenied",
  "tool_name": "Bash",
  "tool_input": {
    "command": "rm -rf /tmp/build",
    "description": "Clean build directory"
  },
  "tool_use_id": "toolu_01ABC123...",
  "reason": "Auto mode denied: command targets a path outside the project"
}
```

| Bidang   | Deskripsi                                                           |
| :------- | :------------------------------------------------------------------ |
| `reason` | Penjelasan pengklasifikasi tentang mengapa pemanggilan tool ditolak |

<h4 id="permissiondenied-decision-control">
  PermissionDenied decision control
</h4>

PermissionDenied hooks dapat memberitahu model itu dapat mencoba lagi pemanggilan tool yang ditolak. Kembalikan objek JSON dengan `hookSpecificOutput.retry` diatur ke `true`:

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PermissionDenied",
    "retry": true
  }
}
```

Ketika `retry` adalah `true`, Claude Code menambahkan pesan ke percakapan memberitahu model itu dapat mencoba lagi pemanggilan tool. Penolakan itu sendiri tidak dibatalkan. Jika hook Anda tidak mengembalikan JSON, atau mengembalikan `retry: false`, penolakan tetap dan model menerima pesan penolakan asli.

<h3 id="notification">
  Notification
</h3>

Dijalankan ketika Claude Code mengirimkan notifikasi. Cocok pada tipe notifikasi. Hilangkan matcher untuk menjalankan hooks untuk semua tipe notifikasi.

| Matcher                | Kapan dijalankan                                                                                                      |
| :--------------------- | :-------------------------------------------------------------------------------------------------------------------- |
| `permission_prompt`    | Claude memerlukan persetujuan tool use                                                                                |
| `idle_prompt`          | Claude selesai dan menunggu prompt berikutnya Anda                                                                    |
| `auth_success`         | Autentikasi selesai                                                                                                   |
| `elicitation_dialog`   | Server MCP membuka formulir elicitation                                                                               |
| `elicitation_complete` | Formulir elicitation MCP dikirimkan atau ditutup                                                                      |
| `elicitation_response` | Respons elicitation MCP dikirim kembali ke server                                                                     |
| `agent_needs_input`    | Sesi latar belakang mulai menunggu input Anda. Dijalankan hanya saat [agent view](/docs/id/agent-view) terbuka di terminal |
| `agent_completed`      | Sesi latar belakang selesai atau gagal. Dijalankan hanya saat [agent view](/docs/id/agent-view) terbuka di terminal        |

Tipe `agent_needs_input` dan `agent_completed` memerlukan Claude Code v2.1.198 atau lebih baru.

Gunakan matchers terpisah untuk menjalankan handler berbeda tergantung pada tipe notifikasi. Konfigurasi ini memicu skrip alert khusus izin ketika Claude memerlukan persetujuan izin dan notifikasi berbeda ketika Claude telah idle:

```json theme={null}
{
  "hooks": {
    "Notification": [
      {
        "matcher": "permission_prompt",
        "hooks": [
          {
            "type": "command",
            "command": "/path/to/permission-alert.sh"
          }
        ]
      },
      {
        "matcher": "idle_prompt",
        "hooks": [
          {
            "type": "command",
            "command": "/path/to/idle-notification.sh"
          }
        ]
      }
    ]
  }
}
```

<h4 id="notification-input">
  Notification input
</h4>

Selain [bidang input umum](#common-input-fields), Notification hooks menerima `message` dengan teks notifikasi, `title` opsional, dan `notification_type` menunjukkan tipe mana yang dijalankan.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "Notification",
  "message": "Claude needs your permission",
  "title": "Permission needed",
  "notification_type": "permission_prompt"
}
```

Notification hooks tidak dapat memblokir atau memodifikasi notifikasi. Mereka dimaksudkan untuk efek samping seperti meneruskan notifikasi ke layanan eksternal. [Bidang output JSON](#json-output) umum seperti `systemMessage` berlaku.

<h3 id="subagentstart">
  SubagentStart
</h3>

Dijalankan ketika subagent Claude Code dispawn melalui tool Agent. Mendukung matchers untuk memfilter berdasarkan nama tipe agent. Untuk agent bawaan, ini adalah nama agent seperti `general-purpose`, `Explore`, atau `Plan`. Untuk [custom subagents](/docs/id/sub-agents), ini adalah bidang `name` dari frontmatter agent, bukan nama file.

Untuk subagents yang dikirim oleh [plugin](/docs/id/plugins), tipe agent adalah pengenal bersifat plugin seperti `my-plugin:reviewer`, bukan nama frontmatter biasa. Titik dua menempatkan nama bersifat plugin pada jalur ekspresi reguler, jadi jangkar matcher dengan `^` dan `$` untuk kecocokan yang tepat: `^my-plugin:reviewer$`.

<h4 id="subagentstart-input">
  SubagentStart input
</h4>

Selain [bidang input umum](#common-input-fields), SubagentStart hooks menerima `agent_id` dengan pengenal unik untuk subagent dan `agent_type` dengan nama agent yang matcher filter.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "SubagentStart",
  "agent_id": "agent-abc123",
  "agent_type": "Explore"
}
```

SubagentStart hooks tidak dapat memblokir pembuatan subagent, tetapi mereka dapat menyuntikkan konteks ke subagent. Selain [bidang output JSON](#json-output) yang tersedia untuk semua hooks, Anda dapat mengembalikan:

| Bidang              | Deskripsi                                                                                                                                                  |
| :------------------ | :--------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `additionalContext` | String ditambahkan ke konteks subagent pada awal percakapannya, sebelum prompt pertamanya. Lihat [Tambahkan konteks untuk Claude](#add-context-for-claude) |

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "SubagentStart",
    "additionalContext": "Follow security guidelines for this task"
  }
}
```

<h3 id="subagentstop">
  SubagentStop
</h3>

Dijalankan ketika subagent Claude Code telah selesai merespons. Cocok pada tipe agent, nilai yang sama seperti SubagentStart.

<h4 id="subagentstop-input">
  SubagentStop input
</h4>

Selain [bidang input umum](#common-input-fields), SubagentStop hooks menerima `stop_hook_active`, `agent_id`, `agent_type`, `agent_transcript_path`, dan `last_assistant_message`. Bidang `agent_type` adalah nilai yang digunakan untuk pemfilteran matcher. `transcript_path` adalah transkrip sesi utama, sementara `agent_transcript_path` adalah transkrip subagent sendiri yang disimpan dalam folder `subagents/` bersarang. Bidang `last_assistant_message` berisi konten teks respons akhir subagent, jadi hooks dapat mengaksesnya tanpa mengurai file transkrip.

SubagentStop hooks juga menerima array `background_tasks` dan `session_crons` yang dijelaskan di bawah [Stop input](#stop-input), tersedia dalam Claude Code v2.1.145 atau lebih baru. Kedua array bersifat scoped ke sesi induk, bukan subagent.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "~/.claude/projects/.../abc123.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "SubagentStop",
  "stop_hook_active": false,
  "agent_id": "def456",
  "agent_type": "Explore",
  "agent_transcript_path": "~/.claude/projects/.../abc123/subagents/agent-def456.jsonl",
  "last_assistant_message": "Analysis complete. Found 3 potential issues...",
  "background_tasks": [],
  "session_crons": []
}
```

SubagentStop hooks menggunakan format kontrol keputusan yang sama seperti [Stop hooks](#stop-decision-control), termasuk `hookSpecificOutput.additionalContext` dengan `hookEventName` diatur ke `"SubagentStop"`, untuk umpan balik non-error yang membuat subagent tetap berjalan. Mengembalikan `decision: "block"` dengan `reason` membuat subagent tetap berjalan dan mengirimkan `reason` ke subagent sebagai instruksi berikutnya. Untuk menyuntikkan konteks ke sesi induk setelah subagent kembali, gunakan hook [`PostToolUse`](#posttooluse) pada tool `Agent` sebagai gantinya.

<h3 id="taskcreated">
  TaskCreated
</h3>

Dijalankan ketika tugas sedang dibuat melalui tool `TaskCreate`. Gunakan ini untuk menegakkan konvensi penamaan, memerlukan deskripsi tugas, atau mencegah tugas tertentu dari dibuat.

Ketika hook `TaskCreated` keluar dengan kode 2, tugas tidak dibuat dan pesan stderr diumpankan kembali ke model sebagai umpan balik. Untuk menghentikan teammate sepenuhnya alih-alih menjalankannya kembali, kembalikan JSON dengan `{"continue": false, "stopReason": "..."}`. TaskCreated hooks tidak mendukung matchers dan dijalankan pada setiap kemunculan.

<h4 id="taskcreated-input">
  TaskCreated input
</h4>

Selain [bidang input umum](#common-input-fields), TaskCreated hooks menerima `task_id`, `task_subject`, dan secara opsional `task_description`, `teammate_name`, dan `team_name`.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "TaskCreated",
  "task_id": "task-001",
  "task_subject": "Implement user authentication",
  "task_description": "Add login and signup endpoints",
  "teammate_name": "implementer",
  "team_name": "session-a1b2c3d4"
}
```

| Bidang             | Deskripsi                                                               |
| :----------------- | :---------------------------------------------------------------------- |
| `task_id`          | Pengenal tugas yang sedang dibuat                                       |
| `task_subject`     | Judul tugas                                                             |
| `task_description` | Deskripsi detail tugas. Mungkin tidak ada                               |
| `teammate_name`    | Nama teammate yang membuat tugas. Mungkin tidak ada                     |
| `team_name`        | Nama team yang diturunkan dari sesi; akan dihapus dalam rilis mendatang |

<h4 id="taskcreated-decision-control">
  TaskCreated decision control
</h4>

TaskCreated hooks mendukung dua cara untuk mengontrol pembuatan tugas:

* **Kode keluar 2**: tugas tidak dibuat dan pesan stderr diumpankan kembali ke model sebagai umpan balik.
* **JSON `{"continue": false, "stopReason": "..."}`**: menghentikan teammate sepenuhnya, mencocokkan perilaku hook `Stop`. `stopReason` ditampilkan ke pengguna.

Contoh ini memblokir tugas yang subjeknya tidak mengikuti format yang diperlukan:

```bash theme={null}
#!/bin/bash
INPUT=$(cat)
TASK_SUBJECT=$(echo "$INPUT" | jq -r '.task_subject')

if [[ ! "$TASK_SUBJECT" =~ ^\[TICKET-[0-9]+\] ]]; then
  echo "Task subject must start with a ticket number, e.g. '[TICKET-123] Add feature'" >&2
  exit 2
fi

exit 0
```

<h3 id="taskcompleted">
  TaskCompleted
</h3>

Dijalankan ketika tugas sedang ditandai sebagai selesai. Ini dijalankan dalam dua situasi: ketika agent apa pun secara eksplisit menandai tugas sebagai selesai melalui tool TaskUpdate, atau ketika [agent team](/docs/id/agent-teams) teammate menyelesaikan giliran dengan tugas yang sedang berlangsung. Gunakan ini untuk menegakkan kriteria penyelesaian seperti passing tests atau lint checks sebelum tugas dapat ditutup.

Ketika hook `TaskCompleted` keluar dengan kode 2, tugas tidak ditandai sebagai selesai dan pesan stderr diumpankan kembali ke model sebagai umpan balik. Untuk menghentikan teammate sepenuhnya alih-alih menjalankannya kembali, kembalikan JSON dengan `{"continue": false, "stopReason": "..."}`. TaskCompleted hooks tidak mendukung matchers dan dijalankan pada setiap kemunculan.

<h4 id="taskcompleted-input">
  TaskCompleted input
</h4>

Selain [bidang input umum](#common-input-fields), TaskCompleted hooks menerima `task_id`, `task_subject`, dan secara opsional `task_description`, `teammate_name`, dan `team_name`.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "TaskCompleted",
  "task_id": "task-001",
  "task_subject": "Implement user authentication",
  "task_description": "Add login and signup endpoints",
  "teammate_name": "implementer",
  "team_name": "session-a1b2c3d4"
}
```

| Bidang             | Deskripsi                                                               |
| :----------------- | :---------------------------------------------------------------------- |
| `task_id`          | Pengenal tugas yang sedang diselesaikan                                 |
| `task_subject`     | Judul tugas                                                             |
| `task_description` | Deskripsi detail tugas. Mungkin tidak ada                               |
| `teammate_name`    | Nama teammate yang menyelesaikan tugas. Mungkin tidak ada               |
| `team_name`        | Nama team yang diturunkan dari sesi; akan dihapus dalam rilis mendatang |

<h4 id="taskcompleted-decision-control">
  TaskCompleted decision control
</h4>

TaskCompleted hooks mendukung dua cara untuk mengontrol penyelesaian tugas:

* **Kode keluar 2**: tugas tidak ditandai sebagai selesai dan pesan stderr diumpankan kembali ke model sebagai umpan balik.
* **JSON `{"continue": false, "stopReason": "..."}`**: menghentikan teammate sepenuhnya, mencocokkan perilaku hook `Stop`. `stopReason` ditampilkan ke pengguna.

Contoh ini menjalankan tests dan memblokir penyelesaian tugas jika gagal:

```bash theme={null}
#!/bin/bash
INPUT=$(cat)
TASK_SUBJECT=$(echo "$INPUT" | jq -r '.task_subject')

# Jalankan test suite
if ! npm test 2>&1; then
  echo "Tests not passing. Fix failing tests before completing: $TASK_SUBJECT" >&2
  exit 2
fi

exit 0
```

<h3 id="stop">
  Stop
</h3>

Dijalankan ketika agent Claude Code utama telah selesai merespons. Tidak dijalankan jika penghentian terjadi karena interupsi pengguna. Kesalahan API menjalankan [StopFailure](#stopfailure) sebagai gantinya.

<Tip>
  Perintah [`/goal`](/docs/id/goal) adalah pintasan bawaan untuk hook Stop berbasis prompt yang bersifat sesi. Gunakan ketika Anda ingin Claude terus bekerja sampai kondisi terpenuhi tanpa menulis konfigurasi hook.
</Tip>

<h4 id="stop-input">
  Stop input
</h4>

Selain [bidang input umum](#common-input-fields), Stop hooks menerima `stop_hook_active`, `last_assistant_message`, `background_tasks`, dan `session_crons`. Bidang `stop_hook_active` adalah `true` ketika Claude Code sudah melanjutkan sebagai hasil dari stop hook. Periksa nilai ini atau proses transkrip untuk menghindari memblokir pada kondisi yang tidak akan pernah terselesaikan. Claude Code menimpa hook dan mengakhiri giliran setelah 8 blok berturut-turut.

Bidang `last_assistant_message` berisi konten teks respons akhir Claude, jadi hooks dapat mengaksesnya tanpa mengurai file transkrip.

Array `background_tasks` dan `session_crons`, tersedia dalam Claude Code v2.1.145 atau lebih baru, memungkinkan hooks membedakan "sesi selesai" dari "sesi dijeda menunggu pekerjaan latar belakang untuk membangunkannya kembali". Kedua array hadir ketika registri tugas dapat dijangkau dan kosong ketika tidak ada yang sedang berlangsung atau dijadwalkan.

Setiap entry dalam `background_tasks` menjelaskan satu tugas yang sedang berlangsung dan menggunakan bidang-bidang ini:

| Bidang        | Deskripsi                                                                                                                                                                                                                                                           |
| :------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `id`          | Pengenal tugas                                                                                                                                                                                                                                                      |
| `type`        | Label tipe tugas yang ramah seperti `shell`, `subagent`, `monitor`, `workflow`, `teammate`, `cloud session`, atau `MCP task`. Setiap label mengidentifikasi fitur Claude Code mana yang membuat tugas. Kembali ke diskriminan mentah untuk tipe yang tidak dikenali |
| `status`      | Status tugas saat ini                                                                                                                                                                                                                                               |
| `description` | Deskripsi teks bebas, dibatasi 1000 karakter dengan penanda `… [+N chars]` dalam string ketika dipotong                                                                                                                                                             |
| `command`     | Baris perintah shell, dibatasi 1000 karakter. Hadir hanya untuk tugas `shell`                                                                                                                                                                                       |
| `agent_type`  | Nama tipe subagent. Hadir hanya untuk tugas `subagent`                                                                                                                                                                                                              |
| `server`      | Nama server MCP. Hadir hanya untuk tugas `monitor` dan `MCP task`                                                                                                                                                                                                   |
| `tool`        | Nama tool MCP. Hadir hanya untuk tugas `monitor` dan `MCP task`                                                                                                                                                                                                     |
| `name`        | Nama workflow. Hadir hanya untuk tugas `workflow`                                                                                                                                                                                                                   |

Setiap entry dalam `session_crons` menjelaskan satu wakeup terjadwal yang bersifat sesi, bersumber dari `CronCreate`, `ScheduleWakeup`, dan `/loop`:

| Bidang      | Deskripsi                                                                                                                            |
| :---------- | :----------------------------------------------------------------------------------------------------------------------------------- |
| `id`        | Pengenal tugas cron                                                                                                                  |
| `schedule`  | Ekspresi cron, misalnya `0 9 * * 1-5`                                                                                                |
| `recurring` | `false` untuk wakeup satu kali yang jadwalnya mengkodekan waktu api tunggal, `true` untuk tugas yang api ulang pada setiap kecocokan |
| `prompt`    | Prompt yang dikirimkan ketika cron api, dibatasi 1000 karakter dengan penanda `… [+N chars]` yang sama                               |

Contoh ini menunjukkan input Stop dengan satu tugas shell yang sedang berlangsung dan satu cron berulang:

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "~/.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "Stop",
  "stop_hook_active": true,
  "last_assistant_message": "I've completed the refactoring. Here's a summary...",
  "background_tasks": [
    {
      "id": "task-001",
      "type": "shell",
      "status": "running",
      "description": "tail logs",
      "command": "tail -f /var/log/syslog"
    }
  ],
  "session_crons": [
    {
      "id": "cron-001",
      "schedule": "0 9 * * 1-5",
      "recurring": true,
      "prompt": "check the build"
    }
  ]
}
```

<h4 id="stop-decision-control">
  Stop decision control
</h4>

Hooks `Stop` dan `SubagentStop` dapat mengontrol apakah Claude melanjutkan. Selain [bidang output JSON](#json-output) yang tersedia untuk semua hooks, skrip hook Anda dapat mengembalikan bidang spesifik event ini:

| Bidang                                 | Deskripsi                                                                                                                                                                                                                    |
| :------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `decision`                             | `"block"` mencegah Claude berhenti. Hilangkan untuk mengizinkan Claude berhenti                                                                                                                                              |
| `reason`                               | Diperlukan saat `decision` adalah `"block"`. Memberitahu Claude mengapa itu harus melanjutkan                                                                                                                                |
| `hookSpecificOutput.additionalContext` | Umpan balik non-error untuk Claude. Percakapan berlanjut sehingga Claude dapat bertindak atas itu, tetapi tidak seperti `decision: "block"` itu ditampilkan dalam transkrip sebagai umpan balik hook daripada kesalahan hook |

```json theme={null}
{
  "decision": "block",
  "reason": "Must be provided when Claude is blocked from stopping"
}
```

Gunakan `additionalContext` ketika hook bekerja seperti yang dirancang dan memberikan panduan Claude, seperti "jalankan test suite sebelum selesai". Ini membuat percakapan terus berlanjut melalui proteksi loop yang sama seperti `decision: "block"`, yaitu input `stop_hook_active` dan batas 8-kontinuasi berturut-turut, tetapi transkrip memberi label `Stop hook feedback` dan tidak ada notifikasi kesalahan hook yang ditampilkan:

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "Stop",
    "additionalContext": "Please run the test suite before finishing"
  }
}
```

<h3 id="stopfailure">
  StopFailure
</h3>

Dijalankan alih-alih [Stop](#stop) ketika giliran berakhir karena kesalahan API. Output dan kode keluar diabaikan. Gunakan ini untuk mencatat kegagalan, mengirim alert, atau mengambil tindakan pemulihan ketika Claude tidak dapat menyelesaikan respons karena rate limits, masalah autentikasi, atau kesalahan API lainnya.

<h4 id="stopfailure-input">
  StopFailure input
</h4>

Selain [bidang input umum](#common-input-fields), StopFailure hooks menerima `error`, `error_details` opsional, dan `last_assistant_message` opsional. Bidang `error` mengidentifikasi tipe kesalahan dan digunakan untuk pemfilteran matcher.

| Bidang                   | Deskripsi                                                                                                                                                                                                                                                             |
| :----------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `error`                  | Tipe kesalahan: `rate_limit`, `overloaded`, `authentication_failed`, `oauth_org_not_allowed`, `billing_error`, `invalid_request`, `model_not_found`, `server_error`, `max_output_tokens`, atau `unknown`                                                              |
| `error_details`          | Detail tambahan tentang kesalahan, ketika tersedia                                                                                                                                                                                                                    |
| `last_assistant_message` | Teks kesalahan yang dirender ditampilkan dalam percakapan. Tidak seperti `Stop` dan `SubagentStop`, di mana bidang ini menyimpan output percakapan Claude, untuk `StopFailure` itu berisi string kesalahan API itu sendiri, seperti `"API Error: Rate limit reached"` |

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "StopFailure",
  "error": "rate_limit",
  "error_details": "429 Too Many Requests",
  "last_assistant_message": "API Error: Rate limit reached"
}
```

StopFailure hooks tidak memiliki kontrol keputusan. Mereka dijalankan untuk tujuan notifikasi dan logging saja.

<h3 id="teammateidle">
  TeammateIdle
</h3>

Dijalankan ketika [agent team](/docs/id/agent-teams) teammate akan menjadi idle setelah menyelesaikan giliran. Gunakan ini untuk menegakkan quality gates sebelum teammate berhenti bekerja, seperti memerlukan passing lint checks atau memverifikasi bahwa file output ada.

Ketika hook `TeammateIdle` keluar dengan kode 2, teammate menerima pesan stderr sebagai umpan balik dan terus bekerja alih-alih menjadi idle. Untuk menghentikan teammate sepenuhnya alih-alih menjalankannya kembali, kembalikan JSON dengan `{"continue": false, "stopReason": "..."}`. TeammateIdle hooks tidak mendukung matchers dan dijalankan pada setiap kemunculan.

<h4 id="teammateidle-input">
  TeammateIdle input
</h4>

Selain [bidang input umum](#common-input-fields), TeammateIdle hooks menerima `teammate_name` dan `team_name`.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "TeammateIdle",
  "teammate_name": "researcher",
  "team_name": "session-a1b2c3d4"
}
```

| Bidang          | Deskripsi                                                               |
| :-------------- | :---------------------------------------------------------------------- |
| `teammate_name` | Nama teammate yang akan menjadi idle                                    |
| `team_name`     | Nama team yang diturunkan dari sesi; akan dihapus dalam rilis mendatang |

<h4 id="teammateidle-decision-control">
  TeammateIdle decision control
</h4>

TeammateIdle hooks mendukung dua cara untuk mengontrol perilaku teammate:

* **Kode keluar 2**: teammate menerima pesan stderr sebagai umpan balik dan terus bekerja alih-alih menjadi idle.
* **JSON `{"continue": false, "stopReason": "..."}`**: menghentikan teammate sepenuhnya, mencocokkan perilaku hook `Stop`. `stopReason` ditampilkan ke pengguna.

Contoh ini memeriksa bahwa artefak build ada sebelum mengizinkan teammate menjadi idle:

```bash theme={null}
#!/bin/bash

if [ ! -f "./dist/output.js" ]; then
  echo "Build artifact missing. Run the build before stopping." >&2
  exit 2
fi

exit 0
```

<h3 id="configchange">
  ConfigChange
</h3>

Dijalankan ketika file konfigurasi berubah selama sesi. Gunakan ini untuk mengaudit perubahan pengaturan, menegakkan kebijakan keamanan, atau memblokir modifikasi tidak sah ke file konfigurasi.

ConfigChange hooks dijalankan untuk perubahan ke file pengaturan, pengaturan kebijakan terkelola, dan file skill. Bidang `source` dalam input memberitahu Anda tipe konfigurasi mana yang berubah, dan bidang `file_path` opsional menyediakan path ke file yang berubah.

Matcher memfilter pada sumber konfigurasi:

| Matcher            | Kapan dijalankan                           |
| :----------------- | :----------------------------------------- |
| `user_settings`    | `~/.claude/settings.json` berubah          |
| `project_settings` | `.claude/settings.json` berubah            |
| `local_settings`   | `.claude/settings.local.json` berubah      |
| `policy_settings`  | Pengaturan kebijakan terkelola berubah     |
| `skills`           | File skill dalam `.claude/skills/` berubah |

Contoh ini mencatat semua perubahan konfigurasi untuk audit keamanan:

```json theme={null}
{
  "hooks": {
    "ConfigChange": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "${CLAUDE_PROJECT_DIR}/.claude/hooks/audit-config-change.sh",
            "args": []
          }
        ]
      }
    ]
  }
}
```

<h4 id="configchange-input">
  ConfigChange input
</h4>

Selain [bidang input umum](#common-input-fields), ConfigChange hooks menerima `source` dan secara opsional `file_path`. Bidang `source` menunjukkan tipe konfigurasi mana yang berubah, dan `file_path` menyediakan path ke file spesifik yang dimodifikasi.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "ConfigChange",
  "source": "project_settings",
  "file_path": "/Users/.../my-project/.claude/settings.json"
}
```

<h4 id="configchange-decision-control">
  ConfigChange decision control
</h4>

ConfigChange hooks dapat memblokir perubahan konfigurasi dari berlaku. Gunakan kode keluar 2 atau JSON `decision` untuk mencegah perubahan. Ketika diblokir, pengaturan baru tidak diterapkan ke sesi yang berjalan.

| Bidang     | Deskripsi                                                                                  |
| :--------- | :----------------------------------------------------------------------------------------- |
| `decision` | `"block"` mencegah perubahan konfigurasi diterapkan. Hilangkan untuk mengizinkan perubahan |
| `reason`   | Penjelasan ditampilkan ke pengguna saat `decision` adalah `"block"`                        |

```json theme={null}
{
  "decision": "block",
  "reason": "Configuration changes to project settings require admin approval"
}
```

Perubahan `policy_settings` tidak dapat diblokir. Hooks masih dijalankan untuk sumber `policy_settings`, jadi Anda dapat menggunakannya untuk audit logging, tetapi keputusan blocking apa pun diabaikan. Ini memastikan pengaturan yang dikelola enterprise selalu berlaku.

<h3 id="cwdchanged">
  CwdChanged
</h3>

Dijalankan ketika direktori kerja berubah selama sesi, misalnya ketika Claude menjalankan perintah `cd`. Gunakan ini untuk bereaksi terhadap perubahan direktori: muat ulang variabel lingkungan, aktifkan toolchains khusus proyek, atau jalankan skrip setup secara otomatis. Berpasangan dengan [FileChanged](#filechanged) untuk tools seperti [direnv](https://direnv.net/) yang mengelola lingkungan per-direktori.

CwdChanged hooks memiliki akses ke `CLAUDE_ENV_FILE`. Variabel yang ditulis ke file itu bertahan ke perintah Bash berikutnya untuk sesi, sama seperti dalam [SessionStart hooks](#persist-environment-variables).

CwdChanged tidak mendukung matchers dan dijalankan pada setiap perubahan direktori.

<h4 id="cwdchanged-input">
  CwdChanged input
</h4>

Selain [bidang input umum](#common-input-fields), CwdChanged hooks menerima `old_cwd` dan `new_cwd`.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../transcript.jsonl",
  "cwd": "/Users/my-project/src",
  "hook_event_name": "CwdChanged",
  "old_cwd": "/Users/my-project",
  "new_cwd": "/Users/my-project/src"
}
```

<h4 id="cwdchanged-output">
  CwdChanged output
</h4>

Selain [bidang output JSON](#json-output) yang tersedia untuk semua hooks, CwdChanged hooks dapat mengembalikan `watchPaths` untuk secara dinamis menetapkan path file mana yang [FileChanged](#filechanged) pantau:

| Bidang       | Deskripsi                                                                                                                                                                                                         |
| :----------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `watchPaths` | Array path absolut. Menggantikan daftar watch dinamis saat ini. Path dari konfigurasi `matcher` Anda selalu dipantau. Mengembalikan array kosong menghapus daftar dinamis, yang khas saat memasuki direktori baru |

CwdChanged hooks tidak memiliki kontrol keputusan. Mereka tidak dapat memblokir perubahan direktori.

<h3 id="filechanged">
  FileChanged
</h3>

Dijalankan ketika file yang dipantau berubah di disk. Berguna untuk memuat ulang variabel lingkungan ketika file konfigurasi proyek dimodifikasi.

Bidang `matcher` untuk event ini melayani dua peran:

* **Bangun daftar watch**: nilai dibagi pada `|` dan setiap segmen terdaftar sebagai nama file literal di direktori kerja, jadi `".envrc|.env"` menonton tepat dua file itu. Pola regex tidak berguna di sini: nilai seperti `^\.env` akan menonton file yang secara harfiah bernama `^\.env`.
* **Filter hooks mana yang dijalankan**: ketika file yang dipantau berubah, nilai yang sama memfilter grup hook mana yang dijalankan menggunakan [aturan matcher](#matcher-patterns) standar terhadap basename file yang berubah.

FileChanged hooks memiliki akses ke `CLAUDE_ENV_FILE`. Variabel yang ditulis ke file itu bertahan ke perintah Bash berikutnya untuk sesi, sama seperti dalam [SessionStart hooks](#persist-environment-variables).

<h4 id="filechanged-input">
  FileChanged input
</h4>

Selain [bidang input umum](#common-input-fields), FileChanged hooks menerima `file_path` dan `event`.

| Bidang      | Deskripsi                                                                                                                          |
| :---------- | :--------------------------------------------------------------------------------------------------------------------------------- |
| `file_path` | Path absolut ke file yang berubah                                                                                                  |
| `event`     | Apa yang terjadi: `"change"` untuk file yang dimodifikasi, `"add"` untuk file yang dibuat, atau `"unlink"` untuk file yang dihapus |

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../transcript.jsonl",
  "cwd": "/Users/my-project",
  "hook_event_name": "FileChanged",
  "file_path": "/Users/my-project/.envrc",
  "event": "change"
}
```

<h4 id="filechanged-output">
  FileChanged output
</h4>

Selain [bidang output JSON](#json-output) yang tersedia untuk semua hooks, FileChanged hooks dapat mengembalikan `watchPaths` untuk secara dinamis memperbarui path file mana yang dipantau:

| Bidang       | Deskripsi                                                                                                                                                                                                                     |
| :----------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `watchPaths` | Array path absolut. Menggantikan daftar watch dinamis saat ini. Path dari konfigurasi `matcher` Anda selalu dipantau. Gunakan ini ketika skrip hook Anda menemukan file tambahan untuk dipantau berdasarkan file yang berubah |

FileChanged hooks tidak memiliki kontrol keputusan. Mereka tidak dapat memblokir perubahan file dari terjadi.

<h3 id="worktreecreate">
  WorktreeCreate
</h3>

Dijalankan ketika worktree sedang dibuat, baik dari `claude --worktree` atau dari [subagent menggunakan `isolation: "worktree"`](/docs/id/sub-agents#choose-the-subagent-scope). Secara default Claude Code membuat salinan kerja terisolasi dengan `git worktree`. Mengonfigurasi hook WorktreeCreate menggantikan perilaku git default itu, memungkinkan Anda menggunakan sistem kontrol versi berbeda seperti SVN, Perforce, atau Mercurial.

Karena hook menggantikan perilaku default sepenuhnya, [`.worktreeinclude`](/docs/id/worktrees#copy-gitignored-files-into-worktrees) tidak diproses. Jika Anda perlu menyalin file konfigurasi lokal seperti `.env` ke worktree baru, lakukan di dalam skrip hook Anda.

Hook harus mengembalikan path ke direktori worktree yang dibuat. Claude Code menggunakan path ini sebagai direktori kerja untuk sesi terisolasi. Lihat [WorktreeCreate output](#worktreecreate-output) untuk cara setiap tipe hook mengembalikan path.

Contoh ini membuat salinan kerja SVN dan mencetak path untuk Claude Code gunakan. Ganti URL repositori dengan milik Anda sendiri:

```json theme={null}
{
  "hooks": {
    "WorktreeCreate": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "bash -c 'NAME=$(jq -r .name); DIR=\"$HOME/.claude/worktrees/$NAME\"; svn checkout https://svn.example.com/repo/trunk \"$DIR\" >&2 && echo \"$DIR\"'"
          }
        ]
      }
    ]
  }
}
```

Hook membaca `name` worktree dari input JSON di stdin, melakukan checkout salinan segar ke direktori baru, dan mencetak path direktori. `echo` pada baris terakhir adalah apa yang Claude Code baca sebagai path worktree. Alihkan output lainnya ke stderr sehingga tidak mengganggu path.

<h4 id="worktreecreate-input">
  WorktreeCreate input
</h4>

Selain [bidang input umum](#common-input-fields), WorktreeCreate hooks menerima bidang `name`. Ini adalah pengenal slug untuk worktree baru, baik ditentukan oleh pengguna atau auto-generated, misalnya `bold-oak-a3f2`.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "WorktreeCreate",
  "name": "feature-auth"
}
```

<h4 id="worktreecreate-output">
  WorktreeCreate output
</h4>

WorktreeCreate hooks tidak menggunakan model keputusan allow/block standar. Sebaliknya, kesuksesan atau kegagalan hook menentukan hasil. Hook harus mengembalikan path ke direktori worktree yang dibuat:

* **Command hooks** (`type: "command"`): cetak path sebagai baris non-kosong terakhir dari stdout. Claude Code menghapus kode escape ANSI sebelum membaca baris itu, jadi banner startup shell yang dicetak sebelum `echo` Anda diabaikan. Alihkan output hook lainnya ke stderr.
* **HTTP hooks** (`type: "http"`): kembalikan `{ "hookSpecificOutput": { "hookEventName": "WorktreeCreate", "worktreePath": "/absolute/path" } }` dalam badan respons.

Jika hook gagal atau tidak menghasilkan path, pembuatan worktree gagal dengan kesalahan.

Claude Code menyelesaikan path relatif terhadap direktori tempat hook dijalankan. Jika path yang dihasilkan bukan direktori yang dapat dimasuki Claude Code, sesi mencetak kesalahan yang menamai path dan keluar dengan kode 1. Sebelum v2.1.205, path relatif atau path yang tidak ada di disk menghancurkan sesi saat startup, dan dengan `-p` itu macet selama sekitar 30 detik sebelum keluar dengan kode 0.

<h3 id="worktreeremove">
  WorktreeRemove
</h3>

Dijalankan ketika worktree sedang dihapus, baik ketika Anda keluar dari sesi `--worktree` dan memilih untuk menghapusnya, atau ketika subagent dengan `isolation: "worktree"` selesai. Ini adalah pasangan cleanup untuk [WorktreeCreate](#worktreecreate).

Untuk git-based worktrees, Claude Code menangani cleanup secara otomatis dengan `git worktree remove`. Jika Anda mengonfigurasi hook WorktreeCreate untuk sistem kontrol versi non-git, pasangkan dengan hook WorktreeRemove untuk menangani cleanup. Tanpanya, direktori worktree ditinggalkan di disk.

Claude Code meneruskan path yang dikembalikan oleh WorktreeCreate sebagai `worktree_path` dalam input hook. Contoh ini membaca path itu dan menghapus direktori:

```json theme={null}
{
  "hooks": {
    "WorktreeRemove": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "bash -c 'jq -r .worktree_path | xargs rm -rf'"
          }
        ]
      }
    ]
  }
}
```

<h4 id="worktreeremove-input">
  WorktreeRemove input
</h4>

Selain [bidang input umum](#common-input-fields), WorktreeRemove hooks menerima bidang `worktree_path`, yang merupakan path absolut ke worktree yang sedang dihapus.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "WorktreeRemove",
  "worktree_path": "/Users/.../my-project/.claude/worktrees/feature-auth"
}
```

WorktreeRemove hooks tidak memiliki kontrol keputusan. Mereka tidak dapat memblokir penghapusan worktree tetapi dapat melakukan tugas cleanup seperti menghapus status kontrol versi atau mengarsipkan perubahan. Kegagalan hook dicatat dalam mode debug saja.

<h3 id="precompact">
  PreCompact
</h3>

Dijalankan sebelum Claude Code akan menjalankan operasi compact.

Nilai matcher menunjukkan apakah compaction dipicu secara manual atau otomatis:

| Matcher  | Kapan dijalankan                         |
| :------- | :--------------------------------------- |
| `manual` | `/compact`                               |
| `auto`   | Auto-compact ketika context window penuh |

Keluar dengan kode 2 untuk memblokir compaction. Untuk manual `/compact`, pesan stderr ditampilkan ke pengguna. Anda juga dapat memblokir dengan mengembalikan JSON dengan `"decision": "block"`.

Memblokir automatic compaction memiliki efek berbeda tergantung pada kapan dijalankan. Jika compaction dipicu secara proaktif sebelum batas konteks, Claude Code melewatinya dan percakapan berlanjut tanpa compaction. Jika compaction dipicu untuk pulih dari kesalahan batas konteks yang sudah dikembalikan oleh API, kesalahan yang mendasar muncul dan permintaan saat ini gagal.

<h4 id="precompact-input">
  PreCompact input
</h4>

Selain [bidang input umum](#common-input-fields), PreCompact hooks menerima `trigger` dan `custom_instructions`. Untuk `manual`, `custom_instructions` berisi apa yang diteruskan pengguna ke `/compact`. Untuk `auto`, `custom_instructions` kosong.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "PreCompact",
  "trigger": "manual",
  "custom_instructions": ""
}
```

<h3 id="postcompact">
  PostCompact
</h3>

Dijalankan setelah Claude Code menyelesaikan operasi compact. Gunakan event ini untuk bereaksi terhadap status compacted baru, misalnya untuk mencatat ringkasan yang dihasilkan atau memperbarui status eksternal.

Nilai matcher yang sama berlaku seperti untuk `PreCompact`:

| Matcher  | Kapan dijalankan                                 |
| :------- | :----------------------------------------------- |
| `manual` | Setelah `/compact`                               |
| `auto`   | Setelah auto-compact ketika context window penuh |

<h4 id="postcompact-input">
  PostCompact input
</h4>

Selain [bidang input umum](#common-input-fields), PostCompact hooks menerima `trigger` dan `compact_summary`. Bidang `compact_summary` berisi ringkasan percakapan yang dihasilkan oleh operasi compact.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "PostCompact",
  "trigger": "manual",
  "compact_summary": "Summary of the compacted conversation..."
}
```

PostCompact hooks tidak memiliki kontrol keputusan. Mereka tidak dapat mempengaruhi hasil compaction tetapi dapat melakukan tugas follow-up.

<h3 id="sessionend">
  SessionEnd
</h3>

Dijalankan ketika sesi Claude Code berakhir. Berguna untuk tugas cleanup, logging statistik sesi, atau menyimpan status sesi. Mendukung matchers untuk memfilter berdasarkan alasan keluar.

Bidang `reason` dalam input hook menunjukkan mengapa sesi berakhir:

| Alasan                        | Deskripsi                                  |
| :---------------------------- | :----------------------------------------- |
| `clear`                       | Sesi dihapus dengan perintah `/clear`      |
| `resume`                      | Sesi beralih melalui `/resume` interaktif  |
| `logout`                      | Pengguna logout                            |
| `prompt_input_exit`           | Pengguna keluar saat input prompt terlihat |
| `bypass_permissions_disabled` | Mode bypass permissions dinonaktifkan      |
| `other`                       | Alasan keluar lainnya                      |

<h4 id="sessionend-input">
  SessionEnd input
</h4>

Selain [bidang input umum](#common-input-fields), SessionEnd hooks menerima bidang `reason` menunjukkan mengapa sesi berakhir. Lihat tabel reason di atas untuk semua nilai.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "SessionEnd",
  "reason": "other"
}
```

SessionEnd hooks tidak memiliki kontrol keputusan. Mereka tidak dapat memblokir penghentian sesi tetapi dapat melakukan tugas cleanup.

SessionEnd hooks memiliki timeout default 1.5 detik. Ini berlaku untuk keluar sesi, `/clear`, dan beralih sesi melalui `/resume` interaktif. Jika hook memerlukan lebih banyak waktu, atur per-hook `timeout` dalam konfigurasi hook. Anggaran keseluruhan secara otomatis dinaikkan ke timeout per-hook tertinggi yang dikonfigurasi dalam file pengaturan, hingga 60 detik. Timeout yang ditetapkan pada hooks yang disediakan plugin tidak menaikkan anggaran. Untuk menimpa anggaran secara eksplisit, atur variabel lingkungan `CLAUDE_CODE_SESSIONEND_HOOKS_TIMEOUT_MS` dalam milidetik.

```bash theme={null}
CLAUDE_CODE_SESSIONEND_HOOKS_TIMEOUT_MS=5000 claude
```

<h3 id="elicitation">
  Elicitation
</h3>

Dijalankan ketika server MCP meminta input pengguna mid-task. Secara default, Claude Code menampilkan dialog interaktif untuk pengguna merespons. Hooks dapat mengintersepsi permintaan ini dan merespons secara programatis, melewati dialog sepenuhnya.

Bidang matcher mencocokkan nama server MCP.

<h4 id="elicitation-input">
  Elicitation input
</h4>

Selain [bidang input umum](#common-input-fields), Elicitation hooks menerima `mcp_server_name`, `message`, dan bidang opsional `mode`, `url`, `elicitation_id`, dan `requested_schema`.

Untuk form-mode elicitation, kasus paling umum:

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "Elicitation",
  "mcp_server_name": "my-mcp-server",
  "message": "Please provide your credentials",
  "mode": "form",
  "requested_schema": {
    "type": "object",
    "properties": {
      "username": { "type": "string", "title": "Username" }
    }
  }
}
```

Untuk URL-mode elicitation, digunakan untuk autentikasi berbasis browser:

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "Elicitation",
  "mcp_server_name": "my-mcp-server",
  "message": "Please authenticate",
  "mode": "url",
  "url": "https://auth.example.com/login"
}
```

<h4 id="elicitation-output">
  Elicitation output
</h4>

Untuk merespons secara programatis tanpa menampilkan dialog, kembalikan objek JSON dengan `hookSpecificOutput`:

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "Elicitation",
    "action": "accept",
    "content": {
      "username": "alice"
    }
  }
}
```

| Bidang    | Nilai                         | Deskripsi                                                                        |
| :-------- | :---------------------------- | :------------------------------------------------------------------------------- |
| `action`  | `accept`, `decline`, `cancel` | Apakah menerima, menolak, atau membatalkan permintaan                            |
| `content` | object                        | Nilai field form untuk dikirimkan. Hanya digunakan saat `action` adalah `accept` |

Kode keluar 2 menolak elicitation dan menampilkan stderr ke pengguna.

<h3 id="elicitationresult">
  ElicitationResult
</h3>

Dijalankan setelah pengguna merespons elicitation MCP. Hooks dapat mengamati, memodifikasi, atau memblokir respons sebelum dikirim kembali ke server MCP.

Bidang matcher mencocokkan nama server MCP.

<h4 id="elicitationresult-input">
  ElicitationResult input
</h4>

Selain [bidang input umum](#common-input-fields), ElicitationResult hooks menerima `mcp_server_name`, `action`, dan bidang opsional `mode`, `elicitation_id`, dan `content`.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "ElicitationResult",
  "mcp_server_name": "my-mcp-server",
  "action": "accept",
  "content": { "username": "alice" },
  "mode": "form",
  "elicitation_id": "elicit-123"
}
```

<h4 id="elicitationresult-output">
  ElicitationResult output
</h4>

Untuk menimpa respons pengguna, kembalikan objek JSON dengan `hookSpecificOutput`:

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "ElicitationResult",
    "action": "decline",
    "content": {}
  }
}
```

| Bidang    | Nilai                         | Deskripsi                                                              |
| :-------- | :---------------------------- | :--------------------------------------------------------------------- |
| `action`  | `accept`, `decline`, `cancel` | Menimpa tindakan pengguna                                              |
| `content` | object                        | Menimpa nilai field form. Hanya bermakna saat `action` adalah `accept` |

Kode keluar 2 memblokir respons, mengubah tindakan efektif menjadi `decline`.

<h2 id="prompt-based-hooks">
  Prompt-based hooks
</h2>

Selain command, HTTP, dan MCP tool hooks, Claude Code mendukung prompt-based hooks (`type: "prompt"`) yang menggunakan LLM untuk mengevaluasi apakah akan mengizinkan atau memblokir tindakan, dan agent hooks (`type: "agent"`) yang spawn agentic verifier dengan akses tool. Tidak semua events mendukung setiap tipe hook.

Events yang mendukung semua lima tipe hook (`command`, `http`, `mcp_tool`, `prompt`, dan `agent`):

* `PermissionDenied`
* `PermissionRequest`
* `PostToolBatch`
* `PostToolUse`
* `PostToolUseFailure`
* `PreToolUse`
* `Stop`
* `SubagentStop`
* `TaskCompleted`
* `TaskCreated`
* `TeammateIdle`
* `UserPromptExpansion`
* `UserPromptSubmit`

Events yang mendukung hooks `command`, `http`, dan `mcp_tool` tetapi bukan `prompt` atau `agent`:

* `ConfigChange`
* `CwdChanged`
* `Elicitation`
* `ElicitationResult`
* `FileChanged`
* `InstructionsLoaded`
* `Notification`
* `PostCompact`
* `PreCompact`
* `SessionEnd`
* `StopFailure`
* `SubagentStart`
* `WorktreeCreate`
* `WorktreeRemove`

`SessionStart` dan `Setup` mendukung hooks `command` dan `mcp_tool`. Mereka tidak mendukung hooks `http`, `prompt`, atau `agent`.

<h3 id="how-prompt-based-hooks-work">
  Bagaimana prompt-based hooks bekerja
</h3>

Alih-alih menjalankan perintah Bash, prompt-based hooks:

1. Mengirimkan input hook dan prompt Anda ke model Claude, Haiku secara default
2. LLM merespons dengan JSON terstruktur yang berisi keputusan
3. Claude Code memproses keputusan secara otomatis

<h3 id="prompt-hook-configuration">
  Konfigurasi prompt hook
</h3>

Atur `type` ke `"prompt"` dan sediakan string `prompt` alih-alih `command`. Gunakan placeholder `$ARGUMENTS` untuk menyuntikkan data JSON input hook ke dalam teks prompt Anda. Claude Code mengirimkan prompt gabungan dan input ke model Claude cepat, yang mengembalikan keputusan JSON.

Hook `Stop` ini meminta LLM untuk mengevaluasi apakah semua tugas selesai sebelum mengizinkan Claude selesai:

```json theme={null}
{
  "hooks": {
    "Stop": [
      {
        "hooks": [
          {
            "type": "prompt",
            "prompt": "Evaluate if Claude should stop: $ARGUMENTS. Check if all tasks are complete."
          }
        ]
      }
    ]
  }
}
```

| Bidang            | Diperlukan | Deskripsi                                                                                                                                                                                                                                                                                      |
| :---------------- | :--------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`            | ya         | Harus `"prompt"`                                                                                                                                                                                                                                                                               |
| `prompt`          | ya         | Teks prompt untuk dikirim ke LLM. Gunakan `$ARGUMENTS` sebagai placeholder untuk JSON input hook. Jika `$ARGUMENTS` tidak ada, JSON input ditambahkan ke prompt                                                                                                                                |
| `model`           | tidak      | Model untuk digunakan untuk evaluasi. Default ke model cepat                                                                                                                                                                                                                                   |
| `timeout`         | tidak      | Timeout dalam detik. Default: 30                                                                                                                                                                                                                                                               |
| `continueOnBlock` | tidak      | Ketika prompt mengembalikan `ok: false`, umpankan alasan kembali ke Claude dan lanjutkan giliran alih-alih berhenti. Default: `false`. Diimplementasikan sebagai `continue: true` pada `decision: "block"` yang dihasilkan. Lihat [Response schema](#response-schema) untuk perilaku per-event |

<h3 id="response-schema">
  Skema respons
</h3>

LLM harus merespons dengan JSON yang berisi:

```json theme={null}
{
  "ok": true | false,
  "reason": "Explanation for the decision"
}
```

| Bidang   | Deskripsi                                                                                             |
| :------- | :---------------------------------------------------------------------------------------------------- |
| `ok`     | `true` untuk mengizinkan. `false` menghasilkan `decision: "block"`. Lihat perilaku per-event di bawah |
| `reason` | Diperlukan saat `ok` adalah `false`. Digunakan sebagai alasan blokir                                  |

Apa yang terjadi pada `ok: false` tergantung pada event:

* `Stop` dan `SubagentStop`: alasan diumpankan kembali ke Claude sebagai instruksi berikutnya dan giliran berlanjut
* `PreToolUse`: panggilan tool ditolak dan alasan dikembalikan ke Claude sebagai kesalahan tool, setara dengan `permissionDecision: "deny"` dari command hook
* `PostToolUse`: secara default giliran berakhir dan alasan muncul dalam chat sebagai baris peringatan. Atur `continueOnBlock: true` untuk umpankan alasan kembali ke Claude dan lanjutkan giliran alih-alih
* `PostToolBatch`, `UserPromptSubmit`, dan `UserPromptExpansion`: giliran berakhir dan alasan muncul sebagai baris peringatan. Events ini mengakhiri giliran pada `decision: "block"` terlepas dari `continue`
* `PostToolUseFailure`, `TaskCreated`, dan `TaskCompleted`: alasan dikembalikan ke Claude sebagai kesalahan tool, mirip dengan `PreToolUse`
* `TeammateIdle`: secara default rekan kerja berhenti dan alasan muncul sebagai baris peringatan. Atur `continueOnBlock: true` untuk umpankan alasan kembali ke rekan kerja dan biarkan tetap bekerja alih-alih
* `PermissionRequest`: `ok: false` tidak berpengaruh. Untuk menolak persetujuan dari hook, gunakan [command hook](#command-hook-fields) yang mengembalikan `hookSpecificOutput.decision.behavior: "deny"`
* `PermissionDenied`: `ok: false` tidak berpengaruh karena penolakan sudah terjadi. Satu-satunya output yang dibaca event ini adalah `hookSpecificOutput.retry`, yang prompt dan agent hooks tidak dapat atur — mereka berjalan pada event ini, tetapi output mereka diabaikan. Gunakan [command hook](#command-hook-fields) untuk mengembalikan `retry`

Jika Anda memerlukan kontrol yang lebih halus pada event apa pun, gunakan [command hook](#command-hook-fields) dengan bidang per-event yang dijelaskan dalam [Decision control](#decision-control).

<h3 id="check-multiple-conditions-before-stopping">
  Periksa beberapa kondisi sebelum berhenti
</h3>

Hook `Stop` ini menggunakan prompt detail untuk memeriksa tiga kondisi sebelum mengizinkan Claude berhenti. Hooks `SubagentStop` menggunakan format yang sama untuk mengevaluasi apakah [subagent](/docs/id/sub-agents) harus berhenti. Jika `"ok"` adalah `false`, Claude terus bekerja dengan alasan yang disediakan sebagai instruksi berikutnya:

```json theme={null}
{
  "hooks": {
    "Stop": [
      {
        "hooks": [
          {
            "type": "prompt",
            "prompt": "You are evaluating whether Claude should stop working. Context: $ARGUMENTS\n\nAnalyze the conversation and determine if:\n1. All user-requested tasks are complete\n2. Any errors need to be addressed\n3. Follow-up work is needed\n\nRespond with JSON: {\"ok\": true} to allow stopping, or {\"ok\": false, \"reason\": \"your explanation\"} to continue working.",
            "timeout": 30
          }
        ]
      }
    ]
  }
}
```

<h2 id="agent-based-hooks">
  Agent-based hooks
</h2>

<Warning>
  Agent hooks adalah eksperimental. Perilaku dan konfigurasi mungkin berubah di rilis mendatang. Untuk alur kerja produksi, lebih suka [command hooks](#command-hook-fields).
</Warning>

Agent-based hooks (`type: "agent"`) seperti prompt-based hooks tetapi dengan akses tool multi-turn. Alih-alih pemanggilan LLM tunggal, hook agent spawn subagent yang dapat membaca file, mencari kode, dan memeriksa codebase untuk memverifikasi kondisi. Agent hooks mendukung events yang sama seperti prompt-based hooks.

<h3 id="how-agent-hooks-work">
  Bagaimana agent hooks bekerja
</h3>

Ketika hook agent dijalankan:

1. Claude Code spawn subagent dengan prompt Anda dan JSON input hook
2. Subagent dapat menggunakan tools seperti Read, Grep, dan Glob untuk menyelidiki
3. Setelah hingga 50 turn, subagent mengembalikan keputusan terstruktur `{ "ok": true/false }`
4. Claude Code memproses keputusan dengan cara yang sama seperti prompt hook

Agent hooks berguna ketika verifikasi memerlukan memeriksa file aktual atau output test, bukan hanya mengevaluasi data input hook saja.

<h3 id="agent-hook-configuration">
  Konfigurasi agent hook
</h3>

Atur `type` ke `"agent"` dan sediakan string `prompt`. Bidang konfigurasi sama seperti [prompt hooks](#prompt-hook-configuration), dengan timeout default lebih lama:

| Bidang    | Diperlukan | Deskripsi                                                                                                |
| :-------- | :--------- | :------------------------------------------------------------------------------------------------------- |
| `type`    | ya         | Harus `"agent"`                                                                                          |
| `prompt`  | ya         | Prompt menjelaskan apa yang diverifikasi. Gunakan `$ARGUMENTS` sebagai placeholder untuk JSON input hook |
| `model`   | tidak      | Model untuk digunakan. Default ke model cepat                                                            |
| `timeout` | tidak      | Timeout dalam detik. Default: 60                                                                         |

Skema respons sama seperti prompt hooks: `{ "ok": true }` untuk mengizinkan atau `{ "ok": false, "reason": "..." }` untuk memblokir.

Hook `Stop` ini memverifikasi bahwa semua unit tests lulus sebelum mengizinkan Claude selesai:

```json theme={null}
{
  "hooks": {
    "Stop": [
      {
        "hooks": [
          {
            "type": "agent",
            "prompt": "Verify that all unit tests pass. Run the test suite and check the results. $ARGUMENTS",
            "timeout": 120
          }
        ]
      }
    ]
  }
}
```

<h2 id="run-hooks-in-the-background">
  Jalankan hooks di latar belakang
</h2>

Secara default, hooks memblokir eksekusi Claude sampai selesai. Untuk tugas yang berjalan lama seperti deployments, test suites, atau panggilan API eksternal, atur `"async": true` untuk menjalankan hook di latar belakang sementara Claude terus bekerja. Async hooks tidak dapat memblokir atau mengontrol perilaku Claude: bidang respons seperti `decision`, `permissionDecision`, dan `continue` tidak berpengaruh, karena tindakan yang akan mereka kontrol sudah selesai.

<h3 id="configure-an-async-hook">
  Konfigurasi async hook
</h3>

Tambahkan `"async": true` ke konfigurasi command hook untuk menjalankannya di latar belakang tanpa memblokir Claude. Bidang ini hanya tersedia pada hooks `type: "command"`.

Hook ini menjalankan skrip test setelah setiap pemanggilan tool `Write`. Claude terus bekerja segera sementara `run-tests.sh` dijalankan hingga 120 detik. Ketika skrip selesai, outputnya disampaikan pada turn percakapan berikutnya:

```json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Write",
        "hooks": [
          {
            "type": "command",
            "command": "/path/to/run-tests.sh",
            "async": true,
            "timeout": 120
          }
        ]
      }
    ]
  }
}
```

Bidang `timeout` menetapkan waktu maksimum dalam detik untuk proses latar belakang. Jika tidak ditentukan, async hooks menggunakan default 10 menit yang sama seperti sync hooks.

<h3 id="how-async-hooks-execute">
  Bagaimana async hooks dijalankan
</h3>

Ketika async hook dijalankan, Claude Code memulai proses hook dan segera melanjutkan tanpa menunggu selesai. Hook menerima JSON input yang sama melalui stdin seperti hook sinkron.

Setelah proses latar belakang keluar, jika hook menghasilkan respons JSON dengan bidang `additionalContext`, konten itu disampaikan ke Claude sebagai konteks pada turn percakapan berikutnya. Bidang `systemMessage` ditampilkan kepada Anda, bukan kepada Claude.

Claude Code memvalidasi respons JSON tersebut terhadap [output schema](#json-output) yang sama seperti hooks sinkron, dan menghapus bidang apa pun yang nilainya memiliki tipe yang salah, seperti `systemMessage` yang bukan string, alih-alih menyampaikannya. Jalankan dengan `--debug` untuk melihat peringatan yang menyebutkan setiap bidang yang dihapus. Sebelum v2.1.202, output JSON yang tidak terbentuk dengan baik dari async hook dapat menghancurkan sesi, dan kerusakan terulang setiap kali sesi dilanjutkan.

Notifikasi penyelesaian async hook ditekan secara default. Untuk melihatnya, aktifkan mode verbose dengan `Ctrl+O` atau mulai Claude Code dengan `--verbose`.

<h3 id="run-tests-after-file-changes">
  Jalankan tests setelah perubahan file
</h3>

Hook ini memulai test suite di latar belakang setiap kali Claude menulis file, kemudian melaporkan hasil kembali ke Claude ketika tests selesai. Simpan skrip ini ke `.claude/hooks/run-tests-async.sh` dalam proyek Anda dan buat dapat dijalankan dengan `chmod +x`:

```bash theme={null}
#!/bin/bash
# run-tests-async.sh

# Baca hook input dari stdin
INPUT=$(cat)
FILE_PATH=$(echo "$INPUT" | jq -r '.tool_input.file_path // empty')

# Hanya jalankan tests untuk file sumber
if [[ "$FILE_PATH" != *.ts && "$FILE_PATH" != *.js ]]; then
  exit 0
fi

# Jalankan tests dan laporkan hasil ke Claude via additionalContext
RESULT=$(npm test 2>&1)
EXIT_CODE=$?

if [ $EXIT_CODE -eq 0 ]; then
  MSG="Tests passed after editing $FILE_PATH"
else
  MSG="Tests failed after editing $FILE_PATH: $RESULT"
fi
jq -nc --arg msg "$MSG" '{hookSpecificOutput: {hookEventName: "PostToolUse", additionalContext: $msg}}'
```

Kemudian tambahkan konfigurasi ini ke `.claude/settings.json` dalam akar proyek Anda. Flag `async: true` memungkinkan Claude terus bekerja sementara tests dijalankan:

```json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Write|Edit",
        "hooks": [
          {
            "type": "command",
            "command": "${CLAUDE_PROJECT_DIR}/.claude/hooks/run-tests-async.sh",
            "args": [],
            "async": true,
            "timeout": 300
          }
        ]
      }
    ]
  }
}
```

<h3 id="limitations">
  Keterbatasan
</h3>

Async hooks memiliki beberapa batasan dibandingkan dengan hooks sinkron:

* Hanya hooks `type: "command"` yang mendukung `async`. Prompt-based hooks tidak dapat dijalankan secara asinkron.
* Async hooks tidak dapat memblokir pemanggilan tool atau mengembalikan keputusan. Pada saat hook selesai, tindakan pemicu sudah dilanjutkan.
* Output hook disampaikan pada turn percakapan berikutnya. Jika sesi idle, respons menunggu sampai interaksi pengguna berikutnya. Pengecualian: hook `asyncRewake` yang keluar dengan kode 2 membangunkan Claude segera bahkan ketika sesi idle.
* Setiap eksekusi membuat proses latar belakang terpisah. Tidak ada deduplikasi di seluruh beberapa penjalankan hook async yang sama.

<h2 id="security-considerations">
  Pertimbangan keamanan
</h2>

<h3 id="disclaimer">
  Penafian
</h3>

Command hooks dijalankan dengan izin pengguna sistem penuh Anda.

<Warning>
  Command hooks menjalankan perintah shell dengan izin pengguna penuh Anda. Mereka dapat memodifikasi, menghapus, atau mengakses file apa pun yang dapat diakses akun pengguna Anda. Tinjau dan uji semua perintah hook sebelum menambahkannya ke konfigurasi Anda.
</Warning>

<h3 id="security-best-practices">
  Praktik terbaik keamanan
</h3>

Ingat praktik-praktik ini saat menulis hooks:

* **Validasi dan sanitasi input**: jangan pernah mempercayai data input secara membabi buta
* **Selalu kutip variabel shell**: gunakan `"$VAR"` bukan `$VAR`
* **Blokir path traversal**: periksa `..` dalam path file
* **Gunakan path absolut**: tentukan path lengkap untuk skrip. Dalam bentuk exec, gunakan `${CLAUDE_PROJECT_DIR}` dan path tidak perlu dikutip. Dalam bentuk shell, bungkus dengan tanda kutip ganda
* **Lewati file sensitif**: hindari `.env`, `.git/`, keys, dll.

<h2 id="windows-powershell-tool">
  Windows PowerShell tool
</h2>

Di Windows, Anda dapat menjalankan hook individual dalam PowerShell dengan menetapkan `"shell": "powershell"` pada command hook. Hooks spawn PowerShell secara langsung, jadi ini bekerja terlepas dari apakah `CLAUDE_CODE_USE_POWERSHELL_TOOL` diatur. Claude Code auto-detects `pwsh.exe`, executable PowerShell 7 dan yang lebih baru, dan fallback ke `powershell.exe` untuk Windows PowerShell 5.1.

```json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Write",
        "hooks": [
          {
            "type": "command",
            "shell": "powershell",
            "command": "Write-Host 'File written'"
          }
        ]
      }
    ]
  }
}
```

Untuk mereferensikan root proyek dari perintah bentuk shell PowerShell, tulis `${CLAUDE_PROJECT_DIR}` atau `$env:CLAUDE_PROJECT_DIR`. Mulai dari v2.1.198, Claude Code menulis ulang placeholder `${CLAUDE_PROJECT_DIR}`, `${CLAUDE_PLUGIN_ROOT}`, dan `${CLAUDE_PLUGIN_DATA}` dalam perintah bentuk shell PowerShell ke bentuk `${env:NAME}` PowerShell, baik hook didefinisikan dalam `settings.json`, plugin, atau skill. PowerShell kemudian menyelesaikan nilai dari lingkungan yang diekspor setelah parsing, jadi placeholder bekerja di dalam string dengan tanda kutip ganda tetapi tidak di dalam string dengan tanda kutip tunggal, di mana PowerShell tidak pernah memperluas variabel.

Sebelum v2.1.198, penulisan ulang ini hanya berlaku untuk plugin hooks. Pada versi yang lebih awal, hook `settings.json` memerlukan bentuk `$env:` atau [exec form](#exec-form-and-shell-form), di mana `${CLAUDE_PROJECT_DIR}` diganti di setiap elemen `args` terlepas dari di mana hook didefinisikan.

Jangan tulis ejaan bare `$CLAUDE_PROJECT_DIR` dalam hook PowerShell. PowerShell menguraikannya sebagai variabel lokal yang tidak terdefinisi dan menyelesaikannya ke `$null`, yang meninggalkan jalur skrip tanpa awalan root proyeknya. Claude Code tidak menulis ulang bentuk itu; sebaliknya, ia mencatat peringatan dalam [debug log](#debug-hooks).

Contoh di bawah menunjukkan hook `settings.json` yang menjalankan skrip proyek dengan bentuk `$env:`, yang bekerja di setiap versi:

```json theme={null}
{
  "type": "command",
  "shell": "powershell",
  "command": "& \"$env:CLAUDE_PROJECT_DIR\\.claude\\hooks\\check.ps1\""
}
```

<h2 id="debug-hooks">
  Debug hooks
</h2>

Detail eksekusi hook, termasuk hook mana yang cocok, kode keluar mereka, dan stdout dan stderr lengkap, ditulis ke file debug log. Mulai Claude Code dengan `claude --debug-file <path>` untuk menulis log ke lokasi yang diketahui, atau jalankan `claude --debug` dan baca log di `~/.claude/debug/<session-id>.txt`. Flag `--debug` tidak mencetak ke terminal.

```text theme={null}
[DEBUG] Executing hooks for PostToolUse:Write
[DEBUG] Found 1 hook commands to execute
[DEBUG] Executing hook command: <Your command> with timeout 600000ms
[DEBUG] Hook command completed with status 0: <Your stdout>
```

Untuk detail pencocokan hook yang lebih granular, atur `CLAUDE_CODE_DEBUG_LOG_LEVEL=verbose` untuk melihat baris log tambahan seperti jumlah matcher hook dan pencocokan query.

Untuk troubleshooting masalah umum seperti hooks tidak dijalankan, Stop hooks yang terus memblokir, atau kesalahan konfigurasi, lihat [Limitations and troubleshooting](/docs/id/hooks-guide#limitations-and-troubleshooting) dalam panduan. Untuk panduan diagnostik yang lebih luas mencakup `/context`, `/doctor`, dan precedence pengaturan, lihat [Debug your config](/docs/id/debug-your-config).
