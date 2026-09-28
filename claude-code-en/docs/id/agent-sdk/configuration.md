> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Konfigurasi agen Anda

> Konfigurasi sesi Agent SDK: susun objek opsi, atur model, lingkungan, dan batas, serta temukan halaman opsi setiap fitur.

Sesi Agent SDK membaca konfigurasi dari file pengaturan, variabel lingkungan, dan objek `options` yang Anda berikan saat memulainya. Halaman ini menunjukkan cara menyusun objek `options` dan apa file pengaturan serta variabel lingkungan yang mengontrolnya.

Untuk setiap tipe opsi dan default, lihat referensi [`Options`](/docs/id/agent-sdk/typescript#options) (TypeScript) dan [`ClaudeAgentOptions`](/docs/id/agent-sdk/python#claudeagentoptions) (Python).

<h2 id="pass-options-to-a-session">
  Berikan opsi ke sesi
</h2>

Setiap panggilan `query()` menerima objek opsi: `Options` di TypeScript, `ClaudeAgentOptions` di Python. Setiap bidang bersifat opsional, dan sesi yang dimulai tanpa opsi berjalan dengan default SDK. Contoh di bawah mengonfigurasi sesi baca-saja yang merangkum TODO terbuka proyek. Pasangan dibaca sebagai TypeScript / Python di mana ejaan berbeda:

* **`model`**: memilih model
* **`allowedTools` / `allowed_tools`**: pra-menyetujui daftar alat baca-saja
* **`maxTurns` / `max_turns`**: membatasi jumlah giliran
* **`cwd`**: menetapkan direktori kerja

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Summarize the open TODOs in this repo",
    options: {
      model: "claude-sonnet-5",
      allowedTools: ["Read", "Glob", "Grep"],
      maxTurns: 8,
      cwd: "/path/to/repo",
    },
  })) {
    if (message.type === "result" && message.subtype === "success" && !message.is_error) {
      console.log(message.result);
    }
  }
  ```

  ```python Python theme={null}
  import asyncio

  from claude_agent_sdk import ClaudeAgentOptions, ResultMessage, query

  async def main():
      options = ClaudeAgentOptions(
          model="claude-sonnet-5",
          allowed_tools=["Read", "Glob", "Grep"],
          max_turns=8,
          cwd="/path/to/repo",
      )

      async for message in query(
          prompt="Summarize the open TODOs in this repo",
          options=options,
      ):
          if isinstance(message, ResultMessage) and not message.is_error:
              print(message.result)

  asyncio.run(main())
  ```
</CodeGroup>

Arahkan `cwd` ke salah satu proyek Anda sendiri dan jalankan contohnya. Ringkasan TODO terbuka proyek tersebut akan dicetak saat pesan hasil tiba.

`allowedTools` (TypeScript) atau `allowed_tools` (Python) pra-menyetujui alat yang terdaftar, sehingga panggilan ke alat tersebut berjalan tanpa menunggu persetujuan. Alat di luar daftar tetap tersedia. Ketika Claude memanggil alat yang tidak terdaftar, mode izin menentukan apakah panggilan berjalan. Untuk informasi lebih lanjut, lihat [Aturan izin dan penolakan](/docs/id/agent-sdk/permissions#allow-and-deny-rules).

<h2 id="load-settings-files">
  Muat file pengaturan
</h2>

File pengaturan menyediakan konfigurasi di luar objek opsi. Dua opsi mengontrol cara memuatnya:

* **`settingSources` / `setting_sources`**: mengontrol sumber sistem file mana yang dimuat: pengguna, proyek, dan lokal. File pengaturan dan file CLAUDE.md tiba melalui sumber ini.
* **`settings`**: memuat jalur file pengaturan atau string JSON sebaris dalam bahasa apa pun, dan TypeScript juga menerima objek pengaturan. Bentuk apa pun yang Anda berikan menggantikan pengaturan sistem file pengguna, proyek, dan lokal; hanya pengaturan kebijakan terkelola yang lebih tinggi. Referensi mendokumentasikan urutan preseden lengkap di bawah [Preseden pengaturan](/docs/id/agent-sdk/typescript#settings-precedence) untuk TypeScript dan [Preseden pengaturan](/docs/id/agent-sdk/python#settings-precedence) untuk Python.

Berikan `[]` untuk menonaktifkan pengaturan pengguna, proyek, dan lokal. Untuk informasi lebih lanjut, lihat [Gunakan fitur Claude Code di SDK](/docs/id/agent-sdk/claude-code-features).

<h2 id="choose-a-model">
  Pilih model
</h2>

Kecuali opsi `model`, pengaturan Anda, atau lingkungan Anda memilih model, sesi baru dimulai pada [model default Claude Code](/docs/id/model-config#default-model-setting). Untuk urutan sumber tersebut, lihat [Atur model Anda](/docs/id/model-config#setting-your-model). Atur `model` untuk menetapkan model tertentu, atau untuk memilih model yang lebih kecil untuk agen yang lebih cepat dan lebih murah. Nilai mengambil alias model atau nama model lengkap; alias dan versi yang mereka selesaikan terdaftar di bawah [Alias model](/docs/id/model-config#model-aliases).

Atur `fallbackModel` (TypeScript) atau `fallback_model` (Python) untuk menamai model cadangan. Ketika model utama kelebihan beban atau tidak tersedia, sesi beralih ke cadangan. Model utama dicoba ulang di awal setiap giliran pengguna, sehingga sesi kembali ke model utama setelah pemadaman berlalu.

Di kedua bahasa, opsi menerima model tunggal atau daftar cadangan yang dipisahkan koma. Untuk urutan dan batas rantai, lihat [Rantai model fallback](/docs/id/model-config#fallback-model-chains). Di TypeScript, fallback yang sama dengan `model` melempar kesalahan saat startup.

Contoh di bawah menunjukkan daftar fallback di TypeScript dan fallback tunggal di Python:

<CodeGroup>
  ```typescript TypeScript theme={null}
  const options = {
    model: "claude-fable-5",
    fallbackModel: "claude-opus-5,claude-sonnet-5",
  };
  ```

  ```python Python theme={null}
  options = ClaudeAgentOptions(
      model="claude-fable-5",
      fallback_model="claude-opus-5",
  )
  ```
</CodeGroup>

<span id="sampling-parameters" />

<Note>
  Parameter permintaan Messages API [Messages API](https://platform.claude.com/docs/en/api/messages) `temperature`, `top_p`, dan `max_tokens` tidak memiliki bidang pada objek opsi di kedua bahasa. Atur [tingkat upaya](/docs/id/agent-sdk/agent-loop#effort-level) atau [batas pengeluaran](#limit-turns-and-spend) sebagai gantinya, atau panggil Messages API ketika Anda memerlukan parameter tersebut secara langsung.
</Note>

<h2 id="set-environment-variables">
  Atur variabel lingkungan
</h2>

Opsi `env` menetapkan variabel lingkungan untuk proses Claude Code yang menjalankan sesi Anda. Apakah nilai Anda menggantikan lingkungan yang diwariskan atau menggabungkannya berbeda menurut bahasa:

* **TypeScript**: `env` menggantikan lingkungan subproses
* **Python**: SDK menggabungkan nilai Anda di atas lingkungan yang diwariskan, dan nilai Anda menggantikan nilai yang diwariskan

Di TypeScript, sebarkan `process.env` ke dalam `env` untuk menyimpan variabel yang diwariskan seperti `PATH`, `HOME`, dan `ANTHROPIC_API_KEY`. Ketika Anda membiarkan `env` tidak diatur, subproses mewarisi lingkungan Anda di kedua bahasa.

Contoh merutekan lalu lintas API melalui gateway dengan menetapkan `ANTHROPIC_BASE_URL`.

<CodeGroup>
  ```typescript TypeScript theme={null}
  const options = {
    env: { ...process.env, ANTHROPIC_BASE_URL: "https://gateway.example.com" },
  };
  ```

  ```python Python theme={null}
  options = ClaudeAgentOptions(
      env={"ANTHROPIC_BASE_URL": "https://gateway.example.com"},
  )
  ```
</CodeGroup>

Variabel yang Anda berikan juga dapat mengonfigurasi Claude Code itu sendiri. Untuk variabel yang dibaca proses Claude Code, lihat [Variabel lingkungan](/docs/id/env-vars). Untuk menyetel waktu tunggu API dan deteksi macet dengan cara ini, ikuti bagian Tangani respons API yang lambat atau macet di referensi [TypeScript](/docs/id/agent-sdk/typescript#handle-slow-or-stalled-api-responses) atau referensi [Python](/docs/id/agent-sdk/python#handle-slow-or-stalled-api-responses).

<h2 id="set-the-working-directory">
  Atur direktori kerja
</h2>

Atur `cwd` untuk menjalankan sesi di direktori tertentu. Ketika Anda membiarkan `cwd` tidak diatur, sesi berjalan di direktori kerja proses Anda. Tidak ada SDK yang memiliki setter untuk `cwd`. Untuk menjalankan di direktori berbeda, mulai sesi lain dengan `cwd` tersebut.

Claude Code membaca direktori kerja untuk menentukan:

* **Pengaturan dan hook proyek**: pengaturan dan hook proyek mana yang [dimuat](/docs/id/agent-sdk/claude-code-features)
* **Skills**: di mana [skill sesi ditemukan](/docs/id/agent-sdk/skills)
* **Penyimpanan sesi**: proyek mana yang [sesi tersimpan miliknya](/docs/id/agent-sdk/session-storage)

Untuk membiarkan alat menjangkau file di luar direktori kerja, tambahkan jalur dengan `additionalDirectories` (TypeScript) atau `add_dirs` (Python). Untuk cakupan hibah tersebut, lihat [Direktori tambahan memberikan akses file, bukan konfigurasi](/docs/id/permissions#additional-directories-grant-file-access-not-configuration).

<h2 id="limit-turns-and-spend">
  Batasi giliran dan pengeluaran
</h2>

Batasi giliran dan pengeluaran dengan `maxTurns` / `max_turns` dan `maxBudgetUsd` / `max_budget_usd`. Kedua batas dimatikan saat tidak diatur. Ketika sesi mencapai batas, jalankan berakhir dengan pesan hasil yang subtipe-nya menamai batas, `error_max_turns` atau `error_max_budget_usd`. Apa yang terjadi selanjutnya berbeda menurut mode input:

* **`query()` sekali jalan**: SDK menghasilkan hasil batas dan kemudian melempar, jadi bungkus loop dalam blok try untuk melanjutkan melewati kesalahan
* **Input streaming**: sesi tetap hidup melewati hasil batas, dan hitungan giliran maksimal dimulai ulang untuk setiap pesan antrian. Total anggaran terakumulasi di seluruh pesan, dan setelah pengeluaran mencapai batas, pesan nanti dalam percakapan yang sama berakhir dengan hasil anggaran yang sama. [`/clear`](/docs/id/agent-sdk/cost-tracking) memulai anggaran dari awal

Kedua batas memperlakukan `0` secara berbeda:

* **`maxTurns` / `max_turns`**: `0` menjalankan sesi tanpa batas giliran, sama dengan membiarkan opsi tidak diatur
* **`maxBudgetUsd` / `max_budget_usd`**: CLI menolak `0` sebagai jumlah yang tidak valid saat startup, dan sesi tidak pernah berjalan

Untuk informasi lebih lanjut tentang kedua batas, termasuk pengeluaran subagen, lihat [Giliran dan anggaran](/docs/id/agent-sdk/agent-loop#turns-and-budget).

<h2 id="change-configuration-mid-session">
  Ubah konfigurasi di tengah sesi
</h2>

Ketika Anda memulai sesi dengan [input streaming](/docs/id/agent-sdk/streaming-vs-single-mode), Anda dapat mengganti model dan mode izinnya saat berjalan. Tempat Anda memanggil setter berbeda menurut bahasa:

* **TypeScript**: metode pada objek yang `query()` kembalikan
* **Python**: metode pada [`ClaudeSDKClient`](/docs/id/agent-sdk/python#claudesdkclient), karena `query()` mengembalikan iterator biasa tanpa metode kontrol

Kedua bahasa memiliki setter yang sama:

* **`setModel()` / `set_model()`**: mengganti model. Panggilnya tanpa model untuk beralih ke [model default Claude Code](/docs/id/model-config#default-model-setting) daripada `model` yang Anda berikan dalam opsi.
* **`setPermissionMode()` / `set_permission_mode()`**: mengganti mode izin

TypeScript juga memiliki `applyFlagSettings()` dan `updateSettings()`:

* **`applyFlagSettings()`**: menerapkan pengaturan saat runtime, seperti dalam `await session.applyFlagSettings({ effortLevel: "high" })`. Metode mengambil kunci file pengaturan daripada bidang opsi, jadi periksa referensi [`applyFlagSettings()`](/docs/id/agent-sdk/typescript#applyflagsettings) untuk skema dan untuk kunci mana yang berlaku di tengah sesi.
* **`updateSettings()`**: menulis satu kunci yang diizinkan ke file pengaturan. Referensi [`updateSettings()`](/docs/id/agent-sdk/typescript#updatesettings) menamai kunci yang diterima setiap sumber dan lantai versi.
  * Berikan `"localSettings"` untuk menulis file pengaturan lokal proyek, seperti dalam `await session.updateSettings("localSettings", { outputStyle: "Explanatory" })`. Kunci yang ditulis berlaku pada permintaan sesi berikutnya dan bertahan untuk sesi nanti yang memuat pengaturan `local`.
  * Berikan `"userSettings"` untuk menulis `effortLevel`, satu-satunya kunci yang sumber terima. Claude Code menyimpannya sebagai tingkat upaya default untuk model sesi saat ini, dan upaya sesi yang berjalan tidak berubah.

Contoh di bawah menjalankan sesi dua giliran, mengubah konfigurasi di antara giliran, dan mencetak model yang menjawab setiap giliran. Di TypeScript, aliran prompt menyimpan pesan kedua sampai setter telah berjalan, dan giliran kedua berjalan pada model baru.

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query, type SDKUserMessage } from "@anthropic-ai/claude-agent-sdk";

  function userMessage(text: string): SDKUserMessage {
    return { type: "user", message: { role: "user", content: text }, parent_tool_use_id: null };
  }

  // Hold the second prompt until the setters have run.
  let startSecondTurn!: () => void;
  const secondTurnReady = new Promise<void>((resolve) => {
    startSecondTurn = resolve;
  });

  async function* turnPrompts(): AsyncGenerator<SDKUserMessage, void> {
    yield userMessage("Reply with exactly: ready");
    await secondTurnReady;
    yield userMessage("Reply with exactly: done");
  }

  const session = query({
    prompt: turnPrompts(),
    options: {
      model: "claude-sonnet-5",
    },
  });

  let turnModel = "";
  let completedTurns = 0;

  for await (const message of session) {
    if (message.type === "assistant") {
      turnModel = message.message.model;
    } else if (message.type === "result") {
      completedTurns += 1;
      if (completedTurns === 1) {
        console.log(`First turn model: ${turnModel}`);
        await session.setModel("claude-opus-5");
        await session.setPermissionMode("acceptEdits");
        startSecondTurn();
      } else {
        console.log(`Second turn model: ${turnModel}`);
        break;
      }
    }
  }
  ```

  ```python Python theme={null}
  import asyncio

  from claude_agent_sdk import AssistantMessage, ClaudeAgentOptions, ClaudeSDKClient

  async def main():
      options = ClaudeAgentOptions(model="claude-sonnet-5")

      async with ClaudeSDKClient(options=options) as client:
          await client.query("Reply with exactly: ready")
          first_model = ""
          async for message in client.receive_response():
              if isinstance(message, AssistantMessage):
                  first_model = message.model

          await client.set_model("claude-opus-5")
          await client.set_permission_mode("acceptEdits")

          await client.query("Reply with exactly: done")
          second_model = ""
          async for message in client.receive_response():
              if isinstance(message, AssistantMessage):
                  second_model = message.model

      print(f"First turn model: {first_model}")
      print(f"Second turn model: {second_model}")

  asyncio.run(main())
  ```
