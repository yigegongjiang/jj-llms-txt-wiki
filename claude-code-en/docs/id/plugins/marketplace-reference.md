> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Referensi Marketplace

> Referensi lengkap untuk field marketplace.json, entri plugin, dan objek sumber plugin dan marketplace, dengan tempat masing-masing valid.

`marketplace.json` adalah file yang mendefinisikan marketplace plugin. File ini berisi nama marketplace, pemiliknya, dan satu entri per plugin. Sumber plugin setiap entri mengatakan di mana Claude Code mengambil plugin tersebut.

Sumber marketplace adalah objek terpisah yang mengatakan di mana Claude Code mengambil file marketplace itu sendiri. Anda menulis satu di pengaturan, atau Claude Code membangunnya ketika Anda menjalankan `claude plugin marketplace add`.

Referensi ini untuk pengelola marketplace yang membutuhkan nama atau nilai field yang tepat, dan untuk administrator yang perlu mengetahui nilai `source` mana yang valid dalam [`extraKnownMarketplaces`](/docs/id/settings-reference#extraknownmarketplaces), [`strictKnownMarketplaces`](/docs/id/settings-reference#strictknownmarketplaces), dan [`blockedMarketplaces`](/docs/id/plugins/org#restrict-what-users-can-install).

<Note>
  Kasus-kasus ini tercakup di halaman lain:

  * **Membangun atau menghosting marketplace**: lihat [Buat marketplace](/docs/id/plugins/create-marketplace) dan [Host dan kelola marketplace](/docs/id/plugins/host-marketplace)
  * **Resep allowlist dan blocklist**: lihat [Kelola plugin untuk organisasi Anda](/docs/id/plugins/org)
</Note>

Temukan bagian untuk apa yang Anda tulis atau baca:

* **File marketplace**: [Field tingkat atas](#top-level-fields) dan [Entri plugin](#plugin-entries)
* **`source` entri**: [Sumber plugin](#plugin-sources)
* **Objek `source` di pengaturan**: [Sumber marketplace](#marketplace-sources)
* **Output dari [`claude plugin validate <path>`](/docs/id/plugins/cli-reference)**: [Pesan validasi](#validation-messages), yang memetakan setiap pesan ke field yang disebutnya

<h2 id="marketplace-file">
  File marketplace
</h2>

Simpan file marketplace di `.claude-plugin/marketplace.json` di direktori marketplace Anda. Jika Anda menyimpan file di tempat lain di repositori, pengguna harus mendeklarasikan marketplace di [`extraKnownMarketplaces`](/docs/id/settings-reference#extraknownmarketplaces) dengan `path` diatur pada sumbernya, karena `claude plugin marketplace add` tidak memiliki opsi untuk itu.

Direktori yang berisi `.claude-plugin/` disebut akar marketplace, dan setiap sumber plugin relatif diselesaikan darinya, bukan dari `.claude-plugin/`.

Setiap pengguna mendaftarkan satu marketplace per `name`, jadi pengguna tidak dapat memiliki dua marketplace dengan nama yang sama terdaftar sekaligus.

Claude Code mengabaikan kunci tingkat atas yang tidak dikenal atau kunci entri plugin daripada menolaknya, jadi typo dimuat diam-diam. `claude plugin validate` melaporkan setiap kunci yang tidak dikenal sebagai peringatan.

<h3 id="reserved-names">
  Nama yang dicadangkan
</h3>

Anda tidak dapat memberikan marketplace Anda salah satu nama berikut:

* **Nama marketplace resmi**: `claude-code-marketplace`, `claude-code-plugins`, `claude-plugins-official`, `anthropic-marketplace`, `anthropic-plugins`, `agent-skills`, `anthropic-agent-skills`, `life-sciences`, `knowledge-work-plugins`, `claude-for-legal`, `claude-for-financial-services`, `financial-services-plugins`, `first-party-plugins`, dan `claude-tag-plugins`. Dicadangkan kecuali marketplace berasal dari `github` atau `git` [sumber marketplace](#marketplace-sources) di bawah `github.com/anthropics/`.
* **Nama marketplace komunitas**: `claude-community`, `claude-plugins-community`, dan `healthcare`. Dicadangkan di bawah aturan yang sama dengan nama resmi.
* **Nama direktori plugin**: `anthropic-plugin-directory` dan `claude-plugin-directory`. Dicadangkan di bawah aturan yang sama dengan nama resmi.
* **Nama yang menyamar sebagai marketplace resmi**: nama seperti `official-claude-plugins` atau `claude-plugins-v2`, dan nama apa pun yang berisi karakter non-ASCII. Kesalahannya adalah `Marketplace name impersonates an official Anthropic/Claude marketplace`. Karakter kontrol atau pemformatan bidirectional dalam nama juga melaporkan `Marketplace name cannot contain control or bidirectional-formatting characters`.
* <span id="reserved-name-spellings" />**Ejaan lain dari nama yang dicadangkan**: nama yang berbeda dari nama yang dicadangkan hanya dengan titik di akhir, atau dengan simbol selain garis bawah sebagai pengganti tanda hubung, jadi `claude.code.plugins` dihitung sebagai `claude-code-plugins`. `claude plugin validate` menerima nama seperti itu; menambahkan marketplace gagal dengan [`is another spelling of "<reserved>", a reserved marketplace name`](/docs/id/errors#marketplace-name-is-another-spelling-of-a-reserved-name), dan marketplace yang sudah terdaftar di bawah satu berhenti dimuat. Pemeriksaan ini memerlukan Claude Code v2.1.280 atau lebih baru.
* **Nama yang Claude Code gunakan untuk plugin yang tidak berasal dari marketplace**: `inline` untuk plugin yang dimuat dengan [`--plugin-dir`](/docs/id/cli-reference), `builtin` untuk plugin bawaan, `skills-dir` untuk plugin yang dimuat otomatis dari [`.claude/skills/`](/docs/id/skills), dan `synced` untuk plugin yang disinkronkan dari akun claude.ai Anda. `claude-plugin-test` juga dicadangkan. `skills-dir` juga muncul sebagai `{"source": "skills-dir"}` dalam `strictKnownMarketplaces` dan `blockedMarketplaces`, dijelaskan di bawah [Nilai sumber yang valid hanya dalam daftar kebijakan](#source-values-valid-only-in-policy-lists).
* **`npm`, `pip`, `uv`, `cargo`, `github`, dan `gh`**: dicadangkan dalam huruf apa pun. Pemeriksaan ini memerlukan Claude Code v2.1.275 atau lebih baru.
* **Nama yang dimulai dengan `claudeai-`**: dicadangkan untuk marketplace yang dihosting di claude.ai. `claude plugin marketplace add` menolak marketplace lain apa pun yang menggunakannya dengan `Cannot add marketplace "<name>": names starting with "claudeai-" are reserved for marketplaces hosted on claude.ai`.

<h2 id="top-level-fields">
  Field tingkat atas
</h2>

Tabel mencantumkan setiap kunci yang Claude Code baca dari `marketplace.json`. `name`, `owner`, dan `plugins` diperlukan.

| Field                                      | Tipe             | Deskripsi                                                                                                                                                                                                                                                                    |
| :----------------------------------------- | :--------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`                                     | string           | Pengenal marketplace. Tanpa spasi, karakter kontrol, atau karakter pemformatan bidirectional, tanpa `/` atau `\`, tanpa `..`, dan bukan `.`. Lihat [Nama yang dicadangkan](#reserved-names). Pengguna mengetiknya setelah `@` ketika mereka memasang plugin                  |
| `owner`                                    | object           | Informasi pengelola. `name` diperlukan; `email` dan `url` opsional                                                                                                                                                                                                           |
| `plugins`                                  | array            | [Entri plugin](#plugin-entries). Setiap entri divalidasi sendiri, jadi satu entri yang tidak valid tidak gagal marketplace                                                                                                                                                   |
| `$schema`                                  | string           | URL JSON Schema untuk pelengkapan otomatis editor. Diabaikan saat waktu muat                                                                                                                                                                                                 |
| `description`                              | string           | Deskripsi marketplace yang ditampilkan kepada pengguna. `claude plugin validate` memperingatkan ketika hilang                                                                                                                                                                |
| `version`                                  | string           | Versi manifest marketplace                                                                                                                                                                                                                                                   |
| `metadata.description`, `metadata.version` | string           | Lokasi alternatif untuk `description` dan `version`                                                                                                                                                                                                                          |
| `metadata.pluginRoot`                      | string           | Direktori yang nama sumber plugin bare diselesaikan di bawahnya. Lihat [Sumber plugin jalur relatif](#relative-path-plugin-source). Memerlukan Claude Code v2.1.239 atau lebih baru                                                                                          |
| `forceRemoveDeletedPlugins`                | boolean          | Ketika `true`, plugin yang Anda hapus dari `plugins` dihapus di mesin pengguna. Lihat [Host dan kelola marketplace](/docs/id/plugins/host-marketplace)                                                                                                                            |
| `allowCrossMarketplaceDependenciesOn`      | array of strings | Nama marketplace yang plugin-nya dapat dipasang sebagai dependensi plugin marketplace ini. Ketika Anda memasang plugin, hanya daftar di marketplace plugin itu sendiri yang berlaku, untuk seluruh rantai dependensinya. Lihat [Dependensi plugin](/docs/id/plugins/dependencies) |
| `renames`                                  | object           | Peta dari `name` plugin sebelumnya ke nama saat ini, atau ke `null` untuk plugin yang Anda hapus. Memerlukan Claude Code v2.1.193 atau lebih baru. Lihat [Host dan kelola marketplace](/docs/id/plugins/host-marketplace)                                                         |

<h2 id="plugin-entries">
  Entri plugin
</h2>

Setiap objek dalam array `plugins` tingkat atas dari `marketplace.json` menamai plugin dan mengatakan di mana mengambilnya. `name` dan `source` diperlukan.

Entri juga menerima setiap [field `plugin.json`](/docs/id/plugins/manifest-reference), seperti `description`, `version`, `author`, `commands`, dan `hooks`. Untuk kapan field tersebut berlaku, lihat [Bagaimana entri menggabungkan dengan plugin.json](#entry-and-plugin-json).

Tabel mencantumkan field entri sendiri dan field manifest yang maknanya berubah dalam entri.

| Field            | Tipe             | Deskripsi                                                                                                                                                                                                                                                                                                                        |
| :--------------- | :--------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`           | string           | Pengenal plugin, tanpa spasi, karakter kontrol, atau karakter pemformatan bidirectional. Pengguna mengetiknya sebelum `@` ketika mereka memasang, bahkan ketika `plugin.json` plugin sendiri menetapkan `name` yang berbeda                                                                                                      |
| `source`         | string or object | Di mana mengambil plugin. Lihat [Sumber plugin](#plugin-sources)                                                                                                                                                                                                                                                                 |
| `description`    | string           | Ditampilkan dalam daftar [`/plugin`](/docs/id/plugins/install) dan detail                                                                                                                                                                                                                                                             |
| `version`        | string           | String versi untuk plugin. Ketika `plugin.json` juga menetapkan `version`, `plugin.json` mengambil prioritas dan `claude plugin validate` memperingatkan. Lihat [Referensi pemuatan plugin](/docs/id/plugins/loading)                                                                                                                 |
| `category`       | string           | Kategori bentuk bebas untuk mengorganisir katalog                                                                                                                                                                                                                                                                                |
| `tags`           | array of strings | Tag bentuk bebas untuk pencarian                                                                                                                                                                                                                                                                                                 |
| `strict`         | boolean          | Default `true`. Apakah `plugin.json` adalah sumber definitif untuk komponen plugin. Lihat [Mode ketat](#strict-mode)                                                                                                                                                                                                             |
| `relevance`      | object           | Sinyal yang memberi tahu Claude Code kapan menyarankan plugin. Lihat [Rekomendasikan plugin untuk organisasi Anda](/docs/id/plugins/relevance)                                                                                                                                                                                        |
| `dependencies`   | array            | Plugin yang harus diaktifkan agar yang ini berfungsi. Setiap item adalah `"name"`, `"name@marketplace"`, atau objek. Lihat [Dependensi plugin](/docs/id/plugins/dependencies)                                                                                                                                                         |
| `defaultEnabled` | boolean          | Default `true`. Apakah plugin dimulai diaktifkan ketika pengguna belum menetapkannya dalam [`enabledPlugins`](/docs/id/settings-reference#enabledplugins). Nilai entri mengambil prioritas atas `plugin.json`                                                                                                                         |
| `displayName`    | string           | Nama yang dapat dibaca manusia ditampilkan di UI. Ketika baik entri maupun `plugin.json` plugin tidak menetapkan satu, pengguna melihat `name` plugin                                                                                                                                                                            |
| `metadata`       | object           | Objek bentuk bebas untuk field Anda sendiri. Claude Code tidak membacanya. Memerlukan Claude Code v2.1.222 atau lebih baru                                                                                                                                                                                                       |
| `headers`        | object           | Header HTTP yang Claude Code kirim ketika mengunduh [arsip](#archive-plugin-source) entri ini. Header yang ditetapkan di sini menggantikan header dengan nama yang sama dari [`headers`](#fields-by-type) sumber marketplace. Memerlukan Claude Code v2.1.238 atau lebih baru                                                    |
| `headersHelper`  | string           | Perintah yang mencetak header unduhan arsip entri ini sebagai satu objek JSON, untuk kredensial yang kedaluwarsa. Entri juga harus menetapkan [`"strict": false`](#strict-mode). Memerlukan Claude Code v2.1.238 atau lebih baru. Lihat [Autentikasi unduhan arsip](/docs/id/plugins/host-marketplace#authenticate-archive-downloads) |

<h3 id="entry-and-plugin-json">
  Bagaimana entri menggabungkan dengan plugin.json
</h3>

Field entri berlaku berbeda untuk plugin yang diambil yang memiliki `.claude-plugin/plugin.json` sendiri dan untuk yang tidak:

* **Tidak ada `plugin.json`**: entri adalah manifest terlepas dari `strict`. Setiap field manifest dalam entri berlaku, termasuk [`mcpServers`, `lspServers`, `userConfig`, dan `channels`](/docs/id/plugins/manifest-reference).
* **`plugin.json` hadir**: `plugin.json` adalah manifest. [Mode ketat](#strict-mode) memutuskan apakah enam field komponen entri, `commands`, `agents`, `skills`, `hooks`, `outputStyles`, dan `themes`, digabungkan dengannya atau ditolak sebagai konflik. Entri `mcpServers`, `lspServers`, `userConfig`, dan `channels` tidak berlaku. Deklarasikan mereka di `plugin.json`.

<h4 id="hooks-in-an-entry">
  Hooks dalam entri
</h4>

Tulis entri `hooks` sebagai objek inline yang memetakan nama acara hook ke array matcher. Jika Anda menulis jalur file atau array sebagai gantinya, `claude plugin validate` melewatinya. Hook tersebut tidak pernah berjalan, dan Claude Code melaporkan kesalahan `not yet supported in a marketplace entry` untuk plugin. Letakkan hook berbasis file di [`hooks/hooks.json`](/docs/id/plugins/components) plugin sendiri atau `plugin.json`.

<h4 id="display-fields">
  Field tampilan
</h4>

Baik entri maupun `plugin.json` plugin sendiri dapat menetapkan field tampilan `displayName`, `description`, `author`, `homepage`, `repository`, `license`, dan `keywords`. Pengguna melihat nilai-nilai ini dalam daftar dan detail plugin, sebelum dan sesudah pemasangan:

* Untuk field yang Anda tetapkan pada entri, pengguna melihat nilai entri, bahkan ketika `plugin.json` menetapkan yang berbeda.
* Untuk field yang entri biarkan tidak diatur, pengguna melihat nilai `plugin.json`.

Sebelum pemasangan, Claude Code hanya dapat membaca `plugin.json` untuk entri dengan [sumber jalur relatif](#relative-path-plugin-source), yang file pluginnya berada di dalam marketplace itu sendiri. Untuk entri dengan tipe sumber apa pun, pengguna hanya melihat field entri sendiri sampai mereka memasang plugin.

<h3 id="strict-mode">
  Mode ketat
</h3>

`strict` memutuskan apa yang terjadi ketika plugin yang diambil memiliki `plugin.json` sendiri dan entri juga mendeklarasikan salah satu [field komponen](#entry-and-plugin-json): `commands`, `agents`, `skills`, `hooks`, `outputStyles`, atau `themes`. Dengan `strict: true`, default, Claude Code menambahkan field komponen entri ke `plugin.json`, kecuali `hooks`, yang matcher-nya menggantikan manifest per acara. Dengan `strict: false`, entri yang mendeklarasikan field komponen apa pun adalah konflik, dan plugin gagal dimuat. Tabel menunjukkan setiap kombinasi `strict`, `plugin.json`, dan field komponen entri.

| `strict`        | `plugin.json` | Field komponen entri | Hasil                                                                                                                                                                                                                                  |
| :-------------- | :------------ | :------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| any             | absent        | any                  | Entri adalah manifest                                                                                                                                                                                                                  |
| `true`, default | present       | any                  | `plugin.json` adalah otoritas. Claude Code menambahkan field komponen entri ke dalamnya, kecuali `hooks`, yang matcher-nya [menggantikan manifest per acara](/docs/id/plugins/manifest-reference#how-entry-fields-combine-with-plugin-json) |
| `false`         | present       | none                 | `plugin.json` adalah manifest, seperti dengan `true`                                                                                                                                                                                   |
| `false`         | present       | one or more          | Konflik. Plugin gagal dimuat dengan `Plugin <name> has conflicting manifests: both plugin.json and marketplace entry specify components`                                                                                               |

<h2 id="plugin-sources">
  Sumber plugin
</h2>

`source` entri plugin mengatakan di mana Claude Code mengambil satu plugin itu. Ini adalah string jalur relatif atau objek yang `source` key-nya sendiri menamai tipenya, jadi entri terlihat seperti `"source": { "source": "github", "repo": "your-org/formatter" }`.

Tabel mencantumkan setiap tipe sumber plugin dan field-nya.

| Tipe          | Field                            | Catatan                                                                                                                                                                                                                              |
| :------------ | :------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Jalur relatif | string itu sendiri               | Direktori di dalam marketplace, diselesaikan dari akar marketplace. Harus dimulai dengan `./`, kecuali Anda menulis [nama bare di bawah `metadata.pluginRoot`](#relative-path-plugin-source). `"."` sendiri berarti akar itu sendiri |
| `github`      | `repo`, `ref`, `sha`             | Repositori GitHub dalam bentuk `owner/repo`                                                                                                                                                                                          |
| `url`         | `url`, `ref`, `sha`              | Repositori git apa pun menurut URL                                                                                                                                                                                                   |
| `git-subdir`  | `url`, `path`, `ref`, `sha`      | Satu subdirektori repositori git, diambil dengan sparse partial clone                                                                                                                                                                |
| `npm`         | `package`, `version`, `registry` | Paket npm, diambil dengan klien npm Anda dan dibuka tanpa menjalankan skrip install                                                                                                                                                  |
| `archive`     | `url`, `sha256`                  | Arsip Zip melalui HTTPS. Memerlukan Claude Code v2.1.224 atau lebih baru                                                                                                                                                             |
| `command`     | `command`, `timeout`, `mode`     | Direktori yang dicetak oleh perintah yang Claude Code jalankan di mesin pengguna. Memerlukan Claude Code v2.1.229 atau lebih baru                                                                                                    |

Nama `url` dan `github` juga [tipe sumber marketplace](#marketplace-sources), di mana `url` berarti tautan langsung ke file `marketplace.json` daripada repositori git. `git` hanya ada sebagai sumber marketplace, dan `npm` ada sebagai keduanya. `git-subdir`, `archive`, dan `command` hanya ada sebagai sumber plugin.

Gunakan jalur relatif untuk plugin di subdirektori repositori marketplace itu sendiri. Gunakan `git-subdir` untuk subdirektori repositori lain.

Sumber `github`, `url`, dan `git-subdir` berbagi field `ref` dan `sha`:

* **`ref`**: cabang atau tag. Default ke cabang default repositori.
* **`sha`**: SHA commit 40-karakter lowercase penuh. Ketika Anda menetapkan baik `ref` maupun `sha`, Claude Code melakukan checkout `sha`. Di sebagian besar host git, termasuk GitHub, GitLab, dan Bitbucket, ini berarti pemasangan berhasil bahkan jika cabang atau tag yang dinamai oleh `ref` telah dihapus upstream, selama commit masih dapat dijangkau dari repositori. Beberapa server, seperti AWS CodeCommit, tidak mendukung pengambilan commit menurut SHA. Di server tersebut `ref` masih harus ada dan commit yang disematkan harus dapat dijangkau darinya.

Untuk cara setiap tipe diambil, di-cache, dan diversi, lihat [Referensi pemuatan plugin](/docs/id/plugins/loading).

<h3 id="relative-path-plugin-source">
  Sumber plugin jalur relatif
</h3>

Jalur diselesaikan dari akar marketplace. `./plugins/formatter` adalah `<root>/plugins/formatter` meskipun file marketplace berada di `<root>/.claude-plugin/`.

Jalur yang berisi `..` gagal validasi. Di macOS dan Linux, Claude Code menolak jalur entri yang berisi garis miring terbalik di mana pun setelah `./` awal, jadi tulis jalur dengan garis miring maju.

```json theme={null}
{ "name": "formatter", "source": "./plugins/formatter" }
```

Jalur relatif diselesaikan hanya ketika Claude Code memiliki file marketplace, jadi periksa [tipe sumber marketplace](#marketplace-sources):

* **`github`, `git`, `file`, dan `directory`**: Claude Code memiliki file marketplace.
* **`url`**: Claude Code hanya mengambil `marketplace.json`, jadi jalur relatif tidak dapat diselesaikan. Berikan setiap plugin sumber objek sebagai gantinya, seperti `github` atau `git-subdir`.
* **`settings`**: jalur relatif ditolak sepenuhnya.

<h4 id="bare-names-under-pluginroot">
  Nama bare di bawah pluginRoot
</h4>

Nama bare adalah nama direktori tunggal tanpa `/`, seperti `"formatter"`. Untuk menulis nama bare daripada jalur `./`, atur [`metadata.pluginRoot`](#top-level-fields) ke direktori yang mereka selesaikan di bawahnya. Dengan `"pluginRoot": "./plugins"`, `"source": "formatter"` diselesaikan ke `./plugins/formatter`. Memerlukan Claude Code v2.1.239 atau lebih baru.

`metadata.pluginRoot` memiliki batasan ini:

* Itu sendiri harus jalur relatif di dalam marketplace.
* Itu tidak berpengaruh pada sumber yang sudah dimulai dengan `./`.
* Sumber yang berisi `/`, seperti `team-a/formatter`, bukan nama bare dan masih memerlukan awalan `./`, bahkan ketika `metadata.pluginRoot` diatur.

<h3 id="github-plugin-source">
  Sumber plugin github
</h3>

`repo` mengambil `owner/repo`. `ref` dan `sha` opsional.

```json theme={null}
{
  "name": "formatter",
  "source": {
    "source": "github",
    "repo": "your-org/formatter",
    "ref": "v2.0.0",
    "sha": "a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0"
  }
}
```

<h3 id="url-plugin-source">
  Sumber plugin url
</h3>

`url` adalah URL git lengkap: `https://`, `http://`, `file://`, atau `git@`. Akhiran `.git` tidak diperlukan, jadi URL Azure DevOps dan AWS CodeCommit berfungsi seperti yang ditulis. Tipe ini tidak mengambil shorthand `owner/repo`.

```json theme={null}
{
  "name": "formatter",
  "source": {
    "source": "url",
    "url": "https://gitlab.example.com/your-group/formatter.git",
    "ref": "main"
  }
}
```

<h3 id="git-subdir-plugin-source">
  Sumber plugin git-subdir
</h3>

`url` menerima URL git lengkap atau shorthand GitHub `owner/repo`. `path` adalah subdirektori yang menyimpan plugin, dan Claude Code hanya mengunduh subdirektori itu.

```json theme={null}
{
  "name": "formatter",
  "source": {
    "source": "git-subdir",
    "url": "https://github.com/your-org/monorepo.git",
    "path": "tools/formatter"
  }
}
```

<h3 id="npm-plugin-source">
  Sumber plugin npm
</h3>

Sumber `npm` mengambil field ini:

* `package`: nama paket, atau nama scoped seperti `@your-org/formatter`
* `version`: versi atau rentang
* `registry`: URL registry untuk paket yang bukan di registry default

Claude Code mengambil paket dengan klien npm Anda. Skrip install paket, seperti `preinstall` atau `postinstall`, tidak pernah berjalan, dan dependensinya tidak dipasang selama pengambilan. Jika paket memiliki lockfile yang didukung di samping `package.json`-nya, Claude Code memasang [dependensi paket Node.js](/docs/id/plugins/loading#node-js-package-dependencies) tersebut dalam langkah terpisah, juga dengan skrip dinonaktifkan.

```json theme={null}
{
  "name": "formatter",
  "source": {
    "source": "npm",
    "package": "@your-org/formatter",
    "version": "^2.0.0",
    "registry": "https://npm.example.com"
  }
}
```

<h3 id="archive-plugin-source">
  Sumber plugin archive
</h3>

`url` harus menggunakan `https://` dan tidak dapat menunjuk ke host loopback, link-local, atau cloud-metadata.

Akar plugin mungkin berada di atas zip atau satu direktori ke bawah.

`sha256` adalah digest arsip sebagai 64 karakter hex, huruf besar atau kecil. Ketika Anda menetapkannya, Claude Code menolak unduhan yang tidak cocok.

```json theme={null}
{
  "name": "formatter",
  "source": {
    "source": "archive",
    "url": "https://artifacts.example.com/formatter-2.0.0.zip",
    "sha256": "6bfa50e3d2e00c052b46abe51fff89346ac803e45771f76dcf6df1ab74cca5e1"
  }
}
```

<h3 id="command-plugin-source">
  Sumber plugin command
</h3>

Gunakan sumber `command` ketika alat yang dipasang di mesin pengguna menghasilkan direktori plugin, seperti IDE yang merender pluginnya untuk toolchain yang dipilih pengguna. Claude Code menjalankan perintah ketika pengguna memasang atau memperbarui plugin, dan [lagi sekali per sesi](/docs/id/plugins/loading#when-a-command-source-re-runs), jadi pengguna mendapatkan output alat yang berubah tanpa memasang ulang.

Sumber `command` mengambil field ini:

* `command`: perintah shell yang mencetak jalur absolut direktori plugin sebagai satu baris dan keluar 0. Claude Code menunjukkan seluruh string kepada pengguna untuk ditinjau sebelum menjalankannya. Tulis sebagai ASCII yang dapat dicetak, paling banyak 500 karakter, tanpa run empat atau lebih spasi.
* `timeout`: bilangan bulat detik dari 1 hingga 600. Default ke 60.
* `mode`: `copy`, default, atau `link`. Lihat [Mode copy dan mode link](#copy-mode-and-link-mode).

```json theme={null}
{
  "name": "formatter",
  "source": {
    "source": "command",
    "command": "my-tool claude-plugin-path",
    "timeout": 120
  }
}
```

Untuk cara pengguna menerima perintah, lihat [Pasang dari shell Anda](/docs/id/plugins/install#install-from-your-shell). Untuk apa yang pengguna lihat setelah Anda mengubahnya, lihat [Ubah perintah sumber command](/docs/id/plugins/host-marketplace#change-the-command-of-a-command-source). Administrator mematikan sumber command dengan [`disableCommandPluginSources`](/docs/id/settings-reference#disablecommandpluginsources).

<h4 id="what-the-command-must-do">
  Apa yang harus dilakukan perintah
</h4>

Tulis perintah untuk memenuhi persyaratan ini:

* **Shell dan direktori kerja**: Claude Code menjalankan perintah melalui `sh`, atau melalui `cmd.exe` di Windows, dari direktori home pengguna. Berikan jalur absolut atau perintah di `PATH`.
* **Output**: cetak tepat satu baris di stdout, jalur absolut direktori plugin, dan keluar 0 dalam `timeout` detik.
* **Isi direktori**: direktori menyimpan plugin lengkap pada saat perintah keluar. Jalur dapat berbeda dari satu run ke run berikutnya.

<h4 id="output-that-fails-the-install-or-update">
  Output yang gagal install atau update
</h4>

Install atau update gagal ketika perintah keluar non-zero, berjalan lebih lama dari `timeout`, atau mencetak apa pun selain satu jalur absolut. Itu juga gagal ketika direktori yang dicetak adalah salah satu dari ini:

* **Tidak ada konten plugin**: direktori yang dicetak tidak memiliki konten plugin di tingkat atasnya, seperti direktori `.claude-plugin/` atau direktori `skills/`, `commands/`, `agents/`, atau `hooks/`.
* **Direktori sesi sendiri**: direktori yang dicetak adalah yang Claude Code dimulai, atau salah satu induknya.
* **Jalur jaringan**: di Windows, jalur yang dicetak adalah jalur UNC.
* **Terlalu besar untuk disalin**: dalam mode copy, direktori lebih besar dari 256 MiB atau memiliki lebih dari 20.000 entri.

<h4 id="copy-mode-and-link-mode">
  Mode copy dan mode link
</h4>

`mode` memutuskan apakah Claude Code menyalin direktori yang dicetak atau menggunakannya di tempat:

* **`copy`**: Claude Code menyalin direktori ke cache plugin dan menurunkan [versi plugin](/docs/id/plugins/loading#how-claude-code-computes-the-version) dari hash file yang disalin. Alat Anda dapat menghapus atau menulis ulang direktori setelah perintah keluar. Re-run yang menghasilkan file identik dihitung sebagai up to date.
* **`link`**: Claude Code mengisi entri cache plugin dengan tautan ke setiap entri tingkat atas direktori yang dicetak dan memuat file di tempat. Tidak ada yang disalin, konten file tidak di-hash, dan batas ukuran tidak berlaku. Gunakan untuk direktori terlalu besar untuk disalin, seperti ekspor SDK yang dirender.

Plugin mode link memiliki persyaratan ini:

* **Simpan direktori di tempat**: Claude Code memuat plugin melalui tautan di setiap startup, jadi direktori yang dicetak harus tetap di mana itu selama plugin tetap dipasang.
* **Cetak jalur berbeda untuk menandakan konten baru**: versi berasal dari jalur nyata direktori yang dicetak dan entri tingkat atasnya, bukan dari file di dalamnya.
* **Simpan symlink tingkat atas di dalam direktori**: install gagal jika entri tingkat atas adalah symlink yang menunjuk di luar direktori yang dicetak.
* **Sertakan `node_modules`**: Claude Code melewati [install dependensi paket Node.js](/docs/id/plugins/loading#node-js-package-dependencies) untuk plugin mode link, jadi cetak direktori yang sudah berisi paket yang dibutuhkan plugin.
* **Sesi dimulai di dalam direktori**: sesi yang dimulai di direktori yang dicetak atau di mana pun di bawahnya tidak memuat plugin.
* **Bukan di Windows**: Claude Code menolak untuk memasang plugin mode link di Windows. Deklarasikan `"mode": "copy"` di sana.

<h2 id="marketplace-sources">
  Sumber marketplace
</h2>

Sumber marketplace mengatakan di mana Claude Code mengambil `marketplace.json` dari. CLI membangun satu untuk Anda ketika Anda menambahkan marketplace, dan Anda menulis satu sendiri di pengaturan:

* **[`claude plugin marketplace add`](/docs/id/plugins/cli-reference)**: Claude Code membangun sumber dari string yang Anda lewatkan.
* **[`extraKnownMarketplaces`](/docs/id/settings-reference#extraknownmarketplaces)**: Anda menulis sumber sendiri sebagai objek `source`.
* **[`strictKnownMarketplaces`](/docs/id/settings-reference#strictknownmarketplaces) dan [`blockedMarketplaces`](/docs/id/plugins/org#restrict-what-users-can-install)**: administrator menulis sumber dalam dua daftar kebijakan ini. `strictKnownMarketplaces` adalah allowlist dan `blockedMarketplaces` adalah blocklist.

Nama tipe `url`, `git`, dan `github` berarti sesuatu yang berbeda dalam sumber marketplace daripada dalam [sumber plugin](#plugin-sources):

| Nama tipe | Sebagai sumber marketplace                                                                     | Sebagai sumber plugin                                                          |
| :-------- | :--------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------- |
| `url`     | Tautan langsung ke file `marketplace.json`, dengan field `url`, `headers`, dan `headersHelper` | Repositori git untuk diklon, dengan field `url`, `ref`, dan `sha`              |
| `git`     | Repositori git untuk diklon, dengan field `url`, `ref`, `path`, dan `sparsePaths`              | Tidak ada                                                                      |
| `github`  | Repositori GitHub, dengan field `repo`, `ref`, `path`, dan `sparsePaths`                       | Repositori GitHub, dengan field `repo`, `ref`, dan `sha`, dan tidak ada `path` |

Tabel mencantumkan setiap tipe sumber marketplace dengan field-nya, input `claude plugin marketplace add` yang menghasilkannya, dan apa yang dilakukannya di masing-masing dari tiga kunci pengaturan.

| Tipe          | Field                                | Input `marketplace add`                                                                                                                                            | `extraKnownMarketplaces`                                    | `strictKnownMarketplaces`                                                                                                                                                                                                            | `blockedMarketplaces`                                                    |
| :------------ | :----------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------- |
| `url`         | `url`, `headers`, `headersHelper`    | URL `http://` atau `https://` yang tidak cocok dengan bentuk git                                                                                                   | Dimuat                                                      | Memungkinkan URL yang sama                                                                                                                                                                                                           | Memblokir URL yang sama                                                  |
| `github`      | `repo`, `ref`, `path`, `sparsePaths` | `owner/repo`, `owner/repo@ref`, atau `owner/repo#ref`                                                                                                              | Dimuat                                                      | Memungkinkan `repo`, `ref`, dan `path` yang sama. `repo` mungkin `owner/*`                                                                                                                                                           | Memblokir yang sama, dan URL `git` ke repositori yang sama               |
| `git`         | `url`, `ref`, `path`, `sparsePaths`  | URL `user@host:path`, atau URL `https://` yang berakhir dengan `.git`, berisi `/_git/`, atau menamai repositori github.com atau gitlab.com. `#ref` menyematkan ref | Dimuat                                                      | Memungkinkan URL, `ref`, dan `path` yang sama                                                                                                                                                                                        | Memblokir yang sama, dan ejaan lain dari repositori github.com yang sama |
| `npm`         | `package`                            | Tidak diproduksi                                                                                                                                                   | Gagal dimuat: `NPM marketplace sources not yet implemented` | Diuraikan tetapi tidak cocok dengan apa pun, karena tidak ada yang mendaftarkan marketplace `npm`                                                                                                                                    | Diuraikan tetapi tidak cocok dengan apa pun                              |
| `file`        | `path`                               | Jalur ke file `.json`                                                                                                                                              | Dimuat                                                      | Memungkinkan jalur yang sama                                                                                                                                                                                                         | Memblokir jalur yang sama                                                |
| `directory`   | `path`                               | Jalur ke direktori                                                                                                                                                 | Dimuat                                                      | Memungkinkan jalur yang sama                                                                                                                                                                                                         | Memblokir jalur yang sama                                                |
| `settings`    | `name`, `plugins`, `owner`           | Tidak diproduksi                                                                                                                                                   | Dimuat                                                      | Memungkinkan entri dengan `name` dan `plugins` yang sama                                                                                                                                                                             | Memblokir `name` yang sama                                               |
| `skills-dir`  | none                                 | Tidak diproduksi                                                                                                                                                   | Gagal dimuat: `Unsupported marketplace source type`         | Menjaga [plugin direktori skills](/docs/id/plugins/org#keep-skills-directory-plugins-loading) tetap dimuat saat allowlist diatur. Lihat [Nilai sumber yang valid hanya dalam daftar kebijakan](#source-values-valid-only-in-policy-lists) | Menghentikan plugin direktori skills dari dimuat                         |
| `hostPattern` | `hostPattern`                        | Tidak diproduksi                                                                                                                                                   | Gagal dimuat: `Unsupported marketplace source type`         | Memungkinkan sumber `github`, `git`, dan `url` yang host-nya cocok                                                                                                                                                                   | Memblokir sumber tersebut                                                |
| `pathPattern` | `pathPattern`                        | Tidak diproduksi                                                                                                                                                   | Gagal dimuat: `Unsupported marketplace source type`         | Memungkinkan sumber `file` dan `directory` yang `path`-nya cocok                                                                                                                                                                     | Memblokir sumber tersebut                                                |

<h3 id="fields-by-type">
  Field menurut tipe
</h3>

Tabel mencantumkan setiap field sumber marketplace yang memiliki default, batasan, atau makna khusus untuk tipenya.

| Field           | Tipe            | Deskripsi                                                                                                                                                                                                                                                               |
| :-------------- | :-------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `url`           | `url`           | Tautan ke file `marketplace.json`. Claude Code hanya mengunduh file itu, jadi plugin marketplace tidak dapat menggunakan [sumber jalur relatif](#relative-path-plugin-source)                                                                                           |
| `url`           | `git`           | Repositori git untuk diklon                                                                                                                                                                                                                                             |
| `headers`       | `url`           | Peta header HTTP yang Claude Code kirim dengan pengambilan, untuk host yang diautentikasi                                                                                                                                                                               |
| `headersHelper` | `url`           | Perintah yang mencetak header yang nilai-nilainya terlalu pendek untuk didaftar dalam `headers`. Memerlukan Claude Code v2.1.238 atau lebih baru. Lihat [Autentikasi unduhan arsip](/docs/id/plugins/host-marketplace#authenticate-archive-downloads)                        |
| `repo`          | `github`        | Dalam `marketplace add` dan `extraKnownMarketplaces`, `repo` harus menamai satu repositori. `marketplace add` menolak `owner/*` sebagai shorthand `owner/repo` yang tidak valid; dalam `extraKnownMarketplaces` Claude Code mengambilnya secara harfiah dan klon gagal  |
| `ref`           | `github`, `git` | Cabang atau tag. Default ke cabang default repositori                                                                                                                                                                                                                   |
| `path`          | `github`, `git` | Jalur file marketplace di dalam repositori. Default ke `.claude-plugin/marketplace.json`                                                                                                                                                                                |
| `path`          | `file`          | File marketplace itu sendiri. Claude Code membacanya di tempat dan mengambil direktori dua level ke atas sebagai akar marketplace, jadi simpan file di `<root>/.claude-plugin/marketplace.json`                                                                         |
| `path`          | `directory`     | Akar marketplace, direktori yang berisi `.claude-plugin/marketplace.json`                                                                                                                                                                                               |
| `sparsePaths`   | `github`, `git` | Array direktori untuk sparse checkout, seperti `[".claude-plugin", "plugins"]`. `claude plugin marketplace add --sparse` menetapkannya                                                                                                                                  |
| `skipLfs`       | `github`, `git` | Diterima dan tidak berpengaruh. Lihat [Simpan file plugin di luar Git LFS](/docs/id/plugins/host-marketplace#keep-plugin-files-out-of-git-lfs)                                                                                                                               |
| `name`          | `settings`      | Harus sama dengan kunci `extraKnownMarketplaces` dan tidak dapat menjadi [nama yang dicadangkan](#reserved-names)                                                                                                                                                       |
| `plugins`       | `settings`      | Katalog inline, tanpa file yang dihosting. Setiap item mengambil `name`, `source`, `description`, `version`, `strict`, `headers`, dan `headersHelper`. Tulis `source` setiap item sebagai tipe objek, karena jalur relatif tidak memiliki repositori untuk diselesaikan |

<h3 id="source-values-valid-only-in-policy-lists">
  Nilai sumber yang valid hanya dalam daftar kebijakan
</h3>

`hostPattern`, `pathPattern`, `skills-dir`, dan bentuk `owner/*` dari `repo` hanya valid dalam dua daftar kebijakan, `strictKnownMarketplaces` dan `blockedMarketplaces`:

* **`hostPattern` dan `pathPattern`**: ekspresi reguler yang Claude Code uji terhadap sumber sebelum mengambil darinya.
* **`skills-dir`**: bukan sumber. Jika Anda menetapkan `strictKnownMarketplaces` sama sekali, [plugin direktori skills](/docs/id/plugins/org#keep-skills-directory-plugins-loading) berhenti dimuat sampai Anda menambahkan `{"source": "skills-dir"}` ke daftar itu.
* **`owner/*`**: sebagai nilai `repo` `github`, cocok dengan setiap repositori di bawah pemilik GitHub yang tepat. Memerlukan Claude Code v2.1.223 atau lebih baru.

Untuk urutan kecocokan, semantik `ref` yang tepat, dan resep, lihat [Kelola plugin untuk organisasi Anda](/docs/id/plugins/org).

<h3 id="source-objects-in-settings">
  Objek sumber di pengaturan
</h3>

Nilai `extraKnownMarketplaces` adalah peta dari nama marketplace ke objek dengan `source`. Entri ini mendaftarkan marketplace dari repositori git di cabang `main`-nya:

```json theme={null}
{
  "extraKnownMarketplaces": {
    "your-marketplace": {
      "source": {
        "source": "git",
        "url": "https://git.example.com/your-org/your-marketplace.git",
        "ref": "main"
      }
    }
  }
}
```

`strictKnownMarketplaces` dan `blockedMarketplaces` adalah array objek sumber. Allowlist ini mengakui satu pemilik GitHub dan satu host internal:

```json theme={null}
{
  "strictKnownMarketplaces": [
    { "source": "github", "repo": "your-org/*" },
    { "source": "hostPattern", "hostPattern": "^git\\.example\\.com$" }
  ]
}
```

<h2 id="validation-messages">
  Pesan validasi
</h2>

`claude plugin validate <path>` mengambil akar marketplace atau file marketplace itu sendiri. Itu mencetak kesalahan dan peringatan. Untuk kode keluar dan `--strict`, lihat [plugin validate](/docs/id/plugins/cli-reference#plugin-validate).

Pesan menamai entri plugin menurut indeksnya, ditulis sebagai `plugins.1.source` atau `plugins[1].source`.

Pesan yang diawali dengan indeks entri dan `plugin.json →`, seperti `plugins[2] plugin.json →`, adalah tentang file plugin itu sendiri. [`claude plugin validate` melaporkan kesalahan](/docs/id/plugins/troubleshooting#claude-plugin-validate-reports-errors) mencantumkan pesan tersebut dengan perbaikannya.

Peringatan yang menyebutkan nama flag Claude Desktop menandai nama yang Claude Code terima tetapi Claude Desktop tolak, karena aturan nama Claude Desktop lebih ketat.

Tabel memetakan pesan tingkat marketplace ke field yang masing-masing tentang.

| Pesan                                                                                                                                                                                         | Level   | Field                                                                                                         |
| :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------ | :------------------------------------------------------------------------------------------------------------ |
| `Marketplace must have a name`                                                                                                                                                                | Error   | `name` kosong                                                                                                 |
| `Marketplace name cannot contain spaces. Use kebab-case (e.g., "my-marketplace")`                                                                                                             | Error   | `name`                                                                                                        |
| `Marketplace name cannot contain path separators (/ or \), ".." sequences, or be "."`                                                                                                         | Error   | `name`                                                                                                        |
| `Marketplace name impersonates an official Anthropic/Claude marketplace`                                                                                                                      | Error   | `name`. Lihat [Nama yang dicadangkan](#reserved-names)                                                        |
| `Marketplace name cannot contain control or bidirectional-formatting characters`                                                                                                              | Error   | `name` berisi karakter kontrol, seperti escape atau newline, atau karakter pemformatan bidirectional Unicode  |
| `Marketplace name "inline" is reserved for --plugin-dir session plugins`, dan varian `builtin`, `skills-dir`, `synced`, `claude-plugin-test`, `npm`, `pip`, `uv`, `cargo`, `github`, dan `gh` | Error   | `name`                                                                                                        |
| `Author name cannot be empty`                                                                                                                                                                 | Error   | `owner.name`                                                                                                  |
| `Plugin name cannot contain spaces. Use kebab-case (e.g., "my-plugin")`                                                                                                                       | Error   | `plugins[i].name`                                                                                             |
| `Plugin name cannot contain control or bidirectional-formatting characters`                                                                                                                   | Error   | `plugins[i].name`                                                                                             |
| `Duplicate plugin name "x" found in marketplace`                                                                                                                                              | Error   | Dua entri berbagi `name`                                                                                      |
| `plugins.i.source: Invalid input`                                                                                                                                                             | Error   | `source` entri tidak cocok dengan tipe apa pun. Lihat [Invalid input pada sumber](#invalid-input-on-a-source) |
| `plugins[i].source: Path contains "..": <path>`                                                                                                                                               | Error   | `source` relatif yang melarikan diri dari akar marketplace                                                    |
| `source.source: 'unsupported' is a parse-time placeholder and cannot be authored`                                                                                                             | Error   | `plugins[i].source`                                                                                           |
| `Plugin "x" sets headersHelper but is not "strict": false`                                                                                                                                    | Error   | `plugins[i].headersHelper`, pada entri `archive`                                                              |
| `chain does not resolve (<reason>) — target must be a name in plugins[], a key in renames, or null`                                                                                           | Error   | `renames.<old>`                                                                                               |
| `target "x" is not a valid plugin name (PluginIdSchema)`                                                                                                                                      | Error   | `renames.<old>`                                                                                               |
| `Unknown field 'x'. Claude Code ignores it at load time.`                                                                                                                                     | Warning | Kunci bernama di tingkat atas, di bawah `metadata`, dalam entri, atau di bawah `relevance` entri              |
| `Marketplace has no plugins defined`                                                                                                                                                          | Warning | `plugins` kosong                                                                                              |
| `Plugin "x" sets headers/headersHelper, which only apply to "archive" sources; they have no effect on this entry.`                                                                            | Warning | `plugins[i].headers` atau `plugins[i].headersHelper`, pada entri yang `source`-nya bukan `archive`            |
| `Plugin "x" fetches its archive with a headersHelper but sets no sha256 pin`                                                                                                                  | Warning | `plugins[i].source.sha256`                                                                                    |
| `Header "x" is a request-routing/identity header that catalog entries may not set; Claude Code drops it at download time.`                                                                    | Warning | `plugins[i].headers.<name>`                                                                                   |
| `Local source "x" is or traverses a symlink, so <path> was not read`                                                                                                                          | Warning | `plugins[i].source`                                                                                           |
| `No marketplace description provided. Adding a description helps users understand what this marketplace offers`                                                                               | Warning | `description`                                                                                                 |
| `Entry declares version "x" but <path>/plugin.json says "y". At install time, plugin.json wins`                                                                                               | Warning | `plugins[i].version`, pada entri jalur relatif                                                                |
| `'relevance' must be an object containing topic and signals; got <type>. It will be ignored at load time.`                                                                                    | Warning | `plugins[i].relevance`                                                                                        |
| `'metadata' must be a free-form object; got <type>. It will be ignored at load time.`                                                                                                         | Warning | `plugins[i].metadata`                                                                                         |
| `'experimental' must be an object containing component declarations; got <type>. It will be ignored at load time.`                                                                            | Warning | `plugins[i].experimental`                                                                                     |
| `Marketplace name "x" is reserved in Claude Desktop`                                                                                                                                          | Warning | `name` adalah `org`, `org-provisioned`, atau `unknown`. Claude Desktop menolak marketplace                    |
| `Marketplace name "x" is not accepted by Claude Desktop (letters, digits, ".", "_", "-"; must start alphanumeric; max 128 chars)`                                                             | Warning | `name`. Claude Desktop menolak marketplace                                                                    |
| `Plugin name "x" is not accepted by Claude Desktop (letters, digits, ".", "_", "-"; must start alphanumeric; max 128 chars)`                                                                  | Warning | `plugins[i].name`. Claude Desktop menjatuhkan entri                                                           |

<h3 id="invalid-input-on-a-source">
  Invalid input pada sumber
</h3>

`Invalid input` pada `source` berarti objek tidak cocok dengan tipe sumber apa pun. Periksa penyebab ini:

* Jalur relatif yang tidak dimulai dengan `./`, selain `"."` atau [nama bare di bawah `metadata.pluginRoot`](#relative-path-plugin-source)
* `package` npm yang berisi `..`
* Tipe `source` yang bukan salah satu dari [sumber plugin](#plugin-sources)
* Tipe yang dikenal dengan field yang diperlukan hilang atau tipe yang salah, seperti `github` tanpa `repo`

<h3 id="failures-that-validation-doesn’t-catch">
  Kegagalan yang validasi tidak tangkap
</h3>

`claude plugin validate` tidak melaporkan setiap kegagalan. Entri `hooks` yang ditulis sebagai jalur file atau array melewati validasi, dan kesalahan muncul hanya ketika plugin dimuat, seperti [Hooks dalam entri](#hooks-in-an-entry) menjelaskan. Kesalahan pengambilan `source` juga muncul hanya setelah install, bukan dalam validasi.

[`claude plugin list`](/docs/id/plugins/cli-reference) menunjukkan plugin yang gagal dimuat dengan kesalahannya, dan [Troubleshoot plugins](/docs/id/plugins/troubleshooting) mencakup string waktu muat.

<h2 id="next-steps">
  Langkah berikutnya
</h2>

* [Buat marketplace](/docs/id/plugins/create-marketplace): bangun marketplace dari field ini dan pasang darinya secara lokal
* [Host dan kelola marketplace](/docs/id/plugins/host-marketplace): di mana menempatkan file dan cara pengguna menerima perubahan
* [Referensi manifest plugin](/docs/id/plugins/manifest-reference): field `plugin.json` yang dapat ditimpa entri
* [Kelola plugin untuk organisasi Anda](/docs/id/plugins/org): resep allowlist dan blocklist yang menggunakan nilai sumber ini
