> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Troubleshoot installation and login

> Perbaiki command not found, PATH, permission, network, dan authentication errors saat menginstal atau masuk ke Claude Code.

Jika instalasi gagal atau Anda tidak dapat masuk, temukan kesalahan Anda di bawah. Untuk masalah runtime setelah Claude Code berfungsi, lihat [Troubleshooting](/docs/id/troubleshooting). Untuk masalah konfigurasi seperti pengaturan tidak diterapkan atau hooks tidak berfungsi, lihat [Debug your configuration](/docs/id/debug-your-config).

<h2 id="find-your-error">
  Temukan kesalahan Anda
</h2>

Cocokkan pesan kesalahan atau gejala yang Anda lihat dengan perbaikan:

| Apa yang Anda lihat                                                                                       | Solusi                                                                                                                                          |
| :-------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------- |
| `command not found: claude` atau `'claude' is not recognized`                                             | [Perbaiki PATH Anda](#command-not-found-claude-after-installation)                                                                              |
| `syntax error near unexpected token '<'`                                                                  | [Install script mengembalikan HTML](#install-script-returns-html-instead-of-a-shell-script)                                                     |
| `curl: (22) The requested URL returned error: 403`                                                        | [Install script mengembalikan 403](#install-script-returns-html-instead-of-a-shell-script)                                                      |
| `curl: (23)` atau `curl: (56) Failure writing output to destination`                                      | [Periksa konektivitas atau gunakan installer alternatif](#curl-56-failure-writing-output-to-destination)                                        |
| `Killed` selama instalasi di Linux, atau `Installation was killed before it could finish (exit code 137)` | [Bebaskan memori atau tambahkan swap space](#install-killed-on-low-memory-linux-servers)                                                        |
| `TLS connect error` atau `SSL/TLS secure channel`                                                         | [Perbarui sertifikat CA](#tls-or-ssl-connection-errors)                                                                                         |
| `Failed to fetch version` atau tidak dapat menjangkau server download                                     | [Periksa pengaturan jaringan dan proxy](#check-network-connectivity)                                                                            |
| `irm is not recognized` atau `&& is not valid`                                                            | [Gunakan perintah yang tepat untuk shell Anda](#wrong-install-command-on-windows)                                                               |
| `Cask 'claude-code' is unavailable: No Cask with this name exists`                                        | [Perbarui Homebrew](#homebrew-cask-unavailable-or-outdated)                                                                                     |
| `'bash' is not recognized as the name of a cmdlet`                                                        | [Gunakan perintah installer Windows](#wrong-install-command-on-windows)                                                                         |
| `A parameter cannot be found that matches parameter name 'fsSL'`                                          | [Gunakan perintah installer Windows](#wrong-install-command-on-windows)                                                                         |
| `Claude Code on Windows requires either Git for Windows (for bash) or PowerShell`                         | [Instal shell](#claude-code-on-windows-requires-either-git-for-windows-for-bash-or-powershell)                                                  |
| `Claude Code does not support 32-bit Windows`                                                             | [Buka Windows PowerShell, bukan entri x86](#claude-code-does-not-support-32-bit-windows)                                                        |
| `The process cannot access the file ... because it is being used by another process`                      | [Kosongkan folder downloads dan coba lagi](#the-process-cannot-access-the-file-during-windows-install)                                          |
| `Error loading shared library`                                                                            | [Binary variant yang salah untuk sistem Anda](#linux-musl-or-glibc-binary-mismatch)                                                             |
| `Illegal instruction`                                                                                     | [Ketidakcocokan arsitektur atau instruction set CPU](#illegal-instruction)                                                                      |
| `cannot execute binary file: Exec format error` di WSL                                                    | [WSL1 native-binary regression](#exec-format-error-on-wsl1)                                                                                     |
| PowerShell installer selesai tetapi `claude` tidak ditemukan atau menunjukkan versi lama                  | [Tambahkan direktori instalasi ke PATH Anda](#verify-your-path), kemudian buka terminal baru                                                    |
| `dyld: cannot load`, `dyld: Symbol not found`, atau `Abort trap` di macOS                                 | [Binary incompatibility](#dyld-cannot-load-on-macos)                                                                                            |
| `Invoke-Expression: Missing argument in parameter list`                                                   | [Install script mengembalikan HTML](#install-script-returns-html-instead-of-a-shell-script)                                                     |
| `App unavailable in region`                                                                               | Claude Code tidak tersedia di negara Anda. Lihat [negara yang didukung](https://www.anthropic.com/supported-countries).                         |
| `unable to get local issuer certificate`                                                                  | [Konfigurasi sertifikat CA perusahaan](#tls-or-ssl-connection-errors)                                                                           |
| `OAuth error` atau `403 Forbidden`                                                                        | [Perbaiki authentication](#login-and-authentication)                                                                                            |
| `Could not load the default credentials` atau `Could not load credentials from any providers`             | [Amazon Bedrock, Google Cloud's Agent Platform, atau Microsoft Foundry credentials](#bedrock-agent-platform-or-foundry-credentials-not-loading) |
| `ChainedTokenCredential authentication failed` atau `CredentialUnavailableError`                          | [Amazon Bedrock, Google Cloud's Agent Platform, atau Microsoft Foundry credentials](#bedrock-agent-platform-or-foundry-credentials-not-loading) |
| `API Error: 500`, `529 Overloaded`, `429`, atau 4xx dan 5xx errors lainnya yang tidak tercantum di atas   | Lihat [Error reference](/docs/id/errors)                                                                                                             |

Jika masalah Anda tidak tercantum, lakukan pemeriksaan diagnostik di bawah untuk mempersempit penyebabnya.

<Tip>
  Jika Anda lebih suka melewati terminal sepenuhnya, [Claude Code Desktop app](/docs/id/desktop-quickstart) memungkinkan Anda menginstal dan menggunakan Claude Code melalui antarmuka grafis. Unduh untuk [macOS](https://claude.ai/api/desktop/darwin/universal/dmg/latest/redirect?utm_source=claude_code\&utm_medium=docs) atau [Windows](https://claude.com/download?utm_source=claude_code\&utm_medium=docs) dan mulai coding tanpa setup command-line apa pun. Di Linux, instal aplikasi dengan apt dengan mengikuti [instruksi instalasi Linux](/docs/id/desktop-linux).
</Tip>

<h2 id="run-diagnostic-checks">
  Jalankan pemeriksaan diagnostik
</h2>

<h3 id="check-network-connectivity">
  Periksa konektivitas jaringan
</h3>

Installer mengunduh dari `downloads.claude.ai`. Verifikasi Anda dapat menjangkaunya:

```bash theme={null}
curl -sI https://downloads.claude.ai/claude-code-releases/latest
```

Di PowerShell, jalankan `curl.exe -sI` sebagai gantinya. PowerShell membuat alias `curl` ke `Invoke-WebRequest`, yang menolak flag `-sI`.

Baris `HTTP/2 200` berarti Anda menjangkau server. Jika Anda tidak melihat output, `Could not resolve host`, atau connection timeout, jaringan Anda memblokir koneksi. Penyebab umum:

* Corporate firewalls atau proxies memblokir `downloads.claude.ai`
* Pembatasan jaringan regional: coba VPN atau jaringan alternatif
* Masalah TLS/SSL: perbarui sertifikat CA sistem Anda, atau periksa apakah `HTTPS_PROXY` dikonfigurasi

Jika Anda berada di belakang corporate proxy, atur `HTTPS_PROXY` dan `HTTP_PROXY` ke alamat proxy Anda sebelum menginstal. Tanyakan tim IT Anda untuk URL proxy jika Anda tidak mengetahuinya, atau periksa pengaturan proxy browser Anda.

Contoh ini mengatur kedua variabel proxy, kemudian menjalankan installer melalui proxy Anda:

<Tabs>
  <Tab title="macOS/Linux">
    ```bash theme={null}
    export HTTP_PROXY=http://proxy.example.com:8080
    export HTTPS_PROXY=http://proxy.example.com:8080
    curl -fsSL https://claude.ai/install.sh | bash
    ```
  </Tab>

  <Tab title="Windows PowerShell">
    ```powershell theme={null}
    $env:HTTP_PROXY = 'http://proxy.example.com:8080'
    $env:HTTPS_PROXY = 'http://proxy.example.com:8080'
    irm https://claude.ai/install.ps1 | iex
    ```
  </Tab>
</Tabs>

<h3 id="verify-your-path">
  Verifikasi PATH Anda
</h3>

Jika instalasi berhasil tetapi Anda mendapatkan error `command not found` atau `not recognized` saat menjalankan `claude`, direktori instalasi tidak ada di PATH Anda. Shell Anda mencari program di direktori yang tercantum di PATH, dan installer menempatkan `claude` di `~/.local/bin/claude` di macOS/Linux atau `%USERPROFILE%\.local\bin\claude.exe` di Windows.

<Note>
  Ekstensi [VS Code](/docs/id/vs-code) tidak menempatkan `claude` di lokasi ini. Ini menggabungkan salinan pribadi CLI di dalam direktori ekstensi untuk panel chat-nya sendiri dan tidak menambahkannya ke PATH. Jika Anda hanya telah menginstal ekstensi, `~/.local/bin/claude` tidak akan ada. Jalankan [instalasi standalone](/docs/id/setup) untuk menggunakan `claude` dari terminal, kemudian lanjutkan di bawah.
</Note>

Periksa apakah direktori instalasi ada di PATH Anda dengan membuat daftar entri PATH dan memfilter untuk `local/bin`:

<Tabs>
  <Tab title="macOS/Linux">
    ```bash theme={null}
    echo $PATH | tr ':' '\n' | grep -Fx "$HOME/.local/bin"
    ```

    Jika ini mencetak `/Users/you/.local/bin` atau `/home/you/.local/bin`, direktori ada di PATH Anda dan Anda dapat melompat ke [Periksa instalasi yang bertentangan](#check-for-conflicting-installations). Jika tidak ada output, tambahkan ke konfigurasi shell Anda.

    Untuk Zsh, default di macOS:

    ```bash theme={null}
    echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.zshrc
    source ~/.zshrc
    ```

    Untuk Bash, default di sebagian besar distribusi Linux:

    ```bash theme={null}
    echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
    source ~/.bashrc
    ```

    Atau, tutup dan buka kembali terminal Anda.

    Untuk shell lain seperti fish atau Nushell, tambahkan `~/.local/bin` ke PATH Anda menggunakan sintaks konfigurasi shell Anda sendiri, kemudian restart terminal Anda.

    Verifikasi perbaikan berhasil:

    ```bash theme={null}
    claude --version
    ```
  </Tab>

  <Tab title="Windows PowerShell">
    ```powershell theme={null}
    $env:PATH -split ';' | Select-String '\.local\\bin'
    ```

    Jika tidak ada output, tambahkan direktori instalasi ke User PATH Anda:

    ```powershell theme={null}
    $currentPath = [Environment]::GetEnvironmentVariable('PATH', 'User')
    [Environment]::SetEnvironmentVariable('PATH', "$currentPath;$env:USERPROFILE\.local\bin", 'User')
    ```

    Restart terminal Anda agar perubahan berlaku.

    Verifikasi perbaikan berhasil:

    ```powershell theme={null}
    claude --version
    ```
  </Tab>

  <Tab title="Windows CMD">
    ```batch theme={null}
    echo %PATH% | findstr /i "local\bin"
    ```

    Jika tidak ada output, buka System Settings, buka Environment Variables, dan tambahkan `%USERPROFILE%\.local\bin` ke variabel User PATH Anda. Restart terminal Anda.

    Verifikasi perbaikan berhasil:

    ```batch theme={null}
    claude --version
    ```
  </Tab>
</Tabs>

<h3 id="check-for-conflicting-installations">
  Periksa instalasi yang bertentangan
</h3>

Beberapa instalasi Claude Code dapat menyebabkan ketidakcocokan versi atau perilaku yang tidak terduga. Periksa apa yang terinstal:

<Tabs>
  <Tab title="macOS/Linux">
    Buat daftar semua binary `claude` yang ditemukan di PATH Anda:

    ```bash theme={null}
    which -a claude
    ```

    Jika ini tidak mencetak apa pun, tidak ada `claude` di PATH Anda. Kembali ke [Verifikasi PATH Anda](#verify-your-path).

    Periksa tiga lokasi tempat binary `claude` dapat berasal. `~/.local/bin/claude` adalah native installer, `~/.claude/local/` adalah legacy local npm install yang dibuat oleh versi Claude Code yang lebih lama, dan npm global list menunjukkan instalasi `-g`:

    ```bash theme={null}
    ls -la ~/.local/bin/claude
    ```

    Native install menunjukkan symlink ke `~/.local/share/claude/versions/`. Script atau symlink yang Anda buat sendiri di path ini adalah custom launcher, yang [auto-update meninggalkan di tempat](/docs/id/setup#auto-updates).

    Jika salah satu perintah `ls` mencetak `No such file or directory`, itu bukan error. Ini berarti tidak ada yang terinstal di lokasi itu, jadi lanjutkan ke pemeriksaan berikutnya.

    ```bash theme={null}
    ls -la ~/.claude/local/
    ```

    ```bash theme={null}
    npm -g ls @anthropic-ai/claude-code 2>/dev/null
    ```
  </Tab>

  <Tab title="Windows PowerShell">
    Buat daftar semua binary `claude` yang ditemukan di PATH Anda:

    ```powershell theme={null}
    where.exe claude
    ```

    Periksa apakah native installer menempatkan binary:

    ```powershell theme={null}
    Test-Path "$env:USERPROFILE\.local\bin\claude.exe"
    ```
  </Tab>
</Tabs>

Jika Anda menemukan beberapa instalasi, pertahankan hanya satu. Native install di `~/.local/bin/claude` di macOS/Linux atau `%USERPROFILE%\.local\bin\claude.exe` di Windows direkomendasikan. Hapus yang lainnya:

Uninstall npm global install:

```bash theme={null}
npm uninstall -g @anthropic-ai/claude-code
```

Hapus legacy local npm install:

```bash theme={null}
rm -rf ~/.claude/local
```

Di Windows, gunakan PowerShell:

```powershell theme={null}
Remove-Item -Recurse -Force "$env:USERPROFILE\.claude\local"
```

Hapus instalasi Homebrew di macOS. Jika Anda menginstal cask `claude-code@latest`, ganti nama itu:

```bash theme={null}
brew uninstall --cask claude-code
```

Hapus instalasi WinGet di Windows:

```powershell theme={null}
winget uninstall Anthropic.ClaudeCode
```

<h3 id="check-directory-permissions">
  Periksa izin direktori
</h3>

Installer memerlukan akses tulis ke `~/.local/bin/` dan `~/.claude/` di macOS dan Linux. Di Windows lokasi instalasi berada di bawah `%USERPROFILE%`, yang dapat ditulis oleh pengguna Anda secara default, jadi bagian ini jarang berlaku di sana.

Periksa apakah direktori dapat ditulis:

```bash theme={null}
test -w ~/.local/bin && echo "writable" || echo "not writable"
test -w ~/.claude && echo "writable" || echo "not writable"
```

Jika direktori mana pun tidak dapat ditulis, buat direktori instalasi dan atur pengguna Anda sebagai pemilik:

```bash theme={null}
sudo mkdir -p ~/.local/bin
sudo chown -R $(whoami) ~/.local
```

<h3 id="verify-the-binary-works">
  Verifikasi binary berfungsi
</h3>

Jika `claude --version` mencetak versi tetapi `claude` crash atau hang pada startup, jalankan pemeriksaan ini untuk mempersempit penyebabnya. Jika `claude --version` mengatakan command not found, buka [Verifikasi PATH Anda](#verify-your-path) terlebih dahulu; perintah di bawah mengasumsikan `claude` ada di PATH Anda.

Konfirmasi binary ada dan dapat dieksekusi:

```bash theme={null}
ls -la "$(command -v claude)"
```

Di Windows, gunakan PowerShell:

```powershell theme={null}
Get-Command claude | Select-Object Source
```

Di Linux, periksa shared libraries yang hilang. Jika `ldd` menunjukkan library yang hilang, Anda mungkin perlu menginstal paket sistem. Di Alpine Linux dan distribusi berbasis musl lainnya, lihat [Alpine Linux setup](/docs/id/setup#alpine-linux-and-musl-based-distributions).

```bash theme={null}
ldd "$(command -v claude)" | grep "not found"
```

Konfirmasi binary dapat dieksekusi:

```bash theme={null}
claude --version
```

<h2 id="common-installation-issues">
  Masalah instalasi umum
</h2>

Ini adalah masalah instalasi yang paling sering dihadapi dan solusinya.

<h3 id="install-script-returns-html-instead-of-a-shell-script">
  Install script returns HTML instead of a shell script
</h3>

Saat menjalankan perintah install, Anda mungkin melihat salah satu error ini:

```text theme={null}
bash: line 1: syntax error near unexpected token `<'
bash: line 1: `<!DOCTYPE html>'
```

Di PowerShell, masalah yang sama muncul sebagai:

```text theme={null}
Invoke-Expression: Missing argument in parameter list.
```

Tergantung pada bagaimana permintaan dirutekan, Anda mungkin malah melihat 403 tanpa body HTML:

```text theme={null}
curl: (22) The requested URL returned error: 403
```

Semuanya berarti URL instalasi mengembalikan halaman HTML atau status error alih-alih script instalasi. Jika halaman HTML mengatakan "App unavailable in region," Claude Code tidak tersedia di negara Anda. Lihat [supported countries](https://www.anthropic.com/supported-countries).

403 tanpa body sering memiliki penyebab yang sama, tetapi juga dapat berasal dari proxy perusahaan atau firewall yang memblokir download. Jika Anda berada di negara yang didukung dan masih melihat 403, kerjakan [Check network connectivity](#check-network-connectivity) sebelum mencoba installer alternatif di bawah, karena installer tersebut menjangkau host yang sama.

Sebaliknya, ini dapat terjadi karena masalah jaringan, routing regional, atau gangguan layanan sementara.

**Solusi:**

1. **Gunakan metode instalasi alternatif**:

   Di macOS, instal melalui Homebrew:

   ```bash theme={null}
   brew install --cask claude-code
   ```

   Di Windows, instal melalui WinGet:

   ```powershell theme={null}
   winget install Anthropic.ClaudeCode
   ```

2. **Coba lagi setelah beberapa menit**: masalahnya sering bersifat sementara. Tunggu dan coba perintah asli lagi.

<h3 id="command-not-found-claude-after-installation">
  `command not found: claude` after installation
</h3>

Instalasi selesai tetapi `claude` tidak berfungsi. Error yang tepat bervariasi menurut platform:

| Platform    | Pesan error                                                            |
| :---------- | :--------------------------------------------------------------------- |
| macOS       | `zsh: command not found: claude`                                       |
| Linux       | `bash: claude: command not found`                                      |
| Windows CMD | `'claude' is not recognized as an internal or external command`        |
| PowerShell  | `claude : The term 'claude' is not recognized as the name of a cmdlet` |

Ini berarti direktori instalasi tidak ada di path pencarian shell Anda. Lihat [Verify your PATH](#verify-your-path) untuk perbaikan di setiap platform.

<h3 id="curl-56-failure-writing-output-to-destination">
  `curl: (56) Failure writing output to destination`
</h3>

Perintah `curl ... | bash` mengunduh script dan menyalurkannya ke Bash untuk dieksekusi. Error ini, dan error terkait `curl: (23) Failure writing output to destination`, berarti Bash tidak menerima script lengkap. Exit code 56 menunjukkan download itu sendiri terputus, dan exit code 23 menunjukkan curl tidak dapat menulis apa yang diterima ke pipe, biasanya karena Bash keluar lebih awal.

**Solusi:**

1. **Periksa stabilitas jaringan**: Binary Claude Code dihosting di `downloads.claude.ai`. Uji bahwa Anda dapat menjangkaunya:
   ```bash theme={null}
   curl -sI https://downloads.claude.ai/claude-code-releases/latest
   ```
   Baris `HTTP/2 200` berarti Anda menjangkau server dan kegagalan asli mungkin bersifat intermiten; coba ulang perintah install. Jika Anda melihat `Could not resolve host` atau connection timeout, jaringan Anda memblokir download.

2. **Coba metode instalasi alternatif**:

   Di macOS:

   ```bash theme={null}
   brew install --cask claude-code
   ```

   Di Windows:

   ```powershell theme={null}
   winget install Anthropic.ClaudeCode
   ```

<h3 id="homebrew-cask-unavailable-or-outdated">
  Homebrew cask unavailable or outdated
</h3>

Homebrew melaporkan `Error: Cask 'claude-code' is unavailable: No Cask with this name exists` ketika salinan lokal indeks cask Homebrew Anda mendahului publikasi cask. Segarkan indeks dan coba ulang:

```bash theme={null}
brew update
brew install --cask claude-code
```

Jika Homebrew menginstal versi Claude Code yang lebih lama dari yang Anda harapkan, indeks yang sudah usang biasanya menjadi penyebabnya. Cask `claude-code` melacak saluran stabil dan biasanya tertinggal sekitar satu minggu dari rilis terbaru; untuk versi terbaru jalankan `brew install --cask claude-code@latest` sebagai gantinya. Lihat [Configure release channel](/docs/id/setup#configure-release-channel) untuk perbedaan antara dua cask.

<h3 id="tls-or-ssl-connection-errors">
  TLS or SSL connection errors
</h3>

Error seperti `curl: (35) TLS connect error`, `schannel: next InitializeSecurityContext failed`, atau PowerShell's `Could not establish trust relationship for the SSL/TLS secure channel` menunjukkan kegagalan TLS handshake.

**Solusi:**

1. **Perbarui sertifikat CA sistem Anda**:

   Di Ubuntu/Debian:

   ```bash theme={null}
   sudo apt-get update && sudo apt-get install ca-certificates
   ```

   Di macOS, curl sistem menggunakan Keychain trust store; memperbarui macOS itu sendiri memperbarui root certificates.

2. **Di Windows, aktifkan TLS 1.2** di PowerShell sebelum menjalankan installer:
   ```powershell theme={null}
   [Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12
   irm https://claude.ai/install.ps1 | iex
   ```

3. **Periksa gangguan proxy atau firewall**: corporate proxies yang melakukan TLS inspection dapat menyebabkan error ini, termasuk `unable to get local issuer certificate` dan `SELF_SIGNED_CERT_IN_CHAIN`. Untuk langkah instalasi, arahkan curl ke bundle CA perusahaan Anda dengan `--cacert`:
   ```bash theme={null}
   curl --cacert /path/to/corporate-ca.pem -fsSL https://claude.ai/install.sh | bash
   ```
   Untuk Claude Code itu sendiri setelah diinstal, atur `NODE_EXTRA_CA_CERTS` sehingga permintaan API mempercayai bundle yang sama:
   ```bash theme={null}
   export NODE_EXTRA_CA_CERTS=/path/to/corporate-ca.pem
   ```
   Tanyakan tim IT Anda untuk file sertifikat jika Anda tidak memilikinya. Anda juga dapat mencoba koneksi langsung untuk mengkonfirmasi proxy adalah penyebabnya.

4. **Di Windows, lewati pemeriksaan revokasi sertifikat** jika Anda melihat `CRYPT_E_NO_REVOCATION_CHECK (0x80092012)` atau `CRYPT_E_REVOCATION_OFFLINE (0x80092013)`. Ini berarti curl menjangkau server tetapi jaringan Anda memblokir pencarian revokasi sertifikat, yang umum di belakang firewall perusahaan. Menambahkan flag `--ssl-revoke-best-effort` curl tidak memperbaiki ini: flag hanya berlaku untuk mengunduh `install.cmd` itu sendiri, dan download script itu sendiri berjalan tanpanya, jadi instalasi gagal dengan error yang sama. Gunakan metode instalasi yang mentoleransi pencarian yang diblokir sebagai gantinya. Buka PowerShell dan jalankan PowerShell installer, yang mengunduh melalui .NET dan tidak gagal ketika server revokasi tidak dapat dijangkau:
   ```powershell theme={null}
   irm https://claude.ai/install.ps1 | iex
   ```
   Anda juga dapat menginstal dengan `winget install Anthropic.ClaudeCode`, yang menghindari curl sepenuhnya.

<h3 id="failed-to-fetch-version-from-downloads-claude-ai">
  `Failed to fetch version from downloads.claude.ai`
</h3>

Installer tidak dapat menjangkau server download. Ini biasanya berarti `downloads.claude.ai` diblokir di jaringan Anda.

**Solusi:**

1. **Uji konektivitas secara langsung**:
   ```bash theme={null}
   curl -sI https://downloads.claude.ai/claude-code-releases/latest
   ```

2. **Jika di belakang proxy**, atur `HTTPS_PROXY` sehingga installer dapat merutekan melaluinya. Lihat [proxy configuration](/docs/id/network-config#proxy-configuration) untuk detail.
   ```bash theme={null}
   export HTTPS_PROXY=http://proxy.example.com:8080
   curl -fsSL https://claude.ai/install.sh | bash
   ```

3. **Jika di jaringan terbatas**, coba jaringan berbeda atau VPN, atau gunakan metode instalasi alternatif:

   Di macOS:

   ```bash theme={null}
   brew install --cask claude-code
   ```

   Di Windows:

   ```powershell theme={null}
   winget install Anthropic.ClaudeCode
   ```

<h3 id="wrong-install-command-on-windows">
  Wrong install command on Windows
</h3>

Jika Anda melihat `'irm' is not recognized`, `The token '&&' is not valid`, `A parameter cannot be found that matches parameter name 'fsSL'`, atau `'bash' is not recognized as the name of a cmdlet`, Anda menyalin perintah install untuk shell atau sistem operasi yang berbeda.

* **`irm` not recognized**: Anda berada di CMD, bukan PowerShell. Anda memiliki dua opsi:

  Buka PowerShell dengan mencari "PowerShell" di Start menu, kemudian jalankan perintah install asli:

  ```powershell theme={null}
  irm https://claude.ai/install.ps1 | iex
  ```

  Atau tetap di CMD dan gunakan CMD installer sebagai gantinya:

  ```batch theme={null}
  curl -fsSL https://claude.ai/install.cmd -o install.cmd && install.cmd && del install.cmd
  ```

* **`&&` not valid**: Anda berada di PowerShell tetapi menjalankan perintah CMD installer. Gunakan PowerShell installer:
  ```powershell theme={null}
  irm https://claude.ai/install.ps1 | iex
  ```

* **`A parameter cannot be found that matches parameter name 'fsSL'`**: Anda menjalankan installer macOS/Linux `curl -fsSL ... | bash` di Windows PowerShell, di mana `curl` adalah alias untuk `Invoke-WebRequest` dan menolak flag `-fsSL`. Gunakan PowerShell installer sebagai gantinya:
  ```powershell theme={null}
  irm https://claude.ai/install.ps1 | iex
  ```

* **`bash` not recognized**: Anda menjalankan installer macOS/Linux di Windows. Gunakan PowerShell installer sebagai gantinya:
  ```powershell theme={null}
  irm https://claude.ai/install.ps1 | iex
  ```

<h3 id="the-process-cannot-access-the-file-during-windows-install">
  `The process cannot access the file` during Windows install
</h3>

Jika PowerShell installer gagal dengan `Failed to download binary: The process cannot access the file ... because it is being used by another process`, installer tidak dapat menulis ke `%USERPROFILE%\.claude\downloads`. Ini biasanya berarti upaya install sebelumnya masih berjalan, atau software antivirus memindai binary yang sebagian diunduh di folder itu.

Tutup jendela PowerShell lain yang menjalankan installer dan tunggu pemindaian antivirus melepaskan file. Kemudian hapus folder downloads dan jalankan installer lagi:

```powershell theme={null}
Remove-Item -Recurse -Force "$env:USERPROFILE\.claude\downloads"
irm https://claude.ai/install.ps1 | iex
```

<h3 id="install-killed-on-low-memory-linux-servers">
  Install killed on low-memory Linux servers
</h3>

Pesan `Killed` selama install biasanya berarti Linux out-of-memory (OOM) killer menghentikan langkah `claude install` karena sistem kehabisan memori gratis. Ini umum di VPS dan cloud instances kecil. Script install melaporkan penyebabnya dan keluar dengan kode 137:

```text theme={null}
Setting up Claude Code...
bash: line 142: 34803 Killed    "$binary_path" install ${TARGET:+"$TARGET"}
Installation was killed before it could finish (exit code 137). This usually means the system ran out of memory.
Claude Code needs roughly 512MB of free memory to install. Free up memory, then run this script again.
```

Sebelum v2.1.200, script keluar hanya dengan baris `Killed` telanjang shell dan tanpa penjelasan.

Instalasi memerlukan kira-kira 512 MB memori gratis, dan menjalankan Claude Code memerlukan lebih banyak. Lihat [system requirements](/docs/id/setup#system-requirements).

**Solusi:**

1. **Tambahkan swap space** jika server Anda memiliki RAM terbatas. Swap menggunakan ruang disk sebagai memori overflow, memungkinkan instalasi selesai bahkan dengan RAM fisik rendah.

   Buat file swap 2 GB dan aktifkan:

   ```bash theme={null}
   sudo fallocate -l 2G /swapfile
   sudo chmod 600 /swapfile
   sudo mkswap /swapfile
   sudo swapon /swapfile
   ```

   Kemudian coba ulang instalasi:

   ```bash theme={null}
   curl -fsSL https://claude.ai/install.sh | bash
   ```

2. **Tutup proses lain** untuk membebaskan memori sebelum menginstal.

3. **Gunakan instance yang lebih besar** jika memungkinkan. Claude Code memerlukan setidaknya 4 GB RAM.

<h3 id="install-hangs-in-docker">
  Install hangs in Docker
</h3>

Saat menginstal Claude Code di Docker container, menginstal sebagai root ke `/` dapat menyebabkan hang.

**Solusi:**

1. **Atur working directory** sebelum menjalankan installer. Saat dijalankan dari `/`, installer memindai seluruh filesystem, yang menyebabkan penggunaan memori berlebihan. Mengatur `WORKDIR` membatasi pemindaian ke direktori kecil:
   ```dockerfile theme={null}
   WORKDIR /tmp
   RUN curl -fsSL https://claude.ai/install.sh | bash
   ```

2. **Tingkatkan batas memori Docker** jika menggunakan Docker Desktop:
   ```bash theme={null}
   docker build --memory=4g .
   ```

<h3 id="claude-desktop-overrides-the-claude-command-on-windows">
  Claude Desktop overrides the `claude` command on Windows
</h3>

Jika Anda menginstal versi Claude Desktop yang lebih lama, mungkin mendaftarkan `Claude.exe` di direktori `WindowsApps` yang mengambil prioritas PATH di atas Claude Code CLI. Menjalankan `claude` membuka Desktop app alih-alih CLI.

Perbarui Claude Desktop ke versi terbaru untuk memperbaiki masalah ini.

<h3 id="claude-code-on-windows-requires-either-git-for-windows-for-bash-or-powershell">
  Claude Code on Windows requires either Git for Windows (for bash) or PowerShell
</h3>

Git for Windows bersifat opsional. Claude Code menggunakan [PowerShell tool](/docs/id/tools-reference#powershell-tool) saat Git Bash tidak ada, jadi error ini berarti tidak ada shell yang ditemukan.

**Jika PowerShell hilang dari PATH Anda**, lokasi defaultnya adalah `C:\Windows\System32\WindowsPowerShell\v1.0\`. Tambahkan direktori itu ke `PATH` Anda, atau instal [PowerShell 7](https://aka.ms/powershell), yang menyediakan `pwsh`.

**Untuk menginstal Git for Windows sebagai gantinya**, unduh dari [git-scm.com/downloads/win](https://git-scm.com/downloads/win). Selama setup, pilih "Add to PATH." Restart terminal Anda setelah menginstal. Menginstalnya mengaktifkan Bash tool, berguna saat bekerja dengan script dan tooling berbasis Bash.

**Jika Git sudah terinstal** tetapi Claude Code tidak dapat menemukannya, atur path di [settings.json file](/docs/id/settings) Anda:

```json theme={null}
{
  "env": {
    "CLAUDE_CODE_GIT_BASH_PATH": "C:\\Program Files\\Git\\bin\\bash.exe"
  }
}
```

Jika Git Anda diinstal di tempat lain, temukan path dengan menjalankan `where.exe git` di PowerShell dan gunakan path `bin\bash.exe` dari direktori itu.

**Jika path sudah benar dan file ada** tetapi Claude Code masih melaporkannya tidak ditemukan, software endpoint security seperti AppLocker, Group Policy software restriction policies, atau EDR agents mungkin mengganggu. Pada versi sebelum v2.1.116, Claude Code menjalankan child process (`cmd.exe`) untuk memverifikasi path, yang policies ini dapat blokir — sinyal umum adalah bahwa `cmd.exe /c dir "C:\Program Files\Git\bin\bash.exe"` berfungsi saat Anda menjalankannya langsung di PowerShell tetapi gagal diam-diam saat diluncurkan oleh `claude.exe`.

Claude Code v2.1.116 dan lebih baru memeriksa filesystem secara langsung, jadi perbarui terlebih dahulu. Jika error berlanjut pada versi saat ini, minta tim IT Anda untuk allowlist `claude.exe` dan proses yang dijalankannya, termasuk `cmd.exe` dan `bash.exe`, dalam kebijakan endpoint protection Anda.

<h3 id="claude-code-does-not-support-32-bit-windows">
  Claude Code does not support 32-bit Windows
</h3>

Windows menyertakan dua entri PowerShell di Start menu: `Windows PowerShell` dan `Windows PowerShell (x86)`. Entri x86 berjalan sebagai proses 32-bit dan memicu error ini bahkan di mesin 64-bit. Untuk memeriksa kasus mana yang Anda alami, jalankan ini di jendela yang sama yang menghasilkan error:

```powershell theme={null}
[Environment]::Is64BitOperatingSystem
```

Jika ini mencetak `True`, sistem operasi Anda baik-baik saja. Tutup jendela, buka `Windows PowerShell` tanpa suffix x86, dan jalankan perintah install lagi.

Jika ini mencetak `False`, Anda berada di edisi Windows 32-bit. Claude Code memerlukan sistem operasi 64-bit. Lihat [system requirements](/docs/id/setup#system-requirements).

<h3 id="linux-musl-or-glibc-binary-mismatch">
  Linux musl or glibc binary mismatch
</h3>

Jika Anda melihat error tentang shared libraries yang hilang seperti `libstdc++.so.6` atau `libgcc_s.so.1` setelah instalasi, installer mungkin telah mengunduh binary variant yang salah untuk sistem Anda.

```text theme={null}
Error loading shared library libstdc++.so.6: No such file or directory
```

Ini dapat terjadi pada sistem berbasis glibc yang memiliki paket cross-compilation musl terinstal, menyebabkan installer salah mendeteksi sistem sebagai musl.

**Solusi:**

1. **Periksa libc mana yang digunakan sistem Anda**:
   ```bash theme={null}
   ldd --version 2>&1 | head -1
   ```
   Output yang menyebutkan `GNU libc` atau `GLIBC` berarti glibc. Output yang menyebutkan `musl` berarti musl.

2. **Jika Anda berada di glibc tetapi mendapat binary musl**, hapus instalasi dan instal ulang. Anda juga dapat secara manual mengunduh binary yang benar menggunakan manifest di `https://downloads.claude.ai/claude-code-releases/{VERSION}/manifest.json`. File [GitHub issue](https://github.com/anthropics/claude-code/issues) dengan output `ldd --version` dan `ls /lib/libc.musl*`.

3. **Jika Anda benar-benar di musl**, seperti Alpine Linux, instal paket yang diperlukan:
   ```bash theme={null}
   apk add libgcc libstdc++ ripgrep
   ```

<h3 id="illegal-instruction">
  `Illegal instruction`
</h3>

Jika menjalankan `claude` atau installer mencetak `Illegal instruction`, binary native menggunakan CPU instructions yang processor Anda tidak dukung. Ada dua penyebab yang berbeda.

**Architecture mismatch.** Installer mengunduh binary yang salah, misalnya x86 di server ARM. Periksa dengan `uname -m` di macOS atau Linux, atau `$env:PROCESSOR_ARCHITECTURE` di PowerShell. Jika hasilnya tidak cocok dengan binary yang Anda terima, [file GitHub issue](https://github.com/anthropics/claude-code/issues) dengan output.

**Missing AVX instruction set.** Jika arsitektur Anda benar tetapi Anda masih melihat `Illegal instruction`, CPU Anda mungkin tidak memiliki AVX atau instruction lain yang binary perlukan. Ini mempengaruhi kira-kira processor Intel dan AMD pre-2013, dan virtual machines di mana hypervisor tidak melewatkan AVX ke guest.

Di VPS atau VM, jalankan `grep -m1 -ow avx /proc/cpuinfo`; hasil kosong berarti AVX tidak tersedia untuk guest.

Tidak ada native-binary workaround; track [issue #50384](https://github.com/anthropics/claude-code/issues/50384) untuk status, dan sertakan model CPU Anda dari `grep -m1 "model name" /proc/cpuinfo` di Linux atau `sysctl -n machdep.cpu.brand_string` di macOS saat melaporkan.

Metode instalasi alternatif mengunduh binary native yang sama dan tidak akan menyelesaikan penyebab apa pun.

<h3 id="dyld-cannot-load-on-macos">
  `dyld: cannot load` on macOS
</h3>

Jika Anda melihat `dyld: cannot load`, `dyld: Symbol not found`, atau `Abort trap: 6` selama instalasi, binary tidak kompatibel dengan versi macOS atau hardware Anda.

```text theme={null}
dyld: cannot load 'claude-2.1.42-darwin-x64' (load command 0x80000034 is unknown)
Abort trap: 6
```

Error `Symbol not found` yang mereferensikan `libicucore` juga menunjukkan versi macOS Anda lebih lama dari yang binary dukung:

```text theme={null}
dyld: Symbol not found: _ubrk_clone
  Referenced from: claude-darwin-x64 (which was built for Mac OS X 13.0)
  Expected in: /usr/lib/libicucore.A.dylib
```

**Solusi:**

1. **Periksa versi macOS Anda**: Claude Code memerlukan macOS 13.0 atau lebih baru. Buka menu Apple dan pilih About This Mac untuk memeriksa versi Anda.

2. **Perbarui macOS** jika Anda berada di versi yang lebih lama. Binary menggunakan load commands dan system libraries yang versi macOS yang lebih lama tidak dukung. Metode instalasi alternatif seperti Homebrew mengunduh binary yang sama dan tidak akan menyelesaikan error ini.

<h3 id="exec-format-error-on-wsl1">
  `Exec format error` on WSL1
</h3>

Jika menjalankan `claude` di WSL mencetak `cannot execute binary file: Exec format error`, Anda berada di WSL1 dan mengalami native-binary regression yang dikenal yang dilacak di [issue #38788](https://github.com/anthropics/claude-code/issues/38788). Program headers binary berubah dengan cara yang WSL1's loader tidak dapat menangani.

Perbaikan paling bersih adalah mengonversi distribusi Anda ke WSL2 dari PowerShell:

```powershell theme={null}
wsl --set-version <DistroName> 2
```

Jika Anda perlu tetap di WSL1, panggil binary melalui dynamic linker. Tambahkan fungsi ini ke `~/.bashrc` di dalam WSL, ganti path jika direktori home Anda berbeda:

```bash theme={null}
claude() {
  /lib64/ld-linux-x86-64.so.2 "$(readlink -f "$HOME/.local/bin/claude")" "$@"
}
```

Kemudian jalankan `source ~/.bashrc` dan coba ulang `claude`.

<h3 id="npm-install-errors-in-wsl">
  npm install errors in WSL
</h3>

Masalah ini berlaku jika Anda menginstal Claude Code dengan `npm install -g` di dalam WSL. Jika Anda menggunakan [native installer](/docs/id/setup), lewati bagian ini.

**OS atau platform detection issues.** Jika npm melaporkan ketidakcocokan platform selama instalasi, WSL mungkin mengambil Windows `npm`. Jalankan `npm config set os linux` terlebih dahulu, kemudian instal dengan `npm install -g @anthropic-ai/claude-code --force`. Jangan gunakan `sudo`.

**`exec: node: not found` saat menjalankan `claude`.** Lingkungan WSL Anda mungkin menggunakan instalasi Windows Node.js. Konfirmasi dengan `which npm` dan `which node`: path yang dimulai dengan `/mnt/c/` adalah binary Windows, sementara path Linux dimulai dengan `/usr/`. Untuk memperbaiki ini, instal Node melalui package manager distribusi Linux Anda atau melalui [`nvm`](https://github.com/nvm-sh/nvm).

**nvm version conflicts.** Jika Anda memiliki nvm terinstal di WSL dan Windows, beralih versi Node di WSL mungkin rusak karena WSL mengimpor Windows PATH secara default dan Windows nvm mengambil prioritas. Penyebab paling umum adalah nvm tidak dimuat di shell Anda. Tambahkan nvm loader ke `~/.bashrc` atau `~/.zshrc`:

```bash theme={null}
export NVM_DIR="$HOME/.nvm"
[ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"
[ -s "$NVM_DIR/bash_completion" ] && \. "$NVM_DIR/bash_completion"
```

Atau muat di sesi saat ini:

```bash theme={null}
source ~/.nvm/nvm.sh
```

Jika nvm dimuat tetapi path Windows masih mengambil prioritas, tambahkan path Node Linux Anda secara eksplisit:

```bash theme={null}
export PATH="$HOME/.nvm/versions/node/$(node -v)/bin:$PATH"
```

<Warning>
  Hindari menonaktifkan Windows PATH importing melalui `appendWindowsPath = false` karena ini merusak kemampuan untuk memanggil Windows executables dari WSL. Demikian pula, hindari menguninstall Node.js dari Windows jika Anda menggunakannya untuk pengembangan Windows.
</Warning>

<h3 id="permission-errors-during-installation">
  Permission errors during installation
</h3>

Jika native installer gagal dengan permission errors, direktori target mungkin tidak dapat ditulis. Lihat [Check directory permissions](#check-directory-permissions).

Jika Anda sebelumnya menginstal dengan npm dan mengalami npm-specific permission errors, beralih ke native installer:

```bash theme={null}
curl -fsSL https://claude.ai/install.sh | bash
```

<h3 id="native-binary-not-found-after-npm-install">
  Native binary not found after npm install
</h3>

Paket npm `@anthropic-ai/claude-code` menarik binary native melalui per-platform optional dependency seperti `@anthropic-ai/claude-code-darwin-arm64`. Jika menjalankan `claude` setelah install mencetak `Could not find native binary package "@anthropic-ai/claude-code-<platform>"`, periksa penyebab berikut:

* **Optional dependencies dinonaktifkan.** Hapus `--omit=optional` dari perintah npm install Anda, `--no-optional` dari pnpm, atau `--ignore-optional` dari yarn, dan periksa bahwa `.npmrc` tidak mengatur `optional=false`. Kemudian instal ulang. Binary native disampaikan hanya sebagai optional dependency, jadi tidak ada JavaScript fallback jika dilewati.
* **Platform tidak didukung.** Binary prebuilt dipublikasikan untuk `darwin-arm64`, `darwin-x64`, `linux-x64`, `linux-arm64`, `linux-x64-musl`, `linux-arm64-musl`, `win32-x64`, dan `win32-arm64`. Claude Code tidak mengirimkan binary untuk platform lain; lihat [system requirements](/docs/id/setup#system-requirements). Di FreeBSD, installer melaporkan platform sebagai tidak didukung. Sebelum v2.1.205, installer memperlakukan FreeBSD sebagai Linux dan mengunduh binary yang tidak dapat dijalankan.
* **Corporate npm mirror kehilangan paket platform.** Pastikan registry Anda mencerminkan semua delapan paket `@anthropic-ai/claude-code-*` platform selain paket meta.

Menginstal dengan `--ignore-scripts` tidak memicu error ini. Langkah postinstall yang menghubungkan binary ke tempat dilewati, jadi Claude Code kembali ke wrapper yang menemukan dan menjalankan binary platform di setiap peluncuran. Ini berfungsi tetapi dimulai lebih lambat; instal ulang dengan scripts diaktifkan untuk eksekusi langsung.

<h2 id="login-and-authentication">
  Login and authentication
</h2>

Bagian ini mengatasi kegagalan login, OAuth errors, dan masalah token.

<h3 id="reset-your-login">
  Reset your login
</h3>

Saat login gagal dan penyebabnya tidak jelas, re-authentication yang bersih menyelesaikan sebagian besar kasus:

1. Jalankan `/logout` untuk sign out sepenuhnya
2. Tutup Claude Code
3. Restart dengan `claude` dan selesaikan proses authentication lagi

Jika browser tidak terbuka secara otomatis selama login, tekan `c` untuk menyalin OAuth URL ke clipboard Anda, kemudian tempel ke browser secara manual. Ini juga berfungsi saat URL membungkus di seluruh baris di terminal sempit atau SSH dan tidak dapat diklik langsung.

<h3 id="oauth-error-invalid-code">
  OAuth error: Invalid code
</h3>

Jika Anda melihat `OAuth error: Invalid code. Please make sure the full code was copied`, kode login kedaluwarsa atau terpotong selama copy-paste.

**Solusi:**

* Tekan Enter untuk coba ulang dan selesaikan login dengan cepat setelah browser terbuka
* Ketik `c` untuk menyalin URL lengkap jika browser tidak terbuka secara otomatis
* Jika menggunakan sesi remote/SSH, browser mungkin terbuka di mesin yang salah. Salin URL yang ditampilkan di terminal dan buka di browser lokal Anda sebagai gantinya.

<h3 id="403-forbidden-after-login">
  403 Forbidden after login
</h3>

Jika Anda melihat `API Error: 403 {"error":{"type":"forbidden","message":"Request not allowed"}}` setelah login:

* **Claude Pro/Max users**: verifikasi subscription Anda aktif di [claude.ai/settings](https://claude.ai/settings)
* **Anthropic Console users**: konfirmasi akun Anda memiliki role "Claude Code" atau "Developer". Admins menetapkan ini di Anthropic Console di bawah Settings → Members.
* **Di belakang proxy**: corporate proxies dapat mengganggu permintaan API. Lihat [network configuration](/docs/id/network-config) untuk setup proxy.

<h3 id="this-organization-has-been-disabled-with-an-active-subscription">
  This organization has been disabled with an active subscription
</h3>

Jika Anda melihat `API Error: 400 ... "This organization has been disabled"` meskipun memiliki subscription Claude aktif, variabel environment `ANTHROPIC_API_KEY` menimpa subscription Anda. Ini biasanya terjadi saat API key lama dari employer atau project sebelumnya masih diatur di shell profile Anda.

Saat `ANTHROPIC_API_KEY` ada dan Anda telah menyetujuinya, Claude Code menggunakan key itu alih-alih OAuth credentials subscription Anda. Dalam mode non-interactive dengan flag `-p`, key selalu digunakan saat ada. Lihat [authentication precedence](/docs/id/authentication#authentication-precedence) untuk urutan resolusi lengkap.

Untuk menggunakan subscription Anda sebagai gantinya, unset variabel environment dan hapus dari shell profile Anda:

```bash theme={null}
unset ANTHROPIC_API_KEY
claude
```

Periksa `~/.zshrc`, `~/.bashrc`, atau `~/.profile` untuk baris `export ANTHROPIC_API_KEY=...` dan hapus untuk membuat perubahan permanen. Di Windows, periksa PowerShell profile Anda di `$PROFILE` dan User environment variables Anda untuk `ANTHROPIC_API_KEY`. Jalankan `/status` di dalam Claude Code untuk mengkonfirmasi metode authentication mana yang aktif.

<h3 id="oauth-login-fails-in-wsl2-ssh-or-containers">
  OAuth login fails in WSL2, SSH, or containers
</h3>

Saat Claude Code berjalan di WSL2, pada mesin remote melalui SSH, atau di dalam container, browser biasanya terbuka di host yang berbeda dan redirectnya tidak dapat menjangkau server callback lokal Claude Code. Setelah Anda sign in, browser menampilkan kode login alih-alih redirect kembali secara otomatis. Tempel kode itu ke terminal di prompt `Paste code here if prompted` untuk menyelesaikan login.

Jika browser tidak terbuka sama sekali dari WSL2, atur variabel environment `BROWSER` ke path Windows browser Anda:

```bash theme={null}
export BROWSER="/mnt/c/Program Files/Google/Chrome/Application/chrome.exe"
claude
```

Atau, tekan `c` di interactive login prompt untuk menyalin OAuth URL, atau salin URL yang `claude auth login` cetak, dan buka di browser di mesin lokal Anda.

Jika menempel kode ke interactive prompt tidak melakukan apa pun, binding paste terminal Anda mungkin tidak menjangkau input field. Coba shortcut paste alternatif terminal Anda, sering kali right-click atau Shift+Insert di Windows Terminal, atau gunakan `claude auth login` sebagai gantinya, yang membaca kode yang ditempel dari standard input:

```bash theme={null}
claude auth login
```

Fallback ini juga berlaku di Windows native atau terminal apa pun di mana menempel ke interactive prompt gagal.

<h3 id="not-logged-in-or-token-expired">
  Not logged in or token expired
</h3>

Jika Claude Code meminta Anda untuk login lagi setelah sesi, OAuth token Anda mungkin telah kedaluwarsa.

Jalankan `/login` untuk re-authenticate. Jika ini terjadi sering, periksa bahwa jam sistem Anda akurat, karena validasi token bergantung pada timestamp yang benar.

Di macOS, login juga dapat gagal saat Keychain terkunci atau passwordnya tidak sinkron dengan password akun Anda, yang mencegah Claude Code menyimpan credentials. Jalankan `claude doctor` untuk memeriksa akses Keychain. Untuk membuka Keychain secara manual, jalankan `security unlock-keychain ~/Library/Keychains/login.keychain-db`. Jika membuka tidak membantu, buka Keychain Access, pilih keychain `login`, dan pilih Edit > Change Password for Keychain "login" untuk menyinkronkannya kembali dengan password akun Anda.

<h3 id="bedrock-agent-platform-or-foundry-credentials-not-loading">
  Bedrock, Agent Platform, or Foundry credentials not loading
</h3>

Jika Anda mengkonfigurasi Claude Code untuk menggunakan cloud provider dan melihat `Could not load credentials from any providers` di Amazon Bedrock, `Could not load the default credentials` di Google Cloud's Agent Platform, atau `ChainedTokenCredential authentication failed` di Microsoft Foundry, cloud provider CLI Anda mungkin tidak authenticated di shell saat ini.

Untuk Amazon Bedrock, konfirmasi AWS credentials Anda valid:

```bash theme={null}
aws sts get-caller-identity
```

Untuk Google Cloud's Agent Platform, konfirmasi `ANTHROPIC_VERTEX_PROJECT_ID` dan `CLOUD_ML_REGION` diatur di shell Anda, kemudian atur application default credentials:

```bash theme={null}
gcloud auth application-default login
```

Untuk Microsoft Foundry, konfirmasi `ANTHROPIC_FOUNDRY_API_KEY` diatur, atau sign in dengan Azure CLI sehingga default credential chain dapat menemukan akun Anda:

```bash theme={null}
az login
```

Jika credentials berfungsi di terminal Anda tetapi tidak di VS Code atau JetBrains extension, proses IDE mungkin tidak mewarisi environment shell Anda. Atur variabel environment provider di pengaturan IDE itu sendiri, atau luncurkan IDE dari terminal di mana mereka sudah diekspor.

Lihat [Amazon Bedrock](/docs/id/amazon-bedrock), [Google Cloud's Agent Platform](/docs/id/google-vertex-ai), atau [Microsoft Foundry](/docs/id/microsoft-foundry) untuk setup provider lengkap.

<h2 id="still-stuck">
  Still stuck
</h2>

Jika tidak ada di atas yang menyelesaikan masalah Anda:

1. Periksa [GitHub repository](https://github.com/anthropics/claude-code/issues) untuk known issues, atau buka yang baru dengan sistem operasi Anda, perintah install yang Anda jalankan, dan output error lengkap
2. Jika `claude --version` berfungsi tetapi sesuatu yang lain salah, jalankan `claude doctor` untuk laporan diagnostik otomatis
3. Jika Anda dapat memulai sesi, gunakan `/feedback` di dalam Claude Code untuk melaporkan masalah
