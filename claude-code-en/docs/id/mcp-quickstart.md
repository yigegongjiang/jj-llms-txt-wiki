> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Terhubung ke server MCP

> Tambahkan server MCP ke Claude Code, verifikasi koneksi, dan temukan konfigurasi di disk.

[Model Context Protocol (MCP)](https://modelcontextprotocol.io/introduction) memungkinkan Claude Code menggunakan alat di luar set bawaan, seperti mencari pelacak masalah, menanyakan database, atau mengontrol browser web. Alat-alat ini berasal dari server MCP, yang berjalan di mesin Anda atau sebagai layanan yang dihosting.

Panduan ini memandu Anda melalui koneksi satu server MCP end-to-end dengan CLI Claude Code. Pada akhirnya, Anda akan memiliki server yang terhubung dan merespons, mengetahui di mana konfigurasinya berada di disk, dan mengetahui cara memperbaiki kesalahan koneksi yang paling umum.

<Note>
  Anda juga dapat menambahkan server MCP dari permukaan lain, termasuk aplikasi desktop, VS Code, dan web. Lihat [Terhubung dari permukaan lain](#connect-from-other-surfaces).
</Note>

Untuk setiap cara menghubungkan dan mengonfigurasi server MCP di Claude Code, lihat [referensi MCP](/docs/id/mcp).

<h2 id="before-you-begin">
  Sebelum Anda mulai
</h2>

Pastikan Anda memiliki:

* [Claude Code terinstal](/docs/id/quickstart) dan terauthentikasi
* Terminal terbuka di direktori proyek. Direktori apa pun berfungsi, termasuk yang kosong.

<h2 id="add-and-verify-a-server">
  Tambahkan dan verifikasi server
</h2>

Contoh di bawah terhubung ke [server MCP dokumentasi Claude Code](https://code.claude.com/docs/mcp), server yang dihosting dengan pencarian teks lengkap di atas dokumen Claude Code. Ini tidak memerlukan autentikasi atau konfigurasi khusus apa pun, jadi berfungsi dengan baik sebagai server pertama untuk menguji alur pengaturan.

Langkah-langkahnya sama untuk server apa pun: tambahkan, periksa status koneksi, kemudian gunakan dalam sesi, dengan langkah pembersihan opsional di akhir. Beberapa server menambahkan langkah, seperti masuk browser, ditampilkan di [Contoh server MCP tambahan](#additional-mcp-server-examples). Untuk lebih banyak server untuk terhubung, telusuri [Direktori Anthropic](/docs/id/mcp#find-and-build-mcp-servers).

<Steps>
  <Step title="Tambahkan server MCP">
    Daftarkan server dengan Claude Code. Jalankan ini di terminal Anda, bukan di dalam sesi `claude`: Anda mengonfigurasi server sebelum memulai percakapan.

    ```bash theme={null}
    claude mcp add --transport http claude-code-docs https://code.claude.com/docs/mcp
    ```

    Bagian-bagian dari perintah:

    * `claude mcp add`: mendaftarkan server dengan Claude Code.
    * `--transport http`: server dihosting di URL daripada berjalan sebagai proses lokal.
    * `claude-code-docs`: nama yang Anda buat. Memanggil server yang sama `docs` akan berfungsi identik. Claude Code menggunakan nama apa pun yang Anda pilih untuk memberi label alat server di output Claude dan untuk merujuk ke server dalam perintah seperti `claude mcp remove`.
    * `https://code.claude.com/docs/mcp`: URL tempat server dihosting.

    Perintah mencetak konfirmasi seperti `Added HTTP MCP server claude-code-docs with URL: https://code.claude.com/docs/mcp to local config`. Bagian `local config` berarti server terdaftar untuk Anda, dalam proyek ini: jika Anda memulai Claude Code di proyek yang berbeda, server ini tidak aktif di sana. Untuk mendaftarkan server sekali untuk semua proyek Anda, tambahkan di cakupan pengguna, tercakup dalam [Ubah cakupan server](#change-server-scope).
  </Step>

  <Step title="Periksa status koneksi">
    Konfirmasi server muncul dalam daftar server Anda dan periksa statusnya:

    ```bash theme={null}
    claude mcp list
    ```

    Server muncul dengan indikator status:

    | Status                             | Arti                                                                                                                                                                                             |
    | :--------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
    | `✓ Connected`                      | Siap digunakan. Ini adalah apa yang seharusnya Anda lihat untuk `claude-code-docs`                                                                                                               |
    | `! Connected · tools fetch failed` | Server terhubung tetapi tidak dapat membuat daftar alatnya. Jalankan `claude mcp get <name>` untuk detail kesalahan                                                                              |
    | `! Needs authentication`           | Server dapat dijangkau tetapi memerlukan masuk browser, atau token yang diteruskan dengan `--header`. Lihat [Terhubung ke server yang memerlukan masuk](#connect-a-server-that-requires-sign-in) |
    | `✗ Failed to connect`              | Server tidak merespons. Lihat [Troubleshooting](#troubleshooting)                                                                                                                                |
    | `✗ Connection error`               | Upaya koneksi melempar kesalahan. Lihat [Troubleshooting](#troubleshooting)                                                                                                                      |
    | `⏸ Pending approval`               | Server yang dibatasi proyek yang belum Anda setujui. Lihat [Edit .mcp.json secara langsung](#edit-mcp-json-directly)                                                                             |
  </Step>

  <Step title="Gunakan server">
    Mulai sesi dan minta Claude menggunakan server baru berdasarkan nama:

    ```bash theme={null}
    claude
    ```

    ```text theme={null}
    Use the claude-code-docs server to look up what MCP_TIMEOUT does
    ```

    <Info>
      Anda biasanya tidak perlu menyebutkan server dalam prompt Anda, karena Claude memilih alat yang relevan dengan sendirinya. Menyebutkannya di sini menjamin demonstrasi melalui server baru daripada alat lain, seperti web fetch, yang dapat menjawab pertanyaan yang sama.
    </Info>

    Pertama kali Claude memanggil server, ia meminta izin untuk menggunakan alat baru. Setujui untuk melanjutkan. Panggilan alat dalam output Claude diberi label dengan nama server, yang merupakan cara Anda mengonfirmasi jawaban berasal dari server MCP daripada pengetahuan bawaan Claude.
  </Step>

  <Step title="Hapus server">
    Langkah ini bersifat opsional. Ketika Anda selesai bereksperimen, Anda dapat menghapus server:

    ```bash theme={null}
    claude mcp remove claude-code-docs
    ```

    <Note>
      Setiap server yang terhubung membutuhkan beberapa ruang di [jendela konteks Claude](/docs/id/how-claude-code-works#the-context-window) karena nama alat dan instruksi server dimuat ke dalam setiap sesi. Menghapus server yang tidak lagi Anda gunakan membuat ruang itu tetap bebas.
    </Note>
  </Step>
</Steps>

<h2 id="where-servers-are-saved">
  Tempat server disimpan
</h2>

Perintah `claude mcp add` menulis detail server ke file konfigurasi. Secara default, ia mendaftarkan server di cakupan `local`: pribadi untuk Anda, aktif hanya dalam proyek saat ini. Teruskan `--scope user` untuk mendaftarkannya sekali untuk semua proyek Anda, atau `--scope project` untuk membagikannya dengan rekan tim. [Ubah cakupan server](#change-server-scope) memandu keduanya.

<Note>
  `claude mcp add` bekerja sama di setiap shell, termasuk PowerShell dan Command Prompt. Di dalam sesi `claude`, gunakan perintah `/mcp` untuk memeriksa dan mengelola server yang telah Anda tambahkan.
</Note>

Ada cara lain untuk menambahkan server, masing-masing tercakup nanti di halaman ini:

* [Tambahkan server lokal](#add-a-local-server): jalankan program di mesin Anda daripada terhubung ke URL.
* [Edit `.mcp.json` secara langsung](#edit-mcp-json-directly): tulis entri JSON sendiri daripada menggunakan perintah.
* [Terhubung ke server yang memerlukan masuk](#connect-a-server-that-requires-sign-in): tambahkan server yang dihosting yang memerlukan masuk browser sebelum alatnya berfungsi.

<h3 id="find-your-configuration-on-disk">
  Temukan konfigurasi Anda di disk
</h3>

Perintah `claude mcp add` menulis server ke salah satu dari tiga cakupan, disimpan di dua file, tergantung pada flag `--scope`. Anda tidak perlu mengedit file ini secara langsung, tetapi mengetahui di mana mereka berada membantu dengan debugging dan kontrol versi.

| Cakupan   | File                                                       | Tersedia untuk                        |
| :-------- | :--------------------------------------------------------- | :------------------------------------ |
| `local`   | `~/.claude.json`, di bawah entri untuk proyek ini          | Hanya Anda, hanya proyek ini. Default |
| `project` | `.mcp.json` di akar proyek Anda                            | Semua orang yang mengkloning proyek   |
| `user`    | `~/.claude.json`, di bawah kunci `mcpServers` tingkat atas | Hanya Anda, semua proyek              |

Di Windows, `~/.claude.json` diselesaikan ke `%USERPROFILE%\.claude.json`, biasanya `C:\Users\YourName\.claude.json`. Jika Anda telah menetapkan [`CLAUDE_CONFIG_DIR`](/docs/id/env-vars), Claude Code membaca `.claude.json` dari dalam direktori itu.

Jalankan `claude mcp get claude-code-docs` untuk melihat cakupan mana yang menyimpan definisi server. Untuk cara cakupan berinteraksi ketika server yang sama didefinisikan di lebih dari satu, lihat [cakupan instalasi MCP](/docs/id/mcp#mcp-installation-scopes).

<h2 id="change-server-scope">
  Ubah cakupan server
</h2>

Cakupan server diperbaiki ketika Anda menambahkannya, jadi mengubah cakupan berarti menghapus entri dan menambahkannya kembali di yang baru. Kedua kasus di bawah dimulai dengan menghapus entri lokal dari panduan pertama, jadi server hanya memiliki satu definisi. Jika Anda sudah menghapusnya di akhir panduan itu, lewati perintah ini:

```bash theme={null}
claude mcp remove claude-code-docs --scope local
```

<h3 id="use-a-server-in-all-your-projects">
  Gunakan server di semua proyek Anda
</h3>

Tambahkan kembali server di cakupan `user` untuk membuatnya aktif di setiap proyek yang Anda buka, masih pribadi untuk Anda:

```bash theme={null}
claude mcp add --scope user --transport http claude-code-docs https://code.claude.com/docs/mcp
```

<h3 id="share-a-server-with-your-team">
  Bagikan server dengan tim Anda
</h3>

Tambahkan kembali server di cakupan `project`, yang menulis ke `.mcp.json` di akar proyek:

```bash theme={null}
claude mcp add --scope project --transport http claude-code-docs https://code.claude.com/docs/mcp
```

Komit `.mcp.json` ke kontrol versi. Rekan tim yang mengkloning repositori dan memulai Claude Code melihat prompt untuk menyetujui server, kemudian terhubung untuk mereka juga.

<h2 id="additional-mcp-server-examples">
  Contoh server MCP tambahan
</h2>

Panduan pertama menggunakan server yang dihosting yang terhubung tanpa masuk apa pun. Contoh di bawah mencakup dua bentuk umum lainnya, dengan alur tambah, periksa, gunakan yang sama.

<h3 id="add-a-local-server">
  Tambahkan server lokal
</h3>

Server stdio lokal adalah program yang Claude Code mulai sebagai subprocess di mesin Anda, daripada layanan yang dijangkaunya melalui URL. Gunakan satu untuk alat yang memerlukan akses ke sumber daya lokal seperti browser, sistem file Anda, atau soket database.

[Server MCP Playwright](https://github.com/microsoft/playwright-mcp) adalah yang bagus untuk dicoba: memberikan Claude browser yang dapat dinavigasi, diklik, dan dibaca, dan tidak memerlukan akun. Ini berjalan melalui `npx`, jadi memerlukan [Node.js](https://nodejs.org/en/download) 18 atau lebih baru.

<Steps>
  <Step title="Tambahkan server Playwright">
    Daftarkan server dengan perintah yang harus Claude Code jalankan untuk memulainya:

    ```bash theme={null}
    claude mcp add playwright -- npx -y @playwright/mcp@latest
    ```

    Perintah ini berbeda dari contoh yang dihosting dalam tiga cara:

    * Tidak ada flag `--transport`, karena server lokal menggunakan transport `stdio` default.
    * Semuanya setelah pemisah `--` adalah perintah yang Claude Code jalankan untuk memulai server.
    * `-y` memberitahu `npx` untuk menginstal paket tanpa meminta.

    Playwright mendorong Chrome apa pun yang sudah terinstal di mesin Anda. Untuk menggunakan browser yang berbeda, tambahkan `--browser` dengan nama browser, misalnya `--browser firefox`, setelah `@playwright/mcp@latest`.
  </Step>

  <Step title="Periksa koneksi">
    Konfirmasi `Added` berarti entri disimpan, bukan bahwa perintah berjalan. Periksa koneksi:

    ```bash theme={null}
    claude mcp list
    ```

    Pemeriksaan pertama dapat menunjukkan `✗ Failed to connect` sementara `npx` mengunduh paket, jadi tunggu sebentar dan jalankan lagi.
  </Step>

  <Step title="Gunakan browser">
    Berikan Claude tugas yang memerlukan browser:

    ```text theme={null}
    Use playwright to open https://example.com and tell me the page title
    ```

    Jendela browser terbuka sehingga Anda dapat menontonnya bekerja, dan panggilan alat dalam output Claude diberi label dengan nama server `playwright` dan tindakan, seperti `browser_navigate`.

    Coba arahkan ke server dev lokal Anda untuk memeriksa bahwa halaman masih dirender setelah perubahan, atau buat Claude menjalani laporan bug langkah demi langkah.
  </Step>
</Steps>

<h3 id="connect-a-server-that-requires-sign-in">
  Terhubung ke server yang memerlukan masuk
</h3>

Layanan yang dihosting seperti Sentry, Linear, dan Notion menjalankan server MCP mereka di belakang OAuth: Anda menambahkan URL server, kemudian masuk melalui browser Anda.

Langkah-langkah di bawah menggunakan Sentry sebagai contoh. Untuk terhubung ke layanan yang berbeda, gantikan URL-nya, yang dapat Anda temukan di [Direktori Anthropic](/docs/id/mcp#find-and-build-mcp-servers) atau dokumentasi layanan.

<Steps>
  <Step title="Tambahkan server">
    Perintah `add` sama seperti untuk server docs, dengan URL Sentry:

    ```bash theme={null}
    claude mcp add --transport http sentry https://mcp.sentry.dev/mcp
    ```

    Setelah menambahkan, `claude mcp list` menunjukkan server dengan `! Needs authentication`. Itu diharapkan: langkah berikutnya menyelesaikan masuk.
  </Step>

  <Step title="Autentikasi di browser Anda">
    Mulai sesi Claude Code dan buka panel MCP:

    ```text theme={null}
    /mcp
    ```

    Pilih `sentry` dari daftar, tekan Enter, dan pilih `Authenticate`. Browser Anda membuka halaman masuk Sentry. Setujui koneksi di sana.

    Kembali di Claude Code, status server berubah menjadi terhubung. Jika masuk gagal atau browser tidak terbuka, lihat [Troubleshooting](#troubleshooting).
  </Step>

  <Step title="Gunakan server">
    Tanyakan Claude sesuatu yang memerlukan layanan, seperti `What Sentry projects do I have access to?`, dan cari panggilan alat yang diberi label dengan nama server `sentry` di outputnya.
  </Step>
</Steps>

Server yang mengautentikasi dengan token statis daripada OAuth mengambil token pada waktu penambahan dengan `--header "Authorization: Bearer <token>"`. Lihat [contoh GitHub](/docs/id/mcp#example-connect-to-github-for-code-reviews) untuk versi yang dikerjakan.

<h2 id="edit-mcp-json-directly">
  Edit .mcp.json secara langsung
</h2>

Setiap file dalam [tabel cakupan](#find-your-configuration-on-disk) menggunakan format JSON yang sama untuk entri server. Bagian ini mengedit `.mcp.json`, file cakupan proyek. Ini adalah yang paling layak ditulis dengan tangan karena diperiksa ke dalam repositori, di mana ia berfungsi ganda sebagai konfigurasi-sebagai-kode untuk tim Anda.

Buat `.mcp.json` di akar proyek Anda. Contoh di bawah mendefinisikan kedua server dari panduan ini, server docs yang dihosting dijangkau melalui HTTP dan server Playwright sebagai proses `stdio` lokal:

```json theme={null}
{
  "mcpServers": {
    "claude-code-docs": {
      "type": "http",
      "url": "https://code.claude.com/docs/mcp"
    },
    "playwright": {
      "type": "stdio",
      "command": "npx",
      "args": ["-y", "@playwright/mcp@latest"]
    }
  }
}
```

Bidang berbeda menurut jenis server:

* Untuk server HTTP, `url` adalah titik akhir yang Claude Code terhubung.
* Untuk server stdio, `command` dan `args` adalah program yang dijalankannya.

Setelah menyimpan file, mulai sesi Claude Code baru di proyek. Claude Code membaca `.mcp.json` saat startup.

Pertama kali Claude Code melihat server yang dibatasi proyek, ia meminta Anda untuk menyetujuinya. Prompt ada sehingga repositori yang Anda kloning tidak dapat meluncurkan proses di mesin Anda tanpa persetujuan Anda. Setujui prompt, atau jalankan `/mcp` untuk menyetujui nanti jika Anda melewatkannya.

Setelah Anda menyetujui, jalankan `/mcp` dan periksa bahwa server menunjukkan sebagai terhubung. Jika salah satu menunjukkan kesalahan, lihat [Troubleshooting](#troubleshooting).

<h2 id="connect-from-other-surfaces">
  Terhubung dari permukaan lain
</h2>

Panduan ini menggunakan perintah CLI `claude mcp`, tetapi setiap permukaan Claude Code dapat terhubung ke server MCP:

* **Aplikasi desktop Claude Code**: tambahkan server melalui [UI Connectors](/docs/id/desktop#connect-external-tools).
* **Aplikasi chat Claude Desktop**: aplikasi terpisah dari Claude Code. Untuk menyalin server dari `claude_desktop_config.json`-nya ke CLI, jalankan `claude mcp add-from-claude-desktop` di macOS atau WSL.
* **VS Code**: lihat [Terhubung ke alat eksternal dengan MCP](/docs/id/vs-code#connect-to-external-tools-with-mcp).
* **Claude Code di web**: membaca `.mcp.json` dari repositori Anda. Lihat [Edit .mcp.json secara langsung](#edit-mcp-json-directly).
* **Claude.ai**: konektor yang Anda tambahkan di [claude.ai/customize/connectors](https://claude.ai/customize/connectors) dimuat secara otomatis di CLI ketika Anda masuk dengan akun itu. Lihat [Gunakan server MCP dari Claude.ai](/docs/id/mcp#use-mcp-servers-from-claude-ai).

<h2 id="troubleshooting">
  Troubleshooting
</h2>

Jika server tidak terhubung, periksa statusnya dengan `/mcp` di dalam sesi atau `claude mcp list` dari shell Anda, kemudian cocokkan gejala di bawah. Panel `/mcp` juga memungkinkan Anda untuk menghubungkan kembali atau mengautentikasi tanpa meninggalkan sesi.

<AccordionGroup>
  <Accordion title="/mcp shows No MCP servers configured">
    Claude Code tidak menemukan server apa pun untuk direktori saat ini. Penyebab paling umum:

    * Anda menjalankan `claude mcp add` dari proyek yang berbeda. Server yang dibatasi lokal terikat pada proyek tempat Anda menambahkannya: akar repositori, atau direktori yang tepat jika Anda tidak berada di repositori git. Tambahkan kembali server dari proyek yang Anda gunakan sekarang, atau tambahkan dengan `--scope user` sehingga tidak terikat pada proyek.
    * Anda mengedit file konfigurasi di jalur yang salah. File yang benar adalah `~/.claude.json` dan `<project>/.mcp.json`. Claude Code tidak membaca jalur seperti `~/.claude/.mcp.json`, `~/.claude/config/mcp.json`, `~/.claude/mcp.json`, atau `%APPDATA%\Claude\mcp.json`. Untuk server yang dibatasi pengguna, jalankan `claude mcp add --scope user`, yang menulis ke kunci `mcpServers` di `~/.claude.json`; untuk server yang dibatasi proyek, edit `.mcp.json` di akar proyek.
  </Accordion>

  <Accordion title="Status shows Failed to connect or Connection error">
    Kedua status berarti server tidak dimulai atau URL tidak merespons. Mereka juga dapat muncul untuk server HTTP yang mengharapkan token daripada masuk browser yang tercakup dalam [Terhubung ke server yang memerlukan masuk](#connect-a-server-that-requires-sign-in).

    Mulai dari v2.1.191, server HTTP yang mengembalikan `404 Not Found` menampilkan `MCP endpoint not found at <url>. Check the URL in your MCP config.` ketika Anda memilih server di `/mcp`, dengan URL yang dicoba Claude Code. Versi sebelumnya menampilkan pesan generik `Error POSTing to endpoint` tanpa URL. Bandingkan URL dengan jalur titik akhir MCP yang didokumentasikan server, kemudian jalankan `claude mcp remove <name>` dan tambahkan kembali dengan URL yang benar.

    Untuk server HTTP, konfirmasi URL dapat dijangkau dari mesin Anda:

    ```bash theme={null}
    curl -I https://mcp.sentry.dev/mcp
    ```

    Di PowerShell, gunakan `curl.exe` daripada `curl` sehingga permintaan pergi ke biner curl nyata daripada alias `Invoke-WebRequest`.

    Respons memberi tahu Anda jenis masalah apa yang Anda miliki:

    * A `404` atau `405`: server aktif. Banyak titik akhir MCP hanya menjawab permintaan POST, jadi ini masih mengonfirmasi URL dapat dijangkau dari mesin Anda.
    * A `401` atau `403`: server aktif dan Anda perlu mengautentikasi. Gunakan masuk browser di [Terhubung ke server yang memerlukan masuk](#connect-a-server-that-requires-sign-in), atau untuk server yang mengambil token, seperti GitHub, teruskan dengan `--header "Authorization: Bearer <token>"` pada perintah `claude mcp add`.
    * Tidak ada respons sama sekali: periksa URL dan jaringan Anda.

    Untuk server stdio, jalankan perintah yang dikonfigurasi langsung di terminal Anda untuk melihat kesalahan yang mendasar. Untuk server Playwright dari panduan ini, jalankan:

    ```bash theme={null}
    npx -y @playwright/mcp@latest
    ```

    Apa yang terjadi selanjutnya memberi tahu Anda di mana masalahnya:

    * Perintah dimulai dan menunggu input: server itu sendiri berfungsi. Jalankan `claude mcp get <name>` dan konfirmasi perintah yang ditampilkan di sana cocok dengan apa yang baru saja Anda jalankan. Jika perintah yang ditampilkan berbeda dari apa yang Anda ketik, Anda mungkin menghilangkan pemisah `--` sebelum perintah server. Hapus server dan tambahkan kembali dengan `--` di tempat. Jika Anda menulis `.mcp.json` dengan tangan, periksa sintaks dan lokasinya.
    * Perintah kesalahan: pesan menyebutkan apa yang hilang, seperti Node.js atau browser.
  </Accordion>

  <Accordion title="Connection timed out at startup">
    Server membutuhkan waktu lebih lama dari batas waktu startup default 30 detik. Jalankan pertama server stdio dapat lambat sementara `npx` mengunduh paket. Tingkatkan batas dengan variabel lingkungan [`MCP_TIMEOUT`](/docs/id/env-vars), dalam milidetik:

    ```bash theme={null}
    MCP_TIMEOUT=60000 claude
    ```

    Di PowerShell, atur variabel sebelum perintah pada baris yang sama:

    ```powershell theme={null}
    $env:MCP_TIMEOUT = "60000"; claude
    ```
  </Accordion>

  <Accordion title="Server already exists">
    Anda telah menambahkan server dengan nama itu di cakupan yang sama. Baik hapus entri yang ada terlebih dahulu atau pilih nama yang berbeda:

    ```bash theme={null}
    claude mcp remove claude-code-docs
    ```

    Jika nama ada di lebih dari satu cakupan, `remove` melaporkan `exists in multiple scopes`. Teruskan `--scope` untuk memilih salinan mana yang akan dihapus, misalnya `claude mcp remove claude-code-docs --scope local`.
  </Accordion>

  <Accordion title="Server connects but no tools appear">
    Jalankan `/mcp` di dalam sesi dan pilih server untuk melihat daftar alatnya. Jika daftar kosong, server dimulai tetapi tidak mendaftarkan alat apa pun, yang biasanya berarti hilang variabel lingkungan yang diperlukan seperti kunci API.

    Teruskan variabel dengan `--env KEY=value` pada `claude mcp add`, atau di bidang `env` dari entri `.mcp.json` server. Dokumentasi server mencantumkan variabel yang dibutuhkannya.
  </Accordion>

  <Accordion title="Changes to .mcp.json don't take effect">
    Claude Code membaca `.mcp.json` saat startup sesi. Keluar dan mulai ulang sesi setelah mengedit file.

    Jika server Anda masih tidak muncul, jalankan `/mcp` dan cari peringatan parse. Claude Code melewati entri yang salah bentuk dan menunjukkan bidang yang menyinggung di sana.

    Jika Anda sebelumnya menolak server saat diminta, atur ulang persetujuan proyek:

    ```bash theme={null}
    claude mcp reset-project-choices
    ```
  </Accordion>

  <Accordion title="OAuth sign-in fails or browser doesn't open">
    Jalankan `/mcp`, pilih server, dan pilih `Authenticate` lagi. Jika browser tidak terbuka secara otomatis, salin URL yang ditampilkan di terminal dan buka secara manual. Lihat [Autentikasi dengan server MCP jarak jauh](/docs/id/mcp#authenticate-with-remote-mcp-servers) untuk port callback tetap dan kredensial yang telah dikonfigurasi sebelumnya.
  </Accordion>
</AccordionGroup>

<h2 id="next-steps">
  Langkah berikutnya
</h2>

Dengan satu server terhubung, jelajahi sisa dari apa yang MCP memungkinkan:

* [Temukan lebih banyak server MCP](/docs/id/mcp#find-and-build-mcp-servers) di Direktori Anthropic
* [Bagikan server dengan tim Anda](/docs/id/mcp#mcp-installation-scopes) menggunakan cakupan instalasi
* [Kelola akses MCP untuk organisasi](/docs/id/managed-mcp) dengan pengaturan terkelola dan kontrol kebijakan
* [Referensi sumber daya MCP](/docs/id/mcp#use-mcp-resources) dalam prompt dengan penyebutan @
* [Jalankan prompt MCP sebagai perintah](/docs/id/mcp#use-mcp-prompts-as-commands) dari menu `/`
* [Bangun server Anda sendiri](https://modelcontextprotocol.io/quickstart/server) dengan SDK MCP
