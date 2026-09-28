> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Konfigurasi alat Bash sandboxed

> Pelajari bagaimana alat Bash sandboxed Claude Code menyediakan isolasi filesystem dan jaringan untuk eksekusi agen yang lebih aman dan mandiri.

Sandbox Bash memungkinkan Claude menjalankan sebagian besar perintah shell tanpa berhenti untuk meminta izin. Alih-alih menyetujui setiap perintah, Anda menentukan file dan domain jaringan mana yang dapat diakses perintah, dan sistem operasi memberlakukan batas itu untuk setiap perintah Bash, PowerShell, atau Monitor dan proses anak-anaknya.

<Note>
  Untuk membandingkan pendekatan isolasi lain seperti dev containers, container khusus, dan mesin virtual, lihat [Sandbox environments](/docs/id/sandbox-environments). Untuk mengurangi prompt izin untuk alat selain Bash, lihat [permission modes](/docs/id/permission-modes).
</Note>

<h2 id="get-started">
  Memulai
</h2>

Sandbox dibangun ke dalam Claude Code dan berjalan di macOS, Linux, dan WSL2. Windows asli tidak didukung. Di Windows, jalankan Claude Code di dalam distribusi WSL2.

Di macOS, tidak ada yang perlu diinstal: sandboxing menggunakan kerangka Seatbelt bawaan. Di Linux dan WSL2, sandbox bergantung pada dua paket, yang dibahas dalam [Set up Linux and WSL2](#set-up-linux-and-wsl2). Bahkan jika Anda belum menginstalnya, Anda dapat memulai dengan `/sandbox`, karena panelnya menunjukkan apakah ada yang hilang.

<Steps>
  <Step title="Jalankan /sandbox">
    Mulai sesi Claude Code dan jalankan perintah `/sandbox`:

    ```text theme={null}
    /sandbox
    ```

    Ini membuka panel sandbox dengan tiga tab, ditambah tab Dependencies di Linux ketika filter seccomp opsional hilang:

    * **Mode**: pilih bagaimana perintah sandboxed disetujui, dibahas dalam langkah berikutnya
    * **Overrides**: pilih apakah perintah yang gagal di bawah sandbox dapat kembali ke menjalankan unsandboxed. Ini adalah pengaturan [`allowUnsandboxedCommands`](/docs/id/settings-reference#sandbox-allowunsandboxedcommands)
    * **Config**: lihat pengaturan sandbox yang diselesaikan

    Jika panel hanya menampilkan tab Dependencies, paket yang diperlukan hilang. Instal seperti yang dijelaskan dalam [Set up Linux and WSL2](#set-up-linux-and-wsl2), restart Claude Code, dan jalankan `/sandbox` lagi.
  </Step>

  <Step title="Pilih mode">
    Di tab Mode, pilih auto-allow atau regular permissions. Auto-allow menjalankan perintah sandboxed tanpa prompt, dan regular permissions menjaga prompt izin reguler bahkan ketika perintah sandboxed. Lihat [Sandbox modes](#sandbox-modes) untuk perintah mana yang masih prompt dalam mode auto-allow.
  </Step>

  <Step title="Jalankan perintah Bash">
    Minta Claude untuk menjalankan perintah, seperti build atau test suite. Secara default, perintah di dalam sandbox dapat menulis ke direktori kerja, direktori temp sesi, dan [direktori apa pun yang telah Anda tambahkan](/docs/id/permissions#additional-directories-grant-file-access-not-configuration) dengan `--add-dir`, `/add-dir`, atau `permissions.additionalDirectories`.

    Pertama kali perintah memerlukan domain jaringan baru, Claude Code meminta persetujuan; dalam [mode auto](/docs/id/permission-modes#eliminate-prompts-with-auto-mode), Claude malah menamai host yang dibutuhkan perintah [pada perintah itu sendiri](#per-command-allowed-domains-in-auto-mode) untuk pengklasifikasi ditinjau bersama dengannya.

    Perintah yang tidak dapat berjalan sandboxed kembali ke alur izin reguler. Claude Code memberi judul prompt izin mereka "Bash command (unsandboxed)" bukan "Bash command", sehingga Anda dapat mengetahui perintah mana yang berjalan di luar sandbox. Untuk memperluas atau mempersempit apa yang diizinkan sandbox, lihat [Configure sandboxing](#configure-sandboxing).

    Jika perintah sandboxed gagal dengan `Operation not permitted` di dalam kontainer, lihat entri Bubblewrap di bawah [Troubleshooting](#troubleshooting).
  </Step>
</Steps>

Ketika Anda memilih mode dalam panel, Claude Code menyimpannya ke pengaturan lokal proyek Anda di `.claude/settings.local.json`, yang berlaku untuk proyek saat ini. Claude Code menambahkan file itu ke gitignore global Anda ketika menyimpan pengaturan di sana. Untuk mengaktifkan sandbox di semua proyek Anda, atur [`sandbox.enabled`](/docs/id/settings-reference#sandbox-enabled) ke `true` dalam pengaturan pengguna Anda di `~/.claude/settings.json`. Untuk memberlakukan sandboxing untuk setiap pengembang dalam organisasi, gunakan [managed settings](#enforce-sandboxing-with-managed-settings).

Untuk mengubah sandbox untuk satu sesi tanpa menulis ke file pengaturan, mulai Claude Code dengan [`--settings`](/docs/id/settings#change-a-setting-for-one-session). Misalnya, perintah ini memulai sesi sandboxed di mana Claude tidak dapat mencoba kembali perintah yang diblokir di luar sandbox:

```bash theme={null}
claude --settings '{"sandbox": {"enabled": true, "allowUnsandboxedCommands": false}}'
```

<Warning>
  Secara default, jika sandbox tidak dapat dimulai karena dependensi hilang atau platform tidak didukung, Claude Code menampilkan peringatan dan menjalankan perintah tanpa sandboxing. Untuk menjadikan ini kegagalan keras sebagai gantinya, atur [`sandbox.failIfUnavailable`](/docs/id/settings-reference#sandbox-failifunavailable) ke `true`. Ini dimaksudkan untuk penyebaran terkelola yang memerlukan sandboxing sebagai gerbang keamanan.
</Warning>

<h3 id="set-up-linux-and-wsl2">
  Set up Linux dan WSL2
</h3>

Di Linux dan WSL2, sandbox bergantung pada dua paket:

* [`bubblewrap`](https://github.com/containers/bubblewrap): alat sandboxing tanpa privilege yang memberlakukan isolasi filesystem
* [`socat`](http://www.dest-unreach.org/socat/): relay yang digunakan untuk merutekan lalu lintas jaringan melalui proxy sandbox

Instal dengan manajer paket distribusi Anda:

<Tabs>
  <Tab title="Ubuntu/Debian">
    ```bash theme={null}
    sudo apt-get install bubblewrap socat
    ```
  </Tab>

  <Tab title="Fedora">
    ```bash theme={null}
    sudo dnf install bubblewrap socat
    ```
  </Tab>
</Tabs>

Ketika dependensi hilang, tab Dependencies dalam `/sandbox` mencantumkan mana dari `ripgrep`, `bubblewrap`, `socat`, dan filter seccomp yang platform Anda kurangi. Jika Anda tidak melihat tab setelah menginstal dan memulai ulang Claude Code, semua dependensi ada.

Ripgrep disertakan dengan binari Claude Code asli. Filter seccomp bersifat opsional dan menambahkan pemblokiran soket domain Unix. Instal dengan `npm install -g @anthropic-ai/sandbox-runtime` jika hilang.

Ketika dependensi yang diperlukan hilang, tab Dependencies adalah satu-satunya tab yang ditampilkan sampai Anda menginstalnya. Ketika hanya filter seccomp opsional yang hilang, tab Dependencies muncul bersama tab lainnya. Pemeriksaan dependensi berjalan saat startup, jadi restart Claude Code setelah menginstal paket untuk `/sandbox` mendeteksinya.

<AccordionGroup>
  <Accordion title="Ubuntu 24.04 dan yang lebih baru: izinkan bubblewrap untuk membuat user namespaces">
    Di Ubuntu 24.04 dan yang lebih baru, kebijakan AppArmor default mencegah bubblewrap dari membuat user namespaces yang dibutuhkannya untuk isolasi.

    Untuk memeriksa apakah lingkungan Anda memberlakukan pembatasan ini, termasuk di dalam WSL2, jalankan `sysctl kernel.apparmor_restrict_unprivileged_userns`. Jika perintah mengembalikan `0`, lewati langkah ini. Jika mencetak kesalahan `No such file or directory`, kunci tidak ada dan Anda dapat melewati langkah ini. Jika mengembalikan `1`, tambahkan profil AppArmor yang memberikan `bwrap` kemampuan ini:

    ```bash theme={null}
    sudo tee /etc/apparmor.d/bwrap > /dev/null <<'EOF'
    abi <abi/4.0>,
    include <tunables/global>

    profile bwrap /usr/bin/bwrap flags=(unconfined) {
      userns,
      include if exists <local/bwrap>
    }
    EOF
    ```

    Profil hanya berlaku untuk `bwrap` itu sendiri, bukan untuk perintah yang berjalan di dalam sandbox. Muat ulang AppArmor untuk menerapkannya:

    ```bash theme={null}
    sudo systemctl reload apparmor
    ```
  </Accordion>

  <Accordion title="Catatan WSL2">
    Periksa versi WSL Anda dengan `wsl -l -v` dari PowerShell. Jika Anda melihat `Sandboxing requires WSL2`, distribusi Anda menjalankan WSL1. Tingkatkan ke WSL2 atau jalankan Claude Code tanpa sandboxing.

    Di WSL2, WSL menyerahkan peluncuran binari Windows seperti `cmd.exe`, `powershell.exe`, atau apa pun di bawah `/mnt/c/` ke host Windows melalui soket Unix, jadi apakah perintah sandboxed dapat meluncurkan satu mengikuti pengaturan [Unix-socket](/docs/id/settings-reference#sandbox-network-allowunixsockets) sandbox: filter seccomp opsional harus diinstal untuk memblokir soket di tempat pertama. Untuk memungkinkan peluncuran ini, atur `allowAllUnixSockets`; untuk menjaganya keluar dari sandbox sepenuhnya, tambahkan perintah ke [`excludedCommands`](/docs/id/settings-reference#sandbox-excludedcommands).
  </Accordion>
</AccordionGroup>

<h3 id="sandbox-modes">
  Mode sandbox
</h3>

Claude Code menawarkan dua mode sandbox. Di keduanya, sandbox memberlakukan pembatasan filesystem dan jaringan yang sama; perbedaannya hanya dalam apakah perintah sandboxed disetujui secara otomatis atau memerlukan izin eksplisit.

<h4 id="auto-allow-mode">
  Mode auto-allow
</h4>

Ketika perintah dapat di-sandbox, Claude Code menjalankannya di dalam sandbox dan menyetujuinya secara otomatis, tanpa meminta izin Anda. Perintah yang tidak dapat di-sandbox, seperti yang memerlukan akses jaringan ke host yang tidak diizinkan, kembali ke alur izin reguler, di mana Claude Code memeriksa [permission rules](/docs/id/permissions) Anda dan membatasi perintah apa pun yang tidak diizinkan oleh aturan tersebut, dengan prompt dalam mode Manual.

Bahkan dalam mode auto-allow, hal berikut masih berlaku:

* [Deny rules](/docs/id/permissions) eksplisit selalu dihormati
* Perintah `rm` atau `rmdir` yang menargetkan [critical path](/docs/id/permission-modes#critical-paths) masih melalui alur izin reguler
* [Ask rules](/docs/id/permissions) yang dibatasi konten seperti `Bash(git push *)` masih memaksa prompt bahkan untuk perintah sandboxed
* Aturan `Bash` ask yang kosong, atau bentuk setara `Bash(*)`, dilewati untuk perintah yang berjalan sandboxed; masih berlaku untuk perintah yang kembali ke alur izin reguler. Dalam [plan mode](/docs/id/permission-modes#analyze-before-you-edit-with-plan-mode), aturan tidak dilewati: itu meminta perintah sandboxed juga, termasuk yang hanya-baca. Sebelum v2.1.212, skip berlaku dalam plan mode juga

<Info>
  Mode auto-allow bekerja secara independen dari pengaturan mode izin Anda, dengan tiga pengecualian: [plan mode](/docs/id/permission-modes#analyze-before-you-edit-with-plan-mode), perintah mode auto yang membawa [per-command allowed domains](#per-command-allowed-domains-in-auto-mode), dan [server-side classifier review](/docs/id/permission-modes#how-the-classifier-evaluates-actions) dari perintah sandboxed dalam mode auto. Bahkan jika Anda tidak dalam mode "accept edits", perintah Bash sandboxed berjalan secara otomatis ketika auto-allow diaktifkan. Ini berarti perintah Bash yang memodifikasi file dalam batas sandbox dieksekusi tanpa prompt, bahkan dalam mode Manual, di mana alat edit file akan meminta.

  Dalam plan mode, auto-allow tidak memperluas persetujuan; lihat [plan mode](/docs/id/permission-modes#analyze-before-you-edit-with-plan-mode) untuk bagaimana Claude Code membatasi perintah saat Anda merencanakan. Sebelum v2.1.212, auto-allow menjalankan perintah sandboxed tanpa prompt dalam plan mode juga.
</Info>

<h4 id="regular-permissions-mode">
  Mode regular permissions
</h4>

Semua perintah Bash melalui alur izin reguler, bahkan ketika sandboxed. Ini memberikan lebih banyak kontrol tetapi memerlukan lebih banyak persetujuan.

<h4 id="the-unsandboxed-retry-escape-hatch">
  Pintu keluar retry unsandboxed
</h4>

Beberapa perintah tidak dapat berjalan di dalam sandbox sama sekali, seperti alat yang tidak kompatibel dengannya atau yang memerlukan host yang belum Anda izinkan. Claude Code melaporkan pelanggaran sandbox dalam hasil perintah yang diblokir, menamai jalur atau host yang sandbox tolak, sehingga Claude melihat apa yang sandbox blokir. Daripada gagal tugas atau memerlukan Anda untuk mematikan sandboxing, Claude Code menyertakan pintu keluar: Claude menganalisis pelanggaran dan dapat mencoba kembali perintah dengan parameter `dangerouslyDisableSandbox`.

Perintah yang dicoba kembali berjalan di luar sandbox, sehingga melalui alur izin reguler. Dalam mode Manual Anda mendapatkan prompt konfirmasi. Dalam [auto mode](/docs/id/permission-modes#eliminate-prompts-with-auto-mode), pengklasifikasi mengevaluasi perintah yang mendasar. Sementara [`permissions.blockReadsOutsideWorkingDirectories`](/docs/id/settings-reference#permissions-blockreadsoutsideworkingdirectories) aktif, retry yang memerlukan persetujuan untuk berjalan di luar sandbox meminta Anda sebagai gantinya. Untuk diminta pada setiap retry unsandboxed bahkan dalam mode auto, tambahkan [ask rule](/docs/id/permissions#match-by-input-parameter) untuk `Bash(dangerouslyDisableSandbox:true)`.

Anda dapat menonaktifkan pintu keluar ini dengan mengatur `"allowUnsandboxedCommands": false` dalam [sandbox settings](/docs/id/settings-reference#sandbox-settings) Anda. Dengan pintu keluar dinonaktifkan, Claude Code mengabaikan parameter `dangerouslyDisableSandbox`, dan setiap perintah Claude jalankan harus berjalan sandboxed kecuali Anda telah mencantumkannya dalam `excludedCommands`. Tab **Overrides** `/sandbox` menampilkan pengaturan ini sebagai **Strict sandbox mode**.

Mode strict sandbox berlaku untuk perintah yang Claude jalankan. Perintah yang Anda ketik sendiri di prompt [shell-mode `!`](/docs/id/interactive-mode#shell-mode-with-prefix) berjalan di luar sandbox kecuali sesi adalah salah satu dari ini:

* **Sesi [background](/docs/id/agent-view)**: mode strict sandbox mencakup perintah shell-mode juga
* **Sesi Linux dengan [`CLAUDE_CODE_SUBPROCESS_ENV_SCRUB`](/docs/id/env-vars#variables) diatur**: setiap perintah berjalan sandboxed, perintah shell-mode termasuk

Sebelum v2.1.260, mode strict sandbox sandboxed perintah shell-mode dalam setiap sesi.

<h4 id="temporary-directories">
  Direktori sementara
</h4>

Direktori temp sesi dapat ditulis di dalam sandbox secara default, bersama dengan direktori kerja. Kecuali Anda [disable filesystem isolation](#disable-filesystem-isolation), Claude Code menetapkan `$TMPDIR` ke direktori ini untuk perintah sandboxed, sehingga alat yang menulis file sementara bekerja tanpa konfigurasi tambahan.

Perintah unsandboxed mewarisi `$TMPDIR` shell Anda ketika diatur, sehingga sementara isolasi filesystem aktif, perintah sandboxed dan unsandboxed menyelesaikan `$TMPDIR` ke direktori yang berbeda. Jika shell Anda membiarkan `$TMPDIR` tidak diatur atau kosong, perintah unsandboxed yang mereferensikan `$TMPDIR` menerima override [`CLAUDE_CODE_TMPDIR`](/docs/id/env-vars) Anda, atau direktori temp sistem operasi ketika Anda belum menetapkan satu atau override adalah jalur panjang, sehingga variabel tidak berkembang menjadi string kosong. Untuk meneruskan file sementara di antara keduanya, tulis di bawah direktori kerja sebagai gantinya.

<h2 id="configure-sandboxing">
  Konfigurasi sandboxing
</h2>

Sesuaikan perilaku sandbox melalui file `settings.json` Anda. Lihat [Settings](/docs/id/settings-reference#sandbox-settings) untuk referensi konfigurasi lengkap.

Secara default, perintah sandboxed dapat menulis ke direktori kerja saat ini, direktori temp sesi, dan [direktori apa pun yang telah Anda tambahkan](/docs/id/permissions#additional-directories-grant-file-access-not-configuration) dengan `--add-dir`, `/add-dir`, atau `permissions.additionalDirectories`. Jika perintah subprocess seperti `kubectl`, `terraform`, atau `npm` perlu menulis di luar direktori tersebut, gunakan `sandbox.filesystem.allowWrite` untuk memberikan akses ke jalur tertentu:

```json theme={null}
{
  "sandbox": {
    "enabled": true,
    "filesystem": {
      "allowWrite": ["~/.kube", "/tmp/build"]
    }
  }
}
```

Jalur ini diberlakukan pada tingkat OS, sehingga semua perintah yang berjalan di dalam sandbox, termasuk proses anak mereka, menghormatinya. Ini adalah pendekatan yang direkomendasikan ketika alat memerlukan akses tulis ke lokasi tertentu, daripada mengecualikan alat dari sandbox sepenuhnya dengan `excludedCommands`.

Ketika Anda menentukan array filesystem yang sama dalam beberapa [settings scopes](/docs/id/settings#settings-precedence), Claude Code menggabungkannya, menggabungkan jalur dari setiap scope daripada mengganti array satu scope dengan yang lain.

Jika Anda mengecualikan sumber dengan [`--setting-sources`](/docs/id/cli-reference) pada CLI atau [`settingSources`](/docs/id/agent-sdk/claude-code-features#control-filesystem-settings-with-settingsources) dalam Agent SDK, Claude Code mengabaikan entri `sandbox.filesystem` nya, aturan izin `Edit` nya, dan aturan penolakan `Read` nya saat membangun konfigurasi sandbox. Memerlukan Claude Code v2.1.246 atau lebih baru.

Ketika Anda mengedit daftar filesystem ini selama sesi, Claude Code [menerapkan perubahan ke sesi yang sedang berjalan](/docs/id/settings#when-edits-take-effect), sehingga perintah sandboxed berikutnya berjalan di bawah jalur baru.

Awalan jalur mengontrol bagaimana jalur diselesaikan:

| Awalan                 | Arti                                                                                                | Contoh                                                                           |
| :--------------------- | :-------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------- |
| `/`                    | Jalur absolut dari akar filesystem                                                                  | `/tmp/build` tetap `/tmp/build`                                                  |
| `~/`                   | Relatif terhadap direktori home                                                                     | `~/.kube` menjadi `$HOME/.kube`                                                  |
| `./` atau tanpa awalan | Relatif terhadap akar proyek untuk pengaturan proyek, atau ke `~/.claude` untuk pengaturan pengguna | `./output` dalam `.claude/settings.json` diselesaikan ke `<project-root>/output` |

Sintaks ini berbeda dari [Read and Edit permission rules](/docs/id/permissions#read-and-edit), yang menggunakan `//path` untuk absolut dan `/path` untuk relatif proyek. Jalur filesystem sandbox menggunakan konvensi standar: `/tmp/build` adalah absolut. Untuk bagaimana Claude Code memperlakukan garis miring di akhir atau wildcard dalam jalur ini, lihat [Sandbox path prefixes](/docs/id/settings-reference#sandbox-path-prefixes).

Anda juga dapat menolak akses tulis atau baca menggunakan `sandbox.filesystem.denyWrite` dan `sandbox.filesystem.denyRead`, dan mengizinkan kembali jalur tertentu dalam wilayah yang ditolak menggunakan `sandbox.filesystem.allowRead`. Ketika aturan baca tumpang tindih, jalur yang lebih spesifik menang:

| Aturan contoh                                             | Hasil                                                                                                                                                                                                                             |
| :-------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `"denyRead": ["~/"]` dengan `"allowRead": ["~/projects"]` | `~/projects` dapat dibaca dan sisa direktori home tetap diblokir. Izin yang lebih sempit membuka kembali bagian itu dari wilayah yang ditolak                                                                                     |
| `"allowRead": ["~/"]` dengan `"denyRead": ["~/.env"]`     | `~/.env` tetap diblokir dan sisa direktori home dapat dibaca. Penolakan yang tepat berlaku di dalam izin yang lebih luas, jadi izin yang luas tidak dapat secara diam-diam membuka kembali rahasia                                |
| `"allowRead": ["~/"]` dengan `"denyRead": ["~/**/.env"]`  | Setiap `.env` di bawah direktori home tetap diblokir dan sisanya dapat dibaca. [Wildcard deny](/docs/id/settings-reference#sandbox-path-prefixes) berlaku di dalam izin yang lebih luas dengan cara yang sama seperti jalur yang tepat |

Contoh di bawah memblokir pembacaan dari seluruh direktori home sambil tetap memungkinkan pembacaan dari proyek saat ini. Tempatkan di `.claude/settings.json` proyek Anda, karena jalur relatif `.` diselesaikan ke akar proyek hanya ketika konfigurasi berada dalam pengaturan proyek:

```json theme={null}
{
  "sandbox": {
    "enabled": true,
    "filesystem": {
      "denyRead": ["~/"],
      "allowRead": ["."]
    }
  }
}
```

Jika Anda menempatkan konfigurasi yang sama dalam `~/.claude/settings.json`, `.` akan diselesaikan ke `~/.claude` sebagai gantinya, dan file proyek akan tetap diblokir oleh aturan `denyRead`.

Untuk menolak perintah sandboxed akses baca ke direktori home dan volume yang dipasang sambil menjaga direktori kerja tetap dapat dibaca, atur [`permissions.blockReadsOutsideWorkingDirectories`](/docs/id/settings-reference#permissions-blockreadsoutsideworkingdirectories) sebagai gantinya dari menulis aturan jalur.

<h3 id="disable-filesystem-isolation">
  Nonaktifkan isolasi filesystem
</h3>

Atur `sandbox.filesystem.disabled` ke `true` untuk melewati isolasi filesystem sambil menjaga isolasi jaringan. Contoh di bawah mematikan isolasi filesystem sambil menjaga daftar izin domain jaringan:

```json theme={null}
{
  "sandbox": {
    "enabled": true,
    "filesystem": {
      "disabled": true
    },
    "network": {
      "allowedDomains": ["github.com", "*.npmjs.org"]
    }
  }
}
```

Sandbox memiliki dua lapisan independen: [filesystem isolation](#filesystem-isolation) mengontrol jalur mana yang dapat dibaca dan ditulis perintah sandboxed, dan [network isolation](#network-isolation) mengontrol domain mana yang dapat mereka jangkau. Dengan lapisan filesystem mati, perintah sandboxed mendapatkan akses baca dan tulis tanpa batas ke filesystem host, sementara egress jaringan mereka tetap terbatas pada domain yang Anda izinkan. Matikan lapisan ketika Anda sandbox untuk mengontrol tempat perintah terhubung daripada apa yang mereka tulis.

Pengaturan ini mati secara default dan berlaku pada platform tempat sandbox berjalan: macOS, Linux, dan WSL2. Memerlukan Claude Code v2.1.216 atau lebih baru.

<Warning>
  Dengan isolasi filesystem mati dan perintah auto-allowed, perintah sandboxed dapat menulis file yang kemudian dijalankan atau dibaca perintah lain, seperti file startup shell, executable di `$PATH`, atau `~/.claude/settings.json`, dan menggunakannya untuk memperluas akses mereka sendiri pada run berikutnya. Atur `filesystem.disabled` ke `true` hanya untuk workload yang Anda percayai tidak akan meningkatkan akses mereka sendiri. Mengunci domain jaringan dengan [`allowManagedDomainsOnly`](#keep-developers-from-widening-the-policy) mempersempit risiko tetapi tidak menghilangkannya, karena kunci itu hanya berlaku untuk perintah yang berjalan di dalam sandbox.
</Warning>

<h4 id="which-settings-can-disable-it">
  Pengaturan mana yang dapat menonaktifkannya
</h4>

Karena mematikan isolasi filesystem memperluas apa yang dapat dilakukan perintah sandboxed, Claude Code menghormati `filesystem.disabled` dari sumber pengaturan ini saja:

* Pengaturan pengguna, pengaturan terkelola, dan bendera CLI `--settings` dapat mengaturnya. Pengaturan proyek dalam `.claude/settings.json` dan `.claude/settings.local.json` tidak dapat, jadi proyek yang diperiksa tidak dapat mematikan isolasi filesystem.
* Ketika pengaturan terkelola mengonfigurasi `sandbox.filesystem` sama sekali, atau mencantumkan entri `sandbox.credentials.files` apa pun dengan `"mode": "deny"`, hanya pengaturan terkelola yang dapat mengatur kunci. Ini menjaga pembatasan filesystem yang diterapkan administrator tetap berlaku; untuk melonggarkan penerapan seperti itu, atur `"disabled": true` dalam pengaturan terkelola.
* Ketika [`CLAUDE_CODE_SUBPROCESS_ENV_SCRUB`](/docs/id/env-vars) diatur, Claude Code mengabaikan `filesystem.disabled` dari setiap sumber, termasuk pengaturan terkelola, dan menjaga isolasi filesystem tetap aktif.

Apakah entri `credentials.files` terkelola menyematkan `filesystem.disabled`, mengunci kunci ke pengaturan terkelola sehingga pengembang tidak dapat mematikan isolasi filesystem, tergantung pada `mode` entri dan apa yang terjadi pada entri ketika sandbox dimulai:

| Entri terkelola                                                                                                    | Menyematkan `filesystem.disabled`  | Apa yang melindungi file ketika isolasi mati                                                                                                        |
| ------------------------------------------------------------------------------------------------------------------ | ---------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------- |
| `"mode": "deny"`                                                                                                   | Ya                                 | Tidak ada: blok baca adalah bagian dari lapisan filesystem                                                                                          |
| `"mode": "mask"`, diterapkan sebagai mask                                                                          | Tidak                              | Masking itu sendiri: [sentinel copy dan proxy](#mask-credential-files) di Linux dan WSL2, aturan baca sandbox itu sendiri di macOS                  |
| `"mode": "mask"`, [jatuh kembali ke `deny`](#mask-credential-files) saat setup                                     | Tidak                              | Tidak ada, sama seperti `deny`. Daftarkan jalur yang tidak dapat di-mask, seperti direktori, sebagai entri `deny` eksplisit, yang menyematkan kunci |
| `"mode": "mask"`, [terdegradasi ke `deny` oleh validasi](/docs/id/managed-settings#invalid-entries-in-managed-settings) | Ya, seperti entri `deny` eksplisit | Tidak ada, sama seperti `deny`                                                                                                                      |

Fallback terjadi ketika sandbox dimulai, setelah Claude Code telah membaca pengaturan yang pemeriksaan pin berjalan, jadi entri yang jatuh kembali tidak pernah menyematkan. Validasi menulis ulang entri yang tidak valid ke `deny` saat pengaturan dimuat, jadi entri yang terdegradasi menyematkan seperti yang Anda tulis sebagai `deny`.

<h4 id="what-changes-when-filesystem-isolation-is-off">
  Apa yang berubah ketika isolasi filesystem mati
</h4>

Mengatur `filesystem.disabled` mengangkat perlindungan yang lapisan filesystem itu sendiri berlakukan. Perlindungan yang lapisan lain berlakukan terus berlaku:

| Perlindungan                                                                           | Dengan isolasi filesystem mati                                                                                                                                          |
| -------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `filesystem.denyRead` dan [`credentials.files`](#protect-credentials) blok baca `deny` | Tidak diberlakukan. Lapisan filesystem menerapkan keduanya                                                                                                              |
| `credentials.envVars` entri `deny` dan `mask`                                          | Diberlakukan. Scrubbing variabel lingkungan independen dari lapisan filesystem                                                                                          |
| [`credentials.files` entri `mask`](#mask-credential-files) diterapkan sebagai mask     | Diberlakukan: masking independen dari lapisan filesystem. Entri yang [jatuh kembali ke `deny`](#mask-credential-files) tidak diberlakukan, seperti entri `deny` apa pun |

Dua hal lain berubah:

* Perintah sandboxed mewarisi `$TMPDIR` shell Anda daripada direktori temp sesi, karena setiap direktori temp dapat ditulis dan Claude Code tidak lagi mengarahkan perintah ke yang sesi.

  Di Linux variabel sering tidak diatur dalam shell induk. Panduan alat Bash memberi tahu Claude untuk membuat direktori scratch dengan `mktemp -d` daripada mengandalkan `$TMPDIR`.
* [`autoAllowBashIfSandboxed`](/docs/id/settings-reference#sandbox-autoallowbashifsandboxed) masih default ke `true`, jadi perintah sandboxed terus berjalan tanpa prompt. Atur ke `false` untuk meminta perintah sandboxed.

<h3 id="protect-credentials">
  Lindungi kredensial
</h3>

Pengaturan `sandbox.credentials` mendeklarasikan file kredensial dan variabel lingkungan yang harus dilindungi dari perintah sandboxed. Setiap entri memberi nama jalur file atau variabel lingkungan dan `mode`. Blok `credentials` khusus menjaga aturan kredensial dikelompokkan bersama dan terpisah dari aturan filesystem umum.

Untuk entri dengan `"mode": "deny"`, jalur file ditolak untuk pembacaan di dalam sandbox, pembatasan yang sama yang diterapkan `filesystem.denyRead`, dan variabel lingkungan tidak diatur sebelum setiap perintah sandboxed berjalan. Perlindungan file adalah bagian dari lapisan filesystem, jadi tidak berlaku jika Anda [menonaktifkan isolasi filesystem](#disable-filesystem-isolation); perlindungan variabel lingkungan masih berlaku.

Contoh di bawah memblokir pembacaan file kredensial AWS dan direktori SSH serta menghapus `GITHUB_TOKEN` dan `NPM_TOKEN` dari lingkungan perintah sandboxed:

```json theme={null}
{
  "sandbox": {
    "enabled": true,
    "credentials": {
      "files": [
        { "path": "~/.aws/credentials", "mode": "deny" },
        { "path": "~/.ssh", "mode": "deny" }
      ],
      "envVars": [
        { "name": "GITHUB_TOKEN", "mode": "deny" },
        { "name": "NPM_TOKEN", "mode": "deny" }
      ]
    }
  }
}
```

Entri variabel lingkungan dan entri file juga menerima `"mode": "mask"`, dijelaskan di bawah [Mask credentials](#mask-credentials).

Jalur file mengikuti [aturan awalan](/docs/id/settings-reference#sandbox-path-prefixes) yang sama dengan pengaturan `sandbox.filesystem.*`.

Claude Code menggabungkan entri `deny` dari setiap [settings scope](/docs/id/settings#settings-precedence) yang dimuat sesi. Entri `deny` hanya pernah mempersempit akses, jadi scope apa pun dapat menambahkan satu, tetapi tidak ada scope yang dapat menghapus satu yang ditambahkan scope lain.

Ketika Anda [mengecualikan sumber pengaturan](#configure-sandboxing):

* **Pengaturan proyek atau lokal**: Claude Code tidak menerapkan entri `credentials` apa pun dari mereka. Memerlukan Claude Code v2.1.246 atau lebih baru.
* **Pengaturan pengguna**: Claude Code masih menerapkan entri `deny` dalam `~/.claude/settings.json` dan menjaga [entri `mask` file](#mask-credential-files) nya sebagai pembatasan, tetapi menghapus [entri `mask` variabel lingkungan](#mask-environment-variables) nya.

Tidak ada daftar penolakan kredensial bawaan, jadi hanya file dan variabel yang Anda daftarkan yang dibatasi.

`sandbox.credentials` mempengaruhi perintah Bash sandboxed saja. Untuk menghapus kredensial dari semua subprocess terlepas dari sandboxing, atur [`CLAUDE_CODE_SUBPROCESS_ENV_SCRUB`](/docs/id/env-vars).

<h3 id="mask-credentials">
  Mask credentials
</h3>

Masking berjalan lebih jauh daripada entri `deny` di bawah [Protect credentials](#protect-credentials). Daripada memblokir kredensial, Claude Code menunjukkan perintah sandboxed placeholder, sentinel, dan [sandbox proxy](#network-isolation) menukar nilai asli pada permintaan keluar ke host yang Anda izinkan. Untuk file, substitusi adalah perilaku Linux dan WSL2; [macOS memblokir file sebagai gantinya](#mask-credential-files).

<h4 id="mask-environment-variables">
  Mask environment variables
</h4>

`"mode": "mask"` melindungi kredensial sambil membuat alat yang mengotentikasi dengannya tetap berfungsi. `deny` menghapus variabel sepenuhnya, yang juga merusak alat yang membutuhkannya, seperti `gh` atau `npm`. Memerlukan Claude Code v2.1.199 atau lebih baru.

Dengan `mask`, perintah sandboxed melihat nilai sentinel per-sesi daripada yang asli. Setiap entri `mask` dapat mencantumkan `injectHosts`, host tempat nilai asli diizinkan untuk menjangkau. Ketika permintaan meninggalkan sandbox untuk salah satu dari mereka, [sandbox proxy](#network-isolation) mengganti sentinel dengan nilai asli. Perintah dan apa pun yang dicatat tidak pernah memegang kredensial asli, tetapi permintaannya tetap mengotentikasi.

Proxy mengganti kredensial di dalam konten permintaan, jadi harus melihatnya. Atur [`network.tlsTerminate`](/docs/id/settings-reference#sandbox-network-tlsterminate) sehingga proxy menghentikan TLS itu sendiri.

Tanpa itu, masking gagal tanpa mengekspos apa pun: perintah masih hanya melihat sentinel, tetapi sentinel mencapai server tidak berubah dan otentikasi gagal. Claude Code melaporkan kesalahan konfigurasi ini saat startup.

Substitusi mencakup header dan badan permintaan. Permintaan yang mengotentikasi dengan tanda tangan yang berasal dari kredensial, daripada kredensial itu sendiri, perlu ditandatangani ulang di proxy; [Re-sign AWS requests](#re-sign-aws-requests) mencakup bagaimana itu bekerja untuk AWS.

Proxy menyuntik hanya pada koneksi yang [daftar domain allowlist](#network-isolation) mengakui, jadi setiap tujuan `injectHosts` juga harus dapat dijangkau melalui `network.allowedDomains`.

Contoh di bawah mask dua token. `GH_TOKEN` diganti hanya pada permintaan ke `api.github.com`, sementara `NPM_TOKEN` tidak memiliki `injectHosts` dan diganti pada permintaan ke setiap host dalam `network.allowedDomains`.

```json theme={null}
{
  "sandbox": {
    "enabled": true,
    "network": {
      "tlsTerminate": {},
      "allowedDomains": ["*.github.com", "registry.npmjs.org"]
    },
    "credentials": {
      "envVars": [
        { "name": "GH_TOKEN", "mode": "mask", "injectHosts": ["api.github.com"] },
        { "name": "NPM_TOKEN", "mode": "mask" }
      ]
    }
  }
}
```

<span id="ipv6-destinations-in-injecthosts" />Ejakan tujuan IPv6 secara berbeda dalam dua daftar, karena setiap daftar memiliki matcher-nya sendiri:

* **`network.allowedDomains`**: [bentuk bracketed yang digunakan daftar domain](#ipv6-addresses-in-domain-lists), seperti `"[::1]"`. Proxy memeriksa daftar ini untuk mengakui koneksi.
* **`injectHosts`**: alamat telanjang dalam bentuk kanonik terkompresinya, seperti `"::1"` atau `"2001:db8::1"`. Proxy mencocokkan setiap entri terhadap alamat tujuan koneksi yang telanjang, mengabaikan port, jadi ejakan bracketed, zone-ID, atau terkompresi secara berbeda tidak pernah cocok dan proxy tidak pernah menyuntik kredensial di sana.

`claude doctor` menandai entri `injectHosts` yang tidak pernah dapat cocok dengan peringatan `Sandbox credential injectHosts entries can never match their destination`. Pemeriksaan ini memerlukan Claude Code v2.1.229 atau lebih baru.

Tidak seperti `deny`, masking mengotorisasi proxy untuk mengirim kredensial asli Anda ke host yang terdaftar, jadi Claude Code menghormatinya hanya dari pengaturan yang Anda atau administrator Anda kontrol: pengaturan pengguna, pengaturan terkelola, dan bendera CLI `--settings`. Claude Code mengabaikan entri `mask` dalam `.claude/settings.json` atau `.claude/settings.local.json` repositori. Dalam file tersebut juga mengabaikan `network.tlsTerminate` dan [`credentials.allowPlaintextInject`](/docs/id/settings-reference#sandbox-credentials-allowplaintextinject), pengaturan yang memungkinkan proxy menyuntik kredensial ke permintaan yang tidak terenkripsi. Jika Anda [mengecualikan pengaturan pengguna](#configure-sandboxing), Claude Code menghapus entri `mask` variabel lingkungan dalam `~/.claude/settings.json` juga.

Ketika administrator Anda mengirimkan entri `mask`, `network.tlsTerminate`, atau `credentials.allowPlaintextInject` melalui pengaturan terkelola server, mereka dihitung sebagai [pengaturan yang memerlukan persetujuan](/docs/id/server-managed-settings#security-approval-dialogs).

Ketika variabel yang sama terdaftar dengan `deny` dalam scope apa pun, `deny` mengambil prioritas.

Masking mengganti seluruh nilai variabel secara default, yang cocok untuk token telanjang. Bidang entri opsional, yang memerlukan Claude Code v2.1.224 atau lebih baru, menangani nilai dengan struktur:

* `extract`: ekspresi reguler yang Claude Code terapkan di seluruh nilai, mengganti hanya teks yang ditangkap oleh grup 1 dari setiap kecocokan, jadi alat yang mengurai nilai, seperti string koneksi `DATABASE_URL`, masih berfungsi di dalam sandbox. Pola harus berisi setidaknya satu grup penangkapan.
* `onExtractNoMatch` mengontrol apa yang terjadi ketika pola tidak cocok dengan apa pun:
  * `warn`, default, memperingatkan dan melewatkan variabel tanpa mask
  * `deny` membatalkan pengaturan variabel di dalam sandbox
  * `error` menghentikan setup sandbox sampai Anda memperbaiki konfigurasi
* `decode: "jwt"`: untuk variabel yang menyimpan JSON Web Token (JWT). Claude Code memverifikasi nilai adalah JWT dan menggantinya dengan token palsu yang valid secara struktural, jadi kode di dalam sandbox yang mendekode token terus berfungsi. Tambahkan `maskClaims` untuk mencantumkan klaim payload tingkat atas untuk di-mask secara individual daripada mengganti seluruh token; klaim lainnya tetap dapat dibaca. Ketika nilai tidak memverifikasi sebagai JWT, atau tidak ada klaim yang terdaftar cocok, Claude Code melewatkan variabel tanpa mask dengan peringatan. `decode` tidak dapat digabungkan dengan `extract`.

Lihat [baris `credentials.envVars[]` dalam referensi pengaturan](/docs/id/settings-reference#sandbox-settings) untuk daftar bidang lengkap.

<h4 id="re-sign-aws-requests">
  Re-sign AWS requests
</h4>

Permintaan AWS membawa tanda tangan SigV4 di atas konten permintaan, jadi mask `AWS_ACCESS_KEY_ID` dan `AWS_SECRET_ACCESS_KEY` bersama-sama. Proxy mendeteksi permintaan SigV4 oleh sentinel kunci akses dan menandatangani ulang setelah mengganti nilai asli. Masking rahasia saja meninggalkan permintaan ditandatangani dengan placeholder, yang proxy tidak dapat mendeteksi, jadi mereka gagal di AWS; Claude Code memperingatkan tentang kasus ini saat startup, tetapi tidak ketika hanya ID kunci akses yang di-mask. Permintaan yang terdeteksi proxy tidak dapat menandatangani ulang, seperti yang kehilangan header `x-amz-date` nya, gagal dengan kesalahan proxy daripada mencapai server dengan tanda tangan yang rusak.

Claude Code menghubungkan variabel `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY`, dan `AWS_SESSION_TOKEN` konvensional ke satu kredensial secara otomatis ketika Anda mask nilai seluruhnya. Jika kredensial AWS Anda berada dalam variabel dengan nama lain, kelompokkan sendiri dengan [`credentials.awsPairs`](/docs/id/settings-reference#sandbox-credentials-awspairs), yang memerlukan Claude Code v2.1.224 atau lebih baru. Contoh ini menambahkan pasangan ke konfigurasi yang sudah mask `MY_KEY_ID`, `MY_SECRET_KEY`, dan `MY_SESSION_TOKEN` seluruh-nilai, seperti dalam [konfigurasi masking di atas](#mask-environment-variables):

```json theme={null}
{
  "sandbox": {
    "credentials": {
      "awsPairs": [
        {
          "accessKeyIdVar": "MY_KEY_ID",
          "secretAccessKeyVar": "MY_SECRET_KEY",
          "sessionTokenVar": "MY_SESSION_TOKEN"
        }
      ]
    }
  }
}
```

Setiap entri mengikuti aturan ini:

* `accessKeyIdVar` dan `secretAccessKeyVar` memberi nama entri `envVars` yang di-mask yang menyimpan ID kunci akses dan kunci rahasia. `sessionTokenVar` opsional memberi nama entri yang menyimpan token sesi untuk kredensial sementara; ketika diatur, proxy mengirim token asli sebagai `x-amz-security-token` pada permintaan yang ditandatangani ulang.
* Setiap variabel yang dinamai harus entri `mask` yang mask seluruh nilainya, tanpa `extract` atau `decode`.
* Proxy menandatangani ulang permintaan pada host yang terdaftar dalam entri ID kunci akses `injectHosts`.
* Penamaan variabel konvensional apa pun dalam pasangan mengganti pasangan otomatis.

Seperti entri `mask`, `awsPairs` dihormati hanya dari pengaturan pengguna, pengaturan terkelola, dan bendera CLI `--settings`.

Tiga bentuk permintaan AWS membawa tanda tangan yang proxy tidak dapat menghitung ulang. Ketika permintaan seperti itu ditandatangani dengan placeholder pasangan yang di-mask, proxy gagalkan daripada teruskan tanda tangan yang rusak; permintaan yang ditandatangani dengan kredensial yang tidak di-mask tidak pernah terpengaruh. Pengaturan [`credentials.sigv4`](/docs/id/settings-reference#sandbox-credentials-sigv4), yang memerlukan Claude Code v2.1.224 atau lebih baru, melonggarkan ini per bentuk: mengatur kunci bentuk ke `passthrough` meneruskan permintaan dengan tanda tangan yang diturunkan placeholder-nya, jadi alat yang memanggil menerima penolakan AWS sendiri daripada kesalahan proxy. Seperti `awsPairs`, `sigv4` dihormati hanya dari pengaturan pengguna, pengaturan terkelola, dan bendera CLI `--settings`.

| Bentuk permintaan              | Kunci `sigv4` | Mengapa proxy tidak dapat menandatangani ulang                                                                        |
| :----------------------------- | :------------ | :-------------------------------------------------------------------------------------------------------------------- |
| unggahan streaming aws-chunked | `streaming`   | Tanda tangan per-chunk berantai dari tanda tangan seed, jadi menandatangani ulang akan memerlukan menulis ulang badan |
| URL yang sudah ditandatangani  | `presigned`   | Tanda tangan berada di URL itu sendiri, tanpa header `Authorization`                                                  |
| Tanda tangan asimetris SigV4A  | `sigv4a`      | Tidak ada HMAC kunci bersama untuk menghitung ulang                                                                   |

<h4 id="mask-credential-files">
  Mask credential files
</h4>

Entri file juga menerima `"mode": "mask"`, yang memerlukan Claude Code v2.1.221 atau lebih baru. Apa yang dilihat perintah sandboxed tergantung pada platform:

* **Linux dan WSL2**: perintah sandboxed membaca salinan sentinel file, pengganti yang rahasianya diganti dengan nilai placeholder, dan [sandbox proxy](#network-isolation) mengganti nilai asli pada egress.
* **macOS**: perintah sandboxed tidak dapat membaca file yang terdaftar sama sekali. Claude Code tidak membangun salinan sentinel dan tidak mengganti apa pun pada egress, jadi alat yang mengotentikasi dengan file tidak berfungsi di dalam sandbox, efek yang sama seperti `deny`. Tidak seperti entri `deny`, blok baca berlaku bahkan ketika Anda [menonaktifkan isolasi filesystem](#disable-filesystem-isolation).

Di setiap platform, Claude Code menerapkan persyaratan [`network.tlsTerminate`](/docs/id/settings-reference#sandbox-network-tlsterminate) dan `injectHosts` dengan cara yang sama seperti untuk [masked environment variables](#mask-environment-variables), dan mengabaikan pengaturan repositori dengan cara yang sama. Jika Anda [mengecualikan pengaturan pengguna](#configure-sandboxing), Claude Code menjaga entri `mask` file dalam `~/.claude/settings.json` sebagai pembatasan, tetapi entri tidak lagi mengotorisasi proxy untuk mengganti nilai asli.

Contoh di bawah mask token GitHub yang disimpan dalam `~/.config/gh/hosts.yml`; pola `extract`, dijelaskan di bawah, memberi tahu Claude Code bagian file mana yang rahasianya. Di Linux dan WSL2, perintah sandboxed yang membaca file mendapatkan sentinel sebagai pengganti token, dan proxy mengganti token asli pada permintaan ke `api.github.com`:

```json theme={null}
{
  "sandbox": {
    "enabled": true,
    "network": {
      "tlsTerminate": {},
      "allowedDomains": ["*.github.com"]
    },
    "credentials": {
      "files": [
        {
          "path": "~/.config/gh/hosts.yml",
          "mode": "mask",
          "extract": "oauth_token:\\s*(\\S+)",
          "injectHosts": ["api.github.com"]
        }
      ]
    }
  }
}
```

Untuk mengkonfirmasi mask aktif, minta Claude menjalankan `cat ~/.config/gh/hosts.yml` dalam perintah sandboxed: di Linux dan WSL2 output menunjukkan nilai sentinel sebagai pengganti token, dan di macOS pembacaan gagal sebagai gantinya.

Di Linux dan WSL2, pola `extract` adalah apa yang menjaga sisa `hosts.yml` dapat dibaca. Claude Code menerapkan ekspresi reguler di seluruh file dan mengganti hanya teks yang ditangkap oleh grup 1 dari setiap kecocokan, jadi `gh` masih mengurai konfignya dan hanya token yang placeholder. Gunakan `extract` untuk file terstruktur apa pun yang alat urai, seperti `.netrc`, JSON, atau YAML; pola harus berisi setidaknya satu grup penangkapan. Tanpa `extract`, Claude Code mengganti seluruh konten file dengan satu nilai sentinel, yang cocok untuk file yang menyimpan satu rahasia telanjang dan tidak ada yang lain.

Untuk file yang menyimpan JSON Web Token (JWT), atur `decode: "jwt"` daripada, atau bersama dengan, `extract`. `decode` memerlukan Claude Code v2.1.224 atau lebih baru. Claude Code menemukan kandidat JWT dengan pola bawaan, atau dengan pola `extract` Anda ketika diatur, memverifikasi setiap kandidat adalah JWT, dan menggantinya dengan token palsu yang valid secara struktural, jadi kode yang mendekode token di dalam sandbox terus berfungsi. Tambahkan `maskClaims` untuk mask hanya klaim payload tingkat atas yang dinamai di dalam setiap token yang diverifikasi dan biarkan klaim lainnya dapat dibaca. Ketika tidak ada kandidat yang memverifikasi, atau tidak ada klaim yang dinamai cocok, bidang `onExtractNoMatch` di bawah mengatur hasil, seperti yang dilakukan untuk pola yang tidak cocok dengan apa pun.

Dua bidang opsional menyempurnakan bagaimana perilaku pencocokan. Keduanya berlaku hanya ketika `mode` adalah `mask` dan `extract` atau `decode` diatur. Di macOS, Claude Code menerapkan entri `mask` sebagai `deny` sebelum pola berjalan kapan pun isolasi filesystem aktif, jadi bidang ini, dan hasil no-match di bawah, berlaku di sana hanya ketika [isolasi filesystem mati](#disable-filesystem-isolation):

* `onExtractNoMatch` mengontrol apa yang terjadi ketika pencocokan menemukan tidak ada yang di-mask dalam file:

  * `warn`, default, memperingatkan dan melewati entri, jadi perintah sandboxed dapat membaca file asli tanpa mask. Default cocok untuk kredensial yang mungkin secara sah tidak ada; jika rahasia mungkin ada tetapi pola mungkin melewatkannya, gunakan `deny`
  * `deny` membuat file tidak dapat dibaca sebagai gantinya
  * `error` menghentikan setup sandbox sampai Anda memperbaiki konfigurasi

  Claude Code memperlakukan `deny` sebagai `error` kapan pun blok baca tidak akan diberlakukan: ketika Anda [menonaktifkan isolasi filesystem](#disable-filesystem-isolation), dan ketika entri `filesystem.allowRead` dari sumber pengaturan apa pun membuka kembali jalur file.
* `maskDuplicates` juga mengganti salinan verbatim dari setiap nilai kredensial yang di-mask, tangkapan `extract` atau token yang diverifikasi `decode`, ditemukan di luar rentang yang cocok, untuk rahasia yang diulang di mana pencocokan tidak menjangkau. Ini mencocokkan substring mentah, jadi nilai pendek atau umum akan diganti di mana pun muncul; cadangkan untuk rahasia panjang, high-entropy. Default: false.

`mask` berlaku untuk satu file, jadi daftarkan setiap file kredensial secara individual. Claude Code jatuh kembali ke `deny` untuk entri `mask` yang tidak dapat di-mask dengan aman: jalur direktori, pola glob, file lebih besar dari 8 MiB, atau file yang bukan teks UTF-8. Tulis direktori sebagai entri `deny` eksplisit sebagai gantinya; tabel di bawah [Which settings can disable it](#which-settings-can-disable-it) mencakup apakah setiap bentuk menyematkan `filesystem.disabled` dan bagaimana perilakunya dengan isolasi filesystem mati.

<h2 id="how-sandboxing-works">
  Cara sandboxing bekerja
</h2>

<h3 id="filesystem-isolation">
  Isolasi filesystem
</h3>

Alat Bash sandboxed membatasi akses sistem file ke direktori tertentu:

* **Perilaku penulisan default**: akses baca dan tulis ke direktori kerja saat ini dan subdirektorinya, direktori apa pun yang telah Anda tambahkan dengan `--add-dir`, `/add-dir`, atau [`permissions.additionalDirectories`](/docs/id/settings-reference#permissions-additionaldirectories), ditambah direktori temp sesi yang ditunjuk oleh `$TMPDIR`
* **Perilaku pembacaan default**: akses baca ke seluruh komputer, kecuali direktori tertentu yang ditolak. Perhatikan bahwa default ini masih memungkinkan pembacaan file kredensial seperti `~/.aws/credentials` dan `~/.ssh/`. Gunakan [`sandbox.credentials`](#protect-credentials) untuk memblokir pembacaan file-file ini dan membatalkan penetapan variabel lingkungan rahasia, atau tambahkan jalur ke `denyRead`.
* **Akses terblokir**: tidak dapat memodifikasi file di luar direktori kerja, direktori yang ditambahkan, dan direktori temp sesi tanpa izin eksplisit, termasuk file konfigurasi shell seperti `~/.bashrc` dan binari sistem di `/bin/`
* **Git worktrees**: ketika direktori kerja adalah [linked git worktree](/docs/id/worktrees), sandbox juga memungkinkan penulisan ke direktori `.git` bersama dari repositori utama sehingga perintah seperti `git commit` dapat memperbarui refs dan indeks. Penulisan ke `hooks/` dan `config` di dalam direktori tersebut tetap ditolak.
* **Dapat dikonfigurasi**: tentukan jalur yang diizinkan dan ditolak khusus melalui pengaturan

Untuk melewati isolasi filesystem sepenuhnya sambil mempertahankan isolasi jaringan, atur [`sandbox.filesystem.disabled`](#disable-filesystem-isolation).

<h3 id="protected-paths">
  Jalur yang dilindungi
</h3>

Di dalam direktori yang dapat ditulis oleh perintah sandboxed, sandbox masih menolak penulisan ke file yang dimuat konfigurasi dan kode Claude Code. Perintah yang dapat mengedit file-file tersebut dapat memberikan izin kepada dirinya sendiri, atau menambahkan hook atau server MCP yang dijalankan Claude Code di luar sandbox. Sistem izin memiliki [jalur yang dilindungi](/docs/id/permission-modes#protected-paths) sendiri, yang mengontrol apa yang disetujui Claude Code sebelum alat berjalan; daftar sandbox berlaku untuk perintah yang sudah berjalan. Ini mencakup empat kelompok jalur:

* **Di direktori kerja Anda dan direktori di atasnya**: file pengaturan `.claude`, direktori `.claude/skills`, `.claude/agents`, `.claude/commands`, dan `.claude/hooks`, `.mcp.json`, dan file yang dijalankan Claude Code sendiri, seperti `.claude/workflows` dan `.claude/scheduled_tasks.json`
* **Di direktori kerja Anda saja**: file startup shell seperti `.bashrc` dan `.zshrc`, `.gitconfig`, direktori `.vscode` dan `.idea`, dan `hooks` dan `config` di dalam `.git`
* **File yang akan mengubah direktori kerja Anda menjadi repositori git bare**: `HEAD`, `objects`, dan `refs` di tingkat atas, ditambah `config` dan `hooks` di sana ketika `HEAD` berada di sebelahnya. File bernama `config` ditolak bahkan tanpa `HEAD`. Di Linux dan WSL2, sandbox menghapus file `HEAD` tingkat atas atau direktori `objects` atau `refs` yang muncul saat perintah sandboxed berjalan
* **Di `~/.claude`, atau direktori yang ditunjuk `CLAUDE_CONFIG_DIR`**: sebagian besar isinya, ditambah `~/.claude.json` dan penyimpanan kredensial `.credentials.json`

Jika symlink muncul di jalur file pengaturan yang dilindungi selama sesi, sandbox juga menolak penulisan ke file yang ditunjuknya, mulai dari perintah berikutnya.

Tidak ada cara untuk mengecualikan salah satu jalur ini: entri `allowWrite` atau aturan izin `Edit` yang mencakup jalur tidak menghilangkan perlindungan. Satu-satunya cara untuk mematikan perlindungan adalah [`filesystem.disabled`](#disable-filesystem-isolation), yang mematikan isolasi filesystem untuk setiap jalur. Untuk melihat sebagian besar jalur ini diselesaikan untuk mesin Anda, jalankan `/sandbox` dan buka tab **Config**, yang mencantumnya di bawah **Denied within allowed**, dicampur dengan entri `denyWrite` Anda sendiri.

Jika `git merge` atau `git checkout` gagal dengan `unable to unlink old` pada salah satu jalur ini, lihat [Troubleshooting](#troubleshooting).

<h3 id="network-isolation">
  Isolasi jaringan
</h3>

Akses jaringan dikendalikan melalui server proxy yang berjalan di luar sandbox:

* **Pembatasan domain**: Claude Code tidak memungkinkan domain apa pun secara default. Pertama kali perintah memerlukan domain baru, Claude Code meminta persetujuan; dalam [mode otomatis](/docs/id/permission-modes#eliminate-prompts-with-auto-mode), Claude malah menamai host yang dibutuhkan perintah pada perintah itu sendiri, per [Domain yang diizinkan per perintah](#per-command-allowed-domains-in-auto-mode).
* **Pilihan persetujuan**: jika Anda memilih Ya saat diminta, Claude Code memungkinkan host untuk sisa sesi saat ini dan tidak meminta lagi untuk koneksi nanti ke host yang sama. Jika Anda memilih "Ya, dan jangan tanya lagi", Claude Code menyimpan aturan izin `WebFetch(domain:...)` ke [pengaturan lokal](/docs/id/permissions#permission-system) Anda, sehingga host tetap diizinkan di sesi mendatang.
* **Domain yang diizinkan sebelumnya**: izinkan domain sebelumnya dengan [`allowedDomains`](/docs/id/settings-reference#sandbox-network-alloweddomains) untuk menghindari prompt sepenuhnya. Claude Code juga memungkinkan domain sebelumnya dari aturan izin `WebFetch(domain:...)`, seperti dijelaskan dalam [Aturan izin](#permission-rules).
* **Allowlist ketat**: jika Anda mengatur [`strictAllowlist`](/docs/id/settings-reference#sandbox-network-strictallowlist) ke `true` dalam pengaturan pengguna, terkelola, atau CLI `--settings`, Claude Code menolak akses perintah sandboxed ke host apa pun di luar allowlist alih-alih meminta. Allowlist adalah yang sama dengan yang diminta sandbox sebaliknya: `allowedDomains` ditambah domain dari aturan izin `WebFetch(domain:...)`, atau hanya entri pengaturan terkelola ketika `allowManagedDomainsOnly` diatur. Claude Code memberlakukan ini hanya untuk perintah sandboxed; alat dalam proses seperti `WebFetch` masih mengikuti [aturan izin](#permission-rules) mereka. Mengaturnya di `.claude/settings.json` atau `.claude/settings.local.json` repositori tidak berpengaruh. Memerlukan Claude Code v2.1.219 atau lebih baru.
* **Lockdown terkelola**: jika [`allowManagedDomainsOnly`](/docs/id/settings-reference#sandbox-network-allowmanageddomainsonly) diatur dalam pengaturan terkelola, domain yang tidak diizinkan diblokir secara otomatis alih-alih meminta, dan hanya `allowedDomains` dan aturan izin `WebFetch(domain:...)` dari pengaturan terkelola yang dihormati.
* **Proxy korporat**: ketika jaringan Anda memerlukan lalu lintas keluar untuk melewati proxy korporat, atur `HTTPS_PROXY`, `HTTP_PROXY`, dan `NO_PROXY` seperti yang dijelaskan [konfigurasi proxy](/docs/id/network-config#proxy-configuration), dalam blok `env` pengaturan Anda sehingga [agen latar belakang](/docs/id/network-config#set-network-variables-in-settings-not-the-shell) mendapatkannya juga, atau di lingkungan tempat Anda meluncurkan Claude Code. Claude Code memberlakukan allowlist domain dan kemudian menerowongi koneksi yang diizinkan melalui proxy hulu tersebut.
* **Dukungan proxy khusus**: pengguna tingkat lanjut dapat menerapkan aturan khusus pada lalu lintas keluar
* **Cakupan komprehensif**: pembatasan berlaku untuk semua skrip, program, dan subprocess yang dihasilkan oleh perintah

Dalam aturan `WebFetch(domain:...)`, sandbox menghormati dua bentuk wildcard: `*.` terdepan, seperti `*.example.com`, dan `*` bare. Bentuk `*` bare memerlukan Claude Code v2.1.186 atau lebih baru. Wildcard di posisi lain, seperti `WebFetch(domain:example.*)`, masih cocok dengan fetch tetapi tidak berpengaruh pada perintah sandboxed.

<Note>
  Proxy bawaan memberlakukan allowlist berdasarkan hostname yang diminta dan, secara default, tidak menghentikan atau memeriksa lalu lintas TLS. Pengaturan eksperimental [`network.tlsTerminate`](/docs/id/settings-reference#sandbox-network-tlsterminate), tersedia di Claude Code v2.1.199 dan lebih baru, membuat proxy bawaan menghentikan TLS itu sendiri, yang [entri kredensial `mask`](#mask-credentials) memerlukan. Lihat [Batasan keamanan](#security-limitations) untuk implikasi default, dan [Konfigurasi proxy khusus](#custom-proxy-configuration) jika model ancaman Anda memerlukan inspeksi TLS.
</Note>

<h4 id="per-command-allowed-domains-in-auto-mode">
  Domain yang diizinkan per perintah dalam mode otomatis
</h4>

Dalam [mode otomatis](/docs/id/permission-modes#eliminate-prompts-with-auto-mode) dengan sandboxing aktif, Claude menamai host yang dibutuhkan perintah pada perintah itu sendiri alih-alih memicu persetujuan jaringan untuk setiap koneksi. Setiap perintah Bash, PowerShell, atau [Monitor](/docs/id/tools-reference#monitor-tool) yang berjalan dalam sandbox dapat membawa daftar host di luar allowlist sandbox: domain seperti `registry.npmjs.org`, wildcard seperti `*.pythonhosted.org`, atau alamat IP, masing-masing dengan `:port` opsional. Pengklasifikasi meninjau host bersama dengan perintah. Memerlukan Claude Code v2.1.271 atau lebih baru.

Daftar yang disetujui membuka host tersebut hanya untuk perintah itu saja, selama berjalan. Tidak ada yang ditambahkan ke host yang diizinkan sesi Anda atau pengaturan Anda; perintah berikutnya menamai host-nya sendiri.

Perintah yang membawa host pergi ke pengklasifikasi alih-alih disetujui oleh aturan izin atau [mode auto-allow](#sandbox-modes) sandbox. Jika [aturan ask](/docs/id/permissions#manage-permissions) memaksa prompt untuk perintah, dialog izin di terminal Anda mencantumkan host di sebelahnya, dan menyetujui di sana mencakup keduanya.

Daftar per perintah memperluas hanya apa yang ditolak sandbox secara default. Entri [`deniedDomains`](/docs/id/settings-reference#sandbox-network-denieddomains) masih memblokir. Ketika [`strictAllowlist`](/docs/id/settings-reference#sandbox-network-strictallowlist) atau [`allowManagedDomainsOnly`](/docs/id/settings-reference#sandbox-network-allowmanageddomainsonly) mengunci allowlist, Claude Code menolak daftar per perintah.

Sementara daftar per perintah berlaku, Claude Code menolak koneksi ke host yang tidak ada perintah yang disetujui daftarkan, tanpa prompt atau pemeriksaan pengklasifikasi. Penolakan menamai host dalam hasil perintah, dan Claude menjalankan kembali perintah dengan host ditambahkan.

<h4 id="ipv6-addresses-in-domain-lists">
  Alamat IPv6 dalam daftar domain
</h4>

Daftar domain sandbox adalah `allowedDomains`, `deniedDomains`, dan aturan `WebFetch(domain:...)` yang memberinya makan. Untuk mencocokkan alamat IPv6 di salah satunya, tulis literal dalam tanda kurung: `"[::1]"` cocok dengan alamat itu di setiap port, dan `"[::1]:443"` cocok dengannya hanya di port 443. Tulis port sebagai angka dari 1 hingga 65535 tanpa nol terdepan. Bentuk dalam tanda kurung memerlukan Claude Code v2.1.229 atau lebih baru. Sebelum v2.1.229, ketika teks setelah titik dua terakhir entri adalah nomor port, Claude Code membacanya sebagai satu, jadi `::1:443` menamai alamat `::1` di port 443.

Ketika Anda memilih "Ya, dan jangan tanya lagi" di prompt persetujuan jaringan untuk alamat IPv6, Claude Code menyimpan aturan `WebFetch(domain:...)` dengan alamat dalam tanda kurung, sehingga aturan terus mencocokkan alamat di sesi mendatang.

Entri tanpa tanda kurung dengan dua atau lebih titik dua ambigu: `::1:443` adalah alamat IPv6 lengkap dan alamat diikuti oleh port. Claude Code memberlakukan ejaan ambigu secara konservatif alih-alih menebak pembacaan mana yang Anda maksudkan:

* **Daftar penolakan**: Claude Code menolak setiap pembacaan yang diuraikan entri, jadi pembacaan mana pun yang Anda maksudkan diblokir. Untuk entri tanpa pembacaan yang dapat diuraikan, Claude Code tidak memblokir apa pun.
* **Daftar izin**: Claude Code tidak pernah memungkinkan lebih dari yang Anda tulis. Ini menulis ulang entri ambigu ke pembacaan host-dan-port ketika pembacaan itu diuraikan dengan bersih, dan mungkin menjatuhkan entri sepenuhnya daripada memperluas allowlist.

Jalankan `claude doctor` di terminal Anda untuk menemukan entri yang terpengaruh: peringatan `Sandbox network domain entries have unreliable spellings` menamai hingga tiga dari mereka dan menghitung sisanya. Tulis ulang masing-masing dalam bentuk dalam tanda kurung untuk menghapus peringatan. Peringatan juga menamai entri yang ejaannya tidak dapat diandalkan karena alasan lain, seperti `@`, karakter jalur atau kueri, atau wildcard di dalam tanda kurung.

<h3 id="os-level-enforcement">
  Penegakan tingkat OS
</h3>

Alat Bash sandboxed menggunakan primitif keamanan sistem operasi:

* **macOS**: menggunakan Seatbelt untuk penegakan sandbox
* **Linux**: menggunakan [bubblewrap](https://github.com/containers/bubblewrap) untuk isolasi
* **WSL2**: menggunakan bubblewrap, sama seperti Linux

WSL1 tidak didukung karena bubblewrap memerlukan fitur kernel yang hanya tersedia di WSL2.

Primitif yang sama tersedia sebagai paket [`@anthropic-ai/sandbox-runtime`](https://github.com/anthropic-experimental/sandbox-runtime) mandiri, yang halaman [Sandbox environments](/docs/id/sandbox-environments#sandbox-runtime) mencakup sebagai pendekatan terpisah untuk membungkus seluruh proses Claude Code.

<h2 id="how-sandboxing-relates-to-permissions-and-permission-modes">
  Bagaimana sandboxing berhubungan dengan izin dan mode izin
</h2>

Sandboxing, [permission rules](/docs/id/permissions), dan [permission modes](/docs/id/permission-modes) adalah lapisan komplementer. Bagian di bawah mencakup bagaimana sandbox berinteraksi dengan masing-masing.

<h3 id="permission-rules">
  Aturan izin
</h3>

Aturan izin dan sandboxing mengontrol hal yang berbeda:

* **Aturan izin** mengontrol alat mana yang dapat digunakan Claude Code dan dievaluasi sebelum alat apa pun berjalan. Mereka berlaku untuk semua alat: Bash, Read, Edit, WebFetch, MCP, dan lainnya, kecuali bahwa aturan deny atau ask tidak dapat memblokir [`EndConversation`](/docs/id/tools-reference#endconversation-tool-behavior) sementara alat lain tetap ada.
* **Sandboxing** menyediakan penegakan tingkat OS yang membatasi apa yang dapat diakses perintah Bash pada tingkat filesystem dan jaringan. Ini hanya berlaku untuk perintah Bash, PowerShell, dan [Monitor](/docs/id/tools-reference#monitor-tool) serta proses anak mereka.

Kedua lapisan juga berbeda dalam cara penegakan mereka. Claude Code mengevaluasi keputusan izin sebelum perintah berjalan, berdasarkan string perintah dan, dalam mode auto, penilaian classifier terpisah tentang apakah perintah aman. Sistem operasi memberlakukan batas sandbox pada proses yang berjalan, sehingga berlaku terlepas dari apa yang dipilih model untuk dijalankan dan bahkan jika perintah yang diizinkan melakukan lebih dari nama yang disarankan.

Pembatasan filesystem dan jaringan dikonfigurasi melalui pengaturan sandbox dan aturan izin:

| Pengaturan atau aturan                                           | Apa yang dilakukannya                                                                                            |
| :--------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------- |
| `sandbox.filesystem.allowWrite`                                  | Memberikan akses tulis subprocess ke jalur di luar direktori kerja                                               |
| `sandbox.filesystem.denyWrite` dan `sandbox.filesystem.denyRead` | Memblokir akses subprocess ke jalur tertentu                                                                     |
| `sandbox.filesystem.allowRead`                                   | Mengizinkan kembali pembacaan jalur tertentu dalam wilayah `denyRead`                                            |
| [`sandbox.filesystem.disabled`](#disable-filesystem-isolation)   | Mematikan lapisan filesystem sepenuhnya sambil mempertahankan isolasi jaringan                                   |
| Aturan izin `Edit`                                               | Memberikan akses tulis ke jalur tertentu, dengan cara yang sama seperti `sandbox.filesystem.allowWrite`          |
| Aturan tolak `Read` dan `Edit`                                   | Memblokir akses ke file atau direktori tertentu                                                                  |
| Aturan izin dan tolak `WebFetch(domain:...)`                     | Mengontrol akses domain                                                                                          |
| Sandbox `allowedDomains`                                         | Mengontrol domain mana yang dapat dijangkau perintah Bash                                                        |
| Sandbox `deniedDomains`                                          | Memblokir domain tertentu bahkan ketika wildcard `allowedDomains` yang lebih luas akan sebaliknya mengizinkannya |

Jalur dan domain dari pengaturan sandbox dan aturan izin digabungkan bersama ke dalam konfigurasi sandbox akhir.

[Direktori contoh repositori claude-code](https://github.com/anthropics/claude-code/tree/main/examples/settings) mencakup konfigurasi pengaturan pemula untuk skenario penyebaran umum, termasuk contoh khusus sandbox. Gunakan ini sebagai titik awal dan sesuaikan dengan kebutuhan Anda.

<h3 id="permission-modes">
  Mode izin
</h3>

`/sandbox` bukan [permission mode](/docs/id/permission-modes). Mode izin memutuskan apakah panggilan alat berjalan dan apakah Anda diminta terlebih dahulu, sementara sandbox membatasi apa yang dapat diakses perintah Bash setelah berjalan. Mereka berbeda dalam apa yang mereka kontrol dan apa yang menggantikan prompt per-aksi:

|                                                                    | Apa yang dikontrol                                    | Apa yang menggantikan prompt                                                                                                                                                                               |
| :----------------------------------------------------------------- | :---------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `/sandbox`                                                         | Apa yang dapat diakses perintah Bash setelah berjalan | Batas sandbox itu sendiri, dalam [mode auto-allow](#sandbox-modes)                                                                                                                                         |
| [Auto mode](/docs/id/permission-modes#eliminate-prompts-with-auto-mode) | Apakah setiap panggilan alat berjalan                 | Classifier yang meninjau tindakan                                                                                                                                                                          |
| `--dangerously-skip-permissions`                                   | Apakah setiap panggilan alat berjalan                 | Tidak ada. Pemeriksaan [Protected path](/docs/id/permission-modes#protected-paths) juga dilewati; [tindakan yang tidak ada mode auto-approve](/docs/id/permission-modes#actions-no-mode-auto-approves) masih berlaku |

Mode [auto-allow](#sandbox-modes) sandbox terpisah dari [auto mode](/docs/id/permission-modes#eliminate-prompts-with-auto-mode): auto-allow menyetujui perintah Bash karena batas sandbox memuatnya, sementara auto mode menggunakan classifier untuk meninjau tindakan. Keduanya bekerja secara independen dan dapat dikombinasikan, dengan pengecualian yang tercantum di bawah [Sandbox modes](#sandbox-modes). Untuk memilih batas isolasi untuk run tanpa pengawasan, lihat [Sandbox environments](/docs/id/sandbox-environments#how-isolation-relates-to-permission-modes). Untuk tabel pasangan mode izin dan sandbox umum dengan flag yang memulai masing-masing, lihat [Common setups](/docs/id/permission-modes#common-setups).

<h2 id="configure-the-sandbox-for-your-organization">
  Konfigurasi sandbox untuk organisasi Anda
</h2>

Administrator dapat memerlukan sandboxing untuk setiap pengguna, mencegah pengembang memperluas kebijakan, dan merutekan lalu lintas sandbox melalui proxy perusahaan.

<h3 id="enforce-sandboxing-with-managed-settings">
  Memberlakukan sandboxing dengan pengaturan terkelola
</h3>

Untuk memerlukan sandbox untuk setiap pengembang, berikan kunci `sandbox` melalui [managed settings](/docs/id/managed-settings#delivery-mechanisms), baik sebagai file yang dikelola oleh MDM Anda atau melalui [server-managed settings](/docs/id/server-managed-settings) di claude.ai.

Konfigurasi pengaturan terkelola berikut mengaktifkan sandbox, menolak untuk memulai Claude Code jika sandbox tidak dapat diinisialisasi, dan mencegah model dari mencoba kembali perintah di luar sandbox:

```json theme={null}
{
  "sandbox": {
    "enabled": true,
    "failIfUnavailable": true,
    "allowUnsandboxedCommands": false
  }
}
```

Dua kunci di luar `enabled` mengontrol apa yang terjadi ketika sandbox tidak dapat menjalankan perintah:

* **`failIfUnavailable`**: dependensi yang hilang seperti bubblewrap di Linux memblokir Claude Code dari memulai daripada menampilkan peringatan dan kembali ke eksekusi unsandboxed
* **`allowUnsandboxedCommands: false`**: Claude Code mengabaikan escape hatch `dangerouslyDisableSandbox`, sehingga ketika perintah gagal di bawah sandbox, Claude tidak dapat mencoba ulangnya tanpa sandbox

Dua penambahan layak dipertimbangkan bersama mereka. Tambahkan `excludedCommands` untuk alat yang disetujui organisasi apa pun yang harus berjalan tanpa isolasi. Tambahkan entri [`sandbox.credentials`](#protect-credentials) untuk direktori kredensial seperti `~/.aws` dan `~/.ssh` dan untuk variabel lingkungan rahasia, karena kebijakan pembacaan default masih memungkinkan mereka.

Konfigurasi ini melakukan sandboxing pada perintah yang dijalankan Claude. Pengembang masih dapat mengetik perintah di [prompt shell-mode `!`](/docs/id/interactive-mode#shell-mode-with-prefix) dan menjalankannya di luar sandbox, dengan akses yang sama yang mereka miliki di terminal apa pun di luar Claude Code. Lihat [The unsandboxed retry escape hatch](#the-unsandboxed-retry-escape-hatch) untuk sesi di mana perintah yang diketik dijalankan dengan sandbox.

Sandbox tidak berjalan di Windows asli, jadi jika armada Anda mencakup host Windows, batasi konfigurasi ini ke macOS dan Linux atau minta pengguna tersebut menjalankan Claude Code di dalam WSL2 atau container.

<h3 id="keep-developers-from-widening-the-policy">
  Cegah pengembang memperluas kebijakan
</h3>

Untuk kunci boolean seperti `enabled` dan `failIfUnavailable`, Claude Code menggunakan nilai terkelola dan mengabaikan apa pun yang ditetapkan pengembang secara lokal. Untuk kunci array seperti `excludedCommands` dan `allowRead`, Claude Code menggabungkan entri dari setiap scope yang dimuat sesi, sehingga pengembang dapat menambahkan entri yang memperluas kebijakan.

Atur `allowManagedReadPathsOnly` ke `true` dalam pengaturan terkelola sehingga hanya entri `allowRead` dari pengaturan terkelola yang dihormati. Ini mencegah pengembang memperluas akses baca di luar jalur yang disetujui organisasi. Untuk mengunci domain jaringan ke nilai terkelola dengan cara yang sama, atur [`allowManagedDomainsOnly`](/docs/id/settings-reference#sandbox-network-allowmanageddomainsonly).

Ketika pengaturan terkelola mengonfigurasi `sandbox.filesystem` atau mencantumkan entri `sandbox.credentials.files` apa pun dengan `"mode": "deny"`, hanya pengaturan terkelola yang dapat mengatur [`filesystem.disabled`](#disable-filesystem-isolation), sehingga pengembang tidak dapat mematikan pembatasan filesystem yang diterapkan administrator. Apakah entri `mask` mengikat kunci tergantung pada cara penyelesaiannya; tabel di bawah [Which settings can disable it](#which-settings-can-disable-it) mencakup empat kasus.

`excludedCommands` tidak memiliki lockdown hanya terkelola yang setara, sehingga pengembang selalu dapat menambahkan entri yang menjalankan perintah tambahan di luar sandbox. Jaga daftar terkelola tetap sempit.

<h3 id="custom-proxy-configuration">
  Konfigurasi proxy khusus
</h3>

Untuk organisasi yang memerlukan keamanan jaringan lanjutan, Anda dapat menerapkan proxy khusus untuk:

* Mendekripsi dan memeriksa lalu lintas HTTPS
* Menerapkan aturan penyaringan khusus
* Mencatat semua permintaan jaringan
* Mengintegrasikan dengan infrastruktur keamanan yang ada

Untuk menunjukkan Claude Code ke proxy Anda, atur port proxy dalam [sandbox settings](/docs/id/settings-reference#sandbox-settings):

```json theme={null}
{
  "sandbox": {
    "network": {
      "httpProxyPort": 8080,
      "socksProxyPort": 8081
    }
  }
}
```

<h2 id="troubleshooting">
  Pemecahan masalah
</h2>

Beberapa perintah gagal di dalam sandbox meskipun bekerja di luar itu. Perbaikan di bawah mencakup kasus paling umum.

* **Perintah gagal dengan kesalahan host-not-allowed**: banyak alat CLI perlu menjangkau host tertentu. Memberikan izin saat diminta menambahkan host ke daftar yang diizinkan sehingga alat berjalan di dalam sandbox di masa depan.
* **`jest` hang atau gagal**: `watchman` tidak kompatibel dengan sandbox. Jalankan `jest --no-watchman` sebagai gantinya.
* **Go-based CLIs gagal verifikasi TLS di macOS**: alat seperti `gh`, `gcloud`, dan `terraform` mungkin gagal verifikasi TLS di bawah Seatbelt. Daftar alat ini dalam [`excludedCommands`](/docs/id/settings-reference#sandbox-excludedcommands). Jika Anda menggunakan `httpProxyPort` dengan proxy MITM dan CA khusus, atur [`enableWeakerNetworkIsolation`](/docs/id/settings-reference#sandbox-enableweakernetworkisolation) ke `true` sebagai gantinya.
* **Perintah `open`, `osascript`, atau alur autentikasi berbasis browser gagal dengan kesalahan `-600` di macOS**: sandbox memblokir Apple Events secara default. Atur [`allowAppleEvents`](/docs/id/settings-reference#sandbox-allowappleevents) ke `true` dalam pengaturan pengguna, terkelola, atau CLI Anda untuk mengizinkannya. Pengaturan proyek diabaikan untuk kunci ini. Mengaktifkannya menghilangkan isolasi eksekusi kode, karena perintah sandboxed kemudian dapat meluncurkan aplikasi lain tanpa sandbox tanpa prompt pengguna dan mengirim perintah AppleScript ke aplikasi yang berjalan, tunduk pada prompt otomasi-persetujuan macOS (TCC). Alternatifnya, tambahkan perintah ke [`excludedCommands`](/docs/id/settings-reference#sandbox-excludedcommands).
* **Perintah `docker` gagal**: `docker` tidak kompatibel dengan sandbox. Tambahkan `docker *` ke [`excludedCommands`](/docs/id/settings-reference#sandbox-excludedcommands).
* **`pbcopy`, `xclip`, atau `wl-copy` tidak memperbarui clipboard**: utilitas clipboard ini dapat gagal menjangkau clipboard sistem dari dalam sandbox, dalam hal ini teks yang dialirkan ke dalamnya tidak tiba.

  Untuk menempatkan output Claude di clipboard Anda, minta Claude untuk mencetaknya dalam responsnya, kemudian jalankan [`/copy`](/docs/id/commands). `/copy` menulis ke clipboard dari proses Claude Code daripada dari perintah sandboxed.

  Ketika Claude mengalirkan teks ke salah satu alat ini, menambahkan alat ke [`excludedCommands`](/docs/id/settings-reference#sandbox-excludedcommands) tidak mengeluarkan panggilan itu dari sandbox dengan sendirinya.
* **Perintah git gagal dengan `unable to unlink old`**: `git merge`, `git checkout`, dan perintah serupa gagal dengan cara ini ketika mereka perlu mengganti file yang sandbox tolak penulisannya, apakah file itu berada di bawah [jalur yang dilindungi](#protected-paths) seperti `.claude/skills`, di bawah salah satu entri `denyWrite` Anda, atau di luar direktori yang sandbox izinkan perintah untuk menulis sama sekali. Di Linux dan WSL2 kesalahan berakhir dengan `Read-only file system`.

  Setelah kegagalan, Claude mungkin [menawarkan untuk menjalankan kembali perintah di luar sandbox](#the-unsandboxed-retry-escape-hatch); setujui retry itu, atau jalankan perintah git sendiri di terminal lain. Jika Anda telah menetapkan `allowUnsandboxedCommands` ke `false`, Claude tidak dapat menawarkan retry, jadi jalankan perintah sendiri. Jika perintah git yang sama sering gagal, tambahkan ke [`excludedCommands`](/docs/id/settings-reference#sandbox-excludedcommands).
* **Bubblewrap gagal memulai di dalam container**: dalam container tanpa privilege, bubblewrap tidak dapat memasang filesystem `/proc` segar, sehingga perintah sandboxed gagal dengan kesalahan `bwrap` seperti `Can't mount proc on /newroot/proc: Operation not permitted`. Atur [`enableWeakerNestedSandbox`](/docs/id/settings-reference#sandbox-enableweakernestedsandbox) ke `true` sehingga sandbox dalam bind-mount `/proc` yang ada dari container sebagai gantinya. Hanya gunakan pengaturan ini ketika container luar sudah menyediakan batas isolasi yang Anda butuhkan, karena mengekspos informasi proses ke perintah sandboxed yang mount `/proc` segar akan menyembunyikan.
* **File read-only 0-byte muncul di jalur pengaturan `.claude`, dan "Ya, dan jangan tanya lagi" tidak menyimpan**: di Linux dan WSL2, sandbox menahan penolakan penulisan pada file yang belum ada dengan membuat placeholder read-only 0-byte di sana sementara perintah sandboxed berjalan. Sandbox menghapus placeholder setelahnya. Jika sesi dibunuh sebelum pembersihan itu berjalan, misalnya oleh SIGKILL, placeholder tetap tertinggal. Sesi kemudian mengikatnya read-only lagi pada setiap awal, jadi penulisan pengaturan seperti menyimpan pilihan izin gagal di mana satu duduk.

  Jalankan `claude doctor` untuk membuat daftar file placeholder yang tersisa. Peringatan [`Stale sandbox mask files left by a killed session`](/docs/id/errors#stale-sandbox-mask-files-left-by-a-killed-session) menamai hingga tiga di antaranya dan menghitung sisanya. Hapus setiap file dengan `rm` sementara tidak ada sesi Claude Code lain yang berjalan di proyek itu. Sebelum v2.1.257, Claude Code meninggalkan placeholder yang sama tanpa menandainya.
* **`--dangerously-skip-permissions` gagal sebagai root**: flag ini diblokir saat menjalankan sebagai root atau melalui sudo di Linux dan macOS, karena akses root dikombinasikan dengan tidak ada prompt izin dapat memodifikasi file atau layanan apa pun di sistem. Pemeriksaan dilewati secara otomatis di dalam sandbox yang dikenali. Untuk menjalankan secara otonom dalam container, gunakan konfigurasi [dev container](/docs/id/devcontainer), yang menjalankan Claude Code sebagai pengguna non-root.

<h2 id="limitations">
  Keterbatasan
</h2>

Sandboxing mengurangi risiko tetapi bukan batas isolasi lengkap. Tinjau keterbatasan di bawah sebelum mengandalkannya sebagai kontrol keamanan keras.

<h3 id="security-limitations">
  Keterbatasan keamanan
</h3>

* **Penyaringan jaringan**: sandbox membatasi domain mana yang dapat terhubung oleh proses. Secara default, proxy bawaan tidak menghentikan atau melakukan inspeksi TLS pada lalu lintas keluar, sehingga isi koneksi terenkripsi tidak diperiksa. Pengaturan eksperimental [`network.tlsTerminate`](/docs/id/settings-reference#sandbox-network-tlsterminate) menghentikan TLS di proxy untuk [substitusi kredensial `mask`](#mask-credentials) tetapi tidak menambahkan penyaringan konten. Anda bertanggung jawab untuk memastikan bahwa hanya domain tepercaya yang diizinkan dalam kebijakan Anda.

<Warning>
  Mengizinkan domain luas seperti `github.com` dapat membuat jalur untuk eksfiltrasi data. Karena proxy membuat keputusan izin dari hostname yang disediakan klien tanpa memeriksa TLS, kode yang berjalan di dalam sandbox berpotensi dapat menggunakan [domain fronting](https://en.wikipedia.org/wiki/Domain_fronting) atau teknik serupa untuk menjangkau host di luar allowlist. Jika model ancaman Anda memerlukan jaminan yang lebih kuat, konfigurasikan [custom proxy](#custom-proxy-configuration) yang menghentikan TLS dan memeriksa lalu lintas, dan instal sertifikat CA-nya di dalam sandbox. Isolasi jaringan yang lebih kuat dan sadar TLS adalah area pengembangan aktif.
</Warning>

* **Eskalasi privilege melalui soket Unix**: konfigurasi `allowUnixSockets` dapat secara tidak sengaja memberikan akses ke layanan sistem yang dapat menyebabkan bypass sandbox. Misalnya, mengizinkan akses ke `/var/run/docker.sock` secara efektif memberikan akses ke sistem host melalui soket Docker. Pertimbangkan dengan hati-hati soket Unix apa pun yang Anda izinkan melalui sandbox.
* **Eskalasi izin filesystem**: izin penulisan filesystem yang terlalu luas dapat memungkinkan serangan eskalasi privilege. Mengizinkan penulisan ke direktori yang berisi executable dalam `$PATH`, direktori konfigurasi sistem, atau file konfigurasi shell pengguna seperti `.bashrc` atau `.zshrc` dapat menyebabkan eksekusi kode dalam konteks keamanan yang berbeda ketika pengguna lain atau proses sistem mengakses file ini.
* **Kekuatan sandbox Linux**: implementasi Linux menyediakan isolasi filesystem dan jaringan yang kuat tetapi mencakup mode `enableWeakerNestedSandbox` yang memungkinkannya bekerja di dalam lingkungan Docker tanpa namespace istimewa, atau pada host Linux di mana user namespaces tanpa privilege dinonaktifkan oleh sysctl. Opsi ini secara konsiderabel melemahkan keamanan dan hanya boleh digunakan ketika isolasi tambahan sebaliknya diberlakukan.
* **Apple Events pada macOS**: sandbox macOS memblokir Apple Events secara default. Pengaturan `allowAppleEvents` menghapus pembatasan ini sehingga alat seperti `open` dan `osascript` berfungsi, tetapi menghilangkan isolasi eksekusi kode: perintah sandboxed dapat meluncurkan aplikasi lain tanpa sandbox tanpa prompt pengguna, dan dapat mengirim perintah AppleScript ke aplikasi yang sedang berjalan, tunduk pada prompt persetujuan otomasi macOS per-aplikasi (TCC). Ini hanya dihormati dari pengaturan pengguna, terkelola, atau CLI. Pengaturan proyek tidak dapat mengaktifkannya.

<h3 id="platform-and-tool-compatibility">
  Kompatibilitas platform dan alat
</h3>

* **Dukungan platform**: mendukung macOS, Linux, dan WSL2. WSL1 dan Windows asli tidak didukung.
* **Overhead kinerja**: minimal, tetapi beberapa operasi filesystem mungkin sedikit lebih lambat.
* **Kompatibilitas alat**: beberapa alat yang memerlukan pola akses sistem tertentu mungkin memerlukan penyesuaian konfigurasi, atau mungkin perlu dijalankan di luar sandbox.

<h3 id="scope">
  Cakupan
</h3>

Sandbox mengisolasi subprocess Bash. Alat lain beroperasi di bawah batas yang berbeda:

* **Alat file bawaan**: Read, Edit, dan Write menggunakan sistem izin secara langsung daripada berjalan melalui sandbox. Lihat [permissions](/docs/id/permissions).
* **Penggunaan komputer**: ketika Claude membuka aplikasi dan mengontrol layar Anda, itu berjalan di desktop aktual Anda daripada di lingkungan terisolasi. Prompt izin per-aplikasi membatasi setiap aplikasi. Lihat [computer use in the CLI](/docs/id/computer-use) atau [computer use in Desktop](/docs/id/desktop#let-claude-use-your-computer).
* **Variabel lingkungan**: perintah Bash sandboxed mewarisi lingkungan proses induk secara default, termasuk kredensial apa pun yang ditetapkan di sana. Gunakan [`sandbox.credentials`](#protect-credentials) untuk menghapus atau menutupi variabel tertentu untuk perintah sandboxed, atau atur [`CLAUDE_CODE_SUBPROCESS_ENV_SCRUB`](/docs/id/env-vars) untuk menghapus kredensial dari semua subprocess.
* **Subagents**: [subagents](/docs/id/sub-agents) berjalan dalam proses yang sama dengan sesi induk dan menggunakan konfigurasi sandbox yang sama. Perintah Bash di dalam subagent di-sandbox ketika sandboxing diaktifkan dalam sesi induk.

<Warning>
  Sandboxing yang efektif memerlukan isolasi filesystem dan jaringan. Tanpa isolasi jaringan, agen yang dikompromikan dapat mengeksfiltrasikan file sensitif seperti kunci SSH. Tanpa isolasi filesystem, baik dari kebijakan yang permisif atau dari [menonaktifkan lapisan filesystem](#disable-filesystem-isolation), agen yang dikompromikan dapat memasang pintu belakang pada sumber daya sistem untuk mendapatkan akses jaringan. Ketika Anda memperluas default, periksa bahwa jalur `allowWrite`, entri `allowedDomains` yang luas, atau pengecualian `excludedCommands` tidak membatalkan pembatasan di sisi lain.
</Warning>

<h2 id="see-also">
  Lihat juga
</h2>

* [Sandbox environments](/docs/id/sandbox-environments): bandingkan sandbox bawaan dengan dev containers, containers, dan VM
* [Security](/docs/id/security): fitur keamanan komprehensif dan praktik terbaik
* [Permissions](/docs/id/permissions): konfigurasi izin dan kontrol akses
* [All settings](/docs/id/settings-reference): setiap kunci pengaturan
* [CLI reference](/docs/id/cli-reference): opsi baris perintah
