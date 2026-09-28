> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Intercept dan kontrol perilaku agent dengan hooks

> Intercept dan customize perilaku agent pada titik eksekusi kunci dengan hooks

Hooks adalah fungsi callback yang menjalankan kode Anda sebagai respons terhadap peristiwa agent, seperti tool yang dipanggil, sesi dimulai, atau eksekusi berhenti. Dengan hooks, Anda dapat:

* **Blokir operasi berbahaya** sebelum dieksekusi, seperti perintah shell yang merusak atau akses file yang tidak sah
* **Log dan audit** setiap pemanggilan tool untuk kepatuhan, debugging, atau analitik
* **Transform input dan output** untuk membersihkan data, menyuntikkan kredensial, atau mengalihkan jalur file
* **Memerlukan persetujuan manusia** untuk tindakan sensitif seperti penulisan database atau panggilan API
* **Track lifecycle sesi** untuk mengelola state, membersihkan resources, atau mengirim notifikasi

<h2 id="how-hooks-work">
  Cara kerja hooks
</h2>

<Steps>
  <Step title="Sebuah event terjadi">
    Sesuatu terjadi selama eksekusi agent dan SDK menembakkan event: tool akan dipanggil (`PreToolUse`), tool mengembalikan hasil (`PostToolUse`), subagent dimulai atau berhenti, agent idle, atau eksekusi selesai. Lihat [daftar lengkap events](#available-hooks).
  </Step>

  <Step title="SDK mengumpulkan hooks terdaftar">
    SDK memeriksa hooks yang terdaftar untuk tipe event tersebut. Ini termasuk callback hooks yang Anda berikan di `options.hooks` dan shell command hooks dari file pengaturan ketika [`settingSources`](/docs/id/agent-sdk/typescript#settingsource) atau [`setting_sources`](/docs/id/agent-sdk/python#settingsource) entry yang sesuai diaktifkan, yang mana untuk opsi `query()` default.
  </Step>

  <Step title="Matchers memfilter hooks mana yang berjalan">
    Jika hook memiliki pola [`matcher`](#matchers) (seperti `"Write|Edit"`), SDK mengujinya terhadap target event (misalnya, nama tool). Hooks tanpa matcher berjalan untuk setiap event dari tipe tersebut.
  </Step>

  <Step title="Fungsi callback dieksekusi">
    Setiap [fungsi callback](#callback-functions) hook yang cocok menerima input tentang apa yang terjadi: nama tool, argumennya, ID sesi, dan detail spesifik event lainnya.
  </Step>

  <Step title="Callback Anda mengembalikan keputusan">
    Setelah melakukan operasi apa pun (logging, panggilan API, validasi), callback Anda mengembalikan [output object](#outputs) yang memberi tahu agent apa yang harus dilakukan: izinkan operasi, blokir, modifikasi input, atau suntikkan konteks ke dalam percakapan.
  </Step>
</Steps>

Contoh berikut menyatukan langkah-langkah ini. Ini mendaftarkan hook `PreToolUse` (langkah 1) dengan matcher `"Write|Edit"` (langkah 3) sehingga callback hanya terjadi untuk tools penulisan file. Ketika dipicu, callback menerima input tool (langkah 4), memeriksa apakah jalur file menargetkan file `.env`, dan mengembalikan `permissionDecision: "deny"` untuk memblokir operasi (langkah 5):

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import (
      AssistantMessage,
      ClaudeSDKClient,
      ClaudeAgentOptions,
      HookMatcher,
      ResultMessage,
  )


  # Define a hook callback that receives tool call details
  async def protect_env_files(input_data, tool_use_id, context):
      # Extract the file path from the tool's input arguments
      file_path = input_data["tool_input"].get("file_path", "")
      file_name = file_path.split("/")[-1]

      # Block the operation if targeting a .env file
      if file_name == ".env":
          return {
              "hookSpecificOutput": {
                  "hookEventName": input_data["hook_event_name"],
                  "permissionDecision": "deny",
                  "permissionDecisionReason": "Cannot modify .env files",
              }
          }

      # Return empty object to allow the operation
      return {}


  async def main():
      options = ClaudeAgentOptions(
          hooks={
              # Register the hook for PreToolUse events
              # The matcher filters to only Write and Edit tool calls
              "PreToolUse": [HookMatcher(matcher="Write|Edit", hooks=[protect_env_files])]
          }
      )

      async with ClaudeSDKClient(options=options) as client:
          await client.query("Create a .env file with the standard local development database configuration")
          async for message in client.receive_response():
              # Filter for assistant and result messages
              if isinstance(message, (AssistantMessage, ResultMessage)):
                  print(message)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query, HookCallback, PreToolUseHookInput } from "@anthropic-ai/claude-agent-sdk";

  // Define a hook callback with the HookCallback type
  const protectEnvFiles: HookCallback = async (input, toolUseID, { signal }) => {
    // Cast input to the specific hook type for type safety
    const preInput = input as PreToolUseHookInput;

    // Cast tool_input to access its properties (typed as unknown in the SDK)
    const toolInput = preInput.tool_input as Record<string, unknown>;
    const filePath = toolInput?.file_path as string;
    const fileName = filePath?.split("/").pop();

    // Block the operation if targeting a .env file
    if (fileName === ".env") {
      return {
        hookSpecificOutput: {
          hookEventName: preInput.hook_event_name,
          permissionDecision: "deny",
          permissionDecisionReason: "Cannot modify .env files"
        }
      };
    }

    // Return empty object to allow the operation
    return {};
  };

  for await (const message of query({
    prompt: "Create a .env file with the standard local development database configuration",
    options: {
      hooks: {
        // Register the hook for PreToolUse events
        // The matcher filters to only Write and Edit tool calls
        PreToolUse: [{ matcher: "Write|Edit", hooks: [protectEnvFiles] }]
      }
    }
  })) {
    // Filter for assistant and result messages
    if (message.type === "assistant" || message.type === "result") {
      console.log(message);
    }
  }
  ```
</CodeGroup>

Ketika Anda menjalankan salah satu skrip, Claude mencoba membuat file `.env`, hook menolak pemanggilan tool, dan respons akhir Claude menjelaskan bahwa ia tidak dapat membuat file `.env`.

<h2 id="available-hooks">
  Available hooks
</h2>

SDK menyediakan hooks untuk tahap berbeda dari eksekusi agent. Beberapa hooks tersedia di kedua SDK, sementara yang lain hanya TypeScript.

| Hook Event                                             | Python SDK | TypeScript SDK | Apa yang memicunya                                                                                                                                        | Contoh use case                                                                                                                                                                                 |
| ------------------------------------------------------ | ---------- | -------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `PreToolUse`                                           | Ya         | Ya             | Permintaan pemanggilan tool (dapat memblokir atau memodifikasi)                                                                                           | Blokir perintah shell berbahaya                                                                                                                                                                 |
| `PostToolUse`                                          | Ya         | Ya             | Hasil eksekusi tool                                                                                                                                       | Log semua perubahan file ke audit trail                                                                                                                                                         |
| `PostToolUseFailure`                                   | Ya         | Ya             | Kegagalan eksekusi tool                                                                                                                                   | Tangani atau log kesalahan tool                                                                                                                                                                 |
| `PostToolBatch`                                        | Tidak      | Ya             | Batch lengkap pemanggilan tool terselesaikan, sekali per batch sebelum panggilan model berikutnya                                                         | Suntikkan konvensi sekali untuk seluruh batch                                                                                                                                                   |
| `UserPromptSubmit`                                     | Ya         | Ya             | Pengajuan prompt pengguna                                                                                                                                 | Suntikkan konteks tambahan ke dalam prompts                                                                                                                                                     |
| [`UserPromptExpansion`](/docs/id/hooks#userpromptexpansion) | Tidak      | Ya             | Perintah yang diketik pengguna, atau prompt MCP, berkembang menjadi prompt sebelum mencapai Claude. Tidak aktif saat Claude menjalankan skill itu sendiri | Blokir perintah dari invokasi langsung atau tambahkan konteks saat skill diketik                                                                                                                |
| `MessageDisplay`                                       | Tidak      | Ya             | Pesan asisten dengan teks selesai, sekali per pesan dengan teks pesan lengkap                                                                             | Redaksi atau format ulang teks yang ditampilkan tanpa mengubah transcript                                                                                                                       |
| `Stop`                                                 | Ya         | Ya             | Penghentian eksekusi agent                                                                                                                                | Simpan state sesi sebelum keluar                                                                                                                                                                |
| `StopFailure`                                          | Tidak      | Ya             | Giliran berakhir dengan kesalahan API alih-alih penghentian normal                                                                                        | Log kegagalan atau kirim alert                                                                                                                                                                  |
| `SubagentStart`                                        | Ya         | Ya             | Inisialisasi subagent                                                                                                                                     | Track spawning tugas paralel                                                                                                                                                                    |
| `SubagentStop`                                         | Ya         | Ya             | Penyelesaian subagent                                                                                                                                     | Agregasi hasil dari tugas paralel                                                                                                                                                               |
| `PreCompact`                                           | Ya         | Ya             | Permintaan compaction percakapan                                                                                                                          | Arsipkan transcript lengkap sebelum merangkum                                                                                                                                                   |
| `PostCompact`                                          | Tidak      | Ya             | Compaction percakapan selesai                                                                                                                             | Log ringkasan yang dihasilkan                                                                                                                                                                   |
| [`PreModelSwitch`](/docs/id/hooks#premodelswitch)           | Tidak      | Ya             | Permintaan perubahan model, sebelum terjadi (dapat memblokir)                                                                                             | Blokir perubahan ke model tertentu                                                                                                                                                              |
| [`PostModelSwitch`](/docs/id/hooks#postmodelswitch)         | Tidak      | Ya             | Model sesi berubah, termasuk fallback otomatis                                                                                                            | Berikan panduan spesifik model Claude untuk model baru                                                                                                                                          |
| `PermissionRequest`                                    | Ya         | Ya             | Pemanggilan tool memerlukan keputusan permission                                                                                                          | Custom permission handling                                                                                                                                                                      |
| `PermissionDenied`                                     | Tidak      | Ya             | Mode otomatis menolak pemanggilan tool, termasuk penolakan tanpa verdict classifier                                                                       | Log penolakan, atau beri tahu model bahwa mungkin dapat mencoba lagi; Claude Code mengabaikan `retry: true` untuk penolakan tanpa verdict. Lihat [PermissionDenied](/docs/id/hooks#permissiondenied) |
| `SessionStart`                                         | Tidak      | Ya             | Inisialisasi sesi                                                                                                                                         | Inisialisasi logging dan telemetry                                                                                                                                                              |
| `SessionEnd`                                           | Tidak      | Ya             | Penghentian sesi                                                                                                                                          | Bersihkan resources sementara                                                                                                                                                                   |
| `Notification`                                         | Ya         | Ya             | Pesan status agent                                                                                                                                        | Kirim update status agent ke Slack atau PagerDuty                                                                                                                                               |
| `Setup`                                                | Tidak      | Ya             | Setup/maintenance sesi                                                                                                                                    | Jalankan tugas inisialisasi                                                                                                                                                                     |
| `TeammateIdle`                                         | Tidak      | Ya             | Teammate menjadi idle                                                                                                                                     | Reassign pekerjaan atau notifikasi                                                                                                                                                              |
| `TaskCreated`                                          | Tidak      | Ya             | Task dibuat melalui tool `TaskCreate`                                                                                                                     | Terapkan konvensi penamaan task                                                                                                                                                                 |
| [`TaskCompleted`](/docs/id/hooks#taskcompleted)             | Tidak      | Ya             | Task ditandai selesai                                                                                                                                     | Perlukan passing tests sebelum task ditutup                                                                                                                                                     |
| `Elicitation`                                          | Tidak      | Ya             | Server MCP meminta input pengguna di tengah-task                                                                                                          | Respons permintaan input MCP secara programatis                                                                                                                                                 |
| `ElicitationResult`                                    | Tidak      | Ya             | Pengguna merespons elicitation MCP                                                                                                                        | Modifikasi atau blokir respons sebelum kembali ke server                                                                                                                                        |
| `ConfigChange`                                         | Tidak      | Ya             | File konfigurasi berubah                                                                                                                                  | Reload pengaturan secara dinamis                                                                                                                                                                |
| `InstructionsLoaded`                                   | Tidak      | Ya             | File `CLAUDE.md` atau rules dimuat ke dalam konteks                                                                                                       | Audit file instruksi mana yang dimuat                                                                                                                                                           |
| `WorktreeCreate`                                       | Tidak      | Ya             | Git worktree dibuat                                                                                                                                       | Track isolated workspaces                                                                                                                                                                       |
| `WorktreeRemove`                                       | Tidak      | Ya             | Git worktree dihapus                                                                                                                                      | Bersihkan workspace resources                                                                                                                                                                   |
| `CwdChanged`                                           | Tidak      | Ya             | Working directory berubah selama sesi                                                                                                                     | Reload environment variables per direktori                                                                                                                                                      |
| `FileChanged`                                          | Tidak      | Ya             | File yang diawasi dimodifikasi, dibuat, atau dihapus                                                                                                      | Reload konfigurasi saat file proyek berubah                                                                                                                                                     |
| `DirectoryAdded`                                       | Tidak      | Ya             | Working directory ditambahkan selama sesi                                                                                                                 | Instal dependencies untuk repository yang ditambahkan di tengah-sesi                                                                                                                            |

<h2 id="configure-hooks">
  Konfigurasi hooks
</h2>

Untuk mengonfigurasi hook, berikan di field `hooks` dari opsi agent Anda (`ClaudeAgentOptions` di Python, object `options` di TypeScript). Snippet ini mengasumsikan Anda telah mendefinisikan callback hook, seperti `protect_env_files` di Python atau `protectEnvFiles` di TypeScript dari contoh di atas:

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(
      hooks={"PreToolUse": [HookMatcher(matcher="Bash", hooks=[my_callback])]}
  )

  async with ClaudeSDKClient(options=options) as client:
      await client.query("Your prompt")
      async for message in client.receive_response():
          print(message)
  ```

  ```typescript TypeScript theme={null}
  for await (const message of query({
    prompt: "Your prompt",
    options: {
      hooks: {
        PreToolUse: [{ matcher: "Bash", hooks: [myCallback] }]
      }
    }
  })) {
    console.log(message);
  }
  ```
</CodeGroup>

Opsi `hooks` adalah dictionary di Python atau object di TypeScript, di mana:

* **Keys**: [nama hook event](#available-hooks) seperti `'PreToolUse'`, `'PostToolUse'`, dan `'Stop'`
* **Values**: array dari [matchers](#matchers), masing-masing berisi pola filter opsional dan [fungsi callback](#callback-functions) Anda

<h3 id="matchers">
  Matchers
</h3>

Gunakan matchers untuk memfilter kapan callbacks Anda terjadi. Field `matcher` cocok dengan nilai berbeda tergantung pada tipe hook event. Misalnya, tool-based hooks cocok dengan nama tool, sementara hooks `Notification` cocok dengan tipe notifikasi.

SDK matchers mengikuti aturan yang sama dengan [matchers dalam file settings](/docs/id/hooks#matcher-patterns). Bagian tersebut mendokumentasikan jalur evaluasi string yang tepat dan regular expression, persyaratan versi mereka, dan nilai matcher untuk setiap tipe event.

| Option    | Type             | Default     | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| --------- | ---------------- | ----------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `matcher` | `string`         | `undefined` | Pola yang cocok dengan field filter event, mengikuti [aturan untuk matchers dalam file settings](/docs/id/hooks#matcher-patterns). Untuk tool hooks, ini adalah nama tool. Built-in tools termasuk `Bash`, `Read`, `Write`, `Edit`, `Glob`, `Grep`, `WebFetch`, `Agent`, dan lainnya (lihat [Tool Input Types](/docs/id/agent-sdk/typescript#tool-input-types) untuk daftar lengkap). MCP tools menggunakan pola `mcp__<server>__<action>`, di mana `<server>` adalah key yang Anda gunakan dalam konfigurasi `mcpServers`. |
| `hooks`   | `HookCallback[]` | -           | Diperlukan. Array dari fungsi callback untuk dieksekusi ketika pola cocok                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `timeout` | `number`         | `undefined` | Timeout dalam detik. Ketika dihilangkan, Claude Code menerapkan [timeout default event](#hook-timeout). SDK callbacks Anda mengikuti default hook `command`                                                                                                                                                                                                                                                                                                                                                       |

Gunakan pola `matcher` untuk menargetkan tools spesifik kapan pun memungkinkan. Matcher dengan `'Bash'` hanya berjalan untuk perintah Bash, sementara menghilangkan pola menjalankan callbacks Anda untuk setiap kemunculan event. Hilangkan dengan sengaja untuk log setiap pemanggilan tool yang dibuat sesi Anda.

<h3 id="callback-functions">
  Callback functions
</h3>

<h4 id="inputs">
  Inputs
</h4>

Setiap hook callback menerima tiga argumen:

* **Input data:** object yang diketik berisi detail event. Setiap tipe hook memiliki bentuk input sendiri. Misalnya, `PreToolUseHookInput` mencakup `tool_name` dan `tool_input`, sementara `NotificationHookInput` mencakup `message`. Lihat definisi tipe lengkap di referensi SDK [TypeScript](/docs/id/agent-sdk/typescript#hookinput) dan [Python](/docs/id/agent-sdk/python#hookinput).
  * Semua hook inputs berbagi `session_id`, `cwd`, dan `hook_event_name`.
  * `agent_id` dan `agent_type` diisi ketika hook terjadi di dalam subagent. Di TypeScript, ini berada di base hook input dan tersedia untuk semua tipe hook. Di Python, ini adalah field opsional pada `PreToolUse`, `PostToolUse`, `PostToolUseFailure`, dan `PermissionRequest`, dan field yang diperlukan pada `SubagentStart` dan `SubagentStop`.
* **Tool use ID** (`str | None` / `string | undefined`): mengkorelasikan events `PreToolUse` dan `PostToolUse` untuk pemanggilan tool yang sama.
* **Context:** di TypeScript, berisi property `signal` (`AbortSignal`) untuk pembatalan. Di Python, argumen ini dicadangkan untuk penggunaan di masa depan.

<h4 id="outputs">
  Outputs
</h4>

Callback Anda mengembalikan object dengan dua kategori fields:

* **Top-level fields** diterima pada setiap event: `systemMessage` menampilkan pesan kepada pengguna, dan `continue` (`continue_` di Python) menentukan apakah agent terus berjalan setelah hook ini. Beberapa events membuangnya atau mengirimnya ke tempat lain. Setiap [bagian event](/docs/id/hooks#hook-events) di halaman hooks mengatakan di mana mereka mendarat.
* **`hookSpecificOutput`** mengontrol operasi saat ini. Fields yang Anda tetapkan di dalamnya tergantung pada tipe hook event:
  * Untuk hooks `PreToolUse`, di sinilah Anda menetapkan `permissionDecision` (`"allow"`, `"deny"`, `"ask"`, atau `"defer"`), `permissionDecisionReason`, dan `updatedInput`. Jika Anda mengembalikan `"defer"`, query berakhir sehingga Anda dapat [melanjutkannya nanti](/docs/id/hooks#defer-a-tool-call-for-later).
  * Untuk hooks `PostToolUse`, Anda dapat menetapkan `additionalContext` untuk menambahkan informasi ke hasil tool. Untuk mengganti output tool sebelum Claude melihatnya, tetapkan `updatedToolOutput`, yang bekerja untuk tool apa pun di kedua SDK. Field `updatedMCPToolOutput` yang lebih lama mengganti output MCP tool saja dan sudah usang.
  * Di TypeScript SDK, callback `PostToolUse` juga dapat mengembalikan `classifierContext`, catatan singkat tentang hasil pemanggilan tool untuk [auto mode](/docs/id/permission-modes#eliminate-prompts-with-auto-mode) permission classifier. Karena callback Anda berjalan dalam proses aplikasi Anda sendiri, classifier dapat menimbang pernyataan pengguna yang Anda sampaikan dalam catatan sebagai intent pengguna. Field memerlukan TypeScript Agent SDK v0.3.236 atau lebih baru. [Annotate a result for the auto mode classifier](/docs/id/hooks#annotate-a-result-for-the-auto-mode-classifier) mencakup batas panjang, aturan synchronous-only, dan apa yang tidak boleh dimasukkan dalam catatan.

Kembalikan `{}` untuk mengizinkan operasi tanpa perubahan. SDK callback hooks menggunakan format output JSON yang sama dengan [Claude Code shell command hooks](/docs/id/hooks#json-output), yang mendokumentasikan setiap field dan opsi spesifik event. Untuk definisi tipe SDK, lihat referensi SDK [TypeScript](/docs/id/agent-sdk/typescript#synchookjsonoutput) dan [Python](/docs/id/agent-sdk/python#synchookjsonoutput).

<Note>
  Ketika multiple hooks atau permission rules berlaku, `deny` mengambil prioritas atas `defer`, yang mengambil prioritas atas `ask`, yang mengambil prioritas atas `allow`. Jika hook apa pun mengembalikan `deny`, operasi diblokir terlepas dari hooks lainnya.
</Note>

<h4 id="asynchronous-output">
  Asynchronous output
</h4>

Secara default, agent menunggu hook Anda kembali sebelum melanjutkan. Jika hook Anda melakukan side effect, seperti logging atau mengirim webhook, dan tidak perlu mempengaruhi perilaku agent, Anda dapat mengembalikan async output sebagai gantinya. Ini memberi tahu agent untuk melanjutkan segera tanpa menunggu hook selesai. Dalam snippet ini, `send_to_logging_service` di Python dan `sendToLoggingService` di TypeScript berdiri untuk fungsi logging apa pun yang Anda definisikan:

<CodeGroup>
  ```python Python theme={null}
  async def async_hook(input_data, tool_use_id, context):
      # Start a background task, then return immediately
      asyncio.create_task(send_to_logging_service(input_data))
      return {"async_": True, "asyncTimeout": 30000}
  ```

  ```typescript TypeScript theme={null}
  const asyncHook: HookCallback = async (input, toolUseID, { signal }) => {
    // Start a background task, then return immediately
    sendToLoggingService(input).catch(console.error);
    return { async: true, asyncTimeout: 30000 };
  };
  ```
</CodeGroup>

| Field          | Type     | Description                                                                                                          |
| -------------- | -------- | -------------------------------------------------------------------------------------------------------------------- |
| `async`        | `true`   | Sinyal mode async. Agent melanjutkan tanpa menunggu. Di Python, gunakan `async_` untuk menghindari reserved keyword. |
| `asyncTimeout` | `number` | Optional timeout dalam milliseconds untuk operasi background                                                         |

<Note>
  Async outputs tidak dapat memblokir, memodifikasi, atau menyuntikkan konteks ke dalam operasi karena agent telah melanjutkan. Gunakan hanya untuk side effects seperti logging, metrics, atau notifikasi.
</Note>

<h2 id="examples">
  Contoh
</h2>

Beberapa contoh di bagian ini menunjukkan hanya fungsi callback. Untuk menjalankan satu, daftarkan callback di bawah event yang cocok di field `hooks` dari opsi Anda, seperti yang ditunjukkan di [Konfigurasi hooks](#configure-hooks).

<h3 id="modify-tool-input">
  Modifikasi input tool
</h3>

Contoh ini mengintersepsi pemanggilan tool Write dan menulis ulang argumen `file_path` untuk menambahkan `/sandbox`, mengalihkan semua penulisan file ke direktori sandboxed. Callback mengembalikan `updatedInput` dengan jalur yang dimodifikasi dan `permissionDecision: 'allow'` untuk auto-approve operasi yang ditulis ulang:

<CodeGroup>
  ```python Python theme={null}
  async def redirect_to_sandbox(input_data, tool_use_id, context):
      if input_data["hook_event_name"] != "PreToolUse":
          return {}

      if input_data["tool_name"] == "Write":
          original_path = input_data["tool_input"].get("file_path", "")
          return {
              "hookSpecificOutput": {
                  "hookEventName": input_data["hook_event_name"],
                  "permissionDecision": "allow",
                  "updatedInput": {
                      **input_data["tool_input"],
                      "file_path": f"/sandbox{original_path}",
                  },
              }
          }
      return {}
  ```

  ```typescript TypeScript theme={null}
  const redirectToSandbox: HookCallback = async (input, toolUseID, { signal }) => {
    if (input.hook_event_name !== "PreToolUse") return {};

    const preInput = input as PreToolUseHookInput;
    const toolInput = preInput.tool_input as Record<string, unknown>;
    if (preInput.tool_name === "Write") {
      const originalPath = toolInput.file_path as string;
      return {
        hookSpecificOutput: {
          hookEventName: preInput.hook_event_name,
          permissionDecision: "allow",
          updatedInput: {
            ...toolInput,
            file_path: `/sandbox${originalPath}`
          }
        }
      };
    }
    return {};
  };
  ```
</CodeGroup>

<Note>
  Pasangkan `updatedInput` dengan `permissionDecision: 'allow'` untuk auto-approve input yang dimodifikasi, atau `permissionDecision: 'ask'` untuk menampilkannya kepada pengguna. Jika Anda menghilangkan `permissionDecision`, input yang dimodifikasi masih berlaku dan mengalir melalui evaluasi permission normal. Dengan `'defer'`, `updatedInput` diabaikan. Selalu kembalikan object baru daripada mutating `tool_input` asli.
</Note>

Untuk mengonfirmasi pengalihan, atur prefix ke path yang dapat Anda tulis, seperti `./sandbox` atau `/tmp/sandbox` (macOS tidak memungkinkan pembuatan direktori root-level `/sandbox`), kemudian minta agent untuk menulis file: hasil tool Write di message stream menamai path dengan prefix sandbox Anda daripada yang diminta Claude.

<h3 id="add-context-and-block-a-tool">
  Tambahkan konteks dan blokir tool
</h3>

Contoh ini memblokir penulisan ke direktori `/etc` dan menjelaskan alasannya kepada model dan pengguna:

* `permissionDecision: 'deny'` menghentikan pemanggilan tool.
* `permissionDecisionReason` memberitahu model mengapa, sehingga menghindari percobaan ulang.
* `systemMessage` menunjukkan kepada pengguna apa yang terjadi.

<CodeGroup>
  ```python Python theme={null}
  async def block_etc_writes(input_data, tool_use_id, context):
      file_path = input_data["tool_input"].get("file_path", "")

      if file_path.startswith("/etc"):
          return {
              # Top-level field: message shown to the user
              "systemMessage": "Remember: system directories like /etc are protected.",
              # hookSpecificOutput: block the operation
              "hookSpecificOutput": {
                  "hookEventName": input_data["hook_event_name"],
                  "permissionDecision": "deny",
                  "permissionDecisionReason": "Writing to /etc is not allowed",
              },
          }
      return {}
  ```

  ```typescript TypeScript theme={null}
  const blockEtcWrites: HookCallback = async (input, toolUseID, { signal }) => {
    const preInput = input as PreToolUseHookInput;
    const toolInput = preInput.tool_input as Record<string, unknown>;
    const filePath = toolInput?.file_path as string;

    if (filePath?.startsWith("/etc")) {
      return {
        // Top-level field: message shown to the user
        systemMessage: "Remember: system directories like /etc are protected.",
        // hookSpecificOutput: block the operation
        hookSpecificOutput: {
          hookEventName: preInput.hook_event_name,
          permissionDecision: "deny",
          permissionDecisionReason: "Writing to /etc is not allowed"
        }
      };
    }
    return {};
  };
  ```
</CodeGroup>

<h3 id="auto-approve-specific-tools">
  Auto-approve tools spesifik
</h3>

Secara default, agent dapat meminta permission sebelum menggunakan tools tertentu. Contoh ini auto-approves read-only filesystem tools (Read, Glob, Grep) dengan mengembalikan `permissionDecision: 'allow'`, membiarkan mereka berjalan tanpa konfirmasi pengguna sambil meninggalkan semua tools lainnya tunduk pada pemeriksaan permission normal:

<CodeGroup>
  ```python Python theme={null}
  async def auto_approve_read_only(input_data, tool_use_id, context):
      if input_data["hook_event_name"] != "PreToolUse":
          return {}

      read_only_tools = ["Read", "Glob", "Grep"]
      if input_data["tool_name"] in read_only_tools:
          return {
              "hookSpecificOutput": {
                  "hookEventName": input_data["hook_event_name"],
                  "permissionDecision": "allow",
                  "permissionDecisionReason": "Read-only tool auto-approved",
              }
          }
      return {}
  ```

  ```typescript TypeScript theme={null}
  const autoApproveReadOnly: HookCallback = async (input, toolUseID, { signal }) => {
    if (input.hook_event_name !== "PreToolUse") return {};

    const preInput = input as PreToolUseHookInput;
    const readOnlyTools = ["Read", "Glob", "Grep"];
    if (readOnlyTools.includes(preInput.tool_name)) {
      return {
        hookSpecificOutput: {
          hookEventName: preInput.hook_event_name,
          permissionDecision: "allow",
          permissionDecisionReason: "Read-only tool auto-approved"
        }
      };
    }
    return {};
  };
  ```
</CodeGroup>

<h3 id="register-multiple-hooks">
  Daftarkan multiple hooks
</h3>

Ketika event terjadi, semua hooks yang cocok berjalan secara paralel. Untuk keputusan permission, hasil yang paling ketat menang: satu `deny` memblokir pemanggilan tool terlepas dari apa yang dikembalikan hooks lainnya. Karena urutan penyelesaian tidak dapat diprediksi, tulis setiap hook untuk bertindak secara independen daripada mengandalkan hook lain yang telah berjalan terlebih dahulu.

Contoh di bawah ini mendaftarkan tiga pemeriksaan independen untuk setiap pemanggilan tool:

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(
      hooks={
          "PreToolUse": [
              HookMatcher(hooks=[authorization_check]),
              HookMatcher(hooks=[input_validator]),
              HookMatcher(hooks=[audit_logger]),
          ]
      }
  )
  ```

  ```typescript TypeScript theme={null}
  const options = {
    hooks: {
      PreToolUse: [
        { hooks: [authorizationCheck] },
        { hooks: [inputValidator] },
        { hooks: [auditLogger] }
      ]
    }
  };
  ```
</CodeGroup>

<h3 id="filter-with-multi-tool-matchers">
  Filter dengan multi-tool matchers
</h3>

Gunakan multi-tool matchers untuk berbagi satu callback di seluruh tools terkait. Contoh ini mendaftarkan tiga matchers dengan scope berbeda:

* Daftar exact yang dipisahkan pipe (`Write|Edit|NotebookEdit`) memicu `file_security_hook` hanya untuk file modification tools.
* Regex (`^mcp__`) memicu `mcp_audit_hook` untuk tool MCP apa pun yang namanya dimulai dengan `mcp__`.
* Matcher yang dihilangkan memicu `global_logger` untuk setiap pemanggilan tool terlepas dari nama.

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(
      hooks={
          "PreToolUse": [
              # Match file modification tools
              HookMatcher(matcher="Write|Edit|NotebookEdit", hooks=[file_security_hook]),
              # Match all MCP tools
              HookMatcher(matcher="^mcp__", hooks=[mcp_audit_hook]),
              # Match everything (no matcher)
              HookMatcher(hooks=[global_logger]),
          ]
      }
  )
  ```

  ```typescript TypeScript theme={null}
  const options = {
    hooks: {
      PreToolUse: [
        // Match file modification tools
        { matcher: "Write|Edit|NotebookEdit", hooks: [fileSecurityHook] },

        // Match all MCP tools
        { matcher: "^mcp__", hooks: [mcpAuditHook] },

        // Match everything (no matcher)
        { hooks: [globalLogger] }
      ]
    }
  };
  ```
</CodeGroup>

<h3 id="track-subagent-activity">
  Lacak aktivitas subagent
</h3>

Gunakan hooks `SubagentStop` untuk memantau ketika subagents menyelesaikan pekerjaan mereka. Lihat tipe input lengkap di referensi SDK [TypeScript](/docs/id/agent-sdk/typescript#hookinput) dan [Python](/docs/id/agent-sdk/python#hookinput). Contoh ini mencatat ringkasan setiap kali subagent selesai:

<CodeGroup>
  ```python Python theme={null}
  async def subagent_tracker(input_data, tool_use_id, context):
      # Log subagent details when it finishes
      print(f"[SUBAGENT] Completed: {input_data['agent_id']}")
      print(f"  Transcript: {input_data['agent_transcript_path']}")
      print(f"  Tool use ID: {tool_use_id}")
      print(f"  Stop hook active: {input_data.get('stop_hook_active')}")
      return {}


  options = ClaudeAgentOptions(
      hooks={"SubagentStop": [HookMatcher(hooks=[subagent_tracker])]}
  )
  ```

  ```typescript TypeScript theme={null}
  import { HookCallback, SubagentStopHookInput } from "@anthropic-ai/claude-agent-sdk";

  const subagentTracker: HookCallback = async (input, toolUseID, { signal }) => {
    // Cast to SubagentStopHookInput to access subagent-specific fields
    const subInput = input as SubagentStopHookInput;

    // Log subagent details when it finishes
    console.log(`[SUBAGENT] Completed: ${subInput.agent_id}`);
    console.log(`  Transcript: ${subInput.agent_transcript_path}`);
    console.log(`  Tool use ID: ${toolUseID}`);
    console.log(`  Stop hook active: ${subInput.stop_hook_active}`);
    return {};
  };

  const options = {
    hooks: {
      SubagentStop: [{ hooks: [subagentTracker] }]
    }
  };
  ```
</CodeGroup>

<h3 id="make-http-requests-from-hooks">
  Buat HTTP requests dari hooks
</h3>

Hooks dapat melakukan operasi asynchronous seperti HTTP requests. Tangkap errors di dalam hook Anda daripada membiarkan mereka menyebar.

Contoh ini mengirim webhook setelah setiap tool selesai, mencatat tool mana yang berjalan dan kapan. Hook menangkap errors dari webhook yang gagal:

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  import json
  import urllib.request
  from datetime import datetime


  def _send_webhook(tool_name):
      """Synchronous helper that POSTs tool usage data to an external webhook."""
      data = json.dumps(
          {
              "tool": tool_name,
              "timestamp": datetime.now().isoformat(),
          }
      ).encode()
      req = urllib.request.Request(
          "https://api.example.com/webhook",
          data=data,
          headers={"Content-Type": "application/json"},
          method="POST",
      )
      urllib.request.urlopen(req)


  async def webhook_notifier(input_data, tool_use_id, context):
      # Only fire after a tool completes (PostToolUse), not before
      if input_data["hook_event_name"] != "PostToolUse":
          return {}

      try:
          # Run the blocking HTTP call in a thread to avoid blocking the event loop
          await asyncio.to_thread(_send_webhook, input_data["tool_name"])
      except Exception as e:
          # Log the error but don't raise
          print(f"Webhook request failed: {e}")

      return {}
  ```

  ```typescript TypeScript theme={null}
  import { query, HookCallback, PostToolUseHookInput } from "@anthropic-ai/claude-agent-sdk";

  const webhookNotifier: HookCallback = async (input, toolUseID, { signal }) => {
    // Only fire after a tool completes (PostToolUse), not before
    if (input.hook_event_name !== "PostToolUse") return {};

    try {
      await fetch("https://api.example.com/webhook", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          tool: (input as PostToolUseHookInput).tool_name,
          timestamp: new Date().toISOString()
        }),
        // Pass signal so the request cancels if the hook times out
        signal
      });
    } catch (error) {
      // Handle cancellation separately from other errors
      if (error instanceof Error && error.name === "AbortError") {
        console.log("Webhook request cancelled");
      }
      // Don't re-throw
    }

    return {};
  };

  // Register as a PostToolUse hook
  for await (const message of query({
    prompt: "Refactor the auth module",
    options: {
      hooks: {
        PostToolUse: [{ hooks: [webhookNotifier] }]
      }
    }
  })) {
    console.log(message);
  }
  ```
</CodeGroup>

Untuk mengonfirmasi hook terjadi, arahkan URL webhook ke endpoint yang dapat Anda pantau dan kirim prompt yang menggunakan tool: hook mengirim POST dengan nama tool dan timestamp setelah setiap tool selesai.

<h3 id="forward-notifications-to-slack">
  Teruskan notifikasi ke Slack
</h3>

Gunakan hooks `Notification` untuk menerima notifikasi sistem dari agent dan meneruskannya ke layanan eksternal. Dalam sesi SDK, Claude Code menjalankan hook ini untuk tipe notifikasi berikut:

* [`permission_prompt`](/docs/id/hooks#notification) sekali permintaan permission telah menunggu sekitar enam detik di callback [`canUseTool`](/docs/id/agent-sdk/user-input) Anda. Memerlukan TypeScript Agent SDK v0.3.233 atau lebih baru, atau Python Agent SDK v0.2.139 atau lebih baru
* `elicitation_complete` dan `elicitation_response` untuk alur elicitation user-prompt

Claude Code memancarkan tipe lainnya, seperti `idle_prompt`, `auth_success`, dan `elicitation_dialog`, dari UI interaktif yang sesi SDK tidak jalankan.

Setiap notifikasi mencakup field `message` dengan deskripsi yang dapat dibaca manusia dan secara opsional `title`.

Contoh ini meneruskan setiap notifikasi ke channel Slack. Ini memerlukan [Slack incoming webhook URL](https://docs.slack.dev/messaging/sending-messages-using-incoming-webhooks/), yang Anda buat dengan menambahkan app ke workspace Slack Anda dan mengaktifkan incoming webhooks:

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  import json
  import urllib.request

  from claude_agent_sdk import ClaudeSDKClient, ClaudeAgentOptions, HookMatcher


  def _send_slack_notification(message):
      """Synchronous helper that sends a message to Slack via incoming webhook."""
      data = json.dumps({"text": f"Agent status: {message}"}).encode()
      req = urllib.request.Request(
          "https://hooks.slack.com/services/YOUR/WEBHOOK/URL",
          data=data,
          headers={"Content-Type": "application/json"},
          method="POST",
      )
      urllib.request.urlopen(req)


  async def notification_handler(input_data, tool_use_id, context):
      try:
          # Run the blocking HTTP call in a thread to avoid blocking the event loop
          await asyncio.to_thread(_send_slack_notification, input_data.get("message", ""))
      except Exception as e:
          print(f"Failed to send notification: {e}")

      # Return empty object. Notification hooks don't modify agent behavior
      return {}


  async def main():
      options = ClaudeAgentOptions(
          hooks={
              # Register the hook for Notification events (no matcher needed)
              "Notification": [HookMatcher(hooks=[notification_handler])],
          },
      )

      async with ClaudeSDKClient(options=options) as client:
          await client.query("Analyze this codebase")
          async for message in client.receive_response():
              print(message)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query, HookCallback, NotificationHookInput } from "@anthropic-ai/claude-agent-sdk";

  // Define a hook callback that sends notifications to Slack
  const notificationHandler: HookCallback = async (input, toolUseID, { signal }) => {
    // Cast to NotificationHookInput to access the message field
    const notification = input as NotificationHookInput;

    try {
      // POST the notification message to a Slack incoming webhook
      await fetch("https://hooks.slack.com/services/YOUR/WEBHOOK/URL", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          text: `Agent status: ${notification.message}`
        }),
        // Pass signal so the request cancels if the hook times out
        signal
      });
    } catch (error) {
      if (error instanceof Error && error.name === "AbortError") {
        console.log("Notification cancelled");
      } else {
        console.error("Failed to send notification:", error);
      }
    }

    // Return empty object. Notification hooks don't modify agent behavior
    return {};
  };

  // Register the hook for Notification events (no matcher needed)
  for await (const message of query({
    prompt: "Analyze this codebase",
    options: {
      hooks: {
        Notification: [{ hooks: [notificationHandler] }]
      }
    }
  })) {
    console.log(message);
  }
  ```
</CodeGroup>

Ketika event `Notification` terjadi, hook memposting `message` notifikasi, dengan awalan `Agent status:`, ke channel yang ditargetkan webhook Anda.

<h2 id="fix-common-issues">
  Perbaiki masalah umum
</h2>

<h3 id="hook-not-firing">
  Hook not firing
</h3>

* Verifikasi nama hook event benar dan case-sensitive (`PreToolUse`, bukan `preToolUse`)
* Periksa bahwa pola matcher Anda cocok dengan nama tool dengan tepat
* Pastikan hook berada di bawah tipe event yang benar di `options.hooks`
* Untuk non-tool hooks yang mendukung matchers, seperti `Notification` dan `SubagentStop`, matchers cocok dengan fields berbeda, dan `Stop` mengabaikan matchers sepenuhnya (lihat [matcher patterns](/docs/id/hooks#matcher-patterns))
* Hooks mungkin tidak terjadi ketika agent mencapai batas [`max_turns`](/docs/id/agent-sdk/python#claudeagentoptions) karena sesi berakhir sebelum hooks dapat dieksekusi

<h3 id="matcher-not-filtering-as-expected">
  Matcher not filtering as expected
</h3>

Matchers hanya cocok dengan nama tool, bukan jalur file atau argumen lainnya. Untuk memfilter berdasarkan jalur file, periksa `tool_input.file_path` di dalam hook Anda:

```typescript theme={null}
const myHook: HookCallback = async (input, toolUseID, { signal }) => {
  const preInput = input as PreToolUseHookInput;
  const toolInput = preInput.tool_input as Record<string, unknown>;
  const filePath = toolInput?.file_path as string;
  if (!filePath?.endsWith(".md")) return {}; // Skip non-markdown files
  // Process markdown files...
  return {};
};
```

<h3 id="hook-timeout">
  Hook timeout
</h3>

Claude Code menjalankan setiap callback dengan timeout, yang Anda atur dalam hitungan detik dengan field `timeout` pada `HookMatcher`-nya. Ketika Anda tidak mengaturnya, Claude Code menggunakan default event: 600 detik untuk sebagian besar events, 30 detik untuk `UserPromptSubmit`, `PreModelSwitch`, dan `PostModelSwitch`, dan 10 detik untuk `MessageDisplay`. Claude Code menjalankan callbacks `SessionEnd` selama shutdown di bawah [SessionEnd timeout budget](/docs/id/hooks#sessionend-input) yang lebih pendek, 1,5 detik secara default.

Ketika callback melampaui timeout-nya, Claude Code membatalkannya dan membuang output-nya, dan sesi berlanjut daripada hang. Apa yang terjadi selanjutnya tergantung pada event:

* `PreToolUse`: Claude Code tidak menjalankan tool call, Claude menerima hasil tool yang menyatakan hook tidak merespons sebelum timeout-nya, dan turn berlanjut. Jika hook `PreToolUse` lain mengembalikan explicit deny, Claude menerima denial itu sebagai gantinya dari error timeout. Sebelum v2.1.210, Claude Code melaporkan timeout ke Claude sebagai user rejection, yang membuat unattended sessions berhenti dan menunggu input.
* `PostToolUse` dan `PostToolUseFailure`: Claude Code menyimpan hasil tool dan turn berlanjut.
* `UserPromptSubmit` dan [`UserPromptExpansion`](/docs/id/hooks#userpromptexpansion): Claude Code memblokir prompt dengan pesan yang menyebutkan hook dan timeout, dan sesi berlanjut. Karena callback pada events ini dapat bertindak sebagai policy gate, Claude Code tidak pernah membiarkan prompt yang timed-out melewati tanpa screening. Sebelum v2.1.208, Claude Code mengakhiri query dengan `error_during_execution` ketika callback pada events ini timed out.
* `Stop` dan `SubagentStop`: callback yang timed-out dihitung sebagai tidak mengembalikan keputusan. Agent atau subagent berhenti seolah-olah callback itu telah mengizinkannya, dan keputusan dari hooks lain Anda pada event masih berlaku. Sebelum Claude Code v2.1.273, callback `Stop` atau `SubagentStop` yang timed-out dihitung sebagai failed hook run, dan Claude Code membuang keputusan hooks lain Anda pada event.
* `SessionStart`: callback yang timed-out dihitung sebagai tidak mengembalikan output, dan sesi berlanjut dengan output dari hooks `SessionStart` lain Anda.
* `PreModelSwitch`: Claude Code memblokir model switch. Hook yang tidak menjawab belum menyetujui switch.
* Events lainnya, seperti `Notification`, `PreCompact`, dan `PostModelSwitch`: Claude Code mencatat kegagalan dan berlanjut.

Pertama kali callback `Stop` atau `SessionStart` timed out di sesi utama, Claude Code juga menambahkan [`SDKInformationalMessage`](/docs/id/agent-sdk/typescript#sdkinformationalmessage) ke message stream mengatakan app yang menjalankan sesi tidak merespons. Timeouts kemudian tidak mengulangi pesan itu saat app Anda tetap tidak responsif.

Jika Anda mengganggu query saat callback tertunda, Claude Code membatalkan pending tool call. Sebelum v2.1.208, tool call masih bisa melanjutkan jika Anda mengganggu selama pending `PreToolUse` callback.

Jika callback Anda membutuhkan lebih banyak waktu, atur `timeout` yang lebih tinggi pada `HookMatcher`-nya. Di TypeScript, gunakan `AbortSignal` dari argumen callback ketiga untuk menangani pembatalan dengan baik ketika timeout terjadi.

<h3 id="tool-blocked-unexpectedly">
  Tool blocked unexpectedly
</h3>

* Periksa semua hooks `PreToolUse` untuk returns `permissionDecision: 'deny'`
* Tambahkan logging ke hooks Anda untuk melihat apa `permissionDecisionReason` yang mereka kembalikan
* Verifikasi pola matcher tidak terlalu luas: matcher kosong cocok dengan semua tools

<h3 id="modified-input-not-applied">
  Modified input not applied
</h3>

* Pastikan `updatedInput` berada di dalam `hookSpecificOutput`, bukan di top level:

  ```typescript theme={null}
  return {
    hookSpecificOutput: {
      hookEventName: "PreToolUse",
      permissionDecision: "allow",
      updatedInput: { command: "new command" }
    }
  };
  ```

* Jangan pasangkan `updatedInput` dengan `permissionDecision: 'defer'`, yang menghilangkan input yang dimodifikasi. Menghilangkan `permissionDecision` tidak masalah: input yang dimodifikasi masih berlaku melalui evaluasi permission normal. Anda juga dapat mengembalikan `'allow'` untuk auto-approve input yang dimodifikasi atau `'ask'` untuk menampilkannya kepada pengguna untuk persetujuan

* Sertakan `hookEventName` di `hookSpecificOutput` untuk mengidentifikasi tipe hook mana output-nya

<h3 id="session-hooks-not-available-in-python">
  Session hooks not available in Python
</h3>

`SessionStart` dan `SessionEnd` dapat didaftarkan sebagai SDK callback hooks di TypeScript, tetapi tidak tersedia di Python SDK karena tipe `HookEvent`-nya menghilangkannya. Di Python, mereka hanya tersedia sebagai [shell command hooks](/docs/id/hooks#hook-events) yang didefinisikan dalam file pengaturan seperti `.claude/settings.json`. Untuk memuat shell command hooks dari aplikasi SDK Anda, sertakan setting source yang sesuai dengan [`setting_sources`](/docs/id/agent-sdk/python#settingsource) atau [`settingSources`](/docs/id/agent-sdk/typescript#settingsource):

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(
      setting_sources=["project"],  # Loads .claude/settings.json including hooks
  )
  ```

  ```typescript TypeScript theme={null}
  const options = {
    settingSources: ["project"] // Loads .claude/settings.json including hooks
  };
  ```
</CodeGroup>

Untuk menjalankan logika inisialisasi sebagai callback Python SDK sebagai gantinya, gunakan pesan pertama dari `client.receive_response()` sebagai trigger Anda.

<h3 id="subagent-permission-prompts-multiplying">
  Subagent permission prompts multiplying
</h3>

Ketika spawning multiple subagents, masing-masing mungkin meminta permissions secara terpisah untuk tool calls-nya sendiri. Untuk menghindari prompts berulang, gunakan hooks `PreToolUse` untuk auto-approve tools spesifik, atau konfigurasi permission rules, yang subagents [warisi dari parent conversation](/docs/id/sub-agents#permission-modes).

<h3 id="recursive-hook-loops-with-subagents">
  Recursive hook loops with subagents
</h3>

Hook `UserPromptSubmit` yang spawns subagents dapat membuat infinite loops jika subagents tersebut memicu hook yang sama. Untuk mencegah ini:

* Gunakan shared variable atau session state untuk track apakah Anda sudah berada di dalam subagent
* Scope hooks untuk hanya berjalan untuk sesi top-level agent

<h3 id="systemmessage-not-appearing-in-output">
  systemMessage not appearing in output
</h3>

Field `systemMessage` menampilkan pesan kepada pengguna, bukan model. Pada Claude Code v2.1.227 atau lebih baru, `systemMessage` hook dapat muncul di message stream sebagai [`SDKInformationalMessage`](/docs/id/agent-sdk/typescript#sdkinformationalmessage). Apakah itu terjadi tergantung pada event. Setiap [section event](/docs/id/hooks#hook-events) di halaman hooks mengatakan bagaimana output muncul. Untuk meneruskan konteks ke model sebagai gantinya, kembalikan [`additionalContext`](/docs/id/hooks#add-context-for-claude).

Sebelum v2.1.227, SDK menampilkan hook output di message stream hanya untuk hooks `SessionStart` dan `Setup`. Untuk event lainnya, output muncul hanya di lifecycle events yang [`includeHookEvents`](/docs/id/agent-sdk/typescript#options) (`include_hook_events` di Python) tambahkan. Entry opsi itu mencakup lifecycle events mana yang setiap hook event hasilkan.

Jika Anda perlu menampilkan hook decisions ke aplikasi Anda dengan andal, log mereka secara terpisah atau gunakan dedicated output channel.

<h2 id="related-resources">
  Related resources
</h2>

* [Claude Code hooks reference](/docs/id/hooks): full JSON input/output schemas, event documentation, dan matcher patterns
* [Claude Code hooks guide](/docs/id/hooks-guide): shell command hook examples dan walkthroughs
* [TypeScript SDK reference](/docs/id/agent-sdk/typescript): hook types, input/output definitions, dan configuration options
* [Python SDK reference](/docs/id/agent-sdk/python): hook types, input/output definitions, dan configuration options
* [Permissions](/docs/id/agent-sdk/permissions): kontrol apa yang dapat dilakukan agent Anda
* [Custom tools](/docs/id/agent-sdk/custom-tools): build tools untuk extend agent capabilities
