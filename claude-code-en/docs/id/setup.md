> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Pengaturan lanjutan

> Persyaratan sistem, instalasi khusus platform, manajemen versi, dan penghapusan instalasi untuk Claude Code.

Halaman ini mencakup persyaratan sistem, detail instalasi khusus platform, pembaruan, dan penghapusan instalasi. Untuk panduan langkah demi langkah sesi pertama Anda, lihat [quickstart](/docs/id/quickstart). Jika Anda belum pernah menggunakan terminal sebelumnya, lihat [panduan terminal](/docs/id/terminal-guide).

<h2 id="system-requirements">
  Persyaratan sistem
</h2>

Claude Code berjalan pada platform dan konfigurasi berikut:

* **Sistem operasi**:
  * macOS 13.0+
  * Windows 10 1809+ atau Windows Server 2019+
  * Ubuntu 20.04+
  * Debian 10+
  * Alpine Linux 3.19+
* **Perangkat keras**: RAM 4 GB+, prosesor x64 atau ARM64
* **Jaringan**: koneksi internet diperlukan. Lihat [konfigurasi jaringan](/docs/id/network-config#network-access-requirements).
* **Shell**: Bash, Zsh, PowerShell, atau CMD.
* **Lokasi**: [negara yang didukung Anthropic](https://www.anthropic.com/supported-countries)

<h3 id="additional-dependencies">
  Dependensi tambahan
</h3>

* **ripgrep**: biasanya disertakan dengan Claude Code. Jika pencarian gagal, lihat [troubleshooting pencarian](/docs/id/troubleshooting#search-and-discovery-issues).

<h2 id="install-claude-code">
  Instal Claude Code
</h2>

<Tip>
  Lebih suka antarmuka grafis? [Aplikasi Desktop](/docs/id/desktop-quickstart) memungkinkan Anda menggunakan Claude Code tanpa terminal. Unduh untuk [macOS](https://claude.ai/api/desktop/darwin/universal/dmg/latest/redirect?utm_source=claude_code\&utm_medium=docs), [Windows](https://claude.com/download?utm_source=claude_code\&utm_medium=docs), atau [Linux](/docs/id/desktop-linux).

  Baru mengenal terminal? Lihat [panduan terminal](/docs/id/terminal-guide) untuk instruksi langkah demi langkah.
</Tip>

To install Claude Code, use one of the following methods:

<Tabs>
  <Tab title="Native Install (Recommended)">
    **macOS, Linux, WSL:**

    ```bash theme={null}
    curl -fsSL https://claude.ai/install.sh | bash
    ```

    **Windows PowerShell:**

    ```powershell theme={null}
    irm https://claude.ai/install.ps1 | iex
    ```

    **Windows CMD:**

    ```batch theme={null}
    curl -fsSL https://claude.ai/install.cmd -o install.cmd && install.cmd && del install.cmd
    ```

    If you see `The token '&&' is not a valid statement separator`, you're in PowerShell, not CMD. If you see `'irm' is not recognized as an internal or external command`, you're in CMD, not PowerShell. Your prompt shows `PS C:\` when you're in PowerShell and `C:\` without the `PS` when you're in CMD.

    If the install command fails with `syntax error near unexpected token '<'`, a `403`, or another curl error, see [Troubleshoot installation](/docs/en/troubleshoot-install#find-your-error) to match the error to a fix and for alternative install methods.

    [Git for Windows](https://git-scm.com/downloads/win) is recommended on native Windows so Claude Code can use the Bash tool. If Git for Windows is not installed, Claude Code uses PowerShell as the shell tool instead. WSL setups do not need Git for Windows.

    <Info>
      Native installations automatically update in the background to keep you on the latest version.
    </Info>
  </Tab>

  <Tab title="Homebrew">
    ```bash theme={null}
    brew install --cask claude-code
    ```

    Homebrew offers two casks. `claude-code` tracks the stable release channel, which is typically about a week behind and skips releases with major regressions. `claude-code@latest` tracks the latest channel and receives new versions as soon as they ship.

    <Info>
      Homebrew installations do not auto-update. Run `brew upgrade claude-code` or `brew upgrade claude-code@latest`, depending on which cask you installed, to get the latest features and security fixes.
    </Info>
  </Tab>

  <Tab title="WinGet">
    ```powershell theme={null}
    winget install Anthropic.ClaudeCode
    ```

    <Info>
      WinGet installations do not auto-update. Run `winget upgrade Anthropic.ClaudeCode` periodically to get the latest features and security fixes.
    </Info>
  </Tab>
</Tabs>

You can also install with [apt, dnf, or apk](/docs/en/setup#install-with-linux-package-managers) on Debian, Fedora, RHEL, and Alpine.

Setelah instalasi selesai, buka terminal di proyek yang ingin Anda kerjakan dan mulai Claude Code:

```bash theme={null}
claude
```

Jika Anda mengalami masalah apa pun selama instalasi, lihat [Troubleshoot installation and login](/docs/id/troubleshoot-install).

<h3 id="set-up-on-windows">
  Pengaturan di Windows
</h3>

Anda dapat menjalankan Claude Code secara asli di Windows atau di dalam WSL. Pilih berdasarkan di mana proyek Anda berada dan fitur apa yang Anda butuhkan:

| Opsi         | Memerlukan                                                                        | [Sandboxing](/docs/id/sandboxing) | Kapan digunakan                                   |
| ------------ | --------------------------------------------------------------------------------- | ---------------------------- | ------------------------------------------------- |
| Windows Asli | Tidak ada; [Git for Windows](https://git-scm.com/downloads/win) bersifat opsional | Tidak didukung               | Proyek dan alat Windows asli                      |
| WSL 2        | WSL 2 diaktifkan                                                                  | Didukung                     | Toolchain Linux atau eksekusi perintah bersandbox |
| WSL 1        | WSL 1 diaktifkan                                                                  | Tidak didukung               | Jika WSL 2 tidak tersedia                         |

**Opsi 1: Windows Asli**

Jalankan perintah instalasi dari PowerShell atau CMD. Anda tidak perlu menjalankan sebagai Administrator. Menginstal [Git for Windows](https://git-scm.com/downloads/win) bersifat opsional. Ini mengaktifkan [alat Bash](/docs/id/tools-reference#bash-tool-behavior) dengan menyediakan Git Bash.

Apakah Anda menginstal dari PowerShell atau CMD hanya mempengaruhi perintah instalasi mana yang Anda jalankan. Prompt Anda menampilkan `PS C:\Users\YourName>` di PowerShell dan `C:\Users\YourName>` tanpa `PS` di CMD. Jika Anda baru mengenal terminal, [panduan terminal](/docs/id/terminal-guide#windows) memandu setiap langkah.

Setelah instalasi, luncurkan `claude` dari terminal apa pun.

* **Tanpa Git for Windows**, Claude Code menjalankan perintah shell melalui [alat PowerShell](/docs/id/tools-reference#powershell-tool).
* **Dengan Git for Windows**, Claude Code menggunakan Git Bash untuk [alat Bash](/docs/id/tools-reference#bash-tool-behavior). Jika Claude Code tidak dapat menemukan Git Bash, atur jalur di [file settings.json](/docs/id/settings) Anda:

  ```json theme={null}
  {
    "env": {
      "CLAUDE_CODE_GIT_BASH_PATH": "C:\\Program Files\\Git\\bin\\bash.exe"
    }
  }
  ```

Ketika Git for Windows diinstal, alat PowerShell sedang diluncurkan secara progresif sebagai opsi tambahan bersama Bash. Atur `CLAUDE_CODE_USE_POWERSHELL_TOOL=1` untuk memilih masuk atau `0` untuk memilih keluar. Lihat [alat PowerShell](/docs/id/tools-reference#powershell-tool) untuk pengaturan dan batasan.

**Opsi 2: WSL**

Buka distribusi WSL Anda dan jalankan penginstal Linux dari [instruksi instalasi](#install-claude-code) di atas. Anda menginstal dan meluncurkan `claude` di dalam terminal WSL, bukan dari PowerShell atau CMD.

<h3 id="alpine-linux-and-musl-based-distributions">
  Alpine Linux dan distribusi berbasis musl
</h3>

Penginstal asli di Alpine dan distribusi berbasis musl/uClibc lainnya memerlukan `libgcc`, `libstdc++`, dan `ripgrep`. Instal ini menggunakan manajer paket distribusi Anda, kemudian atur `USE_BUILTIN_RIPGREP=0`.

Contoh ini menginstal paket yang diperlukan di Alpine:

```bash theme={null}
apk add libgcc libstdc++ ripgrep
```

Kemudian atur `USE_BUILTIN_RIPGREP` ke `0` di file [`settings.json`](/docs/id/settings#available-settings) Anda:

```json theme={null}
{
  "env": {
    "USE_BUILTIN_RIPGREP": "0"
  }
}
```

<h2 id="verify-your-installation">
  Verifikasi instalasi Anda
</h2>

Setelah menginstal, konfirmkan Claude Code berfungsi:

```bash theme={null}
claude --version
```

Jika ini gagal dengan `command not found` atau kesalahan lainnya, lihat [Troubleshoot installation and login](/docs/id/troubleshoot-install).

Untuk pemeriksaan yang lebih terperinci tentang instalasi dan konfigurasi Anda, jalankan [`claude doctor`](/docs/id/troubleshooting#get-more-help):

```bash theme={null}
claude doctor
```

<h2 id="authenticate">
  Autentikasi
</h2>

Claude Code memerlukan akun Pro, Max, Team, Enterprise, atau Console. Paket Claude.ai gratis tidak termasuk akses Claude Code. Anda juga dapat menggunakan Claude Code dengan penyedia API pihak ketiga seperti [Amazon Bedrock](/docs/id/amazon-bedrock), [Google Cloud's Agent Platform](/docs/id/google-vertex-ai), atau [Microsoft Foundry](/docs/id/microsoft-foundry).

Setelah menginstal, masuk dengan menjalankan `claude` dan mengikuti petunjuk browser. Lihat [Autentikasi](/docs/id/authentication) untuk semua jenis akun dan opsi pengaturan tim.

<h2 id="update-claude-code">
  Perbarui Claude Code
</h2>

Instalasi asli secara otomatis diperbarui di latar belakang. Anda dapat [mengonfigurasi saluran rilis](#configure-release-channel) untuk mengontrol apakah Anda menerima pembaruan segera atau sesuai jadwal stabil yang tertunda, atau [menonaktifkan pembaruan otomatis](#disable-auto-updates) sepenuhnya. Instalasi Homebrew, WinGet, dan [manajer paket Linux](#install-with-linux-package-managers) memerlukan pembaruan manual secara default.

<h3 id="auto-updates">
  Pembaruan otomatis
</h3>

Claude Code memeriksa pembaruan saat startup dan secara berkala saat berjalan. Pembaruan diunduh dan diinstal di latar belakang, kemudian berlaku saat Anda memulai Claude Code berikutnya.

Jalankan `claude doctor` untuk melihat hasil upaya pembaruan terbaru.

Di macOS dan Linux, installer asli mengelola launcher di `~/.local/bin/claude` sebagai symlink ke `~/.local/share/claude/versions/`. Jika Anda mengganti launcher itu dengan skrip atau symlink Anda sendiri, auto-update dan `claude update` membiarkannya tetap ada: versi baru masih diinstal di bawah direktori `versions/`, dan launcher Anda memutuskan versi mana yang berjalan. Sebelum v2.1.207, auto-updater mengganti launcher kustom di jalur itu dengan symlink miliknya sendiri pada setiap pembaruan.

Dengan launcher kustom, Claude Code juga menyimpan setiap versi yang diinstal di disk karena tidak dapat menentukan versi mana yang dibutuhkan launcher. `claude doctor` melaporkan launcher yang tidak dibuat oleh installer asli.

Untuk membiarkan Claude Code mengelola launcher lagi, hapus `~/.local/bin/claude` dan jalankan `claude update`.

Jika instalasi npm global tidak dapat auto-update karena direktori global npm tidak dapat ditulis, Claude Code menampilkan pemberitahuan satu kali saat startup, dan `claude doctor` mencantumkan perbaikan yang tersedia. Lihat [permission errors during installation](/docs/id/troubleshoot-install#permission-errors-during-installation) untuk detail.

<Note>
  Instalasi Homebrew, WinGet, apt, dnf, dan apk tidak auto-update secara default; lihat di bawah untuk memilih masuk untuk Homebrew dan WinGet. Untuk upgrade Homebrew secara manual, jalankan `brew upgrade claude-code` atau `brew upgrade claude-code@latest`, tergantung cask mana yang Anda instal. Untuk WinGet, jalankan `winget upgrade Anthropic.ClaudeCode`. Untuk manajer paket Linux, lihat perintah upgrade di [Install with Linux package managers](#install-with-linux-package-managers).

  Untuk membuat Claude Code menjalankan perintah upgrade untuk Anda di Homebrew atau WinGet, atur [`CLAUDE_CODE_PACKAGE_MANAGER_AUTO_UPDATE`](/docs/id/env-vars) ke `1`. Claude Code kemudian menjalankan upgrade di latar belakang ketika versi baru tersedia dan menampilkan prompt restart saat berhasil. Upgrade menargetkan hanya paket Claude Code dan tidak mempengaruhi perangkat lunak lain yang telah Anda instal.

  Di WinGet upgrade mungkin gagal saat Claude Code berjalan karena Windows mengunci executable. Dalam hal itu Claude Code menampilkan perintah manual sebagai gantinya. apt, dnf, dan apk terus memerlukan upgrade manual karena perintah tersebut memerlukan privilege yang ditingkatkan.

  **Masalah yang diketahui:** Claude Code dapat memberi tahu Anda tentang pembaruan sebelum versi baru tersedia di manajer paket ini. Jika upgrade gagal, tunggu dan coba lagi nanti.

  Homebrew menyimpan versi lama di disk setelah upgrade. Jalankan `brew cleanup` secara berkala untuk membebaskan ruang disk.
</Note>

<h3 id="configure-release-channel">
  Konfigurasi saluran rilis
</h3>

Kontrol saluran rilis mana yang diikuti Claude Code untuk pembaruan otomatis dan `claude update` dengan pengaturan `autoUpdatesChannel`:

* `"latest"`, default: terima fitur baru segera setelah dirilis
* `"stable"`: gunakan versi yang biasanya sekitar satu minggu lama, lewati rilis dengan regresi besar

Konfigurasi ini melalui `/config` → **Auto-update channel**, atau tambahkan ke [file settings.json](/docs/id/settings) Anda:

```json theme={null}
{
  "autoUpdatesChannel": "stable"
}
```

Untuk penerapan enterprise, Anda dapat memberlakukan saluran rilis yang konsisten di seluruh organisasi Anda menggunakan [managed settings](/docs/id/permissions#managed-settings).

Instalasi Homebrew memilih saluran berdasarkan nama cask sebagai gantinya: `claude-code` melacak stable dan `claude-code@latest` melacak latest.

<h3 id="pin-a-minimum-version">
  Tetapkan versi minimum
</h3>

Pengaturan `minimumVersion` menetapkan batas bawah. Pembaruan otomatis latar belakang dan `claude update` menolak untuk menginstal versi apa pun di bawah nilai ini, jadi beralih ke saluran `"stable"` tidak menurunkan Anda jika Anda sudah di build `"latest"` yang lebih baru.

Beralih dari `"latest"` ke `"stable"` melalui `/config` meminta Anda untuk tetap di versi saat ini atau memungkinkan downgrade. Memilih untuk tetap menetapkan `minimumVersion` ke versi itu. Beralih kembali ke `"latest"` menghapusnya.

Tambahkan ke [file settings.json](/docs/id/settings) Anda untuk menetapkan batas secara eksplisit:

```json theme={null}
{
  "autoUpdatesChannel": "stable",
  "minimumVersion": "2.1.100"
}
```

Dalam [managed settings](/docs/id/permissions#managed-settings), ini memberlakukan minimum di seluruh organisasi yang tidak dapat ditimpa oleh pengaturan pengguna dan proyek.

Pengaturan `minimumVersion` hanya membatasi pembaruan. Untuk membuat Claude Code menolak untuk memulai di luar rentang versi, gunakan pengaturan terkelola `requiredMinimumVersion` dan `requiredMaximumVersion` sebagai gantinya. Pembaruan juga menghormati batas `requiredMaximumVersion`. Lihat [available settings](/docs/id/settings#available-settings).

<h3 id="disable-auto-updates">
  Nonaktifkan pembaruan otomatis
</h3>

Atur `DISABLE_AUTOUPDATER` ke `"1"` di kunci `env` dari file [`settings.json`](/docs/id/settings#available-settings) Anda:

```json theme={null}
{
  "env": {
    "DISABLE_AUTOUPDATER": "1"
  }
}
```

`DISABLE_AUTOUPDATER` hanya menghentikan pemeriksaan latar belakang; `claude update` dan `claude install` masih berfungsi. Untuk memblokir semua jalur pembaruan, termasuk pembaruan manual, atur [`DISABLE_UPDATES`](/docs/id/env-vars) sebagai gantinya. Gunakan ini ketika Anda mendistribusikan Claude Code melalui saluran Anda sendiri dan perlu pengguna tetap di versi yang Anda sediakan.

<h3 id="update-manually">
  Perbarui secara manual
</h3>

Untuk menerapkan pembaruan segera tanpa menunggu pemeriksaan latar belakang berikutnya, jalankan:

```bash theme={null}
claude update
```

<h2 id="advanced-installation-options">
  Opsi instalasi lanjutan
</h2>

Opsi ini untuk version pinning, manajer paket Linux, npm, dan verifikasi integritas biner.

<h3 id="install-a-specific-version">
  Instal versi tertentu
</h3>

Penginstal asli menerima nomor versi tertentu atau saluran rilis (`latest` atau `stable`). Saluran yang Anda pilih saat instalasi menjadi default Anda untuk pembaruan otomatis. Lihat [konfigurasi saluran rilis](#configure-release-channel) untuk informasi lebih lanjut.

Untuk menginstal versi terbaru (default):

<Tabs>
  <Tab title="macOS, Linux, WSL">
    ```bash theme={null}
    curl -fsSL https://claude.ai/install.sh | bash
    ```
  </Tab>

  <Tab title="Windows PowerShell">
    ```powershell theme={null}
    irm https://claude.ai/install.ps1 | iex
    ```
  </Tab>

  <Tab title="Windows CMD">
    ```batch theme={null}
    curl -fsSL https://claude.ai/install.cmd -o install.cmd && install.cmd && del install.cmd
    ```
  </Tab>
</Tabs>

Untuk menginstal versi stabil:

<Tabs>
  <Tab title="macOS, Linux, WSL">
    ```bash theme={null}
    curl -fsSL https://claude.ai/install.sh | bash -s stable
    ```
  </Tab>

  <Tab title="Windows PowerShell">
    ```powershell theme={null}
    & ([scriptblock]::Create((irm https://claude.ai/install.ps1))) stable
    ```
  </Tab>

  <Tab title="Windows CMD">
    ```batch theme={null}
    curl -fsSL https://claude.ai/install.cmd -o install.cmd && install.cmd stable && del install.cmd
    ```
  </Tab>
</Tabs>

Untuk menginstal nomor versi tertentu:

<Tabs>
  <Tab title="macOS, Linux, WSL">
    ```bash theme={null}
    curl -fsSL https://claude.ai/install.sh | bash -s 2.1.89
    ```
  </Tab>

  <Tab title="Windows PowerShell">
    ```powershell theme={null}
    & ([scriptblock]::Create((irm https://claude.ai/install.ps1))) 2.1.89
    ```
  </Tab>

  <Tab title="Windows CMD">
    ```batch theme={null}
    curl -fsSL https://claude.ai/install.cmd -o install.cmd && install.cmd 2.1.89 && del install.cmd
    ```
  </Tab>
</Tabs>

<h3 id="install-with-linux-package-managers">
  Instal dengan manajer paket Linux
</h3>

Claude Code menerbitkan repositori apt, dnf, dan apk yang ditandatangani. Setiap repositori menawarkan dua saluran: `stable` melayani versi yang biasanya sekitar satu minggu lama, melewati rilis dengan regresi besar, dan `latest` melayani setiap rilis segera setelah dikirim. Perintah di bawah mengonfigurasi saluran `stable`, yang sesuai untuk sebagian besar pengguna; setiap tab juga menunjukkan URL repositori `latest`. Instalasi manajer paket tidak auto-update melalui Claude Code; pembaruan tiba melalui alur upgrade sistem normal Anda.

Semua repositori ditandatangani dengan [kunci penandatanganan rilis Claude Code](#binary-integrity-and-code-signing). Sebelum mempercayai kunci, verifikasi seperti yang dijelaskan di setiap tab.

<Tabs>
  <Tab title="apt">
    Untuk Debian dan Ubuntu. Perintah instalasi di bawah mengunduh kunci penandatanganan dengan `curl`, yang instalasi Debian dan Ubuntu segar mungkin tidak sertakan. Jika unduhan gagal dengan `sudo: curl: command not found`, instal curl terlebih dahulu:

    ```bash theme={null}
    sudo apt install curl
    ```

    Perintah berikut mengonfigurasi saluran `stable`:

    ```bash theme={null}
    sudo install -d -m 0755 /etc/apt/keyrings
    sudo curl -fsSL https://downloads.claude.ai/keys/claude-code.asc \
      -o /etc/apt/keyrings/claude-code.asc
    echo "deb [signed-by=/etc/apt/keyrings/claude-code.asc] https://downloads.claude.ai/claude-code/apt/stable stable main" \
      | sudo tee /etc/apt/sources.list.d/claude-code.list
    sudo apt update
    sudo apt install claude-code
    ```

    Untuk menggunakan saluran `latest` sebagai gantinya, jalur URL dan nama suite keduanya berubah. Gunakan baris `deb` ini:

    ```bash theme={null}
    echo "deb [signed-by=/etc/apt/keyrings/claude-code.asc] https://downloads.claude.ai/claude-code/apt/latest latest main" \
      | sudo tee /etc/apt/sources.list.d/claude-code.list
    ```

    Verifikasi sidik jari kunci GPG sebelum mempercayainya: `gpg --show-keys /etc/apt/keyrings/claude-code.asc` harus melaporkan `31DD DE24 DDFA B679 F42D 7BD2 BAA9 29FF 1A7E CACE`.

    Untuk upgrade nanti, jalankan `sudo apt update && sudo apt upgrade claude-code`.
  </Tab>

  <Tab title="dnf">
    Untuk Fedora dan RHEL. Perintah berikut mengonfigurasi saluran `stable`:

    ```bash theme={null}
    sudo tee /etc/yum.repos.d/claude-code.repo <<'EOF'
    [claude-code]
    name=Claude Code
    baseurl=https://downloads.claude.ai/claude-code/rpm/stable
    enabled=1
    gpgcheck=1
    gpgkey=https://downloads.claude.ai/keys/claude-code.asc
    EOF
    sudo dnf install claude-code
    ```

    Untuk menggunakan saluran `latest` sebagai gantinya, atur `baseurl` ke repositori `latest`:

    ```ini theme={null}
    baseurl=https://downloads.claude.ai/claude-code/rpm/latest
    ```

    dnf mengunduh kunci pada instalasi pertama dan meminta Anda untuk mengkonfirmasi sidik jari. Verifikasi itu cocok dengan `31DD DE24 DDFA B679 F42D 7BD2 BAA9 29FF 1A7E CACE` sebelum menerima.

    Untuk upgrade nanti, jalankan `sudo dnf upgrade claude-code`.
  </Tab>

  <Tab title="apk">
    Untuk Alpine Linux. Perintah berikut mengonfigurasi saluran `stable`:

    ```sh theme={null}
    wget -O /etc/apk/keys/claude-code.rsa.pub \
      https://downloads.claude.ai/keys/claude-code.rsa.pub
    echo "https://downloads.claude.ai/claude-code/apk/stable" >> /etc/apk/repositories
    apk add claude-code
    ```

    Untuk beralih ke saluran `latest`, hapus baris repositori `stable` dan tambahkan repositori `latest`:

    ```sh theme={null}
    sed -i '\|downloads.claude.ai/claude-code/apk/stable|d' /etc/apk/repositories
    echo "https://downloads.claude.ai/claude-code/apk/latest" >> /etc/apk/repositories
    ```

    Verifikasi kunci yang diunduh dengan `sha256sum /etc/apk/keys/claude-code.rsa.pub`, yang harus melaporkan `395759c1f7449ef4cdef305a42e820f3c766d6090d142634ebdb049f113168b6`.

    Untuk upgrade nanti, jalankan `apk update && apk upgrade claude-code`.
  </Tab>
</Tabs>

<h3 id="install-with-npm">
  Instal dengan npm
</h3>

Anda juga dapat menginstal Claude Code sebagai paket npm global. Mulai dari v2.1.198, paket npm memerlukan [Node.js 22 atau lebih baru](https://nodejs.org/en/download). Pada versi Node.js yang lebih lama, npm mencetak peringatan `EBADENGINE` selama instalasi daripada gagal; instalasi selesai dan `claude` masih berjalan, karena paket mengunduh biner asli yang tidak menggunakan Node.js Anda saat runtime.

```bash theme={null}
npm install -g @anthropic-ai/claude-code
```

Paket npm menginstal biner asli yang sama dengan penginstal standalone. npm menarik biner melalui dependensi opsional per-platform seperti `@anthropic-ai/claude-code-darwin-arm64`, dan langkah postinstall menautkannya ke tempat. Biner `claude` yang terinstal tidak sendiri memanggil Node.

Platform instalasi npm yang didukung adalah `darwin-arm64`, `darwin-x64`, `linux-x64`, `linux-arm64`, `linux-x64-musl`, `linux-arm64-musl`, `win32-x64`, dan `win32-arm64`. Manajer paket Anda harus memungkinkan dependensi opsional. Lihat [troubleshooting](/docs/id/troubleshoot-install#native-binary-not-found-after-npm-install) jika biner hilang setelah instalasi.

Untuk upgrade instalasi npm, jalankan `npm install -g @anthropic-ai/claude-code@latest`. Hindari `npm update -g`, yang menghormati rentang semver dari instalasi asli dan mungkin tidak membawa Anda ke rilis terbaru.

<Warning>
  JANGAN gunakan `sudo npm install -g` karena ini dapat menyebabkan masalah izin dan risiko keamanan. Jika Anda mengalami kesalahan izin, lihat [troubleshooting kesalahan izin](/docs/id/troubleshoot-install#permission-errors-during-installation).
</Warning>

<h3 id="binary-integrity-and-code-signing">
  Integritas biner dan penandatanganan kode
</h3>

Setiap rilis menerbitkan `manifest.json` yang berisi checksum SHA256 untuk setiap biner platform. Manifes ditandatangani dengan kunci GPG Anthropic, jadi memverifikasi tanda tangan pada manifes secara transitif memverifikasi setiap biner yang tercantum.

<h4 id="verify-the-manifest-signature">
  Verifikasi tanda tangan manifes
</h4>

Langkah-langkah 1-3 memerlukan shell POSIX dengan `gpg` dan `curl`. Di Windows, jalankan di Git Bash atau WSL. Langkah 4 mencakup opsi PowerShell.

<Steps>
  <Step title="Unduh dan impor kunci publik">
    Kunci penandatanganan rilis dipublikasikan di URL tetap.

    ```bash theme={null}
    curl -fsSL https://downloads.claude.ai/keys/claude-code.asc | gpg --import
    ```

    Tampilkan sidik jari kunci yang diimpor.

    ```bash theme={null}
    gpg --fingerprint security@anthropic.com
    ```

    Konfirmasi output mencakup sidik jari ini:

    ```text theme={null}
    31DD DE24 DDFA B679 F42D  7BD2 BAA9 29FF 1A7E CACE
    ```
  </Step>

  <Step title="Unduh manifes dan tanda tangan">
    Atur `VERSION` ke rilis yang ingin Anda verifikasi.

    ```bash theme={null}
    REPO=https://downloads.claude.ai/claude-code-releases
    VERSION=2.1.89
    curl -fsSLO "$REPO/$VERSION/manifest.json"
    curl -fsSLO "$REPO/$VERSION/manifest.json.sig"
    ```
  </Step>

  <Step title="Verifikasi tanda tangan">
    Verifikasi tanda tangan terpisah terhadap manifes.

    ```bash theme={null}
    gpg --verify manifest.json.sig manifest.json
    ```

    Hasil yang valid melaporkan `Good signature from "Anthropic Claude Code Release Signing <security@anthropic.com>"`.

    `gpg` juga mencetak `WARNING: This key is not certified with a trusted signature!` untuk kunci yang baru diimpor. Ini diharapkan. Baris `Good signature` mengkonfirmasi pemeriksaan kriptografi lulus. Perbandingan sidik jari di Langkah 1 mengkonfirmasi kunci itu sendiri asli.
  </Step>

  <Step title="Periksa biner terhadap manifes">
    Bandingkan checksum SHA256 biner dengan nilai yang tercantum di bawah `platforms.<platform>.checksum` di `manifest.json`. Perintah di bawah mengasumsikan biner `claude` di direktori saat ini. Untuk memverifikasi biner asli yang terinstal sebagai gantinya, jalankan perintah terhadap `~/.local/share/claude/versions/VERSION`, mengganti VERSION dengan rilis yang Anda atur di Langkah 2.

    <Tabs>
      <Tab title="Linux">
        ```bash theme={null}
        sha256sum claude
        ```
      </Tab>

      <Tab title="macOS">
        ```bash theme={null}
        shasum -a 256 claude
        ```
      </Tab>

      <Tab title="Windows PowerShell">
        ```powershell theme={null}
        (Get-FileHash claude.exe -Algorithm SHA256).Hash.ToLower()
        ```
      </Tab>
    </Tabs>
  </Step>
</Steps>

<Note>
  Tanda tangan manifes tersedia untuk rilis dari `2.1.89` ke depan. Rilis sebelumnya menerbitkan checksum di `manifest.json` tanpa tanda tangan terpisah.
</Note>

<h4 id="platform-code-signatures">
  Tanda tangan kode platform
</h4>

Selain manifes yang ditandatangani, biner individual membawa tanda tangan kode native platform di mana didukung.

* **macOS**: ditandatangani oleh "Anthropic PBC" dan dinotarisi oleh Apple. Verifikasi dengan `codesign --verify --verbose ./claude`.
* **Windows**: ditandatangani oleh "Anthropic, PBC". Verifikasi dengan `Get-AuthenticodeSignature .\claude.exe`.
* **Linux**: biner tidak ditandatangani kode secara individual. Jika Anda mengunduh langsung dari bucket `claude-code-releases` atau menggunakan penginstal asli, verifikasi integritas dengan tanda tangan manifes di atas. Jika Anda menginstal dengan [apt, dnf, atau apk](#install-with-linux-package-managers), manajer paket Anda memverifikasi tanda tangan secara otomatis menggunakan kunci penandatanganan repositori.

<h2 id="uninstall-claude-code">
  Hapus instalasi Claude Code
</h2>

Untuk menghapus Claude Code, ikuti instruksi untuk metode instalasi Anda. Jika `claude` masih berjalan setelahnya, Anda kemungkinan memiliki instalasi kedua atau alias shell yang tertinggal dari installer yang lebih lama. Lihat [Periksa instalasi yang bertentangan](/docs/id/troubleshoot-install#check-for-conflicting-installations) untuk menemukan dan menghapusnya.

<h3 id="native-installation">
  Instalasi asli
</h3>

Hapus biner Claude Code dan file versi:

<Tabs>
  <Tab title="macOS, Linux, WSL">
    ```bash theme={null}
    rm -f ~/.local/bin/claude
    rm -rf ~/.local/share/claude
    ```
  </Tab>

  <Tab title="Windows PowerShell">
    ```powershell theme={null}
    Remove-Item -Path "$env:USERPROFILE\.local\bin\claude.exe" -Force
    Remove-Item -Path "$env:USERPROFILE\.local\share\claude" -Recurse -Force
    ```
  </Tab>
</Tabs>

<h3 id="homebrew-installation">
  Instalasi Homebrew
</h3>

Hapus cask Homebrew yang Anda instal. Jika Anda menginstal cask stabil:

```bash theme={null}
brew uninstall --cask claude-code
```

Jika Anda menginstal cask latest:

```bash theme={null}
brew uninstall --cask claude-code@latest
```

<h3 id="winget-installation">
  Instalasi WinGet
</h3>

Hapus paket WinGet:

```powershell theme={null}
winget uninstall Anthropic.ClaudeCode
```

<h3 id="apt-/-dnf-/-apk">
  apt / dnf / apk
</h3>

Hapus paket dan konfigurasi repositori:

<Tabs>
  <Tab title="apt">
    ```bash theme={null}
    sudo apt remove claude-code
    sudo rm /etc/apt/sources.list.d/claude-code.list /etc/apt/keyrings/claude-code.asc
    ```
  </Tab>

  <Tab title="dnf">
    ```bash theme={null}
    sudo dnf remove claude-code
    sudo rm /etc/yum.repos.d/claude-code.repo
    ```
  </Tab>

  <Tab title="apk">
    ```sh theme={null}
    apk del claude-code
    sed -i '\|downloads.claude.ai/claude-code/apk|d' /etc/apk/repositories
    rm /etc/apk/keys/claude-code.rsa.pub
    ```
  </Tab>
</Tabs>

<h3 id="npm">
  npm
</h3>

Hapus paket npm global:

```bash theme={null}
npm uninstall -g @anthropic-ai/claude-code
```

<h3 id="remove-configuration-files">
  Hapus file konfigurasi
</h3>

<Warning>
  Menghapus file konfigurasi akan menghapus semua pengaturan, alat yang diizinkan, konfigurasi server MCP, dan riwayat sesi Anda.
</Warning>

Ekstensi VS Code, plugin JetBrains, dan Aplikasi Desktop juga menulis ke `~/.claude/`. Jika salah satunya masih terinstal, direktori akan dibuat ulang saat berikutnya dijalankan. Untuk menghapus Claude Code sepenuhnya, copot [ekstensi VS Code](/docs/id/vs-code#uninstall-the-extension), plugin JetBrains, dan Aplikasi Desktop sebelum menghapus file ini.

Untuk menghapus pengaturan Claude Code dan data cache:

<Tabs>
  <Tab title="macOS, Linux, WSL">
    ```bash theme={null}
    # Hapus pengaturan pengguna dan status
    rm -rf ~/.claude
    rm ~/.claude.json

    # Hapus pengaturan khusus proyek (jalankan dari direktori proyek Anda)
    rm -rf .claude
    rm -f .mcp.json
    ```
  </Tab>

  <Tab title="Windows PowerShell">
    ```powershell theme={null}
    # Hapus pengaturan pengguna dan status
    Remove-Item -Path "$env:USERPROFILE\.claude" -Recurse -Force
    Remove-Item -Path "$env:USERPROFILE\.claude.json" -Force

    # Hapus pengaturan khusus proyek (jalankan dari direktori proyek Anda)
    Remove-Item -Path ".claude" -Recurse -Force
    Remove-Item -Path ".mcp.json" -Force
    ```
  </Tab>
</Tabs>
