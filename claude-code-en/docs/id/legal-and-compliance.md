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

* [Syarat Komersial](https://www.anthropic.com/legal/commercial-terms) - untuk pengguna Team, Enterprise, dan Claude API
* [Syarat Layanan Konsumen](https://www.anthropic.com/legal/consumer-terms) - untuk pengguna Free, Pro, dan Max

<h3 id="commercial-agreements">
  Perjanjian komersial
</h3>

Baik Anda menggunakan Claude API secara langsung (1P) atau mengaksesnya melalui Amazon Bedrock atau Google Cloud's Agent Platform (3P), perjanjian komersial yang ada akan berlaku untuk penggunaan Claude Code, kecuali kami telah menyetujui sebaliknya.

<h2 id="compliance">
  Kepatuhan
</h2>

<h3 id="healthcare-compliance-baa">
  Kepatuhan kesehatan (BAA)
</h3>

Jika pelanggan memiliki Business Associate Agreement (BAA) dengan kami, dan ingin menggunakan Claude Code, BAA akan secara otomatis diperluas untuk mencakup Claude Code jika pelanggan telah menjalankan BAA dan memiliki [Zero Data Retention (ZDR)](/docs/id/zero-data-retention) diaktifkan. BAA akan berlaku untuk lalu lintas API pelanggan tersebut yang mengalir melalui Claude Code. ZDR diaktifkan berdasarkan per-organisasi, jadi setiap organisasi harus memiliki ZDR diaktifkan secara terpisah untuk dicakup di bawah BAA.

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
* **Pengembang** yang membangun produk atau layanan yang berinteraksi dengan kemampuan Claude, termasuk mereka yang menggunakan [Agent SDK](/docs/id/agent-sdk/overview), harus menggunakan autentikasi kunci API melalui [Claude Console](https://platform.claude.com/) atau penyedia cloud yang didukung. Anthropic tidak mengizinkan pengembang pihak ketiga untuk menawarkan login Claude.ai atau untuk merutekan permintaan melalui kredensial paket Free, Pro, atau Max atas nama pengguna mereka.

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
