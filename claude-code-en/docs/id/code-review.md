> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Code Review

> Siapkan ulasan PR otomatis yang menangkap kesalahan logika, kerentanan keamanan, dan regresi menggunakan analisis multi-agen dari seluruh basis kode Anda

<Note>
  Code Review sedang dalam pratinjau penelitian, tersedia untuk langganan [Team dan Enterprise](https://claude.ai/admin-settings/claude-code). Tidak tersedia untuk organisasi dengan [Zero Data Retention](/docs/id/zero-data-retention) yang diaktifkan. Pada paket lain, Anda masih dapat [meninjau diff secara lokal](#review-a-diff-locally) dengan perintah `/code-review`.
</Note>

Code Review menganalisis permintaan tarik GitHub Anda dan memposting temuan sebagai komentar sebaris pada baris kode tempat ditemukannya masalah. Armada agen khusus memeriksa perubahan kode dalam konteks basis kode lengkap Anda, mencari kesalahan logika, kerentanan keamanan, kasus tepi yang rusak, dan regresi halus.

Temuan diberi tag berdasarkan tingkat keparahan dan tidak menyetujui atau memblokir PR Anda, sehingga alur kerja ulasan yang ada tetap utuh. Anda dapat menyesuaikan apa yang Claude tandai dengan menambahkan file `CLAUDE.md` atau `REVIEW.md` ke repositori Anda.

Untuk menjalankan Claude di infrastruktur CI Anda sendiri alih-alih layanan terkelola ini, lihat [GitHub Actions](/docs/id/github-actions) atau [GitLab CI/CD](/docs/id/gitlab-ci-cd). Untuk repositori pada instans GitHub yang di-host sendiri, lihat [GitHub Enterprise Server](/docs/id/github-enterprise-server).

Halaman ini mencakup:

* [Cara kerja ulasan](#how-reviews-work)
* [Penyiapan](#set-up-code-review)
* [Memicu ulasan secara manual](#manually-trigger-reviews) dengan `@claude review` dan `@claude review always`
* [Menyesuaikan ulasan](#customize-reviews) dengan `CLAUDE.md` dan `REVIEW.md`
* [Harga](#pricing)
* [Pemecahan masalah](#troubleshooting) jalankan yang gagal dan komentar yang hilang
* [Meninjau diff secara lokal](#review-a-diff-locally) dengan perintah `/code-review`

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

Membalas komentar sebaris tidak mendorong Claude untuk merespons atau memperbarui PR. Untuk bertindak atas temuan, perbaiki kode dan push. Jika PR berlangganan ulasan yang dipicu push, jalankan berikutnya menyelesaikan utas ketika masalah diperbaiki. Untuk meminta ulasan segar tanpa push, komentari `@claude review` sebagai [komentar PR tingkat atas](#manually-trigger-reviews).

Untuk menolak temuan tanpa perubahan kode, selesaikan utas; membalas tidak menolaknya.

<h3 id="check-run-output">
  Output jalankan pemeriksaan
</h3>

Selain komentar ulasan sebaris, setiap ulasan mengisi jalankan pemeriksaan **Claude Code Review** yang muncul bersama pemeriksaan CI Anda. Perluas tautan **Details** untuk melihat ringkasan setiap temuan di satu tempat, diurutkan berdasarkan keparahan:

| Keparahan  | File:Baris                | Masalah                                                                     |
| ---------- | ------------------------- | --------------------------------------------------------------------------- |
| 🔴 Penting | `src/auth/session.ts:142` | Penyegaran token berjalan dengan logout, meninggalkan sesi basi aktif       |
| 🟡 Nit     | `src/auth/session.ts:88`  | `parseExpiry` secara diam-diam mengembalikan 0 pada input yang salah bentuk |

Setiap temuan juga muncul sebagai anotasi di tab **Files changed**, ditandai langsung pada baris diff yang relevan. Temuan Penting dirender dengan penanda merah, nit dengan peringatan kuning, dan bug yang sudah ada sebelumnya dengan pemberitahuan abu-abu. Anotasi dan tabel keparahan ditulis ke jalankan pemeriksaan secara independen dari komentar ulasan sebaris, sehingga tetap tersedia bahkan jika GitHub menolak komentar sebaris pada baris yang bergerak.

Jalankan pemeriksaan selalu selesai dengan kesimpulan netral sehingga tidak pernah memblokir penggabungan melalui aturan perlindungan cabang. Jika Anda ingin menggerbang penggabungan pada temuan Code Review, baca rincian keparahan dari output jalankan pemeriksaan di CI Anda sendiri. Baris terakhir dari teks Details adalah komentar yang dapat dibaca mesin yang dapat diurai alur kerja Anda dengan `gh` dan jq. Untuk menemukan ID jalankan pemeriksaan, daftarkan jalankan pemeriksaan komit dengan `gh api repos/OWNER/REPO/commits/<commit-sha>/check-runs --jq '.check_runs[] | {id, name}'` dan ambil `id` dari jalankan `Claude Code Review`. Ganti `OWNER`, `REPO`, dan `CHECK_RUN_ID` dengan pemilik repositori, nama repositori, dan ID tersebut:

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
    Ikuti petunjuk untuk memasang Claude GitHub App: pilih organisasi GitHub yang memiliki repositori yang ingin Anda tinjau, pilih repositori mana yang dapat diakses aplikasi, dan setujui izin yang diminta.

    Untuk meninjau permintaan tarik, Claude membaca konten repositori Anda melalui akses baca aplikasi, dan memposting komentar serta [jalankan pemeriksaan](#check-run-output) melalui akses tulisnya ke permintaan tarik dan pemeriksaan. Selama instalasi, Anda memberikan kumpulan izin yang lebih luas yang dibagikan oleh fitur Claude lainnya, seperti [GitHub Actions](/docs/id/github-actions); lihat [izin GitHub App](/docs/id/github-actions#github-app-permissions) untuk daftar lengkapnya.
  </Step>

  <Step title="Pilih repositori">
    Pilih repositori mana yang akan diaktifkan untuk Code Review. Jika Anda tidak melihat repositori, pastikan Anda memberikan akses Claude GitHub App ke repositori tersebut selama instalasi. Anda dapat menambahkan lebih banyak repositori nanti.
  </Step>

  <Step title="Atur pemicu ulasan per repo">
    Setelah penyiapan selesai, bagian Code Review menampilkan repositori Anda dalam tabel. Untuk setiap repositori, gunakan dropdown **Review Behavior** untuk memilih kapan ulasan berjalan:

    * **Once after PR creation**: ulasan berjalan sekali ketika PR dibuka atau ditandai siap untuk ditinjau
    * **After every push**: ulasan berjalan pada setiap push ke cabang PR, menangkap masalah baru saat PR berkembang dan secara otomatis menyelesaikan utas ketika Anda memperbaiki masalah yang ditandai
    * **Manual**: membuka atau push ke PR tidak memulai ulasan; komentari [`@claude review`](#manually-trigger-reviews) untuk meminta satu, atau `@claude review always` untuk juga berlangganan PR ke ulasan pada push berikutnya

    Pilihan mana pun yang Anda pilih, Claude meninjau [permintaan tarik dari fork](#review-pull-requests-from-forks) hanya ketika seseorang mengomentari `@claude review` di dalamnya.

    Meninjau pada setiap push menjalankan ulasan paling banyak dan biaya paling banyak. Mode manual berguna untuk repo lalu lintas tinggi di mana Anda ingin memilih PR tertentu untuk ditinjau, atau hanya mulai meninjau PR Anda setelah siap.
  </Step>
</Steps>

Tabel repositori juga menampilkan biaya rata-rata per ulasan untuk setiap repo berdasarkan aktivitas terbaru. Gunakan menu tindakan baris untuk mengaktifkan atau menonaktifkan Code Review per repositori, atau untuk menghapus repositori sepenuhnya.

Untuk memverifikasi penyiapan, buka PR pengujian. Jika Anda memilih pemicu otomatis, jalankan pemeriksaan bernama **Claude Code Review** muncul dalam beberapa menit. Jika Anda memilih Manual, komentari `@claude review` pada PR untuk memulai ulasan pertama. Jika tidak ada jalankan pemeriksaan yang muncul, konfirmasi repositori terdaftar di pengaturan admin Anda dan Claude GitHub App memiliki akses ke repositori tersebut.

<h2 id="manually-trigger-reviews">
  Memicu ulasan secara manual
</h2>

Perintah komentar memulai ulasan sesuai permintaan. Perintah-perintah ini berfungsi terlepas dari pemicu yang dikonfigurasi repositori, sehingga Anda dapat menggunakannya untuk memilih PR tertentu ke dalam ulasan dalam mode Manual atau untuk mendapatkan ulasan kembali segera di mode lain.

| Perintah                | Apa yang dilakukannya                                                                 |
| :---------------------- | :------------------------------------------------------------------------------------ |
| `@claude review`        | Memulai ulasan tunggal tanpa berlangganan PR ke ulasan yang dipicu push di masa depan |
| `@claude review always` | Memulai ulasan dan berlangganan PR ke ulasan yang dipicu push ke depannya             |
| `@claude review once`   | Sama seperti `@claude review`: memulai ulasan tunggal tanpa berlangganan              |

Gunakan `@claude review always` ketika Anda menginginkan setiap push berikutnya ke PR untuk memulai ulasan segar, seperti pada PR prioritas tinggi di repositori yang diatur ke mode Manual. Karena perintah dasar tidak berlangganan PR, Anda dapat meminta pendapat kedua sekali saja tanpa mengubah apakah push berikutnya memicu ulasan.

<Note>
  Sebelum pembaruan Juli 2026, `@claude review` berlangganan PR ke ulasan yang dipicu push. Jika Anda mengandalkan perilaku tersebut, komentar `@claude review always` sebagai gantinya. `@claude review once` masih berfungsi dan berperilaku sama seperti perintah dasar.
</Note>

Agar perintah apa pun memicu ulasan:

* Posting sebagai komentar PR tingkat atas, bukan komentar sebaris pada baris diff
* Letakkan perintah di awal komentar, dengan `once` atau `always` pada baris yang sama dengan sisa perintah
* Anda harus memiliki izin tulis, pertahankan, atau admin pada repositori
* PR harus terbuka

Jika repositori milik organisasi dan keanggotaan Anda di organisasi tersebut bersifat pribadi, yang merupakan default GitHub, GitHub tidak mengidentifikasi Anda kepada Claude sebagai anggota. Claude mungkin masih bereaksi terhadap komentar Anda dengan 👀, tetapi tidak memulai ulasan kecuali Anda ditambahkan ke repositori secara langsung sebagai kolaborator, bahkan ketika tim atau izin dasar organisasi memberi Anda akses tulis. Untuk memperbaiki ini, [buat keanggotaan organisasi Anda publik](https://docs.github.com/en/account-and-profile/setting-up-and-managing-your-personal-account-on-github/managing-your-membership-in-organizations/publicizing-or-hiding-organization-membership) atau minta admin repositori untuk menambahkan Anda ke repositori sebagai kolaborator.

Tidak seperti pemicu otomatis, pemicu manual berjalan pada PR draf, karena permintaan eksplisit menandakan Anda menginginkan ulasan sekarang terlepas dari status draf.

Jika ulasan sudah berjalan pada PR tersebut, permintaan antri sampai ulasan yang sedang berlangsung selesai. Anda dapat memantau kemajuan melalui jalankan pemeriksaan pada PR.

<h3 id="review-pull-requests-from-forks">
  Ulasan permintaan tarik dari fork
</h3>

Claude tidak meninjau permintaan tarik dari fork secara otomatis, terlepas dari pengaturan **Review Behavior** repositori. Untuk memulai satu, komentar `@claude review` pada permintaan tarik. [Persyaratan untuk perintah komentar](#manually-trigger-reviews) masih berlaku, dan akses tulis yang Anda butuhkan adalah ke repositori dasar, bukan fork.

Untuk mendapatkan ulasan lain dari permintaan tarik fork, posting komentar `@claude review` baru. `@claude review always` juga berfungsi, tetapi tidak berlangganan permintaan tarik ke ulasan pada push berikutnya. Tidak ada yang lain selain perintah komentar yang memulai ulasan pada permintaan tarik fork:

* Mengklik **Re-run** pada jalankan pemeriksaan tidak memulai ulasan
* Mendorong komit baru tidak memulai ulasan, bahkan di repositori yang diatur ke **After every push**

<h2 id="customize-reviews">
  Sesuaikan ulasan
</h2>

Code Review membaca dua file dari repositori Anda untuk memandu apa yang ditandai. Keduanya berbeda dalam seberapa kuat mereka mempengaruhi ulasan:

* **`CLAUDE.md`**: instruksi proyek bersama yang digunakan Claude Code untuk semua tugas, bukan hanya ulasan. Code Review membacanya sebagai konteks proyek dan menandai pelanggaran yang baru diperkenalkan sebagai nit.
* **`REVIEW.md`**: instruksi khusus ulasan, diberikan kepada agen yang menemukan dan memverifikasi temuan dan dikonsultasikan oleh agen yang menentukan peringkat dan melaporkan temuan. Gunakan untuk mengatakan apa yang ingin ditandai oleh tim Anda, pada tingkat keparahan apa, dan bagaimana temuan dilaporkan.

<h3 id="claude-md">
  CLAUDE.md
</h3>

Code Review membaca file `CLAUDE.md` repositori Anda dan memperlakukan pelanggaran yang baru diperkenalkan sebagai temuan tingkat [nit](#severity-levels). Ini berfungsi dua arah: jika PR Anda mengubah kode dengan cara yang membuat pernyataan `CLAUDE.md` ketinggalan zaman, Claude menandai bahwa dokumen perlu diperbarui juga.

Claude membaca file `CLAUDE.md` di setiap tingkat hierarki direktori Anda, jadi aturan di `CLAUDE.md` subdirektori hanya berlaku untuk file di bawah jalur tersebut. Lihat [dokumentasi memori](/docs/id/memory) untuk lebih lanjut tentang cara kerja `CLAUDE.md`.

Untuk panduan khusus ulasan yang tidak ingin Anda terapkan pada sesi Claude Code umum, gunakan [`REVIEW.md`](#review-md) sebagai gantinya.

<h3 id="review-md">
  REVIEW\.md
</h3>

`REVIEW.md` adalah file di akar repositori Anda yang menyesuaikan Code Review dengan repo Anda. Agen dalam saluran ulasan yang menemukan dan memverifikasi temuan menerima isinya sebagai instruksi ulasan repositori Anda, bersama dengan panduan ulasan default Code Review, dan agen yang menentukan peringkat dan melaporkan temuan berkonsultasi dengannya sebelum menetapkan keparahan dan menulis ulasan.

Letakkan aturan yang ingin Anda terapkan langsung di `REVIEW.md`.

<h4 id="what-you-can-tune">
  Apa yang dapat Anda sesuaikan
</h4>

`REVIEW.md` adalah markdown bentuk bebas, jadi apa pun yang dapat Anda ekspresikan sebagai instruksi ulasan berada dalam cakupan. Pola di bawah ini memiliki dampak paling besar dalam praktik.

**Keparahan**: tentukan ulang apa yang 🔴 Penting berarti untuk repo Anda. Kalibrasi default menargetkan kode produksi; repo dokumen, repo konfigurasi, atau prototipe mungkin menginginkan definisi yang jauh lebih sempit. Nyatakan secara eksplisit kelas temuan mana yang Penting dan mana yang paling banyak Nit. Anda juga dapat meningkatkan ke arah lain, misalnya memperlakukan pelanggaran `CLAUDE.md` apa pun sebagai Penting daripada nit default.

**Volume nit**: batasi berapa banyak komentar 🟡 Nit yang diposting ulasan tunggal. Prosa dan file konfigurasi dapat dipoles selamanya. Batas seperti "laporkan paling banyak lima nit, sebutkan sisanya sebagai hitungan dalam ringkasan" membuat ulasan dapat ditindaklanjuti.

**Aturan lewati**: daftar jalur, pola cabang, dan kategori temuan di mana Claude tidak boleh memposting temuan. Kandidat umum adalah kode yang dihasilkan, lockfile, dependensi yang dijual, dan cabang yang dibuat mesin, bersama dengan apa pun yang CI Anda sudah terapkan seperti linting atau pemeriksaan ejaan. Untuk jalur yang memerlukan beberapa ulasan tetapi bukan pengawasan penuh, tetapkan standar yang lebih tinggi alih-alih melewati sepenuhnya: "di `scripts/`, hanya laporkan jika hampir pasti dan parah."

**Pemeriksaan khusus repo**: tambahkan aturan yang ingin Anda tandai pada setiap PR, seperti "rute API baru harus memiliki tes integrasi." Karena `REVIEW.md` menjangkau setiap agen temuan dan verifikasi secara langsung, ini mendarat lebih andal daripada aturan yang sama dalam `CLAUDE.md` yang panjang.

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

Angka biaya dasbor adalah perkiraan untuk memantau aktivitas. Untuk pengeluaran yang akurat pada tagihan, lihat tagihan Anthropic Anda.

<h2 id="pricing">
  Harga
</h2>

Code Review ditagih berdasarkan penggunaan token. Setiap ulasan rata-rata \$15-25 dalam biaya, diskalakan dengan ukuran PR, kompleksitas basis kode, dan berapa banyak masalah yang memerlukan verifikasi. Penggunaan Code Review ditagih secara terpisah melalui [penggunaan ekstra](https://support.claude.com/id/articles/12429409-extra-usage-for-paid-claude-plans) dan tidak dihitung terhadap penggunaan yang disertakan dalam paket Anda.

Pemicu ulasan yang Anda pilih mempengaruhi biaya total:

* **Once after PR creation**: berjalan sekali per PR
* **After every push**: berjalan pada setiap push, mengalikan biaya dengan jumlah push
* **Manual**: tidak ada ulasan pada PR terbuka atau push, jadi biaya hanya terjadi dari ulasan yang diminta seseorang

Dalam mode Once after PR creation atau Manual, mengomentari `@claude review always` [memilih PR ke dalam ulasan yang dipicu push](#manually-trigger-reviews), jadi biaya tambahan terjadi per push setelah komentar tersebut. Dalam mode After every push, push sudah memicu ulasan, jadi langganan tidak mengubah biaya per-push. Mengomentari `@claude review` menjalankan ulasan tunggal tanpa berlangganan ke push masa depan. Claude meninjau [pull request dari fork](#review-pull-requests-from-forks) hanya ketika seseorang mengomentari `@claude review`, jadi pull request fork tidak pernah mengakumulasi biaya per-push dalam mode apa pun.

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

Untuk menjalankan ulasan lagi, komentari `@claude review` pada PR. Ini memulai ulasan segar tanpa berlangganan PR ke push masa depan. Jika PR tidak [dari fork](#review-pull-requests-from-forks), Anda dapat mengklik **Re-run** pada pemeriksaan **Claude Code Review** di tab Checks GitHub. Jalankan ulang juga memulai ulasan segar tanpa berlangganan PR.

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

Perintah [`/code-review`](/docs/id/commands) meninjau diff di terminal Anda tanpa memasang GitHub App. Ini melaporkan bug kebenaran dan penggunaan kembali, penyederhanaan, dan pembersihan efisiensi.

`/review` adalah alias dari `/code-review`; sebelum v2.1.223, itu adalah perintah terpisah yang menjalankan ulasan baca-saja satu lintasan dari permintaan tarik GitHub.

<Steps>
  <Step title="Jalankan /code-review">
    Dari sesi tempat Anda bekerja, jalankan perintah:

    ```text theme={null}
    /code-review
    ```

    Ini meninjau komit cabang Anda yang berada di depan upstream-nya ditambah perubahan yang tidak dilakukan, jadi itu memerlukan pekerjaan pada cabang atau di pohon kerja untuk memiliki sesuatu untuk dilaporkan. Untuk meninjau sesuatu yang lain, lewatkan target: jalur file, nomor PR, nama cabang, atau rentang ref seperti `main...my-feature`.

    Anda juga dapat menambahkan flag:

    * `--fix`: menerapkan temuan ke pohon kerja Anda setelah ulasan
    * `--comment`: memposting temuan pada permintaan tarik GitHub sebagai komentar sebaris, atau pada permintaan penggabungan GitLab sebagai catatan tunggal
    * `--post`: pada ulasan cloud `ultra` dari permintaan tarik `github.com`, pra-memilih posting temuan yang selesai ke PR dalam dialog peluncuran; lihat [Posting temuan ke permintaan tarik](/docs/id/ultrareview#post-findings-to-the-pull-request). Memerlukan Claude Code v2.1.227 atau lebih baru

    Ketika Anda melewatkan `--comment` untuk permintaan penggabungan GitLab, Claude Code memposting temuan melalui CLI `glab` GitLab. Memerlukan Claude Code v2.1.257 atau lebih baru. Ketika `glab` tidak diinstal, Claude mencetak temuan di terminal sebagai gantinya.

    Lewatkan permintaan penggabungan sebagai URL-nya atau referensi `!123`. Claude Code memperlakukan nomor telanjang atau nama cabang sebagai permintaan penggabungan hanya ketika checkout asal berada di `gitlab.com`. Pada instans GitLab yang dikelola sendiri, lewatkan URL atau bentuk `!123`.
  </Step>

  <Step title="Terus bekerja">
    Ulasan berjalan sebagai [subagent](/docs/id/sub-agents) latar belakang dengan jendela konteks sendiri, jadi itu tidak mengisi percakapan Anda. Temuan tiba dalam percakapan Anda ketika ulasan selesai.
  </Step>

  <Step title="Bertindak atas temuan">
    Minta Claude untuk memperbaiki apa yang ditemukan ulasan. Jika Anda melewatkan `--fix` atau `--comment`, ulasan telah menerapkan atau memposting temuannya.
  </Step>
</Steps>

Claude melaporkan temuan sebagai teks dalam balasan di kedua lintasan ini, bahkan ketika aplikasi host meminta daftar temuan:

* Dalam sesi terminal, di mana `/code-review` menjalankan ulasan sebagai [subagent bercabang](/docs/id/skills#run-skills-in-a-subagent)
* Dalam lintasan `-p` dengan output teks atau JSON

Dalam aplikasi host yang meminta daftar temuan, seperti [aplikasi desktop](/docs/id/desktop), Claude melaporkan temuan ulasan melalui alat [`ReportFindings`](/docs/id/tools-reference). Claude Code merender laporan sebagai daftar temuan, dan setiap entri menunjukkan lokasi file, ringkasan satu kalimat, dan tag kategori seperti `correctness` ketika temuan membawanya. Permintaan host berlaku di setiap tingkat upaya dan memerlukan Claude Code v2.1.218 atau lebih baru.

Ketika Claude memperbaiki temuan yang dilaporkan nanti dalam sesi, itu melaporkannya lagi, dan Claude Code menandai setiap temuan dalam daftar temuan yang diperbarui sebagai diperbaiki, dilewati, atau tidak ada perubahan yang diperlukan.

<h3 id="what-the-review-reads-and-edits">
  Apa yang dibaca dan diedit ulasan
</h3>

Ulasan mengikuti `CLAUDE.md` Anda seperti sesi Claude Code apa pun, tetapi itu tidak membaca [`REVIEW.md`](#review-md). Ulasan latar belakang menerapkan pengeditan `--fix` di luar [checkpoint](/docs/id/checkpointing#subagent-edits-not-restored) sesi Anda, jadi `/rewind` tidak membatalkannya; gunakan git untuk mengembalikannya. Ketika ulasan [berjalan di latar depan](#run-in-the-foreground), itu mengedit pohon kerja Anda selama giliran Anda sendiri, jadi `/rewind` mengembalikan penyuntingannya seperti biasa.

<h3 id="tune-effort-and-arguments">
  Sesuaikan upaya dan argumen
</h3>

Lewatkan [tingkat upaya](/docs/id/model-config#adjust-effort-level) untuk menukar cakupan dengan kepercayaan diri. Pada `low` dan `medium`, ulasan hanya melaporkan temuan yang paling percaya diri, jadi Anda melihat lebih sedikit positif palsu; `high` hingga `max` memperluas cakupan dan mungkin mencakup temuan yang kurang pasti oleh ulasan.

Ketika Anda tidak mengetik level, ulasan menggunakan kembali level terakhir dari `low` hingga `max` yang Anda ketik, bahkan dalam sesi sebelumnya, dan Claude Code menunjukkan pemberitahuan seperti `Reusing high effort, the level you typed last time`. Ketik level, seperti `/code-review high`, untuk mengubah apa yang digunakan kembali oleh lintasan nanti; level yang Anda lewatkan dalam lintasan `-p` non-interaktif tidak memperbaruinya. `ultra` tidak memperbarui atau menggunakan level yang diingat. Jika Anda tidak pernah mengetik level, ulasan menggunakan upaya saat ini sesi. Sebelum v2.1.223, `/code-review` tanpa level selalu menggunakan upaya saat ini sesi.

Setelah tingkat upaya dan flag, Claude Code membaca sisa baris dalam salah satu dari dua cara:

* **Tanpa `ultra`**: semuanya yang tersisa adalah target ulasan, bahkan ketika itu dimulai dengan nama perintah lain. `/code-review /fix-issue 123` meninjau dengan `/fix-issue 123` sebagai teks target alih-alih memuat `/fix-issue` sebagai [skill bertumpuk](/docs/id/skills#pass-arguments-to-skills) kedua. Sebelum v2.1.218, perintah yang ditumpuk setelah `/code-review` diperluas sebagai skill-nya sendiri.
* **Dengan `ultra`**: Claude Code membaca satu kata sebagai cabang dasar atau nomor PR, dan mengubah teks yang lebih panjang yang tidak menamai cabang atau PR menjadi [catatan yang dilampirkan ke ulasan](/docs/id/ultrareview#pass-a-request-in-plain-words). `/code-review ultra check my auth changes` meninjau cabang saat ini Anda, dan Claude menghubungkan temuan dengan catatan Anda.

<h3 id="run-in-the-foreground">
  Jalankan di latar depan
</h3>

Ulasan berjalan di latar belakang secara default; sebelum v2.1.218, itu berjalan di dalam percakapan Anda. Itu berjalan di latar depan sebagai gantinya dalam kasus seperti ini:

* Anda menjalankan `/code-review` lagi sementara ulasan sebelumnya masih berlangsung
* Anda menjalankannya dalam mode non-interaktif, dengan flag `-p` atau Agent SDK; Claude Code menunggu ulasan dan menyertakan temuan dalam respons, kecuali untuk `ultra`, yang [meluncurkan ulasan cloud tanpa menunggu](#escalate-to-ultrareview)
* Anda menetapkan [`CLAUDE_CODE_DISABLE_BACKGROUND_TASKS`](/docs/id/env-vars) ke `1`, yang juga mematikan setiap fitur tugas latar belakang lainnya

<h3 id="let-claude-start-the-review">
  Biarkan Claude memulai ulasan
</h3>

Claude dapat memulai `/code-review` sendiri. Minta untuk meninjau perubahan Anda dalam bahasa biasa dan itu dapat menjalankan skill tanpa Anda mengetik perintah, dan [tugas terjadwal](/docs/id/scheduled-tasks) dengan `/code-review` sebagai prompt-nya menjalankan ulasan.

Tugas terjadwal tidak pernah meluncurkan [ulasan cloud](#escalate-to-ultrareview), jadi jadwalkan `/code-review` tanpa argumen `ultra`.

Untuk menghentikan Claude dan tugas terjadwal dari memulai ulasan sambil menjaga `/code-review` tersedia untuk Anda ketik, tambahkan entri [`skillOverrides`](/docs/id/skills#override-skill-visibility-from-settings) ke [file pengaturan](/docs/id/settings#where-settings-live) seperti `~/.claude/settings.json`:

```json theme={null}
{
  "skillOverrides": {
    "code-review": "user-invocable-only"
  }
}
```

Sebelum v2.1.246, Claude memulai `/code-review` sendiri hanya di mana flag fitur yang diambil dari Anthropic menyalakannya. Dalam [sesi yang tidak mengambil flag fitur](/docs/id/env-vars#features-that-need-feature-flag-fetching), `/code-review` berjalan hanya ketika Anda mengetiknya, dan `/code-review` terjadwal mencapai Claude sebagai teks biasa.

<h3 id="escalate-to-ultrareview">
  Eskalasi ke ultrareview
</h3>

`/code-review ultra --fix` menjalankan [ultrareview](/docs/id/ultrareview) yang lebih dalam di cloud, kemudian menerapkan temuannya ke pohon kerja Anda ketika mereka kembali dalam sesi Anda.

Ultrareview menggunakan cakupannya sendiri: cabang saat ini Anda terhadap cabang default repositori, ditambah perubahan yang tidak dilakukan dan staged dalam pohon kerja. Untuk perubahan yang tidak dilakukan ke file yang dinamai seperti kredensial atau kunci, seperti file `.env` dan `*.tfvars`, Claude Code mengikuti aturan untuk [mengunggah repositori lokal ke sesi cloud](/docs/id/claude-code-on-the-web#send-local-repositories-without-github). Lewatkan nama cabang, seperti `/code-review ultra develop`, untuk membandingkan terhadap dasar yang berbeda.

Ketika target adalah permintaan tarik `github.com`, Anda dapat memiliki Claude [posting temuan yang selesai ke PR](/docs/id/ultrareview#post-findings-to-the-pull-request) sebagai komentar dari akun GitHub Anda. Memerlukan Claude Code v2.1.227 atau lebih baru.

<Note>
  Ultrareview memerlukan autentikasi dengan akun claude.ai dan tidak tersedia di Amazon Bedrock, Agent Platform Google Cloud, atau Microsoft Foundry, atau untuk organisasi dengan Zero Data Retention diaktifkan. Ketika ultrareview tidak tersedia, `/code-review ultra` menjalankan ulasan lokal dalam sesi Anda sebagai gantinya.
</Note>

Untuk memulai ulasan cloud dari skrip atau CI, jalankan `claude -p '/code-review ultra'`. Claude Code meluncurkan ulasan dan mencetak tautan untuk melacaknya. Memerlukan Claude Code v2.1.218 atau lebih baru.

Ketika ulasan akan menagih [kredit penggunaan](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans), Claude Code berhenti sebelum meluncurkan, karena konfirmasi penagihan memerlukan sesi interaktif. Jalankan [subperintah `claude ultrareview`](/docs/id/ultrareview#run-ultrareview-non-interactively) sebagai gantinya; dengan menjalankannya, Anda menyetujui biaya.

Perintah ini dinamai `/simplify` sebelum v2.1.147, ketika itu menerapkan perbaikan secara default. `/simplify` menjalankan ulasan pembersihan terpisah yang menerapkan perbaikan tanpa berburu bug. Jika Anda membuat skrip `/simplify` untuk pencarian bug, beralih ke `/code-review --fix`.

<h2 id="related-resources">
  Sumber daya terkait
</h2>

* [Commands](/docs/id/commands): jalankan `/code-review` dalam sesi Claude Code lokal untuk memeriksa diff sebelum push
* [GitHub Actions](/docs/id/github-actions): jalankan Claude dalam alur kerja GitHub Actions Anda sendiri untuk otomasi khusus di luar ulasan kode
* [GitLab CI/CD](/docs/id/gitlab-ci-cd): integrasi Claude yang di-host sendiri untuk pipeline GitLab
* [Memory](/docs/id/memory): cara kerja file `CLAUDE.md` di seluruh Claude Code
* [Analytics](/docs/id/analytics): lacak penggunaan Claude Code di luar ulasan kode
* [How Anthropic secures its AI-native software development lifecycle](https://claude.com/blog/how-anthropic-secures-its-ai-native-software-development-lifecycle): bagaimana ulasan otomatis cocok sebagai salah satu lapisan proses pengembangan yang aman milik Anthropic
