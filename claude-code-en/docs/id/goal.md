> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Jaga Claude tetap bekerja menuju tujuan

> Tetapkan kondisi penyelesaian dengan /goal dan Claude terus bekerja lintas giliran hingga kondisi terpenuhi.

<Note>
  `/goal` memerlukan Claude Code v2.1.139 atau lebih baru.
</Note>

Perintah `/goal` menetapkan kondisi penyelesaian dan Claude terus bekerja menuju tujuan tersebut tanpa Anda meminta setiap langkah. Setelah setiap giliran, model cepat kecil memeriksa apakah kondisi terpenuhi. Jika tidak, Claude memulai giliran lain alih-alih mengembalikan kontrol kepada Anda. Tujuan dihapus secara otomatis setelah kondisi terpenuhi.

Gunakan tujuan untuk pekerjaan substansial dengan keadaan akhir yang dapat diverifikasi:

* Migrasi modul ke API baru hingga setiap situs panggilan dikompilasi dan tes lulus
* Implementasi dokumen desain hingga semua kriteria penerimaan terpenuhi
* Pemisahan file besar menjadi modul yang terfokus hingga masing-masing berada di bawah anggaran ukuran
* Menyelesaikan antrian masalah berlabel hingga antrean kosong

<h2 id="compare-ways-to-keep-a-session-running">
  Bandingkan cara untuk menjaga sesi tetap berjalan
</h2>

Tiga pendekatan menjaga sesi saat ini berjalan di antara prompt. Pilih berdasarkan apa yang harus memulai giliran berikutnya:

