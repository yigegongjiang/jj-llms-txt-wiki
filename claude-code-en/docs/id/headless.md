> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Jalankan Claude Code secara programatis

> Gunakan Agent SDK untuk menjalankan Claude Code secara programatis dari CLI, Python, atau TypeScript.

[Agent SDK](/docs/id/agent-sdk/overview) memberikan Anda alat yang sama, loop agen, dan manajemen konteks yang mendukung Claude Code. Tersedia sebagai CLI untuk skrip dan CI/CD, atau sebagai paket [Python](/docs/id/agent-sdk/python) dan [TypeScript](/docs/id/agent-sdk/typescript) untuk kontrol programatis penuh.

Untuk menjalankan Claude Code dalam mode non-interaktif, berikan `-p` dengan prompt Anda dan [opsi CLI](/docs/id/cli-reference) apa pun:

```bash theme={null}
claude -p "Find and fix the bug in auth.py" --allowedTools "Read,Edit,Bash"
```

Halaman ini mencakup penggunaan Agent SDK melalui CLI (`claude -p`). Untuk paket SDK Python dan TypeScript dengan output terstruktur, callback persetujuan alat, dan objek pesan asli, lihat [dokumentasi Agent SDK lengkap](/docs/id/agent-sdk/overview).

<h2 id="basic-usage">
  Penggunaan dasar
</h2>

Tambahkan bendera `-p` (atau `--print`) ke perintah `claude` apa pun untuk menjalankannya secara non-interaktif. Semua [opsi CLI](/docs/id/cli-reference) bekerja dengan `-p`, termasuk:

