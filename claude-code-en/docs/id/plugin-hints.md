> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Rekomendasikan plugin Anda dari CLI Anda

> Keluarkan penanda satu baris dari CLI Anda sehingga Claude Code meminta pengguna untuk memasang plugin resmi Anda.

Jika Anda memelihara CLI atau SDK dan memiliki plugin di marketplace resmi Anthropic, alat Anda dapat meminta pengguna Claude Code untuk memasang plugin tersebut. CLI Anda menulis penanda satu baris ke stderr ketika mendeteksi bahwa itu berjalan di dalam Claude Code. Claude Code membaca penanda, menghapusnya dari output, dan menampilkan prompt pemasangan satu kali kepada pengguna.

Claude Code menghapus baris petunjuk dari output perintah sebelum mengirimkannya ke model, sehingga penanda tidak pernah muncul dalam percakapan dan tidak dihitung terhadap penggunaan token. Protokol tidak memerlukan perintah tambahan dan tidak mengubah apa yang CLI Anda cetak untuk pengguna di luar Claude Code.

Halaman ini adalah untuk pengelola CLI dan SDK. Jika Anda mencari untuk memasang plugin, lihat [Temukan dan pasang plugin](/docs/id/discover-plugins).

<h2 id="how-it-works">
  Cara kerjanya
</h2>

Claude Code menetapkan variabel lingkungan [`CLAUDECODE`](/docs/id/env-vars) ke `1` untuk setiap perintah yang dijalankan melalui alat Bash dan PowerShell, dan untuk perintah [hook](/docs/id/hooks). Dari v2.1.172 juga menetapkan [`CLAUDE_CODE_CHILD_SESSION`](/docs/id/env-vars) ke `1` dalam subproses yang sama. Ketika CLI Anda melihat salah satu variabel ini, itu menulis tag `<claude-code-hint />` yang menutup sendiri ke stderr. Dalam perintah hook, tag petunjuk dilepas dan diabaikan. Hanya output alat Bash dan PowerShell yang memicu prompt pemasangan.

Ketika Claude Code menerima output perintah, itu:

1. Memindai baris petunjuk dan menghapusnya sebelum output mencapai model
2. Memeriksa bahwa petunjuk menargetkan plugin di marketplace Anthropic resmi
3. Memeriksa bahwa plugin belum dipasang dan belum diminta sebelumnya
4. Menampilkan prompt pemasangan kepada pengguna yang menyebutkan perintah yang mengeluarkan petunjuk

Claude Code tidak pernah memasang plugin secara otomatis. Pengguna selalu mengonfirmasi.

<h2 id="emit-the-hint">
  Keluarkan petunjuk
</h2>

