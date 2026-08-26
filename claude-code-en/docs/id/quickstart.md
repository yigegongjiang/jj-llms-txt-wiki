> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Panduan Cepat

> Selamat datang di Claude Code!

Panduan cepat ini akan membuat Anda menggunakan bantuan pengkodean bertenaga AI dalam beberapa menit. Di akhir panduan, Anda akan memahami cara menggunakan Claude Code untuk tugas-tugas pengembangan umum.

<h2 id="before-you-begin">
  Sebelum Anda memulai
</h2>

Pastikan Anda memiliki:

* Terminal atau command prompt yang terbuka
  * Jika Anda belum pernah menggunakan terminal sebelumnya, lihat [panduan terminal](/docs/id/terminal-guide)
* Proyek kode untuk dikerjakan
* [Langganan Claude](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=quickstart_prereq) (Pro, Max, Team, atau Enterprise), akun [Claude Console](https://console.anthropic.com/), atau akses melalui [penyedia cloud yang didukung](/docs/id/third-party-integrations)

<Note>
  Panduan ini mencakup CLI terminal. Claude Code juga tersedia di [web](https://claude.ai/code), sebagai [aplikasi desktop](/docs/id/desktop), di [VS Code](/docs/id/vs-code) dan [IDE JetBrains](/docs/id/jetbrains), di [Slack](/docs/id/slack), dan di CI/CD dengan [GitHub Actions](/docs/id/github-actions) dan [GitLab](/docs/id/gitlab-ci-cd). Lihat [semua antarmuka](/docs/id/overview#use-claude-code-everywhere).
</Note>

<h2 id="step-1-install-claude-code">
  Langkah 1: Instal Claude Code
</h2>

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

<h2 id="step-2-log-in-to-your-account">
  Langkah 2: Masuk ke akun Anda
</h2>

Claude Code memerlukan akun untuk digunakan. Mulai sesi interaktif dengan perintah `claude` dan Anda akan diminta untuk masuk pada penggunaan pertama:

```bash theme={null}
claude
```

Untuk akun langganan Claude atau Console, ikuti petunjuk untuk menyelesaikan autentikasi di browser Anda. Untuk beralih akun nanti atau melakukan autentikasi ulang, ketik `/login` di dalam sesi yang sedang berjalan:

```text theme={null}
/login
```

Anda dapat masuk menggunakan salah satu jenis akun ini:

* [Claude Pro, Max, Team, atau Enterprise](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=quickstart_login) (direkomendasikan)
* [Claude Console](https://console.anthropic.com/) (akses API dengan kredit prabayar). Pada login pertama, ruang kerja "Claude Code" secara otomatis dibuat di Console untuk pelacakan biaya terpusat.
* [Amazon Bedrock, Google Cloud's Agent Platform, atau Microsoft Foundry](/docs/id/third-party-integrations) (penyedia cloud enterprise)
* Gateway [Claude apps](/docs/id/claude-apps-gateway) yang di-host sendiri, jika organisasi Anda menjalankannya: admin Anda telah mengkonfigurasi URL gateway sebelumnya, dan `/login` membuka langsung layar **Cloud gateway** untuk Anda masuk dengan SSO perusahaan

Setelah masuk, kredensial Anda disimpan dan Anda tidak perlu masuk lagi.

<h2 id="step-3-start-your-first-session">
  Langkah 3: Mulai sesi pertama Anda
</h2>

Buka terminal Anda di direktori proyek mana pun dan mulai Claude Code:

```bash theme={null}
cd /path/to/your/project
claude
```

Anda akan melihat prompt Claude Code dengan versi, model saat ini, dan direktori kerja yang ditampilkan di atasnya. Ketik `/help` untuk perintah yang tersedia atau `/resume` untuk melanjutkan percakapan sebelumnya.

<Tip>
  Setelah masuk (Langkah 2), kredensial Anda disimpan di sistem Anda. Pelajari lebih lanjut di [Manajemen Kredensial](/docs/id/authentication#credential-management).
</Tip>

<h2 id="step-4-ask-your-first-question">
  Langkah 4: Ajukan pertanyaan pertama Anda
</h2>

Mari kita mulai dengan memahami basis kode Anda. Coba salah satu perintah ini:

```text theme={null}
apa yang dilakukan proyek ini?
```

Claude akan menganalisis file Anda dan memberikan ringkasan. Anda juga dapat mengajukan pertanyaan yang lebih spesifik:

```text theme={null}
teknologi apa yang digunakan proyek ini?
```

```text theme={null}
di mana titik masuk utama?
```

```text theme={null}
jelaskan struktur folder
```

Anda juga dapat menanyakan Claude tentang kemampuannya sendiri:

```text theme={null}
apa yang dapat dilakukan Claude Code?
```

```text theme={null}
bagaimana cara membuat skills kustom di Claude Code?
```

```text theme={null}
bisakah Claude Code bekerja dengan Docker?
```

<Note>
  Claude Code membaca file proyek Anda sesuai kebutuhan. Anda tidak perlu menambahkan konteks secara manual.
</Note>

<h2 id="step-5-make-your-first-code-change">
  Langkah 5: Buat perubahan kode pertama Anda
</h2>

Sekarang mari buat Claude Code melakukan beberapa pengkodean aktual. Coba tugas sederhana:

```text theme={null}
tambahkan fungsi hello world ke file utama
```

Claude Code akan:

1. Menemukan file yang sesuai
2. Menampilkan perubahan yang diusulkan
3. Meminta persetujuan Anda
4. Membuat edit

<Note>
  Claude Code selalu meminta izin sebelum memodifikasi file. Anda dapat menyetujui perubahan individual atau mengaktifkan mode "Terima semua" untuk sesi.
</Note>

<h2 id="step-6-use-git-with-claude-code">
  Langkah 6: Gunakan Git dengan Claude Code
</h2>

Claude Code membuat operasi Git menjadi percakapan:

```text theme={null}
file apa yang telah saya ubah?
```

```text theme={null}
komit perubahan saya dengan pesan deskriptif
```

Anda juga dapat meminta operasi Git yang lebih kompleks:

```text theme={null}
buat cabang baru bernama feature/quickstart
```

```text theme={null}
tunjukkan 5 komit terakhir saya
```

```text theme={null}
bantu saya menyelesaikan konflik penggabungan
```

<h2 id="step-7-fix-a-bug-or-add-a-feature">
  Langkah 7: Perbaiki bug atau tambahkan fitur
</h2>

Claude mahir dalam debugging dan implementasi fitur.

Jelaskan apa yang Anda inginkan dalam bahasa alami:

```text theme={null}
tambahkan validasi input ke formulir pendaftaran pengguna
```

Atau perbaiki masalah yang ada:

```text theme={null}
ada bug di mana pengguna dapat mengirimkan formulir kosong - perbaiki
```

Claude Code akan:

* Menemukan kode yang relevan
* Memahami konteksnya
* Menerapkan solusi
* Menjalankan tes jika tersedia

<h2 id="step-8-test-out-other-common-workflows">
  Langkah 8: Coba alur kerja umum lainnya
</h2>

Ada beberapa cara untuk bekerja dengan Claude:

**Refaktor kode**

```text theme={null}
refaktor modul autentikasi untuk menggunakan async/await alih-alih callback
```

**Tulis tes**

```text theme={null}
tulis unit test untuk fungsi kalkulator
```

**Perbarui dokumentasi**

```text theme={null}
perbarui README dengan instruksi instalasi
```

**Tinjauan kode**

```text theme={null}
tinjau perubahan saya dan sarankan perbaikan
```

<Tip>
  Berbicara dengan Claude seperti Anda berbicara dengan rekan kerja yang membantu. Jelaskan apa yang ingin Anda capai, dan Claude akan membantu Anda mencapainya.
</Tip>

<h2 id="essential-commands">
  Perintah penting
</h2>

Berikut adalah perintah paling penting untuk penggunaan sehari-hari. Perintah shell dijalankan dari terminal Anda untuk memulai atau melanjutkan Claude Code. Perintah sesi dijalankan di dalam Claude Code setelah dimulai.

**Perintah shell**

| Perintah            | Apa yang dilakukannya                              | Contoh                              |
| ------------------- | -------------------------------------------------- | ----------------------------------- |
| `claude`            | Mulai mode interaktif                              | `claude`                            |
| `claude "task"`     | Jalankan tugas satu kali                           | `claude "perbaiki kesalahan build"` |
| `claude -p "query"` | Jalankan kueri sekali, lalu keluar                 | `claude -p "jelaskan fungsi ini"`   |
| `claude -c`         | Lanjutkan percakapan terbaru di direktori saat ini | `claude -c`                         |
| `claude -r`         | Lanjutkan percakapan sebelumnya                    | `claude -r`                         |

**Perintah sesi**

| Perintah            | Apa yang dilakukannya            | Contoh   |
| ------------------- | -------------------------------- | -------- |
| `/clear`            | Hapus riwayat percakapan         | `/clear` |
| `/help`             | Tampilkan perintah yang tersedia | `/help`  |
| `/exit` atau Ctrl+D | Keluar dari Claude Code          | `/exit`  |

Lihat [referensi CLI](/docs/id/cli-reference) untuk daftar lengkap perintah shell dan [referensi perintah](/docs/id/commands) untuk daftar lengkap perintah sesi.

<h2 id="pro-tips-for-beginners">
  Tips pro untuk pemula
</h2>

Untuk informasi lebih lanjut, lihat [praktik terbaik](/docs/id/best-practices) dan [alur kerja umum](/docs/id/common-workflows).

<AccordionGroup>
  <Accordion title="Jadilah spesifik dengan permintaan Anda">
    Alih-alih: "perbaiki bug"

    Coba: "perbaiki bug login di mana pengguna melihat layar kosong setelah memasukkan kredensial yang salah"
  </Accordion>

  <Accordion title="Gunakan instruksi langkah demi langkah">
    Pecah tugas kompleks menjadi langkah-langkah:

    ```text theme={null}
    1. buat tabel database baru untuk profil pengguna
    2. buat endpoint API untuk mendapatkan dan memperbarui profil pengguna
    3. bangun halaman web yang memungkinkan pengguna melihat dan mengedit informasi mereka
    ```
  </Accordion>

  <Accordion title="Biarkan Claude menjelajahi terlebih dahulu">
    Sebelum membuat perubahan, biarkan Claude memahami kode Anda:

    ```text theme={null}
    analisis skema database
    ```

    ```text theme={null}
    bangun dasbor yang menampilkan produk yang paling sering dikembalikan oleh pelanggan Inggris kami
    ```
  </Accordion>

  <Accordion title="Hemat waktu dengan pintasan keyboard">
    * Tekan `/` untuk melihat semua perintah dan skills
    * Gunakan Tab untuk penyelesaian perintah
    * Tekan ↑ untuk riwayat perintah
    * Tekan `Shift+Tab` untuk mengubah mode izin
  </Accordion>
</AccordionGroup>

<h2 id="what’s-next">
  Apa selanjutnya?
</h2>

Sekarang yang Anda telah mempelajari dasar-dasarnya, jelajahi fitur-fitur yang lebih canggih:

<CardGroup cols={2}>
  <Card title="Cara kerja Claude Code" icon="microchip" href="/docs/id/how-claude-code-works">
    Pahami loop agentic, alat bawaan, dan cara Claude Code berinteraksi dengan proyek Anda
  </Card>

  <Card title="Praktik terbaik" icon="star" href="/docs/id/best-practices">
    Dapatkan hasil yang lebih baik dengan prompting yang efektif dan pengaturan proyek
  </Card>

  <Card title="Alur kerja umum" icon="graduation-cap" href="/docs/id/common-workflows">
    Panduan langkah demi langkah untuk tugas-tugas umum
  </Card>

  <Card title="Perluas Claude Code" icon="puzzle-piece" href="/docs/id/features-overview">
    Sesuaikan dengan CLAUDE.md, skills, hooks, MCP, dan lainnya
  </Card>
</CardGroup>

<h2 id="getting-help">
  Mendapatkan bantuan
</h2>

* **Di Claude Code**: Ketik `/help` atau tanya "bagaimana cara saya..."
* **Dokumentasi**: Anda di sini! Jelajahi panduan lainnya
* **Komunitas**: Bergabunglah dengan [Discord](https://www.anthropic.com/discord) kami untuk tips dan dukungan
