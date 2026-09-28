> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Autentikasi

> Masuk ke Claude Code dan konfigurasikan autentikasi untuk individu, tim, dan organisasi.

Claude Code mendukung berbagai metode autentikasi tergantung pada pengaturan Anda. Pengguna individual dapat masuk dengan akun claude.ai, sementara tim dapat menggunakan Claude for Teams atau Enterprise, Claude Console, atau penyedia cloud seperti Amazon Bedrock, Google Cloud's Agent Platform, atau Microsoft Foundry.

<h2 id="log-in-to-claude-code">
  Masuk ke Claude Code
</h2>

Setelah [memasang Claude Code](/docs/id/setup#install-claude-code), jalankan `claude` di terminal Anda. Pada peluncuran pertama, Claude Code membuka jendela browser untuk Anda masuk. Jika Anda telah menetapkan variabel lingkungan `ANTHROPIC_API_KEY`, Claude Code melewati prompt login dan meminta Anda menyetujui kunci sebagai gantinya.

Jika browser tidak terbuka secara otomatis, tekan `c` untuk menyalin URL login ke clipboard Anda, kemudian tempel ke browser Anda.

Jika browser Anda menampilkan kode login alih-alih pengalihan kembali setelah Anda masuk, tempel ke terminal di prompt `Paste code here if prompted`. Ini terjadi ketika browser tidak dapat menjangkau server callback lokal Claude Code, yang umum terjadi di WSL2, sesi SSH, dan kontainer.

Ketika login selesai, terminal menampilkan `Login successful` dan meminta Anda menekan `Enter` untuk melanjutkan.

Anda dapat melakukan autentikasi dengan salah satu jenis akun berikut:

* **Langganan Claude Pro atau Max**: masuk dengan akun claude.ai Anda. Berlangganan di [claude.com/pricing](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=authentication_pro_max).
* **Claude for Teams atau Enterprise**: masuk dengan akun claude.ai yang diundang oleh admin tim Anda.
* **Claude Console**: masuk dengan kredensial Console Anda. Admin Anda harus telah [mengundang Anda](#claude-console-authentication) terlebih dahulu. Anda dapat masuk dengan atau tanpa [membuat kunci API](#sign-in-without-an-api-key).
* **Penyedia cloud**: jika organisasi Anda menggunakan [Amazon Bedrock](/docs/id/amazon-bedrock), [Google Cloud's Agent Platform](/docs/id/google-vertex-ai), atau [Microsoft Foundry](/docs/id/microsoft-foundry), atur variabel lingkungan yang diperlukan sebelum menjalankan `claude`, atau pilih **3rd-party platform** di prompt login, yang meluncurkan wizard pengaturan interaktif untuk Bedrock dan Vertex AI. Tidak diperlukan login browser.
* **Gateway cloud**: jika organisasi Anda menjalankan [gateway aplikasi Claude](/docs/id/claude-apps-gateway) yang di-host sendiri, masuk dengan SSO perusahaan melalui `/login`. Token yang dikeluarkan gateway adalah satu-satunya kredensial sesi.

Admin dapat mengarahkan metode login mana yang digunakan pengembang dan memerlukan login claude.ai untuk milik organisasi tertentu; lihat [Batasi login ke organisasi Anda](#restrict-login-to-your-organization).

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

* **Claude for Teams**: paket layanan mandiri dengan fitur kolaborasi, alat admin, SSO, manajemen penagihan, dan [pengaturan yang dikelola server](/docs/id/server-managed-settings) untuk konfigurasi Claude Code di seluruh organisasi. Terbaik untuk tim yang lebih kecil.
* **Claude for Enterprise**: menambahkan penangkapan domain, izin berbasis peran, dan API kepatuhan. Terbaik untuk organisasi yang lebih besar dengan persyaratan keamanan dan kepatuhan.

<Steps>
  <Step title="Berlangganan">
    Berlangganan [Claude for Teams](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=authentication_teams_step#team-&-enterprise) atau hubungi penjualan untuk [Claude for Enterprise](https://anthropic.com/contact-sales?utm_source=claude_code\&utm_medium=docs\&utm_content=authentication_enterprise_step).
  </Step>

  <Step title="Undang anggota tim">
    Undang anggota tim dari dasbor admin.
  </Step>

  <Step title="Pasang dan masuk">
    Anggota tim memasang Claude Code dan masuk dengan akun claude.ai mereka.
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

<h4 id="sign-in-without-an-api-key">
  Masuk tanpa kunci API
</h4>

Anda dapat masuk ke akun Console Anda tanpa membuat kunci API, bahkan ketika organisasi Anda tidak membiarkan pengembang membuatnya. Pilih akun Anthropic Console di prompt `/login` dan Claude Code menanyakan bagaimana Anda ingin masuk. Memerlukan Claude Code v2.1.242 atau lebih baru. Kedua rute menandatangani Anda ke Console di browser dan berbeda dalam apa yang Claude Code simpan setelahnya:

* **Masuk dengan akun Console Anda**, berlabel `(recommended)`: Claude Code menyimpan token OAuth dari masuk itu dan menyimpannya sebagai [profil Anthropic](#anthropic-profiles-and-federation-credentials). Ini tidak membuat kunci API
* **Buat kunci API**, berlabel `(legacy)`: Claude Code membuat kunci API Console untuk Anda dan menyimpannya dengan kredensial lainnya

Dalam praktiknya, profil menyimpan login OAuth sementara kunci API adalah kredensial statis: Claude Code menyegarkan login profil secara otomatis, dan ketika penyegaran gagal, permintaan gagal dengan [login profil Anthropic kedaluwarsa](/docs/id/errors#anthropic-profile-login-expired) sampai Anda masuk lagi.

Anda tidak mendapatkan pilihan di setiap mesin. Claude Code membuat kunci API tanpa bertanya dalam kasus-kasus ini:

* Anda menjalankan terhadap penyedia cloud, seperti [Amazon Bedrock, Google Cloud's Agent Platform, atau Microsoft Foundry](/docs/id/third-party-integrations) atau [Claude Platform di AWS](/docs/id/claude-platform-on-aws)
* File pengaturan apa pun menetapkan [`forceLoginOrgUUID`](#restrict-login-to-your-organization), atau menetapkan `forceLoginMethod` ke `"claudeai"` atau `"console"`
* Sumber pengaturan terkelola di mesin Anda, seperti file pengaturan terkelola, profil MDM, atau pengaturan terkelola server yang di-cache, ada tetapi Claude Code [tidak dapat membacanya](/docs/id/managed-settings#invalid-entries-in-managed-settings) dan tidak ada sumber terkelola lain yang menyediakan kebijakan

Batalkan `ANTHROPIC_API_KEY` sebelum Anda masuk tanpa kunci. Profil yang ditulis oleh masuk Console Claude Code sendiri, atau oleh CLI Claude Platform `ant auth login`, adalah jenis kredensial yang sama, jadi masuk lagi menggantinya.

Setelah Anda masuk tanpa kunci, Anda memiliki profil alih-alih kunci API yang disimpan:

* **Profil mana yang ditulis**: Claude Code menulis profil yang dinamai oleh `ANTHROPIC_PROFILE`, atau profil aktif Anda, atau `default`. Jika profil itu adalah profil federasi, Claude Code menolak masuk alih-alih menimpanya
* **Apa yang ditandatangani keluar**: Claude Code menandatangani Anda keluar dari login claude.ai apa pun yang disimpan di mesin
* **Cara membatalkannya**: jalankan `/logout`, yang menghapus dan mencabut kredensial yang ditulis masuk ini

Jika organisasi Anda menggunakan [pengaturan terkelola server](/docs/id/server-managed-settings), mereka berlaku untuk masuk ini pada Claude Code v2.1.257 atau lebih baru.

Semuanya tentang profil berlaku untuk masuk ini, termasuk di mana peringkatnya terhadap kredensial lainnya, baris `Profile` yang Anda dapatkan di `/status`, dan fitur yang memerlukan login claude.ai. Lihat [profil Anthropic dan kredensial federasi](#anthropic-profiles-and-federation-credentials).

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

<h3 id="restrict-login-to-your-organization">
  Batasi login ke organisasi Anda
</h3>

Untuk memerlukan bahwa login claude.ai pengembang milik organisasi Anthropic tertentu, atur [`forceLoginMethod`](/docs/id/settings-reference#forceloginmethod) dan [`forceLoginOrgUUID`](/docs/id/settings-reference#forceloginorguuid) dalam [pengaturan terkelola](/docs/id/managed-settings). Atur `forceLoginOrgUUID` ke ID organisasi Anda, ditampilkan dalam [pengaturan admin claude.ai](https://claude.ai/admin-settings/organization) untuk organisasi Claude for Teams atau Enterprise. Claude Code melaporkan kesalahan untuk login claude.ai ke organisasi lain dan keluar saat startup jika kredensial claude.ai yang digunakan milik organisasi yang tidak terdaftar.

Untuk login Claude Console, Claude Code menggunakan `forceLoginOrgUUID` untuk pra-pilih organisasi di halaman masuk Console ketika Anda menetapkannya ke ID organisasi Console tunggal, ditampilkan di [platform.claude.com/settings/organization](https://platform.claude.com/settings/organization). Ini tidak memeriksa organisasi mana yang dimiliki kredensial Console yang dihasilkan, saat login atau saat startup, dan pengembang yang masuk dengan akun Console sebelum Anda menerapkan kunci tetap masuk.

Jika Anda menetapkan `forceLoginOrgUUID` dalam file pengaturan apa pun, Claude Code berhenti menawarkan [masuk Console tanpa kunci](#sign-in-without-an-api-key) dalam sesi yang berlaku file itu dan membuat kunci API sebagai gantinya. Untuk mengarahkan pengembang ke masuk claude.ai sebagai gantinya, atur `forceLoginMethod` ke `"claudeai"`.

Pengembang dapat masuk dari beberapa jalur: alur terminal `/login`, [ekstensi VS Code](/docs/id/vs-code), Agent SDK, `claude setup-token`, `/install-github-app`, dan [masuk gateway](/docs/id/claude-apps-gateway) untuk organisasi yang merutekan melalui gateway cloud. Pada Claude Code v2.1.212 atau lebih baru, setiap jalur menerapkan `forceLoginMethod`; sebelum v2.1.212, hanya login terminal yang menerapkan salah satu kunci. Di layar login interaktif terminal, dicapai oleh `/login` atau onboarding pertama kali, Claude Code pra-pilih metode `claudeai` atau `console` tanpa memberlakukannya, jadi bahkan dengan `forceLoginMethod` diatur ke `"claudeai"`, pengembang masih dapat menyelesaikan login Console di sana. Jalur berbeda pada `forceLoginOrgUUID`:

* **Login terminal, ekstensi VS Code, dan Agent SDK**: verifikasi `forceLoginOrgUUID` untuk login akun claude.ai
* **`claude setup-token` dan `/install-github-app`**: hanya berlakukan `forceLoginMethod`, jadi mereka dapat membuat token di organisasi yang berbeda
* **Masuk [gateway](/docs/id/claude-apps-gateway)**: dipilih oleh `forceLoginMethod: "gateway"` daripada dibatasi olehnya, dan tidak mengautentikasi terhadap organisasi Anthropic, jadi `forceLoginOrgUUID` tidak berlaku; gunakan penyedia identitas gateway Anda untuk membatasi akses

Terapkan kunci melalui alat manajemen perangkat Anda. [Pengaturan terkelola server](/docs/id/server-managed-settings) hanya menjangkau akun yang sudah diautentikasi ke organisasi Anda, jadi mereka tidak dapat mengarahkan login pertama pengembang. Jika organisasi Anda juga mendistribusikan pengaturan terkelola server, atur kunci di kedua tempat: sumber [pengaturan terkelola tidak bergabung](/docs/id/server-managed-settings#settings-precedence), dan pengaturan terkelola server yang di-cache menggantikan file yang dikelola perangkat, terlepas dari beberapa [pengecualian per-kunci](/docs/id/server-managed-settings#per-key-exceptions-across-managed-sources). `forceLoginOrgUUID` dan nilai `"claudeai"` dan `"console"` dari `forceLoginMethod` bukan di antara pengecualian tersebut, jadi simpan di kedua tempat.

Kunci juga memutuskan apakah sesi yang tidak menggunakan kredensial login dapat dimulai. Lihat [`forceLoginOrgUUID`](/docs/id/settings-reference#forceloginorguuid) dalam referensi pengaturan untuk perilaku lengkap.

* **`ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN`, atau `apiKeyHelper`**: diblokir saat startup, karena keanggotaan organisasi tidak dapat diverifikasi untuk kredensial lingkungan
* **Sesi penyedia cloud seperti Amazon Bedrock**: tidak diblokir, karena mereka mengautentikasi terhadap penyedia cloud Anda. Batasi mereka melalui kebijakan IAM cloud Anda
* **[Profil Anthropic atau kredensial federasi](#anthropic-profiles-and-federation-credentials)**: tidak diblokir, dan kunci tidak memeriksa organisasi mana yang dimiliki profil

<h2 id="credential-management">
  Manajemen kredensial
</h2>

Claude Code mengelola kredensial autentikasi Anda dengan aman:

* **Lokasi penyimpanan**:
  * Di macOS, kredensial disimpan di Keychain macOS yang terenkripsi. Ketika Keychain menolak penulisan, seperti ketika terkunci dalam sesi SSH, Claude Code menyimpan login Anda di `~/.claude/.credentials.json` dengan mode file `0600` sebagai gantinya, penyimpanan yang sama yang digunakannya di Linux. Login Console yang membuat kunci API gagal sampai Keychain dapat ditulis. Untuk memindahkan login Anda kembali ke Keychain, ikuti [langkah pemulihan](/docs/id/troubleshoot-install#not-logged-in-or-token-expired).
  * Di Linux, kredensial disimpan di `~/.claude/.credentials.json` dengan mode file `0600`.
  * Di Windows, kredensial disimpan di `%USERPROFILE%\.claude\.credentials.json` dan mewarisi kontrol akses dari direktori profil pengguna Anda, yang membatasi file ke akun pengguna Anda secara default.
  * Jika Anda telah menetapkan variabel lingkungan `CLAUDE_CONFIG_DIR`, Claude Code menyimpan file `.credentials.json` di bawah direktori tersebut sebagai gantinya, termasuk file yang ditulis fallback macOS, dan memberi kunci entri Keychain macOS ke direktori tersebut juga, sehingga sesi dengan `CLAUDE_CONFIG_DIR` yang berbeda membaca entri yang berbeda.
  * Claude Code mengelola `.credentials.json` melalui `/login` dan `/logout`. Untuk merutekan permintaan melalui titik akhir API kustom, atur variabel lingkungan [`ANTHROPIC_BASE_URL`](/docs/id/env-vars) sebagai gantinya.
* **Jenis autentikasi yang didukung**: kredensial claude.ai, kredensial API Claude, Microsoft Foundry Auth, Bedrock Auth, Vertex Auth, kredensial profil Anthropic dan [Workload Identity Federation](https://platform.claude.com/docs/en/manage-claude/workload-identity-federation), dan token sesi [gateway aplikasi Claude](/docs/id/claude-apps-gateway).
* **Skrip kredensial kustom**: konfigurasi pengaturan [`apiKeyHelper`](/docs/id/settings-reference#apikeyhelper) untuk menjalankan skrip shell yang mengembalikan kunci API.
* **Interval penyegaran**: Claude Code menjalankan kembali `apiKeyHelper` setelah lima menit secara default. Atur variabel lingkungan `CLAUDE_CODE_API_KEY_HELPER_TTL_MS` untuk interval penyegaran kustom. Lihat [`apiKeyHelper`](/docs/id/settings-reference#apikeyhelper) untuk kasus lain di mana Claude Code menjalankan kembali helper.
* **Pemberitahuan helper lambat**: jika `apiKeyHelper` membutuhkan waktu lebih lama dari 10 detik untuk mengembalikan kunci, Claude Code menampilkan pemberitahuan peringatan di bilah prompt yang menunjukkan waktu yang telah berlalu. Jika Anda melihat pemberitahuan ini secara teratur, periksa apakah skrip kredensial Anda dapat dioptimalkan.
* **Kegagalan helper**: ketika skrip keluar dengan kesalahan, habis waktu, atau tidak mencetak apa pun, permintaan gagal dengan [`Your apiKeyHelper script is failing`](/docs/id/errors#your-apikeyhelper-script-is-failing) dalam tiga upaya. Sebelum v2.1.208, kegagalan helper muncul sebagai 401 generik setelah sekitar sepuluh upaya ulang diam.

`apiKeyHelper`, `ANTHROPIC_API_KEY`, dan `ANTHROPIC_AUTH_TOKEN` berlaku untuk CLI dan permukaan yang membungkusnya, termasuk ekstensi VS Code, Agent SDK, dan GitHub Actions. Claude Desktop dan sesi cloud tidak memanggil `apiKeyHelper` atau membaca variabel lingkungan ini: mereka menggunakan OAuth, kecuali sesi desktop yang menjalankan [konfigurasi inferensi pihak ketiga](/docs/id/llm-gateway-connect#desktop-app), yang melakukan autentikasi dengan kredensial konfigurasi tersebut.

<h3 id="renew-an-expiring-login">
  Perbarui login yang akan kedaluwarsa
</h3>

Ketika login yang Anda buat dengan `/login` dalam tiga hari akan kedaluwarsa, Claude Code menampilkan peringatan saat startup: `Your login expires in 3 days · run /login to renew`. Memerlukan Claude Code v2.1.203 atau lebih baru. Sebelum v2.1.217, peringatan muncul lima hari keluar.

Jalankan `/login` untuk memperbarui. Peringatan bersifat informatif dan tidak pernah memblokir permintaan: autentikasi terus bekerja sampai login benar-benar kedaluwarsa. Masa hidup login itu sendiri tidak berubah; peringatan awal adalah apa yang ditambahkan v2.1.203.

Setelah login yang disimpan kedaluwarsa dan tidak dapat disegarkan, setiap permintaan model gagal dengan [`Login expired · Please run /login`](/docs/id/errors#login-expired) sampai Anda masuk lagi. Sebelum v2.1.206, Claude Code melaporkan login yang kedaluwarsa pada permintaan model sebagai kesalahan model sebagai gantinya.

Anda dapat memeriksa status ini sebelum permintaan gagal: [`/status`](/docs/id/commands) menampilkan baris `Login` yang membaca `Expired — log in again`, ditambah organisasi dan email yang disimpannya untuk login yang kedaluwarsa. Baris muncul hanya ketika login claude.ai atau Claude Console yang disimpan adalah kredensial aktif. Baris memerlukan Claude Code v2.1.210 atau lebih baru.

Peringatan muncul hanya ketika login claude.ai atau Claude Console adalah kredensial aktif, dan bukan ketika penyedia cloud, `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN`, atau `apiKeyHelper` menyediakan kredensial.

Memperbarui lebih awal paling penting untuk sesi yang berjalan tanpa pengawasan. Sesi [background session in agent view](/docs/id/agent-view) atau sesi [Remote Control](/docs/id/remote-control) yang melampaui login berhenti membuat kemajuan setelah kredensial kedaluwarsa dan tidak dapat pulih sampai Anda masuk lagi.

<h3 id="authentication-precedence">
  Urutan prioritas autentikasi
</h3>

Ketika beberapa kredensial ada, Claude Code memilih salah satu dalam urutan ini:

1. Kredensial penyedia cloud, ketika `CLAUDE_CODE_USE_BEDROCK`, `CLAUDE_CODE_USE_VERTEX`, atau `CLAUDE_CODE_USE_FOUNDRY` diatur. Lihat [integrasi pihak ketiga](/docs/id/third-party-integrations) untuk pengaturan.
2. Variabel lingkungan `ANTHROPIC_AUTH_TOKEN`. Dikirim sebagai header `Authorization: Bearer`. Gunakan ini saat merutekan melalui [gateway LLM atau proxy](/docs/id/llm-gateway) yang melakukan autentikasi dengan token bearer daripada kunci API Anthropic.
3. Variabel lingkungan `ANTHROPIC_API_KEY`. Dikirim sebagai header `X-Api-Key`. Gunakan ini untuk akses API Anthropic langsung dengan kunci dari [Claude Console](https://platform.claude.com). Dalam mode interaktif, Anda diminta sekali untuk menyetujui atau menolak kunci, dan pilihan Anda diingat. Untuk mengubahnya nanti, gunakan toggle "Use custom API key" di `/config`. Toggle hanya muncul saat `ANTHROPIC_API_KEY` diatur di lingkungan Anda. Dalam mode non-interaktif (`-p`), kunci selalu digunakan saat ada.
4. Output skrip [`apiKeyHelper`](/docs/id/settings-reference#apikeyhelper). Gunakan ini untuk kredensial dinamis atau berputar, seperti token berumur pendek yang diambil dari vault.
5. Variabel lingkungan `CLAUDE_CODE_OAUTH_TOKEN`. Token OAuth berumur panjang yang dihasilkan oleh [`claude setup-token`](#generate-a-long-lived-token). Gunakan ini untuk pipeline CI dan skrip di mana login browser tidak tersedia. Jika Anda menjalankan `/login` saat variabel diatur, Claude Code beralih sesi saat ini ke login baru, tetapi membaca variabel lagi di setiap sesi baru sampai Anda menghapusnya dari profil shell atau blok `env` dari [file pengaturan](/docs/id/settings).
6. Kredensial profil Anthropic dan federasi, kredensial yang digunakan CLI `ant` dan Workload Identity Federation. Profil yang ditulis `ant auth login` hanya menempati peringkat di sini ketika Anda menamakannya di `ANTHROPIC_PROFILE`; jika tidak, itu menempati peringkat di bawah `/login`. Lihat [Profil Anthropic dan kredensial federasi](#anthropic-profiles-and-federation-credentials).
7. Kredensial OAuth langganan dari `/login`. Ini adalah default untuk pengguna Claude Pro, Max, Team, dan Enterprise.

Sesi [gateway aplikasi Claude](/docs/id/claude-apps-gateway) yang sudah masuk berada di luar daftar ini: ini adalah pemilihan penyedia seperti Amazon Bedrock atau Agent Platform Google Cloud, dan itu mengungguli mereka. Ketika sesi gateway ada, CLI melakukan autentikasi dengan token gateway bahkan jika `CLAUDE_CODE_USE_BEDROCK`, `CLAUDE_CODE_USE_VERTEX`, atau `CLAUDE_CODE_USE_FOUNDRY` diatur, dan sumber kredensial di atas seperti token bearer, kunci API, `apiKeyHelper`, dan profil tidak digunakan.

Jika [pengaturan terkelola](/docs/id/managed-settings) mesin Anda menetapkan [`forceLoginMethod`](/docs/id/settings-reference#forceloginmethod) ke `"gateway"` atau menetapkan [`forceLoginGatewayUrl`](/docs/id/settings-reference#forcelogingatewayurl), dan Anda tidak memilih penyedia cloud melalui variabel seperti `CLAUDE_CODE_USE_BEDROCK` atau `CLAUDE_CODE_USE_VERTEX`, sesi Anda hanya menggunakan sign-in gateway. Claude Code melewati sumber kredensial lainnya dan meminta Anda untuk masuk dengan `/login`. Lihat [Administrator policy requires a Cloud gateway sign-in](/docs/id/errors#administrator-policy-requires-a-cloud-gateway-sign-in) untuk apa yang Anda lihat dengan setiap kredensial sisa. Sebelum v2.1.261, atau sebelum v2.1.265 pada mesin yang hanya menetapkan `forceLoginGatewayUrl`, Claude Code menggunakan login yang disimpan sisa pada mesin ini sampai Anda masuk ke gateway.

Jika Anda memiliki langganan Claude aktif tetapi juga memiliki `ANTHROPIC_API_KEY` diatur di lingkungan Anda, Claude Code menggunakan kunci API setelah Anda menyetujuinya. Ini dapat menyebabkan kegagalan autentikasi jika kunci milik organisasi yang dinonaktifkan atau kedaluwarsa.

Jalankan `unset ANTHROPIC_API_KEY` untuk kembali ke langganan Anda, dan periksa `/status` untuk mengonfirmasi metode mana yang aktif. Ketika login dan kunci API keduanya dikonfigurasi, `/status` menandai kredensial yang tidak sedang digunakan.

[Sesi cloud](/docs/id/claude-code-on-the-web) selalu menggunakan kredensial langganan Anda. Jika Anda menetapkan `ANTHROPIC_API_KEY` atau `ANTHROPIC_AUTH_TOKEN` di lingkungan cloud, itu tidak menimpa kredensial langganan Anda.

<h4 id="anthropic-profiles-and-federation-credentials">
  Profil Anthropic dan kredensial federasi
</h4>

Profil adalah file konfigurasi kredensial bernama di [direktori konfigurasi Anthropic](https://platform.claude.com/docs/en/manage-claude/wif-reference#configuration-directory) Anda, secara default `~/.config/anthropic` di macOS dan Linux atau `%APPDATA%\Anthropic` di Windows. Mode auth profil adalah `oidc_federation` ketika Anda menyiapkannya untuk [Workload Identity Federation (WIF)](https://platform.claude.com/docs/en/manage-claude/workload-identity-federation) atau `user_oauth` ketika [`ant auth login`](https://platform.claude.com/docs/en/cli-sdks-libraries/cli/authentication) menulisnya atau Anda [masuk ke akun Console tanpa kunci API](#sign-in-without-an-api-key).

Claude Code tidak membaca profil atau variabel federasi dalam [mode bare](/docs/id/headless#start-faster-with-bare-mode), di Claude Desktop, atau dalam sesi cloud. Dalam sesi tersebut, `/status` tidak menampilkan baris `Profile`.

Claude Code memeriksa tiga sumber dalam urutan ini dan berhenti di yang pertama yang diatur. Tabel menunjukkan apa yang menetapkan setiap sumber dan di mana itu menempati peringkat terhadap kredensial `/login` Anda.

| Sumber            | Ditetapkan oleh                                                                                                                                                     | Peringkat terhadap `/login`                                                                                                                 |
| :---------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------ | :------------------------------------------------------------------------------------------------------------------------------------------ |
| Profil bernama    | `ANTHROPIC_PROFILE`                                                                                                                                                 | Di atas, mode auth apa pun yang dimiliki profil                                                                                             |
| Variabel federasi | `ANTHROPIC_FEDERATION_RULE_ID` dan `ANTHROPIC_ORGANIZATION_ID`, keduanya diatur                                                                                     | Di atas                                                                                                                                     |
| Profil aktif      | File [`active_config`](https://platform.claude.com/docs/en/manage-claude/wif-reference#active-profile) di direktori konfigurasi Anda, atau profil bernama `default` | Di atas ketika mode auth-nya adalah `oidc_federation`; di bawah kredensial `/login` yang berfungsi ketika mode auth-nya adalah `user_oauth` |

Aturan `user_oauth` menjaga profil `ant auth login` yang tersisa dari memindahkan permintaan Anda dari akun yang Anda masuki dengan `/login`. Untuk variabel federasi, Claude Code juga membaca variabel lain dalam [referensi WIF](https://platform.claude.com/docs/en/manage-claude/wif-reference#environment-variables), seperti `ANTHROPIC_IDENTITY_TOKEN_FILE`, ketika menukar token identitas Anda. Untuk format file profil, lihat [referensi WIF](https://platform.claude.com/docs/en/manage-claude/wif-reference#profile-configuration-file).

Untuk mengonfirmasi sumber mana yang dipilih Claude Code, jalankan `/status`. Baris `Profile` menamai sumber sebagai pengganti baris `Login method`. Ketika profil adalah kredensial yang sedang digunakan, baris `Organization` dan `Email` menunjukkan akunnya.

Jika Anda memulai Claude Code dengan `--debug`, itu juga menulis baris `Using Anthropic profile auth` dengan nama sumber ke log debug di `~/.claude/debug/<session-id>.txt`. Ketika Claude Code melewati profil aktif `user_oauth` karena Anda memiliki kredensial `/login` yang berfungsi, itu menulis peringatan ke log debug mengatakan itu menggunakan login claude.ai sebagai gantinya.

Ketika login profil `user_oauth` telah kedaluwarsa dan Claude Code tidak dapat memperbarui, permintaan gagal dengan [Anthropic profile login expired](/docs/id/errors#anthropic-profile-login-expired).

Fitur yang memerlukan login claude.ai Anda, seperti [konektor claude.ai](/docs/id/mcp#use-mcp-servers-from-claude-ai) dan [`/schedule`](/docs/id/routines), tidak tersedia saat salah satu sumber ini dipilih. Untuk menghentikan Claude Code dari memilih sumber:

* **Profil bernama atau variabel federasi**: batalkan pengaturan `ANTHROPIC_PROFILE`, atau batalkan pengaturan variabel federasi apa pun
* **Profil aktif**: jalankan `/logout` untuk profil `user_oauth` yang kredensial saat ini Anda tulis dengan [masuk ke akun Console tanpa kunci API](#sign-in-without-an-api-key), jalankan `ant auth logout` untuk yang kredensial saat ini `ant auth login` tulis, atau hapus file profil dari `configs/` di direktori konfigurasi Anda untuk mode auth apa pun

<h3 id="generate-a-long-lived-token">
  Hasilkan token berumur panjang
</h3>

Untuk pipeline CI, skrip, atau lingkungan lain di mana login browser interaktif tidak tersedia, hasilkan token OAuth satu tahun dengan `claude setup-token`:

```bash theme={null}
claude setup-token
```

Perintah membuka alur otorisasi browser yang sama seperti `/login`, dan token dicetak ke terminal setelah Anda menyetujui akses di browser. Itu tidak menyimpan token di mana pun; salin dan atur sebagai variabel lingkungan `CLAUDE_CODE_OAUTH_TOKEN` di mana pun Anda ingin melakukan autentikasi:

```bash theme={null}
export CLAUDE_CODE_OAUTH_TOKEN=your-token
```

Token ini melakukan autentikasi dengan langganan Claude Anda dan memerlukan paket Pro, Max, Team, atau Enterprise. Itu hanya dapat membuat permintaan model, jadi itu tidak dapat membuat sesi [Remote Control](/docs/id/remote-control) atau mengambil [konektor claude.ai](/docs/id/mcp#use-mcp-servers-from-claude-ai). Server MCP yang Anda konfigurasi secara lokal masih berfungsi.

[Mode bare](/docs/id/headless#start-faster-with-bare-mode) tidak membaca `CLAUDE_CODE_OAUTH_TOKEN`. Jika skrip Anda melewatkan `--bare`, lakukan autentikasi dengan `ANTHROPIC_API_KEY` atau `apiKeyHelper` sebagai gantinya.
