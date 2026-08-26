> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Rekomendasikan plugins untuk organisasi Anda

> Tambahkan blok relevance ke entri plugin marketplace sehingga Claude Code menyarankannya ketika pekerjaan pengguna cocok.

Jika Anda mengoperasikan marketplace plugin untuk organisasi Anda, Anda dapat membuat Claude Code menyarankan plugin tertentu kepada pengguna berdasarkan apa yang sedang mereka kerjakan. Tambahkan blok `relevance` ke entri plugin di `marketplace.json`, kemudian daftarkan marketplace di managed settings. Ketika sesi pengguna cocok dengan salah satu sinyal yang dideklarasikan, Claude Code menampilkan saran instalasi untuk plugin tersebut.

Saran yang dideklarasikan marketplace bersifat opt-in per marketplace melalui [managed settings](/docs/id/settings#settings-files). Tidak ada deklarasi `relevance` marketplace yang menghasilkan saran sampai administrator menambahkannya ke daftar allowlist, termasuk marketplace resmi Anthropic. Claude Code juga menyertakan satu saran bawaan yang independen dari daftar allowlist ini; tip tersebut dan semua tip yang dideklarasikan marketplace dinonaktifkan ketika [`spinnerTipsEnabled`](/docs/id/settings#available-settings) diatur ke `false`.

Fitur ini memerlukan Claude Code v2.1.152 atau lebih baru. Klien yang lebih lama mengabaikan bidang `relevance`.

Halaman ini untuk operator marketplace dan administrator enterprise. Jika Anda mencari untuk menginstal plugins, lihat [Temukan dan instal plugins](/docs/id/discover-plugins).

<h2 id="how-it-works">
  Cara kerjanya
</h2>

Setiap entri plugin di `marketplace.json` dapat membawa objek `relevance`. Objek ini menamai topik dan satu atau lebih sinyal. Sinyal adalah pola yang Claude Code uji terhadap sesi saat ini, seperti direktori kerja atau file yang telah dibaca Claude.

Pencocokan sinyal terjadi secara lokal di mesin pengguna. Pencocokan tidak menambah lalu lintas jaringan dan tidak melaporkan sinyal mana yang cocok, atau nilainya, ke Anthropic atau ke operator marketplace.

Ketika sinyal cocok dan plugin belum diinstal, Claude Code menampilkan plugin di tiga tempat:

* **Spinner tip**: pesan "Working with *topic*? Install the *plugin* plugin" dengan perintah `/plugin install` muncul di bawah spinner saat Claude merespons.
* **Session-start suggestion**: jika sinyal `cwd` cocok dengan direktori kerja, notifikasi satu baris `plugin suggestion: <name>@<marketplace> · /plugin` muncul sebelum giliran pertama. Permukaan ini memerlukan Claude Code v2.1.153 atau lebih baru.
* **`/plugin` Discover tab**: plugin disematkan ke bagian atas daftar Discover dengan anotasi seperti "suggested for this directory" atau "suggested for stripe commands". Permukaan ini memerlukan Claude Code v2.1.154 atau lebih baru.

Spinner tip dan notifikasi session-start adalah bagian dari sistem spinner-tips. Keduanya dinonaktifkan ketika pengguna atau proyek menetapkan `spinnerTipsEnabled` ke `false`, atau ketika `spinnerTipsOverride` kustom dikonfigurasi dengan `excludeDefault`. Pin tab Discover independen dari pengaturan tip.

Claude Code tidak pernah menginstal plugin secara otomatis. Pengguna selalu mengonfirmasi.

<h2 id="add-relevance-to-a-plugin-entry">
  Tambahkan relevance ke entri plugin
</h2>

Tambahkan objek `relevance` ke entri plugin di `marketplace.json` Anda. Contoh berikut mendeklarasikan bahwa plugin `terraform-helpers` relevan ketika Claude membaca file `.tf` atau ketika Claude menjalankan `terraform`:

```json theme={null}
{
  "name": "acme-corp-plugins",
  "owner": { "name": "Acme Platform Team" },
  "plugins": [
    {
      "name": "terraform-helpers",
      "source": "./plugins/terraform-helpers",
      "description": "Acme conventions and helpers for Terraform",
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

Plugin dengan blok `relevance` tetapi tanpa sinyal yang cocok berperilaku seperti entri marketplace lainnya. Plugin ini muncul di daftar Discover di posisi normalnya dan tidak pernah muncul sebagai spinner tip.

<h2 id="field-reference">
  Referensi bidang
</h2>

<h3 id="relevance">
  `relevance`
</h3>

| Bidang    | Tipe   | Deskripsi                                                                                                                                                                                                                                                                                                                                                          |
| :-------- | :----- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `topic`   | string | Opsional. Frasa yang mengisi "Working with *topic*?" di spinner tip. Sering kali nama produk, misalnya `Stripe`. Gunakan domain seperti `design` ketika nama plugin tidak terbaca secara alami sebagai topik. Default ke nama plugin dengan setiap segmen tanda hubung dikapitalisasi. Notifikasi session-start tidak menggunakan nilai ini. Maksimal 64 karakter. |
| `signals` | object | Matcher yang menentukan kapan plugin relevan. Setidaknya satu sinyal diperlukan agar plugin dapat disarankan. Lihat tabel di bawah.                                                                                                                                                                                                                                |

<h3 id="relevance-signals">
  `relevance.signals`
</h3>

| Bidang         | Tipe             | Deskripsi                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| :------------- | :--------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `cwd`          | array of strings | Pola Glob yang dicocokkan dengan direktori kerja sesi. Dicocokkan sebagai jalur absolut dan, ketika berada di dalam repositori git, sebagai jalur relatif terhadap akar repositori. Garis miring dinormalisasi dan tidak peka huruf besar-kecil. Setiap pola cocok dengan direktori itu sendiri dan semuanya di bawahnya, jadi `infra`, `infra/`, dan `infra/**` berperilaku identik. Ini adalah satu-satunya sinyal yang dapat cocok pada awal sesi, sebelum giliran pertama. Maksimal 10 pola dengan 256 karakter masing-masing.                                                                                                                                                                                                                                                                                                                                                                         |
| `cli`          | array of strings | Nama perintah dari perintah shell yang telah dijalankan Claude sesi ini, misalnya `["stripe"]`. Berlaku di setiap platform: perintah yang dijalankan di Windows melalui PowerShell atau Git Bash dicatat dengan cara yang sama. Claude Code mencatat satu nama perintah per invokasi alat shell: token pertama setelah penugasan variabel lingkungan apa pun dan `sudo`. Perintah gabungan hanya berkontribusi pada perintah terdepan mereka, jadi `cd infra && terraform plan` mencatat `cd`, bukan `terraform`. Kecocokan tepat. Maksimal 10 entri dengan 64 karakter masing-masing.                                                                                                                                                                                                                                                                                                                     |
| `hosts`        | array of strings | Nama host yang terlihat di URL `http://` atau `https://` dalam perintah Bash sesi ini, misalnya `["api.stripe.com"]`. Hanya nama host huruf kecil telanjang: tanpa skema, port, atau jalur. Kecocokan tepat tidak peka huruf besar-kecil. Maksimal 20 entri dengan 128 karakter masing-masing.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `filesRead`    | array of strings | Pola Glob yang dicocokkan dengan jalur file yang telah dibaca Claude sesi ini, misalnya `["**/*.tf"]`. Garis miring dinormalisasi dan tidak peka huruf besar-kecil. Maksimal 10 pola dengan 256 karakter masing-masing.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `manifestDeps` | array of objects | Dependensi yang dideklarasikan dalam manifes paket yang telah dibaca Claude sesi ini. Setiap entri adalah `{ "file": "...", "pattern": "..." }`, di mana `file` adalah ekspresi reguler yang dicocokkan dengan jalur file manifes seperti yang dicatat dalam status sesi, biasanya jalur absolut, dan `pattern` adalah ekspresi reguler yang dicocokkan dengan konten file tersebut. Jangkar `file` di akhir, misalnya `[/\\\\]package\\.json$` dalam bentuk JSON-escaped, karena pola yang dimulai dengan jangkar tidak pernah cocok dengan jalur absolut. Jalur tidak dinormalisasi pemisah untuk sinyal ini, jadi jalur Windows menggunakan garis miring terbalik. File manifes yang lebih besar dari 512 KB dilewati. Kedua nilai adalah string sumber JavaScript `RegExp` paling banyak 256 karakter. `file` cocok tidak peka huruf besar-kecil. `pattern` peka huruf besar-kecil. Maksimal 10 entri. |

Sinyal `cli`, `hosts`, `filesRead`, dan `manifestDeps` memerlukan riwayat sesi, jadi mereka hanya dapat cocok pada spinner tip dan tab Discover. Hanya `cwd` yang dapat cocok pada awal sesi. Sinyal `filesRead` dan `manifestDeps` menguji status file yang dicatat sesi, yang juga mencakup file yang telah ditulis atau diedit Claude dan file memori `CLAUDE.md` yang dimuat otomatis.

Contoh berikut menggunakan `manifestDeps` untuk menyarankan plugin Stripe setelah Claude membaca `package.json` yang bergantung pada `stripe`. Pola `file` menggunakan `[/\\\\]` sehingga cocok dengan pemisah jalur garis miring dan garis miring terbalik, dan `\\.` sehingga titik adalah literal. Dalam JSON, setiap garis miring terbalik dalam ekspresi reguler ditulis dua kali.

```json theme={null}
{
  "name": "stripe-helpers",
  "source": "./plugins/stripe-helpers",
  "relevance": {
    "topic": "Stripe",
    "signals": {
      "manifestDeps": [
        {
          "file": "[/\\\\]package\\.json$",
          "pattern": "\"stripe\"\\s*:"
        }
      ]
    }
  }
}
```

<Note>
  Bidang yang tidak dikenal di bawah `relevance` dan `relevance.signals` diabaikan pada waktu muat sehingga klien Claude Code yang lebih lama terus memuat marketplace Anda. Jalankan `claude plugin validate` untuk menampilkannya sebagai peringatan.
</Note>

<h2 id="enable-suggestions-in-managed-settings">
  Aktifkan saran di managed settings
</h2>

Mendeklarasikan `relevance` di `marketplace.json` saja tidak cukup. Administrator harus mendaftarkan marketplace di [managed settings](/docs/id/settings#settings-files) sebelum sarannya muncul kepada pengguna.

Tambahkan nama marketplace ke `pluginSuggestionMarketplaces`. Untuk marketplace apa pun selain marketplace resmi Anthropic, juga deklarasikan sumber marketplace di managed settings yang sama, baik sebagai entri nama tersebut di `extraKnownMarketplaces` atau sebagai entri di `strictKnownMarketplaces`. Nama yang didaftarkan diabaikan jika marketplace yang terdaftar di mesin berasal dari sumber yang berbeda. Ini mencegah sumber yang tidak terkait mendaftarkan di bawah nama yang didaftarkan untuk memiliki pluginnya disarankan di seluruh organisasi Anda.

`managed-settings.json` berikut mendaftarkan marketplace organisasi dari repositori GitHub dan mengaktifkan sarannya:

```json theme={null}
{
  "extraKnownMarketplaces": {
    "acme-corp-plugins": {
      "source": {
        "source": "github",
        "repo": "acme-corp/claude-plugins"
      }
    }
  },
  "pluginSuggestionMarketplaces": ["acme-corp-plugins"]
}
```

Marketplace resmi dikecualikan dari persyaratan deklarasi sumber karena namanya hanya dapat terdaftar dari sumber Anthropic resmi. Mendaftarkan nama saja sudah cukup:

```json theme={null}
{
  "pluginSuggestionMarketplaces": ["claude-plugins-official"]
}
```

Lihat [referensi pengaturan](/docs/id/settings) untuk `pluginSuggestionMarketplaces` dan [`extraKnownMarketplaces`](/docs/id/settings#extraknownmarketplaces) untuk detail konfigurasi lengkap.

<h2 id="what-the-user-sees">
  Apa yang dilihat pengguna
</h2>

Ketika sinyal cocok selama sesi, spinner tip berbunyi:

```text theme={null}
Working with Terraform? Install the terraform-helpers plugin:
/plugin install terraform-helpers@acme-corp-plugins
```

Pada awal sesi, sinyal `cwd` yang cocok menampilkan notifikasi satu baris:

```text theme={null}
plugin suggestion: terraform-helpers@acme-corp-plugins · /plugin
```

Saran plugin tertentu muncul paling banyak sekali setiap tiga sesi di seluruh spinner tip dan notifikasi session-start digabungkan, dan tidak ada yang berulang setelah plugin diinstal. Notifikasi session-start juga berhenti muncul setelah saran telah ditampilkan dua kali.

Di tab `/plugin` Discover, plugin disematkan di atas hasil lainnya dengan anotasi yang menamai sinyal yang cocok, seperti `suggested for this directory` atau `suggested for terraform commands`. Tab Discover menyematkan plugin tertentu sekali; kunjungan kemudian mencantumkannya dalam urutan normal. Pin tab Discover memerlukan Claude Code v2.1.154 atau lebih baru. Pada v2.1.152 hanya spinner tip yang muncul; notifikasi session-start ditambahkan di v2.1.153.

<h2 id="validate-your-marketplace">
  Validasi marketplace Anda
</h2>

Jalankan `claude plugin validate` terhadap direktori marketplace Anda untuk memeriksa blok `relevance` sebelum menerbitkan:

```
claude plugin validate ./my-marketplace
```

Validator melaporkan kunci yang tidak dikenal di bawah `relevance` dan `relevance.signals` sebagai peringatan, menandai nilai `relevance` yang bukan objek, dan menolak entri `signals.hosts` yang menyertakan skema, port, atau jalur.

<h2 id="see-also">
  Lihat juga
</h2>

* [Buat dan distribusikan marketplace plugin](/docs/id/plugin-marketplaces): bangun marketplace yang menghosting plugins Anda
* [Rekomendasikan plugin Anda dari CLI Anda](/docs/id/plugin-hints): minta pengguna dari CLI Anda sendiri alih-alih dari sinyal sesi Claude Code
* [Pengaturan](/docs/id/settings): referensi lengkap untuk `pluginSuggestionMarketplaces` dan `extraKnownMarketplaces`
