> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Debug konfigurasi Anda

> Diagnosis mengapa CLAUDE.md, settings, hooks, server MCP, atau skills tidak berlaku. Gunakan /context, /doctor, /hooks, dan /mcp untuk melihat apa yang benar-benar dimuat.

Ketika Claude mengabaikan instruksi atau fitur yang Anda konfigurasi tidak muncul, penyebabnya biasanya adalah file tidak dimuat, dimuat dari lokasi berbeda dari yang Anda harapkan, atau file lain menggantinya. Panduan ini menunjukkan cara memeriksa apa yang benar-benar dimuat oleh Claude Code sehingga Anda dapat mempersempit mana yang berlaku.

Untuk masalah instalasi, autentikasi, dan konektivitas, lihat [Troubleshoot installation and login](/docs/id/troubleshoot-install) sebagai gantinya.

<h2 id="see-what-loaded-into-context">
  Lihat apa yang dimuat ke dalam context
</h2>

Perintah `/context` menampilkan semua yang menempati jendela context untuk sesi saat ini, dipecah berdasarkan kategori: system prompt, file memory, skills, custom subagents dengan sumber masing-masing dimuat dari, alat MCP, dan pesan percakapan. Jalankan terlebih dahulu untuk mengonfirmasi apakah `CLAUDE.md`, rules, atau deskripsi skill Anda ada sama sekali.

Untuk detail tentang kategori tertentu, lanjutkan dengan perintah khusus:

| Perintah         | Menampilkan                                                                                                                                                                                                                                                                          |
| :--------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `/memory`        | File `CLAUDE.md` dan rules mana yang dimuat, ditambah entri auto-memory                                                                                                                                                                                                              |
| `/skills`        | Skill yang tersedia dari sumber proyek, pengguna, dan plugin                                                                                                                                                                                                                         |
| `/hooks`         | Konfigurasi hook aktif                                                                                                                                                                                                                                                               |
| `/mcp`           | Server MCP yang terhubung dan statusnya                                                                                                                                                                                                                                              |
| `/permissions`   | Aturan allow dan deny yang diselesaikan saat ini berlaku                                                                                                                                                                                                                             |
| `/doctor`        | Diagnostik konfigurasi: kesehatan instalasi, file pengaturan tidak valid, ekstensi yang tidak digunakan, nama [subagent](/docs/id/sub-agents) duplikat di direktori yang sama, dan konten `CLAUDE.md` yang diperiksa Claude dapat turunkan dari codebase, dengan perbaikan yang diusulkan |
| `/debug [issue]` | Mengaktifkan debug logging untuk sesi dan meminta Claude untuk mendiagnosis menggunakan output log dan jalur pengaturan                                                                                                                                                              |
| `/status`        | Sumber pengaturan aktif, termasuk apakah pengaturan terkelola berlaku                                                                                                                                                                                                                |

