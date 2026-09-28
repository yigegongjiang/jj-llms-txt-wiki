> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Terapkan pengaturan terkelola

> Terapkan pengaturan terkelola ke mesin setiap pengembang: mekanisme pengiriman per OS, bagaimana Claude Code menggabungkan sumber terkelola, dan cara memverifikasi penegakan.

Pengaturan terkelola adalah pengaturan yang organisasi Anda terapkan ke mesin setiap pengembang. Claude Code menerapkannya di atas setiap level lainnya, sehingga tidak ada nilai pengguna, proyek, lokal, atau `--settings` yang dapat menggantinya, kecuali beberapa [pengecualian sensitif keamanan](/docs/id/settings#exceptions-to-managed-settings-precedence) di mana nilai yang lebih ketat dari level yang lebih rendah masih berlaku.

Halaman ini untuk administrator yang menerapkan pengaturan terkelola atau men-debug mengapa salah satu tidak diterapkan. Untuk memutuskan apa yang akan diberlakukan, mulai dengan tabel [Tentukan apa yang akan diberlakukan](/docs/id/admin-setup#decide-what-to-enforce). Untuk jalur konsol claude.ai, lihat [Pengaturan terkelola server](/docs/id/server-managed-settings). Untuk file tempat nilai pengembang sendiri disimpan, lihat [Pengaturan](/docs/id/settings).

<h2 id="deploy-a-managed-settings-file">
  Terapkan file pengaturan terkelola
</h2>

Ini adalah cara tercepat untuk menempatkan kebijakan di setiap mesin: file `managed-settings.json`. Jika Anda belum memilih cara mengirimkan pengaturan terkelola, atau perangkat Anda berada di bawah MDM atau pengembang menjalankan sesi cloud, baca [Pilih mekanisme pengiriman](#choose-a-delivery-mechanism) terlebih dahulu.

<Steps>
  <Step title="Tulis managed-settings.json">
    Tulis `managed-settings.json` yang menyimpan kunci yang telah Anda putuskan untuk diberlakukan, dalam bentuk JSON yang sama dengan `settings.json`. Tabel [Tentukan apa yang akan diberlakukan](/docs/id/admin-setup#decide-what-to-enforce) mencantumkan kunci di balik setiap kontrol, dan setiap entri dalam [referensi pengaturan](/docs/id/settings-reference) mengatakan apakah sumber terkelola dapat menetapkannya. File ini memblokir dua pembacaan file, mematikan mode bypass, dan membuat Claude Code mengabaikan aturan izin dari file pengguna, proyek, dan lokal serta dari `--allowedTools`:

    ```json managed-settings.json theme={null}
    {
      "permissions": {
        "deny": [
          "Read(./.env)",
          "Read(./secrets/**)"
        ],
        "disableBypassPermissionsMode": "disable"
      },
      "allowManagedPermissionRulesOnly": true
    }
    ```

    Untuk contoh yang lebih lengkap yang menunjukkan bentuk kunci terkelola lainnya, termasuk metode login, model, server MCP, dan marketplace, lihat [Pengaturan terkelola organisasi](/docs/id/settings-example#an-organizations-managed-settings).
  </Step>

  <Step title="Tempatkan file di setiap mesin">
    Simpan file sebagai `managed-settings.json` di direktori sistem untuk sistem operasi, menggunakan alat apa pun yang sudah menempatkan file di armada Anda:

    * **macOS**: `/Library/Application Support/ClaudeCode/managed-settings.json`
    * **Linux dan WSL**: `/etc/claude-code/managed-settings.json`
    * **Windows**: `C:\Program Files\ClaudeCode\managed-settings.json`
  </Step>

  <Step title="Konfirmasi kebijakan diterapkan">
    Di satu mesin, jalankan `/status` di dalam Claude Code. Baris `Setting sources` menunjukkan `Enterprise managed settings (file)`. Lanjutkan ke sisa armada setelah itu; [Periksa bahwa kebijakan berlaku](#check-that-a-policy-is-in-force) mencakup apa yang harus dilihat ketika baris hilang.
  </Step>
</Steps>

<span id="managed-settings-delivery" />

<span id="delivery-mechanisms" />

<h2 id="choose-a-delivery-mechanism">
  Pilih mekanisme pengiriman
</h2>

File dalam langkah-langkah di atas adalah salah satu dari empat cara untuk mendapatkan pengaturan terkelola ke mesin. Setiap mekanisme membawa kunci kebijakan yang sama dengan file `settings.json`, sehingga [referensi pengaturan](/docs/id/settings-reference) berlaku untuk semuanya. Beberapa kunci terikat pada sumber tertentu, dan baris Scope setiap entri mengatakan yang mana:

* **Kontrol pengiriman**: [`policyHelper`](/docs/id/settings-reference#policyhelper), [`wslInheritsWindowsSettings`](/docs/id/settings-reference#wslinheritswindowssettings), dan [`managedSourcesBehavior`](/docs/id/settings-reference#managedsourcesbehavior)
* **Kunci login gateway**: [`forceLoginGatewayUrl`](/docs/id/settings-reference#forcelogingatewayurl), [`gatewayInternalNetworks`](/docs/id/settings-reference#gatewayinternalnetworks), dan nilai `"gateway"` dari [`forceLoginMethod`](/docs/id/settings-reference#forceloginmethod)

File pengaturan terkelola, profil MDM, atau konsol claude.ai menerapkan satu kebijakan untuk semua orang yang dijangkaunya. Untuk memberikan satu kelompok pengembang kebijakan yang berbeda, terapkan file atau profil yang berbeda ke kelompok itu; konsol claude.ai [belum dapat menargetkan grup](/docs/id/server-managed-settings#current-limitations), sementara [gateway aplikasi Claude](/docs/id/claude-apps-gateway) yang di-host sendiri mengirimkan pengaturan terkelola per grup IdP.

Ketika lebih dari satu mekanisme mengirimkan kebijakan ke mesin yang sama, Claude Code secara default menggunakan satu dan mengabaikan yang lain. [Bagaimana Claude Code menggabungkan sumber terkelola](#how-claude-code-combines-managed-sources) memberikan urutan dan opt-in yang menerapkan setiap sumber.

Baris MDM dan file bersama-sama disebut pengaturan terkelola endpoint, karena kebijakan disimpan di perangkat pengembang, berbeda dengan baris terkelola server, di mana Claude Code mengambilnya.

Pilih mekanisme berdasarkan cara Anda sudah mengelola perangkat, menggunakan tabel di bawah.

| Mekanisme                                                  | Cara Anda mengirimkannya                                                                                                                                                                                                 | Kapan Claude Code membacanya                                                                                                                                                                                                                                                                           | Gunakan ketika                                                                                          |
| :--------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------ |
| [Pengaturan terkelola server](/docs/id/server-managed-settings) | Di konsol admin claude.ai, atau di [gateway aplikasi Claude](/docs/id/claude-apps-gateway) yang di-host sendiri                                                                                                               | Diambil saat startup dan dipolling setiap jam; lihat [perubahan yang memerlukan persetujuan](#where-and-when-a-policy-applies)                                                                                                                                                                         | Anda ingin satu tempat untuk mengubah kebijakan untuk organisasi claude.ai tanpa menyentuh setiap mesin |
| Kebijakan MDM atau tingkat OS                              | Sebagai profil konfigurasi macOS atau nilai registry Windows `HKLM`, melalui Jamf, Intune, Group Policy, atau alat serupa; lihat [di mana setiap mekanisme menyimpan kebijakan](#where-each-mechanism-stores-the-policy) | Dibaca saat startup dan diperiksa untuk perubahan setiap 30 menit                                                                                                                                                                                                                                      | Anda sudah mengelola perangkat dengan MDM atau Group Policy                                             |
| Berbasis file                                              | Sebagai `managed-settings.json` di direktori sistem di setiap mesin; lihat [di mana setiap mekanisme menyimpan kebijakan](#where-each-mechanism-stores-the-policy)                                                       | Dibaca saat startup dan dimuat ulang ketika file berubah                                                                                                                                                                                                                                               | Mesin tanpa MDM, host Linux, atau gambar yang Anda bangun sendiri                                       |
| Registry HKCU, Windows dan WSL                             | Sebagai nilai registry Windows `HKCU`; lihat [di mana setiap mekanisme menyimpan kebijakan](#where-each-mechanism-stores-the-policy)                                                                                     | Dibaca saat startup dan diperiksa untuk perubahan setiap 30 menit; Claude Code menggunakannya hanya ketika tidak ada sumber terkelola lain yang mengirimkan kunci kebijakan dan tidak ada [pengaturan induk yang disediakan host](#let-an-embedding-host-add-policy) yang menyediakan kunci yang ketat | Anda tidak dapat menulis kunci tingkat mesin `HKLM`                                                     |

Template pemula untuk Jamf, Iru, Intune, dan Group Policy ada di [repositori contoh MDM](https://github.com/anthropics/claude-code/tree/main/examples/mdm).

Untuk server MCP terkelola, yang Anda terapkan bersama salah satu dari ini melalui `managed-mcp.json` atau sediakan melalui kunci [`managedMcpServers`](/docs/id/settings-reference#managedmcpservers), lihat [Konfigurasi MCP terkelola](/docs/id/managed-mcp).

<h3 id="where-and-when-a-policy-applies">
  Di mana dan kapan kebijakan berlaku
</h3>

Kebijakan yang diterapkan mencapai sesi pengembang sebagai berikut:

* **Permukaan**: di mesin pengembang, terminal, ekstensi VS Code dan JetBrains, tab Code aplikasi desktop, dan sesi [Agent SDK](/docs/id/agent-sdk/typescript) membaca semua sumber ini. Sesi Agent SDK memuat pengaturan terkelola bahkan ketika `settingSources` mengecualikan file pengguna, proyek, dan lokal.
* **Sesi cloud**: sesi di lingkungan yang dihosting Anthropic tidak membaca profil MDM atau file perangkat, jadi kebijakan untuk itu harus berasal dari pengaturan terkelola server. Sesi di [lingkungan yang di-host sendiri](/docs/id/self-hosted-environments) juga membaca file pengaturan terkelola di gambar runner-nya, secara default hanya ketika pengaturan terkelola server tidak mengirimkan kunci kebijakan, terlepas dari [kunci yang Claude Code baca dari setiap sumber admin](#keys-read-from-every-admin-source). [Bagaimana Claude Code menggabungkan sumber terkelola](#how-claude-code-combines-managed-sources) mencakup opt-in yang menerapkan keduanya.
* **Sesi Cowork**: [Cowork](https://claude.com/docs/cowork/overview) di aplikasi Claude Desktop menjalankan sesinya di Claude Code. Dalam sesi Cowork, Claude Code tidak pernah mengambil pengaturan terkelola server dari konsol admin claude.ai, bahkan ketika pengguna masuk dengan akun Team atau Enterprise, jadi kebijakan mana yang berlaku tergantung di mana sesi berjalan:

  * **Di mesin pengguna**: secara default, Claude Code dalam sesi Cowork membaca kebijakan MDM atau tingkat OS dan file pengaturan terkelola di perangkat itu, jadi terapkan kebijakan di sana.
  * **Di sandbox VM penuh**: ketika konfigurasi terkelola Claude Desktop Anda menetapkan [`requireCoworkFullVmSandbox`](https://claude.com/docs/third-party/claude-desktop/configuration#requirecoworkfullvmsandbox), Claude Code berjalan di dalam mesin virtual di mana kebijakan MDM perangkat dan file pengaturan terkelola tidak ada.
  * **Sesi Cowork jarak jauh**: ini berjalan di VM yang dikelola Anthropic, di mana Claude Code tidak memiliki kebijakan perangkat untuk dibaca.

  Di mana pun sesi berjalan, claude.ai menerapkan daftar [`strictKnownMarketplaces`](/docs/id/settings-reference#strictknownmarketplaces) dan [`blockedMarketplaces`](/docs/id/settings-reference#blockedmarketplaces) konsol admin itu sendiri ketika siapa pun menambahkan marketplace dari repositori git di claude.ai atau dari **Customize** di tab Cowork. [Bagaimana pembatasan bekerja](/docs/id/plugins/org#restrict-what-users-can-install) menjelaskan pemeriksaan itu. Tabel [cakupan permukaan](/docs/id/model-config#surface-coverage) membandingkan Cowork dengan permukaan lainnya.
* **Sesi yang berjalan**: sebagian besar perubahan mencapai sesi yang berjalan sesuai jadwal dalam [tabel mekanisme pengiriman](#choose-a-delivery-mechanism), tanpa restart.
  * Perubahan pada [`forceRemoteSettingsRefresh`](/docs/id/settings-reference#forceremotesettingsrefresh), [`requiredMinimumVersion`](/docs/id/settings-reference#requiredminimumversion), dan [beberapa kunci yang dapat diedit pengguna](/docs/id/settings#when-edits-take-effect) berlaku pada startup sesi berikutnya.
  * Entri [`policyHelper`](/docs/id/settings-reference#policyhelper) yang baru atau berubah berlaku pada peluncuran berikutnya. Jika pengaturan terkelola server menaungi helper pada peluncuran itu, helper berjalan segera setelah pengambilan melaporkan pengaturan tersebut dihapus.
* **Perubahan yang memerlukan persetujuan**: terlepas dari [pembaruan yang menunggu peluncuran berikutnya](/docs/id/server-managed-settings#fetch-and-caching-behavior), perubahan terkelola server ke pengaturan yang [memerlukan persetujuan](/docs/id/server-managed-settings#security-approval-dialogs), seperti hook atau variabel `env`, menunggu pengembang menerima dialog dalam sesi interaktif, dan berlaku untuk run saat ini dalam sesi yang dihosting ekstensi IDE atau Agent SDK. Perubahan terkelola server lainnya berlaku pada polling berikutnya.
* **Sesi yang tahan lama**: sesi yang dibiarkan terbuka selama berminggu-minggu masih bisa tertinggal dari rollout. [`requiredMinimumVersion`](/docs/id/settings-reference#requiredminimumversion) memblokir biner yang ketinggalan zaman dari memulai dan tidak mengakhiri sesi yang sudah berjalan.

<span id="format-the-policy-for-each-platform" />

<h3 id="where-each-mechanism-stores-the-policy">
  Di mana setiap mekanisme menyimpan kebijakan
</h3>

Kuncinya sama di mana-mana, tetapi setiap mekanisme menyimpannya di tempat dan bentuk yang berbeda:

* **Terkelola server**: server Anthropic, atau gateway Anda, menyimpan kebijakan. Claude Code menyimpan cache lokal yang diterapkannya saat startup dan [diganti pada setiap pengambilan yang berhasil](/docs/id/server-managed-settings#security-considerations).
* **Profil konfigurasi macOS**: domain preferensi terkelola `com.anthropic.claudecode`. Gunakan kunci tingkat atas yang sama dengan `managed-settings.json`, dengan pengaturan bersarang sebagai kamus dan daftar sebagai array plist.
* **Registry HKLM Windows**: JSON sebagai nilai `REG_SZ` atau `REG_EXPAND_SZ` bernama `Settings` di bawah `HKLM\SOFTWARE\Policies\ClaudeCode`.
* **Berbasis file**: `managed-settings.json`, direktori opsional `managed-settings.d/`, dan `managed-mcp.json` di direktori sistem: `/Library/Application Support/ClaudeCode/` di macOS, `/etc/claude-code/` di Linux dan WSL, dan `C:\Program Files\ClaudeCode\` di Windows. Claude Code tidak membaca jalur Windows warisan `C:\ProgramData\ClaudeCode\managed-settings.json`.
* **Registry HKCU Windows**: nilai `Settings` yang sama di bawah `HKCU\SOFTWARE\Policies\ClaudeCode`.

<h3 id="split-a-file-based-policy-across-teams">
  Pisahkan kebijakan berbasis file di seluruh tim
</h3>

Jika beberapa tim memiliki bagian dari satu kebijakan, letakkan setiap bagian di file sendiri di `managed-settings.d/`, di sebelah `managed-settings.json` di direktori sistem yang sama, alih-alih mengedit satu file bersama.

Claude Code menggabungkan `managed-settings.json` terlebih dahulu, kemudian setiap file `*.json` di direktori dalam urutan abjad. Beri nama file dengan awalan numerik untuk mengontrol urutan, seperti `10-telemetry.json` dan `20-security.json`. Claude Code mengabaikan file tersembunyi dan file yang tidak berakhir dengan `.json`.

Ketika dua file menetapkan kunci yang sama, Claude Code menggabungkannya dengan aturan ini:

* **Nilai tunggal**, seperti `"model": "opus"` atau `"cleanupPeriodDays": 7`: nilai file yang lebih baru menggantikan yang lebih lama
* **Daftar**, seperti `permissions.deny` atau `sandbox.network.allowedDomains`: dua daftar digabungkan, dengan duplikat dihapus
* **Blok bersarang**, seperti `env` atau `sandbox`: dua blok digabungkan kunci demi kunci, dan setiap kunci di dalamnya mengikuti aturan yang sama ini
* **`fallbackModel`**: rantai yang lebih baru menggantikan yang lebih lama secara keseluruhan
* **[`extraKnownMarketplaces`](/docs/id/settings-reference#extraknownmarketplaces) dan [`managedMcpServers`](/docs/id/settings-reference#managedmcpservers)**: entri yang lebih baru dengan nama yang sama menggantikan yang lebih lama secara keseluruhan
* **[`modelPicker`](/docs/id/settings-reference#modelpicker)**: lineup yang lebih baru menggantikan yang lebih lama secara keseluruhan

<span id="precedence-within-the-managed-tier" />

<span id="which-managed-source-claude-code-uses" />

<h2 id="how-claude-code-combines-managed-sources">
  Bagaimana Claude Code menggabungkan sumber terkelola
</h2>

Ketika organisasi Anda mengirimkan lebih dari satu sumber terkelola ke mesin yang sama, kunci [`managedSourcesBehavior`](/docs/id/settings-reference#managedsourcesbehavior) menentukan apa yang Claude Code lakukan dengan yang lain:

* **`"first-wins"`, pengaturan default**: Claude Code menggunakan sumber dengan peringkat tertinggi yang mengirimkan setidaknya satu kunci kebijakan dan mengabaikan sisanya daripada menggabungkannya, kecuali untuk kunci dalam [Kunci yang dibaca dari setiap sumber admin](#keys-read-from-every-admin-source). Claude Code tidak menampilkan peringatan untuk sumber yang dilewati; `/status` [menamai sumber yang digunakan dan yang dilewati](#read-the-source-in-/status).
* **`"merge"`**: Claude Code menerapkan setiap sumber admin yang mengirimkan kunci kebijakan dan menggabungkannya berdasarkan jenis kunci: pada sebagian besar kunci nilai sumber dengan peringkat lebih tinggi berlaku, daftar bersatu, dan kunci mengambil nilai paling ketat. [Compose every managed source](#compose-every-managed-source) mengatakan di mana mengatur kunci dan bagaimana setiap jenis kunci menggabung. Memerlukan Claude Code v2.1.242 atau lebih baru.

Kedua pengaturan mengurutkan sumber dengan cara yang sama. Istilah-istilah ini berulang di bagian ini:

* **Kunci kebijakan**: kunci pengaturan apa pun selain dua kunci kontrol, [`wslInheritsWindowsSettings`](/docs/id/settings-reference#wslinheritswindowssettings) dan [`managedSourcesBehavior`](/docs/id/settings-reference#managedsourcesbehavior). File pengaturan terkelola atau kebijakan MDM yang hanya berisi yang tersebut tidak dihitung, dan Claude Code melanjutkan ke sumber berikutnya.
* **Sumber admin**: salah satu dari tiga sumber pertama di bawah. Registri HKCU dapat ditulis pengguna dan bukan salah satunya.

Claude Code memeriksa sumber dalam urutan ini, prioritas tertinggi terlebih dahulu:

1. Pengaturan jarak jauh, dikirimkan dari claude.ai sebagai [pengaturan yang dikelola server](/docs/id/server-managed-settings) atau oleh [gateway aplikasi Claude](/docs/id/claude-apps-gateway). Claude Code mengambil sumber ini hanya ketika sesi mengautentikasi ke API Anthropic secara langsung dengan [login atau kunci yang memenuhi syarat](/docs/id/server-managed-settings#platform-availability), atau masuk ke gateway dengan `/login`. Pada penyedia lain, atau ketika `ANTHROPIC_BASE_URL` menunjuk ke tempat lain selain API Anthropic, dimulai dari sumber berikutnya
2. Kebijakan MDM atau tingkat OS: plist macOS atau kunci registri HKLM
3. File pengaturan terkelola, `managed-settings.d/*.json` dan `managed-settings.json` digabungkan bersama
4. Registri HKCU, di Windows, dan di WSL setelah registri HKLM atau file pengaturan terkelola Windows mengaktifkan [`wslInheritsWindowsSettings`](/docs/id/settings-reference#wslinheritswindowssettings) dan nilai HKCU juga mengaturnya. Claude Code membacanya hanya ketika tidak ada sumber di atasnya yang mengirimkan kunci kebijakan dan tidak ada [pengaturan induk yang disediakan host](#let-an-embedding-host-add-policy) yang menyediakan kunci pembatasan

Diagram ini menunjukkan peringkat, dengan contoh kunci lintas sumber yang Claude Code baca dari tiga sumber pertama di bawah pengaturan apa pun:

<img src="https://mintcdn.com/claude-code/zuWID2B-Rxm8DEC8/images/managed-source-precedence.svg?fit=max&auto=format&n=zuWID2B-Rxm8DEC8&q=85&s=53f6be49f06eff48e01422c8ae1bc2e6" className="dark:hidden" alt="Diagram menunjukkan empat sumber pengaturan terkelola yang diberi peringkat dari pengaturan jarak jauh di bagian atas melalui MDM, file pengaturan terkelola, dan registri HKCU di bagian bawah. Secara default sumber pertama dengan kunci kebijakan menyediakan kebijakan dan sisanya dilewati; dengan managedSourcesBehavior diatur ke merge, setiap sumber admin dengan kunci kebijakan berkontribusi, digabungkan berdasarkan jenis kunci, dan registri HKCU tetap keluar. Panel samping menunjukkan bahwa kunci lintas sumber seperti kunci sandbox, forceRemoteSettingsRefresh, dan per-variable env merge dibaca dari setiap sumber admin, yang mengecualikan registri HKCU." width="680" height="330" data-path="images/managed-source-precedence.svg" />

<img src="https://mintcdn.com/claude-code/zuWID2B-Rxm8DEC8/images/managed-source-precedence-dark.svg?fit=max&auto=format&n=zuWID2B-Rxm8DEC8&q=85&s=ae407a9a08a3d680e80cf1a2af845d71" className="hidden dark:block" alt="Diagram menunjukkan empat sumber pengaturan terkelola yang diberi peringkat dari pengaturan jarak jauh di bagian atas melalui MDM, file pengaturan terkelola, dan registri HKCU di bagian bawah. Secara default sumber pertama dengan kunci kebijakan menyediakan kebijakan dan sisanya dilewati; dengan managedSourcesBehavior diatur ke merge, setiap sumber admin dengan kunci kebijakan berkontribusi, digabungkan berdasarkan jenis kunci, dan registri HKCU tetap keluar. Panel samping menunjukkan bahwa kunci lintas sumber seperti kunci sandbox, forceRemoteSettingsRefresh, dan per-variable env merge dibaca dari setiap sumber admin, yang mengecualikan registri HKCU." width="680" height="330" data-path="images/managed-source-precedence-dark.svg" />

<h3 id="keys-read-from-every-admin-source">
  Kunci yang dibaca dari setiap sumber admin
</h3>

Di bawah pengaturan default `"first-wins"`, Claude Code membaca sebagian besar kunci hanya dari [sumber yang dipilih](#how-claude-code-combines-managed-sources), dan mengabaikan nilai dalam sumber dengan peringkat lebih rendah bahkan ketika sumber yang dipilih membiarkan kunci itu tidak diatur.

Beberapa kunci bekerja berbeda. Claude Code membacanya dari setiap sumber admin, jadi kebijakan MDM atau file pengaturan terkelola dengan peringkat lebih rendah masih dapat mengaturnya ketika sumber yang dipilih tidak. Claude Code mengeluarkan registri HKCU yang dapat ditulis pengguna dari pemindaian itu; ketika HKCU adalah satu-satunya sumber dan tidak ada host yang menyediakan pengaturan induk, HKCU berlaku seperti sumber yang dipilih.

Kunci lintas sumber mencakup:

* `sandbox.network.allowManagedDomainsOnly` dan `sandbox.filesystem.allowManagedReadPathsOnly`: `true` dalam sumber admin apa pun mengaktifkan kunci. Saat kunci aktif, Claude Code menyatukan daftar yang dikunci, `sandbox.network.allowedDomains` bersama dengan aturan izin `WebFetch(domain:...)`, atau `sandbox.filesystem.allowRead`, di seluruh setiap sumber admin. Tanpa kunci, Claude Code memperlakukan daftar seperti kunci lainnya, jadi di bawah `"first-wins"` daftar sumber admin yang tidak dipilih diabaikan
* `allowAllClaudeAiMcps`
* `allowManagedMcpServersOnly`: `true` dalam sumber admin apa pun mengaktifkan kunci daftar izin MCP. Saat kunci aktif, daftar `allowedMcpServers` yang terkelola berasal dari sumber admin dengan peringkat tertinggi yang mengaturnya. Daftar yang dikelola server menggantikan daftar sumber yang lebih rendah daripada menggabungkannya.

  Jika tidak ada sumber admin yang mengatur daftar, setiap server yang melewati daftar penolakan dimuat, kecuali [pengaturan induk](#let-an-embedding-host-add-policy) menyediakan daftar.

  Tanpa kunci, Claude Code membaca `allowedMcpServers` dari sumber terkelola yang diterapkan, jadi di bawah `"first-wins"` daftar sumber admin yang tidak dipilih diabaikan. Memerlukan Claude Code v2.1.273 atau lebih baru
* `deniedMcpServers` dan [`disableClaudeAiConnectors`](/docs/id/settings-reference#disableclaudeaiconnectors): entri atau `true` dalam sumber admin apa pun berlaku. Memerlukan Claude Code v2.1.273 atau lebih baru
* Jalur biner sandbox `sandbox.bwrapPath` dan `sandbox.socatPath`
* Biner sandbox `ripgrep`, [`sandbox.ripgrep`](/docs/id/settings-reference#sandbox-ripgrep)
* `sandbox.filesystem.disabled` dan `sandbox.network.strictAllowlist`
* [`useAutoModeDuringPlan`](/docs/id/settings-reference#useautomodeduringplan), [`syncClaudeAiSkills`](/docs/id/settings-reference#syncclaudeaiskills), dan [`syncClaudeAiPlugins`](/docs/id/settings-reference#syncclaudeaiplugins), di mana `false` dari sumber admin apa pun mematikan perilaku. `false` dalam pengaturan pengguna atau lokal pengembang juga mematikannya; setiap kunci hanya dapat menolak
* [`enableArtifact`](/docs/id/settings-reference#enableartifact), di mana `false` dari sumber admin apa pun mematikan [alat Artifact](/docs/id/artifacts). `false` dalam pengaturan pengguna, proyek, atau lokal pengembang juga mematikannya, dan tidak ada sumber yang mengaktifkannya kembali; lihat [nilai tingkat lebih rendah mana yang masih dihitung](/docs/id/settings#exceptions-to-managed-settings-precedence). Memerlukan Claude Code v2.1.242 atau lebih baru
* [`maxEffortLevel`](/docs/id/settings-reference#maxeffortlevel), di mana batas terendah dalam sumber admin apa pun berlaku. Jika pengembang menetapkan batas lebih rendah dalam pengaturan mereka sendiri atau dengan `--settings`, Claude Code menerapkan yang itu; tidak ada sumber yang dapat menaikkan batas. Memerlukan Claude Code v2.1.267 atau lebih baru
* Opt-out trailer komit dalam `attribution`, atau dalam `includeCoAuthoredBy` yang sudah usang, dari tingkat apa pun
* [`forceRemoteSettingsRefresh`](/docs/id/server-managed-settings)
* `env`, digabungkan per variabel di seluruh sumber admin: setiap variabel berasal dari sumber prioritas tertinggi yang mendefinisikannya, jadi sumber lebih rendah mengisi variabel yang sumber lebih tinggi biarkan tidak diatur. Beberapa variabel mengikuti aturan mereka sendiri; [Per-key exceptions across managed sources](/docs/id/server-managed-settings#per-key-exceptions-across-managed-sources) menamai masing-masing. Memerlukan Claude Code v2.1.223 atau lebih baru. Sebelum v2.1.223, Claude Code menerapkan blok `env` seluruh sumber yang dipilih saja

Kunci [login gateway](#choose-a-delivery-mechanism) mengikuti aturan terpisah. Claude Code tidak pernah membacanya dari pengaturan yang dikelola server, jadi sementara pengaturan yang dikelola server adalah sumber yang dipilih, sumber admin dengan peringkat tertinggi di mesin yang membawa kunci kebijakan masih menyediakannya. Nilai dalam sumber admin yang diberi peringkat di bawah itu, atau dalam registri HKCU, diabaikan.

Ketika sumber admin mengatur `allowManagedMcpServersOnly` atau daftar `allowedMcpServers` dan nilai itu bukan yang berlaku, `/status` dan `claude doctor` menamai sumber dan kunci itu.

<h3 id="compose-every-managed-source">
  Compose every managed source
</h3>

Untuk membuat Claude Code menerapkan setiap sumber admin yang organisasi Anda kirimkan, atur [`managedSourcesBehavior`](/docs/id/settings-reference#managedsourcesbehavior) ke `"merge"` dalam sumber dengan peringkat tertinggi yang Anda terapkan. Claude Code membaca kunci hanya dari sumber dengan peringkat tertinggi yang membawa kunci atau kunci kebijakan, jadi sumber lebih rendah tidak dapat memilih dirinya sendiri untuk bergabung dengan sumber di atasnya, dan mesin yang tidak pernah menerima pengaturan yang dikelola server memerlukan kunci dalam profil MDM juga. Registri HKCU yang dapat ditulis pengguna tidak pernah bergabung dengan sumber lain. Memerlukan Claude Code v2.1.242 atau lebih baru.

Di bawah `"merge"`, Claude Code menambahkan entri daftar sumber lebih rendah, seperti aturan `permissions.allow` dan hooks, ke kebijakan, jadi aktifkan hanya ketika setiap sumber yang diberi peringkat di bawah yang tertinggi berada di bawah kontrol administrator.

Tabel ini menunjukkan bagaimana Claude Code menggabungkan setiap jenis kunci di bawah `"merge"`. Entri [`managedSourcesBehavior`](/docs/id/settings-reference#managedsourcesbehavior) menamai setiap kunci dalam tiga baris: daftar izin pembatasan, nilai yang diambil seluruhnya, dan kunci yang dibaca dari sumber dengan peringkat tertinggi saja.

| Jenis kunci                                                   | Bagaimana Claude Code menggabungkannya                                                                                                                                               | Contoh                                                                                                                      |
| :------------------------------------------------------------ | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------- |
| Daftar                                                        | Menggabungkan entri dari setiap sumber                                                                                                                                               | `permissions.allow`, `hooks`, `sandbox.network.allowedDomains`, `deniedMcpServers`                                          |
| Kunci                                                         | Menerapkan nilai paling ketat yang ditetapkan sumber apa pun; nilai yang lebih longgar berlaku hanya dari sumber dengan peringkat tertinggi                                          | `allowManagedHooksOnly`, `permissions.disableBypassPermissionsMode`, `crossSessionInbound`                                  |
| Daftar izin pembatasan                                        | Mengambil daftar seluruhnya dari sumber dengan peringkat tertinggi yang mengaturnya, tanpa menambahkan entri dari sumber lebih rendah                                                | `availableModels`, `allowedMcpServers`, `strictKnownMarketplaces`, `allowedChannelPlugins`, dan rantai `fallbackModel`      |
| Nilai yang diambil seluruhnya                                 | Mengambil nilai seluruhnya dari sumber dengan peringkat tertinggi yang mengaturnya, tanpa menggabungkan entri atau bidang dari sumber lebih rendah                                   | `sandbox.credentials.awsPairs`, `sandbox.ripgrep`                                                                           |
| Server MCP yang disediakan                                    | Menggabungkan nama server dari setiap sumber; ketika dua sumber menetapkan nama yang sama, menerapkan entri seluruh sumber dengan peringkat lebih tinggi                             | `managedMcpServers`                                                                                                         |
| Kunci yang dibaca dari sumber dengan peringkat tertinggi saja | Mengabaikan kunci dalam setiap sumber lebih rendah, bahkan ketika sumber dengan peringkat tertinggi membiarkannya tidak diatur                                                       | Pembantu kredensial seperti `apiKeyHelper`, pin login seperti `forceLoginOrgUUID`, `modelPicker`, `permissions.defaultMode` |
| `env`                                                         | Menggabungkan per variabel di seluruh sumber admin di bawah pengaturan apa pun, seperti [Kunci yang dibaca dari setiap sumber admin](#keys-read-from-every-admin-source) menjelaskan |                                                                                                                             |
| Setiap kunci lainnya                                          | Mengambil nilai dari sumber dengan peringkat tertinggi yang mengaturnya                                                                                                              | `model`, `cleanupPeriodDays`                                                                                                |

Untuk mengonfirmasi sumber mana yang digabungkan di mesin, [baca baris `Setting sources` dalam `/status`](#read-the-source-in-/status); bagian itu mengatakan apa arti setiap label.

<h3 id="compute-the-policy-with-a-helper-program">
  Hitung kebijakan dengan program pembantu
</h3>

[`policyHelper`](/docs/id/settings-reference#policyhelper) adalah executable yang dinamai kebijakan MDM atau file pengaturan terkelola Anda, dan Claude Code menjalankannya untuk menghitung pengaturan terkelola saat startup. Ketika sumber yang dipilih mengonfigurasi satu dan pembantu mengeluarkan objek `managedSettings`, output itu mengubah apa yang Claude Code baca:

* **Objek `managedSettings` yang dipancarkan adalah satu-satunya pengaturan terkelola untuk sesi**, termasuk untuk [kunci yang sebaliknya dibaca dari setiap sumber admin](#keys-read-from-every-admin-source), terlepas dari [`forceRemoteSettingsRefresh`, yang memiliki aturan startup sendiri](/docs/id/settings-reference#forceremotesettingsrefresh)

Untuk kegagalan pembantu mana yang terjadi, dan apa yang Claude Code lakukan ketika terjadi, lihat [Helper failures](/docs/id/settings-reference#helper-failures).

<span id="parent-settings-from-embedding-hosts" />

<span id="control-policy-from-an-embedding-host" />

<span id="merge-policy-from-an-embedding-host" />

<h3 id="let-an-embedding-host-add-policy">
  Biarkan host penyematan menambahkan kebijakan
</h3>

Ketika aplikasi lain meluncurkan Claude Code, seperti Claude Desktop, ekstensi IDE, atau aplikasi Agent SDK, host itu dapat melewatkan pengaturan terkelolanya sendiri melalui opsi SDK `managedSettings`. Claude Code menyebut ini pengaturan induk.

Secara default, Claude Code mengabaikan pengaturan induk setiap kali sumber admin ada: pengaturan yang dikelola server, kebijakan MDM atau tingkat OS, atau file pengaturan terkelola.

Untuk membuat Claude Code menggabungkan pengaturan induk bersama sumber admin, atur [`parentSettingsBehavior`](/docs/id/settings-reference#parentsettingsbehavior) ke `"merge"` dalam sumber terkelola dengan prioritas tertinggi; Claude Code membaca kunci dari sumber itu saja.

Claude Code kemudian menyimpan hanya nilai host yang membatasi apa yang dapat dilakukan Claude, dengan satu celah untuk diketahui: kecuali Anda juga menetapkan kunci `allowManaged*Only`, aturan izin host dan daftar sandbox masih berlaku. Lihat [Restrict parent settings](/docs/id/claude-apps-gateway#restrict-parent-settings) untuk kunci.

[`policyHelper`](/docs/id/settings-reference#policyhelper) dapat mematikan penggabungan induk terlepas dari kunci ini; entrinya mengatakan kapan.

Claude Code juga menerapkan pemeriksaan ini ke nilai yang disediakan induk sendiri:

* Ketika sumber admin apa pun menetapkan `allowManagedPermissionRulesOnly`, Claude Code menghapus aturan izin [yang disediakan induk](/docs/id/claude-apps-gateway#restrict-parent-settings) dan `additionalDirectories` saat membacanya, bahkan ketika sumber prioritas lebih tinggi membiarkan kunci tidak diatur. Efek kunci pada aturan Anda sendiri berasal dari pengaturan terkelola yang Claude Code terapkan, atau dari pengaturan induk yang telah Anda pilih untuk digabungkan
* Claude Code memberlakukan nilai `forceLoginOrgUUID` atau `allowedMcpServers` dalam pengaturan terkelola yang diterapkan dan memblokir yang disediakan induk. Di luar kunci daftar izin MCP, nilai dalam sumber admin lebih rendah yang Claude Code tidak terapkan tidak berlaku atau memblokir induk.

  Pada Claude Code v2.1.273 atau lebih baru, saat `allowManagedMcpServersOnly` aktif, daftar `allowedMcpServers` dari sumber admin dengan peringkat tertinggi yang mengaturnya berlaku dan memblokir induk, sebagai [kunci lintas sumber](#keys-read-from-every-admin-source). Daftar induk berlaku hanya ketika tidak ada sumber admin yang mengaturnya. Entri [`managedSourcesBehavior`](/docs/id/settings-reference#managedsourcesbehavior) mengatakan sumber mana yang menyediakan setiap kunci di bawah `"merge"`. Sebelum v2.1.223, nilai dalam sumber admin apa pun memblokir induk
* Untuk `availableModels`, Claude Code memberlakukan nilai dalam pengaturan terkelola yang diterapkan dan memblokir daftar yang disediakan induk
* Untuk `strictKnownMarketplaces`, Claude Code juga memberlakukan daftar dalam pengaturan terkelola yang diterapkan dan memblokir yang disediakan induk. Daftar induk berlaku hanya ketika tidak ada sumber terkelola yang diterapkan yang mengaturnya. Memerlukan Claude Code v2.1.282 atau lebih baru
* Sebuah `blockedMarketplaces` yang disediakan induk berlaku selain daftar blokir apa pun yang ditetapkan sumber terkelola. Memerlukan Claude Code v2.1.282 atau lebih baru

<h4 id="keep-cowork-folder-access-when-only-managed-rules-apply">
  Pertahankan akses folder Cowork ketika hanya aturan terkelola yang berlaku
</h4>

[Cowork](https://claude.com/docs/cowork/overview) dalam aplikasi Claude Desktop menjalankan sesinya di Claude Code dan memberikan setiap sesi akses ke folder kerjanya, seperti folder yang dihubungkan pengguna, melalui aturan izin yang disediakannya saat meluncurkan sesi. Ketika kebijakan terkelola Anda menetapkan [`allowManagedPermissionRulesOnly`](/docs/id/settings-reference#allowmanagedpermissionrulesonly), Claude Code menyimpan hanya aturan izin dalam kebijakan terkelola: itu menghapus aturan izin yang disediakan host sebagai pengaturan induk, sebagai `--allowedTools`, atau dalam file pengaturan, jadi penulisan ke folder tersebut kehilangan pra-persetujuan mereka. Dalam sesi Cowork yang meminta sebelum edit, Cowork tidak dapat menampilkan prompt, dan Claude melaporkan setiap penulisan sebagai diblokir karena jalur diselesaikan ke lokasi yang dilindungi atau jalur di luar folder yang terhubung.

Untuk memulihkan penulisan, tambahkan aturan izin untuk folder tersebut ke sumber terkelola yang Claude Code [pilih](#precedence-within-the-managed-tier) di mesin tersebut: di armada yang dikelola MDM, itu adalah kebijakan MDM daripada file pengaturan terkelola terpisah. Contoh ini menggunakan bentuk file, dan kebijakan MDM mengambil kunci yang sama. Itu menyimpan `allowManagedPermissionRulesOnly` diatur dan memungkinkan edit di bawah folder `CoworkProjects` di direktori home setiap pengguna; ganti jalur dengan folder yang dihubungkan pengguna Anda:

```json managed-settings.json theme={null}
{
  "allowManagedPermissionRulesOnly": true,
  "permissions": {
    "allow": [
      "Edit(~/CoworkProjects/**)"
    ]
  }
}
```

Setelah Anda menerapkan kebijakan, Claude dapat menyimpan file di bawah folder itu dalam sesi Cowork baru. [Read and Edit rules](/docs/id/permissions#read-and-edit) mencakup sintaks jalur, termasuk bentuk `//` untuk jalur absolut.

<h3 id="what-a-developer-can-change">
  Apa yang dapat diubah pengembang
</h3>

File pengaturan pengembang sendiri, nilai `--settings`, dan file proyek tidak pernah mengganti nilai terkelola; [pengecualian](/docs/id/settings#exceptions-to-managed-settings-precedence) hanya membiarkan nilai tingkat lebih rendah yang lebih ketat dihitung. Kasus-kasus ini berada di luar aturan itu:

* **Model untuk sesi**: `model` terkelola adalah default, bukan kunci. `--model` dan `ANTHROPIC_MODEL` masih memilih model untuk sesi itu, jadi terapkan [`availableModels`](/docs/id/settings-reference#availablemodels) untuk membatasi pilihan.
* **Hak admin lokal**: pengembang yang merupakan administrator di mesin dapat mengedit sumber terkelola itu sendiri, itulah mengapa alat MDM dapat menerapkan kembali profil atau file sesuai jadwal dan mengapa registri HKLM dan domain preferensi terkelola macOS ada.
* **Cache yang dikelola server**: pengaturan yang dikelola server berasal dari server Anthropic, dan edit ke cache lokal [berlangsung hanya sampai pengambilan berikutnya yang berhasil](/docs/id/server-managed-settings#security-considerations).
* **Alat lain**: pengaturan terkelola mengikat Claude Code saja. Pengembang yang memanggil API dari alat lain tidak berada di bawahnya.

<span id="verify-enforcement" />

<span id="verify-that-a-policy-is-in-force" />

<h2 id="check-that-a-policy-is-in-force">
  Periksa bahwa kebijakan berlaku
</h2>

Pengembang melaporkan bahwa kebijakan tidak diterapkan, atau Anda ingin mengonfirmasi rollout mendarat sebelum mendorongnya ke armada. Dua perintah di mesin itu menjawabnya: `/status` menunjukkan sumber terkelola mana yang Claude Code pilih, dan `claude doctor` mencantumkan apa yang dihapusnya.

<h3 id="read-the-source-in-/status">
  Baca sumber dalam /status
</h3>

Di mesin pengembang, jalankan `/status` di dalam Claude Code dan baca baris `Setting sources`. Ketika sumber terkelola berlaku, baris mencantumkan `Enterprise managed settings` dengan sumber yang Claude Code pilih dalam tanda kurung:

* `(remote)`: pengaturan terkelola server dari claude.ai atau gateway
* `(plist)` atau `(HKLM)`: kebijakan MDM atau OS
* `(file)`, `(drop-ins)`, atau `(file + drop-ins)`: `managed-settings.json`, direktori drop-in, atau keduanya
* `(remote + file, merged)`, atau daftar lain yang diakhiri dengan `, merged`: organisasi Anda [menyusun setiap sumber terkelola](#compose-every-managed-source), dan Claude Code menggabungkan sumber yang tercantum ke dalam kebijakan. Sumber lebih rendah masih dapat menyediakan variabel `env` tanpa muncul dalam daftar. Memerlukan Claude Code v2.1.242 atau lebih baru
* `(HKCU)`: fallback registry yang dapat ditulis pengguna
* `(parent process)`: [host embedding](#let-an-embedding-host-add-policy) menyediakan pengaturan yang ketat
* `(helper)`: [`policyHelper`](/docs/id/settings-reference#policyhelper) dikonfigurasi oleh sumber MDM atau file yang dipilih

Ketika Claude Code menemukan sumber terkelola di mesin dan tidak memilihnya, baris kedua, `Skipped sources`, menamai setiap sumber seperti itu. Bacanya untuk membedakan kebijakan yang tidak pernah mencapai mesin dari yang mencapainya dan sumber prioritas lebih tinggi mengesampingkannya. Memerlukan Claude Code v2.1.242 atau lebih baru.

Ketika kebijakan tidak diterapkan, baris `Setting sources` memberi tahu Anda masalah mana dari dua yang Anda miliki:

* **Baris hilang**: Claude Code tidak menemukan sumber terkelola yang mengirimkan kunci kebijakan.

  Jika Anda terapkan file pengaturan terkelola, periksa bahwa itu duduk di jalur untuk OS dan bahwa itu berisi [kunci kebijakan](#how-claude-code-combines-managed-sources) daripada hanya kunci kontrol. File yang bukan JSON yang valid tidak menghasilkan keadaan ini; Claude Code [menolak untuk memulai](#find-entries-claude-code-dropped) sebagai gantinya.

  Ketika Anda terapkan melalui pengaturan terkelola server sebagai gantinya, jalankan `claude doctor`, yang melaporkan [hasil pengambilan](/docs/id/server-managed-settings#verify-settings-delivery).
* **Baris menamai sumber selain yang Anda terapkan**: sumber prioritas lebih tinggi ada dan Claude Code mengabaikan milik Anda, dan `Skipped sources` mencantumkannya. [Bagaimana Claude Code menggabungkan sumber terkelola](#how-claude-code-combines-managed-sources) memberikan urutan.

<span id="invalid-entries-in-managed-settings" />

<h3 id="find-entries-claude-code-dropped">
  Temukan entri yang Claude Code hapus
</h3>

Ketika file pengaturan terkelola, profil MDM, nilai registry, atau payload terkelola server gagal validasi skema, Claude Code pertama melewatkan entri individual yang dapat diperbaiki, seperti satu aturan izin yang tidak valid, dengan peringatan untuk masing-masing, kemudian menghapus kunci tingkat atas yang nilainya masih gagal dan terus memberlakukan setiap kunci yang valid yang tersisa.

Claude Code lebih ketat dengan `managedSettings` yang [`policyHelper`](/docs/id/settings-reference#policyhelper) pancarkan: itu membuat perbaikan entri yang sama, tetapi pelanggaran skema apa pun yang bertahan gagal seluruh jalankan pembantu, dan saat startup Claude Code menolak untuk memulai, sama seperti untuk pembantu yang keluar non-nol.

Ketika file pengaturan terkelola, file drop-in, plist MDM, atau nilai registry HKLM ada tetapi tidak dapat diuraikan sebagai objek JSON, Claude Code menolak untuk memulai dan mencetak [kesalahan yang menamai sumber](/docs/id/errors#managed-settings-document-could-not-be-parsed), bahkan ketika sumber admin lain mengirimkan kebijakan yang valid. Setiap sumber gagal dengan cara ini ketika:

* **File pengaturan terkelola atau file drop-in**: file bukan JSON yang valid, atau tingkat atasnya bukan objek
* **Plist MDM**: `plutil` macOS melaporkan plist yang salah bentuk, atau konten yang dikonversi bukan objek JSON
* **Nilai registry HKLM**: nilai `Settings` bukan string, kosong, atau tidak menyimpan objek JSON

Tiga keadaan sumber tidak menyebabkan penolakan ini:

* File, profil, atau nilai registry yang tidak ada bukan kegagalan; Claude Code berjalan tanpa sumber itu.
* File pengaturan terkelola kosong dihitung sebagai `{}`.
* Nilai yang salah bentuk dalam kunci registry HKCU yang dapat ditulis pengguna tidak pernah memblokir peluncuran. Claude Code melaporkannya sebagai pemberitahuan dalam `/status` dan `claude doctor` sebagai gantinya.

Jika file pengaturan terkelola, file drop-in, atau direktori `managed-settings.d/` tidak dapat dibaca dan tidak ada sumber admin yang menyediakan kebijakan, sesi yang masuk dengan kredensial claude.ai atau Claude Console keluar saat startup dengan pesan untuk menghubungi administrator.

Untuk menemukan entri yang dihapus, lihat di salah satu dari tiga tempat:

* Sesi interaktif menunjukkan dialog saat startup yang mencantumkan entri yang tidak valid.
* Jalankan non-interaktif dengan `-p` mencetak ringkasan ke stderr.
* [`claude doctor`](/docs/id/debug-your-config) mencantumkan setiap entri yang tidak valid dengan sumber dan bidangnya.

<h4 id="keys-that-fail-closed">
  Kunci yang gagal tertutup
</h4>

Beberapa kunci penegakan tidak dihapus ketika tidak valid. Claude Code memberlakukan fallback yang lebih ketat sampai nilai diperbaiki; tabel menunjukkan apa yang diberlakukannya untuk setiap kunci:

| Bidang                        | Perilaku ketika ada tetapi tidak valid                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| :---------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `allowedMcpServers`           | Diberlakukan sebagai daftar izin kosong sampai nilai diperbaiki, jadi tidak ada server MCP yang pengguna tambahkan yang diakui. Server yang organisasi Anda berikan melalui [`managedMcpServers`](/docs/id/settings-reference#managedmcpservers) masih dimuat, dan server `managed-mcp.json` dimuat per [Bagaimana server dievaluasi](/docs/id/managed-mcp#how-a-server-is-evaluated). Entri individual yang tidak valid dilepas dan subset yang valid diberlakukan.                                                           |
| `allowedHttpHookUrls`         | Claude Code memberlakukan daftar izin terkelola kosong [allowlist](/docs/id/settings-reference#allowedhttphookurls) sampai Anda memperbaiki nilai, jadi HTTP hook berjalan hanya jika file pengaturan lain mencantumkan URL-nya. Jika hanya entri individual yang tidak valid, Claude Code melepas entri itu dan memberlakukan sisanya.                                                                                                                                                                                   |
| `httpHookAllowedEnvVars`      | Claude Code memberlakukan daftar izin terkelola kosong [allowlist](/docs/id/settings-reference#httphookallowedenvvars) sampai Anda memperbaiki nilai, jadi variabel header diinterpolasi hanya jika file pengaturan lain menamainya. Jika hanya entri individual yang tidak valid, Claude Code melepas entri itu dan memberlakukan sisanya.                                                                                                                                                                               |
| `allowedChannelPlugins`       | Claude Code memberlakukan daftar izin kosong sampai Anda memperbaiki nilai, jadi tidak ada plugin saluran yang dilewatkan ke `--channels` yang diakui. Jika hanya entri individual yang tidak valid, itu melepas entri itu dan memberlakukan sisanya.                                                                                                                                                                                                                                                                |
| `strictKnownMarketplaces`     | Diberlakukan sebagai daftar izin kosong sampai nilai diperbaiki, jadi tidak ada [sumber marketplace](/docs/id/plugins/org#restrict-what-users-can-install) yang diakui. Entri individual yang tidak valid atau tidak dapat diberlakukan, seperti regex `hostPattern` yang tidak dikompilasi, dilepas dan subset yang valid diberlakukan.                                                                                                                                                                                  |
| `allowManagedHooksOnly`       | Diperlakukan sebagai `true` sampai diperbaiki: [pembatasan hook](/docs/id/settings-reference#allowmanagedhooksonly) berlaku dan, kecuali `disableCommandPluginSources` secara eksplisit `false`, plugin bersumber perintah dinonaktifkan.                                                                                                                                                                                                                                                                                 |
| `allowManagedMcpServersOnly`  | Diperlakukan sebagai `true`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `disableCommandPluginSources` | Diperlakukan sebagai `true`, jadi plugin bersumber perintah tetap dinonaktifkan sampai nilai diperbaiki.                                                                                                                                                                                                                                                                                                                                                                                                             |
| `disableSideloadFlags`        | Diperlakukan sebagai `true` sampai nilai diperbaiki, dengan efek yang tercantum untuk [`disableSideloadFlags`](/docs/id/settings-reference#disablesideloadflags).                                                                                                                                                                                                                                                                                                                                                         |
| `availableModels`             | Diberlakukan sebagai daftar izin kosong sampai diperbaiki, jadi hanya model Default yang tersedia; entri non-string dilepas dan subset yang valid diberlakukan.                                                                                                                                                                                                                                                                                                                                                      |
| `enforceAvailableModels`      | Diperlakukan sebagai `true`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `syncClaudeAiPlugins`         | Diperlakukan sebagai `false`, jadi sinkronisasi [plugin claude.ai](/docs/id/settings-reference#syncclaudeaiplugins) mati sampai nilai diperbaiki.                                                                                                                                                                                                                                                                                                                                                                         |
| `forceLoginOrgUUID`           | Tidak ada organisasi yang diizinkan untuk masuk sampai nilai diperbaiki.                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `gatewayInternalNetworks`     | Ketika nilai yang tidak valid berasal dari sumber terkelola tertinggi di mesin, `/login` menolak setiap [gateway cloud](/docs/id/claude-apps-gateway#allow-a-gateway-on-public-address-space-you-own) masuk baru di mesin itu sampai nilai diperbaiki.                                                                                                                                                                                                                                                                    |
| `crossSessionInbound`         | Diperlakukan sebagai `refuse`, nilai paling ketat, jadi [pesan lintas sesi](/docs/id/cross-session-messaging#control-inbound-messages) masuk ditolak sampai nilai diperbaiki. Pengembang melihat [peringatan](/docs/id/errors#crosssessioninbound-must-be-one-of-accept-hold-refuse).                                                                                                                                                                                                                                          |
| `deniedMcpServers`            | Entri individual yang tidak valid dilepas dan subset yang valid diberlakukan. Nilai yang sepenuhnya tidak valid dihapus dengan peringatan, karena menolak setiap server akan memblokir server yang kebijakan tidak pernah namai.                                                                                                                                                                                                                                                                                     |
| `blockedMarketplaces`         | Entri individual yang tidak valid dilepas dan subset yang valid diberlakukan. Entri yang diuraikan tetapi tidak pernah cocok, seperti regex `hostPattern` yang tidak dikompilasi, disimpan dengan peringatan. Itu tidak memblokir apa pun sampai diperbaiki, tetapi [pembatasan marketplace](/docs/id/plugins/org#restrict-what-users-can-install) tetap aktif. Nilai yang sepenuhnya tidak valid dihapus dengan peringatan, karena memblokir setiap marketplace akan memblokir sumber yang kebijakan tidak pernah namai. |
| `sandbox.credentials`         | Entri yang tidak valid yang dapat dipulihkan diturunkan ke `mode: "deny"` dengan peringatan; yang tidak dapat dipulihkan dilepas; entri yang valid tetap diberlakukan. Lihat [entri kredensial yang tidak valid](/docs/id/settings-reference#invalid-credential-entries-in-managed-settings)                                                                                                                                                                                                                              |

`allowedHttpHookUrls` dan `httpHookAllowedEnvVars` menggabungkan di seluruh file pengaturan, jadi entri dalam pengaturan pengguna, proyek, atau lokal Anda masih berlaku sementara daftar terkelola kosong.

Fallback untuk dua kunci itu dan untuk `allowedChannelPlugins` memerlukan Claude Code v2.1.267 atau lebih baru; versi sebelumnya menghapus seluruh kunci ketika nilainya atau entri apa pun tidak valid. Fallback `strictKnownMarketplaces`, `blockedMarketplaces`, dan `disableSideloadFlags` memerlukan Claude Code v2.1.277 atau lebih baru; versi sebelumnya menghapus seluruh kunci ketika nilainya atau entri apa pun tidak valid.

`requiredMinimumVersion` dan `requiredMaximumVersion` gagal terbuka dengan desain: nilai yang tidak valid dihapus daripada diberlakukan.

Toleransi ini hanya berlaku untuk pengaturan terkelola. File pengaturan pengguna, proyek, dan lokal tetap ketat: file yang JSON atau bentuk tingkat atasnya gagal validasi ditolak secara keseluruhan dan dilaporkan, dan entri individual yang gagal, seperti aturan izin yang salah bentuk, dilewatkan dengan peringatan sementara sisa file berlaku.

<span id="managed-only-settings" />

<h2 id="keys-only-a-managed-source-can-set">
  Kunci yang hanya sumber terkelola yang dapat atur
</h2>

Claude Code membaca kunci berikut hanya dari sumber terkelola; menempatkannya dalam file pengaturan pengguna atau proyek tidak berpengaruh.

Sebagian besar adalah kunci: nilai yang dikunci, seperti aturan izin atau `sandbox.network.allowedDomains`, adalah kunci biasa yang dapat diatur tingkat apa pun, dan kunci memberi tahu Claude Code untuk menghormati hanya nilai terkelola.

Tabel mencakup kontrol izin, plugin, dan pengiriman. Untuk kunci apa pun yang tidak tercantum di sini, kolom Scope dari [referensi pengaturan](/docs/id/settings-reference#all-settings) index mengatakan apakah itu terkelola saja; kunci terkelola saja yang tersisa di sana termasuk URL login gateway, versi, browser, mobile-simulator, host SSH, Desktop sesi lokal, jalur biner sandbox, harga model, dan kontrol CLAUDE.md.

| Pengaturan                                                                                                            | Deskripsi                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| :-------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [`allowAllClaudeAiMcps`](/docs/id/settings-reference#allowallclaudeaimcps)                                                 | Muat konektor claude.ai yang Claude Code ambil sendiri bersama `managed-mcp.json` yang diterapkan alih-alih menekannya                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| [`allowedChannelPlugins`](/docs/id/settings-reference#allowedchannelplugins)                                               | Daftar izin plugin channel yang dapat mendorong pesan. Mengganti daftar izin Anthropic default ketika diatur. Memerlukan `channelsEnabled: true`. Lihat [Batasi plugin channel mana yang dapat berjalan](/docs/id/channels#restrict-which-channel-plugins-can-run)                                                                                                                                                                                                                                                                                                    |
| [`allowManagedHooksOnly`](/docs/id/settings-reference#allowmanagedhooksonly)                                               | Ketika `true`, membatasi hook mana yang berjalan; lihat [apa yang berjalan di bawah `allowManagedHooksOnly`](/docs/id/settings-reference#what-runs-under-allowmanagedhooksonly) untuk daftar efek lengkap                                                                                                                                                                                                                                                                                                                                                             |
| [`allowManagedMcpServersOnly`](/docs/id/settings-reference#allowmanagedmcpserversonly)                                     | Ketika `true`, hanya `allowedMcpServers` dari pengaturan terkelola yang dihormati. `deniedMcpServers` masih merge dari semua sumber. Lihat [Kunci yang dibaca dari setiap sumber admin](#keys-read-from-every-admin-source) untuk sumber terkelola mana yang dapat mengaturnya, dan [Konfigurasi MCP terkelola](/docs/id/managed-mcp)                                                                                                                                                                                                                                 |
| [`allowManagedPermissionRulesOnly`](/docs/id/settings-reference#allowmanagedpermissionrulesonly)                           | Membuat pengaturan terkelola satu-satunya sumber pengaturan aturan izin. Entri mencantumkan setiap sumber yang diabaikannya                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| [`blockedMarketplaces`](/docs/id/settings-reference#blockedmarketplaces)                                                   | Daftar blokir sumber marketplace. Sumber yang diblokir diperiksa sebelum mengunduh, jadi mereka tidak pernah menyentuh filesystem. Lihat [pembatasan marketplace terkelola](/docs/id/plugins/org#restrict-what-users-can-install)                                                                                                                                                                                                                                                                                                                                     |
| [`channelsEnabled`](/docs/id/settings-reference#channelsenabled)                                                           | Izinkan [channels](/docs/id/channels) untuk organisasi. Lihat [kontrol enterprise](/docs/id/channels#enterprise-controls) untuk default di setiap paket                                                                                                                                                                                                                                                                                                                                                                                                                    |
| [`disableCommandPluginSources`](/docs/id/settings-reference#disablecommandpluginsources)                                   | Ketika `true`, memblokir [sumber plugin `command`](/docs/id/plugins/marketplace-reference#command-plugin-source) sepenuhnya, jadi perintah yang dideklarasikan marketplace tidak pernah berjalan. Juga memblokir perintah [`headersHelper`](/docs/id/plugins/host-marketplace#authenticate-archive-downloads) marketplace, kecuali untuk marketplace yang pengaturan terkelola sendiri deklarasikan. Ketika tidak diatur, mengikuti `allowManagedHooksOnly`. Memerlukan Claude Code v2.1.229 atau lebih baru, dan blok `headersHelper` memerlukan v2.1.238 atau lebih baru |
| [`disableSideloadFlags`](/docs/id/settings-reference#disablesideloadflags)                                                 | Tolak flag `--plugin-dir`, `--plugin-url`, `--agents`, dan `--mcp-config` saat startup. Dalam sesi cloud, Claude Code menghapus server MCP yang server kirimkan melalui `--mcp-config`, selain entri `type: "sdk"` dalam proses, dan memulai sesi. Memerlukan Claude Code v2.1.193 atau lebih baru                                                                                                                                                                                                                                                               |
| [`forceRemoteSettingsRefresh`](/docs/id/settings-reference#forceremotesettingsrefresh)                                     | Ketika `true`, memblokir startup CLI sampai pengaturan terkelola jarak jauh diambil segar dan keluar jika pengambilan gagal. Lihat [penegakan fail-closed](/docs/id/server-managed-settings#enforce-fail-closed-startup)                                                                                                                                                                                                                                                                                                                                              |
| [`managedMcpServers`](/docs/id/settings-reference#managedmcpservers)                                                       | Server MCP jarak jauh yang disediakan untuk setiap pengguna bersama milik mereka sendiri. Ini menyediakan server daripada mengunci apa pun. Lihat [Sediakan server melalui pengaturan terkelola](/docs/id/managed-mcp#provide-servers-through-managed-settings). Memerlukan Claude Code v2.1.259 atau lebih baru                                                                                                                                                                                                                                                      |
| [`managedSourcesBehavior`](/docs/id/settings-reference#managedsourcesbehavior)                                             | Apakah Claude Code menerapkan hanya sumber terkelola prioritas tertinggi atau [menyusun setiap satu](#compose-every-managed-source)                                                                                                                                                                                                                                                                                                                                                                                                                              |
| [`parentSettingsBehavior`](/docs/id/settings-reference#parentsettingsbehavior)                                             | Apakah pengaturan induk yang disediakan host merge di bawah kebijakan terkelola                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| [`pluginSuggestionMarketplaces`](/docs/id/settings-reference#pluginsuggestionmarketplaces)                                 | Marketplace yang plugin Claude Code dapat sarankan kepada pengguna                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| [`pluginTrustMessage`](/docs/id/settings-reference#plugintrustmessage)                                                     | Pesan khusus ditambahkan ke peringatan kepercayaan plugin yang ditampilkan sebelum instalasi                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| [`policyHelper`](/docs/id/settings-reference#policyhelper)                                                                 | Executable yang menghitung pengaturan terkelola saat startup; lihat [Hitung pengaturan terkelola dengan pembantu kebijakan](/docs/id/settings-reference#policyhelper)                                                                                                                                                                                                                                                                                                                                                                                                 |
| [`sandbox.filesystem.allowManagedReadPathsOnly`](/docs/id/settings-reference#sandbox-filesystem-allowmanagedreadpathsonly) | Ketika `true`, hanya jalur `filesystem.allowRead` dari pengaturan terkelola yang dihormati. `denyRead` masih merge dari semua sumber                                                                                                                                                                                                                                                                                                                                                                                                                             |
| [`sandbox.network.allowManagedDomainsOnly`](/docs/id/settings-reference#sandbox-network-allowmanageddomainsonly)           | Hormati hanya `allowedDomains` terkelola dan aturan izin `WebFetch(domain:...)`; blokir domain lain tanpa meminta                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| [`strictKnownMarketplaces`](/docs/id/settings-reference#strictknownmarketplaces)                                           | Mengontrol sumber marketplace plugin mana yang pengguna dapat tambahkan dan instal plugin darinya. Lihat [pembatasan marketplace terkelola](/docs/id/plugins/org#restrict-what-users-can-install)                                                                                                                                                                                                                                                                                                                                                                     |
| [`strictPluginOnlyCustomization`](/docs/id/settings-reference#strictpluginonlycustomization)                               | Blokir skills, agents, hooks, dan server MCP dari sumber pengguna dan proyek; `true` mengunci keempat, array menamai yang mana                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| [`wslInheritsWindowsSettings`](/docs/id/settings-reference#wslinheritswindowssettings)                                     | Ketika diatur dalam registry HKLM atau file di bawah `C:\Program Files\ClaudeCode`, biarkan WSL membaca rantai kebijakan Windows, dan baca `/etc/claude-code` hanya ketika tidak ada file pengaturan terkelola atau drop-in di bawah direktori itu yang mengirimkan [kunci kebijakan](#how-claude-code-combines-managed-sources); entri memberikan urutan                                                                                                                                                                                                        |

<Note>
  Pada paket Team dan Enterprise, Owner mengaktifkan atau menonaktifkan [Remote Control](/docs/id/remote-control) dan [sesi web](/docs/id/claude-code-on-the-web) di seluruh organisasi dalam [pengaturan admin Claude Code](https://claude.ai/admin-settings/claude-code). Remote Control dapat juga dinonaktifkan per perangkat dengan pengaturan [`disableRemoteControl`](/docs/id/settings-reference#disableremotecontrol). Sesi web tidak memiliki kunci pengaturan terkelola per perangkat.

  Untuk memeriksa apakah pengaturan organisasi ini mencapai mesin tertentu, jalankan `claude doctor` di sana dan baca baris `Organization policy`, yang mengatakan di mana Claude Code memuat kebijakan atau mengapa tidak memuat. Memerlukan Claude Code v2.1.261 atau lebih baru. Dalam sesi yang berjalan, `/status` menunjukkan baris yang sama ketika kebijakan tidak memuat.
</Note>

<h2 id="turn-telemetry-off-for-your-organization">
  Matikan telemetri untuk organisasi Anda
</h2>

Claude Code mengirimkan [telemetri](/docs/id/data-usage#telemetry-services) operasional Anthropic secara default pada sesi yang menggunakan API Anthropic, baik secara langsung, melalui gateway LLM, atau melalui `ANTHROPIC_BASE_URL` khusus; [Perilaku default oleh penyedia API](/docs/id/data-usage#default-behaviors-by-api-provider) mengatakan penyedia mana yang mengirimkannya. Untuk mematikannya untuk setiap pengembang tanpa mengandalkan shell setiap orang, kirimkan `DISABLE_TELEMETRY` melalui blok `env` pengaturan terkelola Anda. Contoh ini menetapkan `DISABLE_TELEMETRY` untuk semua orang yang kebijakan jangkau:

```json theme={null}
{
  "env": {
    "DISABLE_TELEMETRY": "1"
  }
}
```

Claude Code menerapkan nilai `1` tanpa menunjukkan pengguna [dialog persetujuan](/docs/id/server-managed-settings#environment-variables-and-the-approval-dialog).

Jika Anda matikan telemetri, Claude Code berhenti mengirimkan data penggunaan yang memberi makan [dasbor analitik](/docs/id/analytics) organisasi Anda untuk pengembang yang kebijakan jangkau. Variabel juga mematikan pengambilan flag fitur, yang membuat Remote Control, mode auto default, dan [fitur lain yang memerlukan pengambilan flag fitur](/docs/id/env-vars#features-that-need-feature-flag-fetching) tidak tersedia untuk pengembang itu.

[Di mana dan kapan kebijakan berlaku](#where-and-when-a-policy-applies) mengatakan mekanisme pengiriman mana yang menjangkau setiap permukaan, dan [Ketersediaan platform](/docs/id/server-managed-settings#platform-availability) mengatakan sesi mana yang melewatkan pengambilan pengaturan terkelola server.

Jika organisasi Anda menggunakan kunci enkripsi yang dikelola pelanggan dan merutekan Claude Code melalui gateway, [Konfigurasi proxy dan gateway](/docs/id/third-party-integrations#configure-proxies-and-gateways) mengatakan mengapa sesi itu memerlukan variabel ini.

<h2 id="see-also">
  Lihat juga
</h2>

* [Siapkan Claude Code untuk organisasi Anda](/docs/id/admin-setup): tentukan apa yang akan diberlakukan dan bagaimana
* [Pengaturan terkelola server](/docs/id/server-managed-settings): kirimkan kebijakan dari konsol claude.ai atau gateway
* [Konfigurasi MCP terkelola](/docs/id/managed-mcp): kontrol server MCP mana yang dapat digunakan pengembang
* [Semua pengaturan](/docs/id/settings-reference): setiap kunci, dengan apakah sumber terkelola dapat menetapkannya
* [File pengaturan contoh](/docs/id/settings-example#an-organizations-managed-settings): `managed-settings.json` lengkap yang menunjukkan bentuk kunci terkelola
