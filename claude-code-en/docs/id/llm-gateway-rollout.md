> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Luncurkan gateway LLM untuk organisasi Anda

> Terapkan produk gateway untuk Claude Code: konfigurasikan untuk meneruskan apa yang dikirim Claude Code, keluarkan kredensial pengembang, distribusikan konfigurasi melalui pengaturan terkelola, dan verifikasi peluncuran.

Halaman ini memandu administrator melalui peluncuran gateway LLM untuk Claude Code. Halaman ini mengasumsikan Anda memiliki produk gateway yang diterapkan yang memenuhi [persyaratan gateway](#gateway-requirements). Penerapan atau pengoperasian produk spesifik apa pun tidak tercakup di sini; terapkan milik Anda mengikuti dokumentasi vendor.

<Note>
  * Untuk menghubungkan Claude Code di mesin Anda sendiri ke gateway yang ada, lihat [Hubungkan Claude Code ke gateway LLM](/docs/id/llm-gateway-connect)
  * Untuk apa yang dikirim Claude Code ke gateway dan apa yang harus diteruskan, lihat [panduan kompatibilitas gateway](/docs/id/llm-gateway-protocol)
</Note>

<h2 id="prerequisites">
  Prasyarat
</h2>

Untuk menyelesaikan peluncuran, Anda akan memerlukan:

* Gateway yang diterapkan di infrastruktur Anda, melayani HTTPS di alamat yang tepat yang akan Anda distribusikan kepada pengembang, bukan alamat yang mengalihkannya, dan dikonfigurasi untuk merutekan nama model Claude ke penyedia Anda
* Kredensial penyedia untuk gateway yang akan diteruskan dengan:
  * Untuk API Anthropic: kunci API dari [Konsol Claude](https://platform.claude.com/settings/keys)
  * Untuk penyedia cloud: kredensial cloud dengan akses model. Lihat prasyarat di halaman [Amazon Bedrock](/docs/id/amazon-bedrock#prerequisites), [Google Cloud's Agent Platform](/docs/id/google-vertex-ai#prerequisites), atau [Microsoft Foundry](/docs/id/microsoft-foundry#prerequisites)
* Cara untuk mengirimkan file pengaturan ke mesin pengembang, seperti MDM atau manajemen konfigurasi
  * Jika Anda belum memilikinya, [bagaimana pengaturan mencapai perangkat](/docs/id/admin-setup#decide-how-settings-reach-devices) membandingkan opsi

<h3 id="gateway-requirements">
  Persyaratan gateway
</h3>

Produk apa pun yang menyediakan gateway harus:

* **Terima format API yang didukung**: salah satu format dalam [tabel format API](/docs/id/llm-gateway-protocol#api-formats). Langkah peluncuran di bawah mengasumsikan API Pesan Anthropic di `POST /v1/messages`, yang dilayani sebagian besar gateway
* **Alirkan respons**: teruskan peristiwa yang dikirim server saat tiba, termasuk ping keep-alive, daripada membuffer seluruh respons; [streaming](/docs/id/llm-gateway-protocol#streaming) mencakup apa yang buffering atau ping yang dilucuti rusak
* **Rutekan nama model Claude**: petakan setiap nama yang digunakan pengembang ke model upstream. Claude Code mengirimkan nama model seperti `claude-sonnet-4-6` di setiap permintaan; di sebagian besar produk gateway pemetaan adalah daftar model atau tabel perutean dalam konfigurasi gateway sendiri
* **Teruskan header dan body tanpa perubahan**: teruskan `anthropic-beta`, `anthropic-version`, dan badan permintaan di kedua arah; [tabel pass-through fitur](/docs/id/llm-gateway-protocol#feature-pass-through) memetakan masing-masing ke fitur yang rusak tanpanya
* **Kembalikan kesalahan upstream tanpa modifikasi**: pemulihan otomatis Claude Code cocok dengan kata-kata kesalahan, jadi membungkus kesalahan dalam amplop gateway sendiri memecahnya, kecuali amplop pesan membawa salah satu token `capability_rejected:` yang [gateway aplikasi Claude menggantikan untuk kata-kata kesalahan penyedia cloud](/docs/id/claude-apps-gateway-config#upstream-error-messages)
* **Bebaskan jalur dari inspeksi WAF badan permintaan**: prompt Claude Code membawa kode sumber dan tag gaya XML yang cocok dengan aturan badan cross-site-scripting; WAF di depan gateway mengembalikan `403` pada sesi nyata sementara permintaan uji pendek lulus

Secara opsional, layani `GET /v1/models` sehingga Claude Code dapat mengisi pemilih model dari gateway Anda dengan [penemuan model](/docs/id/llm-gateway-protocol#model-discovery).

<h2 id="rollout-steps">
  Langkah peluncuran
</h2>

Peluncuran membutuhkan lima langkah, masing-masing dengan checkpoint:

1. [Konfirmasi gateway merutekan model Anda](#confirm-the-gateway-routes-your-models)
2. [Keluarkan kredensial untuk setiap pengembang](#issue-developer-credentials)
3. [Uji Claude Code terhadap gateway](#test-claude-code-against-the-gateway)
4. [Distribusikan URL dasar dan kredensial](#distribute-the-configuration)
5. [Verifikasi dari mesin pengembang](#verify-the-rollout)

Langkah-langkah melibatkan tiga kredensial berbeda, dan checkpoint menamai mereka dengan placeholder sehingga Anda dapat mengetahui mana yang salah ketika sesuatu gagal:

| Kredensial                       | Siapa yang memegangnya                                                                                         | Placeholder dalam checkpoint                                       |
| :------------------------------- | :------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------- |
| Kredensial penyedia              | Gateway, yang meneruskannya ke penyedia upstream                                                               | Dikonfigurasi di gateway; tidak pernah muncul dalam perintah klien |
| Kredensial administratif gateway | Anda, jika produk gateway Anda mengeluarkan satu untuk antarmuka admin atau testnya                            | `<gateway-key>`                                                    |
| Kunci pengembang                 | Setiap pengembang, dikeluarkan oleh gateway di [Keluarkan kredensial pengembang](#issue-developer-credentials) | `<developer-key>`                                                  |

<h3 id="confirm-the-gateway-routes-your-models">
  Konfirmasi gateway merutekan model Anda
</h3>

Gateway Anda seharusnya sudah dikonfigurasi dengan kredensial penyedia Anda, mendengarkan di URL dasarnya, dan meneruskan permintaan ke API penyedia Anda. Uji bahwa jalur berfungsi end-to-end dengan permintaan minimal, mengganti dua nilai dari penerapan Anda:

* `<gateway-key>` adalah kredensial apa pun yang memungkinkan Anda memanggil gateway sekarang: kunci administratif, kunci uji, atau kunci pengembang Anda sendiri jika Anda sudah mengeluarkan satu. Tidak setiap produk gateway memiliki kredensial admin terpisah; jika milik Anda tidak, keluarkan kunci pengembang untuk diri sendiri di [Keluarkan kredensial pengembang](#issue-developer-credentials) terlebih dahulu
* `model` adalah nama model Claude yang dikonfigurasi gateway untuk dirutekan. Contoh menggunakan `claude-sonnet-4-6`; ganti dengan nama yang telah Anda konfigurasi

<Tabs>
  <Tab title="Bash atau Zsh">
    ```bash theme={null}
    curl -X POST "https://llm-gateway.example.com/v1/messages" \
      -H "Authorization: Bearer <gateway-key>" \
      -H "anthropic-version: 2023-06-01" \
      -H "content-type: application/json" \
      -d '{"model": "claude-sonnet-4-6", "max_tokens": 1, "messages": [{"role": "user", "content": "."}]}'
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    Invoke-RestMethod -Method Post -Uri "https://llm-gateway.example.com/v1/messages" `
      -Headers @{ "Authorization" = "Bearer <gateway-key>"; "anthropic-version" = "2023-06-01" } `
      -ContentType "application/json" `
      -Body '{"model": "claude-sonnet-4-6", "max_tokens": 1, "messages": [{"role": "user", "content": "."}]}'
    ```
  </Tab>
</Tabs>

**Checkpoint**: `200` dengan bidang `content` berarti gateway mencapai penyedia dengan nama model itu. `404` berarti nama itu tidak dirutekan di gateway; `401` dari penyedia berarti kredensial penyedia gateway salah.

Ulangi permintaan sekali per nama model Claude dalam konfigurasi perutean gateway Anda. Nama yang tidak dirutekan gateway mengembalikan `404` kepada pengembang mana pun yang memilihnya, jadi uji setiap nama sebelum peluncuran.

<Note>
  Hindari melayani gateway di belakang pengalihan. Pengalihan dapat menghilangkan badan permintaan atau menghapus header kredensial pada permintaan inferensi, dan [penemuan model](/docs/id/llm-gateway-protocol#model-discovery) memperlakukan pengalihan apa pun sebagai kegagalan sehingga kredensial tidak dapat bocor ke target pengalihan.
</Note>

<h3 id="issue-developer-credentials">
  Keluarkan kredensial pengembang
</h3>

Setiap pengembang memerlukan kunci gateway mereka sendiri untuk autentikasi. Buat kredensial per pengembang di gateway, mengikuti dokumentasi manajemen kredensial produk Anda.

Konfirmasi kunci yang baru dikeluarkan berfungsi terhadap gateway dengan permintaan yang sama seperti [Konfirmasi gateway merutekan model Anda](#confirm-the-gateway-routes-your-models), mengganti `<gateway-key>` dengan `<developer-key>` baru:

<Tabs>
  <Tab title="Bash atau Zsh">
    ```bash theme={null}
    curl -X POST "https://llm-gateway.example.com/v1/messages" \
      -H "Authorization: Bearer <developer-key>" \
      -H "anthropic-version: 2023-06-01" \
      -H "content-type: application/json" \
      -d '{"model": "claude-sonnet-4-6", "max_tokens": 1, "messages": [{"role": "user", "content": "."}]}'
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    Invoke-RestMethod -Method Post -Uri "https://llm-gateway.example.com/v1/messages" `
      -Headers @{ "Authorization" = "Bearer <developer-key>"; "anthropic-version" = "2023-06-01" } `
      -ContentType "application/json" `
      -Body '{"model": "claude-sonnet-4-6", "max_tokens": 1, "messages": [{"role": "user", "content": "."}]}'
    ```
  </Tab>
</Tabs>

**Checkpoint**: `200` dengan bidang `content` berarti kunci pengembang mencapai gateway dan gateway meneruskannya. `401` di sini, ketika [langkah sebelumnya](#confirm-the-gateway-routes-your-models) berhasil, berarti kunci pengembang salah atau belum berlaku di gateway.

Mengeluarkan satu kunci per pengembang daripada kunci bersama adalah apa yang membuat atribusi penggunaan per pengembang dan offboarding individual berfungsi. Variabel lingkungan yang menyimpan kunci tergantung pada header mana yang dibaca gateway. Untuk gateway yang memeriksa kredensial di header `Authorization: Bearer`, pengembang menetapkan kunci mereka di `ANTHROPIC_AUTH_TOKEN`. Untuk gateway yang membaca kunci dari header `x-api-key`, pengembang menetapkan `ANTHROPIC_API_KEY` sebagai gantinya; [tabel kredensial](/docs/id/llm-gateway-connect#set-the-credential-variable) mencakup pemetaan.

<h3 id="test-claude-code-against-the-gateway">
  Uji Claude Code terhadap gateway
</h3>

Jalankan Claude Code melalui gateway sendiri sebelum mendistribusikan apa pun, menggunakan konfigurasi yang sama dengan yang akan didistribusikan peluncuran di seluruh armada. Ketik ini langsung di terminal, bukan di file `.env` atau pengaturan; mereka hanya berlaku untuk sesi terminal ini, jadi menutupnya mengembalikan mesin Anda ke konfigurasi normalnya. Gunakan `ANTHROPIC_API_KEY` daripada `ANTHROPIC_AUTH_TOKEN` jika gateway Anda membaca header `x-api-key`:

<Tabs>
  <Tab title="Bash atau Zsh">
    ```bash theme={null}
    export ANTHROPIC_BASE_URL=https://llm-gateway.example.com
    export ANTHROPIC_AUTH_TOKEN="<developer-key>"
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    $env:ANTHROPIC_BASE_URL = "https://llm-gateway.example.com"
    $env:ANTHROPIC_AUTH_TOKEN = "<developer-key>"
    ```
  </Tab>
</Tabs>

Kemudian kirim prompt satu kali melalui gateway:

```bash theme={null}
claude -p "Reply with one word: connected"
```

**Checkpoint**: prompt mengembalikan respons, dan permintaan muncul dalam log gateway sebagai `POST` ke jalur `/v1/messages` dengan status `200`. Claude Code menambahkan string kueri seperti `?beta=true`, jadi cocokkan pada jalur, bukan URL lengkap. Dua pesan kegagalan menunjuk ke arah berbeda:

* `Not logged in`: periksa log gateway untuk membedakan dua penyebab. Jika kosong, tidak ada kredensial yang mencapai sesi dan tidak ada permintaan yang meninggalkan mesin; jalankan kembali ekspor di shell yang Anda uji. Jika menunjukkan permintaan yang ditolak dengan `x-api-key` dalam badan `401`, gateway mengharapkan kunci di header itu; beralih ke `ANTHROPIC_API_KEY`
* `Failed to authenticate. API Error: 401` berarti kredensial dikirim dan ditolak, dan log gateway mengatakan di mana: `401` yang menyebutkan `api.anthropic.com` atau endpoint penyedia Anda berarti gateway mencapai upstream tetapi kredensial penyedia yang dipegang gateway ditolak, jadi kunci pengembang bekerja dan kredensial penyedia yang dipegang gateway salah atau placeholder

URL dasar yang salah atau tidak dapat dijangkau menghasilkan gejala berbeda: Claude Code [mencoba ulang koneksi dengan backoff](/docs/id/errors#automatic-retries) dan dapat duduk tanpa output selama beberapa menit sebelum melaporkan kesalahan. Jika perintah tampak hang, periksa log gateway daripada menunggu; tidak ada permintaan yang tiba berarti `ANTHROPIC_BASE_URL` tidak menunjuk ke gateway.

<h3 id="distribute-the-configuration">
  Distribusikan konfigurasi
</h3>

Setiap mesin pengembang memerlukan alamat gateway dan kredensial. Anda dapat mendistribusikannya secara terpusat melalui [pengaturan terkelola](/docs/id/managed-settings#delivery-mechanisms), sehingga pengembang tidak mengonfigurasi apa pun, atau berikan pengembang nilai untuk menetapkan sendiri.

<h4 id="what-to-distribute">
  Apa yang harus didistribusikan
</h4>

Set variabel yang sama berlaku jalur mana pun yang Anda pilih. Sebagian besar peluncuran hanya memerlukan `ANTHROPIC_BASE_URL` dan kredensial; sertakan baris bersyarat ketika pengaturan gateway Anda memanggilnya.

| Variabel atau pengaturan                                                                                                                                                                                                         | Apa yang dilakukannya                                                                                                                                                                                                                        | Sertakan ketika                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `ANTHROPIC_BASE_URL`                                                                                                                                                                                                             | Mengirim permintaan API Claude Code ke gateway daripada `api.anthropic.com`                                                                                                                                                                  | Selalu                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `apiKeyHelper`, atau kredensial di `ANTHROPIC_AUTH_TOKEN` atau `ANTHROPIC_API_KEY`                                                                                                                                               | Mengautentikasi setiap permintaan ke gateway. Helper menjalankan perintah untuk mengambil kunci; variabel menyimpan kunci statis, dikirim sebagai `Authorization: Bearer` dan `x-api-key` masing-masing                                      | Selalu; salah satu dari tiga                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `ANTHROPIC_CUSTOM_HEADERS`                                                                                                                                                                                                       | Menambahkan header HTTP ekstra ke setiap permintaan API                                                                                                                                                                                      | Gateway Anda memerlukan header penyewa atau perutean pada setiap permintaan                                                                                                                                                                                                                                                                                                                                                                                                   |
| `CLAUDE_CODE_GATEWAY_HINT_HEADERS`                                                                                                                                                                                               | Mengirim [header hint gateway](/docs/id/llm-gateway-protocol#gateway-hint-headers), yang mengklasifikasikan setiap permintaan untuk keputusan perutean dan penjadwalan di gateway. Memerlukan Claude Code v2.1.273 atau lebih baru                | Gateway Anda membaca header hint                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `CLAUDE_CODE_ENABLE_GATEWAY_MODEL_DISCOVERY`                                                                                                                                                                                     | Menanyakan `/v1/models` gateway saat startup dan menambahkan nama yang dikembalikan ke pemilih `/model`                                                                                                                                      | Gateway Anda melayani `/v1/models` dan Anda ingin pemilih pengembang diisi darinya                                                                                                                                                                                                                                                                                                                                                                                            |
| `CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS`                                                                                                                                                                                         | Menghentikan Claude Code mengirim header dan bidang badan kemampuan pra-rilis. [Nonaktifkan kemampuan pra-rilis](/docs/id/llm-gateway-protocol#disable-pre-release-capabilities) mencakup cakupan yang tepat                                      | Gateway Anda meneruskan ke upstream Amazon Bedrock atau Agent Platform Google Cloud yang menolak bidang beta. Lihat [Persyaratan gateway](#gateway-requirements)                                                                                                                                                                                                                                                                                                              |
| `CLAUDE_CODE_SKIP_FAST_MODE_NETWORK_ERRORS` atau `CLAUDE_CODE_SKIP_FAST_MODE_ORG_CHECK`                                                                                                                                          | Mengembalikan [fast mode](/docs/id/fast-mode) ketika pemeriksaan ketersediaannya, yang memanggil `api.anthropic.com` secara langsung daripada mengikuti `ANTHROPIC_BASE_URL`, gagal, dicegat, atau dilewati karena kurangnya kredensial Anthropic | Organisasi Anda menggunakan fast mode, dan pengembang mengautentikasi dengan `ANTHROPIC_AUTH_TOKEN` saja, dengan kunci yang dikeluarkan gateway di `ANTHROPIC_API_KEY` atau dari `apiKeyHelper`, atau jaringan Anda memblokir atau mencegat permintaan langsung ke `api.anthropic.com`; [gunakan fast mode di belakang proxy dan gateway LLM](/docs/id/fast-mode#use-fast-mode-behind-proxies-and-llm-gateways) mencakup variabel mana dari dua yang cocok dengan konfigurasi Anda |
| `ANTHROPIC_MODEL` atau [`ANTHROPIC_DEFAULT_HAIKU_MODEL`](/docs/id/model-config)                                                                                                                                                       | Tetapkan nama model mana yang diminta Claude Code untuk sesi utama dan lalu lintas latar belakang                                                                                                                                            | Gateway Anda merutekan nama model yang tidak cocok dengan default Claude Code, atau Anda merutekan [fungsionalitas latar belakang](/docs/id/costs#background-token-usage) ke model berbeda. Rutekan nama penggantian dan nama default Claude Code di gateway, karena beberapa sub-panggilan dapat meminta nama default terlepas dari penggantian; [konfigurasi model](/docs/id/model-config) mencakup model mana yang digunakan setiap bagian dari sesi                                 |
| `ANTHROPIC_BEDROCK_BASE_URL`, `ANTHROPIC_VERTEX_BASE_URL`, `ANTHROPIC_FOUNDRY_BASE_URL`, atau `ANTHROPIC_AWS_BASE_URL` dengan [variabel untuk penyedia itu](/docs/id/llm-gateway-connect#route-to-a-cloud-provider-through-a-gateway) | Arahkan Claude Code ke gateway melalui URL dasar khusus penyedia. Bedrock dan Agent Platform juga beralih ke format permintaan asli penyedia                                                                                                 | Gateway Anda di depan Bedrock, Agent Platform, Foundry, atau Platform Claude di AWS; lihat [Format API](/docs/id/llm-gateway-protocol#api-formats)                                                                                                                                                                                                                                                                                                                                 |

<h4 id="distribute-through-managed-settings">
  Distribusikan melalui pengaturan terkelola
</h4>

Berikan variabel melalui blok `env` dari [file pengaturan terkelola](/docs/id/managed-settings#delivery-mechanisms), didorong oleh MDM, kebijakan registri, atau manajemen konfigurasi:

```json theme={null}
{
  "env": {
    "ANTHROPIC_BASE_URL": "https://llm-gateway.example.com"
  },
  "apiKeyHelper": "/usr/local/bin/get-gateway-key"
}
```

Tambahkan variabel bersyarat dari tabel ke blok `env` yang sama. `ANTHROPIC_BASE_URL` yang dikelola diberlakukan dan tidak dapat ditimpa oleh ekspor shell pengembang, karena Claude Code menerapkannya di atas lingkungan proses dan pengaturan prioritas lebih rendah.

Jangan sertakan `forceLoginMethod` atau `forceLoginOrgUUID` dalam pengaturan terkelola bersama kredensial gateway. Salah satu kunci, dengan nilai apa pun, memblokir `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN`, dan `apiKeyHelper` saat startup, dan pengembang tidak dapat melanjutkan. Mereka melihat `This machine's managed settings require a first-party login`, atau [`Administrator policy requires a Cloud gateway sign-in`](/docs/id/errors#administrator-policy-requires-a-cloud-gateway-sign-in) di bawah nilai `"gateway"`.

Pengiriman [pengaturan terkelola server](/docs/id/server-managed-settings#platform-availability) memerlukan koneksi langsung ke `api.anthropic.com`, sehingga tidak mencapai sesi yang dirutekan gateway. Penerapan gateway menggunakan jalur pengaturan terkelola berbasis file ini, yang memberlakukan kunci yang sama.

Untuk kredensial, distribusikan satu perintah [`apiKeyHelper`](/docs/id/llm-gateway-connect#rotate-credentials-with-apikeyhelper) dalam file pengaturan terkelola seperti yang ditunjukkan di atas; perintah mengautentikasi ke penyimpanan rahasia Anda sebagai pengembang lokal, sehingga setiap mesin menerima kuncinya sendiri. Alternatifnya, berikan setiap pengembang kunci mereka melalui proses rahasia yang ada dan minta mereka menetapkan `ANTHROPIC_AUTH_TOKEN` sendiri.

Beberapa lingkungan memerlukan pengiriman terpisah:

* Aplikasi desktop membaca perutean gateway dari konfigurasi inferensi pihak ketiga, bukan dari pengaturan terkelola; terapkan file itu melalui MDM bersama pengaturan terkelola sehingga sesi desktop juga merutekan melalui gateway. Lihat [dokumentasi konfigurasi pihak ketiga desktop](https://claude.com/docs/third-party/claude-desktop/configuration) dan [dokumentasi gateway desktop](https://claude.com/docs/third-party/claude-desktop/gateway)
* Runner CI memerlukan `ANTHROPIC_BASE_URL` dan kredensial yang ditetapkan di [lingkungan runner](/docs/id/llm-gateway-connect#configure-each-surface)
* WSL pada mesin Windows terkelola membaca pengaturan terkelola Windows hanya ketika [`wslInheritsWindowsSettings`](/docs/id/settings-reference#wslinheritswindowssettings) adalah `true`

<h4 id="hand-developers-the-values-to-set-themselves">
  Berikan pengembang nilai untuk menetapkan sendiri
</h4>

Jika Anda tidak memiliki distribusi pengaturan terkelola, kirim setiap pengembang apa yang mereka butuhkan untuk mengikuti [halaman koneksi](/docs/id/llm-gateway-connect#configure-claude-code-yourself):

* URL gateway
* Kredensial pribadi mereka
* **Variabel mana yang harus dimasukkan kredensial**: `ANTHROPIC_AUTH_TOKEN` untuk gateway token-bearer, atau `ANTHROPIC_API_KEY` untuk gateway `x-api-key`. Memberitahu pengembang mana yang menghemat mereka dari trial-and-error yang dijelaskan di [halaman koneksi](/docs/id/llm-gateway-connect#set-the-credential-variable)
* Variabel bersyarat apa pun dari [tabel Apa yang harus didistribusikan](#what-to-distribute), dengan nilainya

[Halaman koneksi](/docs/id/llm-gateway-connect#configure-claude-code-yourself) memandu pengembang melalui pengaturan masing-masing.

**Checkpoint**: pada mesin pengembang, `claude` memulai sesi tanpa menampilkan layar login, karena kredensial yang didistribusikan memenuhi autentikasi. Kemudian jalankan `/status` dan buka tab **Status**: baris `Anthropic base URL` menunjukkan alamat gateway, dan untuk distribusi terkelola baris `Setting sources` mencakup pengaturan terkelola. Layar login, atau baris `Anthropic base URL` yang hilang, berarti konfigurasi tidak mencapai mesin.

<h3 id="verify-the-rollout">
  Verifikasi peluncuran
</h3>

Konfirmasi semuanya berfungsi dari mesin pengembang, bukan host gateway, sehingga tes mencakup jalur jaringan yang digunakan pengembang. Kirim permintaan streaming, yang memeriksa endpoint, pass-through streaming, dan perutean model sekaligus:

<Tabs>
  <Tab title="Bash atau Zsh">
    ```bash theme={null}
    curl -N -X POST "https://llm-gateway.example.com/v1/messages" \
      -H "Authorization: Bearer <developer-key>" \
      -H "anthropic-version: 2023-06-01" \
      -H "content-type: application/json" \
      -d '{"model": "claude-sonnet-4-6", "max_tokens": 16, "stream": true, "messages": [{"role": "user", "content": "count to 3"}]}'
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    $body = '{"model": "claude-sonnet-4-6", "max_tokens": 16, "stream": true, "messages": [{"role": "user", "content": "count to 3"}]}'
    $body | curl.exe -N -X POST "https://llm-gateway.example.com/v1/messages" `
      -H "Authorization: Bearer <developer-key>" `
      -H "anthropic-version: 2023-06-01" `
      -H "content-type: application/json" `
      --data-binary '@-'
    ```
  </Tab>
</Tabs>

Anda seharusnya melihat baris `data:` tiba secara bertahap. Seluruh respons tiba sekaligus setelah jeda berarti gateway membuffer, yang menghentikan Claude Code; `404` berarti nama model tidak dirutekan. Ulangi per nama model.

Kemudian mulai `claude` dan kirim pesan. Setiap gejala pada langkah ini memiliki satu penyebab:

* Prompt login berarti celah kredensial. Jalankan `/status` dan buka tab **Status**: ketika baris `Setting sources` tidak mencakup pengaturan terkelola, distribusi tidak mencapai mesin; ketika itu terjadi, kredensial pengembang tidak dikirimkan, jadi atur `ANTHROPIC_AUTH_TOKEN` atau `apiKeyHelper`
* Kesalahan `Failed to authenticate` berarti gateway menolak permintaan; lognya mengatakan kredensial mana yang gagal. Penolakan yang dicatat gateway sendiri menyebutkan kunci pengembang, sementara `401` dari `api.anthropic.com` atau endpoint penyedia Anda berarti kredensial penyedia yang dipegang gateway ditolak
* Prompt persetujuan satu kali untuk kunci diharapkan pada penggunaan pertama ketika gateway mengharapkan kunci di header `x-api-key`, ditetapkan sebagai `ANTHROPIC_API_KEY`. Dengan `ANTHROPIC_AUTH_TOKEN`, tidak ada prompt yang muncul dan variabel mengambil alih secara diam-diam; login claude.ai yang sebelumnya disimpan tidak aktif untuk sesi itu

Jika organisasi Anda menggunakan [fast mode](/docs/id/fast-mode), jalankan `/fast` di sini juga: pemeriksaan ketersediaan memanggil `api.anthropic.com` secara langsung daripada mengikuti URL dasar gateway, sehingga sesi yang dirutekan gateway dapat melaporkan fast mode sebagai tidak tersedia atau dinonaktifkan meskipun inferensi berfungsi. [Gunakan fast mode di belakang proxy dan gateway LLM](/docs/id/fast-mode#use-fast-mode-behind-proxies-and-llm-gateways) memetakan setiap pesan ke variabel yang mengembalikannya, didistribusikan dengan [sisa konfigurasi](#distribute-the-configuration).

Terakhir, periksa log gateway untuk pesan yang Anda kirim: kredensial mengidentifikasi pengembang, dan [header `x-claude-code-session-id`](/docs/id/llm-gateway-protocol#request-headers) mengelompokkan permintaan berdasarkan sesi. Jika fitur gagal dengan [gejala pemecahan masalah](/docs/id/llm-gateway-connect#troubleshoot-gateway-errors), gateway menghapus header atau menulis ulang kesalahan; lihat [persyaratan gateway](#gateway-requirements) di atas.

<h2 id="maintain-the-gateway">
  Pertahankan gateway
</h2>

Setelah peluncuran, tiga jenis perubahan mencapai gateway seiring waktu. Masing-masing memiliki gejala untuk diperhatikan dan tindakan yang harus diambil.

| Perubahan                                                                             | Gejala ketika gateway belum mengikuti                                                                                                                                                | Tindakan                                                                                                                                                                                                                                                      |
| :------------------------------------------------------------------------------------ | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Rilis Claude Code baru menambahkan nilai `anthropic-beta` dan bidang badan permintaan | Pengembang melaporkan kesalahan `400` yang menyebutkan bidang baru setelah mereka memperbarui Claude Code; lihat [pass-through fitur](/docs/id/llm-gateway-protocol#feature-pass-through) | Teruskan header `anthropic-*` dan badan permintaan secara verbatim daripada allowlisting; uji rilis Claude Code baru terhadap gateway sebelum mencapai pengembang, periksa area di [Rencanakan upgrade versi Claude Code](#plan-claude-code-version-upgrades) |
| Model Claude baru menjadi tersedia                                                    | Pengembang memilih nama model baru mendapat `404`; pemilih `/model` tidak mencantumkannya                                                                                            | Tambahkan nama model ke konfigurasi perutean gateway, kemudian jalankan kembali [pemeriksaan perutean](#confirm-the-gateway-routes-your-models). Jika Anda mendistribusikan `ANTHROPIC_MODEL` atau variabel model default, perbarui pengaturan terkelola      |
| Kredensial kedaluwarsa atau perlu rotasi                                              | Semua permintaan pengembang mulai gagal dengan `401` dari upstream                                                                                                                   | Rotasi kredensial penyedia gateway sesuai jadwal sendiri; kunci pengembang berputar di gateway, dan [`apiKeyHelper`](/docs/id/llm-gateway-connect#rotate-credentials-with-apikeyhelper) menangani rotasi per pengembang tanpa mendistribusikan ulang pengaturan    |

Saat mengukur batas laju per kunci, akun untuk klien [mencoba ulang kegagalan transien](/docs/id/errors#automatic-retries), termasuk respons `429`, hingga 10 kali dengan backoff, menghormati `Retry-After`. Simpan [panduan kompatibilitas](/docs/id/llm-gateway-protocol) sebagai referensi untuk apa yang dikirim setiap rilis Claude Code.

<h3 id="plan-claude-code-version-upgrades">
  Rencanakan upgrade versi Claude Code
</h3>

Beberapa perilaku Claude Code dibangun ke dalam versi yang diinstal daripada diatur di gateway Anda, jadi memindahkan pengembang ke rilis baru dapat mengubah perilaku di seluruh deployment Anda bahkan ketika konfigurasi gateway belum berubah. Untuk mengontrol kapan itu terjadi, pin pengembang ke versi yang telah diuji dengan [`requiredMaximumVersion`](/docs/id/settings-reference#requiredmaximumversion), atau dengan [`DISABLE_UPDATES`](/docs/id/setup#disable-auto-updates) jika Anda mendistribusikan Claude Code melalui saluran Anda sendiri. Sebelum Anda menaikkan pin, baca entri [changelog](/docs/en/changelog) rilis baru dan [uji terhadap gateway](#test-claude-code-against-the-gateway).

Ketika Anda menguji rilis, header baru atau bidang permintaan yang ditolak gateway muncul sebagai kesalahan `400` yang dijelaskan dalam [Pertahankan gateway](#maintain-the-gateway). Tabel di bawah mencakup perubahan yang bergantung pada versi yang tidak menghasilkan kesalahan, dengan pengaturan yang membuat masing-masing tetap konstan di seluruh upgrade.

| Area                    | Apa yang dapat berubah ketika pengembang upgrade                                                                                                                                                                                                                                                                                                                                             | Pengaturan yang membuat tetap konstan                                                                                                                                                                                                                                                                                                                                |
| :---------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Default flag fitur      | Sesi yang [tidak mengambil flag fitur dari Anthropic](/docs/id/env-vars#features-that-need-feature-flag-fetching), seperti sesi di penyedia cloud atau dengan telemetri dimatikan, menggunakan default flag yang dibangun ke dalam versi yang diinstal. Ketika rilis mengubah salah satu default tersebut, perilaku berubah untuk pengembang tersebut segera setelah mereka upgrade               | Pin versi itu sendiri, `requiredMaximumVersion` atau `DISABLE_UPDATES`                                                                                                                                                                                                                                                                                               |
| Asumsi kemampuan model  | ID model yang tidak dikenali versi yang diinstal, seperti alias gateway `prod-opus`, berjalan pada asumsi default untuk [adaptive reasoning](/docs/id/model-config#adaptive-reasoning-and-fixed-thinking-budgets), parameter effort, dan [jendela konteks](/docs/id/model-config#correct-the-window-for-a-gateway-or-custom-model-id) hingga versi yang lebih baru mengenali ID atau Anda memetakannya | Rute ID model Anthropic di gateway, atau tambahkan entri [`modelOverrides`](/docs/id/model-config#override-model-ids-per-version) yang memetakan ID model Anthropic ke alias Anda. Pada koneksi penyedia cloud, Anda dapat sebagai gantinya [mendeklarasikan kemampuan model yang di-pin](/docs/id/model-config#customize-pinned-model-display-and-capabilities)               |
| Model default dan alias | Model yang dimulai sesi baru secara default, dan model yang alias seperti `opus` dan `sonnet` selesaikan, [dibangun ke dalam setiap versi](/docs/id/model-config#pin-models-for-third-party-deployments) dan dapat berubah ketika pengembang upgrade                                                                                                                                              | [`ANTHROPIC_DEFAULT_MODEL`](/docs/id/model-config#set-a-default-model-for-new-sessions) untuk model yang dimulai sesi baru, dan [variabel `ANTHROPIC_DEFAULT_*_MODEL`](/docs/id/model-config#environment-variables), seperti `ANTHROPIC_DEFAULT_OPUS_MODEL`, untuk apa yang setiap alias selesaikan. `ANTHROPIC_DEFAULT_MODEL` memerlukan Claude Code v2.1.236 atau lebih baru |

<h2 id="related-resources">
  Sumber daya terkait
</h2>

* [Hubungkan Claude Code ke gateway LLM](/docs/id/llm-gateway-connect): langkah pengaturan yang menghadap pengembang, dengan konfigurasi per-permukaan dan tabel pemecahan masalah yang dapat Anda berikan kepada pengembang
* [Panduan kompatibilitas gateway](/docs/id/llm-gateway-protocol): referensi untuk operator gateway, mencakup endpoint, header untuk diteruskan, dan tabel pass-through fitur
* [Nilai mana yang digunakan Claude Code](/docs/id/settings#which-value-claude-code-uses): bagaimana pengaturan terkelola, proyek, dan pengguna digabungkan
* [Mekanisme pengiriman](/docs/id/managed-settings#delivery-mechanisms): di mana file terkelola berada di setiap platform
* [Siapkan Claude Code untuk organisasi Anda](/docs/id/admin-setup): peluncuran yang lebih luas yang merupakan bagian dari gateway ini, termasuk penegakan kebijakan, visibilitas penggunaan, dan penanganan data
