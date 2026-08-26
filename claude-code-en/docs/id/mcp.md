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

    Jika Claude Code melaporkan bahwa marketplace tidak ditemukan, jalankan `/plugin marketplace add anthropics/claude-plugins-official` terlebih dahulu, kemudian coba lagi instalnya. Setelah diinstal, jalankan `/reload-plugins` untuk mengaktifkannya dalam sesi saat ini.
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

Server MCP dapat dikonfigurasi dengan beberapa cara berbeda tergantung pada kebutuhan Anda:

<h3 id="option-1-add-a-remote-http-server">
  Opsi 1: Tambahkan server HTTP jarak jauh
</h3>

Server HTTP adalah opsi yang direkomendasikan untuk terhubung ke server MCP jarak jauh. Ini adalah transport yang paling banyak didukung untuk layanan berbasis cloud.

```bash theme={null}
# Sintaks dasar
claude mcp add --transport http <name> <url>

# Contoh nyata: Hubungkan ke Notion
claude mcp add --transport http notion https://mcp.notion.com/mcp

# Contoh dengan token Bearer
claude mcp add --transport http secure-api https://api.example.com/mcp \
  --header "Authorization: Bearer your-token"
```

Saat mengonfigurasi server MCP melalui JSON dalam `.mcp.json`, `~/.claude.json`, atau `claude mcp add-json`, bidang `type` menerima `streamable-http` sebagai alias untuk `http`. Spesifikasi MCP menggunakan nama `streamable-http` untuk transport ini, jadi konfigurasi yang disalin dari dokumentasi server berfungsi tanpa modifikasi.

Entri JSON yang memiliki `url` tetapi tidak ada `type` adalah kesalahan konfigurasi, karena Claude Code membaca entri tanpa `type` sebagai server stdio. Claude Code melewati server tersebut dan melaporkan `MCP server "<name>" has a "url" but no "type"; add "type": "http" (or "sse" / "ws") to this entry`. Sebelum v2.1.202, Claude Code melaporkan kesalahan konfigurasi ini sebagai `command: expected string, received undefined`.

<h3 id="option-2-add-a-remote-sse-server">
  Opsi 2: Tambahkan server SSE jarak jauh
</h3>

<Warning>
  Transport SSE (Server-Sent Events) sudah usang. Gunakan server HTTP sebagai gantinya, jika tersedia.
</Warning>

```bash theme={null}
# Sintaks dasar
claude mcp add --transport sse <name> <url>

# Contoh nyata: Hubungkan ke Asana
claude mcp add --transport sse asana https://mcp.asana.com/sse

# Contoh dengan header autentikasi
claude mcp add --transport sse private-api https://api.company.com/sse \
  --header "X-API-Key: your-key-here"
```

<h3 id="option-3-add-a-local-stdio-server">
  Opsi 3: Tambahkan server stdio lokal
</h3>

Server stdio berjalan sebagai proses lokal di mesin Anda. Mereka ideal untuk alat yang memerlukan akses sistem langsung atau skrip khusus.

Claude Code menetapkan `CLAUDE_PROJECT_DIR` di lingkungan server yang dihasilkan ke akar proyek, sehingga server Anda dapat menyelesaikan jalur relatif proyek tanpa bergantung pada direktori kerja. Ini adalah direktori yang sama yang diterima hooks dalam variabel `CLAUDE_PROJECT_DIR` mereka. Bacalah dari dalam proses server Anda, misalnya `process.env.CLAUDE_PROJECT_DIR` di Node atau `os.environ["CLAUDE_PROJECT_DIR"]` di Python.

