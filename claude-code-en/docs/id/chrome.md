> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Gunakan Claude Code dengan Chrome

> Hubungkan Claude Code ke browser Chrome Anda untuk menguji aplikasi web, debug dengan console logs, otomatisasi pengisian formulir, dan ekstrak data dari halaman web.

Claude Code terintegrasi dengan [ekstensi Claude in Chrome browser](https://chromewebstore.google.com/detail/claude/fcoeoabgfenejglbffodgkkbkcdhcgfn) untuk memberikan Anda kemampuan otomasi browser dari CLI atau [ekstensi VS Code](/docs/id/vs-code#automate-browser-tasks-with-chrome). Bangun kode Anda, kemudian uji dan debug di browser tanpa beralih konteks.

Claude membuka tab baru untuk tugas browser dan berbagi status login browser Anda, sehingga dapat mengakses situs apa pun yang sudah Anda masuki. Tindakan browser berjalan di jendela Chrome yang terlihat secara real-time. Ketika Claude menemukan halaman login atau CAPTCHA, ia berhenti dan meminta Anda menanganinya secara manual.

<Note>
  Integrasi Chrome bekerja dengan Google Chrome dan Microsoft Edge. Belum didukung di Brave, Arc, atau browser berbasis Chromium lainnya. Juga tidak didukung di Windows Subsystem for Linux (WSL).
</Note>

<h2 id="capabilities">
  Kemampuan
</h2>

Dengan Chrome terhubung, Anda dapat menggabungkan tindakan browser dengan tugas coding dalam satu alur kerja:

* **Live debugging**: baca kesalahan console dan status DOM secara langsung, kemudian perbaiki kode yang menyebabkannya
* **Verifikasi desain**: bangun UI dari mock Figma, kemudian buka di browser untuk memverifikasi kecocokannya
* **Pengujian aplikasi web**: uji validasi formulir, periksa regresi visual, atau verifikasi alur pengguna
* **Aplikasi web terautentikasi**: berinteraksi dengan Google Docs, Gmail, Notion, atau aplikasi apa pun yang Anda masuki tanpa konektor API
* **Ekstraksi data**: tarik informasi terstruktur dari halaman web dan simpan secara lokal
* **Otomasi tugas**: otomatisasi tugas browser berulang seperti entri data, pengisian formulir, atau alur kerja multi-situs
* **Perekaman sesi**: rekam interaksi browser sebagai GIF untuk mendokumentasikan atau berbagi apa yang terjadi

<h2 id="prerequisites">
  Prasyarat
</h2>

Sebelum menggunakan Claude Code dengan Chrome, Anda memerlukan:

* Browser [Google Chrome](https://www.google.com/chrome/) atau [Microsoft Edge](https://www.microsoft.com/edge)
* Ekstensi [Claude in Chrome](https://chromewebstore.google.com/detail/claude/fcoeoabgfenejglbffodgkkbkcdhcgfn) versi 1.0.36 atau lebih tinggi, tersedia di Chrome Web Store untuk kedua browser
* [Claude Code](/docs/id/quickstart#step-1-install-claude-code)
* Paket Anthropic langsung (Pro, Max, Team, atau Enterprise)

<Note>
  Integrasi Chrome tidak tersedia melalui penyedia pihak ketiga seperti Amazon Bedrock, Google Cloud's Agent Platform, atau Microsoft Foundry. Jika Anda mengakses Claude secara eksklusif melalui penyedia pihak ketiga, Anda memerlukan akun claude.ai terpisah untuk menggunakan fitur ini.
</Note>

<h2 id="get-started-in-the-cli">
  Mulai di CLI
</h2>

<Steps>
  <Step title="Luncurkan Claude Code dengan Chrome">
    Mulai Claude Code dengan flag `--chrome`:

    ```bash theme={null}
    claude --chrome
    ```

    Anda juga dapat mengaktifkan Chrome dari dalam sesi yang ada dengan menjalankan `/chrome`.
  </Step>

  <Step title="Minta Claude menggunakan browser">
    Contoh ini menavigasi ke halaman, berinteraksi dengannya, dan melaporkan apa yang ditemukannya, semuanya dari terminal atau editor Anda:

    ```text theme={null}
    Go to code.claude.com/docs, click on the search box,
    type "hooks", and tell me what results appear
    ```

    Tindakan browser pertama meminta izin untuk menggunakan skill `claude-in-chrome`. Setujui dan Claude membuka tab baru dan memulai tugas.
  </Step>
</Steps>

Jalankan `/chrome` kapan saja untuk memeriksa status koneksi, mengelola izin, menghubungkan kembali ekstensi, atau memilih browser yang terhubung mana yang akan digunakan. Jika lebih dari satu browser terhubung saat tindakan browser dimulai, Claude akan meminta Anda untuk memilih salah satu.

Untuk VS Code, lihat [otomasi browser di VS Code](/docs/id/vs-code#automate-browser-tasks-with-chrome).

<h3 id="enable-chrome-by-default">
  Aktifkan Chrome secara default
</h3>

Untuk menghindari melewatkan `--chrome` setiap sesi, jalankan `/chrome` dan pilih "Enabled by default".

Di [ekstensi VS Code](/docs/id/vs-code#automate-browser-tasks-with-chrome), Chrome tersedia kapan pun ekstensi Chrome diinstal. Tidak ada flag tambahan yang diperlukan.

<Note>
  Mengaktifkan Chrome secara default di CLI meningkatkan penggunaan konteks karena alat browser selalu dimuat. Jika Anda melihat peningkatan konsumsi konteks, nonaktifkan pengaturan ini dan gunakan `--chrome` hanya saat diperlukan.
</Note>

<h3 id="manage-site-permissions">
  Kelola izin situs
</h3>

Izin tingkat situs diwarisi dari ekstensi Chrome. Kelola izin di pengaturan ekstensi Chrome untuk mengontrol situs mana yang dapat dijelajahi, diklik, dan diketik oleh Claude.

<h3 id="browser-tools-in-plan-mode">
  Alat browser dalam plan mode
</h3>

Dalam [plan mode](/docs/id/permission-modes#analyze-before-you-edit-with-plan-mode), panggilan alat browser yang hanya membaca halaman atau status browser berjalan tanpa permintaan izin, dan panggilan yang mengubah status meminta persetujuan.

* **Panggilan hanya-baca**: `read_page`, `get_page_text`, `find`, membaca pesan konsol atau permintaan jaringan, dan mengambil tangkapan layar
* **Panggilan yang mengubah status**: klik, pengetikan, navigasi, manajemen tab dan jendela, dan merekam GIF

Mulai dari v2.1.199, panggilan yang sebaliknya hanya-baca yang menetapkan flag input yang mengubah status, seperti `createIfEmpty` pada `tabs_context_mcp`, `clear` pada pembaca konsol dan jaringan, atau `save_to_disk` pada tangkapan layar, juga meminta persetujuan. Panggilan `browser_batch` berjalan tanpa permintaan hanya ketika setiap tindakan di dalamnya adalah hanya-baca.

<h2 id="example-workflows">
  Contoh alur kerja
</h2>

Contoh-contoh ini menunjukkan cara umum untuk menggabungkan tindakan browser dengan tugas coding. Jalankan `/mcp`, pilih `claude-in-chrome`, kemudian pilih **View tools** untuk melihat daftar lengkap alat browser yang tersedia.

<h3 id="test-a-local-web-application">
  Uji aplikasi web lokal
</h3>

Saat mengembangkan aplikasi web, minta Claude untuk memverifikasi perubahan Anda berfungsi dengan benar:

```text theme={null}
I just updated the login form validation. Can you open localhost:3000,
try submitting the form with invalid data, and check if the error
messages appear correctly?
```

Claude menavigasi ke server lokal Anda, berinteraksi dengan formulir, dan melaporkan apa yang diamatinya.

<h3 id="debug-with-console-logs">
  Debug dengan console logs
</h3>

Claude dapat membaca output console untuk membantu mendiagnosis masalah. Beri tahu Claude pola apa yang harus dicari daripada meminta semua output console, karena log dapat sangat panjang:

```text theme={null}
Open the dashboard page and check the console for any errors when
the page loads.
```

Claude membaca pesan console dan dapat memfilter pola atau jenis kesalahan tertentu.

<h3 id="automate-form-filling">
  Otomatisasi pengisian formulir
</h3>

Percepat tugas entri data berulang:

```text theme={null}
I have a spreadsheet of customer contacts in contacts.csv. For each row,
go to the CRM at crm.example.com, click "Add Contact", and fill in the
name, email, and phone fields.
```

Claude membaca file lokal Anda, menavigasi antarmuka web, dan memasukkan data untuk setiap catatan.

<h3 id="draft-content-in-google-docs">
  Buat draf konten di Google Docs
</h3>

Gunakan Claude untuk menulis langsung di dokumen Anda tanpa penyiapan API:

```text theme={null}
Draft a project update based on the recent commits and add it to my
Google Doc at docs.google.com/document/d/abc123
```

Claude membuka dokumen, mengklik ke editor, dan mengetik konten. Ini bekerja dengan aplikasi web apa pun yang Anda masuki: Gmail, Notion, Sheets, dan lainnya.

<h3 id="extract-data-from-web-pages">
  Ekstrak data dari halaman web
</h3>

Tarik informasi terstruktur dari situs web:

```text theme={null}
Go to the product listings page and extract the name, price, and
availability for each item. Save the results as a CSV file.
```

Claude menavigasi ke halaman, membaca konten, dan mengompilasi data ke dalam format terstruktur.

<h3 id="run-multi-site-workflows">
  Jalankan alur kerja multi-situs
</h3>

Koordinasikan tugas di berbagai situs web:

```text theme={null}
Check my calendar for meetings tomorrow, then for each meeting with
an external attendee, look up their company website and add a note
about what they do.
```

Claude bekerja di seluruh tab untuk mengumpulkan informasi dan menyelesaikan alur kerja.

<h3 id="record-a-demo-gif">
  Rekam GIF demo
</h3>

Buat rekaman alur interaksi browser yang dapat dibagikan:

```text theme={null}
Record a GIF showing how to complete the checkout flow, from adding
an item to the cart through to the confirmation page.
```

Claude merekam urutan interaksi dan menyimpannya sebagai file GIF.

<h2 id="troubleshooting">
  Troubleshooting
</h2>

<h3 id="extension-not-detected">
  Ekstensi tidak terdeteksi
</h3>

Jika Claude Code tidak dapat mendeteksi ekstensi Chrome:

1. Verifikasi ekstensi Chrome diinstal dan diaktifkan di `chrome://extensions`
2. Verifikasi Claude Code terbaru dengan menjalankan `claude --version`
3. Periksa bahwa Chrome sedang berjalan
4. Jalankan `/chrome` dan pilih "Reconnect extension" untuk membangun kembali koneksi
5. Jika masalah berlanjut, restart Claude Code dan Chrome

Pertama kali Anda mengaktifkan integrasi Chrome, Claude Code menginstal file konfigurasi host messaging asli. Chrome membaca file ini saat startup, jadi jika ekstensi tidak terdeteksi pada upaya pertama Anda, restart Chrome untuk mengambil konfigurasi baru.

Mulai dari v2.1.199, Claude Code membuka tab browser yang meminta Anda untuk menghubungkan ekstensi hanya pada instalasi pertama itu. Sesi-sesi kemudian yang menulis ulang file konfigurasi, misalnya setelah beralih build Claude Code atau direktori konfigurasi, tidak membukanya kembali.

Jika koneksi masih gagal, verifikasi file konfigurasi host ada di:

Untuk Chrome:

* **macOS**: `~/Library/Application Support/Google/Chrome/NativeMessagingHosts/com.anthropic.claude_code_browser_extension.json`
* **Linux**: `~/.config/google-chrome/NativeMessagingHosts/com.anthropic.claude_code_browser_extension.json`
* **Windows**: periksa `HKCU\Software\Google\Chrome\NativeMessagingHosts\` di Windows Registry

Untuk Edge:

* **macOS**: `~/Library/Application Support/Microsoft Edge/NativeMessagingHosts/com.anthropic.claude_code_browser_extension.json`
* **Linux**: `~/.config/microsoft-edge/NativeMessagingHosts/com.anthropic.claude_code_browser_extension.json`
* **Windows**: periksa `HKCU\Software\Microsoft\Edge\NativeMessagingHosts\` di Windows Registry

<h3 id="browser-not-responding">
  Browser tidak merespons
</h3>

Jika perintah browser Claude berhenti bekerja:

1. Periksa apakah dialog modal (alert, confirm, prompt) memblokir halaman. Dialog JavaScript memblokir peristiwa browser dan mencegah Claude menerima perintah. Tutup dialog secara manual, kemudian beri tahu Claude untuk melanjutkan.
2. Minta Claude membuat tab baru dan coba lagi
3. Restart ekstensi Chrome dengan menonaktifkan dan mengaktifkannya kembali di `chrome://extensions`

<h3 id="connection-drops-during-long-sessions">
  Koneksi terputus selama sesi panjang
</h3>

Service worker ekstensi Chrome dapat menjadi idle selama sesi yang diperpanjang, yang memutus koneksi. Jika alat browser berhenti bekerja setelah periode inaktivitas, jalankan `/chrome` dan pilih "Reconnect extension".

<h3 id="windows-specific-issues">
  Masalah khusus Windows
</h3>

Di Windows, Anda mungkin mengalami:

* **Konflik named pipe (EADDRINUSE)**: jika proses lain menggunakan named pipe yang sama, restart Claude Code. Tutup sesi Claude Code lain apa pun yang mungkin menggunakan Chrome.
* **Kesalahan host messaging asli**: jika host messaging asli mogok saat startup, coba instal ulang Claude Code untuk membuat ulang konfigurasi host.

<h3 id="common-error-messages">
  Pesan kesalahan umum
</h3>

Ini adalah kesalahan yang paling sering dihadapi dan cara menyelesaikannya:

| Kesalahan                            | Penyebab                                            | Perbaikan                                                                               |
| ------------------------------------ | --------------------------------------------------- | --------------------------------------------------------------------------------------- |
| "Browser extension is not connected" | Host messaging asli tidak dapat menjangkau ekstensi | Restart Chrome dan Claude Code, kemudian jalankan `/chrome` untuk menghubungkan kembali |
| "Extension not detected"             | Ekstensi Chrome tidak diinstal atau dinonaktifkan   | Instal atau aktifkan ekstensi di `chrome://extensions`                                  |
| "No tab available"                   | Claude mencoba bertindak sebelum tab siap           | Minta Claude membuat tab baru dan coba lagi                                             |
| "Receiving end does not exist"       | Service worker ekstensi menjadi idle                | Jalankan `/chrome` dan pilih "Reconnect extension"                                      |

<h2 id="see-also">
  Lihat juga
</h2>

* [Computer use](/docs/id/computer-use): kontrol aplikasi macOS asli ketika tugas tidak dapat dilakukan di browser
* [Gunakan Claude Code di VS Code](/docs/id/vs-code#automate-browser-tasks-with-chrome): otomasi browser di ekstensi VS Code
* [Referensi CLI](/docs/id/cli-reference): flag baris perintah termasuk `--chrome`
* [Alur kerja umum](/docs/id/common-workflows): lebih banyak cara untuk menggunakan Claude Code
* [Data dan privasi](/docs/id/data-usage): bagaimana Claude Code menangani data Anda
* [Memulai dengan Claude in Chrome](https://support.claude.com/en/articles/12012173-getting-started-with-claude-in-chrome): dokumentasi lengkap untuk ekstensi Chrome, termasuk pintasan keyboard, penjadwalan, dan izin
