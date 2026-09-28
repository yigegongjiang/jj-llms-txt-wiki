> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Retensi data nol

> Pelajari tentang Zero Data Retention (ZDR) untuk Claude Code, tersedia untuk akun yang memenuhi syarat di Claude for Enterprise, termasuk cakupan, fitur yang dinonaktifkan, dan cara meminta pengaktifan.

Zero Data Retention (ZDR) untuk Claude Code tersedia untuk akun yang memenuhi syarat di Claude for Enterprise. Ketika ZDR diaktifkan, prompt dan respons model yang dihasilkan selama sesi Claude Code diproses secara real-time dan tidak disimpan oleh Anthropic setelah respons dikembalikan, kecuali jika diperlukan untuk mematuhi hukum atau memerangi penyalahgunaan.

<Note>
  ZDR tidak termasuk dalam paket Claude for Enterprise standar dan tidak dapat diaktifkan dari pengaturan admin Anda. ZDR tersedia untuk akun yang memenuhi syarat dan memerlukan pengaktifan terpisah oleh Anthropic. Jika organisasi Anda memerlukan ZDR, [hubungi penjualan](https://www.anthropic.com/contact-sales?utm_source=claude_code\&utm_medium=docs\&utm_content=zero_data_retention_request) atau tim akun Anthropic Anda untuk mengonfirmasi kelayakan.
</Note>

ZDR di Claude for Enterprise memberikan pelanggan enterprise kemampuan untuk menggunakan Claude Code dengan retensi data nol dan mengakses kemampuan administratif:

* Kontrol biaya per pengguna
* Dashboard [Analytics](/docs/id/analytics)
* [Server-managed settings](/docs/id/server-managed-settings)
* Audit logs

ZDR untuk Claude Code di Claude for Enterprise hanya berlaku untuk platform langsung Anthropic. Untuk penerapan Claude di Amazon Bedrock, Google Cloud's Agent Platform, atau Microsoft Foundry, lihat kebijakan retensi data platform tersebut.

<h2 id="zdr-scope">
  Cakupan ZDR
</h2>

ZDR mencakup inferensi Claude Code di Claude for Enterprise.

<Warning>
  ZDR diaktifkan berdasarkan per-organisasi. Setiap organisasi baru memerlukan ZDR untuk diaktifkan secara terpisah oleh tim akun Anthropic Anda. ZDR tidak secara otomatis berlaku untuk organisasi baru yang dibuat di bawah akun yang sama. Hubungi tim akun Anda untuk mengaktifkan ZDR untuk organisasi baru apa pun.
</Warning>

<h3 id="route-claude-code-traffic-to-your-zdr-organization">
  Arahkan lalu lintas Claude Code ke organisasi ZDR Anda
</h3>

ZDR berlaku untuk permintaan yang mengautentikasi ke organisasi yang diaktifkan ZDR. Jika pengembang masuk ke Claude Code dengan akun pribadi atau dengan kunci API dari organisasi yang berbeda, sesi tersebut tidak tercakup. Untuk memerlukan bahwa login claude.ai pengembang milik organisasi ZDR Anda, terapkan pengaturan terkelola `forceLoginMethod` dan `forceLoginOrgUUID`; lihat [Batasi login ke organisasi Anda](/docs/id/authentication#restrict-login-to-your-organization), yang juga menjelaskan bagaimana kunci ini memperlakukan login Claude Console.

<h3 id="what-zdr-covers">
  Apa yang dicakup ZDR
</h3>

ZDR mencakup panggilan inferensi model yang dilakukan melalui Claude Code di Claude for Enterprise. Ketika Anda menggunakan Claude Code di terminal Anda, prompt yang Anda kirim dan respons yang dihasilkan Claude tidak disimpan oleh Anthropic. Ini berlaku untuk setiap model yang tersedia untuk organisasi ZDR Anda. Beberapa model memerlukan retensi data secara default; lihat [Ketersediaan model di bawah ZDR](#model-availability-under-zdr).

<h3 id="what-zdr-does-not-cover">
  Apa yang tidak dicakup ZDR
</h3>

ZDR tidak berlaku untuk hal-hal berikut, bahkan untuk organisasi dengan ZDR diaktifkan. Fitur-fitur ini mengikuti [kebijakan retensi data standar](/docs/id/data-usage#data-retention):

| Fitur                        | Detail                                                                                                                                                                                                                                                             |
| ---------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Chat di claude.ai            | Percakapan chat melalui antarmuka web Claude for Enterprise tidak dicakup oleh ZDR.                                                                                                                                                                                |
| Cowork                       | Sesi Cowork tidak dicakup oleh ZDR.                                                                                                                                                                                                                                |
| Claude Code Analytics        | Tidak menyimpan prompt atau respons model, tetapi mengumpulkan metadata produktivitas seperti email akun dan statistik penggunaan. Metrik kontribusi tidak tersedia untuk organisasi ZDR; [dashboard analytics](/docs/id/analytics) menampilkan metrik penggunaan saja. |
| Manajemen pengguna dan kursi | Data administratif seperti email akun dan penugasan kursi disimpan di bawah kebijakan standar.                                                                                                                                                                     |
| Integrasi pihak ketiga       | Data yang diproses oleh alat pihak ketiga, MCP servers, atau integrasi eksternal lainnya tidak dicakup oleh ZDR. Tinjau praktik penanganan data layanan tersebut secara independen.                                                                                |

<h2 id="features-disabled-under-zdr">
  Fitur yang dinonaktifkan di bawah ZDR
</h2>

Ketika ZDR diaktifkan untuk organisasi Claude Code di Claude for Enterprise, fitur-fitur tertentu yang memerlukan penyimpanan prompt atau completion secara otomatis dinonaktifkan di tingkat backend:

| Fitur                                                                                                                   | Alasan                                                                                                     |
| ----------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------- |
| [Cloud sessions](/docs/id/claude-code-on-the-web), termasuk yang dimulai dari [aplikasi Desktop](/docs/id/desktop#cloud-sessions) | Memerlukan penyimpanan data sesi di sisi server, termasuk riwayat percakapan dengan prompt dan completion. |
| [Claude Tag](https://claude.com/docs/claude-tag)                                                                        | Mempertahankan memori saluran dan transkrip sesi.                                                          |
| [Artifacts](/docs/id/artifacts)                                                                                              | Memerlukan penyimpanan konten halaman yang dipublikasikan di infrastruktur yang dioperasikan Anthropic.    |
| Pengiriman umpan balik (`/feedback`, `/bug`, `/share`)                                                                  | Mengirimkan umpan balik mengirimkan data percakapan ke Anthropic.                                          |
| [Remote Control](/docs/id/remote-control)                                                                                    | Menyimpan transkrip sesi di server Anthropic untuk menyinkronkan percakapan di seluruh perangkat.          |

Fitur-fitur ini diblokir di backend terlepas dari tampilan sisi klien. Jika Anda melihat fitur yang dinonaktifkan di terminal Claude Code selama startup, mencoba menggunakannya mengembalikan kesalahan yang menunjukkan kebijakan organisasi tidak memungkinkan tindakan tersebut.

Fitur-fitur di masa depan juga dapat dinonaktifkan jika memerlukan penyimpanan prompt atau completion.

<h3 id="model-availability-under-zdr">
  Ketersediaan model di bawah ZDR
</h3>

Claude Fable 5.1 dan Fable 5 adalah [Covered Models](https://support.claude.com/en/articles/15425695-covered-models) yang [memerlukan retensi data](https://platform.claude.com/docs/en/manage-claude/api-and-data-retention#model-specific-data-retention-requirements) secara default, dan apakah organisasi atau workspace ZDR dapat menggunakannya diatur oleh kebijakan Covered Models daripada oleh Claude Code. Di mana organisasi Anda tidak dapat menggunakannya, model-model tersebut baik tidak ada di pemilih `/model` atau ditampilkan sebagai dinonaktifkan, dan server menolak permintaan untuk mereka terlepas dari konfigurasi klien.

Model lainnya tetap tersedia di bawah ZDR. Model Fable bukan model default, dan alias `best`, yang diselesaikan ke model Fable terbaru di mana tersedia, diselesaikan ke Opus untuk organisasi di mana tidak tersedia.

<h2 id="data-retention-for-policy-violations">
  Retensi data untuk pelanggaran kebijakan
</h2>

Bahkan dengan ZDR diaktifkan, Anthropic dapat menyimpan data jika diperlukan oleh hukum atau untuk mengatasi pelanggaran Usage Policy. Jika sesi ditandai untuk pelanggaran kebijakan, Anthropic dapat menyimpan input dan output terkait selama hingga 2 tahun, konsisten dengan kebijakan ZDR standar Anthropic.

<h2 id="request-zdr">
  Minta ZDR
</h2>

Untuk meminta ZDR untuk Claude Code di Claude for Enterprise, [hubungi penjualan](https://www.anthropic.com/contact-sales?utm_source=claude_code\&utm_medium=docs\&utm_content=zero_data_retention_request) atau tim akun Anthropic Anda. Tim akun Anda akan mengirimkan permintaan secara internal, dan Anthropic akan meninjau dan mengaktifkan ZDR di organisasi Anda setelah mengkonfirmasi kelayakan. Semua tindakan pengaktifan dicatat dalam audit log.

Jika Anda saat ini menggunakan ZDR untuk Claude Code melalui kunci API pay-as-you-go, Anda dapat beralih ke Claude for Enterprise untuk mendapatkan akses ke fitur administratif sambil mempertahankan ZDR untuk Claude Code. Hubungi tim akun Anda untuk mengoordinasikan migrasi.
