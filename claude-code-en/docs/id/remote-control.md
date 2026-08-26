> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Lanjutkan sesi lokal dari perangkat apa pun dengan Remote Control

> Lanjutkan sesi Claude Code lokal dari ponsel, tablet, atau browser apa pun menggunakan Remote Control. Bekerja dengan claude.ai/code dan aplikasi Claude mobile.

<Note>
  Remote Control sedang dalam pratinjau penelitian dan tersedia di semua paket. Di Tim dan Enterprise, Remote Control dimatikan secara default sampai Pemilik mengaktifkan toggle Remote Control di [pengaturan admin Claude Code](https://claude.ai/admin-settings/claude-code).
</Note>

Remote Control menghubungkan [claude.ai/code](https://claude.ai/code) atau aplikasi Claude untuk [iOS](https://apps.apple.com/us/app/claude-by-anthropic/id6473753684) dan [Android](https://play.google.com/store/apps/details?id=com.anthropic.claude) ke sesi Claude Code yang berjalan di mesin Anda. Mulai tugas di meja Anda, kemudian lanjutkan dari ponsel Anda di sofa atau browser di komputer lain.

Ketika Anda memulai sesi Remote Control di mesin Anda, Claude terus berjalan secara lokal sepanjang waktu, jadi eksekusi kode dan akses sistem file Anda tetap berada di mesin Anda. Dengan Remote Control Anda dapat:

* **Gunakan lingkungan lokal penuh Anda dari jarak jauh**: sistem file, [MCP servers](/docs/id/mcp), alat, dan konfigurasi proyek Anda tetap tersedia, dan mengetik `@` melengkapi otomatis jalur file dari proyek lokal Anda
* **Bekerja dari kedua permukaan sekaligus**: percakapan dan kemajuan [subagents](/docs/id/sub-agents) dan [dynamic workflows](/docs/id/workflows) tetap tersinkronisasi di semua perangkat yang terhubung, sehingga Anda dapat mengirim pesan dari terminal, browser, dan ponsel Anda secara bergantian. Sebelum v2.1.207, sesi yang dihosting oleh [Desktop app](/docs/id/desktop) tidak mengirim kemajuan subagent atau workflow ke perangkat yang terhubung.
* **Kirim gambar dan file dari ponsel atau browser Anda**: ketika Anda menambahkan lampiran di aplikasi Claude atau di claude.ai/code, Claude Code mengunduhnya ke mesin Anda dan meneruskannya ke Claude sebagai referensi file `@`, dengan atau tanpa keterangan. Sebelum v2.1.202, Claude Code dapat menghilangkan lampiran yang dikirim tanpa keterangan sebelum mencapai sesi.
* **Bertahan dari gangguan**: jika laptop Anda tidur atau jaringan Anda terputus, sesi akan terhubung kembali secara otomatis ketika mesin Anda kembali online. Claude Code mengantrikan pembaruan status dari subagents dan workflows saat koneksi sedang dibangun kembali dan mengirimkannya setelah pemulihan. Sebelum v2.1.207, pembaruan yang dikirim selama reconnection atau credential refresh dapat hilang, jadi perangkat yang terhubung terus menampilkan tugas yang selesai sebagai sedang berjalan.

Tidak seperti [Claude Code di web](/docs/id/claude-code-on-the-web), yang berjalan di infrastruktur cloud, sesi Remote Control berjalan langsung di mesin Anda dan berinteraksi dengan sistem file lokal Anda. Antarmuka web dan mobile hanyalah jendela ke sesi lokal tersebut.

Halaman ini mencakup pengaturan, cara memulai dan terhubung ke sesi, dan bagaimana Remote Control dibandingkan dengan Claude Code di web.

<h2 id="requirements">
  Persyaratan
</h2>

Sebelum menggunakan Remote Control, konfirmasi bahwa lingkungan Anda memenuhi kondisi berikut:

* **Langganan**: tersedia di paket Pro, Max, Tim, dan Enterprise. Kunci API tidak didukung. Di Tim dan Enterprise, seorang Pemilik harus terlebih dahulu mengaktifkan toggle Remote Control di [pengaturan admin Claude Code](https://claude.ai/admin-settings/claude-code).
* **Autentikasi**: jalankan `claude` dan gunakan `/login` untuk masuk melalui claude.ai jika Anda belum melakukannya.
* **Titik akhir API**: tidak tersedia di Amazon Bedrock, Google Cloud's Agent Platform, atau Microsoft Foundry. Mulai dari v2.1.196, Remote Control juga dinonaktifkan ketika [`ANTHROPIC_BASE_URL`](/docs/id/env-vars) menunjuk ke host selain `api.anthropic.com`, seperti [gateway LLM](/docs/id/llm-gateway) atau proxy. Batalkan pengaturan variabel untuk menggunakan Remote Control.
* **Kepercayaan ruang kerja**: jalankan `claude` di direktori proyek Anda setidaknya sekali untuk menerima dialog kepercayaan ruang kerja.

<h2 id="start-a-remote-control-session">
  Mulai sesi Remote Control
</h2>

Anda dapat memulai sesi Remote Control dari CLI atau ekstensi VS Code. CLI menawarkan tiga mode invokasi; VS Code menggunakan perintah `/remote-control`.

<Tabs>
  <Tab title="Mode server">
    Navigasikan ke direktori proyek Anda dan jalankan:

    ```bash theme={null}
    claude remote-control
    ```

    Proses tetap berjalan di terminal Anda dalam mode server, menunggu koneksi jarak jauh. Ini menampilkan URL sesi yang dapat Anda gunakan untuk [terhubung dari perangkat lain](#connect-from-another-device), dan Anda dapat menekan spacebar untuk menampilkan kode QR untuk akses cepat dari ponsel Anda. Saat sesi jarak jauh aktif, terminal menampilkan status koneksi dan aktivitas alat.

    Bendera yang tersedia:

    | Bendera                                         | Deskripsi                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
    | ----------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
    | `--name "My Project"`                           | Tetapkan judul sesi khusus yang terlihat dalam daftar sesi di claude.ai/code.                                                                                                                                                                                                                                                                                                                                                                                                                              |
    | `--remote-control-session-name-prefix <prefix>` | Awalan untuk nama sesi yang dibuat secara otomatis ketika tidak ada nama eksplisit yang ditetapkan. Default adalah nama mesin Anda, menghasilkan nama seperti `myhost-graceful-unicorn`. Atur `CLAUDE_REMOTE_CONTROL_SESSION_NAME_PREFIX` untuk efek yang sama.                                                                                                                                                                                                                                            |
    | `-c`, `--continue`                              | Lanjutkan sesi Remote Control terbaru yang dimulai dari direktori ini alih-alih membuat yang baru. Tidak dapat digabungkan dengan `--session-id`, `--spawn`, `--capacity`, atau `--create-session-in-dir`. Memerlukan Claude Code v2.1.200 atau lebih baru; versi sebelumnya menolak bendera sebagai argumen yang tidak dikenal.                                                                                                                                                                           |
    | `--session-id <id>`                             | Lanjutkan sesi Remote Control tertentu berdasarkan ID-nya. Tidak dapat digabungkan dengan `--continue`, `--spawn`, `--capacity`, atau `--create-session-in-dir`. Memerlukan Claude Code v2.1.200 atau lebih baru; versi sebelumnya menolak bendera sebagai argumen yang tidak dikenal.                                                                                                                                                                                                                     |
    | `--spawn <mode>`                                | Bagaimana server membuat sesi.<br />• `same-dir` (default): semua sesi berbagi direktori kerja saat ini, sehingga dapat bertentangan jika mengedit file yang sama.<br />• `worktree`: setiap sesi sesuai permintaan mendapatkan [git worktree](/docs/id/worktrees) miliknya sendiri. Memerlukan repositori git.<br />• `session`: mode sesi tunggal. Melayani tepat satu sesi dan menolak koneksi tambahan. Atur saat startup saja.<br />Tekan `w` saat runtime untuk beralih antara `same-dir` dan `worktree`. |
    | `--capacity <N>`                                | Jumlah maksimum sesi bersamaan. Default adalah 32. Tidak dapat digunakan dengan `--spawn=session`.                                                                                                                                                                                                                                                                                                                                                                                                         |
    | `--[no-]create-session-in-dir`                  | Buat sesi sebelumnya di direktori saat ini ketika server dimulai, sehingga Anda memiliki tempat untuk mengetik segera. Dalam mode `worktree` sesi ini tetap berada di direktori saat ini sementara sesi sesuai permintaan mendapatkan worktree terisolasi. Aktif secara default; berikan `--no-create-session-in-dir` untuk memulai tanpa ada.                                                                                                                                                             |
    | `--verbose`                                     | Tampilkan log koneksi dan sesi terperinci.                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
    | `--sandbox` / `--no-sandbox`                    | Aktifkan atau nonaktifkan [sandboxing](/docs/id/sandboxing) untuk isolasi sistem file dan jaringan. Dimatikan secara default.                                                                                                                                                                                                                                                                                                                                                                                   |
  </Tab>

  <Tab title="Sesi interaktif">
    Untuk memulai sesi Claude Code interaktif normal dengan Remote Control diaktifkan, gunakan bendera `--remote-control` (atau `--rc`):

    ```bash theme={null}
    claude --remote-control
    ```

    Secara opsional berikan nama untuk sesi:

    ```bash theme={null}
    claude --remote-control "My Project"
    ```

    Ini memberi Anda sesi interaktif penuh di terminal Anda yang juga dapat Anda kontrol dari claude.ai atau aplikasi Claude. Tidak seperti `claude remote-control` (mode server), Anda dapat mengetik pesan secara lokal sementara sesi juga tersedia dari jarak jauh.
  </Tab>

  <Tab title="Dari sesi yang ada">
    Jika Anda sudah dalam sesi Claude Code dan ingin melanjutkannya dari jarak jauh, gunakan perintah `/remote-control` (atau `/rc`):

    ```text theme={null}
    /remote-control
    ```

    Berikan nama sebagai argumen untuk menetapkan judul sesi khusus:

    ```text theme={null}
    /remote-control My Project
    ```

    Ini memulai sesi Remote Control yang membawa riwayat percakapan saat ini.

    Bendera `--verbose`, `--sandbox`, dan `--no-sandbox` tidak tersedia dengan perintah ini.
  </Tab>

  <Tab title="VS Code">
    Di [ekstensi VS Code Claude Code](/docs/id/vs-code), ketik `/remote-control` atau `/rc` di kotak prompt, atau buka menu perintah dengan `/` dan pilihnya.

    ```text theme={null}
    /remote-control
    ```

    Spanduk muncul di atas kotak prompt yang menunjukkan status koneksi. Setelah terhubung, klik **Open in browser** di spanduk untuk langsung ke sesi, atau temukan di daftar sesi di [claude.ai/code](https://claude.ai/code). URL sesi juga diposting dalam percakapan.

    Untuk memutuskan sambungan, klik ikon tutup di spanduk atau jalankan `/remote-control` lagi.

    Tidak seperti CLI, perintah VS Code tidak menerima argumen nama atau menampilkan kode QR. Judul sesi berasal dari riwayat percakapan Anda atau prompt pertama.
  </Tab>
</Tabs>

<h3 id="check-connection-status">
  Periksa status koneksi
</h3>

Dalam sesi terminal interaktif, indikator `/rc active` berada di footer di bawah kotak input saat koneksi aktif, dan disembunyikan jika terminal terlalu sempit untuk menampilkannya. Teks indikator adalah tautan ke sesi di claude.ai. Pilihnya dengan tombol panah bawah dan tekan Enter, atau jalankan `/remote-control` lagi, untuk membuka panel status dengan URL sesi dan kode QR yang dapat Anda gunakan untuk [terhubung dari perangkat lain](#connect-from-another-device).

Jika koneksi gagal, notifikasi muncul dengan alasan kegagalan dan indikator hilang dari footer. Jalankan `/remote-control` lagi untuk mencoba ulang.

<h3 id="connect-from-another-device">
  Terhubung dari perangkat lain
</h3>

Setelah sesi Remote Control aktif, Anda memiliki beberapa cara untuk terhubung dari perangkat lain:

* **Buka URL sesi** di browser apa pun untuk langsung ke sesi di [claude.ai/code](https://claude.ai/code).
* **Pindai kode QR** yang ditampilkan bersama URL sesi untuk membukanya langsung di aplikasi Claude. Dengan `claude remote-control`, tekan spacebar untuk beralih tampilan kode QR.
* **Buka [claude.ai/code](https://claude.ai/code) atau aplikasi Claude** dan temukan sesi berdasarkan nama dalam daftar sesi. Di aplikasi mobile Claude, ketuk **Code** dalam navigasi untuk mencapai daftar sesi. Sesi Remote Control menampilkan ikon komputer dengan titik status hijau saat online.

Ketika Anda terhubung, perangkat menampilkan subagen dan alur kerja apa pun yang sudah dijalankan sesi di latar belakang. Sebelum v2.1.208, perangkat yang terhubung ke sesi yang dihosting di terminal interaktif tidak menampilkan subagen dan alur kerja yang sudah berjalan sampai salah satu dari mereka dimulai atau berhenti.

Judul sesi jarak jauh dipilih dalam urutan ini:

1. Nama yang Anda berikan ke `--name`, `--remote-control`, atau `/remote-control`
2. Judul yang Anda tetapkan dengan `/rename`
3. Pesan bermakna terakhir dalam riwayat percakapan yang ada
4. Nama yang dibuat secara otomatis seperti `myhost-graceful-unicorn`, di mana `myhost` adalah nama mesin Anda atau awalan yang Anda tetapkan dengan `--remote-control-session-name-prefix`

Jika Anda tidak menetapkan nama eksplisit, judul akan diperbarui untuk mencerminkan prompt Anda setelah Anda mengirimnya. Mulai dari Claude Code v2.1.176, judul yang dibuat secara otomatis cocok dengan bahasa percakapan Anda, atau pengaturan [`language`](/docs/id/settings#available-settings) jika satu dikonfigurasi. Mengganti nama sesi dari claude.ai atau aplikasi Claude juga memperbarui judul lokal yang ditampilkan di `claude --resume`.

Jika lingkungan sudah memiliki sesi aktif, Anda akan ditanya apakah akan melanjutkannya atau memulai yang baru.

Jika Anda belum memiliki aplikasi Claude, gunakan perintah `/mobile` di dalam Claude Code untuk menampilkan kode QR unduhan untuk [iOS](https://apps.apple.com/us/app/claude-by-anthropic/id6473753684) atau [Android](https://play.google.com/store/apps/details?id=com.anthropic.claude).

<h3 id="enable-remote-control-for-all-sessions">
  Aktifkan Remote Control untuk semua sesi
</h3>

Remote Control hanya diaktifkan ketika Anda secara eksplisit menjalankan `claude remote-control`, `claude --remote-control`, atau `/remote-control`, kecuali auto-connect diaktifkan. Untuk mengaktifkannya secara otomatis untuk setiap sesi interaktif, jalankan `/config` di dalam Claude Code dan atur **Enable Remote Control for all sessions** ke `true`. Atur ke `false` untuk tidak pernah auto-connect, atau biarkan tidak diatur untuk mengikuti default organisasi Anda. Di aplikasi Desktop, Anda juga dapat mengalihkan ini dari **Settings → Claude Code → Enable remote control by default**. Di [ekstensi VS Code](/docs/id/vs-code#use-the-prompt-box), toggle yang sama muncul sebagai **Enable Remote Control for all sessions** di bagian Settings menu perintah; memerlukan Claude Code v2.1.203 atau lebih baru.

Dengan pengaturan ini aktif, setiap proses Claude Code interaktif mendaftarkan satu sesi jarak jauh. Jika Anda menjalankan beberapa instance, masing-masing mendapatkan lingkungan dan sesi sendiri. Untuk menjalankan beberapa sesi bersamaan dari satu proses, gunakan [mode server](#start-a-remote-control-session) sebagai gantinya.

<h2 id="connection-and-security">
  Koneksi dan keamanan
</h2>

Sesi Claude Code lokal Anda membuat permintaan HTTPS keluar saja dan tidak pernah membuka port masuk di mesin Anda. Ketika Anda memulai Remote Control, sesi tersebut mendaftarkan dengan API Anthropic dan polling untuk pekerjaan. Ketika Anda terhubung dari perangkat lain, server merutekan pesan antara klien web atau mobile dan sesi lokal Anda melalui koneksi streaming.

Semua lalu lintas berjalan melalui API Anthropic melalui TLS, keamanan transportasi yang sama seperti sesi Claude Code apa pun. Koneksi menggunakan beberapa kredensial berumur pendek, masing-masing dibatasi untuk satu tujuan dan kedaluwarsa secara independen.

Saat Remote Control terhubung, transkrip sesi, termasuk pesan Anda, respons Claude, dan aktivitas alat, disimpan di server Anthropic. Transkrip yang disimpan menjaga percakapan tetap sinkron di seluruh perangkat Anda dan memungkinkan sesi untuk terhubung kembali setelah gangguan jaringan. Eksekusi dan akses sistem file tetap berada di mesin Anda, dan transkrip yang disimpan dipertahankan sesuai dengan kebijakan [Penggunaan data](/docs/id/data-usage).

Untuk mematikan Remote Control sepenuhnya, gunakan pengaturan [`disableRemoteControl`](/docs/id/settings#available-settings). Organisasi dengan persyaratan kepatuhan seperti Zero Data Retention tidak dapat mengaktifkan Remote Control.

<h2 id="trusted-devices">
  Perangkat Terpercaya
</h2>

<Note>
  Perangkat Terpercaya saat ini dalam beta. Fitur dan fungsionalitas dapat berkembang seiring pengalaman disempurnakan.

  Perangkat Terpercaya tersedia di paket Tim dan Enterprise. Ini dimatikan secara default sampai admin mengaktifkannya.
</Note>

Perangkat Terpercaya adalah pengaturan seluruh organisasi yang memerlukan anggota untuk memverifikasi perangkat mereka sebelum mereka dapat melihat atau mengarahkan sesi Remote Control dari claude.ai, aplikasi Claude mobile, atau Claude Desktop. Ini mengikat akses Remote Control ke perangkat yang dikenal dan autentikasi terbaru, bukan hanya akun yang masuk.

Ketika pengaturan aktif, berinteraksi dengan sesi Remote Control memerlukan keduanya:

* **Perangkat yang terdaftar**: setiap browser, ponsel, atau aplikasi desktop yang digunakan anggota untuk Remote Control mendaftarkan kredensialnya sendiri. Pendaftaran hanya ditawarkan segera setelah masuk penuh, jadi perangkat bergabung dengan daftar terpercaya sebagai bagian dari autentikasi nyata daripada diam-diam di latar belakang.
* **Masuk terbaru**: masuk anggota tidak boleh lebih dari 18 jam yang lalu. Alih-alih masuk lagi setiap hari, anggota mengonfirmasi kehadiran dengan Face ID, Touch ID, Windows Hello, atau passkey. Langkah step-up biometrik ini menyegarkan sesi segera.

Pemeriksaan biometrik berjalan di perangkat melalui sistem operasi atau browser, mekanisme yang sama dengan masuk passkey. Anthropic tidak pernah menerima atau menyimpan sidik jari, data wajah, atau informasi biometrik lainnya. Hanya kunci publik perangkat dan metadata dasar seperti nama tampilan, platform, dan waktu pendaftaran yang disimpan.

Pengaturan hanya berlaku untuk Remote Control. Obrolan Claude biasa, Claude Code di terminal, dan penggunaan API tidak terpengaruh.

<h3 id="enable-trusted-devices-for-your-organization">
  Aktifkan Perangkat Terpercaya untuk organisasi Anda
</h3>

Admin mengaktifkan pengaturan dari konsol admin Claude Code.

<Steps>
  <Step title="Buka pengaturan admin Claude Code">
    Buka [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code). Toggle **Require trusted devices** muncul di bawah pengaturan Remote Control.
  </Step>

  <Step title="Aktifkan Require trusted devices">
    Pengaturan berlaku untuk setiap anggota organisasi dan untuk sesi Remote Control yang dimulai setelah Anda mengaktifkannya. Sesi yang sudah berjalan sebelum toggle diaktifkan tidak dilindungi secara retroaktif dan terus tanpa persyaratan perangkat sampai mereka berakhir. Scoping per-tim atau per-proyek tidak tersedia.
  </Step>

  <Step title="Beri tahu anggota apa yang diharapkan">
    Pertama kali anggota melihat atau mengarahkan sesi Remote Control baru dari browser, ponsel, atau aplikasi desktop setelah pengaturan diaktifkan, mereka diminta untuk mendaftarkan perangkat itu. Memberi tahu mereka sebelumnya menghindari kebingungan.
  </Step>
</Steps>

<h3 id="what-members-see">
  Apa yang dilihat anggota
</h3>

Pendaftaran adalah langkah satu kali per perangkat. Setelah itu, satu-satunya perubahan yang terlihat adalah prompt biometrik sesekali.

* **Penggunaan pertama di setiap perangkat**: anggota diminta untuk mendaftarkan. Jika masuk mereka tidak terbaru, mereka masuk terlebih dahulu melalui alur normal Anda, termasuk SSO jika dikonfigurasi, kemudian mengonfirmasi pendaftaran.
* **Hari ke hari**: anggota dengan perangkat terdaftar dan masuk terbaru tidak melihat prompt. Ketika masuk berusia lebih dari 18 jam, interaksi Remote Control berikutnya menunjukkan prompt Face ID, Touch ID, Windows Hello, atau passkey tunggal.
* **Perangkat yang tidak terdaftar**: sesi Remote Control tidak dapat dilihat atau diarahkan sampai perangkat terdaftar. Obrolan Claude biasa di perangkat itu tidak terpengaruh.
* **Tidak ada autentikator platform**: anggota di mesin tanpa Face ID, Touch ID, atau Windows Hello dapat menggunakan kunci keamanan perangkat keras, atau masuk lagi alih-alih step-up.
* **Di terminal**: mesin yang menjalankan Claude Code menerima kredensialnya sendiri secara otomatis ketika pengembang masuk ke CLI. Tidak ada langkah pendaftaran terpisah di terminal.

<h3 id="manage-enrolled-devices">
  Kelola perangkat yang terdaftar
</h3>

Anggota dapat meninjau dan mencabut perangkat mereka sendiri dari pengaturan akun.

Buka [claude.ai/settings/account](https://claude.ai/settings/account#trusted-devices) dan temukan bagian **Trusted devices** untuk melihat setiap perangkat terdaftar dengan nama, platform, dan tanggal pendaftarannya. Menghapus perangkat mencabut kredensialnya segera, dan perangkat dapat mendaftar ulang nanti setelah masuk segar. Kredensial juga kedaluwarsa sendiri jika tidak diperbarui, jadi perangkat yang tidak digunakan jatuh dari daftar terpercaya secara otomatis.

Untuk perangkat yang hilang atau dicuri, anggota menghapusnya dari halaman ini. Jika anggota tidak dapat masuk, admin dapat menggunakan **Sign out everywhere** di konsol admin untuk mencabut setiap sesi dan perangkat terdaftar untuk anggota itu, setelah itu anggota mendaftarkan ulang perangkat yang masih mereka miliki.

<h2 id="remote-control-vs-claude-code-on-the-web">
  Remote Control vs Claude Code di web
</h2>

Remote Control dan [Claude Code di web](/docs/id/claude-code-on-the-web) keduanya menggunakan antarmuka claude.ai/code. Perbedaan utamanya adalah di mana sesi berjalan: Remote Control dieksekusi di mesin Anda, sehingga MCP servers lokal, alat, dan konfigurasi proyek Anda tetap tersedia. Claude Code di web dieksekusi di infrastruktur cloud yang dikelola Anthropic.

Gunakan Remote Control ketika Anda sedang dalam pekerjaan lokal dan ingin terus melanjutkan dari perangkat lain. Gunakan Claude Code di web ketika Anda ingin memulai tugas tanpa pengaturan lokal apa pun, bekerja pada repo yang tidak Anda miliki klonnya, atau menjalankan beberapa tugas secara paralel.

<h2 id="mobile-push-notifications">
  Notifikasi push mobile
</h2>

Ketika Remote Control aktif, Claude dapat mengirim notifikasi push ke ponsel Anda.

Claude memutuskan kapan harus push. Biasanya mengirim satu ketika tugas yang berjalan lama selesai atau ketika memerlukan keputusan dari Anda untuk melanjutkan. Anda juga dapat meminta push dalam prompt Anda, misalnya `notify me when the tests finish`. Selain dua toggle on/off di bawah, tidak ada konfigurasi per-event.

Untuk menyiapkan notifikasi push mobile:

<Steps>
  <Step title="Instal aplikasi Claude mobile">
    Unduh aplikasi Claude untuk [iOS](https://apps.apple.com/us/app/claude-by-anthropic/id6473753684) atau [Android](https://play.google.com/store/apps/details?id=com.anthropic.claude).
  </Step>

  <Step title="Masuk dengan akun Claude Code Anda">
    Gunakan akun dan organisasi yang sama yang Anda gunakan untuk Claude Code di terminal.
  </Step>

  <Step title="Izinkan notifikasi">
    Terima prompt izin notifikasi dari sistem operasi.
  </Step>

  <Step title="Aktifkan push di Claude Code">
    Di terminal Anda, jalankan `/config` dan aktifkan **Push when Claude decides** untuk notifikasi proaktif, **Push when actions required** untuk prompt izin dan pertanyaan, atau keduanya.
  </Step>
</Steps>

Jika notifikasi tidak tiba:

* Jika `/config` menunjukkan **No mobile registered**, buka aplikasi Claude di ponsel Anda sehingga dapat menyegarkan token push-nya. Peringatan hilang saat Remote Control terhubung berikutnya.
* Di iOS, Focus modes dan notification summaries dapat menekan atau menunda push. Periksa Settings → Notifications → Claude.
* Di Android, optimasi baterai yang agresif dapat menunda pengiriman. Kecualikan aplikasi Claude dari optimasi baterai di pengaturan sistem.

Claude Code melewatkan notifikasi push mobile saat Anda mengetik atau fokus pada terminal yang terhubung. Mulai dari v2.1.181, Anda dapat mengatur [`CLAUDE_CLIENT_PRESENCE_FILE`](/docs/id/env-vars) ke jalur file penanda untuk memperluas ini ke kapan saja Anda berada di mesin, bahkan di jendela lain: notifikasi dilewatkan saat file ada. Konfigurasikan pendengar kunci layar atau alat serupa untuk membuat file saat layar Anda membuka kunci dan menghapusnya saat layar Anda terkunci.

<h2 id="limitations">
  Keterbatasan
</h2>

* **Satu sesi jarak jauh per proses interaktif**: di luar mode server, setiap instance Claude Code mendukung satu sesi jarak jauh pada satu waktu. Gunakan [mode server](#start-a-remote-control-session) untuk menjalankan beberapa sesi bersamaan dari satu proses.
* **Proses lokal harus tetap berjalan**: Remote Control berjalan sebagai proses lokal. Jika Anda menutup terminal, keluar dari VS Code, atau menghentikan proses `claude`, sesi berakhir.
* **Pemadaman jaringan yang diperpanjang**: jika mesin Anda aktif tetapi tidak dapat menjangkau jaringan selama lebih dari kira-kira 10 menit, sesi habis waktu dan proses keluar. Jalankan `claude remote-control` lagi untuk memulai sesi baru.
* **Ultraplan memutuskan Remote Control**: memulai sesi [ultraplan](/docs/id/ultraplan) memutuskan sesi Remote Control aktif apa pun karena kedua fitur menempati antarmuka claude.ai/code dan hanya satu yang dapat terhubung pada satu waktu.
* **Beberapa perintah hanya lokal**: perintah yang hanya berjalan di antarmuka terminal, seperti `/plugin` atau `/resume`, hanya bekerja dari CLI lokal, terlepas dari apakah Anda meneruskan argumen atau tidak. Perintah berikut bekerja dari mobile dan web:
  * Perintah output teks: `/compact`, `/clear`, `/context`, `/usage`, `/exit`, `/usage-credits` (menjalankan bentuk teks alih-alih membuka dialog dalam CLI), `/recap`, `/reload-plugins`
  * `/model`, `/effort`, `/fast`, `/color`, dan `/rename`: teruskan nilai sebagai argumen, misalnya `/model sonnet` atau `/effort high`. Dari mobile dan web, `/model` dan `/effort` mengambil argumen sebagai pengganti pemilih terminal atau slider.
  * `/mcp`, dari v2.1.166: dari aplikasi mobile, mengembalikan ringkasan teks status server alih-alih membuka pemilih. Di web, `/mcp` sendiri membuka direktori [konektor claude.ai](/docs/id/mcp#use-mcp-servers-from-claude-ai) alih-alih mengembalikan ringkasan. [Subperintah](/docs/id/commands#all-commands) `reconnect`, `enable`, dan `disable` bekerja dari keduanya. Tidak seperti CLI lokal, `/mcp reconnect` tanpa nama server menghubungkan kembali setiap server yang gagal atau memerlukan autentikasi.
  * `/config`, dari v2.1.181: dari aplikasi mobile, teruskan `key=value` untuk menetapkan pengaturan, atau jalankan tanpa argumen untuk membuat daftar kunci yang dapat Anda atur. Di web, `/config` membuka bagian Claude Code dari pengaturan Anda sebagai gantinya, dan mengabaikan teks setelah perintah.

<h2 id="troubleshooting">
  Pemecahan Masalah
</h2>

<h3 id="remote-control-requires-a-claude-ai-subscription">
  "Remote Control memerlukan langganan claude.ai"
</h3>

Anda tidak diautentikasi dengan akun claude.ai. Jalankan `claude auth login` dan pilih opsi claude.ai. Jika `ANTHROPIC_API_KEY` diatur di lingkungan Anda, batalkan pengaturannya terlebih dahulu.

Sebelum v2.1.206, menjalankan `/remote-control` saat tidak masuk melaporkan `Unknown command: /remote-control` alih-alih pesan ini.

<h3 id="remote-control-requires-a-full-scope-login-token">
  "Remote Control memerlukan token login dengan cakupan penuh"
</h3>

Anda diautentikasi dengan token berumur panjang dari `claude setup-token` atau variabel lingkungan `CLAUDE_CODE_OAUTH_TOKEN`. Token ini terbatas pada inference-only dan tidak dapat membuat sesi Remote Control. Jalankan `claude auth login` untuk autentikasi dengan token sesi cakupan penuh sebagai gantinya.

<h3 id="unable-to-determine-your-organization-for-remote-control-eligibility">
  "Tidak dapat menentukan organisasi Anda untuk kelayakan Remote Control"
</h3>

Informasi akun cache Anda sudah usang atau tidak lengkap. Jalankan `claude auth login` untuk menyegarkannya.

<h3 id="remote-control-is-not-yet-enabled-for-your-account">
  "Remote Control belum diaktifkan untuk akun Anda"
</h3>

Peluncuran Remote Control belum mencapai akun Anda, atau hak akses cache Anda sudah ketinggalan zaman. Jika Anda baru-baru ini mengubah paket, jalankan `claude auth logout` kemudian `claude auth login` untuk menyegarkannya. Jalankan `claude doctor` untuk melihat pemeriksaan kelayakan individual mana yang gagal. Konflik variabel lingkungan, pemeriksaan yang tidak dapat dijangkau, dan kebijakan organisasi masing-masing menghasilkan pesan mereka sendiri, jadi kesalahan ini berarti gerbang peluncuran itu sendiri.

<h3 id="couldn’t-verify-remote-control-eligibility">
  "Tidak dapat memverifikasi kelayakan Remote Control"
</h3>

Claude Code tidak dapat menjangkau layanan feature-flag untuk memeriksa apakah Remote Control diaktifkan untuk akun Anda, biasanya karena Anda offline atau proxy memblokir permintaan. Coba lagi setelah Anda memiliki akses jaringan, atau jalankan `claude doctor` untuk detail. Pesan terkait "Tidak dapat memverifikasi kebijakan Remote Control organisasi Anda" memiliki penyebab yang sama dan perbaikan yang sama. Kedua pesan ditambahkan di v2.1.178.

<h3 id="remote-control-is-only-available-when-using-claude-via-api-anthropic-com">
  "Remote Control hanya tersedia saat menggunakan Claude melalui api.anthropic.com"
</h3>

Sesi tidak berbicara langsung ke API Anthropic, jadi tidak ada backend claude.ai untuk dipasangkan. Ini terjadi di Amazon Bedrock, Google Cloud's Agent Platform, dan Microsoft Foundry. Mulai dari v2.1.196, ini juga terjadi ketika [`ANTHROPIC_BASE_URL`](/docs/id/env-vars) menunjuk ke host selain `api.anthropic.com`, seperti [gateway LLM](/docs/id/llm-gateway) atau proxy, bahkan jika Anda masuk dengan claude.ai. Batalkan pengaturan `ANTHROPIC_BASE_URL` dan mulai ulang sesi untuk menggunakan Remote Control.

<h3 id="remote-control-is-disabled-by-your-organization’s-policy">
  "Remote Control dinonaktifkan oleh kebijakan organisasi Anda"
</h3>

Kesalahan ini memiliki empat penyebab yang berbeda. Jalankan `/status` terlebih dahulu untuk melihat metode login dan langganan mana yang Anda gunakan.

* **Anda diautentikasi dengan kunci API atau akun Console**: Remote Control memerlukan OAuth claude.ai. Jalankan `/login` dan pilih opsi claude.ai. Jika `ANTHROPIC_API_KEY` diatur di lingkungan Anda, batalkan pengaturannya.
* **Pemilik belum mengaktifkannya untuk organisasi Anda**: Remote Control dimatikan secara default di paket Team dan Enterprise. Pemilik dapat mengaktifkannya di [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code) dengan mengaktifkan toggle **Remote Control**. Toggle ini adalah pengaturan organisasi sisi server.
* **Toggle admin berwarna abu-abu**: organisasi Anda memiliki konfigurasi retensi data atau kepatuhan yang tidak kompatibel dengan Remote Control. Ini tidak dapat diubah dari panel admin. Hubungi dukungan Anthropic untuk membahas opsi.
* **Kesalahan menyebutkan `disableRemoteControl`**: administrator IT Anda telah menonaktifkan Remote Control di perangkat ini melalui [pengaturan yang dikelola](/docs/id/settings#settings-files), terlepas dari toggle organisasi-lebar.

<h3 id="remote-credentials-fetch-failed">
  "Remote credentials fetch failed"
</h3>

Claude Code tidak dapat memperoleh kredensial berumur pendek dari API Anthropic untuk membuat koneksi. Jalankan kembali dengan `--verbose` untuk melihat kesalahan lengkapnya:

```bash theme={null}
claude remote-control --verbose
```

Penyebab umum:

* Tidak masuk: jalankan `claude` dan gunakan `/login` untuk autentikasi dengan akun claude.ai Anda. Autentikasi kunci API tidak didukung untuk Remote Control.
* Masalah jaringan atau proxy: firewall atau proxy dapat memblokir permintaan HTTPS keluar. Remote Control memerlukan akses ke API Anthropic di port 443.
* Pembuatan sesi gagal: jika Anda juga melihat `Session creation failed — see debug log`, kegagalan terjadi lebih awal dalam pengaturan. Periksa bahwa langganan Anda aktif.

<h3 id="couldn’t-reconnect-to-your-remote-control-session">
  "Couldn't reconnect to your Remote Control session"
</h3>

Ketika Anda melanjutkan percakapan dengan `claude --resume` atau `claude --continue`, Claude Code terhubung kembali ke sesi Remote Control yang tercatat dalam percakapan tersebut. Pesan ini berarti koneksi ulang gagal karena alasan yang mungkin bersifat sementara, seperti gangguan jaringan atau kesalahan server, jadi Claude Code tidak dapat mengkonfirmasi apakah sesi jarak jauh masih ada. Ketika server mengkonfirmasi sesi sebelumnya tidak lagi ada, Claude Code membuat sesi Remote Control baru tanpa menampilkan pesan ini.

Sesi lokal Anda terus berjalan tanpa Remote Control. Jalankan `/remote-control` untuk mencoba koneksi lagi, atau mulai Claude Code tanpa `--resume` untuk membuat sesi Remote Control baru.

Sebelum v2.1.200, kegagalan koneksi ulang membuat sesi Remote Control baru alih-alih menampilkan pesan ini, yang meninggalkan sesi tambahan dalam daftar sesi di claude.ai/code.

<h3 id="your-organization-requires-trusted-devices-for-remote-control-but-this-device-is-not-enrolled">
  "Organisasi Anda memerlukan Perangkat Terpercaya untuk Remote Control, tetapi perangkat ini tidak terdaftar"
</h3>

Organisasi Anda telah mengaktifkan [Perangkat Terpercaya](#trusted-devices) dan mesin ini belum terdaftar. Jalankan `/login` di Claude Code. Pendaftaran terjadi sebagai bagian dari masuk, dan tidak ada perintah pendaftaran terpisah.

<h3 id="session-expired-for-trusted-device-check">
  "session expired for trusted-device check"
</h3>

Masuk Anda lebih dari 18 jam yang lalu. Jalankan `/login` di Claude Code, atau konfirmkan dengan Face ID, Touch ID, Windows Hello, atau passkey ketika claude.ai atau aplikasi mobile meminta Anda. Lihat [Perangkat Terpercaya](#trusted-devices).

<h2 id="choose-the-right-approach">
  Pilih pendekatan yang tepat
</h2>

Claude Code offers several ways to work when you're not at your terminal. They differ in what triggers the work, where Claude runs, and how much you need to set up.

|                                                          | Trigger                                                                                        | Claude runs on                                                                               | Setup                                                                                                                                | Best for                                                      |
| :------------------------------------------------------- | :--------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------ |
| [Dispatch](/docs/en/desktop#sessions-from-dispatch)           | Message a task from the Claude mobile app                                                      | Your machine (Desktop)                                                                       | [Pair the mobile app with Desktop](https://support.claude.com/en/articles/13947068)                                                  | Delegating work while you're away, minimal setup              |
| [Remote Control](/docs/en/remote-control)                     | Drive a running session from [claude.ai/code](https://claude.ai/code) or the Claude mobile app | Your machine (CLI or VS Code)                                                                | Run `claude remote-control`                                                                                                          | Steering in-progress work from another device                 |
| [Channels](/docs/en/channels)                                 | Push events from a chat app like Telegram or Discord, or your own server                       | Your machine (CLI)                                                                           | [Install a channel plugin](/docs/en/channels#quickstart) or [build your own](/docs/en/channels-reference)                                      | Reacting to external events like CI failures or chat messages |
| [Slack](/docs/en/slack)                                       | Mention `@Claude` in a team channel                                                            | Anthropic cloud                                                                              | [Install the Slack app](/docs/en/slack#setting-up-claude-code-in-slack) with [Claude Code on the web](/docs/en/claude-code-on-the-web) enabled | PRs and reviews from team chat                                |
| [Self-hosted environments](/docs/en/self-hosted-environments) | Start a [cloud session](/docs/en/claude-code-on-the-web) and pick your organization's environment   | Your organization's infrastructure                                                           | [Deploy runners](/docs/en/self-hosted-environments-quickstart), on Team and Enterprise plans                                              | Cloud sessions that must run inside your network              |
| [Scheduled tasks](/docs/en/scheduled-tasks)                   | Set a schedule                                                                                 | [CLI](/docs/en/scheduled-tasks), [Desktop](/docs/en/desktop-scheduled-tasks), or [cloud](/docs/en/routines) | Pick a frequency                                                                                                                     | Recurring automation like daily reviews                       |

<h2 id="related-resources">
  Sumber daya terkait
</h2>

* [Claude Code di web](/docs/id/claude-code-on-the-web): jalankan sesi di lingkungan cloud yang dikelola Anthropic alih-alih di mesin Anda
* [Ultraplan](/docs/id/ultraplan): luncurkan sesi perencanaan cloud dari terminal Anda dan tinjau rencana di browser Anda
* [Channels](/docs/id/channels): teruskan Telegram, Discord, atau iMessage ke sesi sehingga Claude bereaksi terhadap pesan saat Anda pergi
* [Dispatch](/docs/id/desktop#sessions-from-dispatch): kirim pesan tugas dari ponsel Anda dan dapat menjalankan sesi Desktop untuk menanganinya
* [Autentikasi](/docs/id/authentication): atur `/login` dan kelola kredensial untuk claude.ai
* [Referensi CLI](/docs/id/cli-reference): daftar lengkap bendera dan perintah termasuk `claude remote-control`
* [Keamanan](/docs/id/security): bagaimana sesi Remote Control sesuai dengan model keamanan Claude Code
* [Penggunaan data](/docs/id/data-usage): data apa yang mengalir melalui API Anthropic selama sesi lokal dan jarak jauh