* `--continue` untuk [melanjutkan percakapan](#continue-conversations)
* `--allowedTools` untuk [persetujuan otomatis alat](#auto-approve-tools)
* `--output-format` untuk [output terstruktur](#get-structured-output)

Contoh ini menanyakan Claude tentang basis kode Anda dan mencetak respons:

```bash theme={null}
claude -p "What does the auth module do?"
```

<h3 id="start-faster-with-bare-mode">
  Mulai lebih cepat dengan bare mode
</h3>

Tambahkan `--bare` untuk mengurangi waktu startup dengan melewati penemuan otomatis hooks, skills, plugins, server MCP, auto memory, dan CLAUDE.md. Tanpanya, `claude -p` memuat [konteks](/docs/id/how-claude-code-works#the-context-window) yang sama dengan sesi interaktif, termasuk apa pun yang dikonfigurasi di direktori kerja atau `~/.claude`.

Bare mode berguna untuk CI dan skrip di mana Anda memerlukan hasil yang sama di setiap mesin. Hook di `~/.claude` rekan kerja atau server MCP di `.mcp.json` proyek tidak akan berjalan, karena bare mode tidak pernah membacanya. Hanya bendera yang Anda berikan secara eksplisit yang berlaku.

Contoh ini menjalankan tugas ringkasan sekali pakai dalam bare mode dan pra-menyetujui alat Read sehingga panggilan selesai tanpa prompt izin:

```bash theme={null}
claude --bare -p "Summarize this file" --allowedTools "Read"
```

Dalam bare mode Claude memiliki akses ke alat Bash, pembacaan file, dan pengeditan file. Berikan konteks apa pun yang Anda butuhkan dengan bendera:

| Untuk memuat             | Gunakan                                                 |
| ------------------------ | ------------------------------------------------------- |
| Penambahan prompt sistem | `--append-system-prompt`, `--append-system-prompt-file` |
| Pengaturan               | `--settings <file-or-json>`                             |
| Server MCP               | `--mcp-config <file-or-json>`                           |
| Agen kustom              | `--agents <json>`                                       |
| Plugin                   | `--plugin-dir <path>`, `--plugin-url <url>`             |

Bare mode melewati pembacaan OAuth dan keychain. Autentikasi Anthropic harus berasal dari `ANTHROPIC_API_KEY` atau `apiKeyHelper` dalam JSON yang diteruskan ke `--settings`. Amazon Bedrock, Google Cloud's Agent Platform, dan Microsoft Foundry menggunakan kredensial penyedia biasa mereka.

<Note>
  `--bare` adalah mode yang direkomendasikan untuk panggilan skrip dan SDK, dan akan menjadi default untuk `-p` di rilis mendatang.
</Note>

<h3 id="background-tasks-at-exit">
  Tugas latar belakang saat keluar
</h3>

Jika Claude memulai [tugas Bash latar belakang](/docs/id/tools-reference#bash-tool-behavior) selama jalankan `claude -p`, misalnya server dev atau build watch, tugas tersebut dihentikan sekitar lima detik setelah Claude mengembalikan hasil akhirnya dan stdin telah ditutup. Periode grace memungkinkan tugas yang selesai tepat setelah hasil masih memberikan outputnya. Sebelum v2.1.163, proses latar belakang yang tidak pernah keluar akan membuat invokasi `claude -p` tetap terbuka tanpa batas.

[Subagen](/docs/id/sub-agents) latar belakang dan alur kerja dikecualikan dari grace lima detik karena hasil mereka adalah bagian dari output akhir, jadi `claude -p` menunggu mereka selesai. Dari v2.1.182, tunggu itu dibatasi pada sepuluh menit secara default sehingga agen latar belakang yang macet tidak dapat membuat proses tetap terbuka tanpa batas. Sesuaikan batas dengan [`CLAUDE_CODE_PRINT_BG_WAIT_CEILING_MS`](/docs/id/env-vars), atau atur ke `0` untuk menunggu tanpa batas.

<h2 id="examples">
  Contoh
</h2>

Contoh-contoh ini menyoroti pola CLI umum. Untuk CI dan panggilan skrip lainnya, tambahkan [`--bare`](#start-faster-with-bare-mode) sehingga mereka tidak mengambil apa pun yang kebetulan dikonfigurasi secara lokal.

<h3 id="pipe-data-through-claude">
  Saluran data melalui Claude
</h3>

Mode non-interaktif membaca stdin, sehingga Anda dapat menyalurkan data dan mengarahkan respons keluar seperti alat baris perintah lainnya.

Contoh ini menyalurkan log build ke Claude dan menulis penjelasan ke file:

```bash theme={null}
cat build-error.txt | claude -p 'concisely explain the root cause of this build error' > output.txt
```

Dengan `--output-format json`, payload respons mencakup `total_cost_usd` dan rincian biaya per-model, sehingga pemanggil skrip dapat melacak pengeluaran per invokasi tanpa berkonsultasi dengan [dashboard penggunaan](/docs/id/costs).

<Note>
  Sejak Claude Code v2.1.128, stdin yang disalurkan dibatasi pada 10MB. Jika Anda melampaui batas, Claude Code keluar dengan kesalahan yang jelas dan status bukan nol. Untuk bekerja dengan input yang lebih besar, tulis konten ke file dan referensikan jalur file dalam prompt Anda alih-alih menyalurkannya.
</Note>

<h3 id="add-claude-to-a-build-script">
  Tambahkan Claude ke skrip build
</h3>

Anda dapat membungkus panggilan non-interaktif dalam skrip untuk menggunakan Claude sebagai linter atau reviewer khusus proyek.

Skrip `package.json` ini menyalurkan diff terhadap `main` ke Claude dan memintanya untuk melaporkan typo. Menyalurkan diff berarti Claude tidak memerlukan izin Bash untuk membacanya, dan tanda kutip ganda yang di-escape menjaga skrip portabel ke Windows:

```json theme={null}
{
  "scripts": {
    "lint:claude": "git diff main | claude -p \"you are a typo linter. for each typo in this diff, report filename:line on one line and the issue on the next. return nothing else.\""
  }
}
```

<h3 id="get-structured-output">
  Dapatkan output terstruktur
</h3>

Gunakan `--output-format` untuk mengontrol bagaimana respons dikembalikan:

* `text` (default): output teks biasa
* `json`: JSON terstruktur dengan hasil, ID sesi, dan metadata
* `stream-json`: JSON yang dibatasi baris baru untuk streaming real-time

Contoh ini mengembalikan ringkasan proyek sebagai JSON dengan metadata sesi, dengan hasil teks di bidang `result`:

```bash theme={null}
claude -p "Summarize this project" --output-format json
```

Untuk mendapatkan output yang sesuai dengan skema tertentu, gunakan `--output-format json` dengan `--json-schema` dan definisi [JSON Schema](https://json-schema.org/). Respons mencakup metadata tentang permintaan (ID sesi, penggunaan, dll.) dengan output terstruktur di bidang `structured_output`.

Contoh ini mengekstrak nama fungsi dan mengembalikannya sebagai array string:

```bash theme={null}
claude -p "Extract the main function names from auth.py" \
  --output-format json \
  --json-schema '{"type":"object","properties":{"functions":{"type":"array","items":{"type":"string"}}},"required":["functions"]}'
```

Jika nilainya bukan JSON Schema yang valid, `claude` keluar dengan `Error: --json-schema is not a valid JSON Schema` diikuti oleh diagnostik validator. Claude Code menerima skema yang menggunakan kata kunci `format`, seperti `"format": "email"`, tetapi memperlakukan `format` sebagai anotasi dan tidak memberlakukannya. Sebelum v2.1.205, Claude Code secara diam-diam mengabaikan skema yang tidak valid dan mengembalikan teks yang tidak terstruktur, dan memperlakukan skema apa pun yang berisi `format` sebagai tidak valid.

<Tip>
  Gunakan alat seperti [jq](https://jqlang.github.io/jq/) untuk mengurai respons dan mengekstrak bidang tertentu:

  ```bash theme={null}
  # Extract the text result
  claude -p "Summarize this project" --output-format json | jq -r '.result'

  # Extract structured output
  claude -p "Extract function names from auth.py" \
    --output-format json \
    --json-schema '{"type":"object","properties":{"functions":{"type":"array","items":{"type":"string"}}},"required":["functions"]}' \
    | jq '.structured_output'
  ```
</Tip>

<h3 id="stream-responses">
  Streaming respons
</h3>

Gunakan `--output-format stream-json` dengan `--verbose` dan `--include-partial-messages` untuk menerima token saat dihasilkan. Setiap baris adalah objek JSON yang mewakili acara:

```bash theme={null}
claude -p "Explain recursion" --output-format stream-json --verbose --include-partial-messages
```

Baris terakhir dari aliran adalah pesan `result` dengan teks respons akhir, biaya, dan metadata sesi. Sebelum v2.1.208, menyalurkan respons besar dapat memotong baris terakhir dan menghilangkan pesan `result`.

Contoh berikut menggunakan [jq](https://jqlang.github.io/jq/) untuk memfilter delta teks dan menampilkan hanya teks streaming. Bendera `-r` menampilkan string mentah (tanpa tanda kutip) dan `-j` bergabung tanpa baris baru sehingga token streaming terus menerus:

```bash theme={null}
claude -p "Write a poem" --output-format stream-json --verbose --include-partial-messages | \
  jq -rj 'select(.type == "stream_event" and .event.delta.type? == "text_delta") | .event.delta.text'
```

Ketika permintaan API gagal dengan kesalahan yang dapat dicoba ulang, Claude Code memancarkan acara `system/api_retry` sebelum mencoba ulang. Anda dapat menggunakan ini untuk menampilkan kemajuan percobaan ulang atau menerapkan logika backoff kustom.

| Bidang           | Tipe              | Deskripsi                                                                                                                                                                                                    |
| ---------------- | ----------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `type`           | `"system"`        | tipe pesan                                                                                                                                                                                                   |
| `subtype`        | `"api_retry"`     | mengidentifikasi ini sebagai acara percobaan ulang                                                                                                                                                           |
| `attempt`        | integer           | nomor percobaan saat ini, dimulai dari 1                                                                                                                                                                     |
| `max_retries`    | integer           | total percobaan ulang yang diizinkan                                                                                                                                                                         |
| `retry_delay_ms` | integer           | milidetik hingga percobaan berikutnya                                                                                                                                                                        |
| `error_status`   | integer atau null | kode status HTTP, atau `null` untuk kesalahan koneksi tanpa respons HTTP                                                                                                                                     |
| `error`          | string            | kategori kesalahan: `authentication_failed`, `oauth_org_not_allowed`, `billing_error`, `rate_limit`, `overloaded`, `invalid_request`, `model_not_found`, `server_error`, `max_output_tokens`, atau `unknown` |
| `uuid`           | string            | pengidentifikasi acara unik                                                                                                                                                                                  |
| `session_id`     | string            | sesi yang dimiliki acara                                                                                                                                                                                     |

Acara `system/init` melaporkan metadata sesi termasuk model, alat, server MCP, dan plugin yang dimuat. Ini adalah acara pertama dalam aliran kecuali acara startup mendahuluinya:

* Acara `plugin_install`, ketika [`CLAUDE_CODE_SYNC_PLUGIN_INSTALL`](/docs/id/env-vars) diatur.
* [Acara `hook_started`, `hook_progress`, dan `hook_response`](/docs/id/agent-sdk/typescript#sdkhookstartedmessage), saat hook [`SessionStart`](/docs/id/hooks#sessionstart) atau [`Setup`](/docs/id/hooks#setup) yang dikonfigurasi berjalan. Ini streaming saat hook menghasilkannya. Claude Code v2.1.169 hingga v2.1.203 mengirimkannya dalam satu batch setelah hook selesai, masih sebelum `system/init`; v2.1.204 mengembalikan pengiriman langsung.

Acara ini juga membawa array `capabilities` opsional dari string yang menamai perilaku protokol yang diimplementasikan versi Claude Code ini, seperti `interrupt_receipt_v1`. Periksanya untuk mendeteksi fitur alih-alih membandingkan string versi, dan abaikan nilai yang tidak Anda kenali. Bidang ini memerlukan Claude Code v2.1.205 atau lebih baru dan tidak ada di versi sebelumnya. Lihat [`SDKSystemMessage`](/docs/id/agent-sdk/typescript#sdksystemmessage) untuk daftar kemampuan.

Gunakan bidang plugin untuk gagal CI ketika plugin tidak dimuat:

| Bidang          | Tipe  | Deskripsi                                                                                                                                                                                                                                                                                                                              |
| --------------- | ----- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `plugins`       | array | plugin yang berhasil dimuat, masing-masing dengan `name` dan `path`                                                                                                                                                                                                                                                                    |
| `plugin_errors` | array | kesalahan waktu muat plugin, masing-masing dengan `plugin`, `type`, dan `message`. Mencakup versi dependensi yang tidak terpenuhi dan kegagalan muat `--plugin-dir` seperti jalur yang hilang atau arsip yang tidak valid. Plugin yang terpengaruh diturunkan dan tidak ada di `plugins`. Kunci dihilangkan ketika tidak ada kesalahan |

Ketika [`CLAUDE_CODE_SYNC_PLUGIN_INSTALL`](/docs/id/env-vars) diatur, Claude Code memancarkan acara `system/plugin_install` saat plugin marketplace dipasang sebelum giliran pertama. Gunakan ini untuk menampilkan kemajuan pemasangan di UI Anda sendiri.

| Bidang       | Tipe                                                       | Deskripsi                                                                                                              |
| ------------ | ---------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------- |
| `type`       | `"system"`                                                 | tipe pesan                                                                                                             |
| `subtype`    | `"plugin_install"`                                         | mengidentifikasi ini sebagai acara pemasangan plugin                                                                   |
| `status`     | `"started"`, `"installed"`, `"failed"`, atau `"completed"` | `started` dan `completed` membatasi pemasangan keseluruhan; `installed` dan `failed` melaporkan marketplace individual |
| `name`       | string, opsional                                           | nama marketplace, hadir pada `installed` dan `failed`                                                                  |
| `error`      | string, opsional                                           | pesan kegagalan, hadir pada `failed`                                                                                   |
| `uuid`       | string                                                     | pengidentifikasi acara unik                                                                                            |
| `session_id` | string                                                     | sesi yang dimiliki acara                                                                                               |

Untuk streaming programatis dengan callback dan objek pesan, lihat [Stream responses in real-time](/docs/id/agent-sdk/streaming-output) dalam dokumentasi Agent SDK.

<h3 id="auto-approve-tools">
  Persetujuan otomatis alat
</h3>

Gunakan `--allowedTools` untuk membiarkan Claude menggunakan alat tertentu tanpa meminta. Contoh ini menjalankan suite pengujian dan memperbaiki kegagalan, memungkinkan Claude untuk menjalankan perintah Bash dan membaca/mengedit file tanpa meminta izin:

```bash theme={null}
claude -p "Run the test suite and fix any failures" \
  --allowedTools "Bash,Read,Edit"
```

Untuk menetapkan baseline untuk seluruh sesi alih-alih mencantumkan alat individual, berikan [mode izin](/docs/id/permission-modes). `dontAsk` menolak apa pun yang tidak ada dalam aturan `permissions.allow` Anda atau [set perintah read-only](/docs/id/permissions#read-only-commands), yang berguna untuk CI runs yang terkunci. `AskUserQuestion`, alat konektor [organisasi Anda atur ke `ask`](/docs/id/mcp#organization-controls-on-connector-tools), dan alat MCP yang ditandai [`requiresUserInteraction`](/docs/id/mcp#require-approval-for-a-specific-tool) ditolak bahkan ketika aturan allow cocok.

`acceptEdits` memungkinkan Claude menulis file tanpa meminta dan juga persetujuan otomatis perintah filesystem umum seperti `mkdir`, `touch`, `mv`, dan `cp`. Perintah shell lainnya dan permintaan jaringan masih memerlukan entri `--allowedTools` atau aturan `permissions.allow`, jika tidak run akan berhenti ketika salah satu dicoba:

```bash theme={null}
claude -p "Apply the lint fixes" --permission-mode acceptEdits
```

<h3 id="create-a-commit">
  Buat komit
</h3>

Contoh ini meninjau perubahan yang dipentaskan dan membuat komit dengan pesan yang sesuai:

```bash theme={null}
claude -p "Look at my staged changes and create an appropriate commit" \
  --allowedTools "Bash(git diff *),Bash(git log *),Bash(git status *),Bash(git commit *)"
```

Bendera `--allowedTools` menggunakan [sintaks aturan izin](/docs/id/settings#permission-rule-syntax). Spasi di akhir ` *` memungkinkan pencocokan awalan, jadi `Bash(git diff *)` memungkinkan perintah apa pun yang dimulai dengan `git diff`. Spasi sebelum `*` penting: tanpanya, `Bash(git diff*)` juga akan cocok dengan `git diff-index`.

<Note>
  [skills](/docs/id/skills) yang dipanggil pengguna dan perintah kustom bekerja dalam mode `-p`: sertakan `/skill-name` dalam string prompt dan Claude Code memperluasnya sebelum menjalankan. Perintah bawaan yang hanya berjalan di antarmuka terminal, seperti `/login`, tidak tersedia dalam mode `-p`. `/model`, `/effort`, `/fast`, `/color`, dan `/rename` menerima nilai sebagai argumen, misalnya `/model sonnet`, dan `/mcp` tanpa argumen mencetak ringkasan teks status server; bentuk-bentuk ini memerlukan Claude Code v2.1.205 atau lebih baru dan mengikuti [catatan ketersediaan](/docs/id/commands#all-commands) setiap perintah. Untuk mengubah pengaturan dari invokasi `-p`, berikan `key=value` ke `/config`, misalnya `/config thinking=false`.
</Note>

<h3 id="customize-the-system-prompt">
  Sesuaikan prompt sistem
</h3>

Gunakan `--append-system-prompt` untuk menambahkan instruksi sambil mempertahankan perilaku default Claude Code. Contoh ini menyalurkan diff PR ke Claude dan menginstruksikannya untuk meninjau kerentanan keamanan:

```bash theme={null}
gh pr diff "$1" | claude -p \
  --append-system-prompt "You are a security engineer. Review for vulnerabilities." \
  --output-format json
```

Lihat [system prompt flags](/docs/id/cli-reference#system-prompt-flags) untuk opsi lebih lanjut termasuk `--system-prompt` untuk sepenuhnya mengganti prompt default.

<h3 id="continue-conversations">
  Lanjutkan percakapan
</h3>

Gunakan `--continue` untuk melanjutkan percakapan terbaru, atau `--resume` dengan ID sesi untuk melanjutkan percakapan tertentu. Contoh ini menjalankan tinjauan, kemudian mengirim prompt tindak lanjut:

```bash theme={null}
# First request
claude -p "Review this codebase for performance issues"

# Continue the most recent conversation
claude -p "Now focus on the database queries" --continue
claude -p "Generate a summary of all issues found" --continue
```

Jika Anda menjalankan beberapa percakapan, tangkap ID sesi untuk melanjutkan percakapan tertentu:

```bash theme={null}
session_id=$(claude -p "Start a review" --output-format json | jq -r '.session_id')
claude -p "Continue that review" --resume "$session_id"
```

Jalankan kedua perintah dari direktori yang sama: pencarian ID sesi dibatasi pada direktori proyek saat ini dan git worktrees-nya. Lihat [Resume a session](/docs/id/sessions#resume-a-session) untuk aturan cakupan lengkap.

<h2 id="next-steps">
  Langkah berikutnya
</h2>

* [Agent SDK quickstart](/docs/id/agent-sdk/quickstart): bangun agen pertama Anda dengan Python atau TypeScript
* [CLI reference](/docs/id/cli-reference): semua bendera dan opsi CLI
* [GitHub Actions](/docs/id/github-actions): gunakan Agent SDK dalam alur kerja GitHub
* [GitLab CI/CD](/docs/id/gitlab-ci-cd): gunakan Agent SDK dalam pipeline GitLab
