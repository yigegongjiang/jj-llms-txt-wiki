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
| [Web](/docs/id/claude-code-on-the-web) | Tugas yang berjalan lama yang tidak memerlukan banyak pengarahan, atau pekerjaan yang harus dilanjutkan saat Anda offline | Cloud yang dikelola Anthropic, berlanjut setelah Anda terputus                                                                                                                           |
| Mobile                            | Memulai dan memantau tugas saat jauh dari komputer Anda                                                                   | Sesi cloud dari aplikasi Claude untuk iOS dan Android, [Remote Control](/docs/id/remote-control) untuk sesi lokal, [Dispatch](/docs/id/desktop#sessions-from-dispatch) ke Desktop pada Pro dan Max |

CLI adalah permukaan paling lengkap untuk pekerjaan asli terminal: scripting dan Agent SDK hanya tersedia di CLI. Penyedia pihak ketiga juga bekerja di [VS Code](/docs/id/vs-code#use-third-party-providers). Penyebaran [Desktop](/docs/id/desktop) Enterprise mendukung Google Cloud's Agent Platform, dan Desktop mendukung [penyedia gateway](/docs/id/llm-gateway-connect#desktop-app); untuk Amazon Bedrock atau Microsoft Foundry, gunakan CLI atau VS Code, atau [Claude Desktop on 3P](https://claude.com/docs/third-party/claude-desktop/overview), yang menjalankan tab Code pada penyedia tersebut. Desktop dan ekstensi IDE menukar beberapa fitur khusus CLI untuk tinjauan visual dan integrasi editor yang lebih ketat. Web berjalan di cloud Anthropic, jadi tugas terus berlanjut setelah Anda terputus. Mobile adalah klien tipis ke sesi cloud yang sama atau ke sesi lokal melalui Remote Control, dan dapat mengirim tugas ke Desktop dengan Dispatch.

Anda dapat mencampur permukaan pada proyek yang sama. Konfigurasi, memori proyek, dan server MCP dibagikan di seluruh permukaan lokal.

<h2 id="connect-your-tools">
  Hubungkan alat Anda
</h2>

Integrasi memungkinkan Claude bekerja dengan layanan di luar basis kode Anda.

| Integrasi                            | Apa yang dilakukannya                          | Gunakan untuk                                                           |
| :----------------------------------- | :--------------------------------------------- | :---------------------------------------------------------------------- |
| [Chrome](/docs/id/chrome)                 | Mengontrol browser Anda dengan sesi login Anda | Menguji aplikasi web, mengisi formulir, mengotomatisasi situs tanpa API |
| [GitHub Actions](/docs/id/github-actions) | Menjalankan Claude dalam pipeline CI Anda      | Tinjauan PR otomatis, triase masalah, pemeliharaan terjadwal            |
| [GitLab CI/CD](/docs/id/gitlab-ci-cd)     | Sama seperti GitHub Actions untuk GitLab       | Otomasi berbasis CI di GitLab                                           |
| [Code Review](/docs/id/code-review)       | Meninjau setiap PR secara otomatis             | Menangkap bug sebelum tinjauan manusia                                  |
| [Slack](/docs/id/slack)                   | Merespons penyebutan `@Claude` di saluran Anda | Mengubah laporan bug menjadi permintaan tarik dari obrolan tim          |

Untuk integrasi yang tidak tercantum di sini, [server MCP](/docs/id/mcp) dan [konektor](/docs/id/desktop#connect-external-tools) memungkinkan Anda menghubungkan hampir apa pun: Linear, Notion, Google Drive, atau API internal Anda sendiri.

<h2 id="work-when-you-are-away-from-your-terminal">
  Bekerja saat Anda jauh dari terminal Anda
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
* [Claude Code di web](/docs/id/claude-code-on-the-web): sesi cloud yang terus berjalan saat Anda terputus
* Mobile: aplikasi Claude untuk [iOS](https://apps.apple.com/us/app/claude-by-anthropic/id6473753684) dan [Android](https://play.google.com/store/apps/details?id=com.anthropic.claude) untuk memulai dan memantau tugas saat jauh dari komputer Anda

<h3 id="integrations">
  Integrasi
</h3>

* [Chrome](/docs/id/chrome): otomatisasi tugas browser dengan sesi login Anda
* [Computer use](/docs/id/computer-use): biarkan Claude membuka aplikasi dan mengontrol layar Anda di macOS
* [GitHub Actions](/docs/id/github-actions): jalankan Claude dalam pipeline CI Anda
* [GitLab CI/CD](/docs/id/gitlab-ci-cd): yang sama untuk GitLab
* [Code Review](/docs/id/code-review): tinjauan otomatis pada setiap permintaan tarik
* [Slack](/docs/id/slack): kirim tugas dari obrolan tim, dapatkan PR kembali

<h3 id="remote-access">
  Akses jarak jauh
</h3>

* [Dispatch](/docs/id/desktop#sessions-from-dispatch): kirim pesan tugas dari ponsel Anda dan dapat menampilkan sesi Desktop
* [Remote Control](/docs/id/remote-control): jalankan sesi yang sedang berjalan dari ponsel atau browser Anda
* [Channels](/docs/id/channels): dorong acara dari aplikasi obrolan atau server Anda sendiri ke dalam sesi
* [Tugas terjadwal](/docs/id/scheduled-tasks): jalankan prompt pada jadwal berulang
