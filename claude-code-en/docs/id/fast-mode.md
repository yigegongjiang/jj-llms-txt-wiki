> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Percepat respons dengan mode cepat

> Dapatkan respons Opus yang lebih cepat di Claude Code dengan mengaktifkan mode cepat.

<Note>
  Mode cepat berada dalam [pratinjau penelitian](#research-preview). Fitur, harga, dan ketersediaan dapat berubah berdasarkan umpan balik.
</Note>

Mode cepat adalah konfigurasi kecepatan tinggi untuk Claude Opus, membuat model hingga 2,5x lebih cepat dengan biaya per token yang lebih tinggi. Aktifkan dengan `/fast` ketika Anda membutuhkan kecepatan untuk pekerjaan interaktif seperti iterasi cepat atau debugging langsung, dan nonaktifkan ketika biaya lebih penting daripada latensi.

Mode cepat bukan model yang berbeda. Mode ini menggunakan Claude Opus dengan konfigurasi API berbeda yang memprioritaskan kecepatan daripada efisiensi biaya. Anda mendapatkan kualitas dan kemampuan yang identik dengan respons yang lebih cepat. Mode cepat didukung pada Opus 5.5, Opus 5, dan Opus 4.8. Mode ini tidak tersedia pada Sonnet, Haiku, atau model lainnya.

Opus 4.7 tidak mendukung mode cepat, jadi beralih ke model tersebut mematikan mode cepat. Mode cepat untuk Opus 4.7 sudah usang pada 25 Juni 2026, dan dihapus pada 24 Juli 2026.

Yang perlu diketahui:

* Gunakan `/fast` untuk mengaktifkan mode cepat di Claude Code CLI. Ekstensi [VS Code](/docs/id/vs-code) menawarkan perintah **Toggle fast mode** ketika model yang dipilih mendukung mode cepat. Claude Code menyimpan pengaturan tersebut ke pengaturan [`fastMode`](#toggle-fast-mode) Anda.
* Harga mode cepat per MTok input/output adalah \$8/\$40 pada Opus 5.5 dan \$10/\$50 pada Opus 5 dan Opus 4.8.
* Tersedia untuk pengguna Claude Code pada paket berlangganan (Pro/Max/Team/Enterprise) dan di Claude Console. Organisasi Team dan Enterprise memerlukan Owner untuk mengaktifkannya terlebih dahulu, dan organisasi Console memerlukan akses yang disediakan terlebih dahulu, keduanya dijelaskan di bawah [Persyaratan](#requirements).
* Untuk pengguna Claude Code pada paket berlangganan (Pro/Max/Team/Enterprise), mode cepat tersedia hanya melalui penggunaan kredit dan tidak termasuk dalam batas laju penggunaan berlangganan.

<h2 id="toggle-fast-mode">
  Aktifkan mode cepat
</h2>

Dalam CLI, aktifkan mode cepat dengan salah satu cara berikut:

* Jalankan `/fast`, tekan Space untuk mengaktifkan atau menonaktifkan, kemudian tekan Enter untuk mengonfirmasi
* Atur `"fastMode": true` di [file pengaturan pengguna Anda](/docs/id/settings)

Secara default, mode cepat yang Anda aktifkan dalam sesi interaktif bertahan di seluruh sesi. Anda dapat mengonfigurasi mode cepat untuk disetel ulang setiap sesi. Lihat [require per-session opt-in](#require-per-session-opt-in) untuk detail.

Di luar [sesi cloud](#use-fast-mode-in-cloud-sessions), dalam [mode non-interaktif](/docs/id/headless) dengan flag `-p`, `/fast` hanya berfungsi dalam sesi yang diluncurkan dengan mode cepat dalam nilai [`--settings`](/docs/id/cli-reference#cli-flags) nya, misalnya `claude -p --settings '{"fastMode": true}'`; toggle kemudian berlaku hanya untuk sesi itu dan tidak disimpan sebagai default Anda. Bentuk `-p` memerlukan Claude Code v2.1.205 atau lebih baru. Di tempat lain dalam mode non-interaktif, perintah melaporkan bahwa mode cepat tidak tersedia.

Anda dapat menjalankan `/fast` saat Claude sedang bekerja, dan Claude Code mengalihkan mode cepat tanpa menunggu giliran berakhir. Claude Code menyelesaikan giliran yang sedang berjalan pada kecepatan aslinya, sehingga perubahan kecepatan berlaku mulai dari giliran Anda berikutnya. Jika model Anda saat ini tidak mendukung mode cepat, mengaktifkannya juga beralih ke model Anda, dan Claude Code menggunakan model baru dari permintaan berikutnya dalam giliran itu.

Untuk efisiensi biaya terbaik, aktifkan mode cepat di awal sesi daripada beralih di tengah percakapan. Lihat [understand the cost tradeoff](#understand-the-cost-tradeoff) untuk detail.

Ketika Anda mengaktifkan mode cepat:

* Jika model Anda saat ini tidak mendukung mode cepat, Claude Code beralih ke Opus
* Anda akan melihat pesan konfirmasi: "Fast mode ON"
* Ikon kecil `↯` muncul di sebelah prompt saat mode cepat aktif
* Jalankan `/fast` lagi kapan saja untuk memeriksa apakah mode cepat aktif atau tidak

Opus 5.5 adalah default mode cepat di Claude Code v2.1.280 dan lebih baru. Sebelum v2.1.280, mode cepat default ke Opus 5 dari v2.1.219, ke Opus 4.8 pada v2.1.154 hingga v2.1.218, dan ke Opus 4.7 pada v2.1.142 hingga v2.1.153.

Ketika Anda menonaktifkan mode cepat dengan `/fast` lagi, Anda tetap berada di Opus. Untuk beralih ke model yang berbeda, gunakan `/model`.

<h3 id="switch-models-while-fast-mode-is-on">
  Beralih model saat mode cepat aktif
</h3>

Mode cepat mengikuti peralihan model Anda di kedua arah:

* **Beralih pergi**: ketika Anda beralih ke model yang tidak mendukung mode cepat, Claude Code mematikan mode cepat. Ini termasuk Opus 4.7; sebelum v2.1.221, mode cepat tetap aktif setelah peralihan ke Opus 4.7 dan API menolak permintaan.
* **Beralih kembali**: beralih kembali ke model Opus yang didukung menyalakan mode cepat lagi ketika preferensi mode cepat yang disimpan Anda aktif, preferensi yang sama yang dimulai sesi baru secara default. Peralihan model tidak pernah menyalakan mode cepat untuk sesi yang preferensi yang disimpannya mati, dan dengan [per-session opt-in](#require-per-session-opt-in) yang dikonfigurasi, beralih kembali tidak menyalakannya juga; jalankan `/fast` untuk mengaktifkannya kembali.

Kapan pun peralihan model menyalakan atau mematikan mode cepat, Claude Code menampilkan konfirmasi `Fast mode ON` atau `Fast mode OFF`, dan ikon `↯` muncul saat mode cepat aktif. Ini berlaku apakah Anda beralih dengan `/model`, dengan [`/config model=<model>`](/docs/id/settings), atau dari perangkat yang terhubung melalui [Remote Control](/docs/id/remote-control).

Claude Code mengirim ulang status mode cepat sesi ke perangkat yang terhubung melalui Remote Control setelah peralihan model, koneksi ulang, atau pemeriksaan [availability](#use-fast-mode-behind-proxies-and-llm-gateways) yang gagal.

<h3 id="use-fast-mode-in-cloud-sessions">
  Gunakan mode cepat dalam sesi cloud
</h3>

Mode cepat berfungsi dalam [sesi cloud](/docs/id/claude-code-on-the-web) ketika tersedia di akun Anda, apakah sesi berjalan pada infrastruktur yang dikelola Anthropic atau [runner yang di-host sendiri](/docs/id/self-hosted-environments). Memerlukan Claude Code v2.1.271 atau lebih baru di lingkungan sesi.

Ketik `/fast on` dalam sesi untuk mengaktifkan mode cepat. Itu tetap aktif hanya untuk sesi itu dan tidak disimpan sebagai default Anda. [Persyaratan](#requirements) juga berlaku dalam sesi cloud.

<h2 id="understand-the-cost-tradeoff">
  Pahami pertukaran biaya
</h2>

Mode cepat memiliki harga per-token yang lebih tinggi daripada Opus standar:

| Model    | Input (MTok) | Output (MTok) |
| -------- | ------------ | ------------- |
| Opus 5.5 | \$8          | \$40          |
| Opus 5   | \$10         | \$50          |
| Opus 4.8 | \$10         | \$50          |

Harga mode cepat datar di seluruh jendela konteks 1M token penuh. Untuk tarif Opus standar yang akan dibandingkan, lihat [referensi harga Claude](https://platform.claude.com/docs/id/about-claude/pricing).

Pertama kali Anda mengaktifkan mode cepat dalam percakapan, Anda membayar harga token input tanpa cache mode cepat penuh untuk seluruh konteks percakapan. Semakin dalam Anda berada dalam percakapan, semakin mahal biayanya, jadi mengaktifkan mode cepat dari awal lebih murah. Biaya diterapkan sekali per percakapan, jadi mematikan dan menyalakan kembali mode cepat nanti tidak mengulanginya. Untuk mekanismenya, lihat [bagaimana mode cepat berinteraksi dengan prompt cache](/docs/id/prompt-caching#turning-on-fast-mode).

<h3 id="see-where-fast-mode-spend-appears">
  Lihat di mana pengeluaran mode cepat muncul
</h3>

Anda melihat pengeluaran mode cepat di tempat yang berbeda tergantung pada cara Anda masuk, jadi pertama kali jalankan [`/status`](/docs/id/commands) untuk memeriksa. Jika menampilkan baris `Login method` seperti `Claude Max account`, Anda masuk dengan langganan Claude. Jika menampilkan baris `API key` sebagai gantinya, permintaan Anda ditagih ke organisasi Claude Console.

* **Pro dan Max**: Anda membayar mode cepat dari kredit penggunaan Anda. Buka [**Settings > Usage**](https://claude.ai/settings/usage) di claude.ai, di mana bagian **Usage credits** menunjukkan berapa banyak yang telah Anda keluarkan dalam kredit penggunaan bulan ini. Angka tersebut mencakup mode cepat tetapi tidak memisahkannya secara terpisah.
* **Team dan Enterprise**: organisasi Anda membayar penggunaan mode cepat Anda dari kredit penggunaannya. Untuk melihat pengeluaran kredit penggunaan Anda sendiri, jalankan [`/usage`](/docs/id/costs#check-your-usage-credits-spend). Untuk tempat organisasi Anda melihat pengeluaran tersebut, lihat [Claude untuk Teams dan Enterprise](/docs/id/costs#claude-for-teams-and-enterprise).
* **Claude Console**: organisasi Anda membayar mode cepat bersama dengan sisa penggunaan API-nya. Di halaman Console [Usage](https://platform.claude.com/usage) dan [Cost](https://platform.claude.com/cost), pilih **Speed (Research Preview)** di menu **Group by** untuk memisahkan mode cepat dari penggunaan kecepatan standar. Anda hanya melihat opsi tersebut ketika rentang tanggal yang dipilih mencakup penggunaan mode cepat.

<h2 id="decide-when-to-use-fast-mode">
  Tentukan kapan menggunakan mode cepat
</h2>

Mode cepat terbaik untuk pekerjaan interaktif di mana latensi respons lebih penting daripada biaya:

* Iterasi cepat pada perubahan kode
* Sesi debugging langsung
* Pekerjaan sensitif waktu dengan tenggat waktu ketat

Mode standar lebih baik untuk:

* Tugas otonomi jangka panjang di mana kecepatan kurang penting
* Pemrosesan batch atau pipeline CI/CD
* Beban kerja sensitif biaya

<h3 id="fast-mode-vs-effort-level">
  Mode cepat vs tingkat usaha
</h3>

Mode cepat dan tingkat usaha keduanya mempengaruhi kecepatan respons, tetapi dengan cara yang berbeda:

| Pengaturan                     | Efek                                                                                                  |
| ------------------------------ | ----------------------------------------------------------------------------------------------------- |
| **Mode cepat**                 | Kualitas model yang sama, latensi lebih rendah, biaya lebih tinggi                                    |
| **Tingkat usaha lebih rendah** | Waktu pemikiran lebih sedikit, respons lebih cepat, potensi kualitas lebih rendah pada tugas kompleks |

Anda dapat menggabungkan keduanya: gunakan mode cepat dengan [tingkat usaha](/docs/id/model-config#adjust-effort-level) yang lebih rendah untuk kecepatan maksimal pada tugas yang mudah.

<h2 id="requirements">
  Persyaratan
</h2>

Mode cepat memerlukan semua hal berikut:

* **Hanya API Anthropic atau langganan**: mode cepat tersedia melalui API Konsol Anthropic dan untuk paket langganan Claude menggunakan penggunaan kredit. Mode ini tidak tersedia di Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry, atau Claude Platform di AWS. Organisasi Konsol juga harus memiliki [akses mode cepat yang disediakan](#enable-fast-mode-for-your-organization).
* **Penggunaan kredit diaktifkan untuk paket langganan**: pada paket Pro, Max, Team, atau Enterprise, akun Anda harus memiliki [penggunaan kredit](/docs/id/costs#add-usage-credits-to-your-subscription) diaktifkan, yang memungkinkan penagihan di luar penggunaan yang disertakan dalam paket Anda. Sampai diaktifkan, `/fast` menampilkan "Fast mode requires usage credits". Cara Anda mengaktifkannya tergantung pada paket Anda:
  * Pada Pro dan Max, aktifkan di bagian **Usage credits** dari [**Settings > Usage**](https://claude.ai/settings/usage) di claude.ai, atau jalankan `/usage-credits` untuk membuka halaman tersebut.
  * Pada Team dan Enterprise, anggota dengan akses penagihan mengaktifkannya untuk organisasi di [**Admin settings > Usage**](https://claude.ai/admin-settings/usage), dan anggota tanpa akses menjalankan `/usage-credits` untuk mengirim permintaan kepada admin organisasi.

<Note>
  Penggunaan mode cepat diambil langsung dari penggunaan kredit, bahkan jika Anda memiliki penggunaan yang tersisa di paket Anda.
</Note>

* **Organisasi Konsol berbayar**: akun Claude Konsol tidak menggunakan penggunaan kredit, dan organisasi Anda membayar mode cepat per token bersama dengan penggunaan API lainnya. Pada paket Evaluasi gratis Konsol, `/fast` menampilkan "Fast mode unavailable during evaluation. Please purchase credits." Untuk menghapusnya, beli kredit di [pengaturan penagihan Konsol Anda](https://platform.claude.com/settings/billing).
* **Aktivasi Owner untuk Teams dan Enterprise**: mode cepat dinonaktifkan secara default untuk organisasi Teams dan Enterprise. Seorang Owner harus secara eksplisit [mengaktifkan mode cepat](#enable-fast-mode-for-your-organization) sebelum pengguna dapat mengaksesnya.

<Note>
  Empat pengaturan organisasi dapat memblokir pengaktifan mode cepat dengan `/fast`:

  * **Mode cepat tidak diaktifkan**: jika mode cepat belum diaktifkan untuk organisasi Anda, mengaktifkan mode cepat dengan `/fast` menampilkan "Fast mode has been disabled by your organization."
  * **Mode cepat dimatikan oleh pengaturan yang dikelola**: jika organisasi Anda menerapkan [pengaturan yang dikelola](/docs/id/managed-settings) yang menetapkan [`fastMode: false`](/docs/id/settings-reference#fastmode), mengaktifkan mode cepat dengan `/fast` menampilkan pesan "Fast mode has been disabled by your organization" yang sama.
  * **Opt-in per sesi diperlukan**: pengaturan yang dikelola yang menetapkan [`fastModePerSessionOptIn: true`](#require-per-session-opt-in) menolak `/fast on` dengan pesan yang sama di mana-mana kecuali sesi terminal interaktif.
  * **Model mode cepat tidak diizinkan**: jika daftar allowlist [`availableModels`](/docs/id/model-config#restrict-model-selection) organisasi Anda mengecualikan model Opus mode cepat, mengaktifkannya ditolak dengan "is not in your organization's allowed models". Dalam sesi yang sudah berjalan pada model Opus yang diizinkan yang mendukung mode cepat, `/fast` malah mengaktifkan mode cepat pada model Anda saat ini tanpa beralih model.
</Note>

<h3 id="enable-fast-mode-for-your-organization">
  Aktifkan mode cepat untuk organisasi Anda
</h3>

Tempat Anda mengaktifkan mode cepat tergantung pada produk mana yang digunakan organisasi Anda:

* **Konsol** (pelanggan API): admin mengaktifkannya di [preferensi Claude Code](https://platform.claude.com/claude-code/preferences). Mode cepat berada dalam [pratinjau penelitian](#research-preview), jadi organisasi Anda juga harus memiliki akses mode cepat yang disediakan sebelum permintaan mode cepat berhasil. Untuk mendapatkan akses, hubungi manajer akun Anda atau bergabung dengan daftar tunggu, seperti yang dijelaskan dalam [mode cepat pada API Claude](https://platform.claude.com/docs/en/build-with-claude/fast-mode).

  Tanpa akses yang disediakan, API menolak setiap permintaan mode cepat dengan 429, dan Claude Code memperlakukan setiap penolakan sebagai [batas laju mode cepat](#handle-rate-limits). Tidak seperti cooldown batas laju, penolakan terus berlanjut sampai akses disediakan.
* **Claude AI** (Teams dan Enterprise): Owner mengaktifkannya di [Admin Settings > Claude Code](https://claude.ai/admin-settings/claude-code)

Opsi lain untuk menonaktifkan mode cepat sepenuhnya adalah dengan menetapkan `CLAUDE_CODE_DISABLE_FAST_MODE=1`. Lihat [Variabel lingkungan](/docs/id/env-vars).

<h3 id="use-fast-mode-behind-proxies-and-llm-gateways">
  Gunakan mode cepat di balik proxy dan gateway LLM
</h3>

Sebelum menawarkan mode cepat, Claude Code memeriksa ketersediaan mode cepat organisasi Anda dengan permintaan langsung ke `api.anthropic.com`. Pemeriksaan tidak mengikuti [`ANTHROPIC_BASE_URL`](/docs/id/llm-gateway-connect#set-the-base-url-and-credential), jadi pada jaringan yang merutekan lalu lintas Claude melalui [gateway LLM](/docs/id/llm-gateway) dan memblokir egress langsung ke `api.anthropic.com`, pemeriksaan gagal meskipun permintaan inferensi berfungsi. Pemeriksaan menggunakan [proxy HTTP](/docs/id/network-config#proxy-configuration) yang dikonfigurasi, jadi blokir jaringan hanya gagal pemeriksaan di mana `api.anthropic.com` tidak dapat dijangkau bahkan melalui proxy.

Ketika pemeriksaan gagal, `/fast` melaporkan "Fast mode unavailable due to network connectivity issues", dan permintaan berjalan dengan kecepatan standar, bahkan ketika organisasi Anda memiliki mode cepat diaktifkan. Pemeriksaan yang berhasil di masa lalu terus bekerja dari hasil cache-nya, jadi pemeriksaan yang diblokir sebagian besar mempengaruhi instalasi baru.

Pesan konektivitas yang sama muncul pada jaringan terbuka ketika pemeriksaan mencapai `api.anthropic.com` tetapi menyajikan kredensial yang Anthropic tolak. Sesi yang kunci yang diselesaikan adalah kredensial yang dikeluarkan gateway, disimpan dalam [`ANTHROPIC_API_KEY`](/docs/id/llm-gateway-connect#set-the-base-url-and-credential) atau diproduksi oleh [`apiKeyHelper`](/docs/id/settings-reference#apikeyhelper), mengirim pemeriksaan dengan kunci itu, dan permintaan yang ditolak dilaporkan sebagai kegagalan konektivitas.

Untuk mengembalikan mode cepat, allowlist egress langsung ke `api.anthropic.com` di mana blokir jaringan adalah penyebabnya, atau atur variabel mana pun yang cocok dengan cara pemeriksaan gagal:

* `CLAUDE_CODE_SKIP_FAST_MODE_NETWORK_ERRORS=1` memperlakukan pemeriksaan yang gagal sebagai tersedia dan masih menghormati respons "disabled by your organization". Gunakan ketika jaringan Anda menolak koneksi, atau ketika Anthropic menolak kredensial gateway; allowlisting tidak membantu kasus kredensial, karena tidak ada yang diblokir.
* `CLAUDE_CODE_SKIP_FAST_MODE_ORG_CHECK=1` melewati pemeriksaan sepenuhnya. Gunakan ketika jaringan Anda mencegat permintaan daripada menolaknya.

Dua konfigurasi gateway melaporkan "Fast mode has been disabled by your organization" daripada pesan konektivitas, bahkan ketika organisasi Anda memiliki mode cepat diaktifkan:

* Sesi yang mengautentikasi dengan [`ANTHROPIC_AUTH_TOKEN`](/docs/id/llm-gateway-connect#set-the-base-url-and-credential) saja melewati pemeriksaan: tanpa login claude.ai atau kunci API Anthropic, dan tanpa pemeriksaan yang berhasil di-cache, Claude Code memperlakukan mode cepat sebagai dinonaktifkan oleh organisasi Anda tanpa mengirim permintaan.
* Proxy yang mencegat pemeriksaan dan menjawab dengan halaman miliknya sendiri, misalnya proxy yang menginspeksi TLS mengembalikan halaman blokir HTTP 200, dibaca sebagai respons yang mengatakan organisasi Anda memiliki mode cepat dinonaktifkan.

Dalam kedua kasus, atur `CLAUDE_CODE_SKIP_FAST_MODE_ORG_CHECK=1` untuk mengembalikan mode cepat. `CLAUDE_CODE_SKIP_FAST_MODE_NETWORK_ERRORS` tidak berlaku untuk kedua kasus, karena hanya melewati pemeriksaan yang gagal dan keduanya menghasilkan respons yang dinonaktifkan. Allowlisting egress langsung tidak membantu kasus token pembawa, yang tidak pernah mengirim permintaan.

Variabel hanya mempengaruhi pemeriksaan sisi klien. Ketika organisasi Anda memiliki mode cepat dinonaktifkan, API menolak permintaan mode cepat terlepas dari apakah variabel diatur atau tidak. Penolakan dari API tetap berlaku bahkan dengan variabel lewati yang diatur. Claude Code mencoba kembali permintaan yang ditolak dengan kecepatan standar, mematikan mode cepat, dan `/fast` melaporkan bahwa organisasi Anda telah menonaktifkan mode cepat.

Menetapkan `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC` juga menekan pemeriksaan ketersediaan. Tanpa pemeriksaan yang berhasil di-cache sebelumnya, `/fast` melaporkan "Fast mode is currently unavailable"; kedua variabel lewati mengembalikan mode cepat dalam konfigurasi itu juga.

<h3 id="require-per-session-opt-in">
  Require per-session opt-in
</h3>

Secara default, mode cepat yang diaktifkan pengguna dalam sesi interaktif bertahan di seluruh sesi. Untuk mengubah ini, atur `fastModePerSessionOptIn` ke `true` di file [pengaturan](/docs/id/settings#where-settings-live) apa pun, yang menyebabkan setiap sesi dimulai dengan mode cepat mati dan memerlukan pengguna untuk secara eksplisit mengaktifkannya dengan `/fast`. Owners pada paket [Team](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=fast_mode_teams#team-&-enterprise) atau [Enterprise](https://anthropic.com/contact-sales?utm_source=claude_code\&utm_medium=docs\&utm_content=fast_mode_enterprise) dapat menerapkannya di seluruh organisasi melalui [pengaturan yang dikelola server](/docs/id/server-managed-settings).

```json theme={null}
{
  "fastModePerSessionOptIn": true
}
```

Ini berguna untuk mengontrol biaya di organisasi di mana pengguna menjalankan beberapa sesi bersamaan. Preferensi mode cepat pengguna masih disimpan, jadi menghapus pengaturan ini mengembalikan perilaku persisten default.

Ketika pengaturan yang dikelola menetapkan kunci, `/fast on` hanya berfungsi dalam sesi terminal interaktif. Di tempat lain, termasuk [mode non-interaktif](/docs/id/headless), [ekstensi VS Code](/docs/id/vs-code), dan [sesi cloud](#use-fast-mode-in-cloud-sessions), ditolak dengan pesan bahwa organisasi Anda telah menonaktifkan mode cepat.

<h2 id="handle-rate-limits">
  Tangani batas laju
</h2>

Mode cepat memiliki batas laju terpisah dari Opus standar. Semua model Opus yang didukung berbagi satu pool batas laju mode cepat: penggunaan pada salah satu dari mereka menarik dari batas yang sama. Ketika Anda mencapai batas laju mode cepat:

1. Mode cepat secara otomatis kembali ke kecepatan standar
2. Ikon `↯` berubah menjadi abu-abu untuk menunjukkan cooldown
3. Anda terus bekerja dengan kecepatan dan harga standar
4. Ketika cooldown berakhir, mode cepat secara otomatis diaktifkan kembali

Untuk menonaktifkan mode cepat secara manual daripada menunggu cooldown, jalankan `/fast` lagi.

Jika Anda kehabisan kredit penggunaan di tengah sesi, Claude Code mencoba ulang setiap permintaan mode cepat yang ditolak dengan kecepatan dan harga standar, sehingga Anda terus bekerja, dan tidak ada cooldown. Cara Anda melihat penolakan tergantung pada jenis sesi:

* Dalam sesi interaktif, Claude Code menampilkan notifikasi "Fast mode disabled · usage credits exhausted" dan mematikan mode cepat untuk sisa sesi. Preferensi mode cepat yang disimpan Anda tidak berubah; jalankan `/fast` untuk menghidupkan mode cepat kembali.
* Dalam [mode non-interaktif](/docs/id/headless) dengan `--output-format stream-json`, dan melalui Agent SDK, Claude Code memancarkan teks yang sama pada aliran pesan sebagai pesan `system` dengan subtipe `notification`, sekali per giliran saat Anda kehabisan kredit penggunaan. Mode cepat tetap aktif. Memerlukan Claude Code v2.1.221 atau lebih baru.

<h2 id="research-preview">
  Pratinjau penelitian
</h2>

Mode cepat adalah fitur pratinjau penelitian. Ini berarti:

* Fitur dapat berubah berdasarkan umpan balik
* Ketersediaan dan harga dapat berubah
* Konfigurasi API yang mendasar dapat berkembang

Laporkan masalah atau umpan balik melalui saluran dukungan Anthropic biasa Anda.

<h2 id="see-also">
  Lihat juga
</h2>

* [Konfigurasi model](/docs/id/model-config): beralih model dan sesuaikan tingkat usaha
* [Kelola biaya secara efektif](/docs/id/costs): lacak penggunaan token dan kurangi biaya
* [Konfigurasi baris status](/docs/id/statusline): tampilkan informasi model dan konteks
