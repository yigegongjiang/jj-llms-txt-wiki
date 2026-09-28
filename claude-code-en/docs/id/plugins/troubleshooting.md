> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Troubleshoot plugins

> Perbaiki kesalahan plugin di Claude Code. Temukan pesan yang tepat yang Anda lihat, dikelompokkan berdasarkan tahap dari mana /plugin berjalan melalui instalasi dan kebijakan organisasi.

Halaman ini mencantumkan pesan kesalahan dan gejala untuk plugin Claude Code dan untuk marketplace, katalog tempat Claude Code menginstal plugin. Setiap entri memberikan penyebab, satu perbaikan, dan apa yang Anda lihat setelah perbaikan berhasil.

Jika pesan menyebutkan plugin atau marketplace, entri menunjukkan placeholder seperti `<name>` sebagai gantinya.

Gunakan halaman ini apakah Anda menginstal plugin, membangunnya, menyelenggarakan marketplace, atau mengelola plugin untuk organisasi.

<Note>
  Kasus-kasus ini tercakup di halaman lain:

  * **Mengapa scopes, cache, dan precedence berperilaku seperti yang mereka lakukan**: baca [Plugin loading reference](/docs/id/plugins/loading)
  * **Mencari flag, field, atau command**: gunakan [plugin commands reference](/docs/id/plugins/cli-reference), [manifest reference](/docs/id/plugins/manifest-reference), atau [marketplace reference](/docs/id/plugins/marketplace-reference)
</Note>

