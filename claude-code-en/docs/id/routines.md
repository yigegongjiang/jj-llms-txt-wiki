> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Otomatisasi pekerjaan dengan rutinitas

> Letakkan Claude Code pada autopilot. Tentukan rutinitas yang berjalan sesuai jadwal, dipicu oleh panggilan API, atau bereaksi terhadap peristiwa GitHub dari infrastruktur cloud yang dikelola Anthropic.

<Note>
  Rutinitas berada dalam pratinjau penelitian. Perilaku, batas, dan permukaan API mungkin berubah.
</Note>

Rutinitas adalah konfigurasi Claude Code yang disimpan: prompt, satu atau lebih repositori, dan serangkaian [konektor](/docs/id/mcp), dikemas sekali dan dijalankan secara otomatis. Rutinitas dijalankan pada infrastruktur cloud yang dikelola Anthropic, sehingga terus bekerja ketika laptop Anda ditutup.

Setiap rutinitas dapat memiliki satu atau lebih pemicu yang terpasang padanya:

* **Terjadwal**: berjalan dengan frekuensi berulang seperti per jam, malam hari, atau mingguan, atau sekali pada waktu masa depan tertentu
* **API**: dipicu sesuai permintaan dengan mengirim POST HTTP ke titik akhir per-rutinitas dengan token pembawa
* **GitHub**: berjalan secara otomatis sebagai respons terhadap peristiwa repositori seperti permintaan tarik atau rilis

Satu rutinitas dapat menggabungkan pemicu. Misalnya, rutinitas tinjauan PR dapat berjalan malam hari, dipicu dari skrip penyebaran, dan juga bereaksi terhadap setiap PR baru.

