> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Pilih mode izin

> Kontrol apakah Claude meminta izin sebelum bertindak. Alihkan mode izin dengan Shift+Tab di CLI, indikator mode di VS Code, atau pemilih mode di Desktop.

Mode izin menetapkan tindakan mana yang dapat dilakukan Claude dalam sesi tanpa meminta Anda terlebih dahulu. Dalam mode Manual, Claude Code berhenti dan meminta Anda sebelum sebagian besar tindakan yang mengedit file, menjalankan perintah shell, atau menjangkau jaringan. Dalam [mode auto](#eliminate-prompts-with-auto-mode), model kedua, pengklasifikasi, meninjau tindakan sebagai gantinya Anda; [bagaimana pengklasifikasi mengevaluasi tindakan](#how-the-classifier-evaluates-actions) mencantumkan tindakan mana yang ditinjau dan mana yang melewatinya.

Pada paket Pro, Max, dan Team, mode izin awal bawaan adalah mode auto. [Mode mana yang dimulai sesi](#which-mode-a-session-starts-in) mencakup permukaan dan pengaturan yang mengubah mode izin awal. Anda juga dapat mengubah mode izin sesi yang sedang berjalan kapan saja.

<h2 id="available-modes">
  Mode yang tersedia
</h2>

Setiap mode membuat pertukaran yang berbeda antara kenyamanan dan pengawasan. Tabel di bawah menunjukkan apa yang dapat dilakukan Claude tanpa permintaan izin di setiap mode. Mode Manual muncul di bawah nilai konfignya, `default`.

| Mode                                                                | Apa yang berjalan tanpa bertanya                                                                                     | Terbaik untuk                                        |
| :------------------------------------------------------------------ | :------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------- |
| `default`                                                           | Hanya membaca                                                                                                        | Meninjau setiap tindakan sendiri, pekerjaan sensitif |
| [`acceptEdits`](#auto-approve-file-edits-with-acceptedits-mode)     | Membaca, pengeditan file, dan perintah sistem file umum (`mkdir`, `touch`, `mv`, `cp`, dll.)                         | Iterasi pada kode yang Anda tinjau                   |
| [`plan`](#analyze-before-you-edit-with-plan-mode)                   | Membaca, plus perintah yang disetujui pengklasifikasi ketika [mode auto](#eliminate-prompts-with-auto-mode) tersedia | Menjelajahi basis kode sebelum mengubahnya           |
| [`auto`](#eliminate-prompts-with-auto-mode)                         | Segalanya, dengan pemeriksaan keamanan latar belakang                                                                | Tugas panjang, mengurangi kelelahan prompt           |
| [`dontAsk`](#allow-only-pre-approved-tools-with-dontask-mode)       | Membaca dan alat yang telah disetujui sebelumnya; apa pun yang akan meminta izin ditolak                             | CI dan skrip terkunci                                |
| [`bypassPermissions`](#skip-all-checks-with-bypasspermissions-mode) | Segalanya                                                                                                            | Hanya kontainer dan VM terisolasi                    |

Mode yang meninjau setiap tindakan dinamai **Manual** di CLI, di `claude --help`, di ekstensi VS Code dan JetBrains, dan di aplikasi desktop. Nilai konfignya adalah `default`, yang digunakan oleh hooks dan integrasi SDK. CLI menerima `manual` sebagai alias di mana pun Anda mengetik nilainya, misalnya `claude --permission-mode manual` atau `"defaultMode": "manual"`. Label Manual dan alias `manual` memerlukan Claude Code v2.1.200 atau lebih baru. Label aplikasi desktop tidak bergantung pada versi CLI Anda.

Penulisan ke [jalur yang dilindungi](#protected-paths) tidak pernah disetujui otomatis kecuali dalam mode `bypassPermissions` dan dalam sesi mode plan di mana izin bypass tersedia, artinya sesi terminal interaktif yang dimulai dengan cara yang [menempatkan `bypassPermissions` dalam siklus mode](#switch-permission-modes).

Mode menetapkan garis dasar. Lapisi [aturan izin](/docs/id/permissions#manage-permissions) di atas untuk menyetujui sebelumnya atau memblokir alat tertentu. Aturan Deny memblokir di setiap mode, termasuk `bypassPermissions`. Aturan Deny dan ask tidak berlaku untuk [`EndConversation`](/docs/id/tools-reference#endconversation-tool-behavior) selama Claude masih memiliki setidaknya satu alat lain yang dapat dipanggilnya. Aturan Allow tidak berpengaruh dalam `bypassPermissions`.

<h3 id="actions-no-mode-auto-approves">
  Tindakan yang tidak ada mode yang disetujui otomatis
</h3>

Claude Code tidak secara otomatis menyetujui hal-hal berikut di mode apa pun, termasuk `bypassPermissions`. Setiap poin menghubungkan ke bagian yang mengatakan apa yang terjadi sebagai gantinya di setiap mode:

* Alat yang cocok dengan [aturan ask](/docs/id/permissions#manage-permissions) eksplisit
* Alat konektor yang organisasi Anda [atur ke `ask`](/docs/id/mcp#organization-controls-on-connector-tools), dalam sesi di mana pengaturan itu mencapai Claude Code
* Alat yang memerlukan interaksi pengguna: alat `AskUserQuestion` bawaan dan alat MCP yang ditandai [`requiresUserInteraction`](/docs/id/mcp#require-approval-for-a-specific-tool)
* Penghapusan `rm` dan `rmdir` yang menargetkan [jalur kritis](#critical-paths), yang tidak ada aturan allow atau hook `PreToolUse` `"allow"` yang menyetujui
* [Penjaga pesan lintas sesi](#skip-all-checks-with-bypasspermissions-mode)
* Membaca di luar direktori kerja sementara [`permissions.blockReadsOutsideWorkingDirectories`](/docs/id/settings-reference#permissions-blockreadsoutsideworkingdirectories) aktif: perintah Bash pembaca file yang dikenali meminta bahkan dalam mode auto dan mode `bypassPermissions`, dan begitu juga dengan [retry tanpa sandbox](/docs/id/sandboxing#the-unsandboxed-retry-escape-hatch) apa pun yang memerlukan persetujuan untuk berjalan di luar sandbox. Memerlukan Claude Code v2.1.257 atau lebih baru.

  Perintah yang parser shell tidak dapat lacak, seperti yang mengubah direktori lebih dari sekali atau menjalankan subshell, meminta dengan cara yang sama bahkan ketika tidak menamai jalur luar apa pun. Permintaan ini tidak berlaku ketika perintah berjalan di [sandbox](/docs/id/sandboxing) dan sandbox memberlakukan blokir.

<h2 id="common-setups">
  Pengaturan umum
</h2>

Mode izin memutuskan apakah Claude meminta sebelum tindakan, dan [sandbox Bash](/docs/id/sandboxing) dan [batas isolasi](/docs/id/sandbox-environments) luar memutuskan apa yang dapat dijangkau tindakan setelah berjalan. Setiap baris di bawah memasangkan tujuan dengan flag atau pengaturan yang membawanya ke sana dan isolasi yang dibutuhkan, sebagai titik awal. [Mode yang tersedia](#available-modes) mencantumkan apa yang berjalan tanpa prompt di setiap mode.

| Anda ingin                                                       | Mulai dengan                                                                                                                                                                   | Isolasi yang diperlukan                                                                                                                                                                             | Catatan                                                                                                                                                                                                                                                      |
| :--------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Tinjau setiap tindakan sendiri                                   | Mode Manual: `claude --permission-mode default`                                                                                                                                | Tidak ada                                                                                                                                                                                           | Pekerjaan sensitif, kode yang tidak familiar                                                                                                                                                                                                                 |
| Iterasi lokal dengan lebih sedikit prompt, tanpa pengklasifikasi | Mode Manual ditambah sandbox Bash dalam [mode auto-allow](/docs/id/sandboxing#sandbox-modes): `claude --permission-mode default`, kemudian jalankan `/sandbox` dan pilih auto-allow | Sandbox Bash bawaan, di macOS, Linux, dan WSL2                                                                                                                                                      | Aturan deny masih berlaku, dan aturan ask yang menyebutkan perintah, seperti `Bash(git push *)`, masih meminta. Untuk mengaktifkan sandbox dari file pengaturan sebagai gantinya, atur [`sandbox.enabled`](/docs/id/settings-reference#sandbox-enabled) ke `true` |
| Jelajahi sebelum mengubah apa pun                                | `claude --permission-mode plan`                                                                                                                                                | Tidak ada                                                                                                                                                                                           | Claude Code memblokir pengeditan sampai Anda [menyetujui rencana](#review-and-approve-a-plan)                                                                                                                                                                |
| Bekerja hands-off dalam mode auto                                | `claude --permission-mode auto`, [mode izin awal bawaan](#which-mode-a-session-starts-in) pada Pro, Max, dan Team                                                              | Tidak ada; sandbox atau kontainer menambah pertahanan berlapis                                                                                                                                      | Memerlukan [model yang didukung](#eliminate-prompts-with-auto-mode), dan organisasi Anda dapat [mematikan mode auto](#eliminate-prompts-with-auto-mode)                                                                                                      |
| Jalankan di CI dengan allowlist yang tepat                       | `claude -p "run the test suite" --permission-mode dontAsk --allowedTools "Bash(npm test)" "Read"`                                                                              | Tidak ada di luar apa yang disediakan runner CI Anda                                                                                                                                                | [Cloud sessions](/docs/id/claude-code-on-the-web) mengabaikan `dontAsk` dari file pengaturan                                                                                                                                                                      |
| Jalankan sepenuhnya tanpa pengawasan di dalam kontainer          | `claude -p "<prompt>" --dangerously-skip-permissions`                                                                                                                          | Diperlukan: kontainer, VM, atau [runtime sandbox](/docs/id/sandbox-environments#sandbox-runtime); di Linux dan macOS, jalankan sebagai [pengguna non-root](#skip-all-checks-with-bypasspermissions-mode) | Cloud sessions mengabaikan mode ini dari file pengaturan. Dalam jalankan `-p` ini, [beberapa panggilan yang masih akan meminta](#skip-all-checks-with-bypasspermissions-mode) ditolak sebagai gantinya                                                       |

Sandbox Bash dan mode auto bekerja secara independen dan menggabungkan, dengan pengecualian yang tercantum di bawah [Sandbox modes](/docs/id/sandboxing#sandbox-modes). Untuk interaksi penuh, lihat [Bagaimana sandboxing berhubungan dengan izin dan mode izin](/docs/id/sandboxing#how-sandboxing-relates-to-permissions-and-permission-modes) dan [Bagaimana isolasi berhubungan dengan mode izin](/docs/id/sandbox-environments#how-isolation-relates-to-permission-modes).

<h2 id="which-mode-a-session-starts-in">
  Mode mana yang dimulai sesi
</h2>

Ketika Anda memulai sesi baru di terminal, Claude Code mengambil mode izin dari yang pertama dari ini yang berlaku:

1. Flag `--permission-mode`, atau `--dangerously-skip-permissions`

2. `permissions.defaultMode` dalam [file pengaturan](/docs/id/settings#where-settings-live)

   Jika Anda menetapkan `"auto"` di `.claude/settings.json` atau `.claude/settings.local.json`, nilainya tidak berlaku, dan Claude Code kemudian menggunakan default bawaan daripada `defaultMode` dari `~/.claude/settings.json`. Jika Anda menetapkan `"bypassPermissions"` di dua file itu, itu juga tidak berlaku, dan sesi dimulai dalam mode Manual. Nilai lainnya berlaku dari file pengaturan apa pun.

3. Default bawaan

Percakapan yang dimulai ekstensi VS Code mengikuti daftar ekstensi sendiri dalam [Alihkan mode izin](#switch-permission-modes). Untuk mode izin yang dimulai Claude Code dalam sesi yang dilanjutkan, lihat [mode izin pada resume](/docs/id/sessions#permission-mode-on-resume).

Default `auto` bawaan memerlukan Claude Code v2.1.228 atau lebih baru di macOS, Linux, dan WSL, dan v2.1.233 atau lebih baru di Windows asli. Pada versi sebelumnya, default bawaan adalah Manual.

Default bawaan bergantung pada cara Anda menjalankan Claude Code, pada paket Anda, dan pada apakah Claude Code dapat mengambil flag fiturnya. Baris pertama yang cocok dengan sesi Anda berlaku. Tabel mencakup sesi yang Anda mulai di terminal atau melalui ekstensi VS Code; untuk aplikasi desktop dan claude.ai, lihat tab Desktop dan Web dalam [Alihkan mode izin](#switch-permission-modes).

| Cara Anda menjalankan Claude Code                                                                                                                                                                                                          | Mode izin awal bawaan |
| :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :-------------------- |
| File pengaturan apa pun menetapkan `disableAutoMode` ke `"disable"`                                                                                                                                                                        | `default`             |
| [Pengambilan flag fitur](/docs/id/env-vars#features-that-need-feature-flag-fetching) mati                                                                                                                                                       | `default`             |
| [Sesi pertama Anda setelah Anda memasang Claude Code atau upgrade](/docs/id/env-vars#first-session-after-an-install-or-upgrade) ke versi yang menambahkan default ini, kecuali, setelah instalasi segar, Claude Code mengambil flag tepat waktu | `default`             |
| `claude -p` atau [Agent SDK](/docs/id/agent-sdk/permissions)                                                                                                                                                                                    | `default`             |
| Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry, [Claude Platform on AWS](/docs/id/claude-platform-on-aws), atau sesi [Claude apps gateway](/docs/id/claude-apps-gateway) yang masuk                                                | `default`             |
| Paket Pro, Max, atau Team, di terminal atau melalui [ekstensi VS Code](/docs/id/vs-code)                                                                                                                                                        | `auto`                |
| Paket Enterprise atau kunci API Claude Console                                                                                                                                                                                             | `default`             |

Ketika pengambilan flag fitur mati, atau dalam [sesi pertama setelah instalasi atau upgrade](/docs/id/env-vars#first-session-after-an-install-or-upgrade) di mana flag belum tiba, ekstensi VS Code mengabaikan setiap file pengaturan saat memilih mode izin awal.

Ketika flag, file pengaturan, atau default bawaan memilih `auto` tetapi mode auto tidak tersedia untuk sesi, Claude Code memulai sesi dalam Manual sebagai gantinya. Mode auto tidak tersedia ketika sesi tidak memenuhi [persyaratan ketersediaan](#eliminate-prompts-with-auto-mode), seperti file pengaturan mematikannya atau model yang tidak mendukungnya, atau ketika Anthropic telah mematikannya sisi server.

Pertama kali default bawaan memulai salah satu sesi Anda dalam mode auto, Claude Code menampilkan pemberitahuan yang menghubungkan ke halaman ini:

* Di terminal, sekali, di bagian atas sesi
* Di ekstensi VS Code, sebagai kartu di layar percakapan baru yang tetap sampai Anda menutupnya

Pada paket Pro, Max, dan Team, jika `~/.claude/settings.json` Anda menetapkan `defaultMode` selain `auto` dan tidak ada file pengaturan lain yang menetapkan satu, sesi Anda terus dimulai dalam mode itu. Claude Code meminta sekali, di terminal atau di ekstensi VS Code, apakah akan mengubah pengaturan ke mode auto. Jika Anda menolak, pengaturan Anda tetap seperti adanya.

<h3 id="start-in-a-different-mode">
  Mulai dalam mode izin yang berbeda
</h3>

Anda dapat menetapkan mode izin awal untuk satu sesi, atau sebagai default untuk setiap sesi di mesin, proyek, atau organisasi. Ketika lebih dari satu file pengaturan menetapkan `permissions.defaultMode`, [preseden pengaturan](/docs/id/settings#settings-precedence) memutuskan, jadi nilai proyek atau terkelola mengalahkan `~/.claude/settings.json`. Untuk mengubah mode izin sesi yang sudah berjalan, lihat [Alihkan mode izin](#switch-permission-modes).

| Untuk menetapkan mode izin awal untuk                  | Lakukan ini                                                                                                                                                                                                                                                                                                                                                                                             |
| :----------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Satu sesi yang akan Anda mulai                         | Lewatkan mode izin sebagai flag, misalnya `claude --permission-mode default`                                                                                                                                                                                                                                                                                                                            |
| Setiap sesi terminal yang Anda mulai di mesin ini      | Atur `permissions.defaultMode` di `~/.claude/settings.json`. Untuk apa yang dibaca ekstensi VS Code, lihat [Alihkan mode izin](#switch-permission-modes)                                                                                                                                                                                                                                                |
| Setiap sesi terminal yang Anda mulai dalam satu proyek | Atur `permissions.defaultMode` di `.claude/settings.json` proyek. Sesi yang Anda mulai di terminal menghormati setiap nilai kecuali `auto` dan `bypassPermissions`; sesi yang dimulai ekstensi VS Code tidak membaca pengaturan proyek untuk mode izin awal                                                                                                                                             |
| Setiap sesi terminal di organisasi Anda                | Atur `permissions.defaultMode` dalam [pengaturan terkelola](/docs/id/managed-settings). Sesi terminal dimulai dalam mode itu dan orang masih dapat beralih ke mode auto; untuk apa yang dibaca ekstensi VS Code, lihat [Alihkan mode izin](#switch-permission-modes). Untuk menghapus mode auto sehingga tidak ada yang dapat memilihnya, atur `permissions.disableAutoMode` ke `"disable"` sebagai gantinya |

Contoh ini membuat setiap sesi terminal di mesin Anda dimulai dalam mode Manual, yang nilai konfignya adalah `default`. Simpan di `~/.claude/settings.json`:

```json theme={null}
{
  "permissions": {
    "defaultMode": "default"
  }
}
```

Sesi berikutnya yang Anda mulai menampilkan `⏸ manual mode on` di bilah status.

<h2 id="switch-permission-modes">
  Alihkan mode izin
</h2>

Setiap antarmuka memiliki kontrol sendiri untuk beralih mode izin selama sesi dan cara sendiri untuk memilih mode izin yang dimulai sesi baru. Pilih antarmuka Anda untuk melihat kontrolnya.

<Tabs>
  <Tab title="CLI">
    **Selama sesi**: tekan `Shift+Tab` untuk siklus mode izin. Dari `auto`, tekan pertama beralih ke `default`, dan siklus kemudian berjalan `default` → `acceptEdits` → `plan` → kembali ke `default`. Mode opsional, dijelaskan di bawah, masuk setelah `plan`. Bilah status menunjukkan mode aktif sebagai `⏸ manual mode on` abu-abu untuk `default`, atau sebagai `⏵⏵ accept edits on`, `⏸ plan mode on`, `⏵⏵ auto mode on`, `⏵⏵ don't ask on`, atau `⏵⏵ bypass permissions on`.

    Tidak setiap mode ada dalam siklus default:

    * `auto`: muncul ketika [mode auto tersedia](#eliminate-prompts-with-auto-mode); bersiklus ke dalamnya mengganti mode izin tanpa prompt konfirmasi
    * `bypassPermissions`: muncul setelah Anda memulai dengan `--permission-mode bypassPermissions`, `--dangerously-skip-permissions`, `--allow-dangerously-skip-permissions`, atau `permissions.defaultMode: "bypassPermissions"` dalam [pengaturan pengguna, `--settings`, atau terkelola](/docs/id/settings-reference#permissions-defaultmode). Varian `--allow-` menambahkan mode izin ke siklus tanpa mengaktifkannya
    * `dontAsk`: tidak pernah muncul dalam siklus; atur dengan `--permission-mode dontAsk`

    Mode opsional yang diaktifkan masuk setelah `plan`, dengan `bypassPermissions` terlebih dahulu dan `auto` terakhir. Jika Anda memiliki keduanya diaktifkan, Anda akan bersiklus melalui `bypassPermissions` dalam perjalanan ke `auto`.

    **Dari prompt izin Bash**: dalam mode izin Manual dan `acceptEdits`, ketika [mode auto](#eliminate-prompts-with-auto-mode) tersedia, Claude Code menambahkan **Ya, dan beralih ke mode auto** ke prompt izin perintah Bash. Pilih untuk menyetujui perintah dan beralih sesi ke mode auto. Prompt [alat PowerShell](/docs/id/tools-reference#powershell-tool) tidak menawarkan opsi. Memerlukan Claude Code v2.1.247 atau lebih baru.

    Claude Code tidak menambahkan opsi ke prompt yang dipaksa oleh salah satu [aturan `ask`](/docs/id/permissions#manage-permissions) Anda atau oleh [hook](/docs/id/hooks#pretooluse-decision-control), karena mode auto masih menampilkan prompt itu, jadi beralih tidak akan menghapusnya.

    **Saat startup**: lewatkan mode izin sebagai flag.

    ```bash theme={null}
    claude --permission-mode plan
    ```

    **Sebagai default**: atur `permissions.defaultMode` pada cakupan yang Anda inginkan, seperti dijelaskan dalam [Mulai dalam mode izin yang berbeda](#start-in-a-different-mode).

    Flag `--permission-mode` yang sama berfungsi dengan `-p` untuk [jalankan non-interaktif](/docs/id/headless).
  </Tab>

  <Tab title="VS Code">
    **Selama sesi**: klik indikator mode di bagian bawah kotak prompt. Ini menggunakan label ini untuk mode di halaman ini:

    | Label UI             | Mode                |
    | :------------------- | :------------------ |
    | Manual               | `default`           |
    | Edit secara otomatis | `acceptEdits`       |
    | Plan                 | `plan`              |
    | Auto                 | `auto`              |
    | Bypass permissions   | `bypassPermissions` |

    **Sebagai default**: untuk menyematkan mode izin yang dimulai percakapan, atur `claudeCode.initialPermissionMode` dalam pengaturan pengguna VS Code Anda ke `default`, `manual`, `acceptEdits`, `plan`, atau `bypassPermissions`. Pengaturan tidak menerima `auto`; untuk memulai dalam Auto, biarkan tidak diatur dan pilih **Auto** dari indikator mode sekali, sebagai item 2 di bawah menjelaskan. Ekstensi memulai setiap percakapan baru dalam yang pertama dari ini yang berlaku:

    1. `claudeCode.initialPermissionMode`
    2. Mode yang terakhir Anda pilih dari indikator mode, jika itu Manual, Edit secara otomatis, atau Auto. Memilih Plan atau Bypass permissions berlaku hanya untuk percakapan itu
    3. `permissions.defaultMode` dari [pengaturan terkelola](/docs/id/managed-settings) atau `~/.claude/settings.json`, pada paket Pro, Max, dan Team dengan [pengambilan flag fitur](#which-mode-a-session-starts-in) tersedia
    4. [Default bawaan](#which-mode-a-session-starts-in) untuk paket, penyedia, dan pengaturan organisasi Anda

    Ekstensi tidak pernah membaca `.claude/settings.json` atau `.claude/settings.local.json` proyek untuk mode izin awal, dan dalam percakapan yang tidak memenuhi kondisi item 3 itu tidak membaca file pengaturan sama sekali. Ketika `claudeCode.claudeProcessWrapper` diatur, item 3 dan 4 juga tidak berlaku: percakapan itu dimulai dalam Manual kecuali item 1 atau item 2 menetapkan mode izin.

    Auto muncul di indikator mode ketika [mode auto tersedia](#eliminate-prompts-with-auto-mode).

    Bypass permissions memerlukan toggle **Allow dangerously skip permissions** dalam pengaturan ekstensi. Tanpanya, mode izin tidak muncul di indikator, dan nilai `bypassPermissions` dari item 1 atau item 3 memulai percakapan dalam Manual sebagai gantinya. Auto dari item apa pun demikian pula memulai percakapan dalam Manual ketika mode auto tidak tersedia.

    Lihat [panduan VS Code](/docs/id/vs-code) untuk detail khusus ekstensi.
  </Tab>

  <Tab title="JetBrains">
    Plugin JetBrains menjalankan Claude Code di terminal IDE, jadi beralih mode izin berfungsi sama seperti di CLI: tekan `Shift+Tab` untuk bersiklus, atau lewatkan `--permission-mode` saat meluncurkan.
  </Tab>

  <Tab title="Desktop">
    **Selama sesi**: di tab Code, gunakan pemilih mode di sebelah tombol kirim. Tidak setiap mode muncul di pemilih:

    * **Auto**: muncul ketika [mode auto tersedia](#eliminate-prompts-with-auto-mode)
    * **Bypass permissions**: memerlukan toggle **Allow bypass permissions mode** dalam pengaturan Desktop pada paket Pro dan Max; pada paket Team dan Enterprise, kebijakan organisasi mengontrolnya sebagai gantinya

    Tab Cowork tidak menggunakan mode ini. Cowork memiliki mode izinnya sendiri, diaktifkan secara terpisah, dan tab Cowork tidak menampilkan pemilih mode sama sekali sampai mode di luar defaultnya diaktifkan untuk akun Anda. Lihat [dokumen Cowork](https://claude.com/docs/cowork/overview).

    Untuk detail khusus desktop, lihat [Pilih mode izin](/docs/id/desktop#choose-a-permission-mode) dalam panduan Desktop.

    **Sebagai default**: atur `defaultMode` dalam [pengaturan](/docs/id/settings#where-settings-live). Aplikasi desktop membaca file pengaturan yang sama seperti CLI dan menerapkan mode izin ke sesi lokal baru.

    Mode yang Anda pilih di pemilih mode diingat per folder dan mengambil alih `defaultMode` untuk folder itu. Plan adalah pengecualian: memilihnya berlaku hanya untuk sesi saat ini.

    Untuk di mana `defaultMode` masuk dalam file pengaturan, lihat contoh di bawah [Mulai dalam mode izin yang berbeda](#start-in-a-different-mode).
  </Tab>

  <Tab title="Web dan mobile">
    Gunakan dropdown mode di sebelah kotak prompt di [claude.ai/code](https://claude.ai/code) atau di aplikasi mobile. Prompt izin muncul di claude.ai untuk persetujuan. Mode mana yang muncul bergantung pada di mana sesi berjalan:

    * **[Sesi cloud](/docs/id/claude-code-on-the-web)**: Accept edits, Plan, dan Auto. Accept edits sesuai dengan mode `default`: sesi cloud pra-menyetujui pengeditan file terlepas dari mode, jadi dropdown menampilkan Accept edits alih-alih Manual. Sesi cloud masih menghormati `defaultMode: "acceptEdits"` dari pengaturan. Mode Auto muncul hanya ketika organisasi Anda mengizinkannya dan model yang dipilih mendukungnya. Bypass permissions tidak tersedia.
    * **[Sesi Remote Control](/docs/id/remote-control)** di mesin lokal Anda: Manual, Accept edits, dan Plan. Anda tidak dapat memilih Auto atau Bypass permissions dari aplikasi.
      * Kecuali untuk Bypass permissions, dropdown menampilkan mode izin yang sesi lokal gunakan, termasuk yang diatur dari terminal. Itu diperbarui ketika mode izin berubah di aplikasi atau di terminal. Sesi tidak pernah melaporkan Bypass permissions ke claude.ai, jadi beralih ke dalamnya dari terminal tidak mengubah apa yang ditampilkan dropdown.
      * Sesi yang dihosting oleh [aplikasi desktop](/docs/id/desktop) atau [ekstensi VS Code](/docs/id/vs-code) melaporkan perubahan mode izin ke claude.ai saat terjadi, sama seperti sesi yang dihosting di terminal.
      * Sebelum v2.1.202, sesi yang terhubung dengan `/remote-control` atau `claude --remote-control` tidak melaporkan mode mereka sama sekali, jadi claude.ai dan aplikasi mobile dapat menampilkan mode yang sesi tidak gunakan. Ketidaksesuaian hanya mempengaruhi label. Claude Code menghasilkan prompt izin dari mode izin aktual sesi, dan mereka masih muncul di aplikasi untuk persetujuan.

    Untuk Remote Control, mesin lokal yang menjalankan sesi harus masuk dengan akun claude.ai Anda; kunci API tidak didukung. Anda juga dapat menetapkan mode izin awal saat meluncurkan sesi lokal itu:

    ```bash theme={null}
    claude remote-control --permission-mode acceptEdits
    ```
  </Tab>
</Tabs>

<h2 id="auto-approve-file-edits-with-acceptedits-mode">
  Auto-approve pengeditan file dengan mode acceptEdits
</h2>

Mode `acceptEdits` memungkinkan Claude membuat dan mengedit file di direktori kerja Anda tanpa meminta. Bilah status menunjukkan `⏵⏵ accept edits on` saat mode ini aktif.

Selain pengeditan file, mode `acceptEdits` auto-approve perintah Bash filesystem umum: `mkdir`, `touch`, `rm`, `rmdir`, `mv`, `cp`, dan `sed`. Perintah ini juga auto-approved ketika diawali dengan variabel lingkungan aman seperti `LANG=C` atau `NO_COLOR=1`, atau pembungkus proses seperti `timeout`, `nice`, atau `nohup`. Seperti pengeditan file, auto-approval hanya berlaku untuk jalur di dalam direktori kerja Anda atau `additionalDirectories`. Jalur di luar cakupan itu, penulisan ke [jalur yang dilindungi](#protected-paths), penghapusan `rm` dan `rmdir` yang menargetkan [jalur kritis](#critical-paths), dan semua perintah Bash lainnya kecuali [set bawaan read-only](/docs/id/permissions#read-only-commands) masih meminta.

Ketika [alat PowerShell](/docs/id/tools-reference#powershell-tool) diaktifkan, mode `acceptEdits` juga auto-approve `Set-Content`, `Add-Content`, `Clear-Content`, dan `Remove-Item` pada jalur dalam cakupan, bersama dengan alias umum mereka. Aturan cakupan dan jalur yang dilindungi yang sama berlaku, dan `Remove-Item` mendapat [pemeriksaannya sendiri](#remove-item-in-powershell). Argumen posisional yang berisi karakter kutip, seperti apostrof dalam `Set-Content .\notes.txt "It's done"`, masih meminta bahkan pada jalur dalam cakupan, karena Claude Code tidak dapat secara statis memvalidasi argumen yang pembacaan kutip dan tidak dikutipnya berbeda. Lewatkan konten melalui parameter bernama seperti `-Value` untuk menghindari prompt.

Gunakan `acceptEdits` ketika Anda ingin meninjau perubahan di editor Anda atau melalui `git diff` setelahnya daripada menyetujui setiap pengeditan inline.

Tekan `Shift+Tab` sekali dari mode Manual untuk memasukkannya, atau mulai dengannya langsung:

```bash theme={null}
claude --permission-mode acceptEdits
```

<h2 id="analyze-before-you-edit-with-plan-mode">
  Analisis sebelum Anda mengedit dengan mode rencana
</h2>

Plan Mode memberi tahu Claude untuk meneliti dan mengusulkan perubahan tanpa membuatnya. Claude membaca file, menjalankan perintah shell untuk menjelajahi, dan menulis rencana, tetapi tidak mengedit sumber Anda. Kecuali dalam sesi terminal interaktif dengan [bypass permissions tersedia](#skip-all-checks-with-bypasspermissions-mode), pengeditan tetap diblokir sampai Anda menyetujui rencana.

Ketika [auto mode](/docs/id/auto-mode-config) tersedia dan pengaturan `useAutoModeDuringPlan` aktif, yang merupakan default, pengklasifikasi meninjau perintah shell selama perencanaan alih-alih meminta Anda. Perintah yang disetujui berjalan, dan yang ditolak diblokir. Jika tidak, perintah di luar [set bawaan read-only](/docs/id/permissions#read-only-commands) meminta persetujuan, termasuk ketika [auto-allow mode](/docs/id/sandboxing#sandbox-modes) sandbox diaktifkan. Dalam sesi terminal interaktif dengan bypass permissions tersedia, baik pengklasifikasi maupun prompt tidak berlaku untuk perintah perencanaan; [Lewati semua pemeriksaan dengan bypassPermissions mode](#skip-all-checks-with-bypasspermissions-mode) mencakup beberapa hal yang masih meminta di sana. Dalam v2.1.212 hingga v2.1.217, sesi tanpa bypass permissions meminta untuk setiap perintah di luar set read-only, apakah atau tidak auto mode tersedia.

Masukkan mode rencana dengan menekan `Shift+Tab` atau mengawali prompt tunggal dengan `/plan`. Anda juga dapat memulai dalam mode rencana dari CLI:

```bash theme={null}
claude --permission-mode plan
```

Tekan `Shift+Tab` lagi untuk meninggalkan mode rencana tanpa menyetujui rencana.

<h3 id="review-and-approve-a-plan">
  Tinjau dan setujui rencana
</h3>

Ketika rencana siap, Claude menyajikannya dan menanyakan cara melanjutkan. Dari prompt itu Anda dapat memilih:

* **Ya, dan gunakan mode auto**: setujui dan mulai dalam [mode auto](#eliminate-prompts-with-auto-mode). Jika mode auto tidak [tersedia untuk sesi Anda](#eliminate-prompts-with-auto-mode), misalnya karena organisasi Anda mematikannya, opsi ini berbunyi **Ya, auto-accept edits**. Jika Anda memulai sesi dengan bypass permissions diaktifkan, opsi berbunyi **Ya, dan beralih ke BYPASS PERMISSIONS (tidak ada prompt lebih lanjut) untuk sesi ini** sebagai gantinya.
* **Ya, secara manual setujui pengeditan**: setujui dan tinjau setiap pengeditan secara individual.
* **Tidak, terus merencanakan**: tetap dalam mode rencana dan beri tahu Claude apa yang harus diubah.

Menyetujui rencana keluar dari mode rencana dan mengalihkan sesi ke mode izin yang dijelaskan setiap opsi persetujuan, sehingga Claude mulai mengedit. Untuk merencanakan lagi, siklus kembali ke mode rencana dengan `Shift+Tab`, atau awali prompt berikutnya dengan `/plan`.

Tekan `Ctrl+G` untuk membuka rencana yang diusulkan di editor teks default Anda dan mengeditnya langsung sebelum Claude melanjutkan. Ketika [`showClearContextOnPlanAccept`](/docs/id/settings-reference#showclearcontextonplanaccept) diaktifkan, daftar mendapat opsi pertama yang menyetujui rencana dan menghapus konteks perencanaan.

Menerima rencana juga memberi sesi [judul yang dihasilkan](/docs/id/sessions#name-your-sessions) berdasarkan rencana, kecuali Anda telah menamai sesi.

<h3 id="set-plan-mode-as-the-default">
  Atur mode rencana sebagai default
</h3>

Untuk membuat mode rencana default untuk sesi terminal proyek, atur `defaultMode` ke `plan` dalam `.claude/settings.json`, ditempatkan seperti contoh di bawah [Mulai dalam mode izin yang berbeda](#start-in-a-different-mode) menunjukkan. Percakapan yang dimulai [ekstensi VS Code](/docs/id/vs-code) tidak membaca pengaturan proyek untuk mode izin awal. Di sana, atur `claudeCode.initialPermissionMode` ke `plan` dalam pengaturan pengguna VS Code Anda sebagai gantinya.

<h2 id="eliminate-prompts-with-auto-mode">
  Hilangkan permintaan izin dengan mode otomatis
</h2>

Mode otomatis memungkinkan Claude untuk dieksekusi tanpa permintaan izin rutin. Model pengklasifikasi terpisah meninjau tindakan sebelum dijalankan, memblokir apa pun yang melampaui permintaan Anda, menargetkan infrastruktur yang tidak dikenali, atau tampak didorong oleh konten bermusuhan yang dibaca Claude. [Aturan ask](/docs/id/permissions#manage-permissions) eksplisit masih memaksa permintaan.

Pada paket Pro, Max, dan Team, mode otomatis adalah [mode izin awal bawaan](#which-mode-a-session-starts-in).

Pengklasifikasi juga meninjau setiap pesan yang dikirim Claude ke agen lain dengan [`SendMessage`](/docs/id/tools-reference), baik teks biasa atau pesan [tim agen](/docs/id/agent-teams) terstruktur, sebelum Claude Code mengirimkannya, baik dalam mode otomatis maupun dalam [mode rencana saat pengklasifikasi meninjau perintah](#analyze-before-you-edit-with-plan-mode); tinjauan pengiriman memerlukan Claude Code v2.1.222 atau lebih baru.

Pengklasifikasi juga meninjau dan menyetujui atau memblokir penghapusan `rm` dan `rmdir` yang menargetkan [jalur kritis](#critical-paths), seperti `rm -rf /` dan `rm -rf ~`, termasuk ketika penghapusan berada di dalam substitusi perintah atau proses.

Mode otomatis juga mendorong Claude untuk terus bekerja tanpa berhenti untuk pertanyaan klarifikasi, meskipun Claude masih bertanya ketika permintaan Anda atau keterampilan secara eksplisit bergantung padanya. Untuk perilaku otonom yang lebih kuat dalam mode yang masih meminta Anda, atur [gaya output Proaktif](/docs/id/output-styles) sebagai gantinya.

<Warning>
  Mode otomatis mengurangi permintaan izin tetapi tidak menjamin keamanan. Gunakan untuk tugas di mana Anda mempercayai arah umum, bukan sebagai pengganti tinjauan pada operasi sensitif.
</Warning>

Mode otomatis tersedia hanya ketika akun Anda memenuhi semua persyaratan ini:

* **Paket**: Semua paket.
* **Organisasi**: pada Team dan Enterprise, mode otomatis tersedia secara default. Administrator dapat mematikannya untuk organisasi dengan menetapkan `permissions.disableAutoMode` ke `"disable"` dalam [pengaturan terkelola](/docs/id/managed-settings).
* **Model**: pada Anthropic API dan [Claude Platform di AWS](/docs/id/claude-platform-on-aws), Claude Opus 4.6 atau lebih baru, Sonnet 4.6 atau lebih baru, atau [model Fable](/docs/id/model-config#work-with-fable). Di Amazon Bedrock, Agent Platform Google Cloud, Microsoft Foundry, dan sesi [gateway aplikasi Claude](/docs/id/claude-apps-gateway) yang masuk, hanya Claude Sonnet 5, Opus 4.7 atau lebih baru, dan model Fable. Model yang lebih lama, termasuk Sonnet 4.5, Opus 4.5, Haiku, dan model claude-3, tidak didukung di penyedia mana pun.
* **Penyedia**: tersedia secara default pada Anthropic API, Claude Platform di AWS, Amazon Bedrock, Agent Platform Google Cloud, Microsoft Foundry, dan sesi gateway aplikasi Claude yang masuk.

Jika Claude Code melaporkan mode otomatis tidak tersedia, pertama periksa persyaratan ini dan apakah file pengaturan apa pun menetapkan [`disableAutoMode`](/docs/id/settings-reference#disableautomode). Anthropic juga dapat mematikan mode otomatis di sisi server, atau server dapat menolak mode otomatis untuk akun Anda. Sesi yang menerima salah satu jawaban menjaga mode otomatis tetap mati sampai sesi berakhir, jadi mulai sesi baru nanti.

Pesan terpisah yang menyebutkan model dan mengatakan mode otomatis "tidak dapat menentukan keamanan" tindakan berarti permintaan pengklasifikasi gagal. Kegagalan itu biasanya bersifat sementara, tetapi di Amazon Bedrock dapat berulang sampai akun Anda dapat memanggil model yang disebutkan. Lihat [referensi kesalahan](/docs/id/errors#auto-mode-cannot-determine-the-safety-of-an-action) untuk penyebab dan apa yang harus dilakukan.

Jika Anda menetapkan `defaultMode: "auto"` dalam [pengaturan](/docs/id/settings-reference#all-settings) dan sesi terminal dimulai dalam mode Manual tanpa kesalahan, pengaturan kemungkinan berada di `.claude/settings.json` atau `.claude/settings.local.json`. `auto` tidak berlaku dari file tersebut. Pindahkan ke `~/.claude/settings.json`. Untuk percakapan yang dimulai ekstensi VS Code, periksa daftar ekstensi sendiri dalam [Beralih mode izin](#switch-permission-modes) sebagai gantinya.

<h3 id="enable-auto-mode-on-bedrock-agent-platform-or-foundry">
  Mode otomatis di Bedrock, Agent Platform, atau Foundry
</h3>

Di [Amazon Bedrock](/docs/id/amazon-bedrock), [Agent Platform Google Cloud](/docs/id/google-vertex-ai), [Microsoft Foundry](/docs/id/microsoft-foundry), dan sesi [gateway aplikasi Claude](/docs/id/claude-apps-gateway) yang masuk, mode otomatis muncul dalam siklus `Shift+Tab` secara default. Muncul dalam siklus tidak mengubah mode izin yang dimulai sesi: di penyedia ini, sesi terminal dimulai dalam [`defaultMode`](/docs/id/settings-reference#permissions-defaultmode) Anda, yang Manual kecuali Anda mengubahnya, dan percakapan dalam [ekstensi VS Code](/docs/id/vs-code) dimulai dalam Manual kecuali `claudeCode.initialPermissionMode` atau mode yang Anda pilih dalam ekstensi menetapkan satu. Hanya Claude Sonnet 5, Opus 4.7 atau lebih baru, dan model Fable yang didukung di penyedia ini.

Untuk membuat mode otomatis mode izin awal default, atur `"permissions": {"defaultMode": "auto"}` dalam pengaturan pengguna atau terkelola. Dalam sesi yang dimulai ekstensi VS Code, pilih **Auto** dari indikator mode sebagai gantinya. [Beralih mode izin](#switch-permission-modes) mencakup apa yang mengalahkan pilihan itu.

Pemeriksaan [`/doctor`](/docs/id/commands#all-commands) menyarankan default pengaturan pengguna ini di penyedia ini dengan cara yang sama seperti pada Anthropic API.

Untuk mencegah pengembang menggunakan mode otomatis, atur `disableAutoMode` ke `"disable"` dalam [pengaturan terkelola](/docs/id/managed-settings). Ini menghapus `auto` dari siklus `Shift+Tab`, dan sesi yang dimulai dengan `--permission-mode auto` dimulai dalam Manual sebagai gantinya. Sesi yang sudah berjalan dalam mode otomatis meninggalkannya ketika pengaturan mencapai sesi itu dari [sumber yang diterapkan admin](/docs/id/managed-settings#which-managed-source-claude-code-uses), dan menampilkan `auto mode disabled by settings`. Sebelum v2.1.251, sesi yang berjalan menjaga mode otomatis sampai berakhir.

Dalam v2.1.158 hingga v2.1.206, mode otomatis mati di penyedia ini sampai Anda menetapkan `CLAUDE_CODE_ENABLE_AUTO_MODE=1`, dan Claude Code mengabaikan `defaultMode: "auto"` di penyedia ini kecuali variabel juga ditetapkan. Variabel masih diterima untuk kompatibilitas dan tidak berpengaruh dari v2.1.207 ke depan.

<h3 id="server-side-classifier-review">
  Tinjauan pengklasifikasi sisi server
</h3>

Dalam mode otomatis, Claude Code dapat meminta server untuk memeriksa tindakan yang [urutan keputusan](#how-the-classifier-evaluates-actions) mengirim untuk tinjauan, sebagai bagian dari permintaan model sesi, sebagai pengganti mengirim permintaan pengklasifikasi sendiri. Sesi ini bertanya:

* **Koneksi langsung ke Anthropic API**: dalam sesi terminal interaktif, pada setiap paket claude.ai dan pada akun yang menggunakan Claude API, saat Anthropic meluncurkannya. Memerlukan Claude Code v2.1.271 atau lebih baru pada paket Pro, Max, dan Team, dan v2.1.278 atau lebih baru pada paket Enterprise dan akun Claude API. Dari v2.1.282, sesi yang [tidak mengambil bendera fitur](/docs/id/env-vars#features-that-need-feature-flag-fetching), misalnya karena Anda mematikan telemetri, meminta server secara default dalam sesi apa pun.
* **Penyedia cloud, atau gateway LLM atau proxy**: di [Claude Platform di AWS](/docs/id/claude-platform-on-aws), Amazon Bedrock, Agent Platform Google Cloud, dan Microsoft Foundry, dan kapan pun Anda menunjuk `ANTHROPIC_BASE_URL` ke [gateway LLM atau proxy](/docs/id/llm-gateway), apa pun paket Anda. Meminta secara default memerlukan Claude Code v2.1.278 atau lebih baru.
* **Sesi [gateway aplikasi Claude](/docs/id/claude-apps-gateway) yang masuk**: memerlukan Claude Code v2.1.280 atau lebih baru

Di mana server meninjau tindakan, vonis mereka memutuskannya. Dua hasil lain dimungkinkan:

* **Server tidak meninjau sesi**: respons selesai tanpa hasil tinjauan, atau server menjawab bahwa itu tidak meninjau sesi ini. Penyebab paling umum adalah gateway LLM atau proxy yang menghilangkan permintaan untuk tinjauan atau hasilnya, dan platform, wilayah, atau kredensial yang belum memiliki pemeriksaan sisi server. Claude Code kembali ke permintaan pengklasifikasi sendiri. Setelah fallback itu berlaku untuk sisa sesi, itu menunjukkan [pemberitahuan tentang biaya permintaan pengklasifikasi](/docs/id/auto-mode-classifier-billing) pada akun di mana permintaan tersebut ditagih.
* **Server tidak memberikan vonis untuk tindakan**: Claude Code menolak tindakan daripada menjalankannya tanpa tinjauan. Pada koneksi apa pun, ini terjadi ketika respons berakhir sebelum hasil tinjauan tiba atau hasil tiba dalam bentuk yang tidak dapat dibaca Claude Code. Gateway LLM atau proxy yang memotong respons pendek atau menulis ulang hasilnya dapat menyebabkan salah satu. Pada koneksi langsung ke Anthropic API, itu juga terjadi ketika pemeriksaan server gagal untuk tindakan, misalnya dengan waktu habis. [Server tidak mengembalikan vonis keamanan](/docs/id/errors#the-server-returned-no-safety-verdict) mencakup pesan penolakan, apa yang terjadi ketika penolakan berulang, dan apa yang harus dilakukan.

Untuk melewati permintaan ke server dan selalu menggunakan permintaan pengklasifikasi Claude Code sendiri, atur [`CLAUDE_CODE_AUTO_MODE_SERVER=0`](/docs/id/env-vars). Pada koneksi langsung ke Anthropic API, variabel memerlukan Claude Code v2.1.281 atau lebih baru. Menetapkannya ke `1` di sana mengaktifkan tinjauan server dalam sesi yang belum memilikinya, seperti sesi `-p` atau Agent SDK, kecuali Anda juga menetapkan `CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1`. Jika Anda menetapkan `CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1` dan membiarkan `CLAUDE_CODE_AUTO_MODE_SERVER` tidak ditetapkan, Claude Code juga berhenti meminta server.

<h3 id="what-the-classifier-blocks-by-default">
  Apa yang diblokir pengklasifikasi secara default
</h3>

Pengklasifikasi mempercayai direktori kerja Anda dan remote yang dikonfigurasi untuknya ketika sesi dimulai. Remote yang ditambahkan atau ditunjuk ulang selama sesi dengan `git remote add` atau `git remote set-url` tidak dipercaya, dan semuanya diperlakukan sebagai eksternal sampai Anda [mengonfigurasi infrastruktur terpercaya](/docs/id/auto-mode-config). Sebelum v2.1.200, remote yang ditambahkan pertengahan sesi juga dipercaya.

**Diblokir secara default**:

* Mengunduh dan menjalankan kode, seperti `curl | bash`
* Mengirim data sensitif ke titik akhir eksternal
* Penerapan dan migrasi produksi
* Penghapusan massal pada penyimpanan cloud
* Memberikan izin IAM atau repo
* Memodifikasi infrastruktur bersama
* Menghancurkan file secara tidak dapat dibalikkan yang ada sebelum sesi
* Dorong paksa
* Melakukan komit atau mendorong perubahan yang akan mengirim rahasia atau data sensitif di luar repositori saat dijalankan, atau memperluas apa yang diekspos penerapan. Ini mencakup alur kerja CI atau konfigurasi penerapan yang meneruskan rahasia ke tujuan yang tidak sudah menerimanya, skrip atau langkah penyiapan yang membaca penyimpanan rahasia dan mengirim data keluar, dan perubahan konfigurasi yang memperluas apa yang dipublikasikan penerapan, seperti pengaturan registri, visibilitas, artefak, atau sourcemap. Pemeriksaan berlaku di cabang apa pun, berlaku bahkan ketika repositori bersifat publik, dan terbang ketika perubahan dilakukan komit atau didorong, terlepas dari apakah komit atau dorong itu memicu pipeline; menghapusnya memerlukan penamaan efek eksekusi, bukan hanya komit atau dorong. Sebelum v2.1.211, pemeriksaan ini dibatasi pada cabang default sebagai gantinya: dorong di sana diblokir ketika membawa konten sensitif, perubahan tersembunyi atau salah dideskripsikan relatif terhadap apa yang Anda minta, konten yang diportasi dari luar repositori, atau dirutekan di sekitar tinjauan yang Anda minta
* `git reset --hard`, `git checkout -- .`, `git restore .`, `git clean -fd`, `git stash drop`, atau `git stash clear`, yang pengklasifikasi asumsikan akan membuang perubahan yang tidak dilakukan komit
* `git commit --amend` ketika komit di HEAD tidak dibuat dalam sesi ini
* Dari v2.1.198, `git commit --amend` ketika komit di HEAD sudah didorong. Penulisan ulang pesan saja tidak diblokir: `--amend -m` tanpa apa pun yang baru dipentaskan, pada komit yang dibuat Claude selama sesi ini
* `terraform destroy`, `pulumi destroy`, `cdk destroy`, atau `terragrunt destroy`, dan menerapkan rencana yang menghancurkan sumber daya

Claude Code v2.1.195 dan lebih baru memblokir lebih banyak kategori secara default. Beberapa bergantung pada entri [lingkungan](/docs/id/auto-mode-config#define-trusted-infrastructure), seperti target remote sensitif dan cakupan IaC yang dilindungi, yang dapat Anda sempit ke nama konkret.

* Menulis ke pengelola rahasia, atau mengubah catatan DNS atau sertifikat TLS
* Menggabungkan permintaan tarik yang tidak ada manusia yang menyetujui, menyetujui permintaan tarik Claude sendiri, atau menonaktifkan pemeriksaan CI
* Memposting komentar yang dengan sendirinya adalah perintah untuk otomasi, seperti `atlantis apply` atau `/deploy` atau `/merge` bot
* Mengalihkan, merampingkan, atau menghapus bendera fitur produksi
* Menerapkan perubahan infrastruktur ke cakupan IaC yang dilindungi, atau mengalirkan dan menghapus node kluster
* Penulisan ke kluster komputasi bersama yang melampaui sumber daya yang Anda beri nama, seperti pemilih label atau `--all` yang menangkap pekerjaan pengguna lain
* Membuat sumber daya Kubernetes yang berjalan di setiap node atau mencegat lalu lintas kluster, seperti DaemonSets dan webhook penerimaan
* Shell interaktif atau port-forward ke target remote sensitif
* Membuka terowongan atau shell terbalik yang membuat layanan lokal dapat diakses dari internet publik
* Mencetak kredensial atau token langsung ke transkrip atau file
* Mengakses lokasi yang tercantum sebagai lokasi data sensitif dalam [lingkungan](/docs/id/auto-mode-config#define-trusted-infrastructure) Anda, atau menyalin data keluar dari satu. Sejak v2.1.198 ini juga memblokir pengiriman data dari satu ke audiens yang dikecualikan entri
* Merutekan instalasi paket di sekitar registri paket internal Anda ke registri publik. Sejak v2.1.198, ini juga berlaku ketika Anda telah memberi tahu Claude bahwa registri internal atau cermin ada dalam percakapan, bukan hanya ketika satu tercantum dalam lingkungan Anda
* Menjalankan perintah dengan bendera yang melucuti penjaga keamanan, seperti `--insecure`
* Meluncurkan loop agen otonom yang berjalan tanpa persetujuan manusia atau sandbox, seperti yang dimulai dengan `--dangerously-skip-permissions` atau `--no-sandbox`. Sejak v2.1.198 ini juga mencakup menjalankan agen pihak ketiga atau harness eval dengan isolasi dan persetujuan per-tindakan dinonaktifkan, seperti runner yang dimulai dengan `--yes-always`
* Tindakan browser [Claude di Chrome](/docs/id/chrome) yang dapat mengirim konten halaman, cookie, atau kredensial off-origin

Claude Code v2.1.198 dan lebih baru juga memblokir ini secara default:

* Menghapus file di `/tmp`, `$TMPDIR`, atau direktori goresan atau cache bersama lainnya dengan wildcard, glob, atau filter usia daripada dengan jalur bernama spesifik
* Termasuk detail sensitif dalam konten yang dikirim, diunggah, dipublikasikan, atau ditulis ke orang lain atau sistem bersama, ketika pesan Anda sendiri tidak mengotorisasi detail tersebut untuk penerima itu. Badan PR dan masalah, pesan komit, dan komentar dihitung sebagai jenis konten keluar ini ketika repositori berada di luar batas kepercayaan atau publik, termasuk repositori publik organisasi Anda sendiri; jalur file internal, nama kode, data respons API langsung seperti email atau pengidentifikasi akun, dan pengidentifikasi infrastruktur dihitung sebagai detail sensitif. Cakupan PR, masalah, dan pesan komit memerlukan Claude Code v2.1.200 atau lebih baru. Data pribadi langsung dari respons API dalam badan PR atau masalah, seperti alamat email, pengidentifikasi akun atau organisasi, atau metrik penggunaan, memerlukan Anda untuk menyebutkan detail tersebut dan penerima terlepas dari visibilitas atau batas kepercayaan repositori. Pemeriksaan itu memerlukan Claude Code v2.1.203 atau lebih baru
* Mengirim keystroke ke pane tmux Claude Code sendiri untuk mendorong antarmukanya sendiri, yang pengklasifikasi perlakukan sebagai Claude mengubah izinnya sendiri atau pengawasan

Claude Code v2.1.200 dan lebih baru juga memblokir ini secara default:

* Mengomentari, menghapus, atau memaksa lulus tes atau pernyataan yang menjaga perilaku keamanan, seperti auth, kontrol akses, validasi input, atau sandboxing
* Menghapus atau merobohkan sumber daya stateful yang tidak dibuat Claude dalam sesi, ketika tidak ada aturan penghapusan yang lebih spesifik berlaku dan Anda tidak menyebutkan sumber daya itu
* Menunjuk ulang URL dasar API, titik akhir proxy, penerima webhook, atau cermin registri ke host pihak ketiga yang tidak sesuai dengan tugas, termasuk dalam file contoh seperti `.env.example`
* Mengubah ke mana dorong pergi dengan `git remote set-url` atau `git remote add`, kecuali Anda menyebutkan remote baru
* Mendorong rahasia atau data pribadi atau terpercaya ke repositori yang diketahui publik, atau mendorong materi rahasia di sana yang bukan bagian dari pekerjaan repositori itu sendiri. Materi pelajaran repositori dotfiles sendiri adalah satu-satunya pengecualian untuk data pribadi atau terpercaya, dan konten dari repositori pribadi mencapai permukaan publik apa pun diblokir dengan cara yang sama; kedua penyempurnaan memerlukan Claude Code v2.1.203 atau lebih baru. Sebelum v2.1.203, data pribadi dikelompokkan dengan materi rahasia dan diblokir hanya ketika bukan bagian dari pekerjaan repositori itu sendiri. Ketika visibilitas repositori tidak ditetapkan, pengklasifikasi tidak memblokir itu saja; itu menilai konten terhadap aturan lain sebagai gantinya
* Membuka permintaan tarik terhadap repositori atau organisasi yang berbeda, bercabang dengan `gh repo fork`, atau mendorong ke repositori pihak ketiga, kecuali Anda menyebutkan target eksternal itu

Claude Code v2.1.203 dan lebih baru juga memblokir ini secara default:

* Konten dari penyimpanan lokal sensitif, atau dari file yang nama, jalur, atau jenisnya menandainya sebagai sensitif, memasuki komit, dorong, teks PR atau masalah, gist atau paste, atau publikasi paket, kecuali Anda menyebutkan sumber dan tujuan. Transkrip sesi dan log percakapan, folder dot kredensial dan konfigurasi seperti kunci SSH, kredensial cloud, profil browser, dan riwayat shell, dan ekspor data pengguna semuanya dihitung, dan repositori menjadi pribadi tidak menghapusnya

Claude Code v2.1.205 dan lebih baru juga memblokir ini secara default:

* Menulis ke transkrip sesi Claude Code, file riwayat `.jsonl` di bawah `~/.claude/projects/` atau direktori konfigurasi Anda yang dikonfigurasi, baik secara langsung maupun melalui perintah shell. Aturan juga mencakup baris metadata yang Claude Code tambahkan ke setiap entri transkrip untuk pemeriksaannya sendiri. Membaca transkrip tidak diblokir
* Penghapusan paksa rekursif seperti `rm -rf "$VAR"` atau `Remove-Item -Recurse -Force $dir` yang targetnya adalah variabel shell, atau glob yang berakar pada satu, yang tidak ditugaskan di mana pun dalam percakapan yang dilihat pengklasifikasi. Nilai datang hanya dari output perintah sebelumnya, yang tidak pernah diterima pengklasifikasi, jadi pengklasifikasi tidak dapat memverifikasi target penghapusan terhadap aturan penghapusan lainnya. Blok menghapus ketika Anda menyebutkan jalur pasti yang dihapus, atau ketika Claude menjalankan kembali penghapusan dengan jalur literal yang diselesaikan ditulis ke dalam perintah. Penghapusan yang targetnya dapat diselesaikan pengklasifikasi tidak terpengaruh. Target `Remove-Item` yang merupakan `*` telanjang atau berakhir dalam `/*` atau `\*` tidak pernah mencapai pengklasifikasi: Claude Code [menolaknya langsung](#remove-item-in-powershell)

Claude Code v2.1.257 dan lebih baru juga memblokir ini secara default:

* Meminta kredensial dari titik akhir metadata instans cloud, seperti `169.254.169.254`, atau secara eksplisit mengautentikasi panggilan cloud, kluster, atau registri dengan identitas akun layanan atau node mesin itu sendiri
* Mencapai host publik dengan rute selain permintaan langsung, seperti terowongan, shell terbalik, atau konfigurasi resolver atau proxy yang ditulis ulang untuk menunjuk ke luar
* Membaca kredensial yang dimiliki host daripada tugas Anda, seperti sertifikat node atau auth registri kontainer node
* Menghubungkan ke atau memindai kontainer, pod, atau VM saudara yang tidak dimulai Claude, atau node di bawah kontainer

Jika Claude Code berjalan di tempat yang dimaksudkan untuk memungkinkan salah satu dari ini, jelaskan pengaturan itu dalam entri [Host containment](/docs/id/auto-mode-config#define-trusted-infrastructure) dalam `autoMode.environment`.

Claude Code v2.1.261 dan lebih baru juga memblokir ini secara default:

* Memposting atau menulis tautan ke layanan paste, diagram, atau berbagi data publik dalam pesan, teks PR atau masalah, dokumen, atau di mana pun tautan akan dibuka atau diambil, ketika URL itu sendiri membawa konten yang dibagikan, kecuali Anda menyebutkan layanan itu

**Diizinkan secara default**:

* Operasi file lokal di direktori kerja Anda
* Menginstal dependensi yang dideklarasikan dalam file kunci atau manifes Anda
* Membaca `.env` dan mengirim kredensial ke API yang cocok
* Permintaan HTTP hanya-baca
* Mendorong ke cabang apa pun dari repositori yang Anda kerjakan, termasuk cabang default. Cabang non-default yang nama menandainya sebagai target penerapan atau publikasi, seperti `production` atau `gh-pages`, tidak tercakup: pengklasifikasi menilai dorong di sana berdasarkan istilahnya sendiri. Konten dorong masih diperiksa terhadap aturan lain, [aturan `permissions.deny`](/docs/id/permissions#manage-permissions) masih dapat memblokir perintah dorong [seperti yang ditulis](/docs/id/permissions#bash-rule-limits) dalam setiap mode, dan perlindungan cabang remote itu sendiri masih berlaku. Sebelum v2.1.211, hanya dorong ke cabang yang Anda mulai, cabang yang dibuat Claude, dan dorong rutin ke cabang default yang diizinkan secara default, dan sebelum v2.1.203 dorong langsung apa pun ke cabang default diblokir

Claude Code v2.1.195 dan lebih baru juga memungkinkan ini secara default:

* Menghapus pekerjaan pasti yang dibuat Claude sebelumnya dalam sesi yang sama
* Membaca, meninjau, atau menulis kode, konfigurasi, dan model ancaman terkait keamanan sebagai bagian dari tugas Anda
* Pesan antara agen yang bekerja bersama dalam sesi multi-agen yang sama
* Mengirim data ke domain terpercaya, bucket, dan layanan yang Anda daftarkan dalam [`environment`](/docs/id/auto-mode-config#define-trusted-infrastructure). Ini mencakup aliran data saja, bukan operasi destruktif atau kredensial pada infrastruktur yang sama
* [Claude di Chrome](/docs/id/chrome) navigasi ke domain internal terpercaya, localhost, atau URL yang Anda beri nama

Perintah sandboxed tidak mendapatkan akses jaringan secara default. Claude menyebutkan host yang dibutuhkan perintah pada perintah itu sendiri, pengklasifikasi meninjau mereka dengan perintah, dan daftar yang disetujui membuka host tersebut untuk perintah itu saja. [Domain yang diizinkan per-perintah](/docs/id/sandboxing#per-command-allowed-domains-in-auto-mode) mencakup apa yang dapat dan tidak dapat dibuka daftar dan apa yang terjadi ketika perintah mencapai host yang tidak terdaftar.

Jalankan `claude auto-mode defaults` untuk mencetak daftar aturan lengkap sebagai JSON. Jika tindakan rutin diblokir, administrator dapat menambahkan repo terpercaya, bucket, dan layanan melalui pengaturan `autoMode.environment`: lihat [Konfigurasi mode otomatis](/docs/id/auto-mode-config).

Mendorong ke cabang apa pun dari repositori yang Anda kerjakan dan membuat permintaan tarik yang cocok dengan permintaan Anda berjalan tanpa permintaan, kecuali dorong atau permintaan tarik jatuh di bawah [daftar diblokir](#what-the-classifier-blocks-by-default), seperti rahasia atau data sensitif meninggalkan repositori, atau permintaan tarik yang menargetkan repositori atau organisasi yang berbeda. Untuk memerlukan checkpoint manusia sebelum perintah ini sambil tetap dalam mode otomatis, tambahkan aturan `permissions.ask`, yang cocok dengan perintah [seperti yang ditulis](/docs/id/permissions#bash-rule-limits): lihat [Batas umum](/docs/id/auto-mode-config#common-boundaries).

<h3 id="first-read-outside-the-working-directories">
  Pembacaan pertama di luar direktori kerja
</h3>

Sementara [`permissions.blockReadsOutsideWorkingDirectories`](/docs/id/settings-reference#permissions-blockreadsoutsideworkingdirectories) mati, pembacaan file berjalan tanpa permintaan dalam mode otomatis, termasuk pembacaan di luar [direktori kerja](/docs/id/permissions#working-directories). Pertama kali Claude menggunakan alat Read, Grep, atau Glob pada jalur di luar mereka, Claude Code menanyakan Anda apakah akan terus memungkinkan pembacaan tersebut.

Permintaan tidak muncul dalam `-p` non-interaktif berjalan atau sesi latar belakang; pembacaan di sana berjalan seperti sebelumnya.

Apa pun yang Anda jawab, Claude terus bekerja:

* **Terus izinkan**: pembacaan berjalan, pembacaan kemudian di luar direktori kerja berjalan seperti sebelumnya, dan Claude Code mencatat jawaban Anda sehingga permintaan tidak muncul lagi
* **Blokir mulai sekarang**: pembacaan ditolak, dan Claude Code menetapkan [`permissions.blockReadsOutsideWorkingDirectories`](/docs/id/settings-reference#permissions-blockreadsoutsideworkingdirectories) ke `true` dalam pengaturan pengguna Anda, yang membuat alat file menolak pembacaan tersebut dalam setiap sesi kemudian dan setiap mode izin. Untuk membiarkan Claude membaca jalur tersebut nanti, tambahkan direktorinya dengan `/add-dir` atau hapus pengaturannya.
* **Tanya lagi lain kali**: pembacaan ditolak, dan pembacaan berikutnya di luar direktori kerja meminta lagi

<h3 id="boundaries-you-state-in-conversation">
  Batas yang Anda nyatakan dalam percakapan
</h3>

Pengklasifikasi memperlakukan batas yang Anda nyatakan dalam percakapan sebagai sinyal blok. Jika Anda memberi tahu Claude "jangan dorong" atau "tunggu sampai saya meninjau sebelum menerapkan", pengklasifikasi memblokir tindakan yang cocok bahkan ketika aturan default akan memungkinkannya. Batas tetap berlaku sampai Anda mengangkatnya dalam pesan kemudian. Penilaian Claude sendiri bahwa kondisi terpenuhi tidak mengangkatnya.

Batas tidak disimpan sebagai aturan. Pengklasifikasi membaca ulang mereka dari transkrip pada setiap pemeriksaan, jadi batas dapat hilang jika [pemadatan konteks](/docs/id/costs#reduce-token-usage) menghapus pesan yang menyatakannya. Untuk jaminan keras, tambahkan [aturan deny](/docs/id/permissions#permission-rule-syntax) sebagai gantinya.

<h3 id="approvals-you-state-in-conversation">
  Persetujuan yang Anda nyatakan dalam percakapan
</h3>

Jika Anda memberi tahu Claude bahwa tindakan yang diblokir diizinkan, pengklasifikasi membaca itu sebagai persetujuan Anda dan dapat menghapus blok. Cara Anda merumuskannya memutuskan apakah tindakan berjalan, dan seberapa jauh persetujuan mencapai:

* **Beri nama tindakan dan spesifikasinya**: pesan Anda harus menyebutkan tindakan dan hal spesifik yang membuatnya berbahaya, seperti cabang dorong paksa. Menyebutkan verba saja tidak menghapus apa pun, jadi "Anda dapat dorong paksa" membiarkan blok tetap ada.
* **Harapkan itu mencakup satu tindakan**: persetujuan mencakup tindakan destruktif yang Anda beri nama, jadi tindakan kemudian diblokir lagi kecuali Anda memberikan persetujuan sebagai berdiri. Untuk berhenti menyetujui pola rutin satu tindakan pada satu waktu, tambahkan ke [`autoMode.allow`](/docs/id/auto-mode-config#override-the-block-and-allow-rules).
* **Beberapa blok tetap ada**: [urutan preseden pengklasifikasi](/docs/id/auto-mode-config#override-the-block-and-allow-rules) menetapkan blok mana yang dapat dicapai persetujuan Anda. Untuk menjalankan langkah yang tidak akan dihapusnya, [tinggalkan mode otomatis](#switch-permission-modes) dan jawab permintaan izin.

<h3 id="when-auto-mode-falls-back">
  Ketika mode otomatis kembali
</h3>

Ketika mode otomatis tidak dapat menyetujui tindakan sesi Anda, apa yang terjadi tergantung pada kasusnya:

* **Tindakan yang diblokir**: Claude Code menampilkan pemberitahuan dan mencantumkan tindakan dalam `/permissions` di bawah tab **Recently denied**, di mana Anda dapat menekan `r` untuk mencoba ulang dengan persetujuan manual. Ketika pengklasifikasi menghasilkan [tidak ada vonis pada tindakan](/docs/id/errors#auto-mode-cannot-determine-the-safety-of-an-action), karena pemeriksaan keamanan terpisah dari mode otomatis menolak permintaan pengklasifikasi sendiri atau responsnya tidak diuraikan, Claude Code menolak tindakan tanpa pemberitahuan atau entri **Recently denied**.
* **Blok berulang**: jika pengklasifikasi memblokir tindakan 3 kali berturut-turut atau 20 kali total, mode otomatis berhenti dan Claude Code melanjutkan permintaan. Menyetujui tindakan yang diminta melanjutkan mode otomatis. Ambang batas ini tidak dapat dikonfigurasi. Tindakan yang diizinkan apa pun mengatur ulang penghitung berturut-turut, sementara penghitung total bertahan untuk sesi dan mengatur ulang hanya ketika batasnya sendiri memicu fallback. Claude Code tidak menghitung penolakan terhadap ambang batas apa pun ketika [pemeriksaan keamanan terpisah dari mode otomatis menolak permintaan pengklasifikasi](/docs/id/errors#auto-mode-cannot-determine-the-safety-of-an-action); entri tertaut mencakup cara Claude Code menangani penolakan tersebut.
* **Sesi yang tidak dapat meminta**: lari `-p` [non-interaktif](/docs/id/headless) tanpa [`--permission-prompt-tool`](/docs/id/cli-reference#cli-flags) tidak memiliki permintaan untuk kembali. Ketika blok berulang mencapai ambang batas, tindakan tidak berjalan dan Claude terus bekerja. Hal yang sama berlaku ketika [pemeriksaan keamanan terpisah dari mode otomatis menolak permintaan pengklasifikasi](/docs/id/errors#auto-mode-cannot-determine-the-safety-of-an-action). Claude Code tidak menghentikan lari dalam kedua kasus.
* **Tidak ada vonis dari server**: di bawah [tinjauan pengklasifikasi sisi server](#server-side-classifier-review), Claude Code menolak tindakan yang server tidak memberikan vonis, dan menghentikan giliran setelah sepuluh respons berturut-turut tanpa vonis. Lihat [Server tidak mengembalikan vonis keamanan](/docs/id/errors#the-server-returned-no-safety-verdict).
* **Beralih mode selama pemeriksaan**: jika Anda beralih mode izin saat pemeriksaan pengklasifikasi tertunda, Claude Code membuang vonis yang mode baru tidak akan diminta daripada menerapkannya: Anda diminta persetujuan sebagai gantinya, atau tindakan ditolak otomatis dalam [mode `dontAsk`](#allow-only-pre-approved-tools-with-dontask-mode).

Blok berulang biasanya berarti pengklasifikasi kehilangan konteks tentang infrastruktur Anda. Gunakan `/feedback` untuk melaporkan positif palsu, atau biarkan administrator [mengonfigurasi infrastruktur terpercaya](/docs/id/auto-mode-config).

<span id="how-the-classifier-evaluates-actions" />

<AccordionGroup>
  <Accordion title="Bagaimana pengklasifikasi mengevaluasi tindakan">
    Setiap tindakan melalui urutan keputusan tetap. Langkah pertama yang cocok menang:

    1. Tindakan yang cocok dengan [aturan allow, ask, atau deny](/docs/id/permissions#manage-permissions) Anda diselesaikan segera, dengan pengecualian ini:
       * Penulisan ke [jalur yang dilindungi](#protected-paths) rute ke pengklasifikasi bahkan ketika aturan allow cocok, dan begitu juga penghapusan `rm` dan `rmdir` yang menargetkan [jalur kritis](#critical-paths) dalam Claude Code v2.1.218 dan lebih baru
       * Alat MCP yang ditandai [`requiresUserInteraction`](/docs/id/mcp#require-approval-for-a-specific-tool) meminta Anda langsung bahkan ketika aturan allow cocok, dan begitu juga alat konektor [organisasi Anda atur ke `ask`](/docs/id/mcp#organization-controls-on-connector-tools) dalam sesi di mana pengaturan itu mencapai Claude Code
       * Perintah shell yang membawa [domain yang diizinkan per-perintah](/docs/id/sandboxing#per-command-allowed-domains-in-auto-mode) juga rute ke pengklasifikasi bahkan ketika aturan allow cocok, karena aturan menyetujui perintah, bukan hostnya
       * Aturan ask yang cocok pada konten perintah, seperti `Bash(git push *)`, kembali ke permintaan izin
    2. Tindakan hanya-baca dan pengeditan file di direktori kerja Anda disetujui otomatis, kecuali penulisan ke [jalur yang dilindungi](#protected-paths) dan [pembacaan pertama di luar direktori kerja](#first-read-outside-the-working-directories), yang meminta Anda
       * Dalam sesi dengan [tinjauan pengklasifikasi sisi server](#server-side-classifier-review), perintah hanya-baca dan [sandboxed](/docs/id/sandboxing#sandbox-modes) shell menunggu tinjauan itu dan diblokir jika itu menandainya
    3. Semuanya yang lain pergi ke pengklasifikasi. Alat konektor dan alat MCP `requiresUserInteraction` yang meminta Anda langsung dalam langkah 1 tidak pernah mencapai pengklasifikasi, jadi baik persetujuan yang diperlukan org maupun langkah persetujuan tidak disetujui otomatis
    4. Jika pengklasifikasi memblokir, Claude menerima alasan dan mencoba alternatif. Dalam sebagian besar sesi alasan menyebutkan aturan yang cocok pengklasifikasi, seperti `[Data Exfiltration]`, daripada memberikan penjelasan tertulis; lihat [Tinjauan penolakan](/docs/id/auto-mode-config#review-denials)

    Saat memasuki mode otomatis, aturan allow luas yang memberikan eksekusi kode arbitrer dijatuhkan:

    * Blanket `Bash(*)` atau `PowerShell(*)`
    * Interpreter yang diberi wildcard seperti `Bash(python*)`
    * Perintah jalankan pengelola paket
    * Aturan allow `Agent`
    * Aturan allow [`Monitor`](/docs/id/tools-reference#monitor-tool), karena Claude Code menjalankan perintah Monitor melalui shell

    Aturan sempit seperti `Bash(npm test)` tetap berlaku. Claude Code mengembalikan aturan yang dijatuhkan ketika Anda meninggalkan mode otomatis. Sebelum v2.1.236, Claude Code membiarkan aturan allow `Monitor` berlaku dalam mode otomatis, jadi aturan yang cocok dengan seluruh alat menyetujui perintah Monitor tanpa tinjauan pengklasifikasi.

    Claude Code juga menjalankan `git status` itu sendiri sebelum perintah yang akan membuang pekerjaan yang tidak dilakukan komit, seperti `git reset --hard` atau `rm -rf`, dan menunjukkan pengklasifikasi apakah pekerjaan yang dipentaskan, dimodifikasi, atau tidak dilacak ada. Claude Code melaporkan file yang tidak dilacak dalam pemeriksaan itu bahkan ketika konfigurasi git repositori menetapkan `status.showUntrackedFiles=no`.

    Dalam permintaan pengklasifikasi yang dikirim Claude Code itu sendiri, pengklasifikasi melihat pesan pengguna, panggilan alat selain pencarian hanya-baca seperti pembacaan file dan pencarian, dan konten CLAUDE.md Anda. Hasil alat dilepas dari permintaan tersebut, jadi konten bermusuhan dalam file atau halaman web tidak dapat memanipulasi pengklasifikasi secara langsung.

    Anda dapat memberi anotasi hasil panggilan dengan [field `classifierContext` hook PostToolUse](/docs/id/hooks#annotate-a-result-for-the-auto-mode-classifier), yang dibaca pengklasifikasi sebagai konteks yang disediakan aplikasi. Field memerlukan Claude Code v2.1.236 atau lebih baru.

    Probe sisi server terpisah memindai hasil alat masuk dan menandai konten mencurigakan sebelum Claude membacanya. Untuk lebih lanjut tentang cara lapisan ini bekerja bersama, lihat [pengumuman mode otomatis](https://claude.com/blog/auto-mode) dan [penggalian teknik](https://www.anthropic.com/engineering/claude-code-auto-mode).
  </Accordion>

  <Accordion title="Bagaimana mode otomatis menangani subagen">
    Pengklasifikasi memeriksa pekerjaan [subagen](/docs/id/sub-agents) di tiga titik:

    1. Sebelum subagen dimulai, deskripsi tugas yang didelegasikan dievaluasi, jadi tugas yang terlihat berbahaya diblokir pada waktu spawn.
    2. Saat subagen berjalan, setiap tindakannya melalui pengklasifikasi dengan aturan yang sama seperti sesi induk, dan `permissionMode` apa pun dalam frontmatter subagen diabaikan.
    3. Ketika subagen selesai, pengklasifikasi meninjau pekerjaannya dan laporan akhirnya sebelum induk membaca laporan. Ketika pengklasifikasi menandai pekerjaan atau laporan subagen, atau pemeriksaan API keamanan terpisah menolak tinjauan, laporan masih disampaikan, diawali dengan peringatan keamanan. Ketika pengklasifikasi tidak tersedia untuk tinjauan, laporan tiba dengan catatan untuk memverifikasi pekerjaan subagen sebelum bertindak atas dasar itu.
  </Accordion>

  <Accordion title="Biaya dan latensi">
    Pengklasifikasi berjalan pada Claude Sonnet 5 secara default daripada pada pilihan `/model` Anda. Model pengklasifikasi yang dikonfigurasi Anthropic sisi server mengambil preseden atas default itu. Ketika model sesi Anda adalah Claude Sonnet 4.6, atau ketika [`availableModels`](/docs/id/model-config#restrict-model-selection) mengecualikan Sonnet 5, pengklasifikasi berjalan pada model sesi sebagai gantinya, atau pada model Opus ketika sesi berjalan pada [model Fable](/docs/id/model-config#work-with-fable); di penyedia selain Anthropic API, fallback Opus itu adalah model Opus default penyedia.

    Permintaan mode otomatis pertama sesi memvalidasi default Sonnet 5: jika permintaan berhasil, Sonnet 5 tetap model pengklasifikasi sesi, dan jika gagal karena model tidak tersedia, sesi menggunakan fallback sebagai gantinya. Setelah validasi itu diselesaikan, model pengklasifikasi tidak berubah untuk sesi.

    Pada paket Enterprise dan pada akun yang menggunakan Claude API, [Claude Platform di AWS](/docs/id/claude-platform-on-aws), Amazon Bedrock, Agent Platform Google Cloud, atau Microsoft Foundry, panggilan pengklasifikasi dihitung terhadap penggunaan token Anda. Setiap pemeriksaan mengirim sebagian dari transkrip ditambah tindakan yang tertunda, menambahkan putaran perjalanan pulang sebelum eksekusi. Pembacaan dan pengeditan direktori kerja di luar jalur yang dilindungi melewati pengklasifikasi, jadi overhead datang terutama dari perintah shell dan operasi jaringan. Di mana server meninjau tindakan sebagai bagian dari permintaan model sesi, tidak ada permintaan pengklasifikasi terpisah untuk dihitung; lihat [Tinjauan pengklasifikasi sisi server](#server-side-classifier-review).

    Akses jaringan sandboxed tidak menambahkan permintaan pengklasifikasi per-koneksi. Pengklasifikasi menilai [host yang dinamai perintah](/docs/id/sandboxing#per-command-allowed-domains-in-auto-mode) bersama dengan perintah dalam satu tinjauan, dan Claude Code memeriksa setiap koneksi terhadap daftar yang disetujui tanpa memanggil pengklasifikasi lagi.
  </Accordion>
</AccordionGroup>

<h2 id="allow-only-pre-approved-tools-with-dontask-mode">
  Izinkan hanya alat yang telah disetujui sebelumnya dengan mode dontAsk
</h2>

Jika Anda menetapkan mode `dontAsk`, Claude Code secara otomatis menolak setiap panggilan alat yang akan meminta sebaliknya. Claude masih menjalankan tindakan yang tidak memerlukan persetujuan dalam mode Manual, seperti pembacaan file di dalam direktori kerja Anda dan [perintah Bash read-only](/docs/id/permissions#read-only-commands), ditambah tindakan yang cocok dengan aturan `permissions.allow` Anda dan panggilan yang disetujui oleh [hook PreToolUse](/docs/id/permissions#extend-permissions-with-hooks). Gunakan mode ini untuk pipeline CI atau lingkungan terbatas di mana Anda pre-define apa yang Claude boleh lakukan; sesi tidak pernah menunggu input. Bilah status menunjukkan `⏵⏵ don't ask on` saat mode ini aktif.

Claude Code menolak panggilan yang cocok dengan aturan [`ask`](/docs/id/permissions#manage-permissions) eksplisit Anda daripada meminta. Ini juga menolak alat `AskUserQuestion` bawaan bahkan jika aturan allow Anda cocok dengannya, dan melakukan hal yang sama untuk alat konektor [organisasi Anda atur ke `ask`](/docs/id/mcp#organization-controls-on-connector-tools) dalam sesi di mana pengaturan itu mencapai Claude Code. Ini menolak alat MCP yang ditandai [`_meta["anthropic/requiresUserInteraction"]`](/docs/id/mcp#require-approval-for-a-specific-tool) dengan cara yang sama, karena kartu persetujuan mereka memerlukan jawaban yang mode ini tidak pernah kumpulkan; ini memerlukan Claude Code v2.1.199 atau lebih baru.

Penghapusan `rm` dan `rmdir` yang menargetkan [jalur kritis](#critical-paths), seperti `rm -rf /` dan `rm -rf ~`, ditolak bahkan ketika aturan allow cocok dengannya atau hook `PreToolUse` memungkinkannya.

Sesi cloud di [Claude Code di web](/docs/id/claude-code-on-the-web) mengabaikan `defaultMode: "dontAsk"`; lihat [bypassPermissions](#skip-all-checks-with-bypasspermissions-mode) untuk detail.

Atur saat startup dengan flag:

```bash theme={null}
claude --permission-mode dontAsk
```

<h2 id="skip-all-checks-with-bypasspermissions-mode">
  Lewati semua pemeriksaan dengan mode bypassPermissions
</h2>

Mode `bypassPermissions` menonaktifkan prompt izin dan pemeriksaan keamanan sehingga panggilan alat dijalankan segera, termasuk penulisan ke [jalur yang dilindungi](#protected-paths).

[Tindakan yang tidak ada mode auto-approve](#actions-no-mode-auto-approves) masih meminta dalam mode ini.

Dua perlindungan [pesan lintas sesi](/docs/id/cross-session-messaging) masih berlaku dalam mode ini, dan dalam sesi mode rencana terminal interaktif di mana bypass permissions tersedia:

* Prompt persetujuan [`isolatePeerMachines`](/docs/id/settings-reference#isolatepeermachines) untuk pesan ke sesi Anda di luar mesin ini masih muncul.
* Ketika tidak ada nilai [`crossSessionInbound`](/docs/id/cross-session-messaging#control-inbound-messages) berlaku, Claude Code menahan pesan masuk dari sesi lain Anda untuk persetujuan Anda, dan mengirimkan tanpa bertanya hanya ketika sesi pengirim mengidentifikasi dirinya sebagai juga melewati prompt izin. Jika Anda meninggalkan mode izin saat pesan ditahan, Claude Code menerapkan kembali aturan masuk dan mengirimkan pesan yang ditahan apa pun yang sekarang diterima.

Dalam sesi terminal interaktif dengan bypass permissions tersedia, Claude Code juga tidak memberlakukan [blokir mode rencana](#analyze-before-you-edit-with-plan-mode). Claude masih diinstruksikan untuk merencanakan tanpa mengedit, tetapi pengeditan file atau perintah shell yang dicoba selama perencanaan berjalan tanpa meminta. [Aturan ask](/docs/id/permissions#manage-permissions) eksplisit dan penghapusan `rm` dan `rmdir` yang menargetkan [jalur kritis](#critical-paths) masih meminta.

Mode rencana menjaga blokir di mana pun Claude Code berjalan tanpa terminal interaktif, termasuk [run non-interaktif](/docs/id/headless) dengan `-p`, sesi [Agent SDK](/docs/id/agent-sdk/permissions#plan-mode-plan), dan percakapan di panel chat [VS Code extension](/docs/id/vs-code). Di sana, `--allow-dangerously-skip-permissions` membuat `bypassPermissions` dapat dipilih nanti.

<Warning>
  Hanya gunakan mode ini di lingkungan terisolasi seperti kontainer, VM, atau dev container tanpa akses internet, di mana Claude Code tidak dapat merusak sistem host Anda.
</Warning>

Anda tidak dapat memasukkan `bypassPermissions` dari sesi yang dimulai tanpa itu diaktifkan. Aktifkan saat peluncuran dengan [`permissions.defaultMode: "bypassPermissions"`](/docs/id/settings-reference#permissions-defaultmode) atau dengan flag yang mengaktifkan:

```bash theme={null}
claude --permission-mode bypassPermissions
```

Flag `--dangerously-skip-permissions` setara.

Claude Code menolak `bypassPermissions` dalam sesi yang Anda mulai dengan [`--restricted`](/docs/id/cli-reference#cli-flags). `--restricted` memerlukan Claude Code v2.1.248 atau lebih baru.

Pertama kali Anda memulai sesi interaktif dengan mode ini diaktifkan, Claude Code menampilkan dialog peringatan yang meminta Anda menerima tanggung jawab untuk tindakan yang diambil tanpa pemeriksaan izin. Claude Code menyimpan penerimaan Anda ke pengaturan pengguna, jadi dialog muncul hanya sekali. Jika Anda menolak, Claude Code keluar. Dalam [mode non-interaktif](/docs/id/headless) tidak ada dialog yang ditampilkan, dan [sesi latar belakang](/docs/id/agent-view) yang dimulai dengan `--bg` ditolak sampai Anda telah menerima dialog dalam sesi interaktif.

Di Linux dan macOS, Claude Code menolak untuk memulai dalam mode ini saat berjalan sebagai root atau di bawah `sudo`:

```text theme={null}
--dangerously-skip-permissions cannot be used with root/sudo privileges for security reasons
```

Pemeriksaan dilewati secara otomatis di dalam sandbox yang dikenali. Untuk menjalankan secara otonom dalam kontainer, gunakan konfigurasi [dev container](/docs/id/devcontainer), yang menjalankan Claude Code sebagai pengguna non-root.

[Claude Code di web](/docs/id/claude-code-on-the-web) tidak menghormati `defaultMode: "bypassPermissions"` atau `"dontAsk"` dari file pengaturan Anda, jadi pengaturan yang diperiksa dalam repositori tidak dapat memulai sesi cloud dalam mode bypass-permissions. Pengaturan diabaikan secara diam-diam dan sesi dimulai dalam mode izin yang ditampilkan di dropdown mode sebagai gantinya. Lihat [Alihkan mode izin](#switch-permission-modes) untuk mode mana yang ditawarkan sesi cloud.

<Warning>
  `bypassPermissions` tidak menawarkan perlindungan terhadap injeksi prompt atau tindakan yang tidak diinginkan. Untuk pemeriksaan keamanan latar belakang dengan jauh lebih sedikit prompt, gunakan [mode auto](#eliminate-prompts-with-auto-mode) sebagai gantinya. Administrator dapat memblokir mode ini dengan mengatur `permissions.disableBypassPermissionsMode` ke `"disable"` dalam [pengaturan terkelola](/docs/id/managed-settings).
</Warning>

<h2 id="protected-paths">
  Jalur yang dilindungi
</h2>

Penulisan ke serangkaian jalur kecil tidak pernah disetujui otomatis, kecuali dalam mode `bypassPermissions` dan dalam sesi terminal interaktif mode plan dengan [bypass permissions](#skip-all-checks-with-bypasspermissions-mode) tersedia. Ini mencegah kerusakan yang tidak disengaja dari status repositori dan konfigurasi Claude sendiri.

| Mode                     | Penulisan jalur yang dilindungi                                                                                                                                                                                                                                                   |
| :----------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `default`, `acceptEdits` | Diminta                                                                                                                                                                                                                                                                           |
| `plan`                   | Diizinkan dalam sesi terminal interaktif dengan [bypass permissions](#skip-all-checks-with-bypasspermissions-mode) tersedia. Jika tidak, dirutekan ke pengklasifikasi ketika [mode auto](#eliminate-prompts-with-auto-mode) tersedia selama perencanaan, dan diminta ketika tidak |
| `auto`                   | Dirutekan ke pengklasifikasi                                                                                                                                                                                                                                                      |
| `dontAsk`                | Ditolak                                                                                                                                                                                                                                                                           |
| `bypassPermissions`      | Diizinkan                                                                                                                                                                                                                                                                         |

Dalam sesi yang dimulai dengan [`--restricted`](/docs/id/cli-reference#cli-flags), yang memerlukan Claude Code v2.1.248 atau lebih baru, pengklasifikasi tidak dapat menyetujui penulisan jalur yang dilindungi.

Aturan [`permissions.allow`](/docs/id/permissions#manage-permissions) dalam file pengaturan tidak pra-menyetujui penulisan jalur yang dilindungi. Pemeriksaan keamanan berjalan sebelum Claude Code mengevaluasi aturan allow dari pengaturan, jadi entri seperti `Edit(.claude/**)` dalam `~/.claude/settings.json` atau `.claude/settings.json` tidak mengubah hasil per-mode dalam tabel di atas. Dalam mode yang meminta, prompt untuk penulisan `.claude/` menawarkan **Ya, dan izinkan Claude untuk mengedit pengaturannya sendiri untuk sesi ini**, yang menyetujui penulisan `.claude/` berikutnya dalam sesi itu tanpa meminta lagi.

Direktori yang dilindungi:

* `.git`
* `.config/git`
* `.vscode`
* `.idea`
* `.husky`
* `.cargo`
* `.devcontainer`
* `.yarn`
* `.mvn`
* `.claude`, kecuali untuk `.claude/worktrees` di mana Claude menyimpan git worktrees-nya sendiri

File yang dilindungi:

* `.gitconfig`, `.gitmodules`
* `.bashrc`, `.bash_profile`, `.bash_login`, `.bash_aliases`, `.bash_logout`, `.zshrc`, `.zprofile`, `.zshenv`, `.zlogin`, `.zlogout`, `.profile`, `.envrc`
* `.npmrc`, `.yarnrc`, `.yarnrc.yml`, `.pnp.cjs`, `.pnp.loader.mjs`, `.pnpmfile.cjs`, `bunfig.toml`, `.bunfig.toml`
* `.bazelrc`, `.bazelversion`, `.bazeliskrc`
* `.pre-commit-config.yaml`, `lefthook.yml`, `lefthook.yaml`, `.lefthook.yml`, `.lefthook.yaml`
* `gradle-wrapper.properties`, `maven-wrapper.properties`
* `.devcontainer.json`
* `.ripgreprc`, `pyrightconfig.json`
* `.mcp.json`, `.claude.json`

<h2 id="critical-paths">
  Jalur kritis
</h2>

Claude Code tidak pernah membiarkan aturan [`permissions.allow`](/docs/id/permissions#manage-permissions) atau hook [`PreToolUse`](/docs/id/permissions#extend-permissions-with-hooks) yang mengembalikan `"allow"` menyetujui perintah `rm` atau `rmdir` yang menargetkan jalur kritis, bahkan dalam mode yang melewati prompt lain. Pemutus sirkuit ini menjaga terhadap kesalahan model. Aturan deny yang cocok masih memblokir perintah sepenuhnya.

Apa yang terjadi sebagai gantinya bergantung pada mode izin Anda:

| Mode                     | Apa yang dilakukan Claude Code dengan penghapusan jalur kritis                                                                                                                                                          |
| :----------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `default`, `acceptEdits` | Meminta Anda untuk menyetujuinya                                                                                                                                                                                        |
| `plan`                   | Meminta Anda untuk menyetujuinya. Dengan [mode auto tersedia selama perencanaan](#analyze-before-you-edit-with-plan-mode) dan tidak ada bypass permissions tersedia, mengirimkannya ke pengklasifikasi sebagai gantinya |
| `auto`                   | Mengirimkannya ke [pengklasifikasi](#eliminate-prompts-with-auto-mode)                                                                                                                                                  |
| `dontAsk`                | Menolaknya                                                                                                                                                                                                              |
| `bypassPermissions`      | Meminta Anda untuk menyetujuinya                                                                                                                                                                                        |

Jika [aturan ask](/docs/id/permissions#manage-permissions) eksplisit cocok dengan perintah, Claude Code meminta Anda bahkan dalam mode `auto`. Dalam mode yang meminta, hook [`PermissionRequest`](/docs/id/hooks#permissionrequest) dapat menjawab prompt dengan cara yang menjawab apa pun yang lain.

Claude Code memperlakukan target `rm` atau `rmdir` sebagai jalur kritis ketika itu adalah salah satu dari yang berikut:

* Akar filesystem
* Direktori tingkat atas, artinya anak langsung apa pun dari akar, seperti `/usr`, `/etc`, atau `/data`
* Direktori home Anda
* Akar drive Windows dan direktori tingkat atas mereka, seperti `C:\` dan `C:\Windows`
* Direktori kerja Anda dan induknya
* Direktori kerja tambahan Anda dan induk mereka, tetapi hanya ketika penghapusan adalah glob di bawah salah satu dari mereka, seperti `rm -rf <dir>/*`. `rm -rf <dir>` pada direktori itu sendiri tidak memicu pemeriksaan ini

Claude Code juga memperlakukan glob atau garis miring di bawah variabel shell, seperti `rm -rf "$DIR"/*`, sebagai penghapusan jalur kritis, karena perintah menjadi penghapusan dari akar filesystem ketika variabel kosong.

Prompt untuk kasus variabel ini menamai `rm` yang ditandai dan mengatakan cara menulis ulangnya sehingga pemeriksaan lulus:

* Untuk variabel seperti `$DIR`, lindungi setiap ekspansi sehingga shell berhenti dengan kesalahan ketika variabel tidak diatur atau kosong, seperti dalam `rm -rf "${DIR:?}"/*`, atau gunakan jalur literal
* Untuk variabel yang biasanya diatur, seperti `$HOME`, gunakan jalur literal

Penghapusan yang ekspansinya semuanya dilindungi dengan cara itu bukan penghapusan jalur kritis, jadi dalam mode `bypassPermissions` itu berjalan tanpa prompt.

Menyembunyikan penghapusan di dalam subshell dengan `(...)`, grup brace dengan `{ ...; }`, substitusi perintah dengan `$(...)` atau backtick, atau substitusi proses dengan `<(...)`, tidak melewati pemeriksaan. Claude Code menemukan penghapusan jalur kritis apakah itu berada di dalam bentuk bersarang, seperti dalam `(rm -rf ~)` atau `echo "$(rm -rf ~)"`, atau di tempat lain dalam perintah yang sama.

<h3 id="remove-item-in-powershell">
  Remove-Item dalam PowerShell
</h3>

Ketika Anda mengaktifkan [alat PowerShell](/docs/id/tools-reference#powershell-tool), Claude Code memberikan `Remove-Item` pemeriksaannya sendiri, terpisah dari daftar jalur kritis `rm`. Hasilnya bergantung pada target, dan kasus pertama yang cocok berlaku:

* **Jalur sistem**: akar filesystem dan direktori tingkat atasnya, akar drive dan direktori tingkat atas mereka, dan direktori home Anda. Claude Code menolak perintah di setiap mode, tanpa bertanya kepada Anda.
* **Wildcard**: bare `*`, atau target apa pun yang berakhir dalam `/*` atau `\*`, termasuk glob di bawah variabel shell seperti `$dir/*`. Claude Code menolak perintah di setiap mode, tanpa bertanya kepada Anda, sebelum [pengklasifikasi](#eliminate-prompts-with-auto-mode) melihatnya.
* **Direktori kerja Anda atau salah satu induknya, dengan `-Recurse`**: Claude Code memperlakukan perintah seperti apa pun yang memerlukan persetujuan dalam mode izin Anda, jadi itu meminta Anda dalam mode yang meminta, mengirimkannya ke pengklasifikasi dalam mode `auto`, dan menolaknya dalam mode `dontAsk`. Mode `bypassPermissions` melewati pemeriksaan ini.

<h2 id="see-also">
  Lihat juga
</h2>

* [Permissions](/docs/id/permissions): aturan allow, ask, dan deny; kebijakan terkelola
* [Konfigurasi mode auto](/docs/id/auto-mode-config): beri tahu pengklasifikasi infrastruktur mana yang dipercaya organisasi Anda
* [Hooks](/docs/id/hooks): logika izin kustom melalui hook `PreToolUse` dan `PermissionRequest`
* [Security](/docs/id/security): perlindungan dan praktik terbaik
* [Sandboxing](/docs/id/sandboxing): isolasi filesystem dan jaringan untuk perintah Bash
* [Mode non-interaktif](/docs/id/headless): jalankan Claude Code dengan flag `-p`
