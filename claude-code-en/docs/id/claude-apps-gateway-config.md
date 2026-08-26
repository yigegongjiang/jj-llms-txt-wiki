> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Konfigurasi gateway aplikasi Claude

> Referensi untuk setiap opsi gateway.yaml: listener dan TLS, OIDC, session, Postgres store, Amazon Bedrock, Claude Platform on AWS, Google Cloud's Agent Platform, dan Microsoft Foundry upstreams, model routing, managed policies, dan telemetry.

Deployment gateway aplikasi Claude dikonfigurasi oleh satu file YAML, secara konvensional `gateway.yaml`. File ini mendefinisikan semua yang dilakukan gateway: di mana ia mendengarkan, bagaimana pengembang masuk, ke mana inference pergi, dan kebijakan serta telemetry mana yang berlaku. Halaman ini adalah referensi untuk setiap opsi dalam file tersebut.

Untuk menulis yang pertama, mulai dari [quickstart](/docs/id/claude-apps-gateway#quickstart), yang membangun config minimal yang berfungsi dan menjalankannya. Setelah Anda memiliki config yang Anda sukai, [deployment guide](/docs/id/claude-apps-gateway-deploy) mencakup containerizing dan hosting di Kubernetes, Cloud Run, atau platform Anda sendiri.

Gateway membaca file sekali, saat startup, dengan `claude gateway --config /path/to/gateway.yaml`. Setiap opsi divalidasi terhadap schema saat boot, jadi config yang salah format gagal saat start dengan error tingkat field daripada saat penggunaan pertama.

[Complete example](#complete-example) di akhir halaman ini menggunakan setiap bagian.

<h2 id="file-structure">
  Struktur file
</h2>

Lima bagian [diperlukan](#required-sections). Setiap bagian lainnya [opsional](#optional-sections), dan bagian yang dihilangkan mengambil default-nya. Kunci yang tidak dikenal gagal boot, jadi typo muncul sebagai error bernama daripada setting yang diabaikan secara diam-diam.

**Bagian yang diperlukan:**

* [`listen`](#listen): bind address, public URL, TLS termination
* [`oidc`](#oidc): identity provider Anda (IdP), termasuk issuer, client, claim mapping, dan siapa yang boleh masuk
* [`session`](#session): bearer tokens yang dimint gateway, dengan secret dan lifetime
* [`store`](#store): PostgreSQL, untuk device grants dan rate-limit counters
* [`upstreams`](#upstreams): ke mana inference pergi, apakah Anthropic, Amazon Bedrock, Claude Platform di AWS, Agent Platform Google Cloud, atau Microsoft Foundry

**Bagian opsional:**

* [`admin`](#admin): Admin API auth dan retention untuk spend limits
* [`enforcement`](#enforcement): perilaku spend-limit fail-open atau fail-closed
* [`models`](#models) dan `auto_include_builtin_models`: daftar model yang dikurasi admin dan per-upstream IDs
* [`managed`](#managed): managed settings policies berdasarkan IdP group
* [`telemetry`](#telemetry): OTLP forwarding ke observability stack Anda
* [`access_control`, `limits`, `timeouts`, `rate_limits`](#http-tuning): IP allow/deny, request size caps, upstream time-to-first-byte, dan per-IP sign-in limits

<h2 id="secret-expansion">
  Ekspansi secret
</h2>

Jangan tulis secrets seperti `client_secret`, `jwt_secret`, atau `postgres_url` langsung di `gateway.yaml`. Referensikan mereka dengan salah satu bentuk di bawah, dan gateway menyelesaikan nilai saat boot dari environment variable atau file:

| Bentuk          | Diselesaikan ke                                                | Gunakan untuk                                                          |
| --------------- | -------------------------------------------------------------- | ---------------------------------------------------------------------- |
| `${VAR}`        | Environment variable `VAR`. Boot gagal jika tidak terdefinisi. | Container environment variables, AWS Secrets Manager via env injection |
| `${file:/path}` | Isi file, dipangkas                                            | Kubernetes Secret volume mounts, Vault Agent, SOPS                     |

<h2 id="required-sections">
  Bagian yang diperlukan
</h2>

<h3 id="listen">
  `listen`
</h3>

Blok `listen` mengontrol di mana gateway melayani: bind address dan port, origin yang terlihat secara eksternal, dan optional TLS termination.

| Field                  | Diperlukan        | Deskripsi                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| ---------------------- | ----------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `host`                 | Tidak             | Bind address. Default `0.0.0.0`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `port`                 | Tidak             | Bind port. Default `8080`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `public_url`           | Di belakang proxy | Origin `https://` yang terlihat secara eksternal, digunakan untuk membangun IdP `redirect_uri` dan discovery metadata. Diperlukan di belakang proxy apa pun yang menghentikan TLS seperti ALB, Ingress, atau Cloud Run, karena gateway tidak mempercayai header `X-Forwarded-*` saat membangun origin-nya sendiri; mereka dapat dipalsukan oleh client. `trusted_proxies` di bawah mengatur resolusi client-IP saja. Juga diperlukan untuk mengaktifkan [telemetry](#telemetry), karena gateway membangun endpoint OTLP yang didorong ke client dari URL ini. |
| `tls.cert` / `tls.key` | Tidak             | Path PEM jika gateway menghentikan TLS sendiri                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `trusted_proxies`      | Tidak             | CIDR atau IP dari load balancer di depan gateway. Ketika diatur, gateway mempercayai `X-Forwarded-For` hanya dari peer ini dan mencatat IP client yang sebenarnya untuk per-IP rate limiting dan audit. Setara dengan nginx `set_real_ip_from`.                                                                                                                                                                                                                                                                                                               |

<h3 id="oidc">
  `oidc`
</h3>

Blok `oidc` menghubungkan gateway ke identity provider Anda dan memutuskan siapa yang dapat masuk. Ini menamai issuer dan OAuth client, memetakan claims yang membawa email dan groups, dan membatasi sign-in berdasarkan email domain atau group.

OpenID Connect (OIDC) adalah protokol SSO yang digunakan gateway dengan identity provider Anda; lihat [Identity provider setup](/docs/id/claude-apps-gateway-deploy#identity-provider-setup) untuk apa yang harus didaftarkan di sisi IdP.

| Field                           | Diperlukan | Deskripsi                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| ------------------------------- | ---------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `issuer`                        | Ya         | OIDC discovery base. Harus melayani discovery di `/.well-known/openid-configuration`. Gunakan HTTPS dalam production; gateway menerima issuer `http://`. Issuer loopback seperti `http://localhost:8081` ditolak oleh [SSRF guard](/docs/id/claude-apps-gateway-deploy#threat-model-summary) kecuali `CLAUDE_GATEWAY_ALLOW_LOOPBACK=1` diatur di environment gateway.                                                                                                                                                                                                                                  |
| `client_id` / `client_secret`   | Ya         | Dari registrasi OAuth client Anda                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `allowed_email_domains`         | Tidak      | Tolak id\_tokens yang claim `email`-nya tidak ada di salah satu domain ini, case-insensitive. Defense-in-depth terhadap misconfiguration IdP multi-tenant. Independen dari setting ini, id\_token yang claim `email_verified`-nya secara eksplisit `false` selalu ditolak.                                                                                                                                                                                                                                                                                                                        |
| `allowed_groups`                | Tidak      | Batasi sign-in ke anggota IdP groups ini, dicocokkan terhadap `groups_claim`. User dalam email domain yang diizinkan tetapi tidak ada di salah satu groups ini ditolak. Memerlukan IdP untuk memancarkan groups claim.                                                                                                                                                                                                                                                                                                                                                                            |
| `groups_claim`                  | Tidak      | Claim id\_token mana yang membawa group membership. Default `groups`. Microsoft Entra memancarkan app roles di bawah `roles`. Menerima flat key atau RFC 6901 JSON Pointer seperti `/resource_access/gateway/roles` untuk nested claims.                                                                                                                                                                                                                                                                                                                                                          |
| `google_groups`                 | Tidak      | Cari groups user yang masuk melalui Google Workspace Admin SDK Directory API, karena id\_token Google tidak membawa groups claim. Atur `service_account_json_path` ke file service-account key dengan domain-wide delegation pada scope `https://www.googleapis.com/auth/admin.directory.group.readonly`, dan `admin_email` ke administrator Workspace yang disamar oleh service account; Directory API memerlukan subject admin yang sebenarnya. Email address group setiap user menjadi groups claim mereka, jadi `allowed_groups` dan `managed.policies.match.groups` cocok pada group emails. |
| `email_claim`                   | Tidak      | Claim id\_token mana yang membawa email user. Default `email`. Beberapa IdP, seperti ADFS dan Entra B2C, memancarkan `upn` atau `preferred_username` sebagai gantinya. Menerima flat key, JSON Pointer, atau daftar fallback keys di mana key pertama yang ada digunakan.                                                                                                                                                                                                                                                                                                                         |
| `scopes`                        | Tidak      | Override lengkap dari scopes OIDC yang diminta gateway. Default `[openid, profile, email, offline_access]`. Atur ketika IdP Anda menolak scopes yang tidak dikenalinya, atau memerlukan custom scope untuk memancarkan groups atau email. Harus menyertakan `openid`. Menghilangkan `offline_access` menonaktifkan refresh tokens, jadi developer menjalankan kembali browser login setiap `session.ttl_hours`. Lihat [Identity provider setup](/docs/id/claude-apps-gateway-deploy#identity-provider-setup) untuk per-IdP scope recipes seperti Google's refresh-token flow.                          |
| `extra_auth_params`             | Tidak      | Extra query parameters ditambahkan ke IdP authorization request, verbatim. Ini adalah mekanisme override untuk perilaku spesifik IdP, seperti `access_type: offline` untuk Google refresh tokens, `domain_hint` untuk beberapa Entra tenants, atau `acr_values` untuk step-up flows. Tidak dapat override protocol params yang dikelola gateway: `state`, `nonce`, `redirect_uri`, PKCE, `scope`, `response_type`, `response_mode`, dan `client_id`.                                                                                                                                              |
| `userinfo_fallback`             | Tidak      | Ketika id\_token menghilangkan email atau groups, ambil dari `/userinfo`. Diperlukan untuk Keycloak lightweight access tokens, Okta org server, dan ADFS minimal tokens. Id\_token tetap authoritative; userinfo hanya mengisi gaps. Default `false`.                                                                                                                                                                                                                                                                                                                                             |
| `use_pkce`                      | Tidak      | Kirim PKCE (S256) challenge pada authorization request. Default `true`. Atur `false` hanya jika IdP Anda menolak PKCE untuk confidential client ini.                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `clock_skew_seconds`            | Tidak      | Toleransi clock drift saat memvalidasi id\_token time claims. Default `0`, yang ketat. Naikkan jika Anda melihat error "token expired / not yet valid" tepat setelah sign-in karena host/IdP clock skew.                                                                                                                                                                                                                                                                                                                                                                                          |
| `token_endpoint_auth_method`    | Tidak      | Override token-endpoint auth method. Menerima `client_secret_basic` atau `client_secret_post`. Auto-negotiated secara default.                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `id_token_signed_response_alg`  | Tidak      | Expected id\_token signing algorithm. Default `RS256`. Atur untuk IdP yang menandatangani dengan ES256, PS256, atau EdDSA.                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `additional_authorized_parties` | Tidak      | Extra `azp` values untuk diterima di luar `client_id`, untuk Keycloak broker dan token-exchange flows                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `discovery_url`                 | Tidak      | Ambil discovery document dari URL ini daripada menurunkannya dari `issuer`, untuk IdP di belakang proxy yang menulis ulang issuer host. Path harus berisi `/.well-known/`.                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `form_action_origins`           | Tidak      | Origins tambahan untuk direktif `Content-Security-Policy: form-action` halaman `/device`. Gateway sudah mengizinkan `'self'` dan origin `authorization_endpoint` yang ditemukan, tetapi Chrome memberlakukan `form-action` terhadap seluruh redirect chain. Jika IdP Anda mengalihkan melalui host kedua, seperti Azure AD federated ke ADFS, hub-spoke Okta, atau SSO interceptor korporat, daftar setiap origin yang mungkin dialihkan oleh authorization request.                                                                                                                              |
| `ca_cert_pem`                   | Tidak      | PEM CA cert yang menggantikan system trust store untuk IdP requests saja. Gunakan untuk Keycloak atau Dex di belakang corporate PKI.                                                                                                                                                                                                                                                                                                                                                                                                                                                              |

<h3 id="session">
  `session`
</h3>

Blok `session` membentuk bearer tokens yang dimint gateway setelah sign-in: secret yang menandatanganinya dan berapa lama mereka hidup.

| Field        | Diperlukan | Deskripsi                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| ------------ | ---------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `jwt_secret` | Ya         | Minimal 32 bytes entropy, misalnya dari `openssl rand -base64 32`. Menandatangani gateway's HS256 bearer tokens. Menerima single string atau array untuk rotation: index 0 menandatangani dan semua entries memverifikasi. Untuk rotate, prepend secret baru, tunggu `ttl_hours`, kemudian drop yang lama.                                                                                                                                                                                   |
| `ttl_hours`  | Tidak      | Gateway bearer token lifetime. Default `1`. CLI secara diam-diam refresh sebelum expiry ketika IdP mengeluarkan refresh tokens. Lifetime yang lebih pendek menghapus provisioning lebih cepat; yang lebih panjang membuat lebih sedikit IdP round-trips. Jika IdP Anda tidak dapat mengeluarkan refresh tokens karena `offline_access` tidak tersedia, tidak ada silent refresh, jadi naikkan ini ke `8` atau `12` untuk menghindari mengirim developer kembali ke browser login setiap jam. |

<h3 id="store">
  `store`
</h3>

Blok `store` menunjukkan gateway ke database PostgreSQL-nya, yang menyimpan device grants dan rate-limit counters.

| Field             | Diperlukan | Deskripsi                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| ----------------- | ---------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `postgres_url`    | Ya         | `postgres://` atau `postgresql://` URL. Diperlukan: device-grant rendezvous, di mana browser callback menulis dan polling CLI membaca, memerlukan cross-replica state. Gateway menjalankan schema migrations-nya sendiri saat boot, jadi role memerlukan `CREATE TABLE` pada target schema. Jika kebijakan keamanan Anda melarang DDL dari application role, jalankan migrations dengan admin role, awalnya dan lagi setiap kali release baru mengirim migrations, dan berikan app role `SELECT, INSERT, UPDATE, DELETE` pada gateway's tables. Lihat [Upgrades](/docs/id/claude-apps-gateway-deploy#upgrades) dan [Postgres](/docs/id/claude-apps-gateway-deploy#postgres). |
| `username`        | Tidak      | Overrides user di `postgres_url`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `password`        | Tidak      | Database credential. Atur di sini daripada di `postgres_url` jadi credential tetap keluar dari URL. Menerima karakter apa pun dan mengambil precedence atas URL credentials.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `max_connections` | Tidak      | Postgres connection-pool size per replica. Default `5`, yang konservatif dan ramah ke shared databases. Dengan [spend limits](#admin) diaktifkan, hot path melakukan beberapa operasi per inference request, jadi naikkan untuk dedicated database di bawah load, dan simpan replicas × ini di bawah database's `max_connections`.                                                                                                                                                                                                                                                                                                                                 |

Untuk local development, arahkan `postgres_url` ke throwaway Postgres container, misalnya `docker run --rm -p 5432:5432 -e POSTGRES_HOST_AUTH_METHOD=trust postgres`.

<h3 id="upstreams">
  `upstreams`
</h3>

`upstreams` adalah ordered list. Gateway meneruskan inference ke upstream pertama yang menyelesaikan model yang diminta. Pada `5xx`, `429`, `401`, `403`, `404`, atau timeout ia failover ke next; `4xx` lainnya tidak, karena error tersebut dapat diatribusikan ke request daripada upstream. `401` atau `403` berarti credential gateway sendiri gagal terhadap upstream itu, dan `404` berarti upstream itu tidak melayani model yang diminta, jadi upstream yang lebih baru dalam list masih bisa.

Failover pada `404` memerlukan gateway v2.1.198 atau lebih baru. Release sebelumnya mengembalikan `404` pertama ke client bahkan ketika upstream yang lebih baru dalam list melayani model.

Multiple upstreams dari provider yang sama harus menetapkan distinct `name:`.

Amazon Bedrock, Claude Platform on AWS, Google Cloud's Agent Platform, dan Microsoft Foundry clients dibangun sekali saat startup, dan SDK mereka refresh credentials secara internal, jadi rotating cloud credentials tidak memerlukan restart. Static Anthropic API keys dan bearers dibaca saat startup; lihat [Anthropic API](#anthropic-api).

<h4 id="anthropic-api">
  Anthropic API
</h4>

Minimal Anthropic upstream adalah API key dari [Claude Console](https://platform.claude.com):

```yaml theme={null}
upstreams:
  - provider: anthropic
    auth:
      api_key: ${ANTHROPIC_API_KEY}
    # OR an OAuth bearer (e.g. a Workload-Identity-Federation-exchanged token):
    #   oauth_token: ${file:/var/run/secrets/anthropic-oauth-token}
    # base_url: https://api.anthropic.com   # default; override for a forward proxy
```

Dua bentuk credential berbeda dalam header yang mereka kirim:

* **`api_key`**: mengirim `x-api-key`. Rotate di Claude Console dan update env var.
* **`oauth_token`**: mengirim `Authorization: Bearer`. Gunakan bentuk bearer ketika org Anda mengeluarkan short-lived tokens daripada long-lived API keys. Bearer dibaca sekali saat startup, jadi refresh dengan remount secret dan restart.

Daripada static key atau bearer, Anda dapat menggunakan Workload Identity Federation. Buat federation rule dengan mengikuti [Workload Identity Federation guide](https://platform.claude.com/docs/en/manage-claude/workload-identity-federation), kemudian mount workload's OIDC JWT Anda sebagai file, seperti Kubernetes projected service-account token atau CI platform's id-token. Gateway menukar JWT untuk short-lived bearer dan refresh secara otomatis. Token file dibaca ulang pada setiap exchange, jadi rotated projected tokens diambil tanpa restart.

```yaml theme={null}
upstreams:
  - provider: anthropic
    auth:
      federation_rule_id: ${ANTHROPIC_FEDERATION_RULE_ID}
      organization_id: ${ANTHROPIC_ORGANIZATION_ID}
      identity_token_file: /var/run/secrets/anthropic/id-token
      # workspace_id: wrkspc_...       # required if the rule covers >1 workspace
      # service_account_id: svac_...   # optional expected-target check
```

<h4 id="amazon-bedrock">
  Amazon Bedrock
</h4>

Untuk client-side Amazon Bedrock deployment yang digantikan atau di-front oleh gateway, lihat [Claude Code on Amazon Bedrock](/docs/id/amazon-bedrock). Gateway-side upstream:

```yaml theme={null}
upstreams:
  - provider: bedrock
    region: us-east-1
    auth: {}                           # preferred: AWS default credential chain
    # OR explicit credentials:
    # auth:
    #   aws_access_key_id: ${AWS_AKID}
    #   aws_secret_access_key: ${AWS_SK}
    #   aws_session_token: ${AWS_ST}
    # OR a Bedrock API bearer token:
    # auth:
    #   aws_bearer_token: ${AWS_BEARER_TOKEN}
    # Override the bedrock-runtime endpoint for FIPS or VPC-endpoint deployments:
    # base_url: https://bedrock-runtime-fips.us-east-1.amazonaws.com
```

Empty `auth` block menggunakan AWS SDK's default credential chain: env vars, `~/.aws/credentials`, ECS task role, EC2 instance metadata, atau IRSA pada EKS. Dalam production, berikan gateway pod IAM role daripada embedding static keys dalam container image.

Explicit credentials harus lengkap: gateway gagal saat boot ketika `aws_access_key_id` dan `aws_secret_access_key` tidak diatur bersama, atau ketika `aws_session_token` diatur tanpa mereka. Sebelum v2.1.207, partial `auth:` block lulus validasi.

| Setup           | Bagaimana                                                                                                                                                                                                                                                                                                                          |
| --------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| IAM permissions | Berikan gateway's principal `bedrock:InvokeModel` dan `bedrock:InvokeModelWithResponseStream` pada inference-profile ARNs dan underlying foundation-model ARNs. Untuk built-in catalog di US regions: `arn:aws:bedrock:<region>:<account>:inference-profile/us.anthropic.*` dan `arn:aws:bedrock:*::foundation-model/anthropic.*`. |
| Model access    | Di Bedrock console, per region, request dan enable model access untuk Claude models yang Anda inginkan. Cross-region inference profiles (`us.anthropic.*`) memerlukan model access di setiap region yang dicakup profile.                                                                                                          |
| EKS (IRSA)      | Buat IAM role dengan policy di atas dan trust policy untuk cluster's OIDC provider yang scoped ke gateway's service account. Annotate service account dengan `eks.amazonaws.com/role-arn: arn:aws:iam::<acct>:role/claude-gateway`. `auth: {}` mengambilnya.                                                                       |
| ECS / EC2       | Attach IAM role ke task definition atau instance profile. `auth: {}` mengambilnya.                                                                                                                                                                                                                                                 |
| Tempat lain     | Lewatkan credentials via `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY`, dan `AWS_SESSION_TOKEN` env vars, atau atur secara eksplisit di `auth:` dengan `${VAR}` expansion                                                                                                                                                           |
| Region          | `region:` adalah API endpoint region. Cross-region inference profiles route across geo (US, EU, APAC) terlepas dari mana Anda memilih. Untuk non-US regions atau provisioned-throughput ARNs, tambahkan [`models:`](#models) block dengan per-upstream IDs yang benar.                                                             |

<h4 id="claude-platform-on-aws">
  Claude Platform on AWS
</h4>

Claude Platform on AWS melayani first-party Anthropic API pada infrastruktur AWS di `aws-external-anthropic.<region>.api.aws`. Ini menggunakan first-party model IDs, menghormati header `anthropic-beta` seperti yang dikirim, dan melayani `count_tokens`, jadi tidak ada terjemahan spesifik Bedrock yang berlaku. Provider `anthropicAws` memerlukan Claude Code v2.1.198 atau lebih baru; release gateway sebelumnya menolaknya saat boot.

Untuk client-side deployment dari platform yang sama, lihat [Claude Code on Claude Platform on AWS](/docs/id/claude-platform-on-aws). Gateway-side upstream:

```yaml theme={null}
upstreams:
  - provider: anthropicAws
    region: us-east-1
    workspace_id: wrkspc_...
    auth:
      api_key: ${ANTHROPIC_AWS_API_KEY}   # sent as x-api-key
    # OR SigV4 via the AWS default credential chain:
    # auth: {}
    # OR explicit SigV4 credentials:
    # auth:
    #   aws_access_key_id: ${AWS_ACCESS_KEY_ID}
    #   aws_secret_access_key: ${AWS_SECRET_ACCESS_KEY}
    # Override the derived endpoint:
    # base_url: https://aws-external-anthropic.us-east-1.api.aws
```

Platform berjalan di akun AWS terpisah dari Amazon Bedrock dan menandatangani SigV4 requests untuk nama service-nya sendiri, `aws-external-anthropic`, jadi Bedrock-scoped IAM role tidak mengotorisasinya. API key di `auth.api_key` mengambil precedence ketika SigV4 credentials juga diatur. Empty `auth` block menggunakan AWS SDK's default credential chain, chain yang sama yang digunakan upstream [Amazon Bedrock](#amazon-bedrock).

| Field                                                   | Diperlukan | Deskripsi                                                                                                                                          |
| ------------------------------------------------------- | ---------- | -------------------------------------------------------------------------------------------------------------------------------------------------- |
| `region`                                                | Ya         | AWS region, lowercase letters, digits, dan hyphens. Gateway menurunkan endpoint darinya sebagai `https://aws-external-anthropic.<region>.api.aws`. |
| `workspace_id`                                          | Ya         | Dikirim sebagai header pada setiap request; platform memerlukan ini                                                                                |
| `auth.api_key`                                          | Tidak      | API key untuk platform, dikirim sebagai `x-api-key`. Bukan bearer token: dua auth modes adalah API key atau SigV4.                                 |
| `auth.aws_access_key_id` / `auth.aws_secret_access_key` | Tidak      | Explicit SigV4 credentials. Menetapkan satu tanpa yang lain gagal saat boot. `auth.aws_session_token` diterima bersama mereka.                     |
| `base_url`                                              | Tidak      | Override endpoint yang diturunkan                                                                                                                  |

Karena platform menyelesaikan first-party model IDs, built-in catalog routes ke sana tanpa [`models:`](#models) block. Ketika Anda mengkurasi daftar `models:`, key entry `anthropicAws:` dengan first-party ID.

<h4 id="google-cloud-agent-platform">
  Google Cloud Agent Platform
</h4>

Untuk equivalent client-side setup, lihat [Claude Code on Google Cloud](/docs/id/google-vertex-ai). Gateway-side upstream:

```yaml theme={null}
upstreams:
  - provider: vertex
    region: us-east5
    project_id: example-prod
    auth: {}                           # preferred: Application Default Credentials
    # OR a service account key file:
    # auth: { service_account_json: /secrets/sa.json }
    # Override the aiplatform endpoint for Private Service Connect:
    # base_url: https://us-east5-aiplatform.p.googleapis.com
```

Empty `auth` block menggunakan Application Default Credentials: `GOOGLE_APPLICATION_CREDENTIALS`, GCE metadata, atau GKE Workload Identity. Service-account JSON key files didukung tetapi tidak disarankan; gunakan Workload Identity atau attach service account ke GCE atau Cloud Run instance.

Atur `region: global` untuk menggunakan [global endpoint untuk Google Cloud's Agent Platform](https://cloud.google.com/vertex-ai/generative-ai/docs/learn/locations) daripada regional. Google kemudian route setiap request ke available region, jadi Anda tidak track per-region model availability. Menetapkan region spesifik pin setiap request ke sana.

| Setup                   | Bagaimana                                                                                                                                                                                                  |
| ----------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| IAM permissions         | Berikan gateway's service account `roles/aiplatform.user` pada project, atau custom role dengan `aiplatform.endpoints.predict`. Enable Agent Platform API (`aiplatform.googleapis.com`).                   |
| Model access            | Di Model Garden, enable Claude models untuk project Anda. Mereka publish ke specific regions; check model card untuk supported regions.                                                                    |
| GKE (Workload Identity) | Bind GCP service account ke gateway's Kubernetes service account dan annotate KSA dengan `iam.gke.io/gcp-service-account: claude-gateway@<proj>.iam.gserviceaccount.com`. `auth: {}` mengambilnya.         |
| Cloud Run / GCE         | Atur service's service account ke satu dengan `roles/aiplatform.user`. `auth: {}` mengambilnya.                                                                                                            |
| Tempat lain             | `auth: { service_account_json: /secrets/sa.json }`, path ke JSON key file yang di-mount sebagai secret. Field mengambil file path, bukan key contents, jadi tidak ada `${file:…}` expansion yang terlibat. |

<h4 id="microsoft-foundry">
  Microsoft Foundry
</h4>

Untuk client-side Foundry deployment, lihat [Claude Code on Microsoft Foundry](/docs/id/microsoft-foundry). Gateway-side upstream:

```yaml theme={null}
upstreams:
  - provider: foundry
    resource: example-foundry              # https://example-foundry.services.ai.azure.com
    auth: { use_azure_ad: true }        # preferred: DefaultAzureCredential / Managed Identity
    # OR an API key:
    # auth:
    #   api_key: ${FOUNDRY_API_KEY}
```

`use_azure_ad: true` resolves melalui `DefaultAzureCredential`: Managed Identity pada AKS, ACI, atau App Service; Azure CLI; atau environment credentials. API keys bekerja tetapi project-wide dan tidak rotate secara otomatis. Foundry's endpoint diturunkan dari `resource:`; atur optional `base_url` untuk override untuk sovereign clouds seperti Azure Government.

| Setup                   | Bagaimana                                                                                                                                                                       |
| ----------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| RBAC                    | Berikan gateway's identity `Azure AI User` atau `Cognitive Services User` pada Foundry resource                                                                                 |
| Deployments             | Foundry menggunakan admin-chosen deployment names, bukan canonical model IDs. Tambahkan [`models:`](#models) block memetakan setiap canonical ID ke deployment name Anda.       |
| AKS (workload identity) | Federate User-Assigned Managed Identity dengan cluster's OIDC issuer dan bind ke gateway's service account. `use_azure_ad: true` mengambilnya via `WorkloadIdentityCredential`. |
| ACI / App Service       | Enable system-assigned atau user-assigned managed identity pada resource. `use_azure_ad: true` mengambilnya.                                                                    |
| Tempat lain             | `auth: { api_key: "${FOUNDRY_API_KEY}" }`. Quote `${…}` di dalam `{ }`.                                                                                                         |

<h4 id="multiple-upstreams">
  Multiple upstreams
</h4>

Provider yang sama dapat muncul lebih dari sekali dengan distinct `name:`. Ini mencakup different regions, different accounts via different credential chains, provisioned throughput versus on-demand, dan cross-provider fallback.

Gateway mencoba upstreams secara berurutan. `5xx`, `429`, `401`, `403`, `404`, timeouts, dan missing-endpoint (`501`) failover; `4xx` lainnya tidak.

`429` adalah per-upstream capacity, jadi provisioned-throughput (PT) exhaustion failover ke on-demand. `404` adalah per-upstream model availability, jadi upstream yang belum enable model tidak memblokir upstream yang lebih baru dalam list yang melayaninya. Upstream yang tidak dapat menyelesaikan model yang diminta dilewati tanpa network round-trip.

Contoh ini route provisioned-throughput Bedrock allotment pertama, overflow ke on-demand dan second account, dan fallback ke Anthropic API terakhir:

```yaml theme={null}
upstreams:
  # Primary: provisioned throughput in your home region.
  - name: bedrock-pt
    provider: bedrock
    region: us-east-1
    auth: {}
  # Overflow: on-demand cross-region.
  - name: bedrock-od
    provider: bedrock
    region: us-west-2
    auth: {}
  # Different account: a separate Bedrock allotment via assumed-role creds.
  - name: bedrock-acct2
    provider: bedrock
    region: us-east-1
    auth:
      aws_access_key_id: ${ACCT2_AKID}
      aws_secret_access_key: ${ACCT2_SK}
  # Last resort: direct Anthropic API.
  - name: anthropic-fallback
    provider: anthropic
    auth:
      api_key: ${ANTHROPIC_API_KEY}

# Per-upstream model IDs are keyed on the upstream's `name:`; an upstream
# without a `name:` defaults to its provider string (e.g. `bedrock`). Any
# upstream not listed for a model is skipped, which is how you route a model
# to provisioned throughput while everything else stays on-demand.
models:
  - id: claude-opus-4-8
    label: Claude Opus 4.8
    upstream_model:
      bedrock-pt: arn:aws:bedrock:us-east-1:111111111111:provisioned-model/abcdef
      bedrock-od: us.anthropic.claude-opus-4-8
      bedrock-acct2: us.anthropic.claude-opus-4-8
      anthropic-fallback: claude-opus-4-8
```

| Lever                  | Bagaimana                                                                                                                                                                                                                                       |
| ---------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Different regions      | Satu Bedrock upstream per region, masing-masing dengan `region:` sendiri. Dengan [`auto_include_builtin_models: true`](#models) cross-region inference profiles route secara otomatis; untuk region-pinned deployments gunakan `models:` block. |
| Different accounts     | Satu Bedrock upstream per account, masing-masing dengan credentials sendiri di `auth:`. Default chain (`auth: {}`) menggunakan pod's identity; untuk second account, atur explicit credentials atau bearer token.                               |
| Provisioned throughput | Map model ke provisioned-throughput ARN di `models:` untuk upstream's name itu. Upstreams lainnya simpan on-demand ID, jadi PT capacity exhausted sebelum failover.                                                                             |
| VPC / FIPS endpoints   | Atur `base_url:` pada upstream ke VPC endpoint atau FIPS endpoint URL Anda                                                                                                                                                                      |
| Model-scoped routing   | Omit upstream dari model's `upstream_model:` map dan upstream itu dilewati untuk model itu. Misalnya, route Opus ke provisioned throughput dan Sonnet dan Haiku ke on-demand.                                                                   |

Failover antara cloud providers, atau ke direct Anthropic API, mengubah agreement, geography, dan terms lainnya yang mengatur request.

CLI menerapkan feature gating yang sama ke gateways terlepas dari upstream mana yang melayani request tertentu, jadi failover tidak mengirim body field yang upstream akan tolak.

<h2 id="optional-sections">
  Bagian opsional
</h2>

<h3 id="admin">
  `admin`
</h3>

Opsional. Mengaktifkan `/v1/organizations/spend_limits`, yang mencerminkan Anthropic's public Admin API, dan per-developer spend enforcement pada `/v1/messages`. Lihat [Spend limits](/docs/id/claude-apps-gateway-spend-limits) untuk bagaimana caps ditetapkan dan ditegakkan; bagian ini mencakup `gateway.yaml` keys yang mengaktifkan fitur dan menyetelnya.

```yaml theme={null}
admin:
  # Named static API keys for the admin endpoints, sent as x-api-key.
  # The id appears in the audit log as admin-key:<id> so each key is
  # attributable. Array for rotation: add the new key, roll clients,
  # remove the old.
  write_keys:
    - { id: terraform, key: "${GATEWAY_ADMIN_WRITE_KEY_TF}" }
    - { id: ci,        key: "${GATEWAY_ADMIN_WRITE_KEY_CI}" }
  read_keys:
    - { id: reporting, key: "${GATEWAY_ADMIN_READ_KEY}" }
  # IdP groups granted full admin via the normal gateway JWT (no API key).
  admin_groups: [platform-finops]
  blocked_message: request an increase at https://go.example.com/claude-limits
```

| Field                     | Diperlukan | Deskripsi                                                                                                                                                                                                                                                                                |
| ------------------------- | ---------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `write_keys`              | Tidak      | Array dari `{id, key}`. `x-api-key` yang cocok dengan salah satu ini dapat list, set, dan delete spend limits. Nilai key harus minimal 32 karakter; `id`s harus unik di seluruh `read_keys` dan `write_keys`.                                                                            |
| `read_keys`               | Tidak      | Array dari `{id, key}`. Read-only: setiap endpoint `GET`, termasuk listing caps, mengambil satu berdasarkan ID, dan membaca [`/effective`](/docs/id/claude-apps-gateway-spend-limits#%2Feffective) dan [`/audit`](/docs/id/claude-apps-gateway-spend-limits#%2Faudit).                             |
| `admin_groups`            | Tidak      | Nama grup IdP. JWT gateway yang klaim `groups`-nya mencakup salah satu ini memiliki akses admin penuh, read dan write, dan audit sebagai `oidc:<sub>`. Gunakan ini untuk admin manusia; gunakan API keys untuk mesin.                                                                    |
| `blocked_message`         | Tidak      | Ditambahkan verbatim ke `429 billing_error` yang dilihat developer yang diblokir. Tulis seluruh instruksi, seperti URL atau saluran Slack. Jika tidak diatur, error adalah `spend limit reached`.                                                                                        |
| `audit_retention_days`    | Tidak      | Default `365`. Baris `admin_audit` yang lebih lama disapu.                                                                                                                                                                                                                               |
| `spend_retention_months`  | Tidak      | Default `13`. Baris counter `spend` yang lebih tua dari ini disapu. Default menyimpan tahun penuh ditambah bulan parsial saat ini untuk pelaporan year-over-year.                                                                                                                        |
| `identity_retention_days` | Tidak      | Default `90`. Last-seen TTL untuk baris `principal_emails`, yang menyimpan email, nama tampilan, dan grup setiap developer (PII). Sengaja lebih pendek dari spend retention sehingga identitas yang dihapus provisioning-nya berusia keluar sementara counter spend anonimnya tetap ada. |
| `group_limit_mode`        | Tidak      | `min` (default) atau `max`. Ketika developer ada di beberapa grup dengan caps, `min` memberlakukan yang paling ketat dan `max` yang paling longgar. Digunakan oleh enforcement dan `/effective`.                                                                                         |

<h3 id="enforcement">
  `enforcement`
</h3>

Blok `enforcement` mengontrol bagaimana pemeriksaan spend-limit berperilaku ketika store tidak tersedia.

| Field                  | Diperlukan | Deskripsi                                                                                                                                                                                                                                                                                |
| ---------------------- | ---------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `fail_closed_on_error` | Tidak      | Default `false`. Enforcement spend gagal terbuka pada pemadaman Postgres, jadi inference tetap aktif. Atur `true` untuk gagal tertutup: developer yang over-cap diblokir, tetapi begitu juga semua orang jika store tidak dapat dijangkau. Tidak ada efek tanpa blok [`admin:`](#admin). |

<h3 id="models">
  `models`
</h3>

Blok `models` adalah daftar model yang dikurasi admin opsional, disajikan di `/v1/models` dan digunakan untuk menerjemahkan ID model per upstream. Ini diperlukan untuk wilayah Bedrock non-AS, ARN throughput provisioned Bedrock, dan nama deployment Foundry.

```yaml theme={null}
auto_include_builtin_models: true   # false: expose only the list below
models:
  - id: claude-opus-4-8
    label: Claude Opus 4.8
    # description: optional text shown in clients that surface it
    upstream_model:
      anthropic: claude-opus-4-8
      bedrock: us.anthropic.claude-opus-4-8   # or an inference-profile ARN
      foundry: your-opus-deployment-name
```

<h3 id="managed">
  `managed`
</h3>

Blok `managed` mendefinisikan kebijakan akses berbasis peran yang dikunci pada grup IdP atau domain email. Kebijakan dievaluasi secara berurutan; kecocokan pertama dipilih, kemudian digabungkan ke basis catch-all `match: {}` yang dijelaskan di bawah. Mereka disajikan per-user di `GET /managed/settings` dengan caching ETag/304.

```yaml theme={null}
managed:
  policies:
    # Specific groups first.
    - match: { groups: [eng-contractors] }
      cli:
        availableModels: [claude-sonnet-4-6]
        permissions: { deny: ["WebFetch", "WebSearch"] }
    # Default catch-all last: matches everyone who authenticated.
    - match: {}
      cli:
        availableModels: [claude-opus-4-8, claude-sonnet-4-6, claude-haiku-4-5]
```

Catch-all `match: {}`, secara konvensional terdaftar terakhir, diperlakukan sebagai lapisan dasar. Setiap kebijakan lainnya mewarisi kunci apa pun yang tidak ditetapkan dari catch-all, jadi entri per-peran hanya perlu mencantumkan apa yang berbeda dari default org. Aturan penggabungan tergantung pada jenis kunci:

* **Allow-lists**: `availableModels` dan `permissions.allow`. Daftar kebijakan spesifik sepenuhnya menggantikan daftar dasar.
* **Deny-lists dan hook arrays**: `permissions.deny`, `permissions.ask`, `disabledMcpjsonServers`, `deniedMcpServers`, `blockedMarketplaces`, dan setiap array jenis event `hooks`. Ini mengambil union dari dasar dan kebijakan, jadi deny org-wide atau audit hook tidak dapat secara tidak sengaja dijatuhkan oleh override per-peran.
* **Record-typed keys**: `env`, `modelOverrides`, dan `skillOverrides`. Ini shallow-merge, jadi blok `env` per-peran menimpa kunci yang ditetapkan dan mewarisi sisanya dari dasar.

`availableModels` juga ditegakkan server-side di `/v1/messages`, jadi model yang ditolak mengembalikan `400` terlepas dari apa yang dikirim klien.

| Matcher                                             | Perilaku                                                                                                                                 |
| --------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------- |
| `match: {}`                                         | Cocok dengan setiap pengguna yang terautentikasi. Mulai dengan satu ini dan tambahkan kebijakan yang dibatasi grup di atasnya nanti.     |
| `match: { groups: [a, b] }`                         | Cocok jika klaim `groups` JWT berisi salah satu grup yang terdaftar. Case-sensitive: grup harus cocok dengan casing yang tepat dari IdP. |
| `match: { email_domain: example.com }`              | Cocok dengan bagian setelah `@` terakhir dalam klaim `email` JWT, case-insensitive. Menerima satu domain per kebijakan.                  |
| `match: { groups: [a], email_domain: example.com }` | Kedua kondisi harus cocok                                                                                                                |

Pengguna yang terautentikasi yang tidak cocok dengan kebijakan apa pun mendapat default gateway, yang berarti setiap model dalam katalog dan tidak ada pengaturan terkelola. Tambahkan catch-all `match: {}` terakhir jika Anda menginginkan kebijakan default yang dijamin.

<Note>
  Gateway tidak menyimpan direktori pengguna sendiri. Ini mengotorisasi setiap permintaan dari token IdP pengguna, membaca keanggotaan grup dari klaim `groups` token dan mengevaluasi kebijakan terhadapnya. Tidak ada roster untuk dihitung dan tidak ada akun untuk dibuat sebelumnya, dan oleh karena itu tidak ada endpoint SCIM, karena tidak ada apa pun untuk SCIM sinkronkan ke.

  Jalankan manajemen siklus hidup pengguna dan grup di sumber kebenaran, yang merupakan penyediaan SCIM asli IdP atau platform tata kelola identitas khusus. Keanggotaan dan deprovisioning yang diatur di sana mengalir ke gateway secara otomatis melalui token. Jika Anda menginginkan penyediaan SCIM dari akun Claude itu sendiri, itu adalah kemampuan [Claude for Enterprise](/docs/id/admin-setup).

  Dua jam propagasi berlaku:

  * **Konten kebijakan**: mengedit kebijakan dan redeploy mencapai klien yang terhubung pada polling managed-settings berikutnya mereka, dalam satu jam
  * **Keanggotaan grup**: mengubah keanggotaan grup pengguna mengubah kebijakan mana yang cocok dengan mereka. Ini berlaku pada re-mint sesi berikutnya, berarti refresh senyap berikutnya, dibatasi oleh `session.ttl_hours`.
</Note>

<h4 id="what-goes-in-cli">
  Apa yang masuk di `cli`
</h4>

Setiap nilai `cli` adalah dokumen `managed-settings.json` Claude Code yang lengkap, skema yang sama yang akan Anda deploy melalui MDM atau `/etc/claude-code/managed-settings.json`, diekspresikan di sini sebagai YAML. CLI menerapkan dokumen yang dikirimkan pada tingkat terkelola, di atas pengaturan pengguna dan proyek.

Gateway memvalidasi setiap dokumen terhadap skema pengaturan CLI saat boot, jadi kunci tingkat atas yang tidak dikenali atau kunci yang dikenali dengan nilai yang salah bentuk gagal boot dengan error yang menamai setiap kunci yang bermasalah. Bagian skema yang sengaja terbuka masih menerima nilai arbitrer, karena klien yang lebih baru mungkin mengenali entri yang skema gateway tidak. Kunci terbuka ini adalah `env`, `pluginConfigs`, dan kunci yang bersarang di bawah `permissions`.

Karena validasi menggunakan skema yang disertakan dengan versi gateway yang terinstal, menempatkan kunci pengaturan tingkat atas yang diperkenalkan oleh rilis Claude Code yang lebih baru ke konfigurasi terkelola memerlukan upgrade gateway terlebih dahulu. Smoke-test kebijakan baru pada satu klien sebelum meluncurkannya.

Referensi kunci lengkap ada di [Claude Code settings](/docs/id/settings#available-settings). Kunci yang paling sering dicari operator:

```yaml theme={null}
managed:
  policies:
    - match: {}
      cli:
        # Model access (also enforced server-side at /v1/messages)
        availableModels: [claude-opus-4-8, claude-sonnet-4-6, claude-haiku-4-5]

        # Permission policy
        permissions:
          deny:
            - "WebFetch"
            - "Read(./.env)"
            - "Read(./secrets/**)"
          disableBypassPermissionsMode: disable   # blocks --dangerously-skip-permissions
        allowManagedPermissionRulesOnly: true     # ignore user/project permission rules

        # Environment pushed into the CLI process. DISABLE_UPDATES blocks
        # background and manual updates; DISABLE_AUTOUPDATER stops only
        # background updates.
        env:
          DISABLE_UPDATES: "1"                    # pin versions via your own distribution

        # Org-wide hooks. Hook commands run on developer machines, not the
        # gateway, so the path must exist on every client OS in the policy.
        hooks:
          PostToolUse:
            - matcher: "Edit|Write"
              hooks:
                - { type: command, command: /usr/local/bin/audit-edit.sh }
```

| Key                                        | Ditegakkan oleh | Efek                                                                                                                                                                                                       |
| ------------------------------------------ | --------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `availableModels`                          | Gateway + CLI   | Allowlist model. Juga diperiksa di `/v1/messages`, jadi klien yang dipatch tidak dapat membypassnya.                                                                                                       |
| `permissions.allow` / `.deny`              | CLI             | Aturan tool dan command. Lihat [Permissions](/docs/id/permissions).                                                                                                                                             |
| `permissions.disableBypassPermissionsMode` | CLI             | Atur ke `disable` untuk memblokir [`bypassPermissions`](/docs/id/permission-modes#skip-all-checks-with-bypasspermissions-mode), mode yang melewati prompt permission, dan flag `--dangerously-skip-permissions` |
| `allowManagedPermissionRulesOnly`          | CLI             | Ketika `true`, aturan permission pengguna dan proyek diabaikan; hanya aturan dari dokumen ini yang berlaku                                                                                                 |
| `env`                                      | CLI             | Variabel lingkungan yang digabungkan ke proses CLI. Gunakan untuk telemetry, auto-update, dan model-name overrides.                                                                                        |
| `hooks`                                    | CLI             | Org-wide [hooks](/docs/id/hooks)                                                                                                                                                                                |

Karena pengaturan ini tiba melalui jaringan, CLI menunjukkan setiap developer dialog persetujuan keamanan satu kali sebelum menerapkan apa pun yang dapat menjalankan perintah shell atau mengubah ke mana traffic pergi. Dialog mencakup:

* `hooks`
* Variabel `env` yang tidak ada di daftar aman bawaan CLI
* pengaturan eksekusi shell seperti `apiKeyHelper` dan `statusLine`
* konten CLAUDE.md yang terkelola

Daftar aman menentukan variabel `env` mana yang berlaku tanpa persetujuan:

* **Pada daftar aman**: auto-update dan model-name vars
* **Tidak pada daftar aman**: proxy vars, base-URL vars, dan `OTEL_EXPORTER_OTLP_ENDPOINT`

Konfigurasi [telemetry](#telemetry) gateway mendorong `OTEL_EXPORTER_OTLP_ENDPOINT`, jadi pengaturan `telemetry.forward_to` memicu dialog pada setiap klien interaktif. Run non-interaktif dengan flag `-p` tidak dapat menampilkan dialog. Ini menerapkan pengaturan yang didorong untuk run itu saja dan tidak merekamnya sebagai disetujui, jadi sesi interaktif berikutnya developer masih menampilkan dialog. Sebelum v2.1.207, run non-interaktif menyimpan pengaturan sebagai disetujui dan tidak ada sesi interaktif yang lebih baru menampilkan dialog untuk mereka.

Jika developer menolak, Claude Code keluar daripada menerapkan kebijakan. Mendorong hook baru atau variabel env non-aman ke kebijakan yang luas oleh karena itu berarti prompt persetujuan pada startup berikutnya setiap developer yang cocok.

Kunci `cli` dinamai `settings` dalam rilis sebelumnya. Ejaan itu masih diterima sebagai alias, tetapi deployment baru harus menggunakan `cli`.

<h4 id="precedence-with-other-managed-sources">
  Precedence dengan sumber terkelola lainnya
</h4>

Jika perangkat juga memiliki `managed-settings.json` lokal atau kebijakan yang dikirimkan MDM, sumber terkelola tidak bergabung. Sumber prioritas tertinggi menyediakan semua pengaturan kebijakan, diurutkan dalam urutan ini dengan prioritas tertinggi terlebih dahulu:

1. [Policy helper](/docs/id/settings#compute-managed-settings-with-a-policy-helper)
2. Pengaturan yang dikirimkan gateway
3. MDM, melalui registri HKLM di Windows atau plist di macOS
4. File `managed-settings.json`
5. Registri HKCU, hanya di Windows

Host embedding dapat menyediakan kebijakan melalui opsi SDK `managedSettings`. Ini diabaikan secara default dan berlaku hanya ketika sumber terkelola opt in dengan [`parentSettingsBehavior: "merge"`](/docs/id/settings#available-settings), disaring sehingga dapat mengencangkan kebijakan tetapi tidak melonggarkannya.

Satu-satunya pengecualian adalah kunci berikut, yang dihormati ketika sumber admin apa pun di atas tingkat HKCU yang dapat ditulis pengguna menetapkannya, terlepas dari sumber mana yang menyediakan sisa kebijakan:

* `sandbox.network.allowManagedDomainsOnly` dan `sandbox.filesystem.allowManagedReadPathsOnly`: ketika terkunci, allowlist yang sesuai adalah union di seluruh sumber
* [`allowAllClaudeAiMcps`](/docs/id/settings#available-settings): override allow-only untuk allowlist server MCP claude.ai
* `sandbox.bwrapPath` dan `sandbox.socatPath`: jalur filesystem ke binary helper [sandbox](/docs/id/sandboxing)
* [`forceRemoteSettingsRefresh`](/docs/id/server-managed-settings): memblokir startup hingga pengaturan terkelola jarak jauh segar diambil, jadi kebijakan MDM atau file yang menetapkannya dihormati bahkan ketika payload jarak jauh yang di-cache yang kekurangan kunci adalah sumber prioritas tertinggi

Setiap kunci lainnya, termasuk `allowManagedPermissionRulesOnly` dan `disableBypassPermissionsMode`, berasal dari sumber prioritas tertinggi saja. Lihat [Settings precedence](/docs/id/settings#settings-precedence) untuk aturan yang sama di halaman pengaturan.

Kebijakan gateway berlaku untuk setiap invokasi Claude Code pada mesin, termasuk run non-interaktif `claude -p` dan sesi yang dihasilkan oleh Agent SDK. Jika gateway tidak dapat dijangkau saat startup, sesi yang masuk keluar dengan error daripada menjalankan tanpa kebijakan mereka.

<Warning>
  `mcpServers` di dalam blok `cli` kebijakan ditolak saat boot gateway. Distribusi MCP per-grup tidak tersedia; deploy server MCP melalui `managed-mcp.json` berbasis file pada setiap perangkat atau biarkan developer menambahkannya secara lokal.
</Warning>

<h3 id="telemetry">
  `telemetry`
</h3>

CLI mengirim metrik, log, dan, ketika diaktifkan, trace OpenTelemetry Protocol (OTLP) melalui HTTP ke gateway, yang meneruskan mereka verbatim ke setiap tujuan yang dikonfigurasi. Lihat [Monitoring usage](/docs/id/monitoring-usage) untuk metrik dan event yang CLI emit.

CLI memberi stempel setiap export dengan identitas pengguna yang terautentikasi, dibaca dari JWT yang diterbitkan gateway: atribut `user.id`, `user.email`, dan `user.groups`. Atribusi biaya dan penggunaan per-developer oleh karena itu bekerja tanpa konfigurasi sisi developer.

```yaml theme={null}
telemetry:
  forward_to:
    - url: https://otel-collector.internal.example.com
      headers:
        Authorization: ${OTLP_TOKEN}
      # Per-signal opt-in. Default: metrics only.
      metrics: true
      logs: false
      traces: false
    - url: https://api.datadoghq.com/api/v2/otlp
      headers:
        DD-API-KEY: ${DD_API_KEY}
```

<Warning>
  Setiap tujuan opt into `metrics`, `logs`, dan `traces` secara independen, dan default adalah metrics saja. Signal berbeda dalam sensitivitas:

  * **Metrics**: counter agregat seperti token counts, request counts, dan latency
  * **Logs dan traces**: dapat membawa perintah bash lengkap, tool inputs, dan jalur file, mencakup apa pun yang Claude Code lakukan pada mesin developer

  Aktifkan logs dan traces hanya pada tujuan dengan kontrol akses dan kebijakan retensi yang data jamin.
</Warning>

Telemetry off dalam CLI secara default. Mengonfigurasi `telemetry.forward_to` bersama dengan `listen.public_url` mengaktifkannya. Gateway mendorong lima variabel env ke setiap klien yang terhubung melalui `/managed/settings`:

* `CLAUDE_CODE_ENABLE_TELEMETRY=1`
* `OTEL_METRICS_EXPORTER=otlp`
* `OTEL_LOGS_EXPORTER=otlp`
* `OTEL_TRACES_EXPORTER=otlp`
* `OTEL_EXPORTER_OTLP_ENDPOINT=<public_url>`

Endpoint yang didorong dibangun dari URL publik, jadi metrik dan log tidak memerlukan konfigurasi OTEL dari developer atau kebijakan. Konfigurasi yang didorong diterapkan pada tingkat terkelola, menimpa variabel `OTEL_*` yang developer atur secara lokal.

[Traces](/docs/id/monitoring-usage#traces-beta) selain itu memerlukan `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA=1` pada setiap klien. Gateway tidak mendorong variabel itu, jadi atur melalui blok `env` kebijakan terkelola. Ini tidak pada daftar aman CLI, jadi mengirimkannya melalui kebijakan dicakup oleh dialog [security approval](#managed) yang sama yang endpoint OTLP yang didorong sudah trigger.

Kedua encoding OTLP protobuf dan JSON direlai, dan backend apa pun yang kompatibel dengan OpenTelemetry bekerja sebagai tujuan.

<h3 id="http-tuning">
  HTTP tuning
</h3>

Empat blok tingkat atas opsional, `access_control`, `limits`, `timeouts`, dan `rate_limits`, menyetel permukaan HTTP. Default cocok untuk sebagian besar deployment.

| Block            | Key                                            | Default  | Deskripsi                                                                                                                                                                                                                                                                                                                                           |
| ---------------- | ---------------------------------------------- | -------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `access_control` | `allow_cidrs` / `deny_cidrs`                   | empty    | Inbound IP allow/deny berdasarkan alamat klien, setelah resolusi `trusted_proxies`. `deny_cidrs` diperiksa terlebih dahulu; klien yang cocok ditolak bahkan jika `allow_cidrs` juga cocok. Jika `allow_cidrs` non-empty gateway adalah default-deny. `/healthz` dan `/readyz` dikecualikan dari `allow_cidrs`.                                      |
| `limits`         | `max_request_bytes`                            | 32 MiB   | Max inbound request body; permintaan yang terlalu besar mendapat `413` sebelum body dibuffer. Naikkan untuk permintaan file atau gambar besar.                                                                                                                                                                                                      |
| `limits`         | `max_request_header_bytes`                     | unset    | Ketika diatur, header yang terlalu besar mengembalikan `431`                                                                                                                                                                                                                                                                                        |
| `limits`         | `max_url_length`                               | unset    | Ketika diatur, URL yang terlalu panjang mengembalikan `414`                                                                                                                                                                                                                                                                                         |
| `timeouts`       | `upstream_ttfb_ms`                             | 120000   | Max wait untuk header respons upstream (time to first byte). Response body kemudian stream tanpa wall-clock cap. Berlaku ke jalur upstream Anthropic langsung; setiap penyedia lainnya dibatasi oleh timeout SDK penyedia mereka sendiri.                                                                                                           |
| `rate_limits`    | `device_authorization.max` / `.window_seconds` | 30 / 600 | Per-IP rate limit pada endpoint device-authorization yang tidak terautentikasi. Naikkan untuk org besar di belakang IP egress bersama atau NAT. Limit ini berlaku hanya ke alur sign-in device-grant, bukan ke inference `/v1/messages`. Lihat [User-code brute-force resistance](/docs/id/claude-apps-gateway-deploy#user-code-brute-force-resistance). |
| `rate_limits`    | `device_verify.max` / `.window_seconds`        | 10 / 600 | Per-IP rate limit pada pengajuan `user_code` di `/device`                                                                                                                                                                                                                                                                                           |

<h2 id="complete-example">
  Contoh lengkap
</h2>

Config reference penuh ini menggunakan setiap core section; [HTTP tuning blocks](#http-tuning) menyimpan defaults mereka. Salin, hapus apa yang Anda tidak butuhkan, dan isi values Anda. Config dalam [Quickstart](/docs/id/claude-apps-gateway#quickstart) adalah minimal version dari ini.

```yaml gateway.yaml theme={null}
# Run with:
#   claude gateway --config gateway.yaml
#
# Operational log verbosity is controlled by the CLAUDE_GATEWAY_LOG_LEVEL
# environment variable (info | warn | error; default info). It does not
# affect audit events, which are always emitted.

listen:
  host: 0.0.0.0
  port: 8080
  public_url: https://claude-gateway.internal.example.com
  # Omit the tls block when running behind a TLS-terminating ingress.
  # tls:
  #   cert: /certs/gateway.crt
  #   key: /certs/gateway.key
  # trusted_proxies:
  #   - 10.0.0.0/8

oidc:
  issuer: https://example.okta.com
  client_id: 0oa1example2
  client_secret: ${OIDC_CLIENT_SECRET}
  allowed_email_domains:
    - example.com
  # Required when the issuer is the Okta org server, whose id_tokens
  # can omit email and groups; the gateway fills them from /userinfo.
  userinfo_fallback: true
  # allowed_groups: [claude-code-users]
  # Okta emits groups only when the `groups` scope is requested and the
  # app's groups claim filter allows them. The contractors policy below
  # matches on groups, so the scope is requested here.
  scopes: [openid, profile, email, offline_access, groups]
  # extra_auth_params: { access_type: offline, prompt: consent }  # Google
  # groups_claim: groups          # Entra app roles: use `roles`
  # email_claim: email

session:
  jwt_secret: ${GATEWAY_JWT_SECRET}   # openssl rand -base64 32
  # ttl_hours: 1

store:
  postgres_url: ${GATEWAY_POSTGRES_URL}
  # max_connections: 5

# Enables /v1/organizations/spend_limits (mirrors the Anthropic Admin API)
# and per-developer spend enforcement on /v1/messages. Omit to disable.
# Caps themselves are set via the admin API, not here.
# admin:
#   write_keys:
#     - { id: terraform, key: "${GATEWAY_ADMIN_WRITE_KEY_TF}" }
#   read_keys:
#     - { id: reporting, key: "${GATEWAY_ADMIN_READ_KEY}" }
#   admin_groups: [platform-finops]
#   blocked_message: request an increase at https://go.example.com/claude-limits
#   # audit_retention_days: 365
#   # spend_retention_months: 13
#   # identity_retention_days: 90
#   # group_limit_mode: min

# enforcement:
#   fail_closed_on_error: false

upstreams:
  - provider: anthropic
    auth:
      api_key: ${ANTHROPIC_API_KEY}

  # - provider: bedrock
  #   region: us-east-1
  #   auth: {}

  # - provider: anthropicAws
  #   region: us-east-1
  #   workspace_id: wrkspc_...
  #   auth:
  #     api_key: ${ANTHROPIC_AWS_API_KEY}

  # - provider: vertex
  #   region: us-east5
  #   project_id: example-prod
  #   auth: {}

  # - provider: foundry
  #   resource: example-foundry
  #   auth: { use_azure_ad: true }

auto_include_builtin_models: true
models:
  - id: claude-opus-4-8
    label: Claude Opus 4.8
    upstream_model:
      anthropic: claude-opus-4-8
      # bedrock: us.anthropic.claude-opus-4-8
      # anthropicAws: claude-opus-4-8
      # vertex: claude-opus-4-8
      # foundry: <your-opus-deployment-name>
  - id: claude-sonnet-4-6
    label: Claude Sonnet 4.6
    upstream_model:
      anthropic: claude-sonnet-4-6
  - id: claude-haiku-4-5
    label: Claude Haiku 4.5
    upstream_model:
      anthropic: claude-haiku-4-5

managed:
  policies:
    - match: { groups: [contractors] }
      cli:
        availableModels: [claude-haiku-4-5]
        # Constrain the Default picker option to availableModels instead of
        # the tier default, so contractors don't get a 400 on the default.
        enforceAvailableModels: true
        # allow auto-approves these tools; it does not block the rest.
        # Add deny rules to restrict tools.
        permissions: { allow: [Read, Grep] }
    - match: {}
      cli:
        availableModels: [claude-opus-4-8, claude-sonnet-4-6, claude-haiku-4-5]
        permissions:
          allow: [Read, Grep, Bash, Edit]
          deny: ["WebFetch"]
        env: { HTTP_PROXY: http://proxy.example.com:8080 }

telemetry:
  forward_to:
    - url: https://otel.internal.example.com:4318
      headers:
        Authorization: Bearer ${OTEL_TOKEN}
```

<h2 id="client-side-managed-settings">
  Managed settings sisi client
</h2>

Semua di atas mengonfigurasi gateway server. Menunjukkan developer machines ke sana dikonfigurasi secara terpisah, pada setiap device, melalui Claude Code's [managed settings](/docs/id/settings#settings-files). Gateway tidak dapat mendorong keys ini sendiri, karena mereka adalah apa yang memberitahu client di mana gateway berada.

Untuk CLI, atur kedua keys dalam per-OS `managed-settings.json`:

```json theme={null}
{
  "forceLoginMethod": "gateway",
  "forceLoginGatewayUrl": "https://claude-gateway.internal.example.com"
}
```

Deploy file itu ke setiap device, typically via MDM platform Anda. File path berbeda by platform:

| Platform      | Path                                                                                                                        |
| ------------- | --------------------------------------------------------------------------------------------------------------------------- |
| macOS         | `/Library/Application Support/ClaudeCode/managed-settings.json`, atau `com.anthropic.claudecode` managed preferences domain |
| Linux dan WSL | `/etc/claude-code/managed-settings.json`                                                                                    |
| Windows       | `C:\Program Files\ClaudeCode\managed-settings.json`, atau Group Policy via HKLM registry                                    |

`forceLoginGatewayUrl`, dan `"gateway"` value dari `forceLoginMethod`, dihormati hanya dari admin-controlled managed tier. Developer menetapkan mereka dalam `~/.claude/settings.json` mereka sendiri tidak memiliki efek.

<h2 id="related">
  Terkait
</h2>

* [Claude apps gateway overview](/docs/id/claude-apps-gateway): quickstart dan developer connection
* [Deployment guide](/docs/id/claude-apps-gateway-deploy): IdP setup, container image, Kubernetes dan Cloud Run, dan operations
* [Spend limits](/docs/id/claude-apps-gateway-spend-limits): per-developer caps dan Admin API
