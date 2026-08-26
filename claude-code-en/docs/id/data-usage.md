> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Penggunaan data

> Pelajari kebijakan penggunaan data Anthropic untuk Claude

<h2 id="data-policies">
  Kebijakan data
</h2>

<h3 id="data-training-policy">
  Kebijakan pelatihan data
</h3>

**Pengguna konsumen (paket Free, Pro, dan Max)**:
Kami memberi Anda pilihan untuk mengizinkan data Anda digunakan untuk meningkatkan model Claude di masa depan. Kami akan melatih model baru menggunakan data dari akun Free, Pro, dan Max ketika pengaturan ini aktif (termasuk ketika Anda menggunakan Claude Code dari akun-akun ini).

**Pengguna komersial**: (paket Team dan Enterprise, API, platform pihak ketiga, dan Claude Gov) mempertahankan kebijakan yang ada: Anthropic tidak melatih model generatif menggunakan kode atau prompt yang dikirim ke Claude Code berdasarkan syarat komersial, kecuali pelanggan telah memilih untuk memberikan data mereka kepada kami untuk peningkatan model (misalnya, [Program Mitra Pengembang](https://support.claude.com/id/articles/11174108-about-the-development-partner-program)).

<h3 id="development-partner-program">
  Program Mitra Pengembang
</h3>

Jika Anda secara eksplisit memilih untuk memberikan materi kepada kami untuk dilatih, seperti melalui [Program Mitra Pengembang](https://support.claude.com/id/articles/11174108-about-the-development-partner-program), kami dapat menggunakan materi tersebut untuk melatih model kami. Admin organisasi dapat secara tegas memilih untuk bergabung dengan Program Mitra Pengembang untuk organisasi mereka. Perhatikan bahwa program ini hanya tersedia untuk API pihak pertama Anthropic, dan bukan untuk pengguna Amazon Bedrock atau Google Cloud's Agent Platform.

<h3 id="feedback-using-the-/feedback-command">
  Umpan balik menggunakan perintah `/feedback`
</h3>

Jika Anda memilih untuk mengirimkan umpan balik kepada kami tentang Claude Code menggunakan perintah `/feedback`, kami dapat menggunakan umpan balik Anda untuk meningkatkan produk dan layanan kami. Transkrip yang dibagikan melalui `/feedback` disimpan selama 5 tahun.

<h3 id="session-quality-surveys">
  Survei kualitas sesi
</h3>

Ketika Anda melihat prompt "Bagaimana Claude melakukan ini di sesi ini?" di Claude Code, merespons survei ini, termasuk memilih "Abaikan", hanya peringkat Anda yang dicatat. Kami tidak mengumpulkan atau menyimpan transkrip percakapan, input, output, atau data sesi lainnya sebagai bagian dari prompt penilaian itu sendiri. Tidak seperti umpan balik jempol ke atas/ke bawah atau laporan `/feedback`, survei kualitas sesi ini adalah metrik kepuasan produk sederhana.

Setelah prompt penilaian, Anda mungkin melihat pertanyaan tindak lanjut terpisah yang menanyakan "Dapatkah Anthropic melihat transkrip sesi Anda untuk membantu kami meningkatkan Claude Code?". Ini adalah langkah kedua opsional yang berbeda dari penilaian:

* **Ya**: mengunggah transkrip percakapan Anda, transkrip subagen apa pun, dan file log sesi mentah dari disk ke Anthropic. Pola kunci API dan token yang dikenal diredaksi sebelum pengunggahan. Kode sumber, konten file, dan konten percakapan lainnya diunggah apa adanya. Transkrip yang dibagikan disimpan hingga 6 bulan. Di Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry, dan sesi [gateway aplikasi Claude](/docs/id/claude-apps-gateway) yang masuk, Ya menulis payload yang sama ke arsip lokal di bawah `~/.claude/feedback-bundles/` alih-alih mengunggah; tidak ada yang meninggalkan mesin Anda sampai Anda meneruskan file tersebut.
* **Tidak**: menolak tanpa mengirim apa pun
* **Jangan tanya lagi**: menolak dan menghentikan pertanyaan tindak lanjut ini agar tidak muncul di sesi mendatang

Tidak ada yang diunggah kecuali Anda secara eksplisit memilih **Ya**. Organisasi dengan [retensi data nol](/docs/id/zero-data-retention), atau di mana umpan balik produk dinonaktifkan oleh kebijakan organisasi, atau di mana `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC` diatur, tidak pernah melihat pertanyaan tindak lanjut ini. Respons Anda terhadap survei ini, termasuk transkrip sesi yang dikirimkan setelah prompt penilaian, tidak mempengaruhi preferensi pelatihan data Anda dan tidak dapat digunakan untuk melatih model AI kami.

Untuk menonaktifkan survei ini, atur `CLAUDE_CODE_DISABLE_FEEDBACK_SURVEY=1`. Survei juga dinonaktifkan ketika `DISABLE_TELEMETRY`, `DO_NOT_TRACK`, atau `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC` diatur. Organisasi yang memblokir lalu lintas nonessensial tetapi menangkap respons survei melalui [pengumpul OpenTelemetry](/docs/id/monitoring-usage) mereka sendiri dapat memilih survei kembali dengan mengatur `CLAUDE_CODE_ENABLE_FEEDBACK_SURVEY_FOR_OTEL=1`. Survei kemudian mencatat peringkat ke pengumpul yang dikonfigurasi saja. Tindak lanjut berbagi transkrip dan semua lalu lintas umpan balik lainnya yang terikat ke Anthropic tetap dinonaktifkan. Untuk mengontrol frekuensi alih-alih menonaktifkan, atur [`feedbackSurveyRate`](/docs/id/settings#available-settings) dalam file pengaturan Anda ke probabilitas antara `0` dan `1`.

<h3 id="data-retention">
  Retensi data
</h3>

Anthropic menyimpan data Claude Code berdasarkan jenis akun dan preferensi Anda.

**Pengguna konsumen (paket Free, Pro, dan Max)**:

* Pengguna yang mengizinkan penggunaan data untuk peningkatan model: periode retensi 5 tahun untuk mendukung pengembangan model dan peningkatan keamanan
* Pengguna yang tidak mengizinkan penggunaan data untuk peningkatan model: periode retensi 30 hari
* Pengaturan privasi dapat diubah kapan saja di [claude.ai/settings/data-privacy-controls](https://claude.ai/settings/data-privacy-controls).

**Pengguna komersial (Team, Enterprise, dan API)**:

* Standar: periode retensi 30 hari
* [Retensi data nol](/docs/id/zero-data-retention): tersedia untuk Claude Code di Claude untuk Enterprise. ZDR tidak termasuk dalam paket Enterprise standar; diaktifkan berdasarkan per-organisasi oleh tim akun Anda setelah mengkonfirmasi kelayakan
* Penyimpanan lokal: klien Claude Code menyimpan transkrip sesi secara lokal dalam plaintext di bawah `~/.claude/projects/` selama 30 hari secara default untuk memungkinkan pemulihan sesi. Sesuaikan periode dengan `cleanupPeriodDays`. Lihat [data aplikasi](/docs/id/claude-directory#application-data) untuk apa yang disimpan dan cara menghapusnya.

Anda dapat menghapus sesi Claude Code individual di web kapan saja. Menghapus sesi secara permanen menghapus data peristiwa sesi. Untuk instruksi tentang cara menghapus sesi, lihat [Menghapus sesi](/docs/id/claude-code-on-the-web#delete-sessions).

Pelajari lebih lanjut tentang praktik retensi data di [Pusat Privasi](https://privacy.anthropic.com/) kami.

Untuk detail lengkap, silakan tinjau [Syarat Layanan Komersial](https://www.anthropic.com/legal/commercial-terms) kami (untuk pengguna Team, Enterprise, dan API) atau [Syarat Konsumen](https://www.anthropic.com/legal/consumer-terms) (untuk pengguna Free, Pro, dan Max) dan [Kebijakan Privasi](https://www.anthropic.com/legal/privacy).

<h2 id="data-access">
  Akses data
</h2>

Untuk semua pengguna pihak pertama, Anda dapat mempelajari lebih lanjut tentang data apa yang dicatat untuk [Claude Code lokal](#local-claude-code-data-flow-and-dependencies) dan [Claude Code jarak jauh](#cloud-execution-data-flow-and-dependencies). Sesi [Remote Control](/docs/id/remote-control) mengikuti alur data lokal karena semua eksekusi terjadi di mesin Anda; saat terhubung, transkrip sesi juga disimpan di server Anthropic untuk menyinkronkan percakapan di seluruh perangkat, seperti yang dijelaskan dalam [Koneksi dan keamanan](/docs/id/remote-control#connection-and-security). Perhatikan untuk Claude Code jarak jauh, Claude mengakses repositori tempat Anda memulai sesi Claude Code Anda. Claude tidak mengakses repositori yang telah Anda hubungkan tetapi belum memulai sesi di dalamnya.

<h2 id="local-claude-code-data-flow-and-dependencies">
  Claude Code Lokal: Alur data dan dependensi
</h2>

Diagram di bawah menunjukkan bagaimana Claude Code terhubung ke layanan eksternal selama instalasi dan operasi normal. Garis solid menunjukkan koneksi yang diperlukan, sementara garis putus-putus mewakili alur data opsional atau yang dimulai pengguna.

<img src="https://mintcdn.com/claude-code/YR4DRZyI3CdsXkiT/images/claude-code-data-flow.svg?fit=max&auto=format&n=YR4DRZyI3CdsXkiT&q=85&s=2846ea92cfc2297b8620c31c82b482ad" alt="Diagram menunjukkan koneksi eksternal Claude Code: install/update terhubung ke server distribusi, dan permintaan pengguna terhubung ke Anthropic Console auth dan public-api, dengan alur telemetri opsional yang membawa metrik dan laporan kesalahan ke Anthropic dan layanan pihak ketiga. Umpan balik yang dikirim dengan /feedback masuk ke Google Cloud Storage dan secara opsional membuat masalah GitHub" width="720" height="520" data-path="images/claude-code-data-flow.svg" />

Claude Code berjalan secara lokal. Untuk berinteraksi dengan LLM, Claude Code mengirimkan data melalui jaringan. Data ini mencakup semua prompt pengguna dan output model, dienkripsi dalam transit melalui TLS 1.2+. Claude Code kompatibel dengan sebagian besar VPN dan proxy LLM populer.

Enkripsi saat istirahat tergantung pada penyedia model Anda:

| Penyedia                      | Enkripsi saat istirahat                                                                                                                         |
| ----------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------- |
| Anthropic API                 | Enkripsi disk tingkat infrastruktur (AES-256). Aktifkan [Zero Data Retention](/docs/id/zero-data-retention) untuk tidak ada persistensi sisi server. |
| Amazon Bedrock                | AES-256 dengan kunci yang dikelola AWS. Kunci yang dikelola pelanggan tersedia melalui AWS KMS.                                                 |
| Google Cloud's Agent Platform | Kunci enkripsi yang dikelola Google. CMEK tersedia.                                                                                             |
| Microsoft Foundry             | Permintaan dialihkan ke infrastruktur Anthropic dengan enkripsi disk AES-256.                                                                   |

Claude Code dibangun di atas API Anthropic. Untuk detail mengenai kontrol keamanan API, termasuk prosedur logging API, lihat artefak kepatuhan di [Pusat Kepercayaan Anthropic](https://trust.anthropic.com).

<h3 id="cloud-execution-data-flow-and-dependencies">
  Eksekusi cloud: Alur data dan dependensi
</h3>

Saat menggunakan [Claude Code di web](/docs/id/claude-code-on-the-web), sesi berjalan di mesin virtual yang dikelola Anthropic alih-alih secara lokal. Di lingkungan cloud:

* **Penyimpanan kode dan data:** Repositori Anda diklon ke VM terisolasi. Kode dan data sesi tunduk pada kebijakan retensi dan penggunaan untuk jenis akun Anda (lihat bagian Retensi data di atas)
* **Kredensial:** Autentikasi GitHub ditangani melalui proxy aman; kredensial GitHub Anda tidak pernah memasuki sandbox
* **Lalu lintas jaringan:** Semua lalu lintas keluar melewati proxy keamanan untuk logging audit dan pencegahan penyalahgunaan
* **Data sesi:** Prompt, perubahan kode, dan output mengikuti kebijakan data yang sama dengan penggunaan Claude Code lokal

Untuk detail keamanan tentang eksekusi cloud, lihat [Keamanan](/docs/id/security#cloud-execution-security).

<h2 id="telemetry-services">
  Layanan telemetri
</h2>

Claude Code mengirimkan dua jenis telemetri operasional: metrik penggunaan dan laporan kesalahan. Anda dapat mematikan masing-masing secara individual dengan variabel lingkungan di bawah ini, atau menonaktifkan semua lalu lintas non-esensial sekaligus dengan menetapkan `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`.

**Metrik**: latensi, keandalan, dan pola penggunaan, dikirim ke Anthropic dan ke infrastruktur logging pihak ketiga melalui TLS. Metrik tidak pernah mencakup kode, prompt, atau jalur file Anda. Atur `DISABLE_TELEMETRY=1` untuk menolak.

**Laporan kesalahan**: pesan kesalahan dan stack trace dari internal Claude Code sendiri, dikirim ke layanan pelacakan kesalahan pihak ketiga melalui TLS. Claude Code menyunting pola rahasia yang diketahui, jalur file, alamat email, dan informasi pribadi lainnya sebelum apa pun meninggalkan mesin Anda. Atur `DISABLE_ERROR_REPORTING=1` untuk menolak.

Pelaporan kesalahan hanya aktif ketika semua hal berikut berlaku:

* Anda masuk dengan langganan Claude Pro atau Max
* Anda menjalankan Claude Code v2.1.198 atau lebih baru
* Anda terhubung langsung ke Claude API
* Organisasi Anda tidak memiliki perjanjian retensi data nol atau HIPAA

Ketika Anda menjalankan perintah `/feedback`, salinan riwayat percakapan lengkap Anda termasuk kode dikirim ke Anthropic. Sebelum mengirimkan, Anda memilih berapa banyak riwayat yang akan disertakan: sesi saat ini saja, yang merupakan default, atau juga sesi lain dari proyek yang sama selama 24 jam atau 7 hari terakhir. Data dienkripsi dalam transit melalui TLS dan disimpan di Google Cloud Storage, yang mengenkripsi data yang disimpan saat istirahat secara default. Secara opsional, masalah GitHub dibuat di repositori publik. Untuk menolak, atur variabel lingkungan `DISABLE_FEEDBACK_COMMAND` ke `1`.

Ketika Anda menggunakan penyedia pihak ketiga seperti Amazon Bedrock atau Platform Agen Google Cloud, atau tidak memiliki kredensial Anthropic yang dikonfigurasi, `/feedback` menulis laporan ke arsip lokal di bawah `~/.claude/feedback-bundles/` alih-alih mengirimkannya ke Anthropic. Pola kunci API dan token yang diketahui dihapus sebelum arsip ditulis. Tidak ada yang meninggalkan mesin Anda sampai Anda mengirim file tersebut ke perwakilan akun Anthropic Anda atau melampirkannya ke permintaan dukungan.

<h2 id="default-behaviors-by-api-provider">
  Perilaku default menurut penyedia API
</h2>

Secara default, pelaporan kesalahan, telemetri, dan pelaporan bug dinonaktifkan saat menggunakan Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry, atau Claude Platform on AWS. Survei kualitas sesi dan pemeriksaan keamanan domain WebFetch adalah pengecualian dan berjalan terlepas dari penyedia. Pada sesi [gateway aplikasi Claude](/docs/id/claude-apps-gateway) yang masuk, analitik penggunaan, pelaporan kesalahan, dan penilaian survei ke Anthropic dinonaktifkan oleh kredensial gateway itu sendiri, tanpa pengaturan untuk mengaktifkannya kembali. Anda dapat menolak semua lalu lintas non-esensial, termasuk survei, sekaligus dengan mengatur `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`. Variabel ini tidak mempengaruhi pemeriksaan WebFetch, yang memiliki opt-out tersendiri. Berikut adalah perilaku default lengkapnya:

| Layanan                                  | Claude API                                                                                                                 | Google Cloud's Agent Platform API                                                                  | Amazon Bedrock API                                                                                 | Microsoft Foundry API                                                                              | Claude Platform on AWS                                                                             |
| ---------------------------------------- | -------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------- |
| **Metrics**                              | Default aktif.<br />`DISABLE_TELEMETRY=1` untuk menonaktifkan.                                                             | Default nonaktif.<br />`CLAUDE_CODE_USE_VERTEX` harus 1.                                           | Default nonaktif.<br />`CLAUDE_CODE_USE_BEDROCK` harus 1.                                          | Default nonaktif.<br />`CLAUDE_CODE_USE_FOUNDRY` harus 1.                                          | Default nonaktif.<br />`CLAUDE_CODE_USE_ANTHROPIC_AWS` harus 1.                                    |
| **Error reports**                        | Aktif untuk sign-in Pro dan Max pada v2.1.198+, sebaliknya nonaktif.<br />`DISABLE_ERROR_REPORTING=1` untuk menonaktifkan. | Default nonaktif.<br />`CLAUDE_CODE_USE_VERTEX` harus 1.                                           | Default nonaktif.<br />`CLAUDE_CODE_USE_BEDROCK` harus 1.                                          | Default nonaktif.<br />`CLAUDE_CODE_USE_FOUNDRY` harus 1.                                          | Default nonaktif.<br />`CLAUDE_CODE_USE_ANTHROPIC_AWS` harus 1.                                    |
| **Claude API (laporan `/feedback`)**     | Default aktif.<br />`DISABLE_FEEDBACK_COMMAND=1` untuk menonaktifkan.                                                      | Default nonaktif.<br />`CLAUDE_CODE_USE_VERTEX` harus 1.                                           | Default nonaktif.<br />`CLAUDE_CODE_USE_BEDROCK` harus 1.                                          | Default nonaktif.<br />`CLAUDE_CODE_USE_FOUNDRY` harus 1.                                          | Default nonaktif.<br />`CLAUDE_CODE_USE_ANTHROPIC_AWS` harus 1.                                    |
| **Survei kualitas sesi**                 | Default aktif.<br />`CLAUDE_CODE_DISABLE_FEEDBACK_SURVEY=1` untuk menonaktifkan.                                           | Default aktif.<br />`CLAUDE_CODE_DISABLE_FEEDBACK_SURVEY=1` untuk menonaktifkan.                   | Default aktif.<br />`CLAUDE_CODE_DISABLE_FEEDBACK_SURVEY=1` untuk menonaktifkan.                   | Default aktif.<br />`CLAUDE_CODE_DISABLE_FEEDBACK_SURVEY=1` untuk menonaktifkan.                   | Default aktif.<br />`CLAUDE_CODE_DISABLE_FEEDBACK_SURVEY=1` untuk menonaktifkan.                   |
| **Pemeriksaan keamanan domain WebFetch** | Default aktif.<br />`skipWebFetchPreflight: true` di [settings](/docs/id/settings) untuk menonaktifkan.                         | Default aktif.<br />`skipWebFetchPreflight: true` di [settings](/docs/id/settings) untuk menonaktifkan. | Default aktif.<br />`skipWebFetchPreflight: true` di [settings](/docs/id/settings) untuk menonaktifkan. | Default aktif.<br />`skipWebFetchPreflight: true` di [settings](/docs/id/settings) untuk menonaktifkan. | Default aktif.<br />`skipWebFetchPreflight: true` di [settings](/docs/id/settings) untuk menonaktifkan. |

Semua variabel lingkungan dapat diperiksa ke dalam `settings.json` (lihat [referensi settings](/docs/id/settings)).

Mulai dari v2.1.126, ketika platform host mengatur `CLAUDE_CODE_PROVIDER_MANAGED_BY_HOST`, metrics default aktif untuk Google Cloud's Agent Platform, Amazon Bedrock, dan Microsoft Foundry, dan mengikuti opt-out standar `DISABLE_TELEMETRY`. Pelaporan kesalahan dan laporan `/feedback` tetap nonaktif secara default pada penyedia tersebut.

<h3 id="webfetch-domain-safety-check">
  Pemeriksaan keamanan domain WebFetch
</h3>

Sebelum mengambil URL, alat WebFetch mengirimkan nama host yang diminta ke `api.anthropic.com` untuk memeriksanya terhadap daftar blocklist keamanan yang dikelola oleh Anthropic. Hanya nama host yang dikirim, bukan URL lengkap, jalur, atau konten halaman. Hasil disimpan dalam cache per nama host selama lima menit.

Pemeriksaan ini berjalan terlepas dari penyedia model mana yang Anda gunakan dan tidak dipengaruhi oleh `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`. Jika jaringan Anda memblokir `api.anthropic.com`, permintaan WebFetch gagal sampai Anda memungkinkan daftar domain atau mengatur `skipWebFetchPreflight: true` di [settings](/docs/id/settings). Menonaktifkan pemeriksaan berarti WebFetch mencoba mengambil URL apa pun tanpa berkonsultasi dengan daftar blocklist, jadi gabungkan dengan [aturan izin `WebFetch`](/docs/id/permissions#webfetch) jika Anda perlu membatasi domain mana yang dapat diakses Claude.
