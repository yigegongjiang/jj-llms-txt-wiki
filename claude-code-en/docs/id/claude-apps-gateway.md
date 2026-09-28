> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude apps gateway untuk Amazon Bedrock, Claude Platform di AWS, Google Cloud, dan Microsoft Foundry

> Jalankan Claude Code melalui Amazon Bedrock, Claude Platform di AWS, Google Cloud, atau Microsoft Foundry di balik gateway yang di-host sendiri dengan SSO sign-in, akses model per-grup, dan telemetri OTLP.

<Note>
  Claude apps gateway dirancang untuk organisasi yang harus — atau lebih suka — merutekan inferensi melalui penyedia cloud mereka sendiri, misalnya untuk memenuhi persyaratan [residensi data](/docs/id/claude-apps-gateway-deploy#compliance-posture). Jika Anda tidak memiliki persyaratan ini, dan menginginkan akses ke fitur lain seperti penyediaan SCIM atau Claude Code di web dan mobile, Claude Enterprise mungkin lebih cocok. Lihat halaman [ketersediaan fitur](/docs/id/feature-availability) untuk perbandingan lengkap semua metode penyebaran.
</Note>

Claude apps gateway adalah layanan yang di-host sendiri yang berada di antara klien Claude Code pengembang Anda dan penyedia model Anda. Pengembang masuk dengan penyedia identitas perusahaan Anda (IdP) alih-alih menyimpan kunci API atau kredensial cloud. Gateway menyimpan kredensial upstream, memberlakukan akses model dan [pengaturan terkelola](/docs/id/managed-settings) berdasarkan grup IdP, dan meneruskan telemetri penggunaan ke tumpukan observabilitas Anda sendiri.

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
* **Kontrol akses**: grup IdP Anda memetakan ke daftar izin model dan kebijakan [pengaturan terkelola](/docs/id/managed-settings). Gateway memberlakukan akses model di sisi server, menolak permintaan untuk model yang tidak diberikan, dan memilih kebijakan pengaturan terkelola setiap grup, yang diterapkan CLI di [tingkat pengaturan terkelola](/docs/id/settings#settings-precedence). Tim yang berbeda mendapatkan model, alat, dan izin yang berbeda, dan pengembang tidak dapat mengganti apa yang dikunci kebijakan mereka.
* **Pengiriman pengaturan**: gateway mengirimkan pengaturan terkelola ke klien yang masuk sendiri, menggantikan [pengaturan yang dikelola server](/docs/id/server-managed-settings) dari konsol admin claude.ai.
* **Telemetri**: setiap tujuan yang dikonfigurasi menerima [metrik OpenTelemetry Protocol (OTLP)](/docs/id/monitoring-usage) dengan hitungan token, model, identitas pengguna, dan latensi secara default, dengan log dan jejak sebagai opt-in per-tujuan.
* **Perutean upstream**: klien berbicara API Pesan Anthropic ke gateway, dan gateway menerjemahkan untuk setiap upstream, baik Amazon Bedrock, [Claude Platform on AWS](/docs/id/claude-platform-on-aws), Agent Platform Google Cloud, Microsoft Foundry, atau API Anthropic, dengan failover di antara mereka. Anda dapat mengubah wilayah, penyedia, atau urutan failover tanpa pengembang menyadari atau mengonfigurasi ulang.

<Frame>
  <img src="https://mintcdn.com/claude-code/VbyXug8hBU9UK6oT/images/claude-gateway-architecture.svg?fit=max&auto=format&n=VbyXug8hBU9UK6oT&q=85&s=9e4f1190fc56718144190a3db61c63af" alt="Diagram menunjukkan klien Claude Code dan tab Chat, Cowork, dan Code Claude Desktop terhubung melalui HTTPS dengan token bearer ke gateway Claude apps yang di-host sendiri di dalam infrastruktur Anda, yang menandatangani pengguna terhadap IdP Anda, menyimpan status auth di PostgreSQL, meneruskan telemetri ke kolektor OTLP Anda, dan meneruskan inferensi ke Amazon Bedrock, Claude Platform on AWS, Google Cloud, Microsoft Foundry, atau API Anthropic" width="760" height="320" data-path="images/claude-gateway-architecture.svg" />
</Frame>

<Note>
  Bidang data gateway sendiri tidak mengirim apa pun ke infrastruktur Anthropic kecuali API Anthropic adalah upstream yang dikonfigurasi. Anda mengontrol ke mana telemetri, log audit, pengaturan terkelola, dan identitas IdP pengembang Anda pergi, dan gateway tidak mengirimkan salah satu dari mereka ke Anthropic. Untuk lalu lintas yang tersisa proses CLI dapat mengirim dan cara menutupnya, lihat [Compliance posture](/docs/id/claude-apps-gateway-deploy#compliance-posture).
</Note>

Untuk fitur Claude Code mana yang bekerja melalui gateway dan apa yang didukung server itu sendiri, lihat [Ketersediaan dan keterbatasan](#availability-and-limitations) di bawah. Untuk keputusan seperti biaya, bypass, menjalankan beberapa gateway, dan platform serverless, lihat [panduan penyebaran](/docs/id/claude-apps-gateway-deploy#deployment).

<h3 id="other-gateway-implementations">
  Implementasi gateway lainnya
</h3>

Jika Anda sudah menjalankan gateway LLM atau gateway API yang memenuhi kebutuhan Anda, terus gunakan; [Gateway LLM lainnya](/docs/id/llm-gateway) mencakup konfigurasi Claude Code terhadapnya.

[Panduan kompatibilitas gateway](/docs/id/llm-gateway-protocol) mendokumentasikan apa yang diharapkan Claude Code dari gateway apa pun: endpoint yang dipanggilnya, header dan bidang body untuk diteruskan, dan apa yang berhenti bekerja ketika mereka dihapus. Gateway Claude apps yang berjalan juga melayani referensi protokolnya sendiri di `GET /protocol`, yang menjelaskan endpoint yang dieksposnya ke klien Claude Code: SSO sign-in, inferensi, pengiriman pengaturan terkelola, penemuan model, dan telemetri. Ambilnya dengan `curl https://claude-gateway.internal.example.com/protocol` dari gateway yang digunakan apa pun, seperti yang dihasilkan [quickstart](#quickstart) di bawah. Perubahan breaking pada protokol diumumkan sebelumnya, tetapi kompatibilitas backward yang tidak terbatas tidak dijamin.

<h2 id="quickstart">
  Quickstart
</h2>

Quickstart ini berjalan di jalur minimal: daftarkan klien OAuth di IdP Anda, tulis `gateway.yaml`, jalankan gateway bersama Postgres dengan Docker Compose, dan verifikasi sign-in end to end. Ini menggunakan upstream Amazon Bedrock; Claude Platform on AWS, Agent Platform Google Cloud, Microsoft Foundry, dan API Anthropic sama-sama didukung dengan menukar blok `upstreams` seperti yang ditunjukkan dalam [referensi konfigurasi](/docs/id/claude-apps-gateway-config#upstreams). Di akhir Anda memiliki gateway yang dapat `/login` pengembang.

<Note>
  **Terapkan di jaringan pribadi Anda.** Claude Code hanya terhubung ke gateway yang alamatnya pribadi. Ini adalah penjaga keamanan, karena gateway yang dipercaya dapat mendorong pengaturan yang menjalankan perintah pada mesin pengembang. Letakkan gateway di balik load balancer internal atau VPN dan berikan nama host yang hanya diselesaikan ke IP pribadi. Jika jaringan internal Anda bernomor dari ruang IPv4 publik yang dimiliki organisasi Anda, lihat [Izinkan gateway pada ruang alamat publik yang Anda miliki](#allow-a-gateway-on-public-address-space-you-own).
</Note>

<h3 id="prerequisites">
  Prasyarat
</h3>

Miliki ini sebelum Anda mulai:

| Anda membutuhkan                         | Detail                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| ---------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Claude Code v2.1.195 atau lebih baru     | Subperintah `claude gateway` dan alur sign-in gateway dikirim di v2.1.195. Build publik sebelumnya tidak menyertakannya. Baik mesin yang menjalankan server gateway maupun mesin setiap pengembang harus pada v2.1.195 atau lebih baru; jalankan `claude update` untuk mendapatkan rilis terbaru. Upstream [Claude Platform on AWS](/docs/id/claude-apps-gateway-config#claude-platform-on-aws) memerlukan Claude Code v2.1.198 atau lebih baru di server gateway.                                                                                                                                                                                                                                                                                                                                                                                                    |
| Penyedia identitas OpenID Connect (OIDC) | Okta, Microsoft Entra ID, Google Workspace, Keycloak, atau Dex, atau IdP yang sesuai dengan OIDC lainnya seperti PingFederate. Gateway menjalankan penemuan OIDC standar dan alur kode otorisasi terhadapnya. SAML dan LDAP tidak didukung.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| PostgreSQL 14 atau lebih baru            | Mendukung alur sign-in perangkat, di mana callback browser menulis dan CLI polling membaca, ditambah penghitung batas laju. Postgres yang dikelola apa pun berfungsi, termasuk tingkat terkecil. Tanpa batas pengeluaran yang dikonfigurasi, gateway menyimpan beberapa KB status auth berumur pendek; dengan [batas pengeluaran](/docs/id/claude-apps-gateway-spend-limits), ia juga menyimpan tabel pengeluaran, audit, dan identitas yang tahan lama yang harus dicadangkan. TLS melalui `?sslmode=require` direkomendasikan.                                                                                                                                                                                                                                                                                                                                      |
| Model upstream                           | Kredensial Amazon Bedrock, kredensial Claude Platform on AWS, kredensial Google Cloud, sumber daya Microsoft Foundry, atau kunci API Anthropic. Beberapa upstream didukung dengan failover.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| HTTPS                                    | Gateway harus dapat dijangkau melalui `https://` dari laptop pengembang dan dari browser apa pun yang digunakan untuk sign-in; gateway melayani halaman verifikasi perangkat pada pendengar yang sama. Berikan sertifikat TLS melalui `listen.tls` atau jalankan di balik ingress yang menghentikan TLS, dan atur `listen.public_url` ke asal eksternal dalam kedua kasus. Asal `http://` biasa hanya diterima ketika host gateway adalah loopback: `localhost`, `127.0.0.1`, atau `::1`.                                                                                                                                                                                                                                                                                                                                                                        |
| Alamat jaringan pribadi                  | Di `/login`, Claude Code memerlukan nama host atau alamat IP gateway untuk diselesaikan hanya ke alamat pribadi: RFC 1918, link-local, CGNAT `100.64.0.0/10`, IPv6 ULA `fc00::/7`, atau loopback. Untuk gateway yang Anda hosting, alamat publik apa pun di luar blok yang Anda deklarasikan ditolak; lihat [model ancaman](/docs/id/claude-apps-gateway-deploy#threat-model-summary) dalam panduan penyebaran. Jika mesin pengembang merutekan HTTPS melalui proxy perusahaan, sign-in juga memerlukan host proxy untuk diselesaikan ke alamat pribadi; jika tidak, tambahkan host gateway ke `NO_PROXY` sehingga CLI terhubung langsung. Jika jaringan internal Anda bernomor dari ruang IPv4 publik yang dimiliki organisasi Anda, [deklarasikan blok-blok tersebut](#allow-a-gateway-on-public-address-space-you-own) sehingga `/login` menerima gateway di sana. |
| Runtime Linux                            | Server gateway hanya berjalan pada biner Linux asli. macOS berfungsi untuk pengembangan lokal. Windows tidak didukung sebagai platform server.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |

<h3 id="steps">
  Langkah-langkah
</h3>

<Steps>
  <Step title="Daftarkan klien OAuth di IdP Anda">
    Tentukan nama host gateway terlebih dahulu, karena URI pengalihan harus cocok dengannya. Buat aplikasi web OIDC baru dan atur URI pengalihan ke `https://claude-gateway.<your-domain>/oauth/callback`, di mana host adalah nilai yang sama yang Anda atur sebagai [`listen.public_url`](/docs/id/claude-apps-gateway-config#listen) di langkah 3. Catat `client_id` dan `client_secret`. Instruksi per-IdP ada di [Identity provider setup](/docs/id/claude-apps-gateway-deploy#identity-provider-setup).
  </Step>

  <Step title="Sediakan database PostgreSQL">
    Postgres 14 atau lebih baru apa pun berfungsi, termasuk tingkat terkelola terkecil. Gateway menjalankan migrasi skema sendiri saat boot, jadi peran database memerlukan hak untuk membuat dan mengubah tabel; lihat [`store`](/docs/id/claude-apps-gateway-config#store).
  </Step>

  <Step title="Tulis gateway.yaml">
    Rahasia dibaca melalui ekspansi `${ENV_VAR}` sehingga file itu sendiri dapat hidup dalam kontrol versi. Gunakan nama host `public_url` yang diselesaikan ke IP pribadi di jaringan Anda, karena `/login` menolak alamat publik. Konfigurasi minimal memiliki lima bagian, dan setiap bidang lainnya memiliki default:

    ```yaml gateway.yaml theme={null}
    listen:
      host: 0.0.0.0
      port: 8080
      # Diperlukan kecuali host adalah alamat loopback. Digunakan untuk IdP
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
      Upstream Amazon Bedrock memerlukan principal AWS dengan `bedrock:InvokeModel` dan `bedrock:InvokeModelWithResponseStream` pada ARN `inference-profile/us.anthropic.*` dan ARN `foundation-model/anthropic.*` yang mendasar. Ini juga memerlukan formulir kasus penggunaan satu kali Anthropic yang dikirimkan untuk akun dari katalog Model konsol Bedrock.

      Sediakan kredensial dengan IRSA di EKS, peran tugas ECS, atau profil instans EC2 daripada kunci statis. [Referensi `upstreams`](/docs/id/claude-apps-gateway-config#upstreams) memiliki detail IAM lengkap, matriks kredensial lintas cloud, dan blok `auth` untuk penyedia lain.
    </Note>
  </Step>

  <Step title="Jalankan">
    Bangun gambar kontainer di sekitar biner `claude` yang memenuhi [persyaratan gambar](/docs/id/claude-apps-gateway-deploy#container-image), kemudian jalankan bersama Postgres. File Compose mereferensikan gambar sebagai `registry.example.com/claude-gateway:2.1.198`; ganti dengan registry dan tag gambar Anda sendiri:

    ```yaml docker-compose.yaml theme={null}
    services:
      gateway:
        image: registry.example.com/claude-gateway:2.1.198
        ports: ["8080:8080"]
        volumes: ["./gateway.yaml:/etc/claude/gateway.yaml:ro"]
        environment:
          OIDC_CLIENT_SECRET: ${OIDC_CLIENT_SECRET}
          GATEWAY_JWT_SECRET: ${GATEWAY_JWT_SECRET}
          GATEWAY_POSTGRES_URL: postgres://gw:pw@postgres/gateway
          # Kredensial AWS: dalam produksi, hilangkan ini dan gunakan peran instans.
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

    Gateway adalah biner Linux tunggal yang membaca konfigurasi, terhubung ke Postgres dan menerapkan migrasi skemanya, menjalankan penemuan OIDC terhadap IdP Anda, membangun klien upstream, dan mulai mendengarkan.

    Boot gagal-tertutup untuk konfigurasi, koneksi Postgres, penemuan OIDC, dan konstruksi klien upstream. Jika salah satu dari mereka tidak dapat dijangkau atau salah konfigurasi, gateway keluar dengan kesalahan daripada melayani lalu lintas dalam keadaan terdegradasi.

    Boot yang berhasil tidak memvalidasi jalur inferensi, karena kredensial instans Bedrock dan Agent Platform diselesaikan pada permintaan pertama, bukan saat boot.

    Tonton stderr untuk urutan boot. Baris log menggunakan format `[gateway] <timestamp> <level> <message>`, acara audit adalah JSON satu baris dengan bidang `evt`, dan spanduk startup, dihilangkan di bawah, dicetak di antara baris migrasi dan mendengarkan. Database segar mencetak satu baris `migration N applied` per migrasi skema; database yang sudah dimigrasikan tidak mencetak apa pun. Anda harus melihat, dalam urutan:

    ```text theme={null}
    {"ts":"2026-06-10T17:03:21.114Z","evt":"config.load","path":"/etc/claude/gateway.yaml","sha256":"…"}
    [gateway] 2026-06-10T17:03:21.395Z info waiting for migration lock (another replica may be migrating; check pg_locks for key 6775156 if this persists)
    [gateway] 2026-06-10T17:03:21.408Z info migration 1 applied
    …
    [gateway] 2026-06-10T17:03:21.431Z info migration 6 applied
    [gateway] 2026-06-10T17:03:21.512Z info claude gateway listening on http://0.0.0.0:8080
    ```

    Gateway juga mencatat peringatan bahwa `access_control.allow_cidrs` kosong. Itu diharapkan di sini, karena tidak ada yang membatasi alamat klien mana yang dilayani gateway sampai Anda menetapkan daftar izin. [Referensi `access_control`](/docs/id/claude-apps-gateway-config#http-tuning) memiliki rentang yang direkomendasikan.

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
    Langkah terakhir ini terjadi pada mesin pengembang, bukan server. Atur `forceLoginMethod` ke `"gateway"` dan `forceLoginGatewayUrl` ke `public_url` gateway Anda dalam [file pengaturan terkelola](/docs/id/managed-settings#delivery-mechanisms) mesin itu, kemudian jalankan `/login`, tekan Enter pada layar **Cloud gateway**, dan selesaikan sign-in browser. [Atur URL gateway](#set-the-gateway-url) di bawah mencakup distribusi kedua kunci ke setiap mesin pengembang.
  </Step>
</Steps>

<h2 id="connect-developers">
  Hubungkan pengembang
</h2>

Pengembang terhubung dari laptop mereka sendiri dengan satu kali masuk browser, menggunakan akun kerja perusahaan mereka. Mereka tidak memerlukan akun claude.ai, kunci API, atau langganan, karena permintaan ke model melewati gateway menggunakan kredensial upstream organisasi. Koneksi didorong oleh [pengaturan yang dikelola sisi klien](/docs/id/claude-apps-gateway-config#client-side-managed-settings) yang Anda dorong melalui MDM, jadi tidak ada pengaturan manual di sisi pengembang; bagian ini mencakup apa yang dikonfigurasi admin.

CLI memvalidasi sertifikat TLS leaf gateway pada koneksi pertama dan menyematkannya per nama host. Ini memeriksa pin tersebut lagi selama masuk, pada penyegaran sesi senyap, dan pada pengambilan pengaturan yang dikelola, sementara permintaan inferensi menggunakan validasi TLS standar tanpa pin. Permintaan yang dirutekan melalui proxy HTTPS melewati pemeriksaan pin, jadi tambahkan host gateway ke `NO_PROXY` untuk menjaganya tetap langsung.

Publikasikan sidik jari SHA-256 yang diharapkan bersama URL gateway sehingga pengembang memiliki sesuatu untuk dibandingkan. Prompt `/login` menampilkan 16 karakter pertama dari sidik jari sebagai heksadesimal huruf kecil tanpa titik dua. Untuk mencetak sidik jari lengkap dalam bentuk itu dari file sertifikat, jalankan:

```bash theme={null}
openssl x509 -noout -fingerprint -sha256 -in cert.pem | cut -d= -f2 | tr -d : | tr 'A-F' 'a-f'
```

Ketika sertifikat berputar, setiap pengembang melihat prompt kepercayaan lagi, jadi perlakukan rotasi sebagai acara yang direncanakan dan publikasikan ulang sidik jari. Jika kebijakan gateway Anda mencakup [pengaturan yang memerlukan persetujuan](/docs/id/server-managed-settings#security-approval-dialogs), pengembang juga melihat dialog persetujuan itu lagi setelah menerima sertifikat baru, karena Claude Code [memori persetujuan](/docs/id/server-managed-settings#approval-memory) kunci ke sertifikat yang disematkan.

Gateway dapat mengembalikan bidang `email` opsional dalam respons token-nya untuk menamai akun yang digunakan masuk. Ketika itu terjadi, pengembang mengonfirmasi akun sebelum Claude Code menyimpan kredensial. Setelah masuk yang dikonfirmasi, `/status` menampilkan akun.

Konfirmasi memerlukan Claude Code v2.1.275 atau lebih baru di mesin pengembang; klien di bawah versi itu mengabaikan bidang. Server gateway dalam biner `claude` tidak mengembalikan bidang, jadi masuk-nya selesai tanpa konfirmasi.

Setelah pengembang masuk, [pemilih model](/docs/id/model-config) menampilkan model dalam daftar izin `availableModels` mereka. Pengaturan yang dikelola diterapkan saat startup dan menyegarkan setiap jam, dan telemetri dirutekan ke kolektor Anda.

Sesi menyegarkan secara senyap sebelum kedaluwarsa `ttl_hours`. Ketika penyegaran gagal setelah pemberhentian IdP, Claude Code meminta pengembang untuk masuk lagi.

<h3 id="set-the-gateway-url">
  Atur URL gateway
</h3>

Tiga kunci masuk ke file [pengaturan yang dikelola](/docs/id/managed-settings#delivery-mechanisms) per-OS yang Anda terapkan melalui MDM atau langsung di disk. `forceLoginMethod` dan `forceLoginGatewayUrl` membuka `/login` langsung di layar **Cloud gateway** dengan URL terisi, dan `parentSettingsBehavior: "merge"` memungkinkan Claude Desktop mengirimkan daftar izin egress gateway ke sesi Claude Code yang diluncurkannya, dijelaskan dalam [Kirimkan kebijakan ke sesi Claude Desktop](#deliver-policy-to-claude-desktop-sessions):

```json theme={null}
{
  "forceLoginMethod": "gateway",
  "forceLoginGatewayUrl": "https://claude-gateway.internal.example.com",
  "parentSettingsBehavior": "merge"
}
```

Pengembang menekan Enter untuk terhubung. Prompt [sidik jari TLS koneksi pertama](#connect-developers) masih muncul. Setelah file berada di mesin, pengembang yang belum menyelesaikan masuk gateway melihat salah satu pesan yang dijelaskan di bawah [Kebijakan administrator memerlukan masuk Cloud gateway](/docs/id/errors#administrator-policy-requires-a-cloud-gateway-sign-in). Pengembang yang memilih penyedia cloud melalui variabel lingkungan seperti `CLAUDE_CODE_USE_BEDROCK` tidak memerlukan masuk gateway.

Pengembang tidak dapat mengatur ini secara manual. Pemilih masuk tidak memiliki opsi gateway, dan `forceLoginGatewayUrl` diabaikan dalam file pengaturan pengembang sendiri. `forceLoginMethod` saja, tanpa URL, meninggalkan pengembang di pesan "Hubungi administrator IT Anda". Kunci masuk milik file yang Anda dorong ke mesin, bukan di blok `managed.policies[].cli` gateway, yang hanya menjangkau klien yang sudah terhubung.

<h3 id="allow-a-gateway-on-public-address-space-you-own">
  Izinkan gateway pada ruang alamat publik yang Anda miliki
</h3>

Beberapa organisasi menomori jaringan internal mereka dari blok IPv4 publik yang mereka miliki, seperti ruang alamat operator sendiri atau `/8` warisan, jadi gateway mereka tidak dapat memiliki alamat pribadi. Daftarkan blok tersebut dalam pengaturan yang dikelola `gatewayInternalNetworks`. `/login` kemudian menerima gateway di dalam blok yang terdaftar ketika mesin pengembang terhubung ke dalamnya dari alamat di dalam blok yang sama. Ini memerlukan Claude Code v2.1.268 atau lebih baru di mesin pengembang; versi sebelumnya mengabaikan kunci dan menerapkan aturan alamat pribadi.

<Warning>
  `gatewayInternalNetworks` adalah untuk jaringan internal yang kebetulan dinomori dari ruang alamat publik. Itu tidak membuat aman untuk mengekspos gateway ke internet: gateway yang dipercaya dapat mendorong pengaturan yang menjalankan perintah di mesin pengembang.

  Jaga gateway tidak dapat dijangkau dari luar jaringan Anda dengan aturan firewall atau load balancer Anda. Atur [`access_control.allow_cidrs`](/docs/id/claude-apps-gateway-config#http-tuning) gateway ke blok yang sama yang Anda deklarasikan di sini, jadi gateway itu sendiri menolak klien dari tempat lain. Di belakang load balancer atau ingress, atur `listen.trusted_proxies` ke front end itu juga, karena gateway sebaliknya mencocokkan `allow_cidrs` terhadap alamat front end itu sendiri daripada pengembang.
</Warning>

Tambahkan kunci ke sumber pengaturan yang dikelola yang sama dengan kunci masuk: file pengaturan yang dikelola, profil MDM, atau kebijakan registri. Claude Code mengabaikannya dalam pengaturan pengguna, proyek, dan yang dikelola server.

Contoh ini mendeklarasikan satu blok. Ganti `203.0.113.0/24` dengan blok Anda sendiri. Ini adalah rentang dokumentasi, dan Claude Code menolak yang tersebut.

```json theme={null}
{
  "gatewayInternalNetworks": ["203.0.113.0/24"]
}
```

Claude Code memvalidasi daftar di `/login` sebelum menghubungi gateway apa pun:

* Setiap entri adalah blok IPv4 yang ditulis sebagai alamat pertamanya dan awalan dari `/8` hingga `/32`.
* Daftar menampung paling banyak empat blok, dan tidak ada dua yang tumpang tindih.
* Tidak ada blok yang tumpang tindih dengan ruang alamat pribadi: `10.0.0.0/8`, `172.16.0.0/12`, `192.168.0.0/16`, `127.0.0.0/8`, `169.254.0.0/16`, dan `100.64.0.0/10`. `/login` sudah menerima gateway di sana tanpa kunci ini.
* Tidak ada blok yang tumpang tindih dengan ruang yang tidak pernah menjadi jaringan organisasi: `198.18.0.0/15` dan `192.0.0.0/24`, yang dipegang klien VPN dan NAT64 sebagai alamat lokal; rentang dokumentasi `192.0.2.0/24`, `198.51.100.0/24`, dan `203.0.113.0/24`; dan rentang yang dicadangkan `0.0.0.0/8`, `192.88.99.0/24`, dan multicast `224.0.0.0/4`. Anda dapat mendeklarasikan blok di dalam `240.0.0.0/4`, yang digunakan beberapa jaringan besar sebagai ruang unicast internal.

Blok dari `managed-settings.json` dan file drop-in `managed-settings.d/` menggabungkan menjadi satu daftar, dan batas ini berlaku untuk daftar gabungan. Untuk mempersempit blok, ganti entrinya daripada menambahkan yang kedua, tumpang tindih dalam drop-in; `/login` menolak tumpang tindih.

Jika entri melanggar aturan, atau nilainya bukan daftar string, Claude Code menolak setiap masuk gateway baru di mesin itu dan menamai masalahnya dalam pesan. Masuk ke gateway pada alamat pribadi gagal juga, dan masuk yang ada tetap berfungsi. Coba nilai pada satu mesin sebelum Anda menerapkannya. Claude Code juga mencantumkan nilai yang salah ketik di antara [pengaturan yang dikelola tidak valid yang dilaporkannya](/docs/id/managed-settings#keys-that-fail-closed).

Dengan daftar yang valid, `/login` menerapkan tiga pemeriksaan ke gateway yang alamatnya berada di dalam blok yang terdaftar:

* Setiap alamat yang diselesaikan nama host gateway berada di dalam blok itu saja. Claude Code menolak nama yang juga memiliki catatan di luarnya, alamat pribadi dan IPv6 disertakan.
* Mesin pengembang terhubung dari dalam blok yang sama. Claude Code menolak mesin di belakang NAT, di dalam kontainer atau WSL2, atau di VPN yang kumpulan alamatnya berada di luar blok, dan menamai alamat yang terhubung mesin dari.
* Koneksi bersifat langsung. Jika `HTTPS_PROXY` berlaku untuk host gateway, `/login` menolak dan menamai entri `NO_PROXY` untuk ditambahkan.

Ketika ketiga hal tersebut lulus, [prompt kepercayaan](#connect-developers) menambahkan baris yang menamai alamat mesin, alamat gateway, dan blok yang dideklarasikan yang berisi keduanya.

Kunci tidak mengubah apa pun untuk gateway lainnya: masuk ke yang ada di alamat pribadi berfungsi seperti sebelumnya, dan masuk ke yang ada di alamat publik di luar setiap blok yang terdaftar ditolak seperti sebelumnya.

Blok yang dideklarasikan mempersempit siapa yang dapat masuk tetapi tidak membuktikan di mana mesin berada, jadi deklarasikan hanya ruang alamat yang dikendalikan organisasi Anda. Blok yang dibagikan dengan penyewa lain, seperti rentang publik penyedia cloud, membiarkan siapa pun di dalamnya lulus pemeriksaan yang sama.

<h3 id="deliver-policy-to-claude-desktop-sessions">
  Kirimkan kebijakan ke sesi Claude Desktop
</h3>

Claude Desktop menjalankan tab Cowork dan Code-nya, ditambah tab Chat ketika Anda mengaktifkannya, pada sesi Claude Code tertanam dan mengirimkan permintaan model mereka melalui gateway. Ini melewatkan kebijakan ke setiap sesi tersebut, dibangun dari konfigurasi yang dilayani gateway di `/user/bootstrap`: daftar izin model, alat yang dinonaktifkan, dan daftar izin egress yang berasal dari blok `cli` kebijakan yang cocok, ditambah [overlay `desktop`](/docs/id/claude-apps-gateway-config#claude-desktop-overlay).

Kunci `cli` lainnya, seperti hooks, `env`, dan aturan izin berskop seperti `Bash(npm *)`, hanya menjangkau klien yang masuk melalui `/login`. Claude Desktop membaca URL gateway dari konfigurasi yang dikelolanya sendiri dan masuk dengan alurnya sendiri, terpisah dari kunci `forceLoginMethod` dan `forceLoginGatewayUrl` dalam [Atur URL gateway](#set-the-gateway-url).

Pengaturan yang dilewatkan oleh proses peluncur adalah pengaturan induk. Claude Code mengabaikan pengaturan induk di mesin apa pun yang memiliki sumber yang dikelola yang diterapkan admin, kecuali [sumber yang mengirimkan kebijakan](/docs/id/managed-settings#which-managed-source-claude-code-uses) menetapkan `parentSettingsBehavior: "merge"`.

<h4 id="which-machines-need-the-opt-in">
  Mesin mana yang memerlukan opt-in
</h4>

Mesin yang hanya menjalankan Claude Desktop membutuhkannya. Claude Desktop menerapkan daftar model dan daftar alat yang dinonaktifkan ke sesi tertanam itu sendiri, tetapi daftar izin egress menjangkaunya hanya sebagai pengaturan induk, dalam bentuk aturan domain `WebFetch` dan aturan jaringan sandbox. Tanpa opt-in, sesi tersebut berjalan tanpa pembatasan egress, dan tidak ada yang memperingatkan Anda. Gateway masih menolak permintaan inferensi untuk model yang tidak diizinkan kebijakan.

Mesin tempat pengembang masuk melalui `/login` tidak membutuhkannya; setiap sesi Claude Code mengambil kebijakannya dari gateway.

Armada yang [`policyHelper`](/docs/id/settings-reference#policyhelper) mereka suplai pengaturan yang dikelola tidak dapat menggunakannya: Claude Code tidak pernah menggabungkan pengaturan induk pada armada tersebut, karena membaca pengaturan yang dikelola dari output helper saja.

<h4 id="set-the-opt-in">
  Atur opt-in
</h4>

Terapkan cuplikan pengaturan yang dikelola dari [Atur URL gateway](#set-the-gateway-url), cerminkan ke sumber sisi klien apa pun yang melampaui file, kemudian verifikasi.

<Steps>
  <Step title="Terapkan opt-in dalam file pengaturan yang dikelola">
    [Cuplikan di atas](#set-the-gateway-url) sudah mencakup `parentSettingsBehavior: "merge"`, jadi file yang Anda dorong ke mesin membawanya.
  </Step>

  <Step title="Cerminkan cuplikan ke sumber apa pun yang melampaui file">
    Claude Code membaca `parentSettingsBehavior` hanya dari [sumber yang dipilih](/docs/id/managed-settings#which-managed-source-claude-code-uses). Menambahkan kunci kebijakan apa pun ke sumber dapat membuat sumber itu menjadi sumber yang dipilih, jadi dalam sumber sisi klien, cerminkan seluruh cuplikan daripada `parentSettingsBehavior` saja. [Pengaturan yang dikelola sisi klien](/docs/id/claude-apps-gateway-config#client-side-managed-settings) mencakup armada yang mengirimkan kebijakan melalui Group Policy atau profil konfigurasi. Plist preferensi yang dikelola di macOS atau kebijakan HKLM di Windows melampaui file `managed-settings.json`, dan pengaturan yang dikelola jarak jauh gateway melampaui keduanya, jadi pada mesin yang masuk ke gateway, juga atur `parentSettingsBehavior` dalam [blok `cli`](/docs/id/claude-apps-gateway-config#managed) kebijakan gateway.
  </Step>

  <Step title="Periksa sumber mana yang dipilih">
    Pada mesin yang hanya menjalankan Claude Desktop, panggil [`resolveSettings()`](/docs/id/agent-sdk/typescript#resolvesettings) SDK Agent dan baca `policyOrigin` pada entri `managed` dalam daftar `sources`-nya. Nilai tersebut menamai sumber sisi klien yang dipilih, `plist`, `hklm`, atau `file`, yang merupakan sumber yang harus membawa cuplikan. Sesi tertanam Claude Desktop tidak mengambil kebijakan gateway, jadi blok `cli` gateway tidak pernah dihitung sebagai sumber yang dipilih untuk mereka.
  </Step>
</Steps>

<h3 id="restrict-parent-settings">
  Batasi pengaturan induk
</h3>

Setelah Anda menerapkan `parentSettingsBehavior: "merge"`, proses host apa pun yang meluncurkan Claude Code dapat menyuplai pengaturan induk, bukan hanya Claude Desktop tetapi juga aplikasi Agent SDK atau ekstensi IDE.

Claude Code memfilter pengaturan induk terhadap daftar izin kunci yang membatasi, tetapi beberapa kunci yang diizinkan dapat memberikan akses daripada membatasinya. Kecuali Anda menetapkan kunci `allowManaged*Only`, aturan izin izin dan daftar izin sandbox yang disuplai host masih berlaku. Aturan penolakan dan tanya kebijakan Anda tetap berlaku bagaimanapun; [mereka dievaluasi sebelum aturan izin apa pun](/docs/id/permissions#manage-permissions).

Claude Code meneruskan entri [`sandbox.credentials`](/docs/id/settings-reference#sandbox-credentials) yang disuplai induk dalam bentuk yang dilucuti:

* **Entri `deny`**: diteruskan hanya dengan `path` atau `name` dan mode mereka.
* **Entri File dengan [`mode: mask`](/docs/id/sandboxing#mask-credential-files)**: diteruskan hanya sentinel, sebagai topeng seluruh file yang `injectHosts`-nya adalah daftar kosong, jadi proxy tidak pernah mengganti nilai nyata untuk entri yang disuplai induk di platform apa pun. Semua bidang masking terstruktur dijatuhkan juga, jadi pola ekstrak yang disuplai induk tidak dapat menggantikan topeng yang lebih ketat yang sumber lain tetapkan untuk jalur yang sama.
* **Entri `envVars` dengan `mode: mask`**: tidak diteruskan. `deny` adalah satu-satunya pembatasan yang dapat diekspresikan saluran induk melalui entri `envVars`.
* **[`awsPairs` dan `sigv4`](/docs/id/sandboxing#re-sign-aws-requests)**: diteruskan pembatasan saja. Dari `sigv4`, hanya nilai `deny` yang disimpan, dan induk yang mendefinisikan blok `sigv4` sama sekali menyematkan ketiga bentuk permintaan, `streaming`, `presigned`, dan `sigv4a`, ke `deny`. Pasangan `awsPairs` tidak pernah diteruskan dalam bentuk yang dapat menandatangani ulang; pasangan yang menamai salah satu variabel AWS konvensional diganti dengan entri inert yang menjaga pemasangan otomatis `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY`, dan `AWS_SESSION_TOKEN` ditekan.

<h4 id="deploy-the-locks">
  Terapkan kunci
</h4>

Untuk menjaga pengaturan induk sedekat mungkin dengan pembatasan saja seperti yang didukung filter, tambahkan semua lima kunci `allowManaged*Only`, dan daftar izin yang mereka perintah, ke sumber yang sama dengan opt-in penggabungan:

```json theme={null}
{
  "forceLoginMethod": "gateway",
  "forceLoginGatewayUrl": "https://claude-gateway.internal.example.com",
  "parentSettingsBehavior": "merge",
  "allowManagedPermissionRulesOnly": true,
  "allowManagedMcpServersOnly": true,
  "allowManagedHooksOnly": true,
  "allowedMcpServers": [{ "serverUrl": "https://mcp.internal.example.com/*" }],
  "sandbox": {
    "network": {
      "allowManagedDomainsOnly": true,
      "allowedDomains": ["github.com", "*.npmjs.org"]
    },
    "filesystem": {
      "allowManagedReadPathsOnly": true,
      "denyRead": ["~/"],
      "allowRead": ["~/projects"]
    }
  }
}
```

Kebijakan OS, seperti kebijakan registri HKLM atau plist preferensi yang dikelola, melampaui file ini, jadi kirimkan seluruh cuplikan melaluinya daripada file. Pengaturan yang dikelola jarak jauh gateway melampaui sumber kebijakan OS dan file tetapi hanya menjangkau klien yang terhubung. Cerminkan kunci, daftar izin, dan opt-in penggabungan ke dalam [blok `cli`](/docs/id/claude-apps-gateway-config#managed) kebijakan dan jaga file ini tetap diterapkan, karena mesin yang tidak pernah terhubung, termasuk yang hanya menjalankan Claude Desktop, mendapatkan kebijakan mereka dari file saja.

<h4 id="lock-behavior-across-sources">
  Perilaku kunci di seluruh sumber
</h4>

Menetapkan satu kunci tidak membatasi yang lain; setiap kunci didokumentasikan dalam [referensi pengaturan](/docs/id/settings-reference#all-settings).

Dari sumber admin di bawah pemenang, dua kunci sandbox masih berlaku, dan `allowManagedPermissionRulesOnly` masih memblokir aturan izin yang disuplai induk dan `additionalDirectories`. Pada Claude Code v2.1.273 atau lebih baru, kunci server MCP juga berlaku dari sumber di bawah pemenang, dan sementara itu aktif, daftar `allowedMcpServers` yang dikelola berasal dari sumber admin prioritas tertinggi yang menetapkan satu.

Kunci hooks dan efek `allowManagedPermissionRulesOnly` pada aturan pengembang sendiri memerlukan sumber pemenang secara default; di bawah opt-in penggabungan `managedSourcesBehavior` dalam [bagaimana Claude Code menggabungkan sumber yang dikelola](/docs/id/managed-settings#how-claude-code-combines-managed-sources), Claude Code menerapkan nilai paling ketat yang ditetapkan sumber apa pun untuk setiap kunci. Pada armada [`policyHelper`](/docs/id/settings-reference#policyhelper), Claude Code membaca kunci dari output helper saja.

Setiap kunci membuat Claude Code mengabaikan entri pengembang sendiri untuk pengaturan itu, jadi sertakan daftar izin organisasi Anda di sebelah kunci:

* **Domain jaringan**: mengunci dengan daftar domain yang dikelola kosong memblokir semua lalu lintas keluar sandbox.
* **Server MCP**: mengunci tanpa `allowedMcpServers` yang dikelola dalam sumber admin apa pun atau dalam pengaturan yang disuplai induk memuat setiap server yang tidak diblokir `deniedMcpServers`.
* **Jalur baca**: entri `allowRead` hanya mengizinkan ulang jalur di dalam wilayah `denyRead`, jadi pasangkan dengan `denyRead` yang dikelola.

<h4 id="settings-the-locks-don’t-cover">
  Pengaturan yang tidak dicakup kunci
</h4>

Enam pengaturan yang disuplai induk melewati filter bahkan dengan semua lima kunci yang ditetapkan. Di bawah pengaturan pertama-menang default, nilai admin yang memblokir induk adalah yang ada di sumber admin prioritas tertinggi, kecuali untuk `allowedMcpServers` sementara [kunci server MCP](#lock-behavior-across-sources) aktif. Di bawah opt-in penggabungan `managedSourcesBehavior`, [bagaimana Claude Code menggabungkan sumber yang dikelola](/docs/id/managed-settings#how-claude-code-combines-managed-sources) mengatakan nilai sumber mana yang berlaku sebagai gantinya.

* **`forceLoginOrgUUID`**: Claude Code menghormati nilai yang disuplai induk ketika sumber admin prioritas tertinggi tidak menetapkan UUID organisasi. Masuk gateway tidak memeriksa kunci ini, jadi itu penting hanya untuk armada yang juga menggunakan masuk Anthropic pihak pertama. UUID organisasi dalam sumber admin prioritas tertinggi memblokir nilai induk dan adalah yang ditegakkan Claude Code, jadi atur `forceLoginOrgUUID` di sana.
* **`allowedMcpServers`**: Claude Code menghormati daftar izin yang disuplai induk ketika sumber admin prioritas tertinggi tidak menetapkan satu. `allowManagedMcpServersOnly` tidak memblokirnya, karena kunci menegakkan daftar mana pun yang menang sebagai nilai yang dikelola, termasuk daftar yang disuplai induk ketika sumber admin prioritas tertinggi tidak menetapkan satu. Daftar dalam sumber admin prioritas tertinggi memblokir induk dan adalah daftar yang ditegakkan Claude Code, jadi atur `allowedMcpServers` di sana, di sebelah kunci. Sebelum v2.1.223, nilai untuk salah satu kunci dalam sumber admin apa pun memblokir induk.
* **`availableModels`**: Claude Code menghormati daftar model yang disuplai induk ketika sumber yang dikelola pemenang tidak menetapkan satu. Jika armada Anda membatasi model, atur `availableModels` dalam sumber pemenang.
* **`strictKnownMarketplaces`**: Claude Code menghormati daftar izin marketplace plugin yang disuplai induk ketika sumber yang dikelola pemenang tidak menetapkan satu. Jika armada Anda membatasi marketplace, atur `strictKnownMarketplaces` dalam sumber pemenang. Memerlukan Claude Code v2.1.282 atau lebih baru.
* **`blockedMarketplaces`**: daftar blokir marketplace yang disuplai induk melewati dan menambah ke daftar blokir apa pun yang ditetapkan sumber yang dikelola, karena daftar blokir hanya dapat membatasi lebih lanjut. Memerlukan Claude Code v2.1.282 atau lebih baru.
* **`strictPluginOnlyCustomization`**: kunci ini melewati filter terlepas dari kunci apa pun, dan itu membuat Claude Code mengabaikan kustomisasi pengembang sendiri, termasuk hooks pelindung. Tidak ada kunci yang memblokirnya.

<h3 id="connect-claude-desktop">
  Hubungkan Claude Desktop
</h3>

[Claude Desktop](/docs/id/desktop) terhubung ke gateway yang sama melalui kunci MDM yang berbeda: atur `bootstrapUrl` dalam [konfigurasi yang dikelola](https://claude.com/docs/third-party/claude-desktop/configuration) Claude Desktop ke `<listen.public_url>/user/bootstrap`, dan opt-in kebijakan pengguna dengan kunci `desktop`. [Overlay Claude Desktop](/docs/id/claude-apps-gateway-config#claude-desktop-overlay) mencakup kedua bagian. Memerlukan Claude Code v2.1.203 atau lebih baru di server gateway.

Claude Desktop menandatangani pengembang melalui penyedia identitas gateway dengan langkah SSO browser yang sama, kemudian mengambil konfigurasinya dari gateway daripada dari Anthropic. Akses model dan kebijakan mengikuti aturan per-grup yang sama dengan CLI. Pengembang yang menggunakan CLI dan Claude Desktop keduanya masuk ke masing-masing secara terpisah; sesi gateway tidak dibagikan di antara mereka.

Setelah terhubung, Claude Desktop mengirimkan permintaan model dari setiap tab yang diaktifkan melalui gateway. Ini menampilkan tab Cowork dan Code secara default. Untuk mengaktifkan tab Chat juga, atur `chatTabEnabled` ke `true` dalam [konfigurasi yang dikelola](https://claude.com/docs/third-party/claude-desktop/configuration) Claude Desktop, atau dalam [blok `desktop`](/docs/id/claude-apps-gateway-config#claude-desktop-overlay) kebijakan pada gateway yang menjalankan Claude Code v2.1.227 atau lebih baru.

<h3 id="ci-pipelines-and-remote-machines">
  Pipa CI dan mesin jarak jauh
</h3>

Tidak ada alur token layanan untuk pipa yang tidak diawasi. Masuk gateway selalu menjalankan alur perangkat browser, jadi pekerjaan CI tanpa pengembang untuk menyetujui masuk tidak dapat mengautentikasi; konfigurasikan yang terhadap penyedia Anda secara langsung.

Setelah pengembang masuk, setiap sesi Claude Code di mesin itu menggunakan sesi gateway, termasuk `claude -p` berjalan non-interaktif dan sesi yang dimulai oleh Agent SDK. Claude Code menerapkan [kebijakan gateway](/docs/id/claude-apps-gateway-config#managed) ke masing-masing.

Alur perangkat memisahkan CLI polling dari browser yang menyetujui, jadi kotak pengembangan jarak jauh tanpa tampilan masih berfungsi: pengembang menjalankan `/login` melalui SSH di mesin jarak jauh dan membuka tautan verifikasi di browser di laptop mereka.

<h3 id="whats-enforced-on-developers">
  Apa yang ditegakkan pada pengembang
</h3>

Jaminan ini berlaku untuk setiap sesi yang masuk melalui `/login`. Sesi tertanam yang diluncurkan Claude Desktop mendapatkan kebijakan mereka seperti yang dijelaskan dalam [Kirimkan kebijakan ke sesi Claude Desktop](#deliver-policy-to-claude-desktop-sessions), dan poin telemetri mengatakan ke mana ekspor mereka pergi.

* **Akses model**: permintaan untuk model yang tidak diizinkan kebijakan mengembalikan 400, dan pemilih `/model` disaring ke daftar izin `availableModels` kebijakan. Atur [`enforceAvailableModels: true`](/docs/id/model-config#default-model-behavior) dalam kebijakan sehingga opsi Default diselesaikan ke model dalam `availableModels` daripada ke default bawaan Claude Code; tanpanya, Default tetap dapat dipilih dan ditolak pada waktu permintaan jika model itu tidak diizinkan.
* **Tujuan telemetri**: dalam sesi yang masuk melalui `/login`, CLI mengirimkan ekspor OTLP/HTTP-nya ke gateway daripada ke `OTEL_EXPORTER_OTLP_ENDPOINT` yang ditetapkan secara lokal, kecuali kebijakan [menamai kolektor Anda sebagai titik akhir](/docs/id/claude-apps-gateway-config#export-directly-to-your-collector). Gateway meneruskan ekspor yang diterimanya ke tujuan dalam [`telemetry.forward_to`](/docs/id/claude-apps-gateway-config#telemetry).
  * Dalam sesi tertanam yang [diluncurkan Claude Desktop](#connect-claude-desktop), CLI mengirimkan ekspor ke `OTEL_EXPORTER_OTLP_ENDPOINT` yang dikonfigurasi. CLI melampirkan token sesi gateway ke ekspor tersebut hanya ketika titik akhir itu menunjuk ke gateway itu sendiri.
  * Tanpa tujuan yang dikonfigurasi untuk sinyal, gateway menerima dan membuangnya.
  * Jika Anda sudah mengumpulkan telemetri Claude Code secara langsung, tambahkan kolektor Anda sebagai tujuan `forward_to`, atau namakannya dalam kebijakan untuk melewati relay.
* **Kredensial**: token gateway adalah satu-satunya kredensial sesi. [Profil Anthropic](/docs/id/authentication#anthropic-profiles-and-federation-credentials) dan masuk claude.ai sebelumnya diabaikan saat masuk, jadi pengembang tidak perlu keluar dari claude.ai terlebih dahulu. Untuk kredensial `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN`, atau `apiKeyHelper` yang dikonfigurasi, lihat [Kebijakan administrator memerlukan masuk Cloud gateway](/docs/id/errors#administrator-policy-requires-a-cloud-gateway-sign-in).
* **Pengaturan yang dikelola**: kunci yang dikunci tidak dapat ditimpa secara lokal. CLI menerapkan kebijakan saat startup dan menerapkan perubahan pada setiap polling per jam, terlepas dari [perubahan yang hanya berlaku pada peluncuran berikutnya](/docs/id/server-managed-settings#fetch-and-caching-behavior).
* **Startup dengan gateway tidak dapat dijangkau**: sesi yang masuk keluar saat startup dengan kesalahan setelah sekitar 10 detik daripada memulai tanpa pengaturan mereka.
* **Startup setelah gateway mengakhiri sesi**: lihat [Tegakkan startup yang gagal-tertutup](/docs/id/server-managed-settings#enforce-fail-closed-startup) untuk peluncuran yang membuka keluar dari gateway dan yang keluar ketika gateway menjawab dengan `401`.
* **Pemberhentian**: sesi yang pengguna-nya dinonaktifkan dalam IdP kedaluwarsa dalam `ttl_hours` ketika penyegaran berikutnya gagal.
* **Keluar**: `/logout` menghapus kredensial gateway dari mesin pengembang.
  * Ketika dokumen penemuan gateway mengiklankan `revocation_endpoint` pada skema, host, dan port URL gateway sendiri, `/logout` juga mengirimkan token yang disimpan ke titik akhir itu sehingga gateway dapat mengakhiri sesi di sisinya. Permintaan adalah upaya terbaik, jadi keluar selesai di mesin pengembang terlepas dari apakah titik akhir menjawab. Pencabutan memerlukan Claude Code v2.1.275 atau lebih baru di mesin pengembang.
  * Server gateway dalam biner `claude` tidak mengiklankan apa pun, jadi keluar darinya mengakhiri sesi di mesin pengembang saja. Untuk memaksa sesi keluar server-side, lihat [Rotasi rahasia JWT](/docs/id/claude-apps-gateway-deploy#jwt-secret-rotation).

<h3 id="what-the-organization-can-see">
  Apa yang dapat dilihat organisasi
</h3>

Telemetri penggunaan membawa identitas pengembang, hitungan token, model, dan latensi ke kolektor organisasi. Gateway tidak mencatat atau menyimpan konten prompt atau penyelesaian. Apakah telemetri yang lebih kaya seperti log dan jejak dikumpulkan, yang dapat mencakup perintah dan jalur file, adalah [pilihan per-tujuan](/docs/id/claude-apps-gateway-config#telemetry) organisasi.

<h2 id="availability-and-limitations">
  Ketersediaan dan keterbatasan
</h2>

Tabel mencakup fitur Claude Code mana yang bekerja ketika pengembang terhubung melalui gateway, dan apa yang didukung server gateway itu sendiri. Di mana sesuatu tidak didukung, kolom Catatan memberikan alternatif.

Gateway mengirimkan nilai [`anthropic-beta`](https://platform.claude.com/docs/en/api/beta-headers) yang dikirim CLI ke setiap upstream, jadi operator tidak mempertahankan daftar izin beta. Untuk Amazon Bedrock, yang mengabaikan header, gateway memindahkan nilai ke bidang `anthropic_beta` badan permintaan; upstream lainnya menerima header seperti yang dikirim.

| Fitur                                                                                                                   | Status                 | Catatan                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| ----------------------------------------------------------------------------------------------------------------------- | ---------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Penerusan inferensi (Amazon Bedrock, Claude Platform on AWS, Agent Platform Google Cloud, Microsoft Foundry, Anthropic) | Tersedia               | Dengan terjemahan model per-upstream dan failover. Upstream Amazon Bedrock menggunakan endpoint `bedrock-runtime` dan rantai kredensial default AWS; [endpoint Mantle](/docs/id/amazon-bedrock#use-the-mantle-endpoint) Amazon Bedrock bukan upstream yang didukung. [Upstream Claude Platform on AWS](/docs/id/claude-apps-gateway-config#claude-platform-on-aws) memerlukan Claude Code v2.1.198 atau lebih baru di server gateway.                                                   |
| Akses model dan pengaturan terkelola berdasarkan grup IdP                                                               | Tersedia               | Akses model diberlakukan di sisi server; pengaturan terkelola disampaikan per grup IdP dan diterapkan oleh CLI di [tingkat pengaturan terkelola](/docs/id/settings#settings-precedence)                                                                                                                                                                                                                                                                                            |
| Claude Desktop                                                                                                          | Tersedia dengan opt-in | Gateway melayani konfigurasi Claude Desktop di `/user/bootstrap` setelah kebijakan [memilih dengan kunci `desktop`](/docs/id/claude-apps-gateway-config#claude-desktop-overlay), dan Claude Desktop mengirim permintaan model dari tab Cowork dan Code, dan dari tab Chat ketika Anda mengaktifkannya, melalui gateway. Untuk mengaktifkan tab Chat, lihat [Hubungkan Claude Desktop](#connect-claude-desktop). Memerlukan Claude Code v2.1.203 atau lebih baru di server gateway. |
| Fan-out telemetri (OTLP/HTTP)                                                                                           | Tersedia               | Identitas-stamped per ekspor; kedua pengkodean protobuf dan JSON                                                                                                                                                                                                                                                                                                                                                                                                              |
| Penyedia identitas OIDC                                                                                                 | Tersedia               | Penyedia IdP yang sesuai dengan OIDC; gateway menjalankan penemuan OIDC standar dan alur kode otorisasi. Lihat [Pengaturan penyedia identitas](/docs/id/claude-apps-gateway-deploy#identity-provider-setup) untuk konfigurasi per-IdP                                                                                                                                                                                                                                              |
| Batas pengeluaran per-pengguna dan per-grup                                                                             | Tersedia               | Lihat [Spend limits](/docs/id/claude-apps-gateway-spend-limits)                                                                                                                                                                                                                                                                                                                                                                                                                    |
| Pencarian web sisi server                                                                                               | Tidak tersedia         | CLI tidak dapat melihat penyedia upstream mana yang dirutekan gateway, jadi tidak dapat memverifikasi dukungan pencarian web dan menonaktifkan WebSearch pada sesi gateway                                                                                                                                                                                                                                                                                                    |
| [Remote Control](/docs/id/remote-control)                                                                                    | Tidak tersedia         | CLI menampilkan [kesalahan yang menyebutkan gateway](/docs/id/errors#remote-control-requires-the-anthropic-api)                                                                                                                                                                                                                                                                                                                                                                    |
| [`/design-sync`](/docs/id/commands#all-commands) dan `/design-login`                                                         | Tidak tersedia         | Keduanya memerlukan claude.ai, yang tidak dihubungi CLI pada sesi gateway, jadi tidak ada perintah yang muncul di sana                                                                                                                                                                                                                                                                                                                                                        |
| Fitur yang memerlukan pengambilan flag fitur, seperti `/import` dan `claude import`                                     | Tidak tersedia         | CLI melewati pengambilan flag pada sesi gateway. [Fitur yang memerlukan pengambilan flag fitur](/docs/id/env-vars#features-that-need-feature-flag-fetching) mencantumkan apa yang dimatikan                                                                                                                                                                                                                                                                                        |
| Prompt caching standar                                                                                                  | Tersedia               | Gateway meneruskan breakpoint `cache_control` ke setiap upstream. [Di mana cache berada](/docs/id/prompt-caching#where-the-cache-lives) mencakup blok mana yang ditandai CLI, termasuk konteks sistem yang ditambahkannya di tengah percakapan                                                                                                                                                                                                                                     |
| TTL cache 1 jam                                                                                                         | Tidak tersedia         | CLI menghilangkan beta extended-cache-ttl pada sesi gateway, karena tidak setiap upstream yang dapat dirutekan gateway mendukung TTL 1 jam, jadi prompt caching melalui gateway menggunakan TTL 5 menit; lihat catatan header beta di atas                                                                                                                                                                                                                                    |
| Mode Auto                                                                                                               | Tersedia               | Mengikuti [aturan penyedia pihak ketiga](/docs/id/permission-modes#enable-auto-mode-on-bedrock-agent-platform-or-foundry): hanya model yang memenuhi syarat di penyedia pihak ketiga yang dapat menggunakannya. Sebelum v2.1.207, mode auto pada sesi gateway memerlukan pengaturan `CLAUDE_CODE_ENABLE_AUTO_MODE=1`, dapat dikirimkan melalui blok `env` kebijakan terkelola                                                                                                      |
| Optimasi khusus pihak pertama seperti cakupan cache global dan alat yang efisien token                                  | Tidak tersedia         | CLI tidak mengaktifkannya pada sesi gateway; lihat catatan header beta di atas                                                                                                                                                                                                                                                                                                                                                                                                |
| OTLP/gRPC                                                                                                               | Tidak didukung         | OTLP melalui HTTP saja                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| SAML, LDAP, dan auth non-OIDC lainnya                                                                                   | Tidak didukung         | OIDC saja. Depan dengan jembatan OIDC jika diperlukan                                                                                                                                                                                                                                                                                                                                                                                                                         |
| Multi-tenant (beberapa penerbit OIDC)                                                                                   | Tidak didukung         | Satu penerbit per gateway. Jalankan instans terpisah                                                                                                                                                                                                                                                                                                                                                                                                                          |
| Server Windows                                                                                                          | Tidak didukung         | Terapkan di Linux. macOS untuk pengembangan lokal saja                                                                                                                                                                                                                                                                                                                                                                                                                        |
| Helm chart                                                                                                              | Tidak tersedia         | Gateway berjalan sebagai Deployment stateless standar; lihat [panduan penyebaran](/docs/id/claude-apps-gateway-deploy#kubernetes)                                                                                                                                                                                                                                                                                                                                                  |
| Admin UI                                                                                                                | Tidak tersedia         | Konfigurasi adalah file YAML; terapkan ulang untuk mengubahnya                                                                                                                                                                                                                                                                                                                                                                                                                |

<h2 id="next-steps">
  Langkah berikutnya
</h2>

Quickstart meninggalkan Anda dengan konfigurasi minimal yang berjalan di bawah Docker Compose. Untuk membawanya lebih jauh:

* Perluas `gateway.yaml` di luar konfigurasi minimal, misalnya untuk menambahkan RBAC per-grup, failover multi-upstream, atau tujuan telemetri. [Referensi konfigurasi](/docs/id/claude-apps-gateway-config) mencakup setiap opsi.
* Pindah dari Compose ke penyebaran produksi di Kubernetes atau Cloud Run, siapkan IdP Anda dengan benar, dan tinjau model keamanan. [Panduan penyebaran dan operasi](/docs/id/claude-apps-gateway-deploy) mencakup penyiapan per-IdP, persyaratan gambar kontainer, probe kesehatan, dan pemecahan masalah.
* Letakkan batas pengeluaran pada pengembang atau grup individual sehingga beban kerja yang liar tidak dapat mengonsumsi seluruh komitmen Anda. [Spend limits](/docs/id/claude-apps-gateway-spend-limits) mencakup API admin dan cara penegakan bekerja.
* Untuk contoh lengkap yang dikerjakan di AWS, dengan ECS Fargate atau EKS, Amazon RDS, dan Secrets Manager, lihat [Deploy on AWS](/docs/id/claude-apps-gateway-on-aws).
* Untuk contoh lengkap yang dikerjakan di Google Cloud, dengan Cloud Run, Cloud SQL, dan Secret Manager, lihat [Deploy on Google Cloud](/docs/id/claude-apps-gateway-on-gcp).
