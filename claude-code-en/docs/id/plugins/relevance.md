> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Rekomendasikan plugins untuk organisasi Anda

> Tambahkan blok relevansi ke entri plugin marketplace sehingga Claude Code menyarankannya ketika pekerjaan pengguna cocok, dan daftarkan whitelist marketplace dalam pengaturan terkelola.

Claude Code dapat menyarankan pemasangan plugin dari marketplace organisasi Anda ketika sesi pengguna cocok dengan sinyal yang Anda tentukan untuk plugin tersebut. Sinyal mencakup direktori kerja, file yang telah dibaca Claude, dan perintah yang telah dijalankan Claude. Anda menentukannya dengan menambahkan blok `relevance` ke entri plugin di `marketplace.json`.

Operator marketplace menulis entri `relevance`. Administrator kemudian mendaftarkan whitelist marketplace dalam pengaturan terkelola. Pengguna tidak melihat saran dari marketplace sampai marketplace tersebut didaftarkan whitelist.

<Note>
  Kasus-kasus ini tercakup di halaman lain:

  * **Anda ingin memasang plugins**: lihat [Pasang dan kelola plugins](/docs/id/plugins/install)
  * **Anda ingin mematikan saran**: lihat [Pahami cara kerja relevansi plugin](#understand-how-plugin-relevance-works)
</Note>

Mulai dengan bagian untuk peran Anda:

* **Operator marketplace**: baca [cara kerja saran](#understand-how-plugin-relevance-works), kemudian [tambahkan relevansi ke entri plugin](#add-relevance-to-a-plugin-entry) dan [validasi marketplace Anda](#validate-your-marketplace)
* **Administrator**: [aktifkan saran dalam pengaturan terkelola](#enable-suggestions-in-managed-settings)

<h2 id="understand-how-plugin-relevance-works">
  Pahami cara kerja relevansi plugin
</h2>

Setiap entri plugin di `marketplace.json` dapat menyertakan objek `relevance`. Objek tersebut menamai topik dan satu atau lebih sinyal. Sinyal adalah pola yang diuji Claude Code terhadap sesi saat ini, seperti direktori kerja atau file yang telah dibaca Claude.

Pencocokan sinyal terjadi secara lokal di mesin pengguna dan tidak menambah lalu lintas jaringan. Claude Code tidak melaporkan sinyal mana yang cocok atau nilainya kepada Anthropic atau operator marketplace.

Ketika sinyal cocok dan plugin belum terpasang, Claude Code menyarankan plugin di tempat-tempat ini:

* **Spinner tip**: pesan dengan perintah `/plugin install` muncul di bawah spinner saat Claude merespons.
* **Notifikasi awal sesi**: jika sinyal `cwd` cocok dengan direktori kerja, notifikasi satu baris muncul sebelum pengguna mengirim pesan pertama.
* **Tab Discover `/plugin`**: plugin disematkan ke bagian atas daftar Discover.

[Pratinjau apa yang dilihat pengguna](#preview-what-the-user-sees) menunjukkan teks pasti dari masing-masing dan seberapa sering mereka berulang.

Claude Code tidak pernah memasang plugin secara otomatis. Pengguna selalu mengonfirmasi.

Spinner tip dan notifikasi awal sesi keduanya berhenti muncul ketika pengguna atau proyek menetapkan [`spinnerTipsEnabled`](/docs/id/settings-reference#spinnertipsenabled) ke `false`, atau ketika [`spinnerTipsOverride`](/docs/id/settings-reference#spinnertipsoverride) dengan `excludeDefault` menggantikan tips bawaan. Pin tab Discover tidak terpengaruh oleh pengaturan mana pun.

<h2 id="add-relevance-to-a-plugin-entry">
  Tambahkan relevansi ke entri plugin
</h2>

Tambahkan objek `relevance` ke entri plugin di `marketplace.json` Anda. Contoh berikut mendeklarasikan bahwa plugin `terraform-helpers` relevan ketika Claude membaca file `.tf` atau menjalankan `terraform`:

```json theme={null}
{
  "name": "your-marketplace",
  "owner": { "name": "Your Org" },
  "plugins": [
    {
      "name": "terraform-helpers",
      "source": "./plugins/terraform-helpers",
      "description": "Your organization's Terraform conventions and helpers",
      "relevance": {
        "topic": "Terraform",
        "signals": {
          "cli": ["terraform"],
          "filesRead": ["**/*.tf"]
        }
      }
    }
  ]
}
```

Selama tidak ada sinyal yang cocok, plugin mempertahankan posisi normalnya di daftar Discover dan tidak muncul sebagai spinner tip.

Untuk memeriksa blok sebelum menerbitkan, [validasi marketplace Anda](#validate-your-marketplace).

<h2 id="field-reference">
  Referensi bidang
</h2>

Objek `relevance` dan objek `signals` bersarangnya menerima bidang dalam tabel berikut.

Klien yang lebih lama masih memuat marketplace yang menggunakan bidang `relevance` yang tidak mereka kenali, karena bidang yang tidak dikenali di bawah `relevance` dan `relevance.signals` diabaikan saat waktu muat. Bidang yang dikenali yang nilainya melebihi batasnya dalam [referensi bidang](#field-reference) membatalkan seluruh entri plugin, dan pengguna tidak dapat memasang plugin tersebut dari marketplace sampai Anda memperbaikinya; `claude plugin validate` melaporkan batas yang sama.

<h3 id="relevance">
  `relevance`
</h3>

| Bidang    | Tipe   | Deskripsi                                                                                                                                                                  |
| :-------- | :----- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `topic`   | string | Opsional. Frasa yang mengisi "Bekerja dengan *topic*?" dalam spinner tip. Default ke nama plugin dengan setiap segmen tanda hubung dikapitalisasi. Maksimal 64 karakter.   |
| `signals` | object | Pencocokan yang menentukan kapan plugin relevan. Claude Code menyarankan plugin hanya jika setidaknya satu sinyal diatur. Lihat [`relevance.signals`](#relevance-signals). |

`topic` sering kali adalah nama produk, misalnya `Terraform`. Gunakan domain seperti `design` ketika nama plugin tidak terdengar alami sebagai topik.

<h3 id="relevance-signals">
  `relevance.signals`
</h3>

Objek `signals` menerima bidang berikut.

| Bidang         | Tipe             | Deskripsi                                                                                                                                                                                                                                                           | Batas                                                                                                 |
| :------------- | :--------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | :---------------------------------------------------------------------------------------------------- |
| `cwd`          | array of strings | Pola glob yang dicocokkan terhadap direktori kerja sesi. Lihat [pencocokan direktori kerja](#working-directory-matching).                                                                                                                                           | 10 pola dari 256 karakter masing-masing                                                               |
| `cli`          | array of strings | Nama perintah dari perintah shell yang telah dijalankan Claude sesi ini, misalnya `["terraform"]`. Pencocokan tepat. Lihat [pencocokan nama perintah](#command-name-matching).                                                                                      | 10 entri dari 64 karakter masing-masing                                                               |
| `hosts`        | array of strings | Nama host yang terlihat dalam URL `http://` atau `https://` dalam perintah Bash sesi ini, misalnya `["registry.terraform.io"]`. Hanya nama host huruf kecil telanjang: tidak ada skema, port, atau jalur. Pencocokan tepat tidak peka huruf besar-kecil.            | 20 entri dari 128 karakter masing-masing                                                              |
| `filesRead`    | array of strings | Pola glob yang dicocokkan terhadap jalur file yang telah dibaca Claude sesi ini, misalnya `["**/*.tf"]`. Garis miring maju dinormalisasi dan tidak peka huruf besar-kecil.                                                                                          | 10 pola dari 256 karakter masing-masing                                                               |
| `manifestDeps` | array of objects | Dependensi yang dideklarasikan dalam manifes paket yang telah dibaca Claude sesi ini. Setiap entri adalah `{ "file": "...", "pattern": "..." }`, di mana kedua nilai adalah ekspresi reguler. Lihat [pencocokan dependensi manifes](#manifest-dependency-matching). | 10 entri, setiap nilai paling banyak 256 karakter. File manifes yang lebih besar dari 512 KB dilewati |

Sinyal `filesRead` dan `manifestDeps` juga cocok dengan file yang telah ditulis atau diedit Claude sesi ini dan terhadap file memori `CLAUDE.md` yang dimuat otomatis proyek.

<h4 id="working-directory-matching">
  Pencocokan direktori kerja
</h4>

`cwd` adalah satu-satunya sinyal yang dapat cocok saat awal sesi, sebelum pengguna mengirim pesan pertama.

Claude Code mencocokkan setiap pola `cwd` sebagai berikut:

* Pola dicocokkan terhadap direktori kerja sebagai jalur absolut. Ketika sesi berada di dalam repositori git, pola juga dicocokkan terhadap jalur direktori kerja relatif terhadap akar repositori.
* Pencocokan dinormalisasi garis miring maju dan tidak peka huruf besar-kecil.
* Setiap pola cocok dengan direktori itu sendiri dan semuanya di bawahnya, jadi `infra`, `infra/`, dan `infra/**` berperilaku identik.

<h4 id="command-name-matching">
  Pencocokan nama perintah
</h4>

Claude Code mencatat satu nama perintah untuk setiap perintah shell yang dijalankan Claude: token pertama setelah penetapan variabel lingkungan awal apa pun dan `sudo`. Perintah gabungan hanya berkontribusi pada perintah terdepan mereka, jadi `cd infra && terraform plan` mencatat `cd`, bukan `terraform`.

<h4 id="manifest-dependency-matching">
  Pencocokan dependensi manifes
</h4>

Setiap entri `manifestDeps` memasangkan dua string sumber JavaScript `RegExp`:

* `file`: dicocokkan tidak peka huruf besar-kecil terhadap jalur file manifes. Jalur biasanya absolut, jadi jangkarkan pola di akhir daripada di awal. Jalur tidak dinormalisasi pemisah untuk sinyal ini, jadi jalur Windows menggunakan garis miring terbalik.
* `pattern`: dicocokkan peka huruf besar-kecil terhadap isi file tersebut.

Contoh berikut menggunakan `manifestDeps` untuk menyarankan plugin Anda setelah Claude membaca `package.json` yang bergantung pada paket npm SDK Anda, bernama `your-sdk` di sini.

```json theme={null}
{
  "name": "your-plugin",
  "source": "./plugins/your-plugin",
  "relevance": {
    "signals": {
      "manifestDeps": [
        {
          "file": "[/\\\\]package\\.json$",
          "pattern": "\"your-sdk\"\\s*:"
        }
      ]
    }
  }
}
```

Dalam contoh ini, pola `file` menggunakan `[/\\\\]` sehingga cocok dengan pemisah jalur garis miring maju dan garis miring terbalik, dan `\\.` sehingga titik adalah literal. Dalam JSON, setiap garis miring terbalik dalam ekspresi reguler ditulis dua kali.

<h2 id="validate-your-marketplace">
  Validasi marketplace Anda
</h2>

Di shell Anda, jalankan `claude plugin validate` terhadap direktori marketplace Anda untuk memeriksa blok `relevance` sebelum menerbitkan:

```bash theme={null}
claude plugin validate ./my-marketplace
```

Validator melaporkan kesalahan dan peringatan pada blok `relevance`, termasuk ini:

* Melaporkan kunci yang tidak dikenali di bawah `relevance` dan `relevance.signals` sebagai peringatan
* Menandai nilai `relevance` yang bukan objek
* Menolak entri `signals.hosts` yang menyertakan skema, port, atau jalur

Setiap temuan dicetak dengan jalur bidang yang menjadi perhatiannya, dan output berakhir dengan `Validation passed`, `Validation passed with warnings`, atau `Validation failed`.

<h2 id="enable-suggestions-in-managed-settings">
  Aktifkan saran dalam pengaturan terkelola
</h2>

Pengguna tidak melihat saran dari marketplace sampai administrator mendaftarkan whitelist dalam [pengaturan terkelola](/docs/id/plugins/org), bahkan ketika `marketplace.json` mendeklarasikan `relevance`.

Untuk mendaftarkan whitelist marketplace, edit pengaturan terkelola Anda sebagai berikut:

* Tambahkan nama marketplace ke `pluginSuggestionMarketplaces`.
* Untuk marketplace apa pun selain marketplace resmi Anthropic, juga deklarasikan sumber marketplace, baik sebagai entri nama tersebut di [`extraKnownMarketplaces`](/docs/id/plugins/org#require-a-marketplace-and-its-plugins) atau sebagai entri di [`strictKnownMarketplaces`](/docs/id/plugins/org#allowlist-with-strictknownmarketplaces).

Pada mesin di mana marketplace tidak terdaftar, atau terdaftar dengan nama yang didaftarkan whitelist dari sumber yang berbeda, tidak ada saran darinya yang muncul. Pemeriksaan sumber menghentikan sumber yang tidak terkait dari mendaftar dengan nama yang didaftarkan whitelist untuk mendapatkan pluginnya disarankan di seluruh organisasi Anda.

`managed-settings.json` berikut mendaftarkan marketplace organisasi dari repositori GitHub dan mengaktifkan sarannya:

```json theme={null}
{
  "extraKnownMarketplaces": {
    "your-marketplace": {
      "source": {
        "source": "github",
        "repo": "your-org/your-marketplace"
      }
    }
  },
  "pluginSuggestionMarketplaces": ["your-marketplace"]
}
```

Nama marketplace resmi hanya dapat mendaftar dari sumber Anthropic resmi, jadi tidak memerlukan deklarasi sumber. Untuk marketplace resmi, daftarkan whitelist nama saja:

```json theme={null}
{
  "pluginSuggestionMarketplaces": ["claude-plugins-official"]
}
```

<h2 id="preview-what-the-user-sees">
  Pratinjau apa yang dilihat pengguna
</h2>

Ketika sinyal `relevance` plugin cocok selama sesi, tip di bawah spinner berbunyi:

```text theme={null}
Working with Terraform? Install the terraform-helpers plugin:
/plugin install terraform-helpers@your-marketplace
```

Ketika sinyal `cwd` cocok saat awal sesi, notifikasi satu baris berbunyi:

```text theme={null}
plugin suggestion: terraform-helpers@your-marketplace · /plugin
```

Di tab Discover `/plugin`, plugin disematkan di atas hasil lainnya dengan anotasi yang menamai sinyal yang cocok, seperti `suggested for this directory` atau `suggested for terraform commands`.

Claude Code membatasi seberapa sering plugin tertentu disarankan:

* Saran muncul paling banyak sekali setiap tiga sesi di seluruh spinner tip dan notifikasi awal sesi digabungkan.
* Notifikasi awal sesi berhenti muncul setelah spinner tip dan notifikasi telah menunjukkan plugin sebanyak dua kali secara gabungan.
* Baik spinner tip maupun notifikasi awal sesi tidak berulang setelah plugin dipasang.
* Tab Discover menyematkan plugin pertama kali pengguna membuka tab saat sinyal plugin cocok. Claude Code mencatat itu di `~/.claude.json`, jadi setiap kali pengguna membuka `/plugin` di mesin itu, plugin muncul dalam urutan normal.

<h2 id="see-also">
  Lihat juga
</h2>

* [Host marketplace](/docs/id/plugins/host-marketplace): jalankan marketplace yang menghost plugins Anda
* [Referensi marketplace](/docs/id/plugins/marketplace-reference#plugin-entries): setiap bidang yang diterima entri plugin
* [Rekomendasikan plugin Anda dari CLI Anda](/docs/id/plugins/cli-hints): minta pengguna dari CLI Anda sendiri daripada dari sinyal sesi Claude Code
* [Kelola plugins untuk organisasi Anda](/docs/id/plugins/org): `extraKnownMarketplaces`, `strictKnownMarketplaces`, dan sisa kunci kebijakan plugin
