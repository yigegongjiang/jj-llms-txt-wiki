> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Publikasikan dan distribusikan plugin

> Publikasikan plugin Claude Code melalui marketplace Anda sendiri atau marketplace komunitas Anthropic, dengan daftar periksa pra-rilis dan cara pengguna mendapatkan pembaruan.

Mempublikasikan plugin Claude Code berarti mencantumkannya di marketplace, katalog JSON yang mencantumkan plugin dan tempat mengambil masing-masing plugin, sehingga orang lain dapat memasangnya berdasarkan nama dan menerima pembaruan Anda. Anda dapat menjalankan marketplace Anda sendiri atau mengirimkan plugin Anda ke marketplace komunitas Anthropic. Untuk berbagi plugin tanpa mempublikasikannya, kirimkan direktori plugin atau `.zip` darinya kepada orang-orang untuk dimuat sendiri.

Halaman ini adalah untuk penulis plugin yang berfungsi dan siap untuk membagikannya.

<Note>
  Kasus-kasus ini tercakup di halaman lain:

  * **Plugin Anda belum selesai**: mulai dengan [Buat plugin](/docs/id/plugins/create)
  * **Anda memelihara CLI atau SDK dengan plugin di marketplace resmi**: lihat [Rekomendasikan plugin Anda dari CLI Anda](/docs/id/plugins/cli-hints)
</Note>

