> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code dengan GitHub Enterprise Server

> Hubungkan Claude Code ke instans GitHub Enterprise Server yang di-host sendiri untuk sesi web, tinjauan kode, dan pasar plugin.

<Note>
  Dukungan GitHub Enterprise Server tersedia untuk paket Team dan Enterprise.
</Note>

Dukungan GitHub Enterprise Server (GHES) memungkinkan organisasi Anda menggunakan Claude Code dengan repositori yang dihosting di instans GitHub yang dikelola sendiri, bukan github.com. Setelah Owner menghubungkan instans GHES Anda, pengembang dapat menjalankan sesi web dan mendapatkan tinjauan kode otomatis tanpa konfigurasi per-repositori apa pun. Pasar plugin yang dihosting di instans Anda juga didukung; persyaratan kredensial bervariasi menurut permukaan, seperti yang dijelaskan dalam [Plugin marketplaces on GHES](#plugin-marketplaces-on-ghes).

Untuk repositori di github.com, lihat [Claude Code di web](/docs/id/claude-code-on-the-web) dan [Code Review](/docs/id/code-review). Untuk menjalankan Claude di infrastruktur CI Anda sendiri, lihat [GitHub Actions](/docs/id/github-actions).

<h2 id="what-works-with-github-enterprise-server">
  Apa yang berfungsi dengan GitHub Enterprise Server
</h2>

Tabel di bawah menunjukkan fitur Claude Code mana yang mendukung GHES dan perbedaan apa pun dari perilaku github.com.

| Fitur              | Dukungan GHES    | Catatan                                                                                                                                        |
| :----------------- | :--------------- | :--------------------------------------------------------------------------------------------------------------------------------------------- |
| Claude Code di web | ✅ Didukung       | Pemilik menghubungkan instans GHES sekali; pengembang menggunakan `claude --cloud` atau [claude.ai/code](https://claude.ai/code) seperti biasa |
| Code Review        | ✅ Didukung       | Tinjauan PR otomatis yang sama seperti github.com                                                                                              |
| Claude Security    | ✅ Didukung       | Tersedia dalam beta publik untuk paket Enterprise di [claude.ai/security](https://claude.ai/security)                                          |
| Sesi Teleport      | ✅ Didukung       | Pindahkan sesi antara web dan terminal dengan `--teleport`                                                                                     |
| Pasar plugin       | ✅ Didukung       | Persyaratan kredensial berbeda menurut permukaan. Lihat [Pasar plugin di GHES](#plugin-marketplaces-on-ghes)                                   |
| Metrik kontribusi  | ✅ Didukung       | Dikirimkan melalui webhook ke [dasbor analitik](/docs/id/analytics)                                                                                 |
| GitHub Actions     | ✅ Didukung       | Memerlukan pengaturan alur kerja manual; `/install-github-app` hanya untuk github.com                                                          |
| Server GitHub MCP  | ❌ Tidak didukung | Server GitHub MCP tidak berfungsi dengan instans GHES                                                                                          |

<h2 id="admin-setup">
  Pengaturan admin
</h2>

Pemilik menghubungkan instans GHES Anda ke Claude Code sekali. Setelah itu, pengembang di organisasi Anda dapat menggunakan repositori GHES tanpa konfigurasi tambahan apa pun. Anda memerlukan peran Pemilik atau Pemilik Utama di organisasi Claude Anda dan izin untuk membuat GitHub Apps di instans GHES Anda.

Pengaturan terpandu menghasilkan manifes GitHub App dan mengarahkan ulang Anda ke instans GHES untuk membuat aplikasi dalam satu klik. Jika lingkungan Anda memblokir alur pengalihan, [pengaturan manual alternatif](#manual-setup) tersedia.

<Steps>
  <Step title="Buka pengaturan admin Claude Code">
    Buka [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code) dan temukan bagian GitHub Enterprise Server.
  </Step>

  <Step title="Mulai pengaturan terpandu">
    Klik **Connect**. Masukkan nama tampilan untuk koneksi dan nama host GHES Anda, misalnya `github.example.com`. Jika instans GHES Anda menggunakan sertifikat yang ditandatangani sendiri atau otoritas sertifikat pribadi, tempel sertifikat CA di bidang opsional.
  </Step>

  <Step title="Buat GitHub App">
    Klik **Continue to GitHub Enterprise**. Browser Anda mengarahkan ulang ke instans GHES Anda dengan manifes aplikasi yang sudah diisi sebelumnya. Tinjau konfigurasi dan klik **Create GitHub App**. GHES mengarahkan ulang Anda kembali ke Claude dengan kredensial aplikasi disimpan secara otomatis.
  </Step>

  <Step title="Pasang aplikasi di repositori Anda">
    Dari halaman GitHub App di instans GHES Anda, pasang aplikasi di repositori atau organisasi yang ingin Anda akses Claude. Anda dapat memulai dengan subset dan menambahkan lebih banyak nanti.
  </Step>

  <Step title="Aktifkan fitur">
    Kembali ke [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code) dan aktifkan [Code Review](/docs/id/code-review#set-up-code-review), Claude Security, dan [metrik kontribusi](/docs/id/analytics#enable-contribution-metrics) untuk repositori GHES Anda menggunakan konfigurasi yang sama seperti github.com.
  </Step>
</Steps>

<h3 id="github-app-permissions">
  Izin GitHub App
</h3>

Manifes mengonfigurasi GitHub App dengan izin dan acara webhook yang Claude butuhkan di seluruh sesi web, Code Review, Claude Security, dan metrik kontribusi:

| Izin             | Akses          | Digunakan untuk                             |
| :--------------- | :------------- | :------------------------------------------ |
| Contents         | Baca dan tulis | Kloning repositori dan push cabang          |
| Pull requests    | Baca dan tulis | Membuat PR dan memposting komentar tinjauan |
| Issues           | Baca dan tulis | Merespons penyebutan masalah                |
| Checks           | Baca dan tulis | Memposting jalankan pemeriksaan Code Review |
| Actions          | Baca           | Membaca status CI untuk perbaikan otomatis  |
| Repository hooks | Baca dan tulis | Menerima webhook untuk metrik kontribusi    |
| Metadata         | Baca           | Diperlukan oleh GitHub untuk semua aplikasi |

Aplikasi berlangganan acara `pull_request`, `issue_comment`, `pull_request_review_comment`, `pull_request_review`, dan `check_run`.

<h3 id="manual-setup">
  Pengaturan manual
</h3>

Jika alur pengalihan terpandu diblokir oleh konfigurasi jaringan Anda, klik **Add manually** alih-alih Connect. Buat GitHub App di instans GHES Anda dengan [izin dan acara di atas](#github-app-permissions), kemudian masukkan kredensial aplikasi dalam formulir: nama host, ID klien OAuth dan rahasia, ID GitHub App, ID klien, rahasia klien, rahasia webhook, dan kunci pribadi.

<h3 id="network-requirements">
  Persyaratan jaringan
</h3>

Instans GHES Anda harus dapat dijangkau dari infrastruktur Anthropic sehingga Claude dapat mengkloning repositori dan memposting komentar tinjauan. Jika instans GHES Anda berada di belakang firewall, daftarkan [alamat IP API Anthropic](https://platform.claude.com/docs/en/api/ip-addresses).

<h2 id="developer-workflow">
  Alur kerja pengembang
</h2>

Setelah admin Anda menghubungkan instans GHES, tidak ada konfigurasi sisi pengembang yang diperlukan. Claude Code mendeteksi nama host GHES Anda secara otomatis dari git remote di direktori kerja Anda.

Kloning repositori dari instans GHES Anda seperti biasa:

```bash theme={null}
git clone git@github.example.com:platform/api-service.git
cd api-service
```

Kemudian mulai sesi web. Claude mendeteksi host GHES dari git remote Anda dan merutekan sesi melalui instans yang dikonfigurasi organisasi Anda:

```bash theme={null}
claude --cloud "Add retry logic to the payment webhook handler"
```

Sesi berjalan di infrastruktur Anthropic, mengkloning repositori Anda dari GHES, dan mendorong perubahan kembali ke cabang. Pantau kemajuan dengan `/tasks` atau di [claude.ai/code](https://claude.ai/code). Lihat [Claude Code di web](/docs/id/claude-code-on-the-web) untuk alur kerja sesi jarak jauh lengkap termasuk tinjauan diff, perbaikan otomatis, dan rutinitas.

<h3 id="teleport-sessions-to-your-terminal">
  Teleport sesi ke terminal Anda
</h3>

Tarik sesi web ke terminal lokal Anda dengan `claude --teleport`. Teleport memverifikasi Anda berada di checkout repositori GHES yang sama sebelum mengambil cabang dan memuat riwayat sesi. Lihat [persyaratan teleport](/docs/id/claude-code-on-the-web#teleport-requirements) untuk detail.

<h2 id="plugin-marketplaces-on-ghes">
  Pasar plugin di GHES
</h2>

Host pasar plugin di instans GHES Anda untuk mendistribusikan alat internal di seluruh organisasi Anda. Struktur pasar identik dengan pasar yang dihosting github.com, tetapi instalasi bekerja berbeda tergantung di mana Anda menambahkan pasar, dan kredensial berbeda di seluruh permukaan:

| Permukaan                                       | Cara instalasi bekerja                                                                                                                                                                                        | Apa yang setiap pengguna butuhkan                                                                                                                                                                                                            |
| :---------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Claude Code CLI dan desktop                     | Claude Code mengkloning repositori pasar menggunakan kredensial git yang ada di mesin                                                                                                                         | Akses Git ke host GHES Anda dari mesin mereka                                                                                                                                                                                                |
| Pengaturan terkelola (`extraKnownMarketplaces`) | Claude Code mendaftarkan entri dan mengkloning repositori menggunakan kredensial git yang ada di mesin                                                                                                        | Akses Git ke host GHES Anda dari mesin mereka                                                                                                                                                                                                |
| Pengaturan plugin organisasi claude.ai          | Pemilik memilih instans GHES sebagai sumber; backend Anthropic mengambil dan menyinkronkan repositori menggunakan GitHub App dari [pengaturan admin](#admin-setup)                                            | Tidak ada per pengguna setelah ditambahkan. Pemilik yang menambahkannya memerlukan akun GitHub Enterprise mereka sendiri yang terhubung sebagai pemeriksaan akses, dan GitHub App harus diinstal di repositori pasar                         |
| Pengaturan pengguna claude.ai                   | Backend Anthropic mengambil repositori menggunakan koneksi GitHub Enterprise pengguna yang mengirimkan                                                                                                        | Akun GitHub Enterprise mereka sendiri yang terhubung ke Claude                                                                                                                                                                               |
| Claude Code di web                              | Sesi cloud mengkloning pasar di dalam sandbox sesi. Sandbox hanya dapat menjangkau instans GHES Anda ketika repositori sesi berada di instans yang sama, dan kredensial git-nya dibatasi pada repositori sesi | Tidak dapat diandalkan untuk pasar yang dihosting GHES: host yang berbeda dari repositori sesi tidak dapat dijangkau, dan bahkan instalasi instans yang sama dapat gagal. Gunakan CLI, pengaturan terkelola, atau claude.ai sebagai gantinya |

<Warning>
  Koneksi GitHub Enterprise di claude.ai adalah per pengguna ketika pasar ditambahkan dari pengaturan pengguna. [Pengaturan admin](#admin-setup) menghubungkan instans GHES Anda ke organisasi Anda, tetapi tidak menghubungkan akun pengguna individual: setiap pengguna yang menambahkan pasar GHES dari pengaturan mereka sendiri harus terlebih dahulu menghubungkan akun GitHub Enterprise mereka sendiri, dan koneksi satu pengguna, termasuk Pemilik, tidak mencakup siapa pun yang lain. Pasar yang ditambahkan oleh Pemilik dalam pengaturan plugin organisasi tidak memberlakukan persyaratan ini pada pengguna, karena pengambilan berkelanjutan menggunakan GitHub App organisasi. Pemilik yang menambahkan pasar masih memerlukan akun GitHub Enterprise mereka sendiri yang terhubung pada waktu penambahan.
</Warning>

<h3 id="add-a-ghes-marketplace">
  Tambahkan pasar GHES
</h3>

Shorthand `owner/repo` selalu diselesaikan ke github.com. Untuk pasar yang dihosting GHES, gunakan URL git lengkap. URL HTTPS direkomendasikan:

```bash theme={null}
/plugin marketplace add https://github.example.com/platform/claude-plugins.git
```

URL SSH berfungsi jika mesin sudah mempercayai host GHES Anda:

```bash theme={null}
/plugin marketplace add git@github.example.com:platform/claude-plugins.git
```

Claude Code menjalankan git secara non-interaktif dan menolak koneksi SSH ke host yang tidak ada dalam file `known_hosts` mesin. URL HTTPS dengan pembantu kredensial git menghindari persyaratan `known_hosts`.

Lihat [Buat dan distribusikan pasar plugin](/docs/id/plugin-marketplaces) untuk panduan lengkap membangun pasar.

<h3 id="pre-register-ghes-marketplaces-with-managed-settings">
  Daftarkan pasar GHES sebelumnya dengan pengaturan terkelola
</h3>

Pengaturan `extraKnownMarketplaces` mendaftarkan pasar sebelumnya sehingga pengembang mendapatkannya tanpa pengaturan manual. Ini bekerja dari [file pengaturan apa pun](/docs/id/settings#extraknownmarketplaces), termasuk `.claude/settings.json` repositori; pengaturan terkelola mengirimkannya di seluruh organisasi:

```json theme={null}
{
  "extraKnownMarketplaces": {
    "internal-tools": {
      "source": {
        "source": "git",
        "url": "https://github.example.com/platform/claude-plugins.git"
      }
    }
  }
}
```

Claude Code menginstal pasar ini secara lokal: mendaftarkan setiap entri dan mengkloning repositori dengan kredensial git yang ada di mesin. Jalur ini tidak melalui claude.ai, jadi koneksi GitHub Enterprise per pengguna tidak diperlukan. Untuk peluncuran yang berhasil:

* **Gunakan URL git lengkap.** Shorthand `owner/repo` selalu diselesaikan ke github.com dan tidak dapat mereferensikan host GHES.
* **Lebih suka URL HTTPS.** Kloning SSH gagal pada mesin yang tidak sudah mempercayai kunci host GHES Anda. URL HTTPS dengan pembantu kredensial git standar organisasi Anda bekerja pada mesin apa pun dengan kredensial yang dikonfigurasi.
* **Konfirmasi setiap mesin dapat mengkloning dari host GHES Anda.** Jika mesin tidak memiliki kredensial, pasar didaftarkan tetapi tidak pernah diinstal, dan plugin-nya melaporkan sebagai tidak ditemukan alih-alih meminta kredensial.
* **Konfirmasi pengaturan mencapai setiap mesin.** File pengaturan terkelola hanya berlaku pada mesin yang diterapkan, misalnya melalui sistem manajemen perangkat Anda. Lihat [pengaturan terkelola](/docs/id/settings#settings-files) untuk lokasi file.

<h3 id="allowlist-ghes-marketplaces-in-managed-settings">
  Daftarkan pasar GHES dalam pengaturan terkelola
</h3>

Jika organisasi Anda menggunakan [pengaturan terkelola](/docs/id/settings) untuk membatasi pasar mana yang dapat ditambahkan pengembang, gunakan tipe sumber `hostPattern` untuk memungkinkan semua pasar dari instans GHES Anda tanpa menghitung setiap repositori:

```json theme={null}
{
  "strictKnownMarketplaces": [
    {
      "source": "hostPattern",
      "hostPattern": "^github\\.example\\.com$"
    }
  ]
}
```

Lihat referensi pengaturan [strictKnownMarketplaces](/docs/id/settings#strictknownmarketplaces) dan [extraKnownMarketplaces](/docs/id/settings#extraknownmarketplaces) untuk skema lengkap.

<h2 id="limitations">
  Keterbatasan
</h2>

Beberapa fitur berperilaku berbeda di GHES daripada di github.com. [Tabel fitur](#what-works-with-github-enterprise-server) merangkum dukungan; bagian ini mencakup solusi.

* **Perintah `/install-github-app`**: ikuti alur [pengaturan admin](#admin-setup) di claude.ai sebagai gantinya. Jika Anda juga menginginkan alur kerja GitHub Actions di GHES, sesuaikan [alur kerja contoh](https://github.com/anthropics/claude-code-action/blob/main/examples/claude.yml) secara manual.
* **Server GitHub MCP**: gunakan CLI `gh` yang dikonfigurasi untuk host GHES Anda sebagai gantinya. Jalankan `gh auth login --hostname github.example.com` untuk autentikasi, kemudian Claude dapat menggunakan perintah `gh` dalam sesi.

<h2 id="troubleshooting">
  Troubleshooting
</h2>

<h3 id="web-session-fails-to-clone-repository">
  Sesi web gagal mengkloning repositori
</h3>

Jika `claude --cloud` gagal dengan kesalahan kloning, verifikasi bahwa seorang Owner telah menyelesaikan pengaturan untuk instans GHES Anda dan bahwa GitHub App dipasang di repositori tempat Anda bekerja. Tanyakan kepada Owner yang menghubungkan instans untuk mengonfirmasi bahwa nama host yang terdaftar dalam pengaturan Claude cocok dengan nama host di git remote Anda.

<h3 id="marketplace-add-fails-with-a-policy-error">
  Penambahan pasar gagal dengan kesalahan kebijakan
</h3>

Jika `/plugin marketplace add` diblokir untuk URL GHES Anda, organisasi Anda telah membatasi sumber pasar. Minta admin Anda untuk menambahkan entri `hostPattern` untuk nama host GHES Anda dalam [pengaturan terkelola](#allowlist-ghes-marketplaces-in-managed-settings).

<h3 id="marketplace-add-on-claude-ai-fails-with-a-github-access-error">
  Penambahan pasar di claude.ai gagal dengan kesalahan akses GitHub
</h3>

Jika menambahkan pasar GHES dari pengaturan pengguna Anda gagal dengan kesalahan generik seperti "Marketplace couldn't be added", periksa koneksi GitHub Enterprise Anda terlebih dahulu. Ini adalah apa yang muncul ketika akun GitHub Enterprise Anda sendiri tidak terhubung ke Claude, bahkan jika instans GHES organisasi Anda dikonfigurasi dan pengguna lain terhubung. Dialog tidak menunjuk ke alur koneksi GitHub Enterprise, dan opsi "Connect to GitHub" di tab Browse masuk ke github.com, yang tidak memberikan akses ke repositori GHES.

Untuk menghubungkan akun GitHub Enterprise Anda: pemilih repositori di [claude.ai/code](https://claude.ai/code) menawarkan opsi koneksi untuk setiap instans GHES yang dikonfigurasi, dan Owner juga dapat terhubung dari bagian GitHub Enterprise dari [pengaturan admin Claude Code](https://claude.ai/admin-settings/claude-code). Kemudian tambahkan pasar lagi. Alternatifnya, minta Owner untuk menambahkan pasar dalam pengaturan plugin organisasi, yang menghilangkan persyaratan koneksi per pengguna.

Di permukaan claude.ai lainnya, kesalahan "Repository not found. If it's private, GitHub access is required" di pasar GHES biasanya menunjukkan koneksi yang hilang yang sama. Hubungkan akun GitHub Enterprise Anda melalui salah satu jalur di atas, kemudian coba lagi.

<h3 id="ghes-instance-not-reachable">
  Instans GHES tidak dapat dijangkau
</h3>

Jika tinjauan atau sesi web habis waktu, instans GHES Anda mungkin tidak dapat dijangkau dari infrastruktur Anthropic. Konfirmasi firewall Anda memungkinkan koneksi masuk dari [alamat IP API Anthropic](https://platform.claude.com/docs/en/api/ip-addresses).

<h2 id="related-resources">
  Sumber daya terkait
</h2>

Halaman-halaman ini mencakup fitur yang direferensikan di seluruh panduan ini secara lebih mendalam:

* [Claude Code di web](/docs/id/claude-code-on-the-web): jalankan sesi Claude Code di infrastruktur cloud
* [Code Review](/docs/id/code-review): tinjauan PR otomatis
* [Pasar plugin](/docs/id/plugin-marketplaces): bangun dan distribusikan katalog plugin
* [Analytics](/docs/id/analytics): lacak penggunaan dan metrik kontribusi
* [Pengaturan terkelola](/docs/id/settings): konfigurasi kebijakan di seluruh organisasi
* [Konfigurasi jaringan](/docs/id/network-config): persyaratan firewall dan daftar izin IP
