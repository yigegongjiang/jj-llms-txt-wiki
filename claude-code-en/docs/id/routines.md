> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Otomatisasi pekerjaan dengan rutinitas

> Letakkan Claude Code pada autopilot. Tentukan rutinitas yang berjalan sesuai jadwal, dipicu oleh panggilan API, atau bereaksi terhadap peristiwa GitHub dari infrastruktur cloud.

<Note>
  Rutinitas berada dalam pratinjau penelitian. Perilaku, batas, dan permukaan API mungkin berubah.
</Note>

Rutinitas adalah konfigurasi Claude Code yang disimpan: prompt, satu atau lebih repositori, dan serangkaian [konektor](/docs/id/mcp), dikemas sekali dan dijalankan secara otomatis. Rutinitas dijalankan pada infrastruktur cloud yang dikelola Anthropic, atau pada [lingkungan self-hosted](/docs/id/self-hosted-environments) organisasi Anda ketika dialihkan ke sana, sehingga terus bekerja ketika laptop Anda ditutup.

Setiap rutinitas dapat memiliki satu atau lebih pemicu yang terpasang padanya:

* **Terjadwal**: berjalan dengan frekuensi berulang seperti per jam, malam hari, atau mingguan, atau sekali pada waktu masa depan tertentu
* **API**: dipicu sesuai permintaan dengan mengirim POST HTTP ke titik akhir per-rutinitas dengan token pembawa
* **GitHub**: berjalan secara otomatis sebagai respons terhadap peristiwa repositori seperti permintaan tarik atau rilis

Satu rutinitas dapat menggabungkan pemicu. Misalnya, rutinitas tinjauan PR dapat berjalan malam hari, dipicu dari skrip penyebaran, dan juga bereaksi terhadap setiap PR baru.