`CLAUDE_PROJECT_DIR` adalah akar proyek yang stabil dan tidak berubah ketika Anda menambah atau menghapus direktori kerja di tengah sesi. Server yang membatasi akses sistem file-nya sendiri ke serangkaian direktori yang diizinkan harus mengimplementasikan permintaan MCP `roots/list` sebagai gantinya. Claude Code menjawab `roots/list` dengan direktori peluncuran sesi ditambah setiap [direktori kerja tambahan](/docs/id/permissions#working-directories) yang telah Anda berikan dengan `--add-dir`, `/add-dir`, atau pengaturan `additionalDirectories`. Claude Code mengirim `notifications/roots/list_changed` ketika set tersebut berubah. Sebelum v2.1.203, `roots/list` hanya mengembalikan direktori peluncuran dan Claude Code tidak mengirim `notifications/roots/list_changed`.

Variabel ini ditetapkan di lingkungan server, bukan di lingkungan Claude Code itu sendiri, jadi mereferensikannya melalui ekspansi `${VAR}` dalam `.mcp.json` yang bersifat proyek atau pengguna `command` atau `args` memerlukan default seperti `${CLAUDE_PROJECT_DIR:-.}`. Konfigurasi MCP yang disediakan plugin mengganti `${CLAUDE_PROJECT_DIR}` secara langsung dan tidak memerlukan default.

```bash theme={null}
# Sintaks dasar
claude mcp add [options] <name> -- <command> [args...]

# Contoh nyata: Tambahkan server Airtable
claude mcp add --env AIRTABLE_API_KEY=YOUR_KEY --transport stdio airtable \
  -- npx -y airtable-mcp-server
```

<Note>
  **Penting: Pisahkan argumen server dengan `--`**

  Untuk server stdio, `--` (tanda hubung ganda) memisahkan opsi Claude sendiri, seperti `--transport`, `--env`, dan `--scope`, dari perintah dan argumen yang menjalankan server. Semuanya setelah `--` diteruskan ke server tanpa diubah.

  Sebagai contoh:

  * `claude mcp add --transport stdio myserver -- npx server` → menjalankan `npx server`
  * `claude mcp add --env KEY=value --transport stdio myserver -- python server.py --port 8080` → menjalankan `python server.py --port 8080` dengan `KEY=value` di lingkungan

  Tanpa `--`, Claude Code akan mencoba mengurai flag server, seperti `--port` di atas, sebagai opsinya sendiri.

  `--env` menerima beberapa pasangan `KEY=value`. Jika nama server datang langsung setelah `--env`, CLI membaca nama sebagai pasangan lain dan menolaknya, jadi tempatkan setidaknya satu opsi lain antara `--env` dan nama server, seperti dalam contoh di atas.
</Note>

<h3 id="option-4-add-a-remote-websocket-server">
  Opsi 4: Tambahkan server WebSocket jarak jauh
</h3>

Server WebSocket mempertahankan koneksi bidirectional yang persisten, yang cocok untuk server MCP jarak jauh yang mendorong peristiwa ke Claude tanpa diminta. Gunakan HTTP sebagai gantinya ketika server Anda hanya merespons permintaan, karena HTTP mendukung OAuth dan flag `claude mcp add --transport`, sementara WebSocket tidak mendukung keduanya.

Konfigurasi server WebSocket dalam `.mcp.json` atau dengan `claude mcp add-json`:

```bash theme={null}
claude mcp add-json events-server \
  '{"type":"ws","url":"wss://mcp.example.com/socket","headers":{"Authorization":"Bearer YOUR_TOKEN"}}'
```

Entri `type: "ws"` menerima bidang `url`, `headers`, `headersHelper`, `timeout`, dan `alwaysLoad` yang sama seperti `http`. Autentikasi hanya header, jadi teruskan token statis dalam `headers` atau buat satu pada waktu koneksi dengan [`headersHelper`](#use-dynamic-headers-for-custom-authentication). Flag `claude mcp add --transport` tidak menerima `ws`.

<h3 id="managing-your-servers">
  Mengelola server Anda
</h3>

Setelah dikonfigurasi, Anda dapat mengelola server MCP Anda dengan perintah ini:

```bash theme={null}
# Daftar semua server yang dikonfigurasi
claude mcp list

# Dapatkan detail untuk server tertentu
claude mcp get github

# Hapus server
claude mcp remove github

# (dalam Claude Code) Periksa status server
/mcp
```

Server yang bersifat proyek dari `.mcp.json` yang menunggu persetujuan Anda muncul dalam `claude mcp list` sebagai `⏸ Pending approval`. Jalankan `claude` secara interaktif untuk meninjau dan menyetujui mereka. `claude mcp get <name>` menampilkan server yang tertunda sebagai `⏸ Pending approval` dan server yang ditolak sebagai `✗ Rejected`.

Mulai dari v2.1.196, `claude mcp list` dan `claude mcp get` membaca persetujuan `.mcp.json` hanya dari file pengaturan yang tidak dimasukkan ke dalam repositori sampai Anda mempercayai workspace dengan menjalankan `claude` di dalamnya dan menerima dialog kepercayaan workspace. Repositori yang diklon tidak dapat menyetujui server-nya sendiri: [`enableAllProjectMcpServers` atau `enabledMcpjsonServers`](/docs/id/settings#available-settings) yang dikomitkan ke `.claude/settings.json` proyek diabaikan dalam folder yang tidak dipercaya, dan server tetap di `⏸ Pending approval` alih-alih terhubung dan diperiksa kesehatannya.

Persetujuan dari sumber-sumber ini masih berlaku dalam folder yang tidak dipercaya:

* `~/.claude/settings.json` pengguna Anda
* pengaturan yang dikelola
* pengaturan yang diteruskan dengan `--settings`

Persetujuan dalam `.claude/settings.local.json` yang tidak dilacak juga berlaku, tetapi hanya setelah Anda menerima dialog kepercayaan untuk folder tersebut atau salah satu direktori induknya: Claude Code menjalankan git untuk memeriksa apakah file dilacak, dan menjalankan pemeriksaan itu hanya dalam folder yang dipercaya. Dalam folder yang belum pernah Anda percayai, persetujuan file menunggu dialog kepercayaan kecuali folder adalah rumah konfigurasi Anda sendiri: direktori rumah Anda, atau direktori yang `.claude` Anda telah atur sebagai [`CLAUDE_CONFIG_DIR`](/docs/id/env-vars). Sebelum v2.1.207, `.claude/settings.local.json` yang tidak dilacak menyetujui server dalam folder yang belum pernah Anda percayai.

Entri `disabledMcpjsonServers` dalam file pengaturan apa pun masih menolak server.

Panel `/mcp` menampilkan jumlah alat di sebelah setiap server yang terhubung dan menandai server yang mengiklankan kemampuan alat tetapi tidak mengekspos alat apa pun.

Server jarak jauh yang konfigurasinya memiliki `url` kosong ditampilkan sebagai `not configured` dalam `/mcp`, dalam `claude mcp list`, dan dalam [manajer `/plugin`](/docs/id/plugins), dan Claude Code tidak mencoba terhubung ke server tersebut. Plugin dapat menyertakan entri placeholder seperti ini untuk konektor yang Anda konfigurasi nanti, sehingga Claude Code tidak melaporkannya sebagai kesalahan atau masalah pengaturan. Tampilan detail server dalam `/mcp` membaca `No URL configured for this server`; atur `url` entri untuk menghubungkannya. Sebelum v2.1.208, Claude Code melaporkan `url` kosong sebagai masalah konfigurasi dengan prompt untuk menghubungkan kembali.

Jika permintaan Anda memerlukan alat dari server yang masih terhubung di latar belakang, Claude menunggu server tersebut sebelum melanjutkan. Dengan [pencarian alat](#scale-with-mcp-tool-search) diaktifkan, yang merupakan default, penantian terjadi di dalam panggilan `ToolSearch`. Dalam konfigurasi tanpa pencarian alat, seperti Platform Agen Google Cloud, `ANTHROPIC_BASE_URL` khusus, atau `ENABLE_TOOL_SEARCH=false`, Claude menggunakan alat `WaitForMcpServers` sebagai gantinya.

Beberapa nama server dicadangkan untuk server bawaan Claude Code: `workspace`, `claude-in-chrome`, `computer-use`, `Claude Preview`, dan `Claude Browser`. Jika konfigurasi Anda menentukan server dengan nama yang dicadangkan, Claude Code melewatinya saat waktu muat dan menampilkan peringatan yang meminta Anda untuk mengganti namanya. `claude mcp add` menolak nama yang dicadangkan dengan kesalahan.

`Claude Preview` dan `Claude Browser` keduanya menamai server bawaan yang digunakan oleh [panel pratinjau aplikasi desktop Claude Code](/docs/id/desktop#preview-your-app). Sebelum v2.1.205, `Claude Browser` tidak dicadangkan, jadi server yang dikonfigurasi pengguna dapat terdaftar di bawah nama tersebut.

<h3 id="dynamic-tool-updates">
  Pembaruan alat dinamis
</h3>

Claude Code mendukung notifikasi `list_changed` MCP, memungkinkan server MCP untuk secara dinamis memperbarui alat, prompt, dan sumber daya yang tersedia tanpa memerlukan Anda untuk memutuskan dan menghubungkan kembali. Ketika server MCP mengirim notifikasi `list_changed`, Claude Code secara otomatis menyegarkan kemampuan yang tersedia dari server tersebut.

<h3 id="automatic-reconnection">
  Koneksi ulang otomatis
</h3>

Jika server HTTP atau SSE terputus di tengah sesi, Claude Code secara otomatis menghubungkan kembali dengan backoff eksponensial: hingga lima upaya, dimulai dengan penundaan satu detik dan berlipat ganda setiap kali. Server muncul sebagai tertunda dalam `/mcp` saat koneksi ulang sedang berlangsung. Setelah lima upaya gagal, server ditandai sebagai gagal dan Anda dapat mencoba lagi secara manual dari `/mcp`. Server stdio adalah proses lokal dan tidak dihubungkan kembali secara otomatis.

Backoff yang sama berlaku ketika server HTTP atau SSE gagal koneksi awalnya saat startup. Mulai dari v2.1.121, Claude Code mencoba ulang koneksi awal hingga tiga kali pada kesalahan transien seperti respons 5xx, koneksi ditolak, atau timeout, kemudian menandai server sebagai gagal jika masih tidak dapat terhubung. Kesalahan autentikasi dan not-found tidak dicoba ulang karena memerlukan perubahan konfigurasi untuk diselesaikan.

Ketika server yang dikonfigurasi gagal terhubung, Claude Code memberi tahu Claude server mana yang gagal dan kesalahan koneksinya, termasuk dalam hasil `ToolSearch` yang tidak menemukan alat yang cocok, sehingga Claude melaporkan kegagalan koneksi dalam responsnya. Memerlukan [pencarian alat](#scale-with-mcp-tool-search), yang diaktifkan secara default. Dalam konfigurasi tanpa pencarian alat, seperti `ANTHROPIC_BASE_URL` khusus, `ENABLE_TOOL_SEARCH=false`, atau model yang tidak mendukung pencarian alat, dan di Amazon Bedrock, Platform Agen Google Cloud, dan Microsoft Foundry, Claude Code tidak melaporkan kegagalan koneksi server yang gagal kepada Claude. Sebelum v2.1.205, Claude Code tidak meneruskan kesalahan koneksi kepada Claude, dan Claude dapat merespons seolah-olah alat server yang gagal tidak pernah dikonfigurasi.

Mulai dari v2.1.191, permintaan penemuan kemampuan yang berjalan setelah koneksi yang berhasil, seperti `tools/list`, `prompts/list`, dan `resources/list`, juga mencoba ulang kesalahan jaringan transien dan server hingga tiga kali dengan backoff pendek. Kesalahan autentikasi, respons 4xx, dan timeout permintaan tidak dicoba ulang.

<h3 id="push-messages-with-channels">
  Dorong pesan dengan saluran
</h3>

Server MCP juga dapat mendorong pesan langsung ke dalam sesi Anda sehingga Claude dapat bereaksi terhadap peristiwa eksternal seperti hasil CI, peringatan pemantauan, atau pesan obrolan. Untuk mengaktifkan ini, server Anda mendeklarasikan kemampuan `claude/channel` dan Anda memilihnya dengan flag `--channels` saat startup. Lihat [Saluran](/docs/id/channels) untuk menggunakan saluran yang didukung secara resmi, atau [Referensi saluran](/docs/id/channels-reference) untuk membangun saluran Anda sendiri.

<Tip>
  Tips:

  * Gunakan flag `-s` atau `--scope` untuk menentukan di mana konfigurasi disimpan:
    * `local` (default): Hanya tersedia untuk Anda di proyek saat ini. Versi yang lebih lama menyebut cakupan ini `project`
    * `project`: Dibagikan dengan semua orang di proyek melalui file `.mcp.json`
    * `user`: Tersedia untuk Anda di semua proyek. Versi yang lebih lama menyebut cakupan ini `global`
  * Atur variabel lingkungan dengan flag `-e` atau `--env` (misalnya, `-e KEY=value`)
  * Flag `--transport` dan `--header` juga menerima bentuk pendek `-t` dan `-H`
  * Konfigurasi waktu tunggu startup server MCP menggunakan variabel lingkungan `MCP_TIMEOUT` (misalnya, `MCP_TIMEOUT=10000 claude` menetapkan waktu tunggu 10 detik)
  * Atur waktu tunggu eksekusi alat per-server dengan menambahkan bidang `timeout` dalam milidetik ke entri `.mcp.json` server tersebut, misalnya `"timeout": 600000` untuk sepuluh menit. Ini menggantikan variabel lingkungan `MCP_TOOL_TIMEOUT` hanya untuk server tersebut
  * Claude Code menampilkan peringatan ketika output alat MCP melebihi 10.000 token dan membatasi output hingga 25.000 token secara default. Untuk meningkatkan batas, atur variabel lingkungan `MAX_MCP_OUTPUT_TOKENS` (misalnya, `MAX_MCP_OUTPUT_TOKENS=50000`); ambang peringatan tetap. Lihat [Batas dan peringatan output MCP](#mcp-output-limits-and-warnings)
  * Gunakan `/mcp` untuk autentikasi dengan server jarak jauh yang memerlukan autentikasi OAuth 2.0
</Tip>

Waktu tunggu per-server `timeout` adalah batas dinding jam yang keras per panggilan alat, dan notifikasi kemajuan dari server tidak memperpanjangnya. Nilai di bawah 1000 diabaikan dan jatuh melalui `MCP_TOOL_TIMEOUT`, atau ke default-nya sekitar 28 jam ketika variabel tersebut tidak diatur. Untuk server HTTP, SSE, atau [konektor claude.ai](/docs/id/mcp#use-mcp-servers-from-claude-ai) ada juga timer per-permintaan kedua yang mencakup setiap permintaan hingga byte respons pertama server. Timer itu adalah 60 detik kecuali Anda menetapkan waktu tunggu per-server `timeout` atau `MCP_TOOL_TIMEOUT`; menetapkan salah satu ke 60 detik atau lebih tinggi menaikkan timer per-permintaan ke nilai itu, nilai yang lebih rendah tidak mempersingkatnya, dan default 28 jam dari `MCP_TOOL_TIMEOUT` yang tidak diatur tidak pernah memberinya makan. Server stdio dan WebSocket tidak memiliki timer per-permintaan. Sebelum v2.1.162, nilai di bawah 1000 dibulatkan ke satu detik sebagai gantinya.

Waktu tunggu per-server `timeout` setidaknya 1000 juga bertindak sebagai lantai pada timeout idle yang dijelaskan di bawah: Claude Code tidak pernah membatalkan panggilan alat server tersebut karena menganggur lebih cepat daripada waktu tunggu per-server `timeout`. Memerlukan Claude Code v2.1.203 atau lebih baru.

Panggilan alat ke server MCP yang tidak mengirim respons dan tidak ada notifikasi kemajuan untuk jendela idle akan dibatalkan dengan kesalahan alih-alih menunggu batas dinding jam. Timeout idle memerlukan Claude Code v2.1.187 atau lebih baru. Ini berlaku untuk setiap jenis server kecuali server IDE dan server in-process SDK. Jendela idle default ke lima menit untuk HTTP, SSE, WebSocket, dan server [konektor claude.ai](#use-mcp-servers-from-claude-ai), dan ke 30 menit untuk server stdio. Sebelum v2.1.203, server stdio dikecualikan dari timeout idle.

Atur variabel lingkungan [`CLAUDE_CODE_MCP_TOOL_IDLE_TIMEOUT`](/docs/id/env-vars) dalam milidetik untuk mengubah jendela idle, atau atur ke `0` untuk menonaktifkan pemeriksaan.

<h3 id="plugin-provided-mcp-servers">
  Server MCP yang disediakan plugin
</h3>

[Plugin](/docs/id/plugins) dapat menggabungkan server MCP, secara otomatis menyediakan alat dan integrasi ketika plugin diaktifkan. Server MCP plugin bekerja identik dengan server yang dikonfigurasi pengguna.

**Cara kerja server MCP plugin**:

* Plugin menentukan server MCP dalam `.mcp.json` di root plugin atau inline dalam `plugin.json`
* Ketika plugin diaktifkan, server MCP-nya dimulai secara otomatis
* Alat MCP plugin muncul bersama alat MCP yang dikonfigurasi secara manual
* Server plugin dikelola melalui instalasi plugin, bukan perintah `/mcp`

**Contoh konfigurasi MCP plugin**:

Dalam `.mcp.json` di root plugin:

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

Atau inline dalam `plugin.json`:

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

* **Siklus hidup otomatis**: Pada startup sesi, server untuk plugin yang diaktifkan terhubung secara otomatis. Jika Anda mengaktifkan atau menonaktifkan plugin selama sesi, jalankan `/reload-plugins` untuk menghubungkan atau memutuskan server MCP-nya
* **Variabel placeholder jalur**: `${CLAUDE_PLUGIN_ROOT}` diselesaikan ke direktori instalasi plugin, `${CLAUDE_PLUGIN_DATA}` ke direktori [status persisten](/docs/id/plugins-reference#persistent-data-directory)-nya, dan `${CLAUDE_PROJECT_DIR}` ke akar proyek yang stabil. Substitusi berlaku untuk:
  * server `stdio`: `command`, `args`, `env`
  * server `http`, `sse`, dan `ws`: `url`, `headers`, dan `headersHelper`. Sebelum v2.1.195, `headersHelper` meneruskan placeholder sebagai string literal
* **Akses lingkungan pengguna**: Akses ke variabel lingkungan yang sama seperti server yang dikonfigurasi secara manual
* **Jenis transport berganda**: Dukungan transport stdio, SSE, HTTP, dan WebSocket, meskipun dukungan transport dapat bervariasi menurut server

**Melihat server MCP plugin**:

```bash theme={null}
# Dalam Claude Code, lihat semua server MCP termasuk yang dari plugin
/mcp
```

Server plugin muncul dalam daftar dengan indikator yang menunjukkan mereka berasal dari plugin.

**Nama alat MCP plugin**:

Alat dari server MCP bundel plugin mencakup nama plugin dan kunci server dalam nama yang dapat dipanggil. Bentuk lengkapnya adalah `mcp__plugin_<plugin-name>_<server-name>__<tool-name>`, di mana karakter apa pun di luar `A-Z`, `a-z`, `0-9`, `_`, dan `-` diganti dengan `_`. Untuk server `database-tools` yang dibundel dalam plugin bernama `my-plugin`, alat `query` dapat dipanggil sebagai:

```
mcp__plugin_my-plugin_database-tools__query
```

Gunakan nama lengkap ini saat mereferensikan alat dalam [aturan izin](/docs/id/permissions), daftar `allowed-tools` skill, [bidang `tools` subagent](/docs/id/sub-agents#available-tools), atau [pencocokan hook](/docs/id/hooks#match-mcp-tools). Pencocokan hook yang ditulis terhadap kunci server telanjang, seperti `mcp__database-tools__.*`, tidak pernah aktif untuk server bundel plugin.

Server itu sendiri terdaftar di bawah nama yang bersifat scoped `plugin:<plugin-name>:<server-name>`, seperti `plugin:my-plugin:database-tools`. Gunakan nama tersebut di mana nama server yang dikonfigurasi diharapkan, seperti [bidang `server` hook `mcp_tool`](/docs/id/hooks#mcp-tool-hook-fields).

**Manfaat server MCP plugin**:

* **Distribusi bundel**: Alat dan server dikemas bersama
* **Pengaturan otomatis**: Tidak perlu konfigurasi MCP manual
* **Konsistensi tim**: Semua orang mendapatkan alat yang sama ketika plugin diinstal

Lihat [referensi komponen plugin](/docs/id/plugins-reference#mcp-servers) untuk detail tentang penggabungan server MCP dengan plugin.

<h2 id="mcp-installation-scopes">
  Cakupan instalasi MCP
</h2>

Server MCP dapat dikonfigurasi pada tiga cakupan berbeda. Cakupan yang Anda pilih mengontrol proyek mana tempat server dimuat dan apakah konfigurasi dibagikan dengan tim Anda. Administrator juga dapat menerapkan server di tingkat enterprise melalui [konfigurasi terkelola](#managed-mcp-configuration).

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
  Istilah "cakupan lokal" untuk server MCP berbeda dari pengaturan lokal umum. Server MCP dengan cakupan lokal disimpan dalam `~/.claude.json` (direktori home Anda), sementara pengaturan lokal umum menggunakan `.claude/settings.local.json` (di direktori proyek). Lihat [Pengaturan](/docs/id/settings#settings-files) untuk detail tentang lokasi file pengaturan.
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

Server dengan cakupan proyek memungkinkan kolaborasi tim dengan menyimpan konfigurasi dalam file `.mcp.json` di direktori root proyek Anda. File ini dirancang untuk diperiksa ke dalam kontrol versi, memastikan semua anggota tim memiliki akses ke alat dan layanan MCP yang sama. Ketika Anda menambahkan server dengan cakupan proyek, Claude Code secara otomatis membuat atau memperbarui file ini dengan struktur konfigurasi yang sesuai.

```bash theme={null}
# Tambahkan server dengan cakupan proyek
claude mcp add --transport http paypal --scope project https://mcp.paypal.com/mcp
```

File `.mcp.json` yang dihasilkan mengikuti format standar:

```json theme={null}
{
  "mcpServers": {
    "shared-server": {
      "command": "/path/to/server",
      "args": [],
      "env": {}
    }
  }
}
```

Untuk alasan keamanan, Claude Code meminta persetujuan sebelum menggunakan server dengan cakupan proyek dari file `.mcp.json`. Jika Anda perlu mengatur ulang pilihan persetujuan ini, gunakan perintah `claude mcp reset-project-choices`.

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
4. [Server yang disediakan plugin](/docs/id/plugins)
5. [Konektor claude.ai](#use-mcp-servers-from-claude-ai)

Tiga cakupan mencocokkan duplikat berdasarkan nama. Plugin dan konektor mencocokkan berdasarkan endpoint, jadi yang menunjuk ke URL atau perintah yang sama dengan server di atas diperlakukan sebagai duplikat.

<h3 id="environment-variable-expansion-in-mcp-json">
  Ekspansi variabel lingkungan dalam `.mcp.json`
</h3>

Claude Code mendukung ekspansi variabel lingkungan dalam file `.mcp.json`, memungkinkan tim untuk berbagi konfigurasi sambil mempertahankan fleksibilitas untuk jalur spesifik mesin dan nilai sensitif seperti kunci API.

**Sintaks yang didukung:**

* `${VAR}`: berkembang menjadi nilai variabel lingkungan `VAR`
* `${VAR:-default}`: berkembang menjadi `VAR` jika diatur, jika tidak menggunakan `default`

**Lokasi ekspansi:**
Variabel lingkungan dapat berkembang dalam:

* `command`: jalur executable server
* `args`: argumen baris perintah
* `env`: variabel lingkungan yang diteruskan ke server
* `url`: untuk jenis server HTTP
* `headers`: untuk autentikasi server HTTP

**Contoh dengan ekspansi variabel:**

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

Jika variabel lingkungan yang dirujuk tidak diatur dan tidak memiliki nilai default, Claude Code membiarkan teks literal `${VAR}` dalam nilai dan melaporkan peringatan variabel yang hilang untuk server tersebut. Konfigurasi masih dimuat, jadi atur variabel atau tambahkan fallback `:-default` sehingga server dimulai dengan nilai yang Anda maksudkan.

<h2 id="practical-examples">
  Contoh praktis
</h2>

<h3 id="example-monitor-errors-with-sentry">
  Contoh: Pantau kesalahan dengan Sentry
</h3>

```bash theme={null}
claude mcp add --transport http sentry https://mcp.sentry.dev/mcp
```

Autentikasi dengan akun Sentry Anda:

```text theme={null}
/mcp
```

Kemudian debug masalah produksi:

```text theme={null}
What are the most common errors in the last 24 hours?
```

```text theme={null}
Show me the stack trace for error ID abc123
```

```text theme={null}
Which deployment introduced these new errors?
```

<h3 id="example-connect-to-github-for-code-reviews">
  Contoh: Hubungkan ke GitHub untuk tinjauan kode
</h3>

Server MCP jarak jauh GitHub diautentikasi dengan token akses pribadi GitHub yang diteruskan sebagai header. Untuk mendapatkan satu, buka [pengaturan token GitHub Anda](https://github.com/settings/personal-access-tokens), hasilkan token baru yang bersifat fine-grained dengan akses ke repositori yang ingin Claude kerjakan, kemudian tambahkan server:

```bash theme={null}
claude mcp add --transport http github https://api.githubcopilot.com/mcp/ \
  --header "Authorization: Bearer YOUR_GITHUB_PAT"
```

Kemudian bekerja dengan GitHub:

```text theme={null}
Review PR #456 and suggest improvements
```

```text theme={null}
Create a new issue for the bug we just found
```

```text theme={null}
Show me all open PRs assigned to me
```

<h3 id="example-query-your-postgresql-database">
  Contoh: Tanyakan database PostgreSQL Anda
</h3>

```bash theme={null}
claude mcp add --transport stdio db -- npx -y @bytebase/dbhub \
  --dsn "postgresql://readonly:pass@prod.db.com:5432/analytics"
```

Kemudian tanyakan database Anda secara alami:

```text theme={null}
What's our total revenue this month?
```

```text theme={null}
Show me the schema for the orders table
```

```text theme={null}
Find customers who haven't made a purchase in 90 days
```

<h2 id="authenticate-with-remote-mcp-servers">
  Autentikasi dengan server MCP jarak jauh
</h2>

Banyak server MCP berbasis cloud memerlukan autentikasi. Claude Code mendukung OAuth 2.0 untuk koneksi yang aman.

Claude Code menandai server jarak jauh sebagai memerlukan autentikasi ketika server merespons dengan `401 Unauthorized` atau `403 Forbidden`. Untuk server yang belum Anda masuki, salah satu kode status ini menandainya di `/mcp` sehingga Anda dapat menyelesaikan alur OAuth.

Ketika permintaan ke server OAuth yang sudah Anda masuki mengembalikan `401 Unauthorized`, Claude Code menyegarkan token yang disimpan, terhubung kembali, dan mencoba ulang permintaan sekali. Itu menandai server di `/mcp` hanya jika percobaan ulang itu juga gagal. Sebelum v2.1.206, penyegaran token yang gagal karena alasan sementara, seperti kesalahan jaringan, menandai server OAuth sebagai memerlukan autentikasi untuk sisa sesi meskipun token penyegarannya masih valid.

Mulai dari v2.1.195, ketika penyegaran token gagal karena server menolak token penyegaran yang disimpan, Claude Code segera menampilkan pemberitahuan yang menunjuk ke `/mcp`. Menu server yang terhubung di sana menawarkan Re-authenticate, sehingga Anda dapat masuk lagi sebelum panggilan alat berikutnya gagal.

Server kustom yang mengembalikan header `WWW-Authenticate` yang menunjuk ke server otorisasinya mendapatkan penemuan otomatis yang sama seperti server jarak jauh lainnya.

Mulai dari v2.1.193, Claude Code juga menampilkan pemberitahuan startup ketika satu atau lebih server yang dikonfigurasi memerlukan autentikasi, sehingga Anda tidak perlu membuka `/mcp` untuk menemukan server mana yang memerlukan sign-in.

Dalam mode non-interaktif tidak ada panel `/mcp`, jadi Claude Code tidak dapat menjalankan alur OAuth untuk Anda. Mulai dari v2.1.196, ketika server yang dikonfigurasi memerlukan autentikasi selama `claude -p` atau jalankan Agent SDK dengan [pencarian alat](#scale-with-mcp-tool-search) diaktifkan, yang merupakan default, Claude Code memberi tahu Claude bahwa alat server tidak tersedia sampai Anda mengotorisasinya. Claude kemudian dapat menyebutkan server yang memerlukan sign-in alih-alih merespons seolah-olah server tidak dikonfigurasi. Selesaikan sign-in dari sesi interaktif dengan `/mcp` atau `claude mcp login <name>`.

Jika Anda mengonfigurasi `headers.Authorization` untuk server dan server menolak header tersebut, Claude Code melaporkan koneksi sebagai gagal alih-alih kembali ke OAuth. Periksa bahwa token valid untuk endpoint MCP, atau hapus header untuk menggunakan alur OAuth.

<Steps>
  <Step title="Tambahkan server yang memerlukan autentikasi">
    Sebagai contoh:

    ```bash theme={null}
    claude mcp add --transport http sentry https://mcp.sentry.dev/mcp
    ```
  </Step>

  <Step title="Gunakan perintah /mcp dalam Claude Code">
    Dalam Claude Code, gunakan perintah:

    ```text theme={null}
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

Dari v2.1.186, `claude mcp login <name>` menjalankan alur OAuth server yang dikonfigurasi langsung dari shell Anda, sehingga Anda tidak perlu membuka panel `/mcp` di dalam sesi.

```bash theme={null}
claude mcp login sentry
```

Untuk menghapus kredensial yang disimpan nanti, jalankan `claude mcp logout <name>`.

Mulai dari v2.1.191, perintah mendeteksi ketika tidak ada browser lokal yang tersedia, seperti selama sesi SSH atau di Linux tanpa server tampilan, dan mencetak URL otorisasi alih-alih mencoba membuka browser. Buka URL di mesin lokal Anda, kemudian tempel URL pengalihan lengkap dari bilah alamat browser Anda kembali ke prompt. Perintah memerlukan terminal interaktif untuk langkah paste, jadi hubungkan dengan `ssh -t`. Teruskan `--no-browser` untuk memaksa prompt URL bahkan ketika browser lokal terdeteksi.

```bash theme={null}
claude mcp login sentry --no-browser
```

<h3 id="use-a-fixed-oauth-callback-port">
  Gunakan port callback OAuth tetap
</h3>

Beberapa server MCP memerlukan URI pengalihan tertentu yang terdaftar sebelumnya. Secara default, Claude Code memilih port acak yang tersedia untuk callback OAuth. Gunakan `--callback-port` untuk memperbaiki port sehingga cocok dengan URI pengalihan yang telah terdaftar sebelumnya dalam bentuk `http://localhost:PORT/callback`.

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
  * Jika server menggunakan klien OAuth publik tanpa rahasia, gunakan hanya `--client-id` tanpa `--client-secret`
  * `--callback-port` dapat digunakan dengan atau tanpa `--client-id`
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

Jika server kemudian mengembalikan 403 `insufficient_scope` untuk panggilan alat, Claude Code melakukan autentikasi ulang dengan cakupan yang disematkan yang sama. Perluas `oauth.scopes` ketika alat yang Anda butuhkan memerlukan cakupan di luar set yang disematkan.

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
* Perintah berjalan dalam shell dengan waktu tunggu 10 detik, dari direktori kerja saat ini sesi. Gunakan jalur absolut atau perintah di `PATH` untuk skrip
* Header dinamis mengganti `headers` statis apa pun dengan nama yang sama

Helper berjalan segar pada setiap koneksi, pada startup sesi dan pada reconnect. Tidak ada caching, jadi skrip Anda bertanggung jawab untuk penggunaan kembali token apa pun.

Mulai dari v2.1.193, jika panggilan alat mengembalikan `401 Unauthorized` atau `403 Forbidden`, Claude Code secara otomatis menjalankan kembali helper, terhubung kembali dengan header segar, dan mencoba panggilan sekali lagi. Claude Code menandai server sebagai memerlukan autentikasi di `/mcp` hanya jika percobaan ulang itu juga gagal.

Claude Code menetapkan variabel lingkungan ini saat menjalankan helper:

| Variabel                      | Nilai                                                                                                     |
| :---------------------------- | :-------------------------------------------------------------------------------------------------------- |
| `CLAUDE_CODE_MCP_SERVER_NAME` | nama server MCP                                                                                           |
| `CLAUDE_CODE_MCP_SERVER_URL`  | URL server MCP                                                                                            |
| `CLAUDE_PLUGIN_ROOT`          | direktori root plugin. Diatur hanya ketika [plugin](/docs/id/plugins-reference#mcp-servers) menyediakan server |

Gunakan ini untuk menulis satu skrip helper yang melayani beberapa server MCP.

Untuk server yang disediakan plugin, helper juga berjalan dengan direktori kerjanya diatur ke root plugin, sehingga jalur `headersHelper` relatif diselesaikan di dalam direktori plugin daripada terhadap direktori kerja sesi. Memerlukan Claude Code v2.1.195 atau lebih baru.

Plugin-provided `headersHelper` tidak dapat mereferensikan nilai [`${user_config.*}`](/docs/id/plugins-reference#user-configuration) plugin, karena perintah berjalan melalui shell. Claude Code melaporkan server sebagai salah konfigurasi dengan [kesalahan](/docs/id/errors#plugin-command-references-user-config) dan tidak mengganti nilai. Letakkan `${user_config.KEY}` di bidang `headers` server sebagai gantinya, yang tidak diurai shell, atau buat skrip helper membaca nilai dari lingkungannya sendiri atau file konfigurasi. Sebelum v2.1.207, `headersHelper` mengganti nilai `${user_config.*}`.

<Note>
  `headersHelper` mengeksekusi perintah shell arbitrer. Ketika ditentukan pada cakupan proyek atau lokal, itu hanya berjalan setelah Anda menerima dialog kepercayaan ruang kerja.
</Note>

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
    Tambahkan server di [claude.ai/customize/connectors](https://claude.ai/customize/connectors). Pada paket Tim dan Enterprise, hanya admin yang dapat menambahkan server.
  </Step>

  <Step title="Autentikasi server MCP">
    Selesaikan langkah autentikasi yang diperlukan di claude.ai.
  </Step>

  <Step title="Lihat dan kelola server di Claude Code">
    Di Claude Code, gunakan perintah:

    ```text theme={null}
    /mcp
    ```

    Server dari claude.ai muncul dalam daftar dengan indikator yang menunjukkan mereka berasal dari claude.ai.
  </Step>
</Steps>

Mulai dari v2.1.161, konektor yang belum pernah Anda masuki disembunyikan di balik baris `Show unused connectors` di akhir bagian claude.ai, sehingga daftar yang disediakan organisasi tidak mengisi panel. Pilih baris untuk memperluas mereka. Konektor yang Anda masuki sebelumnya tetap terlihat bahkan ketika saat ini memerlukan autentikasi ulang.

Konektor dari claude.ai diambil hanya ketika [metode autentikasi](/docs/id/authentication#authentication-precedence) aktif Anda adalah langganan claude.ai Anda. Mereka tidak dimuat ketika `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN`, `apiKeyHelper`, atau penyedia pihak ketiga seperti Amazon Bedrock atau Google Cloud's Agent Platform aktif, bahkan jika Anda sebelumnya menjalankan `/login`.

Jika `/mcp` tidak mencantumkan konektor yang Anda tambahkan, jalankan `/status` untuk mengonfirmasi metode autentikasi mana yang aktif, hapus variabel lingkungan tersebut atau hapus pengaturan `apiKeyHelper`, kemudian jalankan `/login` untuk memilih akun claude.ai Anda.

Server yang telah Anda tambahkan di Claude Code mengambil [prioritas](#scope-hierarchy-and-precedence) atas konektor claude.ai yang menunjuk ke URL yang sama. Ketika ini terjadi, `/mcp` mencantumkan konektor sebagai tersembunyi dan menunjukkan cara menghapus duplikat jika Anda lebih suka menggunakan konektor.

Beberapa konektor yang dihosting Anthropic, seperti Microsoft 365, Gmail, dan Google Calendar, tidak mendukung OAuth lokal dari Claude Code karena penyedia identitas hulu hanya menerima URL pengalihan yang didaftarkan claude.ai. Mulai dari v2.1.162, mengautentikasi salah satu host ini di `/mcp` menampilkan pesan yang mengarahkan Anda untuk menghubungkannya di Settings → Connectors di claude.ai. Setelah terhubung di sana, konektor muncul di Claude Code secara otomatis.

<h3 id="organization-controls-on-connector-tools">
  Kontrol organisasi pada alat connector
</h3>

Organisasi Anda dapat mengatur kontrol per-alat pada [connector claude.ai](https://claude.com/docs/connectors). Claude Code membaca pengaturan ini saat startup dan memberlakukannya secara lokal. Jalankan `/mcp` untuk melihat pengaturan mana yang berlaku untuk setiap alat pada connector.

* **Alat diatur ke `ask`**: Claude Code meminta pada setiap panggilan dengan alasan `Your organization requires approval for this tool`. Prompt muncul bahkan dalam [mode izin](/docs/id/permissions#permission-modes) `acceptEdits`, `auto`, dan `bypassPermissions`, dan tidak pernah menawarkan opsi untuk mengingat pilihan Anda. [Aturan Allow](/docs/id/permissions) yang cocok dengan alat tidak melewati prompt juga. Dalam mode `dontAsk`, yang tidak pernah meminta, Claude Code menolak panggilan sebagai gantinya.
* **Alat diatur ke `blocked`**: Claude Code memfilter alat sebelum Claude melihatnya, sehingga tidak pernah muncul dalam daftar alat.

Memberlakukan kontrol ini memerlukan Claude Code v2.1.129 atau lebih baru. Versi sebelumnya mengabaikan pengaturan dan menerapkan alur izin standar.

<h3 id="disable-claude-ai-connectors">
  Nonaktifkan konektor claude.ai
</h3>

Untuk menonaktifkan server MCP claude.ai di Claude Code, atur [`disableClaudeAiConnectors`](/docs/id/settings#available-settings) ke `true` dalam cakupan pengaturan apa pun:

```json theme={null}
{
  "disableClaudeAiConnectors": true
}
```

Pengaturan ini menggunakan semantik any-source-true: `true` dalam sumber pengaturan apa pun mengambil prioritas. File `.claude/settings.json` proyek yang diperiksa dapat mengeluarkan repositori dari konektor cloud, tetapi `false` tingkat proyek tidak dapat mengaktifkan kembali konektor yang telah dinonaktifkan oleh `true` tingkat pengguna atau kebijakan. Server yang dilewatkan secara eksplisit melalui `--mcp-config` tidak terpengaruh.

Anda juga dapat mengatur variabel lingkungan `ENABLE_CLAUDEAI_MCP_SERVERS` ke `false`, yang memiliki efek yang sama untuk sesi shell saat ini:

```bash theme={null}
ENABLE_CLAUDEAI_MCP_SERVERS=false claude
```

Untuk memblokir konektor claude.ai individual alih-alih semuanya, tambahkan mereka ke [`deniedMcpServers`](/docs/id/managed-mcp) berdasarkan nama atau pola URL. Misalnya, entri `serverName` dari `"claude.ai Slack"` memblokir konektor Slack. Untuk mengalihkan konektor aktif atau nonaktif hanya untuk proyek saat ini, gunakan panel `/mcp`.

<Note>
  Pengaturan sisi klien ini mengatur sesi Claude Code lokal. Dalam sesi [Claude Code di web](/docs/id/claude-code-on-the-web), konektor claude.ai disediakan oleh host jarak jauh dan tiba sebagai entri `--mcp-config` eksplisit, jadi `disableClaudeAiConnectors` tidak berlaku di sana. URL konektor juga ditulis ulang melalui proxy sesi, jadi pola `serverUrl` `deniedMcpServers` yang menargetkan URL vendor tidak akan cocok. Kelola konektor mana yang dapat digunakan sesi cloud dari pengaturan organisasi claude.ai Anda.
</Note>

<h2 id="use-claude-code-as-an-mcp-server">
  Gunakan Claude Code sebagai server MCP
</h2>

Anda dapat menggunakan Claude Code itu sendiri sebagai server MCP yang dapat terhubung oleh aplikasi lain:

```bash theme={null}
# Mulai Claude sebagai server MCP stdio
claude mcp serve
```

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
  **Mengonfigurasi jalur executable**: Field `command` harus mereferensikan executable Claude Code. Jika perintah `claude` tidak ada di PATH sistem Anda, Anda perlu menentukan jalur lengkap ke executable.

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

  * Server menyediakan akses ke alat Claude seperti View, Edit, LS, dll.
  * Di Claude Desktop, coba minta Claude untuk membaca file di direktori, membuat edit, dan lainnya.
  * Server MCP ini hanya mengekspos alat Claude Code ke klien MCP Anda, jadi klien Anda sendiri bertanggung jawab untuk menerapkan konfirmasi pengguna untuk panggilan alat individual.
</Tip>

<h2 id="mcp-output-limits-and-warnings">
  Batas output MCP dan peringatan
</h2>

Ketika alat MCP menghasilkan output besar, Claude Code membantu mengelola penggunaan token untuk mencegah membanjiri konteks percakapan Anda:

* **Ambang batas peringatan output**: Claude Code menampilkan peringatan ketika output alat MCP apa pun melebihi 10.000 token
* **Batas yang dapat dikonfigurasi**: Anda dapat menyesuaikan token output MCP maksimum yang diizinkan menggunakan variabel lingkungan `MAX_MCP_OUTPUT_TOKENS`
* **Batas default**: Maksimum default adalah 25.000 token
* **Cakupan**: Variabel lingkungan berlaku untuk alat yang tidak mendeklarasikan batas mereka sendiri. Alat yang menetapkan [`anthropic/maxResultSizeChars`](#raise-the-limit-for-a-specific-tool) menggunakan nilai itu sebagai gantinya untuk konten teks, terlepas dari apa yang `MAX_MCP_OUTPUT_TOKENS` diatur. Alat yang mengembalikan data gambar masih tunduk pada `MAX_MCP_OUTPUT_TOKENS`

Untuk meningkatkan batas untuk alat yang menghasilkan output besar:

```bash theme={null}
export MAX_MCP_OUTPUT_TOKENS=50000
claude
```

Ini sangat berguna saat bekerja dengan server MCP yang:

* Menanyakan dataset atau database besar
* Menghasilkan laporan atau dokumentasi terperinci
* Memproses file log atau informasi debugging yang luas

<h3 id="raise-the-limit-for-a-specific-tool">
  Naikkan batas untuk alat tertentu
</h3>

Jika Anda membangun server MCP, Anda dapat mengizinkan alat individual untuk mengembalikan hasil yang lebih besar dari ambang batas default dengan menetapkan `_meta["anthropic/maxResultSizeChars"]` dalam entri `tools/list` alat. Claude Code menaikkan ambang batas alat tersebut ke nilai yang dianotasi, hingga batas keras 500.000 karakter.

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

Anotasi berlaku secara independen dari `MAX_MCP_OUTPUT_TOKENS` untuk konten teks, jadi pengguna tidak perlu menaikkan variabel lingkungan untuk alat yang mendeklarasikannya. Alat yang mengembalikan data gambar masih tunduk pada batas token.

<Warning>
  Jika Anda sering mengalami peringatan output dengan server MCP tertentu yang tidak Anda kontrol, pertimbangkan untuk meningkatkan batas `MAX_MCP_OUTPUT_TOKENS`. Anda juga dapat meminta penulis server untuk menambahkan anotasi `anthropic/maxResultSizeChars` atau untuk membuat halaman respons mereka. Anotasi tidak berpengaruh pada alat yang mengembalikan konten gambar; untuk itu, menaikkan `MAX_MCP_OUTPUT_TOKENS` adalah satu-satunya opsi.
</Warning>

<h2 id="tool-input-schemas-with-a-root-level-combinator">
  Skema input alat dengan combinator tingkat root
</h2>

Beberapa server MCP mendeklarasikan skema input alat sebagai union JSON Schema, dengan `anyOf`, `oneOf`, atau `allOf` di tingkat atas skema. Claude API tidak menerima kata kunci tersebut di root skema. Itu menerima combinator yang bersarang di dalam `properties`, yang Claude Code kirim tanpa perubahan.

Mulai dari Claude Code v2.1.195, alat dengan combinator tingkat root tetap tersedia. Sebelum mengirim alat ke API, Claude Code meratakan skema menjadi satu objek dan menambahkan kalimat ke deskripsi alat yang memberi tahu Claude grup parameter mana yang termasuk bersama:

* `allOf`: properti dari setiap cabang digabungkan, dan daftar `required` setiap cabang masih berlaku
* `anyOf` dan `oneOf`: properti dari setiap cabang digabungkan, dan daftar `required` setiap cabang dijelaskan dalam deskripsi alat alih-alih ditegakkan oleh skema

Server Anda menerima argumen apa pun yang dipilih Claude, jadi terus validasi kombinasi sisi server.

Ketika Claude Code tidak dapat menghasilkan skema yang diterima API, atau pada deployment yang tidak menerima konfigurasi jarak jauh yang mengaktifkan penulisan ulang, seperti mesin offline, itu melewati alat itu, mencatat alasan dalam log server, dan meninggalkan alat server lainnya tersedia. Versi lebih awal dari v2.1.195 melewati setiap alat yang skema input-nya memiliki `anyOf`, `oneOf`, atau `allOf` tingkat root.

<h2 id="require-approval-for-a-specific-tool">
  Memerlukan persetujuan untuk alat tertentu
</h2>

Jika Anda membangun server MCP, Anda dapat menandai alat sebagai memerlukan persetujuan eksplisit pada setiap panggilan dengan menetapkan `_meta["anthropic/requiresUserInteraction"]` ke `true` dalam entri `tools/list` alat. Nilainya harus boolean JSON `true`; nilai lain diabaikan.

Claude Code menampilkan prompt izin alat itu pada setiap panggilan, bahkan dalam mode izin `acceptEdits`, `auto`, dan `bypassPermissions` [permission modes](/docs/id/permissions#permission-modes), dan tidak menawarkan opsi "jangan tanya lagi" untuk itu. [Aturan izin](/docs/id/permissions#permission-rule-syntax) yang cocok dengan alat tidak melewati prompt baik. Dalam mode `dontAsk`, yang tidak pernah meminta, Claude Code menolak panggilan sebagai gantinya.

Prompt harus mencapai seseorang. Dalam mode non-interaktif dengan [`--permission-prompt-tool`](/docs/id/cli-reference#cli-flags), hasil `allow` dari alat prompt untuk alat yang ditandai dikonversi menjadi deny dengan pesan `MCP tool requires user interaction; not supported via --permission-prompt-tool`. Callback [`canUseTool`](/docs/id/agent-sdk/permissions) Agent SDK menerima panggilan ini dan dapat menyetujuinya, karena host SDK diharapkan menunjukkannya kepada pengguna.

Gunakan ini untuk alat yang prompt izinnya adalah poin itu sendiri, seperti langkah persetujuan atau pemberian akses di mana persetujuan otomatis berarti tidak ada manusia yang pernah setuju. Alat lain dari server yang sama menjaga perilaku izin normal mereka.

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

Ketika sesi terhubung ke [Remote Control](/docs/id/remote-control) atau host SDK, Claude Code menandai permintaan izin sebagai memerlukan interaksi pengguna, sehingga klien menampilkan prompt izin alat untuk Anda jawab alih-alih tindakan persetujuan satu ketukan.

<h2 id="respond-to-mcp-elicitation-requests">
  Tanggapi permintaan elicitasi MCP
</h2>

Server MCP dapat meminta input terstruktur dari Anda di tengah tugas menggunakan elicitasi. Ketika server memerlukan informasi yang tidak dapat diperolehnya sendiri, Claude Code menampilkan dialog interaktif dan meneruskan respons Anda kembali ke server. Tidak ada konfigurasi yang diperlukan di pihak Anda: dialog elicitasi muncul secara otomatis ketika server memintanya.

Server dapat meminta input dengan dua cara:

* **Mode formulir**: Claude Code menampilkan dialog dengan field formulir yang ditentukan oleh server (misalnya, prompt nama pengguna dan kata sandi). Isi field dan kirim.
* **Mode URL**: Claude Code membuka URL browser untuk autentikasi atau persetujuan. Selesaikan alur di browser, kemudian konfirmasi di CLI.

Untuk merespons otomatis permintaan elicitasi tanpa menampilkan dialog, gunakan [hook `Elicitation`](/docs/id/hooks#elicitation).

Jika Anda membangun server MCP yang menggunakan elicitasi, lihat [spesifikasi elicitasi MCP](https://modelcontextprotocol.io/docs/learn/client-concepts#elicitation) untuk detail protokol dan contoh skema.

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

    ```text theme={null}
    Can you analyze @github:issue://123 and suggest a fix?
    ```

    ```text theme={null}
    Please review the API documentation at @docs:file://api/authentication
    ```
  </Step>

  <Step title="Referensi sumber daya berganda">
    Anda dapat mereferensikan beberapa sumber daya dalam satu prompt:

    ```text theme={null}
    Compare @postgres:schema://users with @docs:file://database/user-model
    ```
  </Step>
</Steps>

<Tip>
  Tips:

  * Sumber daya secara otomatis diambil dan disertakan sebagai lampiran saat direferensikan
  * Jalur sumber daya dapat dicari fuzzy dalam pelengkapan otomatis penyebutan @
  * Claude Code secara otomatis menyediakan alat untuk membuat daftar dan membaca sumber daya MCP ketika server mendukungnya
  * Sumber daya dapat berisi jenis konten apa pun yang disediakan server MCP (teks, JSON, data terstruktur, dll.)
</Tip>

<h2 id="scale-with-mcp-tool-search">
  Skala dengan Pencarian Alat MCP
</h2>

Pencarian Alat menjaga penggunaan konteks MCP tetap rendah dengan menunda definisi alat sampai Claude membutuhkannya. Hanya nama alat dan instruksi server yang dimuat saat startup sesi, jadi menambahkan lebih banyak server MCP memiliki dampak minimal pada jendela konteks Anda. Claude Code tidak memberlakukan batas alat tetap per-server; batas praktisnya adalah anggaran jendela konteks Anda.

<h3 id="how-it-works">
  Cara kerjanya
</h3>

Pencarian Alat diaktifkan secara default. Alat MCP ditangguhkan daripada dimuat ke konteks sebelumnya, dan Claude menggunakan alat pencarian untuk menemukan yang relevan saat tugas membutuhkannya. Hanya alat yang benar-benar digunakan Claude yang memasuki konteks. Dari perspektif Anda, alat MCP bekerja persis seperti sebelumnya.

Jika Anda lebih suka pemuatan berbasis ambang batas, atur `ENABLE_TOOL_SEARCH=auto` untuk memuat skema sebelumnya ketika mereka cocok dalam 10% jendela konteks dan hanya menunda overflow. Lihat [Konfigurasi pencarian alat](#configure-tool-search) untuk semua opsi.

<h3 id="for-mcp-server-authors">
  Untuk penulis server MCP
</h3>

Jika Anda membangun server MCP, field instruksi server menjadi lebih berguna dengan Pencarian Alat yang diaktifkan. Instruksi server membantu Claude memahami kapan harus mencari alat Anda, mirip dengan cara [skills](/docs/id/skills) bekerja.

Tambahkan instruksi server yang jelas dan deskriptif yang menjelaskan:

* Kategori tugas apa yang ditangani alat Anda
* Kapan Claude harus mencari alat Anda
* Kemampuan utama yang disediakan server Anda

Claude Code memotong deskripsi alat dan instruksi server pada 2KB masing-masing. Jaga agar ringkas untuk menghindari pemotongan, dan letakkan detail penting di dekat awal.

<h3 id="configure-tool-search">
  Konfigurasi pencarian alat
</h3>

Pencarian alat diaktifkan secara default: alat MCP ditangguhkan dan ditemukan sesuai permintaan. Claude Code menonaktifkannya secara default di Platform Agent Google Cloud. Pencarian alat juga dinonaktifkan ketika `ANTHROPIC_BASE_URL` menunjuk ke host non-pihak pertama, karena sebagian besar proxy tidak meneruskan blok `tool_reference`. Atur `ENABLE_TOOL_SEARCH` secara eksplisit untuk mengganti fallback mana pun.

Setting [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS`](/docs/id/env-vars) menjaga pencarian alat tetap mati, dan `ENABLE_TOOL_SEARCH` tidak dapat menggantikannya. Variabel ini menghapus header beta yang diperlukan oleh definisi alat `defer_loading` dan blok konten `tool_reference`.

Pencarian alat memerlukan model yang mendukung blok `tool_reference`: Claude Sonnet 4.5, Claude Haiku 4.5, Claude Opus 4.5, dan model yang lebih baru. Lihat [kompatibilitas model dalam dokumentasi API](https://platform.claude.com/docs/en/agents-and-tools/tool-use/tool-search-tool#model-compatibility) untuk daftar terkini. Di Platform Agent Google Cloud, pencarian alat didukung untuk Claude Sonnet 4.5 dan lebih baru serta Claude Opus 4.5 dan lebih baru.

Kontrol perilaku pencarian alat dengan variabel lingkungan `ENABLE_TOOL_SEARCH`:

| Nilai          | Perilaku                                                                                                                                                                                                                                                                            |
| :------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| (tidak diatur) | Semua alat MCP ditangguhkan dan dimuat sesuai permintaan. Kembali ke pemuatan sebelumnya di Platform Agent Google Cloud atau ketika `ANTHROPIC_BASE_URL` adalah host non-pihak pertama                                                                                              |
| `true`         | Semua alat MCP ditangguhkan. Claude Code mengirim header beta bahkan di Platform Agent Google Cloud dan melalui proxy. Permintaan gagal pada model Platform Agent Google Cloud lebih awal dari Sonnet 4.5 atau Opus 4.5, atau pada proxy yang tidak mendukung blok `tool_reference` |
| `auto`         | Mode ambang batas: alat dimuat sebelumnya jika cocok dalam 10% jendela konteks, ditangguhkan sebaliknya                                                                                                                                                                             |
| `auto:N`       | Mode ambang batas dengan persentase khusus, di mana `N` adalah 0-100. Misalnya, `auto:5` untuk 5%                                                                                                                                                                                   |
| `false`        | Semua alat MCP dimuat sebelumnya, tidak ada penundaan                                                                                                                                                                                                                               |

```bash theme={null}
# Gunakan ambang batas khusus 5%
ENABLE_TOOL_SEARCH=auto:5 claude

# Nonaktifkan pencarian alat sepenuhnya
ENABLE_TOOL_SEARCH=false claude
```

Atau atur nilai dalam [field `env` settings.json](/docs/id/settings#available-settings) Anda.

Anda juga dapat menonaktifkan alat `ToolSearch` secara khusus:

```json theme={null}
{
  "permissions": {
    "deny": ["ToolSearch"]
  }
}
```

<h3 id="exempt-a-server-from-deferral">
  Keluarkan server dari penundaan
</h3>

Jika alat server harus selalu terlihat oleh Claude tanpa langkah pencarian, atur `alwaysLoad` ke `true` dalam konfigurasi server tersebut. Setiap alat dari server tersebut kemudian dimuat ke konteks saat startup sesi terlepas dari pengaturan `ENABLE_TOOL_SEARCH`. Gunakan ini untuk sejumlah kecil alat yang Claude butuhkan di setiap putaran, karena setiap alat sebelumnya mengonsumsi konteks yang sebaliknya akan tersedia untuk percakapan Anda.

Entri `.mcp.json` berikut mengecualikan satu server HTTP sambil membiarkan server lain ditangguhkan:

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

Field `alwaysLoad` tersedia di semua jenis server dan memerlukan Claude Code v2.1.121 atau lebih baru. Server MCP juga dapat menandai alat individual sebagai selalu-dimuat dengan menyertakan `"anthropic/alwaysLoad": true` dalam objek `_meta` alat, yang memiliki efek yang sama hanya untuk alat tersebut.

Pengaturan `alwaysLoad: true` juga memblokir startup sampai server terhubung, dibatasi pada timeout koneksi standar 5 detik. Ini berlaku bahkan ketika MCP startup adalah [non-blocking secara default](/docs/id/env-vars), karena alat harus ada saat prompt pertama dibangun. Server lain terus terhubung di latar belakang.

<h2 id="use-mcp-prompts-as-commands">
  Gunakan prompt MCP sebagai perintah
</h2>

Server MCP dapat mengekspos prompt yang menjadi tersedia sebagai perintah di Claude Code.

<h3 id="execute-mcp-prompts">
  Jalankan prompt MCP
</h3>

<Steps>
  <Step title="Temukan prompt yang tersedia">
    Ketik `/` untuk melihat semua perintah yang tersedia, termasuk yang dari server MCP. Prompt MCP muncul dengan format `/mcp__servername__promptname`.
  </Step>

  <Step title="Jalankan prompt tanpa argumen">
    ```text theme={null}
    /mcp__github__list_prs
    ```
  </Step>

  <Step title="Jalankan prompt dengan argumen">
    Banyak prompt menerima argumen. Teruskan mereka dipisahkan spasi setelah perintah:

    ```text theme={null}
    /mcp__github__pr_review 456
    ```

    ```text theme={null}
    /mcp__jira__create_issue "Bug in login flow" high
    ```
  </Step>
</Steps>

<Tip>
  Tips:

  * Prompt MCP ditemukan secara dinamis dari server yang terhubung
  * Argumen diurai berdasarkan parameter yang ditentukan prompt
  * Hasil prompt disuntikkan langsung ke dalam percakapan
  * Nama server dan prompt dinormalisasi, dengan spasi dikonversi menjadi garis bawah
</Tip>

<h2 id="managed-mcp-configuration">
  Konfigurasi MCP yang dikelola
</h2>

Untuk organisasi yang memerlukan kontrol terpusat atas server MCP mana yang dapat dihubungkan pengguna, lihat [Konfigurasi MCP yang dikelola](/docs/id/managed-mcp). Ini mencakup penerapan set server tetap dengan `managed-mcp.json`, pembatasan server dengan `allowedMcpServers` dan `deniedMcpServers`, dan apa yang dilihat pengguna ketika server diblokir.
