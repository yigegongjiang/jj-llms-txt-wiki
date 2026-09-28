> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Lanjutkan sesi lokal dari perangkat apa pun dengan Remote Control

> Lanjutkan sesi Claude Code lokal dari ponsel, tablet, atau browser apa pun menggunakan Remote Control. Bekerja dengan claude.ai/code dan aplikasi Claude mobile.

<Note>
  Remote Control tersedia di semua paket. Di Tim dan Enterprise, Remote Control dimatikan secara default sampai Pemilik mengaktifkan toggle Remote Control di [pengaturan admin Claude Code](https://claude.ai/admin-settings/claude-code).
</Note>

Remote Control menghubungkan [claude.ai/code](https://claude.ai/code) atau aplikasi Claude untuk [iOS](https://apps.apple.com/us/app/claude-by-anthropic/id6473753684) dan [Android](https://play.google.com/store/apps/details?id=com.anthropic.claude) ke sesi Claude Code yang berjalan di mesin Anda. Mulai tugas di meja Anda, kemudian lanjutkan dari ponsel Anda di sofa atau browser di komputer lain.

Ketika Anda memulai sesi Remote Control di mesin Anda, Claude terus berjalan secara lokal sepanjang waktu, jadi eksekusi kode dan akses sistem file Anda tetap berada di mesin Anda. Dengan Remote Control Anda dapat:

* **Gunakan lingkungan lokal penuh Anda dari jarak jauh**: sistem file, [MCP servers](/docs/id/mcp), alat, dan konfigurasi proyek Anda tetap tersedia, dan mengetik `@` melengkapi otomatis jalur file dari proyek lokal Anda.
* **Bekerja dari kedua permukaan sekaligus**: percakapan dan kemajuan [subagents](/docs/id/sub-agents) dan [dynamic workflows](/docs/id/workflows) tetap tersinkronisasi di semua perangkat yang terhubung, sehingga Anda dapat mengirim pesan dari terminal, browser, dan ponsel Anda secara bergantian.
* **Kirim gambar dan file dari ponsel atau browser Anda**: lampirkan foto atau file di aplikasi Claude atau di claude.ai/code, dengan atau tanpa keterangan. Claude melihat foto yang dilampirkan secara langsung sebagai bagian dari pesan Anda. Claude Code mengunduh file lain ke mesin Anda dan meneruskannya ke Claude sebagai referensi file `@`.
* **Bertahan dari gangguan**: jika laptop Anda tidur atau jaringan Anda terputus, Claude Code terhubung kembali secara otomatis ketika mesin Anda kembali online. Saat koneksi sedang dibangun kembali, Claude Code mengantrikan pesan, prompt izin, dan pembaruan status dari subagents dan workflows, dan mengirimkannya setelah koneksi pulih.

Tidak seperti [Claude Code di web](/docs/id/claude-code-on-the-web), yang berjalan di infrastruktur cloud, sesi Remote Control berjalan langsung di mesin Anda dan berinteraksi dengan sistem file lokal Anda. Antarmuka web dan mobile hanyalah jendela ke sesi lokal tersebut.

Halaman ini mencakup pengaturan, cara memulai dan terhubung ke sesi, dan bagaimana Remote Control dibandingkan dengan Claude Code di web.

<h2 id="requirements">
  Persyaratan
</h2>

Sebelum menggunakan Remote Control, konfirmasi bahwa lingkungan Anda memenuhi kondisi berikut:

* **Langganan**: tersedia di paket Pro, Max, Tim, dan Enterprise. Kunci API tidak didukung. Di Tim dan Enterprise, seorang Pemilik harus terlebih dahulu mengaktifkan toggle Remote Control di [pengaturan admin Claude Code](https://claude.ai/admin-settings/claude-code).
* **Autentikasi**: jalankan `claude` dan gunakan `/login` untuk masuk melalui claude.ai jika Anda belum melakukannya. Tanpa login yang memenuhi syarat, `claude remote-control` keluar dengan kesalahan, sementara `claude --remote-control` tetap memulai sesi interaktif dan menampilkan notifikasi kegagalan Remote Control segera setelah peluncuran.
* **Titik akhir API**: tidak tersedia di salah satu konfigurasi berikut:
  * Anda menggunakan Amazon Bedrock, Google Cloud's Agent Platform, atau Microsoft Foundry.
  * Anda menunjukkan [`ANTHROPIC_BASE_URL`](/docs/id/env-vars) ke host selain `api.anthropic.com`, seperti [gateway LLM](/docs/id/llm-gateway) atau proxy. Batalkan pengaturan variabel untuk menggunakan Remote Control. Sebelum v2.1.196, Claude Code memungkinkan Remote Control dengan `ANTHROPIC_BASE_URL` khusus.
  * Anda masuk melalui [gateway aplikasi Claude](/docs/id/claude-apps-gateway) perusahaan.
* **Evaluasi bendera fitur**: [`DISABLE_TELEMETRY`, `DO_NOT_TRACK`, `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`, dan `DISABLE_GROWTHBOOK`](/docs/id/env-vars) masing-masing menonaktifkan evaluasi bendera fitur yang bergantung pada ketersediaan Remote Control. Batalkan pengaturan variabel di mana pun variabel tersebut diatur, di lingkungan shell Anda atau di blok `env` dari [file `settings.json`](/docs/id/settings-reference#all-settings), untuk menggunakan Remote Control.
* **Kepercayaan ruang kerja**: jalankan `claude` di direktori proyek Anda setidaknya sekali untuk menerima dialog kepercayaan ruang kerja. Dialog kepercayaan startup tidak pernah menyimpan kepercayaan untuk direktori beranda Anda, jadi mulai Remote Control dari direktori proyek.

<h2 id="start-a-remote-control-session">
  Mulai sesi Remote Control
</h2>

Anda dapat memulai sesi Remote Control dari CLI atau ekstensi VS Code. CLI menawarkan tiga mode invokasi; VS Code menggunakan perintah `/remote-control`.

<Tabs>
  <Tab title="Mode server">
    Di direktori proyek Anda, jalankan:

    ```bash theme={null}
    claude remote-control
    ```

    Sampai Anda menerima konfirmasi satu kali Remote Control, `claude remote-control` menjelaskan apa yang dilakukannya dan menanyakan `Enable Remote Control? (y/n)` sebelum memulai server. Jawab `y` untuk menerima dan memulai server. Jika Anda menolak, Claude Code keluar tanpa memulai server dan menanyakan lagi saat berikutnya Anda menjalankan perintah.

    Proses tetap berjalan di terminal Anda dalam mode server, menunggu koneksi jarak jauh. Ini menampilkan URL sesi yang dapat Anda gunakan untuk [terhubung dari perangkat lain](#connect-from-another-device), dan Anda dapat menekan spacebar untuk menampilkan kode QR untuk akses cepat dari ponsel Anda. Saat sesi jarak jauh aktif, terminal menampilkan status koneksi dan aktivitas alat.

    Bendera yang tersedia:

    | Bendera                                         | Deskripsi                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
    | ----------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
    | `--name "My Project"`                           | Tetapkan judul sesi khusus yang terlihat dalam daftar sesi di claude.ai/code.                                                                                                                                                                                                                                                                                                                                                                                                                                      |
    | `--remote-control-session-name-prefix <prefix>` | Awalan untuk nama sesi yang dibuat secara otomatis ketika tidak ada nama eksplisit yang ditetapkan. Default adalah nama mesin Anda, menghasilkan nama seperti `myhost-graceful-unicorn`. Atur `CLAUDE_REMOTE_CONTROL_SESSION_NAME_PREFIX` untuk efek yang sama.                                                                                                                                                                                                                                                    |
    | `-c`, `--continue`                              | Bawa kembali sesi yang dimulai server terakhir di direktori ini, alih-alih membuat yang baru. Lihat [Resume sesi setelah menghentikan server](#resume-sessions-after-stopping-the-server). Tidak dapat digabungkan dengan `--session-id`, `--spawn`, `--capacity`, atau `--create-session-in-dir`. Memerlukan Claude Code v2.1.200 atau lebih baru; versi sebelumnya menolak bendera sebagai argumen yang tidak dikenal.                                                                                           |
    | `--session-id <id>`                             | Bawa kembali satu sesi berdasarkan ID-nya. Lihat [Resume sesi setelah menghentikan server](#resume-sessions-after-stopping-the-server). Tidak dapat digabungkan dengan `--continue`, `--spawn`, `--capacity`, atau `--create-session-in-dir`. Memerlukan Claude Code v2.1.200 atau lebih baru; versi sebelumnya menolak bendera sebagai argumen yang tidak dikenal.                                                                                                                                                |
    | `--spawn <mode>`                                | Bagaimana server membuat sesi.<br />• `same-dir` (default): semua sesi berbagi direktori kerja saat ini, sehingga dapat bertentangan jika mengedit file yang sama.<br />• `worktree`: setiap sesi sesuai permintaan mendapatkan [git worktree](/docs/id/worktrees) miliknya sendiri. Memerlukan repositori git.<br />• `session`: mode sesi tunggal. Melayani tepat satu sesi dan menolak koneksi tambahan. Atur saat startup saja.<br />Tekan `w` saat runtime untuk beralih antara `same-dir` dan `worktree`.         |
    | `--capacity <N>`                                | Jumlah maksimum sesi bersamaan. Default adalah 32. Tidak dapat digunakan dengan `--spawn=session`.                                                                                                                                                                                                                                                                                                                                                                                                                 |
    | `--[no-]create-session-in-dir`                  | Buat sesi sebelumnya di direktori saat ini ketika server dimulai, sehingga Anda memiliki tempat untuk mengetik segera. Dalam mode `worktree` sesi ini tetap berada di direktori saat ini sementara sesi sesuai permintaan mendapatkan worktree terisolasi. Aktif secara default. Jika Anda melewatkan `--no-create-session-in-dir` untuk memulai tanpa ada, Claude Code mengarsipkan sesi server ketika Anda menghentikannya, jadi tidak ada yang dapat [dilanjutkan](#resume-sessions-after-stopping-the-server). |
    | `--permission-mode <mode>`                      | Atur [mode izin](/docs/id/permission-modes) awal untuk sesi server, seperti `acceptEdits`. Menerima `manual` sebagai alias untuk `default`; mode yang tidak dikenali menghentikan server saat startup dan mencantumkan mode yang valid.                                                                                                                                                                                                                                                                                 |
    | `--debug-file <path>`                           | Tulis log debug ke file yang diberikan.                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
    | `--verbose`                                     | Tampilkan log koneksi dan sesi terperinci.                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
    | `--sandbox` / `--no-sandbox`                    | Aktifkan atau nonaktifkan [sandboxing](/docs/id/sandboxing) untuk isolasi sistem file dan jaringan. Dimatikan secara default.                                                                                                                                                                                                                                                                                                                                                                                           |

    Berikan bendera ini setelah `remote-control`.

    Jika Anda melewatkan bendera global `claude` sebelum `remote-control`, atau skrip wrapper menambahkan satu, Claude Code tidak membawa bendera ke sesi yang dibuat server. Claude Code membiarkan bendera melewati hanya ketika menjatuhkannya diketahui tidak mengubah apa yang dapat dilakukan sesi tersebut, seperti `--verbose` atau `--model`. Untuk bendera lainnya, seperti `--settings`, Claude Code [menolak untuk memulai](/docs/id/errors#not-carried-over-to-the-sessions-remote-control-starts) dan menyebutkan bendera yang akan dihapus. Sebelum v2.1.248, opsi apa pun sebelum `remote-control` membuat Claude Code menolak bendera setelahnya dengan kesalahan `unknown option`.

    Claude Code memeriksa kelayakan Remote Control sebelum mencetak bantuan, jadi `claude remote-control --help` mengembalikan kesalahan alih-alih daftar bendera ini ketika Anda tidak masuk dengan akun yang memenuhi syarat.
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

    Sampai Anda menerima konfirmasi satu kali Remote Control, dialog muncul sebelum `/remote-control` terhubung. Pilih **Enable Remote Control** untuk menerima dan terhubung. Jika Anda memilih **Never mind** atau menekan Esc, Claude Code tidak terhubung dan menanyakan lagi saat berikutnya Anda menjalankan `/remote-control`.

    Bendera `--verbose`, `--sandbox`, dan `--no-sandbox` tidak tersedia dengan perintah ini.
  </Tab>

  <Tab title="VS Code">
    Di [ekstensi VS Code Claude Code](/docs/id/vs-code), ketik `/remote-control` atau `/rc` di kotak prompt.

    ```text theme={null}
    /remote-control
    ```

    Saat Remote Control aktif, Claude Code menampilkan indikator **Remote Control** di footer kotak prompt. Setelah sesi terhubung, klik indikator untuk langsung ke sesi, atau temukan di daftar sesi di [claude.ai/code](https://claude.ai/code). Claude Code juga memposting URL sesi dalam percakapan. Untuk memutuskan sambungan, jalankan `/remote-control` lagi.

    Tidak seperti CLI, perintah VS Code tidak menerima argumen nama atau menampilkan kode QR. Judul sesi berasal dari riwayat percakapan Anda atau prompt pertama.
  </Tab>
</Tabs>

<h3 id="check-connection-status">
  Periksa status koneksi
</h3>

Dalam sesi interaktif, saat Remote Control terhubung, terminal menampilkan indikator `/rc active` yang menautkan ke sesi di claude.ai. Indikator disembunyikan ketika terminal terlalu sempit untuk menampilkannya. Untuk melihat URL sesi dan kode QR untuk [terhubung dari perangkat lain](#connect-from-another-device), jalankan `/remote-control` lagi untuk membuka panel status. Panel juga memungkinkan Anda memutuskan sambungan Remote Control sementara sesi lokal Anda terus berjalan.

<span id="session-ended-elsewhere" />Jika koneksi gagal dalam sesi interaktif, indikator berubah untuk menunjukkan kegagalan, dan Claude Code menampilkan alasan dalam notifikasi dan menambahkannya ke percakapan. Jalankan `/remote-control` untuk terhubung kembali, kecuali alasan mengatakan sesi berubah di tempat lain:

* **Koneksi lain mengambil alih sesi ini**: perangkat lain atau sesi Claude Code sekarang memilikinya. Jalankan `/remote-control` hanya jika Anda ingin mengambilnya kembali.
* **Sesi ini diakhiri atau diarsipkan dari perangkat atau aplikasi lain**: jalankan `/remote-control` hanya jika Anda menginginkannya kembali. Claude Code membuka kembali sesi yang diarsipkan.
* **Server tidak lagi melaporkan sesi ini**: mungkin telah dihapus dari perangkat atau aplikasi lain.

<h3 id="session-url-reminders">
  Pengingat URL sesi
</h3>

Saat Remote Control terhubung, Claude Code mengingatkan Anda tentang URL sesi ketika beralih ke ponsel atau browser membantu paling banyak, sehingga Anda tidak harus menemukan tautan di `/remote-control`. Pengingat muncul di atas kotak prompt pada salah satu momen ini:

* **Giliran panjang**: ketika giliran berjalan lebih lama dari ambang batas yang disetel server, Claude Code menampilkan notifikasi **Still working** dengan tautan **Check in from your phone**, sehingga Anda dapat mengikuti giliran dari ponsel atau browser Anda alih-alih menunggu di terminal. Claude Code menghapusnya ketika giliran berakhir.
* **Prompt izin berulang**: setelah Anda menjawab beberapa [prompt izin](/docs/id/permissions) dalam sesi, notifikasi **Approve tool calls from your phone** menampilkan URL sesi. Claude Code menghapusnya ketika giliran berikutnya dimulai.

Pengingat dapat muncul di sesi apa pun yang terhubung, termasuk sesi di mana Remote Control [terhubung secara otomatis](#enable-remote-control-for-all-sessions). Mereka tidak muncul setiap kali kondisi ini terjadi, dan masing-masing hanya muncul beberapa kali total di seluruh sesi. Anda tidak dapat mengonfigurasi atau mematikannya; masing-masing menghapus sendiri.

<h3 id="connect-from-another-device">
  Terhubung dari perangkat lain
</h3>

Setelah sesi Remote Control aktif, Anda memiliki beberapa cara untuk terhubung dari perangkat lain:

* **Buka URL sesi** di browser apa pun untuk langsung ke sesi di [claude.ai/code](https://claude.ai/code).
* **Pindai kode QR** yang ditampilkan bersama URL sesi untuk membukanya langsung di aplikasi Claude. Dengan `claude remote-control`, tekan spacebar untuk beralih tampilan kode QR.
* **Buka [claude.ai/code](https://claude.ai/code) atau aplikasi Claude** dan temukan sesi berdasarkan nama dalam daftar sesi. Di aplikasi mobile Claude, ketuk **Code** dalam navigasi untuk mencapai daftar sesi. Sesi Remote Control menampilkan ikon komputer dengan titik status hijau saat online.

Ketika Anda terhubung, perangkat menampilkan subagen dan alur kerja apa pun yang sudah dijalankan sesi di latar belakang. Hentikan salah satu dari perangkat, dan Claude Code menghentikan tugas itu di mesin Anda.

Judul sesi jarak jauh dipilih dalam urutan ini:

1. Nama yang Anda berikan ke `--name`, `--remote-control`, atau `/remote-control`
2. Judul yang Anda tetapkan dengan `/rename`
3. Pesan bermakna terakhir dalam riwayat percakapan yang ada
4. Nama yang dibuat secara otomatis seperti `myhost-graceful-unicorn`, di mana `myhost` adalah nama mesin Anda atau awalan yang Anda tetapkan dengan `--remote-control-session-name-prefix`

Jika Anda tidak menetapkan nama eksplisit, Claude Code memperbarui judul untuk mencerminkan prompt Anda setelah Anda mengirimnya. Claude Code mencocokkan judul yang dibuat secara otomatis dengan bahasa percakapan Anda, atau pengaturan [`language`](/docs/id/settings-reference#language) jika satu dikonfigurasi.

Ketika Anda mengganti nama sesi dari claude.ai atau aplikasi Claude, Claude Code juga memperbarui judul lokal yang ditampilkan di `claude --resume`. Claude Code menerapkan penggantian nama yang sama ke nama sesi yang ditampilkan di bilah prompt, dan dalam daftar `claude agents` ketika sesi [berjalan di latar belakang](/docs/id/agent-view). Sebelum v2.1.221, penggantian nama dari daftar sesi di claude.ai atau di aplikasi Claude hanya memperbarui judul, dan CLI menyimpan nama sesi sebelumnya; `/rename`, yang berjalan di CLI itu sendiri, menetapkan nama pada versi apa pun.

Jika Anda belum memiliki aplikasi Claude, jalankan `/mobile` di dalam Claude Code untuk menampilkan kode QR untuk [claude.ai/mobile](https://claude.ai/mobile), yang membuka app store yang tepat untuk ponsel Anda.

<h3 id="what-connected-devices-see">
  Apa yang dilihat perangkat yang terhubung
</h3>

Perangkat yang terhubung menampilkan percakapan di terminal Anda saat terjadi. Kasus-kasus ini melampaui pesan biasa:

* **Pemadatan dan `/clear`**: saat Claude Code [memadatkan percakapan](/docs/id/context-window#what-survives-compaction), perangkat yang terhubung menampilkan kemajuan dan kemudian di mana percakapan dipadatkan. Ketika Anda menjalankan `/clear`, percakapan direset pada perangkat yang terhubung juga.
* **Beralih percakapan dengan `/resume`**: perangkat yang terhubung tidak menerima judul percakapan yang dialihkan atau riwayat sebelumnya, tetapi pesan baru di kedua arah pergi ke dan dari percakapan mana pun yang terbuka di terminal Anda. Untuk bekerja pada percakapan asli dari perangkat lagi, jalankan `/resume` di terminal Anda dan beralih kembali ke itu.
* **Menarik sesi dengan `/teleport`**: ketika Anda menarik sesi [cloud](/docs/id/claude-code-on-the-web#from-cloud-to-terminal) ke terminal Anda dengan `/teleport`, perangkat yang terhubung tidak menerima riwayat percakapan yang ditarik sebelumnya. Pesan baru di kedua arah pergi ke dan dari percakapan yang ditarik, yang sekarang adalah yang terbuka di terminal Anda.
* **Pesan dari sesi lain Anda**: dengan [pesan lintas sesi](/docs/id/cross-session-messaging), koneksi yang sama membawa pesan antara sesi Anda sendiri di mesin yang berbeda dan dari [sesi cloud](/docs/id/claude-code-on-the-web) Anda, melalui server Anthropic seperti sisa lalu lintas Remote Control. [Pesan sesi di mesin lain](/docs/id/cross-session-messaging#message-sessions-on-other-machines) mencakup aturan pengiriman dan [Kontrol pesan masuk](/docs/id/cross-session-messaging#control-inbound-messages) mencakup kontrol masuk. Memerlukan Claude Code v2.1.224 atau lebih baru.
* **Prompt yang Anda kirim di tengah giliran**: ketika Anda mengirim prompt dari perangkat yang terhubung sebelum giliran saat ini berakhir, Claude Code mengantreannya dan menyimpannya dalam transkrip perangkat setelah giliran itu selesai.
* **Diff perubahan Anda**: ketika direktori sesi berada di repositori git, panel diff perangkat yang terhubung menampilkan perubahan Anda. Perangkat meminta diff melalui koneksi, dan Claude Code menghitungnya di mesin Anda. Pada cabang yang memiliki komit di depan cabang default repositori, panel menampilkan perubahan sejak cabang terpisah darinya, termasuk edit yang belum dikomit Anda. Pada cabang default itu sendiri, atau pada cabang yang tidak di depan itu, panel menampilkan hanya perubahan yang belum dikomit Anda. Sebelum v2.1.247, Claude Code melaporkan diff ke perangkat yang terhubung hanya dalam sesi yang dilayani oleh `claude remote-control`.
* **Model**: ketika Anda memilih [model](/docs/id/model-config) dari perangkat yang terhubung, Claude Code menjalankan sesi pada model itu. Pemilih `/model` terminal, `/status`, dan `/config` menampilkan model itu. Memerlukan Claude Code v2.1.238 atau lebih baru.
  * Model yang Anda pilih dari kontrol model perangkat berlaku hanya untuk sesi saat ini. Ketika Anda mengirim `/model <name>` dari perangkat ke sesi interaktif, Claude Code juga menetapkan default Anda untuk sesi baru.
  * Jika Anda mengirim nama yang Claude Code tidak kenali, seperti nama tampilan di mana ID model diharapkan, Claude Code [menolak pilihan](/docs/id/errors#model-is-not-a-recognized-model-id) dan sesi menyimpan model saat ini. Sebelum v2.1.260, Claude Code menyimpan pilihan yang tidak dikenali dari kontrol model perangkat, dan pesan berikutnya Anda gagal.
* **Tingkat upaya**: ketika Anda menetapkan [tingkat upaya](/docs/id/model-config#adjust-effort-level) dari perangkat yang terhubung, dengan `/effort` atau kontrol upaya perangkat, Claude Code menerapkannya ke sesi di mesin Anda, dan claude.ai/code menampilkan tingkat yang digunakan sesi. Jika Anda menyematkan tingkat dengan `CLAUDE_CODE_EFFORT_LEVEL`, sesi menyimpan tingkat itu, dan Claude Code menolak pilihan berbeda dari kontrol upaya. Memilih tingkat dari kontrol upaya memerlukan Claude Code v2.1.234 atau lebih baru di mesin Anda.
* **Terhubung kembali setelah kegagalan koneksi**: jalankan `/remote-control` untuk terhubung kembali. Jika pemadatan menulis ulang percakapan atau Anda beralih percakapan dengan `/resume` sementara itu, Claude Code mengarsipkan sesi server yang digunakannya alih-alih meninggalkannya dalam daftar sesi. Anda masih dapat menemukannya dengan [memfilter sesi yang diarsipkan](/docs/id/claude-code-on-the-web#archive-sessions). Beralih percakapan saat perangkat masih terhubung tidak mengarsipkan sesi.

<h3 id="enable-remote-control-for-all-sessions">
  Aktifkan Remote Control untuk semua sesi
</h3>

Remote Control hanya diaktifkan ketika Anda secara eksplisit menjalankan `claude remote-control`, `claude --remote-control`, atau `/remote-control`, kecuali auto-connect diaktifkan. Untuk mengaktifkannya secara otomatis untuk setiap sesi interaktif, jalankan `/config` di dalam Claude Code dan atur **Enable Remote Control for all sessions**. Toggle mengambil tiga nilai:

* **`true`**: terhubung secara otomatis ketika sesi interaktif dimulai.
* **`false`**: matikan auto-connect, meskipun `true` dari [pengaturan terkelola](/docs/id/managed-settings) mengungguli itu, karena Claude Code menyimpan pilihan ke pengaturan pengguna Anda. Sebuah `false` dalam pengaturan proyek atau lokal (`.claude/settings.json`, `.claude/settings.local.json`) mematikan auto-connect bahkan atas `true` yang dikelola.
* **`default`**: hapus pilihan Anda dan ikuti default admin organisasi Anda jika satu diatur, jika tidak default Claude Code saat ini.

Toggle yang sama muncul di luar CLI:

* **Aplikasi Desktop**: **Settings > Claude Code > Enable remote control by default**.
* **Ekstensi VS Code**: **Enable Remote Control for all sessions** di bagian Settings [menu perintah](/docs/id/vs-code#use-the-prompt-box). Memerlukan Claude Code v2.1.203 atau lebih baru.

Untuk mengaktifkan auto-connect dari file pengaturan, atur [`remoteControlAtStartup`](/docs/id/settings-reference#remotecontrolatstartup) ke `true` dalam `~/.claude/settings.json` pengguna Anda atau dalam [pengaturan terkelola](/docs/id/managed-settings). Dalam pengaturan proyek atau lokal (`.claude/settings.json`, `.claude/settings.local.json`), Claude Code menghormati `false` dan mematikan auto-connect untuk repositori itu, tetapi mengabaikan `true`, sehingga file yang diperiksa tidak dapat mengaktifkan Remote Control untuk semua orang yang membuka repositori.

Auto-connect masuk dengan akun claude.ai Anda sendiri, jadi sesi yang dimulainya hanya muncul di aplikasi Claude akun Anda sendiri dan tidak memberikan akses kepada siapa pun.

Dengan pengaturan ini aktif, setiap proses Claude Code interaktif mendaftarkan satu sesi jarak jauh. Jika Anda menjalankan beberapa instance, masing-masing mendapatkan sesi jarak jauh sendiri. Untuk menjalankan beberapa sesi bersamaan dari satu proses, gunakan [mode server](#start-a-remote-control-session) sebagai gantinya.

<h3 id="resume-sessions-after-stopping-the-server">
  Resume sesi setelah menghentikan server
</h3>

Ketika Anda menghentikan `claude remote-control` dengan Ctrl+C, sesi yang dilayaninya berhenti merespons dari ponsel atau browser Anda. Selama Anda tidak menjalankan `claude remote-control` lain di direktori yang sama dan tidak memulai yang ini dengan `--no-create-session-in-dir`, Claude Code tidak mengarsipkannya. Untuk membawanya kembali, jalankan salah satu perintah ini di direktori yang sama:

* **`claude remote-control`**: membawa kembali setiap sesi yang dilayani server.
* **`claude remote-control --continue`**: membawa kembali hanya sesi yang dimulai server, dan keluar ketika sesi itu berakhir. Jika direktori ini tidak memiliki catatan, Claude Code menggunakan yang terbaru dari worktree git lain repositori ini.
* **`claude remote-control --session-id <id>`**: membawa kembali hanya sesi yang ID-nya Anda lewatkan, dan keluar ketika sesi itu berakhir. ID adalah bagian dari URL sesi di claude.ai/code antara `/code/` dan `?` apa pun.

Perintah ini bekerja selama sekitar empat jam setelah server berhenti. Setelah itu, jalankan `claude remote-control` untuk memulai sesi baru. Jika Anda mengarsipkan sesi sementara itu, `--continue` dan `--session-id` membatalkan arsipnya pada Claude Code v2.1.228 atau lebih baru.

Untuk membawa kembali sesi yang Anda mulai dengan `claude --remote-control` atau `/remote-control`, lanjutkan percakapan dengan `claude --continue` atau `claude --resume`. Apakah Claude Code terhubung kembali, dan ke sesi mana, tergantung pada [catatan koneksi ulang](#resume-outcomes) percakapan.

Jika Anda melanjutkan percakapan di terminal kedua sementara yang pertama masih memiliki Remote Control aktif, Claude Code mencetak pemberitahuan di terminal kedua dan membiarkan Remote Control mati di sana alih-alih mengambil sesi dari yang pertama. Saat Remote Control tetap mati di sana, Claude di terminal itu tidak melihat [sesi Anda di mesin lain](/docs/id/cross-session-messaging#see-which-sessions-claude-can-reach), dan mereka tidak dapat menjangkaunya. Jalankan `/remote-control` di terminal kedua untuk memindahkan Remote Control ke sana.

Ketika Anda melanjutkan percakapan di Claude Desktop atau ekstensi IDE yang memiliki Remote Control aktif, Claude Code melampirkannya kembali ke sesi claude.ai yang ada alih-alih menambahkan yang baru ke daftar sesi.

<h2 id="connection-and-security">
  Koneksi dan keamanan
</h2>

Sesi Claude Code lokal Anda membuat permintaan HTTPS keluar saja dan tidak pernah membuka port masuk di mesin Anda. Ketika Anda memulai Remote Control, sesi tersebut mendaftarkan dengan API Anthropic dan polling untuk pekerjaan. Ketika Anda terhubung dari perangkat lain, server merutekan pesan antara klien web atau mobile dan sesi lokal Anda melalui koneksi streaming.

Semua lalu lintas berjalan melalui API Anthropic melalui TLS, keamanan transportasi yang sama seperti sesi Claude Code apa pun. Koneksi menggunakan beberapa kredensial berumur pendek, masing-masing dibatasi untuk satu tujuan dan kedaluwarsa secara independen. Ketika kredensial pendaftaran server `claude remote-control` kedaluwarsa, server mendaftarkan dengan API Anthropic lagi dan terus melayani sesinya.

Saat Remote Control terhubung, transkrip sesi, termasuk pesan Anda, respons Claude, dan aktivitas alat, disimpan di server Anthropic. Transkrip yang disimpan menjaga percakapan tetap sinkron di seluruh perangkat Anda dan memungkinkan sesi untuk terhubung kembali setelah gangguan jaringan. Eksekusi dan akses sistem file tetap berada di mesin Anda, dan transkrip yang disimpan dipertahankan sesuai dengan kebijakan [Penggunaan data](/docs/id/data-usage).

Untuk mematikan Remote Control sepenuhnya, gunakan pengaturan [`disableRemoteControl`](/docs/id/settings-reference#disableremotecontrol). Organisasi dengan persyaratan kepatuhan seperti Zero Data Retention tidak dapat mengaktifkan Remote Control.

<h2 id="trusted-devices">
  Perangkat Terpercaya
</h2>

<Note>
  Perangkat Terpercaya saat ini dalam beta. Fitur dan fungsionalitas dapat berkembang seiring pengalaman disempurnakan.

  Perangkat Terpercaya tersedia di paket Pro, Max, Tim, dan Enterprise dan dimatikan secara default. Di paket Tim dan Enterprise, Pemilik mengaktifkannya untuk organisasi. Di paket Pro dan Max, Anda mengaktifkan **Require trusted devices** sendiri di pengaturan Anda, di halaman Cowork atau Account.
</Note>

Perangkat Terpercaya memerlukan setiap anggota organisasi Anda, atau Anda sendiri di paket Pro atau Max, untuk memverifikasi perangkat mereka sebelum mereka dapat melihat atau mengarahkan sesi Remote Control dari claude.ai, aplikasi Claude mobile, atau Claude Desktop. Ini mengikat akses Remote Control ke perangkat yang dikenal dan autentikasi terbaru, bukan hanya akun yang masuk.

Ketika pengaturan aktif, berinteraksi dengan sesi Remote Control memerlukan keduanya:

* **Perangkat yang terdaftar**: setiap browser, ponsel, atau aplikasi desktop yang digunakan anggota untuk Remote Control mendaftarkan kredensialnya sendiri. Pendaftaran hanya ditawarkan segera setelah masuk penuh, jadi perangkat bergabung dengan daftar terpercaya sebagai bagian dari autentikasi nyata daripada diam-diam di latar belakang.
* **Masuk terbaru**: masuk anggota tidak boleh lebih dari 18 jam yang lalu. Alih-alih masuk lagi setiap hari, anggota mengonfirmasi kehadiran dengan Face ID, Touch ID, Windows Hello, atau passkey. Langkah step-up biometrik ini menyegarkan sesi segera.

Pemeriksaan biometrik berjalan di perangkat melalui sistem operasi atau browser, mekanisme yang sama dengan masuk passkey. Anthropic tidak pernah menerima atau menyimpan sidik jari, data wajah, atau informasi biometrik lainnya. Hanya kunci publik perangkat dan metadata dasar seperti nama tampilan, platform, dan waktu pendaftaran yang disimpan.

Pengaturan hanya berlaku untuk Remote Control. Obrolan Claude biasa, Claude Code di terminal, dan penggunaan API tidak terpengaruh.

<h3 id="enable-trusted-devices-for-your-organization">
  Aktifkan Perangkat Terpercaya untuk organisasi Tim atau Enterprise
</h3>

Pemilik mengaktifkan pengaturan dari pengaturan organisasi claude.ai.

<Steps>
  <Step title="Buka halaman Capabilities">
    Buka [**Organization settings > Capabilities > Remote sessions**](https://claude.ai/admin-settings/capabilities). Toggle **Require trusted devices** muncul di bagian itu.
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

<h2 id="remote-control-vs-cloud-sessions">
  Remote Control vs cloud sessions
</h2>

Remote Control dan [cloud sessions](/docs/id/claude-code-on-the-web) keduanya menggunakan antarmuka claude.ai/code. Perbedaan utamanya adalah di mana sesi berjalan: Remote Control dieksekusi di mesin Anda, sehingga MCP servers lokal, alat, dan konfigurasi proyek Anda tetap tersedia. Sesi cloud dieksekusi di infrastruktur cloud, dikelola oleh Anthropic secara default.

Gunakan Remote Control ketika Anda sedang dalam pekerjaan lokal dan ingin terus melanjutkan dari perangkat lain. Gunakan sesi cloud ketika Anda ingin memulai tugas tanpa pengaturan lokal apa pun, bekerja pada repo yang tidak Anda miliki klonnya, atau menjalankan beberapa tugas secara paralel.

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

* **Satu sesi jarak jauh per proses interaktif**: di luar mode server, setiap instans Claude Code mendukung satu sesi jarak jauh pada satu waktu. Gunakan [mode server](#start-a-remote-control-session) untuk menjalankan beberapa sesi bersamaan dari satu proses.
* **Proses lokal harus tetap berjalan**: Remote Control berjalan sebagai proses lokal. Jika Anda menutup terminal, keluar dari VS Code, atau menghentikan proses `claude` dengan cara lain, sesi akan offline sampai Anda [membawanya kembali](#resume-sessions-after-stopping-the-server). Kecuali Claude sedang menjalankan tugas, claude.ai dan aplikasi Claude menampilkan sesi sebagai offline dalam hitungan detik setelah proses keluar. Untuk menjaga sesi tetap berjalan di mesin jarak jauh setelah Anda memutuskan sambungan SSH, mulai di dalam `tmux` atau `screen`.
* **Sesi yang mogok dalam mode server**: jika sesi yang dilayani oleh `claude remote-control` mogok, kirimkan pesan kepadanya dari perangkat yang terhubung. Claude Code melayaninya lagi. Anda tidak perlu memulai ulang server. Memerlukan Claude Code v2.1.238 atau lebih baru.
* **Penolakan HTTP 403 pada sesi yang terhubung**: setelah sesi interaktif terhubung, Claude Code terus mencoba ulang hingga tiga menit ketika sesuatu antara mesin Anda dan server Anthropic menjawab dengan HTTP 403, yang dapat terjadi setelah perubahan VPN atau jaringan. Jika penolakan berlangsung lebih lama, Claude Code memutuskan sambungan, dan alasannya menunjukkan apa yang menolak: tepi jaringan, atau proxy, VPN, atau firewall di jaringan Anda sendiri.
* **Pemadaman jaringan yang berkepanjangan**: jika mesin Anda aktif tetapi tidak dapat menjangkau jaringan, apa yang Anda lakukan selanjutnya tergantung pada mode:
  * **Mode server**: Claude Code menyerah setelah kira-kira 10 menit dan proses `claude remote-control` keluar. Jalankan `claude remote-control` lagi untuk memulai sesi baru.
  * **Sesi interaktif**: terus bekerja secara lokal. Claude Code mencoba ulang selama pemadaman berlangsung dan terhubung kembali secara otomatis ketika jaringan kembali.
* **Detak jantung kehadiran gagal**: jika sesi interaktif terputus dengan `could not reach the Remote Control server for about 30 minutes`, jalankan `/remote-control` untuk terhubung kembali. Claude Code menampilkan pesan ini hanya ketika detak jantung kehadiran sesi telah gagal sementara sisa koneksi tetap aktif; sesi didaftarkan ulang selama kira-kira 30 menit sebelum terputus.
* **Dialog yang diteruskan kedaluwarsa**: Claude Code membuat prompt izin dan pertanyaan `AskUserQuestion` tetap terbuka sampai Anda menjawabnya. Ketika Claude Code meneruskan dialog jenis lain ke sesi jarak jauh, seperti prompt pilihan model yang ditampilkan setelah penolakan keselamatan, ia menunggu lima menit secara default, kemudian menutup dialog dan melanjutkan dengan default tanpa tindakan dialog. Atur [`dialogExpiry`](/docs/id/settings-reference#dialogexpiry) untuk menyesuaikan atau menonaktifkan batas waktu. Memerlukan Claude Code v2.1.224 atau lebih baru.
* **Prompt persetujuan kredit penggunaan Fable tidak diteruskan**: Claude Code menampilkan prompt persetujuan kredit penggunaan [Fable](/docs/id/model-config#fable-and-usage-credits) pertengahan sesi hanya di tempat sesi berjalan, bukan di perangkat Anda. Ketika sesi berjalan di terminal dan tidak ada yang menjawab sebelum Claude Code menutup prompt, giliran berakhir tanpa mengirim permintaan; lihat [The prompt to confirm went unanswered](/docs/id/errors#the-prompt-to-confirm-went-unanswered).
* **Beberapa perintah hanya lokal**: perintah yang hanya berjalan di antarmuka terminal, seperti `/plugin` atau `/resume`, hanya berfungsi dari CLI lokal, terlepas dari apakah Anda meneruskan argumen atau tidak. Berikut ini berfungsi dari mobile dan web:
  * Perintah keluaran teks: `/compact`, `/clear`, `/context`, `/usage`, `/exit`, `/usage-credits`, `/recap`, dan `/reload-plugins`. `/usage-credits` mencetak URL penagihan alih-alih membuka browser. `/reload-plugins` hanya berfungsi ketika sesi berjalan di terminal interaktif; sesi tanpa satu menolaknya.
  * `/model`, `/effort`, `/fast`, `/color`, dan `/rename`: teruskan nilai sebagai argumen, misalnya `/model sonnet` atau `/effort high`. Dari mobile dan web, `/model` dan `/effort` mengambil argumen sebagai pengganti pemilih terminal atau slider.
  * `/mcp`: dari aplikasi mobile, mengembalikan ringkasan teks status server alih-alih membuka pemilih. Di web, `/mcp` sendiri membuka direktori [konektor claude.ai](/docs/id/mcp#use-mcp-servers-from-claude-ai) alih-alih mengembalikan ringkasan. [Subperintah](/docs/id/commands#all-commands) `reconnect`, `enable`, dan `disable` berfungsi dari keduanya. Tidak seperti CLI lokal, `/mcp reconnect` tanpa nama server menghubungkan kembali setiap server yang telah gagal atau memerlukan autentikasi.
  * `/config`: dari aplikasi mobile, teruskan `key=value` untuk menetapkan pengaturan, atau jalankan tanpa argumen untuk membuat daftar kunci yang dapat Anda atur. Di web, `/config` membuka bagian Claude Code dari pengaturan Anda, dan mengabaikan teks setelah perintah.
  * Di Team dan Enterprise, `/usage-credits` dari mobile atau web tidak mengirim [permintaan kredit penggunaan ke admin Anda](/docs/id/costs#add-usage-credits-to-your-subscription). Pengiriman memerlukan konfirmasi yang hanya muncul di CLI interaktif, jadi perintah memberi tahu Anda untuk menjalankannya di sana. Sebelum v2.1.211, bentuk teks mengirim permintaan tanpa konfirmasi.
  * `/autocompact`, dari v2.1.221: teruskan ukuran jendela sebagai argumen, misalnya `/autocompact 500k`. Tanpa argumen, ia mencetak ukuran jendela saat ini sebagai teks alih-alih membuka dialog yang ditampilkan perintah dalam sesi terminal.
  * `/advisor`, dari v2.1.260: teruskan model sebagai argumen, misalnya `/advisor opus`, atau teruskan `off` untuk mematikan penasihat. Kedua bentuk berlaku untuk sesi saat ini saja dan membiarkan default yang disimpan tidak berubah. Tanpa argumen, ia mencetak penasihat saat ini sebagai teks alih-alih membuka pemilih.
  * `/output-style`, dari v2.1.269: teruskan nama gaya sebagai argumen, misalnya `/output-style concise`, atau jalankan tanpa argumen untuk membuat daftar gaya. Dari mobile dan web, Anda hanya dapat membuat daftar dan memilih [gaya bawaan](/docs/id/output-styles#built-in-output-styles). Untuk menggunakan [gaya kustom](/docs/id/output-styles#create-a-custom-output-style), pilih di sesi itu sendiri.

<h2 id="troubleshooting">
  Pemecahan Masalah
</h2>

<h3 id="remote-control-requires-a-claude-ai-subscription">
  "Remote Control memerlukan langganan claude.ai"
</h3>

Anda tidak masuk dengan akun claude.ai, atau kredensial lain mengambil alih login Anda. Pesan ini mengambil salah satu bentuk berikut:

* Tidak masuk, dari `/remote-control` atau `--remote-control`: `Remote Control requires a claude.ai subscription.` atau `/remote-control requires a claude.ai subscription.`
* Tidak masuk, dari `claude remote-control`: `You must be logged in to use Remote Control. Remote Control is only available with claude.ai subscriptions.`
* Masuk, tetapi kunci API atau token sedang digunakan: `Remote Control requires claude.ai subscription auth.` diikuti oleh kredensial yang digunakan, seperti `ANTHROPIC_API_KEY is set, so this session is using API-key auth`. Pengaturan `apiKeyHelper` dan `ANTHROPIC_AUTH_TOKEN` dinamai dengan cara yang sama.

Jalankan `claude auth login` dan pilih opsi claude.ai. Jika pesan menyebutkan `ANTHROPIC_API_KEY` atau `ANTHROPIC_AUTH_TOKEN`, hapus di mana pun diatur: lingkungan shell Anda atau blok `env` dari [file pengaturan](/docs/id/settings-reference#env). Jika menyebutkan `apiKeyHelper`, hapus pengaturan tersebut.

Sebelum v2.1.206, menjalankan `/remote-control` saat tidak masuk melaporkan `Unknown command: /remote-control` alih-alih pesan ini.

<h3 id="remote-control-requires-a-full-scope-login-token">
  "Remote Control memerlukan token login dengan cakupan penuh"
</h3>

Anda diautentikasi dengan token berumur panjang dari `claude setup-token` atau variabel lingkungan `CLAUDE_CODE_OAUTH_TOKEN`. Token ini hanya dapat membuat permintaan model, jadi token ini tidak dapat membuat sesi Remote Control. Jalankan `claude auth login` untuk autentikasi dengan token sesi cakupan penuh sebagai gantinya.

<h3 id="unable-to-determine-your-organization-for-remote-control-eligibility">
  "Tidak dapat menentukan organisasi Anda untuk kelayakan Remote Control"
</h3>

Informasi akun cache Anda sudah usang atau tidak lengkap. Jalankan `claude auth login` untuk menyegarkannya.

<h3 id="remote-control-isn’t-enabled-for-this-account">
  "Remote Control belum diaktifkan untuk akun ini"
</h3>

Claude Code memeriksa ketersediaan Remote Control untuk akun yang Anda masuki dan pemeriksaan kembali mati. Penyebab biasanya adalah hak akses cache yang sudah ketinggalan zaman setelah perubahan paket. Jalankan `claude auth logout` kemudian `claude auth login` untuk menyegarkannya, dan perbarui Claude Code jika Anda menggunakan versi lama.

Jalankan `claude doctor` untuk melihat pemeriksaan kelayakan individual mana yang gagal. Konflik variabel lingkungan, pemeriksaan yang tidak dapat dijangkau, dan pengaturan Remote Control organisasi Anda masing-masing menghasilkan pesan mereka sendiri, jadi kesalahan ini berarti pemeriksaan tingkat akun itu sendiri.

Sebelum v2.1.239, pesan ini berbunyi "Remote Control is not yet enabled for your account". Sebelum v2.1.154, variabel yang menonaktifkan evaluasi feature-flag, seperti `DISABLE_TELEMETRY` atau `DO_NOT_TRACK`, juga menghasilkan pesan ini; entri "Remote Control memerlukan evaluasi feature-flag" di bawah mencakup konfigurasi tersebut.

<h3 id="couldn’t-verify-remote-control-eligibility">
  "Tidak dapat memverifikasi kelayakan Remote Control"
</h3>

Claude Code tidak dapat menjangkau layanan feature-flag untuk memeriksa apakah Remote Control diaktifkan untuk akun Anda, biasanya karena Anda offline atau proxy memblokir permintaan. Coba lagi setelah Anda memiliki akses jaringan, atau jalankan `claude doctor` untuk detail. Pesan terkait "Tidak dapat memverifikasi kebijakan Remote Control organisasi Anda" berarti Claude Code tidak dapat membaca kebijakan tersebut, dan memiliki perbaikan yang sama. Kedua pesan ditambahkan di v2.1.178.

<h3 id="remote-control-requires-feature-flag-evaluation">
  "Remote Control memerlukan evaluasi feature-flag"
</h3>

Salah satu variabel ini diatur: [`DISABLE_TELEMETRY`, `DO_NOT_TRACK`, `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`, atau `DISABLE_GROWTHBOOK`](/docs/id/env-vars). Masing-masing menonaktifkan evaluasi feature-flag yang ketersediaan Remote Control bergantung padanya, dan pesan lengkap menyebutkan variabel yang Claude Code temukan. Batalkan pengaturan variabel tersebut di mana pun diatur, di lingkungan shell Anda atau di blok `env` dari file [`settings.json`](/docs/id/settings-reference#all-settings). Pada versi sebelum 2.1.154, konfigurasi yang sama menghasilkan "Remote Control is not yet enabled for your account" sebagai gantinya.

<h3 id="remote-control-is-only-available-when-using-claude-via-api-anthropic-com">
  "Remote Control hanya tersedia saat menggunakan Claude melalui api.anthropic.com"
</h3>

Sesi tidak berbicara langsung ke API Anthropic, jadi tidak ada backend claude.ai untuk dipasangkan. Ini terjadi di Amazon Bedrock, Google Cloud's Agent Platform, dan Microsoft Foundry. Ini juga terjadi ketika [`ANTHROPIC_BASE_URL`](/docs/id/env-vars) menunjuk ke host selain `api.anthropic.com`, seperti [gateway LLM](/docs/id/llm-gateway) atau proxy, bahkan jika Anda masuk dengan claude.ai. Sebelum v2.1.196, Claude Code tidak menampilkan pesan ini untuk `ANTHROPIC_BASE_URL` kustom. Lihat [referensi kesalahan](/docs/id/errors#remote-control-requires-the-anthropic-api) untuk daftar penyebab lengkap.

Pesan menyebutkan apa yang mengarahkan sesi jauh dari API Anthropic, seperti `CLAUDE_CODE_USE_BEDROCK` atau `ANTHROPIC_BASE_URL` kustom. Jika Anda memiliki login claude.ai yang memenuhi syarat, batalkan pengaturan variabel yang disebutkan, hapus dari kunci `env` di [pengaturan](/docs/id/settings) jika Anda menetapkannya di sana, dan mulai ulang sesi. Sebelum v2.1.219, pesan hanya berupa kalimat di header bagian ini, jadi pada versi yang lebih lama periksa lingkungan Anda sendiri untuk variabel penyedia seperti `CLAUDE_CODE_USE_BEDROCK` dan `CLAUDE_CODE_USE_VERTEX`, dan untuk `ANTHROPIC_BASE_URL`.

<h3 id="remote-control-is-disabled-by-your-organization’s-policy">
  "Remote Control dinonaktifkan oleh kebijakan organisasi Anda"
</h3>

Kebijakan memblokir Remote Control, atau Claude Code tidak dapat memuat kebijakan organisasi Anda di mesin ini dan terus menjaga Remote Control tetap mati untuk sementara. Periksa penyebab ini secara berurutan:

* **Kesalahan menyebutkan `disableRemoteControl`**: administrator IT Anda telah menonaktifkan Remote Control di perangkat ini melalui [pengaturan yang dikelola](/docs/id/managed-settings), terlepas dari toggle organisasi-lebar dan bagaimana Anda masuk.
* **Paket claude.ai Anda adalah Pro atau Max**: Claude Code masih masuk di bawah organisasi Team atau Enterprise dari login sebelumnya, jadi memeriksa kebijakan Remote Control organisasi tersebut. Jalankan `/status` untuk melihat paket dan organisasi mana yang digunakan sign-in Anda. Jalankan `claude auth logout` kemudian `claude auth login` untuk masuk lagi di bawah paket Anda saat ini.
* **Kebijakan organisasi tidak dimuat di mesin ini**: jalankan `claude doctor` dan baca baris `Organization policy`. Jika baris menunjukkan kebijakan tidak dimuat, itulah yang membuat Remote Control tetap mati. Sebelum v2.1.261, `claude doctor` tidak mencetak baris ini.
* **Pesan tidak mengatakan untuk menghubungi admin organisasi Anda**: organisasi Anda memiliki konfigurasi HIPAA yang tidak kompatibel dengan Remote Control, dan `/status` mencantumkan `HIPAA` di baris `Compliance` nya. Dalam keadaan ini toggle Remote Control panel admin berwarna abu-abu, jadi Pemilik tidak dapat mengubahnya di sana. Hubungi dukungan Anthropic untuk membahas opsi. Sebelum v2.1.267, kasus ini menampilkan "Remote Control isn't available for your organization due to its compliance policy" sebagai gantinya.
* **Jika tidak, Pemilik belum mengaktifkannya untuk organisasi Anda**: Remote Control dimatikan secara default di paket Team dan Enterprise. Pemilik dapat mengaktifkannya di [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code) dengan mengaktifkan toggle **Remote Control**. Toggle ini adalah pengaturan organisasi sisi server.

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

Token login yang sudah usang tidak menyebabkan kesalahan ini. Ketika API Anthropic menolak token yang disimpan, misalnya karena proses Claude Code lain sudah menyegarkannya, Claude Code menyegarkan token dan mencoba lagi dengan sendirinya. Sebelum v2.1.224, token yang sudah usang gagal startup Remote Control dengan pesan ini, jadi sesi yang diatur untuk [terhubung secara otomatis](#enable-remote-control-for-all-sessions) dapat gagal secara berkala saat peluncuran.

<h3 id="couldn’t-reconnect-to-your-remote-control-session">
  "Tidak dapat terhubung kembali ke sesi Remote Control Anda"
</h3>

Ketika Anda melanjutkan percakapan dengan `claude --resume` atau `claude --continue`, Claude Code terhubung kembali ke sesi Remote Control yang tercatat dalam percakapan tersebut. Pesan ini berarti koneksi ulang gagal karena alasan yang mungkin bersifat sementara, seperti gangguan jaringan atau kesalahan server, jadi Claude Code tidak dapat mengkonfirmasi apakah sesi jarak jauh masih ada.

Jalankan `/remote-control` untuk mencoba koneksi lagi, atau mulai sesi baru dengan `claude --remote-control` untuk membuat sesi Remote Control baru. Sesi lokal Anda terus berjalan tanpa Remote Control untuk sementara.

<span id="resume-outcomes" />Ketika Anda melanjutkan, Anda juga dapat mendapatkan salah satu hasil ini alih-alih pesan ini:

* **Server melaporkan sesi yang tercatat hilang, atau catatan koneksi ulang menyebutkan akun yang berbeda**: Claude Code mengikuti apa yang dikatakan catatan koneksi ulang percakapan:
  * **Catatan menyebutkan akun yang Anda masuki**: Claude Code memulai sesi pengganti dengan nama yang dibuat secara otomatis dan meninggalkan pesan sebelumnya dari percakapan. Anda mendapatkan ini setelah Anda menghapus sesi dari claude.ai atau aplikasi Claude, misalnya.
  * **Catatan menyebutkan akun yang berbeda**: Claude Code memulai sesi baru tanpa pesan sebelumnya dari percakapan dan tanpa menampilkan pesan, terlepas dari apakah sesi yang tercatat masih ada.
  * **Catatan tidak mengatakan akun mana yang memiliki sesi, atau Claude Code tidak dapat membaca sign-in yang disimpan Anda**: Claude Code menampilkan [`Previous session is unavailable — run /remote-control to start a new one`](#previous-session-is-unavailable) alih-alih pesan ini, tidak memulai apa pun, dan menghapus catatan dari percakapan.
* **Anda mematikan Remote Control sebelum melanjutkan**: kecuali aplikasi yang menghosting Claude Code telah memberitahu bahwa aplikasi memiliki sesi claude.ai, Claude Code menghapus catatan koneksi ulang ketika Anda mematikan Remote Control dari [panel status](#check-connection-status) CLI, ekstensi VS Code, atau host yang dibangun di [Agent SDK](/docs/id/agent-sdk/overview), jadi tidak terhubung kembali. Ketika aplikasi pemilik mematikannya, Claude Code menyimpan catatan dan terhubung kembali.
* **Claude Code lain di mesin ini masih memiliki sesi**: Anda melihat pemberitahuan yang dimulai dengan `Remote Control not started here`, dan Claude Code [meninggalkan Remote Control mati di sesi yang dilanjutkan](#resume-sessions-after-stopping-the-server). Jalankan `/remote-control` di sana untuk memindahkannya.

<span id="reconnect-history" />Sebelum v2.1.232, Claude Code merespons berbeda ketika server melaporkan sesi yang tercatat hilang. Dari v2.1.227 hingga v2.1.231, Claude Code menolak untuk memulai pengganti bahkan ketika catatan cocok dengan akun Anda. Melalui v2.1.226, Claude Code memulai pengganti terlepas dari apakah catatan cocok dengan akun Anda, dan di v2.1.224 hingga v2.1.226 membuatnya di bawah akun yang masuk di mesin itu, tidak pernah akun lain, tanpa mengunggah pesan sebelumnya dari percakapan ke dalamnya. Sebelum v2.1.200, Claude Code membuat sesi baru setelah kegagalan koneksi ulang apa pun.

<h3 id="previous-session-is-unavailable">
  "Previous session is unavailable — run /remote-control to start a new one"
</h3>

Claude Code tidak dapat membawa kembali sesi Remote Control sebelumnya dan berhenti alih-alih memulai sesi baru dengan sendirinya. Anda dapat melihat pesan ini setelah Anda melanjutkan percakapan dengan `claude --resume` atau `claude --continue`, atau setelah Claude Code [terhubung kembali dengan sendirinya setelah putus](/docs/id/errors#remote-control-couldnt-refresh-your-login).

Jalankan `/remote-control` untuk memulai sesi Remote Control baru di bawah login saat ini; sesi lokal Anda terus berjalan tanpa Remote Control untuk sementara. Pesan terkait `Remote Control could not verify the signed-in account — run /remote-control to reconnect` memiliki perbaikan yang sama; Claude Code menampilkannya ketika akun yang masuk berubah atau tidak dapat dibaca antara memvalidasinya dan terhubung kembali. Jika Anda menjalankan `/remote-control` setelah `Previous session is unavailable` tanpa memulai ulang Claude Code terlebih dahulu, Claude Code meninggalkan pesan sebelumnya dari percakapan di luar sesi baru.

Saat melanjutkan, Claude Code [memulai sesi baru sebagai penggantinya](#resume-outcomes) hanya jika catatan koneksi ulang percakapan menyebutkan akun yang memiliki sesi, karena server melaporkan sesi yang Anda hapus dan sesi yang dimiliki oleh akun lain dengan cara yang sama. Claude Code sebelum v2.1.227 tidak mencatat akun itu, dan Claude Code tidak dapat memeriksa catatan ketika tidak dapat membaca sign-in yang disimpan Anda. Claude Code sebelum v2.1.232 menampilkan `Remote Control could not resume the previous session under the current login — run /remote-control to start fresh` sebagai gantinya, di [serangkaian kasus yang berbeda](#reconnect-history).

<h3 id="remote-control-got-an-unexpected-server-response">
  "Remote Control got an unexpected server response"
</h3>

Server Remote Control menerima permintaan tetapi membalas dalam bentuk yang versi Claude Code ini tidak dapat baca, saat membuat sesi jarak jauh atau mengambil kredensialnya. Mencoba lagi pada versi yang sama gagal dengan cara yang sama. Jalankan `claude update`, kemudian jalankan `/remote-control` untuk terhubung kembali. Pesan ini ditambahkan di v2.1.225.

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

Claude Code menawarkan beberapa cara untuk bekerja ketika Anda tidak berada di terminal. Mereka berbeda dalam hal apa yang memicu pekerjaan, di mana Claude berjalan, dan berapa banyak yang perlu Anda atur.

|                                                          | Pemicu                                                                                                       | Claude berjalan di                                                                             | Pengaturan                                                                                                                            | Terbaik untuk                                                          |
| :------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------ | :--------------------------------------------------------------------- |
| [Dispatch](/docs/id/desktop#sessions-from-dispatch)           | Kirim pesan tugas dari aplikasi mobile Claude                                                                | Mesin Anda (Desktop)                                                                           | [Pasangkan aplikasi mobile dengan Desktop](https://support.claude.com/en/articles/13947068)                                           | Mendelegasikan pekerjaan saat Anda pergi, pengaturan minimal           |
| [Remote Control](/docs/id/remote-control)                     | Jalankan sesi yang sedang berjalan dari [claude.ai/code](https://claude.ai/code) atau aplikasi mobile Claude | Mesin Anda (CLI atau VS Code)                                                                  | Jalankan `claude remote-control`                                                                                                      | Mengarahkan pekerjaan yang sedang berlangsung dari perangkat lain      |
| [Channels](/docs/id/channels)                                 | Dorong acara dari aplikasi chat seperti Telegram atau Discord, atau server Anda sendiri                      | Mesin Anda (CLI)                                                                               | [Instal plugin channel](/docs/id/channels#quickstart) atau [buat milik Anda sendiri](/docs/id/channels-reference)                               | Bereaksi terhadap acara eksternal seperti kegagalan CI atau pesan chat |
| [Slack](/docs/id/slack)                                       | Sebutkan `@Claude` di saluran tim                                                                            | Cloud Anthropic                                                                                | [Instal aplikasi Slack](/docs/id/slack#setting-up-claude-code-in-slack) dengan [Claude Code di web](/docs/id/claude-code-on-the-web) diaktifkan | PR dan ulasan dari chat tim                                            |
| [Self-hosted environments](/docs/id/self-hosted-environments) | Mulai [sesi cloud](/docs/id/claude-code-on-the-web) dan pilih lingkungan organisasi Anda                          | Infrastruktur organisasi Anda                                                                  | [Terapkan runner](/docs/id/self-hosted-environments-quickstart), pada paket Team dan Enterprise                                            | Sesi cloud yang harus berjalan di dalam jaringan Anda                  |
| [Scheduled tasks](/docs/id/scheduled-tasks)                   | Atur jadwal                                                                                                  | [CLI](/docs/id/scheduled-tasks), [Desktop](/docs/id/desktop-scheduled-tasks), atau [cloud](/docs/id/routines) | Pilih frekuensi                                                                                                                       | Otomasi berulang seperti ulasan harian                                 |

<h2 id="related-resources">
  Sumber daya terkait
</h2>

* [Claude Code di web](/docs/id/claude-code-on-the-web): jalankan sesi di cloud alih-alih di mesin Anda, dikonfigurasi melalui [lingkungan cloud](/docs/id/cloud-environments)
* [Pesan lintas sesi](/docs/id/cross-session-messaging): biarkan Claude mengirim pesan ke sesi Anda di mesin lain atau di [sesi cloud](/docs/id/claude-code-on-the-web)
* [Channels](/docs/id/channels): teruskan Telegram, Discord, atau iMessage ke sesi sehingga Claude bereaksi terhadap pesan saat Anda pergi
* [Dispatch](/docs/id/desktop#sessions-from-dispatch): kirim pesan tugas dari ponsel Anda dan dapat menjalankan sesi Desktop untuk menanganinya
* [Autentikasi](/docs/id/authentication): atur `/login` dan kelola kredensial untuk claude.ai
* [Referensi CLI](/docs/id/cli-reference): daftar lengkap bendera dan perintah termasuk `claude remote-control`
* [Keamanan](/docs/id/security): bagaimana sesi Remote Control sesuai dengan model keamanan Claude Code
* [Penggunaan data](/docs/id/data-usage): data apa yang mengalir melalui API Anthropic selama sesi lokal, Remote Control, dan cloud
