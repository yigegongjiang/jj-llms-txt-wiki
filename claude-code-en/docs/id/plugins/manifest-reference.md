> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Referensi manifest plugin

> Referensi lengkap untuk plugin.json: setiap field dengan tipenya dan default, bentuk path yang diterima, dan skema userConfig serta variabel lingkungan.

Manifest plugin adalah file `plugin.json` di direktori `.claude-plugin/` plugin. File ini membawa metadata plugin dan nilai [`userConfig`](#user-configuration) yang diminta Claude Code kepada pengguna. File ini juga mendeklarasikan komponen apa pun yang Anda tentukan secara inline atau simpan di luar [lokasi defaultnya](#standard-layout).

Referensi ini untuk pembuat plugin, dan untuk pemilik marketplace yang menempatkan field komponen dalam entri marketplace.

<Note>
  Kasus-kasus ini tercakup di halaman lain:

  * **Belajar membangun plugin**: mulai dengan [Buat plugin](/docs/id/plugins/create)
  * **Apa yang dilakukan setiap komponen saat runtime**: lihat [Komponen plugin](/docs/id/plugins/components)
</Note>

Mulai dari bagian yang sesuai dengan apa yang Anda cari:

* Sebuah field: tabel [Fields](#fields) memberikan tipe setiap field, apakah diperlukan, defaultnya, dan apa yang diterima. [Path rules](#path-rules) mencakup awalan `./` dan containment untuk setiap path komponen
* Opsi `userConfig` atau entri `channels`: skema [User configuration](#user-configuration) dan [Channels](#channels)
* `${CLAUDE_PLUGIN_ROOT}` atau variabel lain yang dapat direferensikan plugin: [Environment variables](#environment-variables)
* Di mana file setiap komponen berada: [Standard layout](#standard-layout)
* Pesan dari `claude plugin validate`: halaman [troubleshooting](/docs/id/plugins/troubleshooting) mencantumkan setiap pesan dengan perbaikannya dan tautan ke bagian relevan di halaman ini

<h2 id="manifest-file">
  Manifest file
</h2>

Manifest bersifat opsional. Tanpanya, Claude Code memuat komponen yang ditemukannya di [standard layout](#standard-layout). Nama plugin kemudian berasal dari entri marketplace, atau dari nama direktori saat Anda memuat plugin dengan `--plugin-dir`.

Tulis manifest saat Anda menginginkan metadata, komponen di luar direktori defaultnya, `userConfig`, atau definisi komponen inline.

Simpan manifest di `.claude-plugin/plugin.json` di bawah root plugin. Letakkan setiap file plugin lainnya di root plugin, bukan di dalam `.claude-plugin/`. Ini termasuk `skills/`, `commands/`, dan `hooks/`.

Contoh berikut menetapkan sebagian besar kunci dalam tabel [Fields](#fields). Ini melewati validasi di direktori plugin yang berisi setiap path yang direferensikan.

```json theme={null}
{
  "name": "deploy-tools",
  "displayName": "Deploy Tools",
  "version": "1.2.0",
  "description": "Deployment commands, a review agent, and a status monitor",
  "author": {
    "name": "Example Team",
    "email": "dev@example.com",
    "url": "https://example.com"
  },
  "homepage": "https://example.com/docs/deploy-tools",
  "repository": "https://github.com/example/deploy-tools",
  "license": "MIT",
  "keywords": ["deployment", "ci"],
  "defaultEnabled": true,
  "dependencies": ["secrets-vault"],
  "metadata": { "catalogId": "cat-123" },
  "skills": ["./extra-skills/"],
  "commands": {
    "status": {
      "source": "./commands/status.md",
      "description": "Show the current deployment status"
    },
    "about": {
      "content": "Explain what the deploy-tools plugin provides.",
      "description": "Describe this plugin"
    }
  },
  "agents": ["./agents/reviewer.md"],
  "hooks": "./config/extra-hooks.json",
  "mcpServers": {
    "deploy-api": {
      "command": "node",
      "args": ["${CLAUDE_PLUGIN_ROOT}/server.js"]
    }
  },
  "lspServers": "./.lsp.json",
  "outputStyles": "./styles/",
  "experimental": {
    "themes": "./themes/",
    "monitors": "./config/monitors.json"
  },
  "userConfig": {
    "api_token": {
      "type": "string",
      "title": "API token",
      "description": "Token for the deployment API",
      "sensitive": true
    }
  }
}
```

<h3 id="unrecognized-fields">
  Unrecognized fields
</h3>

Kunci tingkat atas yang tidak dikenali akan dihapus, dan kunci yang tidak dikenali di dalam opsi `userConfig`, entri `channels`, config `lspServers`, atau entri `monitors` akan ditolak:

* **Top-level fields**: field dihapus dan plugin dimuat. `claude plugin validate` melaporkan setiap field tingkat atas yang tidak dikenali sebagai peringatan
* **Strict objects**: opsi `userConfig`, entri `channels`, config `lspServers`, dan entri `monitors` bersifat ketat. Kunci yang tidak diketahui di dalamnya adalah kesalahan, dan plugin tidak dimuat

<h3 id="validate-the-manifest">
  Validate the manifest
</h3>

`claude plugin validate` adalah pemeriksaan otoritatif untuk manifest. Jalankan dari shell Anda terhadap direktori plugin:

```bash theme={null}
claude plugin validate ./my-plugin
```

Perintah melaporkan salah satu hasil berikut:

* **`Validation passed`**: manifest dimuat
* **`Validation passed with warnings`**: manifest dimuat, tetapi validator menemukan sesuatu untuk diperbaiki, seperti field tingkat atas yang tidak diketahui yang Claude Code hapus, `name` yang bukan kebab-case, atau `version`, `description`, atau `author` yang hilang. Lewatkan `--strict` untuk mengubah peringatan menjadi kegagalan di CI
* **`Validation failed`**: manifest memiliki ketidakcocokan tipe, path yang hilang atau keluar dari root plugin, atau kunci yang tidak diketahui di dalam opsi `userConfig`, entri `channels`, config `lspServers`, atau entri `monitors`. Claude Code melaporkan masalah yang sama saat memuat plugin

<h2 id="fields">
  Fields
</h2>

Tabel mencantumkan kunci tingkat atas dalam `plugin.json`. `name` adalah satu-satunya kunci yang diperlukan. Jika nama field adalah tautan, bagian yang ditautkan memiliki aturan lengkapnya.

Untuk kunci komponen seperti `commands` dan `hooks`, [Component path forms](#component-path-forms) menunjukkan setiap bentuk yang diterima dengan contoh, dan setiap path mengikuti [path rules](#path-rules) untuk awalan `./`, ekstensi, dan containment.

| Field                                | Type                             | Description                                                                                                                                                                                                                                                                                                                     |
| :----------------------------------- | :------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `$schema`                            | String                           | JSON Schema URL untuk autocomplete editor. Claude Code mengabaikannya saat waktu muat                                                                                                                                                                                                                                           |
| [`name`](#name)                      | String                           | Identifier plugin, diperlukan. Gunakan kebab-case. Setiap komponen di-namespace di bawahnya                                                                                                                                                                                                                                     |
| [`displayName`](#displayname)        | String                           | Nama yang ditampilkan di UI sebagai pengganti `name`                                                                                                                                                                                                                                                                            |
| [`version`](#version)                | String                           | String versi. Menetapkannya membuat pengguna tetap pada versi itu sampai Anda mengubahnya                                                                                                                                                                                                                                       |
| `description`                        | String                           | Penjelasan singkat tentang apa yang disediakan plugin                                                                                                                                                                                                                                                                           |
| `author`                             | Object                           | `name`, yang diperlukan, ditambah `email` dan `url` opsional                                                                                                                                                                                                                                                                    |
| `homepage`                           | String                           | URL dokumentasi. Harus diparse sebagai URL, atau plugin gagal dimuat                                                                                                                                                                                                                                                            |
| `repository`                         | String                           | URL repositori sumber. Tidak divalidasi                                                                                                                                                                                                                                                                                         |
| `license`                            | String                           | Identifier SPDX seperti `MIT` atau `Apache-2.0`                                                                                                                                                                                                                                                                                 |
| `keywords`                           | Array of strings                 | Tag penemuan                                                                                                                                                                                                                                                                                                                    |
| [`metadata`](#metadata)              | Object                           | Objek bentuk bebas untuk data Anda sendiri. Claude Code tidak membacanya                                                                                                                                                                                                                                                        |
| [`defaultEnabled`](#defaultenabled)  | Boolean                          | Apakah plugin dimulai diaktifkan saat pengguna belum menetapkannya. Default ke `true`                                                                                                                                                                                                                                           |
| [`dependencies`](#dependencies)      | Array of strings or objects      | Plugin yang harus diaktifkan agar yang ini berfungsi                                                                                                                                                                                                                                                                            |
| [`settings`](#settings)              | Object                           | Pengaturan yang Claude Code terapkan saat plugin diaktifkan. Hanya `agent` dan `subagentStatusLine` yang berlaku                                                                                                                                                                                                                |
| [`userConfig`](#user-configuration)  | Object                           | Nilai yang Claude Code minta kepada pengguna saat plugin diaktifkan                                                                                                                                                                                                                                                             |
| [`channels`](#channels)              | Array of objects                 | Saluran pesan yang disediakan plugin, masing-masing terikat ke salah satu server MCP-nya                                                                                                                                                                                                                                        |
| `skills`                             | Path, or array of paths          | Direktori untuk dipindai untuk skills, masing-masing direktori folder `<name>/SKILL.md` atau satu folder yang menyimpan `SKILL.md` secara langsung. `"."` menamai root plugin. Menambah pemindaian default `skills/`                                                                                                            |
| [`commands`](#commands)              | Path, array of paths, or object  | File perintah `.md` datar, direktori mereka, atau peta objek nama perintah ke `source` atau `content`. Menggantikan pemindaian default `commands/`                                                                                                                                                                              |
| `agents`                             | Path, or array of paths          | File agent `.md`. Direktori tidak diterima. Menggantikan pemindaian default `agents/`                                                                                                                                                                                                                                           |
| [`hooks`](#hooks)                    | Path, object, or array of either | File hook `.json` atau config hook inline. Dimuat bersama dengan `hooks/hooks.json`                                                                                                                                                                                                                                             |
| [`mcpServers`](#mcpservers)          | Path, object, or array of either | File config MCP `.json`, bundle `.mcpb` atau `.dxt`, atau config server inline yang di-key berdasarkan nama. Dimuat bersama dengan `.mcp.json`; nama server yang dideklarasikan kemudian menggantikan yang sebelumnya                                                                                                           |
| [`lspServers`](#lspservers)          | Path, object, or array of either | File config LSP `.json` atau config server inline yang di-key berdasarkan nama. Dimuat bersama dengan `.lsp.json`                                                                                                                                                                                                               |
| `outputStyles`                       | Path, or array of paths          | File gaya output atau direktori. Menggantikan pemindaian default `output-styles/`                                                                                                                                                                                                                                               |
| `workflows`                          | Path, or array of paths          | File [Workflow](/docs/id/workflows#distribute-a-workflow-in-a-plugin) `.js` atau direktori. Menggantikan pemindaian default `workflows/`                                                                                                                                                                                             |
| `experimental`                       | Object                           | Kontainer untuk `themes`, `monitors`, dan `evals`, yang bentuk manifestnya mungkin masih berubah                                                                                                                                                                                                                                |
| `experimental.themes`                | Path, or array of paths          | File tema atau direktori. Menggantikan pemindaian default `themes/`. Kunci `themes` tingkat atas masih dimuat, dengan peringatan `claude plugin validate`                                                                                                                                                                       |
| [`experimental.monitors`](#monitors) | Path, or inline array            | File `.json` yang menyimpan array monitors, atau array itu sendiri. Default ke `monitors/monitors.json`. Kunci `monitors` tingkat atas masih dimuat, dengan peringatan `claude plugin validate`. Monitor hanya berjalan dalam sesi interaktif, dan bukan di Amazon Bedrock, Agent Platform Google Cloud, atau Microsoft Foundry |
| `experimental.evals`                 | Path, or array of paths          | Direktori yang menyimpan [eval cases](/docs/id/plugin-evals#use-a-different-eval-directory) plugin saat bukan default `evals/`. `claude plugin eval --eval-dir` menimpanya                                                                                                                                                           |

Di kolom Type, path adalah string relatif terhadap root plugin, seperti `"./custom/commands"`.

<h3 id="name">
  `name`
</h3>

Identifier plugin. Harus non-kosong, tanpa spasi, `@`, `:`, pemisah path, karakter kontrol, atau karakter pemformatan bidirectional; gunakan kebab-case.

Claude Code mem-namespace setiap komponen di bawahnya, jadi agent `reviewer` dalam plugin `deploy-tools` muncul sebagai `deploy-tools:reviewer`.

<h3 id="displayname">
  `displayName`
</h3>

Nama yang ditampilkan di UI sebagai pengganti `name`. Mungkin berisi spasi dan casing apa pun, dan tidak digunakan untuk namespacing atau pencarian.

Untuk plugin yang diinstal dari marketplace, `displayName` di [entri marketplace](/docs/id/plugins/marketplace-reference#plugin-entries) mengambil alih nilai ini.

<h3 id="version">
  `version`
</h3>

String versi, tidak diperiksa terhadap semver. Menetapkannya mengunci plugin ke versi itu sampai Anda mengubahnya; lihat [Versions and updates](/docs/id/plugins/loading#versions-and-updates). Plugin dengan [`command` source](/docs/id/plugins/marketplace-reference), plugin dari [marketplace yang dihosting di claude.ai](/docs/id/plugins/install#add-from-claude-ai), dan plugin [dimuat di tempat](/docs/id/plugins/loading#find-plugins-on-disk) dari marketplace yang ditambahkan sebagai direktori lokal tidak dikunci oleh field ini.

<h3 id="metadata">
  `metadata`
</h3>

Objek bentuk bebas untuk data Anda sendiri, seperti field katalog atau hak. Claude Code tidak membacanya. Memerlukan Claude Code v2.1.222 atau lebih baru.

<h3 id="defaultenabled">
  `defaultEnabled`
</h3>

Apakah plugin dimulai diaktifkan saat pengguna belum menetapkannya di [`enabledPlugins`](/docs/id/settings-reference#enabledplugins). Default ke `true`. Plugin yang diaktifkan plugin tergantung dimulai diaktifkan terlepas dari itu. Field yang sama di entri marketplace menimpanya.

Setelah entri `enabledPlugins` pengguna ditulis, itu bertahan di seluruh pembaruan plugin, jadi mengubah `defaultEnabled` dalam rilis kemudian tidak mengubah pengaturan untuk pengguna yang ada.

<h3 id="dependencies">
  `dependencies`
</h3>

Plugin yang harus diaktifkan agar yang ini berfungsi. Setiap entri adalah `"name"`, `"name@marketplace"`, atau `{ "name": "...", "marketplace": "...", "version": "..." }`. Nama bare diselesaikan terhadap marketplace plugin ini sendiri. Lihat [dependency constraints](/docs/id/plugins/dependencies).

<h3 id="settings">
  `settings`
</h3>

Pengaturan yang Claude Code terapkan saat plugin diaktifkan. Hanya `agent` dan `subagentStatusLine` yang berlaku; kunci lain dihapus saat muat. `settings.json` di root plugin mengambil alih kunci ini. Lihat [Default settings](/docs/id/plugins/components#default-settings).

<h2 id="component-path-forms">
  Component path forms
</h2>

Setiap kunci komponen menerima path relatif terhadap root plugin. `hooks`, `mcpServers`, `lspServers`, dan `experimental.monitors` juga menerima config inline, `commands` juga menerima peta objek, dan `mcpServers` juga menerima path bundle MCP dan URL. Contoh-contoh berikut menunjukkan setiap bentuk yang diterima sekali. Untuk apa yang dilakukan setiap komponen saat runtime, lihat [Plugin components](/docs/id/plugins/components).

<h3 id="path-only-fields">
  Path-only fields
</h3>

`agents`, `skills`, `outputStyles`, `workflows`, dan `experimental.themes` mengambil satu path atau array path. Entri `agents` harus file `.md`, dan entri `skills` harus direktori. Tiga lainnya menerima direktori atau file.

```json theme={null}
{
  "agents": ["./custom-agents/reviewer.md", "./custom-agents/tester.md"],
  "skills": ["./extra-skills/", "."],
  "outputStyles": "./styles/"
}
```

<h3 id="commands">
  `commands`
</h3>

`commands` mengambil path, array path, atau peta objek. Path menamai file perintah `.md` datar atau direktori. Dalam peta objek, setiap kunci menjadi nama perintah setelah awalan plugin. Misalnya, `"about"` dalam plugin `deploy-tools` berjalan sebagai `/deploy-tools:about`.

Setiap nilai menetapkan tepat satu dari `source` atau `content`, dan entri yang menetapkan keduanya atau tidak ada gagal validasi. Field lain dalam tabel ini opsional:

| Field          | Type             | Description                                                           |
| :------------- | :--------------- | :-------------------------------------------------------------------- |
| `source`       | string           | Path ke file Markdown perintah, relatif terhadap root plugin          |
| `content`      | string           | Markdown inline untuk badan perintah, sebagai pengganti `source`      |
| `description`  | string           | Deskripsi yang ditampilkan untuk perintah                             |
| `argumentHint` | string           | Hint argumen yang ditampilkan setelah nama perintah, seperti `[file]` |
| `model`        | string           | Model default untuk perintah                                          |
| `allowedTools` | array of strings | Tools yang dapat digunakan perintah tanpa prompt                      |

Peta ini mendeklarasikan satu perintah dari file dan satu dari konten inline:

```json theme={null}
{
  "commands": {
    "status": { "source": "./commands/status.md", "argumentHint": "[env]" },
    "about": { "content": "Explain what this plugin provides." }
  }
}
```

<h3 id="hooks">
  `hooks`
</h3>

`hooks` mengambil path file `.json`, objek hooks inline dalam bentuk yang sama seperti [`hooks` dalam `settings.json`](/docs/id/hooks#configuration), atau array yang mencampur keduanya. Untuk event hook dan field handler, lihat [hooks reference](/docs/id/hooks#hook-events).

Claude Code menggabungkan apa pun yang Anda deklarasikan dengan `hooks/hooks.json` saat file itu ada.

```json theme={null}
{
  "hooks": [
    "./config/extra-hooks.json",
    {
      "PostToolUse": [
        {
          "matcher": "Write|Edit",
          "hooks": [
            { "type": "command", "command": "\"${CLAUDE_PLUGIN_ROOT}\"/scripts/format.sh" }
          ]
        }
      ]
    }
  ]
}
```

<h3 id="mcpservers">
  `mcpServers`
</h3>

`mcpServers` mengambil path file `.json`, path bundle MCP atau URL, peta inline, atau array yang mencampur mereka. Untuk field config server, lihat [plugin-provided MCP servers](/docs/id/mcp#plugin-provided-mcp-servers).

Claude Code memuat `.mcp.json` di root plugin terlebih dahulu, kemudian setiap bentuk yang dideklarasikan secara berurutan. Nama server yang dideklarasikan kemudian menggantikan yang sebelumnya.

Nilai `mcpServers` mengambil salah satu bentuk berikut:

| Shape             | Example value                                                                          | What Claude Code does                                                                                      |
| :---------------- | :------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------- |
| `.json` file path | `"./mcp/servers.json"`                                                                 | Membaca file sebagai peta `mcpServers`                                                                     |
| MCP bundle path   | `"./bundle.mcpb"`                                                                      | Mengekstrak bundle `.mcpb` atau `.dxt` ke `.mcpb-cache/` di bawah root plugin dan membaca config servernya |
| MCP bundle URL    | `"https://example.com/server.mcpb"`                                                    | Mengunduh bundle ke `.mcpb-cache/`, kemudian membacanya                                                    |
| Inline map        | `{ "deploy-api": { "command": "node", "args": ["${CLAUDE_PLUGIN_ROOT}/server.js"] } }` | Menggunakan peta sebagai config server yang di-key berdasarkan nama                                        |

Path bundle atau URL harus berakhir dengan `.mcpb` atau `.dxt`. Ekstensi lain apa pun gagal validasi.

<h3 id="lspservers">
  `lspServers`
</h3>

`lspServers` mengambil path file `.json`, peta inline nama server ke config, atau array keduanya.

Claude Code memuat `.lsp.json` di root plugin terlebih dahulu, kemudian setiap config yang dideklarasikan secara berurutan. Nama server yang dideklarasikan kemudian menggantikan yang sebelumnya.

Setiap config server adalah objek ketat dengan field berikut. Kunci yang tidak diketahui gagal validasi.

| Field                   | Required | Description                                                                                                                                                                                 |
| :---------------------- | :------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `command`               | Yes      | Binary language server. Tanpa spasi kecuali nilai dimulai dengan `/`; letakkan argumen di `args`                                                                                            |
| `extensionToLanguage`   | Yes      | Peta ekstensi file ke LSP language ID, setidaknya satu entri. Kunci dimulai dengan titik, seperti `".go"`                                                                                   |
| `args`                  | No       | Argumen yang dilewatkan ke server                                                                                                                                                           |
| `transport`             | No       | Transport komunikasi: `stdio` (default) atau `socket`. Claude Code menerima `socket` tetapi menjalankan setiap server melalui stdio, jadi aturan protokol stdout berlaku untuk semua server |
| `env`                   | No       | Variabel lingkungan untuk proses server                                                                                                                                                     |
| `initializationOptions` | No       | Opsi yang dikirim dalam permintaan initialize                                                                                                                                               |
| `settings`              | No       | Pengaturan yang dikirim oleh `workspace/didChangeConfiguration`                                                                                                                             |
| `workspaceFolder`       | No       | Path folder workspace untuk server                                                                                                                                                          |
| `startupTimeout`        | No       | Milidetik untuk menunggu startup, integer positif                                                                                                                                           |
| `shutdownTimeout`       | No       | Milidetik untuk menunggu shutdown yang elegan, integer positif. Saat timeout berlalu, Claude Code menghentikan proses server. Saat tidak diatur, tidak ada timeout yang berlaku             |
| `restartOnCrash`        | No       | Apakah memulai ulang server setelah crash. Default ke `true`. Atur ke `false` untuk membiarkan server yang crash tetap berhenti daripada memulai ulang                                      |
| `maxRestarts`           | No       | Upaya restart sebelum menyerah, nol atau lebih                                                                                                                                              |
| `diagnostics`           | No       | Apakah mendorong diagnostik ke konteks setelah edit. Default ke `true`                                                                                                                      |

Config inline ini menjalankan `gopls` untuk file `.go`:

```json theme={null}
{
  "lspServers": {
    "go": {
      "command": "gopls",
      "args": ["serve"],
      "extensionToLanguage": { ".go": "go" }
    }
  }
}
```

Untuk language server yang Anthropic terbitkan sebagai plugin dan bagaimana server berperilaku saat runtime, lihat [Code intelligence](/docs/id/plugins/code-intelligence).

<h3 id="monitors">
  `monitors`
</h3>

`experimental.monitors` mengambil path file `.json` atau array inline. Saat Anda menghilangkan kunci, Claude Code memuat `monitors/monitors.json` jika ada.

Setiap entri adalah objek ketat dengan field berikut.

| Field         | Required | Description                                                                                                                                                    |
| :------------ | :------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`        | Yes      | Identifier unik dalam plugin                                                                                                                                   |
| `command`     | Yes      | Perintah shell yang Claude Code jalankan sebagai proses latar belakang persisten dalam direktori kerja sesi                                                    |
| `description` | Yes      | Ringkasan singkat yang ditampilkan di panel tugas dan ringkasan notifikasi                                                                                     |
| `when`        | No       | Dengan `"always"`, default, monitor dimulai saat awal sesi dan pada reload plugin. Dengan `"on-skill-invoke:<skill>"`, dimulai pertama kali skill itu berjalan |

Array inline ini mendeklarasikan satu monitor yang dimulai pertama kali skill `deploy` berjalan:

```json theme={null}
{
  "experimental": {
    "monitors": [
      {
        "name": "deploy-status",
        "command": "\"${CLAUDE_PLUGIN_ROOT}\"/scripts/poll-deploy.sh",
        "description": "Deployment status changes",
        "when": "on-skill-invoke:deploy"
      }
    ]
  }
}
```

Perintah monitor `command` tidak dapat mereferensikan `${user_config.*}`. Lihat [Fields that run through a shell](#fields-that-run-through-a-shell).

<h2 id="path-rules">
  Path rules
</h2>

Setiap path komponen dalam manifest relatif terhadap root plugin dan harus dimulai dengan `./`. Path seperti `commands/foo.md` gagal validasi. `skills` dan `mcpServers` masing-masing menerima satu bentuk di luar aturan itu:

* **`skills`**: juga menerima `"."`. Baik `"."` maupun `"./"` menunjukkan root plugin. Sebelum v2.1.221, `"."` gagal validasi manifest, jadi gunakan `"./"` saat plugin harus dimuat di versi sebelumnya
* **`mcpServers`**: juga menerima URL bundle `https://`

<h3 id="containment-and-existence">
  Containment and existence
</h3>

Setiap path komponen harus diselesaikan di dalam root plugin dan harus ada. `claude plugin validate` tidak memeriksa path `outputStyles`, `lspServers`, `monitors`, atau `themes`, jadi path yang buruk di field tersebut gagal hanya saat plugin dimuat:

* **Containment**: path yang diselesaikan di luar root plugin tidak dimuat, dan tab **Errors** `/plugin` menampilkan `<component> path escapes plugin directory: <path>`. Path yang berisi `..` adalah kasus biasa, dan `claude plugin validate` melaporkannya sebagai `Path contains ".." which could be a path traversal attempt`
* **Existence**: path yang tidak ada tidak dimuat, dan tab **Errors** `/plugin` menampilkan `<component> path not found: <path>`. `claude plugin validate` melaporkannya sebagai `Path not found`

<h3 id="how-each-key-combines-with-its-default-location">
  How each key combines with its default location
</h3>

Setiap kunci komponen baik menggantikan lokasi defaultnya, menambahnya, atau menggabungkannya:

* **Menggantikan default**: `commands`, `agents`, `outputStyles`, `workflows`, `experimental.themes`, `experimental.monitors`. Saat Anda menetapkan `commands`, direktori default `commands/` tidak dipindai. Untuk menyimpan default dan menambah lebih banyak, daftarkan secara eksplisit: `"commands": ["./commands/", "./extras/"]`
* **Menambah default**: `skills`. Direktori `skills/` masih dipindai, dan direktori yang terdaftar dimuat bersama dengannya
* **Menggabungkan**: `hooks`, `mcpServers`, `lspServers`. File default dimuat terlebih dahulu, dan apa yang manifest deklarasikan digabungkan ke dalamnya, seperti dijelaskan di bawah [Component path forms](#component-path-forms)

Jika plugin memiliki folder default seperti `commands/` dan juga menetapkan kunci manifest yang menggantinya, Claude Code memuat path manifest dan bukan folder. `claude plugin list` dan antarmuka `/plugin` kemudian menampilkan peringatan `Default <folder>/ folder is ignored because the manifest sets "<key>"`.

Untuk menghindari peringatan, atur kunci ke path di dalam folder itu: `"commands": ["./commands/deploy.md"]` menamai file di folder default dan tidak menghasilkan peringatan.

<h2 id="user-configuration">
  User configuration
</h2>

`userConfig` mendeklarasikan nilai yang Claude Code minta kepada pengguna saat plugin diaktifkan, sehingga pengguna tidak mengedit `settings.json` sendiri.

Kunci adalah identifier yang terdiri dari huruf, digit, dan garis bawah, dan tidak dapat dimulai dengan digit.

Setiap nilai adalah objek ketat dengan field berikut. Kunci yang tidak diketahui gagal validasi.

| Field         | Required | Description                                                                                                                                                                                                  |
| :------------ | :------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`        | Yes      | Salah satu dari `string`, `number`, `boolean`, `directory`, atau `file`                                                                                                                                      |
| `title`       | Yes      | Label yang ditampilkan dalam dialog konfigurasi                                                                                                                                                              |
| `description` | Yes      | Teks bantuan yang ditampilkan di bawah field                                                                                                                                                                 |
| `required`    | No       | Jika `true`, dialog konfigurasi tidak menerima nilai kosong                                                                                                                                                  |
| `default`     | No       | Nilai yang digunakan saat pengguna tidak memberikan apa pun: string, number, boolean, atau array string                                                                                                      |
| `options`     | No       | Untuk `string`, nilai yang diterima field, ditampilkan sebagai picker di `/config`. Lihat [Limit a field to fixed options](#limit-a-field-to-fixed-options). Memerlukan Claude Code v2.1.271 atau lebih baru |
| `multiple`    | No       | Untuk `string`, memungkinkan array string                                                                                                                                                                    |
| `sensitive`   | No       | Jika `true`, menyembunyikan input dan menyimpan nilai di penyimpanan aman daripada `settings.json`                                                                                                           |
| `min` / `max` | No       | Batas untuk `number`                                                                                                                                                                                         |

Setiap opsi setiap plugin yang diaktifkan juga muncul sebagai baris di panel `/config`, kecuali opsi `sensitive` dan daftar `multiple`. Baris `/config` memerlukan Claude Code v2.1.269 atau lebih baru.

`userConfig` ini mendeklarasikan endpoint dan token yang disembunyikan:

```json theme={null}
{
  "userConfig": {
    "api_endpoint": {
      "type": "string",
      "title": "API endpoint",
      "description": "Your team's API endpoint"
    },
    "api_token": {
      "type": "string",
      "title": "API token",
      "description": "API authentication token",
      "sensitive": true
    }
  }
}
```

<h3 id="limit-a-field-to-fixed-options">
  Limit a field to fixed options
</h3>

Atur `options` pada field `userConfig` untuk membuat pengguna memilih nilainya dari daftar tetap.

Untuk membatasi field `tone` ke tiga opsi, daftarkan di `options` dan atur `default` ke salah satunya:

```json theme={null}
{
  "userConfig": {
    "tone": {
      "type": "string",
      "title": "Tone",
      "description": "Voice for generated replies",
      "options": ["neutral", "warm", "formal"],
      "default": "neutral"
    }
  }
}
```

Jika Anda mendeklarasikan `options` pada field apa pun, pengguna di versi Claude Code sebelum v2.1.271 tidak dapat memuat plugin.

`options` berlaku untuk field `string` yang bukan `multiple` atau `sensitive`. Atur `default` ke salah satu nilai yang terdaftar, atau atur `required: true` sehingga pengguna harus memilih satu. Setiap opsi adalah label polos 1 hingga 64 karakter, dan `claude plugin validate`, yang Anda jalankan di shell Anda, melaporkan apa pun yang ditolaknya. Plugin yang `options`-nya melanggar aturan ini gagal dimuat.

<h3 id="where-values-are-stored">
  Where values are stored
</h3>

Nilai non-sensitif disimpan di bawah [`pluginConfigs`](/docs/id/settings-reference#pluginconfigs) dalam `settings.json` pengguna. Nilai sensitif masuk ke penyimpanan kredensial aman platform sebagai gantinya. [Halaman pengaturan](/docs/id/settings-reference#pluginconfigs) mencantumkan file pengaturan mana yang `pluginConfigs` dibaca.

<h3 id="reference-a-saved-value">
  Reference a saved value
</h3>

Referensikan nilai yang disimpan di mana plugin membutuhkannya, dalam salah satu dari dua bentuk:

* **`${user_config.KEY}`**: disubstitusi dalam config server MCP, config server LSP, [exec-form](/docs/id/hooks#exec-form-and-shell-form) hook `args`, dan konten skill dan agent. Dalam konten skill dan agent, hanya nilai non-sensitif yang disubstitusi, dan nilai sensitif di sana menjadi placeholder
* **`CLAUDE_PLUGIN_OPTION_<KEY>`**: diekspor ke proses hook untuk setiap opsi, dengan `<KEY>` huruf besar. Hook bentuk shell membaca `$CLAUDE_PLUGIN_OPTION_API_TOKEN` untuk `api_token`

<h3 id="fields-that-run-through-a-shell">
  Fields that run through a shell
</h3>

Perintah hook bentuk shell, perintah monitor, dan MCP [`headersHelper`](/docs/id/mcp#use-dynamic-headers-for-custom-authentication) menolak `${user_config.*}`. Komponen yang mereferensikannya di salah satu field ini gagal dengan [error](/docs/id/errors#plugin-command-references-user-config) daripada berjalan, karena nilai field dilewatkan ke shell yang akan mem-parse ulang nilai yang disubstitusi.

Tabel menunjukkan bagaimana nilai dapat mencapai setiap field ini sebagai gantinya.

| Field                    | How the value can reach it                                                                                                                                                                                                |
| :----------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Shell-form hook commands | Gunakan [exec form](/docs/id/hooks#exec-form-and-shell-form) dengan `args`, atau baca `CLAUDE_PLUGIN_OPTION_<KEY>` dari lingkungan hook                                                                                        |
| Monitor commands         | Bukan melalui Claude Code. Proses monitor tidak menerima `CLAUDE_PLUGIN_OPTION_<KEY>`, jadi skrip monitor harus mendapatkan nilai sendiri                                                                                 |
| MCP `headersHelper`      | Bukan melalui Claude Code. Lingkungan helper membawa `CLAUDE_PLUGIN_ROOT`, `CLAUDE_CODE_MCP_SERVER_NAME`, dan `CLAUDE_CODE_MCP_SERVER_URL` tetapi tidak ada nilai opsi, jadi skrip helper harus mendapatkan nilai sendiri |

<h2 id="channels">
  Channels
</h2>

`channels` mendeklarasikan saluran pesan yang disediakan plugin, seperti jembatan ke aplikasi chat. Saat Anda mendeklarasikan satu, Claude Code dapat meminta konfigurasi saluran saat plugin diaktifkan. Untuk bagaimana server menyuntikkan pesan, lihat [channels reference](/docs/id/channels-reference#package-as-a-plugin).

Setiap entri adalah objek ketat yang terikat ke salah satu server MCP plugin, dengan field berikut:

| Field         | Required | Description                                                                                                                                                                                |
| :------------ | :------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `server`      | Yes      | Kunci server MCP dalam `mcpServers` plugin ini yang saluran ikat ke                                                                                                                        |
| `displayName` | No       | Nama yang ditampilkan dalam judul dialog konfigurasi. Default ke nama server                                                                                                               |
| `userConfig`  | No       | Opsi untuk diminta, dalam bentuk yang sama seperti [`userConfig` tingkat atas](#user-configuration). Nilai yang disimpan disubstitusi ke referensi `${user_config.KEY}` dalam `env` server |

Manifest ini mengikat saluran ke server MCP `telegram` plugin dan meminta token bot yang disubstitusi ke dalam `env` server:

```json theme={null}
{
  "mcpServers": {
    "telegram": {
      "command": "node",
      "args": ["${CLAUDE_PLUGIN_ROOT}/server.js"],
      "env": { "BOT_TOKEN": "${user_config.bot_token}" }
    }
  },
  "channels": [
    {
      "server": "telegram",
      "displayName": "Telegram",
      "userConfig": {
        "bot_token": {
          "type": "string",
          "title": "Bot token",
          "description": "Telegram bot token",
          "sensitive": true
        }
      }
    }
  ]
}
```

<h2 id="environment-variables">
  Environment variables
</h2>

Claude Code menyediakan tiga variabel path ke komponen plugin. Referensikan sebagai `${NAME}` di field yang tercantum di bawah [Where each variable resolves](#where-each-variable-resolves), dan bacanya sebagai variabel lingkungan dalam proses yang menerimanya.

| Variable                | Resolves to                                                                                                                                                                                                          | Use it for                                                                       |
| :---------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------- |
| `${CLAUDE_PLUGIN_ROOT}` | Path absolut dari versi plugin yang diinstal                                                                                                                                                                         | Skrip, binary, dan file config yang dikemas dengan plugin                        |
| `${CLAUDE_PLUGIN_DATA}` | `~/.claude/plugins/data/<id>/`, dibuat pada referensi pertama dan disimpan di seluruh pembaruan plugin. `<id>` adalah identifier plugin dengan setiap karakter selain huruf, digit, `_`, atau `-` diganti dengan `-` | Dependensi yang diinstal seperti `node_modules`, kode yang dihasilkan, dan cache |
| `${CLAUDE_PROJECT_DIR}` | Root proyek                                                                                                                                                                                                          | Skrip dan file config lokal proyek                                               |

`${CLAUDE_PLUGIN_ROOT}` berubah saat plugin diperbarui, jadi jangan tulis state di sana. Untuk di mana root bergerak dan kapan direktori lama dibersihkan, lihat [halaman loading](/docs/id/plugins/loading).

Saat Anda mencopot plugin dari tempat terakhir diinstal, direktori `${CLAUDE_PLUGIN_DATA}` dihapus kecuali Anda melewatkan [`--keep-data`](/docs/id/plugins/cli-reference).

<h3 id="where-each-variable-resolves">
  Where each variable resolves
</h3>

Di setiap komponen plugin, referensi `${...}` diselesaikan inline di field tertentu, dan beberapa komponen juga menerima variabel di lingkungan proses mereka:

| Plugin component                  | Fields where `${...}` resolves              | Exported to the process                                                                            |
| :-------------------------------- | :------------------------------------------ | :------------------------------------------------------------------------------------------------- |
| Hook commands                     | Di mana saja dalam `command` dan `args`     | `CLAUDE_PLUGIN_ROOT`, `CLAUDE_PLUGIN_DATA`, `CLAUDE_PROJECT_DIR`, dan `CLAUDE_PLUGIN_OPTION_<KEY>` |
| Monitor commands                  | Di mana saja dalam `command`                | Tidak diekspor                                                                                     |
| MCP `stdio` servers               | `command`, `args`, `env`                    | `CLAUDE_PLUGIN_ROOT`, `CLAUDE_PLUGIN_DATA`                                                         |
| MCP `http`, `sse`, `ws` servers   | `url`, `headers`, `headersHelper`           | Tidak berlaku                                                                                      |
| LSP servers                       | `command`, `args`, `env`, `workspaceFolder` | `CLAUDE_PLUGIN_ROOT`, `CLAUDE_PLUGIN_DATA`, `CLAUDE_PROJECT_DIR`                                   |
| Skill, command, and agent content | Di mana saja dalam badan Markdown           | Tidak berlaku                                                                                      |

Variabel tidak ada dalam lingkungan perintah yang Claude jalankan melalui tool Bash, dalam sesi utama atau dalam subagent. Dalam konten skill, command, dan agent, tulis referensi `${...}` dalam badan Markdown sebagai gantinya, dan Claude Code mensubstitusi path inline saat memuat konten.

<h3 id="quoting-and-path-separators">
  Quoting and path separators
</h3>

Simpan setiap path yang disubstitusi sebagai argumen tunggal:

* **Hook commands**: gunakan [exec form](/docs/id/hooks#exec-form-and-shell-form) dengan `args` sehingga setiap path adalah satu argumen tanpa quoting
* **Shell-form hooks dan monitor commands**: bungkus variabel dalam tanda kutip ganda sehingga path dengan spasi tetap satu kata

Hook bentuk shell ini menjalankan skrip yang dikemas dengan plugin:

```json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "\"${CLAUDE_PLUGIN_ROOT}\"/scripts/process.sh"
          }
        ]
      }
    ]
  }
}
```

Di Windows, path yang disubstitusi menggunakan garis miring ke depan sehingga shell tidak membaca garis miring terbalik sebagai escape.

<h2 id="standard-layout">
  Standard layout
</h2>

Setiap tipe komponen memiliki lokasi default di bawah root plugin, digunakan saat manifest tidak menunjuk ke tempat lain.

| Component     | Default location             | Contents                                                                                                                                                                                                                                                                                                                                   |
| :------------ | :--------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Manifest      | `.claude-plugin/plugin.json` | Metadata dan konfigurasi plugin. Opsional                                                                                                                                                                                                                                                                                                  |
| Skills        | `skills/`                    | Satu `<name>/SKILL.md` per skill. Plugin dengan `SKILL.md` di rootnya, tidak ada `skills/`, dan tidak ada kunci `skills` dimuat sebagai skill tunggal                                                                                                                                                                                      |
| Commands      | `commands/`                  | File perintah Markdown datar. Lebih suka `skills/` untuk plugin baru                                                                                                                                                                                                                                                                       |
| Agents        | `agents/`                    | File Markdown agent. Subfolder adalah bagian dari [nama agent](/docs/id/plugins/components#agents)                                                                                                                                                                                                                                              |
| Hooks         | `hooks/hooks.json`           | Konfigurasi hook                                                                                                                                                                                                                                                                                                                           |
| MCP servers   | `.mcp.json`                  | Definisi server MCP                                                                                                                                                                                                                                                                                                                        |
| LSP servers   | `.lsp.json`                  | Konfigurasi server LSP                                                                                                                                                                                                                                                                                                                     |
| Output styles | `output-styles/`             | File gaya output Markdown                                                                                                                                                                                                                                                                                                                  |
| Workflows     | `workflows/`                 | File workflow `.js`                                                                                                                                                                                                                                                                                                                        |
| Themes        | `themes/`                    | File tema JSON                                                                                                                                                                                                                                                                                                                             |
| Monitors      | `monitors/monitors.json`     | Array monitors                                                                                                                                                                                                                                                                                                                             |
| Executables   | `bin/`                       | File di sini ada di `PATH` tool Bash saat plugin diaktifkan, jadi Claude menjalankannya sebagai perintah bare. claude.ai dan Cowork tidak menginstal plugin yang memiliki direktori ini, termasuk yang Anda [distribusikan melalui pengaturan organisasi claude.ai](/docs/id/plugins/host-marketplace#distribute-through-organization-settings) |
| Settings      | `settings.json`              | Default `agent` dan `subagentStatusLine` diterapkan saat plugin diaktifkan                                                                                                                                                                                                                                                                 |

Plugin yang menggunakan setiap lokasi default, ditambah folder `scripts/` yang dipanggil hook-nya, diatur seperti ini:

```text theme={null}
deploy-tools/
├── .claude-plugin/
│   └── plugin.json
├── skills/
│   └── deploy/
│       └── SKILL.md
├── commands/
│   └── status.md
├── agents/
│   └── reviewer.md
├── hooks/
│   └── hooks.json
├── monitors/
│   └── monitors.json
├── output-styles/
│   └── terse.md
├── themes/
│   └── dracula.json
├── workflows/
│   └── release-audit.js
├── bin/
│   └── deploy-tool
├── scripts/
│   └── format.sh
├── settings.json
├── .mcp.json
└── .lsp.json
```

Untuk mengklik melalui tata letak ini dan membaca apa yang dilakukan setiap file, buka [plugin explorer](/docs/id/plugins/components#explore-the-plugin-directory).

`CLAUDE.md` di root plugin tidak dimuat sebagai konteks, dan `claude plugin validate` memperingatkan saat menemukannya. Untuk menyertakan instruksi yang dimuat ke dalam konteks Claude, letakkan di skill.

<h2 id="marketplace-entries-and-the-manifest">
  Marketplace entries and the manifest
</h2>

Entri [marketplace](/docs/id/plugins/marketplace-reference) menerima setiap field di halaman ini bersama dengan [field-nya sendiri](/docs/id/plugins/marketplace-reference#plugin-entries), termasuk `strict`.

Field `strict` memutuskan apakah entri dapat menambahkan komponen ke plugin yang memiliki `plugin.json`-nya sendiri. Default ke `true`.

<h3 id="how-entry-fields-combine-with-plugin-json">
  How entry fields combine with `plugin.json`
</h3>

Entri baik berfungsi sebagai manifest, menambahkan komponen ke dalamnya, atau berkonflik dengannya:

* **Tidak ada `plugin.json`**: entri adalah manifest, terlepas dari `strict`. Entry `hooks` dimuat hanya dalam bentuk objek inline. Untuk path file atau array di sana, tab **Errors** `/plugin` menampilkan error `not yet supported in a marketplace entry`
* **`plugin.json` ada, `strict` tidak diatur atau `true`**: Claude Code memuat manifest dan menambahkan `commands`, `agents`, `skills`, `outputStyles`, dan `themes` entri ke dalamnya. Untuk `hooks`, matcher entri untuk event menggantikan matcher manifest untuk event yang sama, dan event hanya manifest yang mendeklarasikan tetap miliknya
* **`plugin.json` ada, `strict: false`**: entri yang mendeklarasikan salah satu dari `commands`, `agents`, `skills`, `hooks`, `outputStyles`, atau `themes` adalah konflik, dan plugin gagal dimuat dengan `Plugin <name> has conflicting manifests`

Saat [entri marketplace yang `source`-nya adalah root marketplace](/docs/id/plugins/marketplace-reference) mencantumkan subdirektori `skills` tertentu, hanya subdirektori tersebut yang dimuat, dan direktori default `skills/` plugin tidak dipindai. Kunci `skills` dalam manifest sebagai gantinya [menambah default](#how-each-key-combines-with-its-default-location).

<h3 id="metadata-precedence">
  Metadata precedence
</h3>

Beberapa field metadata memiliki preseden tetap terlepas dari `strict`:

* **`defaultEnabled` dan display fields**: `defaultEnabled` entri dan [display fields](/docs/id/plugins/marketplace-reference#entry-and-plugin-json)-nya seperti `displayName` menimpakan manifest
* **`version`**: `version` manifest menimpakan entri
* **`name`**: saat entri mencantumkan plugin di bawah `name` berbeda dari manifest, `enabledPlugins` menggunakan nama entri, dan komponen di-namespace di bawah nama manifest

Untuk tabel preseden lengkap, lihat [Strict mode](/docs/id/plugins/marketplace-reference).

<h2 id="next-steps">
  Next steps
</h2>

* [Tambahkan komponen ke plugin](/docs/id/plugins/components): apa yang dilakukan setiap komponen saat runtime, dengan contoh yang divalidasi
* [Referensi marketplace](/docs/id/plugins/marketplace-reference): field entri yang dapat ditetapkan marketplace untuk plugin Anda
* [Referensi perintah plugin](/docs/id/plugins/cli-reference#plugin-validate): flag dan output `claude plugin validate`
* [Troubleshoot plugins](/docs/id/plugins/troubleshooting#claude-plugin-validate-reports-errors): setiap pesan validasi dengan perbaikannya
