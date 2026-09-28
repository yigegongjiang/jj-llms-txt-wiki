> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Hubungkan Claude Code ke gateway LLM

> Arahkan Claude Code ke gateway LLM organisasi Anda. Periksa apakah admin Anda sudah mengonfigurasinya, atau atur URL dasar dan kredensial sendiri, kemudian verifikasi koneksi dan perbaiki kesalahan gateway.

[Gateway LLM](/docs/id/llm-gateway) adalah proxy yang dijalankan organisasi Anda antara Claude Code dan penyedia model. Ketika organisasi Anda menggunakan satu, Claude Code melakukan autentikasi ke gateway dengan kredensial yang dikeluarkan organisasi Anda, bukan login claude.ai pribadi Anda.

Halaman ini untuk pengembang yang menjalankan Claude Code melalui gateway yang dioperasikan organisasi mereka. Ini mencakup dua jalur: [memeriksa apakah administrator Anda sudah mengonfigurasinya untuk Anda](#check-for-an-existing-configuration), dan [mengonfigurasinya sendiri](#configure-claude-code-yourself) ketika mereka belum.

<Note>
  * Untuk menerapkan gateway untuk organisasi Anda, lihat [Luncurkan gateway LLM](/docs/id/llm-gateway-rollout)
  * Untuk apa yang Claude Code kirim ke gateway, lihat [referensi protokol gateway](/docs/id/llm-gateway-protocol)
</Note>

<h2 id="check-for-an-existing-configuration">
  Periksa konfigurasi yang ada
</h2>

Administrator dapat mendistribusikan alamat gateway dan kredensial melalui [pengaturan terkelola](/docs/id/managed-settings), manajemen perangkat, atau [`apiKeyHelper`](#rotate-credentials-with-apikeyhelper), sehingga Claude Code mengambilnya saat startup tanpa apa pun yang perlu Anda atur. Untuk memeriksa apakah organisasi Anda sudah melakukan ini:

<Steps>
  <Step title="Mulai Claude Code">
    Jalankan `claude`. Jika membuka ke layar login alih-alih sesi, tidak ada kredensial gateway yang didistribusikan; [konfigurasikan sendiri](#configure-claude-code-yourself) di bawah.
  </Step>

  <Step title="Periksa tab Status">
    Jika Claude Code memulai sesi tanpa menampilkan layar login, jalankan `/status`, yang membuka tab **Status**, dan periksa dua baris:

    * `Anthropic base URL`: baris ini hanya muncul ketika alamat gateway diatur. Jika tidak ada, Claude Code tidak menunjuk ke gateway; [konfigurasikan sendiri](#configure-claude-code-yourself) di bawah.
    * `Auth token` atau `API key`: baris yang menamai `ANTHROPIC_AUTH_TOKEN`, `ANTHROPIC_API_KEY`, atau `apiKeyHelper` mengonfirmasi kredensial gateway aktif. Baris `Login method` yang menamai akun claude.ai sebagai gantinya berarti kredensial tidak didistribusikan; [aturnya sendiri](#set-the-credential-variable).
  </Step>

  <Step title="Kirim pesan uji">
    Tutup menu `/status` dan kirim prompt apa pun di Claude Code. Respons normal dari Claude, tanpa kesalahan, mengonfirmasi koneksi gateway berfungsi.
  </Step>
</Steps>

Jika kedua baris di menu `/status` terlihat benar tetapi pesan ke Claude gagal, lihat [tabel pemecahan masalah](#troubleshoot-gateway-errors).

<h2 id="configure-claude-code-yourself">
  Konfigurasikan Claude Code sendiri
</h2>

Untuk mengonfigurasi Claude Code untuk gateway sendiri, Anda memerlukan dari tim gateway Anda:

* URL dasar gateway
* Kredensial: string kunci atau token, atau perintah yang mengambilnya
  * Jika tim gateway Anda tidak mengatakan jenis kredensial apa itu, bagian [variabel kredensial](#set-the-credential-variable) di bawah mencakup apa yang harus dicoba

Bagian di bawah mencakup konfigurasi secara berurutan:

* [Atur variabel kredensial](#set-the-credential-variable) dan [atur URL dasar](#set-the-base-url-and-credential): dua variabel yang setiap koneksi gateway butuhkan
* [Verifikasi koneksi](#verify-the-connection): konfirmasi berfungsi sebelum menyimpan apa pun
* [Konfigurasikan setiap permukaan](#configure-each-surface): jika Anda menggunakan permukaan selain CLI Claude Code, seperti VS Code, lihat cara mengonfigurasinya dengan kredensial gateway Anda
* [Konfigurasi tambahan](#additional-configuration): variabel yang beberapa gateway butuhkan di luar URL dasar dan kredensial, seperti header khusus, pembantu kredensial, penemuan model, URL dasar format penyedia, atau mematikan lalu lintas di luar jalur gateway. Atur ini hanya jika administrator Anda menamakannya atau jaringan Anda membatasi egress

<h3 id="set-the-credential-variable">
  Atur variabel kredensial
</h3>

Untuk melakukan autentikasi Claude Code ke gateway, atur kredensial Anda dalam variabel lingkungan. Variabel mana tergantung pada apa yang diberitahu tim gateway Anda:

| Atur kredensial di                                      | Gunakan ketika                                                         |
| :------------------------------------------------------ | :--------------------------------------------------------------------- |
| `ANTHROPIC_AUTH_TOKEN`                                  | Tim gateway Anda mengatakan "bearer token" atau "Authorization header" |
| `ANTHROPIC_API_KEY`                                     | Tim gateway Anda mengatakan "API key" atau "x-api-key"                 |
| [`apiKeyHelper`](#rotate-credentials-with-apikeyhelper) | Kredensial berputar atau berasal dari vault                            |

Jika Anda tidak diberitahu jenis mana, gunakan `ANTHROPIC_AUTH_TOKEN`; [permintaan verifikasi](#verify-the-connection) di bawah menunjukkan cara mengetahui apakah Anda perlu beralih.

<h3 id="set-the-base-url-and-credential">
  Atur URL dasar dan kredensial
</h3>

Atur URL dasar gateway dan variabel kredensial yang Anda pilih di atas sebagai variabel lingkungan. Contoh menggunakan `ANTHROPIC_AUTH_TOKEN`; tukarnya dengan `ANTHROPIC_API_KEY` jika itu [variabel yang Anda pilih](#set-the-credential-variable). Anda dapat mengaturnya [di shell Anda](#set-as-shell-environment-variables), yang berlaku untuk satu sesi terminal, atau [di file pengaturan Claude Code](#set-in-a-settings-file), yang bertahan di mana pun Claude Code berjalan.

Untuk koneksi pertama Anda, mulai dengan ekspor shell dan jalankan [permintaan verifikasi](#verify-the-connection) sebelum memindahkan nilai ke file pengaturan.

<h4 id="set-as-shell-environment-variables">
  Atur sebagai variabel lingkungan shell
</h4>

Ganti nilai dengan yang diberikan tim gateway Anda:

<Tabs>
  <Tab title="Bash atau Zsh">
    ```bash theme={null}
    export ANTHROPIC_BASE_URL=https://llm-gateway.example.com
    export ANTHROPIC_AUTH_TOKEN=sk-gateway-key
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    $env:ANTHROPIC_BASE_URL = "https://llm-gateway.example.com"
    $env:ANTHROPIC_AUTH_TOKEN = "sk-gateway-key"
    ```
  </Tab>
</Tabs>

Ekspor shell hanya berlaku untuk sesi terminal itu dan program yang dimulai darinya. Editor yang diluncurkan dari dock atau menu Start tidak akan melihatnya. Untuk membuatnya bertahan di terminal baru, tambahkan baris yang sama ke profil shell Anda, seperti `~/.zshrc`, `~/.bashrc`, atau `$PROFILE` PowerShell Anda.

Jika Anda mengekspor gateway hanya di shell Anda, itu tidak dapat diandalkan mencapai agen latar belakang yang dihosting oleh [supervisor](/docs/id/agent-view#how-background-sessions-are-hosted); lihat [bagaimana setiap sesi latar belakang bersumber gatewaynya](/docs/id/agent-view#llm-gateway). Gunakan file pengaturan untuk gateway apa pun yang harus selalu dirutekan oleh agen latar belakang.

<h4 id="set-in-a-settings-file">
  Atur di file pengaturan
</h4>

Untuk membuat konfigurasi berlaku di mana pun Claude Code berjalan, termasuk [agen latar belakang](/docs/id/agent-view#how-background-sessions-are-hosted), atur variabel di blok `env` dari [file pengaturan](/docs/id/settings) daripada mengandalkan shell Anda. File pengaturan memiliki cakupan berbeda:

* `~/.claude/settings.json` berlaku untuk semua proyek Anda. Di Windows jalurnya adalah `%USERPROFILE%\.claude\settings.json`
* `.claude/settings.local.json` berlaku untuk satu proyek. Claude Code menambahkannya ke gitignore global Anda ketika menyimpan pengaturan di sana; jika Anda membuatnya dengan tangan atau membiarkan Claude menulisnya, tambahkan ke gitignore Anda sendiri terlebih dahulu sehingga Anda tidak secara tidak sengaja melakukan komit kredensial Anda

<Warning>
  Jangan letakkan kredensial di `.claude/settings.json` proyek. File itu dilakukan komit dan dibagikan dengan semua orang yang mengkloning repositori.
</Warning>

Blok `env` terlihat sama di kedua file:

```json theme={null}
{
  "env": {
    "ANTHROPIC_BASE_URL": "https://llm-gateway.example.com",
    "ANTHROPIC_AUTH_TOKEN": "sk-gateway-key"
  }
}
```

Ketika ekspor shell dan blok `env` file pengaturan mengatur variabel yang sama, nilai file pengaturan berlaku. Jalankan `/status` untuk melihat URL dasar dan sumber kredensial mana yang digunakan Claude Code.

<h3 id="verify-the-connection">
  Verifikasi koneksi
</h3>

Dengan variabel yang diekspor di shell Anda, kirim permintaan satu token ke gateway secara langsung. Ini mengonfirmasi URL dan kredensial berfungsi sebelum Anda membuka Claude Code, sehingga kegagalan menunjuk ke gateway daripada konfigurasi Anda. Perintah di bawah membaca variabel shell, jadi mereka memerlukan [ekspor shell](#set-as-shell-environment-variables) bahkan jika Anda juga menempatkan nilai di file pengaturan.

<Tabs>
  <Tab title="Bash atau Zsh">
    ```bash theme={null}
    curl -X POST "$ANTHROPIC_BASE_URL/v1/messages" \
      -H "Authorization: Bearer $ANTHROPIC_AUTH_TOKEN" \
      -H "anthropic-version: 2023-06-01" \
      -H "content-type: application/json" \
      -d '{"model": "claude-sonnet-4-6", "max_tokens": 1, "messages": [{"role": "user", "content": "."}]}'
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    Invoke-RestMethod -Method Post -Uri "$env:ANTHROPIC_BASE_URL/v1/messages" `
      -Headers @{ "Authorization" = "Bearer $env:ANTHROPIC_AUTH_TOKEN"; "anthropic-version" = "2023-06-01" } `
      -ContentType "application/json" `
      -Body '{"model": "claude-sonnet-4-6", "max_tokens": 1, "messages": [{"role": "user", "content": "."}]}'
    ```
  </Tab>
</Tabs>

Jika gateway Anda mengharapkan kunci di header `x-api-key`, ganti header `Authorization` dengan `x-api-key: $ANTHROPIC_API_KEY` di perintah Bash, atau entri hashtable `"Authorization"` dengan `"x-api-key" = "$env:ANTHROPIC_API_KEY"` di perintah PowerShell.

Respons JSON yang dimulai dengan `{"id":"msg_` dan menyertakan bidang `"content":[...]` berarti gateway dapat dijangkau dan kredensial berfungsi. Kesalahan yang menamai model yang tidak dikenal masih membuktikan URL dan kredensial berfungsi, karena gateway melakukan autentikasi permintaan sebelum menolak nama model; Anda tidak perlu menemukan model yang dilayani gateway Anda untuk tes ini. `401` berarti kredensial ditolak: jika Anda menebak variabel, beralih ke yang lain dan ekspor ulang.

<h4 id="confirm-in-claude-code">
  Konfirmasi di Claude Code
</h4>

Mulai `claude` dari shell yang sama sehingga mewarisi ekspor, kirim pesan, dan jalankan `/status`.

Di tab **Status**, baris `Anthropic base URL` harus menampilkan alamat gateway Anda, yang mengonfirmasi permintaan dirutekan ke sana; jika baris tidak ada, variabel tidak mencapai sesi. Baris `Auth token` atau `API key` yang menamai variabel yang Anda atur mengonfirmasi kredensial gateway aktif daripada login claude.ai yang disimpan.

Jika pesan gagal, atau `/status` tidak menampilkan URL gateway, lihat [tabel pemecahan masalah](#troubleshoot-gateway-errors) di bawah.

<h3 id="how-the-credential-variable-maps-to-a-header">
  Bagaimana variabel kredensial memetakan ke header
</h3>

Setiap variabel mengirim kredensial di header HTTP berbeda: `ANTHROPIC_AUTH_TOKEN` di `Authorization: Bearer`, `ANTHROPIC_API_KEY` di `x-api-key`, dan `apiKeyHelper` di keduanya. Kredensial dalam variabel yang salah mencapai gateway di header yang tidak dibacanya, dan permintaan gagal dengan `401`. Jika permintaan verifikasi mengembalikan `401`, beralih ke variabel lain dan coba lagi.

<h3 id="conflicts-with-an-existing-login">
  Konflik dengan login yang ada
</h3>

Variabel kredensial gateway mengambil alih login claude.ai yang disimpan atau kunci Console. Login claude.ai Anda tetap disimpan dan tidak digunakan sementara variabel diatur; batalkan pengaturan variabel dan Claude Code kembali ke itu. Dengan `ANTHROPIC_AUTH_TOKEN`, variabel mengambil alih segera. Dengan `ANTHROPIC_API_KEY`, Anda diminta sekali dalam mode interaktif untuk menyetujui kunci sebelum mengambil alih.

Jalankan `/status` untuk mengonfirmasi sumber kredensial mana yang aktif. Jika startup menampilkan peringatan konflik auth yang menamai dua sumber, lihat baris pertama [tabel pemecahan masalah](#troubleshoot-gateway-errors) untuk mengetahui mana yang harus dihapus. Untuk menghapus login yang disimpan sehingga hanya kredensial gateway yang tersisa, jalankan `/logout`.

<h2 id="configure-each-surface">
  Konfigurasikan setiap permukaan
</h2>

CLI membaca variabel lingkungan dan file pengaturan di atas. Permukaan lainnya adalah ekstensi VS Code, aplikasi desktop, GitHub Actions, Agent SDK, dan permukaan cloud seperti Slack dan web; bagian di bawah mencakup apakah pengaturan itu mencapai masing-masing.

<h3 id="vs-code-extension">
  Ekstensi VS Code
</h3>

Atur variabel gateway untuk [ekstensi VS Code](/docs/id/vs-code) di `claudeCode.environmentVariables`, di pengaturan pengguna VS Code sendiri yang dibuka dengan perintah **Preferences: Open User Settings (JSON)**. Ekstensi memeriksa kredensial dari pengaturan ini sebelum meluncurkan, jadi itu tempat yang andal untuk kredensial gateway; nilai di `~/.claude/settings.json` mencapai proses yang dihasilkan tetapi bukan pemeriksaan login ekstensi sendiri.

```json theme={null}
{
  "claudeCode.environmentVariables": [
    { "name": "ANTHROPIC_BASE_URL", "value": "https://llm-gateway.example.com" },
    { "name": "ANTHROPIC_AUTH_TOKEN", "value": "sk-gateway-key" }
  ]
}
```

<h3 id="desktop-app">
  Aplikasi desktop
</h3>

Aplikasi desktop membaca perutean gateway dari [konfigurasi inferensi pihak ketiga](https://claude.com/docs/third-party/claude-desktop/gateway), bukan dari `ANTHROPIC_BASE_URL` atau `settings.json`. Konfigurasi itu dapat berasal dari organisasi Anda atau dari formulir di aplikasi itu sendiri:

* **Didistribusikan oleh administrator**: jika organisasi Anda telah [menerapkan konfigurasi](/docs/id/llm-gateway-rollout#distribute-through-managed-settings), aplikasi desktop merutekan melalui gateway tanpa pengaturan di pihak Anda
* **Dikonfigurasi secara lokal**: untuk perangkat tanpa konfigurasi yang didistribusikan administrator, buka Help → Troubleshooting → Enable Developer Mode, yang memulai ulang aplikasi dengan menu Developer. Kemudian buka Developer → Configure Third-Party Inference dan masukkan URL dasar gateway Anda. Konfigurasi yang didistribusikan administrator memiliki prioritas dan membuat formulir ini hanya-baca

Dengan konfigurasi gateway aktif, aplikasi desktop menjalankan sesi hanya di mesin lokal Anda: pemilih lingkungan tidak menawarkan sesi SSH atau lingkungan cloud yang dihosting Anthropic, dan [Remote Control](/docs/id/remote-control) tidak tersedia. Untuk menggunakan Claude Code pada host jarak jauh melalui gateway, jalankan CLI pada host itu dengan [`ANTHROPIC_BASE_URL` dan kredensial gateway](#set-the-base-url-and-credential) diatur di sana.

Jika aplikasi desktop menampilkan `Gateway was unreachable`, aplikasi tidak dapat menjangkau URL dasar yang dikonfigurasi saat startup; periksa URL dan jalur jaringan dengan [tes curl di atas](#verify-the-connection).

<h3 id="github-actions">
  GitHub Actions
</h3>

[Claude Code GitHub Actions](/docs/id/github-actions) membaca `ANTHROPIC_BASE_URL` dan `ANTHROPIC_CUSTOM_HEADERS` dari blok `env` alur kerja. Teruskan kredensial sebagai input `anthropic_api_key` tindakan; tindakan mengaturnya sebagai `ANTHROPIC_API_KEY`, sehingga mencapai gateway di header `x-api-key`.

Untuk gateway `x-api-key`, atur URL dasar di `env` dan teruskan kunci gateway sebagai input:

```yaml theme={null}
env:
  ANTHROPIC_BASE_URL: https://llm-gateway.example.com

steps:
  - uses: anthropics/claude-code-action@v1
    with:
      anthropic_api_key: ${{ secrets.GATEWAY_API_KEY }}
```

Untuk gateway bearer-token, teruskan rahasia yang sama dua kali: sebagai input `anthropic_api_key` dan sebagai `ANTHROPIC_AUTH_TOKEN` di blok `env` alur kerja. Tindakan memerlukan `anthropic_api_key`, `CLAUDE_CODE_OAUTH_TOKEN`, atau federasi identitas beban kerja sebelum meluncurkan Claude Code, dan tidak membaca `ANTHROPIC_AUTH_TOKEN`, jadi input hanya ada untuk memenuhi pemeriksaan peluncuran itu. Variabel env adalah yang menempatkan kunci di header `Authorization` yang dibaca gateway; salinan di `x-api-key` diabaikan:

```yaml theme={null}
env:
  ANTHROPIC_BASE_URL: https://llm-gateway.example.com
  ANTHROPIC_AUTH_TOKEN: ${{ secrets.GATEWAY_API_KEY }}

steps:
  - uses: anthropics/claude-code-action@v1
    with:
      anthropic_api_key: ${{ secrets.GATEWAY_API_KEY }}
```

Untuk opsi autentikasi lain tindakan, termasuk `CLAUDE_CODE_OAUTH_TOKEN` dan federasi identitas beban kerja, lihat [Claude Code GitHub Actions](/docs/id/github-actions) dan [README](https://github.com/anthropics/claude-code-action#readme) tindakan.

<h3 id="agent-sdk">
  Agent SDK
</h3>

[Agent SDK](/docs/id/agent-sdk/overview) tidak memiliki opsi khusus gateway; itu melewatkan variabel lingkungan ke proses Claude Code yang dihasilkannya. Setiap SDK menerima opsi `env` yang mengatur lingkungan proses yang dihasilkan, dan SDK TypeScript dan Python memperlakukannya berbeda:

* TypeScript: proses yang dihasilkan mewarisi lingkungan induk secara default, tetapi pengaturan `options.env` mengganti lingkungan sepenuhnya. Sebarkan `process.env` ke dalamnya untuk menyimpan variabel gateway Anda.
* Python: `ClaudeAgentOptions(env=...)` menggabungkan di atas lingkungan yang diwarisi, jadi variabel gateway yang diatur dalam proses induk membawa tanpa penyebaran.

<CodeGroup>
  ```ts TypeScript theme={null}
  const result = query({
    prompt: "...",
    options: {
      env: {
        ...process.env,
        ANTHROPIC_BASE_URL: "https://llm-gateway.example.com",
        ANTHROPIC_AUTH_TOKEN: process.env.GATEWAY_KEY,
      },
    },
  })
  ```

  ```python Python theme={null}
  options = ClaudeAgentOptions(
      env={
          "ANTHROPIC_BASE_URL": "https://llm-gateway.example.com",
          "ANTHROPIC_AUTH_TOKEN": os.environ["GATEWAY_KEY"],
      }
  )
  ```
</CodeGroup>

<h3 id="slack-cloud-sessions-and-remote-control">
  Slack, sesi cloud, dan Remote Control
</h3>

[Claude Code di Slack](/docs/id/slack) dan [sesi cloud](/docs/id/claude-code-on-the-web) selalu menggunakan API Anthropic; mereka bukan bagian dari penerapan gateway. Variabel gateway yang diatur dalam konfigurasi lingkungan sesi cloud tidak diterapkan. Jika lalu lintas Anda harus tetap di gateway, jangan aktifkan permukaan ini untuk pengguna tersebut.

[Remote Control](/docs/id/remote-control) dan [voice dictation](/docs/id/voice-dictation) keduanya mengandalkan identitas claude.ai: Remote Control untuk memasangkan sesi langsung dengan akun Anda, dan voice dictation untuk menjangkau titik akhir transkripsi claude.ai. Mereka tidak tersedia sementara `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN`, atau `apiKeyHelper` aktif. Remote Control juga dinonaktifkan sementara `ANTHROPIC_BASE_URL` menunjuk ke host non-Anthropic, jadi masuk dengan claude.ai saja tidak cukup. Sebelum v2.1.196, URL dasar non-Anthropic tidak memblokir Remote Control.

Untuk mengembalikan salah satu fitur, masuk dengan claude.ai dan batalkan pengaturan variabel gateway yang diperiksa fitur tersebut. Bagian Remote Control dari `claude doctor` menamai apa yang saat ini memblokir Remote Control.

* Voice dictation: batalkan pengaturan kredensial gateway
* Remote Control: batalkan pengaturan kredensial gateway dan `ANTHROPIC_BASE_URL`

<h2 id="additional-configuration">
  Konfigurasi tambahan
</h2>

Pengaturan ini mencakup kasus di luar URL dasar dan kredensial. Atur hanya jika instruksi administrator Anda, aturan egress jaringan Anda, atau [tabel pemecahan masalah](#troubleshoot-gateway-errors) memanggil satu.

<h3 id="send-additional-headers">
  Kirim header tambahan
</h3>

Beberapa gateway merutekan atau menandai permintaan menggunakan header khusus selain kredensial, misalnya pengidentifikasi penyewa atau kunci perutean. Untuk mengirim satu, atur [`ANTHROPIC_CUSTOM_HEADERS`](/docs/id/env-vars) dengan satu pasangan `Name: Value` per baris. Contoh di bawah menambahkan header perutean bernama `X-Org-Route`:

<Tabs>
  <Tab title="Bash atau Zsh">
    ```bash theme={null}
    export ANTHROPIC_CUSTOM_HEADERS="X-Org-Route: prod"
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    $env:ANTHROPIC_CUSTOM_HEADERS = "X-Org-Route: prod"
    ```
  </Tab>
</Tabs>

Anda juga dapat mengatur `ANTHROPIC_CUSTOM_HEADERS` di blok `env` file pengaturan. Gunakan `\n` antara pasangan di sana, karena string JSON tidak dapat mencakup beberapa baris:

```json theme={null}
{
  "env": {
    "ANTHROPIC_CUSTOM_HEADERS": "X-Org-Route: prod\nX-Tenant: example"
  }
}
```

Header perutean dan penyewa seperti ini dihitung sebagai [header yang memerlukan persetujuan](/docs/id/server-managed-settings#environment-variables-and-the-approval-dialog). Ketika header berasal dari file pengaturan proyek, Claude Code menerapkannya di bawah [aturan untuk ketika menerapkan nilai `env`](/docs/id/settings-reference#when-claude-code-applies-env-values).

<h3 id="add-gateway-models-to-the-model-picker">
  Tambahkan model gateway ke pemilih model
</h3>

Dengan penemuan model diaktifkan, Claude Code menanyakan gateway untuk daftar modelnya saat startup dan menambahkan nama tersebut ke pemilih `/model` bersama entri bawaan. Jika Anda atau administrator Anda mengatur `replaceBuiltInOptions` dalam lineup [`modelPicker`](/docs/id/settings-reference#modelpicker), Claude Code juga menyembunyikan nama yang ditemukan. Ini menyimpan baris untuk model yang sudah digunakan sesi.

Aktifkan jika gateway Anda melayani nama model yang tidak ada dalam daftar bawaan Claude Code dan Anda ingin memilihnya dari pemilih. Jika model bawaan adalah apa yang Anda gunakan, Anda tidak memerlukan penemuan; administrator Anda mungkin juga telah mengaktifkannya melalui pengaturan terkelola.

Untuk mengaktifkannya, atur `CLAUDE_CODE_ENABLE_GATEWAY_MODEL_DISCOVERY=1` di shell Anda atau di blok `env` dari `~/.claude/settings.json`.

Model yang ditemukan muncul sebagai entri `/model` tambahan. Setiap entri menampilkan deskripsi yang disediakan gateway Anda untuk model, atau `From gateway` ketika tidak menyediakannya.

Untuk mengonfirmasi penemuan berjalan, mulai `claude --debug` dan cari baris `[gatewayDiscovery]` dalam log debug di `~/.claude/debug/<session-id>.txt`. Pertama kali penemuan berhasil, Claude Code mencatat berapa banyak model yang di-cache, dan hanya mencatat lagi ketika daftar gateway berubah. `404`, timeout, atau pengalihan juga muncul di sana. Untuk kapan penemuan berjalan, apa yang disaringnya, dan format respons yang dilayani gateway, lihat [referensi penemuan model](/docs/id/llm-gateway-protocol#model-discovery).

<h3 id="rotate-credentials-with-apikeyhelper">
  Putar kredensial dengan apiKeyHelper
</h3>

`apiKeyHelper` adalah perintah yang dijalankan Claude Code untuk mengambil kredensial gateway Anda, alih-alih membacanya dari variabel lingkungan statis.

Gunakan pembantu ketika kredensial kedaluwarsa sesuai jadwal, berasal dari vault atau perintah SSO, atau administrator Anda mengatakan untuk mengonfigurasi satu. Jika kredensial Anda adalah string tetap yang Anda atur sekali, [variabel kredensial](#set-the-credential-variable) adalah semua yang Anda butuhkan dan Anda dapat melewati bagian ini.

Pembantu adalah perintah shell apa pun yang mencetak kredensial saat ini ke stdout. Claude Code menjalankannya melalui shell sistem Anda, jadi di Windows itu dapat berupa executable atau invokasi PowerShell. Pastikan perintah tersebut hanya mencetak kredensial, tanpa output lain. Pada Claude Code v2.1.227 atau lebih baru, banner atau baris log yang dicetak bersama kunci membuat [pembantu gagal](/docs/id/errors#your-apikeyhelper-script-is-failing). Tulis skrip, buat dapat dieksekusi, dan referensikan dari `apiKeyHelper` di [file pengaturan](/docs/id/settings) Anda:

<Tabs>
  <Tab title="Bash atau Zsh">
    Misalnya, skrip yang membaca dari vault:

    ```bash theme={null}
    #!/bin/bash
    vault kv get -field=api_key secret/llm-gateway/claude-code
    ```

    Referensikan jalurnya di `~/.claude/settings.json`:

    ```json theme={null}
    {
      "apiKeyHelper": "~/bin/get-gateway-key.sh"
    }
    ```
  </Tab>

  <Tab title="PowerShell">
    Misalnya, skrip yang membaca dari vault:

    ```powershell theme={null}
    vault kv get -field=api_key secret/llm-gateway/claude-code
    ```

    Referensikan invokasi PowerShell di `%USERPROFILE%\.claude\settings.json`, meloloskan garis miring terbalik dalam string JSON:

    ```json theme={null}
    {
      "apiKeyHelper": "powershell -NoProfile -File C:\\scripts\\get-gateway-key.ps1"
    }
    ```
  </Tab>
</Tabs>

Claude Code menyimpan output pembantu selama lima menit secara default dan menjalankan kembali pembantu setelah masa pakai cache berakhir. Untuk mengubah masa pakai, atur `CLAUDE_CODE_API_KEY_HELPER_TTL_MS` dalam milidetik, misalnya `CLAUDE_CODE_API_KEY_HELPER_TTL_MS=900000` untuk 15 menit.

Lihat [`apiKeyHelper`](/docs/id/settings-reference#apikeyhelper) untuk kasus lain di mana Claude Code menjalankan kembali pembantu.

Nilai pembantu dikirim di header `Authorization` dan `x-api-key`, jadi berfungsi di mana pun header gateway Anda dibaca.

<h3 id="turn-off-traffic-outside-the-gateway-path">
  Matikan lalu lintas di luar jalur gateway
</h3>

Gateway membawa permintaan model, tetapi Claude Code juga mengirim lalu lintas latar belakang yang tidak penting di luar jalur gateway, ke Anthropic dan ke layanan pihak ketiga seperti GitHub: pemeriksaan versi, telemetri, catatan rilis, dan permintaan serupa. Di jaringan yang hanya memungkinkan egress ke gateway, permintaan ini gagal dan dapat muncul sebagai koneksi yang diblokir dalam pemantauan egress Anda.

Claude Code melampirkan kredensial ke permintaan telemetri atau metrik penggunaan hanya ketika permintaan pergi ke host yang kredensial miliknya. Sementara `ANTHROPIC_BASE_URL` menunjuk ke gateway, Claude Code mengirim acara telemetrinya ke Anthropic tanpa kredensial gateway Anda. Dengan [variabel kredensial](#set-the-credential-variable) atau `apiKeyHelper` juga aktif, Claude Code tidak melaporkan metrik penggunaan ke [dasbor analitik](/docs/id/analytics#access-analytics-for-api-customers) Konsol. Sebelum v2.1.246, Claude Code dapat melampirkan kredensial gateway ke permintaan telemetri dan metrik penggunaan yang ditujukan untuk host Anthropic; permintaan model selalu pergi ke gateway dengan kredensial yang diharapkan gateway.

Untuk mematikan lalu lintas itu, atur `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC=1` bersama variabel gateway, di ekspor shell yang sama atau blok `env` file pengaturan:

<Tabs>
  <Tab title="Bash atau Zsh">
    ```bash theme={null}
    export CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC=1
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    $env:CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC = "1"
    ```
  </Tab>
</Tabs>

Mengatur variabel memiliki efek dan batasan ini:

* Ini menonaktifkan pembaruan otomatis, jadi rencanakan jalur pembaruan lain, seperti pengelola paket Anda atau distribusi terkelola.
* Ini menekan pemeriksaan ketersediaan [mode cepat](/docs/id/fast-mode). Kecuali pemeriksaan sebelumnya telah mengaktifkan mode cepat di mesin, `/fast` melaporkan bahwa mode cepat tidak tersedia.
* Ini tidak mempengaruhi [penemuan model gateway](#add-gateway-models-to-the-model-picker), yang menanyakan hanya gateway Anda. Sebelum v2.1.257, variabel juga menghentikan penemuan dari penyegaran, jadi pemilih menyimpan daftar yang sebelumnya di-cache.
* [Pemeriksaan keamanan domain](/docs/id/data-usage#webfetch-domain-safety-check) alat WebFetch tidak terpengaruh dan masih memanggil `api.anthropic.com`. Matikan secara terpisah dengan `skipWebFetchPreflight: true` di [pengaturan](/docs/id/settings) jika jaringan Anda memblokir host itu.
* Untuk setiap aliran telemetri dan variabel yang mengontrolnya, lihat [layanan telemetri](/docs/id/data-usage#telemetry-services).

<h3 id="route-to-a-cloud-provider-through-a-gateway">
  Rutekan ke penyedia cloud melalui gateway
</h3>

Konfigurasi ini menunjukkan Claude Code ke gateway melalui variabel URL dasar khusus penyedia sebagai pengganti `ANTHROPIC_BASE_URL`. Gateway Amazon Bedrock dan Google Cloud's Agent Platform menerima format permintaan asli penyedia tersebut; gateway Microsoft Foundry dan Claude Platform di AWS menerima format Anthropic Messages. Pada rute Amazon Bedrock dan Google Cloud's Agent Platform, Claude Code juga membatasi header beta dan bidang permintaan yang dikirimnya ke set yang diterima penyedia tersebut. Untuk apa yang diterima gateway Anda di setiap rute, lihat [panduan kompatibilitas gateway](/docs/id/llm-gateway-protocol).

Gunakan satu hanya jika tim gateway Anda secara khusus menamai Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry, atau Claude Platform di AWS. Jika [permintaan verifikasi](#verify-the-connection) di atas mengembalikan JSON, Anda dapat melewati bagian ini.

Atur blok untuk penyedia yang dinamai tim gateway Anda. Variabel skip-auth di blok Amazon Bedrock, Google Cloud's Agent Platform, dan Claude Platform di AWS memberitahu Claude Code untuk tidak menandatangani permintaan dengan kredensial penyedia cloud, karena gateway menyimpannya. Jika gateway juga memerlukan token sendiri, tempat Anda meletakkannya tergantung pada penyedia:

* **Amazon Bedrock, Google Cloud's Agent Platform, atau Claude Platform di AWS**: tambahkan `ANTHROPIC_AUTH_TOKEN` setelah blok. Claude Code mengirimnya ke gateway sebagai header `Authorization: Bearer`. Untuk kredensial dalam skema atau header yang berbeda, gunakan [`ANTHROPIC_CUSTOM_HEADERS`](#send-additional-headers) sebagai gantinya. Jaga variabel skip-auth tetap diatur baik cara apa pun, karena tanpanya Claude Code menghapus header `Authorization` apa pun yang akan ditambahkan oleh `ANTHROPIC_AUTH_TOKEN`, [`apiKeyHelper`](#rotate-credentials-with-apikeyhelper), atau `ANTHROPIC_CUSTOM_HEADERS`.
* **Microsoft Foundry**: gunakan `ANTHROPIC_FOUNDRY_API_KEY` seperti yang ditunjukkan [bloknya](#microsoft-foundry)

<h4 id="amazon-bedrock">
  Amazon Bedrock
</h4>

Biarkan `AWS_BEARER_TOKEN_BEDROCK` tidak diatur ketika gateway mengeluarkan kredensialnya sendiri. Jika Anda mengaturnya, Claude Code mengirim [kunci API Amazon Bedrock](/docs/id/amazon-bedrock#2-configure-aws-credentials) itu sebagai header `Authorization` alih-alih token gateway Anda, bahkan dengan `CLAUDE_CODE_SKIP_BEDROCK_AUTH` diatur.

<Tabs>
  <Tab title="Bash atau Zsh">
    ```bash theme={null}
    export ANTHROPIC_BEDROCK_BASE_URL=https://llm-gateway.example.com/bedrock
    export CLAUDE_CODE_SKIP_BEDROCK_AUTH=1
    export CLAUDE_CODE_USE_BEDROCK=1
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    $env:ANTHROPIC_BEDROCK_BASE_URL = "https://llm-gateway.example.com/bedrock"
    $env:CLAUDE_CODE_SKIP_BEDROCK_AUTH = "1"
    $env:CLAUDE_CODE_USE_BEDROCK = "1"
    ```
  </Tab>
</Tabs>

<h4 id="google-cloud’s-agent-platform">
  Google Cloud's Agent Platform
</h4>

Ganti ID proyek dan wilayah dengan nilai Anda sendiri. Claude Code menyertakan keduanya dalam jalur setiap permintaan yang dikirimnya ke gateway:

<Tabs>
  <Tab title="Bash atau Zsh">
    ```bash theme={null}
    export ANTHROPIC_VERTEX_BASE_URL=https://llm-gateway.example.com/vertex
    export ANTHROPIC_VERTEX_PROJECT_ID=your-gcp-project-id
    export CLAUDE_CODE_SKIP_VERTEX_AUTH=1
    export CLAUDE_CODE_USE_VERTEX=1
    export CLOUD_ML_REGION=us-east5
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    $env:ANTHROPIC_VERTEX_BASE_URL = "https://llm-gateway.example.com/vertex"
    $env:ANTHROPIC_VERTEX_PROJECT_ID = "your-gcp-project-id"
    $env:CLAUDE_CODE_SKIP_VERTEX_AUTH = "1"
    $env:CLAUDE_CODE_USE_VERTEX = "1"
    $env:CLOUD_ML_REGION = "us-east5"
    ```
  </Tab>
</Tabs>

Blok ini mencakup perutean dan autentikasi. Penggantian wilayah dan pin model dari [pengaturan Agent Platform](/docs/id/google-vertex-ai#4-configure-claude-code) berlaku melalui gateway juga:

* **Wilayah per-model**: jika gateway Anda melayani beberapa model dari wilayah selain `CLOUD_ML_REGION`, atur variabel `VERTEX_REGION_CLAUDE_*` yang cocok untuk masing-masing, misalnya `VERTEX_REGION_CLAUDE_4_6_SONNET=europe-west1`. [Referensi variabel lingkungan](/docs/id/env-vars) mencantumkan nama yang tepat.
* **Versi model**: pin `ANTHROPIC_DEFAULT_OPUS_MODEL`, `ANTHROPIC_DEFAULT_SONNET_MODEL`, dan `ANTHROPIC_DEFAULT_HAIKU_MODEL` seperti dalam [Pin model versions](/docs/id/google-vertex-ai#5-pin-model-versions). Mengatur `ANTHROPIC_DEFAULT_HAIKU_MODEL` juga memindahkan tugas latar belakang seperti judul sesi ke model itu, dan bagian itu menjelaskan model mana yang menjalankannya sebaliknya.
* **Kemampuan model**: jika Anda pin ID model yang versi Claude Code Anda tidak kenal, fitur seperti tingkat upaya atau pemikiran yang diperluas dapat tetap dinonaktifkan di dalamnya. Deklarasikan apa yang didukung model dengan [`ANTHROPIC_DEFAULT_OPUS_MODEL_SUPPORTED_CAPABILITIES`](/docs/id/model-config#customize-pinned-model-display-and-capabilities) dan rekan Sonnet dan Haiku-nya.

<h4 id="microsoft-foundry">
  Microsoft Foundry
</h4>

Letakkan kredensial gateway di `ANTHROPIC_FOUNDRY_API_KEY`; itu dikirim ke gateway sebagai header `x-api-key`. Gateway yang mengharapkan token bearer dapat mengambil [`ANTHROPIC_FOUNDRY_AUTH_TOKEN`](/docs/id/env-vars) sebagai gantinya. Claude Code mengirim nilai itu sebagai header `Authorization: Bearer`, dan itu mengambil prioritas atas `ANTHROPIC_FOUNDRY_API_KEY` ketika keduanya diatur. Memerlukan Claude Code v2.1.203 atau lebih baru.

Untuk gateway yang menyuntikkan header `Authorization` miliknya sendiri, atur `CLAUDE_CODE_SKIP_FOUNDRY_AUTH=1` dan biarkan kedua variabel kredensial tidak diatur. Claude Code kemudian mengirim permintaan tanpa kredensial Azure dan mempertahankan header `Authorization` yang Anda suplai, misalnya melalui `ANTHROPIC_CUSTOM_HEADERS`. Sebelum v2.1.203, `CLAUDE_CODE_SKIP_FOUNDRY_AUTH` tanpa kunci API membuat klien Microsoft Foundry tidak dapat mengirim permintaan.

<Tabs>
  <Tab title="Bash atau Zsh">
    ```bash theme={null}
    export ANTHROPIC_FOUNDRY_BASE_URL=https://llm-gateway.example.com/foundry
    export ANTHROPIC_FOUNDRY_API_KEY=sk-gateway-key
    export CLAUDE_CODE_USE_FOUNDRY=1
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    $env:ANTHROPIC_FOUNDRY_BASE_URL = "https://llm-gateway.example.com/foundry"
    $env:ANTHROPIC_FOUNDRY_API_KEY = "sk-gateway-key"
    $env:CLAUDE_CODE_USE_FOUNDRY = "1"
    ```
  </Tab>
</Tabs>

<h4 id="claude-platform-on-aws">
  Claude Platform di AWS
</h4>

Lihat [Claude Platform di AWS](/docs/id/claude-platform-on-aws) untuk ID ruang kerja.

<Tabs>
  <Tab title="Bash atau Zsh">
    ```bash theme={null}
    export ANTHROPIC_AWS_BASE_URL=https://llm-gateway.example.com/anthropic-aws
    export ANTHROPIC_AWS_WORKSPACE_ID=wrkspc_01ABCDEFGHIJKLMN
    export CLAUDE_CODE_SKIP_ANTHROPIC_AWS_AUTH=1
    export CLAUDE_CODE_USE_ANTHROPIC_AWS=1
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    $env:ANTHROPIC_AWS_BASE_URL = "https://llm-gateway.example.com/anthropic-aws"
    $env:ANTHROPIC_AWS_WORKSPACE_ID = "wrkspc_01ABCDEFGHIJKLMN"
    $env:CLAUDE_CODE_SKIP_ANTHROPIC_AWS_AUTH = "1"
    $env:CLAUDE_CODE_USE_ANTHROPIC_AWS = "1"
    ```
  </Tab>
</Tabs>

<h4 id="confirm-the-provider-route">
  Konfirmasi rute penyedia
</h4>

Mulai `claude` dari shell tempat Anda mengatur blok dan jalankan `/status`. Dengan blok Amazon Bedrock, tab **Status** menampilkan baris seperti ini:

```text theme={null}
API provider: Amazon Bedrock
Bedrock base URL: https://llm-gateway.example.com/bedrock
AWS auth skipped
```

Blok lainnya menghasilkan baris yang sama di bawah nama penyedia mereka, misalnya `Vertex base URL` dan `GCP auth skipped` untuk Google Cloud's Agent Platform; blok Microsoft Foundry menampilkan baris auth skipped hanya jika Anda mengatur `CLAUDE_CODE_SKIP_FOUNDRY_AUTH`. Jika Anda juga merutekan melalui proxy korporat, baris `Proxy` menampilkan URL proxy. Jika baris URL dasar hilang, variabel tidak mencapai sesi.

<h2 id="troubleshoot-gateway-errors">
  Pemecahan masalah kesalahan gateway
</h2>

Ini adalah kesalahan paling umum saat menjalankan Claude Code melalui gateway, dengan penyebab sisi gateway dan perbaikannya:

| Kesalahan                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 | Penyebab                                                                                                                                                                                                                                                                                                                                                                                                                                                    | Perbaikan                                                                                                                                                                                                                                                                                                                                                                                                              |
| :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Peringatan startup yang menamai dua sumber kredensial dan berakhir dengan `auth may not work as expected`. Versi lebih lama menampilkan `Auth conflict: Both a token (SOURCE) and an API key (SOURCE) are set` sebagai gantinya.                                                                                                                                                                                                                                                                          | Kredensial gateway dan login yang disimpan keduanya aktif; variabel digunakan untuk permintaan, tetapi login basi dapat menyebabkan perilaku auth yang tidak terduga                                                                                                                                                                                                                                                                                        | Batalkan pengaturan variabel untuk menggunakan login yang disimpan, atau jalankan `/logout` untuk menggunakan kredensial gateway                                                                                                                                                                                                                                                                                       |
| Kesalahan `401` yang menamai token yang tidak valid atau tidak dikenali                                                                                                                                                                                                                                                                                                                                                                                                                                   | Kredensial bukan yang dikeluarkan gateway, atau berada di header yang tidak dibaca gateway                                                                                                                                                                                                                                                                                                                                                                  | Konfirmasi variabel cocok dengan jenis kredensial Anda di [tabel kredensial](#set-the-credential-variable), dan regenerasi kunci di gateway jika itu dicabut                                                                                                                                                                                                                                                           |
| `Your apiKeyHelper script is failing`, atau `apiKeyHelper failed:` pada stderr dalam mode non-interaktif                                                                                                                                                                                                                                                                                                                                                                                                  | Perintah dalam pengaturan [`apiKeyHelper`](/docs/id/settings-reference#apikeyhelper) tidak menghasilkan kunci yang dapat digunakan, sehingga permintaan membawa kunci placeholder                                                                                                                                                                                                                                                                                | Jalankan perintah secara langsung untuk melihat mengapa itu gagal, dan autentikasi ulang dengan penyedia kredensial Anda jika itu melaporkan sesi yang kedaluwarsa; lihat [referensi kesalahan](/docs/id/errors#your-apikeyhelper-script-is-failing)                                                                                                                                                                        |
| `Connection refused — a firewall or proxy may be blocking it (ConnectionRefused)` ketika tidak ada yang menjawab di alamat, atau `Can't reach the API server — check your internet or DNS (ENOTFOUND)` ketika hostname tidak terselesaikan, sering setelah jeda senyap sementara Claude Code [mencoba ulang dengan backoff](/docs/id/errors#automatic-retries). Kode dalam tanda kurung bervariasi; [Unable to connect to API](/docs/id/errors#unable-to-connect-to-api) mencakup ejaan kode dan wording sebelumnya | Tidak ada yang menjawab di URL dasar: alamatnya salah, atau VPN atau firewall memblokir jalur ke gateway                                                                                                                                                                                                                                                                                                                                                    | Jalankan [tes curl di atas](#verify-the-connection), yang gagal segera dengan penyebab yang sama, dan konfirmasi URL dan jalur jaringan dengan tim gateway Anda                                                                                                                                                                                                                                                        |
| `API returned an empty or malformed response (HTTP 200)`                                                                                                                                                                                                                                                                                                                                                                                                                                                  | Gateway atau proxy perantara mengembalikan respons non-API, sering halaman HTML error atau login                                                                                                                                                                                                                                                                                                                                                            | Uji dengan [permintaan curl di atas](#verify-the-connection); perbaiki rute gateway yang menjawab dengan sesuatu selain respons Claude API. [Referensi kesalahan](/docs/id/errors#api-returned-an-empty-or-malformed-response) menjelaskan detail yang dilaporkan pesan                                                                                                                                                     |
| Kesalahan `400` yang menamai `context_management`, `Extra inputs are not permitted`, atau bidang lain yang tidak dikenali                                                                                                                                                                                                                                                                                                                                                                                 | Gateway meneruskan permintaan ke upstream yang menolak bidang yang dikirim Claude Code ke titik akhir format Anthropic                                                                                                                                                                                                                                                                                                                                      | Atur `CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1`, yang menekan sebagian besar bidang pra-rilis; lihat [feature pass-through](/docs/id/llm-gateway-protocol#feature-pass-through). Beberapa beta tidak dijaga oleh bendera ini; untuk itu, atur variabel penyedia `CLAUDE_CODE_USE_*` yang cocok sehingga Claude Code hanya mengirim apa yang diterima penyedia itu                                                           |
| Kesalahan `400` yang menamai `thinking` atau `adaptive`, seperti `Input tag 'adaptive' found`                                                                                                                                                                                                                                                                                                                                                                                                             | Build model upstream tidak menerima adaptive reasoning, yang diminta Claude Code untuk model Claude 4.6 dan lebih baru                                                                                                                                                                                                                                                                                                                                      | Tingkatkan upstream gateway. Di Opus 4.6 dan Sonnet 4.6, `CLAUDE_CODE_DISABLE_ADAPTIVE_THINKING=1` berfungsi sebagai gantinya. Variabel kemampuan [konfigurasi model](/docs/id/model-config) hanya berlaku untuk konfigurasi penyedia, seperti `CLAUDE_CODE_USE_BEDROCK` dan `CLAUDE_CODE_USE_VERTEX`, bukan di belakang gateway `ANTHROPIC_BASE_URL`                                                                       |
| Kesalahan `400` yang menyatakan batas konteks atau token dalam kata-kata gateway sendiri, seperti `ContextWindowExceededError` atau `prompt token count of N exceeds the limit of M`                                                                                                                                                                                                                                                                                                                      | Gateway memberlakukan konteks yang lebih kecil dari jendela asli model dan menulis ulang kesalahan upstream, jadi Claude Code tidak mengenalinya sebagai [kesalahan terlalu panjang](/docs/id/errors#prompt-is-too-long) dan tidak compact dan retry secara otomatis                                                                                                                                                                                             | Jalankan `/compact` untuk memulihkan sesi. Untuk mencegahnya, atur `CLAUDE_CODE_AUTO_COMPACT_WINDOW` ke batas gateway; Claude Code mengklem nilai ke setidaknya 100.000 token dan paling banyak jendela konteks model, jadi Anda tidak dapat mencocokkan batas gateway di bawah 100.000, dan `/compact` tetap menjadi pemulihan di sana. Juga atur `CLAUDE_CODE_MAX_OUTPUT_TOKENS` di bawah batas output model gateway |
| Kesalahan `400` pada setiap permintaan, dalam kata-kata gateway sendiri menolak skema input alat atau `pattern`-nya, pada Claude Code v2.1.265 hingga v2.1.267                                                                                                                                                                                                                                                                                                                                            | Dalam peluncuran bertahap pada versi tersebut, skema [alat Artifact](/docs/id/artifacts#availability) membawa ekspresi reguler dengan kelas karakter Unicode `\p{...}`. API Anthropic menerimanya, tetapi gateway atau upstream yang memeriksa `pattern` skema setiap alat dengan mesin regex-nya sendiri menolak seluruh permintaan                                                                                                                             | Perbarui ke v2.1.268 atau lebih baru, yang tidak mengirim ekspresi reguler. Pada versi yang terpengaruh, [matikan artifacts](/docs/id/artifacts#disable-artifacts), yang menghapus alat dan skemanya dari permintaan                                                                                                                                                                                                        |
| Kesalahan `400` pada setiap permintaan, dalam kata-kata gateway sendiri menolak jenis alat yang tidak dikenali, seperti `Input tag 'advisor_20260301'`, pada Claude Code v2.1.275                                                                                                                                                                                                                                                                                                                         | Dalam peluncuran bertahap pada versi itu, permintaan membawa entri [alat advisor](/docs/id/advisor) bahkan dengan advisor dimatikan. API Anthropic menerimanya, tetapi gateway atau upstream yang memvalidasi jenis alat menolak seluruh permintaan; satu yang [meneruskan bidang badan permintaan tanpa perubahan](/docs/id/llm-gateway-protocol#forward-as-open-lists) meneruskannya tanpa terpengaruh. Entri adalah deklarasi yang tidak membawa konten percakapan | Perbarui ke v2.1.276 atau lebih baru, yang tidak mengirim entri di belakang gateway `ANTHROPIC_BASE_URL` kecuali Anda menghidupkan advisor. Pada v2.1.275, atur [`CLAUDE_CODE_DISABLE_ADVISOR_TOOL=1`](/docs/id/env-vars), yang menghapus entri dari permintaan                                                                                                                                                             |
| Model hilang dari pemilih `/model`                                                                                                                                                                                                                                                                                                                                                                                                                                                                        | Nama model gateway tidak ada dalam daftar bawaan Claude Code, atau Claude Code menampilkan lineup [`modelPicker`](/docs/id/settings-reference#modelpicker) yang menggantikan opsi bawaan                                                                                                                                                                                                                                                                         | Aktifkan [penemuan model gateway](#add-gateway-models-to-the-model-picker) atau tambahkan nama dengan variabel [konfigurasi model](/docs/id/model-config). Jika Claude Code menampilkan lineup `modelPicker` yang menggantikan, tambahkan model gateway ke dalamnya, atau minta administrator Anda untuk menambahkannya ketika pengaturan terkelola menyediakannya                                                          |
| `/fast` melaporkan `Fast mode unavailable due to network connectivity issues` sementara permintaan inference bekerja                                                                                                                                                                                                                                                                                                                                                                                      | Pemeriksaan ketersediaan [fast mode](/docs/id/fast-mode) berjalan langsung ke `api.anthropic.com` dan tidak mengikuti `ANTHROPIC_BASE_URL`, jadi egress langsung yang diblokir gagal pemeriksaan. Pesan yang sama muncul di jaringan terbuka ketika pemeriksaan menyajikan kunci yang dikeluarkan gateway dari `ANTHROPIC_API_KEY` atau `apiKeyHelper` dan Anthropic menolaknya                                                                                  | Daftar putih `api.anthropic.com` jika egress diblokir, atau atur variabel skip; untuk kunci gateway yang ditolak hanya variabel skip yang membantu. Lihat [gunakan fast mode di belakang proxy dan LLM gateway](/docs/id/fast-mode#use-fast-mode-behind-proxies-and-llm-gateways)                                                                                                                                           |
| `/fast` melaporkan `Fast mode has been disabled by your organization` dalam sesi yang diautentikasi dengan `ANTHROPIC_AUTH_TOKEN`, meskipun organisasi memiliki fast mode yang diaktifkan                                                                                                                                                                                                                                                                                                                 | Pemeriksaan ketersediaan memerlukan login claude.ai atau kunci API Anthropic; dengan hanya token bearer, Claude Code memperlakukan fast mode sebagai dinonaktifkan tanpa mengirim pemeriksaan                                                                                                                                                                                                                                                               | Atur `CLAUDE_CODE_SKIP_FAST_MODE_ORG_CHECK=1`; lihat [gunakan fast mode di belakang proxy dan LLM gateway](/docs/id/fast-mode#use-fast-mode-behind-proxies-and-llm-gateways)                                                                                                                                                                                                                                                |
| Claude Code meminta Anda untuk masuk meskipun [tes curl](#verify-the-connection) berhasil                                                                                                                                                                                                                                                                                                                                                                                                                 | CLI tidak memiliki kredensial sendiri: URL dasar yang dapat dijangkau bukan satu, dan dalam sesi interaktif blok `env` di `.claude/settings.json` atau `.claude/settings.local.json` proyek hanya berlaku setelah wizard first-run dan [prompt kepercayaan](/docs/id/permissions#what-runs-before-you-trust-a-folder)                                                                                                                                            | Atur `ANTHROPIC_AUTH_TOKEN` di mana pun Claude Code membaca sebelum pengaturan first-run: ekspor shell, blok `env` di `~/.claude/settings.json`, atau pengaturan terkelola                                                                                                                                                                                                                                             |
| `ANTHROPIC_API_KEY` diatur tetapi diabaikan, tanpa prompt                                                                                                                                                                                                                                                                                                                                                                                                                                                 | Kunci memerlukan persetujuan satu kali dalam sesi interaktif, dan kunci yang sebelumnya ditolak diabaikan tanpa bertanya lagi                                                                                                                                                                                                                                                                                                                               | Aktifkannya di bawah `/config` dengan opsi `Use custom API key`                                                                                                                                                                                                                                                                                                                                                        |
| `This machine's managed settings require a first-party login`                                                                                                                                                                                                                                                                                                                                                                                                                                             | Pengaturan terkelola menyertakan `forceLoginMethod` atau `forceLoginOrgUUID`, yang tidak dapat hidup berdampingan dengan `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN`, atau `apiKeyHelper`                                                                                                                                                                                                                                                                   | Administrator Anda harus menghapus `forceLoginMethod` dan `forceLoginOrgUUID` dari pengaturan terkelola untuk menggunakan kredensial gateway, atau menghapus kredensial gateway untuk menggunakan login pihak pertama. Keduanya tidak dapat digabungkan                                                                                                                                                                |
| `403` dengan badan HTML seperti `403 Forbidden`, ketika log gateway sendiri menunjukkan tidak ada permintaan yang diterima                                                                                                                                                                                                                                                                                                                                                                                | Firewall aplikasi web atau proxy terbalik di depan gateway memblokir badan permintaan sebelum mencapai gateway. Prompt Claude Code menyertakan tag gaya XML dan kode sumber yang cocok dengan aturan badan cross-site-scripting, jadi tes curl pendek lulus sementara sesi nyata tidak                                                                                                                                                                      | Bebaskan jalur `/v1/messages` gateway dari inspeksi badan permintaan. Di AWS WAF ini adalah aturan terkelola `CrossSiteScripting_Body`; di nginx dengan ModSecurity itu adalah aturan badan OWASP CRS yang setara                                                                                                                                                                                                      |
| Kesalahan sertifikat atau TLS seperti `SSL certificate verification failed` atau `Self-signed certificate detected`, ketika [tes curl](#verify-the-connection) berhasil                                                                                                                                                                                                                                                                                                                                   | Runtime Claude Code tidak mempercayai otoritas sertifikat yang sama dengan yang digunakan `curl`. Umum di belakang proxy inspeksi TLS korporat                                                                                                                                                                                                                                                                                                              | Atur `NODE_EXTRA_CA_CERTS` ke jalur bundel CA; lihat [CA certificate store](/docs/id/network-config#ca-certificate-store)                                                                                                                                                                                                                                                                                                   |

Jika Claude Code meminta Anda untuk masuk berulang kali setelah menghapus konfigurasi gateway, penyebabnya biasanya penyimpanan kredensial daripada gateway; lihat [kesalahan autentikasi](/docs/id/errors#authentication-errors).

<h2 id="related-resources">
  Sumber daya terkait
</h2>

* [Ikhtisar gateway LLM](/docs/id/llm-gateway): apa itu gateway dan bagaimana berinteraksi dengan langganan claude.ai
* [Luncurkan gateway LLM untuk organisasi Anda](/docs/id/llm-gateway-rollout): daftar periksa yang menghadap admin untuk menerapkan dan mendistribusikan konfigurasi gateway
* [Panduan kompatibilitas gateway](/docs/id/llm-gateway-protocol): apa yang dikirim Claude Code ke gateway, termasuk header dan bidang yang harus diteruskan gateway
* [Pengaturan](/docs/id/settings): di mana file pengaturan berada dan bagaimana blok `env` dibaca
* [Autentikasi](/docs/id/authentication): bagaimana variabel kredensial, `apiKeyHelper`, dan login OAuth berinteraksi