Rutinitas tersedia pada paket Pro, Max, Team, dan Enterprise. Buat dan kelola di [claude.ai/code/routines](https://claude.ai/code/routines), atau dari CLI dengan `/schedule`.

Pemilik Team dan Enterprise dapat menonaktifkan rutinitas untuk semua anggota dengan toggle Routines di [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code). Ketika dinonaktifkan, rutinitas yang ada berhenti berjalan dan anggota tidak dapat membuat yang baru.

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

<h2 id="create-a-routine">
  Buat rutinitas
</h2>

Buat rutinitas dari web di [claude.ai/code/routines](https://claude.ai/code/routines), dari aplikasi Desktop, atau dari CLI. Ketiga permukaan menulis ke akun cloud yang sama, sehingga rutinitas yang Anda buat di satu tempat muncul di tempat lain segera. Di aplikasi Desktop, klik **Routines** di bilah sisi atau di menu **More** bilah sisi, lalu **New routine**, dan pilih **Cloud**; memilih **Local** malah membuat [tugas terjadwal Desktop](/docs/id/desktop-scheduled-tasks), yang berjalan di mesin Anda daripada di cloud.

Formulir pembuatan menyiapkan prompt rutinitas, repositori, lingkungan, konektor, dan pemicu.

Rutinitas berjalan secara otonom sebagai sesi cloud Claude Code penuh: tidak ada pemilih mode izin, dan sesi menjalankan perintah shell, menggunakan [skills](/docs/id/skills) yang berkomitmen pada repositori yang diklon, dan memanggil konektor apa pun yang Anda sertakan, semuanya tanpa berhenti untuk persetujuan selain beberapa tindakan [artifact](/docs/id/artifacts).

Apa yang dapat dijangkau rutinitas ditentukan oleh repositori yang Anda pilih, [lingkungan](/docs/id/cloud-environments) akses jaringan dan variabel, dan konektor yang Anda sertakan. Cakupan masing-masing ke apa yang benar-benar dibutuhkan rutinitas.

Ketika jadwal rutinitas atau **Run now** memulai jalankan, Claude menerbitkan kembali artifact yang ada tanpa bertanya hanya ketika semua hal ini berlaku:

* Anda dapat mengedit artifact dan itu milik organisasi Anda sendiri
* Artifact tidak dibagikan secara publik, dan tidak dibagikan dengan orang-orang tertentu atau organisasi Anda dengan versi terbaru dipilih sebagai versi yang dilihat penonton
* Penerbitan hanya membawa halaman, tanpa file pendukung atau apa pun yang ditambahkan, dan tidak memaksa versi yang lebih baru
* Halaman tidak memiliki hibah yang melampaui halaman, seperti [panggilan konektor](/docs/id/artifacts#pull-live-data-with-mcp-connectors)

Dalam setiap kasus lain, termasuk menerbitkan artifact baru, Claude bertanya terlebih dahulu. Ketika pekerjaan rutinitas adalah menjaga halaman tetap terkini, berikan artifact yang sudah Anda terbitkan.

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

    Ketika pemicu menyala, sesi menerima prompt rutinitas yang disimpan sebagai tugas yang ditugaskan dan melaksanakannya, daripada memperlakukannya sebagai konten yang tidak dipercaya yang tiba di tengah percakapan. Pemicu hanya membuktikan bahwa prompt disimpan sebelumnya oleh sesi yang berwenang di akun Anda, jadi prompt yang dipecat bukan masukan pengguna langsung dan tidak dapat bertindak sebagai persetujuan atau persetujuan untuk tindakan selama jalankan. Konten yang sesi ambil selama jalankan mempertahankan penanganannya yang normal. Sebelum v2.1.213, sesi menerima prompt yang sama dibingkai sebagai notifikasi latar belakang yang tidak dipercaya dan dapat menolak untuk bertindak atasnya.

    Input prompt mencakup pemilih model. Claude menggunakan model yang dipilih pada setiap jalankan.
  </Step>

  <Step title="Pilih repositori">
    Tambahkan satu atau lebih repositori GitHub untuk Claude kerjakan. Setiap repositori diklon di awal jalankan, dimulai dari cabang default. Claude membuat cabang dengan awalan `claude/` untuk perubahannya.
  </Step>

  <Step title="Pilih lingkungan">
    Pilih [lingkungan cloud](/docs/id/cloud-environments) untuk rutinitas. Lingkungan mengontrol apa yang dapat diakses sesi cloud:

    * **Network access**: atur tingkat akses internet yang tersedia selama setiap jalankan
    * **Environment variables**: sediakan nilai yang dapat digunakan Claude selama setiap jalankan. Mereka [terlihat oleh siapa pun yang menggunakan lingkungan](/docs/id/cloud-environments#what-carries-over-from-your-setup), jadi pada paket Pro dan Max, simpan kunci untuk API yang Claude panggil selama jalankan sebagai [API credentials](/docs/id/cloud-environments#add-api-credentials) sebagai gantinya. Bagian itu juga mencantumkan permintaan yang tidak pernah mendapatkan kredensial
    * **Setup script**: instal dependensi dan alat yang dibutuhkan rutinitas. Hasilnya [di-cache](/docs/id/cloud-environments#environment-caching), jadi skrip tidak berjalan ulang pada setiap sesi

    Lingkungan **Default** disediakan dengan akses jaringan **Trusted**, yang memungkinkan hanya [daftar allowlist default](/docs/id/cloud-environments#default-allowed-domains) registri paket, API penyedia cloud, registri kontainer, dan domain pengembangan umum melalui jaringan sesi. Konektor yang Anda tambahkan ke rutinitas menjangkau layanan mereka melalui server Anthropic, jadi mereka tidak memerlukan perubahan allowlist. Jika rutinitas Anda perlu menjangkau layanan Anda sendiri secara langsung, atau domain di luar daftar itu, edit [akses jaringan](/docs/id/cloud-environments#network-access) lingkungan sebelum menjalankan. Untuk menggunakan lingkungan terpisah, [buat satu](/docs/id/cloud-environments#configure-your-environment) terlebih dahulu.
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

  <Step title="Tinjau konektor">
    Di bawah **Connectors** di bagian bawah formulir, semua [konektor MCP](/docs/id/mcp) yang terhubung disertakan secara default. Hapus yang tidak dibutuhkan rutinitas: Claude dapat menggunakan setiap alat dari konektor yang disertakan, termasuk penulisan, tanpa meminta izin selama jalankan.
  </Step>

  <Step title="Buat rutinitas">
    Klik **Create**. Rutinitas muncul dalam daftar dan berjalan saat salah satu pemicunya cocok. Untuk memulai jalankan segera, klik **Run now** di halaman detail rutinitas.

    Setiap jalankan membuat sesi baru bersama sesi lainnya, di mana Anda dapat melihat apa yang dilakukan Claude, meninjau perubahan, dan membuat permintaan tarik.
  </Step>
</Steps>

<h3 id="create-from-the-cli">
  Buat dari CLI
</h3>

Jalankan `/schedule` dalam sesi apa pun untuk membuat rutinitas terjadwal secara percakapan. Anda juga dapat meneruskan deskripsi langsung, untuk rutinitas berulang seperti `/schedule daily PR review at 9am` atau satu kali seperti `/schedule clean up feature flag in one week`. Claude menjalani informasi yang sama yang dikumpulkan formulir web, lalu menyimpan rutinitas ke akun Anda. Perintah juga tersedia di bawah alias `/routines`.

Awal yang berhasil terlihat seperti percakapan: Claude mengajukan pertanyaan lanjutan tentang jadwal, repositori, dan prompt sebelum menyimpan. Jika Claude malah menjawab bahwa Anda perlu mengautentikasi atau bahwa Claude tidak dapat terhubung ke akun claude.ai jarak jauh Anda, tidak ada rutinitas yang dibuat; lihat [Troubleshooting](#troubleshooting).

`/schedule` di CLI membuat rutinitas terjadwal. Untuk menambahkan pemicu API, edit rutinitas di web di [claude.ai/code/routines](https://claude.ai/code/routines). Anda dapat menambahkan [pemicu GitHub](#add-a-github-trigger) dari web atau dari CLI. Jalur CLI memerlukan Claude Code v2.1.225 atau lebih baru.

Rutinitas tanpa pemicu jadwal, seperti yang dimulai hanya oleh panggilan API atau peristiwa GitHub, tidak memiliki waktu jalankan berikutnya, dan CLI tidak menunjukkan apa pun ketika Claude menyimpan atau memperbarui. Sebelum v2.1.211, CLI melaporkan waktu jalankan berikutnya pada tahun 1 untuk rutinitas ini.

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

Buat jalankan sekali dari CLI dengan mendeskripsikan waktu dalam bahasa alami. Claude menyelesaikan frasa terhadap waktu saat ini dan mengonfirmasi stempel waktu absolut sebelum menyimpan.

```text theme={null}
/schedule tomorrow at 9am, summarize yesterday's merged PRs
```

```text theme={null}
/schedule in 2 weeks, open a cleanup PR that removes the feature flag
```

Konversi lokal-ke-UTC yang sama seperti jadwal berulang berlaku untuk stempel waktu sekali.

Jalankan sekali tidak dihitung terhadap batas jalankan rutinitas harian. Lihat [Usage and limits](#usage-and-limits) untuk detail.

<h3 id="add-an-api-trigger">
  Tambahkan pemicu API
</h3>

Pemicu API memberikan rutinitas titik akhir HTTP khusus. POSTing ke titik akhir dengan token pembawa rutinitas memulai sesi baru dan mengembalikan URL sesi. Gunakan ini untuk menghubungkan Claude Code ke sistem peringatan, saluran pipa penyebaran, alat internal, atau di mana pun Anda dapat membuat permintaan HTTP yang diautentikasi.

Pemicu API ditambahkan ke rutinitas yang ada dari web. CLI saat ini tidak dapat membuat atau mencabut token.

<Steps>
  <Step title="Buka rutinitas untuk diedit">
    Buka [claude.ai/code/routines](https://claude.ai/code/routines), klik rutinitas yang ingin Anda picu melalui API, lalu buka menu di sebelah nama rutinitas dan pilih **Edit**.
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

Nilai `text` tidak mencapai rutinitas sebagai pesan telanjang. Nilai tersebut tiba dibungkus dalam blok `<routine-fire-payload>` yang memberi labelnya sebagai data yang tidak dipercaya dan memberitahu Claude untuk tidak mengikuti instruksi di dalamnya kecuali prompt rutinitas sendiri mengatakan demikian. Pembungkus yang sama berlaku untuk teks yang disediakan dengan **Run now** di UI web.

Ini berarti prompt yang disimpan rutinitas harus memilih untuk bertindak pada teks api: tulis prompt untuk mereferensikan payload secara eksplisit, misalnya "Investigasi peringatan yang dijelaskan dalam blok routine-fire-payload", atau rutinitas memperlakukan teks sebagai konteks inert. Siapa pun yang memegang token pembawa dapat mengirim `text`, jadi pembungkus membuat teks api dari token yang bocor tiba berlabel sebagai data yang tidak dipercaya daripada sebagai instruksi langsung ke rutinitas Anda.

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

Untuk referensi API lengkap, termasuk semua respons kesalahan, aturan validasi, dan batas bidang, lihat [Trigger a routine via API](https://platform.claude.com/docs/en/api/claude-code/routines-fire) dalam dokumentasi Platform Claude.

Titik akhir `/fire` tersedia untuk pengguna claude.ai saja dan bukan bagian dari permukaan API Platform Claude.

<h3 id="add-a-github-trigger">
  Tambahkan pemicu GitHub
</h3>

Pemicu GitHub memulai sesi baru secara otomatis ketika peristiwa yang cocok terjadi pada repositori yang terhubung. Claude Code tidak menggunakan kembali sesi di seluruh peristiwa, jadi dua pembaruan PR menghasilkan dua sesi independen.

<Note>
  Selama pratinjau penelitian, peristiwa webhook GitHub tunduk pada batas per jam per-rutinitas dan per-akun. Peristiwa di luar batas dijatuhkan sampai jendela direset. Lihat batas saat ini Anda di [claude.ai/code/routines](https://claude.ai/code/routines).
</Note>

Aplikasi GitHub Claude harus diinstal pada repositori yang ingin Anda berlangganan, permukaan apa pun yang Anda konfigurasi pemicu darinya.

* Konfigurasi pemicu GitHub dari UI web, yang meminta Anda untuk menginstal aplikasi saat hilang. Ikuti langkah-langkah di bawah untuk mengonfigurasi satu di web.
* Dari CLI, instal aplikasi dari [halaman Aplikasi GitHub](https://github.com/apps/claude) terlebih dahulu, lalu minta Claude untuk melampirkan pemicu GitHub ke rutinitas yang ada, misalnya `/schedule add a GitHub trigger to my nightly review for pull requests opened in acme/webapp`. Jalur CLI memerlukan Claude Code v2.1.225 atau lebih baru. Ketika Claude menambahkan pemicu, Claude membalas dengan tautan ke rutinitas yang dipicu pemicu.

<Steps>
  <Step title="Buka rutinitas untuk diedit">
    Buka [claude.ai/code/routines](https://claude.ai/code/routines), klik rutinitas, lalu buka menu di sebelah nama rutinitas dan pilih **Edit**.
  </Step>

  <Step title="Tambahkan pemicu peristiwa GitHub">
    Gulir ke bagian **Select a trigger**, klik **Add another trigger**, dan pilih **GitHub event**.

    <Note>
      Menjalankan `/web-setup` di CLI memberikan akses repositori untuk kloning, tetapi tidak menginstal Aplikasi GitHub Claude dan tidak mengaktifkan pengiriman webhook.
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

Gunakan filter untuk mempersempit permintaan tarik mana yang memulai sesi baru. Semua kondisi filter harus cocok untuk rutinitas dipicu. Bidang filter yang tersedia adalah:

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

* Klik **Run now** untuk memulai jalankan segera tanpa menunggu waktu terjadwal berikutnya. Anda dapat secara opsional menyediakan teks khusus jalankan, yang mencapai rutinitas dengan cara yang sama seperti bidang `text` pemicu API.
* Gunakan toggle di bagian atas halaman untuk menjeda atau melanjutkan jadwal. Rutinitas yang dijeda menyimpan konfigurasi mereka tetapi tidak berjalan sampai Anda mengaktifkan kembali.
* Buka menu di sebelah nama rutinitas dan pilih **Edit** untuk mengubah nama, prompt, repositori, lingkungan, konektor, atau pemicu rutinitas apa pun. Bagian **Select a trigger** adalah tempat Anda menambah atau menghapus jadwal, token API, dan pemicu peristiwa GitHub.
* Buka menu yang sama dan pilih **Delete** untuk menghapus rutinitas.

<h3 id="manage-routines-from-the-cli">
  Kelola rutinitas dari CLI
</h3>

CLI mendukung pengelolaan rutinitas yang ada. Jalankan `/schedule list` untuk melihat semua rutinitas, `/schedule update` untuk mengubah satu, atau `/schedule run` untuk memicunya segera.

Anda juga dapat menanyakan tentang riwayat jalankan rutinitas, misalnya `/schedule why did my nightly review do nothing this morning?`. Claude mencantumkan jalankan terbaru rutinitas dengan status mereka dan tautan untuk [membuka setiap jalankan di web](#view-and-interact-with-runs), dan membaca log jalankan untuk menjelaskan apa yang terjadi, termasuk kesalahan alat, penolakan izin, dan hasil akhir. Memerlukan Claude Code v2.1.227 atau lebih baru.

<h3 id="repositories-and-branch-permissions">
  Repositori dan izin cabang
</h3>

Rutinitas memerlukan akses GitHub untuk mengklon repositori. Ketika Anda membuat rutinitas dari CLI dengan `/schedule`, Claude memeriksa apakah akun Anda memiliki akses GitHub untuk repositori tempat Anda menjalankannya dan, jika tidak, menambahkan catatan penyiapan yang menamai cara memberikan akses. Lihat [GitHub authentication options](/docs/id/claude-code-on-the-web#github-authentication-options) untuk dua cara memberikan akses.

Jika koneksi GitHub Anda hilang atau kedaluwarsa ketika jalankan akan dilakukan, rutinitas melewati jalankan hingga Anda terhubung kembali, hingga 72 jam. Hubungkan kembali GitHub dalam jendela tersebut dan rutinitas akan dilanjutkan dengan sendirinya. Setelah 72 jam tanpa koneksi, rutinitas mati, dan Anda menghidupkannya kembali setelah terhubung kembali ke GitHub.

Setiap repositori yang Anda tambahkan diklon pada setiap jalankan. Claude dimulai dari cabang default repositori kecuali prompt Anda menentukan sebaliknya.

Claude mendorong pekerjaannya ke cabang dengan awalan `claude/`, yang selalu diterima. Ketika prompt Anda mengarahkan Claude untuk mendorong ke cabang lain, Claude Code memeriksa dorongan terlebih dahulu dan menolaknya jika salah satu dari berikut ini benar:

* Cabang dilindungi di GitHub
* Seseorang lain memiliki permintaan tarik terbuka dari cabang tersebut
* Cabang membawa komit yang ditulis oleh seseorang selain Anda

<h3 id="connectors">
  Konektor
</h3>

Rutinitas dapat menggunakan konektor MCP yang terhubung untuk membaca dari dan menulis ke layanan eksternal selama setiap jalankan. Misalnya, rutinitas yang melakukan triase permintaan dukungan mungkin membaca dari saluran Slack dan membuat masalah di Linear.

Konektor adalah [integrasi claude.ai](/docs/id/mcp#use-mcp-servers-from-claude-ai) di akun Anda. Server MCP yang Anda tambahkan secara lokal di CLI dengan `claude mcp add` disimpan di mesin Anda daripada akun claude.ai Anda, jadi mereka tidak muncul dalam daftar konektor. Untuk menggunakan salah satu server tersebut dalam rutinitas, tambahkan sebagai konektor di [claude.ai/customize/connectors](https://claude.ai/customize/connectors). Untuk rutinitas dengan satu repositori, Anda dapat sebagai gantinya mendeklarasikannya dalam [`.mcp.json`](/docs/id/mcp#project-scope) yang berkomitmen sehingga itu adalah bagian dari repositori yang diklon.

Ketika Anda membuat rutinitas, semua konektor yang saat ini terhubung disertakan secara default. Hapus yang tidak diperlukan untuk membatasi alat mana yang dapat diakses Claude selama jalankan. Anda juga dapat menambahkan konektor langsung dari formulir rutinitas.

Untuk mengelola atau menambahkan konektor di luar formulir rutinitas, kunjungi [claude.ai/customize/connectors](https://claude.ai/customize/connectors) atau gunakan `/schedule update` di CLI.

<h3 id="environments-and-network-access">
  Lingkungan dan akses jaringan
</h3>

Setiap rutinitas menggunakan [lingkungan cloud](/docs/id/cloud-environments) yang mengontrol akses jaringan, variabel lingkungan, dan skrip penyiapan. Rutinitas mewarisi kebijakan jaringan lingkungan pada setiap jalankan.

Lingkungan **Default** menggunakan akses jaringan **Trusted**, yang hanya memungkinkan [daftar allowlist default](/docs/id/cloud-environments#default-allowed-domains) melalui jaringan sesi. Permintaan pada jalur tersebut ke host di luar allowlist gagal dengan `403` dan `x-deny-reason: host_not_allowed`. Lalu lintas konektor MCP dirutekan melalui server Anthropic daripada jalur tersebut, jadi konektor yang Anda tambahkan ke rutinitas bekerja tanpa menambahkan host mereka ke **Allowed domains**. Hapus konektor apa pun yang tidak Anda butuhkan di bawah [Konektor](#connectors).

Untuk memungkinkan domain tambahan pada salah satu lingkungan Anda sendiri, ikuti langkah-langkah ini. [Lingkungan bersama organisasi](/docs/id/cloud-environments#organization-shared-environments) membuka baca-saja di sini, jadi Pemilik mengubah akses jaringannya dari halaman **Cloud environments** di [pengaturan admin](https://claude.ai/admin-settings) sebagai gantinya.

<Steps>
  <Step title="Buka rutinitas untuk diedit">
    Pada halaman detail rutinitas, buka menu di sebelah nama rutinitas dan pilih **Edit**.
  </Step>

  <Step title="Buka pemilih lingkungan">
    Di bawah kotak **Instructions**, pilih ikon cloud yang menampilkan nama lingkungan Anda, seperti **Default**.
  </Step>

  <Step title="Buka pengaturan lingkungan">
    Arahkan ke lingkungan dalam daftar dan klik ikon pengaturan yang muncul di sebelah kanan.
  </Step>

  <Step title="Ubah tingkat akses jaringan">
    Dalam dialog **Update cloud environment**, ubah **Network access** menjadi **Custom** dan masukkan domain Anda di **Allowed domains**. Periksa **Also include default list of common package managers** untuk menyimpan [daftar allowlist default](/docs/id/cloud-environments#default-allowed-domains) bersama domain kustom Anda. Pilih **Full** sebagai gantinya untuk akses tanpa batas.
  </Step>

  <Step title="Simpan">
    Klik **Save changes**. Kebijakan baru berlaku dari jalankan berikutnya.
  </Step>
</Steps>

Lihat [Network access](/docs/id/cloud-environments#network-access) untuk detail tentang tingkat akses dan daftar allowlist default.

<h2 id="usage-and-limits">
  Penggunaan dan batas
</h2>

Rutinitas mengurangi penggunaan langganan dengan cara yang sama seperti sesi interaktif. Selain batas langganan standar, rutinitas memiliki batas harian tentang berapa banyak jalankan yang dapat dimulai per akun. Lihat konsumsi saat ini dan jalankan rutinitas harian yang tersisa di [claude.ai/code/routines](https://claude.ai/code/routines) atau [claude.ai/settings/usage](https://claude.ai/settings/usage).

Ketika rutinitas mencapai batas harian atau batas penggunaan langganan Anda, organisasi dengan kredit penggunaan yang diaktifkan dapat terus menjalankan rutinitas pada overage terukur. Tanpa kredit penggunaan, jalankan tambahan ditolak sampai jendela direset. Aktifkan kredit penggunaan di [claude.ai/settings/usage](https://claude.ai/settings/usage). Pada paket Team dan Enterprise, admin mengaktifkannya untuk organisasi di [claude.ai/admin-settings/usage](https://claude.ai/admin-settings/usage).

Jalankan sekali saja tidak dihitung terhadap batas jalankan rutinitas harian. Mereka mengurangi penggunaan langganan reguler Anda seperti sesi lainnya.

Sementara langganan Anda dijeda, rutinitas Anda ditahan dan tidak berjalan. Setelah langganan Anda aktif kembali, aktifkan kembali.

<h2 id="troubleshooting">
  Pemecahan masalah
</h2>

<h3 id="schedule-returns-unknown-command">
  `/schedule` menampilkan "Unknown command"
</h3>

CLI menyembunyikan `/schedule` ketika salah satu persyaratannya tidak terpenuhi: menu perintah menampilkan `No commands match "/schedule"` saat Anda mengetik. Mengirimkannya mengembalikan `Unknown command: /schedule`, kecuali dalam kasus-kasus di bawah ini yang mencatat jawaban berbeda.

Penyebabnya biasanya salah satu dari berikut ini:

* Anda diautentikasi dengan Console API key, [profil Anthropic atau kredensial federasi](/docs/id/authentication#anthropic-profiles-and-federation-credentials), atau penyedia cloud seperti Amazon Bedrock, Google Cloud's Agent Platform, atau Microsoft Foundry. `/schedule` memerlukan login langganan claude.ai. Dengan Console API key atau profil, dan pengambilan feature-flag diaktifkan, mengirimkan `/schedule` menampilkan `/schedule is available with Claude for Enterprise — ask your admin about migrating from API-key access`. Dengan login penyedia cloud, Anda masih melihat `Unknown command: /schedule`. Jika `ANTHROPIC_API_KEY` atau `ANTHROPIC_AUTH_TOKEN` diatur di shell Anda, atau `apiKeyHelper` diatur di `settings.json`, hapus terlebih dahulu, karena ini memiliki prioritas lebih tinggi daripada login claude.ai. Profil atau kredensial federasi juga memiliki prioritas, jadi matikan itu juga
* Anda sepenuhnya keluar, tanpa API key atau kredensial lainnya. Dengan pengambilan feature-flag diaktifkan, mengirimkan `/schedule` menampilkan `/schedule requires a claude.ai subscription. Run /login to sign in with your claude.ai account.` Sebelum v2.1.268, sesi yang keluar menampilkan pesan Claude for Enterprise yang sama seperti Console API key
* Anda berada di dalam sesi cloud, di mana mengirimkan `/schedule` menjawab bahwa perintah tidak tersedia di lingkungan tersebut. Kelola rutinitas dari [UI web](https://claude.ai/code/routines) sebagai gantinya
* Kebijakan organisasi Anda menonaktifkan [sesi cloud](/docs/id/claude-code-on-the-web), yang rutinitas jalankan. Dalam kasus ini, mengirimkan `/schedule` menjawab [`Cloud sessions are disabled by your organization's policy`](/docs/id/errors#cloud-sessions-are-disabled-by-your-organizations-policy) sebagai gantinya. Sebelum v2.1.268, ini mengembalikan `Unknown command: /schedule`
* Pemilik [mematikan rutinitas](#routines-are-disabled-by-your-organizations-policy) untuk organisasi Team atau Enterprise Anda. Sebelum v2.1.227, perintah masih muncul dalam kasus ini, dan claude.ai menolak rutinitas ketika Claude mencoba membuat atau menjalankannya

Kecuali kebijakan organisasi Anda menonaktifkan rutinitas atau sesi cloud, Anda dapat membuat dan mengelola rutinitas di [claude.ai/code/routines](https://claude.ai/code/routines) terlepas dari bagaimana CLI dikonfigurasi.

<h3 id="routines-are-disabled-by-your-organizations-policy">
  "Rutinitas dinonaktifkan oleh kebijakan organisasi Anda"
</h3>

Pemilik di organisasi Team atau Enterprise Anda mungkin telah mematikan toggle **Routines** di [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code). Pada Claude Code v2.1.227 atau lebih baru, toggle yang sama juga menyembunyikan `/schedule` di CLI. Ini adalah pengaturan organisasi sisi server, jadi tidak dapat ditimpa dari konfigurasi lokal Anda. Hubungi Pemilik untuk mengaktifkan rutinitas untuk organisasi Anda.

<h2 id="related-resources">
  Sumber daya terkait
</h2>

* [`/loop` and in-session scheduling](/docs/id/scheduled-tasks): jadwalkan tugas lokal dalam sesi CLI terbuka
* [Desktop scheduled tasks](/docs/id/desktop-scheduled-tasks): tugas terjadwal lokal yang berjalan di mesin Anda dengan akses ke file lokal
* [Cloud environments](/docs/id/cloud-environments): konfigurasi akses jaringan, variabel lingkungan, dan skrip penyiapan untuk sesi cloud
* [Projects](/docs/id/claude-projects): pekerjaan berkelanjutan yang Claude koordinasikan di seluruh sesi cloud paralel; rutinitas yang dibuat dari proyek muncul di tab **Routines**
* [MCP connectors](/docs/id/mcp): hubungkan layanan eksternal seperti Slack, Linear, dan Google Drive
* [GitHub Actions](/docs/id/github-actions): jalankan Claude dalam saluran pipa CI pada peristiwa repositori