Mulai dengan [Pilih cara mendistribusikan](#choose-how-to-distribute) untuk membandingkan opsi distribusi. Jika Anda sudah tahu rute Anda, buka [Siapkan plugin Anda untuk rilis](#prepare-your-plugin-for-release), kemudian ikuti bagian rute Anda untuk apa yang harus diberitahukan kepada pengguna Anda dan bagaimana mereka menerima pembaruan Anda.

<h2 id="choose-how-to-distribute">
  Pilih cara mendistribusikan
</h2>

Pilih opsi distribusi berdasarkan siapa yang perlu memasang plugin:

| Rute                                                                    | Siapa yang dapat memasang                                                                                      | Apa yang Anda butuhkan                                                                               | Apakah pengguna mendapatkan pembaruan Anda secara otomatis? |
| :---------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------- | :---------------------------------------------------------- |
| [Tanpa marketplace](#share-a-plugin-without-a-marketplace)              | Orang-orang yang Anda kirimkan folder plugin atau `.zip` darinya                                               | Folder plugin                                                                                        | Tidak ada. Mereka memuat salinan yang Anda kirimkan         |
| [Marketplace Anda sendiri](#publish-through-your-own-marketplace)       | Siapa pun yang dapat menjangkau repositori, yang dapat berupa repositori pribadi yang dapat dikloning tim Anda | Repositori git atau host lain dengan `.claude-plugin/marketplace.json` yang mencantumkan plugin Anda | Mati                                                        |
| [Marketplace komunitas Anthropic](#submit-to-the-community-marketplace) | Siapa pun yang menambahkan `anthropics/claude-plugins-community`                                               | Pengajuan melalui formulir pengajuan direktori plugin                                                | Mati                                                        |

Auto-update adalah pengaturan per-marketplace di sisi pengguna yang mengambil versi baru di latar belakang.

<h2 id="prepare-your-plugin-for-release">
  Siapkan plugin Anda untuk rilis
</h2>

Nama, versi, validasi, dan pemasangan dari marketplace menentukan apakah rilis berfungsi untuk orang-orang yang memasangnya. Periksa sebelum rilis pertama dan lagi sebelum setiap rilis berikutnya.

<Steps>
  <Step title="Pilih nama permanen">
    Pengguna memasang, mengaktifkan, dan mengonfigurasi plugin Anda dengan `name@marketplace`, jadi plugin yang diganti nama adalah plugin yang berbeda untuk setiap pemasangan yang ada. Pilih nama kebab-case seperti `deploy-helper`, karena `claude plugin validate` memperingatkan bentuk lain, dan perlakukan sebagai permanen. Atur `displayName` di `plugin.json` untuk label yang dilihat pengguna.
  </Step>

  <Step title="Tentukan cara Anda akan membuat versi">
    Jika Anda menetapkan `version` di `plugin.json` dan kemudian mendorong commit tanpa mengubahnya, `claude plugin update` mencetak `<name> is already at the latest version (1.0.0).` dan pengguna menyimpan salinan lama. Baik tingkatkan `version` pada setiap rilis, atau hilangkan di marketplace yang dihosting git sehingga Claude Code menggunakan SHA commit sebagai gantinya. Lihat [Versi dan pembaruan](/docs/id/plugins/loading#versions-and-updates).
  </Step>

  <Step title="Validasi">
    Di shell Anda, jalankan `claude plugin validate --strict ./your-plugin`. Jalankan bersih mencetak `✔ Validation passed`.

    * **Di CI**: pertahankan `--strict`, yang juga gagal jalankan dengan kode keluar 1 pada peringatan seperti bidang manifes yang tidak dikenal atau `version` yang hilang. Lepaskan `--strict` jika Anda memilih untuk menghilangkan `version` di langkah sebelumnya.
    * **Jalur**: validasi melaporkan jalur komponen yang tidak dimulai dengan `./`. Di dalam perintah hook dan konfigurasi server MCP, rujuk file sebagai `${CLAUDE_PLUGIN_ROOT}/...`. Lihat [aturan jalur](/docs/id/plugins/manifest-reference#path-rules).
  </Step>

  <Step title="Pasang dari marketplace lokal">
    Di shell Anda, tambahkan marketplace lokal yang mencantumkan plugin dengan `claude plugin marketplace add ./path-to-marketplace`, pasang plugin darinya, dan mulai sesi untuk mengonfirmasi bahwa plugin dimuat.

    * Untuk marketplace terkecil yang berfungsi, lihat [Buat marketplace](/docs/id/plugins/create-marketplace).
    * Untuk mengetahui apakah pemasangan memuat direktori sumber Anda atau salinan cache, lihat [Plugin in-place dan disalin](/docs/id/plugins/loading#in-place-and-copied-plugins).
  </Step>

  <Step title="Isi metadata yang dilihat pengguna">
    Atur `description`, `author`, `homepage`, dan `repository` di `plugin.json`, dan tambahkan `README.md` di root plugin. `homepage` harus diurai sebagai URL. [Referensi manifes](/docs/id/plugins/manifest-reference#fields) mencantumkan setiap bidang.
  </Step>

  <Step title="Jalankan suite eval Anda">
    Jika Anda memiliki suite eval, jalankan `claude plugin eval` di shell Anda. Ini menjalankan kasus uji plugin dan mencetak hasil, yang menangkap regresi saat Anda mengubah plugin. Lihat [Plugin uji dengan eval](/docs/id/plugin-evals).
  </Step>
</Steps>

<h2 id="share-a-plugin-without-a-marketplace">
  Bagikan plugin tanpa marketplace
</h2>

Jika plugin berada di repositori git, orang dapat mengklonnya dan memuat checkout, atau memulai Claude Code dari shell mereka dengan `--plugin-url` yang menunjuk ke `.zip` yang Anda lampirkan ke rilis. Untuk mendapatkan versi berikutnya mereka menarik atau mengunduh lagi. Jika tidak ada di repositori, kirimkan direktori atau `.zip` darinya. Mereka memuat dengan salah satu dari dua cara:

* **Untuk satu sesi**: mereka memulai Claude Code dari shell mereka dengan `claude --plugin-dir ./deploy-helper`, di mana jalurnya adalah klon, folder yang tidak dizip, atau `.zip` itu sendiri. Lihat [Bendera yang memuat plugin untuk satu sesi](/docs/id/plugins/cli-reference#flags-that-load-a-plugin-for-one-session).
* **Untuk setiap sesi**: mereka memindahkan direktori plugin, dengan `.claude-plugin/plugin.json` miliknya, di bawah `~/.claude/skills/` sehingga Claude Code [memuat dalam setiap sesi](/docs/id/plugins/loading#find-where-a-plugin-came-from).

Menambahkan `.claude-plugin/marketplace.json` ke repositori yang sama adalah apa yang memungkinkan orang memasang berdasarkan nama dan memperbarui dengan perintah; lihat [Publikasikan melalui marketplace Anda sendiri](#publish-through-your-own-marketplace).

<h3 id="ship-a-plugin-with-your-own-tool">
  Kirim plugin dengan alat Anda sendiri
</h3>

Jika Anda memelihara CLI atau SDK, publikasikan plugin di marketplace dan buat installer atau pesan pasca-instalasi Anda menjalankan atau mencetak dua perintah yang dibutuhkan pengguna: `claude plugin marketplace add <source>`, kemudian `claude plugin install <name>@<marketplace>`. Untuk penemuan dalam sesi ketika seseorang menggunakan alat Anda, lihat [Rekomendasikan plugin Anda dari CLI Anda](/docs/id/plugins/cli-hints).

<h2 id="publish-through-your-own-marketplace">
  Publikasikan melalui marketplace Anda sendiri
</h2>

Marketplace Anda sendiri adalah file `.claude-plugin/marketplace.json` yang mencantumkan plugin Anda, ditambahkan ke repositori git. Setelah file berada di repositori, plugin dipublikasikan, tanpa formulir pengajuan. Anda dapat menyimpan file di repositori plugin Anda sendiri atau di repositori terpisah.

<h3 id="add-the-marketplace-file-to-your-repository">
  Tambahkan file marketplace ke repositori Anda
</h3>

Untuk mempublikasikan dari repositori plugin Anda sendiri, simpan file marketplace di samping `plugin.json` di `.claude-plugin/`, dengan satu entri yang `source` adalah `"./"`, root repositori. Berikan entri `name` yang sama dengan `plugin.json`, per [Jaga nama entri dan nama manifes tetap sama](/docs/id/plugins/create-marketplace#keep-the-entry-name-and-the-manifest-name-the-same):

```json .claude-plugin/marketplace.json theme={null}
{
  "name": "your-marketplace",
  "owner": { "name": "Your Name" },
  "plugins": [
    { "name": "deploy-helper", "source": "./" }
  ]
}
```

Di shell Anda, jalankan `claude plugin validate .` di repositori untuk memeriksa file sebelum Anda mendorong.

[Buat marketplace](/docs/id/plugins/create-marketplace) mencakup tata letak dengan beberapa plugin di satu repositori.

<h3 id="control-who-can-install">
  Kontrol siapa yang dapat memasang
</h3>

Siapa pun yang dapat mengkloning repositori dapat memasang darinya, jadi jika repositori pribadi, marketplace juga pribadi. Untuk host selain repositori git, lihat [Host marketplace](/docs/id/plugins/host-marketplace). Untuk menjangkau semua orang di perusahaan, termasuk orang yang tidak menggunakan git, lihat [Luncurkan ke seluruh perusahaan](/docs/id/plugins/host-marketplace#roll-out-to-a-whole-company).

<h3 id="tell-users-how-to-install">
  Beritahu pengguna cara memasang
</h3>

Beritahu pengguna Anda untuk menambahkan marketplace dan kemudian memasang plugin dari shell mereka, mengganti sumber dan nama dengan milik Anda:

* Tambahkan marketplace sekali: `claude plugin marketplace add your-org/your-marketplace`, di mana argumennya adalah shorthand GitHub `owner/repo`, URL, atau jalur
* Pasang plugin: `claude plugin install deploy-helper@your-marketplace`
* Atau lakukan keduanya dari dalam sesi: `/plugin install deploy-helper --marketplace your-org/your-marketplace`. Memerlukan Claude Code v2.1.275 atau lebih baru. Lihat [Tambahkan marketplace dan pasang dalam satu perintah](/docs/id/plugins/install#add-a-marketplace-and-install-in-one-command)

<h3 id="ship-updates-to-users">
  Kirim pembaruan kepada pengguna
</h3>

Pengguna menerima rilis ketika mereka memintanya atau ketika auto-update aktif untuk marketplace Anda:

* **Atas permintaan**: `claude plugin update deploy-helper@your-marketplace` di shell pengguna menyegarkan marketplace dan memasang salinan baru ketika versi plugin Anda telah berubah
* **Auto-update**: mati secara default untuk marketplace Anda. Lihat [Aktifkan auto-update](/docs/id/plugins/host-marketplace#turn-on-auto-update). Setelah aktif, ini melakukan hal yang sama dengan `claude plugin update` dengan penundaan setelah sesi dimulai

[Pasang plugin](/docs/id/plugins/install) mencakup perintah sisi pengguna, dan [kapan auto-update berjalan](/docs/id/plugins/loading#when-auto-update-runs) mencakup waktu.

<h2 id="submit-to-the-community-marketplace">
  Kirimkan ke marketplace komunitas
</h2>

Marketplace komunitas Anthropic, `claude-community`, adalah marketplace publik yang mencantumkan plugin yang dikirimkan melalui formulir pengajuan direktori plugin.

Pengguna menambahkan marketplace komunitas dalam sesi Claude Code dengan `/plugin marketplace add anthropics/claude-plugins-community` dan memasang darinya sebagai `@claude-community`.

Untuk cara marketplace komunitas berbeda dari marketplace resmi, lihat [Marketplace Anthropic](/docs/id/plugins/anthropic-marketplaces).

Untuk mengirimkan plugin Anda ke marketplace komunitas, gunakan salah satu formulir dalam aplikasi:

* **claude.ai**: [claude.ai/admin-settings/directory/submissions/plugins/new](https://claude.ai/admin-settings/directory/submissions/plugins/new)
* **Console**: [platform.claude.com/plugins/submit](https://platform.claude.com/plugins/submit)

Formulir claude.ai memerlukan organisasi Team atau Enterprise dan izin Directory, yang dipegang Owner secara default. Penulis individual yang bukan bagian dari organisasi Team atau Enterprise dapat menggunakan formulir Console sebagai gantinya.

Di shell Anda, jalankan `claude plugin validate ./your-plugin` secara lokal sebelum Anda mengirimkan, mengganti `./your-plugin` dengan jalur ke direktori plugin Anda. Ketika validasi lulus, Claude Code mencetak `✔ Validation passed`, atau `✔ Validation passed with warnings` jika ada peringatan. Peringatan tidak gagal validasi; tambahkan `--strict` untuk memperlakukan mereka sebagai kesalahan.

Plugin yang tercantum muncul di katalog [`anthropics/claude-plugins-community`](https://github.com/anthropics/claude-plugins-community), dalam hampir setiap kasus disematkan ke SHA commit tertentu.

Mungkin ada penundaan antara pengajuan dan plugin Anda muncul di `marketplace.json`. Untuk memeriksa apakah plugin Anda dapat dipasang, cari namanya di [katalog komunitas](https://github.com/anthropics/claude-plugins-community/blob/main/.claude-plugin/marketplace.json).

Marketplace resmi, `claude-plugins-official`, tidak menerima pengajuan melalui formulir ini. Jika Anda bekerja dengan kontak mitra Anthropic, tanyakan tentang daftar marketplace resmi.

<h2 id="ship-updates-renames-and-removals">
  Kirim pembaruan, penggantian nama, dan penghapusan
</h2>

<h3 id="release-a-new-version">
  Rilis versi baru
</h3>

Jika Anda mempublikasikan melalui marketplace Anda sendiri dan `plugin.json` Anda menetapkan `version`, tingkatkan dan dorong. Pengguna yang menjalankan `claude plugin update` atau memiliki auto-update aktif kemudian menerima versi baru, seperti dijelaskan di bawah [Kirim pembaruan kepada pengguna](#ship-updates-to-users).

<h3 id="tag-a-release">
  Tag rilis
</h3>

Tag rilis di git ketika plugin lain mendeklarasikan rentang versi pada milik Anda, karena rentang tersebut diselesaikan terhadap tag. Jika tidak, Anda tidak memerlukan tag.

Untuk tag, jalankan `claude plugin tag` di shell Anda dari direktori plugin. Ini membuat tag `{name}--v{version}`. Tambahkan `--push` untuk mengirim tag ke `origin`. [Referensi `plugin tag`](/docs/id/plugins/cli-reference#plugin-tag) mencantumkan bendera-bendera miliknya.

<h3 id="rename-or-remove-a-plugin">
  Ganti nama atau hapus plugin
</h3>

Jangan pernah ubah `name` plugin yang dipublikasikan. Setelah penggantian nama, pengguna yang sudah memasangnya kehilangan plugin, karena pemasangan mereka dicatat di bawah nama lama. Entri `renames` di file marketplace Anda memigrasikan mereka sebagai gantinya. Ubah `displayName` ketika Anda menginginkan label yang berbeda.

Jika penggantian nama tidak dapat dihindari, gunakan peta `renames` file marketplace sehingga pemasangan yang ada bermigrasi alih-alih gagal dengan [`Plugin "<name>" not found in marketplace`](/docs/id/plugins/troubleshooting#plugin-not-found-in-marketplace). Untuk menghapus plugin dari marketplace, atau untuk detail `renames` lengkap, lihat [Ganti nama atau hapus plugin](/docs/id/plugins/host-marketplace#rename-or-remove-a-plugin) di halaman hosting. [Referensi marketplace](/docs/id/plugins/marketplace-reference#top-level-fields) memiliki bidang.

<h2 id="declare-dependencies">
  Deklarasikan dependensi
</h2>

Jika plugin Anda memerlukan plugin lain dari marketplace yang sama untuk diaktifkan, cantumkan di array `dependencies` dari `plugin.json`. Setiap entri adalah nama telanjang atau objek dengan rentang `version` semver. Ketika pengguna memasang plugin Anda, Claude Code memasang dan mengaktifkan dependensi juga.

[Dependensi plugin](/docs/id/plugins/dependencies) mencakup sintaks rentang, dependensi lintas-marketplace, dan cara pengguna memangkas dependensi yang tidak lagi mereka butuhkan.

<h2 id="next-steps">
  Langkah berikutnya
</h2>

* [Host dan pertahankan marketplace](/docs/id/plugins/host-marketplace): rilis versi baru dan jaga pengguna tetap terkini
* [Dependensi plugin](/docs/id/plugins/dependencies): deklarasikan dan buat versi plugin yang Anda andalkan
* [Rekomendasikan plugin Anda dari CLI Anda](/docs/id/plugins/cli-hints): minta pengguna Claude Code dari CLI Anda untuk memasang plugin
* [Ukur biaya dan penggunaan plugin](/docs/id/plugins/measure): lihat berapa biaya plugin Anda dalam konteks dan apakah orang menggunakannya
