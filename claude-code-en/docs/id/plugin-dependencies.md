> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Batasi versi dependensi plugin

> Deklarasikan batasan versi pada dependensi plugin, dan bundel satu set plugin yang dikurasi di balik satu instalasi.

Sebuah plugin dapat bergantung pada plugin lain dengan mencantumkannya di `plugin.json` atau di entri marketplacenya. Secara default, dependensi melacak versi terbaru yang tersedia, sehingga rilis upstream dapat mengubah dependensi di bawah plugin Anda tanpa peringatan. Batasan versi memungkinkan Anda menahan dependensi pada rentang versi yang telah diuji sampai Anda memilih untuk pindah.

Ketika Anda menginstal plugin yang mendeklarasikan dependensi, Claude Code menyelesaikan dan menginstal dependensi tersebut secara otomatis dan mencantumkan dependensi mana yang ditambahkan di akhir output instalasi. Jika dependensi kemudian hilang, `/reload-plugins` dan pembaruan plugin otomatis latar belakang akan menginstal ulangnya, asalkan marketplacenya sudah ada di marketplace yang Anda konfigurasi. Menjalankan kembali `claude plugin install` pada plugin yang bergantung, atau menambahkan marketplace dengan `claude plugin marketplace add`, juga menyelesaikan dependensi yang belum terselesaikan. Dependensi dari marketplace yang belum Anda tambahkan dibiarkan tidak terselesaikan.

Panduan ini ditujukan untuk penulis plugin yang mendeklarasikan dependensi di `plugin.json` dan untuk pengelola marketplace yang menandai rilis. Untuk menginstal plugin yang memiliki dependensi, lihat [Temukan dan instal plugin](/docs/id/discover-plugins). Untuk skema manifes lengkap, lihat [Referensi Plugin](/docs/id/plugins-reference).

<h2 id="why-constrain-dependency-versions">
  Mengapa membatasi versi dependensi
</h2>

Pertimbangkan marketplace internal di mana dua tim menerbitkan plugin. Tim platform memelihara `secrets-vault`, server MCP yang membungkus backend rahasia. Tim deploy memelihara `deploy-kit`, yang memanggil `secrets-vault` untuk mengambil kredensial selama deploy.

`deploy-kit` diuji terhadap `secrets-vault` v2.1.0. Tanpa batasan versi, saat tim platform menandai rilis berikutnya yang mengganti nama alat MCP, auto-update memindahkan `secrets-vault` setiap insinyur ke versi baru dan `deploy-kit` rusak.

Dengan batasan versi, `deploy-kit` mendeklarasikan bahwa ia memerlukan `secrets-vault` dalam rentang `~2.1.0`. Insinyur dengan `deploy-kit` yang diinstal tetap berada di patch `2.1.x` tertinggi yang cocok. Tim deploy meningkatkan sesuai jadwal mereka sendiri dengan menerbitkan versi `deploy-kit` baru dengan batasan yang lebih luas.

<h2 id="declare-a-dependency-with-a-version-constraint">
  Deklarasikan dependensi dengan batasan versi
</h2>

Cantumkan dependensi dalam array `dependencies` dari `plugin.json` plugin Anda. Setiap entri adalah nama plugin atau objek dengan batasan versi.

Manifes berikut mendeklarasikan satu dependensi tanpa versi dan satu dependensi terbatas:

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

Entri dapat berupa string kosong dengan hanya nama plugin, seperti `"audit-logger"` dalam contoh di atas, yang bergantung pada versi apa pun yang disediakan marketplace plugin tersebut. Untuk kontrol lebih, gunakan objek dengan bidang-bidang ini:

| Bidang        | Tipe   | Deskripsi                                                                                                                                                                                                                                                                   |
| :------------ | :----- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`        | string | Nama plugin. Diselesaikan dalam marketplace yang sama dengan plugin yang mendeklarasikan. Diperlukan.                                                                                                                                                                       |
| `version`     | string | Rentang [semver](https://github.com/npm/node-semver#ranges) seperti `~2.1.0`, `^2.0`, `>=1.4`, atau `=2.1.0`. Dependensi diambil pada versi tertinggi yang ditandai yang memenuhi rentang ini.                                                                              |
| `marketplace` | string | Marketplace berbeda untuk menyelesaikan `name` di dalamnya. Dependensi lintas-marketplace diblokir kecuali marketplace target tercantum dalam [`allowCrossMarketplaceDependenciesOn`](#depend-on-a-plugin-from-another-marketplace) di `marketplace.json` marketplace root. |

Bidang `version` menerima ekspresi apa pun yang didukung oleh paket `semver` Node, termasuk rentang caret, tilde, hyphen, dan comparator. Versi pra-rilis seperti `2.0.0-beta.1` dikecualikan kecuali rentang Anda memilih dengan sufiks pra-rilis seperti `^2.0.0-0`.

<h2 id="bundle-plugins-for-a-team">
  Bundel plugin untuk tim
</h2>

Selain `name` yang diperlukan, manifes plugin dapat terdiri dari hanya array `dependencies`. Menginstalnya akan menarik setiap dependensi, yang menjadikannya cara untuk mengemas set plugin yang dikurasi di balik satu instalasi.

Misalnya, tim platform dapat menerbitkan bundel khusus peran di marketplace internal sehingga insinyur menjalankan satu `claude plugin install` alih-alih menginstal setiap alat secara terpisah:

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

Menginstal `backend-standard` menyelesaikan dan menginstal keempat dependensi.

Untuk menambahkan alat ke set standar nanti, terbitkan versi `backend-standard` baru dengan dependensi tambahan. Auto-update dimatikan secara default untuk marketplace non-Anthropic, jadi insinyur mengambil versi baru dengan salah satu dari dua cara:

* Aktifkan auto-update untuk marketplace di `/plugin`. Auto-update berikutnya memindahkan bundel ke versi baru dan menginstal dependensi apa pun yang ditambahkannya.
* Jalankan `claude plugin update backend-standard`, kemudian `/reload-plugins` untuk menginstal dependensi yang baru ditambahkan.

Untuk meluncurkan bundel di seluruh organisasi, tambahkan plugin bundel ke `enabledPlugins` dalam [pengaturan terkelola](/docs/id/settings#enabledplugins).

<h2 id="depend-on-a-plugin-from-another-marketplace">
  Bergantung pada plugin dari marketplace lain
</h2>

Secara default, Claude Code menolak untuk auto-install dependensi yang berada di marketplace berbeda dari plugin yang mendeklarasikannya. Ini mencegah satu marketplace secara diam-diam menarik plugin dari sumber yang belum Anda tinjau.

Untuk mengizinkannya, pengelola marketplace root menambahkan nama marketplace target ke `allowCrossMarketplaceDependenciesOn` di `marketplace.json`. Marketplace root adalah yang menghosting plugin yang diinstal pengguna; hanya daftar putihnya yang dikonsultasikan, sehingga kepercayaan tidak berantai melalui marketplace perantara.

`marketplace.json` berikut memungkinkan `deploy-kit` bergantung pada plugin dari `acme-shared`:

```json .claude-plugin/marketplace.json theme={null}
{
  "name": "acme-tools",
  "owner": { "name": "Acme" },
  "allowCrossMarketplaceDependenciesOn": ["acme-shared"],
  "plugins": [
    {
      "name": "deploy-kit",
      "source": "./deploy-kit",
      "dependencies": [
        { "name": "audit-logger", "marketplace": "acme-shared" }
      ]
    }
  ]
}
```

Jika bidang hilang atau tidak menyertakan marketplace target, instalasi gagal dengan kesalahan `cross-marketplace` yang menamai bidang yang akan diatur. Pengguna masih dapat menginstal dependensi secara manual terlebih dahulu, yang memenuhi batasan tanpa mengubah daftar putih.

<h2 id="tag-plugin-releases-for-version-resolution">
  Tandai rilis plugin untuk resolusi versi
</h2>

Batasan versi diselesaikan terhadap tag git di repositori marketplace. Agar Claude Code menemukan versi yang tersedia dari dependensi, rilis plugin upstream harus ditandai menggunakan konvensi penamaan tertentu.

Tandai setiap rilis sebagai `{plugin-name}--v{version}`, di mana `{version}` cocok dengan bidang `version` di `plugin.json` commit tersebut. Dari direktori plugin, jalankan:

```bash theme={null}
claude plugin tag --push
```

Perintah `claude plugin tag` menurunkan nama tag dari manifes plugin dan entri marketplace yang melingkupinya. Sebelum membuat tag, perintah ini memvalidasi konten plugin, memeriksa bahwa `plugin.json` dan entri marketplace setuju tentang versi, memerlukan pohon kerja yang bersih di bawah direktori plugin, dan menolak jika tag sudah ada. Tambahkan `--dry-run` untuk melihat apa yang akan ditandai tanpa membuatnya. Menjalankan `git tag secrets-vault--v2.1.0` secara langsung setara jika Anda menjaga `plugin.json` dan entri marketplace tetap sinkron sendiri.

Awalan nama plugin memungkinkan satu repositori marketplace menghosting beberapa plugin dengan lini versi independen. Pemisah `--v` diuraikan sebagai pencocokan awalan pada nama plugin lengkap, sehingga nama plugin yang berisi tanda hubung ditangani dengan benar.

Ketika Anda menginstal plugin yang mendeklarasikan `{ "name": "secrets-vault", "version": "~2.1.0" }`, Claude Code mencantumkan tag marketplace, memfilter ke yang dimulai dengan `secrets-vault--v`, dan mengambil versi tertinggi yang memenuhi `~2.1.0`. Jika tidak ada tag yang cocok, plugin dependen dinonaktifkan dengan kesalahan yang mencantumkan versi yang tersedia.

Marketplace yang ditambahkan sebagai jalur folder lokal menyelesaikan tag dengan cara yang sama ketika folder adalah repositori git. Ini memerlukan Claude Code v2.1.196 atau lebih baru. Dalam dua kasus Claude Code menginstal dependensi dari konten folder saat ini:

* Versi sebelumnya tidak membaca tag dari marketplace folder lokal, jadi dependensi yang dibatasi hanya dimuat jika salinan tersebut memenuhi rentang.
* Folder lokal yang bukan repositori git tidak memiliki tag, terlepas dari versi.

Semver tag yang diselesaikan dicatat secara terpisah dari `version` milik `plugin.json`, sehingga pemeriksaan batasan menggunakan tag yang benar-benar diambil bahkan jika `plugin.json` pada commit tersebut memiliki nilai yang ketinggalan zaman. Nama direktori cache untuk instalasi yang diselesaikan tag mencakup akhiran SHA commit 12 karakter, sehingga jika pengelola memaksa memindahkan tag ke commit yang berbeda, instalasi berikutnya mendapatkan direktori cache segar alih-alih menggunakan kembali konten yang ketinggalan zaman.

<Note>
  Untuk sumber marketplace `npm`, batasan tidak mengontrol versi mana yang diambil, karena resolusi berbasis tag hanya berlaku untuk sumber yang didukung git. Batasan masih diperiksa pada waktu muat, dan plugin dependen dinonaktifkan dengan `dependency-version-unsatisfied` jika versi yang diinstal tidak memenuhinya.
</Note>

<h2 id="how-constraints-interact">
  Bagaimana batasan berinteraksi
</h2>

Ketika beberapa plugin yang diinstal membatasi dependensi yang sama, Claude Code memotong rentang mereka dan menyelesaikan dependensi ke versi tertinggi yang memenuhi semuanya. Tabel di bawah menunjukkan bagaimana kombinasi umum diselesaikan.

| Plugin A memerlukan | Plugin B memerlukan | Hasil                                                                                               |
| :------------------ | :------------------ | :-------------------------------------------------------------------------------------------------- |
| `^2.0`              | `>=2.1`             | Satu instalasi pada tag `2.x` tertinggi pada atau di atas `2.1.0`. Kedua plugin dimuat.             |
| `~2.1`              | `~3.0`              | Instalasi plugin B gagal dengan `range-conflict`. Plugin A dan dependensi tetap seperti sebelumnya. |
| `=2.1.0`            | tidak ada           | Dependensi tetap di `2.1.0`. Auto-update melewati versi yang lebih baru saat plugin A diinstal.     |

Auto-update mengambil dependensi terbatas pada tag git tertinggi yang memenuhi rentang setiap plugin yang diinstal, bukan pada versi terbaru marketplace, sehingga dependensi terus menerima pembaruan dalam rentang yang diizinkan. Jika tidak ada tag yang memenuhi semua rentang, auto-update melewati dependensi tersebut dan mencantumkan pelewatan di tab Errors `/plugin`, menamai plugin yang membatasi.

Ketika Anda mencopot plugin terakhir yang membatasi dependensi, dependensi tidak lagi ditahan dan melanjutkan pelacakan entri marketplacenya pada pembaruan berikutnya.

<h2 id="enable-or-disable-a-plugin-with-dependencies">
  Aktifkan atau nonaktifkan plugin dengan dependensi
</h2>

Mengaktifkan plugin juga mengaktifkan plugin yang bergantung padanya, dan menonaktifkan plugin diblokir jika plugin yang diaktifkan lain masih membutuhkannya. Kedua perilaku memerlukan Claude Code v2.1.143 atau lebih baru. Versi sebelumnya hanya mengaktifkan atau menonaktifkan plugin bernama dan menampilkan kesalahan `dependency-unsatisfied` pada beban berikutnya.

Ketika Anda mengaktifkan plugin, Claude Code juga mengaktifkan dependensinya pada cakupan yang sama. Jika dependensi memiliki dependensinya sendiri, Claude Code mengaktifkan yang tersebut juga. Pesan kesuksesan mencantumkan apa lagi yang diaktifkan bersama plugin yang Anda namai. Jika dependensi tidak dapat diaktifkan, perintah menolak dan memberi tahu Anda apa yang memblokir dan cara memperbaikinya:

| Kondisi                                                                                     | Hasil                                                                                                              |
| :------------------------------------------------------------------------------------------ | :----------------------------------------------------------------------------------------------------------------- |
| Dependensi tidak diinstal                                                                   | Aktifkan gagal dan mencetak perintah `claude plugin install` untuk setiap dependensi yang hilang.                  |
| Dependensi diblokir oleh kebijakan plugin organisasi Anda                                   | Aktifkan gagal dan menamai dependensi yang diblokir.                                                               |
| Dependensi diatur ke `false` pada cakupan dengan prioritas lebih tinggi dari cakupan target | Aktifkan gagal. Aktifkan dependensi pada cakupan tersebut, atau lewatkan `--scope` untuk menulis di sana.          |
| Semua dependensi diinstal dan diizinkan                                                     | Aktifkan berhasil dan menulis `true` untuk plugin dan setiap dependensi yang belum diaktifkan pada cakupan target. |

Ini berlaku bahkan ketika dependensi menetapkan [`defaultEnabled: false`](/docs/id/plugins-reference#default-enablement) dalam manifestnya, karena Claude Code menulis `true` eksplisit untuk itu. Hal yang sama berlaku saat instalasi: dependensi yang ditarik untuk memenuhi plugin aktif diinstal dengan `true` terlepas dari defaultnya sendiri.

Ketika Anda menonaktifkan plugin, Claude Code menolak jika plugin yang diaktifkan lain masih bergantung padanya. Kesalahan menamai plugin yang bergantung padanya dan memberikan Anda perintah berantai yang menonaktifkannya dalam urutan yang benar, diakhiri dengan yang Anda minta.

Misalnya, jika `deploy-kit` bergantung pada `secrets-vault`, menonaktifkan `secrets-vault` saja gagal dengan output serupa dengan berikut:

```text theme={null}
secrets-vault is still required by deploy-kit. Disable that plugin first, or
disable everything together: claude plugin disable deploy-kit@acme-tools && claude plugin disable secrets-vault@acme-tools
```

Salin perintah berantai dari kesalahan untuk menonaktifkan set lengkap dalam satu langkah.

<h2 id="remove-orphaned-auto-installed-dependencies">
  Hapus dependensi auto-install yang yatim piatu
</h2>

Dependensi auto-install tetap di disk setelah plugin yang menginstalnya dicopot, untuk berjaga-jaga jika Anda menginstal ulang plugin dependen atau ingin terus menggunakan dependensi secara langsung. Untuk membersihkannya, jalankan `claude plugin prune` untuk mencantumkan dependensi auto-install yang tidak lagi memiliki plugin yang diinstal yang memerlukan mereka dan menghapusnya setelah prompt konfirmasi. Ini memerlukan Claude Code v2.1.121 atau lebih baru.

```bash theme={null}
claude plugin prune
```

Secara default, prune beroperasi pada cakupan pengguna. Gunakan `--scope project` atau `--scope local` untuk menargetkan cakupan berbeda. Lewatkan `--dry-run` untuk mencantumkan apa yang akan dihapus tanpa mengubah apa pun. Lewatkan `-y` untuk melewati prompt konfirmasi. Ketika stdin atau stdout bukan terminal, prune mencantumkan yatim piatu dan keluar tanpa menghapusnya kecuali `-y` dilewatkan.

Untuk prune sebagai bagian dari uninstall, lewatkan `--prune` ke `claude plugin uninstall`. Setelah menghapus plugin bernama, Claude Code memindai dan menghapus dependensi auto-install apa pun yang sekarang yatim piatu. Plugin yang Anda instal sendiri tidak pernah dipangkas, hanya yang diinstal secara otomatis melalui array `dependencies` plugin lain.

Misalnya, untuk mencopot `deploy-kit` dan membersihkan dependensi yang ditinggalkannya:

```bash theme={null}
claude plugin uninstall deploy-kit --prune
```

<h2 id="resolve-dependency-errors">
  Selesaikan kesalahan dependensi
</h2>

Masalah dependensi muncul di `claude plugin list` dan di antarmuka `/plugin`. Claude Code menonaktifkan plugin yang terpengaruh sampai Anda menyelesaikan kesalahan. Tabel di bawah mencantumkan kesalahan paling umum dan cara menyelesaikannya.

| Kesalahan                        | Arti                                                                                                                                                                                                                                            | Cara menyelesaikan                                                                                                                                                                                                                                                                     |
| :------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `dependency-unsatisfied`         | Dependensi yang dideklarasikan tidak diinstal, atau diinstal tetapi dinonaktifkan.                                                                                                                                                              | Jalankan perintah `claude plugin install` yang ditampilkan dalam pesan kesalahan. Jika marketplace dependensi belum dikonfigurasi, tambahkan dengan `claude plugin marketplace add` dan Claude Code menyelesaikan dependensi secara otomatis. Jika dependensi dinonaktifkan, aktifkan. |
| `range-conflict`                 | Persyaratan versi untuk dependensi tidak dapat digabungkan. Pesan kesalahan menamai penyebabnya: tidak ada versi yang memenuhi semua rentang, rentang bukan sintaks semver yang valid, atau rentang gabungan terlalu kompleks untuk dipotongan. | Copot atau perbarui salah satu plugin yang bertentangan, perbaiki string `version` yang tidak valid, sederhanakan rantai `\|\|` panjang, atau minta penulis upstream untuk memperluas batasannya.                                                                                      |
| `dependency-version-unsatisfied` | Versi dependensi yang diinstal berada di luar rentang yang dideklarasikan plugin ini.                                                                                                                                                           | Jalankan `claude plugin install <dependency>@<marketplace>` untuk menyelesaikan kembali dependensi terhadap semua batasan saat ini.                                                                                                                                                    |
| `no-matching-tag`                | Repositori dependensi tidak memiliki tag `{name}--v*` yang memenuhi rentang.                                                                                                                                                                    | Periksa bahwa upstream telah menandai rilis menggunakan konvensi di atas, atau relakskan rentang Anda.                                                                                                                                                                                 |

Untuk memeriksa kesalahan ini secara terprogram, jalankan `claude plugin list --json` dan baca bidang `errors` pada setiap plugin.

<h2 id="see-also">
  Lihat juga
</h2>

* [Buat plugin](/docs/id/plugins): bangun plugin dengan skills, agents, dan hooks
* [Buat dan distribusikan marketplace plugin](/docs/id/plugin-marketplaces): hosting plugin untuk tim Anda
* [Referensi Plugin](/docs/id/plugins-reference#plugin-manifest-schema): skema `plugin.json` lengkap
* [Manajemen versi](/docs/id/plugins-reference#version-management): bagaimana versi plugin itu sendiri diselesaikan dan digunakan sebagai kunci cache
