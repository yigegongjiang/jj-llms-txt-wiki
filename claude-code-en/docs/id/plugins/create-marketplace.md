> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Buat marketplace

> Bangun marketplace plugin dari file marketplace.json dan uji secara lokal sebelum Anda menghosting-nya.

Marketplace plugin adalah direktori atau repositori dengan file `.claude-plugin/marketplace.json` yang mencantumkan plugin Anda dan tempat mengambil masing-masing plugin. Anda mendorong direktori ke host git, dan siapa pun yang memiliki akses mendaftarkannya di Claude Code dengan satu perintah dan menginstal plugin Anda darinya.

Buat marketplace Anda sendiri ketika Anda ingin grup yang Anda pilih, seperti tim atau organisasi Anda, menginstal plugin Anda dan terus menerima pembaruan Anda dari katalog yang Anda kontrol. Repositori dapat bersifat pribadi, dapat mencantumkan sebanyak mungkin plugin yang Anda inginkan, dan administrator dapat [memerlukannya di setiap mesin](/docs/id/plugins/org).

<Note>
  Kasus-kasus ini tercakup di halaman lain:

  * **Berbagi satu plugin dengan beberapa orang**: kirimkan mereka direktori plugin atau `.zip` darinya. Lihat [Bagikan plugin tanpa marketplace](/docs/id/plugins/publish#share-a-plugin-without-a-marketplace).
  * **Menawarkan plugin kepada semua orang**: kirimkan ke marketplace komunitas Anthropic. Lihat [Kirim ke marketplace komunitas](/docs/id/plugins/publish#submit-to-the-community-marketplace).
  * **Menggunakan plugin sendiri**: muat dengan `--plugin-dir` atau simpan di direktori skills Anda. Lihat [Kembangkan tanpa marketplace](/docs/id/plugins/create#develop-without-a-marketplace).
</Note>

Mulai dengan [Buat marketplace](#create-a-marketplace) untuk membangun satu di mesin Anda sendiri dan menginstal plugin darinya, kemudian [tambahkan entri plugin lainnya](#add-plugin-entries).

<h2 id="create-a-marketplace">
  Buat marketplace
</h2>

Langkah-langkah berikut membuat marketplace di mesin Anda, menambahkan plugin ke dalamnya, mendaftarkannya di Claude Code, dan menginstal plugin darinya. Itu adalah seluruh loop, dan itu adalah loop yang sama yang dilalui pengguna Anda setelah Anda menghosting marketplace di tempat yang dapat mereka jangkau. Jalankan setiap perintah di shell Anda, dari direktori tempat Anda ingin `my-marketplace/` dibuat.

Anda memerlukan plugin untuk dicantumkan. Contoh menggunakan `my-first-plugin` dari [Buat plugin pertama Anda](/docs/id/plugins/create#create-your-first-plugin), plugin dengan satu skill yang Anda jalankan sebagai `/my-first-plugin:hello`; bangun terlebih dahulu jika Anda belum memiliki plugin. Untuk menggunakan plugin Anda sendiri, gantikan direktorinya dan `name`-nya di mana pun langkah-langkah mengatakan `my-first-plugin`. Untuk apa yang dapat dimuat direktori plugin, lihat [penjelajah direktori plugin](/docs/id/plugins/components#explore-the-plugin-directory).

<Steps>
  <Step title="Siapkan direktori marketplace">
    Marketplace adalah direktori dengan file `.claude-plugin/marketplace.json`, ditambah plugin yang dicantumkannya. Buat direktori marketplace dan folder `.claude-plugin/`-nya, kemudian salin plugin Anda di bawah `plugins/`:

    ```bash theme={null}
    mkdir -p my-marketplace/.claude-plugin my-marketplace/plugins
    cp -r my-first-plugin my-marketplace/plugins/
    ```

    Periksa bahwa plugin valid di mana ia sekarang berada, sehingga kesalahan apa pun kemudian adalah tentang marketplace dan bukan plugin:

    ```bash theme={null}
    claude plugin validate ./my-marketplace/plugins/my-first-plugin
    ```

    Baris terakhir dari output membaca `✔ Validation passed`.
  </Step>

  <Step title="Buat file marketplace">
    Simpan `marketplace.json` di `my-marketplace/.claude-plugin/marketplace.json`. File memerlukan `name`, `owner`, dan array `plugins`.

    Setiap objek dalam `plugins` adalah entri plugin dan memerlukan `name` dan `source`. Tulis `source` entri sebagai jalur dari akar marketplace. Akar adalah `my-marketplace/`, direktori yang berisi `.claude-plugin/`.

    ```json my-marketplace/.claude-plugin/marketplace.json theme={null}
    {
      "name": "my-marketplace",
      "description": "Plugins for my team",
      "owner": {
        "name": "Your Name"
      },
      "plugins": [
        {
          "name": "my-first-plugin",
          "source": "./plugins/my-first-plugin",
          "description": "A greeting plugin to learn the basics"
        }
      ]
    }
    ```
  </Step>

  <Step title="Validasi marketplace">
    Jalankan `claude plugin validate` pada direktori marketplace untuk memeriksa sintaks JSON, bidang yang diperlukan, dan setiap entri plugin dalam `.claude-plugin/marketplace.json`-nya.

    ```bash theme={null}
    claude plugin validate ./my-marketplace
    ```

    Untuk file seperti yang ditulis di langkah 2, baris terakhir dari output membaca `✔ Validation passed`.
  </Step>

  <Step title="Tambahkan marketplace dan instal plugin">
    Daftarkan direktori sebagai marketplace.

    ```bash theme={null}
    claude plugin marketplace add ./my-marketplace
    ```

    Perintah mencetak `✔ Successfully added marketplace: my-marketplace (declared in user settings)`, yang berarti marketplace dicatat dalam file pengaturan pengguna Anda.

    Instal plugin. ID instalasi adalah `name` entri, `@`, dan `name` marketplace.

    ```bash theme={null}
    claude plugin install my-first-plugin@my-marketplace
    ```

    Perintah mencetak `✔ Successfully installed plugin: my-first-plugin@my-marketplace (scope: user)`.

    Di dalam sesi, `/plugin marketplace add ./my-marketplace` mendaftarkan marketplace dengan cara yang sama. `/plugin install my-first-plugin@my-marketplace` membuka detail plugin di panel `/plugin`, tempat Anda menginstalnya. Untuk alur itu, lihat [Instal dan kelola plugin](/docs/id/plugins/install).
  </Step>

  <Step title="Konfirmasi plugin dimuat">
    Daftar plugin yang diinstal.

    ```bash theme={null}
    claude plugin list
    ```

    Output mencantumkan `my-first-plugin@my-marketplace` dengan `Status: ✔ enabled`.

    Untuk melihat apa yang dimuat plugin, tampilkan detailnya.

    ```bash theme={null}
    claude plugin details my-first-plugin
    ```

    Bagian `Component inventory` membaca `Skills (1)  hello`.

    Untuk menjalankan skill, mulai sesi dan masukkan `/my-first-plugin:hello`. Claude menyapa Anda. Perintah memiliki nama plugin sebagai awalan, seperti halnya nama skill plugin apa pun.
  </Step>
</Steps>

<h2 id="add-plugin-entries">
  Tambahkan entri plugin
</h2>

Setiap plugin yang Anda distribusikan adalah satu objek dalam array `plugins` dari `marketplace.json`. Untuk menambahkan plugin kedua, tambahkan objek kedua. Bidang-bidang ini mencakup sebagian besar entri:

* `name`: pengidentifikasi yang diketik orang sebelum `@` saat mereka menginstal. Tidak dapat berisi spasi.
* `source`: tempat Claude Code mengambil plugin. Tulis string jalur relatif untuk plugin di dalam direktori marketplace, seperti dalam [panduan walkthrough](#create-a-marketplace), atau objek sumber untuk plugin di luar. Lihat [Pilih sumber plugin](#choose-a-plugin-source).
* `description`: baris yang dilihat orang di sebelah plugin saat mereka menjelajahi marketplace Anda di `/plugin`.

Untuk daftar bidang lengkap, lihat [Entri plugin](/docs/id/plugins/marketplace-reference#plugin-entries).

Entri juga dapat menetapkan bidang [`plugin.json`](/docs/id/plugins/manifest-reference) apa pun. Untuk kapan bidang `plugin.json` entri berlaku untuk plugin yang memiliki `plugin.json`-nya sendiri, lihat [Entri dan plugin.json](/docs/id/plugins/marketplace-reference#entry-and-plugin-json).

<h2 id="rules-for-plugin-entries">
  Aturan untuk entri plugin
</h2>

Sebagian besar instalasi yang gagal dari marketplace baru berasal dari jalur relatif yang ditulis dari direktori yang salah, atau dari nama entri yang berbeda dari `name` dalam `plugin.json` plugin.

<h3 id="write-relative-paths-from-the-marketplace-root">
  Tulis jalur relatif dari akar marketplace
</h3>

Akar marketplace adalah direktori yang berisi `.claude-plugin/`. Dalam [panduan walkthrough](#create-a-marketplace), itu adalah `my-marketplace/`, jadi `source` entri adalah `"./plugins/my-first-plugin"`. Jalur tidak dimulai di dalam `.claude-plugin/`, jadi jangan gunakan `..` untuk meninggalkannya.

Jalur dengan `..` dan jalur ke direktori yang hilang gagal pada perintah yang berbeda:

* **Jalur dengan `..`**: `claude plugin validate` melaporkan entri sebagai tidak valid. Pesan dimulai dengan `Path contains "..": ./../plugins/my-first-plugin`.
* **Jalur ke direktori yang tidak ada**: `claude plugin validate` lulus. `claude plugin install` gagal dengan `Source path does not exist: <path>`, dan `<path>` adalah lokasi absolut yang diperiksa Claude Code.

<h3 id="keep-the-entry-name-and-the-manifest-name-the-same">
  Jaga nama entri dan nama manifest tetap sama
</h3>

Plugin marketplace memiliki `name` entri dalam `marketplace.json` dan `name` dalam `plugin.json`-nya sendiri, disebut nama manifest. Setiap nama muncul di tempat yang berbeda:

* **Nama entri**: ID instalasi, `<entry-name>@<marketplace>`. Ini adalah apa yang diketik orang untuk menginstal, apa yang ditampilkan `claude plugin list`, dan kunci yang ditulis Claude Code di bawah [`enabledPlugins`](/docs/id/settings-reference#enabledplugins) dalam file pengaturan mereka.
* **Nama manifest**: awalan pada skill plugin, dan nama yang diambil `claude plugin details`.

Ketika dua nama berbeda dan seseorang menginstal dengan nama manifest, Claude Code melaporkan `Plugin "<manifest-name>" not found in marketplace "<marketplace>"`. Jaga dua nama tetap sama. Untuk lebih lanjut tentang bagaimana Claude Code menggunakan dua nama, lihat [Referensi pemuatan plugin](/docs/id/plugins/loading#find-where-a-plugin-came-from).

<h2 id="choose-a-plugin-source">
  Pilih sumber plugin
</h2>

Setiap entri plugin dalam `marketplace.json` memiliki `source` yang memberi tahu Claude Code tempat mengambil plugin itu. Pilih sumber berdasarkan tempat file plugin disimpan. Tabel mencantumkan sumber yang paling sering digunakan pemilik marketplace.

| Sumber        | Gunakan ketika                                                             | Nilai `source` minimal                                                                    |
| :------------ | :------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------- |
| Jalur relatif | File plugin berada di dalam direktori marketplace itu sendiri              | `"./plugins/my-first-plugin"`                                                             |
| `github`      | Plugin adalah repositori GitHub sendiri                                    | `{ "source": "github", "repo": "your-org/my-first-plugin" }`                              |
| `git-subdir`  | Plugin adalah subdirektori dari beberapa repositori lain, seperti monorepo | `{ "source": "git-subdir", "url": "your-org/monorepo", "path": "tools/my-first-plugin" }` |

Dalam sumber `git-subdir`, `url` mengambil URL git atau shorthand GitHub `owner/repo`.

Plugin juga dapat berasal dari salah satu jenis sumber ini:

* `url`: repositori git berdasarkan URL, di host apa pun
* `archive`: file zip yang diunduh melalui HTTPS
* `npm`: paket npm
* `command`: direktori yang dihasilkan dengan menjalankan perintah di mesin tempat plugin diinstal

Untuk bidang setiap jenis sumber, dan untuk menyematkan sumber berbasis git ke `ref` atau `sha`, lihat [Sumber plugin](/docs/id/plugins/marketplace-reference#plugin-sources).

<h2 id="validate-and-test">
  Validasi dan uji
</h2>

Saat Anda menambahkan plugin, jalankan `claude plugin validate ./my-marketplace` di shell Anda setelah setiap edit, dan instal dari marketplace di mesin Anda sendiri sebelum Anda membagikannya. Validasi dan instalasi menangkap masalah yang berbeda.

<h3 id="problems-that-validation-reports">
  Masalah yang dilaporkan validasi
</h3>

`claude plugin validate` hanya membaca file di dalam direktori marketplace. Ini melaporkan:

* Kesalahan sintaks JSON, sebagai `json: Invalid JSON syntax: <reason>`
* Bidang yang diperlukan hilang, seperti `owner: Invalid input`
* Nama marketplace dengan spasi, karakter non-ASCII, atau bentuk yang meniru marketplace Anthropic resmi, seperti `claude-official`
* `source` relatif yang berisi `..`
* Bidang yang tidak dikenal di tingkat atas atau dalam entri plugin, sebagai peringatan
* Masalah dalam `plugin.json` dari setiap plugin jalur-relatif, sebagai `plugins[N] plugin.json → <field>: <message>`

Untuk setiap pesan yang dapat dicetak `validate`, lihat [Pesan validasi](/docs/id/plugins/marketplace-reference#validation-messages). Untuk flag dan kode keluarnya, lihat [`plugin validate`](/docs/id/plugins/cli-reference#plugin-validate).

<h3 id="problems-that-surface-when-you-add-or-install">
  Masalah yang muncul saat Anda menambahkan atau menginstal
</h3>

Masalah yang tidak dilaporkan `claude plugin validate` muncul saat Anda menambahkan marketplace atau menginstal darinya:

* **Saat Anda menambahkan marketplace**: [nama marketplace resmi](/docs/id/plugins/marketplace-reference#reserved-names) yang tepat, seperti `claude-plugins-official`, lulus validasi. Saat Anda menambahkan marketplace dengan salah satu nama tersebut, Claude Code menolaknya dengan pesan yang dimulai dengan `The name '<name>' is reserved for official Anthropic marketplaces`.
* **Saat Anda menginstal plugin**:
  * Claude Code pertama kali mengambil sumber `github`, `git-subdir`, atau sumber jarak jauh lainnya saat Anda menginstal plugin, jadi `repo` atau `path` yang salah muncul kemudian.
  * `source` relatif yang direktorinya tidak ada juga gagal saat instalasi, dengan `Source path does not exist: <path>`.

<h3 id="test-an-edit-to-a-plugin">
  Uji edit ke plugin
</h3>

Dalam [panduan walkthrough](#create-a-marketplace), Anda menambahkan `my-marketplace` dari direktori lokal dengan `source` jalur-relatif. Dengan pengaturan itu, Claude Code membaca file plugin langsung dari `my-marketplace/plugins/`. Edit Anda berlaku saat mulai sesi berikutnya atau saat Anda menjalankan `/reload-plugins` dalam sesi, tanpa perubahan pada `version` plugin.

Orang yang menginstal dari marketplace yang dihosting Anda mendapatkan salinan dalam cache plugin sebagai gantinya. Untuk bagaimana mereka menerima versi baru, lihat [Jaga pengguna tetap terbaru](/docs/id/plugins/host-marketplace#keep-users-up-to-date).

<h3 id="remove-the-marketplace-to-start-over">
  Hapus marketplace untuk memulai lagi
</h3>

Untuk menghapus semuanya dan memulai lagi, jalankan `claude plugin marketplace remove my-marketplace` di shell Anda. Perintah menghapus marketplace dan mencopot plugin-nya.

<h2 id="host-your-marketplace">
  Hosting marketplace Anda
</h2>

Setelah Anda dapat menginstal plugin dari marketplace di mesin Anda sendiri, seperti dalam [Buat marketplace](#create-a-marketplace), dorong direktori marketplace ke host git.

Rekan tim Anda kemudian menjalankan `claude plugin marketplace add <owner>/<repo>` di shell mereka untuk repositori GitHub, atau perintah yang sama dengan URL repositori. Mereka kemudian menginstal plugin berdasarkan nama seperti dalam [panduan walkthrough](#create-a-marketplace).

Untuk akses repositori pribadi, pembaruan, versioning, dan penggantian nama atau penghapusan entri, lihat [Host dan pertahankan marketplace](/docs/id/plugins/host-marketplace).

<h2 id="next-steps">
  Langkah berikutnya
</h2>

* [Host dan pertahankan marketplace](/docs/id/plugins/host-marketplace): pilih host, jaga pengguna tetap terbaru, dan ganti nama atau hapus plugin dengan aman
* [Referensi marketplace](/docs/id/plugins/marketplace-reference): bidang `marketplace.json` dan jenis sumber
* [Kelola plugin untuk organisasi Anda](/docs/id/plugins/org): perlukan marketplace dan plugin-nya di setiap mesin
* [Sarankan plugin berdasarkan relevansi](/docs/id/plugins/relevance): buat Claude Code menyarankan plugin dari marketplace Anda saat sesi cocok
