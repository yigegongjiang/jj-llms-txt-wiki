> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Konfigurasi pengaturan yang dikelola server

> Konfigurasi Claude Code secara terpusat untuk organisasi Anda melalui pengaturan yang dikirimkan server, tanpa memerlukan infrastruktur manajemen perangkat.

Pengaturan yang dikelola server memungkinkan Pemilik organisasi untuk mengonfigurasi Claude Code secara terpusat dari [**Admin Settings > Claude Code > Managed settings**](https://claude.ai/admin-settings/claude-code) di konsol claude.ai. Klien Claude Code secara otomatis mengambil pengaturan ini ketika pengguna melakukan autentikasi dengan kredensial yang memenuhi syarat di platform tempat pengiriman yang dikelola server didukung. Lihat [Ketersediaan platform](#platform-availability) untuk kredensial dan platform yang memenuhi syarat.

<Note>
  Pengaturan yang dikelola server tersedia untuk pelanggan [Claude for Teams](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=server_settings_teams#team-&-enterprise) dan [Claude for Enterprise](https://anthropic.com/contact-sales?utm_source=claude_code\&utm_medium=docs\&utm_content=server_settings_enterprise).
</Note>

<h2 id="requirements">
  Persyaratan
</h2>

Untuk menggunakan pengaturan yang dikelola server, Anda memerlukan:

* Paket Claude for Teams atau Claude for Enterprise
* Peran Owner atau Primary Owner di organisasi Claude Anda, untuk melihat dan mengedit konfigurasi
* Akses jaringan ke `api.anthropic.com`

<h2 id="choose-between-server-managed-and-endpoint-managed-settings">
  Pilih antara pengaturan yang dikelola server dan endpoint
</h2>

Claude Code mendukung dua pendekatan untuk konfigurasi terpusat. Pengaturan yang dikelola server mengirimkan konfigurasi dari server Anthropic. [Pengaturan yang dikelola endpoint](/docs/id/managed-settings#delivery-mechanisms) digunakan langsung ke perangkat melalui kebijakan OS asli (preferensi terkelola macOS, registri Windows) atau file pengaturan terkelola.

| Pendekatan                                                                        | Terbaik untuk                                                          | Model keamanan                                                                                                       |
| :-------------------------------------------------------------------------------- | :--------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------- |
| **Pengaturan yang dikelola server**                                               | Organisasi tanpa MDM, atau pengguna pada perangkat yang tidak dikelola | Pengaturan yang Claude Code ambil dari server Anthropic saat startup dan segarkan setiap jam selama sesi             |
| **[Pengaturan yang dikelola endpoint](/docs/id/managed-settings#delivery-mechanisms)** | Organisasi dengan MDM atau manajemen endpoint                          | Pengaturan digunakan ke perangkat melalui profil konfigurasi MDM, kebijakan registri, atau file pengaturan terkelola |

Jika perangkat Anda terdaftar dalam solusi MDM atau manajemen endpoint, pengaturan yang dikelola endpoint memberikan jaminan keamanan yang lebih kuat karena file pengaturan dapat dilindungi dari modifikasi pengguna di tingkat OS. Pengaturan yang dikelola endpoint tidak mencapai [sesi cloud](/docs/id/model-config#surface-coverage) di lingkungan yang dihosting Anthropic, jadi organisasi yang menggunakan Claude Code di web harus mengonfigurasi pengaturan yang dikelola server juga. Sesi di [lingkungan yang dihosting sendiri](/docs/id/self-hosted-environments) juga membaca file pengaturan terkelola di gambar runner. [Prioritas pengaturan](#settings-precedence) di bawah mengatakan kapan file itu berlaku.

<h2 id="configure-server-managed-settings">
  Konfigurasi pengaturan yang dikelola server
</h2>

<Steps>
  <Step title="Buka konsol admin">
    Di konsol claude.ai, buka [**Admin Settings > Claude Code > Managed settings**](https://claude.ai/admin-settings/claude-code).

    Jika tautan mengarahkan ulang Anda ke halaman Admin Settings yang berbeda alih-alih halaman Claude Code, akun Anda tidak memiliki peran yang diperlukan. Peran Admin dan peran non-Owner lainnya tidak dapat melihat atau mengedit pengaturan terkelola, jadi minta Owner atau Primary Owner di organisasi Anda untuk membuat perubahan. Lihat [Kontrol akses](#access-control).
  </Step>

  <Step title="Tentukan pengaturan Anda">
    Tambahkan konfigurasi Anda sebagai JSON. Semua [pengaturan yang tersedia di `settings.json`](/docs/id/settings-reference#all-settings) didukung kecuali yang dibatasi untuk pengiriman kebijakan tingkat OS; lihat [Batasan saat ini](#current-limitations) untuk daftar singkat itu. Ini mencakup [hooks](/docs/id/hooks), [variabel lingkungan](/docs/id/env-vars), dan [pengaturan yang hanya dikelola](/docs/id/managed-settings#managed-only-settings) seperti `allowManagedPermissionRulesOnly`.

    Contoh ini memberlakukan daftar penolakan izin, mencegah pengguna dari melewati izin, dan membatasi aturan izin hanya pada yang ditentukan dalam pengaturan terkelola. Aturan `Bash(curl *)` cocok dengan `curl` [seperti yang ditulis Claude](/docs/id/permissions#bash-rule-limits), bukan `/usr/bin/curl` atau `sh -c 'curl …'`; untuk penegakan jaringan yang tidak bergantung pada teks perintah, tambahkan [blok `sandbox` dengan `allowManagedDomainsOnly`](/docs/id/sandboxing#configure-the-sandbox-for-your-organization).

    ```json theme={null}
    {
      "permissions": {
        "deny": [
          "Bash(curl *)",
          "Read(./.env)",
          "Read(./.env.*)",
          "Read(./secrets/**)"
        ],
        "disableBypassPermissionsMode": "disable"
      },
      "allowManagedPermissionRulesOnly": true
    }
    ```

    Hooks menggunakan format yang sama seperti di `settings.json`.

    Contoh ini menjalankan skrip audit setelah setiap pengeditan file di seluruh organisasi:

    ```json theme={null}
    {
      "hooks": {
        "PostToolUse": [
          {
            "matcher": "Edit|Write",
            "hooks": [
              { "type": "command", "command": "/usr/local/bin/audit-edit.sh" }
            ]
          }
        ]
      }
    }
    ```

    Karena hooks menjalankan perintah shell, pengguna dalam sesi interaktif melihat [dialog persetujuan keamanan](#security-approval-dialogs) sebelum Claude Code menerapkannya.

    Untuk mengonfigurasi pengklasifikasi [mode otomatis](/docs/id/permission-modes#eliminate-prompts-with-auto-mode) sehingga mengetahui repositori, bucket, dan domain mana yang dipercaya organisasi Anda, berikan blok `autoMode` dengan cara yang sama; lihat [Konfigurasi mode otomatis](/docs/id/auto-mode-config) untuk cara entri `autoMode` mempengaruhi apa yang diblokir pengklasifikasi dan peringatan penting tentang bidang `environment`, `allow`, `soft_deny`, dan `hard_deny`.
  </Step>

  <Step title="Simpan dan terapkan">
    Simpan perubahan Anda. Klien Claude Code menerima pengaturan yang diperbarui pada startup berikutnya atau siklus polling per jam.
  </Step>
</Steps>

<h3 id="verify-settings-delivery">
  Verifikasi pengiriman pengaturan
</h3>

Untuk mengonfirmasi bahwa pengaturan sedang diterapkan, minta pengguna untuk memulai ulang Claude Code. Jika konfigurasi mencakup pengaturan yang memicu [dialog persetujuan keamanan](#security-approval-dialogs), pengguna melihat prompt yang menjelaskan pengaturan terkelola pada waktu Claude Code mengambilnya berikutnya: pada startup berikutnya, atau dalam satu jam dalam sesi interaktif yang sedang berjalan. Anda juga dapat memverifikasi bahwa aturan izin terkelola aktif dengan meminta pengguna menjalankan `/permissions` untuk melihat aturan izin efektif mereka.

Untuk memeriksa hasil pengambilan pada mesin tertentu, minta pengguna menjalankan `claude doctor` dan baca baris `Managed settings (remote)`. Memerlukan Claude Code v2.1.248 atau lebih baru. Baris ini melaporkan salah satu dari empat hasil:

* Pengaturan yang dikirimkan dimuat
* Organisasi Anda tidak memiliki pengaturan yang dikelola server yang dikonfigurasi
* Pengambilan gagal, dengan penyebab dan apakah kebijakan yang di-cache masih berlaku
* Claude Code melewati pengambilan, dengan alasannya. Lihat [Ketersediaan platform](#platform-availability) untuk penyedia dan konfigurasi yang melewatinya

Sementara pengambilan masih berlangsung, baris melaporkan itu sebagai gantinya.

Dalam sesi yang sedang berjalan, `/status` menunjukkan baris yang sama setelah pengambilan gagal, dan untuk beberapa penyebab pengambilan yang dilewati, seperti variabel penyedia pihak ketiga atau `ANTHROPIC_BASE_URL` kustom yang diekspor dalam shell pengguna.

<h3 id="access-control">
  Kontrol akses
</h3>

Peran berikut dapat mengelola pengaturan yang dikelola server:

* **Primary Owner**
* **Owner**

Batasi akses ke personel terpercaya, karena perubahan pengaturan berlaku untuk semua pengguna dalam organisasi.

<h3 id="managed-only-settings">
  Pengaturan yang hanya dikelola
</h3>

Sebagian besar [kunci pengaturan](/docs/id/settings-reference#all-settings) bekerja dalam cakupan apa pun. Segelintir kunci hanya dibaca dari pengaturan terkelola dan tidak berpengaruh ketika ditempatkan dalam file pengaturan pengguna atau proyek. Lihat [pengaturan yang hanya dikelola](/docs/id/managed-settings#managed-only-settings) untuk kontrol izin dan plugin, atau baca kolom Scope dari indeks [Semua pengaturan](/docs/id/settings-reference#all-settings) untuk set lengkapnya.

<h3 id="current-limitations">
  Batasan saat ini
</h3>

Pengaturan yang dikelola server memiliki batasan berikut:

* Pengaturan berlaku secara seragam untuk semua pengguna dalam organisasi. Konfigurasi per-grup belum didukung.
* Anda tidak dapat mendistribusikan file [`managed-mcp.json`](/docs/id/managed-mcp) melalui pengaturan yang dikelola server. Berikan kunci kebijakan `allowedMcpServers` dan `deniedMcpServers` di sana sebagai gantinya. Pada Claude Code v2.1.259 atau lebih baru, Anda juga dapat menyediakan server jarak jauh dengan [`managedMcpServers`](/docs/id/managed-mcp#provide-servers-through-managed-settings), yang menerima server `http` dan `sse` saja dan tidak mengambil kontrol eksklusif seperti yang dilakukan file.

  Claude Code membaca file `managed-mcp.json` yang digunakan di [jalur sistemnya](/docs/id/managed-mcp#exclusive-control-with-managed-mcp-json) secara terpisah dari tingkat pengaturan terkelola, jadi file masih berlaku ketika pengaturan yang dikelola server berlaku.
* Pengaturan yang dibatasi untuk sumber kebijakan tingkat OS, seperti `policyHelper` dan `wslInheritsWindowsSettings`, tidak dihormati. Terapkan melalui MDM atau file `managed-settings.json` sistem sebagai gantinya. `policyHelper` yang digunakan dengan cara itu berjalan hanya ketika sumbernya adalah yang dipilih di bawah [prioritas dalam tingkat terkelola](/docs/id/managed-settings#precedence-within-the-managed-tier).

<h2 id="settings-delivery">
  Pengiriman pengaturan
</h2>

<h3 id="settings-precedence">
  Prioritas pengaturan
</h3>

Pengaturan yang dikelola server dan [pengaturan yang dikelola endpoint](/docs/id/managed-settings#delivery-mechanisms) keduanya menempati tingkat tertinggi dalam [hierarki pengaturan](/docs/id/settings#settings-precedence) Claude Code. Tidak ada tingkat pengaturan lain yang dapat menggantinya, termasuk argumen baris perintah, kecuali [pengecualian terhadap prioritas pengaturan terkelola](/docs/id/settings#exceptions-to-managed-settings-precedence).

Dalam tingkat terkelola, Claude Code secara default menggunakan sumber pertama yang mengirimkan setidaknya satu kunci kebijakan, memeriksa pengaturan yang dikelola server terlebih dahulu dan kemudian pengaturan yang dikelola endpoint, kecuali [kunci pengecualian per-kunci yang dibahas selanjutnya](#per-key-exceptions-across-managed-sources). [Bagaimana Claude Code menggabungkan sumber terkelola](/docs/id/managed-settings#precedence-within-the-managed-tier) memiliki peringkat lengkap, pengecualian untuk kunci kontrol, dan opt-in yang berlaku untuk setiap sumber.

Jika sumber yang dipilih adalah kebijakan MDM atau file pengaturan terkelola yang [`policyHelper`](/docs/id/settings-reference#policyhelper)-nya menyediakan pengaturan terkelola, output helper menggantikan sumber tersebut sebagai satu-satunya konfigurasi terkelola untuk jalankan. Claude Code tidak berkonsultasi dengan `policyHelper` yang dikonfigurasi dalam pengaturan berbasis MDM atau file saat pengaturan yang dikelola server mengirimkan kunci kebijakan.

Jika pengambilan yang lebih baru menemukan pengaturan yang dikelola server dihapus, Claude Code menjalankan helper tersebut segera daripada pada peluncuran berikutnya. Entri [`policyHelper`](/docs/id/settings-reference#policyhelper) mencakup apa yang terjadi ketika jalankan tersebut gagal.

Jika Anda menghapus konfigurasi pengaturan yang dikelola server di konsol admin dengan tujuan untuk kembali ke plist yang dikelola endpoint atau kebijakan registri, perhatikan bahwa [pengaturan yang di-cache](#fetch-and-caching-behavior) bertahan pada mesin klien hingga pengambilan berikutnya yang berhasil, dan kunci yang [hanya berlaku pada peluncuran berikutnya](#fetch-and-caching-behavior), seperti `model`, tetap berlaku hingga setiap klien diluncurkan ulang. Jalankan `/status` untuk melihat sumber terkelola mana yang aktif.

<h3 id="per-key-exceptions-across-managed-sources">
  Pengecualian per-kunci di seluruh sumber terkelola
</h3>

Tiga jenis kunci adalah pengecualian terhadap aturan tanpa penggabungan:

* **Kunci kunci lintas sumber**: serangkaian kecil kunci, seperti kunci daftar pasir sandbox, [tercantum di halaman pengaturan terkelola](/docs/id/managed-settings#precedence-within-the-managed-tier). Claude Code menghormatinya ketika sumber terkelola yang dikendalikan admin apa pun menetapkannya; tingkat registri HKCU yang dapat ditulis pengguna dikecualikan.

  Ketika [`policyHelper`](/docs/id/settings-reference#policyhelper) menyediakan pengaturan terkelola, outputnya adalah satu-satunya sumber yang pemeriksaan ini baca, terlepas dari [`forceRemoteSettingsRefresh`](/docs/id/settings-reference#forceremotesettingsrefresh), yang Claude Code baca dari sumber admin secara langsung pada startup.
* **Blok `env`**: terlepas dari unit telemetri dan variabel routing yang dipasangkan dengan kunci kredensial, keduanya dibahas di bawah, blok ini menggabungkan per kunci di seluruh sumber yang dikendalikan admin. Untuk setiap variabel lingkungan, sumber dengan prioritas tertinggi yang mendefinisikannya menang, dan sumber admin yang lebih rendah mengisi variabel yang sumber yang lebih tinggi biarkan tidak diatur. Entri `env` yang dikelola endpoint oleh karena itu berlaku kapan pun konfigurasi yang dikelola server meninggalkan variabel tersebut tidak diatur, atau saat nilai server yang di-cache untuk itu [ditahan menunggu konfirmasi server](#fetch-and-caching-behavior). Memerlukan Claude Code v2.1.223 atau lebih baru. Sebelum v2.1.223, Claude Code menerapkan blok `env` sumber yang dipilih secara keseluruhan saja.
  * **Unit telemetri**: kunci exporter `OTEL_EXPORTER_OTLP_*`, toggle penangkapan konten `OTEL_LOG_*`, `OTEL_LOGS_EXPORTER`, dan variabel tracing beta `ENABLE_BETA_TRACING_DETAILED` dan `BETA_TRACING_ENDPOINT` mengikuti sumber tertinggi yang menetapkan salah satu dari mereka sebagai unit. Sumber yang mengirimkan kunci kredensial `otelHeadersHelper` juga mengklaim unit, tetapi mendarat variabel ini hanya ketika itu adalah sumber yang dipilih: sumber yang tidak dipilih tetapi mengirimkan kunci tidak berkontribusi pada salah satu dari mereka dan masih memblokir sumber yang lebih rendah dari mengisinya. Bagaimanapun, endpoint exporter dari satu sumber tidak pernah dapat dipasangkan dengan kredensial dari sumber lain.
  * **Routing yang dipasangkan kredensial**: sumber yang memasangkan variabel routing dengan kunci kredensial yang hanya dipilih-sumber, seperti `apiKeyHelper` atau `otelHeadersHelper`, berkontribusi pada variabel routing tersebut hanya ketika itu memenangkan slot.
* **Kunci masuk gateway**: Claude Code tidak pernah membaca [`forceLoginGatewayUrl`](/docs/id/settings-reference#forcelogingatewayurl), [`gatewayInternalNetworks`](/docs/id/settings-reference#gatewayinternalnetworks), atau nilai `"gateway"` dari [`forceLoginMethod`](/docs/id/settings-reference#forceloginmethod) dari pengaturan yang dikelola server, jadi nilai di sana tidak berlaku atau menyembunyikan yang diatur dalam kebijakan MDM atau file pengaturan terkelola. Entri [`managedSourcesBehavior`](/docs/id/settings-reference#managedsourcesbehavior) mengatakan sumber admin mana di mesin yang menyediakannya.

<h3 id="fetch-and-caching-behavior">
  Perilaku pengambilan dan caching
</h3>

Claude Code mengambil pengaturan dari server Anthropic pada startup dan melakukan polling untuk pembaruan setiap jam selama sesi aktif.

Klien yang masuk melalui [gateway aplikasi Claude](#platform-availability) mengambil pengaturannya dari gateway dan menunggu pengambilan tersebut sebelum sesi dimulai, jadi pengambilan dalam daftar di bawah tidak berlaku untuk itu. [Paksakan startup yang tertutup gagal](#enforce-fail-closed-startup) mencakup apa yang terjadi ketika pengambilan tersebut gagal.

**Peluncuran pertama tanpa pengaturan yang di-cache:**

* Ketika pengembang masuk pada startup, seperti pada jalankan pertama atau setelah `/logout`, Claude Code menunggu hingga lima detik untuk pengambilan sebelum membuka sesi. Ketika kebijakan tiba tepat waktu, Claude Code memberlakukannya dari layar pertama dan menampilkan [`companyAnnouncements`](/docs/id/settings-reference#companyannouncements) Anda di atasnya. Ketika payload memerlukan [persetujuan keamanan](#security-approval-dialogs), Claude Code mengakhiri penunggu dan menerapkan payload setelah pengembang menyetujui
* Pada startup lainnya, dan ketika penunggu lima detik habis, Claude Code membuka sesi sementara pengambilan berlanjut, jadi jendela singkat berlalu sebelum pengaturan dimuat dan pembatasan berlaku
* Jika pengambilan gagal, Claude Code melanjutkan tanpa pengaturan yang dikelola server dan memperingatkan dalam sesi interaktif bahwa tidak ada kebijakan jarak jauh yang berlaku; pengaturan yang dikelola endpoint masih berlaku. Jika sumber terkelola menetapkan [`forceRemoteSettingsRefresh`](#enforce-fail-closed-startup), Claude Code keluar sebagai gantinya

**Peluncuran berikutnya dengan pengaturan yang di-cache:**

* Pengaturan yang di-cache berlaku segera pada startup, kecuali untuk nilai `modelPricing` dan `managedMcpServers` yang di-cache dan variabel lingkungan yang Claude Code tahan sampai server mengonfirmasi payload
* [`modelPricing`](/docs/id/settings-reference#modelpricing) yang di-cache tidak berlaku hingga pengambilan sesi mengonfirmasi payload. Sampai saat itu, angka biaya yang dilihat pengembang di `/usage` dan baris status berada pada harga daftar
* Blok [`managedMcpServers`](/docs/id/settings-reference#managedmcpservers) yang di-cache tidak berlaku hingga pengambilan sesi mengonfirmasi payload. Claude Code menunggu hingga 30 detik untuk pengambilan tersebut sebelum menghubungkan server MCP. Jika pengambilan gagal atau habis waktu, sesi dimulai tanpa server organisasi, `/status` mengatakan demikian, dan mereka terhubung setelah pengambilan yang lebih baru mengonfirmasinya. Lihat [Ketika server yang disediakan terhubung](/docs/id/managed-mcp#when-provided-servers-connect) untuk perilaku lengkap, termasuk peluncuran pertama. Memerlukan Claude Code v2.1.259 atau lebih baru
* Claude Code mengambil pengaturan segar di latar belakang
* Pengaturan yang di-cache bertahan melalui kegagalan jaringan. Jika pengambilan startup gagal, Claude Code memperingatkan dalam sesi interaktif bahwa kebijakan yang di-cache berlaku
* Sampai pengambilan berhasil, nilai yang ditahan pada startup tetap ditahan

Claude Code menahan beberapa kategori variabel dalam blok `env` yang di-cache hingga server mengonfirmasi payload untuk sesi. Ini mencegah nilai proxy, otoritas sertifikat, endpoint, atau kredensial yang di-cache dari mengarahkan ulang, mencegat, atau melakukan autentikasi ulang pengambilan pengaturan yang mengonfirmasi payload. Pengerasan hanya berlaku pada cache pengaturan yang diambil server: [pengaturan yang dikelola endpoint](/docs/id/managed-settings#delivery-mechanisms) yang digunakan melalui MDM atau `managed-settings.json` tidak terpengaruh. Penahan memerlukan Claude Code v2.1.198 atau lebih baru; sebelum v2.1.198, seluruh blok `env` yang di-cache berlaku pada startup. Kategori yang ditahan mencakup:

* Konfigurasi proxy dan TLS, seperti `HTTPS_PROXY`, `NODE_EXTRA_CA_CERTS`, dan variabel sertifikat klien mTLS `CLAUDE_CODE_CLIENT_CERT` dan `CLAUDE_CODE_CLIENT_KEY`
* Routing API dan pemilihan penyedia, termasuk `ANTHROPIC_BASE_URL`, variabel pemilihan penyedia seperti `CLAUDE_CODE_USE_BEDROCK` dan `CLAUDE_CODE_USE_VERTEX`, dan URL endpoint penyedia seperti `ANTHROPIC_BEDROCK_BASE_URL`
* Kredensial autentikasi, seperti `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN`, dan `CLAUDE_CODE_OAUTH_TOKEN`
* Pemilih direktori konfigurasi `CLAUDE_CONFIG_DIR`
* Pemilih sumber kredensial dan direktori konfigurasi, dalam Claude Code v2.1.223 atau lebih baru: variabel Workload Identity Federation seperti `ANTHROPIC_FEDERATION_RULE_ID` dan `ANTHROPIC_IDENTITY_TOKEN`, pemilih profil dan direktori konfigurasi `ANTHROPIC_PROFILE` dan `ANTHROPIC_CONFIG_DIR`, dan variabel direktori sistem operasi `HOME`, `XDG_CONFIG_HOME`, `APPDATA`, dan `USERPROFILE`

Claude Code membaca variabel Workload Identity Federation dan pemilih `ANTHROPIC_PROFILE` dan `ANTHROPIC_CONFIG_DIR` hanya pada startup, jadi nilai yang dikirimkan server untuk mereka tidak mengalihkan sumber kredensial sesi bahkan setelah pengambilan berhasil. Untuk mengirimkan pemilih tersebut pada Claude Code v2.1.223 atau lebih baru, gunakan [pengaturan yang dikelola endpoint](/docs/id/managed-settings#delivery-mechanisms) seperti MDM atau `managed-settings.json`. Untuk `CLAUDE_CONFIG_DIR` dan variabel direktori sistem operasi, penahan itu sendiri adalah perlindungan: nilai yang di-cache tetap keluar dari lingkungan sampai server mengonfirmasi payload.

Setiap kunci lain dalam blok `env` yang di-cache berlaku pada startup. Setelah server mengonfirmasi payload, dan Anda menyetujuinya jika memerlukan [persetujuan keamanan](#security-approval-dialogs), variabel yang ditahan berlaku untuk sisa sesi.

Jika organisasi Anda memerlukan proxy untuk menjangkau `api.anthropic.com`, penahan hanya mempengaruhi blok `env` yang dikirimkan server itu sendiri: proxy yang diatur dalam blok `env` yang [dikelola endpoint](/docs/id/managed-settings#delivery-mechanisms) melalui MDM atau `managed-settings.json`, di lingkungan shell, atau di [pengaturan pengguna](/docs/id/settings#where-settings-live) mencapai pengambilan pengaturan. Sumber yang dikelola endpoint memerlukan Claude Code v2.1.223 atau lebih baru: nilai proxy yang dikelola server yang di-cache ditahan sampai pengambilan mengonfirmasinya, jadi nilai yang dikelola endpoint mengisi per kunci dan mencapai pengambilan itu sendiri. Sebelum v2.1.223, gunakan lingkungan shell atau pengaturan pengguna sehingga proxy berlaku bersama payload server yang di-cache. Peluncuran pertama tidak memiliki cache, jadi sumber yang dikelola endpoint, lingkungan shell, atau pengaturan pengguna masih diperlukan untuk pengambilan awal.

Claude Code menerapkan sebagian besar pembaruan pengaturan ke sesi yang sedang berjalan tanpa restart. Beberapa pembaruan hanya berlaku pada peluncuran berikutnya, termasuk konfigurasi exporter OpenTelemetry, kunci `model`, dan penghapusan variabel dari blok `env`.

<h3 id="invalid-entries-in-delivered-settings">
  Entri tidak valid dalam pengaturan yang dikirimkan
</h3>

Ketika bagian dari payload gagal validasi skema, Claude Code menampilkan kesalahan validasi dan menerapkan setiap pengaturan yang valid yang tersisa; [Entri tidak valid dalam pengaturan terkelola](/docs/id/managed-settings#invalid-entries-in-managed-settings) mengatakan apa yang dihapusnya dan kunci mana yang kembali ke nilai yang lebih ketat. Memerlukan Claude Code v2.1.169 atau lebih baru.

Pengiriman yang dikelola server menambahkan perilaku ini:

* Cache di `~/.claude/remote-settings.json` menyimpan payload yang diselamatkan dengan entri tidak valid dihapus, terlepas dari nilai `cleanupPeriodDays` dan `desktopSessionCleanupPeriodDays` yang tidak valid, yang tetap dalam salinan yang di-cache dan tidak pernah diterapkan.
* Ketika tidak ada bidang dalam payload yang dapat diselamatkan dan payload bukan hanya kunci retensi tersebut, Claude Code menolak payload, menyimpan pengaturan cache yang terakhir diterima, dan menulis `Remote settings: Settings validation failed - no fields could be salvaged` ke log debug. Dengan `forceRemoteSettingsRefresh` diatur, CLI keluar sebagai gantinya.
* [Dialog persetujuan keamanan](#security-approval-dialogs) mengevaluasi payload yang diselamatkan, jadi entri tidak valid yang dilucuti tidak pernah disajikan untuk persetujuan dan tidak pernah dieksekusi.

Untuk men-debug masalah pengiriman, jalankan `claude --debug-file <path>` dan cari log untuk `Remote settings`. Validasi perubahan payload dengan `claude doctor` pada mesin uji sebelum meluncurkannya ke organisasi.

<h3 id="enforce-fail-closed-startup">
  Paksakan startup yang tertutup gagal
</h3>

Secara default, jika pengambilan pengaturan jarak jauh gagal pada startup, CLI melanjutkan dengan pengaturan yang di-cache dari pengambilan terakhir yang berhasil, kecuali untuk [nilai yang Claude Code tahan](#fetch-and-caching-behavior) sampai pengambilan berhasil. Pada mesin yang tidak pernah mengambilnya, CLI melanjutkan tanpa pengaturan yang dikelola server dan masih menerapkan [pengaturan yang dikelola endpoint](/docs/id/managed-settings#delivery-mechanisms) apa pun di perangkat.

Untuk menghentikan klien agar tidak memulai pada pengaturan yang dikelola server yang di-cache atau tidak ada, atur `forceRemoteSettingsRefresh: true` dalam pengaturan terkelola Anda.

Klien yang masuk melalui [gateway aplikasi Claude](#platform-availability) menunggu pengambilan startup apakah atau tidak Anda mengatur ini, dan menangani pengambilan yang gagal sebagai berikut:

* Jika gateway menjawab peluncuran interaktif yang dihadiri dengan `401` dan pengaturan ini mati, gateway telah mengakhiri masuk tersebut. Claude Code mencetak [`Cloud gateway session expired — run /login to reconnect.`](/docs/id/errors#cloud-gateway-session-expired) dan membuka sesi yang keluar dari gateway sampai pengguna menjalankan `/login`.
* Ketika pengambilan gagal dengan cara lain, atau dalam jenis peluncuran lain apa pun kecuali subperintah `claude auth`, klien keluar dengan kesalahan.

Ketika pengaturan ini aktif dalam sesi yang mengambil pengaturan yang dikelola server, CLI memblokir pada startup sampai pengaturan jarak jauh diambil segar. Jika pengambilan gagal, CLI keluar daripada melanjutkan tanpa kebijakan. Pengaturan ini memperpanjang dirinya sendiri: setelah dikirimkan dari server, pengaturan ini juga di-cache secara lokal sehingga startup berikutnya memberlakukan perilaku yang sama bahkan sebelum pengambilan pertama yang berhasil dari sesi baru. Sesi yang [tidak mengambil pengaturan yang dikelola server](#platform-availability) dimulai tanpa menunggu.

Untuk mengaktifkan ini, tambahkan kunci ke konfigurasi pengaturan terkelola Anda:

```json theme={null}
{
  "forceRemoteSettingsRefresh": true
}
```

Anda juga dapat mengatur kunci ini dalam profil MDM yang [dikelola endpoint](/docs/id/managed-settings#delivery-mechanisms) atau file `managed-settings.json` sistem untuk memberlakukan perilaku tertutup gagal pada peluncuran pertama, sebelum payload server apa pun telah dikirimkan. Flag ini adalah pengecualian terhadap [aturan prioritas](#settings-precedence) di atas: Claude Code menghormatinya ketika diatur dalam sumber terkelola yang dikendalikan admin apa pun, bahkan jika payload yang dikelola server yang di-cache juga ada, jadi nilai yang dikirimkan MDM tidak diabaikan ketika pengaturan yang dikelola server ada.

Ketika [`policyHelper`](/docs/id/settings-reference#policyhelper) menyediakan pengaturan terkelola, outputnya menggantikan setiap sumber terkelola lainnya untuk kunci yang Claude Code baca setelah startup. Untuk sumber yang Claude Code baca kunci ini dari, lihat [entri pengaturannya](/docs/id/settings-reference#forceremotesettingsrefresh). Entri `policyHelper` mengatakan sumber mana Claude Code membaca helper dari dan kapan itu berjalan.

Pengambilan pengaturan juga mengirimkan header `Cache-Control: no-cache` sehingga proxy HTTP perantara tidak melayani respons yang sudah usang.

Sebelum mengaktifkan pengaturan ini, pastikan kebijakan jaringan Anda memungkinkan konektivitas ke `api.anthropic.com`. Jika endpoint tersebut tidak dapat dijangkau, CLI keluar pada startup dan pengguna tidak dapat memulai Claude Code.

Subperintah `claude auth` seperti `claude auth login` dikecualikan dari pemeriksaan ini dan dari keluar startup gateway, sehingga pengguna dapat melakukan autentikasi ulang ketika kredensial yang kedaluwarsa adalah alasan pengambilan pengaturan gagal.

<h3 id="security-approval-dialogs">
  Dialog persetujuan keamanan
</h3>

Pengaturan tertentu yang dapat menimbulkan risiko keamanan memerlukan persetujuan pengguna eksplisit sebelum Claude Code menerapkannya dalam sesi interaktif:

* **Pengaturan perintah shell**: pengaturan yang menjalankan perintah shell, seperti `apiKeyHelper`, `statusLine`, dan `otelHeadersHelper`
* **Pengaturan biner sandbox**: `sandbox.bwrapPath`, `sandbox.socatPath`, dan `sandbox.ripgrep`. Masing-masing pengaturan ini menunjuk pada executable, dan Claude Code menjalankan executable tersebut
* **Pengaturan jaringan dan isolasi sandbox**: pengaturan [sandbox](/docs/id/sandboxing) yang memungkinkan proxy sandbox membaca, mengarahkan ulang, atau mengautentikasi lalu lintas, atau yang melemahkan isolasi sandbox: `sandbox.network.tlsTerminate`, `sandbox.network.httpProxyPort`, `sandbox.network.socksProxyPort`, `sandbox.credentials`, `sandbox.allowAppleEvents`, `sandbox.enableWeakerNestedSandbox`, `sandbox.enableWeakerNetworkIsolation`, `sandbox.filesystem.disabled`, `sandbox.network.allowAllUnixSockets`, `sandbox.network.allowUnixSockets`, dan `sandbox.network.allowMachLookup`. Blok `sandbox.credentials` yang hanya berisi aturan `deny` tidak memerlukan persetujuan, karena membatasi sandbox tanpa memberikan proxy kredensial. Sebelum v2.1.251, Claude Code menerapkan pengaturan ini tanpa persetujuan
* **Variabel lingkungan kustom**: variabel `env` yang dikirimkan yang memerlukan persetujuan pengguna, seperti proxy dan variabel URL dasar; lihat [Variabel lingkungan dan dialog persetujuan](#environment-variables-and-the-approval-dialog)
* **Konfigurasi hook**: definisi hook apa pun

Ketika pengaturan ini ada, pengguna melihat dialog keamanan yang menjelaskan apa yang sedang dikonfigurasi. Pengguna harus menyetujui untuk melanjutkan. Jika pengguna menolak pengaturan, Claude Code keluar.

CLAUDE.md yang dikelola yang dikirimkan melalui kunci [`claudeMd`](/docs/id/settings-reference#claudemd) tidak memerlukan persetujuan, karena itu adalah teks instruksi untuk Claude daripada perintah yang Claude Code jalankan. Claude Code masih memeriksa [izin](/docs/id/permissions) untuk alat yang Claude gunakan saat mengikuti instruksi tersebut. Sebelum v2.1.260, nilai `claudeMd` memerlukan persetujuan juga.

<h4 id="approval-memory">
  Memori persetujuan
</h4>

Claude Code mencatat persetujuan Anda di direktori konfigurasi Anda, `~/.claude` kecuali Anda mengatur [`CLAUDE_CONFIG_DIR`](/docs/id/env-vars). Apa yang dicatat tergantung pada kredensial yang digunakan pengambilan pengaturan:

* **Login claude.ai yang disimpan oleh `/login` atau `claude auth login`, atau [masuk Console tanpa kunci](/docs/id/authentication#sign-in-without-an-api-key)**: satu persetujuan per organisasi, dipegang oleh akun yang menyetujui paling baru.
* **Masuk [gateway aplikasi Claude](/docs/id/claude-apps-gateway)**: satu persetujuan per gateway.

  Jika Anda keluar dan masuk kembali ke gateway yang sama, Claude Code tidak menampilkan dialog lagi saat pengaturan yang memerlukan persetujuan tidak berubah. Claude Code menampilkannya lagi ketika pengaturan tersebut berubah, ketika Anda masuk ke gateway yang berbeda, dan ketika Anda menerima sertifikat baru untuk gateway yang sama.

  Claude Code tidak menyimpan persetujuan untuk gateway pengembangan loopback yang dicapai melalui HTTP biasa, jadi dialog muncul lagi setelah setiap masuk.
* **Kredensial lainnya**, seperti kunci API atau `CLAUDE_CODE_OAUTH_TOKEN`: satu persetujuan untuk pengaturan yang dikirimkan, disimpan dengan salinan yang di-cache dari pengaturan di direktori konfigurasi tersebut. Claude Code menampilkan dialog lagi ketika pengaturan yang memerlukan persetujuan berubah, dan setelah Anda menjalankan `/logout` atau `claude auth logout`, salah satu dari keduanya menghapus salinan yang di-cache.

Persetujuan untuk `sandbox.credentials` atau `sandbox.network.tlsTerminate` juga mencakup entri [`sandbox.network.allowedDomains`](/docs/id/settings-reference#sandbox-network-alloweddomains) dalam pengaturan yang dikirimkan yang sama, karena kedua pengaturan bertindak pada daftar izin tersebut. Dialog muncul lagi ketika administrator Anda menambah atau menghapus salah satu entri tersebut, meskipun `sandbox.network.allowedDomains` tidak memerlukan persetujuan dengan sendirinya.

Dengan login claude.ai yang disimpan:

* Jika Anda keluar dan masuk kembali, atau beralih ke organisasi lain dan kemudian kembali, Claude Code tidak menampilkan dialog lagi saat pengaturan tersebut tidak berubah, kecuali akun lain menyetujuinya untuk organisasi tersebut di direktori konfigurasi yang sama di antara.
* Jika Anda masuk ke organisasi yang sama dengan akun yang berbeda, Claude Code menampilkan dialog lagi bahkan ketika pengaturan tidak berubah. Persetujuan akun tersebut menggantikan yang sebelumnya, jadi ketika Anda beralih kembali, Claude Code menampilkan dialog sekali lagi.

Claude Code tidak selalu dapat menampilkan dialog. Setiap kasus di bawah mengatakan pengaturan mana yang berlaku ketika tidak dapat dan kapan Anda berikutnya melihat dialog:

* **Sesi interaktif yang tidak dapat menampilkan dialog**: Claude Code tidak menerapkan pengaturan yang dikirimkan dan menyimpan pengaturan yang terakhir disetujui. Dialog muncul dalam sesi berikutnya yang dapat menampilkannya. Memerlukan Claude Code v2.1.211 atau lebih baru.
* **`claude install` atau `claude update`**: Claude Code tidak menampilkan dialog selama perintah apa pun. Perintah berjalan dengan pengaturan yang terakhir disetujui, dan dialog muncul dalam sesi interaktif Anda berikutnya. Jika Claude Code menunggu pengambilan pengaturan pada startup, seperti dengan [`forceRemoteSettingsRefresh`](#enforce-fail-closed-startup) diatur atau pada penyebaran [gateway aplikasi Claude](/docs/id/claude-apps-gateway), itu menampilkan dialog selama perintah sebagai gantinya, dan jalankan install dari pipa gagal; lihat [`Raw mode is not supported` during install](/docs/id/troubleshoot-install#raw-mode-is-not-supported-during-install). Sebelum v2.1.246, Claude Code mencoba menampilkan dialog selama perintah ini juga.
* **Kesalahan menutup dialog sebelum Anda menjawab**: Claude Code tidak menerapkan pengaturan yang dikirimkan dan menyimpan pengaturan yang terakhir disetujui. Itu menampilkan dialog lagi dalam sesi berikutnya yang dapat menampilkannya.
* **Jalankan non-interaktif**, seperti `claude -p` atau sesi Agent SDK: Claude Code tidak dapat menampilkan dialog, jadi ketika pengaturan yang dikirimkan memerlukan persetujuan, itu menerapkannya hanya untuk jalankan itu. Itu tidak merekamnya sebagai disetujui atau menulisnya ke [cache lokal](#fetch-and-caching-behavior), dan sesi interaktif berikutnya menampilkan dialog. Sampai pengguna menyetujui dalam sesi interaktif, setiap jalankan non-interaktif mengambil pengaturan lagi pada startup. Sebelum v2.1.207, jalankan non-interaktif menyimpan pengaturan sebagai disetujui, jadi sesi interaktif kemudian tidak pernah menampilkan dialog untuk mereka.

<h4 id="environment-variables-and-the-approval-dialog">
  Variabel lingkungan dan dialog persetujuan
</h4>

Claude Code menerapkan beberapa variabel `env` yang dikirimkan tanpa menampilkan dialog persetujuan pengguna, termasuk:

* Pengalihan fitur dan perintah
* Pemilihan model dan pengaturan perilaku, seperti `ANTHROPIC_MODEL`, `DISABLE_PROMPT_CACHING`, dan `CLAUDE_CODE_EFFORT_LEVEL`
* Jendela konteks dan pengaturan pemadatan, seperti `DISABLE_AUTO_COMPACT`
* Opsi UI terminal dan aksesibilitas
* Batas numerik, anggaran, dan waktu tunggu

Variabel yang dikirimkan lainnya dapat memerlukan persetujuan pengguna sebelum berlaku; nilai proxy, URL dasar, atau `OTEL_EXPORTER_OTLP_ENDPOINT` yang tidak kosong selalu demikian. Ketika variabel yang dikirimkan memerlukan persetujuan, dialog menamakannya, sehingga pengguna melihat dengan tepat apa yang diminta kebijakan untuk diatur. Sebelum v2.1.218, Claude Code menerapkan lebih sedikit variabel tanpa bertanya kepada pengguna, jadi pengaturan seperti `DISABLE_AUTO_COMPACT` memicu dialog pada nilai apa pun yang tidak kosong.

Claude Code memutuskan apakah empat toggle privasi memerlukan persetujuan berdasarkan nilai yang dikirimkan daripada nama variabel: `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`, `DISABLE_ERROR_REPORTING`, `DISABLE_TELEMETRY`, dan `DO_NOT_TRACK`. Nilai yang benar seperti `1` atau `true` hanya mematikan pelacakan, pelaporan, atau lalu lintas nonessential lainnya, jadi Claude Code menerapkannya tanpa bertanya kepada pengguna. Untuk nilai apa pun yang tidak kosong lainnya, Claude Code menampilkan dialog. Sebelum v2.1.218, semua kecuali `DO_NOT_TRACK` diterapkan tanpa persetujuan pada nilai apa pun, dan `DO_NOT_TRACK` memicu dialog pada nilai apa pun yang tidak kosong.

Claude Code juga memutuskan apakah [`API_FORCE_IDLE_TIMEOUT`](/docs/id/env-vars) memerlukan persetujuan berdasarkan nilai yang dikirimkan: nilai yang benar hanya mengaktifkan [body idle timeout](/docs/id/network-config#streaming-idle-watchdogs), jadi Claude Code menerapkannya tanpa bertanya kepada pengguna. Untuk nilai apa pun yang tidak kosong lainnya, Claude Code menampilkan dialog. Sebelum v2.1.248, nilai apa pun yang tidak kosong memicu dialog.

Apakah [`ANTHROPIC_CUSTOM_HEADERS`](/docs/id/env-vars#variables) memerlukan persetujuan juga tergantung pada nilai yang dikirimkan. Header yang hanya menandai permintaan, seperti `Accept-Language`, diterapkan tanpa dialog. Baris yang menamai kredensial, pemilih org atau tenant, penggantian routing atau host, atau header perilaku API, seperti `Authorization`, `X-Api-Key`, `Host`, `anthropic-beta`, atau header `X-Amzn-Bedrock-*`, memerlukan persetujuan. Demikian juga baris yang namanya bukan token header HTTP yang valid, atau yang nilainya berisi karakter yang header HTTP tidak dapat bawa. Pemeriksaan mencocokkan kata di dalam nama header, jadi `X-Client-Version`, yang berisi `client` dan `version`, memerlukan persetujuan juga. Sebelum v2.1.251, nilai `ANTHROPIC_CUSTOM_HEADERS` apa pun diterapkan tanpa itu.

Nilai falsy seperti `0` atau `false` untuk [`ENABLE_BETA_TRACING_DETAILED`](/docs/id/env-vars#variables) atau [`OTEL_LOG_RAW_API_BODIES`](/docs/id/env-vars#variables) diterapkan tanpa dialog, karena hanya mematikan tracing terperinci atau penangkapan badan API mentah. Nilai apa pun yang tidak kosong lainnya untuk variabel apa pun memerlukan persetujuan.

<h2 id="platform-availability">
  Ketersediaan platform
</h2>

Pengaturan yang dikelola server memerlukan koneksi langsung ke `api.anthropic.com`. Pengiriman juga memerlukan sesi untuk melakukan autentikasi dengan salah satu kredensial berikut:

* Login OAuth Organisasi atau Perusahaan
* Token OAuth yang disediakan melalui `CLAUDE_CODE_OAUTH_TOKEN`
* Kunci API yang dikonfigurasi secara langsung
* Profil `user_oauth` [Anthropic](/docs/id/authentication#anthropic-profiles-and-federation-credentials), kecuali profil menetapkan `base_url` selain API Anthropic. Memerlukan Claude Code v2.1.257 atau lebih baru.

Kunci yang dikembalikan oleh skrip [`apiKeyHelper`](/docs/id/settings-reference#apikeyhelper) maupun kredensial [Workload Identity Federation](https://platform.claude.com/docs/en/manage-claude/workload-identity-federation) tidak memicu pengambilan pengaturan.

Dalam sesi [Cowork](https://claude.com/docs/cowork/overview) di aplikasi Claude Desktop, Claude Code tidak mengambil pengaturan yang dikelola server dari konsol admin claude.ai, bahkan ketika pengguna masuk dengan akun Tim atau Perusahaan. [Di mana dan kapan kebijakan berlaku](/docs/id/managed-settings#where-and-when-a-policy-applies) mencakup kebijakan mana yang mencapai sesi Cowork di mesin pengguna dan sesi Cowork jarak jauh. claude.ai masih menerapkan daftar [`strictKnownMarketplaces`](/docs/id/settings-reference#strictknownmarketplaces) dan [`blockedMarketplaces`](/docs/id/settings-reference#blockedmarketplaces) Anda sendiri ketika pengguna Cowork menambahkan marketplace dari repositori git di claude.ai atau dari **Customize** di tab Cowork. [Bagaimana pembatasan bekerja](/docs/id/plugins/org#restrict-what-users-can-install) menjelaskan pemeriksaan tersebut.

Jika Anda mengekspor variabel penyedia `CLAUDE_CODE_USE_*` atau `ANTHROPIC_BASE_URL` non-default di shell Anda, Claude Code melewati pengambilan pengaturan untuk sesi Anda. [`claude doctor` dan `/status` melaporkan pengambilan yang dilewati dan penyebabnya](#verify-settings-delivery).

Anda tidak dapat menghapus ekspor dengan blok `env` yang dikelola server, karena blok tiba melalui pengambilan yang dicegah oleh ekspor. Blok `env` [pengaturan yang dikelola endpoint](/docs/id/managed-settings#delivery-mechanisms) juga tidak mengembalikan pengambilan: Claude Code memeriksa kelayakan sebelum menerapkan blok `env` yang dikelola, jadi nilai yang dikelola endpoint mengubah pemilihan penyedia sesi tetapi pengambilan tetap dilewati.

Untuk mengembalikan pengiriman yang dikelola server, hapus ekspor dari shell Anda, atau atur variabel ke `""` di blok `env` pengaturan pengguna Anda, yang diterapkan sebelum pemeriksaan kelayakan. Untuk memberlakukan kebijakan tanpa mengandalkan pengguna untuk mengubah shell mereka, berikan pengaturan melalui saluran yang dikelola endpoint sebagai gantinya.

Untuk penyebaran Amazon Bedrock, Platform Agen Google Cloud, Microsoft Foundry, dan [Claude Platform on AWS](/docs/id/claude-platform-on-aws), gateway [aplikasi Claude](/docs/id/claude-apps-gateway) yang di-host sendiri menyediakan pengiriman pengaturan yang dikelola jarak jauh yang setara: klien yang masuk ke gateway mengambil pengaturan yang dikelola dari gateway alih-alih `api.anthropic.com`. Semantik kegagalan berbeda saat startup: klien gateway yang tidak dapat menjangkau gateway keluar dengan kesalahan alih-alih kembali ke pengaturan yang di-cache, sementara penyegaran latar belakang per jam adalah fail-open di kedua saluran.

<h2 id="audit-logging">
  Audit logging
</h2>

Acara log audit untuk perubahan pengaturan tersedia melalui API kepatuhan atau ekspor log audit. Hubungi tim akun Anthropic Anda untuk akses.

Acara audit mencakup jenis tindakan yang dilakukan, akun dan perangkat yang melakukan tindakan, dan referensi ke nilai sebelumnya dan baru.

<h2 id="security-considerations">
  Pertimbangan keamanan
</h2>

Pengaturan yang dikelola server menyediakan penegakan kebijakan terpusat, tetapi mereka beroperasi sebagai kontrol sisi klien, bukan batas keamanan. Pada perangkat yang tidak dikelola, pengguna tidak perlu akses admin atau sudo untuk melewatinya.

| Skenario                                                                      | Perilaku                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| :---------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Pengguna mengedit file pengaturan yang di-cache                               | File yang dirusak berlaku pada startup, kecuali untuk [nilai yang Claude Code tahan](#fetch-and-caching-behavior) sampai server mengonfirmasi payload. Pengambilan server berikutnya memulihkan pengaturan yang benar, kecuali untuk [kunci yang hanya berlaku pada peluncuran berikutnya](#fetch-and-caching-behavior), seperti `model` atau variabel yang ditambahkan ke blok `env`, yang tetap berlaku sampai reluncur                                                                                                                                                                                                                                                                                                                |
| Pengguna menghapus file pengaturan yang di-cache                              | [Perilaku peluncuran pertama](#fetch-and-caching-behavior) terjadi                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| Pengguna menjalankan biner Claude Code yang dimodifikasi                      | Pengguna yang dapat menjalankan klien yang dimodifikasi dapat melewati kontrol sisi klien apa pun                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| Pengguna menjalankan versi Claude Code yang lebih lama                        | Versi yang mendahului pengaturan yang dikelola server tidak mengambil atau menerapkannya                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| API tidak tersedia                                                            | Pengaturan yang di-cache berlaku jika tersedia, kecuali untuk [nilai yang Claude Code tahan](#fetch-and-caching-behavior) sampai pengambilan berhasil. Tanpa cache, Claude Code tidak memberlakukan pengaturan yang dikelola server sampai pengambilan yang berhasil berikutnya dan masih menerapkan [pengaturan yang dikelola endpoint](/docs/id/managed-settings#delivery-mechanisms) apa pun pada perangkat. Dengan `forceRemoteSettingsRefresh: true`, CLI keluar sebagai gantinya melanjutkan, kecuali untuk [subperintah `claude auth`](#enforce-fail-closed-startup). Klien yang masuk melalui [gateway aplikasi Claude](#platform-availability) keluar pada startup tanpa pengaturan itu, dengan pengecualian `claude auth` yang sama |
| Pengguna melakukan autentikasi dengan organisasi yang berbeda                 | Pengaturan tidak dikirimkan untuk akun di luar organisasi yang dikelola                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| Pengguna mengonfigurasi [penyedia model pihak ketiga](#platform-availability) | Pengaturan yang dikelola server dilewati. Ini termasuk pengaturan `CLAUDE_CODE_USE_BEDROCK`, `CLAUDE_CODE_USE_MANTLE`, `CLAUDE_CODE_USE_VERTEX`, `CLAUDE_CODE_USE_FOUNDRY`, `CLAUDE_CODE_USE_ANTHROPIC_AWS`, atau `ANTHROPIC_BASE_URL` non-default                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| Lalu lintas jaringan dicegat atau dialihkan                                   | Validasi TLS yang dinonaktifkan atau lalu lintas yang dicegat dapat mengubah pengaturan yang diterima klien                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |

Untuk mencatat pengeditan file pengaturan lokal, termasuk `managed-settings.json`, gunakan [hook `ConfigChange`](/docs/id/hooks#configchange). Claude Code tidak menjalankannya ketika pengaturan yang dikelola server tiba atau menyegarkan, atau ketika profil MDM atau kebijakan registri berubah, dan hook tidak dapat memblokir perubahan `policy_settings`.

Untuk membatasi organisasi mana yang dapat diakses pengguna Anda dengan kredensial yang disediakan klien, lihat [Enforce network-level access control with Tenant Restrictions](https://support.claude.com/en/articles/13198485-enforce-network-level-access-control-with-tenant-restrictions) di Claude Help Center. Untuk jaminan penegakan yang lebih kuat, gunakan [pengaturan yang dikelola endpoint](/docs/id/managed-settings#delivery-mechanisms) pada perangkat yang terdaftar dalam solusi MDM.

<h2 id="see-also">
  Lihat juga
</h2>

Halaman terkait untuk mengelola konfigurasi Claude Code:

* [Semua pengaturan](/docs/id/settings-reference): setiap kunci pengaturan
* [Pengaturan yang dikelola endpoint](/docs/id/managed-settings#delivery-mechanisms): pengaturan terkelola yang digunakan ke perangkat oleh IT
* [Authentication](/docs/id/authentication): atur akses pengguna ke Claude Code
* [Security](/docs/id/security): perlindungan keamanan dan praktik terbaik
