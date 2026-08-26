> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Gunakan Claude Code di web

> Konfigurasikan lingkungan cloud, skrip setup, akses jaringan, dan Docker di sandbox Anthropic. Pindahkan sesi antara web dan terminal dengan `--cloud` dan `--teleport`.

<Note>
  Claude Code di web sedang dalam pratinjau penelitian untuk pengguna Pro, Max, dan Team, serta untuk pengguna Enterprise dengan kursi premium atau kursi Chat + Claude Code.
</Note>

Claude Code di web menjalankan tugas pada infrastruktur cloud yang dikelola Anthropic di [claude.ai/code](https://claude.ai/code). Sesi bertahan bahkan jika Anda menutup browser, dan Anda dapat memantaunya dari aplikasi mobile Claude.

<Tip>
  Baru mengenal Claude Code di web? Mulai dengan [Memulai](/docs/id/web-quickstart) untuk menghubungkan akun GitHub Anda dan mengirimkan tugas pertama Anda.
</Tip>

Halaman ini mencakup:

* [Opsi autentikasi GitHub](#github-authentication-options): dua cara untuk menghubungkan GitHub
* [Lingkungan cloud](#the-cloud-environment): konfigurasi apa yang terbawa, alat apa yang diinstal, dan cara mengonfigurasi lingkungan
* [Skrip setup](#setup-scripts) dan manajemen dependensi
* [Akses jaringan](#network-access): tingkat, proxy, dan daftar putih default
* [Pindahkan tugas antara web dan terminal](#move-tasks-between-web-and-terminal) dengan `--cloud` dan `--teleport`
* [Bekerja dengan sesi](#work-with-sessions): meninjau, berbagi, mengarsipkan, menghapus
* [Auto-fix pull request](#auto-fix-pull-requests): merespons secara otomatis kegagalan CI dan komentar ulasan
* [Keamanan dan isolasi](#security-and-isolation): bagaimana sesi diisolasi
* [Batasan](#limitations): batas laju dan pembatasan platform

<h2 id="github-authentication-options">
  Opsi autentikasi GitHub
</h2>

Sesi cloud memerlukan akses ke repositori GitHub Anda untuk mengkloning kode dan mendorong cabang. Anda dapat memberikan akses dengan dua cara:

| Metode           | Cara kerjanya                                                                                        | Terbaik untuk                                                                 |
| :--------------- | :--------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------- |
| **GitHub App**   | Otorisasi Claude GitHub App selama [onboarding web](/docs/id/web-quickstart).                             | Onboarding browser; tim yang menginginkan [Auto-fix](#auto-fix-pull-requests) |
| **`/web-setup`** | Jalankan `/web-setup` di terminal Anda untuk menyinkronkan token CLI `gh` lokal ke akun Claude Anda. | Pengembang individual yang sudah menggunakan `gh`                             |

<Note>
  Dengan salah satu metode, sesi cloud dapat mengakses repositori apa pun yang dapat dilihat akun GitHub yang terhubung, bukan hanya repositori tempat Claude GitHub App diinstal. Instalasi App memungkinkan webhook PR untuk [Auto-fix](#auto-fix-pull-requests); ini bukan kontrol akses tingkat sesi. Untuk membatasi repositori mana yang dapat dijangkau tim Anda dari sesi cloud, batasi akses di GitHub itu sendiri, misalnya dengan membatasi keanggotaan tim atau repositori untuk akun GitHub yang terhubung.
</Note>

Salah satu metode berfungsi. [`/schedule`](/docs/id/routines) memeriksa salah satu bentuk akses dan meminta Anda menjalankan `/web-setup` jika tidak ada yang dikonfigurasi. Lihat [Hubungkan dari terminal Anda](/docs/id/web-quickstart#connect-from-your-terminal) untuk panduan `/web-setup`.

GitHub App diperlukan untuk [Auto-fix](#auto-fix-pull-requests), yang menggunakan App untuk menerima webhook PR. Jika Anda terhubung dengan `/web-setup` dan kemudian menginginkan Auto-fix, instal App di repositori tersebut.

Admin Team dan Enterprise dapat menonaktifkan `/web-setup` dengan toggle Quick web setup di [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code).

<Note>
  Organisasi dengan [Zero Data Retention](/docs/id/zero-data-retention) yang diaktifkan tidak dapat menggunakan `/web-setup` atau fitur sesi cloud lainnya.
</Note>

<h2 id="the-cloud-environment">
  Lingkungan cloud
</h2>

Setiap sesi berjalan di VM yang dikelola Anthropic yang segar dengan repositori Anda dikloning. Bagian ini mencakup apa yang tersedia saat sesi dimulai dan cara menyesuaikannya.

<h3 id="what’s-available-in-cloud-sessions">
  Apa yang tersedia di sesi cloud
</h3>

Sesi cloud dimulai dari klon segar repositori Anda. Apa pun yang dikomit ke repo tersedia. Apa pun yang Anda instal atau konfigurasikan hanya di mesin Anda sendiri tidak tersedia di sesi. Kebijakan organisasi Anda tiba secara terpisah melalui [pengaturan yang dikelola server](/docs/id/server-managed-settings).

|                                                                                | Tersedia di sesi cloud | Mengapa                                                                                                                                                                                                                                                                                                                            |
| :----------------------------------------------------------------------------- | :--------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `CLAUDE.md` repositori Anda                                                    | Ya                     | Bagian dari klon                                                                                                                                                                                                                                                                                                                   |
| Hook `.claude/settings.json` repositori Anda                                   | Ya                     | Bagian dari klon                                                                                                                                                                                                                                                                                                                   |
| Server MCP `.mcp.json` repositori Anda                                         | Ya                     | Bagian dari klon                                                                                                                                                                                                                                                                                                                   |
| `.claude/rules/` repositori Anda                                               | Ya                     | Bagian dari klon                                                                                                                                                                                                                                                                                                                   |
| `.claude/skills/`, `.claude/agents/`, `.claude/commands/` repositori Anda      | Ya                     | Bagian dari klon                                                                                                                                                                                                                                                                                                                   |
| Plugin yang dideklarasikan di `.claude/settings.json`                          | Ya                     | Diinstal saat startup sesi dari [marketplace](/docs/id/plugin-marketplaces) yang Anda deklarasikan. Memerlukan akses jaringan untuk menjangkau sumber marketplace                                                                                                                                                                       |
| [Pengaturan yang dikelola server](/docs/id/server-managed-settings) organisasi Anda | Ya                     | Diambil dari server Anthropic saat sesi dimulai. Lihat [Cakupan permukaan](/docs/id/model-config#surface-coverage) untuk cara `availableModels` diterapkan di sesi cloud. Pengaturan yang diterapkan ke perangkat Anda melalui MDM atau file pengaturan yang dikelola tidak berlaku, karena sesi berjalan di VM yang dikelola Anthropic |
| `~/.claude/CLAUDE.md` pengguna Anda                                            | Tidak                  | Hidup di mesin Anda, bukan di repo                                                                                                                                                                                                                                                                                                 |
| `~/.claude/skills/`, `~/.claude/agents/`, `~/.claude/commands/` pengguna Anda  | Tidak                  | Hidup di mesin Anda, bukan di repo. Komitkan ke direktori `.claude/` repo sebagai gantinya. Skill yang Anda aktifkan di claude.ai dimuat ke sesi cloud secara otomatis                                                                                                                                                             |
| Plugin yang diaktifkan hanya di pengaturan pengguna Anda                       | Tidak                  | `enabledPlugins` bersistem hidup di `~/.claude/settings.json`. Deklarasikan di `.claude/settings.json` repo sebagai gantinya                                                                                                                                                                                                       |
| Server MCP yang Anda tambahkan dengan `claude mcp add`                         | Tidak                  | Itu menulis ke konfigurasi pengguna lokal Anda, bukan repo. Deklarasikan server di [`.mcp.json`](/docs/id/mcp#project-scope) sebagai gantinya                                                                                                                                                                                           |
| Token API statis dan kredensial                                                | Tidak                  | Tidak ada penyimpanan rahasia khusus yang ada. Lihat di bawah                                                                                                                                                                                                                                                                      |
| Autentikasi interaktif seperti AWS SSO                                         | Tidak                  | Tidak didukung. SSO memerlukan login berbasis browser yang tidak dapat berjalan di sesi cloud                                                                                                                                                                                                                                      |

Untuk membuat konfigurasi Anda sendiri tersedia di sesi cloud, komitkan ke repo; kebijakan organisasi tiba secara terpisah melalui [pengaturan yang dikelola server](/docs/id/server-managed-settings).

Penyimpanan rahasia khusus belum tersedia. Baik variabel lingkungan maupun skrip setup disimpan dalam konfigurasi lingkungan, terlihat oleh siapa pun yang dapat mengedit lingkungan itu. Jika Anda memerlukan rahasia di sesi cloud, tambahkan sebagai variabel lingkungan dengan visibilitas itu dalam pikiran.

<h3 id="installed-tools">
  Alat yang diinstal
</h3>

Sesi cloud dilengkapi dengan runtime bahasa umum, alat build, dan database yang sudah diinstal sebelumnya. Tabel di bawah merangkum apa yang disertakan menurut kategori.

| Kategori      | Disertakan                                                                               |
| :------------ | :--------------------------------------------------------------------------------------- |
| **Python**    | Python 3.x dengan pip, poetry, uv, black, mypy, pytest, ruff                             |
| **Node.js**   | 20, 21, dan 22 melalui nvm, dengan npm, yarn, pnpm, bun¹, eslint, prettier, chromedriver |
| **Ruby**      | 3.1, 3.2, 3.3 dengan gem, bundler, rbenv                                                 |
| **PHP**       | 8.4 dengan Composer                                                                      |
| **Java**      | OpenJDK 21 dengan Maven dan Gradle                                                       |
| **Go**        | stabil terbaru dengan dukungan modul                                                     |
| **Rust**      | rustc dan cargo                                                                          |
| **C/C++**     | GCC, Clang, cmake, ninja, conan                                                          |
| **Docker**    | docker, dockerd, docker compose                                                          |
| **Databases** | PostgreSQL 16, Redis 7.0                                                                 |
| **Utilities** | git, jq, yq, ripgrep, tmux, vim, nano                                                    |

¹ Bun diinstal tetapi memiliki [masalah kompatibilitas proxy](#install-dependencies-with-a-sessionstart-hook) yang diketahui untuk pengambilan paket.

Untuk versi yang tepat, minta Claude menjalankan `check-tools` di sesi cloud. Perintah ini hanya ada di sesi cloud.

<h3 id="work-with-github-issues-and-pull-requests">
  Bekerja dengan masalah GitHub dan pull request
</h3>

Sesi cloud menyertakan alat GitHub bawaan yang memungkinkan Claude membaca masalah, membuat daftar pull request, mengambil diff, dan memposting komentar tanpa setup apa pun. Alat ini mengautentikasi melalui [proxy GitHub](#github-proxy) menggunakan metode apa pun yang Anda konfigurasikan di bawah [Opsi autentikasi GitHub](#github-authentication-options), jadi token Anda tidak pernah memasuki kontainer.

Anda dapat mengatur `GH_TOKEN` atau `GITHUB_TOKEN` sendiri di [pengaturan lingkungan](#configure-your-environment), atau biarkan keduanya tidak diatur dan biarkan [proxy GitHub](#github-proxy) mengautentikasi untuk Anda:

* Jika Anda mengatur token, itu melewati ke kontainer tanpa perubahan, jadi `gh` dan skrip Anda menggunakannya secara langsung.
* Jika Anda tidak mengatur keduanya, kontainer mengatur kedua variabel ke string placeholder `proxy-injected` dan proxy mengganti kredensial nyata Anda pada permintaan GitHub keluar. `gh` bekerja tanpa token Anda sendiri, tetapi skrip yang membaca `GITHUB_TOKEN` secara langsung mendapatkan placeholder, bukan token yang dapat digunakan.

Untuk memeriksa kasus mana yang berlaku untuk sesi Anda, minta Claude menjalankan `echo $GH_TOKEN`.

CLI `gh` tidak diinstal sebelumnya. Jika Anda memerlukan perintah `gh` yang tidak dicakup alat bawaan, seperti `gh release` atau `gh workflow run`, instal dan autentikasi sendiri:

<Steps>
  <Step title="Instal gh di skrip setup Anda">
    Tambahkan `apt update && apt install -y gh` ke [skrip setup](#setup-scripts) Anda.
  </Step>

  <Step title="Sediakan token jika proxy tidak menangani autentikasi">
    Jika `echo $GH_TOKEN` mencetak `proxy-injected`, [proxy GitHub](#github-proxy) mengautentikasi `gh` untuk Anda dan langkah ini tidak perlu. Jika tidak, tambahkan variabel lingkungan `GH_TOKEN` ke [pengaturan lingkungan](#configure-your-environment) Anda dengan token akses pribadi GitHub. `gh` membaca `GH_TOKEN` secara otomatis, jadi tidak ada langkah `gh auth login` yang diperlukan.
  </Step>
</Steps>

<h3 id="link-output-back-to-the-session">
  Tautkan output kembali ke sesi
</h3>

Setiap sesi cloud memiliki URL transkrip di claude.ai, dan sesi dapat membaca ID-nya sendiri dari variabel lingkungan `CLAUDE_CODE_REMOTE_SESSION_ID`. Gunakan ini untuk menempatkan tautan yang dapat dilacak di badan PR, pesan komit, posting Slack, atau laporan yang dihasilkan sehingga pengulas dapat membuka jalannya yang menghasilkannya.

Mulai dari v2.1.179, komit yang dibuat Claude di sesi web menyertakan trailer git `Claude-Session: <url>`, dan badan PR menyertakan URL sesi di barisnya sendiri. Dari v2.1.182, atur [`attribution.sessionUrl`](/docs/id/settings#attribution-settings) ke `false` untuk menghilangkan trailer dan tautan badan PR.

Untuk menyertakan tautan sesi dalam sesuatu selain komit atau PR, seperti pesan Slack yang diposting Claude atau file laporan yang ditulisnya, minta Claude menjalankan perintah berikut dan gunakan outputnya. Perintah mengonversi awalan `cse_` dalam nilai variabel lingkungan ke awalan `session_` yang diharapkan URL transkrip:

```bash theme={null}
echo "https://claude.ai/code/${CLAUDE_CODE_REMOTE_SESSION_ID/#cse_/session_}"
```

<h3 id="run-tests-start-services-and-add-packages">
  Jalankan tes, mulai layanan, dan tambahkan paket
</h3>

Claude menjalankan tes sebagai bagian dari mengerjakan tugas. Minta di prompt Anda, seperti "perbaiki tes yang gagal di `tests/`" atau "jalankan pytest setelah setiap perubahan." Pelari tes seperti pytest, jest, dan cargo test sudah diinstal sebelumnya dan bekerja tanpa setup tambahan.

PostgreSQL dan Redis sudah diinstal tetapi tidak berjalan secara default. Minta Claude untuk memulai masing-masing selama sesi:

```bash theme={null}
service postgresql start
```

```bash theme={null}
service redis-server start
```

Docker tersedia untuk menjalankan layanan terkontainerisasi. Minta Claude menjalankan `docker compose up` untuk memulai layanan proyek Anda. Akses jaringan untuk menarik gambar mengikuti [tingkat akses](#access-levels) lingkungan Anda, dan [default yang dipercaya](#default-allowed-domains) mencakup Docker Hub dan registri umum lainnya.

Jika gambar Anda besar atau lambat untuk ditarik, tambahkan `docker compose pull` atau `docker compose build` ke [skrip setup](#setup-scripts) Anda. Gambar yang ditarik disimpan di [lingkungan yang di-cache](#environment-caching), jadi setiap sesi baru memilikinya di disk. Cache menyimpan file saja, bukan proses yang berjalan, jadi Claude masih memulai kontainer setiap sesi.

Untuk menambahkan paket yang tidak diinstal sebelumnya, gunakan [skrip setup](#setup-scripts). Output skrip [di-cache](#environment-caching), jadi paket yang Anda instal di sana tersedia di awal setiap sesi tanpa menginstal ulang setiap kali. Anda juga dapat meminta Claude menginstal paket selama sesi, tetapi instalasi itu tidak bertahan di sesi lain.

<h3 id="resource-limits">
  Batas sumber daya
</h3>

Sesi cloud berjalan dengan batas sumber daya perkiraan yang mungkin berubah seiring waktu:

* 4 vCPU
* 16 GB RAM
* 30 GB disk

Tugas yang memerlukan memori jauh lebih banyak, seperti pekerjaan build besar atau tes intensif memori, mungkin gagal atau dihentikan. Untuk beban kerja di luar batas ini, gunakan [Remote Control](/docs/id/remote-control) untuk menjalankan Claude Code pada perangkat keras Anda sendiri.

<h3 id="configure-your-environment">
  Konfigurasikan lingkungan Anda
</h3>

Lingkungan mengontrol [akses jaringan](#network-access), variabel lingkungan, dan [skrip setup](#setup-scripts) yang berjalan sebelum sesi dimulai. Lihat [Alat yang diinstal](#installed-tools) untuk apa yang tersedia tanpa konfigurasi apa pun. Anda dapat mengelola lingkungan dari antarmuka web atau terminal:

| Tindakan                                     | Cara                                                                                                                                                                                                                               |
| :------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Tambahkan lingkungan                         | Pilih lingkungan saat ini untuk membuka pemilih, kemudian pilih **Tambahkan lingkungan**. Dialog mencakup nama, tingkat akses jaringan, variabel lingkungan, dan skrip setup.                                                      |
| Edit lingkungan                              | Pilih ikon cloud yang menunjukkan nama lingkungan saat ini untuk membuka pemilih, arahkan ke lingkungan, dan klik ikon pengaturan yang muncul di sebelah kanan.                                                                    |
| Arsipkan lingkungan                          | Buka lingkungan untuk pengeditan dan pilih **Arsipkan**. Lingkungan yang diarsipkan disembunyikan dari pemilih tetapi sesi yang ada terus berjalan.                                                                                |
| Atur lingkungan default untuk sesi cloud CLI | Jalankan `/remote-env` di terminal Anda. Jika Anda memiliki satu lingkungan, perintah ini menunjukkan konfigurasi saat ini Anda. `/remote-env` hanya memilih default; tambahkan, edit, dan arsipkan lingkungan dari antarmuka web. |

Variabel lingkungan menggunakan format `.env` dengan satu pasangan `KEY=value` per baris. Jangan bungkus nilai dalam tanda kutip, karena tanda kutip disimpan sebagai bagian dari nilai. Contoh ini mendefinisikan tiga variabel:

```text theme={null}
NODE_ENV=development
LOG_LEVEL=debug
DATABASE_URL=postgres://localhost:5432/myapp
```

<h3 id="organization-shared-environments">
  Lingkungan bersama organisasi
</h3>

Pemilik dan admin pada paket Team dan Enterprise dapat membuat lingkungan cloud yang dibagikan dengan setiap anggota organisasi. Lingkungan bersama muncul di pemilih lingkungan setiap anggota bersama dengan yang pribadi, jadi tim dapat menstandarkan satu konfigurasi alih-alih setiap anggota membuatnya ulang.

Kelola lingkungan bersama dari halaman **Cloud environments** di [pengaturan admin](https://claude.ai/admin-settings). Dari sana Anda dapat:

* Membuat, mengedit, dan mengarsipkan lingkungan bersama. Masing-masing memiliki bidang yang sama dengan lingkungan pribadi: nama, [tingkat akses jaringan](#access-levels), [variabel lingkungan](#configure-your-environment) dalam format `.env`, dan [skrip setup](#setup-scripts).
* Atur lingkungan default untuk organisasi.

Nilai dalam lingkungan bersama mencapai sesi setiap anggota di lingkungan itu. Seperti lingkungan pribadi, lingkungan bersama tidak memiliki penyimpanan rahasia khusus, jadi jangan sertakan rahasia.

<h2 id="setup-scripts">
  Skrip setup
</h2>

Skrip setup adalah skrip Bash yang berjalan saat sesi cloud baru dimulai, sebelum Claude Code diluncurkan. Gunakan skrip setup untuk menginstal dependensi, mengonfigurasi alat, atau mengambil apa pun yang dibutuhkan sesi yang tidak diinstal sebelumnya.

Skrip berjalan sebagai root di Ubuntu 24.04, jadi `apt install` dan sebagian besar pengelola paket bahasa bekerja.

Untuk menambahkan skrip setup, buka dialog pengaturan lingkungan dan masukkan skrip Anda di bidang **Skrip setup**.

Contoh ini menginstal CLI `gh`, yang tidak diinstal sebelumnya:

```bash theme={null}
#!/bin/bash
apt update && apt install -y gh
```

Jika skrip keluar dengan non-zero, sesi gagal dimulai. Tambahkan `|| true` ke perintah non-kritis untuk menghindari pemblokiran sesi pada kegagalan instalasi yang tidak stabil.

Jaga total runtime skrip di bawah kira-kira lima menit sehingga [cache lingkungan](#environment-caching) dapat dibangun. Jalankan instalasi independen secara paralel dengan `&` dan `wait`. Jika satu unduhan tidak akan cocok dalam batas lima menit, pindahkan ke hook [SessionStart](#setup-scripts-vs-sessionstart-hooks) yang meluncurkannya di latar belakang.

<Note>
  Skrip setup yang menginstal paket memerlukan akses jaringan untuk menjangkau registri. Akses jaringan **Trusted** default memungkinkan koneksi ke [domain paket umum](#default-allowed-domains) termasuk npm, PyPI, RubyGems, dan crates.io. Skrip akan gagal menginstal paket jika lingkungan Anda menggunakan akses jaringan **None**.
</Note>

<h3 id="environment-caching">
  Caching lingkungan
</h3>

Skrip setup berjalan pertama kali Anda memulai sesi di lingkungan. Setelah selesai, Anthropic membuat snapshot sistem file dan menggunakan kembali snapshot itu sebagai titik awal untuk sesi nanti. Sesi baru dimulai dengan dependensi, alat, dan gambar Docker Anda sudah di disk, dan langkah skrip setup dilewati. Ini membuat startup tetap cepat bahkan ketika skrip menginstal toolchain besar atau menarik gambar kontainer.

Cache menangkap file, bukan proses yang berjalan. Apa pun yang ditulis skrip setup ke disk terbawa. Layanan atau kontainer yang dimulainya tidak, jadi mulai itu per sesi dengan meminta Claude atau dengan hook [SessionStart](#setup-scripts-vs-sessionstart-hooks).

Skrip setup berjalan lagi untuk membangun kembali cache saat Anda mengubah skrip setup lingkungan atau host jaringan yang diizinkan, dan ketika cache mencapai kedaluwarsa setelah kira-kira tujuh hari. Melanjutkan sesi yang ada tidak pernah menjalankan kembali skrip setup.

Anda tidak perlu mengaktifkan caching atau mengelola snapshot sendiri.

<h3 id="setup-scripts-vs-sessionstart-hooks">
  Skrip setup vs. hook SessionStart
</h3>

Gunakan skrip setup untuk menginstal hal-hal yang dibutuhkan cloud tetapi laptop Anda sudah memiliki, seperti runtime bahasa atau alat CLI. Gunakan hook [SessionStart](/docs/id/hooks#sessionstart) untuk penyiapan proyek yang harus berjalan di mana-mana, cloud dan lokal, seperti `npm install`.

Keduanya berjalan di awal sesi, tetapi mereka termasuk di tempat yang berbeda:

|                  | Skrip setup                                                                                                 | Hook SessionStart                                                           |
| ---------------- | ----------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------- |
| Terlampir pada   | Lingkungan cloud                                                                                            | Repositori Anda                                                             |
| Dikonfigurasi di | UI lingkungan cloud                                                                                         | `.claude/settings.json` di repo Anda                                        |
| Berjalan         | Sebelum Claude Code diluncurkan, ketika tidak ada [lingkungan yang di-cache](#environment-caching) tersedia | Setelah Claude Code diluncurkan, pada setiap sesi termasuk yang dilanjutkan |
| Cakupan          | Hanya lingkungan cloud                                                                                      | Lokal dan cloud                                                             |

Hook SessionStart juga dapat didefinisikan di `~/.claude/settings.json` tingkat pengguna Anda secara lokal, tetapi pengaturan tingkat pengguna tidak terbawa ke sesi cloud. Di cloud, hook berasal dari repo dan dari [pengaturan yang dikelola server](/docs/id/server-managed-settings) organisasi Anda.

<h3 id="install-dependencies-with-a-sessionstart-hook">
  Instal dependensi dengan hook SessionStart
</h3>

Untuk menginstal dependensi hanya di sesi cloud, tambahkan hook SessionStart ke `.claude/settings.json` repo Anda:

```json theme={null}
{
  "hooks": {
    "SessionStart": [
      {
        "matcher": "startup|resume",
        "hooks": [
          {
            "type": "command",
            "command": "\"$CLAUDE_PROJECT_DIR\"/scripts/install_pkgs.sh"
          }
        ]
      }
    ]
  }
}
```

Buat skrip di `scripts/install_pkgs.sh` dan buat dapat dieksekusi dengan `chmod +x`. Variabel lingkungan `CLAUDE_CODE_REMOTE` diatur ke `true` di sesi cloud, jadi Anda dapat menggunakannya untuk melewati eksekusi lokal:

```bash theme={null}
#!/bin/bash

if [ "$CLAUDE_CODE_REMOTE" != "true" ]; then
  exit 0
fi

npm install
pip install -r requirements.txt
exit 0
```

Hook SessionStart memiliki beberapa batasan di sesi cloud:

* **Tidak ada scoping khusus cloud**: hook berjalan di sesi lokal dan cloud. Untuk melewati eksekusi lokal, periksa variabel lingkungan `CLAUDE_CODE_REMOTE` seperti yang ditunjukkan di atas.
* **Memerlukan akses jaringan**: perintah install perlu menjangkau registri paket. Jika lingkungan Anda menggunakan akses jaringan **None**, hook ini gagal. [Daftar putih default](#default-allowed-domains) di bawah **Trusted** mencakup npm, PyPI, RubyGems, dan crates.io.
* **Kompatibilitas proxy**: semua lalu lintas keluar melewati [proxy keamanan](#security-proxy). Beberapa pengelola paket tidak bekerja dengan benar dengan proxy ini. Bun adalah contoh yang diketahui.
* **Menambah latensi startup**: hook berjalan setiap kali sesi dimulai atau dilanjutkan, tidak seperti skrip setup yang mendapat manfaat dari [caching lingkungan](#environment-caching). Jaga skrip install tetap cepat dengan memeriksa apakah dependensi sudah ada sebelum menginstal ulang.

Untuk mempertahankan variabel lingkungan untuk perintah Bash berikutnya, tulis ke file di `$CLAUDE_ENV_FILE`. Lihat [hook SessionStart](/docs/id/hooks#sessionstart) untuk detail.

Mengganti gambar dasar dengan gambar Docker Anda sendiri belum didukung. Gunakan skrip setup untuk menginstal apa yang Anda butuhkan di atas [gambar yang disediakan](#installed-tools), atau jalankan gambar Anda sebagai kontainer bersama Claude dengan `docker compose`.

<h2 id="network-access">
  Akses jaringan
</h2>

Akses jaringan mengontrol koneksi keluar dari lingkungan cloud. Setiap lingkungan menentukan satu tingkat akses, dan Anda dapat memperluas dengan domain yang diizinkan khusus. Default adalah **Trusted**, yang memungkinkan registri paket dan [domain yang diizinkan lainnya](#default-allowed-domains).

Untuk mengubah akses jaringan lingkungan, [buka untuk pengeditan](#configure-your-environment) dan gunakan pemilih **Akses jaringan** di dialog. Tidak ada halaman Environments terpisah. Ikon cloud muncul di mana pun Anda memulai sesi cloud atau mengonfigurasi [routine](/docs/id/routines#environments-and-network-access).

<Note>
  Lalu lintas konektor MCP dirutekan melalui server Anthropic, jadi konektor yang Anda aktifkan pada sesi atau routine berfungsi tanpa menambahkan host mereka ke **Domain yang diizinkan**. Konektor dikonfigurasi per sesi atau per routine; hapus yang tidak Anda butuhkan untuk membatasi alat mana yang dapat dijangkau Claude. Ini bergantung pada saluran terikat Anthropic yang sama yang dicatat di bawah [Keamanan dan isolasi](#security-and-isolation).
</Note>

<h3 id="access-levels">
  Tingkat akses
</h3>

Pilih tingkat akses saat Anda membuat atau mengedit lingkungan:

| Tingkat     | Koneksi keluar                                                                            |
| :---------- | :---------------------------------------------------------------------------------------- |
| **None**    | Tidak ada akses jaringan keluar                                                           |
| **Trusted** | [Domain yang diizinkan](#default-allowed-domains) saja: registri paket, GitHub, SDK cloud |
| **Full**    | Domain apa pun                                                                            |
| **Custom**  | Daftar putih Anda sendiri, secara opsional termasuk default                               |

Operasi GitHub menggunakan [proxy terpisah](#github-proxy) yang independen dari pengaturan ini.

<h3 id="allow-specific-domains">
  Izinkan domain tertentu
</h3>

Untuk mengizinkan domain yang tidak ada di daftar Trusted, pilih **Custom** di pengaturan akses jaringan lingkungan. Bidang **Domain yang diizinkan** muncul. Masukkan satu domain per baris:

```text theme={null}
api.example.com
*.internal.example.com
registry.example.com
```

Gunakan `*.` untuk pencocokan subdomain wildcard. Periksa **Juga sertakan daftar default pengelola paket umum** untuk menjaga [domain Trusted](#default-allowed-domains) bersama entri khusus Anda, atau biarkan tidak dicentang untuk hanya mengizinkan apa yang Anda daftarkan.

Domain yang diizinkan dikonfigurasi per lingkungan. Tidak ada daftar putih tingkat organisasi yang dapat didorong Owners ke lingkungan semua pengguna; [pengaturan yang dikelola server](/docs/id/server-managed-settings) dapat membatasi sesi cloud tetapi tidak dapat menambahkan domain yang diizinkan.

<h3 id="github-proxy">
  Proxy GitHub
</h3>

Untuk keamanan, semua operasi GitHub melewati layanan proxy khusus yang menjaga kredensial GitHub asli Anda di luar sandbox. Proxy mengautentikasi dua jenis lalu lintas:

* Interaksi Git: klien git di dalam sandbox menggunakan kredensial bersistem yang dibuat khusus, yang diverifikasi proxy dan diterjemahkan ke token autentikasi GitHub aktual Anda
* Permintaan API GitHub: proxy mengganti kredensial asli Anda pada permintaan dari alat GitHub bawaan, dan dari `gh` ketika sesi Anda menetapkan placeholder `proxy-injected` yang dijelaskan dalam [Bekerja dengan masalah dan permintaan tarik GitHub](#work-with-github-issues-and-pull-requests)

Proxy juga membatasi operasi git push ke cabang kerja saat ini untuk keamanan, dan memungkinkan kloning, pengambilan, dan operasi PR sambil mempertahankan batas keamanan.

Proxy membatasi permintaan API GitHub dan aset rilis ke repositori yang terlampir pada sesi, terlepas dari [tingkat akses jaringan](#access-levels) lingkungan. Skrip setup yang mengunduh aset rilis dari repositori yang tidak terlampir mengembalikan 403. File yang berkomitmen dari repositori publik diambil melalui `raw.githubusercontent.com`, yang ditangani oleh [proxy keamanan](#security-proxy) sebagai gantinya. Domain itu ada di daftar [Trusted](#default-allowed-domains) default, jadi file tetap dapat diakses kecuali [tingkat akses](#access-levels) lingkungan mengecualikannya.

<h3 id="security-proxy">
  Proxy keamanan
</h3>

Lingkungan berjalan di belakang proxy jaringan HTTP/HTTPS untuk tujuan keamanan dan pencegahan penyalahgunaan. Semua lalu lintas internet keluar melewati proxy ini, yang menyediakan:

* Perlindungan terhadap permintaan berbahaya
* Pembatasan laju dan pencegahan penyalahgunaan
* Penyaringan konten untuk keamanan yang ditingkatkan
* Jejak audit tingkat DNS dari nama host yang diminta

<h3 id="default-allowed-domains">
  Domain yang diizinkan secara default
</h3>

Saat menggunakan akses jaringan **Trusted**, domain berikut diizinkan secara default. Domain yang ditandai dengan `*` menunjukkan pencocokan subdomain wildcard, jadi `*.gcr.io` memungkinkan subdomain apa pun dari `gcr.io`.

<AccordionGroup>
  <Accordion title="Layanan Anthropic">
    * api.anthropic.com
    * statsig.anthropic.com
    * docs.claude.com
    * platform.claude.com
    * code.claude.com
    * claude.ai
  </Accordion>

  <Accordion title="Kontrol versi">
    * github.com
    * [www.github.com](http://www.github.com)
    * api.github.com
    * npm.pkg.github.com
    * raw\.githubusercontent.com
    * pkg-npm.githubusercontent.com
    * objects.githubusercontent.com
    * release-assets.githubusercontent.com
    * codeload.github.com
    * avatars.githubusercontent.com
    * camo.githubusercontent.com
    * gist.github.com
    * gitlab.com
    * [www.gitlab.com](http://www.gitlab.com)
    * registry.gitlab.com
    * bitbucket.org
    * [www.bitbucket.org](http://www.bitbucket.org)
    * api.bitbucket.org
  </Accordion>

  <Accordion title="Registri kontainer">
    * registry-1.docker.io
    * auth.docker.io
    * index.docker.io
    * hub.docker.com
    * [www.docker.com](http://www.docker.com)
    * production.cloudflare.docker.com
    * download.docker.com
    * gcr.io
    * \*.gcr.io
    * ghcr.io
    * mcr.microsoft.com
    * \*.data.mcr.microsoft.com
    * public.ecr.aws
  </Accordion>

  <Accordion title="Platform cloud">
    * cloud.google.com
    * accounts.google.com
    * gcloud.google.com
    * \*.googleapis.com
    * storage.googleapis.com
    * compute.googleapis.com
    * container.googleapis.com
    * azure.com
    * portal.azure.com
    * microsoft.com
    * [www.microsoft.com](http://www.microsoft.com)
    * \*.microsoftonline.com
    * packages.microsoft.com
    * dotnet.microsoft.com
    * dot.net
    * visualstudio.com
    * dev.azure.com
    * \*.amazonaws.com
    * \*.api.aws
    * oracle.com
    * [www.oracle.com](http://www.oracle.com)
    * java.com
    * [www.java.com](http://www.java.com)
    * java.net
    * [www.java.net](http://www.java.net)
    * download.oracle.com
    * yum.oracle.com
  </Accordion>

  <Accordion title="Pengelola paket JavaScript dan Node">
    * registry.npmjs.org
    * [www.npmjs.com](http://www.npmjs.com)
    * [www.npmjs.org](http://www.npmjs.org)
    * npmjs.com
    * npmjs.org
    * yarnpkg.com
    * registry.yarnpkg.com
  </Accordion>

  <Accordion title="Pengelola paket Python">
    * pypi.org
    * [www.pypi.org](http://www.pypi.org)
    * files.pythonhosted.org
    * pythonhosted.org
    * test.pypi.org
    * pypi.python.org
    * pypa.io
    * [www.pypa.io](http://www.pypa.io)
  </Accordion>

  <Accordion title="Pengelola paket Ruby">
    * rubygems.org
    * [www.rubygems.org](http://www.rubygems.org)
    * api.rubygems.org
    * index.rubygems.org
    * ruby-lang.org
    * [www.ruby-lang.org](http://www.ruby-lang.org)
    * rubyforge.org
    * [www.rubyforge.org](http://www.rubyforge.org)
    * rubyonrails.org
    * [www.rubyonrails.org](http://www.rubyonrails.org)
    * rvm.io
    * get.rvm.io
  </Accordion>

  <Accordion title="Pengelola paket Rust">
    * crates.io
    * [www.crates.io](http://www.crates.io)
    * index.crates.io
    * static.crates.io
    * rustup.rs
    * static.rust-lang.org
    * [www.rust-lang.org](http://www.rust-lang.org)
  </Accordion>

  <Accordion title="Pengelola paket Go">
    * proxy.golang.org
    * sum.golang.org
    * index.golang.org
    * golang.org
    * [www.golang.org](http://www.golang.org)
    * goproxy.io
    * pkg.go.dev
  </Accordion>

  <Accordion title="Pengelola paket JVM">
    * maven.org
    * repo.maven.org
    * central.maven.org
    * repo1.maven.org
    * repo.maven.apache.org
    * jcenter.bintray.com
    * gradle.org
    * [www.gradle.org](http://www.gradle.org)
    * services.gradle.org
    * plugins.gradle.org
    * kotlinlang.org
    * [www.kotlinlang.org](http://www.kotlinlang.org)
    * spring.io
    * repo.spring.io
  </Accordion>

  <Accordion title="Pengelola paket lainnya">
    * packagist.org (PHP Composer)
    * [www.packagist.org](http://www.packagist.org)
    * repo.packagist.org
    * nuget.org (.NET NuGet)
    * [www.nuget.org](http://www.nuget.org)
    * api.nuget.org
    * pub.dev (Dart/Flutter)
    * api.pub.dev
    * hex.pm (Elixir/Erlang)
    * [www.hex.pm](http://www.hex.pm)
    * cpan.org (Perl CPAN)
    * [www.cpan.org](http://www.cpan.org)
    * metacpan.org
    * [www.metacpan.org](http://www.metacpan.org)
    * api.metacpan.org
    * cocoapods.org (iOS/macOS)
    * [www.cocoapods.org](http://www.cocoapods.org)
    * cdn.cocoapods.org
    * haskell.org
    * [www.haskell.org](http://www.haskell.org)
    * hackage.haskell.org
    * swift.org
    * [www.swift.org](http://www.swift.org)
  </Accordion>

  <Accordion title="Distribusi Linux">
    * archive.ubuntu.com
    * security.ubuntu.com
    * ubuntu.com
    * [www.ubuntu.com](http://www.ubuntu.com)
    * \*.ubuntu.com
    * ppa.launchpad.net
    * launchpad.net
    * [www.launchpad.net](http://www.launchpad.net)
    * \*.nixos.org
  </Accordion>

  <Accordion title="Alat pengembangan dan platform">
    * dl.k8s.io (Kubernetes)
    * pkgs.k8s.io
    * k8s.io
    * [www.k8s.io](http://www.k8s.io)
    * releases.hashicorp.com (HashiCorp)
    * apt.releases.hashicorp.com
    * rpm.releases.hashicorp.com
    * archive.releases.hashicorp.com
    * hashicorp.com
    * [www.hashicorp.com](http://www.hashicorp.com)
    * repo.anaconda.com (Anaconda/Conda)
    * conda.anaconda.org
    * anaconda.org
    * [www.anaconda.com](http://www.anaconda.com)
    * anaconda.com
    * continuum.io
    * apache.org (Apache)
    * [www.apache.org](http://www.apache.org)
    * archive.apache.org
    * downloads.apache.org
    * eclipse.org (Eclipse)
    * [www.eclipse.org](http://www.eclipse.org)
    * download.eclipse.org
    * nodejs.org (Node.js)
    * [www.nodejs.org](http://www.nodejs.org)
    * developer.apple.com
    * developer.android.com
    * pkg.stainless.com
    * binaries.prisma.sh
  </Accordion>

  <Accordion title="Layanan cloud dan monitoring">
    * statsig.com
    * [www.statsig.com](http://www.statsig.com)
    * api.statsig.com
    * sentry.io
    * \*.sentry.io
    * downloads.sentry-cdn.com
    * http-intake.logs.datadoghq.com
    * browser-intake-us5-datadoghq.com
    * \*.datadoghq.com
    * \*.datadoghq.eu
    * api.honeycomb.io
  </Accordion>

  <Accordion title="Pengiriman konten dan mirror">
    * sourceforge.net
    * \*.sourceforge.net
    * packagecloud.io
    * \*.packagecloud.io
    * fonts.googleapis.com
    * fonts.gstatic.com
  </Accordion>

  <Accordion title="Skema dan konfigurasi">
    * json-schema.org
    * [www.json-schema.org](http://www.json-schema.org)
    * json.schemastore.org
    * [www.schemastore.org](http://www.schemastore.org)
  </Accordion>

  <Accordion title="Model Context Protocol">
    * \*.modelcontextprotocol.io
  </Accordion>
</AccordionGroup>

<h2 id="move-tasks-between-web-and-terminal">
  Pindahkan tugas antara web dan terminal
</h2>

Alur kerja ini memerlukan [Claude Code CLI](/docs/id/quickstart) yang masuk ke akun claude.ai yang sama. Anda dapat memulai sesi cloud baru dari terminal, atau menarik sesi cloud ke terminal untuk melanjutkan secara lokal. Sesi cloud bertahan bahkan jika Anda menutup laptop, dan Anda dapat memantaunya dari mana saja termasuk aplikasi mobile Claude.

<Note>
  Dari CLI, handoff sesi adalah satu arah: Anda dapat menarik sesi cloud ke terminal Anda dengan `--teleport`, tetapi Anda tidak dapat mendorong sesi terminal yang ada ke web. Bendera `--cloud` membuat sesi cloud baru untuk repositori saat ini Anda. [Aplikasi Desktop](/docs/id/desktop#continue-in-another-surface) menyediakan menu Continue in yang dapat mengirim sesi lokal ke web.
</Note>

<h3 id="from-terminal-to-web">
  Dari terminal ke web
</h3>

Mulai sesi cloud dari baris perintah dengan bendera `--cloud`:

```bash theme={null}
claude --cloud "Fix the authentication bug in src/auth/login.ts"
```

Ini membuat sesi cloud baru di claude.ai. Sesi mengkloning remote GitHub direktori saat ini Anda di cabang saat ini Anda, jadi dorong terlebih dahulu jika Anda memiliki komit lokal, karena VM mengkloning dari GitHub daripada mesin Anda. `--cloud` bekerja dengan satu repositori pada satu waktu. Tugas berjalan di cloud sementara Anda terus bekerja secara lokal. Ejaan `--remote` yang lebih lama masih berfungsi sebagai alias yang sudah usang untuk `--cloud`.

Mulai dari v2.1.195, CLI menampilkan daftar periksa langsung dari langkah-langkah penyiapan, seperti mengkloning repositori dan menjalankan [skrip penyiapan Anda](#setup-scripts), sementara kontainer cloud dimulai. Pesan yang Anda ketik saat kontainer sedang disediakan antri dan dikirim setelah sesi siap.

<Note>
  `--cloud` membuat sesi cloud. `--remote-control` tidak terkait: itu mengekspos sesi CLI lokal untuk pemantauan dari web. Lihat [Remote Control](/docs/id/remote-control).
</Note>

Gunakan `/tasks` di Claude Code CLI untuk memeriksa kemajuan, atau buka sesi di claude.ai atau aplikasi mobile Claude untuk berinteraksi langsung. Dari sana Anda dapat mengarahkan Claude, memberikan umpan balik, atau menjawab pertanyaan seperti percakapan lainnya.

<h4 id="tips-for-cloud-tasks">
  Tips untuk tugas cloud
</h4>

**Rencanakan secara lokal, jalankan dari jarak jauh**: untuk tugas kompleks, mulai Claude dalam plan mode untuk berkolaborasi pada pendekatan, kemudian kirim pekerjaan ke cloud:

```bash theme={null}
claude --permission-mode plan
```

Dalam plan mode, Claude membaca file, menjalankan perintah untuk menjelajahi, dan mengusulkan rencana tanpa mengedit kode sumber. Setelah Anda puas, simpan rencana ke repo, komit, dan dorong sehingga VM cloud dapat mengklonnya. Kemudian mulai sesi cloud untuk eksekusi otonom:

```bash theme={null}
claude --cloud "Execute the migration plan in docs/migration-plan.md"
```

Pola ini memberi Anda kontrol atas strategi sambil membiarkan Claude mengeksekusi secara otonom di cloud.

**Rencanakan di cloud dengan ultraplan**: untuk menyusun dan meninjau rencana itu sendiri dalam sesi web, gunakan [ultraplan](/docs/id/ultraplan). Claude menghasilkan rencana di Claude Code di web sementara Anda terus bekerja, kemudian Anda berkomentar pada bagian di browser Anda dan memilih untuk mengeksekusi dari jarak jauh atau mengirim rencana kembali ke terminal Anda.

**Jalankan tugas secara paralel**: setiap perintah `--cloud` membuat sesi cloud sendiri yang berjalan secara independen. Anda dapat memulai beberapa tugas dan semuanya akan berjalan secara bersamaan dalam sesi terpisah:

```bash theme={null}
claude --cloud "Fix the flaky test in auth.spec.ts"
claude --cloud "Update the API documentation"
claude --cloud "Refactor the logger to use structured output"
```

Pantau semua sesi dengan `/tasks` di Claude Code CLI. Ketika sesi selesai, Anda dapat membuat PR dari antarmuka web atau [teleport](#from-web-to-terminal) sesi ke terminal Anda untuk melanjutkan bekerja.

<h4 id="send-local-repositories-without-github">
  Kirim repositori lokal tanpa GitHub
</h4>

Saat Anda menjalankan `claude --cloud` dari repositori yang tidak terhubung ke GitHub, Claude Code membundel repositori lokal Anda dan mengunggahnya langsung ke sesi cloud. Bundle mencakup riwayat repositori lengkap Anda di semua cabang, ditambah perubahan yang tidak dikomit ke file yang dilacak.

Fallback ini diaktifkan secara otomatis saat akses GitHub tidak tersedia. Untuk memaksanya bahkan saat GitHub terhubung, atur `CCR_FORCE_BUNDLE=1`:

```bash theme={null}
CCR_FORCE_BUNDLE=1 claude --cloud "Run the test suite and fix any failures"
```

Repositori yang dibundel harus memenuhi batas ini:

* Direktori harus berupa repositori git dengan setidaknya satu komit
* Repositori yang dibundel harus di bawah 100 MB. Repositori yang lebih besar kembali ke bundling hanya cabang saat ini, kemudian ke snapshot kerja tunggal yang diperas, dan gagal hanya jika snapshot masih terlalu besar
* File yang tidak dilacak tidak disertakan; jalankan `git add` pada file yang ingin dilihat sesi cloud
* Sesi yang dibuat dari bundle tidak dapat didorong kembali ke remote kecuali Anda juga memiliki [autentikasi GitHub](#github-authentication-options) yang dikonfigurasi

<h3 id="from-web-to-terminal">
  Dari web ke terminal
</h3>

Tarik sesi cloud ke terminal Anda menggunakan salah satu dari ini:

* **Menggunakan `--teleport`**: dari baris perintah, jalankan `claude --teleport` untuk pemilih sesi interaktif, atau `claude --teleport <session-id>` untuk melanjutkan sesi tertentu secara langsung. Jika Anda memiliki perubahan yang tidak dikomit, Anda akan diminta untuk menyimpannya terlebih dahulu.
* **Menggunakan `/teleport`**: di dalam sesi CLI yang ada, jalankan `/teleport` atau `/tp` untuk membuka pemilih sesi yang sama tanpa memulai ulang Claude Code.
* **Dari `/tasks`**: jalankan `/tasks` untuk melihat sesi latar belakang Anda, kemudian tekan `t` untuk teleport ke salah satunya.
* **Dari antarmuka web**: pilih **Buka di CLI** untuk menyalin perintah yang dapat Anda tempel ke terminal Anda.

Saat Anda teleport sesi, Claude memverifikasi Anda berada di repositori yang benar, mengambil dan checkout cabang dari sesi cloud, dan memuat riwayat percakapan lengkap ke terminal Anda.

`--teleport` berbeda dari `--resume`. `--resume` membuka kembali percakapan dari riwayat lokal mesin ini dan tidak mencantumkan sesi cloud; `--teleport` menarik sesi cloud dan cabangnya.

<h4 id="teleport-requirements">
  Persyaratan teleport
</h4>

Teleport memeriksa persyaratan ini sebelum melanjutkan sesi. Jika ada persyaratan yang tidak terpenuhi, Anda akan melihat kesalahan atau diminta untuk menyelesaikan masalah.

| Persyaratan           | Detail                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| --------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Status git bersih     | Direktori kerja Anda harus tidak memiliki perubahan yang tidak dikomit. Teleport meminta Anda untuk menyimpan perubahan jika diperlukan.                                                                                                                                                                                                                                                                                                             |
| Repositori yang benar | Anda harus menjalankan `--teleport` dari checkout repositori yang sama, bukan fork. Mulai dari v2.1.199, Claude Code menerima checkout bahkan ketika tidak dapat mengurai remote menjadi nama host, seperti alias host SSH seperti `git@work:owner/repo.git` atau bentuk pendek yang ditulis ulang `insteadOf`. Ini menampilkan prompt konfirmasi terlebih dahulu, dan hanya ketika pemilik remote dan nama repositori cocok dengan repositori sesi. |
| Cabang tersedia       | Cabang dari sesi cloud harus telah didorong ke remote. Teleport secara otomatis mengambil dan checkout.                                                                                                                                                                                                                                                                                                                                              |
| Akun yang sama        | Anda harus diautentikasi ke akun claude.ai yang sama yang digunakan dalam sesi cloud.                                                                                                                                                                                                                                                                                                                                                                |

<h4 id="teleport-is-unavailable">
  `--teleport` tidak tersedia
</h4>

Teleport memerlukan autentikasi langganan claude.ai. Jika Anda diautentikasi melalui kunci API, Amazon Bedrock, Google Cloud's Agent Platform, atau Microsoft Foundry, jalankan `/login` untuk masuk dengan akun claude.ai Anda sebagai gantinya. Jika Anda sudah masuk melalui claude.ai dan `--teleport` masih tidak tersedia, organisasi Anda mungkin telah menonaktifkan sesi cloud.

<h2 id="work-with-sessions">
  Bekerja dengan sesi
</h2>

Sesi muncul di sidebar di claude.ai/code. Dari sana Anda dapat meninjau perubahan, berbagi dengan rekan kerja, mengarsipkan pekerjaan yang selesai, atau menghapus sesi secara permanen.

<h3 id="manage-context">
  Kelola konteks
</h3>

Sesi cloud mendukung [perintah bawaan](/docs/id/commands) yang menghasilkan output teks. Perintah yang hanya berjalan di antarmuka terminal, seperti `/plugin` atau `/resume`, tidak tersedia. Perintah yang membuka pemilih atau panel di terminal berperilaku berbeda di sesi cloud:

* **`/model`, `/effort`, `/fast`, `/color`, dan `/rename`**: teruskan nilai sebagai argumen, misalnya `/model sonnet`, alih-alih membuka pemilih terminal atau slider. Bentuk argumen memerlukan Claude Code v2.1.205 atau lebih baru di lingkungan sesi dan mengikuti [catatan ketersediaan](/docs/id/commands#all-commands) setiap perintah: `/effort` melaporkan `Not applied` sementara [launch-default effort hold](/docs/id/model-config#adjust-effort-level) model sedang berlaku, dan `/fast` hanya bekerja di sesi yang dimulai dengan mode cepat diaktifkan.
* **`/config`**: di web, membuka bagian Claude Code dari pengaturan Anda alih-alih menetapkan nilai, dan teks setelah perintah, termasuk `key=value`, diabaikan. Untuk mengubah pengaturan sesi cloud, gunakan [variabel lingkungan](#configure-your-environment) atau komit [file pengaturan](/docs/id/settings) ke repositori.

Untuk manajemen konteks khususnya:

| Perintah   | Bekerja di sesi cloud | Catatan                                                                                                                   |
| :--------- | :-------------------- | :------------------------------------------------------------------------------------------------------------------------ |
| `/compact` | Ya                    | Merangkum percakapan untuk membebaskan konteks. Menerima instruksi fokus opsional seperti `/compact keep the test output` |
| `/context` | Ya                    | Menunjukkan apa yang saat ini ada di jendela konteks                                                                      |
| `/clear`   | Tidak                 | Mulai sesi baru dari sidebar sebagai gantinya                                                                             |

Auto-compaction berjalan secara otomatis saat jendela konteks mendekati kapasitas. Untuk memicunya lebih awal, atur [`CLAUDE_AUTOCOMPACT_PCT_OVERRIDE`](/docs/id/env-vars) di [variabel lingkungan](#configure-your-environment) Anda. Misalnya, `CLAUDE_AUTOCOMPACT_PCT_OVERRIDE=70` mengompak pada kapasitas 70% daripada menunggu hingga jendela hampir penuh. Untuk mengubah ukuran jendela efektif untuk perhitungan compaction, gunakan [`CLAUDE_CODE_AUTO_COMPACT_WINDOW`](/docs/id/env-vars).

[Subagents](/docs/id/sub-agents) bekerja dengan cara yang sama seperti secara lokal. Claude dapat menelurkan mereka dengan alat Task untuk mengalihkan penelitian atau pekerjaan paralel ke jendela konteks terpisah, menjaga percakapan utama lebih ringan. Subagents yang didefinisikan di `.claude/agents/` repo Anda diambil secara otomatis.

[Tim agen](/docs/id/agent-teams) dimatikan secara default tetapi dapat diaktifkan dengan menambahkan `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS=1` ke [variabel lingkungan](#configure-your-environment) Anda.

<h3 id="review-changes">
  Tinjau perubahan
</h3>

Setiap sesi menunjukkan indikator diff dengan baris yang ditambahkan dan dihapus, seperti `+42 -18`. Pilih untuk membuka tampilan diff, tinggalkan komentar inline pada baris tertentu, dan kirimkan ke Claude dengan pesan berikutnya. Lihat [Tinjau dan ulangi](/docs/id/web-quickstart#review-and-iterate) untuk panduan lengkap termasuk pembuatan PR. Untuk membuat Claude memantau PR untuk kegagalan CI dan komentar ulasan secara otomatis, lihat [Auto-fix pull requests](#auto-fix-pull-requests).

<h3 id="share-sessions">
  Bagikan sesi
</h3>

Untuk berbagi sesi, alihkan visibilitasnya sesuai dengan jenis akun di bawah ini. Setelah itu, bagikan tautan sesi apa adanya. Penerima melihat status terbaru saat mereka membuka tautan, tetapi tampilan mereka tidak diperbarui secara real-time.

<h4 id="share-from-an-enterprise-or-team-account">
  Bagikan dari akun Enterprise atau Team
</h4>

Untuk akun Enterprise dan Team, dua opsi visibilitas adalah **Private** dan **Team**. Visibilitas Team membuat sesi terlihat oleh anggota lain dari organisasi claude.ai Anda. [Claude in Slack](/docs/id/slack) sesi secara otomatis dibagikan dengan visibilitas Team.

Verifikasi akses repositori diaktifkan secara default, berdasarkan akun GitHub yang terhubung ke akun penerima Anda. Nama tampilan akun Anda terlihat oleh semua penerima dengan akses.

<h4 id="share-from-a-max-or-pro-account">
  Bagikan dari akun Max atau Pro
</h4>

Untuk akun Max dan Pro, dua opsi visibilitas adalah **Private** dan **Public**. Visibilitas Public membuat sesi terlihat oleh pengguna mana pun yang masuk ke claude.ai.

Periksa sesi Anda untuk konten sensitif sebelum berbagi. Sesi dapat berisi kode dan kredensial dari repositori GitHub pribadi. Verifikasi akses repositori tidak diaktifkan secara default.

Untuk memerlukan penerima memiliki akses repositori, atau untuk menyembunyikan nama Anda dari sesi bersama, buka Settings > Claude Code > Sharing settings.

<h3 id="archive-sessions">
  Arsipkan sesi
</h3>

Anda dapat mengarsipkan sesi untuk menjaga daftar sesi Anda tetap terorganisir. Sesi yang diarsipkan disembunyikan dari daftar sesi default tetapi dapat dilihat dengan memfilter sesi yang diarsipkan.

Untuk mengarsipkan sesi, arahkan ke sesi di sidebar dan pilih ikon arsip.

<h3 id="delete-sessions">
  Hapus sesi
</h3>

Menghapus sesi secara permanen menghapus sesi dan datanya. Tindakan ini tidak dapat dibatalkan. Anda dapat menghapus sesi dengan dua cara:

* **Dari sidebar**: filter untuk sesi yang diarsipkan, kemudian arahkan ke sesi yang ingin Anda hapus dan pilih ikon hapus
* **Dari menu sesi**: buka sesi, pilih dropdown di sebelah judul sesi, dan pilih **Hapus**

Anda akan diminta untuk mengonfirmasi sebelum sesi dihapus.

<h2 id="auto-fix-pull-requests">
  Auto-fix pull requests
</h2>

Claude dapat memantau pull request dan secara otomatis merespons kegagalan CI dan komentar ulasan. Claude berlangganan aktivitas GitHub di PR, dan ketika pemeriksaan gagal atau pengulas meninggalkan komentar, Claude menyelidiki dan mendorong perbaikan jika ada yang jelas.

<Note>
  Auto-fix memerlukan Claude GitHub App untuk diinstal di repositori Anda. Jika Anda belum melakukannya, instal dari [halaman GitHub App](https://github.com/apps/claude) atau saat diminta selama [setup](/docs/id/web-quickstart#connect-github-and-create-an-environment).
</Note>

Ada beberapa cara untuk mengaktifkan auto-fix tergantung di mana PR berasal dan perangkat apa yang Anda gunakan:

* **PR yang dibuat di Claude Code di web**: buka bilah status CI dan pilih **Auto-fix**
* **Dari terminal Anda**: jalankan [`/autofix-pr`](/docs/id/commands) saat berada di cabang PR. Claude Code mendeteksi PR terbuka dengan `gh`, menelurkan sesi web, dan mengaktifkan auto-fix dalam satu langkah
* **Dari aplikasi mobile**: beri tahu Claude untuk auto-fix PR, misalnya "watch this PR and fix any CI failures or review comments"
* **PR yang ada**: tempel URL PR ke sesi dan beri tahu Claude untuk auto-fix

Auto-fix adalah toggle per-PR. Untuk berhenti memantau, buka bilah status CI dalam sesi web dan hapus toggle **Auto-fix**, atau beri tahu Claude untuk berhenti memantau PR.

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

* **Mesin virtual terisolasi**: setiap sesi berjalan di VM yang terisolasi dan dikelola Anthropic
* **Kontrol akses jaringan**: akses jaringan dibatasi secara default, dan dapat dinonaktifkan. Saat berjalan dengan akses jaringan dinonaktifkan, Claude Code masih dapat berkomunikasi dengan API Anthropic, yang mungkin memungkinkan data keluar dari VM.
* **Perlindungan kredensial**: kredensial sensitif seperti kredensial git atau kunci penandatanganan tidak pernah ada di dalam sandbox dengan Claude Code. Autentikasi ditangani melalui proxy aman menggunakan kredensial bersistem.
* **Analisis aman**: kode dianalisis dan dimodifikasi dalam VM terisolasi sebelum membuat PR

<h2 id="troubleshooting">
  Troubleshooting
</h2>

Untuk kesalahan API runtime yang muncul dalam percakapan seperti `API Error: 500`, `529 Overloaded`, `429`, atau `Prompt is too long`, lihat [referensi Error](/docs/id/errors). Kesalahan tersebut dan perbaikannya dibagikan dengan CLI dan Desktop app. Bagian di bawah mencakup masalah khusus untuk sesi cloud.

<h3 id="session-creation-failed">
  Session creation failed
</h3>

Jika sesi baru gagal dimulai dengan `Session creation failed` atau macet di provisioning, Claude Code tidak dapat mengalokasikan lingkungan cloud.

* Periksa [status.claude.com](https://status.claude.com) untuk insiden sesi cloud
* Coba lagi setelah satu menit, karena kapasitas disediakan sesuai permintaan
* Konfirmasi repositori Anda dapat dijangkau. Akun GitHub yang terhubung harus memiliki akses ke repositori di GitHub, baik melalui otorisasi Claude GitHub App atau token `gh` yang disinkronkan melalui `/web-setup`. Menginstal App di repositori tidak diperlukan. Lihat [GitHub authentication options](#github-authentication-options).

<h3 id="remote-control-session-expired-or-access-denied">
  Remote Control session expired or access denied
</h3>

`--teleport` terhubung melalui infrastruktur sesi Remote Control yang sama yang digunakan sesi cloud, jadi kesalahan autentikasi dan kedaluwarsa sesi muncul dengan wording Remote Control. Anda mungkin melihat `Remote Control session expired` atau `Access denied`. Token koneksi berumur pendek dan dibatasi pada akun Anda.

* Jalankan `/login` secara lokal untuk menyegarkan kredensial Anda, kemudian sambungkan kembali
* Konfirmasi Anda masuk ke akun yang sama yang memiliki sesi
* Jika Anda melihat `Remote Control may not be available for this organization`, pemilik belum mengaktifkan sesi cloud untuk organisasi Anda

<h3 id="environment-expired">
  Environment expired
</h3>

Sesi cloud berhenti setelah periode inaktivitas dan lingkungan yang mendasarinya diambil kembali. Dari terminal lokal, ini muncul sebagai `Could not resume session ... its environment has expired. Creating a fresh session instead.` Di web, sesi ditandai kedaluwarsa dalam daftar sesi.

Buka kembali sesi dari [claude.ai/code](https://claude.ai/code) untuk menyediakan lingkungan segar dengan riwayat percakapan Anda dipulihkan.

<h2 id="limitations">
  Batasan
</h2>

Sebelum mengandalkan sesi cloud untuk alur kerja, pertimbangkan batasan ini:

* **Batas laju**: Claude Code di web berbagi batas laju dengan semua penggunaan Claude dan Claude Code lainnya dalam akun Anda. Menjalankan beberapa tugas secara paralel mengonsumsi lebih banyak batas laju secara proporsional. Tidak ada biaya komputasi terpisah untuk VM cloud.
* **Autentikasi repositori**: Anda hanya dapat memindahkan sesi dari web ke lokal saat Anda diautentikasi ke akun yang sama
* **Pembatasan platform**: kloning repositori dan pembuatan pull request memerlukan GitHub. Instans [GitHub Enterprise Server](/docs/id/github-enterprise-server) yang di-host sendiri didukung untuk rencana Team dan Enterprise. Repositori GitLab, Bitbucket, dan non-GitHub lainnya dapat dikirim ke sesi cloud sebagai [bundle lokal](#send-local-repositories-without-github), tetapi sesi tidak dapat mendorong hasil kembali ke remote
* **Daftar putih IP organisasi**: sesi cloud memanggil API Anthropic dari infrastruktur yang dikelola Anthropic, bukan jaringan Anda. Jika organisasi Anda memiliki [IP allowlisting](https://support.claude.com/en/articles/13200993-restrict-access-to-claude-with-ip-allowlisting) yang diaktifkan, setiap sesi cloud gagal dengan kesalahan autentikasi. Hal yang sama berlaku untuk [Code Review](/docs/id/code-review) dan [Routines](/docs/id/routines). Hubungi [dukungan Anthropic](https://support.claude.com/) untuk mengecualikan layanan yang di-host Anthropic dari daftar putih IP organisasi Anda.

<h2 id="related-resources">
  Sumber daya terkait
</h2>

* [Ultraplan](/docs/id/ultraplan): menyusun rencana di sesi cloud dan meninjau di browser Anda
* [Ultrareview](/docs/id/ultrareview): jalankan ulasan kode multi-agen mendalam di sandbox cloud
* [Routines](/docs/id/routines): otomatisasi pekerjaan pada jadwal, melalui panggilan API, atau sebagai respons terhadap acara GitHub
* [Konfigurasi Hooks](/docs/id/hooks): jalankan skrip pada acara siklus hidup sesi
* [Referensi Pengaturan](/docs/id/settings): semua opsi konfigurasi
* [Keamanan](/docs/id/security): jaminan isolasi dan penanganan data
* [Penggunaan Data](/docs/id/data-usage): apa yang Anthropic pertahankan dari sesi cloud
* [Claude Tag](https://claude.com/docs/claude-tag/overview): @Claude yang dikelola organisasi di Slack yang berjalan di lingkungan cloud yang sama
