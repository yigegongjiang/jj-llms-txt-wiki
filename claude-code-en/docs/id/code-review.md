> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Code Review

> Siapkan ulasan PR otomatis yang menangkap kesalahan logika, kerentanan keamanan, dan regresi menggunakan analisis multi-agen dari seluruh basis kode Anda

<Note>
  Code Review sedang dalam pratinjau penelitian, tersedia untuk langganan [Team dan Enterprise](https://claude.ai/admin-settings/claude-code). Tidak tersedia untuk organisasi dengan [Zero Data Retention](/docs/id/zero-data-retention) yang diaktifkan.
</Note>

Code Review menganalisis permintaan tarik GitHub Anda dan memposting temuan sebagai komentar sebaris pada baris kode tempat ditemukannya masalah. Armada agen khusus memeriksa perubahan kode dalam konteks basis kode lengkap Anda, mencari kesalahan logika, kerentanan keamanan, kasus tepi yang rusak, dan regresi halus.

Temuan diberi tag berdasarkan tingkat keparahan dan tidak menyetujui atau memblokir PR Anda, sehingga alur kerja ulasan yang ada tetap utuh. Anda dapat menyesuaikan apa yang Claude tandai dengan menambahkan file `CLAUDE.md` atau `REVIEW.md` ke repositori Anda.

Untuk menjalankan Claude di infrastruktur CI Anda sendiri alih-alih layanan terkelola ini, lihat [GitHub Actions](/docs/id/github-actions) atau [GitLab CI/CD](/docs/id/gitlab-ci-cd). Untuk repositori pada instans GitHub yang di-host sendiri, lihat [GitHub Enterprise Server](/docs/id/github-enterprise-server).

Halaman ini mencakup:

* [Cara kerja ulasan](#how-reviews-work)
* [Penyiapan](#set-up-code-review)
* [Memicu ulasan secara manual](#manually-trigger-reviews) dengan `@claude review` dan `@claude review once`
* [Menyesuaikan ulasan](#customize-reviews) dengan `CLAUDE.md` dan `REVIEW.md`
* [Harga](#pricing)
* [Pemecahan masalah](#troubleshooting) jalankan yang gagal dan komentar yang hilang
* [Meninjau diff secara lokal](#review-a-diff-locally) dengan perintah `/code-review`

<Note>
  Untuk meninjau diff secara lokal di terminal Anda tanpa memasang GitHub App, jalankan perintah `/code-review` dalam sesi Claude Code apa pun. Lihat [Meninjau diff secara lokal](#review-a-diff-locally).
</Note>

<h2 id="how-reviews-work">
  Cara kerja ulasan
</h2>

Setelah Owner [mengaktifkan Code Review](#set-up-code-review) untuk organisasi Anda, ulasan dipicu ketika PR dibuka, pada setiap push, atau ketika diminta secara manual, tergantung pada perilaku yang dikonfigurasi repositori. Mengomentari `@claude review` [memulai ulasan pada PR](#manually-trigger-reviews) dalam mode apa pun.

Ketika ulasan berjalan, beberapa agen menganalisis diff dan kode sekitarnya secara paralel pada infrastruktur Anthropic. Setiap agen mencari kelas masalah yang berbeda, kemudian langkah verifikasi memeriksa kandidat terhadap perilaku kode aktual untuk menyaring positif palsu. Hasilnya dideduplikasi, diurutkan berdasarkan tingkat keparahan, dan diposting sebagai komentar sebaris pada baris spesifik tempat masalah ditemukan, dengan ringkasan dalam badan ulasan. Jika tidak ada masalah yang ditemukan, Code Review memperbarui jalankan pemeriksaan GitHub untuk menunjukkan bahwa tidak ada masalah yang terdeteksi. Claude juga dapat memposting komentar konfirmasi singkat pada PR.

Ulasan diskalakan dalam biaya dengan ukuran dan kompleksitas PR, selesai rata-rata dalam 20 menit. Owner dapat memantau aktivitas ulasan dan pengeluaran melalui [dasbor analitik](#view-usage).

<h3 id="severity-levels">
  Tingkat keparahan
</h3>

Setiap temuan diberi tag dengan tingkat keparahan:

| Penanda | Keparahan            | Arti                                                              |
| :------ | :------------------- | :---------------------------------------------------------------- |
| 🔴      | Penting              | Bug yang harus diperbaiki sebelum penggabungan                    |
| 🟡      | Nit                  | Masalah kecil, layak diperbaiki tetapi tidak memblokir            |
| 🟣      | Sudah ada sebelumnya | Bug yang ada di basis kode tetapi tidak diperkenalkan oleh PR ini |

Temuan mencakup bagian penalaran yang dapat diperluas yang dapat Anda perluas untuk memahami mengapa Claude menandai masalah dan bagaimana Claude memverifikasi masalah.

<h3 id="rate-and-reply-to-findings">
  Menilai dan membalas temuan
</h3>

Setiap komentar ulasan dari Claude tiba dengan 👍 dan 👎 sudah terpasang sehingga kedua tombol muncul di UI GitHub untuk penilaian satu klik. Klik 👍 jika temuan berguna atau 👎 jika salah atau bising. Anthropic mengumpulkan hitungan reaksi setelah PR digabungkan dan menggunakannya untuk menyetel pengulas. Reaksi tidak memicu ulasan kembali atau mengubah apa pun pada PR.

Membalas komentar sebaris tidak mendorong Claude untuk merespons atau memperbarui PR. Untuk bertindak atas temuan, perbaiki kode dan push. Jika PR berlangganan ulasan yang dipicu push, jalankan berikutnya menyelesaikan utas ketika masalah diperbaiki. Untuk meminta ulasan segar tanpa push, komentari `@claude review once` sebagai [komentar PR tingkat atas](#manually-trigger-reviews).

<h3 id="check-run-output">
  Output jalankan pemeriksaan
</h3>

Selain komentar ulasan sebaris, setiap ulasan mengisi jalankan pemeriksaan **Claude Code Review** yang muncul bersama pemeriksaan CI Anda. Perluas tautan **Details** untuk melihat ringkasan setiap temuan di satu tempat, diurutkan berdasarkan keparahan:

| Keparahan  | File:Baris                | Masalah                                                                     |
| ---------- | ------------------------- | --------------------------------------------------------------------------- |
| 🔴 Penting | `src/auth/session.ts:142` | Penyegaran token berjalan dengan logout, meninggalkan sesi basi aktif       |
| 🟡 Nit     | `src/auth/session.ts:88`  | `parseExpiry` secara diam-diam mengembalikan 0 pada input yang salah bentuk |

Setiap temuan juga muncul sebagai anotasi di tab **Files changed**, ditandai langsung pada baris diff yang relevan. Temuan Penting dirender dengan penanda merah, nit dengan peringatan kuning, dan bug yang sudah ada sebelumnya dengan pemberitahuan abu-abu. Anotasi dan tabel keparahan ditulis ke jalankan pemeriksaan secara independen dari komentar ulasan sebaris, sehingga tetap tersedia bahkan jika GitHub menolak komentar sebaris pada baris yang bergerak.

Jalankan pemeriksaan selalu selesai dengan kesimpulan netral sehingga tidak pernah memblokir penggabungan melalui aturan perlindungan cabang. Jika Anda ingin menggerbang penggabungan pada temuan Code Review, baca rincian keparahan dari output jalankan pemeriksaan di CI Anda sendiri. Baris terakhir dari teks Details adalah komentar yang dapat dibaca mesin yang dapat diurai alur kerja Anda dengan `gh` dan jq:

```bash theme={null}
gh api repos/OWNER/REPO/check-runs/CHECK_RUN_ID \
  --jq '.output.text | split("bughunter-severity: ")[1] | split(" -->")[0] | fromjson'
```

Ini mengembalikan objek JSON dengan hitungan per keparahan, misalnya `{"normal": 2, "nit": 1, "pre_existing": 0}`. Kunci `normal` menyimpan hitungan temuan Penting; nilai bukan nol berarti Claude menemukan setidaknya satu bug yang layak diperbaiki sebelum penggabungan.

<h3 id="what-code-review-checks">
  Apa yang Code Review periksa
</h3>

Secara default, Code Review berfokus pada kebenaran: bug yang akan merusak produksi, bukan preferensi pemformatan atau cakupan pengujian yang hilang. Anda dapat memperluas apa yang diperiksa dengan [menambahkan file panduan](#customize-reviews) ke repositori Anda.

<h2 id="set-up-code-review">
  Siapkan Code Review
</h2>

Pemilik mengaktifkan Code Review sekali untuk organisasi dan memilih repositori mana yang akan disertakan.

<Steps>
  <Step title="Buka pengaturan admin Claude Code">
    Buka [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code) dan temukan bagian Code Review. Anda memerlukan peran Pemilik atau Pemilik Utama di organisasi Claude Anda dan izin untuk memasang GitHub Apps di organisasi GitHub Anda.
  </Step>

  <Step title="Mulai penyiapan">
    Klik **Setup**. Ini memulai alur instalasi GitHub App.
  </Step>

  <Step title="Pasang Claude GitHub App">
    Ikuti petunjuk untuk memasang Claude GitHub App ke organisasi GitHub Anda. Aplikasi meminta izin repositori ini:

    * **Contents**: baca dan tulis
    * **Issues**: baca dan tulis
    * **Pull requests**: baca dan tulis

    Code Review menggunakan akses baca ke konten dan akses tulis ke permintaan tarik. Kumpulan izin yang lebih luas juga mendukung [GitHub Actions](/docs/id/github-actions) jika Anda mengaktifkannya nanti.
  </Step>

  <Step title="Pilih repositori">
    Pilih repositori mana yang akan diaktifkan untuk Code Review. Jika Anda tidak melihat repositori, pastikan Anda memberikan akses Claude GitHub App ke repositori tersebut selama instalasi. Anda dapat menambahkan lebih banyak repositori nanti.
  </Step>

  <Step title="Atur pemicu ulasan per repo">
    Setelah penyiapan selesai, bagian Code Review menampilkan repositori Anda dalam tabel. Untuk setiap repositori, gunakan dropdown **Review Behavior** untuk memilih kapan ulasan berjalan:

    * **Once after PR creation**: ulasan berjalan sekali ketika PR dibuka atau ditandai siap untuk ditinjau
    * **After every push**: ulasan berjalan pada setiap push ke cabang PR, menangkap masalah baru saat PR berkembang dan secara otomatis menyelesaikan utas ketika Anda memperbaiki masalah yang ditandai
    * **Manual**: ulasan dimulai hanya ketika seseorang [mengomentari `@claude review` atau `@claude review once` pada PR](#manually-trigger-reviews); `@claude review` juga berlangganan PR ke ulasan pada push berikutnya

    Meninjau pada setiap push menjalankan ulasan paling banyak dan biaya paling banyak. Mode manual berguna untuk repo lalu lintas tinggi di mana Anda ingin memilih PR tertentu untuk ditinjau, atau hanya mulai meninjau PR Anda setelah siap.
  </Step>
</Steps>

Tabel repositori juga menampilkan biaya rata-rata per ulasan untuk setiap repo berdasarkan aktivitas terbaru. Gunakan menu tindakan baris untuk mengaktifkan atau menonaktifkan Code Review per repositori, atau untuk menghapus repositori sepenuhnya.

Untuk memverifikasi penyiapan, buka PR pengujian. Jika Anda memilih pemicu otomatis, jalankan pemeriksaan bernama **Claude Code Review** muncul dalam beberapa menit. Jika Anda memilih Manual, komentari `@claude review` pada PR untuk memulai ulasan pertama. Jika tidak ada jalankan pemeriksaan yang muncul, konfirmasi repositori terdaftar di pengaturan admin Anda dan Claude GitHub App memiliki akses ke repositori tersebut.

<h2 id="manually-trigger-reviews">
  Memicu ulasan secara manual
</h2>

Dua perintah komentar memulai ulasan sesuai permintaan. Keduanya berfungsi terlepas dari pemicu yang dikonfigurasi repositori, sehingga Anda dapat menggunakannya untuk memilih PR tertentu ke dalam ulasan dalam mode Manual atau untuk mendapatkan ulasan kembali segera di mode lain.

| Perintah              | Apa yang dilakukannya                                                     |
| :-------------------- | :------------------------------------------------------------------------ |
| `@claude review`      | Memulai ulasan dan berlangganan PR ke ulasan yang dipicu push ke depannya |
| `@claude review once` | Memulai ulasan tunggal tanpa berlangganan PR ke push masa depan           |

Gunakan `@claude review once` ketika Anda menginginkan umpan balik tentang keadaan saat ini dari PR tetapi tidak menginginkan setiap push berikutnya untuk menimbulkan ulasan. Ini berguna untuk PR yang berjalan lama dengan push yang sering, atau ketika Anda menginginkan pendapat kedua sekali saja tanpa mengubah perilaku ulasan PR.

Agar perintah apa pun memicu ulasan:

* Posting sebagai komentar PR tingkat atas, bukan komentar sebaris pada baris diff
* Letakkan perintah di awal komentar, dengan `once` pada baris yang sama jika Anda menggunakan bentuk satu kali
* Anda harus memiliki akses pemilik, anggota, atau kolaborator ke repositori
* PR harus terbuka

Tidak seperti pemicu otomatis, pemicu manual berjalan pada PR draf, karena permintaan eksplisit menandakan Anda menginginkan ulasan sekarang terlepas dari status draf.

Jika ulasan sudah berjalan pada PR tersebut, permintaan antri sampai ulasan yang sedang berlangsung selesai. Anda dapat memantau kemajuan melalui jalankan pemeriksaan pada PR.

<h2 id="customize-reviews">
  Sesuaikan ulasan
</h2>

Code Review membaca dua file dari repositori Anda untuk memandu apa yang ditandai. Keduanya berbeda dalam seberapa kuat mereka mempengaruhi ulasan:

* **`CLAUDE.md`**: instruksi proyek bersama yang digunakan Claude Code untuk semua tugas, bukan hanya ulasan. Code Review membacanya sebagai konteks proyek dan menandai pelanggaran yang baru diperkenalkan sebagai nit.
* **`REVIEW.md`**: instruksi khusus ulasan, disuntikkan langsung ke setiap agen dalam saluran ulasan sebagai prioritas tertinggi. Gunakan untuk mengubah apa yang ditandai, pada tingkat keparahan apa, dan bagaimana temuan dilaporkan.

<h3 id="claude-md">
  CLAUDE.md
</h3>

Code Review membaca file `CLAUDE.md` repositori Anda dan memperlakukan pelanggaran yang baru diperkenalkan sebagai temuan tingkat [nit](#severity-levels). Ini berfungsi dua arah: jika PR Anda mengubah kode dengan cara yang membuat pernyataan `CLAUDE.md` ketinggalan zaman, Claude menandai bahwa dokumen perlu diperbarui juga.

Claude membaca file `CLAUDE.md` di setiap tingkat hierarki direktori Anda, jadi aturan di `CLAUDE.md` subdirektori hanya berlaku untuk file di bawah jalur tersebut. Lihat [dokumentasi memori](/docs/id/memory) untuk lebih lanjut tentang cara kerja `CLAUDE.md`.

Untuk panduan khusus ulasan yang tidak ingin Anda terapkan pada sesi Claude Code umum, gunakan [`REVIEW.md`](#review-md) sebagai gantinya.

<h3 id="review-md">
  REVIEW\.md
</h3>

`REVIEW.md` adalah file di akar repositori Anda yang mengganti cara Code Review berperilaku di repo Anda. Isinya disuntikkan ke dalam prompt sistem setiap agen dalam saluran ulasan sebagai blok instruksi prioritas tertinggi, mengambil alih dari panduan ulasan default.

Karena ditempel verbatim, `REVIEW.md` adalah instruksi biasa: sintaks [`@` import](/docs/id/memory#import-additional-files) tidak diperluas, dan file yang direferensikan tidak dibaca ke dalam prompt. Letakkan aturan yang ingin Anda terapkan langsung di file.

<h4 id="what-you-can-tune">
  Apa yang dapat Anda sesuaikan
</h4>

`REVIEW.md` adalah markdown bentuk bebas, jadi apa pun yang dapat Anda ekspresikan sebagai instruksi ulasan berada dalam cakupan. Pola di bawah ini memiliki dampak paling besar dalam praktik.

**Keparahan**: tentukan ulang apa yang 🔴 Penting berarti untuk repo Anda. Kalibrasi default menargetkan kode produksi; repo dokumen, repo konfigurasi, atau prototipe mungkin menginginkan definisi yang jauh lebih sempit. Nyatakan secara eksplisit kelas temuan mana yang Penting dan mana yang paling banyak Nit. Anda juga dapat meningkatkan ke arah lain, misalnya memperlakukan pelanggaran `CLAUDE.md` apa pun sebagai Penting daripada nit default.

**Volume nit**: batasi berapa banyak komentar 🟡 Nit yang diposting ulasan tunggal. Prosa dan file konfigurasi dapat dipoles selamanya. Batas seperti "laporkan paling banyak lima nit, sebutkan sisanya sebagai hitungan dalam ringkasan" membuat ulasan dapat ditindaklanjuti.

**Aturan lewati**: daftar jalur, pola cabang, dan kategori temuan di mana Claude tidak boleh memposting temuan. Kandidat umum adalah kode yang dihasilkan, lockfile, dependensi yang dijual, dan cabang yang dibuat mesin, bersama dengan apa pun yang CI Anda sudah terapkan seperti linting atau pemeriksaan ejaan. Untuk jalur yang memerlukan beberapa ulasan tetapi bukan pengawasan penuh, tetapkan standar yang lebih tinggi alih-alih melewati sepenuhnya: "di `scripts/`, hanya laporkan jika hampir pasti dan parah."

**Pemeriksaan khusus repo**: tambahkan aturan yang ingin Anda tandai pada setiap PR, seperti "rute API baru harus memiliki tes integrasi." Karena `REVIEW.md` disuntikkan sebagai prioritas tertinggi, ini mendarat lebih andal daripada aturan yang sama dalam `CLAUDE.md` yang panjang.

**Bilah verifikasi**: memerlukan bukti sebelum kelas temuan diposting. Misalnya, "klaim perilaku memerlukan kutipan `file:line` dalam sumber, bukan inferensi dari penamaan" mengurangi positif palsu yang akan menghabiskan penulis putaran perjalanan.

**Konvergensi ulasan kembali**: beri tahu Claude cara berperilaku ketika PR sudah ditinjau. Aturan seperti "setelah ulasan pertama, tekan nit baru dan posting temuan Penting saja" menghentikan perbaikan satu baris dari mencapai putaran ketujuh hanya berdasarkan gaya.

**Bentuk ringkasan**: minta badan ulasan untuk dibuka dengan tally satu baris seperti `2 faktual, 4 gaya`, dan untuk memimpin dengan "tidak ada masalah faktual" ketika itu kasusnya. Penulis ingin mengetahui bentuk pekerjaan sebelum detail.

<h4 id="example">
  Contoh
</h4>

`REVIEW.md` ini mengkalibrasi ulang keparahan untuk layanan backend, membatasi nit, melewati file yang dihasilkan, dan menambahkan pemeriksaan khusus repo.

```markdown theme={null}
# Instruksi ulasan

## Apa yang Penting berarti di sini

Cadangkan Penting untuk temuan yang akan merusak perilaku, membocorkan data,
atau memblokir rollback: logika yang tidak benar, kueri basis data yang tidak terbatas, PII
dalam log atau pesan kesalahan, dan migrasi yang tidak kompatibel
ke belakang. Gaya, penamaan, dan saran refactoring adalah Nit paling
banyak.

## Batasi nit

Laporkan paling banyak lima Nit per ulasan. Jika Anda menemukan lebih banyak, katakan "plus N
item serupa" dalam ringkasan alih-alih mempostingnya sebaris. Jika
semuanya yang Anda temukan adalah Nit, pimpin ringkasan dengan "Tidak ada masalah pemblokiran."

## Jangan laporkan

- Apa pun yang CI sudah terapkan: lint, pemformatan, kesalahan tipe
- File yang dihasilkan di bawah `src/gen/` dan file `*.lock` apa pun
- Kode khusus pengujian yang sengaja melanggar aturan produksi

## Selalu periksa

- Rute API baru memiliki tes integrasi
- Baris log tidak menyertakan alamat email, ID pengguna, atau badan permintaan
- Kueri basis data dibatasi ke penyewa pemanggil
```

<h4 id="keep-it-focused">
  Jaga agar tetap fokus
</h4>

Panjang memiliki biaya: `REVIEW.md` yang panjang mengencerkan aturan yang paling penting. Jaga agar tetap pada instruksi yang mengubah perilaku ulasan, dan tinggalkan konteks proyek umum di `CLAUDE.md`.

<h2 id="view-usage">
  Lihat penggunaan
</h2>

Buka [claude.ai/analytics/code-review](https://claude.ai/analytics/code-review) untuk melihat aktivitas Code Review di seluruh organisasi Anda. Dasbor menampilkan:

| Bagian               | Apa yang ditampilkan                                                                           |
| :------------------- | :--------------------------------------------------------------------------------------------- |
| PRs reviewed         | Hitungan harian permintaan tarik yang ditinjau selama rentang waktu yang dipilih               |
| Cost weekly          | Pengeluaran mingguan pada Code Review                                                          |
| Feedback             | Hitungan komentar ulasan yang secara otomatis diselesaikan karena pengembang mengatasi masalah |
| Repository breakdown | Hitungan per-repo PR yang ditinjau dan komentar yang diselesaikan                              |

Tabel repositori di pengaturan admin juga menampilkan biaya rata-rata per ulasan untuk setiap repo. Angka biaya dasbor adalah perkiraan untuk memantau aktivitas; untuk pengeluaran yang akurat pada tagihan, lihat tagihan Anthropic Anda.

<h2 id="pricing">
  Harga
</h2>

Code Review ditagih berdasarkan penggunaan token. Setiap ulasan rata-rata \$15-25 dalam biaya, diskalakan dengan ukuran PR, kompleksitas basis kode, dan berapa banyak masalah yang memerlukan verifikasi. Penggunaan Code Review ditagih secara terpisah melalui [penggunaan ekstra](https://support.claude.com/id/articles/12429409-extra-usage-for-paid-claude-plans) dan tidak dihitung terhadap penggunaan yang disertakan dalam paket Anda.

Pemicu ulasan yang Anda pilih mempengaruhi biaya total:

* **Once after PR creation**: berjalan sekali per PR
* **After every push**: berjalan pada setiap push, mengalikan biaya dengan jumlah push
* **Manual**: tidak ada ulasan sampai seseorang mengomentari `@claude review` pada PR

Dalam mode apa pun, mengomentari `@claude review` [memilih PR ke dalam ulasan yang dipicu push](#manually-trigger-reviews), jadi biaya tambahan terjadi per push setelah komentar tersebut. Untuk menjalankan ulasan tunggal tanpa berlangganan ke push masa depan, komentari `@claude review once` sebagai gantinya.

Biaya muncul pada tagihan Anthropic Anda terlepas dari apakah organisasi Anda menggunakan Amazon Bedrock atau Google Cloud's Agent Platform untuk fitur Claude Code lainnya. Untuk menetapkan batas pengeluaran bulanan untuk Code Review, buka [claude.ai/admin-settings/usage](https://claude.ai/admin-settings/usage) dan konfigurasikan batas untuk layanan Claude Code Review.

Pantau pengeluaran melalui bagan biaya mingguan di [analitik](#view-usage) atau kolom biaya rata-rata per-repo di pengaturan admin.

<h2 id="troubleshooting">
  Pemecahan masalah
</h2>

Jalankan ulasan adalah upaya terbaik. Jalankan yang gagal tidak pernah memblokir PR Anda, tetapi juga tidak mencoba ulang dengan sendirinya. Bagian ini mencakup cara pulih dari jalankan yang gagal dan tempat mencari ketika jalankan pemeriksaan melaporkan masalah yang tidak dapat Anda temukan.

<h3 id="retrigger-a-failed-or-timed-out-review">
  Picu ulang ulasan yang gagal atau habis waktu
</h3>

Ketika infrastruktur ulasan mengalami kesalahan internal atau melampaui batas waktu, jalankan pemeriksaan selesai dengan judul **Code review encountered an error** atau **Code review timed out**. Kesimpulannya masih netral, jadi tidak ada yang memblokir penggabungan Anda, tetapi tidak ada temuan yang diposting.

Untuk menjalankan ulasan lagi, komentari `@claude review once` pada PR. Ini memulai ulasan segar tanpa berlangganan PR ke push masa depan. Jika PR sudah berlangganan ulasan yang dipicu push, push komit baru juga memulai ulasan baru.

Tombol **Re-run** di tab Checks GitHub tidak memicu ulang Code Review. Gunakan perintah komentar atau push baru sebagai gantinya.

<h3 id="review-didn’t-run-and-the-pr-shows-a-spend-cap-message">
  Ulasan tidak berjalan dan PR menampilkan pesan batas pengeluaran
</h3>

Ketika batas pengeluaran bulanan organisasi Anda tercapai, Code Review memposting komentar tunggal pada PR yang menjelaskan bahwa ulasan dilewati. Ulasan dilanjutkan secara otomatis pada awal periode penagihan berikutnya, atau segera ketika admin menaikkan batas di [claude.ai/admin-settings/usage](https://claude.ai/admin-settings/usage).

<h3 id="find-issues-that-aren’t-showing-as-inline-comments">
  Temukan masalah yang tidak ditampilkan sebagai komentar sebaris
</h3>

Jika judul jalankan pemeriksaan mengatakan masalah ditemukan tetapi Anda tidak melihat komentar ulasan sebaris pada diff, cari di lokasi lain tempat temuan ditampilkan:

* **Check run Details**: klik **Details** di sebelah jalankan pemeriksaan Claude Code Review di tab Checks. Tabel keparahan mencantumkan setiap temuan dengan file, baris, dan ringkasannya terlepas dari apakah komentar sebaris diterima.
* **Files changed annotations**: buka tab **Files changed** pada PR. Temuan dirender sebagai anotasi yang terpasang langsung ke baris diff, terpisah dari komentar ulasan.
* **Review body**: jika Anda push ke PR saat ulasan sedang berjalan, beberapa temuan mungkin mereferensikan baris yang tidak lagi ada di diff saat ini. Ini muncul di bawah judul **Additional findings** dalam teks badan ulasan daripada sebagai komentar sebaris.

<h2 id="review-a-diff-locally">
  Meninjau diff secara lokal
</h2>

Perintah [`/code-review`](/docs/id/commands) meninjau diff di terminal Anda tanpa memasang GitHub App. Jalankan dalam sesi Claude Code apa pun: ini melaporkan bug kebenaran dan penggunaan kembali, penyederhanaan, dan pembersihan efisiensi. Secara default, ulasan lokal mencakup komit cabang Anda yang berada di depan upstream-nya ditambah perubahan yang tidak dilakukan dalam pohon kerja. Lewatkan `--comment` untuk memposting temuan sebagai komentar PR sebaris, atau `--fix` untuk menerapkan temuan ke pohon kerja Anda setelah ulasan.

[Tingkat upaya](/docs/id/model-config#adjust-effort-level) yang lebih rendah mengembalikan temuan yang lebih sedikit dan lebih percaya diri, sementara `high` hingga `max` memberikan cakupan yang lebih luas dan mungkin mencakup temuan yang tidak pasti. Tanpa argumen upaya, ulasan menggunakan upaya saat ini sesi. Untuk meninjau sesuatu selain diff default, lewatkan target: jalur file, nomor PR, nama cabang, atau rentang ref seperti `main...my-feature`. Bentuk rentang ref meninjau diff yang dilakukan yang akan dimuat pull request dari `my-feature` ke `main`, terlepas dari bagaimana upstream cabang dikonfigurasi.

`/code-review ultra --fix` menjalankan [ultrareview](/docs/id/ultrareview) yang lebih dalam di cloud, kemudian menerapkan temuannya ke pohon kerja Anda ketika mereka kembali dalam sesi Anda. Ultrareview menggunakan cakupannya sendiri: cabang saat ini Anda terhadap cabang default repositori, ditambah perubahan yang tidak dilakukan dan staged dalam pohon kerja.

Perintah ini dinamai `/simplify` sebelum v2.1.147, ketika ia menerapkan perbaikan secara default. Dari v2.1.154, `/simplify` menjalankan ulasan pembersihan terpisah yang menerapkan perbaikan tanpa mencari bug. Jika Anda membuat skrip `/simplify` untuk pencarian bug, beralih ke `/code-review --fix`, yang tidak berubah.

<h2 id="related-resources">
  Sumber daya terkait
</h2>

Code Review dirancang untuk bekerja bersama dengan sisa Claude Code. Jika Anda ingin menjalankan ulasan secara lokal sebelum membuka PR, memerlukan penyiapan yang di-host sendiri, atau ingin mendalami cara `CLAUDE.md` membentuk perilaku Claude di seluruh alat, halaman-halaman ini adalah perhentian berikutnya yang baik:

* [Commands](/docs/id/commands): jalankan `/code-review` dalam sesi Claude Code lokal untuk memeriksa diff sebelum push
* [GitHub Actions](/docs/id/github-actions): jalankan Claude dalam alur kerja GitHub Actions Anda sendiri untuk otomasi khusus di luar ulasan kode
* [GitLab CI/CD](/docs/id/gitlab-ci-cd): integrasi Claude yang di-host sendiri untuk pipeline GitLab
* [Memory](/docs/id/memory): cara kerja file `CLAUDE.md` di seluruh Claude Code
* [Analytics](/docs/id/analytics): lacak penggunaan Claude Code di luar ulasan kode