</CodeGroup>

Pada Claude API, program mencetak `First turn model: claude-sonnet-5`, kemudian `Second turn model: claude-opus-5` setelah pengalihan.

<Note>
  Setiap model memiliki cache prompt-nya sendiri, jadi setelah pengalihan di tengah sesi, permintaan berikutnya menghitung ulang percakapan lengkap tanpa cache pada tarif model baru. Untuk informasi lebih lanjut, lihat [Mengganti model](/docs/id/prompt-caching#switching-models).
</Note>

<h2 id="configure-specific-features">
  Konfigurasi fitur spesifik
</h2>

Tabel di bawah memetakan setiap opsi ke fitur yang dikonfigurasinya. Untuk opsi yang tidak dicakup halaman ini, lihat referensi [TypeScript](/docs/id/agent-sdk/typescript#options) dan [Python](/docs/id/agent-sdk/python#claudeagentoptions). Jika Anda tahu tujuan Anda tetapi tidak tahu opsi mana yang melayaninya, mulai dari [Pilih fitur yang tepat](/docs/id/agent-sdk/claude-code-features#choose-the-right-feature).

| TypeScript                | Python                      | Mengontrol                                                    | Tercakup dalam                                                                                                                                                                                                   |
| ------------------------- | --------------------------- | ------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `permissionMode`          | `permission_mode`           | Apa yang dapat dilakukan agen tanpa persetujuan               | [Konfigurasi izin](/docs/id/agent-sdk/permissions)                                                                                                                                                                    |
| `allowedTools`            | `allowed_tools`             | Panggilan alat mana yang pra-disetujui                        | [Konfigurasi izin](/docs/id/agent-sdk/permissions)                                                                                                                                                                    |
| `canUseTool`              | `can_use_tool`              | Callback persetujuan Anda untuk panggilan alat                | [Tangani permintaan persetujuan alat](/docs/id/agent-sdk/user-input#handle-tool-approval-requests)                                                                                                                    |
| `systemPrompt`            | `system_prompt`             | Instruksi agen                                                | [Memodifikasi prompt sistem](/docs/id/agent-sdk/modifying-system-prompts)                                                                                                                                             |
| `settingSources`          | `setting_sources`           | Pengaturan sistem file mana yang dimuat                       | [Gunakan fitur Claude Code di SDK](/docs/id/agent-sdk/claude-code-features)                                                                                                                                           |
| `mcpServers`              | `mcp_servers`               | Server alat eksternal                                         | [Hubungkan ke alat eksternal dengan MCP](/docs/id/agent-sdk/mcp)                                                                                                                                                      |
| `agents`                  | `agents`                    | Definisi subagen                                              | [Subagen](/docs/id/agent-sdk/subagents)                                                                                                                                                                               |
| `hooks`                   | `hooks`                     | Callback pada titik siklus hidup                              | [Hooks](/docs/id/agent-sdk/hooks)                                                                                                                                                                                     |
| `skills`                  | `skills`                    | Skill mana yang dimuat                                        | [Perluas agen dengan skill](/docs/id/agent-sdk/skills)                                                                                                                                                                |
| `plugins`                 | `plugins`                   | Plugin mana yang dimuat                                       | [Plugin](/docs/id/agent-sdk/plugins)                                                                                                                                                                                  |
| `outputFormat`            | `output_format`             | Skema output terstruktur                                      | [Output terstruktur](/docs/id/agent-sdk/structured-outputs)                                                                                                                                                           |
| `resume`                  | `resume`                    | Melanjutkan sesi tersimpan                                    | [Sesi](/docs/id/agent-sdk/sessions)                                                                                                                                                                                   |
| `forkSession`             | `fork_session`              | Percabangan sesi                                              | [Sesi](/docs/id/agent-sdk/sessions)                                                                                                                                                                                   |
| `sessionStore`            | `session_store`             | Persistensi sesi eksternal                                    | [Penyimpanan sesi](/docs/id/agent-sdk/session-storage)                                                                                                                                                                |
| `enableFileCheckpointing` | `enable_file_checkpointing` | Edit file yang dapat diputar ulang                            | [Checkpointing file](/docs/id/agent-sdk/file-checkpointing)                                                                                                                                                           |
| `effort`                  | `effort`                    | Berapa banyak pekerjaan yang Claude masukkan ke dalam respons | [Tingkat upaya](/docs/id/agent-sdk/agent-loop#effort-level)                                                                                                                                                           |
| `sandbox`                 | `sandbox`                   | Perilaku sandbox untuk eksekusi alat                          | Referensi [TypeScript](/docs/id/agent-sdk/typescript#sandbox-configuration) dan [Python](/docs/id/agent-sdk/python#sandbox-configuration), dengan konteks penyebaran di [Penyebaran aman](/docs/id/agent-sdk/secure-deployment) |

<h2 id="next-steps">
  Langkah berikutnya
</h2>

Untuk melihat konfigurasi yang disusun menjadi agen yang berfungsi:

* **[Quickstart](/docs/id/agent-sdk/quickstart)**: bangun dan jalankan agen pertama dari awal hingga akhir
* **[Contoh](/docs/id/agent-sdk/examples)**: temukan proyek lengkap yang dapat dijalankan atau resep Claude Cookbook yang dipandu yang cocok dengan apa yang ingin Anda bangun
* **[Isolasi multi-penyewa](/docs/id/agent-sdk/hosting#multi-tenant-isolation)**: isolasi pengaturan dan memori setiap penyewa dengan `settingSources` / `setting_sources`, `env`, dan `cwd`
