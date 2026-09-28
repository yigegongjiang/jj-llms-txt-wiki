> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Temukan bug dengan ultrareview

> Jalankan tinjauan kode multi-agen yang mendalam di cloud dengan /code-review ultra untuk menemukan dan memverifikasi bug sebelum Anda merge.

<Note>
  Ultrareview adalah fitur pratinjau penelitian. Fitur, harga, dan ketersediaan dapat berubah berdasarkan umpan balik. Perintah adalah `/code-review ultra`. Ketika ultrareview tersedia untuk akun Anda, `/ultrareview` adalah alias.
</Note>

Ultrareview adalah tinjauan kode yang mendalam yang berjalan sebagai [sesi cloud](/docs/id/claude-code-on-the-web) pada infrastruktur Anthropic. Ketika Anda menjalankan `/code-review ultra`, Claude Code meluncurkan armada agen peninjau dalam sandbox cloud untuk menemukan bug di cabang atau pull request Anda.

Dibandingkan dengan `/code-review` lokal, ultrareview menawarkan:

* **Sinyal yang lebih tinggi**: setiap temuan yang dilaporkan secara independen direproduksi dan diverifikasi, sehingga hasil fokus pada bug nyata daripada saran gaya
* **Cakupan yang lebih luas**: armada agen peninjau yang lebih besar menjelajahi perubahan secara paralel, yang mengungkap masalah yang mungkin terlewatkan oleh tinjauan lokal
* **Tidak ada penggunaan sumber daya lokal**: tinjauan berjalan sepenuhnya dalam sandbox cloud, sehingga terminal Anda tetap bebas untuk pekerjaan lain saat berjalan

Ultrareview memerlukan autentikasi dengan akun claude.ai karena berjalan sebagai sesi cloud pada infrastruktur Anthropic. Jika Anda masuk hanya dengan kunci API, jalankan `/login` dan autentikasi dengan claude.ai terlebih dahulu. Ultrareview tidak tersedia saat menggunakan Claude Code dengan Amazon Bedrock, Google Cloud's Agent Platform, atau Microsoft Foundry, dan tidak tersedia untuk organisasi yang telah mengaktifkan Zero Data Retention. Ketika ultrareview tidak tersedia, `/code-review ultra` menjalankan tinjauan lokal dalam sesi Anda sebagai gantinya.

<h2 id="run-ultrareview-from-the-cli">
  Jalankan ultrareview dari CLI
</h2>

Mulai tinjauan dari repositori git apa pun:

```text theme={null}
/code-review ultra
```

