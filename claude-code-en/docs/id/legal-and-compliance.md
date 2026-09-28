> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Hukum dan kepatuhan

> Perjanjian hukum, sertifikasi kepatuhan, dan informasi keamanan untuk Claude Code.

<h2 id="legal-agreements">
  Perjanjian hukum
</h2>

<h3 id="license">
  Lisensi
</h3>

Penggunaan Claude Code Anda tunduk pada:

* [Syarat Layanan Komersial](https://www.anthropic.com/legal/commercial-terms) - untuk pengguna Team, Enterprise, dan Claude API
* [Syarat Layanan Konsumen](https://www.anthropic.com/legal/consumer-terms) - untuk pengguna Free, Pro, dan Max

<h3 id="commercial-agreements">
  Perjanjian komersial
</h3>

Baik Anda menggunakan Claude API secara langsung (1P) atau mengaksesnya melalui Amazon Bedrock atau Google Cloud's Agent Platform (3P), perjanjian komersial yang ada akan berlaku untuk penggunaan Claude Code, kecuali kami telah menyetujui sebaliknya.

<h3 id="can-customers-offer-claude-code-in-their-products">
  Dapatkah pelanggan menawarkan Claude Code dalam produk mereka?
</h3>

Kecuali kami telah menyetujui sebaliknya, pra-instalasi atau menjalankan Claude Code dalam produk atau layanan Anda (misalnya dalam sandbox yang dihosting atau infrastruktur agen lainnya) memerlukan persetujuan dengan [Syarat Layanan Komersial](https://www.anthropic.com/legal/commercial-terms) kami dan kepatuhan terhadap kondisi di bawah ini:

* **Biner Claude Code tidak boleh dimodifikasi.** Claude Code harus diinstal dan dijalankan seperti yang dipublikasikan oleh Anthropic, dan pelanggan tidak boleh menghapus, menonaktifkan, atau membatasi metode autentikasi apa pun yang tertanam di dalamnya (termasuk metode yang memungkinkan masuk dengan akun Claude atau kunci API pengguna mereka sendiri).
* **Pelanggan tidak boleh membayar, menjual kembali, atau menengahi penggunaan Claude atas nama pengguna akhir mereka.** Setiap pengguna akhir harus melakukan autentikasi dengan kunci API Anthropic mereka sendiri, kredensial rencana langganan Claude, atau kredensial penyedia inferensi pihak ketiga (Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry). Penggunaan tersebut ditagihkan langsung kepada pengguna akhir berdasarkan perjanjian mereka sendiri dengan Anthropic atau, untuk penyedia inferensi pihak ketiga, dengan penyedia yang berlaku.

**Menggunakan nama dan logo Claude Code.** Anda dapat dengan akurat mengatakan, dalam teks biasa, bahwa produk Anda memiliki Claude Code pra-terinstal atau bahwa produk Anda menjalankan Claude Code. Tetapi Anda tidak dapat menggunakan nama atau logo Claude Code atau Anthropic sebagai bagian dari nama produk, fitur, atau perusahaan Anda sendiri, dalam logo Anda sendiri, atau dengan cara yang menunjukkan bahwa Anthropic membangun, mendukung, atau bermitra dengan produk Anda. Penggunaan nama atau logo Anthropic lainnya diatur oleh [Panduan Merek Dagang](https://www.anthropic.com/legal/trademark-guidelines) kami dan memerlukan izin tertulis kami.

Claude Code tetap diatur oleh syarat standar Anthropic (lihat bagian Lisensi dan Perjanjian komersial di atas) terlepas dari platform melalui mana Claude Code diakses.

<h2 id="compliance">
  Kepatuhan
</h2>

<h3 id="healthcare-compliance-baa">
  Kepatuhan kesehatan (BAA)
</h3>

Jika pelanggan telah menjalankan Business Associate Agreement (BAA) dengan Anthropic dan memiliki [Zero Data Retention (ZDR)](/docs/id/zero-data-retention) diaktifkan untuk organisasi yang relevan, BAA tersebut berlaku untuk lalu lintas API pelanggan melalui Claude Code.

<h2 id="usage-policy">
  Kebijakan penggunaan
</h2>

<h3 id="acceptable-use">
  Penggunaan yang dapat diterima
</h3>

Penggunaan Claude Code tunduk pada [Kebijakan Penggunaan Anthropic](https://www.anthropic.com/legal/aup). Batas penggunaan yang diiklankan untuk paket Pro dan Max mengasumsikan penggunaan biasa dan individual dari Claude Code dan Agent SDK.

<h3 id="authentication-and-credential-use">
  Autentikasi dan penggunaan kredensial
</h3>

Claude Code melakukan autentikasi dengan server Anthropic menggunakan token OAuth atau kunci API. Metode autentikasi ini melayani tujuan yang berbeda:

* **Autentikasi OAuth** dimaksudkan secara eksklusif untuk pembeli paket langganan Claude Free, Pro, Max, Team, dan Enterprise dan dirancang untuk mendukung penggunaan biasa Claude Code dan aplikasi asli Anthropic lainnya. Untuk langkah-langkah masuk, lihat [Masuk ke akun Claude Anda](https://support.claude.com/en/articles/13189465-logging-in-to-your-claude-account); untuk cara Claude Code melakukan autentikasi OAuth, lihat [Authentication](/docs/id/authentication).
* **Pengembang** yang membangun produk atau layanan yang berinteraksi dengan kemampuan Claude, termasuk mereka yang menggunakan [Agent SDK](/docs/id/agent-sdk/overview), harus menggunakan autentikasi kunci API melalui [Claude Console](https://platform.claude.com/) atau penyedia cloud yang didukung. Anthropic tidak mengizinkan pengembang pihak ketiga untuk menawarkan login Claude.ai ke dalam aplikasi mereka sendiri, atau untuk merutekan permintaan melalui kredensial paket Free, Pro, atau Max atas nama pengguna mereka. Selain itu, pengembang tidak boleh mengumpulkan, menyimpan, atau menengahi kredensial Claude.ai atau token sesi — masuk ke akun Claude harus diselesaikan melalui alur Anthropic sendiri.

Ini tidak membatasi bagaimana pelanggan menyediakan dan mengelola kunci API mereka sendiri atau kredensial penyedia inferensi pihak ketiga — misalnya, mengonfigurasi kunci API di lingkungan pengembangan, pengelola rahasia, atau citra mesin untuk digunakan oleh pengguna yang berwenang dari pelanggan — asalkan penggunaan yang dihasilkan ditagihkan kepada pemilik kunci berdasarkan perjanjian mereka dengan Anthropic (atau penyedia yang berlaku) dan tidak dijual kembali atau ditengahi seperti yang dijelaskan di atas. Ini juga tidak mencegah pengguna akhir untuk masuk ke biner Claude Code yang tidak dimodifikasi dengan langganan Claude mereka sendiri, termasuk di mana platform menyelenggarakan Claude Code seperti yang dijelaskan di bawah *Dapatkah pelanggan menawarkan Claude Code dalam produk mereka?* di atas.

Anthropic berhak mengambil langkah untuk memberlakukan pembatasan ini dan dapat melakukannya tanpa pemberitahuan sebelumnya.

Untuk pertanyaan tentang metode autentikasi yang diizinkan untuk kasus penggunaan Anda, silakan [hubungi penjualan](https://www.anthropic.com/contact-sales?utm_source=claude_code\&utm_medium=docs\&utm_content=legal_compliance_contact_sales).

<h2 id="security-and-trust">
  Keamanan dan kepercayaan
</h2>

<h3 id="trust-and-safety">
  Kepercayaan dan keselamatan
</h3>

Anda dapat menemukan informasi lebih lanjut di [Pusat Kepercayaan Anthropic](https://trust.anthropic.com) dan [Hub Transparansi](https://www.anthropic.com/transparency).

<h3 id="security-vulnerability-reporting">
  Pelaporan kerentanan keamanan
</h3>

Anthropic mengelola program keamanan kami melalui HackerOne. [Gunakan formulir ini untuk melaporkan kerentanan](https://hackerone.com/4f1f16ba-10d3-4d09-9ecc-c721aad90f24/embedded_submissions/new).

***

© Anthropic PBC. Semua hak dilindungi. Penggunaan tunduk pada Syarat Layanan Anthropic yang berlaku.
