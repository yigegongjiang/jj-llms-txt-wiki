> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code GitHub Actions

> Jalankan Claude Code dalam alur kerja GitHub Actions untuk merespons penyebutan @claude, mengotomatisasi tugas, dan mengubah issue menjadi pull request

[Claude Code GitHub Actions](https://github.com/anthropics/claude-code-action) adalah GitHub Action yang menjalankan Claude Code di dalam alur kerja repositori Anda. Sebutkan `@claude` dalam komentar pull request atau issue untuk membuat Claude menganalisis kode, mengimplementasikan perubahan, dan push commit. Anda juga dapat memberikan Claude Code GitHub Action sebuah prompt untuk dijalankan secara otomatis pada event GitHub apa pun. Gunakan untuk mengubah issue menjadi pull request, memperbaiki bug dari komentar, atau mengotomatisasi tugas berulang.

Beberapa produk berbagi nama Claude Code. Halaman ini mencakup integrasi alur kerja `claude-code-action`, yang Anda konfigurasi dengan file alur kerja di repositori Anda. Untuk produk terkait, lihat:

* [Code Review](/docs/id/code-review): ulasan otomatis pada setiap pull request, tanpa menulis alur kerja
* [Claude Code di cloud](/docs/id/claude-code-on-the-web): sesi Claude Code yang berjalan pada infrastruktur cloud alih-alih mesin Anda
* [Claude Agent SDK](/docs/id/agent-sdk/overview): otomasi kustom di luar GitHub Actions. Claude Code GitHub Action dibangun di atas SDK
* [GitHub Enterprise Server](/docs/id/github-enterprise-server): Claude Code dengan GitHub yang di-host sendiri

<h2 id="setup">
  Penyiapan
</h2>

Anda dapat menyiapkan Claude Code GitHub Action dengan salah satu dari dua cara:

* **Penyiapan cepat**: jalankan `/install-github-app` dari Claude Code. Claude Code menginstal GitHub App, menambahkan rahasia autentikasi Anda, dan menyiapkan pull request alur kerja untuk Anda
* **Penyiapan manual**: instal aplikasi, tambahkan rahasia, dan salin file alur kerja ke repositori Anda sendiri. Gunakan jalur ini ketika Anda tidak menjalankan Claude Code secara lokal, ketika perintah gagal, atau ketika Anda menginginkan kontrol penuh atas file alur kerja

Untuk jalur apa pun, Anda memerlukan akses admin ke repositori.

<h3 id="quick-setup">
  Penyiapan cepat
</h3>

`/install-github-app` hanya bekerja dengan repositori github.com. Jika git remote repositori Anda berada di gitlab.com atau bitbucket.org, perintah mencetak pemberitahuan dan keluar alih-alih memulai penyiapan. Untuk menjalankan Claude Code dari pipeline GitLab, lihat [Claude Code GitLab CI/CD](/docs/id/gitlab-ci-cd).

Sebelum Anda mulai, instal [GitHub CLI](https://cli.github.com) dan autentikasi dengan `gh auth login`. Claude Code memeriksa dan memperingatkan Anda jika tidak ada.

Buka `claude` di repositori yang ingin Anda hubungkan, jalankan `/install-github-app`, dan ikuti prompt. Claude Code menginstal Claude GitHub App, kemudian menyiapkan rahasia autentikasi untuk alur kerja:

* Jika Claude Code sudah memiliki kunci API, Claude Code menggunakan kembali kunci tersebut, dan menawarkan untuk menyimpan rahasia `ANTHROPIC_API_KEY` yang ada di repositori jika sudah diatur
* Jika tidak, pilih antara membuat token jangka panjang dengan langganan Claude Anda dan menempel kunci API

Claude Code menyimpan kredensial sebagai rahasia repositori, bernama `ANTHROPIC_API_KEY` untuk kunci API atau `CLAUDE_CODE_OAUTH_TOKEN` untuk token langganan.

Claude Code kemudian push branch dengan file alur kerja yang Anda pilih, sudah diatur untuk menggunakan rahasia tersebut, dan membuka GitHub di browser Anda dengan pull request siap dibuat. Buat dan merge pull request tersebut, dan `@claude` bekerja di repositori.

Jika Anda memilih alur kerja ulasan, Claude memposting setiap ulasan di pull request itu sendiri, sebagai komentar inline pada setiap issue yang ditemukannya atau sebagai satu komentar ringkasan ketika tidak menemukan apa pun. Claude melewati beberapa pull request, seperti draft. Contoh [alur kerja ulasan](#run-a-skill) menggunakan skill yang sama dan mencantumkannya. Sebelum v2.1.229, Claude menulis ulasannya hanya ke log run alur kerja.

Untuk memperbarui alur kerja ulasan yang dihasilkan versi sebelumnya, lakukan salah satu dari berikut:

* Jalankan `/install-github-app` lagi. Ketika repositori sudah memiliki `claude.yml`, pilih **Update workflow file with latest version**. Claude Code push salinan segar file alur kerja ke branch baru dan membuka pull request, sama seperti instalasi pertama.
* Tambahkan argumen `--comment` dan baris `claude_args` dari [contoh alur kerja ulasan](#run-a-skill) ke file yang di-check-in sendiri, yang menjaga edit lain yang Anda buat padanya.

Setelah menginstal GitHub App, Claude Code menanyakan apakah akan melanjutkan dengan penyiapan GitHub Actions. Pilih **Skip for now** untuk berhenti hanya dengan GitHub App yang diinstal. Jalankan `/install-github-app` lagi nanti untuk menyelesaikan langkah alur kerja dan rahasia.

<Note>
  * Ketika Anda menginstal GitHub App, Anda memberikan beberapa izin. Lihat [izin GitHub App](#github-app-permissions) untuk set lengkapnya
  * Penyiapan cepat bekerja dengan Claude API dan langganan Claude. Jika Anda menggunakan Amazon Bedrock, Google Cloud's Agent Platform, atau Microsoft Foundry, lihat [Gunakan Claude Code GitHub Actions dengan penyedia cloud](/docs/id/github-actions-cloud-providers)
</Note>

<h3 id="manual-setup">
  Penyiapan manual
</h3>

Untuk mengonfigurasi Claude Code GitHub Action tanpa menjalankan `/install-github-app`, instal aplikasi, tambahkan rahasia, dan salin file alur kerja sendiri:

<Steps>
  <Step title="Instal Claude GitHub App">
    Instal [Claude GitHub App](https://github.com/apps/claude) ke repositori Anda. Claude Code GitHub Action mengandalkan tiga izin aplikasi:

    * **Contents**: baca dan tulis, sehingga Claude dapat memodifikasi file repositori
    * **Issues**: baca dan tulis, sehingga Claude dapat merespons issue
    * **Pull requests**: baca dan tulis, sehingga Claude dapat membuat PR dan push perubahan

    Selama instalasi, Anda juga memberikan izin yang fitur Claude lain gunakan. Lihat [izin GitHub App](#github-app-permissions) untuk set lengkapnya.
  </Step>

  <Step title="Tambahkan rahasia autentikasi">
    Tambahkan salah satu rahasia berikut ke repositori Anda, tergantung pada cara Anda mengautentikasi. Lihat panduan GitHub untuk [menggunakan rahasia di GitHub Actions](https://docs.github.com/en/actions/security-guides/using-secrets-in-github-actions).

    * `ANTHROPIC_API_KEY`: kunci Claude API dari [Claude Console](https://platform.claude.com)
    * `CLAUDE_CODE_OAUTH_TOKEN`: token OAuth yang mengautentikasi dengan langganan Claude Anda, tersedia di paket Pro, Max, Team, dan Enterprise. Hasilkan satu dengan menjalankan `claude setup-token` secara lokal. Lihat [Hasilkan token jangka panjang](/docs/id/authentication#generate-a-long-lived-token)

    Dalam file alur kerja, teruskan rahasia ke input yang cocok: `anthropic_api_key` untuk kunci API, atau `claude_code_oauth_token` untuk token OAuth.
  </Step>

  <Step title="Salin file alur kerja">
    Salin [examples/claude.yml](https://github.com/anthropics/claude-code-action/blob/main/examples/claude.yml) ke direktori `.github/workflows/` repositori Anda. File ini adalah alur kerja yang berfungsi, bukan hanya contoh. Seperti yang di-commit, Claude merespons setiap kali seseorang menyebutkan `@claude` dalam issue atau pull request, mengautentikasi dengan rahasia `ANTHROPIC_API_KEY`. Jika Anda menambahkan `CLAUDE_CODE_OAUTH_TOKEN` sebagai gantinya, ubah baris `anthropic_api_key` alur kerja menjadi `claude_code_oauth_token: ${{ secrets.CLAUDE_CODE_OAUTH_TOKEN }}`.
  </Step>
</Steps>

<Tip>
  Setelah penyiapan, uji Claude Code GitHub Action dengan menandai `@claude` dalam komentar issue atau PR.
</Tip>

<h3 id="set-up-for-an-organization">
  Penyiapan untuk organisasi
</h3>

Dengan penyiapan cepat atau manual, Anda mengonfigurasi satu repositori sekaligus. Untuk meluncurkan Claude Code GitHub Action di seluruh organisasi:

* Instal [Claude GitHub App](https://github.com/apps/claude) sekali di tingkat organisasi, memilih semua repositori atau daftar yang dipilih
* Simpan rahasia autentikasi sebagai rahasia Actions tingkat organisasi sehingga setiap repositori tidak perlu salinannya sendiri
* Tambahkan file alur kerja ke setiap repositori yang harus menjalankan Claude Code GitHub Action, atau tentukan pekerjaan sekali sebagai [alur kerja yang dapat digunakan kembali](https://docs.github.com/en/actions/using-workflows/reusing-workflows) yang setiap repositori panggil

Untuk rahasia yang dibagikan di seluruh repositori, autentikasi dengan kunci API dari [Claude Console](https://platform.claude.com) daripada token OAuth, karena token OAuth terikat pada langganan orang yang menjalankan `claude setup-token`.

Untuk menghindari menyimpan rahasia jangka panjang sama sekali, autentikasi melalui federasi identitas beban kerja, di mana Claude Code GitHub Action menukar token OpenID Connect (OIDC) GitHub alur kerja untuk akses Claude API melalui akun layanan Claude Console. Atur input ini:

* `anthropic_federation_rule_id`: ID aturan federasi, `fdrl_...`
* `anthropic_organization_id`: ID organisasi Anthropic Anda
* `anthropic_service_account_id`: ID akun layanan, `svac_...`. Opsional, karena aturan federasi yang Anda buat di Console sudah menargetkan akun layanan
* `anthropic_workspace_id`: ID workspace, `wrkspc_...`. Opsional ketika aturan federasi menargetkan workspace tunggal

Berikan alur kerja izin `id-token: write`, yang Claude Code GitHub Action butuhkan untuk pertukaran federasi bahkan ketika Anda meneruskan `github_token` Anda sendiri. Lihat [panduan penyiapan Claude Code GitHub Action](https://github.com/anthropics/claude-code-action/blob/main/docs/setup.md) untuk konfigurasi sisi Console.

Untuk pertanyaan penanganan dan retensi data dalam tinjauan keamanan, lihat [penggunaan data](/docs/id/data-usage) dan [keamanan](/docs/id/security).

<h3 id="uninstall">
  Uninstal
</h3>

Untuk menghapus Claude Code GitHub Action, batalkan setiap bagian penyiapan yang berlaku untuk instalasi Anda:

* **File alur kerja**: hapus alur kerja yang menggunakan `anthropics/claude-code-action` dari `.github/workflows/`. Jika Anda menggunakan penyiapan cepat, cari `claude.yml` dan, jika Anda memilih alur kerja ulasan, `claude-code-review.yml`. Dengan alur kerja dihapus, Claude Code GitHub Action tidak lagi berjalan
* **Rahasia**: hapus rahasia `ANTHROPIC_API_KEY` atau `CLAUDE_CODE_OAUTH_TOKEN` dari repositori, dan dari rahasia Actions tingkat organisasi jika Anda [membagikannya di seluruh repositori](#set-up-for-an-organization). Jika Anda menghapus rahasia, kredensial yang disimpannya tetap valid. Untuk pensiun kunci API sepenuhnya, juga hapus kunci di [Claude Console](https://platform.claude.com)
* **GitHub App**: uninstal Claude GitHub App di pengaturan repositori atau organisasi Anda di bawah GitHub Apps, tetapi hanya jika Anda tidak menggunakannya untuk fitur Claude lain, seperti Code Review atau web auto-fix

Jika Anda mengonfigurasi [penyedia cloud](/docs/id/github-actions-cloud-providers), juga hapus rahasia penyedia, seperti `AWS_ROLE_TO_ASSUME`, rahasia `GCP_*`, atau rahasia `AZURE_*`, dan uninstal GitHub App kustom bersama dengan rahasia `APP_ID` dan `APP_PRIVATE_KEY` miliknya.

<h3 id="github-app-permissions">
  Izin GitHub App
</h3>

[Claude GitHub App](https://github.com/apps/claude) dibagikan oleh setiap fitur Claude yang terintegrasi dengan GitHub, termasuk Claude Code GitHub Action, [Code Review](/docs/id/code-review), dan [auto-fix untuk pull request](/docs/id/claude-code-on-the-web#auto-fix-pull-requests) di sesi cloud. GitHub App memiliki set izin tunggal yang mencakup semua fiturnya, jadi set mencakup beberapa izin yang Claude Code GitHub Action tidak gunakan.

Ketika Anda menginstal aplikasi, Anda memberikan izin berikut:

| Izin             | Akses          |
| ---------------- | -------------- |
| Actions          | Baca dan tulis |
| Checks           | Baca dan tulis |
| Contents         | Baca dan tulis |
| Discussions      | Baca dan tulis |
| Issues           | Baca dan tulis |
| Members          | Baca           |
| Metadata         | Baca           |
| Pull requests    | Baca dan tulis |
| Repository hooks | Baca dan tulis |
| Statuses         | Baca           |
| Workflows        | Baca dan tulis |

Set izin juga dapat berubah sebelum fitur yang menggunakannya. Ketika aplikasi meminta izin yang sebelumnya tidak dimilikinya, GitHub meminta pemilik akun untuk menyetujuinya, pemilik organisasi untuk instalasi organisasi, dan instalasi menyimpan izin lamanya sampai mereka melakukannya. Misalnya, ketika akses Actions berubah dari baca menjadi tulis, aplikasi dapat menjalankan kembali alur kerja daripada hanya melihat run dan log, jadi GitHub meminta pemilik untuk menyetujui perubahan.

Ketika Anda menginstal aplikasi, Anda menerima set izin penuhnya. GitHub tidak membiarkan Anda menerima subset. Jika organisasi Anda hanya memerlukan izin yang Claude Code GitHub Action gunakan, buat GitHub App kustom dengan Contents, Issues, dan Pull requests sebagai gantinya, mengikuti [panduan penyiapan Claude Code GitHub Action](https://github.com/anthropics/claude-code-action/blob/main/docs/setup.md). Aplikasi kustom hanya mencakup Claude Code GitHub Action. Code Review dan web auto-fix masih memerlukan aplikasi resmi.

Untuk detail tentang bagaimana Claude Code GitHub Action membatasi apa yang Claude dapat lakukan dengan izin ini, lihat [dokumentasi keamanan](https://github.com/anthropics/claude-code-action/blob/main/docs/security.md).

<h2 id="interactive-and-automation-modes">
  Mode interaktif dan otomasi
</h2>

Claude Code GitHub Action mendeteksi cara menjalankan dari konfigurasi alur kerja Anda:

* **Mode interaktif**: ketika alur kerja tidak menyediakan input `prompt`, Claude menunggu frasa pemicu, `@claude` secara default, dalam komentar issue atau pull request, dalam ulasan pull request, atau dalam badan atau judul issue yang baru dibuka, kemudian merespons permintaan tersebut. Kemajuan dan hasil muncul sebagai komentar pada issue atau PR yang memicu.
* **Mode otomasi**: ketika alur kerja menyediakan input `prompt`, Claude berjalan tanpa menunggu penyebutan, tunduk hanya pada [pemeriksaan siapa yang dapat memicu run](#who-can-trigger-runs). Secara default, hasil muncul dalam log run alur kerja daripada komentar. Claude dapat memposting ke issue atau pull request ketika prompt mengarahkannya dan memiliki alat yang dapat memposting, seperti dalam [contoh code-review](#run-a-skill).

<h3 id="who-can-trigger-runs">
  Siapa yang dapat memicu run
</h3>

Di kedua mode, Claude Code GitHub Action menjalankan dua pemeriksaan pada aktor pemicu sebelum Claude dimulai, dan run gagal ketika salah satu pemeriksaan menolaknya:

* **Akses tulis**: pada event issue dan pull request, pengguna pemicu harus memiliki akses tulis ke repositori. Untuk memungkinkan pengguna tertentu tanpa akses tulis, atur `allowed_non_write_users` dan teruskan input `github_token` Anda sendiri. Event yang tidak ada pengguna yang mengarang, seperti pemicu `schedule`, lewati pemeriksaan ini.
* **Aktor manusia**: pada setiap event, Claude Code GitHub Action menolak aktor bot kecuali Anda mencantumkannya di `allowed_bots`, yang menjaga bot agar tidak memicu Claude dalam loop. Pemeriksaan ini juga berlaku untuk run terjadwal, yang GitHub atribusikan ke pengguna repositori, biasanya yang terakhir mengubah jadwal `cron` alur kerja. Jika pengguna itu adalah bot, cantumkan di `allowed_bots`.

<h2 id="example-use-cases">
  Contoh kasus penggunaan
</h2>

Direktori [examples](https://github.com/anthropics/claude-code-action/tree/main/examples) berisi alur kerja siap pakai untuk skenario berbeda.

Contoh di halaman ini menunjukkan autentikasi kunci API. Jika Anda mengautentikasi dengan langganan Claude, ganti baris `anthropic_api_key` dalam contoh apa pun dengan `claude_code_oauth_token: ${{ secrets.CLAUDE_CODE_OAUTH_TOKEN }}`.

<h3 id="respond-to-claude-mentions">
  Merespons penyebutan @claude
</h3>

Alur kerja ini menjalankan Claude Code GitHub Action dalam mode interaktif, sehingga Claude merespons setiap kali seseorang menyebutkan `@claude` dalam komentar issue atau PR.

```yaml theme={null}
name: Claude Code
on:
  issue_comment:
    types: [created]
  pull_request_review_comment:
    types: [created]
jobs:
  claude:
    if: contains(github.event.comment.body, '@claude')
    runs-on: ubuntu-latest
    permissions:
      contents: write
      pull-requests: write
      issues: write
      id-token: write
      actions: read
    steps:
      - uses: actions/checkout@v6
        with:
          fetch-depth: 1
      - uses: anthropics/claude-code-action@v1
        with:
          anthropic_api_key: ${{ secrets.ANTHROPIC_API_KEY }}
```

Bagian alur kerja ini yang bukan boilerplate:

* `id-token: write`: diperlukan untuk autentikasi GitHub App default Claude Code GitHub Action
* `actions: read`: memungkinkan Claude membaca hasil CI pada PR
* `actions/checkout`: memberikan Claude salinan lokal repositori untuk dikerjakan
* `if`: menjaga runner agar tidak dimulai pada komentar yang tidak menyebutkan `@claude`. Claude Code GitHub Action juga memeriksa frasa pemicu itu sendiri sebelum merespons

Setelah alur kerja ada, sebutkan `@claude` dalam komentar issue atau PR apa pun dengan permintaan:

```text wrap theme={null}
@claude implement this feature based on the issue description
@claude how should I implement user authentication for this endpoint?
@claude fix the TypeError in the user dashboard component
```

Claude membalas dalam komentar pada issue atau PR yang sama dan memperbarui saat bekerja.

<h3 id="run-a-skill">
  Jalankan skill
</h3>

Input `prompt` menerima invokasi [skill](/docs/id/skills) serta teks biasa:

* Untuk skill di direktori `.claude/skills/` repositori Anda, jalankan `actions/checkout` sebelum langkah `anthropics/claude-code-action` sehingga file skill tersedia di runner, kemudian teruskan `/skill-name` sebagai `prompt`.
* Untuk skill yang dikemas dalam [plugin](/docs/id/plugins/overview), instal plugin dengan input `plugin_marketplaces` dan `plugins`, kemudian teruskan `/plugin-name:skill-name` dengan namespace sebagai `prompt`. Input `plugins` mengambil `plugin-name@marketplace-name`, di mana nama marketplace berasal dari manifest marketplace itu sendiri daripada URL repositorinya.

Alur kerja berikut menginstal plugin `code-review` dan menjalankan skillnya ketika pull request dibuka, diperbarui, dibuka kembali, atau ditandai siap untuk ulasan. Ini menjalankan plugin yang sama dengan alur kerja ulasan dari penyiapan cepat. Gunakan alur kerja seperti ini ketika Anda ingin mengontrol prompt, model, dan pemicu sendiri. Untuk ulasan otomatis tanpa memelihara file alur kerja, lihat [Code Review](/docs/id/code-review). Di repositori publik, GitHub menahan rahasia dari run yang dipicu oleh pull request fork, jadi ulasan hanya berjalan pada pull request dari branch di repositori yang sama.

```yaml theme={null}
name: Code Review
on:
  pull_request:
    types: [opened, synchronize, ready_for_review, reopened]
jobs:
  review:
    runs-on: ubuntu-latest
    permissions:
      contents: read
      pull-requests: read
      issues: read
      id-token: write
    steps:
      - uses: actions/checkout@v6
        with:
          fetch-depth: 1
      - uses: anthropics/claude-code-action@v1
        with:
          anthropic_api_key: ${{ secrets.ANTHROPIC_API_KEY }}
          plugin_marketplaces: "https://github.com/anthropics/claude-code.git"
          plugins: "code-review@claude-code-plugins"
          prompt: "/code-review:code-review --comment ${{ github.repository }}/pull/${{ github.event.pull_request.number }}"
          claude_args: '--allowedTools "mcp__github_inline_comment__create_inline_comment"'
```

Dua baris dalam alur kerja ini mengontrol di mana ulasan pergi:

* **`--comment`**: Claude memposting ulasannya di pull request, sebagai komentar inline pada setiap issue yang ditemukannya atau sebagai satu komentar ringkasan ketika tidak menemukan apa pun. Tanpanya, Claude tidak memposting apa pun, dan Anda membaca temuan dalam log run alur kerja.
* **`claude_args`**: simpan baris ini meskipun frontmatter `allowed-tools` skill itu sendiri menamai alat yang sama, karena Claude Code GitHub Action memulai server MCP yang memposting komentar inline hanya ketika `--allowedTools` di `claude_args` menamakannya.

Claude melewati pull request draft dan tertutup, pull request yang Claude nilai tidak perlu ulasan, seperti yang otomatis atau trivial, dan pull request yang sudah memiliki komentar dari Claude.

<h3 id="run-on-a-schedule">
  Jalankan pada jadwal
</h3>

Dengan input `prompt`, Claude Code GitHub Action berjalan dalam mode otomasi pada event GitHub apa pun, termasuk jadwal cron. Untuk prompt teks biasa, Claude tidak memiliki akses shell atau GitHub API sampai Anda memberikan alat yang prompt butuhkan, dengan `--allowedTools` di `claude_args` atau aturan [`permissions.allow`](/docs/id/permissions#permission-rule-syntax) di input `settings`. Jika Anda menjalankan skill sebagai gantinya, Claude dapat menggunakan alat yang frontmatter [`allowed-tools`](/docs/id/skills#pre-approve-tools-for-a-skill) miliknya berikan. GitHub menjalankan alur kerja terjadwal hanya dari branch default dan, di repositori publik, menonaktifkan jadwal setelah 60 hari tanpa aktivitas repositori.

Alur kerja ini menghasilkan laporan dalam log run alur kerja pada 09:00 UTC setiap hari. Baris `claude_args` miliknya [meneruskan argumen CLI](#pass-cli-arguments) yang memilih model dan memungkinkan dua alat GitHub MCP. Claude membaca commit dan issue melalui GitHub API dengan alat tersebut, jadi Anda dapat menghilangkan langkah checkout:

```yaml theme={null}
name: Daily Report
on:
  schedule:
    - cron: "0 9 * * *"
jobs:
  report:
    runs-on: ubuntu-latest
    permissions:
      contents: read
      issues: read
      id-token: write
    steps:
      - uses: anthropics/claude-code-action@v1
        with:
          anthropic_api_key: ${{ secrets.ANTHROPIC_API_KEY }}
          prompt: "Generate a summary of yesterday's commits and open issues"
          claude_args: |
            --model claude-opus-5-5
            --allowedTools "mcp__github__list_commits,mcp__github__list_issues"
```

<h2 id="best-practices">
  Praktik terbaik
</h2>

<h3 id="define-project-standards-in-claude-md">
  Tentukan standar proyek di CLAUDE.md
</h3>

Buat file `CLAUDE.md` di root repositori Anda untuk mendefinisikan panduan gaya kode, kriteria ulasan, aturan khusus proyek, dan pola yang disukai. Claude mengikuti panduan ini saat membuat PR dan merespons permintaan. Lihat [dokumentasi memory](/docs/id/memory) untuk detail.

<h3 id="protect-your-credentials">
  Lindungi kredensial Anda
</h3>

<Warning>
  Jangan pernah commit kunci API atau token OAuth langsung ke repositori Anda. Selalu simpan sebagai GitHub Secrets dan referensikan dalam alur kerja, misalnya `anthropic_api_key: ${{ secrets.ANTHROPIC_API_KEY }}`.
</Warning>

Berikan alur kerja hanya izin yang dibutuhkannya, dan tinjau perubahan Claude sebelum merge.

Untuk panduan keamanan komprehensif termasuk izin dan autentikasi, lihat [dokumentasi keamanan Claude Code Action](https://github.com/anthropics/claude-code-action/blob/main/docs/security.md).

<h3 id="manage-costs">
  Kelola biaya
</h3>

Setiap run mengonsumsi dua jenis sumber daya:

* **Menit GitHub Actions**: Claude Code GitHub Action berjalan di runner yang dihosting GitHub, yang mengonsumsi menit GitHub Actions Anda. Lihat [dokumentasi penagihan GitHub](https://docs.github.com/en/billing/managing-billing-for-your-products/managing-billing-for-github-actions/about-billing-for-github-actions) untuk harga dan batas menit.
* **Token API**: setiap interaksi mengonsumsi token berdasarkan panjang prompt dan respons, kompleksitas tugas, dan ukuran codebase. Lihat [halaman harga Claude](https://claude.com/platform/api) untuk tarif token saat ini. Jika Anda mengautentikasi dengan token OAuth, run menggunakan langganan Claude Anda daripada penagihan API.

Anda dapat menurunkan kedua jenis biaya dengan memberikan Claude konteks yang lebih jelas dan dengan membatasi berapa banyak pekerjaan yang dapat dilakukan setiap run:

* Tulis permintaan `@claude` spesifik sehingga Claude memerlukan lebih sedikit turn untuk menyelesaikan
* Gunakan template issue untuk memberikan konteks di muka
* Jaga `CLAUDE.md` Anda ringkas, karena Claude membacanya pada setiap run
* Atur `--max-turns` di `claude_args` untuk membatasi iterasi
* Atur timeout tingkat alur kerja untuk menghindari pekerjaan yang tidak terkontrol
* Gunakan kontrol concurrency GitHub untuk membatasi run paralel

Untuk pelacakan penggunaan di seluruh organisasi Anda, lihat [dashboard analytics](/docs/id/analytics) dan [monitoring](/docs/id/monitoring-usage). Untuk cara penggunaan diukur dan ditagih, lihat [biaya](/docs/id/costs).

<h2 id="use-a-cloud-provider">
  Gunakan penyedia cloud
</h2>

Secara default, Claude Code GitHub Action memanggil Claude API secara langsung dengan kunci API atau token OAuth Anda. Untuk merutekan inferensi melalui akun cloud Anda sendiri, atur input untuk penyedia Anda dan ikuti [Gunakan Claude Code GitHub Actions dengan penyedia cloud](/docs/id/github-actions-cloud-providers):

* **Amazon Bedrock**: `use_bedrock: "true"`
* **Google Cloud's Agent Platform**: `use_vertex: "true"`
* **Microsoft Foundry**: `use_foundry: "true"`

Dengan ketiga penyedia, Anda mengautentikasi melalui federasi identitas OIDC daripada kunci API Claude, jadi Anda tidak menyimpan kredensial cloud statis di repositori Anda.

<h2 id="troubleshooting">
  Troubleshooting
</h2>

<h3 id="claude-not-responding-to-claude-commands">
  Claude tidak merespons perintah @claude
</h3>

* Verifikasi GitHub App diinstal di repositori
* Periksa bahwa alur kerja diaktifkan untuk repositori
* Pastikan kunci API atau token OAuth diatur dalam rahasia repositori
* Konfirmkan komentar berisi `@claude` sebagai kata lengkap, bukan `/claude` atau `@claude-bot`
* Konfirmkan pengguna yang berkomentar memiliki akses tulis ke repositori. Lihat [Siapa yang dapat memicu run](#who-can-trigger-runs) untuk pengecualian

<h3 id="ci-not-running-on-claude’s-commits">
  CI tidak berjalan pada commit Claude
</h3>

* GitHub tidak memicu alur kerja pada commit yang dibuat dengan `GITHUB_TOKEN` default. Jika Anda meneruskan `github_token: ${{ secrets.GITHUB_TOKEN }}` ke Claude Code GitHub Action, hapus sehingga mengautentikasi sebagai Claude GitHub App, atau teruskan token aplikasi kustom sebagai gantinya
* Periksa bahwa pemicu alur kerja CI Anda mencakup event yang push Claude hasilkan, seperti `push` atau `pull_request`

<h3 id="authentication-errors">
  Kesalahan autentikasi
</h3>

* Konfirmkan kunci API atau token OAuth valid dengan mengujinya secara lokal dengan `claude` sebelum men-debug alur kerja
* Untuk Bedrock, Agent Platform, dan Foundry, lihat bagian [troubleshooting](/docs/id/github-actions-cloud-providers#troubleshooting) halaman penyedia cloud

Untuk solusi lebih lanjut, lihat [FAQ](https://github.com/anthropics/claude-code-action/blob/main/docs/faq.md) Claude Code GitHub Action.

<h2 id="advanced-configuration">
  Konfigurasi lanjutan
</h2>

<h3 id="action-parameters">
  Parameter action
</h3>

Ini adalah input yang paling umum digunakan. Masing-masing memetakan ke kunci `with:` dalam langkah `anthropics/claude-code-action`.

| Parameter                 | Deskripsi                                                                                                                                                                             | Diperlukan                                                                                                                                                                                           |
| ------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `prompt`                  | Instruksi untuk Claude, sebagai teks biasa atau invokasi [skill](/docs/id/skills). Ketika dihilangkan, Claude merespons [frasa pemicu](#interactive-and-automation-modes) sebagai gantinya | Tidak                                                                                                                                                                                                |
| `claude_args`             | Argumen CLI yang diteruskan ke Claude Code                                                                                                                                            | Tidak                                                                                                                                                                                                |
| `anthropic_api_key`       | Kunci Claude API                                                                                                                                                                      | Untuk Claude API, kecuali Anda menggunakan `claude_code_oauth_token` atau [federasi identitas beban kerja](#set-up-for-an-organization). Tidak digunakan untuk Bedrock, Agent Platform, atau Foundry |
| `claude_code_oauth_token` | Token OAuth untuk mengautentikasi dengan langganan Claude, dihasilkan dengan `claude setup-token`                                                                                     | Tidak                                                                                                                                                                                                |
| `github_token`            | Token untuk operasi GitHub. Ketika dihilangkan, Claude Code GitHub Action mengautentikasi sebagai Claude GitHub App                                                                   | Tidak                                                                                                                                                                                                |
| `plugin_marketplaces`     | Daftar URL Git marketplace plugin yang dipisahkan baris baru                                                                                                                          | Tidak                                                                                                                                                                                                |
| `plugins`                 | Daftar nama plugin yang dipisahkan baris baru untuk diinstal sebelum eksekusi                                                                                                         | Tidak                                                                                                                                                                                                |
| `settings`                | Pengaturan Claude Code, sebagai string JSON atau path ke file JSON pengaturan                                                                                                         | Tidak                                                                                                                                                                                                |
| `trigger_phrase`          | Frasa pemicu Claude merespons. Default: `@claude`                                                                                                                                     | Tidak                                                                                                                                                                                                |
| `use_bedrock`             | Gunakan Amazon Bedrock daripada Claude API                                                                                                                                            | Tidak                                                                                                                                                                                                |
| `use_vertex`              | Gunakan Google Cloud's Agent Platform daripada Claude API                                                                                                                             | Tidak                                                                                                                                                                                                |
| `use_foundry`             | Gunakan Microsoft Foundry daripada Claude API                                                                                                                                         | Tidak                                                                                                                                                                                                |

Untuk daftar input lengkap, lihat [referensi konfigurasi](https://github.com/anthropics/claude-code-action/blob/main/docs/usage.md#inputs) Claude Code GitHub Action.

<h3 id="pass-cli-arguments">
  Teruskan argumen CLI
</h3>

Parameter `claude_args` menerima argumen [Claude Code CLI](/docs/id/cli-reference) apa pun:

```yaml theme={null}
claude_args: "--max-turns 5 --model claude-sonnet-5 --mcp-config /path/to/config.json"
```

Argumen umum:

* `--max-turns`: batasi jumlah conversation turn
* `--model`: model yang digunakan, misalnya `claude-sonnet-5`. Tanpa argumen ini, Claude Code GitHub Action menggunakan [model default](/docs/id/model-config) Claude Code
* `--mcp-config`: path ke [konfigurasi MCP](/docs/id/mcp)
* `--allowedTools`: daftar alat yang diizinkan yang dipisahkan koma. Alias `--allowed-tools` juga berfungsi
* `--debug`: aktifkan output debug

<h2 id="upgrade-from-beta">
  Upgrade dari beta
</h2>

Jika alur kerja Anda masih mereferensikan `anthropics/claude-code-action@beta`, perbarui ke v1:

1. Ubah `@beta` menjadi `@v1` dalam baris `uses`
2. Hapus input `mode`, karena Claude Code GitHub Action sekarang [mendeteksi mode secara otomatis](#interactive-and-automation-modes)
3. Ganti `direct_prompt` dengan `prompt`
4. Pindahkan opsi CLI seperti `max_turns` dan `model` ke `claude_args`. `custom_instructions` tidak memiliki flag dengan nama yang sama dan menjadi `--append-system-prompt`

Untuk pemetaan input lengkap dan contoh sebelum-dan-sesudah, lihat [panduan migrasi](https://github.com/anthropics/claude-code-action/blob/main/docs/migration-guide.md).

<h2 id="what’s-next">
  Apa selanjutnya
</h2>

* [Gunakan Claude Code GitHub Actions dengan penyedia cloud](/docs/id/github-actions-cloud-providers): rutekan inferensi melalui Amazon Bedrock, Google Cloud's Agent Platform, atau Microsoft Foundry
* [Referensi konfigurasi](https://github.com/anthropics/claude-code-action/blob/main/docs/usage.md#inputs): daftar lengkap input action
* [Direktori examples](https://github.com/anthropics/claude-code-action/tree/main/examples): alur kerja siap pakai untuk skenario lebih lanjut
* [Code Review](/docs/id/code-review): ulasan pull request otomatis tanpa memelihara file alur kerja