| Pendekatan                                                          | Giliran berikutnya dimulai ketika | Berhenti ketika                                                |
| :------------------------------------------------------------------ | :-------------------------------- | :------------------------------------------------------------- |
| `/goal`                                                             | Giliran sebelumnya selesai        | Model mengkonfirmasi kondisi terpenuhi                         |
| [`/loop`](/docs/id/scheduled-tasks#run-a-prompt-repeatedly-with-%2Floop) | Interval waktu berlalu            | Anda menghentikannya, atau Claude memutuskan pekerjaan selesai |
| [Stop hook](/docs/id/hooks-guide#prompt-based-hooks)                     | Giliran sebelumnya selesai        | Skrip atau prompt Anda sendiri memutuskan                      |

`/goal` dan Stop hook keduanya diaktifkan setelah setiap giliran. `/goal` adalah pintasan berskop sesi: Anda mengetik kondisi dan itu aktif hanya untuk sesi saat ini. Stop hook berada di file pengaturan Anda, berlaku untuk setiap sesi dalam cakupannya, dan dapat menjalankan skrip untuk pemeriksaan deterministik atau prompt untuk yang dievaluasi model.

[Auto mode](/docs/id/auto-mode-config) dengan sendirinya menyetujui panggilan alat dalam satu giliran tetapi tidak memulai yang baru. Claude berhenti ketika menilai pekerjaan selesai. `/goal` menambahkan evaluator terpisah yang memeriksa kondisi Anda setelah setiap giliran, sehingga penyelesaian diputuskan oleh model segar daripada yang melakukan pekerjaan. Keduanya saling melengkapi: auto mode menghilangkan prompt per-alat, dan `/goal` menghilangkan prompt per-giliran.

<Tip>
  Pendekatan di atas menjaga sesi saat ini berjalan. Anda juga dapat menjadwalkan pekerjaan yang berjalan independen dari sesi terbuka apa pun, seperti tes malam atau triase pagi. Lihat [opsi penjadwalan](/docs/id/scheduled-tasks#compare-scheduling-options) untuk rutinitas cloud dan tugas terjadwal desktop.
</Tip>

<h2 id="use-/goal">
  Gunakan `/goal`
</h2>

Satu tujuan dapat aktif per sesi. Perintah yang sama menetapkan, memeriksa, dan menghapusnya tergantung pada argumennya.

<h3 id="set-a-goal">
  Tetapkan tujuan
</h3>

Jalankan `/goal` diikuti dengan kondisi yang ingin Anda penuhi. Jika tujuan sudah aktif, yang baru menggantikannya.

```text theme={null}
/goal all tests in test/auth pass and the lint step is clean
```

Menetapkan tujuan memulai giliran segera, dengan kondisi itu sendiri sebagai arahan. Anda tidak perlu mengirim prompt terpisah. Saat tujuan aktif, indikator `◎ /goal active` menunjukkan berapa lama tujuan telah berjalan.

Sebuah tujuan tidak mengubah izin. Dalam mode izin default, Claude masih meminta sebelum panggilan alat yang pengaturan Anda tidak sudah izinkan, seperti perintah tes di atas. Untuk membiarkan giliran tujuan berjalan tanpa pengawasan, pasangkan `/goal` dengan [mode otomatis](/docs/id/auto-mode-config).

Setelah setiap giliran, evaluator mengembalikan alasan singkat yang menjelaskan mengapa kondisi terpenuhi atau tidak. Alasan paling baru muncul di tampilan status dan dalam transkrip sehingga Anda dapat melihat apa yang Claude kerjakan selanjutnya.

<Note>
  Tujuan terus berjalan hingga kondisi terpenuhi atau Anda menjalankan `/goal clear`. Jalankan `/goal` tanpa argumen untuk melihat giliran dan token yang dihabiskan sejauh ini.
</Note>

<h3 id="write-an-effective-condition">
  Tulis kondisi yang efektif
</h3>

[Evaluator](#how-evaluation-works) menilai kondisi Anda terhadap apa yang telah Claude tampilkan dalam percakapan. Ini tidak menjalankan perintah atau membaca file secara independen, jadi tulis kondisi sebagai sesuatu yang output Claude sendiri dapat demonstrasikan. "Semua tes dalam `test/auth` lulus" berfungsi karena Claude menjalankan tes dan hasilnya mendarat dalam transkrip untuk evaluator dibaca.

Kondisi yang bertahan di banyak giliran biasanya memiliki:

* **Satu keadaan akhir yang terukur**: hasil tes, kode keluar build, jumlah file, antrian kosong
* **Pemeriksaan yang dinyatakan**: bagaimana Claude harus membuktikannya, seperti "`npm test` exits 0" atau "`git status` is clean"
* **Batasan yang penting**: apa pun yang tidak boleh berubah dalam perjalanan ke sana, seperti "tidak ada file tes lain yang dimodifikasi"

Kondisi dapat mencapai 4.000 karakter.

Untuk membatasi berapa lama tujuan berjalan, sertakan klausa giliran atau waktu dalam kondisi, seperti `or stop after 20 turns`. Claude melaporkan kemajuan terhadap klausa itu setiap giliran dan evaluator menilainya dari percakapan.

<h3 id="check-status">
  Periksa status
</h3>

Jalankan `/goal` tanpa argumen untuk melihat keadaan saat ini.

```text theme={null}
/goal
```

Jika tujuan aktif, status menunjukkan:

* Kondisinya
* Berapa lama itu telah berjalan
* Berapa banyak giliran yang telah dievaluasi
* Pengeluaran token saat ini
* Alasan paling baru evaluator

Jumlah giliran dan alasan paling baru muncul setelah evaluasi pertama telah berjalan.

Jika tidak ada tujuan aktif tetapi satu dicapai sebelumnya dalam sesi, status menunjukkan kondisi yang dicapai bersama dengan durasi, jumlah giliran, dan pengeluaran tokennya.

<h3 id="clear-a-goal">
  Hapus tujuan
</h3>

Jalankan `/goal clear` untuk menghapus tujuan aktif sebelum kondisinya terpenuhi.

```text theme={null}
/goal clear
```

Claude mencetak `Goal cleared:` diikuti dengan kondisi untuk mengonfirmasi, atau `No goal set` jika tidak ada yang aktif.

`stop`, `off`, `reset`, `none`, dan `cancel` diterima sebagai alias untuk `clear`. Menjalankan `/clear` untuk memulai percakapan baru juga menghapus tujuan aktif apa pun.

<h3 id="resume-with-an-active-goal">
  Lanjutkan dengan tujuan aktif
</h3>

Tujuan yang masih aktif ketika sesi berakhir dipulihkan ketika Anda melanjutkan sesi itu dengan `--resume` atau `--continue`. Kondisi terbawa, tetapi jumlah giliran, timer, dan baseline pengeluaran token semuanya direset saat dilanjutkan. Tujuan yang sudah dicapai atau dihapus tidak dipulihkan.

<h3 id="run-non-interactively">
  Jalankan non-interaktif
</h3>

`/goal` bekerja dalam [mode non-interaktif](/docs/id/headless), di aplikasi desktop (/id/desktop), dan melalui [Remote Control](/docs/id/remote-control). Menetapkan tujuan dengan `-p` menjalankan loop hingga selesai dalam satu invokasi:

```bash theme={null}
claude -p "/goal CHANGELOG.md has an entry for every PR merged this week"
```

Dengan output teks default, tidak ada yang dicetak sampai kondisi terpenuhi, jadi tujuan yang berjalan banyak giliran dapat terlihat macet. Tambahkan `--output-format stream-json --verbose` untuk memancarkan setiap pesan saat loop berjalan.

Hentikan proses dengan Ctrl+C untuk menghentikan tujuan non-interaktif sebelum kondisi terpenuhi.

<h2 id="how-evaluation-works">
  Cara evaluasi bekerja
</h2>

`/goal` adalah pembungkus di sekitar [Stop hook berbasis prompt](/docs/id/hooks#prompt-based-hooks) berskop sesi. Setiap kali Claude menyelesaikan giliran, kondisi dan percakapan sejauh ini dikirim ke [model cepat kecil](/docs/id/model-config) yang dikonfigurasi, yang secara default adalah Haiku. Model mengembalikan keputusan ya-atau-tidak dan alasan singkat. "Tidak" memberi tahu Claude untuk terus bekerja dan menyertakan alasan sebagai panduan untuk giliran berikutnya. "Ya" menghapus tujuan dan mencatat entri yang dicapai dalam transkrip.

Evaluator berjalan di mana pun penyedia sesi Anda dikonfigurasi. Ini tidak memanggil alat, jadi hanya dapat menilai apa yang telah Claude tampilkan dalam percakapan.

<Note>
  Token evaluasi ditagih pada model cepat kecil yang dikonfigurasi untuk penyedia Anda dan biasanya dapat diabaikan dibandingkan dengan pengeluaran giliran utama.
</Note>

<h2 id="requirements">
  Persyaratan
</h2>

`/goal` hanya berjalan di ruang kerja tempat Anda telah menerima dialog kepercayaan, karena evaluator adalah bagian dari sistem hooks. `/goal` juga tidak tersedia ketika [`disableAllHooks`](/docs/id/hooks#disable-or-remove-hooks) diatur di tingkat pengaturan apa pun atau ketika [`allowManagedHooksOnly`](/docs/id/settings#hook-configuration) diatur dalam pengaturan terkelola. Dalam setiap kasus, perintah memberi tahu Anda mengapa alih-alih diam-diam tidak melakukan apa pun.

<h2 id="see-also">
  Lihat juga
</h2>

* [Jalankan prompt berulang kali dengan `/loop`](/docs/id/scheduled-tasks#run-a-prompt-repeatedly-with-%2Floop): jalankan ulang pada interval waktu alih-alih hingga kondisi terpenuhi
* [Prompt-based hooks](/docs/id/hooks-guide#prompt-based-hooks): tulis Stop hook Anda sendiri ketika Anda memerlukan logika evaluasi kustom
* [Auto mode](/docs/id/auto-mode-config): setujui panggilan alat secara otomatis sehingga setiap giliran tujuan berjalan tanpa dihadiri
* [Perbandingan penjadwalan](/docs/id/scheduled-tasks#compare-scheduling-options): jalankan pekerjaan sesuai jadwal independen dari sesi terbuka apa pun
