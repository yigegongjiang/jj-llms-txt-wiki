> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Konfigurasi pengaturan yang dikelola server

> Konfigurasi Claude Code secara terpusat untuk organisasi Anda melalui pengaturan yang dikirimkan server, tanpa memerlukan infrastruktur manajemen perangkat.

Pengaturan yang dikelola server memungkinkan Pemilik organisasi untuk mengonfigurasi Claude Code secara terpusat dari [**Admin Settings > Claude Code > Managed settings**](https://claude.ai/admin-settings/claude-code) di konsol claude.ai. Klien Claude Code secara otomatis mengambil pengaturan ini ketika pengguna melakukan autentikasi dengan login OAuth organisasi atau kunci API yang dikonfigurasi secara langsung, di platform tempat pengiriman yang dikelola server didukung. Lihat [Ketersediaan platform](#platform-availability).

Pendekatan ini dirancang untuk organisasi yang tidak memiliki infrastruktur manajemen perangkat, atau yang perlu mengelola pengaturan untuk pengguna pada perangkat yang tidak dikelola.

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

Claude Code mendukung dua pendekatan untuk konfigurasi terpusat. Pengaturan yang dikelola server mengirimkan konfigurasi dari server Anthropic. [Pengaturan yang dikelola endpoint](/docs/id/settings#settings-files) digunakan langsung ke perangkat melalui kebijakan OS asli (preferensi terkelola macOS, registri Windows) atau file pengaturan terkelola.

| Pendekatan                                                           | Terbaik untuk                                                          | Model keamanan                                                                                                       |
| :------------------------------------------------------------------- | :--------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------- |
| **Pengaturan yang dikelola server**                                  | Organisasi tanpa MDM, atau pengguna pada perangkat yang tidak dikelola | Pengaturan dikirimkan dari server Anthropic pada waktu autentikasi                                                   |
| **[Pengaturan yang dikelola endpoint](/docs/id/settings#settings-files)** | Organisasi dengan MDM atau manajemen endpoint                          | Pengaturan digunakan ke perangkat melalui profil konfigurasi MDM, kebijakan registri, atau file pengaturan terkelola |

Jika perangkat Anda terdaftar dalam solusi MDM atau manajemen endpoint, pengaturan yang dikelola endpoint memberikan jaminan keamanan yang lebih kuat karena file pengaturan dapat dilindungi dari modifikasi pengguna di tingkat OS. Pengaturan yang dikelola endpoint tidak mencapai [sesi cloud](/docs/id/model-config#surface-coverage), jadi organisasi yang menggunakan Claude Code di web harus mengonfigurasi pengaturan yang dikelola server juga.

<h2 id="configure-server-managed-settings">
  Konfigurasi pengaturan yang dikelola server
</h2>

<Steps>
  <Step title="Buka konsol admin">
    Di konsol claude.ai, buka [**Admin Settings > Claude Code > Managed settings**](https://claude.ai/admin-settings/claude-code).

    Jika tautan mengarahkan ulang Anda ke halaman Admin Settings yang berbeda alih-alih halaman Claude Code, akun Anda tidak memiliki peran yang diperlukan. Peran Admin dan peran non-Owner lainnya tidak dapat melihat atau mengedit pengaturan terkelola, jadi minta Owner atau Primary Owner di organisasi Anda untuk membuat perubahan. Lihat [Kontrol akses](#access-control).
  </Step>

  <Step title="Tentukan pengaturan Anda">
    Tambahkan konfigurasi Anda sebagai JSON. Semua [pengaturan yang tersedia di `settings.json`](/docs/id/settings#available-settings) didukung kecuali yang dibatasi untuk pengiriman kebijakan tingkat OS; lihat [Batasan saat ini](#current-limitations) untuk daftar singkat itu. Ini mencakup [hooks](/docs/id/hooks), [variabel lingkungan](/docs/id/env-vars), dan [pengaturan yang hanya dikelola](/docs/id/permissions#managed-only-settings) seperti `allowManagedPermissionRulesOnly`.

    Contoh ini memberlakukan daftar penolakan izin, mencegah pengguna dari melewati izin, dan membatasi aturan izin hanya pada yang ditentukan dalam pengaturan terkelola:

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

    Untuk mengonfigurasi pengklasifikasi [mode otomatis](/docs/id/permission-modes#eliminate-prompts-with-auto-mode) sehingga mengetahui repositori, bucket, dan domain mana yang dipercaya organisasi Anda:

    ```json theme={null}
    {
      "autoMode": {
        "environment": [
          "Source control: github.example.com/acme-corp and all repos under it",
          "Trusted cloud buckets: s3://acme-build-artifacts, gs://acme-ml-datasets",
          "Trusted internal domains: *.corp.example.com"
        ]
      }
    }
    ```

    Karena hooks menjalankan perintah shell, pengguna melihat [dialog persetujuan keamanan](#security-approval-dialogs) sebelum diterapkan. Lihat [Konfigurasi mode otomatis](/docs/id/auto-mode-config) untuk cara entri `autoMode` mempengaruhi apa yang diblokir pengklasifikasi dan peringatan penting tentang bidang `environment`, `allow`, `soft_deny`, dan `hard_deny`.
  </Step>

  <Step title="Simpan dan terapkan">
    Simpan perubahan Anda. Klien Claude Code menerima pengaturan yang diperbarui pada startup berikutnya atau siklus polling per jam.
  </Step>
</Steps>

<h3 id="verify-settings-delivery">
  Verifikasi pengiriman pengaturan
</h3>

Untuk mengonfirmasi bahwa pengaturan sedang diterapkan, minta pengguna untuk memulai ulang Claude Code. Jika konfigurasi mencakup pengaturan yang memicu [dialog persetujuan keamanan](#security-approval-dialogs), pengguna melihat prompt yang menjelaskan pengaturan yang dikelola pada startup. Anda juga dapat memverifikasi bahwa aturan izin terkelola aktif dengan meminta pengguna menjalankan `/permissions` untuk melihat aturan izin efektif mereka.

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

Sebagian besar [kunci pengaturan](/docs/id/settings#available-settings) bekerja dalam cakupan apa pun. Segelintir kunci hanya dibaca dari pengaturan terkelola dan tidak berpengaruh ketika ditempatkan dalam file pengaturan pengguna atau proyek. Lihat [pengaturan yang hanya dikelola](/docs/id/permissions#managed-only-settings) untuk daftar lengkap. Pengaturan apa pun yang tidak ada dalam daftar itu masih dapat ditempatkan dalam pengaturan terkelola dan memiliki prioritas tertinggi.

<h3 id="current-limitations">
  Batasan saat ini
</h3>

Pengaturan yang dikelola server memiliki batasan berikut:

* Pengaturan berlaku secara seragam untuk semua pengguna dalam organisasi. Konfigurasi per-grup belum didukung.
* File [`managed-mcp.json`](/docs/id/managed-mcp) tidak dapat didistribusikan melalui pengaturan yang dikelola server. Berikan kunci kebijakan `allowedMcpServers` dan `deniedMcpServers` di sana sebagai gantinya.
* Pengaturan yang dibatasi untuk sumber kebijakan tingkat OS, seperti `policyHelper` dan `wslInheritsWindowsSettings`, tidak dihormati. Terapkan melalui MDM atau file `managed-settings.json` sistem sebagai gantinya.

<h2 id="settings-delivery">
  Pengiriman pengaturan
</h2>

<h3 id="settings-precedence">
  Prioritas pengaturan
</h3>

Pengaturan yang dikelola server dan [pengaturan yang dikelola endpoint](/docs/id/settings#settings-files) keduanya menempati tingkat tertinggi dalam [hierarki pengaturan](/docs/id/settings#settings-precedence) Claude Code. Tidak ada tingkat pengaturan lain yang dapat menggantinya, termasuk argumen baris perintah.

Dalam tingkat terkelola, [`policyHelper`](/docs/id/settings#compute-managed-settings-with-a-policy-helper) yang dikonfigurasi mendahului setiap sumber terkelola lainnya, termasuk pengaturan yang dikelola server: outputnya menjadi satu-satunya konfigurasi terkelola untuk jalankan.

Jika tidak, Claude Code menggunakan sumber pertama yang mengirimkan konfigurasi non-kosong. Pengaturan yang dikelola server diperiksa terlebih dahulu, kemudian pengaturan yang dikelola endpoint. Sumber tidak digabungkan: jika pengaturan yang dikelola server mengirimkan kunci apa pun, pengaturan yang dikelola endpoint lainnya diabaikan sepenuhnya. Jika pengaturan yang dikelola server tidak mengirimkan apa pun, pengaturan yang dikelola endpoint berlaku.

Satu pengecualian berlaku: serangkaian kecil [kunci kunci lintas sumber](/docs/id/settings#settings-precedence), seperti kunci daftar pasir sandbox, dihormati ketika sumber terkelola yang dikendalikan admin apa pun menetapkannya; tingkat registri HKCU yang dapat ditulis pengguna dikecualikan.

Jika Anda menghapus konfigurasi pengaturan yang dikelola server di konsol admin dengan tujuan untuk kembali ke plist yang dikelola endpoint atau kebijakan registri, perhatikan bahwa [pengaturan yang di-cache](#fetch-and-caching-behavior) bertahan pada mesin klien hingga pengambilan berikutnya yang berhasil. Jalankan `/status` untuk melihat sumber terkelola mana yang aktif.

<h3 id="fetch-and-caching-behavior">
  Perilaku pengambilan dan caching
</h3>

Claude Code mengambil pengaturan dari server Anthropic pada startup dan melakukan polling untuk pembaruan setiap jam selama sesi aktif.

**Peluncuran pertama tanpa pengaturan yang di-cache:**

* Claude Code mengambil pengaturan secara asinkron
* Jika pengambilan gagal, Claude Code melanjutkan tanpa pengaturan terkelola
* Ada jendela singkat sebelum pengaturan dimuat di mana pembatasan belum diterapkan

**Peluncuran berikutnya dengan pengaturan yang di-cache:**

* Pengaturan yang di-cache berlaku segera pada startup, kecuali untuk variabel lingkungan transport, routing, dan autentikasi yang dijelaskan di bawah
* Claude Code mengambil pengaturan segar di latar belakang
* Pengaturan yang di-cache bertahan melalui kegagalan jaringan. Variabel lingkungan yang ditahan tetap ditahan hingga pengambilan berhasil

Mulai dari v2.1.198, Claude Code menahan tiga kategori variabel dalam blok `env` yang di-cache hingga server mengonfirmasi payload untuk sesi. Ini mencegah nilai proxy yang di-cache, otoritas sertifikat, endpoint, atau kredensial dari mengarahkan ulang, mencegat, atau melakukan autentikasi ulang pengambilan pengaturan yang mengonfirmasi payload. Pengerasan hanya berlaku pada cache pengaturan yang diambil server: [pengaturan yang dikelola endpoint](/docs/id/settings#settings-files) yang digunakan melalui MDM atau `managed-settings.json` tidak terpengaruh. Kategori yang ditahan adalah:

* Konfigurasi proxy dan TLS, seperti `HTTPS_PROXY`, `NODE_EXTRA_CA_CERTS`, dan variabel sertifikat klien mTLS `CLAUDE_CODE_CLIENT_CERT` dan `CLAUDE_CODE_CLIENT_KEY`
* Routing API dan pemilihan penyedia, termasuk `ANTHROPIC_BASE_URL`, variabel pemilihan penyedia seperti `CLAUDE_CODE_USE_BEDROCK` dan `CLAUDE_CODE_USE_VERTEX`, dan URL endpoint penyedia seperti `ANTHROPIC_BEDROCK_BASE_URL`
* Kredensial autentikasi, seperti `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN`, dan `CLAUDE_CODE_OAUTH_TOKEN`

Setiap kunci lain dalam blok `env` yang di-cache, seperti telemetri dan konfigurasi OpenTelemetry, berlaku pada startup seperti sebelumnya. Setelah pengambilan berhasil, variabel yang ditahan berlaku untuk sisa sesi.

Jika organisasi Anda memerlukan proxy untuk menjangkau `api.anthropic.com`, atur di lingkungan shell atau di [pengaturan pengguna](/docs/id/settings#settings-files) daripada hanya di blok `env` terkelola. Peluncuran pertama tidak memiliki cache, jadi sumber-sumber tersebut sudah diperlukan untuk pengambilan awal.

Claude Code menerapkan pembaruan pengaturan secara otomatis tanpa restart, kecuali untuk pengaturan lanjutan seperti konfigurasi OpenTelemetry, yang memerlukan restart penuh untuk berlaku.

<h3 id="invalid-entries-in-delivered-settings">
  Entri tidak valid dalam pengaturan yang dikirimkan
</h3>

Payload yang dikirimkan diurai dengan toleran menggunakan aturan yang sama seperti sumber terkelola lainnya. Ketika payload berisi entri yang gagal validasi skema, Claude Code menghapus entri tersebut, menampilkan kesalahan validasi, dan menerapkan setiap pengaturan yang valid yang tersisa. Lihat [Entri tidak valid dalam pengaturan terkelola](/docs/id/settings#invalid-entries-in-managed-settings) untuk perilaku tingkat bidang, termasuk cara penanganan bidang penegakan keamanan. Memerlukan Claude Code v2.1.169 atau lebih baru.

Pengiriman yang dikelola server menambahkan perilaku ini:

* Cache di `~/.claude/remote-settings.json` menyimpan payload yang diselamatkan dengan entri tidak valid dihapus. Payload tidak valid mentah tidak pernah disimpan.
* Ketika tidak ada bidang dalam payload yang dapat diselamatkan, Claude Code menyimpan pengaturan cache yang terakhir diterima dan mencatat kesalahan fatal.
* [Dialog persetujuan keamanan](#security-approval-dialogs) mengevaluasi payload yang diselamatkan, sehingga entri tidak valid yang dilucuti tidak pernah disajikan untuk persetujuan dan tidak pernah dieksekusi.

Untuk men-debug masalah pengiriman, jalankan `claude --debug-file <path>` dan cari log untuk `Remote settings`. Validasi perubahan payload dengan `claude doctor` pada mesin uji sebelum meluncurkannya ke organisasi.

<h3 id="enforce-fail-closed-startup">
  Paksakan startup yang tertutup gagal
</h3>

Secara default, jika pengambilan pengaturan jarak jauh gagal pada startup, CLI melanjutkan tanpa pengaturan terkelola. Untuk lingkungan di mana jendela yang tidak diterapkan singkat ini tidak dapat diterima, atur `forceRemoteSettingsRefresh: true` dalam pengaturan terkelola Anda.

Ketika pengaturan ini aktif, CLI memblokir pada startup hingga pengaturan jarak jauh diambil segar. Jika pengambilan gagal, CLI keluar daripada melanjutkan tanpa kebijakan. Pengaturan ini memperpanjang dirinya sendiri: setelah dikirimkan dari server, pengaturan ini juga di-cache secara lokal sehingga startup berikutnya memberlakukan perilaku yang sama bahkan sebelum pengambilan pertama yang berhasil dari sesi baru.

Untuk mengaktifkan ini, tambahkan kunci ke konfigurasi pengaturan terkelola Anda:

```json theme={null}
{
  "forceRemoteSettingsRefresh": true
}
```

Anda juga dapat mengatur kunci ini dalam profil MDM yang [dikelola endpoint](/docs/id/settings#settings-files) atau file `managed-settings.json` sistem untuk memberlakukan perilaku tertutup gagal pada peluncuran pertama, sebelum payload server apa pun telah dikirimkan. Mulai dari v2.1.191, flag ini adalah pengecualian terhadap [aturan prioritas](#settings-precedence) di atas: flag ini dihormati ketika diatur dalam sumber terkelola apa pun bahkan jika payload yang di-cache yang dikelola server juga ada, sehingga nilai yang dikirimkan MDM tidak diabaikan ketika pengaturan yang dikelola server ada.

Pengambilan pengaturan juga mengirimkan header `Cache-Control: no-cache` sehingga proxy HTTP perantara tidak melayani respons yang sudah usang.

Sebelum mengaktifkan pengaturan ini, pastikan kebijakan jaringan Anda memungkinkan konektivitas ke `api.anthropic.com`. Jika endpoint tersebut tidak dapat dijangkau, CLI keluar pada startup dan pengguna tidak dapat memulai Claude Code.

Mulai dari v2.1.139, subperintah `claude auth` seperti `claude auth login` dikecualikan dari pemeriksaan ini, sehingga pengguna dapat melakukan autentikasi ulang ketika kredensial yang kedaluwarsa adalah alasan pengambilan pengaturan gagal.

<h3 id="security-approval-dialogs">
  Dialog persetujuan keamanan
</h3>

Pengaturan tertentu yang dapat menimbulkan risiko keamanan memerlukan persetujuan pengguna eksplisit sebelum Claude Code menerapkannya:

* **Pengaturan perintah shell**: pengaturan yang menjalankan perintah shell
* **Variabel lingkungan kustom**: variabel yang tidak ada dalam daftar aman yang diketahui
* **Konfigurasi hook**: definisi hook apa pun
* **Konten CLAUDE.md yang dikelola**: nilai `claudeMd` yang dikirimkan melalui pengaturan terkelola

Ketika pengaturan ini ada, pengguna melihat dialog keamanan yang menjelaskan apa yang sedang dikonfigurasi. Pengguna harus menyetujui untuk melanjutkan. Jika pengguna menolak pengaturan, Claude Code keluar.

<Note>
  Jalankan non-interaktif, seperti `claude -p` atau sesi Agent SDK, tidak dapat menampilkan dialog. Ketika pengaturan yang dikirimkan memerlukan persetujuan, Claude Code menerapkannya hanya untuk jalankan itu: tidak merekamnya sebagai disetujui atau menulisnya ke [cache lokal](#fetch-and-caching-behavior), dan sesi interaktif berikutnya menampilkan dialog. Hingga pengguna menyetujui dalam sesi interaktif, setiap jalankan non-interaktif mengambil pengaturan lagi pada startup. Sebelum v2.1.207, jalankan non-interaktif menyimpan pengaturan sebagai disetujui, sehingga sesi interaktif kemudian tidak pernah menampilkan dialog untuk mereka.
</Note>

<h2 id="platform-availability">
  Ketersediaan platform
</h2>

Pengaturan yang dikelola server memerlukan koneksi langsung ke `api.anthropic.com`, dan pengiriman memerlukan sesi untuk melakukan autentikasi dengan login OAuth organisasi atau kunci API yang dikonfigurasi secara langsung. Kunci yang dikembalikan oleh skrip [`apiKeyHelper`](/docs/id/settings#available-settings) tidak memicu pengambilan pengaturan.

Pengaturan yang dikelola server tidak tersedia saat menggunakan penyedia model pihak ketiga:

* Amazon Bedrock
* Platform Agen Google Cloud
* Microsoft Foundry
* [Claude Platform on AWS](/docs/id/claude-platform-on-aws)
* Endpoint API kustom melalui `ANTHROPIC_BASE_URL` atau gateway [LLM pihak ketiga](/docs/id/llm-gateway)

Jika Anda mengekspor variabel penyedia `CLAUDE_CODE_USE_*` atau `ANTHROPIC_BASE_URL` non-default di shell Anda, Claude Code melewati pengambilan pengaturan untuk sesi Anda. Anda tidak dapat menghapus ekspor dengan blok `env` yang dikelola server, karena blok tiba melalui pengambilan yang dicegah oleh ekspor. Blok `env` pengaturan yang dikelola endpoint juga tidak mengembalikan pengambilan: Claude Code memeriksa kelayakan sebelum menerapkan blok `env` yang dikelola, jadi penggantian mengubah pemilihan penyedia sesi tetapi pengambilan tetap dilewati.

Untuk mengembalikan pengiriman yang dikelola server, hapus ekspor dari shell Anda, atau atur variabel ke `""` di blok `env` pengaturan pengguna Anda, yang diterapkan sebelum pemeriksaan kelayakan. Untuk memberlakukan kebijakan tanpa mengandalkan pengguna untuk mengubah shell mereka, berikan pengaturan melalui saluran yang dikelola endpoint sebagai gantinya.

Untuk penyebaran Amazon Bedrock, Platform Agen Google Cloud, dan Microsoft Foundry, gateway [aplikasi Claude](/docs/id/claude-apps-gateway) yang di-host sendiri menyediakan pengiriman pengaturan yang dikelola jarak jauh yang setara: klien yang masuk ke gateway mengambil pengaturan yang dikelola dari gateway alih-alih `api.anthropic.com`. Semantik kegagalan berbeda saat startup: klien gateway yang tidak dapat menjangkau gateway keluar dengan kesalahan alih-alih kembali ke pengaturan yang di-cache, sementara penyegaran latar belakang per jam adalah fail-open di kedua saluran.

<h2 id="audit-logging">
  Audit logging
</h2>

Acara log audit untuk perubahan pengaturan tersedia melalui API kepatuhan atau ekspor log audit. Hubungi tim akun Anthropic Anda untuk akses.

Acara audit mencakup jenis tindakan yang dilakukan, akun dan perangkat yang melakukan tindakan, dan referensi ke nilai sebelumnya dan baru.

<h2 id="security-considerations">
  Pertimbangan keamanan
</h2>

Pengaturan yang dikelola server menyediakan penegakan kebijakan terpusat, tetapi mereka beroperasi sebagai kontrol sisi klien, bukan batas keamanan. Pada perangkat yang tidak dikelola, pengguna tidak perlu akses admin atau sudo untuk melewatinya.

| Skenario                                                                      | Perilaku                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| :---------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Pengguna mengedit file pengaturan yang di-cache                               | File yang dirusak berlaku pada startup, tetapi pengaturan yang benar dipulihkan pada pengambilan server berikutnya. Mulai dari v2.1.198, variabel lingkungan transport, API-routing, dan autentikasi dalam blok `env` adalah [ditahan sampai server mengonfirmasi payload](#fetch-and-caching-behavior)                                                                                                                                                                                                                       |
| Pengguna menghapus file pengaturan yang di-cache                              | Perilaku peluncuran pertama terjadi: pengaturan mengambil secara asinkron dengan jendela yang tidak diterapkan singkat                                                                                                                                                                                                                                                                                                                                                                                                        |
| Pengguna menjalankan biner Claude Code yang dimodifikasi                      | Pengguna yang dapat menjalankan klien yang dimodifikasi dapat melewati kontrol sisi klien apa pun                                                                                                                                                                                                                                                                                                                                                                                                                             |
| Pengguna menjalankan versi Claude Code yang lebih lama                        | Versi yang mendahului pengaturan yang dikelola server tidak mengambil atau menerapkannya                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| API tidak tersedia                                                            | Pengaturan yang di-cache berlaku jika tersedia, jika tidak pengaturan terkelola tidak diterapkan sampai pengambilan yang berhasil berikutnya. Mulai dari v2.1.198, variabel lingkungan transport, API-routing, dan autentikasi dalam blok `env` yang di-cache adalah [ditahan pada kegagalan pengambilan](#fetch-and-caching-behavior); sisa cache masih berlaku. Dengan `forceRemoteSettingsRefresh: true`, CLI keluar sebagai gantinya melanjutkan, kecuali untuk [subperintah `claude auth`](#enforce-fail-closed-startup) |
| Pengguna melakukan autentikasi dengan organisasi yang berbeda                 | Pengaturan tidak dikirimkan untuk akun di luar organisasi yang dikelola                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| Pengguna mengonfigurasi [penyedia model pihak ketiga](#platform-availability) | Pengaturan yang dikelola server dilewati. Ini termasuk pengaturan `CLAUDE_CODE_USE_BEDROCK`, `CLAUDE_CODE_USE_MANTLE`, `CLAUDE_CODE_USE_VERTEX`, `CLAUDE_CODE_USE_FOUNDRY`, `CLAUDE_CODE_USE_ANTHROPIC_AWS`, atau `ANTHROPIC_BASE_URL` non-default                                                                                                                                                                                                                                                                            |
| Lalu lintas jaringan dicegat atau dialihkan                                   | Validasi TLS yang dinonaktifkan atau lalu lintas yang dicegat dapat mengubah pengaturan yang diterima klien                                                                                                                                                                                                                                                                                                                                                                                                                   |

Untuk mendeteksi perubahan konfigurasi runtime, gunakan [hook `ConfigChange`](/docs/id/hooks#configchange) untuk mencatat modifikasi atau memblokir perubahan yang tidak sah sebelum berlaku.

Untuk membatasi organisasi mana yang dapat diakses pengguna Anda dengan kredensial yang disediakan klien, lihat [Enforce network-level access control with Tenant Restrictions](https://support.claude.com/en/articles/13198485-enforce-network-level-access-control-with-tenant-restrictions) di Claude Help Center. Untuk jaminan penegakan yang lebih kuat, gunakan [pengaturan yang dikelola endpoint](/docs/id/settings#settings-files) pada perangkat yang terdaftar dalam solusi MDM.

<h2 id="see-also">
  Lihat juga
</h2>

Halaman terkait untuk mengelola konfigurasi Claude Code:

* [Settings](/docs/id/settings): referensi konfigurasi lengkap termasuk semua pengaturan yang tersedia
* [Pengaturan yang dikelola endpoint](/docs/id/settings#settings-files): pengaturan terkelola yang digunakan ke perangkat oleh IT
* [Authentication](/docs/id/authentication): atur akses pengguna ke Claude Code
* [Security](/docs/id/security): perlindungan keamanan dan praktik terbaik
