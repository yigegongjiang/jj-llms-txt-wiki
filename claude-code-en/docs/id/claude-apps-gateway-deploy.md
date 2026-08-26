> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Penyebaran dan operasi gateway aplikasi Claude

> Daftarkan gateway dengan IdP Anda, bangun kontainer, sebarkan di Kubernetes atau Cloud Run, dan operasikan: pemeriksaan kesehatan, rotasi rahasia, peningkatan, dan keamanan.

Halaman ini mencakup sisi operasional menjalankan [gateway aplikasi Claude](/docs/id/claude-apps-gateway): mendaftarkan klien OAuth di penyedia identitas (IdP) Anda, menyebarkan gateway sebagai kontainer, dan menjalankannya sehari-hari. Untuk setiap opsi dalam file `gateway.yaml` yang dibaca gateway saat boot, lihat [Referensi Konfigurasi](/docs/id/claude-apps-gateway-config).

Penyebaran produksi mengikuti empat langkah secara berurutan, dan bagian di bawah cocok dengan mereka. Dua yang pertama adalah tempat Anda membuat pilihan; dua yang kedua adalah materi referensi untuk dikonsultasikan setelah berjalan.

1. [Siapkan penyedia identitas Anda](#identity-provider-setup): daftarkan klien OAuth dan periksa catatan per-IdP untuk Okta, Entra, dan Google
2. [Sebarkan gateway](#deployment): bangun gambar kontainer yang disematkan dan jalankan di Kubernetes, Cloud Run, atau platform Anda sendiri. Bagian ini juga mencakup keputusan biaya, bypass, gateway-ganda, dan serverless
3. [Siapkan operasi](#operations): log, probe kesehatan, perilaku pemadaman, rotasi rahasia, dan peningkatan. Referensi untuk ketika Anda menghubungkan pemantauan dan runbook
4. [Tinjau postur keamanan](#security): aliran data ke mana, model ancaman, dan jawaban kepatuhan. Referensi untuk tinjauan keamanan

Jika masuk atau boot gagal di sepanjang jalan, langsung ke [Troubleshooting](#troubleshooting), yang dikunci pada kesalahan yang Anda lihat.

<Note>
  **Sebarkan di jaringan pribadi Anda.** Claude Code hanya terhubung ke gateway yang alamatnya pribadi. Ini adalah penjaga keamanan, karena gateway yang dipercaya dapat mendorong pengaturan yang menjalankan perintah pada mesin pengembang. Letakkan gateway yang Anda sebarkan di belakang penyeimbang beban internal atau VPN dan berikan nama host yang hanya diselesaikan ke IP pribadi.

  Titik akhir gateway publik yang dioperasikan Anthropic adalah pengecualian: `/login` menerimanya melalui `https://`. Ini adalah kumpulan tetap kecil dari gateway yang dioperasikan Anthropic sendiri; mereka bukan opsi penyebaran yang dapat Anda pilih atau konfigurasi. Daftar dikompilasi ke dalam Claude Code, jadi tidak ada konfigurasi yang dapat menambahkan nama host ke dalamnya dan tidak ada gateway yang Anda hosting yang memenuhi syarat untuk pengecualian. Sebelum v2.1.206, `/login` menolak titik akhir tersebut seperti alamat publik lainnya.
</Note>

<h2 id="identity-provider-setup">
  Penyiapan penyedia identitas
</h2>

Daftarkan aplikasi web OAuth/OpenID Connect (OIDC) rahasia dengan URI pengalihan tunggal, `https://<gateway>/oauth/callback`, dan tetapkan ke pengguna atau grup yang harus memiliki akses gateway.

IdP apa pun yang sesuai dengan OIDC berfungsi: Okta, Microsoft Entra ID, Google Workspace, Keycloak, Dex, PingFederate, dan lainnya. IdP harus memenuhi tiga persyaratan:

* Melayani `/.well-known/openid-configuration`, melalui HTTPS dalam produksi; gateway menerima [`http://` issuer](/docs/id/claude-apps-gateway-config#oidc), dan issuer loopback juga memerlukan `CLAUDE_GATEWAY_ALLOW_LOOPBACK=1`
* Mendukung aliran kode otorisasi. PKCE (Proof Key for Code Exchange) aktif secara default; nonaktifkan dengan `oidc.use_pkce: false` untuk IdP yang tidak mendukungnya
* Mengembalikan `email` dan secara opsional `groups` dalam id\_token, atau melayaninya dari endpoint userinfo dengan `oidc.userinfo_fallback: true`

Untuk PKI pribadi, atur `oidc.ca_cert_pem`.

Beberapa penyedia menangani klaim email dan grup secara berbeda:

* **Okta**: server otorisasi org di `https://example.okta.com` mengembalikan id\_token tipis yang menghilangkan `email` dan `groups`, jadi atur `oidc.userinfo_fallback: true` kapan pun Anda menggunakannya sebagai `issuer`. Server otorisasi khusus seperti `https://example.okta.com/oauth2/default` yang menyertakan `email` dan secara opsional `groups` dalam id\_token memancarkannya secara langsung dan tidak memerlukan fallback. Okta memancarkan `groups` hanya ketika scope `groups` diminta dalam `oidc.scopes` dan filter klaim grup aplikasi memungkinkannya; `userinfo_fallback` tidak dapat mengisi klaim yang IdP tidak diminta.
* **Microsoft Entra ID**: `issuer` = `https://login.microsoftonline.com/<tenant-id>/v2.0`. Entra memancarkan Object ID grup daripada nama, jadi gunakan GUID dalam `managed.policies.match.groups`, atau gunakan App Roles untuk nama yang dapat dibaca manusia. Jika penyewa Anda memancarkan peran di bawah `roles` bukan `groups`, atur `oidc.groups_claim: roles`.
* **Google Workspace**: `issuer` = `https://accounts.google.com`. id\_token Google tidak membawa grup. Untuk menggunakan `allowed_groups` berbasis grup atau `managed.policies` dengan Google sebagai IdP, konfigurasikan [`oidc.google_groups`](/docs/id/claude-apps-gateway-config#oidc), yang mencari grup setiap pengguna melalui Admin SDK Directory API menggunakan akun layanan dengan delegasi di seluruh domain. Tanpa itu, gunakan `oidc.allowed_email_domains` untuk gating keanggotaan dan `managed.policies.match.email_domain` untuk penugasan kebijakan. Google juga mengabaikan scope `offline_access` standar. Untuk token refresh, atur `oidc.scopes: [openid, profile, email]` dan `oidc.extra_auth_params: { access_type: offline, prompt: consent }`.

Untuk dukungan dengan penyedia identitas yang tidak tercakup di atas, lihat [Troubleshooting](#troubleshooting).

<Warning>
  Token refresh memungkinkan gateway untuk memperbarui sesi pengembang secara diam-diam, tanpa mengirim pengembang kembali ke browser. Mereka juga mendorong deprovisioning, karena ketika IdP menonaktifkan pengguna, refresh berikutnya gagal dan sesi berakhir dalam `ttl_hours`. Gateway meminta `offline_access` secara default untuk mendapatkan token refresh. Jika IdP Anda memerlukan persetujuan eksplisit untuk akses offline, konfigurasikan klien OAuth untuk memungkinkannya.

  Jika IdP Anda tidak dapat mengeluarkan token refresh sama sekali, gateway masih berfungsi, tetapi tidak ada pembaruan diam-diam, jadi pengembang menjalankan kembali login browser ketika sesi mereka berakhir. Untuk mencegah itu terjadi setiap jam, naikkan [`session.ttl_hours`](/docs/id/claude-apps-gateway-config#session) ke `8` atau `12`. Tradeoff adalah latensi deprovisioning, karena tanpa token refresh pengguna yang dinonaktifkan tetap memiliki akses sampai TTL yang lebih lama berlalu.
</Warning>

<h2 id="deployment">
  Penyebaran
</h2>

Gateway adalah satu biner Linux. Ini diskalakan secara horizontal karena replika tidak memiliki status dan Postgres adalah lapisan koordinasi bersama. Jalankan bagaimanapun Anda menjalankan layanan stateless di lingkungan Anda. Sisa bagian ini menyatakan apa yang dibutuhkan gambar, dengan catatan singkat untuk Kubernetes dan Cloud Run.

Gateway dirancang untuk berjalan di dalam jaringan Anda, karena menyimpan kredensial upstream Anda dan bertindak sebagai titik egress tunggal untuk inferensi. Ini dapat berjalan di mana pun pengembang Anda dan IdP Anda dapat menjangkau melalui HTTPS; perlakukan seperti layanan lain yang menyimpan kredensial produksi.

Beberapa keputusan membentuk penyebaran di luar tempat berjalan:

* **Biaya**: tidak ada lisensi terpisah atau biaya per-kursi untuk gateway; itu adalah bagian dari biner `claude`. Anda membayar untuk inferensi melalui komitmen cloud atau Anthropic yang ada, ditambah komputasi untuk kontainer dan kolektor telemetri Anda.
* **Bypass**: gateway tidak memberlakukan bahwa satu-satunya rute ke model melaluinya. Pengembang dengan kredensial mereka sendiri masih dapat memanggil penyedia secara langsung, jadi menutup jalur itu adalah keputusan kebijakan jaringan, misalnya memblokir egress ke `api.anthropic.com` kecuali dari gateway. Memblokir egress itu juga merusak [pemeriksaan keamanan domain WebFetch](/docs/id/data-usage#webfetch-domain-safety-check), yang memanggil `api.anthropic.com` dari mesin setiap pengembang; atur `skipWebFetchPreflight: true` dalam kebijakan terkelola untuk menonaktifkannya.
* **Multiple gateways**: setiap gateway adalah penyebaran terpisah dengan konfigurasinya sendiri. CLI menyimpan sidik jari kepercayaan dan kredensialnya per nama host gateway, jadi tim yang berbeda dapat terhubung ke gateway yang berbeda tanpa konflik. Untuk melayani beberapa issuer OIDC, jalankan instance terpisah.
* **Serverless**: Cloud Run berfungsi; atur `min-instances: 1` untuk menghindari penemuan OIDC dingin. Lambda dan Cloud Functions tidak, karena gateway adalah server HTTP yang berjalan lama.

Setiap topologi produksi di sini menempatkan proxy L7, seperti Ingress, frontend Cloud Run, atau ALB, di depan replika HTTP biasa. Atur [`listen.trusted_proxies`](/docs/id/claude-apps-gateway-config#listen) ke rentang sumber proxy sehingga gateway membaca IP klien dari `X-Forwarded-For`. Gateway menghormati header hanya ketika peer TCP dipercaya; [contoh yang dikerjakan Google Cloud](/docs/id/claude-apps-gateway-on-gcp) memiliki nilai konkret per topologi. Tanpa proxy terpercaya, setiap permintaan tampak berasal dari IP proxy, yang meruntuhkan batas laju per-IP menjadi satu bucket bersama dan mencatat IP proxy dalam acara audit.

<h3 id="container-image">
  Gambar kontainer
</h3>

Bangun gambar Anda sendiri di sekitar biner `claude` asli dari rilis Claude Code standar:

1. Unduh build Linux untuk arsitektur gambar Anda dari rilis yang disematkan; lihat [Instal versi spesifik](/docs/id/setup#install-a-specific-version) untuk URL unduhan.
2. Verifikasi terhadap `manifest.json` yang ditandatangani GPG rilis seperti yang dijelaskan dalam [Integritas biner dan penandatanganan kode](/docs/id/setup#binary-integrity-and-code-signing).
3. Salin ke konteks build.

Cerminkan rilis ke registri internal Anda jika build Anda tidak dapat menjangkau host rilis, dan sematkan versi yang dijalankan armada Anda.

Di luar biner, gambar membutuhkan:

* **Gambar berbasis glibc**: build glibc hanya memiliki dependensi dinamis perpustakaan glibc. Gambar berbasis Musl memerlukan build `linux-x64-musl` atau `linux-arm64-musl` ditambah paket tambahan; lihat [Penyiapan Alpine Linux](/docs/id/setup#alpine-linux-and-musl-based-distributions).
* **Direktori status yang dapat ditulis**: gateway berjalan sebagai pengguna apa pun, tetapi gambar minimal tidak memiliki rumah yang dapat ditulis. Atur `CLAUDE_CONFIG_DIR` ke jalur yang dapat ditulis seperti `/tmp/.claude`.
* **Perintah kontainer**: `claude gateway --config /etc/claude/gateway.yaml`, dengan file konfigurasi dipasang hanya-baca dan rahasia disuplai sebagai variabel lingkungan; gateway mendengarkan di `listen.port`, default `8080`.

<h3 id="kubernetes">
  Kubernetes
</h3>

Jalankan gateway sebagai Deployment, seperti layanan stateless apa pun:

* Pasang konfigurasi dari ConfigMap dan rahasia dari Secret; referensikan rahasia dalam YAML melalui `${file:/path/to/secret}` atau sebagai variabel lingkungan
* Hentikan TLS di Ingress dan atur `listen.public_url` ke nama host Ingress
* Arahkan probe kesiapan ke `GET /readyz` dan probe liveness ke `GET /healthz`

<Note>
  **Identitas beban kerja**

  Lebih suka identitas beban kerja platform daripada kunci statis: IRSA di EKS untuk Bedrock dan untuk Claude Platform di AWS, Workload Identity di GKE untuk Agent Platform, dan identitas beban kerja di AKS untuk Foundry. Atur `auth: {}` dalam blok upstream, atau `use_azure_ad: true` untuk Foundry, dan gateway mengambil identitas pod melalui rantai kredensial default penyedia itu. Untuk pasangan lintas cloud, seperti upstream Bedrock di GKE, atur kredensial eksplisit dalam blok `auth` upstream sebagai gantinya. Referensi [`upstreams`](/docs/id/claude-apps-gateway-config#upstreams) memiliki detail penyiapan per-platform.
</Note>

<h3 id="cloud-run">
  Cloud Run
</h3>

Konfigurasikan layanan sebagai berikut:

* Biarkan `listen.port` pada default `8080`, yang cocok dengan `PORT` default Cloud Run, atau atur `port: ${PORT}`
* Atur `public_url` ke asal yang dapat dijangkau secara eksternal. Untuk produksi ini biasanya nama host penyeimbang beban internal, karena `/login` [menolak alamat publik](/docs/id/claude-apps-gateway#prerequisites) dan URL `*.run.app` diselesaikan ke satu, jadi URL Cloud Run saja hanya berfungsi untuk uji coba `curl` atau browser. Pengecualiannya adalah jaringan di mana `*.run.app` diselesaikan secara pribadi melalui Private Service Connect dan zona pribadi Cloud DNS; dalam topologi itu URL Cloud Run adalah `public_url` yang valid. [Contoh yang dikerjakan Google Cloud](/docs/id/claude-apps-gateway-on-gcp#deploy-the-gateway) mencakup keduanya.
* Pasang konfigurasi sebagai volume rahasia
* Atur `min-instances: 1` untuk menghindari penemuan OIDC dingin pada permintaan pertama

<Note>
  Untuk contoh lengkap yang dikerjakan di Google Cloud, mencakup Cloud Run atau GKE, Cloud SQL, dan Secret Manager, lihat [Sebarkan di Google Cloud](/docs/id/claude-apps-gateway-on-gcp).
</Note>

<h3 id="push-the-gateway-url-to-developer-machines">
  Dorong URL gateway ke mesin pengembang
</h3>

Setelah gateway melayani, dorong `forceLoginMethod` dan `forceLoginGatewayUrl` ke mesin setiap pengembang melalui pengaturan terkelola, melalui MDM atau dengan menulis `managed-settings.json` per-OS secara langsung. Tanpa ini, `/login` menampilkan pemilih akun standar tanpa opsi gateway. Lihat [Pengaturan terkelola sisi klien](/docs/id/claude-apps-gateway-config#client-side-managed-settings) untuk jalur file.

<h2 id="operations">
  Operasi
</h2>

Setelah gateway melayani lalu lintas, operasi sehari-hari membaca lognya, menyelidiki kesehatannya, dan memutar rahasianya sesuai jadwal Anda. Subbagian mencakup masing-masing, ditambah apa yang disimpan Postgres dan bagaimana upgrade dan rollback berperilaku.

<h3 id="logs">
  Log
</h3>

Gateway menulis dua aliran ke stderr, keduanya ramah JSON:

* **Acara audit**: JSON satu baris per acara yang relevan dengan keamanan. Pipa stderr ke agregator log Anda. Acara yang dipancarkan termasuk `config.load`, `session.mint`, `session.refresh`, `device.authorize`, `device.verify`, `auth.denied`, `access.denied`, `inference`, `managed.serve`, `spend.blocked`, dan `admin.denied`. Bidang bervariasi menurut acara:
  * Acara mint dan refresh yang berhasil membawa `sub`, `email`, `client_ip`, dan hasilnya
  * Acara penolakan membawa alasan, jalur, dan IP klien, karena tidak ada identitas pada penolakan
  * `inference` mencatat upstream mana yang melayani permintaan dan status respons
  * `admin.denied` mencatat upaya autentikasi admin-API yang ditolak dengan alasan (`invalid_key` atau `no_credentials`), IP klien, metode, dan jalur, tanpa materi kunci yang disajikan
* **Log operasional**: baris yang dapat dibaca manusia dengan awalan `[gateway]` untuk boot, peringatan, dan kesalahan upstream. Variabel lingkungan `CLAUDE_GATEWAY_LOG_LEVEL` mengontrol verbositas dan menerima `info`, `warn`, atau `error`, dengan `info` sebagai default. Ini tidak mempengaruhi acara audit, yang selalu dipancarkan.

<h3 id="health">
  Kesehatan
</h3>

Gateway melayani `GET /healthz` sebagai probe liveness dan `GET /readyz` sebagai probe kesiapan; `/readyz` memverifikasi toko dapat dijangkau. Keduanya dikecualikan dari `access_control.allow_cidrs`, jadi probe terus bekerja pada pendengar yang terkunci.

Dokumen penemuan OAuth di `/.well-known/oauth-authorization-server` juga mengembalikan `200` hanya setelah pemuatan konfigurasi, penemuan OIDC, konstruksi klien upstream, dan migrasi Postgres semua berhasil, jadi berfungsi ganda sebagai pemeriksaan boot end-to-end.

Gateway yang berjalan juga melayani deskripsi jalur dan bentuk permintaan yang diterimanya di `<public_url>/protocol`, cocok dengan versi yang Anda jalankan. Isinya tidak stabil di seluruh rilis.

<h3 id="outage-behavior">
  Perilaku pemadaman
</h3>

Jika Postgres turun, gateway itu sendiri terus melayani pengembang yang masuk dan masuk baru gagal. Apakah pengembang benar-benar terus bekerja tergantung pada bagaimana orchestrator Anda menangani kesiapan:

* **Sesi yang ada**: token pembawa memvalidasi secara lokal dengan rahasia JWT, penyegaran sesi tidak menyentuh toko, dan proses gateway masih dapat melayani inferensi
* **Masuk baru**: gagal sampai Postgres pulih, karena aliran perangkat dan penghitung batas lajunya tinggal di Postgres
* **[Penegakan batas pengeluaran](/docs/id/claude-apps-gateway-spend-limits#postgres-availability)**: gagal terbuka secara default selama pemadaman, jadi inferensi masih mengalir; balikkan ke gagal tertutup jika Anda lebih suka memblokir daripada menjalankan tanpa meter
* **Kesiapan**: `/readyz` melaporkan tidak siap selama pemadaman, jadi orchestrator yang gating lalu lintas pada kesiapan menghapus setiap replika dari rotasi sekaligus. Dalam topologi itu semua lalu lintas, termasuk inferensi yang dapat masih dilayani gateway, gagal di penyeimbang beban sampai Postgres pulih. Probe liveness di `/healthz` terus lulus, jadi replika tidak dimulai ulang. Arahkan probe kesiapan ke `/healthz` sebagai gantinya jika Anda lebih suka pengembang yang masuk terus bekerja melalui pemadaman toko; biayanya adalah masuk baru gagal terhadap replika yang masih melaporkan siap.

Jika IdP Anda turun, sesi yang ada bekerja sampai `ttl_hours`, dan login dan penyegaran baru gagal. Atur `ttl_hours` yang lebih lama jika IdP Anda memiliki jendela pemeliharaan yang sering.

<h3 id="jwt-secret-rotation">
  Rotasi rahasia JWT
</h3>

Putar rahasia penandatanganan dalam tiga langkah sehingga sesi yang ada tetap valid:

1. Hasilkan rahasia baru. Tambahkan ke depan array `session.jwt_secret`.
2. Gulung penyebaran. Token baru menandatangani dengan rahasia baru; token lama masih memverifikasi.
3. Setelah `ttl_hours` ditambah margin, hapus rahasia lama dan gulung lagi.

Rotasi juga satu-satunya cara untuk memaksa sesi keluar sebelum mereka berakhir: token pembawa memvalidasi secara lokal terhadap rahasia JWT, jadi tidak ada pencabutan per-sesi. Mengganti rahasia sepenuhnya, tanpa menyimpan yang lama dalam array, membatalkan setiap sesi yang luar biasa sekaligus. Untuk offboarding individual, deprovision pengguna di IdP Anda; sesi mereka berakhir dalam `ttl_hours`.

<h3 id="postgres">
  Postgres
</h3>

Gateway menyimpan lima tabel, semuanya dibuat oleh migrasi waktu boot-nya:

| Tabel              | Isi                                                                               | Retensi                                                              |
| ------------------ | --------------------------------------------------------------------------------- | -------------------------------------------------------------------- |
| `kv`               | Hibah perangkat (TTL 10 menit) dan penghitung batas laju                          | TTL per baris                                                        |
| `spend`            | Penghitung pengeluaran periode-ke-tanggal per-principal, dalam sen                | `admin.spend_retention_months`, default 13                           |
| `spend_limits`     | Batas pengeluaran yang dikonfigurasi                                              | Sampai dihapus melalui API                                           |
| `admin_audit`      | Jejak mutasi Admin API                                                            | `admin.audit_retention_days`, default 365                            |
| `principal_emails` | Email terakhir dilihat setiap principal, nama tampilan, dan grup IdP. Berisi PII. | `admin.identity_retention_days` sejak aktivitas terakhir, default 90 |

Loop 30 detik mengakhiri baris `kv` melewati TTL mereka, dan sapuan per jam memberlakukan jendela retensi pada tabel pengeluaran, jadi tidak ada yang tumbuh tanpa batas. Tanpa [batas pengeluaran](/docs/id/claude-apps-gateway-spend-limits) yang dikonfigurasi, hanya `kv` yang ditulis. Jika kebijakan keamanan Anda melarang DDL dari peran aplikasi, buat tabel ini sebelumnya dan `_migrations` dengan peran admin dan berikan peran aplikasi `SELECT, INSERT, UPDATE, DELETE` pada masing-masing.

Dengan batas pengeluaran digunakan, database yang hilang berarti pelacakan pengeluaran dan batas yang hilang, bukan hanya login ulang pengembang, jadi jalankan backup reguler. Untuk menghapus satu pengembang yang pergi segera daripada menunggu retensi, jalankan `DELETE FROM principal_emails WHERE principal = '<sub>'` secara langsung; itu menghapus satu-satunya tabel yang menyimpan email, nama, dan grup mereka. Baris `spend` dan `admin_audit` mereferensikan hanya `sub` OIDC pseudonim.

<h3 id="upgrades">
  Upgrade
</h3>

Replika tidak memiliki status, jadi restart bergulir aman kapan saja. Gateway menjalankan migrasi skema saat boot, yang berarti menyebarkan biner baru secara otomatis memigrasikan database. Jika peran database tidak dapat menjalankan DDL, buat skema sebelumnya, termasuk tabel `_migrations` yang ditanam ke versi saat ini; jika tidak boot gagal mencoba `CREATE TABLE`.

Migrasi adalah append-only, jadi rollback ke biner sebelumnya yang mengetahui lebih sedikit migrasi aman; itu mengabaikan baris ekstra. Rollback juga memvalidasi ulang YAML terhadap skema biner yang lebih lama, jadi konfigurasi yang mengadopsi kunci yang diperkenalkan oleh rilis yang lebih baru gagal boot pada yang lebih lama. Hapus kunci baru sebelum rollback.

Karena Anda menyematkan versi gateway dalam gambar Anda sendiri, perbaikan dalam rilis Claude Code baru, termasuk perbaikan keamanan, mencapai penyebaran Anda hanya ketika Anda memperbarui pin dan menyebarkan ulang. Sertakan gateway dalam kadence patching yang sama yang Anda gunakan untuk layanan lain yang menyimpan kredensial produksi.

<h2 id="security">
  Keamanan
</h2>

Bagian ini menjawab pertanyaan yang ditanyakan tinjauan keamanan: aliran data apa melalui gateway dan ke mana perginya, serangan mana yang dipertahankan desain, dan jawaban mana yang termasuk dalam kuesioner kepatuhan.

<h3 id="data-flow">
  Aliran data
</h3>

| Data                                                                                               | Jalur                                                           | Dikirim ke Anthropic oleh gateway                           |
| -------------------------------------------------------------------------------------------------- | --------------------------------------------------------------- | ----------------------------------------------------------- |
| Inferensi (prompt, penyelesaian)                                                                   | CLI → gateway → upstream Anda                                   | Hanya jika API Anthropic adalah upstream yang dikonfigurasi |
| Telemetri (metrik OTLP, ditambah [log dan jejak opt-in](/docs/id/claude-apps-gateway-config#telemetry)) | CLI → gateway → kolektor Anda                                   | Tidak pernah                                                |
| Identitas (email, grup, sub)                                                                       | IdP → gateway → JWT → CLI; CLI memberi stempel pada ekspor OTLP | Tidak pernah                                                |
| Pengaturan terkelola                                                                               | YAML gateway Anda → CLI                                         | Tidak pernah                                                |
| Log audit                                                                                          | Stderr gateway → agregator Anda                                 | Tidak pernah                                                |

<h3 id="threat-model-summary">
  Ringkasan model ancaman
</h3>

Gateway duduk di dalam perimeter jaringan Anda, tetapi laptop pengembang individual tidak diperlakukan sebagai terpercaya. Desain memperhitungkan ini dalam tiga cara:

* Pengembang menyimpan JWT berumur pendek bukan kunci upstream mentah. Leg CLI-ke-gateway menggunakan hibah perangkat RFC 8628, dan pertukaran kode otorisasi gateway dengan IdP menjalankan PKCE dalam konfigurasi default, jadi kode otorisasi IdP yang disadap tidak berguna.
* Halaman verifikasi perangkat memberlakukan POST asal-sama dan batas laju per-IP per RFC 8628 §5.1. Lihat [Resistansi brute-force kode pengguna](#user-code-brute-force-resistance).
* Permintaan keluar melalui penjaga server-side request forgery (SSRF) yang menyelesaikan DNS, memblokir alamat link-lokal dan cloud-metadata ditambah loopback secara default, dan menyematkan koneksi ke IP yang diselesaikan, jadi URL yang dipengaruhi operator seperti IdP dan tujuan OTLP tidak dapat dialihkan ke endpoint metadata cloud. Rentang pribadi RFC 1918 secara sengaja diizinkan, karena IdP dan kolektor OTLP biasanya tinggal di IP pribadi. Untuk pengembangan lokal terhadap IdP loopback atau kolektor, atur `CLAUDE_GATEWAY_ALLOW_LOOPBACK=1` dalam lingkungan gateway; biarkan tidak disetel dalam produksi.

Jika Anda menambahkan kontrol egress Anda sendiri, gateway harus menjangkau server metadata kapan pun menggunakan kredensial metadata instance seperti identitas beban kerja.

Dua ancaman berada di luar cakupan karena mereka adalah infrastruktur Anda untuk diamankan:

* **Host gateway yang dikompromikan**: host menyimpan kredensial upstream dan mendistribusikan [pengaturan terkelola](/docs/id/claude-apps-gateway-config#managed) ke setiap pengembang yang terhubung, jadi kontrol atas konfigurasi gateway sebanding dengan kontrol atas MDM Anda. Dialog persetujuan satu kali CLI untuk pengaturan yang mampu shell membatasi perubahan diam-diam tetapi tidak menggantikan keamanan host.
* **Penyedia OIDC yang berbahaya**: penyedia menandatangani id\_token yang dipercaya gateway, jadi dapat menegaskan identitas apa pun. Penyaringan dan pengamanan IdP Anda adalah tanggung jawab Anda.

<h3 id="user-code-brute-force-resistance">
  Resistansi brute-force kode pengguna
</h3>

`user_code` yang diketik pengembang ke halaman verifikasi `/device` adalah 8 karakter yang diambil dari alfabet 20 karakter, yang menghasilkan 20⁸ atau sekitar 2,56×10¹⁰ kombinasi, dan berakhir setelah 10 menit.

Gateway menerapkan batas laju per-IP pada endpoint hibah perangkat, dapat dikonfigurasi melalui [`rate_limits`](/docs/id/claude-apps-gateway-config#http-tuning). Naikkan batas jika banyak pengembang masuk dari alamat NAT korporat bersama tunggal. Batas hanya berlaku pada aliran masuk, bukan pada inferensi.

<h3 id="compliance-posture">
  Postur kepatuhan
</h3>

* **Residensi data**: bidang data sendiri gateway mengirim tidak ada ke Anthropic kecuali API Anthropic adalah upstream yang dikonfigurasi; ketika itu, perjanjian penanganan data yang ada berlaku untuk jalur inferensi. Telemetri, audit, identitas, dan pengaturan hanya pergi ke tujuan yang Anda konfigurasikan.
* **Lalu lintas proses host**: proses host adalah CLI Claude Code, yang dapat mengirim analitik startup dan pemeriksaan pembaruan ke Anthropic. Untuk penyebaran egress ketat, atur `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC=1` dalam lingkungan kontainer gateway.
* **Analitik klien**: CLI menonaktifkan analitik penggunaan sendiri saat masuk ke gateway, dan pelaporan kesalahan dimatikan secara default pada permukaan API pihak ketiga.
* **Mesin klien**: CLI pengembang masih mengirim pemeriksaan nama host WebFetch dan pemeriksaan versi ke Anthropic kecuali `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC=1` dan `skipWebFetchPreflight: true` diatur. Lihat [penggunaan data](/docs/id/data-usage).
* **Peringkat survei**: kredensial gateway menonaktifkan sink peringkat terikat Anthropic, jadi peringkat tidak dikirim ke Anthropic.
* **Berbagi transkrip**: memilih Ya pada prompt berbagi transkrip survei menulis file lokal di bawah `~/.claude/feedback-bundles/` bukan mengunggah ke Anthropic.
* **Pembaruan klien**: pemeriksaan pembaruan terpisah dari lalu lintas gateway. Sematkan versi melalui distribusi Anda sendiri dan atur `DISABLE_UPDATES` jika laptop tidak boleh mengambil rilis. `DISABLE_AUTOUPDATER` menghentikan hanya pembaruan latar belakang sementara `claude update` masih berfungsi.
* **TLS**: layani `public_url` melalui HTTPS dalam produksi, baik dari pendengar gateway sendiri melalui `listen.tls` atau dari ingress yang menghentikan TLS di depan replika HTTP biasa dengan `listen.public_url` diatur. Gateway tidak menolak HTTP biasa. IdP harus melayani HTTPS dalam produksi, dan Postgres mendukung `?sslmode=require`. Atur `Strict-Transport-Security` di ingress Anda.
* **Pengungkapan kerentanan**: ikuti [Melaporkan masalah keamanan](/docs/id/security#reporting-security-issues)

<h2 id="troubleshooting">
  Troubleshooting
</h2>

Untuk pertanyaan dan umpan balik, gunakan [dukungan Claude Code](https://support.claude.com/en/collections/14445694-claude-code), atau buka masalah di [repositori GitHub Claude Code](https://github.com/anthropics/claude-code/issues). Saat melaporkan masalah, sertakan:

* **Masalah gateway**: stderr gateway untuk jendela yang relevan, `gateway.yaml` Anda dengan rahasia diedit, versi gateway, ditampilkan di halaman pendaratan di `/` dan dalam header respons `x-cc-gateway-version` di `/managed/settings`, dan apa yang berubah baru-baru ini
* **Masalah login**: pengembang menjalankan `claude --debug-file ./claude-debug.txt`, mereproduksi, dan mengirim file itu ditambah log audit gateway untuk jendela yang sama
* **Masalah inferensi**: model yang diminta, upstream yang dikonfigurasi, dan log audit gateway untuk permintaan, yang mencatat upstream mana yang melayaninya dan status respons

Stderr gateway mencakup aliran acara audit, log audit mencatat identitas pengembang, dan file debug mencatat output hook dan server MCP dari mesin pengembang. Tinjau dan redaksi ini sebelum memposting ke issue publik.

| Gejala                                                                                                                                                                                    | Penyebab                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  | Perbaikan                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `/login` pengembang menampilkan pemilih akun standar bukan layar **Cloud gateway**                                                                                                        | `forceLoginMethod` atau `forceLoginGatewayUrl` tidak diatur dalam pengaturan terkelola pada mesin itu                                                                                                                                                                                                                                                                                                                                                                                                                                                                     | Sebarkan [file pengaturan terkelola](/docs/id/claude-apps-gateway#set-the-gateway-url) ke perangkat; `/login` membaca URL gateway dari sana                                                                                                                                                                                                                                                                                                                           |
| Startup menampilkan `Gateway login is configured in managed settings, but this Claude Code build does not include Cloud gateway support.`                                                 | Build Claude Code yang diinstal mendahului dukungan gateway                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               | Minta pengembang memperbarui Claude Code ke rilis yang mencakup dukungan Cloud gateway                                                                                                                                                                                                                                                                                                                                                                           |
| CLI `/login`: `Gateway hosts must be on your organization's private network; <host> resolves to the public (or unrecognized) address <ip>`                                                | Nama host gateway diselesaikan ke setidaknya satu alamat IP publik. Claude Code memeriksa setiap alamat yang diselesaikan dan memerlukan setiap satu menjadi pribadi. Penyebab umum adalah nama dual-stack di mana satu keluarga diselesaikan ke alamat publik, termasuk penyeimbang beban dual-stack internal AWS, yang mengembalikan alamat AAAA rentang publik. Titik akhir gateway publik yang dioperasikan Anthropic dikecualikan dari pemeriksaan, dan `/login` menerimanya melalui `https://`. Sebelum v2.1.206, `/login` menolaknya seperti alamat publik lainnya | Minta nama gateway hanya diselesaikan ke alamat pribadi pada mesin pengembang. Untuk nama dual-stack, lepaskan catatan rentang publik atau layani nama DNS internal saja. Lihat [prasyarat jaringan pribadi](/docs/id/claude-apps-gateway#prerequisites).                                                                                                                                                                                                             |
| CLI `/login`: `Gateway login requires a direct connection and does not support connecting through an HTTP proxy`                                                                          | `HTTPS_PROXY` atau `HTTP_PROXY` berlaku untuk host gateway dan nama host proxy diselesaikan ke alamat publik. Proxy yang nama hostnya hanya diselesaikan ke alamat pribadi diizinkan dan tidak memicu kesalahan ini                                                                                                                                                                                                                                                                                                                                                       | Tambahkan host gateway ke `NO_PROXY` pada mesin pengembang sehingga koneksi langsung, atau gunakan proxy yang nama hostnya diselesaikan ke alamat pribadi                                                                                                                                                                                                                                                                                                        |
| CLI `/login`: `Could not resolve gateway host <host>`                                                                                                                                     | Mesin tidak dapat menyelesaikan nama DNS internal gateway, biasanya karena tidak berada di jaringan korporat                                                                                                                                                                                                                                                                                                                                                                                                                                                              | Minta pengembang terhubung ke jaringan atau VPN Anda, lalu coba ulang `/login`                                                                                                                                                                                                                                                                                                                                                                                   |
| Boot keluar dengan kesalahan validasi konfigurasi yang menamai `store.postgres_url`                                                                                                       | Tidak ada Postgres yang dikonfigurasi; gateway memerlukan Postgres                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        | Atur `store.postgres_url`. Untuk pengembangan lokal, gunakan kontainer sekali pakai: `docker run --rm -p 5432:5432 -e POSTGRES_HOST_AUTH_METHOD=trust postgres`.                                                                                                                                                                                                                                                                                                 |
| Boot keluar: `requires the native binary`                                                                                                                                                 | Berjalan di bawah Node bukan biner asli                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   | Instal Claude Code dengan salah satu [metode instalasi mandiri](/docs/id/setup)                                                                                                                                                                                                                                                                                                                                                                                       |
| Boot keluar dengan kesalahan penemuan OIDC setelah `config.load`                                                                                                                          | `oidc.issuer` tidak dapat dijangkau, atau rantai TLS tidak dipercaya                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      | Periksa issuer dapat dijangkau dari pod dan melayani `/.well-known/openid-configuration`. Atur `ca_cert_pem` untuk PKI pribadi.                                                                                                                                                                                                                                                                                                                                  |
| Boot keluar dengan kesalahan izin Postgres                                                                                                                                                | Peran aplikasi kekurangan `CREATE TABLE`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  | Buat skema sebelumnya dengan peran admin dan berikan DML ke peran aplikasi, atau berikan DDL sementara untuk boot yang menerapkan migrasi baru                                                                                                                                                                                                                                                                                                                   |
| `/oauth/callback` menampilkan "Sign-in could not be completed"                                                                                                                            | Domain email ditolak, validasi id\_token gagal, atau `email_verified` secara eksplisit `false`, yang gateway selalu tolak tanpa override                                                                                                                                                                                                                                                                                                                                                                                                                                  | Periksa `allowed_email_domains` dan bahwa IdP mengembalikan klaim `email` yang diverifikasi. Untuk `email_verified: false`, perbaiki verifikasi sisi IdP. Jika IdP Anda memancarkan email di bawah nama klaim berbeda, atur `oidc.email_claim`.                                                                                                                                                                                                                  |
| Log: `token exchange failed: id_token missing email claim`                                                                                                                                | IdP tidak menyertakan `email` dalam id\_token secara default. Penolakan ini hanya terjadi ketika `allowed_email_domains` diatur; tanpanya, email yang hilang mencetak sesi tanpa email                                                                                                                                                                                                                                                                                                                                                                                    | Konfigurasikan IdP untuk memancarkan `email` dalam id\_token. Okta: tambahkan `email` ke klaim ID-token server otorisasi khusus. Entra: tambahkan `email` sebagai klaim opsional pada pendaftaran aplikasi. PingFederate: aktifkan Kebijakan OpenID Connect yang memancarkan `email`. Jika IdP melayani `email` dari endpoint userinfo tetapi tidak akan menyertakannya dalam id\_token, seperti server otorisasi org Okta, atur `oidc.userinfo_fallback: true`. |
| Setiap permintaan Amazon Bedrock mengembalikan 502; log menunjukkan `Could not load credentials from any providers`                                                                       | Di EC2, batas hop default IMDSv2 sebesar 1 memblokir permintaan metadata instance dari dalam kontainer. Boot dan `/readyz` lulus bagaimanapun karena AWS SDK menyelesaikan kredensial instance pada permintaan pertama, bukan pada konstruksi klien                                                                                                                                                                                                                                                                                                                       | Naikkan batas hop dengan `aws ec2 modify-instance-metadata-options --instance-id <id> --http-put-response-hop-limit 2`, atau atur dalam template peluncuran. Perubahan berlaku untuk setiap kontainer pada instance. Lebih suka peran tugas ECS di mana tersedia, yang membaca kredensial dari endpoint kredensial kontainer ECS dan menghindari perubahan sepenuhnya, atau terapkan perubahan pada instance gateway khusus untuk membatasi eksposur.            |
| Kesalahan IdP: unknown or unsupported scope                                                                                                                                               | IdP menolak scope yang tidak dikenalinya                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  | Atur `oidc.scopes` ke tepat daftar yang diterima IdP Anda; itu harus menyertakan `openid`. Default adalah `openid profile email offline_access`.                                                                                                                                                                                                                                                                                                                 |
| Sesi tidak secara diam-diam memperbarui setelah menetapkan `oidc.scopes`                                                                                                                  | `offline_access` dijatuhkan dari override                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 | Tambahkan `offline_access` kembali jika IdP Anda mendukungnya. Tanpa token refresh, pengembang menjalankan kembali login browser setiap `session.ttl_hours`.                                                                                                                                                                                                                                                                                                     |
| Browser menampilkan "This request came from another site and was blocked"                                                                                                                 | POST formulir lintas situs, diblokir sebagai perlindungan CSRF. Diharapkan untuk halaman tertanam atau diproksi                                                                                                                                                                                                                                                                                                                                                                                                                                                           | Buka tautan verifikasi secara langsung                                                                                                                                                                                                                                                                                                                                                                                                                           |
| Chrome memblokir tombol Approve dengan "Refused to send form data … violates … Content Security Policy directive: form-action", tetapi halaman yang sama berfungsi di Safari atau Firefox | Chrome memberlakukan `form-action` terhadap seluruh rantai pengalihan. IdP Anda mengalihkan ke host kedua yang tidak ada dalam daftar putih.                                                                                                                                                                                                                                                                                                                                                                                                                              | Tambahkan setiap asal tambahan dalam rantai pengalihan ke `oidc.form_action_origins`. Buka Chrome DevTools → Console pada halaman Approve untuk melihat asal mana yang diblokir.                                                                                                                                                                                                                                                                                 |
| Masuk selesai di IdP tetapi callback gagal, dengan kesalahan CSP di Chrome atau "this sign-in link has expired" di Safari                                                                 | IdP mengembalikan kode melalui `response_mode=form_post`, yang secara otomatis mengirimkannya lintas asal melalui POST ke `/oauth/callback`. Chrome memblokir itu di bawah CSP ketat; Safari memungkinkan pengiriman tetapi callback hanya membaca string kueri.                                                                                                                                                                                                                                                                                                          | Pastikan IdP Anda menghormati `response_mode=query`, yang gateway minta secara eksplisit sehingga callback adalah pengalihan biasa                                                                                                                                                                                                                                                                                                                               |
| Login bekerja secara lokal tetapi gagal di belakang ALB                                                                                                                                   | `public_url` tidak diatur, jadi IdP mendapat asal `http://` dalam sebagai `redirect_uri`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  | Atur `listen.public_url` ke asal `https://` eksternal                                                                                                                                                                                                                                                                                                                                                                                                            |
| Pengembang melihat prompt kepercayaan berulang kali                                                                                                                                       | Sertifikat TLS berputar per replika atau per permintaan                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   | Gunakan sertifikat stabil di ingress, atau hentikan TLS sekali dan jalankan replika melalui HTTP biasa secara internal                                                                                                                                                                                                                                                                                                                                           |
| CLI `/login`: "Could not verify the gateway's TLS certificate" atau `SELF_SIGNED_CERT_IN_CHAIN`                                                                                           | Rantai TLS gateway ditandatangani oleh CA pribadi bukan dalam toko kepercayaan host CLI                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   | Claude Code membaca toko kepercayaan OS secara default pada biner asli dan pada Node 22.15 atau lebih baru; [`CLAUDE_CODE_CERT_STORE`](/docs/id/network-config#ca-certificate-store) mengontrol perilaku ini. Jika CA diinstal dalam toko kepercayaan OS, pastikan pengembang berada di runtime saat ini. Jika tidak atur `NODE_EXTRA_CA_CERTS` ke PEM sertifikat CA sebelum meluncurkan. Prompt sidik jari koneksi pertama masih berlaku.                            |

<h2 id="related">
  Terkait
</h2>

* [Gambaran umum gateway aplikasi Claude](/docs/id/claude-apps-gateway): quickstart dan koneksi pengembang
* [Referensi konfigurasi](/docs/id/claude-apps-gateway-config): setiap opsi `gateway.yaml`
