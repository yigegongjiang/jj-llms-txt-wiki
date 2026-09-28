> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Keamanan

> Pelajari tentang perlindungan keamanan Claude Code dan praktik terbaik untuk penggunaan yang aman.

<h2 id="how-we-approach-security">
  Bagaimana kami mendekati keamanan
</h2>

<h3 id="security-foundation">
  Fondasi keamanan
</h3>

Keamanan kode Anda adalah prioritas utama. Claude Code dibangun dengan keamanan sebagai inti, dikembangkan sesuai dengan program keamanan komprehensif Anthropic. Pelajari lebih lanjut dan akses sumber daya (laporan SOC 2 Type 2, sertifikat ISO 27001, dll.) di [Anthropic Trust Center](https://trust.anthropic.com).

<h3 id="permission-based-architecture">
  Arsitektur berbasis izin
</h3>

Dalam mode Manual, Claude Code dimulai dengan izin baca-saja. Ketika Claude Code perlu mengedit file, menjalankan tes, atau mengeksekusi perintah, Claude Code meminta Anda terlebih dahulu, dan Anda memilih apakah akan menyetujui tindakan sekali atau mengizinkannya dari saat itu.

Dalam mode Manual, Claude Code juga meminta sebelum menjalankan perintah Bash yang dapat memodifikasi sistem Anda. Claude Code menjalankan serangkaian [perintah baca-saja](/docs/id/permissions#read-only-commands) bawaan seperti `ls`, `cat`, dan `git status` tanpa meminta. Anda dan organisasi Anda mengonfigurasi izin ini secara langsung.

Dalam [mode auto](/docs/id/permission-modes#eliminate-prompts-with-auto-mode), model pengklasifikasi terpisah meninjau tindakan alih-alih Anda dan memblokir tindakan yang dianggapnya tidak aman. [Bagaimana pengklasifikasi mengevaluasi tindakan](/docs/id/permission-modes#how-the-classifier-evaluates-actions) mencantumkan tindakan mana yang Claude Code setujui sepenuhnya, tindakan mana yang dikirimnya ke pengklasifikasi, dan tindakan mana yang masih diminta Claude Code kepada Anda. Aturan permintaan dan penolakan eksplisit Anda masih berlaku, dan organisasi Anda dapat [mematikan mode auto](/docs/id/permission-modes#eliminate-prompts-with-auto-mode).

Mode izin mana yang dimulai sesi tergantung pada paket Anda, permukaan tempat Anda memulainya, dan pengaturan Anda serta organisasi Anda; lihat [Permission modes](/docs/id/permission-modes#which-mode-a-session-starts-in).

Untuk konfigurasi izin terperinci, lihat [Permissions](/docs/id/permissions).

<h3 id="built-in-protections">
  Perlindungan bawaan
</h3>

Untuk mengurangi risiko dalam sistem agentic:

* **Alat bash bersandbox**: [Sandbox](/docs/id/sandboxing) perintah bash dengan isolasi filesystem dan jaringan, mengurangi permintaan izin sambil mempertahankan keamanan. Konfigurasi dengan `/sandbox` untuk menentukan batas tempat Claude Code dapat bekerja secara otonom
* **Pembatasan direktori kerja**: Dalam mode Manual, Claude Code hanya dapat menulis ke folder tempat dimulai dan subfolder-nya, dan tidak dapat memodifikasi file di direktori induk tanpa izin eksplisit. Dalam mode Manual, Claude Code juga meminta Anda sebelum membaca jalur di luar batas ini dengan alat Read, Grep, dan Glob. Perluas batas dengan [direktori tambahan](/docs/id/permissions#working-directories) untuk melewati permintaan, atau batasi akses baca yang lebih luas tersedia untuk perintah Bash baca-saja dengan [aturan sandbox `denyRead`](/docs/id/sandboxing#filesystem-isolation), yang hanya berlaku ketika sandboxing diaktifkan
* **Mitigasi kelelahan permintaan**: Dukungan untuk allowlisting perintah aman yang sering digunakan per-pengguna, per-codebase, atau per-organisasi
* **Mode Accept Edits**: Persetujuan otomatis untuk edit file dan serangkaian perintah Bash filesystem tetap seperti `mkdir`, `touch`, `rm`, `mv`, `cp`, dan `sed` untuk jalur di direktori kerja. Perintah Bash lainnya dan jalur di luar cakupan masih meminta persetujuan

<h3 id="user-responsibility">
  Tanggung jawab pengguna
</h3>

Claude Code hanya memiliki izin yang Anda berikan. Anda bertanggung jawab untuk meninjau kode dan perintah yang diusulkan untuk keamanan sebelum persetujuan.

<h2 id="protect-against-prompt-injection">
  Lindungi dari prompt injection
</h2>

Prompt injection adalah teknik di mana penyerang mencoba mengganti atau memanipulasi instruksi asisten AI dengan menyisipkan teks berbahaya. Claude Code mencakup beberapa perlindungan terhadap serangan ini:

<h3 id="core-protections">
  Perlindungan inti
</h3>

* **Sistem izin**: Dalam Manual mode, operasi sensitif memerlukan persetujuan eksplisit
* **Analisis yang menyadari konteks**: Mendeteksi instruksi yang berpotensi berbahaya dengan menganalisis permintaan lengkap
* **Sanitasi input**: Mencegah command injection dengan memproses input pengguna
* **Persetujuan perintah jaringan**: Perintah yang mengambil konten dari web seperti `curl` dan `wget` tidak disetujui secara otomatis secara default. Dalam Manual mode mereka meminta seperti perintah Bash non-read-only lainnya, sehingga Anda masih dapat menyetujui sekali atau menambahkan aturan izin eksplisit seperti `Bash(curl *)`. Untuk menghentikan Claude dari menjalankannya, tambahkan ke [`permissions.deny`](/docs/id/permissions#tool-specific-permission-rules). Aturan deny cocok dengan perintah [seperti yang ditulis](/docs/id/permissions#bash-rule-limits); untuk penegakan jaringan yang tidak bergantung pada teks perintah, lihat [isolasi jaringan sandbox](/docs/id/sandboxing#network-isolation)

<h3 id="privacy-safeguards">
  Perlindungan privasi
</h3>

Kami telah menerapkan beberapa perlindungan untuk melindungi data Anda, termasuk:

* Periode retensi terbatas untuk informasi sensitif (lihat [Privacy Center](https://privacy.anthropic.com/en/articles/10023548-how-long-do-you-store-my-data) untuk mempelajari lebih lanjut)
* Akses terbatas ke data sesi pengguna
* Kontrol pengguna atas preferensi pelatihan data. Pengguna konsumen dapat mengubah [pengaturan privasi](https://claude.ai/settings/privacy) mereka kapan saja.

Untuk detail lengkap, silakan tinjau [Commercial Terms of Service](https://www.anthropic.com/legal/commercial-terms) kami (untuk pengguna Team, Enterprise, dan API) atau [Consumer Terms](https://www.anthropic.com/legal/consumer-terms) (untuk pengguna Free, Pro, dan Max) dan [Privacy Policy](https://www.anthropic.com/legal/privacy).

<h3 id="additional-safeguards">
  Perlindungan tambahan
</h3>

* **Persetujuan permintaan jaringan**: Dalam Manual mode, sebagian besar alat yang membuat permintaan jaringan memerlukan persetujuan pengguna secara default
* **Jendela konteks terisolasi**: Web fetch menggunakan jendela konteks terpisah untuk menghindari injeksi prompt yang berpotensi berbahaya
* **Verifikasi kepercayaan**: Jalankan codebase pertama kali dan server MCP baru memerlukan verifikasi kepercayaan
  * Catatan: Verifikasi kepercayaan dinonaktifkan saat menjalankan secara non-interaktif dengan flag `-p`
  * Catatan: Ketika Anda memulai Claude Code langsung di direktori home Anda, penerimaan kepercayaan disimpan untuk sesi saat ini saja dan tidak ditulis ke disk, jadi prompt muncul kembali pada setiap peluncuran. Tidak ada pengaturan untuk mempertahankannya. Mulai Claude Code dari subdirektori proyek sebagai gantinya, di mana penerimaan kepercayaan disimpan per direktori
* **Deteksi command injection**: Dalam Manual mode, perintah bash yang mencurigakan memerlukan persetujuan manual bahkan jika sebelumnya allowlisted
* **Pencocokan fail-closed**: Dalam Manual mode, perintah yang tidak cocok memerlukan persetujuan secara default
* **Deskripsi bahasa alami**: Perintah bash kompleks menyertakan penjelasan untuk pemahaman pengguna
* **Penyimpanan kredensial aman**: Kunci API dan token disimpan di macOS Keychain jika tersedia, dan dilindungi oleh izin file di Windows dan Linux. Lihat [Credential Management](/docs/id/authentication#credential-management)

<Warning>
  **Risiko keamanan Windows WebDAV**: Saat menjalankan Claude Code di Windows, kami merekomendasikan untuk tidak mengaktifkan WebDAV atau mengizinkan Claude Code mengakses path seperti `\\*` yang mungkin berisi subdirektori WebDAV. [WebDAV telah dihentikan oleh Microsoft](https://learn.microsoft.com/en-us/windows/whats-new/deprecated-features#:~:text=The%20Webclient%20\(WebDAV\)%20service%20is%20deprecated) karena risiko keamanan. Mengaktifkan WebDAV dapat memungkinkan Claude Code memicu permintaan jaringan ke host jarak jauh, melewati sistem izin.
</Warning>

**Praktik terbaik untuk bekerja dengan konten yang tidak dipercaya**:

1. Tinjau perintah yang disarankan sebelum persetujuan
2. Hindari piping konten yang tidak dipercaya langsung ke Claude
3. Verifikasi perubahan yang diusulkan pada file kritis
4. Gunakan mesin virtual (VM) untuk menjalankan skrip dan membuat panggilan alat, terutama saat berinteraksi dengan layanan web eksternal
5. Laporkan perilaku mencurigakan dengan `/feedback`

<Warning>
  Meskipun perlindungan ini secara signifikan mengurangi risiko, tidak ada sistem yang
  sepenuhnya kebal terhadap semua serangan. Selalu pertahankan praktik keamanan yang baik saat bekerja
  dengan alat AI apa pun.
</Warning>

<h2 id="mcp-security">
  Keamanan MCP
</h2>

Claude Code memungkinkan pengguna untuk mengonfigurasi server Model Context Protocol (MCP). Daftar server MCP yang diizinkan dikonfigurasi dalam kode sumber Anda, sebagai bagian dari pengaturan Claude Code yang diperiksa insinyur ke dalam kontrol sumber.

Kami mendorong untuk menulis server MCP Anda sendiri atau menggunakan server MCP dari penyedia yang Anda percayai. Anda dapat mengonfigurasi izin Claude Code untuk server MCP. Anthropic meninjau konektor terhadap [kriteria pendaftarannya](https://claude.com/docs/connectors/building/review-criteria) sebelum menambahkannya ke [Direktori Anthropic](https://claude.ai/directory), tetapi tidak melakukan audit keamanan atau mengelola server MCP apa pun.

<h2 id="ide-security">
  Keamanan IDE
</h2>

Lihat [VS Code security and privacy](/docs/id/vs-code#security-and-privacy) untuk informasi lebih lanjut tentang menjalankan Claude Code di IDE.

<h2 id="cloud-execution-security">
  Keamanan eksekusi cloud
</h2>

Saat menggunakan [sesi cloud](/docs/id/claude-code-on-the-web), kontrol keamanan tambahan tersedia. Sesi yang organisasi Anda arahkan ke [lingkungan yang di-host sendiri](/docs/id/self-hosted-environments) berjalan di infrastruktur Anda sendiri, di mana isolasi, egress jaringan, dan kredensial git adalah tanggung jawab deployment Anda. Di lingkungan yang di-host Anthropic:

* **Mesin virtual terisolasi**: Setiap sesi cloud berjalan di VM yang terisolasi dan dikelola Anthropic
* **Kontrol akses jaringan**: Akses jaringan dibatasi secara default dan dapat dikonfigurasi untuk dinonaktifkan atau hanya mengizinkan domain tertentu
* **Perlindungan kredensial**: Autentikasi ditangani melalui proxy aman yang menggunakan kredensial bersisir di dalam sandbox, yang kemudian diterjemahkan ke token autentikasi GitHub aktual Anda
* **Pembatasan cabang**: Operasi git push dibatasi pada cabang kerja saat ini
* **Pencatatan audit**: Semua operasi di sesi cloud dicatat untuk kepatuhan dan tujuan audit
* **Pembersihan otomatis**: VM sesi diklaim kembali setelah periode tidak aktif

Untuk detail lebih lanjut tentang eksekusi cloud, lihat [Gunakan Claude Code di cloud](/docs/id/claude-code-on-the-web); untuk mengonfigurasi akses jaringan untuk sesi cloud, lihat [Konfigurasi lingkungan cloud](/docs/id/cloud-environments#network-access).

Sesi [Remote Control](/docs/id/remote-control) bekerja berbeda: antarmuka web terhubung ke proses Claude Code yang berjalan di mesin lokal Anda. Semua eksekusi kode dan akses file tetap lokal, dan lalu lintas sesi berjalan melalui Anthropic API melalui TLS; saat terhubung, transkrip sesi disimpan di server Anthropic untuk menyinkronkan percakapan di seluruh perangkat, seperti yang dijelaskan dalam [Connection and security](/docs/id/remote-control#connection-and-security). Tidak ada VM cloud atau sandboxing yang terlibat. Koneksi menggunakan beberapa kredensial berumur pendek dengan cakupan sempit, masing-masing dibatasi untuk tujuan tertentu dan kedaluwarsa secara independen, untuk membatasi radius ledakan dari kredensial tunggal yang dikompromikan.

<h2 id="security-best-practices">
  Praktik terbaik keamanan
</h2>

<h3 id="working-with-sensitive-code">
  Bekerja dengan kode sensitif
</h3>

* Tinjau semua perubahan yang disarankan sebelum persetujuan
* Gunakan pengaturan izin khusus proyek untuk repositori sensitif
* Pertimbangkan menggunakan [dev containers](/docs/id/devcontainer) untuk isolasi tambahan
* Audit secara teratur pengaturan izin Anda dengan `/permissions`

<h3 id="team-security">
  Keamanan tim
</h3>

* Gunakan [managed settings](/docs/id/settings#where-settings-live) untuk menegakkan standar organisasi
* Bagikan konfigurasi izin yang disetujui melalui kontrol versi
* Latih anggota tim tentang praktik terbaik keamanan
* Pantau penggunaan Claude Code melalui [OpenTelemetry metrics](/docs/id/monitoring-usage)
* Audit atau blokir perubahan pengaturan selama sesi dengan [`ConfigChange` hooks](/docs/id/hooks#configchange)

<h3 id="reporting-security-issues">
  Melaporkan masalah keamanan
</h3>

Jika Anda menemukan kerentanan keamanan di Claude Code:

1. Jangan ungkapkan secara publik
2. Laporkan melalui [program HackerOne](https://hackerone.com/4f1f16ba-10d3-4d09-9ecc-c721aad90f24/embedded_submissions/new) kami
3. Sertakan langkah reproduksi terperinci
4. Berikan waktu bagi kami untuk mengatasi masalah sebelum pengungkapan publik

<h2 id="related-resources">
  Sumber daya terkait
</h2>

* [Security guidance plugin](/docs/id/security-guidance): biarkan Claude meninjau dan memperbaiki kerentanan dalam perubahan kode miliknya sendiri selama sesi
* [`/security-review`](/docs/id/commands#all-commands): jalankan pemeriksaan keamanan sesuai permintaan atas perubahan pada cabang Anda saat ini
* [Sandbox environments](/docs/id/sandbox-environments): bandingkan pendekatan isolasi dan pilih satu untuk model ancaman Anda
* [Sandboxing](/docs/id/sandboxing): isolasi filesystem dan jaringan untuk perintah Bash
* [Permissions](/docs/id/permissions): konfigurasi izin dan kontrol akses
* [Monitoring usage](/docs/id/monitoring-usage): lacak dan audit aktivitas Claude Code
* [Development containers](/docs/id/devcontainer): lingkungan yang aman dan terisolasi
* [Anthropic Trust Center](https://trust.anthropic.com): sertifikasi keamanan dan kepatuhan
* [CISO's guide to agentic AI](https://claude.com/blog/ciso-guide-to-agentic-ai): kerangka kerja pemimpin keamanan untuk menilai penyebaran AI agentic
