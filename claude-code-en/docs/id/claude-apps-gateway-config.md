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
* [`pricing`](#pricing): contracted rates dan multiplier untuk spend meter dan untuk cost figures yang dilihat developer
* [`models`](#models) dan `auto_include_builtin_models`: daftar model yang dikurasi admin dan per-upstream IDs
* [`managed`](#managed): managed settings policies berdasarkan IdP group
* [`telemetry`](#telemetry): OTLP forwarding ke observability stack Anda
* [`access_control`, `limits`, `timeouts`, `rate_limits`](#http-tuning): IP allow/deny, request size caps, upstream time-to-first-byte, dan per-IP sign-in limits
* [`load_test_mode`](#load_test_mode): load test gateway tanpa memanggil model provider

<h2 id="secret-expansion">
  Ekspansi secret
</h2>

Jangan tulis secrets seperti `client_secret`, `jwt_secret`, atau `postgres_url` langsung di `gateway.yaml`. Referensikan mereka dengan salah satu bentuk di bawah, dan gateway menyelesaikan nilai saat boot dari environment variable atau file:

| Bentuk          | Diselesaikan ke                                                                                                                                                                                                                                                        | Gunakan untuk                                                          |
| --------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------- |
| `${VAR}`        | Environment variable `VAR`. Boot gagal jika tidak terdefinisi.                                                                                                                                                                                                         | Container environment variables, AWS Secrets Manager via env injection |
| `${file:/path}` | Isi file pada path absolut tersebut, dipangkas. Referensi harus menjadi seluruh nilai field: tidak seperti `${VAR}`, tidak diperluas di dalam string yang lebih panjang, jadi untuk password database atur `store.password` daripada menyematkannya di `postgres_url`. | Kubernetes Secret volume mounts, Vault Agent, SOPS                     |

<h2 id="required-sections">
  Bagian yang diperlukan
</h2>

<h3 id="listen">
  `listen`
</h3>

Blok `listen` mengontrol di mana gateway melayani: bind address dan port, origin yang terlihat secara eksternal, dan optional TLS termination.

| Field                  | Diperlukan                     | Deskripsi                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| ---------------------- | ------------------------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `host`                 | Tidak                          | Bind address. Default `0.0.0.0`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `port`                 | Tidak                          | Bind port. Default `8080`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `public_url`           | Kecuali `host` adalah loopback | Origin `https://` yang terlihat secara eksternal, digunakan untuk membangun IdP `redirect_uri` dan discovery metadata. Diperlukan setiap kali `host` bukan loopback address, baik TLS menghentikan di proxy seperti ALB, Ingress, atau Cloud Run atau di gateway itu sendiri melalui `tls`, karena gateway tidak pernah menurunkan origin-nya sendiri dari header `X-Forwarded-*`; mereka dapat dipalsukan oleh client. Boot gagal tanpa itu. `trusted_proxies` di bawah mengatur resolusi client-IP saja. Juga diperlukan untuk mengaktifkan [telemetry](#telemetry), karena gateway membangun endpoint OTLP yang didorong ke client dari URL ini. |
| `tls.cert` / `tls.key` | Tidak                          | Path PEM jika gateway menghentikan TLS sendiri                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `trusted_proxies`      | Tidak                          | CIDR atau IP dari load balancer di depan gateway. Ketika diatur, gateway mempercayai `X-Forwarded-For` hanya dari peer ini dan mencatat IP client yang sebenarnya untuk per-IP rate limiting dan audit. Setara dengan nginx `set_real_ip_from`. Entri `X-Forwarded-For` yang ditulis sebagai `ipv4:port` atau `[ipv6]:port`, seperti yang dilakukan beberapa load balancer, dibaca dengan port dihilangkan. Alamat IPv6 dengan port ditambahkan dan tanpa bracket dapat dibaca sebagai alamat yang berbeda atau tidak dibaca sama sekali, jadi matikan opsi port pada proxy apa pun yang menulis bentuk itu.                                        |

<h3 id="oidc">
  `oidc`
</h3>

Blok `oidc` menghubungkan gateway ke identity provider Anda dan memutuskan siapa yang dapat masuk. Ini menamai issuer dan OAuth client, memetakan claims yang membawa email dan groups, dan membatasi sign-in berdasarkan email domain atau group.

OpenID Connect (OIDC) adalah protokol SSO yang digunakan gateway dengan identity provider Anda; lihat [Identity provider setup](/docs/id/claude-apps-gateway-deploy#identity-provider-setup) untuk apa yang harus didaftarkan di sisi IdP.

| Field                           | Diperlukan | Deskripsi                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| ------------------------------- | ---------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `issuer`                        | Ya         | OIDC discovery base. Harus melayani discovery di `/.well-known/openid-configuration`. Gunakan HTTPS dalam production; gateway menerima issuer `http://`. Issuer loopback seperti `http://localhost:8081` ditolak oleh [SSRF guard](/docs/id/claude-apps-gateway-deploy#threat-model-summary) kecuali `CLAUDE_GATEWAY_ALLOW_LOOPBACK=1` diatur di environment gateway.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `client_id` / `client_secret`   | Ya         | Dari registrasi OAuth client Anda                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `allowed_email_domains`         | Tidak      | Tolak id\_tokens yang claim `email`-nya tidak ada di salah satu domain ini, case-insensitive. Defense-in-depth terhadap misconfiguration IdP multi-tenant. Independen dari setting ini, id\_token yang claim `email_verified`-nya secara eksplisit `false` selalu ditolak.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `allowed_groups`                | Tidak      | Batasi sign-in ke anggota IdP groups ini, dicocokkan terhadap `groups_claim`. User dalam email domain yang diizinkan tetapi tidak ada di salah satu groups ini ditolak. Memerlukan IdP untuk memancarkan groups claim. Pencocokan adalah perbandingan string yang tepat dan case-sensitive terhadap nilai dalam claim itu, dan gateway tidak memperluas nested groups: untuk mengakui anggota sub-group, daftar sub-group di sini atau konfigurasi IdP untuk memancarkan flattened membership.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `groups_claim`                  | Tidak      | Claim id\_token mana yang membawa group membership. Default `groups`. Microsoft Entra memancarkan app roles di bawah `roles`. Menerima flat key atau RFC 6901 JSON Pointer seperti `/resource_access/gateway/roles` untuk nested claims.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `google_groups`                 | Tidak      | Cari groups user yang masuk melalui Google Workspace Admin SDK Directory API, karena id\_token Google tidak membawa groups claim. Atur `service_account_json_path` ke file service-account key dengan domain-wide delegation pada scope `https://www.googleapis.com/auth/admin.directory.group.readonly`, dan `admin_email` ke administrator Workspace yang disamar oleh service account; Directory API memerlukan subject admin yang sebenarnya. Email address group setiap user menjadi groups claim mereka, jadi `allowed_groups` dan `managed.policies.match.groups` cocok pada group emails.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `email_claim`                   | Tidak      | Claim id\_token mana yang membawa email user. Default `email`. Beberapa IdP, seperti ADFS dan Entra B2C, memancarkan `upn` atau `preferred_username` sebagai gantinya. Menerima flat key, JSON Pointer, atau daftar fallback keys di mana key pertama yang ada digunakan.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `scopes`                        | Tidak      | Override lengkap dari scopes OIDC yang diminta gateway. Default `[openid, profile, email, offline_access]`. Atur ketika IdP Anda menolak scopes yang tidak dikenalinya, atau memerlukan custom scope untuk memancarkan groups atau email. Harus menyertakan `openid`. Menghilangkan `offline_access` menonaktifkan refresh tokens, jadi developer menjalankan kembali browser login setiap `session.ttl_hours`. Lihat [Identity provider setup](/docs/id/claude-apps-gateway-deploy#identity-provider-setup) untuk per-IdP scope recipes seperti Google's refresh-token flow.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `scope_on_refresh`              | Tidak      | Juga kirim `scope`, dengan daftar yang sama seperti sign-in request, ketika gateway menukar refresh token. Default `false`: refresh request menghilangkan `scope`. Sebagian besar IdP mengembalikan id\_token pada setiap refresh dan tidak memerlukan ini. Atur `true` ketika IdP Anda mengembalikan id\_token pada refresh hanya jika diminta `openid` lagi, yang Okta dokumentasikan untuk refresh grant-nya. Tanpa id\_token, setiap refresh tergantung pada endpoint userinfo IdP menerima refreshed access token. Jika Anda gate sign-in atau match policies pada groups dan id\_token refresh-time IdP Anda menghilangkan mereka, juga atur `userinfo_fallback: true` jadi gateway mengisinya dari endpoint userinfo. IdP yang memberikan fewer scopes daripada yang diminta dapat menolak refresh dengan `invalid_scope`, termasuk untuk existing sessions jika Anda menambahkan entries ke `scopes` saat ini aktif. Unset key jika refreshes mulai gagal di `token_endpoint` setelah Anda mengaturnya. Memerlukan Claude Code v2.1.260 atau lebih baru di server gateway. |
| `extra_auth_params`             | Tidak      | Extra query parameters ditambahkan ke IdP authorization request, verbatim. Ini adalah mekanisme override untuk perilaku spesifik IdP, seperti `access_type: offline` untuk Google refresh tokens, `domain_hint` untuk beberapa Entra tenants, atau `acr_values` untuk step-up flows. Tidak dapat override protocol params yang dikelola gateway: `state`, `nonce`, `redirect_uri`, PKCE, `scope`, `response_type`, `response_mode`, dan `client_id`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `userinfo_fallback`             | Tidak      | Ketika id\_token menghilangkan email atau groups, ambil dari `/userinfo`. Diperlukan untuk Keycloak lightweight access tokens, Okta org server, dan ADFS minimal tokens. Id\_token tetap authoritative; userinfo hanya mengisi gaps. Default `false`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `use_pkce`                      | Tidak      | Kirim PKCE (S256) challenge pada authorization request. Default `true`. Atur `false` hanya jika IdP Anda menolak PKCE untuk confidential client ini.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `clock_skew_seconds`            | Tidak      | Toleransi clock drift saat memvalidasi id\_token time claims. Default `0`, yang ketat. Naikkan jika Anda melihat error "token expired / not yet valid" tepat setelah sign-in karena host/IdP clock skew.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `token_endpoint_auth_method`    | Tidak      | Override token-endpoint auth method. Menerima `client_secret_basic` atau `client_secret_post`. Auto-negotiated secara default.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `id_token_signed_response_alg`  | Tidak      | Expected id\_token signing algorithm. Default `RS256`. Atur untuk IdP yang menandatangani dengan ES256, PS256, atau EdDSA.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `additional_authorized_parties` | Tidak      | Extra `azp` values untuk diterima di luar `client_id`, untuk Keycloak broker dan token-exchange flows                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `discovery_url`                 | Tidak      | Ambil discovery document dari URL ini daripada menurunkannya dari `issuer`, untuk IdP di belakang proxy yang menulis ulang issuer host. Path harus berisi `/.well-known/`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `use_proxy`                     | Tidak      | Kirim IdP requests gateway sendiri melalui forward proxy di `HTTPS_PROXY` atau `HTTP_PROXY`, menghormati `NO_PROXY`. `false` menjaga requests tersebut langsung. Memerlukan v2.1.227 atau lebih baru; lihat [IdP requests through a forward proxy](#idp-requests-through-a-forward-proxy) di bawah.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `form_action_origins`           | Tidak      | Origins tambahan untuk direktif `Content-Security-Policy: form-action` halaman `/device`. Gateway sudah mengizinkan `'self'` dan origin `authorization_endpoint` yang ditemukan, tetapi Chrome memberlakukan `form-action` terhadap seluruh redirect chain. Jika IdP Anda mengalihkan melalui host kedua, seperti Azure AD federated ke ADFS, hub-spoke Okta, atau SSO interceptor korporat, daftar setiap origin yang mungkin dialihkan oleh authorization request.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `ca_cert_pem`                   | Tidak      | PEM-encoded CA certificate itu sendiri, bukan path ke file. Ini menggantikan system trust store untuk IdP requests saja. Untuk memuat file yang di-mount, tulis `${file:/etc/gateway/idp-ca.pem}`. Gunakan untuk Keycloak atau Dex di belakang corporate PKI.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |

<h4 id="idp-requests-through-a-forward-proxy">
  IdP requests through a forward proxy
</h4>

Inference upstreams menghormati `HTTPS_PROXY` dan `HTTP_PROXY` pada setiap version. Requests gateway sendiri ke IdP, discovery, JWKS, token, dan userinfo, langsung kecuali Anda menetapkan `oidc.use_proxy: true`, yang memerlukan v2.1.227 atau lebih baru. Ketika proxy variable diatur, `use_proxy` unset, dan issuer tidak dicakup oleh `NO_PROXY`, gateway menjaga requests tersebut langsung dan mencatat notice saat boot meminta Anda memilih; `use_proxy: false` menjaganya langsung dan membisukan notice.

Dengan `use_proxy: true`, pod menyelesaikan hostname setiap IdP endpoint itu sendiri dan meminta proxy untuk `CONNECT` ke resolved IP address, jadi proxy harus menerima `CONNECT` ke IP address setiap host yang discovery document namai, bukan hanya issuer. Gunakan `http://` proxy URL. `ca_cert_pem` dan [SSRF guard](/docs/id/claude-apps-gateway-deploy#threat-model-summary) berlaku pada proxied path juga.

[Proxy-only egress](#proxy-only-egress) mengubah keduanya: saat aktif, IdP requests mengikuti proxy kecuali Anda menetapkan `use_proxy: false`, dan gateway menyerahkan proxy setiap hostname IdP tanpa menyelesaikannya terlebih dahulu.

<h4 id="proxy-only-egress">
  Proxy-only egress
</h4>

Atur `CLAUDE_GATEWAY_PROXY_IS_EGRESS_BOUNDARY=1` di environment gateway, di sebelah `HTTPS_PROXY`, ketika pod mencapai host lain hanya melalui forward proxy itu dan tidak dapat menyelesaikan public DNS names itu sendiri, atau ketika proxy menolak `CONNECT` ke IP address. Memerlukan v2.1.277 atau lebih baru. Ini adalah environment variable daripada kunci `gateway.yaml` jadi tidak ada apa pun dalam file config yang dapat melonggarkan address check gateway.

```bash theme={null}
export HTTPS_PROXY=http://proxy.corp.example.com:3128
export NO_PROXY=
export no_proxy=
export CLAUDE_GATEWAY_PROXY_IS_EGRESS_BOUNDARY=1
```

Gateway mencatat satu baris `network:` saat boot saat proxy-only egress aktif.

Setiap baris di bawah adalah satu class dari outbound request pada gateway dengan `HTTPS_PROXY` diatur, secara default dan saat proxy-only egress aktif.

| Outbound request                                                                                                             | Default                                                                                                                                                                       | Proxy-only egress active                                                                    |
| ---------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------- |
| `provider: anthropic` upstreams, Workload Identity Federation token exchange, `telemetry.forward_to` exports                 | Resolved dan checked secara lokal, kemudian `CONNECT` ke checked IP address melalui proxy. Telemetry collector yang terdaftar di `NO_PROXY` dicapai langsung sebagai gantinya | Hostname diserahkan ke proxy                                                                |
| IdP discovery, JWKS, token, dan userinfo                                                                                     | Direct kecuali [`oidc.use_proxy: true`](#idp-requests-through-a-forward-proxy), kemudian `CONNECT` ke checked IP address                                                      | Hostname diserahkan ke proxy, kecuali `oidc.use_proxy: false` menjaga internal IdP langsung |
| Amazon Bedrock, Claude Platform on AWS, Google Cloud's Agent Platform, dan Microsoft Foundry upstreams; Google group lookups | Hostname diserahkan ke proxy                                                                                                                                                  | Unchanged                                                                                   |

Proxy-only egress tetap off kecuali environment gateway memenuhi ketiga kondisi ini:

* `HTTPS_PROXY` atau `HTTP_PROXY` diatur.
* `NO_PROXY` dan `no_proxy` kosong. Jika platform Anda menyuntikkan salah satu ke pods, atur keduanya ke nilai kosong pada container gateway. Mendaftar telemetry collector di `NO_PROXY` menjaga proxy-only egress off.
* `CLAUDE_GATEWAY_ALLOW_LOOPBACK` tidak diaktifkan. Collector atau IdP pada loopback pod sendiri tidak dapat dikombinasikan dengan proxy-only egress, karena loopback address yang diserahkan ke proxy akan menjadi proxy host sendiri, jadi berikan services tersebut address yang dapat dicapai proxy. Untuk alasan yang sama gateway menolak `localhost`-style names sepenuhnya saat proxy-only egress aktif.

Ketika salah satu kondisi itu tidak terpenuhi, gateway mencatat warning saat boot menamai variable yang menghentikannya dan menjaga default behavior.

Setelah proxy-only egress aktif, izinkan setiap destination di proxy, termasuk internal collector dan host apa pun yang dikonfigurasi oleh IP address. Anda masih dapat menjaga internal IdP langsung dengan [`oidc.use_proxy: false`](#idp-requests-through-a-forward-proxy).

<Warning>
  Aktifkan ini hanya ketika allowlist proxy setidaknya setat ketat dengan check gateway sendiri. Proxy harus menolak cloud metadata endpoints seperti `169.254.169.254` dan `metadata.google.internal`, link-local addresses, dan loopback proxy host sendiri, dan harus menolaknya oleh address yang name resolves ke, bukan hanya oleh name, karena gateway tidak lagi menangkap hostname yang resolves ke salah satu dari mereka. Proxy yang terhubung ke mana pun diminta menghapus [SSRF guard](/docs/id/claude-apps-gateway-deploy#threat-model-summary) gateway untuk requests ini.
</Warning>

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

| Field                     | Diperlukan | Deskripsi                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| ------------------------- | ---------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `postgres_url`            | Ya         | `postgres://` atau `postgresql://` URL. Diperlukan: device-grant rendezvous, di mana browser callback menulis dan polling CLI membaca, memerlukan cross-replica state. Gateway menjalankan schema migrations-nya sendiri saat boot dan pada upgrade, jadi role memerlukan rights untuk membuat dan mengubah tables pada target schema. Lihat [Upgrades](/docs/id/claude-apps-gateway-deploy#upgrades) dan [Postgres](/docs/id/claude-apps-gateway-deploy#postgres). |
| `username`                | Tidak      | Overrides user di `postgres_url`                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `password`                | Tidak      | Database credential. Atur di sini daripada di `postgres_url` jadi credential tetap keluar dari URL. Menerima karakter apa pun dan mengambil precedence atas URL credentials.                                                                                                                                                                                                                                                                              |
| `max_connections`         | Tidak      | Postgres connection-pool size per replica. Default `5`, yang konservatif dan ramah ke shared databases. Dengan [spend limits](#admin) diaktifkan, hot path melakukan beberapa operasi per inference request, jadi naikkan untuk dedicated database di bawah load, dan simpan replicas × ini di bawah database's `max_connections`.                                                                                                                        |
| `connect_timeout_seconds` | Tidak      | Detik gateway menunggu ketika membuka koneksi Postgres. Seluruh number dari `1` hingga `60`, default `5`. Naikkan jika upaya koneksi timeout ketika instance gateway baru dimulai. Memerlukan Claude Code v2.1.274 atau lebih baru di server gateway. Versi sebelumnya menolak untuk memulai ketika key diatur.                                                                                                                                           |

Untuk local development, arahkan `postgres_url` ke throwaway Postgres container, misalnya `docker run --rm -p 5432:5432 -e POSTGRES_HOST_AUTH_METHOD=trust postgres`.

<h3 id="upstreams">
  `upstreams`
</h3>

`upstreams` adalah ordered list. Gateway meneruskan inference ke upstream pertama yang menyelesaikan model yang diminta.

Pada `5xx`, `429`, `401`, `403`, `404`, atau timeout gateway failover ke next upstream; `4xx` lainnya tidak, karena error tersebut dapat diatribusikan ke request daripada upstream. `401` atau `403` berarti credential gateway sendiri gagal terhadap upstream itu. `404` berarti upstream itu tidak melayani model yang diminta, jadi upstream yang lebih baru dalam list masih bisa.

Jika Anda menetapkan `forward_user_identity: true` pada upstream, `429` yang dikembalikan ke request yang membawa email developer tidak failover. Lihat [bagaimana per-user limit denial mencapai developer](#per-user-identity-headers-for-a-proxy-you-run).

Failover pada `404` memerlukan gateway v2.1.198 atau lebih baru. Release sebelumnya mengembalikan `404` pertama ke client bahkan ketika upstream yang lebih baru dalam list melayani model.

Multiple upstreams dari provider yang sama harus menetapkan distinct `name:`.

Amazon Bedrock, Claude Platform on AWS, Google Cloud's Agent Platform, dan Microsoft Foundry clients dibangun sekali saat startup, dan SDK mereka refresh credentials secara internal, jadi rotating cloud credentials tidak memerlukan restart. Static Anthropic API keys dan bearers dibaca saat startup; lihat [Anthropic API](#anthropic-api).

<h4 id="upstream-error-messages">
  Upstream error messages
</h4>

Gateway mengembalikan satu response upstream, atau `502` miliknya sendiri, tergantung bagaimana upstreams menjawab:

* **Upstream mengembalikan status yang gateway tidak [fail over](#multiple-upstreams) pada**: response upstream itu. Gateway tidak mencoba upstreams lebih lanjut.
* **Setiap upstream yang gateway coba gagal dengan cara yang [fails over](#multiple-upstreams) pada**: `429` terakhir. Ketika tidak ada yang mengembalikan `429`, gateway lebih suka, secara berurutan, `401` atau `403` terakhir, `404` terakhir, dan `501` terakhir. Ketika tidak ada yang mengembalikan salah satu dari itu, `502` gateway sendiri, `all upstreams failed (N attempted)`, di mana N menghitung setiap entry dalam [`upstreams`](#upstreams), termasuk entries yang gateway lewati karena mereka tidak melayani model yang diminta.

Ketika gateway mengembalikan response upstream, ia menjaga status code upstream. Apakah ia menjaga message upstream tergantung pada provider. Body error upstream Anthropic API mencapai developer tidak berubah.

Amazon Bedrock, Claude Platform on AWS, Google Cloud's Agent Platform, dan Microsoft Foundry upstreams dapat menamai account IDs, role ARNs, dan project IDs Anda dalam text error mereka. Gateway mencatat text lengkap itu dalam [operational log](/docs/id/claude-apps-gateway-deploy#logs). Apa yang developer lihat dari upstreams itu tergantung pada rejection:

* `400` atau `413` dalam Anthropic's standard error envelope: message upstream sendiri, seperti `prompt is too long`. Claude Platform on AWS, Agent Platform, dan Microsoft Foundry mengembalikan envelope ini untuk model API rejections.
* `400` atau `413` dalam provider's own shape: token `capability_rejected:`. Ketika gateway tidak dapat mengklasifikasi rejection, `upstream rejected the request` pada `400` atau `request too large for this upstream` pada `413`.
* Status apa pun: generic per-status copy, seperti `upstream rate limit exceeded` pada `429`.

Misalnya, gateway menggantikan Amazon Bedrock's `Input is too long for requested model.` dengan `capability_rejected: prompt_too_long`. Claude Code [compacts automatically](/docs/id/errors#prompt-is-too-long) pada token itu, seperti yang dilakukan pada `prompt is too long`.

Menjaga cloud upstream's `400` atau `413` message, atau menggantinya dengan token `capability_rejected:`, memerlukan gateway v2.1.233 atau lebih baru.

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

<a id="per-user-identity-headers-for-a-proxy-you-run" />

<h5 id="per-user-identity-headers-for-a-proxy-you-run">
  Per-user identity headers for a proxy you run
</h5>

Anda dapat menunjukkan `provider: anthropic` upstream's `base_url` ke proxy yang Anda jalankan daripada ke Anthropic API. Untuk memberitahu proxy mana developer yang mengirim setiap request, atur `forward_user_identity: true` pada upstream itu. Proxy kemudian dapat mengatribusikan spend per developer. Memerlukan gateway yang menjalankan Claude Code v2.1.233 atau lebih baru.

Misalnya, untuk proxy di `upstream-gateway.internal.example.com`:

```yaml theme={null}
upstreams:
  - provider: anthropic
    base_url: https://upstream-gateway.internal.example.com
    auth:
      api_key: ${PROXY_KEY}
    forward_user_identity: true        # default false
```

Gateway menambahkan headers ini ke setiap request yang diteruskan ke upstream itu.

| Header                        | Value                                            |
| ----------------------------- | ------------------------------------------------ |
| `x-litellm-end-user-id`       | Email developer, ketika IdP menyediakannya.      |
| `x-claude-gateway-user-id`    | IdP subject developer, dari token's `sub` claim. |
| `x-claude-gateway-user-email` | Email developer, ketika IdP menyediakannya.      |

Ketika IdP token tidak membawa email, gateway mengirim hanya `x-claude-gateway-user-id` dan menghilangkan dua email headers. Jika IdP Anda menempatkan email di claim yang berbeda, atur [`oidc.email_claim`](#oidc) ke claim itu.

Ketika proxy Anda menjawab `429` ke request yang membawa email developer, gateway mengembalikan response itu ke developer apa adanya daripada failover ke next upstream, jadi per-user budget atau rate limit proxy Anda berlaku. Response lainnya dari proxy mengikuti [failover rules](#upstreams) biasa. Jika token IdP developer tidak membawa email, gateway meneruskan requests mereka tanpa email headers, jadi `429` ke salah satu requests itu menghitung sebagai upstream capacity dan failover. Sebelum v2.1.267 di server gateway, setiap `429` failover.

Atur `forward_user_identity` hanya pada upstream yang `base_url` adalah proxy yang Anda operasikan. Gateway mengirim developer emails ke server apa pun yang `base_url` namai. Jika `base_url` adalah Anthropic API, yang merupakan default, gateway menolak untuk memulai.

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

| Setup           | Bagaimana                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| --------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| IAM permissions | Berikan gateway's principal `bedrock:InvokeModel` dan `bedrock:InvokeModelWithResponseStream` pada inference-profile ARNs dan underlying foundation-model ARNs. Untuk built-in catalog di US regions: `arn:aws:bedrock:<region>:<account>:inference-profile/us.anthropic.*` dan `arn:aws:bedrock:*::foundation-model/anthropic.*`. Juga berikan `bedrock:CountTokens` pada foundation-model ARNs. Gateway menggunakannya, tanpa biaya, untuk menghitung input tokens dari request yang client abaikan, jadi [spend limits](#admin) tetap akurat. Tanpa itu gateway fallback ke one-token Bedrock request untuk count itu. |
| Model access    | Amazon Bedrock mengaktifkan model access secara default di commercial regions. Gate tingkat account yang tersisa adalah Anthropic's one-time use case form: jika tidak ada seorang pun di AWS account Anda yang telah mengirimkannya, buka Amazon Bedrock console, pilih model Anthropic dari Model catalog, dan lengkapi form. Lihat [Submit use case details](/docs/id/amazon-bedrock#1-submit-use-case-details) untuk AWS Organizations form dan permissions yang submitter butuhkan.                                                                                                                                       |
| EKS (IRSA)      | Buat IAM role dengan policy di atas dan trust policy untuk cluster's OIDC provider yang scoped ke gateway's service account. Annotate service account dengan `eks.amazonaws.com/role-arn: arn:aws:iam::<acct>:role/claude-gateway`. `auth: {}` mengambilnya.                                                                                                                                                                                                                                                                                                                                                              |
| ECS / EC2       | Attach IAM role ke task definition atau instance profile. `auth: {}` mengambilnya.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| Tempat lain     | Lewatkan credentials via `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY`, dan `AWS_SESSION_TOKEN` env vars, atau atur secara eksplisit di `auth:` dengan `${VAR}` expansion                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| Region          | `region:` adalah API endpoint region. Cross-region inference profiles route across geo (US, EU, APAC) terlepas dari mana Anda memilih. Untuk non-US regions atau provisioned-throughput ARNs, tambahkan [`models:`](#models) block dengan per-upstream IDs yang benar.                                                                                                                                                                                                                                                                                                                                                    |

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

Untuk client-side Microsoft Foundry deployment, lihat [Claude Code on Microsoft Foundry](/docs/id/microsoft-foundry). Gateway-side upstream:

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

| Setup                   | Bagaimana                                                                                                                                                                           |
| ----------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| RBAC                    | Berikan gateway's identity `Azure AI User` atau `Cognitive Services User` pada Foundry resource                                                                                     |
| Deployments             | Microsoft Foundry menggunakan admin-chosen deployment names, bukan canonical model IDs. Tambahkan [`models:`](#models) block memetakan setiap canonical ID ke deployment name Anda. |
| AKS (workload identity) | Federate User-Assigned Managed Identity dengan cluster's OIDC issuer dan bind ke gateway's service account. `use_azure_ad: true` mengambilnya via `WorkloadIdentityCredential`.     |
| ACI / App Service       | Enable system-assigned atau user-assigned managed identity pada resource. `use_azure_ad: true` mengambilnya.                                                                        |
| Tempat lain             | `auth: { api_key: "${FOUNDRY_API_KEY}" }`. Quote `${…}` di dalam `{ }`.                                                                                                             |

<h4 id="static-headers-on-upstream-requests">
  Static headers on upstream requests
</h4>

Untuk menambahkan fixed headers ke requests yang gateway kirim ke satu upstream, atur `headers:` pada upstream itu. Gunakan ketika proxy yang Anda jalankan di depan provider routes atau attributes traffic oleh header.

`headers:` memerlukan Claude Code v2.1.277 atau lebih baru di server gateway. Gateway sebelumnya menolak untuk memulai ketika menemukan key. Upgrade setiap replica sebelum Anda menambahkan key, dan hapus key sebelum Anda rollback ke versi sebelumnya.

Headers pergi ke server yang `base_url` namai, atau ke endpoint provider sendiri ketika `base_url` unset. Provider menerimanya juga kecuali proxy Anda menghapusnya.

Contoh ini mencapai upstream `provider: vertex` melalui proxy di `upstream-proxy.internal.example.com`. Ini menetapkan header `x-source` yang proxy baca, dan mengirim token dari environment variable `PROXY_TOKEN` sebagai `x-proxy-token`:

```yaml theme={null}
upstreams:
  - provider: vertex
    region: us-east5
    project_id: example-prod
    base_url: https://upstream-proxy.internal.example.com
    auth: {}
    headers:
      x-source: claude-apps-gateway
      x-proxy-token: ${PROXY_TOKEN}
```

Values adalah printable ASCII text tanpa space di kedua ujung. Quote number, `true`, atau `false` jadi YAML membacanya sebagai text.

Untuk menjaga secret keluar dari file config, gunakan [secret expansion](#secret-expansion) untuk memuat value dari environment variable dengan `${VAR}` atau dari file dengan `${file:/path}`. `${VAR}` yang resolves ke empty value menghentikan gateway dari memulai.

`headers:` bekerja pada setiap provider, dan setiap upstream mengirim hanya miliknya sendiri.

Tidak setiap request yang gateway kirim ke upstream membawanya:

| Request yang gateway kirim ke upstream ini                            | Carries `headers:`                |
| --------------------------------------------------------------------- | --------------------------------- |
| `/v1/messages`, streaming atau tidak, dan `/v1/messages/count_tokens` | Ya                                |
| Request yang failover dari upstream lain                              | Ya, hanya `headers:` upstream ini |
| Amazon Bedrock's `CountTokens` call untuk request yang client abaikan | Tidak                             |
| Workload Identity Federation token exchange                           | Tidak                             |

Pada Amazon Bedrock atau Claude Platform on AWS upstream yang menandatangani requests dengan AWS SigV4, headers ini adalah bagian dari signature, jadi proxy Anda harus meneruskannya tanpa perubahan.

Jika Anda menggunakan name yang gateway reserve, ia menolak untuk memulai, dan startup error menamai header. Reserved names termasuk:

* `authorization` dan `x-api-key`
* `host`, `content-type`, dan `user-agent`
* Nama apa pun yang dimulai dengan `anthropic-`, `x-goog-`, `x-amz-`, atau `x-amzn-`

<h4 id="multiple-upstreams">
  Multiple upstreams
</h4>

Provider yang sama dapat muncul lebih dari sekali dengan distinct `name:`. Ini mencakup different regions, different accounts via different credential chains, provisioned throughput versus on-demand, dan cross-provider fallback.

Gateway mencoba upstreams secara berurutan. `5xx`, `429`, `401`, `403`, `404`, timeouts, dan missing-endpoint (`501`) failover; `4xx` lainnya tidak.

`429` adalah per-upstream capacity, jadi provisioned-throughput (PT) exhaustion failover ke on-demand. Jika Anda menetapkan [`forward_user_identity: true`](#per-user-identity-headers-for-a-proxy-you-run) pada upstream, `429` ke request yang membawa email developer adalah per-user denial daripada dan tidak failover.

Setiap request dimulai pada upstream pertama. Request mencapai upstream yang lebih baru hanya ketika setiap upstream di depannya telah gagal atau tidak melayani model yang diminta.

Gateway menjaga tidak ada record dari failed upstreams, jadi saat upstream down, setiap request yang mencapainya masih mencobanya dan menunggu untuk gagal sebelum pindah.

Untuk Anthropic API upstream, [`timeouts.upstream_ttfb_ms`](#http-tuning) bounds wait pada down upstream. Setting ini tidak berlaku pada provider lainnya, di mana gateway menunggu hingga satu jam untuk upstream mulai merespons.

`404` adalah per-upstream model availability, jadi upstream yang belum enable model tidak memblokir upstream yang lebih baru dalam list yang melayaninya. Upstream yang tidak dapat menyelesaikan model yang diminta dilewati tanpa network round-trip.

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

# Per-upstream model IDs are keyed on the upstream's `name:`.
models:
  - id: claude-opus-4-8
    label: Claude Opus 4.8
    upstream_model:
      bedrock-pt: arn:aws:bedrock:us-east-1:111111111111:provisioned-model/abcdef
      bedrock-od: us.anthropic.claude-opus-4-8
      bedrock-acct2: us.anthropic.claude-opus-4-8
      anthropic-fallback: claude-opus-4-8
```

| Lever                  | Bagaimana                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| ---------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Different regions      | Satu Bedrock upstream per region, masing-masing dengan `region:` sendiri. Dengan [`auto_include_builtin_models: true`](#models) cross-region inference profiles route secara otomatis; untuk region-pinned deployments gunakan `models:` block.                                                                                                                                                                                                                                             |
| Different accounts     | Satu Bedrock upstream per account, masing-masing dengan credentials sendiri di `auth:`. Default chain (`auth: {}`) menggunakan pod's identity; untuk second account, atur explicit credentials atau bearer token.                                                                                                                                                                                                                                                                           |
| Provisioned throughput | Map model ke provisioned-throughput ARN di `models:` untuk upstream's name itu. Upstreams lainnya simpan on-demand ID, jadi PT capacity exhausted sebelum failover.                                                                                                                                                                                                                                                                                                                         |
| VPC / FIPS endpoints   | Atur `base_url:` pada upstream ke VPC endpoint atau FIPS endpoint URL Anda                                                                                                                                                                                                                                                                                                                                                                                                                  |
| Model-scoped routing   | Hanya custom model `id`, satu yang bukan built-in Claude model, melewati upstreams yang tidak ada dalam `upstream_model:` map-nya. Gateway mencoba built-in models pada setiap upstream secara berurutan dan menggunakan provider's default ID di mana map tidak memiliki entry, jadi untuk built-in models map mengubah ID mana yang upstream terima daripada apakah itu dicoba; upstream yang menolak ID mengikuti [failover rules](#upstreams) yang sama seperti upstream error apa pun. |

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

| Field                     | Diperlukan | Deskripsi                                                                                                                                                                                                                                                                                                                                                                                |
| ------------------------- | ---------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `write_keys`              | Tidak      | Array dari `{id, key}`. `x-api-key` yang cocok dengan salah satu ini dapat list, set, dan delete spend limits. Nilai key harus minimal 32 karakter; `id`s harus unik di seluruh `read_keys` dan `write_keys`.                                                                                                                                                                            |
| `read_keys`               | Tidak      | Array dari `{id, key}`. Read-only: setiap endpoint `GET`, termasuk listing caps, mengambil satu berdasarkan ID, dan membaca [`/effective`](/docs/id/claude-apps-gateway-spend-limits#%2Feffective) dan [`/audit`](/docs/id/claude-apps-gateway-spend-limits#%2Faudit).                                                                                                                             |
| `admin_groups`            | Tidak      | Nama grup IdP. JWT gateway yang klaim `groups`-nya mencakup salah satu ini memiliki akses admin penuh, read dan write, dan audit sebagai `oidc:<sub>`. Gunakan ini untuk admin manusia; gunakan API keys untuk mesin. Entri kosong dalam daftar ini menghentikan gateway saat boot. Lihat [Matcher values that stop the gateway at boot](#matcher-values-that-stop-the-gateway-at-boot). |
| `blocked_message`         | Tidak      | Ditambahkan verbatim ke `429 billing_error` yang dilihat developer yang diblokir. Tulis seluruh instruksi, seperti URL atau saluran Slack. Jika tidak diatur, gateway mengirimkan hanya pesan default. Lihat [How enforcement works](/docs/id/claude-apps-gateway-spend-limits#how-enforcement-works).                                                                                        |
| `audit_retention_days`    | Tidak      | Default `365`. Baris `admin_audit` yang lebih lama disapu.                                                                                                                                                                                                                                                                                                                               |
| `spend_retention_months`  | Tidak      | Default `13`. Baris counter `spend` yang lebih tua dari ini disapu. Default menyimpan tahun penuh ditambah bulan parsial saat ini untuk pelaporan year-over-year.                                                                                                                                                                                                                        |
| `identity_retention_days` | Tidak      | Default `90`. Last-seen TTL untuk baris `principal_emails`, yang menyimpan email, nama tampilan, dan grup setiap developer (PII). Sengaja lebih pendek dari spend retention sehingga identitas yang dihapus provisioning-nya berusia keluar sementara counter spend anonimnya tetap ada.                                                                                                 |
| `group_limit_mode`        | Tidak      | `min` (default) atau `max`. Ketika developer ada di beberapa grup dengan caps, `min` memberlakukan yang paling ketat dan `max` yang paling longgar. Digunakan oleh enforcement dan `/effective`.                                                                                                                                                                                         |

<h3 id="enforcement">
  `enforcement`
</h3>

Blok `enforcement` mengontrol bagaimana pemeriksaan spend-limit berperilaku ketika store tidak tersedia.

| Field                  | Diperlukan | Deskripsi                                                                                                                                                                                                                                                                                                                                                                                                                 |
| ---------------------- | ---------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `fail_closed_on_error` | Tidak      | Default `false`. Enforcement spend gagal terbuka pada pemadaman Postgres, jadi inference tetap aktif. Atur `true` untuk gagal tertutup: developer yang over-cap diblokir, tetapi begitu juga semua orang jika store tidak dapat dijangkau. Memerlukan blok [`admin:`](#admin): enforcement spend hanya berjalan ketika `admin` dikonfigurasi, dan gateway menolak untuk memulai jika Anda mengatur ini `true` tanpa satu. |

<h3 id="pricing">
  `pricing`
</h3>

Blok `pricing` memberi tahu spend meter apa yang harus dikenakan alih-alih harga list USD, jadi caps dan [`/effective`](/docs/id/claude-apps-gateway-spend-limits#%2Feffective) mencerminkan tarif kontrak Anda. Jumlah tetap dalam USD dan tetap merupakan estimasi, bukan invoice. Dua prasyarat:

* Claude Code v2.1.227 atau lebih baru di server gateway. Versi sebelumnya menolak kunci yang tidak dikenal saat boot.
* Blok [`admin:`](#admin) atau, dalam v2.1.268 atau lebih baru, blok [`managed:`](#managed) dengan setidaknya satu kebijakan. Gateway menolak untuk memulai dengan `pricing` diatur dan tidak ada blok, karena tidak ada yang akan membacanya.

```yaml theme={null}
pricing:
  multiplier: 0.85
  overrides:
    - upstream: bedrock-eu
      model: claude-sonnet-4-6
      input: 3.30
      output: 16.50
      cache_read: 0.33
      cache_write: 4.125
```

| Field        | Diperlukan | Deskripsi                                                                                                                                                                                                                                   |
| ------------ | ---------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `multiplier` | Tidak      | Default `1`. Meter mengalikan setiap jumlah yang diukur dengan ini, baik harga list atau override, jadi `0.85` menagih 85% dari harga. Harus lebih besar dari 0 dan paling banyak 10, dan nilai di atas 1 adalah [markup](#mark-prices-up). |
| `overrides`  | Tidak      | Baris dari `{upstream, model, input, output, cache_read, cache_write}` dalam USD per juta token. Keempat tarif diperlukan. Masing-masing harus lebih besar dari 0 dan paling banyak 10000.                                                  |

Bagaimana meter mencocokkan baris override:

* Baris menggantikan harga list untuk permintaan yang `upstream`, [`upstreams[].name`](#upstreams), layani untuk `model`. Itu termasuk tarif [fast mode](/docs/id/fast-mode#understand-the-cost-tradeoff) yang lebih tinggi, jadi permintaan fast dan standard meter pada tarif empat yang sama.
* ID bawaan seperti `claude-sonnet-4-6`, dicocokkan seperti [`models[].id`](#models), mencakup setiap bentuk bertanggal, bentuk Amazon Bedrock regional, atau bentuk Google Cloud's Agent Platform yang meter hargai sebagai model itu. String lain apa pun, seperti alias atau ARN profil inference, cocok dengan ID yang klien kirim atau string yang dikirim upstream, case-insensitively.
* Di mana baris tumpang tindih, meter memilih baris yang paling spesifik daripada baris pertama: baris yang `model`-nya adalah string model yang tepat dikirim upstream, kemudian baris yang cocok dengan ID yang tepat yang klien kirim, kemudian baris yang menamai model bawaan.
* Nama upstream yang tidak dikenal gagal boot, dan begitu juga dua baris untuk satu upstream yang menamai model yang sama, termasuk dua ejaan dari satu model bawaan. Gateway memperingatkan saat boot tentang baris yang tidak ada model yang dapat diminta yang dapat menggunakannya.
* Permintaan web-search tetap pada harga list \$0.01; multiplier masih berlaku untuk mereka.

Untuk tarif per-region, berikan setiap region upstream bernama sendiri dan satu baris per upstream.

<h4 id="mark-prices-up">
  Tandai harga naik
</h4>

Dengan v2.1.271 atau lebih baru di server gateway, Anda dapat mengatur `multiplier` di atas 1, hingga 10, untuk meter lebih dari yang penyedia kenakan, misalnya tarif chargeback internal. Contoh ini meter setiap permintaan pada 120% dari harga:

```yaml theme={null}
pricing:
  multiplier: 1.2
```

Dengan blok [`admin:`](#admin), markup juga berlaku untuk spend limits. Meter menghitung 120% dari harga, jadi developer mencapai caps mereka lebih cepat. Gateway mencatat peringatan saat boot yang mengatakan demikian.

Multiplier tidak mengubah apa yang penyedia upstream kenakan untuk permintaan.

Jika gateway juga [mengirimkan tarif ke klien yang masuk](#send-the-rates-to-signed-in-clients), developer memerlukan Claude Code v2.1.271 atau lebih baru untuk melihat markup. Klien sebelumnya mengabaikan `multiplier` di atas 1 dan menampilkan biaya tanpanya.

Server gateway sebelumnya dari v2.1.271 menolak untuk memulai jika Anda mengatur `multiplier` di atas 1.

<h4 id="send-the-rates-to-signed-in-clients">
  Kirim tarif ke klien yang masuk
</h4>

Dengan v2.1.268 atau lebih baru di server gateway, gateway juga menempatkan tarif dari `pricing` ke dalam kebijakan [`managed`](#managed) yang disajikannya, sebagai pengaturan terkelola [`modelPricing`](/docs/id/settings-reference#modelpricing). Developer yang cocok dengan kebijakan kemudian melihat tarif `pricing` untuk upstream pertama yang melayani setiap ID model dalam `/usage`, baris status, dan OpenTelemetry. Developer yang tidak cocok dengan kebijakan apa pun menerima tidak ada pengaturan terkelola, jadi angka mereka tetap pada harga list. Klien menerapkan pengaturan dalam Claude Code v2.1.242 atau lebih baru.

* Apa yang ditambahkan gateway: kecuali blok `cli` kebijakan sudah mengatur `modelPricing`, gateway menambahkan `multiplier` dan, untuk setiap ID model yang dapat diminta klien, baris override dari upstream pertama yang melayani ID itu. Tarif yang hanya upstream failover yang mengenakan tetap di gateway.
* Opt satu kebijakan keluar: atur `modelPricing` ke `{}` dalam blok `cli` kebijakan itu, dan developer-nya tetap pada harga list.
* Pertahankan tarif kebijakan sendiri: kebijakan yang blok `cli`-nya mengatur `modelPricing` dengan `multiplier` atau `overrides` sendiri menyimpan `modelPricing` itu utuh, dan gateway menambahkan tidak ada tarif sendiri ke dalamnya.

<h3 id="models">
  `models`
</h3>

Blok `models` adalah daftar model yang dikurasi admin opsional, disajikan di `/v1/models` dan digunakan untuk menerjemahkan ID model per upstream. Ini diperlukan untuk wilayah Amazon Bedrock non-US, ARN throughput provisioned Amazon Bedrock, dan nama deployment Microsoft Foundry.

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

Setiap kunci di bawah `upstream_model` harus cocok dengan `name` dari upstream yang dikonfigurasi, yang default ke nama penyedia. Kunci yang tidak cocok dengan upstream apa pun gagal boot, jadi hilangkan baris untuk penyedia yang tidak Anda gunakan.

<h3 id="managed">
  `managed`
</h3>

Blok `managed` mendefinisikan kebijakan akses berbasis peran yang dikunci pada grup IdP atau domain email. Kebijakan dievaluasi secara berurutan; kecocokan pertama dipilih, kemudian digabungkan ke basis catch-all `match: {}`. Mereka disajikan per-user di `GET /managed/settings` dengan caching ETag/304.

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

Gateway memvalidasi nilai `model` itu sendiri sebelum meneruskan permintaan, jadi nilai yang salah bentuk tidak pernah mencapai upstream. Ini menolak permintaan dengan `400` dalam dua kasus:

* Ketika nilai hilang atau kosong, gateway menolak permintaan dengan pesan `model is required`. Pemeriksaan itu memerlukan gateway yang menjalankan Claude Code v2.1.228 atau lebih baru.
* Ketika nilai ada tetapi bukan string, gateway menolak permintaan dengan pesan `model must be a string`. Memerlukan gateway yang menjalankan Claude Code v2.1.221 atau lebih baru.

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

  * **Konten kebijakan**: mengedit kebijakan dan redeploy mencapai klien yang terhubung pada polling managed-settings berikutnya mereka, dalam satu jam, terlepas dari [perubahan yang berlaku hanya pada peluncuran berikutnya](/docs/id/server-managed-settings#fetch-and-caching-behavior)
  * **Keanggotaan grup**: mengubah keanggotaan grup pengguna mengubah kebijakan mana yang cocok dengan mereka. Ini berlaku pada re-mint sesi berikutnya, berarti refresh senyap berikutnya, dibatasi oleh `session.ttl_hours`.
</Note>

<h4 id="matcher-values-that-stop-the-gateway-at-boot">
  Matcher values that stop the gateway at boot
</h4>

Saat boot, gateway memeriksa blok `match` dari setiap kebijakan dan daftar [`admin_groups`](#admin). Salah satu dari nilai ini menghentikan gateway dengan error yang menamai field:

* Daftar `groups` kosong
* Entri kosong dalam `groups` atau dalam `admin_groups`
* `email_domain` kosong
* `email_domain` yang berisi `@`, whitespace, atau koma. Gateway memotong nilai dan menghilangkan satu `@` terkemuka sebelum pemeriksaan ini. Tulis satu domain telanjang, seperti `example.com`.

Sebelum v2.1.232, gateway dimulai dengan nilai-nilai ini. Setiap nilai memiliki efek ini:

* `email_domain` kosong: gateway melewati pemeriksaan domain, jadi kebijakan dengan `email_domain` kosong dan tidak ada daftar `groups` cocok dengan setiap pengguna yang terautentikasi
* Daftar `groups` kosong: kebijakan tidak cocok dengan siapa pun
* `email_domain` berisi `@`, whitespace, atau koma: kebijakan tidak cocok dengan siapa pun
* Entri kosong dalam `groups` atau dalam `admin_groups`: entri cocok dengan pengguna hanya ketika klaim IdP `groups` pengguna itu juga berisi entri kosong. Dalam `admin_groups`, kecocokan itu memberikan akses admin. Jika daftar `admin_groups` Anda tidak pernah berisi entri kosong, tidak ada yang mendapatkan akses admin dengan cara ini.

<h4 id="what-goes-in-cli">
  What goes in `cli`
</h4>

Setiap nilai `cli` adalah dokumen `managed-settings.json` Claude Code yang lengkap, skema yang sama yang akan Anda deploy melalui MDM atau `/etc/claude-code/managed-settings.json`, diekspresikan di sini sebagai YAML. CLI menerapkan dokumen yang dikirimkan pada tingkat terkelola, di atas pengaturan pengguna dan proyek, sebagai pengganti pengaturan yang dikelola server. Oleh karena itu, ini mengabaikan pengaturan [terbatas pada sumber kebijakan tingkat OS](/docs/id/server-managed-settings#current-limitations), seperti `policyHelper` dan `wslInheritsWindowsSettings`.

Gateway memvalidasi setiap dokumen terhadap skema pengaturan CLI saat boot, jadi kunci tingkat atas yang tidak dikenali gagal boot dengan error yang menamai setiap kunci yang bermasalah. Bagian skema yang sengaja terbuka masih menerima nilai arbitrer, karena klien yang lebih baru mungkin mengenali entri yang skema gateway tidak. Kunci terbuka ini adalah `env`, `pluginConfigs`, dan kunci yang bersarang di bawah `permissions`.

Karena validasi menggunakan skema yang disertakan dengan versi gateway yang terinstal, menempatkan kunci pengaturan tingkat atas yang diperkenalkan oleh rilis Claude Code yang lebih baru ke konfigurasi terkelola memerlukan upgrade gateway terlebih dahulu. Smoke-test kebijakan baru pada satu klien sebelum meluncurkannya.

Referensi kunci lengkap ada di [Claude Code settings](/docs/id/settings-reference#all-settings). Kunci yang paling sering dicari operator:

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

| Key                                        | Ditegakkan oleh | Efek                                                                                                                                                                                                                                                                                                                                                                           |
| ------------------------------------------ | --------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `availableModels`                          | Gateway + CLI   | Allowlist model. Juga diperiksa di `/v1/messages`, jadi klien yang dipatch tidak dapat membypassnya.                                                                                                                                                                                                                                                                           |
| `permissions.allow` / `.deny`              | CLI             | Aturan tool dan command. Lihat [Permissions](/docs/id/permissions).                                                                                                                                                                                                                                                                                                                 |
| `permissions.disableBypassPermissionsMode` | CLI             | Atur ke `disable` untuk memblokir [`bypassPermissions`](/docs/id/permission-modes#skip-all-checks-with-bypasspermissions-mode), mode yang melewati prompt permission, dan flag `--dangerously-skip-permissions`                                                                                                                                                                     |
| `allowManagedPermissionRulesOnly`          | CLI             | Ketika `true`, pengaturan terkelola menjadi satu-satunya sumber pengaturan aturan permission. Entri [`allowManagedPermissionRulesOnly`](/docs/id/settings-reference#allowmanagedpermissionrulesonly) mencantumkan setiap sumber yang Claude Code kemudian abaikan.                                                                                                                  |
| `env`                                      | CLI             | Variabel lingkungan yang digabungkan ke proses CLI. Gunakan untuk telemetry, auto-update, dan model-name overrides.                                                                                                                                                                                                                                                            |
| `hooks`                                    | CLI             | Org-wide [hooks](/docs/id/hooks)                                                                                                                                                                                                                                                                                                                                                    |
| `managedMcpServers`                        | CLI             | Remote MCP servers [disediakan ke setiap developer yang cocok](/docs/id/managed-mcp#provide-servers-through-managed-settings) bersama server yang mereka tambahkan sendiri, `http` dan `sse` saja. Lihat [MCP servers in a policy](#mcp-servers-in-a-policy). Memerlukan Claude Code v2.1.259 atau lebih baru di server gateway dan pada klien. Klien sebelumnya mengabaikan kunci. |

Karena pengaturan ini tiba melalui jaringan, CLI menunjukkan setiap developer dialog persetujuan keamanan sebelum menerapkan pengaturan yang tercantum di bawah:

* `hooks`
* Variabel `env` yang memerlukan persetujuan developer, seperti proxy dan base-URL variables
* pengaturan eksekusi shell seperti `apiKeyHelper` dan `statusLine`
* pengaturan binary sandbox `sandbox.bwrapPath`, `sandbox.socatPath`, dan `sandbox.ripgrep`
* Pengaturan Sandbox yang mengintersepsi traffic, menyuntikkan kredensial, atau melemahkan isolasi, seperti `sandbox.network.tlsTerminate` dan pengaturan port proxy. [Security approval dialogs](/docs/id/server-managed-settings#security-approval-dialogs) mencantumkan semuanya.

[Approval memory](/docs/id/server-managed-settings#approval-memory) mencakup berapa lama persetujuan berlangsung dan kapan dialog muncul lagi.

Claude Code menerapkan beberapa variabel `env` yang dikirimkan tanpa menunjukkan developer dialog persetujuan, seperti pengaturan pemilihan model dan batas numerik. Variabel yang dikirimkan lainnya dapat memerlukan persetujuan developer sebelum berlaku; nilai proxy, base-URL, atau `OTEL_EXPORTER_OTLP_ENDPOINT` yang tidak kosong selalu melakukannya. Ketika variabel yang dikirimkan memerlukan persetujuan, dialog menamakannya.

[Environment variables and the approval dialog](/docs/id/server-managed-settings#environment-variables-and-the-approval-dialog) memiliki detail, termasuk empat privacy toggles yang nilai yang dikirimkan menentukan apakah mereka memerlukan persetujuan. Sebelum v2.1.218, Claude Code menerapkan lebih sedikit variabel tanpa bertanya kepada developer, jadi lebih banyak variabel yang dikirimkan memicu dialog.

Konfigurasi [telemetry](#telemetry) gateway mendorong `OTEL_EXPORTER_OTLP_ENDPOINT`, jadi pengaturan `telemetry.forward_to` memicu dialog pada setiap klien interaktif. Dialog melindungi mesin developer dari gateway yang dikompromikan atau bermusuhan, bukan organisasi dari developer.

Run non-interaktif dengan flag `-p` tidak dapat menampilkan dialog. Ini menerapkan pengaturan yang didorong untuk run itu saja dan tidak merekamnya sebagai disetujui, jadi sesi interaktif berikutnya developer masih menampilkan dialog. Sebelum v2.1.207, run non-interaktif menyimpan pengaturan sebagai disetujui dan tidak ada sesi interaktif yang lebih baru menampilkan dialog untuk mereka.

Jika developer menolak, Claude Code keluar dari sesi itu daripada menerapkan kebijakan. Ketika Anda mendorong hook baru, atau variabel env apa pun yang memicu dialog, ke kebijakan yang luas, Claude Code oleh karena itu menampilkan dialog kepada setiap developer yang cocok. Ini menampilkan dialog dalam sesi yang berjalan pada polling per jam berikutnya, dan sebaliknya pada startup developer berikutnya.

Kunci `cli` dinamai `settings` dalam rilis sebelumnya. Ejaan itu masih diterima sebagai alias, tetapi deployment baru harus menggunakan `cli`.

<h4 id="mcp-servers-in-a-policy">
  MCP servers in a policy
</h4>

Untuk menyediakan MCP servers ke klien Claude Code yang cocok dengan kebijakan, atur [`managedMcpServers`](/docs/id/managed-mcp#provide-servers-through-managed-settings) dalam blok `cli` kebijakan itu. Anda memerlukan Claude Code v2.1.259 atau lebih baru di server gateway dan pada klien.

Gateway memeriksa setiap entri saat boot dengan [aturan yang sama yang Claude Code terapkan pada klien](/docs/id/managed-mcp#what-an-entry-can-contain), dan jika entri gagal pemeriksaan, gateway menolak untuk memulai dan menamai entri.

Jika Anda menulis referensi `${VAR}` dalam `gateway.yaml`, gateway menyelesaikannya dari lingkungannya saat boot melalui [secret expansion](#secret-expansion) sebelum menjalankan pemeriksaan entri, jadi setiap klien yang cocok menerima nilai literal dan dapat membacanya. [Header guidance untuk server yang disediakan](/docs/id/managed-mcp#provide-servers-through-managed-settings) berlaku untuk nilai yang diperluas.

Gateway menolak ejaan `.mcp.json` `mcpServers` dalam blok `cli`, dan error boot-nya menamai `managedMcpServers` sebagai kunci yang digunakan. Sebelum v2.1.259, gateway menolak definisi MCP server apa pun dalam blok `cli`.

<h4 id="claude-desktop-overlay">
  Claude Desktop overlay
</h4>

Jika organisasi Anda juga menerapkan [Claude Desktop](/docs/id/desktop), gateway yang sama melayani kedua klien. Arahkan `bootstrapUrl`, dalam [managed configuration](https://claude.com/docs/third-party/claude-desktop/configuration) Claude Desktop, ke `<listen.public_url>/user/bootstrap`. Claude Desktop menurunkan issuer OAuth dari URL itu, menjalankan sign-in device-code yang sama terhadap gateway ini, dan mengambil konfigurasinya dari respons.

<Note>
  Memerlukan Claude Code v2.1.203 atau lebih baru di server gateway, dan opt-in eksplisit: `/user/bootstrap` mengembalikan 404 kecuali kebijakan yang cocok dengan pengguna membawa kunci `desktop`. `desktop: {}` kosong opt-in kebijakan, dan kunci `desktop` pada lapisan dasar `match: {}` opt-in setiap kebijakan yang mewarisnya. Log audit merekam setiap permintaan sebagai `desktop_bootstrap.serve` atau `desktop_bootstrap.denied`.
</Note>

Gateway menurunkan banyak respons dari blok `cli` kebijakan yang cocok dan dari konfigurasi gateway tingkat atas:

* Daftar model, dari `availableModels`
* Tool yang dinonaktifkan, dari entri `permissions.deny` nama tool telanjang. Jika Anda mengatur `disabledBuiltinTools` dalam blok `desktop` kebijakan, gateway melayani union dari nilai Anda dan daftar yang diturunkan, jadi Anda dapat menonaktifkan lebih banyak tool dengan cara ini tetapi tidak dapat mengaktifkan kembali yang Anda nonaktifkan melalui `permissions.deny`
* Allowlist egress, dari `sandbox.network.allowedDomains`. Jika Anda mengatur `coworkEgressAllowedHosts` dalam blok `desktop` kebijakan, gateway menggunakan nilai itu alih-alih daftar yang diturunkan
* Endpoint OTLP yang menunjuk ke gateway itu sendiri, dan atribut identitas pengguna yang masuk. Gateway meneruskan export yang diterima di endpoint itu ke tujuan `forward_to` Anda. Ini menyertakan endpoint dan atribut ketika Anda mengatur [`telemetry.forward_to`](#telemetry) dan `listen.public_url`.

  Claude Desktop mengekspor setiap signal dengan satu encoding: `http/protobuf`, atau `http/json` ketika Anda mengatur `OTEL_EXPORTER_OTLP_PROTOCOL` atau salah satu varian per-signal-nya ke `http/json` dalam `env` kebijakan. Sebelum Claude Code v2.1.261 di server gateway, respons mengatur `http/json` terlepas, jadi kolektor yang hanya menerima protobuf menolak export Claude Desktop

Untuk mengatur `disabledBuiltinTools`, `coworkEgressAllowedHosts`, atau pengaturan `managedMcpServers` Claude Desktop sendiri dalam blok `desktop` kebijakan, Anda memerlukan Claude Code v2.1.232 atau lebih baru di server gateway. `managedMcpServers` Claude Desktop mengambil nilai array daripada objek.

Gateway menghilangkan kunci tanpa padanan Claude Desktop, seperti `hooks` dan aturan permission yang dibatasi seperti `Bash(npm *)`, dari respons bootstrap.

Tambahkan blok `desktop` opsional bersama `cli` untuk mengatur pengaturan Claude Desktop secara langsung. Tulis pengaturan dari [managed configuration reference](https://claude.com/docs/third-party/claude-desktop/configuration) Claude Desktop sebagai nama kunci datar. Tinggalkan kunci yang Claude Desktop baca hanya dari MDM atau file lokal, seperti `bootstrapUrl`; gateway menolaknya saat boot. Sebelum v2.1.232, gateway menerima daftar tetap dari 11 kunci feature-gate, seperti `chatTabEnabled` dan `disableAutoUpdates`, dan menolak setiap kunci lainnya saat boot. Sebelum v2.1.227, gateway juga menolak `chatTabEnabled` dan `chatAdvancedFileAnalysisEnabled` saat boot.

```yaml theme={null}
managed:
  policies:
    - match: { groups: [eng-contractors] }
      cli:
        availableModels: [claude-sonnet-4-6]
      desktop:
        isLocalDevMcpEnabled: false
        disableAutoUpdates: true
        banner: { text: "Contractor build: internal use only" }
```

Setiap kunci opsional; Claude Desktop menerapkan default-nya sendiri untuk kunci apa pun yang Anda hilangkan. Gateway memvalidasi setiap blok `desktop` saat boot terhadap skema konfigurasi yang Claude Desktop itu sendiri gunakan, jadi kesalahan muncul saat startup gateway sebagai error yang menamai kunci daripada mencapai setiap desktop yang terhubung. Gateway gagal saat boot ketika blok berisi:

* Kunci yang tidak dikenal
* Kunci yang dikenali yang nilai-nya Claude Desktop akan menolak atau diam-diam jatuhkan, seperti nilai kosong atau sub-kunci yang salah eja di dalam entri bersarang. Sebelum v2.1.260, gateway diam-diam menjatuhkan field yang salah eja di dalam objek bersarang dari entri `managedMcpServers` atau `orgPluginSettings` alih-alih gagal saat boot.
* Kunci yang gateway hitung sendiri: koneksi inference, daftar model, dan relay OTLP. Konfigurasikan ini melalui [`upstreams`](#upstreams), [`models`](#models), dan bagian [`telemetry`](#telemetry) `forward_to`.
* Alias legacy dari kunci saat ini. Dalam error boot, gateway menamai kunci kanonis untuk ditulis.

Jika Anda menggunakan nilai atau bentuk entri yang sudah usang, seperti entri `managedMcpServers` tanpa `transport`, gateway dimulai dan mencatat peringatan yang menamai pengganti.

Gateway memvalidasi blok `desktop` terhadap skema yang disertakan dengan versi yang terinstal, seperti yang dilakukannya pada blok `cli`. Untuk mengirimkan pengaturan yang diperkenalkan oleh rilis Claude Desktop yang lebih baru, upgrade gateway terlebih dahulu. Misalnya, `userPluginMarketplacesEnabled` dan `userPluginUploadsEnabled` memerlukan Claude Code v2.1.260 atau lebih baru di server gateway dan Claude Desktop 1.37937.0 atau lebih baru pada mesin anggota.

Jika Anda mengatur `orgPluginSettings` dalam blok `desktop` kebijakan, gateway melayaninya dalam bentuk array yang Claude Desktop 1.15200.0 dan lebih baru baca. Desktop yang lebih lama mengabaikan array dan tidak memberlakukan kebijakan tool plugin, jadi perbarui anggota ke 1.15200.0 atau lebih baru sebelum Anda mengandalkannya.

Gateway mengisi kunci yang blok `desktop` kebijakan tidak atur dari blok `desktop` catch-all `match: {}`, dengan cara yang sama mengisi blok `cli` kebijakan dari dasar. Jika Anda mengatur `disabledBuiltinTools` atau `builtinToolPolicy` dalam dasar dan kebijakan peran, gateway menyimpan pembatasan dasar:

* `disabledBuiltinTools`: gateway menggunakan union dari daftar dasar dan daftar kebijakan
* `builtinToolPolicy`: jika Anda mengatur tool ke nilai selain `allow` dalam dasar, gateway menyimpan nilai itu bahkan jika Anda mengatur `allow` untuk tool yang sama dalam kebijakan peran

Untuk setiap kunci lainnya, jika Anda mengaturnya dalam kebijakan peran, gateway menggunakan nilai kebijakan peran. Gateway mengganti array atau objek bersarang seperti `banner` secara keseluruhan, jadi jika Anda mengatur `banner.text` dalam kebijakan peran, gateway menjatuhkan `banner.backgroundColor` dasar.

Jika Anda tidak menerapkan Claude Desktop, tinggalkan `desktop` sepenuhnya dari kebijakan Anda; gateway kemudian mengembalikan 404 dari `/user/bootstrap` untuk setiap pengguna.

<h4 id="precedence-with-other-managed-sources">
  Precedence dengan sumber terkelola lainnya
</h4>

Jika perangkat juga memiliki kebijakan yang dikirimkan MDM atau `managed-settings.json` lokal, pengaturan yang dikirimkan gateway mendapat peringkat pertama. [Precedence within the managed tier](/docs/id/managed-settings#precedence-within-the-managed-tier) di halaman pengaturan terkelola mengatakan kapan sumber lokal berlaku, dan memiliki [kunci yang Claude Code baca dari setiap sumber admin](/docs/id/managed-settings#keys-read-from-every-admin-source) terlepas dari sumber mana yang dipilihnya, seperti kunci sandbox lock, `forceRemoteSettingsRefresh`, dan per-variabel `env` merge. [`policyHelper`](/docs/id/settings-reference#policyhelper) yang dikonfigurasi dalam profil MDM atau file pengaturan terkelola berjalan hanya ketika gateway tidak mengirimkan pengaturan; entri mengatakan apa yang outputnya gantikan.

Host embedding seperti [Claude Desktop](/docs/id/desktop) dapat menyediakan kebijakan melalui opsi SDK `managedSettings`. [Parent settings from embedding hosts](/docs/id/managed-settings#parent-settings-from-embedding-hosts) mengatakan kapan Claude Code menerapkannya, dan [Restrict parent settings](/docs/id/claude-apps-gateway#restrict-parent-settings) mencantumkan pengaturan arah allow mana yang masih berlaku tanpa kunci `allowManaged*Only`.

Kebijakan gateway berlaku untuk setiap invokasi Claude Code pada mesin, termasuk run non-interaktif `claude -p` dan sesi yang dihasilkan oleh Agent SDK. Jika gateway tidak dapat dijangkau saat startup, sesi yang masuk keluar dengan error daripada menjalankan tanpa kebijakan mereka.

<h3 id="telemetry">
  `telemetry`
</h3>

CLI mengirim metrik, log, dan, ketika diaktifkan, trace ke gateway, yang meneruskan mereka verbatim ke setiap tujuan yang dikonfigurasi. Export menggunakan OpenTelemetry Protocol (OTLP) melalui HTTP. Untuk melewati relay dan memiliki sesi mengekspor langsung ke kolektor Anda, [namai kolektor dalam kebijakan](#export-directly-to-your-collector). Lihat [Monitoring usage](/docs/id/monitoring-usage) untuk metrik dan event yang CLI emit.

CLI memberi stempel setiap export dengan identitas pengguna yang terautentikasi, dibaca dari JWT yang diterbitkan gateway: atribut `user.id`, `user.email`, dan `user.groups`. Atribusi biaya dan penggunaan per-developer oleh karena itu bekerja tanpa konfigurasi sisi developer.

[Claude Desktop](#claude-desktop-overlay) dan sesi Cowork yang masuk melalui gateway memberi stempel telemetry mereka dengan `user.email` dan `user.groups` bersama `enduser.id`, jadi Anda dapat mencakup penggunaan terminal, Desktop, dan Cowork dengan satu query pada `user.email` atau `user.groups`. `user.groups` adalah daftar grup IdP yang dipisahkan koma.

Desktop dan Cowork telemetry juga membawa `enduser.sub`, klaim `sub` yang penyedia identitas Anda terbitkan untuk pengguna, yang tetap sama ketika email pengguna berubah. Sesi terminal memberi stempel nilai yang sama di bawah `user.id`, jadi query yang cocok `enduser.sub` terhadap terminal `user.id` mencakup penggunaan terminal, Desktop, dan Cowork satu pengguna bersama-sama. Pada export Desktop dan Cowork, `user.id` adalah identifier anonim, bukan subject.

Seperti semua data OpenTelemetry dari Claude Code, atribut ini hanya pergi ke tujuan yang organisasi Anda konfigurasikan, tidak pernah ke Anthropic.

Jika daftar grup pengguna lebih panjang dari 255 karakter setelah percent-encoded, atau nama grup berisi koma atau tanda sama dengan, gateway meninggalkan `user.groups` dari telemetry Desktop dan Cowork pengguna itu daripada memotongnya. Sesi terminal pengguna itu masih membawa daftar lengkap.

Gateway meninggalkan `enduser.sub` ketika subject lebih panjang dari 255 karakter setelah percent-encoded, atau berisi spasi, karakter di luar printable ASCII, atau salah satu dari `,` `;` `=` `\` `"` `%`. Telemetry Desktop dan Cowork pengguna itu menyimpan atribut lainnya.

Anda memerlukan Claude Code v2.1.265 atau lebih baru di server gateway untuk `user.email` dan `user.groups` pada telemetry Desktop dan Cowork, dan Claude Desktop 1.24012 atau lebih baru pada mesin setiap developer untuk `user.groups`.

Anda memerlukan Claude Code v2.1.274 atau lebih baru di server gateway untuk `enduser.sub`.

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

Setiap URL `forward_to` harus menggunakan `https://`, dengan satu pengecualian untuk kolektor di interface loopback gateway itu sendiri:

* `http://localhost:<port>` melewati validasi konfigurasi, tetapi [SSRF guard](/docs/id/claude-apps-gateway-deploy#threat-model-summary) memblokir setiap export dengan `ECONNREFUSED_SSRF` kecuali Anda mengatur `CLAUDE_GATEWAY_ALLOW_LOOPBACK=1` dalam lingkungan gateway
* `http://127.0.0.1:<port>` atau `http://[::1]:<port>` gagal boot kecuali variabel itu diatur

Untuk kolektor in-cluster, paparkan melalui HTTPS di alamat internal-nya sendiri, atau jalankan sebagai sidecar dengan variabel diatur.

Ketika `HTTPS_PROXY` diatur, gateway mengirimkan export melalui proxy itu.

Untuk menjangkau kolektor internal secara langsung, tambahkan ke `NO_PROXY` berdasarkan hostname atau berdasarkan domain dengan titik terkemuka seperti `.internal.example.com`, yang memerlukan Claude Code v2.1.277 atau lebih baru di server gateway. Pastikan gateway dapat menjangkau kolektor tanpa proxy. Entri tanpa titik terkemuka cocok hanya dengan nama yang tepat itu, bukan nama di bawahnya. Rentang CIDR tidak cocok.

Dengan [proxy-only egress](#proxy-only-egress) diaktifkan, izinkan kolektor dalam proxy alih-alih, karena entri `NO_PROXY` apa pun menjaga proxy-only egress tetap mati.

Telemetry off dalam CLI secara default. Ketika Anda mengatur `telemetry.forward_to` dan `listen.public_url`, gateway mengaktifkannya untuk klien yang terhubung dengan mendorong enam variabel lingkungan melalui `/managed/settings`:

* `CLAUDE_CODE_ENABLE_TELEMETRY=1`
* `OTEL_METRICS_EXPORTER`, `OTEL_LOGS_EXPORTER`, dan `OTEL_TRACES_EXPORTER`, masing-masing diatur ke `otlp` jika setidaknya satu tujuan `forward_to` mengaktifkan signal itu dan ke `none` sebaliknya
* `OTEL_EXPORTER_OTLP_ENDPOINT=<public_url>`
* `OTEL_EXPORTER_OTLP_PROTOCOL=http/protobuf`

Sebelum Claude Code v2.1.265 di server gateway, gateway mendorong ketiga selector exporter sebagai `otlp`, termasuk untuk signal yang tidak ada tujuan yang opt into.

Endpoint yang didorong dibangun dari URL publik, jadi metrik dan log tidak memerlukan konfigurasi OTEL dari developer atau kebijakan.

Developer yang masuk melalui `/login` tidak dapat mengalihkan export dengan konfigurasi OTEL mereka sendiri:

* **Variabel yang diatur secara lokal**: Claude Code menerapkan variabel yang didorong pada tingkat terkelola, jadi masing-masing menimpa nilai yang developer atur untuk itu secara lokal.
* **Endpoint yang dikonfigurasi secara lokal**: dengan export OTLP/HTTP diaktifkan, CLI mengabaikan endpoint yang dikonfigurasi secara lokal apa pun, terlepas dari apakah gateway mendorong variabel telemetry. Export-nya pergi ke gateway kecuali kebijakan [menamai kolektor Anda sebagai endpoint](#export-directly-to-your-collector).

Tanpa tujuan `forward_to` untuk signal, gateway menerima dan membuangnya. Jika developer sudah mengekspor telemetry Claude Code ke salah satu kolektor Anda, tambahkan sebagai tujuan `forward_to`, dengan logs atau traces diaktifkan jika mereka mengekspor itu, jadi terus menerima data mereka setelah mereka masuk. Untuk melewati relay alih-alih, [namai kolektor dalam kebijakan](#export-directly-to-your-collector).

[Traces](/docs/id/monitoring-usage#traces-beta) juga memerlukan `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA=1` pada setiap klien. Atur dalam blok `env` kebijakan terkelola, karena gateway tidak mendorongnya. Developer menyetujuinya dalam dialog [security approval](#managed) yang sama yang endpoint yang didorong sudah trigger.

Atur ke `1` hanya dalam kebijakan yang grup-nya Anda ingin trace. Kebijakan yang tidak mengaturnya mewarisi nilai dari kebijakan catch-all `match: {}` Anda jika kebijakan itu mengatur satu, per [merge rules](#managed). Untuk menjaga klien grup dari mengirim trace bahkan ketika developer mengatur variabel secara lokal, atur ke `0` dalam kebijakan grup itu.

Kedua encoding OTLP protobuf dan JSON direlai, dan backend apa pun yang kompatibel dengan OpenTelemetry bekerja sebagai tujuan.

<h4 id="export-directly-to-your-collector">
  Export directly to your collector
</h4>

Untuk memiliki sesi yang masuk melalui `/login` mengirim telemetry langsung ke kolektor Anda alih-alih melalui relay, atur `OTEL_EXPORTER_OTLP_ENDPOINT` ke URL dasar `https://` kolektor dalam blok `env` [kebijakan terkelola](#managed). Claude Code menambahkan `/v1/metrics`, `/v1/logs`, atau `/v1/traces` ke URL yang Anda atur, seperti `https://otel-collector.example.com:4318`, dan mengekspor setiap signal di sana melalui OTLP/HTTP. Memerlukan Claude Code v2.1.265 atau lebih baru pada mesin setiap developer. Klien sebelumnya mengekspor melalui relay.

Untuk mengautentikasi ke kolektor, atur `OTEL_EXPORTER_OTLP_HEADERS` dalam blok `env` yang sama. Sesi tidak pernah mengirim token sesi gateway developer ke kolektor yang dinamai dengan cara ini.

Ketika Anda menambah atau mengubah endpoint ini dalam kebijakan, Claude Code meminta setiap developer untuk menyetujuinya dalam [security approval dialog](#managed) sebelum menerapkannya dalam sesi interaktif.

Claude Code memeriksa endpoint sebelum mengekspor signal langsung, dan menyimpan signal itu di relay ketika pemeriksaan gagal. Pemeriksaan termasuk:

* Endpoint berasal dari gateway itu sendiri. Jika Anda mengatur variabel yang sama dalam profil MDM atau `managed-settings.json` lokal, export tetap di relay.
* URL menggunakan `https://`, atau `http://` ke alamat loopback
* URL menyelesaikan ke path yang berakhir dalam `/v1/<signal>`, tanpa query atau fragment. Claude Code membangun path itu sendiri dari variabel generik. Ini menggunakan variabel per-signal seperti `OTEL_EXPORTER_OTLP_METRICS_ENDPOINT` seperti yang ditulis, jadi sertakan path lengkap di sana.
* URL bukan host gateway itu sendiri. Endpoint yang ditujukan ke gateway menyimpan path relay dan token sesi-nya.
* Baik Anda maupun developer tidak mengonfigurasi [`otelHeadersHelper`](/docs/id/settings-reference#otelheadershelper) dalam sumber pengaturan apa pun. Dengan helper yang dikonfigurasi, setiap signal tetap di relay.

Endpoint yang Anda namai mengubah hanya di mana export pergi. Anda masih memilih signal mana yang mengekspor sama sekali dengan selector `OTEL_*_EXPORTER`.

Endpoint saja tidak mengaktifkan export, jadi juga atur variabel yang melakukannya, kecuali gateway sudah mendorongnya:

* Jika gateway sudah [mendorong variabel telemetry](#telemetry), mereka mencakup enablement, selector, dan protokol, dan endpoint eksplisit Anda menimpa nilai `<public_url>` yang didorong. Atur selector `OTEL_*_EXPORTER` ke `otlp` sendiri hanya untuk signal yang tidak ada tujuan `forward_to` yang mengaktifkan.
* Jika tidak, juga atur `CLAUDE_CODE_ENABLE_TELEMETRY=1`, selector `OTEL_*_EXPORTER`, dan `OTEL_EXPORTER_OTLP_PROTOCOL=http/protobuf`.

Ketika developer masuk keluar, atau masuk ke gateway yang berbeda, export ke kolektor berhenti dan Claude Code menjatuhkan setiap batch yang tersisa daripada mengirimkannya.

<h4 id="when-a-destination-fails">
  When a destination fails
</h4>

Gateway tidak buffer, retry, atau menyimpan telemetry, jadi itu menjatuhkan export yang tidak mencapai tujuan daripada mengirimkannya terlambat. Setiap tujuan berhasil atau gagal sendiri, dan klien yang mengekspor menerima respons sukses baik cara, jadi pengiriman yang gagal muncul hanya dalam log gateway.

Setelah lima pengiriman berturut-turut yang gagal ke tujuan, gateway menjeda penerusan ke dalamnya dalam peregangan 30 detik, mencatat setiap jeda, sampai pengiriman berhasil. Respons error apa pun, timeout, atau error koneksi dihitung sebagai pengiriman yang gagal, kecuali `400`, `413`, `415`, `422`, dan `431`, yang berarti kolektor menolak payload export itu sebagai salah bentuk atau terlalu besar.

Payload yang ditolak tidak memajukan atau mereset failure count: gateway terus meneruskan ke tujuan dan mencatat peringatan yang menamakannya dan status, pada penolakan pertama tujuan dan setiap seratus setelah.

<h3 id="http-tuning">
  HTTP tuning
</h3>

Empat blok tingkat atas opsional, `access_control`, `limits`, `timeouts`, dan `rate_limits`, menyetel permukaan HTTP. Default cocok untuk sebagian besar deployment.

| Block            | Key                                            | Default  | Deskripsi                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| ---------------- | ---------------------------------------------- | -------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `access_control` | `allow_cidrs` / `deny_cidrs`                   | empty    | Inbound IP allow/deny berdasarkan alamat klien, setelah resolusi `trusted_proxies`. `deny_cidrs` diperiksa terlebih dahulu; klien yang cocok ditolak bahkan jika `allow_cidrs` juga cocok. Jika `allow_cidrs` non-empty gateway adalah default-deny. `/healthz` dan `/readyz` dikecualikan dari `allow_cidrs`. Ketika proxy terpercaya mengirim entri `X-Forwarded-For` yang bukan alamat IP, klien nyata tidak diketahui dan gateway mencatat peringatan sekali yang menamai apa yang harus diperiksa. Di mana daftar apa pun berlaku untuk permintaan, itu menolaknya dengan `403` dan alasan audit `xff_unparseable`. Di mana tidak ada yang berlaku, itu melayani permintaan dan menggunakan alamat proxy sendiri sebagai IP klien untuk rate limits per-IP dan audit. |
| `limits`         | `max_request_bytes`                            | 32 MiB   | Max inbound request body; permintaan yang terlalu besar mendapat `413` sebelum body dibuffer. Naikkan untuk permintaan file atau gambar besar.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `limits`         | `max_request_header_bytes`                     | unset    | Ketika diatur, header yang terlalu besar mengembalikan `431`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `limits`         | `max_url_length`                               | unset    | Ketika diatur, URL yang terlalu panjang mengembalikan `414`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `timeouts`       | `upstream_ttfb_ms`                             | 120000   | Max wait untuk header respons upstream (time to first byte). Response body kemudian stream tanpa wall-clock cap. Berlaku ke jalur upstream Anthropic langsung; pada setiap penyedia lainnya, gateway menunggu hingga satu jam untuk respons mulai.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `rate_limits`    | `device_authorization.max` / `.window_seconds` | 30 / 600 | Per-IP rate limit pada endpoint device-authorization yang tidak terautentikasi. Naikkan untuk org besar di belakang IP egress bersama atau NAT. [Large rollouts](/docs/id/claude-apps-gateway-deploy#large-rollouts) menunjukkan cara mengukurnya. Limit ini berlaku hanya ke alur sign-in device-grant, bukan ke inference `/v1/messages`. Lihat [User-code brute-force resistance](/docs/id/claude-apps-gateway-deploy#user-code-brute-force-resistance).                                                                                                                                                                                                                                                                                                                          |
| `rate_limits`    | `device_verify.max` / `.window_seconds`        | 10 / 600 | Per-IP rate limit pada pengajuan `user_code` di `/device`. Ini adalah apa yang menghentikan seseorang dari menebak kode developer lain. [Large rollouts](/docs/id/claude-apps-gateway-deploy#large-rollouts) menunjukkan seberapa jauh untuk menaikkannya.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |

Jika Anda meninggalkan kedua daftar `access_control` kosong, yang merupakan default, gateway melayani alamat klien apa pun, jadi hanya jaringan Anda yang membatasi siapa yang dapat menjangkaunya. Itu penting karena gateway dapat mendorong [pengaturan terkelola](#managed) yang menjalankan perintah pada mesin developer.

Sementara `allow_cidrs` kosong, gateway memperingatkan di dua tempat, tanpa mengubah cara itu menjawab permintaan apa pun:

* **Saat boot**: peringatan dalam log operasional merekomendasikan hanya mengizinkan rentang pribadi `10.0.0.0/8`, `172.16.0.0/12`, `192.168.0.0/16`, `100.64.0.0/10`, `127.0.0.0/8`, `::1/128`, dan `fc00::/7`, ditambah rentang internal lainnya yang developer Anda terhubung dari. Jika Anda mengikat gateway ke alamat loopback dan tidak mengatur `trusted_proxies` atau `public_url`, seperti dalam pengembangan lokal, peringatan tidak muncul.
* **Saat runtime**: pertama kali permintaan tiba dari alamat di luar rentang pribadi itu, gateway mencatat peringatan dan memancarkan event audit [`access.public_client`](/docs/id/claude-apps-gateway-deploy#logs) yang membawa IP klien. Keduanya tembak sekali per proses. Alamat link-local, `169.254.0.0/16` dan `fe80::/10`, tidak dihitung sebagai publik. Gateway menjawab `/healthz` dan `/readyz` sebelum pemeriksaan ini berjalan, jadi probe kesehatan dari rentang publik tidak memicunya.

Kedua sinyal menggunakan alamat klien seperti gateway menyelesaikannya. Jika load balancer, port-forward, atau tunnel meneruskan traffic dan tidak terdaftar dalam `listen.trusted_proxies`, gateway melihat alamat relay, yang biasanya pribadi, jadi baik peringatan runtime maupun daftar allow pribadi menangkapnya.

Di belakang front end seperti itu, atur [`listen.trusted_proxies`](#listen) terlebih dahulu sehingga gateway melihat alamat klien nyata, dan jaga gateway dan segalanya di depannya tidak dapat dijangkau dari internet publik terlepas.

<h3 id="load_test_mode">
  `load_test_mode`
</h3>

Blok `load_test_mode` memungkinkan Anda load test gateway tanpa memanggil penyedia model. Saat diaktifkan, gateway membangun dan menandatangani setiap permintaan penyedia seperti biasa, membuangnya alih-alih mengirimkannya, dan streaming respons kaleng kembali melalui jalur respons normalnya. Respons adalah teks pengisi yang dimulai dengan kalimat mengatakan itu kaleng.

Memerlukan v2.1.283 atau lebih baru. Versi sebelumnya menolak untuk memulai ketika kunci diatur, jadi upgrade setiap replika sebelum Anda menambahkan blok dan hapus sebelum Anda rollback.

Contoh di bawah mengaktifkan mode dengan default, respons 750 token output yang di-stream selama sekitar 10 detik:

```yaml theme={null}
load_test_mode:
  enabled: true
  reply_tokens: 750     # roughly how many tokens of text each canned reply carries
  reply_seconds: 9.5    # how long a streamed reply takes
```

| Field           | Diperlukan | Deskripsi                                                                                                                                                                                          |
| --------------- | ---------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `enabled`       | Ya         | `true` mengaktifkan mode. `false` menyimpan angka Anda dalam file dengan mode off. Gateway menolak untuk memulai jika blok ada tanpa itu.                                                          |
| `reply_tokens`  | Tidak      | Default `750`. Kira-kira berapa banyak token teks yang setiap respons kaleng bawa, angka bulat dari 1 hingga 100000.                                                                               |
| `reply_seconds` | Tidak      | Default `9.5`. Berapa lama respons yang di-stream memakan waktu, dari 0 hingga 600. `0` mengirimkan seluruh respons sekaligus. Respons terhadap permintaan non-streaming selalu kembali sekaligus. |

Load test dalam mode ini mencakup gateway, Postgres Anda, dan segalanya di depan gateway. Ini tidak mencakup batas, kecepatan, atau jalur jaringan penyedia.

Saat mode diaktifkan, permintaan dapat membawa header `x-load-test-user` yang menyimpan angka bulat hingga tujuh digit, dan gateway menghitung setiap angka sebagai developer terpisah dengan email dan grup developer yang token-nya datang dengan permintaan. Berikan deployment load-test database kosong sendiri, karena gateway menolak untuk memulai dengan mode diaktifkan terhadap database di mana developer apa pun sudah menghabiskan apa pun.

<Warning>
  Jangan pernah mengaktifkan ini untuk gateway yang developer gunakan. Setiap permintaan mendapat respons kaleng dan tidak ada model yang dipanggil. Gateway mencatat peringatan `load_test_mode is on` saat boot dan menandai setiap event audit [`inference`](/docs/id/claude-apps-gateway-deploy#logs) dengan `load_test: true` saat mode diaktifkan.
</Warning>

<h2 id="complete-example">
  Contoh lengkap
</h2>

Config reference penuh ini menggunakan setiap core section; [HTTP tuning blocks](#http-tuning) menyimpan defaults mereka. Salin, hapus apa yang Anda tidak butuhkan, dan isi values Anda. Config dalam [Quickstart](/docs/id/claude-apps-gateway#quickstart) adalah minimal version dari ini.

```yaml gateway.yaml theme={null}
# Run with:
#   claude gateway --config gateway.yaml
#
# Operational log verbosity is controlled by the CLAUDE_GATEWAY_LOG_LEVEL
# environment variable (debug | info | warn | error; default info). debug
# also logs the claim names in each id_token, for groups_claim diagnosis.
# It does not affect audit events, which are always emitted.

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
  # connect_timeout_seconds: 5

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

# Load test this deployment without calling a model provider. Never on a
# gateway that developers use: every request gets a canned reply.
# load_test_mode:
#   enabled: true
#   # reply_tokens: 750
#   # reply_seconds: 9.5

# Meter at contracted rates instead of USD list price. Requires admin: or a
# managed: policy. With managed:, the same rates also go to signed-in clients.
# Rates below are placeholders, not real contract prices.
# pricing:
#   multiplier: 0.85
#   overrides:
#     - { upstream: anthropic, model: claude-sonnet-4-6, input: 3.30, output: 16.50, cache_read: 0.33, cache_write: 4.125 }

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

Semua di atas mengonfigurasi gateway server. Anda menunjukkan developer machines ke gateway secara terpisah, pada setiap device, melalui Claude Code's [managed settings](/docs/id/managed-settings). Gateway tidak dapat mendorong login keys itu sendiri, karena mereka adalah apa yang memberitahu client di mana gateway berada.

Untuk CLI, atur keys ini dalam per-OS `managed-settings.json`. Dua login keys merutekan setiap developer's `/login` ke gateway Anda:

```json theme={null}
{
  "forceLoginMethod": "gateway",
  "forceLoginGatewayUrl": "https://claude-gateway.internal.example.com",
  "parentSettingsBehavior": "merge"
}
```

`parentSettingsBehavior: "merge"` menjaga pengiriman Claude Desktop dari egress allowlist ke embedded Claude Code sessions-nya tetap berfungsi; [Deliver policy to Claude Desktop sessions](/docs/id/claude-apps-gateway#deliver-policy-to-claude-desktop-sessions) menjelaskan mekanisme dan di mana opt-in harus berada.

Deploy file `managed-settings.json` ke setiap device, biasanya melalui platform MDM Anda. File path berbeda menurut platform. Lihat [di mana setiap mekanisme menyimpan policy](/docs/id/managed-settings#where-each-mechanism-stores-the-policy).

Secara default, registry policy di Windows atau managed-preferences plist di macOS menggantikan file `managed-settings.json` daripada menggabungkannya, terlepas dari [exception keys dan cross-source checks di atas](#precedence-with-other-managed-sources). Ketiga keys dalam snippet ini mengikuti highest-priority-source rule, jadi fleets yang mengirimkan policy melalui Group Policy atau configuration profiles harus menempatkan ketiganya dalam mekanisme itu sebagai gantinya.

Untuk Claude Desktop, atur key `bootstrapUrl` dalam [managed configuration](https://claude.com/docs/third-party/claude-desktop/configuration) Claude Desktop sendiri ke `<listen.public_url>/user/bootstrap`. Sign-in flow dan per-group policy kemudian cocok dengan CLI's setelah policy opt-in server-side dengan key `desktop`; tanpa opt-in, `/user/bootstrap` mengembalikan 404. Lihat [Claude Desktop overlay](#claude-desktop-overlay) untuk bagian server-side.

Claude Code menghormati [`forceLoginGatewayUrl`](/docs/id/settings-reference#forcelogingatewayurl), [`gatewayInternalNetworks`](/docs/id/settings-reference#gatewayinternalnetworks), dan nilai `"gateway"` dari [`forceLoginMethod`](/docs/id/settings-reference#forceloginmethod) hanya dari managed source di mesin: `managed-settings.json`, plist macOS atau Windows HKLM registry, atau policy helper. Developer menetapkannya dalam `~/.claude/settings.json` mereka sendiri tidak memiliki efek, dan begitu juga menetapkannya dalam gateway payload.

<h2 id="related">
  Terkait
</h2>

* [Claude apps gateway overview](/docs/id/claude-apps-gateway): quickstart dan developer connection
* [Deployment guide](/docs/id/claude-apps-gateway-deploy): IdP setup, container image, Kubernetes dan Cloud Run, dan operations
* [Spend limits](/docs/id/claude-apps-gateway-spend-limits): per-developer caps dan Admin API
