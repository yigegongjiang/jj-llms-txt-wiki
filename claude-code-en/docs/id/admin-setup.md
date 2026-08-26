> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Siapkan Claude Code untuk organisasi Anda

> Peta keputusan untuk administrator yang menerapkan Claude Code, mencakup penyedia API, pengaturan terkelola, penegakan kebijakan, pemantauan penggunaan, dan penanganan data.

Claude Code memberlakukan kebijakan organisasi melalui pengaturan terkelola yang mengambil alih konfigurasi pengembang lokal. Anda mengirimkan pengaturan tersebut dari konsol admin Claude, sistem manajemen perangkat seluler (MDM) Anda, atau file di disk. Pengaturan mengontrol alat, perintah, server, dan tujuan jaringan mana yang dapat dijangkau Claude.

Halaman ini memandu keputusan penerapan secara berurutan. Setiap baris menautkan ke bagian di bawah dan ke halaman referensi untuk area tersebut.

<Note>
  SSO, penyediaan SCIM, dan penugasan kursi dikonfigurasi di tingkat akun Claude. Lihat [Panduan Administrator Enterprise Claude](https://claude.com/resources/tutorials/claude-enterprise-administrator-guide) dan [penugasan kursi](https://support.claude.com/en/articles/11845131-use-claude-code-with-your-team-or-enterprise-plan) untuk langkah-langkah tersebut.
</Note>

| Keputusan                                                                         | Yang Anda pilih                                             | Referensi                                                                                                                                                                     |
| :-------------------------------------------------------------------------------- | :---------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [Pilih penyedia API Anda](#choose-your-api-provider)                              | Tempat Claude Code melakukan autentikasi dan cara penagihan | [Authentication](/docs/id/authentication), [Amazon Bedrock](/docs/id/amazon-bedrock), [Google Cloud's Agent Platform](/docs/id/google-vertex-ai), [Microsoft Foundry](/docs/id/microsoft-foundry) |
| [Tentukan cara pengaturan mencapai perangkat](#decide-how-settings-reach-devices) | Bagaimana kebijakan terkelola mencapai mesin pengembang     | [Server-managed settings](/docs/id/server-managed-settings), [Settings files](/docs/id/settings#settings-files)                                                                         |
| [Tentukan apa yang akan diberlakukan](#decide-what-to-enforce)                    | Alat, perintah, dan integrasi mana yang diizinkan           | [Permissions](/docs/id/permissions), [Sandboxing](/docs/id/sandboxing)                                                                                                                  |
| [Siapkan visibilitas penggunaan](#set-up-usage-visibility)                        | Cara Anda melacak pengeluaran dan adopsi                    | [Analytics](/docs/id/analytics), [Monitoring](/docs/id/monitoring-usage), [Costs](/docs/id/costs)                                                                                            |
| [Tinjau penanganan data](#review-data-handling)                                   | Retensi data dan postur kepatuhan                           | [Data usage](/docs/id/data-usage), [Security](/docs/id/security)                                                                                                                        |

<h2 id="choose-your-api-provider">
  Pilih penyedia API Anda
</h2>

Claude Code terhubung ke Claude melalui salah satu dari beberapa penyedia API. Pilihan Anda mempengaruhi penagihan, autentikasi, postur kepatuhan mana yang Anda warisi, dan fitur Claude Code mana yang dapat digunakan pengembang Anda.

| Penyedia                      | Pilih ini ketika                                                                                                                                    |
| :---------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------- |
| Claude for Teams / Enterprise | Anda menginginkan Claude Code dan claude.ai di bawah satu langganan per-kursi tanpa infrastruktur untuk dijalankan. Ini adalah rekomendasi default. |
| Claude Console                | Anda adalah API-first atau menginginkan penagihan pay-as-you-go                                                                                     |
| Amazon Bedrock                | Anda ingin mewarisi kontrol kepatuhan dan penagihan AWS yang ada                                                                                    |
| Google Cloud's Agent Platform | Anda ingin mewarisi kontrol kepatuhan dan penagihan GCP yang ada                                                                                    |
| Microsoft Foundry             | Anda ingin mewarisi kontrol kepatuhan dan penagihan Azure yang ada                                                                                  |

Beberapa fitur Claude Code memerlukan akun claude.ai. [Claude Code di web](/docs/id/claude-code-on-the-web), [Routines](/docs/id/routines), [Code Review](/docs/id/code-review), [Remote Control](/docs/id/remote-control), dan [ekstensi Chrome](/docs/id/chrome) tidak tersedia melalui kunci API Console atau kredensial penyedia cloud saja. Jika Anda menerapkan melalui Amazon Bedrock, Google Cloud's Agent Platform, atau Microsoft Foundry, rencanakan apakah pengembang juga memerlukan kursi Claude for Teams atau Enterprise. Setiap halaman fitur mencantumkan persyaratan rencananya.

Untuk perbandingan penyedia lengkap yang mencakup autentikasi, wilayah, dan kesetaraan fitur, lihat [ikhtisar penerapan enterprise](/docs/id/third-party-integrations). Pengaturan auth setiap penyedia ada di [Authentication](/docs/id/authentication).

Persyaratan proxy dan firewall di [Network configuration](/docs/id/network-config) berlaku terlepas dari penyedia. Jika Anda menginginkan satu titik akhir di depan beberapa penyedia atau pencatatan permintaan terpusat, lihat [LLM gateway](/docs/id/llm-gateway).

<h2 id="decide-how-settings-reach-devices">
  Tentukan cara pengaturan mencapai perangkat
</h2>

Pengaturan terkelola mendefinisikan kebijakan yang mengambil alih konfigurasi pengembang lokal. Claude Code memeriksa empat sumber di bawah ini dalam urutan prioritas dan menerapkan yang pertama yang mengembalikan konfigurasi non-kosong, dengan satu pengecualian: serangkaian kecil [kunci kunci lintas-sumber](/docs/id/settings#settings-precedence), seperti kunci daftar izin sandbox, dihormati ketika sumber yang dikendalikan admin menetapkannya.

| Mekanisme               | Pengiriman                                                                                                                                                                                            | Prioritas | Platform       |
| :---------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :-------- | :------------- |
| Server-managed          | Konsol admin claude.ai, atau [Claude apps gateway](/docs/id/claude-apps-gateway) yang di-host sendiri untuk sign-in gateway                                                                                | Tertinggi | Semua          |
| plist / registry policy | macOS: `com.anthropic.claudecode` plist<br />Windows: `HKLM\SOFTWARE\Policies\ClaudeCode`                                                                                                             | Tinggi    | macOS, Windows |
| File-based managed      | macOS: `/Library/Application Support/ClaudeCode/managed-settings.json`<br />Linux dan WSL: `/etc/claude-code/managed-settings.json`<br />Windows: `C:\Program Files\ClaudeCode\managed-settings.json` | Sedang    | Semua          |
| Windows user registry   | `HKCU\SOFTWARE\Policies\ClaudeCode`                                                                                                                                                                   | Terendah  | Windows saja   |

Sebuah [`policyHelper`](/docs/id/settings#compute-managed-settings-with-a-policy-helper) yang dikonfigurasi mendahului keempat sumber: outputnya menjadi satu-satunya konfigurasi terkelola untuk jalankan. Lihat [Settings precedence](/docs/id/settings#settings-precedence).

Pengaturan terkelola server mencapai perangkat pada waktu autentikasi dan menyegarkan setiap jam selama sesi aktif, tanpa infrastruktur titik akhir. Pengiriman melalui konsol admin claude.ai memerlukan rencana Claude for Teams atau Enterprise. Penerapan pada Amazon Bedrock, Google Cloud's Agent Platform, atau Microsoft Foundry dapat mendapatkan pengiriman jarak jauh yang sama dengan menjalankan [Claude apps gateway](/docs/id/claude-apps-gateway), atau menggunakan salah satu mekanisme berbasis file atau tingkat OS sebagai gantinya.

Jika organisasi Anda mencampur penyedia, konfigurasikan [server-managed settings](/docs/id/server-managed-settings) untuk pengguna claude.ai ditambah [fallback berbasis file atau plist/registry](/docs/id/settings#settings-files) sehingga pengguna lain masih menerima kebijakan terkelola.

Lokasi plist dan HKLM registry bekerja dengan penyedia apa pun dan tahan terhadap gangguan karena memerlukan hak istimewa admin untuk menulis. Registry pengguna Windows di HKCU dapat ditulis tanpa elevasi, jadi perlakukan sebagai default kenyamanan daripada saluran penegakan.

Secara default, WSL hanya membaca jalur file Linux di `/etc/claude-code`. Untuk memperluas kebijakan registry Windows dan `C:\Program Files\ClaudeCode` Anda ke WSL pada mesin yang sama, atur [`wslInheritsWindowsSettings: true`](/docs/id/settings#available-settings) di salah satu sumber Windows yang hanya admin.

Mekanisme apa pun yang Anda pilih, nilai terkelola mengambil alih pengaturan pengguna dan proyek. Pengaturan array seperti `permissions.allow` dan `permissions.deny` menggabungkan entri dari semua sumber, jadi pengembang dapat memperluas daftar terkelola tetapi tidak menghapusnya. Untuk [dua pengecualian](/docs/id/settings#settings-precedence), `fallbackModel` dan `availableModels`, nilai terkelola menggantikan lapisan yang lebih rendah daripada menggabungkan.

Lihat [Server-managed settings](/docs/id/server-managed-settings) dan [Settings files and precedence](/docs/id/settings#settings-files).

<h3 id="wsl-sessions-in-claude-code-desktop">
  Sesi WSL di Claude Code Desktop
</h3>

Di Windows, [Claude Code Desktop dapat menjalankan sesi Code di dalam distribusi WSL 2](/docs/id/desktop-wsl). Proses Claude Code sesi berjalan di dalam distribusi, jadi ia menyelesaikan pengaturan terkelola melalui jalur penemuan WSL di atas: sumber khusus Windows tidak mencapainya kecuali `wslInheritsWindowsSettings: true` diterapkan.

Pada perangkat di mana pengaturan terkelola ada, sesi Desktop WSL tidak tersedia secara default. Jika organisasi Anda ingin mengaktifkannya, hubungi tim akun Anthropic Anda. Ketika mereka diaktifkan:

* Terapkan `wslInheritsWindowsSettings: true` melalui registry HKLM atau file `C:\Program Files\ClaudeCode` sehingga sesi WSL mewarisi kebijakan yang sama dengan sesi host.
* Verifikasi dengan menjalankan `/status` di dalam sesi WSL: baris `Setting sources` harus menampilkan `Enterprise managed settings` dengan sumber Windows yang Anda terapkan, `(HKLM)` atau `(file)`.

Proses di dalam VM utilitas WSL 2 tidak terlihat oleh sensor deteksi titik akhir sisi Windows. Jika Anda menggunakan CrowdStrike Falcon, aktifkan sensor Falcon untuk Linux di WSL 2 dengan dua pengecualian yang diperlukan dokumentasi WSL CrowdStrike, untuk proses mesin virtual WSL dan citra disk VM, sehingga aktivitas proses dan file dalam-distro dapat diamati. [Telemetri eksekusi alat OpenTelemetry](/docs/id/monitoring-usage) Claude Code dipancarkan secara identik untuk sesi WSL dan native.

<h2 id="decide-what-to-enforce">
  Tentukan apa yang akan diberlakukan
</h2>

Pengaturan terkelola dapat mengunci alat, eksekusi sandbox, membatasi server MCP dan sumber plugin, dan mengontrol hook mana yang berjalan. Setiap baris adalah permukaan kontrol dengan kunci pengaturan yang mendorong.

| Kontrol                                                                                | Apa yang dilakukan                                                                                                                                                                                                                                                                | Pengaturan kunci                                                                                                    |
| :------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------ |
| [Permission rules](/docs/id/permissions)                                                    | Izinkan, tanyakan, atau tolak alat dan perintah tertentu                                                                                                                                                                                                                          | `permissions.allow`, `permissions.deny`                                                                             |
| [Permission lockdown](/docs/id/permissions#managed-only-settings)                           | Hanya aturan izin terkelola yang berlaku; nonaktifkan `--dangerously-skip-permissions`                                                                                                                                                                                            | `allowManagedPermissionRulesOnly`, `permissions.disableBypassPermissionsMode`                                       |
| [Sandboxing](/docs/id/sandboxing)                                                           | Isolasi filesystem dan jaringan tingkat OS dengan daftar allowlist domain                                                                                                                                                                                                         | `sandbox.enabled`, `sandbox.network.allowedDomains`                                                                 |
| [Managed policy CLAUDE.md](/docs/id/memory#deploy-organization-wide-claude-md)              | Instruksi di seluruh organisasi dimuat di setiap sesi, tidak dapat dikecualikan                                                                                                                                                                                                   | File di jalur kebijakan terkelola                                                                                   |
| [MCP server control](/docs/id/managed-mcp)                                                  | Batasi server MCP mana yang dapat ditambahkan atau dihubungkan pengguna, atau terapkan set tetap                                                                                                                                                                                  | `allowedMcpServers`, `deniedMcpServers`, `allowManagedMcpServersOnly`, atau file `managed-mcp.json` yang diterapkan |
| [Plugin marketplace control](/docs/id/plugin-marketplaces#managed-marketplace-restrictions) | Batasi sumber marketplace mana yang dapat ditambahkan dan diinstal pengguna, tolak flag CLI yang memuat plugin, agent, dan server MCP untuk satu kali berjalan, dan daftar allowlist plugin marketplace mana yang dapat disarankan                                                | `strictKnownMarketplaces`, `blockedMarketplaces`, `disableSideloadFlags`, `pluginSuggestionMarketplaces`            |
| [Customization lockdown](/docs/id/settings#strictpluginonlycustomization)                   | Blokir skills, agents, hooks, dan server MCP dari sumber pengguna dan proyek, sehingga mereka hanya dapat berasal dari plugin atau pengaturan terkelola                                                                                                                           | `strictPluginOnlyCustomization`                                                                                     |
| [Hook restrictions](/docs/id/settings#hook-configuration)                                   | Hanya hook terkelola yang dimuat; batasi URL hook HTTP                                                                                                                                                                                                                            | `allowManagedHooksOnly`, `allowedHttpHookUrls`                                                                      |
| [Login enforcement](/docs/id/settings#available-settings)                                   | Batasi login interaktif ke metode tertentu atau organisasi Anthropic. Ketika diatur, sesi yang diautentikasi oleh `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN`, atau `apiKeyHelper` diblokir saat startup; sesi penyedia cloud tidak terpengaruh                                   | `forceLoginMethod`, `forceLoginOrgUUID`                                                                             |
| [Disable agent view](/docs/id/agent-view#how-background-sessions-are-hosted)                | Matikan `claude agents`, `--bg`, `/background`, dan supervisor on-demand                                                                                                                                                                                                          | `disableAgentView`                                                                                                  |
| [Model restrictions](/docs/id/model-config#restrict-model-selection)                        | `availableModels` memfilter model mana yang muncul di pemilih. Menambahkan `enforceAvailableModels` juga membatasi model default yang dipilih secara otomatis. Lihat [surface coverage](/docs/id/model-config#surface-coverage) untuk cara pengaturan ini menjangkau CLI, web, dan IDE | `availableModels`, `enforceAvailableModels`                                                                         |
| [Version floor](/docs/id/settings)                                                          | Cegah auto-update dari penginstalan di bawah minimum di seluruh organisasi                                                                                                                                                                                                        | `minimumVersion`                                                                                                    |
| [Required version range](/docs/id/settings)                                                 | Tolak untuk memulai sama sekali ketika versi yang berjalan berada di luar rentang yang disetujui organisasi. Lebih kuat daripada `minimumVersion`, yang hanya memblokir downgrade                                                                                                 | `requiredMinimumVersion`, `requiredMaximumVersion`                                                                  |

Organisasi yang anggotanya melakukan autentikasi melalui claude.ai atau Anthropic API juga dapat mengatur model tanpa menerapkan pengaturan: [pembatasan model organisasi](/docs/id/model-config#organization-model-restrictions) menonaktifkan model individual, [model default organisasi](/docs/id/model-config#organization-default-model) menetapkan model mana yang dimulai sesi baru, dan [batas upaya organisasi](/docs/id/model-config#organization-effort-limits) membatasi tingkat upaya per peran. Ketiga kontrol memerlukan paket Claude Enterprise. Pembatasan model dan batas upaya diberlakukan di sisi server; model default adalah titik awal yang dapat diubah pengguna, kecuali organisasi memberlakukannya. Penegakan tersedia untuk serangkaian organisasi terbatas; tanyakan tim akun Anthropic Anda tentang ketersediaan. Tidak ada kontrol ini yang menjangkau sesi di Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry, atau [Claude Platform on AWS](/docs/id/claude-platform-on-aws); di penyedia tersebut, gunakan `availableModels` di atas untuk pembatasan dan kunci `model` dalam pengaturan terkelola untuk default.

[Claude Code on the web](/docs/id/claude-code-on-the-web) memiliki permukaan admin sendiri: di halaman Cloud environments di pengaturan admin, pemilik dan admin membuat [organization-shared environments](/docs/id/claude-code-on-the-web#organization-shared-environments) yang menetapkan [network access level](/docs/id/claude-code-on-the-web#network-access), variabel lingkungan, dan skrip setup untuk sesi cloud anggota, dan memilih environment default organisasi.

Aturan izin dan sandboxing mencakup lapisan berbeda. Menolak WebFetch memblokir alat fetch Claude, tetapi jika Bash diizinkan, `curl` dan `wget` masih dapat menjangkau URL apa pun. Sandboxing menutup celah itu dengan daftar allowlist domain jaringan yang diberlakukan di tingkat OS.

Untuk model ancaman yang dilindungi kontrol ini, lihat [Security](/docs/id/security).

<h2 id="set-up-usage-visibility">
  Siapkan visibilitas penggunaan
</h2>

Pilih pemantauan berdasarkan apa yang perlu Anda laporkan. Dasbor, API, dan kontrol pengeluaran berbeda antara Claude for Teams atau paket Enterprise dan organisasi Claude Console, jadi periksa kolom Ketersediaan sebelum Anda merencanakan pelaporan Anda di sekitar kemampuan.

| Kemampuan              | Yang Anda dapatkan                                                                                                                   | Ketersediaan                                                                                                                                                                                                                                                    | Mulai dari mana                                       |
| :--------------------- | :----------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------- |
| Usage monitoring       | Ekspor OpenTelemetry sesi, alat, dan token                                                                                           | Semua penyedia                                                                                                                                                                                                                                                  | [Monitoring usage](/docs/id/monitoring-usage)              |
| Analytics dashboard    | Metrik adopsi dan kontribusi dengan papan peringkat di Teams / Enterprise; metrik penggunaan dan pengeluaran per-pengguna di Console | Teams / Enterprise di [claude.ai/analytics](https://claude.ai/analytics/claude-code), Console di [platform.claude.com/claude-code](https://platform.claude.com/claude-code)                                                                                     | [Analytics](/docs/id/analytics)                            |
| Programmatic reporting | Data penggunaan dan biaya per-pengguna melalui API                                                                                   | [Enterprise Analytics API](https://platform.claude.com/docs/en/api/admin/analytics) untuk Enterprise, [Claude Code Analytics API](https://platform.claude.com/docs/en/build-with-claude/claude-code-analytics-api) untuk Console                                | [Costs](/docs/id/costs#manage-costs-for-your-organization) |
| Spend controls         | Batas pengeluaran dan batas laju                                                                                                     | Pengaturan admin untuk Teams / Enterprise, batas ruang kerja untuk Console; di cloud pihak ketiga, kontrol anggaran cloud atau [gateway aplikasi Claude](/docs/id/claude-apps-gateway) dengan [batas pengeluaran](/docs/id/claude-apps-gateway-spend-limits) per-pengguna | [Costs](/docs/id/costs#manage-costs-for-your-organization) |

Di Teams dan Enterprise, angka penggunaan dan pengeluaran per-pengguna berasal dari [laporan pengeluaran](https://support.claude.com/en/articles/12883420-view-usage-analytics-for-team-and-enterprise-plans) di pengaturan analitik organisasi Anda, bukan dasbor analitik. Penyedia cloud mengekspos pengeluaran melalui AWS Cost Explorer, GCP Billing, atau Azure Cost Management. Untuk merencanakan anggaran enterprise di seluruh Claude chat, Claude Code, dan Cowork, lihat [panduan konsumsi Claude Enterprise](https://support.claude.com/en/articles/14782391-claude-enterprise-consumption-guide).

<h2 id="review-data-handling">
  Tinjau penanganan data
</h2>

Pada rencana Team, Enterprise, Claude API, dan penyedia cloud, Anthropic tidak melatih model pada kode atau prompt Anda. Penyedia API Anda menentukan retensi dan postur kepatuhan.

| Topik                     | Yang perlu diketahui                                                                                                  | Mulai dari mana                                |
| :------------------------ | :-------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------- |
| Data usage policy         | Apa yang dikumpulkan Anthropic, berapa lama disimpan, apa yang tidak pernah digunakan untuk pelatihan                 | [Data usage](/docs/id/data-usage)                   |
| Zero Data Retention (ZDR) | Tidak ada yang disimpan setelah permintaan selesai. Tersedia untuk akun yang memenuhi syarat di Claude for Enterprise | [Zero data retention](/docs/id/zero-data-retention) |
| Security architecture     | Model jaringan, enkripsi, autentikasi, jejak audit                                                                    | [Security](/docs/id/security)                       |

Jika Anda memerlukan pencatatan audit tingkat permintaan atau untuk merutekan lalu lintas berdasarkan sensitivitas data, tempatkan gateway antara pengembang dan penyedia Anda: [Claude apps gateway](/docs/id/claude-apps-gateway) yang di-host sendiri mencatat log audit per-permintaan dengan identitas IdP, atau gunakan [LLM gateway](/docs/id/llm-gateway) lainnya. Untuk persyaratan peraturan dan sertifikasi, lihat [Legal and compliance](/docs/id/legal-and-compliance).

<h2 id="verify-and-onboard">
  Verifikasi dan onboard
</h2>

Setelah mengonfigurasi pengaturan terkelola, minta pengembang menjalankan `/status` di dalam Claude Code. Pada tab **Status**, baris `Setting sources` menunjukkan `Enterprise managed settings` diikuti oleh sumber dalam tanda kurung, salah satu dari `(remote)`, `(plist)`, `(HKLM)`, `(HKCU)`, atau `(file)`. Lihat [Verify active settings](/docs/id/settings#verify-active-settings).

Bagikan sumber daya ini untuk membantu pengembang memulai:

* [Quickstart](/docs/id/quickstart): panduan sesi pertama dari instalasi hingga bekerja dengan proyek
* [Common workflows](/docs/id/common-workflows): pola untuk tugas sehari-hari seperti tinjauan kode, refactoring, dan debugging
* [Claude 101](https://anthropic.skilljar.com/claude-101) dan [Claude Code in Action](https://anthropic.skilljar.com/claude-code-in-action): kursus Anthropic Academy dengan kecepatan sendiri

Untuk masalah login, arahkan pengembang ke [authentication troubleshooting](/docs/id/troubleshoot-install#login-and-authentication). Perbaikan paling umum adalah:

* Jalankan `/logout` kemudian `/login` untuk beralih akun
* Jalankan `claude update` jika opsi auth enterprise hilang
* Mulai ulang terminal setelah memperbarui

Jika pengembang melihat "You haven't been added to your organization yet," kursi mereka tidak termasuk akses Claude Code dan perlu diperbarui di konsol admin.

<h2 id="next-steps">
  Langkah berikutnya
</h2>

Dengan penyedia dan mekanisme pengiriman dipilih, lanjutkan ke konfigurasi terperinci:

* [Server-managed settings](/docs/id/server-managed-settings): kirimkan kebijakan terkelola dari konsol admin Claude
* [Settings reference](/docs/id/settings): setiap kunci pengaturan, lokasi file, dan aturan prioritas
* [Monorepos and large repos](/docs/id/large-codebases): pola konfigurasi per-direktori untuk organisasi yang menerapkan ke dalam monorepo
* [Amazon Bedrock](/docs/id/amazon-bedrock), [Google Cloud's Agent Platform](/docs/id/google-vertex-ai), [Microsoft Foundry](/docs/id/microsoft-foundry): penerapan khusus penyedia
* [Claude Enterprise Administrator Guide](https://claude.com/resources/tutorials/claude-enterprise-administrator-guide): SSO, SCIM, manajemen kursi, dan playbook peluncuran