Cari pesan yang tepat yang Anda lihat. Setiap pesan tercantum di bawah tahap yang menghasilkannya, yang tidak selalu merupakan perintah yang Anda jalankan. Misalnya, instalasi dapat gagal karena marketplace hilang, jadi pesan itu berada di bawah [Add a marketplace](#add-a-marketplace).

<h2 id="find-where-/plugin-runs">
  Find where `/plugin` runs
</h2>

`/plugin` adalah command yang Anda ketik di dalam sesi terminal Claude Code yang sedang berjalan, dan itu membuka panel interaktif. Entri di bagian ini mencakup tempat-tempat di mana Anda dapat mengetiknya tetapi tidak dapat berjalan, dan spelling command yang tidak ada.

<h3 id="plugin-isnt-available-in-this-environment">
  `/plugin isn't available in this environment`
</h3>

Anda mengetik `/plugin` di tempat lain selain sesi terminal Claude Code, dan Claude menjawab dengan baris ini alih-alih membuka apa pun.

Anda mendapatkan balasan ini dalam sesi yang tidak memiliki terminal untuk menggambar panel `/plugin` di dalamnya: [non-interactive mode](/docs/id/headless) dengan `claude -p`, Agent SDK, tab Code aplikasi desktop Claude, panel ekstensi VS Code, dan browser di claude.ai/code.

Di panel ekstensi VS Code, hanya baris `/plugin` dengan sesuatu setelahnya, seperti `/plugin install <plugin>@<marketplace>`, yang mendapatkan balasan ini. `/plugin` atau `/plugins` yang diketik sendiri membuka dialog **Manage plugins**.

Instal plugin dari permukaan yang Anda gunakan:

* **Aplikasi desktop Claude, sesi lokal atau SSH**: klik tombol **+** di sebelah prompt, lalu **Plugins**, lalu **Add plugin** untuk membuka [plugin browser](/docs/id/desktop#install-plugins)
* **Ekstensi VS Code**: gunakan tab **VS Code** di bawah [Install a plugin](/docs/id/plugins/install#install-a-plugin)
* **Claude Code di web, atau sesi cloud desktop**: sesi cloud tidak memiliki plugin browser. Lihat tab **Cloud session** di bawah [Install a plugin](/docs/id/plugins/install#install-a-plugin) untuk apa yang dimuat sesi cloud
* **Terminal yang Anda miliki akses**: jalankan `claude` dan ketik `/plugin` di sana, atau jalankan `claude plugin install <plugin>@<marketplace>` di shell Anda tanpa memulai sesi

Ketika instalasi terminal berhasil, `/plugin` mencetak ringkasan instalasi yang dimulai dengan `✓ Installed <plugin>.` dan `claude plugin install` mencetak `Successfully installed plugin: <plugin>@<marketplace>`.

<h3 id="zsh-no-such-file-or-directory-plugin">
  `zsh: no such file or directory: /plugin`
</h3>

Anda mengetik `/plugin ...` di prompt shell, dan shell melaporkan bahwa tidak ada file bernama `/plugin`. Bash melaporkan `bash: /plugin: No such file or directory`.

`/plugin` adalah command yang Anda ketik di dalam sesi Claude Code, bukan di prompt shell. Mulai sesi dan ketik command yang sama di sana:

```shell theme={null}
claude
```

Kemudian, di prompt Claude Code:

```text theme={null}
/plugin install <plugin>@<marketplace>
```

Instalasi yang berhasil mencetak ringkasan yang dimulai dengan `✓ Installed <plugin>.` Jika instalasi itu sendiri kemudian gagal, pesannya ada di bawah [Add a marketplace](#add-a-marketplace) atau [Install a plugin](#install-a-plugin).

Untuk menginstal dari shell tanpa memulai sesi, jalankan `claude plugin install <plugin>@<marketplace>` sebagai gantinya.

<h3 id="the-term-plugin-is-not-recognized-as-the-name-of-a-cmdlet">
  `The term '/plugin' is not recognized as the name of a cmdlet`
</h3>

Anda mengetik `/plugin ...` di prompt PowerShell, dan `/plugin` adalah command Claude Code, bukan program. Bash dan Zsh melaporkan [bentuk kesalahan mereka sendiri](#zsh-no-such-file-or-directory-plugin).

Gunakan salah satu dari ini:

* Jalankan `claude`, lalu ketik `/plugin` di prompt Claude Code
* Jalankan `claude plugin install <plugin>@<marketplace>` di PowerShell tanpa memulai sesi

<h3 id="claude-command-not-found-after-claude-plugin">
  `claude: command not found` after `claude plugin ...`
</h3>

Anda menjalankan `claude plugin install ...` di shell Anda, dan shell tidak dapat menemukan `claude` sama sekali. Di Windows pesannya adalah `'claude' is not recognized as the name of a cmdlet` atau `'claude' is not recognized as an internal or external command`.

Penyebabnya bukan command plugin. Baik Claude Code tidak terinstal, atau direktori instalasinya tidak ada di `PATH` Anda di shell ini. Ikuti [`command not found: claude` after installation](/docs/id/troubleshoot-install#command-not-found-claude-after-installation), lalu coba ulang command plugin.

<h3 id="unknown-command-and-command-spellings-that-dont-exist">
  `Unknown command` and command spellings that don't exist
</h3>

Anda mengetik command plugin yang Anda lihat di suatu tempat dan mendapatkan `Unknown command: /<name>` dalam sesi, atau `error: unknown command '<name>'` atau `error: unknown option '<flag>'` dari binary `claude` di shell Anda.

Beberapa spelling command sedang digunakan yang Claude Code tidak miliki. Tabel di bawah memetakan masing-masing ke command yang sebenarnya. [Plugin commands reference](/docs/id/plugins/cli-reference) mencantumkan setiap subcommand dan flag.

| Anda mengetik                              | Apa yang Claude Code katakan                                                 | Gunakan sebagai gantinya                                                                                                                            |
| :----------------------------------------- | :--------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------- |
| `claude plugin add <source>`               | `error: unknown command 'add'`                                               | `claude plugin marketplace add <source>` untuk menambahkan marketplace, atau `claude plugin install <plugin>@<marketplace>` untuk menginstal plugin |
| `claude plugin install <plugin> --project` | `error: unknown option '--project'`                                          | `claude plugin install <plugin>@<marketplace> --scope project`                                                                                      |
| `/install <plugin>`                        | `Unknown command: /install`                                                  | `/plugin install <plugin>@<marketplace>`                                                                                                            |
| `/plugin add <source>`                     | Panel `/plugin` terbuka di tab **Discover**                                  | `/plugin marketplace add <source>`                                                                                                                  |
| `marketplace.anthropic.com` sebagai sumber | `Invalid marketplace source format. Try: owner/repo, https://..., or ./path` | `anthropics/claude-plugins-official` untuk marketplace resmi                                                                                        |

Spelling ini terlihat salah tetapi berfungsi:

* `claude plugins` adalah alias dari `claude plugin`
* `claude plugin remove` adalah alias dari `claude plugin uninstall`
* `/plugins` dan `/marketplace` dalam sesi membuka panel yang sama seperti `/plugin`

<h2 id="add-a-marketplace">
  Add a marketplace
</h2>

Marketplace adalah katalog yang Anda tambahkan ke Claude Code dari repositori git, URL, atau path lokal. Entri ini mencakup pesan yang Anda dapatkan ketika menambahkan satu gagal atau refresh kemudian gagal.

<h3 id="marketplace-claude-plugins-official-not-found">
  `Marketplace "claude-plugins-official" not found`
</h3>

Anda menjalankan `/plugin install <plugin>@claude-plugins-official` dalam sesi, dan Claude Code melaporkan bahwa tidak memiliki marketplace dengan nama itu.

Marketplace resmi belum terdaftar di mesin ini. Claude Code biasanya mendaftarkannya sendiri pertama kali Anda memulai sesi terminal interaktif. Belum berjalan jika Anda hanya menggunakan Claude Code melalui ekstensi VS Code, dan itu melewati atau menunda langkah itu:

* Ketika kebijakan memblokir sumber
* Ketika `CLAUDE_CODE_DISABLE_OFFICIAL_MARKETPLACE_AUTOINSTALL` diatur
* Setelah upaya yang gagal yang menunggu untuk mencoba ulang

Command shell `claude plugin` tidak pernah mendaftarkannya untuk Anda.

Tambahkan, lalu coba ulang instalasi:

```text theme={null}
/plugin marketplace add anthropics/claude-plugins-official
```

Claude Code mencetak `Successfully added marketplace: claude-plugins-official`, dan `/plugin marketplace list` menunjukkan marketplace dengan sumbernya.

Untuk nama marketplace lain dalam pesan ini, lihat [`Marketplace "<name>" not found`](#marketplace-not-found).

String yang sama juga muncul di tab **Errors** `/plugin`, daftar kegagalan beban panel, ketika plugin yang tercantum dalam pengaturan Anda menyebutkan marketplace yang belum Anda tambahkan.

<h3 id="marketplace-not-found">
  `Marketplace "<name>" not found`
</h3>

Anda menjalankan `/plugin install <plugin>@<name>` dalam sesi, sering kali dari baris instalasi yang dikirim seseorang kepada Anda, dan Claude Code melaporkan bahwa tidak memiliki marketplace dengan nama itu.

Jika nama dimulai dengan `claudeai-`, marketplace dihosting di claude.ai, dan Anda menambahkannya berdasarkan nama dari shell Anda dengan `claude plugin marketplace add --claudeai <name>`. Lihat [Add a marketplace from claude.ai](/docs/id/plugins/install#add-from-claude-ai).

Untuk nama lain, baris instalasi menyebutkan marketplace tetapi tidak mengatakan di mana marketplace dihosting, dan Claude Code tidak memiliki indeks untuk mencari nama marketplace. Tanyakan kepada siapa pun yang mengirim baris untuk sumber marketplace, yang merupakan GitHub `owner/repo`, URL git, atau path. Kemudian [tambahkan marketplace](/docs/id/plugins/install#add-a-marketplace) dan jalankan baris instalasi lagi.

Marketplace yang dikirim seseorang kepada Anda adalah pihak ketiga, jadi [tinjau plugin sebelum Anda menginstalnya](/docs/id/plugins/security#review-a-plugin-before-you-install).

Jika Anda sudah menambahkan marketplace, periksa spelling terhadap `/plugin marketplace list`.

<h3 id="invalid-marketplace-source-format">
  `Invalid marketplace source format`
</h3>

Anda menjalankan `/plugin marketplace add <source>` atau `claude plugin marketplace add <source>`, dan Claude Code menjawab `Invalid marketplace source format. Try: owner/repo, https://..., or ./path`.

Claude Code menerima sumber dalam salah satu bentuk ini:

* Shorthand GitHub `owner/repo`
* URL `https://` atau `http://`
* URL SSH `user@host:path`
* Path lokal dimulai dengan `./`, `../`, `/`, atau `~`

Nama telanjang seperti `claude-plugins-official` tidak cocok dengan salah satu dari mereka. Begitu juga hostname telanjang seperti `marketplace.anthropic.com`.

Ketik ulang sumber dalam salah satu bentuk yang diterima:

```text theme={null}
/plugin marketplace add anthropics/claude-plugins-official
```

Claude Code mencetak `Successfully added marketplace: <name>` ketika penambahan berhasil.

<h3 id="is-not-a-valid-github-owner-repo-shorthand">
  `'<source>' is not a valid GitHub owner/repo shorthand`
</h3>

Anda melewatkan sumber dengan garis miring yang bukan `owner/repo`, seperti `github.com/owner/repo` atau path `gitlab.example.com/group/project`. Claude Code menolaknya dengan pesan ini dan daftar bentuk yang diterima.

Shorthand `owner/repo` hanya untuk GitHub dan harus mengikuti aturan penamaan GitHub, jadi hostname atau segmen path tambahan gagal. Lewatkan sumber dalam bentuk yang cocok dengan tempat marketplace dihosting:

* **Repository di host apa pun**: URL klon lengkap
* **`marketplace.json` yang dihosting**: URL `https://` nya
* **Checkout lokal**: `./path` atau path absolut

Misalnya, untuk menambahkan marketplace resmi dengan URL klon-nya, dalam sesi:

```text theme={null}
/plugin marketplace add https://github.com/anthropics/claude-plugins-official.git
```

Penambahan yang berhasil mencetak `Successfully added marketplace: <name>`.

<h3 id="path-does-not-exist">
  `Path does not exist: <path>`
</h3>

Anda melewatkan path lokal ke `marketplace add`, dan tidak ada yang ada di path itu. Path relatif diselesaikan terhadap direktori saat ini Anda.

Periksa path yang diselesaikan dalam pesan. Kemudian jalankan command dari direktori path relatif dimulai, atau lewatkan path absolut ke direktori marketplace. Penambahan yang berhasil mencetak `Successfully added marketplace: <name>`.

Claude Code menerima direktori yang berisi `.claude-plugin/marketplace.json`, atau path ke file `.json`. Path ke file lain apa pun gagal dengan `File path must point to a .json file (marketplace.json)`.

<h3 id="marketplace-file-not-found-at-claude-plugin-marketplace-json">
  `Marketplace file not found at <path>/.claude-plugin/marketplace.json`
</h3>

Claude Code mengklon atau mengunduh marketplace tetapi tidak menemukan `marketplace.json` di path yang diharapkan di dalamnya. Command add melaporkannya sebagai `Failed to add marketplace: Marketplace file not found at ...`.

Lokasi default adalah `.claude-plugin/marketplace.json` di root repository, dan [marketplace reference](/docs/id/plugins/marketplace-reference) mencantumkan lokasi yang diterima.

Perbaikannya berbeda untuk pemilik dan untuk semua orang:

* **Anda memiliki marketplace**: letakkan file di lokasi itu dan tambahkan kembali marketplace
* **Seseorang lain menyelenggarakannya**: tanyakan kepada pemilik untuk sumber yang tepat yang mereka terbitkan

<h3 id="ssh-authentication-failed-or-https-authentication-failed">
  `SSH authentication failed` or `HTTPS authentication failed`
</h3>

Anda menambahkan atau memperbarui marketplace dari repositori git, dan klon gagal dengan `Failed to clone marketplace repository:` diikuti oleh salah satu baris ini.

Pertama periksa repository itu sendiri: `owner/repo` yang salah eja, repository yang tidak ada, atau repository pribadi yang tidak dapat Anda lihat juga berakhir dalam pesan ini. Buka URL repository di browser Anda, atau jalankan `git ls-remote <url>` di terminal Anda, untuk mengonfirmasi bahwa itu ada dan Anda memiliki akses.

Jika repository benar, penyebabnya adalah kredensial. Claude Code menjalankan git dengan prompt interaktif dinonaktifkan, jadi tidak dapat meminta Anda untuk password, passphrase kunci, atau kredensial seperti yang dilakukan terminal Anda. Jika git perlu meminta, Anda melihat `fatal: Cannot prompt because user interactivity has been disabled` atau `terminal prompts disabled` dalam kesalahan asli. Hanya kredensial yang sudah berfungsi non-interaktif yang berhasil:

* **SSH**: `ssh -T git@<host>` harus berhasil tanpa meminta passphrase, dan host harus sudah ada di `known_hosts`
* **HTTPS**: credential helper Anda harus menyimpan token untuk host. Untuk GitHub, jalankan `gh auth login` dan `gh auth setup-git`. Untuk host lain, simpan personal access token di git credential helper Anda. Uji dengan `git ls-remote <url>`

Setelah `git ls-remote` berhasil di terminal Anda tanpa prompt, jalankan add atau update lagi. Penambahan yang berhasil mencetak `Successfully added marketplace: <name>`. Update yang berhasil mencetak `Successfully updated marketplace: <name>` dari shell Anda, atau `✔ Updated 1 marketplace` dalam sesi.

Untuk membuat Claude Code melewati SSH untuk sumber GitHub `owner/repo`, atur `CLAUDE_CODE_PLUGIN_PREFER_HTTPS=1`. Tanpanya, Claude Code mengklon sumber-sumber itu melalui SSH ketika kunci SSH untuk `github.com` terlihat dikonfigurasi, dan kembali ke HTTPS ketika klon SSH gagal.

Untuk apa yang dapat dan tidak dapat dilakukan auto-update latar belakang dengan kredensial Anda, lihat [What background auto-update does with credentials](/docs/id/plugins/host-marketplace#what-background-auto-update-does-with-credentials).

<h3 id="ssh-host-key-is-not-in-your-known-hosts-file">
  `SSH host key is not in your known_hosts file`
</h3>

Anda menambahkan marketplace melalui SSH dari host yang belum pernah Anda hubungkan, dan klon gagal dengan baris ini dan hint `ssh -T git@<host>`. Untuk host yang kuncinya berubah, pesannya adalah `SSH host key has changed` dengan hint `ssh-keygen -R <host>` sebagai gantinya.

Claude Code mengklon dengan `StrictHostKeyChecking=yes`, jadi itu menolak host yang kuncinya belum Anda terima daripada menerima kunci secara otomatis. Hubungkan sekali dari terminal Anda untuk menerima fingerprint, lalu coba ulang:

```shell theme={null}
ssh -T git@github.com
```

Untuk repository publik, tambahkan marketplace dengan URL `https://` nya sebagai gantinya untuk menghindari SSH sepenuhnya.

<h3 id="command-git-not-found-or-is-in-an-unsafe-location">
  `Command 'git' not found or is in an unsafe location`
</h3>

Di Windows, Anda menambahkan marketplace dan Claude Code melaporkan `Failed to clone marketplace repository: Command 'git' not found or is in an unsafe location (current directory)`.

Claude Code mencari `git` di `PATH` Anda dan menolak untuk menjalankan yang ditemukan hanya di direktori saat ini. Untuk memperbaikinya, instal Git dan coba ulang:

<Steps>
  <Step title="Install Git for Windows">
    Instal Git untuk Windows sehingga `git` ada di `PATH` Anda.
  </Step>

  <Step title="Open a new terminal">
    Buka terminal baru sehingga `PATH` yang diperbarui berlaku.
  </Step>

  <Step title="Confirm git runs">
    Konfirmasi `git --version` mencetak versi.
  </Step>

  <Step title="Retry the add">
    Jalankan command `marketplace add` lagi.
  </Step>
</Steps>

<h3 id="git-clone-timed-out-after-120s">
  `Git clone timed out after 120s`
</h3>

Anda menambahkan atau memperbarui marketplace, dan itu gagal dengan `Git clone timed out after 120s`, diikuti oleh hint untuk mengatur `CLAUDE_CODE_PLUGIN_GIT_TIMEOUT_MS`.

Mengklon marketplace, dan mengklon ulang satu untuk memperbaruinya, mendapat 120 detik secara default. Untuk repository besar atau koneksi lambat, naikkan batasnya. Nilainya dalam milidetik:

<Tabs>
  <Tab title="Bash or Zsh">
    ```bash theme={null}
    export CLAUDE_CODE_PLUGIN_GIT_TIMEOUT_MS=300000
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    $env:CLAUDE_CODE_PLUGIN_GIT_TIMEOUT_MS = "300000"
    ```
  </Tab>
</Tabs>

Kemudian coba ulang di shell yang sama.

Jika repository adalah monorepo, batasi checkout ke direktori yang Anda beri nama dengan `claude plugin marketplace add <source> --sparse <paths>`.

<h3 id="marketplace-updates-keep-failing-offline">
  Marketplace updates keep failing offline
</h3>

Anda bekerja di lingkungan di mana host git marketplace tidak dapat dijangkau, dan setiap sesi mengulangi refresh yang gagal di latar belakang. Checkout marketplace Anda yang ada tetap di tempat dan startup tidak tertunda.

Setiap sesi, untuk marketplace dengan [auto-update on](/docs/id/plugins/loading#which-marketplaces-and-plugins-auto-update), Claude Code memeriksa host git marketplace untuk commit baru di latar belakang. Ketika pemeriksaan itu tidak dapat menjangkau host, itu mencoba mengklon marketplace lagi, dan offline klon itu juga gagal.

Atur variabel ini untuk melewati upaya re-clone dan terus menggunakan checkout yang ada ketika pemeriksaan tidak dapat menjangkau host:

<Tabs>
  <Tab title="Bash or Zsh">
    ```bash theme={null}
    export CLAUDE_CODE_PLUGIN_KEEP_MARKETPLACE_ON_FAILURE=1
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    $env:CLAUDE_CODE_PLUGIN_KEEP_MARKETPLACE_ON_FAILURE = "1"
    ```
  </Tab>
</Tabs>

Dengan variabel yang diatur, Claude Code melewati re-clone hanya untuk checkout yang sudah berisi `.claude-plugin/marketplace.json`. Marketplace yang tidak pernah diklon atau yang klonnya berhenti di tengah jalan masih mendapat upaya klon, jadi tambahkan sekali saat online.

Untuk deployment yang sepenuhnya offline, pre-populate direktori plugins pada waktu build gambar dengan `CLAUDE_CODE_PLUGIN_SEED_DIR` sebagai gantinya, mengikuti [Seed containers and CI](/docs/id/plugins/org#seed-containers-and-ci).

<h3 id="marketplace-add-fails-on-a-github-enterprise-server-host">
  Marketplace add fails on a GitHub Enterprise Server host
</h3>

Anda menambahkan marketplace dari URL GitHub Enterprise Server (GHES) dan mendapatkan kesalahan kebijakan, atau Anda menambahkannya dari claude.ai dan mendapatkan kesalahan akses GitHub.

Kedua kasus ada di halaman GHES:

* [Kesalahan kebijakan](/docs/id/github-enterprise-server#marketplace-add-fails-with-a-policy-error) berarti organisasi Anda membatasi sumber marketplace dan admin perlu menambahkan `hostPattern` untuk host
* [Kesalahan akses GitHub di claude.ai](/docs/id/github-enterprise-server#marketplace-add-on-claude-ai-fails-with-a-github-access-error) berarti akun GitHub Enterprise Anda sendiri belum terhubung

<h2 id="install-a-plugin">
  Install a plugin
</h2>

Anda menambahkan marketplace dan menjalankan instalasi, dan instalasi berhenti dengan pesan alih-alih menginstal apa pun. Entri ini mencakup pesan-pesan itu. Mereka juga mencakup pesan terkait yang muncul kemudian di tab **Errors** `/plugin`, atau sebagai tab **Discover** kosong, ketika plugin atau marketplace-nya tidak dapat ditemukan, dibaca, atau dipercaya.

<h3 id="plugin-not-found-in-marketplace">
  `Plugin "<name>" not found in marketplace "<marketplace>"`
</h3>

Anda menjalankan `/plugin install <name>@<marketplace>` atau `claude plugin install <name>@<marketplace>`, dan nama plugin tidak ada dalam salinan katalog marketplace itu di mesin Anda.

`claude plugin install` di shell Anda mencetak pesan yang sama ketika Anda belum menambahkan marketplace sama sekali. Jika `claude plugin marketplace update <marketplace>` kemudian menjawab `Marketplace '<marketplace>' not found`, [tambahkan marketplace](#add-a-marketplace) terlebih dahulu.

<h4 id="the-message-ends-with-a-refresh-hint">
  `not found in marketplace` with a refresh hint
</h4>

Hint berbunyi `Your local copy may be out of date — try claude plugin marketplace update <marketplace>` atau `The marketplace couldn't be refreshed (...)`. Claude Code tidak merefresh marketplace sebelum pencarian, seperti ketika Anda offline, jadi salinan katalog Anda mungkin sudah ketinggalan zaman. Refresh dengan nama marketplace, lalu instal lagi:

```text theme={null}
/plugin marketplace update <marketplace>
```

`claude plugin marketplace update` mencetak `Successfully updated marketplace: <name>`, dan `/plugin marketplace update` menunjukkan `✔ Updated 1 marketplace`. Jika instalasi yang dicoba ulang mencetak pesan yang sama, periksa nama seperti yang dijelaskan [`not found in marketplace` with no hint](#the-message-has-no-hint). [When Claude Code refreshes a marketplace before an install](/docs/id/plugins/loading#when-claude-code-refreshes-a-marketplace-before-an-install) mencantumkan kasus lain di mana refresh tidak berjalan.

<h4 id="the-message-has-no-hint">
  `not found in marketplace` with no hint
</h4>

Nama adalah masalah yang paling mungkin. Buka `/plugin`, buka **Discover**, dan salin nama dari daftar.

Sebelum v2.1.232, Claude Code merefresh marketplace yang dinamai hanya setelah pencarian terlewat, dan hanya ketika auto-update aktif untuk itu.

<h3 id="plugin-not-found-in-any-marketplace">
  `Plugin "<name>" not found in any marketplace`
</h3>

Anda menjalankan `/plugin install <name>` tanpa `@marketplace`, dan tidak ada marketplace terdaftar yang memiliki plugin itu. `claude plugin install <name>` melaporkan `Plugin "<name>" not found in any configured marketplace`.

Tanpa nama marketplace, `claude plugin install` mencari katalog yang sudah dimilikinya dan tidak merefresh mereka terlebih dahulu, dan `/plugin install` merefresh hanya marketplace yang memiliki auto-update aktif. Beri nama marketplace, dan Claude Code merefresh-nya sebelum mencari plugin:

```text theme={null}
/plugin install <name>@<marketplace>
```

Ketika instalasi berhasil, Anda melihat `✓ Installed <plugin>.` dalam sesi, atau `Successfully installed plugin: <plugin>@<marketplace>` dari `claude plugin install`.

Jika Anda tidak tahu marketplace mana yang mencantumkan plugin, jalankan `/plugin marketplace list` untuk marketplace yang Anda miliki, dan telusuri **Discover** di `/plugin` untuk nama plugin.

<h3 id="plugin-is-already-installed-globally">
  `Plugin '<name>@<marketplace>' is already installed globally`
</h3>

Anda menjalankan `/plugin install` untuk plugin yang sudah diinstal di scope pengguna atau oleh pengaturan terkelola, dan Claude Code menolak dengan `Use '/plugin' to manage existing plugins.` Jika Anda mengetik nama plugin tanpa `@<marketplace>`, pesan menghilangkan `globally`.

Plugin sudah tersedia di setiap proyek, jadi tidak ada yang perlu ditambahkan. Untuk mengubah [scope](/docs/id/plugins/install)-nya, mengaktifkan atau menonaktifkannya, atau mengonfigurasinya, buka `/plugin` dan buka **Installed**.

Plugin yang diinstal hanya di scope proyek atau lokal tidak memicu pesan ini. Claude Code memungkinkan Anda menginstalnya di scope pengguna juga, jadi tersedia di proyek lain.

`claude plugin install` di shell Anda mencetak pesan yang berbeda. Untuk plugin yang sudah diinstal di scope target, itu mencetak `Plugin "<name>@<marketplace>" is already installed (scope: user)` dan keluar 0. Jika direktori cache-nya hilang, command yang sama mengunduh ulang-nya.

<h3 id="this-plugin-uses-a-source-type-your-claude-code-version-does-not-suppo">
  `This plugin uses a source type your Claude Code version does not support`
</h3>

Anda menginstal plugin yang entri marketplace-nya menggunakan tipe sumber yang versi Claude Code ini tidak dapat ambil, dan Claude Code berhenti dengan pesan ini dan `Update Claude Code and try again.`

Perbarui Claude Code, lalu coba ulang instalasi. Tipe sumber ada di [marketplace reference](/docs/id/plugins/marketplace-reference).

<h3 id="plugin-archive-integrity-check-failed">
  `Plugin archive integrity check failed`
</h3>

Anda menginstal plugin yang didistribusikan sebagai arsip zip, dan Claude Code menolaknya dengan baris ini dan `The archive was not installed.` Entri marketplace plugin menggunakan [`archive` source](/docs/id/plugins/marketplace-reference) dengan pin `sha256`, dan digest file yang diunduh tidak cocok dengan pin.

Pesan lengkapnya terlihat seperti ini:

```text theme={null}
Plugin archive integrity check failed for https://artifacts.example.com/claude-plugins/my-plugin.zip: expected sha256 6bfa50e3d2e00c052b46abe51fff89346ac803e45771f76dcf6df1ab74cca5e1, got ac52220c0914ef8ca6a602e4a7362f88d30fb021110f72a6d15b68c3fe7df2b7. The archive was not installed. Verify the sha256 in the marketplace entry, or that the URL serves the intended file.
```

Perbaikannya berbeda untuk penerbit dan penginstal:

* **Anda menerbitkan plugin**: hitung ulang digest file yang tepat yang URL layani dan perbarui `sha256` dalam entri marketplace. Gunakan `shasum -a 256 my-plugin.zip`, atau `Get-FileHash -Algorithm SHA256 my-plugin.zip` di PowerShell
* **Anda menginstal plugin**: jalankan `/plugin marketplace update <name>` dalam sesi untuk merefresh katalog jika entri diperbaiki, lalu coba ulang instalasi. Jika digest masih tidak setuju setelah refresh, tanyakan kepada pemilik marketplace file mana yang mereka pin sebelum menginstal

<h3 id="marketplace-is-registered-from-an-untrusted-source">
  `Marketplace "<name>" is registered from an untrusted source`
</h3>

Marketplace yang Anda tambahkan sebelumnya berhenti memuat, begitu juga plugin-nya. Baris ini muncul di tab **Errors** `/plugin` atau pada refresh berikutnya.

Marketplace terdaftar di bawah nama yang [dicadangkan untuk marketplace resmi Anthropic](/docs/id/plugins/marketplace-reference), tetapi sumber terdaftar-nya bukan repository GitHub `anthropics`. Nama yang dicadangkan diperiksa ulang setiap kali marketplace memuat atau merefresh, jadi marketplace dan plugin yang diinstal darinya berhenti memuat.

Pesan lengkap menyebutkan nama yang dicadangkan dan perbaikannya:

```text theme={null}
Marketplace "claude-community" is registered from an untrusted source: The name 'claude-community' is reserved for official Anthropic marketplaces. Only repositories from 'github.com/anthropics/' can use this name. To fix it, remove the marketplace and re-add it from the official source.
```

Perbaikannya berbeda untuk pengguna dan penerbit:

* **Anda menggunakan marketplace**: di shell Anda, jalankan `claude plugin marketplace remove <name>`, lalu tambahkan marketplace lagi dari repository `github.com/anthropics` resmi
* **Anda menerbitkan marketplace pihak ketiga yang menggunakan nama sebelum itu menjadi dicadangkan**: ganti namanya dan minta pengguna untuk menambahkan kembali dari sumber Anda

Sebelum v2.1.205, Claude Code memeriksa nama hanya ketika Anda menambahkan marketplace, jadi entri yang terdaftar sebelum nama-nya menjadi dicadangkan terus memuat.

<h3 id="plugin-has-a-corrupt-manifest-file-or-has-an-invalid-manifest-file">
  `Plugin <name> has a corrupt manifest file` or `has an invalid manifest file`
</h3>

Claude Code mengambil plugin, lalu gagal membaca `.claude-plugin/plugin.json`-nya. Di shell, `<name>` dalam baris ini dapat berupa nama direktori sementara; awalan `Failed to install plugin "<name>@<marketplace>"` membawa nama asli plugin. Wording mengatakan pemeriksaan mana yang gagal:

* **`corrupt manifest file`, diikuti oleh `JSON parse error:`**: file bukan JSON yang valid
* **`invalid manifest file`, diikuti oleh `Validation errors:`**: file parse tetapi gagal schema, seperti `name: Invalid input` untuk field yang diperlukan yang hilang

`claude plugin install` melaporkan salah satu sebagai `Failed to install plugin "<name>@<marketplace>":` dan keluar dengan kode 1.

Penulis plugin harus memperbaiki file, dan plugin tidak dapat diinstal sampai saat itu:

* **Jika itu Anda**: jalankan `claude plugin validate <plugin-directory>` di shell Anda untuk melihat kesalahan yang sama dengan path yang menyinggung, lalu perbaiki file
* **Jika bukan Anda**: laporkan pesan ke pemilik marketplace

<h3 id="plugin-directory-not-found-at-path">
  `Plugin directory not found at path: <path>`
</h3>

Tab **Errors** di `/plugin` menunjukkan ini untuk plugin yang diaktifkan yang marketplace-nya mencantumkan dengan path relatif, seperti `./plugins/my-plugin`, ketika tidak ada direktori yang ada di path itu di dalam marketplace. Jika Anda memelihara marketplace, perbaiki path `source` entri atau pulihkan folder. Jika tidak, laporkan pesan ke pemilik marketplace.

`Marketplace directory not found at path: <path>` berarti direktori marketplace itu sendiri hilang. Untuk marketplace yang Anda tambahkan dari path lokal, direktori itu pindah atau dihapus. Pulihkan, atau hapus marketplace dan tambahkan lagi dari lokasi barunya.

<h3 id="no-plugins-available-or-no-marketplaces-configured">
  `No plugins available` or `No marketplaces configured`
</h3>

Anda membuka `/plugin` dan tab **Discover** kosong, atau `claude plugin marketplace list` mencetak `No marketplaces configured`.

Tidak ada marketplace yang terdaftar, jadi tidak ada katalog untuk ditampilkan. Dalam sesi, tambahkan marketplace resmi, `anthropics/claude-plugins-official`:

```text theme={null}
/plugin marketplace add anthropics/claude-plugins-official
```

Claude Code mencetak `Successfully added marketplace: claude-plugins-official`, dan **Discover** mencantumkan plugin-nya. Halaman [Anthropic marketplaces](/docs/id/plugins/anthropic-marketplaces) mencantumkan marketplace lain yang dapat Anda tambahkan.

<h3 id="marketplace-is-already-added-from-a-different-source">
  `Marketplace "<name>" is already added from a different source`
</h3>

Anda mengonfirmasi penambahan marketplace melalui [`/plugin install <plugin> --marketplace <source>`](/docs/id/plugins/install#add-a-marketplace-and-install-in-one-command), dan katalog yang Claude Code ambil dari sumber itu memiliki nama yang sama dengan marketplace yang sudah Anda tambahkan dari sumber berbeda. Claude Code menyimpan marketplace yang ada alih-alih menggantinya, dan plugin tidak diinstal.

Pesan lengkapnya terlihat seperti ini:

```text theme={null}
Marketplace "acme-tools" is already added from a different source (github:acme/plugins). To use this source instead, remove that marketplace first with /plugin marketplace remove acme-tools.
```

Pilih sumber mana yang Anda inginkan:

* **Marketplace yang sudah Anda tambahkan**: instal darinya berdasarkan nama dengan `/plugin install <plugin>@<name>`
* **Sumber baru**: jalankan `/plugin marketplace remove <name>`, lalu coba ulang instalasi

<h3 id="cannot-add-marketplace-its-network-source-differs">
  `Cannot add marketplace "<name>": its network source differs from the one declared for it in settings`
</h3>

Anda menjalankan `marketplace add`, dan katalog di sumber itu memiliki nama yang sama dengan marketplace yang file pengaturan sudah deklarasikan di bawah [`extraKnownMarketplaces`](/docs/id/settings-reference#extraknownmarketplaces) dengan sumber berbeda. Claude Code menolak penambahan dan tidak mendaftarkan apa pun.

Pesan berakhir dengan perbaikannya: sumber harus cocok dengan yang dideklarasikan untuk nama ini dalam pengaturan, atau Anda mengubah deklarasi. Bandingkan sumber yang Anda lewatkan terhadap entri `extraKnownMarketplaces` untuk nama itu, termasuk `ref`, `path`, dan `headers`-nya, lalu lakukan salah satu dari ini:

* **Gunakan sumber yang dideklarasikan**: tambahkan marketplace dari sumber yang entri pengaturan beri nama
* **Gunakan sumber baru**: edit atau hapus entri `extraKnownMarketplaces`, lalu tambahkan marketplace lagi. Jika pengaturan terkelola mendeklarasikannya, tanyakan administrator Anda

<h3 id="failed-to-install-from-the-plugin-menu">
  `Failed to install: <plugin> (<reason>)`
</h3>

Anda memilih plugin untuk diinstal di menu `/plugin`, tidak satupun dari mereka diinstal, dan menu ditutup dengan ringkasan apa yang gagal.

Beberapa alasan, seperti output git setelah klon yang gagal, menunjukkan hanya baris pertama mereka. Ketika alasan seperti itu diperpendek, ringkasan berakhir dengan `Installing a plugin from its details (Enter) in /plugin shows its full error.`

Apa yang harus dilakukan tergantung pada apakah ringkasan memendekkan alasan:

* Perbaiki apa yang alasan dalam tanda kurung beri nama
* Ketika alasan diperpendek, jalankan `/plugin`, pilih plugin di tab **Discover**, dan tekan **Enter** untuk menginstalnya dari detailnya. Jika instalasi gagal di sana, tampilan detail menunjukkan kesalahan lengkap

<h3 id="could-not-move-the-new-copy-of-this-plugin-version">
  `Could not move the new copy of this plugin version into <path>`
</h3>

Ketika Anda menginstal plugin, Claude Code mengunduh salinan segar file-nya dan memindahkannya ke folder versi itu di [plugin cache](/docs/id/plugins/loading#find-plugins-on-disk). Pesan ini berarti perpindahan gagal, biasanya karena program lain menggunakan folder saat instalasi berjalan. Kode sistem file muncul dalam tanda kurung:

```text theme={null}
Could not move the new copy of this plugin version into /home/user/.claude/plugins/cache/acme-tools/formatter/1.2.0: the new copy or the version folder stayed busy while the install ran (ENOTEMPTY) — usually a scanner still reading the freshly downloaded files, another program using that folder, or another process re-creating it. The previously installed copy was moved back. Run the install again once other Claude Code sessions or programs using that folder have finished.
```

Pesan mengatakan apa yang terjadi pada salinan yang diinstal sebelumnya, yang memberi tahu Anda apakah plugin masih berfungsi:

* `The previously installed copy was moved back`: versi yang Anda miliki masih diinstal
* `had to be removed first`, `was not moved back`, atau `could not be moved back`: versi plugin itu tidak diinstal sampai instalasi berhasil
* Tidak ada kalimat seperti itu: tidak ada salinan sebelumnya, jadi versi tidak diinstal namun

Di Windows, ketika program lain menyimpan salinan yang diinstal itu sendiri, pesan malah mengatakan salinan itu `could not be replaced` dan bahwa `It was not replaced and the new copy was discarded`, jadi versi yang Anda miliki masih diinstal.

Daftar `Left on disk` menyebutkan folder yang disisihkan di dalam cache. Instalasi versi itu nanti atau pembersihan cache plugin menghapusnya, jadi Anda tidak perlu menghapusnya.

Untuk memperbaiki instalasi:

* Tutup sesi Claude Code lain, editor, dan terminal yang menggunakan folder plugin di bawah `~/.claude/plugins/cache`, lalu jalankan instalasi lagi
* Ketika pesan mengatakan untuk memeriksa izin folder cache plugin, pulihkan izin tulis Anda di folder yang dinamai dan bebaskan ruang disk, lalu jalankan instalasi lagi

<h3 id="dependency-errors">
  Dependency errors
</h3>

Plugin yang mendeklarasikan dependensi dapat gagal diinstal, atau diinstal dan tetap dinonaktifkan, ketika dependensi tidak dapat dipenuhi. Pesan mencapai Anda pada waktu instalasi atau pada waktu beban:

* **Selama instalasi**: penolakan kembali sebagai pesan kesalahan instalasi
* **Ketika plugin memuat**: masalah muncul di `claude plugin list` dan tab **Errors** `/plugin`, dan Claude Code membuat plugin yang terpengaruh tetap dinonaktifkan sampai Anda menyelesaikannya

Tabel mencantumkan setiap pesan dan perbaikannya. Untuk mendeklarasikan dependensi sebagai penulis, lihat [Plugin dependencies](/docs/id/plugins/dependencies).

| Pesan                                                                                             | Arti                                                                                              | Cara menyelesaikan                                                                                                                                                                                                                                                    |
| :------------------------------------------------------------------------------------------------ | :------------------------------------------------------------------------------------------------ | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `Dependency "<dep>" is not installed`                                                             | Dependensi yang dideklarasikan tidak diinstal.                                                    | Instal di shell Anda dengan `claude plugin install <dep>@<marketplace>`, atau uninstal plugin. Jika marketplace dependensi belum terdaftar, tambahkan dan jalankan `/reload-plugins` dalam sesi Anda, yang menginstal dependensi yang hilang yang dapat diselesaikan. |
| `Dependency "<dep>" is disabled`                                                                  | Dependensi diinstal tetapi dimatikan.                                                             | Aktifkan dependensi, atau uninstal plugin yang membutuhkannya.                                                                                                                                                                                                        |
| `Requires "<dep>" <range>, installed <version>`                                                   | Versi dependensi yang diinstal berada di luar rentang yang dideklarasikan plugin.                 | Perbarui dependensi ke versi dalam rentang, atau uninstal plugin.                                                                                                                                                                                                     |
| `<Plugin or Dependency> "<name>" has conflicting version requirements`                            | Tidak ada versi yang memenuhi setiap rentang yang menyematkannya. Pesan mencantumkan rentang-nya. | Uninstal atau perbarui salah satu plugin yang bertentangan, atau minta penulis upstream untuk memperluas batasan-nya.                                                                                                                                                 |
| `... has version requirements too complex to intersect` atau `has an invalid version requirement` | Rentang bukan semver yang valid, atau rentang gabungan tidak dapat dipotong.                      | Perbaiki rentang yang tidak valid atau sederhanakan rantai `\|\|` yang panjang.                                                                                                                                                                                       |
| `... has no git tag satisfying <range>`                                                           | Repository dependensi tidak memiliki tag `<name>--v*` dalam rentang.                              | Periksa bahwa tag upstream rilis dengan konvensi itu, atau relakskan rentang.                                                                                                                                                                                         |
| `Dependency "<dep>" (required by <plugin>) is in <marketplace>, which is not in the allowlist`    | Dependensi ada di marketplace berbeda, dan resolusi lintas-marketplace dimatikan secara default.  | Instal dependensi sendiri di scope yang sama, di shell Anda dengan `claude plugin install <dep>@<marketplace>` ditambah `--scope` yang Anda instal plugin, lalu coba ulang.                                                                                           |

Untuk melihat ini secara terprogram, jalankan `claude plugin list --json` di shell Anda. Plugin dengan masalah membawa field `errors` dengan pesan dan field `errorDetails` dengan `type` untuk masing-masing: dua baris pertama adalah `dependency-unsatisfied` dan yang ketiga adalah `dependency-version-unsatisfied`.

<h2 id="plugin-installed-but-not-working">
  Plugin installed but not working
</h2>

Instalasi berhasil, tetapi skills, hooks, atau server plugin tidak melakukan apa pun. Mulai dengan [Plugin doesn't appear or its skills don't show up](#plugin-doesnt-appear-or-its-skills-dont-show-up), yang memberi tahu Anda di mana Claude Code melaporkan apa yang dimuat, lalu cocokkan pesannya.

<h3 id="plugin-doesnt-appear-or-its-skills-dont-show-up">
  Plugin doesn't appear or its skills don't show up
</h3>

Anda menginstal plugin dan mengetik `/` mengharapkan skills-nya, atau meminta Claude untuk menggunakannya, dan tidak ada yang terjadi.

Periksa status plugin sebelum mengubah apa pun:

<Steps>
  <Step title="Confirm the plugin is installed and enabled">
    Jalankan `/plugin` dan buka **Installed**. Konfirmasi plugin tercantum dan diaktifkan. `claude plugin list` di shell Anda mencetak daftar yang sama dengan versi, scope, dan `Status: ✔ enabled` setiap plugin.
  </Step>

  <Step title="Read the Errors tab">
    Buka tab **Errors** di panel yang sama. Setiap entri memasangkan pesan dengan baris panduan. Sebagian besar pesan di sisa bagian ini berasal dari tab itu.
  </Step>

  <Step title="Reload if you installed during this session">
    Jika plugin diinstal dan bebas kesalahan tetapi Anda menginstalnya selama sesi ini, jalankan `/reload-plugins`. Itu mencetak `Reloaded:` dengan hitungan plugin, skills, agents, hooks, dan server. Ketika sesuatu gagal itu menambahkan `N errors during load. Run /plugin for details.`
  </Step>
</Steps>

Jika plugin memuat tanpa kesalahan dan skills-nya masih tidak muncul, langkah berikutnya berbeda untuk plugin Anda sendiri dan untuk plugin orang lain:

* **Plugin yang Anda bangun**: lihat [Plugin loads but its skills are missing](#plugin-loads-but-its-skills-are-missing)
* **Plugin yang diterbitkan orang lain**: buka **Installed** di `/plugin` dan buka pane detail plugin, yang mencantumkan apa yang berisi plugin. Plugin yang tidak mencantumkan skills di sana tidak memiliki yang ditawarkan ketika Anda mengetik `/`

<h3 id="run-reload-plugins-to-activate">
  `Run /reload-plugins to activate.`
</h3>

Ringkasan instalasi di `/plugin` berakhir dengan `Run /reload-plugins to activate.` alih-alih `Plugin is now active.`

Claude Code tidak mengaktifkan plugin selama instalasi, baik karena mengaktifkannya akan [membatalkan prompt cache](/docs/id/prompt-caching#enabling-or-disabling-a-plugin) atau karena upaya aktivasi gagal.

Anda tidak perlu mengetik command. Panel ditutup dan Claude Code menjalankan `/reload-plugins` untuk Anda, atau mengantrekannya sampai respons yang streaming selesai.

Baca apa yang reload itu cetak:

* **`Reloaded:` dengan hitungan plugin, skills, agents, hooks, dan server**: plugin sekarang aktif. Ketika sesuatu gagal memuat, baris menambahkan `N errors during load. Run /plugin for details.`
* **`This reload changes MCP tools (...) — your next message will re-read the whole conversation instead of using the cache. Run /reload-plugins --force to apply.`**: reload akan menambah atau menghapus server MCP plugin, atau tool `LSP`, dan membatalkan prompt cache Anda. Untuk kasus LSP baris dimulai `This reload adds the LSP tool` atau `This reload removes the LSP tool`. Jalankan dengan `--force` untuk mengaktifkan plugin bagaimanapun, atau mulai sesi baru

Sebelum v2.1.268, instalasi yang tidak diaktifkan selama instalasi tetap tertunda sampai Anda menjalankan `/reload-plugins` sendiri.

Sebelum v2.1.246, hitungan skills dalam ringkasan itu hanya termasuk entri `commands/` plugin, jadi reload dapat memuat skills `SKILL.md` plugin dan masih melaporkan `0 skills`.

<h3 id="plugin-not-cached-at">
  `Plugin "<name>" not cached at <path>`
</h3>

Tab **Errors** menunjukkan baris ini dengan panduan `Run /plugin to refresh the plugin cache`. Claude Code memiliki catatan instalasi untuk plugin, tetapi direktori yang ditunjuk catatan hilang, misalnya setelah Anda menghapus cache.

Instal ulang plugin dari shell Anda. `claude plugin install <name>@<marketplace>` mengunduh ulang plugin yang direktori instalasi-nya hilang meskipun catatan ada:

```shell theme={null}
claude plugin install <name>@<marketplace>
```

Kemudian jalankan `/reload-plugins` dalam sesi Anda. Entri tab **Errors** menghilang dan plugin kembali di bawah **Installed**.

<h3 id="a-plugin-you-disabled-still-loads">
  `Disabled in ~/.claude/settings.json but still loads`
</h3>

Anda menetapkan plugin ke `false` di `~/.claude/settings.json`, dan barisnya di `claude plugin list` atau `/plugin` menunjukkan pesan ini diikuti oleh sumber yang mengaktifkannya, seperti `— project settings enable it, which overrides your user setting`. `true` dalam sumber precedence yang lebih tinggi itu menimpa pengaturan pengguna Anda.

Untuk opt out dari plugin yang diaktifkan proyek di mesin Anda, atur id ke `false` di `.claude/settings.local.json`, yang memiliki precedence lebih tinggi daripada file proyek. Untuk sumber lain yang pesan dapat beri nama, lihat [Disabled in user settings but still loads](/docs/id/plugins/loading#disabled-in-user-settings-but-still-loads).

Jika `claude plugin list` malah menandai plugin `required by your org`, tidak ada file pengaturan yang terlibat: organisasi Anda menandai plugin yang disinkronkan itu sebagai diperlukan di claude.ai, dan itu memuat bahkan jika Anda menonaktifkannya sebelumnya. Lihat [Plugins synced from claude.ai](/docs/id/plugins/loading#synced-plugins).

<h3 id="plugin-is-enabled-in-project-settings-but-isnt-installed-here">
  `Plugin "<name>" is enabled in project settings but isn't installed here`
</h3>

Tab **Errors** menunjukkan baris ini untuk plugin yang `.claude/settings.json` proyek Anda aktifkan, dengan panduan `Run claude plugin install <name>@<marketplace> --scope project to install it for this project`.

Pengaturan repository dapat mengaktifkan plugin untuk semua orang yang membukanya, tetapi mereka tidak menginstalnya. Ketika plugin berasal dari sumber eksternal seperti repository GitHub atau paket npm, Claude Code tidak mengunduhnya sampai Anda menginstalnya sendiri. Jalankan command dari baris panduan di shell Anda, lalu reload:

```shell theme={null}
claude plugin install <name>@<marketplace> --scope project
```

Setelah Anda menjalankan `/reload-plugins` dalam sesi Anda, entri tab **Errors** hilang dan plugin tercantum di bawah **Installed**.

Jika organisasi Anda pre-install plugin untuk Anda, itu melakukannya melalui pengaturan terkelola. Lihat [Pre-install and require plugins](/docs/id/plugins/org#pre-install-and-require-plugins).

<h3 id="failed-to-load-hooks-from-and-hooks-that-dont-fire">
  `Failed to load hooks from <path>` and hooks that don't fire
</h3>

Hooks plugin tidak berjalan. Baik tab **Errors** menunjukkan kegagalan beban untuk mereka, hooks memuat dan Anda melihat pemberitahuan `<Event> hook error` dalam transkrip, atau hook memuat tanpa kesalahan dan tidak pernah menyala.

<h4 id="hooks-fail-to-load">
  Hooks fail to load
</h4>

Tab **Errors** menunjukkan salah satu pesan ini:

* **`Failed to load hooks from <path>: <reason>`**: `hooks/hooks.json` bukan JSON yang valid atau gagal schema hooks. Alasan menyebutkan parse atau kesalahan validasi. Perbaiki file. Untuk menangkap masalah sintaks JSON di `hooks/hooks.json` sebelum Anda menerbitkan plugin, jalankan `claude plugin validate <plugin-directory>` di shell Anda
* **`hooks path not found: <path>`**: field `hooks` manifest menyebutkan file yang tidak ada di path itu relatif terhadap root plugin. Perbaiki path atau tambahkan file

<h4 id="hook-error-notices-in-the-transcript">
  `hook error` notices in the transcript
</h4>

Pemberitahuan bentuk `... hook error: Failed with non-blocking status code: <stderr>` berarti hook berjalan dan command-nya gagal. Misalnya, `Stop hook error: Failed with non-blocking status code: /bin/sh: node: command not found` berarti shell yang Claude Code spawn tidak dapat menemukan `node`. Instal, atau pastikan itu ada di `PATH` dari terminal yang Anda mulai `claude` dari.

Untuk kesalahan lain apa pun, jalankan command hook sendiri dari direktori plugin untuk melihat output lengkap, atau tangkap stderr lengkap dengan [debug logging](/docs/id/hooks#debug-hooks).

<h4 id="hook-loads-but-never-fires">
  Hook loads but never fires
</h4>

Jika hook memuat tanpa kesalahan tetapi tidak pernah menyala, periksa definisinya lalu tonton itu berjalan:

<Steps>
  <Step title="Check the event name">
    Nama event peka huruf besar-kecil, jadi konfirmasi milik Anda cocok persis, misalnya `PostToolUse`.
  </Step>

  <Step title="Check the matcher">
    Konfirmasi `matcher` hook cocok dengan nama tool.
  </Step>

  <Step title="Trigger the event on purpose">
    Untuk hook `PostToolUse`, minta Claude untuk mengedit file.
  </Step>

  <Step title="Read the debug log">
    Buka [debug log](/docs/id/hooks#debug-hooks), yang mencatat hook mana yang cocok. Hook yang berjalan muncul di sana dengan kode keluar-nya.
  </Step>
</Steps>

<h3 id="invalid-mcp-server-config-for-and-mcp-servers-that-dont-start">
  `Invalid MCP server config for "<server>"` and MCP servers that don't start
</h3>

Plugin menggabungkan server MCP, dan tab **Errors** menunjukkan `Invalid MCP server config for "<server>": <error>`, atau server tercantum tetapi `/mcp` tidak pernah menunjukkannya terhubung.

<h4 id="invalid-mcp-server-config-for-server-error">
  `Invalid MCP server config for "<server>": <error>`
</h4>

Konfigurasi server melewati pemeriksaan schema, tetapi Claude Code tidak dapat menyelesaikannya untuk sesi ini. Teks setelah titik dua menyebutkan penyebab dan memutuskan perbaikan:

* **`Missing environment variables: <names>`**: atur variabel itu di shell yang Anda mulai Claude Code dari, lalu mulai sesi baru
* **`URL is unset or invalid`**: opsi `${user_config.*}` yang URL gunakan tidak diatur. Jalankan `/plugin configure <plugin>` untuk mengaturnya
* **`has an invalid MCP url`** atau **`headersHelper for MCP server '<server>' references ${user_config.*}`**: konfigurasi plugin itu sendiri yang salah. Perbaiki `url` atau `headersHelper` dalam konfigurasi MCP plugin Anda, atau laporkan ke penulis plugin jika plugin bukan milik Anda. Kasus `headersHelper` memiliki entri sendiri di bawah [plugin command references user\_config](/docs/id/errors#plugin-command-references-user-config)

<h4 id="server-is-configured-but-never-connects">
  Server is configured but never connects
</h4>

Jalankan `/mcp` untuk melihat status server. Ketika server sehat, `/mcp` mencantumkannya sebagai terhubung.

Untuk membaca kesalahan yang dicetak server saat memulai, jalankan `claude --debug` dan buka log di `~/.claude/debug/<session-id>.txt`. Flag `--debug` tidak mencetak ke terminal.

Entri server di `.mcp.json` yang gagal schema tidak muncul di tab **Errors**. Claude Code menjatuhkan server itu dan mencatat `Invalid MCP server config for <server> in <path>` hanya dalam log debug itu. Untuk menemukan entri tanpa memuat plugin, jalankan `claude plugin validate` di shell Anda pada direktori plugin, yang melaporkannya sebagai kesalahan.

Sebelum v2.1.281, `claude plugin validate` tidak memeriksa `.mcp.json`.

<h4 id="server-works-with-plugin-dir-but-fails-after-install">
  Server works with `--plugin-dir` but fails after install
</h4>

Anda adalah penulis plugin, dan server dimulai ketika Anda memuat plugin dari direktori sumber-nya dengan `--plugin-dir` tetapi gagal setelah plugin diinstal.

Claude Code menyalin plugin yang diinstal ke dalam cache-nya, jadi path yang hanya berfungsi dari direktori sumber rusak. Tulis path di dalam plugin dengan `${CLAUDE_PLUGIN_ROOT}`.

Untuk path yang menjangkau di luar direktori plugin, lihat [Files the plugin references outside its directory aren't found](#files-the-plugin-references-outside-its-directory-arent-found).

<h3 id="language-server-doesnt-start">
  Language server doesn't start, uses too much memory, or reports wrong diagnostics
</h3>

Anda menginstal [code intelligence plugin](/docs/id/plugins/code-intelligence) dan Claude tidak melihat diagnostik, atau language server menggunakan terlalu banyak memori atau melaporkan kesalahan yang tidak nyata.

<h4 id="language-server-doesn’t-start">
  Language server doesn't start
</h4>

Plugin terhubung ke binary language server yang Anda instal secara terpisah, dan Claude Code memunculkannya dengan nama command dari `PATH` Anda.

Tab **Errors** `/plugin` menunjukkan kegagalan dengan alasannya, seperti `Executable not found in $PATH: "<binary>"`, dan `claude --debug` mencatatnya sebagai `LSP server <name> failed to start: <reason>`.

Instal binary dan konfirmasi itu ada di `PATH` dari terminal yang Anda mulai `claude` dari, misalnya dengan `which typescript-language-server`. Kemudian mulai sesi baru.

<h4 id="language-server-uses-too-much-memory">
  Language server uses too much memory
</h4>

Language server seperti `rust-analyzer` dan `pyright` mengindeks seluruh proyek. Nonaktifkan plugin dengan `/plugin disable <plugin>` dalam sesi dan andalkan tool pencarian bawaan Claude sebagai gantinya.

<h4 id="false-positive-diagnostics-in-a-monorepo">
  False positive diagnostics in a monorepo
</h4>

Language server yang tidak dikonfigurasi untuk workspace dapat melaporkan impor yang tidak terselesaikan untuk paket internal. Tidak ada yang perlu diperbaiki di sisi Claude Code, dan diagnostik tidak menghentikan Claude dari mengedit kode.

<h2 id="build-a-plugin">
  Build a plugin
</h2>

Anda mengembangkan plugin dan memuat dengan `--plugin-dir` atau menginstalnya dari marketplace lokal. Entri ini mencakup kegagalan yang Anda alami saat mengembangkan plugin. Untuk pemeriksaan yang berjalan setelah setiap perubahan, lihat [Test and debug](/docs/id/plugins/create#test-and-debug).

Dua kegagalan yang juga menjangkau pengguna plugin memiliki entri mereka di bawah [Plugin installed but not working](#plugin-installed-but-not-working):

* **Hook yang tidak menyala**: lihat [hooks that don't fire](#failed-to-load-hooks-from-and-hooks-that-dont-fire)
* **Server MCP yang tidak dimulai**: lihat [MCP servers that don't start](#invalid-mcp-server-config-for-and-mcp-servers-that-dont-start)

<h3 id="commands-path-not-found">
  `commands path not found: <path>`
</h3>

Tab **Errors** menunjukkan `commands path not found: <absolute path>` dengan panduan `Check that the path in your manifest or marketplace config is correct`. Pesan yang sama muncul untuk `skills`, `agents`, dan `hooks`.

Claude Code menyelesaikan path dari `plugin.json` Anda atau entri marketplace terhadap root plugin dan menemukan tidak ada di sana. Path dalam pesan adalah path absolut yang diperiksa, jadi bandingkan dengan apa yang ada di disk. Perbaiki path atau buat direktori, lalu jalankan `/reload-plugins`.

Path dalam manifest relatif terhadap root plugin dan dimulai dengan `./`. Path yang diselesaikan di luar root plugin dilaporkan sebagai `<component> path escapes plugin directory` sebagai gantinya dan dijatuhkan.

<h3 id="plugin-dir-loads-a-plugin-with-no-components">
  `--plugin-dir` at a marketplace root doesn't load the plugins under `plugins/`
</h3>

Anda memulai `claude --plugin-dir <path>` dan tidak melihat kesalahan, tetapi skills, agents, dan hooks plugin tidak ada.

`--plugin-dir` mengambil direktori root plugin, yang berisi `.claude-plugin/plugin.json` dan direktori komponen seperti `skills/`. Jika Anda menunjuknya ke root marketplace, Claude Code tidak membaca `marketplace.json`, jadi plugin di bawah `plugins/` tidak memuat, dan Anda tidak melihat kesalahan. Sebelum v2.1.281, Claude Code memuat root marketplace sebagai satu plugin kosong bernama setelah direktori itu. Tunjukkan flag ke direktori plugin itu sendiri:

```shell theme={null}
claude --plugin-dir ./my-marketplace/plugins/my-plugin
```

Kemudian buka **Installed** di `/plugin`, di mana pane detail plugin mencantumkan komponen-nya.

<h3 id="files-the-plugin-references-outside-its-directory-arent-found">
  Files the plugin references outside its directory aren't found
</h3>

Plugin berfungsi dari direktori sumber-nya dengan `--plugin-dir` tetapi gagal setelah instalasi, dengan kesalahan tentang path seperti `../shared-utils`.

Claude Code menyalin plugin yang diinstal ke dalam cache-nya dan memuat dari sana, jadi path yang menjangkau di luar direktori plugin itu sendiri menunjuk pada tidak ada dalam cache. Pindahkan file bersama di dalam direktori plugin, atau referensikan mereka melalui symlink di dalamnya. Untuk di mana cache dan bagaimana path diselesaikan, lihat [Find plugins on disk](/docs/id/plugins/loading#find-plugins-on-disk).

<h3 id="claude-plugin-root-shows-forward-slashes-on-windows">
  `${CLAUDE_PLUGIN_ROOT}` shows forward slashes on Windows
</h3>

Di Windows, hook plugin menerima `${CLAUDE_PLUGIN_ROOT}` sebagai `C:/Users/you/...` daripada `C:\Users\you\...`, dan script yang mengharapkan backslash rusak.

Claude Code menjalankan hook bentuk shell melalui Git Bash di Windows dan mengganti root plugin dalam bentuk Win32 forward-slash dengan tujuan. Bash builtins, alat MSYS, dan binary Windows asli semuanya menerima bentuk itu.

Jika script Anda membutuhkan backslash, alihkan hook ke salah satu bentuk yang menyimpan path asli, dijelaskan di bawah [exec form and shell form](/docs/id/hooks#exec-form-and-shell-form):

* Hook bentuk exec, yang memunculkan proses langsung dengan array `args`
* Hook dengan `"shell": "powershell"`

<h3 id="plugin-loads-but-its-skills-are-missing">
  Plugin loads but its skills are missing
</h3>

Plugin Anda tercantum di bawah **Installed** tanpa kesalahan, tetapi skills-nya tidak ditawarkan ketika Anda mengetik `/`.

Skills memuat dari `skills/` di root plugin dan command dari `commands/` di root plugin. Hanya `plugin.json` yang termasuk di dalam `.claude-plugin/`, dan direktori `skills/` di dalam `.claude-plugin/` tidak dipindai. Pindahkan direktori ke root plugin dan jalankan `/reload-plugins`. Setelah itu, pane detail plugin di `/plugin` mencantumkan skills, dan mengetik `/` menawarkan mereka.

Setiap skill adalah direktori yang berisi `SKILL.md`. Entri `skills` dalam manifest yang menunjuk pada file `SKILL.md` daripada direktorinya dilaporkan sebagai `path is a file; skills entries must be directories containing SKILL.md`.

<h3 id="skill-loads-but-claude-never-invokes-the-skill">
  Skill loads but Claude never invokes the skill
</h3>

Skill plugin Anda berjalan ketika Anda mengetik command `/<plugin>:<skill>` nya, tetapi Claude tidak pernah memanggilnya sebagai respons terhadap permintaan biasa.

Periksa penyebab ini secara berurutan:

* **Skill menetapkan `disable-model-invocation: true`**: dengan field itu diatur, hanya Anda yang dapat memanggil skill. Template skill di [Create your first plugin](/docs/id/plugins/create#create-your-first-plugin) mengaturnya. Hapus baris dari skill yang ingin Claude panggil sendiri. [Control who invokes a skill](/docs/id/skills#control-who-invokes-a-skill) mencakup field
* **Deskripsi tidak cocok dengan cara orang bertanya**: kerjakan pemeriksaan di [Skill not triggering](/docs/id/skills#skill-not-triggering)
* **Deskripsi dipotong**: ketika banyak skills diinstal, Claude Code memendekkan deskripsi agar sesuai dengan anggaran karakter daftar, yang dapat menghapus kata kunci yang Claude butuhkan untuk mencocokkan permintaan. Lihat [Skill descriptions are cut short](/docs/id/skills#skill-descriptions-are-cut-short)

Untuk mengukur seberapa sering skill memicu di seluruh prompt realistis daripada memeriksa satu per satu, tulis kasus eval dengan [grader `tool_used: Skill`](/docs/id/plugin-evals#create-your-first-eval-suite) dan jalankan dengan `claude plugin eval` setelah setiap perubahan deskripsi.

<h3 id="is-not-a-plugin-or-skill-folder">
  `<directory> is not a plugin or skill folder` from `claude plugin eval init`
</h3>

Anda menjalankan `claude plugin eval init` dari direktori yang bukan root plugin, seperti direktori home Anda atau root repository yang menyimpan plugin dalam subdirektori. `init` menulis suite di bawah direktori kerja, jadi itu berhenti alih-alih membuat direktori `evals/` yang plugin tidak akan pernah lihat.

Ubah ke root plugin, direktori yang menyimpan `.claude-plugin/plugin.json` atau `SKILL.md` skill, dan jalankan command lagi. Untuk scaffold suite di tempat lain dengan tujuan, lewatkan `--eval-dir`. Lihat [Test plugins with evals](/docs/id/plugin-evals).

<h3 id="the-userconfig-dialog-never-appears">
  The `userConfig` dialog never appears
</h3>

Plugin Anda mendeklarasikan opsi `userConfig`, tetapi tidak ada dialog konfigurasi yang muncul ketika Anda menginstalnya.

Instalasi interaktif menunjukkan dialog, dan command shell mengambil nilai sebagai flag:

* **`/plugin install` dalam sesi, atau tab Discover di `/plugin`**: dialog adalah bagian dari instalasi interaktif ini
* **`claude plugin install` di shell Anda**: tidak pernah meminta nilai `userConfig`. Itu menyimpan nilai `--config KEY=VALUE` apa pun yang Anda lewatkan, dan ketika opsi tetap tidak diatur itu mencetak `N userConfig options not yet set — run /plugin configure <plugin>@<marketplace> in Claude Code, or pass --config KEY=VALUE.` Ketika salah satu opsi yang tidak diatur diperlukan, `(M required)` mengikuti `not yet set`.

Jika Anda menginstal dari shell, lewatkan nilai dengan `--config`, satu flag per opsi:

```shell theme={null}
claude plugin install my-plugin@my-marketplace --config api_url=https://example.com
```

Ketika setiap opsi diatur, output instalasi tidak membawa baris `not yet set`. Untuk membuka dialog setelahnya, jalankan `/plugin configure my-plugin@my-marketplace` dalam sesi.

Jika Anda melewatkan kunci `--config` yang manifest tidak deklarasikan, plugin masih diinstal, dan command mencetak `⚠ Installed, but --config not applied: --config key "<key>" isn't declared in this plugin's userConfig.` diikuti oleh kunci yang plugin deklarasikan.

<h3 id="claude-plugin-validate-reports-errors">
  `claude plugin validate` reports errors
</h3>

Anda menjalankan `claude plugin validate <path>`, atau `/plugin validate <path>` dalam sesi, dan itu mencetak `Found N errors` dan `Validation failed`, lalu keluar dengan kode 1.

Validator membaca manifest di path yang Anda berikan: `.claude-plugin/plugin.json` untuk direktori plugin, atau `.claude-plugin/marketplace.json` untuk direktori marketplace. Untuk marketplace, itu memprefiks masalah dalam manifest entri sendiri dengan indeks entri, sebagai `plugins[1] plugin.json → json: ...`.

Tabel mencakup pesan yang menghentikan validasi dan dua peringatan, `No frontmatter block found` dan `Unknown field '<key>'`, yang menghentikannya hanya ketika Anda melewatkan `--strict`. Peringatan lain, seperti deskripsi yang hilang, tidak tercantum.

| Pesan                                                                                                    | Penyebab                                                                                | Perbaikan                                                                                                        |
| :------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------- |
| `File not found: <path>`                                                                                 | Path tidak memiliki manifest, atau tidak ada.                                           | Jalankan command terhadap root plugin atau marketplace, direktori yang berisi `.claude-plugin/`.                 |
| `No manifest found in directory. Expected .claude-plugin/marketplace.json or .claude-plugin/plugin.json` | Direktori tidak memiliki manifest `.claude-plugin/`.                                    | Buat manifest, atau tunjuk ke direktori yang benar.                                                              |
| `Invalid JSON syntax: <parse error>`                                                                     | Manifest, atau `hooks/hooks.json`, bukan JSON yang valid.                               | Perbaiki JSON. Sampai Anda memperbaiki `hooks/hooks.json`, sesi memuat plugin tanpa hooks dalam file itu.        |
| `Path not found: <path>. The runtime loader will report this as a load failure.`                         | Path komponen dalam manifest tidak ada.                                                 | Perbaiki path atau buat direktori.                                                                               |
| `Path contains ".." which could be a path traversal attempt: <path>`                                     | Path komponen melarikan diri dari direktori plugin.                                     | Gunakan path di dalam root plugin.                                                                               |
| `Path is a file; skills entries must be directories containing SKILL.md`                                 | Entri `skills` menunjuk pada `SKILL.md` alih-alih direktorinya.                         | Tunjuk ke direktori induk, atau `.` untuk `SKILL.md` tingkat root.                                               |
| `No frontmatter block found` atau `YAML frontmatter failed to parse: <error>`                            | File skill, agent, atau command memiliki frontmatter YAML yang hilang atau tidak valid. | Tambahkan atau perbaiki frontmatter antara pembatas `---`. Dilaporkan saat memvalidasi direktori plugin.         |
| `Unknown field '<key>'`                                                                                  | Manifest memiliki field yang schema tidak tentukan.                                     | Hapus, atau gunakan nama yang pesan sarankan. Claude Code mengabaikan field yang tidak dikenal pada waktu beban. |

Jalankan command lagi setelah setiap perbaikan sampai itu mencetak tidak ada kesalahan.

Field `plugin.json` ada di [manifest reference](/docs/id/plugins/manifest-reference), dan pesan tingkat marketplace ada di bawah [Marketplace validation errors](#marketplace-validation-errors).

<h3 id="plugin-has-conflicting-manifests">
  `Plugin <name> has conflicting manifests`
</h3>

Plugin gagal memuat dengan `Plugin <name> has conflicting manifests: both plugin.json and marketplace entry specify components.`

Plugin memiliki `plugin.json` sendiri, dan entri marketplace-nya menetapkan `strict: false` sambil juga mendeklarasikan salah satu dari `commands`, `agents`, `skills`, `hooks`, `outputStyles`, atau `themes`. Hapus field itu dari entri, atau atur `strict: true` dalam entri sehingga Claude Code menambahkannya ke `plugin.json`. Lihat [Strict mode](/docs/id/plugins/marketplace-reference#strict-mode).

<h3 id="warning-no-commands-found-in-plugin-custom-directory">
  `Warning: No commands found in plugin <name> custom directory`
</h3>

Ketika plugin memuat, log `claude --debug` di `~/.claude/debug/<session-id>.txt` mencatat `Warning: No commands found in plugin <name> custom directory: <path>. Expected .md files or SKILL.md in subdirectories.` Tidak ada yang muncul dalam sesi atau tab **Errors**.

Path `commands` dalam manifest ada tetapi tidak menyimpan file `.md` dan tidak ada `SKILL.md` dalam subdirektori. Tambahkan file command, atau hapus path dari manifest.

<h2 id="host-a-marketplace">
  Host a marketplace
</h2>

Anda menerbitkan marketplace dan pengguna melaporkan kesalahan, atau validasi Anda sendiri gagal. Entri ini untuk pemilik marketplace.

<h3 id="plugins-with-relative-paths-fail-in-url-based-marketplaces">
  Plugins with relative paths fail in URL-based marketplaces
</h3>

Pengguna menambahkan marketplace Anda dengan URL `https://example.com/marketplace.json`. Instalasi plugin yang `source` adalah path relatif, seperti `./plugins/my-plugin`, gagal dengan `its marketplace entry path does not stay inside the marketplace directory`. Plugin yang sudah diinstal gagal memuat dengan `Plugin source path refused`. Kedua pesan memiliki [entri referensi kesalahan](/docs/id/errors#marketplace-entry-path-does-not-stay-inside-the-marketplace-directory).

Ketika pengguna menambahkan marketplace berbasis URL, Claude Code hanya mengunduh file `marketplace.json` itu sendiri. Itu tidak mengambil file plugin dengan path relatif dari server itu, jadi path relatif dalam entri menunjuk pada direktori yang tidak pernah diambil. Berikan setiap entri sumber yang Claude Code dapat ambil sendiri, seperti repository GitHub:

```json theme={null}
{ "name": "my-plugin", "source": { "source": "github", "repo": "owner/repo" } }
```

Atau, host marketplace dalam repository git dan beri tahu pengguna untuk menambahkannya dengan URL repository. Untuk sumber git, Claude Code mengklon seluruh repository, jadi path relatif diselesaikan. Tipe sumber ada di [marketplace reference](/docs/id/plugins/marketplace-reference).

<h3 id="marketplace-validation-errors">
  Marketplace validation errors
</h3>

Anda menjalankan `claude plugin validate .` dari direktori marketplace Anda dan itu melaporkan kesalahan atau peringatan pada file marketplace itu sendiri.

`claude plugin validate` juga memvalidasi setiap entri yang `source` adalah path lokal dan memperingatkan ketika `version` entri tidak setuju dengan manifest plugin itu sendiri.

Tabel mencantumkan pesan tingkat marketplace. Pesan tingkat entri adalah pesan plugin di bawah [`claude plugin validate` reports errors](#claude-plugin-validate-reports-errors), diprefiks dengan `plugins[N] plugin.json →`.

| Pesan                                                                                                                       | Jenis      | Perbaikan                                                                                                                                       |
| :-------------------------------------------------------------------------------------------------------------------------- | :--------- | :---------------------------------------------------------------------------------------------------------------------------------------------- |
| `Duplicate plugin name "<name>" found in marketplace`                                                                       | Kesalahan  | Berikan setiap plugin `name` yang unik.                                                                                                         |
| `Path contains "..": <path>` di bawah `plugins[N].source`                                                                   | Kesalahan  | Gunakan path relatif terhadap root marketplace tanpa segmen `..`.                                                                               |
| `Marketplace name cannot contain control or bidirectional-formatting characters`                                            | Kesalahan  | Hapus karakter dari nama, seperti escape atau newline.                                                                                          |
| `Plugin name cannot contain control or bidirectional-formatting characters`                                                 | Kesalahan  | Hapus karakter dari plugin `name`.                                                                                                              |
| `Marketplace has no plugins defined`                                                                                        | Peringatan | Tambahkan setidaknya satu entri ke `plugins`.                                                                                                   |
| `No marketplace description provided`                                                                                       | Peringatan | Tambahkan `description` tingkat atas.                                                                                                           |
| `Plugin name "<name>" is not kebab-case` di bawah `plugins[N] plugin.json → name`                                           | Peringatan | Ganti nama ke huruf kecil, digit, dan tanda hubung. Claude Code menerima bentuk lain, tetapi sinkronisasi marketplace claude.ai menolaknya.     |
| `Entry declares version "<a>" but <path>/plugin.json says "<b>"`                                                            | Peringatan | Perbarui entri agar cocok dengan `plugin.json`, yang otoritatif pada waktu instalasi.                                                           |
| `Marketplace name "<name>" is reserved in Claude Desktop`                                                                   | Peringatan | Ganti nama marketplace. Sinkronisasi marketplace terkelola Claude Desktop menolak `org`, `org-provisioned`, dan `unknown` dalam casing apa pun. |
| `Marketplace name "<name>" is not accepted by Claude Desktop` atau `Plugin name "<name>" is not accepted by Claude Desktop` | Peringatan | Ganti nama ke paling banyak 128 karakter huruf, digit, `.`, `_`, dan `-`, dimulai dengan huruf atau digit.                                      |

Sebelum v2.1.247, nama marketplace yang berisi karakter kontrol atau bidirectional-formatting dilaporkan hanya sebagai `Marketplace name impersonates an official Anthropic/Claude marketplace`.

<h2 id="blocked-by-your-organization">
  Blocked by your organization
</h2>

Organisasi Anda menerapkan pengaturan terkelola yang membatasi plugin, dan command ditolak dengan pesan kebijakan. Entri ini menyebutkan pengaturan di balik setiap penolakan sehingga Anda tahu apa yang harus diminta administrator Anda. Untuk sisi admin, lihat [Manage plugins for your organization](/docs/id/plugins/org).

<h3 id="marketplace-source-is-blocked-by-enterprise-policy">
  `Marketplace source '<source>' is blocked by enterprise policy`
</h3>

Anda menjalankan `/plugin marketplace add`, `update`, atau instalasi, dan Claude Code menolak dengan baris ini. Untuk sumber GitHub atau git, host mengikuti sumber dalam tanda kurung, seperti dalam `'github:owner/repo' (github.com)`.

Administrator Anda menetapkan `blockedMarketplaces` atau `strictKnownMarketplaces` dalam pengaturan terkelola, dan sumber ini tidak diizinkan. Minta administrator Anda untuk mengizinkan sumber, atau tambahkan salah satu sumber yang diizinkan yang pesan cantumkan.

Cocokkan sisa pesan untuk melihat jenis kebijakan apa yang memblokir sumber:

* **`Allowed sources: <list>`**: blok berasal dari allowlist `strictKnownMarketplaces` daripada blocklist `blockedMarketplaces`
* **`No external marketplaces are allowed.`**: allowlist `strictKnownMarketplaces` kosong
* **`Tip:` bahwa shorthand mengasumsikan github.com**: allowlist mengizinkan host git dengan hostname, dan shorthand `owner/repo` yang Anda lewatkan menunjuk pada github.com. Jika repository tinggal di host internal Anda, tambahkan lagi dengan URL lengkapnya, seperti `git@your-git-host.com:owner/repo.git`

Marketplace yang Anda tambahkan sebelum kebijakan menjadi lebih ketat berhenti merefresh juga, karena kebijakan berlaku pada setiap refresh.

<h3 id="marketplace-is-not-in-the-allowed-marketplace-list">
  `Marketplace "<name>" is not in the allowed marketplace list`
</h3>

Tab **Errors** menunjukkan baris ini, atau `Marketplace "<name>" is blocked by enterprise policy`, untuk marketplace yang sudah Anda daftarkan.

Pengaturan terkelola yang sama yang memblokir [marketplace source](#marketplace-source-is-blocked-by-enterprise-policy) berlaku pada waktu beban. `strictKnownMarketplaces` tidak termasuk marketplace ini, atau `blockedMarketplaces` menyebutkannya, jadi Claude Code berhenti memuat dan plugin-nya. Untuk varian allowlist, baris panduan menunjukkan sumber yang diizinkan, atau `Contact your administrator to configure allowed marketplace sources`. Untuk varian blocklist itu berbunyi `This marketplace source is explicitly blocked by your administrator`.

<h3 id="plugin-is-blocked-by-your-organizations-policy-and-cannot-be-installed">
  `Plugin "<name>" is blocked by your organization's policy and cannot be installed`
</h3>

Instalasi ditolak dengan baris ini, enable dengan baris yang sama berakhir `cannot be enabled`, atau instalasi atau update dengan satu yang menyebutkan alasan: `Plugin "<name>" is from marketplace "<marketplace>", which is blocked by your organization's policy`, atau `Plugin "<name>" depends on "<dep>", which is blocked by your organization's policy`.

Pengaturan terkelola memblokir plugin ini, marketplace-nya, atau dependensi yang dibutuhkannya. Tanyakan administrator Anda entri mana yang berlaku. Dependensi yang diblokir berarti plugin tidak dapat diinstal sampai marketplace dependensi diizinkan.

<h3 id="plugin-dir-is-disabled-by-your-organizations-managed-settings-disables">
  `--plugin-dir is disabled by your organization's managed settings (disableSideloadFlags)`
</h3>

Anda memulai `claude` dengan `--plugin-dir`, `--plugin-url`, `--agents`, atau `--mcp-config`. Claude Code keluar dengan pesan ini dan `Plugins, custom agents, and MCP servers can only be loaded from sources your administrator has approved.`

Administrator Anda menetapkan `disableSideloadFlags` dalam pengaturan terkelola, yang mematikan flag yang memuat plugin, agents, dan server dari path arbitrer. Muat plugin dari marketplace yang disetujui, atau minta administrator Anda untuk menghapus pengaturan.

Pesan terkait di tab **Errors** `/plugin` adalah `--plugin-dir copy of "<name>" ignored: plugin is locked by managed settings`. Pengaturan terkelola mengaktifkan atau menonaktifkan plugin itu berdasarkan nama, dan Claude Code mengabaikan salinan `--plugin-dir` Anda sehingga flag tidak dapat menimpa kebijakan.

<h3 id="plugins-from-claude-skills-are-blocked-by-your-organizations-managed-s">
  `Plugins from ~/.claude/skills/ are blocked by your organization's managed settings`
</h3>

Anda menjalankan `claude plugin init` atau `claude plugin enable`, dan itu berhenti dengan baris ini. Pesan menyebutkan `strictKnownMarketplaces or blockedMarketplaces` dan meminta administrator Anda untuk menambahkan `{"source":"skills-dir"}` ke `strictKnownMarketplaces` atau menghapusnya dari `blockedMarketplaces`.

Sumber `skills-dir` berdiri untuk plugin yang Claude Code muat dari direktori `~/.claude/skills/` Anda. Minta administrator Anda untuk membuat perubahan yang pesan sebutkan.

<h3 id="command-sourced-plugins-are-disabled-by-your-organizations-managed-set">
  `Command-sourced plugins are disabled by your organization's managed settings`
</h3>

Anda menginstal atau memperbarui plugin dengan sumber `command`, dan itu berhenti dengan baris ini dan `The plugin was not installed or updated and its command was not run.`

Administrator Anda menetapkan `disableCommandPluginSources`, jadi Claude Code menolak untuk menjalankan command yang dideklarasikan marketplace yang menghasilkan plugin. Menetapkan `allowManagedHooksOnly` saja memiliki efek yang sama ketika `disableCommandPluginSources` tidak diatur. Tanyakan administrator Anda apakah plugin dapat diterbitkan dari tipe sumber yang kebijakan izinkan.

<h3 id="marketplace-is-seed-managed">
  `Marketplace '<name>' is seed-managed`
</h3>

Anda menjalankan `claude plugin marketplace update <name>`, dan itu gagal dengan `Marketplace '<name>' is seed-managed (<dir>)` dan hint untuk meminta admin Anda.

Operator pre-populated marketplace ini melalui `CLAUDE_CODE_PLUGIN_SEED_DIR`, dan Claude Code memperlakukan marketplace yang seed-managed sebagai read-only. `marketplace update` massal melewatinya dan memperbarui yang lain.

Untuk mengubah konten marketplace, minta orang yang memelihara gambar seed untuk memperbaruinya. Untuk prosedurnya, lihat [Seed containers and CI](/docs/id/plugins/org#seed-containers-and-ci).

<h2 id="next-steps">
  Next steps
</h2>

* [Plugin loading reference](/docs/id/plugins/loading): mengapa scope, cache, dan precedence berperilaku seperti yang mereka lakukan
* [Plugin commands reference](/docs/id/plugins/cli-reference): flag, default, output, dan exit code untuk command `claude plugin`
* [Install and manage plugins](/docs/id/plugins/install): langkah instalasi dari awal
* [Manage plugins for your organization](/docs/id/plugins/org#troubleshoot-policy): troubleshooting sisi kebijakan untuk administrator
