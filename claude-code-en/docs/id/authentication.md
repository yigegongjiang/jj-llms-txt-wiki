> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Autentikasi

> Masuk ke Claude Code dan konfigurasikan autentikasi untuk individu, tim, dan organisasi.

Claude Code mendukung berbagai metode autentikasi tergantung pada pengaturan Anda. Pengguna individual dapat masuk dengan akun Claude.ai, sementara tim dapat menggunakan Claude for Teams atau Enterprise, Claude Console, atau penyedia cloud seperti Amazon Bedrock, Google Cloud's Agent Platform, atau Microsoft Foundry.

<h2 id="log-in-to-claude-code">
  Masuk ke Claude Code
</h2>

Setelah [memasang Claude Code](/docs/id/setup#install-claude-code), jalankan `claude` di terminal Anda. Pada peluncuran pertama, Claude Code membuka jendela browser untuk Anda masuk.

Jika browser tidak terbuka secara otomatis, tekan `c` untuk menyalin URL login ke clipboard Anda, kemudian tempel ke browser Anda.

Jika browser Anda menampilkan kode login alih-alih pengalihan kembali setelah Anda masuk, tempel ke terminal di prompt `Paste code here if prompted`. Ini terjadi ketika browser tidak dapat menjangkau server callback lokal Claude Code, yang umum terjadi di WSL2, sesi SSH, dan kontainer.

Ketika login selesai, terminal menampilkan `Login successful` dan meminta Anda menekan `Enter` untuk melanjutkan.

Anda dapat melakukan autentikasi dengan salah satu jenis akun berikut:

* **Langganan Claude Pro atau Max**: masuk dengan akun Claude.ai Anda. Berlangganan di [claude.com/pricing](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=authentication_pro_max).
* **Claude for Teams atau Enterprise**: masuk dengan akun Claude.ai yang diundang oleh admin tim Anda.
* **Claude Console**: masuk dengan kredensial Console Anda. Admin Anda harus telah [mengundang Anda](#claude-console-authentication) terlebih dahulu.
* **Penyedia cloud**: jika organisasi Anda menggunakan [Amazon Bedrock](/docs/id/amazon-bedrock), [Google Cloud's Agent Platform](/docs/id/google-vertex-ai), atau [Microsoft Foundry](/docs/id/microsoft-foundry), atur variabel lingkungan yang diperlukan sebelum menjalankan `claude`, atau pilih **3rd-party platform** di prompt login, yang meluncurkan wizard pengaturan interaktif untuk Bedrock dan Vertex AI. Tidak diperlukan login browser.
* **Gateway cloud**: jika organisasi Anda menjalankan [gateway aplikasi Claude](/docs/id/claude-apps-gateway) yang di-host sendiri, masuk dengan SSO perusahaan melalui `/login`. Token yang dikeluarkan gateway adalah satu-satunya kredensial sesi.

Admin dapat membatasi login interaktif dengan pengaturan terkelola [`forceLoginMethod` dan `forceLoginOrgUUID`](/docs/id/settings#available-settings). Ketika salah satu diatur, sesi yang diautentikasi oleh `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN`, atau `apiKeyHelper` diblokir saat startup; sesi penyedia cloud tidak terpengaruh.

Untuk keluar dan melakukan autentikasi ulang, ketik `/logout` di prompt Claude Code. Keluar juga mengatur ulang status pengaturan peluncuran pertama Anda, jadi lain kali Anda menjalankan `claude` akan memandu Anda melalui login dan pengaturan lagi.

Jika Anda mengalami kesulitan masuk, lihat [pemecahan masalah autentikasi](/docs/id/troubleshoot-install#login-and-authentication).

<h2 id="set-up-team-authentication">
  Atur autentikasi tim
</h2>

Untuk tim dan organisasi, Anda dapat mengonfigurasi akses Claude Code dengan salah satu cara berikut:

* [Claude for Teams atau Enterprise](#claude-for-teams-or-enterprise), direkomendasikan untuk sebagian besar tim
* [Claude Console](#claude-console-authentication)
* [Claude apps gateway](/docs/id/claude-apps-gateway), gateway yang di-host sendiri yang menandatangani pengembang dengan IdP Anda dan merutekan inferensi ke penyedia cloud yang Anda konfigurasi
* [Amazon Bedrock](/docs/id/amazon-bedrock)
* [Google Cloud's Agent Platform](/docs/id/google-vertex-ai)
* [Microsoft Foundry](/docs/id/microsoft-foundry)

<h3 id="claude-for-teams-or-enterprise">
  Claude for Teams atau Enterprise
</h3>

[Claude for Teams](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=authentication_teams#team-&-enterprise) dan [Claude for Enterprise](https://anthropic.com/contact-sales?utm_source=claude_code\&utm_medium=docs\&utm_content=authentication_enterprise) memberikan pengalaman terbaik bagi organisasi yang menggunakan Claude Code. Anggota tim mendapatkan akses ke Claude Code dan Claude di web dengan penagihan terpusat dan manajemen tim.

* **Claude for Teams**: paket layanan mandiri dengan fitur kolaborasi, alat admin, dan manajemen penagihan. Terbaik untuk tim yang lebih kecil.
* **Claude for Enterprise**: menambahkan SSO, penangkapan domain, izin berbasis peran, API kepatuhan, dan pengaturan kebijakan terkelola untuk konfigurasi Claude Code di seluruh organisasi. Terbaik untuk organisasi yang lebih besar dengan persyaratan keamanan dan kepatuhan.

<Steps>
  <Step title="Berlangganan">
    Berlangganan [Claude for Teams](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=authentication_teams_step#team-&-enterprise) atau hubungi penjualan untuk [Claude for Enterprise](https://anthropic.com/contact-sales?utm_source=claude_code\&utm_medium=docs\&utm_content=authentication_enterprise_step).
  </Step>

  <Step title="Undang anggota tim">
    Undang anggota tim dari dasbor admin.
  </Step>

  <Step title="Pasang dan masuk">
    Anggota tim memasang Claude Code dan masuk dengan akun Claude.ai mereka.
  </Step>
</Steps>

<h3 id="claude-console-authentication">
  Autentikasi Claude Console
</h3>

Untuk organisasi yang lebih suka penagihan berbasis API, Anda dapat menyiapkan akses melalui Claude Console.

<Steps>
  <Step title="Buat atau gunakan akun Console">
    Gunakan akun Claude Console yang sudah ada atau buat yang baru.
  </Step>

  <Step title="Tambahkan pengguna">
    Anda dapat menambahkan pengguna melalui salah satu metode:

    * Undang pengguna secara massal dari dalam Console: Settings -> Members -> Invite
    * [Atur SSO](https://support.claude.com/en/articles/13132885-setting-up-single-sign-on-sso)
  </Step>

  <Step title="Tetapkan peran">
    Saat mengundang pengguna, tetapkan salah satu dari:

    * **Peran Claude Code**: pengguna hanya dapat membuat kunci API Claude Code
    * **Peran Developer**: pengguna dapat membuat jenis kunci API apa pun
  </Step>

  <Step title="Pengguna menyelesaikan pengaturan">
    Setiap pengguna yang diundang perlu:

    * Menerima undangan Console
    * [Periksa persyaratan sistem](/docs/id/setup#system-requirements)
    * [Pasang Claude Code](/docs/id/setup#install-claude-code)
    * Masuk dengan kredensial akun Console
  </Step>
</Steps>

<h3 id="cloud-provider-authentication">
  Autentikasi penyedia cloud
</h3>

Untuk tim yang menggunakan Amazon Bedrock, Google Cloud's Agent Platform, atau Microsoft Foundry:

<Steps>
  <Step title="Ikuti pengaturan penyedia">
    Ikuti [dokumen Amazon Bedrock](/docs/id/amazon-bedrock), [dokumen Google Cloud's Agent Platform](/docs/id/google-vertex-ai), atau [dokumen Microsoft Foundry](/docs/id/microsoft-foundry).
  </Step>

  <Step title="Distribusikan konfigurasi">
    Distribusikan variabel lingkungan dan instruksi untuk menghasilkan kredensial cloud kepada pengguna Anda. Baca lebih lanjut tentang cara [mengelola konfigurasi di sini](/docs/id/settings).
  </Step>

  <Step title="Pasang Claude Code">
    Pengguna dapat [memasang Claude Code](/docs/id/setup#install-claude-code).
  </Step>
</Steps>

<h2 id="credential-management">
  Manajemen kredensial
</h2>

Claude Code mengelola kredensial autentikasi Anda dengan aman:

* **Lokasi penyimpanan**:
  * Di macOS, kredensial disimpan di Keychain macOS yang terenkripsi.
  * Di Linux, kredensial disimpan di `~/.claude/.credentials.json` dengan mode file `0600`.
  * Di Windows, kredensial disimpan di `%USERPROFILE%\.claude\.credentials.json` dan mewarisi kontrol akses dari direktori profil pengguna Anda, yang membatasi file ke akun pengguna Anda secara default.
  * Jika Anda telah menetapkan variabel lingkungan `CLAUDE_CONFIG_DIR` di Linux atau Windows, file `.credentials.json` berada di bawah direktori tersebut.
  * Claude Code mengelola `.credentials.json` melalui `/login` dan `/logout`. Untuk merutekan permintaan melalui titik akhir API kustom, atur variabel lingkungan [`ANTHROPIC_BASE_URL`](/docs/id/env-vars) sebagai gantinya.
* **Jenis autentikasi yang didukung**: kredensial Claude.ai, kredensial API Claude, Microsoft Foundry Auth, Bedrock Auth, Vertex Auth, dan token sesi [gateway aplikasi Claude](/docs/id/claude-apps-gateway).
* **Skrip kredensial kustom**: pengaturan [`apiKeyHelper`](/docs/id/settings#available-settings) dapat dikonfigurasi untuk menjalankan skrip shell yang mengembalikan kunci API.
* **Interval penyegaran**: secara default, `apiKeyHelper` dipanggil setelah 5 menit atau pada respons HTTP 401. Atur variabel lingkungan `CLAUDE_CODE_API_KEY_HELPER_TTL_MS` untuk interval penyegaran kustom.
* **Pemberitahuan helper lambat**: jika `apiKeyHelper` membutuhkan waktu lebih lama dari 10 detik untuk mengembalikan kunci, Claude Code menampilkan pemberitahuan peringatan di bilah prompt yang menunjukkan waktu yang telah berlalu. Jika Anda melihat pemberitahuan ini secara teratur, periksa apakah skrip kredensial Anda dapat dioptimalkan.
* **Kegagalan helper**: ketika skrip keluar dengan kesalahan, habis waktu, atau tidak mencetak apa pun, permintaan gagal dengan [`Your apiKeyHelper script is failing`](/docs/id/errors#your-apikeyhelper-script-is-failing) dalam tiga upaya. Sebelum v2.1.208, kegagalan helper muncul sebagai 401 generik setelah sekitar sepuluh upaya ulang diam.

`apiKeyHelper`, `ANTHROPIC_API_KEY`, dan `ANTHROPIC_AUTH_TOKEN` berlaku untuk CLI dan permukaan yang membungkusnya, termasuk ekstensi VS Code, Agent SDK, dan GitHub Actions. Claude Desktop dan sesi cloud tidak memanggil `apiKeyHelper` atau membaca variabel lingkungan ini: mereka menggunakan OAuth, kecuali sesi desktop yang menjalankan [konfigurasi inferensi pihak ketiga](/docs/id/llm-gateway-connect#desktop-app), yang melakukan autentikasi dengan kredensial konfigurasi tersebut.

<h3 id="renew-an-expiring-login">
  Perbarui login yang akan kedaluwarsa
</h3>

Ketika login yang Anda buat dengan `/login` dalam lima hari akan kedaluwarsa, Claude Code menampilkan peringatan saat startup: `Your login expires in 3 days · run /login to renew`. Memerlukan Claude Code v2.1.203 atau lebih baru.

Jalankan `/login` untuk memperbarui. Peringatan bersifat informatif dan tidak pernah memblokir permintaan: autentikasi terus bekerja sampai login benar-benar kedaluwarsa. Masa hidup login itu sendiri tidak berubah; peringatan awal adalah apa yang ditambahkan v2.1.203.

Setelah login yang disimpan kedaluwarsa dan tidak dapat disegarkan, setiap permintaan gagal dengan [`Login expired · Please run /login`](/docs/id/errors#login-expired) sampai Anda masuk lagi. Sebelum v2.1.206, login yang kedaluwarsa muncul sebagai kesalahan model sebagai gantinya.

Peringatan muncul hanya ketika login claude.ai atau Claude Console adalah kredensial aktif, dan bukan ketika penyedia cloud, `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN`, atau `apiKeyHelper` menyediakan kredensial.

Memperbarui lebih awal paling penting untuk sesi yang berjalan tanpa pengawasan. Sesi [background session in agent view](/docs/id/agent-view) atau sesi [Remote Control](/docs/id/remote-control) yang melampaui login berhenti membuat kemajuan setelah kredensial kedaluwarsa dan tidak dapat pulih sampai Anda masuk lagi.

<h3 id="authentication-precedence">
  Urutan prioritas autentikasi
</h3>

Ketika beberapa kredensial ada, Claude Code memilih salah satu dalam urutan ini:

1. Kredensial penyedia cloud, ketika `CLAUDE_CODE_USE_BEDROCK`, `CLAUDE_CODE_USE_VERTEX`, atau `CLAUDE_CODE_USE_FOUNDRY` diatur. Lihat [integrasi pihak ketiga](/docs/id/third-party-integrations) untuk pengaturan.
2. Variabel lingkungan `ANTHROPIC_AUTH_TOKEN`. Dikirim sebagai header `Authorization: Bearer`. Gunakan ini saat merutekan melalui [gateway LLM atau proxy](/docs/id/llm-gateway) yang melakukan autentikasi dengan token bearer daripada kunci API Anthropic.
3. Variabel lingkungan `ANTHROPIC_API_KEY`. Dikirim sebagai header `X-Api-Key`. Gunakan ini untuk akses API Anthropic langsung dengan kunci dari [Claude Console](https://platform.claude.com). Dalam mode interaktif, Anda diminta sekali untuk menyetujui atau menolak kunci, dan pilihan Anda diingat. Untuk mengubahnya nanti, gunakan toggle "Use custom API key" di `/config`. Toggle hanya muncul saat `ANTHROPIC_API_KEY` diatur di lingkungan Anda. Dalam mode non-interaktif (`-p`), kunci selalu digunakan saat ada.
4. Output skrip [`apiKeyHelper`](/docs/id/settings#available-settings). Gunakan ini untuk kredensial dinamis atau berputar, seperti token berumur pendek yang diambil dari vault.
5. Variabel lingkungan `CLAUDE_CODE_OAUTH_TOKEN`. Token OAuth berumur panjang yang dihasilkan oleh [`claude setup-token`](#generate-a-long-lived-token). Gunakan ini untuk pipeline CI dan skrip di mana login browser tidak tersedia.
6. Kredensial OAuth langganan dari `/login`. Ini adalah default untuk pengguna Claude Pro, Max, Team, dan Enterprise.

Sesi [gateway aplikasi Claude](/docs/id/claude-apps-gateway) yang sudah masuk berada di luar daftar ini: ini adalah pemilihan penyedia seperti Amazon Bedrock atau Agent Platform Google Cloud, dan itu mengungguli mereka. Ketika sesi gateway ada, CLI melakukan autentikasi dengan token gateway bahkan jika `CLAUDE_CODE_USE_BEDROCK`, `CLAUDE_CODE_USE_VERTEX`, atau `CLAUDE_CODE_USE_FOUNDRY` diatur, dan entri token bearer, kunci API, dan `apiKeyHelper` di atas tidak digunakan.

Jika Anda memiliki langganan Claude aktif tetapi juga memiliki `ANTHROPIC_API_KEY` diatur di lingkungan Anda, kunci API memiliki prioritas setelah disetujui. Ini dapat menyebabkan kegagalan autentikasi jika kunci milik organisasi yang dinonaktifkan atau kedaluwarsa. Jalankan `unset ANTHROPIC_API_KEY` untuk kembali ke langganan Anda, dan periksa `/status` untuk mengonfirmasi metode mana yang aktif. Baris `Login method` menunjukkan akun langganan Anda, dan baris `API key` muncul saat kunci API sedang digunakan.

[Claude Code di Web](/docs/id/claude-code-on-the-web) selalu menggunakan kredensial langganan Anda. Jika Anda menetapkan `ANTHROPIC_API_KEY` atau `ANTHROPIC_AUTH_TOKEN` di lingkungan sandbox, itu tidak menimpa kredensial langganan Anda.

<h3 id="generate-a-long-lived-token">
  Hasilkan token berumur panjang
</h3>

Untuk pipeline CI, skrip, atau lingkungan lain di mana login browser interaktif tidak tersedia, hasilkan token OAuth satu tahun dengan `claude setup-token`:

```bash theme={null}
claude setup-token
```

Perintah memandu Anda melalui otorisasi OAuth dan mencetak token ke terminal. Perintah tidak menyimpan token di mana pun; salin dan atur sebagai variabel lingkungan `CLAUDE_CODE_OAUTH_TOKEN` di mana pun Anda ingin melakukan autentikasi:

```bash theme={null}
export CLAUDE_CODE_OAUTH_TOKEN=your-token
```

Token ini melakukan autentikasi dengan langganan Claude Anda dan memerlukan paket Pro, Max, Team, atau Enterprise. Token ini dibatasi untuk inferensi saja dan tidak dapat membuat sesi [Remote Control](/docs/id/remote-control).

[Mode bare](/docs/id/headless#start-faster-with-bare-mode) tidak membaca `CLAUDE_CODE_OAUTH_TOKEN`. Jika skrip Anda melewatkan `--bare`, lakukan autentikasi dengan `ANTHROPIC_API_KEY` atau `apiKeyHelper` sebagai gantinya.
