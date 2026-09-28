> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Gunakan Claude Code di cloud

> Jalankan sesi Claude Code di cloud dari browser, ponsel, aplikasi desktop, atau terminal Anda, pindahkan dengan --cloud dan --teleport, dan auto-fix pull request.

<Note>
  Sesi cloud tersedia pada paket Pro, Max, dan Team, serta untuk pengguna Enterprise dengan kursi premium atau kursi Chat + Claude Code.
</Note>

Sesi cloud adalah sesi Claude Code yang berjalan pada infrastruktur cloud alih-alih pada mesin Anda. Secara default, sesi berjalan pada infrastruktur yang dikelola Anthropic, atau pada [lingkungan self-hosted](/docs/id/self-hosted-environments) organisasi Anda saat dialihkan ke sana. Sesi terus berjalan setelah Anda menutup laptop, dan Anda dapat memeriksanya atau mengarahkannya dari perangkat apa pun.

Anda dapat memulai sesi cloud dari salah satu permukaan ini:

* **Browser**: [claude.ai/code](https://claude.ai/code), juga disebut Claude Code di web
* **Mobile**: tab **Code** di [aplikasi Claude](/docs/id/mobile)
* **Aplikasi desktop**: pilih **Cloud** alih-alih **Local** saat Anda [memulai sesi](/docs/id/desktop#run-long-running-tasks-in-the-cloud)
* **Terminal**: [`claude --cloud`](#from-terminal-to-cloud)
* **Routines**: [penjadwalan dan pemicu berjalan](/docs/id/routines) masing-masing berjalan sebagai sesi cloud

Untuk membuat Claude memulai dan melacak banyak sesi cloud untuk satu badan pekerjaan, gunakan [proyek](/docs/id/claude-projects). Sesi di terminal Anda, IDE Anda, atau aplikasi Desktop dengan **Local** yang dipilih berjalan pada mesin Anda sendiri. Untuk mengarahkan salah satu sesi lokal tersebut dari ponsel atau browser Anda, gunakan [Remote Control](/docs/id/remote-control).

<Tip>
  Baru mengenal sesi cloud? Mulai dengan [Memulai](/docs/id/web-quickstart) untuk menghubungkan akun GitHub Anda dan mengirimkan tugas pertama Anda.
</Tip>

Halaman ini mencakup:

* [Lingkungan cloud](#cloud-environments): tempat sesi berjalan, dan tempat untuk mengonfigurasinya
* [Opsi autentikasi GitHub](#github-authentication-options): dua cara untuk menghubungkan GitHub
* [Pindahkan tugas antara terminal dan cloud](#move-tasks-between-terminal-and-cloud) dengan `--cloud` dan `--teleport`
* [Bekerja dengan sesi](#work-with-sessions): mode izin, meninjau, berbagi, mengarsipkan, menghapus
* [Auto-fix pull request](#auto-fix-pull-requests): merespons secara otomatis kegagalan CI dan komentar ulasan
* [Keamanan dan isolasi](#security-and-isolation): bagaimana sesi diisolasi
* [Batasan](#limitations): batas laju dan pembatasan platform

<h2 id="cloud-environments">
  Lingkungan cloud
</h2>

Setiap sesi cloud berjalan dalam [lingkungan cloud](/docs/id/cloud-environments), konfigurasi yang disimpan yang mengontrol akses jaringan, variabel lingkungan, dan skrip setup. Jika Anda belum memiliki lingkungan, onboarding menyiapkan lingkungan **Default** dengan akses jaringan [**Trusted**](/docs/id/cloud-environments#access-levels), baik dengan membuatnya untuk Anda atau dengan meminta Anda membuatnya. Lihat [Lingkungan Default](/docs/id/cloud-environments#the-default-environment) untuk mengetahui mana yang terjadi pada paket Anda dan bagaimana sesi memilih lingkungan saat Anda memiliki lebih dari satu.

Lingkungan yang sama berlaku di mana pun Anda memulai sesi cloud: web, terminal, [Claude Tag](https://claude.com/docs/claude-tag/overview), [routines](/docs/id/routines), dan aplikasi mobile dan Desktop. Sesi saluran Claude Tag menggunakan lingkungan tingkat organisasi saja, baik [lingkungan bersama](/docs/id/cloud-environments#organization-shared-environments) atau [lingkungan self-hosted](/docs/id/self-hosted-environments).

Lihat [Konfigurasi lingkungan cloud](/docs/id/cloud-environments) untuk mengubah apa yang diizinkan lingkungan, menetapkan variabel, atau menambahkan skrip setup, dan [Alat yang diinstal](/docs/id/cloud-environments#installed-tools) untuk apa yang disertakan sesi tanpa konfigurasi apa pun.

<h2 id="github-authentication-options">
  Opsi autentikasi GitHub
</h2>

Sesi cloud memerlukan akses ke repositori GitHub Anda untuk mengkloning kode dan mendorong cabang. Anda dapat memberikan akses dengan dua cara:

| Metode           | Cara Anda terhubung                                                                                 | Repositori yang dapat dijangkau sesi                                                                | Terbaik untuk                                                                 |
| :--------------- | :-------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------- |
| **GitHub App**   | Otorisasi Claude GitHub App selama [onboarding web](/docs/id/web-quickstart)                             | Repositori publik apa pun, dan repositori pribadi tempat Claude GitHub App diinstal                 | Onboarding browser; tim yang menginginkan [Auto-fix](#auto-fix-pull-requests) |
| **`/web-setup`** | Jalankan `/web-setup` di terminal Anda untuk mengirim token CLI `gh` lokal Anda ke akun Claude Anda | Repositori apa pun yang dapat diakses token `gh` Anda, terlepas dari apakah App diinstal atau tidak | Pengembang individual yang sudah menggunakan `gh`                             |

Menginstal Claude GitHub App di repositori juga mengaktifkan [Auto-fix](#auto-fix-pull-requests) untuk pull request di dalamnya.

Thread dalam [project](/docs/id/claude-projects) memerlukan Claude GitHub App diinstal di setiap repositori yang mereka kloning, metode mana pun yang Anda gunakan untuk terhubung. Lihat [Set up GitHub access](/docs/id/claude-projects#set-up-github-access).

Untuk cara `/schedule` memeriksa akses repositori sebelum membuat routine, lihat [Repositories and branch permissions](/docs/id/routines#repositories-and-branch-permissions). Lihat [Connect from your terminal](/docs/id/web-quickstart#connect-from-your-terminal) untuk panduan `/web-setup`, termasuk apa yang disimpan `/web-setup` dan cara menghapusnya.

Quick web setup adalah pengaturan organisasi yang memungkinkan anggota menghubungkan GitHub dengan `/web-setup`, melewati prompt instalasi Claude GitHub App selama onboarding browser, dan membuat onboarding browser membuat lingkungan [**Default**](/docs/id/cloud-environments#the-default-environment) untuk mereka alih-alih menampilkan formulir lingkungan. Pada paket Team dan Enterprise, ini dimatikan secara default, yang menyembunyikan `/web-setup`. [Owner](/docs/id/server-managed-settings#access-control) mengaktifkannya dengan toggle **Quick web setup** di [**Admin settings > Claude Code**](https://claude.ai/admin-settings/claude-code).

<Note>
  Organisasi dengan [Zero Data Retention](/docs/id/zero-data-retention) yang diaktifkan tidak dapat menggunakan `/web-setup` atau fitur sesi cloud lainnya.
</Note>

<h2 id="move-tasks-between-terminal-and-cloud">
  Memindahkan tugas antara terminal dan cloud
</h2>

Alur kerja ini memerlukan [Claude Code CLI](/docs/id/quickstart) yang masuk ke akun claude.ai yang sama. Anda dapat memulai sesi cloud baru dari terminal Anda, atau menarik sesi cloud ke terminal Anda untuk melanjutkan secara lokal. Sesi cloud tetap ada bahkan jika Anda menutup laptop, dan Anda dapat memantaunya dari mana saja termasuk aplikasi mobile Claude.

<Note>
  Dari CLI, handoff sesi adalah satu arah: Anda dapat menarik sesi cloud ke terminal Anda dengan `--teleport`, tetapi Anda tidak dapat mendorong sesi terminal yang sudah ada ke cloud. Flag `--cloud` dengan deskripsi tugas membuat sesi cloud baru untuk repositori Anda saat ini; dengan `-p` dan ID sesi atau URL claude.ai/code, flag tersebut malah [antri pesan ke sesi yang sudah ada itu](/docs/id/claude-code-on-the-web#send-follow-ups-from-the-cli). [Aplikasi Desktop](/docs/id/desktop#continue-in-another-surface) menyediakan menu **Continue in** yang dapat mengirim sesi lokal ke cloud.
</Note>

<h3 id="from-terminal-to-cloud">
  Dari terminal ke cloud
</h3>

Mulai sesi cloud dari baris perintah dengan flag `--cloud`:

```bash theme={null}
claude --cloud "Fix the authentication bug in src/auth/login.ts"
```

Ini membuat sesi cloud baru di claude.ai. VM cloud mengkloning remote GitHub direktori Anda saat ini di cabang Anda saat ini, bukan checkout lokal Anda, jadi dorong terlebih dahulu jika Anda memiliki commit lokal. Lihat [Mengirim repositori lokal tanpa GitHub](#send-local-repositories-without-github) untuk kasus di mana Claude Code mengunggah repositori lokal Anda alih-alih mengkloning.

`--cloud` bekerja dengan satu repositori pada satu waktu. Tugas berjalan di cloud sementara Anda terus bekerja secara lokal. Ejaan `--remote` yang lebih lama masih berfungsi sebagai alias yang sudah usang untuk `--cloud`.

Saat kontainer cloud dimulai, CLI menampilkan daftar periksa langsung dari langkah-langkah penyiapan, seperti mengkloning repositori dan menjalankan [skrip penyiapan Anda](/docs/id/cloud-environments#setup-scripts). Ini antri pesan yang Anda ketik selama penyediaan dan mengirimnya setelah sesi siap.

<Note>
  `--cloud` membuat sesi cloud. `--remote-control` tidak terkait: ini memungkinkan Anda memantau dan mengarahkan sesi CLI lokal dari claude.ai atau aplikasi Claude. Lihat [Remote Control](/docs/id/remote-control).
</Note>

Buka sesi di claude.ai atau aplikasi mobile Claude untuk memeriksa kemajuan atau berinteraksi langsung. Dari sana Anda dapat mengarahkan Claude, memberikan umpan balik, atau menjawab pertanyaan seperti dalam percakapan lainnya.

Jika Claude mengajukan pertanyaan dan sesi tetap menganggur, Anda masih dapat menjawab saat Anda kembali, hingga [kedaluwarsa lingkungan](#environment-expired), dan sesi berlanjut dari jawaban Anda.

<h4 id="tips-for-cloud-tasks">
  Tips untuk tugas cloud
</h4>

**Rencanakan secara lokal, jalankan di cloud**: untuk tugas yang kompleks, mulai Claude dalam mode rencana untuk berkolaborasi pada pendekatan, kemudian kirim pekerjaan ke cloud:

```bash theme={null}
claude --permission-mode plan
```

Dalam mode rencana, Claude membaca file, menjalankan perintah untuk menjelajahi, dan mengusulkan rencana tanpa mengedit kode sumber. Setelah Anda puas, simpan rencana ke repo, commit, dan dorong sehingga VM cloud dapat mengklonnya. Kemudian mulai sesi cloud untuk eksekusi otonom:

```bash theme={null}
claude --cloud "Execute the migration plan in docs/migration-plan.md"
```

**Jalankan tugas secara paralel**: setiap perintah `--cloud` membuat sesi cloud sendiri yang berjalan secara independen. Anda dapat memulai beberapa tugas dan semuanya akan berjalan secara bersamaan dalam sesi terpisah:

```bash theme={null}
claude --cloud "Fix the flaky test in auth.spec.ts"
claude --cloud "Update the API documentation"
claude --cloud "Refactor the logger to use structured output"
```

Ketika sesi selesai, Anda dapat membuat PR dari claude.ai/code atau [teleport](#from-cloud-to-terminal) sesi ke terminal Anda untuk melanjutkan bekerja.

<h4 id="send-local-repositories-without-github">
  Mengirim repositori lokal tanpa GitHub
</h4>

Ketika Anda menjalankan `claude --cloud` dari repositori yang tidak memiliki remote git, atau dari repositori github.com yang Claude GitHub App tidak diinstal, Claude Code membundel repositori lokal Anda dan mengunggahnya langsung ke sesi cloud. Ini berlaku bahkan jika Anda menghubungkan GitHub dengan `/web-setup`. Bundle mencakup riwayat repositori lengkap Anda di semua cabang, ditambah perubahan yang tidak dicommit ke file yang dilacak.

Di macOS, Linux, dan WSL, Claude Code meninggalkan perubahan yang tidak dicommit ke file bernama seperti kredensial atau kunci dari unggahan dan memberi nama file yang ditinggalkannya. Ini mencakup file `.env`, file Terraform `*.tfvars`, dan file kunci seperti `id_rsa` dan `*.pem`. Sesi dimulai dengan versi yang dicommit dari masing-masing, atau tanpa file jika tidak ada yang dicommit. Dalam worktree tertaut, submodul, atau tata letak serupa, Claude Code mengunggah perubahan ini dengan sisanya dan memberi nama file yang diunggahnya.

Untuk mengunggah bundle bahkan ketika Claude Code akan mengkloning dari remote, atur `CCR_FORCE_BUNDLE=1`:

```bash theme={null}
CCR_FORCE_BUNDLE=1 claude --cloud "Run the test suite and fix any failures"
```

Repositori bundel harus memenuhi batas-batas ini:

* Direktori harus berupa repositori git dengan setidaknya satu commit
* Repositori bundel harus di bawah 100 MB. Repositori yang lebih besar kembali ke bundling hanya cabang saat ini, kemudian ke snapshot pohon kerja yang dikompres tunggal, dan gagal jika snapshot masih terlalu besar
* File yang tidak dilacak tidak disertakan; jalankan `git add` pada file yang ingin dilihat sesi cloud
* Sesi yang dibuat dari bundle dapat mendorong kembali ke remote GitHub hanya ketika [koneksi GitHub](#github-authentication-options) Anda memiliki akses push ke repositori itu

<h3 id="send-follow-ups-from-the-cli">
  Mengirim tindak lanjut dari CLI
</h3>

Setelah sesi cloud berjalan, di mana pun sesi itu dijalankan, kirimkan pesan tindak lanjut kepadanya dari CLI `claude` di mesin mana pun tempat Anda masuk dengan `claude auth login`. CLI mengautentikasi dengan kredensial akun Anthropic Anda dan tidak mengirim status sesi lokal, jadi perintah tidak perlu berjalan dari mesin yang memulai sesi, dan sama di setiap shell, termasuk PowerShell.

Perintah memposting satu pesan dan keluar:

```bash theme={null}
claude -p "your message" --cloud <session-id>
```

CLI antri pesan ke dalam sesi dan keluar tanpa menunggu balasan. Gunakan untuk mengarahkan sesi yang berjalan lama, antri langkah berikutnya sementara yang saat ini masih selesai, atau kirim tindak lanjut dari [skrip CI](/docs/id/self-hosted-environments-testing#run-the-test-loop). Anda juga dapat menyalurkan pesan pada stdin alih-alih meneruskannya sebagai argumen: `echo "your message" | claude -p --cloud <session-id>`.

Untuk `<session-id>`, teruskan ID telanjang, seperti `session_...` atau `cse_...`, atau URL `claude.ai/code/<id>` sesi, dengan atau tanpa skema atau string kueri. Temukan ID dalam daftar sesi Anda di claude.ai/code.

<Note>
  `--cloud` memerlukan akun Anthropic. Ini tidak tersedia ketika Claude Code dikonfigurasi untuk Amazon Bedrock, Google Cloud's Agent Platform, atau penyedia pihak ketiga lainnya. [Gateway LLM](/docs/id/llm-gateway) yang dikonfigurasi hanya melalui `ANTHROPIC_BASE_URL` tidak dihitung sebagai penyedia pihak ketiga untuk pemeriksaan ini, tetapi Anda masih perlu masuk dengan `claude auth login`. Kebijakan `allow_remote_sessions` organisasi Anda juga harus diaktifkan. Pemilik dapat mengaktifkannya di pengaturan admin Claude Code di claude.ai/admin-settings/claude-code.
</Note>

<h4 id="output-and-errors">
  Output dan kesalahan
</h4>

Saat berhasil, perintah mencetak ID sesi dan tautan untuk melihat sesi:

```
Sent to cloud session.
Session ID: session_01DiUkqY2kzbUbDmW1w96rfi
View: https://claude.ai/code/session_01DiUkqY2kzbUbDmW1w96rfi?from=cli&m=0
```

Teruskan `--output-format json` untuk hasil yang dapat dibaca mesin: `{ok, session_id, url}` saat berhasil, atau `{ok: false, session_id, error}` ketika pengiriman gagal, misalnya ketika sesi hilang atau diarsipkan. Kesalahan konfigurasi, seperti penyedia yang tidak didukung atau kebijakan organisasi yang dinonaktifkan, dicetak ke stderr tanpa JSON. `--output-format stream-json` tidak didukung dengan `--cloud <session-id>`.

CLI memberi awalan kesalahan dengan `Error: `. Pengiriman yang gagal dibungkus sebagai `failed to send message to cloud session <id>: <reason>`.

| Pesan                                                                                                                       | Artinya                                                                                                                                                                                                                                                                                                                                    |
| --------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `Cloud sessions aren't available with <provider>. They run on Anthropic's infrastructure and require an Anthropic account.` | Claude Code dikonfigurasi untuk penyedia pihak ketiga. Pesan menyebutkan penyedia dengan label yang digunakan konfigurasi Anda, seperti `Amazon Bedrock` atau `Google Vertex AI`. Hapus konfigurasi penyedia itu, misalnya dengan membatalkan pengaturan `CLAUDE_CODE_USE_BEDROCK`, dan masuk dengan akun Anthropic (`claude auth login`). |
| `Cloud sessions are disabled by your organization's policy. Contact your organization admin to enable them.`                | Kebijakan organisasi `allow_remote_sessions` mati.                                                                                                                                                                                                                                                                                         |
| `Couldn't verify your organization's policy for cloud sessions. Check your network connection and try again.`               | Claude Code tidak dapat mengambil kebijakan organisasi Anda, jadi menolak pengiriman daripada menganggap sesi cloud diizinkan. Periksa koneksi jaringan Anda dan coba lagi.                                                                                                                                                                |
| `Attaching to an existing cloud session is not enabled for your account.`                                                   | Anda menjalankan `--cloud <session-id>` tanpa `-p`. Kirim pesan dengan `claude -p "your message" --cloud <session-id>`.                                                                                                                                                                                                                    |
| `Session not found: <id>`                                                                                                   | ID atau URL tidak cocok dengan sesi yang dapat Anda akses. Periksa terhadap URL claude.ai/code sesi.                                                                                                                                                                                                                                       |
| `cloud session <id> is archived and cannot accept new messages`                                                             | Sesi telah diarsipkan. Mulai sesi baru sebagai gantinya.                                                                                                                                                                                                                                                                                   |

<h3 id="from-cloud-to-terminal">
  Dari cloud ke terminal
</h3>

Tarik sesi cloud ke terminal Anda menggunakan salah satu dari ini:

* **Menggunakan `--teleport`**: dari baris perintah, jalankan `claude --teleport` untuk pemilih sesi interaktif, atau `claude --teleport <session-id>` untuk melanjutkan sesi tertentu secara langsung. Jika Anda memiliki perubahan yang tidak dicommit, Anda akan diminta untuk menyimpannya terlebih dahulu.
* **Menggunakan `/teleport`**: di dalam sesi CLI yang sudah ada, jalankan `/teleport` atau `/tp` untuk membuka pemilih sesi yang sama tanpa memulai ulang Claude Code.
* **Dari `/tasks`**: jalankan `/tasks` untuk melihat sesi latar belakang Anda, kemudian tekan `t` untuk teleport ke dalamnya.
* **Dari claude.ai/code**: pilih **Open in > Terminal** dari menu sesi untuk menyalin perintah yang dapat Anda tempel ke terminal Anda.
* **Dari dalam sesi cloud**: ketik `/teleport` dan Claude Code membalas dengan perintah `claude --teleport <session-id>` yang tepat untuk sesi itu, siap dijalankan dari checkout repositori. Memerlukan Claude Code v2.1.223 atau lebih baru di lingkungan sesi.

Ketika Anda teleport sesi, Claude memverifikasi Anda berada di repositori yang benar, mengambil dan checkout cabang dari sesi cloud, dan memuat riwayat percakapan lengkap ke terminal Anda. Terminal mendapatkan salinannya sendiri dari sesi: pekerjaan baru di sana tetap lokal dan tidak muncul di sesi cloud di claude.ai atau aplikasi mobile Claude. Untuk terus mengarahkan dari ponsel Anda setelah teleport, mulai [`/remote-control`](/docs/id/remote-control) dalam sesi lokal.

`--teleport` berbeda dari `--resume`. `--resume` membuka kembali percakapan dari riwayat lokal mesin ini dan tidak mencantumkan sesi cloud; `--teleport` menarik sesi cloud dan cabangnya.

<h4 id="teleport-requirements">
  Persyaratan Teleport
</h4>

Teleport memeriksa persyaratan ini sebelum melanjutkan sesi. Jika ada persyaratan yang tidak terpenuhi, Anda akan melihat kesalahan atau diminta untuk menyelesaikan masalah.

| Persyaratan           | Detail                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| --------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Status git bersih     | Direktori kerja Anda tidak boleh memiliki perubahan yang tidak dicommit. Teleport meminta Anda untuk menyimpan perubahan jika diperlukan.                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| Repositori yang benar | Anda harus menjalankan `--teleport` dari checkout repositori yang sama, bukan fork. Jika Anda menjalankannya dari checkout repositori yang berbeda, Claude Code menampilkan kesalahan yang menyebutkan repositori sesi dan checkout Anda. Sebelum v2.1.219, kesalahan tidak menyebutkan repositori checkout Anda. Jika Claude Code tidak dapat mengurai remote Anda menjadi nama host, misalnya alias host SSH seperti `git@work:owner/repo.git`, Claude Code meminta Anda untuk mengonfirmasi, dan menerima checkout ketika pemilik remote dan nama repositori cocok dengan repositori sesi. |
| Cabang tersedia       | Cabang dari sesi cloud harus telah didorong ke remote. Teleport secara otomatis mengambil dan checkout.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| Akun yang sama        | Anda harus diautentikasi ke akun claude.ai yang sama yang digunakan dalam sesi cloud.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |

<h4 id="teleport-is-unavailable">
  `--teleport` tidak tersedia
</h4>

Teleport memerlukan autentikasi langganan claude.ai. Jika Anda diautentikasi melalui kunci API, jalankan `/login` untuk masuk dengan akun claude.ai Anda sebagai gantinya. Jika kesalahan menyebutkan penyedia Anda, sesi cloud tidak tersedia melalui penyedia pihak ketiga; lihat [tabel kesalahan](#output-and-errors). Jika Anda sudah masuk melalui claude.ai dan `--teleport` masih tidak tersedia, organisasi Anda mungkin telah menonaktifkan sesi cloud.

<h2 id="work-with-sessions">
  Bekerja dengan sesi
</h2>

Sesi muncul di bilah samping di claude.ai/code. Dari sana Anda dapat meninjau perubahan, berbagi dengan rekan kerja, mengarsipkan pekerjaan yang selesai, atau menghapus sesi secara permanen.

<h3 id="take-back-a-queued-message">
  Ambil kembali pesan yang antri
</h3>

Jika Anda mengirim pesan saat Claude sedang bekerja, pesan akan antri hingga Claude membacanya. Untuk mengambil kembali pesan yang antri, klik ✕ di atasnya. Teks kembali ke kotak pesan sehingga Anda dapat mengeditnya atau mengirim sesuatu yang lain.

Jika Claude sudah membaca pesan, pesan tetap ada dalam percakapan.

<h3 id="manage-context">
  Kelola konteks
</h3>

Sesi cloud mendukung [perintah bawaan](/docs/id/commands) yang menghasilkan keluaran teks. Perintah yang hanya berjalan di antarmuka terminal, seperti `/plugin` atau `/resume`, tidak tersedia. Perintah yang membuka pemilih atau panel di terminal berperilaku berbeda di sesi cloud:

* **`/model`, `/effort`, `/color`, dan `/rename`**: teruskan nilai sebagai argumen, misalnya `/model sonnet`, alih-alih membuka pemilih terminal atau slider. Bentuk argumen memerlukan Claude Code v2.1.205 atau lebih baru di lingkungan sesi dan mengikuti [catatan ketersediaan](/docs/id/commands#all-commands) setiap perintah.
* **`/fast`**: mengalihkan [mode cepat](/docs/id/fast-mode#use-fast-mode-in-cloud-sessions) untuk sesi ketika mode cepat [tersedia di akun Anda](/docs/id/fast-mode#requirements). Memerlukan Claude Code v2.1.271 atau lebih baru di lingkungan sesi.
* **`/config`**: di peramban Anda di claude.ai/code, membuka bagian Claude Code dari pengaturan Anda alih-alih menetapkan nilai, dan teks setelah perintah, termasuk `key=value`, diabaikan. Untuk mengubah pengaturan sesi cloud, atur [variabel lingkungan](/docs/id/cloud-environments#set-environment-variables) pada lingkungan, atau dalam sesi dengan satu repositori, komit kunci ke `.claude/settings.json` repositori tersebut. [Pengaturan di sesi cloud](/docs/id/settings#settings-in-cloud-sessions) mencantumkan apa yang dibaca setiap sesi.

Untuk manajemen konteks secara khusus:

| Perintah   | Berfungsi di sesi cloud | Catatan                                                                                                                   |
| :--------- | :---------------------- | :------------------------------------------------------------------------------------------------------------------------ |
| `/compact` | Ya                      | Merangkum percakapan untuk membebaskan konteks. Menerima instruksi fokus opsional seperti `/compact keep the test output` |
| `/context` | Ya                      | Menunjukkan apa yang saat ini ada di jendela konteks                                                                      |
| `/clear`   | Tidak                   | Mulai sesi baru dari bilah samping sebagai gantinya                                                                       |

Pemadatan otomatis berjalan secara otomatis ketika jendela konteks mendekati kapasitas. Sesi cloud menetapkan [`CLAUDE_AUTOCOMPACT_PCT_OVERRIDE`](/docs/id/env-vars) mereka sendiri, sehingga pemadatan dipicu di tengah [jendela auto-compact](/docs/id/model-config#set-the-auto-compact-window) daripada ketika jendela penuh. Nilai itu menggantikan nilai yang Anda tambahkan di [variabel lingkungan](/docs/id/cloud-environments#set-environment-variables) Anda, jadi menambahkan variabel di sana tidak mengubah kapan pemadatan dipicu.

Untuk mengubah jendela auto-compact sebagai gantinya, atur [`CLAUDE_CODE_AUTO_COMPACT_WINDOW`](/docs/id/env-vars) di variabel lingkungan Anda, atau jalankan [`/autocompact`](/docs/id/commands#all-commands) dengan hitungan token di sesi tempat variabel tidak diatur.

[Subagents](/docs/id/sub-agents) bekerja dengan cara yang sama seperti yang mereka lakukan secara lokal. Claude dapat menelurkan mereka dengan alat Agent untuk memindahkan penelitian atau pekerjaan paralel ke jendela konteks terpisah, menjaga percakapan utama lebih ringan. Subagents yang ditentukan di `.claude/agents/` repo Anda diambil secara otomatis.

[Agent teams](/docs/id/agent-teams) dimatikan secara default tetapi dapat diaktifkan dengan menambahkan `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS=1` ke [variabel lingkungan](/docs/id/cloud-environments#set-environment-variables) Anda.

<h3 id="permission-modes-in-cloud-sessions">
  Mode izin di sesi cloud
</h3>

Anda memilih [mode izin](/docs/id/permission-modes) sesi cloud dari [dropdown mode](/docs/id/permission-modes#switch-permission-modes), baik ketika Anda membuat tugas maupun saat sesi berjalan. Ketika Anda membuka kembali sesi yang [lingkungannya yang dihosting Anthropic telah kedaluwarsa](#environment-expired), atau mengirim pesan ke sesi yang [runner yang dihosting sendiri dirilis saat menganggur](/docs/id/self-hosted-environments-reference#runner-cli-flags), Claude Code melanjutkan sesi dalam mode izin yang sedang berlaku.

<h3 id="review-changes">
  Tinjau perubahan
</h3>

Setiap sesi menampilkan indikator diff dengan baris yang ditambahkan dan dihapus, seperti `+42 -18`. Pilihnya untuk membuka tampilan diff, tinggalkan komentar sebaris pada baris tertentu, dan kirimkan ke Claude dengan pesan berikutnya.

Tampilan diff membandingkan perubahan sesi dengan cabang dasarnya secara default. Untuk membandingkan dengan cabang lain mana pun di repositori, pilih **Compare against** dan pilih salah satu.

Claude Code menghitung diff ini, termasuk diff per-file yang ditampilkan saat Claude mengedit, dari konten blob git mentah, sehingga driver diff dan filter `textconv` yang dikonfigurasi di repositori tidak berlaku. Untuk file di repositori yang bukan salah satu checkout sesi itu sendiri, seperti yang dikloning di dalam workspace selama sesi, diff per-file menunjukkan edit Claude itu sendiri daripada perbandingan git.

Lihat [Review and iterate](/docs/id/web-quickstart#review-and-iterate) untuk panduan lengkap termasuk pembuatan PR. Untuk membuat Claude memantau PR untuk kegagalan CI dan komentar ulasan secara otomatis, lihat [Auto-fix pull requests](#auto-fix-pull-requests).

<h3 id="share-sessions">
  Bagikan sesi
</h3>

Untuk berbagi sesi, alihkan visibilitasnya sesuai dengan jenis akun di bawah. Setelah itu, bagikan tautan sesi apa adanya. Penerima melihat status terbaru ketika mereka membuka tautan, tetapi tampilan mereka tidak diperbarui secara real-time.

<h4 id="share-from-an-enterprise-or-team-account">
  Bagikan dari akun Enterprise atau Team
</h4>

Untuk akun Enterprise dan Team, dua opsi visibilitas adalah **Private** dan **Team**. Visibilitas Team membuat sesi terlihat oleh anggota lain dari organisasi claude.ai Anda. Sesi [Claude in Slack](/docs/id/slack) secara otomatis dibagikan dengan visibilitas Team.

Verifikasi akses repositori diaktifkan secara default, berdasarkan akun GitHub yang terhubung ke akun penerima. Nama tampilan akun Anda terlihat oleh semua penerima dengan akses.

<h4 id="share-from-a-max-or-pro-account">
  Bagikan dari akun Max atau Pro
</h4>

Untuk akun Max dan Pro, dua opsi visibilitas adalah **Private** dan **Public**. Visibilitas Public membuat sesi terlihat oleh pengguna mana pun yang masuk ke claude.ai.

Periksa sesi Anda untuk konten sensitif sebelum berbagi. Sesi dapat berisi kode dan kredensial dari repositori GitHub pribadi. Verifikasi akses repositori tidak diaktifkan secara default.

Untuk memerlukan penerima memiliki akses repositori, atau untuk menyembunyikan nama Anda dari sesi bersama, buka [**Settings > Claude Code > Sharing settings**](https://claude.ai/settings/claude-code).

<h3 id="archive-sessions">
  Arsipkan sesi
</h3>

Anda dapat mengarsipkan sesi untuk menjaga daftar sesi Anda tetap terorganisir. Sesi yang diarsipkan disembunyikan dari daftar sesi default tetapi dapat dilihat dengan memfilter sesi yang diarsipkan.

Untuk mengarsipkan sesi, arahkan ke sesi di bilah samping dan pilih ikon arsip.

<h3 id="delete-sessions">
  Hapus sesi
</h3>

Menghapus sesi secara permanen menghapus sesi dan datanya. Tindakan ini tidak dapat dibatalkan. Anda dapat menghapus sesi dengan dua cara:

* **Dari bilah samping**: filter untuk sesi yang diarsipkan, kemudian arahkan ke sesi yang ingin Anda hapus dan pilih ikon hapus
* **Dari menu sesi**: buka sesi, pilih dropdown di sebelah judul sesi, dan pilih **Delete**

Anda akan diminta untuk mengonfirmasi sebelum sesi dihapus.

<h2 id="auto-fix-pull-requests">
  Auto-fix pull requests
</h2>

Claude dapat memantau pull request dan secara otomatis merespons kegagalan CI dan komentar ulasan. Claude berlangganan aktivitas GitHub di PR, dan ketika pemeriksaan gagal atau pengulas meninggalkan komentar, Claude menyelidiki dan mendorong perbaikan jika ada yang jelas.

<Note>
  Auto-fix memerlukan Claude GitHub App untuk diinstal di repositori Anda. Jika Anda belum melakukannya, instal dari [halaman GitHub App](https://github.com/apps/claude).
</Note>

Ada beberapa cara untuk mengaktifkan auto-fix tergantung di mana PR berasal dan perangkat apa yang Anda gunakan:

* **PR yang dibuat dalam sesi cloud**: buka sesi di claude.ai/code, buka bilah status CI, dan pilih **Auto-fix**
* **Dari terminal Anda**: jalankan [`/autofix-pr`](/docs/id/commands) saat berada di cabang PR. Claude Code mendeteksi PR terbuka dengan `gh`, menelurkan sesi cloud, dan mengaktifkan auto-fix dalam satu langkah
* **Dari aplikasi mobile**: beri tahu Claude untuk auto-fix PR, misalnya "watch this PR and fix any CI failures or review comments"
* **PR yang ada**: tempel URL PR ke sesi dan beri tahu Claude untuk auto-fix

Auto-fix adalah toggle per-PR. Untuk berhenti memantau, buka bilah status CI dalam sesi di claude.ai/code dan hapus toggle **Auto-fix**, atau beri tahu Claude untuk berhenti memantau PR.

<h3 id="how-claude-responds-to-pr-activity">
  Bagaimana Claude merespons aktivitas PR
</h3>

Saat auto-fix aktif, Claude menerima acara GitHub untuk PR termasuk komentar ulasan baru dan kegagalan pemeriksaan CI. Untuk setiap acara, Claude menyelidiki dan memutuskan cara melanjutkan:

* **Perbaikan yang jelas**: jika Claude yakin dengan perbaikan dan tidak bertentangan dengan instruksi sebelumnya, Claude membuat perubahan, mendorongnya, dan menjelaskan apa yang dilakukan dalam sesi
* **Permintaan yang ambigu**: jika komentar pengulas dapat diinterpretasikan dengan beberapa cara atau melibatkan sesuatu yang secara arsitektur signifikan, Claude bertanya kepada Anda sebelum bertindak
* **Acara duplikat atau tanpa tindakan**: jika acara adalah duplikat atau tidak memerlukan perubahan, Claude mencatatnya dalam sesi dan melanjutkan

GitHub tidak mengeluarkan webhook ketika cabang dasar maju dan membuat konflik penggabungan, jadi auto-fix tidak dapat bereaksi terhadap konflik dengan sendirinya. Untuk menyelesaikan konflik, buka sesi dan minta Claude untuk melakukan rebase.

Claude dapat membalas utas komentar ulasan di GitHub sebagai bagian dari penyelesaiannya. Balasan ini diposting menggunakan akun GitHub Anda, sehingga muncul di bawah nama pengguna Anda, tetapi setiap balasan diberi label sebagai berasal dari Claude Code sehingga pengulas tahu itu ditulis oleh agen dan bukan oleh Anda secara langsung.

<Warning>
  Jika repositori Anda menggunakan otomasi yang dipicu komentar seperti Atlantis, Terraform Cloud, atau GitHub Actions kustom yang berjalan pada acara `issue_comment`, ketahui bahwa Claude dapat membalas atas nama Anda, yang dapat memicu alur kerja tersebut. Tinjau otomasi repositori Anda sebelum mengaktifkan auto-fix, dan pertimbangkan untuk menonaktifkan auto-fix untuk repositori di mana komentar PR dapat menerapkan infrastruktur atau menjalankan operasi istimewa.
</Warning>

<h2 id="security-and-isolation">
  Keamanan dan isolasi
</h2>

Setiap sesi cloud dipisahkan dari mesin Anda dan dari sesi lain melalui beberapa lapisan:

* **Mesin virtual terisolasi**: setiap sesi berjalan di VM yang terisolasi dan dikelola Anthropic. Sesi yang organisasi Anda arahkan ke [lingkungan self-hosted](/docs/id/self-hosted-environments) berjalan pada infrastruktur Anda sendiri sebagai gantinya, di mana isolasi adalah tanggung jawab deployment Anda
* <span id="default-allowed-domains" />**Kontrol akses jaringan**: di lingkungan yang dihosting Anthropic, akses jaringan dibatasi secara default dan dapat dinonaktifkan. Lihat [Akses jaringan](/docs/id/cloud-environments#network-access) untuk tingkat akses, [domain yang diizinkan secara default](/docs/id/cloud-environments#default-allowed-domains), dan lalu lintas yang tidak melewati daftar izin. Di lingkungan self-hosted, Anda membatasi egress sesi di batas jaringan Anda sendiri. Saat berjalan dengan akses jaringan dinonaktifkan, Claude Code masih dapat berkomunikasi dengan API Anthropic, yang mungkin memungkinkan data keluar dari VM.
* **Perlindungan kredensial**: di lingkungan yang dihosting Anthropic, kredensial git dan kunci penandatanganan tetap di luar sandbox, dan proxy mengautentikasi atas nama sesi dengan kredensial bersistem. Di lingkungan self-hosted, deployment Anda menyediakan kredensial git; lihat [Konfigurasi git](/docs/id/self-hosted-environments-deploy#configure-git)
* **Kredensial API**: di lingkungan yang dihosting Anthropic pada paket Pro dan Max, kunci yang Anda [tambahkan ke lingkungan cloud](/docs/id/cloud-environments#add-api-credentials) tetap di luar sandbox dengan cara yang sama, dilampirkan ke permintaan yang cocok setelah meninggalkan sesi. Lingkungan self-hosted tidak memiliki kredensial API, dan paket Team dan Enterprise belum memilikinya
* **Analisis aman**: kode dianalisis dan dimodifikasi dalam lingkungan terisolasi sesi sebelum membuat PR

<h2 id="troubleshooting">
  Troubleshooting
</h2>

Untuk kesalahan API runtime yang muncul dalam percakapan seperti `API Error: 500`, `529 Overloaded`, `429`, atau `Prompt is too long`, lihat [referensi Error](/docs/id/errors). Kesalahan tersebut dan perbaikannya dibagikan dengan CLI dan Desktop app. Bagian di bawah mencakup masalah khusus untuk sesi cloud.

<h3 id="session-creation-failed">
  Session creation failed
</h3>

Jika sesi baru gagal dimulai dengan `Session creation failed` atau macet di provisioning, Claude Code tidak dapat mengalokasikan VM untuk sesi.

* Periksa [status.claude.com](https://status.claude.com) untuk insiden sesi cloud
* Coba lagi setelah satu menit, karena kapasitas disediakan sesuai permintaan
* Konfirmasi koneksi GitHub Anda dapat menjangkau repositori dengan mengikuti [No repositories appear after connecting GitHub](/docs/id/web-quickstart#no-repositories-appear-after-connecting-github)

<h3 id="unable-to-get-organization-uuid">
  Unable to get organization UUID
</h3>

`claude --cloud` dan `claude --teleport` memerlukan sign-in dengan akun claude.ai. Jika Anda mengautentikasi dengan kunci API, atau detail akun yang disimpan sudah usang, perintah ini gagal dengan `Unable to get organization UUID` atau pesan bahwa autentikasi kunci API tidak cukup. Dengan autentikasi kunci API atau detail akun yang usang, menjalankan `claude --teleport` tanpa ID sesi menampilkan `Error loading Claude Code sessions` di pemilih sesi alih-alih salah satu pesan, dan perbaikan yang sama berlaku.

Jalankan `/login` untuk masuk dengan akun claude.ai Anda, kemudian coba lagi perintah. Jika kesalahan menyebutkan penyedia Anda, lihat [tabel kesalahan](#output-and-errors): sesi cloud tidak tersedia melalui penyedia pihak ketiga.

<h3 id="remote-control-session-expired-or-access-denied">
  Remote Control session expired or access denied
</h3>

`--teleport` terhubung melalui infrastruktur sesi Remote Control yang sama yang digunakan sesi cloud, jadi kesalahan autentikasi dan kedaluwarsa sesi muncul dengan wording Remote Control. Anda mungkin melihat `Remote Control session expired` atau `Access denied`. Token koneksi berumur pendek dan dibatasi pada akun Anda.

* Jalankan `/login` secara lokal untuk menyegarkan kredensial Anda, kemudian sambungkan kembali
* Konfirmasi Anda masuk ke akun yang sama yang memiliki sesi
* Jika Anda melihat `Remote Control may not be available for this organization`, Owner belum mengaktifkan sesi cloud untuk organisasi Anda

<h3 id="environment-expired">
  Environment expired
</h3>

Sesi cloud berhenti setelah periode inaktivitas dan VM sesi diambil kembali. Sesi dihitung sebagai tidak aktif saat menunggu Anda menyetujui panggilan alat [MCP connector](/docs/id/cloud-environments#network-access) atau untuk masuk ke server MCP, dan dapat kedaluwarsa selama menunggu itu.

Buka kembali sesi dari [claude.ai/code](https://claude.ai/code) untuk menyediakan VM segar dengan riwayat percakapan Anda dipulihkan. Pekerjaan latar belakang yang masih berjalan saat VM diambil kembali, seperti subagents dan perintah shell, tidak dipulihkan.

<h2 id="limitations">
  Batasan
</h2>

Sebelum mengandalkan sesi cloud untuk alur kerja, pertimbangkan batasan ini:

* **Batas laju**: sesi cloud berbagi batas laju dengan semua penggunaan Claude dan Claude Code lainnya dalam akun Anda. Menjalankan beberapa tugas secara paralel mengonsumsi lebih banyak batas laju secara proporsional. Tidak ada biaya komputasi terpisah untuk VM cloud.
* **Autentikasi repositori**: Anda hanya dapat menarik sesi cloud ke terminal Anda saat Anda diautentikasi ke akun yang sama
* **Pembatasan platform**: kloning repositori dan pembuatan pull request memerlukan GitHub. Instans [GitHub Enterprise Server](/docs/id/github-enterprise-server) self-hosted didukung untuk paket Team dan Enterprise. Anda dapat mengirim repositori GitLab, Bitbucket, atau non-GitHub lainnya ke sesi cloud sebagai [bundle lokal](#send-local-repositories-without-github) dengan menetapkan `CCR_FORCE_BUNDLE=1`, tetapi sesi tidak dapat mendorong hasil kembali ke remote tersebut
* **Daftar putih IP organisasi**: sesi cloud memanggil API Anthropic dari infrastruktur yang dikelola Anthropic, bukan jaringan Anda, sementara sesi di [lingkungan self-hosted](/docs/id/self-hosted-environments) memanggilnya dari jaringan Anda sendiri. Jika organisasi Anda memiliki [IP allowlisting](https://support.claude.com/en/articles/13200993-restrict-access-to-claude-with-ip-allowlisting) yang diaktifkan, setiap sesi cloud yang dihosting Anthropic gagal dengan kesalahan autentikasi. Hal yang sama berlaku untuk [Code Review](/docs/id/code-review) dan untuk [routines](/docs/id/routines) yang berjalan pada lingkungan yang dihosting Anthropic; routine yang dialihkan ke lingkungan self-hosted memanggil API dari jaringan Anda sendiri. Hubungi [dukungan Anthropic](https://support.claude.com/) untuk mengecualikan layanan yang dihosting Anthropic dari daftar putih IP organisasi Anda.

<h2 id="related-resources">
  Sumber daya terkait
</h2>

* [Lingkungan cloud](/docs/id/cloud-environments): konfigurasi akses jaringan, variabel lingkungan, dan skrip setup untuk sesi cloud
* [Proyek](/docs/id/claude-projects): satu percakapan di mana Claude mengoordinasikan sesi cloud paralel pada repositori Anda dan melaporkan kembali
* [Ultrareview](/docs/id/ultrareview): jalankan ulasan kode multi-agen mendalam di sandbox cloud
* [Routines](/docs/id/routines): otomatisasi pekerjaan pada jadwal, melalui panggilan API, atau sebagai respons terhadap acara GitHub
* [Konfigurasi Hooks](/docs/id/hooks): jalankan skrip pada acara siklus hidup sesi
* [Semua pengaturan](/docs/id/settings-reference): semua opsi konfigurasi
* [Keamanan](/docs/id/security): jaminan isolasi dan penanganan data
* [Penggunaan Data](/docs/id/data-usage): apa yang Anthropic pertahankan dari sesi cloud
* [Claude Tag](https://claude.com/docs/claude-tag/overview): @Claude yang dikelola organisasi di Slack yang berjalan pada infrastruktur cloud yang sama
