> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Konfigurasi jaringan enterprise

> Konfigurasikan Claude Code untuk lingkungan enterprise dengan server proxy, Certificate Authorities (CA) kustom, dan autentikasi mutual Transport Layer Security (mTLS).

Claude Code mendukung berbagai konfigurasi jaringan dan keamanan enterprise melalui variabel lingkungan. Ini termasuk merutekan lalu lintas melalui server proxy perusahaan, mempercayai Certificate Authorities (CA) kustom, dan mengautentikasi dengan sertifikat mutual Transport Layer Security (mTLS) untuk keamanan yang ditingkatkan.

<Note>
  Semua variabel lingkungan yang ditampilkan di halaman ini juga dapat dikonfigurasi di [`settings.json`](/docs/id/settings).
</Note>

<h2 id="proxy-configuration">
  Konfigurasi proxy
</h2>

<h3 id="environment-variables">
  Variabel lingkungan
</h3>

Claude Code menghormati variabel lingkungan proxy standar:

```bash theme={null}
# HTTPS proxy (direkomendasikan)
export HTTPS_PROXY=https://proxy.example.com:8080

# HTTP proxy (jika HTTPS tidak tersedia)
export HTTP_PROXY=http://proxy.example.com:8080

# Lewati proxy untuk permintaan tertentu - format terpisah spasi
export NO_PROXY="localhost 192.168.1.1 example.com .example.com"
# Lewati proxy untuk permintaan tertentu - format terpisah koma
export NO_PROXY="localhost,192.168.1.1,example.com,.example.com"
# Lewati proxy untuk semua permintaan
export NO_PROXY="*"
```

<Note>
  Claude Code tidak mendukung proxy SOCKS.
</Note>

<h3 id="basic-authentication">
  Autentikasi dasar
</h3>

Jika proxy Anda memerlukan autentikasi dasar, sertakan kredensial dalam URL proxy:

```bash theme={null}
export HTTPS_PROXY=http://username:password@proxy.example.com:8080
```

<Warning>
  Hindari hardcoding kata sandi dalam skrip. Gunakan variabel lingkungan atau penyimpanan kredensial aman sebagai gantinya.
</Warning>

<Tip>
  Untuk proxy yang memerlukan autentikasi lanjutan (NTLM, Kerberos, dll.), pertimbangkan menggunakan layanan LLM Gateway yang mendukung metode autentikasi Anda.
</Tip>

<h2 id="ca-certificate-store">
  Penyimpanan sertifikat CA
</h2>

Secara default, Claude Code mempercayai baik sertifikat CA Mozilla yang disertakan maupun penyimpanan sertifikat sistem operasi Anda. Membaca penyimpanan OS memerlukan runtime dengan `tls.getCACertificates`: installer native selalu memilikinya, dan instalasi npm memerlukan Node 22.15 atau lebih baru. Pada versi Node yang lebih lama, hanya set yang disertakan dan `NODE_EXTRA_CA_CERTS` yang berlaku. Proxy inspeksi TLS enterprise seperti CrowdStrike Falcon dan Zscaler bekerja tanpa konfigurasi tambahan ketika sertifikat akar mereka diinstal di penyimpanan kepercayaan OS dan runtime dapat membacanya.

`CLAUDE_CODE_CERT_STORE` menerima daftar sumber yang dipisahkan koma. Nilai yang dikenali adalah `bundled` untuk set CA Mozilla yang dikirimkan dengan Claude Code dan `system` untuk penyimpanan kepercayaan sistem operasi. Default adalah `bundled,system`.

Untuk mempercayai hanya set CA Mozilla yang disertakan:

```bash theme={null}
export CLAUDE_CODE_CERT_STORE=bundled
```

Untuk mempercayai hanya penyimpanan sertifikat OS:

```bash theme={null}
export CLAUDE_CODE_CERT_STORE=system
```

<Note>
  `CLAUDE_CODE_CERT_STORE` tidak memiliki kunci skema `settings.json` khusus. Aturnya melalui blok `env` di `~/.claude/settings.json` atau langsung di lingkungan proses.
