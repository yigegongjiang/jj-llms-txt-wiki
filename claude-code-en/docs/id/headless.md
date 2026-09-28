> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Jalankan Claude Code secara programatis

> Gunakan Agent SDK untuk menjalankan Claude Code secara programatis dari CLI, Python, atau TypeScript.

[Agent SDK](/docs/id/agent-sdk/overview) memberikan Anda alat yang sama, loop agen, dan manajemen konteks yang mendukung Claude Code. Tersedia sebagai CLI untuk skrip dan CI/CD, atau sebagai paket [Python](/docs/id/agent-sdk/python) dan [TypeScript](/docs/id/agent-sdk/typescript) untuk kontrol programatis penuh.

Untuk menjalankan Claude Code dalam mode non-interaktif, berikan `-p` dengan prompt Anda dan [opsi CLI](/docs/id/cli-reference) apa pun yang Anda butuhkan:

```bash theme={null}
claude -p "Find and fix the bug in auth.py" --allowedTools "Read,Edit,Bash"
```

Halaman ini mencakup penggunaan Agent SDK melalui CLI (`claude -p`). Untuk paket SDK Python dan TypeScript dengan output terstruktur, callback persetujuan alat, dan objek pesan asli, lihat [dokumentasi Agent SDK lengkap](/docs/id/agent-sdk/overview).

<h2 id="basic-usage">
  Penggunaan dasar
</h2>

