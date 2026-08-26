> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Aplikasi desktop

> Dapatkan lebih banyak dari Claude Code Desktop: sesi paralel dengan isolasi Git, tata letak pane drag-and-drop, terminal terintegrasi dan editor file, side chats, computer use, Dispatch sessions dari ponsel Anda, tinjauan diff visual, pratinjau aplikasi, pemantauan PR, konektor, dan konfigurasi enterprise.

Aplikasi Claude Desktop memiliki tiga tab: **Chat** untuk percakapan, **Cowork** untuk [Dispatch dan pekerjaan agentic yang lebih panjang](https://claude.com/product/cowork), dan **Code** untuk pengembangan perangkat lunak. Halaman ini adalah referensi untuk tab Code.

<CardGroup cols={3}>
  <Card title="Download for macOS" icon="apple" href="https://claude.ai/api/desktop/darwin/universal/dmg/latest/redirect?utm_source=claude_code&utm_medium=docs">
    Universal build for Intel and Apple Silicon
  </Card>

  <Card title="Download for Windows" icon="windows" href="https://claude.ai/api/desktop/win32/x64/setup/latest/redirect?utm_source=claude_code&utm_medium=docs">
    For x64 processors
  </Card>

  <Card title="Get Claude for Linux (beta)" icon="linux" href="/docs/en/desktop-linux">
    apt or .deb for Ubuntu and Debian
  </Card>
</CardGroup>

For Windows ARM64, download the [ARM64 installer](https://claude.ai/api/desktop/win32/arm64/setup/latest/redirect?utm_source=claude_code\&utm_medium=docs). On Linux, install with apt; see [Claude Desktop on Linux](/docs/en/desktop-linux).

Setelah menginstal, luncurkan Claude, masuk, dan klik tab **Code**. Pertama kali Anda membukanya di Windows, Anda perlu menginstal [Git for Windows](https://git-scm.com/downloads/win); mulai ulang aplikasi setelah menginstalnya. Untuk panduan sesi pertama Anda, lihat [panduan Memulai](/docs/id/desktop-quickstart).

Di tab Code, setiap percakapan adalah sebuah **sesi**: ia memiliki riwayat chat sendiri, folder proyek, dan perubahan kode, independen dari sesi lainnya. Bilah sisi mencantumkan sesi Anda dan memungkinkan Anda menjalankan beberapa secara paralel. Dalam sesi Anda dapat:

* [Meninjau dan mengomentari diff](#review-changes-with-diff-view), kemudian [memantau PR yang dihasilkan melalui CI](#monitor-pull-request-status)
* [Pratinjau aplikasi yang berjalan](#preview-your-app) di pane Browser sementara Claude memverifikasi perubahan miliknya sendiri, dan [membuka situs eksternal](#browse-external-sites) berdampingannya
* [Mengatur pane](#arrange-your-workspace) untuk chat, diff, browser, terminal, dan editor file berdampingan
* Mengajukan [pertanyaan sampingan](#ask-a-side-question-without-derailing-the-session) yang menggunakan konteks sesi tanpa mengganggu alurnya
* [Menghubungkan alat eksternal](#connect-external-tools) seperti GitHub, Slack, dan Linear
* Biarkan Claude [membuka aplikasi dan mengontrol layar Anda](#let-claude-use-your-computer)
* Jalankan di mesin Anda, di [cloud](#run-long-running-tasks-remotely), atau melalui [SSH](#ssh-sessions)

Untuk [pekerjaan berulang terjadwal](/docs/id/desktop-scheduled-tasks), [pintasan keyboard](#keyboard-shortcuts), atau [mengirim tugas dari ponsel Anda](#sessions-from-dispatch), lihat halaman dan bagian yang ditautkan. Jika Anda sudah menggunakan CLI berbasis terminal, lihat [perbandingan CLI](#coming-from-the-cli) untuk apa yang berlanjut.

<h2 id="start-a-session">
  Mulai sesi
</h2>

Sebelum Anda mengirim pesan pertama, konfigurasikan empat hal di area prompt:

* **Environment**: pilih di mana Claude berjalan. Pilih **Local** untuk mesin Anda, **Remote** untuk sesi cloud yang dihosting Anthropic, [**koneksi SSH**](#ssh-sessions) untuk mesin jarak jauh yang Anda kelola, atau di Windows [**distribusi WSL**](/docs/id/desktop-wsl). Lihat [konfigurasi lingkungan](#environment-configuration).
* **Project folder**: pilih folder atau repositori tempat Claude bekerja. Untuk sesi jarak jauh, Anda dapat menambahkan [beberapa repositori](#run-long-running-tasks-remotely).
* **Model**: pilih [model](/docs/id/model-config#available-models) dari dropdown di sebelah tombol kirim. Anda dapat mengubah ini selama sesi.
* **Permission mode**: pilih berapa banyak otonomi yang dimiliki Claude dari [pemilih mode](#choose-a-permission-mode). Anda dapat mengubah ini selama sesi.

Ketik tugas Anda dan tekan **Enter** untuk memulai. Setiap sesi melacak konteksnya sendiri dan perubahan secara independen.

<h2 id="work-with-code">
  Bekerja dengan kode
</h2>

Berikan Claude konteks yang tepat, kontrol berapa banyak yang dilakukannya sendiri, dan tinjau apa yang diubahnya.

<h3 id="use-the-prompt-box">
  Gunakan kotak prompt
</h3>

Ketik apa yang ingin Anda lakukan Claude dan tekan **Enter** untuk mengirim. Claude membaca file proyek Anda, membuat perubahan, dan menjalankan perintah berdasarkan [permission mode](#choose-a-permission-mode) Anda. Anda dapat mengarahkan ulang Claude kapan saja: klik tombol stop untuk menghentikan segera, atau ketik koreksi dan tekan **Enter** untuk mengirimnya tanpa menghentikan tindakan yang sedang berjalan. Claude membaca koreksi segera setelah tindakan saat ini selesai dan menyesuaikan sebelum langkah berikutnya.

Tombol **+** di sebelah kotak prompt memberi Anda akses ke lampiran file, [skills](#use-skills), [konektor](#connect-external-tools), dan [plugins](#install-plugins).

<h3 id="add-files-and-context-to-prompts">
  Tambahkan file dan konteks ke prompt
</h3>

Kotak prompt mendukung dua cara untuk membawa konteks eksternal:

* **@mention files**: ketik `@` diikuti dengan nama file untuk menambahkan file ke konteks percakapan. Claude kemudian dapat membaca dan mereferensikan file tersebut. @mention tidak tersedia di sesi cloud atau WSL.
* **Attach files**: lampirkan gambar, PDF, dan file lainnya ke prompt Anda menggunakan tombol lampiran, atau seret dan lepas file langsung ke prompt. Ini berguna untuk berbagi tangkapan layar bug, mockup desain, atau dokumen referensi.

<h3 id="choose-a-permission-mode">
  Pilih permission mode
</h3>

Permission modes mengontrol berapa banyak otonomi yang dimiliki Claude selama sesi: apakah itu meminta izin sebelum mengedit file, menjalankan perintah, atau keduanya. Anda dapat beralih mode kapan saja menggunakan pemilih mode di sebelah tombol kirim. Mulai dengan Manual untuk melihat dengan tepat apa yang dilakukan Claude, kemudian pindah ke Accept edits atau Plan saat Anda merasa nyaman.

Untuk menetapkan mode default untuk sesi lokal baru, tambahkan `permissions.defaultMode` ke [file pengaturan](/docs/id/settings#settings-files) Anda. Aplikasi desktop membaca file pengaturan yang sama dengan CLI. Mode yang Anda pilih di pemilih diingat per folder dan mengambil alih `defaultMode` untuk folder itu, kecuali Plan, yang berlaku hanya untuk sesi saat ini.

| Mode                   | Settings key        | Behavior                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| ---------------------- | ------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Manual**             | `default`           | Claude meminta izin sebelum mengedit file atau menjalankan perintah. Anda melihat diff dan dapat menerima atau menolak setiap perubahan. Direkomendasikan untuk pengguna baru.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| **Accept edits**       | `acceptEdits`       | Claude secara otomatis menerima edit file dan perintah filesystem umum seperti `mkdir`, `touch`, dan `mv`, tetapi masih meminta izin sebelum menjalankan perintah terminal lainnya. Gunakan ini ketika Anda mempercayai perubahan file dan menginginkan iterasi yang lebih cepat.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| **Plan**               | `plan`              | Claude membaca file dan menjalankan perintah untuk menjelajahi, kemudian mengusulkan rencana tanpa mengedit kode sumber Anda. Bagus untuk tugas kompleks di mana Anda ingin meninjau pendekatan terlebih dahulu.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| **Auto**               | `auto`              | Claude mengeksekusi semua tindakan dengan pemeriksaan keamanan latar belakang yang memverifikasi keselarasan dengan permintaan Anda. Mengurangi prompt izin sambil mempertahankan pengawasan. Muncul ketika akun Anda memenuhi [availability requirements](#auto-mode-availability) di bawah; tidak ada toggle Settings terpisah untuk itu.                                                                                                                                                                                                                                                                                                                                                                                                                                |
| **Bypass permissions** | `bypassPermissions` | Claude berjalan tanpa prompt izin apa pun, kecuali yang dipaksa oleh [ask rules](/docs/id/permissions#manage-permissions) eksplisit, connector tools [organisasi Anda atur ke `ask`](/docs/id/mcp#organization-controls-on-connector-tools), MCP tools yang ditandai [`requiresUserInteraction`](/docs/id/mcp#require-approval-for-a-specific-tool), atau safety classifiers ketika Claude [bertindak di situs eksternal](#browse-external-sites); setara dengan `--dangerously-skip-permissions` di CLI. Di paket Pro dan Max, aktifkan di Settings → Claude Code Anda di bawah "Allow bypass permissions mode"; di paket Team dan Enterprise tidak ada toggle Settings, dan kebijakan organisasi mengontrolnya sebagai gantinya. Hanya gunakan ini di kontainer atau VM yang disandbox. |

Versi sebelumnya dari tab Code memberi label pada mode ini sebagai Ask permissions, Auto accept edits, dan Plan mode.

Permission mode `dontAsk` hanya tersedia di [CLI](/docs/id/permission-modes#allow-only-pre-approved-tools-with-dontask-mode).

<span id="auto-mode-availability" />

Auto mode tersedia untuk semua pengguna di Anthropic API dan memerlukan Claude Opus 4.6 atau lebih baru, atau Sonnet 4.6 atau lebih baru. Administrator organisasi dapat menonaktifkan auto mode dengan kunci `disableAutoMode` di [managed settings](#managed-settings).

Dalam penyebaran Enterprise yang merutekan Desktop ke Google Cloud's Agent Platform, auto mode [tersedia secara default](/docs/id/permission-modes#enable-auto-mode-on-bedrock-agent-platform-or-foundry), dan hanya Claude Sonnet 5, Opus 4.7, dan Opus 4.8 yang didukung di sana. Sebelum Claude Code v2.1.207, penyebaran Enterprise di Google Cloud's Agent Platform harus menetapkan `CLAUDE_CODE_ENABLE_AUTO_MODE` untuk mengaktifkan auto mode.

<Tip title="Best practice">
  Mulai tugas kompleks di Plan sehingga Claude memetakan pendekatan sebelum membuat perubahan. Setelah Anda menyetujui rencana, beralih ke Accept edits atau Manual untuk menjalankannya. Lihat [explore first, then plan, then code](/docs/id/best-practices#explore-first-then-plan-then-code) untuk informasi lebih lanjut tentang alur kerja ini.
</Tip>

Sesi cloud mendukung Accept edits, Plan, dan Auto. Accept edits sesuai dengan mode `default`: sesi cloud secara otomatis menyetujui edit file, jadi pemilih menampilkan Accept edits alih-alih Manual. Bypass permissions tidak tersedia karena lingkungan cloud sudah disandbox.

Admin enterprise dapat membatasi permission modes mana yang tersedia. Lihat [enterprise configuration](#enterprise-configuration) untuk detail.

<h3 id="preview-your-app">
  Pratinjau aplikasi Anda
</h3>

Claude dapat memulai dev server dan membuka browser tertanam untuk memverifikasi perubahannya. Ini berfungsi untuk aplikasi web frontend serta server backend: Claude dapat menguji endpoint API, melihat log server, dan mengulangi masalah yang ditemukannya. Dalam kebanyakan kasus, Claude memulai server secara otomatis setelah mengedit file proyek. Anda juga dapat meminta Claude untuk pratinjau kapan saja. Secara default, Claude [auto-verifies](#auto-verify-changes) perubahan setelah setiap edit.

Pane Browser juga dapat membuka file HTML statis, PDF, gambar, dan video dari proyek Anda. Klik path HTML, PDF, gambar, atau video di chat untuk membukanya di sana.

Dari pane Browser, Anda dapat:

* Berinteraksi dengan aplikasi yang sedang berjalan langsung di pane Browser
* Tonton Claude memverifikasi perubahannya sendiri secara otomatis: mengambil tangkapan layar, memeriksa DOM, mengklik elemen, mengisi formulir, dan memperbaiki masalah yang ditemukannya
* Mulai atau hentikan server dari dropdown server di toolbar sesi
* Pertahankan cookie dan penyimpanan lokal di seluruh restart server dengan memilih **Persist sessions** di dropdown, sehingga Anda tidak perlu masuk kembali selama pengembangan
* Edit konfigurasi server atau hentikan semua server sekaligus

Claude membuat konfigurasi server awal berdasarkan proyek Anda. Jika aplikasi Anda menggunakan perintah dev kustom, edit `.claude/launch.json` agar sesuai dengan setup Anda. Lihat [Configure preview servers](#configure-preview-servers) untuk referensi lengkap.

Untuk menghapus data sesi yang disimpan, atau untuk menonaktifkan Browser sepenuhnya, gunakan toggle di Settings → Claude Code.

<h3 id="browse-external-sites">
  Jelajahi situs eksternal
</h3>

Pane Browser adalah browser bertab, jadi Anda dapat membuka dokumentasi, pelacak masalah, atau situs lainnya di sebelah aplikasi yang sedang berjalan. Untuk membuka Browser, tekan **Cmd+Shift+B** di macOS atau **Ctrl+Shift+B** di Windows, atau pilih dari menu **Views**. Ketika Anda mengklik tautan eksternal di chat, pemilih menawarkan **Open in app** untuk menggunakan pane Browser atau **Default browser** untuk menggunakan browser Anda sendiri; **Cmd**-klik di macOS atau **Ctrl**-klik di Windows membuka tautan di browser sistem Anda secara langsung. Anda dapat masuk ke situs di pane, termasuk alur sign-in popup seperti Google OAuth.

Claude dapat membaca dan berinteraksi dengan halaman eksternal menggunakan alat yang sama yang digunakan untuk [memverifikasi aplikasi Anda](#preview-your-app), dengan dua pemeriksaan keamanan tambahan:

* Safety classifiers meninjau tindakan penulisan Claude di halaman eksternal, seperti mengklik dan mengetik, di setiap permission mode. Ini adalah classifier yang sama yang digunakan [auto mode](#choose-a-permission-mode), dan ketika mereka menandai tindakan, Anda mendapatkan prompt izin terlepas dari mode.
* Di permission modes selain Auto dan Bypass permissions, pemeriksaan domain allowlist juga berlaku sebelum Claude menavigasi ke situs baru.

<h4 id="approve-claude’s-actions-on-a-site">
  Setujui tindakan Claude di situs
</h4>

Pertama kali Claude bertindak di situs eksternal, kartu izin muncul dan Claude menunggu pilihan Anda: **Allow once**, **Always allow**, atau **Deny**. **Allow once** menyetujui tindakan tanpa menyimpan apa pun. **Always allow** menyimpan persetujuan untuk situs itu di perangkat Anda, dan Anda dapat mencabut di Settings. Setiap situs memerlukan persetujuannya sendiri, termasuk subdomain. Server dev lokal Anda dan file proyek tidak memerlukan persetujuan, jadi [auto-verify](#auto-verify-changes) terus bekerja tanpa prompt.

Bahkan di situs yang disetujui, Claude tidak akan membeli item, membuat akun, atau melewati CAPTCHA tanpa input Anda. Browsing di pane Browser menggunakan model keamanan yang sama dengan [Claude in Chrome extension](/docs/id/chrome). Lihat [Using Claude in Chrome safely](https://support.claude.com/en/articles/12902428-using-claude-in-chrome-safely) untuk cara Claude menangani situs sensitif dan tindakan berisiko.

<h4 id="choose-between-the-browser-and-the-chrome-extension">
  Pilih antara Browser dan Chrome extension
</h4>

Pane Browser menggunakan profil browser yang bersih, terpisah dari browser pribadi Anda, tanpa login atau riwayat yang disimpan. Gunakan untuk membangun dan menguji aplikasi Anda dan untuk situs yang tidak memerlukan identitas Anda. Ketika Anda ingin Claude bertindak sebagai Anda dalam sesi login Anda, gunakan [Claude in Chrome extension](/docs/id/chrome) sebagai gantinya, yang berbagi status login browser Anda.

<h4 id="restrict-external-browsing-for-your-organization">
  Batasi browsing eksternal untuk organisasi Anda
</h4>

Browser mengikuti [site allowlist dan blocklist controls](https://support.claude.com/en/articles/13065128-claude-in-chrome-admin-controls) yang sama dengan Claude in Chrome extension. Jika organisasi Anda sudah mengonfigurasi daftar tersebut untuk extension, Browser menghormatinya secara otomatis. Administrator juga dapat menonaktifkan alat Claude di halaman eksternal dengan [managed setting](#managed-settings) `browserExternalPageTools`. Dengan alat dinonaktifkan, pengguna masih dapat menavigasi ke situs eksternal; alat Claude tidak dapat membaca atau bertindak pada mereka.

Untuk menonaktifkan browsing eksternal sepenuhnya, atur [managed setting](#managed-settings) `disableBrowserExternalNavigation` ke `true`. Ini memblokir semua navigasi eksternal di Browser, termasuk situs di allowlist organisasi Anda; server dev localhost dan pratinjau file tetap berfungsi. Gunakan `browserExternalPageTools` untuk membiarkan pengguna terus menjelajahi situs eksternal tanpa alat Claude, dan `disableBrowserExternalNavigation` untuk memblokir situs eksternal untuk pengguna dan Claude.

<h3 id="review-changes-with-diff-view">
  Tinjau perubahan dengan diff view
</h3>

Setelah Claude membuat perubahan pada kode Anda, diff view memungkinkan Anda meninjau modifikasi file demi file sebelum membuat pull request.

Ketika Claude mengubah file, indikator statistik diff muncul menunjukkan jumlah baris yang ditambahkan dan dihapus, seperti `+12 -1`. Klik indikator ini untuk membuka diff viewer, yang menampilkan daftar file di sebelah kiri dan perubahan untuk setiap file di sebelah kanan.

Untuk mengomentari baris tertentu, klik baris apa pun di diff untuk membuka kotak komentar. Ketik umpan balik Anda dan tekan **Enter** untuk menambahkan komentar. Setelah menambahkan komentar ke beberapa baris, kirimkan semua komentar sekaligus:

* **macOS**: tekan **Cmd+Enter**
* **Windows**: tekan **Ctrl+Enter**

Claude membaca komentar Anda dan membuat perubahan yang diminta, yang muncul sebagai diff baru yang dapat Anda tinjau.

<h3 id="review-your-code">
  Tinjau kode Anda
</h3>

Di diff view, klik **Review code** di toolbar kanan atas untuk meminta Claude mengevaluasi perubahan sebelum Anda melakukan commit. Claude memeriksa diff saat ini dan meninggalkan komentar langsung di diff view. Anda dapat merespons komentar apa pun atau meminta Claude untuk merevisi.

Tinjauan berfokus pada masalah sinyal tinggi: kesalahan kompilasi, kesalahan logika pasti, kerentanan keamanan, dan bug yang jelas. Ini tidak menandai gaya, pemformatan, masalah yang sudah ada sebelumnya, atau apa pun yang akan ditangkap linter.

<h3 id="monitor-pull-request-status">
  Pantau status pull request
</h3>

Setelah Anda membuka pull request, bilah status CI muncul di sesi. Claude Code menggunakan GitHub CLI untuk menanyakan hasil pemeriksaan dan menampilkan kegagalan.

* **Auto-fix**: ketika diaktifkan, Claude secara otomatis mencoba memperbaiki pemeriksaan CI yang gagal dengan membaca output kegagalan dan mengulangi.
* **Auto-merge**: ketika diaktifkan, Claude menggabungkan PR setelah semua pemeriksaan lulus. Metode penggabungan adalah squash. Auto-merge harus [diaktifkan di pengaturan repositori GitHub Anda](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/configuring-pull-request-merges/managing-auto-merge-for-pull-requests-in-your-repository) agar ini berfungsi.

Gunakan toggle **Auto-fix** dan **Auto-merge** di bilah status CI untuk mengaktifkan salah satu opsi. Claude Code juga mengirim notifikasi desktop ketika CI selesai. Untuk mengarsipkan sesi secara otomatis setelah PR digabungkan atau ditutup, aktifkan [auto-archive](#work-in-parallel-with-sessions) di Settings → Claude Code.

<Note>
  Pemantauan PR memerlukan [GitHub CLI (`gh`)](https://cli.github.com/) untuk diinstal dan diautentikasi di mesin Anda. Jika `gh` tidak diinstal, Desktop akan meminta Anda untuk memasangnya saat pertama kali Anda mencoba membuat PR.
</Note>

<h2 id="arrange-your-workspace">
  Atur workspace Anda
</h2>

Tab Code dibangun di sekitar pane yang dapat Anda atur dalam tata letak apa pun: chat, diff, browser, terminal, file, plan, tasks, dan subagent. Seret pane dengan headernya untuk memposisikan ulang, atau seret tepi pane untuk mengubah ukurannya. Tekan **Cmd+\\** di macOS atau **Ctrl+\\** di Windows untuk menutup pane yang fokus. Buka pane tambahan dari menu **Views** di toolbar sesi.

<Note>
  Tata letak pane, terminal, editor file, dan view modes di bagian ini memerlukan Claude Desktop v1.2581.0 atau lebih baru. Buka **Claude → Check for Updates** di macOS atau **Help → Check for Updates** di Windows untuk memperbarui.
</Note>

<h3 id="run-commands-in-the-terminal">
  Jalankan perintah di terminal
</h3>

Terminal terintegrasi memungkinkan Anda menjalankan perintah bersama sesi Anda tanpa beralih ke aplikasi lain. Buka dari menu **Views** atau tekan **Ctrl+\`** di macOS atau Windows. Terminal terbuka di direktori kerja sesi Anda dan berbagi lingkungan yang sama dengan Claude, jadi perintah seperti `npm test` atau `git status` melihat file yang sama yang sedang diedit Claude. Untuk membuka tab terminal kedua, klik **+** di header pane terminal atau klik kanan folder di chat untuk memilih **Open in terminal**. Terminal hanya tersedia di sesi lokal.

<h3 id="open-and-edit-files">
  Buka dan edit file
</h3>

Klik path file di chat atau diff viewer untuk membukanya di pane file. Path HTML, PDF, gambar, dan video terbuka di [pane Browser](#preview-your-app) sebagai gantinya. Buat edit spot dan klik **Save** untuk menulisnya kembali. Jika file berubah di disk sejak Anda membukanya, pane memperingatkan Anda dan memungkinkan Anda menimpa atau membuang. Klik **Discard** untuk mengembalikan edit Anda, atau klik path di header pane untuk menyalin path absolut.

Pane file tersedia di sesi lokal dan SSH. Untuk sesi cloud, minta Claude untuk membuat perubahan.

<h3 id="open-files-in-other-apps">
  Buka file di aplikasi lain
</h3>

Klik kanan path file apa pun di chat, diff viewer, atau pane file untuk membuka menu konteks:

* **Attach as context**: tambahkan file ke prompt berikutnya Anda
* **Open in**: buka file di editor yang diinstal seperti VS Code, Cursor, atau Zed
* **Show in Finder** di macOS, **Show in Explorer** di Windows: buka folder yang berisi
* **Copy path**: salin path absolut ke clipboard Anda

<h3 id="switch-view-modes">
  Alihkan view modes
</h3>

View modes mengontrol berapa banyak detail yang muncul di transkrip chat. Alihkan mode dari dropdown **Transcript view** di sebelah tombol kirim, atau tekan **Ctrl+O** di macOS atau Windows untuk bersiklus melaluinya.

| Mode        | Apa yang ditampilkan                                                   |
| ----------- | ---------------------------------------------------------------------- |
| **Normal**  | Tool calls yang diciutkan menjadi ringkasan, dengan respons teks penuh |
| **Verbose** | Setiap tool call, file read, dan langkah perantara yang diambil Claude |
| **Summary** | Hanya respons final Claude dan perubahan yang dibuatnya                |

Gunakan Verbose saat men-debug mengapa Claude mengambil tindakan tertentu. Gunakan Summary ketika Anda menjalankan beberapa sesi dan ingin memindai hasil dengan cepat.

<h3 id="keyboard-shortcuts">
  Keyboard shortcuts
</h3>

Tekan **Cmd+/** di macOS atau **Ctrl+/** di Windows untuk melihat semua pintasan keyboard yang tersedia di tab Code. Di Windows, gunakan **Ctrl** sebagai pengganti **Cmd** untuk pintasan di bawah. Siklus sesi, toggle terminal, dan toggle view-mode menggunakan **Ctrl** di setiap platform.

| Shortcut                              | Action                          |
| ------------------------------------- | ------------------------------- |
| `Cmd` `/`                             | Tampilkan pintasan keyboard     |
| `Cmd` `N`                             | Sesi baru                       |
| `Cmd` `W`                             | Tutup sesi                      |
| `Ctrl` `Tab` / `Ctrl` `Shift` `Tab`   | Sesi berikutnya atau sebelumnya |
| `Cmd` `Shift` `]` / `Cmd` `Shift` `[` | Sesi berikutnya atau sebelumnya |
| `Esc`                                 | Hentikan respons Claude         |
| `Cmd` `Shift` `D`                     | Alihkan pane diff               |
| `Cmd` `Shift` `B`                     | Alihkan pane Browser            |
| `Cmd` `Shift` `S`                     | Pilih elemen di Browser         |
| `Ctrl` `` ` ``                        | Alihkan pane terminal           |
| `Cmd` `\`                             | Tutup pane yang fokus           |
| `Cmd` `;`                             | Buka side chat                  |
| `Ctrl` `O`                            | Siklus view modes               |
| `Cmd` `Shift` `M`                     | Buka menu permission mode       |
| `Cmd` `Shift` `I`                     | Buka menu model                 |
| `Cmd` `Shift` `E`                     | Buka menu effort                |
| `1`–`9`                               | Pilih item di menu terbuka      |

Pintasan ini hanya berlaku untuk tab Code. Pintasan [interactive mode](/docs/id/interactive-mode#keyboard-shortcuts) berbasis terminal, seperti `Shift+Tab` untuk siklus mode, tidak berlaku di Desktop.

<h3 id="check-usage">
  Periksa penggunaan
</h3>

Klik cincin penggunaan di sebelah pemilih model untuk melihat penggunaan jendela konteks saat ini Anda dan penggunaan rencana Anda untuk periode tersebut. Penggunaan konteks per sesi; penggunaan rencana dibagikan di semua permukaan Claude Code Anda.

<h2 id="let-claude-use-your-computer">
  Biarkan Claude menggunakan komputer Anda
</h2>

Computer use memungkinkan Claude membuka aplikasi Anda, mengontrol layar Anda, dan bekerja langsung di mesin Anda seperti yang Anda lakukan. Minta Claude untuk menguji aplikasi native di mobile simulator, berinteraksi dengan alat desktop yang tidak memiliki CLI, atau mengotomatisasi sesuatu yang hanya berfungsi melalui GUI.

<Note>
  Computer use adalah pratinjau penelitian di macOS dan Windows yang memerlukan rencana Pro atau Max. Ini tidak tersedia di rencana Team atau Enterprise. Aplikasi Claude Desktop harus berjalan.
</Note>

Computer use dimatikan secara default. [Aktifkan di Settings](#enable-computer-use) sebelum Claude dapat mengontrol layar Anda. Di macOS, Anda juga perlu memberikan izin Accessibility dan Screen Recording.

<Warning>
  Tidak seperti [alat Bash yang disandbox](/docs/id/sandboxing), computer use berjalan di desktop aktual Anda dengan akses ke apa pun yang Anda setujui. Claude memeriksa setiap tindakan dan menandai potensi prompt injection dari konten di layar, tetapi batas kepercayaan berbeda. Lihat [panduan keamanan computer use](https://support.claude.com/en/articles/14128542) untuk praktik terbaik.
</Warning>

<h3 id="when-computer-use-applies">
  Kapan computer use berlaku
</h3>

Claude memiliki beberapa cara untuk berinteraksi dengan aplikasi atau layanan, dan computer use adalah yang paling luas dan paling lambat. Ini mencoba alat yang paling presisi terlebih dahulu:

* Jika Anda memiliki [konektor](#connect-external-tools) untuk layanan, Claude menggunakan konektor.
* Jika tugas adalah perintah shell, Claude menggunakan Bash.
* Jika tugas adalah pekerjaan browser dan Anda memiliki [Claude di Chrome](/docs/id/chrome) yang disiapkan, Claude menggunakan itu.
* Jika tidak ada yang berlaku, Claude menggunakan computer use.

[Per-app access tiers](#app-permissions) memperkuat ini: browser dibatasi hanya tampilan, dan terminal serta IDE dibatasi hanya klik, mengarahkan Claude ke alat khusus bahkan ketika computer use aktif. Kontrol layar dicadangkan untuk hal-hal yang tidak dapat dijangkau oleh yang lain, seperti aplikasi native, panel kontrol perangkat keras, mobile simulator, atau alat proprietary tanpa API.

<h3 id="enable-computer-use">
  Aktifkan computer use
</h3>

Computer use dimatikan secara default. Jika Anda meminta Claude melakukan sesuatu yang membutuhkannya saat dimatikan, Claude memberi tahu Anda bahwa itu dapat melakukan tugas jika Anda mengaktifkan computer use di Settings.

<Steps>
  <Step title="Perbarui aplikasi desktop">
    Pastikan Anda memiliki versi terbaru Claude Desktop. Pada macOS dan Windows, unduh atau perbarui di [claude.com/download](https://claude.com/download); di Linux, perbarui melalui manajer paket Anda ([instruksi](/docs/id/desktop-linux)). Kemudian mulai ulang aplikasi.
  </Step>

  <Step title="Aktifkan toggle">
    Di aplikasi desktop, buka **Settings > General** (di bawah **Desktop app**). Temukan toggle **Computer use** dan aktifkan. Di Windows, toggle berlaku segera dan setup selesai. Di macOS, lanjutkan ke langkah berikutnya.

    Jika Anda tidak melihat toggle, konfirmasi Anda menggunakan macOS atau Windows dengan rencana Pro atau Max, kemudian perbarui dan mulai ulang aplikasi.
  </Step>

  <Step title="Berikan izin macOS">
    Di macOS, berikan dua izin sistem sebelum toggle berlaku:

    * **Accessibility**: memungkinkan Claude mengklik, mengetik, dan menggulir
    * **Screen Recording**: memungkinkan Claude melihat apa yang ada di layar Anda

    Halaman Settings menunjukkan status saat ini dari setiap izin. Jika salah satu ditolak, klik badge untuk membuka pane System Settings yang relevan.
  </Step>
</Steps>

<h3 id="app-permissions">
  App permissions
</h3>

Pertama kali Claude perlu menggunakan aplikasi, prompt muncul di sesi Anda. Klik **Allow for this session** atau **Deny**. Persetujuan berlaku untuk sesi saat ini, atau 30 menit di [sesi yang dispawn Dispatch](#sessions-from-dispatch).

Prompt juga menunjukkan tingkat kontrol apa yang diperoleh Claude untuk aplikasi itu. Tingkat ini diperbaiki berdasarkan kategori aplikasi dan tidak dapat diubah:

| Tier         | Apa yang dapat dilakukan Claude                                          | Berlaku untuk                 |
| :----------- | :----------------------------------------------------------------------- | :---------------------------- |
| View only    | Lihat aplikasi di tangkapan layar                                        | Browser, platform perdagangan |
| Click only   | Klik dan gulir, tetapi tidak mengetik atau menggunakan pintasan keyboard | Terminal, IDE                 |
| Full control | Klik, ketik, seret, dan gunakan pintasan keyboard                        | Semuanya yang lain            |

Aplikasi dengan jangkauan luas seperti terminal, Finder atau File Explorer, dan System Settings atau Settings menampilkan peringatan tambahan di prompt sehingga Anda tahu apa yang disetujui.

Anda dapat mengonfigurasi dua pengaturan di **Settings > General** (di bawah **Desktop app**):

* **Denied apps**: tambahkan aplikasi di sini untuk menolaknya tanpa meminta. Claude mungkin masih mempengaruhi aplikasi yang ditolak secara tidak langsung melalui tindakan di aplikasi yang diizinkan, tetapi tidak dapat berinteraksi dengan aplikasi yang ditolak secara langsung.
* **Unhide apps when Claude finishes**: saat Claude bekerja, jendela lain Anda disembunyikan sehingga hanya berinteraksi dengan aplikasi yang disetujui. Ketika Claude selesai, jendela yang disembunyikan dipulihkan kecuali Anda mematikan pengaturan ini.

<h2 id="manage-sessions">
  Kelola sesi
</h2>

Setiap sesi adalah percakapan independen dengan konteks dan perubahannya sendiri. Anda dapat menjalankan beberapa sesi secara paralel, membuat side chat, mengirim pekerjaan ke cloud, atau membiarkan Dispatch memulai sesi untuk Anda dari ponsel Anda.

<h3 id="work-in-parallel-with-sessions">
  Bekerja secara paralel dengan sesi
</h3>

Klik **+ New session** di sidebar, atau tekan **Cmd+N** di macOS atau **Ctrl+N** di Windows, untuk bekerja pada beberapa tugas secara paralel. Tekan **Ctrl+Tab** dan **Ctrl+Shift+Tab** untuk bersiklus melalui sesi di sidebar. Untuk repositori Git, setiap sesi mendapatkan salinan proyek Anda yang terisolasi menggunakan [Git worktrees](/docs/id/worktrees), sehingga perubahan dalam satu sesi tidak mempengaruhi sesi lain sampai Anda melakukan commit.

Untuk melihat dua sesi sekaligus, tahan **Cmd** di macOS atau **Ctrl** di Windows dan klik sesi di sidebar. Sesi terbuka di pane kedua di samping yang sudah Anda buka. Saat split aktif, mengklik sesi sidebar lain mengganti pane mana pun yang memiliki fokus. Tekan **Cmd+\\** di macOS atau **Ctrl+\\** di Windows untuk menutup pane yang difokuskan dan kembali ke sesi tunggal.

Worktrees disimpan di `<project-root>/.claude/worktrees/` secara default. Anda dapat mengubah ini ke direktori kustom di Settings → Claude Code di bawah "Worktree location". Anda juga dapat mengatur awalan cabang yang ditambahkan ke setiap nama cabang worktree, yang berguna untuk menjaga cabang yang dibuat Claude tetap terorganisir. Untuk menghapus worktree ketika selesai, arahkan ke sesi di sidebar dan klik ikon archive. Untuk memiliki sesi mengarsipkan diri mereka sendiri ketika pull request mereka digabungkan atau ditutup, aktifkan **Auto-archive after PR merge or close** di Settings → Claude Code. Auto-archive hanya berlaku untuk sesi lokal yang telah selesai berjalan.

Untuk menyertakan file yang diabaikan git seperti `.env` di worktree baru, buat file [`.worktreeinclude`](/docs/id/worktrees#copy-gitignored-files-into-worktrees) di root proyek Anda.

<Note>
  Isolasi sesi memerlukan [Git](https://git-scm.com/downloads). Sebagian besar Mac menyertakan Git secara default. Jalankan `git --version` di Terminal untuk memeriksa. Di Windows, Git diperlukan agar tab Code berfungsi: [unduh Git untuk Windows](https://git-scm.com/downloads/win), pasang, dan mulai ulang aplikasi. Jika Anda mengalami kesalahan Git, tanyakan Claude di tab [Cowork](https://claude.com/product/cowork) untuk membantu memecahkan masalah setup Anda.
</Note>

Gunakan kontrol di bagian atas sidebar untuk memfilter sesi berdasarkan status, proyek, atau lingkungan, dan untuk mengelompokkan sesi berdasarkan proyek. Untuk mengganti nama sesi, klik judul sesi di toolbar di bagian atas sesi aktif. Untuk memeriksa penggunaan konteks, lihat [Check usage](#check-usage). Ketika konteks penuh, Claude secara otomatis merangkum percakapan dan terus bekerja. Anda juga dapat mengetik `/compact` untuk memicu perangkuman lebih awal dan membebaskan ruang konteks. Lihat [jendela konteks](/docs/id/how-claude-code-works#the-context-window) untuk detail tentang cara pemadatan bekerja.

Aplikasi desktop mengirimkan notifikasi OS ketika sesi Code menyelesaikan tugas dan Anda tidak sedang melihat sesi tersebut.

<h3 id="ask-a-side-question-without-derailing-the-session">
  Tanyakan pertanyaan sampingan tanpa menggoyahkan sesi
</h3>

Side chat memungkinkan Anda mengajukan pertanyaan kepada Claude yang menggunakan konteks sesi Anda tetapi tidak menambahkan apa pun kembali ke percakapan utama. Gunakan ketika Anda ingin memahami sepotong kode, memeriksa asumsi, atau menjelajahi ide tanpa mengarahkan sesi ke arah yang berbeda.

Tekan **Cmd+;** di macOS atau **Ctrl+;** di Windows untuk membuka side chat, atau ketik `/btw` di kotak prompt. Side chat dapat membaca semuanya di thread utama hingga titik itu. Ketika selesai, tutup side chat dan lanjutkan sesi utama di mana Anda tinggalkan. Side chats tersedia di sesi lokal, SSH, dan WSL.

<h3 id="watch-background-tasks">
  Tonton background tasks
</h3>

Pane tasks menunjukkan pekerjaan latar belakang yang berjalan di dalam sesi saat ini: subagent, perintah shell latar belakang, dan [dynamic workflows](/docs/id/workflows). Buka dari menu **Views** atau seret ke dalam tata letak Anda.

Klik entri apa pun untuk melihat outputnya di pane subagent atau hentikan. Untuk melihat apa yang dilakukan sesi lain, gunakan [sidebar](#work-in-parallel-with-sessions).

<h3 id="run-long-running-tasks-remotely">
  Jalankan tugas jangka panjang dari jarak jauh
</h3>

Untuk refaktor besar, suite pengujian, migrasi, atau tugas jangka panjang lainnya, pilih **Remote** alih-alih **Local** saat memulai sesi. Sesi jarak jauh berjalan pada infrastruktur cloud Anthropic dan terus berjalan bahkan jika Anda menutup aplikasi atau mematikan komputer. Periksa kembali kapan saja untuk melihat kemajuan atau mengarahkan Claude ke arah yang berbeda. Anda juga dapat memantau sesi jarak jauh dari [claude.ai/code](https://claude.ai/code) atau aplikasi Claude iOS.

Sesi jarak jauh juga mendukung beberapa repositori. Setelah memilih lingkungan cloud, klik tombol **+** di sebelah pil repo untuk menambahkan repositori tambahan ke sesi. Setiap repo mendapatkan pemilih cabang sendiri. Ini berguna untuk tugas yang mencakup beberapa basis kode, seperti memperbarui perpustakaan bersama dan konsumennya.

Lihat [Claude Code di web](/docs/id/claude-code-on-the-web) untuk informasi lebih lanjut tentang cara kerja sesi jarak jauh.

<h3 id="continue-in-another-surface">
  Lanjutkan di permukaan lain
</h3>

Menu **Continue in**, dapat diakses dari ikon VS Code di kanan bawah toolbar sesi, memungkinkan Anda memindahkan sesi ke permukaan lain:

* **Claude Code on the Web**: mengirim sesi lokal Anda untuk terus berjalan dari jarak jauh. Desktop mendorong cabang Anda, menghasilkan ringkasan percakapan, dan membuat sesi jarak jauh baru dengan konteks lengkap. Anda kemudian dapat memilih untuk mengarsipkan sesi lokal atau menyimpannya. Ini memerlukan pohon kerja yang bersih, dan tidak tersedia untuk sesi SSH.
* **Your IDE**: membuka proyek Anda di IDE yang didukung di direktori kerja saat ini.

<h3 id="sessions-from-dispatch">
  Sesi dari Dispatch
</h3>

[Dispatch](https://support.claude.com/en/articles/13947068) adalah percakapan persisten dengan Claude yang tinggal di tab [Cowork](https://claude.com/product/cowork). Anda mengirim pesan Dispatch dengan tugas, dan itu memutuskan cara menanganinya.

Tugas dapat berakhir sebagai sesi Code dengan dua cara: Anda meminta satu secara langsung, seperti "buka sesi Claude Code dan perbaiki bug login", atau Dispatch memutuskan tugas adalah pekerjaan pengembangan dan menspawn satu sendiri. Tugas yang biasanya diarahkan ke Code termasuk memperbaiki bug, memperbarui dependensi, menjalankan tes, atau membuka pull request. Penelitian, pengeditan dokumen, dan pekerjaan spreadsheet tetap di Cowork.

Bagaimanapun, sesi Code muncul di sidebar tab Code dengan badge **Dispatch**. Anda mendapatkan notifikasi push di ponsel Anda ketika selesai atau memerlukan persetujuan Anda.

Jika Anda memiliki [computer use](#let-claude-use-your-computer) diaktifkan, sesi Code yang dispawn Dispatch juga dapat menggunakannya. Persetujuan aplikasi di sesi tersebut kedaluwarsa setelah 30 menit dan meminta kembali, daripada berlangsung untuk sesi penuh seperti sesi Code biasa.

Untuk setup, pairing, dan pengaturan Dispatch, lihat [artikel bantuan Dispatch](https://support.claude.com/en/articles/13947068). Dispatch memerlukan rencana Pro atau Max dan tidak tersedia di rencana Team atau Enterprise.

Dispatch adalah salah satu dari beberapa cara untuk bekerja dengan Claude ketika Anda jauh dari terminal Anda. Lihat [Platforms and integrations](/docs/id/platforms#work-when-you-are-away-from-your-terminal) untuk membandingkannya dengan Remote Control, Channels, Slack, dan tugas terjadwal.

<h2 id="extend-claude-code">
  Perluas Claude Code
</h2>

Hubungkan layanan eksternal, tambahkan alur kerja yang dapat digunakan kembali, sesuaikan perilaku Claude, dan konfigurasikan server pratinjau. Untuk mengelola connectors, skills, dan plugins di satu tempat, klik **Customize** di sidebar.

<h3 id="connect-external-tools">
  Hubungkan alat eksternal
</h3>

Untuk sesi lokal dan [SSH](#ssh-sessions), klik tombol **+** di sebelah kotak prompt dan pilih **Connectors** untuk menambahkan integrasi seperti Google Calendar, Slack, GitHub, Linear, Notion, dan lainnya. Anda dapat menambahkan connectors sebelum atau selama sesi. Tombol **+** tidak tersedia di sesi cloud atau WSL, tetapi [routines](/docs/id/routines) mengonfigurasi connectors pada waktu pembuatan routine.

Untuk mengelola atau memutuskan connectors, buka Settings → Connectors di aplikasi desktop, atau pilih **Manage connectors** dari menu Connectors di kotak prompt.

Setelah terhubung, Claude dapat membaca kalender Anda, mengirim pesan, membuat masalah, dan berinteraksi dengan alat Anda secara langsung. Anda dapat meminta Claude konektor apa yang dikonfigurasi di sesi Anda.

Connectors adalah [MCP servers](/docs/id/mcp) dengan alur pengaturan grafis. Gunakan untuk integrasi cepat dengan layanan yang didukung. Untuk integrasi yang tidak tercantum di Connectors, tambahkan MCP servers secara manual melalui [file pengaturan](/docs/id/mcp#installing-mcp-servers). Anda juga dapat [membuat custom connectors](https://support.claude.com/en/articles/11175166-getting-started-with-custom-connectors-using-remote-mcp).

<h3 id="use-skills">
  Gunakan skills
</h3>

[Skills](/docs/id/skills) memperluas apa yang dapat dilakukan Claude. Claude memuatnya secara otomatis ketika relevan, atau Anda dapat menginvokan satu secara langsung: ketik `/` di kotak prompt atau klik tombol **+** dan pilih **Slash commands** untuk melihat apa yang tersedia. Ini mencakup [built-in commands](/docs/id/commands), [custom skills](/docs/id/skills#create-your-first-skill) Anda, project skills dari basis kode Anda, dan skills dari [installed plugins](/docs/id/plugins) apa pun. Pilih satu dan itu muncul disorot di bidang input. Ketik tugas Anda setelahnya dan kirim seperti biasa.

Anda dapat mengirim perintah saat Claude sedang bekerja, sama seperti pesan lainnya, dan sesi kembali ke idle setelah giliran selesai. Sebelum v2.1.206, perintah yang dikirim di tengah giliran dapat membuat sesi tetap menampilkan sebagai berjalan dan pesan yang Anda kirim setelahnya tidak terkirim.

<h3 id="install-plugins">
  Instal plugins
</h3>

[Plugins](/docs/id/plugins) adalah paket yang dapat digunakan kembali yang menambahkan skills, agents, hooks, MCP servers, dan konfigurasi LSP ke Claude Code. Anda dapat memasang plugins dari aplikasi desktop tanpa menggunakan terminal.

Untuk sesi lokal dan [SSH](#ssh-sessions), klik tombol **+** di sebelah kotak prompt dan pilih **Plugins** untuk melihat plugins yang diinstal dan skills mereka. Untuk menambahkan plugin, pilih **Add plugin** dari submenu untuk membuka plugin browser, yang menampilkan plugins yang tersedia dari [marketplaces](/docs/id/plugin-marketplaces) yang dikonfigurasi termasuk marketplace Anthropic resmi. Pilih **Manage plugins** untuk mengaktifkan, menonaktifkan, atau mencopot plugins.

Plugins dapat dibatasi pada akun pengguna Anda, proyek tertentu, atau lokal saja. Jika organisasi Anda mengelola plugins secara terpusat, plugins tersebut tersedia di sesi desktop dengan cara yang sama seperti di CLI. Plugins tidak tersedia untuk sesi cloud atau WSL. Untuk referensi plugin lengkap termasuk membuat plugins Anda sendiri, lihat [plugins](/docs/id/plugins).

<h3 id="configure-preview-servers">
  Konfigurasikan server pratinjau
</h3>

Claude secara otomatis mendeteksi setup dev server Anda dan menyimpan konfigurasi di `.claude/launch.json` di root folder yang Anda pilih saat memulai sesi. Preview menggunakan folder ini sebagai direktori kerjanya, jadi jika Anda memilih folder induk, subfolder dengan server dev mereka sendiri tidak akan terdeteksi secara otomatis. Untuk bekerja dengan server subfolder, mulai sesi di folder itu secara langsung atau tambahkan konfigurasi secara manual.

Untuk menyesuaikan cara server Anda dimulai, misalnya menggunakan `yarn dev` alih-alih `npm run dev` atau mengubah port, edit file secara manual atau klik **Edit configuration** di dropdown server untuk membukanya di editor kode Anda. File mendukung JSON dengan komentar.

```json theme={null}
{
  "version": "0.0.1",
  "configurations": [
    {
      "name": "my-app",
      "runtimeExecutable": "npm",
      "runtimeArgs": ["run", "dev"],
      "port": 3000
    }
  ]
}
```

Anda dapat menentukan beberapa konfigurasi untuk menjalankan server berbeda dari proyek yang sama, seperti frontend dan API. Lihat [examples](#examples) di bawah.

<h4 id="auto-verify-changes">
  Auto-verify changes
</h4>

Ketika `autoVerify` diaktifkan, Claude secara otomatis memverifikasi perubahan kode setelah mengedit file. Mengambil tangkapan layar, memeriksa kesalahan, dan mengkonfirmasi perubahan berfungsi sebelum menyelesaikan responsnya.

Auto-verify aktif secara default. Nonaktifkan per-proyek dengan menambahkan `"autoVerify": false` ke `.claude/launch.json`, atau alihkan dari menu dropdown server.

```json theme={null}
{
  "version": "0.0.1",
  "autoVerify": false,
  "configurations": [...]
}
```

Ketika dinonaktifkan, alat pratinjau masih tersedia dan Anda dapat meminta Claude untuk memverifikasi kapan saja. Auto-verify membuatnya otomatis setelah setiap edit.

<h4 id="configuration-fields">
  Configuration fields
</h4>

Setiap entri dalam array `configurations` menerima bidang berikut:

| Field               | Type      | Description                                                                                                                                                                                                                                                                                              |
| ------------------- | --------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`              | string    | Pengidentifikasi unik untuk server ini                                                                                                                                                                                                                                                                   |
| `runtimeExecutable` | string    | Perintah untuk dijalankan, seperti `npm`, `yarn`, atau `node`                                                                                                                                                                                                                                            |
| `runtimeArgs`       | string\[] | Argumen yang dilewatkan ke `runtimeExecutable`, seperti `["run", "dev"]`                                                                                                                                                                                                                                 |
| `port`              | number    | Port yang didengarkan server Anda. Default ke 3000                                                                                                                                                                                                                                                       |
| `cwd`               | string    | Direktori kerja relatif terhadap root proyek Anda. Default ke root proyek. Gunakan `${workspaceFolder}` untuk mereferensikan root proyek secara eksplisit                                                                                                                                                |
| `env`               | object    | Variabel lingkungan tambahan sebagai pasangan kunci-nilai, seperti `{ "NODE_ENV": "development" }`. Jangan letakkan rahasia di sini karena file ini dilakukan commit ke repo Anda. Untuk meneruskan rahasia ke dev server Anda, aturlah di [local environment editor](#local-sessions) sebagai gantinya. |
| `autoPort`          | boolean   | Cara menangani konflik port. Lihat di bawah                                                                                                                                                                                                                                                              |
| `program`           | string    | Skrip untuk dijalankan dengan `node`. Lihat [when to use `program` vs `runtimeExecutable`](#when-to-use-program-vs-runtimeexecutable)                                                                                                                                                                    |
| `args`              | string\[] | Argumen yang dilewatkan ke `program`. Hanya digunakan ketika `program` diatur                                                                                                                                                                                                                            |

<a id="when-to-use-program-vs-runtimeexecutable" />

<h5 id="when-to-use-program-vs-runtimeexecutable">
  When to use `program` vs `runtimeExecutable`
</h5>

Gunakan `runtimeExecutable` dengan `runtimeArgs` untuk memulai dev server melalui package manager. Misalnya, `"runtimeExecutable": "npm"` dengan `"runtimeArgs": ["run", "dev"]` menjalankan `npm run dev`.

Gunakan `program` ketika Anda memiliki skrip mandiri yang ingin Anda jalankan dengan `node` secara langsung. Misalnya, `"program": "server.js"` menjalankan `node server.js`. Lewatkan flag tambahan dengan `args`.

<h4 id="port-conflicts">
  Port conflicts
</h4>

Bidang `autoPort` mengontrol apa yang terjadi ketika port pilihan Anda sudah digunakan:

* **`true`**: Claude menemukan dan menggunakan port gratis secara otomatis. Cocok untuk sebagian besar dev server.
* **`false`**: Claude gagal dengan kesalahan. Gunakan ini ketika server Anda harus menggunakan port tertentu, seperti untuk callback OAuth atau allowlist CORS.
* **Not set (default)**: Claude menanyakan apakah server memerlukan port itu, kemudian menyimpan jawaban Anda.

Ketika Claude memilih port yang berbeda, itu melewatkan port yang ditugaskan ke server Anda melalui variabel lingkungan `PORT`.

<h4 id="examples">
  Examples
</h4>

Konfigurasi ini menunjukkan setup umum untuk tipe proyek berbeda:

<Tabs>
  <Tab title="Next.js">
    Konfigurasi ini menjalankan aplikasi Next.js menggunakan Yarn di port 3000:

    ```json theme={null}
    {
      "version": "0.0.1",
      "configurations": [
        {
          "name": "web",
          "runtimeExecutable": "yarn",
          "runtimeArgs": ["dev"],
          "port": 3000
        }
      ]
    }
    ```
  </Tab>

  <Tab title="Multiple servers">
    Untuk monorepo dengan server frontend dan API, tentukan beberapa konfigurasi. Frontend menggunakan `autoPort: true` sehingga memilih port gratis jika 3000 diambil, sementara server API memerlukan port 8080 dengan tepat:

    ```json theme={null}
    {
      "version": "0.0.1",
      "configurations": [
        {
          "name": "frontend",
          "runtimeExecutable": "npm",
          "runtimeArgs": ["run", "dev"],
          "cwd": "apps/web",
          "port": 3000,
          "autoPort": true
        },
        {
          "name": "api",
          "runtimeExecutable": "npm",
          "runtimeArgs": ["run", "start"],
          "cwd": "server",
          "port": 8080,
          "env": { "NODE_ENV": "development" },
          "autoPort": false
        }
      ]
    }
    ```
  </Tab>

  <Tab title="Node.js script">
    Untuk menjalankan skrip Node.js secara langsung alih-alih menggunakan perintah package manager, gunakan bidang `program`:

    ```json theme={null}
    {
      "version": "0.0.1",
      "configurations": [
        {
          "name": "server",
          "program": "server.js",
          "args": ["--verbose"],
          "port": 4000
        }
      ]
    }
    ```
  </Tab>
</Tabs>

<h2 id="environment-configuration">
  Konfigurasi lingkungan
</h2>

Lingkungan yang Anda pilih saat [memulai sesi](#start-a-session) menentukan di mana Claude mengeksekusi dan cara Anda terhubung:

* **Local**: berjalan di mesin Anda dengan akses langsung ke file Anda
* **Remote**: berjalan pada infrastruktur cloud Anthropic. Sesi terus berlanjut bahkan jika Anda menutup aplikasi.
* **SSH**: berjalan di mesin jarak jauh yang Anda hubungkan melalui SSH, seperti server Anda sendiri, cloud VM, atau dev container
* **WSL** (Windows): berjalan di dalam [distribusi WSL 2](/docs/id/desktop-wsl) di mesin Anda, menggunakan toolchain Linux dan path aslinya

<h3 id="local-sessions">
  Local sessions
</h3>

Aplikasi desktop tidak selalu mewarisi lingkungan shell lengkap Anda. Di macOS, ketika Anda meluncurkan aplikasi dari Dock atau Finder, itu membaca profil shell Anda, seperti `~/.zshrc` atau `~/.bashrc`, untuk mengekstrak `PATH` dan set tetap variabel Claude Code, tetapi variabel lain yang Anda ekspor di sana tidak diambil. Di Windows, aplikasi mewarisi variabel lingkungan pengguna dan sistem tetapi tidak membaca profil PowerShell.

Untuk mengatur variabel lingkungan untuk sesi lokal dan dev server di platform apa pun, buka dropdown lingkungan di kotak prompt, arahkan ke **Local**, dan klik ikon gear untuk membuka editor lingkungan lokal. Variabel yang Anda simpan di sini disimpan terenkripsi di mesin Anda dan berlaku untuk setiap sesi lokal dan server pratinjau yang Anda mulai. Anda juga dapat menambahkan variabel ke kunci `env` di file `~/.claude/settings.json` Anda, meskipun ini hanya mencapai sesi Claude dan bukan dev server. Lihat [environment variables](/docs/id/env-vars) untuk daftar lengkap variabel yang didukung.

[Extended thinking](/docs/id/model-config#extended-thinking) diaktifkan secara default, yang meningkatkan kinerja pada tugas penalaran kompleks tetapi menggunakan token tambahan. Untuk menonaktifkan pemikiran, atur `MAX_THINKING_TOKENS` ke `0` di editor lingkungan lokal; ini tidak berpengaruh pada Fable 5, yang selalu menggunakan extended thinking. Pada [penyedia pihak ketiga](/docs/id/third-party-integrations), `0` menghilangkan parameter `thinking` sebagai gantinya, dan model adaptive-reasoning mungkin masih berpikir. Pada model dengan [adaptive reasoning](/docs/id/model-config#adjust-effort-level), nilai `MAX_THINKING_TOKENS` apa pun yang lain diabaikan karena adaptive reasoning mengontrol kedalaman pemikiran sebagai gantinya. Pada Opus 4.6 dan Sonnet 4.6, atur `CLAUDE_CODE_DISABLE_ADAPTIVE_THINKING` ke `1` untuk menggunakan anggaran pemikiran tetap; Fable 5, Sonnet 5, dan Opus 4.7 dan yang lebih baru selalu menggunakan adaptive reasoning dan tidak memiliki mode anggaran tetap.

<h3 id="cloud-sessions">
  Cloud sessions
</h3>

Sesi cloud terus berlanjut di latar belakang bahkan jika Anda menutup aplikasi. Penggunaan dihitung terhadap [batas rencana langganan](/docs/id/costs) Anda tanpa biaya komputasi terpisah.

Anda dapat membuat lingkungan cloud kustom dengan tingkat akses jaringan dan variabel lingkungan yang berbeda. Pilih dropdown lingkungan saat memulai sesi cloud dan pilih **Add environment**. Lihat [cloud environment](/docs/id/claude-code-on-the-web#the-cloud-environment) untuk detail tentang mengonfigurasi akses jaringan dan variabel lingkungan.

<h3 id="ssh-sessions">
  SSH sessions
</h3>

Sesi SSH memungkinkan Anda menjalankan Claude Code di mesin jarak jauh sambil menggunakan aplikasi desktop sebagai antarmuka Anda. Ini berguna untuk bekerja dengan basis kode yang tinggal di cloud VM, dev container, atau server dengan perangkat keras atau dependensi tertentu.

Untuk menambahkan koneksi SSH, klik dropdown lingkungan sebelum memulai sesi dan pilih **+ Add SSH connection**. Dialog menanyakan:

* **Name**: label ramah untuk koneksi ini
* **SSH Host**: `user@hostname` atau host yang ditentukan di `~/.ssh/config`
* **SSH Port**: default ke 22 jika dibiarkan kosong, atau menggunakan port dari konfigurasi SSH Anda
* **Identity File**: path ke kunci pribadi Anda, seperti `~/.ssh/id_rsa`. Biarkan kosong untuk menggunakan kunci default atau konfigurasi SSH Anda.

Setelah ditambahkan, koneksi muncul di dropdown lingkungan. Pilih untuk memulai sesi di mesin itu. Claude berjalan di mesin jarak jauh dengan akses ke file dan alatnya.

Mesin jarak jauh harus menjalankan Linux atau macOS. Desktop menginstal Claude Code di mesin jarak jauh secara otomatis saat pertama kali Anda terhubung. Setelah terhubung, sesi SSH mendukung permission modes, connectors, plugins, dan MCP servers.

<h4 id="pre-configure-ssh-connections-for-your-team">
  Pre-configure SSH connections for your team
</h4>

Administrator dapat mendistribusikan koneksi SSH kepada anggota tim dengan menambahkan `sshConfigs` ke file [managed settings](/docs/id/settings#settings-precedence). Koneksi yang ditentukan dengan cara ini muncul di dropdown lingkungan setiap pengguna secara otomatis dan ditampilkan sebagai terkelola, sehingga pengguna dapat memilihnya tetapi tidak dapat mengedit atau menghapusnya di aplikasi.

Contoh berikut pre-configure satu koneksi yang terbuka di `~/projects` pada host jarak jauh:

```json theme={null}
{
  "sshConfigs": [
    {
      "id": "shared-dev-vm",
      "name": "Shared Dev VM",
      "sshHost": "user@dev.example.com",
      "sshPort": 22,
      "sshIdentityFile": "~/.ssh/id_ed25519",
      "startDirectory": "~/projects"
    }
  ]
}
```

Setiap entri memerlukan `id`, `name`, dan `sshHost`. Bidang `sshPort`, `sshIdentityFile`, dan `startDirectory` bersifat opsional. Pengguna juga dapat menambahkan `sshConfigs` ke `~/.claude/settings.json` mereka sendiri, yang merupakan tempat koneksi yang ditambahkan melalui dialog disimpan.

<h4 id="restrict-which-ssh-hosts-users-can-connect-to">
  Restrict which SSH hosts users can connect to
</h4>

Administrator dapat membatasi sesi SSH Desktop ke set host yang disetujui dengan menambahkan `sshHostAllowlist` ke file [managed settings](/docs/id/settings#settings-precedence). Ketika diatur, pengguna hanya dapat terhubung ke host yang nama hostname terselesaikannya cocok dengan salah satu pola. Atur ke array kosong untuk menonaktifkan sesi SSH sepenuhnya.

Contoh berikut memungkinkan koneksi ke host apa pun di bawah `devboxes.example.com` dan ke satu host bastion bernama:

```json theme={null}
{
  "sshHostAllowlist": ["*.devboxes.example.com", "bastion.example.com"]
}
```

Pola tidak peka huruf besar-kecil. `*` cocok dengan host apa pun, dan `*.example.com` cocok dengan `example.com` dan subdomain apa pun. Apa pun yang lain adalah kecocokan yang tepat. Pemeriksaan berjalan terhadap hostname setelah resolusi `~/.ssh/config` melalui `ssh -G`, sehingga alias `Host` dan entri `ProxyCommand`/`ProxyJump` diizinkan selama `HostName` yang terselesaikan cocok.

`sshHostAllowlist` dibaca dari managed settings saja; nilai dalam pengaturan pengguna atau proyek diabaikan. Hanya aplikasi Claude Desktop yang menghormati pengaturan ini; CLI Claude Code dan ekstensi IDE tidak membacanya, dan itu tidak membatasi perintah `ssh` yang dijalankan melalui alat Bash. Ini mengatur host mana yang terhubung oleh aplikasi Desktop, bukan egress jaringan, jadi pasangkan dengan kontrol jaringan organisasi Anda atau kontrol zero-trust jika Anda memerlukan batas yang keras.

<h2 id="enterprise-configuration">
  Konfigurasi Enterprise
</h2>

Organisasi pada rencana Team atau Enterprise dapat mengelola perilaku aplikasi desktop melalui kontrol konsol admin, file pengaturan yang dikelola, dan kebijakan manajemen perangkat.

<h3 id="admin-console-controls">
  Kontrol konsol admin
</h3>

Pengaturan ini dikonfigurasi melalui [konsol pengaturan admin](https://claude.ai/admin-settings/claude-code):

* **Code in the desktop**: kontrol apakah pengguna di organisasi Anda dapat mengakses Claude Code di aplikasi desktop
* **Code in the web**: aktifkan atau nonaktifkan [web sessions](/docs/id/claude-code-on-the-web) untuk organisasi Anda
* **Remote Control**: aktifkan atau nonaktifkan [Remote Control](/docs/id/remote-control) untuk organisasi Anda
* **Disable Bypass permissions mode**: cegah pengguna di organisasi Anda dari mengaktifkan bypass permissions mode

<h3 id="managed-settings">
  Pengaturan yang dikelola
</h3>

Pengaturan yang dikelola menimpa pengaturan proyek dan pengguna dan berlaku untuk sesi Claude Code di Desktop. Anda dapat mengatur kunci ini di file [managed settings](/docs/id/settings#settings-precedence) organisasi Anda atau mendorongnya dari jarak jauh melalui konsol admin.

| Key                                        | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| ------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `permissions.disableBypassPermissionsMode` | atur ke `"disable"` untuk mencegah pengguna dari mengaktifkan Bypass permissions mode.                                                                                                                                                                                                                                                                                                                                                                                                     |
| `disableAutoMode`                          | atur ke `"disable"` untuk mencegah pengguna dari mengaktifkan [Auto](/docs/id/permission-modes#eliminate-prompts-with-auto-mode) mode. Menghapus Auto dari pemilih mode. Juga diterima di bawah `permissions`.                                                                                                                                                                                                                                                                                  |
| `autoMode`                                 | sesuaikan apa yang dipercaya dan diblokir oleh pengklasifikasi auto mode di seluruh organisasi Anda. Lihat [Configure auto mode](/docs/id/auto-mode-config).                                                                                                                                                                                                                                                                                                                                    |
| `browserExternalPageTools`                 | atur ke `"disabled"` untuk mencegah Claude menggunakan tools untuk membaca atau bertindak pada halaman eksternal di [Browser pane](#browse-external-sites). Pengguna masih dapat menavigasi ke situs eksternal sendiri, dan pratinjau server dev lokal tidak terpengaruh.                                                                                                                                                                                                                  |
| `disableBrowserExternalNavigation`         | atur ke `true` untuk mematikan penjelajahan eksternal di [Browser pane](#browse-external-sites) sepenuhnya. Baik pengguna maupun Claude tidak dapat menavigasi ke situs eksternal, dan pratinjau server dev localhost tidak terpengaruh. Nilainya harus berupa boolean JSON `true`; string `"true"` diabaikan.                                                                                                                                                                             |
| `sshConfigs`                               | pre-configure [SSH connections](#pre-configure-ssh-connections-for-your-team) yang muncul di dropdown lingkungan. Pengguna tidak dapat mengedit atau menghapus koneksi yang dikelola.                                                                                                                                                                                                                                                                                                      |
| `sshHostAllowlist`                         | batasi [SSH sessions](#restrict-which-ssh-hosts-users-can-connect-to) ke host yang nama hostname yang diselesaikannya cocok dengan salah satu pola ini. Array kosong menonaktifkan sesi SSH. Dibaca dari pengaturan yang dikelola saja.                                                                                                                                                                                                                                                    |
| `managedMcpServers`                        | dorong konfigurasi server MCP ke semua pengguna dalam penyebaran pihak ketiga. Setiap entri menentukan transport `"http"`, `"sse"`, atau `"stdio"`, detail koneksi, dan secara opsional peta `toolPolicy` yang membatasi alat mana dalam server tersebut yang dapat dipanggil pengguna. Tersedia dalam penyebaran Desktop pihak ketiga (3P) saja. Berikan kunci ini melalui file pengaturan yang dikelola atau MDM, karena penyebaran pihak ketiga tidak menerima pengaturan konsol admin. |

Pengaturan yang dikelola mana yang mencapai sesi Desktop tergantung pada tempat sesi tersebut berjalan. Pembatasan model seperti [`availableModels`](/docs/id/model-config#restrict-model-selection) diberlakukan dalam sesi Claude Code Desktop dengan cara yang sama seperti di CLI terminal; lihat [surface coverage](/docs/id/model-config#surface-coverage).

* **Sesi lokal di mesin ini**: file pengaturan yang dikelola yang disebarkan ke disk berlaku. Pengaturan yang dikelola yang didorong dari jarak jauh melalui konsol admin juga mencapai sesi ini di API Anthropic ketika sesi mengautentikasi dengan login organisasi atau kunci API yang dikonfigurasi secara langsung, mengikuti [settings precedence](/docs/id/settings#settings-precedence) yang sama seperti CLI terminal.
* **[Sesi cloud](#cloud-sessions)**: berjalan pada VM yang dikelola Anthropic dan menerima [server-managed settings](/docs/id/server-managed-settings) saja.
* **[Sesi SSH](#ssh-sessions)**: sesi membaca file pengaturan yang dikelola dari host jarak jauh. Desktop itu sendiri membaca `sshConfigs` dan `sshHostAllowlist` dari pengaturan yang dikelola mesin lokal ketika membuat koneksi.

`permissions.disableBypassPermissionsMode` dan `disableAutoMode` juga bekerja di pengaturan pengguna dan proyek, tetapi menempatkannya di pengaturan yang dikelola mencegah pengguna dari menimpanya.

Claude Code membaca `autoMode` dari pengaturan pengguna, flag `--settings`, dan pengaturan yang dikelola, tetapi bukan dari `.claude/settings.json` atau `.claude/settings.local.json`: kedua file berada di direktori repo, jadi repo yang diklon atau langkah build tidak dapat menyuntikkan aturan pengklasifikasinya sendiri. Sebelum v2.1.207, Claude Code juga membaca `.claude/settings.local.json`.

Untuk daftar lengkap pengaturan khusus yang dikelola termasuk `allowManagedPermissionRulesOnly` dan `allowManagedHooksOnly`, lihat [managed-only settings](/docs/id/permissions#managed-only-settings).

<h3 id="device-management-policies">
  Kebijakan manajemen perangkat
</h3>

Tim IT dapat mengelola aplikasi desktop melalui MDM di macOS atau group policy di Windows. Kebijakan yang tersedia termasuk mengaktifkan atau menonaktifkan fitur Claude Code, mengontrol auto-updates, dan menetapkan URL penyebaran kustom.

* **macOS**: konfigurasikan melalui domain preferensi `com.anthropic.claudefordesktop` menggunakan alat seperti Jamf atau Kandji
* **Windows**: konfigurasikan melalui registri di `SOFTWARE\Policies\Claude`

<h3 id="network-access-requirements">
  Persyaratan akses jaringan
</h3>

Desktop memuat kode aplikasinya dan konten pengguna dari host CDN Anthropic.

```text theme={null}
anthropic.com
*.anthropic.com
claude.ai
*.claude.ai
claude.com
*.claude.com
claude.app
*.claude.app
*.claudeusercontent.com
*.claudemcpcontent.com
```

Lalu lintas adalah HTTPS pada port 443 kecuali Anda mengonfigurasi port kustom untuk [OTLP](/docs/id/monitoring-usage), gateway LLM, atau server MCP.

Untuk server proxy, otoritas sertifikat kustom, mTLS, dan domain yang dibutuhkan CLI standalone, lihat [network configuration](/docs/id/network-config).

Untuk mengurangi jumlah wildcard firewall, izinkan host Anthropic ini sebagai gantinya. Subdomain tertentu dihasilkan secara dinamis dan harus tetap sebagai wildcard.

```text theme={null}
anthropic.com
api.anthropic.com
a-api.anthropic.com
a-cdn.anthropic.com
s-cdn.anthropic.com
assets-proxy.anthropic.com
claude.ai
a.claude.ai
a-cdn.claude.ai
assets.claude.ai
downloads.claude.ai
*.livepreview.claude.ai
claude.com
platform.claude.com
*.livepreview.claude.app
*.claudeusercontent.com
*.claudemcpcontent.com
```

<h3 id="authentication-and-sso">
  Autentikasi dan SSO
</h3>

Organisasi enterprise dapat memerlukan SSO untuk semua pengguna. Lihat [authentication](/docs/id/authentication) untuk detail tingkat rencana dan [Setting up SSO](https://support.claude.com/en/articles/13132885-setting-up-single-sign-on-sso) untuk konfigurasi SAML; setup OIDC tercakup dalam [Claude Enterprise Administrator Guide](https://claude.com/resources/tutorials/claude-enterprise-administrator-guide).

<h3 id="data-handling">
  Penanganan data
</h3>

Claude Code memproses kode Anda secara lokal dalam sesi lokal atau pada infrastruktur cloud Anthropic dalam sesi cloud. Percakapan dan konteks kode dikirim ke API Anthropic untuk diproses. Lihat [data handling](/docs/id/data-usage) untuk detail tentang retensi data, privasi, dan kepatuhan.

<h3 id="deployment">
  Penyebaran
</h3>

Desktop dapat didistribusikan melalui alat penyebaran enterprise:

* **macOS**: distribusikan melalui MDM seperti Jamf atau Kandji menggunakan installer `.dmg`
* **Windows**: sebarkan melalui paket MSIX. Lihat [Deploy Claude Desktop for Windows](https://support.claude.com/en/articles/12622703-deploy-claude-desktop-for-windows) untuk opsi penyebaran enterprise termasuk instalasi senyap

Untuk domain yang akan diizinkan dalam firewall Anda, lihat [network access requirements](#network-access-requirements) di atas. Untuk pengaturan proxy, otoritas sertifikat kustom, dan gateway LLM, lihat [network configuration](/docs/id/network-config).

Untuk referensi konfigurasi enterprise lengkap, lihat [enterprise configuration guide](https://support.claude.com/en/articles/12622667-enterprise-configuration).

<h2 id="coming-from-the-cli">
  Datang dari CLI?
</h2>

Jika Anda sudah menggunakan CLI Claude Code, Desktop menjalankan mesin yang sama dengan antarmuka grafis. Anda dapat menjalankan keduanya secara bersamaan di mesin yang sama, bahkan di proyek yang sama. Masing-masing mempertahankan riwayat sesi terpisah, tetapi mereka berbagi konfigurasi dan memori proyek melalui file CLAUDE.md.

Untuk memindahkan sesi CLI ke Desktop, jalankan `/desktop` di terminal. Claude menyimpan sesi Anda dan membukanya di aplikasi desktop, kemudian keluar dari CLI. Perintah ini tersedia di macOS dan Windows ketika Anda masuk dengan langganan Claude. Perintah ini tidak tersedia dengan autentikasi kunci API atau di Amazon Bedrock, Google Cloud's Agent Platform, atau Microsoft Foundry.

<Tip>
  Kapan menggunakan Desktop vs CLI: gunakan Desktop ketika Anda ingin mengelola sesi paralel di satu jendela, mengatur pane berdampingan, atau meninjau perubahan secara visual. Gunakan CLI ketika Anda memerlukan scripting, otomasi, atau lebih suka alur kerja terminal.
</Tip>

<h3 id="cli-flag-equivalents">
  Setara flag CLI
</h3>

Tabel ini menunjukkan setara aplikasi desktop untuk flag CLI umum. Flag yang tidak tercantum tidak memiliki setara desktop karena dirancang untuk scripting atau otomasi.

| CLI                                   | Setara Desktop                                                                                                                                                                        |
| ------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `--model sonnet`                      | Dropdown model di sebelah tombol kirim                                                                                                                                                |
| `--resume`, `--continue`              | Klik sesi di sidebar                                                                                                                                                                  |
| `--permission-mode`                   | Pemilih mode di sebelah tombol kirim                                                                                                                                                  |
| `--dangerously-skip-permissions`      | Bypass permissions mode. Di paket Pro dan Max, aktifkan di Settings → Claude Code → "Allow bypass permissions mode"; di paket Team dan Enterprise, kebijakan organisasi mengontrolnya |
| `--add-dir`                           | Tambahkan beberapa repo dengan tombol **+** di sesi cloud                                                                                                                             |
| `--allowedTools`, `--disallowedTools` | Tidak ada setara per-sesi. Aturan izin di [file pengaturan](/docs/id/settings) masih berlaku.                                                                                              |
| `--verbose`                           | [Mode tampilan verbose](#switch-view-modes) di dropdown tampilan Transcript                                                                                                           |
| `--print`, `--output-format`          | Tidak tersedia. Desktop hanya interaktif.                                                                                                                                             |
| `ANTHROPIC_MODEL` env var             | Dropdown model di sebelah tombol kirim                                                                                                                                                |
| `MAX_THINKING_TOKENS` env var         | Atur di editor lingkungan lokal. Lihat [konfigurasi lingkungan](#environment-configuration).                                                                                          |

<h3 id="shared-configuration">
  Konfigurasi bersama
</h3>

Desktop dan CLI membaca file konfigurasi yang sama, jadi setup Anda terbawa:

* **[CLAUDE.md](/docs/id/memory)** dan file `CLAUDE.local.md` di proyek Anda digunakan oleh keduanya
* **[MCP servers](/docs/id/mcp)** yang dikonfigurasi di `~/.claude.json` atau `.mcp.json` bekerja di keduanya
* **[Hooks](/docs/id/hooks)** dan **[skills](/docs/id/skills)** yang ditentukan dalam pengaturan berlaku untuk keduanya
* **[Settings](/docs/id/settings)** di `~/.claude.json` dan `~/.claude/settings.json` dibagikan. Aturan izin, alat yang diizinkan, dan pengaturan lainnya di `settings.json` berlaku untuk sesi Desktop.
* **Models**: model yang sama [models](/docs/id/model-config#available-models) tersedia di keduanya. Di Desktop, pilih model dari dropdown di sebelah tombol kirim. Anda dapat mengubah model selama sesi dari dropdown yang sama.

<Note>
  **MCP servers dari aplikasi chat Claude Desktop**: aplikasi Desktop memuat MCP servers dari `claude_desktop_config.json` ke dalam sesi tab Code, bersama dengan servers dari `~/.claude.json` dan `.mcp.json`. Server yang ditentukan di `claude_desktop_config.json` tersedia di permukaan chat Desktop dan tab Code.

  CLI mandiri tidak membaca `claude_desktop_config.json`. Di macOS dan WSL, jalankan `claude mcp add-from-claude-desktop` untuk menyalin servers tersebut ke dalam `~/.claude.json`. Lihat [Impor MCP servers dari Claude Desktop](/docs/id/mcp#import-mcp-servers-from-claude-desktop) untuk alur impor dan opsi cakupan.
</Note>

<h3 id="feature-comparison">
  Perbandingan fitur
</h3>

Tabel ini membandingkan kemampuan inti antara CLI dan Desktop. Untuk daftar lengkap flag CLI, lihat [referensi CLI](/docs/id/cli-reference).

| Fitur                                                 | CLI                                                              | Desktop                                                                                                                                                                                                                                                                                                                                                                     |
| ----------------------------------------------------- | ---------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Mode izin                                             | Semua mode termasuk `dontAsk`                                    | Manual, Terima edits, Plan, dan Auto. Bypass permissions muncul di pemilih mode setelah diaktifkan: melalui toggle Settings di paket Pro dan Max, atau melalui kebijakan organisasi di paket Team dan Enterprise                                                                                                                                                            |
| `--dangerously-skip-permissions`                      | Flag CLI                                                         | Bypass permissions mode. Di paket Pro dan Max, aktifkan di Settings → Claude Code → "Allow bypass permissions mode"; di paket Team dan Enterprise, kebijakan organisasi mengontrolnya                                                                                                                                                                                       |
| [Penyedia pihak ketiga](/docs/id/third-party-integrations) | Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry | API Anthropic secara default. Untuk perutean gateway, lihat [hubungkan aplikasi desktop ke gateway](/docs/id/llm-gateway-connect#desktop-app). Untuk menjalankan tab Code di Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry, atau gateway LLM yang di-host sendiri, lihat [Claude Desktop on 3P](https://claude.com/docs/third-party/claude-desktop/overview). |
| [MCP servers](/docs/id/mcp)                                | Konfigurasikan di file pengaturan                                | UI Connectors untuk sesi lokal dan SSH, atau file pengaturan                                                                                                                                                                                                                                                                                                                |
| [Plugins](/docs/id/plugins)                                | Perintah `/plugin`                                               | UI plugin manager                                                                                                                                                                                                                                                                                                                                                           |
| File @mention                                         | Berbasis teks                                                    | Dengan autocomplete; hanya sesi lokal dan SSH                                                                                                                                                                                                                                                                                                                               |
| Lampiran file                                         | Tidak tersedia                                                   | Gambar, PDF                                                                                                                                                                                                                                                                                                                                                                 |
| Isolasi sesi                                          | Flag [`--worktree`](/docs/id/cli-reference)                           | Worktrees otomatis                                                                                                                                                                                                                                                                                                                                                          |
| Sesi ganda                                            | Terminal terpisah                                                | Tab sidebar                                                                                                                                                                                                                                                                                                                                                                 |
| Tugas berulang                                        | Cron jobs, CI pipelines                                          | [Tugas terjadwal](/docs/id/desktop-scheduled-tasks)                                                                                                                                                                                                                                                                                                                              |
| Penggunaan komputer                                   | [Aktifkan via `/mcp`](/docs/id/computer-use) di macOS                 | [Kontrol aplikasi dan layar](#let-claude-use-your-computer) di macOS dan Windows                                                                                                                                                                                                                                                                                            |
| Integrasi Dispatch                                    | Tidak tersedia                                                   | [Sesi Dispatch](#sessions-from-dispatch) di sidebar                                                                                                                                                                                                                                                                                                                         |
| Scripting dan otomasi                                 | [`--print`](/docs/id/cli-reference), [Agent SDK](/docs/id/headless)        | Tidak tersedia                                                                                                                                                                                                                                                                                                                                                              |

<h3 id="what’s-not-available-in-desktop">
  Apa yang tidak tersedia di Desktop
</h3>

Fitur berikut hanya tersedia di CLI atau ekstensi VS Code, kecuali jika dicatat:

* **Penyedia pihak ketiga**: Desktop terhubung ke API Anthropic secara default. Untuk merutekan Desktop melalui gateway, lihat [hubungkan aplikasi desktop ke gateway](/docs/id/llm-gateway-connect#desktop-app). Penerapan enterprise dapat mengonfigurasi Google Cloud's Agent Platform dan penyedia gateway melalui [pengaturan terkelola](https://claude.com/docs/third-party/claude-desktop/configuration). Untuk Amazon Bedrock atau Microsoft Foundry di CLI, lihat [quickstart](/docs/id/quickstart). Sebagai pengecualian terhadap bagian di atas, [Claude Desktop on 3P](https://claude.com/docs/third-party/claude-desktop/overview) menjalankan tab Code di Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry, atau gateway LLM yang di-host sendiri.
* **Linux (beta)**: Computer Use belum tersedia di aplikasi desktop Linux. Lihat [Claude Desktop on Linux](/docs/id/desktop-linux).
* **Saran kode inline**: Desktop tidak menyediakan saran gaya autocomplete. Ini bekerja melalui prompt percakapan dan perubahan kode eksplisit.
* **Tim agent**: sesi Claude Code paralel yang saling berkirim pesan tersedia di [CLI](/docs/id/agent-teams), bukan di Desktop. Untuk pekerjaan multi-agent di dalam satu sesi, gunakan [dynamic workflows](/docs/id/workflows), yang berjalan di Desktop.
* **Perintah terminal-dialog**: perintah bawaan yang membuka panel interaktif di terminal berperilaku berbeda di tab Code. Edit [file pengaturan](/docs/id/settings) secara langsung untuk mengelola aturan izin dan konfigurasi, atau jalankan perintah dari CLI mandiri.
  * Perintah tanpa bentuk argumen, seperti `/permissions`, membalas dengan `isn't available in this environment`.
  * `/config` membuka Settings → Claude Code. Teks setelah perintah diabaikan, jadi `/config theme=dark` tidak mengatur tema.

<h2 id="troubleshooting">
  Troubleshooting
</h2>

Bagian di bawah mencakup masalah khusus untuk aplikasi desktop. Untuk kesalahan API runtime yang muncul di chat seperti `API Error: 500`, `529 Overloaded`, `429`, atau `Prompt is too long`, lihat [Error reference](/docs/id/errors). Kesalahan tersebut dan perbaikannya sama di CLI, desktop, dan web.

<h3 id="check-your-version">
  Check your version
</h3>

Untuk melihat versi aplikasi desktop yang Anda jalankan:

* **macOS**: klik **Claude** di menu bar, kemudian **About Claude**
* **Windows**: klik **Help**, kemudian **About**

Klik nomor versi untuk menyalinnya ke clipboard Anda.

<h3 id="403-or-authentication-errors-in-the-code-tab">
  403 or authentication errors in the Code tab
</h3>

Jika Anda melihat `Error 403: Forbidden` atau kegagalan autentikasi lainnya saat menggunakan tab Code:

1. Keluar dan masuk kembali dari menu aplikasi. Ini adalah perbaikan paling umum.
2. Verifikasi Anda memiliki langganan berbayar aktif: Pro, Max, Team, atau Enterprise.
3. Jika CLI berfungsi tetapi Desktop tidak, keluar dari aplikasi desktop sepenuhnya, bukan hanya tutup jendela, kemudian buka kembali dan masuk.
4. Periksa koneksi internet dan pengaturan proxy Anda.

<h3 id="blank-or-stuck-screen-on-launch">
  Blank or stuck screen on launch
</h3>

Jika aplikasi terbuka tetapi menampilkan layar kosong atau tidak responsif:

1. Mulai ulang aplikasi.
2. Periksa pembaruan yang tertunda. Pada macOS dan Windows, aplikasi secara otomatis memperbarui saat peluncuran; di Linux, perbarui melalui apt seperti yang dijelaskan dalam [Claude Desktop on Linux](/docs/id/desktop-linux).
3. Di jaringan yang dikelola, konfirmkan firewall Anda memungkinkan host CDN dalam [network access requirements](#network-access-requirements).
4. Di Windows, periksa Event Viewer untuk log crash di bawah **Windows Logs → Application**.

<h3 id="failed-to-load-session">
  "Failed to load session"
</h3>

Jika Anda melihat `Failed to load session`, folder yang dipilih mungkin tidak lagi ada, repositori Git mungkin memerlukan Git LFS yang tidak diinstal, atau izin file mungkin mencegah akses. Coba pilih folder berbeda atau mulai ulang aplikasi.

<h3 id="session-not-finding-installed-tools">
  Session not finding installed tools
</h3>

Jika Claude tidak dapat menemukan alat seperti `npm`, `node`, atau perintah CLI lainnya, verifikasi alat bekerja di terminal biasa Anda, periksa bahwa profil shell Anda dengan benar menyiapkan PATH, dan mulai ulang aplikasi desktop untuk memuat ulang variabel lingkungan.

<h3 id="git-and-git-lfs-errors">
  Git and Git LFS errors
</h3>

Di Windows, Git diperlukan agar tab Code memulai sesi lokal. Jika Anda melihat "Git is required," instal [Git for Windows](https://git-scm.com/downloads/win) dan mulai ulang aplikasi.

Jika Anda melihat "Git LFS is required by this repository but is not installed," instal Git LFS dari [git-lfs.com](https://git-lfs.com/), jalankan `git lfs install`, dan mulai ulang aplikasi.

<h3 id="mcp-servers-not-working-on-windows">
  MCP servers not working on Windows
</h3>

Jika toggle MCP server tidak merespons atau server gagal terhubung di Windows, periksa bahwa server dikonfigurasi dengan benar di pengaturan Anda, mulai ulang aplikasi, verifikasi proses server berjalan di Task Manager, dan tinjau log server untuk kesalahan koneksi.

<h3 id="app-won’t-quit">
  App won't quit
</h3>

* **macOS**: tekan Cmd+Q. Jika aplikasi tidak merespons, gunakan Force Quit dengan Cmd+Option+Esc, pilih Claude, dan klik Force Quit.
* **Windows**: gunakan Task Manager dengan Ctrl+Shift+Esc untuk mengakhiri proses Claude.

<h3 id="windows-specific-issues">
  Windows-specific issues
</h3>

* **PATH not updated after install**: buka jendela terminal baru. Pembaruan PATH hanya berlaku untuk sesi terminal baru.
* **Concurrent installation error**: jika Anda melihat kesalahan tentang instalasi lain sedang berlangsung tetapi tidak ada, coba jalankan installer sebagai Administrator.

<h3 id="branch-doesn’t-exist-yet-when-opening-in-cli">
  "Branch doesn't exist yet" when opening in CLI
</h3>

Sesi cloud dapat membuat cabang yang tidak ada di mesin lokal Anda. Klik nama cabang di toolbar sesi untuk menyalinnya, kemudian ambil secara lokal:

```bash theme={null}
git fetch origin <branch-name>
git checkout <branch-name>
```

<h3 id="still-stuck">
  Still stuck?
</h3>

* Buka Help → Get Support di aplikasi desktop, atau kunjungi [Claude support center](https://support.claude.com/) secara langsung
* Untuk masalah yang juga muncul kembali di CLI `claude` standalone, cari atau laporkan bug di [GitHub Issues](https://github.com/anthropics/claude-code/issues)

Saat melaporkan masalah, sertakan versi aplikasi desktop Anda, sistem operasi Anda, pesan kesalahan yang tepat, dan log yang relevan. Di macOS, periksa Console.app. Di Windows, periksa Event Viewer → Windows Logs → Application.
