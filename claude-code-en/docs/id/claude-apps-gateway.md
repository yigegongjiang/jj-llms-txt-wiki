> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude apps gateway untuk Amazon Bedrock, Claude Platform di AWS, Google Cloud, dan Microsoft Foundry

> Jalankan Claude Code melalui Amazon Bedrock, Claude Platform di AWS, Google Cloud, atau Microsoft Foundry di balik gateway yang di-host sendiri dengan SSO sign-in, akses model per-grup, dan telemetri OTLP.

<Note>
  Claude apps gateway dirancang untuk organisasi yang harus — atau lebih suka — merutekan inferensi melalui penyedia cloud mereka sendiri, misalnya untuk memenuhi persyaratan [residensi data](/docs/id/claude-apps-gateway-deploy#compliance-posture). Jika Anda tidak memiliki persyaratan ini, dan menginginkan akses ke fitur lain seperti penyediaan SCIM atau Claude Code di web dan mobile, Claude Enterprise mungkin lebih cocok. Lihat halaman [ketersediaan fitur](/docs/id/feature-availability) untuk perbandingan lengkap semua metode penyebaran.
</Note>

Claude apps gateway adalah layanan yang di-host sendiri yang berada di antara klien Claude Code pengembang Anda dan penyedia model Anda. Pengembang masuk dengan penyedia identitas perusahaan Anda (IdP) alih-alih menyimpan kunci API atau kredensial cloud. Gateway menyimpan kredensial upstream, memberlakukan akses model dan [pengaturan terkelola](/docs/id/permissions#managed-settings) berdasarkan grup IdP, dan meneruskan telemetri penggunaan ke tumpukan observabilitas Anda sendiri.

Ini disertakan dalam biner `claude`, jadi executable yang sama yang menjalankan Claude Code di laptop menjalankan server gateway dengan `claude gateway --config gateway.yaml`.

Halaman ini mencakup:

* [Mengapa Claude apps gateway](#why-claude-apps-gateway), apa yang ditambahkannya dibandingkan menjalankan milik Anda sendiri, dan kapan sesuatu yang lain lebih cocok
* [Quickstart](#quickstart) dengan [prasyarat](#prerequisites) yang membawa gateway dari nol ke pengembang yang masuk
* [Menghubungkan pengembang](#connect-developers), termasuk menetapkan URL gateway melalui pengaturan terkelola
* [Ketersediaan dan keterbatasan](#availability-and-limitations) mencakup fitur Claude Code mana yang bekerja melalui gateway dan apa yang didukung server

Halaman pendamping menggali lebih dalam. [Referensi konfigurasi](/docs/id/claude-apps-gateway-config) mencakup setiap opsi dalam file YAML yang ditulis quickstart, dan [panduan penyebaran](/docs/id/claude-apps-gateway-deploy) mencakup penyiapan per-IdP, penyebaran Kubernetes dan Cloud Run, serta operasi.

<h2 id="why-claude-apps-gateway">
  Mengapa Claude apps gateway
</h2>

[Gambaran umum gateway](/docs/id/gateways) mencakup apa yang dilakukan gateway dan mengapa Anda akan menjalankannya. Claude apps gateway adalah gateway Anthropic sendiri, dibangun ke dalam biner `claude` dan diuji bersama setiap rilis Claude Code, jadi ia meneruskan header dan bidang permintaan yang dikirim Claude Code tanpa operator mempertahankan daftar izin terpisah. Setelah digunakan, ia memberi Anda:

* **Kredensial**: kunci API upstream atau kredensial cloud hanya ada di infrastruktur Anda. Pengembang melakukan autentikasi dengan SSO perusahaan dan menerima token bearer berumur pendek, jadi offboarding terjadi di IdP Anda. Hapus penyediaan pengguna dan akses gateway mereka kedaluwarsa dalam masa pakai sesi, satu jam secara default.
* **Kontrol akses**: grup IdP Anda memetakan ke daftar izin model dan kebijakan [pengaturan terkelola](/docs/id/permissions#managed-settings). Gateway memberlakukan akses model di sisi server, menolak permintaan untuk model yang tidak diberikan, dan memilih kebijakan pengaturan terkelola setiap grup, yang diterapkan CLI di [tingkat pengaturan terkelola](/docs/id/settings#settings-precedence). Tim yang berbeda mendapatkan model, alat, dan izin yang berbeda, dan pengembang tidak dapat mengganti apa yang dikunci kebijakan mereka.
* **Pengiriman pengaturan**: gateway mengirimkan pengaturan terkelola ke klien yang masuk sendiri, menggantikan [pengaturan yang dikelola server](/docs/id/server-managed-settings) dari konsol admin claude.ai.
* **Telemetri**: setiap tujuan yang dikonfigurasi, seperti Datadog, Splunk, atau ClickHouse, menerima [metrik OpenTelemetry Protocol (OTLP)](/docs/id/monitoring-usage) dengan hitungan token, model, identitas pengguna, dan latensi secara default, dengan log dan jejak sebagai opt-in per-tujuan.
* **Perutean upstream**: klien berbicara API Pesan Anthropic ke gateway, dan gateway menerjemahkan untuk setiap upstream, baik Bedrock, [Claude Platform on AWS](/docs/id/claude-platform-on-aws), Agent Platform Google Cloud, Foundry, atau API Anthropic, dengan failover di antara mereka. Anda dapat mengubah wilayah, penyedia, atau urutan failover tanpa pengembang menyadari atau mengonfigurasi ulang.

<Frame>
  <img src="https://mintcdn.com/claude-code/VbyXug8hBU9UK6oT/images/claude-gateway-architecture.svg?fit=max&auto=format&n=VbyXug8hBU9UK6oT&q=85&s=9e4f1190fc56718144190a3db61c63af" alt="Diagram menunjukkan klien Claude Code terhubung melalui HTTPS dengan token bearer ke gateway Claude apps yang di-host sendiri di dalam infrastruktur Anda, yang menandatangani pengguna terhadap IdP Anda, menyimpan status auth di PostgreSQL, meneruskan telemetri ke kolektor OTLP Anda, dan meneruskan inferensi ke Amazon Bedrock, Claude Platform on AWS, Google Cloud, Microsoft Foundry, atau API Anthropic" width="760" height="320" data-path="images/claude-gateway-architecture.svg" />
</Frame>

<Note>
  Bidang data gateway sendiri tidak mengirim apa pun ke infrastruktur Anthropic kecuali API Anthropic adalah upstream yang dikonfigurasi. Anda mengontrol ke mana telemetri, log audit, pengaturan terkelola, dan identitas IdP pengembang Anda pergi, dan gateway tidak mengirimkan salah satu dari mereka ke Anthropic. Untuk lalu lintas yang tersisa proses CLI dapat mengirim dan cara menutupnya, lihat [Compliance posture](/docs/id/claude-apps-gateway-deploy#compliance-posture).
</Note>

Untuk fitur Claude Code mana yang bekerja melalui gateway dan apa yang didukung server itu sendiri, lihat [Ketersediaan dan keterbatasan](#availability-and-limitations) di bawah. Untuk keputusan seperti biaya, bypass, menjalankan beberapa gateway, dan platform serverless, lihat [panduan penyebaran](/docs/id/claude-apps-gateway-deploy#deployment).

<h3 id="other-gateway-implementations">
  Implementasi gateway lainnya
</h3>

Jika Anda sudah menjalankan gateway LLM atau gateway API yang memenuhi kebutuhan Anda, terus gunakan; [Gateway LLM lainnya](/docs/id/llm-gateway) mencakup konfigurasi Claude Code terhadapnya.

[Referensi protokol gateway](/docs/id/llm-gateway-protocol) mendokumentasikan kontrak yang diharapkan Claude Code dari gateway apa pun: endpoint yang dipanggilnya, header dan bidang body untuk diteruskan, dan apa yang berhenti bekerja ketika mereka dihapus. Gateway Claude apps yang berjalan melayani superset kontrak itu di `GET /protocol`, menambahkan endpoint khusus gateway Claude apps untuk SSO sign-in, pengiriman pengaturan terkelola, dan telemetri. Ambilnya dengan `curl https://claude-gateway.internal.example.com/protocol` dari gateway yang digunakan apa pun, seperti yang dihasilkan [quickstart](#quickstart) di bawah. Perubahan breaking pada protokol diumumkan sebelumnya, tetapi kompatibilitas backward yang tidak terbatas tidak dijamin.

<h2 id="quickstart">
  Quickstart
</h2>

Quickstart ini berjalan di jalur minimal: daftarkan klien OAuth di IdP Anda, tulis `gateway.yaml`, jalankan gateway bersama Postgres dengan Docker Compose, dan verifikasi sign-in end to end. Ini menggunakan upstream Amazon Bedrock; Claude Platform on AWS, Agent Platform Google Cloud, Microsoft Foundry, dan API Anthropic sama-sama didukung dengan menukar blok `upstreams` seperti yang ditunjukkan dalam [referensi konfigurasi](/docs/id/claude-apps-gateway-config#upstreams). Di akhir Anda memiliki gateway yang dapat `/login` pengembang.

<Note>
  **Terapkan di jaringan pribadi Anda.** Claude Code hanya terhubung ke gateway yang alamatnya pribadi. Ini adalah penjaga keamanan, karena gateway yang dipercaya dapat mendorong pengaturan yang menjalankan perintah pada mesin pengembang. Letakkan gateway di balik load balancer internal atau VPN dan berikan nama host yang hanya diselesaikan ke IP pribadi.

  Titik akhir gateway publik yang dioperasikan Anthropic adalah pengecualian: `/login` menerimanya melalui `https://`. Ini adalah kumpulan tetap kecil dari gateway yang dioperasikan Anthropic sendiri; mereka bukan opsi penyebaran yang dapat Anda pilih atau konfigurasi. Daftar dikompilasi ke dalam Claude Code, jadi tidak ada konfigurasi yang dapat menambahkan nama host ke dalamnya dan tidak ada gateway yang Anda hosting yang memenuhi syarat untuk pengecualian. Sebelum v2.1.206, `/login` menolak titik akhir tersebut seperti alamat publik lainnya.
</Note>

<h3 id="prerequisites">
  Prasyarat
</h3>

Miliki ini sebelum Anda mulai:

| Anda membutuhkan                         | Detail                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| ---------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Claude Code v2.1.195 atau lebih baru     | Subperintah `claude gateway` dan alur sign-in gateway dikirim di v2.1.195. Build publik sebelumnya tidak menyertakannya. Baik mesin yang menjalankan server gateway maupun mesin setiap pengembang harus pada v2.1.195 atau lebih baru; jalankan `claude update` untuk mendapatkan rilis terbaru. Upstream [Claude Platform on AWS](/docs/id/claude-apps-gateway-config#claude-platform-on-aws) memerlukan Claude Code v2.1.198 atau lebih baru di server gateway.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| Penyedia identitas OpenID Connect (OIDC) | Okta, Microsoft Entra ID, Google Workspace, Keycloak, atau Dex, atau IdP yang sesuai dengan OIDC lainnya seperti PingFederate. Gateway menjalankan penemuan OIDC standar dan alur kode otorisasi terhadapnya. SAML dan LDAP tidak didukung.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| PostgreSQL 14 atau lebih baru            | Mendukung alur sign-in perangkat, di mana callback browser menulis dan CLI polling membaca, ditambah penghitung batas laju. Postgres yang dikelola apa pun berfungsi, termasuk tingkat terkecil. Tanpa batas pengeluaran yang dikonfigurasi, gateway menyimpan beberapa KB status auth berumur pendek; dengan [batas pengeluaran](/docs/id/claude-apps-gateway-spend-limits), ia juga menyimpan tabel pengeluaran, audit, dan identitas yang tahan lama yang harus dicadangkan. TLS melalui `?sslmode=require` direkomendasikan.                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| Model upstream                           | Kredensial Amazon Bedrock, kredensial Claude Platform on AWS, kredensial Google Cloud, sumber daya Microsoft Foundry, atau kunci API Anthropic. Beberapa upstream didukung dengan failover.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| HTTPS                                    | Gateway harus dapat dijangkau melalui `https://` dari laptop pengembang dan dari browser apa pun yang digunakan untuk sign-in; gateway melayani halaman verifikasi perangkat pada pendengar yang sama. Berikan sertifikat TLS melalui `listen.tls`, atau jalankan di balik ingress yang menghentikan TLS dan atur `listen.public_url`. Asal `http://` biasa hanya diterima di loopback, untuk pengembangan lokal.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| Alamat jaringan pribadi                  | Di `/login`, Claude Code memerlukan nama host atau alamat IP gateway untuk diselesaikan hanya ke alamat pribadi: RFC 1918, CGNAT `100.64.0.0/10`, IPv6 ULA `fc00::/7`, atau loopback untuk pengembangan lokal. Pemeriksaan berjalan pada setiap IP yang diselesaikan, jadi jika alamat apa pun yang diselesaikan nama adalah publik, `/login` menolak URL. Jika mesin pengembang merutekan HTTPS melalui proxy perusahaan, sign-in juga memerlukan host proxy untuk diselesaikan ke alamat pribadi; jika tidak, tambahkan host gateway ke `NO_PROXY` sehingga CLI terhubung langsung. Titik akhir gateway publik yang dioperasikan Anthropic dikecualikan dari pemeriksaan alamat pribadi dan proxy: `/login` menerimanya melalui `https://` dengan pencocokan nama host yang tepat, jadi persyaratan jaringan pribadi hanya berlaku untuk gateway yang Anda hosting sendiri. Sebelum v2.1.206, `/login` menolak titik akhir yang dioperasikan Anthropic seperti alamat publik lainnya. |
| Runtime Linux                            | Server gateway hanya berjalan pada biner Linux asli. macOS berfungsi untuk pengembangan lokal. Windows tidak didukung sebagai platform server.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |

Server gateway memerlukan biner `claude` asli; unduh rilis yang disematkan seperti yang dijelaskan dalam [Install Claude Code](/docs/id/setup). Server menggunakan fitur runtime yang tidak tersedia ketika Claude Code berjalan di bawah Node. Jika Anda melihat `requires the native binary` saat boot, beralih ke salah satu metode instalasi standalone.

<h3 id="steps">
  Langkah-langkah
</h3>

<Steps>
  <Step title="Daftarkan klien OAuth di IdP Anda">
    Tentukan nama host gateway terlebih dahulu, karena URI pengalihan harus cocok dengannya. Buat aplikasi web OIDC baru dan atur URI pengalihan ke `https://claude-gateway.<your-domain>/oauth/callback`, di mana host adalah nilai yang sama yang Anda atur sebagai [`listen.public_url`](/docs/id/claude-apps-gateway-config#listen) di langkah 3. Catat `client_id` dan `client_secret`. Instruksi per-IdP ada di [Identity provider setup](/docs/id/claude-apps-gateway-deploy#identity-provider-setup).
  </Step>

  <Step title="Sediakan database PostgreSQL">
    Postgres 14 atau lebih baru apa pun berfungsi, termasuk tingkat terkelola terkecil. Gateway menjalankan migrasi skema sendiri saat boot, jadi pengguna database memerlukan izin `CREATE TABLE`. Jika kebijakan keamanan Anda melarang DDL dari peran aplikasi, buat skema sebelumnya; lihat [`store`](/docs/id/claude-apps-gateway-config#store).
  </Step>

  <Step title="Tulis gateway.yaml">
    Rahasia dibaca melalui ekspansi `${ENV_VAR}` sehingga file itu sendiri dapat hidup dalam kontrol versi. Gunakan nama host `public_url` yang diselesaikan ke IP pribadi di jaringan Anda, karena `/login` menolak alamat publik. Konfigurasi minimal memiliki lima bagian, dan setiap bidang lainnya memiliki default:

    ```yaml gateway.yaml theme={null}
    listen:
      host: 0.0.0.0
      port: 8080
      # Diperlukan di balik proxy apa pun yang menghentikan TLS. Digunakan untuk IdP
      # redirect_uri dan dokumen penemuan.
      public_url: https://claude-gateway.internal.example.com

    oidc:
      issuer: https://login.example.com        # harus melayani /.well-known/openid-configuration
      client_id: 0oa1example2
      client_secret: ${OIDC_CLIENT_SECRET}
      allowed_email_domains: [example.com]        # tolak id_tokens di luar organisasi Anda
      userinfo_fallback: true                  # untuk IdP yang id_token-nya menghilangkan email/groups; tidak berbahaya sebaliknya

    session:
      jwt_secret: ${GATEWAY_JWT_SECRET}        # openssl rand -base64 32
      ttl_hours: 1                             # juga membatasi latensi revokasi pada deprovisi IdP

    store:
      postgres_url: ${GATEWAY_POSTGRES_URL}    # tambahkan ?sslmode=require untuk Postgres terkelola

    upstreams:
      - provider: bedrock
        region: us-east-1
        auth: {} # kosong: rantai kredensial default AWS
    # (IRSA, peran tugas EC2/ECS, variabel env, ~/.aws)

    # Model diterjemahkan per upstream secara otomatis. Katalog bawaan
    # memetakan claude-opus-4-8 ke us.anthropic.claude-opus-4-8 dan seterusnya untuk setiap
    # model Claude yang didukung Bedrock. Atur false dan tambahkan daftar `models:` untuk
    # mengekspos hanya model tertentu.
    auto_include_builtin_models: true
    ```

    Konfigurasi ini cukup untuk loop sign-in yang berfungsi dengan katalog model Bedrock default. Setelah berjalan, tambahkan RBAC per-grup dan pengaturan terkelola melalui [`managed.policies`](/docs/id/claude-apps-gateway-config#managed), fan-out telemetri melalui [`telemetry`](/docs/id/claude-apps-gateway-config#telemetry), dan failover multi-upstream, ARN throughput yang disediakan, atau wilayah non-AS melalui [`models`](/docs/id/claude-apps-gateway-config#models).

    <Note>
      Upstream Bedrock memerlukan principal AWS dengan `bedrock:InvokeModel` dan `bedrock:InvokeModelWithResponseStream` pada ARN `inference-profile/us.anthropic.*` dan ARN `foundation-model/anthropic.*` yang mendasar, dan akses model diaktifkan di konsol Bedrock untuk model Claude yang Anda inginkan. Sediakan kredensial dengan IRSA di EKS, peran tugas ECS, atau profil instans EC2 daripada kunci statis. [Referensi `upstreams`](/docs/id/claude-apps-gateway-config#upstreams) memiliki detail IAM lengkap, matriks kredensial lintas cloud, dan blok `auth` untuk penyedia lain.
    </Note>
  </Step>

  <Step title="Jalankan">
    Bangun gambar kontainer di sekitar biner `claude` yang memenuhi [persyaratan gambar](/docs/id/claude-apps-gateway-deploy#container-image), kemudian jalankan bersama Postgres:

    ```yaml docker-compose.yaml theme={null}
    services:
      gateway:
        image: <your-registry>/claude-gateway:<version>
        ports: ["8080:8080"]
        volumes: ["./gateway.yaml:/etc/claude/gateway.yaml:ro"]
        environment:
          OIDC_CLIENT_SECRET: ${OIDC_CLIENT_SECRET}
          GATEWAY_JWT_SECRET: ${GATEWAY_JWT_SECRET}
          GATEWAY_POSTGRES_URL: postgres://gw:pw@postgres/gateway
          # Kredensial AWS: dalam produksi, hilangkan ini dan gunakan peran instans
          # Untuk pengujian Compose lokal, teruskan milik Anda sendiri:
          AWS_ACCESS_KEY_ID: ${AWS_ACCESS_KEY_ID}
          AWS_SECRET_ACCESS_KEY: ${AWS_SECRET_ACCESS_KEY}
          AWS_SESSION_TOKEN: ${AWS_SESSION_TOKEN}
        depends_on:
          postgres:
            condition: service_healthy
      postgres:
        image: postgres:16-alpine
        environment: { POSTGRES_USER: gw, POSTGRES_PASSWORD: pw, POSTGRES_DB: gateway }
        healthcheck:
          test: ["CMD-SHELL", "pg_isready -U gw"]
          interval: 5s
        volumes: ["pgdata:/var/lib/postgresql/data"]
    volumes: { pgdata: }
    ```

    Gateway adalah biner Linux tunggal yang membaca konfigurasi, menjalankan penemuan OIDC terhadap IdP Anda, menerapkan migrasi skema Postgres-nya, membangun klien upstream, dan mulai mendengarkan. Boot gagal-tertutup untuk konfigurasi, koneksi Postgres dengan timeout 5 detik, penemuan OIDC, dan konstruksi klien upstream. Jika salah satu dari mereka tidak dapat dijangkau atau salah konfigurasi, gateway keluar dengan kesalahan daripada melayani lalu lintas dalam keadaan terdegradasi.

    Boot yang berhasil tidak memvalidasi jalur inferensi, karena kredensial instans Bedrock dan Agent Platform diselesaikan pada permintaan pertama, bukan saat boot.

    Tonton stderr untuk urutan boot. Baris log menggunakan format `[gateway] <timestamp> <level> <message>`, acara audit adalah JSON satu baris dengan bidang `evt`, dan spanduk startup, dihilangkan di bawah, dicetak di antara baris migrasi dan mendengarkan. Anda harus melihat, dalam urutan:

    ```text theme={null}
    {"ts":"2026-06-10T17:03:21.114Z","evt":"config.load","path":"/etc/claude/gateway.yaml","sha256":"…"}
    [gateway] 2026-06-10T17:03:21.408Z info migration 1 applied
    [gateway] 2026-06-10T17:03:21.512Z info claude gateway listening on http://0.0.0.0:8080
    ```

    Jika boot keluar sebelum baris `claude gateway listening on`, baris terakhir stderr menamai masalahnya:

    * Postgres yang tidak dapat dijangkau
    * Peran Postgres tanpa izin DDL
    * Dokumen penemuan OIDC yang tidak dapat dijangkau atau tidak valid
    * Pelanggaran skema konfigurasi dengan jalur bidang yang menyinggung

    Perbaiki dan mulai ulang.

    Jika Anda sudah memiliki ingress yang menghentikan TLS, lewati Compose dan jalankan biner secara langsung dengan `claude gateway --config gateway.yaml`. Atur `public_url` ke asal ingress dan ikat `listen` ke alamat loopback atau internal kluster.
  </Step>

  <Step title="Verifikasi permukaan auth">
    Tiga pemeriksaan mengkonfirmasi gateway dapat mengautentikasi pengguna nyata sebelum Anda menyerahkannya kepada pengembang.

    Contoh menggunakan URL publik gateway; untuk penyiapan Compose lokal tanpa ingress, ganti `http://localhost:8080` dalam dua pemeriksaan pertama. Pemeriksaan ketiga membuka `verification_uri_complete`, yang dibangun dari `public_url`, jadi untuk Compose lokal atur `public_url: http://localhost:8080` di `gateway.yaml`, dan tambahkan `http://localhost:8080/oauth/callback` sebagai URI pengalihan kedua pada klien OAuth dari langkah 1, karena gateway membangun IdP `redirect_uri` dari `public_url`. Tautan verifikasi kemudian terbuka di browser lokal Anda.

    Di Windows PowerShell, jalankan `curl.exe`; `curl` biasa adalah alias untuk `Invoke-WebRequest` dan menolak flag ini.

    Pertama, ambil dokumen penemuan, yang mengkonfirmasi gateway aktif, konfigurasi valid, dan semua pemeriksaan boot lulus:

    ```bash theme={null}
    curl -s https://claude-gateway.internal.example.com/.well-known/oauth-authorization-server | jq
    ```

    ```json theme={null}
    {
      "issuer": "https://claude-gateway.internal.example.com",
      "device_authorization_endpoint": "…/oauth/device_authorization",
      "token_endpoint": "…/oauth/token",
      "grant_types_supported": ["urn:ietf:params:oauth:grant-type:device_code", "refresh_token"]
    }
    ```

    Respons mencakup bidang tambahan, seperti `response_types_supported` dan `scopes_supported`.

    Kedua, minta otorisasi perangkat, yang mengkonfirmasi alur sign-in perangkat berfungsi dan Postgres dapat dijangkau dan dapat ditulis:

    ```bash theme={null}
    curl -s -X POST https://claude-gateway.internal.example.com/oauth/device_authorization | jq
    ```

    ```json theme={null}
    {
      "device_code": "…",
      "user_code": "WDJB-MJHT",
      "verification_uri": "https://claude-gateway.internal.example.com/device",
      "verification_uri_complete": "https://claude-gateway.internal.example.com/device?user_code=WDJB-MJHT",
      "expires_in": 600,
      "interval": 5
    }
    ```

    Ketiga, uji leg browser dengan membuka `verification_uri_complete` di browser dan mengkonfirmasi kode. Anda harus dialihkan ke halaman sign-in IdP Anda, dan setelah masuk, mendarat kembali di gateway dengan konfirmasi yang masuk.

    Gunakan pemeriksaan pertama yang gagal untuk menemukan masalahnya:

    * **Pemeriksaan pertama gagal**: boot tidak selesai; periksa stderr
    * **Pemeriksaan kedua gagal**: Postgres tidak dapat dijangkau dari gateway atau peran tidak dapat menulis; periksa string koneksi dan hibah
    * **Pemeriksaan ketiga tidak mencapai IdP**: periksa bahwa URI pengalihan IdP cocok dengan `https://<gateway>/oauth/callback` persis
    * **Pemeriksaan ketiga mencapai IdP tetapi memantul kembali dengan kesalahan**: baca log audit gateway, yang mencatat setiap penolakan auth dengan alasan, seperti `email domain not allowed`
  </Step>

  <Step title="Masukkan pengembang">
    Langkah terakhir ini terjadi pada mesin pengembang, bukan server. Atur `forceLoginMethod` ke `"gateway"` dan `forceLoginGatewayUrl` ke `public_url` gateway Anda dalam [file pengaturan terkelola](/docs/id/settings#settings-files) mesin itu, kemudian jalankan `/login`, tekan Enter pada layar **Cloud gateway**, dan selesaikan sign-in browser. [Atur URL gateway](#set-the-gateway-url) di bawah mencakup distribusi kedua kunci dalam skala besar.
  </Step>
</Steps>

<h2 id="connect-developers">
  Hubungkan pengembang
</h2>

Pengembang terhubung dari laptop mereka sendiri dengan satu sign-in browser, menggunakan akun kerja perusahaan mereka. Mereka tidak memerlukan akun claude.ai, kunci API, atau langganan, karena permintaan ke model melewati gateway menggunakan kredensial upstream organisasi. Koneksi didorong oleh [pengaturan terkelola sisi klien](/docs/id/claude-apps-gateway-config#client-side-managed-settings) yang Anda dorong melalui MDM, jadi tidak ada penyiapan manual di sisi pengembang; bagian ini mencakup apa yang dikonfigurasi admin.

CLI memfingerprintkan sertifikat daun TLS gateway pada koneksi pertama dan menyematkannya per nama host. Publikasikan sidik jari SHA-256 yang diharapkan bersama URL gateway sehingga pengembang memiliki sesuatu untuk dibandingkan. Dapatkan sidik jari dari file sertifikat dengan `openssl x509 -noout -fingerprint -sha256 -in cert.pem`; prompt `/login` menunjukkan 16 karakter pertama dari digest sebagai heksadesimal huruf kecil tanpa pemisah.

Ketika sertifikat berputar, setiap pengembang melihat prompt kepercayaan lagi, jadi perlakukan rotasi sebagai acara yang direncanakan dan publikasikan ulang sidik jari.

Setelah masuk, [pemilih model](/docs/id/model-config) menunjukkan model dalam daftar izin `availableModels` pengembang, pengaturan terkelola diterapkan saat startup dan refresh setiap jam, dan telemetri merutekan ke kolektor Anda. Sesi menyegarkan secara diam-diam sebelum kedaluwarsa `ttl_hours`, dan refresh yang gagal setelah deprovisi IdP meminta re-login.

<h3 id="set-the-gateway-url">
  Atur URL gateway
</h3>

Atur kedua kunci dalam file [pengaturan terkelola](/docs/id/settings#settings-files) per-OS yang Anda terapkan melalui MDM atau langsung di disk, dan `/login` terbuka langsung pada layar **Cloud gateway** dengan URL diisi:

```json theme={null}
{
  "forceLoginMethod": "gateway",
  "forceLoginGatewayUrl": "https://claude-gateway.internal.example.com"
}
```

Pengembang menekan Enter untuk terhubung. Prompt sidik jari TLS koneksi pertama masih muncul.

Tidak ada opsi gateway di pemilih login untuk pengembang memilih secara manual, dan `forceLoginGatewayUrl` diabaikan dalam file pengaturan pengembang mereka sendiri. `forceLoginMethod` saja, tanpa URL, meninggalkan pengembang pada pesan "Hubungi administrator IT Anda". Kedua kunci milik file yang Anda dorong ke mesin, bukan di blok `managed.policies[].cli` gateway, yang hanya menjangkau klien yang sudah terhubung.

<h3 id="ci-pipelines-and-remote-machines">
  Pipeline CI dan mesin jarak jauh
</h3>

Tidak ada alur token layanan untuk pipeline yang tidak diawasi. Sign-in gateway selalu menjalankan alur perangkat browser, jadi pekerjaan CI tanpa pengembang untuk menyetujui sign-in tidak dapat mengautentikasi; konfigurasikan mereka terhadap penyedia Anda secara langsung.

Setelah pengembang masuk, setiap invokasi Claude Code pada mesin itu menggunakan sesi gateway, termasuk run `claude -p` non-interaktif dan sesi yang dimulai oleh Agent SDK, dan [kebijakan gateway berlaku untuk semuanya](/docs/id/claude-apps-gateway-config#managed).

Alur perangkat memisahkan CLI polling dari browser yang menyetujui, jadi kotak pengembangan jarak jauh tanpa tampilan masih berfungsi: pengembang menjalankan `/login` melalui SSH pada mesin jarak jauh dan membuka tautan verifikasi di browser di laptop mereka.

<h3 id="what’s-enforced-on-developers">
  Apa yang diberlakukan pada pengembang
</h3>

Jaminan ini berlaku untuk setiap sesi gateway yang masuk.

* **Akses model**: permintaan untuk model yang tidak diberikan kebijakan mengembalikan 400, dan pemilih `/model` disaring ke daftar izin `availableModels` kebijakan. Atur [`enforceAvailableModels: true`](/docs/id/model-config#default-model-behavior) dalam kebijakan sehingga opsi Default diselesaikan ke model di dalam `availableModels` alih-alih ke default bawaan Claude Code; tanpanya, Default tetap dapat dipilih dan ditolak saat permintaan jika model itu tidak diberikan.
* **Tujuan telemetri**: ketika [penerusan telemetri](/docs/id/claude-apps-gateway-config#telemetry) dikonfigurasi, endpoint ekspor OTLP disematkan ke gateway, dan konfigurasi yang didorong gateway mengganti variabel `OTEL_*` yang ditetapkan secara lokal.
* **Kredensial**: token gateway adalah satu-satunya kredensial sesi. `ANTHROPIC_AUTH_TOKEN`, `ANTHROPIC_API_KEY`, `apiKeyHelper`, dan login claude.ai sebelumnya diabaikan saat masuk, jadi pengembang tidak perlu keluar dari claude.ai terlebih dahulu.
* **Pengaturan terkelola**: kunci terkunci tidak dapat ditimpa secara lokal. CLI menerapkan kebijakan saat startup dan pada setiap polling setiap jam.
* **Startup**: sesi yang masuk keluar saat startup dengan kesalahan setelah sekitar 10 detik ketika gateway tidak dapat dijangkau, daripada memulai tanpa pengaturan mereka.
* **Deprovisi**: sesi yang penggunanya dinonaktifkan di IdP kedaluwarsa dalam `ttl_hours` ketika refresh berikutnya gagal.

<h3 id="what-the-organization-can-see">
  Apa yang dapat dilihat organisasi
</h3>

Telemetri penggunaan membawa identitas pengembang, hitungan token, model, dan latensi ke kolektor organisasi. Gateway tidak mencatat atau menyimpan konten prompt atau penyelesaian. Apakah telemetri yang lebih kaya seperti log dan jejak dikumpulkan, yang dapat mencakup perintah dan jalur file, adalah pilihan organisasi [per-tujuan](/docs/id/claude-apps-gateway-config#telemetry).

<h2 id="availability-and-limitations">
  Ketersediaan dan keterbatasan
</h2>

Tabel mencakup fitur Claude Code mana yang bekerja ketika pengembang terhubung melalui gateway, dan apa yang didukung server gateway itu sendiri. Di mana sesuatu tidak didukung, kolom Catatan memberikan alternatif.

Gateway mengirimkan nilai [`anthropic-beta`](https://platform.claude.com/docs/en/api/beta-headers) yang dikirim CLI ke setiap upstream, jadi operator tidak mempertahankan daftar izin beta. Untuk Amazon Bedrock, yang mengabaikan header, gateway memindahkan nilai ke bidang `anthropic_beta` badan permintaan; upstream lainnya menerima header seperti yang dikirim.

Set beta sesi gateway CLI menghilangkan beta khusus pihak pertama dan beta extended-cache-ttl, itulah mengapa baris tersebut di bawah menunjukkan sebagai tidak tersedia.

| Fitur                                                                                                                   | Status         | Catatan                                                                                                                                                                                                                                                                                                                                                                                                                     |
| ----------------------------------------------------------------------------------------------------------------------- | -------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Penerusan inferensi (Amazon Bedrock, Claude Platform on AWS, Agent Platform Google Cloud, Microsoft Foundry, Anthropic) | Tersedia       | Dengan terjemahan model per-upstream dan failover. Upstream Amazon Bedrock menggunakan endpoint `bedrock-runtime` dan rantai kredensial default AWS; [endpoint Mantle](/docs/id/amazon-bedrock#use-the-mantle-endpoint) Amazon Bedrock bukan upstream yang didukung. [Upstream Claude Platform on AWS](/docs/id/claude-apps-gateway-config#claude-platform-on-aws) memerlukan Claude Code v2.1.198 atau lebih baru di server gateway. |
| Akses model dan pengaturan terkelola berdasarkan grup IdP                                                               | Tersedia       | Akses model diberlakukan di sisi server; pengaturan terkelola disampaikan per grup IdP dan diterapkan oleh CLI di [tingkat pengaturan terkelola](/docs/id/settings#settings-precedence)                                                                                                                                                                                                                                          |
| Fan-out telemetri (OTLP/HTTP)                                                                                           | Tersedia       | Identitas-stamped per ekspor; kedua pengkodean protobuf dan JSON                                                                                                                                                                                                                                                                                                                                                            |
| Penyedia identitas OIDC                                                                                                 | Tersedia       | Penyedia IdP yang sesuai dengan OIDC; gateway menjalankan penemuan OIDC standar dan alur kode otorisasi. Lihat [Pengaturan penyedia identitas](/docs/id/claude-apps-gateway-deploy#identity-provider-setup) untuk konfigurasi per-IdP                                                                                                                                                                                            |
| Batas pengeluaran per-pengguna dan per-grup                                                                             | Tersedia       | Lihat [Spend limits](/docs/id/claude-apps-gateway-spend-limits)                                                                                                                                                                                                                                                                                                                                                                  |
| Pencarian web sisi server                                                                                               | Tidak tersedia | CLI tidak dapat melihat penyedia upstream mana yang dirutekan gateway, jadi tidak dapat memverifikasi dukungan pencarian web dan menonaktifkan WebSearch pada sesi gateway                                                                                                                                                                                                                                                  |
| Prompt caching standar                                                                                                  | Tersedia       | Breakpoint `cache_control` diteruskan ke setiap upstream                                                                                                                                                                                                                                                                                                                                                                    |
| TTL cache 1 jam                                                                                                         | Tidak tersedia | CLI menghilangkan beta extended-cache-ttl pada sesi gateway, karena tidak setiap upstream yang dapat dirutekan gateway mendukung TTL 1 jam, jadi prompt caching melalui gateway menggunakan TTL 5 menit; lihat catatan header beta di atas                                                                                                                                                                                  |
| Mode Auto                                                                                                               | Tersedia       | Mengikuti [aturan penyedia pihak ketiga](/docs/id/permission-modes#enable-auto-mode-on-bedrock-agent-platform-or-foundry): hanya model yang memenuhi syarat di penyedia pihak ketiga yang dapat menggunakannya. Sebelum v2.1.207, mode auto pada sesi gateway memerlukan pengaturan `CLAUDE_CODE_ENABLE_AUTO_MODE=1`, dapat dikirimkan melalui blok `env` kebijakan terkelola                                                    |
| Optimasi khusus pihak pertama seperti cakupan cache global dan alat yang efisien token                                  | Tidak tersedia | CLI tidak mengaktifkannya pada sesi gateway; lihat catatan header beta di atas                                                                                                                                                                                                                                                                                                                                              |
| OTLP/gRPC                                                                                                               | Tidak didukung | OTLP melalui HTTP saja                                                                                                                                                                                                                                                                                                                                                                                                      |
| SAML, LDAP, dan auth non-OIDC lainnya                                                                                   | Tidak didukung | OIDC saja. Depan dengan jembatan OIDC jika diperlukan                                                                                                                                                                                                                                                                                                                                                                       |
| Multi-tenant (beberapa penerbit OIDC)                                                                                   | Tidak didukung | Satu penerbit per gateway. Jalankan instans terpisah                                                                                                                                                                                                                                                                                                                                                                        |
| Server Windows                                                                                                          | Tidak didukung | Terapkan di Linux. macOS untuk pengembangan lokal saja                                                                                                                                                                                                                                                                                                                                                                      |
| Helm chart                                                                                                              | Tidak tersedia | Gateway berjalan sebagai Deployment stateless standar; lihat [panduan penyebaran](/docs/id/claude-apps-gateway-deploy#kubernetes)                                                                                                                                                                                                                                                                                                |
| Admin UI                                                                                                                | Tidak tersedia | Konfigurasi adalah file YAML; terapkan ulang untuk mengubahnya                                                                                                                                                                                                                                                                                                                                                              |

<h2 id="next-steps">
  Langkah berikutnya
</h2>

Quickstart meninggalkan Anda dengan konfigurasi minimal yang berjalan di bawah Docker Compose. Untuk membawanya lebih jauh:

* Perluas `gateway.yaml` di luar konfigurasi minimal, misalnya untuk menambahkan RBAC per-grup, failover multi-upstream, atau tujuan telemetri. [Referensi konfigurasi](/docs/id/claude-apps-gateway-config) mencakup setiap opsi.
* Pindah dari Compose ke penyebaran produksi di Kubernetes atau Cloud Run, siapkan IdP Anda dengan benar, dan tinjau model keamanan. [Panduan penyebaran dan operasi](/docs/id/claude-apps-gateway-deploy) mencakup penyiapan per-IdP, persyaratan gambar kontainer, probe kesehatan, dan pemecahan masalah.
* Letakkan batas pengeluaran pada pengembang atau grup individual sehingga beban kerja yang liar tidak dapat mengonsumsi seluruh komitmen Anda. [Spend limits](/docs/id/claude-apps-gateway-spend-limits) mencakup API admin dan cara penegakan bekerja.
* Untuk contoh lengkap yang dikerjakan di Google Cloud, dengan Cloud Run, Cloud SQL, dan Secret Manager, lihat [Deploy on Google Cloud](/docs/id/claude-apps-gateway-on-gcp).
