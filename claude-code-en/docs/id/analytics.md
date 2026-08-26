> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Lacak penggunaan tim dengan analitik

> Lihat metrik penggunaan Claude Code, lacak adopsi, dan ukur kecepatan teknik dalam dasbor analitik.

Claude Code menyediakan dasbor analitik untuk membantu organisasi memahami pola penggunaan pengembang, melacak metrik kontribusi, dan mengukur bagaimana Claude Code mempengaruhi kecepatan teknik. Akses dasbor untuk paket Anda:

| Paket                         | URL Dasbor                                                                 | Mencakup                                                                                   | Baca selengkapnya                                   |
| ----------------------------- | -------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------ | --------------------------------------------------- |
| Claude for Teams / Enterprise | [claude.ai/analytics/claude-code](https://claude.ai/analytics/claude-code) | Metrik penggunaan, metrik kontribusi dengan integrasi GitHub, papan peringkat, ekspor data | [Detail](#access-analytics-for-team-and-enterprise) |
| API (Claude Console)          | [platform.claude.com/claude-code](https://platform.claude.com/claude-code) | Metrik penggunaan, pelacakan pengeluaran, wawasan tim                                      | [Detail](#access-analytics-for-api-customers)       |

<h2 id="access-analytics-for-team-and-enterprise">
  Akses analitik untuk Team dan Enterprise
</h2>

Navigasikan ke [claude.ai/analytics/claude-code](https://claude.ai/analytics/claude-code). Admin dan Pemilik dapat melihat dasbor.

Dasbor Team dan Enterprise mencakup:

* **Metrik penggunaan**: baris kode yang diterima, tingkat penerimaan saran, pengguna aktif harian dan sesi
* **Metrik kontribusi**: PR dan baris kode yang dikirim dengan bantuan Claude Code, dengan [integrasi GitHub](#enable-contribution-metrics)
* **Papan peringkat**: kontributor teratas yang diperingkat berdasarkan penggunaan Claude Code
* **Ekspor data**: unduh data kontribusi sebagai CSV untuk pelaporan khusus

Untuk penghitungan token per pengguna dan perkiraan biaya, konfigurasikan [ekspor OpenTelemetry](/docs/id/monitoring-usage), atau ekspor [laporan pengeluaran](https://support.claude.com/en/articles/12883420-view-usage-analytics-for-team-and-enterprise-plans) dari pengaturan analitik organisasi Anda, yang mencantumkan penggunaan token dan perkiraan pengeluaran kredit penggunaan per pengguna dan per model.

<h3 id="enable-contribution-metrics">
  Aktifkan metrik kontribusi
</h3>

<Note>
  Metrik kontribusi berada dalam beta publik dan tersedia di paket Claude for Teams dan Claude for Enterprise. Metrik ini hanya mencakup pengguna dalam organisasi claude.ai Anda. Penggunaan melalui API Claude Console atau integrasi pihak ketiga tidak disertakan.
</Note>

Data penggunaan dan adopsi tersedia untuk semua akun Claude for Teams dan Claude for Enterprise. Metrik kontribusi memerlukan pengaturan tambahan untuk menghubungkan organisasi GitHub Anda.

Anda memerlukan peran Pemilik untuk mengonfigurasi pengaturan analitik. Admin GitHub harus memasang aplikasi GitHub.

<Warning>
  Metrik kontribusi tidak tersedia untuk organisasi dengan [Zero Data Retention](/docs/id/zero-data-retention) diaktifkan. Dasbor analitik hanya akan menampilkan metrik penggunaan.
</Warning>

<Steps>
  <Step title="Pasang aplikasi GitHub">
    Admin GitHub memasang aplikasi Claude GitHub di akun GitHub organisasi Anda di [github.com/apps/claude](https://github.com/apps/claude).
  </Step>

  <Step title="Aktifkan analitik Claude Code">
    Pemilik Claude menavigasi ke [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code) dan mengaktifkan fitur analitik Claude Code.
  </Step>

  <Step title="Aktifkan analitik GitHub">
    Di halaman yang sama, aktifkan toggle "GitHub analytics".
  </Step>

  <Step title="Autentikasi dengan GitHub">
    Selesaikan alur autentikasi GitHub dan pilih organisasi GitHub mana yang akan disertakan dalam analisis.
  </Step>
</Steps>

Data biasanya muncul dalam 24 jam setelah diaktifkan, dengan pembaruan harian. Jika tidak ada data yang muncul, Anda mungkin melihat salah satu pesan ini:

* **"GitHub app required"**: pasang aplikasi GitHub untuk melihat metrik kontribusi
* **"Data processing in progress"**: periksa kembali dalam beberapa hari dan konfirmasi aplikasi GitHub terpasang jika data tidak muncul

Metrik kontribusi mendukung GitHub Cloud dan GitHub Enterprise Server.

<h3 id="review-summary-metrics">
  Tinjau metrik ringkasan
</h3>

<Note>
  Metrik ini sengaja konservatif dan mewakili perkiraan rendah dari dampak sebenarnya Claude Code. Hanya baris dan PR di mana ada kepercayaan tinggi pada keterlibatan Claude Code yang dihitung.
</Note>

Dasbor menampilkan metrik ringkasan ini di bagian atas:

* **PRs with CC**: jumlah total permintaan tarik yang digabungkan yang berisi setidaknya satu baris kode yang ditulis dengan Claude Code
* **Lines of code with CC**: total baris kode di semua PR yang digabungkan yang ditulis dengan bantuan Claude Code. Hanya "baris efektif" yang dihitung: baris dengan lebih dari 3 karakter setelah normalisasi, tidak termasuk baris kosong dan baris dengan hanya tanda kurung atau tanda baca sepele.
* **PRs with Claude Code (%)**: persentase semua PR yang digabungkan yang berisi kode yang dibantu Claude Code
* **Suggestion accept rate**: persentase waktu pengguna menerima saran pengeditan kode Claude Code, termasuk penggunaan alat Edit, Write, dan NotebookEdit
* **Lines of code accepted**: total baris kode yang ditulis oleh Claude Code yang telah diterima pengguna dalam sesi mereka. Ini tidak termasuk saran yang ditolak dan tidak melacak penghapusan berikutnya.

<h3 id="explore-the-charts">
  Jelajahi grafik
</h3>

Dasbor mencakup beberapa grafik untuk memvisualisasikan tren dari waktu ke waktu.

<h4 id="track-adoption">
  Lacak adopsi
</h4>

Grafik Adopsi menunjukkan tren penggunaan harian:

* **users**: pengguna aktif harian
* **sessions**: jumlah sesi Claude Code aktif per hari

<h4 id="measure-prs-per-user">
  Ukur PR per pengguna
</h4>

Grafik ini menampilkan aktivitas pengembang individu dari waktu ke waktu:

* **PRs per user**: jumlah total PR yang digabungkan per hari dibagi dengan pengguna aktif harian
* **users**: pengguna aktif harian

Gunakan ini untuk memahami bagaimana produktivitas individu berubah seiring dengan meningkatnya adopsi Claude Code.

<h4 id="view-pull-requests-breakdown">
  Lihat rincian permintaan tarik
</h4>

Grafik Pull requests menunjukkan rincian harian PR yang digabungkan:

* **PRs with CC**: permintaan tarik yang berisi kode yang dibantu Claude Code
* **PRs without CC**: permintaan tarik tanpa kode yang dibantu Claude Code

Alihkan ke tampilan **Lines of code** untuk melihat rincian yang sama berdasarkan baris kode daripada jumlah PR.

<h4 id="find-top-contributors">
  Temukan kontributor teratas
</h4>

Papan Peringkat menampilkan 10 pengguna teratas yang diperingkat berdasarkan volume kontribusi. Alihkan antara:

* **Pull requests**: menampilkan PR dengan Claude Code vs Semua PR untuk setiap pengguna
* **Lines of code**: menampilkan baris dengan Claude Code vs Semua baris untuk setiap pengguna

Klik **Export all users** untuk mengunduh data kontribusi lengkap untuk semua pengguna sebagai file CSV. Ekspor mencakup semua pengguna, bukan hanya 10 teratas yang ditampilkan.

<h3 id="pr-attribution">
  Atribusi PR
</h3>

Ketika metrik kontribusi diaktifkan, Claude Code menganalisis permintaan tarik yang digabungkan untuk menentukan kode mana yang ditulis dengan bantuan Claude Code. Ini dilakukan dengan mencocokkan aktivitas sesi Claude Code terhadap kode di setiap PR.

<h4 id="tagging-criteria">
  Kriteria penandaan
</h4>

PR ditandai sebagai "with Claude Code" jika berisi setidaknya satu baris kode yang ditulis selama sesi Claude Code. Sistem menggunakan pencocokan konservatif: hanya kode di mana ada kepercayaan tinggi pada keterlibatan Claude Code yang dihitung sebagai dibantu.

<h4 id="attribution-process">
  Proses atribusi
</h4>

Ketika permintaan tarik digabungkan:

1. Baris yang ditambahkan diekstrak dari diff PR
2. Sesi Claude Code yang mengedit file yang cocok dalam jendela waktu diidentifikasi
3. Baris PR dicocokkan terhadap output Claude Code menggunakan beberapa strategi
4. Metrik dihitung untuk baris yang dibantu AI dan total baris

Sebelum perbandingan, baris dinormalisasi: spasi dipangkas, beberapa spasi diruntuhkan, tanda kutip distandarkan, dan teks dikonversi ke huruf kecil.

Permintaan tarik yang digabungkan yang berisi baris yang dibantu Claude Code diberi label `claude-code-assisted` di GitHub.

<h4 id="time-window">
  Jendela waktu
</h4>

Sesi dari 21 hari sebelum hingga 2 hari setelah tanggal penggabungan PR dipertimbangkan untuk pencocokan atribusi.

<h4 id="excluded-files">
  File yang dikecualikan
</h4>

File tertentu secara otomatis dikecualikan dari analisis karena mereka dibuat secara otomatis:

* File kunci: package-lock.json, yarn.lock, Cargo.lock, dan serupa
* Kode yang dibuat: output Protobuf, artefak build, file yang diminimalkan
* Direktori build: dist/, build/, node\_modules/, target/
* Perlengkapan uji: snapshot, kaset, data mock
* Baris lebih dari 1.000 karakter, yang mungkin diminimalkan atau dibuat

<h4 id="attribution-notes">
  Catatan atribusi
</h4>

Ingat detail tambahan ini saat menafsirkan data atribusi:

* Kode yang ditulis ulang secara substansial oleh pengembang, dengan perbedaan lebih dari 20%, tidak dikaitkan dengan Claude Code
* Sesi di luar jendela 21 hari tidak dipertimbangkan
* Algoritma tidak mempertimbangkan sumber PR atau cabang tujuan saat melakukan atribusi

<h3 id="get-the-most-from-analytics">
  Dapatkan yang terbaik dari analitik
</h3>

Gunakan metrik kontribusi untuk menunjukkan ROI, mengidentifikasi pola adopsi, dan menemukan anggota tim yang dapat membantu orang lain memulai.

<h4 id="monitor-adoption">
  Pantau adopsi
</h4>

Lacak grafik Adopsi dan jumlah pengguna untuk mengidentifikasi:

* Pengguna aktif yang dapat berbagi praktik terbaik
* Tren adopsi keseluruhan di seluruh organisasi Anda
* Penurunan penggunaan yang mungkin menunjukkan gesekan atau masalah

<h4 id="measure-roi">
  Ukur ROI
</h4>

Metrik kontribusi membantu menjawab "Apakah alat ini layak untuk investasi?" dengan data dari basis kode Anda sendiri:

* Lacak perubahan dalam PR per pengguna dari waktu ke waktu seiring dengan meningkatnya adopsi
* Bandingkan PR dan baris kode yang dikirim dengan vs. tanpa Claude Code
* Gunakan bersama [metrik DORA](https://dora.dev/), kecepatan sprint, atau KPI teknik lainnya untuk memahami perubahan dari adopsi Claude Code

<h4 id="identify-power-users">
  Identifikasi pengguna power
</h4>

Papan Peringkat membantu Anda menemukan anggota tim dengan adopsi Claude Code tinggi yang dapat:

* Berbagi teknik prompting dan alur kerja dengan tim
* Memberikan umpan balik tentang apa yang berfungsi dengan baik
* Membantu menginisialisasi pengguna baru

<h4 id="access-data-programmatically">
  Akses data secara terprogram
</h4>

Pada paket Enterprise, [Claude Enterprise Analytics API](https://platform.claude.com/docs/en/api/admin/analytics) mengembalikan laporan keterlibatan per pengguna, penggunaan, dan biaya untuk organisasi Anda di seluruh permukaan Claude, termasuk Claude Code. Pemilik Utama membuat kunci dengan cakupan `read:analytics` di [claude.ai/analytics/api-keys](https://claude.ai/analytics/api-keys). API tidak tersedia di paket Teams.

Untuk menanyakan data kontribusi melalui GitHub sebagai gantinya, cari PR yang diberi label dengan `claude-code-assisted`.

<h2 id="access-analytics-for-api-customers">
  Akses analitik untuk pelanggan API
</h2>

Pelanggan API yang menggunakan Claude Console dapat mengakses analitik di [platform.claude.com/claude-code](https://platform.claude.com/claude-code). Anda memerlukan izin UsageView untuk mengakses dasbor, yang diberikan kepada peran Developer, Billing, Admin, Owner, dan Primary Owner. Untuk menarik metrik harian per pengguna yang sama secara terprogram, gunakan [Claude Code Analytics API](https://platform.claude.com/docs/en/build-with-claude/claude-code-analytics-api) dengan kunci API Admin.

<Note>
  Metrik kontribusi dengan integrasi GitHub saat ini tidak tersedia untuk pelanggan API. Dasbor Console menampilkan metrik penggunaan dan pengeluaran saja.
</Note>

Dasbor Console menampilkan:

* **Lines of code accepted**: total baris kode yang ditulis oleh Claude Code yang telah diterima pengguna dalam sesi mereka. Ini tidak termasuk saran yang ditolak dan tidak melacak penghapusan berikutnya.
* **Suggestion accept rate**: persentase waktu pengguna menerima penggunaan alat pengeditan kode, termasuk alat Edit, Write, dan NotebookEdit.
* **Activity**: pengguna aktif harian dan sesi ditampilkan pada grafik.
* **Spend**: biaya API harian dalam dolar bersama jumlah pengguna.

<h3 id="view-team-insights">
  Lihat wawasan tim
</h3>

Tabel wawasan tim menampilkan metrik per pengguna:

* **Members**: semua pengguna yang telah diautentikasi ke Claude Code. Pengguna kunci API ditampilkan berdasarkan pengidentifikasi kunci, pengguna OAuth ditampilkan berdasarkan alamat email.
* **Spend this month**: total biaya API per pengguna untuk bulan saat ini.
* **Lines this month**: total per pengguna dari baris kode yang diterima untuk bulan saat ini.

<Note>
  Angka pengeluaran di dasbor Console adalah perkiraan untuk tujuan analitik. Untuk biaya aktual, lihat halaman penagihan Anda.
</Note>

<h2 id="related-resources">
  Sumber daya terkait
</h2>

* [Monitoring with OpenTelemetry](/docs/id/monitoring-usage): ekspor metrik dan acara real-time ke tumpukan observabilitas Anda
* [Manage costs effectively](/docs/id/costs): tetapkan batas pengeluaran dan optimalkan penggunaan token
* [Permissions](/docs/id/permissions): konfigurasi peran dan izin