Tambahkan bendera `-p` (atau `--print`) ke perintah `claude` apa pun untuk menjalankannya secara non-interaktif. Tidak semua [opsi CLI](/docs/id/cli-reference) menggabung dengan `-p`. Claude Code menolak `--bg`, dan menolak `--cloud` dengan deskripsi tugas, dengan kesalahan yang menyebutkan konflik; `--cloud` dengan ID sesi dan `-p` sebagai gantinya [antrian pesan ke sesi cloud tersebut](/docs/id/claude-code-on-the-web#send-follow-ups-from-the-cli) dan keluar. Opsi yang akan Anda gabungkan dengan `-p` sering kali mencakup:

* `--continue` untuk [melanjutkan percakapan](#continue-conversations)
* `--allowedTools` untuk [persetujuan otomatis alat](#auto-approve-tools)
* `--output-format` untuk [output terstruktur](#get-structured-output)

Contoh ini menanyakan Claude tentang basis kode Anda dan mencetak respons:

```bash theme={null}
claude -p "What does the auth module do?"
```

Claude Code keluar dengan kode 0 saat berhasil dan kode bukan nol ketika jalankan gagal, sehingga skrip Anda dapat bercabang pada status keluar. Jika Anda melewatkan bendera yang tidak valid, Claude Code melaporkan kesalahan ke stderr sebelum jalankan dimulai. Ketika kegagalan terjadi di dalam jalankan, seperti autentikasi yang hilang, Claude Code mencetak kegagalan sebagai hasil pada stdout.

<h3 id="start-faster-with-bare-mode">
  Mulai lebih cepat dengan bare mode
</h3>

Tambahkan `--bare` untuk mengurangi waktu startup dengan melewati penemuan otomatis hooks, skills, perintah kustom, [subagents](/docs/id/sub-agents), plugins, server MCP, auto memory, dan CLAUDE.md. Tanpanya, `claude -p` memuat [konteks](/docs/id/how-claude-code-works#the-context-window) yang sama dengan sesi interaktif, termasuk apa pun yang dikonfigurasi di direktori kerja atau `~/.claude`.

Bare mode berguna untuk CI dan skrip di mana Anda memerlukan hasil yang sama di setiap mesin. Hook di `~/.claude` rekan kerja atau server MCP di `.mcp.json` proyek tidak akan berjalan, karena bare mode tidak pernah membacanya. Direktori yang Anda beri nama dengan `--add-dir` adalah pengecualian parsial: bare mode memuat skills dari folder `.claude/skills/` nya, tetapi masih melewati folder `.claude/commands/` dan `.claude/agents/` nya. [Skills dari direktori tambahan](/docs/id/skills#skills-from-additional-directories) mencakup apa yang dimuat dan tidak dimuat.

Tanpa `--bare`, sesi `-p` menjalankan hooks di `.claude/settings.json` proyek dan menghubungkan server di `.mcp.json` nya, bahkan di folder yang belum pernah Anda percayai. Sesi `-p` tidak menampilkan dialog kepercayaan ruang kerja dan tidak ada prompt persetujuan per-server. [Apa yang berjalan sebelum Anda mempercayai folder](/docs/id/permissions#what-runs-before-you-trust-a-folder) mencakup setiap jenis konten repositori di bawah `-p` dan cara menjaganya tetap keluar.

Contoh ini menjalankan tugas ringkasan sekali pakai dalam bare mode dan pra-menyetujui alat Read sehingga panggilan selesai tanpa prompt izin. Atur `ANTHROPIC_API_KEY` sebelum menjalankannya, karena bare mode tidak menggunakan login langganan Anda:

```bash theme={null}
claude --bare -p "Summarize README.md" --allowedTools "Read"
```

Dalam bare mode, Claude Code tidak pernah membaca kredensial OAuth atau keychain sistem. Untuk Anthropic API, atur `ANTHROPIC_API_KEY` di lingkungan, dengan kunci yang dibuat di [Claude Console](https://platform.claude.com), atau berikan `apiKeyHelper` di JSON `--settings`. Amazon Bedrock, Google Cloud's Agent Platform, dan Microsoft Foundry terus membaca kredensial penyedia mereka sendiri seperti biasanya.

Dalam bare mode Claude memiliki akses ke alat Bash, pembacaan file, dan pengeditan file. Berikan konteks apa pun yang Anda butuhkan dengan bendera:

| Untuk memuat             | Gunakan                                                 |
| ------------------------ | ------------------------------------------------------- |
| Penambahan prompt sistem | `--append-system-prompt`, `--append-system-prompt-file` |
| Pengaturan               | `--settings <file-or-json>`                             |
| Server MCP               | `--mcp-config <file-or-json>`                           |
| Agen kustom              | `--agents <json>`                                       |
| Plugin                   | `--plugin-dir <path>`, `--plugin-url <url>`             |

<Note>
  `--bare` adalah mode yang direkomendasikan untuk panggilan skrip dan SDK, dan akan menjadi default untuk `-p` di rilis mendatang.
</Note>

<h3 id="background-tasks-at-exit">
  Tugas latar belakang saat keluar
</h3>

Jika Claude memulai [tugas Bash latar belakang](/docs/id/tools-reference#bash-tool-behavior) selama jalankan `claude -p`, misalnya server dev atau build watch, shell tersebut dihentikan sekitar lima detik setelah Claude mengembalikan hasil akhirnya dan stdin telah ditutup. Periode grace memungkinkan tugas yang selesai tepat setelah hasil masih memberikan outputnya.

Jika Claude memulai [subagent](/docs/id/sub-agents) latar belakang atau alur kerja, `claude -p` sebagai gantinya tetap terbuka sampai pekerjaan itu selesai, karena hasilnya adalah bagian dari output akhir.

Secara default tunggu berakhir setelah 10 menit menunggu idle berkelanjutan, jadi subagent atau alur kerja yang macet tidak dapat membuat proses tetap terbuka tanpa batas. Pada titik itu Claude Code menghentikan apa pun yang masih berjalan dan menjatuhkan hasil parsialnya. Untuk mengubah batas, atur [`CLAUDE_CODE_PRINT_BG_WAIT_CEILING_MS`](/docs/id/env-vars), atau atur ke `0` untuk menunggu tanpa batas.

Jika Claude memulai watch [Monitor](/docs/id/tools-reference#monitor-tool) selama jalankan `claude -p`, Claude Code menunggu watch sampai waktu habis atau batas sepuluh menit mengakhiri tunggu, mana pun yang terjadi lebih dulu. Saat menunggu, Claude terus merespons apa yang dilaporkan watch. Secara default, watch habis waktu lima menit setelah Claude memulainya.

<h3 id="stop-a-run-with-sigterm">
  Hentikan jalankan dengan SIGTERM
</h3>

Jika Anda menghentikan jalankan `claude -p` dengan SIGTERM, misalnya dengan `kill` atau dari pengawas proses, Claude Code keluar dengan kode 143. Claude Code meninggalkan giliran yang sedang berlangsung tidak selesai dan tidak mencatat hasil untuknya. Untuk mengakhiri giliran sebagai gantinya, kirim SIGINT, atau panggil `interrupt()` Agent SDK, sebelum Anda menghentikan proses.

Pada SIGTERM, Claude Code menghentikan pohon proses dari perintah Bash apa pun yang masih berjalan. Claude Code kemudian menjalankan [`SessionEnd` hooks](/docs/id/hooks#sessionend) dan keluar. Saat keluar, Claude Code tidak memulai panggilan alat baru, tidak mengirim permintaan model baru, dan tidak menjalankan hook selain `SessionEnd`. Jika jalankan berada di tengah perintah atau menunggu jawaban untuk prompt izin ketika sinyal tiba, Claude Code menangani langkah itu sebagai berikut:

* **Menjalankan perintah**: Claude Code mencatat perintah sebagai terbunuh dalam sesi.
* **Menunggu jawaban untuk prompt izin**: jika Anda mengirim SIGTERM ke proses, Claude Code meninggalkan prompt tanpa jawaban. Jika program Anda menutup sesi melalui Agent SDK, SDK mengakhiri input Claude Code sebelum mengirim sinyal apa pun, dan Claude Code membatalkan prompt segera setelah input berakhir.

Ketika Anda [melanjutkan sesi](#continue-conversations), Claude Code melanjutkan giliran yang SIGTERM tinggalkan tidak selesai.

<h2 id="examples">
  Contoh
</h2>

Contoh-contoh ini menyoroti pola CLI umum. Jika perintah menamai file seperti `auth.py` atau `build-error.txt`, gantikan dengan file dari proyek Anda sendiri. Dalam CI atau lingkungan skrip lainnya, tambahkan [`--bare`](#start-faster-with-bare-mode) sehingga Claude Code dimulai tanpa memuat hooks, plugins, auto memory, atau `CLAUDE.md` host.

<h3 id="pipe-data-through-claude">
  Saluran data melalui Claude
</h3>

Mode non-interaktif membaca stdin, sehingga Anda dapat menyalurkan data dan mengarahkan respons keluar seperti alat baris perintah lainnya.

Contoh ini menyalurkan log build ke Claude dan menulis penjelasan ke file:

```bash theme={null}
cat build-error.txt | claude -p 'concisely explain the root cause of this build error' > output.txt
```

Dengan `--output-format json`, payload respons mencakup `total_cost_usd` dan rincian biaya per-model, sehingga pemanggil skrip dapat melacak pengeluaran tanpa berkonsultasi dengan [dashboard penggunaan](/docs/id/costs). Ketika Anda melanjutkan percakapan sebelumnya dengan `--continue` atau `--resume`, jalankan melaporkan total keseluruhan percakapan, [pengeluaran jalankan sebelumnya disertakan](/docs/id/agent-sdk/cost-tracking#accumulate-costs-across-multiple-calls). Kedua angka tersebut adalah [perkiraan sisi klien](/docs/id/agent-sdk/cost-tracking) dan dapat berbeda dari tagihan aktual Anda.

<Note>
  Stdin yang disalurkan dibatasi pada 10MB. Jika Anda melampaui batas, Claude Code keluar dengan kesalahan yang jelas dan status bukan nol. Untuk bekerja dengan input yang lebih besar, tulis konten ke file dan referensikan jalur file dalam prompt Anda alih-alih menyalurkannya.
</Note>

Jika Claude Code tidak dapat membaca stdin, misalnya karena proses yang memulainya memutuskan ujungnya, Claude Code mencetak peringatan ke stderr dan melanjutkan dengan prompt dari baris perintah. Sebelum v2.1.211, stdin yang tidak dapat dibaca di Windows menghancurkan sesi atau membuatnya keluar diam-diam tanpa output.

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

Jalankan dengan `npm run lint:claude`.

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
  Gunakan alat seperti [jq](https://jqlang.org/) untuk mengurai respons dan mengekstrak bidang tertentu:

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
  Stream respons
</h3>

Gunakan `--output-format stream-json` dengan `--verbose` dan `--include-partial-messages` untuk menerima token saat dihasilkan. Setiap baris adalah objek JSON yang mewakili acara:

```bash theme={null}
claude -p "Explain recursion" --output-format stream-json --verbose --include-partial-messages
```

Baris terakhir dari aliran adalah pesan `result` dengan teks respons akhir, biaya, dan metadata sesi.

Jika konsumen Anda membaca aliran dengan lambat, Claude Code menunggu output yang antri untuk mengalir sebelum keluar, menskalakan tunggu dengan berapa banyak yang masih antri, dibatasi pada 30 detik. Sebelum v2.1.214 tunggu keluar dibatasi pada sekitar dua detik, yang dapat memotong akhir respons besar.

Contoh berikut menggunakan [jq](https://jqlang.org/) untuk memfilter delta teks dan menampilkan hanya teks streaming. Bendera `-r` menampilkan string mentah (tanpa tanda kutip) dan `-j` bergabung tanpa baris baru sehingga token streaming terus menerus:

```bash theme={null}
claude -p "Write a poem" --output-format stream-json --verbose --include-partial-messages | \
  jq -rj 'select(.type == "stream_event" and .event.delta.type? == "text_delta") | .event.delta.text'
```

Untuk streaming programatis dengan callback dan objek pesan, lihat [Stream responses in real-time](/docs/id/agent-sdk/streaming-output) dalam dokumentasi Agent SDK.

<h4 id="follow-subagent-messages">
  Ikuti pesan subagent
</h4>

Pesan dari [subagents](/docs/id/sub-agents) muncul dalam aliran sebagai pesan `assistant` dan `user` yang bidang `parent_tool_use_id` adalah ID dari tool call yang menelurkan subagent. Pesan dari percakapan utama membawa `null` di bidang itu.

Pesan pertama dari subagent yang berjalan dalam [foreground](/docs/id/sub-agents#run-subagents-in-foreground-or-background) adalah pesan `user` yang membawa prompt yang mendorong subagent. Setelah pesan pertama itu, Claude Code memancarkan:

* **Secara default**: blok `tool_use` dan `tool_result` subagent.
* **Dengan [`--forward-subagent-text`](/docs/id/cli-reference#cli-flags) atau [`CLAUDE_CODE_FORWARD_SUBAGENT_TEXT`](/docs/id/env-vars)**: blok teks dan thinking subagent juga, sehingga Anda dapat merekonstruksi transkrip setiap subagent. Ini memerlukan Claude Code v2.1.211 atau lebih baru.

Ketika Anda mengaktifkan salah satu opsi, Claude Code meneruskan pesan dari [subagents di setiap kedalaman nesting](/docs/id/sub-agents#let-subagents-spawn-their-own-subagents), baik setiap satu dispawn dengan Agent tool atau dimulai sebagai [forked skill](/docs/id/skills#run-skills-in-a-subagent). Pesan subagent yang forked skill spawn, dan forked skill yang dimulai di dalam subagent atau forked skill lainnya, memerlukan Claude Code v2.1.275 atau lebih baru. Di `parent_tool_use_id`, pesan subagent bersarang membawa ID dari Agent atau Skill tool call yang memulainya, sehingga Anda dapat membangun kembali pohon nesting penuh dengan mengikuti ID tersebut. Sebelum v2.1.219, pesan dari subagent bersarang tidak muncul dalam aliran.

Skills yang [berjalan dalam subagent](/docs/id/skills#run-skills-in-a-subagent) muncul dalam aliran dengan cara yang sama: pesan pertama skill yang di-fork adalah pesan `user` yang membawa konten skill yang mendorong jalankan. Jika Anda mengaktifkan salah satu opsi, aliran juga membawa blok teks dan thinking skill yang di-fork. Sebelum v2.1.265, hanya blok `tool_use` dan `tool_result` skill yang di-fork yang muncul dalam aliran.

<h4 id="handle-api-retries">
  Tangani percobaan ulang API
</h4>

Ketika permintaan API gagal dengan kesalahan yang dapat dicoba ulang, Claude Code memancarkan acara `system/api_retry` sebelum mencoba ulang. Pada v2.1.246 atau lebih baru, ketika `401` atau `403` menolak kredensial [`apiKeyHelper`](/docs/id/settings-reference#apikeyhelper), Claude Code membuat dua percobaan ulang pertama diam-diam tanpa acara, kemudian memancarkan acara seperti biasa dari percobaan ulang berturut-turut ketiga dan seterusnya. Percobaan ulang diam-diam masih dihitung menuju `attempt`. Anda dapat menggunakan acara untuk menampilkan kemajuan percobaan ulang di antarmuka Anda sendiri.

| Bidang           | Tipe              | Deskripsi                                                                                                                                                                                                                                                                                                                                                                                                                               |
| ---------------- | ----------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`           | `"system"`        | tipe pesan                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `subtype`        | `"api_retry"`     | mengidentifikasi ini sebagai acara percobaan ulang                                                                                                                                                                                                                                                                                                                                                                                      |
| `attempt`        | integer           | nomor percobaan saat ini, dimulai dari 1                                                                                                                                                                                                                                                                                                                                                                                                |
| `max_retries`    | integer           | total percobaan ulang yang diizinkan untuk penyebab kegagalan ini, yang dapat lebih sedikit dari anggaran seluruh sesi                                                                                                                                                                                                                                                                                                                  |
| `retry_delay_ms` | integer           | milidetik hingga percobaan berikutnya                                                                                                                                                                                                                                                                                                                                                                                                   |
| `error_status`   | integer atau null | kode status HTTP dari percobaan yang gagal, atau `null` ketika percobaan tidak mendapat respons HTTP dari API                                                                                                                                                                                                                                                                                                                           |
| `no_response`    | object, opsional  | hadir hanya ketika percobaan yang gagal mendapat [tidak ada header respons tepat waktu](/docs/id/errors#no-response-from-api). `waited_ms` adalah berapa lama percobaan itu menunggu dan `retry_wait_ms` adalah berapa lama percobaan ulang akan menunggu. Dalam acara ini, `max_retries` mencerminkan satu percobaan ulang yang biasanya didapat penyebab ini, bukan anggaran seluruh sesi. Memerlukan Claude Code v2.1.261 atau lebih baru |
| `error`          | string            | kategori kesalahan: `authentication_failed`, `oauth_org_not_allowed`, `account_on_hold`, `billing_error`, `rate_limit`, `overloaded`, `invalid_request`, `model_not_found`, `server_error`, `max_output_tokens`, `cloud_credential_error`, atau `unknown`                                                                                                                                                                               |
| `uuid`           | string            | pengidentifikasi acara unik                                                                                                                                                                                                                                                                                                                                                                                                             |
| `session_id`     | string            | sesi yang dimiliki acara                                                                                                                                                                                                                                                                                                                                                                                                                |

<h4 id="read-session-metadata">
  Baca metadata sesi
</h4>

Acara `system/init` melaporkan metadata sesi termasuk model, alat, server MCP, dan plugin yang dimuat. Ini adalah acara pertama dalam aliran kecuali acara startup mendahuluinya:

* Acara `plugin_install`, ketika [`CLAUDE_CODE_SYNC_PLUGIN_INSTALL`](/docs/id/env-vars) diatur.
* [Acara `hook_started`, `hook_progress`, dan `hook_response`](/docs/id/agent-sdk/typescript#sdkhookstartedmessage), saat hook [`SessionStart`](/docs/id/hooks#sessionstart) atau [`Setup`](/docs/id/hooks#setup) yang dikonfigurasi berjalan. Ini streaming saat hook menghasilkannya. Claude Code v2.1.169 hingga v2.1.203 mengirimkannya dalam satu batch setelah hook selesai, masih sebelum `system/init`; v2.1.204 mengembalikan pengiriman langsung.

Acara ini juga membawa array `capabilities` opsional dari string yang menamai perilaku protokol yang diimplementasikan versi Claude Code ini, seperti `interrupt_receipt_v1` atau `interrupt_cancel_queued_v1`. Periksanya untuk mendeteksi fitur alih-alih membandingkan string versi, dan abaikan nilai yang tidak Anda kenali. Bidang ini memerlukan Claude Code v2.1.205 atau lebih baru dan tidak ada di versi sebelumnya. Lihat [`SDKSystemMessage`](/docs/id/agent-sdk/typescript#sdksystemmessage) untuk daftar kemampuan.

<h4 id="fail-ci-when-a-plugin-or-mcp-server-doesn’t-load">
  Gagalkan CI ketika plugin atau server MCP tidak dimuat
</h4>

Gunakan bidang plugin dalam acara `system/init` untuk menangkap plugin yang tidak dimuat:

| Bidang          | Tipe  | Deskripsi                                                                                                                                                                                                                                                                                                                              |
| --------------- | ----- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `plugins`       | array | plugin yang berhasil dimuat, masing-masing dengan `name` dan `path`                                                                                                                                                                                                                                                                    |
| `plugin_errors` | array | kesalahan waktu muat plugin, masing-masing dengan `plugin`, `type`, dan `message`. Mencakup versi dependensi yang tidak terpenuhi dan kegagalan muat `--plugin-dir` seperti jalur yang hilang atau arsip yang tidak valid. Plugin yang terpengaruh diturunkan dan tidak ada di `plugins`. Kunci dihilangkan ketika tidak ada kesalahan |

Gunakan bidang server MCP dengan cara yang sama. Ketika Anda berikan [`--mcp-config`](/docs/id/cli-reference#cli-flags) dengan `-p`, Claude Code menunggu server yang masih tertunda sebelum menjalankan giliran pertama, hingga timeout startup [`MCP_TIMEOUT`](/docs/id/env-vars), 30 detik secara default. Server jarak jauh dengan [daftar alat yang di-cache](/docs/id/agent-sdk/mcp#connection-timing) melewati tunggu, menampilkan `pending` di `system/init`, dan terhubung pada pemanggilan alat pertamanya. Tunggu memerlukan Claude Code v2.1.221 atau lebih baru.

Claude Code memvalidasi setiap entri `--mcp-config` pada startup dan melewati entri yang gagal validasi, misalnya entri `url` tanpa `type`. Jalankan berlanjut dan keluar dengan bersih, jadi periksa bidang ini untuk menangkap server yang tidak pernah dimuat:

| Bidang              | Tipe  | Deskripsi                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| ------------------- | ----- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `mcp_servers`       | array | server MCP dalam sesi, masing-masing dengan `name` dan `status`                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `mcp_server_errors` | array | entri `--mcp-config` yang dilewati oleh validasi konfigurasi, masing-masing dengan `name`, `type`, dan `message`. `type` adalah kategori lewati seperti `unknown_type`, `url_missing_type`, `invalid_config`, atau `reserved_name`; perlakukan nilai yang tidak Anda kenali sebagai lewati generik. Server yang terpengaruh tidak ada di `mcp_servers`. Kunci dihilangkan ketika tidak ada kesalahan, jadi gerbang CI dapat gagal pada array yang tidak kosong. Memerlukan Claude Code v2.1.219 atau lebih baru |

Ketika Anda menjalankan perintah dengan tangan di terminal, Claude Code juga mencetak peringatan startup ke stderr, seperti `Warning: 1 MCP server skipped due to invalid config:`, diikuti oleh alasan untuk setiap entri yang dilewati. Ketika Anda mengarahkan stderr, atau ketika program seperti runner CI atau host SDK menangkapnya, Claude Code tidak mencetak peringatan dan melaporkan entri yang dilewati hanya di bidang `mcp_server_errors`. Peringatan memerlukan Claude Code v2.1.219 atau lebih baru.

<h4 id="track-plugin-installs">
  Lacak pemasangan plugin
</h4>

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

<h3 id="auto-approve-tools">
  Persetujuan otomatis alat
</h3>

Gunakan `--allowedTools` untuk membiarkan Claude menggunakan alat tertentu tanpa meminta. Contoh ini menjalankan suite pengujian dan memperbaiki kegagalan, memungkinkan Claude untuk menjalankan perintah Bash dan membaca/mengedit file tanpa meminta izin:

```bash theme={null}
claude -p "Run the test suite and fix any failures" \
  --allowedTools "Bash,Read,Edit"
```

Untuk menetapkan baseline untuk seluruh sesi alih-alih mencantumkan alat individual, berikan [mode izin](/docs/id/permission-modes). Untuk `-p`, [mode izin awal bawaan](/docs/id/permission-modes#which-mode-a-session-starts-in) adalah Manual di setiap paket, jadi berikan mode izin yang Anda inginkan:

* **`auto`**: berikan `--permission-mode auto` untuk memiliki pengklasifikasi meninjau sebagian besar tindakan alih-alih Anda
* **`dontAsk`**: Claude Code menolak setiap panggilan yang akan meminta sebaliknya, yang berguna untuk CI runs yang terkunci. Tindakan yang tidak memerlukan persetujuan dalam mode Manual masih berjalan, seperti pembacaan file di direktori kerja Anda dan [set perintah read-only](/docs/id/permissions#read-only-commands), dan begitu juga tindakan yang entri `--allowedTools` Anda atau aturan `permissions.allow` cover. `AskUserQuestion`, alat konektor [organisasi Anda atur ke `ask`](/docs/id/mcp#organization-controls-on-connector-tools), dan alat MCP yang ditandai [`requiresUserInteraction`](/docs/id/mcp#require-approval-for-a-specific-tool) ditolak bahkan ketika aturan allow cocok
* **`acceptEdits`**: Claude menulis file tanpa meminta, dan Claude Code auto-approves perintah filesystem umum seperti `mkdir`, `touch`, `mv`, dan `cp`. [Tindakan yang tidak ada mode auto-approve](/docs/id/permission-modes#actions-no-mode-auto-approves) masih berlaku. Terlepas dari set perintah read-only, perintah shell lainnya dan permintaan jaringan masih memerlukan entri `--allowedTools` atau aturan `permissions.allow`. Lihat [apa yang `acceptEdits` auto-approve](/docs/id/permission-modes#auto-approve-file-edits-with-acceptedits-mode) untuk daftar lengkap

Contoh ini menerapkan perbaikan lint dengan `acceptEdits` sebagai baseline:

```bash theme={null}
claude -p "Apply the lint fixes" --permission-mode acceptEdits
```

<h3 id="turn-off-permission-prompts-in-unattended-runs">
  Matikan prompt izin dalam jalankan tanpa pengawasan
</h3>

Berikan `--permission-prompts none` ketika tidak ada yang tersedia untuk menjawab prompt izin, misalnya dalam pekerjaan terjadwal. Bendera paling penting ketika jalankan Anda memiliki host izin: aplikasi Agent SDK dengan callback [`canUseTool`](/docs/id/agent-sdk/user-input), atau alat MCP yang Anda berikan dengan [`--permission-prompt-tool`](/docs/id/cli-reference#cli-flags). Tanpa bendera, jalankan Anda menunggu host itu menjawab setiap permintaan izin.

Dengan bendera, jalankan Anda tidak berkonsultasi dengan host atau menunggu itu. Apa pun yang akan meminta ditolak kecuali hook `PermissionRequest` mengizinkannya, Claude diberitahu bahwa tidak ada yang dapat menyetujui permintaan dan tidak mencoba ulang, dan jalankan berlanjut. Dalam jalankan `-p` tanpa host, permintaan ini ditolak baik cara, dan bendera juga memberitahu Claude tidak mencoba ulang mereka. Aturan izin, [hook `PermissionRequest`](/docs/id/hooks#permissionrequest), dan mode izin yang Anda atur masih memutuskan setiap panggilan terlebih dahulu; Claude Code hanya menolak permintaan yang tidak ada yang lain selesaikan.

Contoh ini menjalankan tugas tanpa pengawasan dalam [mode auto](/docs/id/permission-modes#eliminate-prompts-with-auto-mode). Pengklasifikasi meninjau setiap tindakan seperti biasa, dan Claude Code menolak apa pun yang akan jatuh kembali ke prompt:

```bash theme={null}
claude -p "Update the dependency pins and run the tests" --permission-mode auto --permission-prompts none
```

Dengan `--permission-prompts none`, Claude Code menghapus alat yang memerlukan jawaban dari orang, seperti [`AskUserQuestion`](/docs/id/tools-reference#askuserquestion-tool-behavior), sehingga Claude tidak dapat memanggilnya. Setiap [permintaan elicitasi MCP](/docs/id/mcp#respond-to-mcp-elicitation-requests) yang tidak ada hook [`Elicitation`](/docs/id/hooks#elicitation) jawab dibatalkan.

Dengan `--output-format stream-json`, penolakan muncul sebagai pesan sistem `permission_denied`, dan pesan hasil akhir mencantumnya di `permission_denials`.

<Note>
  Bendera `--permission-prompts` memerlukan Claude Code v2.1.259 atau lebih baru. Versi sebelumnya menolaknya dengan kesalahan opsi tidak dikenal.
</Note>

<h3 id="create-a-commit">
  Buat komit
</h3>

Contoh ini meninjau perubahan yang dipentaskan dan membuat komit dengan pesan yang sesuai:

```bash theme={null}
claude -p "Look at my staged changes and create an appropriate commit" \
  --allowedTools "Bash(git diff *),Bash(git log *),Bash(git status *),Bash(git commit *)"
```

Bendera `--allowedTools` menggunakan [sintaks aturan izin](/docs/id/settings-reference#permission-rule-syntax). Spasi di akhir ` *` memungkinkan pencocokan awalan, jadi `Bash(git diff *)` memungkinkan perintah apa pun yang dimulai dengan `git diff`. Spasi sebelum `*` penting: tanpanya, `Bash(git diff*)` juga akan cocok dengan `git diff-index`.

<Note>
  Dukungan perintah berbeda dalam mode `-p`:

  * [skills](/docs/id/skills) yang dipanggil pengguna dan perintah kustom bekerja. Sertakan `/skill-name` dalam string prompt dan Claude Code memperluasnya sebelum menjalankan.
  * Perintah bawaan yang hanya berjalan di antarmuka terminal, seperti `/login`, tidak tersedia.
  * `/model`, `/effort`, `/fast`, `/color`, dan `/rename` menerima nilai sebagai argumen, misalnya `/model sonnet`, dan `/mcp` tanpa argumen mencetak ringkasan teks status server. Bentuk-bentuk ini memerlukan Claude Code v2.1.205 atau lebih baru dan mengikuti [catatan ketersediaan](/docs/id/commands#all-commands) setiap perintah.
  * Untuk mengubah pengaturan, berikan `key=value` ke `/config`, misalnya `/config thinking=false`.
  * `/output-style <style>` beralih [output styles](/docs/id/output-styles) dan `/output-style` saja mencantumnya. Memerlukan Claude Code v2.1.269 atau lebih baru.
</Note>

<h3 id="customize-the-system-prompt">
  Sesuaikan prompt sistem
</h3>

Gunakan `--append-system-prompt` untuk menambahkan instruksi sambil mempertahankan perilaku default Claude Code. Contoh ini menyalurkan diff PR ke Claude dan menginstruksikannya untuk meninjau kerentanan keamanan. Simpan sebagai skrip shell, misalnya `review.sh`:

```bash theme={null}
gh pr diff "$1" | claude -p \
  --append-system-prompt "You are a security engineer. Review for vulnerabilities." \
  --output-format json
```

Dalam skrip, `"$1"` berdiri untuk argumen pertama yang Anda berikan di baris perintah. Jalankan `bash review.sh 123` dan shell mengganti `"$1"` dengan `123`, jadi skrip mengambil diff untuk PR 123. Claude Code mencetak tinjauan sebagai JSON, dengan teks di bidang `result`.

Lihat [system prompt flags](/docs/id/cli-reference#system-prompt-flags) untuk opsi lebih lanjut termasuk `--system-prompt` untuk sepenuhnya mengganti prompt default.

<h3 id="continue-conversations">
  Lanjutkan percakapan
</h3>

Gunakan `--continue` untuk melanjutkan percakapan terbaru, atau `--resume` dengan ID sesi untuk melanjutkan percakapan tertentu. Pada Claude Code v2.1.257 atau lebih baru, ketika Anda berikan `--continue`, Claude Code membuka [sesi latar belakang](/docs/id/sessions#resume-a-session) yang telah selesai, tetapi bukan yang masih berjalan. Contoh ini menjalankan tinjauan, kemudian mengirim prompt tindak lanjut:

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

Anda dapat menjalankan dua perintah dari direktori yang berbeda: Claude Code [menemukan sesi berdasarkan ID-nya](/docs/id/sessions#resume-a-session) di proyek apa pun di mesin ini. Sebelum v2.1.223, Claude Code mencari ID hanya di direktori proyek saat ini dan git worktrees-nya, jadi Anda harus menjalankan kedua perintah dari direktori yang sama.

Sebagai ganti ID sesi, Anda dapat memberikan `--resume` jalur absolut ke file [transkrip](/docs/id/sessions#where-transcripts-are-stored) `.jsonl` sesi, dan Claude Code melanjutkan percakapan yang disimpan dalam file itu.

<h2 id="next-steps">
  Langkah berikutnya
</h2>

* [Agent SDK quickstart](/docs/id/agent-sdk/quickstart): bangun agen pertama Anda dengan Python atau TypeScript
* [CLI reference](/docs/id/cli-reference): semua bendera dan opsi CLI
* [GitHub Actions](/docs/id/github-actions): gunakan Agent SDK dalam alur kerja GitHub
* [GitLab CI/CD](/docs/id/gitlab-ci-cd): gunakan Agent SDK dalam pipeline GitLab
