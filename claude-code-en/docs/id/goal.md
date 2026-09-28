> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Jaga Claude tetap bekerja menuju tujuan

> Tetapkan kondisi penyelesaian dengan /goal dan Claude terus bekerja hingga kondisi terpenuhi, model menilai tidak mungkin, atau kesalahan yang harus Anda perbaiki menghapus tujuan.

Perintah `/goal` menetapkan kondisi penyelesaian dan Claude terus bekerja menuju tujuan tersebut tanpa Anda meminta setiap langkah. Setelah setiap giliran, model cepat kecil memeriksa apakah kondisi terpenuhi. Jika model menilai belum terpenuhi, Claude memulai giliran lain alih-alih mengembalikan kontrol kepada Anda. Tujuan dihapus secara otomatis setelah kondisi terpenuhi, jika model menilai kondisi tidak mungkin dipenuhi, atau jika giliran gagal pada [kesalahan yang harus Anda perbaiki](#errors-you-have-to-fix-clear-the-goal).

Gunakan tujuan untuk pekerjaan substansial dengan keadaan akhir yang dapat diverifikasi:

* Migrasi modul ke API baru hingga setiap situs panggilan dikompilasi dan tes lulus
* Implementasi dokumen desain hingga semua kriteria penerimaan terpenuhi
* Pemisahan file besar menjadi modul yang terfokus hingga masing-masing berada di bawah anggaran ukuran
* Menyelesaikan antrian masalah berlabel hingga antrean kosong

<h2 id="compare-ways-to-keep-a-session-running">
  Bandingkan cara untuk menjaga sesi tetap berjalan
</h2>

Tiga pendekatan menjaga sesi saat ini berjalan di antara prompt. Pilih berdasarkan apa yang harus memulai giliran berikutnya:

| Pendekatan                                                          | Giliran berikutnya dimulai ketika                                                                                                                                                                  | Berhenti ketika                                                                                                                                                                                                                  |
| :------------------------------------------------------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `/goal`                                                             | Giliran sebelumnya selesai, atau, dalam sesi interaktif, [pemeriksaan idle](#background-work-defers-evaluation) atau [percobaan ulang otomatis](#other-errors-retry-or-pause-the-goal) akan datang | Model mengkonfirmasi kondisi terpenuhi atau menilainya tidak mungkin, atau giliran gagal pada [kesalahan yang harus Anda perbaiki](#errors-you-have-to-fix-clear-the-goal), atau Anda menjalankan [`/goal clear`](#clear-a-goal) |
| [`/loop`](/docs/id/scheduled-tasks#run-a-prompt-repeatedly-with-%2Floop) | Interval waktu berlalu                                                                                                                                                                             | Anda menghentikannya, atau Claude memutuskan pekerjaan selesai                                                                                                                                                                   |
| [Stop hook](/docs/id/hooks-guide#prompt-based-hooks)                     | Giliran sebelumnya selesai                                                                                                                                                                         | Skrip atau prompt Anda sendiri memutuskan                                                                                                                                                                                        |

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

Sebuah tujuan tidak mengubah mode izin Anda. Untuk membiarkan giliran tujuan berjalan tanpa pengawasan, jalankan `/goal` dalam [mode otomatis](/docs/id/auto-mode-config). Dalam [mode Manual](/docs/id/permission-modes), Claude masih meminta sebelum panggilan alat yang pengaturan Anda tidak sudah izinkan, seperti perintah tes di atas.

Saat tujuan aktif, transkrip menunjukkan setiap putusan yang dikembalikan evaluator, dan Anda dapat menekan Ctrl+O untuk melihat alasan di baliknya. Tampilan status juga menunjukkan alasan paling baru, sehingga Anda dapat melihat apa yang Claude kerjakan selanjutnya.

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

Ketika Anda melanjutkan sesi, Claude Code memulihkan tujuan yang masih aktif ketika sesi berakhir. Claude Code memulihkannya di setiap rute lanjutan: `--continue`, `--resume` dengan ID atau nama sesi, atau [jalur file transkrip](/docs/id/sessions#resume-a-session), dan [pemilih sesi](/docs/id/sessions#use-the-session-picker). Sebelum v2.1.239, Claude Code memulihkan tujuan di setiap rute kecuali pemilih `claude --resume`.

Claude Code membawa kondisi tetapi mengatur ulang jumlah giliran, timer, dan baseline pengeluaran token. Ini tidak memulihkan tujuan yang sudah dicapai atau dihapus.

<h3 id="run-non-interactively">
  Jalankan non-interaktif
</h3>

`/goal` bekerja dalam [mode non-interaktif](/docs/id/headless), di [aplikasi desktop](/docs/id/desktop), dan melalui [Remote Control](/docs/id/remote-control). Menetapkan tujuan dengan `-p` menjalankan loop hingga selesai dalam satu invokasi:

```bash theme={null}
claude -p "/goal CHANGELOG.md has an entry for every PR merged this week"
```

Dengan output teks default, tidak ada yang dicetak sampai jalannya berakhir, jadi tujuan yang berjalan banyak giliran dapat terlihat macet. Tambahkan `--output-format stream-json --verbose` untuk memancarkan setiap pesan saat loop berjalan.

Hentikan proses dengan Ctrl+C untuk menghentikan tujuan non-interaktif sebelum kondisinya terpenuhi.

<h2 id="how-evaluation-works">
  Cara evaluasi bekerja
</h2>

`/goal` adalah pembungkus di sekitar [Stop hook berbasis prompt](/docs/id/hooks#prompt-based-hooks) berskop sesi. Setiap kali Claude menyelesaikan giliran, Claude Code mengirimkan kondisi dan percakapan sejauh ini ke [model cepat kecil](/docs/id/model-config) yang dikonfigurasi, yang secara default adalah Haiku pada Claude API; pada penyedia pihak ketiga, periksa [halaman penyedia Anda](/docs/id/third-party-integrations) untuk default platform. Model mengembalikan salah satu dari tiga keputusan, masing-masing dengan alasan singkat:

* **Belum terpenuhi**: Claude terus bekerja dan mengambil alasan sebagai panduan untuk giliran berikutnya.
* **Terpenuhi**: Claude Code menghapus tujuan dan mencatat entri yang dicapai dalam transkrip.
* **Tidak mungkin**: evaluator menilai bahwa kondisi tidak dapat pernah dipenuhi. Claude Code menghapus tujuan dan mencatat entri yang gagal dalam transkrip bersama dengan alasannya. Anda tidak perlu menghapusnya sendiri.

Jika Claude terus menjawab evaluator tanpa membuat kemajuan (tidak ada penggunaan alat selama beberapa giliran berturut-turut), Claude Code menghentikan loop, mencetak peringatan, dan mengembalikan kontrol kepada Anda dengan tujuan masih ditetapkan. Evaluasi dilanjutkan setelah prompt Anda berikutnya. [Panduan hooks](/docs/id/hooks-guide#stop-hook-hits-the-block-cap) menjelaskan mekanisme yang mendasarinya.

<h3 id="when-a-turn-fails">
  Ketika giliran gagal
</h3>

Ketika giliran gagal, Claude Code menghapus tujuan jika kesalahan adalah salah satu yang harus Anda perbaiki. Setelah kesalahan lainnya, tujuan tetap ditetapkan.

<h4 id="errors-you-have-to-fix-clear-the-goal">
  Kesalahan yang harus Anda perbaiki menghapus tujuan
</h4>

Jika giliran gagal pada kesalahan yang tidak akan dihapus sampai Anda memperbaikinya, Claude Code menghapus tujuan dan mencetak peringatan yang menyebutkan penyebabnya. Peringatan dimulai dengan `Goal cleared after an unrecoverable error` dan diakhiri dengan `Run /goal again to continue`. Perbaiki penyebabnya, kemudian [tetapkan tujuan lagi](#set-a-goal) dengan `/goal <condition>`. Empat jenis kegagalan menghapus tujuan:

* Kegagalan autentikasi, ketika Claude Code mengelola kredensialnya sendiri. Ketika host mengelolanya untuk Anda, seperti aplikasi desktop, ekstensi VS Code, atau [sesi cloud](/docs/id/claude-code-on-the-web), Claude Code membiarkan tujuan tetap aktif karena host memulihkan akses dengan sendirinya.
* Saldo kredit yang habis
* Overflow konteks yang [auto-compaction](/docs/id/model-config#set-the-auto-compact-window) tidak dapat menghapus
* Model yang tidak tersedia

<h4 id="other-errors-retry-or-pause-the-goal">
  Kesalahan lainnya mencoba ulang atau menjeda tujuan
</h4>

Setelah kegagalan lainnya, tujuan tetap ditetapkan. Dalam sesi interaktif pada Claude Code v2.1.269 atau lebih baru, Claude Code juga mencetak baris yang menyebutkan penyebabnya dan baik mencoba ulang dengan sendirinya atau menunggu Anda:

* **Coba ulang**: setelah kegagalan yang cenderung menghapus dengan sendirinya, seperti server yang kelebihan beban atau koneksi yang terputus, pemberitahuan yang dimulai dengan `Goal still active` menunjukkan waktu tunggu sebelum percobaan berikutnya. Setelah tiga percobaan ulang otomatis, tujuan dijeda sebagai gantinya.
* **Jeda**: setelah kegagalan yang percobaan ulang hanya akan mengulangi, seperti batas laju API, [batas penggunaan](/docs/id/errors#youve-hit-your-session-limit) claude.ai, atau hook yang mengakhiri giliran, pemberitahuan yang dimulai dengan `Goal paused` menyebutkan penyebabnya. Jika sesi [menunggu untuk melanjutkan secara otomatis ketika batas penggunaan direset](/docs/id/interactive-mode#wait-for-a-usage-limit-to-reset), Claude melanjutkan pekerjaan menuju tujuan kemudian.

Kirimkan pesan kapan saja untuk memulai giliran berikutnya segera. Untuk mematikan percobaan ulang otomatis, atur [`CLAUDE_CODE_GOAL_CHECKIN_MINUTES`](/docs/id/env-vars) ke `0`, yang juga mematikan [check-in](#background-work-defers-evaluation).

<h3 id="background-work-defers-evaluation">
  Pekerjaan latar belakang menunda evaluasi
</h3>

Jika subagent atau perintah shell latar belakang masih berjalan ketika giliran berakhir, Claude Code melewati evaluasi untuk giliran itu. Ini mengevaluasi pada akhir giliran berikutnya yang selesai tanpa pekerjaan latar belakang yang berjalan. Ketika pekerjaan latar belakang selesai, Claude Code mengirimkan hasilnya kepada Claude sebagai giliran baru, jadi Anda tidak perlu memberi prompt.

Setelah pekerjaan latar belakang membuat tujuan menunggu selama 30 menit, check-in sudah jatuh tempo. Dalam check-in, Claude Code mencantumkan tugas yang sedang berjalan dan meminta Claude untuk membaca output mereka, terus menunggu jika mereka membuat kemajuan, dan memperbaiki atau menghentikan yang macet. Setelah check-in pertama, Claude Code menunggu dua kali lebih lama sebelum setiap check-in kemudian, hingga empat kali interval pertama: dengan default, 1 jam setelah check-in pertama, kemudian setiap 2 jam. Claude Code mengirimkan check-in yang jatuh tempo, yang pertama disertakan, dalam salah satu dari dua cara:

* **Ketika giliran berakhir**: Claude Code mengirimkan check-in pada akhir giliran berikutnya yang selesai dengan pekerjaan masih berjalan. Dalam sesi non-interaktif, seperti yang dimulai dengan `-p`, ini adalah satu-satunya cara Claude Code mengirimkan check-in.
* **Saat sesi menganggur**: dalam sesi interaktif, Claude Code juga memulai giliran dengan sendirinya untuk mengirimkan check-in alih-alih menunggu prompt Anda berikutnya. Jika pekerjaan latar belakang telah berhenti tanpa melaporkan hasil, Claude Code meminta Claude untuk melanjutkan menuju tujuan. Claude Code memulai paling banyak tiga check-in menganggur per tujuan di antara prompt Anda. Dalam check-in menganggur ketiga, Claude Code mengatakan bahwa check-in menganggur dijeda sampai Anda mengirimkan prompt lain. Sebelum v2.1.246, check-in menganggur tidak terbatas. Check-in menganggur memerlukan Claude Code v2.1.236 atau lebih baru.

Sebelum v2.1.239, hanya check-in menganggur yang mundur dengan cara ini; check-in yang disampaikan pada akhir giliran berulang pada interval pertama.

Untuk mengubah interval pertama, atur [`CLAUDE_CODE_GOAL_CHECKIN_MINUTES`](/docs/id/env-vars). Claude Code menggunakan nilai Anda sebagai pengganti interval 30 menit dan menskalakan interval kemudian dengannya. Atur ke `0` untuk mematikan check-in dan [percobaan ulang otomatis](#other-errors-retry-or-pause-the-goal).

Check-in memerlukan Claude Code v2.1.234 atau lebih baru.

<h3 id="evaluation-model-and-cost">
  Model evaluasi dan biaya
</h3>

Untuk mengevaluasi pada model yang berbeda, atur [`ANTHROPIC_DEFAULT_HAIKU_MODEL`](/docs/id/model-config#environment-variables).

<Warning>
  Claude Code membaca `ANTHROPIC_DEFAULT_HAIKU_MODEL` di mana pun ia menggunakan model cepat kecil, bukan hanya untuk evaluasi `/goal`. Ketika Anda menetapkannya, Claude Code juga menyelesaikan [alias `haiku`](/docs/id/model-config#model-aliases) ke model itu dan menjalankan [fungsionalitas latar belakang](/docs/id/costs#background-token-usage), seperti peringkasan percakapan, di atasnya.
</Warning>

Evaluator berjalan di mana pun penyedia sesi Anda dikonfigurasi. Ini tidak memanggil alat, jadi hanya dapat menilai apa yang telah Claude tampilkan dalam percakapan.

<Note>
  Token evaluasi ditagih pada model cepat kecil yang dikonfigurasi untuk penyedia Anda dan biasanya dapat diabaikan dibandingkan dengan pengeluaran giliran utama.
</Note>

<h2 id="requirements">
  Persyaratan
</h2>

Claude Code membuat `/goal` tersedia di bawah [aturan kepercayaan ruang kerja yang sama dengan hooks dalam file pengaturan](/docs/id/permissions#what-runs-before-you-trust-a-folder), karena evaluator adalah bagian dari sistem hooks. `/goal` juga tidak tersedia ketika [`disableAllHooks`](/docs/id/hooks#disable-or-remove-hooks) adalah `true` setelah penerapan prioritas pengaturan, atau ketika [`allowManagedHooksOnly`](/docs/id/settings-reference#allowmanagedhooksonly) diatur dalam pengaturan terkelola. Dalam setiap kasus, perintah memberi tahu Anda mengapa alih-alih diam-diam tidak melakukan apa pun.

<h2 id="see-also">
  Lihat juga
</h2>

* [Jalankan prompt berulang kali dengan `/loop`](/docs/id/scheduled-tasks#run-a-prompt-repeatedly-with-%2Floop): jalankan ulang pada interval waktu alih-alih hingga kondisi terpenuhi
* [Prompt-based hooks](/docs/id/hooks-guide#prompt-based-hooks): tulis Stop hook Anda sendiri ketika Anda memerlukan logika evaluasi kustom
* [Auto mode](/docs/id/auto-mode-config): setujui panggilan alat secara otomatis sehingga setiap giliran tujuan berjalan tanpa dihadiri
* [Perbandingan penjadwalan](/docs/id/scheduled-tasks#compare-scheduling-options): jalankan pekerjaan sesuai jadwal independen dari sesi terbuka apa pun