Petunjuk hint hanya aktif untuk plugin yang terdaftar di marketplace resmi Anthropic. Lihat [Dapatkan plugin Anda ke marketplace resmi](#get-your-plugin-into-the-official-marketplace) sebelum Anda meluncurkan integrasi.

Gerbang emisi pada variabel lingkungan sehingga penanda tidak mungkin muncul ketika manusia menjalankan CLI Anda secara langsung, kemudian tulis tag ke stderr pada barisnya sendiri. Pilih variabel mana yang akan diperiksa:

* `CLAUDECODE`: diatur pada setiap versi Claude Code, sehingga mencapai sebagian besar sesi. Ini juga diatur dalam sesi tmux dan subprocess server MCP stdio yang Claude Code mulai. Ekstensi IDE juga mengaturnya di terminal terintegrasi mereka, di mana manusia dapat menjalankan CLI Anda secara langsung.
* `CLAUDE_CODE_CHILD_SESSION`: diatur hanya dalam subprocess yang Claude Code sendiri spawn, seperti pemanggilan alat, perintah hook, dan perintah [status line](/docs/id/statusline), sehingga tag biasanya tidak mencapai terminal manusia. Proses yang berumur panjang yang dimulai di dalam sesi, seperti server tmux, menangkap variabel, sehingga shell yang diluncurkan kemudian dari proses itu masih menampilkan tag mentah. Memerlukan Claude Code v2.1.172 atau lebih baru, sehingga sesi pada versi yang lebih lama melewatkan petunjuk.

Contoh berikut gerbang pada `CLAUDECODE` untuk jangkauan maksimal dan mengeluarkan petunjuk untuk plugin bernama `example-cli` di marketplace resmi:

<CodeGroup>
  ```javascript Node.js theme={null}
  if (process.env.CLAUDECODE) {
    process.stderr.write(
      '<claude-code-hint v="1" type="plugin" value="example-cli@claude-plugins-official" />\n',
    )
  }
  ```

  ```python Python theme={null}
  import os, sys

  if os.environ.get("CLAUDECODE"):
      print(
          '<claude-code-hint v="1" type="plugin" value="example-cli@claude-plugins-official" />',
          file=sys.stderr,
      )
  ```

  ```go Go theme={null}
  if os.Getenv("CLAUDECODE") != "" {
      fmt.Fprintln(os.Stderr,
          `<claude-code-hint v="1" type="plugin" value="example-cli@claude-plugins-official" />`)
  }
  ```

  ```shell Shell theme={null}
  [ -n "$CLAUDECODE" ] &&
    printf '%s\n' '<claude-code-hint v="1" type="plugin" value="example-cli@claude-plugins-official" />' >&2
  ```
</CodeGroup>

Ganti `example-cli` dengan nama plugin Anda di marketplace resmi.

<h2 id="choose-where-to-emit">
  Pilih tempat untuk mengeluarkan
</h2>

Anda mengontrol jalur kode mana yang mengeluarkan petunjuk. Claude Code menghilangkan duplikat berdasarkan plugin, jadi mengeluarkan pada setiap invokasi tidak memiliki kelemahan. Titik sentuh yang bekerja dengan baik meliputi:

| Penempatan                               | Mengapa itu bekerja                                                    |
| :--------------------------------------- | :--------------------------------------------------------------------- |
| Output `--help`                          | Claude sering menjalankan help saat menjelajahi CLI yang tidak dikenal |
| Kesalahan subperintah yang tidak dikenal | Mencapai momen ketika Claude bingung tentang antarmuka Anda            |
| Keberhasilan login atau autentikasi      | Pengguna sudah dalam pola pikir pengaturan                             |
| Pesan sambutan first-run                 | Momen onboarding yang alami                                            |

<h2 id="what-the-user-sees">
  Apa yang dilihat pengguna
</h2>

Ketika petunjuk melewati semua pemeriksaan, Claude Code menampilkan prompt seperti berikut:

```text theme={null}
─────────────────────────────────────────────────────────────
  Plugin recommendation

    The example-cli command suggests installing a plugin.

    Plugin: example-cli
    Marketplace: claude-plugins-official
    Official integration for example-cli deployments

    Would you like to install it?
    ❯ 1. Yes, install example-cli
      2. No
      3. No, and don't show plugin installation hints again

─────────────────────────────────────────────────────────────
```

Prompt menyebutkan perintah yang menghasilkan petunjuk sehingga pengguna dapat mendeteksi ketidaksesuaian antara alat dan plugin yang direkomendasikannya. Jika pengguna tidak merespons dalam 30 detik, prompt ditutup sebagai **No**.

Frekuensi prompt dibatasi:

* **Sekali per plugin**: setelah prompt ditampilkan, Claude Code mencatat plugin dan tidak pernah memintanya lagi, terlepas dari jawaban pengguna.
* **Sekali per sesi**: di semua CLI di mesin, paling banyak satu prompt petunjuk muncul per sesi Claude Code.

Memilih **Yes** memasang plugin ke cakupan pengguna. Memilih **No, and don't show plugin installation hints again** menonaktifkan semua prompt petunjuk di masa depan untuk pengguna.

<h2 id="hint-format">
  Format petunjuk
</h2>

Petunjuk adalah tag yang menutup sendiri dengan tiga atribut yang diperlukan.

```text theme={null}
<claude-code-hint v="1" type="plugin" value="example-cli@claude-plugins-official" />
```

| Atribut | Diperlukan | Deskripsi                                                        |
| :------ | :--------- | :--------------------------------------------------------------- |
| `v`     | Ya         | Versi protokol. `1` adalah satu-satunya nilai yang didukung      |
| `type`  | Ya         | Jenis petunjuk. `plugin` adalah satu-satunya nilai yang didukung |
| `value` | Ya         | Pengenal plugin dalam bentuk `name@marketplace`                  |

Nilai atribut dapat dikutip dengan tanda kutip ganda atau dibiarkan tanpa tanda kutip. Nilai tanpa tanda kutip tidak dapat berisi spasi. Urutan escape tidak didukung.

<h2 id="requirements">
  Persyaratan
</h2>

Claude Code memberlakukan dua kondisi sebelum bertindak atas petunjuk. Petunjuk yang gagal salah satu pemeriksaan dijatuhkan:

* **Baris sendiri**: tag harus menempati barisnya sendiri. Tag yang tertanam di tengah baris, misalnya di dalam pernyataan log, diabaikan. Spasi di awal dan akhir baris diizinkan.
* **Marketplace resmi**: `value` harus mereferensikan plugin di marketplace yang dikendalikan Anthropic seperti `claude-plugins-official`. Petunjuk yang menunjuk ke marketplace lain secara diam-diam dijatuhkan.

Baris petunjuk selalu dihapus dari output sebelum mencapai model, bahkan ketika versi atau jenis tidak dikenali, sehingga penanda tidak pernah dihitung terhadap penggunaan token.

Panduan yang tersisa direkomendasikan tetapi tidak diberlakukan. Claude Code tidak dapat mengamati apakah CLI Anda mengikutinya:

* **Tulis ke stderr**: stderr menjaga tag keluar dari pipa shell seperti `example-cli deploy | jq`. Claude Code memindai kedua aliran, jadi stdout juga berfungsi.
* **Gerbang pada variabel lingkungan**: hanya keluarkan ketika `CLAUDECODE` atau `CLAUDE_CODE_CHILD_SESSION` diatur. Lihat [Emit the hint](#emit-the-hint) untuk mengetahui bagaimana kedua variabel berbeda.

<h2 id="get-your-plugin-into-the-official-marketplace">
  Dapatkan plugin Anda ke marketplace resmi
</h2>

Protokol petunjuk hanya berlaku untuk plugin yang terdaftar di marketplace Anthropic resmi, `claude-plugins-official`. Anthropic mengkurasi marketplace tersebut atas kebijakannya sendiri, dan formulir pengajuan dalam aplikasi menambahkan plugin ke [marketplace komunitas](/docs/id/plugins#submit-your-plugin-to-the-community-marketplace) sebagai gantinya, yang protokol petunjuk tidak periksa. Jika Anda bekerja dengan kontak mitra Anthropic, hubungi mereka untuk mengoordinasikan daftar marketplace resmi.

<h2 id="see-also">
  Lihat juga
</h2>

* [Buat plugin](/docs/id/plugins): bangun plugin yang direkomendasikan CLI Anda
* [Buat dan distribusikan marketplace plugin](/docs/id/plugin-marketplaces): host plugin di luar marketplace resmi
* [Variabel lingkungan](/docs/id/env-vars): referensi lengkap untuk `CLAUDECODE` dan variabel terkait
