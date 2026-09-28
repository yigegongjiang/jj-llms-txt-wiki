> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Verifikasi identitas sesi di lingkungan yang di-host sendiri

> Verifikasi JWT CLAUDE_CODE_SESSION_ACCESS_TOKEN sehingga layanan di jaringan Anda dapat mempercayai permintaan dari sesi di lingkungan yang di-host sendiri Anda.

<Note>
  Lingkungan yang di-host sendiri berada dalam beta publik pada paket Team dan Enterprise; [Pemilik](/docs/id/cloud-environments#organization-shared-environments) mengaktifkannya dengan mengaktifkan **Allow self-hosted environments** di [halaman admin **Cloud environments**](https://claude.ai/admin-settings/cloud-environments). Halaman ini mencakup verifikasi identitas sesi; lihat [quickstart](/docs/id/self-hosted-environments-quickstart) untuk setup dan [Deploy to production](/docs/id/self-hosted-environments-deploy) untuk resep fleet.
</Note>

[Lingkungan yang di-host sendiri](/docs/id/self-hosted-environments) memungkinkan sesi [Claude Code di web](/docs/id/claude-code-on-the-web) berjalan pada infrastruktur yang Anda operasikan alih-alih di Anthropic. Karena sesi berjalan di dalam jaringan Anda, Claude dapat memanggil layanan internal Anda secara langsung. Layanan-layanan tersebut memerlukan cara untuk mengkonfirmasi bahwa permintaan berasal dari sesi Claude Code di lingkungan Anda, dan untuk mengidentifikasi identitas pengguna atau layanan yang membuat sesi tersebut.

Setiap sesi di lingkungan yang di-host sendiri menerima JSON Web Token (JWT) yang ditandatangani dalam variabel lingkungan `CLAUDE_CODE_SESSION_ACCESS_TOKEN`. Sesi menyajikan token seperti kredensial bearer apa pun; misalnya, skrip yang Claude jalankan dapat memanggil layanan Anda dengan `curl -H "Authorization: Bearer $CLAUDE_CODE_SESSION_ACCESS_TOKEN"`. Anthropic menandatangani token dan menerbitkan kunci verifikasi di endpoint JWKS publik. Layanan Anda mengambil kunci-kunci tersebut, memverifikasi tanda tangan, dan membaca klaim untuk memutuskan akses apa yang akan diberikan.

<h2 id="the-session-token">
  Token sesi
</h2>

Sebelum Anda menulis kode verifikasi, ketahui apa yang dibuktikan token dan bentuk yang akan dilihat oleh perpustakaan JWT Anda.

<h3 id="what-the-token-proves">
  Apa yang dibuktikan token
</h3>

Token yang valid membuktikan beberapa fakta dan sengaja tidak membuktikan yang lain:

* **Membuktikan**: Anthropic mengeluarkan token untuk sesi tertentu di lingkungan tertentu, dan bagaimana sesi dibuat: oleh pengguna di organisasi Anda, atau oleh identitas layanan organisasi Anda, yang merupakan cara [sesi saluran Claude Tag](https://claude.com/docs/claude-tag/concepts/agent-identity) dimulai
* **Tidak membuktikan**: proses mana di host runner yang menyajikannya. Token berada di variabel lingkungan di dalam sesi, jadi kode apa pun yang Claude jalankan, dan alat atau server MCP apa pun yang dimulai sesi, dapat membaca dan menyajikannya.

Dua konsekuensi untuk layanan Anda:

* Verifikasi klaim `aud` terhadap ID lingkungan Anda, nilai `ccpool_...` yang ditampilkan dengan lingkungan Anda di [halaman admin **Cloud environments**](https://claude.ai/admin-settings/cloud-environments), untuk menolak token yang dikeluarkan untuk lingkungan organisasi lain.
* Batasi kredensial yang Anda turunkan dari token ke apa yang dapat dilakukan sesi coding tunggal, bukan ke semua yang dapat dilakukan pembuat sesi. Lihat [Scope derived credentials](#scope-derived-credentials).

<h3 id="token-format">
  Format token
</h3>

Nilai `CLAUDE_CODE_SESSION_ACCESS_TOKEN` memiliki awalan `sk-ant-cc-` diikuti oleh JWT tiga bagian standar:

```text theme={null}
sk-ant-cc-<base64url header>.<base64url payload>.<base64url signature>
```

Lepaskan awalan sebelum meneruskan nilai ke perpustakaan JWT. Token yang dikeluarkan untuk sesi cloud yang di-host Anthropic membawa awalan `sk-ant-si-` sebagai gantinya dan ditandatangani oleh set kunci yang berbeda, jadi tolak nilai apa pun yang tidak dimulai dengan `sk-ant-cc-`.

Algoritma tanda tangan adalah `ES256`, yang merupakan ECDSA pada kurva P-256 dengan SHA-256. Header token membawa `kid` yang mengidentifikasi kunci mana di JWKS yang menandatanganinya.

<h2 id="verify-the-token">
  Verifikasi token
</h2>

Verifikasi berjalan di salah satu dari dua tempat. Layanan di jaringan Anda memverifikasi token secara kriptografis terhadap kunci yang diterbitkan Anthropic, dan skrip wrapper di dalam sesi dapat menggunakan decoder bawaan binary runner sebagai gantinya.

<h3 id="verify-the-token-from-your-service">
  Verifikasi token dari layanan Anda
</h3>

Anthropic menerbitkan kunci verifikasi di endpoint publik yang tidak diautentikasi:

```text theme={null}
https://api.anthropic.com/v1/code/.well-known/jwks.json
```

Respons adalah [JSON Web Key Set](https://www.rfc-editor.org/rfc/rfc7517) standar. Anthropic memutar kunci penandatanganan secara berkala, dan kunci dari sebelum rotasi tetap berada di set cukup lama sehingga token yang mereka tandatangani terus memverifikasi, jadi jangan pin kunci tunggal. Endpoint menetapkan `Cache-Control: public, max-age=300`, jadi caching set kunci dan refetching setiap lima menit aman.

Verifikasi setiap token masuk terhadap pemeriksaan ini:

<Steps>
  <Step title="Periksa awalan">
    Tolak nilai jika tidak dimulai dengan `sk-ant-cc-`, kemudian lepaskan awalan itu. Sisanya adalah JWT kompak standar.
  </Step>

  <Step title="Verifikasi tanda tangan">
    Ambil JWKS, pilih kunci yang `kid`-nya cocok dengan header token, dan verifikasi tanda tangan `ES256`. Tolak token yang header `alg`-nya bukan `ES256`. Jika token tiba dengan `kid` yang tidak ada di set kunci cache Anda, ambil JWKS sekali sebelum menolaknya: setelah rotasi, token baru ditandatangani dengan kunci yang set cache Anda belum miliki.
  </Step>

  <Step title="Verifikasi penerbit">
    Tolak token jika `iss` bukan persis `ccr`.
  </Step>

  <Step title="Verifikasi audiens terhadap lingkungan Anda">
    Klaim `aud` adalah array. Tolak token kecuali berisi ID lingkungan Anda, yang memiliki bentuk `ccpool_...`. ID lingkungan ditampilkan di dialog detail lingkungan Anda di [halaman admin **Cloud environments**](https://claude.ai/admin-settings/cloud-environments), dan muncul sebagai klaim `ccr:pool_id` di salah satu token sesi lingkungan. Pemeriksaan ini adalah apa yang membatasi token ke lingkungan Anda dan menolak token yang dikeluarkan untuk organisasi lain.
  </Step>

  <Step title="Verifikasi peran">
    Tolak token jika `ccr:role` bukan persis `session_worker`. Token lain yang dikeluarkan untuk lingkungan yang di-host sendiri, seperti rahasia lingkungan, token runner, dan pesanan kerja, ditandatangani oleh set kunci yang sama tetapi membawa peran yang berbeda.
  </Step>

  <Step title="Verifikasi kedaluwarsa">
    Tolak token jika `exp` berada di masa lalu. Anthropic mengeluarkan token sesi dengan masa pakai empat jam secara default dan maksimal delapan jam. Runner menyegarkan token sebelum kedaluwarsa dan mendorong nilai baru ke sesi, jadi subproses yang Claude mulai setelah penyegaran mewarisinya. Satu sesi dapat oleh karena itu menyajikan beberapa token valid yang berbeda ke layanan Anda selama masa hidupnya.
  </Step>

  <Step title="Baca identitas">
    Identitas pengguna yang membuat ada di klaim `act`: `act.sub` adalah ID pengguna Anthropic mereka dalam bentuk dengan awalan `user:<id>`, dan `act.email`, ketika permukaan yang membuat merekamnya, adalah alamat email mereka. Sesi yang identitas layanan organisasi Anda buat, termasuk sesi saluran Claude Tag, membawa subjek `agent:` sebagai gantinya, jadi perlakukan sesi sebagai dibuat pengguna hanya ketika `act.sub` membawa awalan `user:`, daripada menguji apakah klaim identitas tidak ada. Lihat [referensi klaim](#claims-reference) untuk struktur lengkap dan klaim duplikat datar.
  </Step>
</Steps>

Pemeriksaan memetakan langsung ke perpustakaan JWT standar. Contoh di bawah mengimplementasikan urutan lengkap di Node.js dengan [`jose`](https://www.npmjs.com/package/jose), yang menangani pengambilan JWKS, caching, dan pemilihan `kid`, dan di Python dengan [`PyJWT`](https://pyjwt.readthedocs.io/) dan klien JWKS bawaan-nya.

<Tabs>
  <Tab title="Node.js (jose)">
    ```typescript theme={null}
    import { createRemoteJWKSet, jwtVerify } from "jose";

    const JWKS = createRemoteJWKSet(
      new URL("https://api.anthropic.com/v1/code/.well-known/jwks.json")
    );

    const PREFIX = "sk-ant-cc-";
    const EXPECTED_POOL_ID = "ccpool_...";

    export async function verifySessionToken(raw: string) {
      if (!raw.startsWith(PREFIX)) {
        throw new Error("not a self-hosted runner session token");
      }
      const jwt = raw.slice(PREFIX.length);

      const { payload } = await jwtVerify(jwt, JWKS, {
        issuer: "ccr",
        audience: EXPECTED_POOL_ID,
        algorithms: ["ES256"],
      });

      if (payload["ccr:role"] !== "session_worker") {
        throw new Error("token is not a session_worker token");
      }

      const act = payload.act as { email?: string; sub?: string };
      return {
        sessionId: payload["ccr:session_id"] as string,
        poolId: payload["ccr:pool_id"] as string,
        orgId: payload["ccr:org_id"] as string,
        creatorEmail: act?.email,
        creatorSub: act?.sub,
      };
    }
    ```
  </Tab>

  <Tab title="Python (PyJWT)">
    ```python theme={null}
    import jwt
    from jwt import PyJWKClient

    JWKS_URL = "https://api.anthropic.com/v1/code/.well-known/jwks.json"
    PREFIX = "sk-ant-cc-"
    EXPECTED_POOL_ID = "ccpool_..."

    jwks = PyJWKClient(JWKS_URL)


    def verify_session_token(raw: str) -> dict:
        if not raw.startswith(PREFIX):
            raise ValueError("not a self-hosted runner session token")
        token = raw.removeprefix(PREFIX)

        signing_key = jwks.get_signing_key_from_jwt(token)
        payload = jwt.decode(
            token,
            signing_key.key,
            algorithms=["ES256"],
            issuer="ccr",
            audience=EXPECTED_POOL_ID,
        )

        if payload.get("ccr:role") != "session_worker":
            raise ValueError("token is not a session_worker token")

        act = payload.get("act") or {}
        return {
            "session_id": payload["ccr:session_id"],
            "pool_id": payload["ccr:pool_id"],
            "org_id": payload["ccr:org_id"],
            "creator_email": act.get("email"),
            "creator_sub": act.get("sub"),
        }
    ```
  </Tab>
</Tabs>

<h3 id="verify-the-token-inside-the-session">
  Verifikasi token di dalam sesi
</h3>

[Skrip wrapper](/docs/id/self-hosted-environments-configuration#wrapper-scripts) berjalan di dalam sesi, sebelum Claude dimulai. Alih-alih memanggil perpustakaan JWT, mereka dapat menjalankan subperintah `self-hosted-runner decode-token` binary runner. Subperintah membaca token dari argumen posisional, dari `CLAUDE_CODE_SESSION_ACCESS_TOKEN`, atau dari stdin yang disalurkan, dalam urutan itu, kemudian melepaskan awalan, memverifikasi tanda tangan terhadap endpoint JWKS, memeriksa kedaluwarsa, dan mencetak klaim sebagai JSON. Subperintah melakukan pemeriksaan tanda tangan dan kedaluwarsa saja; tidak memeriksa `iss`, `aud`, atau `ccr:role`. Ketika keputusan auth wrapper Anda bergantung pada klaim tersebut, baca mereka dari JSON yang dicetak dan bandingkan secara eksplisit.

Perintah ini mengekstrak identitas pembuat, lebih memilih subjek penyedia SSO, kemudian alamat email, kemudian subjek `act.sub` pembuat, `user:<id>` atau `agent:<id>`:

```bash theme={null}
"$CLAUDE_RUNNER_CLAUDE_BIN" self-hosted-runner decode-token | jq -re '.act.attested_by.sub // .act.email // .act.sub'
```

Wrapper menerima jalur absolut ke binary runner sendiri di `CLAUDE_RUNNER_CLAUDE_BIN`; gunakan jalur itu daripada `claude` yang diselesaikan PATH sehingga decode berjalan pada binary yang sama yang digunakan runner sendiri.

Gunakan `jq -re` daripada `jq -r` sehingga klaim yang hilang menyebabkan exit bukan nol. Dengan `-r` saja, klaim yang hilang mencetak string literal `null` dan keluar nol, yang secara diam-diam melewatkan nilai buruk ke hilir. Teruskan `--no-verify` ke `decode-token` hanya untuk inspeksi offline di mana endpoint JWKS tidak dapat dijangkau.

<h2 id="claims-reference">
  Referensi klaim
</h2>

Tabel di bawah mencantumkan klaim token sesi yang relevan untuk verifikasi. Baca identitas dari namespace `ccr:*` dan rantai `act`; klaim duplikat backward-compatibility yang datar `account_email`, `organization_uuid`, dan `account_uuid` dapat dihapus. Sesi yang dibuat oleh identitas layanan organisasi Anda, termasuk sesi saluran Claude Tag, membawa subjek `agent:` dalam `act.sub` dan menghilangkan `act.email`, `ccr:account_id`, `account_email`, dan `account_uuid`. Dua klaim email bersifat opsional untuk sesi yang dibuat pengguna juga: Anthropic merekamnya saat pembuatan sesi hanya ketika kredensial permintaan pembuatan membawa email, dan sesi yang dikirim dari CLI dapat tidak memiliki keduanya, jadi kunci identitas pada `act.sub` atau `ccr:account_id` daripada pada email. Token juga dapat membawa klaim tambahan di luar tabel ini; abaikan klaim yang tidak Anda kenali.

| Klaim               | Tipe             | Deskripsi                                                                                                                                                                                                                                                                                                                                                                                                   |
| :------------------ | :--------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `iss`               | string           | Selalu `ccr`.                                                                                                                                                                                                                                                                                                                                                                                               |
| `sub`               | string           | `ccr:session:<session_id>`.                                                                                                                                                                                                                                                                                                                                                                                 |
| `aud`               | array of strings | Selalu berisi `anthropic-api`. Untuk sesi di lingkungan yang di-host sendiri, array juga berisi ID lingkungan Anda, seperti `ccpool_...`. Verifikasi ID lingkungan, bukan `anthropic-api`.                                                                                                                                                                                                                  |
| `exp`               | number           | Kedaluwarsa sebagai Unix timestamp. Masa pakai default empat jam, maksimal delapan jam.                                                                                                                                                                                                                                                                                                                     |
| `iat`               | number           | Issued-at sebagai Unix timestamp.                                                                                                                                                                                                                                                                                                                                                                           |
| `jti`               | string           | Pengidentifikasi token unik.                                                                                                                                                                                                                                                                                                                                                                                |
| `ccr:role`          | string           | Selalu `session_worker` untuk token sesi.                                                                                                                                                                                                                                                                                                                                                                   |
| `ccr:session_id`    | string           | ID sesi. Nilai yang sama dengan akhiran `sub`.                                                                                                                                                                                                                                                                                                                                                              |
| `ccr:pool_id`       | string           | ID lingkungan Anda. Nilai yang sama yang muncul dalam `aud`.                                                                                                                                                                                                                                                                                                                                                |
| `ccr:org_id`        | string           | ID organisasi Anthropic Anda.                                                                                                                                                                                                                                                                                                                                                                               |
| `ccr:account_id`    | string           | ID akun Anthropic pengguna pembuat: nilai `act.sub` tanpa awalan `user:`, ID `user_...` yang diberi tag. Nilai yang sama yang dibawa `CLAUDE_RUNNER_ACCOUNT_ID` dari [spawn-runner hook](/docs/id/self-hosted-environments-configuration#the-spawn-runner-hook) dan [`--lock-to-account`](/docs/id/self-hosted-environments-reference#runner-cli-flags) terima, jadi ketiganya dibandingkan sebagai string yang sama. |
| `account_email`     | string           | Duplikat dari `act.email`; tidak ada kapan pun `act.email` tidak ada.                                                                                                                                                                                                                                                                                                                                       |
| `organization_uuid` | string           | UUID organisasi Anthropic Anda.                                                                                                                                                                                                                                                                                                                                                                             |
| `account_uuid`      | string           | UUID akun Anthropic pengguna pembuat.                                                                                                                                                                                                                                                                                                                                                                       |
| `act`               | object           | Rantai delegasi [RFC 8693](https://www.rfc-editor.org/rfc/rfc8693). Lihat [Rantai `act`](#the-act-chain).                                                                                                                                                                                                                                                                                                   |

<h3 id="the-act-chain">
  Rantai `act`
</h3>

Klaim `act` merekam jalur delegasi lengkap dari identitas pengguna atau layanan yang membuat sesi hingga ke [lingkungan](/docs/id/self-hosted-environments#key-concepts) yang rahasianya mengakui runner, dan identitas yang membuat rahasia itu. Pembuat adalah aktor terluar, jadi `act.sub` mengidentifikasi mereka secara langsung.

| Jalur             | Deskripsi                                                                                                                                                                                                                                                                |
| :---------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `act.sub`         | ID pengguna Anthropic pengguna pembuat, dalam bentuk `user:<id>`, atau `agent:<id>` ketika identitas layanan organisasi Anda membuat sesi, seperti yang dilakukannya untuk sesi saluran Claude Tag.                                                                      |
| `act.email`       | Alamat email pengguna pembuat, ketika satu dicatat saat pembuatan sesi. Jangan memerlukan itu; kunci pada `act.sub`.                                                                                                                                                     |
| `act.attested_by` | Atestasi penyedia identitas hulu untuk pengguna pembuat, ketika tersedia. `act.attested_by.sub` adalah subjek yang dikeluarkan penyedia SSO Anda, seperti Google atau Okta. Lebih suka ini daripada `act.email` ketika memetakan ke identitas dalam sistem Anda sendiri. |
| `act.act`         | Runner yang menelurkan sesi. `act.act.sub` adalah `ccr:runner:<runner_id>`.                                                                                                                                                                                              |
| `act.act.act`     | Lingkungan. `act.act.act.sub` adalah `ccr:pool:<pool_id>`.                                                                                                                                                                                                               |
| `act.act.act.act` | Identitas yang membuat rahasia lingkungan yang didaftarkan runner. Rantai berakhir di sini.                                                                                                                                                                              |

<h2 id="scope-derived-credentials">
  Scope derived credentials
</h2>

Token sesi mengidentifikasi identitas pengguna atau layanan yang membuat sesi, tetapi jangan perlakukan sebagai setara dengan pembuat itu masuk langsung. Token berada di variabel lingkungan di dalam sesi, jadi kode apa pun yang Claude jalankan, dan alat atau server MCP apa pun yang dimulai sesi, dapat membaca dan menyajikannya.

Verifikasi juga offline: token yang memverifikasi terhadap JWKS tetap valid hingga `exp`-nya, apa pun yang telah terjadi pada sesi sejak saat itu, dan Anthropic tidak menerbitkan feed revokasi untuk token sesi. Ikat apa pun yang Anda turunkan dari token sesuai dengan itu.

Ketika layanan Anda menukar token untuk kredensial internal, keluarkan kredensial yang dibatasi untuk apa yang dapat dijangkau satu sesi coding:

* **Batasi kemampuan**: berikan akses baca dan tulis ke sumber daya yang dibutuhkan sesi untuk tugas coding, bukan kemampuan administratif yang dimiliki pembuat di tempat lain.
* **Batasi masa pakai**: ikat kredensial yang diturunkan ke `exp` token, atau lebih pendek.
* **Audit sebagai sesi**: catat `ccr:session_id` dan `jti` bersama identitas pembuat sehingga Anda dapat melacak tindakan kembali ke sesi tertentu.

<h2 id="related-environment-variables">
  Variabel lingkungan terkait
</h2>

Identitas pembuat juga muncul dalam variabel lingkungan biasa di dua permukaan yang tidak pernah memverifikasi token:

* **[Hook `spawn-runner`](/docs/id/self-hosted-environments-configuration#the-spawn-runner-hook), di orchestrator**: hook berjalan sebelum ada runner untuk sesi antrian dan menerima identitas pembuat dalam variabel seperti `CLAUDE_RUNNER_ACCOUNT_EMAIL` dan `CLAUDE_RUNNER_ACCOUNT_ID`. Orchestrator membacanya dari pesanan kerja, token sekali pakai yang ditandatangani yang mengotorisasi pemunculan satu runner, tanpa memverifikasi tanda tangan pesanan kerja itu sendiri; klaim dipercaya karena pesanan kerja tiba melalui koneksi orchestrator ke Anthropic, yang rahasia lingkungan autentikasi.
* **[Skrip wrapper](/docs/id/self-hosted-environments-configuration#wrapper-scripts), di dalam sesi**: wrapper menerima `CCR_SESSION_ACCOUNT_EMAIL`, email pembuat yang telah diekstrak sebelumnya dari token tanpa verifikasi tanda tangan. Variabel cocok untuk pelabelan, seperti trailer commit, bukan untuk keputusan auth.

Gunakan variabel biasa untuk keputusan sisi orchestrator seperti memilih gambar mesin. Gunakan `CLAUDE_CODE_SESSION_ACCESS_TOKEN` ketika layanan hilir memerlukan bukti kriptografis independen daripada mempercayai lingkungan runner.

<h2 id="what’s-next">
  Apa selanjutnya
</h2>

* [Lingkungan yang di-host sendiri](/docs/id/self-hosted-environments): lingkungan, runner, dan model sesi; [quickstart](/docs/id/self-hosted-environments-quickstart) dan [Deploy to production](/docs/id/self-hosted-environments-deploy) memegang setup dan operasi
* [Sesuaikan sesi](/docs/id/self-hosted-environments-configuration): skrip wrapper yang menggunakan token, dan hook `spawn-runner`
* [Referensi](/docs/id/self-hosted-environments-reference): flag CLI, variabel lingkungan, dan metrik
