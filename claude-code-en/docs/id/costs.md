> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Kelola biaya secara efektif

> Lacak penggunaan token, tetapkan batas pengeluaran tim, dan kurangi biaya Claude Code dengan manajemen konteks, pemilihan model, pengaturan pemikiran yang diperluas, dan hook prapemrosesan.

Claude Code mengenakan biaya berdasarkan konsumsi token API. Untuk harga paket langganan (Pro, Max, Team, Enterprise), lihat [claude.com/pricing](https://claude.com/pricing). Biaya per pengembang bervariasi luas berdasarkan pemilihan model, ukuran basis kode, dan pola penggunaan seperti menjalankan beberapa instans atau otomasi.

Di seluruh penyebaran perusahaan, biaya rata-rata adalah sekitar \$13 per pengembang per hari aktif dan \$150-250 per pengembang per bulan, dengan biaya tetap di bawah \$30 per hari aktif untuk 90% pengguna. Untuk memperkirakan pengeluaran untuk tim Anda sendiri, mulai dengan kelompok pilot kecil dan gunakan alat pelacakan di bawah untuk membangun baseline sebelum peluncuran yang lebih luas.

Halaman ini mencakup cara [melacak biaya Anda](#track-your-costs), [mengelola biaya untuk organisasi Anda](#manage-costs-for-your-organization), dan [mengurangi penggunaan token](#reduce-token-usage).

<h2 id="track-your-costs">
  Lacak biaya Anda
</h2>

<h3 id="using-the-/usage-command">
  Menggunakan perintah `/usage`
</h3>

<Note>
  Blok Session dalam `/usage` menampilkan penggunaan token API dan dimaksudkan untuk pengguna API. Pelanggan Claude Max dan Pro memiliki penggunaan yang disertakan dalam langganan mereka, jadi angka biaya sesi tidak relevan untuk tujuan penagihan. Pelanggan melihat bilah penggunaan paket, statistik aktivitas, dan rincian penggunaan di layar yang sama.
</Note>

Blok Session di bagian atas `/usage` menampilkan statistik penggunaan token terperinci untuk sesi Anda saat ini. Claude Code menghitung angka dolar secara lokal dari jumlah token dengan harga daftar, kecuali tabel [`modelPricing`](/docs/id/settings-reference#modelpricing) berlaku. Administrator menetapkan satu dalam pengaturan terkelola organisasi Anda sehingga angka menggunakan tarif kontrak Anda, dan sementara tabel berlaku, baris `Total cost` membawa catatan `at your organization's configured rates`. Angka ini adalah perkiraan, jadi untuk penagihan yang berwenang, lihat halaman Penggunaan di [Claude Console](https://platform.claude.com/usage).

```text theme={null}
Total cost:            $0.55
Total duration (API):  6m 20s
Total duration (wall): 6h 33m 10s
Total code changes:    0 lines added, 0 lines removed
Usage by model:
   claude-sonnet-4-6:  1.2k input, 5.3k output, 940.0k cache read, 50.0k cache write ($0.55)
```

Total ini direset ketika `/clear` memulai sesi baru, jadi biaya total sesi berikutnya dimulai dari \$0. Sebelum v2.1.211, mereka terus terakumulasi di seluruh `/clear` untuk seumur hidup proses Claude Code.

Untuk respons dari Claude API yang ditagih dengan [tarif residensi data](https://platform.claude.com/docs/en/about-claude/pricing#data-residency-pricing) 1.1×, Claude Code mengalikan harga daftar token respons tersebut dengan 1.1 dalam angka biaya sesi. Total yang sama muncul dalam [bidang biaya baris status](/docs/id/statusline#cost-and-duration-tracking), dan angka yang dikalikan juga diperhitungkan terhadap [`--max-budget-usd`](/docs/id/cli-reference#cli-flags). Sebelum v2.1.239, Claude Code tidak menerapkan 1.1× ke respons tersebut, jadi angka biaya sesi lebih rendah dari tagihan.

<h4 id="prompt-cache-statistics">
  Statistik cache prompt
</h4>

Setelah respons API pertama percakapan utama, Claude Code juga menambahkan baris `Prompt cache (main)` ke blok Session, merangkum penggunaan [prompt cache](/docs/id/prompt-caching) sesi: jumlah permintaan, bagian token input yang disajikan dari cache, cache miss, dan apakah cache hangat sekarang. Memerlukan Claude Code v2.1.251 atau lebih baru.

```text theme={null}
Prompt cache (main):   14 requests · 91% of input tokens from cache · 2 misses (last 6m 10s ago, 310.2k tokens re-cached) · 1 expected rebuild (compaction or tool-result clearing) · warm (1h TTL, last activity 40s ago)
```

Bagian miss, expected rebuild, dan warm atau cold dari baris berarti hal berikut:

* **Misses**: permintaan yang memproses ulang konten yang sudah dipegang cache, dengan waktu miss terakhir dan berapa banyak token yang ditulis kembali ke cache oleh permintaan tersebut. Claude Code menghitung permintaan sebagai miss ketika permintaan memproses ulang lebih dari 5% dan setidaknya 2.000 token dari apa yang bisa dibaca dari cache. [Tindakan yang membatalkan cache](/docs/id/prompt-caching#actions-that-invalidate-the-cache) mencantumkan penyebab umum. Ketika Claude Code dapat mengidentifikasi kemungkinan penyebab miss terakhir, baris menamakannya juga, misalnya `likely cause: tool definitions changed`. Teks kemungkinan-penyebab memerlukan Claude Code v2.1.260 atau lebih baru.
* **Expected rebuilds**: ketika Claude Code sendiri baru saja menulis ulang percakapan, dengan [compaction](/docs/id/prompt-caching#compacting-the-conversation) atau dengan menghapus hasil tool lama dari konteks, ia menghitung jenis miss yang sama sebagai expected rebuild. Bagian ini muncul hanya setelah setidaknya satu expected rebuild telah terjadi.
* **Warm atau cold**: apakah awalan cache masih dalam [cache lifetime](/docs/id/prompt-caching#cache-lifetime), dengan TTL yang berlaku. Ketika cache dingin, baris menunjukkan berapa lama sesi telah idle. Ketika tidak ada respons yang melaporkan token cache, baris berakhir dengan `no prompt caching reported by the API`.

Hitungan berasal dari bidang token cache dalam respons API, jadi baris bekerja di setiap penyedia dan gateway. Ini mencakup percakapan utama saja, bukan subagent. `/clear` mereset dengan sisa blok Session.

Skrip baris status dapat membaca angka yang sama dari [`prompt_cache` object](/docs/id/statusline#prompt-cache-fields).

<h4 id="plan-usage-breakdown">
  Rincian penggunaan paket
</h4>

Pada paket Pro, Max, Team, atau Enterprise, `/usage` juga menampilkan rincian tentang apa yang diperhitungkan terhadap batas paket Anda:

* **Attribution**: penggunaan terbaru yang dikaitkan dengan skills, subagents, plugins, dan server MCP individual, masing-masing ditampilkan sebagai persentase dari total. Bagian server MCP hanya menghitung permintaan yang menggunakan salah satu hasil tool-nya. Sebelum v2.1.222, setelah satu panggilan ke server MCP, Claude Code mengatribusikan setiap permintaan berikutnya ke server tersebut, melebih-lebihkan bagiannya.
* **Behavior flags**: perilaku seperti konteks panjang atau cache miss, ditandai ketika satu menyumbang 10% atau lebih dari penggunaan terbaru.
* **Loops**: baris untuk masing-masing dari [`/loop` atau tugas terjadwal lainnya](/docs/id/scheduled-tasks) terberat yang berjalan baru-baru ini, diurutkan berdasarkan total token, dengan hitungan sisanya. Claude Code melaporkan seberapa sering setiap tugas dijalankan, berapa kali tugas itu berjalan, total dan token per-jalannya, dan kapan terakhir kali dijalankan. Claude Code mengkunci baris dengan prompt tugas, jadi loop yang Anda hentikan dan buat ulang tetap menjadi satu baris. Memerlukan Claude Code v2.1.242 atau lebih baru.

Tekan `d` atau `w` untuk beralih antara 24 jam terakhir dan 7 hari terakhir. Angka-angka tersebut bersifat perkiraan dan dihitung dari riwayat sesi lokal di mesin ini, jadi penggunaan dari perangkat lain atau claude.ai tidak disertakan.

Di [ekstensi VS Code](/docs/id/vs-code#check-account-and-usage), bagian attribution dan behavior flags muncul dalam dialog Account & usage dengan toggle Day dan Week, tanpa baris Loops.

<h4 id="check-your-usage-credits-spend">
  Periksa pengeluaran kredit penggunaan Anda
</h4>

`/usage` juga menampilkan baris kredit penggunaan saat [kredit penggunaan](#add-usage-credits-to-your-subscription) aktif. Apa yang ditampilkan baris tergantung pada paket Anda:

* **Pro dan Max**: pengeluaran Anda untuk bulan saat ini, diukur terhadap batas pengeluaran bulanan Anda ketika Anda telah menetapkan satu. Ketika Anda belum menetapkan batas, baris menampilkan `Unlimited` dan tidak ada angka pengeluaran.
* **Team dan Enterprise**: pengeluaran Anda sendiri untuk bulan saat ini, diukur terhadap [batas yang ditetapkan organisasi Anda](#claude-for-teams-and-enterprise) yang berlaku untuk Anda. Batas yang mencakup seluruh organisasi tidak muncul dalam baris. Ketika Anda tidak memiliki batas Anda sendiri, baris menampilkan pengeluaran Anda tanpa batas di sampingnya. Sementara kredit penggunaan dimatikan untuk Anda, `/usage` tidak menampilkan baris kredit penggunaan.

Ketika Anda memiliki batas pengeluaran, baris muncul segera setelah kredit penggunaan aktif dan menampilkan 0% sampai Anda pertama kali menghabiskan kredit penggunaan. Sebelum v2.1.236, `/usage` menampilkan baris hanya pada paket Pro dan Max, dan baris dengan batas pengeluaran tetap tersembunyi sampai Anda telah menghabiskan sesuatu.

<h4 id="when-the-usage-request-fails">
  Ketika permintaan penggunaan gagal
</h4>

Ketika permintaan untuk batas paket Anda gagal, paling sering karena endpoint penggunaan dibatasi laju, `/usage` menampilkan bilah penggunaan terakhir yang dimuat di mesin ini dalam 60 menit terakhir, bersama dengan catatan `Showing last-known usage` yang menyatakan berapa lama yang lalu data tersebut diambil. Tekan `r` untuk mencoba lagi; percobaan ulang yang berhasil menggantikan bilah terakhir yang diketahui dengan data segar. Tanpa snapshot dari 60 menit terakhir, `/usage` melaporkan bahwa endpoint penggunaan dibatasi laju dan menawarkan pintasan percobaan ulang yang sama. Sebelum v2.1.208, permintaan yang dibatasi laju dalam sesi yang belum memuat penggunaan selalu menampilkan kesalahan tanpa bilah.

<h3 id="analyze-your-usage-patterns">
  Analisis pola penggunaan Anda
</h3>

Jalankan [`/insights`](/docs/id/commands#all-commands) untuk laporan tentang cara Anda bekerja daripada berapa banyak token yang telah Anda gunakan. Ini menganalisis sesi terbaru Anda di mesin ini dan menulis laporan HTML yang mencakup apa yang Anda kerjakan, titik gesekan seperti permintaan yang salah dipahami atau kode yang bermasalah, dan saran untuk menggunakan Claude Code lebih efektif. Satu kali jalankan menganalisis hingga 200 sesi yang belum pernah dilihat sebelumnya dan melewati yang sangat pendek. Ketika sesi ditinggalkan, header laporan menampilkan jumlah yang dianalisis dengan total dalam tanda kurung, misalnya `200 sessions (412 total)`.

Claude Code menulis laporan terbaru ke `~/.claude/usage-data/report.html` dan menyimpan salinan dengan stempel waktu dari setiap jalankan di direktori yang sama, jadi laporan sebelumnya tidak ditimpa. Claude Code menghapus laporan sesuai jadwal yang sama dengan sisa data sesi Anda: saat startup, ia menghapus file yang lebih lama dari [`cleanupPeriodDays`](/docs/id/claude-directory#cleaned-up-automatically), 30 hari secara default.

Anda dapat menjalankan `/insights` pada paket apa pun dan dengan penyedia apa pun. Analisis berjalan melalui penyedia dan akun yang sama dengan sesi reguler Anda, dan token diperhitungkan terhadap penggunaan paket atau API Anda. Sesi dari perangkat lain dan claude.ai tidak disertakan.

<h3 id="add-usage-credits-to-your-subscription">
  Tambahkan kredit penggunaan ke langganan Anda
</h3>

[Kredit penggunaan](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) memungkinkan Anda terus bekerja melampaui batas penggunaan paket Anda. Untuk mengelolanya, jalankan `/usage-credits` setelah masuk dengan langganan claude.ai Anda melalui `/login`; perintah tidak tersedia dengan autentikasi kunci API. Dalam organisasi Enterprise self-serve, uji coba Enterprise, dan organisasi Enterprise yang ditagih melalui AWS Marketplace, perintah memerlukan Claude Code v2.1.248 atau lebih baru; versi sebelumnya menolaknya dengan [`Unknown command: /usage-credits`](/docs/id/errors#unknown-command). Apa yang dibukanya tergantung pada peran Anda:

| Peran Anda                                          | Apa yang dilakukan `/usage-credits`                                                                                                                                                                                                                                      |
| :-------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Pelanggan Pro atau Max                              | Membuka [**Settings > Usage**](https://claude.ai/settings/usage) di claude.ai di browser. Di bagian **Usage credits** Anda dapat mengaktifkan atau menonaktifkan kredit penggunaan dan memeriksa saldo kredit, pengeluaran bulan ini, dan batas pengeluaran bulanan Anda |
| Anggota Team atau Enterprise dengan akses penagihan | Membuka pengaturan penggunaan organisasi Anda, [**Admin settings > Usage**](https://claude.ai/admin-settings/usage), di browser                                                                                                                                          |
| Anggota Team atau Enterprise tanpa akses penagihan  | Meminta Anda untuk mengonfirmasi, kemudian mengirim permintaan ke admin organisasi Anda. Sebelum v2.1.211, Claude Code mengirim permintaan tanpa langkah konfirmasi                                                                                                      |

Untuk anggota Team dan Enterprise tanpa akses penagihan, konfirmasi muncul hanya dalam sesi interaktif: dalam mode non-interaktif dengan flag `-p` dan dari [Remote Control](/docs/id/remote-control), perintah tidak mengirim permintaan dan memberi tahu Anda untuk menjalankannya dalam sesi interaktif.

Jika Anda menjalankan `/usage-credits` lagi sementara permintaan sebelumnya menunggu admin, Claude Code memberi tahu Anda bahwa permintaan sudah dikirim daripada mengirim duplikat. Setelah admin membatalkan permintaan Anda, menjalankan perintah lagi mengirim yang baru. Sebelum v2.1.222, permintaan yang dibatalkan juga memblokir permintaan baru.

Pada paket Pro dan Max, ketika Anda mencapai batas pengeluaran dengan kredit penggunaan masih tersedia, Claude Code meminta Anda untuk menaikkan atau menghapus batas tanpa meninggalkan CLI. Jika server menolak perubahan, lihat [Could not update your spend limit](/docs/id/errors#could-not-update-your-spend-limit).

<h2 id="manage-costs-for-your-organization">
  Mengelola biaya untuk organisasi Anda
</h2>

Kontrol mana yang Anda miliki tergantung pada bagaimana organisasi Anda mengakses Claude Code: paket Claude for Teams atau Enterprise, Claude Console, atau penyedia cloud. Pada paket Teams dan Enterprise, penggunaan ditarik dari tunjangan kursi setiap anggota. Di Console dan di penyedia cloud, penggunaan ditagih per token ke organisasi Anda. Jika organisasi Anda mencampur metode masuk, setiap pengembang diukur sesuai dengan yang mereka autentikasi.

Tabel memetakan setiap pengaturan ke tempat Anda melihat pengeluaran, tempat Anda membatasinya, dan bagaimana Anda menarik angka per pengguna. Pada paket Pro atau Max individual Anda tidak memiliki organisasi untuk dikelola, jadi lacak pengeluaran kredit penggunaan Anda sendiri, termasuk [fast mode](/docs/id/fast-mode#see-where-fast-mode-spend-appears), di bawah [Tambahkan kredit penggunaan ke langganan Anda](#add-usage-credits-to-your-subscription).

| Pengaturan Anda                                                                           | Lihat pengeluaran                                                                                                                                   | Batasi pengeluaran                       | Pelaporan per pengguna                                                                                                                                                                                                           |
| :---------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [Claude for Teams atau Enterprise](#claude-for-teams-and-enterprise)                      | [Laporan pengeluaran dalam analitik organisasi](https://support.claude.com/en/articles/12883420-view-usage-analytics-for-team-and-enterprise-plans) | Batas pengeluaran dalam pengaturan admin | [CSV laporan pengeluaran](https://support.claude.com/en/articles/12883420-view-usage-analytics-for-team-and-enterprise-plans); [Enterprise Analytics API](https://platform.claude.com/docs/en/api/admin/analytics) di Enterprise |
| [Claude Console (API)](#claude-console)                                                   | [Halaman penggunaan Console](https://platform.claude.com/usage)                                                                                     | Batas pengeluaran ruang kerja            | [Dashboard Console](https://platform.claude.com/claude-code), [Claude Code Analytics API](https://platform.claude.com/docs/en/build-with-claude/claude-code-analytics-api)                                                       |
| [Amazon Bedrock, Google Cloud's Agent Platform, atau Microsoft Foundry](#cloud-providers) | Konsol penagihan cloud Anda                                                                                                                         | Kontrol anggaran cloud Anda              | [OpenTelemetry](/docs/id/monitoring-usage) atau [gateway LLM](/docs/id/llm-gateway)                                                                                                                                                        |

[Ekspor OpenTelemetry](/docs/id/monitoring-usage) bekerja pada setiap pengaturan dan merupakan satu-satunya opsi yang mengalirkan metrik token dan biaya per pengguna ke dalam tumpukan observabilitas Anda sendiri secara real-time.

<h3 id="report-spend-at-your-contracted-rates">
  Laporkan pengeluaran pada tarif kontrak Anda
</h3>

Secara default, Claude Code menghitung setiap angka biaya yang ditunjukkannya kepada pengembang pada harga daftar, jadi jika organisasi Anda membayar tarif kontrak, angka-angka di `/usage`, baris status, dan OpenTelemetry tidak cocok dengan tagihan Anda. Untuk membuatnya cocok, atur pengaturan terkelola [`modelPricing`](/docs/id/settings-reference#modelpricing) ke tarif Anda. Pengaturan mengubah apa yang dilaporkan Claude Code, bukan apa yang ditagihkan Anthropic. Memerlukan Claude Code v2.1.242 atau lebih baru.

<Steps>
  <Step title="Ambil tarif dari kontrak Anda">
    Masukkan tarif per-juta-token dari kontrak Anda. Claude Code tidak mengambilnya dari Claude Console, jadi perbarui pengaturan ketika kontrak berubah.
  </Step>

  <Step title="Tulis pengaturannya">
    Atur `multiplier` di bawah 1 untuk diskon datar atau di atas 1 untuk markup, daftarkan empat tarif per-token setiap model di bawah `overrides`, atau lakukan keduanya. Markup memerlukan Claude Code v2.1.271 atau lebih baru. Entri [`modelPricing`](/docs/id/settings-reference#modelpricing) memiliki bentuk dan contoh siap tempel.
  </Step>

  <Step title="Terapkan melalui pengaturan terkelola">
    Berikan sebagai [pengaturan terkelola](/docs/id/managed-settings): pengaturan terkelola server, kebijakan MDM, `managed-settings.json`, atau [pembantu kebijakan](/docs/id/managed-settings#compute-the-policy-with-a-helper-program). Claude Code mengabaikan kunci dalam pengaturan pengguna, proyek, dan lokal serta dalam `--settings`.
  </Step>
</Steps>

Untuk mengonfirmasi tarif berlaku, jalankan `/usage` dalam sesi yang telah [menerima pengaturan terkelola](/docs/id/managed-settings#read-the-source-in-%2Fstatus): blok Sesi `Total cost` membawa catatan `at your organization's configured rates`. Angka-angka masih merupakan perkiraan, bukan faktur. Harga per-juta-token di pemilih `/model` tetap pada harga daftar.

<h3 id="claude-for-teams-and-enterprise">
  Claude for Teams dan Enterprise
</h3>

Pada paket Claude for Teams dan Enterprise, penggunaan Claude Code setiap anggota ditarik dari tunjangan per-kursi yang direset pada jendela lima jam bergulir dan jendela mingguan. Tunjangan dibagikan dengan Claude chat dan Cowork, dan ukurannya tergantung pada [tingkat kursi](https://support.claude.com/en/articles/11845131-use-claude-code-with-your-team-or-enterprise-plan) anggota (Standard atau Premium). Kontrol Anda berada di konsol admin claude.ai, bukan Claude Console.

* **Lihat pengeluaran**: [laporan pengeluaran dalam analitik organisasi](https://support.claude.com/en/articles/12883420-view-usage-analytics-for-team-and-enterprise-plans) menunjukkan pengeluaran perkiraan per pengguna dan per model, dengan ekspor CSV, diperbarui setiap hari. Laporan mencakup pengeluaran kredit penggunaan dan muncul setelah kredit penggunaan diaktifkan. Penggunaan dalam tunjangan kursi tidak diukur dalam dolar.
* **Lihat adopsi**: [dashboard analitik](https://claude.ai/analytics/claude-code) menunjukkan pengguna aktif harian, sesi, dan metrik kontribusi, dengan ekspor CSV data kontribusi. Lihat [lacak penggunaan tim dengan analitik](/docs/id/analytics).
* **Batasi pengeluaran**: tunjangan kursi adalah batas default. Untuk membiarkan anggota melanjutkan melampauinya, aktifkan [kredit penggunaan](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) dan tetapkan batas pengeluaran di tingkat organisasi, grup, atau anggota individual.
* **Tarik angka per pengguna**: pada paket Enterprise, [Enterprise Analytics API](https://platform.claude.com/docs/en/api/admin/analytics) mengembalikan laporan penggunaan dan biaya per pengguna di seluruh permukaan Claude, termasuk Claude Code. Pemilik Utama membuat kunci dengan cakupan `read:analytics` di [claude.ai/analytics/api-keys](https://claude.ai/analytics/api-keys). Pada paket Teams, ekspor [CSV laporan pengeluaran](https://support.claude.com/en/articles/12883420-view-usage-analytics-for-team-and-enterprise-plans), yang mencantumkan penggunaan token dan pengeluaran perkiraan per pengguna dan per model.

[Panduan konsumsi Claude Enterprise](https://support.claude.com/en/articles/14782391-claude-enterprise-consumption-guide) adalah referensi perencanaan untuk admin. Ini menjelaskan bagaimana konsumsi berbeda di seluruh Claude chat, Claude Code, dan Cowork, dan memberikan titik awal dolar per pengguna untuk penganggaran. Anggaran lebih banyak untuk kursi coding daripada kursi chat: setiap putaran Claude Code membawa konten file, panggilan alat, dan penalaran multi-langkah, jadi satu sesi debugging dapat mengonsumsi lebih dari sehari chat.

<h3 id="claude-console">
  Claude Console
</h3>

Organisasi API mengelola pengeluaran Claude Code melalui [ruang kerja](https://platform.claude.com/docs/en/build-with-claude/workspaces). Anda dapat [menetapkan batas pengeluaran ruang kerja](https://platform.claude.com/docs/en/build-with-claude/workspaces#workspace-limits) pada total pengeluaran Claude Code dan [melihat pelaporan biaya dan penggunaan](https://platform.claude.com/docs/en/build-with-claude/workspaces#usage-and-cost-tracking) di Console.

<Note>
  Ketika Anda pertama kali mengautentikasi Claude Code dengan akun Claude Console Anda, ruang kerja yang disebut "Claude Code" secara otomatis dibuat untuk Anda. Ruang kerja ini menyediakan pelacakan dan manajemen biaya terpusat untuk semua penggunaan Claude Code di organisasi Anda. Anda tidak dapat membuat kunci API untuk ruang kerja ini; ini secara eksklusif untuk autentikasi dan penggunaan Claude Code.

  Untuk organisasi dengan batas laju kustom, lalu lintas Claude Code di ruang kerja ini dihitung terhadap batas laju API keseluruhan organisasi Anda. Anda dapat menetapkan [batas laju ruang kerja](https://platform.claude.com/docs/en/api/rate-limits#setting-lower-limits-for-workspaces) di halaman Batas ruang kerja ini di Claude Console untuk membatasi bagian Claude Code dan melindungi beban kerja produksi lainnya.
</Note>

Untuk pelaporan per pengguna, [dashboard Console](https://platform.claude.com/claude-code) menunjukkan pengeluaran dan baris yang diterima per anggota, dan [Claude Code Analytics API](https://platform.claude.com/docs/en/build-with-claude/claude-code-analytics-api) mengembalikan metrik harian per pengguna yang sama secara terprogram dengan [kunci API Admin](https://platform.claude.com/settings/admin-keys). Lihat [analitik untuk pelanggan API](/docs/id/analytics#access-analytics-for-api-customers).

<h4 id="rate-limit-recommendations">
  Rekomendasi batas laju
</h4>

Saat menyiapkan Claude Code untuk tim, pertimbangkan rekomendasi Token Per Minute (TPM) dan Request Per Minute (RPM) per pengguna ini berdasarkan ukuran organisasi Anda:

| Ukuran tim       | TPM per pengguna | RPM per pengguna |
| ---------------- | ---------------- | ---------------- |
| 1-5 pengguna     | 200k-300k        | 5-7              |
| 5-20 pengguna    | 100k-150k        | 2.5-3.5          |
| 20-50 pengguna   | 50k-75k          | 1.25-1.75        |
| 50-100 pengguna  | 25k-35k          | 0.62-0.87        |
| 100-500 pengguna | 15k-20k          | 0.37-0.47        |
| 500+ pengguna    | 10k-15k          | 0.25-0.35        |

Misalnya, jika Anda memiliki 200 pengguna, Anda mungkin meminta 20k TPM untuk setiap pengguna, atau 4 juta total TPM (200\*20.000 = 4 juta).

TPM per pengguna menurun seiring pertumbuhan ukuran tim karena lebih sedikit pengguna yang cenderung menggunakan Claude Code secara bersamaan di organisasi yang lebih besar. Batas laju ini berlaku di tingkat organisasi, bukan per pengguna individual, yang berarti pengguna individual dapat sementara mengonsumsi lebih dari bagian yang dihitung mereka ketika orang lain tidak secara aktif menggunakan layanan.

<Note>
  Jika Anda mengantisipasi skenario dengan penggunaan bersamaan yang tidak biasa tinggi (seperti sesi pelatihan langsung dengan kelompok besar), Anda mungkin memerlukan alokasi TPM yang lebih tinggi per pengguna.
</Note>

<h3 id="cloud-providers">
  Penyedia cloud
</h3>

Di Amazon Bedrock, Google Cloud's Agent Platform, dan Microsoft Foundry, Claude Code ditagih per token ke akun cloud Anda, dan kontrol pengeluaran berada di konsol penagihan penyedia cloud Anda. Claude Code tidak mengirim metrik dari cloud Anda kembali ke Anthropic, jadi [dashboard analitik](/docs/id/analytics) dan Claude Code Analytics API tidak mencakup penggunaan ini.

Untuk atribusi biaya per pengguna, Anda memiliki tiga opsi:

* **OpenTelemetry**: [ekspor metrik](/docs/id/monitoring-usage) dari mesin setiap pengembang ke tumpukan observabilitas Anda sendiri. Ini memberi Anda penghitungan token per pengguna, biaya, dan aktivitas alat terlepas dari penyedia.
* **Gateway aplikasi Claude**: [gateway aplikasi Claude](/docs/id/claude-apps-gateway) yang di-host sendiri menyediakan atribusi penggunaan per pengguna, metrik OTLP dengan penghitungan token, dan [batas pengeluaran per pengguna](/docs/id/claude-apps-gateway-spend-limits) pada penyedia ini.
* **Gateway LLM**: arahkan semua lalu lintas Claude Code melalui proxy yang melacak pengeluaran per kunci. Beberapa perusahaan besar melaporkan menggunakan [LiteLLM](/docs/id/llm-gateway), alat sumber terbuka yang [melacak pengeluaran berdasarkan kunci](https://docs.litellm.ai/docs/proxy/virtual_keys#tracking-spend). Proyek ini tidak berafiliasi dengan Anthropic dan belum diaudit untuk keamanan.

<h3 id="when-a-developer-asks-about-a-limit">
  Ketika pengembang menanyakan tentang batas
</h3>

Pengembang biasanya membawa pertanyaan batas ke admin mereka, jadi membantu mengetahui batas mana yang mereka capai. Situasi-situasi ini berarti hal yang berbeda:

* **"Anda telah mencapai batas sesi Anda" atau "Anda telah mencapai batas mingguan Anda"**: jendela penggunaan berbasis kursi pada paket berlangganan, dibagikan di semua model, jadi pengembang tidak dapat mengembalikan akses dengan beralih model dengan `/model`. Pesan menunjukkan kapan jendela direset. Setelah pesan khusus model "Anda telah mencapai batas Opus Anda" atau "Anda telah mencapai batas Sonnet Anda", beralih ke model di luar keluarga itu dengan `/model` memang membuat pengembang terus bekerja. Lihat [kesalahan batas penggunaan](/docs/id/errors#youve-hit-your-session-limit). Apa yang dapat dilakukan pengembang sementara itu:
  * Jalankan `/usage-credits` untuk meminta penggunaan di luar tunjangan, jika Anda memiliki [kredit penggunaan](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) diaktifkan.
  * Pada Claude Code v2.1.234 atau lebih baru, [tunggu dan lanjutkan tugas yang terputus secara otomatis setelah reset](/docs/id/interactive-mode#wait-for-a-usage-limit-to-reset); bagian itu mencantumkan kapan Claude Code memulai tunggu sendiri dan kapan pengembang memilihnya dari `/rate-limit-options`. Untuk mengontrol armada Anda apakah Claude Code memulai tunggu itu sendiri, atur [`autoContinueAtUsageLimit`](/docs/id/settings-reference#autocontinueatusagelimit) dalam [pengaturan terkelola](/docs/id/settings#settings-precedence).
* **"Anda telah mencapai batas pengeluaran individual Anda", "batas pengeluaran bulanan organisasi", atau "anggaran bersama tim"**: permintaan pengembang akan ditagih ke kredit penggunaan, dan kredit tersebut telah mencapai batas pengeluaran yang Anda tetapkan. Untuk membiarkan pengembang melanjutkan, buka [**Pengaturan Admin > Penggunaan**](https://claude.ai/admin-settings/usage) dan tingkatkan batas yang dinamai pesan. Ketika pesan juga menyebutkan waktu reset paket, pengembang dapat menunggu sampai saat itu. Lihat [referensi kesalahan](/docs/id/errors#youve-hit-your-monthly-spend-limit) untuk setiap varian.
* **Pesan batas pengeluaran dari [gateway aplikasi Claude](/docs/id/claude-apps-gateway)**: pengembang melampaui batas pengeluaran yang Anda tetapkan pada gateway yang di-host sendiri, dan gateway memblokir permintaan mereka sampai periode direset atau Anda menaikkan batas. Lihat [batas pengeluaran gateway](/docs/id/claude-apps-gateway-spend-limits) untuk batas, jadwal reset, dan pesan yang dilihat pengembang.
* **Peringatan konteks atau auto-compact**: bukan batas penggunaan. Percakapan telah tumbuh dekat dengan [jendela auto-compact](/docs/id/model-config#set-the-auto-compact-window) sesi, ambang batas di mana Claude Code merangkum riwayat yang lebih lama untuk membebaskan ruang. Arahkan pengembang ke [kurangi penggunaan token](#reduce-token-usage).
* **Pengeluaran yang tidak terduga tinggi pada paket API atau penyedia cloud**: biasanya dapat dilacak kembali ke sesi panjang yang tidak pernah dihapus atau Opus yang ditinggalkan sebagai model default. Kebiasaan berdampak tertinggi untuk dibagikan adalah membersihkan antara tugas yang tidak terkait dan mencocokkan model dengan pekerjaan, keduanya tercakup dalam [kurangi penggunaan token](#reduce-token-usage).

<h3 id="agent-team-token-costs">
  Biaya token tim agen
</h3>

[Tim agen](/docs/id/agent-teams) menjalankan beberapa instans Claude Code, masing-masing dengan jendela konteks sendiri. Penggunaan token diskalakan dengan jumlah rekan kerja aktif dan berapa lama masing-masing berjalan.

Untuk menjaga biaya tim agen tetap dapat dikelola:

* Gunakan Sonnet untuk rekan kerja. Ini menyeimbangkan kemampuan dan biaya untuk tugas koordinasi.
* Jaga tim tetap kecil. Setiap rekan kerja menjalankan jendela konteks sendiri, jadi penggunaan token kira-kira sebanding dengan ukuran tim.
* Jaga prompt spawn tetap fokus. Rekan kerja memuat CLAUDE.md, server MCP, dan skills secara otomatis, tetapi semuanya dalam prompt spawn menambah konteks mereka dari awal.
* Bersihkan tim ketika pekerjaan selesai. Setiap rekan kerja aktif terus mengonsumsi token sampai keluar atau sesi berakhir.
* Tim agen dinonaktifkan secara default. Atur `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS=1` di [settings.json](/docs/id/settings) atau lingkungan Anda untuk mengaktifkannya. Lihat [aktifkan tim agen](/docs/id/agent-teams#enable-agent-teams).

<h2 id="reduce-token-usage">
  Kurangi penggunaan token
</h2>

Biaya token diskalakan dengan ukuran konteks: semakin banyak konteks yang diproses Claude, semakin banyak token yang Anda gunakan. Claude Code secara otomatis mengoptimalkan biaya melalui [prompt caching](/docs/id/prompt-caching), yang mengurangi biaya untuk konten berulang seperti prompt sistem, dan auto-compact, yang merangkum riwayat percakapan saat mendekati batas konteks.

Strategi berikut membantu Anda menjaga konteks tetap kecil dan mengurangi biaya per pesan.

<h3 id="manage-context-proactively">
  Kelola konteks secara proaktif
</h3>

Gunakan `/usage` untuk memeriksa penggunaan token Anda saat ini, atau [konfigurasi baris status Anda](/docs/id/statusline#context-window-usage) untuk menampilkannya secara berkelanjutan.

* **Bersihkan antar tugas**: Gunakan `/clear` untuk memulai segar saat beralih ke pekerjaan yang tidak terkait. Konteks basi membuang token pada setiap pesan berikutnya. Gunakan `/rename` sebelum membersihkan sehingga Anda dapat dengan mudah menemukan sesi nanti, kemudian `/resume` untuk kembali ke sana.
* **Tambahkan instruksi compaction kustom**: `/compact Focus on code samples and API usage` memberi tahu Claude apa yang harus dipertahankan selama perangkuman. Dalam sesi baru, `/compact` mencetak `Not enough messages to compact.` karena belum ada riwayat percakapan untuk dirangkum.

Anda juga dapat menyesuaikan perilaku compaction di file CLAUDE.md Anda di root proyek Anda:

```markdown theme={null}
# Compact instructions

When you are using compact, please focus on test output and code changes
```

<h3 id="choose-the-right-model">
  Pilih model yang tepat
</h3>

Sonnet menangani sebagian besar tugas pengkodean dengan baik dan biayanya lebih rendah dari Opus. Cadangkan Opus untuk keputusan arsitektur yang kompleks atau penalaran multi-langkah. Gunakan `/model` untuk beralih model di tengah sesi, atau atur default di `/config`. Sakelar ke Opus juga berlaku untuk [subagents yang mewarisi model sesi Anda](/docs/id/model-config#setting-your-model). Untuk tugas subagent sederhana, tentukan `model: haiku` di [konfigurasi subagent](/docs/id/sub-agents#choose-a-model) Anda.

<h3 id="reduce-mcp-server-overhead">
  Kurangi overhead server MCP
</h3>

Definisi alat MCP adalah [ditunda secara default](/docs/id/mcp#scale-with-mcp-tool-search), jadi hanya nama alat dan instruksi server yang masuk ke konteks sampai Claude menggunakan alat tertentu. Jalankan `/context` untuk melihat apa yang mengonsumsi ruang.

* **Lebih suka alat CLI jika tersedia**: Alat seperti `gh`, `aws`, `gcloud`, dan `sentry-cli` masih lebih efisien konteks daripada server MCP karena mereka tidak menambahkan daftar per-alat apa pun. Claude dapat menjalankan perintah CLI secara langsung.
* **Nonaktifkan server yang tidak digunakan**: Jalankan `/mcp` untuk melihat server yang dikonfigurasi dan nonaktifkan yang tidak Anda gunakan secara aktif.

<h3 id="install-code-intelligence-plugins-for-typed-languages">
  Instal plugin kecerdasan kode untuk bahasa yang diketik
</h3>

[Plugin kecerdasan kode](/docs/id/plugins/code-intelligence) memberi Claude navigasi simbol yang tepat daripada pencarian berbasis teks, mengurangi pembacaan file yang tidak perlu saat menjelajahi kode yang tidak dikenal. Satu panggilan "go to definition" menggantikan apa yang mungkin merupakan grep diikuti dengan membaca beberapa file kandidat. Server bahasa yang diinstal juga melaporkan kesalahan tipe secara otomatis setelah pengeditan, jadi Claude menangkap kesalahan tanpa menjalankan compiler.

<h3 id="offload-processing-to-hooks-and-skills">
  Offload pemrosesan ke hooks dan skills
</h3>

[Hooks](/docs/id/hooks) kustom dapat memproses data sebelum Claude melihatnya. Alih-alih Claude membaca file log 10.000 baris untuk menemukan kesalahan, hook dapat grep untuk `ERROR` dan mengembalikan hanya baris yang cocok, mengurangi konteks dari puluhan ribu token menjadi ratusan.

[Skill](/docs/id/skills) dapat memberi Claude pengetahuan domain sehingga tidak harus menjelajahi. Misalnya, skill "codebase-overview" dapat mendeskripsikan arsitektur proyek Anda, direktori kunci, dan konvensi penamaan. Ketika Claude memanggil skill, ia mendapatkan konteks ini segera daripada menghabiskan token membaca beberapa file untuk memahami struktur.

Misalnya, hook PreToolUse ini memfilter output tes untuk menampilkan hanya kegagalan:

<Tabs>
  <Tab title="settings.json">
    Tambahkan ini ke [settings.json](/docs/id/settings#where-settings-live) Anda untuk menjalankan hook sebelum setiap perintah Bash:

    ```json theme={null}
    {
      "hooks": {
        "PreToolUse": [
          {
            "matcher": "Bash",
            "hooks": [
              {
                "type": "command",
                "command": "~/.claude/hooks/filter-test-output.sh"
              }
            ]
          }
        ]
      }
    }
    ```
  </Tab>

  <Tab title="filter-test-output.sh">
    Hook memanggil skrip ini. Buat folder dengan `mkdir -p ~/.claude/hooks`, simpan skrip di bawah sebagai `~/.claude/hooks/filter-test-output.sh`, dan buat dapat dieksekusi dengan `chmod +x ~/.claude/hooks/filter-test-output.sh`. Ini memeriksa apakah perintah adalah test runner dan memodifikasinya untuk menampilkan hanya kegagalan:

    ```bash theme={null}
    #!/bin/bash
    input=$(cat)
    cmd=$(echo "$input" | jq -r '.tool_input.command')

    # If running tests, filter to show only failures
    if [[ "$cmd" =~ ^(npm test|pytest|go test) ]]; then
      filtered_cmd="$cmd 2>&1 | grep -A 5 -E '(FAIL|ERROR|error:)' | head -100"
      echo "$input" | jq --arg filtered "$filtered_cmd" \
        '{hookSpecificOutput: {hookEventName: "PreToolUse", permissionDecision: "allow", updatedInput: (.tool_input + {command: $filtered})}}'
    else
      echo "{}"
    fi
    ```
  </Tab>
</Tabs>

Untuk memverifikasi pengaturan, jalankan `/hooks` dan periksa bahwa hook muncul di bawah PreToolUse. Anda juga dapat memulai Claude Code dengan `claude --debug-file ./claude-debug.txt` dan minta Claude untuk menjalankan `npm test`. Ketika hook menulis ulang perintah, file log tersebut berisi baris `modified tool input keys` yang mencantumkan `command` dan bidang input Bash lainnya.

<h3 id="move-instructions-from-claude-md-to-skills">
  Pindahkan instruksi dari CLAUDE.md ke skills
</h3>

File [CLAUDE.md](/docs/id/memory) Anda dimuat ke konteks saat awal sesi. Jika berisi instruksi terperinci untuk alur kerja spesifik (seperti ulasan PR atau migrasi database), token tersebut ada bahkan ketika Anda melakukan pekerjaan yang tidak terkait. [Skills](/docs/id/skills) dimuat sesuai permintaan hanya saat dipanggil, jadi memindahkan instruksi khusus ke skills menjaga konteks dasar Anda tetap lebih kecil. Bertujuan untuk menjaga CLAUDE.md di bawah 200 baris dengan hanya menyertakan hal-hal penting.

<h3 id="adjust-extended-thinking">
  Sesuaikan pemikiran yang diperluas
</h3>

Pemikiran yang diperluas diaktifkan secara default karena secara signifikan meningkatkan kinerja pada tugas perencanaan dan penalaran yang kompleks. Token pemikiran ditagih sebagai token output, dan anggaran default dapat mencapai puluhan ribu token per permintaan tergantung pada model.

Untuk tugas yang lebih sederhana di mana penalaran mendalam tidak diperlukan, Anda dapat mengurangi biaya dengan menurunkan [tingkat upaya](/docs/id/model-config#adjust-effort-level) dengan `/effort` atau di `/model`, atau dengan menonaktifkan pemikiran di `/config`. Anda tidak dapat mematikan pemikiran pada Opus 5.5 atau model Fable, yang selalu menggunakan pemikiran yang diperluas.

Pada model dengan [anggaran pemikiran tetap](/docs/id/model-config#adaptive-reasoning-and-fixed-thinking-budgets), Anda juga dapat menurunkan anggaran dengan menetapkan [variabel lingkungan](/docs/id/env-vars) `MAX_THINKING_TOKENS`, misalnya `MAX_THINKING_TOKENS=8000`. Model adaptive-reasoning mengabaikan anggaran bukan nol, jadi gunakan tingkat upaya di sana.

<h3 id="delegate-verbose-operations-to-subagents">
  Delegasikan operasi verbose ke subagents
</h3>

Menjalankan tes, mengambil dokumentasi, atau memproses file log dapat mengonsumsi konteks yang signifikan. Delegasikan ini ke [subagents](/docs/id/sub-agents#isolate-high-volume-operations) sehingga output verbose tetap dalam konteks subagent sementara hanya ringkasan yang kembali ke percakapan utama Anda.

<h3 id="manage-agent-team-costs">
  Kelola biaya tim agen
</h3>

Tim agen menggunakan sekitar 7x lebih banyak token daripada sesi standar ketika rekan kerja berjalan dalam plan mode, karena setiap rekan kerja mempertahankan jendela konteks sendiri dan berjalan sebagai instans Claude terpisah. Jaga tugas tim tetap kecil dan mandiri untuk membatasi penggunaan token per rekan kerja. Lihat [tim agen](/docs/id/agent-teams) untuk detail.

<h3 id="write-specific-prompts">
  Tulis prompt spesifik
</h3>

Permintaan yang tidak jelas seperti "tingkatkan basis kode ini" memicu pemindaian luas. Permintaan spesifik seperti "tambahkan validasi input ke fungsi login di auth.ts" memungkinkan Claude bekerja secara efisien dengan pembacaan file minimal.

<h3 id="work-efficiently-on-complex-tasks">
  Bekerja secara efisien pada tugas yang kompleks
</h3>

Untuk pekerjaan yang lebih lama atau lebih kompleks, kebiasaan ini membantu menghindari token yang terbuang dari mengambil jalan yang salah:

* **Gunakan plan mode untuk tugas yang kompleks**: Tekan Shift+Tab untuk memasuki [plan mode](/docs/id/permission-modes#analyze-before-you-edit-with-plan-mode) sebelum implementasi. Claude menjelajahi basis kode dan mengusulkan pendekatan untuk persetujuan Anda, mencegah pekerjaan ulang yang mahal ketika arah awal salah.
* **Koreksi kursus lebih awal**: Jika Claude mulai menuju arah yang salah, tekan Escape untuk berhenti segera. Gunakan `/rewind` atau tekan dua kali Escape untuk mengembalikan percakapan dan kode ke checkpoint sebelumnya.
* **Berikan target verifikasi**: Sertakan kasus uji, tempel tangkapan layar, atau tentukan output yang diharapkan dalam prompt Anda. Ketika Claude dapat memverifikasi pekerjaan sendiri, ia menangkap masalah sebelum Anda perlu meminta perbaikan.
* **Uji secara bertahap**: Tulis satu file, uji, kemudian lanjutkan. Ini menangkap masalah lebih awal ketika murah untuk diperbaiki.

<h2 id="background-token-usage">
  Penggunaan token latar belakang
</h2>

Claude Code menggunakan token untuk beberapa fungsi latar belakang bahkan saat menganggur:

* **Perangkuman percakapan**: Pekerjaan latar belakang yang merangkum percakapan sebelumnya untuk fitur `claude --resume`
* **Pemrosesan perintah**: Beberapa perintah seperti `/usage` dapat menghasilkan permintaan untuk memeriksa status

Proses latar belakang ini mengonsumsi sejumlah kecil token (biasanya di bawah \$0,04 per sesi) bahkan tanpa interaksi aktif.

Ketika saran prompt aktif, Claude Code juga mengirimkan permintaan singkat ke model yang digunakan sesi Anda setelah Claude merespons, untuk [menyarankan prompt berikutnya Anda](/docs/id/interactive-mode#prompt-suggestions). Permintaan tersebut menggunakan kembali prompt cache percakapan, sehingga sebagian besar merupakan pembacaan cache ditambah beberapa token output. Claude Code [melewatkannya saat akun Anda mendekati atau mencapai batas penggunaan](/docs/id/interactive-mode#when-claude-code-skips-suggestions). Untuk menghentikan permintaan ini, [matikan saran prompt](/docs/id/interactive-mode#turn-prompt-suggestions-off).

<h2 id="why-usage-climbs-in-a-long-session">
  Mengapa penggunaan meningkat dalam sesi yang panjang
</h2>

Sesi yang telah dibuka selama berjam-jam dapat menggunakan jauh lebih banyak batas paket Anda daripada yang disarankan oleh aktivitas Anda, biasanya karena salah satu alasan berikut:

* **Konteks panjang**: Claude Code mengirimkan percakapan lengkap Anda dengan setiap permintaan, dan setiap kali Claude menggunakan tools, ia mengirimkan permintaan lain yang membawa batch hasil tool tersebut. Dengan [prompt caching](/docs/id/prompt-caching), Claude Code membaca ulang riwayat tersebut pada [tingkat token yang di-cache](https://platform.claude.com/docs/en/about-claude/pricing), jadi pertanyaan satu baris dalam sesi yang telah dibuka sepanjang hari masih menarik penggunaan untuk seluruh percakapan. Lihat [Kelola konteks secara proaktif](#manage-context-proactively) untuk cara-cara menjaga konteks Anda tetap kecil
* **Cache misses**: pesan pertama Anda setelah istirahat yang lebih lama dari [cache lifetime](/docs/id/prompt-caching#cache-lifetime) melewatkan cache dan memproses ulang konteks lengkap Anda. Masa hidup adalah satu jam pada langganan dan turun menjadi lima menit setelah Anda menggunakan [usage credits](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans); pada API key atau cloud provider, defaultnya adalah lima menit. Untuk mempertahankan masa hidup satu jam sambil menggunakan usage credits, [pilih TTL sendiri](/docs/id/prompt-caching#choose-the-ttl-yourself). Pada paket Pro dan Max, ketika Anda melanjutkan sesi besar setelah istirahat panjang, Claude Code [menawarkan untuk melanjutkan dari ringkasan](/docs/id/sessions#resume-from-a-summary) sehingga permintaan selanjutnya tidak membawa riwayat lengkap
* **Tugas terjadwal**: [tugas terjadwal](/docs/id/scheduled-tasks) dijalankan pada intervalnya bahkan saat sesi idle, mengirimkan konteks lengkap Anda setiap kali
* **Pesan lintas sesi**: Claude Code mengirimkan [pesan dari salah satu sesi Anda yang lain](/docs/id/cross-session-messaging) sebagai giliran baru ketika sesi ini idle, mengirimkan konteks lengkap Anda setiap kali. Untuk menahan pesan masuk alih-alih mengirimkannya, atur [`crossSessionInbound`](/docs/id/settings-reference#crosssessioninbound) ke `hold`
* **Pemeriksaan tujuan**: sementara pekerjaan latar belakang membuat [tujuan](/docs/id/goal) aktif menunggu, Claude Code [meminta Claude untuk memeriksa pekerjaan tersebut](/docs/id/goal#background-work-defers-evaluation) bahkan ketika sesi idle, memulai giliran baru yang mengirimkan konteks lengkap Anda. Claude Code memulai paling banyak tiga pemeriksaan idle per tujuan di antara prompt Anda. Sebelum v2.1.246, pemeriksaan idle tidak terbatas. Untuk mematikan pemeriksaan, atur [`CLAUDE_CODE_GOAL_CHECKIN_MINUTES`](/docs/id/env-vars) ke `0`. Pemeriksaan idle memerlukan Claude Code v2.1.236 atau lebih baru
* **Rekan tim agen**: setiap [rekan tim](#agent-team-token-costs) aktif terus mengonsumsi token sampai keluar
* **Pemadatan**: `/compact` membaca percakapan yang diringkasnya, jadi [memadatkan konteks besar](/docs/id/prompt-caching#compacting-the-conversation) adalah permintaan besar itu sendiri. Ketika Anda menginginkan awal yang segar alih-alih kontinuitas, `/clear` tidak memerlukan biaya apa pun

Pada paket Pro, Max, Team, atau Enterprise, rincian `/usage` menandai perilaku yang menyumbang 10% atau lebih dari penggunaan terbaru Anda, seperti konteks panjang atau cache misses, masing-masing dengan tip untuk menguranginya.

<h2 id="understanding-changes-in-claude-code-behavior">
  Memahami perubahan dalam perilaku Claude Code
</h2>

Claude Code secara teratur menerima pembaruan yang dapat mengubah cara fitur bekerja, termasuk pelaporan biaya. Jalankan `claude --version` untuk memeriksa versi Anda saat ini.

Untuk pertanyaan penagihan tentang akun spesifik Anda, hubungi dukungan Anthropic melalui messenger dalam produk:

* **Paket langganan** (Pro, Max, Team, Enterprise): masuk di [claude.ai](https://claude.ai), klik inisial Anda di sudut kiri bawah, dan pilih **Get help**
* **Penagihan Console (API)**: masuk di [platform.claude.com](https://platform.claude.com), klik inisial Anda, dan pilih **Get help**

Lihat [How to get support](https://support.claude.com/en/articles/9015913-how-to-get-support) untuk alur lengkap, termasuk siapa yang dapat menghubungi agen manusia di setiap paket.
