> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Otomatisasi tindakan dengan hooks

> Jalankan perintah shell secara otomatis ketika Claude Code mengedit file, menyelesaikan tugas, atau memerlukan input. Format kode, kirim notifikasi, validasi perintah, dan terapkan aturan proyek.

Hooks adalah perintah shell yang ditentukan pengguna yang dijalankan pada titik-titik spesifik dalam siklus hidup Claude Code. Mereka memberikan kontrol deterministik atas perilaku Claude Code, memastikan tindakan tertentu selalu terjadi daripada mengandalkan LLM untuk memilih menjalankannya. Gunakan hooks untuk menegakkan aturan proyek, mengotomatisasi tugas berulang, dan mengintegrasikan Claude Code dengan alat yang sudah ada.

Untuk keputusan yang memerlukan penilaian daripada aturan deterministik, Anda juga dapat menggunakan [prompt-based hooks](#prompt-based-hooks) atau [agent-based hooks](#agent-based-hooks) yang menggunakan model Claude untuk mengevaluasi kondisi.

Untuk cara lain memperluas Claude Code, lihat [skills](/docs/id/skills) untuk memberikan Claude instruksi tambahan dan perintah yang dapat dieksekusi, [subagents](/docs/id/sub-agents) untuk menjalankan tugas dalam konteks terisolasi, dan [plugins](/docs/id/plugins) untuk mengemas ekstensi untuk dibagikan di seluruh proyek.

<Tip>
  Panduan ini mencakup kasus penggunaan umum dan cara memulai. Untuk skema acara lengkap, format input/output JSON, dan fitur lanjutan seperti async hooks dan MCP tool hooks, lihat [Hooks reference](/docs/id/hooks).
</Tip>

<h2 id="set-up-your-first-hook">
  Siapkan hook pertama Anda
</h2>

Untuk membuat hook, tambahkan blok `hooks` ke [file pengaturan](#configure-hook-location). Panduan ini membuat hook notifikasi desktop, sehingga Anda mendapat peringatan kapan pun Claude menunggu input Anda daripada menonton terminal.

<Steps>
  <Step title="Tambahkan hook ke pengaturan Anda">
    Buka `~/.claude/settings.json` dan tambahkan hook `Notification`. Contoh di bawah menggunakan `osascript` untuk macOS; lihat [Dapatkan notifikasi ketika Claude memerlukan input](#get-notified-when-claude-needs-input) untuk perintah Linux dan Windows.

    ```json theme={null}
    {
      "hooks": {
        "Notification": [
          {
            "matcher": "",
            "hooks": [
              {
                "type": "command",
                "command": "osascript -e 'display notification \"Claude Code needs your attention\" with title \"Claude Code\"'"
              }
            ]
          }
        ]
      }
    }
    ```

    Jika file pengaturan Anda sudah memiliki kunci `hooks`, tambahkan `Notification` sebagai sibling dari kunci acara yang ada daripada mengganti seluruh objek. Setiap nama acara adalah kunci di dalam objek `hooks` tunggal:

    ```json theme={null}
    {
      "hooks": {
        "PostToolUse": [
          {
            "matcher": "Edit|Write",
            "hooks": [{ "type": "command", "command": "jq -r '.tool_input.file_path' | xargs npx prettier --write" }]
          }
        ],
        "Notification": [
          {
            "matcher": "",
            "hooks": [{ "type": "command", "command": "osascript -e 'display notification \"Claude Code needs your attention\" with title \"Claude Code\"'" }]
          }
        ]
      }
    }
    ```

    Anda juga dapat meminta Claude untuk menulis hook untuk Anda dengan mendeskripsikan apa yang Anda inginkan di CLI.
  </Step>

  <Step title="Verifikasi konfigurasi">
    Ketik `/hooks` untuk membuka browser hooks. Anda akan melihat daftar semua acara hook yang tersedia, dengan hitungan di sebelah setiap acara yang memiliki hooks yang dikonfigurasi. Pilih `Notification` untuk mengonfirmasi hook baru Anda muncul dalam daftar. Memilih hook menampilkan detailnya: acara, matcher, jenis, file sumber, dan perintah.
  </Step>

  <Step title="Uji hook">
    Tekan `Esc` untuk kembali ke CLI. Minta Claude untuk melakukan sesuatu yang memerlukan izin, kemudian beralih dari terminal. Anda harus menerima notifikasi desktop.
  </Step>
</Steps>

<Tip>
  Menu `/hooks` bersifat read-only. Untuk menambah, memodifikasi, atau menghapus hooks, edit JSON pengaturan Anda secara langsung atau minta Claude untuk membuat perubahan.
</Tip>

<h2 id="what-you-can-automate">
  Apa yang dapat Anda otomatisasi
</h2>

Hooks memungkinkan Anda menjalankan kode pada titik-titik kunci dalam siklus hidup Claude Code: format file setelah edit, blokir perintah sebelum dijalankan, kirim notifikasi ketika Claude memerlukan input, injeksi konteks saat awal sesi, dan banyak lagi. Untuk daftar lengkap acara hook, lihat [Hooks reference](/docs/id/hooks#hook-lifecycle).

Setiap contoh mencakup blok konfigurasi siap pakai yang Anda tambahkan ke [file pengaturan](#configure-hook-location).

Untuk contoh produksi hooks yang menjalankan review model terpisah dan mengirimkan temuan kembali ke sesi, lihat [bagaimana plugin `security-guidance` terintegrasi dengan Claude Code](/docs/id/security-guidance#how-the-plugin-integrates-with-claude-code).

<h3 id="get-notified-when-claude-needs-input">
  Dapatkan notifikasi ketika Claude memerlukan input
</h3>

Dapatkan notifikasi desktop kapan pun Claude selesai bekerja dan memerlukan input Anda, sehingga Anda dapat beralih ke tugas lain tanpa memeriksa terminal.

Hook ini menggunakan acara `Notification`, yang aktif ketika Claude menunggu input atau izin. Setiap tab di bawah menggunakan perintah notifikasi asli platform. Tambahkan ini ke `~/.claude/settings.json`:

<Tabs>
  <Tab title="macOS">
    ```json theme={null}
    {
      "hooks": {
        "Notification": [
          {
            "matcher": "",
            "hooks": [
              {
                "type": "command",
                "command": "osascript -e 'display notification \"Claude Code needs your attention\" with title \"Claude Code\"'"
              }
            ]
          }
        ]
      }
    }
    ```

    <Accordion title="Jika tidak ada notifikasi yang muncul">
      `osascript` merutekan notifikasi melalui aplikasi Script Editor bawaan. Jika Script Editor tidak memiliki izin notifikasi, perintah gagal diam-diam, dan macOS tidak akan meminta Anda untuk memberikannya. Jalankan ini di Terminal sekali untuk membuat Script Editor muncul di pengaturan notifikasi Anda:

      ```bash theme={null}
      osascript -e 'display notification "test"'
      ```

      Tidak ada yang akan muncul dulu. Buka **System Settings > Notifications**, temukan **Script Editor** dalam daftar, dan aktifkan **Allow Notifications**. Jalankan perintah lagi untuk mengonfirmasi notifikasi uji muncul.
    </Accordion>
  </Tab>

  <Tab title="Linux">
    ```json theme={null}
    {
      "hooks": {
        "Notification": [
          {
            "matcher": "",
            "hooks": [
              {
                "type": "command",
                "command": "notify-send 'Claude Code' 'Claude Code needs your attention'"
              }
            ]
          }
        ]
      }
    }
    ```
  </Tab>

  <Tab title="Windows (PowerShell)">
    ```json theme={null}
    {
      "hooks": {
        "Notification": [
          {
            "matcher": "",
            "hooks": [
              {
                "type": "command",
                "command": "powershell.exe -Command \"[System.Reflection.Assembly]::LoadWithPartialName('System.Windows.Forms'); [System.Windows.Forms.MessageBox]::Show('Claude Code needs your attention', 'Claude Code')\""
              }
            ]
          }
        ]
      }
    }
    ```
  </Tab>
</Tabs>

Matcher kosong `matcher` aktif pada semua jenis notifikasi. Untuk aktif hanya pada acara tertentu, atur ke salah satu nilai berikut:

| Matcher                | Aktif ketika                                                                                         |
| :--------------------- | :--------------------------------------------------------------------------------------------------- |
| `permission_prompt`    | Claude memerlukan Anda untuk menyetujui penggunaan alat                                              |
| `idle_prompt`          | Claude selesai dan menunggu prompt berikutnya Anda                                                   |
| `auth_success`         | Autentikasi selesai                                                                                  |
| `elicitation_dialog`   | Server MCP membuka formulir elicitation                                                              |
| `elicitation_complete` | Formulir elicitation MCP dikirimkan atau ditutup                                                     |
| `elicitation_response` | Respons elicitation MCP dikirim kembali ke server                                                    |
| `agent_needs_input`    | Sesi latar belakang mulai menunggu input Anda. Hanya aktif saat [agent view](/docs/id/agent-view) terbuka |
| `agent_completed`      | Sesi latar belakang selesai atau gagal. Hanya aktif saat [agent view](/docs/id/agent-view) terbuka        |

Matcher `agent_needs_input` dan `agent_completed` memerlukan Claude Code v2.1.198 atau lebih baru.

Ketik `/hooks` dan pilih `Notification` untuk mengonfirmasi hook terdaftar. Untuk skema acara lengkap, lihat [Notification reference](/docs/id/hooks#notification).

<h3 id="auto-format-code-after-edits">
  Auto-format kode setelah edit
</h3>

Jalankan [Prettier](https://prettier.io/) secara otomatis pada setiap file yang Claude edit, sehingga pemformatan tetap konsisten tanpa intervensi manual.

Hook ini menggunakan acara `PostToolUse` dengan matcher `Edit|Write`, sehingga hanya berjalan setelah alat pengeditan file. Perintah mengekstrak jalur file yang diedit dengan [`jq`](https://jqlang.github.io/jq/) dan meneruskannya ke Prettier. Tambahkan ini ke `.claude/settings.json` di root proyek Anda:

```json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Edit|Write",
        "hooks": [
          {
            "type": "command",
            "command": "jq -r '.tool_input.file_path' | xargs npx prettier --write"
          }
        ]
      }
    ]
  }
}
```

Pada Claude Code v2.1.191 atau lebih baru Anda juga dapat menulis matcher sebagai `Edit,Write`, karena `|` dan `,` adalah pemisah daftar yang dapat dipertukarkan untuk matcher nama alat pada versi tersebut.

<Note>
  Contoh Bash di halaman ini menggunakan `jq` untuk parsing JSON. Instal dengan `brew install jq` di macOS, `apt-get install jq` di Debian dan Ubuntu, atau lihat [`jq` downloads](https://jqlang.github.io/jq/download/).
</Note>

<h3 id="block-edits-to-protected-files">
  Blokir edit ke file yang dilindungi
</h3>

Cegah Claude dari memodifikasi file sensitif seperti `.env`, `package-lock.json`, atau apa pun di `.git/`. Claude menerima umpan balik yang menjelaskan mengapa edit diblokir, sehingga dapat menyesuaikan pendekatannya.

Contoh ini menggunakan file skrip terpisah yang dipanggil hook. Skrip memeriksa jalur file target terhadap daftar pola yang dilindungi dan keluar dengan kode 2 untuk memblokir edit.

<Steps>
  <Step title="Buat skrip hook">
    Simpan ini ke `.claude/hooks/protect-files.sh`:

    ```bash theme={null}
    #!/bin/bash
    # protect-files.sh

    INPUT=$(cat)
    FILE_PATH=$(echo "$INPUT" | jq -r '.tool_input.file_path // empty')

    PROTECTED_PATTERNS=(".env" "package-lock.json" ".git/")

    for pattern in "${PROTECTED_PATTERNS[@]}"; do
      if [[ "$FILE_PATH" == *"$pattern"* ]]; then
        echo "Blocked: $FILE_PATH matches protected pattern '$pattern'" >&2
        exit 2
      fi
    done

    exit 0
    ```
  </Step>

  <Step title="Buat skrip dapat dieksekusi di macOS dan Linux">
    Skrip hook harus dapat dieksekusi agar Claude Code dapat menjalankannya:

    ```bash theme={null}
    chmod +x .claude/hooks/protect-files.sh
    ```
  </Step>

  <Step title="Daftarkan hook">
    Tambahkan hook `PreToolUse` ke `.claude/settings.json` yang menjalankan skrip sebelum panggilan alat `Edit` atau `Write`:

    ```json theme={null}
    {
      "hooks": {
        "PreToolUse": [
          {
            "matcher": "Edit|Write",
            "hooks": [
              {
                "type": "command",
                "command": "\"$CLAUDE_PROJECT_DIR\"/.claude/hooks/protect-files.sh"
              }
            ]
          }
        ]
      }
    }
    ```
  </Step>
</Steps>

<h3 id="re-inject-context-after-compaction">
  Re-inject konteks setelah compaction
</h3>

Ketika jendela konteks Claude penuh, compaction merangkum percakapan untuk membebaskan ruang. Ini dapat kehilangan detail penting. Gunakan hook `SessionStart` dengan matcher `compact` untuk re-inject konteks kritis setelah setiap compaction.

Teks apa pun yang ditulis perintah Anda ke stdout ditambahkan ke konteks Claude. Contoh ini mengingatkan Claude tentang konvensi proyek dan pekerjaan terbaru. Tambahkan ini ke `.claude/settings.json` di root proyek Anda:

```json theme={null}
{
  "hooks": {
    "SessionStart": [
      {
        "matcher": "compact",
        "hooks": [
          {
            "type": "command",
            "command": "echo 'Reminder: use Bun, not npm. Run bun test before committing. Current sprint: auth refactor.'"
          }
        ]
      }
    ]
  }
}
```

Anda dapat mengganti `echo` dengan perintah apa pun yang menghasilkan output dinamis, seperti `git log --oneline -5` untuk menampilkan commit terbaru. Untuk injeksi konteks pada setiap awal sesi, pertimbangkan menggunakan [CLAUDE.md](/docs/id/memory) sebagai gantinya. Untuk variabel lingkungan, lihat [`CLAUDE_ENV_FILE`](/docs/id/hooks#persist-environment-variables) dalam referensi.

<h3 id="audit-configuration-changes">
  Audit perubahan konfigurasi
</h3>

Lacak ketika file pengaturan atau skills berubah selama sesi. Acara `ConfigChange` aktif ketika proses eksternal atau editor memodifikasi file konfigurasi, sehingga Anda dapat mencatat perubahan untuk kepatuhan atau memblokir modifikasi yang tidak sah.

Contoh ini menambahkan setiap perubahan ke log audit. Tambahkan ini ke `~/.claude/settings.json`:

```json theme={null}
{
  "hooks": {
    "ConfigChange": [
      {
        "matcher": "",
        "hooks": [
          {
            "type": "command",
            "command": "jq -c '{timestamp: now | todate, source: .source, file: .file_path}' >> ~/claude-config-audit.log"
          }
        ]
      }
    ]
  }
}
```

Matcher memfilter berdasarkan jenis konfigurasi: `user_settings`, `project_settings`, `local_settings`, `policy_settings`, atau `skills`. Untuk memblokir perubahan agar tidak berlaku, keluar dengan kode 2 atau kembalikan `{"decision": "block"}`. Lihat [ConfigChange reference](/docs/id/hooks#configchange) untuk skema input lengkap.

<h3 id="reload-environment-when-directory-or-files-change">
  Muat ulang lingkungan ketika direktori atau file berubah
</h3>

Beberapa proyek menetapkan variabel lingkungan berbeda tergantung pada direktori mana Anda berada. Alat seperti [direnv](https://direnv.net/) melakukan ini secara otomatis di shell Anda, tetapi alat Bash Claude tidak mengambil perubahan itu sendiri.

Memasangkan hook `SessionStart` dengan hook `CwdChanged` memperbaiki ini. `SessionStart` memuat variabel untuk direktori tempat Anda meluncurkan, dan `CwdChanged` memuat ulang variabel setiap kali Claude mengubah direktori. Keduanya menulis ke `CLAUDE_ENV_FILE`, yang Claude Code jalankan sebagai preamble skrip sebelum setiap perintah Bash. Tambahkan ini ke `~/.claude/settings.json`:

```json theme={null}
{
  "hooks": {
    "SessionStart": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "direnv export bash > \"$CLAUDE_ENV_FILE\""
          }
        ]
      }
    ],
    "CwdChanged": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "direnv export bash > \"$CLAUDE_ENV_FILE\""
          }
        ]
      }
    ]
  }
}
```

Jalankan `direnv allow` sekali di setiap direktori yang memiliki `.envrc` sehingga direnv diizinkan untuk memuatnya. Jika Anda menggunakan devbox atau nix sebagai gantinya direnv, pola yang sama berfungsi dengan `devbox shellenv` atau `devbox global shellenv` sebagai pengganti `direnv export bash`.

Untuk bereaksi terhadap file spesifik daripada setiap perubahan direktori, gunakan `FileChanged` dengan `matcher` yang mencantumkan nama file yang akan dipantau, dipisahkan dengan `|`. Ketika membangun daftar pantau, Claude Code membagi nilai ini menjadi nama file literal daripada mengevaluasinya sebagai regex. Lihat [FileChanged](/docs/id/hooks#filechanged) untuk cara nilai yang sama juga memfilter hook mana yang berjalan ketika file berubah. Contoh ini memantau `.envrc` dan `.env` di direktori kerja:

```json theme={null}
{
  "hooks": {
    "FileChanged": [
      {
        "matcher": ".envrc|.env",
        "hooks": [
          {
            "type": "command",
            "command": "direnv export bash > \"$CLAUDE_ENV_FILE\""
          }
        ]
      }
    ]
  }
}
```

Lihat entri referensi [CwdChanged](/docs/id/hooks#cwdchanged) dan [FileChanged](/docs/id/hooks#filechanged) untuk skema input, output `watchPaths`, dan detail `CLAUDE_ENV_FILE`.

<h3 id="auto-approve-specific-permission-prompts">
  Auto-approve prompt izin tertentu
</h3>

Lewati dialog persetujuan untuk panggilan alat yang selalu Anda izinkan. Contoh ini auto-approve `ExitPlanMode`, alat yang Claude panggil ketika selesai menyajikan rencana dan meminta untuk melanjutkan, sehingga Anda tidak diminta setiap kali rencana siap.

Tidak seperti contoh kode keluar di atas, auto-approval memerlukan hook Anda untuk menulis keputusan JSON ke stdout. Hook `PermissionRequest` aktif ketika Claude Code akan menampilkan dialog izin, dan mengembalikan `"behavior": "allow"` menjawabnya atas nama Anda.

Matcher membatasi hook ke `ExitPlanMode` saja, sehingga tidak ada prompt lain yang terpengaruh. Tambahkan ini ke `~/.claude/settings.json`:

```json theme={null}
{
  "hooks": {
    "PermissionRequest": [
      {
        "matcher": "ExitPlanMode",
        "hooks": [
          {
            "type": "command",
            "command": "echo '{\"hookSpecificOutput\": {\"hookEventName\": \"PermissionRequest\", \"decision\": {\"behavior\": \"allow\"}}}'"
          }
        ]
      }
    ]
  }
}
```

Ketika hook menyetujui, Claude Code keluar dari plan mode dan mengembalikan mode izin apa pun yang aktif sebelum Anda memasuki plan mode. Transkrip menunjukkan "Allowed by PermissionRequest hook" di mana dialog akan muncul. Jalur hook selalu menjaga percakapan saat ini: tidak dapat menghapus konteks dan memulai sesi implementasi segar seperti yang dapat dilakukan dialog.

Untuk menetapkan mode izin tertentu sebagai gantinya, output hook Anda dapat menyertakan array `updatedPermissions` dengan entri `setMode`. Nilai `mode` adalah mode izin apa pun seperti `default`, `acceptEdits`, atau `bypassPermissions`, dan `destination: "session"` menerapkannya hanya untuk sesi saat ini.

<Note>
  `bypassPermissions` hanya berlaku jika sesi diluncurkan dengan mode bypass sudah tersedia: `--dangerously-skip-permissions`, `--permission-mode bypassPermissions`, `--allow-dangerously-skip-permissions`, atau `permissions.defaultMode: "bypassPermissions"` dalam pengaturan, dan tidak dinonaktifkan oleh [`permissions.disableBypassPermissionsMode`](/docs/id/permissions#managed-settings). Ini tidak pernah disimpan sebagai `defaultMode`.
</Note>

Untuk beralih sesi ke `acceptEdits`, hook Anda menulis JSON ini ke stdout:

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PermissionRequest",
    "decision": {
      "behavior": "allow",
      "updatedPermissions": [
        { "type": "setMode", "mode": "acceptEdits", "destination": "session" }
      ]
    }
  }
}
```

Jaga matcher sesempit mungkin. Mencocokkan pada `.*` atau membiarkan matcher kosong akan auto-approve setiap prompt izin, termasuk penulisan file dan perintah shell. Lihat [PermissionRequest reference](/docs/id/hooks#permissionrequest-decision-control) untuk set lengkap bidang keputusan.

<h2 id="how-hooks-work">
  Cara kerja hooks
</h2>

Acara hook aktif pada titik-titik siklus hidup spesifik di Claude Code. Ketika acara aktif, semua hook yang cocok berjalan secara paralel, dan perintah hook yang identik secara otomatis dideduplikasi. Tabel di bawah menunjukkan setiap acara dan kapan dipicu:

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
| `PreModelSwitch`      | Before Claude Code applies a model switch that you or a client requested. Can block the switch                                                                                                                                                        |
| `PostModelSwitch`     | After the session's model changes, including changes Claude Code makes on its own, such as restoring the model when you resume a session                                                                                                              |
| `Elicitation`         | When an MCP server requests user input during a tool call                                                                                                                                                                                             |
| `ElicitationResult`   | After a user responds to an MCP elicitation, before the response is sent back to the server                                                                                                                                                           |
| `SessionEnd`          | When a session terminates                                                                                                                                                                                                                             |

Setiap hook memiliki `type` yang menentukan cara menjalankannya. Sebagian besar hooks menggunakan `"type": "command"`, yang menjalankan perintah shell. Empat jenis lain tersedia:

* `"type": "http"`: POST data acara ke URL. Lihat [HTTP hooks](#http-hooks).
* `"type": "mcp_tool"`: panggil alat pada server MCP yang sudah terhubung. Lihat [MCP tool hooks](/docs/id/hooks#mcp-tool-hook-fields).
* `"type": "prompt"`: evaluasi LLM single-turn. Lihat [Prompt-based hooks](#prompt-based-hooks).
* `"type": "agent"`: verifikasi multi-turn dengan akses alat. Agent hooks bersifat eksperimental dan mungkin berubah. Lihat [Agent-based hooks](#agent-based-hooks).

<h3 id="combine-results-from-multiple-hooks">
  Gabungkan hasil dari beberapa hooks
</h3>

Ketika beberapa hooks cocok dengan acara yang sama, setiap perintah hook berjalan hingga selesai sebelum Claude Code menggabungkan hasilnya. Satu hook yang mengembalikan `deny` tidak menghentikan hook sibling dari eksekusi. Jangan andalkan `deny` dari satu hook untuk menekan efek samping di hook lain.

Setelah semua hooks yang cocok selesai, Claude Code menggabungkan output mereka. Untuk keputusan izin `PreToolUse`, jawaban yang paling ketat menang, dalam urutan `deny`, `defer`, `ask`, `allow`. Teks dari `additionalContext` disimpan dari setiap hook dan diteruskan ke Claude bersama-sama.

Contoh di bawah mendaftarkan dua hooks `PreToolUse` pada `Bash`. Yang pertama menambahkan setiap perintah ke file log dan keluar 0. Yang kedua menjalankan skrip yang keluar 2 untuk menolak ketika perintah berisi `rm -rf`:

```json theme={null}
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "Bash",
        "hooks": [
          {
            "type": "command",
            "command": "jq -r .tool_input.command >> ~/.claude/bash.log"
          },
          {
            "type": "command",
            "command": "\"$CLAUDE_PROJECT_DIR\"/.claude/hooks/block-rm-rf.sh"
          }
        ]
      }
    ]
  }
}
```

Ketika Claude mencoba menjalankan `rm -rf /tmp/build`, kedua hooks dieksekusi secara paralel. Hook logging menulis perintah ke `~/.claude/bash.log` dan keluar 0, yang melaporkan tidak ada keputusan. Hook guardrail keluar 2, yang menolak panggilan alat. Deny menang, jadi Claude Code memblokir perintah dan menunjukkan stderr guardrail kepada Claude. Entri log masih ditulis karena hook logging sudah berjalan.

<h3 id="read-input-and-return-output">
  Baca input dan kembalikan output
</h3>

Hooks berkomunikasi dengan Claude Code melalui stdin, stdout, stderr, dan kode keluar. Ketika acara aktif, Claude Code meneruskan data spesifik acara sebagai JSON ke stdin skrip Anda. Skrip Anda membaca data itu, melakukan pekerjaan, dan memberi tahu Claude Code apa yang harus dilakukan selanjutnya melalui kode keluar.

<h4 id="hook-input">
  Hook input
</h4>

Setiap acara mencakup bidang umum seperti `session_id` dan `cwd`, tetapi setiap jenis acara menambahkan data berbeda. Misalnya, ketika Claude menjalankan perintah Bash, hook `PreToolUse` menerima sesuatu seperti ini di stdin:

```json theme={null}
{
  "session_id": "abc123",          // unique ID for this session
  "cwd": "/Users/sarah/myproject", // working directory when the event fired
  "hook_event_name": "PreToolUse", // which event triggered this hook
  "tool_name": "Bash",             // the tool Claude is about to use
  "tool_input": {                  // the arguments Claude passed to the tool
    "command": "npm test"          // for Bash, this is the shell command
  }
}
```

Skrip Anda dapat mengurai JSON itu dan bertindak atas bidang apa pun. Hook `UserPromptSubmit` mendapatkan teks `prompt` sebagai gantinya, hook `SessionStart` mendapatkan `source` dari `startup`, `resume`, `clear`, atau `compact`, dan seterusnya. Lihat [Common input fields](/docs/id/hooks#common-input-fields) dalam referensi untuk bidang bersama, dan bagian setiap acara untuk skema spesifik acara.

<h4 id="hook-output">
  Hook output
</h4>

Skrip Anda memberi tahu Claude Code apa yang harus dilakukan selanjutnya dengan menulis ke stdout atau stderr dan keluar dengan kode spesifik. Hook `PreToolUse` berikut memblokir perintah:

```bash theme={null}
#!/bin/bash
INPUT=$(cat)
COMMAND=$(echo "$INPUT" | jq -r '.tool_input.command')

if echo "$COMMAND" | grep -q "drop table"; then
  echo "Blocked: dropping tables is not allowed" >&2  # stderr becomes Claude's feedback
  exit 2 # exit 2 = block the action
fi

exit 0  # exit 0 = no decision; the normal permission flow applies
```

Kode keluar menentukan apa yang terjadi selanjutnya:

* **Exit 0**: hook melaporkan tidak ada keberatan dan tindakan berlanjut secara normal. Untuk hook `PreToolUse` ini tidak menyetujui panggilan alat: [alur izin](/docs/id/permissions) normal masih berlaku. Untuk hook `UserPromptSubmit`, `UserPromptExpansion`, dan `SessionStart`, apa pun yang Anda tulis ke stdout ditambahkan ke konteks Claude.
* **Exit 2**: tindakan diblokir. Tulis alasan ke stderr, dan Claude menerimanya sebagai umpan balik sehingga dapat menyesuaikan. Beberapa acara tidak dapat diblokir: untuk `SessionStart`, `Setup`, `Notification`, dan lainnya, exit 2 menampilkan stderr kepada pengguna dan eksekusi berlanjut. Lihat [exit code 2 behavior per event](/docs/id/hooks#exit-code-2-behavior-per-event) untuk daftar lengkap.
* **Kode keluar lainnya**: tindakan berlanjut. Transkrip menunjukkan pemberitahuan `<hook name> hook error` diikuti oleh baris pertama stderr; stderr lengkap masuk ke [debug log](/docs/id/hooks#debug-hooks).

<h4 id="structured-json-output">
  Structured JSON output
</h4>

Kode keluar hanya memberi Anda cara untuk memblokir atau tetap diam. Untuk kontrol lebih, keluar 0 dan cetak objek JSON ke stdout sebagai gantinya.

<Note>
  Gunakan exit 2 untuk memblokir dengan pesan stderr, atau exit 0 dengan JSON untuk kontrol terstruktur. Jangan campur: Claude Code mengabaikan JSON ketika Anda exit 2.
</Note>

Misalnya, hook `PreToolUse` dapat menolak panggilan alat dan memberi tahu Claude mengapa, atau meningkatkannya ke pengguna untuk persetujuan:

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PreToolUse",
    "permissionDecision": "deny",
    "permissionDecisionReason": "Use rg instead of grep for better performance"
  }
}
```

Dengan `"deny"`, Claude Code membatalkan panggilan alat dan memberi makan `permissionDecisionReason` kembali ke Claude. Nilai `permissionDecision` ini spesifik untuk `PreToolUse`:

* `"allow"`: lewati prompt izin interaktif. Aturan deny dan ask, termasuk daftar deny yang dikelola perusahaan, masih berlaku, seperti juga prompt untuk connector tools yang [organisasi Anda atur ke `ask`](/docs/id/mcp#organization-controls-on-connector-tools) dan MCP tools yang ditandai [`requiresUserInteraction`](/docs/id/mcp#require-approval-for-a-specific-tool)
* `"deny"`: batalkan panggilan alat dan kirim alasan ke Claude
* `"ask"`: tampilkan prompt izin kepada pengguna seperti biasa

Nilai keempat, `"defer"`, tersedia dalam [non-interactive mode](/docs/id/headless) dengan flag `-p`. Ini keluar dari proses dengan panggilan alat yang dipertahankan sehingga pembungkus Agent SDK dapat mengumpulkan input dan melanjutkan. Lihat [Defer a tool call for later](/docs/id/hooks#defer-a-tool-call-for-later) dalam referensi.

Mengembalikan `"allow"` melewati prompt interaktif tetapi tidak mengesampingkan [aturan izin](/docs/id/permissions#manage-permissions). Jika aturan deny cocok dengan panggilan alat, panggilan diblokir bahkan ketika hook Anda mengembalikan `"allow"`. Jika aturan ask cocok, pengguna masih diminta, dan juga connector tools yang [organisasi Anda atur ke `ask`](/docs/id/mcp#organization-controls-on-connector-tools) dan MCP tools yang ditandai [`requiresUserInteraction`](/docs/id/mcp#require-approval-for-a-specific-tool). Ini berarti aturan deny dari cakupan pengaturan apa pun, termasuk [pengaturan terkelola](/docs/id/settings#settings-files), selalu mengambil alih persetujuan hook.

Acara lain menggunakan pola keputusan berbeda. Misalnya, hook `PostToolUse` dan `Stop` menggunakan bidang `decision: "block"` tingkat atas, sementara `PermissionRequest` menggunakan `hookSpecificOutput.decision.behavior`. Lihat [summary table](/docs/id/hooks#decision-control) dalam referensi untuk rincian lengkap berdasarkan acara.

Untuk hook `UserPromptSubmit`, gunakan `hookSpecificOutput.additionalContext` sebagai gantinya untuk menyuntikkan teks ke dalam konteks Claude. Sarangkan `additionalContext` di dalam `hookSpecificOutput`; jika Anda menempatkannya di tingkat atas JSON, Claude Code diam-diam mengabaikannya. Misalnya, output ini menambahkan status cabang saat ini ke setiap prompt:

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "UserPromptSubmit",
    "additionalContext": "Current branch: release-42. Deploy freeze until Friday."
  }
}
```

Lihat [UserPromptSubmit decision control](/docs/id/hooks#userpromptsubmit-decision-control) untuk bentuk output lengkap, termasuk memblokir prompt dan menetapkan judul sesi.

Hooks dengan `type: "prompt"` menangani output secara berbeda: lihat [Prompt-based hooks](#prompt-based-hooks).

<h3 id="filter-hooks-with-matchers">
  Filter hooks dengan matchers
</h3>

Tanpa matcher, hook aktif pada setiap kemunculan acaranya. Matchers memungkinkan Anda mempersempit itu. Misalnya, jika Anda ingin menjalankan formatter hanya setelah edit file, bukan setelah setiap panggilan alat, tambahkan matcher ke hook `PostToolUse` Anda:

```json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Edit|Write",
        "hooks": [
          { "type": "command", "command": "prettier --write ..." }
        ]
      }
    ]
  }
}
```

Matcher `"Edit|Write"` aktif hanya ketika Claude menggunakan alat `Edit` atau `Write`, bukan ketika menggunakan `Bash`, `Read`, atau alat lainnya. Pada Claude Code v2.1.191 atau lebih baru, koma memisahkan alternatif dengan cara yang sama, jadi `"Edit, Write"` setara. Lihat [Matcher patterns](/docs/id/hooks#matcher-patterns) untuk cara nama biasa dan ekspresi reguler dievaluasi.

<Note>
  Claude juga dapat membuat atau memodifikasi file dengan menjalankan perintah shell melalui alat `Bash`. Jika hook Anda harus melihat setiap perubahan file, seperti untuk pemindaian kepatuhan atau pencatatan audit, tambahkan hook [`Stop`](/docs/id/hooks#stop) yang memindai pohon kerja sekali per giliran. Untuk cakupan per-panggilan sebagai gantinya, juga cocokkan `Bash` dan buat skrip Anda mencantumkan file yang dimodifikasi dan tidak dilacak dengan `git status --porcelain`.
</Note>

Setiap jenis acara cocok pada bidang spesifik:

| Acara                                                                                                                                                           | Apa yang difilter matcher                                                    | Contoh nilai matcher                                                                                                                                                                |
| :-------------------------------------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `PreToolUse`, `PostToolUse`, `PostToolUseFailure`, `PermissionRequest`, `PermissionDenied`                                                                      | nama alat                                                                    | `Bash`, `Edit\|Write`, `mcp__.*`                                                                                                                                                    |
| `SessionStart`                                                                                                                                                  | cara sesi dimulai                                                            | `startup`, `resume`, `clear`, `compact`                                                                                                                                             |
| `Setup`                                                                                                                                                         | flag CLI mana yang memicu setup                                              | `init`, `maintenance`                                                                                                                                                               |
| `SessionEnd`                                                                                                                                                    | mengapa sesi berakhir                                                        | `clear`, `resume`, `logout`, `prompt_input_exit`, `bypass_permissions_disabled`, `other`                                                                                            |
| `Notification`                                                                                                                                                  | jenis notifikasi                                                             | `permission_prompt`, `idle_prompt`, `auth_success`, `elicitation_dialog`, `elicitation_complete`, `elicitation_response`, `agent_needs_input`, `agent_completed`                    |
| `SubagentStart`                                                                                                                                                 | jenis agen                                                                   | `general-purpose`, `Explore`, `Plan`, atau nama agen khusus                                                                                                                         |
| `PreCompact`, `PostCompact`                                                                                                                                     | apa yang memicu compaction                                                   | `manual`, `auto`                                                                                                                                                                    |
| `SubagentStop`                                                                                                                                                  | jenis agen                                                                   | nilai yang sama seperti `SubagentStart`                                                                                                                                             |
| `ConfigChange`                                                                                                                                                  | sumber konfigurasi                                                           | `user_settings`, `project_settings`, `local_settings`, `policy_settings`, `skills`                                                                                                  |
| `StopFailure`                                                                                                                                                   | jenis kesalahan                                                              | `rate_limit`, `overloaded`, `authentication_failed`, `oauth_org_not_allowed`, `billing_error`, `invalid_request`, `model_not_found`, `server_error`, `max_output_tokens`, `unknown` |
| `InstructionsLoaded`                                                                                                                                            | alasan pemuatan                                                              | `session_start`, `nested_traversal`, `path_glob_match`, `include`, `compact`                                                                                                        |
| `Elicitation`                                                                                                                                                   | nama server MCP                                                              | nama server MCP yang dikonfigurasi Anda                                                                                                                                             |
| `ElicitationResult`                                                                                                                                             | nama server MCP                                                              | nilai yang sama seperti `Elicitation`                                                                                                                                               |
| `FileChanged`                                                                                                                                                   | nama file literal yang dipantau (lihat [FileChanged](/docs/id/hooks#filechanged)) | `.envrc\|.env`                                                                                                                                                                      |
| `UserPromptExpansion`                                                                                                                                           | nama perintah                                                                | nama skill atau perintah Anda                                                                                                                                                       |
| `UserPromptSubmit`, `PostToolBatch`, `Stop`, `TeammateIdle`, `TaskCreated`, `TaskCompleted`, `WorktreeCreate`, `WorktreeRemove`, `CwdChanged`, `MessageDisplay` | tidak ada dukungan matcher                                                   | selalu aktif pada setiap kemunculan                                                                                                                                                 |

Tab di bawah menunjukkan beberapa matchers lagi pada jenis acara berbeda.

<Tabs>
  <Tab title="Catat setiap perintah Bash">
    Cocokkan hanya panggilan alat `Bash` dan catat setiap perintah ke file. Acara `PostToolUse` aktif setelah perintah selesai, jadi `tool_input.command` berisi apa yang berjalan. Hook menerima data acara sebagai JSON di stdin, dan `jq -r '.tool_input.command'` mengekstrak hanya string perintah, yang `>>` tambahkan ke file log:

    ```json theme={null}
    {
      "hooks": {
        "PostToolUse": [
          {
            "matcher": "Bash",
            "hooks": [
              {
                "type": "command",
                "command": "jq -r '.tool_input.command' >> ~/.claude/command-log.txt"
              }
            ]
          }
        ]
      }
    }
    ```
  </Tab>

  <Tab title="Cocokkan alat MCP">
    Alat MCP menggunakan konvensi penamaan berbeda dari alat bawaan: `mcp__<server>__<tool>`, di mana `<server>` adalah nama server MCP dan `<tool>` adalah alat yang disediakannya. Misalnya, `mcp__github__search_repositories` atau `mcp__filesystem__read_file`. Alat dari [server MCP yang disediakan plugin](/docs/id/mcp#plugin-provided-mcp-servers) menggunakan segmen server yang diberi cakupan sebagai gantinya, seperti `mcp__plugin_my-plugin_db__query`. Gunakan matcher regex untuk menargetkan semua alat dari server spesifik, atau cocokkan di seluruh server dengan pola seperti `mcp__.*__write.*`. Lihat [Match MCP tools](/docs/id/hooks#match-mcp-tools) dalam referensi untuk daftar lengkap contoh.

    Perintah di bawah mengekstrak nama alat dari input JSON hook dengan `jq` dan menulisnya ke stderr. Menulis ke stderr menjaga stdout bersih untuk output JSON dan mengirim pesan ke [debug log](/docs/id/hooks#debug-hooks):

    ```json theme={null}
    {
      "hooks": {
        "PreToolUse": [
          {
            "matcher": "mcp__github__.*",
            "hooks": [
              {
                "type": "command",
                "command": "echo \"GitHub tool called: $(jq -r '.tool_name')\" >&2"
              }
            ]
          }
        ]
      }
    }
    ```
  </Tab>

  <Tab title="Bersihkan saat akhir sesi">
    Acara `SessionEnd` mendukung matchers pada alasan sesi berakhir. Hook ini hanya aktif pada alasan `clear`, diatur ketika Anda menjalankan `/clear`, bukan pada keluar normal:

    ```json theme={null}
    {
      "hooks": {
        "SessionEnd": [
          {
            "matcher": "clear",
            "hooks": [
              {
                "type": "command",
                "command": "rm -f /tmp/claude-scratch-*.txt"
              }
            ]
          }
        ]
      }
    }
    ```
  </Tab>
</Tabs>

Untuk sintaks matcher lengkap, lihat [Hooks reference](/docs/id/hooks#configuration).

<h4 id="filter-by-tool-name-and-arguments-with-the-if-field">
  Filter berdasarkan nama alat dan argumen dengan bidang `if`
</h4>

Bidang `if` menggunakan [sintaks aturan izin](/docs/id/permissions) untuk memfilter hooks berdasarkan nama alat dan argumen bersama-sama, sehingga proses hook hanya muncul ketika panggilan alat cocok. Ini melampaui `matcher`, yang memfilter pada tingkat grup berdasarkan nama alat saja.

Misalnya, konfigurasi ini menjalankan hook hanya ketika Claude menggunakan perintah `git` daripada semua perintah Bash:

```json theme={null}
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "Bash",
        "hooks": [
          {
            "type": "command",
            "if": "Bash(git *)",
            "command": "\"$CLAUDE_PROJECT_DIR\"/.claude/hooks/check-git-policy.sh"
          }
        ]
      }
    ]
  }
}
```

Apakah proses hook Anda berjalan tergantung pada bentuk pola `if` Anda dan perintah Bash yang Claude panggil:

| Pola `if`          | Perintah Bash          | Hook berjalan? | Mengapa                                                                                                        |
| :----------------- | :--------------------- | :------------- | :------------------------------------------------------------------------------------------------------------- |
| `Bash(git *)`      | `git push`             | ya             | nama perintah cocok                                                                                            |
| `Bash(git *)`      | `npm test && git push` | ya             | setiap subperintah diperiksa; `git push` cocok                                                                 |
| `Bash(git *)`      | `echo $(git log)`      | ya             | perintah di dalam `$()` dan backticks diperiksa; `git log` cocok                                               |
| `Bash(git *)`      | `echo $(date)`         | tidak          | tidak ada subperintah yang cocok dengan `git *`                                                                |
| `Bash(git push *)` | `echo $(date)`         | ya             | pola yang menentukan lebih dari nama perintah menjalankan hook bagaimanapun pada `$()`, backticks, atau `$VAR` |

Filter juga gagal terbuka, menjalankan hook Anda terlepas dari pola, ketika perintah Bash tidak dapat diurai. Karena filter adalah best-effort, gunakan [sistem izin](/docs/id/permissions) daripada hook untuk memberlakukan allow atau deny yang keras.

Bidang `if` menerima pola yang sama seperti aturan izin: `"Bash(git *)"`, `"Edit(*.ts)"`, dan seterusnya. Untuk mencocokkan beberapa nama alat, gunakan handler terpisah masing-masing dengan nilai `if` sendiri, atau cocokkan pada tingkat `matcher` di mana alternasi pipa didukung.

`if` hanya bekerja pada acara alat: `PreToolUse`, `PostToolUse`, `PostToolUseFailure`, `PermissionRequest`, dan `PermissionDenied`. Menambahkannya ke acara lain mencegah hook dari berjalan.

<h3 id="configure-hook-location">
  Konfigurasi lokasi hook
</h3>

Di mana Anda menambahkan hook menentukan cakupannya:

| Lokasi                                                       | Cakupan                     | Dapat Dibagikan                                 |
| :----------------------------------------------------------- | :-------------------------- | :---------------------------------------------- |
| `~/.claude/settings.json`                                    | Semua proyek Anda           | Tidak, lokal ke mesin Anda                      |
| `.claude/settings.json`                                      | Proyek tunggal              | Ya, dapat dikomit ke repo                       |
| `.claude/settings.local.json`                                | Proyek tunggal              | Tidak, gitignored ketika Claude Code membuatnya |
| Pengaturan kebijakan terkelola                               | Seluruh organisasi          | Ya, dikendalikan admin                          |
| [Plugin](/docs/id/plugins) `hooks/hooks.json`                     | Ketika plugin diaktifkan    | Ya, dikemas dengan plugin                       |
| [Skill](/docs/id/skills) atau [agent](/docs/id/sub-agents) frontmatter | Saat skill atau agent aktif | Ya, didefinisikan dalam file komponen           |

Jalankan [`/hooks`](/docs/id/hooks#the-%2Fhooks-menu) di Claude Code untuk menjelajahi semua hooks yang dikonfigurasi dikelompokkan berdasarkan acara.

Untuk menonaktifkan hooks, atur `"disableAllHooks": true` dalam file pengaturan Anda. Hooks yang dikonfigurasi dalam pengaturan terkelola masih berjalan kecuali `disableAllHooks` juga diatur di sana.

Jika Anda mengedit file pengaturan secara langsung saat Claude Code berjalan, file watcher biasanya mengambil perubahan hook secara otomatis.

<h2 id="prompt-based-hooks">
  Prompt-based hooks
</h2>

Untuk keputusan yang memerlukan penilaian daripada aturan deterministik, gunakan hook `type: "prompt"`. Daripada menjalankan perintah shell, Claude Code mengirim prompt Anda dan data input hook ke model Claude (Haiku secara default) untuk membuat keputusan. Anda dapat menentukan model berbeda dengan bidang `model` jika Anda memerlukan kemampuan lebih.

Satu-satunya pekerjaan model adalah mengembalikan keputusan ya/tidak sebagai JSON:

* `"ok": true`: tindakan berlanjut
* `"ok": false`: apa yang terjadi tergantung pada peristiwa:
  * `Stop` dan `SubagentStop`: `reason` diberi makan kembali ke Claude sehingga terus bekerja
  * `PreToolUse`: panggilan alat ditolak dan `reason` dikembalikan ke Claude sebagai kesalahan alat, sehingga dapat menyesuaikan dan melanjutkan
  * `PostToolUse`, `PostToolBatch`, `UserPromptSubmit`, dan `UserPromptExpansion`: giliran berakhir dan `reason` muncul dalam obrolan sebagai baris peringatan

Contoh ini menggunakan hook `Stop` untuk menanyakan kepada model apakah semua tugas yang diminta selesai. Jika model mengembalikan `"ok": false`, Claude terus bekerja dan menggunakan `reason` sebagai instruksi berikutnya:

```json theme={null}
{
  "hooks": {
    "Stop": [
      {
        "hooks": [
          {
            "type": "prompt",
            "prompt": "Check if all tasks are complete. If not, respond with {\"ok\": false, \"reason\": \"what remains to be done\"}."
          }
        ]
      }
    ]
  }
}
```

Untuk opsi konfigurasi lengkap, lihat [Prompt-based hooks](/docs/id/hooks#prompt-based-hooks) dalam referensi.

<h2 id="agent-based-hooks">
  Agent-based hooks
</h2>

<Warning>
  Agent hooks bersifat eksperimental. Perilaku dan konfigurasi mungkin berubah dalam rilis mendatang. Untuk alur kerja produksi, lebih suka [command hooks](/docs/id/hooks#command-hook-fields).
</Warning>

Ketika verifikasi memerlukan inspeksi file atau menjalankan perintah, gunakan hook `type: "agent"`. Tidak seperti hook prompt yang membuat panggilan LLM tunggal, hook agent menelurkan subagent yang dapat membaca file, mencari kode, dan menggunakan alat lain untuk memverifikasi kondisi sebelum mengembalikan keputusan.

Hook agent menggunakan format respons `"ok"` / `"reason"` yang sama seperti hook prompt, tetapi dengan timeout default lebih lama 60 detik dan hingga 50 putaran penggunaan alat.

Contoh ini memverifikasi bahwa tes lulus sebelum memungkinkan Claude berhenti:

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

Gunakan hook prompt ketika data input hook saja cukup untuk membuat keputusan. Gunakan hook agent ketika Anda perlu memverifikasi sesuatu terhadap keadaan aktual codebase.

Untuk opsi konfigurasi lengkap, lihat [Agent-based hooks](/docs/id/hooks#agent-based-hooks) dalam referensi.

<h2 id="http-hooks">
  HTTP hooks
</h2>

Gunakan hook `type: "http"` untuk POST data acara ke endpoint HTTP daripada menjalankan perintah shell. Endpoint menerima JSON yang sama yang diterima hook perintah di stdin, dan mengembalikan hasil melalui badan respons HTTP menggunakan format JSON yang sama.

HTTP hooks berguna ketika Anda ingin server web, fungsi cloud, atau layanan eksternal menangani logika hook: misalnya, layanan audit bersama yang mencatat acara penggunaan alat di seluruh tim.

Contoh ini memposting setiap penggunaan alat ke layanan logging lokal:

```json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "hooks": [
          {
            "type": "http",
            "url": "http://localhost:8080/hooks/tool-use",
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

Endpoint harus mengembalikan badan respons JSON menggunakan [output format](/docs/id/hooks#json-output) yang sama seperti hook perintah. Untuk memblokir panggilan alat, kembalikan respons 2xx dengan bidang `hookSpecificOutput` yang sesuai. Kode status HTTP saja tidak dapat memblokir tindakan.

Nilai header mendukung interpolasi variabel lingkungan menggunakan sintaks `$VAR_NAME` atau `${VAR_NAME}`. Hanya variabel yang tercantum dalam array `allowedEnvVars` yang diselesaikan; semua referensi `$VAR` lainnya tetap kosong.

Untuk opsi konfigurasi lengkap dan penanganan respons, lihat [HTTP hooks](/docs/id/hooks#http-hook-fields) dalam referensi.

<h2 id="limitations-and-troubleshooting">
  Keterbatasan dan troubleshooting
</h2>

<h3 id="limitations">
  Keterbatasan
</h3>

Pertimbangkan batasan-batasan ini saat merancang hooks:

* Command hooks berkomunikasi melalui stdout, stderr, dan kode keluar saja. Mereka tidak dapat memicu perintah `/` atau panggilan alat. Teks yang dikembalikan melalui `additionalContext` disuntikkan sebagai pengingat sistem yang Claude baca sebagai teks biasa. HTTP hooks berkomunikasi melalui badan respons sebagai gantinya.
* Timeout hook bervariasi menurut jenis. Timpa per hook dengan bidang `timeout` dalam detik.
  * `command`, `http`, `mcp_tool`: 10 menit. `UserPromptSubmit` menurunkan ini menjadi 30 detik, dan `MessageDisplay` menurunkan ini menjadi 10 detik.
  * `prompt`: 30 detik.
  * `agent`: 60 detik.
* Hook `PostToolUse` tidak dapat membatalkan tindakan karena alat sudah dieksekusi.
* Hook `PermissionRequest` tidak aktif dalam [mode non-interaktif](/docs/id/headless) dengan flag `-p`. Gunakan hook `PreToolUse` untuk keputusan izin otomatis.
* Hook `Stop` aktif kapan pun Claude selesai merespons, bukan hanya pada penyelesaian tugas. Mereka tidak aktif pada interupsi pengguna. Kesalahan API menjalankan [StopFailure](/docs/id/hooks#stopfailure) sebagai gantinya.
* Ketika beberapa hook `PreToolUse` mengembalikan [`updatedInput`](/docs/id/hooks#pretooluse) untuk menulis ulang argumen alat, yang terakhir selesai menang. Karena hooks berjalan secara paralel, urutannya tidak deterministik. Hindari memiliki lebih dari satu hook memodifikasi input alat yang sama.

<h3 id="hooks-and-permission-modes">
  Hooks dan mode izin
</h3>

Hook `PreToolUse` aktif sebelum pemeriksaan mode izin apa pun. Hook yang mengembalikan `permissionDecision: "deny"` memblokir alat bahkan dalam mode `bypassPermissions` atau dengan `--dangerously-skip-permissions`. Ini memungkinkan Anda menegakkan kebijakan yang pengguna tidak dapat lewati dengan mengubah mode izin mereka.

Kebalikannya tidak benar: hook yang mengembalikan `"allow"` tidak melewati aturan deny dari pengaturan, dan tidak dapat menekan prompt untuk alat konektor [organisasi Anda atur ke `ask`](/docs/id/mcp#organization-controls-on-connector-tools) atau alat MCP yang ditandai [`requiresUserInteraction`](/docs/id/mcp#require-approval-for-a-specific-tool). Hooks dapat mengetatkan pembatasan tetapi tidak melonggarkan mereka melampaui apa yang aturan izin izinkan.

<h3 id="hook-not-firing">
  Hook tidak aktif
</h3>

Hook dikonfigurasi tetapi tidak pernah dieksekusi.

* Jalankan `/hooks` dan konfirmasi hook muncul di bawah acara yang benar
* Periksa bahwa pola matcher cocok dengan nama alat dengan tepat. Matchers peka huruf besar-kecil
* Verifikasi Anda memicu jenis acara yang benar: `PreToolUse` aktif sebelum eksekusi alat, `PostToolUse` aktif setelah
* Jika menggunakan hook `PermissionRequest` dalam mode non-interaktif dengan flag `-p`, beralih ke `PreToolUse` sebagai gantinya

<h3 id="hook-error-in-output">
  Hook error dalam output
</h3>

Anda melihat pesan seperti "PreToolUse hook error: ..." dalam transkrip.

* Skrip Anda keluar dengan kode non-nol secara tidak terduga. Uji secara manual dengan menyalurkan JSON sampel:
  ```bash theme={null}
  echo '{"tool_name":"Bash","tool_input":{"command":"ls"}}' | ./my-hook.sh
  echo $?  # Check the exit code
  ```
* Jika Anda melihat "command not found", gunakan jalur absolut atau `${CLAUDE_PROJECT_DIR}` untuk mereferensikan skrip. Untuk menghindari quoting shell sepenuhnya, tambahkan `"args": []` untuk beralih ke [exec form](/docs/id/hooks#exec-form-and-shell-form), yang menelurkan skrip secara langsung tanpa shell
* Jika Anda melihat "jq: command not found", instal `jq` atau gunakan Python/Node.js untuk parsing JSON
* Jika skrip tidak berjalan sama sekali, buat dapat dieksekusi: `chmod +x ./my-hook.sh`

<h3 id="/hooks-shows-no-hooks-configured">
  `/hooks` menunjukkan tidak ada hooks yang dikonfigurasi
</h3>

Anda mengedit file pengaturan tetapi hooks tidak muncul dalam menu.

* Edit file biasanya diambil secara otomatis. Jika belum muncul setelah beberapa detik, file watcher mungkin melewatkan perubahan: mulai ulang sesi Anda untuk memaksa reload.
* Verifikasi JSON Anda valid: trailing commas dan comments tidak diizinkan
* Konfirmkan file pengaturan berada di lokasi yang benar: `.claude/settings.json` untuk hook proyek, `~/.claude/settings.json` untuk hook global

<h3 id="stop-hook-hits-the-block-cap">
  Stop hook mencapai batas blokir
</h3>

Claude terus bekerja daripada berhenti, kemudian mengakhiri giliran dengan peringatan bahwa Stop hook memblokir terlalu banyak kali berturut-turut.

Claude Code menimpa Stop hook setelah memblokir delapan kali berturut-turut tanpa kemajuan. Skrip hook Anda perlu memeriksa apakah sudah memicu kelanjutan. Parse bidang `stop_hook_active` dari input JSON dan keluar lebih awal jika `true`:

```bash theme={null}
#!/bin/bash
INPUT=$(cat)
if [ "$(echo "$INPUT" | jq -r '.stop_hook_active')" = "true" ]; then
  exit 0  # Allow Claude to stop
fi
# ... rest of your hook logic
```

Jika hook Anda secara sah memerlukan lebih dari delapan iterasi untuk konvergen, naikkan batas dengan [`CLAUDE_CODE_STOP_HOOK_BLOCK_CAP`](/docs/id/env-vars).

<h3 id="json-validation-failed">
  JSON validation failed
</h3>

Claude Code menampilkan kesalahan parsing JSON meskipun skrip hook Anda mengeluarkan JSON yang valid.

Ketika Claude Code menjalankan hook perintah bentuk shell, satu tanpa `args`, ia menelurkan `sh -c` pada macOS dan Linux atau Git Bash pada Windows secara default. Shell ini non-interaktif, tetapi Git Bash dan beberapa konfigurasi, seperti `BASH_ENV` menunjuk ke `~/.bashrc`, masih bersumber dari profil Anda. Jika profil itu berisi pernyataan `echo` tanpa syarat, output itu ditambahkan ke JSON hook Anda:

```text theme={null}
Shell ready on arm64
{"decision": "block", "reason": "Not allowed"}
```

Claude Code mencoba mengurai ini sebagai JSON dan gagal. Untuk memperbaiki ini, bungkus pernyataan echo dalam profil shell Anda sehingga hanya berjalan di shell interaktif:

```bash theme={null}
# In ~/.zshrc or ~/.bashrc
if [[ $- == *i* ]]; then
  echo "Shell ready"
fi
```

Variabel `$-` berisi flag shell, dan `i` berarti interaktif. Hooks berjalan di shell non-interaktif, jadi echo dilewati.

<h3 id="debug-techniques">
  Teknik debug
</h3>

Tampilan transkrip, diaktifkan dengan `Ctrl+O`, menunjukkan ringkasan satu baris untuk setiap hook yang aktif: kesuksesan diam-diam, kesalahan pemblokiran menampilkan stderr, dan kesalahan non-pemblokiran menampilkan pemberitahuan `<hook name> hook error` diikuti oleh baris pertama stderr.

Untuk detail eksekusi lengkap termasuk hook mana yang cocok, kode keluar mereka, stdout, dan stderr, baca debug log. Mulai Claude Code dengan `claude --debug-file /tmp/claude.log` untuk menulis ke jalur yang diketahui, kemudian `tail -f /tmp/claude.log` di terminal lain. Jika Anda memulai tanpa flag itu, jalankan `/debug` di tengah sesi untuk mengaktifkan logging dan temukan jalur log.

<h2 id="learn-more">
  Pelajari lebih lanjut
</h2>

* [Hooks reference](/docs/id/hooks): skema acara lengkap, format output JSON, async hooks, dan MCP tool hooks
* [Security considerations](/docs/id/hooks#security-considerations): tinjau sebelum menerapkan hooks dalam lingkungan bersama atau produksi
* [Bash command validator example](https://github.com/anthropics/claude-code/blob/main/examples/hooks/bash_command_validator_example.py): implementasi referensi lengkap
