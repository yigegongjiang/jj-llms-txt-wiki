> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Konfigurasi lingkungan cloud

> Konfigurasi lingkungan cloud untuk sesi Claude Code cloud: tingkat akses jaringan, variabel lingkungan, skrip setup, dan caching lingkungan.

<Note>
  Lingkungan cloud berlaku untuk [sesi cloud](/docs/id/claude-code-on-the-web), yang tersedia pada paket Pro, Max, dan Team, serta untuk pengguna Enterprise dengan [kursi premium atau kursi Chat + Claude Code](https://support.claude.com/en/articles/11845131-use-claude-code-with-your-team-or-enterprise-plan).
</Note>

Setiap [sesi cloud](/docs/id/claude-code-on-the-web) berjalan di lingkungan cloud. Anda dapat mengonfigurasi lingkungan untuk mengizinkan atau menolak [akses jaringan](#access-levels), [menetapkan variabel lingkungan](#set-environment-variables) untuk sesi, pada paket Pro dan Max menyimpan [kredensial API](#add-api-credentials) yang digunakan sesi tanpa melihatnya, dan menjalankan [skrip setup](#setup-scripts) sebelum Claude mulai bekerja.

Lingkungan yang sama berlaku di mana pun Anda memulai sesi cloud: [aplikasi Desktop](/docs/id/desktop), [aplikasi mobile Claude](/docs/id/mobile), browser Anda di [claude.ai/code](https://claude.ai/code), terminal dengan [`claude --cloud`](/docs/id/claude-code-on-the-web#from-terminal-to-cloud), [routines](/docs/id/routines), dan [Claude Tag](https://claude.com/docs/claude-tag/overview). Setiap permukaan ini juga dapat mengarahkan ke [lingkungan yang di-host sendiri](/docs/id/self-hosted-environments). [Ketersediaan dan batasan](/docs/id/self-hosted-environments#availability-and-limitations) mencakup apa yang Claude belum dapat gunakan ketika sesi Claude Tag berjalan di dalamnya.

<Info>
  Sesi [Remote Control](/docs/id/remote-control) menghubungkan antarmuka web dan mobile ke sesi di mesin Anda sendiri, yang menggunakan jaringan dan file mesin Anda, bukan lingkungan cloud. Sesi saluran Claude Tag menggunakan lingkungan tingkat organisasi saja, baik [lingkungan bersama](#organization-shared-environments) atau [lingkungan yang di-host sendiri](/docs/id/self-hosted-environments).
</Info>

<h2 id="the-default-environment">
  Lingkungan Default
</h2>

Jika Anda belum memiliki lingkungan, onboarding menyiapkan lingkungan **Default**. Caranya tergantung di mana Anda melakukan onboarding:

* **Alur CLI seperti `/web-setup`**: membuat **Default** untuk Anda
* **Onboarding web pada Pro dan Max**: membuat **Default** untuk Anda
* **Onboarding web pada Team dan Enterprise**: menampilkan formulir **Buat lingkungan cloud pertama Anda** kecuali Owner telah mengaktifkan [Pengaturan web cepat](/docs/id/claude-code-on-the-web#github-authentication-options); pertahankan default formulir dan klik **Buat & selesai** untuk mendapatkan lingkungan **Default** yang sama

**Default** tidak membawa konfigurasi apa pun:

* [Akses jaringan **Trusted**](#access-levels): sesi menjangkau registri paket dan [domain yang diizinkan lainnya](#default-allowed-domains), dan tidak ada yang lain melalui jaringan sesi.
* Tidak ada konfigurasi lain: **Default** tidak mendefinisikan variabel lingkungan atau skrip setup, jadi sesi dimulai dengan hanya [alat yang sudah diinstal sebelumnya](#installed-tools).

Dengan hanya **Default** yang tersedia, setiap sesi berjalan di dalamnya. Ketika Anda memiliki lebih dari satu lingkungan, sesi memilih satu per permukaan:

* Di aplikasi Desktop, aplikasi mobile, dan di claude.ai/code, sesi yang Anda mulai sendiri menggunakan lingkungan yang ditampilkan di [pemilih](#configure-your-environment). [Default organisasi](#organization-shared-environments) yang ditetapkan Owner mengisi pilihan ketika Anda belum memilih satu. Thread dalam [proyek](/docs/id/claude-projects#project-settings-reference) menggunakan lingkungan yang ditetapkan dalam pengaturan proyek sebagai gantinya.
* Dari CLI, Claude Code menggunakan pilihan [`/remote-env`](#select-an-environment-from-the-cli) Anda, atau kembali ke lingkungan yang di-host Anthropic ketika daftar Anda memiliki satu, dan sebaliknya ke lingkungan pertama dalam daftar Anda yang bukan lingkungan bridge, entri [Remote Control](/docs/id/remote-control) mendaftarkan untuk mewakili mesin Anda sendiri daripada lingkungan cloud. Untuk [lingkungan yang di-host sendiri](/docs/id/self-hosted-environments), melewatkan `--environment <environment-id>` dengan ID `ccpool_` miliknya [ketika Anda mengirim sesi](/docs/id/self-hosted-environments-testing#run-the-test-loop) menimpa pilihan `/remote-env` dan fallback untuk invokasi itu. Claude Code menolak ID `env_` yang di-host Anthropic yang dilewatkan ke flag, jadi gunakan `/remote-env` untuk menargetkan yang tersebut. Flag memerlukan Claude Code v2.1.224 atau lebih baru.

Konfigurasi lingkungan ketika default tidak cukup: ketika Claude perlu menjangkau domain di luar [daftar allowlist default](#default-allowed-domains), memerlukan variabel lingkungan yang ditetapkan untuk sesinya, atau memerlukan dependensi yang diinstal sebelum mulai bekerja.

<h2 id="configure-your-environment">
  Konfigurasi lingkungan Anda
</h2>

Buat, edit, dan arsipkan lingkungan dari pemilih lingkungan, yang Anda jangkau di [claude.ai/code](https://claude.ai/code) setelah [onboarding web](/docs/id/web-quickstart), atau dari kotak prompt di [aplikasi Desktop](/docs/id/desktop#cloud-sessions). Lingkungan yang Anda buat bersifat pribadi untuk akun Anda; [lingkungan bersama](#organization-shared-environments) yang dibuat oleh Owner muncul di pemilih yang sama. Lihat [Alat yang diinstal](#installed-tools) untuk melihat apa yang tersedia tanpa konfigurasi apa pun.

<Steps>
  <Step title="Buka pemilih lingkungan">
    Di [claude.ai/code](https://claude.ai/code), pilih ikon cloud yang menampilkan nama lingkungan saat ini, di baris di atas kotak pesan. Tidak ada halaman pengaturan atau URL langsung untuk pemilih.

    <Frame>
      <img src="https://mintcdn.com/claude-code/ZFId6l95856c5LSw/images/cloud-environment-selector.png?fit=max&auto=format&n=ZFId6l95856c5LSw&q=85&s=cc2813a5664519eaf5a89d793ce5af26" alt="Pemilih lingkungan terbuka di atas kotak pesan di claude.ai/code. Tombol cloud yang menampilkan nama lingkungan Default duduk di baris di atas kotak pesan. Menu terbuka mencantumkan baris Local dengan label Download dan Desktop only, bagian Cloud di mana lingkungan Default dipilih dengan tanda centang dan menampilkan ikon roda gigi pengaturan saat hover, opsi Add cloud environment, dan bagian Remote Control dengan instruksi setup." width="1672" height="682" data-path="images/cloud-environment-selector.png" />
    </Frame>
  </Step>

  <Step title="Tambahkan atau edit lingkungan">
    Pilih **Add cloud environment**, atau arahkan ke lingkungan yang ada dan pilih ikon pengaturan yang muncul di sebelah kanan. Dialog mencakup nama, tingkat akses jaringan, variabel lingkungan, dan skrip setup. Ketika Anda mengedit lingkungan cloud yang ada pada paket Pro atau Max, dialog juga mencakup [kredensial API](#add-api-credentials).

    <Frame>
      <img src="https://mintcdn.com/claude-code/ZFId6l95856c5LSw/images/cloud-environment-dialog.png?fit=max&auto=format&n=ZFId6l95856c5LSw&q=85&s=30d4478b31d1f879f7ee287ddab32505" alt="Dialog New cloud environment. Bidang Name dengan placeholder Default, pemilih Network access yang diatur ke Trusted dengan tautan ke kebijakan jaringan dan tingkat akses, kotak Environment variables yang menampilkan teks placeholder format .env dengan catatan bahwa nilai terlihat oleh siapa pun yang menggunakan lingkungan, kotak Setup script yang dijelaskan sebagai skrip Bash yang berjalan ketika sesi baru dimulai sebelum Claude Code diluncurkan, dan tombol Cancel dan Create environment." width="874" height="1372" data-path="images/cloud-environment-dialog.png" />
    </Frame>
  </Step>
</Steps>

<h3 id="set-environment-variables">
  Tetapkan variabel lingkungan
</h3>

Variabel lingkungan menggunakan format `.env`, satu pasangan `KEY=value` per baris. Nilai biasa tidak memerlukan tanda kutip, dan jika Anda mengutip nilai dengan pasangan yang cocok, tanda kutip tidak menjadi bagian dari nilai. Kutip nilai yang mencakup beberapa baris atau berisi `#`: dalam nilai yang tidak dikutip, `#` memulai komentar dan sisa baris dijatuhkan.

Contoh berikut mendefinisikan tiga variabel.

```text theme={null}
NODE_ENV=development
LOG_LEVEL=debug
DATABASE_URL=postgres://localhost:5432/myapp
```

Setiap sesi menyalin nilai lingkungan sekali, saat startup, ke dalam variabel lingkungan biasa yang dapat dibaca oleh perintah apa pun yang dijalankan Claude. Karena sesi yang berjalan tidak membaca ulang konfigurasi, mengedit atau menambahkan variabel mempengaruhi sesi yang Anda mulai setelahnya; sesi yang sudah berjalan menyimpan nilai yang mereka mulai.

Sesi cloud juga menetapkan beberapa variabel sendiri ketika memulai. Untuk [`CLAUDE_AUTOCOMPACT_PCT_OVERRIDE`](/docs/id/claude-code-on-the-web#manage-context), nilai yang ditetapkan sesi menimpa nilai yang Anda tambahkan di sini, jadi menambahkan kunci itu di sini tidak berpengaruh.

Siapa pun yang menggunakan lingkungan dapat membaca nilai. Pada paket Pro dan Max, gunakan [kredensial API](#add-api-credentials) sebagai gantinya untuk kunci yang dapat dilampirkan proxy agen. [Permintaan yang tidak pernah mendapatkan kredensial](#requests-that-never-get-the-credential) tercantum di sana.

<h3 id="add-api-credentials">
  Tambahkan kredensial API
</h3>

Kredensial API adalah kunci API atau token yang Anda simpan di lingkungan cloud sehingga Claude dapat memanggil API itu dari sesi apa pun di lingkungan tanpa melihat kunci. Proxy agen Anthropic menambahkan kunci ke permintaan untuk host yang Anda cantumkan, setelah setiap permintaan meninggalkan VM sesi. Kunci tidak pernah mencapai Claude, perintah yang dijalankannya, atau variabel lingkungan sesi.

Kredensial API tersedia pada paket Pro dan Max. Mereka tidak tersedia pada paket Team atau Enterprise namun, jadi bagian **API credentials** tidak muncul di dialog lingkungan pada paket tersebut.

<h4 id="requirements">
  Persyaratan
</h4>

Dua dari ini memutuskan apakah Anda dapat menambahkan kredensial, dan dua memutuskan apakah proxy agen dapat menggunakannya setelah ditambahkan:

* **Peran**: peran admin organisasi di organisasi claude.ai Anda
  * Pada Team dan Enterprise, Owner memegangnya dan Admin tidak
  * Pada Pro dan Max, Anda memegangnya di organisasi Anda sendiri
  * Tanpa itu, Anda melihat catatan alih-alih daftar kredensial, bahkan di lingkungan Anda sendiri. Minta Owner menambahkan kredensial ke lingkungan bersama dan jalankan sesi Anda di sana
* **Jenis lingkungan**: lingkungan cloud yang di-host Anthropic yang sudah ada. [Lingkungan yang di-host sendiri](/docs/id/self-hosted-environments) tidak memiliki kredensial API
* **Jangkauan API**: API menerima koneksi dari internet, karena permintaan meninggalkan dari jaringan Anthropic
* **Kunci enkripsi**: jika organisasi Anda menggunakan kunci enkripsi yang dikelola pelanggan, Anda tidak dapat menyimpan kredensial

<h4 id="add-a-credential">
  Tambahkan kredensial
</h4>

Anda menambahkan kredensial satu per satu dari editor lingkungan yang sudah ada. Dialog untuk lingkungan baru tidak menawarkannya. Tidak ada edit juga. Untuk mengubah host atau nilai kredensial, hapus dan tambahkan lagi.

<Steps>
  <Step title="Buka kredensial API lingkungan">
    [Buka lingkungan untuk diedit](#configure-your-environment) di [claude.ai/code](https://claude.ai/code). Di dialog **Update cloud environment**, temukan **API credentials** di bawah **Environment variables**. Anda melihat kredensial yang sudah ada di lingkungan, masing-masing dengan host yang berlaku.
  </Step>

  <Step title="Tambahkan kredensial">
    Pilih **Add credential** dan isi formulir. Pertahankan **Credential type** default, **Bearer**, untuk kunci API yang berjalan di header permintaan, dan isi bidang ini:

    * **Name**: label untuk kredensial, seperti `Internal billing API`
    * **Allowed websites**: host API, seperti `api.example.com`. `*.` di depan cocok dengan setiap subdomain
    * **Custom headers**: satu baris untuk header yang membawa kunci. Baris dimulai dengan `Authorization` sebagai **Name** header dan `Bearer` sebagai **Prefix**; tempel kunci itu sendiri sebagai **Value**. Untuk header seperti `X-Api-Key` yang mengambil nilai telanjang, ubah nama dan kosongkan prefix

    Untuk API yang mengautentikasi dengan cara lain, pilih **Credential type** yang berbeda. Daftar adalah yang sama yang ditawarkan [Claude Tag](https://claude.com/docs/claude-tag/overview), integrasi Slack untuk paket Team dan Enterprise, untuk [koneksi](https://claude.com/docs/claude-tag/admins/add-connections).
  </Step>

  <Step title="Simpan kredensial">
    Pilih **Connect**. Kredensial muncul dalam daftar dengan hostnya, disimpan tanpa tombol **Save changes** dialog. Anda tidak dapat melihat nilai lagi setelah menyimpan.
  </Step>
</Steps>

Untuk mengonfirmasi kredensial berfungsi, mulai sesi di lingkungan dan minta Claude memanggil API, misalnya dengan `curl`. API menjawab seolah-olah kunci ada dalam permintaan, dan kunci tidak muncul di variabel lingkungan sesi atau di file apa pun. Jika daftar menandai kredensial **Not sent** sebagai gantinya, catatan di bawahnya mengatakan mengapa dan apa yang harus dilakukan. Dua kredensial yang hostnya tumpang tindih tanpa cocok persis tidak mendapat penanda, dan proxy agen mengirim hanya satu dari mereka.

<h4 id="which-requests-get-the-credential">
  Permintaan mana yang mendapatkan kredensial
</h4>

Proxy agen melampirkan kredensial ke permintaan ketika host permintaan cocok dengan salah satu yang Anda cantumkan pada kredensial itu. Sesi dapat menjangkau host tersebut bahkan ketika [tingkat akses jaringan](#access-levels) lingkungan tidak akan mengizinkannya, kecuali [host yang tidak pernah mendapatkan kredensial](#requests-that-never-get-the-credential). Kredensial berlaku di setiap sesi yang berjalan di lingkungan, siapa pun yang memulainya, sampai Anda menghapusnya.

<h4 id="requests-that-never-get-the-credential">
  Permintaan yang tidak pernah mendapatkan kredensial
</h4>

Proxy agen tidak pernah melampirkan kredensial yang Anda tambahkan ke permintaan ini:

* **GitHub**: [proxy GitHub](#github-proxy) mengautentikasi permintaan ke GitHub sebagai gantinya, jadi Anda tidak memerlukan kredensial API untuk itu
* **API Anthropic dan registri paket publik**: `api.anthropic.com`, `registry.npmjs.org`, `jsr.io`, `npm.jsr.io`, `pypi.org`, `files.pythonhosted.org`, `index.crates.io`, dan `proxy.golang.org`
* **Permintaan skrip setup**: Claude Code terhubung ke proxy agen ketika diluncurkan, setelah [skrip setup](#setup-scripts) telah berjalan

<h3 id="select-an-environment-from-the-cli">
  Pilih lingkungan dari CLI
</h3>

Jalankan `/remote-env` di terminal Anda untuk memilih lingkungan default untuk sesi cloud yang Anda buat dari CLI, seperti [`claude --cloud`](/docs/id/claude-code-on-the-web#from-terminal-to-cloud). Perintah membuka pemilih lingkungan yang ada dan menyimpan pilihan Anda ke kunci `remote.defaultEnvironmentId` di [pengaturan pengguna](/docs/id/settings#where-settings-live) Anda, sehingga berlaku di setiap proyek di mesin Anda sampai Anda mengubahnya, kecuali kunci yang sama ditetapkan di [lapisan pengaturan](/docs/id/settings#settings-precedence) dengan prioritas lebih tinggi, seperti pengaturan proyek repo.

ID [lingkungan yang di-host sendiri](/docs/id/self-hosted-environments), yang memiliki bentuk `ccpool_...`, mengikuti aturan sumber yang lebih ketat. Lihat [`remote.defaultEnvironmentId`](/docs/id/settings-reference#remote-defaultenvironmentid) untuk lapisan pengaturan yang dihormati Claude Code darinya.

`/remote-env` hanya menetapkan default: tidak memulai sesi, dan tidak dapat menambah atau mengedit lingkungan. Kelola mereka dari [pemilih lingkungan](#configure-your-environment).

<h3 id="archive-an-environment">
  Arsipkan lingkungan
</h3>

Untuk mengarsipkan salah satu lingkungan Anda sendiri, buka untuk diedit dan pilih **Archive**. Owner mengarsipkan [lingkungan bersama](#organization-shared-environments) dari halaman **Cloud environments** di pengaturan admin. Anda tidak dapat menghapus lingkungan, hanya mengarsipkannya.

Pengarsipan mempengaruhi sesi baru, bukan yang sedang berjalan:

* Sesi yang sudah berjalan di lingkungan terus bekerja.
* Lingkungan menghilang dari pemilih dan dari `/remote-env`, jadi Anda tidak dapat memilihnya untuk sesi baru.
* Kredensial API di lingkungan tetap terlampir di sesinya yang berjalan. Hapus yang tidak lagi Anda inginkan sebelum mengarsipkan.
* Tidak ada sesi baru yang dapat dimulai di lingkungan yang diarsipkan, di permukaan apa pun. Jika lingkungan adalah [default CLI](#select-an-environment-from-the-cli) yang disimpan, Claude Code memulai sesi cloud CLI di lingkungan yang di-host Anthropic ketika daftar Anda memiliki satu, dan sebaliknya di lingkungan pertama dalam daftar Anda yang bukan [lingkungan bridge Remote Control](#the-default-environment). Apa pun yang dikonfigurasi dengan lingkungan secara eksplisit, seperti [routine](/docs/id/routines#environments-and-network-access), tidak dapat memulai sesi baru di dalamnya. Arahkan ke lingkungan lain.

<h3 id="organization-shared-environments">
  Lingkungan bersama organisasi
</h3>

Pada paket Team dan Enterprise, Owner dapat membuat lingkungan cloud yang dibagikan dengan setiap anggota organisasi. Peran yang sama mengelola segalanya di halaman admin **Cloud environments**, termasuk [lingkungan yang di-host sendiri](/docs/id/self-hosted-environments); peran Admin tidak dapat membuka halaman. Daftar lengkap peran yang dapat membukanya adalah yang untuk [mengelola pengaturan yang dikelola server](/docs/id/server-managed-settings#access-control).

Lingkungan bersama muncul di [pemilih lingkungan](#configure-your-environment) setiap anggota di bawah judul **Organization**, setelah lingkungan pribadi anggota di bawah **Personal**, sehingga tim dapat menstandarkan satu konfigurasi alih-alih setiap anggota membuat ulangnya. Memilih ikon pengaturan lingkungan bersama di sana membuka ringkasan konfigurasinya yang hanya dapat dibaca untuk setiap anggota, termasuk Owner.

Owner membuat lingkungan tersedia untuk organisasi dengan salah satu dari dua cara:

* **Buat lingkungan bersama**: gunakan halaman **Cloud environments** di [pengaturan admin](https://claude.ai/admin-settings), yang juga merupakan tempat Owner mengedit dan mengarsipkan lingkungan bersama. Setiap lingkungan memiliki nama, [tingkat akses jaringan](#access-levels), [variabel lingkungan](#set-environment-variables) dalam format `.env`, dan [skrip setup](#setup-scripts).
* **Bagikan lingkungan pribadi**: buka salah satu lingkungan Anda sendiri untuk diedit di pemilih lingkungan, kemudian bagikan dari baris **Who can use it**. Lingkungan menyimpan ID-nya, jadi sesi dan routine yang sudah menggunakannya tidak terpengaruh, dan setiap anggota kemudian dapat melihatnya dan memulai sesi di dalamnya.

Owner memilih [lingkungan default](#the-default-environment) organisasi secara terpisah, di [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code).

Sesi setiap anggota di lingkungan bersama membaca variabelnya, jadi jangan sertakan rahasia di dalamnya. [Kredensial API](#add-api-credentials), yang memberikan sesi kunci yang tidak dapat dibacanya, tidak tersedia pada paket Team atau Enterprise namun.

<h3 id="set-the-environment-a-claude-tag-channel-uses">
  Tetapkan lingkungan yang digunakan saluran Claude Tag
</h3>

Di saluran [Claude Tag](https://claude.com/docs/claude-tag/overview), Claude bekerja sebagai identitas bersama organisasi Anda, bukan sebagai anggota apa pun, jadi sesi saluran menggunakan lingkungan tingkat organisasi saja, baik lingkungan bersama atau [lingkungan yang di-host sendiri](/docs/id/self-hosted-environments). Untuk memberikan saluran toolchain yang bukan [sudah diinstal sebelumnya](#installed-tools), seperti .NET, Owner dapat membuat [lingkungan bersama](#organization-shared-environments) dari halaman admin **Cloud environments** dengan [skrip setup](#setup-scripts) yang menginstalnya. Arahkan saluran ke lingkungan dengan salah satu dari dua cara:

* Tetapkan lingkungan bersama atau yang di-host sendiri sebagai [lingkungan default](#the-default-environment) organisasi di [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code).
* [Sematkan satu ke saluran](https://claude.com/docs/claude-tag/admins/troubleshooting#channel-sessions-use-the-wrong-environment-or-can%E2%80%99t-find-one) di pengaturan admin Claude Tag.

<h2 id="network-access">
  Akses jaringan
</h2>

Setiap lingkungan menetapkan satu tingkat akses jaringan, yang mengontrol koneksi keluar yang dapat dibuat sesinya. Tingkat default, **Trusted**, memungkinkan registri paket dan [domain yang diizinkan lainnya](#default-allowed-domains); **Custom** mengambil daftar domain Anda sendiri.

Untuk mengubah akses jaringan lingkungan, [buka untuk diedit](#configure-your-environment) dan gunakan pemilih **Network access** di dialog. Lingkungan [bersama](#organization-shared-environments) membuka read-only di sana, jadi Owner mengubah akses jaringannya dari halaman **Cloud environments** di [pengaturan admin](https://claude.ai/admin-settings) sebagai gantinya. Ikon cloud yang membuka pemilih muncul di permukaan aplikasi yang tercantum di bawah [Lingkungan Default](#the-default-environment) dan di [editor routine](/docs/id/routines#environments-and-network-access); lingkungan pribadi tidak memiliki halaman terpisah di pengaturan akun claude.ai Anda.

<Note>
  Konektor MCP yang Anda aktifkan pada sesi atau routine bekerja tanpa menambahkan host mereka ke **Allowed domains**, karena lalu lintas konektor berjalan melalui server Anthropic daripada jaringan sesi. Ini bergantung pada saluran yang terikat Anthropic yang sama yang dicatat di bawah [Security and isolation](/docs/id/claude-code-on-the-web#security-and-isolation). Matikan konektor apa pun yang tidak Anda butuhkan untuk membatasi alat mana yang dapat dijangkau Claude.
</Note>

<h3 id="access-levels">
  Tingkat akses
</h3>

Bidang **Network access** di [dialog lingkungan](#configure-your-environment) mengambil salah satu dari empat tingkat:

| Tingkat     | Koneksi keluar                                                                            |
| :---------- | :---------------------------------------------------------------------------------------- |
| **None**    | Tidak ada akses jaringan keluar melalui jaringan sesi                                     |
| **Trusted** | [Domain yang diizinkan](#default-allowed-domains) saja: registri paket, GitHub, cloud SDK |
| **Full**    | Domain apa pun                                                                            |
| **Custom**  | Allowlist Anda sendiri, secara opsional termasuk default                                  |

Apa pun tingkat yang Anda pilih, sesi masih dapat menjangkau ini, karena masing-masing mengambil jalur yang tidak melewati allowlist jaringan sesi:

* GitHub, melalui [proxy terpisahnya](#github-proxy)
* [Konektor MCP](#network-access) yang Anda aktifkan, yang lalu lintasnya berjalan melalui server Anthropic
* Host yang Anda cantumkan di [kredensial API](#add-api-credentials) lingkungan, kecuali [host yang tidak pernah mendapatkan kredensial](#requests-that-never-get-the-credential)
* API Anthropic, untuk permintaan Claude Code sendiri, bahkan di **None**, seperti yang dicatat di bawah [Security and isolation](/docs/id/claude-code-on-the-web#security-and-isolation)

<h3 id="allow-specific-domains">
  Izinkan domain tertentu
</h3>

Untuk mengizinkan domain yang tidak ada dalam daftar Trusted, pilih **Custom** di pengaturan akses jaringan lingkungan, kemudian cantumkan satu domain per baris di bidang **Allowed domains**. Contoh ini memungkinkan tiga host yang mungkin dibutuhkan proyek internal.

```text theme={null}
api.example.com
*.internal.example.com
registry.example.com
```

Sesi di lingkungan ini sekarang dapat menjangkau `api.example.com`, subdomain apa pun dari `internal.example.com`, dan `registry.example.com`, dan tidak ada domain lain melalui jaringan sesi. [Lalu lintas GitHub](#github-proxy), [lalu lintas konektor MCP](#network-access), dan permintaan ke host [kredensial API](#add-api-credentials) lingkungan, selain [host yang tidak pernah mendapatkan kredensial](#requests-that-never-get-the-credential), tidak melewati allowlist ini. `*.` di depan cocok dengan setiap subdomain. Untuk menyimpan [domain Trusted](#default-allowed-domains) juga, centang **Also include default list of common package managers**; biarkan tidak dicentang untuk hanya mengizinkan apa yang Anda cantumkan.

Jika organisasi Anda menggunakan [artifacts](/docs/id/artifacts#availability), Anda tidak memerlukan `*.frame.claudeusercontent.com` dalam daftar untuk sesi membacanya. Ketika daftar meninggalkan host itu, Claude Code membaca konten artifact melalui koneksi sesi ke Anthropic sebagai gantinya. Pertahankan host dalam allowlist dalam dua situasi:

* **Sesi di lingkungan ini membuka artifact publik organisasi lain**: Claude Code mengambil yang dari host secara langsung, jadi tambahkan ke daftar ini.
* **Anda mengonfigurasi CLI lokal atau runner yang di-host sendiri**: pertahankan host dalam allowlist itu. Lihat [persyaratan akses jaringan](/docs/id/network-config#network-access-requirements) dan [persyaratan jaringan](/docs/id/self-hosted-environments-deploy#network-requirements) yang di-host sendiri.

Setiap lingkungan memiliki daftar domain yang diizinkan sendiri; tidak ada allowlist tingkat organisasi yang dapat didorong admin ke lingkungan setiap anggota. [Pengaturan yang dikelola server](/docs/id/server-managed-settings) masih berlaku di dalam sesi cloud, tetapi tidak ada yang menambahkan domain ke allowlist jaringan lingkungan. Untuk memberikan tim satu daftar standar, Owner dapat membuat [lingkungan bersama organisasi](#organization-shared-environments) dengan akses jaringan **Custom** dan daftar itu.

<h3 id="github-proxy">
  Proxy GitHub
</h3>

Di lingkungan yang di-host Anthropic, semua operasi GitHub melewati proxy khusus yang menjaga kredensial GitHub nyata Anda di luar VM sesi, independen dari [tingkat akses](#access-levels) lingkungan. Sesi di lingkungan yang di-host sendiri mengautentikasi operasi git dengan kredensial yang disediakan deployment Anda; [Konfigurasi git](/docs/id/self-hosted-environments-deploy#configure-git) mencakup opsi, termasuk kredensial yang dicetak per sesi dan opt-in ke proxy yang sama ini. Proxy menyediakan:

* **Kredensial Git**: klien git di dalam VM menggunakan kredensial yang dibatasi, yang diverifikasi proxy dan ditukar dengan token GitHub aktual Anda.
* **Permintaan API**: permintaan dari alat GitHub bawaan, dan dari `gh` di bawah [placeholder `proxy-injected`](#work-with-github-issues-and-pull-requests), keluar dengan kredensial nyata Anda diganti.
* **Push protection**: `git push` hanya bekerja terhadap cabang kerja saat ini sesi; kloning, pengambilan, dan operasi PR bekerja secara normal.
* **Cakupan repositori**: API GitHub dan permintaan aset rilis hanya menjangkau repositori yang terlampir pada sesi, jadi skrip setup yang mengunduh aset rilis dari repositori yang tidak terlampir mendapat 403.
* **Pembatasan GraphQL**: proxy melayani hanya set operasi GraphQL yang disematkan untuk alur kerja pull-request. Proxy menolak segalanya di endpoint GraphQL dengan 403 yang mengatakan `This GraphQL query is not enabled for this session` dan menyebutkan fallback REST, `gh api repos/{owner}/{repo}/...`. Pembatasan berlaku untuk setiap permintaan melalui proxy terlepas dari kredensial yang Anda berikan, jadi `GH_TOKEN` yang Anda tetapkan mendapat 403 yang sama. Claude tidak dapat menjangkau API GitHub yang hanya ada di GraphQL, seperti Projects v2, melalui proxy.

File yang berkomitmen dari repositori publik tiba melalui `raw.githubusercontent.com`, yang ditangani [proxy keamanan](#security-proxy) sebagai gantinya. Domain itu ada dalam daftar [Trusted](#default-allowed-domains) default, jadi file tersebut tetap dapat dijangkau kecuali [tingkat akses](#access-levels) lingkungan mengecualikannya.

<h3 id="security-proxy">
  Proxy keamanan
</h3>

Sesi cloud di lingkungan yang di-host Anthropic berjalan di belakang proxy jaringan HTTP/HTTPS untuk tujuan keamanan dan pencegahan penyalahgunaan; di [lingkungan yang di-host sendiri](/docs/id/self-hosted-environments-deploy#default-deny-egress), lalu lintas keluar meninggalkan melalui batas jaringan Anda sendiri sebagai gantinya. Semua lalu lintas internet keluar dari sesi yang di-host Anthropic melewati proxy ini, yang menyediakan:

* Perlindungan terhadap permintaan berbahaya
* Pembatasan laju dan pencegahan penyalahgunaan
* Penyaringan konten untuk keamanan yang ditingkatkan
* Jejak audit tingkat DNS dari nama host yang diminta

<h2 id="what’s-available-in-cloud-sessions">
  Apa yang tersedia di sesi cloud
</h2>

Di lingkungan yang di-host Anthropic, setiap sesi mendapat mesin virtual (VM) segar yang menjalankan Ubuntu 24.04 pada x86\_64, terlepas dari sistem operasi dan arsitektur CPU Anda sendiri, dengan repositori Anda yang dikloning dan toolchain umum yang sudah diinstal sebelumnya. Ketika dependensi menyediakan binari yang sudah dikompilasi, seperti gem Ruby dengan ekstensi asli atau wheel Python yang sudah dibangun, gunakan build Linux x86\_64 untuk mencocokkan VM. Bagian ini mencakup default yang di-host Anthropic, alat GitHub bawaan, cara [menjalankan tes dan layanan](#run-tests-start-services-and-add-packages), dan [batas sumber daya](#resource-limits) yang diterima setiap VM.

<Note>
  Sesi yang organisasi Anda arahkan ke [lingkungan yang di-host sendiri](/docs/id/self-hosted-environments) berjalan di runner Anda sendiri sebagai gantinya, dengan alat yang disediakan gambar runner Anda.
</Note>

<h3 id="what-carries-over-from-your-setup">
  Apa yang terbawa dari setup Anda
</h3>

Cloud sessions start from a fresh clone of your repository. Apa pun yang Anda komitkan ke repo tersedia. Apa pun yang Anda instal atau konfigurasi hanya di mesin Anda sendiri tidak tersedia di sesi. Kebijakan organisasi Anda tiba secara terpisah melalui [pengaturan yang dikelola server](/docs/id/server-managed-settings).

|                                                                                                                                                                                | Tersedia di sesi cloud                                                 | Mengapa                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `CLAUDE.md` repo Anda                                                                                                                                                          | Ya                                                                     | Bagian dari klon                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| Hook `.claude/settings.json` repo Anda dan aturan izin                                                                                                                         | Ya, dalam sesi dengan satu repositori                                  | Bagian dari klon. Sesi dengan beberapa repositori, termasuk thread [proyek](/docs/id/claude-projects#what-threads-pick-up-from-your-repositories), dimulai di atas klon dan tidak membacanya                                                                                                                                                                                                                                                                                                                                                                                                               |
| Server MCP `.mcp.json` repo Anda                                                                                                                                               | Ya, dalam sesi dengan satu repositori                                  | Bagian dari klon, ditemukan dari direktori kerja sesi                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `.claude/rules/` repo Anda                                                                                                                                                     | Ya                                                                     | Bagian dari klon                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `.claude/skills/`, `.claude/agents/`, `.claude/commands/` repo Anda                                                                                                            | Ya                                                                     | Bagian dari klon                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| Plugin dan marketplace yang dideklarasikan di `.claude/settings.json` repo Anda                                                                                                | Tidak                                                                  | Sesi cloud tidak menginstal plugin yang dihidupkan repositori di bawah [`enabledPlugins`](/docs/id/settings-reference#enabledplugins), termasuk yang dari marketplace yang dicantumkan di bawah [`extraKnownMarketplaces`](/docs/id/settings-reference#extraknownmarketplaces)                                                                                                                                                                                                                                                                                                                                  |
| [Pengaturan yang dikelola server](/docs/id/server-managed-settings) organisasi Anda                                                                                                 | Ya                                                                     | Diambil dari server Anthropic ketika sesi dimulai. Lihat [Surface coverage](/docs/id/model-config#surface-coverage) untuk cara `availableModels` diterapkan di sesi cloud. Pengaturan yang digunakan di perangkat Anda melalui MDM atau file pengaturan yang dikelola tidak berlaku, karena sesi berjalan di VM yang dikelola Anthropic; di [lingkungan yang di-host sendiri](/docs/id/self-hosted-environments), sesi juga membaca file pengaturan yang dikelola dalam gambar runner, per [cara Claude Code menggabungkan sumber yang dikelola](/docs/id/managed-settings#how-claude-code-combines-managed-sources) |
| `~/.claude/CLAUDE.md` pengguna Anda                                                                                                                                            | Tidak                                                                  | Hidup di mesin Anda, bukan di repo                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `~/.claude/skills/`, `~/.claude/agents/`, `~/.claude/commands/` pengguna Anda                                                                                                  | Tidak                                                                  | Hidup di mesin Anda, bukan di repo. Komitkan mereka ke direktori `.claude/` repo sebagai gantinya. Sesi cloud secara otomatis memuat skill yang Anda aktifkan di claude.ai                                                                                                                                                                                                                                                                                                                                                                                                                            |
| Plugin yang hanya diaktifkan di pengaturan pengguna Anda                                                                                                                       | Tidak                                                                  | `enabledPlugins` yang dibatasi pengguna hidup di `~/.claude/settings.json` di mesin Anda                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| Server MCP yang Anda tambahkan dengan `claude mcp add` di cakupan lokal default atau cakupan pengguna                                                                          | Tidak                                                                  | Mereka menulis ke `~/.claude.json` di mesin Anda, bukan repo. Tambahkan server dengan `claude mcp add --scope project`, yang menulis [`.mcp.json`](/docs/id/mcp#project-scope) repo, dan komitkan file itu. Sesi dengan satu repositori memuat file itu                                                                                                                                                                                                                                                                                                                                                    |
| Variabel transport di blok `env` `.claude/settings.json` repo Anda, seperti `NODE_EXTRA_CA_CERTS` dan [variabel sertifikat klien mTLS](/docs/id/network-config#mtls-authentication) | Tidak                                                                  | Lingkungan hosting mengelola koneksi API sesi, jadi Claude Code mengabaikan kunci ini dan mencatat setiap kunci yang diabaikan di log debug sesi                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| Token API dan kredensial untuk layanan yang dipanggil Claude                                                                                                                   | Pada paket Pro dan Max, sebagai [kredensial API](#add-api-credentials) | Anda menambahkan kunci sekali di lingkungan dan proxy agen melampirkannya ke permintaan untuk host yang Anda cantumkan. Kunci yang proxy agen [tidak dapat lampirkan](#requests-that-never-get-the-credential), atau kunci apa pun pada paket Team atau Enterprise, tetap dalam variabel lingkungan                                                                                                                                                                                                                                                                                                   |
| Auth interaktif seperti AWS SSO                                                                                                                                                | Tidak                                                                  | Tidak didukung. SSO memerlukan login berbasis browser yang tidak dapat berjalan di sesi cloud                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |

Untuk membuat konfigurasi Anda sendiri tersedia di sesi cloud, komitkan ke repo.

Siapa pun yang menggunakan lingkungan dapat membaca variabel lingkungan dan skrip setupnya. Catatan dialog di bawah **Environment variables** mengatakan demikian dan memperingatkan terhadap penambahan rahasia di sana. Pada paket Pro dan Max, simpan kunci yang dapat dilampirkan proxy agen sebagai [kredensial API](#add-api-credentials) sebagai gantinya.

<h3 id="installed-tools">
  Alat yang diinstal
</h3>

Sesi cloud dilengkapi dengan runtime bahasa umum, alat build, dan database yang sudah diinstal sebelumnya. Tabel di bawah merangkum apa yang disertakan menurut kategori.

| Kategori      | Disertakan                                                                   |
| :------------ | :--------------------------------------------------------------------------- |
| **Python**    | Python 3.x dengan pip, poetry, uv, black, mypy, pytest, ruff                 |
| **Node.js**   | 20, 21, dan 22, dengan npm, yarn, pnpm, bun¹, eslint, prettier, chromedriver |
| **Ruby**      | 3.1, 3.2, 3.3 dengan gem, bundler, rbenv                                     |
| **PHP**       | 8.3 dengan Composer                                                          |
| **Java**      | OpenJDK 21 dengan Maven dan Gradle                                           |
| **Go**        | Go dengan dukungan modul                                                     |
| **Rust**      | rustc dan cargo                                                              |
| **C/C++**     | GCC, Clang, cmake, ninja, conan                                              |
| **Docker**    | docker, dockerd, docker compose                                              |
| **Databases** | PostgreSQL 16, Redis 7.0                                                     |
| **Utilities** | git, gh, jq, yq, ripgrep, tmux, vim, nano                                    |

¹ Bun diinstal tetapi memiliki [masalah kompatibilitas proxy](#install-dependencies-with-a-sessionstart-hook) yang diketahui untuk pengambilan paket.

Untuk mendapatkan versi sebagian besar alat dalam tabel ini, minta Claude menjalankan `check-tools` di sesi cloud. Ini adalah perintah shell yang diinstal di VM sesi, bukan perintah yang Anda ketik dengan `/`; Anda meminta Claude karena [Claude menjalankan semua perintah VM untuk Anda](#run-tests-start-services-and-add-packages). Untuk alat yang tidak dilaporkannya, seperti Ruby, PHP, bun, PostgreSQL, atau Redis, minta Claude menjalankan perintah versi alat itu sendiri, misalnya `psql --version`.

Versi Node.js diinstal di `/opt/node20`, `/opt/node21`, dan `/opt/node22`, dengan 22 di `PATH` secara default. Untuk bekerja dengan versi berbeda, minta Claude menambahkan direktori `bin` versi itu, seperti `/opt/node20/bin`, ke `PATH`.

Toolchain di luar daftar ini, seperti .NET SDK, tidak diinstal sebelumnya bahkan ketika registri paket mereka ada di [allowlist default](#default-allowed-domains). Instal mereka dengan [skrip setup](#setup-scripts).

<h3 id="work-with-github-issues-and-pull-requests">
  Bekerja dengan masalah GitHub dan permintaan tarik
</h3>

Sesi cloud mencakup alat GitHub bawaan yang memungkinkan Claude membaca masalah, mencantumkan permintaan tarik, mengambil diff, dan memposting komentar tanpa setup apa pun. Alat ini mengautentikasi melalui [proxy GitHub](#github-proxy) menggunakan metode apa pun yang Anda konfigurasi di bawah [opsi autentikasi GitHub](/docs/id/claude-code-on-the-web#github-authentication-options), jadi token Anda tidak pernah memasuki kontainer.

Anda dapat menetapkan `GH_TOKEN` atau `GITHUB_TOKEN` sendiri di [pengaturan lingkungan](#set-environment-variables), atau biarkan keduanya tidak ditetapkan dan biarkan [proxy GitHub](#github-proxy) mengautentikasi untuk Anda:

* Jika Anda menetapkan token, itu melewati ke kontainer tanpa perubahan, jadi skrip Anda dan [`gh` CLI](https://cli.github.com) GitHub menggunakannya secara langsung.
* Jika Anda tidak menetapkan keduanya dan [proxy GitHub](#github-proxy) menangani autentikasi untuk sesi Anda, kedua variabel membaca sebagai string placeholder `proxy-injected` dalam perintah yang dijalankan Claude, dan proxy mengganti kredensial nyata Anda pada permintaan GitHub keluar. `gh` bekerja tanpa token Anda sendiri, tetapi skrip yang membaca `GITHUB_TOKEN` secara langsung mendapat placeholder, bukan token yang dapat digunakan.

Token yang Anda tetapkan adalah variabel lingkungan biasa, jadi siapa pun yang menggunakan lingkungan dapat membacanya; jalur proxy menjaga kredensial di luar konfigurasi lingkungan dan VM sesi.

Untuk memeriksa kasus mana yang berlaku untuk sesi Anda, minta Claude menjalankan `echo $GH_TOKEN`.

[`gh` CLI](https://cli.github.com) GitHub sudah diinstal sebelumnya. Jika Anda memerlukan perintah `gh` yang tidak dicakup alat bawaan, seperti `gh release` atau `gh workflow run`, minta Claude menjalankannya. `gh` membaca `GH_TOKEN` secara otomatis, jadi Anda tidak perlu menjalankan `gh auth login`.

<h3 id="link-output-back-to-the-session">
  Hubungkan output kembali ke sesi
</h3>

Setiap sesi cloud memiliki URL transkrip di claude.ai, dan sesi dapat membaca ID-nya sendiri dari variabel lingkungan `CLAUDE_CODE_REMOTE_SESSION_ID`. Gunakan ini untuk menempatkan tautan yang dapat dilacak di badan PR, pesan komitmen, posting Slack, atau laporan yang dihasilkan sehingga reviewer dapat membuka jalankan yang menghasilkannya.

Komitmen yang dibuat Claude di sesi cloud mencakup trailer git `Claude-Session: <url>`, dan badan PR mencakup URL sesi di barisnya sendiri. Untuk menghilangkan trailer dan tautan badan PR, atur [`attribution.sessionUrl`](/docs/id/settings-reference#attribution-sessionurl) ke `false`.

Untuk menyertakan tautan sesi dalam sesuatu selain komitmen atau PR, seperti pesan Slack yang diposting Claude atau file laporan yang ditulisnya, minta Claude menjalankan perintah berikut dan gunakan outputnya. Perintah mengonversi awalan `cse_` dalam nilai variabel lingkungan ke awalan `session_` yang diharapkan URL transkrip:

```bash theme={null}
echo "https://claude.ai/code/${CLAUDE_CODE_REMOTE_SESSION_ID/#cse_/session_}"
```

<h3 id="run-tests-start-services-and-add-packages">
  Jalankan tes, mulai layanan, dan tambahkan paket
</h3>

Anda tidak mendapatkan shell ke VM sesi. Claude menjalankan setiap perintah untuk Anda, jadi frasekan tugas di bagian ini sebagai permintaan dalam prompt Anda.

<h4 id="run-tests">
  Jalankan tes
</h4>

Claude menjalankan tes sebagai bagian dari mengerjakan tugas. Minta dalam prompt Anda, seperti "perbaiki tes yang gagal di `tests/`" atau "jalankan pytest setelah setiap perubahan." Pelari tes yang datang dengan [toolchain yang sudah diinstal sebelumnya](#installed-tools), seperti pytest dan cargo test, bekerja tanpa setup tambahan. Pelari yang dideklarasikan proyek Anda sebagai dependensi, seperti jest, diinstal dengan dependensi Anda.

<h4 id="start-services">
  Mulai layanan
</h4>

PostgreSQL dan Redis sudah diinstal tetapi tidak berjalan secara default. Minta Claude untuk memulai yang Anda butuhkan; perintah yang dijalankannya adalah:

```bash theme={null}
service postgresql start
```

```bash theme={null}
service redis-server start
```

Docker tersedia untuk menjalankan layanan yang dikontainerisasi. Minta Claude menjalankan `docker compose up` untuk memulai layanan proyek Anda. Akses jaringan untuk menarik gambar mengikuti [tingkat akses](#access-levels) lingkungan Anda, dan [default Trusted](#default-allowed-domains) mencakup Docker Hub dan registri umum lainnya.

Jika gambar Anda besar atau lambat ditarik, tambahkan `docker compose pull` atau `docker compose build` ke [skrip setup](#setup-scripts) Anda. [Cache lingkungan](#environment-caching) menyimpan gambar yang ditarik, jadi setiap sesi baru memilikinya di disk. Cache menyimpan file saja, bukan proses yang berjalan, jadi Claude masih memulai kontainer setiap sesi.

<h4 id="add-packages">
  Tambahkan paket
</h4>

Untuk menambahkan paket yang tidak diinstal sebelumnya, gunakan [skrip setup](#setup-scripts). [Cache lingkungan](#environment-caching) menyimpan apa yang diinstal skrip, jadi paket yang Anda instal di sana tersedia di awal setiap sesi tanpa menginstal ulang setiap kali. Anda juga dapat meminta Claude menginstal paket di tengah sesi, tetapi instalasi tersebut tidak terbawa ke sesi lain.

<h3 id="resource-limits">
  Batas sumber daya
</h3>

Sesi cloud di lingkungan yang di-host Anthropic berjalan dengan batas sumber daya perkiraan yang mungkin berubah seiring waktu:

* 4 vCPU
* 16 GB RAM
* 30 GB disk

VM mungkin menghentikan tugas yang memerlukan memori jauh lebih banyak, seperti pekerjaan build besar atau tes yang memakan memori. Untuk beban kerja di luar batas ini, gunakan [Remote Control](/docs/id/remote-control) untuk menjalankan Claude Code di perangkat keras Anda sendiri, atau jalankan sesi cloud di [lingkungan yang di-host sendiri](/docs/id/self-hosted-environments) pada komputasi yang dioperasikan organisasi Anda.

<h2 id="setup-scripts">
  Skrip setup
</h2>

Skrip setup adalah skrip Bash yang berjalan ketika sesi cloud baru dimulai, sebelum Claude Code diluncurkan. Gunakan skrip setup untuk menginstal dependensi, mengonfigurasi alat, atau mengambil apa pun yang dibutuhkan sesi yang tidak diinstal sebelumnya.

Skrip berjalan sebagai root di Ubuntu 24.04, jadi `apt install` dan sebagian besar manajer paket bahasa bekerja.

Untuk menambahkan skrip setup, buka dialog pengaturan lingkungan dan masukkan skrip Anda di bidang **Setup script**.

Contoh ini menginstal [ShellCheck](https://www.shellcheck.net/), yang tidak diinstal sebelumnya.

```bash theme={null}
#!/bin/bash
apt update && apt install -y shellcheck
```

<h3 id="script-requirements">
  Persyaratan skrip
</h3>

Skrip setup memiliki tiga batasan untuk ditulis:

* **Exit zero**: jika skrip keluar non-zero, sesi gagal dimulai. Tambahkan `|| true` ke perintah non-kritis sehingga kegagalan instalasi sesekali tidak memblokir sesi.
* **Selesai dalam lima menit**: jaga runtime total skrip di bawah kira-kira lima menit sehingga [cache lingkungan](#environment-caching) dapat dibangun. Jalankan instalasi independen secara paralel dengan `&` dan `wait`, dan pindahkan unduhan tunggal apa pun yang tidak cocok ke [hook SessionStart](#setup-scripts-vs-sessionstart-hooks) yang meluncurkannya di latar belakang.
* **Akses jaringan untuk instalasi**: instalasi paket perlu menjangkau registri. Tingkat **Trusted** default mencakup [registri paket umum](#default-allowed-domains) termasuk npm, PyPI, RubyGems, dan crates.io; dengan akses jaringan **None**, instalasi gagal.

<h3 id="environment-caching">
  Caching lingkungan
</h3>

Skrip setup berjalan pertama kali Anda memulai sesi di lingkungan. Setelah selesai, Anthropic membuat snapshot sistem file dan menggunakan kembali snapshot itu sebagai titik awal untuk sesi nanti. Sesi baru dimulai dengan dependensi, alat, dan gambar Docker Anda sudah di disk, dan melewati langkah skrip setup. Ini menjaga startup cepat bahkan ketika skrip menginstal toolchain besar atau menarik gambar kontainer.

Cache adalah snapshot sistem file, jadi menyimpan apa yang ditulis skrip setup ke disk dan kehilangan apa pun yang hanya berjalan. Paket yang Anda instal, gambar Docker yang Anda tarik, dan file yang Anda tulis semuanya terbawa. Database yang dimulai skrip, tumpukan `docker compose up`, atau proses latar belakang lainnya tidak; mulai mereka per sesi dengan meminta Claude atau dengan [hook SessionStart](#setup-scripts-vs-sessionstart-hooks).

Skrip setup berjalan lagi untuk membangun ulang cache ketika Anda mengubah skrip setup lingkungan atau host jaringan yang diizinkan, dan ketika cache mencapai kedaluwarsa setelah kira-kira tujuh hari. Melanjutkan sesi yang ada tidak pernah menjalankan kembali skrip setup.

Anda tidak perlu mengaktifkan caching atau mengelola snapshot sendiri.

<h3 id="setup-scripts-vs-sessionstart-hooks">
  Skrip setup vs. hook SessionStart
</h3>

Gunakan skrip setup untuk menyediakan VM itu sendiri: toolchain dan alat CLI yang tidak [diinstal sebelumnya](#installed-tools). Gunakan [hook SessionStart](/docs/id/hooks#sessionstart) untuk setup proyek yang harus berjalan di mana-mana, cloud dan lokal, seperti `npm install`.

Skrip setup dan hook SessionStart berjalan dalam urutan tetap ketika sesi cloud dimulai. Tabel membandingkan di mana Anda mengonfigurasinya, kapan mereka berjalan, dan di mana mereka berjalan.

|                                    | Skrip setup                                                                                                                                                                | Hook SessionStart                                                                                                                                                                                                      |
| ---------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Di mana Anda mengonfigurasinya** | Dialog lingkungan di [claude.ai/code](https://claude.ai/code), ditambah halaman admin **Cloud environments** untuk [lingkungan bersama](#organization-shared-environments) | File [pengaturan](/docs/id/settings#where-settings-live) seperti `.claude/settings.json` repo Anda; lihat [Apa yang terbawa dari setup Anda](#what-carries-over-from-your-setup) untuk file mana yang menjangkau sesi cloud |
| **Kapan mereka berjalan**          | Sebelum Claude Code diluncurkan, dilewati ketika [lingkungan cache](#environment-caching) ada                                                                              | Setelah Claude Code diluncurkan, di setiap sesi termasuk yang dilanjutkan                                                                                                                                              |
| **Di mana mereka berjalan**        | Sesi cloud saja                                                                                                                                                            | Sesi lokal dan cloud                                                                                                                                                                                                   |

Jika Anda memiliki hook SessionStart di `~/.claude/settings.json` tingkat pengguna, jangan harapkan mereka di cloud. Pengaturan tingkat pengguna tetap di mesin Anda. Hook mana yang berjalan tergantung di mana sesi berjalan:

* **Lingkungan yang dihosting Anthropic**: Claude Code menjalankan hook dari repositori dan dari [pengaturan yang dikelola server](/docs/id/server-managed-settings) organisasi Anda.
* **[Lingkungan yang dihosting sendiri](/docs/id/self-hosted-environments-configuration#permissions-and-tool-approval)**: Claude Code juga menjalankan hook yang ditanam operator dari `~/.claude/` host runner, dan hook dalam file pengaturan yang dikelola gambar runner ketika file itu adalah salah satu dari [sumber yang dikelola Claude Code terapkan](/docs/id/managed-settings#how-claude-code-combines-managed-sources).

<h3 id="install-dependencies-with-a-sessionstart-hook">
  Instal dependensi dengan hook SessionStart
</h3>

Untuk menginstal dependensi hanya di sesi cloud, pasangkan hook SessionStart dengan skrip yang memeriksa di mana itu berjalan.

Pertama, tambahkan hook SessionStart ke `.claude/settings.json` repo Anda. Konfigurasi ini memberitahu Claude Code untuk menjalankan `scripts/install_pkgs.sh` dari repositori Anda setiap kali sesi dimulai atau dilanjutkan:

```json theme={null}
{
  "hooks": {
    "SessionStart": [
      {
        "matcher": "startup|resume",
        "hooks": [
          {
            "type": "command",
            "command": "bash \"$CLAUDE_PROJECT_DIR\"/scripts/install_pkgs.sh"
          }
        ]
      }
    ]
  }
}
```

`matcher` membatasi hook ke acara `startup` dan `resume`, dan `$CLAUDE_PROJECT_DIR` diselesaikan ke akar repositori, jadi hook menemukan skrip terlepas dari direktori kerja sesi.

Selanjutnya, buat skrip di `scripts/install_pkgs.sh`. Itu keluar segera di luar cloud, kemudian menginstal dependensi Anda:

```bash theme={null}
#!/bin/bash

if [ "$CLAUDE_CODE_REMOTE" != "true" ]; then
  exit 0
fi

npm install
pip install -r requirements.txt
exit 0
```

Pemeriksaan `CLAUDE_CODE_REMOTE` adalah apa yang membatasi instalasi ke sesi cloud: VM sesi membawa variabel itu sebagai `true`, tidak pernah `true` secara lokal, jadi di laptop Anda skrip keluar sebelum menginstal apa pun.

Bersama-sama, dua file memberikan setiap sesi cloud `npm install` dan `pip install` segar saat startup sambil meninggalkan sesi lokal tidak tersentuh.

<h4 id="limitations-in-cloud-sessions">
  Batasan di sesi cloud
</h4>

Hook SessionStart berperilaku sama di cloud seperti secara lokal, dengan peringatan ini:

* **Satu repositori per sesi**: sesi dengan beberapa repositori tidak memuat hook dari `.claude/settings.json` repositori apa pun, jadi hook SessionStart yang Anda tentukan di sana tidak berjalan. Instal dependensi untuk sesi tersebut dengan [skrip setup](#setup-scripts) sebagai gantinya.
* **Tidak ada scoping khusus cloud**: hook berjalan di sesi lokal dan cloud. Untuk melewati eksekusi lokal, keluar lebih awal kecuali variabel lingkungan `CLAUDE_CODE_REMOTE` adalah `true`, cara yang ditunjukkan oleh [skrip instalasi dependensi](#install-dependencies-with-a-sessionstart-hook).
* **Memerlukan akses jaringan**: perintah instalasi perlu menjangkau registri paket. Jika lingkungan Anda menggunakan akses jaringan **None**, hook ini gagal. [Allowlist default](#default-allowed-domains) di bawah **Trusted** mencakup npm, PyPI, RubyGems, dan crates.io.
* **Kompatibilitas proxy**: di lingkungan yang dihosting Anthropic, semua lalu lintas keluar melewati [proxy keamanan](#security-proxy), dan beberapa manajer paket tidak bekerja dengan benar dengannya; Bun adalah contoh yang diketahui. Di [lingkungan yang dihosting sendiri](/docs/id/self-hosted-environments-deploy#default-deny-egress), lalu lintas keluar melewati batas jaringan Anda sendiri.
* **Menambah latensi startup**: hook berjalan setiap kali sesi dimulai atau dilanjutkan, tidak seperti skrip setup yang mendapat manfaat dari [caching lingkungan](#environment-caching). Jaga skrip instalasi cepat dengan memeriksa apakah dependensi sudah ada sebelum menginstal ulang.

Untuk menyesuaikan gambar dasar, gunakan skrip setup untuk menginstal apa yang Anda butuhkan di atas [gambar yang disediakan](#installed-tools), atau jalankan gambar Anda sendiri sebagai kontainer bersama Claude dengan `docker compose`. Mengganti gambar dasar sepenuhnya belum didukung.

<h2 id="default-allowed-domains">
  Domain yang diizinkan default
</h2>

Dengan akses jaringan **Trusted**, sesi dapat menjangkau domain berikut secara default. Domain yang ditandai dengan `*` menunjukkan pencocokan subdomain wildcard, jadi `*.gcr.io` memungkinkan subdomain apa pun dari `gcr.io`.

<AccordionGroup>
  <Accordion title="Layanan Anthropic">
    * api.anthropic.com
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
    * \*.r2.cloudflarestorage.com
  </Accordion>

  <Accordion title="Manajer paket JavaScript dan Node">
    * registry.npmjs.org
    * [www.npmjs.com](http://www.npmjs.com)
    * [www.npmjs.org](http://www.npmjs.org)
    * npmjs.com
    * npmjs.org
    * yarnpkg.com
    * registry.yarnpkg.com
    * jsr.io
    * npm.jsr.io
  </Accordion>

  <Accordion title="Manajer paket Python">
    * pypi.org
    * [www.pypi.org](http://www.pypi.org)
    * files.pythonhosted.org
    * pythonhosted.org
    * test.pypi.org
    * pypi.python.org
    * pypa.io
    * [www.pypa.io](http://www.pypa.io)
  </Accordion>

  <Accordion title="Manajer paket Ruby">
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

  <Accordion title="Manajer paket Rust">
    * crates.io
    * [www.crates.io](http://www.crates.io)
    * index.crates.io
    * static.crates.io
    * rustup.rs
    * static.rust-lang.org
    * [www.rust-lang.org](http://www.rust-lang.org)
  </Accordion>

  <Accordion title="Manajer paket Go">
    * proxy.golang.org
    * sum.golang.org
    * index.golang.org
    * golang.org
    * [www.golang.org](http://www.golang.org)
    * goproxy.io
    * pkg.go.dev
  </Accordion>

  <Accordion title="Manajer paket JVM">
    * maven.org
    * repo.maven.org
    * central.maven.org
    * repo1.maven.org
    * repo.maven.apache.org
    * maven.google.com
    * jcenter.bintray.com
    * gradle.org
    * [www.gradle.org](http://www.gradle.org)
    * services.gradle.org
    * plugins.gradle.org
    * plugins-artifacts.gradle.org
    * kotlinlang.org
    * [www.kotlinlang.org](http://www.kotlinlang.org)
    * spring.io
    * repo.spring.io
  </Accordion>

  <Accordion title="Manajer paket lainnya">
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
    * http-intake.logs.datadoghq.com
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

<h2 id="related-resources">
  Sumber daya terkait
</h2>

* [Referensi sesi cloud](/docs/id/claude-code-on-the-web): mulai, kelola, dan bagikan sesi cloud
* [Quickstart sesi cloud](/docs/id/web-quickstart): hubungkan GitHub dan mulai sesi cloud pertama Anda
* [Claude Tag](https://claude.com/docs/claude-tag/overview): sesi yang dimulai Claude dari Slack berjalan di lingkungan yang sama
* [Routines](/docs/id/routines): jalankan terjadwal menggunakan lingkungan dan tingkat akses jaringan yang sama
* [Remote Control](/docs/id/remote-control): jalankan sesi di jaringan dan file mesin Anda sendiri sebagai gantinya
* [Lingkungan yang di-host sendiri](/docs/id/self-hosted-environments): jalankan sesi cloud di infrastruktur organisasi Anda sendiri
* [Hook SessionStart](/docs/id/hooks#sessionstart): setup yang berkomitmen repo yang berjalan di sesi lokal dan cloud
* [Pengaturan yang dikelola server](/docs/id/server-managed-settings): kebijakan organisasi yang menjangkau sesi cloud
