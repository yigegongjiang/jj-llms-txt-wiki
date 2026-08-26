> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Ikhtisar

> Claude Code adalah alat pengkodean agentic yang membaca basis kode Anda, mengedit file, menjalankan perintah, dan terintegrasi dengan alat pengembangan Anda. Tersedia di terminal, IDE, aplikasi desktop, dan browser.

Claude Code adalah asisten pengkodean bertenaga AI yang membantu Anda membangun fitur, memperbaiki bug, dan mengotomatisasi tugas pengembangan. Ini memahami seluruh basis kode Anda dan dapat bekerja di berbagai file dan alat untuk menyelesaikan pekerjaan.

<h2 id="get-started">
  Memulai
</h2>

Claude Code berjalan di beberapa permukaan: terminal, ekstensi IDE, aplikasi desktop, dan web. Pilih salah satu dari tab di bawah untuk memulai. Sebagian besar permukaan memerlukan [langganan Claude](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=overview_pricing) atau akun [Konsol Anthropic](https://console.anthropic.com/). CLI Terminal dan VS Code juga mendukung [penyedia pihak ketiga](/docs/id/third-party-integrations).

<Tabs>
  <Tab title="Terminal">
    CLI lengkap untuk bekerja dengan Claude Code langsung di terminal Anda. Edit file, jalankan perintah, dan kelola seluruh proyek Anda dari baris perintah.

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

    Kemudian mulai Claude Code di proyek apa pun:

    ```bash theme={null}
    cd your-project
    claude
    ```

    Anda akan diminta untuk masuk pada penggunaan pertama. Itu saja! [Lanjutkan dengan Quickstart →](/docs/id/quickstart)

    <Tip>
      Lihat [pengaturan lanjutan](/docs/id/setup) untuk opsi instalasi, pembaruan manual, atau instruksi penghapusan. Kunjungi [pemecahan masalah instalasi](/docs/id/troubleshoot-install) jika Anda mengalami masalah.
    </Tip>
  </Tab>

  <Tab title="VS Code">
    Ekstensi VS Code menyediakan diff inline, @-mentions, tinjauan rencana, dan riwayat percakapan langsung di editor Anda.

    * [Instal untuk VS Code](vscode:extension/anthropic.claude-code)
    * [Instal untuk Cursor](cursor:extension/anthropic.claude-code)

    Atau cari "Claude Code" di tampilan Ekstensi (`Cmd+Shift+X` di Mac, `Ctrl+Shift+X` di Windows/Linux). Setelah menginstal, buka Palet Perintah (`Cmd+Shift+P` / `Ctrl+Shift+P`), ketik "Claude Code", dan pilih **Buka di Tab Baru**.

    [Mulai dengan VS Code →](/docs/id/vs-code#get-started)
  </Tab>

  <Tab title="Desktop app">
    Aplikasi mandiri untuk menjalankan Claude Code di luar IDE atau terminal Anda. Tinjau diff secara visual, jalankan beberapa sesi berdampingan, jadwalkan tugas berulang, dan mulai sesi cloud.

    Unduh dan instal:

    * [macOS](https://claude.ai/api/desktop/darwin/universal/dmg/latest/redirect?utm_source=claude_code\&utm_medium=docs) (Intel dan Apple Silicon)
    * [Windows](https://claude.ai/api/desktop/win32/x64/setup/latest/redirect?utm_source=claude_code\&utm_medium=docs) (x64)
    * [Windows ARM64](https://claude.ai/api/desktop/win32/arm64/setup/latest/redirect?utm_source=claude_code\&utm_medium=docs)

    Setelah menginstal, luncurkan Claude, masuk, dan klik tab **Code** untuk mulai pengkodean. [Langganan berbayar](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=overview_desktop_pricing) diperlukan.

    [Pelajari lebih lanjut tentang aplikasi desktop →](/docs/id/desktop-quickstart)
  </Tab>

  <Tab title="Web">
    Jalankan Claude Code di browser Anda tanpa pengaturan lokal. Mulai tugas yang berjalan lama dan periksa kembali saat selesai, bekerja pada repo yang tidak Anda miliki secara lokal, atau jalankan beberapa tugas secara paralel. Tersedia di browser desktop dan aplikasi Claude iOS.

    Mulai pengkodean di [claude.ai/code](https://claude.ai/code).

    [Mulai di web →](/docs/id/web-quickstart)
  </Tab>

  <Tab title="JetBrains">
    Plugin untuk IntelliJ IDEA, PyCharm, WebStorm, dan IDE JetBrains lainnya dengan tampilan diff interaktif dan berbagi konteks seleksi.

    Instal [plugin Claude Code](https://plugins.jetbrains.com/plugin/27310-claude-code-beta-) dari JetBrains Marketplace dan mulai ulang IDE Anda. Plugin memerlukan CLI Claude Code, diinstal secara terpisah; lihat [langkah-langkah pengaturan JetBrains](/docs/id/jetbrains#installation).

    [Mulai dengan JetBrains →](/docs/id/jetbrains)
  </Tab>
</Tabs>

<h2 id="what-you-can-do">
  Apa yang dapat Anda lakukan
</h2>

Berikut adalah beberapa cara Anda dapat menggunakan Claude Code:

<AccordionGroup>
  <Accordion title="Otomatisasi pekerjaan yang terus Anda tunda" icon="wand-magic-sparkles">
    Claude Code menangani tugas-tugas membosankan yang menghabiskan hari Anda: menulis tes untuk kode yang tidak diuji, memperbaiki kesalahan lint di seluruh proyek, menyelesaikan konflik penggabungan, memperbarui dependensi, dan menulis catatan rilis.

    ```bash theme={null}
    claude "write tests for the auth module, run them, and fix any failures"
    ```
  </Accordion>

  <Accordion title="Bangun fitur dan perbaiki bug" icon="hammer">
    Jelaskan apa yang Anda inginkan dalam bahasa biasa. Claude Code merencanakan pendekatan, menulis kode di berbagai file, dan memverifikasi bahwa itu berfungsi.

    Untuk bug, tempel pesan kesalahan atau jelaskan gejalanya. Claude Code melacak masalah melalui basis kode Anda, mengidentifikasi akar penyebabnya, dan menerapkan perbaikan. Lihat [alur kerja umum](/docs/id/common-workflows) untuk contoh lebih lanjut.
  </Accordion>

  <Accordion title="Buat commit dan pull request" icon="code-branch">
    Claude Code bekerja langsung dengan git. Ini menampilkan perubahan, menulis pesan commit, membuat cabang, dan membuka pull request.

    ```bash theme={null}
    claude "commit my changes with a descriptive message"
    ```

    Di CI, Anda dapat mengotomatisasi tinjauan kode dan triase masalah dengan [GitHub Actions](/docs/id/github-actions) atau [GitLab CI/CD](/docs/id/gitlab-ci-cd).
  </Accordion>

  <Accordion title="Hubungkan alat Anda dengan MCP" icon="plug">
    [Model Context Protocol (MCP)](/docs/id/mcp) adalah standar terbuka untuk menghubungkan alat AI ke sumber data eksternal. Dengan MCP, Claude Code dapat membaca dokumen desain Anda di Google Drive, memperbarui tiket di Jira, menarik data dari Slack, atau menggunakan alat khusus Anda sendiri. [Panduan cepat MCP](/docs/id/mcp-quickstart) menghubungkan server pertama Anda dari awal hingga akhir.
  </Accordion>

  <Accordion title="Sesuaikan dengan instruksi, skills, dan hooks" icon="sliders">
    [`CLAUDE.md`](/docs/id/memory) adalah file markdown yang Anda tambahkan ke root proyek Anda yang dibaca Claude Code di awal setiap sesi. Gunakan untuk menetapkan standar pengkodean, keputusan arsitektur, perpustakaan pilihan, dan daftar periksa tinjauan. Claude juga membangun [memori otomatis](/docs/id/memory#auto-memory) saat bekerja, menyimpan pembelajaran seperti perintah build dan wawasan debugging di seluruh sesi tanpa Anda menulis apa pun.

    Buat [skills](/docs/id/skills) untuk mengemas alur kerja yang dapat diulang yang dapat dibagikan tim Anda, seperti `/review-pr` atau `/deploy-staging`.

    [Hooks](/docs/id/hooks) memungkinkan Anda menjalankan perintah shell sebelum atau sesudah tindakan Claude Code, seperti pemformatan otomatis setelah setiap pengeditan file atau menjalankan lint sebelum commit.
  </Accordion>

  <Accordion title="Jalankan tim agen dan bangun agen khusus" icon="users">
    Spawn [beberapa agen Claude Code](/docs/id/sub-agents) yang bekerja pada bagian berbeda dari tugas secara bersamaan. Agen utama mengoordinasikan pekerjaan, menetapkan subtask, dan menggabungkan hasil.

    Untuk menjalankan beberapa sesi lengkap secara paralel dan menontonnya dari satu layar, gunakan [agen latar belakang](/docs/id/agent-view). Untuk alur kerja yang sepenuhnya khusus, [Agent SDK](/docs/id/agent-sdk/overview) memungkinkan Anda membangun agen Anda sendiri yang didukung oleh alat dan kemampuan Claude Code, dengan kontrol penuh atas orkestrasi, akses alat, dan izin.
  </Accordion>

  <Accordion title="Pipa, skrip, dan otomatisasi dengan CLI" icon="terminal">
    Claude Code dapat dikomposisi dan mengikuti filosofi Unix. Pipa log ke dalamnya, jalankan di CI, atau rantai dengan alat lain:

    ```bash theme={null}
    # Analisis keluaran log terbaru
    tail -200 app.log | claude -p "Slack me if you see any anomalies"

    # Otomatisasi terjemahan di CI
    claude -p "translate new strings into French and raise a PR for review"

    # Operasi massal di seluruh file
    git diff main --name-only | claude -p "review these changed files for security issues"
    ```

    Lihat [referensi CLI](/docs/id/cli-reference) untuk set lengkap perintah dan flag.
  </Accordion>

  <Accordion title="Jadwalkan tugas berulang" icon="clock">
    Jalankan Claude sesuai jadwal untuk mengotomatisasi pekerjaan yang berulang: tinjauan PR pagi, analisis kegagalan CI semalam, audit dependensi mingguan, atau sinkronisasi dokumen setelah PR digabung.

    * [Routines](/docs/id/routines) berjalan pada infrastruktur yang dikelola Anthropic, jadi mereka terus berjalan bahkan ketika komputer Anda mati. Mereka juga dapat dipicu oleh panggilan API atau acara GitHub. Buatnya dari web, aplikasi Desktop, atau dengan menjalankan `/schedule` di CLI.
    * [Tugas terjadwal desktop](/docs/id/desktop-scheduled-tasks) berjalan di mesin Anda, dengan akses langsung ke file dan alat lokal Anda
    * [`/loop`](/docs/id/scheduled-tasks) mengulangi prompt dalam sesi CLI untuk polling cepat
  </Accordion>

  <Accordion title="Bekerja dari mana saja" icon="globe">
    Sesi tidak terikat pada satu permukaan. Pindahkan pekerjaan antar lingkungan saat konteks Anda berubah:

    * Tinggalkan meja Anda dan terus bekerja dari ponsel atau browser apa pun dengan [Remote Control](/docs/id/remote-control)
    * Kirim pesan [Dispatch](/docs/id/desktop#sessions-from-dispatch) tugas dari ponsel Anda dan buka sesi Desktop yang dibuatnya
    * Mulai tugas yang berjalan lama di [web](/docs/id/claude-code-on-the-web) atau [aplikasi iOS](https://apps.apple.com/app/claude-by-anthropic/id6473753684), kemudian tariknya ke terminal Anda dengan `claude --teleport`. Teleport memerlukan langganan claude.ai.
    * Serahkan sesi terminal ke [aplikasi Desktop](/docs/id/desktop) dengan `/desktop` untuk tinjauan diff visual
    * Rute tugas dari obrolan tim: sebutkan `@Claude` di [Slack](/docs/id/slack) dengan laporan bug dan dapatkan pull request kembali
  </Accordion>
</AccordionGroup>

<h2 id="use-claude-code-everywhere">
  Gunakan Claude Code di mana saja
</h2>

Setiap [permukaan](/docs/id/glossary#surface) terhubung ke mesin Claude Code yang mendasar yang sama, jadi file CLAUDE.md, pengaturan, dan server MCP Anda bekerja di semua permukaan.

Selain permukaan [Terminal](/docs/id/quickstart), [VS Code](/docs/id/vs-code), [JetBrains](/docs/id/jetbrains), [Desktop](/docs/id/desktop), dan [Web](/docs/id/claude-code-on-the-web) di atas, Claude Code terintegrasi dengan alur kerja CI/CD, obrolan, dan browser:

| Saya ingin...                                                                          | Opsi terbaik                                                                                                              |
| -------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------- |
| Lanjutkan sesi lokal dari ponsel atau perangkat lain                                   | [Remote Control](/docs/id/remote-control)                                                                                      |
| Dorong acara dari Telegram, Discord, iMessage, atau webhook saya sendiri ke dalam sesi | [Channels](/docs/id/channels)                                                                                                  |
| Mulai tugas secara lokal, lanjutkan di mobile                                          | [Web](/docs/id/claude-code-on-the-web) atau [aplikasi Claude iOS](https://apps.apple.com/app/claude-by-anthropic/id6473753684) |
| Jalankan Claude sesuai jadwal berulang                                                 | [Routines](/docs/id/routines) atau [Tugas terjadwal desktop](/docs/id/desktop-scheduled-tasks)                                      |
| Otomatisasi tinjauan PR dan triase masalah                                             | [GitHub Actions](/docs/id/github-actions) atau [GitLab CI/CD](/docs/id/gitlab-ci-cd)                                                |
| Dapatkan tinjauan kode otomatis di setiap PR                                           | [GitHub Code Review](/docs/id/code-review)                                                                                     |
| Rute laporan bug dari Slack ke pull request                                            | [Slack](/docs/id/slack)                                                                                                        |
| Debug aplikasi web langsung                                                            | [Chrome](/docs/id/chrome)                                                                                                      |
| Bangun agen khusus untuk alur kerja Anda sendiri                                       | [Agent SDK](/docs/id/agent-sdk/overview)                                                                                       |

<h2 id="next-steps">
  Langkah berikutnya
</h2>

Setelah Anda menginstal Claude Code, panduan ini membantu Anda menggali lebih dalam.

* [Quickstart](/docs/id/quickstart): berjalan melalui tugas nyata pertama Anda, dari menjelajahi basis kode hingga melakukan perbaikan
* [Simpan instruksi dan memori](/docs/id/memory): berikan Claude instruksi persisten dengan file CLAUDE.md dan memori otomatis
* [Alur kerja umum](/docs/id/common-workflows) dan [praktik terbaik](/docs/id/best-practices): pola untuk mendapatkan hasil maksimal dari Claude Code
* [Sebuah harness untuk setiap tugas](https://claude.com/blog/a-harness-for-every-task-dynamic-workflows-in-claude-code): bagaimana tim Claude Code menggunakan [dynamic workflows](/docs/id/workflows) untuk mengorkestra subagen dalam skala besar
* [Pengaturan](/docs/id/settings): sesuaikan Claude Code untuk alur kerja Anda
* [Pemecahan masalah](/docs/id/troubleshooting): solusi untuk masalah umum
* [code.claude.com](https://code.claude.com/): demo, harga, dan detail produk
