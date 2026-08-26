> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Gunakan Claude Code di VS Code

> Instal dan konfigurasi ekstensi Claude Code untuk VS Code. Dapatkan bantuan pengkodean AI dengan diff inline, @-mentions, review rencana, dan pintasan keyboard.

<img src="https://mintcdn.com/claude-code/-YhHHmtSxwr7W8gy/images/vs-code-extension-interface.jpg?fit=max&auto=format&n=-YhHHmtSxwr7W8gy&q=85&s=300652d5678c63905e6b0ea9e50835f8" alt="Editor VS Code dengan panel ekstensi Claude Code terbuka di sisi kanan, menampilkan percakapan dengan Claude" width="2500" height="1155" data-path="images/vs-code-extension-interface.jpg" />

Ekstensi VS Code menyediakan antarmuka grafis asli untuk Claude Code, terintegrasi langsung ke dalam IDE Anda. Ini adalah cara yang direkomendasikan untuk menggunakan Claude Code di VS Code.

Dengan ekstensi, Anda dapat meninjau dan mengedit rencana Claude sebelum menerimanya, auto-accept edits saat dibuat, @-mention file dengan rentang baris tertentu dari pilihan Anda, mengakses riwayat percakapan, dan membuka beberapa percakapan di tab atau jendela terpisah.

<h2 id="prerequisites">
  Prasyarat
</h2>

Sebelum menginstal, pastikan Anda memiliki:

