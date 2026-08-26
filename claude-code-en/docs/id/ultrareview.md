> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Temukan bug dengan ultrareview

> Jalankan tinjauan kode multi-agen yang mendalam di cloud dengan /code-review ultra untuk menemukan dan memverifikasi bug sebelum Anda merge.

<Note>
  Ultrareview adalah fitur pratinjau penelitian. Fitur, harga, dan ketersediaan dapat berubah berdasarkan umpan balik. Perintah sekarang dipanggil sebagai `/code-review ultra`, dan `/ultrareview` tetap sebagai alias.
</Note>

Ultrareview adalah tinjauan kode yang mendalam yang berjalan di Claude Code pada infrastruktur web. Ketika Anda menjalankan `/code-review ultra`, Claude Code meluncurkan armada agen peninjau dalam sandbox jarak jauh untuk menemukan bug di cabang atau pull request Anda.

Dibandingkan dengan `/code-review` lokal atau `/review`, ultrareview menawarkan:

* **Sinyal yang lebih tinggi**: setiap temuan yang dilaporkan secara independen direproduksi dan diverifikasi, sehingga hasil fokus pada bug nyata daripada saran gaya
* **Cakupan yang lebih luas**: armada agen peninjau yang lebih besar menjelajahi perubahan secara paralel, yang mengungkap masalah yang mungkin terlewatkan oleh tinjauan lokal
* **Tidak ada penggunaan sumber daya lokal**: tinjauan berjalan sepenuhnya dalam sandbox jarak jauh, sehingga terminal Anda tetap bebas untuk pekerjaan lain saat berjalan

Ultrareview memerlukan autentikasi dengan akun Claude.ai karena berjalan di Claude Code pada infrastruktur web. Jika Anda masuk hanya dengan kunci API, jalankan `/login` dan autentikasi dengan Claude.ai terlebih dahulu. Ultrareview tidak tersedia saat menggunakan Claude Code dengan Amazon Bedrock, Google Cloud's Agent Platform, atau Microsoft Foundry, dan tidak tersedia untuk organisasi yang telah mengaktifkan Zero Data Retention.

<h2 id="run-ultrareview-from-the-cli">
  Jalankan ultrareview dari CLI
</h2>

Mulai tinjauan dari repositori git apa pun di Claude Code CLI.

```text theme={null}
/code-review ultra
```

Tanpa argumen, ultrareview meninjau perbedaan antara cabang saat ini Anda dan cabang default, termasuk perubahan yang tidak berkomitmen dan staged di pohon kerja Anda. Claude Code membundel status repositori dan mengunggahnya ke sandbox jarak jauh untuk tinjauan.

Untuk meninjau pull request GitHub sebagai gantinya, teruskan nomor PR.

```text theme={null}
/code-review ultra 1234
```

Dalam mode PR, sandbox jarak jauh mengkloning pull request langsung dari host daripada membundel pohon kerja lokal Anda. Mode PR bekerja dengan repositori di `github.com` dan pada instans [GitHub Enterprise Server](/docs/id/github-enterprise-server) yang telah dihubungkan oleh Owner ke Claude Code.

<Tip>
  Jika repositori Anda terlalu besar untuk dibundel, Claude Code akan meminta Anda menggunakan mode PR sebagai gantinya. Dorong cabang Anda dan buka PR draft, kemudian jalankan `/code-review ultra <PR-number>`.

  Jika diff pull request terlalu besar, Claude Code menolak tinjauan dengan petunjuk scoping sebelum pekerjaan tinjauan apa pun berjalan.
</Tip>

Sebelum meluncurkan, Claude Code menampilkan dialog konfirmasi dengan cakupan tinjauan (termasuk jumlah file dan baris saat meninjau cabang), sisa run gratis Anda, dan perkiraan biaya. Setelah Anda mengonfirmasi, tinjauan berlanjut di latar belakang dan Anda dapat terus menggunakan sesi Anda. Perintah hanya berjalan ketika Anda memanggilnya dengan `/code-review ultra`; Claude tidak memulai ultrareview dengan sendirinya.

