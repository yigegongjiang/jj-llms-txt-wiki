> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Kontrol akses server MCP untuk organisasi Anda

> Batasi server MCP mana yang dapat ditambahkan atau dihubungkan pengguna dengan file konfigurasi yang dikelola, daftar izin, dan daftar penolakan.

Secara default, siapa pun yang menjalankan Claude Code dapat menghubungkan [server MCP](/docs/id/mcp) apa pun yang mereka pilih. Anthropic meninjau konektor terhadap [kriteria pendaftarannya](https://claude.com/docs/connectors/building/review-criteria) sebelum menambahkannya ke [Direktori Anthropic](https://claude.ai/directory), tetapi tidak melakukan audit keamanan atau mengelola server MCP apa pun. Sebagai administrator, Anda dapat membatasi server mana yang berjalan di organisasi Anda, mulai dari menerapkan set yang disetujui tetap hingga menonaktifkan MCP sepenuhnya.

Halaman ini mencakup cara untuk:

* [Memilih pola](#choose-a-pattern) yang sesuai dengan tingkat kontrol yang Anda butuhkan
* [Menerapkan set server tetap dengan `managed-mcp.json`](#exclusive-control-with-managed-mcp-json), termasuk cara [menonaktifkan MCP sepenuhnya](#disable-mcp-entirely)
* [Mengontrol server dengan daftar izin dan daftar penolakan](#policy-based-control-with-allowlists-and-denylists)
* [Memberitahu pengguna apa yang diharapkan](#how-restrictions-appear-to-users) ketika pembatasan memblokir server
* [Memantau server mana yang benar-benar digunakan organisasi Anda](#monitor-mcp-usage)

<Note>
  Halaman [Security](/docs/id/security) mencakup model ancaman MCP dan cara mengevaluasi server sebelum menyetujuinya. [Tentukan apa yang akan diterapkan](/docs/id/admin-setup#decide-what-to-enforce) mencakup pembatasan MCP bersama dengan kontrol administratif lainnya.
</Note>

<h2 id="choose-a-pattern">
  Pilih pola
</h2>

Claude Code mendukung berbagai tingkat pembatasan. Setiap pola menggunakan satu atau kedua mekanisme yang dibahas di bawah: `managed-mcp.json` untuk menerapkan set tetap, dan `allowedMcpServers`/`deniedMcpServers` untuk memfilter apa yang dikonfigurasi pengguna.

| Pola                       | Apa yang dilakukan                                                                                       | Konfigurasi                                                                                             |
| :------------------------- | :------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------ |
| **Nonaktifkan MCP**        | Tidak ada server yang dimuat di mana pun                                                                 | `managed-mcp.json` dengan peta server kosong                                                            |
| **Penerapan tetap**        | Setiap pengguna mendapatkan server yang sama dan tidak dapat menambah yang lain                          | `managed-mcp.json` dengan server yang Anda inginkan                                                     |
| **Katalog yang disetujui** | Publikasikan daftar server yang disetujui; pengguna menambahkan yang mereka inginkan, yang lain diblokir | `allowedMcpServers` + `allowManagedMcpServersOnly: true`                                                |
| **Server plugin saja**     | Server hanya dapat berasal dari plugin; pengguna tidak dapat menambah milik mereka sendiri               | [`strictPluginOnlyCustomization`](/docs/id/settings#strictpluginonlycustomization) dengan `mcp` dalam daftar |
| **Daftar izin lunak**      | Terapkan daftar izin yang dapat diperluas pengguna dalam pengaturan mereka sendiri                       | `allowedMcpServers` tanpa `allowManagedMcpServersOnly`                                                  |
| **Hanya daftar penolakan** | Blokir server yang diketahui buruk, izinkan yang lain                                                    | `deniedMcpServers`                                                                                      |
| **Tanpa pembatasan**       | Pengguna menambahkan apa pun                                                                             | Jangan terapkan konfigurasi MCP yang dikelola                                                           |

<Note>
  Claude Code tidak memiliki registri server MCP bawaan yang dapat ditelusuri dan diinstal pengguna. Untuk pola katalog yang disetujui, bagikan daftar yang disetujui dan perintah `claude mcp add` di tempat yang akan ditemukan pengguna Anda, seperti wiki internal, atau distribusikan server sebagai plugin melalui [marketplace plugin yang dikelola](/docs/id/plugin-marketplaces#managed-marketplace-restrictions) sehingga pengguna dapat menelusuri dan menginstalnya dari `/plugin`.
</Note>

<h2 id="exclusive-control-with-managed-mcp-json">
  Kontrol eksklusif dengan managed-mcp.json
</h2>

Jika Anda menerapkan file `managed-mcp.json`, Claude Code hanya memuat server yang didefinisikan file tersebut. Pengguna tidak dapat menambah, memodifikasi, atau menggunakan server MCP lain apa pun, termasuk server yang disediakan plugin. File juga menekan konektor claude.ai kecuali Anda [mengizinkannya bersama set yang dikelola](#allow-claude-ai-connectors-alongside-the-managed-set).

Dua pengaturan lainnya dapat memfilter set yang dikelola lebih lanjut:

* `allowedMcpServers` dan `deniedMcpServers` juga berlaku untuk server yang dikelola, jadi server yang dikelola yang tidak melewati mereka tidak akan dimuat.
* `deniedMcpServers` pengguna sendiri digabungkan dari pengaturan mereka, jadi pengguna dapat memblokir server yang dikelola untuk diri mereka sendiri.

Lihat [Bagaimana server dievaluasi](#how-a-server-is-evaluated) untuk urutan pemeriksaan lengkap.

`managed-mcp.json` adalah file mandiri, jadi tidak dapat dikirimkan melalui [pengaturan yang dikelola server](/docs/id/server-managed-settings). Proses apa pun yang dapat menulis ke jalur sistem dengan hak istimewa administrator dapat menerapkannya. Dalam skala besar, itu biasanya melalui alat manajemen perangkat, seperti Jamf atau profil konfigurasi di macOS, Group Policy atau Intune di Windows, atau manajemen armada pilihan Anda di Linux. Claude Code mencari file di salah satu jalur berikut:

| Platform      | Jalur                                                      |
| :------------ | :--------------------------------------------------------- |
| macOS         | `/Library/Application Support/ClaudeCode/managed-mcp.json` |
| Linux dan WSL | `/etc/claude-code/managed-mcp.json`                        |
| Windows       | `C:\Program Files\ClaudeCode\managed-mcp.json`             |

File menggunakan format yang sama dengan file proyek [`.mcp.json`](/docs/id/mcp#project-scope):

```json theme={null}
{
  "mcpServers": {
    "github": {
      "type": "http",
      "url": "https://api.githubcopilot.com/mcp/"
    },
    "sentry": {
      "type": "http",
      "url": "https://mcp.sentry.dev/mcp"
    },
    "company-internal": {
      "type": "stdio",
      "command": "/usr/local/bin/company-mcp-server",
      "args": ["--config", "/etc/company/mcp-config.json"],
      "env": {
        "COMPANY_API_URL": "https://internal.example.com"
      }
    }
  }
}
```

<h3 id="authenticate-with-per-user-credentials">
  Autentikasi dengan kredensial per pengguna
</h3>

Pengguna apa pun di mesin dapat membaca file ini, jadi jangan simpan kunci API atau kredensial lain dalam blok `env`. Teruskan kredensial per pengguna dengan salah satu dari ini:

* [Ekspansi `${VAR}`](/docs/id/mcp#environment-variable-expansion-in-mcp-json) untuk membaca rahasia dari lingkungan setiap pengguna.
* [OAuth atau header per pengguna](/docs/id/mcp#authenticate-with-remote-mcp-servers) sehingga setiap pengguna mengautentikasi sebagai diri mereka sendiri.
* [`headersHelper`](/docs/id/mcp#use-dynamic-headers-for-custom-authentication) untuk menghasilkan kredensial pada waktu koneksi.

<h3 id="validate-the-configuration">
  Validasi konfigurasi
</h3>

Untuk mengonfirmasi file berlaku, jalankan dua pemeriksaan pada mesin yang dikelola:

1. `claude mcp list` menampilkan hanya server di `managed-mcp.json`. Jika server pengguna sendiri masih muncul, file tidak dibaca; periksa jalur dan izin.
2. `claude mcp add --transport http test https://example.com/mcp` gagal dengan `Cannot add MCP server: enterprise MCP configuration is active and has exclusive control over MCP servers`. URL tidak perlu menjadi server nyata, karena pemeriksaan kebijakan menolak perintah sebelum apa pun dihubungi.

<h3 id="disable-mcp-entirely">
  Nonaktifkan MCP sepenuhnya
</h3>

Terapkan `managed-mcp.json` yang berisi peta server kosong untuk memblokir setiap server MCP:

```json theme={null}
{
  "mcpServers": {}
}
```

Pengguna tidak melihat server MCP apa pun di `/mcp`, dan `claude mcp add` gagal dengan kesalahan kebijakan perusahaan di atas. Server yang dikonfigurasi pengguna sebelumnya berhenti dimuat saat mereka memulai sesi berikutnya, tanpa peringatan bahwa kebijakan adalah alasannya.

<h3 id="allow-claude-ai-connectors-alongside-the-managed-set">
  Izinkan konektor claude.ai bersama set yang dikelola
</h3>

Menerapkan `managed-mcp.json` menekan [konektor claude.ai](/docs/id/mcp#use-mcp-servers-from-claude-ai) secara default, termasuk konektor yang dikonfigurasi administrator untuk organisasi di konsol admin claude.ai. Untuk memuat konektor tersebut bersama server di `managed-mcp.json`, atur `"allowAllClaudeAiMcps": true` dalam [sumber pengaturan yang dikelola](/docs/id/admin-setup#decide-how-settings-reach-devices). Memerlukan Claude Code v2.1.149 atau lebih baru.

Dengan pengaturan diaktifkan, Claude Code memuat konektor claude.ai yang sama yang akan dimuat jika `managed-mcp.json` tidak diterapkan. [Daftar izin dan daftar penolakan](#policy-based-control-with-allowlists-and-denylists) masih berlaku untuk konektor tersebut, jadi Anda dapat memblokir yang spesifik dengan `deniedMcpServers`. Pengaturan hanya mempengaruhi konektor claude.ai; server yang disediakan plugin tetap ditekan.

Claude Code membaca pengaturan ini hanya dari tingkat kebijakan yang dikendalikan admin: pengaturan yang dikelola server, kunci plist yang diterapkan MDM atau kunci registri HKLM, atau file `managed-settings.json` sistem. Menempatkannya dalam pengaturan pengguna atau proyek tidak berpengaruh, jadi pengguna tidak dapat mengaktifkan kembali konektor yang kontrol eksklusif tekan.

<h2 id="policy-based-control-with-allowlists-and-denylists">
  Kontrol berbasis kebijakan dengan daftar izin dan daftar penolakan
</h2>

Daftar izin dan daftar penolakan memfilter server mana yang dikonfigurasi yang diizinkan untuk dimuat. Mereka bukan registri: server masih harus ditambahkan oleh pengguna, plugin, atau `managed-mcp.json` sebelum daftar izin atau daftar penolakan berlaku padanya. Untuk menerapkan server kepada pengguna, gunakan [`managed-mcp.json`](#exclusive-control-with-managed-mcp-json). Kedua daftar juga memfilter server yang dilewatkan dengan bendera CLI [`--mcp-config`](/docs/id/cli-reference#cli-flags); `--strict-mcp-config` membatasi file konfigurasi mana yang dimuat dan tidak melewati salah satu daftar.

Untuk membuat daftar izin berwenang, atur `allowedMcpServers` dan `allowManagedMcpServersOnly: true` bersama-sama dalam [sumber pengaturan yang dikelola](/docs/id/admin-setup#decide-how-settings-reach-devices), seperti pengaturan yang dikelola server atau file `managed-settings.json` yang diterapkan. [Batasi daftar izin ke pengaturan yang dikelola saja](#restrict-the-allowlist-to-managed-settings-only) menunjukkan konfigurasi. Tanpa `allowManagedMcpServersOnly`, daftar izin dari setiap sumber pengaturan digabungkan, termasuk `~/.claude/settings.json` pengguna sendiri, jadi pengguna dapat memperluas apa yang diizinkan daftar izin Anda. Daftar penolakan digabungkan dari setiap sumber terlepas dari itu.

<Note>
  `allowManagedMcpServersOnly` terpisah dari `allowManagedPermissionRulesOnly`, yang mengunci [aturan izin](/docs/id/permissions#managed-settings) saja. Menetapkan bendera itu tidak menerapkan daftar izin MCP.
</Note>

<h3 id="match-servers-by-url-command-or-name">
  Cocokkan server berdasarkan URL, perintah, atau nama
</h3>

`allowedMcpServers` dan `deniedMcpServers` adalah daftar entri. Setiap entri adalah objek dengan satu kunci yang mengidentifikasi server berdasarkan URL, perintah, atau nama mereka:

| Kunci           | Cocok                                                                          | Gunakan untuk                                  |
| :-------------- | :----------------------------------------------------------------------------- | :--------------------------------------------- |
| `serverUrl`     | URL server jarak jauh, tepat atau dengan wildcard `*`                          | Server HTTP dan SSE                            |
| `serverCommand` | Perintah dan argumen yang tepat yang memulai server stdio                      | Server stdio                                   |
| `serverName`    | Label yang ditetapkan pengguna. Kecocokan tepat saja; wildcard tidak diperluas | Tipe apa pun, tetapi lihat Peringatan di bawah |

Membiarkan `allowedMcpServers` tidak diatur berbeda dari menetapkannya ke array kosong:

| Pengaturan          | Tidak diatur (default)         | Array kosong `[]`               | Diisi                             |
| :------------------ | :----------------------------- | :------------------------------ | :-------------------------------- |
| `allowedMcpServers` | Semua server diizinkan         | Tidak ada server yang diizinkan | Hanya server yang cocok diizinkan |
| `deniedMcpServers`  | Tidak ada server yang diblokir | Tidak ada server yang diblokir  | Server yang cocok diblokir        |

Lihat [Entri tidak valid dalam pengaturan yang dikelola](/docs/id/settings#invalid-entries-in-managed-settings) untuk mengetahui apa yang terjadi ketika entri gagal validasi skema.

<Warning>
  Entri `serverName`, dalam daftar apa pun, bukan kontrol keamanan. Nama adalah label yang ditetapkan pengguna saat menjalankan `claude mcp add` atau mengedit file konfigurasi, bukan server yang mendasar, jadi pengguna dapat memanggil server apa pun `github`. Untuk konektor claude.ai, nama adalah nama tampilan yang dikembalikan oleh claude.ai, yang dapat berubah. Untuk menerapkan server mana yang benar-benar berjalan, tambahkan entri `serverCommand` atau `serverUrl`.
</Warning>

Validasi `serverName` berbeda antara dua daftar:

* Dalam `deniedMcpServers`, `serverName` menerima string non-kosong apa pun, jadi Anda dapat memblokir [konektor claude.ai](/docs/id/mcp#use-mcp-servers-from-claude-ai) berdasarkan nama tampilan mereka. Misalnya, `{ "serverName": "claude.ai Slack" }` memblokir konektor Slack. Lebih suka entri `serverUrl` ketika Anda memerlukan penolakan yang kuat terhadap penggantian nama, atau ketika nama konektor bertabrakan dan mendapatkan akhiran ` (N)`.
* Dalam `allowedMcpServers`, `serverName` terbatas pada huruf, angka, tanda hubung, dan garis bawah. Gunakan `serverUrl` untuk daftar izin konektor claude.ai.

Untuk mematikan semua konektor claude.ai, lihat [`disableClaudeAiConnectors`](/docs/id/mcp#disable-claude-ai-connectors).

<h3 id="how-a-server-is-evaluated">
  Bagaimana server dievaluasi
</h3>

Sebelum memuat server, termasuk yang dari `managed-mcp.json`, Claude Code menjalankan tiga pemeriksaan secara berurutan:

1. **Gabungkan daftarnya.** Entri daftar izin dan daftar penolakan dari setiap sumber pengaturan digabungkan menjadi satu daftar izin dan satu daftar penolakan. Ketika `allowManagedMcpServersOnly` adalah `true`, hanya daftar izin yang dikelola yang disimpan; daftar penolakan selalu digabungkan dari setiap sumber.
2. **Periksa daftar penolakan.** Server yang cocok dengan entri daftar penolakan apa pun, berdasarkan URL, perintah, atau nama, diblokir. Tidak ada yang mengganti kecocokan daftar penolakan.
3. **Periksa daftar izin.** Jika `allowedMcpServers` tidak diatur di mana pun, setiap server yang melewati daftar penolakan dimuat. Jika diatur, apa yang harus cocok dengan server tergantung pada jenisnya, ditunjukkan dalam tabel di bawah.

| Jenis server               | Diizinkan ketika cocok                                                                                             |
| :------------------------- | :----------------------------------------------------------------------------------------------------------------- |
| Jarak jauh (HTTP atau SSE) | Entri `serverUrl`. Kecocokan `serverName` hanya dihitung ketika daftar izin tidak berisi entri `serverUrl`         |
| Stdio                      | Entri `serverCommand`. Kecocokan `serverName` hanya dihitung ketika daftar izin tidak berisi entri `serverCommand` |

Tiga aturan pencocokan berlaku dalam pemeriksaan tersebut:

* **Perintah cocok dengan tepat.** Setiap argumen, dalam urutan. `["npx", "-y", "server"]` tidak cocok dengan `["npx", "server"]` atau `["npx", "-y", "server", "--flag"]`.
* **Nilai `serverCommand` dan `serverUrl` diperluas sebelum pencocokan.** Baik entri kebijakan maupun nilai yang dikonfigurasi server melalui ekspansi [`${VAR}` dan `${VAR:-default}`](/docs/id/mcp#environment-variable-expansion-in-mcp-json) yang sama seperti `.mcp.json`, jadi entri yang ditulis sebagai `["${HOME}/bin/server"]` cocok dengan konfigurasi server yang menggunakan referensi yang sama atau jalur yang diperluas. Di Windows, referensikan variabel lingkungan yang diatur di sana, seperti `${USERPROFILE}` bukan `${HOME}`. Nilai `serverName` cocok secara harfiah dan tidak pernah diperluas.
* **URL mendukung wildcard `*`** di mana pun dalam pola, termasuk skema. Pencocokan nama host tidak peka huruf besar-kecil dan mengabaikan titik FQDN yang tertinggal, jadi `https://Mcp.Example.com/*` cocok dengan `https://mcp.example.com/api`. Jalur tetap peka huruf besar-kecil.

| Pola                        | Mengizinkan                                                                 |
| :-------------------------- | :-------------------------------------------------------------------------- |
| `https://mcp.example.com/*` | Semua jalur di domain tertentu                                              |
| `https://mcp.example.com`   | Juga semua jalur di domain itu. Pola tanpa jalur cocok dengan jalur apa pun |
| `https://*.example.com/*`   | Subdomain apa pun dari `example.com`                                        |
| `http://localhost:*/*`      | Port apa pun di localhost                                                   |
| `*://mcp.example.com/*`     | Skema apa pun ke domain tertentu                                            |

Karena ekspansi `${VAR}` membaca lingkungan proses Claude Code sendiri, entri kebijakan `serverCommand` atau `serverUrl` yang mereferensikan variabel diperluas ke nilai apa pun yang ditetapkan pengguna. Gunakan URL dan perintah literal untuk entri yang Anda andalkan untuk penegakan.

<h3 id="example-configuration">
  Contoh konfigurasi
</h3>

Konfigurasi di bawah menyiapkan daftar izin keras dengan daftar penolakan. Baris yang disorot mengubah cara sisa daftar dievaluasi, dan callout setelah blok menjelaskan masing-masing:

```json {3,5,11} theme={null}
{
  "allowedMcpServers": [
    { "serverUrl": "https://api.githubcopilot.com/*" },
    { "serverUrl": "https://mcp.sentry.dev/*" },
    { "serverCommand": ["npx", "-y", "@modelcontextprotocol/server-filesystem", "."] },
    { "serverCommand": ["python", "/usr/local/bin/approved-server.py"] },
    { "serverUrl": "https://mcp.example.com/*" },
    { "serverUrl": "https://*.internal.example.com/*" }
  ],
  "deniedMcpServers": [
    { "serverName": "dangerous-server" },
    { "serverCommand": ["npx", "-y", "unapproved-package"] },
    { "serverUrl": "https://*.untrusted.example.com/*" }
  ]
}
```

* **Baris 3**: entri `serverUrl` pertama. Setelah satu ada, setiap server jarak jauh harus cocok dengan pola URL, jadi pengguna tidak dapat mendapatkan server jarak jauh yang tidak terdaftar dengan memberikannya nama yang diizinkan.
* **Baris 5**: entri `serverCommand` pertama. Efek yang sama untuk server stdio, jadi setiap server lokal harus cocok dengan perintah yang terdaftar dengan tepat.
* **Baris 11**: entri `serverName` dalam daftar penolakan. Entri daftar penolakan selalu berlaku, jadi server apa pun yang bernama `dangerous-server` diblokir terlepas dari URL atau perintahnya.

Entri `serverName` dalam daftar izin ini tidak akan pernah cocok dengan apa pun, karena kedua jenis transportasi sudah memiliki entri yang lebih ketat.

Accordion di bawah menjelaskan cara server dievaluasi terhadap kombinasi daftar izin dan daftar penolakan lainnya.

<Accordion title="Daftar izin hanya URL">
  ```json theme={null}
  {
    "allowedMcpServers": [
      { "serverUrl": "https://mcp.example.com/*" },
      { "serverUrl": "https://*.internal.example.com/*" }
    ]
  }
  ```

  | Server                                                | Hasil                                                    |
  | :---------------------------------------------------- | :------------------------------------------------------- |
  | Server HTTP di `https://mcp.example.com/api`          | Diizinkan: cocok dengan pola URL                         |
  | Server HTTP di `https://api.internal.example.com/mcp` | Diizinkan: cocok dengan subdomain wildcard               |
  | Server HTTP di `https://external.example.com/mcp`     | Diblokir: tidak cocok dengan pola URL apa pun            |
  | Server stdio dengan perintah apa pun                  | Diblokir: tidak ada entri nama atau perintah untuk cocok |
</Accordion>

<Accordion title="Daftar izin hanya perintah">
  ```json theme={null}
  {
    "allowedMcpServers": [
      { "serverCommand": ["npx", "-y", "approved-package"] }
    ]
  }
  ```

  | Server                                                  | Hasil                                      |
  | :------------------------------------------------------ | :----------------------------------------- |
  | Server stdio dengan `["npx", "-y", "approved-package"]` | Diizinkan: cocok dengan perintah           |
  | Server stdio dengan `["node", "server.js"]`             | Diblokir: tidak cocok dengan perintah      |
  | Server HTTP bernama `my-api`                            | Diblokir: tidak ada entri nama untuk cocok |
</Accordion>

<Accordion title="Daftar izin nama dan perintah campuran">
  ```json theme={null}
  {
    "allowedMcpServers": [
      { "serverName": "github" },
      { "serverCommand": ["npx", "-y", "approved-package"] }
    ]
  }
  ```

  | Server                                                                       | Hasil                                                                        |
  | :--------------------------------------------------------------------------- | :--------------------------------------------------------------------------- |
  | Server stdio bernama `local-tool` dengan `["npx", "-y", "approved-package"]` | Diizinkan: cocok dengan perintah                                             |
  | Server stdio bernama `local-tool` dengan `["node", "server.js"]`             | Diblokir: entri perintah ada tetapi tidak cocok                              |
  | Server stdio bernama `github` dengan `["node", "server.js"]`                 | Diblokir: server stdio harus cocok dengan perintah ketika entri perintah ada |
  | Server HTTP bernama `github`                                                 | Diizinkan: cocok dengan nama                                                 |
  | Server HTTP bernama `other-api`                                              | Diblokir: nama tidak cocok                                                   |
</Accordion>

<Accordion title="Daftar izin hanya nama">
  ```json theme={null}
  {
    "allowedMcpServers": [
      { "serverName": "github" },
      { "serverName": "internal-tool" }
    ]
  }
  ```

  | Server                                                       | Hasil                                    |
  | :----------------------------------------------------------- | :--------------------------------------- |
  | Server stdio bernama `github` dengan perintah apa pun        | Diizinkan: tidak ada pembatasan perintah |
  | Server stdio bernama `internal-tool` dengan perintah apa pun | Diizinkan: tidak ada pembatasan perintah |
  | Server HTTP bernama `github`                                 | Diizinkan: cocok dengan nama             |
  | Server apa pun bernama `other`                               | Diblokir: nama tidak cocok               |
</Accordion>

<Accordion title="Daftar izin dengan penggantian daftar penolakan">
  ```json theme={null}
  {
    "allowedMcpServers": [
      { "serverUrl": "https://*.example.com/*" }
    ],
    "deniedMcpServers": [
      { "serverUrl": "https://staging.example.com/*" }
    ]
  }
  ```

  | Server                                           | Hasil                                                                              |
  | :----------------------------------------------- | :--------------------------------------------------------------------------------- |
  | Server HTTP di `https://mcp.example.com/api`     | Diizinkan: cocok dengan pola URL daftar izin, tidak ada kecocokan daftar penolakan |
  | Server HTTP di `https://staging.example.com/api` | Diblokir: cocok dengan keduanya, tetapi daftar penolakan memiliki prioritas        |
  | Server HTTP di `https://other.com/mcp`           | Diblokir: tidak cocok dengan daftar izin                                           |
</Accordion>

<h3 id="restrict-the-allowlist-to-managed-settings-only">
  Batasi daftar izin ke pengaturan yang dikelola saja
</h3>

Untuk membuat daftar izin yang dikelola satu-satunya yang berlaku, atur `allowManagedMcpServersOnly` dalam file pengaturan yang dikelola:

```json theme={null}
{
  "allowManagedMcpServersOnly": true,
  "allowedMcpServers": [
    { "serverUrl": "https://api.githubcopilot.com/*" },
    { "serverUrl": "https://*.internal.example.com/*" }
  ]
}
```

Ketika `allowManagedMcpServersOnly` adalah `true`, daftar izin dari pengaturan pengguna, proyek, dan lokal diabaikan. Daftar penolakan masih digabungkan dari semua sumber, jadi pengguna selalu dapat memblokir server untuk diri mereka sendiri.

<h2 id="how-restrictions-appear-to-users">
  Bagaimana pembatasan muncul kepada pengguna
</h2>

Ketika pembatasan memblokir server, pengguna melihat kesalahan dari `claude mcp add` atau server berhenti dimuat secara diam-diam. Gunakan tabel ini untuk mengenali laporan tersebut dan untuk memberitahu pengguna apa yang diharapkan sebelum Anda meluncurkan perubahan:

| Pembatasan                                                                | Apa yang dilihat pengguna                                                                                  |
| :------------------------------------------------------------------------ | :--------------------------------------------------------------------------------------------------------- |
| `managed-mcp.json` ada dan pengguna menjalankan `claude mcp add`          | `Cannot add MCP server: enterprise MCP configuration is active and has exclusive control over MCP servers` |
| Server ada di daftar penolakan dan pengguna menjalankan `claude mcp add`  | `Cannot add MCP server "<name>": server is explicitly blocked by enterprise policy`                        |
| Server tidak ada di daftar izin dan pengguna menjalankan `claude mcp add` | `Cannot add MCP server "<name>": not allowed by enterprise policy`                                         |
| Server yang dikonfigurasi sebelumnya sekarang diblokir oleh kebijakan     | Server menghilang secara diam-diam dari `/mcp` dan `claude mcp list` tanpa peringatan                      |

Dalam kasus terakhir, pengguna tidak mendapat sinyal bahwa kebijakan adalah alasan server mereka menghilang, jadi beri tahu pengguna yang terpengaruh server mana yang diblokir saat Anda meluncurkan pembatasan baru.

<h2 id="monitor-mcp-usage">
  Pantau penggunaan MCP
</h2>

Ketika [ekspor OpenTelemetry](/docs/id/monitoring-usage) dikonfigurasi, Claude Code dapat merekam server MCP dan alat mana yang digunakan pengguna. Atur `OTEL_LOG_TOOL_DETAILS=1` untuk menyertakan nama server dan alat MCP dalam acara alat, kemudian agregasikan di kolektor Anda untuk melihat server mana yang benar-benar dihubungkan pengguna Anda. Lihat [Monitoring](/docs/id/monitoring-usage) untuk menyiapkan pengekspor dan untuk skema acara lengkap.

<h2 id="configuration-summary">
  Ringkasan konfigurasi
</h2>

Setiap file dan pengaturan yang dibahas halaman ini, apa yang dikontrolnya, dan cara mengirimkannya:

| Permukaan                    | Apa yang dikontrol                                                        | Di mana itu berada                                                                                                                       | Cara mengirimkan                                                                                                                                                                           |
| :--------------------------- | :------------------------------------------------------------------------ | :--------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `managed-mcp.json`           | Set server tetap, kontrol eksklusif                                       | Jalur sistem: `/Library/Application Support/ClaudeCode/`, `/etc/claude-code/`, atau `C:\Program Files\ClaudeCode\`                       | MDM, GPO, manajemen armada, atau proses apa pun dengan hak istimewa administrator. Tidak dapat diatur melalui pengaturan yang dikelola server                                              |
| `allowedMcpServers`          | Daftar izin server yang diizinkan                                         | File [pengaturan](/docs/id/settings#settings-files) apa pun; entri dari setiap sumber digabungkan kecuali `allowManagedMcpServersOnly` diatur | Untuk penegakan, [sumber pengaturan yang dikelola](/docs/id/admin-setup#decide-how-settings-reach-devices): pengaturan yang dikelola server, `managed-settings.json`, profil MDM, atau registri |
| `deniedMcpServers`           | Daftar penolakan server yang diblokir                                     | File pengaturan apa pun; entri dari setiap sumber digabungkan                                                                            | Sama seperti `allowedMcpServers`                                                                                                                                                           |
| `allowManagedMcpServersOnly` | Mengunci daftar izin ke sumber yang dikelola saja                         | Hanya sumber pengaturan yang dikelola; pengaturan tidak berpengaruh di tempat lain                                                       | Sama seperti `allowedMcpServers`                                                                                                                                                           |
| `allowAllClaudeAiMcps`       | Memuat konektor claude.ai bersama `managed-mcp.json` alih-alih menekannya | Hanya sumber pengaturan yang dikelola; pengaturan tidak berpengaruh di tempat lain                                                       | Sama seperti `allowedMcpServers`                                                                                                                                                           |

<h2 id="related-resources">
  Sumber daya terkait
</h2>

* [Tentukan apa yang akan diterapkan](/docs/id/admin-setup#decide-what-to-enforce): pembatasan MCP bersama dengan aturan izin, sandboxing, dan kontrol admin lainnya
* [Hubungkan Claude Code ke alat melalui MCP](/docs/id/mcp): referensi MCP lengkap, termasuk transportasi, cakupan, dan autentikasi
* [Pengaturan](/docs/id/settings): hierarki pengaturan dan bagaimana pengaturan yang dikelola memiliki prioritas
* [Pengaturan yang dikelola server](/docs/id/server-managed-settings): kirimkan `allowedMcpServers` dan `deniedMcpServers` dari konsol admin Claude.ai
* [Keamanan](/docs/id/security): model ancaman yang dilindungi kontrol ini
* [Panduan Administrator Perusahaan Claude](https://claude.com/resources/tutorials/claude-enterprise-administrator-guide): SSO, SCIM, manajemen kursi, dan playbook peluncuran
