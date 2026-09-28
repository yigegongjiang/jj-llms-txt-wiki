> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Referensi pemuatan plugin

> Lacak dari mana Claude Code memuat setiap plugin, file pengaturan mana yang menentukan apakah plugin dimuat, dan mengapa pembaruan tidak mengubah apa pun.

Gunakan halaman ini ketika plugin tidak dimuat, memuat salinan berbeda dari yang Anda harapkan, atau tidak mengambil pembaruan, dan Anda ingin melihat sumber, cakupan pengaturan, atau file di disk mana yang menentukan hal tersebut. Halaman ini memberikan aturan yang diterapkan Claude Code ketika sesi dimulai dan setiap kali Anda menjalankan `/reload-plugins`. Anda juga dapat meminta Claude untuk membaca halaman ini dan mendiagnosis pengaturan Anda.

<Note>
  Kasus-kasus ini tercakup di halaman lain:

  * **Langkah-langkah install, enable, disable, dan update**: lihat [Install dan kelola plugin](/docs/id/plugins/install)
  * **Anda memiliki pesan kesalahan spesifik**: lihat [Troubleshoot plugin](/docs/id/plugins/troubleshooting)
</Note>

Mulai dengan [Periksa tahap mana yang dicapai plugin](#check-which-stage-a-plugin-reached) untuk tiga tahap yang dilalui plugin yang terinstal, atau buka bagian yang sesuai dengan apa yang Anda lihat:

* Plugin yang Anda matikan masih dimuat: [Temukan di mana plugin diaktifkan](#find-where-a-plugin-is-enabled)
* Pembaruan tidak mengubah apa pun: [Versi dan pembaruan](#versions-and-updates)
* Anda melihat file di bawah `~/.claude/plugins/`: [Temukan plugin di disk](#find-plugins-on-disk)
* Plugin `--plugin-dir` tidak dimuat, atau plugin dengan nama yang sama dimuat sebagai gantinya: [Konflik nama](#name-conflicts)

<h2 id="check-which-stage-a-plugin-reached">
  Periksa tahap mana yang dicapai plugin
</h2>

Entri `enabledPlugins` menjadi plugin yang dapat Anda gunakan dalam tahap: pengaturan Anda mendeklarasikannya, Claude Code mengambilnya ke disk, dan sesi yang berjalan memuatnya. Ketika plugin tidak berperilaku seperti yang disarankan file pengaturan, periksa tahap mana yang dicapainya:

* **Dideklarasikan, dalam pengaturan**: `enabledPlugins` mengatakan plugin mana yang harus aktif, dan `extraKnownMarketplaces` mengatakan marketplace mana yang harus ada. Ketika Anda menjalankan `claude plugin marketplace add`, Claude Code menulis marketplace ke `extraKnownMarketplaces` dalam pengaturan pengguna Anda serta ke disk
* **Diambil, di disk di bawah `~/.claude/plugins/`**: catatan apa yang telah diambil Claude Code, dan file yang diambil itu sendiri:
  * `known_marketplaces.json` mencatat setiap marketplace yang telah diambil Claude Code, dengan `source`, `installLocation`, `lastUpdated`, dan `autoUpdate`. Ada satu `known_marketplaces.json` per pengguna, jadi marketplace yang Anda tambahkan dalam satu proyek tersedia di setiap proyek
  * `installed_plugins.json` mencatat setiap install dengan `scope`, `installPath`, dan `version`
  * `cache/` menyimpan file plugin
* **Dimuat, dalam sesi yang berjalan**: set plugin yang dimuat Claude Code saat startup atau di `/reload-plugins` terakhir. Perubahan pada pengaturan atau disk tidak mencapai lapisan ini sampai Anda menjalankan `/reload-plugins` atau memulai sesi baru. Itulah mengapa `claude plugin update` berakhir dengan `Restart to apply changes.` dan pembaruan latar belakang memberi tahu Anda dengan `Run /reload-plugins to apply`

<h3 id="plugins-and-marketplaces-that-aren’t-on-disk-at-session-start">
  Plugin dan marketplace yang tidak ada di disk saat startup sesi
</h3>

Plugin dimuat saat startup sesi dari `installed_plugins.json` dan cache tanpa menggunakan jaringan. Setelah sesi dimulai, Claude Code memeriksa marketplace yang dideklarasikan di latar belakang:

* **Marketplace yang dideklarasikan pengaturan tetapi `known_marketplaces.json` tidak memiliki**: Claude Code mengklonnya, kemudian memuat ulang plugin dan mengunduh plugin yang diaktifkan yang belum di-cache
* **Marketplace yang dideklarasikan yang sumbernya berubah dalam pengaturan**: Claude Code mengambilnya kembali dari sumber baru dan menampilkan `Plugins changed. Run /reload-plugins to activate.`

Plugin yang diaktifkan yang tidak diambil oleh jalur mana pun dan yang tidak memiliki direktori cache yang dapat digunakan menampilkan `Plugin "<name>" not cached at <path>` di tab **Errors** `/plugin`, dan `claude plugin list` menambahkan `— run /plugin to refresh` ke baris yang sama. Untuk perbaikannya, lihat [`Plugin "<name>" not cached at <path>`](/docs/id/plugins/troubleshooting#plugin-not-cached-at).

<h2 id="find-where-a-plugin-came-from">
  Temukan dari mana plugin berasal
</h2>

Setiap plugin memiliki id dalam bentuk `<name>@<origin>`, yang merupakan apa yang Anda lihat dalam file pengaturan dan dalam `claude plugin list --json`. Bagian setelah `@` memberi tahu Anda di mana Claude Code menemukan plugin:

| ID berakhir dengan | Bagaimana plugin sampai di sana                                                                                                                                                                                        | Bagaimana Anda mengaktifkan atau menonaktifkannya                                                                                                                                                   |
| :----------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `@<marketplace>`   | Anda menginstalnya dari marketplace yang Anda tambahkan                                                                                                                                                                | `"<name>@<marketplace>": true` atau `false` di bawah `enabledPlugins` dalam file pengaturan                                                                                                         |
| `@inline`          | Anda memulai Claude Code dengan `--plugin-dir` atau `--plugin-url`, mengatur [`CLAUDE_CODE_PLUGIN_DIRS`](/docs/id/env-vars#variables), atau aplikasi Agent SDK meneruskan opsi `plugins`. Plugin dimuat untuk sesi itu saja | Aktif untuk sesi kecuali manifest menetapkan `defaultEnabled: false` atau file pengaturan menetapkan `"<name>@inline": false`                                                                       |
| `@skills-dir`      | Anda menyimpan direktori plugin yang memiliki `.claude-plugin/plugin.json` di bawah `~/.claude/skills/` atau `.claude/skills/` proyek                                                                                  | `defaultEnabled` manifest, kecuali file pengaturan menetapkan `"<name>@skills-dir"` ke `true` atau `false`                                                                                          |
| `@synced`          | Anda atau organisasi Anda mengaktifkannya untuk akun claude.ai Anda, dan Claude Code [mengunduhnya](#synced-plugins)                                                                                                   | Aktif kecuali manifest menetapkan `defaultEnabled: false` atau file pengaturan menetapkan `"<name>@synced": false`. Plugin yang ditandai organisasi Anda sebagai wajib dimuat terlepas dari apa pun |

Untuk plugin marketplace, `<name>` adalah nama entri dalam `marketplace.json`; untuk `@inline` dan `@skills-dir` ini adalah `name` dalam manifest plugin.

Nama asal dalam tabel ini dicadangkan, jadi tidak ada marketplace yang dapat dinamai `inline`, `skills-dir`, atau `synced`.

<h3 id="entry-name-and-manifest-name">
  Nama entri dan nama manifest
</h3>

Plugin marketplace memiliki dua nama, dan keduanya dapat berbeda:

* **Nama entri dalam `marketplace.json`**: kunci install dan enable. Ini adalah apa yang Anda tulis dalam `enabledPlugins`, apa yang dinamai direktori cache, dan apa yang ditampilkan `claude plugin list`
* **`name` dalam manifest**: apa yang komponen plugin di-namespace di bawahnya, dan apa yang dibandingkan [konflik nama](#name-conflicts)

<h3 id="plugins-shared-through-a-repository">
  Plugin yang dibagikan melalui repositori
</h3>

Untuk membagikan plugin melalui repositori, daftarkan di bawah `enabledPlugins` dalam `.claude/settings.json` atau tempatkan di bawah `.claude/skills/`. Claude Code tidak memindai direktori `.claude/plugins/` proyek.

Sesi cloud tidak menambahkan marketplace yang daftar repositori di bawah [`extraKnownMarketplaces`](/docs/id/settings-reference#extraknownmarketplaces), karena itu memerlukan dialog kepercayaan workspace, yang tidak pernah ditampilkan sesi cloud.

Plugin direktori skills dengan cakupan proyek dimuat hanya dari `.claude/skills/` dari [direktori kerja utama](/docs/id/permissions#working-directories) sesi, dan hanya setelah Anda menerima [dialog kepercayaan workspace](/docs/id/permissions#what-runs-before-you-trust-a-folder) untuk folder itu. Plugin tidak [mencari direktori induk hingga akar repositori](/docs/id/skills#discovery-from-parent-and-nested-directories) seperti yang dilakukan skill dan perintah biasa. Jika Anda meluncurkan dari subdirektori, plugin di akar repositori tidak dimuat. Luncurkan dari akar repositori sebagai gantinya, atau [pindahkan sesi ke sana dengan `/cd`](/docs/id/permissions#move-the-session-to-another-directory) pada v2.1.246 atau lebih baru.

Plugin dengan cakupan proyek diperiksa ke dalam repositori dan mencapai setiap kolaborator yang mengklonnya. Karena konten itu berasal dari repositori daripada dari Anda, plugin dimuat hanya setelah pemeriksaan kepercayaan yang sama yang berlaku untuk aturan izin proyek dalam `.claude/settings.json`. Mempercayai folder induk atau menjalankan dengan `-p` tidak cukup. Komponen yang menjalankan kode dibatasi lebih lanjut:

* Server MCP yang dideklarasikannya melalui [persetujuan per-server yang sama](/docs/id/mcp) seperti `.mcp.json` proyek
* Server MCP yang dideklarasikannya sebagai [bundel MCP](/docs/id/plugins/manifest-reference#mcpservers), file `.mcpb` atau `.dxt`, atau dari file di luar direktori plugin dilewati. Deklarasikan secara inline atau dalam `.mcp.json` di dalam direktori plugin
* [Monitor latar belakang](/docs/id/plugins/components#monitors) tidak dimuat

Plugin dengan cakupan personal tidak memiliki batasan ini.

Untuk cara menulis plugin `--plugin-dir` dan direktori skills, lihat [Buat plugin](/docs/id/plugins/create).

<h3 id="synced-plugins">
  Plugin yang disinkronkan dari claude.ai
</h3>

Plugin yang Anda aktifkan untuk akun claude.ai Anda juga dimuat dalam Claude Code, bersama dengan plugin yang Anda instal dari marketplace. Ini termasuk plugin yang organisasi Anda aktifkan untuk anggotanya. Setiap plugin ini dimuat sebagai `<name>@synced`, tanpa marketplace dan tanpa [catatan install](#check-which-stage-a-plugin-reached).

Dalam sesi terminal, skill, agent, hooks, server MCP, dan server LSP plugin yang disinkronkan semuanya dimuat, dengan kepercayaan yang sama seperti plugin marketplace yang Anda instal.

Untuk komponen yang dimuat Cowork, lihat [Plugin di claude.ai dan di Cowork](https://claude.com/docs/plugins/overview) di claude.com.

Plugin yang disinkronkan dimuat dalam sesi Cowork dan dalam sesi terminal di mana Anda masuk dengan akun claude.ai Anda:

* **[Cowork](https://claude.com/product/cowork)**: Claude Code mengunduhnya ke lingkungan sesi itu sendiri ketika sesi dimulai
* **Sesi terminal**: setiap kali Anda memulai Claude Code, plugin disinkronkan sekali di latar belakang, mengunduh plugin baru dan yang diperbarui serta menghapus yang Anda atau organisasi Anda matikan. Sinkronisasi dalam sesi terminal memerlukan Claude Code v2.1.273 atau lebih baru

<h4 id="sync-timing-in-terminal-sessions">
  Waktu sinkronisasi dalam sesi terminal
</h4>

Karena sinkronisasi terminal berjalan di latar belakang, dapat selesai setelah sesi Anda telah dimulai. Ketika menambah, memperbarui, atau menghapus plugin yang disinkronkan dalam sesi interaktif, Anda melihat `Plugins changed. Run /reload-plugins to activate.` Jalankan `/reload-plugins` untuk memuat perubahan dalam sesi itu, atau biarkan untuk lain kali Anda memulai Claude Code.

Jika Anda mengaktifkan plugin di claude.ai saat sesi sedang berjalan, plugin mengunduh lain kali Anda memulai Claude Code.

<h4 id="sign-in-requirements-for-terminal-sync">
  Persyaratan masuk untuk sinkronisasi terminal
</h4>

Di terminal Anda, plugin disinkronkan hanya dalam sesi di mana Anda masuk dengan akun claude.ai Anda.

Jika Anda masuk pada versi Claude Code yang lebih awal, masuk itu tidak mencakup plugin sampai Claude Code memperbarui di latar belakang. Untuk mendapatkan akses lebih cepat, jalankan `/login` lagi. Sinkronisasi plugin kemudian dimulai lain kali Anda memulai Claude Code.

<h4 id="control-which-synced-plugins-load">
  Kontrol plugin yang disinkronkan mana yang dimuat
</h4>

Anda dapat menonaktifkan plugin yang disinkronkan satu per satu, kecuali plugin yang organisasi Anda wajibkan, atau menonaktifkan setiap plugin yang disinkronkan di mesin:

* **Satu plugin**: `claude plugin disable <name>@synced` dalam shell Anda dan tab **Installed** `/plugin` dalam sesi keduanya menyimpan `"<name>@synced": false` dalam [`enabledPlugins`](/docs/id/settings-reference#enabledplugins) tingkat pengguna Anda. Untuk menjaga plugin keluar dari proyek di setiap lingkungan, atur kunci yang sama dalam `.claude/settings.json` yang berkomitmen proyek
* **Setiap plugin yang disinkronkan di mesin**: atur [`syncClaudeAiPlugins`](/docs/id/settings-reference#syncclaudeaiplugins) ke `false` dalam pengaturan pengguna Anda, atau organisasi Anda mengaturnya dalam [pengaturan terkelola](/docs/id/managed-settings). Claude Code berhenti mengunduh, dan lain kali Anda memulainya, plugin yang sudah disinkronkan dipindahkan ke `~/.claude/plugins/.trash/` dan tidak lagi dimuat. Jika organisasi Anda menonaktifkan Skills di claude.ai, plugin juga berhenti disinkronkan
* **Plugin yang organisasi Anda wajibkan**: plugin yang organisasi Anda tandai sebagai wajib di claude.ai dimuat bahkan jika Anda menonaktifkannya sebelumnya. `claude plugin disable` menolaknya dengan `Plugin "<name>@synced" is required by your organization and can't be disabled here. Contact your admin to change it.`, dan `claude plugin list` menandainya `required by your org`

Untuk menghapus plugin di claude.ai, lihat [Kelola plugin yang terinstal](/docs/id/plugins/install#manage-installed-plugins).

<h2 id="find-where-a-plugin-is-enabled">
  Temukan di mana plugin diaktifkan
</h2>

Anda dapat mengatur entri `enabledPlugins` dalam salah satu dari enam sumber. Tabel mencantumnya dari preseden terendah ke tertinggi, dan siapa yang masing-masing berlaku. Untuk file pengaturan itu sendiri, lihat [File pengaturan dan siapa yang mereka pengaruhi](/docs/id/settings#where-settings-live).

| Sumber      | Di mana Anda mengaturnya                                                                                         | Menjangkau                                                                                                              |
| :---------- | :--------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------- |
| `--add-dir` | `.claude/settings.json` atau `.claude/settings.local.json` dalam direktori yang Anda teruskan dengan `--add-dir` | Sesi ini saja. Hanya nilai `true` yang memiliki efek, dan setiap sumber lain menimpanya                                 |
| `user`      | `~/.claude/settings.json`                                                                                        | Anda, dalam setiap proyek                                                                                               |
| `project`   | `.claude/settings.json`                                                                                          | Semua orang yang mengklona repositori                                                                                   |
| `local`     | `.claude/settings.local.json`                                                                                    | Anda, dalam repositori ini saja                                                                                         |
| `flag`      | Nilai `--settings` yang Anda teruskan saat peluncuran                                                            | Sesi ini saja                                                                                                           |
| `managed`   | [Pengaturan terkelola](/docs/id/managed-settings)                                                                     | Setiap pengguna yang dicakup kebijakan. `true` force-enable dan `false` blok, dan tidak ada sumber lain yang menimpanya |

Sumber-sumber ini menggabungkan kunci demi kunci. Untuk setiap id plugin, nilai yang berlaku adalah nilai dari sumber preseden tertinggi yang menyebutkan id. Sumber yang tidak menyebutkan id membiarkan nilai dari sumber preseden lebih rendah tetap berlaku.

<h3 id="disabled-in-user-settings-but-still-loads">
  Dinonaktifkan dalam pengaturan pengguna tetapi masih dimuat
</h3>

Jika Anda mengatur plugin ke `false` dalam `~/.claude/settings.json` dan masih dimuat, `true` dalam sumber preseden lebih tinggi menimpanya. Baris plugin dalam `claude plugin list` dan dalam `/plugin` menampilkan `Disabled in ~/.claude/settings.json but still loads — project settings enable it, which overrides your user setting`. Pesan menyebutkan sumber yang menimpa Anda: `project`, `project, gitignored` untuk `.claude/settings.local.json`, `cli flag`, atau `managed`.

Untuk keluar dari plugin yang diaktifkan proyek di mesin Anda, atur id ke `false` dalam `.claude/settings.local.json`, yang memiliki preseden lebih tinggi daripada file proyek.

<h3 id="enabled-in-project-settings-but-not-installed">
  Diaktifkan dalam pengaturan proyek tetapi tidak terinstal
</h3>

Ketika satu-satunya `true` plugin dalam `.claude/settings.json` proyek, Claude Code tidak mengambilnya ke mesin di mana plugin tidak terinstal, kecuali entri marketplace-nya memiliki [sumber jalur relatif](/docs/id/plugins/marketplace-reference#plugin-sources) atau [direktori seed](/docs/id/plugins/org#seed-containers-and-ci) sudah menyimpannya. Sebagai gantinya, tab **Errors** `/plugin` menampilkan `Plugin "<name>" is enabled in project settings but isn't installed here`.

Plugin jalur relatif tidak memerlukan catatan install karena dimuat dari marketplace itu sendiri.

Claude Code mengambil plugin dengan sumber eksternal hanya ketika salah satu sumber ini mengaturnya ke `true`:

* Pengaturan pengguna Anda
* `.claude/settings.local.json` yang tidak dilacak git
* Bendera `--settings`
* Pengaturan terkelola

<h2 id="find-plugins-on-disk">
  Temukan plugin di disk
</h2>

Claude Code menyimpan file plugin dan catatan status di bawah satu root plugin, yaitu `~/.claude/plugins` kecuali Anda menetapkan [`CLAUDE_CODE_PLUGIN_CACHE_DIR`](/docs/id/env-vars). Setiap path dalam tabel adalah relatif terhadap root tersebut.

| Path                                                   | Apa yang disimpannya                                                                                                                                                                                                                                                                                                                                                                                                                                |
| :----------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `cache/<marketplace>/<plugin>/<version>/`              | Satu direktori per versi plugin marketplace yang terinstal. `<plugin>` adalah nama entri marketplace dan `<version>` adalah [versi yang diselesaikan](#versions-and-updates). `${CLAUDE_PLUGIN_ROOT}` menunjuk ke direktori ini                                                                                                                                                                                                                     |
| `data/<plugin-id>/`                                    | Direktori persisten plugin, diekspos sebagai `${CLAUDE_PLUGIN_DATA}`. Untuk cara `<plugin-id>` dibentuk, lihat [Path variables and persistent data](/docs/id/plugins/components#path-variables-and-persistent-data). Claude Code membuatnya ketika komponen plugin pertama kali menggunakannya dan menyimpannya di seluruh pembaruan. Claude Code menghapusnya ketika Anda mencopot plugin dari scope terakhirnya, kecuali Anda melewatkan `--keep-data` |
| `marketplaces/<name>/`                                 | Klon atau unduhan marketplace yang ditambahkan dari GitHub, host Git lain, atau URL. Marketplace yang ditambahkan dari sumber `file` atau `directory` lokal tidak memiliki salinan di sini, dan `installLocation` dalam `known_marketplaces.json` adalah path yang Anda berikan                                                                                                                                                                     |
| `synced/`                                              | Plugin yang Claude Code [sinkronkan dari akun claude.ai Anda](#synced-plugins)                                                                                                                                                                                                                                                                                                                                                                      |
| `.trash/`                                              | Plugin yang sinkronisasi claude.ai hapus, seperti setelah Anda mematikannya di claude.ai atau berhenti menyinkronkan                                                                                                                                                                                                                                                                                                                                |
| `installed_plugins.json` dan `known_marketplaces.json` | Catatan tentang apa yang telah Claude Code instal dan marketplace mana yang telah diambilnya, dijelaskan di bawah [Check which stage a plugin reached](#check-which-stage-a-plugin-reached). [Marketplace yang dihosting di claude.ai](/docs/id/plugins/install#add-from-claude-ai) dicatat dalam `known_marketplaces_claudeai.json` sebagai gantinya                                                                                                    |
| `flagged-plugins.json`                                 | Plugin yang Claude Code copot karena marketplace mereka menghapus daftarnya. Mereka muncul di bagian **Flagged** dari `/plugin`; lihat [Host a marketplace](/docs/id/plugins/host-marketplace)                                                                                                                                                                                                                                                           |

Karena `${CLAUDE_PLUGIN_ROOT}` menunjuk ke direktori versi, path root plugin berubah dengan setiap versi. Simpan file tahan lama plugin dalam `${CLAUDE_PLUGIN_DATA}` sebagai gantinya.

<h3 id="in-place-and-copied-plugins">
  Plugin in-place dan yang disalin
</h3>

Claude Code memuat beberapa plugin di tempat dari mana Anda menyimpannya dan menyalin sisanya ke dalam cache, sesuai dengan asal mereka:

* **Plugin `--plugin-dir` dan skills-directory**: direktori dimuat di tempat dan tidak pernah disalin. Arsip `--plugin-url` atau `.zip` `--plugin-dir` diekstrak ke direktori temp sesi terlebih dahulu
* **Plugin dengan path relatif di marketplace yang Anda tambahkan dari direktori lokal**: plugin dimuat di tempat dari pathnya di dalam folder marketplace. Edit Anda ke direktori sumber berlaku pada awal sesi berikutnya atau `/reload-plugins`, dan Anda tidak perlu meningkatkan versi. Proses hook plugin dan server MCP dan LSP menerima `CLAUDE_PLUGIN_ROOT` yang menunjuk ke direktori sumber. Untuk dependensi paket Node.js-nya, lihat [When the dependency install runs](#when-the-dependency-install-runs)
* **Plugin sumber `command` dalam [link mode](/docs/id/plugins/marketplace-reference#command-plugin-source)**: direktori yang dicetak perintah dimuat di tempat, melalui link di entri cache
* **Setiap plugin marketplace lainnya**: Claude Code menyalin plugin ke dalam `cache/<marketplace>/<plugin>/<version>/` saat instalasi dan memuat salinan tersebut. File di luar direktori plugin tidak disalin, jadi ketika skrip di dalam plugin yang disalin membaca path di atas root plugin, seperti `../shared`, itu tidak menemukannya

<h3 id="paths-that-escape-the-plugin-directory">
  Path yang keluar dari direktori plugin
</h3>

Baik plugin dimuat di tempat atau dari salinan cache, Claude Code tidak membiarkannya mendeklarasikan komponen di luar direktorinya sendiri. Claude Code menolak path komponen yang diselesaikan di luar root plugin, baik path dideklarasikan dalam `plugin.json` atau dalam entri marketplace:

* **Path yang menunjuk di luar plugin seperti yang ditulis**, seperti `../shared-utils`
* **Symlink yang mengarah di luar plugin**, selain [link antar plugin dalam satu marketplace](/docs/id/plugins/host-marketplace#share-files-within-a-marketplace-with-symlinks)
* **Di macOS dan Linux, path yang berisi backslash di mana pun di dalamnya**, bahkan ketika path tetap berada di dalam plugin. Komponen yang dideklarasikan dengan path backslash oleh karena itu hanya dimuat di Windows, jadi tulis path komponen dengan forward slash, seperti `./commands/deploy.md`

Path yang ditolak muncul sebagai kesalahan [`path escapes plugin directory`](/docs/id/errors#path-escapes-plugin-directory), dan plugin dimuat tanpa komponen tersebut.

<h3 id="cleanup-of-previous-versions">
  Pembersihan versi sebelumnya
</h3>

Ketika Anda memperbarui atau mencopot plugin, Claude Code menulis penanda `.orphaned_at` ke dalam direktori versi sebelumnya. Claude Code menghapus direktori tersebut dalam pembersihan latar belakang 14 hari kemudian, jadi sesi yang sudah memuat versi lama terus berjalan.

Sweep hanya berjalan saat `installed_plugins.json` mencatat setidaknya satu instalasi. Setelah Anda mencopot plugin terakhir Anda, direktori orphaned tetap ada sampai Anda menginstal yang lain.

<h3 id="node-js-package-dependencies">
  Dependensi paket Node.js
</h3>

Ketika Claude Code menyalin plugin ke dalam cache, Claude Code juga menginstal dependensi paket Node.js plugin di sana, sehingga hook dan server MCP plugin dapat memuatnya.

Bagian ini mencakup paket npm dan Bun yang dideklarasikan plugin dalam `package.json`-nya sendiri. Untuk plugin yang bergantung pada plugin lain, lihat [plugin dependency versions](/docs/id/plugins/dependencies).

<h4 id="when-the-dependency-install-runs">
  Kapan dependency install berjalan
</h4>

Claude Code menjalankan instalasi di dalam direktori versi yang disalin setiap kali membuat satu:

* Ketika Anda menginstal plugin
* Ketika Claude Code memperbarui plugin ke versi baru
* Saat awal sesi ketika plugin yang diaktifkan belum di-cache, seperti di mesin baru

Untuk plugin dengan path relatif [dimuat di tempat](#in-place-and-copied-plugins) dari marketplace direktori lokal, Claude Code tidak menginstal dependensi ke dalam direktori sumber. Instal mereka di sana sendiri, atau dari hook ke [`${CLAUDE_PLUGIN_DATA}`](/docs/id/plugins/components#path-variables-and-persistent-data).

Instalasi hanya berjalan ketika direktori root plugin berisi baik `package.json` maupun lockfile yang didukung. Lockfile menentukan perintah mana yang Claude Code jalankan:

| Lockfile                                       | Perintah                                         |
| :--------------------------------------------- | :----------------------------------------------- |
| `bun.lock` atau `bun.lockb`                    | `bun install --frozen-lockfile --ignore-scripts` |
| `npm-shrinkwrap.json` atau `package-lock.json` | `npm ci --ignore-scripts`                        |

Jika plugin berisi lebih dari satu lockfile ini, Claude Code menggunakan kecocokan pertama, memeriksa dalam urutan: `bun.lock`, `bun.lockb`, `npm-shrinkwrap.json`, `package-lock.json`.

Claude Code melewati instalasi untuk lockfile Yarn dan pnpm dan untuk `bunfig.toml` di samping lockfile Bun:

* Jika plugin Anda hanya memiliki `yarn.lock` atau `pnpm-lock.yaml`, gantikan dengan lockfile npm
* Jika `bunfig.toml` berada di direktori yang sama dengan lockfile Bun, hapus `bunfig.toml`, atau gantikan lockfile Bun dengan lockfile npm

Sertakan lockfile npm untuk menjangkau pengguna paling banyak. Claude Code menjalankan package manager lockfile yang cocok dari PATH pengguna dan tidak mencoba lockfile lain sebagai gantinya jika package manager tersebut hilang.

Untuk plugin yang didistribusikan melalui sumber npm, gunakan `npm-shrinkwrap.json`, karena npm mengecualikan `package-lock.json` dari paket yang dipublikasikan.

<h4 id="limits-on-the-dependency-install">
  Batas pada dependency install
</h4>

Claude Code membatasi dependency install ini sehingga tidak ada kode dari plugin atau paketnya yang dieksekusi selama itu, dan membatasi berapa lama itu dapat berjalan:

* **Frozen resolution**: Bun dan npm menginstal dengan tepat apa yang lockfile pin, dan gagal daripada menyelesaikan ulang versi ketika `package.json` dan lockfile tidak setuju
* **No lifecycle scripts**: `--ignore-scripts` menjaga skrip `preinstall`, `install`, dan `postinstall` agar tidak berjalan, sehingga dependensi yang membangun modul native dalam skrip tersebut diunduh tetapi tidak dikompilasi selama instalasi ini
* **60-second timeout**: Claude Code menghentikan instalasi yang berjalan lebih lama dan memperlakukannya sebagai gagal

Claude Code mengambil plugin sumber npm sebelum dependency install ini, dan tidak ada skrip instalasi paket sendiri yang berjalan selama pengambilan. Lihat [npm plugin source](/docs/id/plugins/marketplace-reference#npm-plugin-source).

Anda tidak dapat mematikan instalasi otomatis. Tidak ada pengaturan atau variabel lingkungan yang menonaktifkannya.

Di jaringan terbatas, lihat [network access requirements](/docs/id/network-config#network-access-requirements) untuk host yang diizinkan.

<h4 id="when-the-dependency-install-fails-or-is-skipped">
  Ketika dependency install gagal atau dilewati
</h4>

Instalasi yang gagal atau dilewati tidak pernah memblokir plugin, dan setiap kasus meninggalkan tanda yang berbeda:

* Instalasi yang gagal, atau yang dilewati karena lockfile Yarn atau pnpm atau `bunfig.toml`, muncul sebagai peringatan dalam output `claude --debug`
* Plugin dengan `package.json` dan tidak ada lockfile dilewati tanpa entri log
* Instalasi yang habis waktu dapat meninggalkan pohon `node_modules` parsial dalam salinan cache

Ketika instalasi otomatis tidak dapat menyediakan dependensi, instal dari hook ke [persistent data directory](/docs/id/plugins/components#path-variables-and-persistent-data). Itu termasuk paket yang memerlukan skrip lifecycle mereka untuk membangun, dependensi Python, dan plugin yang dikunci dengan Yarn atau pnpm.

<h2 id="versions-and-updates">
  Versi dan pembaruan
</h2>

Jika penulis plugin mendorong komit baru dan `claude plugin update` mencetak `<name> is already at the latest version (<version>).`, versi yang dihitung Claude Code untuk plugin tidak berubah, jadi tidak ada yang berubah di disk.

Claude Code menghitung versi untuk setiap plugin yang diinstal, dan versi itu adalah cara mendeteksi pembaruan. `claude plugin update` dan auto-update latar belakang menghitung versi lagi dan melewati plugin ketika cocok dengan apa yang dicatat `installed_plugins.json`.

Versi juga menamai direktori cache plugin.

Manifest yang menyematkan `"version"` adalah salah satu cara versi yang dihitung tetap sama di seluruh komit. Lihat [Bagaimana Claude Code menghitung versi](#how-claude-code-computes-the-version) untuk urutan resolusi.

Plugin [dimuat di tempat](#in-place-and-copied-plugins) dari marketplace direktori lokal memuat file sumber saat ini di setiap startup sesi, apa pun string versinya. Untuk plugin dari [marketplace yang dihosting di claude.ai](/docs/id/plugins/install#add-from-claude-ai), versi yang dicatat claude.ai untuk plugin adalah versinya, dan `version` manifest tidak dibaca.

<h3 id="how-claude-code-computes-the-version">
  Bagaimana Claude Code menghitung versi
</h3>

Untuk marketplace yang Anda tambahkan berdasarkan sumber, Claude Code memilih aturan berdasarkan tipe `source` entri marketplace plugin. [Referensi marketplace](/docs/id/plugins/marketplace-reference#plugin-sources) mencantumkan tipe sumber. Untuk setiap tipe sumber dalam daftar itu kecuali `command`:

1. Bidang `version` dalam manifest plugin datang terlebih dahulu
2. Kemudian bidang `version` dalam entri marketplace plugin
3. Ketika tidak ada yang ditetapkan, versi berasal dari tipe sumber:

| Tipe sumber                                                                          | Versi ketika tidak ada bidang `version` yang ditetapkan                                                                                   |
| :----------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------- |
| `github`, `url`, atau `git-subdir`                                                   | SHA komit dari sumber, diperpendek menjadi 12 karakter. Versi `git-subdir` juga membawa hash dari jalur subdirektori                      |
| `archive`                                                                            | Digest SHA-256, diperpendek menjadi 12 karakter: pin `sha256` dalam entri marketplace, atau digest file yang diunduh ketika tidak ada pin |
| Jalur relatif di dalam marketplace yang dihosting Git                                | SHA komit dari direktori yang diinstal                                                                                                    |
| Direktori lokal, ketika direktori plugin maupun marketplace-nya bukan repositori git | `unknown`                                                                                                                                 |
| `npm`                                                                                | `unknown`                                                                                                                                 |

Claude Code tidak mengambil versi dari repositori yang menutup jalur install, seperti `~/.claude` yang dikelola git.

Untuk sumber `command`, Claude Code selalu menurunkan versi dari apa yang dihasilkan perintah: hash 12 karakter sendiri, atau `<manifest version>-<hash>` ketika manifest menetapkan satu. Entri marketplace `version` diabaikan untuk sumber command. Untuk apa yang dicakup hash, lihat [Mode copy dan mode link](/docs/id/plugins/marketplace-reference#copy-mode-and-link-mode).

Karena manifest datang terlebih dahulu, manifest yang menyematkan `"version": "1.0.0"` menjaga setiap pengguna pada salinan cache sampai penulis mengubah string, berapa pun komit yang mereka dorong. Untuk membiarkan pengguna melacak komit sebagai gantinya, tinggalkan `version` dari manifest dan entri. [Host marketplace](/docs/id/plugins/host-marketplace) mencakup pilihan mana yang sesuai dengan setup rilis mana.

<h3 id="when-claude-code-refreshes-a-marketplace-before-an-install">
  Kapan Claude Code menyegarkan marketplace sebelum install
</h3>

Ketika Anda menginstal plugin, Claude Code mencarinya dalam salinan lokal katalog marketplace. Anda dapat menjalankan `/plugin install` dalam sesi atau `claude plugin install` dalam shell Anda, dan menamai plugin dengan atau tanpa marketplace-nya. Tabel menunjukkan kombinasi mana dari itu yang menyegarkan salinan lokal.

| Nama plugin        | Perintah                                       | Apa yang Claude Code segarkan                                                        |
| :----------------- | :--------------------------------------------- | :----------------------------------------------------------------------------------- |
| `name@marketplace` | `/plugin install` atau `claude plugin install` | Marketplace yang dinamai, sebelum pencarian                                          |
| `name` saja        | `/plugin install`                              | Hanya marketplace yang memiliki auto-update aktif, dan hanya setelah pencarian gagal |
| `name` saja        | `claude plugin install`                        | Tidak ada. Plugin membaca katalog cache tanpa menyegarkan                            |

Penyegaran sebelum install `name@marketplace` tidak bergantung pada pengaturan auto-update marketplace atau pada `DISABLE_AUTOUPDATER`.

Ketika penyegaran gagal, install berlanjut dari katalog cache dan `claude plugin install` melaporkan `marketplace not refreshed`.

Claude Code melewati penyegaran sebelum install `name@marketplace` ketika:

* Marketplace ditambahkan dari sumber `file` atau `directory` lokal, atau didefinisikan inline dalam pengaturan dengan [sumber `settings`](/docs/id/settings-reference#extraknownmarketplaces)
* [Direktori seed](/docs/id/env-vars) menyediakan marketplace
* Claude Code menyegarkan marketplace dalam 30 detik terakhir
* Anda mengatur `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`
* [Pengaturan terkelola](/docs/id/plugins/org#restrict-what-users-can-install) memblokir marketplace, dalam hal ini Claude Code juga menolak install

<h3 id="when-auto-update-runs">
  Kapan auto-update berjalan
</h3>

Dalam sesi interaktif, setelah Anda mengirim pesan pertama, Claude Code menunggu penundaan acak hingga sepuluh menit. Plugin kemudian menyegarkan setiap marketplace dengan auto-update aktif dan memperbarui plugin yang diinstal dari mereka di disk.

Sesi yang berjalan menyimpan versi yang dimuat, dan Anda melihat `Plugin updated: <name> · Run /reload-plugins to apply`. Apakah Anda memuat ulang atau tidak, versi baru dimuat pada peluncuran berikutnya Anda.

<h4 id="which-marketplaces-and-plugins-auto-update">
  Marketplace dan plugin mana yang auto-update
</h4>

Apakah marketplace auto-update mengikuti yang pertama dari ini yang ditetapkan:

1. **`autoUpdate` pada entri `extraKnownMarketplaces`-nya** dalam file pengaturan
2. **`autoUpdate` pada entri `known_marketplaces.json`-nya**, yang toggle **Enable auto-update** di bawah `/plugin` **Marketplaces** tulis. Ketika file pengaturan juga mendeklarasikan marketplace di bawah `extraKnownMarketplaces`, toggle juga menulis `autoUpdate` ke entri pengaturan itu
3. **Default**: aktif untuk marketplace resmi Anthropic seperti `claude-plugins-official`, nonaktif untuk `knowledge-work-plugins` dan `first-party-plugins`, aktif untuk [marketplace yang ditambahkan dari claude.ai](/docs/id/plugins/install#add-from-claude-ai), dan nonaktif untuk setiap marketplace lainnya

Jika Anda mengatur `DISABLE_UPDATES=1`, `DISABLE_AUTOUPDATER=1`, atau `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC=1`, seluruh pass nonaktif dan toggle **Enable auto-update** disembunyikan, kecuali Anda juga mengatur `FORCE_AUTOUPDATE_PLUGINS=1`. [Referensi variabel lingkungan](/docs/id/env-vars) mencakup efek lebih luas setiap variabel.

Auto-update juga melewati plugin yang entri marketplace-nya mendeklarasikan `headersHelper`. [Install dan pembaruan yang menolak perintah alih-alih bertanya](/docs/id/plugins/host-marketplace#installs-and-updates-that-refuse-the-command-instead-of-asking) menjelaskan kapan plugin seperti itu muncul di tab **Errors** `/plugin` dan cara Anda memperbarui dari sana.

Ketika plugin yang disalin diperbarui mid-session, perintah hook, monitor, server MCP, dan server LSP terus menggunakan jalur versi sebelumnya. Jalankan `/reload-plugins` untuk beralih hook, server MCP, dan server LSP ke jalur baru. Monitor memerlukan restart sesi.

<h3 id="when-a-command-source-re-runs">
  Kapan sumber command dijalankan kembali
</h3>

Plugin dengan sumber `command` tidak menunggu [pass auto-update](#when-auto-update-runs). Direktori yang dicetak mencerminkan status alat pada waktu perintah berjalan, jadi Claude Code menjalankan [perintah yang Anda terima](/docs/id/plugins/host-marketplace#change-the-command-of-a-command-source) lagi pada waktu-waktu ini:

* Setiap kali Anda menginstal atau memperbarui plugin
* Sekali per sesi untuk setiap plugin yang bersumber command yang diaktifkan, di latar belakang, segera setelah sesi dimulai. Jalankan ini tidak bergantung pada pengaturan auto-update marketplace atau pada `DISABLE_AUTOUPDATER`
* Saat startup atau di `/reload-plugins`, ketika versi yang diinstal plugin yang diaktifkan hilang dari cache plugin

Claude Code melewati dua jalankan latar belakang ketika Anda mengatur [`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`](/docs/id/env-vars). Install dan pembaruan eksplisit masih menjalankan perintah dengan variabel itu ditetapkan.

Ketika output hash perintah telah berubah, Claude Code menginstal hasil sebagai versi baru dan memuat ulang dalam sesi interaktif yang berjalan, beralih [komponen yang sama yang `/reload-plugins` alihkan](/docs/id/plugins/cli-reference#reload-plugins). Anda melihat notifikasi bahwa plugin dimuat ulang.

Jika memuat ulang di tempat akan membatalkan cache prompt sesi, Claude Code sebagai gantinya memberi tahu Anda untuk menjalankan `/reload-plugins`, yang [memperingatkan tentang biaya cache dan berlaku ketika dijalankan kembali dengan `--force`](/docs/id/prompt-caching#enabling-or-disabling-a-plugin).

<h2 id="name-conflicts">
  Konflik nama
</h2>

Ketika plugin yang diaktifkan dari asal berbeda berbagi nama manifest, urutan ini menentukan mana yang dimuat, dari preseden tertinggi ke terendah:

1. Plugin yang id-nya muncul dalam pengaturan terkelola `enabledPlugins`, sebagai `true` atau `false`. Salinan `--plugin-dir` yang nama manifest-nya cocok dengan bagian nama id tidak dimuat, dan Anda melihat `--plugin-dir copy of "<name>" ignored: plugin is locked by managed settings`
2. Plugin `--plugin-dir`, `--plugin-url`, atau `CLAUDE_CODE_PLUGIN_DIRS` yang diaktifkan. Plugin menggantikan plugin marketplace atau direktori skills yang sama-nama yang terinstal:
   * **Plugin marketplace yang terinstal**: digantikan diam-diam. `claude plugin list` masih menampilkan baris marketplace sebagai diaktifkan, karena baris itu mencerminkan pengaturan Anda. Hanya log yang ditulis Claude Code di bawah `~/.claude/debug/` ketika Anda memulai dengan `--debug` yang mencatat `Plugin "<name>" from --plugin-dir overrides installed version`
   * **Plugin direktori skills**: digantikan dengan baris tab **Errors** `/plugin` yang berbunyi `Not loaded — the name "<name>" is already taken by a session-only plugin (--plugin-dir / --plugin-url), which takes precedence`
3. Plugin marketplace yang terinstal. Plugin direktori skills dengan nama yang sama mendapat baris `Not loaded` yang sama, menamai plugin yang terinstal
4. Plugin direktori skills. Antara dua ini, salinan di bawah `~/.claude/skills/` dimuat dan salinan `.claude/skills/` proyek dijatuhkan, dengan baris yang mengatakan jalur mana yang menaunginya
5. Plugin [disinkronkan dari claude.ai](#synced-plugins). Ketika plugin yang diaktifkan dari asal lain cocok dengan namanya, Claude Code memuat plugin itu dan melaporkan salinan yang disinkronkan sebagai tidak dimuat. Untuk menggunakan salinan claude.ai sebagai gantinya, nonaktifkan salinan Anda sendiri

Karena urutan membandingkan nama manifest, plugin `--plugin-dir` bernama `hello-plugin` menggantikan `hello@example-marketplace` ketika manifest plugin itu juga mengatakan `"name": "hello-plugin"`.

<h3 id="keep-a-session-only-plugin-from-loading">
  Jaga plugin session-only agar tidak dimuat
</h3>

Untuk menjaga plugin `--plugin-dir` agar tidak menaungi apa pun, atau untuk menonaktifkannya ketika proses induk meneruskan bendera untuk Anda, atur id-nya ke `false` dalam file pengaturan apa pun. Untuk plugin yang nama manifest-nya adalah `hello-plugin`, entri adalah `"enabledPlugins": {"hello-plugin@inline": false}`. Plugin session-only yang dinonaktifkan tidak menaungi, jadi salinan marketplace atau direktori skills dimuat sebagai gantinya.

<h2 id="next-steps">
  Langkah berikutnya
</h2>

* [Install dan kelola plugin](/docs/id/plugins/install): langkah install, enable, disable, dan update itu sendiri
* [Troubleshoot plugin](/docs/id/plugins/troubleshooting): pesan kesalahan berdasarkan tahap yang menghasilkannya
* [Referensi perintah plugin](/docs/id/plugins/cli-reference): bendera dan perintah yang dinamai di halaman ini
* [Kelola plugin untuk organisasi Anda](/docs/id/plugins/org): pengaturan terkelola yang force-enable atau blok plugin
