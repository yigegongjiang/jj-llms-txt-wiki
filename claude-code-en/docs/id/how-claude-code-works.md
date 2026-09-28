> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Cara Kerja Claude Code

> Pahami loop agentic, tools bawaan, dan bagaimana Claude Code berinteraksi dengan proyek Anda.

Claude Code adalah asisten agentic yang berjalan di terminal Anda. Meskipun unggul dalam coding, Claude Code dapat membantu dengan apa pun yang dapat Anda lakukan dari command line: menulis dokumentasi, menjalankan build, mencari file, meneliti topik, dan banyak lagi.

Panduan ini mencakup arsitektur inti, kemampuan bawaan, dan [tips untuk bekerja secara efektif dengan Claude Code](#work-effectively-with-claude-code). Untuk panduan langkah demi langkah, lihat [Common workflows](/docs/id/common-workflows). Untuk fitur extensibility seperti skills, MCP, dan hooks, lihat [Extend Claude Code](/docs/id/features-overview).

<h2 id="the-agentic-loop">
  Loop agentic
</h2>

Ketika Anda memberikan tugas kepada Claude, Claude bekerja melalui tiga fase: **mengumpulkan konteks**, **mengambil tindakan**, dan **memverifikasi hasil**. Fase-fase ini berpadu bersama. Claude menggunakan tools di seluruh proses, baik mencari file untuk memahami kode Anda, mengedit untuk membuat perubahan, atau menjalankan test untuk memeriksa pekerjaannya.

<img src="https://mintcdn.com/claude-code/ikqp3_70mqIahteV/images/agentic-loop.svg?fit=max&auto=format&n=ikqp3_70mqIahteV&q=85&s=4a30fb7ce2815012a9f27c955e2c6bb0" className="dark:hidden" alt="Diagram loop agentic: Prompt Anda mengarah ke Claude mengumpulkan konteks, mengambil tindakan, memverifikasi hasil, dan mengulangi sampai tugas selesai. Anda dapat mengganggu kapan saja." width="720" height="280" data-path="images/agentic-loop.svg" />

<img src="https://mintcdn.com/claude-code/_xqph1dUOslCOwsj/images/agentic-loop-dark.svg?fit=max&auto=format&n=_xqph1dUOslCOwsj&q=85&s=75e1d55ed76857a952f9a2dffbab02df" className="hidden dark:block" alt="Diagram loop agentic: Prompt Anda mengarah ke Claude mengumpulkan konteks, mengambil tindakan, memverifikasi hasil, dan mengulangi sampai tugas selesai. Anda dapat mengganggu kapan saja." width="720" height="280" data-path="images/agentic-loop-dark.svg" />

Loop beradaptasi dengan apa yang Anda minta. Pertanyaan tentang codebase Anda mungkin hanya memerlukan pengumpulan konteks. Perbaikan bug melakukan siklus melalui ketiga fase berulang kali. Refactor mungkin melibatkan verifikasi ekstensif. Claude memutuskan apa yang setiap langkah perlukan berdasarkan apa yang dipelajarinya dari langkah sebelumnya, menghubungkan puluhan tindakan bersama-sama dan melakukan koreksi jalur di sepanjang jalan.

Anda juga bagian dari loop ini. Anda dapat mengganggu kapan saja untuk mengarahkan Claude ke arah yang berbeda, memberikan konteks tambahan, atau memintanya mencoba pendekatan yang berbeda. Claude bekerja secara otonom tetapi tetap responsif terhadap input Anda.

Loop agentic didukung oleh dua komponen: [models](#models) yang bernalar dan [tools](#tools) yang bertindak. Claude Code adalah lapisan di sekitar model yang menyediakan tools dan mengelola konteks yang dilihat model. Lapisan sekitarnya ini adalah apa yang istilah agentic harness mengacu pada.

<h3 id="models">
  Models
</h3>

Claude Code menggunakan model Claude untuk memahami kode Anda dan bernalar tentang tugas. Claude dapat membaca kode dalam bahasa apa pun, memahami bagaimana komponen terhubung, dan mengetahui apa yang perlu berubah untuk mencapai tujuan Anda. Untuk tugas kompleks, Claude memecah pekerjaan menjadi langkah-langkah, menjalankannya, dan menyesuaikan berdasarkan apa yang dipelajarinya.

[Multiple models](/docs/id/model-config) tersedia dengan trade-off yang berbeda. Sonnet menangani sebagian besar tugas coding dengan baik. Opus memberikan penalaran yang lebih kuat untuk keputusan arsitektur yang kompleks. Beralih dengan `/model` selama sesi atau mulai dengan `claude --model <name>`.

Ketika panduan ini mengatakan "Claude memilih" atau "Claude memutuskan," itu adalah model yang melakukan penalaran.

<h3 id="tools">
  Tools
</h3>

Tools adalah apa yang membuat Claude Code agentic. Tanpa tools, Claude hanya dapat merespons dengan teks. Dengan tools, Claude dapat bertindak: membaca kode Anda, mengedit file, menjalankan perintah, mencari web, dan berinteraksi dengan layanan eksternal. Setiap penggunaan tool mengembalikan informasi yang umpan balik ke dalam loop, menginformasikan keputusan Claude berikutnya.

Tools bawaan umumnya terbagi menjadi lima kategori, masing-masing mewakili jenis agency yang berbeda.

| Kategori              | Apa yang dapat dilakukan Claude                                                                                                                                |
| --------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **File operations**   | Membaca file, mengedit kode, membuat file baru, mengganti nama dan mengorganisir ulang                                                                         |
| **Search**            | Menemukan file berdasarkan pola, mencari konten dengan regex, menjelajahi codebase                                                                             |
| **Execution**         | Menjalankan perintah shell, memulai server, menjalankan test, menggunakan git                                                                                  |
| **Web**               | Mencari web, mengambil dokumentasi, mencari pesan error                                                                                                        |
| **Code intelligence** | Melihat type error dan warning setelah edit, melompat ke definisi, menemukan referensi (memerlukan [code intelligence plugins](/docs/id/plugins/code-intelligence)) |

Ini adalah kemampuan utama. Claude juga memiliki tools untuk spawning subagents, mengajukan pertanyaan kepada Anda, dan tugas orchestration lainnya. Lihat [Tools available to Claude](/docs/id/tools-reference) untuk daftar lengkap.

Claude memilih tools mana yang akan digunakan berdasarkan prompt Anda dan apa yang dipelajarinya di sepanjang jalan. Ketika Anda mengatakan "perbaiki test yang gagal," Claude mungkin:

1. Menjalankan test suite untuk melihat apa yang gagal
2. Membaca output error
3. Mencari file sumber yang relevan
4. Membaca file tersebut untuk memahami kode
5. Mengedit file untuk memperbaiki masalah
6. Menjalankan test lagi untuk memverifikasi

Setiap penggunaan tool memberikan Claude informasi baru yang menginformasikan langkah berikutnya. Ini adalah loop agentic dalam aksi.

**Memperluas kemampuan dasar:** Tools bawaan adalah fondasi. Anda dapat memperluas apa yang diketahui Claude dengan [skills](/docs/id/skills), terhubung ke layanan eksternal dengan [MCP](/docs/id/mcp), mengotomatisasi workflow dengan [hooks](/docs/id/hooks), dan mendelegasikan tugas ke [subagents](/docs/id/sub-agents). Ekstensi ini membentuk lapisan di atas loop agentic inti. Lihat [Extend Claude Code](/docs/id/features-overview) untuk panduan memilih ekstensi yang tepat untuk kebutuhan Anda.

<h2 id="what-claude-can-access">
  Apa yang dapat diakses Claude
</h2>

Ketika Anda menjalankan `claude` di direktori, Claude Code mendapatkan akses ke:

* **Proyek Anda.** File di direktori dan subdirektori Anda, ditambah file di tempat lain dengan izin Anda.
* **Terminal Anda.** Perintah apa pun yang dapat Anda jalankan: build tools, git, package managers, system utilities, scripts. Jika Anda dapat melakukannya dari command line, Claude juga dapat.
* **Status git Anda.** Branch saat ini, perubahan yang belum di-commit, dan riwayat commit terbaru.
* **[CLAUDE.md](/docs/id/memory) Anda.** File markdown tempat Anda menyimpan instruksi khusus proyek, konvensi, dan konteks yang harus diketahui Claude setiap sesi. Jika repositori Anda memiliki AGENTS.md untuk agen coding lainnya, Claude [dapat membacanya](/docs/id/memory#agents-md) sendiri atau bersama CLAUDE.md.
* **[Auto memory](/docs/id/memory#auto-memory).** Pembelajaran yang disimpan Claude secara otomatis saat Anda bekerja, seperti preferensi Anda. 200 baris pertama atau 25KB MEMORY.md, mana pun yang lebih dulu, dimuat di awal setiap sesi.
* **Ekstensi yang Anda konfigurasi.** [MCP servers](/docs/id/mcp) untuk layanan eksternal, [skills](/docs/id/skills) untuk workflow, [subagents](/docs/id/sub-agents) untuk pekerjaan yang didelegasikan, dan [Claude in Chrome](/docs/id/chrome) untuk interaksi browser.

Karena Claude melihat seluruh proyek Anda, Claude dapat bekerja di seluruhnya. Ketika Anda meminta Claude untuk "perbaiki bug autentikasi," Claude mencari file yang relevan, membaca beberapa file untuk memahami konteks, membuat edit terkoordinasi di seluruhnya, menjalankan test untuk memverifikasi perbaikan, dan melakukan commit perubahan jika Anda meminta. Ini berbeda dari asisten kode inline yang hanya melihat file saat ini.

<h2 id="environments-and-interfaces">
  Lingkungan dan interface
</h2>

[Loop agentic](#the-agentic-loop), [tools](#tools), dan kemampuan adalah sama di mana pun Anda menggunakan Claude Code. Apa yang berubah adalah di mana kode dieksekusi dan bagaimana Anda berinteraksi dengannya.

<h3 id="execution-environments">
  Lingkungan eksekusi
</h3>

Claude Code berjalan di tiga lingkungan, masing-masing dengan trade-off berbeda untuk di mana kode Anda dieksekusi.

| Lingkungan         | Di mana kode berjalan                                                                                                     | Use case                                                                    |
| ------------------ | ------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------- |
| **Local**          | Mesin Anda                                                                                                                | Default. Akses penuh ke file, tools, dan lingkungan Anda                    |
| **Cloud**          | VM yang dikelola Anthropic, atau [lingkungan self-hosted](/docs/id/self-hosted-environments) yang dioperasikan organisasi Anda | Mendelegasikan tugas, bekerja pada repo yang tidak Anda miliki secara lokal |
| **Remote Control** | Mesin Anda, dikontrol dari browser                                                                                        | Gunakan web UI sambil eksekusi dan file Anda tetap lokal                    |

<h3 id="interfaces">
  Interface
</h3>

Anda dapat mengakses Claude Code melalui terminal, [desktop app](/docs/id/desktop), [IDE extensions](/docs/id/vs-code), [claude.ai/code](https://claude.ai/code), [Remote Control](/docs/id/remote-control), [Slack](/docs/id/slack), dan [CI/CD pipelines](/docs/id/github-actions). Interface menentukan bagaimana Anda melihat dan berinteraksi dengan Claude, tetapi loop agentic yang mendasarinya identik. Lihat [Use Claude Code everywhere](/docs/id/overview#use-claude-code-everywhere) untuk daftar lengkap.

<h2 id="work-with-sessions">
  Bekerja dengan session
</h2>

Claude Code menyimpan percakapan Anda secara lokal saat Anda bekerja. Setiap pesan, penggunaan tool, dan hasil ditulis ke file plaintext JSONL di bawah `~/.claude/projects/`, yang memungkinkan [rewinding](#undo-changes-with-checkpoints), [resuming, dan forking](#resume-or-fork-sessions) session. Sebelum Claude membuat perubahan kode, Claude juga membuat snapshot file yang terpengaruh sehingga Anda dapat mengembalikan jika diperlukan. Untuk path, retention, dan cara menghapus data ini, lihat [application data in `~/.claude`](/docs/id/claude-directory#application-data).

**Session bersifat independen.** Setiap session baru dimulai dengan context window segar, tanpa riwayat percakapan dari session sebelumnya. Claude dapat mempertahankan pembelajaran di seluruh session menggunakan [auto memory](/docs/id/memory#auto-memory), dan Anda dapat menambahkan instruksi persisten Anda sendiri di [CLAUDE.md](/docs/id/memory).

<h3 id="work-across-branches">
  Bekerja di seluruh branch
</h3>

Setiap percakapan Claude Code adalah session yang terikat pada direktori saat ini Anda. Picker `/resume` menampilkan session dari worktree saat ini secara default, dengan pintasan keyboard untuk memperluas daftar ke worktree atau proyek lain. Lihat [Manage sessions](/docs/id/sessions#use-the-session-picker) untuk daftar lengkap pintasan picker dan bagaimana name resolution bekerja.

Claude melihat file branch saat ini Anda. Ketika Anda beralih branch, Claude melihat file branch baru, tetapi riwayat percakapan Anda tetap sama. Claude mengingat apa yang Anda diskusikan bahkan setelah beralih.

Karena session terikat pada direktori, Anda dapat menjalankan session Claude paralel dengan menggunakan [git worktrees](/docs/id/worktrees), yang membuat direktori terpisah untuk branch individual.

<h3 id="resume-or-fork-sessions">
  Resume atau fork session
</h3>

Melanjutkan session dengan `claude --continue` atau `claude --resume` membuka kembali session di bawah session ID yang sama dan menambahkan pesan baru ke percakapan yang ada. Forking dengan `--fork-session` atau `/branch` menyalin riwayat ke session ID baru, meninggalkan yang asli tidak berubah.

<img src="https://mintcdn.com/claude-code/ikqp3_70mqIahteV/images/session-continuity.svg?fit=max&auto=format&n=ikqp3_70mqIahteV&q=85&s=04ed0984a58e4127e05b3640265241a3" className="dark:hidden" alt="Diagram of session continuity: resume continues the same session, fork creates a new branch with a new ID." width="560" height="280" data-path="images/session-continuity.svg" />

<img src="https://mintcdn.com/claude-code/_xqph1dUOslCOwsj/images/session-continuity-dark.svg?fit=max&auto=format&n=_xqph1dUOslCOwsj&q=85&s=886a384bce8298594e43f124617ea665" className="hidden dark:block" alt="Diagram of session continuity: resume continues the same session, fork creates a new branch with a new ID." width="560" height="280" data-path="images/session-continuity-dark.svg" />

Untuk flag resume, picker `/resume`, naming, dan apa yang terjadi ketika session yang sama terbuka di dua terminal, lihat [Manage sessions](/docs/id/sessions).

<h3 id="the-context-window">
  The context window
</h3>

Context window Claude menampung riwayat percakapan Anda, konten file, output perintah, [CLAUDE.md](/docs/id/memory), [auto memory](/docs/id/memory#auto-memory), skill yang dimuat, dan instruksi sistem. Saat Anda bekerja, konteks terisi. Claude melakukan compacting secara otomatis, tetapi instruksi dari awal percakapan dapat hilang. Letakkan aturan persisten di CLAUDE.md, dan jalankan `/context` untuk melihat apa yang menggunakan ruang.

Untuk panduan interaktif tentang apa yang dimuat dan kapan, lihat [Explore the context window](/docs/id/context-window).

<h4 id="when-context-fills-up">
  When context fills up
</h4>

Claude Code mengelola konteks secara otomatis saat Anda mendekati batas. Claude menghapus output tool yang lebih lama terlebih dahulu, kemudian merangkum percakapan jika diperlukan. Permintaan Anda dan snippet kode kunci dipertahankan; instruksi terperinci dari awal percakapan mungkin hilang. Letakkan aturan persisten di CLAUDE.md daripada mengandalkan riwayat percakapan.

Untuk mengontrol apa yang dipertahankan selama compacting, tambahkan bagian "Compact Instructions" ke CLAUDE.md atau jalankan `/compact` dengan fokus (seperti `/compact focus on the API changes`).

Jika file tunggal atau output tool sangat besar sehingga konteks terisi kembali segera setelah setiap ringkasan, Claude Code berhenti auto-compacting setelah beberapa upaya dan menampilkan error sebagai gantinya dari looping. Lihat [Auto-compaction stops with a thrashing error](/docs/id/troubleshooting#auto-compaction-stops-with-a-thrashing-error) untuk langkah pemulihan.

Jalankan `/context` untuk melihat apa yang menggunakan ruang. Definisi tool MCP ditunda secara default dan dimuat sesuai permintaan melalui [tool search](/docs/id/mcp#scale-with-mcp-tool-search), jadi hanya nama tool dan instruksi server yang mengonsumsi konteks sampai Claude menggunakan tool spesifik.

<h4 id="manage-context-with-skills-and-subagents">
  Manage context with skills and subagents
</h4>

Selain compacting, Anda dapat menggunakan fitur lain untuk mengontrol apa yang dimuat ke dalam konteks.

[Skills](/docs/id/skills) dimuat sesuai permintaan. Claude melihat deskripsi skill pada awal session, tetapi konten lengkap hanya dimuat ketika skill digunakan. Untuk skill yang Anda panggil secara manual, atur `disable-model-invocation: true` untuk menjaga deskripsi keluar dari konteks sampai Anda membutuhkannya. Untuk skill yang tidak Anda tulis, gunakan [`skillOverrides`](/docs/id/skills#override-skill-visibility-from-settings) untuk melakukan hal yang sama dari settings.

[Subagents](/docs/id/sub-agents) bekerja dalam context window mereka sendiri. Subagent dimulai segar kecuali itu adalah [fork](/docs/id/sub-agents#fork-the-current-conversation), yang dimulai dengan salinan percakapan Anda sejauh ini. Bagaimanapun, panggilan tool subagent tetap keluar dari konteks Anda, dan Claude mendapatkan ringkasan kembali ketika subagent selesai.

Lihat [context costs](/docs/id/features-overview#understand-context-costs) untuk apa yang setiap fitur biayai, dan [reduce token usage](/docs/id/costs#reduce-token-usage) untuk tips mengelola konteks.

<h2 id="stay-safe-with-checkpoints-and-permissions">
  Tetap aman dengan checkpoint dan permission
</h2>

Claude memiliki dua mekanisme keamanan: checkpoint memungkinkan Anda membatalkan perubahan file, dan permission mengontrol apa yang dapat dilakukan Claude tanpa bertanya.

<h3 id="undo-changes-with-checkpoints">
  Batalkan perubahan dengan checkpoint
</h3>

**Setiap edit file dapat dikembalikan.** Sebelum Claude mengedit file, Claude membuat snapshot konten saat ini. Jika ada yang salah, tekan `Esc` dua kali untuk kembali ke state sebelumnya, atau minta Claude untuk membatalkan.

Checkpoint terpisah dari git dan tetap tersedia ketika Anda melanjutkan percakapan. Mereka hanya mencakup perubahan file, dan restore [melewati file yang di-symlink dan hard-link](/docs/id/checkpointing#symlinked-and-hard-linked-paths-not-restored). Tindakan yang mempengaruhi sistem remote (database, API, deployment) tidak dapat di-checkpoint. Anda mengontrol tindakan tersebut dengan mode permission dan aturan permission Anda.

<h3 id="control-what-claude-can-do">
  Kontrol apa yang dapat dilakukan Claude
</h3>

Pilih mode permission untuk mengatur apa yang dapat dilakukan Claude tanpa bertanya kepada Anda. Tekan `Shift+Tab` untuk melakukan siklus melalui mode permission:

* **Auto**: classifier meninjau sebagian besar tindakan di latar belakang dan memblokir tindakan yang berisiko alih-alih bertanya kepada Anda. Pada paket Pro, Max, dan Team, ini adalah [mode permission awal bawaan](/docs/id/permission-modes#which-mode-a-session-starts-in) untuk sesi terminal interaktif dan VS Code
* **Manual**: Claude bertanya sebelum edit file dan perintah shell
* **Accept edits**: Claude mengedit file dan menjalankan perintah filesystem umum seperti `mkdir` dan `mv` tanpa bertanya, masih bertanya untuk perintah lain
* **Plan**: Claude mengeksplorasi dan mengusulkan rencana tanpa mengedit file sumber Anda

Anda juga dapat mengizinkan perintah spesifik di `.claude/settings.json` sehingga Claude tidak bertanya setiap kali. Ini berguna untuk perintah terpercaya seperti `npm test` atau `git status`. Settings dapat dibatasi dari kebijakan organisasi-luas hingga preferensi pribadi. Lihat [Permissions](/docs/id/permissions) untuk detail.

***

<h2 id="work-effectively-with-claude-code">
  Bekerja secara efektif dengan Claude Code
</h2>

Tips ini membantu Anda mendapatkan hasil yang lebih baik dari Claude Code. Untuk informasi lebih lanjut tentang prompt spesifik, verifikasi, dan perencanaan, lihat [Best practices](/docs/id/best-practices).

<h3 id="ask-claude-code-for-help">
  Minta bantuan Claude Code
</h3>

Claude Code dapat mengajarkan Anda cara menggunakannya. Ajukan pertanyaan seperti "bagaimana cara mengatur hooks?" atau "apa cara terbaik untuk menyusun CLAUDE.md saya?" dan Claude akan menjelaskan.

Perintah bawaan juga memandu Anda melalui setup:

* `/init` menghasilkan CLAUDE.md pemula untuk proyek Anda
* `/doctor` menjalankan pemeriksaan setup yang mendiagnosis masalah instalasi dan konfigurasi serta dapat memperbaikinya

<h3 id="it’s-a-conversation">
  Ini adalah percakapan
</h3>

Claude Code bersifat conversational. Anda tidak memerlukan prompt yang sempurna. Mulai dengan apa yang Anda inginkan, kemudian perbaiki:

```text theme={null}
Perbaiki bug login
```

\[Claude menyelidiki, mencoba sesuatu]

```text theme={null}
Itu tidak cukup benar. Masalahnya ada di session handling.
```

\[Claude menyesuaikan pendekatan]

Ketika upaya pertama tidak benar, Anda tidak memulai dari awal. Anda melakukan iterasi.

<h4 id="interrupt-and-steer">
  Ganggu dan arahkan
</h4>

Anda dapat mengalihkan Claude kapan saja tanpa memulai dari awal. Lakukan salah satu dari ini:

* **Tekan `Esc`** untuk menghentikan Claude segera. Panggilan tool yang sedang berjalan dibatalkan dan Claude menunggu instruksi Anda berikutnya. Jika Anda memiliki pesan yang antri, Claude Code [mengirimnya selanjutnya](/docs/id/interactive-mode#queue-messages-while-claude-works).
* **Ketik koreksi dan tekan `Enter`** tanpa menghentikan Claude. Pesan ditampilkan sebagai antri di atas kotak input. Jika Claude menjalankan panggilan tool, pesan dibaca segera setelah panggilan tersebut selesai, dalam giliran yang sama, dan disesuaikan sebelum langkah berikutnya. [Antri pesan saat Claude bekerja](/docs/id/interactive-mode#queue-messages-while-claude-works) mencakup kapan entri antri lainnya dikirim.

<h3 id="delegate-don’t-dictate">
  Delegasikan, jangan mendikte
</h3>

Pikirkan mendelegasikan kepada rekan kerja yang mampu. Berikan konteks dan arah, kemudian percayai Claude untuk mengetahui detail:

```text theme={null}
Alur checkout rusak untuk pengguna dengan kartu yang kadaluarsa.
Kode yang relevan ada di src/payments/. Bisakah Anda menyelidiki dan memperbaikinya?
```

Anda tidak perlu menentukan file mana yang harus dibaca atau perintah mana yang harus dijalankan. Claude mengetahui itu.

<h2 id="what’s-next">
  Apa selanjutnya
</h2>

<CardGroup cols={2}>
  <Card title="Perluas dengan fitur" icon="puzzle-piece" href="/docs/id/features-overview">
    Tambahkan skills dan koneksi MCP
  </Card>

  <Card title="Alur kerja umum" icon="graduation-cap" href="/docs/id/common-workflows">
    Panduan langkah demi langkah untuk tugas khas
  </Card>
</CardGroup>
