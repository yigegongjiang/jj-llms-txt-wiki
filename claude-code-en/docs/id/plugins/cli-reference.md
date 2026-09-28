> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Referensi perintah plugin

> Referensi lengkap untuk perintah shell plugin claude, /plugin dan /reload-plugins dalam sesi, dan flag yang memuat plugin untuk satu sesi.

Anda menjalankan perintah plugin baik sebagai `claude plugin` dari shell atau skrip Anda, atau sebagai `/plugin` dan `/reload-plugins` di dalam sesi Claude Code. Referensi ini memberikan flag, default, output, dan kode keluar setiap perintah, bersama dengan dua flag yang memuat plugin untuk satu sesi.

Jalankan `claude plugin --help` pada build Anda untuk mengonfirmasi subperintah mana yang dimiliki versi Anda.

<Note>
  Kasus-kasus ini tercakup di halaman lain:

  * **Instal dan kelola langkah, dan di mana `/plugin` berjalan**: lihat [Instal dan kelola plugin](/docs/id/plugins/install)
  * **Apa yang diubah perintah di disk dan cakupan mana yang diutamakan**: lihat [Referensi pemuatan plugin](/docs/id/plugins/loading)
  * **Apa arti pesan kesalahan**: lihat [Troubleshoot plugin](/docs/id/plugins/troubleshooting)
</Note>

<h2 id="claude-plugin-commands">
  Perintah claude plugin
</h2>

