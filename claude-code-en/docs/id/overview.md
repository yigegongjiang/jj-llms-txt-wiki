> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Ikhtisar

> Claude Code adalah alat pengkodean agentic yang membaca basis kode Anda, mengedit file, menjalankan perintah, dan terintegrasi dengan alat pengembangan Anda. Tersedia di terminal, IDE, aplikasi desktop, dan browser.

Claude Code adalah asisten pengkodean bertenaga AI yang membantu Anda membangun fitur, memperbaiki bug, dan mengotomatisasi tugas pengembangan. Ini memahami seluruh basis kode Anda dan dapat bekerja di berbagai file dan alat untuk menyelesaikan pekerjaan.

<h2 id="get-started">
  Memulai
</h2>

Claude Code berjalan di beberapa permukaan: terminal, ekstensi IDE, aplikasi desktop, dan web. Pilih salah satu dari tab di bawah untuk memulai. Sebagian besar permukaan memerlukan [langganan Claude](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=overview_pricing) atau akun [Konsol Anthropic](https://platform.claude.com/). CLI Terminal, VS Code, dan JetBrains juga mendukung [penyedia pihak ketiga](/docs/id/third-party-integrations).

<Tabs>
  <Tab title="Terminal">
    CLI lengkap untuk bekerja dengan Claude Code langsung di terminal Anda. Edit file, jalankan perintah, dan kelola seluruh proyek Anda dari baris perintah.

    Untuk menginstal Claude Code, gunakan salah satu metode berikut:

    <Tabs>
      <Tab title="Native Install (Direkomendasikan)">
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

        Jika Anda melihat `The token '&&' is not a valid statement separator`, Anda berada di PowerShell, bukan CMD. Jika Anda melihat `'irm' is not recognized as an internal or external command`, Anda berada di CMD, bukan PowerShell. Prompt Anda menunjukkan `PS C:\` ketika Anda berada di PowerShell dan `C:\` tanpa `PS` ketika Anda berada di CMD.

        Jika perintah instalasi gagal dengan `syntax error near unexpected token '<'`, `403`, atau kesalahan curl lainnya, lihat [Troubleshoot installation](/docs/id/troubleshoot-install#find-your-error) untuk mencocokkan kesalahan dengan perbaikan dan untuk metode instalasi alternatif.

        [Git for Windows](https://git-scm.com/downloads/win) direkomendasikan pada Windows native sehingga Claude Code dapat menggunakan alat Bash. Jika Git for Windows tidak diinstal, Claude Code menggunakan PowerShell sebagai alat shell sebagai gantinya. Pengaturan WSL tidak memerlukan Git for Windows.

        <Info>
          Instalasi native secara otomatis diperbarui di latar belakang untuk membuat Anda tetap menggunakan versi terbaru.
        </Info>
      </Tab>

      <Tab title="Homebrew">
        ```bash theme={null}
        brew install --cask claude-code
        ```

        Homebrew menawarkan dua casks. `claude-code` melacak saluran rilis stabil, yang biasanya sekitar seminggu di belakang dan melewatkan rilis dengan regresi besar. `claude-code@latest` melacak saluran terbaru dan menerima versi baru segera setelah mereka dirilis.

        <Info>
          Instalasi Homebrew tidak auto-update. Jalankan `brew upgrade claude-code` atau `brew upgrade claude-code@latest`, tergantung pada cask mana yang Anda instal, untuk mendapatkan fitur terbaru dan perbaikan keamanan.
        </Info>
      </Tab>

      <Tab title="WinGet">
        ```powershell theme={null}
        winget install Anthropic.ClaudeCode
        ```

        <Info>
          Instalasi WinGet tidak auto-update. Jalankan `winget upgrade Anthropic.ClaudeCode` secara berkala untuk mendapatkan fitur terbaru dan perbaikan keamanan.
        </Info>
      </Tab>
    </Tabs>

    Anda juga dapat menginstal dengan [apt, dnf, atau apk](/docs/id/setup#install-with-linux-package-managers) pada Debian, Fedora, RHEL, dan Alpine.

    Kemudian mulai Claude Code di proyek apa pun. Ganti `your-project` dengan jalur ke direktori proyek di mesin Anda:

    ```bash theme={null}
    cd your-project
    claude
    ```

    Anda akan diminta untuk masuk pada penggunaan pertama. Jika Anda telah menetapkan variabel lingkungan `ANTHROPIC_API_KEY`, Claude Code melewati prompt login dan meminta Anda untuk menyetujui kunci sebagai gantinya. Itu saja! [Lanjutkan dengan Quickstart →](/docs/id/quickstart)

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
    * Di Ubuntu atau Debian, di mana aplikasi masih dalam tahap beta, instal dengan apt dengan mengikuti [instruksi instalasi Linux](/docs/id/desktop-linux)

    Setelah menginstal, luncurkan Claude, masuk, dan klik tab **Code** untuk mulai pengkodean. Aplikasi ini mencakup Claude Code, jadi Anda tidak perlu menginstal CLI secara terpisah. [Langganan berbayar](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=overview_desktop_pricing) diperlukan.

    [Pelajari lebih lanjut tentang aplikasi desktop →](/docs/id/desktop-quickstart)
  </Tab>

  <Tab title="Web">
    Jalankan Claude Code di browser Anda tanpa pengaturan lokal. Mulai tugas yang berjalan lama dan periksa kembali saat selesai, bekerja pada repo yang tidak Anda miliki secara lokal, atau jalankan beberapa tugas secara paralel. Untuk badan pekerjaan yang lebih lama, buat [proyek](/docs/id/claude-projects) dan biarkan Claude mengoordinasikan sesi paralel untuk Anda. Tersedia di browser desktop dan [aplikasi Claude untuk iOS dan Android](/docs/id/mobile).

    Mulai pengkodean di [claude.ai/code](https://claude.ai/code).

    [Mulai →](/docs/id/web-quickstart)
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
    [`CLAUDE.md`](/docs/id/memory) adalah file markdown yang Anda tambahkan ke root proyek Anda yang dibaca Claude Code di awal setiap sesi. Gunakan untuk menetapkan standar pengkodean, keputusan arsitektur, perpustakaan pilihan, dan daftar periksa tinjauan. Jika repositori Anda sudah memiliki `AGENTS.md` untuk agen pengkodean lainnya, Claude Code [dapat membacanya](/docs/id/memory#agents-md) sendiri atau bersama `CLAUDE.md`. Claude juga membangun [memori otomatis](/docs/id/memory#auto-memory) saat bekerja, menyimpan pembelajaran di seluruh sesi tanpa Anda menulis apa pun.

    Buat [skills](/docs/id/skills) untuk mengemas alur kerja yang dapat diulang yang dapat dibagikan tim Anda, seperti `/review-pr` atau `/deploy-staging`.

    [Hooks](/docs/id/hooks) memungkinkan Anda menjalankan perintah shell sebelum atau sesudah tindakan Claude Code, seperti pemformatan otomatis setelah setiap pengeditan file atau menjalankan lint sebelum commit.
  </Accordion>

  <Accordion title="Jalankan agen secara paralel dan bangun agen khusus" icon="users">
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

    * [Routines](/docs/id/routines) berjalan di cloud, jadi mereka terus berjalan bahkan ketika komputer Anda mati. Mereka juga dapat dipicu oleh panggilan API atau acara GitHub. Buatnya dari web, aplikasi Desktop, atau dengan menjalankan `/schedule` di CLI.
    * [Tugas terjadwal desktop](/docs/id/desktop-scheduled-tasks) berjalan di mesin Anda, dengan akses langsung ke file dan alat lokal Anda
    * [`/loop`](/docs/id/scheduled-tasks) mengulangi prompt dalam sesi CLI untuk polling cepat
  </Accordion>

  <Accordion title="Bekerja dari mana saja" icon="globe">
    Sesi tidak terikat pada satu permukaan. Pindahkan pekerjaan antar lingkungan saat konteks Anda berubah:

    * Tinggalkan meja Anda dan terus bekerja dari ponsel atau browser apa pun dengan [Remote Control](/docs/id/remote-control)
    * Kirim pesan [Dispatch](/docs/id/desktop#sessions-from-dispatch) tugas dari ponsel Anda dan buka sesi Desktop yang dibuatnya
    * Mulai tugas yang berjalan lama di [web](/docs/id/claude-code-on-the-web) atau [aplikasi Claude mobile](/docs/id/mobile), kemudian tariknya ke terminal Anda dengan `claude --teleport`. Teleport memerlukan langganan claude.ai.
    * Jalankan `/desktop` untuk melanjutkan sesi terminal Anda saat ini di [aplikasi Desktop](/docs/id/desktop), tempat Anda dapat meninjau diff secara visual. Handoff `/desktop` memerlukan langganan claude.ai. Tersedia di macOS dan Windows x64.
    * Rute tugas dari obrolan tim: sebutkan `@Claude` di [Slack](/docs/id/slack) dengan laporan bug dan dapatkan pull request kembali
  </Accordion>
</AccordionGroup>

<h2 id="use-claude-code-everywhere">
  Gunakan Claude Code di mana saja
</h2>

Setiap [permukaan](/docs/id/glossary#surface) terhubung ke mesin Claude Code yang mendasar yang sama, jadi file CLAUDE.md, pengaturan, dan server MCP Anda bekerja di semua permukaan.

Selain permukaan [Terminal](/docs/id/quickstart), [VS Code](/docs/id/vs-code), [JetBrains](/docs/id/jetbrains), [Desktop](/docs/id/desktop), dan [Web](/docs/id/claude-code-on-the-web) di atas, Claude Code terintegrasi dengan alur kerja CI/CD, obrolan, dan browser:

| Apa yang ingin saya lakukan                                                            | Opsi terbaik                                                                                                         |
| -------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------- |
| Lanjutkan sesi lokal dari ponsel atau perangkat lain                                   | [Remote Control](/docs/id/remote-control)                                                                                 |
| Dorong acara dari Telegram, Discord, iMessage, atau webhook saya sendiri ke dalam sesi | [Channels](/docs/id/channels)                                                                                             |
| Mulai tugas secara lokal, lanjutkan di mobile                                          | [`claude --cloud`](/docs/id/claude-code-on-the-web#from-terminal-to-cloud), kemudian [aplikasi Claude mobile](/docs/id/mobile) |
| Jalankan Claude sesuai jadwal berulang                                                 | [Routines](/docs/id/routines) atau [Tugas terjadwal desktop](/docs/id/desktop-scheduled-tasks)                                 |
| Otomatisasi tinjauan PR dan triase masalah                                             | [GitHub Actions](/docs/id/github-actions) atau [GitLab CI/CD](/docs/id/gitlab-ci-cd)                                           |
| Dapatkan tinjauan kode otomatis di setiap PR                                           | [GitHub Code Review](/docs/id/code-review)                                                                                |
| Rute laporan bug dari Slack ke pull request                                            | [Slack](/docs/id/slack)                                                                                                   |
| Debug aplikasi web langsung                                                            | [Chrome](/docs/id/chrome)                                                                                                 |
| Bangun agen khusus untuk alur kerja Anda sendiri                                       | [Agent SDK](/docs/id/agent-sdk/overview)                                                                                  |

<h2 id="next-steps">
  Langkah berikutnya
</h2>

Setelah Anda menginstal Claude Code, panduan ini membantu Anda menggali lebih dalam.

* [Quickstart](/docs/id/quickstart): berjalan melalui tugas nyata pertama Anda, dari menjelajahi basis kode hingga melakukan perbaikan
* [Simpan instruksi dan memori](/docs/id/memory): berikan Claude instruksi persisten dengan file CLAUDE.md dan memori otomatis
* [Alur kerja umum](/docs/id/common-workflows) dan [praktik terbaik](/docs/id/best-practices): pola untuk mendapatkan hasil maksimal dari Claude Code
* [Claude Academy](https://academy.claude.com/): kursus gratis dengan kecepatan sendiri, termasuk [Claude Code 101](https://academy.claude.com/courses/claude-code-101) dan [Claude Code in Action](https://academy.claude.com/courses/claude-code-in-action)
* [Sebuah harness untuk setiap tugas](https://claude.com/blog/a-harness-for-every-task-dynamic-workflows-in-claude-code): bagaimana tim Claude Code menggunakan [dynamic workflows](/docs/id/workflows) untuk mengorkestra subagen dalam skala besar
* [Pengaturan](/docs/id/settings): sesuaikan Claude Code untuk alur kerja Anda
* [Pemecahan masalah](/docs/id/troubleshooting): solusi untuk masalah umum
* [code.claude.com](https://code.claude.com/): demo, harga, dan detail produk
