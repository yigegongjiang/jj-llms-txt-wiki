> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Hubungkan Claude Code ke alat melalui MCP

> Pelajari cara menghubungkan Claude Code ke alat Anda dengan Model Context Protocol.

Claude Code dapat terhubung ke ratusan alat eksternal dan sumber data melalui [Model Context Protocol (MCP)](https://modelcontextprotocol.io/introduction), standar sumber terbuka untuk integrasi AI-alat. Server MCP memberikan Claude Code akses ke alat, database, dan API Anda.

Hubungkan server ketika Anda menemukan diri Anda menyalin data ke dalam chat dari alat lain, seperti pelacak masalah atau dasbor pemantauan. Setelah terhubung, Claude dapat membaca dan bertindak pada sistem tersebut secara langsung alih-alih bekerja dari apa yang Anda tempel.

Jika Anda menghubungkan server pertama Anda, mulai dengan [panduan cepat MCP](/docs/id/mcp-quickstart) untuk panduan langkah demi langkah. Halaman ini adalah referensi lengkap.

<h2 id="what-you-can-do-with-mcp">
  Apa yang dapat Anda lakukan dengan MCP
</h2>

Dengan server MCP yang terhubung, Anda dapat meminta Claude Code untuk:

* **Menerapkan fitur dari pelacak masalah**: "Tambahkan fitur yang dijelaskan dalam masalah JIRA ENG-4521 dan buat PR di GitHub."
* **Menganalisis data pemantauan**: "Periksa Sentry dan Statsig untuk memeriksa penggunaan fitur yang dijelaskan dalam ENG-4521."
* **Menanyakan database**: "Temukan email 10 pengguna acak yang menggunakan fitur ENG-4521, berdasarkan database PostgreSQL kami."
* **Mengintegrasikan desain**: "Perbarui template email standar kami berdasarkan desain Figma baru yang diposting di Slack"
* **Mengotomatisasi alur kerja**: "Buat draf Gmail mengundang 10 pengguna ini ke sesi umpan balik tentang fitur baru."
* **Bereaksi terhadap peristiwa eksternal**: Server MCP juga dapat bertindak sebagai [saluran](/docs/id/channels) yang mendorong pesan ke dalam sesi Anda, sehingga Claude bereaksi terhadap pesan Telegram, obrolan Discord, atau peristiwa webhook saat Anda sedang pergi.

<h2 id="find-and-build-mcp-servers">
  Temukan dan bangun server MCP
</h2>

Jelajahi konektor yang telah ditinjau di [Direktori Anthropic](https://claude.ai/directory). Konektor Direktori menggunakan infrastruktur MCP yang sama dengan Claude Code, jadi Anda dapat menambahkan server jarak jauh apa pun yang terdaftar di sana dengan `claude mcp add`.

<Warning>
  Verifikasi bahwa Anda mempercayai setiap server sebelum menghubungkannya. Server yang mengambil konten eksternal dapat mengekspos Anda ke [risiko injeksi prompt](/docs/id/security#protect-against-prompt-injection).
</Warning>

Untuk membangun server Anda sendiri, lihat [panduan server MCP](https://modelcontextprotocol.io/docs/develop/build-server) untuk dasar-dasar protokol dan [dokumentasi pembangun konektor Claude](https://claude.com/docs/connectors/building) untuk autentikasi, pengujian, dan pengajuan Direktori.

Anda juga dapat membuat Claude membangun server untuk Anda dengan plugin resmi [`mcp-server-dev`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/mcp-server-dev).

<Steps>
  <Step title="Instal plugin">
    Dalam sesi Claude Code, jalankan:

    ```
    /plugin install mcp-server-dev@claude-plugins-official
    ```

    Jika instalasi gagal, cocokkan pesan yang dilaporkan Claude Code:

    * `Marketplace "claude-plugins-official" not found`: tambahkan marketplace dengan `/plugin marketplace add anthropics/claude-plugins-official`, kemudian coba lagi instalnya.
    * Plugin [tidak ditemukan di marketplace](/docs/id/plugins/install#install-a-plugin): periksa nama plugin.

    Jika ringkasan instalasi melaporkan `Run /reload-plugins to activate.`, Claude Code kemudian menjalankan reload tersebut untuk Anda. Jika reload memperingatkan bahwa pesan berikutnya Anda akan membaca ulang percakapan, jalankan `/reload-plugins --force`.
  </Step>

  <Step title="Jalankan skill build">
    ```
    /mcp-server-dev:build-mcp-server
    ```

    Claude menanyakan tentang kasus penggunaan Anda dan membangun server HTTP jarak jauh atau server stdio lokal.
  </Step>
</Steps>

<h2 id="installing-mcp-servers">
  Menginstal server MCP
</h2>

Server MCP dapat dikonfigurasi dengan beberapa cara tergantung pada kebutuhan Anda:

<h3 id="option-1-add-a-remote-http-server">
  Opsi 1: Tambahkan server HTTP jarak jauh
</h3>

Server HTTP adalah opsi yang direkomendasikan untuk terhubung ke server MCP jarak jauh. Ini adalah transport yang paling banyak didukung untuk layanan berbasis cloud.

```bash theme={null}
# Sintaks dasar
claude mcp add --transport http <name> <url>

# Contoh nyata: Terhubung ke Notion
claude mcp add --transport http notion https://mcp.notion.com/mcp

# Contoh dengan token Bearer
claude mcp add --transport http secure-api https://api.example.com/mcp \
  --header "Authorization: Bearer your-token"
```

Saat mengonfigurasi server MCP melalui JSON di `.mcp.json`, `~/.claude.json`, atau `claude mcp add-json`, bidang `type` menerima `streamable-http` sebagai alias untuk `http`. Spesifikasi MCP menggunakan nama `streamable-http` untuk transport ini, jadi konfigurasi yang disalin dari dokumentasi server berfungsi tanpa modifikasi.

Entri JSON yang memiliki `url` tetapi tidak ada `type` adalah kesalahan konfigurasi, karena Claude Code membaca entri tanpa `type` sebagai server stdio. Claude Code melewati server itu dan melaporkan `MCP server "<name>" has a "url" but no "type"; add "type": "http" (or "sse" / "ws") to this entry`. Sebelum v2.1.202, Claude Code melaporkan kesalahan konfigurasi ini sebagai `command: expected string, received undefined`.

Dalam `--output-format stream-json` runs, Claude Code juga melaporkan entri `--mcp-config` yang dilewati dalam [`mcp_server_errors` field](/docs/id/headless#stream-responses) dari event `system/init`, sehingga skrip dapat mendeteksi bahwa server tidak pernah dimuat. Ini memerlukan Claude Code v2.1.219 atau lebih baru.

<h3 id="option-2-add-a-remote-sse-server">
  Opsi 2: Tambahkan server SSE jarak jauh
</h3>

<Warning>
  Transport SSE (Server-Sent Events) sudah usang. Gunakan server HTTP sebagai gantinya, jika tersedia.
</Warning>

Beberapa layanan masih hanya mengekspos endpoint SSE. Tambahkan ini dengan perintah `claude mcp add --transport http <name> <url>` yang sama seperti [server HTTP](#option-1-add-a-remote-http-server). Claude Code mencoba transport HTTP terlebih dahulu dan beralih ke SSE ketika server tidak menerimanya. Pengalihan otomatis memerlukan Claude Code v2.1.265 atau lebih baru.

Pada versi sebelumnya, atau untuk terhubung melalui SSE secara langsung, teruskan `--transport sse` sebagai gantinya:

```bash theme={null}
# Sintaks dasar
claude mcp add --transport sse <name> <url>

# Contoh nyata: Terhubung ke Asana
claude mcp add --transport sse asana https://mcp.asana.com/sse

# Contoh dengan header autentikasi
claude mcp add --transport sse private-api https://api.company.com/sse \
  --header "X-API-Key: your-key-here"
```

<h3 id="option-3-add-a-local-stdio-server">
  Opsi 3: Tambahkan server stdio lokal
</h3>

Server stdio berjalan sebagai proses lokal di mesin Anda. Mereka ideal untuk alat yang memerlukan akses sistem langsung atau skrip khusus.

Claude Code menetapkan `CLAUDE_PROJECT_DIR` di lingkungan server yang dihasilkan ke akar proyek, sehingga server Anda dapat menyelesaikan jalur relatif proyek tanpa bergantung pada direktori kerja. Ini adalah direktori yang sama yang diterima hooks dalam variabel `CLAUDE_PROJECT_DIR` mereka. Bacanya dari dalam proses server Anda, misalnya `process.env.CLAUDE_PROJECT_DIR` di Node atau `os.environ["CLAUDE_PROJECT_DIR"]` di Python.

`CLAUDE_PROJECT_DIR` adalah akar proyek yang stabil dan tidak berubah saat Anda menambah atau menghapus direktori kerja di tengah sesi. Server yang membatasi akses sistem file-nya sendiri ke serangkaian direktori yang diizinkan harus mengimplementasikan permintaan MCP `roots/list` sebagai gantinya. Claude Code menjawab `roots/list` dengan direktori peluncuran sesi ditambah setiap [direktori kerja tambahan](/docs/id/permissions#working-directories) yang Anda berikan dengan `--add-dir`, `/add-dir`, atau pengaturan `additionalDirectories`. Claude Code mengirim `notifications/roots/list_changed` saat set itu berubah. Sebelum v2.1.203, `roots/list` hanya mengembalikan direktori peluncuran dan Claude Code tidak mengirim `notifications/roots/list_changed`.

Variabel ini ditetapkan di lingkungan server, bukan di lingkungan Claude Code sendiri, jadi mereferensikannya melalui ekspansi `${VAR}` di `command` atau `args` dari entri `.mcp.json` yang dibatasi proyek atau entri server lokal atau pengguna di `~/.claude.json` memerlukan default seperti `${CLAUDE_PROJECT_DIR:-.}`. Konfigurasi MCP yang disediakan plugin mengganti `${CLAUDE_PROJECT_DIR}` secara langsung dan tidak memerlukan default.

```bash theme={null}
# Sintaks dasar
claude mcp add [options] <name> -- <command> [args...]

# Contoh nyata: Tambahkan server Airtable
claude mcp add --env AIRTABLE_API_KEY=YOUR_KEY --transport stdio airtable \
  -- npx -y airtable-mcp-server
```

<Note>
  **Penting: Pisahkan argumen server dengan `--`**

  Untuk server stdio, `--` (garis miring ganda) memisahkan opsi Claude sendiri, seperti `--transport`, `--env`, dan `--scope`, dari perintah dan argumen yang menjalankan server. Semuanya setelah `--` diteruskan ke server tanpa diubah.

  Sebagai contoh:

  * `claude mcp add --transport stdio myserver -- npx server` → menjalankan `npx server`
  * `claude mcp add --env KEY=value --transport stdio myserver -- python server.py --port 8080` → menjalankan `python server.py --port 8080` dengan `KEY=value` di lingkungan

  Tanpa `--`, Claude Code akan mencoba mengurai flag server, seperti `--port` di atas, sebagai opsinya sendiri.

  `--env` menerima beberapa pasangan `KEY=value`. Jika nama server datang langsung setelah `--env`, CLI membaca nama sebagai pasangan lain dan menolaknya, jadi tempatkan setidaknya satu opsi lain, seperti `--transport stdio`, antara `--env` dan nama server.
</Note>

<h3 id="option-4-add-a-remote-websocket-server">
  Opsi 4: Tambahkan server WebSocket jarak jauh
</h3>

Server WebSocket mempertahankan koneksi bidirectional yang persisten, yang cocok untuk server MCP jarak jauh yang mendorong acara ke Claude tanpa diminta. Gunakan HTTP sebagai gantinya ketika server Anda hanya merespons permintaan, karena HTTP mendukung OAuth dan flag `claude mcp add --transport`, sementara WebSocket tidak mendukung keduanya.

Konfigurasi server WebSocket di `.mcp.json` atau dengan `claude mcp add-json`:

```bash theme={null}
claude mcp add-json events-server \
  '{"type":"ws","url":"wss://mcp.example.com/socket","headers":{"Authorization":"Bearer YOUR_TOKEN"}}'
```

Entri `type: "ws"` menerima bidang `url`, `headers`, `headersHelper`, `timeout`, dan `alwaysLoad` yang sama seperti `http`. Autentikasi hanya header, jadi teruskan token statis di `headers` atau buat satu pada waktu koneksi dengan [`headersHelper`](#use-dynamic-headers-for-custom-authentication). Flag `claude mcp add --transport` tidak menerima `ws`.

<h3 id="add-a-server-from-setup-instructions-written-for-another-client">
  Tambahkan server dari instruksi setup yang ditulis untuk klien lain
</h3>

Server MCP tidak spesifik untuk Claude Code, jadi instruksi setup server mungkin ditulis untuk Claude Desktop, Cursor, atau klien MCP lain dan tidak memberikan perintah `claude mcp add`. Untuk menambahkan server bagaimanapun, cari dalam instruksi itu untuk salah satu dari tiga hal ini:

* **URL** seperti `https://mcp.example.com/mcp`: server adalah jarak jauh.
* **Perintah peluncuran** seperti `npx -y @example/mcp-server`: server berjalan di mesin Anda.
* **Blok JSON `mcpServers`**: konfigurasi yang ditulis untuk file pengaturan klien lain.

Masing-masing adalah salah satu input yang empat opsi di [Menginstal server MCP](#installing-mcp-servers) ambil. Temukan bentuk yang Anda miliki di bawah untuk mengubahnya menjadi perintah yang Claude Code terima. Setiap perintah menulis ke [cakupan lokal](#local-scope) kecuali Anda menambahkan `--scope project` atau `--scope user`.

<h4 id="from-a-url">
  Dari URL
</h4>

URL berarti server adalah jarak jauh. Untuk endpoint `https://`, tambahkan dengan `--transport http`, atau ikuti [Opsi 2](#option-2-add-a-remote-sse-server) ketika instruksi mengatakan endpoint menggunakan SSE. Untuk endpoint `wss://`, gunakan [Opsi 4](#option-4-add-a-remote-websocket-server) sebagai gantinya, karena `--transport` tidak menerima `ws`:

```bash theme={null}
claude mcp add --transport http example https://mcp.example.com/mcp
```

Jika instruksi juga memberikan kunci API atau header token, teruskan dengan `--header` seperti yang ditunjukkan di [Opsi 1](#option-1-add-a-remote-http-server).

<h4 id="from-an-npx-uvx-or-binary-command">
  Dari perintah `npx`, `uvx`, atau binary
</h4>

Perintah peluncuran berarti server berjalan sebagai proses stdio lokal. Letakkan seluruh perintah setelah `--`, sehingga Claude Code meneruskan flag seperti `-y` ke perintah yang memulai server alih-alih membacanya sebagai opsinya sendiri. Teruskan variabel lingkungan apa pun yang diminta instruksi dengan `--env`, setelah nama server dan sebelum `--`:

```bash theme={null}
claude mcp add example --env API_KEY=your-key -- npx -y @example/mcp-server
```

[Opsi 3](#option-3-add-a-local-stdio-server) mencakup pemisah `--` secara lengkap.

<h4 id="from-an-mcpservers-json-block">
  Dari blok JSON `mcpServers`
</h4>

Blok `mcpServers` yang ditulis untuk klien MCP lain, seperti Claude Desktop, menggunakan kunci pembungkus dan bentuk entri yang Claude Code baca. Teruskan `claude mcp add-json` objek di dalam `mcpServers`, bukan pembungkusnya. Dua entri memerlukan perbaikan terlebih dahulu:

* **`url` tanpa `type`**: tambahkan `"type": "http"`, `"type": "sse"`, atau `"type": "ws"` untuk mencocokkan endpoint. Claude Code membaca entri tanpa `type` sebagai server stdio, jadi entri `url` tanpa `type` gagal.
* **Kunci dengan karakter selain huruf, angka, tanda hubung, dan garis bawah**: pilih nama server yang hanya menggunakan karakter tersebut. Jika tidak, kunci adalah nama server.

Sebagai contoh, blok ini:

```json theme={null}
{
  "mcpServers": {
    "example": {
      "command": "npx",
      "args": ["-y", "@example/mcp-server"]
    }
  }
}
```

menjadi perintah ini:

```bash theme={null}
claude mcp add-json example '{"command":"npx","args":["-y","@example/mcp-server"]}'
```

[Tambahkan server MCP dari konfigurasi JSON](#add-mcp-servers-from-json-configuration) mencakup escaping shell dan flag `--scope` untuk `add-json`. Untuk berbagi server dengan tim Anda sebagai gantinya, tambahkan `--scope project`, atau tambahkan entri di bawah `mcpServers` di `.mcp.json` di akar proyek Anda dan commit. [Cakupan proyek](#project-scope) mencakup cara Claude Code memuat dan menyetujui file itu.

Setiap perintah `claude mcp add` dan `claude mcp add-json` mencetak baris `Added ...`. Untuk memeriksa bahwa Claude Code terhubung, jalankan `claude mcp get <name>`; [Status server](#server-status) mencakup status yang ditunjukkannya dan langkah persetujuan untuk server `.mcp.json`.

<h3 id="managing-your-servers">
  Mengelola server Anda
</h3>

Setelah dikonfigurasi, Anda dapat mengelola server MCP Anda dengan perintah ini:

```bash theme={null}
# Daftar semua server yang dikonfigurasi
claude mcp list

# Dapatkan detail untuk server tertentu
claude mcp get notion

# Hapus server
claude mcp remove notion

# (dalam Claude Code) Periksa status server
/mcp
```

Saat Anda menghapus server jarak jauh, Claude Code juga menghapus token OAuth dan registrasi klien yang disimpannya untuk server itu.

<h4 id="server-status">
  Status server
</h4>

`claude mcp add` mengonfirmasi penambahan yang berhasil dengan mencetak baris `Added ...`, yang berarti konfigurasi ditulis. `claude mcp list` kemudian menunjukkan status kesehatan di sebelah setiap server yang dicantumkannya, seperti `✔ Connected`, `! Needs authentication`, atau `✘ Failed to connect`. Status kegagalan berarti Claude Code tidak dapat terhubung ke server itu, bukan bahwa perintah list gagal.

Status dalam daftar ini melaporkan keputusan konfigurasi daripada upaya koneksi, jadi Claude Code mencetaknya tanpa terhubung ke server:

* ``⏸ Pending approval (run `claude` to approve)``: server yang dibatasi proyek dari `.mcp.json` yang belum Anda setujui. Claude Code menunjukkannya di `claude mcp list` dan `claude mcp get <name>`. Jalankan `claude` secara interaktif untuk meninjau dan menyetujuinya.
* `✘ Rejected (see disabledMcpjsonServers in settings)`: server `.mcp.json` yang entri [`disabledMcpjsonServers`](/docs/id/settings-reference#disabledmcpjsonservers) tolak. Claude Code menunjukkannya hanya di `claude mcp get <name>`.
* `⊘ Disabled for this project (re-enable via /mcp)`: server yang daftar [`disabledMcpServers`](#disable-a-server-without-removing-it) proyek namakan. Claude Code menunjukkannya di `claude mcp list` dan `claude mcp get <name>`. Nyalakan server kembali dari panel `/mcp`. Sebelum v2.1.238, kedua perintah terhubung ke server yang dinonaktifkan untuk health-check dan melaporkan hasil koneksi.

Server WebSocket tidak muncul dalam output `claude mcp list`. Gunakan `claude mcp get <name>` atau panel `/mcp` untuk memeriksanya.

<h4 id="project-server-approvals-and-workspace-trust">
  Persetujuan server proyek dan kepercayaan workspace
</h4>

Sejak v2.1.196, `claude mcp list` dan `claude mcp get` membaca persetujuan `.mcp.json` hanya dari file pengaturan yang tidak dimasukkan ke dalam repositori sampai Anda mempercayai workspace dengan menjalankan `claude` di dalamnya dan menerima dialog kepercayaan workspace. Repositori yang dikloning tidak dapat menyetujui server-nya sendiri: [`enableAllProjectMcpServers`](/docs/id/settings-reference#enableallprojectmcpservers) atau [`enabledMcpjsonServers`](/docs/id/settings-reference#enabledmcpjsonservers) yang dicommit ke `.claude/settings.json` proyek diabaikan di folder yang tidak dipercaya, dan server tetap di `⏸ Pending approval` alih-alih terhubung dan health-checked.

Persetujuan dari sumber ini masih berlaku di folder yang tidak dipercaya:

* `~/.claude/settings.json` pengguna Anda
* pengaturan yang dikelola
* pengaturan yang diteruskan dengan `--settings`

Claude Code juga menerapkan persetujuan dari `.claude/settings.local.json` yang tidak dilacak, tetapi menjalankan git untuk memeriksa apakah file dilacak, dan menjalankan pemeriksaan itu hanya di [folder yang dipercaya](/docs/id/permissions#project-allow-rules-and-workspace-trust). Di folder yang belum pernah Anda percayai, Claude Code menunggu dialog kepercayaan sebelum menerapkan persetujuan file, kecuali folder adalah rumah konfigurasi Anda sendiri: direktori rumah Anda, atau direktori yang `.claude` Anda tetapkan sebagai [`CLAUDE_CONFIG_DIR`](/docs/id/env-vars). Sebelum v2.1.207, Claude Code menerapkan persetujuan dari `.claude/settings.local.json` yang tidak dilacak bahkan di folder yang belum pernah Anda percayai.

Entri `disabledMcpjsonServers` di file pengaturan apa pun masih menolak server.

<h4 id="server-status-detail">
  Detail status server
</h4>

Di `/mcp`, termasuk menu server di sana, dan di [manager `/plugin`](/docs/id/plugins/install), server HTTP atau SSE jarak jauh yang pernah Anda gunakan sebelumnya dapat menunjukkan status `cached` seperti `cached 2h ago · connects on first use · 5 tools`. Claude Code memuat daftar alat server dari cache penemuan-nya, disimpan dalam sesi sebelumnya, alih-alih terhubung pada startup, dan Claude Code menghubungkan server pertama kali Claude memanggil salah satu alat server. Alat tersedia dari pesan pertama Anda, jadi Anda tidak perlu melakukan apa pun. Cache penemuan dan status `cached`-nya memerlukan Claude Code v2.1.221 atau lebih baru.

Cache penemuan dimatikan secara default kecuali rollout bertahap telah mengaktifkannya untuk akun Anda. Atur [`MCP_DISCOVERY_CACHE=1`](/docs/id/env-vars) untuk mengaktifkannya, atau `0` untuk tetap mematikannya bahkan ketika rollout telah mengaktifkannya. Sebelum v2.1.238, cache diaktifkan secara default.

Dua tindakan dalam menu server di `/mcp` juga mempengaruhi entri cache server itu:

* **Reconnect**: pada server `cached`, Claude Code menghubungkannya sekarang daripada pada panggilan alat pertamanya dan menyimpan entri. Pada server yang terhubung atau gagal, Claude Code menghubungkannya kembali dan juga membuang entri.
* **Clear authentication**: Claude Code mencabut autentikasi server dan juga membuang entri.

Setelah membuang entri, Claude Code mengambil daftar alat server dari server alih-alih dari cache.

Ketika status server adalah `✘ Failed to connect`, `claude mcp list` menambahkan detail kegagalan ke baris status itu, dan `claude mcp get <name>` menunjukkannya pada baris `Issue:`: status HTTP atau kode kesalahan, ditambah teks kesalahan apa pun yang dikembalikan server. Tampilan detail server di `/mcp` mencakup teks yang dilaporkan server yang sama di baris `Issue:` nya. Claude Code menyunting teks seperti kredensial dari detail ini dan tidak pernah menyertakan URL server yang diperluas, yang dapat membawa rahasia. Claude Code tidak menambahkan detail ke status `✘ Connection error`, karena teks pengecualian yang akan dicetak di sana dapat menyematkan URL itu. Sebelum v2.1.219, kedua perintah menunjukkan hanya status kegagalan telanjang, tanpa kode status atau teks kesalahan server.

Ketika Anda menyelesaikan autentikasi dari `/mcp` dan koneksi masih gagal dengan status HTTP atau kode kesalahan transport, Claude Code menambahkan kode itu dan asal URL yang dicobanya ke pesan yang dicetak setelah upaya. Asal adalah skema dan host, ditambah port ketika URL menamakannya, seperti `https://mcp.example.com`.

* Jalur dan kueri tidak pernah muncul dalam pesan itu.
* Untuk server dalam [cakupan](#mcp-installation-scopes) lokal, proyek, atau pengguna atau dalam konfigurasi MCP yang dikelola, asal menunjukkan host seperti yang ditulis dalam konfigurasi itu, jadi referensi `${VAR}` di host tidak diperluas dalam pesan.
* Untuk kegagalan tanpa status atau kode kesalahan, Claude Code menunjukkan teks kesalahan tanpa asal.

Server jarak jauh yang konfigurasinya memiliki `url` kosong ditampilkan sebagai `not configured` di `/mcp`, di `claude mcp list`, dan di [manager `/plugin`](/docs/id/plugins/install), dan Claude Code tidak mencoba terhubung ke sana. Plugin dapat menyertakan entri placeholder seperti ini untuk konektor yang Anda konfigurasi nanti, jadi Claude Code tidak melaporkannya sebagai kesalahan atau masalah setup. Tampilan detail server di `/mcp` membaca `No URL configured for this server`; atur `url` entri untuk menghubungkannya. Sebelum v2.1.208, Claude Code melaporkan `url` kosong sebagai masalah konfigurasi dengan prompt untuk menghubungkan kembali.

<h4 id="configuration-warnings">
  Peringatan konfigurasi
</h4>

Claude Code memperingatkan tentang masalah konfigurasi di bawah. Setiap entri mengatakan apa yang Claude Code periksa dan cara menghapus peringatan:

* **Whitespace tersembunyi**: Claude Code memperingatkan ketika nilai config MCP membawa whitespace terkemuka atau tertinggal yang tersembunyi, yang sering berasal dari menempel token dengan newline tertinggal. Claude Code memeriksa `command`, `url`, setiap entri `args`, dan nilai dan nama kunci di bawah `env` dan `headers`. Claude Code menunjukkan peringatan dalam output `claude mcp list` dan di `/mcp`, menamai bidang yang terpengaruh tanpa menggema nilainya, misalnya `Leading or trailing whitespace in: headers.Authorization`. Claude Code tidak memangkas whitespace dan menggunakan nilai persis seperti yang ditulis, jadi edit konfigurasi untuk menghapusnya.
* **Nama yang sama di lebih dari satu cakupan**: jika Anda mendefinisikan nama server yang sama di lebih dari satu [cakupan](#mcp-installation-scopes) dengan endpoint berbeda, Claude Code memperingatkan tentang konflik dalam output `claude mcp list` dan di `/mcp`. Claude Code menyimpan sign-in OAuth per endpoint, jadi ketika Anda mengautentikasi definisi yang dimuat dalam satu proyek, Anda masih perlu sign in secara terpisah di proyek di mana definisi berbeda dimuat. Simpan endpoint yang Anda inginkan dan hapus yang lain dengan `claude mcp remove <name> --scope <scope>`. Dalam peringatan, Claude Code mengutip endpoint setiap cakupan seperti yang ditulis dalam konfigurasi Anda, dengan referensi [`${VAR}`](#environment-variable-expansion-in-mcp-json) tidak diperluas, jadi tidak pernah menunjukkan nilai yang diselesaikan seperti kunci API.
* **Nama yang dicadangkan**: Claude Code mencadangkan nama server bawaan-nya, termasuk `workspace`, `claude-in-chrome`, `computer-use`, `Claude Preview`, dan `Claude Browser`. Jika konfigurasi Anda mendefinisikan server dengan nama yang dicadangkan, Claude Code melewatinya pada waktu muat dan menunjukkan peringatan meminta Anda untuk mengganti namanya. `claude mcp add` menolak nama yang dicadangkan dengan kesalahan. `Claude Preview` dan `Claude Browser` keduanya menamai server bawaan yang digunakan [pane preview aplikasi desktop Claude Code](/docs/id/desktop#preview-your-app). Sebelum v2.1.205, `Claude Browser` tidak dicadangkan, jadi server yang dikonfigurasi pengguna dapat mendaftar di bawah nama itu.
* **Variabel lingkungan yang hilang**: jika referensi [`${VAR}`](#environment-variable-expansion-in-mcp-json) dalam konfigurasi server menamai variabel yang tidak diatur dan tidak memiliki `:-default`, Claude Code memperingatkan dalam output `claude mcp list` dan di `/mcp`, menamai variabel, dan masih memuat server dengan teks `${VAR}` tidak diperluas. Atur variabel atau tambahkan fallback `${VAR:-default}`. Dalam URL dan `headers` server jarak jauh, beberapa variabel kredensial [membaca sebagai kosong](#credential-variables-that-read-as-empty) sebagai gantinya, tanpa peringatan.

<h4 id="tool-availability">
  Ketersediaan alat
</h4>

Panel `/mcp` menunjukkan jumlah alat di sebelah setiap server yang terhubung dan menandai server yang mengiklankan kemampuan alat tetapi tidak mengekspos alat.

Jika permintaan Anda memerlukan alat dari server yang masih terhubung di latar belakang, Claude menunggu server itu sebelum melanjutkan. Cara menunggu terjadi tergantung pada konfigurasi Anda:

* **Dengan [pencarian alat](#scale-with-mcp-tool-search), default**: menunggu terjadi di dalam panggilan `ToolSearch`.
* **Tanpa pencarian alat**: Claude menggunakan alat `WaitForMcpServers` sebagai gantinya. Konfigurasi tanpa pencarian alat mencakup `ANTHROPIC_BASE_URL` khusus, `ENABLE_TOOL_SEARCH=false`, dan model lebih awal dari generasi Claude 4.5 di Agent Platform Google Cloud.
* **Pada [deployment Microsoft Foundry yang dihosting di Azure](https://platform.claude.com/docs/en/build-with-claude/claude-in-microsoft-foundry#hosting-options)**: Claude dimulai pada jalur pencarian alat daripada dengan `WaitForMcpServers`, karena Claude Code menemukan penolakan sisi server deployment hanya dari API. Setelah Claude Code beralih deployment itu ke [pemuatan awal](#scale-with-mcp-tool-search), alat dari server yang selesai terhubung menjadi tersedia pada permintaan Claude berikutnya.

Dengan pencarian alat diaktifkan, ketika server selesai terhubung saat Claude bekerja, Claude Code mencantumkan nama alat server ke Claude pada permintaan berikutnya dalam giliran yang sama. Claude kemudian dapat mencari dan memanggil alat itu tanpa menunggu pesan Anda berikutnya.

<h3 id="disable-a-server-without-removing-it">
  Nonaktifkan server tanpa menghapusnya
</h3>

Alihkan server di panel `/mcp` untuk menghentikan Claude Code dari terhubung ke sana tanpa kehilangan konfigurasinya. Claude Code masih mencantumkan server di `/mcp`, ditandai sebagai dinonaktifkan.

Ketika Anda mengalihkan server, Claude Code mencatat pilihan Anda per proyek di `~/.claude.json`, dalam salah satu dari dua daftar yang mencakup set server yang terpisah:

* `disabledMcpServers`: daftar opt-out untuk server yang dikonfigurasi pengguna, server plugin, server yang organisasi Anda [sediakan melalui pengaturan yang dikelola](/docs/id/managed-mcp#provide-servers-through-managed-settings), konektor claude.ai yang Claude Code [ambil sendiri](#how-connectors-reach-claude-code), dan server bawaan yang default ke on. Claude Code tidak terhubung ke server yang Anda cantumkan di sini. Ketika Anda menonaktifkan konektor claude.ai dengan toggle `/mcp` per-proyek yang dijelaskan di [Nonaktifkan konektor claude.ai](#disable-claude-ai-connectors), Claude Code menulisnya ke daftar ini di bawah nama tampilan-nya, misalnya `claude.ai Slack`.
* `enabledMcpServers`: daftar opt-in untuk server bawaan yang default ke off, seperti `computer-use`. Claude Code terhubung ke server default-off hanya ketika Anda mencantumkannya di sini.

Claude Code berkonsultasi dengan tepat satu dari dua daftar untuk setiap server, jadi tidak ada daftar yang mengesampingkan yang lain. Jika Anda menambahkan server reguler ke `enabledMcpServers`, atau server bawaan default-off ke `disabledMcpServers`, Claude Code mengabaikan entri.

`disabledMcpServers` dan `enabledMcpServers` tidak terkait dengan [`enabledMcpjsonServers`](/docs/id/settings-reference#enabledmcpjsonservers) dan [`disabledMcpjsonServers`](/docs/id/settings-reference#disabledmcpjsonservers), yang mengontrol persetujuan server yang didefinisikan dalam file `.mcp.json` proyek.

<h3 id="mcp-client-runtimes">
  Runtime klien MCP
</h3>

Claude Code terhubung ke server MCP melalui salah satu dari dua runtime klien. Runtime v1 dibangun di MCP TypeScript SDK 1.x. Runtime v2 adalah kode yang sama di [MCP TypeScript SDK 2.0](https://ts.sdk.modelcontextprotocol.io/v2/), yang menambahkan revisi protokol MCP 2026-07-28. Sisa halaman ini berlaku untuk kedua runtime, kecuali bagian yang menamai runtime v2.

Claude Code memilih runtime setiap kali Anda memulainya dan menyimpannya sampai Anda keluar. Dalam sesi di mana ia [mengambil flag fitur](/docs/id/env-vars#features-that-need-feature-flag-fetching), ia menggunakan runtime v2 pada Claude Code v2.1.232 atau lebih baru.

Dalam sesi di mana ia tidak mengambil flag fitur, Claude Code menggunakan runtime v2 secara default pada Claude Code v2.1.274 atau lebih baru:

* Sesi di Amazon Bedrock, Claude Platform di AWS, Agent Platform Google Cloud, atau Microsoft Foundry, kecuali platform host yang menyematkan Claude Code menetapkan [`CLAUDE_CODE_PROVIDER_MANAGED_BY_HOST`](/docs/id/env-vars)
* Sesi yang masuk melalui [gateway aplikasi Claude](/docs/id/claude-apps-gateway)
* Sesi di mana Anda mematikan telemetri atau pengambilan flag fitur, misalnya dengan `DISABLE_TELEMETRY`

Pada v2, Claude Code juga:

* Menanyakan server HTTP apakah mereka mendukung revisi yang lebih baru, dan menggunakannya dengan yang mendukung. Ia juga menanyakan server konektor claude.ai dalam sesi di mana ia mengambil flag fitur. Untuk membuatnya menanyakan server stdio, atau server konektor dalam setiap sesi, atur [`MCP_PROTOCOL_NEGOTIATION`](/docs/id/env-vars) ke `auto`. Ia terhubung ke setiap server lain seperti v1 lakukan.
* Menerima notifikasi `list_changed` dari server pada revisi yang lebih baru melalui [stream yang dipegang terbuka](#notification-streams-on-the-v2-runtime).
* Tidak mendaftarkan server [channel](#push-messages-with-channels) yang terhubung pada revisi yang lebih baru, karena revisi itu tidak dapat membawa pesan channel.
* Gagal [sign-in OAuth MCP](#authenticate-with-remote-mcp-servers) yang respons otorisasinya menamai penerbit yang tidak terduga.

Anthropic dapat menyimpan server tertentu pada protokol sebelumnya, atau off stream itu, dengan flag fitur Claude Code ambil.

Untuk memilih runtime sendiri, atur [`MCP_SDK_GENERATION`](/docs/id/env-vars) ke `v1` atau `v2`. Untuk memutuskan apakah Claude Code menanyakan, atur [`MCP_PROTOCOL_NEGOTIATION`](/docs/id/env-vars) ke `auto` atau `legacy`.

<h3 id="dynamic-tool-updates">
  Pembaruan alat dinamis
</h3>

Claude Code mendukung notifikasi MCP `list_changed`, memungkinkan server MCP untuk secara dinamis memperbarui alat, prompt, dan sumber daya mereka yang tersedia tanpa memerlukan Anda untuk memutuskan dan menghubungkan kembali. Ketika server MCP mengirim notifikasi `list_changed`, Claude Code secara otomatis menyegarkan kemampuan yang tersedia dari server itu.

Jika permintaan penyegaran gagal, Claude Code menyimpan alat, prompt, dan sumber daya server yang sebelumnya ditemukan sampai penyegaran nanti berhasil. Sebelum v2.1.214, kesalahan sementara selama penyegaran mengganti alat, prompt, dan sumber daya server dengan daftar kosong.

<h4 id="notification-streams-on-the-v2-runtime">
  Stream notifikasi pada runtime v2
</h4>

Pada [runtime v2](#mcp-client-runtimes), Claude Code menerima notifikasi `list_changed` dari server pada revisi protokol yang lebih baru melalui stream yang dipegang terbuka. Ketika stream ditutup, Claude Code membukanya kembali, dengan dua batas:

* **Stream ditutup lagi dalam 10 detik**: Claude Code membukanya kembali hingga tiga kali, kemudian berhenti untuk koneksi itu.
* **Stream tetap terbuka lebih lama dari 10 detik, kemudian ditutup**, seperti stream ke host serverless biasanya lakukan: setelah lima pembukaan kembali dalam satu jam, Claude Code menunggu sekitar enam jam sebelum yang berikutnya.

Sampai stream dibuka kembali, Anda menyimpan alat, prompt, dan sumber daya server yang terakhir diambil. Untuk mengambil perubahannya lebih cepat, hubungkan kembali server dari `/mcp`.

<h3 id="automatic-reconnection">
  Koneksi ulang otomatis
</h3>

Claude Code menghubungkan kembali server jarak jauh yang putus di tengah sesi dan mencoba ulang koneksi pertama server HTTP atau SSE setelah kesalahan sementara. Server stdio adalah proses lokal, dan Claude Code tidak menghubungkan kembali mereka secara otomatis.

<h4 id="mid-session-drops-of-a-remote-server">
  Putus di tengah sesi dari server jarak jauh
</h4>

Claude Code menghubungkan kembali server jarak jauh yang putus dengan backoff eksponensial: hingga lima upaya, dimulai dengan penundaan satu detik dan menggandakannya setiap kali. Apa yang Anda lihat tergantung pada cara Anda menjalankan Claude Code:

* **Dalam sesi interaktif**: `/mcp` menunjukkan server sebagai tertunda saat Claude Code menghubungkan kembali. Setelah lima upaya gagal, Claude Code menandai server sebagai gagal, atau sebagai memerlukan autentikasi ketika server perlu diotorisasi lagi. Anda dapat mencoba ulang secara manual dari `/mcp`.
* **Dalam [`claude -p`](/docs/id/headless) runs dan sesi [Agent SDK](/docs/id/agent-sdk/overview)**: Claude Code menghubungkan kembali pada jadwal yang sama, tanpa panel `/mcp` untuk menunjukkan upaya.

<h4 id="failed-first-connections">
  Koneksi pertama yang gagal
</h4>

Ketika koneksi pertama server HTTP atau SSE gagal dengan kesalahan sementara, seperti respons 5xx, koneksi ditolak, atau timeout, Claude Code mencoba ulang hingga tiga kali. Jika koneksi masih gagal, Claude Code menandai server sebagai gagal. Claude Code mencoba ulang dengan cara ini pada startup dan ketika server ditambahkan di tengah sesi. Itu termasuk server yang Claude Code tambahkan ke sesi [cloud](/docs/id/claude-code-on-the-web) dari konfigurasinya dan server yang Anda tambahkan dengan [`setMcpServers()`](/docs/id/agent-sdk/typescript) Agent SDK.

Claude Code tidak mencoba ulang dalam kasus ini:

* Koneksi pertama server WebSocket
* Kesalahan autentikasi atau not-found, karena memerlukan perubahan konfigurasi untuk diselesaikan. Ketika [`headersHelper`](#use-dynamic-headers-for-custom-authentication) adalah satu-satunya sumber header `Authorization` server, Claude Code mencoba ulang kesalahan autentikasi bagaimanapun, karena menjalankan kembali helper pada setiap upaya dan dapat mengambil kredensial segar

<h4 id="failed-discovery-requests">
  Permintaan penemuan yang gagal
</h4>

Setelah server terhubung, Claude Code mengirimnya permintaan penemuan kemampuan seperti `tools/list`, `prompts/list`, dan `resources/list`. Claude Code mencoba ulang permintaan itu hingga tiga kali dengan backoff pendek setelah kesalahan jaringan atau server sementara. Tidak mencoba ulang kesalahan autentikasi, respons 4xx, atau timeout permintaan.

<h4 id="how-claude-learns-that-a-server-failed">
  Bagaimana Claude belajar bahwa server gagal
</h4>

Apakah Claude Code memberi tahu Claude tentang server yang dikonfigurasi yang gagal terhubung tergantung pada [pencarian alat](#scale-with-mcp-tool-search), yang diaktifkan secara default:

* Dengan pencarian alat, Claude Code memberi tahu Claude server mana yang gagal dan kesalahan koneksinya, jadi Claude melaporkan kegagalan koneksi dalam responsnya. Claude Code menyertakan informasi yang sama dalam hasil `ToolSearch` yang tidak menemukan alat yang cocok.
* Dalam [konfigurasi apa pun tanpa pencarian alat](#configure-tool-search), Claude Code tidak melaporkan koneksi server yang gagal ke Claude.

<h3 id="push-messages-with-channels">
  Dorong pesan dengan channel
</h3>

Server MCP juga dapat mendorong pesan langsung ke sesi Anda sehingga Claude dapat bereaksi terhadap acara eksternal seperti hasil CI, peringatan pemantauan, atau pesan chat. Untuk mengaktifkan ini, server Anda mendeklarasikan kemampuan `claude/channel` dan Anda memilihnya dengan flag `--channels` pada startup. Lihat [Channels](/docs/id/channels) untuk menggunakan channel yang didukung secara resmi, atau [Channels reference](/docs/id/channels-reference) untuk membangun milik Anda sendiri.

Pada [runtime v2](#mcp-client-runtimes), jika Anda menetapkan [`MCP_PROTOCOL_NEGOTIATION`](/docs/id/env-vars) ke `auto` dan server channel menegosiasikan revisi protokol MCP 2026-07-28, tidak dapat mengirimkan pesan channel, jadi Claude Code tidak mendaftarkannya sebagai channel. Membiarkan variabel tidak diatur, atau menetapkannya ke `legacy`, menyimpan server stdio pada handshake sebelumnya.

<Tip>
  Tips:

  * Gunakan flag `-s` atau `--scope` untuk menentukan di mana konfigurasi disimpan:
    * `local` (default): tersedia hanya untuk Anda di proyek saat ini
    * `project`: dibagikan dengan semua orang di proyek melalui file `.mcp.json`
    * `user`: tersedia untuk Anda di semua proyek
  * Atur variabel lingkungan dengan flag `-e` atau `--env` (misalnya, `-e KEY=value`)
  * Flag `--transport` dan `--header` juga menerima bentuk pendek `-t` dan `-H`
  * Konfigurasi timeout startup server MCP menggunakan variabel lingkungan `MCP_TIMEOUT` (misalnya, `MCP_TIMEOUT=10000 claude` menetapkan timeout 10 detik)
  * Atur timeout eksekusi alat per-server dengan menambahkan bidang `timeout` dalam milidetik ke entri `.mcp.json` server itu, misalnya `"timeout": 600000` untuk sepuluh menit. Ini mengesampingkan variabel lingkungan `MCP_TOOL_TIMEOUT` hanya untuk server itu
  * Claude Code menampilkan peringatan ketika output alat MCP melebihi 10.000 token dan membatasi output ke 25.000 token secara default. Untuk menaikkan batas, atur variabel lingkungan `MAX_MCP_OUTPUT_TOKENS` (misalnya, `MAX_MCP_OUTPUT_TOKENS=50000`); ambang peringatan tetap. Lihat [Batas dan peringatan output MCP](#mcp-output-limits-and-warnings)
  * Gunakan `/mcp` untuk mengautentikasi dengan server jarak jauh yang memerlukan autentikasi OAuth 2.0
</Tip>

`timeout` per-server adalah batas jam dinding keras per panggilan alat, dan notifikasi kemajuan dari server tidak memperpanjangnya. Nilai di bawah 1000 diabaikan dan jatuh melalui `MCP_TOOL_TIMEOUT`, atau default-nya sekitar 28 jam ketika variabel itu tidak diatur. Untuk server HTTP, SSE, atau [konektor claude.ai](/docs/id/mcp#use-mcp-servers-from-claude-ai) ada juga timer per-permintaan kedua yang mencakup setiap permintaan melalui byte respons pertama server. Claude Code menetapkan timer itu ke yang terbesar dari tiga nilai: 60 detik, timeout alat yang berlaku untuk server, dan `MCP_TIMEOUT`. Default 28 jam dari `MCP_TOOL_TIMEOUT` yang tidak diatur tidak memasuki perbandingan itu, dan nilai di bawah 60 detik tidak mempersingkat timer. Server stdio dan WebSocket tidak memiliki timer per-permintaan.

`timeout` per-server setidaknya 1000 juga bertindak sebagai lantai pada timeout idle yang dijelaskan di bawah: Claude Code tidak pernah menghentikan panggilan alat server itu karena kemalasan lebih cepat dari `timeout` per-server. Memerlukan Claude Code v2.1.203 atau lebih baru.

Panggilan alat ke server MCP yang mengirim tidak ada respons dan tidak ada notifikasi kemajuan untuk jendela idle membatalkan dengan kesalahan alih-alih menunggu batas jam dinding. Ini berlaku untuk setiap jenis server kecuali server IDE dan server in-process SDK. Jendela idle default ke lima menit untuk server HTTP, SSE, WebSocket, dan [konektor claude.ai](#use-mcp-servers-from-claude-ai), dan ke 30 menit untuk server stdio. Sebelum v2.1.203, server stdio dikecualikan dari timeout idle.

Atur variabel lingkungan [`CLAUDE_CODE_MCP_TOOL_IDLE_TIMEOUT`](/docs/id/env-vars) dalam milidetik untuk mengubah jendela idle, atau atur ke `0` untuk menonaktifkan pemeriksaan.

Timeout ini membatasi berapa lama panggilan dapat berjalan, tidak selalu berapa lama itu memblokir sesi: panggilan percakapan utama yang berjalan melewati dua menit bergerak ke tugas latar belakang terlebih dahulu. Lihat [Backgrounding otomatis panggilan alat panjang](#automatic-backgrounding-of-long-tool-calls).

<h3 id="automatic-backgrounding-of-long-tool-calls">
  Backgrounding otomatis panggilan alat panjang
</h3>

Panggilan alat MCP dalam percakapan utama yang masih berjalan setelah dua menit bergerak ke tugas latar belakang alih-alih memblokir sesi. Claude menerima ID tugas segera dan terus bekerja, dan hasilnya tiba sebagai notifikasi tugas ketika panggilan diselesaikan. Backgrounding otomatis memerlukan Claude Code v2.1.212 atau lebih baru.

Tugas muncul di [`/tasks`](/docs/id/commands#all-commands), di mana Anda juga dapat menghentikannya, dan tidak bertahan keluar dari sesi. Batas per-panggilan masih berlaku saat panggilan berjalan di latar belakang: batas jam dinding yang ditetapkan oleh `timeout` per-server atau [`MCP_TOOL_TIMEOUT`](/docs/id/env-vars), dan timeout idle yang ditetapkan oleh [`CLAUDE_CODE_MCP_TOOL_IDLE_TIMEOUT`](/docs/id/env-vars).

Atur variabel lingkungan [`CLAUDE_CODE_MCP_AUTO_BACKGROUND_MS`](/docs/id/env-vars) dalam milidetik untuk mengubah ambang, atau atur ke `0` untuk menonaktifkan backgrounding otomatis. Menetapkan `CLAUDE_CODE_DISABLE_BACKGROUND_TASKS` ke `1` juga menonaktifkannya, bersama dengan semua fitur tugas latar belakang lainnya.

Beberapa panggilan tidak pernah bergerak ke latar belakang:

* Panggilan dari [subagents](/docs/id/sub-agents); Claude Code hanya melatar belakangkan panggilan percakapan utama
* Panggilan ke server IDE
* Panggilan dalam [mode non-interaktif](/docs/id/headless), kecuali `CLAUDE_AUTO_BACKGROUND_TASKS` diatur ke `1`, karena run satu kali dapat berakhir sebelum hasil tiba

Panggilan menunggu dialog [elicitation](#respond-to-mcp-elicitation-requests) terbuka tidak dilatar belakangkan saat dialog terbuka; server diblokir pada input Anda, bukan lambat, jadi Claude Code menunda perpindahan sampai dialog ditutup.

<h3 id="plugin-provided-mcp-servers">
  Server MCP yang disediakan plugin
</h3>

[Plugins](/docs/id/plugins/overview) dapat menggabungkan server MCP yang menyediakan alat dan integrasi ketika Anda mengaktifkan plugin. Server MCP plugin bekerja identik dengan server yang dikonfigurasi pengguna.

**Cara kerja server MCP plugin**:

* Plugin mendefinisikan server MCP di `.mcp.json` di akar plugin atau inline di `plugin.json`
* Ketika Anda mengaktifkan plugin, Claude Code memulai server MCP-nya secara otomatis
* Claude Code menawarkan alat MCP plugin bersama alat MCP yang dikonfigurasi secara manual
* Anda menambah dan menghapus server plugin dengan memasang atau mencopot plugin, bukan dengan perintah `/mcp`. Anda masih dapat [mengalihkan server plugin yang dipasang off](#disable-a-server-without-removing-it) di `/mcp`, yang menghentikan Claude Code dari terhubung ke sana tanpa menghapus plugin

**Contoh konfigurasi MCP plugin**:

Di `.mcp.json` di akar plugin:

```json theme={null}
{
  "mcpServers": {
    "database-tools": {
      "command": "${CLAUDE_PLUGIN_ROOT}/servers/db-server",
      "args": ["--config", "${CLAUDE_PLUGIN_ROOT}/config.json"],
      "env": {
        "DB_URL": "${DB_URL}"
      }
    }
  }
}
```

Atau inline di `plugin.json`:

```json theme={null}
{
  "name": "my-plugin",
  "mcpServers": {
    "plugin-api": {
      "command": "${CLAUDE_PLUGIN_ROOT}/servers/api-server",
      "args": ["--port", "8080"]
    }
  }
}
```

**Fitur MCP plugin**:

* **Siklus hidup otomatis**: server terhubung dan terputus pada titik-titik ini:
  * Pada startup sesi, Claude Code menghubungkan server untuk plugin yang diaktifkan secara otomatis. Di `/mcp`, server plugin jarak jauh (HTTP atau SSE) yang pernah Anda gunakan sebelumnya dapat menunjukkan status [`cached`](#server-status-detail) sebagai gantinya; Claude Code menghubungkannya ketika Claude pertama kali memanggil salah satu alatnya
  * Jika Anda mengaktifkan atau menonaktifkan plugin selama sesi, Claude Code menghubungkan atau memutuskan server MCP-nya ketika perubahan diterapkan. [Terapkan perubahan plugin tanpa memulai ulang](/docs/id/plugins/cli-reference#reload-plugins) menjelaskan kapan itu terjadi. Dalam sesi tanpa terminal interaktif, `/reload-plugins` tidak menghubungkan atau memutuskan server MCP plugin; perubahan itu berlaku dalam sesi Anda berikutnya
  * Ketika Anda memuat ulang, Claude Code menyimpan koneksi langsung dari server plugin yang konfigurasinya tidak berubah, dan melakukan hal yang sama ketika Anda [mengganti daftar server MCP sesi](/docs/id/agent-sdk/typescript#mcpsetserversresult) dari Agent SDK tanpa menamakannya
  * Ketika Anda [memindahkan sesi dengan `/cd`](/docs/id/permissions#move-the-session-to-another-directory) pada v2.1.246 atau lebih baru, Claude Code menghubungkan server plugin yang pengaturan direktori baru aktifkan dan memutuskan server plugin yang tidak lagi diaktifkan, jadi Anda tidak perlu menjalankan `/reload-plugins` setelah perpindahan
  * Dalam [sesi cloud](/docs/id/claude-code-on-the-web), panggilan MCP ke server plugin yang belum terhubung, seperti tepat setelah sesi idle bangun, memulai server sesuai permintaan dan menunggu untuk terhubung
* **Placeholder jalur**: `${CLAUDE_PLUGIN_ROOT}` diselesaikan ke direktori instalasi plugin, `${CLAUDE_PLUGIN_DATA}` ke direktori [status persisten](/docs/id/plugins/components#path-variables-and-persistent-data)-nya, dan `${CLAUDE_PROJECT_DIR}` ke akar proyek yang stabil. Substitusi berlaku untuk:
  * server `stdio`: `command`, `args`, `env`
  * server `http`, `sse`, dan `ws`: `url`, `headers`, dan `headersHelper`. Sebelum v2.1.195, `headersHelper` meneruskan placeholder sebagai string literal
* **Akses lingkungan pengguna**: akses ke variabel lingkungan yang sama seperti server yang dikonfigurasi secara manual
* **Jenis transport berganda**: dukungan untuk transport stdio, SSE, HTTP, dan WebSocket, meskipun dukungan transport dapat bervariasi menurut server

Server plugin muncul di `/mcp` dengan indikator menunjukkan mereka berasal dari plugin.

**Nama alat MCP plugin**:

Alat dari server MCP yang digabungkan plugin mencakup nama plugin dan kunci server dalam nama yang dapat dipanggil mereka. Bentuk lengkapnya adalah `mcp__plugin_<plugin-name>_<server-name>__<tool-name>`, di mana karakter apa pun di luar `A-Z`, `a-z`, `0-9`, `_`, dan `-` diganti dengan `_`. Untuk server `database-tools` yang digabungkan dalam plugin bernama `my-plugin`, alat `query` dapat dipanggil sebagai:

```
mcp__plugin_my-plugin_database-tools__query
```

Gunakan nama lengkap ini saat mereferensikan alat dalam [aturan izin](/docs/id/permissions), daftar `allowed-tools` skill, bidang `tools` [subagent](/docs/id/sub-agents#available-tools), atau [pencocokan hook](/docs/id/hooks#match-mcp-tools). Pencocokan hook yang ditulis terhadap kunci server telanjang, seperti `mcp__database-tools__.*`, tidak pernah menyala untuk server yang digabungkan plugin.

Server itu sendiri mendaftar di bawah nama yang dibatasi `plugin:<plugin-name>:<server-name>`, seperti `plugin:my-plugin:database-tools`. Gunakan nama itu di mana nama server yang dikonfigurasi diharapkan, seperti [`server` field hook `mcp_tool`](/docs/id/hooks#mcp-tool-hook-fields).

Lihat [referensi komponen plugin](/docs/id/plugins/components#mcp-servers) untuk detail tentang menggabungkan server MCP dengan plugin.

<h2 id="mcp-installation-scopes">
  Cakupan instalasi MCP
</h2>

Server MCP dapat dikonfigurasi pada tiga cakupan berbeda. Cakupan yang Anda pilih mengontrol proyek mana tempat server dimuat dan apakah konfigurasi dibagikan dengan tim Anda. Administrator juga dapat menerapkan atau menyediakan server untuk setiap pengguna melalui [konfigurasi terkelola](#managed-mcp-configuration).

| Cakupan                  | Dimuat dalam          | Dibagikan dengan tim      | Disimpan dalam             |
| ------------------------ | --------------------- | ------------------------- | -------------------------- |
| [Lokal](#local-scope)    | Hanya proyek saat ini | Tidak                     | `~/.claude.json`           |
| [Proyek](#project-scope) | Hanya proyek saat ini | Ya, melalui kontrol versi | `.mcp.json` di root proyek |
| [Pengguna](#user-scope)  | Semua proyek Anda     | Tidak                     | `~/.claude.json`           |

<h3 id="local-scope">
  Cakupan lokal
</h3>

Cakupan lokal adalah default. Server dengan cakupan lokal hanya dimuat di proyek tempat Anda menambahkannya dan tetap pribadi untuk Anda. Claude Code menyimpannya dalam `~/.claude.json` di bawah jalur proyek tersebut, jadi server yang sama tidak akan muncul di proyek lain Anda. Gunakan cakupan lokal untuk server pengembangan pribadi, konfigurasi eksperimental, atau server dengan kredensial yang tidak ingin Anda masukkan ke dalam kontrol versi.

<Note>
  Istilah "cakupan lokal" untuk server MCP berbeda dari pengaturan lokal umum. Server MCP dengan cakupan lokal disimpan dalam `~/.claude.json` (direktori home Anda), sementara pengaturan lokal umum menggunakan `.claude/settings.local.json` (di direktori proyek). Lihat [Pengaturan](/docs/id/settings#where-settings-live) untuk detail tentang lokasi file pengaturan.
</Note>

```bash theme={null}
# Tambahkan server dengan cakupan lokal (default)
claude mcp add --transport http stripe https://mcp.stripe.com

# Tentukan cakupan lokal secara eksplisit
claude mcp add --transport http stripe --scope local https://mcp.stripe.com
```

Perintah menulis server ke dalam entri untuk proyek saat ini di dalam `~/.claude.json`. Contoh di bawah menunjukkan hasilnya ketika Anda menjalankannya dari `/path/to/your/project`:

```json theme={null}
{
  "projects": {
    "/path/to/your/project": {
      "mcpServers": {
        "stripe": {
          "type": "http",
          "url": "https://mcp.stripe.com"
        }
      }
    }
  }
}
```

<h3 id="project-scope">
  Cakupan proyek
</h3>

Server dengan cakupan proyek memungkinkan kolaborasi tim dengan menyimpan konfigurasi dalam file `.mcp.json` di direktori root proyek Anda. Ketika Anda menambahkan server dengan cakupan proyek, Claude Code secara otomatis membuat atau memperbarui file ini dengan struktur konfigurasi yang sesuai. Periksa `.mcp.json` ke dalam kontrol versi sehingga semua orang di tim Anda mendapatkan alat dan layanan MCP yang sama.

```bash theme={null}
# Tambahkan server dengan cakupan proyek
claude mcp add --transport http shared-server --scope project https://example.com/mcp
```

File `.mcp.json` yang dihasilkan mengikuti format standar:

```json theme={null}
{
  "mcpServers": {
    "shared-server": {
      "type": "http",
      "url": "https://example.com/mcp"
    }
  }
}
```

Untuk alasan keamanan, Claude Code meminta persetujuan dalam sesi interaktif sebelum menggunakan server dengan cakupan proyek dari file `.mcp.json`. Untuk mengatur ulang pilihan persetujuan tersebut, jalankan `claude mcp reset-project-choices`.

Dalam menjalankan `claude -p`, sesi [Agent SDK](/docs/id/headless), dan [sesi cloud](/docs/id/claude-code-on-the-web), Claude Code tidak dapat menampilkan prompt tersebut: Claude Code memuat server dengan cakupan proyek tanpa bertanya. Claude Code juga melewati prompt dalam sesi yang Anda mulai dalam mode `bypassPermissions` dengan [`skipDangerousModePermissionPrompt`](/docs/id/settings-reference#skipdangerousmodepermissionprompt) diatur dalam pengaturan pengguna Anda atau dalam pengaturan terkelola. Untuk tetap menjaga server agar tidak dimuat:

* Tambahkan ke [`disabledMcpjsonServers`](/docs/id/settings-reference#disabledmcpjsonservers), yang memblokir server di setiap mode izin.
* Kecualikan pengaturan proyek sepenuhnya dengan [`--setting-sources`](/docs/id/cli-reference#cli-flags) atau opsi `settingSources` SDK.
* Mulai sesi dengan [`--strict-mcp-config`](/docs/id/cli-reference#cli-flags). Claude Code kemudian hanya menggunakan server MCP yang Anda teruskan dengan `--mcp-config`. Melewati prompt persetujuan untuk server dengan cakupan proyek yang Claude Code tidak memuat memerlukan Claude Code v2.1.246 atau lebih baru; sebelum v2.1.246, sesi ketat masih menunggu persetujuan untuk mereka, yang membuat sesi latar belakang menunggu saat startup. Lihat [Kontrol eksklusif dengan managed-mcp.json](/docs/id/managed-mcp#exclusive-control-with-managed-mcp-json) untuk apa yang dilakukan flag di bawah file MCP terkelola.

[Persetujuan server proyek dan kepercayaan workspace](#project-server-approvals-and-workspace-trust) mencakup bagaimana persetujuan yang dikomitkan ke repositori berinteraksi dengan kepercayaan workspace.

<h3 id="user-scope">
  Cakupan pengguna
</h3>

Server dengan cakupan pengguna disimpan dalam `~/.claude.json` dan menyediakan aksesibilitas lintas proyek, menjadikannya tersedia di semua proyek di mesin Anda sambil tetap pribadi untuk akun pengguna Anda. Cakupan ini bekerja dengan baik untuk server utilitas pribadi, alat pengembangan, atau layanan yang sering Anda gunakan di berbagai proyek.

```bash theme={null}
# Tambahkan server pengguna
claude mcp add --transport http hubspot --scope user https://mcp.hubspot.com/anthropic
```

<h3 id="scope-hierarchy-and-precedence">
  Hierarki cakupan dan prioritas
</h3>

Ketika server yang sama ditentukan di lebih dari satu tempat, Claude Code terhubung ke server tersebut sekali, menggunakan definisi dari sumber dengan prioritas tertinggi. Seluruh entri server dari sumber tersebut digunakan; bidang tidak digabungkan di seluruh cakupan.

1. Cakupan lokal
2. Cakupan proyek
3. Cakupan pengguna
4. [Server yang disediakan plugin](/docs/id/plugins/components#mcp-servers)
5. [Konektor claude.ai](#use-mcp-servers-from-claude-ai)

Tiga cakupan mencocokkan duplikat berdasarkan nama. Plugin dan konektor mencocokkan berdasarkan endpoint, jadi yang menunjuk ke URL atau perintah yang sama dengan server di atas diperlakukan sebagai duplikat.

Server yang disediakan organisasi Anda melalui pengaturan terkelola [`managedMcpServers`](/docs/id/managed-mcp#provide-servers-through-managed-settings) menempati peringkat di atas semua ini, jadi ketika salah satu dari mereka menduplikasinya, Claude Code terhubung ke definisi organisasi. Memerlukan Claude Code v2.1.259 atau lebih baru.

Jika Anda membuka sesi lokal di [Tab Kode aplikasi Desktop](/docs/id/desktop#mcp-servers-from-the-claude-desktop-chat-app) dengan nama server stdio yang sama di tingkat atas `~/.claude.json` (cakupan pengguna) dan di `.mcp.json`, Tab Kode menggunakan definisi `~/.claude.json`.

<h3 id="environment-variable-expansion-in-mcp-json">
  Ekspansi variabel lingkungan dalam `.mcp.json`
</h3>

Claude Code mendukung ekspansi variabel lingkungan dalam file `.mcp.json`, memungkinkan tim untuk berbagi konfigurasi sambil mempertahankan fleksibilitas untuk jalur spesifik mesin dan nilai sensitif seperti kunci API.

<h4 id="supported-syntax">
  Sintaks yang didukung
</h4>

* `${VAR}`: berkembang menjadi nilai variabel lingkungan `VAR`
* `${VAR:-default}`: berkembang menjadi `VAR` jika diatur, jika tidak menggunakan `default`

<h4 id="expansion-locations">
  Lokasi ekspansi
</h4>

Variabel lingkungan dapat berkembang dalam:

* `command`: jalur executable server
* `args`: argumen baris perintah
* `env`: variabel lingkungan yang diteruskan ke server
* `url`: untuk jenis server HTTP
* `headers`: untuk autentikasi server HTTP

<h4 id="example-with-variable-expansion">
  Contoh dengan ekspansi variabel
</h4>

```json theme={null}
{
  "mcpServers": {
    "api-server": {
      "type": "http",
      "url": "${API_BASE_URL:-https://api.example.com}/mcp",
      "headers": {
        "Authorization": "Bearer ${API_KEY}"
      }
    }
  }
}
```

<h4 id="unset-variables-without-a-default">
  Variabel yang tidak diatur tanpa default
</h4>

Jika variabel lingkungan yang dirujuk tidak diatur dan tidak memiliki nilai default, konfigurasi masih dimuat: Claude Code melaporkan peringatan variabel yang hilang untuk server tersebut dalam output `claude mcp list` dan menggunakan teks `${VAR}` yang tidak diperluas sebagaimana adanya. Atur variabel atau tambahkan fallback `:-default` sehingga server dimulai dengan nilai yang Anda maksudkan. Dalam `url` dan `headers` server jarak jauh, beberapa variabel kredensial [dibaca sebagai kosong](#credential-variables-that-read-as-empty) sebagai gantinya, tanpa peringatan.

<h4 id="credential-variables-that-read-as-empty">
  Variabel kredensial yang dibaca sebagai kosong
</h4>

Dalam `url` dan `headers` server jarak jauh, Claude Code membaca variabel kredensial dari lingkungan Anda sebagai kosong daripada memperluasnya. Ini mencegah `.mcp.json` proyek atau plugin dari mengirim kredensial Claude Code atau penyedia cloud Anda ke server yang dinamainya. Jika Anda menulis `Bearer ${ANTHROPIC_AUTH_TOKEN}`, server menerima `Bearer ` tanpa kredensial dan menolak permintaan, biasanya dengan `401`. Claude Code melaporkan itu sebagai koneksi yang gagal.

Nama yang tercakup adalah:

* Kredensial Claude Code sendiri, seperti `ANTHROPIC_API_KEY` dan `ANTHROPIC_AUTH_TOKEN`
* Kredensial penyedia cloud Anda, seperti `AWS_BEARER_TOKEN_BEDROCK`
* Kredensial lain yang dibawa lingkungan Anda, seperti `HTTPS_PROXY` dan `NPM_TOKEN`

Nama yang tercakup dibaca sebagai kosong apakah pun Anda telah menetapkan variabel, dan fallback `:-default` padanya diabaikan. URL dasar penyedia seperti `ANTHROPIC_BASE_URL` masih berkembang, jadi `"url": "${ANTHROPIC_BASE_URL}/mcp"` berfungsi, kecuali nilai URL itu sendiri menyematkan kredensial seperti nama pengguna dan kata sandi.

Nama di luar set ini, seperti `API_KEY`, berkembang seperti yang ditulis. Untuk memberikan server salah satu kredensial yang tercakup, salinnya ke dalam variabel dengan nama Anda sendiri dan referensikan nama itu sebagai gantinya.

Ketika `url` atau `headers` server jarak jauh mereferensikan variabel yang tercakup yang telah Anda atur, Claude Code menamakannya dalam baris log debug. Untuk membaca baris tersebut, jalankan `claude --debug-file /tmp/claude-debug.log` dan cari file itu untuk `never expanded toward a remote server`.

<h4 id="how-references-appear-in-/mcp-and-cli-output">
  Bagaimana referensi muncul dalam output `/mcp` dan CLI
</h4>

Untuk server dalam [cakupan](#mcp-installation-scopes) lokal, proyek, atau pengguna, permukaan berikut menampilkan referensi `${VAR}` berdasarkan nama daripada sebagai nilai yang diselesaikannya:

* URL atau baris perintah dalam tampilan detail `/mcp` server
* Output `claude mcp list` dan `claude mcp get`

Tampilan detail `/mcp` menampilkan referensi dengan cara ini dalam Claude Code v2.1.268 atau lebih baru.

Untuk server yang disediakan organisasi Anda melalui pengaturan `managedMcpServers`, permukaan ini menampilkan [hanya host URL](/docs/id/managed-mcp#what-users-can-see-and-change).

Untuk memeriksa apa yang ditampilkan `claude mcp list`, `claude mcp get`, dan `/mcp` ketika koneksi gagal, lihat [Detail status server](#server-status-detail).

<h2 id="practical-examples">
  Contoh praktis
</h2>

<h3 id="example-connect-to-github-for-code-reviews">
  Contoh: Hubungkan ke GitHub untuk tinjauan kode
</h3>

Server MCP jarak jauh GitHub diautentikasi dengan token akses pribadi GitHub yang diteruskan sebagai header. Untuk mendapatkan satu, buka [pengaturan token GitHub Anda](https://github.com/settings/personal-access-tokens), hasilkan token baru yang bersifat fine-grained dengan akses ke repositori yang ingin Claude kerjakan, kemudian tambahkan server:

```bash theme={null}
claude mcp add --transport http github https://api.githubcopilot.com/mcp/ \
  --header "Authorization: Bearer YOUR_GITHUB_PAT"
```

Ganti `YOUR_GITHUB_PAT` dengan token akses pribadi Anda. Perintah `claude mcp add` menyimpan konfigurasi tanpa memvalidasi kredensial, jadi nilai placeholder diterima di sini tetapi server gagal terhubung nanti. Untuk memverifikasi koneksi, jalankan `/mcp` dan periksa bahwa server menunjukkan `connected`. Server dengan kredensial buruk menunjukkan `failed`, dan detail kegagalan mencakup status HTTP yang dikembalikan server, seperti 401.

Kemudian bekerja dengan GitHub:

```text wrap theme={null}
Review PR #456 and suggest improvements
```

```text wrap theme={null}
Create a new issue for the bug we just found
```

```text wrap theme={null}
Show me all open PRs assigned to me
```

<h3 id="example-query-your-postgresql-database">
  Contoh: Tanyakan database PostgreSQL Anda
</h3>

[DBHub](https://github.com/bytebase/dbhub), paket `@bytebase/dbhub`, adalah server MCP yang menghubungkan Claude ke database relasional melalui string koneksi yang Anda teruskan di `--dsn`. Gunakan pengguna database read-only dalam string koneksi sehingga kueri yang Claude jalankan tidak dapat memodifikasi data:

```bash theme={null}
claude mcp add --transport stdio db -- npx -y @bytebase/dbhub \
  --dsn "postgresql://readonly:pass@prod.db.com:5432/analytics"
```

Untuk mengonfirmasi server dimulai, jalankan `/mcp` dan periksa bahwa `db` menunjukkan `connected`.

Kemudian tanyakan database Anda secara alami:

```text wrap theme={null}
What's our total revenue this month?
```

```text wrap theme={null}
Show me the schema for the orders table
```

```text wrap theme={null}
Find customers who haven't made a purchase in 90 days
```

<h2 id="authenticate-with-remote-mcp-servers">
  Autentikasi dengan server MCP jarak jauh
</h2>

Banyak server MCP berbasis cloud memerlukan autentikasi. Claude Code mendukung OAuth 2.0 untuk koneksi yang aman.

Claude Code menandai server jarak jauh sebagai memerlukan autentikasi ketika server merespons dengan `401 Unauthorized` atau `403 Forbidden`. Apa yang Claude Code tampilkan tergantung pada server:

* Untuk server yang belum Anda masuki, salah satu kode status ini menandainya di `/mcp` sehingga Anda dapat menyelesaikan alur OAuth.
* Untuk [konektor claude.ai](#use-mcp-servers-from-claude-ai), `401` yang disebabkan oleh claude.ai menolak token sesi Anda tidak menandai konektor, karena mengotorisasi ulang konektor tidak dapat memperbaiki login Anda. Claude Code menampilkan [status token-sesi-ditolak](/docs/id/errors#claude-ai-rejected-the-session-token) sebagai gantinya.
* Untuk server yang header `Authorization` Anda konfigurasi, di `headers` atau melalui [`headersHelper`](#use-dynamic-headers-for-custom-authentication), `401` atau `403` saat menghubungkan tidak menandai server, karena kredensial untuk diperbaiki adalah yang Anda konfigurasi. Claude Code melaporkan koneksi sebagai gagal sebagai gantinya. Jika Anda menetapkan header tersebut dari referensi `${VAR}`, periksa apakah variabel itu adalah salah satu yang Claude Code [baca sebagai kosong](#credential-variables-that-read-as-empty).
* Untuk konektor [yang dikirimkan ke sesi cloud](#how-connectors-reach-claude-code), Claude Code tidak menjalankan alur sign-in, karena proxy sesi mengautentikasi ke konektor dengan otorisasi yang Anda berikan di claude.ai. Ketika konektor di sana memerlukan otorisasi lagi, hubungkan kembali di [claude.ai/customize/connectors](https://claude.ai/customize/connectors) daripada dari sesi.

Ketika permintaan ke server OAuth yang sudah Anda masuki mengembalikan `401 Unauthorized`, Claude Code menyegarkan token yang disimpan, terhubung kembali, dan mencoba ulang permintaan sekali. Itu menandai server di `/mcp` hanya jika percobaan ulang itu juga gagal. Sebelum v2.1.206, penyegaran token yang gagal karena alasan sementara, seperti kesalahan jaringan, menandai server OAuth sebagai memerlukan autentikasi untuk sisa sesi meskipun token penyegarannya masih valid.

Ketika server menolak token penyegaran yang disimpan, Claude Code segera menampilkan pemberitahuan yang menunjuk ke `/mcp`. Buka `/mcp` dan pilih **Re-authenticate** pada server untuk masuk lagi sebelum panggilan alat berikutnya gagal.

Server kustom yang mengembalikan header `WWW-Authenticate` yang menunjuk ke server otorisasinya mendapatkan penemuan otomatis yang sama seperti server jarak jauh lainnya.

Claude Code juga menampilkan pemberitahuan startup ketika satu atau lebih server yang dikonfigurasi memerlukan autentikasi, sehingga Anda tidak perlu membuka `/mcp` untuk menemukan server mana yang memerlukan sign-in. Pemberitahuan memerlukan Claude Code v2.1.193 atau lebih baru. Itu hanya menghitung server yang dapat Anda masuki dari Claude Code. Sebelum v2.1.218, itu juga menghitung [konektor claude.ai](#use-mcp-servers-from-claude-ai) yang tidak terhubung di claude.ai, yang hanya dapat Anda hubungkan dari pengaturan claude.ai.

Pemberitahuan mengumumkan setiap server sekali dan mengeluarkannya dari hitungan di peluncuran berikutnya sampai server itu telah terhubung dan memerlukan sign-in lagi. `/mcp` masih mencantumkan setiap server yang memerlukan sign-in.

Dalam mode non-interaktif tidak ada panel `/mcp`, jadi Claude Code tidak dapat menjalankan alur OAuth untuk Anda. Mulai dari v2.1.196, ketika server yang dikonfigurasi memerlukan autentikasi selama `claude -p` atau jalankan Agent SDK dengan [pencarian alat](#scale-with-mcp-tool-search) diaktifkan, yang merupakan default, Claude Code memberi tahu Claude bahwa alat server tidak tersedia sampai Anda mengotorisasinya. Claude kemudian dapat menyebutkan server yang memerlukan sign-in alih-alih merespons seolah-olah server tidak dikonfigurasi. Selesaikan sign-in dari sesi interaktif dengan `/mcp` atau `claude mcp login <name>`.

Jika Anda mengonfigurasi `headers.Authorization` untuk server dan server menolak header tersebut, Claude Code melaporkan koneksi sebagai gagal alih-alih kembali ke OAuth. Periksa bahwa token valid untuk endpoint MCP, atau hapus header untuk menggunakan alur OAuth.

<Steps>
  <Step title="Tambahkan server yang memerlukan autentikasi">
    Jika Anda sudah menambahkan server `sentry` di [panduan cepat MCP](/docs/id/mcp-quickstart#connect-a-server-that-requires-sign-in), lewati langkah ini: menjalankan `claude mcp add` lagi dengan nama server yang sama di cakupan yang sama gagal dengan `MCP server sentry already exists in local config`. Jika tidak, jalankan:

    ```bash theme={null}
    claude mcp add --transport http sentry https://mcp.sentry.dev/mcp
    ```
  </Step>

  <Step title="Gunakan perintah /mcp dalam Claude Code">
    Dalam Claude Code, gunakan perintah:

    ```text wrap theme={null}
    /mcp
    ```

    Kemudian ikuti langkah-langkah di browser Anda untuk login.
  </Step>
</Steps>

<Tip>
  Tips:

  * Token autentikasi disimpan dengan aman dan disegarkan secara otomatis
  * Gunakan "Clear authentication" dalam menu `/mcp` untuk mencabut akses
  * Jika browser Anda tidak terbuka secara otomatis, salin URL yang disediakan dan buka secara manual
  * Jika pengalihan browser gagal dengan kesalahan koneksi setelah autentikasi, tempel URL callback lengkap dari bilah alamat browser Anda ke prompt URL yang muncul di Claude Code
  * Autentikasi OAuth bekerja dengan server HTTP
</Tip>

<h3 id="authenticate-from-the-command-line">
  Autentikasi dari baris perintah
</h3>

Perintah `claude mcp login <name>` menjalankan alur OAuth server yang dikonfigurasi langsung dari shell Anda, sehingga Anda tidak perlu membuka panel `/mcp` di dalam sesi.

```bash theme={null}
claude mcp login sentry
```

Untuk menghapus kredensial yang disimpan nanti, jalankan `claude mcp logout <name>`.

`claude mcp login` mendeteksi ketika tidak ada browser lokal yang tersedia, seperti selama sesi SSH atau di Linux tanpa server tampilan, dan mencetak URL otorisasi alih-alih mencoba membuka browser. Buka URL di mesin lokal Anda, kemudian tempel URL pengalihan lengkap dari bilah alamat browser Anda kembali ke prompt. Perintah memerlukan terminal interaktif untuk langkah paste, jadi hubungkan dengan `ssh -t`. Teruskan `--no-browser` untuk memaksa prompt URL bahkan ketika browser lokal terdeteksi.

```bash theme={null}
claude mcp login sentry --no-browser
```

<h3 id="use-a-fixed-oauth-callback-port">
  Gunakan port callback OAuth tetap
</h3>

Beberapa server MCP memerlukan URI pengalihan tertentu yang terdaftar sebelumnya. Secara default, Claude Code memilih port acak yang tersedia untuk callback OAuth. Gunakan `--callback-port` untuk memperbaiki port sehingga cocok dengan URI pengalihan yang telah terdaftar sebelumnya dalam bentuk `http://localhost:PORT/callback`. Jika sign-in pada Claude Code v2.1.229 gagal dengan ketidakcocokan URI pengalihan, lihat catatan versi di bawah [Gunakan kredensial OAuth yang telah dikonfigurasi sebelumnya](#use-pre-configured-oauth-credentials).

Anda dapat menggunakan `--callback-port` sendiri (dengan pendaftaran klien dinamis) atau bersama dengan `--client-id` (dengan kredensial yang telah dikonfigurasi sebelumnya).

```bash theme={null}
# Port callback tetap dengan pendaftaran klien dinamis
claude mcp add --transport http \
  --callback-port 8080 \
  my-server https://mcp.example.com/mcp
```

<h3 id="use-pre-configured-oauth-credentials">
  Gunakan kredensial OAuth yang telah dikonfigurasi sebelumnya
</h3>

Beberapa server MCP tidak mendukung pengaturan OAuth otomatis melalui Dynamic Client Registration. Jika Anda melihat kesalahan seperti "Incompatible auth server: does not support dynamic client registration," server memerlukan kredensial yang telah dikonfigurasi sebelumnya. Claude Code juga mendukung server yang menggunakan Client ID Metadata Document (CIMD) alih-alih Dynamic Client Registration, dan menemukan ini secara otomatis. Jika penemuan otomatis gagal, daftarkan aplikasi OAuth melalui portal pengembang server terlebih dahulu, kemudian berikan kredensial saat menambahkan server.

<Steps>
  <Step title="Daftarkan aplikasi OAuth dengan server">
    Buat aplikasi melalui portal pengembang server dan catat ID klien dan rahasia klien Anda.

    Banyak server juga memerlukan URI pengalihan. Jika demikian, pilih port dan daftarkan URI pengalihan dalam format `http://localhost:PORT/callback`. Gunakan port yang sama dengan `--callback-port` di langkah berikutnya.

    Di v2.1.229, Claude Code mengirim `http://127.0.0.1:PORT/callback` sebagai gantinya, dan server yang cocok persis dengan URI pengalihan yang terdaftar menolak sign-in dengan ketidakcocokan URI pengalihan. Claude Code v2.1.231 mengembalikan bentuk `localhost`. Untuk pulih di v2.1.229, tingkatkan Claude Code, atau sementara tambahkan bentuk `http://127.0.0.1:PORT/callback` ke URI pengalihan yang terdaftar di server.
  </Step>

  <Step title="Tambahkan server dengan kredensial Anda">
    Pilih salah satu metode berikut. Port yang digunakan untuk `--callback-port` dapat berupa port apa pun yang tersedia. Itu hanya perlu cocok dengan URI pengalihan yang Anda daftarkan di langkah sebelumnya.

    <Tabs>
      <Tab title="claude mcp add">
        Gunakan `--client-id` untuk meneruskan ID klien aplikasi Anda. Flag `--client-secret` meminta rahasia dengan input yang disembunyikan:

        ```bash theme={null}
        claude mcp add --transport http \
          --client-id your-client-id --client-secret --callback-port 8080 \
          my-server https://mcp.example.com/mcp
        ```
      </Tab>

      <Tab title="claude mcp add-json">
        Sertakan objek `oauth` dalam konfigurasi JSON dan teruskan `--client-secret` sebagai flag terpisah:

        ```bash theme={null}
        claude mcp add-json my-server \
          '{"type":"http","url":"https://mcp.example.com/mcp","oauth":{"clientId":"your-client-id","callbackPort":8080}}' \
          --client-secret
        ```
      </Tab>

      <Tab title="claude mcp add-json (callback port only)">
        Gunakan `--callback-port` tanpa ID klien untuk memperbaiki port sambil menggunakan pendaftaran klien dinamis:

        ```bash theme={null}
        claude mcp add-json my-server \
          '{"type":"http","url":"https://mcp.example.com/mcp","oauth":{"callbackPort":8080}}'
        ```
      </Tab>

      <Tab title="CI / env var">
        Atur rahasia melalui variabel lingkungan untuk melewati prompt interaktif:

        ```bash theme={null}
        MCP_CLIENT_SECRET=your-secret claude mcp add --transport http \
          --client-id your-client-id --client-secret --callback-port 8080 \
          my-server https://mcp.example.com/mcp
        ```
      </Tab>
    </Tabs>
  </Step>

  <Step title="Autentikasi di Claude Code">
    Jalankan `/mcp` di Claude Code dan ikuti alur login browser.
  </Step>
</Steps>

<Tip>
  Tips:

  * Rahasia klien disimpan dengan aman di keychain sistem Anda (macOS) atau file kredensial, bukan di konfigurasi Anda
  * Anda dapat mengatur rahasia klien hanya ketika Anda menambahkan server. Ketika Anda melakukan autentikasi dengan `claude mcp login` atau dari `/mcp`, Claude Code menggunakan rahasia yang disimpan dan tidak meminta satu atau membaca `MCP_CLIENT_SECRET`
  * Untuk menambah atau mengubah rahasia nanti, hapus server dengan `claude mcp remove <name>`, kemudian tambahkan lagi dengan `--client-secret` dan `--scope` yang sama
  * Jika server menggunakan klien OAuth publik tanpa rahasia, gunakan hanya `--client-id` tanpa `--client-secret`
  * Flag ini hanya berlaku untuk transport HTTP dan SSE. Mereka tidak berpengaruh pada server stdio
  * Gunakan `claude mcp get <name>` untuk memverifikasi bahwa kredensial OAuth dikonfigurasi untuk server
</Tip>

<h3 id="override-oauth-metadata-discovery">
  Ganti penemuan metadata OAuth
</h3>

Arahkan Claude Code ke URL metadata otorisasi OAuth tertentu untuk melewati rantai penemuan default. Atur `authServerMetadataUrl` ketika endpoint standar server MCP mengembalikan kesalahan, atau ketika Anda ingin merutekan penemuan melalui proxy internal. Secara default, Claude Code pertama kali memeriksa Protected Resource Metadata RFC 9728 di `/.well-known/oauth-protected-resource`, kemudian kembali ke metadata server otorisasi RFC 8414 di `/.well-known/oauth-authorization-server`.

Atur `authServerMetadataUrl` dalam objek `oauth` dari konfigurasi server Anda di `.mcp.json`:

```json theme={null}
{
  "mcpServers": {
    "my-server": {
      "type": "http",
      "url": "https://mcp.example.com/mcp",
      "oauth": {
        "authServerMetadataUrl": "https://auth.example.com/.well-known/openid-configuration"
      }
    }
  }
}
```

URL harus menggunakan `https://`. `scopes_supported` dari URL metadata mengganti cakupan yang diiklankan server upstream.

<h3 id="restrict-oauth-scopes">
  Batasi cakupan OAuth
</h3>

Atur `oauth.scopes` untuk menyematkan cakupan yang diminta Claude Code selama alur otorisasi. Ini adalah cara yang didukung untuk membatasi server MCP ke subset yang disetujui tim keamanan ketika server otorisasi upstream mengiklankan lebih banyak cakupan daripada yang ingin Anda berikan. Nilainya adalah string tunggal yang dipisahkan spasi, cocok dengan format parameter `scope` dalam RFC 6749 §3.3.

```json theme={null}
{
  "mcpServers": {
    "slack": {
      "type": "http",
      "url": "https://mcp.slack.com/mcp",
      "oauth": {
        "scopes": "channels:read chat:write search:read"
      }
    }
  }
}
```

`oauth.scopes` mengambil prioritas atas `authServerMetadataUrl` dan cakupan yang ditemukan server di `/.well-known`. Biarkan tidak diatur untuk membiarkan server MCP menentukan set cakupan yang diminta.

Mulai dari v2.1.196, ketika `oauth.scopes` tidak diatur, Claude Code meminta cakupan yang disediakan oleh header `WWW-Authenticate` server atau metadata sumber daya terlindungnya, dan tidak mengirim parameter `scope` ketika tidak ada yang menyediakannya. Itu tidak lagi meminta katalog `scopes_supported` lengkap dari metadata server otorisasi yang ditemukan secara otomatis. Meminta katalog itu membuat penyedia identitas yang mengiklankan cakupan khusus admin atau template menolak permintaan otorisasi dengan kesalahan `invalid_scope`. Metadata yang diambil dari `authServerMetadataUrl` yang dikonfigurasi masih menyediakan `scopes_supported` sebagai cakupan yang diminta.

Jika server otorisasi mengiklankan `offline_access` dalam `scopes_supported`, Claude Code menambahkannya ke cakupan yang disematkan sehingga token akses dapat disegarkan tanpa login browser baru.

Jika server kemudian mengembalikan 403 `insufficient_scope` untuk panggilan alat, panggilan gagal dengan pesan [`needs additional permissions`](/docs/id/errors#mcp-server-needs-you-to-sign-in-again) yang menyebutkan cakupan yang diminta server. Server ditampilkan sebagai memerlukan autentikasi di `/mcp`.

Jika cakupan itu tidak ada dalam `oauth.scopes` yang disematkan Anda, tambahkan, kemudian jalankan `/mcp` dan autentikasi server lagi. Claude Code meminta cakupan yang disematkan daripada cakupan yang dinamai server, jadi jika Anda melakukan autentikasi lagi tanpa menambahkannya, token yang Anda dapatkan masih tidak memilikinya.

<h3 id="use-dynamic-headers-for-custom-authentication">
  Gunakan header dinamis untuk autentikasi khusus
</h3>

Jika server MCP Anda menggunakan skema autentikasi selain OAuth, seperti Kerberos, token berumur pendek, atau SSO internal, gunakan `headersHelper` untuk menghasilkan header permintaan pada waktu koneksi. Claude Code menjalankan perintah dan menggabungkan outputnya ke dalam header koneksi.

```json theme={null}
{
  "mcpServers": {
    "internal-api": {
      "type": "http",
      "url": "https://mcp.internal.example.com",
      "headersHelper": "/opt/bin/get-mcp-auth-headers.sh"
    }
  }
}
```

Perintah juga dapat inline:

```json theme={null}
{
  "mcpServers": {
    "internal-api": {
      "type": "http",
      "url": "https://mcp.internal.example.com",
      "headersHelper": "echo '{\"Authorization\": \"Bearer '\"$(get-token)\"'\"}'"
    }
  }
}
```

**Persyaratan:**

* Perintah harus menulis objek JSON dari pasangan kunci-nilai string ke stdout
* Claude Code menjalankan perintah dalam shell dan menyerah padanya setelah 10 detik
* Claude Code memilih direktori kerja perintah dengan [tempat Anda mengonfigurasi server](#where-the-helper-runs), jadi berikan skrip sebagai jalur absolut atau letakkan di `PATH`
* Header dinamis mengganti `headers` statis apa pun dengan nama yang sama

Claude Code menjalankan helper segar pada setiap koneksi, pada startup sesi dan pada reconnect, setelah [aturan kepercayaan untuk server cakupan proyek dan lokal](#trust-a-folder-before-its-headershelper-runs) membiarkannya berjalan. Itu tidak menyimpan hasil, jadi skrip Anda bertanggung jawab untuk penggunaan kembali token apa pun.

Jika panggilan alat mengembalikan `401 Unauthorized` atau `403 Forbidden`, Claude Code secara otomatis menjalankan kembali helper di bawah aturan yang sama, terhubung kembali dengan header segar, dan mencoba panggilan sekali lagi. Claude Code menandai server sebagai memerlukan autentikasi di `/mcp` hanya jika percobaan ulang itu juga gagal.

Ketika output helper menyertakan header `Authorization`, Claude Code menggunakan kredensial itu sebagai autentikasi server dan tidak kembali ke OAuth untuk server.

Jika server menolak kredensial helper saat menghubungkan, Claude Code melaporkan koneksi sebagai gagal daripada menandai server sebagai memerlukan autentikasi. Perbaiki kredensial yang dikembalikan helper Anda, kemudian hubungkan kembali dari `/mcp` untuk menjalankan kembali helper.

Claude Code menetapkan variabel lingkungan ini saat menjalankan helper:

| Variabel                      | Nilai                                                                                                      |
| :---------------------------- | :--------------------------------------------------------------------------------------------------------- |
| `CLAUDE_CODE_MCP_SERVER_NAME` | nama server MCP                                                                                            |
| `CLAUDE_CODE_MCP_SERVER_URL`  | URL server MCP                                                                                             |
| `CLAUDE_PLUGIN_ROOT`          | direktori root plugin. Diatur hanya ketika [plugin](/docs/id/plugins/components#mcp-servers) menyediakan server |

Gunakan ini untuk menulis satu skrip helper yang melayani beberapa server MCP.

Plugin-provided `headersHelper` tidak dapat mereferensikan nilai [`${user_config.*}`](/docs/id/plugins/manifest-reference#user-configuration) plugin, karena perintah berjalan melalui shell. Claude Code melaporkan server sebagai salah konfigurasi dengan [kesalahan](/docs/id/errors#plugin-command-references-user-config) dan tidak mengganti nilai. Letakkan `${user_config.KEY}` di bidang `headers` server sebagai gantinya, yang tidak diurai shell, atau buat skrip helper membaca nilai dari file konfigurasi. Sebelum v2.1.207, `headersHelper` mengganti nilai `${user_config.*}`.

<h4 id="where-the-helper-runs">
  Tempat helper berjalan
</h4>

Claude Code memilih direktori kerja perintah `headersHelper` dari konfigurasi yang mendeklarasikan server. `cd` yang Claude jalankan di Bash tidak memindahkannya, dan [`/cd`](/docs/id/permissions#move-the-session-to-another-directory) memindahkannya hanya untuk server yang berjalan dari direktori kerja utama sesi. Setiap baris di bawah memberikan direktori yang jalur relatif dalam perintah `headersHelper` Anda diselesaikan.

| Tempat Anda mengonfigurasi server                                                                                                                                                                   | Direktori kerja                                                                                     |
| :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------- |
| [Plugin](/docs/id/plugins/components#mcp-servers)                                                                                                                                                        | Direktori root plugin. Memerlukan Claude Code v2.1.195 atau lebih baru                              |
| File `.mcp.json` proyek atau server [cakupan lokal](#local-scope)                                                                                                                                   | Direktori proyek tempat server dideklarasikan                                                       |
| File agen di proyek Anda, server dari opsi `mcpServers` SDK atau metode `setMcpServers()`, atau [`--mcp-config`](/docs/id/cli-reference)                                                                 | [Direktori kerja utama](/docs/id/permissions#working-directories) sesi                                   |
| [Cakupan pengguna](#user-scope), [MCP terkelola](/docs/id/managed-mcp), [konektor claude.ai](#use-mcp-servers-from-claude-ai), atau file agen dari luar proyek Anda, termasuk dari direktori `--add-dir` | Direktori konfigurasi Anda, `~/.claude` kecuali Anda menetapkan [`CLAUDE_CONFIG_DIR`](/docs/id/env-vars) |

Sebelum v2.1.238, Claude Code juga menjalankan helper dari server cakupan pengguna, terkelola, dan konektor claude.ai, dan file agen dari luar proyek Anda, dari direktori tempat Anda memulainya.

<h4 id="which-variables-a-helper-can-read">
  Variabel mana yang dapat dibaca helper
</h4>

`headersHelper` yang disediakan repositori atau plugin adalah perintah yang tidak Anda tulis, jadi Claude Code menjalankannya tanpa variabel kredensial dari lingkungan Anda, seperti `ANTHROPIC_API_KEY`. Tempat Anda mengonfigurasi server menentukan apakah ini berlaku:

* **Dihapus**: server di `.mcp.json` proyek atau di plugin, dan server inline di file agen dari proyek Anda atau dari direktori `--add-dir`
* **Tidak dihapus**: server di [cakupan pengguna](#user-scope) atau [cakupan lokal](#local-scope), di [MCP terkelola](/docs/id/managed-mcp), dari [konektor claude.ai](#use-mcp-servers-from-claude-ai), atau disediakan oleh SDK atau [`--mcp-config`](/docs/id/cli-reference), dan server inline di file agen dari `~/.claude/agents/`, dari pengaturan terkelola, atau diteruskan dengan `--agents`

Terlepas dari variabel `GIT_CONFIG_KEY_<n>` Git, Claude Code menghapus setiap variabel dari lingkungan Anda yang namanya terlihat seperti kredensial, seperti nama dengan `TOKEN`, `SECRET`, `PASSWORD`, `KEY`, atau `AUTH` di dalamnya dalam huruf besar atau kecil, jadi `ANTHROPIC_API_KEY` dan `MY_REGISTRY_TOKEN` keduanya dihapus. Claude Code juga menghapus daftar tetap variabel kredensial yang namanya tidak mengikuti pola itu, seperti `ANTHROPIC_CUSTOM_HEADERS`.

Ketika ini berlaku untuk helper Anda, buat skrip membaca kredensialnya dari file atau penyimpanan kredensial. Jika `url` server [memperluas salah satu variabel ini](#environment-variable-expansion-in-mcp-json), nilai `CLAUDE_CODE_MCP_SERVER_URL` yang diterima helper memiliki bagian itu diganti dengan `REDACTED` juga.

<h4 id="trust-a-folder-before-its-headershelper-runs">
  Percayai folder sebelum headersHelper-nya berjalan
</h4>

Claude Code menjalankan `headersHelper` sebagai perintah shell arbitrer. Untuk server di `.mcp.json` proyek atau di [cakupan lokal](#local-scope), itu menjalankan helper hanya setelah Anda menerima [dialog kepercayaan](/docs/id/permissions#project-allow-rules-and-workspace-trust) untuk direktori proyek tempat server dideklarasikan. Sebelum v2.1.238, sesi `claude -p` atau SDK menjalankan helper ini tanpa memeriksa kepercayaan, dan sesi interaktif menjalankannya setelah Anda mempercayai folder induk.

* **Kepercayaan yang tidak dihitung**: kepercayaan folder induk, dan kepercayaan otomatis yang diterima sesi `claude -p` atau SDK untuk [hooks di file pengaturan](/docs/id/permissions#what-runs-before-you-trust-a-folder)
* **Sampai Anda mempercayai folder**: Claude Code menghubungkan server dengan `headers` statis saja. Dalam sesi `claude -p` atau SDK itu juga mencetak satu baris [`headersHelper not run`](/docs/id/errors#headershelper-not-run) per server ke stderr, memberi tahu Anda cara memberikan kepercayaan.
* **Kepercayaan tanpa dialog**: atur `projects["<path>"].hasTrustDialogAccepted` ke `true` di `~/.claude.json`. `<path>` adalah folder yang [Project allow rules and workspace trust](/docs/id/permissions#project-allow-rules-and-workspace-trust) katakan Claude Code kunci kepercayaan padanya.

Claude Code menerapkan aturan yang sama ke server yang dideklarasikan inline di [file agen](/docs/id/sub-agents#scope-mcp-servers-to-a-subagent), memeriksa tempat file agen itu berasal: proyek Anda, untuk file di direktori `.claude/agents/` -nya, atau direktori `--add-dir`. Sampai Anda [mempercayai proyek atau direktori itu sendiri](/docs/id/permissions#what-runs-before-you-trust-a-folder), Claude Code tidak memuat server sama sekali, jadi helper-nya tidak pernah berjalan juga.

<h2 id="add-mcp-servers-from-json-configuration">
  Tambahkan server MCP dari konfigurasi JSON
</h2>

Jika Anda memiliki konfigurasi JSON untuk server MCP, Anda dapat menambahkannya secara langsung:

<Steps>
  <Step title="Tambahkan server MCP dari JSON">
    ```bash theme={null}
    # Sintaks dasar
    claude mcp add-json <name> '<json>'

    # Contoh: Menambahkan server HTTP dengan konfigurasi JSON
    claude mcp add-json weather-api '{"type":"http","url":"https://api.weather.com/mcp","headers":{"Authorization":"Bearer token"}}'

    # Contoh: Menambahkan server stdio dengan konfigurasi JSON
    claude mcp add-json local-weather '{"type":"stdio","command":"/path/to/weather-cli","args":["--api-key","abc123"],"env":{"CACHE_DIR":"/tmp"}}'

    # Contoh: Menambahkan server HTTP dengan kredensial OAuth yang telah dikonfigurasi sebelumnya
    claude mcp add-json my-server '{"type":"http","url":"https://mcp.example.com/mcp","oauth":{"clientId":"your-client-id","callbackPort":8080}}' --client-secret
    ```
  </Step>

  <Step title="Verifikasi server ditambahkan">
    ```bash theme={null}
    claude mcp get weather-api
    ```
  </Step>
</Steps>

<Tip>
  Tips:

  * Pastikan JSON diloloskan dengan benar di shell Anda
  * JSON harus sesuai dengan skema konfigurasi server MCP
  * Anda dapat menggunakan `--scope user` untuk menambahkan server ke konfigurasi pengguna Anda alih-alih yang spesifik proyek
</Tip>

<h2 id="import-mcp-servers-from-claude-desktop">
  Impor server MCP dari Claude Desktop
</h2>

Jika Anda telah mengonfigurasi server MCP di Claude Desktop, Anda dapat mengimpornya:

<Steps>
  <Step title="Impor server dari Claude Desktop">
    ```bash theme={null}
    # Sintaks dasar 
    claude mcp add-from-claude-desktop 
    ```
  </Step>

  <Step title="Pilih server mana yang akan diimpor">
    Setelah menjalankan perintah, Anda akan melihat dialog interaktif yang memungkinkan Anda memilih server mana yang ingin Anda impor.
  </Step>

  <Step title="Verifikasi server diimpor">
    ```bash theme={null}
    claude mcp list 
    ```
  </Step>
</Steps>

Nama server yang ditambahkan melalui perintah `claude mcp` hanya dapat berisi huruf, angka, tanda hubung, dan garis bawah. Claude Desktop tidak menerapkan pembatasan itu, jadi server Claude Desktop yang namanya berisi karakter lain, seperti spasi, tidak dapat diimpor. Impor melaporkan setiap nama yang ditolak dan tetap mengimpor server lain yang Anda pilih. Sebelum v2.1.205, nama tidak valid pertama menghentikan impor dan tidak ada server yang dipilih yang ditambahkan.

<Tip>
  Tips:

  * Fitur ini hanya bekerja di macOS dan Windows Subsystem for Linux (WSL)
  * Ini membaca file konfigurasi Claude Desktop dari lokasi standarnya di platform tersebut
  * Gunakan flag `--scope user` untuk menambahkan server ke konfigurasi pengguna Anda
  * Server yang diimpor mempertahankan nama yang sama seperti di Claude Desktop ketika nama hanya berisi huruf, angka, tanda hubung, dan garis bawah. Claude Code melaporkan server yang namanya berisi karakter lain dan melewatkannya
  * Jika server dengan nama yang sama sudah ada, mereka akan mendapatkan akhiran numerik (misalnya, `server_1`)
</Tip>

<h2 id="use-mcp-servers-from-claude-ai">
  Gunakan server MCP dari claude.ai
</h2>

Jika Anda telah masuk ke Claude Code dengan akun [claude.ai](https://claude.ai), server MCP yang telah Anda tambahkan di claude.ai, yang dikenal sebagai [connectors](https://claude.com/docs/connectors), secara otomatis tersedia di Claude Code:

<Steps>
  <Step title="Konfigurasi server MCP di claude.ai">
    Tambahkan server di [claude.ai/customize/connectors](https://claude.ai/customize/connectors). Pada paket Team dan Enterprise, hanya admin yang dapat menambahkan server.
  </Step>

  <Step title="Autentikasi server MCP">
    Selesaikan langkah-langkah autentikasi yang diperlukan di claude.ai.
  </Step>

  <Step title="Lihat dan kelola server di Claude Code">
    Di Claude Code, gunakan perintah:

    ```text wrap theme={null}
    /mcp
    ```

    Server dari claude.ai muncul dalam daftar dengan indikator yang menunjukkan bahwa mereka berasal dari claude.ai.
  </Step>
</Steps>

Claude Code menandai connector sebagai `managed` di `/mcp` dan di manajer [`/plugin`](/docs/id/plugins/install) ketika organisasi Anda mengelola autentikasinya di claude.ai. Status managed tidak mengubah cara Claude Code terhubung ke connector atau menerapkan [kontrol alat](#organization-controls-on-connector-tools) organisasi Anda.

Connector yang belum pernah Anda masuki dilipat di belakang baris `Show unused connectors` di akhir bagian claude.ai, sehingga daftar yang disediakan organisasi tidak mengisi panel. Pilih baris untuk memperluasnya. Connector yang Anda masuki sebelumnya tetap terlihat bahkan ketika saat ini memerlukan re-autentikasi.

Connector dari claude.ai diambil hanya ketika [metode autentikasi](/docs/id/authentication#authentication-precedence) aktif Anda adalah login langganan claude.ai. Mereka tidak dimuat, bahkan jika Anda sebelumnya menjalankan `/login`, ketika:

* `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN`, atau `apiKeyHelper` aktif
* Penyedia pihak ketiga seperti Amazon Bedrock atau Agent Platform Google Cloud aktif
* `ANTHROPIC_PROFILE`, variabel federasi, atau [profil Anthropic](/docs/id/authentication#anthropic-profiles-and-federation-credentials) aktif menyediakan kredensial
* `CLAUDE_CODE_OAUTH_TOKEN` menyimpan token dari [`claude setup-token`](/docs/id/authentication#generate-a-long-lived-token), yang hanya dapat membuat permintaan model

Jika `/mcp` tidak mencantumkan connector yang Anda tambahkan, jalankan `/status` untuk mengonfirmasi metode autentikasi mana yang aktif. Batalkan pengaturan variabel lingkungan itu, hapus pengaturan `apiKeyHelper`, atau [matikan profil](/docs/id/authentication#anthropic-profiles-and-federation-credentials), kemudian jalankan `/login` untuk memilih akun claude.ai Anda.

Jika masalah jaringan sementara mencegah daftar connector dimuat saat sesi Anda dimulai, Claude Code mencoba pengambilan hingga tiga kali di latar belakang, dan connector muncul setelah percobaan berhasil. Jika mereka masih belum muncul, mulai ulang Claude Code untuk mengambil daftar lagi.

Jika `/mcp` menunjukkan connector sebagai `connected · session token rejected`, atau tampilan detailnya menunjukkan [`claude.ai rejected the session token`](/docs/id/errors#claude-ai-rejected-the-session-token), claude.ai menolak token dari login Claude Code Anda, biasanya karena login kedaluwarsa dan tidak dapat disegarkan. Mengotorisasi connector lagi tidak menghapus status ini, karena otorisasi connector itu sendiri di claude.ai bukan yang ditolak. Untuk menghapusnya:

1. Jalankan `/login` untuk masuk lagi.
2. Hubungkan kembali connector dari `/mcp`.

Sebelum v2.1.222, Claude Code menandai connector sebagai memerlukan autentikasi, dan mengotorisasi mereka tidak menyelesaikannya.

Server yang telah Anda tambahkan di Claude Code mengambil [precedence](#scope-hierarchy-and-precedence) atas connector claude.ai yang menunjuk ke URL yang sama. Ketika ini terjadi, `/mcp` mencantumkan connector sebagai tersembunyi dan menunjukkan cara menghapus duplikat jika Anda lebih suka menggunakan connector.

Beberapa connector yang dihosting Anthropic, seperti Microsoft 365, Gmail, dan Google Calendar, tidak mendukung OAuth lokal dari Claude Code karena penyedia identitas upstream hanya menerima URL pengalihan yang didaftarkan claude.ai. Ketika server yang Anda tambahkan dengan `claude mcp add` atau di `.mcp.json` menunjuk ke salah satu host ini dan Anda masuk ke dalamnya dari `/mcp` atau dengan `claude mcp login`, Claude Code menunjukkan [`is Anthropic-hosted and doesn't support local OAuth`](/docs/id/errors#anthropic-hosted-and-doesnt-support-local-oauth), mengarahkan Anda untuk menghubungkan layanan di [claude.ai/customize/connectors](https://claude.ai/customize/connectors) sebagai gantinya.

Setelah Anda menghapus entri Anda dengan `claude mcp remove <name>` dan menghubungkan layanan di claude.ai, connector muncul di Claude Code secara otomatis.

<h3 id="how-connectors-reach-claude-code">
  Bagaimana connector mencapai Claude Code
</h3>

Pengaturan mana yang mengatur connector claude.ai tergantung pada tempat sesi Anda berjalan, karena hanya beberapa sesi yang mengambil connector dari claude.ai sendiri. Setiap baris di bawah menunjukkan bagaimana connector tiba dalam satu jenis sesi dan apa yang mengontrolnya di sana. [Sesi WSL](/docs/id/desktop-wsl#what-works-in-a-wsl-session) aplikasi desktop tidak memiliki baris karena connector belum tersedia di dalamnya.

| Tempat sesi berjalan                                                                                                   | Bagaimana connector tiba                     | Apa yang mengontrol mereka                                                                                                                                                                                                           |
| :--------------------------------------------------------------------------------------------------------------------- | :------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Terminal, [VS Code](/docs/id/vs-code), [JetBrains](/docs/id/jetbrains), dan sesi [Agent SDK](/docs/id/agent-sdk/claude-code-features) | Claude Code mengambilnya dari claude.ai      | Pengaturan di bagian ini dan [konfigurasi MCP yang dikelola](/docs/id/managed-mcp)                                                                                                                                                        |
| [Sesi cloud](/docs/id/claude-code-on-the-web)                                                                               | Host jarak jauh meneruskannya                | Pengaturan organisasi claude.ai Anda, ditambah pengaturan [allowlist dan denylist](/docs/id/managed-mcp#policy-based-control-with-allowlists-and-denylists) yang mencapai sesi dan `managed-mcp.json` apa pun di host yang menjalankannya |
| [Aplikasi desktop](/docs/id/desktop) sesi lokal dan SSH                                                                     | Aplikasi desktop mengirimkannya dalam proses | Entri `blocked` dalam [kontrol alat connector](#organization-controls-on-connector-tools) organisasi Anda                                                                                                                            |

[`disableClaudeAiConnectors`](#disable-claude-ai-connectors), `ENABLE_CLAUDEAI_MCP_SERVERS`, dan [`allowAllClaudeAiMcps`](/docs/id/settings-reference#allowallclaudeaimcps) hanya bertindak pada baris pertama, connector yang Claude Code ambil sendiri. Dua baris lainnya berbeda darinya dengan cara-cara ini:

* **Sesi cloud**: Entri `allowedMcpServers` dan `deniedMcpServers` yang mencapai sesi, misalnya melalui [pengaturan yang dikelola server](/docs/id/server-managed-settings), juga memfilter connector yang dikirimkan. Proxy sesi menulis ulang URL setiap connector, jadi pola `serverUrl` yang ditulis untuk URL connector itu sendiri tidak cocok. Untuk mengakui connector yang dikirimkan bersama allowlist URL di lingkungan yang dihosting sendiri, tambahkan entri `serverUrl` yang tercantum di bawah [Lalu lintas Connector meninggalkan jaringan Anda](/docs/id/self-hosted-environments-deploy#connector-traffic-leaves-your-network). Claude Code menghapus connector yang dikirimkan ketika `managed-mcp.json` ada di host yang menjalankan sesi, seperti [host runner yang dihosting sendiri](/docs/id/self-hosted-environments-configuration#mcp-servers), terlepas dari apakah Anda menetapkan `allowAllClaudeAiMcps`.
* **Sesi lokal dan SSH aplikasi desktop**: aplikasi desktop mendaftarkan connector sebagai server `type: "sdk"` dalam proses, dan tidak ada pengaturan MCP atau `managed-mcp.json` yang mencapainya. Pengguna menjaga connector keluar dari sesi mereka sendiri dengan memutusnya di [claude.ai/customize/connectors](https://claude.ai/customize/connectors). Organisasi memblokir [alat](#organization-controls-on-connector-tools) connector atau mematikan [Claude Code di aplikasi desktop](/docs/id/desktop#admin-console-controls) sepenuhnya.

<h3 id="organization-controls-on-connector-tools">
  Kontrol organisasi pada alat connector
</h3>

Organisasi Anda dapat menetapkan kontrol per-alat pada [connector claude.ai](https://claude.com/docs/connectors). Claude Code membaca pengaturan ini saat startup dan memberlakukannya secara lokal, kecuali di [sesi lokal dan SSH](#how-connectors-reach-claude-code) aplikasi desktop. Di sana, aplikasi desktop menahan alat `blocked` sebelum mengirimkan connector, dan pengaturan `ask` tidak mencapai Claude Code, jadi ia menerapkan [aturan izin](/docs/id/permissions) biasa sesi ke alat tersebut alih-alih meminta pada setiap panggilan. Di sesi tempat Claude Code mengambil connector sendiri, jalankan `/mcp` untuk melihat pengaturan mana yang berlaku untuk setiap alat pada connector.

* **Alat diatur ke `ask`**: Claude Code meminta pada setiap panggilan dengan alasan `Your organization requires approval for this tool`. Prompt muncul bahkan dalam [mode izin](/docs/id/permissions#permission-modes) `acceptEdits`, `auto`, dan `bypassPermissions`, dan tidak pernah menawarkan opsi untuk mengingat pilihan Anda. [Aturan Allow](/docs/id/permissions) yang cocok dengan alat tidak melewati prompt juga. Dalam mode `dontAsk`, yang tidak pernah meminta, Claude Code menolak panggilan sebagai gantinya.
* **Alat diatur ke `blocked`**: Claude Code memfilter alat keluar sebelum Claude melihatnya, jadi tidak pernah muncul dalam daftar alat. Aplikasi desktop dan chat claude.ai menerapkan pengaturan `blocked` yang sama, jadi Claude tidak dapat menggunakan alat di sana juga, dan Anda tidak dapat menahan alat dari sesi aplikasi desktop sambil membuatnya tersedia di chat. Aplikasi desktop melewati connector yang semua alatnya diblokir.

<h3 id="disable-claude-ai-connectors">
  Nonaktifkan connector claude.ai
</h3>

Claude Code menerapkan [`disableClaudeAiConnectors`](/docs/id/settings-reference#disableclaudeaiconnectors) hanya ke connector yang [diambilnya sendiri](#how-connectors-reach-claude-code), bukan ke connector yang dikirimkan host cloud atau aplikasi desktop. Untuk mematikan connector yang diambilnya, atur pengaturan ke `true` dalam cakupan pengaturan apa pun:

```json theme={null}
{
  "disableClaudeAiConnectors": true
}
```

Pengaturan ini menggunakan semantik any-source-true: `true` dalam sumber pengaturan apa pun mengambil alih. File `.claude/settings.json` proyek yang diperiksa dapat memilih repositori keluar dari connector yang Claude Code ambil sendiri, tetapi `false` tingkat proyek tidak dapat mengaktifkan kembali connector yang `true` tingkat pengguna atau kebijakan telah nonaktifkan. Server yang diteruskan secara eksplisit melalui `--mcp-config` tidak terpengaruh.

Anda juga dapat menetapkan variabel lingkungan `ENABLE_CLAUDEAI_MCP_SERVERS` ke `false`, yang memiliki efek yang sama untuk sesi shell saat ini:

```bash theme={null}
ENABLE_CLAUDEAI_MCP_SERVERS=false claude
```

Untuk memblokir connector claude.ai individual alih-alih semuanya, tambahkan mereka ke [`deniedMcpServers`](/docs/id/managed-mcp) berdasarkan nama atau pola URL. Misalnya, entri `serverName` dari `"claude.ai Slack"` memblokir connector Slack. Anda juga dapat menjalankan `/mcp` untuk mengalihkan connector apa pun yang Claude Code ambil atau matikan hanya untuk proyek saat ini.

<h2 id="use-claude-code-as-an-mcp-server">
  Gunakan Claude Code sebagai server MCP
</h2>

Anda dapat menggunakan Claude Code itu sendiri sebagai server MCP yang dapat dihubungkan oleh aplikasi lain:

```bash theme={null}
# Mulai Claude sebagai server MCP stdio
claude mcp serve
```

Perintah tidak mencetak apa pun saat dimulai. Server MCP stdio berkomunikasi melalui stdin dan stdout, jadi terminal yang diam dan terblokir berarti server sedang berjalan dan menunggu klien untuk terhubung.

Anda dapat menggunakan ini di Claude Desktop dengan menambahkan konfigurasi ini ke claude\_desktop\_config.json:

```json theme={null}
{
  "mcpServers": {
    "claude-code": {
      "type": "stdio",
      "command": "claude",
      "args": ["mcp", "serve"],
      "env": {}
    }
  }
}
```

<Warning>
  **Mengonfigurasi jalur executable**: bidang `command` harus mereferensikan executable Claude Code. Jika perintah `claude` tidak ada di PATH sistem Anda, Anda perlu menentukan jalur lengkap ke executable.

  Untuk menemukan jalur lengkap:

  ```bash theme={null}
  which claude
  ```

  Kemudian gunakan jalur lengkap dalam konfigurasi Anda:

  ```json theme={null}
  {
    "mcpServers": {
      "claude-code": {
        "type": "stdio",
        "command": "/full/path/to/claude",
        "args": ["mcp", "serve"],
        "env": {}
      }
    }
  }
  ```

  Tanpa jalur executable yang benar, Anda akan mengalami kesalahan seperti `spawn claude ENOENT`.
</Warning>

<Tip>
  Tips:

  * Di Claude Desktop, coba minta Claude untuk membaca file di direktori, membuat edit, dan lainnya.
  * Server MCP ini hanya mengekspos tools Claude Code ke klien MCP Anda, jadi klien Anda sendiri bertanggung jawab untuk mengimplementasikan konfirmasi pengguna untuk panggilan tool individual.
</Tip>

<h2 id="mcp-output-limits-and-warnings">
  Batas dan peringatan output MCP
</h2>

Ketika alat MCP menghasilkan output besar, Claude Code membantu mengelola penggunaan token untuk mencegah membanjiri konteks percakapan Anda:

* **Ambang batas peringatan output**: Claude Code menampilkan peringatan ketika output alat MCP apa pun melebihi 10.000 token
* **Batas yang dapat dikonfigurasi**: Anda dapat menyesuaikan token output MCP maksimum yang diizinkan menggunakan variabel lingkungan `MAX_MCP_OUTPUT_TOKENS`
* **Batas default**: maksimum default adalah 25.000 token
* **Cakupan**: variabel lingkungan berlaku untuk alat yang tidak mendeklarasikan batas mereka sendiri. Alat yang menetapkan [`anthropic/maxResultSizeChars`](#raise-the-limit-for-a-specific-tool) menggunakan nilai tersebut sebagai gantinya untuk konten teks, terlepas dari apa yang ditetapkan `MAX_MCP_OUTPUT_TOKENS`. Alat yang mengembalikan data gambar masih tunduk pada `MAX_MCP_OUTPUT_TOKENS`
* **Melampaui batas**: ketika hasil tanpa konten gambar melebihi batas, Claude Code menyimpannya ke file dan menggantinya dalam percakapan dengan pesan yang menyebutkan jalur file, sehingga Claude membaca file ketika membutuhkan konten. File tersebut berada di direktori `tool-results` sesi di bawah [`~/.claude/projects/`](/docs/id/claude-directory#cleaned-up-automatically).

Untuk meningkatkan batas untuk alat yang menghasilkan output besar:

```bash theme={null}
export MAX_MCP_OUTPUT_TOKENS=50000
claude
```

<h3 id="raise-the-limit-for-a-specific-tool">
  Tingkatkan batas untuk alat tertentu
</h3>

Jika Anda membangun server MCP, Anda dapat memungkinkan alat individual mengembalikan hasil yang lebih besar dari ambang batas persist-to-disk default dengan menetapkan `_meta["anthropic/maxResultSizeChars"]` dalam entri respons `tools/list` alat. Claude Code menaikkan ambang batas alat tersebut ke nilai yang dianotasi, hingga batas keras 500.000 karakter.

Ini berguna untuk alat yang mengembalikan output yang secara inheren besar tetapi diperlukan, seperti skema database atau pohon file lengkap. Tanpa anotasi, hasil yang melebihi ambang batas default disimpan ke disk dan diganti dengan referensi file dalam percakapan.

```json theme={null}
{
  "name": "get_schema",
  "description": "Returns the full database schema",
  "_meta": {
    "anthropic/maxResultSizeChars": 200000
  }
}
```

Anotasi berlaku secara independen dari `MAX_MCP_OUTPUT_TOKENS` untuk konten teks, sehingga pengguna tidak perlu menaikkan variabel lingkungan untuk alat yang mendeklarasikannya. Alat yang mengembalikan data gambar masih tunduk pada batas token.

<Warning>
  Jika Anda sering mengalami peringatan output dengan server MCP tertentu yang tidak Anda kontrol, pertimbangkan untuk meningkatkan batas `MAX_MCP_OUTPUT_TOKENS`. Anda juga dapat meminta penulis server untuk menambahkan anotasi `anthropic/maxResultSizeChars` atau untuk membuat pagina respons mereka. Anotasi tidak berpengaruh pada alat yang mengembalikan konten gambar; untuk itu, menaikkan `MAX_MCP_OUTPUT_TOKENS` adalah satu-satunya pilihan.
</Warning>

<h2 id="tool-input-schemas-with-a-root-level-combinator">
  Skema input tool dengan combinator tingkat root
</h2>

Beberapa server MCP mendeklarasikan skema input tool sebagai union JSON Schema, dengan `anyOf`, `oneOf`, atau `allOf` di tingkat atas skema. Claude API tidak menerima kata kunci tersebut di root skema. API menerima combinator yang bersarang di dalam `properties`, yang Claude Code kirimkan tanpa perubahan.

Tool dengan combinator tingkat root tetap tersedia. Sebelum mengirim tool ke API, Claude Code meratakan skema menjadi satu objek dan menambahkan kalimat di awal deskripsi tool yang memberi tahu Claude kelompok parameter mana yang termasuk bersama:

* `allOf`: properti dari setiap cabang digabungkan, dan daftar `required` setiap cabang masih berlaku
* `anyOf` dan `oneOf`: properti dari setiap cabang digabungkan, dan daftar `required` setiap cabang dijelaskan dalam deskripsi tool alih-alih diterapkan oleh skema

Server Anda menerima argumen apa pun yang dipilih Claude, jadi terus validasi kombinasi di sisi server.

Ketika Claude Code tidak dapat menghasilkan skema yang diterima API, atau pada deployment yang tidak menerima konfigurasi jarak jauh yang mengaktifkan penulisan ulang, Claude Code melewati satu tool tersebut, mencatat alasannya dalam log server, dan membiarkan tool server lainnya tetap tersedia. Versi lebih awal dari v2.1.195 melewati setiap tool yang skema input-nya memiliki root-level `anyOf`, `oneOf`, atau `allOf`.

<h2 id="tools-with-invalid-input-schemas">
  Tools dengan skema input yang tidak valid
</h2>

Claude API memeriksa skema input setiap tool dalam permintaan dan menolak seluruh permintaan ketika salah satu skema gagal, sehingga satu tool MCP dengan skema yang salah bentuk akan membuat setiap permintaan yang menyertakannya gagal dengan kesalahan 400. Claude Code menjalankan dua pemeriksaan API itu sendiri ketika memuat tools server dan mengecualikan setiap tool yang akan gagal dalam pemeriksaan tersebut, sehingga tools lainnya di server tetap berfungsi:

* Nama properti tingkat atas harus panjangnya 1 hingga 64 karakter dan hanya menggunakan huruf ASCII dan angka, `_`, `.`, dan `-`
* Skema harus valid terhadap meta-skema JSON Schema draft 2020-12. Claude Code menerapkan pemeriksaan ini pada skema yang tidak mendeklarasikan `$schema` dan skema yang mendeklarasikan draft 2020-12. Skema yang mendeklarasikan dialek lain melewati pemeriksaan ini, meskipun pemeriksaan nama properti di atas tetap berlaku

Claude Code menjalankan pemeriksaan setelah [penulisan ulang combinator tingkat root](#tool-input-schemas-with-a-root-level-combinator), pada skema yang sebenarnya akan dikirimkan.

Ketika Claude Code mengecualikan tool, tool tersebut mencatat alasannya dalam log server dan memberi tahu Claude tools mana yang dikecualikan dan mengapa, sehingga Anda dapat menanyakan kepada Claude mengapa tool hilang. Jika Anda memperbaiki skema di server, tool akan kembali lagi saat Claude Code memuat tools server berikutnya.

Claude Code mengaktifkan pengecualian melalui bendera fitur yang diambilnya dari Anthropic. Pada [deployment tempat pengambilan bendera dimatikan](/docs/id/env-vars#features-that-need-feature-flag-fetching), atau pada mesin yang bendanya tidak pernah tiba, seperti mesin yang terisolasi udara, Claude Code tetap menjalankan pemeriksaan dan mencatat dalam log server tool mana yang akan ditolak, tetapi tetap mengirimkan skema tool ke API. API menolak permintaan yang menyertakan skema tersebut dengan [kesalahan 400 yang menamai tool berdasarkan posisinya](/docs/id/errors#tool-input-schema-is-invalid). Sebelum v2.1.216, tidak ada deployment yang menjalankan pemeriksaan ini.

Penanganan [combinator tingkat root](#tool-input-schemas-with-a-root-level-combinator) terpisah dan mempertahankan perilakunya sendiri ketika pengambilan bendera dimatikan atau bendera tidak pernah tiba.

<h2 id="require-approval-for-a-specific-tool">
  Memerlukan persetujuan untuk alat tertentu
</h2>

Jika Anda membangun server MCP, Anda dapat menandai alat sebagai memerlukan persetujuan eksplisit pada setiap panggilan dengan menetapkan `_meta["anthropic/requiresUserInteraction"]` ke `true` dalam entri respons `tools/list` alat. Nilainya harus berupa boolean JSON `true`; nilai lainnya diabaikan.

Claude Code menampilkan prompt izin alat tersebut pada setiap panggilan, bahkan dalam [mode izin](/docs/id/permissions#permission-modes) `acceptEdits`, `auto`, dan `bypassPermissions`, dan tidak menawarkan opsi "jangan tanya lagi" untuk itu. [Aturan Allow](/docs/id/permissions#permission-rule-syntax) yang cocok dengan alat juga tidak melewati prompt. Dalam mode `dontAsk`, yang tidak pernah menampilkan prompt, Claude Code menolak panggilan sebagai gantinya.

Prompt harus mencapai seseorang. Dalam mode non-interaktif dengan [`--permission-prompt-tool`](/docs/id/cli-reference#cli-flags), hasil `allow` dari alat prompt untuk alat yang ditandai dikonversi menjadi penolakan dengan pesan `MCP tool requires user interaction; not supported via --permission-prompt-tool`. Callback [`canUseTool`](/docs/id/agent-sdk/permissions) dari Agent SDK menerima panggilan ini dan dapat menyetujuinya, karena aplikasi SDK Anda diharapkan menampilkannya kepada pengguna.

Gunakan ini untuk alat yang prompt izinnya sendiri adalah intinya, seperti langkah persetujuan atau pemberian akses di mana persetujuan otomatis berarti tidak ada manusia yang pernah setuju. Alat lain dari server yang sama mempertahankan perilaku izin normal mereka.

Entri `tools/list` berikut menandai satu alat sebagai selalu memerlukan persetujuan.

```json theme={null}
{
  "name": "grant_access",
  "description": "Requests access to a protected resource",
  "_meta": {
    "anthropic/requiresUserInteraction": true
  }
}
```

Anotasi `anthropic/requiresUserInteraction` memerlukan Claude Code v2.1.199 atau lebih baru. Versi sebelumnya mengabaikannya dan menerapkan alur izin standar.

Beberapa permukaan, seperti [Remote Control](/docs/id/remote-control) dan aplikasi yang dibangun di [Agent SDK](/docs/id/agent-sdk/overview), biasanya memungkinkan Anda menyetujui panggilan alat dengan satu ketukan. Untuk alat yang ditandai dengan anotasi ini, Claude Code menahan tindakan satu ketukan dan menampilkan prompt izin lengkap alat sebagai gantinya, sehingga persetujuan masih berasal dari seseorang yang menjawab prompt daripada ketukan.

Claude Code menahan persetujuan satu ketukan dengan cara yang sama untuk permintaan izin apa pun yang hanya dialog terminal yang dapat merender sepenuhnya, seperti yang membawa peringatan keamanan atau opsi selalu-izinkan yang permukaan jarak jauh tidak dapat tampilkan. Anda menjawab permintaan itu di dialog terminal daripada dari Remote Control. Memerlukan Claude Code v2.1.214 atau lebih baru.

<h2 id="respond-to-mcp-elicitation-requests">
  Merespons permintaan elicitation MCP
</h2>

Server MCP dapat meminta input terstruktur dari Anda di tengah tugas menggunakan elicitation. Ketika server membutuhkan informasi yang tidak dapat diperolehnya sendiri, Claude Code menampilkan dialog interaktif dan meneruskan respons Anda kembali ke server. Tidak ada konfigurasi yang diperlukan di pihak Anda: dialog elicitation muncul secara otomatis ketika server memintanya.

Server dapat meminta input dengan dua cara:

* **Form mode**: Claude Code menampilkan dialog dengan bidang formulir yang ditentukan oleh server (misalnya, prompt nama pengguna dan kata sandi). Isi bidang dan kirimkan.
* **URL mode**: Claude Code membuka URL browser untuk autentikasi atau persetujuan. Selesaikan alur di browser, kemudian konfirmasi di CLI.

Dalam URL mode, Claude Code meneruskan URL sebagai argumen baris perintah ke penanganan URL sistem Anda, dan membatasi seberapa lama argumen tersebut dapat diterima. Ketika URL, setelah diloloskan untuk baris perintah, melampaui batas tersebut, Anda hanya dapat menolak permintaan. Setiap karakter yang perlu diloloskan, seperti `%` atau `&`, dihitung empat kali terhadap batas: karakternya sendiri ditambah tiga karakter lolos. URL tanpa karakter tersebut mencapai batas pada sekitar 8.000 karakter. URL yang dibangun sebagian besar dari percent-escapes, di mana setiap karakter ketiga adalah `%`, mencapainya pada kira-kira 4.000.

Untuk merespons otomatis permintaan elicitation tanpa menampilkan dialog, gunakan hook [`Elicitation`](/docs/id/hooks#elicitation).

Jika Anda membangun server MCP yang menggunakan elicitation, lihat [spesifikasi elicitation MCP](https://modelcontextprotocol.io/docs/learn/client-concepts#elicitation) untuk detail protokol dan contoh skema.

<h2 id="use-mcp-resources">
  Gunakan sumber daya MCP
</h2>

Server MCP dapat mengekspos sumber daya yang dapat Anda referensikan menggunakan penyebutan @, mirip dengan cara Anda mereferensikan file.

<h3 id="reference-mcp-resources">
  Referensikan sumber daya MCP
</h3>

<Steps>
  <Step title="Daftar sumber daya yang tersedia">
    Ketik `@` dalam prompt Anda untuk melihat sumber daya yang tersedia dari semua server MCP yang terhubung. Sumber daya muncul bersama file dalam menu pelengkapan otomatis.
  </Step>

  <Step title="Referensikan sumber daya tertentu">
    Gunakan format `@server:protocol://resource/path` untuk mereferensikan sumber daya:

    ```text wrap theme={null}
    Can you analyze @github:issue://123 and suggest a fix?
    ```

    ```text wrap theme={null}
    Please review the API documentation at @docs:file://api/authentication
    ```
  </Step>

  <Step title="Referensi sumber daya ganda">
    Anda dapat mereferensikan beberapa sumber daya dalam satu prompt:

    ```text wrap theme={null}
    Compare @postgres:schema://users with @docs:file://database/user-model
    ```
  </Step>
</Steps>

<Tip>
  Tips:

  * Sumber daya diambil secara otomatis dan disertakan sebagai lampiran saat direferensikan
  * Jalur sumber daya dapat dicari dengan fuzzy dalam pelengkapan otomatis penyebutan @
  * Claude Code secara otomatis menyediakan alat untuk membuat daftar dan membaca sumber daya MCP ketika server mendukungnya
  * Sumber daya dapat berisi jenis konten apa pun yang disediakan server MCP (teks, JSON, data terstruktur, dll.)
</Tip>

<h2 id="scale-with-mcp-tool-search">
  Skalakan dengan pencarian tool MCP
</h2>

Pencarian tool menjaga penggunaan konteks MCP tetap rendah dengan menunda definisi tool hingga Claude membutuhkannya. Hanya nama tool dan instruksi server yang dimuat saat awal sesi, jadi menambahkan lebih banyak server MCP memiliki dampak minimal pada jendela konteks Anda. Claude Code tidak memberlakukan batas tool tetap per-server; batas praktisnya adalah anggaran jendela konteks Anda.

<Note>
  Pencarian tool tidak didukung pada Microsoft Foundry [deployment yang dihosting di Azure](https://platform.claude.com/docs/en/build-with-claude/claude-in-microsoft-foundry#hosting-options), yang menolaknya di sisi server: Claude Code mendeteksi penolakan dan memuat tool MCP di awal untuk deployment tersebut. [`ENABLE_TOOL_SEARCH`](#configure-tool-search) tidak dapat mengesampingkan ini, karena penolakan berasal dari deployment itu sendiri.
</Note>

<h3 id="for-mcp-server-authors">
  Untuk penulis server MCP
</h3>

Jika Anda membangun server MCP, bidang instruksi server menjadi lebih berguna dengan pencarian tool diaktifkan. Instruksi server membantu Claude memahami kapan harus mencari tool Anda, mirip dengan cara [skills](/docs/id/skills) bekerja.

Tambahkan instruksi server yang jelas dan deskriptif yang menjelaskan:

* Kategori tugas apa yang ditangani tool Anda
* Kapan Claude harus mencari tool Anda
* Kemampuan utama yang disediakan server Anda

Claude Code memotong setiap deskripsi tool dan instruksi server masing-masing pada 2.048 karakter secara default. Jaga agar ringkas, dan letakkan detail penting di dekat awal.

Untuk mengubah batas untuk setiap server MCP dalam sesi Anda, atur [`CLAUDE_CODE_MAX_MCP_DESCRIPTION_LENGTH`](/docs/id/env-vars#variables) ke sejumlah karakter. Variabel ini memerlukan Claude Code v2.1.280 atau lebih baru.

<h3 id="configure-tool-search">
  Konfigurasi pencarian tool
</h3>

Pencarian tool diaktifkan secara default: tool MCP ditunda dan ditemukan sesuai permintaan. Claude Code menonaktifkannya ketika `ANTHROPIC_BASE_URL` menunjuk ke host non-pihak pertama, karena sebagian besar proxy tidak meneruskan blok `tool_reference`. Atur `ENABLE_TOOL_SEARCH` secara eksplisit untuk mengesampingkan fallback tersebut.

Mengatur [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS`](/docs/id/env-vars) menjaga pencarian tool tetap mati. Anda tidak dapat mengesampingkannya dengan mengatur `ENABLE_TOOL_SEARCH` sendiri. Organisasi Anda dapat menjaga pencarian tool tetap aktif melalui [pengaturan terkelola](/docs/id/managed-settings), pada Claude Code v2.1.227 atau lebih baru. [Nonaktifkan kemampuan pra-rilis](/docs/id/llm-gateway-protocol#disable-pre-release-capabilities) mencakup tempat pengesampingan berlaku dan apa yang dihapus variabel.

Pencarian tool memerlukan model yang mendukung blok `tool_reference`: Claude Sonnet 4.5, Claude Haiku 4.5, Claude Opus 4.5, dan model yang lebih baru. Lihat [kompatibilitas model dalam dokumen API](https://platform.claude.com/docs/en/agents-and-tools/tool-use/tool-search-tool#model-compatibility) untuk daftar terkini.

Di Agent Platform Google Cloud, Claude Code memutuskan berdasarkan generasi model:

* **Claude Opus 4.5, Sonnet 4.5, Haiku 4.5, dan yang lebih baru**: pencarian tool aktif secara default, sama seperti di Anthropic API.
* **Model Agent Platform sebelumnya**: Claude Code memuat semua tool MCP di awal, karena stack penyajian mereka menolak header beta yang diperlukan. `ENABLE_TOOL_SEARCH=true` tidak mengesampingkan ini.

Sebelum v2.1.221, Claude Code menonaktifkan pencarian tool untuk semua model di Agent Platform Google Cloud kecuali Anda mengatur `ENABLE_TOOL_SEARCH=true`.

Kontrol perilaku pencarian tool dengan variabel lingkungan `ENABLE_TOOL_SEARCH`:

| Nilai          | Perilaku                                                                                                                                                                                                                                                                                                                                                                                                                |
| :------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| (tidak diatur) | Semua tool MCP ditunda dan dimuat sesuai permintaan. Kembali ke pemuatan di awal pada model Agent Platform Google Cloud yang lebih awal dari generasi Claude 4.5, ketika `ANTHROPIC_BASE_URL` adalah host non-pihak pertama, atau pada deployment Microsoft Foundry yang dihosting di Azure                                                                                                                             |
| `true`         | Semua tool MCP ditunda, kecuali pada deployment Microsoft Foundry yang dihosting di Azure, di mana penolakan sisi server masih memaksa pemuatan di awal, dan pada model Agent Platform Google Cloud yang lebih awal dari generasi Claude 4.5, di mana Claude Code terus memuat tool di awal. Claude Code mengirim header beta melalui proxy, dan permintaan gagal pada proxy yang tidak mendukung blok `tool_reference` |
| `auto`         | Mode ambang: Claude Code memuat tool yang akan ditunda di awal sementara definisinya berjumlah kurang dari 10% jendela konteks, dan menunda semuanya setelah definisi mencapai 10%                                                                                                                                                                                                                                      |
| `auto:N`       | Mode ambang dengan persentase khusus, di mana `N` adalah 0-100. Misalnya, `auto:5` untuk 5%                                                                                                                                                                                                                                                                                                                             |
| `false`        | Semua tool MCP dimuat di awal, tidak ada penundaan                                                                                                                                                                                                                                                                                                                                                                      |

```bash theme={null}
# Gunakan ambang khusus 5%
ENABLE_TOOL_SEARCH=auto:5 claude

# Nonaktifkan pencarian tool sepenuhnya
ENABLE_TOOL_SEARCH=false claude
```

Atau atur nilai dalam [bidang `env` settings.json](/docs/id/settings-reference#env) Anda.

Anda juga dapat menonaktifkan tool `ToolSearch` secara khusus:

```json theme={null}
{
  "permissions": {
    "deny": ["ToolSearch"]
  }
}
```

<h3 id="exempt-a-server-from-deferral">
  Bebaskan server dari penundaan
</h3>

Jika tool server harus selalu terlihat oleh Claude tanpa langkah pencarian, atur `alwaysLoad` ke `true` dalam konfigurasi server tersebut. Setiap tool dari server tersebut kemudian dimuat ke dalam konteks saat awal sesi terlepas dari pengaturan `ENABLE_TOOL_SEARCH`. Gunakan ini untuk sejumlah kecil tool yang Claude butuhkan di setiap giliran, karena setiap tool di awal mengonsumsi konteks yang akan tersedia untuk percakapan Anda.

Entri `.mcp.json` berikut membebaskan satu server HTTP sambil membiarkan server lain ditunda:

```json theme={null}
{
  "mcpServers": {
    "core-tools": {
      "type": "http",
      "url": "https://mcp.example.com/mcp",
      "alwaysLoad": true
    }
  }
}
```

Bidang `alwaysLoad` tersedia di semua jenis server. Server MCP juga dapat menandai tool individual sebagai selalu-dimuat dengan menyertakan `"anthropic/alwaysLoad": true` dalam objek `_meta` tool, yang memiliki efek yang sama hanya untuk tool tersebut.

Mengatur `alwaysLoad: true` juga membuat startup menunggu tool server, dibatasi pada timeout koneksi standar 5 detik, karena mereka harus ada ketika prompt pertama dibangun. Server jarak jauh dengan entri [`cached`](#server-status-detail) yang valid menyediakan tool-nya dari cache tanpa terhubung, jadi tidak menahan startup. Server lain terhubung di latar belakang secara default; atur [`MCP_CONNECTION_NONBLOCKING=0`](/docs/id/env-vars) untuk membuat startup menunggu mereka juga.

<h2 id="use-mcp-prompts-as-commands">
  Gunakan MCP prompts sebagai perintah
</h2>

Server MCP dapat mengekspos prompts yang menjadi tersedia sebagai perintah di Claude Code.

<h3 id="execute-mcp-prompts">
  Jalankan MCP prompts
</h3>

<Steps>
  <Step title="Temukan prompts yang tersedia">
    Ketik `/` untuk melihat perintah yang tersedia untuk Anda, termasuk yang dari server MCP. Claude Code mencantumkan setiap MCP prompt sebagai `/servername:promptname (MCP)`. Mengetik `/mcp__servername__promptname` juga menjalankannya.
  </Step>

  <Step title="Jalankan prompt tanpa argumen">
    ```text wrap theme={null}
    /mcp__github__list_prs
    ```
  </Step>

  <Step title="Jalankan prompt dengan argumen">
    Banyak prompts menerima argumen. Berikan mereka terpisah dengan spasi setelah perintah. Claude Code membagi argumen pada whitespace, jadi setiap argumen adalah satu token:

    ```text wrap theme={null}
    /mcp__github__pr_review 456
    ```

    ```text wrap theme={null}
    /mcp__jira__create_issue login-bug high
    ```
  </Step>
</Steps>

<Tip>
  Tips:

  * MCP prompts ditemukan secara dinamis dari server yang terhubung
  * Argumen diuraikan berdasarkan parameter yang ditentukan oleh prompt
  * Hasil prompt disuntikkan langsung ke dalam percakapan
  * Dalam bentuk `/mcp__servername__promptname`, Claude Code mengganti karakter apa pun dalam nama server di luar `A-Z`, `a-z`, `0-9`, `_`, dan `-` dengan `_`, dan menggunakan nama prompt seperti yang dideklarasikan server
</Tip>

<h2 id="managed-mcp-configuration">
  Konfigurasi MCP yang dikelola
</h2>

Untuk organisasi yang memerlukan kontrol terpusat atas server MCP mana yang dapat dihubungkan pengguna, lihat [Konfigurasi MCP yang dikelola](/docs/id/managed-mcp). Ini mencakup penerapan set server tetap dengan `managed-mcp.json`, menyediakan server kepada setiap pengguna dengan `managedMcpServers`, membatasi server dengan `allowedMcpServers` dan `deniedMcpServers`, dan apa yang dilihat pengguna ketika server diblokir.
