> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Konfigurasi alat Bash sandboxed

> Pelajari bagaimana alat Bash sandboxed Claude Code menyediakan isolasi filesystem dan jaringan untuk eksekusi agen yang lebih aman dan mandiri.

Sandbox Bash memungkinkan Claude menjalankan sebagian besar perintah shell tanpa berhenti untuk meminta izin. Alih-alih menyetujui setiap perintah, Anda menentukan file dan domain jaringan mana yang dapat diakses perintah, dan sistem operasi memberlakukan batas itu untuk setiap perintah Bash dan proses anak-anaknya.

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

    Ini membuka panel sandbox dengan tiga tab:

    * **Mode**: pilih bagaimana perintah sandboxed disetujui, dibahas dalam langkah berikutnya
    * **Overrides**: pilih apakah perintah yang gagal di bawah sandbox dapat kembali ke menjalankan unsandboxed. Ini adalah pengaturan [`allowUnsandboxedCommands`](/docs/id/settings#sandbox-settings)
    * **Config**: lihat pengaturan sandbox yang diselesaikan

    Jika panel hanya menampilkan tab Dependencies, paket yang diperlukan hilang. Instal seperti yang dijelaskan dalam [Set up Linux and WSL2](#set-up-linux-and-wsl2), restart Claude Code, dan jalankan `/sandbox` lagi.
  </Step>

  <Step title="Pilih mode">
    Di tab Mode, pilih auto-allow atau regular permissions. Auto-allow menjalankan perintah sandboxed tanpa prompt, dan regular permissions menjaga prompt izin reguler bahkan ketika perintah sandboxed. Lihat [Sandbox modes](#sandbox-modes) untuk perintah mana yang masih prompt dalam mode auto-allow.
  </Step>

  <Step title="Jalankan perintah Bash">
    Minta Claude untuk menjalankan perintah, seperti build atau test suite. Secara default, perintah di dalam sandbox hanya dapat menulis ke direktori kerja dan direktori temp sesi. Pertama kali perintah memerlukan domain jaringan baru, Claude Code meminta persetujuan.

    Perintah yang tidak dapat berjalan sandboxed kembali ke alur izin reguler. Untuk memperluas atau mempersempit batas ini, lihat [Configure sandboxing](#configure-sandboxing).
  </Step>
</Steps>

Memilih mode dalam panel menulis ke pengaturan lokal proyek Anda di `.claude/settings.local.json`, yang berlaku untuk proyek saat ini dan tidak diperiksa ke git. Untuk mengaktifkan sandbox di semua proyek Anda, atur [`sandbox.enabled`](/docs/id/settings#sandbox-settings) ke `true` dalam pengaturan pengguna Anda di `~/.claude/settings.json`. Untuk memberlakukan sandboxing untuk setiap pengembang dalam organisasi, gunakan [managed settings](#enforce-sandboxing-with-managed-settings).

<Warning>
  Secara default, jika sandbox tidak dapat dimulai karena dependensi hilang atau platform tidak didukung, Claude Code menampilkan peringatan dan menjalankan perintah tanpa sandboxing. Untuk menjadikan ini kegagalan keras sebagai gantinya, atur [`sandbox.failIfUnavailable`](/docs/id/settings#sandbox-settings) ke `true`. Ini dimaksudkan untuk penyebaran terkelola yang memerlukan sandboxing sebagai gerbang keamanan.
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

Setelah menginstal, tab Dependencies dalam `/sandbox` menunjukkan apakah `ripgrep`, `bubblewrap`, `socat`, dan filter seccomp tersedia di platform Anda. Ripgrep disertakan dengan binari Claude Code asli. Filter seccomp bersifat opsional dan menambahkan pemblokiran soket domain Unix. Instal dengan `npm install -g @anthropic-ai/sandbox-runtime` jika hilang.

Ketika dependensi yang diperlukan hilang, tab Dependencies adalah satu-satunya tab yang ditampilkan sampai Anda menginstalnya. Pemeriksaan dependensi berjalan saat startup, jadi restart Claude Code setelah menginstal paket untuk `/sandbox` mendeteksinya.

<AccordionGroup>
  <Accordion title="Ubuntu 24.04 dan yang lebih baru: izinkan bubblewrap untuk membuat user namespaces">
    Di Ubuntu 24.04 dan yang lebih baru, kebijakan AppArmor default mencegah bubblewrap dari membuat user namespaces yang dibutuhkannya untuk isolasi.

    Untuk memeriksa apakah lingkungan Anda memberlakukan pembatasan ini, termasuk di dalam WSL2, jalankan `sysctl kernel.apparmor_restrict_unprivileged_userns`. Jika kunci tidak ada atau mengembalikan `0`, lewati langkah ini. Jika mengembalikan `1`, tambahkan profil AppArmor yang memberikan `bwrap` kemampuan ini:

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

    Di WSL2, perintah sandboxed tidak dapat meluncurkan binari Windows seperti `cmd.exe`, `powershell.exe`, atau apa pun di bawah `/mnt/c/`. WSL menyerahkan ini ke host Windows melalui soket Unix, yang sandbox blokir. Jika perintah perlu memanggil binari Windows, tambahkan ke [`excludedCommands`](/docs/id/settings#sandbox-settings) sehingga berjalan di luar sandbox.
  </Accordion>
</AccordionGroup>

<h3 id="sandbox-modes">
  Mode sandbox
</h3>

Claude Code menawarkan dua mode sandbox:

**Mode auto-allow**: Perintah Bash akan mencoba berjalan di dalam sandbox dan secara otomatis diizinkan tanpa memerlukan izin. Perintah yang tidak dapat di-sandbox, seperti yang memerlukan akses jaringan ke host yang tidak diizinkan, kembali ke alur izin reguler, di mana Claude Code memeriksa [permission rules](/docs/id/permissions) Anda dan meminta Anda untuk perintah apa pun yang tidak diizinkan oleh aturan tersebut.

Bahkan dalam mode auto-allow, hal berikut masih berlaku:

* [Deny rules](/docs/id/permissions) eksplisit selalu dihormati
* Perintah `rm` atau `rmdir` yang menargetkan `/`, direktori home Anda, atau jalur sistem kritis lainnya masih memicu prompt izin
* [Ask rules](/docs/id/permissions) berlaku untuk perintah yang kembali ke alur izin reguler

**Mode regular permissions**: Semua perintah Bash melalui alur izin reguler, bahkan ketika sandboxed. Ini memberikan lebih banyak kontrol tetapi memerlukan lebih banyak persetujuan.

Di kedua mode, sandbox memberlakukan pembatasan filesystem dan jaringan yang sama. Perbedaannya hanya dalam apakah perintah sandboxed disetujui secara otomatis atau memerlukan izin eksplisit.

Direktori temp sesi dapat ditulis di dalam sandbox secara default, bersama dengan direktori kerja. Claude Code menetapkan `$TMPDIR` ke direktori ini untuk perintah sandboxed, sehingga alat yang menulis file sementara bekerja tanpa konfigurasi tambahan. Perintah unsandboxed mewarisi `$TMPDIR` shell Anda tanpa perubahan, yang berarti perintah sandboxed dan unsandboxed menyelesaikan `$TMPDIR` ke direktori yang berbeda. Untuk meneruskan file sementara di antara keduanya, tulis di bawah direktori kerja sebagai gantinya.

Beberapa perintah tidak dapat berjalan di dalam sandbox sama sekali, seperti alat yang tidak kompatibel dengannya atau yang memerlukan host yang belum Anda izinkan. Daripada gagal tugas atau memerlukan Anda untuk mematikan sandboxing, Claude Code menyertakan pintu keluar: ketika perintah gagal karena pembatasan sandbox, Claude menganalisis kegagalan dan dapat mencoba kembali perintah dengan parameter `dangerouslyDisableSandbox`. Perintah yang dicoba kembali berjalan di luar sandbox, sehingga melalui alur izin reguler: dalam mode default Anda mendapatkan prompt konfirmasi; dalam [auto mode](/docs/id/permission-modes#eliminate-prompts-with-auto-mode) pengklasifikasi mengevaluasi perintah yang mendasar daripada meminta Anda. Untuk diminta pada setiap percobaan ulang unsandboxed bahkan dalam mode auto, tambahkan [ask rule](/docs/id/permissions#match-by-input-parameter) untuk `Bash(dangerouslyDisableSandbox:true)`.

Anda dapat menonaktifkan pintu keluar ini dengan mengatur `"allowUnsandboxedCommands": false` dalam [sandbox settings](/docs/id/settings#sandbox-settings) Anda. Ketika dinonaktifkan, yang ditampilkan tab Overrides `/sandbox` sebagai **Strict sandbox mode**, parameter `dangerouslyDisableSandbox` sepenuhnya diabaikan dan semua perintah harus berjalan sandboxed atau secara eksplisit terdaftar dalam `excludedCommands`.

<Info>
  Mode auto-allow bekerja secara independen dari pengaturan mode izin Anda. Bahkan jika Anda tidak dalam mode "accept edits", perintah Bash sandboxed akan berjalan secara otomatis ketika auto-allow diaktifkan. Ini berarti perintah Bash yang memodifikasi file dalam batas sandbox akan dieksekusi tanpa prompt, bahkan ketika alat edit file biasanya memerlukan persetujuan.
</Info>

<h2 id="configure-sandboxing">
  Konfigurasi sandboxing
</h2>

Sesuaikan perilaku sandbox melalui file `settings.json` Anda. Lihat [Settings](/docs/id/settings#sandbox-settings) untuk referensi konfigurasi lengkap.

Secara default, perintah sandboxed hanya dapat menulis ke direktori kerja saat ini dan direktori temp sesi. Jika perintah subprocess seperti `kubectl`, `terraform`, atau `npm` perlu menulis di luar direktori tersebut, gunakan `sandbox.filesystem.allowWrite` untuk memberikan akses ke jalur tertentu:

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

Ketika array filesystem yang sama didefinisikan dalam beberapa [settings scopes](/docs/id/settings#settings-precedence), array digabungkan: jalur dari setiap scope dikombinasikan, bukan diganti.

Awalan jalur mengontrol bagaimana jalur diselesaikan:

| Awalan                 | Arti                                                                                                | Contoh                                                                           |
| :--------------------- | :-------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------- |
| `/`                    | Jalur absolut dari akar filesystem                                                                  | `/tmp/build` tetap `/tmp/build`                                                  |
| `~/`                   | Relatif terhadap direktori home                                                                     | `~/.kube` menjadi `$HOME/.kube`                                                  |
| `./` atau tanpa awalan | Relatif terhadap akar proyek untuk pengaturan proyek, atau ke `~/.claude` untuk pengaturan pengguna | `./output` dalam `.claude/settings.json` diselesaikan ke `<project-root>/output` |

Sintaks ini berbeda dari [Read and Edit permission rules](/docs/id/permissions#read-and-edit), yang menggunakan `//path` untuk absolut dan `/path` untuk relatif proyek. Jalur filesystem sandbox menggunakan konvensi standar: `/tmp/build` adalah absolut.

Anda juga dapat menolak akses tulis atau baca menggunakan `sandbox.filesystem.denyWrite` dan `sandbox.filesystem.denyRead`, dan mengizinkan kembali jalur tertentu dalam wilayah yang ditolak menggunakan `sandbox.filesystem.allowRead`. Ketika aturan baca tumpang tindih, jalur yang lebih spesifik menang:

| Aturan contoh                                             | Hasil                                                                                                                                                                                              |
| :-------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `"denyRead": ["~/"]` dengan `"allowRead": ["~/projects"]` | `~/projects` dapat dibaca dan sisa direktori home tetap diblokir. Izin yang lebih sempit membuka kembali bagian itu dari wilayah yang ditolak                                                      |
| `"allowRead": ["~/"]` dengan `"denyRead": ["~/.env"]`     | `~/.env` tetap diblokir dan sisa direktori home dapat dibaca. Penolakan yang tepat berlaku di dalam izin yang lebih luas, jadi izin yang luas tidak dapat secara diam-diam membuka kembali rahasia |

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

`.` dalam `allowRead` diselesaikan ke akar proyek karena konfigurasi ini berada dalam pengaturan proyek. Jika Anda menempatkan konfigurasi yang sama dalam `~/.claude/settings.json`, `.` akan diselesaikan ke `~/.claude` sebagai gantinya, dan file proyek akan tetap diblokir oleh aturan `denyRead`.

<h3 id="protect-credentials">
  Lindungi kredensial
</h3>

Pengaturan `sandbox.credentials` mendeklarasikan file kredensial dan variabel lingkungan yang harus dilindungi dari perintah sandboxed. Setiap entri memberi nama jalur file atau variabel lingkungan dan `mode`. Blok `credentials` khusus menjaga aturan kredensial dikelompokkan bersama dan terpisah dari aturan filesystem umum. Memerlukan Claude Code v2.1.187 atau lebih baru.

Untuk entri dengan `"mode": "deny"`, jalur file ditolak untuk pembacaan di dalam sandbox, pembatasan yang sama yang diterapkan `filesystem.denyRead`, dan variabel lingkungan tidak diatur sebelum setiap perintah sandboxed berjalan.

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

Entri file hanya mendukung `"mode": "deny"`. Entri variabel lingkungan juga menerima `"mode": "mask"`, dijelaskan di bawah.

Jalur file mengikuti [aturan awalan](/docs/id/settings#sandbox-path-prefixes) yang sama dengan pengaturan `sandbox.filesystem.*`, dan entri `deny` dari setiap [settings scope](/docs/id/settings#settings-precedence) digabungkan. Entri `deny` hanya pernah mempersempit akses, jadi scope apa pun dapat menambahkan satu, tetapi tidak ada scope yang dapat menghapus satu yang ditambahkan scope lain.

Tidak ada daftar penolakan kredensial bawaan, jadi hanya file dan variabel yang Anda daftarkan yang dibatasi. Pengaturan ini mempengaruhi perintah Bash sandboxed saja. Untuk menghapus kredensial Anthropic dan penyedia cloud dari semua subprocess terlepas dari sandboxing, atur [`CLAUDE_CODE_SUBPROCESS_ENV_SCRUB`](/docs/id/env-vars).

<h4 id="mask-environment-variables">
  Topeng variabel lingkungan
</h4>

`"mode": "mask"` melindungi kredensial sambil membuat alat yang mengotentikasi dengannya tetap berfungsi. `deny` menghapus variabel sepenuhnya, yang juga merusak alat yang membutuhkannya, seperti `gh` atau `npm`. Memerlukan Claude Code v2.1.199 atau lebih baru.

Dengan `mask`, perintah sandboxed melihat nilai sentinel per-sesi sebagai gantinya dari yang asli. Ketika permintaan meninggalkan sandbox untuk salah satu `injectHosts` kredensial, [sandbox proxy](#network-isolation) mengganti sentinel dengan nilai asli. Perintah dan apa pun yang dicatat tidak pernah memegang kredensial asli, tetapi permintaannya tetap mengotentikasi.

Proxy mengganti kredensial di dalam konten permintaan, jadi harus melihatnya. Atur [`network.tlsTerminate`](/docs/id/settings#sandbox-settings) sehingga proxy menghentikan TLS itu sendiri. Tanpa itu, masking gagal tertutup: perintah masih hanya melihat sentinel, tetapi sentinel mencapai server tidak berubah dan otentikasi gagal. Claude Code melaporkan kesalahan konfigurasi ini saat startup.

Contoh di bawah topeng dua token. `GH_TOKEN` diganti hanya pada permintaan ke `api.github.com`, sementara `NPM_TOKEN` tidak memiliki `injectHosts` dan diganti pada permintaan ke setiap host dalam `network.allowedDomains`. Setiap entri `injectHosts` itu sendiri harus dicakup oleh `network.allowedDomains`.

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

Tidak seperti `deny`, masking mengotorisasi proxy untuk mengirim kredensial asli Anda ke host yang terdaftar, jadi itu dihormati hanya dari pengaturan yang Anda atau administrator Anda kontrol: pengaturan pengguna, pengaturan terkelola, dan bendera CLI `--settings`. Entri `mask`, `network.tlsTerminate`, dan [`credentials.allowPlaintextInject`](/docs/id/settings#sandbox-settings) dalam `.claude/settings.json` atau `.claude/settings.local.json` repositori diabaikan.

Ketika variabel yang sama terdaftar dengan `deny` dalam scope apa pun, `deny` mengambil prioritas.

<h2 id="how-sandboxing-works">
  Cara sandboxing bekerja
</h2>

<h3 id="filesystem-isolation">
  Isolasi filesystem
</h3>

Alat Bash sandboxed membatasi akses sistem file ke direktori tertentu:

* **Perilaku penulisan default**: akses baca dan tulis ke direktori kerja saat ini dan subdirektorinya, ditambah direktori temp sesi yang ditunjuk oleh `$TMPDIR`
* **Perilaku pembacaan default**: akses baca ke seluruh komputer, kecuali direktori tertentu yang ditolak. Perhatikan bahwa default ini masih memungkinkan pembacaan file kredensial seperti `~/.aws/credentials` dan `~/.ssh/`. Gunakan [`sandbox.credentials`](#protect-credentials) untuk memblokir pembacaan file-file ini dan membatalkan penetapan variabel lingkungan rahasia, atau tambahkan jalur ke `denyRead`.
* **Akses terblokir**: tidak dapat memodifikasi file di luar direktori kerja saat ini dan direktori temp sesi tanpa izin eksplisit, termasuk file konfigurasi shell seperti `~/.bashrc` dan binari sistem di `/bin/`
* **Git worktrees**: ketika direktori kerja adalah [linked git worktree](/docs/id/worktrees), sandbox juga memungkinkan penulisan ke direktori `.git` bersama dari repositori utama sehingga perintah seperti `git commit` dapat memperbarui refs dan indeks. Penulisan ke `hooks/` dan `config` di dalam direktori tersebut tetap ditolak.
* **Dapat dikonfigurasi**: tentukan jalur yang diizinkan dan ditolak khusus melalui pengaturan

Anda dapat memberikan akses tulis ke jalur tambahan menggunakan `sandbox.filesystem.allowWrite` dalam pengaturan Anda. Pembatasan ini diberlakukan pada tingkat OS, sehingga berlaku untuk semua perintah subprocess, termasuk alat seperti `kubectl`, `terraform`, dan `npm`, bukan hanya alat file Claude.

<h3 id="network-isolation">
  Isolasi jaringan
</h3>

Akses jaringan dikendalikan melalui server proxy yang berjalan di luar sandbox:

* **Pembatasan domain**: tidak ada domain yang diizinkan sebelumnya. Pertama kali perintah memerlukan domain baru, Claude Code meminta persetujuan. Mulai dari v2.1.191, memilih Ya memungkinkan host untuk sisa sesi saat ini, sehingga koneksi nanti ke host yang sama tidak meminta lagi. Izinkan domain sebelumnya dengan [`allowedDomains`](/docs/id/settings#sandbox-settings) untuk menghindari prompt sepenuhnya.
* **Lockdown terkelola**: jika [`allowManagedDomainsOnly`](/docs/id/settings#sandbox-settings) diatur dalam pengaturan terkelola, domain yang tidak diizinkan diblokir secara otomatis alih-alih prompt, dan hanya `allowedDomains` dari pengaturan terkelola yang dihormati.
* **Dukungan proxy khusus**: pengguna tingkat lanjut dapat menerapkan aturan khusus pada lalu lintas keluar
* **Cakupan komprehensif**: pembatasan berlaku untuk semua skrip, program, dan subprocess yang dihasilkan oleh perintah

<Note>
  Proxy bawaan memberlakukan allowlist berdasarkan hostname yang diminta dan, secara default, tidak menghentikan atau memeriksa lalu lintas TLS. Pengaturan eksperimental [`network.tlsTerminate`](/docs/id/settings#sandbox-settings), tersedia di Claude Code v2.1.199 dan yang lebih baru, membuat proxy bawaan menghentikan TLS itu sendiri, yang [`mask` credential entries](#protect-credentials) memerlukan. Lihat [Security limitations](#security-limitations) untuk implikasi default, dan [Custom proxy configuration](#custom-proxy-configuration) jika model ancaman Anda memerlukan inspeksi TLS.
</Note>

<h3 id="os-level-enforcement">
  Penegakan tingkat OS
</h3>

Alat Bash sandboxed memanfaatkan primitif keamanan sistem operasi:

* **macOS**: menggunakan Seatbelt untuk penegakan sandbox
* **Linux**: menggunakan [bubblewrap](https://github.com/containers/bubblewrap) untuk isolasi
* **WSL2**: menggunakan bubblewrap, sama seperti Linux

WSL1 tidak didukung karena bubblewrap memerlukan fitur kernel yang hanya tersedia di WSL2. Pembatasan tingkat OS ini memastikan bahwa semua proses anak yang dihasilkan oleh perintah Claude Code mewarisi batas keamanan yang sama.

Primitif yang sama tersedia sebagai paket [`@anthropic-ai/sandbox-runtime`](https://github.com/anthropic-experimental/sandbox-runtime) mandiri, yang halaman [Sandbox environments](/docs/id/sandbox-environments#sandbox-runtime) mencakup sebagai pendekatan terpisah untuk membungkus seluruh proses Claude Code.

<h2 id="how-sandboxing-relates-to-permissions-and-permission-modes">
  Bagaimana sandboxing berhubungan dengan izin dan mode izin
</h2>

Sandboxing, [permission rules](/docs/id/permissions), dan [permission modes](/docs/id/permission-modes) adalah lapisan komplementer. Bagian di bawah mencakup bagaimana sandbox berinteraksi dengan masing-masing.

<h3 id="permission-rules">
  Aturan izin
</h3>

Aturan izin dan sandboxing mengontrol hal yang berbeda:

* **Aturan izin** mengontrol alat mana yang dapat digunakan Claude Code dan dievaluasi sebelum alat apa pun berjalan. Mereka berlaku untuk semua alat: Bash, Read, Edit, WebFetch, MCP, dan lainnya.
* **Sandboxing** menyediakan penegakan tingkat OS yang membatasi apa yang dapat diakses perintah Bash pada tingkat filesystem dan jaringan. Ini hanya berlaku untuk perintah Bash dan proses anak mereka.

Kedua lapisan juga berbeda dalam cara penegakan mereka. Claude Code mengevaluasi keputusan izin sebelum perintah berjalan, berdasarkan string perintah dan, dalam mode auto, penilaian classifier terpisah tentang apakah perintah aman. Sistem operasi memberlakukan batas sandbox pada proses yang berjalan, sehingga berlaku terlepas dari apa yang dipilih model untuk dijalankan dan bahkan jika perintah yang diizinkan melakukan lebih dari nama yang disarankan.

Pembatasan filesystem dan jaringan dikonfigurasi melalui pengaturan sandbox dan aturan izin:

| Pengaturan atau aturan                                           | Apa yang dilakukannya                                                                                            |
| :--------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------- |
| `sandbox.filesystem.allowWrite`                                  | Memberikan akses tulis subprocess ke jalur di luar direktori kerja                                               |
| `sandbox.filesystem.denyWrite` dan `sandbox.filesystem.denyRead` | Memblokir akses subprocess ke jalur tertentu                                                                     |
| `sandbox.filesystem.allowRead`                                   | Mengizinkan kembali pembacaan jalur tertentu dalam wilayah `denyRead`                                            |
| Aturan izin `Edit`                                               | Memberikan akses tulis ke jalur tertentu, dengan cara yang sama seperti `sandbox.filesystem.allowWrite`          |
| Aturan tolak `Read` dan `Edit`                                   | Memblokir akses ke file atau direktori tertentu                                                                  |
| Aturan izin dan tolak `WebFetch`                                 | Mengontrol akses domain                                                                                          |
| Sandbox `allowedDomains`                                         | Mengontrol domain mana yang dapat dijangkau perintah Bash                                                        |
| Sandbox `deniedDomains`                                          | Memblokir domain tertentu bahkan ketika wildcard `allowedDomains` yang lebih luas akan sebaliknya mengizinkannya |

Jalur dari pengaturan `sandbox.filesystem` dan aturan izin digabungkan bersama ke dalam konfigurasi sandbox akhir.

[Direktori contoh repositori claude-code](https://github.com/anthropics/claude-code/tree/main/examples/settings) mencakup konfigurasi pengaturan pemula untuk skenario penyebaran umum, termasuk contoh khusus sandbox. Gunakan ini sebagai titik awal dan sesuaikan dengan kebutuhan Anda.

<h3 id="permission-modes">
  Mode izin
</h3>

`/sandbox` bukan [permission mode](/docs/id/permission-modes). Mode izin memutuskan apakah panggilan alat berjalan dan apakah Anda diminta terlebih dahulu, sementara sandbox membatasi apa yang dapat diakses perintah Bash setelah berjalan. Mereka berbeda dalam apa yang mereka kontrol dan apa yang menggantikan prompt per-aksi:

|                                                                    | Apa yang dikontrol                                    | Apa yang menggantikan prompt                                                                                                                                                                                                                                                                                                                                                                                                        |
| :----------------------------------------------------------------- | :---------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `/sandbox`                                                         | Apa yang dapat diakses perintah Bash setelah berjalan | Batas sandbox itu sendiri, dalam [mode auto-allow](#sandbox-modes)                                                                                                                                                                                                                                                                                                                                                                  |
| [Auto mode](/docs/id/permission-modes#eliminate-prompts-with-auto-mode) | Apakah setiap panggilan alat berjalan                 | Classifier yang meninjau tindakan                                                                                                                                                                                                                                                                                                                                                                                                   |
| `--dangerously-skip-permissions`                                   | Apakah setiap panggilan alat berjalan                 | Tidak ada. Pemeriksaan [Protected path](/docs/id/permission-modes#protected-paths) juga dilewati; hanya [ask rules](/docs/id/permissions#manage-permissions) eksplisit, connector tools [yang diatur organisasi Anda ke `ask`](/docs/id/mcp#organization-controls-on-connector-tools), MCP tools yang ditandai [`requiresUserInteraction`](/docs/id/mcp#require-approval-for-a-specific-tool), dan menghapus `/` atau direktori home Anda masih meminta |

Mode [auto-allow](#sandbox-modes) sandbox terpisah dari [auto mode](/docs/id/permission-modes#eliminate-prompts-with-auto-mode): auto-allow menyetujui perintah Bash karena batas sandbox memuatnya, sementara auto mode menggunakan classifier untuk meninjau tindakan. Keduanya bekerja secara independen dan dapat dikombinasikan. Untuk memilih batas isolasi untuk run tanpa pengawasan, lihat [Sandbox environments](/docs/id/sandbox-environments#how-isolation-relates-to-permission-modes).

<h2 id="configure-the-sandbox-for-your-organization">
  Konfigurasi sandbox untuk organisasi Anda
</h2>

Administrator dapat memerlukan sandboxing untuk setiap pengguna, mencegah pengembang memperluas kebijakan, dan merutekan lalu lintas sandbox melalui proxy perusahaan.

<h3 id="enforce-sandboxing-with-managed-settings">
  Memberlakukan sandboxing dengan pengaturan terkelola
</h3>

Untuk memerlukan sandbox untuk setiap pengembang, berikan kunci `sandbox` melalui [managed settings](/docs/id/settings#settings-files), baik sebagai file yang dikelola oleh MDM Anda atau melalui [server-managed settings](/docs/id/server-managed-settings) di Claude.ai.

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
* **`allowUnsandboxedCommands: false`**: pintu keluar `dangerouslyDisableSandbox` diabaikan, sehingga perintah yang gagal di bawah sandbox tidak dapat dicoba kembali di luar itu

Dua penambahan layak dipertimbangkan bersama mereka. Tambahkan `excludedCommands` untuk alat yang disetujui organisasi apa pun yang harus berjalan tanpa isolasi. Tambahkan entri [`sandbox.credentials`](#protect-credentials) untuk direktori kredensial seperti `~/.aws` dan `~/.ssh` dan untuk variabel lingkungan rahasia, karena kebijakan pembacaan default masih memungkinkan mereka.

Sandbox tidak berjalan di Windows asli, jadi jika armada Anda mencakup host Windows, batasi konfigurasi ini ke macOS dan Linux atau minta pengguna tersebut menjalankan Claude Code di dalam WSL2 atau container.

<h3 id="keep-developers-from-widening-the-policy">
  Cegah pengembang memperluas kebijakan
</h3>

Untuk kunci boolean seperti `enabled` dan `failIfUnavailable`, Claude Code menggunakan nilai terkelola dan mengabaikan apa pun yang ditetapkan pengembang secara lokal. Untuk kunci array seperti `excludedCommands` dan `allowRead`, Claude Code menggabungkan entri dari setiap scope, sehingga pengembang dapat menambahkan entri yang memperluas kebijakan.

Atur `allowManagedReadPathsOnly` ke `true` dalam pengaturan terkelola sehingga hanya entri `allowRead` dari pengaturan terkelola yang dihormati. Entri `allowRead` pengguna, proyek, dan lokal diabaikan. Ini mencegah pengembang memperluas akses baca di luar jalur yang disetujui organisasi. Untuk mengunci domain jaringan ke nilai terkelola dengan cara yang sama, atur [`allowManagedDomainsOnly`](/docs/id/settings#sandbox-settings).

`excludedCommands` tidak memiliki lockdown hanya terkelola yang setara, sehingga pengembang selalu dapat menambahkan entri yang menjalankan perintah tambahan di luar sandbox. Jaga daftar terkelola tetap sempit.

<h3 id="custom-proxy-configuration">
  Konfigurasi proxy khusus
</h3>

Untuk organisasi yang memerlukan keamanan jaringan lanjutan, Anda dapat menerapkan proxy khusus untuk:

* Mendekripsi dan memeriksa lalu lintas HTTPS
* Menerapkan aturan penyaringan khusus
* Mencatat semua permintaan jaringan
* Mengintegrasikan dengan infrastruktur keamanan yang ada

Untuk menunjukkan Claude Code ke proxy Anda, atur port proxy dalam [sandbox settings](/docs/id/settings#sandbox-settings):

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
* **Go-based CLIs gagal verifikasi TLS di macOS**: alat seperti `gh`, `gcloud`, dan `terraform` mungkin gagal verifikasi TLS di bawah Seatbelt. Daftar alat ini dalam `excludedCommands` untuk menjalankannya di luar sandbox. Jika Anda menggunakan `httpProxyPort` dengan proxy MITM dan CA khusus, atur [`enableWeakerNetworkIsolation`](/docs/id/settings#sandbox-settings) ke `true` sebagai gantinya.
* **Perintah `open`, `osascript`, atau alur autentikasi berbasis browser gagal dengan kesalahan `-600` di macOS**: sandbox memblokir Apple Events secara default. Atur [`allowAppleEvents`](/docs/id/settings#sandbox-settings) ke `true` dalam pengaturan pengguna, terkelola, atau CLI Anda untuk mengizinkannya. Pengaturan proyek diabaikan untuk kunci ini. Mengaktifkannya menghilangkan isolasi eksekusi kode, karena perintah sandboxed kemudian dapat meluncurkan aplikasi lain tanpa sandbox tanpa prompt pengguna dan mengirim perintah AppleScript ke aplikasi yang berjalan, tunduk pada prompt otomasi-persetujuan macOS (TCC). Alternatifnya, tambahkan perintah ke `excludedCommands` untuk menjalankannya di luar sandbox.
* **Perintah `docker` gagal**: `docker` tidak kompatibel dengan sandbox. Tambahkan `docker *` ke `excludedCommands` untuk menjalankannya di luar sandbox.
* **Bubblewrap gagal memulai di dalam container**: dalam container tanpa privilege, bubblewrap tidak dapat memasang filesystem `/proc` segar. Atur [`enableWeakerNestedSandbox`](/docs/id/settings#sandbox-settings) ke `true` sehingga sandbox dalam bind-mount `/proc` yang ada dari container sebagai gantinya. Hanya gunakan pengaturan ini ketika container luar sudah menyediakan batas isolasi yang Anda butuhkan, karena mengekspos informasi proses ke perintah sandboxed yang mount `/proc` segar akan menyembunyikan.
* **Filter seccomp di Linux**: filter seccomp diperlukan untuk memblokir soket domain Unix. Tab Dependencies dalam `/sandbox` menunjukkan apakah tersedia. Jika hilang, jalankan `npm install -g @anthropic-ai/sandbox-runtime` untuk menginstal helper.
* **`--dangerously-skip-permissions` gagal sebagai root**: flag ini diblokir saat menjalankan sebagai root atau melalui sudo di Linux dan macOS, karena akses root dikombinasikan dengan tidak ada prompt izin dapat memodifikasi file atau layanan apa pun di sistem. Pemeriksaan dilewati secara otomatis di dalam sandbox yang dikenali. Untuk menjalankan secara otonom dalam container, gunakan konfigurasi [dev container](/docs/id/devcontainer), yang menjalankan Claude Code sebagai pengguna non-root.

<h2 id="limitations">
  Keterbatasan
</h2>

Sandboxing mengurangi risiko tetapi bukan batas isolasi lengkap. Tinjau keterbatasan di bawah sebelum mengandalkannya sebagai kontrol keamanan keras.

<h3 id="security-limitations">
  Keterbatasan keamanan
</h3>

* **Penyaringan jaringan**: sandbox membatasi domain mana yang dapat terhubung oleh proses. Secara default, proxy bawaan tidak menghentikan atau melakukan inspeksi TLS pada lalu lintas keluar, sehingga isi koneksi terenkripsi tidak diperiksa. Pengaturan eksperimental [`network.tlsTerminate`](/docs/id/settings#sandbox-settings) menghentikan TLS di proxy untuk [substitusi kredensial `mask`](#protect-credentials) tetapi tidak menambahkan penyaringan konten. Anda bertanggung jawab untuk memastikan bahwa hanya domain tepercaya yang diizinkan dalam kebijakan Anda.

<Warning>
  Mengizinkan domain luas seperti `github.com` dapat membuat jalur untuk eksfiltrasi data. Karena proxy membuat keputusan izin dari hostname yang disediakan klien tanpa memeriksa TLS, kode yang berjalan di dalam sandbox berpotensi dapat menggunakan [domain fronting](https://en.wikipedia.org/wiki/Domain_fronting) atau teknik serupa untuk menjangkau host di luar allowlist. Jika model ancaman Anda memerlukan jaminan yang lebih kuat, konfigurasikan [custom proxy](#custom-proxy-configuration) yang menghentikan TLS dan memeriksa lalu lintas, dan instal sertifikat CA-nya di dalam sandbox. Isolasi jaringan yang lebih kuat dan sadar TLS adalah area pengembangan aktif.
</Warning>

* **Eskalasi privilege melalui soket Unix**: konfigurasi `allowUnixSockets` dapat secara tidak sengaja memberikan akses ke layanan sistem yang kuat yang dapat menyebabkan bypass sandbox. Misalnya, mengizinkan akses ke `/var/run/docker.sock` secara efektif memberikan akses ke sistem host melalui soket Docker. Pertimbangkan dengan hati-hati soket Unix apa pun yang Anda izinkan melalui sandbox.
* **Eskalasi izin filesystem**: izin penulisan filesystem yang terlalu luas dapat memungkinkan serangan eskalasi privilege. Mengizinkan penulisan ke direktori yang berisi executable dalam `$PATH`, direktori konfigurasi sistem, atau file konfigurasi shell pengguna seperti `.bashrc` atau `.zshrc` dapat menyebabkan eksekusi kode dalam konteks keamanan yang berbeda ketika pengguna lain atau proses sistem mengakses file ini.
* **Kekuatan sandbox Linux**: implementasi Linux menyediakan isolasi filesystem dan jaringan yang kuat tetapi mencakup mode `enableWeakerNestedSandbox` yang memungkinkannya bekerja di dalam lingkungan Docker tanpa namespace istimewa, atau pada host Linux di mana user namespaces tanpa privilege dinonaktifkan oleh sysctl. Opsi ini secara konsiderabel melemahkan keamanan dan hanya boleh digunakan ketika isolasi tambahan sebaliknya diberlakukan.
* **Apple Events pada macOS**: sandbox macOS memblokir Apple Events secara default. Pengaturan `allowAppleEvents` menghapus pembatasan ini sehingga alat seperti `open` dan `osascript` berfungsi, tetapi menghilangkan isolasi eksekusi kode: perintah sandboxed dapat meluncurkan aplikasi lain tanpa sandbox tanpa prompt pengguna, dan dapat mengirim perintah AppleScript ke aplikasi yang sedang berjalan, tunduk pada prompt persetujuan otomasi macOS per-aplikasi (TCC). Ini hanya dihormati dari pengaturan pengguna, terkelola, atau CLI. Pengaturan proyek tidak dapat mengaktifkannya.
* **File pengaturan dilindungi**: sandbox secara otomatis menolak akses tulis ke file `settings.json` Claude Code di setiap scope dan ke direktori pengaturan terkelola, sehingga perintah sandboxed tidak dapat memodifikasi kebijakan sendiri.

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
* **Variabel lingkungan**: perintah Bash sandboxed mewarisi lingkungan proses induk secara default, termasuk kredensial apa pun yang ditetapkan di sana. Gunakan [`sandbox.credentials`](#protect-credentials) untuk menghapus atau menutupi variabel tertentu untuk perintah sandboxed, atau atur [`CLAUDE_CODE_SUBPROCESS_ENV_SCRUB`](/docs/id/env-vars) untuk menghapus kredensial Anthropic dan penyedia cloud dari semua subprocess.
* **Subagents**: [subagents](/docs/id/sub-agents) berjalan dalam proses yang sama dengan sesi induk dan menggunakan konfigurasi sandbox yang sama. Perintah Bash di dalam subagent di-sandbox ketika sandboxing diaktifkan dalam sesi induk.

<Warning>
  Sandboxing yang efektif memerlukan isolasi filesystem dan jaringan. Tanpa isolasi jaringan, agen yang dikompromikan dapat mengeksfiltrasikan file sensitif seperti kunci SSH. Tanpa isolasi filesystem, agen yang dikompromikan dapat memasang pintu belakang pada sumber daya sistem untuk mendapatkan akses jaringan. Ketika Anda memperluas default, periksa bahwa jalur `allowWrite`, entri `allowedDomains` yang luas, atau pengecualian `excludedCommands` tidak membatalkan pembatasan di sisi lain.
</Warning>

<h2 id="see-also">
  Lihat juga
</h2>

* [Sandbox environments](/docs/id/sandbox-environments): bandingkan sandbox bawaan dengan dev containers, containers, dan VM
* [Security](/docs/id/security): fitur keamanan komprehensif dan praktik terbaik
* [Permissions](/docs/id/permissions): konfigurasi izin dan kontrol akses
* [Settings](/docs/id/settings): referensi konfigurasi lengkap
* [CLI reference](/docs/id/cli-reference): opsi baris perintah