<h2 id="pricing-and-free-runs">
  Harga dan run gratis
</h2>

Ultrareview adalah fitur premium yang ditagih terhadap penggunaan ekstra daripada penggunaan yang disertakan dalam paket Anda.

| Paket               | Run gratis yang disertakan | Setelah run gratis                                                                                                     |
| ------------------- | -------------------------- | ---------------------------------------------------------------------------------------------------------------------- |
| Pro                 | 3 run gratis               | ditagih sebagai [penggunaan ekstra](https://support.claude.com/id/articles/12429409-extra-usage-for-paid-claude-plans) |
| Max                 | 3 run gratis               | ditagih sebagai [penggunaan ekstra](https://support.claude.com/id/articles/12429409-extra-usage-for-paid-claude-plans) |
| Team dan Enterprise | tidak ada                  | ditagih sebagai [penggunaan ekstra](https://support.claude.com/id/articles/12429409-extra-usage-for-paid-claude-plans) |

Pelanggan Pro dan Max menerima tiga run ultrareview gratis untuk mencoba fitur. Ketiga run ini adalah alokasi satu kali per akun dan tidak diperbarui. Setelah Anda menggunakan ketiga run tersebut, atau setelah periode run gratis berakhir, setiap tinjauan ditagih ke penggunaan ekstra dan biasanya biaya berkisar \$5 hingga \$20 tergantung pada ukuran perubahan. Satu run dihitung setelah sesi jarak jauh dimulai, jadi tinjauan yang Anda hentikan lebih awal atau yang gagal diselesaikan masih menggunakan satu run gratis. Untuk tinjauan berbayar, penggunaan ekstra hanya ditagih untuk bagian yang berjalan.

Karena ultrareview selalu ditagih sebagai penggunaan ekstra di luar run gratis, akun atau organisasi Anda harus memiliki penggunaan ekstra diaktifkan sebelum Anda dapat meluncurkan tinjauan berbayar. Jika penggunaan ekstra tidak diaktifkan, Claude Code memblokir peluncuran dan menautkan Anda ke pengaturan penagihan tempat Anda dapat mengaktifkannya. Anda juga dapat menjalankan `/usage-credits` untuk memeriksa atau mengubah pengaturan saat ini Anda.

<h2 id="track-a-running-review">
  Lacak tinjauan yang sedang berjalan
</h2>

Tinjauan biasanya memakan waktu 5 hingga 10 menit. Tinjauan berjalan sebagai tugas latar belakang, sehingga Anda dapat terus bekerja di sesi Anda, memulai perintah lain, atau menutup terminal sepenuhnya.

Gunakan `/tasks` untuk melihat tinjauan yang sedang berjalan dan selesai, buka tampilan detail untuk tinjauan, atau hentikan tinjauan yang sedang berlangsung. Menghentikan tinjauan mengarsipkan sesi cloud, dan temuan parsial tidak dikembalikan. Ketika tinjauan selesai, temuan yang diverifikasi muncul sebagai notifikasi di sesi Anda. Setiap temuan mencakup lokasi file dan penjelasan masalah sehingga Anda dapat meminta Claude untuk memperbaikinya secara langsung.

<h2 id="run-ultrareview-non-interactively">
  Jalankan ultrareview secara non-interaktif
</h2>

Gunakan subperintah `claude ultrareview` untuk memulai ultrareview dari CI atau skrip tanpa sesi interaktif. Subperintah meluncurkan tinjauan yang sama seperti `/code-review ultra`, memblokir hingga tinjauan jarak jauh selesai, mencetak temuan ke stdout, dan keluar dengan kode 0 pada kesuksesan atau 1 pada kegagalan.

```bash theme={null}
claude ultrareview
claude ultrareview 1234
claude ultrareview origin/main
```

Tanpa argumen, subperintah meninjau perbedaan antara cabang saat ini Anda dan cabang default. Teruskan nomor PR untuk meninjau pull request, atau teruskan cabang dasar untuk meninjau perbedaan terhadap cabang itu sebagai gantinya. Memanggil subperintah dihitung sebagai persetujuan untuk penagihan dan prompt syarat yang ditampilkan perintah interaktif.

Pesan kemajuan dan URL sesi langsung pergi ke stderr sehingga stdout tetap dapat diurai. Gunakan bendera ini untuk mengontrol output dan timeout:

| Bendera               | Deskripsi                                                      |
| --------------------- | -------------------------------------------------------------- |
| `--json`              | Cetak payload `bugs.json` mentah daripada temuan yang diformat |
| `--timeout <minutes>` | Menit maksimal untuk menunggu tinjauan selesai. Default ke 30  |

Menjalankan `claude ultrareview` memerlukan autentikasi yang sama dan konfigurasi penggunaan kredit seperti `/code-review ultra`. Subperintah keluar dengan kode 0 ketika tinjauan selesai dengan atau tanpa temuan, kode 1 ketika tinjauan gagal diluncurkan, sesi jarak jauh mengalami kesalahan, atau timeout berlalu, dan kode 130 ketika diinterupsi dengan Ctrl-C. Tinjauan jarak jauh terus berjalan jika Anda mengganggu subperintah; ikuti URL sesi yang dicetak ke stderr untuk menontonnya di browser.

Untuk tinjauan otomatis pada pull request GitHub, [Code Review](/docs/id/code-review) terintegrasi dengan repositori Anda secara langsung dan memposting temuan sebagai komentar PR inline tanpa langkah CLI.

<h2 id="how-ultrareview-compares-to-/code-review-and-/review">
  Bagaimana ultrareview dibandingkan dengan /code-review dan /review
</h2>

Ketiga perintah meninjau kode, tetapi menargetkan tahap alur kerja yang berbeda.

|               | `/code-review`                      | `/review <pr>`                             | `/code-review ultra`                                                                  |
| ------------- | ----------------------------------- | ------------------------------------------ | ------------------------------------------------------------------------------------- |
| Target        | diff kerja Anda                     | permintaan tarik GitHub                    | diff kerja Anda atau permintaan tarik                                                 |
| Berjalan      | secara lokal di sesi Anda           | secara lokal di sesi Anda                  | secara jarak jauh di sandbox cloud                                                    |
| Kedalaman     | skala dengan argumen effort         | tinjauan satu-lintasan pada effort sesi    | armada multi-agen dengan verifikasi independen                                        |
| Durasi        | detik hingga beberapa menit         | detik hingga beberapa menit                | kira-kira 5 hingga 10 menit                                                           |
| Biaya         | dihitung terhadap penggunaan normal | dihitung terhadap penggunaan normal        | run gratis, kemudian kira-kira \$5 hingga \$20 per tinjauan sebagai kredit penggunaan |
| Terbaik untuk | umpan balik cepat saat iterasi      | meninjau PR rekan kerja sebelum menyetujui | kepercayaan pra-merge pada perubahan substansial                                      |

Gunakan `/code-review` untuk umpan balik cepat saat Anda bekerja. Gunakan `/review <pr>` untuk melihat permintaan tarik dengan cara yang sama seperti sebelum menyetujuinya. Gunakan `/code-review ultra` sebelum merge perubahan substansial ketika Anda menginginkan lintasan yang lebih dalam yang menangkap masalah yang mungkin terlewatkan oleh tinjauan lokal.

<h2 id="related-resources">
  Sumber daya terkait
</h2>

* [Claude Code di web](/docs/id/claude-code-on-the-web): pelajari cara kerja sesi cloud dan sandbox cloud
* [Rencanakan perubahan kompleks dengan ultraplan](/docs/id/ultraplan): rekan perencanaan untuk ultrareview untuk pekerjaan desain di muka
* [Kelola biaya secara efektif](/docs/id/costs): lacak penggunaan dan tetapkan batas pengeluaran
