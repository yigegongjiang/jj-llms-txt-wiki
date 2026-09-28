> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Host dan kelola marketplace

> Publikasikan marketplace plugin tempat pengguna dapat mengaksesnya, berikan akses ke marketplace pribadi, dan rilis pembaruan serta perubahan nama tanpa merusak instalasi.

Menghost marketplace berarti menempatkan katalog `marketplace.json` Anda di tempat di mana orang lain dapat menambahkannya dengan `/plugin marketplace add`, memasang plugin-nya, dan terus menerima perubahan Anda setelah Anda push.

Halaman ini untuk orang yang mengoperasikan marketplace.

<Note>
  Kasus-kasus ini tercakup di halaman lain:

  * **Anda belum menulis file katalog**: mulai dengan [Create a marketplace](/docs/id/plugins/create-marketplace)
  * **Anda adalah admin yang memerlukan, membatasi, atau pra-memasang marketplace di seluruh mesin organisasi Anda**: baca [Manage plugins for your organization](/docs/id/plugins/org)
</Note>

Mulai dengan [Host your marketplace](#host-your-marketplace) untuk memilih host dan perintah yang dijalankan pengguna Anda. Baca [Keep users up to date](#keep-users-up-to-date) sebelum rilis pertama Anda. Baca [Rename or remove a plugin](#rename-or-remove-a-plugin) sebelum Anda mengubah `name` plugin.

<h2 id="host-your-marketplace">
  Host your marketplace
</h2>

Anda dapat menghost marketplace di GitHub, di host git lain, sebagai URL `marketplace.json` yang dihosting, atau di direktori pada filesystem bersama. Kirim perintah add kepada pengguna Anda untuk host Anda dan beri tahu mereka apa yang mereka butuhkan di mesin mereka:

| Host                                                            | Pengguna jalankan, dalam sesi Claude Code                              | Apa yang pengguna butuhkan                                                                                                                         |
| :-------------------------------------------------------------- | :--------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------- |
| GitHub                                                          | `/plugin marketplace add your-org/your-marketplace`                    | `git`, dan untuk repositori pribadi akses yang dijelaskan di bawah [Grant access to a private marketplace](#grant-access-to-a-private-marketplace) |
| GitLab, Bitbucket, GitHub Enterprise Server, atau host git lain | `/plugin marketplace add https://gitlab.example.com/team/plugins.git`  | `git`, dan akses ke host dari mesin mereka. Kirim URL lengkap, karena shorthand `owner/repo` selalu berarti github.com                             |
| URL `marketplace.json` yang dihosting                           | `/plugin marketplace add https://plugins.example.com/marketplace.json` | Akses HTTPS ke URL. Pengguna tidak memerlukan `git` untuk katalog itu sendiri                                                                      |
| Direktori pada filesystem bersama                               | `/plugin marketplace add /Volumes/shared/claude-plugins`               | Akses baca ke path                                                                                                                                 |

Untuk mengunci branch atau tag dari marketplace GitHub atau git-URL, beri tahu pengguna untuk menambahkan `#<ref>`, seperti dalam `your-org/your-marketplace#stable`. [Plugin commands reference](/docs/id/plugins/cli-reference#plugin-marketplace-add) mencantumkan setiap bentuk yang diterima perintah.

Penambahan yang berhasil mencetak `Successfully added marketplace: your-marketplace`. Claude Code mengambil nama itu dari field `name` di `marketplace.json` Anda, bukan dari nama repositori.

Pengguna kemudian memasang plugin berdasarkan `name` entri dan `name` marketplace, seperti dalam `/plugin install code-formatter@your-marketplace`.

<h3 id="register-the-marketplace-for-everyone-in-a-repository">
  Register the marketplace for everyone in a repository
</h3>

Untuk berbagi marketplace dengan semua orang yang bekerja di satu repositori, jalankan `claude plugin marketplace add your-org/your-marketplace --scope project` di sana sekali dari shell Anda dan commit `.claude/settings.json` yang ditulis. Claude Code kemudian mendaftarkan marketplace untuk setiap rekan kerja yang [trusts the folder](/docs/id/plugins/org#require-plugins-per-repository).

<h3 id="avoid-relative-path-entries-in-a-url-hosted-marketplace">
  Avoid relative-path entries in a URL-hosted marketplace
</h3>

Ketika pengguna menambahkan marketplace Anda sebagai URL `marketplace.json` bare, Claude Code hanya mengunduh file itu. Entri di array `plugins` Anda yang `source`-nya adalah path relatif seperti `./plugins/formatter` kemudian gagal saat instalasi dengan [`its marketplace entry path does not stay inside the marketplace directory`](/docs/id/plugins/troubleshooting#plugins-with-relative-paths-fail-in-url-based-marketplaces). Berikan setiap entri source yang dapat diambil sendiri, seperti repositori `github` atau URL `archive`, atau host marketplace di repositori git sehingga Claude Code mengkloning seluruh pohon.

<h3 id="edit-plugins-in-place-on-a-shared-directory">
  Edit plugins in place on a shared directory
</h3>

Ketika pengguna menambahkan marketplace Anda dari direktori bersama, Claude Code membaca plugin dengan source path relatif langsung dari direktori itu alih-alih menyalinnya. Pengguna melihat edit Anda ketika mereka memulai sesi berikutnya atau menjalankan `/reload-plugins`, tanpa langkah pembaruan atau bump versi.

<h3 id="keep-plugin-files-out-of-git-lfs">
  Keep plugin files out of Git LFS
</h3>

Jauhkan file yang plugin Anda butuhkan dari [Git LFS](https://git-lfs.com). Ketika pengguna menambahkan marketplace yang dihosting di repositori git, atau memasang plugin berbasis git yang dicantumkannya, Claude Code mengkloning marketplace atau repositori plugin itu ke mesin mereka. Klon tidak pernah mengunduh konten LFS, jadi file yang dilacak LFS tiba sebagai file pointer.

<h3 id="share-files-within-a-marketplace-with-symlinks">
  Share files within a marketplace with symlinks
</h3>

Untuk berbagi file antara plugin Anda dan bagian lain dari marketplace yang sama, buat symbolic links di dalam direktori plugin Anda. Ketika Claude Code menyalin plugin ke cache-nya, ia menangani setiap symlink berdasarkan di mana target diselesaikan:

* **Dalam direktori plugin itu sendiri**: symlink dipertahankan sebagai symlink relatif dalam cache, jadi tetap menyelesaikan target yang disalin saat runtime.
* **Di tempat lain dalam marketplace yang sama**: symlink didereferensi. Konten target disalin ke cache di tempatnya. Ini memungkinkan direktori `skills/` meta-plugin untuk menghubungkan ke skills yang ditentukan oleh plugin lain di marketplace.
* **Di luar marketplace**: symlink dilewati untuk keamanan.

Untuk plugin yang dipasang dari path lokal, atau dari [`command` source](/docs/id/plugins/marketplace-reference#command-plugin-source) yang `mode`-nya adalah default `copy`, Claude Code hanya mempertahankan symlink yang diselesaikan dalam direktori plugin itu sendiri dan melewati semua yang lain.

Perintah berikut membuat link dari dalam plugin marketplace ke skill bersama yang ditentukan oleh plugin sibling. Di Windows, gunakan `mklink /D` dari Command Prompt yang ditinggikan atau aktifkan Developer Mode:

```bash theme={null}
ln -s ../../shared-plugin/skills/foo ./skills/foo
```

<h2 id="distribute-through-organization-settings">
  Distribute through organization settings
</h2>

Pada paket Team atau Enterprise, Anda juga dapat mendistribusikan marketplace melalui [**Organization settings > Plugins & skills**](https://claude.ai/admin-settings/skills?tab=inventory) di claude.ai alih-alih menghosting-nya di tempat pengguna menambahkannya sendiri. Organization sync membaca repositori melalui koneksi GitHub atau GitLab organisasi Anda di claude.ai, jadi kredensial git pengguna Anda tidak terlibat.

Organization sync lebih ketat tentang repositori daripada `/plugin marketplace add`:

* **Marketplace repository**: di github.com dan gitlab.com, harus pribadi atau internal
* **Plugin sources**: setiap plugin source harus bertipe `github`, `url`, atau `git-subdir`, atau [relative path](/docs/id/plugins/marketplace-reference#relative-path-plugin-source) yang dimulai dengan `./`
* **Top-level `bin/` directory**: claude.ai menolak plugin yang memilikinya dan mensinkronkan sisa marketplace. Pesan kesalahan dimulai dengan `Plugin contains a top-level bin/ directory`. Simpan executable di direktori lain, seperti `scripts/`, dan referensikan sebagai `${CLAUDE_PLUGIN_ROOT}/scripts/<name>` dari hooks atau konfigurasi server MCP Anda

Lihat [Manage plugins for your organization](https://support.claude.com/en/articles/13837433) untuk workflow admin.

<h2 id="grant-access-to-a-private-marketplace">
  Grant access to a private marketplace
</h2>

Ketika pengguna menambahkan, memasang dari, atau memperbarui marketplace Anda, Claude Code menjalankan `git` di mesin mereka dengan prompt interaktif dimatikan dan mengandalkan kredensial apa pun yang sudah dipegang mesin itu. Claude Code tidak memiliki token git sendiri, dan `marketplace.json` tidak memiliki field untuk satu.

Anda memilih apakah klon berjalan melalui SSH atau HTTPS berdasarkan bentuk perintah add yang Anda kirim pengguna:

* **GitHub `owner/repo`**: Claude Code menyelidiki `ssh -T git@github.com` dan mengkloning melalui SSH ketika penyelidikan berhasil. Jika penyelidikan gagal, atau klon SSH itu sendiri gagal, ia mengkloning melalui HTTPS. Pengguna di mesin tanpa kunci SSH GitHub dapat mengatur `CLAUDE_CODE_PLUGIN_PREFER_HTTPS=1` untuk melewati penyelidikan dan mengkloning melalui HTTPS.
* **`git@host:path.git`**: SSH.
* **`https://example.com/repo.git`**: HTTPS.

Beri tahu pengguna apa yang setiap protokol butuhkan di mesin mereka:

* **SSH**: kunci harus bekerja tanpa prompt passphrase, misalnya karena dimuat di `ssh-agent`. Host harus sudah ada di `known_hosts`.
* **HTTPS**: Claude Code membiarkan helper kredensial git pengguna diaktifkan tetapi melarangnya dari prompting. Kredensial yang sudah disimpan helper bekerja; satu yang harus diminta gagal. Di GitHub, `gh auth login` diikuti oleh `gh auth setup-git` menyimpan satu.

Untuk host GitHub Enterprise Server, pengguna memerlukan akses git ke host itu dari mesin mereka. Lihat [Plugin marketplaces on GHES](/docs/id/github-enterprise-server#plugin-marketplaces-on-ghes) untuk apa yang setiap permukaan Claude Code butuhkan untuk mencapai marketplace yang dihosting GHES.

Jika Anda mendistribusikan melalui **Organization settings > Plugins & skills** di claude.ai sebagai gantinya, kredensial git pengguna Anda tidak terlibat. Lihat [Distribute through organization settings](#distribute-through-organization-settings) untuk plugin source mana yang dapat pribadi di sana.

<h3 id="serve-users-who-have-no-git-host-account">
  Serve users who have no git-host account
</h3>

Pengguna tanpa akun git-host dapat menambahkan marketplace yang Anda layani sebagai URL `marketplace.json` atau dari direktori bersama, tetapi mereka hanya dapat memasang plugin yang entry source-nya juga dapat mereka jangkau. Entri yang menunjuk ke repositori `github` pribadi masih gagal saat instalasi untuk mereka, karena Claude Code mengambilnya dengan `git` non-interaktif yang sama yang digunakan untuk marketplace yang dihosting git.

Entry source ini tidak memerlukan akun git:

* **`archive`**: zip yang diunduh melalui HTTPS. Pengguna tidak memerlukan `git` atau akun, hanya akses jaringan ke URL. Memerlukan Claude Code v2.1.224 atau lebih baru. Pin setiap archive dengan `sha256` sehingga Claude Code menolak unduhan yang berubah. Untuk mengirim kredensial dengan unduhan, lihat [Authenticate archive downloads](#authenticate-archive-downloads).
* **Repositori git publik**: Claude Code mengkloning source `url` atau `git-subdir` publik melalui HTTPS tanpa kredensial ketika entri memberikan URL `https://`. Untuk source `github`, atau source `git-subdir` yang ditulis sebagai `owner/repo`, pengguna tanpa kunci SSH GitHub mengatur `CLAUDE_CODE_PLUGIN_PREFER_HTTPS=1`.

Untuk tim di satu jaringan, marketplace `directory` pada filesystem bersama juga bekerja tanpa akun git. Pengguna hanya memerlukan akses baca ke path.

<h3 id="what-background-auto-update-does-with-credentials">
  What background auto-update does with credentials
</h3>

Background auto-update adalah refresh tanpa pengawasan Claude Code dari marketplace dan plugin yang dipasang setelah sesi dimulai. Ini dimatikan untuk marketplace Anda sampai pengguna atau admin menyalakannya, seperti yang tercakup di bawah [Keep users up to date](#keep-users-up-to-date).

Ketika aktif untuk marketplace pribadi, pemeriksaan latar belakang untuk commit baru menggunakan helper kredensial git yang dikonfigurasi pengguna dan tidak pernah prompt. Setiap jenis remote dan helper memberikan hasil yang berbeda:

* **SSH remotes**: kunci yang dimuat di `ssh-agent` mengautentikasi pemeriksaan.
* **HTTPS remotes dengan kredensial yang disimpan**: helper yang dapat menyediakan kredensial yang disimpan tanpa prompting mengautentikasi pemeriksaan. Git Credential Manager, helper Keychain macOS, dan `git-credential-store` bekerja dengan cara ini setelah mereka memegang kredensial untuk host.
* **HTTPS remotes dengan helper yang perlu prompt**: helper tidak dapat menjawab di latar belakang. Pembaruan gagal diam-diam dan checkout yang ada tetap di tempat, jadi plugin pengguna terus bekerja dari status yang terakhir disinkronkan.

Setelah pemeriksaan, Claude Code melakukan salah satu dari berikut:

* **Checkout sudah terbaru**: Claude Code membiarkannya apa adanya.
* **Pemeriksaan menemukan commit baru, atau gagal karena tidak dapat menjangkau atau mengautentikasi ke remote**: Claude Code mengkloning marketplace lagi dan mengganti checkout yang ada dengan klon baru. Jika klon itu gagal, checkout yang ada tetap di tempat. Klon ulang dapat [time out on large repositories](/docs/id/plugins/troubleshooting#git-clone-timed-out-after-120s).

Untuk menjaga marketplace pribadi tetap terkini, pengguna dapat melakukan salah satu dari berikut:

* **Simpan kredensial**: masuk ke helper kredensial terlebih dahulu sehingga memegang kredensial untuk host. Untuk GitHub, jalankan `gh auth login`, kemudian `gh auth setup-git`.
* **Simpan checkout saat gagal**: jika pengguna mengatur `CLAUDE_CODE_PLUGIN_KEEP_MARKETPLACE_ON_FAILURE=1`, Claude Code menyimpan checkout yang ada tanpa mencoba klon ulang ketika pemeriksaan latar belakang tidak dapat menjangkau atau mengautentikasi ke remote. Plugin terus bekerja dari status yang terakhir disinkronkan.

Jika pengguna mengatur `GITHUB_TOKEN` atau token provider lain di lingkungan, itu saja tidak mengautentikasi pemeriksaan latar belakang. Token berlaku melalui helper kredensial, seperti helper CLI `gh`, yang membaca `GH_TOKEN` dan `GITHUB_TOKEN`.

<h2 id="roll-out-to-a-whole-company">
  Roll out to a whole company
</h2>

Meluncurkan plugin ke perusahaan melibatkan Anda sebagai pemilik marketplace, administrator yang mengontrol pengaturan terkelola, dan setiap orang yang menggunakan Claude Code. Anda dapat menjalankan peluncuran tanpa administrator, dalam hal ini setiap orang menambahkan marketplace dan memasang plugin sendiri.

| Siapa                     | Apa yang mereka lakukan                                                                                                                                                                 | Di mana tercakup                                                                                                                                          |
| :------------------------ | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Anda, pemilik marketplace | Simpan katalog di repositori yang hanya dapat dibaca perusahaan, kirim perintah add untuk host Anda, dan katakan apa yang setiap orang butuhkan di mesin mereka                         | [Host your marketplace](#host-your-marketplace) dan [Grant access to a private marketplace](#grant-access-to-a-private-marketplace)                       |
| Administrator             | Mendaftarkan marketplace dan menyalakan plugin-nya untuk semua orang dengan `extraKnownMarketplaces` dan `enabledPlugins` dalam pengaturan terkelola, dan mengatur `autoUpdate` di sana | [Require a marketplace and its plugins](/docs/id/plugins/org#require-a-marketplace-and-its-plugins) dan [Set update policy](/docs/id/plugins/org#set-update-policy) |
| Setiap orang              | Memerlukan akses baca ke repositori git pribadi, dengan kredensial sudah disimpan di mesin mereka. Tanpa administrator, mereka juga menjalankan perintah add dan install                | [Add a private marketplace](/docs/id/plugins/install#add-a-private-marketplace)                                                                                |

Untuk orang yang tidak memiliki akun git-host, bagian-bagian ini masing-masing mencakup satu cara untuk menjangkau mereka:

* **Entry sources yang tidak memerlukan akun git**: [Serve users who have no git-host account](#serve-users-who-have-no-git-host-account)
* **Direktori plugin yang sudah diisi sebelumnya**: [Seed containers and CI](/docs/id/plugins/org#seed-containers-and-ci), yang juga melayani pengguna yang tidak memiliki akun git-host
* **Pengaturan organisasi claude.ai**: [Distribute through organization settings](#distribute-through-organization-settings), di mana kredensial git pengguna Anda tidak terlibat

<h2 id="keep-users-up-to-date">
  Keep users up to date
</h2>

Perubahan Anda mencapai pengguna melalui background auto-update, setelah diaktifkan untuk marketplace Anda, atau ketika pengguna memperbarui plugin sendiri. Dalam kedua kasus pengguna mendapatkan salinan baru plugin hanya ketika versi yang dihitung berubah, seperti dijelaskan di bawah [Release a new version](#release-a-new-version).

<h3 id="turn-on-auto-update">
  Turn on auto-update
</h3>

Background auto-update dimatikan untuk marketplace Anda secara default, dan `marketplace.json` tidak memiliki field untuk menyalakannya. Pengguna atau admin menyalakannya:

* **Beri tahu pengguna untuk menyalakannya**: setiap pengguna pergi ke **Marketplaces** di `/plugin`, memilih marketplace Anda, dan memilih **Enable auto-update**.
* **Minta admin untuk mengaturnya**: jika admin mengatur `"autoUpdate": true` pada entri `extraKnownMarketplaces` marketplace Anda dalam pengaturan terkelola, itu aktif untuk semua orang yang menerima pengaturan itu. Lihat [Set update policy](/docs/id/plugins/org#set-update-policy).

Tanpa auto-update, pengguna menerima perubahan Anda ketika mereka menjalankan `/plugin marketplace update <name>` dalam sesi atau `claude plugin update <plugin>@<name>` di shell.

Untuk apa yang pengguna lihat ketika pembaruan mencapai mereka, lihat [When auto-update runs](/docs/id/plugins/loading#when-auto-update-runs).

<h3 id="release-a-new-version">
  Release a new version
</h3>

Untuk merilis versi baru kepada pengguna, ubah `version` plugin. Pengguna mendapatkan salinan baru hanya ketika versi yang dihitung plugin berbeda dari yang mereka miliki. Versi itu berasal dari `plugin.json` terlebih dahulu, kemudian dari entri marketplace, per [Versions and updates](/docs/id/plugins/loading#versions-and-updates).

Plugin yang pengguna [load in place](/docs/id/plugins/loading#find-plugins-on-disk) dari marketplace yang mereka tambahkan sebagai direktori lokal tidak dikendalikan oleh `version`. Ia memuat file Anda saat ini di setiap awal sesi, apa pun string versinya.

Untuk setiap instalasi selain in-place load atau satu dari `command` source, tingkatkan `version` pada setiap rilis atau hilangkan:

* **Bump `version` pada setiap rilis**: pengguna tetap pada salinan cache mereka sampai string berubah. Jika Anda mengatur `"version": "1.0.0"` dan push commit baru tanpa mengubahnya, pengguna tidak menerimanya.
* **Hilangkan `version`**: pengguna melacak commit Anda sebagai gantinya. Tinggalkan `version` dari `plugin.json` dan entri marketplace.

Jangan atur `version` di `plugin.json` dan entri marketplace. Jika Anda melakukannya, Claude Code menggunakan nilai `plugin.json` tanpa peringatan, dan `claude plugin validate` melaporkan ketidakcocokan sebagai `Entry declares version "<a>" but <path>/plugin.json says "<b>"`.

<h3 id="hold-users-on-one-version">
  Hold users on one version
</h3>

Satu marketplace melayani satu versi setiap plugin pada satu waktu, jadi Anda menahan pengguna pada versi dengan memilih apa yang setiap entri tunjuk:

* **`ref` dan `sha` pada entri plugin**: `ref` menamai branch atau tag dan `sha` menamai commit untuk source `github`, `url`, atau `git-subdir`. Lihat [Plugin sources](/docs/id/plugins/marketplace-reference#plugin-sources).
* **`#<ref>` pada perintah add**: pengguna yang menambahkan `your-org/your-marketplace#stable` mendapatkan branch atau tag itu dari katalog. Untuk dua baris rilis sekaligus, lihat [Run release channels](#run-release-channels).
* **`<plugin>--v<version>` tags**: rentang versi dependensi diselesaikan terhadap tag ini. Lihat [Release a plugin that others depend on](/docs/id/plugins/dependencies#tag-plugin-releases-for-version-resolution).

[Release a new version](#release-a-new-version) mengatakan kapan entri yang berubah mencapai pengguna.

<h3 id="change-the-command-of-a-command-source">
  Change the command of a command source
</h3>

Jika Anda mengubah `command` dari [`command` source](/docs/id/plugins/marketplace-reference#command-plugin-source), atau mengalihkan `mode`-nya, setiap pengguna harus menerima perintah baru sebelum Claude Code menjalankannya. Claude Code hanya menjalankan perintah yang tepat yang diterima pengguna ketika mereka memasang atau terakhir memperbarui plugin.

Setelah salinan marketplace pengguna mengambil perubahan, pengguna itu melihat berikut:

* **Tidak ada lagi background runs**: [once-per-session run](/docs/id/plugins/loading#when-a-command-source-re-runs) perintah berhenti untuk pengguna itu, jadi output baru tool tidak mencapai mereka.
* **Entri di tab `/plugin` Errors**: entri menunjukkan perintah baru dan perintah `claude plugin update` untuk dijalankan.

Beri tahu pengguna untuk menjalankan perintah `claude plugin update` yang ditunjukkan entri itu, di terminal. Claude Code menunjukkan perintah baru dan meminta mereka menerimanya.

<h2 id="run-release-channels">
  Run release channels
</h2>

Untuk menawarkan track stabil dan early-access, host dua marketplace yang entri-nya menunjuk ke ref berbeda dari plugin yang sama, dan biarkan setiap pengguna menambahkan yang mereka inginkan. Claude Code tidak memiliki konsep release-channel, dan satu marketplace melayani satu versi setiap plugin pada satu waktu.

Berikan dua file `marketplace.json` nilai `name` yang berbeda. Claude Code mengidentifikasi marketplace berdasarkan `name`-nya, jadi pengguna tidak dapat memiliki dua marketplace dengan nama yang sama terdaftar sekaligus.

Dengan dua katalog ini, pengguna yang menambahkan `stable-tools` memasang `code-formatter` dari branch `stable`, dan pengguna yang menambahkan `latest-tools` memasangnya dari `latest`:

```json theme={null}
{
  "name": "stable-tools",
  "owner": { "name": "Your Org" },
  "plugins": [
    { "name": "code-formatter", "source": { "source": "github", "repo": "your-org/code-formatter", "ref": "stable" } }
  ]
}
```

```json theme={null}
{
  "name": "latest-tools",
  "owner": { "name": "Your Org" },
  "plugins": [
    { "name": "code-formatter", "source": { "source": "github", "repo": "your-org/code-formatter", "ref": "latest" } }
  ]
}
```

Berikan dua ref versi `plugin.json` yang berbeda, atau hilangkan `version` sehingga commit SHA membedakan mereka. Pembaruan dideteksi dengan membandingkan versi, jadi ref yang bergerak tanpa perubahan versi meninggalkan pengguna pada salinan cache.

Untuk menetapkan channel ke grup pengguna alih-alih membiarkan pengguna memilih, admin memberikan setiap grup entri `extraKnownMarketplaces` yang cocok, seperti dijelaskan di bawah [Set update policy](/docs/id/plugins/org#set-update-policy).

<h2 id="rename-or-remove-a-plugin">
  Rename or remove a plugin
</h2>

`name` plugin adalah pengidentifinya. Pengguna mereferensikannya dalam kunci pengaturan `enabledPlugins` dan `pluginConfigs` dan di `/plugin install`, jadi mengubahnya merusak setiap instalasi yang ada.

Untuk mengubah label yang pengguna lihat di `/plugin` tanpa merusak apa pun, atur `displayName` di `plugin.json` dan simpan `name` tidak berubah.

<h3 id="migrate-users-with-a-renames-map">
  Migrate users with a renames map
</h3>

Ketika Anda harus mengubah `name`, tambahkan peta `renames` tingkat atas ke `marketplace.json` sehingga Claude Code memigrasikan pengguna yang ada alih-alih melaporkan [`Plugin "<name>" not found in marketplace`](/docs/id/plugins/troubleshooting#plugin-not-found-in-marketplace). Lakukan hal yang sama ketika Anda menghapus entri dari `plugins`. Migrasi otomatis memerlukan Claude Code v2.1.193 atau lebih baru.

Petakan setiap nama lama ke nama saat ini, atau ke `null` ketika plugin hilang. Marketplace ini mengganti nama `formatter` menjadi `code-formatter` dan mencatat bahwa `legacy-linter` dihapus:

```json theme={null}
{
  "name": "your-marketplace",
  "owner": { "name": "Your Org" },
  "plugins": [
    { "name": "code-formatter", "source": "./plugins/code-formatter" }
  ],
  "renames": {
    "formatter": "code-formatter",
    "legacy-linter": null
  }
}
```

Setelah Anda push, pengguna yang masih memiliki nama lama yang diaktifkan melihat salah satu hasil ini:

* **Entri yang diganti nama**: plugin memuat di bawah nama barunya. `claude plugin list` dan detail plugin di bawah `/plugin` menunjukkan `Renamed to "code-formatter" in the "your-marketplace" marketplace` sekali, dan Claude Code menulis ulang kunci lama ke yang baru di `enabledPlugins` dan `pluginConfigs` dalam scope pengaturan pengguna, proyek, dan lokal.
* **Entri `null`**: kunci lama dijatuhkan dari scope itu dan pengguna melihat `Removed from the "your-marketplace" marketplace`.
* **Diaktifkan dalam pengaturan terkelola**: plugin masih memuat di bawah nama barunya, tetapi Claude Code tidak dapat menulis ulang pengaturan terkelola, jadi pemberitahuan berulang sampai admin memperbarui `enabledPlugins` di sana.

Untuk marketplace yang pengguna tambahkan dari repositori git atau URL, plugin yang diganti nama melaporkan [`Plugin "<name>" not cached at <path>`](/docs/id/plugins/troubleshooting#plugin-not-cached-at) sampai pengguna menjalankan `/plugin install code-formatter@your-marketplace` sekali dalam sesi.

Perlakukan `renames` sebagai riwayat append-only. Simpan entri lama setelah semua orang bermigrasi. Ketika Anda mengganti nama lagi, tambahkan entri kedua daripada mengedit yang pertama, karena Claude Code mengikuti rantai dari nama tertua.

Di shell Anda, jalankan `claude plugin validate .` setelah mengedit peta. Ini menolak rantai yang bersiklus atau yang berakhir di mana pun selain `null` atau nama di `plugins`, dengan `renames.<name>: chain does not resolve`.

<h3 id="uninstall-removed-plugins-from-users’-machines">
  Uninstall removed plugins from users' machines
</h3>

Untuk mencopot plugin yang dihapus dari mesin pengguna alih-alih meninggalkan salinan di belakang, atur `"forceRemoveDeletedPlugins": true` di tingkat atas `marketplace.json`. Tanpa field, plugin yang dihapus tetap terpasang dan melaporkan `Plugin "<name>" not found in marketplace` ketika sesi memuat-nya. Dengan itu, Claude Code melakukan berikut di setiap awal sesi:

1. Membandingkan apa yang pengguna pasang dari marketplace Anda terhadap entri dan peta `renames`, dan memperlakukan plugin apa pun yang tidak tercantum atau diganti nama sebagai dihapus.
2. Mencopot setiap plugin yang dihapus dari scope pengguna, proyek, dan lokal. Plugin yang hanya pengaturan terkelola pasang tetap di tempat.
3. Mencantumkan setiap plugin yang dihapus di bawah heading **Flagged** di `/plugin` dengan status `Removed from marketplace`.

<h2 id="authenticate-archive-downloads">
  Autentikasi unduhan arsip
</h2>

Untuk mengautentikasi unduhan [`archive`](/docs/id/plugins/marketplace-reference#archive-plugin-source), seperti unduhan dari registri pribadi, atur header HTTP yang dikirim Claude Code dengannya. Anda dapat mengatur `headers` di salah satu tempat berikut:

* **Sumber `url` marketplace**: sumber `url` yang Anda daftarkan marketplace darinya, seperti entri [`extraKnownMarketplaces`](/docs/id/settings-reference#extraknownmarketplaces).
* **Entri plugin**: pada Claude Code v2.1.238 atau lebih baru, Anda dapat mengaturnya pada entri `marketplace.json` plugin sebagai gantinya, di samping `source`.

Di salah satu tempat, atur perintah `headersHelper` sebagai gantinya dari `headers` ketika nilainya berumur pendek, seperti token yang dihasilkan registri Anda atas permintaan. Claude Code menjalankan perintah dan mengirim objek JSON yang dicetak sebagai header tempat tersebut. Memerlukan Claude Code v2.1.238 atau lebih baru.

[Referensi marketplace](/docs/id/plugins/marketplace-reference#plugin-entries) mencantumkan bidang entri `headers` dan `headersHelper`.

Tempat yang Anda pilih menentukan unduhan mana yang mendapatkan header dan kapan Claude Code menjalankan perintah:

| Tempat                   | Unduhan yang mendapatkan header                                                  | Kapan Claude Code menjalankan `headersHelper` yang diatur di sana                                                                                                                     |
| :----------------------- | :------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Sumber `url` Marketplace | Unduhan arsip pada asal URL marketplace, artinya skema, host, dan port yang sama | Sebelum setiap pengambilan `marketplace.json` marketplace dan sebelum setiap unduhan arsip pada asal tersebut. Claude Code menggunakan kembali output satu run selama hingga 60 detik |
| Entri plugin             | Hanya unduhan entri tersebut                                                     | Hanya ketika pengguna memasang atau memperbarui plugin tersebut saja dan [menerima perintah](#how-users-accept-a-headershelper-command)                                               |

Jika kedua tempat mengatur header dengan nama yang sama, Claude Code mengirim nilai entri. Dalam satu tempat, header yang dicetak perintah menimpa header dengan nama yang sama yang tercantum dalam `headers`.

<h3 id="add-a-headershelper-to-a-plugin-entry">
  Tambahkan headersHelper ke entri plugin
</h3>

Entri ini mengatur `headersHelper` di samping `source`. Ini juga mengatur [`"strict": false`](/docs/id/plugins/marketplace-reference#strict-mode), yang Claude Code perlukan dari entri `marketplace.json` yang mengatur `headersHelper`:

```json theme={null}
{
  "name": "my-plugin",
  "description": "Formatting commands for internal services",
  "strict": false,
  "source": {
    "source": "archive",
    "url": "https://registry.example.com/plugins/my-plugin-2.1.0.zip"
  },
  "headersHelper": "/opt/bin/mint-registry-token.sh"
}
```

Untuk memeriksa entri, jalankan `claude plugin install my-plugin@your-marketplace` di shell Anda. Claude Code menunjukkan perintah dan URL arsip, dan mengunduh zip setelah Anda menerima.

<h3 id="write-the-headershelper-command">
  Tulis perintah headersHelper
</h3>

Baik Anda mengatur `headersHelper` pada sumber `url` marketplace atau pada entri plugin, tulis perintah untuk memenuhi persyaratan berikut:

* **Teks perintah**: paling banyak 500 karakter ASCII yang dapat dicetak, tanpa run empat atau lebih spasi.
* **Output**: cetak satu objek JSON dari nama header dan nilai string pada stdout, kemudian keluar 0 dalam 10 detik.
* **Shell dan direktori kerja**: Claude Code menjalankan perintah melalui `sh`, atau melalui `cmd.exe` di Windows. Direktori kerja adalah direktori konfigurasi, yaitu `~/.claude` atau [`CLAUDE_CONFIG_DIR`](/docs/id/env-vars#variables). Berikan jalur absolut atau perintah pada `PATH`, karena jalur relatif diselesaikan terhadap direktori tersebut, bukan proyek pengguna.
* **Variabel yang Claude Code hapus**: ketika perintah diatur dalam entri `marketplace.json`, atau dalam `.claude/settings.json` atau `.claude/settings.local.json` proyek, Claude Code menghapus dari lingkungan setiap variabel yang namanya terlihat seperti kredensial, dengan [aturan yang sama yang diterapkannya pada MCP `headersHelper`](/docs/id/mcp#which-variables-a-helper-can-read). `ANTHROPIC_API_KEY` dan `MY_REGISTRY_TOKEN` keduanya dihapus, jadi biarkan perintah membaca kredensialnya dari file atau penyimpanan kredensial. Penghapusan ini tidak berlaku untuk perintah yang diatur dalam pengaturan pengguna, file `--settings`, atau pengaturan terkelola.
* **Variabel yang Claude Code atur**: `CLAUDE_CODE_MARKETPLACE_URL` dan `CLAUDE_CODE_MARKETPLACE_NAME` untuk perintah sumber `url`, dan `CLAUDE_CODE_PLUGIN_NAME` dan `CLAUDE_CODE_PLUGIN_ARCHIVE_URL` untuk perintah entri. `CLAUDE_CODE_MARKETPLACE_NAME` tidak diatur pada pengambilan pertama setelah pengguna menambahkan marketplace berdasarkan URL, karena pengambilan tersebut adalah yang menyediakan nama.

Perintah yang mencetak token pembawa mencetak objek seperti ini:

```json theme={null}
{"Authorization": "Bearer eyJhbGciOiJSUzI1NiJ9"}
```

<h3 id="when-claude-code-skips-a-headershelper-command-or-drops-its-output">
  Kapan Claude Code melewati perintah headersHelper atau menghapus outputnya
</h3>

Perintah `headersHelper` tidak berjalan, atau header dari `headers` atau dari output perintah dihapus, ketika salah satu dari berikut berlaku:

* **Perintah gagal**: jika perintah keluar non-nol, berjalan melampaui 10 detik, atau mencetak apa pun selain objek JSON dari nilai string, pengambilan atau unduhan yang perintah dijalankan tidak terjadi.
* **URL Marketplace tidak dimulai dengan `https://`**: perintah sumber `url` tersebut tidak berjalan, dan permintaan hanya membawa header yang tercantum dalam bidang `headers`-nya.
* **Pengalihan meninggalkan asal**: ketika unduhan dialihkan dari asal URL arsip, permintaan yang dialihkan tidak membawa nilai `headers` atau output perintah dari sumber `url` marketplace atau entri plugin.
* **Entri mengatur header perutean atau identitas**: Claude Code menghapus nama perutean permintaan dan identitas klien seperti `Host`, `Cookie`, dan `X-Forwarded-*` dari `headers` entri dan output perintah, dan mempertahankan nama autentikasi seperti `Authorization`. Setiap entri `marketplace.json` disaring dengan cara ini. Untuk entri plugin inline dalam pengaturan, lihat [`extraKnownMarketplaces`](/docs/id/settings-reference#extraknownmarketplaces).
* **Perintah diatur dalam pengaturan direktori `--add-dir`**: perintah diabaikan, pada sumber `url` dan pada [entri plugin inline](/docs/id/settings-reference#extraknownmarketplaces) juga, dan hanya `headers` file tersebut yang dikirim.
* **Pengaturan terkelola memblokir perintah**: mengatur [`disableCommandPluginSources`](/docs/id/settings-reference#disablecommandpluginsources) ke `true` memblokir perintah `headersHelper`, dan [`allowManagedHooksOnly`](/docs/id/settings-reference#allowmanagedhooksonly) juga memblokir mereka kecuali `disableCommandPluginSources` secara eksplisit `false`. Di bawah salah satu blok, Claude Code masih menjalankan perintah untuk marketplace yang pengaturan terkelola sendiri nyatakan.

<h3 id="how-users-accept-a-headershelper-command">
  Bagaimana pengguna menerima perintah headersHelper
</h3>

Pengguna menerima perintah entri plugin setiap kali mereka memasang atau memperbarui plugin tersebut saja. Mereka melakukan itu dari tampilan plugin sendiri di `/plugin`, atau dengan `claude plugin install` atau `claude plugin update`. Claude Code menunjukkan perintah dan URL arsip, dan menjalankan perintah hanya setelah pengguna menerima.

Dalam shell non-interaktif, teruskan [`--yes`](/docs/id/plugins/cli-reference#plugin-install) untuk menerima perintah. Untuk menerima hanya perintah yang ditampilkan run `--json` sebelumnya, teruskan [`--accept-command`](/docs/id/plugins/cli-reference#plugin-install) dengan `sha256` yang dilaporkan run.

Claude Code menjalankan hanya perintah yang ditunjukkan, untuk URL arsip yang ditunjukkan. Jika perintah entri atau URL arsip berubah di antara, Claude Code menolak pemasangan atau pembaruan. Perubahan dalam string kueri saja tidak dihitung.

<h3 id="installs-and-updates-that-refuse-the-command-instead-of-asking">
  Pemasangan dan pembaruan yang menolak perintah alih-alih bertanya
</h3>

Pada operasi apa pun selain pemasangan atau pembaruan plugin tunggal, Claude Code tidak menjalankan perintah entri atau mengunduh arsipnya. Plugin tetap pada versi terpasang atau tetap tidak terpasang, dan pengguna melihat salah satu hasil berikut:

* **Memasang beberapa plugin sekaligus, dari saran plugin, atau sebagai ketergantungan plugin lain**: Claude Code menolak plugin yang memiliki perintah dan mengarahkan pengguna ke tampilan plugin sendiri di `/plugin`. Plugin lain dalam pemasangan massal masih terpasang. Plugin yang bergantung pada plugin yang ditolak gagal terpasang sampai pengguna memasang plugin yang ditolak sendiri.
* **Pembaruan otomatis latar belakang, atau awal sesi untuk plugin yang arsipnya tidak pernah diunduh**: Claude Code mencantumkan plugin dalam tab Errors `/plugin` sehingga pengguna tahu untuk memasang atau memperbarui sendiri.

<h3 id="when-a-marketplace-url-sources-command-runs">
  Kapan perintah sumber `url` marketplace berjalan
</h3>

Anda mendeklarasikan `headersHelper` sumber `url` marketplace dalam file pengaturan, seperti entri [`extraKnownMarketplaces`](/docs/id/settings-reference#extraknownmarketplaces), daripada dalam katalog yang dipublikasikan marketplace. Claude Code oleh karena itu tidak meminta pengguna untuk menerimanya pada setiap pemasangan atau pembaruan. Sebagai gantinya, file pengaturan yang mendeklarasikannya menentukan kapan Claude Code menjalankannya:

| File pengaturan                                                                   | Kapan Claude Code menjalankan perintah                                                                                                                                                                                                                           |
| :-------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Pengaturan pengguna, file `--settings`, atau file pengaturan terkelola pada mesin | Tanpa bertanya, termasuk selama penyegaran marketplace latar belakang                                                                                                                                                                                            |
| `.claude/settings.json` atau `.claude/settings.local.json` proyek                 | Hanya setelah pengguna menerima [dialog kepercayaan ruang kerja](/docs/id/permissions#what-runs-before-you-trust-a-folder) untuk folder itu sendiri. Sesi `-p` atau SDK tidak dihitung sebagai menerimanya, dan kepercayaan yang diberikan ke folder induk juga tidak |
| Pengaturan terkelola server                                                       | Dalam sesi interaktif, hanya setelah pengguna menyetujui pengaturan yang dikirimkan dalam [dialog persetujuan keamanan](/docs/id/server-managed-settings#security-approval-dialogs)                                                                                   |

Untuk [entri plugin inline](/docs/id/settings-reference#extraknownmarketplaces) dalam salah satu file ini, Claude Code memerlukan kepercayaan folder atau persetujuan pengaturan yang sama seperti untuk perintah tingkat marketplace dalam file tersebut, dan pengguna juga menerima perintah entri pada setiap pemasangan atau pembaruan.

<h2 id="depend-on-and-recommend-other-plugins">
  Depend on and recommend other plugins
</h2>

Entri dapat mendeklarasikan dependensi pada plugin lain.

* **Rentang versi**: dependensi dapat membawa rentang semver.
* **Dependensi lintas marketplace**: dependensi dari marketplace lain memasang hanya ketika marketplace Anda mencantumkan marketplace itu di `allowCrossMarketplaceDependenciesOn`.

Untuk rentang versi, konvensi tag git `<plugin>--v<version>` yang diselesaikan, dan kepercayaan lintas marketplace, lihat [Plugin dependencies](/docs/id/plugins/dependencies).

Untuk membuat Claude Code menyarankan plugin ketika proyek cocok dengannya, tambahkan blok `relevance` ke entri dengan sinyal yang mengidentifikasi proyek. Pengguna melihat saran dari marketplace Anda hanya ketika admin mencantumkannya di `pluginSuggestionMarketplaces`. Untuk sinyal dan langkah enablement, lihat [Plugin relevance](/docs/id/plugins/relevance).

<h2 id="work-around-what-a-marketplace-can’t-do">
  Work around what a marketplace can't do
</h2>

Beberapa hal yang diminta pemilik tidak memiliki field di `marketplace.json`. Berikut adalah opsi terdekat untuk masing-masing:

* **Batasi apa yang pengguna lain pasang**: daftar allowlist marketplace adalah pengaturan terkelola, `strictKnownMarketplaces`. Lihat [Restrict what users can install](/docs/id/plugins/org#restrict-what-users-can-install).
* **Pasang atau aktifkan plugin tanpa pengguna bertanya**: tidak ada field entri yang memasang plugin. `enabledPlugins` terkelola melakukan itu untuk armada; lihat [Pre-install and require plugins](/docs/id/plugins/org#pre-install-and-require-plugins).
* **Tampilkan entri berbeda kepada pengguna berbeda**: entri tidak membawa field audiens, dan setiap pengguna yang menambahkan marketplace melihat seluruh katalog. Host marketplace terpisah untuk audiens terpisah.
* **Tandai plugin sebagai deprecated**: tidak ada status deprecation. Opsinya adalah menghapus entri, memetakan nama-nya ke `null` di `renames`, dan secara opsional mengatur `forceRemoveDeletedPlugins`.
* **Nyalakan auto-update untuk pengguna Anda**: setiap pengguna menyalakannya di bawah **Marketplaces** di `/plugin`, atau admin mengatur `autoUpdate` dalam pengaturan terkelola. Lihat [Turn on auto-update](#turn-on-auto-update).
* **Bawa kredensial git**: tidak ada field marketplace yang menyimpan token git. Akses ke marketplace atau plugin yang dihosting git mengikuti setup git pengguna, per [Grant access to a private marketplace](#grant-access-to-a-private-marketplace). Untuk source `archive`, entri dapat mengatur [`headers` atau `headersHelper`](#authenticate-archive-downloads) sebagai gantinya.

<h2 id="next-steps">
  Next steps
</h2>

* [Marketplace reference](/docs/id/plugins/marketplace-reference): field `marketplace.json`, tipe source, dan pesan validasi
* [Manage plugins for your organization](/docs/id/plugins/org): memerlukan, membatasi, atau seed marketplace Anda di seluruh mesin organisasi Anda
* [Plugin dependencies](/docs/id/plugins/dependencies): tag rilis sehingga plugin yang bergantung pada Anda dapat menyelesaikan versi
* [Troubleshoot plugins](/docs/id/plugins/troubleshooting): kesalahan yang pengguna Anda lihat saat menambahkan atau memperbarui dari marketplace Anda
