> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Konfigurasi jaringan enterprise

> Konfigurasikan Claude Code untuk lingkungan enterprise dengan server proxy, Certificate Authorities (CA) kustom, dan autentikasi mutual Transport Layer Security (mTLS).

Claude Code mendukung berbagai konfigurasi jaringan dan keamanan enterprise melalui variabel lingkungan. Ini termasuk merutekan lalu lintas melalui server proxy perusahaan, mempercayai Certificate Authorities (CA) kustom, dan mengautentikasi dengan sertifikat mutual Transport Layer Security (mTLS) untuk keamanan yang ditingkatkan.

Atur variabel lingkungan ini sebelum Anda meluncurkan Claude Code. Variabel yang diekspor di shell Anda dibaca sekali saat startup, jadi sesi yang sedang berjalan tidak mengambil perubahan nanti ke lingkungan shell Anda.

<Note>
  Semua variabel lingkungan yang ditampilkan di halaman ini juga dapat dikonfigurasi di [`settings.json`](/docs/id/settings).
</Note>

<h2 id="proxy-configuration">
  Konfigurasi proxy
</h2>

<h3 id="environment-variables">
  Variabel lingkungan
</h3>

Claude Code menghormati variabel lingkungan proxy standar. Dalam sesi Claude Desktop di mana aplikasi mengelola koneksi penyedia, Claude Code membacanya hanya dari pengaturan terkelola dan `~/.claude/settings.json`; lihat [autentikasi mTLS](#mtls-authentication) untuk aturan cakupan.

```bash theme={null}
# HTTPS proxy (direkomendasikan)
export HTTPS_PROXY=https://proxy.example.com:8080

# HTTP proxy (jika HTTPS tidak tersedia)
export HTTP_PROXY=http://proxy.example.com:8080

# Lewati proxy untuk permintaan tertentu - format terpisah spasi
export NO_PROXY="localhost 192.168.1.1 example.com .example.com"
# Lewati proxy untuk permintaan tertentu - format terpisah koma
export NO_PROXY="localhost,192.168.1.1,example.com,.example.com"
# Lewati proxy untuk semua permintaan
export NO_PROXY="*"
```

Varian huruf kecil juga berfungsi, dan Claude Code menggunakan yang pertama yang diatur dalam urutan `https_proxy`, `HTTPS_PROXY`, `http_proxy`, `HTTP_PROXY`.

Claude Code tidak pernah mengirim koneksi WebSocket-nya ke `localhost`, `::1`, atau `127.0.0.0/8` melalui proxy, jadi Anda tidak perlu entri loopback dalam `NO_PROXY` untuk mereka.

<Note>
  Claude Code tidak mendukung proxy SOCKS.
</Note>

<h3 id="basic-authentication">
  Autentikasi dasar
</h3>

Jika proxy Anda memerlukan autentikasi dasar, sertakan kredensial dalam URL proxy:

```bash theme={null}
export HTTPS_PROXY=http://username:password@proxy.example.com:8080
```

<Warning>
  Hindari hardcoding kata sandi dalam skrip. Gunakan variabel lingkungan atau penyimpanan kredensial aman sebagai gantinya.
</Warning>

<Tip>
  Untuk proxy yang memerlukan autentikasi lanjutan (NTLM, Kerberos, dll.), pertimbangkan menggunakan layanan LLM Gateway yang mendukung metode autentikasi Anda.
</Tip>

<h2 id="ca-certificate-store">
  Penyimpanan sertifikat CA
</h2>

Secara default, Claude Code mempercayai baik sertifikat CA Mozilla yang disertakan maupun penyimpanan sertifikat sistem operasi Anda. Membaca penyimpanan OS memerlukan runtime dengan `tls.getCACertificates`: installer native selalu memilikinya, dan instalasi npm memerlukan Node 22.15 atau lebih baru. Pada versi Node yang lebih lama, hanya set yang disertakan dan `NODE_EXTRA_CA_CERTS` yang berlaku. Proxy inspeksi TLS enterprise bekerja tanpa konfigurasi tambahan ketika sertifikat akar mereka diinstal di penyimpanan kepercayaan OS dan runtime dapat membacanya.

`CLAUDE_CODE_CERT_STORE` menerima daftar sumber yang dipisahkan koma. Nilai yang dikenali adalah `bundled` untuk set CA Mozilla yang dikirimkan dengan Claude Code dan `system` untuk penyimpanan kepercayaan sistem operasi. Default adalah `bundled,system`.

Untuk mempercayai hanya set CA Mozilla yang disertakan:

```bash theme={null}
export CLAUDE_CODE_CERT_STORE=bundled
```

Untuk mempercayai hanya penyimpanan sertifikat OS:

```bash theme={null}
export CLAUDE_CODE_CERT_STORE=system
```

<Note>
  `CLAUDE_CODE_CERT_STORE` tidak memiliki kunci skema `settings.json` khusus. Aturnya melalui blok `env` di `~/.claude/settings.json` atau langsung di lingkungan proses.
</Note>

<h2 id="custom-ca-certificates">
  Sertifikat CA kustom
</h2>

Jika lingkungan enterprise Anda menggunakan CA kustom, konfigurasikan Claude Code untuk mempercayainya secara langsung:

```bash theme={null}
export NODE_EXTRA_CA_CERTS=/path/to/ca-cert.pem
```

<h2 id="mtls-authentication">
  Autentikasi mTLS
</h2>

Untuk lingkungan enterprise yang memerlukan autentikasi sertifikat klien:

```bash theme={null}
# Sertifikat klien untuk autentikasi
export CLAUDE_CODE_CLIENT_CERT=/path/to/client-cert.pem

# Kunci privat klien
export CLAUDE_CODE_CLIENT_KEY=/path/to/client-key.pem

# Opsional: Frasa sandi untuk kunci privat terenkripsi
export CLAUDE_CODE_CLIENT_KEY_PASSPHRASE="your-passphrase"
```

Claude Code membaca file sertifikat dan kunci saat startup dan membacanya kembali setiap kali menerapkan pengaturan, seperti ketika organisasi Anda mengubah blok `env` dalam [pengaturan terkelola](/docs/id/server-managed-settings) di tengah sesi.

Untuk merotasi sertifikat dan kunci, ganti file di jalur yang sama. Claude Code mengambil penggantian dalam sesi yang sedang berjalan tanpa perlu restart. Ketika permintaan API gagal dengan kesalahan tingkat koneksi, seperti reset koneksi atau kesalahan handshake TLS, Claude Code membaca kembali kedua file dan mencoba ulang permintaan dengan pasangan baru. Sebelum v2.1.232, Claude Code tidak membaca ulang pada kesalahan koneksi, jadi Claude Code tetap menggunakan pasangan yang sudah dimuat sampai berikutnya menerapkan pengaturan atau Anda melakukan restart.

Claude Code membaca kembali file sebagai respons terhadap permintaan yang gagal, bukan dengan memantau perubahan file:

* **Waktu**: Claude Code tidak melakukan apa pun pada saat Anda mengganti file. Claude Code menyajikan pasangan baru pada percobaan ulang setelah kegagalan yang memenuhi syarat, atau pada permintaan berikutnya setelah menerapkan pengaturan, mana pun yang lebih dulu.
* **Penolakan gateway**: Claude Code membaca ulang ketika gateway Anda mereset koneksi atau menolak handshake TLS setelah berhenti menerima pasangan lama. Claude Code tidak membaca ulang ketika gateway menyelesaikan handshake dan menjawab dengan kesalahan HTTP. Dalam hal ini, Claude Code memuat pasangan baru ketika berikutnya menerapkan pengaturan atau ketika Anda melakukan restart.
* **Rotasi setengah tertulis**: ketika Claude Code membaca ulang saat rotasi Anda sedang dalam proses penulisan, seperti membaca sertifikat dan kunci yang tidak cocok satu sama lain, Claude Code tetap menggunakan pasangan sebelumnya dan membaca ulang pada kegagalan berikutnya.
* **Pengekspor telemetri OTLP**: Claude Code menyimpan sertifikat yang [pengekspor](/docs/id/monitoring-usage#mtls-authentication) dimuat pada penggunaan pertama, jadi restart Claude Code untuk sertifikat yang dirotasi agar dapat menjangkau kolektor telemetri Anda.
* **Matikan pemuatan ulang**: atur [`CLAUDE_CODE_DISABLE_MTLS_RELOAD_ON_STALE_CONNECTION=1`](/docs/id/env-vars#variables) untuk mematikan pembacaan ulang kesalahan koneksi. Claude Code kemudian mengambil file yang dirotasi hanya ketika berikutnya menerapkan pengaturan atau pada startup berikutnya.

Untuk mengonfirmasi Claude Code mengambil rotasi, [mulai sesi dengan pencatatan debug](#verify-your-configuration) dan cari `Stale connection — reloaded rotated mTLS client material` dalam log. Claude Code tidak mencatat baris ini ketika mengambil rotasi saat menerapkan pengaturan, jadi baris yang hilang saja tidak berarti rotasi gagal.

Ganti file sebelum pasangan saat ini kedaluwarsa sehingga Claude Code tidak memuat pasangan yang sudah kedaluwarsa pada startup berikutnya.

Dalam [sesi cloud](/docs/id/claude-code-on-the-web), lingkungan hosting mengelola koneksi ke API, jadi Claude Code mengabaikan variabel berikut ketika berasal dari blok `env` file pengaturan:

* `CLAUDE_CODE_CLIENT_CERT`
* `CLAUDE_CODE_CLIENT_KEY`
* `CLAUDE_CODE_CLIENT_KEY_PASSPHRASE`
* `NODE_EXTRA_CA_CERTS`
* `NODE_TLS_REJECT_UNAUTHORIZED`
* `CLAUDE_CODE_OAUTH_SCOPES`

Claude Code mencatat setiap kunci yang diabaikan dalam log debug sesi.

Dalam sesi [Claude Desktop](/docs/id/desktop) di mana aplikasi mengelola koneksi penyedia, seperti tab Code pada [penyedia pihak ketiga](/docs/id/third-party-integrations) dan sesi Cowork, Claude Code membaca variabel ini dan variabel proxy `HTTP_PROXY`, `HTTPS_PROXY`, dan `NO_PROXY` hanya dari [pengaturan terkelola](/docs/id/managed-settings) dan `~/.claude/settings.json`: Claude Code mengabaikannya dalam file pengaturan repositori sendiri, jadi repositori yang diperiksa tidak dapat mengalihkan jalur TLS atau proxy dari sesi yang kredensialnya berasal dari aplikasi. Dalam sesi tab Code lokal, SSH, atau WSL yang masuk melalui claude.ai, aplikasi tidak mengelola koneksi, dan Claude Code membaca variabel ini dari setiap cakupan pengaturan, seperti sesi terminal apa pun; [sesi cloud](/docs/id/claude-code-on-the-web) mengikuti aturan sesi cloud di atas di mana pun Anda memulainya. Sebelum v2.1.217, Claude Code mengabaikan variabel ini dalam setiap file pengaturan ketika aplikasi mengelola koneksi.

<h2 id="verify-your-configuration">
  Verifikasi konfigurasi Anda
</h2>

Biasanya Anda mengetahui tentang alamat proxy yang salah atau jalur sertifikat yang buruk dari [kesalahan koneksi atau sertifikat](/docs/id/errors#network-and-connection-errors) pada permintaan yang lebih lambat, karena Claude Code tidak memvalidasi sebagian besar pengaturan ini saat membacanya. Satu-satunya pengaturan yang diperiksa saat startup adalah URL proxy: ketika tidak dapat mengurai nilainya, seperti yang hilang skema `http://`, Claude Code menghentikan peluncuran dengan kesalahan yang menamai variabel untuk diperbaiki.

Untuk mengonfirmasi konfigurasi Anda dimuat sebelum Anda mengirim permintaan, mulai Claude Code dengan pencatatan debug:

```bash theme={null}
claude --debug
```

Output debug masuk ke `~/.claude/debug/<session-id>.txt` daripada terminal, atau ke jalur yang Anda atur dengan `--debug-file <path>`. Dalam log, cari baris yang mengonfirmasi setiap file dimuat:

```text theme={null}
CA certs: Appended extra certificates from NODE_EXTRA_CA_CERTS (/etc/ssl/certs/corp-ca.pem)
mTLS: Loaded client certificate from CLAUDE_CODE_CLIENT_CERT
mTLS: Loaded client key from CLAUDE_CODE_CLIENT_KEY
```

Jika Claude Code tidak dapat membaca salah satu file ini, log menunjukkan baris `Failed to read` atau `Failed to load` dengan alasan sebagai gantinya.

Anda juga dapat menjalankan `/status` dalam sesi interaktif dan memeriksa baris-baris ini:

* **Proxy**: menampilkan URL proxy aktif, dan menandai nilai yang tidak dapat diurakannya sebagai tidak valid dan diabaikan.
* **mTLS client cert** dan **mTLS client key**: muncul hanya ketika file dimuat, jadi baris yang hilang berarti pemuatan gagal dan log debug memiliki alasannya.
* **Additional CA cert(s)**: menampilkan jalur `NODE_EXTRA_CA_CERTS` tanpa memeriksa bahwa file dimuat, jadi konfirmasi yang satu ini dalam log debug.

<h2 id="apply-network-settings-to-background-agents">
  Terapkan pengaturan jaringan ke agen latar belakang
</h2>

[Agen latar belakang](/docs/id/agent-view) tidak berjalan di dalam terminal yang mengirimnya. Proses supervisor per-pengguna dimulai sesuai permintaan, bertahan lebih lama dari shell Anda, dan menghosting setiap sesi `claude agents`, `--bg`, dan `/background`. Lihat [Bagaimana sesi latar belakang dihosting](/docs/id/agent-view#how-background-sessions-are-hosted). Ini mengubah cara konfigurasi di halaman ini mencapai sesi-sesi tersebut.

<h3 id="set-network-variables-in-settings-not-the-shell">
  Atur variabel jaringan dalam pengaturan, bukan shell
</h3>

Supervisor adalah satu proses yang dibagikan oleh setiap terminal. Supervisor mewarisi lingkungan dari shell mana pun yang memulainya terlebih dahulu, dan supervisor yang diinstal OS tidak menerima lingkungan shell sama sekali. Jika Anda mengekspor proxy, jalur CA, atau variabel mTLS hanya di shell Anda, variabel tersebut mencapai agen latar belakang ketika shell itu kebetulan cold-start supervisor, dan diam-diam tidak mencapai ketika shell yang berbeda melakukannya.

Letakkan variabel yang sama dalam blok `env` dari `~/.claude/settings.json` atau [pengaturan terkelola](/docs/id/settings) sebagai gantinya. Setiap variabel di halaman ini dapat diatur di sana, dan pengaturan adalah satu-satunya konfigurasi yang mencapai setiap sesi latar belakang di setiap mesin.

<h3 id="configure-a-corporate-launcher-as-a-setting">
  Konfigurasikan peluncur perusahaan sebagai pengaturan
</h3>

Beberapa organisasi memerlukan setiap proses Claude Code dimulai melalui peluncur perusahaan yang menerapkan sandboxing, kontrol jaringan, atau injeksi kredensial. Supervisor dan pekerja-pekerjanya memulai Claude Code dari jalur tetap daripada mencari `claude` di `PATH`, sehingga setiap agen latar belakang melewati wrapper yang Anda tempatkan lebih awal di `PATH`.

Atur pengaturan [`processWrapper`](/docs/id/settings-reference#processwrapper) untuk menambahkan awalan supervisor, pekerja-pekerjanya, dan proses latar belakang lainnya yang tercantum di bawah [Apa yang dicakup peluncur](/docs/id/corporate-launcher#what-the-launcher-covers) dengan peluncur Anda. Variabel lingkungan [`CLAUDE_CODE_PROCESS_WRAPPER`](/docs/id/env-vars) yang setara mengambil prioritas ketika keduanya diatur, dan variabel tersebut tunduk pada aturan yang sama: berikan melalui pengaturan terkelola atau `~/.claude/settings.json`, bukan ekspor shell. [Jalankan Claude Code di balik peluncur perusahaan](/docs/id/corporate-launcher) mencakup kontrak yang harus dipenuhi peluncur, apa yang dilakukan dan tidak dilakukan, dan cara menerapkannya.

<Note>
  Supervisor yang sudah berjalan menyimpan konfigurasi peluncuran yang dimulainya. Setelah menerapkan pengaturan peluncur, jalankan [`claude daemon stop --any`](/docs/id/agent-view#the-supervisor-process) sehingga `claude agents` atau `--bg` berikutnya memulai supervisor yang menghormatinya. Layanan yang diinstal memerlukan `claude daemon stop` tanpa `--any`.
</Note>

<h2 id="streaming-idle-watchdogs">
  Watchdog idle streaming
</h2>

Claude Code menjalankan empat timer independen yang menghentikan respons model streaming ketika diam, sehingga koneksi yang mati gagal dan mencoba ulang alih-alih menggantung. Tenggat waktu byte pertama mencakup penantian header respons, sebelum ada respons yang tiba. Masing-masing dari tiga lainnya memantau respons langsung untuk sinyal yang berbeda.

| Timer                | Menghentikan ketika                                                                                                                                                                                                    | Berjalan pada                                                                                                                                                                                                                                                                                                                                                                         | Tenggat waktu default                                                                                        |
| :------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | :----------------------------------------------------------------------------------------------------------- |
| First-byte deadline  | Tidak ada header respons yang tiba setelah Claude Code mengirim permintaan                                                                                                                                             | Direct Anthropic API dan [Claude Platform on AWS](/docs/id/claude-platform-on-aws), termasuk melalui proxy HTTPS, tetapi tidak ketika `ANTHROPIC_BASE_URL` atau `ANTHROPIC_AWS_BASE_URL` merutekannya melalui [gateway](/docs/id/gateways). Opt-in pada Amazon Bedrock dengan `CLAUDE_ENABLE_BYTE_WATCHDOG_BEDROCK=1`; tidak berjalan pada Google Cloud's Agent Platform atau Microsoft Foundry | 180 detik pada direct Anthropic API, 300 detik di tempat lain, ditambah satu detik per 32KB badan permintaan |
| Event-level watchdog | Tidak ada event respons yang diurai. Pada koneksi di mana byte-level watchdog berjalan, byte yang tiba, termasuk keep-alive pings, juga mengatur ulang watchdog ini, selama sekitar lima menit tanpa event yang diurai | Setiap penyedia                                                                                                                                                                                                                                                                                                                                                                       | 300 detik                                                                                                    |
| Byte-level watchdog  | Tidak ada byte yang tiba di wire, termasuk SSE keep-alive pings                                                                                                                                                        | Direct Anthropic API, [Claude Platform on AWS](/docs/id/claude-platform-on-aws), dan [gateway](/docs/id/gateways) koneksi, termasuk `ANTHROPIC_BASE_URL` kustom. Opt-in pada Amazon Bedrock `vnd.amazon.eventstream` respons dengan `CLAUDE_ENABLE_BYTE_WATCHDOG_BEDROCK=1`; tidak berjalan pada Google Cloud's Agent Platform atau Microsoft Foundry                                           | 180 detik pada direct Anthropic API, 300 detik di tempat lain                                                |
| Body idle timeout    | Tidak ada byte yang tiba selama 5 menit                                                                                                                                                                                | Penyedia selain direct Anthropic API dan Claude Platform on AWS, kecuali [`API_FORCE_IDLE_TIMEOUT`](/docs/id/env-vars) mengubahnya                                                                                                                                                                                                                                                         | 5 menit                                                                                                      |

Konfigurasikan timer dengan variabel-variabel ini, masing-masing dirinci dalam [referensi variabel lingkungan](/docs/id/env-vars):

* `CLAUDE_ENABLE_STREAM_WATCHDOG` dan `CLAUDE_ENABLE_BYTE_WATCHDOG` memaksa watchdog yang sesuai dengan `1` atau mematikan dengan `0`, dalam koneksi yang tabel sebutkan; tidak ada variabel yang memperluas watchdog ke tipe koneksi yang tidak dicakupnya. `CLAUDE_ENABLE_BYTE_WATCHDOG` diatur ke `0` juga mematikan first-byte deadline.
* `CLAUDE_STREAM_IDLE_TIMEOUT_MS` menetapkan tenggat waktu kedua watchdog. Claude Code menaikkan nilai di bawah 5 menit menjadi 5 menit, dan membatasi nilai pada 30 menit untuk byte-level watchdog.
* `CLAUDE_BYTE_STREAM_IDLE_TIMEOUT_MS` menetapkan tenggat waktu byte-level watchdog tanpa mengubah event-level watchdog, diklem antara 10 detik dan 30 menit, dan mengambil prioritas atas `CLAUDE_STREAM_IDLE_TIMEOUT_MS` untuk watchdog itu.
* `CLAUDE_STREAM_FIRST_BYTE_TIMEOUT_MS` menetapkan first-byte deadline secara langsung. Biarkan tidak diatur dan Claude Code menggunakan tenggat waktu byte-level watchdog, sehingga `CLAUDE_STREAM_IDLE_TIMEOUT_MS` dan `CLAUDE_BYTE_STREAM_IDLE_TIMEOUT_MS` juga mengubah deadline. Untuk klem, tunjangan unggah, batas `API_TIMEOUT_MS`, dan berapa lama percobaan ulang menunggu setelah penghentian tanpa respons, lihat [No response from API](/docs/id/errors#no-response-from-api).
* `API_FORCE_IDLE_TIMEOUT` diatur ke `0` mematikan body idle timeout, dan diatur ke `1` menghidupkannya untuk setiap penyedia. Watchdog berjalan independen darinya, jadi untuk membiarkan stream berhenti lebih lama dari ambang batas mereka, juga naikkan atau nonaktifkan mereka.

Ketika watchdog menghentikan stream yang macet, Claude Code memperlakukan penghentian sebagai kegagalan mid-stream, dan apa yang Anda lihat tergantung pada seberapa jauh respons telah sampai. Claude Code mencoba ulang permintaan atau mengakhiri giliran dengan kesalahan, menyimpan output yang selesai dan menampilkan [pemberitahuan respons tidak lengkap](/docs/id/errors#the-response-above-may-be-incomplete), atau mengakhiri giliran secara normal. [Percobaan ulang otomatis](/docs/id/errors#automatic-retries) mengatakan di mana setiap hasil berlaku.

Dalam [sesi non-interaktif](/docs/id/headless), dan untuk respons subagent dalam sesi apa pun, Claude Code mungkin terlebih dahulu meminta Claude untuk melanjutkan respons yang dipotong; [entri pemberitahuan itu](/docs/id/errors#the-response-above-may-be-incomplete) mengatakan kapan itu terjadi dan kapan Anda masih melihat pemberitahuan.

Ketika first-byte deadline menyala, tidak ada respons yang telah dimulai, jadi tidak ada output parsial untuk disimpan. Untuk cara Claude Code mengirim ulang permintaan dan kapan giliran berakhir, lihat [No response from API](/docs/id/errors#no-response-from-api).

<h2 id="network-access-requirements">
  Persyaratan akses jaringan
</h2>

Claude Code memerlukan akses ke URL berikut. Daftarkan URL ini dalam konfigurasi proxy dan aturan firewall Anda, terutama di lingkungan jaringan terkontainer atau terbatas. Pemeriksaan konektivitas pengaturan pertama kali menunjuk ke sini ketika tidak dapat menjangkau `api.anthropic.com` atau `platform.claude.com`; lihat [Unable to connect to Anthropic services](/docs/id/errors#unable-to-connect-to-anthropic-services) untuk pesan pemeriksaan dan langkah pemulihan.

| URL                                  | Diperlukan untuk                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| ------------------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `api.anthropic.com`                  | Permintaan Claude API, termasuk pemeriksaan keamanan domain WebFetch [domain safety check](/docs/id/data-usage#webfetch-domain-safety-check), pengambilan bendera fitur, dan pencatatan acara telemetri                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `claude.ai`                          | Autentikasi akun claude.ai                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `claude.com`                         | Masuk akun claude.ai membuka halaman `claude.com` di browser, yang dialihkan ke `claude.ai`; pencarian dokumentasi WebFetch yang telah disetujui sebelumnya juga menjangkau host ini dari CLI                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `platform.claude.com`                | Autentikasi akun Anthropic Console. Pertukaran token OAuth, penyegaran, dan pencabutan juga menuju host ini untuk akun claude.ai, jadi masuk Console dan claude.ai memerlukan keduanya                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `mcp-proxy.anthropic.com`            | [MCP connectors dari claude.ai](/docs/id/mcp#use-mcp-servers-from-claude-ai), termasuk konektor yang dikonfigurasi administrator organisasi. Lalu lintas konektor merutekan melalui proxy ini; konektor diaktifkan secara default untuk pengguna yang diautentikasi claude.ai. Untuk menghentikan Claude Code dari mengambilnya, atur [`ENABLE_CLAUDEAI_MCP_SERVERS=false`](/docs/id/env-vars) atau pengaturan [`disableClaudeAiConnectors`](/docs/id/settings-reference#disableclaudeaiconnectors)                                                                                                                                   |
| `downloads.claude.ai`                | Unduhan executable plugin; installer asli, auto-updater asli, dan pemeriksaan versi pembaruan                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `storage.googleapis.com`             | Jumlah instalasi plugin dan metadata yang ditampilkan di `/plugin`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `storage.googleapis.com`             | Installer asli dan auto-updater asli pada versi sebelum 2.1.116                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `registry.npmjs.org`                 | Instalasi plugin (pengambilan paket plugin bersumber npm dan instalasi dependensi paket Node.js plugin), server MCP yang diluncurkan `npx`, dan registri paket untuk instalasi npm dan bun dari Claude Code itu sendiri                                                                                                                                                                                                                                                                                                                                                                                                |
| `bridge.claudeusercontent.com`       | [Claude in Chrome](/docs/id/chrome) jembatan WebSocket ekstensi                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `*.frame.claudeusercontent.com`      | Pembacaan konten [Artifact](/docs/id/artifacts). CLI mengambil file artefak dari host ini ketika Claude membuka satu, dan hanya ketika alat Artifact [tersedia](/docs/id/artifacts#availability) untuk akun Anda. Untuk mematikan alat dan menghilangkan persyaratan ini, atur [`"enableArtifact": false`](/docs/id/settings-reference#enableartifact) atau [`CLAUDE_CODE_DISABLE_ARTIFACT=1`](/docs/id/env-vars); Claude Code juga menghormati pengaturan [`disableArtifact`](/docs/id/settings-reference#disableartifact) yang sudah usang. Lihat [Disable artifacts](/docs/id/artifacts#disable-artifacts) untuk cara pengaturan ini berinteraksi |
| `github.com`                         | Mengkloning [plugin marketplaces](/docs/id/plugins/overview) dan plugin yang dihosting GitHub, termasuk marketplace resmi Anthropic, melalui HTTPS atau SSH. Untuk mengkloning sumber GitHub `owner/repo` hanya melalui HTTPS, atur [`CLAUDE_CODE_PLUGIN_PREFER_HTTPS=1`](/docs/id/env-vars)                                                                                                                                                                                                                                                                                                                                     |
| `raw.githubusercontent.com`          | Umpan changelog untuk [`/release-notes`](/docs/id/commands). Dalam sesi interaktif, Claude Code juga mengambilnya di latar belakang saat startup ketika changelog cache-nya belum mencakup versi yang berjalan, seperti awal pertama setelah pembaruan; sesi non-interaktif dan cloud tidak pernah mengambilnya                                                                                                                                                                                                                                                                                                             |
| `*-review.googlesource.com`          | Pencarian perubahan Gerrit pada checkout `googlesource.com`. Ketika sesi tab Claude Desktop Code dimulai atau dilanjutkan pada checkout [terpercaya](/docs/id/permissions#project-allow-rules-and-workspace-trust) yang `origin`-nya adalah host `googlesource.com`, Claude Code menanyakan server `-review` host itu secara anonim untuk perubahan terbuka yang cocok dengan `Change-Id` HEAD, sekali per awal atau lanjutan. Jenis sesi lain melewati pencarian, dan tidak ada host Gerrit lain yang dihubungi. Opsional: nonaktifkan dengan [`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`](/docs/id/env-vars)                   |
| `http-intake.logs.us5.datadoghq.com` | Acara telemetri operasional, dikirim hanya ketika CLI menggunakan Anthropic API secara langsung, tidak pernah untuk Amazon Bedrock, Agent Platform Google Cloud, atau Microsoft Foundry. Opsional: nonaktifkan dengan [`DISABLE_TELEMETRY`](/docs/id/data-usage#telemetry-services) atau `DO_NOT_TRACK`                                                                                                                                                                                                                                                                                                                     |
| `browser-intake-us5-datadoghq.com`   | Laporan kesalahan operasional, dikirim ketika CLI menggunakan Anthropic API secara langsung dan gerbang peluncuran sisi server mengaktifkannya. Opsional: nonaktifkan dengan `DISABLE_ERROR_REPORTING` atau `DISABLE_TELEMETRY`; lihat [Telemetry services](/docs/id/data-usage#telemetry-services)                                                                                                                                                                                                                                                                                                                         |
| `formulae.brew.sh`                   | Pemeriksaan versi pembaruan pada instalasi Homebrew. Metode instalasi lain tidak menghubungi host ini                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `code.claude.com`                    | Pencarian dokumentasi Claude Code oleh agen claude-code-guide bawaan dan permintaan WebFetch yang telah disetujui sebelumnya. Memblokir host ini hanya mempengaruhi pencarian dokumentasi                                                                                                                                                                                                                                                                                                                                                                                                                              |

Jika Anda menginstal Claude Code melalui npm atau mengelola distribusi biner Anda sendiri, pengguna akhir tidak memerlukan installer asli dan penggunaan auto-updater dari `downloads.claude.ai`, tetapi instalasi npm dan bun memerlukan registri paket mereka, `registry.npmjs.org`, kecuali organisasi Anda mencerminkannya. Penggunaan lain dalam tabel berlaku terlepas dari metode instalasi.

Dua host intake Datadog hanya membawa telemetri operasional opsional, dan pengaturan [`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`](/docs/id/env-vars) menonaktifkan keduanya. Sesi pada penyedia pihak ketiga tidak pernah mengirim ke host ini, bahkan ketika platform menetapkan [`CLAUDE_CODE_PROVIDER_MANAGED_BY_HOST`](/docs/id/env-vars) dan metrik telemetri default aktif. Lihat [Telemetry services](/docs/id/data-usage#telemetry-services) untuk semua yang Claude Code kirim dan cara menonaktifkannya sebelum menyelesaikan daftar putih Anda.

Saat menggunakan [Amazon Bedrock](/docs/id/amazon-bedrock), [Agent Platform Google Cloud](/docs/id/google-vertex-ai), [Microsoft Foundry](/docs/id/microsoft-foundry), atau sesi [Claude apps gateway](/docs/id/claude-apps-gateway) yang masuk, lalu lintas model dan autentikasi menuju penyedia atau gateway Anda alih-alih `api.anthropic.com`, `claude.ai`, atau `platform.claude.com`. Alat WebFetch masih memanggil `api.anthropic.com` untuk [domain safety check](/docs/id/data-usage#webfetch-domain-safety-check) kecuali Anda menetapkan `skipWebFetchPreflight: true` dalam [settings](/docs/id/settings).

Saat merutekan melalui [LLM gateway](/docs/id/llm-gateway) dengan [`ANTHROPIC_BASE_URL`](/docs/id/llm-gateway-connect#set-the-base-url-and-credential), pemeriksaan ketersediaan [fast mode](/docs/id/fast-mode) masih memanggil `api.anthropic.com` daripada URL dasar gateway. Pemeriksaan menghormati proxy HTTP yang dikonfigurasi, jadi di mana blokir jaringan adalah penyebabnya, entri daftar putih untuk `api.anthropic.com` dalam proxy adalah perbaikannya. Blokir jaringan hanya gagal pemeriksaan di mana host tidak dapat dijangkau bahkan melalui proxy, dan fast mode kemudian melaporkan kesalahan konektivitas. Kesalahan konektivitas yang sama muncul ketika pemeriksaan menyajikan kredensial yang dikeluarkan gateway yang ditolak Anthropic; daftar putih tidak membantu di sana, karena tidak ada yang diblokir. Lihat [use fast mode behind proxies and LLM gateways](/docs/id/fast-mode#use-fast-mode-behind-proxies-and-llm-gateways) untuk variabel yang memulihkannya.

<h3 id="organization-ip-allowlists-and-proxy-egress">
  Daftar putih IP organisasi dan egress proxy
</h3>

Jika organisasi Anda memiliki [IP allowlisting](https://support.claude.com/en/articles/13200993-restrict-access-to-claude-with-ip-allowlisting) diaktifkan untuk Claude, rutekan `bridge.claudeusercontent.com` melalui egress proxy yang sama dengan `claude.ai` dan `api.anthropic.com`, misalnya dengan menempatkannya dalam segmen aplikasi Zscaler yang sama atau kebijakan steering Netskope. Jika Anda tidak dapat meroutekannya dengan cara itu, tambahkan alamat egress yang digunakan proxy Anda untuk host itu ke daftar putih IP organisasi Anda, tetapi hanya ketika alamat itu didedikasikan untuk organisasi Anda: rentang egress proxy bersama juga mengakui pelanggan lain dari vendor proxy.

Anthropic memeriksa koneksi ke `bridge.claudeusercontent.com` terhadap daftar putih IP organisasi Anda menggunakan alamat yang mereka tiba dari. Jika proxy Anda mengirim lalu lintas untuk host itu melalui alamat yang tidak ada di daftar putih itu, Claude Code tidak dapat terhubung ke ekstensi [Claude in Chrome](/docs/id/chrome) meskipun sisa Claude Code berfungsi.

<h3 id="github-allow-lists-and-firewalls">
  Daftar putih GitHub dan firewall
</h3>

[Cloud sessions](/docs/id/claude-code-on-the-web) di lingkungan yang dihosting Anthropic dan [Code Review](/docs/id/code-review) terhubung ke repositori Anda dari infrastruktur yang dikelola Anthropic; sesi di [lingkungan yang dihosting sendiri](/docs/id/self-hosted-environments) terhubung dari dalam jaringan Anda, kecuali runner memilih [Anthropic git proxy](/docs/id/self-hosted-environments-deploy#use-the-anthropic-git-proxy), yang mengambil dari sisi Anthropic.

Jika organisasi GitHub Enterprise Cloud Anda membatasi akses berdasarkan alamat IP, aktifkan [IP allow list inheritance untuk GitHub Apps yang diinstal](https://docs.github.com/en/enterprise-cloud@latest/organizations/keeping-your-organization-secure/managing-security-settings-for-your-organization/managing-allowed-ip-addresses-for-your-organization#allowing-access-by-github-apps) dan juga [tambahkan entri daftar putih](https://docs.github.com/en/enterprise-cloud@latest/organizations/keeping-your-organization-secure/managing-security-settings-for-your-organization/managing-allowed-ip-addresses-for-your-organization#adding-an-allowed-ip-address) untuk [alamat IP keluar](https://platform.claude.com/docs/en/api/ip-addresses#outbound-ip-addresses) Anthropic. Warisan mencakup hanya permintaan yang dibuat Claude GitHub App sebagai instalasi, bukan permintaan yang dibuat atas nama pengguna Anda. Untuk firewall lain, lihat [Anthropic API IP addresses](https://platform.claude.com/docs/en/api/ip-addresses).

Untuk instans [GitHub Enterprise Server](/docs/id/github-enterprise-server) yang dihosting sendiri di belakang firewall, daftarkan putih [alamat IP keluar](https://platform.claude.com/docs/en/api/ip-addresses#outbound-ip-addresses) Anthropic sehingga infrastruktur Anthropic dapat menjangkau host GHES Anda untuk mengkloning repositori dan memposting komentar ulasan. Sesi di [lingkungan yang dihosting sendiri](/docs/id/self-hosted-environments-deploy#configure-git) menjangkau host GHES Anda dari dalam jaringan Anda sebagai gantinya, jadi paparan itu hanya berlaku untuk sesi yang dihosting Anthropic, untuk alur pra-sesi yang dihosting seperti pemilih repositori, dan untuk runner yang dihosting sendiri yang memilih [Anthropic git proxy](/docs/id/self-hosted-environments-deploy#use-the-anthropic-git-proxy), yang mengambil dari sisi Anthropic. Untuk host GHES yang hanya dapat dirutekan di dalam jaringan Anda, [SCM connector](/docs/id/self-hosted-environments-reference#scm-connector-flags) membawa alur pra-sesi yang dihosting melalui koneksi keluar sebagai gantinya, jadi daftar putih tidak diperlukan untuk mereka.

<h3 id="desktop-and-claude-ai">
  Desktop dan claude.ai
</h3>

Tabel sebelumnya mencakup CLI mandiri. Aplikasi Claude Desktop dan claude.ai di browser memuat kode aplikasi dan konten pengguna mereka dari host CDN Anthropic tambahan, termasuk `assets-proxy.anthropic.com` dan asal `*.claudeusercontent.com` lainnya yang melayani [artifacts](/docs/id/artifacts) di aplikasi tersebut. Mengizinkan `claude.ai` sambil memblokir host tersebut menghasilkan halaman kosong daripada kesalahan. Lihat [network access requirements](/docs/id/desktop#network-access-requirements) di halaman Desktop.

[Artifact](/docs/id/artifacts) yang memuat typeface dari [Google Fonts](/docs/id/artifacts#improve-the-visual-design) juga meminta `fonts.googleapis.com` dan `fonts.gstatic.com`. Kedua host bersifat opsional. Jika Anda memblokir mereka, artifact merender dalam typeface fallback. Blokir dengan penolakan cepat daripada penjatuhan senyap sehingga permintaan font gagal segera daripada menunda render pertama halaman.

Artifact juga dapat memuat pustaka JavaScript, seperti React atau paket charting, dari `cdnjs.cloudflare.com`, `cdn.jsdelivr.net`, `cdn.tailwindcss.com`, `code.jquery.com`, dan `unpkg.com`, dan dari tidak ada host eksternal lainnya. Jika Anda memblokir host tersebut, bagian dari artifact yang bergantung pada pustaka tidak berfungsi, dan tidak seperti font yang diblokir, pustaka yang diblokir tidak memiliki fallback. Blokir dengan penolakan cepat di sini juga, sehingga permintaan pustaka yang diblokir gagal sekaligus daripada menggantung sampai waktu habis.

<h2 id="additional-resources">
  Sumber daya tambahan
</h2>

* [File pengaturan dan urutan prioritas](/docs/id/settings)
* [Referensi variabel lingkungan](/docs/id/env-vars)
* [Panduan pemecahan masalah](/docs/id/troubleshooting)