</Note>

<h2 id="custom-ca-certificates">
  Sertifikat CA kustom
</h2>

Jika lingkungan enterprise Anda menggunakan CA kustom, konfigurasikan Claude Code untuk mempercayainya secara langsung:

```bash theme={null}
export NODE_EXTRA_CA_CERTS=/path/to/ca-cert.pem
```

<h2 id="mtls-authentication">
  Autentikasi mTLS
</h2>

Untuk lingkungan enterprise yang memerlukan autentikasi sertifikat klien:

```bash theme={null}
# Sertifikat klien untuk autentikasi
export CLAUDE_CODE_CLIENT_CERT=/path/to/client-cert.pem

# Kunci pribadi klien
export CLAUDE_CODE_CLIENT_KEY=/path/to/client-key.pem

# Opsional: Frasa sandi untuk kunci pribadi terenkripsi
export CLAUDE_CODE_CLIENT_KEY_PASSPHRASE="your-passphrase"
```

Claude Code membaca file sertifikat dan kunci saat startup dan membacanya kembali setiap kali menerapkan pengaturan, termasuk ketika pengaturan berubah selama sesi. Untuk merotasi sertifikat dan kunci, ganti file di jalur yang sama.

<h2 id="network-access-requirements">
  Persyaratan akses jaringan
</h2>

Claude Code memerlukan akses ke URL berikut. Izinkan URL ini dalam konfigurasi proxy dan aturan firewall Anda, terutama di lingkungan jaringan terkontainer atau terbatas.