Jalankan `claude plugin <subcommand>` dari shell atau skrip Anda, di luar sesi Claude Code. Subperintah ini menginstal dan mengelola plugin tanpa membuka panel [`/plugin`](#plugin-in-a-session).

`claude plugins` adalah alias untuk `claude plugin`.

Setiap subperintah berbagi kode keluar, argumen plugin, dan nilai cakupan ini:

* **Kode keluar**: `0` pada kesuksesan dan `1` pada kegagalan. `validate` menambahkan keluar `2` untuk kesalahan yang tidak terduga, dan `eval` menambahkan kode yang tercantum di [bagiannya](#plugin-eval).
* **Argumen plugin**: argumen `<plugin>` adalah plugin `name` atau `name@marketplace`. Ketika dua marketplace menawarkan nama yang sama, gunakan bentuk yang memenuhi syarat.
* **Cakupan**: `--scope` mengambil `user`, `project`, atau `local`, dan menamai file pengaturan yang ditulis perintah. `update` juga mengambil `managed`.

<h3 id="plugin-init">
  plugin init
</h3>

Rangka kerja plugin baru di `~/.claude/skills/<name>/`. Plugin ini dimuat dalam sesi berikutnya Anda sebagai `<name>@skills-dir` tanpa langkah instal.

`new` adalah alias untuk `init`.

Untuk alur kerja buat, uji, dan edit yang dimulai dengan perintah ini, lihat [Buat plugin](/docs/id/plugins/create).

```bash theme={null}
claude plugin init <name> [options]
```

`<name>` menjadi nama direktori di bawah `~/.claude/skills/` dan `name` plugin dalam manifestnya.

Perintah ini tidak memiliki flag untuk lokasi lain. Untuk rangka kerja di dalam proyek, lihat [Buat plugin](/docs/id/plugins/create).

| Flag                     | Deskripsi                                                                                                     |
| :----------------------- | :------------------------------------------------------------------------------------------------------------ |
| `--description <text>`   | Deskripsi manifest                                                                                            |
| `--author <name>`        | Nama penulis. Default ke `git config user.name`                                                               |
| `--author-email <email>` | Email penulis. Default ke `git config user.email`                                                             |
| `--with <components...>` | Juga rangka kerja file pemula untuk `skills`, `agents`, `hooks`, `mcp`, `lsp`, `output-style`, atau `channel` |
| `-f, --force`            | Timpa `.claude-plugin/` yang ada di target                                                                    |

Rangka kerja plugin dengan file skill dan hook pemula:

```bash theme={null}
claude plugin init my-helper --with skills hooks
```

Claude Code memvalidasi apa yang ditulis dan mencetak `Created plugin "my-helper" at ~/.claude/skills/my-helper`, diikuti oleh id yang dimuat sebagai dan perintah `claude plugin disable` yang mematikannya.

Claude Code keluar `1` tanpa menulis ketika tidak dapat rangka kerja dengan aman, dan pesan menamai alasannya. Ini adalah alasan umum:

* Nilai `--with` yang tidak dikenal
* Rangka kerja yang ada di target tanpa `--force`
* Pengaturan terkelola yang memblokir plugin direktori skills

<h3 id="plugin-install">
  plugin install
</h3>

Instal plugin dari marketplace yang telah Anda tambahkan. `i` adalah alias untuk `install`.

```bash theme={null}
claude plugin install <plugin> [options]
```

Sebagian besar plugin diinstal tanpa prompt. Untuk plugin yang entri marketplacenya [menjalankan perintah untuk menginstalnya](/docs/id/plugins/host-marketplace) atau [menetapkan `headersHelper` untuk downloadnya](/docs/id/plugins/host-marketplace#how-users-accept-a-headershelper-command), Claude Code terlebih dahulu mencetak perintah dan menanyakan `Run this command now? [y/N]`.

| Flag                        | Deskripsi                                                                                                                                                                                                                                                                                                                                |
| :-------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `-s, --scope <scope>`       | Cakupan instalasi: `user`, `project`, atau `local`. Default ke `user`                                                                                                                                                                                                                                                                    |
| `--config <key=value>`      | Tetapkan opsi [`userConfig`](/docs/id/plugins/manifest-reference) yang dideklarasikan manifest plugin. Ulangi flag untuk setiap opsi. Memerlukan Claude Code v2.1.147 atau lebih baru                                                                                                                                                         |
| `-y, --yes`                 | Terima perintah instal yang ditampilkan tanpa prompt `Run this command now?`. Diabaikan ketika perintah berjalan di dalam sesi Claude Code, seperti dari alat Bash atau hook. Memerlukan Claude Code v2.1.229 atau lebih baru                                                                                                            |
| `--accept-command <sha256>` | Terima perintah instal yang ditampilkan yang `sha256` sebelumnya [`--json` run](#plugin-json-result) dilaporkan dalam `shownCommand`, sebagai pengganti `-y`. Tidak dapat digabungkan dengan `-y`. Lihat [Terima perintah instal yang ditampilkan](#accept-a-displayed-install-command). Memerlukan Claude Code v2.1.271 atau lebih baru |
| `--json`                    | Cetak hasil sebagai satu objek JSON pada baris terakhir stdout alih-alih pesan yang dapat dibaca manusia, untuk digunakan dalam skrip. Lihat [Format hasil JSON](#plugin-json-result). Memerlukan Claude Code v2.1.268 atau lebih baru                                                                                                   |

Lewatkan `-y` dari terminal Anda sendiri untuk menerima perintah yang ditampilkan tanpa prompt. Berikut yang terjadi tanpa TTY dan ketika Claude menjalankan perintah:

* **stdin atau stdout bukan TTY, dan Anda tidak melewatkan `-y` atau `--accept-command`**: instalasi ditolak. Output mengatakan perintah hanya ditampilkan, dan kode keluar adalah `1`
* **Claude menjalankan perintah melalui alat Bash-nya**: `-y` diabaikan. Jalankan perintah dari terminal Anda sendiri

Instal plugin untuk semua orang yang mengkloning proyek:

```bash theme={null}
claude plugin install formatter@my-marketplace --scope project
```

Claude Code mencetak `Successfully installed plugin: formatter@my-marketplace (scope: project)`. Ketika tidak ada yang baru diinstal, output mengatakan mengapa:

* **Sudah diinstal di cakupan itu**: output adalah `Plugin "formatter@my-marketplace" is already installed (scope: project)` dan kode keluar adalah `0`
* **Anda menolak prompt sumber perintah**: output adalah `Aborted.` dan kode keluar adalah `1`
* **Anda menolak prompt `headersHelper`, atau tidak dapat dikonfirmasi tanpa TTY**: output adalah `Aborted — the command was not run.` dan kode keluar adalah `1`

<h4 id="plugin-json-result">
  Format hasil JSON
</h4>

Ketika Anda melewatkan `--json` ke `plugin install`, baris terakhir stdout adalah satu objek JSON. Parsing hanya baris itu, karena Claude Code mencetak perintah apa pun yang dideklarasikan marketplace sebelumnya.

Tiga field selalu ada:

* `command`: subperintah yang berjalan, seperti `install`
* `outcome`: `ok` atau `failed`
* `message`: deskripsi hasil yang dapat dibaca manusia

Field lain, seperti `pluginId`, `scope`, dan `failureCode`, muncul hanya ketika berlaku.

Kesalahan penggunaan, seperti `--scope` yang tidak valid, tidak mencetak baris hasil dan keluar `1` dengan alasan di stderr.

<h4 id="accept-a-displayed-install-command">
  Terima perintah instal yang ditampilkan
</h4>

Ketika run `--json` menampilkan perintah yang dideklarasikan marketplace dan tidak menjalankannya, hasil `failed` juga membawa objek `shownCommand`. Fieldnya mencakup perintah seperti yang ditampilkan, plugin yang dimilikinya, dan `sha256` perintah.

Untuk menerima perintah yang tepat itu, jalankan kembali dengan `sha256` itu sebagai `--accept-command` dari terminal Anda sendiri, karena flag tidak memiliki efek di dalam sesi Claude Code. Memerlukan Claude Code v2.1.271 atau lebih baru.

`sha256` dihitung sebagai penerimaan untuk perintah, plugin, dan katalog marketplace yang tepat. Jika salah satu dari mereka berubah sejak perintah ditampilkan, Claude Code tidak menerima `sha256` dan menampilkan perintah lagi. Perubahan yang refresh marketplace run sendiri ambil juga dihitung sebagai perubahan seperti itu.

Jika `shownCommand.acceptCommandMatched` adalah `false`, `sha256` yang Anda lewatkan tidak cocok dengan perintah yang sekarang ditampilkan. Tinjau perintah itu sebelum menjalankan kembali dengan `sha256`-nya.

<h3 id="plugin-uninstall">
  plugin uninstall
</h3>

Hapus plugin yang diinstal dari satu cakupan. `remove` dan `rm` adalah alias untuk `uninstall`.

```bash theme={null}
claude plugin uninstall <plugin> [options]
```

| Flag                  | Deskripsi                                                                                                                                                                                                                                |
| :-------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `-s, --scope <scope>` | Copot dari cakupan: `user`, `project`, atau `local`. Default ke `user`                                                                                                                                                                   |
| `--keep-data`         | Pertahankan direktori data persisten plugin, `~/.claude/plugins/data/<id>/`                                                                                                                                                              |
| `--prune`             | Juga hapus [dependensi](/docs/id/plugins/dependencies) yang diinstal otomatis yang tidak diperlukan plugin yang tersisa                                                                                                                       |
| `-y, --yes`           | Lewati prompt konfirmasi `--prune`. Diperlukan dengan `--prune` ketika stdin atau stdout bukan TTY                                                                                                                                       |
| `--json`              | Cetak hasil sebagai satu objek JSON pada baris terakhir stdout, dalam [format yang sama seperti `plugin install --json`](#plugin-json-result). Tidak dapat digabungkan dengan `--prune`. Memerlukan Claude Code v2.1.268 atau lebih baru |

Copot plugin dari cakupan proyek:

```bash theme={null}
claude plugin uninstall formatter@my-marketplace --scope project
```

Claude Code mencetak `Successfully uninstalled plugin: formatter (scope: project)`. Ketika plugin tidak diinstal di cakupan itu, perintah mencetak baris yang dimulai `Failed to uninstall plugin "formatter@my-marketplace":` dan keluar `1`.

<h3 id="plugin-enable">
  plugin enable
</h3>

Aktifkan plugin yang dinonaktifkan. Untuk [plugin yang disinkronkan dari claude.ai](/docs/id/plugins/loading#synced-plugins), lewatkan `<name>@synced` sebagai plugin.

```bash theme={null}
claude plugin enable <plugin> [options]
```

| Flag                  | Deskripsi                                                                                                                                                                                      |
| :-------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `-s, --scope <scope>` | Cakupan untuk diaktifkan: `user`, `project`, atau `local`. Terdeteksi otomatis ketika dihilangkan                                                                                              |
| `--json`              | Cetak hasil sebagai satu objek JSON pada baris terakhir stdout, dalam [format yang sama seperti `plugin install --json`](#plugin-json-result). Memerlukan Claude Code v2.1.268 atau lebih baru |

Tanpa `--scope`, perintah memeriksa file pengaturan Anda dalam urutan lokal, proyek, pengguna, dan menggunakan cakupan pertama yang menyebutkan plugin.

Jika Anda melewatkan `--scope` di mana plugin tidak dideklarasikan, perintah menulis override atau gagal:

* **Cakupan yang [diutamakan](/docs/id/plugins/loading) di atas yang mendeklarasikan**: Claude Code menulis override di cakupan yang Anda lewatkan. Misalnya, `claude plugin disable formatter --scope local` mematikan plugin yang diaktifkan proyek hanya untuk Anda
* **Cakupan lain**: perintah gagal dengan `Plugin "formatter" is installed at project scope, not user. Use --scope project or omit --scope to auto-detect.`

Jika plugin sudah diaktifkan di cakupan yang diselesaikan, perintah mencetak `Plugin "formatter" is already enabled` dan keluar `1`. Dengan `--json`, hasil memiliki `"failureCode": "already_in_goal_state"` dan `"alreadyInGoalState": true`, jadi skrip dapat memperlakukan kasus itu sebagai kesuksesan.

Ketika plugin mendeklarasikan [dependensi](/docs/id/plugins/dependencies), Claude Code juga mengaktifkannya. Perintah gagal dalam kasus ini:

* **Dependensi tidak diinstal**: aktifkan gagal dan mencetak perintah `claude plugin install` untuk setiap dependensi yang hilang
* **Dependensi diblokir oleh kebijakan plugin organisasi Anda**: aktifkan gagal dan menamai dependensi yang diblokir
* **Dependensi diatur ke `false` di cakupan dengan prioritas lebih tinggi dari cakupan target**: aktifkan gagal. Aktifkan dependensi di cakupan itu, atau lewatkan `--scope` untuk menulis di sana

Aktifkan kembali plugin di mana pun dideklarasikan:

```bash theme={null}
claude plugin enable formatter
```

Claude Code mencetak `Successfully enabled plugin: formatter (scope: project)`, menamai cakupan yang terdeteksi.

<h3 id="plugin-disable">
  plugin disable
</h3>

Nonaktifkan plugin tanpa mencopot. Untuk [plugin yang disinkronkan dari claude.ai](/docs/id/plugins/loading#synced-plugins), lewatkan `<name>@synced` sebagai plugin.

```bash theme={null}
claude plugin disable [plugin] [options]
```

| Flag                  | Deskripsi                                                                                                                                                                                      |
| :-------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `-a, --all`           | Nonaktifkan setiap plugin yang diaktifkan. Tidak dapat digabungkan dengan nama plugin atau `--scope`                                                                                           |
| `-s, --scope <scope>` | Cakupan untuk dinonaktifkan: `user`, `project`, atau `local`. Terdeteksi otomatis ketika dihilangkan                                                                                           |
| `--json`              | Cetak hasil sebagai satu objek JSON pada baris terakhir stdout, dalam [format yang sama seperti `plugin install --json`](#plugin-json-result). Memerlukan Claude Code v2.1.268 atau lebih baru |

Tanpa `--scope`, cakupan terdeteksi otomatis dalam urutan lokal, proyek, pengguna yang sama seperti [`plugin enable`](#plugin-enable).

Jika Anda tidak melewatkan nama plugin atau `--all`, Claude Code mencetak `Please specify a plugin name or use --all to disable all plugins` dan keluar `1`. Menonaktifkan plugin yang sudah dinonaktifkan mencetak `Plugin "formatter" is already disabled` dan keluar `1`, seperti yang dilakukan [`plugin enable`](#plugin-enable) untuk plugin yang sudah diaktifkan.

Perintah gagal untuk plugin yang masih diperlukan:

* **Plugin yang diaktifkan lain [bergantung pada](/docs/id/plugins/dependencies) itu**: perintah gagal dan menamai dependennya untuk dinonaktifkan terlebih dahulu
* **Organisasi Anda memerlukannya sebagai plugin yang disinkronkan**: perintah gagal dan tidak menyimpan apa pun

Nonaktifkan satu plugin:

```bash theme={null}
claude plugin disable formatter
```

Claude Code mencetak `Successfully disabled plugin: formatter (scope: project)`.

<h3 id="plugin-update">
  plugin update
</h3>

Perbarui plugin ke versi terbaru yang ditawarkan marketplacenya. Versi baru dimuat dalam sesi berikutnya Anda, atau setelah Anda menjalankan `/reload-plugins` di sesi yang sedang berjalan.

```bash theme={null}
claude plugin update <plugin> [options]
```

| Flag                        | Deskripsi                                                                                                                                                                                                                                                  |
| :-------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `-s, --scope <scope>`       | Cakupan untuk diperbarui: `user`, `project`, `local`, atau `managed`. Default ke cakupan tempat plugin diinstal                                                                                                                                            |
| `-y, --yes`                 | Terima perintah instal yang berubah dari plugin [command-source](/docs/id/plugins/host-marketplace), tanpa prompt. Diperlukan ketika stdin atau stdout bukan TTY, kecuali Anda melewatkan `--accept-command`. Memerlukan Claude Code v2.1.229 atau lebih baru   |
| `--accept-command <sha256>` | Terima perintah yang dideklarasikan marketplace yang `sha256` sebelumnya [`--json` run](#plugin-json-result) dilaporkan dalam `shownCommand`, sebagai pengganti `-y`. Tidak dapat digabungkan dengan `-y`. Memerlukan Claude Code v2.1.271 atau lebih baru |
| `--json`                    | Cetak hasil sebagai satu objek JSON pada baris terakhir stdout, dalam [format yang sama seperti `plugin install --json`](#plugin-json-result). Memerlukan Claude Code v2.1.268 atau lebih baru                                                             |

`managed` adalah satu-satunya cakupan yang dapat Anda perbarui tetapi tidak instal. Untuk plugin yang diinstal admin, lihat [Kelola plugin untuk organisasi Anda](/docs/id/plugins/org).

Perbarui plugin:

```bash theme={null}
claude plugin update formatter@my-marketplace
```

Claude Code mencetak `Checking for updates for plugin "formatter@my-marketplace"…`, kemudian hasilnya. Ketika tidak ada yang lebih baru, mencetak `formatter is already at the latest version (1.0.0).` dan keluar `0`.

Anda dapat melewatkan nama plugin telanjang, yang dicocokkan perintah terhadap plugin yang diinstal. Ketika plugin yang diinstal dari marketplace berbeda berbagi nama, perintah menolak pembaruan dan mencantumkan perintah `plugin-name@marketplace-name` yang memenuhi syarat untuk dijalankan. Memperbarui dengan nama telanjang memerlukan Claude Code v2.1.246 atau lebih baru.

<h3 id="plugin-list">
  plugin list
</h3>

Daftar plugin yang diinstal dengan versi, cakupan, dan status mereka.

```bash theme={null}
claude plugin list [options]
```

| Flag          | Deskripsi                                                                                                    |
| :------------ | :----------------------------------------------------------------------------------------------------------- |
| `--json`      | Cetak daftar sebagai JSON                                                                                    |
| `--available` | Juga daftar plugin yang ditawarkan marketplace Anda yang belum Anda instal. Tidak berpengaruh tanpa `--json` |

Claude Code mengelompokkan output yang dapat dibaca manusia berdasarkan cara setiap plugin dimuat:

* **`Installed plugins:`**: plugin yang Anda instal dari marketplace
* **`Session-only plugins (--plugin-dir / --plugin-url):`**: plugin yang dimuat oleh flag itu dalam perintah yang sama, seperti dalam `claude --plugin-dir ./my-plugin plugin list`
* **`Skills-directory plugins (.claude/skills/*):`**: plugin yang Claude Code temukan di direktori skills
* **`Synced from claude.ai`**: [plugin yang disinkronkan dari akun claude.ai Anda](/docs/id/plugins/loading#synced-plugins)

Tanpa apa pun di grup mana pun, Claude Code mencetak ``No plugins installed. Use `claude plugin install` to install a plugin.``

<h4 id="json-output">
  Output JSON
</h4>

Dengan `--json`, Claude Code mencetak array dengan satu objek per instalasi. Setiap objek membawa field di bawah. `id`, `version`, `scope`, `enabled`, dan `installPath` selalu ada, dan yang lain muncul hanya ketika berlaku.

| Field          | Tipe             | Deskripsi                                                                                                                                                                                                                                      |
| :------------- | :--------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `id`           | string           | `name@marketplace` untuk instal, `name@inline` untuk plugin hanya sesi, `name@skills-dir` untuk plugin direktori skills, `name@synced` untuk plugin yang disinkronkan dari claude.ai                                                           |
| `version`      | string           | Untuk instal marketplace, [versi yang Claude Code hitung](/docs/id/plugins/loading#versions-and-updates) saat instal. Untuk plugin hanya sesi, direktori skills, atau disinkronkan, `version` manifest, atau `unknown` ketika tidak mendeklarasikan |
| `scope`        | string           | `user`, `project`, `local`, atau `managed` untuk instal; `user` atau `project` untuk plugin direktori skills; `session` untuk plugin hanya sesi; `synced` untuk plugin yang disinkronkan dari claude.ai                                        |
| `enabled`      | boolean          | Apakah plugin diaktifkan dalam pengaturan gabungan Anda                                                                                                                                                                                        |
| `installPath`  | string           | Direktori tempat plugin dimuat                                                                                                                                                                                                                 |
| `installedAt`  | string           | Timestamp ISO instalasi. Hanya instal marketplace                                                                                                                                                                                              |
| `lastUpdated`  | string           | Timestamp ISO pembaruan terakhir. Hanya instal marketplace                                                                                                                                                                                     |
| `projectPath`  | string           | Proyek yang dimiliki instal. Hanya cakupan `project` dan `local`                                                                                                                                                                               |
| `mcpServers`   | object           | Definisi server MCP plugin, ketika plugin yang diinstal marketplace memiliki                                                                                                                                                                   |
| `errors`       | array of strings | Kesalahan muat, ketika plugin gagal dimuat                                                                                                                                                                                                     |
| `notes`        | array of strings | Peringatan penulisan untuk plugin yang dimuat dan berfungsi                                                                                                                                                                                    |
| `errorDetails` | array of objects | Satu objek per entri `errors`, memberikan `type` diagnostik dan nama yang dirujuknya, seperti plugin, marketplace, server, atau file. Memerlukan Claude Code v2.1.268 atau lebih baru                                                          |
| `noteDetails`  | array of objects | Objek detail yang sama untuk setiap entri `notes`. Memerlukan Claude Code v2.1.268 atau lebih baru                                                                                                                                             |

Dengan `--json --available`, Claude Code mencetak satu objek alih-alih array. Field `installed`-nya memegang array objek plugin yang diinstal, dan field `available`-nya memegang satu objek per plugin marketplace yang tidak diinstal dengan field di bawah.

| Field             | Tipe             | Deskripsi                                                                                                   |
| :---------------- | :--------------- | :---------------------------------------------------------------------------------------------------------- |
| `pluginId`        | string           | `name@marketplace`                                                                                          |
| `name`            | string           | Nama plugin di marketplace                                                                                  |
| `marketplaceName` | string           | Marketplace yang menawarkannya                                                                              |
| `source`          | string or object | [Sumber](/docs/id/plugins/marketplace-reference) entri marketplace: string untuk jalur relatif, objek sebaliknya |
| `description`     | string           | Deskripsi entri, ketika memiliki                                                                            |
| `version`         | string           | Versi entri, ketika mendeklarasikan                                                                         |
| `installCount`    | number           | Jumlah instal, ketika Claude Code memiliki satu untuk plugin                                                |

<h3 id="plugin-details">
  plugin details
</h3>

Tampilkan inventaris komponen plugin dan biaya token proyeksiannya.

Plugin harus dimuat: diinstal, ditemukan di direktori skills, atau dilewatkan dengan `--plugin-dir` atau `--plugin-url` dalam perintah yang sama. `<name>` adalah plugin `name` atau `name@marketplace`.

```bash theme={null}
claude plugin details <name>
```

Perintah tidak mengambil flag di luar `--help`.

Tampilkan apa yang disumbangkan plugin yang diinstal:

```bash theme={null}
claude plugin details formatter
```

Claude Code mencetak nama, versi, deskripsi, dan sumber plugin, kemudian bagian ini:

* **`Component inventory`**: skills, agents, hooks, server MCP, dan server LSP plugin
* **`Projected token cost`**: token yang selalu aktif yang ditambahkan plugin ke setiap sesi
* **`Per-component (rounded)`**: perkiraan selalu aktif dan on-invoke untuk setiap skill, agent, dan perintah. Dihilangkan ketika plugin tidak memiliki

Untuk apa dua angka biaya berarti, lihat [Ukur biaya dan penggunaan plugin](/docs/id/plugins/measure).

Untuk plugin yang tidak dimuat, Claude Code mencetak ``Plugin "formatter" not found. Run `claude plugin list` to see installed plugins, or pass --plugin-dir <path> to load one from disk.`` dan keluar `1`.

<h3 id="plugin-prune">
  plugin prune
</h3>

Hapus [dependensi](/docs/id/plugins/dependencies) yang diinstal otomatis yang tidak diperlukan plugin yang diinstal lagi. Perintah tidak pernah menghapus plugin yang Anda instal sendiri. `autoremove` adalah alias untuk `prune`.

```bash theme={null}
claude plugin prune [options]
```

| Flag                  | Deskripsi                                                               |
| :-------------------- | :---------------------------------------------------------------------- |
| `-s, --scope <scope>` | Prune di cakupan: `user`, `project`, atau `local`. Default ke `user`    |
| `--dry-run`           | Daftar apa yang akan dihapus tanpa menghapusnya                         |
| `-y, --yes`           | Lewati prompt konfirmasi. Diperlukan ketika stdin atau stdout bukan TTY |

Pratinjau apa yang akan dihapus prune:

```bash theme={null}
claude plugin prune --dry-run
```

Claude Code mencantumkan dependensi yatim piatu dan berakhir dengan `(dry run — nothing removed)`. Tanpa yang dihapus, mencetak baris yang dimulai `Nothing to prune`.

Tanpa `--dry-run`, perintah menghapus dependensi yatim piatu hanya setelah Anda mengonfirmasi di prompt atau melewatkan `-y`.

Kode keluar adalah `0` apa pun yang Anda jawab di prompt.

Apa yang dilakukan `prune` tergantung pada apakah terminal terpasang dan apakah Anda melewatkan `-y`:

| Terminal dan flag                         | Apa yang terjadi                                                                                 |
| :---------------------------------------- | :----------------------------------------------------------------------------------------------- |
| Terminal interaktif, tidak ada `-y`       | Mencantumkan dependensi yatim piatu dan menanyakan `Remove? [y/N]`                               |
| Terminal apa pun, `-y`                    | Menghapusnya dan mencetak `Removed N auto-installed plugins: <names>`                            |
| stdin atau stdout non-TTY, tidak ada `-y` | Mencetak daftar dan ``Not a TTY — run `claude plugin prune -y` to remove.``, menghapus tidak ada |

<h3 id="plugin-eval">
  plugin eval
</h3>

Jalankan [kasus eval](/docs/id/plugin-evals) plugin dan laporkan hasil yang dinilai. Memerlukan Claude Code v2.1.269 atau lebih baru.

Setiap kasus adalah prompt plus grader. Claude Code menjalankannya beberapa kali dalam sesi terisolasi dengan hanya plugin target yang dimuat, dan secara default juga tanpa plugin sehingga laporan menunjukkan perbedaannya.

Lihat [Uji plugin dengan eval](/docs/id/plugin-evals) untuk format kasus, grader, hasil, dan penggunaan CI.

```bash theme={null}
claude plugin eval [target] [options]
```

`target` opsional default ke direktori saat ini dan mengambil salah satu bentuk ini:

* Direktori plugin
* File `prompt.md` atau `case.yaml` tunggal
* Plugin yang diinstal sebagai `name` atau `name@marketplace`
* `name@skills-dir`

Letakkan target sebelum `--tag`, `--allow-tools`, dan `--json`. Setiap opsi ini mengambil kata-kata yang mengikutinya sebagai nilainya, jadi target yang ditulis setelah salah satu dari mereka dibaca sebagai tag, nama alat, atau jalur output JSON alih-alih sebagai target.

Tabel ini mencantumkan opsi yang paling banyak digunakan run. Jalankan `claude plugin eval --help` untuk set lengkap, termasuk `--case`, `--tag`, `--output-dir`, `--report`, `--allow-real-servers`, `--keep-temp`, dan `--verbose`.

| Opsi                       | Deskripsi                                                                                                                                                           | Default                                                                            |
| :------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------ | :--------------------------------------------------------------------------------- |
| `--runs <n>`               | Berjalan per kasus di setiap [arm](/docs/id/plugin-evals#compare-against-a-no-plugin-baseline)                                                                           | `runs` setiap kasus, atau 3                                                        |
| `-j, --concurrency <n>`    | Sesi agen untuk dijalankan sekaligus, 1 hingga 8. Mereka berbagi batas laju Anda                                                                                    | `1`                                                                                |
| `--model <model>`          | Model untuk agen di bawah pengujian                                                                                                                                 | `model` setiap kasus, atau `ANTHROPIC_MODEL` jika diatur, atau default Claude Code |
| `--judge-model <model>`    | Model untuk grader `llm` dan `baseline`                                                                                                                             | Model kecil cepat                                                                  |
| `--ablation <mode>`        | `none` atau `with-without`. Lihat [Bandingkan dengan baseline tanpa plugin](/docs/id/plugin-evals#compare-against-a-no-plugin-baseline)                                  | `with-without` ketika plugin diselesaikan, atau `none`                             |
| `--threshold <0..1>`       | Keluar 1 jika kasus apa pun mencetak di bawah ini                                                                                                                   | `1.0`                                                                              |
| `--max-cost-usd <usd>`     | Berhenti sebelum run berikutnya setelah pengeluaran mencapai ini, keluar 2, dan laporkan hasil parsial                                                              | Tidak ada batas                                                                    |
| `--allow-tools <tools...>` | Berikan alat di luar set hanya baca, seperti `Bash`, `Write`, `Edit`, atau `"mcp__plugin_<plugin>_<server>__*"`. Lihat [Berikan alat](/docs/id/plugin-evals#grant-tools) |                                                                                    |
| `--scaffold`               | Jalankan [`scaffold_script`](/docs/id/plugin-evals#add-setup-or-history-with-case-yaml) setiap kasus                                                                     | Mati                                                                               |
| `--trust-plugin`           | Lewati prompt kepercayaan run pertama, untuk CI. Lihat [Apa yang dapat diakses run](/docs/id/plugin-evals#security)                                                      | Mati                                                                               |
| `--mocks <mode>`           | `record` atau `off`. Lihat [Mock server MCP](/docs/id/plugin-evals#mock-mcp-servers)                                                                                     | `record`                                                                           |
| `--eval-dir <dir>`         | Direktori di bawah plugin yang memegang kasus                                                                                                                       | `experimental.evals` manifest, atau `evals`                                        |
| `--json [path]`            | Cetak [dokumen hasil](/docs/id/plugin-evals#json-result) ke stdout, atau tulis ke jalur `.json`                                                                          |                                                                                    |
| `--no-publish`             | Jaga laporan HTML tetap lokal                                                                                                                                       |                                                                                    |

Kode keluar melaporkan bagaimana run berakhir. Untuk bertindak atas itu dalam pipeline, lihat [Jalankan eval dalam CI](/docs/id/plugin-evals#run-evals-in-ci).

| Kode keluar | Arti                                                                         |
| :---------- | :--------------------------------------------------------------------------- |
| `0`         | Setiap kasus memenuhi ambang                                                 |
| `1`         | Kasus yang gagal, kesalahan muat, atau direktori plugin yang tidak dipercaya |
| `2`         | Run parsial                                                                  |
| `130`       | Terputus                                                                     |
| `143`       | Dihentikan                                                                   |

<h3 id="plugin-eval-init">
  plugin eval init
</h3>

Buat suite eval untuk plugin di direktori saat ini. Memerlukan Claude Code v2.1.269 atau lebih baru. Lihat [Buat suite eval pertama Anda](/docs/id/plugin-evals#create-your-first-eval-suite).

```bash theme={null}
claude plugin eval init [name] [options]
```

Di terminal, perintah membuka sesi Claude Code interaktif untuk wawancara penulisan. Dalam wawancara, Claude melakukan hal berikut:

1. Membaca plugin
2. Menanyakan apa yang harus dilakukan dengan baik
3. Mengusulkan kasus dan grader
4. Menulis file kasus
5. Menjalankan kasus dan meninjau nilai dengan Anda untuk memeriksa bahwa grader mencetak cara Anda

Dengan `--bare`, atau tanpa terminal, perintah menulis template kasus tunggal kosong. Ketika Claude menjalankan perintah dari dalam sesi Claude Code, perintah mencetak instruksi wawancara untuk sesi itu ikuti daripada menulis template.

`name` opsional adalah nama kasus. Diperlukan dengan `--bare` atau tanpa terminal, karena perintah menulis template kosong untuk kasus itu. Wawancara tidak memerlukan.

Perintah menerima opsi ini:

| Opsi                | Deskripsi                                                                                         | Default                                     |
| :------------------ | :------------------------------------------------------------------------------------------------ | :------------------------------------------ |
| `--bare`            | Tulis `prompt.md` dan `graders/criteria.md` kosong untuk `<name>` alih-alih menjalankan wawancara |                                             |
| `-i, --interactive` | Perlukan wawancara. Gagal tanpa terminal alih-alih menulis template                               |                                             |
| `--eval-dir <dir>`  | Direktori di bawah direktori saat ini untuk menulis kasus ke                                      | `experimental.evals` manifest, atau `evals` |

<h3 id="plugin-tag">
  plugin tag
</h3>

Buat tag git beranotasi bernama `<name>--v<version>` untuk rilis plugin. Sebelum menandai, perintah memeriksa bahwa `plugin.json` plugin dan entri marketplace apa pun yang mencantumkannya setuju pada versi.

Untuk kapan menandai rilis, lihat [Terbitkan plugin](/docs/id/plugins/publish).

```bash theme={null}
claude plugin tag [path] [options]
```

`[path]` adalah direktori plugin, default ke direktori saat ini. Perintah menemukan entri marketplace dengan berjalan naik dari direktori itu ke `.claude-plugin/marketplace.json` yang mencantumkan plugin.

| Flag                  | Deskripsi                                                                    |
| :-------------------- | :--------------------------------------------------------------------------- |
| `--push`              | Dorong tag ke `--remote` setelah membuatnya                                  |
| `--dry-run`           | Cetak apa yang akan ditandai tanpa membuat tag                               |
| `-f, --force`         | Lewati pohon kerja kotor dan pemeriksaan tag-sudah-ada                       |
| `-m, --message <msg>` | Pesan anotasi tag. `%s` singkatan untuk versi. Default ke `<name> <version>` |
| `--remote <name>`     | Remote untuk didorong dengan `--push`. Default ke `origin`                   |

Pratinjau tag untuk plugin dalam checkout marketplace:

```bash theme={null}
claude plugin tag plugins/formatter --dry-run
```

Claude Code mencetak rencana:

* Nama plugin
* Versi dan file mana asalnya
* Entri marketplace yang cocok, ketika ada
* Nama tag
* Perintah `git tag` dan `git push` yang akan dijalankan

Tanpa `--dry-run`, Claude Code mencetak `Created tag formatter--v1.0.0` dan baik `Pushed to origin` atau perintah push untuk dijalankan sendiri. Jika push gagal, tag masih dibuat secara lokal dan perintah keluar dengan kesalahan.

Perintah keluar `1` dan mencetak alasan ketika tidak dapat menandai dengan aman. Alasan umum adalah:

* Tidak ada `version` di `plugin.json` atau entri marketplace
* Tag sudah ada
* Pohon kerja kotor

<h3 id="plugin-validate">
  plugin validate
</h3>

Validasi manifest plugin, manifest marketplace, atau skills, agents, dan perintah dalam direktori, dan keluar dengan kode yang dapat ditindaklanjuti pekerjaan CI. Untuk alur kerja buat, uji, dan edit, lihat [Buat plugin](/docs/id/plugins/create). Untuk apa yang diperiksa validator di setiap manifest, lihat [referensi manifest plugin](/docs/id/plugins/manifest-reference) dan [referensi marketplace](/docs/id/plugins/marketplace-reference).

```bash theme={null}
claude plugin validate <path> [options]
```

| Flag       | Deskripsi                                                                                                                                                                            |
| :--------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `--strict` | Perlakukan peringatan sebagai kesalahan, jadi field yang tidak dikenali dan metadata yang hilang yang ditoleransi runtime gagal run. Memerlukan Claude Code v2.1.145 atau lebih baru |
| `--json`   | Output laporan validasi sebagai satu objek JSON dengan kode keluar yang sama. Memerlukan Claude Code v2.1.259 atau lebih baru                                                        |

Validasi plugin sebelum melakukan:

```bash theme={null}
claude plugin validate ./my-plugin --strict
```

<h4 id="validate-a-directory">
  Validasi direktori
</h4>

`<path>` adalah file manifest atau direktori. Diberikan direktori, Claude Code memilih apa yang akan divalidasi oleh apa yang ditemukannya di sana:

* `.claude-plugin/marketplace.json`, ketika ada
* Sebaliknya `.claude-plugin/plugin.json`
* Sebaliknya file komponen, dipilih oleh nama direktori. Memvalidasi file komponen tanpa manifest memerlukan Claude Code v2.1.233 atau lebih baru:
  * Direktori bernama `skills`, `agents`, atau `commands`: file di dalamnya
  * Direktori bernama `.claude`: direktori `skills`, `agents`, dan `commands` di dalamnya
  * Direktori lain: tiga direktori itu di bawah `.claude`-nya

Claude Code tidak mengikuti symlink di dalam direktori yang Anda beri nama. Apa yang dilakukan tergantung di mana linknya:

* **Direktori `skills`, `agents`, atau `commands` yang tertaut di bawah root plugin atau `.claude`**: Claude Code memperingatkan bahwa tidak ada yang di dalamnya dibaca.
* **Entri tertaut di dalam direktori `skills`, `agents`, atau `commands`**: Claude Code melewatinya dan memperingatkan, per direktori, berapa banyak entri yang dilewati yang akan dimuat sesi.
* **Direktori `skills`, `agents`, atau `commands` yang Anda beri nama adalah symlink sendiri, atau direktori `.claude` induknya**: Claude Code melaporkan kesalahan dan tidak memeriksa apa pun di dalamnya. Beri nama direktori nyata.

Beberapa file tidak dibaca oleh run validasi:

* **`SKILL.md` di root plugin**: ketika Anda menjalankan `claude plugin validate` terhadap direktori plugin, Claude Code tidak memeriksa `SKILL.md` di root plugin
* **`CLAUDE.md` di root plugin**: dalam run plugin, Claude Code juga memperingatkan tentang `CLAUDE.md` di root plugin
* **File plugin dalam run marketplace**: dari direktori marketplace, Claude Code tidak membuka file skill, agent, command, atau hook plugin. Untuk menemukan kesalahan di file itu, validasi setiap direktori plugin

<h4 id="output-and-exit-codes">
  Output dan kode keluar
</h4>

Claude Code mencetak file yang divalidasi, kesalahan dan peringatan apa pun dengan jalurnya, dan baris putusan. Kode keluar mengikuti putusan:

| Kode keluar | Baris putusan                                                                     | Arti                                                                |
| :---------- | :-------------------------------------------------------------------------------- | :------------------------------------------------------------------ |
| `0`         | `Validation passed` atau `Validation passed with warnings`                        | Manifest dimuat. Dengan `--strict`, tidak ada peringatan            |
| `1`         | `Validation failed` atau `Validation failed (--strict treats warnings as errors)` | Kesalahan, atau peringatan di bawah `--strict`                      |
| `2`         | `Unexpected error during validation: <reason>`                                    | Validator sendiri gagal, seperti pada jalur yang tidak dapat dibaca |

Dengan `--json`, Claude Code menulis laporan ke stdout sebagai satu objek JSON dengan field tingkat atas ini:

* `success`: putusan yang sama yang diberikan kode keluar
* `strict`: apakah run memperlakukan peringatan sebagai kesalahan
* `target`: jalur yang diselesaikan Claude Code divalidasi
* `manifest`: hasil manifest sendiri, atau `null` untuk run tanpa manifest
* `contents`: hasil per-file, masing-masing menamai `file`-nya dan membawa array `errors`, `warnings`, dan `notes`

Pada keluar `2`, perintah tidak menulis apa pun ke stdout. Pesan kesalahan pergi ke stderr.

<h2 id="claude-plugin-marketplace-commands">
  Perintah claude plugin marketplace
</h2>

Jalankan `claude plugin marketplace <subcommand>` dari shell Anda untuk menambah, mencantumkan, menyegarkan, dan menghapus marketplace yang Anda instal plugin.

* **Kode keluar**: subperintah ini mengikuti [konvensi kode keluar](#claude-plugin-commands) perintah plugin
* **Cakupan**: flag `--scope` mereka tidak memiliki bentuk pendek `-s`

Untuk apa marketplace dan bagaimana Claude Code menyimpannya, lihat [Referensi pemuatan plugin](/docs/id/plugins/loading).

<h3 id="plugin-marketplace-add">
  plugin marketplace add
</h3>

Tambahkan marketplace dari repositori GitHub, URL git, `marketplace.json` yang dihosting, atau jalur lokal, dan deklarasikan dalam file pengaturan.

Setelah menambahkannya, Claude Code menginstal [dependensi](/docs/id/plugins/dependencies) apa pun yang hilang dari plugin yang diinstal.

```bash theme={null}
claude plugin marketplace add <source> [options]
```

| Flag                  | Deskripsi                                                                                                                                                                     |
| :-------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `--scope <scope>`     | File pengaturan untuk mendeklarasikan marketplace: `user`, `project`, atau `local`. Default ke `user`                                                                         |
| `--sparse <paths...>` | Batasi checkout git ke direktori ini, untuk monorepo. Hanya sumber `github` dan `git`                                                                                         |
| `--claudeai`          | Baca argumen sebagai nama [marketplace yang dihosting di claude.ai](/docs/id/plugins/install#add-from-claude-ai) alih-alih sumber. Memerlukan Claude Code v2.1.273 atau lebih baru |

`<source>` mengambil salah satu bentuk dalam tabel di bawah, dan bentuknya memutuskan tipe sumber dan cara Claude Code mengambil marketplace. Untuk objek sumber yang dihasilkan, lihat [referensi marketplace](/docs/id/plugins/marketplace-reference).

| Anda ketik                                                                                | Tipe sumber | Cara Claude Code mengambilnya                                                                                                |
| :---------------------------------------------------------------------------------------- | :---------- | :--------------------------------------------------------------------------------------------------------------------------- |
| `owner/repo`, `owner/repo#ref`, atau `owner/repo@ref`                                     | `github`    | Mengkloning repositori GitHub, disematkan ke `ref` ketika diberikan. Pemilik dan repo harus mengikuti aturan penamaan GitHub |
| `user@host:path[.git][#ref]`                                                              | `git`       | Mengkloning melalui SSH                                                                                                      |
| `https://example.com/repo.git[#ref]`, atau URL yang berisi `/_git/`                       | `git`       | Mengkloning melalui HTTPS, termasuk URL Azure DevOps                                                                         |
| `https://github.com/owner/repo` atau `https://gitlab.com/namespace/project`               | `git`       | Mengkloning melalui HTTPS setelah menambahkan `.git`                                                                         |
| URL `http://` atau `https://` lain, termasuk host git yang dihosting sendiri tanpa `.git` | `url`       | Mengambil URL sebagai `marketplace.json`. Untuk mengkloning repositori di sana, tambahkan `.git`                             |
| `./path`, `../path`, `/path`, atau `~/path` ke direktori                                  | `directory` | Membaca direktori di tempat. Di Windows, bentuk `.\`, `..\`, dan `C:\` juga berfungsi                                        |
| Bentuk jalur yang sama, ke file `.json`                                                   | `file`      | Membaca file di tempat                                                                                                       |

Untuk host yang URL klonnya tidak membawa akhiran `.git`, seperti AWS CodeCommit, tambahkan marketplace sebagai entri git dalam [`extraKnownMarketplaces`](/docs/id/settings-reference#extraknownmarketplaces). Claude Code mengkloning entri git apakah atau tidak URL-nya berakhir dengan `.git`.

Claude Code juga mengkloning URL `gitlab.com` dengan subgrup bersarang, seperti `https://gitlab.com/group/subgroup/project`.

Tambahkan marketplace dan bagikan dengan proyek:

```bash theme={null}
claude plugin marketplace add your-org/your-marketplace --scope project
```

Claude Code mencetak `Successfully added marketplace: your-marketplace (declared in project settings)`, menggunakan `name` dari manifest marketplace sendiri. Penambahan berulang atau sumber yang tidak valid mencetak salah satu hasil ini:

* **Marketplace sudah di disk**: output adalah `Marketplace 'your-marketplace' already on disk — declared in project settings` dan kode keluar adalah `0`
* **Sumber yang tidak dikenali**: output adalah `Invalid marketplace source format. Try: owner/repo, https://..., or ./path` dan kode keluar adalah `1`
* **Host telanjang seperti `gitlab.example.com/team/plugins`**: penambahan gagal sebagai shorthand `owner/repo` yang tidak valid, dan pesan memberi tahu Anda untuk menambahkan `https://` atau menggunakan jalur lokal

Tambahkan [marketplace yang dihosting di claude.ai](/docs/id/plugins/install#add-from-claude-ai) dengan nama yang dicetak dalam bagian `From claude.ai:` dari `claude plugin marketplace list`:

```bash theme={null}
claude plugin marketplace add --claudeai claudeai-organization-library
```

Dengan `--claudeai`, perintah menolak `--scope` dan `--sparse`. Marketplace dihosting untuk akun Anda, bukan dideklarasikan dalam file pengaturan, jadi Anda tidak dapat membagikannya melalui `.claude/settings.json` proyek.

<h3 id="plugin-marketplace-list">
  plugin marketplace list
</h3>

Daftar setiap marketplace yang telah Anda tambahkan, dengan sumbernya.

```bash theme={null}
claude plugin marketplace list [options]
```

| Flag     | Deskripsi                 |
| :------- | :------------------------ |
| `--json` | Cetak daftar sebagai JSON |

Claude Code mencetak `Configured marketplaces:` dan satu baris `Source:` per marketplace, atau `No marketplaces configured`.

Dengan `--json`, Claude Code mencetak array dengan satu objek per marketplace, membawa field di bawah. Setiap field adalah string.

| Field             | Deskripsi                                                                           |
| :---------------- | :---------------------------------------------------------------------------------- |
| `name`            | Nama marketplace                                                                    |
| `source`          | `github`, `git`, `url`, `directory`, `file`, atau `claudeai`                        |
| `repo`            | `owner/repo`. Hanya sumber `github`                                                 |
| `url`             | URL kloning atau pengambilan. Hanya sumber `git` dan `url`                          |
| `path`            | Jalur lokal. Hanya sumber `directory` dan `file`                                    |
| `ref`             | Cabang atau tag yang disematkan. Sumber `github` dan `git`, hanya ketika disematkan |
| `installLocation` | Di mana Claude Code menyimpan marketplace                                           |

Marketplace [claude.ai](/docs/id/plugins/install#add-from-claude-ai) yang ditambahkan tidak memiliki kloning lokal, jadi entrinya membawa pengidentifikasi claude.ai-nya, `marketplaceId` dan `organizationUuid`, sebagai pengganti `installLocation`. Juga membawa `scope` ketika satu dicatat, dan `status`.

Jika sesi terminal Anda [menyinkronkan plugin dari akun claude.ai Anda](/docs/id/plugins/loading#synced-plugins), daftar teks berakhir dengan bagian `From claude.ai:`. Bagian itu menamai marketplace yang claude.ai daftar untuk akun Anda yang belum Anda tambahkan, baik berbasis git maupun dihosting. Memerlukan Claude Code v2.1.273 atau lebih baru.

Untuk menambahkan marketplace dari bagian itu, lihat [Tambahkan marketplace dari claude.ai](/docs/id/plugins/install#add-from-claude-ai).

Output `--json` mencakup marketplace yang dikonfigurasi saja dan meninggalkan bagian.

<h3 id="plugin-marketplace-remove">
  plugin marketplace remove
</h3>

Hapus deklarasi marketplace dari pengaturan Anda. `rm` adalah alias untuk `remove`.

<Warning>
  Ketika Anda menghapus marketplace dari cakupan terakhir yang mendeklarasikannya, Claude Code juga menghapus cache dan mencopot setiap plugin yang Anda instal darinya. Tanpa `--scope`, perintah menghapus deklarasi dari setiap cakupan. Untuk menyegarkan marketplace tanpa kehilangan plugin, jalankan `plugin marketplace update`.
</Warning>

```bash theme={null}
claude plugin marketplace remove <name> [options]
```

`<name>` adalah nama marketplace yang ditunjukkan `plugin marketplace list`, bukan sumber yang Anda lewatkan ke `add`.

| Flag              | Deskripsi                                                                                                                                    |
| :---------------- | :------------------------------------------------------------------------------------------------------------------------------------------- |
| `--scope <scope>` | Hapus deklarasi dari satu cakupan pengaturan: `user`, `project`, atau `local`. Tanpanya, Claude Code menghapus deklarasi dari setiap cakupan |

Hapus marketplace dari setiap cakupan:

```bash theme={null}
claude plugin marketplace remove your-marketplace
```

Claude Code mencetak `Successfully removed marketplace: your-marketplace`, menambahkan `(from project settings)` ketika Anda membatasi cakupannya. Jika Anda membatasi ke file pengaturan yang tidak mendeklarasikan marketplace, perintah gagal dengan `Marketplace 'your-marketplace' is not declared in project settings. Omit --scope to remove it from all scopes.`

<h3 id="plugin-marketplace-update">
  plugin marketplace update
</h3>

Segarkan satu marketplace, atau setiap marketplace, dari sumbernya untuk mengambil plugin dan versi baru. Marketplace yang ditambahkan dengan cabang atau tag `ref` diperbarui ke commit terbaru dari ref itu, bukan cabang default repositori.

```bash theme={null}
claude plugin marketplace update [name]
```

Perintah tidak mengambil flag di luar `--help`.

Segarkan satu marketplace:

```bash theme={null}
claude plugin marketplace update your-marketplace
```

Claude Code mencetak `Successfully updated marketplace: your-marketplace`. Ketika Anda menghilangkan nama, mencetak hitungan seperti `Successfully updated 2 marketplaces`. Tanpa marketplace yang ditambahkan, mencetak `No marketplaces configured` dan keluar `0`.

<h2 id="plugin-in-a-session">
  /plugin dalam sesi
</h2>

Di dalam sesi interaktif, `/plugin` membuka panel plugin. Setiap subperintah membuka panel di tab, menjalankan tindakan di sana, atau mencetak hasil inline. `/plugins` dan `/marketplace` adalah alias untuk `/plugin`.

Anda hanya dapat menjalankan perintah ini dalam sesi terminal interaktif. Dalam run non-interaktif seperti `claude -p`, Claude Code menjawab bahwa `/plugin` tidak tersedia di lingkungan ini.

Untuk permukaan mana yang memiliki `/plugin`, cara menginstal tanpanya, dan apa yang ditunjukkan setiap tab panel, lihat [Instal dan kelola plugin](/docs/id/plugins/install).

`<plugin>` adalah plugin `name` atau `name@marketplace`.

Tabel di bawah mencantumkan setiap bentuk sesi. Subperintah shell `init`, `update`, `details`, `prune`, `eval`, dan `eval init` tidak memiliki bentuk sesi.

| Perintah                                            | Alias                                          | Apa yang dilakukan                                                                                                                                                                                                                                                                                                                  |
| :-------------------------------------------------- | :--------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `/plugin`                                           |                                                | Membuka panel di tab **Discover**. Kata pertama yang tidak dikenali setelah `/plugin` melakukan hal yang sama                                                                                                                                                                                                                       |
| `/plugin help`                                      | `/plugin --help`, `/plugin -h`                 | Menampilkan daftar penggunaan subperintah `/plugin`                                                                                                                                                                                                                                                                                 |
| `/plugin list [--enabled\|--disabled]`              | `ls`                                           | Mencetak plugin yang diinstal marketplace Anda inline, dengan versi, cakupan, dan status. Flag filter menampilkan hanya status itu. Plugin yang status aktifnya belum diterapkan ditandai `— run /reload-plugins to apply`. Memerlukan Claude Code v2.1.163 atau lebih baru                                                         |
| `/plugin install`                                   | `i`                                            | Membuka tab **Discover**                                                                                                                                                                                                                                                                                                            |
| `/plugin install <plugin>`                          | `i`                                            | Membuka detail plugin di tab **Discover**. Dengan `name@marketplace`, membukanya dalam daftar marketplace itu                                                                                                                                                                                                                       |
| `/plugin install <plugin> --marketplace <source>`   | `i`                                            | Menambahkan marketplace di `<source>` ketika Anda belum menambahkannya, meminta Anda mengonfirmasi terlebih dahulu, kemudian membuka detail plugin. Lihat [Tambahkan marketplace dan instal dalam satu perintah](/docs/id/plugins/install#add-a-marketplace-and-install-in-one-command). Memerlukan Claude Code v2.1.275 atau lebih baru |
| `/plugin manage`                                    |                                                | Membuka tab **Installed**                                                                                                                                                                                                                                                                                                           |
| `/plugin stats`                                     |                                                | Membuka tab **Stats**, dalam sesi di mana [`/skill-doctor`](/docs/id/skills#find-unused-skills) tersedia. Di tempat lain membuka panel di tab **Discover**                                                                                                                                                                               |
| `/plugin enable <plugin>`                           |                                                | Membuka tab **Installed** di plugin dan mengaktifkannya                                                                                                                                                                                                                                                                             |
| `/plugin disable <plugin>`                          |                                                | Membuka tab **Installed** di plugin dan menonaktifkannya                                                                                                                                                                                                                                                                            |
| `/plugin uninstall <plugin>`                        |                                                | Membuka tab **Installed** di plugin dan mencopot                                                                                                                                                                                                                                                                                    |
| `/plugin configure <plugin>`                        | `config`                                       | Membuka dialog [`userConfig`](/docs/id/plugins/manifest-reference) plugin, atau melaporkan bahwa plugin tidak mendeklarasikan. Memerlukan Claude Code v2.1.147 atau lebih baru                                                                                                                                                           |
| `/plugin validate <path>`                           |                                                | Mencetak laporan yang sama seperti `claude plugin validate`, inline                                                                                                                                                                                                                                                                 |
| `/plugin tag [path] [--push] [--dry-run] [--force]` |                                                | Membuat tag rilis sebagai `claude plugin tag` lakukan. Menerima `--push`, `--dry-run`, dan `--force` atau `-f`; dengan flag lain atau argumen ekstra, Claude Code mencetak penggunaan                                                                                                                                               |
| `/plugin marketplace`                               | `market`                                       | Tidak melakukan apa pun yang terlihat. Lewatkan `add`, `list`, `update`, atau `remove`                                                                                                                                                                                                                                              |
| `/plugin marketplace add [source]`                  | `market add`                                   | Dengan sumber, menambahkannya dan melaporkan hasil. Tanpanya, membuka input **Add marketplace**                                                                                                                                                                                                                                     |
| `/plugin marketplace list`                          | `market list`                                  | Mencetak nama marketplace Anda inline                                                                                                                                                                                                                                                                                               |
| `/plugin marketplace update [name]`                 | `market update`                                | Membuka tab **Marketplaces**. Dengan nama, menyegarkan marketplace itu di sana                                                                                                                                                                                                                                                      |
| `/plugin marketplace remove [name]`                 | `market remove`, `market rm`, `marketplace rm` | Membuka tab **Marketplaces**. Dengan nama, menghapus marketplace itu di sana                                                                                                                                                                                                                                                        |

Jika Anda menamai plugin yang tidak diinstal dalam proyek saat ini dalam `/plugin enable`, `disable`, `uninstall`, atau `configure`, Claude Code mencetak `Plugin "<plugin>" is not installed in this project` alih-alih bertindak.

<h2 id="reload-plugins">
  /reload-plugins
</h2>

Terapkan perubahan plugin yang tertunda ke sesi yang sedang berjalan tanpa memulai ulang. Perubahan yang tertunda adalah plugin yang Anda instal, perbarui, aktifkan, nonaktifkan, atau edit di disk sejak sesi dimulai.

Ketika Anda menutup panel `/plugin` dengan perubahan yang tertunda yang Anda buat di dalamnya, Claude Code menjalankan `/reload-plugins` untuk Anda. Jalankan sendiri setelah perubahan plugin yang terjadi di luar panel, seperti perintah `claude plugin` yang Anda jalankan di terminal lain.

```text theme={null}
/reload-plugins [--force]
```

| Flag      | Deskripsi                                                                                      |
| :-------- | :--------------------------------------------------------------------------------------------- |
| `--force` | Terapkan reload bahkan ketika akan membatalkan cache prompt. `force` tanpa dash juga berfungsi |

<h3 id="reload-summary">
  Ringkasan reload
</h3>

Claude Code memuat ulang setiap plugin aktif dan mencetak satu baris ringkasan, `Reloaded: N plugins · N skills · N agents · N hooks · N plugin MCP servers · N plugin LSP servers`, menghilangkan hitungan server MCP plugin dalam sesi tanpa terminal interaktif. Ketika plugin apa pun gagal, ringkasan menambahkan `N errors during load. Run /plugin for details.`

Hitungan skills mencakup setiap skill yang disediakan plugin, baik entri `commands/` maupun skill `SKILL.md`. Hitungan agents adalah jumlah agen yang dimuat dalam sesi, termasuk yang tidak berasal dari plugin.

Ketika [dependensi](/docs/id/plugins/dependencies) plugin yang dimuat ulang hilang, Claude Code menginstalnya, memuat ulang lagi, dan menambahkan `(+ N dependencies: <names>) resolved` ke ringkasan.

<h3 id="reloads-that-change-mcp-tools">
  Reload yang mengubah alat MCP
</h3>

Ketika reload akan menambah atau menghapus server MCP plugin atau alat `LSP`, dan perubahan itu akan membatalkan [cache prompt](/docs/id/prompt-caching#enabling-or-disabling-a-plugin), Claude Code tidak menerapkan reload. Mencetak baris seperti `This reload changes MCP tools (<server>) — your next message will re-read the whole conversation instead of using the cache. Run /reload-plugins --force to apply.` Lewatkan `--force` untuk menerapkannya.

<h3 id="sessions-without-an-interactive-terminal">
  Sesi tanpa terminal interaktif
</h3>

`/reload-plugins` juga berjalan dalam sesi tanpa terminal interaktif, seperti aplikasi desktop, Agent SDK, dan [mode non-interaktif](/docs/id/headless) dengan `-p`. Memerlukan Claude Code v2.1.260 atau lebih baru.

Dalam sesi itu, perintah berjalan hanya ketika Anda mengetiknya ke dalam sesi sendiri, seperti dalam prompt `-p` atau kotak prompt aplikasi desktop. Ketika tiba dengan cara lain, seperti melalui [Remote Control](/docs/id/remote-control) atau pesan yang disalurkan dari Slack, perintah menjawab `/reload-plugins isn't available over a remote connection in this session.` dan memuat ulang tidak ada.

Reload dalam sesi itu tidak menghubungkan atau memutuskan server MCP plugin. Perubahan itu berlaku dalam sesi berikutnya Anda.

<h2 id="flags-that-load-a-plugin-for-one-session">
  Flag yang memuat plugin untuk satu sesi
</h2>

Dua flag `claude` memuat plugin untuk satu sesi saja, tanpa menginstalnya. Keduanya dapat diulang.

Penulis plugin menggunakannya untuk menguji plugin sebelum menerbitkan. Untuk alur kerja muat-edit-muat ulang, lihat [Kembangkan tanpa marketplace](/docs/id/plugins/create#develop-without-a-marketplace).

| Flag                  | Deskripsi                                                                                                                                                                  | Contoh                                                                      |
| :-------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------- |
| `--plugin-dir <path>` | Muat plugin dari direktori atau arsip `.zip` darinya. Folder plugin memuat setiap folder anak yang memegang `.claude-plugin/plugin.json`. Setiap flag mengambil satu jalur | `claude --plugin-dir ./my-plugin --plugin-dir ./other.zip`                  |
| `--plugin-url <url>`  | Ambil arsip plugin `.zip` dari URL. Ulangi flag, atau lewatkan beberapa URL yang dipisahkan spasi dalam satu nilai yang dikutip                                            | `claude --plugin-url "https://example.com/a.zip https://example.com/b.zip"` |

Plugin yang dimuat flag apa pun adalah plugin hanya sesi. `claude plugin list` menampilkannya sebagai `<name>@inline` dengan cakupan `session`, tetapi hanya ketika flag yang sama mendahului subperintah. Misalnya, jalankan `claude --plugin-dir ./my-plugin plugin list`.

Ketika plugin hanya sesi berbagi nama dengan plugin yang diinstal, Claude Code memuat salinan hanya sesi untuk sesi itu dan melewati yang diinstal. Salinan yang diinstal memuat sebagai gantinya jika Anda menonaktifkan salinan hanya sesi dengan `claude plugin disable <name>@inline`, atau jika pengaturan terkelola mengunci nama plugin itu. Untuk prioritas, lihat [Referensi pemuatan plugin](/docs/id/plugins/loading).

Administrator dapat menolak kedua flag, dan folder yang dinamai dalam variabel [`CLAUDE_CODE_PLUGIN_DIRS`](/docs/id/env-vars#variables), dengan pengaturan terkelola [`disableSideloadFlags`](/docs/id/settings-reference#disablesideloadflags). Claude Code kemudian mencetak bahwa flag dinonaktifkan oleh pengaturan terkelola organisasi Anda dan keluar `1` tanpa memulai.

Dari Agent SDK, opsi [`plugins`](/docs/id/agent-sdk/plugins) adalah setara dengan `--plugin-dir`.

<h2 id="next-steps">
  Langkah berikutnya
</h2>

* [Instal dan kelola plugin](/docs/id/plugins/install): operasi yang sama seperti langkah, dengan apa yang Anda lihat di setiap
* [Referensi pemuatan plugin](/docs/id/plugins/loading): apa yang diubah setiap perintah di disk dan cakupan mana yang berlaku
* [Troubleshoot plugin](/docs/id/plugins/troubleshooting): pesan kesalahan instal, marketplace, muat, dan validasi dengan perbaikannya
* [Referensi manifest plugin](/docs/id/plugins/manifest-reference): field yang diperiksa `claude plugin validate`
