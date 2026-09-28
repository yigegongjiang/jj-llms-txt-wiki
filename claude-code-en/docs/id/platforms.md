> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Platform dan integrasi

> Pilih di mana menjalankan Claude Code dan apa yang akan dihubungkan. Bandingkan CLI, Desktop, VS Code, JetBrains, web, mobile, dan integrasi seperti Chrome, Slack, dan CI/CD.

Claude Code menjalankan mesin yang sama di mana pun, tetapi setiap permukaan disesuaikan untuk cara kerja yang berbeda. Halaman ini membantu Anda memilih platform yang tepat untuk alur kerja Anda dan menghubungkan alat yang sudah Anda gunakan.

<h2 id="where-to-run-claude-code">
  Di mana menjalankan Claude Code
</h2>

Pilih platform berdasarkan cara Anda suka bekerja dan di mana proyek Anda berada.

| Platform                          | Terbaik untuk                                                                                                             | Yang Anda dapatkan                                                                                                                                                                       |
| :-------------------------------- | :------------------------------------------------------------------------------------------------------------------------ | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [CLI](/docs/id/quickstart)             | Alur kerja terminal, scripting, server jarak jauh                                                                         | Set fitur lengkap, [Agent SDK](/docs/id/headless), [penggunaan komputer](/docs/id/computer-use) di macOS (Pro dan Max), penyedia pihak ketiga                                                      |
| [Desktop](/docs/id/desktop)            | Tinjauan visual, sesi paralel, pengaturan terkelola                                                                       | Penampil diff, pratinjau aplikasi, [penggunaan komputer](/docs/id/desktop#let-claude-use-your-computer) dan [Dispatch](/docs/id/desktop#sessions-from-dispatch) pada Pro dan Max                   |
| [VS Code](/docs/id/vs-code)            | Bekerja di dalam VS Code tanpa beralih ke terminal                                                                        | Diff inline, terminal terintegrasi, konteks file                                                                                                                                         |
| [JetBrains](/docs/id/jetbrains)        | Bekerja di dalam IntelliJ, PyCharm, WebStorm, atau IDE JetBrains lainnya                                                  | Penampil diff, berbagi seleksi, sesi terminal                                                                                                                                            |
| [Web](/docs/id/claude-code-on-the-web) | Tugas yang berjalan lama yang tidak memerlukan banyak pengarahan, atau pekerjaan yang harus dilanjutkan saat Anda offline | Cloud, dikelola Anthropic secara default; berlanjut setelah Anda terputus                                                                                                                |
| [Mobile](/docs/id/mobile)              | Memulai dan memantau tugas saat jauh dari komputer Anda                                                                   | Sesi cloud dari aplikasi Claude untuk iOS dan Android, [Remote Control](/docs/id/remote-control) untuk sesi lokal, [Dispatch](/docs/id/desktop#sessions-from-dispatch) ke Desktop pada Pro dan Max |

CLI adalah permukaan paling lengkap untuk pekerjaan asli terminal: scripting dan Agent SDK hanya tersedia di CLI. Penyedia pihak ketiga juga bekerja di [VS Code](/docs/id/vs-code#use-third-party-providers) dan di [JetBrains](/docs/id/feature-availability#features-available-on-every-provider), yang menjalankan CLI di terminal IDE Anda. Penyebaran [Desktop](/docs/id/desktop) Enterprise mendukung Agent Platform Google Cloud, dan Desktop mendukung [penyedia gateway](/docs/id/llm-gateway-connect#desktop-app); untuk Amazon Bedrock atau Microsoft Foundry, gunakan CLI atau ekstensi IDE, atau [Claude Desktop on 3P](https://claude.com/docs/third-party/claude-desktop/overview), yang menjalankan tab Code pada penyedia tersebut. Desktop dan ekstensi IDE menukar beberapa fitur khusus CLI untuk tinjauan visual dan integrasi editor yang lebih ketat. Web berjalan di cloud, jadi tugas terus berlanjut setelah Anda terputus. Mobile adalah klien tipis ke sesi cloud yang sama atau ke sesi lokal melalui Remote Control, dan dapat mengirim tugas ke Desktop dengan Dispatch.

Anda dapat mencampur permukaan pada proyek yang sama. Konfigurasi, memori proyek, dan server MCP dibagikan di seluruh permukaan lokal.

<h2 id="connect-your-tools">
  Hubungkan alat Anda
</h2>

Integrasi memungkinkan Claude bekerja dengan layanan di luar basis kode Anda.

| Integrasi                                        | Apa yang dilakukannya                                                                                 | Gunakan untuk                                                                   |
| :----------------------------------------------- | :---------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------ |
| [Chrome](/docs/id/chrome)                             | Mengontrol browser Anda dengan sesi login Anda                                                        | Menguji aplikasi web, mengisi formulir, mengotomatisasi situs tanpa API         |
| [GitHub Actions](/docs/id/github-actions)             | Menjalankan Claude dalam pipeline CI Anda                                                             | Tinjauan PR otomatis, triase masalah, pemeliharaan terjadwal                    |
| [GitLab CI/CD](/docs/id/gitlab-ci-cd)                 | Sama seperti GitHub Actions untuk GitLab                                                              | Otomasi berbasis CI di GitLab                                                   |
| [Code Review](/docs/id/code-review)                   | Meninjau setiap PR secara otomatis                                                                    | Menangkap bug sebelum tinjauan manusia                                          |
| [Slack](/docs/id/slack)                               | Merespons penyebutan `@Claude` di saluran Anda                                                        | Mengubah laporan bug menjadi permintaan tarik dari obrolan tim                  |
| [Claude Tag](https://claude.com/docs/claude-tag) | Menjalankan `@Claude` sebagai identitas bersama organisasi Anda dengan akses yang dikonfigurasi admin | Akses tim bersama pada paket Team dan Enterprise, bukan sesi Slack per pengguna |

Untuk integrasi yang tidak tercantum di sini, [server MCP](/docs/id/mcp) dan [konektor](/docs/id/desktop#connect-external-tools) memungkinkan Anda menghubungkan hampir apa pun: Linear, Notion, Google Drive, atau API internal Anda sendiri.

<h2 id="work-when-you-are-away-from-your-terminal">
  Bekerja saat Anda jauh dari terminal Anda
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

Jika Anda tidak yakin di mana harus memulai, [instal CLI](/docs/id/quickstart) dan jalankan di direktori proyek. Jika Anda lebih suka tidak menggunakan terminal, [Desktop](/docs/id/desktop-quickstart) memberi Anda mesin yang sama dengan antarmuka grafis.

<h2 id="related-resources">
  Sumber daya terkait
</h2>

<h3 id="platforms">
  Platform
</h3>

* [Panduan cepat CLI](/docs/id/quickstart): instal dan jalankan perintah pertama Anda di terminal
* [Desktop](/docs/id/desktop): tinjauan diff visual, sesi paralel, penggunaan komputer, dan Dispatch
* [VS Code](/docs/id/vs-code): ekstensi Claude Code di dalam editor Anda
* [JetBrains](/docs/id/jetbrains): ekstensi untuk IntelliJ, PyCharm, dan IDE JetBrains lainnya
* [Web](/docs/id/claude-code-on-the-web): sesi cloud dari browser Anda di claude.ai/code yang terus berjalan saat Anda terputus
* [Proyek](/docs/id/claude-projects): satu percakapan di mana Claude mengoordinasikan banyak sesi cloud untuk sebuah pekerjaan dan melaporkan kembali
* [Mobile](/docs/id/mobile): aplikasi Claude untuk [iOS](https://apps.apple.com/us/app/claude-by-anthropic/id6473753684) dan [Android](https://play.google.com/store/apps/details?id=com.anthropic.claude) untuk memulai dan memantau tugas saat jauh dari komputer Anda

<h3 id="integrations">
  Integrasi
</h3>

* [Chrome](/docs/id/chrome): otomatisasi tugas browser dengan sesi login Anda
* [Computer use](/docs/id/computer-use): biarkan Claude membuka aplikasi dan mengontrol layar Anda di macOS
* [GitHub Actions](/docs/id/github-actions): jalankan Claude dalam pipeline CI Anda
* [GitLab CI/CD](/docs/id/gitlab-ci-cd): yang sama untuk GitLab
* [Code Review](/docs/id/code-review): tinjauan otomatis pada setiap permintaan tarik
* [Slack](/docs/id/slack): kirim tugas dari obrolan tim, dapatkan PR kembali
* [Claude Tag](https://claude.com/docs/claude-tag): jalankan `@Claude` sebagai identitas bersama organisasi Anda pada paket Team dan Enterprise

<h3 id="remote-access">
  Akses jarak jauh
</h3>

* [Dispatch](/docs/id/desktop#sessions-from-dispatch): kirim pesan tugas dari ponsel Anda dan dapat menampilkan sesi Desktop
* [Remote Control](/docs/id/remote-control): jalankan sesi yang sedang berjalan dari ponsel atau browser Anda
* [Channels](/docs/id/channels): dorong acara dari aplikasi obrolan atau server Anda sendiri ke dalam sesi
* [Tugas terjadwal](/docs/id/scheduled-tasks): jalankan prompt pada jadwal berulang
