> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Percepat respons dengan mode cepat

> Dapatkan respons Opus yang lebih cepat di Claude Code dengan mengaktifkan mode cepat.

<Note>
  Mode cepat berada dalam [pratinjau penelitian](#research-preview). Fitur, harga, dan ketersediaan dapat berubah berdasarkan umpan balik.
</Note>

Mode cepat adalah konfigurasi kecepatan tinggi untuk Claude Opus, membuat model hingga 2,5x lebih cepat dengan biaya per token yang lebih tinggi. Aktifkan dengan `/fast` ketika Anda membutuhkan kecepatan untuk pekerjaan interaktif seperti iterasi cepat atau debugging langsung, dan nonaktifkan ketika biaya lebih penting daripada latensi.

Mode cepat bukan model yang berbeda. Mode ini menggunakan Claude Opus dengan konfigurasi API berbeda yang memprioritaskan kecepatan daripada efisiensi biaya. Anda mendapatkan kualitas dan kemampuan yang identik dengan respons yang lebih cepat. Mode cepat didukung pada Opus 4.8 dan Opus 4.7. Mode ini tidak tersedia pada Sonnet, Haiku, atau model lainnya.

<Warning>
  Mode cepat untuk Opus 4.7 sudah usang sejak 25 Juni 2026, dan akan dihapus pada 24 Juli 2026. Setelah penghapusan, permintaan mode cepat pada Opus 4.7 mengembalikan kesalahan dan tidak kembali ke Opus 4.7 standar. Migrasikan ke Opus 4.8 untuk mempertahankan percepatan.
</Warning>

Yang perlu diketahui:

* Gunakan `/fast` untuk mengaktifkan mode cepat di Claude Code CLI. Mode cepat tidak didukung di ekstensi VS Code.
* Harga mode cepat per MTok input/output adalah \$10/\$50 pada Opus 4.8 dan \$30/\$150 pada Opus 4.7.
* Tersedia untuk semua pengguna Claude Code pada paket berlangganan (Pro/Max/Team/Enterprise) dan Claude Console.
* Untuk pengguna Claude Code pada paket berlangganan (Pro/Max/Team/Enterprise), mode cepat tersedia hanya melalui penggunaan kredit dan tidak termasuk dalam batas laju penggunaan berlangganan.

<h2 id="toggle-fast-mode">
  Aktifkan mode cepat
</h2>

Aktifkan mode cepat dengan salah satu cara berikut:

* Ketik `/fast` dan tekan Tab untuk mengaktifkan atau menonaktifkan
* Atur `"fastMode": true` di [file pengaturan pengguna Anda](/docs/id/settings)

Secara default, mode cepat yang Anda aktifkan dalam sesi interaktif bertahan di seluruh sesi. Dalam [mode non-interaktif](/docs/id/headless), dengan flag `-p`, `/fast` hanya berfungsi dalam sesi yang diluncurkan dengan mode cepat dalam nilai [`--settings`](/docs/id/cli-reference#cli-flags) nya, misalnya `claude -p --settings '{"fastMode": true}'`; toggle kemudian berlaku hanya untuk sesi itu dan tidak disimpan sebagai default Anda, dan dalam sesi non-interaktif lainnya perintah melaporkan bahwa mode cepat tidak tersedia. Anda dapat mengonfigurasi mode cepat untuk disetel ulang setiap sesi. Lihat [require per-session opt-in](#require-per-session-opt-in) untuk detail.

Untuk efisiensi biaya terbaik, aktifkan mode cepat di awal sesi daripada beralih di tengah percakapan. Lihat [understand the cost tradeoff](#understand-the-cost-tradeoff) untuk detail.

Ketika Anda mengaktifkan mode cepat:

* Jika Anda berada di model yang berbeda, Claude Code secara otomatis beralih ke Opus
* Anda akan melihat pesan konfirmasi: "Fast mode ON"
* Ikon kecil `↯` muncul di sebelah prompt saat mode cepat aktif
* Jalankan `/fast` lagi kapan saja untuk memeriksa apakah mode cepat aktif atau tidak

Ketika Anda menonaktifkan mode cepat dengan `/fast` lagi, Anda tetap berada di Opus. Model tidak kembali ke model sebelumnya. Untuk beralih ke model yang berbeda, gunakan `/model`.

Beralih ke model yang tidak mendukung mode cepat mematikan mode cepat. Beralih kembali ke model Opus yang didukung menyalakannya lagi ketika preferensi mode cepat yang disimpan Anda aktif, preferensi yang sama yang dimulai sesi baru secara default. Dengan [per-session opt-in](#require-per-session-opt-in) yang dikonfigurasi, beralih kembali tidak menyalakan mode cepat lagi; jalankan `/fast` untuk mengaktifkannya kembali. Mode cepat tidak pernah menyala untuk sesi yang preferensi yang disimpannya mati, dan ikon `↯` serta konfirmasi `Fast mode ON` muncul kapan pun itu diaktifkan. Sebelum v2.1.208, mode cepat tetap mati setelah Anda beralih kembali sampai Anda menjalankan `/fast` lagi.

Opus 4.8 adalah default mode cepat di Claude Code v2.1.154 dan lebih baru. Pada v2.1.142 hingga v2.1.153, mode cepat default ke Opus 4.7.

<h2 id="understand-the-cost-tradeoff">
  Pahami pertukaran biaya
</h2>

Mode cepat memiliki harga per-token yang lebih tinggi daripada Opus standar, dengan pengganda yang bervariasi menurut model:

| Model    | Input (MTok) | Output (MTok) |
| -------- | ------------ | ------------- |
| Opus 4.8 | \$10         | \$50          |
| Opus 4.7 | \$30         | \$150         |

Harga mode cepat datar di seluruh jendela konteks 1M token penuh. Untuk tarif Opus standar yang akan dibandingkan, lihat [referensi harga Claude](https://platform.claude.com/docs/id/about-claude/pricing).

Pertama kali Anda mengaktifkan mode cepat dalam percakapan, Anda membayar harga token input tanpa cache mode cepat penuh untuk seluruh konteks percakapan. Semakin dalam Anda berada dalam percakapan, semakin mahal biayanya, jadi mengaktifkan mode cepat dari awal lebih murah. Biaya diterapkan sekali per percakapan, jadi mematikan dan menyalakan kembali mode cepat nanti tidak mengulanginya. Untuk mekanismenya, lihat [bagaimana mode cepat berinteraksi dengan prompt cache](/docs/id/prompt-caching#turning-on-fast-mode).

<h2 id="decide-when-to-use-fast-mode">
  Tentukan kapan menggunakan mode cepat
</h2>

Mode cepat terbaik untuk pekerjaan interaktif di mana latensi respons lebih penting daripada biaya:

* Iterasi cepat pada perubahan kode
* Sesi debugging langsung
* Pekerjaan sensitif waktu dengan tenggat waktu ketat

Mode standar lebih baik untuk:

* Tugas otonomi jangka panjang di mana kecepatan kurang penting
* Pemrosesan batch atau pipeline CI/CD
* Beban kerja sensitif biaya

<h3 id="fast-mode-vs-effort-level">
  Mode cepat vs tingkat usaha
</h3>

Mode cepat dan tingkat usaha keduanya mempengaruhi kecepatan respons, tetapi dengan cara yang berbeda:

| Pengaturan                     | Efek                                                                                                  |
| ------------------------------ | ----------------------------------------------------------------------------------------------------- |
| **Mode cepat**                 | Kualitas model yang sama, latensi lebih rendah, biaya lebih tinggi                                    |
| **Tingkat usaha lebih rendah** | Waktu pemikiran lebih sedikit, respons lebih cepat, potensi kualitas lebih rendah pada tugas kompleks |

Anda dapat menggabungkan keduanya: gunakan mode cepat dengan [tingkat usaha](/docs/id/model-config#adjust-effort-level) yang lebih rendah untuk kecepatan maksimal pada tugas yang mudah.

<h2 id="requirements">
  Persyaratan
</h2>

Mode cepat memerlukan semua hal berikut:

* **Hanya API Anthropic atau langganan**: mode cepat tersedia melalui API Konsol Anthropic dan untuk paket langganan Claude menggunakan penggunaan kredit. Mode ini tidak tersedia di Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry, atau Claude Platform di AWS.
* **Penggunaan kredit diaktifkan**: akun Anda harus memiliki penggunaan kredit diaktifkan, yang memungkinkan penagihan di luar penggunaan yang disertakan dalam paket Anda. Untuk akun individual, aktifkan ini di [pengaturan penagihan Konsol Anda](https://platform.claude.com/settings/billing). Untuk Teams dan Enterprise, admin harus mengaktifkan penggunaan kredit untuk organisasi.

<Note>
  Penggunaan mode cepat ditagih langsung ke penggunaan kredit, bahkan jika Anda memiliki penggunaan yang tersisa di paket Anda. Ini berarti token mode cepat tidak dihitung terhadap penggunaan yang disertakan dalam paket Anda dan dikenakan biaya dengan tarif mode cepat dari token pertama.
</Note>

* **Aktivasi Owner untuk Teams dan Enterprise**: mode cepat dinonaktifkan secara default untuk organisasi Teams dan Enterprise. Seorang Owner harus secara eksplisit [mengaktifkan mode cepat](#enable-fast-mode-for-your-organization) sebelum pengguna dapat mengaksesnya.

<Note>
  Jika mode cepat belum diaktifkan untuk organisasi Anda, perintah `/fast` akan menampilkan "Fast mode has been disabled by your organization." Jika daftar allowlist [`availableModels`](/docs/id/model-config#restrict-model-selection) organisasi Anda mengecualikan model Opus mode cepat, `/fast` ditolak dengan "is not in your organization's allowed models". Pengecualiannya adalah sesi yang sudah berjalan pada model Opus yang diizinkan yang mendukung mode cepat: `/fast` kemudian mengaktifkan mode cepat pada model Anda saat ini alih-alih beralih model.
</Note>

<h3 id="enable-fast-mode-for-your-organization">
  Aktifkan mode cepat untuk organisasi Anda
</h3>

Tempat Anda mengaktifkan mode cepat tergantung pada produk mana yang digunakan organisasi Anda:

* **Konsol** (pelanggan API): admin mengaktifkannya di [preferensi Claude Code](https://platform.claude.com/claude-code/preferences)
* **Claude AI** (Teams dan Enterprise): Owner mengaktifkannya di [Admin Settings > Claude Code](https://claude.ai/admin-settings/claude-code)

Opsi lain untuk menonaktifkan mode cepat sepenuhnya adalah dengan menetapkan `CLAUDE_CODE_DISABLE_FAST_MODE=1`. Lihat [Variabel lingkungan](/docs/id/env-vars).

<h3 id="require-per-session-opt-in">
  Require per-session opt-in
</h3>

Secara default, mode cepat yang diaktifkan pengguna dalam sesi interaktif bertahan di seluruh sesi: mode ini tetap aktif di sesi mendatang. Untuk mengubah ini, atur `fastModePerSessionOptIn` ke `true` di file [pengaturan](/docs/id/settings#settings-files) apa pun, yang menyebabkan setiap sesi dimulai dengan mode cepat mati dan memerlukan pengguna untuk secara eksplisit mengaktifkannya dengan `/fast`. Owners pada paket [Teams](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=fast_mode_teams#team-&-enterprise) atau [Enterprise](https://anthropic.com/contact-sales?utm_source=claude_code\&utm_medium=docs\&utm_content=fast_mode_enterprise) dapat menerapkannya di seluruh organisasi melalui [pengaturan yang dikelola server](/docs/id/server-managed-settings).

```json theme={null}
{
  "fastModePerSessionOptIn": true
}
```

Ini berguna untuk mengontrol biaya di organisasi di mana pengguna menjalankan beberapa sesi bersamaan. Pengguna masih dapat mengaktifkan mode cepat dengan `/fast` ketika mereka membutuhkan kecepatan, tetapi mode ini disetel ulang di awal setiap sesi baru. Preferensi mode cepat pengguna masih disimpan, jadi menghapus pengaturan ini mengembalikan perilaku persisten default.

<h2 id="handle-rate-limits">
  Tangani batas laju
</h2>

Mode cepat memiliki batas laju terpisah dari Opus standar. Mode cepat pada Opus 4.8 dan Opus 4.7 berbagi pool batas laju yang sama: penggunaan pada salah satu dari mereka menarik dari batas yang sama. Ketika Anda mencapai batas laju mode cepat atau kehabisan kredit penggunaan:

1. Mode cepat secara otomatis kembali ke kecepatan standar
2. Ikon `↯` berubah menjadi abu-abu untuk menunjukkan cooldown
3. Anda terus bekerja dengan kecepatan dan harga standar
4. Ketika cooldown berakhir, mode cepat secara otomatis diaktifkan kembali

Untuk menonaktifkan mode cepat secara manual daripada menunggu cooldown, jalankan `/fast` lagi.

<h2 id="research-preview">
  Pratinjau penelitian
</h2>

Mode cepat adalah fitur pratinjau penelitian. Ini berarti:

* Fitur dapat berubah berdasarkan umpan balik
* Ketersediaan dan harga dapat berubah
* Konfigurasi API yang mendasar dapat berkembang

Laporkan masalah atau umpan balik melalui saluran dukungan Anthropic biasa Anda.

<h2 id="see-also">
  Lihat juga
</h2>

* [Konfigurasi model](/docs/id/model-config): beralih model dan sesuaikan tingkat usaha
* [Kelola biaya secara efektif](/docs/id/costs): lacak penggunaan token dan kurangi biaya
* [Konfigurasi baris status](/docs/id/statusline): tampilkan informasi model dan konteks
