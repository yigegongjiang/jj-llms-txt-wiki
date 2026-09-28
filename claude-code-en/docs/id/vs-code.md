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

* VS Code 1.94.0 atau lebih tinggi
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
    Di seluruh VS Code, ikon Spark menunjukkan Claude Code: <img src="https://mintcdn.com/claude-code/c5r9_6tjPMzFdDDT/images/vs-code-spark-icon.svg?fit=max&auto=format&n=c5r9_6tjPMzFdDDT&q=85&s=3ca45e00deadec8c8f4b4f807da94505" alt="Ikon Spark" style={{display: "inline", height: "0.85em", verticalAlign: "middle"}} width="16" height="16" data-path="images/vs-code-spark-icon.svg" />

    Cara tercepat untuk membuka Claude adalah dengan mengklik ikon Spark di **Editor Toolbar** (sudut kanan atas editor). Ikon hanya muncul ketika Anda memiliki file yang terbuka.

    <img src="https://mintcdn.com/claude-code/mfM-EyoZGnQv8JTc/images/vs-code-editor-icon.png?fit=max&auto=format&n=mfM-EyoZGnQv8JTc&q=85&s=eb4540325d94664c51776dbbfec4cf02" alt="VS Code editor menampilkan ikon Spark di Editor Toolbar" width="2796" height="734" data-path="images/vs-code-editor-icon.png" />

    Cara lain untuk membuka Claude Code:

    * **Activity Bar**: klik ikon Spark di sidebar kiri untuk membuka daftar sesi. Klik sesi apa pun untuk membukanya di [lokasi pilihan Anda](#extension-settings), atau mulai yang baru. Ikon ini selalu terlihat di Activity Bar.
    * **Command Palette**: `Cmd+Shift+P` (Mac) atau `Ctrl+Shift+P` (Windows/Linux), ketik "Claude Code", dan pilih opsi seperti "Open in New Tab"
    * **Status Bar**: jika Anda telah menetapkan [`preferredLocation`](#extension-settings) ke `sidebar`, atau membuka Claude dengan **Claude Code: Open in Side Bar**, klik **✻ Claude Code** di sudut kanan bawah jendela. Ini berfungsi bahkan ketika tidak ada file yang terbuka.

    Anda dapat menyeret panel Claude untuk memposisikannya kembali di mana saja di VS Code. Lihat [Sesuaikan alur kerja Anda](#customize-your-workflow) untuk detail.
  </Step>

  <Step title="Masuk">
    Pertama kali Anda membuka panel, layar masuk muncul. Klik **Sign in** dan selesaikan otorisasi di browser Anda.

    Jika Anda melihat **Not logged in · Please run /login** nanti, ekstensi membuka kembali layar masuk secara otomatis. Jika tidak muncul, muat ulang jendela dari Command Palette dengan **Developer: Reload Window**.

    Jika Anda memiliki `ANTHROPIC_API_KEY` yang ditetapkan di shell Anda tetapi masih melihat prompt masuk, VS Code mungkin tidak mewarisi lingkungan shell Anda. Luncurkan VS Code dari terminal dengan `code .` sehingga mewarisi variabel lingkungan Anda, atau masuk dengan akun Claude Anda sebagai gantinya.

    Setelah Anda masuk, daftar periksa **Learn Claude Code** muncul. Kerjakan setiap item dengan mengklik **Show me**, atau tutup dengan X. Untuk membukanya kembali nanti, hapus centang **Hide Onboarding** di pengaturan VS Code di bawah Extensions → Claude Code.
  </Step>

  <Step title="Kirim prompt">
    Minta Claude untuk membantu dengan kode atau file Anda, baik itu menjelaskan cara kerja sesuatu, men-debug masalah, atau membuat perubahan.

    <Tip>Claude secara otomatis melihat teks yang Anda pilih. Tekan `Option+K` (Mac) / `Alt+K` (Windows/Linux) untuk juga menyisipkan referensi @-mention (seperti `@file.ts#5-10`) ke dalam prompt Anda.</Tip>

    Berikut adalah contoh menanyakan tentang baris tertentu dalam file:

    <img src="https://mintcdn.com/claude-code/FVYz38sRY-VuoGHA/images/vs-code-send-prompt.png?fit=max&auto=format&n=FVYz38sRY-VuoGHA&q=85&s=ede3ed8d8d5f940e01c5de636d009cfd" alt="VS Code editor dengan baris 2-3 dipilih dalam file Python, dan panel Claude Code menampilkan pertanyaan tentang baris-baris tersebut dengan referensi @-mention" width="3288" height="1876" data-path="images/vs-code-send-prompt.png" />
  </Step>

  <Step title="Tinjau perubahan">
    Apa yang Anda lihat tergantung pada [mode izin](/docs/id/permission-modes#which-mode-a-session-starts-in) yang ditampilkan di bagian bawah kotak prompt:

    * Dalam mode Auto atau Edit automatically, Claude mengedit sebagian besar file di workspace Anda tanpa bertanya.
    * Dalam mode Manual, ketika Claude ingin mengedit file, Claude menampilkan perbandingan berdampingan dari perubahan asli dan yang diusulkan, kemudian meminta izin. Anda dapat menerima, menolak, atau memberi tahu Claude apa yang harus dilakukan sebagai gantinya. Jika Anda mengedit konten yang diusulkan secara langsung di tampilan diff sebelum menerima, Claude diberitahu bahwa Anda memodifikasinya sehingga tidak menganggap file cocok dengan proposal aslinya.

          <img src="https://mintcdn.com/claude-code/FVYz38sRY-VuoGHA/images/vs-code-edits.png?fit=max&auto=format&n=FVYz38sRY-VuoGHA&q=85&s=e005f9b41c541c5c7c59c082f7c4841c" alt="VS Code menampilkan diff dari perubahan yang diusulkan Claude dengan prompt izin yang menanyakan apakah akan membuat edit" width="3292" height="1876" data-path="images/vs-code-edits.png" />

    Untuk meninjau edit yang diusulkan satu perubahan pada satu waktu, gunakan tombol **Accept this change** dan **Reject this change** di bawah setiap perubahan dalam diff. Menolak perubahan mengembalikannya dalam konten yang diusulkan; menerima menandainya sebagai ditinjau. Menerima atau menolak seluruh file masih menyelesaikan tinjauan. Diff dengan lebih dari 100 perubahan terbuka tanpa tombol per-perubahan, jadi tinjau sebagai seluruh file. Tinjauan per-perubahan memerlukan Claude Code v2.1.275 atau lebih baru.

    Tindakan yang sama tersedia di kursor dari menu konteks editor dan dari Command Palette sebagai **Claude Code: Accept Change at Cursor** dan **Claude Code: Reject Change at Cursor**.
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

* **Mode izin**: klik indikator mode di bagian bawah kotak prompt untuk beralih mode izin. Pada paket Pro, Max, dan Team, Auto adalah mode izin bawaan awal. Lihat [bagaimana ekstensi memilih mode izin awal](/docs/id/permission-modes#switch-permission-modes) untuk mengetahui apa yang mengubahnya, dan setiap mode izin yang ditawarkan indikator.
  * **Auto**: pengklasifikasi meninjau sebagian besar tindakan alih-alih meminta Anda. Lihat [mode auto](/docs/id/permission-modes#eliminate-prompts-with-auto-mode) untuk mengetahui apa yang ditinjau dan diblokir.
  * **Manual**: Claude meminta izin sebelum pengeditan file dan sebagian besar perintah shell.
  * **Plan**: Claude menjelaskan apa yang akan dilakukan dan menunggu persetujuan sebelum membuat perubahan. VS Code secara otomatis membuka rencana sebagai dokumen Markdown lengkap di mana Anda dapat menambahkan komentar inline untuk memberikan umpan balik sebelum Claude dimulai.

    Anda juga dapat mengetik `/plan` di kotak prompt. Memerlukan Claude Code v2.1.280 atau lebih baru.

    * `/plan`: beralih ke plan mode. Jika Anda sudah dalam plan mode, menampilkan rencana saat ini sebagai gantinya.
    * `/plan` dengan tugas, seperti `/plan fix the auth bug`: beralih ke plan mode dan mulai merencanakan tugas itu.
    * `/plan open`: ketika Anda sudah dalam plan mode, membuka file rencana di editor.
  * **Edit automatically**: Claude membuat pengeditan tanpa bertanya.
* **Model**: pilih **Switch model…** dari menu perintah untuk mengubah model di tengah sesi. Anda juga dapat mengklik nama model di bagian bawah kotak prompt untuk membuka pemilih yang sama.

  Ketika model saat ini mendukung [tingkat upaya](/docs/id/model-config#adjust-effort-level), pemilih juga menampilkan baris **Effort** dan tombol nama model menunjukkan tingkat yang dipilih. Ketika Anda memilih tingkat selain `max`, Claude Code menyimpannya untuk model saat ini sebagai default Anda, di bawah [`modelSettings`](/docs/id/settings-reference#modelsettings) dalam pengaturan pengguna Anda; `max` berlaku hanya untuk sesi saat ini. Tombol nama model dan baris **Effort** memerlukan Claude Code v2.1.257 atau lebih baru.
* **Command menu**: klik `/` atau ketik `/` untuk membuka menu perintah. Opsi termasuk melampirkan file, beralih model, dan mengalihkan pemikiran yang diperluas.

  Bagian Customize menyediakan akses ke server MCP, slash commands, gaya output, hooks, memory, instructions, permissions, dan plugins. Item dengan ikon terminal terbuka di terminal terintegrasi.

  * Untuk menelusuri perintah seperti `/usage` atau [`/remote-control`](/docs/id/remote-control), pilih **Slash commands** di bagian Customize. Dialog mencantumkan mereka dengan kotak filter. Pilih satu untuk menjalankannya. Mengetik `/` di kotak prompt masih menyarankan perintah secara inline. Memerlukan Claude Code v2.1.257 atau lebih baru.

    Mengetik `/skills` juga membuka dialog ini. Setiap baris [skill](/docs/id/skills) menunjukkan [visibility](/docs/id/skills#override-skill-visibility-from-settings)-nya, seperti **On** atau **Name only**. Klik visibility untuk mengubahnya, kecuali pada baris yang ditandai **locked**, seperti plugin skills. Pintasan `/skills` dan kontrol visibility memerlukan Claude Code v2.1.280 atau lebih baru.
  * Pilih **Output styles** di bagian Customize untuk memilih [gaya output](/docs/id/output-styles), termasuk gaya kustom Anda. Memerlukan Claude Code v2.1.257 atau lebih baru.

    Untuk membuat gaya kustom sebagai gantinya, pilih **Build a custom style** dari menu **Output styles**. Claude Code menulis [file gaya](/docs/id/output-styles#create-a-custom-output-style) untuk Anda di tingkat proyek atau pengguna. Memerlukan Claude Code v2.1.261 atau lebih baru.
  * Pilih **Hooks** di bagian Customize untuk melihat [hooks](/docs/id/hooks) yang dimuat dalam sesi, dikelompokkan berdasarkan acara. Anda dapat menambah, mengedit, atau menghapus hooks yang disimpan di file pengaturan pengguna, proyek, dan lokal Anda. Hooks dari sumber lain, seperti pengaturan terkelola atau plugins, bersifat read-only. Memerlukan Claude Code v2.1.269 atau lebih baru.
  * Pilih **Permissions** di bagian Customize untuk melihat [aturan izin](/docs/id/permissions) sesi, dikelompokkan menjadi Allow, Ask, dan Deny. Anda dapat menambahkan aturan ke pengaturan pengguna, proyek, atau lokal Anda dan menghapus aturan yang disimpan di sana. Aturan dari sumber lain, seperti pengaturan terkelola atau persetujuan yang dibuat hanya untuk sesi ini, bersifat read-only. Memerlukan Claude Code v2.1.269 atau lebih baru.
  * Pilih **Memory** di bagian Customize untuk mengaktifkan atau menonaktifkan [auto memory](/docs/id/memory#auto-memory). Saat aktif, Anda juga dapat menelusuri memori yang telah disimpan Claude dan mengungkapkan folder yang menyimpannya di pengelola file Anda. Memerlukan Claude Code v2.1.274 atau lebih baru.

    Klik memori yang disimpan untuk membacanya di dialog, di mana Anda dapat mengedit teks, menghapus memori, atau membuka filenya di editor. Melihat, mengedit, dan menghapus memori di dialog memerlukan Claude Code v2.1.275 atau lebih baru.
  * Pilih **Instructions** di bagian Customize untuk mengedit [file CLAUDE.md](/docs/id/memory#claude-md-files) yang dibaca Claude. Pilih file untuk membukanya di editor. Jika file belum ada, Claude Code akan membuatnya terlebih dahulu. Memerlukan Claude Code v2.1.274 atau lebih baru.
  * Pilih **Status** di bagian Customize, atau ketik `/status`, untuk memeriksa versi Claude Code sesi, akun, model, dan detail server MCP. Memerlukan Claude Code v2.1.280 atau lebih baru.
  * Pilih **Sandbox** di bagian Customize, atau ketik `/sandbox`, untuk melihat apakah perintah Bash Claude berjalan [sandboxed](/docs/id/sandboxing). Anda dapat beralih mode sandbox dan menambahkan [excluded commands](/docs/id/settings-reference#sandbox-excludedcommands) di sana. Memerlukan Claude Code v2.1.280 atau lebih baru.
  * Pilih **Claude in Chrome** di bagian Customize, atau ketik `/chrome`, untuk memeriksa dan mengelola koneksi [Claude in Chrome](/docs/id/chrome). Keduanya memerlukan masuk dengan akun claude.ai. Memerlukan Claude Code v2.1.280 atau lebih baru.
  * Pilih **Export conversation** di bagian Context, atau ketik `/export`, untuk menyalin percakapan sebagai teks biasa atau menyimpannya ke file. Tambahkan nama file, seperti `/export notes.txt`, untuk melewati dialog dan memilih tempat menyimpan file. Memerlukan Claude Code v2.1.280 atau lebih baru.
  * Bagian Settings mencakup **Enable Remote Control for all sessions**, yang menetapkan [`remoteControlAtStartup`](/docs/id/settings-reference#remotecontrolatstartup) untuk mengontrol apakah [sesi interaktif baru terhubung ke Remote Control secara otomatis](/docs/id/remote-control#enable-remote-control-for-all-sessions). Memerlukan Claude Code v2.1.203 atau lebih baru.

    Ketika Anda mengalihkan toggle on atau off di jendela VS Code, perubahan berlaku untuk sesi yang sudah terbuka di jendela VS Code itu, bukan hanya untuk sesi yang Anda mulai setelahnya. Jika Anda mematikannya, sesi terbuka akan terputus. Dengan Claude Code v2.1.261 atau lebih baru, perubahan juga mencapai sesi yang terbuka di jendela VS Code lainnya Anda.
  * Bagian Settings juga mencakup **Focus view**, yang menyembunyikan panggilan alat, hasil alat, dan pemikiran di balik baris yang dapat diperluas, meninggalkan prompt Anda dan respons Claude. Alihkan di sana, dengan `Ctrl+Option+F` (Mac) / `Ctrl+Alt+F` (Windows/Linux), atau dari Command Palette dengan **Claude Code: Toggle Focus view**. Perubahan berlaku untuk setiap sesi terbuka dan bertahan di seluruh sesi. Memerlukan Claude Code v2.1.221 atau lebih baru.

    Daftar tugas terbaru Claude tetap terlihat, dan begitu juga dengan teks pertanyaan yang tertunda dari Claude yang ditanyakan; ini memerlukan Claude Code v2.1.225 atau lebih baru. Saat Claude menjalankan [subagents](/docs/id/sub-agents), baris kemajuan langsung dengan aktivitas terbaru mereka muncul di bawah grup panggilan alat yang memulainya. Ini memerlukan Claude Code v2.1.269 atau lebih baru.
  * Untuk keluar dari akun Anthropic Anda, pilih **Sign out** di bagian Settings, atau ketik `/logout`. Pada [penyedia pihak ketiga](#use-third-party-providers), menu tidak menawarkan keduanya. Memerlukan Claude Code v2.1.277 atau lebih baru.
  * Untuk melaporkan bug, klik **Report a problem** di bagian bawah menu, atau ketik `/bug` atau `/feedback` dengan deskripsi opsional yang mengisi laporan sebelumnya. Ketika Anda mengirimkan laporan dan Anda masuk ke Anthropic pada koneksi pihak pertama, Claude Code mengirimkannya ke Anthropic. Pada penyedia pihak ketiga, atau tanpa kredensial Anthropic, dialog masih terbuka, tetapi mengirimkan menunjukkan kesalahan dan tidak mengirim apa pun: tidak seperti `/bug` CLI, ekstensi tidak menulis arsip lokal. Memerlukan Claude Code v2.1.229 atau lebih baru.

    Jika kebijakan organisasi Anda mematikan umpan balik produk, **Report a problem** tidak muncul di menu, dan `/bug` serta `/feedback` menampilkan pemberitahuan `Feedback is turned off by your organization's policy or this environment's settings.` alih-alih membuka laporan.
* **Side questions**: ketik `/btw` diikuti oleh pertanyaan untuk bertanya tentang sesi Anda [tanpa menambah percakapan](/docs/id/interactive-mode#side-questions-with-%2Fbtw). Jawaban terbuka di panel di samping obrolan, di mana Anda dapat mengajukan pertanyaan lanjutan. Thread bertahan dari muat ulang jendela. Claude Code menyimpan 20 pertukaran terbaru dan thread yang disimpan kedaluwarsa pada jadwal [`cleanupPeriodDays`](/docs/id/settings-reference#cleanupperioddays), selama Claude Code dapat [dengan aman menentukan periode retensi](/docs/id/claude-directory#cleaned-up-automatically). Untuk menghapus thread, klik ikon tempat sampah di panel. Memerlukan Claude Code v2.1.227 atau lebih baru.
* **Copy a response**: arahkan kursor ke respons dan klik **Copy response** untuk menyalinnya ke clipboard Anda, atau ketik `/copy` untuk menyalin respons terbaru. `/copy 2` menyalin yang kedua terakhir. Memerlukan Claude Code v2.1.277 atau lebih baru.
* **Context indicator**: kotak prompt menunjukkan berapa banyak jendela konteks Claude yang Anda gunakan. Claude secara otomatis mengompres saat diperlukan, atau Anda dapat menjalankan `/compact` secara manual.
* **Prompt cache clock**: ikon jam di samping indikator konteks memperkirakan berapa banyak waktu [prompt cache](/docs/id/prompt-caching) percakapan yang tersisa sebelum kedaluwarsa. Ini menghitung mundur dari [lifetime](/docs/id/prompt-caching#cache-lifetime) cache lima menit atau satu jam, dan setiap respons yang menggunakan cache memulai ulang hitungan mundur. Terlepas dari pemadatan, [tindakan yang membatalkan cache](/docs/id/prompt-caching#actions-that-invalidate-the-cache) tidak mengatur ulang jam, jadi masih dapat menunjukkan menit yang tersisa setelah Anda beralih model.
  * Sampai hitungan mundur habis, ikon menunjukkan menit yang tersisa, seperti **12m**.
  * Ketika hitungan mundur habis, menit menghilang dan ikon berubah merah, atau warna kesalahan tema Anda, sampai respons berikutnya. Cache mungkin telah kedaluwarsa, jadi harapkan respons yang lebih lambat dan lebih mahal untuk pesan Anda berikutnya saat cache dibangun kembali. Jika lifetime lima menit terus habis di antara pesan Anda, lihat [Pilih TTL sendiri](/docs/id/prompt-caching#choose-the-ttl-yourself).
  * Tepat setelah percakapan [dipadatkan](/docs/id/prompt-caching#compacting-the-conversation), ikon juga berubah merah tanpa menit sampai respons berikutnya, karena cache tidak mencakup percakapan yang dipadatkan namun.
* **Agent map**: ketika percakapan mencakup [subagents](/docs/id/sub-agents), hitungan agen seperti **2 agents** muncul di bagian bawah kotak prompt. Titiknya menunjukkan apakah ada subagent yang bekerja atau menunggu izin Anda.

  Klik hitungan agen untuk membuka peta agen, yang menggambar subagent percakapan sebagai pohon di bawah agen utama, masing-masing dengan status, waktu yang telah berlalu, dan hitungan token. Klik subagent untuk melihat prompt dan panggilan alat, buka transkrip read-only, atau hentikan saat berjalan. Memerlukan Claude Code v2.1.269 atau lebih baru.

  Peta juga mencantumkan [tugas latar belakang](/docs/id/tools-reference#background-commands) sesi lainnya, seperti perintah shell latar belakang dan [monitors](/docs/id/tools-reference#monitor-tool), di bawah agen. Klik baris untuk membuka kartu tugas dan hentikan di sana.

  Untuk membuka peta ketika tidak ada hitungan agen yang ditampilkan, seperti ketika Claude telah memulai shell latar belakang tetapi tidak ada subagents, ketik `/tasks` di kotak prompt. Tugas latar belakang dalam peta dan `/tasks` yang diketik memerlukan Claude Code v2.1.277 atau lebih baru.
* **Extended thinking**: memungkinkan Claude menghabiskan lebih banyak waktu untuk bernalar melalui masalah yang kompleks. Alihkan melalui menu perintah (`/`). Penalaran Claude muncul dalam percakapan sebagai blok yang runtuh: klik blok untuk membacanya, atau tekan `Ctrl+O` untuk memperluas atau meruntuhkan setiap blok pemikiran dalam sesi. Lihat [Extended thinking](/docs/id/model-config#extended-thinking) untuk detail.
* **Multi-line input**: tekan `Shift+Enter` untuk menambahkan baris baru tanpa mengirim. Ini juga berfungsi di input teks bebas "Other" dari dialog pertanyaan.

<h3 id="reference-files-and-folders">
  Referensikan file dan folder
</h3>

Gunakan @-mentions untuk memberikan Claude konteks tentang file atau folder tertentu. Ketika Anda mengetik `@` diikuti oleh nama file atau folder, Claude membaca konten itu dan dapat menjawab pertanyaan tentangnya atau membuat perubahan padanya. Claude Code mendukung fuzzy matching, jadi Anda dapat mengetik nama parsial untuk menemukan apa yang Anda butuhkan:

```text wrap theme={null}
Explain the logic in @auth (fuzzy matches auth.js, AuthService.ts, etc.)
What's in @src/components/ (include a trailing slash for folders)
```

Untuk PDF besar, Anda dapat meminta Claude membaca halaman tertentu alih-alih seluruh file: satu halaman, rentang seperti halaman 1-10, atau rentang terbuka seperti halaman 3 ke depan.

Ketika Anda memilih teks di editor, Claude dapat melihat kode yang disorot secara otomatis. Footer kotak prompt menunjukkan berapa banyak baris yang dipilih. Tekan `Option+K` (Mac) / `Alt+K` (Windows/Linux) untuk menyisipkan @-mention dengan jalur file dan nomor baris (misalnya, `@app.ts#5-10`). Klik **X** pada indikator seleksi untuk menghapusnya sehingga Claude tidak menerima seleksi. Indikator muncul kembali ketika Anda memilih teks lain.

Ekstensi menahan teks yang dipilih dari beberapa file. Ketika file berada di dalam workspace Anda dan cocok dengan pengaturan `files.exclude` atau `search.exclude` Anda, Claude menerima paling banyak jalur file dan bukan teks yang Anda pilih. Hal yang sama berlaku untuk file yang diabaikan git, selama pengaturan `search.useIgnoreFiles` VS Code dan pengaturan [`respectGitIgnore`](#extension-settings) ekstensi keduanya aktif, yang merupakan default. Filter ini mencakup panel obrolan saja: ketika Claude Code berjalan di terminal terintegrasi, CLI mengirimkan teks yang dipilih Anda apa pun filenya, jadi tambahkan [aturan deny `Read`](#the-built-in-ide-mcp-server) untuk menjaga konten file dari Claude di sana.

Claude juga melihat file mana yang Anda buka di editor, bahkan ketika tidak ada yang dipilih, dan kotak prompt menunjukkan namanya. Untuk menambahkan hanya teks yang dipilih, matikan [pengaturan Attach Open File](vscode://settings/claudeCode.attachOpenFile). Pengaturan memerlukan Claude Code v2.1.271 atau lebih baru.

Anda juga dapat melampirkan gambar dan file ke pesan Anda:

* Untuk melampirkan gambar, tempel dari clipboard Anda ke kotak prompt.
* Untuk melampirkan file, tahan `Shift` sambil menyeret mereka ke kotak prompt.
* Untuk menghapus lampiran dari konteks, klik X padanya.

<h3 id="paste-text">
  Tempel teks
</h3>

Teks yang Anda tempel tetap terlihat di kotak prompt, daripada runtuh menjadi placeholder seperti yang terjadi [di terminal](/docs/id/terminal-config#paste-large-content). Dalam sesi di mana Claude Code [menandai teks yang ditempel](/docs/id/terminal-config#how-claude-treats-pasted-text), Claude masih melihat tempel besar sebagai teks yang Anda tempel daripada ketik.

Claude Code juga menghapus [karakter Unicode yang tidak terlihat](/docs/id/interactive-mode#invisible-characters-in-prompts) dari teks yang Anda tempel ke kotak prompt dan dari apa pun yang Anda kirim:

* Jika pemberitahuan seperti `Removed 3 invisible characters from the pasted text` muncul ketika Anda menempel, teks masuk tanpa karakter tersebut.
* Jika pemberitahuan tentang karakter yang dihapus muncul ketika Anda mengirim, tidak ada yang dikirim. Teks yang dibersihkan kembali di kotak prompt. Kirim lagi untuk mengirim teks seperti yang ditampilkan.

<h3 id="resume-past-conversations">
  Lanjutkan percakapan masa lalu
</h3>

Klik tombol **Session history** di bagian atas panel Claude Code untuk mengakses riwayat percakapan Anda. Anda dapat mencari berdasarkan kata kunci atau menelusuri berdasarkan waktu.

Klik percakapan apa pun untuk melanjutkannya dengan riwayat pesan lengkap. Jika percakapan sudah terbuka di tab lain dari jendela saat ini, mengkliknya akan beralih ke tab itu. Untuk informasi lebih lanjut tentang melanjutkan sesi, lihat [Manage sessions](/docs/id/sessions).

* **Session titles**: sesi baru menerima judul yang dihasilkan AI berdasarkan pesan pertama Anda.
* **Rename and archive**: arahkan kursor ke sesi untuk mengungkapkan tindakan ini. Rename untuk memberikannya judul deskriptif, atau archive untuk memindahkannya ke grup **Archived sessions** di bagian bawah daftar.

Secara default, sesi tanpa aktivitas selama 14 hari pindah ke **Archived sessions** secara otomatis, kecuali jika terbuka, belum dibaca, atau dalam [grup](#organize-sessions-into-groups). Pengarsipan otomatis memerlukan Claude Code v2.1.265 atau lebih baru. Untuk mengubah periode atau mematikannya, buka [Archive Inactive Sessions setting](vscode://settings/claudeCode.archiveInactiveSessions) dan pilih jumlah hari atau **Never**.

Untuk memulihkan sesi yang diarsipkan, perluas **Archived sessions** dan klik **Unarchive session**. Untuk memulihkan setiap sesi yang diarsipkan sekaligus, arahkan kursor ke header **Archived sessions** dalam daftar sesi di Activity Bar dan klik ikon unarchive-nya, yang memerlukan Claude Code v2.1.277 atau lebih baru. Sebelum v2.1.257, tindakannya adalah **Delete session**, yang menyembunyikan sesi tanpa cara untuk memulihkannya. Sesi yang Anda hapus kemudian muncul di bawah **Archived sessions** setelah Anda upgrade.

Ketika percakapan yang Anda lanjutkan berakhir dalam plan mode, Claude Code memulihkan plan mode. Memerlukan Claude Code v2.1.246 atau lebih baru. Claude Code tidak memulihkannya dalam dua kasus:

* Ekstensi [memilih mode izin awal](/docs/id/permission-modes#switch-permission-modes) dari `claudeCode.initialPermissionMode` atau pilihan yang dibawa dari percakapan sebelumnya
* Anda memiliki `claudeCode.claudeProcessWrapper` yang dikonfigurasi

<h3 id="resume-cloud-sessions-from-claude-ai">
  Lanjutkan sesi cloud dari Claude.ai
</h3>

Jika Anda menjalankan [sesi cloud](/docs/id/claude-code-on-the-web), Anda dapat melanjutkan sesi tersebut langsung di VS Code. Ini memerlukan masuk dengan **Claude.ai Subscription**, bukan Anthropic Console.

<Steps>
  <Step title="Open session history">
    Klik tombol **Session history** di bagian atas panel Claude Code.
  </Step>

  <Step title="Select the Web tab">
    Dialog menampilkan dua tab: Local dan Web. Klik **Web** untuk melihat sesi dari claude.ai.
  </Step>

  <Step title="Select a session to resume">
    Telusuri atau cari sesi cloud Anda. Klik sesi apa pun untuk mengunduhnya dan melanjutkan percakapan secara lokal.
  </Step>
</Steps>

<Note>
  Hanya sesi cloud yang dimulai dengan repositori GitHub yang muncul di tab Web. Melanjutkan memuat riwayat percakapan secara lokal; perubahan tidak disinkronkan kembali ke claude.ai.
</Note>

<h3 id="check-account-and-usage">
  Periksa akun dan penggunaan
</h3>

Jalankan `/usage` untuk membuka dialog Account & usage. Dialog menunjukkan akun yang masuk, dan penggunaan yang dilaporkan berbeda menurut masuk:

* **Paket claude.ai**: bilah penggunaan untuk batas paket Anda, seperti sesi saat ini dan minggu ini. Setiap bilah menunjukkan berapa lama sampai batasnya direset.

  Dialog juga merinci apa yang berkontribusi pada batas paket Anda. Ini menandai perilaku yang menyumbang 10% atau lebih dari penggunaan baru-baru ini, seperti cache misses, konteks panjang, dan sesi yang berat subagent atau sangat paralel, masing-masing dengan tip untuk menguranginya. Tabel atribusi menunjukkan berapa banyak penggunaan yang berasal dari setiap skill, subagent, plugin, dan server MCP.

  Gunakan toggle Day dan Week untuk beralih antara 24 jam terakhir dan 7 hari terakhir. Angka-angka tersebut perkiraan dan dihitung dari sesi lokal di mesin ini, jadi penggunaan dari perangkat lain atau claude.ai tidak disertakan.
* **Masuk lainnya**: ketika batas paket tidak berlaku untuk masuk Anda, seperti pada [penyedia pihak ketiga](#use-third-party-providers) atau dengan kunci API, bagian Usage menunjukkan biaya sesi sendiri dan penggunaan token sebagai gantinya. `/usage` CLI menunjukkan total yang sama dalam [Session block](/docs/id/costs#track-your-costs)-nya. Daftar sesi dalam Activity Bar juga menunjukkan total sesi aktif di bawah header **Account & usage**-nya. Memerlukan Claude Code v2.1.277 atau lebih baru.

Untuk informasi lebih lanjut tentang pelacakan dan pengurangan penggunaan, lihat [Track your costs](/docs/id/costs#track-your-costs).

<h2 id="customize-your-workflow">
  Sesuaikan alur kerja Anda
</h2>

Anda dapat mengubah posisi panel Claude, menjalankan beberapa percakapan, mengorganisir daftar sesi ke dalam grup, atau beralih ke mode terminal.

<h3 id="choose-where-claude-lives">
  Pilih di mana Claude berada
</h3>

Anda dapat menyeret panel Claude untuk mengubah posisinya di mana saja di VS Code. Ambil tab atau bilah judul panel dan seret ke:

* **Secondary sidebar**: sisi kanan jendela. Membuat Claude tetap terlihat saat Anda coding.
* **Primary sidebar**: sidebar kiri dengan ikon untuk Explorer, Search, dll.
* **Editor area**: membuka Claude sebagai tab di samping file Anda. Berguna untuk tugas sampingan.

Ketika Claude membuka tab di grup editor baru, ekstensi mengunci grup tersebut, sehingga file yang Anda buka saat tab Claude fokus masuk ke grup lain alih-alih di sebelahnya.

Untuk menghentikan ekstensi dari mengunci grup, matikan [pengaturan Lock Editor Groups](vscode://settings/claudeCode.lockEditorGroups). Grup yang sudah terkunci tetap terkunci sampai Anda membukanya. Pengaturan ini memerlukan Claude Code v2.1.274 atau lebih baru.

<Tip>
  Gunakan sidebar untuk sesi Claude utama Anda dan buka tab tambahan untuk tugas sampingan. Claude mengingat lokasi pilihan Anda. Ikon daftar sesi Activity Bar terpisah dari panel Claude: daftar sesi selalu terlihat di Activity Bar, sementara ikon panel Claude hanya muncul di sana ketika panel ditambatkan ke sidebar kiri.
</Tip>

Setelah Anda menjalankan **Developer: Reload Window** atau memulai ulang VS Code, apakah percakapan kembali dengan percakapannya tergantung di mana percakapan itu dibuka:

* **Editor tab**: percakapan kembali dengan tabnya.
* **Sidebar**: percakapan kembali jika Anda mengirim pesan atau Claude merespons di dalamnya dalam 10 menit terakhir. Jika tidak kembali, lanjutkan percakapan dari [Session history](#resume-past-conversations).

Jika reload mengganggu Claude di tengah-tengah langkah, Claude melanjutkan langkah tersebut ketika percakapan kembali, dan pemberitahuan di chat menandai kelanjutannya. Memerlukan Claude Code v2.1.274 atau lebih baru. Jika langkah tersebut terganggu lebih dari satu jam yang lalu atau sesi terbuka di tempat lain, percakapan kembali dalam keadaan idle.

Untuk mematikan kelanjutan, buka [pengaturan Continue After Reload](vscode://settings/claudeCode.continueAfterReload) dan hapus centangnya.

<h3 id="run-multiple-conversations">
  Jalankan beberapa percakapan
</h3>

Gunakan **Open in New Tab** atau **Open in New Window** dari Command Palette untuk memulai percakapan tambahan. Setiap percakapan mempertahankan riwayat dan konteksnya sendiri, memungkinkan Anda bekerja pada tugas berbeda secara paralel.

Saat menggunakan tab, titik berwarna kecil pada ikon spark menunjukkan status: biru berarti permintaan izin tertunda, oranye berarti Claude selesai saat tab tersembunyi.

<h3 id="organize-sessions-into-groups">
  Organisir sesi ke dalam grup
</h3>

Di daftar sesi di Activity Bar, Anda dapat mengumpulkan sesi terkait ke dalam grup bernama yang dapat diciutkan. Memerlukan Claude Code v2.1.229 atau lebih baru.

* **Kelompokkan atau pisahkan sesi**: klik kanan sesi untuk membuat grup darinya, memindahkannya ke grup yang ada, atau menghapusnya dari grupnya. Setiap sesi termasuk dalam satu grup pada satu waktu, jadi memindahkannya ke grup lain menghapusnya dari yang pertama.
* **Pindahkan beberapa sesi sekaligus**: `Cmd`-klik (Mac) / `Ctrl`-klik (Windows/Linux) setiap sesi, atau `Shift`-klik untuk memilih rentang, kemudian klik kanan pilihan.
* **Kelompokkan sesi dari tabnya**: jalankan **Claude Code: Add Session Tab to Group** dari Command Palette, kemudian pilih atau buat grup. Memerlukan Claude Code v2.1.257 atau lebih baru.
* **Ubah nama atau hapus grup**: klik kanan header grup. Menghapus grup hanya menghapus grup, dan sesinya kembali ke daftar yang tidak dikelompokkan.

Ekstensi menyimpan grup per folder workspace, sehingga mereka bertahan dari reload jendela dan muncul di setiap jendela tempat Anda membuka folder yang sama. Saat Anda mencari daftar, ekstensi menampilkan kecocokan dalam satu daftar datar di semua grup.

<h3 id="switch-to-terminal-mode">
  Beralih ke mode terminal
</h3>

Secara default, ekstensi membuka panel chat grafis. Jika Anda lebih suka antarmuka gaya CLI, buka [pengaturan Use Terminal](vscode://settings/claudeCode.useTerminal) dan centang kotak.

Anda juga dapat membuka pengaturan VS Code (`Cmd+,` di Mac atau `Ctrl+,` di Windows/Linux), buka Extensions → Claude Code, dan centang **Use Terminal**.

<h2 id="manage-plugins">
  Kelola plugin
</h2>

Ekstensi VS Code menyertakan antarmuka grafis untuk menginstal dan mengelola [plugin](/docs/id/plugins/overview). Ketik `/plugins` di kotak prompt untuk membuka antarmuka **Kelola plugin**.

<h3 id="install-plugins">
  Instal plugin
</h3>

Dialog plugin menampilkan dua tab: **Plugin** dan **Marketplaces**.

Di tab Plugin:

* **Plugin yang terinstal** muncul di bagian atas dengan tombol toggle untuk mengaktifkan atau menonaktifkan
* **Plugin yang tersedia** dari marketplace yang dikonfigurasi muncul di bawah
* Cari untuk memfilter plugin berdasarkan nama atau deskripsi
* Klik **Instal** pada plugin yang tersedia

Ketika Anda menginstal plugin, pilih cakupan instalasi:

* **Instal untuk Anda**: tersedia di semua proyek Anda (cakupan pengguna)
* **Instal untuk proyek ini**: dibagikan dengan kolaborator proyek (cakupan proyek)
* **Instal secara lokal**: hanya untuk Anda, hanya di repositori ini (cakupan lokal)

<h3 id="share-a-plugin-install-link">
  Bagikan tautan instalasi plugin
</h3>

Untuk mengirim seseorang langsung ke instalasi plugin tertentu, berikan mereka URL `install-plugin` ekstensi. Membukanya meluncurkan atau memfokuskan VS Code, membuka panel Claude Code, dan membuka dialog **Kelola plugin** pada pilihan cakupan plugin tersebut. Tidak ada yang terinstal sampai orang tersebut memilih cakupan. Jika marketplace plugin belum dikonfigurasi di Claude Code mereka, dialog terlebih dahulu meminta mereka untuk menambahkannya.

```text theme={null}
vscode://anthropic.claude-code/install-plugin?plugin=code-review&marketplace=anthropics/claude-plugins-official
```

URL mengambil dua parameter query:

| Parameter     | Deskripsi                                                                                                                                                                                            |
| ------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `plugin`      | Nama plugin seperti yang tercantum di marketplace-nya. Diperlukan.                                                                                                                                   |
| `marketplace` | Tempat plugin berasal: repositori GitHub `owner/repo`, URL `https://`, atau URL git SSH seperti `git@github.com:owner/repo.git`. Default ke `anthropics/claude-plugins-official` ketika dihilangkan. |

Beberapa nilai yang diterima [tab Marketplaces](#manage-marketplaces) tidak berfungsi dalam tautan, seperti jalur lokal atau alamat `http://`. Untuk yang tersebut, VS Code menampilkan pesan kesalahan dan dialog tidak terbuka.

Dua kasus berakhir pada pesan dalam dialog alih-alih pilihan cakupan:

* **Marketplace tidak mencantumkan plugin dengan nama tersebut**: dialog melaporkan bahwa plugin tidak ditemukan. Periksa nilai `plugin` terhadap daftar marketplace.
* **Plugin sudah terinstal**: dialog mengatakan demikian, dan tidak ada yang berubah.

README GitHub, issues, dan beberapa host Markdown lainnya menghapus tautan yang skemanya bukan `http` atau `https`, jadi tautan `vscode://` di sana ditampilkan sebagai teks biasa. Letakkan URL dalam blok kode di host tersebut, seperti yang dijelaskan [Tautan ditampilkan sebagai teks biasa alih-alih dapat diklik](/docs/id/deep-links#the-link-renders-as-plain-text-instead-of-being-clickable) untuk tautan `claude-cli://`.

<h3 id="manage-marketplaces">
  Kelola marketplace
</h3>

Beralih ke tab **Marketplaces** untuk menambah atau menghapus sumber plugin:

* Masukkan repo GitHub, URL, atau jalur lokal untuk menambahkan marketplace baru
* Klik ikon refresh untuk memperbarui daftar plugin marketplace
* Klik ikon sampah untuk menghapus marketplace

Perubahan plugin yang Anda buat dalam dialog diterapkan segera ke sesi Claude Code yang terbuka di jendela VS Code tersebut. Jika sesi tempat Anda membuka dialog tidak dapat memuat ulang pluginnya, dialog menawarkan untuk mencoba lagi atau memulai ulang Claude di sesi tersebut.

<Note>
  Manajemen plugin di VS Code menggunakan perintah CLI yang sama di balik layar. Plugin dan marketplace yang Anda konfigurasi di ekstensi juga tersedia di CLI, dan sebaliknya.
</Note>

Untuk informasi lebih lanjut tentang sistem plugin, lihat [Plugin](/docs/id/plugins/overview) dan [Plugin marketplaces](/docs/id/plugins/overview).

<h2 id="automate-browser-tasks-with-chrome">
  Otomatisasi tugas browser dengan Chrome
</h2>

Hubungkan Claude ke browser Chrome Anda untuk menguji aplikasi web, debug dengan console logs, dan otomatisasi alur kerja browser tanpa meninggalkan VS Code. Ini memerlukan ekstensi [Claude in Chrome](https://chromewebstore.google.com/detail/claude/fcoeoabgfenejglbffodgkkbkcdhcgfn) versi 1.0.36 atau lebih tinggi.

Ketik `@browser` di kotak prompt diikuti dengan apa yang ingin Anda lakukan Claude:

```text wrap theme={null}
@browser go to localhost:3000 and check the console for errors
```

Anda juga dapat membuka menu lampiran untuk memilih alat browser tertentu seperti membuka tab baru atau membaca konten halaman.

Claude membuka tab baru untuk tugas browser dan berbagi status login browser Anda, sehingga dapat mengakses situs apa pun yang sudah Anda masuki.

Untuk petunjuk penyiapan, daftar lengkap kemampuan, dan pemecahan masalah, lihat [Gunakan Claude Code dengan Chrome](/docs/id/chrome).

<h2 id="vs-code-commands-and-shortcuts">
  Perintah dan pintasan keyboard VS Code
</h2>

Buka Command Palette (`Cmd+Shift+P` di Mac atau `Ctrl+Shift+P` di Windows/Linux) dan ketik "Claude Code" untuk melihat semua perintah VS Code yang tersedia untuk ekstensi Claude Code.

Beberapa pintasan keyboard tergantung pada panel mana yang "fokus" (menerima input keyboard). Ketika kursor Anda berada di file kode, editor fokus. Ketika kursor Anda berada di kotak prompt Claude, Claude fokus. Gunakan `Cmd+Esc` / `Ctrl+Esc` untuk beralih di antara keduanya.

<Note>
  Ini adalah perintah VS Code untuk mengontrol ekstensi. Tidak semua perintah Claude Code bawaan tersedia di ekstensi. Lihat [Ekstensi VS Code vs. Claude Code CLI](#vs-code-extension-vs-claude-code-cli) untuk detail.
</Note>

| Perintah                   | Pintasan Keyboard                                        | Deskripsi                                                                                                                                                                                                                                                                                |
| -------------------------- | -------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Focus Input                | `Cmd+Esc` (Mac) / `Ctrl+Esc` (Windows/Linux)             | Beralih fokus antara editor dan Claude                                                                                                                                                                                                                                                   |
| Focus last message         | -                                                        | Pindahkan fokus keyboard ke pesan terbaru dalam percakapan, atau ke prompt izin yang menunggu, sehingga Anda dapat membaca dari sana dengan keyboard atau pembaca layar. Tidak tersedia dalam [mode terminal](#switch-to-terminal-mode). Memerlukan Claude Code v2.1.268 atau lebih baru |
| Open in Side Bar           | -                                                        | Buka Claude di sidebar                                                                                                                                                                                                                                                                   |
| Open in Terminal           | -                                                        | Buka Claude dalam mode terminal                                                                                                                                                                                                                                                          |
| Open in New Tab            | `Cmd+Shift+Esc` (Mac) / `Ctrl+Shift+Esc` (Windows/Linux) | Buka percakapan baru sebagai tab editor                                                                                                                                                                                                                                                  |
| Open in New Window         | -                                                        | Buka percakapan baru di jendela terpisah                                                                                                                                                                                                                                                 |
| New Conversation           | `Cmd+N` (Mac) / `Ctrl+N` (Windows/Linux)                 | Mulai percakapan baru. Memerlukan Claude fokus dan `enableNewConversationShortcut` diatur ke `true`                                                                                                                                                                                      |
| Reopen Closed Session      | `Cmd+Shift+T` (Mac) / `Ctrl+Shift+T` (Windows/Linux)     | Buka kembali tab sesi Claude yang ditutup paling baru. Jatuh kembali ke pembukaan kembali editor normal VS Code ketika tab yang ditutup terakhir bukan sesi Claude. Nonaktifkan dengan `enableReopenClosedSessionShortcut`                                                               |
| Insert @-Mention Reference | `Option+K` (Mac) / `Alt+K` (Windows/Linux)               | Sisipkan referensi ke file saat ini dan pilihan (memerlukan editor fokus)                                                                                                                                                                                                                |
| Accept Change at Cursor    | -                                                        | Terima perubahan di kursor sambil [meninjau edit yang diusulkan](#get-started) satu perubahan pada satu waktu. Memerlukan Claude Code v2.1.275 atau lebih baru                                                                                                                           |
| Reject Change at Cursor    | -                                                        | Kembalikan perubahan di kursor sambil meninjau edit yang diusulkan satu perubahan pada satu waktu. Memerlukan Claude Code v2.1.275 atau lebih baru                                                                                                                                       |
| Toggle Focus view          | `Ctrl+Option+F` (Mac) / `Ctrl+Alt+F` (Windows/Linux)     | Sembunyikan atau tampilkan aktivitas alat dalam percakapan. Bekerja saat panel Claude atau sidebar terlihat. Memerlukan Claude Code v2.1.221 atau lebih baru                                                                                                                             |
| Rename Session Tab         | -                                                        | Ubah nama sesi di tab Claude aktif. Memerlukan Claude Code v2.1.257 atau lebih baru                                                                                                                                                                                                      |
| Add Session Tab to Group   | -                                                        | Tambahkan sesi di tab Claude aktif ke [grup sesi](#organize-sessions-into-groups) yang Anda pilih atau buat. Memerlukan Claude Code v2.1.257 atau lebih baru                                                                                                                             |
| Mark Session as Unread     | -                                                        | Tandai sesi di tab Claude aktif sebagai belum dibaca dalam daftar sesi. Memerlukan Claude Code v2.1.257 atau lebih baru                                                                                                                                                                  |
| Show Logs                  | -                                                        | Lihat log debug ekstensi                                                                                                                                                                                                                                                                 |
| Logout                     | -                                                        | Keluar dari akun Anthropic Anda                                                                                                                                                                                                                                                          |

<h3 id="launch-a-vs-code-tab-from-other-tools">
  Luncurkan tab VS Code dari alat lain
</h3>

Ekstensi mendaftarkan penanganan URI di `vscode://anthropic.claude-code/open`. Gunakan untuk membuka tab Claude Code baru dari alat Anda sendiri: alias shell, bookmarklet browser, atau skrip apa pun yang dapat membuka URL. Jika VS Code belum berjalan, membuka URL meluncurkannya terlebih dahulu. Jika VS Code sudah berjalan, URL membuka di jendela mana pun yang saat ini fokus.

Panggil penanganan dengan pembuka URL sistem operasi Anda.

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

    Perintah `xdg-open` berasal dari paket `xdg-utils`. Jika shell melaporkan tidak ditemukan, lihat [xdg-open tidak ditemukan di Linux](/docs/id/deep-links#xdg-open-is-not-found-on-linux).
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

Penanganan menerima dua parameter kueri opsional:

| Parameter | Deskripsi                                                                                                                                                                                                                                                                                                                                                           |
| --------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `prompt`  | Teks untuk pra-isi di kotak prompt. Harus dikodekan URL. Prompt pra-isi tetapi tidak dikirim secara otomatis.                                                                                                                                                                                                                                                       |
| `session` | ID sesi untuk melanjutkan alih-alih memulai percakapan baru. Sesi harus milik ruang kerja yang saat ini terbuka di VS Code. Jika sesi tidak ditemukan, percakapan segar dimulai sebagai gantinya. Jika sesi sudah terbuka di tab, tab itu difokuskan. Untuk menangkap ID sesi secara terprogram, lihat [Lanjutkan percakapan](/docs/id/headless#continue-conversations). |

Misalnya, untuk membuka tab yang pra-isi dengan "review my changes":

```text theme={null}
vscode://anthropic.claude-code/open?prompt=review%20my%20changes
```

Ekstensi juga menangani `vscode://anthropic.claude-code/install-plugin`, yang [membuka dialog plugin pada satu plugin](#share-a-plugin-install-link). Untuk meluncurkan sesi terminal alih-alih tab VS Code, gunakan penanganan `claude-cli://` CLI. Lihat [Luncurkan sesi dari tautan](/docs/id/deep-links).

<h2 id="configure-settings">
  Konfigurasi pengaturan
</h2>

Ekstensi memiliki dua jenis pengaturan:

* **Pengaturan ekstensi** di VS Code: mengontrol perilaku ekstensi dalam VS Code. Buka dengan `Cmd+,` (Mac) atau `Ctrl+,` (Windows/Linux), kemudian buka Extensions → Claude Code. Anda juga dapat mengetik `/` dan memilih **General config…** untuk membuka pengaturan.
* **Pengaturan Claude Code** di `~/.claude/settings.json`: dibagikan antara ekstensi dan CLI. Gunakan untuk perintah yang diizinkan, variabel lingkungan, hooks, dan server MCP. Pada paket Pro, Max, dan Team, ini juga merupakan salah satu input untuk mode izin percakapan dimulai. [Switch permission modes](/docs/id/permission-modes#switch-permission-modes) mencantumkan urutannya. Lihat [Settings](/docs/id/settings) untuk detail.

<Tip>
  Tambahkan `"$schema": "https://json.schemastore.org/claude-code-settings.json"` ke `settings.json` Anda untuk mendapatkan autocomplete dan validasi inline untuk semua pengaturan yang tersedia langsung di VS Code.
</Tip>

<h3 id="extension-settings">
  Pengaturan ekstensi
</h3>

VS Code membaca `initialPermissionMode` dari pengaturan pengguna Anda dan mengabaikan nilai workspace. Sebelum v2.1.225, VS Code menganggap pengaturan default ke `default` dan menerapkan nilai workspace.

| Pengaturan                          | Default | Deskripsi                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| ----------------------------------- | ------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `useTerminal`                       | `false` | Luncurkan Claude dalam mode terminal alih-alih panel grafis                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `initialPermissionMode`             | -       | Mengontrol prompt persetujuan untuk percakapan baru: `default`, `plan`, `acceptEdits`, atau `bypassPermissions`. `manual` adalah alias untuk `default` dan memilih mode yang diberi label **Manual** dalam indikator mode. Ketika Anda membiarkannya tidak diatur, ekstensi memilih mode izin awal seperti yang dijelaskan dalam [Switch permission modes](/docs/id/permission-modes#switch-permission-modes).                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `preferredLocation`                 | `panel` | Tempat Claude membuka: `sidebar` (kanan) atau `panel` (tab baru)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `lockEditorGroups`                  | `true`  | [Kunci grup editor yang dimulai Claude untuk tabnya](#choose-where-claude-lives), sehingga file yang Anda buka saat tab Claude difokuskan pergi ke grup lain. Ketika dimatikan, ekstensi tidak pernah mengunci grup editor. Memerlukan Claude Code v2.1.274 atau lebih baru                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `autosave`                          | `true`  | Simpan file secara otomatis sebelum Claude membaca atau menulisnya                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `attachOpenFile`                    | `true`  | Tambahkan file yang terbuka di editor ke pesan Anda dan tampilkan di kotak prompt. Ketika dimatikan, hanya teks yang dipilih yang ditambahkan. Memerlukan Claude Code v2.1.271 atau lebih baru                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `useCtrlEnterToSend`                | `false` | Gunakan Ctrl/Cmd+Enter alih-alih Enter untuk mengirim prompt                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `scrollToBottomOnSend`              | `true`  | Gulir percakapan ke bawah saat Anda mengirim pesan. Ketika dimatikan, percakapan tetap berada di tempat Anda meninggalkannya. Memerlukan Claude Code v2.1.275 atau lebih baru                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `enableNewConversationShortcut`     | `false` | Aktifkan Cmd/Ctrl+N untuk memulai percakapan baru                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `enableReopenClosedSessionShortcut` | `true`  | Gunakan Cmd/Ctrl+Shift+T untuk membuka kembali tab sesi Claude yang paling baru ditutup. Ketika tab terakhir yang ditutup bukan sesi Claude, pintasan menjalankan perintah reopen-closed-editor normal VS Code.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `archiveInactiveSessions`           | `14`    | [Arsipkan sesi secara otomatis](#resume-past-conversations) setelah jumlah hari tanpa aktivitas ini: `1`, `2`, `7`, atau `14`. Atur `0` untuk mematikannya. Memerlukan Claude Code v2.1.265 atau lebih baru                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `continueAfterReload`               | `true`  | Setelah reload jendela, Claude [melanjutkan langkah yang terputus](#choose-where-claude-lives) dalam sesi yang dipulihkan. Memerlukan Claude Code v2.1.274 atau lebih baru                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `hideOnboarding`                    | `false` | Sembunyikan daftar periksa onboarding (ikon topi kelulusan)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `focusView`                         | `false` | Sembunyikan panggilan alat, hasil alat, dan pemikiran di balik baris yang dapat diperluas, meninggalkan prompt Anda dan respons Claude. Daftar tugas terbaru Claude tetap terlihat; ini memerlukan Claude Code v2.1.225 atau lebih baru. Anda juga dapat mengalihkan Focus view dari menu perintah. Memerlukan Claude Code v2.1.221 atau lebih baru                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `respectGitIgnore`                  | `true`  | Kecualikan pola .gitignore dari pencarian file dan dari [konteks seleksi](#reference-files-and-folders)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `usePythonEnvironment`              | `true`  | Aktifkan lingkungan Python workspace saat menjalankan Claude. Memerlukan ekstensi Python.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `environmentVariables`              | `[]`    | Atur variabel lingkungan untuk proses Claude. Gunakan pengaturan Claude Code sebagai gantinya untuk konfigurasi bersama.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `disableLoginPrompt`                | `false` | Lewati prompt autentikasi (untuk pengaturan penyedia pihak ketiga)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `allowDangerouslySkipPermissions`   | `false` | Menambahkan Bypass permissions ke pemilih mode. Gunakan hanya di sandbox tanpa akses internet.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `claudeProcessWrapper`              | -       | Executable yang digunakan untuk meluncurkan proses Claude. Jalur biner bundel diteruskan sebagai argumen saat ada. Atur ini ke biner `claude` yang dipasang secara terpisah jika build ekstensi tidak menyertakan satu untuk platform Anda. Dalam pengaturan terbungkus, percakapan dimulai dalam mode Manual kecuali Anda menetapkan `initialPermissionMode` atau memilih Manual, Edit automatically, atau Auto dalam percakapan sebelumnya, karena ekstensi melewati pengaturan dan langkah default bawaan di sana; lihat [Switch permission modes](/docs/id/permission-modes#switch-permission-modes). Kesalahan "Unsupported platform" saat aktivasi berarti tidak ada biner yang bundel untuk platform Anda; lihat [which platforms have prebuilt binaries](/docs/id/troubleshoot-install#native-binary-not-found-after-npm-install). |

<h2 id="use-a-screen-reader">
  Gunakan pembaca layar
</h2>

Panel obrolan ekstensi berfungsi dengan pembaca layar. Anda tidak perlu mengaktifkan apa pun: ekstensi mengumumkan aktivitas percakapan untuk setiap pengguna, tanpa perubahan visual. Ini terpisah dari [mode pembaca layar](/docs/id/accessibility) CLI yang bersifat opt-in, yang menyesuaikan antarmuka terminal.

Dukungan pembaca layar di panel obrolan memerlukan Claude Code v2.1.236 atau lebih baru.

Selama percakapan, ekstensi mengumumkan:

* **Balasan Claude**: ekstensi mengumumkan setiap balasan sekali, ketika selesai, dan tetap diam saat teks mengalir masuk. Pembaca layar Anda membaca blok kode sebagai ringkasan jumlah baris, membaca tautan berdasarkan label mereka, dan membaca tabel sel demi sel; balasan lengkap tetap dapat dibaca dalam transkrip.
* **Permintaan izin dan pertanyaan**: ekstensi mengumumkan permintaan ketika prompt izinnya muncul, menyebutkan alat yang ingin digunakan Claude. Ini mengumumkan dengan cara yang sama ketika Claude mengajukan pertanyaan kepada Anda dan ketika Claude menyelesaikan rencana dan menunggu ulasan Anda.
* **Perubahan status**: ekstensi mengumumkan ketika Claude mulai bekerja, ketika Claude siap untuk input Anda, dan ketika Claude Code mulai mengompres percakapan.
* **Kesalahan dan prompt model**: ekstensi mengumumkan kesalahan dalam percakapan, dan mengumumkan ketika [prompt persetujuan kredit penggunaan](/docs/id/model-config#fable-and-usage-credits) atau [prompt permintaan yang ditandai](/docs/id/model-config#ask-before-switching) muncul.

Saat Claude bekerja, pembaca layar Anda membaca label teks sebagai pengganti animasi spinner kemajuan.

Ketika Anda membuka kembali sesi atau beralih ke sesi lain, ekstensi tidak mengumumkan apa pun: riwayat yang dipulihkan, prompt izin yang tertunda, dan status yang sedang berlangsung tetap diam sampai sesuatu yang baru terjadi.

<h3 id="use-the-chat-panel-from-the-keyboard">
  Gunakan panel obrolan dari keyboard
</h3>

Setiap giliran dalam transkrip dimulai dengan judul yang tersembunyi secara visual yang diberi label dengan prompt yang memulai giliran, sehingga Anda dapat melompat antar giliran dengan navigasi judul pembaca layar Anda.

Dalam satu giliran, pembaca layar Anda mengumumkan pesan siapa yang sedang Anda baca saat Anda bergerak melaluinya:

* **Pesan Anda**: "Anda"
* **Pesan Claude**: "Claude"
* **Langkah alat**: "Claude" ditambah nama alat, seperti "Claude, Bash"
* **Blok pemikiran**: "Claude, thinking"

Karena ekstensi mengeksposnya sebagai wilayah berlabel, Anda juga dapat memindahkan fokus ke transkrip itu sendiri dengan `Tab` dan membacanya dengan kecepatan Anda sendiri. Untuk memindahkan fokus ke pesan terbaru atau prompt izin yang menunggu, jalankan **Claude Code: Focus last message** dari [Command Palette](#vs-code-commands-and-shortcuts).

Ketika opsi pada prompt izin menyimpan aturan izin atau akses direktori, labelnya diakhiri dengan menyebutkan tempat persetujuan disimpan, seperti "semua proyek" atau "sesi ini". Dengan opsi yang difokuskan, tekan tombol panah `Left` atau `Right` untuk mengubah tujuan, dan ekstensi mengumumkan setiap tujuan saat Anda berpindah ke tujuan tersebut. Anda juga dapat mengklik tujuan dalam label. Tombol panah memerlukan Claude Code v2.1.268 atau lebih baru.

<h2 id="vs-code-extension-vs-claude-code-cli">
  Ekstensi VS Code vs. Claude Code CLI
</h2>

Claude Code tersedia sebagai ekstensi VS Code (panel grafis) dan CLI (antarmuka baris perintah di terminal). Beberapa fitur hanya tersedia di CLI. Jika Anda memerlukan fitur khusus CLI, jalankan `claude` di terminal terintegrasi VS Code. Ini memerlukan [instalasi CLI mandiri](/docs/id/setup): ekstensi tidak menambahkan `claude` ke PATH Anda. Lihat [Jalankan CLI di VS Code](#run-cli-in-vs-code).

| Fitur                  | CLI                   | Ekstensi VS Code                                                                                    |
| ---------------------- | --------------------- | --------------------------------------------------------------------------------------------------- |
| Perintah dan skills    | [Semua](/docs/id/commands) | Subset (ketik `/` untuk melihat yang tersedia)                                                      |
| Konfigurasi server MCP | Ya                    | Ya ([tambahkan dan kelola server](#connect-to-external-tools-with-mcp) dengan `/mcp` di panel chat) |
| Checkpoints            | Ya                    | Ya                                                                                                  |
| Pintasan bash `!`      | Ya                    | Tidak                                                                                               |
| Penyelesaian tab       | Ya                    | Tidak                                                                                               |

<h3 id="rewind-with-checkpoints">
  Rewind dengan checkpoints
</h3>

Ekstensi VS Code mendukung checkpoints, yang melacak pengeditan file Claude dan memungkinkan Anda untuk kembali ke keadaan sebelumnya. Arahkan kursor ke pesan apa pun untuk mengungkapkan tombol rewind, kemudian pilih dari tiga opsi:

* **Fork conversation from here**: mulai cabang percakapan baru dari pesan ini sambil mempertahankan semua perubahan kode
* **Rewind code to here**: kembalikan perubahan file ke titik ini dalam percakapan sambil mempertahankan riwayat percakapan lengkap
* **Fork conversation and rewind code**: mulai cabang percakapan baru dan kembalikan perubahan file ke titik ini

Untuk detail lengkap tentang cara kerja checkpoints dan keterbatasannya, lihat [Checkpointing](/docs/id/checkpointing).

<h3 id="run-cli-in-vs-code">
  Jalankan CLI di VS Code
</h3>

Untuk menggunakan CLI sambil tetap berada di VS Code, buka terminal terintegrasi (`` Ctrl+` `` di Windows/Linux atau `` Cmd+` `` di Mac) dan jalankan `claude`. CLI secara otomatis terintegrasi dengan IDE Anda untuk fitur seperti tampilan diff dan berbagi diagnostik.

Menginstal ekstensi tidak menempatkan `claude` di PATH shell Anda. Ekstensi menggabungkan salinan pribadi CLI untuk panel chatnya, tetapi mengetik `claude` di terminal memerlukan [instalasi CLI mandiri](/docs/id/setup). Jalankan instalasi sekali dan perintah di halaman ini, termasuk `claude mcp add` dan `claude --resume`, bekerja di terminal apa pun. Jika `claude` masih tidak ditemukan setelah menginstal, [verifikasi PATH Anda](/docs/id/troubleshoot-install#verify-your-path).

Jika menggunakan terminal eksternal, jalankan `/ide` di dalam Claude Code untuk menghubungkannya ke VS Code.

<h3 id="switch-between-extension-and-cli">
  Beralih antara ekstensi dan CLI
</h3>

Ekstensi dan CLI berbagi riwayat percakapan yang sama. Untuk melanjutkan percakapan ekstensi di CLI, jalankan `claude --resume` di terminal. Ini membuka pemilih interaktif di mana Anda dapat mencari dan memilih percakapan Anda.

<h3 id="include-terminal-output-in-prompts">
  Sertakan output terminal dalam prompt
</h3>

Referensikan output terminal dalam prompt Anda menggunakan `@terminal:name` di mana `name` adalah judul terminal. Ini memungkinkan Claude melihat output perintah, pesan kesalahan, atau log tanpa menyalin dan menempel.

<h3 id="monitor-background-processes">
  Pantau proses latar belakang
</h3>

Ketik `/tasks` di kotak prompt untuk membuka [peta agen](#use-the-prompt-box), yang mencantumkan tugas latar belakang sesi, seperti server dev yang Claude biarkan berjalan sebagai perintah shell latar belakang. Klik tugas untuk membuka kartunya dan hentikan di sana. Memerlukan Claude Code v2.1.277 atau lebih baru.

<h3 id="connect-to-external-tools-with-mcp">
  Hubungkan ke alat eksternal dengan MCP
</h3>

Server MCP (Model Context Protocol) memberi Claude akses ke alat eksternal, database, dan API.

Untuk mengelola server MCP tanpa meninggalkan VS Code, ketik `/mcp` di panel chat. Dari dialog yang terbuka, Anda dapat menambahkan server, menghapus server yang disimpan di [scope](/docs/id/mcp#mcp-installation-scopes) lokal, pengguna, atau proyek, mengaktifkan atau menonaktifkan server, terhubung kembali ke server, dan mengelola autentikasi OAuth. Menambahkan dan menghapus server di dialog memerlukan Claude Code v2.1.261 atau lebih baru.

Anda juga dapat menjalankan `claude mcp add` di terminal terintegrasi VS Code (`` Ctrl+` `` atau `` Cmd+` ``). Dialog dan perintah terminal menyimpan ke konfigurasi MCP yang sama, dan perubahan dari salah satu berlaku dalam percakapan yang Anda mulai setelahnya. Contoh di bawah menambahkan server MCP jarak jauh GitHub, yang melakukan autentikasi dengan [token akses pribadi](https://github.com/settings/personal-access-tokens) yang diteruskan sebagai header:

```bash theme={null}
claude mcp add --transport http github https://api.githubcopilot.com/mcp/ \
  --header "Authorization: Bearer YOUR_GITHUB_PAT"
```

Ganti `YOUR_GITHUB_PAT` dengan token akses pribadi Anda. Perintah `claude mcp add` menyimpan konfigurasi tanpa memvalidasi kredensial, jadi nilai placeholder diterima di sini tetapi server gagal terhubung nanti. Untuk memverifikasi koneksi, mulai percakapan baru, ketik `/mcp`, dan periksa bahwa server menunjukkan **Connected**. Server dengan kredensial buruk menunjukkan **Failed**.

Setelah dikonfigurasi, minta Claude untuk menggunakan alat (misalnya, "Review PR #456").

Untuk menemukan server yang akan dihubungkan, lihat [Temukan dan bangun server MCP](/docs/id/mcp#find-and-build-mcp-servers).

<h2 id="work-with-git">
  Bekerja dengan git
</h2>

Claude Code terintegrasi dengan git untuk membantu alur kerja kontrol versi langsung di VS Code. Minta Claude untuk melakukan commit perubahan, membuat pull request, atau bekerja di berbagai branch. Untuk memulai Claude dalam worktree terisolasi dengan file dan branch-nya sendiri, lihat [Jalankan sesi paralel dengan worktrees](/docs/id/worktrees).

<h3 id="create-commits-and-pull-requests">
  Buat commit dan pull request
</h3>

Claude dapat melakukan staging perubahan, menulis pesan commit, dan membuat pull request berdasarkan pekerjaan Anda:

```text wrap theme={null}
commit my changes with a descriptive message
create a pr for this feature
summarize the changes I've made to the auth module
```

Saat membuat pull request, Claude menghasilkan deskripsi berdasarkan perubahan kode aktual dan dapat menambahkan konteks tentang pengujian atau keputusan implementasi.

<h2 id="use-third-party-providers">
  Gunakan penyedia pihak ketiga
</h2>

Secara default, Claude Code terhubung langsung ke API Anthropic. Jika organisasi Anda menggunakan Amazon Bedrock, Google Cloud's Agent Platform, atau Microsoft Foundry untuk mengakses Claude, konfigurasikan ekstensi untuk menggunakan penyedia Anda sebagai gantinya:

<Steps>
  <Step title="Nonaktifkan prompt login">
    Buka [pengaturan Disable Login Prompt](vscode://settings/claudeCode.disableLoginPrompt) dan centang kotak tersebut.

    Anda juga dapat membuka pengaturan VS Code (`Cmd+,` di Mac atau `Ctrl+,` di Windows/Linux), cari "Claude Code login", dan centang **Disable Login Prompt**.
  </Step>

  <Step title="Konfigurasikan penyedia Anda">
    Ikuti panduan penyiapan untuk penyedia Anda:

    * [Claude Code on Amazon Bedrock](/docs/id/amazon-bedrock)
    * [Claude Code on Google Cloud's Agent Platform](/docs/id/google-vertex-ai)
    * [Claude Code on Microsoft Foundry](/docs/id/microsoft-foundry)

    Panduan ini mencakup konfigurasi penyedia Anda di `~/.claude/settings.json`, yang memastikan pengaturan Anda dibagikan antara ekstensi VS Code dan CLI.
  </Step>
</Steps>

Pada penyedia pihak ketiga, ekstensi tidak menawarkan fitur yang memerlukan akun claude.ai, seperti bilah penggunaan rencana, [voice dictation](/docs/id/voice-dictation), dan tab Web untuk [sesi cloud](#resume-cloud-sessions-from-claude-ai). Untuk apa yang ditampilkan dialog Akun & penggunaan pada masuk ini, lihat [Periksa akun dan penggunaan](#check-account-and-usage).

Masuk claude.ai yang tersisa dari `/login` sebelumnya tetap tidak digunakan: ekstensi tidak mengirimkannya dengan permintaan apa pun.

<h2 id="security-and-privacy">
  Keamanan dan privasi
</h2>

Kode Anda tetap pribadi. Claude Code memproses kode Anda untuk memberikan bantuan tetapi tidak menggunakannya untuk melatih model. Untuk detail tentang penanganan data dan cara untuk tidak ikut serta dalam pencatatan, lihat [Data dan privasi](/docs/id/data-usage).

Dengan izin auto-edit diaktifkan, Claude Code dapat memodifikasi file konfigurasi VS Code (seperti `settings.json` atau `tasks.json`) yang mungkin dijalankan secara otomatis oleh VS Code. Untuk mengurangi risiko saat bekerja dengan kode yang tidak terpercaya:

* Aktifkan [VS Code Restricted Mode](https://code.visualstudio.com/docs/editor/workspace-trust#_restricted-mode) untuk workspace yang tidak terpercaya
* Gunakan mode Manual alih-alih Edit automatically atau Auto untuk edit
* Tinjau perubahan dengan hati-hati sebelum menerimanya

<h3 id="the-built-in-ide-mcp-server">
  Server MCP IDE bawaan
</h3>

Ketika ekstensi aktif, ia menjalankan server MCP lokal yang terhubung secara otomatis oleh CLI. Ini adalah cara CLI membuka diff di penampil diff asli VS Code, membaca pilihan Anda saat ini untuk penyebutan `@`, dan — ketika Anda bekerja di notebook Jupyter — meminta VS Code untuk menjalankan sel.

Server bernama `ide` dan tersembunyi dari `/mcp` karena tidak ada yang perlu dikonfigurasi. Namun, jika organisasi Anda menggunakan hook `PreToolUse` untuk membuat daftar putih alat MCP, Anda perlu mengetahui bahwa itu ada.

**Konteks pilihan dan file terbuka.** Saat terhubung, CLI menyertakan pilihan editor Anda saat ini dan jalur file aktif sebagai konteks pada setiap prompt yang Anda kirim. Transkrip menunjukkan baris `⧉ Selected N lines from <file>` ketika ini terjadi.

Untuk mengecualikan file sensitif seperti `.env`, tambahkan [aturan deny `Read`](/docs/id/permissions#read-and-edit) untuk jalurnya. Aturan deny yang cocok mencegah baik teks yang dipilih maupun pemberitahuan file terbuka untuk file tersebut dari mencapai Claude.

Jika Anda mematikan pengaturan [Attach Open File](#extension-settings), CLI menerima jalur file aktif hanya saat Anda memiliki teks yang dipilih di dalamnya.

**Transportasi dan autentikasi.** Server mengikat ke `127.0.0.1` pada port acak dalam rentang 10000–65535, dan port tidak dapat dikonfigurasi. Transportasi adalah `ws://` yang tidak terenkripsi; karena soket hanya loopback, proses apa pun yang dapat menangkap lalu lintas juga dapat membaca token dari file kunci, jadi TLS tidak akan menambah perlindungan. Setiap aktivasi ekstensi menghasilkan token autentikasi acak segar, menulisnya ke file kunci di `~/.claude/ide/<port>.lock`, dan CLI harus menyajikannya sebagai header `X-Claude-Code-Ide-Authorization` untuk terhubung. File kunci memiliki izin `0600` dalam direktori `0700`, jadi hanya pengguna yang menjalankan VS Code yang dapat membacanya. Jika `CLAUDE_CONFIG_DIR` diatur, file kunci ditulis ke `$CLAUDE_CONFIG_DIR/ide/` sebagai gantinya.

**Alat yang diekspos ke model.** Server menampilkan selusin alat, tetapi hanya dua yang terlihat oleh model. Sisanya adalah RPC internal yang digunakan CLI untuk UI-nya sendiri — membuka diff, membaca pilihan, menyimpan file — dan disaring sebelum daftar alat mencapai Claude.

| Nama alat (seperti yang terlihat oleh hook) | Apa yang dilakukannya                                                                                                                   | Hanya baca |
| ------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------- | ---------- |
| `mcp__ide__getDiagnostics`                  | Mengembalikan diagnostik language-server — kesalahan dan peringatan di panel Problems VS Code. Secara opsional dibatasi pada satu file. | Ya         |
| `mcp__ide__executeCode`                     | Menjalankan kode Python di kernel notebook Jupyter yang aktif. Lihat alur konfirmasi di bawah.                                          | Tidak      |

**Eksekusi Jupyter selalu bertanya terlebih dahulu.** `mcp__ide__executeCode` tidak dapat menjalankan apa pun secara diam-diam. Pada setiap panggilan, kode dimasukkan sebagai sel baru di akhir notebook aktif, VS Code menggulirnya ke tampilan, dan Quick Pick asli meminta Anda untuk **Execute** atau **Cancel**. Membatalkan — atau menutup picker dengan `Esc` — mengembalikan kesalahan ke Claude dan tidak ada yang berjalan. Alat ini juga menolak dengan tegas ketika tidak ada notebook aktif, ketika ekstensi Jupyter (`ms-toolsai.jupyter`) tidak diinstal, atau ketika kernel bukan Python.

<Note>
  Konfirmasi Quick Pick terpisah dari hook `PreToolUse`. Entri daftar putih untuk `mcp__ide__executeCode` memungkinkan Claude *mengusulkan* menjalankan sel; Quick Pick di dalam VS Code adalah yang memungkinkannya *benar-benar* berjalan.
</Note>

<a id="troubleshooting" />

<h2 id="fix-common-issues">
  Perbaiki masalah umum
</h2>

<h3 id="extension-won’t-install">
  Ekstensi tidak akan dipasang
</h3>

* Pastikan Anda memiliki versi VS Code yang kompatibel (1.94.0 atau lebih baru)
* Periksa bahwa VS Code memiliki izin untuk memasang ekstensi
* Coba pasang langsung dari [VS Code Marketplace](https://marketplace.visualstudio.com/items?itemName=anthropic.claude-code)

<h3 id="spark-icon-not-visible">
  Ikon Spark tidak terlihat
</h3>

Ikon Spark muncul di **Editor Toolbar** (sudut kanan atas editor) ketika Anda memiliki file yang terbuka. Jika Anda tidak melihatnya:

1. **Buka file**: Ikon memerlukan file untuk dibuka. Hanya membuka folder saja tidak cukup.
2. **Periksa versi VS Code**: Memerlukan 1.94.0 atau lebih tinggi (Help → About)
3. **Mulai ulang VS Code**: Jalankan "Developer: Reload Window" dari Command Palette
4. **Nonaktifkan ekstensi yang bertentangan**: Nonaktifkan sementara ekstensi AI lainnya (Cline, Continue, dll.)
5. **Periksa kepercayaan workspace**: Ekstensi tidak berfungsi dalam Mode Terbatas

Alternatifnya, jika Anda telah menetapkan [`preferredLocation`](#extension-settings) ke `sidebar`, atau membuka Claude dengan **Claude Code: Open in Side Bar**, klik "✻ Claude Code" di **Status Bar** (sudut kanan bawah). Ini berfungsi bahkan tanpa file yang terbuka. Anda juga dapat menggunakan **Command Palette** (`Cmd+Shift+P` / `Ctrl+Shift+P`) dan ketik "Claude Code".

<h3 id="cmd-esc-does-nothing-on-macos">
  Cmd+Esc tidak melakukan apa pun di macOS
</h3>

Di macOS Tahoe dan lebih baru, pintasan Game Overlay sistem terikat ke `Cmd+Esc` secara default dan mencegat penekanan tombol sebelum mencapai VS Code. Untuk membebaskan pintasan:

1. Buka System Settings
2. Buka Keyboard, kemudian Keyboard Shortcuts, kemudian Game Controllers
3. Hapus centang Game Overlay

Alternatifnya, ikat ulang ekstensi ke tombol yang berbeda: buka editor [Keyboard Shortcuts](https://code.visualstudio.com/docs/configure/keybindings) VS Code (`Cmd+K Cmd+S`), cari `Claude Code: Focus input`, dan tetapkan pengikatan baru.

<h3 id="claude-code-never-responds">
  Claude Code tidak pernah merespons
</h3>

Jika Claude Code tidak merespons prompt Anda:

1. **Periksa koneksi internet Anda**: Pastikan Anda memiliki koneksi internet yang stabil
2. **Mulai percakapan baru**: Coba mulai percakapan baru untuk melihat apakah masalah berlanjut
3. **Coba CLI**: Jalankan `claude` dari terminal untuk melihat apakah Anda mendapatkan pesan kesalahan yang lebih terperinci

Jika masalah berlanjut, [buat laporan masalah di GitHub](https://github.com/anthropics/claude-code/issues) dengan detail tentang kesalahan.

<h2 id="uninstall-the-extension">
  Uninstall the extension
</h2>

Untuk menguninstall ekstensi Claude Code:

1. Buka tampilan Extensions (`Cmd+Shift+X` di Mac atau `Ctrl+Shift+X` di Windows/Linux)
2. Cari "Claude Code"
3. Klik **Uninstall**

Jika Anda menjalankan `claude` di terminal terintegrasi VS Code, Claude Code akan menginstal ulang ekstensi secara otomatis. Untuk tetap menguninstallnya, matikan **Auto-install IDE extension** di `/config`, atau atur [`autoInstallIdeExtension`](/docs/id/settings-reference#autoinstallideextension) ke `false`. Anda juga dapat mengatur variabel lingkungan [`CLAUDE_CODE_IDE_SKIP_AUTO_INSTALL`](/docs/id/env-vars) ke `1`.

Untuk juga menghapus data ekstensi dan mengatur ulang semua pengaturan, hapus direktori penyimpanan ekstensi untuk platform Anda.

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

Untuk bantuan tambahan, lihat [panduan pemecahan masalah](/docs/id/troubleshooting).

<h2 id="next-steps">
  Langkah berikutnya
</h2>

Sekarang Anda telah menyiapkan Claude Code di VS Code:

* [Jelajahi alur kerja umum](/docs/id/common-workflows) untuk mendapatkan hasil maksimal dari Claude Code
* [Siapkan MCP servers](/docs/id/mcp) untuk memperluas kemampuan Claude dengan alat eksternal. Tambahkan dan kelola dengan `/mcp` di panel chat.
* [Konfigurasi pengaturan Claude Code](/docs/id/settings) untuk menyesuaikan perintah yang diizinkan, hooks, dan lainnya. Pengaturan ini dibagikan antara ekstensi dan CLI.