Jika file memory hilang dari `/memory`, periksa lokasinya terhadap [bagaimana file CLAUDE.md dimuat](/docs/id/memory#how-claude-md-files-load). File `CLAUDE.md` subdirektori dimuat sesuai permintaan ketika Claude membaca file di direktori itu dengan alat Read, bukan pada awal sesi.

Jika `/memory` mengonfirmasi file dimuat tetapi Claude masih tidak mengikuti instruksi tertentu, masalahnya kemungkinan adalah cara instruksi ditulis daripada apakah itu dimuat. CLAUDE.md bekerja dengan baik untuk jenis panduan yang akan Anda berikan kepada rekan kerja baru, seperti konvensi proyek, perintah build, dan di mana file berada.

Kepatuhan menurun ketika instruksi cukup samar untuk diinterpretasikan dengan berbagai cara, ketika dua file memberikan arahan yang bertentangan, atau ketika file telah tumbuh cukup panjang sehingga aturan individual mendapat perhatian lebih sedikit. [Tulis instruksi yang efektif](/docs/id/memory#write-effective-instructions) mencakup pola spesifisitas, ukuran, dan struktur yang menjaga kepatuhan tetap tinggi.

<Note>
  CLAUDE.md dan permissions menyelesaikan masalah yang berbeda. CLAUDE.md memberi tahu Claude bagaimana proyek Anda bekerja sehingga membuat keputusan yang baik. [Permissions](/docs/id/permissions) dan [hooks](/docs/id/hooks) memberlakukan batas terlepas dari apa yang Claude putuskan. Gunakan CLAUDE.md untuk "kami melakukannya dengan cara ini di sini." Gunakan permissions atau hooks untuk batas keamanan dan apa pun yang tidak boleh terjadi, di mana Anda membutuhkan jaminan daripada panduan.
</Note>

<h2 id="check-resolved-settings">
  Periksa pengaturan yang diselesaikan
</h2>

Pengaturan menggabungkan di seluruh cakupan terkelola, pengguna, proyek, dan lokal. Pengaturan terkelola selalu menang ketika ada. Di antara sisanya, cakupan yang lebih dekat menggantikan yang lebih luas dalam urutan lokal, kemudian proyek, kemudian pengguna. Beberapa pengaturan juga dapat diatur oleh flag baris perintah atau [variabel lingkungan](/docs/id/env-vars), yang bertindak sebagai lapisan penggantian lain. Ketika pengaturan tidak tampak berlaku, nilai yang Anda atur biasanya ditimpa oleh cakupan lain atau variabel lingkungan.

Jalankan `/doctor` untuk memeriksa konfigurasi dan instalasi Anda. Ini melaporkan apa yang ditemukannya, termasuk file pengaturan yang tidak valid, instalasi duplikat, ekstensi yang tidak digunakan, dan konten `CLAUDE.md` yang diperiksa yang Claude dapat turunkan dari basis kode, kemudian mengusulkan perbaikan yang diterapkannya hanya setelah Anda mengonfirmasi. Pemeriksaan pemangkasan `CLAUDE.md` memerlukan Claude Code v2.1.206 atau lebih baru. Sebelum v2.1.205, `/doctor` membuka layar diagnostik baca-saja dan menekan `f` mengirim laporan ke Claude untuk diperbaiki.

Dari terminal, `claude doctor` mencetak diagnostik instalasi dan pengaturan baca-saja tanpa memulai sesi.

Jalankan `/status` untuk melihat sumber pengaturan mana yang aktif, termasuk apakah pengaturan terkelola berlaku. Untuk memahami cakupan mana yang menang untuk kunci tertentu, lihat [Bagaimana cakupan berinteraksi](/docs/id/settings#how-scopes-interact).

<h2 id="check-mcp-servers">
  Periksa server MCP
</h2>

Jalankan `/mcp` untuk melihat setiap server yang dikonfigurasi, status koneksinya, dan apakah Anda telah menyetujuinya untuk proyek saat ini. Server dapat didefinisikan dengan benar tetapi masih tidak menyediakan alat untuk beberapa alasan umum:

* Server berscopeproyek di `.mcp.json` memerlukan persetujuan satu kali. Jika prompt ditutup, server tetap dinonaktifkan sampai Anda menyetujuinya dari `/mcp`.
* Server yang gagal dimulai ditampilkan sebagai gagal di `/mcp`. Jalur file relatif di `command` atau `args` adalah penyebab yang sering, karena mereka diselesaikan terhadap direktori tempat Anda meluncurkan Claude Code daripada lokasi `.mcp.json`.
* Server yang menampilkan sebagai terhubung tetapi mencantumkan alat nol telah dimulai dengan sukses tetapi tidak mengembalikan daftar alat. Pilih **Reconnect** dari `/mcp`. Jika hitungan tetap nol, jalankan `claude --debug mcp` untuk melihat output stderr server.

Untuk lokasi konfigurasi dan aturan cakupan, lihat [MCP](/docs/id/mcp).

<h2 id="check-hooks">
  Periksa hooks
</h2>

Jalankan `/hooks` untuk mencantumkan setiap hook yang terdaftar untuk sesi saat ini, dikelompokkan berdasarkan acara. Jika hook yang Anda tentukan tidak muncul, itu tidak dibaca: hooks berada di bawah kunci `"hooks"` dalam file pengaturan, bukan dalam file mandiri.

Jika hook muncul tetapi tidak aktif, matcher adalah penyebab yang biasa. Periksa matcher untuk kesalahan berikut:

* Bidang `matcher` adalah string tunggal yang menggunakan `|` untuk mencocokkan beberapa nama alat, misalnya `"Edit|Write"`. Pemisah `,` setara, jadi `"Edit,Write"` mencocokkan alat yang sama. Sebelum v2.1.191, koma jatuh ke evaluasi regex dan matcher tidak pernah cocok, jadi gunakan `|` jika Anda belum berada di v2.1.191.
* Nama alat yang salah eja menghasilkan matcher yang tidak mencocokkan apa pun, jadi hook gagal diam-diam.
* Nilai array adalah kesalahan schema: Claude Code menampilkan pemberitahuan kesalahan pengaturan dan menolak seluruh file pengaturan pengguna, proyek, atau lokal, `claude doctor` melaporkan kegagalan validasi, dan tidak ada hook dari file tersebut yang muncul di `/hooks`. Dalam [pengaturan terkelola](/docs/id/settings#settings-files), hanya entri yang tidak valid yang dihapus dan hook lain dari file masih berlaku.

Edit ke `settings.json` berlaku dalam sesi yang berjalan setelah penundaan stabilitas file singkat. Anda tidak perlu memulai ulang. Jika `/hooks` masih menampilkan definisi lama beberapa detik setelah menyimpan, jalankan `/hooks` lagi untuk menyegarkan tampilan.

Jika `/hooks` menampilkan hook tetapi masih tidak aktif, langkah berikutnya adalah menonton evaluasi hook secara langsung. Mulai sesi dengan `claude --debug hooks` dan picu panggilan alat. Log debug mencatat setiap acara, matcher mana yang diperiksa, dan kode keluar dan output hook. Lihat [Debug hooks](/docs/id/hooks#debug-hooks) untuk format log dan [troubleshooting hooks](/docs/id/hooks-guide#limitations-and-troubleshooting) untuk pola kegagalan umum.

<h2 id="test-against-a-clean-configuration">
  Uji terhadap konfigurasi bersih
</h2>

Mulai dengan [`claude --safe-mode`](/docs/id/cli-reference#cli-flags), yang meluncurkan sesi dengan semua kustomisasi dinonaktifkan, termasuk `CLAUDE.md`, skills, plugins, hooks, server MCP, dan perintah dan agen kustom. Autentikasi, pemilihan model, alat bawaan, dan izin berfungsi secara normal. Jika masalah hilang dalam safe mode, salah satu permukaan tersebut adalah penyebabnya; gunakan pemeriksaan yang ditargetkan di atas untuk menemukan mana. Safe mode masih menerapkan hooks yang dikelola dan kebijakan pengaturan dari organisasi Anda. Plugin yang dikelola, skills, CLAUDE.md, dan server MCP dimatikan.

Jika masalah berlanjut dalam safe mode, atau pengaturan Anda sendiri mencurigakan, bandingkan dengan sesi yang tidak memuat apa pun dari pengaturan biasa Anda. Arahkan [`CLAUDE_CONFIG_DIR`](/docs/id/env-vars) ke direktori kosong untuk melewati semua yang ada di bawah `~/.claude`, dan luncurkan dari direktori yang tidak memiliki folder `.claude`, `.mcp.json`, atau `CLAUDE.md` sehingga konfigurasi proyek juga dilewati.

```bash theme={null}
cd /tmp && CLAUDE_CONFIG_DIR=/tmp/claude-clean claude
```

Sesi bersih tidak memiliki pengaturan pengguna atau proyek, hooks, server MCP, plugins, atau memory.

* Pengaturan terkelola masih berlaku jika organisasi Anda menerapkannya, karena mereka berada di jalur sistem di luar `~/.claude`
* Di Linux dan Windows, Anda akan diminta untuk masuk lagi karena kredensial disimpan di bawah direktori konfigurasi
* Di macOS, kredensial berada di Keychain dan terbawa ke sesi bersih

Jika masalah hilang di sini, penyebabnya ada di suatu tempat di file `~/.claude` atau proyek `.claude` Anda yang sebenarnya. Perkenalkan kembali satu per satu, dengan menyalin file ke direktori sementara atau dengan meluncurkan dari proyek Anda, untuk menemukan mana yang menjadi penyebabnya. Jika itu bertahan dalam sesi bersih, penyebabnya ada di luar konfigurasi pengguna dan proyek Anda. Jalankan `/status` untuk memeriksa apakah pengaturan terkelola berlaku, cari [variabel lingkungan](/docs/id/env-vars) yang mempengaruhi Claude Code, kemudian lihat [Troubleshooting](/docs/id/troubleshooting).

<h2 id="check-common-causes">
  Periksa penyebab umum
</h2>

Sebagian besar kejutan konfigurasi dapat dilacak kembali ke serangkaian kecil aturan lokasi dan sintaks. Periksa ini sebelum menganggap bug:

| Gejala                                                                              | Penyebab                                                                                                                                 | Perbaikan                                                                                                                                                                                                                                                            |
| :---------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Hook tidak pernah aktif                                                             | `matcher` adalah array JSON bukan string                                                                                                 | Gunakan string tunggal dengan `\|` untuk mencocokkan beberapa alat, misalnya `"Edit\|Write"`. Lihat [pola matcher](/docs/id/hooks#matcher-patterns).                                                                                                                      |
| Hook tidak pernah aktif                                                             | `matcher` menggunakan `,` sebagai pemisah pada versi sebelum v2.1.191                                                                    | Claude Code v2.1.191 atau lebih baru memperlakukan `,` sebagai pemisah daftar seperti `\|`. Versi sebelumnya mengevaluasi koma sebagai karakter literal, jadi `"Edit,Write"` tidak cocok dengan apa pun. Gunakan `\|` sebagai gantinya, atau tingkatkan Claude Code. |
| Hook tidak pernah aktif                                                             | Nilai `matcher` adalah huruf kecil, misalnya `"bash"`                                                                                    | Pencocokan peka huruf besar-kecil. Nama alat dikapitalisasi: `Bash`, `Edit`, `Write`, `Read`.                                                                                                                                                                        |
| Hook tidak pernah aktif                                                             | Hook berada dalam file standalone bukan `settings.json`                                                                                  | Tidak ada file hooks standalone untuk konfigurasi proyek atau pengguna. Tentukan hooks di bawah kunci `"hooks"` di `settings.json`. Hanya [plugins](/docs/id/plugins-reference#hooks) yang memuat `hooks/hooks.json` terpisah. Lihat [konfigurasi hook](/docs/id/hooks).       |
| Permissions, hooks, atau env yang diatur secara global diabaikan                    | Konfigurasi ditambahkan ke `~/.claude.json`                                                                                              | `~/.claude.json` menyimpan status aplikasi dan toggle UI. `permissions`, `hooks`, dan `env` termasuk dalam `~/.claude/settings.json`. Ini adalah dua file berbeda.                                                                                                   |
| Nilai `settings.json` tampak diabaikan                                              | Kunci yang sama diatur di `settings.local.json`                                                                                          | `settings.local.json` menggantikan `settings.json`, dan keduanya menggantikan `~/.claude/settings.json`. Lihat [preseden pengaturan](/docs/id/settings#how-scopes-interact).                                                                                              |
| Skill tidak muncul di `/skills`                                                     | File skill berada di `.claude/skills/name.md` bukan dalam folder                                                                         | Gunakan folder dengan `SKILL.md` di dalamnya: `.claude/skills/name/SKILL.md`.                                                                                                                                                                                        |
| Skill muncul di `/skills` tetapi Claude tidak pernah menginvokasinya                | Skill memiliki `disable-model-invocation: true` di frontmatter-nya, atau deskripsinya tidak cocok dengan cara Anda merumuskan permintaan | Periksa lencana di `/skills`: label "user-only" berarti Claude tidak akan memicunya sendiri. Lihat [skill invocation](/docs/id/skills).                                                                                                                                   |
| Instruksi `CLAUDE.md` subdirektori tampak diabaikan                                 | File subdirektori dimuat sesuai permintaan, bukan pada awal sesi                                                                         | Mereka dimuat ketika Claude membaca file di direktori itu dengan alat Read, bukan saat peluncuran dan bukan saat menulis atau membuat file di sana. Lihat [bagaimana file CLAUDE.md dimuat](/docs/id/memory#how-claude-md-files-load).                                    |
| Subagent mengabaikan instruksi `CLAUDE.md`                                          | Agen Explore dan Plan bawaan melewati `CLAUDE.md`. Subagent kustom memuatnya dengan cara yang sama seperti percakapan utama              | Untuk Explore atau Plan, nyatakan kembali instruksi dalam prompt delegasi Anda. Untuk subagent kustom, letakkan instruksi penting di badan file agen, yang menjadi system prompt agen. Lihat [apa yang dimuat saat startup](/docs/id/sub-agents#what-loads-at-startup).   |
| Logika pembersihan tidak pernah berjalan di akhir sesi                              | Tidak ada hook `SessionEnd` yang dikonfigurasi                                                                                           | Tambahkan hook `SessionEnd` di `settings.json`. Lihat [daftar acara hook](/docs/id/hooks#hook-events).                                                                                                                                                                    |
| Server MCP di `.mcp.json` tidak pernah dimuat                                       | File berada di bawah `.claude/` atau menggunakan format konfigurasi Claude Desktop                                                       | Konfigurasi MCP proyek berada di akar repositori sebagai `.mcp.json`, bukan di dalam `.claude/`. Lihat [konfigurasi MCP](/docs/id/mcp).                                                                                                                                   |
| Server MCP ditambahkan di bawah `mcpServers` di `settings.json` tidak pernah muncul | `settings.json` tidak membaca kunci `mcpServers`                                                                                         | Tentukan server proyek di `.mcp.json` di akar repositori, atau jalankan `claude mcp add --scope user` untuk server berscopepengguna. Lihat [konfigurasi MCP](/docs/id/mcp).                                                                                               |
| Server MCP proyek ditambahkan tetapi tidak muncul                                   | Prompt persetujuan satu kali ditutup                                                                                                     | Server berscopeproyek memerlukan persetujuan. Jalankan `/mcp` untuk melihat status dan setujui.                                                                                                                                                                      |
| Server MCP gagal dimulai dari beberapa direktori                                    | `command` atau `args` menggunakan jalur file relatif                                                                                     | Gunakan jalur absolut untuk skrip lokal. Executable di `PATH` Anda seperti `npx` atau `uvx` bekerja apa adanya.                                                                                                                                                      |
| Server MCP dimulai tanpa variabel lingkungan yang diharapkan                        | Variabel berada di `settings.json` `env`, yang tidak menyebar ke proses anak MCP                                                         | Atur `env` per-server di dalam `.mcp.json` sebagai gantinya.                                                                                                                                                                                                         |
| Aturan deny `Bash(rm *)` tidak memblokir `/bin/rm` atau `find -delete`              | Aturan awalan mencocokkan string perintah literal, bukan executable yang mendasar                                                        | Tambahkan pola eksplisit untuk setiap varian, atau gunakan [hook PreToolUse](/docs/id/hooks-guide) atau [sandbox](/docs/id/sandboxing) untuk jaminan keras.                                                                                                                    |

<h2 id="related-resources">
  Sumber daya terkait
</h2>

Untuk referensi lengkap pada setiap permukaan konfigurasi, lihat halaman khusus:

* **[Referensi direktori `.claude`](/docs/id/claude-directory)**: setiap lokasi file konfigurasi dan apa yang membacanya
* **[Settings](/docs/id/settings)**: urutan preseden dan daftar kunci lengkap
* **[Referensi Hooks](/docs/id/hooks)**: nama acara, payload, dan format output `--debug hooks`
* **[MCP](/docs/id/mcp)**: konfigurasi server, persetujuan, dan output `/mcp`
* **[Troubleshoot installation and login](/docs/id/troubleshoot-install)**: `command not found`, PATH, dan masalah autentikasi
* **[Troubleshooting](/docs/id/troubleshooting)**: kinerja, hang, dan masalah pencarian