| URL                            | Diperlukan untuk                                                                                                                                                                                                                                                                                                                                                                                                                     |
| ------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `api.anthropic.com`            | Permintaan Claude API                                                                                                                                                                                                                                                                                                                                                                                                                |
| `claude.ai`                    | Autentikasi akun claude.ai                                                                                                                                                                                                                                                                                                                                                                                                           |
| `platform.claude.com`          | Autentikasi akun Anthropic Console                                                                                                                                                                                                                                                                                                                                                                                                   |
| `mcp-proxy.anthropic.com`      | [Konektor MCP dari claude.ai](/docs/id/mcp#use-mcp-servers-from-claude-ai), termasuk konektor yang dikonfigurasi administrator organisasi. Lalu lintas konektor merutekan melalui proxy ini; konektor diaktifkan secara default untuk pengguna yang diautentikasi claude.ai. Untuk menonaktifkan, atur [`ENABLE_CLAUDEAI_MCP_SERVERS=false`](/docs/id/env-vars) atau pengaturan [`disableClaudeAiConnectors`](/docs/id/settings#available-settings) |
| `downloads.claude.ai`          | Unduhan plugin yang dapat dieksekusi; penginstal asli dan pembaruan otomatis asli                                                                                                                                                                                                                                                                                                                                                    |
| `storage.googleapis.com`       | Jumlah instalasi dan metadata plugin yang ditampilkan di `/plugin`. Unggahan [artifact](/docs/id/artifacts) yang ditandatangani mencoba host ini terlebih dahulu; penerbitan kembali ke `api.anthropic.com` ketika host ini diblokir                                                                                                                                                                                                      |
| `storage.googleapis.com`       | Penginstal asli dan pembaruan otomatis asli pada versi sebelum 2.1.116                                                                                                                                                                                                                                                                                                                                                               |
| `bridge.claudeusercontent.com` | Jembatan WebSocket ekstensi [Claude di Chrome](/docs/id/chrome)                                                                                                                                                                                                                                                                                                                                                                           |
| `*.claudeusercontent.com`      | Melihat [artifacts](/docs/id/artifacts) di claude.ai. Penampil memuat konten setiap artifact dari subdomain yang disandbox dari asal ini. Diperlukan di browser penampil, bukan oleh CLI itu sendiri                                                                                                                                                                                                                                      |
| `raw.githubusercontent.com`    | Umpan catatan perubahan untuk [`/release-notes`](/docs/id/commands) dan catatan rilis yang ditampilkan setelah pembaruan                                                                                                                                                                                                                                                                                                                  |

Jika Anda menginstal Claude Code melalui npm atau mengelola distribusi biner Anda sendiri, pengguna akhir tidak perlu menggunakan penginstal asli dan pembaruan otomatis `downloads.claude.ai`. Penggunaan lain dalam tabel berlaku terlepas dari metode instalasi.

Claude Code juga mengirimkan telemetri operasional opsional secara default, yang dapat Anda nonaktifkan dengan variabel lingkungan. Lihat [Layanan telemetri](/docs/id/data-usage#telemetry-services) untuk cara menonaktifkannya sebelum menyelesaikan daftar izin Anda.

Saat menggunakan [Amazon Bedrock](/docs/id/amazon-bedrock), [Google Cloud's Agent Platform](/docs/id/google-vertex-ai), [Microsoft Foundry](/docs/id/microsoft-foundry), atau sesi [gateway aplikasi Claude](/docs/id/claude-apps-gateway) yang sudah masuk, lalu lintas model dan autentikasi menuju penyedia atau gateway Anda alih-alih `api.anthropic.com`, `claude.ai`, atau `platform.claude.com`. Alat WebFetch masih memanggil `api.anthropic.com` untuk [pemeriksaan keamanan domainnya](/docs/id/data-usage#webfetch-domain-safety-check) kecuali Anda menetapkan `skipWebFetchPreflight: true` di [pengaturan](/docs/id/settings).

[Claude Code di web](/docs/id/claude-code-on-the-web) dan [Code Review](/docs/id/code-review) terhubung ke repositori Anda dari infrastruktur yang dikelola Anthropic. Jika organisasi GitHub Enterprise Cloud Anda membatasi akses berdasarkan alamat IP, aktifkan [pewarisan daftar izin IP untuk GitHub Apps yang diinstal](https://docs.github.com/en/enterprise-cloud@latest/organizations/keeping-your-organization-secure/managing-security-settings-for-your-organization/managing-allowed-ip-addresses-for-your-organization#allowing-access-by-github-apps). Claude GitHub App mendaftarkan rentang IP-nya, jadi mengaktifkan pengaturan ini memungkinkan akses tanpa konfigurasi manual. Untuk [menambahkan rentang ke daftar izin Anda secara manual](https://docs.github.com/en/enterprise-cloud@latest/organizations/keeping-your-organization-secure/managing-security-settings-for-your-organization/managing-allowed-ip-addresses-for-your-organization#adding-an-allowed-ip-address) sebagai gantinya, atau untuk mengonfigurasi firewall lainnya, lihat [Alamat IP API Anthropic](https://platform.claude.com/docs/en/api/ip-addresses).

Untuk instans [GitHub Enterprise Server](/docs/id/github-enterprise-server) yang dihosting sendiri di belakang firewall, izinkan daftar [Alamat IP API Anthropic](https://platform.claude.com/docs/en/api/ip-addresses) yang sama sehingga infrastruktur Anthropic dapat menjangkau host GHES Anda untuk mengkloning repositori dan memposting komentar tinjauan.

<h3 id="desktop-and-claude-ai">
  Desktop dan claude.ai
</h3>

Tabel sebelumnya terutama mencakup CLI mandiri. Aplikasi Claude Desktop dan claude.ai di browser memuat kode aplikasi mereka dari host CDN Anthropic tambahan, termasuk `assets-proxy.anthropic.com`. Mengizinkan `claude.ai` sambil memblokir host tersebut menghasilkan halaman kosong daripada kesalahan. Lihat [persyaratan akses jaringan](/docs/id/desktop#network-access-requirements) di halaman Desktop.

<h2 id="additional-resources">
  Sumber daya tambahan
</h2>

* [Pengaturan Claude Code](/docs/id/settings)
* [Referensi variabel lingkungan](/docs/id/env-vars)
* [Panduan pemecahan masalah](/docs/id/troubleshooting)
