> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Dorong acara ke dalam sesi yang sedang berjalan dengan channels

> Gunakan channels untuk mendorong pesan, peringatan, dan webhooks ke dalam sesi Claude Code Anda dari server MCP. Teruskan hasil CI, pesan obrolan, dan acara pemantauan sehingga Claude dapat bereaksi saat Anda tidak ada.

<Note>
  Channels berada dalam [research preview](#research-preview). Mereka memerlukan autentikasi Anthropic melalui claude.ai atau kunci API Console, dan tidak tersedia di Amazon Bedrock, Google Cloud's Agent Platform, atau Microsoft Foundry. Organisasi Tim dan Enterprise harus [secara eksplisit mengaktifkannya](#enterprise-controls).
</Note>

Sebuah channel adalah server MCP yang mendorong acara ke dalam sesi Claude Code yang sedang berjalan, sehingga Claude dapat bereaksi terhadap hal-hal yang terjadi saat Anda tidak berada di terminal. Channels dapat dua arah: Claude membaca acara dan membalas kembali melalui channel yang sama, seperti jembatan obrolan. Acara hanya tiba saat sesi terbuka, jadi untuk pengaturan yang selalu aktif Anda menjalankan Claude dalam proses latar belakang atau terminal persisten.

Tidak seperti integrasi yang menjalankan sesi cloud baru atau menunggu untuk diambil, acara tiba dalam sesi yang sudah Anda buka: lihat [bagaimana channels dibandingkan](#how-channels-compare).

Anda menginstal channel sebagai plugin dan mengonfigurasinya dengan kredensial Anda sendiri. Telegram, Discord, dan iMessage disertakan dalam research preview.

Ketika Claude membalas melalui channel, Anda melihat pesan masuk di terminal Anda tetapi bukan teks balasan. Terminal menampilkan panggilan tool dan konfirmasi (seperti "terkirim"), dan balasan sebenarnya muncul di platform lain.

Jika Anda mengelola organisasi Tim, Enterprise, atau Console, lihat [Aktifkan channels untuk organisasi Anda](#enterprise-controls). Untuk membangun channel Anda sendiri, lihat [referensi Channels](/docs/id/channels-reference).

<h2 id="supported-channels">
  Saluran yang Didukung
</h2>

Setiap saluran yang didukung adalah plugin yang memerlukan [Bun](https://bun.sh). Untuk demo langsung dari alur plugin sebelum menghubungkan platform nyata, coba [quickstart fakechat](#quickstart).

<Tabs>
  <Tab title="Telegram">
    Lihat [sumber plugin Telegram](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins/telegram) lengkap.

    <Steps>
      <Step title="Buat bot Telegram">
        Buka [BotFather](https://t.me/BotFather) di Telegram dan kirim `/newbot`. Berikan nama tampilan dan nama pengguna unik yang diakhiri dengan `bot`. Salin token yang dikembalikan BotFather.
      </Step>

      <Step title="Instal plugin">
        Di Claude Code, jalankan:

        ```
        /plugin install telegram@claude-plugins-official
        ```

        Jika Claude Code melaporkan bahwa plugin tidak ditemukan di pasar mana pun, pasar Anda hilang atau ketinggalan zaman. Jalankan `/plugin marketplace update claude-plugins-official` untuk menyegarkannya, atau `/plugin marketplace add anthropics/claude-plugins-official` jika Anda belum menambahkannya sebelumnya. Kemudian coba instal lagi.

        Setelah menginstal, jalankan `/reload-plugins` untuk mengaktifkan perintah konfigurasi plugin.
      </Step>

      <Step title="Konfigurasikan token Anda">
        Jalankan perintah konfigurasi dengan token dari BotFather:

        ```
        /telegram:configure <token>
        ```

        Ini menyimpannya ke `~/.claude/channels/telegram/.env`. Anda juga dapat mengatur `TELEGRAM_BOT_TOKEN` di lingkungan shell Anda sebelum meluncurkan Claude Code.
      </Step>

      <Step title="Mulai ulang dengan channels diaktifkan">
        Keluar dari Claude Code dan mulai ulang dengan bendera channel. Ini memulai plugin Telegram, yang mulai polling untuk pesan dari bot Anda:

        ```bash theme={null}
        claude --channels plugin:telegram@claude-plugins-official
        ```
      </Step>

      <Step title="Pasangkan akun Anda">
        Buka Telegram dan kirim pesan apa pun ke bot Anda. Bot membalas dengan kode pemasangan.

        <Note>Jika bot Anda tidak merespons, pastikan Claude Code berjalan dengan `--channels` dari langkah sebelumnya. Bot hanya dapat membalas saat channel aktif.</Note>

        Kembali di Claude Code, jalankan:

        ```
        /telegram:access pair <code>
        ```

        Kemudian kunci akses sehingga hanya akun Anda yang dapat mengirim pesan:

        ```
        /telegram:access policy allowlist
        ```
      </Step>
    </Steps>
  </Tab>

  <Tab title="Discord">
    Lihat [sumber plugin Discord](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins/discord) lengkap.

    <Steps>
      <Step title="Buat bot Discord">
        Buka [Portal Pengembang Discord](https://discord.com/developers/applications), klik **Aplikasi Baru**, dan beri nama. Di bagian **Bot**, buat nama pengguna, kemudian klik **Reset Token** dan salin token.
      </Step>

      <Step title="Aktifkan Message Content Intent">
        Di pengaturan bot Anda, gulir ke **Privileged Gateway Intents** dan aktifkan **Message Content Intent**.
      </Step>

      <Step title="Undang bot ke server Anda">
        Buka **OAuth2 > URL Generator**. Pilih cakupan `bot` dan aktifkan izin ini:

        * View Channels
        * Send Messages
        * Send Messages in Threads
        * Read Message History
        * Attach Files
        * Add Reactions

        Buka URL yang dihasilkan untuk menambahkan bot ke server Anda.
      </Step>

      <Step title="Instal plugin">
        Di Claude Code, jalankan:

        ```
        /plugin install discord@claude-plugins-official
        ```

        Jika Claude Code melaporkan bahwa plugin tidak ditemukan di pasar mana pun, pasar Anda hilang atau ketinggalan zaman. Jalankan `/plugin marketplace update claude-plugins-official` untuk menyegarkannya, atau `/plugin marketplace add anthropics/claude-plugins-official` jika Anda belum menambahkannya sebelumnya. Kemudian coba instal lagi.

        Setelah menginstal, jalankan `/reload-plugins` untuk mengaktifkan perintah konfigurasi plugin.
      </Step>

      <Step title="Konfigurasikan token Anda">
        Jalankan perintah konfigurasi dengan token bot yang Anda salin:

        ```
        /discord:configure <token>
        ```

        Ini menyimpannya ke `~/.claude/channels/discord/.env`. Anda juga dapat mengatur `DISCORD_BOT_TOKEN` di lingkungan shell Anda sebelum meluncurkan Claude Code.
      </Step>

      <Step title="Mulai ulang dengan channels diaktifkan">
        Keluar dari Claude Code dan mulai ulang dengan bendera channel. Ini menghubungkan plugin Discord sehingga bot Anda dapat menerima dan merespons pesan:

        ```bash theme={null}
        claude --channels plugin:discord@claude-plugins-official
        ```
      </Step>

      <Step title="Pasangkan akun Anda">
        DM bot Anda di Discord. Bot membalas dengan kode pemasangan.

        <Note>Jika bot Anda tidak merespons, pastikan Claude Code berjalan dengan `--channels` dari langkah sebelumnya. Bot hanya dapat membalas saat channel aktif.</Note>

        Kembali di Claude Code, jalankan:

        ```
        /discord:access pair <code>
        ```

        Kemudian kunci akses sehingga hanya akun Anda yang dapat mengirim pesan:

        ```
        /discord:access policy allowlist
        ```
      </Step>
    </Steps>
  </Tab>

  <Tab title="iMessage">
    Lihat [sumber plugin iMessage](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins/imessage) lengkap.

    Saluran iMessage membaca database Pesan Anda secara langsung dan mengirim balasan melalui AppleScript. Ini memerlukan macOS dan tidak memerlukan token bot atau layanan eksternal.

    <Steps>
      <Step title="Berikan Akses Disk Penuh">
        Database Pesan di `~/Library/Messages/chat.db` dilindungi oleh macOS. Pertama kali server membacanya, macOS meminta akses: klik **Izinkan**. Prompt menamai aplikasi apa pun yang meluncurkan Bun, seperti Terminal, iTerm, atau IDE Anda.

        Jika prompt tidak muncul atau Anda mengklik Jangan Izinkan, berikan akses secara manual di bawah **Pengaturan Sistem > Privasi & Keamanan > Akses Disk Penuh** dan tambahkan terminal Anda. Tanpa ini, server keluar segera dengan `authorization denied`.
      </Step>

      <Step title="Instal plugin">
        Di Claude Code, jalankan:

        ```
        /plugin install imessage@claude-plugins-official
        ```

        Jika Claude Code melaporkan bahwa plugin tidak ditemukan di pasar mana pun, pasar Anda hilang atau ketinggalan zaman. Jalankan `/plugin marketplace update claude-plugins-official` untuk menyegarkannya, atau `/plugin marketplace add anthropics/claude-plugins-official` jika Anda belum menambahkannya sebelumnya. Kemudian coba instal lagi.
      </Step>

      <Step title="Mulai ulang dengan channels diaktifkan">
        Keluar dari Claude Code dan mulai ulang dengan bendera channel:

        ```bash theme={null}
        claude --channels plugin:imessage@claude-plugins-official
        ```
      </Step>

      <Step title="Kirim pesan ke diri sendiri">
        Buka Pesan di perangkat apa pun yang masuk ke Apple ID Anda dan kirim pesan ke diri sendiri. Ini mencapai Claude segera: obrolan mandiri melewati kontrol akses tanpa pengaturan.

        <Note>Balasan pertama yang Claude kirim memicu prompt Otomasi macOS yang menanyakan apakah terminal Anda dapat mengontrol Pesan. Klik **OK**.</Note>
      </Step>

      <Step title="Izinkan pengirim lain">
        Secara default, hanya pesan Anda sendiri yang melewati. Untuk membiarkan kontak lain mencapai Claude, tambahkan handle mereka:

        ```
        /imessage:access allow +15551234567
        ```

        Handle adalah nomor telepon dalam format `+country` atau email Apple ID seperti `user@example.com`.
      </Step>
    </Steps>
  </Tab>
</Tabs>

Anda juga dapat [membangun saluran Anda sendiri](/docs/id/channels-reference) untuk sistem yang belum memiliki plugin.

<h2 id="quickstart">
  Quickstart
</h2>

Fakechat adalah channel demo yang didukung secara resmi yang menjalankan UI obrolan di localhost, tanpa apa pun untuk diautentikasi dan tidak ada layanan eksternal untuk dikonfigurasi.

Setelah Anda menginstal dan mengaktifkan fakechat, Anda dapat mengetik di browser dan pesan tiba di sesi Claude Code Anda. Claude membalas, dan balasan muncul kembali di browser. Setelah Anda menguji antarmuka fakechat, coba [Telegram](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins/telegram), [Discord](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins/discord), atau [iMessage](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins/imessage).

Untuk mencoba demo fakechat, Anda memerlukan:

* Claude Code [diinstal dan diautentikasi](/docs/id/quickstart#step-1-install-claude-code) dengan akun claude.ai atau kunci API Console Claude
* [Bun](https://bun.sh) diinstal. Plugin channel yang sudah dibangun sebelumnya adalah skrip Bun. Periksa dengan `bun --version`; jika itu gagal, [instal Bun](https://bun.sh/docs/installation).
* **Organisasi Tim, Enterprise, atau Console terkelola**: admin Anda harus [mengaktifkan channels](#enterprise-controls) dalam pengaturan terkelola

<Steps>
  <Step title="Instal plugin channel fakechat">
    Mulai sesi Claude Code dan jalankan perintah instal:

    ```text theme={null}
    /plugin install fakechat@claude-plugins-official
    ```

    Jika Claude Code melaporkan bahwa plugin tidak ditemukan di pasar mana pun, pasar Anda hilang atau ketinggalan zaman. Jalankan `/plugin marketplace update claude-plugins-official` untuk menyegarkannya, atau `/plugin marketplace add anthropics/claude-plugins-official` jika Anda belum menambahkannya sebelumnya. Kemudian coba instal lagi.
  </Step>

  <Step title="Mulai ulang dengan channel diaktifkan">
    Keluar dari Claude Code, kemudian mulai ulang dengan `--channels` dan teruskan plugin fakechat yang Anda instal:

    ```bash theme={null}
    claude --channels plugin:fakechat@claude-plugins-official
    ```

    Server fakechat dimulai secara otomatis.

    <Tip>
      Anda dapat melewatkan beberapa plugin ke `--channels`, dipisahkan dengan spasi.
    </Tip>
  </Step>

  <Step title="Dorong pesan masuk">
    Buka UI fakechat di [http://localhost:8787](http://localhost:8787) dan ketik pesan:

    ```text theme={null}
    hey, what's in my working directory?
    ```

    Pesan tiba di sesi Claude Code Anda sebagai acara `<channel source="fakechat">`. Claude membacanya, melakukan pekerjaan, dan memanggil tool `reply` fakechat. Jawabannya muncul di UI obrolan.
  </Step>
</Steps>

Jika Claude mengalami prompt izin saat Anda jauh dari terminal, sesi dijeda sampai Anda merespons. Server channel yang mendeklarasikan [kemampuan relai izin](/docs/id/channels-reference#relay-permission-prompts) dapat meneruskan prompt ini kepada Anda sehingga Anda dapat menyetujui atau menolak dari jarak jauh. Untuk penggunaan tanpa pengawasan, [`--dangerously-skip-permissions`](/docs/id/permission-modes#skip-all-checks-with-bypasspermissions-mode) melewati sebagian besar prompt, tetapi hanya gunakan di lingkungan yang Anda percayai. Aturan permintaan eksplisit, tool connector [organisasi Anda atur ke `ask`](/docs/id/mcp#organization-controls-on-connector-tools), dan tool MCP yang ditandai [`requiresUserInteraction`](/docs/id/mcp#require-approval-for-a-specific-tool) masih menampilkan prompt.

Ketika Anda menjalankan channels dalam mode non-interaktif dengan `-p`, tool yang memerlukan input terminal, seperti pertanyaan pilihan ganda dan persetujuan plan mode, dinonaktifkan sehingga sesi tidak pernah macet menunggu input.

<h2 id="security">
  Security
</h2>

Setiap plugin channel yang disetujui mempertahankan daftar penyetujuan pengirim: hanya ID yang telah Anda tambahkan yang dapat mendorong pesan, dan semua orang lain diam-diam dijatuhkan.

Telegram dan Discord mem-bootstrap daftar dengan memasangkan:

1. Temukan bot Anda di Telegram atau Discord dan kirim pesan apa pun
2. Bot membalas dengan kode pemasangan
3. Di sesi Claude Code Anda, setujui kode saat diminta
4. ID pengirim Anda ditambahkan ke daftar penyetujuan

iMessage bekerja berbeda: mengirim pesan ke diri sendiri melewati gerbang secara otomatis, dan Anda menambahkan kontak lain berdasarkan handle dengan `/imessage:access allow`.

Di atas itu, Anda mengontrol server mana yang diaktifkan setiap sesi dengan `--channels`, dan organisasi Anda mengontrol ketersediaan dengan [`channelsEnabled`](#enterprise-controls) pada paket Tim dan Enterprise claude.ai dan pada organisasi Console yang menerapkan pengaturan terkelola.

Berada di `.mcp.json` tidak cukup untuk mendorong pesan: server juga harus dinamai dalam `--channels`.

Daftar penyetujuan juga membatasi [relai izin](/docs/id/channels-reference#relay-permission-prompts) jika channel mendeklarasikannya. Siapa pun yang dapat membalas melalui channel dapat menyetujui atau menolak penggunaan tool dalam sesi Anda, jadi hanya daftar penyetujuan pengirim yang Anda percayai dengan otoritas itu.

<h2 id="enterprise-controls">
  Enterprise controls
</h2>

Admin mengontrol ketersediaan melalui dua [pengaturan terkelola](/docs/id/settings) yang tidak dapat ditimpa pengguna. Default tergantung pada cara Anda diautentikasi:

* **claude.ai Tim dan Enterprise**: channels diblokir sampai Owner mengaktifkannya.
* **Anthropic Console dengan autentikasi kunci API**: channels diizinkan secara default. Anda hanya memerlukan pengaturan ini jika organisasi Anda menerapkan pengaturan terkelola.

Dalam semua kasus, tidak ada channel yang berjalan sampai pengguna memilihnya untuk sesi dengan `--channels`.

| Pengaturan              | Tujuan                                                                                                                                                                                                                                                                                | Ketika tidak dikonfigurasi                                                                                                                                                                    |
| :---------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `channelsEnabled`       | Master switch. Harus `true` agar channel apa pun dapat mengirimkan pesan. Atur melalui toggle [konsol Admin claude.ai](https://claude.ai/admin-settings/claude-code) atau langsung dalam pengaturan terkelola. Memblokir semua channels termasuk bendera pengembangan saat dimatikan. | claude.ai Tim dan Enterprise: channels diblokir. Console: channels diizinkan kecuali organisasi Anda menerapkan pengaturan terkelola, dalam hal ini channels diblokir sampai kunci ini diatur |
| `allowedChannelPlugins` | Plugin mana yang dapat mendaftar setelah channels diaktifkan. Menggantikan daftar yang dipertahankan Anthropic saat diatur. Hanya berlaku saat `channelsEnabled` adalah `true`.                                                                                                       | Daftar default Anthropic berlaku                                                                                                                                                              |

Pengguna Pro dan Max tanpa organisasi melewati pemeriksaan ini sepenuhnya: channels tersedia dan pengguna memilih per sesi dengan `--channels`.

<h3 id="enable-channels-for-your-organization">
  Aktifkan channels untuk organisasi Anda
</h3>

Aktifkan channels untuk organisasi Anda dari [**claude.ai → Admin settings → Claude Code → Channels**](https://claude.ai/admin-settings/claude-code), yang memerlukan peran Owner, atau dengan mengatur `channelsEnabled` ke `true` dalam pengaturan terkelola.

Setelah diaktifkan, pengguna di organisasi Anda dapat menggunakan `--channels` untuk memilih server channel ke sesi individual. Jika pengaturan dinonaktifkan atau tidak diatur, server MCP masih terhubung dan alatnya berfungsi, tetapi pesan channel tidak akan tiba. Peringatan startup memberi tahu pengguna untuk meminta admin mengaktifkan pengaturan.

<h3 id="restrict-which-channel-plugins-can-run">
  Batasi plugin channel mana yang dapat berjalan
</h3>

Secara default, plugin apa pun di daftar penyetujuan yang dipertahankan Anthropic dapat mendaftar sebagai channel. Admin pada paket Tim dan Enterprise dapat menggantikan daftar penyetujuan itu dengan milik mereka sendiri dengan mengatur `allowedChannelPlugins` dalam pengaturan terkelola. Gunakan ini untuk membatasi plugin resmi mana yang diizinkan, menyetujui channels dari marketplace internal Anda, atau keduanya. Setiap entri menamai plugin dan marketplace tempat asalnya:

```json theme={null}
{
  "channelsEnabled": true,
  "allowedChannelPlugins": [
    { "marketplace": "claude-plugins-official", "plugin": "telegram" },
    { "marketplace": "claude-plugins-official", "plugin": "discord" },
    { "marketplace": "acme-corp-plugins", "plugin": "internal-alerts" }
  ]
}
```

Ketika `allowedChannelPlugins` diatur, itu menggantikan daftar penyetujuan Anthropic sepenuhnya: hanya plugin yang terdaftar yang dapat mendaftar. Biarkan tidak diatur untuk kembali ke daftar penyetujuan Anthropic default. Array kosong memblokir semua plugin channel dari daftar penyetujuan, tetapi `--dangerously-load-development-channels` masih dapat melewatinya untuk pengujian lokal. Untuk memblokir channels sepenuhnya termasuk bendera pengembangan, biarkan `channelsEnabled` tidak diatur sebagai gantinya.

Pengaturan ini memerlukan `channelsEnabled: true`. Jika pengguna melewatkan plugin ke `--channels` yang tidak ada di daftar Anda, Claude Code dimulai secara normal tetapi channel tidak mendaftar, dan pemberitahuan startup menjelaskan bahwa plugin tidak ada di daftar yang disetujui organisasi.

<h2 id="research-preview">
  Research preview
</h2>

Channels adalah fitur research preview. Ketersediaan sedang diluncurkan secara bertahap, dan sintaks bendera `--channels` serta kontrak protokol dapat berubah berdasarkan umpan balik.

Selama pratinjau, `--channels` hanya menerima plugin dari daftar penyetujuan yang dipertahankan Anthropic, atau dari daftar penyetujuan organisasi Anda jika admin telah mengatur [`allowedChannelPlugins`](#restrict-which-channel-plugins-can-run). Plugin channel di [claude-plugins-official](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins) adalah set yang disetujui secara default. Jika Anda melewatkan sesuatu yang tidak ada di daftar penyetujuan yang efektif, Claude Code dimulai secara normal tetapi channel tidak mendaftar, dan pemberitahuan startup memberi tahu Anda mengapa.

Untuk menguji channel yang Anda bangun, gunakan `--dangerously-load-development-channels`. Lihat [Test during the research preview](/docs/id/channels-reference#test-during-the-research-preview) untuk informasi tentang menguji channels khusus yang Anda bangun.

Laporkan masalah atau umpan balik di [repositori GitHub Claude Code](https://github.com/anthropics/claude-code/issues).

<h2 id="how-channels-compare">
  Perbandingan channel
</h2>

Beberapa fitur Claude Code terhubung ke sistem di luar terminal, masing-masing cocok untuk jenis pekerjaan yang berbeda:

| Fitur                                                | Apa yang dilakukannya                                                     | Bagus untuk                                                         |
| ---------------------------------------------------- | ------------------------------------------------------------------------- | ------------------------------------------------------------------- |
| [Claude Code on the web](/docs/id/claude-code-on-the-web) | Menjalankan tugas dalam sandbox cloud baru, diklon dari GitHub            | Mendelegasikan pekerjaan async yang mandiri yang Anda periksa nanti |
| [Claude in Slack](/docs/id/slack)                         | Menjalankan sesi web dari penyebutan `@Claude` di channel atau thread     | Memulai tugas langsung dari konteks percakapan tim                  |
| Standard [MCP server](/docs/id/mcp)                       | Claude menanyainya selama tugas; tidak ada yang didorong ke sesi          | Memberi Claude akses on-demand untuk membaca atau menanyakan sistem |
| [Remote Control](/docs/id/remote-control)                 | Anda mengemudi sesi lokal Anda dari claude.ai atau aplikasi Claude mobile | Mengarahkan sesi yang sedang berlangsung saat jauh dari meja Anda   |

Channels mengisi celah dalam daftar itu dengan mendorong acara dari sumber non-Claude ke dalam sesi lokal Anda yang sudah berjalan.

* **Chat bridge**: tanyakan sesuatu kepada Claude dari ponsel Anda melalui Telegram, Discord, atau iMessage, dan jawabannya kembali dalam obrolan yang sama saat pekerjaan berjalan di mesin Anda terhadap file nyata Anda.
* **[Webhook receiver](/docs/id/channels-reference#example-build-a-webhook-receiver)**: webhook dari CI, pelacak kesalahan Anda, pipeline deploy, atau layanan eksternal lainnya tiba di mana Claude sudah memiliki file Anda terbuka dan mengingat apa yang Anda debug.

<h2 id="next-steps">
  Langkah selanjutnya
</h2>

Setelah Anda memiliki channel yang berjalan, jelajahi fitur terkait ini:

* [Build your own channel](/docs/id/channels-reference) untuk sistem yang belum memiliki plugins
* [Remote Control](/docs/id/remote-control) untuk mengemudi sesi lokal dari ponsel Anda alih-alih meneruskan acara ke dalamnya
* [Scheduled tasks](/docs/id/scheduled-tasks) untuk polling pada timer alih-alih bereaksi terhadap acara yang didorong
