> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Dependensi plugin

> Deklarasikan plugin yang plugin Anda bergantung padanya, dengan rentang versi seperti ^1.2, dan lihat bagaimana Claude Code menginstal, menyelesaikan, dan memangkas dependensi tersebut.

Dependensi plugin adalah plugin lain yang plugin Anda andalkan, seperti plugin yang memanggil MCP server atau skill-nya. Setiap dependensi melacak versi terbaru yang disediakan marketplace-nya kecuali Anda mendeklarasikan batasan versi, rentang semantic-version seperti `^2.0` atau `~2.1.0` yang telah Anda uji.

Halaman ini adalah untuk penulis plugin yang mendeklarasikan dependensi dalam `plugin.json` dan untuk pengelola marketplace yang menandai rilis.

<Note>
  Kasus-kasus ini tercakup di halaman lain:

  * **Menginstal plugin yang memiliki dependensi**: lihat [Kelola plugin yang diinstal](/docs/id/plugins/install#manage-installed-plugins)
  * **Membaca kesalahan dependensi**: lihat [Kesalahan dependensi](/docs/id/plugins/troubleshooting#dependency-errors)
  * **Mendeklarasikan paket npm dan Bun yang kode plugin Anda sendiri butuhkan**: lihat [Dependensi paket Node.js](/docs/id/plugins/loading#node-js-package-dependencies)
</Note>

Untuk menambahkan batasan, mulai dari [Deklarasikan dependensi dengan batasan versi](#declare-a-dependency-with-a-version-constraint). Jika Anda mengelola plugin yang bergantung padanya oleh orang lain, [tandai rilis Anda](#tag-plugin-releases-for-version-resolution) sehingga batasan mereka dapat diselesaikan.

<h2 id="declare-dependencies">
  Deklarasikan dependensi
</h2>

<span id="decide-whether-to-constrain-dependency-versions" />Tanpa batasan versi, dependensi bergerak ke setiap rilis baru yang marketplace-nya terbitkan saat pengguna memperbarui. Jika rilis itu mengganti nama alat MCP yang plugin Anda panggil, plugin Anda rusak untuk semua orang yang memperbarui.

Dengan batasan seperti `~2.1.0` pada dependensi dari sumber yang didukung git, pengguna yang memiliki plugin Anda terinstal terus menerima patch `2.1.x` dari dependensi dan tidak pernah pindah ke `2.2`. Untuk meningkatkan sesuai jadwal Anda sendiri, uji terhadap rilis yang lebih baru dan kemudian terbitkan versi baru plugin Anda dengan batasan yang lebih luas.

<h3 id="declare-a-dependency-with-a-version-constraint">
  Deklarasikan dependensi dengan batasan versi
</h3>

Daftar dependensi dalam array `dependencies` dari `.claude-plugin/plugin.json` plugin Anda. Manifes berikut mendeklarasikan satu dependensi tanpa versi dan satu dependensi terbatas:

```json .claude-plugin/plugin.json theme={null}
{
  "name": "deploy-kit",
  "version": "3.1.0",
  "dependencies": [
    "audit-logger",
    { "name": "secrets-vault", "version": "~2.1.0" }
  ]
}
```

Entri dapat berupa string: nama plugin saja, seperti `"audit-logger"` dalam manifes ini, atau `"name@marketplace"` untuk menyelesaikannya di marketplace lain. Dengan string kosong, plugin Anda bergantung pada versi apa pun yang disediakan marketplace plugin tersebut.

Untuk menetapkan batasan versi, gunakan objek dengan bidang-bidang ini, masing-masing berupa string:

| Bidang        | Deskripsi                                                                                                                                                                                                                                                                                      |
| :------------ | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`        | Nama plugin dependensi, seperti yang muncul dalam entri marketplace-nya. Claude Code mencarinya di marketplace yang sama dengan plugin yang mendeklarasikan kecuali Anda menetapkan `marketplace`. Diperlukan.                                                                                 |
| `version`     | Rentang [semantic-version](https://github.com/npm/node-semver#ranges) seperti `~2.1.0`, `^2.0`, `>=1.4`, atau `=2.1.0`. Dependensi menginstal pada tag git tertinggi yang memenuhi rentang ini, jadi pengelola dependensi harus [menandai rilis](#tag-plugin-releases-for-version-resolution). |
| `marketplace` | Marketplace berbeda untuk menyelesaikan `name` di dalamnya. Daftar izin mengontrol dependensi lintas-marketplace, dijelaskan dalam [Bergantung pada plugin dari marketplace lain](#depend-on-a-plugin-from-another-marketplace).                                                               |

Rentang tidak cocok dengan versi pra-rilis seperti `2.0.0-beta.1` kecuali Anda memilih dengan akhiran pra-rilis seperti `^2.0.0-0`.

<h3 id="bundle-plugins-for-a-team">
  Bundel plugin untuk tim
</h3>

Untuk memungkinkan insinyur menginstal set plugin yang dikurasi dengan satu perintah, terbitkan plugin yang manifesnya berisi `name` dan array `dependencies`. Manifes plugin hanya membutuhkan `name`, jadi ini adalah plugin yang valid, dan menginstalnya menginstal setiap dependensi.

Misalnya, tim platform dapat menerbitkan bundel khusus peran di marketplace internal sehingga insinyur menjalankan satu `claude plugin install` alih-alih menginstal setiap plugin secara terpisah:

```json .claude-plugin/plugin.json theme={null}
{
  "name": "backend-standard",
  "version": "1.0.0",
  "description": "Standard plugin set for backend engineers",
  "dependencies": [
    "secrets-vault",
    "deploy-kit",
    { "name": "db-migrate", "version": "^3.0" },
    "oncall-runbook"
  ]
}
```

Untuk menambahkan plugin ke set standar nanti, terbitkan versi `backend-standard` baru dengan dependensi tambahan. Ketika marketplace tidak [auto-update secara default](/docs/id/plugins/loading#which-marketplaces-and-plugins-auto-update), insinyur baik mengaktifkan auto-update untuk marketplace atau memperbarui secara manual:

* **Aktifkan auto-update untuk marketplace**: pembaruan auto-update berikutnya memindahkan bundel ke versi baru dan menginstal dependensi apa pun yang ditambahkannya.
* **Perbarui secara manual**: jalankan `claude plugin update backend-standard` di shell, kemudian `/reload-plugins` dalam sesi terbuka untuk menginstal dependensi yang baru ditambahkan.

Untuk langkah-langkah sisi insinyur, lihat [Jaga plugin tetap diperbarui](/docs/id/plugins/install#keep-plugins-updated).

Untuk menerapkan bundel ke semua orang dalam organisasi, administrator menambahkannya ke `enabledPlugins` dalam pengaturan terkelola. Lihat [Pra-instal dan butuhkan plugin](/docs/id/plugins/org#pre-install-and-require-plugins).

<h3 id="depend-on-a-plugin-from-another-marketplace">
  Bergantung pada plugin dari marketplace lain
</h3>

Secara default, Claude Code tidak menginstal dependensi dari marketplace berbeda dari marketplace plugin yang mendeklarasikan sendiri, kecuali pengguna sudah memiliki dependensi itu terinstal dan diaktifkan pada cakupan yang sama. Default ini mencegah satu marketplace secara diam-diam menginstal plugin dari sumber yang belum ditinjau pengguna.

Untuk memungkinkan instalasi, tambahkan nama marketplace target ke `allowCrossMarketplaceDependenciesOn` dalam `marketplace.json` marketplace root. Marketplace root adalah yang menghosting plugin yang pengguna instal. Hanya daftar izin marketplace root yang berlaku.

`marketplace.json` berikut memungkinkan `deploy-kit` bergantung pada plugin dari `your-shared-marketplace`:

```json .claude-plugin/marketplace.json theme={null}
{
  "name": "your-marketplace",
  "owner": { "name": "Your Org" },
  "allowCrossMarketplaceDependenciesOn": ["your-shared-marketplace"],
  "plugins": [
    {
      "name": "deploy-kit",
      "source": "./deploy-kit",
      "dependencies": [
        { "name": "audit-logger", "marketplace": "your-shared-marketplace" }
      ]
    }
  ]
}
```

Jika `allowCrossMarketplaceDependenciesOn` hilang atau tidak menyertakan marketplace target, Claude Code tidak menginstal dependensi. Ketika dependensi dideklarasikan dalam entri marketplace, instalasi itu sendiri ditolak dengan pesan yang dimulai `Dependency "audit-logger@your-shared-marketplace" (required by deploy-kit@your-marketplace) is in marketplace "your-shared-marketplace", which is not in the allowlist` dan menyebutkan bidang yang akan ditetapkan. Ketika dideklarasikan dalam `plugin.json`, instalasi selesai tanpa dependensi dan plugin Anda kemudian gagal dimuat.

Pemeriksaan daftar izin tidak berlaku untuk dependensi yang sudah diaktifkan. Jika pengguna menginstal `audit-logger` dari `your-shared-marketplace` sendiri terlebih dahulu, pada cakupan yang sama, `deploy-kit` kemudian menginstal tanpa perubahan apa pun pada daftar izin.

<h3 id="test-a-plugin-and-its-dependency-locally">
  Uji plugin dan dependensinya secara lokal
</h3>

Jika Anda mengembangkan plugin dan plugin yang bergantung padanya pada saat yang sama, mulai Claude Code dari shell Anda dan muat keduanya dengan [`--plugin-dir`](/docs/id/plugins/cli-reference#flags-that-load-a-plugin-for-one-session):

```bash theme={null}
claude --plugin-dir ./my-dependency --plugin-dir ./my-plugin
```

Salinan lokal dependensi memenuhi entri dependensi plugin Anda, jadi Anda tidak perlu menginstal dependensi dari marketplace-nya.

* **Tidak ada `version` yang diperlukan**: `plugin.json` lokal juga tidak perlu `version`, karena [batasan versi](#declare-a-dependency-with-a-version-constraint) tidak diperiksa terhadap salinan lokal.
* **Entri yang menyebutkan marketplace**: entri yang menyebutkan marketplace juga cocok dengan salinan lokal pada Claude Code v2.1.242 atau lebih baru.

Sampai Anda menginstal dependensi dari marketplace-nya, plugin Anda berhenti dimuat setiap kali salinan lokal dinonaktifkan atau tidak ada:

* **Anda menonaktifkan salinan lokal**: plugin Anda dinonaktifkan pada pemuatan plugin berikutnya, dengan kesalahan yang berakhir `is disabled — enable it or remove the dependency`. Ketika kesalahan menyebutkan dependensi sebagai `<name>@inline`, pengidentifikasi itu merujuk pada salinan `--plugin-dir`.
* **Anda memulai sesi tanpa flag `--plugin-dir` dependensi**: kesalahan melaporkan dependensi sebagai tidak terinstal. Lewatkan flag lagi, atau instal dependensi dari marketplace-nya.

Ketika kedua plugin berada dalam satu folder induk, Anda dapat meneruskan folder itu ke `--plugin-dir` sekali. Jika folder itu sendiri bukan plugin, Claude Code memuat setiap folder anak yang memiliki `.claude-plugin/plugin.json`. Memerlukan Claude Code v2.1.265 atau lebih baru.

<h2 id="tag-plugin-releases-for-version-resolution">
  Rilis plugin yang bergantung pada orang lain
</h2>

Jika Anda mengelola plugin yang plugin lain bergantung padanya dengan batasan versi, tandai rilis-nya sehingga batasan itu dapat diselesaikan. Batasan diselesaikan terhadap tag git pada repositori yang menghosting plugin. Tandai repositori yang [sumber plugin](/docs/id/plugins/marketplace-reference#plugin-sources) plugin dalam `marketplace.json` tunjuk:

* **Sumber `github`, `url`, atau `git-subdir`**: repositori plugin itu sendiri, jadi penulis plugin membuat tag
* **Jalur relatif seperti `./plugins/secrets-vault`**: repositori marketplace, jadi pengelola marketplace membuat tag

<h3 id="create-a-release-tag">
  Buat tag rilis
</h3>

Tandai setiap rilis sebagai `<plugin-name>--v<version>`, di mana `<version>` cocok dengan bidang `version` dalam `plugin.json` commit itu. Awalan plugin-name memungkinkan satu repositori marketplace menghosting beberapa plugin dengan riwayat versi independen.

Buat tag dari direktori plugin, dengan remote `origin` yang dikonfigurasi untuk menerima tag yang didorong, menggunakan [`claude plugin tag`](/docs/id/plugins/cli-reference#plugin-tag):

```bash theme={null}
claude plugin tag --push
```

Perintah membangun nama tag dari manifes plugin. Sebelum membuat tag, ia menjalankan pemeriksaan ini:

* Memvalidasi plugin
* Memeriksa bahwa `plugin.json` dan entri marketplace setuju pada versi, ketika direktori plugin berada di dalam checkout marketplace
* Memerlukan pohon kerja yang bersih di bawah direktori plugin
* Menolak jika tag sudah ada

Jalankan yang berhasil mencetak `Created tag secrets-vault--v2.1.0`. Dengan `--push`, ia juga mencetak `Pushed to origin`. Tanpa `--push`, ia mencetak perintah `git push` untuk dijalankan sendiri.

Lewatkan `--dry-run` untuk melihat rencana tanpa membuat apa pun.

Referensi [`claude plugin tag`](/docs/id/plugins/cli-reference#plugin-tag) mencantumkan flag yang tersisa.

Anda juga dapat menjalankan `git tag secrets-vault--v2.1.0` secara langsung, selama Anda menjaga `version` dalam `plugin.json` dan dalam entri marketplace tetap sinkron sendiri.

<h3 id="constrain-a-dependency-that-has-a-non-git-source">
  Batasi dependensi yang memiliki sumber non-git
</h3>

Resolusi berbasis tag hanya berlaku untuk sumber yang didukung git. Untuk dependensi dengan sumber plugin `npm`, `archive`, atau `command` [](/docs/id/plugins/marketplace-reference#plugin-sources), batasan tidak mengontrol versi mana yang diambil. Ini masih diperiksa ketika plugin dimuat, dan plugin dependen dinonaktifkan jika versi yang terinstal tidak memenuhinya.

Untuk sumber `npm`, `archive`, dan `command`, versi yang diperiksa adalah `version` dalam `plugin.json` dependensi. Tetapkan satu di sana sebelum Anda membatasi dependensi itu, karena `plugin.json` yang tidak menetapkan versi tidak memenuhi batasan apa pun.

Claude Code tidak pernah menginstal dependensi dengan sumber `command` itu sendiri, jadi pengguna [menginstalnya terlebih dahulu](/docs/id/plugins/marketplace-reference#command-plugin-source). Ini juga tidak pernah menjalankan [`headersHelper`](/docs/id/plugins/host-marketplace#authenticate-archive-downloads) dependensi, jadi pengguna juga menginstal dependensi yang entri marketplace-nya menetapkan satu sebelum mereka menginstal plugin Anda.

Selain `claude plugin install`, operasi ini juga menginstal dependensi yang dideklarasikan yang hilang, dan batasan `command` dan `headersHelper` berlaku untuk mereka juga:

* `/reload-plugins`
* Auto-update dari marketplace plugin dependen
* Menjalankan kembali `claude plugin install` pada plugin dependen
* `claude plugin marketplace add`

<h2 id="how-dependencies-behave-for-your-users">
  Bagaimana dependensi berperilaku untuk pengguna Anda
</h2>

Bagian-bagian ini menjelaskan bagaimana Claude Code menyelesaikan, memeriksa, dan menggabungkan batasan yang Anda deklarasikan setelah plugin Anda diinstal bersama yang lain.

<h3 id="how-a-constraint-resolves-against-tags">
  Bagaimana batasan diselesaikan terhadap tag
</h3>

Ketika pengguna menginstal plugin yang mendeklarasikan `{ "name": "secrets-vault", "version": "~2.1.0" }`, dependensi menginstal dari tag `secrets-vault--v` tertinggi yang memenuhi `~2.1.0` pada repositori yang menghosting `secrets-vault`. Ketika tidak ada tag yang memenuhi rentang, instalasi baik gagal atau menggunakan salinan marketplace saat ini:

* **Plugin dengan repositori sendiri**: instalasi gagal dengan pesan yang berisi `Dependency "secrets-vault@your-marketplace" has no git tag satisfying`.
* **Plugin yang direferensikan oleh jalur relatif**: instalasi menggunakan salinan marketplace saat ini sebagai gantinya, dan batasan diperiksa ketika plugin dimuat. Jika salinan itu berada di luar rentang, plugin dependen tetap dinonaktifkan dan `claude plugin list` menunjukkan `Requires "secrets-vault@your-marketplace" ~2.1.0, installed 3.0.0`.

Untuk plugin yang direferensikan marketplace oleh jalur folder lokal, marketplace yang Anda tambahkan sebagai jalur folder lokal juga menyelesaikan batasan terhadap tag git folder itu, ketika folder adalah repositori git. Ini memerlukan Claude Code v2.1.196 atau lebih baru. Folder lokal yang bukan repositori git tidak memiliki tag, jadi Claude Code menginstal dependensi dari konten folder saat ini sebagai gantinya.

<h3 id="confirm-the-resolved-version">
  Konfirmasi versi yang diselesaikan
</h3>

Untuk mengonfirmasi versi mana yang diselesaikan batasan, jalankan `claude plugin list` di shell Anda. Dependensi yang diselesaikan tag menunjukkan versinya dengan akhiran commit 12 karakter, seperti `2.1.0-8713c5b11005`.

Pemeriksaan batasan menggunakan versi tag daripada `version` dalam `plugin.json`, bahkan jika `plugin.json` pada commit itu tertinggal.

Jika Anda memaksa-pindahkan tag ke commit berbeda, instalasi berikutnya mengambil konten commit itu alih-alih menggunakan kembali salinan cache yang basi. Lihat [Versi dan pembaruan](/docs/id/plugins/loading#versions-and-updates) untuk bagaimana versi plugin menjadi kunci cache-nya.

<h3 id="combine-constraints-from-several-plugins">
  Gabungkan batasan dari beberapa plugin
</h3>

Ketika beberapa plugin yang terinstal membatasi dependensi yang sama, dependensi diselesaikan ke versi tertinggi yang memenuhi semua rentang mereka. Kombinasi umum diselesaikan seperti ini:

| Plugin A memerlukan | Plugin B memerlukan | Hasil                                                                                                                            |
| :------------------ | :------------------ | :------------------------------------------------------------------------------------------------------------------------------- |
| `^2.0`              | `>=2.1`             | Satu instalasi pada tag `2.x` tertinggi pada atau di atas `2.1.0`. Kedua plugin dimuat.                                          |
| `~2.1`              | `~3.0`              | Menginstal plugin B gagal dengan pesan `has conflicting version requirements`. Plugin A dan dependensi tetap seperti sebelumnya. |
| `=2.1.0`            | tidak ada           | Dependensi tetap pada `2.1.0`. Auto-update melewati versi yang lebih baru saat plugin A terinstal.                               |

Auto-update mengambil dependensi terbatas pada tag git tertinggi yang memenuhi rentang setiap plugin yang terinstal, daripada pada versi terbaru marketplace. Jika rentang plugin yang terinstal tidak tumpang tindih, auto-update meninggalkan dependensi itu pada versi saat ini, dan tab **Errors** `/plugin` menunjukkan entri yang menyebutkan plugin yang membatasi. Jika mereka tumpang tindih tetapi tidak ada tag yang jatuh dalam rentang, auto-update mengambil salinan marketplace saat ini dan melewati pembaruan ketika `version` salinan itu jatuh di luar rentang plugin yang terinstal apa pun.

Ketika pengguna mencopot plugin terakhir yang membatasi dependensi, dependensi tidak lagi dibatasi ke rentang versi dan melanjutkan pelacakan entri marketplace-nya pada pembaruan berikutnya.

<h2 id="see-also">
  Lihat juga
</h2>

* [`claude plugin prune`](/docs/id/plugins/cli-reference#plugin-prune): hapus dependensi yang diinstal otomatis yang tidak diperlukan plugin lagi
* [Hosting marketplace](/docs/id/plugins/host-marketplace): saluran rilis dan merekomendasikan plugin lain