* VS Code 1.98.0 atau lebih tinggi
* Akun Anthropic: langganan Claude berbayar apa pun (Pro, Max, Team, atau Enterprise) atau akun Claude Console berfungsi, dan tidak ada kunci API yang diperlukan. Anda akan [masuk](/docs/id/authentication#log-in-to-claude-code) dengan akun ini saat pertama kali membuka ekstensi. Jika Anda mengakses Claude melalui penyedia pihak ketiga seperti Amazon Bedrock atau Google Cloud's Agent Platform, lihat [Gunakan penyedia pihak ketiga](#use-third-party-providers) untuk petunjuk penyiapan.

<Tip>
  Ekstensi menggabungkan salinan CLI (command-line interface) miliknya sendiri untuk panel obrolan. Untuk menjalankan `claude` di terminal terintegrasi VS Code, Anda juga memerlukan [instalasi CLI mandiri](/docs/id/setup). Lihat [Ekstensi VS Code vs. Claude Code CLI](#vs-code-extension-vs-claude-code-cli) untuk detail.
</Tip>

<h2 id="install-the-extension">
  Instal ekstensi
</h2>

Klik tautan untuk IDE Anda untuk menginstal secara langsung:

* [Instal untuk VS Code](vscode:extension/anthropic.claude-code)
* [Instal untuk Cursor](cursor:extension/anthropic.claude-code)

Atau di VS Code, tekan `Cmd+Shift+X` (Mac) atau `Ctrl+Shift+X` (Windows/Linux) untuk membuka tampilan Extensions, cari "Claude Code", dan klik **Install**.

Ekstensi juga dapat diinstal di fork VS Code lainnya seperti Devin Desktop atau Kiro. Cari "Claude Code" di tampilan Extensions editor Anda, atau instal dari [registri Open VSX](https://open-vsx.org/extension/Anthropic/claude-code). Jika editor Anda tidak dapat menginstal ekstensi, [instal CLI](/docs/id/quickstart) dan jalankan `claude` di terminal terintegrasi-nya. CLI berfungsi di terminal apa pun.

<Note>Jika ekstensi tidak muncul setelah instalasi, restart VS Code atau jalankan "Developer: Reload Window" dari Command Palette.</Note>

<h2 id="get-started">
  Memulai
</h2>

Setelah diinstal, Anda dapat mulai menggunakan Claude Code melalui antarmuka VS Code:

<Steps>
  <Step title="Buka panel Claude Code">
    Di seluruh VS Code, ikon Spark menunjukkan Claude Code: <img src="https://mintcdn.com/claude-code/c5r9_6tjPMzFdDDT/images/vs-code-spark-icon.svg?fit=max&auto=format&n=c5r9_6tjPMzFdDDT&q=85&s=3ca45e00deadec8c8f4b4f807da94505" alt="Spark icon" style={{display: "inline", height: "0.85em", verticalAlign: "middle"}} width="16" height="16" data-path="images/vs-code-spark-icon.svg" />

    Cara tercepat untuk membuka Claude adalah dengan mengklik ikon Spark di **Editor Toolbar** (sudut kanan atas editor). Ikon hanya muncul saat Anda memiliki file terbuka.

    <img src="https://mintcdn.com/claude-code/mfM-EyoZGnQv8JTc/images/vs-code-editor-icon.png?fit=max&auto=format&n=mfM-EyoZGnQv8JTc&q=85&s=eb4540325d94664c51776dbbfec4cf02" alt="Editor VS Code menampilkan ikon Spark di Editor Toolbar" width="2796" height="734" data-path="images/vs-code-editor-icon.png" />

    Cara lain untuk membuka Claude Code:

    * **Activity Bar**: klik ikon Spark di sidebar kiri untuk membuka daftar sesi. Klik sesi apa pun untuk membukanya sebagai tab editor penuh, atau mulai yang baru. Ikon ini selalu terlihat di Activity Bar.
    * **Command Palette**: `Cmd+Shift+P` (Mac) atau `Ctrl+Shift+P` (Windows/Linux), ketik "Claude Code", dan pilih opsi seperti "Open in New Tab"
    * **Status Bar**: klik **✱ Claude Code** di sudut kanan bawah jendela. Ini berfungsi bahkan saat tidak ada file yang terbuka.

    Anda dapat menyeret panel Claude untuk memposisikan ulang di mana saja di VS Code. Lihat [Sesuaikan alur kerja Anda](#customize-your-workflow) untuk detail.
  </Step>

  <Step title="Masuk">
    Saat pertama kali Anda membuka panel, layar masuk muncul. Klik **Sign in** dan selesaikan otorisasi di browser Anda.

    Jika Anda melihat **Not logged in · Please run /login** nanti, ekstensi membuka kembali layar masuk secara otomatis. Jika tidak muncul, muat ulang jendela dari Command Palette dengan **Developer: Reload Window**.

    Jika Anda memiliki `ANTHROPIC_API_KEY` yang diatur di shell Anda tetapi masih melihat prompt masuk, VS Code mungkin tidak mewarisi lingkungan shell Anda. Luncurkan VS Code dari terminal dengan `code .` sehingga mewarisi variabel lingkungan Anda, atau masuk dengan akun Claude Anda sebagai gantinya.

    Setelah Anda masuk, daftar periksa **Learn Claude Code** muncul. Kerjakan setiap item dengan mengklik **Show me**, atau tutup dengan X. Untuk membukanya kembali nanti, hapus centang **Hide Onboarding** di pengaturan VS Code di bawah Extensions → Claude Code.
  </Step>

  <Step title="Kirim prompt">
    Minta Claude untuk membantu dengan kode atau file Anda, baik itu menjelaskan cara kerja sesuatu, men-debug masalah, atau membuat perubahan.

    <Tip>Claude secara otomatis melihat teks pilihan Anda. Tekan `Option+K` (Mac) / `Alt+K` (Windows/Linux) untuk juga menyisipkan referensi @-mention (seperti `@file.ts#5-10`) ke dalam prompt Anda.</Tip>

    Berikut adalah contoh menanyakan tentang baris tertentu dalam file:

    <img src="https://mintcdn.com/claude-code/FVYz38sRY-VuoGHA/images/vs-code-send-prompt.png?fit=max&auto=format&n=FVYz38sRY-VuoGHA&q=85&s=ede3ed8d8d5f940e01c5de636d009cfd" alt="Editor VS Code dengan baris 2-3 dipilih dalam file Python, dan panel Claude Code menampilkan pertanyaan tentang baris tersebut dengan referensi @-mention" width="3288" height="1876" data-path="images/vs-code-send-prompt.png" />
  </Step>

  <Step title="Tinjau perubahan">
    Saat Claude ingin mengedit file, ia menampilkan perbandingan berdampingan dari perubahan asli dan yang diusulkan, kemudian meminta izin. Anda dapat menerima, menolak, atau memberi tahu Claude apa yang harus dilakukan sebagai gantinya. Jika Anda mengedit konten yang diusulkan secara langsung di tampilan diff sebelum menerima, Claude diberitahu bahwa Anda memodifikasinya sehingga tidak menganggap file cocok dengan proposal aslinya.

    <img src="https://mintcdn.com/claude-code/FVYz38sRY-VuoGHA/images/vs-code-edits.png?fit=max&auto=format&n=FVYz38sRY-VuoGHA&q=85&s=e005f9b41c541c5c7c59c082f7c4841c" alt="VS Code menampilkan diff dari perubahan yang diusulkan Claude dengan prompt izin menanyakan apakah akan membuat edit" width="3292" height="1876" data-path="images/vs-code-edits.png" />
  </Step>
</Steps>

Untuk lebih banyak ide tentang apa yang dapat Anda lakukan dengan Claude Code, lihat [Alur kerja umum](/docs/id/common-workflows).

<Tip>
  Jalankan "Claude Code: Open Walkthrough" dari Command Palette untuk tur terpandu tentang dasar-dasarnya.
</Tip>

<h2 id="use-the-prompt-box">
  Gunakan kotak prompt
</h2>

Kotak prompt mendukung beberapa fitur:

* **Mode izin**: klik indikator mode di bagian bawah kotak prompt untuk beralih mode, atau atur default di pengaturan VS Code di bawah `claudeCode.initialPermissionMode`. Lihat [mode izin](/docs/id/permission-modes#switch-permission-modes) untuk setiap mode yang ditawarkan indikator.
  * **Manual**: Claude meminta izin sebelum edit file dan sebagian besar perintah shell.
  * **Plan**: Claude menjelaskan apa yang akan dilakukan dan menunggu persetujuan sebelum membuat perubahan. VS Code secara otomatis membuka rencana sebagai dokumen Markdown penuh di mana Anda dapat menambahkan komentar inline untuk memberikan umpan balik sebelum Claude mulai.
  * **Edit automatically**: Claude membuat edit tanpa bertanya.
* **Command menu**: klik `/` atau ketik `/` untuk membuka menu perintah. Opsi termasuk melampirkan file, beralih model, mengalihkan extended thinking, melihat penggunaan rencana (`/usage`), dan memulai sesi [Remote Control](/docs/id/remote-control) (`/remote-control`). Bagian Customize menyediakan akses ke MCP servers, hooks, memory, permissions, dan plugins. Item dengan ikon terminal terbuka di terminal terintegrasi.
  * Bagian Settings mencakup **Enable Remote Control for all sessions**, yang menetapkan [`remoteControlAtStartup`](/docs/id/settings#available-settings) sehingga [setiap sesi interaktif baru terhubung ke Remote Control secara otomatis](/docs/id/remote-control#enable-remote-control-for-all-sessions). Memerlukan Claude Code v2.1.203 atau lebih baru.
* **Context indicator**: kotak prompt menunjukkan berapa banyak context window Claude yang Anda gunakan. Claude secara otomatis melakukan compact saat diperlukan, atau Anda dapat menjalankan `/compact` secara manual.
* **Extended thinking**: memungkinkan Claude menghabiskan lebih banyak waktu untuk bernalar melalui masalah kompleks. Alihkan melalui menu perintah (`/`). Penalaran Claude muncul dalam percakapan sebagai blok yang dilipat: klik blok untuk membacanya, atau tekan `Ctrl+O` untuk memperluas atau melipat setiap blok thinking dalam sesi. Lihat [Extended thinking](/docs/id/model-config#extended-thinking) untuk detail.
* **Multi-line input**: tekan `Shift+Enter` untuk menambahkan baris baru tanpa mengirim. Ini juga berfungsi di input teks bebas "Other" dari dialog pertanyaan.

<h3 id="reference-files-and-folders">
  Reference files and folders
</h3>

Gunakan @-mentions untuk memberikan Claude konteks tentang file atau folder tertentu. Saat Anda mengetik `@` diikuti dengan nama file atau folder, Claude membaca konten tersebut dan dapat menjawab pertanyaan tentangnya atau membuat perubahan padanya. Claude Code mendukung fuzzy matching, jadi Anda dapat mengetik nama parsial untuk menemukan apa yang Anda butuhkan:

```text theme={null}
> Explain the logic in @auth (fuzzy matches auth.js, AuthService.ts, etc.)
> What's in @src/components/ (include a trailing slash for folders)
```

Untuk PDF besar, Anda dapat meminta Claude membaca halaman tertentu alih-alih seluruh file: satu halaman, rentang seperti halaman 1-10, atau rentang terbuka seperti halaman 3 ke depan.

Saat Anda memilih teks di editor, Claude dapat melihat kode yang disorot secara otomatis. Footer kotak prompt menunjukkan berapa banyak baris yang dipilih. Tekan `Option+K` (Mac) / `Alt+K` (Windows/Linux) untuk menyisipkan @-mention dengan jalur file dan nomor baris (misalnya, `@app.ts#5-10`). Klik indikator pilihan untuk mengalihkan apakah Claude dapat melihat teks yang disorot Anda - ikon eye-slash berarti pilihan tersembunyi dari Claude.

Anda juga dapat menahan `Shift` sambil menyeret file ke kotak prompt untuk menambahkannya sebagai lampiran. Klik X pada lampiran apa pun untuk menghapusnya dari konteks.

<h3 id="resume-past-conversations">
  Resume past conversations
</h3>

Klik tombol **Session history** di bagian atas panel Claude Code untuk mengakses riwayat percakapan Anda. Anda dapat mencari berdasarkan kata kunci atau menelusuri berdasarkan waktu (Today, Yesterday, Last 7 days, dll.). Klik percakapan apa pun untuk melanjutkannya dengan riwayat pesan lengkap. Sesi baru menerima judul yang dihasilkan AI berdasarkan pesan pertama Anda. Arahkan kursor ke sesi untuk mengungkapkan tindakan rename dan remove: rename untuk memberikan judul deskriptif, atau remove untuk menghapusnya dari daftar. Untuk lebih lanjut tentang melanjutkan sesi, lihat [Manage sessions](/docs/id/sessions).

<h3 id="resume-cloud-sessions-from-claude-ai">
  Resume cloud sessions from Claude.ai
</h3>

Jika Anda menggunakan [Claude Code on the web](/docs/id/claude-code-on-the-web), Anda dapat melanjutkan sesi jarak jauh tersebut langsung di VS Code. Ini memerlukan masuk dengan **Claude.ai Subscription**, bukan Anthropic Console.

<Steps>
  <Step title="Open session history">
    Klik tombol **Session history** di bagian atas panel Claude Code.
  </Step>

  <Step title="Select the Remote tab">
    Dialog menampilkan dua tab: Local dan Remote. Klik **Remote** untuk melihat sesi dari claude.ai.
  </Step>

  <Step title="Select a session to resume">
    Telusuri atau cari sesi jarak jauh Anda. Klik sesi apa pun untuk mengunduhnya dan melanjutkan percakapan secara lokal.
  </Step>
</Steps>

<Note>
  Hanya sesi web yang dimulai dengan repositori GitHub yang muncul di tab Remote. Melanjutkan memuat riwayat percakapan secara lokal; perubahan tidak disinkronkan kembali ke claude.ai.
</Note>

<h3 id="check-account-and-usage">
  Check account and usage
</h3>

Jalankan `/usage` dari menu perintah untuk membuka dialog Account & usage. Dialog ini menampilkan akun yang Anda masuki, paket, dan batang penggunaan untuk sesi saat ini dan minggu ini dengan berapa lama hingga setiap batas direset.

Dialog juga merinci apa yang berkontribusi pada batas paket Anda. Dialog ini menandai perilaku yang menyumbang 10% atau lebih dari penggunaan terbaru, seperti cache misses, konteks panjang, dan sesi yang berat subagent atau sangat paralel, masing-masing dengan tip untuk menguranginya. Tabel atribusi menunjukkan berapa banyak penggunaan yang berasal dari setiap skill, subagent, plugin, dan MCP server. Memerlukan Claude Code v2.1.174 atau lebih baru.

Gunakan toggle Day dan Week untuk beralih antara 24 jam terakhir dan 7 hari terakhir. Angka-angka tersebut perkiraan dan dihitung dari sesi lokal di mesin ini, jadi penggunaan dari perangkat lain atau claude.ai tidak disertakan. Untuk lebih lanjut tentang pelacakan dan pengurangan penggunaan, lihat [Track your costs](/docs/id/costs#track-your-costs).

<h2 id="customize-your-workflow">
  Sesuaikan alur kerja Anda
</h2>

Setelah Anda siap dan berjalan, Anda dapat memposisikan ulang panel Claude, menjalankan beberapa sesi, atau beralih ke mode terminal.

<h3 id="choose-where-claude-lives">
  Pilih di mana Claude berada
</h3>

Anda dapat menyeret panel Claude untuk memposisikan ulang di mana saja di VS Code. Ambil tab atau title bar panel dan seret ke:

* **Secondary sidebar**: sisi kanan jendela. Membuat Claude tetap terlihat saat Anda coding.
* **Primary sidebar**: sidebar kiri dengan ikon untuk Explorer, Search, dll.
* **Editor area**: membuka Claude sebagai tab bersama file Anda. Berguna untuk tugas sampingan.

<Tip>
  Gunakan sidebar untuk sesi Claude utama Anda dan buka tab tambahan untuk tugas sampingan. Claude mengingat lokasi pilihan Anda. Ikon daftar sesi Activity Bar terpisah dari panel Claude: daftar sesi selalu terlihat di Activity Bar, sementara ikon panel Claude hanya muncul di sana saat panel ditambatkan ke sidebar kiri.
</Tip>

<h3 id="run-multiple-conversations">
  Jalankan beberapa percakapan
</h3>

Gunakan **Open in New Tab** atau **Open in New Window** dari Command Palette untuk memulai percakapan tambahan. Setiap percakapan mempertahankan riwayat dan konteksnya sendiri, memungkinkan Anda bekerja pada tugas berbeda secara paralel.

Saat menggunakan tab, titik berwarna kecil pada ikon spark menunjukkan status: biru berarti permintaan izin tertunda, oranye berarti Claude selesai saat tab tersembunyi.

<h3 id="switch-to-terminal-mode">
  Beralih ke mode terminal
</h3>

Secara default, ekstensi membuka panel chat grafis. Jika Anda lebih suka antarmuka gaya CLI, buka [pengaturan Use Terminal](vscode://settings/claudeCode.useTerminal) dan centang kotak.

Anda juga dapat membuka pengaturan VS Code (`Cmd+,` di Mac atau `Ctrl+,` di Windows/Linux), buka Extensions → Claude Code, dan centang **Use Terminal**.

<h2 id="manage-plugins">
  Kelola plugins
</h2>

Ekstensi VS Code mencakup antarmuka grafis untuk menginstal dan mengelola [plugins](/docs/id/plugins). Ketik `/plugins` di kotak prompt untuk membuka antarmuka **Manage plugins**.

<h3 id="install-plugins">
  Instal plugins
</h3>

Dialog plugin menampilkan dua tab: **Plugins** dan **Marketplaces**.

Di tab Plugins:

* **Installed plugins** muncul di bagian atas dengan switch toggle untuk mengaktifkan atau menonaktifkannya
* **Available plugins** dari marketplace yang dikonfigurasi muncul di bawah
* Cari untuk memfilter plugins berdasarkan nama atau deskripsi
* Klik **Install** pada plugin yang tersedia apa pun

Saat Anda menginstal plugin, pilih cakupan instalasi:

* **Install for you**: tersedia di semua proyek Anda (user scope)
* **Install for this project**: dibagikan dengan kolaborator proyek (project scope)
* **Install locally**: hanya untuk Anda, hanya di repositori ini (local scope)

<h3 id="manage-marketplaces">
  Kelola marketplaces
</h3>

Beralih ke tab **Marketplaces** untuk menambah atau menghapus sumber plugin:

* Masukkan repo GitHub, URL, atau jalur lokal untuk menambahkan marketplace baru
* Klik ikon refresh untuk memperbarui daftar plugin marketplace
* Klik ikon trash untuk menghapus marketplace

Setelah membuat perubahan, banner meminta Anda untuk restart Claude Code untuk menerapkan pembaruan.

<Note>
  Manajemen plugin di VS Code menggunakan perintah CLI yang sama di balik layar. Plugins dan marketplaces yang Anda konfigurasi di ekstensi juga tersedia di CLI, dan sebaliknya.
</Note>

Untuk lebih lanjut tentang sistem plugin, lihat [Plugins](/docs/id/plugins) dan [Plugin marketplaces](/docs/id/plugin-marketplaces).

<h2 id="automate-browser-tasks-with-chrome">
  Otomatisasi tugas browser dengan Chrome
</h2>

Hubungkan Claude ke browser Chrome Anda untuk menguji aplikasi web, debug dengan console logs, dan otomatisasi alur kerja browser tanpa meninggalkan VS Code. Ini memerlukan ekstensi [Claude in Chrome](https://chromewebstore.google.com/detail/claude/fcoeoabgfenejglbffodgkkbkcdhcgfn) versi 1.0.36 atau lebih tinggi.

Ketik `@browser` di kotak prompt diikuti dengan apa yang ingin Anda lakukan Claude:

```text theme={null}
@browser go to localhost:3000 and check the console for errors
```

Anda juga dapat membuka menu lampiran untuk memilih alat browser tertentu seperti membuka tab baru atau membaca konten halaman.

Claude membuka tab baru untuk tugas browser dan berbagi status login browser Anda, sehingga dapat mengakses situs apa pun yang sudah Anda masuki.

Untuk instruksi setup, daftar lengkap kemampuan, dan troubleshooting, lihat [Gunakan Claude Code dengan Chrome](/docs/id/chrome).

<h2 id="vs-code-commands-and-shortcuts">
  Perintah dan pintasan VS Code
</h2>

Buka Command Palette (`Cmd+Shift+P` di Mac atau `Ctrl+Shift+P` di Windows/Linux) dan ketik "Claude Code" untuk melihat semua perintah VS Code yang tersedia untuk ekstensi Claude Code.

Beberapa pintasan tergantung pada panel mana yang "focused" (menerima input keyboard). Saat kursor Anda berada di file kode, editor difokuskan. Saat kursor Anda berada di kotak prompt Claude, Claude difokuskan. Gunakan `Cmd+Esc` / `Ctrl+Esc` untuk beralih di antara keduanya.

<Note>
  Ini adalah perintah VS Code untuk mengontrol ekstensi. Tidak semua perintah Claude Code bawaan tersedia di ekstensi. Lihat [Ekstensi VS Code vs. Claude Code CLI](#vs-code-extension-vs-claude-code-cli) untuk detail.
</Note>

| Perintah                   | Pintasan                                                 | Deskripsi                                                                                                                                                                                                                |
| -------------------------- | -------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Focus Input                | `Cmd+Esc` (Mac) / `Ctrl+Esc` (Windows/Linux)             | Alihkan fokus antara editor dan Claude                                                                                                                                                                                   |
| Open in Side Bar           | -                                                        | Buka Claude di sidebar kiri                                                                                                                                                                                              |
| Open in Terminal           | -                                                        | Buka Claude dalam mode terminal                                                                                                                                                                                          |
| Open in New Tab            | `Cmd+Shift+Esc` (Mac) / `Ctrl+Shift+Esc` (Windows/Linux) | Buka percakapan baru sebagai tab editor                                                                                                                                                                                  |
| Open in New Window         | -                                                        | Buka percakapan baru di jendela terpisah                                                                                                                                                                                 |
| New Conversation           | `Cmd+N` (Mac) / `Ctrl+N` (Windows/Linux)                 | Mulai percakapan baru. Memerlukan Claude difokuskan dan `enableNewConversationShortcut` diatur ke `true`                                                                                                                 |
| Reopen Closed Session      | `Cmd+Shift+T` (Mac) / `Ctrl+Shift+T` (Windows/Linux)     | Buka kembali tab sesi Claude yang paling baru ditutup. Jatuh kembali ke pembukaan ulang editor normal VS Code ketika tab terakhir yang ditutup bukan sesi Claude. Nonaktifkan dengan `enableReopenClosedSessionShortcut` |
| Insert @-Mention Reference | `Option+K` (Mac) / `Alt+K` (Windows/Linux)               | Sisipkan referensi ke file saat ini dan pilihan (memerlukan editor difokuskan)                                                                                                                                           |
| Show Logs                  | -                                                        | Lihat log debug ekstensi                                                                                                                                                                                                 |
| Logout                     | -                                                        | Keluar dari akun Anthropic Anda                                                                                                                                                                                          |

<h3 id="launch-a-vs-code-tab-from-other-tools">
  Luncurkan tab VS Code dari alat lain
</h3>

Ekstensi mendaftarkan URI handler di `vscode://anthropic.claude-code/open`. Gunakan untuk membuka tab Claude Code baru dari tooling Anda sendiri: alias shell, bookmarklet browser, atau script apa pun yang dapat membuka URL. Jika VS Code belum berjalan, membuka URL meluncurkannya terlebih dahulu. Jika VS Code sudah berjalan, URL terbuka di jendela mana pun yang saat ini difokuskan.

Panggil handler dengan pembuka URL sistem operasi Anda.

<Tabs>
  <Tab title="macOS">
    ```bash theme={null}
    open "vscode://anthropic.claude-code/open"
    ```
  </Tab>

  <Tab title="Linux">
    ```bash theme={null}
    xdg-open "vscode://anthropic.claude-code/open"
    ```
  </Tab>

  <Tab title="Windows">
    Di PowerShell:

    ```powershell theme={null}
    Start-Process "vscode://anthropic.claude-code/open"
    ```

    Di `cmd.exe`, `start` memperlakukan argumen pertama yang dikutip sebagai judul jendela, jadi berikan judul kosong sebelum URL:

    ```cmd theme={null}
    start "" "vscode://anthropic.claude-code/open"
    ```
  </Tab>
</Tabs>

Handler menerima dua parameter query opsional:

| Parameter | Deskripsi                                                                                                                                                                                                                                                                                                                                                                |
| --------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `prompt`  | Teks untuk pre-fill di kotak prompt. Harus URL-encoded. Prompt di-pre-fill tetapi tidak dikirim secara otomatis.                                                                                                                                                                                                                                                         |
| `session` | ID sesi untuk dilanjutkan alih-alih memulai percakapan baru. Sesi harus milik workspace yang saat ini terbuka di VS Code. Jika sesi tidak ditemukan, percakapan segar dimulai sebagai gantinya. Jika sesi sudah terbuka di tab, tab tersebut difokuskan. Untuk menangkap ID sesi secara terprogram, lihat [Continue conversations](/docs/id/headless#continue-conversations). |

Misalnya, untuk membuka tab yang di-pre-fill dengan "review my changes":

```text theme={null}
vscode://anthropic.claude-code/open?prompt=review%20my%20changes
```

Untuk meluncurkan sesi terminal alih-alih tab VS Code, gunakan handler `claude-cli://` CLI. Lihat [Launch sessions from links](/docs/id/deep-links).

<h2 id="configure-settings">
  Konfigurasi pengaturan
</h2>

Ekstensi memiliki dua jenis pengaturan:

* **Extension settings** di VS Code: mengontrol perilaku ekstensi dalam VS Code. Buka dengan `Cmd+,` (Mac) atau `Ctrl+,` (Windows/Linux), kemudian buka Extensions → Claude Code. Anda juga dapat mengetik `/` dan memilih **General Config** untuk membuka pengaturan.
* **Claude Code settings** di `~/.claude/settings.json`: dibagikan antara ekstensi dan CLI. Gunakan untuk perintah yang diizinkan, variabel lingkungan, hooks, dan MCP servers. Lihat [Settings](/docs/id/settings) untuk detail.

<Tip>
  Tambahkan `"$schema": "https://json.schemastore.org/claude-code-settings.json"` ke `settings.json` Anda untuk mendapatkan autocomplete dan validasi inline untuk semua pengaturan yang tersedia langsung di VS Code.
</Tip>

<h3 id="extension-settings">
  Pengaturan ekstensi
</h3>

| Pengaturan                          | Default   | Deskripsi                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| ----------------------------------- | --------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `useTerminal`                       | `false`   | Luncurkan Claude dalam mode terminal alih-alih panel grafis                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `initialPermissionMode`             | `default` | Mengontrol prompt persetujuan untuk percakapan baru: `default`, `plan`, `acceptEdits`, atau `bypassPermissions`. `manual` adalah alias untuk `default` dan memilih mode yang berlabel **Manual** dalam indikator mode. Memerlukan Claude Code v2.1.200 atau lebih baru. Lihat [permission modes](/docs/id/permission-modes).                                                                                                                                                       |
| `preferredLocation`                 | `panel`   | Di mana Claude terbuka: `sidebar` (kanan) atau `panel` (tab baru)                                                                                                                                                                                                                                                                                                                                                                                                             |
| `autosave`                          | `true`    | Auto-save file sebelum Claude membaca atau menulisnya                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `useCtrlEnterToSend`                | `false`   | Gunakan Ctrl/Cmd+Enter alih-alih Enter untuk mengirim prompt                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `enableNewConversationShortcut`     | `false`   | Aktifkan Cmd/Ctrl+N untuk memulai percakapan baru                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `enableReopenClosedSessionShortcut` | `true`    | Gunakan Cmd/Ctrl+Shift+T untuk membuka kembali tab sesi Claude yang paling baru ditutup. Ketika tab terakhir yang ditutup bukan sesi Claude, pintasan keyboard menjalankan perintah reopen-closed-editor normal VS Code sebagai gantinya.                                                                                                                                                                                                                                     |
| `hideOnboarding`                    | `false`   | Sembunyikan daftar periksa onboarding (ikon graduation cap)                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `respectGitIgnore`                  | `true`    | Kecualikan pola .gitignore dari pencarian file                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `usePythonEnvironment`              | `true`    | Aktifkan lingkungan Python workspace saat menjalankan Claude. Memerlukan ekstensi Python.                                                                                                                                                                                                                                                                                                                                                                                     |
| `environmentVariables`              | `[]`      | Atur variabel lingkungan untuk proses Claude. Gunakan pengaturan Claude Code sebagai gantinya untuk konfigurasi bersama.                                                                                                                                                                                                                                                                                                                                                      |
| `disableLoginPrompt`                | `false`   | Lewati prompt autentikasi (untuk setup penyedia pihak ketiga)                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `allowDangerouslySkipPermissions`   | `false`   | Menambahkan Bypass permissions ke pemilih mode. Gunakan hanya di sandbox tanpa akses internet.                                                                                                                                                                                                                                                                                                                                                                                |
| `claudeProcessWrapper`              | -         | Executable yang digunakan untuk meluncurkan proses Claude. Jalur binary bundel dilewatkan sebagai argumen saat ada. Atur ini ke binary `claude` yang diinstal secara terpisah jika build ekstensi tidak menyertakan satu untuk platform Anda. Error "Unsupported platform" saat aktivasi berarti tidak ada binary yang dibundel untuk platform Anda; lihat [platform mana yang memiliki binary prebuilt](/docs/id/troubleshoot-install#native-binary-not-found-after-npm-install). |

<h2 id="vs-code-extension-vs-claude-code-cli">
  Ekstensi VS Code vs. Claude Code CLI
</h2>

Claude Code tersedia sebagai ekstensi VS Code (panel grafis) dan CLI (command-line interface di terminal). Beberapa fitur hanya tersedia di CLI. Jika Anda memerlukan fitur khusus CLI, jalankan `claude` di terminal terintegrasi VS Code. Ini memerlukan [instalasi CLI standalone](/docs/id/setup): ekstensi tidak menambahkan `claude` ke PATH Anda. Lihat [Jalankan CLI di VS Code](#run-cli-in-vs-code).

| Fitur                  | CLI                   | Ekstensi VS Code                                                                           |
| ---------------------- | --------------------- | ------------------------------------------------------------------------------------------ |
| Perintah dan skills    | [Semua](/docs/id/commands) | Subset (ketik `/` untuk melihat yang tersedia)                                             |
| Konfigurasi MCP server | Ya                    | Parsial (tambahkan server melalui CLI; kelola server yang ada dengan `/mcp` di panel chat) |
| Checkpoints            | Ya                    | Ya                                                                                         |
| Pintasan bash `!`      | Ya                    | Tidak                                                                                      |
| Tab completion         | Ya                    | Tidak                                                                                      |

<h3 id="rewind-with-checkpoints">
  Rewind dengan checkpoints
</h3>

Ekstensi VS Code mendukung checkpoints, yang melacak edit file Claude dan memungkinkan Anda untuk rewind ke status sebelumnya. Arahkan kursor ke pesan apa pun untuk mengungkapkan tombol rewind, kemudian pilih dari tiga opsi:

* **Fork conversation from here**: mulai cabang percakapan baru dari pesan ini sambil menjaga semua perubahan kode tetap utuh
* **Rewind code to here**: kembalikan perubahan file ke titik ini dalam percakapan sambil menjaga riwayat percakapan lengkap
* **Fork conversation and rewind code**: mulai cabang percakapan baru dan kembalikan perubahan file ke titik ini

Untuk detail lengkap tentang cara kerja checkpoints dan keterbatasannya, lihat [Checkpointing](/docs/id/checkpointing).

<h3 id="run-cli-in-vs-code">
  Jalankan CLI di VS Code
</h3>

Untuk menggunakan CLI sambil tetap berada di VS Code, buka terminal terintegrasi (`` Ctrl+` `` di Windows/Linux atau `` Cmd+` `` di Mac) dan jalankan `claude`. CLI secara otomatis terintegrasi dengan IDE Anda untuk fitur seperti tampilan diff dan berbagi diagnostik.

Menginstal ekstensi tidak menempatkan `claude` di PATH shell Anda. Ekstensi menggabungkan salinan pribadi CLI untuk panel chatnya, tetapi mengetik `claude` di terminal memerlukan [instalasi CLI standalone](/docs/id/setup). Jalankan instalasi sekali dan perintah di halaman ini, termasuk `claude mcp add` dan `claude --resume`, bekerja di terminal apa pun. Jika `claude` masih tidak ditemukan setelah menginstal, [verifikasi PATH Anda](/docs/id/troubleshoot-install#verify-your-path).

Jika menggunakan terminal eksternal, jalankan `/ide` di dalam Claude Code untuk menghubungkannya ke VS Code.

<h3 id="switch-between-extension-and-cli">
  Beralih antara ekstensi dan CLI
</h3>

Ekstensi dan CLI berbagi riwayat percakapan yang sama. Untuk melanjutkan percakapan ekstensi di CLI, jalankan `claude --resume` di terminal. Ini membuka picker interaktif di mana Anda dapat mencari dan memilih percakapan Anda.

<h3 id="include-terminal-output-in-prompts">
  Sertakan output terminal dalam prompt
</h3>

Referensikan output terminal dalam prompt Anda menggunakan `@terminal:name` di mana `name` adalah judul terminal. Ini memungkinkan Claude melihat output perintah, pesan kesalahan, atau log tanpa copy-paste.

<h3 id="monitor-background-processes">
  Pantau proses latar belakang
</h3>

Saat Claude menjalankan perintah yang berjalan lama, ekstensi menampilkan kemajuan di status bar. Namun, visibilitas untuk tugas latar belakang terbatas dibandingkan dengan CLI. Untuk visibilitas yang lebih baik, minta Claude menampilkan perintah sehingga Anda dapat menjalankannya di terminal terintegrasi VS Code.

<h3 id="connect-to-external-tools-with-mcp">
  Hubungkan ke alat eksternal dengan MCP
</h3>

MCP (Model Context Protocol) servers memberikan Claude akses ke alat eksternal, database, dan API.

Untuk menambahkan MCP server, buka terminal terintegrasi (`` Ctrl+` `` atau `` Cmd+` ``) dan jalankan `claude mcp add`. Contoh di bawah ini menambahkan MCP server jarak jauh GitHub, yang melakukan autentikasi dengan [personal access token](https://github.com/settings/personal-access-tokens) yang diteruskan sebagai header:

```bash theme={null}
claude mcp add --transport http github https://api.githubcopilot.com/mcp/ \
  --header "Authorization: Bearer YOUR_GITHUB_PAT"
```

Setelah dikonfigurasi, minta Claude untuk menggunakan alat (misalnya, "Review PR #456").

Untuk mengelola MCP servers tanpa meninggalkan VS Code, ketik `/mcp` di panel chat. Dialog manajemen MCP memungkinkan Anda mengaktifkan atau menonaktifkan server, reconnect ke server, dan mengelola autentikasi OAuth. Lihat [dokumentasi MCP](/docs/id/mcp) untuk server yang tersedia.

<h2 id="work-with-git">
  Bekerja dengan git
</h2>

Claude Code terintegrasi dengan git untuk membantu dengan alur kerja kontrol versi langsung di VS Code. Minta Claude untuk commit perubahan, membuat pull request, atau bekerja di seluruh branch.

<h3 id="create-commits-and-pull-requests">
  Buat commit dan pull request
</h3>

Claude dapat stage perubahan, menulis pesan commit, dan membuat pull request berdasarkan pekerjaan Anda:

```text theme={null}
> commit my changes with a descriptive message
> create a pr for this feature
> summarize the changes I've made to the auth module
```

Saat membuat pull request, Claude menghasilkan deskripsi berdasarkan perubahan kode aktual dan dapat menambahkan konteks tentang pengujian atau keputusan implementasi.

<h3 id="use-git-worktrees-for-parallel-tasks">
  Gunakan git worktrees untuk tugas paralel
</h3>

Gunakan flag `--worktree` (`-w`) untuk memulai Claude di worktree terisolasi dengan file dan branch-nya sendiri:

```bash theme={null}
claude --worktree feature-auth
```

Setiap worktree mempertahankan status file independen sambil berbagi riwayat git. Ini mencegah instance Claude saling mengganggu saat bekerja pada tugas berbeda. Untuk detail lebih lanjut, lihat [Jalankan sesi paralel dengan Git worktrees](/docs/id/worktrees).

<h2 id="use-third-party-providers">
  Gunakan penyedia pihak ketiga
</h2>

Secara default, Claude Code terhubung langsung ke API Anthropic. Jika organisasi Anda menggunakan Amazon Bedrock, Google Cloud's Agent Platform, atau Microsoft Foundry untuk mengakses Claude, konfigurasi ekstensi untuk menggunakan penyedia Anda sebagai gantinya:

<Steps>
  <Step title="Nonaktifkan prompt login">
    Buka pengaturan [Disable Login Prompt](vscode://settings/claudeCode.disableLoginPrompt) dan centang kotak.

    Anda juga dapat membuka pengaturan VS Code (`Cmd+,` di Mac atau `Ctrl+,` di Windows/Linux), cari "Claude Code login", dan centang **Disable Login Prompt**.
  </Step>

  <Step title="Konfigurasi penyedia Anda">
    Ikuti panduan setup untuk penyedia Anda:

    * [Claude Code di Amazon Bedrock](/docs/id/amazon-bedrock)
    * [Claude Code di Google Cloud's Agent Platform](/docs/id/google-vertex-ai)
    * [Claude Code di Microsoft Foundry](/docs/id/microsoft-foundry)

    Panduan ini mencakup konfigurasi penyedia Anda di `~/.claude/settings.json`, yang memastikan pengaturan Anda dibagikan antara ekstensi VS Code dan CLI.
  </Step>
</Steps>

<h2 id="security-and-privacy">
  Keamanan dan privasi
</h2>

Kode Anda tetap pribadi. Claude Code memproses kode Anda untuk memberikan bantuan tetapi tidak menggunakannya untuk melatih model. Untuk detail tentang penanganan data dan cara opt out dari logging, lihat [Data and privacy](/docs/id/data-usage).

Dengan izin auto-edit diaktifkan, Claude Code dapat memodifikasi file konfigurasi VS Code (seperti `settings.json` atau `tasks.json`) yang mungkin dijalankan VS Code secara otomatis. Untuk mengurangi risiko saat bekerja dengan kode yang tidak dipercaya:

* Aktifkan [VS Code Restricted Mode](https://code.visualstudio.com/docs/editor/workspace-trust#_restricted-mode) untuk workspace yang tidak dipercaya
* Gunakan mode persetujuan manual alih-alih auto-accept untuk edit
* Tinjau perubahan dengan hati-hati sebelum menerimanya

<h3 id="the-built-in-ide-mcp-server">
  Server MCP IDE bawaan
</h3>

Saat ekstensi aktif, ia menjalankan server MCP lokal yang terhubung oleh CLI secara otomatis. Ini adalah cara CLI membuka diff di viewer diff asli VS Code, membaca pilihan saat ini Anda untuk `@`-mentions, dan — saat Anda bekerja di notebook Jupyter — meminta VS Code untuk menjalankan sel.

Server bernama `ide` dan tersembunyi dari `/mcp` karena tidak ada yang perlu dikonfigurasi. Namun, jika organisasi Anda menggunakan hook `PreToolUse` untuk allowlist alat MCP, Anda perlu mengetahui bahwa itu ada.

**Konteks seleksi dan file terbuka.** Saat terhubung, CLI menyertakan pilihan editor saat ini Anda dan jalur file aktif sebagai konteks pada setiap prompt yang Anda kirim. Transkrip menunjukkan baris `⧉ Selected N lines from <file>` saat ini terjadi. Untuk mengecualikan file sensitif seperti `.env`, tambahkan [aturan deny `Read`](/docs/id/permissions#read-and-edit) untuk jalurnya. Aturan deny yang cocok mencegah baik teks yang dipilih maupun pemberitahuan file terbuka untuk file tersebut dari mencapai Claude.

**Transport dan autentikasi.** Server mengikat ke `127.0.0.1` pada port tinggi acak dalam rentang 10000–65535, dan port tidak dapat dikonfigurasi. Transport adalah `ws://` yang tidak terenkripsi; karena soket hanya loopback, proses apa pun yang dapat menangkap lalu lintas juga dapat membaca token dari file kunci, jadi TLS tidak akan menambah perlindungan. Setiap aktivasi ekstensi menghasilkan token auth acak segar, menulisnya ke file kunci di `~/.claude/ide/<port>.lock`, dan CLI harus menyajikannya sebagai header `X-Claude-Code-Ide-Authorization` untuk terhubung. File kunci memiliki izin `0600` di direktori `0700`, jadi hanya pengguna yang menjalankan VS Code yang dapat membacanya. Jika `CLAUDE_CONFIG_DIR` diatur, file kunci ditulis ke `$CLAUDE_CONFIG_DIR/ide/` sebagai gantinya.

**Alat yang diekspos ke model.** Server menampilkan selusin alat, tetapi hanya dua yang terlihat oleh model. Sisanya adalah RPC internal yang digunakan CLI untuk UI-nya sendiri — membuka diff, membaca pilihan, menyimpan file — dan disaring sebelum daftar alat mencapai Claude.

| Nama alat (seperti yang terlihat oleh hooks) | Apa yang dilakukannya                                                                                                                 | Hanya-baca |
| -------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------- | ---------- |
| `mcp__ide__getDiagnostics`                   | Mengembalikan diagnostik language-server — kesalahan dan peringatan di panel Problems VS Code. Secara opsional dibatasi ke satu file. | Ya         |
| `mcp__ide__executeCode`                      | Menjalankan kode Python di kernel notebook Jupyter yang aktif. Lihat alur konfirmasi di bawah.                                        | Tidak      |

**Eksekusi Jupyter selalu bertanya terlebih dahulu.** `mcp__ide__executeCode` tidak dapat menjalankan apa pun secara diam-diam. Pada setiap panggilan, kode dimasukkan sebagai sel baru di akhir notebook aktif, VS Code menggulirnya ke tampilan, dan Quick Pick asli meminta Anda untuk **Execute** atau **Cancel**. Membatalkan — atau menutup picker dengan `Esc` — mengembalikan kesalahan ke Claude dan tidak ada yang berjalan. Alat juga menolak dengan tegas saat tidak ada notebook aktif, saat ekstensi Jupyter (`ms-toolsai.jupyter`) tidak diinstal, atau saat kernel bukan Python.

<Note>
  Konfirmasi Quick Pick terpisah dari hook `PreToolUse`. Entri allowlist untuk `mcp__ide__executeCode` memungkinkan Claude *mengusulkan* menjalankan sel; Quick Pick di dalam VS Code adalah apa yang memungkinkannya *benar-benar* berjalan.
</Note>

<a id="troubleshooting" />

<h2 id="fix-common-issues">
  Perbaiki masalah umum
</h2>

<h3 id="extension-won’t-install">
  Ekstensi tidak akan diinstal
</h3>

* Pastikan Anda memiliki versi VS Code yang kompatibel (1.98.0 atau lebih baru)
* Periksa bahwa VS Code memiliki izin untuk menginstal ekstensi
* Coba instal langsung dari [VS Code Marketplace](https://marketplace.visualstudio.com/items?itemName=anthropic.claude-code)

<h3 id="spark-icon-not-visible">
  Ikon Spark tidak terlihat
</h3>

Ikon Spark muncul di **Editor Toolbar** (kanan atas editor) saat Anda memiliki file terbuka. Jika Anda tidak melihatnya:

1. **Buka file**: Ikon memerlukan file untuk dibuka. Hanya membuka folder tidak cukup.
2. **Periksa versi VS Code**: Memerlukan 1.98.0 atau lebih tinggi (Help → About)
3. **Restart VS Code**: Jalankan "Developer: Reload Window" dari Command Palette
4. **Nonaktifkan ekstensi yang bertentangan**: Sementara nonaktifkan ekstensi AI lainnya (Cline, Continue, dll.)
5. **Periksa kepercayaan workspace**: Ekstensi tidak berfungsi dalam Restricted Mode

Alternatifnya, klik "✱ Claude Code" di **Status Bar** (sudut kanan bawah). Ini berfungsi bahkan tanpa file terbuka. Anda juga dapat menggunakan **Command Palette** (`Cmd+Shift+P` / `Ctrl+Shift+P`) dan ketik "Claude Code".

<h3 id="cmd-esc-does-nothing-on-macos">
  Cmd+Esc tidak melakukan apa pun di macOS
</h3>

Di macOS Tahoe dan yang lebih baru, pintasan Game Overlay sistem terikat ke `Cmd+Esc` secara default dan mengintersepsi penekanan tombol sebelum mencapai VS Code. Untuk membebaskan pintasan:

1. Buka System Settings
2. Buka Keyboard, kemudian Keyboard Shortcuts, kemudian Game Controllers
3. Hapus centang Game Overlay

Alternatifnya, ikat ulang ekstensi ke tombol yang berbeda: buka editor [Keyboard Shortcuts](https://code.visualstudio.com/docs/configure/keybindings) VS Code (`Cmd+K Cmd+S`), cari `Claude Code: Focus input`, dan tetapkan pengikatan baru.

<h3 id="claude-code-never-responds">
  Claude Code tidak pernah merespons
</h3>

Jika Claude Code tidak merespons prompt Anda:

1. **Periksa koneksi internet Anda**: Pastikan Anda memiliki koneksi internet yang stabil
2. **Mulai percakapan baru**: Coba mulai percakapan segar untuk melihat apakah masalah berlanjut
3. **Coba CLI**: Jalankan `claude` dari terminal untuk melihat apakah Anda mendapatkan pesan kesalahan yang lebih detail

Jika masalah berlanjut, [file an issue on GitHub](https://github.com/anthropics/claude-code/issues) dengan detail tentang kesalahan.

<h2 id="uninstall-the-extension">
  Uninstal ekstensi
</h2>

Untuk menguninstal ekstensi Claude Code:

1. Buka tampilan Extensions (`Cmd+Shift+X` di Mac atau `Ctrl+Shift+X` di Windows/Linux)
2. Cari "Claude Code"
3. Klik **Uninstall**

Menjalankan `claude` di terminal terintegrasi VS Code akan menginstal ulang ekstensi secara otomatis. Untuk tetap menguninstalnya, matikan **Auto-install IDE extension** di `/config`, atau atur [`autoInstallIdeExtension`](/docs/id/settings#global-config-settings) ke `false`. Anda juga dapat mengatur variabel lingkungan [`CLAUDE_CODE_IDE_SKIP_AUTO_INSTALL`](/docs/id/env-vars) ke `1`.

Untuk juga menghapus data ekstensi dan reset semua pengaturan, hapus direktori penyimpanan ekstensi untuk platform Anda.

Di macOS:

```bash theme={null}
rm -rf ~/Library/"Application Support"/Code/User/globalStorage/anthropic.claude-code
```

Di Linux:

```bash theme={null}
rm -rf ~/.config/Code/User/globalStorage/anthropic.claude-code
```

Di Windows, di PowerShell:

```powershell theme={null}
Remove-Item -Recurse -Force "$env:APPDATA\Code\User\globalStorage\anthropic.claude-code"
```

Untuk bantuan tambahan, lihat [panduan troubleshooting](/docs/id/troubleshooting).

<h2 id="next-steps">
  Langkah berikutnya
</h2>

Sekarang Anda telah menyiapkan Claude Code di VS Code:

* [Jelajahi alur kerja umum](/docs/id/common-workflows) untuk mendapatkan hasil maksimal dari Claude Code
* [Siapkan MCP servers](/docs/id/mcp) untuk memperluas kemampuan Claude dengan alat eksternal. Tambahkan server menggunakan CLI, kemudian kelola dengan `/mcp` di panel chat.
* [Konfigurasi pengaturan Claude Code](/docs/id/settings) untuk menyesuaikan perintah yang diizinkan, hooks, dan lainnya. Pengaturan ini dibagikan antara ekstensi dan CLI.
