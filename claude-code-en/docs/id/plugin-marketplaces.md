> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Buat dan distribusikan marketplace plugin

> Bangun dan host marketplace plugin untuk mendistribusikan ekstensi Claude Code di seluruh tim dan komunitas.

Sebuah **marketplace plugin** adalah katalog yang memungkinkan Anda mendistribusikan plugin kepada orang lain. Marketplace menyediakan penemuan terpusat, pelacakan versi, pembaruan otomatis, dan dukungan untuk berbagai jenis sumber, termasuk repositori git dan jalur lokal. Panduan ini menunjukkan cara membuat marketplace Anda sendiri untuk berbagi plugin dengan tim atau komunitas Anda.

Mencari cara memasang plugin dari marketplace yang sudah ada? Lihat [Temukan dan pasang plugin yang sudah dibuat](/docs/id/discover-plugins).

<h2 id="overview">
  Ikhtisar
</h2>

Membuat dan mendistribusikan marketplace melibatkan:

1. **Membuat plugin**: bangun satu atau lebih plugin dengan skills, agents, hooks, MCP servers, atau LSP servers. Panduan ini mengasumsikan Anda sudah memiliki plugin untuk didistribusikan; lihat [Buat plugin](/docs/id/plugins) untuk detail tentang cara membuat plugin.
2. **Membuat file marketplace**: tentukan `marketplace.json` yang mencantumkan plugin Anda dan di mana menemukannya. Lihat [Buat file marketplace](#create-the-marketplace-file).
3. **Host marketplace**: dorong ke GitHub, GitLab, atau host git lainnya. Lihat [Host dan distribusikan marketplace](#host-and-distribute-marketplaces).
4. **Bagikan dengan pengguna**: pengguna menambahkan marketplace Anda dengan `/plugin marketplace add` dan memasang plugin individual. Lihat [Temukan dan pasang plugin](/docs/id/discover-plugins).

Setelah marketplace Anda aktif, Anda dapat memperbaruinya dengan mendorong perubahan ke repositori Anda. Pengguna menyegarkan salinan lokal mereka dengan `/plugin marketplace update`.

<h2 id="walkthrough-create-a-local-marketplace">
  Panduan: buat marketplace lokal
</h2>

Contoh ini membuat marketplace dengan satu plugin: skill `quality-review` untuk ulasan kode. Anda akan membuat struktur direktori, menambahkan skill, membuat manifest plugin dan katalog marketplace, kemudian memasang dan mengujinya.

<Steps>
  <Step title="Buat struktur direktori">
    ```bash theme={null}
    mkdir -p my-marketplace/.claude-plugin
    mkdir -p my-marketplace/plugins/quality-review-plugin/.claude-plugin
    mkdir -p my-marketplace/plugins/quality-review-plugin/skills/quality-review
    ```
  </Step>

  <Step title="Buat skill">
    Buat file `SKILL.md` yang mendefinisikan apa yang dilakukan skill `quality-review`.

    ```markdown my-marketplace/plugins/quality-review-plugin/skills/quality-review/SKILL.md theme={null}
    ---
    description: Review code for bugs, security, and performance
    ---

    Review the code I've selected or the recent changes for:
    - Potential bugs or edge cases
    - Security concerns
    - Performance issues
    - Readability improvements

    Be concise and actionable.
    ```
  </Step>

  <Step title="Buat manifest plugin">
    Buat file `plugin.json` yang mendeskripsikan plugin. Manifest berada di direktori `.claude-plugin/`.

    ```json my-marketplace/plugins/quality-review-plugin/.claude-plugin/plugin.json theme={null}
    {
      "name": "quality-review-plugin",
      "description": "Adds a quality-review skill for quick code reviews",
      "version": "1.0.0"
    }
    ```

    <Note>
      Menetapkan `version` berarti pengguna hanya menerima pembaruan ketika Anda mengubah bidang ini, jadi tingkatkan pada setiap rilis. Jika Anda menghilangkan `version` dan menghosting marketplace ini di git, setiap commit secara otomatis dihitung sebagai versi baru. Lihat [Version resolution](#version-resolution-and-release-channels) untuk memilih pendekatan yang tepat.
    </Note>
  </Step>

  <Step title="Buat file marketplace">
    Buat katalog marketplace yang mencantumkan plugin Anda.

    ```json my-marketplace/.claude-plugin/marketplace.json theme={null}
    {
      "name": "my-plugins",
      "owner": {
        "name": "Your Name"
      },
      "plugins": [
        {
          "name": "quality-review-plugin",
          "source": "./plugins/quality-review-plugin",
          "description": "Adds a quality-review skill for quick code reviews"
        }
      ]
    }
    ```
  </Step>

  <Step title="Tambahkan dan pasang">
    Tambahkan marketplace dan pasang plugin.

    ```shell theme={null}
    /plugin marketplace add ./my-marketplace
    /plugin install quality-review-plugin@my-plugins
    ```
  </Step>

  <Step title="Coba">
    Pilih beberapa kode di editor Anda dan jalankan skill baru Anda. Plugin skills memiliki namespace dengan nama plugin.

    ```shell theme={null}
    /quality-review-plugin:quality-review
    ```
  </Step>
</Steps>

Untuk mempelajari lebih lanjut tentang apa yang dapat dilakukan plugin, termasuk hooks, agents, MCP servers, dan LSP servers, lihat [Plugins](/docs/id/plugins).

<Note>
  **Cara plugin dipasang**: Ketika pengguna memasang plugin, Claude Code menyalin direktori plugin ke lokasi cache. Ini berarti plugin tidak dapat mereferensikan file di luar direktorinya menggunakan jalur seperti `../shared-utils`, karena file tersebut tidak akan disalin.

  Jika Anda perlu berbagi file di seluruh plugin, gunakan symlink. Lihat [Plugin caching and file resolution](/docs/id/plugins-reference#plugin-caching-and-file-resolution) untuk detail.
</Note>

<h2 id="create-the-marketplace-file">
  Buat file marketplace
</h2>

Buat `.claude-plugin/marketplace.json` di root repositori Anda. File ini mendefinisikan nama marketplace Anda, informasi pemilik, dan daftar plugin dengan sumbernya.

Setiap entri plugin memerlukan minimal `name` dan `source` yang memberitahu Claude Code di mana mengambilnya. Lihat [skema lengkap](#marketplace-schema) di bawah untuk semua field yang tersedia.

```json theme={null}
{
  "name": "company-tools",
  "owner": {
    "name": "DevTools Team",
    "email": "devtools@example.com"
  },
  "plugins": [
    {
      "name": "code-formatter",
      "source": "./plugins/formatter",
      "description": "Automatic code formatting on save",
      "version": "2.1.0",
      "author": {
        "name": "DevTools Team"
      }
    },
    {
      "name": "deployment-tools",
      "source": {
        "source": "github",
        "repo": "company/deploy-plugin"
      },
      "description": "Deployment automation tools"
    }
  ]
}
```

<h2 id="marketplace-schema">
  Skema marketplace
</h2>

<h3 id="required-fields">
  Field yang diperlukan
</h3>

| Field     | Type   | Deskripsi                                                                                                                                                                                                                                                                                                                                                                                                                                                                       | Contoh         |
| :-------- | :----- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | :------------- |
| `name`    | string | Identifier marketplace (kebab-case, tanpa spasi). Ini menghadap publik: pengguna melihatnya saat memasang plugin (misalnya, `/plugin install my-tool@your-marketplace`). Setiap pengguna hanya dapat mendaftarkan satu marketplace per nama: menambahkan marketplace kedua dengan nama yang sama menggantikan yang pertama. Untuk menerbitkan beberapa plugin di bawah satu nama marketplace, daftarkan semuanya dalam satu [`marketplace.json`](#create-the-marketplace-file). | `"acme-tools"` |
| `owner`   | object | Informasi pengelola marketplace ([lihat field di bawah](#owner-fields))                                                                                                                                                                                                                                                                                                                                                                                                         |                |
| `plugins` | array  | Daftar plugin yang tersedia                                                                                                                                                                                                                                                                                                                                                                                                                                                     | Lihat di bawah |

<Note>
  **Nama yang dicadangkan**: Nama marketplace berikut dicadangkan untuk penggunaan resmi Anthropic dan tidak dapat digunakan oleh marketplace pihak ketiga: `claude-code-marketplace`, `claude-code-plugins`, `claude-plugins-official`, `claude-plugins-community`, `claude-community`, `anthropic-marketplace`, `anthropic-plugins`, `agent-skills`, `anthropic-agent-skills`, `knowledge-work-plugins`, `life-sciences`, `claude-for-legal`, `claude-for-financial-services`, `financial-services-plugins`, `first-party-plugins`, `healthcare`. Nama yang meniru marketplace resmi, seperti `official-claude-plugins` atau `anthropic-plugins-v2`, juga diblokir. Pencadangan nama-nama ini mencegah marketplace pihak ketiga menyajikan dirinya sebagai sumber yang diterbitkan Anthropic.

  Claude Code memeriksa kembali nama yang dicadangkan setiap kali memuat marketplace, bukan hanya saat Anda menambahkan satu. Marketplace yang terdaftar di bawah salah satu nama ini sebelum nama menjadi dicadangkan berhenti memuat dan melaporkan bahwa itu [terdaftar dari sumber yang tidak terpercaya](/docs/id/errors#marketplace-is-registered-from-an-untrusted-source). Hapus marketplace itu dan tambahkan kembali dari sumber Anthropic resmi. Marketplace pihak ketiga yang terpengaruh oleh nama yang baru dicadangkan memuat lagi segera setelah Anda menambahkannya kembali dengan nama yang berbeda. Sebelum v2.1.205, `first-party-plugins` dan `healthcare` tidak dicadangkan, dan marketplace yang sudah terdaftar di bawah nama yang dicadangkan terus memuat.
</Note>

<h3 id="owner-fields">
  Field pemilik
</h3>

| Field   | Type   | Diperlukan | Deskripsi                    |
| :------ | :----- | :--------- | :--------------------------- |
| `name`  | string | Ya         | Nama pengelola atau tim      |
| `email` | string | Tidak      | Email kontak untuk pengelola |

<h3 id="optional-fields">
  Field opsional
</h3>

| Field                                 | Type   | Deskripsi                                                                                                                                                                                                                                                                                                                                            |
| :------------------------------------ | :----- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `$schema`                             | string | URL JSON Schema untuk autocomplete dan validasi editor. Claude Code mengabaikan field ini saat waktu muat.                                                                                                                                                                                                                                           |
| `description`                         | string | Deskripsi marketplace singkat                                                                                                                                                                                                                                                                                                                        |
| `version`                             | string | Versi manifest marketplace                                                                                                                                                                                                                                                                                                                           |
| `metadata.pluginRoot`                 | string | Direktori dasar yang ditambahkan ke jalur sumber plugin relatif (misalnya, `"./plugins"` memungkinkan Anda menulis `"source": "formatter"` alih-alih `"source": "./plugins/formatter"`)                                                                                                                                                              |
| `allowCrossMarketplaceDependenciesOn` | array  | Marketplace lain yang plugin di marketplace ini dapat bergantung padanya. Dependensi dari marketplace yang tidak tercantum di sini diblokir saat instalasi. Lihat [Bergantung pada plugin dari marketplace lain](/docs/id/plugin-dependencies#depend-on-a-plugin-from-another-marketplace).                                                               |
| `renames`                             | object | Peta dari nama plugin `name` sebelumnya ke nama saat ini, atau ke `null` jika plugin dihapus. Memungkinkan pengguna yang ada untuk bermigrasi secara otomatis saat Anda mengganti nama atau menghapus entri di `plugins`. Lihat [Mengganti nama atau menghapus plugin](#rename-or-remove-a-plugin). Memerlukan Claude Code v2.1.193 atau lebih baru. |

`description` dan `version` juga diterima di bawah `metadata` untuk kompatibilitas mundur.

<h2 id="plugin-entries">
  Entri plugin
</h2>

Setiap entri plugin dalam array `plugins` mendeskripsikan plugin dan di mana menemukannya. Anda dapat menyertakan field apa pun dari [skema manifest plugin](/docs/id/plugins-reference#plugin-manifest-schema), seperti `description`, `version`, `author`, `commands`, dan `hooks`, ditambah field khusus marketplace ini: `source`, `category`, `tags`, `strict`, dan `relevance`.

<h3 id="required-fields-2">
  Field yang diperlukan
</h3>

| Field    | Type           | Deskripsi                                                                                                                                                 |
| :------- | :------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`   | string         | Identifier plugin (kebab-case, tanpa spasi). Ini menghadap publik: pengguna melihatnya saat memasang (misalnya, `/plugin install my-plugin@marketplace`). |
| `source` | string\|object | Di mana mengambil plugin (lihat [Plugin sources](#plugin-sources) di bawah)                                                                               |

<h3 id="optional-plugin-fields">
  Field plugin opsional
</h3>

**Field metadata standar:**

| Field            | Type    | Deskripsi                                                                                                                                                                                                                                                                                                                                           |
| :--------------- | :------ | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `displayName`    | string  | Nama yang dapat dibaca manusia ditampilkan di permukaan UI. Kembali ke `name` saat dihilangkan. Dapat berisi spasi dan huruf apa pun. Tidak digunakan untuk namespacing atau pencarian. Memerlukan Claude Code v2.1.143 atau lebih baru.                                                                                                            |
| `description`    | string  | Deskripsi plugin singkat                                                                                                                                                                                                                                                                                                                            |
| `version`        | string  | Versi plugin. Jika diatur (di sini atau di `plugin.json`), plugin disematkan ke string ini dan pengguna hanya menerima pembaruan saat berubah. Hilangkan untuk kembali ke SHA commit git. Lihat [Version resolution](#version-resolution-and-release-channels).                                                                                     |
| `author`         | object  | Informasi penulis plugin (`name` diperlukan, `email` opsional)                                                                                                                                                                                                                                                                                      |
| `homepage`       | string  | URL homepage atau dokumentasi plugin                                                                                                                                                                                                                                                                                                                |
| `repository`     | string  | URL repositori kode sumber                                                                                                                                                                                                                                                                                                                          |
| `license`        | string  | Identifier lisensi SPDX (misalnya, MIT, Apache-2.0)                                                                                                                                                                                                                                                                                                 |
| `keywords`       | array   | Tag untuk penemuan dan kategorisasi plugin                                                                                                                                                                                                                                                                                                          |
| `category`       | string  | Kategori plugin untuk organisasi                                                                                                                                                                                                                                                                                                                    |
| `tags`           | array   | Tag untuk kemudahan pencarian                                                                                                                                                                                                                                                                                                                       |
| `strict`         | boolean | Mengontrol apakah `plugin.json` adalah otoritas untuk definisi komponen (default: true). Lihat [Strict mode](#strict-mode) di bawah.                                                                                                                                                                                                                |
| `relevance`      | object  | Sinyal yang memberi tahu Claude Code kapan harus menyarankan plugin ini kepada pengguna. Hanya berlaku untuk marketplace yang diizinkan administrator dalam pengaturan terkelola. Lihat [Recommend plugins for your org](/docs/id/plugin-relevance). Memerlukan Claude Code v2.1.152 atau lebih baru.                                                    |
| `defaultEnabled` | boolean | Apakah plugin diaktifkan setelah pemasangan (default: true). Atur ke `false` untuk memasang plugin yang dinonaktifkan sampai pengguna memilih untuk mengaktifkannya. Mengambil alih field yang sama di `plugin.json` plugin. Lihat [Default enablement](/docs/id/plugins-reference#default-enablement). Memerlukan Claude Code v2.1.154 atau lebih baru. |

**Field konfigurasi komponen:**

| Field        | Type           | Deskripsi                                                     |
| :----------- | :------------- | :------------------------------------------------------------ |
| `skills`     | string\|array  | Jalur kustom ke direktori skill yang berisi `<name>/SKILL.md` |
| `commands`   | string\|array  | Jalur kustom ke file skill `.md` datar atau direktori         |
| `agents`     | string\|array  | Jalur kustom ke file agent                                    |
| `hooks`      | string\|object | Konfigurasi hooks kustom atau jalur ke file hooks             |
| `mcpServers` | string\|object | Konfigurasi MCP server atau jalur ke config MCP               |
| `lspServers` | string\|object | Konfigurasi LSP server atau jalur ke config LSP               |

<h2 id="plugin-sources">
  Plugin sources
</h2>

Plugin sources memberitahu Claude Code di mana mengambil setiap plugin individual yang tercantum di marketplace Anda. Ini diatur dalam field `source` dari setiap entri plugin di `marketplace.json`.

Setelah Claude Code mengklon atau mengunduh plugin ke mesin lokal, plugin disalin ke cache plugin lokal yang tersimpan di `~/.claude/plugins/cache`.

| Source        | Type                                | Fields                             | Catatan                                                                                                                                              |
| ------------- | ----------------------------------- | ---------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------- |
| Relative path | `string` (misalnya `"./my-plugin"`) | none                               | Direktori lokal dalam repo marketplace. Harus dimulai dengan `./`. Diselesaikan relatif terhadap root marketplace, bukan direktori `.claude-plugin/` |
| `github`      | object                              | `repo`, `ref?`, `sha?`             |                                                                                                                                                      |
| `url`         | object                              | `url`, `ref?`, `sha?`              | Sumber URL Git                                                                                                                                       |
| `git-subdir`  | object                              | `url`, `path`, `ref?`, `sha?`      | Subdirektori dalam repo git. Mengklon secara sparse untuk meminimalkan bandwidth untuk monorepo                                                      |
| `npm`         | object                              | `package`, `version?`, `registry?` | Dipasang via `npm install`                                                                                                                           |

<Note>
  **Marketplace sources vs plugin sources**: Ini adalah konsep berbeda yang mengontrol hal berbeda.

  * **Marketplace source**: di mana mengambil katalog `marketplace.json` itu sendiri. Diatur ketika pengguna menjalankan `/plugin marketplace add` atau dalam pengaturan `extraKnownMarketplaces`. Mendukung `ref` (branch/tag) tetapi bukan `sha`.
  * **Plugin source**: di mana mengambil plugin individual yang tercantum di marketplace. Diatur dalam field `source` dari setiap entri plugin di dalam `marketplace.json`. Mendukung baik `ref` (branch/tag) maupun `sha` (commit yang tepat).

  Misalnya, marketplace yang dihosting di `acme-corp/plugin-catalog` (marketplace source) dapat mencantumkan plugin yang diambil dari `acme-corp/code-formatter` (plugin source). Marketplace source dan plugin source menunjuk ke repositori berbeda dan disematkan secara independen.
</Note>

Jenis sumber berbasis git di bawah ini adalah `github`, `url`, dan `git-subdir`. Ketika baik `ref` maupun `sha` diatur pada salah satu dari mereka, `sha` adalah pin yang efektif. Claude Code mengambil dan melakukan checkout pada commit yang disematkan secara langsung.

Pada sebagian besar host git, termasuk GitHub, GitLab, dan Bitbucket, ini berarti instalasi berhasil bahkan jika branch atau tag yang dinamai oleh `ref` telah dihapus upstream, selama commit masih dapat dijangkau dari repositori. Beberapa server, seperti AWS CodeCommit, tidak mendukung pengambilan commit berdasarkan SHA. Di server tersebut `ref` masih harus ada dan commit yang disematkan harus dapat dijangkau darinya.

<h3 id="relative-paths">
  Jalur relatif
</h3>

Untuk plugin di repositori yang sama, gunakan jalur yang dimulai dengan `./`:

```json theme={null}
{
  "name": "my-plugin",
  "source": "./plugins/my-plugin"
}
```

Jalur diselesaikan relatif terhadap root marketplace, yang merupakan direktori yang berisi `.claude-plugin/`. Dalam contoh di atas, `./plugins/my-plugin` menunjuk ke `<repo>/plugins/my-plugin`, meskipun `marketplace.json` berada di `<repo>/.claude-plugin/marketplace.json`. Jangan gunakan `../` untuk mereferensikan jalur di luar root marketplace.

<Note>
  Jalur relatif diselesaikan terhadap salinan lokal marketplace, jadi mereka berfungsi ketika pengguna menambahkan marketplace Anda dari sumber git atau direktori lokal. Jika pengguna menambahkan marketplace Anda melalui URL langsung ke file `marketplace.json`, jalur relatif tidak akan diselesaikan, karena hanya file itu yang diunduh. Untuk distribusi berbasis URL, gunakan sumber GitHub, npm, atau URL git sebagai gantinya. Lihat [Troubleshooting](#plugins-with-relative-paths-fail-in-url-based-marketplaces) untuk detail.
</Note>

<h3 id="github-repositories">
  Repositori GitHub
</h3>

```json theme={null}
{
  "name": "github-plugin",
  "source": {
    "source": "github",
    "repo": "owner/plugin-repo"
  }
}
```

Anda dapat menyematkan ke branch, tag, atau commit tertentu:

```json theme={null}
{
  "name": "github-plugin",
  "source": {
    "source": "github",
    "repo": "owner/plugin-repo",
    "ref": "v2.0.0",
    "sha": "a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0"
  }
}
```

| Field  | Type   | Deskripsi                                                                        |
| :----- | :----- | :------------------------------------------------------------------------------- |
| `repo` | string | Diperlukan. Repositori GitHub dalam format `owner/repo`                          |
| `ref`  | string | Opsional. Branch atau tag Git (default ke branch default repositori)             |
| `sha`  | string | Opsional. SHA commit git 40-karakter penuh untuk menyematkan ke versi yang tepat |

<h3 id="git-repositories">
  Repositori Git
</h3>

```json theme={null}
{
  "name": "git-plugin",
  "source": {
    "source": "url",
    "url": "https://gitlab.com/team/plugin.git"
  }
}
```

Anda dapat menyematkan ke branch, tag, atau commit tertentu:

```json theme={null}
{
  "name": "git-plugin",
  "source": {
    "source": "url",
    "url": "https://gitlab.com/team/plugin.git",
    "ref": "main",
    "sha": "a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0"
  }
}
```

| Field | Type   | Deskripsi                                                                                                                                                  |
| :---- | :----- | :--------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `url` | string | Diperlukan. URL repositori git lengkap (`https://` atau `git@`). Akhiran `.git` opsional, jadi URL Azure DevOps dan AWS CodeCommit tanpa akhiran berfungsi |
| `ref` | string | Opsional. Branch atau tag Git (default ke branch default repositori)                                                                                       |
| `sha` | string | Opsional. SHA commit git 40-karakter penuh untuk menyematkan ke versi yang tepat                                                                           |

<h3 id="git-subdirectories">
  Subdirektori Git
</h3>

Gunakan `git-subdir` untuk menunjuk ke plugin yang berada di dalam subdirektori repositori git. Claude Code menggunakan klon parsial dan sparse untuk mengambil hanya subdirektori, meminimalkan bandwidth untuk monorepo besar.

```json theme={null}
{
  "name": "my-plugin",
  "source": {
    "source": "git-subdir",
    "url": "https://github.com/acme-corp/monorepo.git",
    "path": "tools/claude-plugin"
  }
}
```

Anda dapat menyematkan ke branch, tag, atau commit tertentu:

```json theme={null}
{
  "name": "my-plugin",
  "source": {
    "source": "git-subdir",
    "url": "https://github.com/acme-corp/monorepo.git",
    "path": "tools/claude-plugin",
    "ref": "v2.0.0",
    "sha": "a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0"
  }
}
```

Field `url` juga menerima shorthand GitHub (`owner/repo`) atau URL SSH (`git@github.com:owner/repo.git`).

| Field  | Type   | Deskripsi                                                                                        |
| :----- | :----- | :----------------------------------------------------------------------------------------------- |
| `url`  | string | Diperlukan. URL repositori Git, shorthand GitHub `owner/repo`, atau URL SSH                      |
| `path` | string | Diperlukan. Jalur subdirektori dalam repo yang berisi plugin (misalnya, `"tools/claude-plugin"`) |
| `ref`  | string | Opsional. Branch atau tag Git (default ke branch default repositori)                             |
| `sha`  | string | Opsional. SHA commit git 40-karakter penuh untuk menyematkan ke versi yang tepat                 |

<h3 id="npm-packages">
  Paket npm
</h3>

Plugin yang didistribusikan sebagai paket npm dipasang menggunakan `npm install`. Ini berfungsi dengan paket apa pun di registry npm publik atau registry pribadi yang dihosting tim Anda.

```json theme={null}
{
  "name": "my-npm-plugin",
  "source": {
    "source": "npm",
    "package": "@acme/claude-plugin"
  }
}
```

Untuk menyematkan ke versi tertentu, tambahkan field `version`:

```json theme={null}
{
  "name": "my-npm-plugin",
  "source": {
    "source": "npm",
    "package": "@acme/claude-plugin",
    "version": "2.1.0"
  }
}
```

Untuk memasang dari registry pribadi atau internal, tambahkan field `registry`:

```json theme={null}
{
  "name": "my-npm-plugin",
  "source": {
    "source": "npm",
    "package": "@acme/claude-plugin",
    "version": "^2.0.0",
    "registry": "https://npm.example.com"
  }
}
```

| Field      | Type   | Deskripsi                                                                              |
| :--------- | :----- | :------------------------------------------------------------------------------------- |
| `package`  | string | Diperlukan. Nama paket atau paket scoped (misalnya, `@org/plugin`)                     |
| `version`  | string | Opsional. Versi atau rentang versi (misalnya, `2.1.0`, `^2.0.0`, `~1.5.0`)             |
| `registry` | string | Opsional. URL registry npm kustom. Default ke registry npm sistem (biasanya npmjs.org) |

<h3 id="advanced-plugin-entries">
  Entri plugin lanjutan
</h3>

Contoh ini menunjukkan entri plugin menggunakan banyak field opsional, termasuk jalur kustom untuk commands, agents, hooks, dan MCP servers:

```json theme={null}
{
  "name": "enterprise-tools",
  "source": {
    "source": "github",
    "repo": "company/enterprise-plugin"
  },
  "description": "Enterprise workflow automation tools",
  "version": "2.1.0",
  "author": {
    "name": "Enterprise Team",
    "email": "enterprise@example.com"
  },
  "homepage": "https://docs.example.com/plugins/enterprise-tools",
  "repository": "https://github.com/company/enterprise-plugin",
  "license": "MIT",
  "keywords": ["enterprise", "workflow", "automation"],
  "category": "productivity",
  "commands": [
    "./commands/core/",
    "./commands/enterprise/",
    "./commands/experimental/preview.md"
  ],
  "agents": ["./agents/security-reviewer.md", "./agents/compliance-checker.md"],
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Write|Edit",
        "hooks": [
          {
            "type": "command",
            "command": "${CLAUDE_PLUGIN_ROOT}/scripts/validate.sh"
          }
        ]
      }
    ]
  },
  "mcpServers": {
    "enterprise-db": {
      "command": "${CLAUDE_PLUGIN_ROOT}/servers/db-server",
      "args": ["--config", "${CLAUDE_PLUGIN_ROOT}/config.json"]
    }
  },
  "strict": false
}
```

Hal-hal penting untuk diperhatikan:

* **`commands` dan `agents`**: Anda dapat menentukan beberapa direktori atau file individual. Jalur relatif terhadap root plugin.
* **`${CLAUDE_PLUGIN_ROOT}`**: Gunakan variabel ini dalam hooks dan config MCP server untuk mereferensikan file dalam direktori instalasi plugin. Ini diperlukan karena plugin disalin ke lokasi cache saat dipasang.
  * Lihat [tabel substitusi](/docs/id/plugins-reference#environment-variables) untuk field config mana yang mensubstitusinya per tipe server
  * Untuk dependensi atau state yang harus bertahan pembaruan plugin, gunakan [`${CLAUDE_PLUGIN_DATA}`](/docs/id/plugins-reference#persistent-data-directory) sebagai gantinya
* **`strict: false`**: Karena ini diatur ke false, plugin tidak memerlukan `plugin.json` sendiri. Entri marketplace mendefinisikan semuanya. Lihat [Strict mode](#strict-mode) di bawah.

Secara default, skills plugin dimuat dari direktori `skills/` di bawah `source`-nya. Jalur yang tercantum dalam field `skills` menambah pemindaian itu:

```json theme={null}
"skills": ["./skills/", "./extra-skills/"]
```

Ketika beberapa entri plugin berbagi satu folder `skills/` di root marketplace (`source: "./"`), cantumkan subdirektori spesifik sebagai gantinya sehingga setiap entri hanya memuat skills-nya sendiri:

```json theme={null}
"source": "./",
"skills": ["./skills/code-review", "./skills/docs"]
```

Dengan sumber root marketplace, jalur yang tercantum adalah set lengkap untuk entri itu, dan direktori lain di folder `skills/` bersama tidak dimuat. Mencantumkan `./skills/` itu sendiri, atau root plugin, menjaga pemindaian penuh. Jika tidak ada jalur yang tercantum ada, pemindaian default berjalan sebagai gantinya.

<h3 id="strict-mode">
  Strict mode
</h3>

Field `strict` mengontrol apakah `plugin.json` adalah otoritas untuk definisi komponen (skills, agents, hooks, MCP servers, output styles).

| Value            | Perilaku                                                                                                                                                      |
| :--------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `true` (default) | `plugin.json` adalah otoritas. Entri marketplace dapat melengkapinya dengan komponen tambahan, dan kedua sumber digabungkan.                                  |
| `false`          | Entri marketplace adalah definisi lengkap. Jika plugin juga memiliki `plugin.json` yang mendeklarasikan komponen, itu adalah konflik dan plugin gagal dimuat. |

**Kapan menggunakan setiap mode:**

* **`strict: true`**: plugin memiliki `plugin.json` sendiri dan mengelola komponennya sendiri. Entri marketplace dapat menambahkan skills atau hooks tambahan di atas. Ini adalah default dan berfungsi untuk sebagian besar plugin.
* **`strict: false`**: operator marketplace menginginkan kontrol penuh. Repo plugin menyediakan file mentah, dan entri marketplace mendefinisikan file mana yang diekspos sebagai skills, agents, hooks, dll. Berguna ketika marketplace merestruktur atau mengkurasi komponen plugin secara berbeda dari yang dimaksudkan penulis plugin.

<h2 id="host-and-distribute-marketplaces">
  Host dan distribusikan marketplace
</h2>

<h3 id="host-on-github-recommended">
  Host di GitHub (direkomendasikan)
</h3>

GitHub adalah cara yang direkomendasikan untuk host dan distribusikan marketplace:

1. **Buat repositori**: siapkan repositori baru untuk marketplace Anda
2. **Tambahkan file marketplace**: buat `.claude-plugin/marketplace.json` dengan definisi plugin Anda
3. **Bagikan dengan tim**: pengguna menambahkan marketplace Anda dengan `/plugin marketplace add owner/repo`

**Manfaat**: kontrol versi bawaan, pelacakan masalah, dan fitur kolaborasi tim.

<h3 id="host-on-other-git-services">
  Host di layanan git lainnya
</h3>

Layanan hosting git apa pun berfungsi, seperti GitLab, Bitbucket, dan server yang dihosting sendiri. Pengguna menambahkan dengan URL repositori lengkap:

```shell theme={null}
/plugin marketplace add https://gitlab.com/company/plugins.git
```

<h3 id="private-repositories">
  Repositori pribadi
</h3>

Claude Code mendukung pemasangan plugin dari repositori pribadi. Untuk instalasi manual dan pembaruan, Claude Code menggunakan helper kredensial git yang ada, jadi akses HTTPS melalui `gh auth login`, Keychain macOS, atau `git-credential-store` berfungsi sama seperti di terminal Anda. Akses SSH berfungsi selama host sudah ada di file `known_hosts` Anda dan kunci dimuat di `ssh-agent`, karena Claude Code menekan prompt SSH interaktif untuk sidik jari host dan passphrase kunci. Shorthand `owner/repo` GitHub mengklon melalui SSH secara default; atur [`CLAUDE_CODE_PLUGIN_PREFER_HTTPS=1`](/docs/id/env-vars#variables) untuk mengklonnya melalui HTTPS sebagai gantinya.

Pembaruan otomatis latar belakang berjalan berbeda. Secara default, refresh latar belakang menonaktifkan helper kredensial git untuk `git pull`-nya, jadi pull tidak dapat mengautentikasi ke repositori pribadi melalui HTTPS bahkan ketika helper dikonfigurasi. Remote SSH tidak terpengaruh: kunci yang dimuat di `ssh-agent` mengautentikasi pull latar belakang dengan cara yang sama seperti operasi manual. Ketika pull latar belakang gagal, Claude Code kembali ke pengklonaan ulang marketplace dari awal. Pengklonaan ulang memang menggunakan kredensial git yang disimpan, tetapi dapat [time out pada repositori besar](#git-operations-time-out), jadi pembaruan otomatis marketplace pribadi mungkin gagal secara berkala.

Dua pengaturan membuat marketplace pribadi berperilaku dapat diprediksi:

* Atur `CLAUDE_CODE_PLUGIN_KEEP_MARKETPLACE_ON_FAILURE=1` untuk menyimpan klon yang ada ketika pull latar belakang gagal, alih-alih menghapus dan mengklon ulang. Plugin Anda terus bekerja dari status terakhir yang disinkronkan, dan pembaruan manual dengan `/plugin marketplace update` masih pull dengan kredensial Anda.
* Konfigurasikan helper kredensial git, misalnya dengan `gh auth setup-git` untuk GitHub, sehingga fallback pengklonaan ulang dapat mengautentikasi tanpa prompt.

Menetapkan token penyedia seperti `GITHUB_TOKEN` di lingkungan Anda tidak dengan sendirinya mengaktifkan autentikasi latar belakang. Token hanya berlaku melalui helper kredensial yang dikonfigurasi, misalnya helper CLI `gh`, yang membaca `GH_TOKEN` dan `GITHUB_TOKEN`.

Untuk membuat pull latar belakang itu sendiri mengautentikasi melalui HTTPS, konfigurasikan penulisan ulang URL git global. Penulisan ulang menyematkan token dalam URL jarak jauh, jadi berlaku meskipun pull latar belakang menonaktifkan helper kredensial, dan pull yang berhasil melewati fallback pengklonaan ulang. Contoh berikut menulis ulang URL repositori marketplace untuk menyertakan token akses:

```bash theme={null}
git config --global url."https://x-access-token:YOUR_TOKEN@github.com/acme-corp/plugins".insteadOf "https://github.com/acme-corp/plugins"
```

Cakupkan penulisan ulang ke jalur repositori marketplace atau organisasi. Penulisan ulang yang dasarnya hanya host berlaku untuk setiap fetch dan push ke host itu di mesin dan mengganti kredensial normal Anda, termasuk push ke repositori Anda sendiri.

Setiap penyedia mengharapkan nama pengguna berbeda dalam URL yang ditulis ulang, dan cakupan jalur yang sama berlaku untuk setiap penyedia. Untuk server yang dihosting sendiri, ganti nama host dengan nama host server Anda:

| Penyedia  | Bentuk URL yang ditulis ulang                                     |
| :-------- | :---------------------------------------------------------------- |
| GitHub    | `https://x-access-token:YOUR_TOKEN@github.com/acme-corp/plugins`  |
| GitLab    | `https://oauth2:YOUR_TOKEN@gitlab.com/acme-corp/plugins`          |
| Bitbucket | `https://x-token-auth:YOUR_TOKEN@bitbucket.org/acme-corp/plugins` |

Penulisan ulang menyimpan token dalam plaintext di gitconfig Anda, jadi gunakan token dengan akses read-only ke repositori marketplace.

<Note>
  Dalam lingkungan CI/CD, konfigurasikan helper kredensial git sebelum memasang plugin dari repositori pribadi. Di GitHub Actions, ekspor token dengan akses read ke repositori marketplace sebagai `GH_TOKEN`, kemudian jalankan `gh auth setup-git`. Token workflow default hanya dapat mengakses repositori workflow itu sendiri, jadi marketplace pribadi di repositori lain memerlukan token akses pribadi atau token aplikasi. Penulisan ulang URL global yang dikonfigurasi dalam pipeline juga mengautentikasi pull latar belakang secara langsung.
</Note>

<h3 id="test-locally-before-distribution">
  Uji secara lokal sebelum distribusi
</h3>

Uji marketplace Anda secara lokal sebelum berbagi:

```shell theme={null}
/plugin marketplace add ./my-marketplace
/plugin install quality-review-plugin@my-plugins
```

Untuk rangkaian lengkap perintah add (GitHub, URL Git, jalur lokal, URL jarak jauh), lihat [Tambahkan marketplace](/docs/id/discover-plugins#add-marketplaces).

<h3 id="require-marketplaces-for-your-team">
  Wajibkan marketplace untuk tim Anda
</h3>

Anda dapat mengonfigurasi repositori Anda sehingga anggota tim secara otomatis diminta untuk memasang marketplace Anda ketika mereka mempercayai folder proyek. Tambahkan marketplace Anda ke `.claude/settings.json`:

```json theme={null}
{
  "extraKnownMarketplaces": {
    "company-tools": {
      "source": {
        "source": "github",
        "repo": "your-org/claude-plugins"
      }
    }
  }
}
```

Anda juga dapat menentukan plugin mana yang harus diaktifkan secara default:

```json theme={null}
{
  "enabledPlugins": {
    "code-formatter@company-tools": true,
    "deployment-tools@company-tools": true
  }
}
```

Untuk opsi konfigurasi lengkap, lihat [Plugin settings](/docs/id/settings#plugin-settings).

<Note>
  Jika Anda menggunakan sumber `directory` atau `file` lokal dengan jalur relatif, jalur diselesaikan terhadap checkout utama repositori Anda. Ketika Anda menjalankan Claude Code dari git worktree, jalur masih menunjuk ke checkout utama, jadi semua worktrees berbagi lokasi marketplace yang sama. Status marketplace disimpan sekali per pengguna di `~/.claude/plugins/known_marketplaces.json`, bukan per proyek.
</Note>

<h3 id="pre-populate-plugins-for-containers">
  Pra-isi plugin untuk container
</h3>

Untuk image container dan lingkungan CI, Anda dapat pra-isi direktori plugin saat waktu build sehingga Claude Code dimulai dengan marketplace dan plugin yang sudah tersedia, tanpa mengklon apa pun saat runtime. Atur variabel lingkungan `CLAUDE_CODE_PLUGIN_SEED_DIR` untuk menunjuk ke direktori ini.

Untuk melapisi beberapa direktori seed, pisahkan jalur dengan `:` di Unix atau `;` di Windows. Claude Code mencari setiap direktori secara berurutan dan menggunakan seed pertama yang berisi marketplace atau cache plugin yang diberikan.

Direktori seed mencerminkan struktur `~/.claude/plugins`:

```
$CLAUDE_CODE_PLUGIN_SEED_DIR/
  known_marketplaces.json
  marketplaces/<name>/...
  cache/<marketplace>/<plugin>/<version>/...
```

Untuk membangun direktori seed, jalankan Claude Code sekali selama image build, pasang plugin yang Anda butuhkan, kemudian salin direktori `~/.claude/plugins` yang dihasilkan ke image Anda dan tunjukkan `CLAUDE_CODE_PLUGIN_SEED_DIR` ke sana.

Untuk melewati langkah copy, atur `CLAUDE_CODE_PLUGIN_CACHE_DIR` ke jalur target seed Anda selama build sehingga plugin dipasang langsung ke sana:

```bash theme={null}
CLAUDE_CODE_PLUGIN_CACHE_DIR=/opt/claude-seed claude plugin marketplace add your-org/plugins
CLAUDE_CODE_PLUGIN_CACHE_DIR=/opt/claude-seed claude plugin install my-tool@your-plugins
```

Kemudian atur `CLAUDE_CODE_PLUGIN_SEED_DIR=/opt/claude-seed` di lingkungan runtime container Anda sehingga Claude Code membaca dari seed saat startup.

Saat startup, Claude Code mendaftarkan marketplace yang ditemukan di `known_marketplaces.json` seed ke dalam konfigurasi utama, dan menggunakan cache plugin yang ditemukan di bawah `cache/` di tempat tanpa mengklon ulang. Ini berfungsi dalam mode interaktif dan mode non-interaktif dengan flag `-p`.

Detail perilaku:

* **Read-only**: direktori seed tidak pernah ditulis. Pembaruan otomatis dinonaktifkan untuk marketplace seed karena git pull akan gagal di filesystem read-only.
* **Entri seed mengambil prioritas**: marketplace yang dideklarasikan dalam seed menimpa entri yang cocok apa pun dalam konfigurasi pengguna di setiap startup. Untuk opt out dari plugin seed, gunakan `/plugin disable` daripada menghapus marketplace.
* **Resolusi jalur**: Claude Code menemukan konten marketplace dengan menyelidiki `$CLAUDE_CODE_PLUGIN_SEED_DIR/marketplaces/<name>/` saat runtime, bukan dengan mempercayai jalur yang disimpan di dalam JSON seed. Ini berarti seed berfungsi dengan benar bahkan ketika dipasang di jalur berbeda dari tempat dibangun.
* **Mutasi diblokir**: menjalankan `/plugin marketplace remove` atau `/plugin marketplace update` terhadap marketplace yang dikelola seed gagal dengan panduan untuk meminta administrator Anda memperbarui image seed.
* **Komposisi dengan pengaturan**: jika `extraKnownMarketplaces` atau `enabledPlugins` mendeklarasikan marketplace yang sudah ada di seed, Claude Code menggunakan salinan seed alih-alih mengklon.

<h3 id="managed-marketplace-restrictions">
  Pembatasan marketplace yang dikelola
</h3>

Untuk organisasi yang memerlukan kontrol ketat atas sumber plugin, administrator dapat membatasi marketplace plugin mana yang diizinkan pengguna untuk tambahkan menggunakan pengaturan [`strictKnownMarketplaces`](/docs/id/settings#strictknownmarketplaces) dalam pengaturan yang dikelola. Untuk juga menolak flag CLI yang sideload plugin, agen, dan server MCP untuk satu kali jalankan, pasangkan dengan [`disableSideloadFlags`](/docs/id/settings#available-settings). Untuk allowlist marketplace mana yang plugin-nya dapat muncul sebagai saran instalasi kontekstual, atur [`pluginSuggestionMarketplaces`](/docs/id/settings#available-settings).

Ketika `strictKnownMarketplaces` dikonfigurasi dalam pengaturan yang dikelola, perilaku pembatasan tergantung pada nilainya:

| Value                       | Perilaku                                                                                |
| --------------------------- | --------------------------------------------------------------------------------------- |
| Tidak terdefinisi (default) | Tidak ada pembatasan. Pengguna dapat menambahkan marketplace apa pun                    |
| Array kosong `[]`           | Lockdown lengkap. Pengguna tidak dapat menambahkan marketplace baru apa pun             |
| Daftar sumber               | Pengguna hanya dapat menambahkan marketplace yang cocok dengan daftar izin secara tepat |

<h4 id="common-configurations">
  Konfigurasi umum
</h4>

Nonaktifkan semua penambahan marketplace:

```json theme={null}
{
  "strictKnownMarketplaces": []
}
```

Izinkan marketplace tertentu saja:

```json theme={null}
{
  "strictKnownMarketplaces": [
    {
      "source": "github",
      "repo": "acme-corp/approved-plugins"
    },
    {
      "source": "github",
      "repo": "acme-corp/security-tools",
      "ref": "v2.0"
    },
    {
      "source": "url",
      "url": "https://plugins.example.com/marketplace.json"
    }
  ]
}
```

Izinkan semua marketplace dari server git internal menggunakan pencocokan pola regex pada host. Ini adalah pendekatan yang direkomendasikan untuk [GitHub Enterprise Server](/docs/id/github-enterprise-server#plugin-marketplaces-on-ghes) atau instance GitLab yang dihosting sendiri:

```json theme={null}
{
  "strictKnownMarketplaces": [
    {
      "source": "hostPattern",
      "hostPattern": "^github\\.example\\.com$"
    }
  ]
}
```

Izinkan marketplace berbasis filesystem dari direktori tertentu menggunakan pencocokan pola regex pada jalur:

```json theme={null}
{
  "strictKnownMarketplaces": [
    {
      "source": "pathPattern",
      "pathPattern": "^/opt/approved/"
    }
  ]
}
```

Gunakan `".*"` sebagai `pathPattern` untuk mengizinkan jalur filesystem apa pun sambil tetap mengontrol sumber jaringan dengan `hostPattern`.

<Note>
  `strictKnownMarketplaces` membatasi apa yang dapat ditambahkan pengguna, tetapi tidak mendaftarkan marketplace dengan sendirinya. Untuk membuat marketplace yang diizinkan tersedia secara otomatis tanpa pengguna menjalankan `/plugin marketplace add`, pasangkan dengan [`extraKnownMarketplaces`](/docs/id/settings#extraknownmarketplaces) dalam `managed-settings.json` yang sama. Lihat [Menggunakan keduanya bersama-sama](/docs/id/settings#strictknownmarketplaces).
</Note>

<h4 id="how-restrictions-work">
  Cara pembatasan bekerja
</h4>

Pembatasan diperiksa sebelum operasi jaringan atau filesystem apa pun. Pemeriksaan berjalan pada marketplace add dan pada plugin install, update, refresh, dan auto-update. Jika marketplace ditambahkan sebelum kebijakan dikonfigurasi dan sumbernya tidak lagi cocok dengan daftar izin, Claude Code menolak untuk memasang atau memperbarui plugin darinya. Penegakan yang sama berlaku untuk `blockedMarketplaces`.

Daftar izin menggunakan pencocokan tepat untuk sebagian besar jenis sumber. Agar marketplace diizinkan, semua field yang ditentukan harus cocok secara tepat:

* Untuk sumber GitHub: `repo` diperlukan, dan `ref` atau `path` juga harus cocok jika ditentukan dalam daftar izin
* Untuk sumber URL: URL lengkap harus cocok secara tepat
* Untuk sumber `hostPattern`: host marketplace dicocokkan dengan pola regex
* Untuk sumber `pathPattern`: jalur filesystem marketplace dicocokkan dengan pola regex

Pencocokan tepat tidak menormalkan URL: garis miring trailing, akhiran `.git`, atau bentuk `ssh://` versus `https://` diperlakukan sebagai nilai berbeda. Jika marketplace organisasi Anda dapat diklon oleh lebih dari satu bentuk URL, lebih suka entri `hostPattern` daripada URL literal sehingga semua bentuk cocok.

Karena `strictKnownMarketplaces` diatur dalam [pengaturan yang dikelola](/docs/id/settings#settings-files), konfigurasi pengguna individual dan proyek tidak dapat mengganti pembatasan ini.

Untuk detail konfigurasi lengkap termasuk semua jenis sumber yang didukung dan perbandingan dengan `extraKnownMarketplaces`, lihat [referensi strictKnownMarketplaces](/docs/id/settings#strictknownmarketplaces).

<h3 id="version-resolution-and-release-channels">
  Resolusi versi dan saluran rilis
</h3>

Versi plugin menentukan jalur cache dan deteksi pembaruan: jika versi yang diselesaikan cocok dengan apa yang sudah dimiliki pengguna, `/plugin update` dan auto-update melewati plugin.

Claude Code menyelesaikan versi plugin dari yang pertama dari ini yang diatur:

1. `version` dalam `plugin.json` plugin
2. `version` dalam entri marketplace plugin
3. SHA commit git dari sumber plugin

Untuk jenis sumber berbasis git `github`, `url`, `git-subdir`, dan jalur relatif di dalam marketplace yang dihosting git, Anda dapat menghilangkan `version` sepenuhnya dan setiap commit baru diperlakukan sebagai versi baru. Ini adalah setup paling sederhana untuk plugin internal atau yang sedang dikembangkan secara aktif.

<Warning>
  Menetapkan `version` menyematkan plugin. Jika `plugin.json` mendeklarasikan `"version": "1.0.0"`, mendorong commit baru tanpa mengubah string itu tidak melakukan apa pun untuk pengguna yang ada, karena Claude Code melihat versi yang sama dan menyimpan salinan cache. Bump field pada setiap rilis, atau hilangkan untuk menggunakan SHA commit.

  Hindari menetapkan `version` di kedua `plugin.json` dan entri marketplace. Nilai `plugin.json` selalu menang secara diam-diam, jadi versi manifest yang basi dapat menyembunyikan versi yang Anda atur di `marketplace.json`.
</Warning>

<h4 id="set-up-release-channels">
  Siapkan saluran rilis
</h4>

Untuk mendukung saluran rilis "stable" dan "latest" untuk plugin Anda, Anda dapat menyiapkan dua marketplace yang menunjuk ke refs atau SHA berbeda dari repo yang sama. Anda kemudian dapat menetapkan dua marketplace ke grup pengguna berbeda melalui [pengaturan yang dikelola](/docs/id/settings#settings-files).

<Warning>
  Setiap saluran harus diselesaikan ke versi yang berbeda. Jika Anda menggunakan versi eksplisit, `plugin.json` harus mendeklarasikan `version` berbeda di setiap ref yang disematkan. Jika Anda menghilangkan `version`, SHA commit yang berbeda sudah membedakan saluran. Jika dua refs diselesaikan ke string versi yang sama, Claude Code memperlakukannya sebagai identik dan melewati pembaruan.
</Warning>

<h5 id="example">
  Contoh
</h5>

```json theme={null}
{
  "name": "stable-tools",
  "plugins": [
    {
      "name": "code-formatter",
      "source": {
        "source": "github",
        "repo": "acme-corp/code-formatter",
        "ref": "stable"
      }
    }
  ]
}
```

```json theme={null}
{
  "name": "latest-tools",
  "plugins": [
    {
      "name": "code-formatter",
      "source": {
        "source": "github",
        "repo": "acme-corp/code-formatter",
        "ref": "latest"
      }
    }
  ]
}
```

<h5 id="assign-channels-to-user-groups">
  Tetapkan saluran ke grup pengguna
</h5>

Tetapkan setiap marketplace ke grup pengguna yang sesuai melalui pengaturan yang dikelola. Misalnya, grup stabil menerima:

```json theme={null}
{
  "extraKnownMarketplaces": {
    "stable-tools": {
      "source": {
        "source": "github",
        "repo": "acme-corp/stable-tools"
      }
    }
  }
}
```

Grup early-access menerima `latest-tools` sebagai gantinya:

```json theme={null}
{
  "extraKnownMarketplaces": {
    "latest-tools": {
      "source": {
        "source": "github",
        "repo": "acme-corp/latest-tools"
      }
    }
  }
}
```

<h4 id="pin-dependency-versions">
  Sematkan versi dependensi
</h4>

Plugin dapat membatasi dependensinya ke rentang semver sehingga pembaruan dependensi tidak merusak plugin yang bergantung. Lihat [Batasi versi dependensi plugin](/docs/id/plugin-dependencies) untuk konvensi git-tag `{plugin-name}--v{version}`, sintaks rentang, dan bagaimana beberapa batasan pada dependensi yang sama digabungkan.

<h3 id="rename-or-remove-a-plugin">
  Ubah nama atau hapus plugin
</h3>

`name` plugin adalah pengidentifikasi stabilnya. Pengguna mereferensikannya dalam `enabledPlugins`, `pluginConfigs`, dan perintah `/plugin install`, jadi mengubahnya merusak setiap instalasi yang ada. Untuk mengubah label yang ditampilkan di UI tanpa merusak instalasi, atur [`displayName`](#optional-plugin-fields) dan jaga `name` tetap tidak berubah.

Jika Anda harus mengubah `name` plugin, atau Anda menghapus plugin dari array `plugins`, tambahkan entri `renames` tingkat atas sehingga pengguna yang ada bermigrasi alih-alih melihat kesalahan `plugin-not-found`. Migrasi otomatis memerlukan Claude Code v2.1.193 atau lebih baru. Petakan setiap nama lama ke nama barunya, atau ke `null` jika plugin tidak lagi ada. Contoh berikut mengubah nama `formatter` menjadi `code-formatter` dan mencatat bahwa `legacy-linter` dihapus:

```json theme={null}
{
  "name": "acme-tools",
  "owner": { "name": "Acme" },
  "plugins": [
    { "name": "code-formatter", "source": "./plugins/code-formatter" }
  ],
  "renames": {
    "formatter": "code-formatter",
    "legacy-linter": null
  }
}
```

Ketika pengguna memulai Claude Code dengan nama lama masih dalam pengaturan mereka, Claude Code mengikuti peta `renames`:

* Jika entri menunjuk ke nama baru, Claude Code memuat plugin dengan nama barunya dan menampilkan pemberitahuan satu baris seperti `Renamed to "code-formatter" in the "acme-tools" marketplace`. Kemudian menulis ulang kunci lama ke kunci baru dalam cakupan pengaturan pengguna, proyek, dan lokal untuk kedua `enabledPlugins` dan `pluginConfigs`, sehingga pemberitahuan muncul sekali.
* Untuk entri `null`, Claude Code menghapus kunci lama dan pemberitahuan melaporkan bahwa plugin dihapus dari marketplace.
* Jika plugin yang diubah nama menggunakan sumber jarak jauh seperti `github` atau `npm`, Claude Code melaporkan `plugin-cache-miss` setelah pengubahan nama dan pengguna harus menjalankan `/plugin install` sekali untuk mengambilnya dengan nama baru.

Perlakukan `renames` sebagai riwayat append-only: jaga entri lama tetap ada bahkan setelah Anda mengharapkan setiap pengguna untuk bermigrasi. Claude Code mengikuti rantai, jadi jika Anda kemudian mengubah nama `code-formatter` menjadi `formatter-pro`, tambahkan entri kedua daripada mengedit yang pertama. Pengguna yang masih memiliki `formatter` asli yang diaktifkan kemudian diselesaikan melalui kedua entri ke `formatter-pro`.

Jalankan `claude plugin validate .` setelah mengedit peta; itu menolak entri apa pun yang rantainya membentuk siklus atau tidak berakhir di `null` atau nama yang terdaftar dalam `plugins`.

<Note>
  Pengaturan yang dikelola dan kebijakan adalah read-only untuk Claude Code, jadi plugin yang diaktifkan di sana tidak dapat ditulis ulang secara otomatis. Plugin yang diubah nama masih dimuat setiap sesi, tetapi pemberitahuan pengubahan nama berulang sampai administrator memperbarui `enabledPlugins` dalam file pengaturan yang dikelola untuk menggunakan nama baru. Hal yang sama berlaku untuk plugin yang diaktifkan melalui sumber read-only lainnya seperti `--add-dir`.
</Note>

Versi Claude Code sebelumnya mengabaikan field `renames` dan melaporkan `plugin-not-found` untuk nama lama.

<h2 id="validation-and-testing">
  Validasi dan pengujian
</h2>

Uji marketplace Anda sebelum berbagi.

Validasi sintaks JSON marketplace Anda:

```bash theme={null}
claude plugin validate .
```

Atau dari dalam Claude Code:

```shell theme={null}
/plugin validate .
```

Tambahkan marketplace untuk pengujian:

```shell theme={null}
/plugin marketplace add ./path/to/marketplace
```

Pasang plugin uji untuk memverifikasi semuanya berfungsi:

```shell theme={null}
/plugin install test-plugin@marketplace-name
```

Untuk alur kerja pengujian plugin lengkap, lihat [Uji plugin Anda secara lokal](/docs/id/plugins#test-your-plugins-locally). Untuk troubleshooting teknis, lihat [Plugins reference](/docs/id/plugins-reference).

<h2 id="manage-marketplaces-from-the-cli">
  Kelola marketplace dari CLI
</h2>

Claude Code menyediakan subperintah `claude plugin marketplace` non-interaktif untuk scripting dan otomasi. Ini setara dengan perintah `/plugin marketplace` yang tersedia dalam sesi interaktif.

<h3 id="plugin-marketplace-add">
  Plugin marketplace add
</h3>

Tambahkan marketplace dari repositori GitHub, URL git, URL jarak jauh, atau jalur lokal.

```bash theme={null}
claude plugin marketplace add <source> [options]
```

**Argumen:**

* `<source>`: Shorthand GitHub `owner/repo`, URL git, URL jarak jauh ke file `marketplace.json`, atau jalur direktori lokal. Untuk menyematkan ke branch atau tag, tambahkan `@ref` ke shorthand GitHub atau `#ref` ke URL git

URL harus menyertakan skemanya. Mulai dari Claude Code v2.1.196, host yang diketik tanpa skema, seperti `gitlab.example.com/team/plugins`, ditolak sebagai shorthand `owner/repo` yang tidak valid dan kesalahan memberi tahu Anda untuk menambahkan `https://` atau menggunakan `./` untuk jalur lokal. Versi sebelumnya salah membacanya sebagai jalur repositori GitHub dan gagal saat clone dengan kesalahan GitHub not-found.

**Opsi:**

| Opsi                  | Deskripsi                                                                                                                                                  | Default |
| :-------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------- | :------ |
| `--scope <scope>`     | Di mana mendeklarasikan marketplace: `user`, `project`, atau `local`. Lihat [Plugin installation scopes](/docs/id/plugins-reference#plugin-installation-scopes) | `user`  |
| `--sparse <paths...>` | Batasi checkout ke direktori tertentu melalui git sparse-checkout. Berguna untuk monorepo                                                                  |         |

Tambahkan marketplace dari GitHub menggunakan shorthand `owner/repo`:

```bash theme={null}
claude plugin marketplace add acme-corp/claude-plugins
```

Sematkan ke branch atau tag tertentu dengan `@ref`:

```bash theme={null}
claude plugin marketplace add acme-corp/claude-plugins@v2.0
```

Tambahkan dari URL git di host non-GitHub:

```bash theme={null}
claude plugin marketplace add https://gitlab.example.com/team/plugins.git
```

Tambahkan dari URL jarak jauh yang melayani file `marketplace.json` secara langsung:

```bash theme={null}
claude plugin marketplace add https://example.com/marketplace.json
```

Tambahkan dari direktori lokal untuk pengujian:

```bash theme={null}
claude plugin marketplace add ./my-marketplace
```

Deklarasikan marketplace di scope proyek sehingga dibagikan dengan tim Anda melalui `.claude/settings.json`:

```bash theme={null}
claude plugin marketplace add acme-corp/claude-plugins --scope project
```

Untuk monorepo, batasi checkout ke direktori yang berisi konten plugin:

```bash theme={null}
claude plugin marketplace add acme-corp/monorepo --sparse .claude-plugin plugins
```

<h3 id="plugin-marketplace-list">
  Plugin marketplace list
</h3>

Daftar semua marketplace yang dikonfigurasi.

```bash theme={null}
claude plugin marketplace list [options]
```

**Opsi:**

| Opsi     | Deskripsi           |
| :------- | :------------------ |
| `--json` | Output sebagai JSON |

Dengan `--json`, setiap entri mencakup `name`, `source`, dan bidang khusus sumber: `repo` untuk sumber GitHub, `url` untuk sumber git dan URL, dan `path` untuk sumber lokal. Sumber GitHub dan git juga menyertakan bidang `ref` ketika marketplace ditambahkan dengan branch atau tag yang disematkan.

<h3 id="plugin-marketplace-remove">
  Plugin marketplace remove
</h3>

Hapus marketplace yang dikonfigurasi. Alias `rm` juga diterima.

```bash theme={null}
claude plugin marketplace remove <name> [options]
```

**Argumen:**

* `<name>`: nama marketplace untuk dihapus, seperti yang ditunjukkan oleh `claude plugin marketplace list`. Ini adalah `name` dari `marketplace.json`, bukan sumber yang Anda teruskan ke `add`

**Opsi:**

| Opsi              | Deskripsi                                                                                                                                                                                                                                                                                                                                                                                                                                 | Default       |
| :---------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------ |
| `--scope <scope>` | Batasi penghapusan ke scope pengaturan tunggal: `user`, `project`, atau `local`. Lihat [Plugin installation scopes](/docs/id/plugins-reference#plugin-installation-scopes). Ketika dihilangkan, deklarasi dihapus dari setiap scope yang dapat diedit. Ketika diberikan, hanya deklarasi scope tersebut yang dihapus; status bersama, cache, dan data plugin yang dipasang dipertahankan ketika marketplace masih dideklarasikan di scope lain | (semua scope) |

<Warning>
  Menghapus marketplace dari scope terakhirnya yang tersisa juga mencopot plugin apa pun yang Anda pasang darinya. Untuk menyegarkan marketplace tanpa kehilangan plugin yang dipasang, gunakan `claude plugin marketplace update` sebagai gantinya.
</Warning>

<h3 id="plugin-marketplace-update">
  Plugin marketplace update
</h3>

Segarkan marketplace dari sumbernya untuk mengambil plugin baru dan perubahan versi. Marketplace yang ditambahkan dengan branch atau tag `ref` diperbarui ke commit terbaru dari ref tersebut, bukan branch default repositori.

```bash theme={null}
claude plugin marketplace update [name]
```

**Argumen:**

* `[name]`: nama marketplace untuk diperbarui, seperti yang ditunjukkan oleh `claude plugin marketplace list`. Memperbarui semua marketplace jika dihilangkan

Baik `remove` maupun `update` gagal ketika dijalankan terhadap marketplace yang dikelola seed, yang bersifat read-only. Saat memperbarui semua marketplace, entri yang dikelola seed dilewati dan marketplace lainnya masih diperbarui. Untuk mengubah plugin yang disediakan seed, minta administrator Anda memperbarui image seed. Lihat [Pra-isi plugin untuk container](#pre-populate-plugins-for-containers).

<h2 id="troubleshooting">
  Troubleshooting
</h2>

<h3 id="marketplace-not-loading">
  Marketplace tidak memuat
</h3>

**Gejala**: Tidak dapat menambahkan marketplace atau melihat plugin darinya

**Solusi**:

* Verifikasi URL marketplace dapat diakses
* Periksa bahwa `.claude-plugin/marketplace.json` ada di jalur yang ditentukan
* Pastikan sintaks JSON valid menggunakan `claude plugin validate` atau `/plugin validate`. Untuk memeriksa frontmatter skill, agent, dan command, jalankan perintah terhadap setiap direktori plugin
* Untuk repositori pribadi, konfirmasi Anda memiliki izin akses

<h3 id="marketplace-validation-errors">
  Kesalahan validasi marketplace
</h3>

Jalankan `claude plugin validate .` atau `/plugin validate .` dari direktori marketplace Anda untuk memeriksa masalah. Ketika ditunjukkan ke direktori marketplace, validator memeriksa `marketplace.json` untuk kesalahan skema, nama plugin duplikat, dan traversal jalur sumber. Untuk setiap entri yang `source`-nya adalah jalur lokal, validator juga memvalidasi `plugin.json` plugin tersebut dan memberikan peringatan ketika `version` entri tidak cocok dengan yang ada di `plugin.json`. Masalah yang ditemukan dalam `plugin.json` plugin diawali dengan indeks entri, dalam bentuk `plugins[2] plugin.json →`.

Mulai dari Claude Code v2.1.196, pass per-entri juga:

* mencakup plugin yang `source`-nya adalah `.`
* berjalan ketika `marketplace.json` berada di luar direktori `.claude-plugin`, menyelesaikan sumber terhadap direktori file itu sendiri
* melaporkan masalah setiap entri bahkan ketika bagian lain dari file memiliki kesalahan skema

Versi sebelumnya melewati plugin di root marketplace dan hanya turun dari `.claude-plugin/marketplace.json`.

Untuk memvalidasi `plugin.json` plugin individual dan file skill, agent, command, dan hook-nya, jalankan perintah terhadap direktori plugin itu sendiri, misalnya `claude plugin validate ./plugins/my-plugin`. Kesalahan umum:

| Kesalahan                                         | Penyebab                                               | Solusi                                                                                                                                                             |
| :------------------------------------------------ | :----------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `File not found: .claude-plugin/marketplace.json` | Manifest hilang                                        | Buat `.claude-plugin/marketplace.json` dengan field yang diperlukan                                                                                                |
| `Invalid JSON syntax: Unexpected token...`        | Kesalahan sintaks JSON dalam marketplace.json          | Periksa koma yang hilang, koma ekstra, atau string yang tidak dikutip                                                                                              |
| `Duplicate plugin name "x" found in marketplace`  | Dua plugin berbagi nama yang sama                      | Berikan setiap plugin nilai `name` yang unik                                                                                                                       |
| `plugins[0].source: Path contains ".."`           | Jalur sumber berisi `..`                               | Gunakan jalur relatif terhadap root marketplace tanpa `..`. Lihat [Jalur relatif](#relative-paths)                                                                 |
| `YAML frontmatter failed to parse: ...`           | YAML tidak valid dalam file skill, agent, atau command | Perbaiki sintaks YAML dalam blok frontmatter. Saat runtime file ini dimuat tanpa metadata. Dilaporkan hanya saat memvalidasi direktori plugin                      |
| `Invalid JSON syntax: ...` (hooks.json)           | `hooks/hooks.json` yang tidak terbentuk dengan baik    | Perbaiki sintaks JSON. `hooks/hooks.json` yang tidak terbentuk dengan baik mencegah seluruh plugin dari dimuat. Dilaporkan hanya saat memvalidasi direktori plugin |

**Peringatan** (non-blocking):

* `Marketplace has no plugins defined`: tambahkan setidaknya satu plugin ke array `plugins`
* `No marketplace description provided`: tambahkan `description` tingkat atas untuk membantu pengguna memahami marketplace Anda
* `Plugin name "x" is not kebab-case`: nama plugin berisi huruf besar, spasi, atau karakter khusus. Ubah nama menjadi huruf kecil, digit, dan tanda hubung saja (misalnya, `my-plugin`). Claude Code menerima bentuk lain, tetapi sinkronisasi marketplace claude.ai menolaknya.

<h3 id="plugin-installation-failures">
  Kegagalan instalasi plugin
</h3>

**Gejala**: Marketplace muncul tetapi instalasi plugin gagal

**Solusi**:

* Verifikasi URL sumber plugin dapat diakses
* Periksa bahwa direktori plugin berisi file yang diperlukan
* Untuk sumber GitHub, pastikan repositori publik atau Anda memiliki akses
* Uji sumber plugin secara manual dengan mengklon/mengunduh
* Jika sumber menentukan baik `ref` maupun `sha`, cabang atau tag upstream yang dihapus tidak memblokir instalasi pada sebagian besar host git, termasuk GitHub, GitLab, dan Bitbucket. Pada server yang tidak mendukung pengambilan commit berdasarkan SHA, seperti AWS CodeCommit, `ref` masih harus ada dan commit yang ditentukan harus dapat dijangkau darinya. Jika instalasi masih gagal, konfirmasi commit yang ditentukan masih ada di repositori

<h3 id="private-repository-authentication-fails">
  Autentikasi repositori pribadi gagal
</h3>

**Gejala**: Kesalahan autentikasi saat memasang plugin dari repositori pribadi

**Solusi**:

Untuk instalasi manual dan pembaruan:

* Verifikasi Anda diautentikasi dengan penyedia git Anda (misalnya, jalankan `gh auth status` untuk GitHub)
* Periksa bahwa helper kredensial Anda dikonfigurasi dengan benar: `git config --global credential.helper`
* Coba klon repositori secara manual untuk memverifikasi kredensial Anda berfungsi

Untuk pembaruan otomatis latar belakang:

* Secara default, penyegaran latar belakang menonaktifkan helper kredensial git untuk pull, sehingga pull tidak dapat diautentikasi melalui HTTPS. Remote SSH dengan kunci yang dimuat di `ssh-agent` masih diautentikasi. Pull yang gagal memicu re-clone dari awal, yang menggunakan kredensial tersimpan Anda tetapi mungkin time out pada repositori besar
* Atur `CLAUDE_CODE_PLUGIN_KEEP_MARKETPLACE_ON_FAILURE=1` untuk menyimpan klon yang ada ketika pull latar belakang gagal
* Konfigurasikan helper kredensial git, misalnya `gh auth setup-git`, sehingga fallback re-clone dapat diautentikasi
* Jika re-clone time out pada repositori besar, tingkatkan batas dengan [`CLAUDE_CODE_PLUGIN_GIT_TIMEOUT_MS`](#git-operations-time-out)
* Konfigurasikan [rewrite URL git](#private-repositories) yang dibatasi pada repositori marketplace sehingga pull latar belakang diautentikasi secara langsung
* Atau perbarui marketplace pribadi secara manual dengan `/plugin marketplace update <name>`, yang menggunakan kredensial Anda

<h3 id="marketplace-updates-fail-in-offline-environments">
  Pembaruan marketplace gagal di lingkungan offline
</h3>

**Gejala**: Marketplace `git pull` gagal di latar belakang dan Claude Code berulang kali mencoba re-clone yang tidak dapat berhasil.

**Penyebab**: Secara default, ketika `git pull` gagal, Claude Code mencoba re-clone dari awal. Di lingkungan offline atau airgapped, re-cloning gagal dengan cara yang sama, dan pemulihan cache sebelumnya sesudahnya adalah best-effort. Refresh berjalan di latar belakang setelah startup, sehingga tidak menunda startup, tetapi setiap sesi mengulangi upaya yang gagal dan setiap operasi git dapat menunggu [timeout 120 detik](#git-operations-time-out).

**Solusi**: Atur `CLAUDE_CODE_PLUGIN_KEEP_MARKETPLACE_ON_FAILURE=1` untuk melewati upaya re-clone dan terus menggunakan cache yang ada ketika pull gagal:

```bash theme={null}
export CLAUDE_CODE_PLUGIN_KEEP_MARKETPLACE_ON_FAILURE=1
```

Dengan variabel ini diatur, Claude Code mempertahankan klon marketplace yang sudah usang pada kegagalan `git pull` dan terus menggunakan status terakhir yang diketahui baik. Untuk deployment yang sepenuhnya offline di mana repositori tidak akan pernah dapat dijangkau, gunakan [`CLAUDE_CODE_PLUGIN_SEED_DIR`](#pre-populate-plugins-for-containers) untuk pra-isi direktori plugin saat waktu build sebagai gantinya.

<h3 id="git-operations-time-out">
  Operasi Git time out
</h3>

**Gejala**: Instalasi plugin atau pembaruan marketplace gagal dengan kesalahan timeout seperti "Git clone timed out after 120s" atau "Git pull timed out after 120s".

**Penyebab**: Claude Code menggunakan timeout 120 detik untuk semua operasi git, termasuk mengklon repositori plugin dan menarik pembaruan marketplace. Repositori besar atau koneksi jaringan lambat mungkin melebihi batas ini.

**Solusi**: Tingkatkan timeout menggunakan variabel lingkungan `CLAUDE_CODE_PLUGIN_GIT_TIMEOUT_MS`. Nilainya dalam milidetik:

```bash theme={null}
export CLAUDE_CODE_PLUGIN_GIT_TIMEOUT_MS=300000  # 5 minutes
```

<h3 id="plugins-with-relative-paths-fail-in-url-based-marketplaces">
  Plugin dengan jalur relatif gagal di marketplace berbasis URL
</h3>

**Gejala**: Menambahkan marketplace melalui URL (seperti `https://example.com/marketplace.json`), tetapi plugin dengan sumber jalur relatif seperti `"./plugins/my-plugin"` gagal dipasang dengan kesalahan "path not found".

**Penyebab**: Marketplace berbasis URL hanya mengunduh file `marketplace.json` itu sendiri. Mereka tidak mengunduh file plugin dari server. Jalur relatif dalam entri marketplace mereferensikan file di server jarak jauh yang tidak diunduh.

**Solusi**:

* **Gunakan sumber eksternal**: Ubah entri plugin untuk menggunakan sumber GitHub, npm, atau URL git alih-alih jalur relatif:
  ```json theme={null}
  { "name": "my-plugin", "source": { "source": "github", "repo": "owner/repo" } }
  ```
* **Gunakan marketplace berbasis Git**: Host marketplace Anda di repositori Git dan tambahkan dengan URL git. Marketplace berbasis Git mengklon seluruh repositori, membuat jalur relatif berfungsi dengan benar.

<h3 id="files-not-found-after-installation">
  File tidak ditemukan setelah instalasi
</h3>

**Gejala**: Plugin dipasang tetapi referensi ke file gagal, terutama file di luar direktori plugin

**Penyebab**: Plugin disalin ke direktori cache daripada digunakan di tempat. Jalur yang mereferensikan file di luar direktori plugin (seperti `../shared-utils`) tidak akan berfungsi karena file tersebut tidak disalin.

**Solusi**: Lihat [Plugin caching and file resolution](/docs/id/plugins-reference#plugin-caching-and-file-resolution) untuk solusi termasuk symlink dan restruktur direktori.

Untuk alat debugging tambahan dan masalah umum, lihat [Debugging and development tools](/docs/id/plugins-reference#debugging-and-development-tools).

<h2 id="see-also">
  Lihat juga
</h2>

* [Temukan dan pasang plugin yang sudah dibuat](/docs/id/discover-plugins) - Memasang plugin dari marketplace yang ada
* [Plugins](/docs/id/plugins) - Membuat plugin Anda sendiri
* [Plugins reference](/docs/id/plugins-reference) - Spesifikasi teknis lengkap dan skema
* [Plugin settings](/docs/id/settings#plugin-settings) - Opsi konfigurasi plugin
* [strictKnownMarketplaces reference](/docs/id/settings#strictknownmarketplaces) - Pembatasan marketplace yang dikelola
