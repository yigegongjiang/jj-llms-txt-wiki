> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Kelola plugin Claude Code untuk organisasi Anda

> Kontrol plugin mana yang Claude Code instal dan izinkan di seluruh organisasi Anda melalui pengaturan terkelola.

Pengaturan terkelola memungkinkan Anda memutuskan plugin mana yang Claude Code instal dan izinkan di setiap mesin dalam organisasi Anda. Pengguna tidak dapat menggantinya. Anda mengirimkannya baik sebagai [pengaturan terkelola server](/docs/id/server-managed-settings) dari konsol admin claude.ai atau sebagai pengaturan terkelola endpoint melalui MDM atau file `managed-settings.json`. Sebagian besar kontrol di halaman ini hanya berlaku dari pengaturan terkelola.

Halaman ini untuk administrator, dan pengaturan di sini mengatur Claude Code.

<Note>
  Kasus-kasus ini tercakup di halaman lain:

  * **Menginstal plugin untuk diri sendiri**: mulai dari [Install plugins](/docs/id/plugins/install)
  * **Mengontrol plugin mana yang dapat digunakan anggota di claude.ai dan Cowork**: lihat [Manage plugins for your organization](https://support.claude.com/en/articles/13837433) di pusat bantuan
  * **Halaman plugin di pengaturan admin claude.ai**: [**Organization settings > Plugins & skills**](https://claude.ai/admin-settings/skills?tab=inventory) mengaktifkan plugin untuk akun claude.ai anggota, dan yang tersebut mencapai Claude Code sebagai [synced plugins](/docs/id/plugins/loading#synced-plugins). Ini tidak menetapkan salah satu kunci di halaman ini
</Note>

Bagian-bagian mengikuti urutan yang paling sering dilakukan peluncuran: [memerlukan plugin](#pre-install-and-require-plugins) untuk semua orang atau per repositori, [seed containers dan CI](#seed-containers-and-ci), [batasi](#restrict-what-users-can-install) apa yang dapat ditambahkan pengguna sendiri, [tetapkan kebijakan pembaruan](#set-update-policy), kemudian [audit](#audit-and-review) apa yang diinstal. Untuk meninjau setiap kunci kebijakan di satu tempat, lihat [matriks kontrol](#control-matrix).

<h2 id="pre-install-and-require-plugins">
  Pre-install dan require plugins
</h2>

Marketplace adalah katalog plugin yang Claude Code ambil dari repositori git, URL, atau jalur lokal. Setelah Anda mendaftarkan marketplace di mesin, Claude Code dapat menginstal plugin darinya.

Untuk menginstal plugin untuk armada, tetapkan dua kunci bersama-sama dalam [pengaturan terkelola](/docs/id/managed-settings), file kebijakan atau kebijakan yang disampaikan server yang dibaca setiap mesin dalam organisasi Anda: `extraKnownMarketplaces` mendaftarkan marketplace di setiap mesin, dan `enabledPlugins` menamai plugin yang akan diinstal dan diaktifkan darinya. [Pilih mekanisme pengiriman](#choose-a-delivery-mechanism) mencakup cara pengaturan terkelola mencapai setiap mesin.

<h3 id="choose-a-delivery-mechanism">
  Pilih mekanisme pengiriman
</h3>

Pengaturan terkelola mencapai mesin melalui salah satu dari tiga mekanisme pengiriman:

* **Pengaturan terkelola server**: tetapkan kunci plugin sebagai JSON di [**Organization settings > Claude Code > Managed settings**](https://claude.ai/admin-settings/claude-code). Memerlukan peran [Owner](/docs/id/server-managed-settings#access-control) dalam organisasi Claude Anda. Sesi cloud mengambil pengaturan ini sebelum menginstal plugin.
* **Kebijakan MDM**: di macOS, berikan plist yang kunci tingkat atas adalah kunci pengaturan. Di Windows, simpan seluruh dokumen JSON sebagai string dalam nilai registri. Domain plist dan kunci registri ada di [Tempat setiap mekanisme menyimpan kebijakan](/docs/id/managed-settings#where-each-mechanism-stores-the-policy).
* **File pengaturan terkelola**: tempatkan `managed-settings.json` di jalur sistem platform. Anda juga dapat menambahkan file ke direktori drop-in `managed-settings.d/` di sebelahnya. Jalur file per platform ada di [Tempat setiap mekanisme menyimpan kebijakan](/docs/id/managed-settings#where-each-mechanism-stores-the-policy), dan aturan penggabungan drop-in ada di [Pisahkan kebijakan berbasis file di seluruh tim](/docs/id/managed-settings#split-a-file-based-policy-across-teams).

Gunakan pengaturan terkelola server jika Anda memiliki organisasi Claude for Teams atau Enterprise di claude.ai dan perangkat Anda tidak semuanya di bawah MDM. Jika tidak, gunakan kebijakan MDM atau file pengaturan terkelola. Untuk pertukaran, lihat [Pilih antara pengaturan terkelola server dan endpoint](/docs/id/server-managed-settings#choose-between-server-managed-and-endpoint-managed-settings).

<h4 id="which-managed-source-applies-on-a-machine">
  Sumber terkelola mana yang berlaku di mesin
</h4>

Secara default, hanya satu dari tiga sumber ini yang berlaku di mesin. Claude Code menggunakan yang pertama yang mengirimkan kunci kebijakan, memeriksa pengaturan terkelola server terlebih dahulu, kemudian kebijakan MDM, kemudian file pengaturan terkelola. Jika pengaturan terkelola server mengirimkan bahkan satu kunci kebijakan yang tidak terkait, Claude Code mengabaikan kunci plugin dalam kebijakan MDM atau file pengaturan terkelola di mesin itu, terlepas dari [kunci yang dibacanya dari setiap sumber](/docs/id/managed-settings#keys-read-from-every-admin-source).

Untuk menerapkan setiap sumber sebagai gantinya, tetapkan [`managedSourcesBehavior`](/docs/id/managed-settings#compose-every-managed-source) ke `"merge"`.

[Bagaimana Claude Code menggabungkan sumber terkelola](/docs/id/managed-settings#how-claude-code-combines-managed-sources) juga mencantumkan kunci yang Claude Code baca dari setiap sumber dalam kedua mode.

<h3 id="require-a-marketplace-and-its-plugins">
  Memerlukan marketplace dan pluginnya
</h3>

Tambahkan marketplace di bawah `extraKnownMarketplaces`, dikunci dengan `name` marketplace sendiri dari `marketplace.json`-nya. Kemudian tambahkan setiap plugin di bawah `enabledPlugins` sebagai `plugin-name@marketplace-name`. Setiap entri marketplace membawa objek `source` dengan bidang `source` yang menamai tipe, seperti `github`. Contoh pengaturan terkelola ini mendaftarkan marketplace organisasi dan force-enable dua plugin darinya:

```json theme={null}
{
  "extraKnownMarketplaces": {
    "your-marketplace": {
      "source": { "source": "github", "repo": "your-org/your-marketplace" },
      "autoUpdate": true
    }
  },
  "enabledPlugins": {
    "code-formatter@your-marketplace": true,
    "deploy-helper@your-marketplace": true
  }
}
```

Setelah pengaturan mencapai mesin, Claude Code mendaftarkan marketplace dan menginstal dua plugin di awal sesi pengguna berikutnya. Pengguna melihatnya di `/plugin`, dan menonaktifkan satu di cakupan mereka sendiri tidak menghentikannya dari memuat, karena pengaturan terkelola mengambil alih setiap cakupan lain.

Untuk memblokir plugin di setiap cakupan dan menyembunyikannya dari daftar marketplace, tetapkan ke `false` dalam `enabledPlugins` terkelola sebagai gantinya.

Sesuaikan bidang `autoUpdate` dan `source` untuk marketplace Anda:

* **`autoUpdate`**: `true` membuat marketplace dan pluginnya menyegarkan di latar belakang, dan `false` mematikannya. Lihat [Tetapkan kebijakan pembaruan](#set-update-policy).
* **`source`**: `github` adalah salah satu dari beberapa jenis sumber. Sumber `git` mengambil `url` untuk GitLab atau host internal, dan sumber `url` mengambil alamat `marketplace.json` yang dihosting. Setiap bentuk sumber ada di [referensi marketplace](/docs/id/plugins/marketplace-reference).

Jika marketplace adalah repositori git pribadi, setiap pengguna memerlukan akses baca ke dalamnya. Klon marketplace berbasis git berjalan dengan git di mesin pengguna, menggunakan kredensial yang disimpan dan tanpa prompt. Untuk pengguna tanpa akun host git, gunakan [seed](#seed-containers-and-ci) sebagai gantinya.

Entri terkelola juga mengganti entri marketplace dengan nama yang sama atau salinan `--plugin-dir` dari sumber lain:

* **Marketplaces**: entri marketplace terkelola mengganti entri dengan prioritas lebih rendah dengan nama yang sama, dan bidang dua entri tidak digabungkan.
* **Salinan `--plugin-dir`**: `--plugin-dir` memuat plugin dari direktori lokal untuk satu sesi. Untuk apa yang terjadi ketika nama salinan itu cocok dengan plugin yang dinamai `enabledPlugins` terkelola Anda, lihat [Konflik nama](/docs/id/plugins/loading#name-conflicts).

Marketplace resmi Anthropic `claude-plugins-official` tidak memerlukan entri `extraKnownMarketplaces` ketika `enabledPlugins` menetapkan salah satu pluginnya ke `true`. Entri `name@claude-plugins-official` itu mendeklarasikan marketplace oleh dirinya sendiri, di mana pun kunci ini berlaku. Jika Anda tidak mengaktifkan salah satu pluginnya dan masih menginginkannya didaftarkan di setiap mesin, berikan entri eksplisit, seperti yang dilakukan [Izinkan marketplace resmi dan milik Anda sendiri](#allow-the-official-marketplace-and-your-own).

<h3 id="require-plugins-per-repository">
  Memerlukan plugin per repositori
</h3>

Untuk mencakup kontributor satu repositori alih-alih seluruh armada Anda, tetapkan `extraKnownMarketplaces` dan `enabledPlugins` dalam `.claude/settings.json` repositori itu. Entri `extraKnownMarketplaces` hanya berlaku dalam folder yang telah dipercaya kontributor, dan dalam folder yang tidak dipercaya Claude Code mengabaikannya tanpa pesan:

* **Sesi interaktif**: Claude Code mendaftarkan marketplace hanya setelah kontributor menerima [dialog kepercayaan workspace](/docs/id/permissions#what-runs-before-you-trust-a-folder) untuk folder itu.
* **[Lari `-p` non-interaktif](/docs/id/headless)**: entri hanya berlaku dalam folder yang kepercayaannya sudah diterima pengguna secara interaktif, atau yang bendera `hasTrustDialogAccepted` Anda tetapkan dalam `~/.claude.json`.

Plugin yang marketplace cantumkan dengan jalur relatif dimuat dari salinan marketplace setelah entri `extraKnownMarketplaces` repositori berlaku. Plugin yang entri marketplace-nya menunjuk ke sumber eksternal sebagai gantinya, seperti repositori GitHub plugin sendiri, tidak diinstal dari pengaturan repositori saja. Setiap kontributor melihat `Plugin "<name>" is enabled in project settings but isn't installed` sampai mereka menjalankan `claude plugin install <name>@<marketplace> --scope project`, seperti yang dijelaskan [Install plugins](/docs/id/plugins/install).

Jika Anda menggunakan sumber `directory` atau `file` lokal dengan jalur relatif, jalur diselesaikan terhadap checkout utama repositori Anda. Ketika Anda menjalankan Claude Code dari git worktree, jalur masih menunjuk ke checkout utama, jadi semua worktrees berbagi lokasi marketplace yang sama.

Untuk meluncurkan bundel plugin dengan dependensi, letakkan plugin bundel dalam `enabledPlugins`, seperti yang dijelaskan [Plugin dependencies](/docs/id/plugins/dependencies).

<h3 id="when-each-surface-applies-the-plugin-keys">
  Ketika setiap permukaan menerapkan kunci plugin
</h3>

Tabel menunjukkan kapan setiap jenis sesi Claude Code menerapkan `extraKnownMarketplaces` dan `enabledPlugins`, dari pengaturan terkelola dan dari `.claude/settings.json` repositori. Untuk Desktop app dan ekstensi IDE, lihat [Install a plugin](/docs/id/plugins/install#install-a-plugin).

| Permukaan            | Managed `extraKnownMarketplaces` dan `enabledPlugins`                                                                                                                                                                                                                                                                                                           | Repository `.claude/settings.json`                                                                  |
| :------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------- |
| Terminal, interaktif | Diterapkan pada awal sesi di setiap mesin yang menerima pengaturan                                                                                                                                                                                                                                                                                              | `extraKnownMarketplaces` diterapkan setelah kepercayaan; `enabledPlugins` diterapkan pada awal sesi |
| `-p` dan CI          | Diterapkan pada awal sesi, dengan instalasi berjalan di latar belakang                                                                                                                                                                                                                                                                                          | `extraKnownMarketplaces` hanya dalam folder terpercaya; `enabledPlugins` diterapkan                 |
| Sesi cloud           | Dalam lingkungan yang dihosting Anthropic, hanya pengaturan terkelola server yang mencapai sesi, yang menunggu mereka sebelum menginstal plugin. Kebijakan MDM dan file pengaturan terkelola tetap di mesin pengguna. Untuk lingkungan yang dihosting sendiri, lihat [Tempat dan kapan kebijakan berlaku](/docs/id/managed-settings#where-and-when-a-policy-applies) | Lihat tab **Cloud session** di bawah [Install a plugin](/docs/id/plugins/install#install-a-plugin)       |

Dalam lari `-p` atau CI, marketplace dan plugin diinstal di latar belakang, jadi plugin dapat hilang dari putaran pertama. Tetapkan `CLAUDE_CODE_SYNC_PLUGIN_INSTALL=1` untuk membuat lari menunggu instalasi sebelum kueri pertamanya.

<h3 id="confirm-the-rollout">
  Konfirmasi peluncuran
</h3>

Periksa bahwa marketplace dan plugin tiba di mesin atau dalam lari CI:

* **Di satu mesin**: mulai Claude Code dan jalankan `/plugin`. Marketplace dan plugin terdaftar.
* **Dalam CI**: jalankan `claude -p` dengan `--output-format stream-json --verbose`. Acara `init` mencantumkan plugin yang dimuat di bawah `plugins`.

<h2 id="seed-containers-and-ci">
  Seed containers dan CI
</h2>

Untuk image container dan CI runner yang tidak dapat mengklon saat runtime, pra-isi direktori plugin pada waktu build dan arahkan `CLAUDE_CODE_PLUGIN_SEED_DIR` ke dalamnya. Claude Code mendaftarkan marketplace seed pada startup dan memuat cache plugin dari seed di tempat, tanpa mengklon.

Seed juga melayani pengguna yang tidak memiliki akun host git.

<Note>
  Dalam lingkungan CI/CD, konfigurasikan pembantu kredensial git sebelum menginstal plugin dari repositori pribadi. Di GitHub Actions, ekspor token dengan akses baca ke repositori marketplace sebagai `GH_TOKEN`, kemudian jalankan `gh auth setup-git`. Token alur kerja default hanya dapat mengakses repositori alur kerja itu sendiri, jadi marketplace pribadi di repositori lain memerlukan token akses pribadi atau token aplikasi.
</Note>

<Steps>
  <Step title="Instal ke seed pada waktu build">
    Tetapkan `CLAUDE_CODE_PLUGIN_CACHE_DIR` ke jalur seed sehingga marketplace dan plugin diinstal di sana alih-alih `~/.claude/plugins`:

    ```bash theme={null}
    CLAUDE_CODE_PLUGIN_CACHE_DIR=/opt/claude-seed claude plugin marketplace add your-org/your-marketplace
    CLAUDE_CODE_PLUGIN_CACHE_DIR=/opt/claude-seed claude plugin install code-formatter@your-marketplace
    ```

    Seed memiliki tata letak yang sama dengan `~/.claude/plugins`: `known_marketplaces.json`, `marketplaces/<name>/`, dan `cache/<marketplace>/<plugin>/<version>/`. Anda dapat memasang seed di jalur berbeda dari tempat Anda membangunnya.
  </Step>

  <Step title="Arahkan runtime ke seed">
    Tetapkan `CLAUDE_CODE_PLUGIN_SEED_DIR=/opt/claude-seed` dalam lingkungan container. Untuk menggunakan beberapa seed, pisahkan jalurnya dengan `:` di Unix atau `;` di Windows. Claude Code menggunakan seed pertama yang berisi marketplace atau cache plugin yang diberikan.
  </Step>

  <Step title="Aktifkan plugin">
    Plugin dalam seed tidak diaktifkan dengan sendirinya. Tetapkan `enabledPlugins` untuk setiap plugin seed yang ingin Anda muat, dalam pengaturan terkelola atau dalam `.claude/settings.json` repositori.
  </Step>
</Steps>

Untuk memverifikasi seed, jalankan `claude -p` dengan `--output-format stream-json --verbose` dalam image. Dalam daftar `plugins` acara `init`, jalur setiap plugin yang dimuat ada di bawah seed, seperti `/opt/claude-seed/cache/your-marketplace/code-formatter/1.0.0`.

Marketplace seed mengikuti aturan ini:

* **Baca-saja**: Claude Code tidak pernah menulis ke seed dan memaksa `autoUpdate` mati untuk marketplace seed.
* **Entri seed mengambil prioritas**: pada setiap startup, marketplace yang dideklarasikan dalam seed menimpa entri pengguna dengan nama yang sama. Pengguna keluar dari plugin seed dengan `claude plugin disable`, bukan dengan menghapus marketplace.
* **Pembaruan dan penghapusan gagal**: `claude plugin marketplace update <name>` dan `remove` tanpa `--scope` pada marketplace seed gagal dengan pesan yang menamai direktori seed.
* **Kebijakan masih berlaku**: [allowlist dan blocklist](#restrict-what-users-can-install) memeriksa sumber marketplace seed yang tercatat juga. Izinkan sumber yang Anda bangun seed darinya.

Untuk armada tanpa akses git keluar, gabungkan seed dengan sumber marketplace `directory` atau `file` pada mount bersama. Tetapkan `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC=1` juga, yang juga mematikan [plugin auto-update](/docs/id/plugins/loading#when-auto-update-runs). Jika proxy tersedia, lihat [Proxy configuration](/docs/id/network-config#proxy-configuration) untuk variabel yang harus ditetapkan.

<h2 id="restrict-what-users-can-install">
  Batasi apa yang dapat diinstal pengguna
</h2>

Allowlist `strictKnownMarketplaces` terkelola dan blocklist `blockedMarketplaces` memutuskan sumber marketplace mana yang dapat diambil plugin. Sumber marketplace adalah repositori git, URL, atau jalur lokal yang Claude Code ambil darinya. Kedua daftar cocok dengan sumber marketplace yang diambil plugin, bukan entri plugin sendiri di dalam marketplace itu.

Untuk lockdown umum, yang memungkinkan marketplace resmi dan milik Anda sendiri, lihat [Izinkan marketplace resmi dan milik Anda sendiri](#allow-the-official-marketplace-and-your-own). Pasangkan dengan [`disableSideloadFlags`](#control-matrix) sehingga pengguna tidak dapat memuat plugin dari direktori lokal atau URL.

Kedua daftar berlaku sebelum apa pun diunduh dan lagi pada awal sesi:

* **Sebelum unduhan**: daftar berlaku ketika pengguna menambahkan marketplace dan pada setiap instalasi, pembaruan, penyegaran, dan auto-update.
* **Pada awal sesi**: daftar berlaku lagi ke plugin yang sudah diinstal, jadi plugin yang diinstal yang sumber marketplace-nya tidak lagi cocok tidak dimuat. `/plugin` mencantumkannya dengan `Marketplace "<name>" is not in the allowed marketplace list` atau `Marketplace "<name>" is blocked by enterprise policy`.

Tempat dua daftar ditegakkan tergantung di mana Anda menetapkannya:

* **Konsol admin claude.ai**: Claude Code menerapkan kedua daftar dalam sesi yang [membaca pengaturan terkelola server](/docs/id/managed-settings#where-and-when-a-policy-applies). claude.ai juga memeriksanya ketika siapa pun dalam organisasi Anda menambahkan marketplace baru dari repositori git di claude.ai, atau dari **Customize** di Claude Desktop app di luar tab Code-nya. Itu mencakup marketplace yang ditambahkan anggota untuk akun mereka sendiri dan yang ditambahkan untuk seluruh organisasi di bawah [**Organization settings > Plugins**](https://claude.ai/admin-settings/plugins). claude.ai menolak repositori yang allowlist tidak terima atau yang blocklist namai. Ini tidak memeriksa ulang marketplace yang ditambahkan di tempat mana pun sebelum Anda menetapkan daftar, dan tidak memeriksa plugin yang diunggah.
* **File pengaturan terkelola, kebijakan tingkat OS, atau sumber terkelola lainnya**: Claude Code menerapkan kedua daftar di mana ia membaca sumber itu. claude.ai tidak membacanya.

Sementara allowlist apa pun ditetapkan, atau blocklist menamai sumber apa pun selain [`skills-dir`](#blocklist-with-blockedmarketplaces), plugin yang marketplace Claude Code tidak dapat temukan tidak dimuat. `/plugin` menunjukkan kesalahan kebijakan untuk itu daripada kesalahan tidak ditemukan. Kasus umum adalah entri `enabledPlugins` basi untuk marketplace yang tidak ada yang daftarkan.

<h3 id="control-matrix">
  Matriks kontrol
</h3>

Tabel mencantumkan setiap kunci kebijakan plugin, apa yang ditegakkannya, dan apa yang tidak dapat dilakukannya.

| Kunci                                                                    | Apa yang ditegakkannya                                                                                                                                                                                                                                                                           | Apa yang tidak dapat dilakukannya                                                                                                                                                                                                       |
| :----------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `strictKnownMarketplaces`                                                | Allowlist sumber marketplace. `[]` memblokir setiap sumber, termasuk marketplace resmi. Alias: `allowedMarketplaces`                                                                                                                                                                             | Tidak mendaftarkan marketplace, membatasi entri dalam marketplace yang diizinkan, atau memblokir `--plugin-dir`                                                                                                                         |
| `blockedMarketplaces`                                                    | Blocklist sumber marketplace, diperiksa sebelum allowlist                                                                                                                                                                                                                                        | Tidak memblokir marketplace yang sudah didaftarkan dari sumber yang tidak cocok                                                                                                                                                         |
| `syncClaudeAiPlugins`                                                    | Tetapkan `false` untuk menghentikan Claude Code mengunduh dan memuat plugin [disinkronkan dari claude.ai](/docs/id/plugins/loading#synced-plugins) untuk akun setiap pengguna. Memerlukan Claude Code v2.1.273 atau lebih baru                                                                        | Tidak mematikan satu plugin yang disinkronkan. Untuk itu, tetapkan `"<name>@synced": false` dalam [`enabledPlugins`](/docs/id/settings-reference#enabledplugins)                                                                             |
| `enabledPlugins`                                                         | `true` force-enable, `false` memblokir di setiap cakupan dan menyembunyikan plugin                                                                                                                                                                                                               | Tidak menginstal plugin yang marketplace-nya tidak didaftarkan atau diizinkan                                                                                                                                                           |
| `disableSideloadFlags`                                                   | Menolak `--plugin-dir`, `--plugin-url`, `--agents`, opsi `plugins` Agent SDK, dan `--mcp-config` non-SDK pada startup, dan menolak folder yang dinamai dalam variabel [`CLAUDE_CODE_PLUGIN_DIRS`](/docs/id/env-vars#variables) dengan cara yang sama                                                  | Tidak membatasi `.mcp.json`, `claude mcp add`, atau server yang disediakan SDK. Pasangkan dengan [`allowedMcpServers`](/docs/id/managed-mcp)                                                                                                 |
| `disableCommandPluginSources`                                            | Memblokir plugin dengan sumber `command` dari instalasi, pembaruan, atau pemuatan. Sumber `command` adalah sumber yang direktori plugin-nya dihasilkan dengan menjalankan perintah di mesin. Ketika tidak ditetapkan, mengambil nilai `allowManagedHooksOnly`                                    | Tidak mempengaruhi jenis sumber lainnya                                                                                                                                                                                                 |
| `allowManagedHooksOnly`                                                  | Membatasi hook mana yang berjalan. Lihat [`allowManagedHooksOnly`](/docs/id/settings-reference#allowmanagedhooksonly)                                                                                                                                                                                 | Tidak mempercayai hook dari plugin yang diaktifkan pengguna sendiri                                                                                                                                                                     |
| `strictPluginOnlyCustomization`                                          | Memblokir skills, agents, hooks, dan server MCP yang tidak berasal dari plugin, pengaturan terkelola, atau built-in Claude Code. Tetapkan `true` untuk mencakup semua empat jenis, atau array nilai `skills`, `agents`, `hooks`, dan `mcp` seperti `["skills", "hooks"]` untuk mencakup beberapa | Tidak membatasi plugin mana yang diinstal pengguna. Pasangkan dengan `strictKnownMarketplaces`                                                                                                                                          |
| `pluginSuggestionMarketplaces`                                           | Marketplace yang pluginnya dapat muncul sebagai saran instalasi. Lihat [Recommend plugins](#recommend-plugins)                                                                                                                                                                                   | Tidak mempengaruhi tips built-in                                                                                                                                                                                                        |
| `pluginTrustMessage`                                                     | Menambahkan teks Anda ke peringatan kepercayaan yang ditunjukkan `/plugin` sebelum plugin diinstal                                                                                                                                                                                               | Tidak mengubah teks peringatan itu sendiri                                                                                                                                                                                              |
| `allowedChannelPlugins`                                                  | Mengganti daftar default plugin yang diizinkan untuk mendorong pesan channel. Memerlukan `channelsEnabled: true`                                                                                                                                                                                 | Lihat [Restrict which channel plugins can run](/docs/id/channels#restrict-which-channel-plugins-can-run)                                                                                                                                     |
| [`CLAUDE_CODE_DISABLE_OFFICIAL_MARKETPLACE_AUTOINSTALL=1`](/docs/id/env-vars) | Menghentikan sesi terminal interaktif dari auto-registering marketplace resmi                                                                                                                                                                                                                    | Tidak menghapus marketplace yang sudah didaftarkan. Allowlist dan blocklist gerbang auto-registering yang sama tanpanya. Mesin yang dimulai sekali dengan itu ditetapkan tidak melanjutkan auto-registering setelah Anda membatalkannya |

Setiap kunci dalam tabel adalah pengaturan terkelola, terlepas dari `enabledPlugins`, `syncClaudeAiPlugins`, dan `CLAUDE_CODE_DISABLE_OFFICIAL_MARKETPLACE_AUTOINSTALL`:

* **`enabledPlugins`**: Anda dapat menetapkannya dalam cakupan apa pun, dan pengaturan terkelola menguncinya.
* **`syncClaudeAiPlugins`**: setiap pengguna juga dapat menetapkannya dalam pengaturan pengguna atau lokal mereka sendiri. Lihat [cakupannya dalam referensi pengaturan](/docs/id/settings-reference#syncclaudeaiplugins).
* **`CLAUDE_CODE_DISABLE_OFFICIAL_MARKETPLACE_AUTOINSTALL`**: ini adalah variabel lingkungan yang Anda berikan melalui blok `env` terkelola yang ditunjukkan di bawah [Matikan pembaruan untuk seluruh armada](#turn-updates-off-for-the-whole-fleet).

Setiap kunci pengaturan di sini memiliki entri dalam [referensi pengaturan](/docs/id/settings-reference).

<h4 id="aliases-for-the-marketplace-keys">
  Alias untuk kunci marketplace
</h4>

`strictKnownMarketplaces` juga dapat dieja `allowedMarketplaces`, dan `extraKnownMarketplaces` juga dapat dieja `additionalMarketplaces`.

* **Versi**: alias memerlukan Claude Code v2.1.232 atau lebih baru, dan klien yang lebih lama mengabaikannya. Dalam file yang dibaca armada campuran, pertahankan nama kanonik.
* **Kedua ejaan ditetapkan**: ketika file menetapkan kedua ejaan, nilai kunci kanonik berlaku.

<h3 id="allowlist-with-strictknownmarketplaces">
  Allowlist dengan `strictKnownMarketplaces`
</h3>

Tetapkan allowlist ke daftar objek sumber ini. Sebagian besar entri cocok persis, entri `hostPattern` dan `pathPattern` cocok sebagai ekspresi reguler, dan wildcard pemilik `github` cocok berdasarkan pemilik:

* **`github`**: `{ "source": "github", "repo": "your-org/approved-plugins" }`, dengan `ref` dan `path` opsional.
* **Wildcard pemilik `github`**: `{ "source": "github", "repo": "your-org/*" }` cocok dengan setiap repositori di bawah pemilik itu. `*` harus berdiri untuk seluruh nama repositori. Claude Code mengabaikan entri seperti `*/plugins` dan `your-org/tools-*` sebagai tidak valid, jadi mereka tidak cocok dengan apa pun. Memerlukan Claude Code v2.1.223 atau lebih baru.
* **`git`**: `{ "source": "git", "url": "https://gitlab.example.com/tools/plugins.git" }`, dengan `ref` dan `path` opsional.
* **`url`**: `{ "source": "url", "url": "https://plugins.example.com/marketplace.json" }`, dengan `headers` opsional.
* **`file` dan `directory`**: `{ "source": "file", "path": "/opt/marketplace/marketplace.json" }` atau `{ "source": "directory", "path": "/opt/marketplace/plugins" }`, dengan jalur absolut.
* **`hostPattern`**: `{ "source": "hostPattern", "hostPattern": "^github\\.example\\.com$" }`, dicocokkan terhadap host sumber `github`, `git`, dan `url`. Pola cocok di mana saja dalam nama host, jadi jangkarnya dengan `^` dan `$` seperti yang ditunjukkan untuk mencocokkan seluruh host. Sumber `github` selalu dihitung sebagai `github.com`. Gunakan entri `hostPattern` untuk GitHub Enterprise Server atau host GitLab di mana pengembang membuat marketplace mereka sendiri. [Halaman GHES](/docs/id/github-enterprise-server#allowlist-ghes-marketplaces-in-managed-settings) memiliki contoh yang dikerjakan.
* **`pathPattern`**: `{ "source": "pathPattern", "pathPattern": "^/opt/approved/" }`, dicocokkan terhadap `path` sumber `file` dan `directory`. Pola cocok di mana saja dalam jalur, jadi mulai dengan `^` untuk menjepit awalan direktori. `".*"` memungkinkan setiap jalur lokal.
* **`skills-dir`**: `{ "source": "skills-dir" }` membuat [plugin direktori skills](#keep-skills-directory-plugins-loading) terus memuat saat allowlist ditetapkan, dan tidak cocok dengan marketplace apa pun.

<h4 id="how-entries-match">
  Bagaimana entri cocok
</h4>

Entri `url` cocok pada nilai `url`-nya; `headers` tidak dibandingkan. Untuk entri `github` dan `git`, `repo` atau `url`, `ref`, dan `path` harus semuanya cocok, atau tidak ada di kedua sisi:

* Entri tanpa `ref` tidak mencakup sumber dengan `ref: "main"`.
* Entri untuk `your-org/your-marketplace` tidak mencakup URL `git` yang mengklon repositori yang sama.
* Garis miring trailing, akhiran `.git`, atau `ssh://` sebagai pengganti `https://` adalah nilai berbeda. Ketika marketplace dapat diklon oleh lebih dari satu URL, lebih suka entri `hostPattern`.

Entri wildcard pemilik mengikuti aturan persis untuk `ref` dan cocok dengan `path` apa pun di dalam repositori kecuali entri menjepit satu. Pencocokan wildcard peka huruf besar-kecil pada allowlist.

<h4 id="keep-skills-directory-plugins-loading">
  Jaga plugin direktori skills tetap memuat
</h4>

Plugin direktori skills adalah plugin yang pengguna simpan di bawah `~/.claude/skills/` atau `.claude/skills/` proyek dalam folder yang membawa `.claude-plugin/plugin.json`. Jika Anda menetapkan allowlist apa pun tanpa entri `{ "source": "skills-dir" }`, mereka berhenti memuat. [Skills](/docs/id/skills) biasa, artinya `SKILL.md` tanpa manifest itu, terus memuat.

<h4 id="marketplaces-hosted-on-claude-ai">
  Marketplace yang dihosting di claude.ai
</h4>

Allowlist dan blocklist cocok dengan [marketplace yang dihosting di claude.ai](/docs/id/plugins/install#add-from-claude-ai) berdasarkan hostnya. Untuk memungkinkan atau memblokir satu, tambahkan entri `hostPattern` yang cocok dengan `claude.ai` ke `strictKnownMarketplaces` atau `blockedMarketplaces`. Pada allowlist, entri seperti itu mengakui marketplace claude.ai organisasi Anda dan marketplace default claude.ai, tetapi bukan marketplace yang dibuat dari unggahan claude.ai anggota sendiri atau yang cakupannya claude.ai tidak nyatakan. Memerlukan Claude Code v2.1.273 atau lebih baru.

<h4 id="lock-every-source-out">
  Kunci setiap sumber
</h4>

Allowlist kosong, `[]`, mengunci setiap sumber marketplace, termasuk marketplace resmi.

Lockdown ini tidak mencakup plugin [disinkronkan dari claude.ai](/docs/id/plugins/loading#synced-plugins), yang Claude Code unduh dari akun setiap pengguna daripada dari marketplace. Untuk menghentikan yang juga, tetapkan [`syncClaudeAiPlugins`](/docs/id/settings-reference#syncclaudeaiplugins) ke `false` dalam pengaturan terkelola, atau matikan Skills untuk organisasi Anda di claude.ai.

<h3 id="blocklist-with-blockedmarketplaces">
  Blocklist dengan `blockedMarketplaces`
</h3>

`blockedMarketplaces` mengambil objek sumber yang sama dengan [`strictKnownMarketplaces`](#allowlist-with-strictknownmarketplaces) dan diperiksa terlebih dahulu, jadi sumber pada kedua daftar diblokir. Pencocokan blocklist lebih luas daripada pencocokan allowlist:

* URL Git dikanonikalisasi, jadi bentuk `git@` dan `https://`, akhiran `.git`, dan garis miring trailing dari satu repositori `github.com` semuanya cocok dengan entri yang sama.
* Entri `github` juga memblokir URL `git` yang setara, dan sebaliknya.
* Untuk entri `owner/*`, perbandingan pemilik tidak peka huruf besar-kecil.
* Entri tanpa `ref` atau `path` memblokir setiap ref dan path dari repositori yang cocok.

Entri ini memblokir setiap repositori di bawah satu pemilik GitHub:

```json theme={null}
{
  "blockedMarketplaces": [
    { "source": "github", "repo": "untrusted-org/*" }
  ]
}
```

Entri `url` dalam `blockedMarketplaces` juga berlaku ketika pengguna menambahkan URL repositori `https://` yang Claude Code [klon daripada ambil](/docs/id/plugins/cli-reference#plugin-marketplace-add), seperti URL repositori `github.com` atau `gitlab.com` biasa. Pengguna tidak dapat menambahkan URL itu jika entri menamakannya. Pencocokan mengabaikan akhiran `.git` dan ref apa pun yang ditambahkan pengguna setelah `#`. Memerlukan Claude Code v2.1.232 atau lebih baru.

Entri `{ "source": "skills-dir" }` di sini menghentikan [plugin direktori skills](#keep-skills-directory-plugins-loading) dari memuat, dari `~/.claude/skills/` dan `.claude/skills/` proyek.

Blocklist yang hanya menamai entri itu tidak dihitung sebagai pembatasan aktif, jadi tidak [menghentikan plugin yang marketplace Claude Code tidak dapat temukan](#restrict-what-users-can-install) dari memuat.

<h3 id="allow-the-official-marketplace-and-your-own">
  Izinkan marketplace resmi dan milik Anda sendiri
</h3>

Sebagian besar organisasi memungkinkan marketplace resmi dan milik mereka sendiri, dan mendaftarkan keduanya sehingga setiap mesin memilikinya. Kebijakan pengaturan terkelola ini memungkinkan kedua marketplace, mendaftarkan keduanya, force-enable dua plugin, dan menolak `--plugin-dir`:

```json theme={null}
{
  "strictKnownMarketplaces": [
    { "source": "github", "repo": "anthropics/claude-plugins-official" },
    { "source": "github", "repo": "your-org/*" },
    { "source": "skills-dir" }
  ],
  "extraKnownMarketplaces": {
    "claude-plugins-official": {
      "source": { "source": "github", "repo": "anthropics/claude-plugins-official" }
    },
    "your-marketplace": {
      "source": { "source": "github", "repo": "your-org/your-marketplace" }
    }
  },
  "enabledPlugins": {
    "code-formatter@your-marketplace": true,
    "deploy-helper@your-marketplace": true
  },
  "disableSideloadFlags": true
}
```

Pada mesin dengan kebijakan ini, menambahkan sumber apa pun di luar daftar, misalnya `/plugin marketplace add https://example.com/other-marketplace.git`, gagal dengan pesan yang berisi `is blocked by enterprise policy` diikuti oleh sumber yang diizinkan. `claude --plugin-dir ./x` keluar dengan pesan yang menamai `disableSideloadFlags`.

Entri `{ "source": "skills-dir" }` membuat [plugin direktori skills](#keep-skills-directory-plugins-loading) terus memuat di bawah allowlist ini. Hapus entri itu dan mereka berhenti memuat.

Daftarkan kedua marketplace dengan entri `extraKnownMarketplaces` eksplisit, seperti yang dilakukan kebijakan ini, daripada mengandalkan allowlist atau marketplace resmi mendaftarkan dirinya sendiri:

* **Allowlist tidak mendaftarkan apa pun**: entri `extraKnownMarketplaces` melakukannya, dan itu harus melewati allowlist itu sendiri. Claude Code menolak untuk mendaftarkan marketplace terkelola yang sumber allowlist tidak cocok.
* **Marketplace resmi mendaftarkan dirinya sendiri hanya dalam sesi terminal interaktif**: bahkan di sana, itu mendaftarkan hanya ketika allowlist mengizinkannya. Lari `-p` atau terminal yang terpasang ke sesi cloud tidak pernah mendaftarkannya.
* **Upaya yang diblokir diingat**: jika mesin pernah berjalan di bawah kebijakan yang memblokir marketplace resmi, Claude Code mencatat upaya yang diblokir dan tidak mencoba ulang setelah kebijakan berubah. Lockdown `[]` adalah satu kebijakan seperti itu. Mesin itu mendaftarkannya lagi hanya melalui entri `extraKnownMarketplaces` seperti yang ada dalam kebijakan ini, entri `enabledPlugins` untuk salah satu pluginnya, atau `/plugin marketplace add` manual.

<h2 id="set-update-policy">
  Tetapkan kebijakan pembaruan
</h2>

Anda dapat menetapkan kebijakan pembaruan per marketplace, untuk seluruh armada, atau per grup pengguna melalui saluran rilis.

<h3 id="turn-auto-update-on-or-off-per-marketplace">
  Aktifkan atau matikan auto-update per marketplace
</h3>

Plugin auto-update berjalan di latar belakang setelah startup untuk marketplace yang memilikinya diaktifkan. Untuk marketplace mana yang memilikinya aktif secara default, lihat [Kapan auto-update berjalan](/docs/id/plugins/loading#when-auto-update-runs). Untuk memutuskan untuk armada, tetapkan `"autoUpdate": true` atau `false` pada entri `extraKnownMarketplaces` terkelola:

* Jika entri terkelola menetapkan bidang, Claude Code menolak toggle `/plugin` pengguna dengan kesalahan yang dimulai `Auto-update for '<name>' is set by`.
* Jika entri terkelola membiarkan bidang tidak ditetapkan, toggle pengguna bertahan.

<h3 id="turn-updates-off-for-the-whole-fleet">
  Matikan pembaruan untuk seluruh armada
</h3>

Untuk mematikan plugin auto-update untuk setiap marketplace, tetapkan `DISABLE_AUTOUPDATER` dalam blok `env` terkelola, seperti yang dilakukan contoh ini. Variabel yang sama juga menghentikan pembaruan Claude Code sendiri:

```json theme={null}
{
  "env": {
    "DISABLE_AUTOUPDATER": "1"
  }
}
```

Untuk menghentikan pembaruan Claude Code sendiri tetapi menjaga plugin auto-update, tambahkan `"FORCE_AUTOUPDATE_PLUGINS": "1"` ke blok yang sama. [Variabel lingkungan lainnya yang menghentikan plugin auto-update](/docs/id/plugins/loading#when-auto-update-runs) bekerja dengan cara yang sama.

`DISABLE_AUTOUPDATER` tidak mencakup plugin dengan [sumber `command`](/docs/id/plugins/marketplace-reference#command-plugin-source). Claude Code menjalankan kembali perintah setiap plugin yang diaktifkan setiap sesi dan menginstal output ketika berubah. Untuk apa yang menghentikan lari itu, lihat [Kapan sumber command menjalankan kembali](/docs/id/plugins/loading#when-a-command-source-re-runs).

<h3 id="assign-release-channels-to-user-groups">
  Tetapkan saluran rilis ke grup pengguna
</h3>

Untuk menjalankan saluran stabil dan akses awal, host dua marketplace yang menunjuk ke ref berbeda dari plugin yang sama. Kemudian berikan setiap grup pengguna marketplace-nya sendiri melalui pengaturan terkelola endpoint terpisah atau kebijakan gateway. Pengaturan terkelola server dari konsol admin [berlaku untuk setiap pengguna dalam organisasi Anda](/docs/id/server-managed-settings#current-limitations), jadi mereka tidak dapat menetapkan pengaturan berbeda untuk grup berbeda.

* Terapkan [pengaturan terkelola endpoint](/docs/id/managed-settings#delivery-mechanisms) terpisah, seperti file pengaturan terkelola atau profil MDM, ke perangkat setiap grup. Untuk memeriksa apakah file per-grup atau profil berlaku pada perangkat yang juga memiliki sumber organisasi-lebar, lihat [Bagaimana Claude Code menggabungkan sumber terkelola](/docs/id/managed-settings#precedence-within-the-managed-tier).
* Tentukan satu [kebijakan gateway aplikasi Claude](/docs/id/claude-apps-gateway-config#managed) per grup. Gateway menerapkan kebijakan pertama yang aturan kecocokannya sesuai dengan pengguna, jadi urutkan kebijakan sehingga setiap pengguna mencapai kebijakan grup mereka. Peta `extraKnownMarketplaces` kebijakan itu tidak digabungkan dengan peta kebijakan lain, jadi cantumkan setiap marketplace yang dibutuhkan grup di dalamnya, bukan hanya marketplace saluran-nya.

Dengan mekanisme apa pun, grup stabil menerima konfigurasi ini:

```json theme={null}
{
  "extraKnownMarketplaces": {
    "stable-tools": {
      "source": { "source": "github", "repo": "your-org/stable-tools" }
    }
  }
}
```

Grup akses awal menerima `latest-tools` sebagai gantinya. Untuk menyiapkan dua marketplace, lihat [Jalankan saluran rilis](/docs/id/plugins/host-marketplace#run-release-channels).

<h2 id="recommend-plugins">
  Rekomendasikan plugin
</h2>

Pemilik marketplace dapat melampirkan sinyal `relevance` ke entri sehingga Claude Code menyarankan plugin ketika proyek cocok.

Saran dari marketplace hanya muncul ketika itu didaftarkan di mesin pengguna, Anda mencantumkan namanya dalam `pluginSuggestionMarketplaces` dalam pengaturan terkelola, dan Anda mendeklarasikan sumbernya dalam kebijakan yang sama. Deklarasikan sumber baik sebagai entri `extraKnownMarketplaces` marketplace atau sebagai entri allowlist. Marketplace resmi hanya memerlukan nama. Lihat [Aktifkan saran dalam pengaturan terkelola](/docs/id/plugins/relevance#enable-suggestions-in-managed-settings).

<h2 id="audit-and-review">
  Audit dan tinjau
</h2>

Acara OpenTelemetry dan Analytics API memberi tahu Anda apa yang diinstal dan dijalankan armada Anda.

Untuk apa yang dapat dijalankan plugin di mesin dan apa yang diizinkan setiap tingkat kepercayaan, baca [Plugin security](/docs/id/plugins/security) sebelum Anda menyetujui marketplace.

<h3 id="opentelemetry-events">
  Acara OpenTelemetry
</h3>

`claude_code.plugin_installed` mencatat setiap instalasi, dan `claude_code.plugin_loaded` mencatat setiap plugin yang diaktifkan pada awal sesi. Kedua acara menyunting atau menghilangkan nama plugin dan marketplace pihak ketiga kecuali Anda menetapkan `OTEL_LOG_TOOL_DETAILS=1`, seperti yang ditunjukkan [Nama plugin yang disunting dalam backend Anda](/docs/id/plugins/measure#redacted-plugin-names-in-your-backend). Daftar bidang ada di bawah [Plugin installed event](/docs/id/monitoring-usage#plugin-installed-event) dan [Plugin loaded event](/docs/id/monitoring-usage#plugin-loaded-event).

<h3 id="analytics-api">
  Analytics API
</h3>

Pada paket Enterprise, `GET /v1/organizations/analytics/plugins` mengembalikan instalasi per-plugin, per-hari dan jumlah invokasi di seluruh Claude Code dan Cowork. Anda dapat mengelompokkan jumlah berdasarkan pengguna atau grup RBAC. Aktivitas plugin yang mencapai Anthropic tanpa nama plugin muncul dalam satu baris `third-party` agregat. Lihat [referensi endpoint](https://platform.claude.com/docs/en/api/admin/analytics/plugins/list) dan [Akses data secara terprogram](/docs/id/analytics#access-data-programmatically) untuk kunci yang diperlukannya.

<h2 id="plan-for-what-managed-settings-can’t-enforce">
  Rencanakan apa yang tidak dapat ditegakkan pengaturan terkelola
</h2>

Permintaan ini dari tinjauan keamanan tidak memiliki kunci khusus dalam skema pengaturan saat ini. Kontrol yang ada terdekat adalah:

* **Penargetan per-pengguna atau per-grup**: setiap kunci plugin berlaku untuk setiap pengguna yang menerima pengaturan. Pengaturan terkelola server mengirimkan satu konfigurasi per organisasi. Untuk kebijakan per-grup, gunakan pengaturan terkelola endpoint terpisah atau kebijakan gateway, seperti di bawah [Tetapkan saluran rilis ke grup pengguna](#assign-release-channels-to-user-groups).
* **Membatasi entri dalam marketplace yang diizinkan**: allowlist cocok dengan sumber marketplace. Untuk memblokir satu plugin dari marketplace yang diizinkan, tetapkan ke `false` dalam `enabledPlugins` terkelola.
* **Menyembunyikan `/plugin`**: tidak ada kunci yang menonaktifkan perintah. Setara terdekat menggabungkan allowlist yang hanya menamai marketplace Anda, entri `enabledPlugins` terkelola untuk plugin yang Anda sediakan, dan `disableSideloadFlags`.
* **Gating `--plugin-dir` melalui allowlist**: allowlist tidak mencakup `--plugin-dir`. `disableSideloadFlags` melakukannya.
* **Menerapkan toggle plugin claude.ai melalui kunci ini**: [**Organization settings > Plugins & skills**](https://claude.ai/admin-settings/skills?tab=inventory) tidak menetapkan kunci di halaman ini. Apa yang diaktifkan anggota dan organisasi Anda di sana mencapai CLI sebagai [synced plugins](/docs/id/plugins/loading#synced-plugins), yang memiliki kontrol mereka sendiri.

<h2 id="troubleshoot-policy">
  Troubleshoot kebijakan
</h2>

Jika kebijakan plugin tidak berperilaku seperti yang diharapkan di mesin, periksa gejala ini terlebih dahulu:

* **File terkelola tidak diurai**: ketika `managed-settings.json` bukan JSON yang valid, Claude Code menolak untuk memulai dan mencetak [kesalahan yang menamai file](/docs/id/errors#managed-settings-document-could-not-be-parsed). File yang diurai tetapi memiliki satu entri tidak valid menyimpan sisa kebijakan-nya. Lihat [Entri tidak valid dalam pengaturan terkelola](/docs/id/managed-settings#invalid-entries-in-managed-settings).
* **Sumber terkelola tidak dimuat**: jalankan `/status` dan cari `Enterprise managed settings` dalam baris `Setting sources`. Jika hilang, sumber tidak dimuat.
* **Pengguna melaporkan `blocked by enterprise policy`**: pesan menamai marketplace atau sumbernya. Untuk allowlist, itu juga mencantumkan sumber yang diizinkan. Entri yang menghadap pengguna ada di [Troubleshoot plugins](/docs/id/plugins/troubleshooting).
* **Plugin yang dinonaktifkan pengguna dalam `~/.claude/settings.json` masih dimuat**: sumber pengaturan lain mengaktifkannya kembali, seperti entri `enabledPlugins` terkelola yang force-enable-nya. `/plugin` dan `claude plugin list` menunjukkan `Disabled in ~/.claude/settings.json but still loads` dengan sumber pengaturan itu.

<h2 id="next-steps">
  Langkah berikutnya
</h2>

* [Referensi marketplace](/docs/id/plugins/marketplace-reference#marketplace-sources): nilai `source` yang `extraKnownMarketplaces`, `strictKnownMarketplaces`, dan `blockedMarketplaces` terima
* [Host dan pertahankan marketplace](/docs/id/plugins/host-marketplace): jalankan marketplace yang kebijakan Anda arahkan
* [Plugin security dan trust](/docs/id/plugins/security): apa yang dapat dilakukan plugin di mesin dan cara meninjau satu sebelum menginstal
* [Pengaturan terkelola server](/docs/id/server-managed-settings): berikan kunci ini dari konsol admin claude.ai
* [Troubleshoot plugins](/docs/id/plugins/troubleshooting#blocked-by-your-organization): pesan yang dilihat pengguna ketika kebijakan memblokir mereka
