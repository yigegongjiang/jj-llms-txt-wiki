> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Glosarium

> Definisi untuk terminologi Claude Code. Pelajari apa itu agentic loop, compaction, CLAUDE.md, hooks, subagents, MCP, dan konsep inti lainnya.

Glosarium ini mendefinisikan terminologi Claude Code. Setiap entri menghubungkan ke halaman tempat konsep dibahas secara mendalam. Untuk konsep tingkat model seperti tokens, temperature, dan RAG, lihat [glosarium platform](https://platform.claude.com/docs/id/about-claude/glossary).

<h2 id="a">
  A
</h2>

<h3 id="agent-teams">
  Agent teams
</h3>

Beberapa sesi Claude Code independen yang dikoordinasikan oleh pemimpin tim, dengan daftar tugas bersama dan pesan peer-to-peer. Tidak seperti [subagents](#subagent), yang berjalan dalam satu sesi dan hanya melaporkan ke induk, rekan kerja masing-masing memiliki jendela konteks mereka sendiri dan Anda dapat berinteraksi dengan salah satu dari mereka secara langsung. Agent teams bersifat eksperimental dan harus diaktifkan dengan menetapkan `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS=1`.

Pelajari lebih lanjut: [Run agent teams](/docs/id/agent-teams)

<h3 id="agentic-coding">
  Agentic coding
</h3>

Alur kerja di mana AI dapat membaca file, menjalankan perintah, dan membuat perubahan secara otonom saat Anda menonton, mengalihkan, atau pergi, berbeda dengan asisten berbasis chat yang hanya merespons dengan teks yang harus Anda terapkan sendiri. Claude Code bersifat agentic karena memiliki [tools](#tool) yang memungkinkannya bertindak, bukan hanya memberi saran.

Pelajari lebih lanjut: [How Claude Code works](/docs/id/how-claude-code-works)

<h3 id="agentic-harness">
  Agentic harness
</h3>

Tools, manajemen konteks, dan lingkungan eksekusi yang mengubah model bahasa menjadi agen coding yang mampu. Claude Code adalah harness; Claude adalah model di dalamnya. Harness menyediakan akses file, eksekusi shell, gating izin, pemuatan memori, dan loop yang menghubungkan tindakan bersama-sama.

Pelajari lebih lanjut: [How Claude Code works](/docs/id/how-claude-code-works)

<h3 id="agentic-loop">
  Agentic loop
</h3>

Siklus yang Claude lalui untuk setiap tugas: kumpulkan konteks, ambil tindakan, verifikasi hasil, dan ulangi sampai selesai. Setiap penggunaan tool mengembalikan informasi yang menginformasikan langkah berikutnya. Anda dapat mengganggu loop kapan saja untuk mengalihkan. Sebagian besar titik ekstensi, termasuk [hooks](#hook), [skills](#skill), dan [MCP](#mcp-model-context-protocol), terhubung ke fase spesifik dari loop ini.

Pelajari lebih lanjut: [How Claude Code works](/docs/id/how-claude-code-works#the-agentic-loop)

<h3 id="artifact">
  Artifact
</h3>

Halaman web interaktif langsung yang Claude Code terbitkan dari sesi Anda ke URL pribadi di claude.ai, sehingga Anda dapat melihat output secara visual atau membagikannya daripada membaca teks terminal. Halaman diperbarui di tempat ketika sesi menerbitkan ulang. Artifact yang Anda buat dari Claude Code muncul di galeri yang sama dengan artifact yang dibuat dalam percakapan claude.ai. Berbagi tergantung pada paket Anda: di Pro dan Max, tautan publik yang dapat dibuka siapa saja; di Team dan Enterprise, berbagi dalam organisasi Anda, ditambah tautan publik setelah Pemilik mengaktifkannya.

Pelajari lebih lanjut: [Share session output as artifacts](/docs/id/artifacts)

<h3 id="auto-memory">
  Auto memory
</h3>

Catatan yang Claude tulis untuk dirinya sendiri berdasarkan koreksi dan preferensi Anda, disimpan per repositori git di bawah `~/.claude/projects/`. Semua worktrees dari repositori yang sama berbagi satu direktori auto memory. 200 baris pertama atau 25 KB dari indeks `MEMORY.md` dimuat di awal setiap sesi. Auto memory adalah rekan Claude-written untuk [CLAUDE.md](#claude-md), yang Anda tulis.

Pelajari lebih lanjut: [Auto memory](/docs/id/memory#auto-memory)

<h3 id="auto-mode">
  Auto mode
</h3>

Sebuah [permission mode](#permission-mode) di mana model classifier terpisah meninjau tindakan di latar belakang, sehingga sebagian besar berjalan tanpa prompt persetujuan; aturan ask eksplisit masih meminta. Classifier memblokir eskalasi scope, infrastruktur yang tidak dipercaya, dan [prompt injection](#prompt-injection). Classifier tidak pernah melihat hasil tool, jadi instruksi yang disuntikkan tidak dapat mempengaruhi keputusannya.

Pelajari lebih lanjut: [Eliminate prompts with auto mode](/docs/id/permission-modes#eliminate-prompts-with-auto-mode)

<h2 id="b">
  B
</h2>

<h3 id="bare-mode">
  Bare mode
</h3>

Bendera startup, `--bare`, yang melewati auto-discovery hooks, skills, plugins, MCP servers, auto memory, dan CLAUDE.md. Hanya bendera yang Anda lewatkan secara eksplisit yang berlaku. Direkomendasikan untuk CI dan panggilan script di mana Anda memerlukan perilaku identik di seluruh mesin terlepas dari konfigurasi lokal.

Pelajari lebih lanjut: [Mulai lebih cepat dengan bare mode](/docs/id/headless#start-faster-with-bare-mode)

<h3 id="bundled-skills">
  Bundled skills
</h3>

Playbook berbasis prompt yang disertakan dengan Claude Code, seperti `/batch`, `/code-review`, `/debug`, dan `/loop`. Tidak seperti perintah built-in, yang mengeksekusi logika tetap, bundled skills memberikan Claude prompt terperinci dan membiarkannya mengorkestrasi pekerjaan, sehingga mereka dapat menelurkan agen, membaca file, dan beradaptasi dengan codebase Anda.

Pelajari lebih lanjut: [Bundled skills](/docs/id/skills#bundled-skills)

<h2 id="c">
  C
</h2>

<h3 id="channel">
  Channel
</h3>

Sebuah [MCP server](#mcp-model-context-protocol) yang mendorong peristiwa ke sesi yang sedang berjalan sehingga Claude dapat bereaksi terhadap hal-hal yang terjadi saat Anda jauh dari terminal. Channel dapat dua arah: Claude membaca peristiwa masuk dan membalas kembali melalui channel yang sama. Telegram, Discord, dan iMessage disertakan dalam pratinjau penelitian.

Pelajari lebih lanjut: [Channels](/docs/id/channels)

<h3 id="checkpoint">
  Checkpoint
</h3>

Titik pemulihan yang dibuat di setiap prompt yang Anda kirim. Claude Code mengambil snapshot file sebelum setiap edit sehingga checkpoint dapat mengembalikannya. Tekan `Esc` dua kali atau jalankan `/rewind` untuk mengembalikan kode, percakapan, atau keduanya ke titik sebelumnya, atau untuk merangkum bagian percakapan dari pesan yang dipilih. Checkpoint disimpan dengan percakapan, sehingga sesi yang dilanjutkan masih dapat `/rewind` ke dalamnya. Mereka terpisah dari git dan tidak melacak perubahan yang dibuat melalui tool Bash.

Pelajari lebih lanjut: [Checkpointing](/docs/id/checkpointing)

<h3 id="claude-directory">
  `.claude` directory
</h3>

Direktori tempat Claude Code membaca konfigurasi yang dibatasi proyek: settings, hooks, skills, subagents, rules, dan auto memory. Sebuah proyek memiliki `.claude/` di akarnya; default tingkat pengguna Anda berada di `~/.claude/`.

Pelajari lebih lanjut: [The `.claude` directory](/docs/id/claude-directory)

<h3 id="claude-md">
  CLAUDE.md
</h3>

File markdown dari instruksi persisten yang Anda tulis untuk Claude, dimuat di awal setiap sesi sebagai pesan pengguna setelah system prompt. Letakkan konvensi proyek, catatan arsitektur, dan aturan "selalu lakukan X" di sini. Project-root CLAUDE.md bertahan [compaction](#compaction) dan dibaca ulang segar dari disk sesudahnya.

Anda dapat menempatkan CLAUDE.md di scope proyek di `./CLAUDE.md` atau `./.claude/CLAUDE.md`, di scope pengguna di `~/.claude/CLAUDE.md`, atau sebagai [managed policy](#managed-settings) untuk organisasi Anda. Semua file yang ditemukan digabungkan ke dalam konteks daripada menimpa satu sama lain, diurutkan dari scope terluas ke paling spesifik.

Pelajari lebih lanjut: [CLAUDE.md files](/docs/id/memory#claude-md-files)

<h3 id="command">
  Command
</h3>

Instruksi yang dapat digunakan kembali yang Anda panggil dengan mengetik `/name` dalam prompt. Perintah built-in seperti `/clear`, `/model`, dan `/compact` mengontrol sesi. Anda dapat menentukan perintah Anda sendiri sebagai file di `.claude/commands/`, atau menginstalnya dari [plugin](#plugin). [Skills](#skill) adalah cara yang direkomendasikan untuk mengemas perintah multi-langkah.

Pelajari lebih lanjut: [Commands](/docs/id/commands) · [Skills](/docs/id/skills)

<h3 id="compaction">
  Compaction
</h3>

Ringkasan otomatis percakapan Anda ketika [context window](#context-window) mendekati batasnya. Output tool yang lebih lama dihapus terlebih dahulu, kemudian percakapan diringkas. Project-root CLAUDE.md dan auto memory bertahan compaction dan dimuat ulang dari disk; instruksi yang diberikan hanya dalam percakapan mungkin hilang. Jalankan `/compact` untuk memicu secara manual, secara opsional dengan fokus seperti `/compact focus on the API changes`.

Pelajari lebih lanjut: [What survives compaction](/docs/id/context-window#what-survives-compaction) · [When context fills up](/docs/id/how-claude-code-works#when-context-fills-up)

<h3 id="context-window">
  Context window
</h3>

Memori kerja untuk sesi, menampung riwayat percakapan, konten file, output perintah, CLAUDE.md, auto memory, skills yang dimuat, dan instruksi sistem. Saat Anda bekerja, konteks terisi sampai [compaction](#compaction) meringkasnya. Jalankan `/context` untuk melihat apa yang menggunakan ruang. Untuk konsep model yang mendasar, lihat [glosarium platform](https://platform.claude.com/docs/en/about-claude/glossary#context-window).

Pelajari lebih lanjut: [Explore the context window](/docs/id/context-window)

<h2 id="d">
  D
</h2>

<h3 id="dispatch">
  Dispatch
</h3>

Router tugas yang diinisiasi telepon yang menelurkan sesi Claude Code di aplikasi Desktop ketika Anda mengirim tugas coding dari aplikasi mobile Claude. Prompt Anda merutekan ke tool yang tepat secara otomatis. Tersedia di paket Pro dan Max.

Pelajari lebih lanjut: [Sessions from Dispatch](/docs/id/desktop#sessions-from-dispatch)

<h2 id="e">
  E
</h2>

<h3 id="effort-level">
  Effort level
</h3>

Pengaturan yang mengontrol berapa banyak anggaran thinking adaptive-reasoning yang Claude gunakan pada setiap giliran. Effort yang lebih tinggi berarti lebih banyak thinking tokens dan reasoning yang lebih dalam; effort yang lebih rendah lebih cepat dan lebih murah. Effort didukung di Fable 5, di Opus 4.6 dan yang lebih baru, serta di Sonnet 4.6 dan yang lebih baru.

Pelajari lebih lanjut: [Adjust effort level](/docs/id/model-config#adjust-effort-level)

<h3 id="extended-thinking">
  Extended thinking
</h3>

Reasoning step-by-step yang terlihat yang dilakukan model sebelum merespons. Anda dapat menyesuaikannya dengan [effort level](#effort-level), atau membatasi thinking tokens dengan `MAX_THINKING_TOKENS` pada model dengan anggaran thinking tetap. Thinking muncul dalam teks italic abu-abu di terminal.

Pelajari lebih lanjut: [Use extended thinking](/docs/id/model-config#extended-thinking)

<h2 id="h">
  H
</h2>

<h3 id="hook">
  Hook
</h3>

Handler yang ditentukan pengguna yang dieksekusi secara otomatis pada titik spesifik dalam lifecycle Claude Code, seperti sebelum tool berjalan, setelah edit file, atau di awal sesi. Handler dapat berupa perintah shell, endpoint HTTP, MCP tool, prompt LLM, atau subagent. Hook bersifat deterministik: mereka menyala pada titik lifecycle tetap daripada atas kebijakan model.

Konfigurasi hook memiliki tiga level:

* **Hook event**: titik lifecycle
* **Matcher**: filter yang peristiwa menyalakannya
* **Hook handler**: apa yang berjalan

Pelajari lebih lanjut: [Get started with hooks](/docs/id/hooks-guide) · [Hooks reference](/docs/id/hooks)

<h2 id="m">
  M
</h2>

<h3 id="managed-settings">
  Managed settings
</h3>

Pengaturan yang diberlakukan di seluruh organisasi oleh IT atau DevOps, dikirimkan dari server Anthropic melalui konsol admin atau diterapkan ke perangkat di jalur tingkat OS di luar `~/.claude`. Pengguna dan pengaturan proyek tidak dapat mengesampingkan managed settings. Pengiriman yang dikelola server berlaku pada [konfigurasi yang memenuhi syarat](/docs/id/server-managed-settings#platform-availability); lihat [Pertimbangan keamanan](/docs/id/server-managed-settings#security-considerations). Gunakan ini untuk kebijakan keamanan, persyaratan kepatuhan, atau tooling standar di seluruh armada.

Pelajari lebih lanjut: [Server-managed settings](/docs/id/server-managed-settings) · [Settings files](/docs/id/settings#settings-files)

<h3 id="mcp-model-context-protocol">
  MCP (Model Context Protocol)
</h3>

Standar terbuka untuk menghubungkan tools AI ke sumber data eksternal dan layanan. MCP servers memberikan Claude tools baru untuk Slack, Jira, database, browser, dan ratusan integrasi lainnya. Anda menghubungkan servers melalui `/mcp` atau dengan menambahkannya ke `.mcp.json`. Untuk protokol itu sendiri, lihat [glosarium platform](https://platform.claude.com/docs/en/about-claude/glossary#mcp-model-context-protocol).

Pelajari lebih lanjut: [Model Context Protocol](/docs/id/mcp)

<h3 id="mcp-tool-search">
  MCP Tool Search
</h3>

Mekanisme penghematan konteks yang menunda skema MCP tool sampai diperlukan. Hanya nama tool yang dimuat saat startup; Claude mengambil skema lengkap sesuai permintaan ketika memutuskan untuk menggunakan tool spesifik. Ini menjaga MCP servers idle dari mengonsumsi banyak konteks.

Pelajari lebih lanjut: [Scale with MCP Tool Search](/docs/id/mcp#scale-with-mcp-tool-search)

<h2 id="n">
  N
</h2>

<h3 id="non-interactive-mode">
  Non-interactive mode
</h3>

Mode yang mengeksekusi prompt tunggal dan keluar tanpa sesi percakapan interaktif, dipanggil dengan `-p` atau `--print`. Digunakan untuk CI, script, dan piping. Jalannya masih disimpan sebagai sesi yang dapat dilanjutkan kecuali Anda melewatkan `--no-session-persistence`. [Agent SDK](/docs/id/agent-sdk/overview) adalah setara Python dan TypeScript. Sebelumnya disebut headless mode.

Pelajari lebih lanjut: [Run Claude Code programmatically](/docs/id/headless)

<h2 id="o">
  O
</h2>

<h3 id="output-style">
  Output style
</h3>

Konfigurasi yang memodifikasi system prompt Claude untuk mengubah perilaku respons, nada, atau format. Output styles mematikan bagian khusus software-engineering dari system prompt default, tidak seperti [CLAUDE.md](#claude-md) yang dikirimkan sebagai pesan pengguna mengikuti system prompt. Style built-in termasuk Default, Proactive, Explanatory, dan Learning.

Pelajari lebih lanjut: [Output styles](/docs/id/output-styles)

<h2 id="p">
  P
</h2>

<h3 id="permission-mode">
  Permission mode
</h3>

Perilaku persetujuan baseline untuk sesi. Siklus dengan `Shift+Tab` di CLI atau gunakan pemilih mode di VS Code, Desktop, dan claude.ai. Mode yang tersedia adalah `default`, `acceptEdits`, `plan`, `auto`, `dontAsk`, dan `bypassPermissions`.

Mode `default` diberi label Manual di CLI, di ekstensi VS Code dan JetBrains, dan di aplikasi desktop, dan Claude Code menerima `manual` sebagai alias untuk nilai tersebut.

Pelajari lebih lanjut: [Pilih permission mode](/docs/id/permission-modes)

<h3 id="permission-rule">
  Permission rule
</h3>

Entri settings yang memungkinkan, menanyakan tentang, atau menolak invokasi tool berdasarkan nama tool dan pola argumen. Aturan dievaluasi deny→ask→allow, kecocokan pertama menang. Permission rules adalah kontrol granular yang berlapis di atas [permission mode](#permission-mode) yang lebih luas.

Pelajari lebih lanjut: [Konfigurasi permissions](/docs/id/permissions)

<h3 id="plan-mode">
  Plan mode
</h3>

Sebuah [permission mode](#permission-mode) di mana Claude meneliti dan mengusulkan perubahan tanpa mengedit file sumber Anda. Dapat membaca, mencari, dan menjalankan perintah eksplorasi, kemudian menyajikan rencana untuk persetujuan sebelum menyentuh apa pun. Masukkan plan mode dengan `/plan` atau dengan menekan `Shift+Tab`.

Pelajari lebih lanjut: [Analisis sebelum Anda mengedit dengan plan mode](/docs/id/permission-modes#analyze-before-you-edit-with-plan-mode)

<h3 id="plugin">
  Plugin
</h3>

Bundle skills, hooks, subagents, dan MCP servers yang dikemas sebagai unit yang dapat diinstal tunggal. Plugin skills diberi namespace sebagai `plugin-name:skill-name` sehingga beberapa plugin dapat hidup berdampingan. Distribusikan plugins di seluruh tim melalui [marketplace](/docs/id/plugin-marketplaces).

Pelajari lebih lanjut: [Plugins](/docs/id/plugins)

<h3 id="project-trust">
  Project trust
</h3>

Dialog yang menerima direktori sebelum Claude Code memuat konfigurasinya. Penerimaan disimpan per direktori proyek, kecuali direktori home Anda, di mana kepercayaan dipegang untuk sesi saat ini saja dan prompt muncul kembali pada setiap peluncuran. Trust gates auto-installation marketplace plugins dan eksekusi project-defined hooks. Mempercayai direktori berarti `.claude/settings.json`, `.mcp.json`, dan file config lainnya berlaku.

Pelajari lebih lanjut: [Direktori `.claude`](/docs/id/claude-directory)

<h3 id="prompt-injection">
  Prompt injection
</h3>

Instruksi bermusuhan yang tertanam dalam file, halaman web, atau hasil tool yang mencoba mengalihkan Claude ke arah tindakan yang tidak pernah Anda minta. Pertahanan Claude Code termasuk sistem izin, deteksi injeksi perintah, dan verifikasi kepercayaan. [Auto mode](#auto-mode) menambahkan probe sisi server yang memindai hasil tool untuk konten mencurigakan dan classifier yang tidak pernah melihat hasil tool, jadi teks yang disuntikkan tidak dapat mempengaruhi keputusan persetujuannya.

Pelajari lebih lanjut: [Lindungi dari prompt injection](/docs/id/security#protect-against-prompt-injection)

<h2 id="r">
  R
</h2>

<h3 id="remote-control">
  Remote Control
</h3>

Cara untuk melanjutkan sesi Claude Code lokal dari telepon atau browser Anda melalui claude.ai. Eksekusi kode dan file Anda tetap di mesin Anda; antarmuka bersifat remote. Berbeda dari Claude Code di web, yang berjalan dalam sandbox cloud.

Pelajari lebih lanjut: [Remote Control](/docs/id/remote-control)

<h3 id="rules">
  Rules
</h3>

File instruksi modular di `.claude/rules/` yang dimuat bersama CLAUDE.md. Aturan dapat dibatasi path dengan frontmatter YAML `paths:` sehingga hanya dimuat ketika Claude membaca file yang cocok, menjaga konteks tetap ramping sampai relevan.

Pelajari lebih lanjut: [Organize rules with `.claude/rules/`](/docs/id/memory#organize-rules-with-claude/rules/)

<h2 id="s">
  S
</h2>

<h3 id="sandboxing">
  Sandboxing
</h3>

Isolasi filesystem dan jaringan tingkat OS untuk tool Bash. Perintah berjalan di dalam batas yang Anda tentukan di muka, sehingga Claude dapat bekerja dengan bebas di dalamnya tanpa prompt persetujuan per-perintah. Sandboxing adalah lapisan terpisah dari [permission rules](#permission-rule).

Pelajari lebih lanjut: [Sandboxing](/docs/id/sandboxing)

<h3 id="session">
  Session
</h3>

Percakapan yang terikat pada direktori saat ini, dengan [context window](#context-window) independen sendiri. Sesi dapat dilanjutkan dengan `claude -c`, difork dengan `--fork-session` untuk mempertahankan riwayat di bawah ID sesi baru, atau dijalankan secara paralel di seluruh terminal. Menjalankan `/clear` memulai sesi baru; sesi sebelumnya tetap disimpan dan tersedia melalui `/resume`. Transkrip setiap sesi disimpan di bawah `~/.claude/projects/`.

Pelajari lebih lanjut: [Work with sessions](/docs/id/how-claude-code-works#work-with-sessions)

<h3 id="settings-layers">
  Settings layers
</h3>

Hierarki yang Claude Code baca konfigurasi dari, dalam urutan prioritas dari tertinggi ke terendah: [managed policy](#managed-settings), argumen command-line, local settings di `.claude/settings.local.json`, project settings di `.claude/settings.json`, kemudian user settings di `~/.claude/settings.json`. Array merge di seluruh layer; scalar di layer yang lebih tinggi mengesampingkan yang lebih rendah.

Pelajari lebih lanjut: [Settings files](/docs/id/settings#settings-files)

<h3 id="skill">
  Skill
</h3>

File `SKILL.md` yang berisi instruksi, pengetahuan, atau alur kerja yang Claude tambahkan ke toolkit-nya. Claude memuat skill secara otomatis ketika relevan, atau Anda memanggilnya secara langsung dengan `/skill-name`. Skills mengikuti standar Agent Skills terbuka; Claude Code memperluas dengan kontrol invokasi dan eksekusi subagent.

Skills adalah penerus yang direkomendasikan untuk perintah kustom. File di `.claude/commands/deploy.md` dan satu di `.claude/skills/deploy/SKILL.md` keduanya membuat `/deploy` dan bekerja dengan cara yang sama; file perintah yang ada terus bekerja.

Pelajari lebih lanjut: [Extend Claude with skills](/docs/id/skills)

<h3 id="subagent">
  Subagent
</h3>

Asisten AI khusus yang berjalan di jendela konteks sendiri dengan system prompt kustom, akses tool spesifik, dan izin independen. Bekerja pada tugas yang didelegasikan dan mengembalikan ringkasan ke percakapan utama. Gunakan subagents untuk menjaga eksplorasi besar keluar dari konteks utama Anda atau untuk menjalankan penelitian paralel. Berbeda dari [agent teams](#agent-teams), di mana setiap agen adalah sesi independen penuh yang dapat Anda bicarakan secara langsung.

Subagents built-in termasuk Explore, Plan, dan general-purpose.

Pelajari lebih lanjut: [Create custom subagents](/docs/id/sub-agents)

<h3 id="surface">
  Surface
</h3>

Tempat apa pun Anda mengakses Claude Code: CLI, VS Code, JetBrains, Desktop, atau claude.ai. Semua surface berbagi engine yang sama, jadi CLAUDE.md, settings, dan skills Anda bekerja dengan cara yang sama di seluruhnya. Slack dan Chrome extension adalah integrasi yang terhubung ke surface daripada surface itu sendiri.

Pelajari lebih lanjut: [Platforms and integrations](/docs/id/platforms)

<h2 id="t">
  T
</h2>

<h3 id="teleport">
  Teleport
</h3>

Perintah, `/teleport`, yang menarik sesi Claude Code cloud ke terminal lokal Anda. Claude mengambil branch, memuat riwayat percakapan, dan melanjutkan dari keadaan terakhir sesi web. Arah sebaliknya adalah `--cloud`, yang mengirim tugas lokal untuk dijalankan di web.

Pelajari lebih lanjut: [Dari web ke terminal](/docs/id/claude-code-on-the-web#from-web-to-terminal)

<h3 id="tool">
  Tool
</h3>

Tindakan yang dapat Claude ambil: baca file, edit kode, jalankan perintah shell, cari web, telurkan subagent. Tools adalah apa yang membuat Claude Code agentic. Tanpa mereka, Claude hanya dapat merespons dengan teks. Setiap penggunaan tool mengembalikan hasil yang menginformasikan keputusan Claude berikutnya dalam [agentic loop](#agentic-loop).

Pelajari lebih lanjut: [Tools available to Claude](/docs/id/tools-reference)

<h3 id="turn">
  Turn
</h3>

Satu respons lengkap dari Claude dalam [session](#session). Sebuah turn dimulai ketika Anda mengirim pesan dan berakhir ketika Claude selesai merespons, dengan sejumlah panggilan [tool](#tool) di antaranya. [Stop hooks](#hook) diaktifkan di akhir setiap turn. Sebuah session terdiri dari banyak turn, dan [agentic loop](#agentic-loop) menjelaskan apa yang terjadi di dalam satu turn.

Pelajari lebih lanjut: [How Claude Code works](/docs/id/how-claude-code-works#the-agentic-loop)

<h2 id="v">
  V
</h2>

<h3 id="verification-loop">
  Verification loop
</h3>

Bagaimana sesi mengetahui pekerjaan benar-benar selesai daripada hanya masuk akal. Anda memberi Claude pemeriksaan yang dapat dijalankan, seperti suite test, build, atau perbandingan screenshot, dan Claude melakukan iterasi sampai pemeriksaan lulus alih-alih berhenti setelah satu percobaan. Loop verifikasi adalah prasyarat untuk [`/goal`](/docs/id/goal), unattended runs, dan [dynamic workflows](/docs/id/workflows): tanpa satu, satu-satunya hal yang memutuskan agen selesai adalah agen itu sendiri.

Pelajari lebih lanjut: [Berikan Claude cara untuk memverifikasi pekerjaannya](/docs/id/best-practices#give-claude-a-way-to-verify-its-work)

<h2 id="w">
  W
</h2>

<h3 id="worktree-isolation">
  Worktree isolation
</h3>

Mode isolasi yang menjalankan Claude di git worktree terpisah di bawah `.claude/worktrees/`, diaktifkan dengan bendera `-w` atau `isolation: worktree` dalam config subagent. Perubahan tetap di branch terpisah di direktori terpisah, sehingga agen paralel tidak menimpa file satu sama lain.

Pelajari lebih lanjut: [Run parallel sessions with git worktrees](/docs/id/worktrees)

***

<h2 id="deprecated-and-renamed-terms">
  Istilah yang sudah usang dan diganti nama
</h2>

Istilah-istilah ini muncul dalam docs yang lebih lama, posting blog, dan konten komunitas. Gunakan nama saat ini saat mencari di situs ini.

| Old term        | Now called                                    | Notes                                |
| --------------- | --------------------------------------------- | ------------------------------------ |
| Headless mode   | [Non-interactive mode](#non-interactive-mode) | Same `-p` flag, same behavior        |
| Custom commands | [Skills](#skill)                              | `.claude/commands/` files still work |
| Slash commands  | Commands                                      | "Slash" dropped from product copy    |