Rutinitas tersedia pada paket Pro, Max, Team, dan Enterprise dengan [Claude Code di web](/docs/id/claude-code-on-the-web) diaktifkan. Buat dan kelola di [claude.ai/code/routines](https://claude.ai/code/routines), atau dari CLI dengan `/schedule`.

Admin Team dan Enterprise dapat menonaktifkan rutinitas untuk semua anggota dengan toggle Routines di [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code). Ketika dinonaktifkan, rutinitas yang ada berhenti berjalan dan anggota tidak dapat membuat yang baru.

Halaman ini mencakup pembuatan rutinitas, mengonfigurasi setiap jenis pemicu, mengelola jalankan, dan bagaimana batas penggunaan berlaku.

<h2 id="example-use-cases">
  Contoh kasus penggunaan
</h2>

Setiap contoh memasangkan jenis pemicu dengan jenis pekerjaan yang cocok untuk rutinitas: tanpa pengawasan, dapat diulang, dan terikat pada hasil yang jelas.

**Pemeliharaan backlog.** Pemicu jadwal berjalan setiap malam kerja terhadap pelacak masalah Anda melalui konektor. Rutinitas membaca masalah yang dibuka sejak jalankan terakhir, menerapkan label, menetapkan pemilik berdasarkan area kode yang direferensikan, dan memposting ringkasan ke Slack sehingga tim memulai hari dengan antrian yang terawat.

**Triase peringatan.** Alat pemantauan Anda memanggil titik akhir API rutinitas ketika ambang batas kesalahan terlampaui, meneruskan badan peringatan sebagai `text`. Rutinitas menarik jejak tumpukan, menghubungkannya dengan komit terbaru di repositori, dan membuka permintaan tarik draf dengan perbaikan yang diusulkan dan tautan kembali ke peringatan. On-call meninjau PR alih-alih memulai dari terminal kosong.

**Tinjauan kode khusus.** Pemicu GitHub berjalan pada `pull_request.opened`. Rutinitas menerapkan daftar periksa tinjauan tim Anda sendiri, meninggalkan komentar sebaris untuk masalah keamanan, kinerja, dan gaya, dan menambahkan komentar ringkasan sehingga peninjau manusia dapat fokus pada desain alih-alih pemeriksaan mekanis.

**Verifikasi penyebaran.** Saluran pipa CD Anda memanggil titik akhir API rutinitas setelah setiap penyebaran produksi. Rutinitas menjalankan pemeriksaan asap terhadap build baru, memindai log kesalahan untuk regresi, dan memposting go atau no-go ke saluran rilis sebelum jendela penyebaran ditutup.

**Hanyut dokumentasi.** Pemicu jadwal berjalan mingguan. Rutinitas memindai PR yang digabungkan sejak jalankan terakhir, menandai dokumentasi yang mereferensikan API yang berubah, dan membuka PR pembaruan terhadap repositori dokumen untuk editor ditinjau.

**Port perpustakaan.** Pemicu GitHub berjalan pada `pull_request.closed` disaring ke PR yang digabungkan di satu repositori SDK. Rutinitas memindahkan perubahan ke SDK paralel dalam bahasa lain dan membuka PR yang cocok, menjaga kedua perpustakaan tetap sinkron tanpa manusia mengimplementasikan ulang setiap perubahan.

Bagian di bawah ini menjelaskan cara membuat rutinitas dan mengonfigurasi setiap jenis pemicu ini.

<h2 id="create-a-routine">
  Buat rutinitas
</h2>

Buat rutinitas dari web di [claude.ai/code/routines](https://claude.ai/code/routines), dari aplikasi Desktop, atau dari CLI. Ketiga permukaan menulis ke akun cloud yang sama, sehingga rutinitas yang Anda buat di satu tempat muncul di tempat lain segera. Di aplikasi Desktop, klik **Routines** di bilah sisi, lalu **New routine**, dan pilih **Remote**; memilih **Local** malah membuat [tugas terjadwal Desktop](/docs/id/desktop-scheduled-tasks), yang berjalan di mesin Anda daripada di cloud.

Formulir pembuatan menyiapkan prompt rutinitas, repositori, lingkungan, konektor, dan pemicu.

Rutinitas berjalan secara otonom sebagai sesi cloud Claude Code penuh: tidak ada pemilih mode izin dan tidak ada prompt persetujuan selama jalankan. Sesi dapat menjalankan perintah shell, menggunakan [skills](/docs/id/skills) yang berkomitmen pada repositori yang diklon, dan memanggil konektor apa pun yang Anda sertakan. Apa yang dapat dijangkau rutinitas ditentukan oleh repositori yang Anda pilih dan pengaturan push cabang mereka, [lingkungan](/docs/id/claude-code-on-the-web#the-cloud-environment) akses jaringan dan variabel, dan konektor yang Anda sertakan. Cakupan masing-masing ke apa yang benar-benar dibutuhkan rutinitas.

Rutinitas milik akun claude.ai individual Anda. Mereka tidak dibagikan dengan rekan kerja, dan mereka dihitung terhadap tunjangan jalankan harian akun Anda. Apa pun yang dilakukan rutinitas melalui identitas GitHub yang terhubung atau konektor muncul sebagai Anda: komit dan permintaan tarik membawa pengguna GitHub Anda, dan pesan Slack, tiket Linear, atau tindakan konektor lainnya menggunakan akun tertaut Anda untuk layanan tersebut.

<h3 id="create-from-the-web">
  Buat dari web
</h3>

<Steps>
  <Step title="Buka formulir pembuatan">
    Kunjungi [claude.ai/code/routines](https://claude.ai/code/routines) dan klik **New routine**.
  </Step>

  <Step title="Beri nama rutinitas dan tulis prompt">
    Berikan rutinitas nama deskriptif dan tulis prompt yang Claude jalankan setiap kali. Prompt adalah bagian paling penting: rutinitas berjalan secara otonom, jadi prompt harus mandiri dan eksplisit tentang apa yang harus dilakukan dan seperti apa kesuksesan itu.

    Input prompt mencakup pemilih model. Claude menggunakan model yang dipilih pada setiap jalankan.
  </Step>

  <Step title="Pilih repositori">
    Tambahkan satu atau lebih repositori GitHub untuk Claude kerjakan. Setiap repositori diklon di awal jalankan, dimulai dari cabang default. Claude membuat cabang dengan awalan `claude/` untuk perubahannya.
  </Step>

  <Step title="Pilih lingkungan">
    Pilih [lingkungan cloud](/docs/id/claude-code-on-the-web#the-cloud-environment) untuk rutinitas. Lingkungan mengontrol apa yang dapat diakses sesi cloud:

    * **Network access**: atur tingkat akses internet yang tersedia selama setiap jalankan
    * **Environment variables**: sediakan kunci API, token, atau rahasia lainnya yang dapat digunakan Claude
    * **Setup script**: instal dependensi dan alat yang dibutuhkan rutinitas. Hasilnya [di-cache](/docs/id/claude-code-on-the-web#environment-caching), jadi skrip tidak berjalan ulang pada setiap sesi

    Lingkungan **Default** disediakan dengan akses jaringan **Trusted**, yang memungkinkan [set default](/docs/id/claude-code-on-the-web#default-allowed-domains) registri paket, API penyedia cloud, registri kontainer, dan domain pengembangan umum, tetapi memblokir semuanya. Jika rutinitas Anda perlu menjangkau layanan Anda sendiri atau domain di luar daftar itu, edit [akses jaringan](/docs/id/claude-code-on-the-web#network-access) lingkungan sebelum menjalankan. Untuk menggunakan lingkungan terpisah, [buat satu](/docs/id/claude-code-on-the-web#configure-your-environment) terlebih dahulu.
  </Step>

  <Step title="Pilih pemicu">
    Di bawah **Select a trigger**, pilih bagaimana rutinitas dimulai. Anda dapat memilih satu jenis pemicu atau menggabungkan beberapa.

    <Tabs>
      <Tab title="Schedule">
        Pilih frekuensi preset untuk jalankan berulang, atau jadwalkan satu jalankan satu kali pada stempel waktu tertentu. Lihat [Add a schedule trigger](#add-a-schedule-trigger) untuk penanganan zona waktu, stagger, interval cron khusus, dan jalankan satu kali.
      </Tab>

      <Tab title="GitHub event">
        Pilih repositori, peristiwa untuk bereaksi, dan filter opsional. Lihat [Add a GitHub trigger](#add-a-github-trigger) untuk daftar lengkap peristiwa yang didukung dan bidang filter.
      </Tab>

      <Tab title="API">
        Pilih **API** di sini, lalu simpan rutinitas. URL dan token dihasilkan setelah rutinitas disimpan, karena bergantung pada ID rutinitas. Lihat [Add an API trigger](#add-an-api-trigger) untuk menyalin URL dan menghasilkan token.
      </Tab>
    </Tabs>
  </Step>

  <Step title="Tinjau konektor dan izin">
    Tab **Connectors** dan **Permissions** di bagian bawah formulir mengontrol apa yang dapat dijangkau rutinitas.

    Di bawah Connectors, semua [konektor MCP](/docs/id/mcp) yang terhubung disertakan secara default. Hapus yang tidak dibutuhkan rutinitas. Claude dapat menggunakan setiap alat dari konektor yang disertakan, termasuk penulisan, tanpa meminta izin selama jalankan.

    Di bawah Permissions, aktifkan **Allow unrestricted branch pushes** untuk repositori apa pun di mana Claude harus dapat push ke cabang yang ada daripada hanya yang dengan awalan `claude/`.
  </Step>

  <Step title="Buat rutinitas">
    Klik **Create**. Rutinitas muncul dalam daftar dan berjalan saat salah satu pemicunya cocok. Untuk memulai jalankan segera, klik **Run now** di halaman detail rutinitas.

    Setiap jalankan membuat sesi baru bersama sesi lainnya, di mana Anda dapat melihat apa yang dilakukan Claude, meninjau perubahan, dan membuat permintaan tarik.
  </Step>
</Steps>

<h3 id="create-from-the-cli">
  Buat dari CLI
</h3>

Jalankan `/schedule` dalam sesi apa pun untuk membuat rutinitas terjadwal secara percakapan. Anda juga dapat meneruskan deskripsi langsung, untuk rutinitas berulang seperti `/schedule daily PR review at 9am` atau satu kali seperti `/schedule clean up feature flag in one week`. Claude menjalani informasi yang sama yang dikumpulkan formulir web, lalu menyimpan rutinitas ke akun Anda.

Awal yang berhasil terlihat seperti percakapan: Claude mengajukan pertanyaan lanjutan tentang jadwal, repositori, dan prompt sebelum menyimpan. Jika Claude malah menjawab bahwa Anda perlu mengautentikasi atau bahwa Claude tidak dapat terhubung ke akun claude.ai jarak jauh Anda, tidak ada rutinitas yang dibuat; lihat [Troubleshooting](#troubleshooting).

`/schedule` di CLI hanya membuat rutinitas terjadwal. Untuk menambahkan pemicu API atau GitHub, edit rutinitas di web di [claude.ai/code/routines](https://claude.ai/code/routines).

CLI juga mendukung pengelolaan rutinitas yang ada. Jalankan `/schedule list` untuk melihat semua rutinitas, `/schedule update` untuk mengubah satu, atau `/schedule run` untuk memicunya segera.

<h2 id="configure-triggers">
  Konfigurasi pemicu
</h2>

Rutinitas dimulai ketika salah satu pemicunya cocok. Anda dapat melampirkan kombinasi apa pun dari pemicu jadwal, API, dan GitHub ke rutinitas yang sama, dan menambah atau menghapusnya kapan saja dari bagian **Select a trigger** formulir edit rutinitas.

<h3 id="add-a-schedule-trigger">
  Tambahkan pemicu jadwal
</h3>

Pemicu jadwal menjalankan rutinitas dengan frekuensi berulang, atau sekali pada waktu masa depan tertentu. Pilih frekuensi preset di bagian **Select a trigger**: per jam, harian, hari kerja, atau mingguan. Waktu dimasukkan dalam zona lokal Anda dan dikonversi secara otomatis, sehingga rutinitas berjalan pada waktu dinding jam itu terlepas dari di mana infrastruktur cloud berada.

Jalankan mungkin dimulai beberapa menit setelah waktu terjadwal karena stagger. Offset konsisten untuk setiap rutinitas.

Untuk interval khusus seperti setiap dua jam atau tanggal pertama setiap bulan, pilih preset terdekat dalam formulir, lalu jalankan `/schedule update` di CLI untuk menetapkan ekspresi cron spesifik. Interval minimum adalah satu jam; ekspresi yang berjalan lebih sering ditolak.

<h4 id="schedule-a-one-off-run">
  Jadwalkan jalankan sekali
</h4>

Jadwal sekali menjalankan rutinitas satu kali pada stempel waktu tertentu. Gunakan untuk mengingatkan diri sendiri nanti dalam minggu ini, untuk membuka PR pembersihan setelah rollout selesai, atau untuk memulai tugas tindak lanjut ketika perubahan upstream tiba. Setelah rutinitas dijalankan, rutinitas secara otomatis menonaktifkan dan UI web menandainya sebagai **Ran**. Untuk menjalankannya lagi, edit rutinitas dan atur waktu sekali baru.

<Note>
  Penjadwalan sekali dari CLI sedang diluncurkan secara bertahap dan mungkin belum tersedia di akun Anda. Jika `/schedule` hanya menawarkan jadwal berulang, buat jalankan sekali dari web di [claude.ai/code/routines](https://claude.ai/code/routines) sebagai gantinya.
</Note>

Buat jalankan sekali dari CLI dengan mendeskripsikan waktu dalam bahasa alami. Claude menyelesaikan frasa terhadap waktu saat ini dan mengonfirmasi stempel waktu absolut sebelum menyimpan.

```text theme={null}
/schedule tomorrow at 9am, summarize yesterday's merged PRs
```

```text theme={null}
/schedule in 2 weeks, open a cleanup PR that removes the feature flag
```

Konversi lokal-ke-UTC yang sama seperti jadwal berulang berlaku untuk stempel waktu sekali.

Jalankan sekali tidak dihitung terhadap batas jalankan rutinitas harian. Mereka mengonsumsi penggunaan langganan reguler paket Anda seperti sesi lainnya. Lihat [Usage and limits](#usage-and-limits) untuk detail.

<h3 id="add-an-api-trigger">
  Tambahkan pemicu API
</h3>

Pemicu API memberikan rutinitas titik akhir HTTP khusus. POSTing ke titik akhir dengan token pembawa rutinitas memulai sesi baru dan mengembalikan URL sesi. Gunakan ini untuk menghubungkan Claude Code ke sistem peringatan, saluran pipa penyebaran, alat internal, atau di mana pun Anda dapat membuat permintaan HTTP yang diautentikasi.

Pemicu API ditambahkan ke rutinitas yang ada dari web. CLI saat ini tidak dapat membuat atau mencabut token.

<Steps>
  <Step title="Buka rutinitas untuk diedit">
    Buka [claude.ai/code/routines](https://claude.ai/code/routines), klik rutinitas yang ingin Anda picu melalui API, lalu klik ikon pensil untuk membuka **Edit routine**.
  </Step>

  <Step title="Tambahkan pemicu API">
    Gulir ke bagian **Select a trigger** di bawah kotak **Instructions**, klik **Add another trigger**, dan pilih **API**.
  </Step>

  <Step title="Salin URL dan hasilkan token">
    Modal menampilkan URL untuk rutinitas ini bersama dengan contoh perintah curl. Salin URL, lalu klik **Generate token** dan salin token segera. Token ditampilkan sekali dan tidak dapat diambil nanti, jadi simpan di tempat yang aman seperti penyimpanan rahasia alat peringatan Anda.
  </Step>

  <Step title="Panggil titik akhir">
    Kirim token di header `Authorization: Bearer` ketika Anda POST ke URL. Bagian [Trigger a routine](#trigger-a-routine) di bawah menunjukkan contoh lengkap.
  </Step>
</Steps>

Setiap rutinitas memiliki token sendiri, dibatasi untuk memicu rutinitas itu saja. Untuk memutar atau mencabut, kembali ke modal yang sama dan klik **Regenerate** atau **Revoke**.

<h4 id="trigger-a-routine">
  Picu rutinitas
</h4>

Kirim permintaan POST ke titik akhir `/fire` dengan token pembawa di header `Authorization`. Badan permintaan menerima bidang `text` opsional untuk konteks spesifik jalankan seperti badan peringatan atau log yang gagal, diteruskan ke rutinitas bersama prompt yang disimpannya. Nilainya adalah teks freeform dan tidak diuraikan: jika Anda mengirim JSON atau muatan terstruktur lainnya, rutinitas menerimanya sebagai string literal.

Contoh di bawah memicu rutinitas dari shell. ID rutinitas dan token yang ditampilkan adalah placeholder: gantikan dengan URL dan token yang Anda salin saat [menambahkan pemicu API](#add-an-api-trigger), atau permintaan gagal dengan kesalahan autentikasi `401`:

```bash theme={null}
curl -X POST https://api.anthropic.com/v1/claude_code/routines/trig_01ABCDEFGHJKLMNOPQRSTUVW/fire \
  -H "Authorization: Bearer sk-ant-oat01-xxxxx" \
  -H "anthropic-beta: experimental-cc-routine-2026-04-01" \
  -H "anthropic-version: 2023-06-01" \
  -H "Content-Type: application/json" \
  -d '{"text": "Sentry alert SEN-4521 fired in prod. Stack trace attached."}'
```

Permintaan yang berhasil mengembalikan badan JSON dengan ID sesi baru dan URL:

```json theme={null}
{
  "type": "routine_fire",
  "claude_code_session_id": "session_01HJKLMNOPQRSTUVWXYZ",
  "claude_code_session_url": "https://claude.ai/code/session_01HJKLMNOPQRSTUVWXYZ"
}
```

Buka URL sesi di browser untuk menonton jalankan secara real-time, meninjau perubahan, atau melanjutkan percakapan secara manual.

<Warning>
  Titik akhir `/fire` dikirim di bawah header beta `experimental-cc-routine-2026-04-01`. Bentuk permintaan dan respons, batas laju, dan semantik token mungkin berubah saat fitur berada dalam pratinjau penelitian. Perubahan yang merusak dikirim di balik versi header beta bertanggal baru, dan dua versi header sebelumnya paling baru terus bekerja sehingga pemanggil memiliki waktu untuk bermigrasi.
</Warning>

<h4 id="api-reference">
  Referensi API
</h4>

Untuk referensi API lengkap, termasuk semua respons kesalahan, aturan validasi, dan batas bidang, lihat [Trigger a routine via API](https://platform.claude.com/docs/id/api/claude-code/routines-fire) dalam dokumentasi Platform Claude.

Titik akhir `/fire` tersedia untuk pengguna claude.ai saja dan bukan bagian dari permukaan API Platform Claude.

<h3 id="add-a-github-trigger">
  Tambahkan pemicu GitHub
</h3>

Pemicu GitHub memulai sesi baru secara otomatis ketika peristiwa yang cocok terjadi pada repositori yang terhubung. Setiap peristiwa yang cocok memulai sesinya sendiri.

<Note>
  Selama pratinjau penelitian, peristiwa webhook GitHub tunduk pada batas per jam per-rutinitas dan per-akun. Peristiwa di luar batas dijatuhkan sampai jendela direset. Lihat batas saat ini Anda di [claude.ai/code/routines](https://claude.ai/code/routines).
</Note>

Pemicu GitHub dikonfigurasi dari UI web saja.

<Steps>
  <Step title="Buka rutinitas untuk diedit">
    Buka [claude.ai/code/routines](https://claude.ai/code/routines), klik rutinitas, lalu klik ikon pensil untuk membuka **Edit routine**.
  </Step>

  <Step title="Tambahkan pemicu peristiwa GitHub">
    Gulir ke bagian **Select a trigger**, klik **Add another trigger**, dan pilih **GitHub event**.
  </Step>

  <Step title="Instal Aplikasi GitHub Claude">
    Aplikasi GitHub Claude harus diinstal pada repositori yang ingin Anda berlangganan. Penyiapan pemicu meminta Anda untuk menginstalnya jika belum.

    <Note>
      Menjalankan `/web-setup` di CLI memberikan akses repositori untuk kloning, tetapi tidak menginstal Aplikasi GitHub Claude dan tidak mengaktifkan pengiriman webhook. Pemicu GitHub memerlukan penginstalan Aplikasi GitHub Claude, yang diminta penyiapan pemicu untuk dilakukan.
    </Note>
  </Step>

  <Step title="Konfigurasi pemicu">
    Pilih repositori, pilih peristiwa dari daftar [peristiwa yang didukung](#supported-events), dan secara opsional tambahkan filter. Simpan pemicu.
  </Step>
</Steps>

<h4 id="supported-events">
  Peristiwa yang didukung
</h4>

Pemicu GitHub dapat berlangganan salah satu dari kategori peristiwa berikut. Dalam setiap kategori Anda dapat memilih tindakan spesifik, seperti `pull_request.opened`, atau bereaksi terhadap semua tindakan dalam kategori.

| Peristiwa    | Dipicu ketika                                                                          |
| :----------- | :------------------------------------------------------------------------------------- |
| Pull request | PR dibuka, ditutup, ditugaskan, diberi label, disinkronkan, atau diperbarui sebaliknya |
| Release      | Rilis dibuat, dipublikasikan, diedit, atau dihapus                                     |

<h4 id="filter-pull-requests">
  Filter permintaan tarik
</h4>

Gunakan filter untuk mempersempit permintaan tarik mana yang memulai sesi baru. Semua kondisi filter harus cocok agar rutinitas dipicu. Bidang filter yang tersedia adalah:

| Filter      | Cocok                           |
| :---------- | :------------------------------ |
| Author      | Nama pengguna GitHub penulis PR |
| Title       | Teks judul PR                   |
| Body        | Teks deskripsi PR               |
| Base branch | Cabang yang ditargetkan PR      |
| Head branch | Cabang yang berasal dari PR     |
| Labels      | Label yang diterapkan pada PR   |
| Is draft    | Apakah PR dalam status draf     |
| Is merged   | Apakah PR telah digabungkan     |

Setiap filter memasangkan bidang dengan operator: sama dengan, berisi, dimulai dengan, adalah salah satu, bukan salah satu, atau cocok regex.

Operator `matches regex` menguji seluruh nilai bidang, bukan substring di dalamnya. Untuk mencocokkan judul apa pun yang berisi `hotfix`, tulis `.*hotfix.*`. Tanpa `.*` di sekitarnya, filter hanya cocok dengan judul yang tepat `hotfix` tanpa apa pun sebelum atau sesudah. Untuk pencocokan substring literal tanpa sintaks regex, gunakan operator `contains` sebagai gantinya.

Beberapa contoh kombinasi filter:

* **Auth module review**: base branch `main`, head branch berisi `auth-provider`. Mengirim PR apa pun yang menyentuh autentikasi ke peninjau yang fokus.
* **Ready-for-review only**: is draft adalah `false`. Melewati draf sehingga rutinitas hanya berjalan ketika PR siap untuk ditinjau.
* **Label-gated backport**: labels termasuk `needs-backport`. Memicu rutinitas port-ke-cabang-lain hanya ketika pengelola memberi tag PR.

<h4 id="how-sessions-map-to-events">
  Bagaimana sesi memetakan ke peristiwa
</h4>

Setiap peristiwa GitHub yang cocok memulai sesi baru. Penggunaan ulang sesi di seluruh peristiwa tidak tersedia untuk rutinitas yang dipicu GitHub, jadi dua pembaruan PR menghasilkan dua sesi independen.

<h2 id="manage-routines">
  Kelola rutinitas
</h2>

Klik rutinitas dalam daftar untuk membuka halaman detailnya. Halaman detail menampilkan repositori rutinitas, konektor, prompt, jadwal, token API, pemicu GitHub, dan daftar jalankan masa lalu.

<h3 id="view-and-interact-with-runs">
  Lihat dan berinteraksi dengan jalankan
</h3>

Klik jalankan apa pun untuk membukanya sebagai sesi penuh. Dari sana Anda dapat melihat apa yang dilakukan Claude, meninjau perubahan, membuat permintaan tarik, atau melanjutkan percakapan. Setiap sesi jalankan bekerja seperti sesi lainnya: gunakan menu dropdown di sebelah judul sesi untuk mengganti nama, mengarsipkan, atau menghapusnya.

<Note>
  Status hijau dalam daftar jalankan berarti sesi dimulai dan keluar tanpa kesalahan infrastruktur. Ini tidak berarti tugas dalam prompt Anda berhasil. Buka jalankan untuk membaca transkrip dan konfirmasi apa yang sebenarnya dilakukan Claude. Permintaan jaringan yang diblokir, alat konektor yang hilang, dan kegagalan tingkat tugas semuanya muncul di sana daripada di indikator status.
</Note>

<h3 id="edit-and-control-routines">
  Edit dan kontrol rutinitas
</h3>

Dari halaman detail rutinitas Anda dapat:

* Klik **Run now** untuk memulai jalankan segera tanpa menunggu waktu terjadwal berikutnya.
* Gunakan toggle di bagian **Repeats** untuk menjeda atau melanjutkan jadwal. Rutinitas yang dijeda menyimpan konfigurasi mereka tetapi tidak berjalan sampai Anda mengaktifkan kembali.
* Klik ikon pensil untuk membuka **Edit routine** dan ubah nama, prompt, repositori, lingkungan, konektor, atau pemicu rutinitas apa pun. Bagian **Select a trigger** adalah tempat Anda menambah atau menghapus jadwal, token API, dan pemicu peristiwa GitHub.
* Klik ikon hapus untuk menghapus rutinitas. Sesi masa lalu yang dibuat oleh rutinitas tetap dalam daftar sesi Anda.

<h3 id="repositories-and-branch-permissions">
  Repositori dan izin cabang
</h3>

Rutinitas memerlukan akses GitHub untuk mengklon repositori. Ketika Anda membuat rutinitas dari CLI dengan `/schedule`, Claude memeriksa apakah akun Anda memiliki GitHub yang terhubung dan meminta Anda menjalankan `/web-setup` jika tidak. Lihat [GitHub authentication options](/docs/id/claude-code-on-the-web#github-authentication-options) untuk dua cara memberikan akses.

Setiap repositori yang Anda tambahkan diklon pada setiap jalankan. Claude dimulai dari cabang default repositori kecuali prompt Anda menentukan sebaliknya.

Secara default, Claude hanya dapat push ke cabang dengan awalan `claude/`. Ini mencegah rutinitas secara tidak sengaja memodifikasi cabang yang dilindungi atau jangka panjang. Untuk menghapus pembatasan ini untuk repositori spesifik, aktifkan **Allow unrestricted branch pushes** untuk repositori tersebut saat membuat atau mengedit rutinitas.

<h3 id="connectors">
  Konektor
</h3>

Rutinitas dapat menggunakan konektor MCP yang terhubung untuk membaca dari dan menulis ke layanan eksternal selama setiap jalankan. Misalnya, rutinitas yang melakukan triase permintaan dukungan mungkin membaca dari saluran Slack dan membuat masalah di Linear.

Konektor adalah [integrasi claude.ai](/docs/id/mcp#use-mcp-servers-from-claude-ai) di akun Anda. Server MCP yang Anda tambahkan secara lokal di CLI dengan `claude mcp add` disimpan di mesin Anda daripada akun claude.ai Anda, jadi mereka tidak muncul dalam daftar konektor. Untuk menggunakan salah satu server tersebut dalam rutinitas, tambahkan sebagai konektor di [claude.ai/customize/connectors](https://claude.ai/customize/connectors), atau deklarasikan dalam [`.mcp.json`](/docs/id/mcp#project-scope) yang berkomitmen sehingga itu adalah bagian dari repositori yang diklon.

Ketika Anda membuat rutinitas, semua konektor yang saat ini terhubung disertakan secara default. Hapus yang tidak diperlukan untuk membatasi alat mana yang dapat diakses Claude selama jalankan. Anda juga dapat menambahkan konektor langsung dari formulir rutinitas.

Untuk mengelola atau menambahkan konektor di luar formulir rutinitas, kunjungi **Settings > Connectors** di claude.ai atau gunakan `/schedule update` di CLI.

<h3 id="environments-and-network-access">
  Lingkungan dan akses jaringan
</h3>

Setiap rutinitas berjalan dalam [lingkungan cloud](/docs/id/claude-code-on-the-web#the-cloud-environment) yang mengontrol akses jaringan, variabel lingkungan, dan skrip penyiapan. Rutinitas mewarisi kebijakan jaringan lingkungan pada setiap jalankan.

Lingkungan **Default** menggunakan akses jaringan **Trusted**: [daftar allowlist default](/docs/id/claude-code-on-the-web#default-allowed-domains) dari registri paket, API penyedia cloud, registri kontainer, dan domain pengembangan umum dapat dijangkau, tetapi domain arbitrer tidak. Permintaan keluar ke host lain gagal dengan `403` dan `x-deny-reason: host_not_allowed`. Lalu lintas konektor MCP dirutekan melalui server Anthropic, jadi konektor yang Anda tambahkan ke rutinitas bekerja tanpa menambahkan host mereka ke **Allowed domains**. Hapus konektor apa pun yang tidak Anda butuhkan di bawah [Konektor](#connectors).

Untuk memungkinkan domain tambahan:

<Steps>
  <Step title="Buka rutinitas untuk diedit">
    Pada halaman detail rutinitas, klik ikon pensil untuk membuka **Edit routine**.
  </Step>

  <Step title="Buka pemilih lingkungan">
    Di bawah kotak **Instructions**, pilih ikon cloud yang menampilkan nama lingkungan Anda, seperti **Default**.
  </Step>

  <Step title="Buka pengaturan lingkungan">
    Arahkan ke lingkungan dalam daftar dan klik ikon pengaturan yang muncul di sebelah kanan.
  </Step>

  <Step title="Ubah tingkat akses jaringan">
    Dalam dialog **Update cloud environment**, ubah **Network access** menjadi **Custom** dan masukkan domain Anda di **Allowed domains**. Periksa **Also include default list of common package managers** untuk menyimpan [daftar allowlist default](/docs/id/claude-code-on-the-web#default-allowed-domains) bersama domain kustom Anda. Pilih **Full** sebagai gantinya untuk akses tanpa batas.
  </Step>

  <Step title="Simpan">
    Klik **Save changes**. Kebijakan baru berlaku dari jalankan berikutnya.
  </Step>
</Steps>

Lihat [Network access](/docs/id/claude-code-on-the-web#network-access) untuk detail tentang tingkat akses dan daftar allowlist default.

<h2 id="usage-and-limits">
  Penggunaan dan batas
</h2>

Rutinitas mengurangi penggunaan langganan dengan cara yang sama seperti sesi interaktif. Selain batas langganan standar, rutinitas memiliki batas harian tentang berapa banyak jalankan yang dapat dimulai per akun. Lihat konsumsi saat ini dan jalankan rutinitas harian yang tersisa di [claude.ai/code/routines](https://claude.ai/code/routines) atau [claude.ai/settings/usage](https://claude.ai/settings/usage).

Ketika rutinitas mencapai batas harian atau batas penggunaan langganan Anda, organisasi dengan penggunaan ekstra yang diaktifkan dapat terus menjalankan rutinitas pada overage terukur. Tanpa penggunaan ekstra, jalankan tambahan ditolak sampai jendela direset. Aktifkan penggunaan ekstra dari **Settings > Billing** di claude.ai.

Jalankan sekali saja tidak dihitung terhadap batas jalankan rutinitas harian. Mereka mengurangi penggunaan langganan reguler Anda seperti sesi lainnya, tetapi mereka dikecualikan dari tunjangan jalankan rutinitas harian per akun.

<h2 id="troubleshooting">
  Pemecahan masalah
</h2>

<h3 id="/schedule-returns-unknown-command">
  `/schedule` menampilkan "Unknown command"
</h3>

CLI menyembunyikan `/schedule` ketika salah satu persyaratannya tidak terpenuhi: menu perintah menampilkan `No commands match "/schedule"` saat Anda mengetik, dan mengirimkannya mengembalikan `Unknown command: /schedule`. Penyebabnya biasanya salah satu dari berikut ini:

* Anda diautentikasi dengan Console API key atau penyedia cloud seperti Amazon Bedrock, Google Cloud's Agent Platform, atau Microsoft Foundry. `/schedule` memerlukan login langganan claude.ai. Jika `ANTHROPIC_API_KEY` atau `ANTHROPIC_AUTH_TOKEN` diatur di shell Anda, atau `apiKeyHelper` diatur di `settings.json`, hapus terlebih dahulu, karena ini memiliki prioritas lebih tinggi daripada login claude.ai
* `DISABLE_TELEMETRY`, `DO_NOT_TRACK`, `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`, atau `DISABLE_GROWTHBOOK` diatur di lingkungan shell Anda atau di blok `env` dari [file `settings.json`](/docs/id/settings#available-settings). Ini menonaktifkan pengambilan feature-flag, yang `/schedule` bergantung padanya
* Anda berada di dalam sesi Claude Code di web. Kelola rutinitas dari [UI web](https://claude.ai/code/routines) sebagai gantinya

Anda selalu dapat membuat dan mengelola rutinitas di [claude.ai/code/routines](https://claude.ai/code/routines) terlepas dari bagaimana CLI dikonfigurasi.

<h3 id="/schedule-asks-you-to-authenticate">
  `/schedule` meminta Anda untuk diautentikasi
</h3>

Jika `/schedule` berjalan tetapi Claude merespons bahwa Anda perlu diautentikasi dengan akun claude.ai terlebih dahulu, CLI tidak memiliki login claude.ai yang tersimpan. Akun API tidak didukung untuk rutinitas. Jalankan `/login`, masuk dengan akun claude.ai Anda, kemudian jalankan `/schedule` lagi.

<h3 id="routines-are-disabled-by-your-organization’s-policy">
  "Rutinitas dinonaktifkan oleh kebijakan organisasi Anda"
</h3>

Pemilik di organisasi Team atau Enterprise Anda mungkin telah mematikan toggle **Routines** di [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code). Ini adalah pengaturan organisasi sisi server, jadi tidak dapat ditimpa dari konfigurasi lokal Anda. Hubungi Pemilik untuk meminta agar rutinitas diaktifkan untuk organisasi Anda.

<h2 id="related-resources">
  Sumber daya terkait
</h2>

* [`/loop` and in-session scheduling](/docs/id/scheduled-tasks): jadwalkan tugas lokal dalam sesi CLI terbuka
* [Desktop scheduled tasks](/docs/id/desktop-scheduled-tasks): tugas terjadwal lokal yang berjalan di mesin Anda dengan akses ke file lokal
* [Cloud environment](/docs/id/claude-code-on-the-web#the-cloud-environment): konfigurasi lingkungan runtime untuk sesi cloud
* [MCP connectors](/docs/id/mcp): hubungkan layanan eksternal seperti Slack, Linear, dan Google Drive
* [GitHub Actions](/docs/id/github-actions): jalankan Claude dalam saluran pipa CI pada peristiwa repositori