Tanpa argumen, ultrareview meninjau perbedaan antara cabang saat ini Anda dan cabang default, termasuk perubahan yang tidak berkomitmen dan staged. Untuk perubahan yang tidak berkomitmen pada file bernama seperti kredensial atau kunci, seperti file `.env` dan `*.tfvars`, Claude Code mengikuti aturan untuk [mengunggah repositori lokal ke sesi cloud](/docs/id/claude-code-on-the-web#send-local-repositories-without-github).

Untuk tinjauan cabang, Claude Code membundel status repositori dan mengunggahnya ke sandbox jarak jauh; ketika Anda [meninjau pull request](#review-a-pull-request), Claude Code tidak mengunggah apa pun dari mesin Anda.

Sebelum meluncurkan, Claude Code menampilkan dialog konfirmasi dengan cakupan tinjauan, sisa run gratis Anda, dan perkiraan biaya; untuk tinjauan cabang, cakupan mencakup jumlah file dan baris. Setelah Anda mengonfirmasi, tinjauan berlanjut di latar belakang sementara Anda terus menggunakan sesi Anda.

Perintah hanya berjalan ketika Anda memanggilnya dengan `/code-review ultra`; Claude tidak memulai ultrareview dengan sendirinya.

<h3 id="review-against-a-different-base">
  Tinjau terhadap basis yang berbeda
</h3>

Untuk membandingkan terhadap basis selain cabang default, teruskan nama cabang. Contoh ini meninjau cabang saat ini Anda terhadap `develop` sebagai gantinya:

```text theme={null}
/code-review ultra develop
```

Cabang basis tidak perlu ada di klon lokal Anda; Claude Code mengambilnya dari `origin`. Jika nama memiliki kesalahan ketik, Claude Code menyarankan nama cabang terdekat dalam kesalahan.

Commit id atau tag juga berfungsi sebagai basis, dan tinjauan kemudian mencakup perubahan pada cabang Anda sejak commit itu.

<h3 id="review-a-pull-request">
  Tinjau pull request
</h3>

Untuk meninjau pull request GitHub sebagai gantinya dari cabang lokal, teruskan nomor PR:

```text theme={null}
/code-review ultra 1234
```

Perintah juga menerima `#1234`, `PR 1234`, dan URL PR yang ditempel; URL yang ditempel harus menunjuk ke repositori di direktori saat ini Anda.

Dalam mode PR, sandbox jarak jauh mengkloning pull request langsung dari host daripada membundel pohon kerja lokal Anda. Mode PR bekerja dengan repositori di `github.com` dan pada instans [GitHub Enterprise Server](/docs/id/github-enterprise-server) yang telah dihubungkan oleh Owner ke Claude Code.

Untuk repositori di `github.com`, sandbox mengkloning dengan akun GitHub yang terhubung ke akun Claude Anda, jadi akun harus dapat membaca repositori PR. Claude Code memeriksa ini sebelum membuat sesi cloud, kecuali Anda telah menetapkan [`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`](/docs/id/env-vars#variables), dan menolak peluncuran ketika [tidak ada akun yang terhubung](/docs/id/errors#no-github-account-is-connected-to-your-claude-account) atau [akun tidak dapat melihat repositori](/docs/id/errors#your-connected-github-account-cant-see-the-repository); penolakan menyebutkan perbaikannya. Sebelum v2.1.248, Claude Code tidak memeriksa ini sebelum peluncuran.

Jalankan [`/web-setup`](/docs/id/web-quickstart#connect-from-your-terminal) untuk menghubungkan login GitHub CLI Anda ke akun Claude Anda.

<h3 id="post-findings-to-the-pull-request">
  Posting temuan ke pull request
</h3>

Pada Claude Code v2.1.227 atau lebih baru, ketika Anda meninjau pull request di `github.com`, Anda dapat membuat Claude memposting temuan yang selesai ke PR sebagai komentar polos tunggal dari akun GitHub Anda sendiri. Komentar bukan tinjauan atau persetujuan, dan berakhir dengan catatan "Generated by Claude Code". Ketika Anda meninjau cabang atau pull request GitHub Enterprise Server, Claude Code menampilkan temuan di sesi Anda saja.

Claude Code tidak pernah memposting kecuali Anda memilih untuk melakukannya pada run itu, dan `--no-post` adalah default. Posting adalah pilihan yang Anda buat untuk setiap run:

* **Interaktif**: dalam dialog peluncuran, pilih **Run and post the findings to the PR as me**. Jika Anda menambahkan `--post` ke perintah, seperti `/code-review ultra 1234 --post`, Claude Code memilih sebelumnya pilihan itu dan masih bertanya sebelum meluncurkan.
* **Non-interaktif**: jalankan [subperintah `claude ultrareview`](#run-ultrareview-non-interactively) dengan `--post`. Anda menyetujui posting dengan menjalankan subperintah dengan flag, jadi Claude Code memposting tanpa bertanya. Dalam run `claude -p '/code-review ultra'`, Claude Code keluar sebelum temuan tiba, jadi tidak memposting apa pun; gunakan subperintah sebagai gantinya.

Claude Code tidak memposting dari mesin Anda. Ini mengirim ID sesi tinjauan ke Anthropic API, yang memposting temuan tersimpan tinjauan sebagai komentar melalui akun GitHub yang telah Anda hubungkan ke Claude. Posting memerlukan sign-in claude.ai yang sama seperti tinjauan itu sendiri, dan tidak tersedia pada penyedia pihak ketiga atau ketika Anda menetapkan [`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`](/docs/id/env-vars).

Dalam sesi interaktif, Claude Code memulai posting ketika temuan tiba, jadi jaga sesi tetap terbuka sampai tinjauan selesai. Claude Code menyimpan pilihan posting hanya dalam sesi itu. Jika sesi berakhir sebelum tinjauan selesai, Claude Code tidak memposting apa pun, bahkan jika Anda melanjutkan percakapan nanti.

Ketika posting selesai, Claude memberi tahu Anda hasilnya:

* **Posted**: Claude memberi Anda tautan ke komentar.
* **Already posted**: posting sebelumnya dari tinjauan yang sama telah menempatkan komentar di PR, jadi Claude menautkan Anda ke pull request sebagai gantinya memposting lagi.
* **Failed**: Claude memberi tahu Anda mengapa, dan temuan tetap di terminal Anda sehingga Anda dapat mempostingnya dengan tangan.

<h3 id="pass-a-request-in-plain-words">
  Teruskan permintaan dalam kata-kata polos
</h3>

Pada Claude Code v2.1.218 atau lebih baru, Anda juga dapat mendeskripsikan apa yang Anda kerjakan dalam kata-kata polos:

```text theme={null}
/code-review ultra check my auth changes
```

Tinjauan masih mencakup cabang saat ini Anda, cakupan yang sama seperti menjalankan tanpa argumen. Claude menyimpan teks Anda sebagai catatan, ditampilkan dalam dialog peluncuran, dan menghubungkan temuan dengannya ketika tiba.

Claude Code memperlakukan teks Anda sebagai catatan hanya ketika memiliki lebih dari satu kata dan bukan nama cabang atau referensi PR. Ini membaca satu kata sebagai nama cabang atau referensi PR, jadi nama cabang yang salah ketik mendapat kesalahan cabang terdekat dari [Tinjau terhadap basis yang berbeda](#review-against-a-different-base) sebagai gantinya meluncurkan dengan catatan. Jika teks Anda menggabungkan referensi PR dengan kata-kata lain, seperti `check PR 123 again`, Claude Code juga tidak meluncurkan; itu meminta Anda untuk menjalankan kembali dengan nomor PR saja untuk meninjau PR itu, atau tanpa referensi untuk meninjau cabang saat ini Anda.

<Tip>
  Jika repositori Anda terlalu besar untuk dibundel, Claude Code meminta Anda menggunakan mode PR sebagai gantinya. Dorong cabang Anda dan buka PR draft, kemudian jalankan `/code-review ultra <PR-number>`.
</Tip>

<h3 id="diff-limits-and-fallbacks">
  Batas diff dan fallback
</h3>

Ultrareview memeriksa diff sebelum pekerjaan tinjauan apa pun berjalan dan memberi tahu Anda ketika tidak dapat meninjau seperti apa adanya:

* **Diff terlalu besar**: tinjauan cabang dapat mencakup hingga 500 file yang diubah dan 8.000 baris yang diubah secara default. Nilai yang tepat dapat berubah, dan [penolakan](/docs/id/errors#diff-is-too-large-for-ultrareview) menyebutkan yang berlaku, ukuran diff Anda, dan file dengan baris yang paling banyak diubah. Claude Code menolak pull request yang terlalu besar dengan cara yang sama, menyebutkan jumlah file dan barisnya tetapi bukan rincian per-file
* **Tidak ada yang ditinjau**: ketika diff terhadap basis kosong, ultrareview menolak dan menyebutkan cabang atau commit yang dibandingkannya dan kasus yang Anda alami, seperti berada di cabang basis itu sendiri tanpa apa pun yang tidak berkomitmen, atau cabang yang commitnya semuanya sudah bagian dari basis. Ini juga menyarankan cara keluar untuk kasus itu, seperti beralih ke cabang dengan pekerjaan Anda, staging atau committing editan lokal, atau melewatkan basis yang berbeda
* **Commit pertama**: commit pertama repositori tidak memiliki apa pun sebelumnya untuk dibandingkan, jadi ultrareview meninjau setiap file di dalamnya setelah Anda mengonfirmasi dalam dialog peluncuran. Jika Anda memiliki file yang tidak dilacak, itu menolak sebagai gantinya dan memberi tahu Anda untuk `git add` yang ingin Anda tinjau. Batas ukuran yang sama berlaku.

  Commit pertama ditinjau secara keseluruhan hanya setelah konfirmasi itu, jadi subperintah `claude ultrareview` dan `claude -p` menolaknya dan menunjukkan Anda ke sesi interaktif sebagai gantinya. Memerlukan Claude Code v2.1.277 atau lebih baru
* **Tidak ada merge base**: ketika cabang Anda tidak berbagi sejarah dengan cabang basis, atau repositori tidak memiliki cabang basis untuk dibandingkan, ultrareview meninjau setiap file yang dilacak dalam repositori sebagai gantinya. Fallback memerlukan klon penuh dan menerapkan batas ukuran yang sama. Ini diluncurkan hanya ketika Anda mengonfirmasi dalam dialog peluncuran atau menjalankan subperintah `claude ultrareview` sendiri. Dalam `claude -p` dan di mana pun tidak ada yang terjadi, ultrareview menolak, mengatakan tinjauan akan mencakup setiap file, dan menunjukkan Anda ke sesi interaktif.

  Pada checkout tanpa cabang atau ref lainnya, seperti HEAD terlepas yang dibuat dengan checkout `FETCH_HEAD` setelah mengambil URL, Claude Code [menolak tinjauan](/docs/id/errors#your-checkout-has-no-branches) dan menyarankan membuat cabang terlebih dahulu

<h2 id="pricing-and-free-runs">
  Harga dan run gratis
</h2>

Ultrareview adalah fitur premium yang ditagih terhadap penggunaan ekstra daripada penggunaan yang disertakan dalam paket Anda.

| Paket               | Run gratis yang disertakan | Setelah run gratis                                                                                                     |
| ------------------- | -------------------------- | ---------------------------------------------------------------------------------------------------------------------- |
| Pro                 | 3 run gratis               | ditagih sebagai [penggunaan ekstra](https://support.claude.com/id/articles/12429409-extra-usage-for-paid-claude-plans) |
| Max                 | 3 run gratis               | ditagih sebagai [penggunaan ekstra](https://support.claude.com/id/articles/12429409-extra-usage-for-paid-claude-plans) |
| Team dan Enterprise | tidak ada                  | ditagih sebagai [penggunaan ekstra](https://support.claude.com/id/articles/12429409-extra-usage-for-paid-claude-plans) |

* **Run gratis**: tiga run Pro dan Max adalah alokasi satu kali per akun dan tidak diperbarui.
* **Biaya per tinjauan**: setelah Anda menggunakan run gratis, biasanya \$5 hingga \$25 dalam penggunaan ekstra tergantung pada ukuran perubahan, sesuai dengan perkiraan yang ditampilkan dialog peluncuran sebelum setiap run.
* **Kapan run dihitung**: setelah sesi cloud dimulai. Tinjauan yang Anda hentikan lebih awal atau yang gagal diselesaikan masih menggunakan run gratis; tinjauan berbayar hanya ditagih untuk bagian yang berjalan.

Karena ultrareview selalu ditagih sebagai penggunaan ekstra di luar run gratis, akun atau organisasi Anda harus memiliki penggunaan ekstra diaktifkan sebelum Anda dapat meluncurkan tinjauan berbayar. Jika penggunaan ekstra tidak diaktifkan, Claude Code memblokir peluncuran, dan cara Anda mengaktifkannya tergantung pada akses penagihan Anda:

* Jika Anda dapat mengelola penagihan untuk akun Anda, Claude Code menautkan Anda ke pengaturan penagihan tempat Anda dapat mengaktifkan penggunaan ekstra.
* Pada paket Team dan Enterprise, anggota tanpa akses penagihan mengirimkan permintaan dari CLI meminta admin mereka untuk mengaktifkan penggunaan ekstra.

Anda juga dapat menjalankan `/usage-credits` untuk memeriksa atau mengubah pengaturan penggunaan-ekstra Anda.

Claude Code meminta Anda untuk mengonfirmasi penagihan penggunaan-ekstra sekali per percakapan: ketika Anda memulai percakapan baru, misalnya dengan `/clear`, Claude Code menampilkan konfirmasi lagi untuk tinjauan berbayar berikutnya.

<h2 id="track-a-running-review">
  Lacak tinjauan yang sedang berjalan
</h2>

Tinjauan biasanya memakan waktu 5 hingga 10 menit. Tinjauan berjalan sebagai tugas latar belakang, sehingga Anda dapat terus bekerja di sesi Anda, memulai perintah lain, atau menutup terminal sepenuhnya. Jika Anda memilih untuk [memposting temuan ke permintaan tarik](#post-findings-to-the-pull-request), jaga sesi tetap terbuka hingga tinjauan selesai; jika sesi berakhir terlebih dahulu, Claude Code tidak memposting apa pun.

Gunakan `/tasks` untuk melihat tinjauan yang sedang berjalan dan selesai, buka tampilan detail untuk tinjauan, atau hentikan tinjauan yang sedang berlangsung. Jika Anda menghentikan tinjauan, Claude Code mengarsipkan sesi cloud dan tidak mengembalikan temuan parsial.

Claude juga dapat memberi tahu Anda bahwa tinjauan dihentikan atau bahwa sesinya tidak ditemukan:

* Jika sesi cloud tinjauan dihentikan atau [diarsipkan](/docs/id/claude-code-on-the-web#archive-sessions) di claude.ai sebelum tinjauan selesai, Claude memberi tahu Anda bahwa tinjauan dihentikan.
* Jika sesi cloud tinjauan dihapus, atau Anda telah masuk ke akun Claude atau organisasi yang berbeda sejak meluncurkannya, Claude memberi tahu Anda bahwa sesi tidak ditemukan.
* Jika Anda beralih akun, tinjauan mungkin masih selesai di bawah akun yang memulainya. Jika tinjauan masih berjalan, masuk kembali sebagai akun tersebut dan lanjutkan percakapan dengan `claude --resume` untuk melampirkannya kembali.

Ketika tinjauan selesai, Claude Code menampilkan temuan yang diverifikasi sebagai notifikasi di sesi Anda. Setiap temuan mencakup lokasi file dan penjelasan masalah sehingga Anda dapat meminta Claude untuk memperbaikinya secara langsung.

<h2 id="run-ultrareview-non-interactively">
  Jalankan ultrareview secara non-interaktif
</h2>

Gunakan subperintah `claude ultrareview` untuk memulai ultrareview dari CI atau skrip tanpa sesi interaktif. Subperintah meluncurkan tinjauan yang sama seperti `/code-review ultra`, memblokir hingga tinjauan jarak jauh selesai, dan mencetak temuan ke stdout.

```bash theme={null}
claude ultrareview
claude ultrareview 1234
claude ultrareview origin/main
```

Tanpa argumen, subperintah meninjau perbedaan antara cabang saat ini Anda dan cabang default, dengan [fallback seluruh repositori](#diff-limits-and-fallbacks) yang sama seperti `/code-review ultra` ketika tidak ada merge base. Teruskan nomor PR untuk meninjau pull request, atau teruskan cabang dasar untuk meninjau terhadapnya; [penanganan cabang dasar](#review-against-a-different-base) cocok dengan perintah interaktif.

Anda menyetujui fallback seluruh repositori dan prompt penagihan serta syarat ketika Anda menjalankan subperintah, sehingga jalankan dimulai tanpa menunggu input. Menjalankannya sendiri adalah apa yang dihitung sebagai persetujuan. Ketika Claude menjalankan subperintah untuk Anda sebagai gantinya, misalnya melalui alat Bash, Claude Code menolak tinjauan seluruh repositori.

Pada Claude Code v2.1.218 atau lebih baru, Anda juga dapat memulai tinjauan cloud dengan menjalankan `/code-review ultra` dalam sesi non-interaktif, misalnya `claude -p '/code-review ultra'`. Claude Code meluncurkan tinjauan dan mencetak tautan pelacakan tanpa menunggu temuan, tidak seperti `claude ultrareview`, yang memblokir hingga temuan tiba. Ketika tinjauan akan menagih kredit penggunaan, Claude Code berhenti sebelum meluncurkan dan mengarahkan Anda ke `claude ultrareview`, karena konfirmasi penagihan memerlukan sesi interaktif. Sebelum v2.1.218, `/code-review ultra` dalam sesi non-interaktif menjalankan tinjauan lokal.

Pesan kemajuan dan URL sesi langsung pergi ke stderr sehingga stdout tetap dapat diurai. Gunakan bendera ini untuk mengontrol output, timeout, dan apakah akan memposting temuan:

| Bendera               | Deskripsi                                                                                                                                                                                                                                                                                                      |
| --------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `--json`              | Cetak payload `bugs.json` mentah daripada temuan yang diformat                                                                                                                                                                                                                                                 |
| `--timeout <minutes>` | Menit maksimal untuk menunggu tinjauan selesai. Default ke 45                                                                                                                                                                                                                                                  |
| `--post`              | [Posting temuan yang selesai](#post-findings-to-the-pull-request) ke pull request sebagai satu komentar biasa dari akun GitHub Anda. Bekerja pada target pull request `github.com`; pada target lain, Claude Code mengabaikan bendera dan mengatakan demikian. Memerlukan Claude Code v2.1.227 atau lebih baru |
| `--no-post`           | Jangan posting temuan. Ini adalah default, dan jika Anda melewatkan kedua bendera, Claude Code tidak memposting. Memerlukan Claude Code v2.1.227 atau lebih baru                                                                                                                                               |

Menjalankan `claude ultrareview` memerlukan autentikasi yang sama dan konfigurasi penggunaan kredit seperti `/code-review ultra`.

Subperintah keluar dengan salah satu dari tiga kode:

* **0**: tinjauan selesai, dengan atau tanpa temuan
* **1**: tinjauan gagal diluncurkan atau dihentikan sebelum selesai, sesi cloud mengalami kesalahan, atau timeout berlalu
* **130**: Anda mengganggu subperintah dengan Ctrl-C

Jika Anda mengganggu subperintah, tinjauan jarak jauh terus berjalan; ikuti URL sesi yang dicetak ke stderr untuk menontonnya di browser.

Dengan `--post`, subperintah memulai posting tepat setelah mencetak temuan, dan mencetak tautan ke stderr.

* Jika jalankan gagal, dihentikan, atau timeout, atau jika Anda mengganggu, subperintah tidak memposting apa pun.
* Jika tinjauan selesai tetapi komentar tidak diposting, Claude Code mencetak alasan ke stderr, dan temuan tetap di stdout sehingga Anda dapat mempostingnya dengan tangan.

Untuk tinjauan otomatis pada pull request GitHub, [Code Review](/docs/id/code-review) terintegrasi dengan repositori Anda secara langsung dan memposting temuan sebagai komentar PR inline tanpa langkah CLI.

<h2 id="how-ultrareview-compares-to-/code-review">
  Bagaimana ultrareview dibandingkan dengan /code-review
</h2>

Kedua tinjauan menguji kode, tetapi Anda menggunakannya pada tahap alur kerja yang berbeda.

|               | `/code-review`                                        | `/code-review ultra`                                                                  |
| ------------- | ----------------------------------------------------- | ------------------------------------------------------------------------------------- |
| Target        | diff kerja Anda, permintaan tarik, cabang, atau jalur | diff kerja Anda atau permintaan tarik                                                 |
| Berjalan      | secara lokal di sesi Anda                             | di sandbox cloud                                                                      |
| Kedalaman     | skala dengan argumen effort                           | armada multi-agen dengan verifikasi independen                                        |
| Durasi        | detik hingga beberapa menit                           | kira-kira 5 hingga 10 menit                                                           |
| Biaya         | dihitung terhadap penggunaan normal                   | run gratis, kemudian kira-kira \$5 hingga \$25 per tinjauan sebagai kredit penggunaan |
| Terbaik untuk | umpan balik cepat saat iterasi                        | kepercayaan pra-merge pada perubahan substansial                                      |

Gunakan `/code-review` untuk umpan balik cepat saat Anda bekerja, atau berikan nomor PR untuk meninjau permintaan tarik rekan kerja sebelum menyetujuinya. Gunakan `/code-review ultra` sebelum menggabungkan perubahan substansial ketika Anda menginginkan lintasan yang lebih dalam yang menangkap masalah yang mungkin terlewatkan oleh tinjauan lokal.

<h2 id="related-resources">
  Sumber daya terkait
</h2>

* [Claude Code di web](/docs/id/claude-code-on-the-web): pelajari cara kerja sesi cloud dan sandbox cloud
* [Kelola biaya secara efektif](/docs/id/costs): lacak penggunaan dan tetapkan batas pengeluaran
