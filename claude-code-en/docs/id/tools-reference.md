> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Referensi Tools

> Referensi lengkap untuk tools yang dapat digunakan Claude Code, termasuk persyaratan izin dan perilaku per-tool.

Claude Code memiliki akses ke serangkaian tools bawaan yang membantu memahami dan memodifikasi basis kode Anda. Nama tools adalah string yang tepat yang Anda gunakan dalam [aturan izin](/docs/id/permissions#tool-specific-permission-rules), [daftar tools subagent](/docs/id/sub-agents), dan [pencocokan hooks](/docs/id/hooks).

Untuk mengontrol tools mana yang dapat digunakan Claude dan kapan diminta terlebih dahulu, konfigurasikan [aturan izin](/docs/id/permissions#tool-specific-permission-rules) dalam pengaturan Anda, [hooks](/docs/id/hooks), atau [daftar tools subagent](/docs/id/sub-agents#supported-frontmatter-fields). Lihat [Konfigurasi tools dengan aturan izin dan hooks](#configure-tools-with-permission-rules-and-hooks) untuk setiap tempat yang menerima nama tool.

Untuk menambahkan tools kustom, hubungkan [server MCP](/docs/id/mcp). Untuk memperluas Claude dengan alur kerja berbasis prompt yang dapat digunakan kembali, tulis [skill](/docs/id/skills), yang berjalan melalui tool `Skill` yang ada daripada menambahkan entri tool baru.

<Info>
  Pada paket Pro, Max, dan Team, Claude Code memulai sesi dalam [mode otomatis](/docs/id/permission-modes#eliminate-prompts-with-auto-mode), di mana pengklasifikasi memutuskan sebagian besar prompt ini alih-alih Anda. Kolom `Permission required` menunjukkan apakah prompt tool dalam [Mode Manual](/docs/id/permission-modes) untuk path di dalam direktori kerja. Tools akses file yang ditandai Tidak, termasuk `Read`, `Grep`, dan `Glob`, masih meminta path di luar [direktori kerja dan direktori tambahan](/docs/id/permissions#working-directories). `Bash` ditandai Ya tetapi menjalankan set bawaan [perintah read-only](/docs/id/permissions#read-only-commands) tanpa meminta.
</Info>

| Tool                   | Deskripsi                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         | Izin diperlukan |
| :--------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | :-------------- |
| `Agent`                | Menjalankan [subagent](/docs/id/sub-agents) dengan jendela konteks sendiri untuk menangani tugas. Dengan [tim agent](/docs/id/agent-teams) diaktifkan, panggilan yang membawa `name` dapat meluncurkan [rekan tim](/docs/id/agent-teams#how-claude-starts-agent-teams) sebagai gantinya. Lihat [Perilaku tool Agent](#agent-tool-behavior)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       | Tidak           |
| `Artifact`             | Menerbitkan file HTML atau Markdown sebagai [artifact](/docs/id/artifacts): halaman pribadi dan interaktif di claude.ai. Anda dapat membagikannya dengan tautan publik, atau di dalam organisasi Anda pada paket Team dan Enterprise, di mana berbagi publik memerlukan Owner untuk [mengaktifkannya](/docs/id/artifacts#control-public-sharing). Memerlukan paket Pro, Max, Team, atau Enterprise dan autentikasi `/login`; lihat [Ketersediaan](/docs/id/artifacts#availability)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               | Ya              |
| `AskUserQuestion`      | Mengajukan pertanyaan pilihan ganda untuk mengumpulkan persyaratan atau mengklarifikasi ambiguitas. Pertanyaan tetap terbuka sampai Anda menjawabnya secara default. Lihat [Perilaku tool AskUserQuestion](#askuserquestion-tool-behavior)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        | Tidak           |
| `Bash`                 | Menjalankan perintah shell di lingkungan Anda. Lihat [Perilaku tool Bash](#bash-tool-behavior)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    | Ya              |
| `CronCreate`           | Menjadwalkan prompt berulang atau satu kali dalam sesi saat ini. Tugas bersifat sesi-scoped dan dipulihkan pada `--resume` atau `--continue` jika belum kadaluarsa. Lihat [tugas terjadwal](/docs/id/scheduled-tasks)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  | Tidak           |
| `CronDelete`           | Membatalkan tugas terjadwal berdasarkan ID                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        | Tidak           |
| `CronList`             | Mencantumkan semua tugas terjadwal dalam sesi                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     | Tidak           |
| `Edit`                 | Membuat pengeditan tertarget ke file tertentu. Lihat [Perilaku tool Edit](#edit-tool-behavior)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    | Ya              |
| `EndConversation`      | Mengakhiri sesi, dalam kasus langka input penyalahgunaan berkelanjutan atau ketika Anda meminta Claude untuk mendemonstrasikan tool. Memerlukan Claude Code v2.1.213 atau lebih baru. Lihat [Perilaku tool EndConversation](#endconversation-tool-behavior)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       | Tidak           |
| `EnterPlanMode`        | Beralih ke mode rencana untuk merancang pendekatan sebelum coding                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 | Tidak           |
| `EnterWorktree`        | Membuat [git worktree](/docs/id/worktrees) terisolasi dan beralih ke dalamnya. Berikan `path` untuk beralih ke worktree yang ada alih-alih membuat yang baru. Pada entri pertama target mungkin worktree dari repositori saat ini atau, dalam ruang kerja multi-repo, dari repositori yang bersarang di dalamnya. Sebelum v2.1.203, worktree repositori bersarang ditolak. A `path` di luar `.claude/worktrees/` meminta persetujuan Anda sebelum memasuki, karena memindahkan direktori kerja sesi dan akses tulis ke lokasi tersebut. Pembuatan worktree baru dan path di bawah `.claude/worktrees/` tidak meminta. Sebelum v2.1.206, Claude memasuki path di luar `.claude/worktrees/` tanpa prompt. Dari dalam sesi worktree, atau dari subagent dengan direktori kerja yang disematkan seperti [`isolation: worktree`](/docs/id/sub-agents#supported-frontmatter-fields), hanya bentuk `path` yang tersedia dan target harus berada di bawah `.claude/worktrees/` dari repositori sesi                                                                 | Ya              |
| `ExitPlanMode`         | Menyajikan rencana untuk persetujuan dan keluar dari mode rencana                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 | Ya              |
| `ExitWorktree`         | Keluar dari sesi worktree dan kembali ke direktori asli. Tidak tersedia untuk subagent yang sudah berjalan di direktori kerja mereka sendiri, seperti dengan [`isolation: worktree`](/docs/id/sub-agents#supported-frontmatter-fields)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 | Tidak           |
| `Glob`                 | Menemukan file berdasarkan pencocokan pola. Tidak ada secara default di macOS, Linux, dan WSL. Lihat [Perilaku tool Glob](#glob-tool-behavior)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    | Tidak           |
| `Grep`                 | Mencari pola dalam konten file. Tidak ada secara default di macOS, Linux, dan WSL. Lihat [Perilaku tool Grep](#grep-tool-behavior)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                | Tidak           |
| `ListAgents`           | Mencantumkan agent yang dapat dihubungi Claude dengan `SendMessage`: subagent dalam sesi, rekan tim [tim agent](/docs/id/agent-teams), sesi Claude Code lokal Anda yang lain, dan, saat sesi ini terhubung ke [Remote Control](/docs/id/remote-control), sesi [Claude Code di web](/docs/id/claude-code-on-the-web) Anda dan sesi Remote Control Anda di mesin lain. Mendukung perintah `/list-agents`. Lihat [pesan lintas-sesi](/docs/id/cross-session-messaging). Memerlukan Claude Code v2.1.224 atau lebih baru, dan muncul hanya dalam sesi di mana [pesan lintas-sesi diaktifkan](/docs/id/cross-session-messaging#availability). Baris rekan tim dan baris pertama yang menunjukkan nama sesi ini sendiri memerlukan v2.1.239 atau lebih baru                                                                                                                                                                                                                                                                                                                      | Tidak           |
| `ListMcpResourcesTool` | Mencantumkan sumber daya yang diekspos oleh [server MCP](/docs/id/mcp) yang terhubung                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  | Tidak           |
| `LSP`                  | Intelijen kode melalui server bahasa: lompat ke definisi, temukan referensi, laporkan kesalahan tipe dan peringatan. Lihat [Perilaku tool LSP](#lsp-tool-behavior)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                | Tidak           |
| `Monitor`              | Menjalankan perintah di latar belakang dan memberi makan setiap baris output kembali ke Claude, sehingga dapat bereaksi terhadap entri log, perubahan file, atau status yang disurvei di tengah percakapan. Dapat juga membuka WebSocket dan memperlakukan setiap pesan masuk sebagai acara. Lihat [Tool Monitor](#monitor-tool)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  | Ya              |
| `NotebookEdit`         | Memodifikasi sel notebook Jupyter. Lihat [Perilaku tool NotebookEdit](#notebookedit-tool-behavior)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                | Ya              |
| `PowerShell`           | Menjalankan perintah PowerShell secara native. Lihat [Tool PowerShell](#powershell-tool) untuk ketersediaan                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       | Ya              |
| `PushNotification`     | Mengirim notifikasi desktop, dan push ponsel ketika [Remote Control](/docs/id/remote-control) terhubung, sehingga tugas yang berjalan lama atau [tugas terjadwal](/docs/id/scheduled-tasks) dapat menjangkau Anda ketika Anda pergi. Pengiriman push berjalan melalui infrastruktur yang dihosting Anthropic, yang tidak dapat diakses dari Amazon Bedrock, Claude Platform di AWS, Agent Platform Google Cloud, atau Microsoft Foundry                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     | Tidak           |
| `Read`                 | Membaca konten file. Lihat [Perilaku tool Read](#read-tool-behavior)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              | Tidak           |
| `ReadMcpResourceTool`  | Membaca sumber daya MCP tertentu berdasarkan URI                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  | Tidak           |
| `RemoteTrigger`        | Membuat, memperbarui, menjalankan, dan mencantumkan [Routines](/docs/id/routines) di claude.ai. Mendukung perintah `/schedule`. [Referensi input `RemoteTrigger`](/docs/id/agent-sdk/typescript#remotetrigger) mendokumentasikan setiap tindakan dan kebijakan organisasi yang menghapus tool. Routines tinggal di claude.ai dan memerlukan paket Pro, Max, Team, atau Enterprise, jadi tool ini tidak dapat diakses dari Amazon Bedrock, Claude Platform di AWS, Agent Platform Google Cloud, atau Microsoft Foundry                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       | Tidak           |
| `ReportFindings`       | Melaporkan temuan tinjauan kode sebagai daftar terstruktur, dengan file, ringkasan, dan skenario kegagalan per temuan, sehingga Claude Code dapat merender mereka alih-alih mencetaknya sebagai teks. Claude memanggilnya ketika instruksi tinjauan kode aktif memberitahunya untuk melakukannya. Memerlukan Claude Code v2.1.196 atau lebih baru. Mulai dari v2.1.199, temuan juga dapat membawa slug `category` opsional, seperti `correctness` atau `test-coverage`, ditampilkan di sebelah lokasi file dalam daftar yang dirender                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             | Tidak           |
| `ScheduleWakeup`       | Menjadwalkan ulang iterasi berikutnya dari [`/loop` yang berjalan sendiri](/docs/id/scheduled-tasks#let-claude-choose-the-interval). Claude memanggilnya di akhir setiap iterasi untuk memilih kapan yang berikutnya berjalan, antara satu menit dan satu jam ke depan; Anda tidak memanggilnya secara langsung. Untuk mengakhiri loop sebagai gantinya, Claude memanggilnya dengan `stop: true`, yang membatalkan wakeup yang tertunda. Bidang `stop` memerlukan Claude Code v2.1.202 atau lebih baru. Wakeup yang tertunda muncul dalam `session_crons` dalam [Input hook Stop](/docs/id/hooks#stop-input)                                                                                                                                                                                                                                                                                                                                                                                                                                                | Tidak           |
| `SendFeedback`         | Menyusun laporan umpan balik tentang Claude Code, mencakup masalah produk atau perilaku Claude sendiri dalam sesi, dan mengantreannya di mesin Anda untuk Anda tinjau. Claude Code tidak mengirim apa pun sampai Anda memilih untuk mengirim draf. Lihat [Perilaku tool SendFeedback](#sendfeedback-tool-behavior). Memerlukan Claude Code v2.1.238 atau lebih baru                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               | Tidak           |
| `SendMessage`          | Mengirim pesan ke agent lain: rekan tim [tim agent](/docs/id/agent-teams), [subagent yang dilanjutkan](/docs/id/sub-agents#resume-subagents) berdasarkan ID atau nama agent, atau salah satu sesi Claude Code Anda yang lain, di mesin ini atau di luar. Pesan ke sesi lain memerlukan Claude Code v2.1.224 atau lebih baru. [Pesan lintas-sesi](/docs/id/cross-session-messaging) mencakup sesi mana yang dapat dijangkau Claude, [seperti apa pesan ketika tiba](/docs/id/cross-session-messaging#what-a-message-looks-like), dan [bagaimana Claude mendapat pemberitahuan ketika sesi lain menjadi idle](/docs/id/cross-session-messaging#get-a-notice-when-another-session-goes-idle). Claude dapat menyertakan input `summary` opsional, biasanya 5-10 kata, yang ditampilkan Claude Code sebagai pratinjau satu baris. Ketika Claude menghilangkannya pada [pesan teks biasa](/docs/id/cross-session-messaging#limitations), Claude Code menggunakan baris pertama pesan sebagai ringkasan. Claude Code memotong ringkasan lebih panjang dari 200 karakter dengan elipsis | Tidak           |
| `SendUserFile`         | Mengirim file dari sesi ke Anda dengan keterangan opsional, sehingga laporan yang dihasilkan, diagram, tangkapan layar, atau artefak yang dibangun mencapai perangkat Anda alih-alih hanya disebutkan dalam transkrip. Mulai dari v2.1.196, input `display` opsional mengontrol presentasi: `render` membuka file inline dalam klien, `attach` menampilkan kartu unduhan saja, dan ketika tidak diatur klien memutuskan berdasarkan jenis file. Tersedia ketika klien [Remote Control](/docs/id/remote-control) terhubung atau dalam [sesi cloud](/docs/id/claude-code-on-the-web). Pengiriman berjalan melalui infrastruktur yang dihosting Anthropic, jadi tool tidak tersedia di Amazon Bedrock, Agent Platform Google Cloud, atau Microsoft Foundry                                                                                                                                                                                                                                                                                                     | Tidak           |
| `ShareOnboardingGuide` | Mengunggah `ONBOARDING.md` dan mengembalikan tautan berbagi yang dapat dibuka rekan tim di Claude Code. Dipanggil dari `/team-onboarding` setelah panduan ditulis. Tersedia untuk pelanggan claude.ai pada paket Pro, Max, Team, dan Enterprise                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   | Ya              |
| `Skill`                | Menjalankan [skill](/docs/id/skills#control-who-invokes-a-skill) dalam percakapan utama                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                | Ya              |
| `SubagentHandback`     | Mengirimkan laporan akhir subagent ke percakapan mana pun yang menerima hasil subagent tersebut. Disediakan hanya dalam [mode otomatis](/docs/id/permission-modes#eliminate-prompts-with-auto-mode), ke subagent yang dijalankan tool Agent secara lokal selain [fork](/docs/id/sub-agents#fork-the-current-conversation), dan tersedia di CLI terminal, ekstensi IDE, sesi cloud, dan Agent SDK; pengklasifikasi meninjau laporan sebelum dikirimkan. Memerlukan Claude Code v2.1.271 atau lebih baru                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      | Tidak           |
| `TaskCreate`           | Membuat tugas baru dalam daftar tugas. Disediakan secara default hanya pada model yang tercantum di bawah [Ketersediaan tool Task](#task-tool-availability), dan pada model lain ketika Anda memilih untuk ikut serta                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             | Tidak           |
| `TaskGet`              | Mengambil detail lengkap untuk tugas tertentu. Disediakan secara default hanya pada model yang tercantum di bawah [Ketersediaan tool Task](#task-tool-availability), dan pada model lain ketika Anda memilih untuk ikut serta                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     | Tidak           |
| `TaskList`             | Mencantumkan semua tugas dengan status saat ini mereka. Disediakan secara default hanya pada model yang tercantum di bawah [Ketersediaan tool Task](#task-tool-availability), dan pada model lain ketika Anda memilih untuk ikut serta                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            | Tidak           |
| `TaskOutput`           | Mengambil output dari tugas latar belakang. Tidak direkomendasikan lagi mendukung `Read` pada jalur file output tugas. Ketika tidak ada tugas yang cocok dengan ID, kesalahan mencantumkan agent latar belakang yang berjalan berdasarkan ID dan deskripsi. Sebelum v2.1.203, kesalahan hanya menyebutkan ID yang hilang                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          | Tidak           |
| `TaskStop`             | Menghentikan tugas latar belakang yang berjalan berdasarkan ID. Ini juga menerima [rekan tim tim agent](/docs/id/agent-teams) atau agent latar belakang bernama berdasarkan ID atau nama agent. Sebelum v2.1.198, itu hanya menerima ID tugas latar belakang. Ketika tidak ada tugas yang cocok dengan ID, kesalahan mencantumkan agent latar belakang yang berjalan berdasarkan ID dan deskripsi, termasuk agent yang agent lain jalankan. Sebelum v2.1.203, kesalahan mencantumkan rekan tim dan agent bernama yang berjalan tetapi bukan agent latar belakang yang agent lain jalankan, jadi mereka tidak dapat diidentifikasi atau dihentikan dari percakapan utama                                                                                                                                                                                                                                                                                                                                                                                | Tidak           |
| `TaskUpdate`           | Memperbarui status tugas, dependensi, detail, atau menghapus tugas. Disediakan secara default hanya pada model yang tercantum di bawah [Ketersediaan tool Task](#task-tool-availability), dan pada model lain ketika Anda memilih untuk ikut serta                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                | Tidak           |
| `TodoWrite`            | Mengelola daftar periksa tugas sesi. Dinonaktifkan secara default mendukung `TaskCreate`, `TaskGet`, `TaskList`, dan `TaskUpdate`. Atur `CLAUDE_CODE_ENABLE_TASKS=0` untuk mengaktifkannya kembali dalam [sesi yang memiliki tools pelacakan tugas](#task-tool-availability)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      | Tidak           |
| `ToolSearch`           | Mencari dan memuat tools yang ditangguhkan ketika [pencarian tool](/docs/id/mcp#scale-with-mcp-tool-search) diaktifkan                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 | Tidak           |
| `WaitForMcpServers`    | Menunggu satu atau lebih [server MCP](/docs/id/mcp) yang masih terhubung di latar belakang, sehingga permintaan dapat menggunakan tools mereka tanpa memulai ulang sesi. Claude memanggilnya ketika server yang diperlukan belum terhubung. Hanya muncul ketika [pencarian tool](/docs/id/mcp#scale-with-mcp-tool-search) dinonaktifkan, karena `ToolSearch` menangani tunggu ketika diaktifkan                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             | Tidak           |
| `WebFetch`             | Mengambil konten dari URL yang ditentukan. Lihat [Perilaku tool WebFetch](#webfetch-tool-behavior)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                | Ya              |
| `WebSearch`            | Melakukan pencarian web. Lihat [Perilaku tool WebSearch](#websearch-tool-behavior)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                | Ya              |
| `Workflow`             | Menjalankan [alur kerja dinamis](/docs/id/workflows): skrip yang mengorkestra banyak subagent di latar belakang dan mengembalikan satu hasil yang dikonsolidasikan                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     | Ya              |
| `Write`                | Membuat atau menimpa file. Lihat [Perilaku tool Write](#write-tool-behavior)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      | Ya              |

<h2 id="configure-tools-with-permission-rules-and-hooks">
  Konfigurasi tools dengan aturan izin dan hooks
</h2>

Sebagian besar waktu, Claude memutuskan kapan menggunakan tools ini dan Anda tidak perlu menamakannya sendiri saat berinteraksi dengan Claude. Anda mereferensikan nama tools secara langsung saat menentukan izin dan konfigurasi lainnya:

* dalam [`permissions.allow`](/docs/id/settings-reference#permissions-allow) dan [`permissions.deny`](/docs/id/settings-reference#permissions-deny) dalam pengaturan, dan antarmuka `/permissions`
* dalam flag CLI [`--allowedTools` dan `--disallowedTools`](/docs/id/cli-reference)
* dalam opsi Agent SDK [`allowedTools` dan `disallowedTools`](/docs/id/agent-sdk/permissions#allow-and-deny-rules)
* dalam frontmatter [`allowed-tools`](/docs/id/skills#frontmatter-reference) skill
* dalam kondisi [`if`](/docs/id/hooks-guide#filter-by-tool-name-and-arguments-with-the-if-field) hook

Semua ini menerima format aturan yang sama, `ToolName(specifier)`. Specifier tergantung pada tools, dan beberapa tools berbagi format:

| Format aturan                  | Berlaku untuk             | Detail                                                                     |
| :----------------------------- | :------------------------ | :------------------------------------------------------------------------- |
| `Bash(npm run *)`              | Bash, Monitor             | [Pencocokan pola perintah](/docs/id/permissions#bash)                           |
| `PowerShell(Get-ChildItem *)`  | PowerShell                | [Pencocokan pola perintah](/docs/id/permissions#powershell)                     |
| `Read(~/secrets/**)`           | Read, Grep, Glob, LSP     | [Pencocokan pola jalur](/docs/id/permissions#read-and-edit)                     |
| `Edit(/src/**)`                | Edit, Write, NotebookEdit | [Pencocokan pola jalur](/docs/id/permissions#read-and-edit)                     |
| `Skill(deploy *)`              | Skill                     | [Pencocokan nama skill](/docs/id/skills#restrict-claude%E2%80%99s-skill-access) |
| `Agent(Explore)`               | Agent                     | [Pencocokan tipe subagent](/docs/id/permissions#agent-subagents)                |
| `WebFetch(domain:example.com)` | WebFetch                  | [Pencocokan domain](/docs/id/permissions#webfetch)                              |
| `WebSearch`                    | WebSearch                 | Tidak ada specifier; izinkan atau tolak tools secara keseluruhan           |

Tools yang tidak tercantum di sini, seperti `ExitPlanMode` atau `ShareOnboardingGuide`, hanya menerima nama tools tanpa specifier.

Aturan izin `Edit(...)` juga memberikan akses baca ke jalur yang sama, jadi Anda tidak perlu aturan `Read(...)` yang cocok. Aturan penolakan `Read(...)` juga memblokir tools Edit dan Write pada jalur yang sama, termasuk membuat file baru di sana, karena kedua tools mengubah konten yang harus dapat dibaca kembali oleh Claude. Pemeriksaan penolakan `Read` memerlukan Claude Code v2.1.208 atau lebih baru pada edit, dan v2.1.228 atau lebih baru pada write.

Field `matcher` Hook menggunakan nama tools tanpa format yang diparenthesiskan. Lihat [pola matcher](/docs/id/hooks#matcher-patterns) untuk aturan pencocokan. Untuk nama field yang setiap tools teruskan ke `tool_input` dalam hooks, lihat [referensi input PreToolUse](/docs/id/hooks#pretooluse-input).

<h2 id="agent-tool-behavior">
  Perilaku tool Agent
</h2>

Tool Agent menjalankan subagent dalam jendela konteks terpisah. Subagent bekerja melalui tugasnya secara otonom, kemudian mengembalikan hasilnya ke percakapan induk. Induk tidak melihat panggilan tool perantara atau output subagent, hanya hasil akhir itu. Dengan [agent teams](/docs/id/agent-teams) diaktifkan, panggilan yang membawa `name` dapat meluncurkan [teammate](/docs/id/agent-teams#how-claude-starts-agent-teams) sebagai gantinya, yang melaporkan kembali melalui pesan tim daripada dengan mengembalikan hasil.

Untuk membatasi berapa banyak putaran yang dijalankan subagent, atur `maxTurns` dalam [definisi subagent](/docs/id/sub-agents#supported-frontmatter-fields). Ketika subagent mencapai batas, Claude Code menandai hasil yang dikembalikan sebagai output parsial, dan Claude dapat [melanjutkan subagent](/docs/id/sub-agents#resume-subagents) untuk melanjutkan.

Tool Agent yang sama juga meluncurkan [forked subagents](/docs/id/sub-agents#fork-the-current-conversation) di mana pun [fork mode](/docs/id/sub-agents#turn-fork-mode-on-or-off) aktif. Fork mewarisi percakapan induk penuh daripada memulai dari awal, berjalan di latar belakang terlepas dari [kasus yang tetap di latar depan](/docs/id/sub-agents#run-subagents-in-foreground-or-background), dan masih menampilkan prompt izin di terminal Anda. Sisa bagian ini menjelaskan subagent non-fork.

Tool mana yang dapat digunakan subagent non-fork tergantung pada bidang `tools` dan `disallowedTools` dalam [definisi subagent](/docs/id/sub-agents):

* **Tidak ada bidang yang diatur**: subagent mewarisi setiap [tool yang tersedia untuk subagents](/docs/id/sub-agents#available-tools).
* **`tools` saja**: subagent hanya mendapatkan tool yang terdaftar.
* **`disallowedTools` saja**: subagent mendapatkan setiap tool induk kecuali yang terdaftar.
* **Keduanya diatur**: `disallowedTools` memiliki prioritas. Tool yang terdaftar di keduanya dihapus.

Dalam setiap kasus, set yang diselesaikan dibatasi pada [tool yang tersedia untuk subagents](/docs/id/sub-agents#available-tools): tool yang tidak tersedia untuk subagents tidak pernah diberikan, bahkan ketika terdaftar dalam `tools`. Di mana kondisi dalam entri tabel tool `SubagentHandback` berlaku, Claude Code juga memberikan tool itu kepada subagent, bahkan jika Anda menghilangkannya dari `tools` atau mencantumkannya dalam `disallowedTools`.

Jika setiap entri dalam daftar `tools` subagent gagal cocok dengan tool yang dapat digunakan, tool Agent biasanya mengembalikan kesalahan yang menamai entri daripada meluncurkan subagent; lihat [Agent would be spawned with zero tools](/docs/id/errors#agent-would-be-spawned-with-zero-tools) untuk pesan dan cara memperbaiki setiap entri.

Meluncurkan subagent tidak sendiri meminta izin. Claude Code memeriksa panggilan tool subagent sendiri terhadap aturan izin Anda saat berjalan.

Di mana Anda melihat prompt izin subagent tergantung pada apakah berjalan di latar depan atau latar belakang. Claude Code menjalankan subagent di latar belakang secara default, terlepas dari [kasus yang berjalan di latar depan](/docs/id/sub-agents#run-subagents-in-foreground-or-background).

* **Subagent latar depan** menampilkan prompt izin yang sama yang akan Anda lihat dalam percakapan utama, pada saat setiap panggilan tool terjadi.
* **Subagent latar belakang** menampilkan prompt izin di sesi utama Anda sejak v2.1.186. Prompt menamai subagent mana yang meminta, dan menekan Esc menolak panggilan tool itu saja tanpa menghentikan subagent. Sebelum v2.1.186, subagent latar belakang secara otomatis menolak panggilan tool apa pun yang akan meminta sebaliknya dan melanjutkan tanpa tool itu.

Untuk [membatasi apa yang dapat dijangkau subagent](/docs/id/sub-agents#control-subagent-capabilities) di tempat pertama, persempit bidang `tools` nya, misalnya dengan meninggalkan Bash dari daftar, atau atur aturan penolakan dalam pengaturan Anda.

<h2 id="askuserquestion-tool-behavior">
  Perilaku tool AskUserQuestion
</h2>

Claude menggunakan `AskUserQuestion` untuk mengajukan pertanyaan pilihan ganda kepada Anda ketika memerlukan keputusan atau klarifikasi. Jawab dengan memilih opsi, atau ketik teks Anda sendiri melalui baris `Other` atau kolom catatan.

Ketika Anda menjawab dengan mengetik teks Anda sendiri, Claude Code menyampaikan jawaban dengan redaksi netral sehingga Claude mengikuti apa yang Anda tulis, termasuk permintaan untuk menunggu atau menjelaskan terlebih dahulu.

<h3 id="question-auto-continue-timeout">
  Timeout auto-continue pertanyaan
</h3>

Pertanyaan tetap terbuka sampai Anda menjawabnya. Jika Anda ingin pertanyaan yang Anda biarkan tanpa jawaban akhirnya ditutup dan membiarkan Claude melanjutkan tanpa Anda, atur pengaturan [`askUserQuestionTimeout`](/docs/id/settings-reference#askuserquestiontimeout) ke `60s`, `5m`, atau `10m`, baik di `settings.json` pengguna Anda atau dari baris **Question auto-continue timeout** di `/config`.

Setelah pertanyaan duduk selama itu tanpa input, dialog ditutup dengan sendirinya: ia mengirimkan opsi apa pun yang sudah Anda pilih dan memberi tahu Claude bahwa Anda mungkin jauh dari keyboard, jadi Claude melanjutkan dengan penilaian sendiri dan dapat menanyakan kembali nanti. Anda melihat hitungan mundur untuk 20 detik terakhir. Tekan tombol apa pun untuk memulai ulang timer; di terminal yang melaporkan fokus, beralih ke jendela juga memulai ulangnya.

Timeout hanya berlaku untuk pertanyaan pilihan ganda `AskUserQuestion`; prompt izin, termasuk persetujuan rencana, tidak pernah auto-resolve saat idle.

<h2 id="bash-tool-behavior">
  Perilaku alat Bash
</h2>

Alat Bash menjalankan setiap perintah dalam proses terpisah.

<h3 id="what-persists-between-commands">
  Apa yang bertahan di antara perintah
</h3>

* Ketika Claude menjalankan `cd` dalam sesi utama, direktori kerja baru terbawa ke perintah Bash yang lebih baru selama tetap berada di dalam direktori proyek atau [direktori kerja tambahan](/docs/id/permissions#working-directories) yang Anda tambahkan dengan `--add-dir`, `/add-dir`, atau `additionalDirectories` dalam pengaturan. Ini termasuk perintah yang Claude jalankan sebagai respons terhadap pesan Anda yang lebih baru.
  * Sesi subagent tidak pernah membawa perubahan direktori kerja.
  * Jika `cd` mendarat di luar direktori tersebut, Claude Code mengatur ulang ke direktori proyek dan menambahkan `Shell cwd was reset to <dir>` ke hasil alat.
  * Untuk menonaktifkan carry-over ini sehingga setiap perintah Bash dimulai di direktori proyek, atur `CLAUDE_BASH_MAINTAIN_PROJECT_WORKING_DIR=1`.
* Variabel lingkungan tidak bertahan. `export` dalam satu perintah tidak akan tersedia di perintah berikutnya.
* Alias dan fungsi shell yang ditentukan dalam file startup shell Anda tersedia. Pada awal sesi, Claude Code bersumber dari `~/.zshrc`, `~/.bashrc`, atau `~/.profile` tergantung pada shell Anda, menangkap alias, fungsi, dan opsi shell yang dihasilkan, dan menerapkannya ke setiap perintah Bash.

Aktifkan virtualenv atau lingkungan conda Anda sebelum meluncurkan Claude Code. Untuk membuat variabel lingkungan bertahan di seluruh perintah Bash, atur [`CLAUDE_ENV_FILE`](/docs/id/env-vars) ke skrip shell sebelum meluncurkan Claude Code, atau gunakan [hook SessionStart](/docs/id/hooks#persist-environment-variables) untuk mengisinya secara dinamis.

<h3 id="timeout-and-output-limits">
  Batas waktu dan output
</h3>

Setiap perintah berjalan di bawah batas waktu, dan Claude mengelolanya: ketika menginginkan lebih lama dari default untuk perintah, ia melewatkan parameter `timeout` dengan panggilan itu — Anda tidak pernah menetapkan batas waktu per perintah. Dua [variabel lingkungan](/docs/id/env-vars) membatasi apa yang Claude dapatkan:

* `BASH_DEFAULT_TIMEOUT_MS` — default ketika Claude tidak melewatkan batas waktu; dua menit di luar kotak
* `BASH_MAX_TIMEOUT_MS` — dengan default, menetapkan batas yang membatasi apa pun yang Claude minta: batas efektif adalah yang lebih besar dari keduanya, sepuluh menit di luar kotak

<h4 id="output-limits">
  Batas output
</h4>

Claude Code mengalirkan output perintah ke file kerja saat perintah berjalan; perintah yang outputnya melampaui 5 GB dibunuh. Ketika perintah selesai, Claude Code membaca output kembali dari file itu, hingga jendela read-back yang dijelaskan di bawah. Berapa banyak output yang mencapai Claude secara inline tergantung pada apakah Claude Code memperlakukan hasil sebagai kegagalan:

| Hasil     | Apa yang Claude dapatkan                                                                                                                                                                                                                                           |
| :-------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Valid     | Inline hingga kira-kira 30.000 karakter secara default; melampaui itu, jalur file yang disimpan ke direktori sesi dan dipotong melampaui 64 MiB, ditambah pratinjau hingga 2.000 karakter pertama, dan Claude membaca atau mencari file ketika membutuhkan sisanya |
| Kegagalan | Inline hingga kira-kira 10.000 karakter; melampaui itu, kutipan kepala-dan-ekor ukuran itu dipotong dari jendela read-back, tanpa jalur file                                                                                                                       |

Perintah yang keluar 1 dihitung sebagai hasil valid untuk alat Bash hanya ketika Claude Code mengenali kode keluar 1 sebagai hasil yang jinak untuk perintah itu: `grep`, `rg`, `egrep`, `fgrep`, `find`, `diff`, `test`, dan `[`, ditambah `git diff` dan `git grep`. Setiap perintah lain yang keluar 1 dihitung sebagai kegagalan, bahkan ketika keluar 1 adalah hasil informasional yang jinak: tidak ada kecocokan untuk `pgrep` dan `jq -e`, file yang berbeda untuk `cmp`.

[`BASH_MAX_OUTPUT_LENGTH`](/docs/id/env-vars) menetapkan berapa banyak karakter output yang Claude Code baca kembali dari file kerja ke hasil perintah: 30.000 secara default, hingga batas keras 150.000. Naikkan ketika perintah Anda secara rutin melampaui jendela itu, seperti build verbose atau log test-suite lengkap. Menaikkannya memperbesar jendela read-back, yang juga merupakan jendela kutipan perintah yang gagal dipotong. Itu tidak menaikkan batas inline: hasil valid di atas batas inline tiba sebagai jalur file ditambah pratinjau terlepas dari variabel ini.

Untuk mengubah berapa banyak hasil valid yang Claude terima secara inline, atur pengaturan [`bashOutputMaxChars`](/docs/id/settings-reference#bashoutputmaxchars) sebagai gantinya, hingga 128.000 karakter. Ini mengukur batas inline dan jendela read-back bersama-sama, dan Claude Code kemudian mengabaikan `BASH_MAX_OUTPUT_LENGTH`. Memerlukan Claude Code v2.1.261 atau lebih baru.

<h3 id="background-commands">
  Perintah latar belakang
</h3>

Untuk proses yang berjalan lama seperti server dev atau build watch, Claude dapat mengatur `run_in_background: true` untuk memulai perintah sebagai tugas latar belakang dan terus bekerja saat berjalan. Daftar dan hentikan tugas latar belakang dengan `/tasks`. Setelah Anda menghentikan satu di sana, atau dari klien yang terhubung seperti aplikasi desktop, Claude melanjutkan alih-alih menunggu. Jika subagent memulai perintah, itu adalah subagent itu yang melanjutkan.

Perintah yang dimulai oleh [subagent foreground](/docs/id/sub-agents#run-subagents-in-foreground-or-background) berhenti ketika subagent itu memberikan respons finalnya. Perintah yang dimulai oleh percakapan utama atau subagent latar belakang terus berjalan setelah respons final. Dalam mode non-interaktif dengan flag `-p`, [perintah latar belakang berakhir segera setelah hasil final run](/docs/id/headless#background-tasks-at-exit).

Ketika perintah mencapai batas waktunya tanpa selesai, Claude Code memindahkannya ke latar belakang alih-alih menghentikannya, kecuali perintah dimulai dengan `sleep`. Claude terus bekerja saat perintah berlanjut. Claude Code menerapkan aturan lifetime yang sama ke perintah yang dipindahkan seperti ke perintah latar belakang lainnya, jadi masih menghentikan perintah subagent foreground pada respons final subagent itu. Mengatur [`CLAUDE_CODE_DISABLE_BACKGROUND_TASKS=1`](/docs/id/env-vars#variables) menonaktifkan auto-backgrounding bersama dengan sisa fungsionalitas tugas latar belakang.

Hasil perintah yang dipindahkan ke latar belakang menyatakan apa yang terjadi:

* Ketika batas waktu memicu perpindahan, hasil melaporkannya secara eksplisit: `Command did not complete within its 120s timeout and was moved to the background`, dengan detik sesuai dengan batas waktu yang berlaku, diikuti oleh ID tugas dan jalur file tempat output ditulis.
* Sebuah `cd`, `pushd`, `popd`, atau `chdir` di dalam perintah yang dipindahkan ke latar belakang tidak pernah terbawa: hasil menyatakan `Session cwd remains <dir>; directory changes made by the backgrounded command do not apply to subsequent commands.`, jadi Claude tidak bertindak atas perubahan direktori yang tidak terjadi.

<h3 id="memory-limit-on-linux-and-wsl">
  Batas memori di Linux dan WSL
</h3>

Di Linux dan WSL, atur [`CLAUDE_CODE_TOOL_MEMORY_LIMIT`](/docs/id/env-vars#variables) ke ukuran seperti `4G` untuk membatasi memori yang perintah Bash, PowerShell, dan alat [Monitor](#monitor-tool) dapat gunakan, sehingga satu build yang lari tidak dapat mengambil memori yang sesi lainnya butuhkan. Memerlukan Claude Code v2.1.233 atau lebih baru. Sebelum v2.1.246, perintah alat Monitor berjalan di luar batas.

* Tulis ukuran sebagai jumlah byte atau dengan akhiran `K`, `M`, `G`, atau `T`. Atur `0`, `off`, `false`, `no`, atau `none` untuk mematikan batas. Claude Code mengabaikan nilai apa pun yang tidak dapat dibacanya sebagai ukuran, seperti `4e9`.
* Claude Code menghitung semua perintah Bash, PowerShell, dan Monitor sesi terhadap satu batas, bukan setiap perintah pada dirinya sendiri.
* Claude Code menerapkan batas dengan cgroup memori. Ketika tidak dapat mengatur cgroup, perintah berjalan tanpa batas, dan log debug dari `claude --debug` mengatakan mengapa.
* Setelah proses pertama yang Claude Code mulai telah menghidupkan batas, atau telah mematikannya karena nilai off atau setup cgroup yang gagal, Claude Code menahan hasil itu sampai Anda meluncurkan kembali. Untuk menerapkan nilai yang diubah atau dihapus, atau setup yang diperbaiki, luncurkan `claude` lagi.
* Ketika perintah tidak dapat tetap di bawah batas, kernel membunuh perintah, dan tidak ada dalam hasilnya yang menyebutkan batas.

Claude Code juga dapat menghitung jenis proses lain yang dimulainya terhadap batas yang sama. Atur [`CLAUDE_CODE_TOOL_MEMORY_CGROUP_EXCLUDE`](/docs/id/env-vars#variables) ke daftar yang dipisahkan koma dari jenis yang dikecualikan dari batas; Claude Code menerapkan batas ke setiap jenis yang tidak ada di daftar Anda. Atur ke `none` untuk membatasi setiap jenis, atau ke `all-new` untuk membatasi hanya perintah alat Bash, PowerShell, dan Monitor. Memerlukan Claude Code v2.1.246 atau lebih baru. Jenis yang dapat Anda namai:

* `mcp`: server [MCP](/docs/id/mcp) lokal
* `lsp`: [server bahasa](#lsp-tool-behavior)
* `hooks`: perintah [hook](/docs/id/hooks)
* `plugin`: perintah yang [plugin](/docs/id/plugins/overview) jalankan
* `helper`: perintah helper Claude Code sendiri, seperti `git`
* `agent`: proses Claude Code anak, seperti [rekan tim agen](/docs/id/agent-teams)

Apa pun yang Anda daftar, aturan ini berlaku:

* **Nama tidak dikenal**: Claude Code mengabaikan nama yang tidak dikenalinya
* **Bash, PowerShell, dan Monitor**: Claude Code menjaga perintah alat Bash, PowerShell, dan Monitor di bawah batas apa pun yang Anda daftar
* **Variabel tidak diatur**: Claude Code mengambil set jenis yang dibatasi lainnya dari konfigurasi yang Anthropic berikan dari server, dan set itu dapat berubah seiring waktu, jadi atur variabel ketika Anda membutuhkan set yang tidak berubah
* **Hook gating izin**: bahkan dengan setiap jenis dibatasi, Claude Code mengecualikan dari batas hook yang dapat memblokir atau mengubah hasil tindakan, dan server MCP apa pun yang hook tersebut panggil, sehingga kernel membunuh hook gating izin tidak dapat memungkinkan tindakan yang sedang diblokir.

<h2 id="edit-tool-behavior">
  Perilaku alat Edit
</h2>

Alat Edit melakukan penggantian string yang tepat. Alat ini mengambil `old_string` dan `new_string` dan mengganti yang pertama dengan yang kedua. Alat ini tidak menggunakan regex atau fuzzy matching.

Tiga pemeriksaan harus lulus agar edit dapat diterapkan. Sebelum salah satu dari mereka, jalur yang cocok dengan [aturan deny `Read`](/docs/id/permissions#tool-specific-permission-rules) ditolak, termasuk membuat file baru di sana. Penolakan memerlukan Claude Code v2.1.208 atau lebih baru.

* **Read-before-edit**: Claude membaca file dalam percakapan saat ini sebelum mengeditnya, dan pembacaan yang terpotong dengan pemberitahuan [`PARTIAL view`](#read-tool-behavior) tidak dihitung. Claude Opus 4.6, Claude Haiku 4.5, dan model yang lebih lama selalu memerlukan pembacaan. Model yang lebih baru dapat mengedit file yang belum dibaca ketika membacanya tidak memerlukan prompt izin dan alat Read tersedia.
* **Match**: `old_string` harus muncul dalam file persis seperti yang ditulis. Perbedaan spasi atau indentasi satu karakter saja sudah cukup untuk tidak cocok.
* **Uniqueness**: `old_string` harus muncul tepat satu kali. Ketika muncul lebih dari sekali, Claude baik menyediakan string yang lebih panjang dengan konteks sekitar yang cukup untuk menentukan satu kemunculan, atau menetapkan `replace_all: true` untuk mengganti semuanya.

File yang berubah di disk setelah Claude terakhir membacanya masih dapat diedit ketika `old_string` cocok dengan konten saat ini dengan tepat dan jelas serta Claude Code dapat membaca file tanpa meminta. Pencocokan terhadap konten file saat ini menjaga keamanan ini, dan hasilnya mencatat bahwa file membawa perubahan lain sehingga Claude membacanya kembali sebelum edit yang bergantung pada konten sekitar. Dalam kasus lain, seperti `old_string` yang sudah usang atau yang cocok lebih dari sekali tanpa `replace_all`, Claude membaca file lagi sebelum mengedit. Penanganan yang santai terhadap file yang belum dibaca dan berubah memerlukan Claude Code v2.1.208 atau lebih baru; sebelum itu, Claude Code menolak edit apa pun ke file yang belum dibacanya dalam percakapan atau yang berubah di disk setelah pembacaan.

Melihat file dengan Bash juga memenuhi persyaratan read-before-edit ketika perintahnya adalah `cat`, `nl`, `bat`, `batcat`, `head`, `tail`, `sed -n 'X,Yp'`, `grep`, `egrep`, `fgrep`, atau `rg` pada satu file tanpa pipa atau pengalihan. Output yang dipipa dan perintah Bash lainnya tidak dihitung terhadap pemeriksaan read-before-edit.

Melihat file dengan Bash mempengaruhi kelayakan edit saja, bukan izin. Lihat [Aturan izin Read dan Edit](/docs/id/permissions#read-and-edit) untuk perintah Bash mana yang dicakup oleh aturan deny `Read` dan `Edit` Anda.

<h2 id="endconversation-tool-behavior">
  Perilaku tool EndConversation
</h2>

Tool EndConversation mengakhiri sesi saat ini. Claude menggunakannya hanya dalam dua situasi:

* sebagai upaya terakhir melawan input yang terus-menerus kasar, setelah upaya untuk mengalihkan percakapan telah gagal dan setelah peringatan yang jelas dalam pesan sebelumnya
* ketika Anda secara eksplisit meminta untuk melihat tool yang ditunjukkan dan mengonfirmasi bahwa Anda ingin sesi berakhir

Frustrasi umum, kata-kata kasar, atau tugas yang berjalan buruk tidak memenuhi syarat, begitu juga dengan permintaan konten berbahaya, yang Claude tolak daripada mengakhiri sesi. Claude Code mengikuti pendekatan yang sama seperti claude.ai, yang dapat [mengakhiri subset chat yang jarang terjadi](https://www.anthropic.com/research/end-subset-conversations).

Setelah Claude mengakhiri sesi interaktif, sesi terkunci. Prompt baru dan sebagian besar perintah mengembalikan `Claude ended this conversation. Start a new session (or /clear) to continue.`, dan hanya `/clear`, `/resume`, `/help`, `/exit`, dan `/feedback` yang masih berjalan. Claude Code mencatat akhir dalam transkrip sesi, jadi melanjutkan sesi yang berakhir memulihkan kunci; riwayat sesi tidak dihapus.

Melanjutkan sesi yang berakhir dalam [mode non-interaktif](/docs/id/headless) dengan flag `-p` menghasilkan error dan keluar dengan kode 1, sehingga skrip tidak membaca run yang berakhir sebagai kesuksesan.

Tool tidak pernah meminta izin, dan [PreToolUse hooks](/docs/id/hooks#pretooluse) tidak berjalan untuk itu. Selama tool lain tetap ada, Anda tidak dapat memblokir itu juga: [aturan deny dan ask](/docs/id/permissions#tool-specific-permission-rules) yang menyebutkan `EndConversation` tidak berpengaruh, dan baik `--disallowedTools` maupun daftar `--tools` dapat menghapusnya. Pengecualian ini disengaja: tool tidak melakukan apa pun kecuali mengakhiri percakapan, tidak pernah membaca atau memodifikasi file atau data, dan safeguard semacam ini hanya berlaku jika sesi yang diterapkan tidak dapat mematikannya. Ketika aturan deny Anda menghapus setiap tool lain dan juga cocok dengan `EndConversation`, seperti yang dilakukan `"*"`, Claude Code menghapusnya juga daripada meninggalkannya sebagai satu-satunya tool, kecuali jika aturan allow secara eksplisit menyebutkan `EndConversation`. Daftar deny yang menghapus setiap tool lain tanpa cocok dengan `EndConversation` membiarkannya tetap ada.

[Subagents](/docs/id/sub-agents) tidak pernah mendapatkan tool. Tugas latar belakang yang berbagi daftar tool percakapan utama melihatnya, tetapi memanggilnya di sana tidak mengakhiri apa pun.

Tool muncul hanya ketika semua hal berikut berlaku:

* **Version**: Claude Code v2.1.213 atau lebih baru.
* **Model**: model sesi adalah Claude Opus 4.8, Claude Sonnet 5, Claude Fable 5, atau versi yang lebih baru dari salah satu keluarga tersebut.
* **Surface**: sesi terminal interaktif, termasuk sesi `claude` di terminal terintegrasi IDE, yang merupakan cara [plugin JetBrains](/docs/id/jetbrains) menjalankannya. Permukaan lain tidak menyertakan tool, seperti:
  * run `-p` non-interaktif
  * sesi melalui paket TypeScript dan Python [Agent SDK](/docs/id/agent-sdk/overview)
  * panel [VS Code extension](/docs/id/vs-code), yang menggabungkan CLI-nya sendiri
  * [GitHub Actions](/docs/id/github-actions)
  * [Claude Code di web](/docs/id/claude-code-on-the-web)
* **Startup mode**: bukan sesi [`--bare`](/docs/id/headless#start-faster-with-bare-mode). Mode bare hanya memuat shell dan file tools, jadi tool tidak pernah terdaftar di sana.
* **Provider**: tidak tersedia di [Amazon Bedrock](/docs/id/amazon-bedrock), [Claude Platform on AWS](/docs/id/claude-platform-on-aws), [Google Cloud's Agent Platform](/docs/id/google-vertex-ai), atau [Microsoft Foundry](/docs/id/microsoft-foundry), atau di sesi yang masuk melalui [cloud gateway](/docs/id/claude-apps-gateway).

<h2 id="glob-tool-behavior">
  Perilaku Glob tool
</h2>

Glob tool menemukan file berdasarkan pola nama. Di Windows, ini adalah bagian dari set tool default. Di macOS, Linux, dan WSL, Claude Code mengeluarkan Glob dan [Grep](#grep-tool-behavior) dari set tool default, dan Claude mencari dengan `find` dan `grep` melalui Bash tool sebagai gantinya. Di shell Claude, kedua perintah tersebut menjalankan versi embedded dari `bfs` dan `ugrep`, dan pencarian mencapai hooks dan aturan izin Anda sebagai panggilan `Bash`.

Di macOS, Linux, dan WSL, Anda mendapatkan kembali Glob dan Grep tools dalam kasus-kasus ini:

* Anda menyebutkan `Glob` atau `Grep` dalam [`--tools` atau `--allowedTools`](/docs/id/cli-reference#cli-flags) ketika Anda memulai sesi, atau dalam opsi [Agent SDK](/docs/id/agent-sdk/overview) yang setara. Dengan `--tools` Anda mendapatkan yang Anda daftarkan, dan menyebutkan salah satu tool dalam `--allowedTools` mengembalikan keduanya. Aturan allow dalam file settings tidak memiliki efek ini.
* Aturan [deny](/docs/id/permissions#match-all-uses-of-a-tool) izin, flag `--disallowedTools`, atau [`--restricted`](/docs/id/cli-reference#cli-flags) menghapus `Bash` dari sesi.
* [Subagent](/docs/id/sub-agents#available-tools) mencantumkan `Glob` atau `Grep` dalam field `tools` dan mengeluarkan `Bash`. Tool yang tercantum kembali untuk subagent itu saja, atau untuk seluruh sesi ketika berjalan sebagai agent sesi utama melalui [`--agent`](/docs/id/sub-agents#invoke-subagents-explicitly) atau pengaturan `agent`.

Glob mendukung sintaks glob standar termasuk `**` untuk pencocokan direktori rekursif:

* `**/*.js` cocok dengan semua file `.js` pada kedalaman apa pun
* `src/**/*.ts` cocok dengan semua file `.ts` di bawah `src/`
* `*.{json,yaml}` cocok dengan file `.json` dan `.yaml` dalam direktori saat ini

Hasil diurutkan berdasarkan waktu modifikasi dan dibatasi pada 100 file. Jika batas tercapai, Claude melihat flag truncation dalam hasil dan dapat mempersempit pola.

Glob tidak menghormati `.gitignore` secara default, jadi menemukan file gitignored bersama yang dilacak. Ini berbeda dari [Grep](#grep-tool-behavior), yang melewati file gitignored. Untuk membuat Glob menghormati `.gitignore`, atur `CLAUDE_CODE_GLOB_NO_IGNORE=false` sebelum meluncurkan Claude Code.

Claude Code memutuskan izin untuk panggilan Glob sebelum memeriksa apakah direktori pencarian ada. Ini masih menjalankan pemeriksaan izin baca untuk `path` yang hilang di luar [direktori kerja](/docs/id/permissions#working-directories), jadi prompt izin untuk path tidak berarti path tersebut ada.

Nilai `pattern` atau `path` yang berisi byte null mengembalikan kesalahan yang meminta Claude untuk menghapusnya.&#x20;

<h2 id="grep-tool-behavior">
  Perilaku Grep tool
</h2>

Grep tool mencari konten file untuk pola. Di mana [Glob](#glob-tool-behavior) menemukan file berdasarkan nama, Grep menemukan baris di dalamnya. Pada macOS, Linux, dan WSL, Grep tidak ada secara default dalam kondisi yang sama dengan Glob. Lihat [Perilaku Glob tool](#glob-tool-behavior) untuk mengetahui kapan kedua tool tersedia.

Grep dibangun di atas [ripgrep](https://github.com/BurntSushi/ripgrep) dan menggunakan sintaks regex ripgrep, bukan POSIX grep. Pola yang mencakup karakter metacharacter regex perlu escape. Misalnya, menemukan `interface{}` dalam kode Go memerlukan pola `interface\{\}`.

Pola, glob, atau tipe file yang ditolak oleh ripgrep mengembalikan kesalahan yang mencakup diagnostik ripgrep, sehingga Claude dapat memperbaiki input dan mencari lagi. Sebelum v2.1.208, Claude Code melaporkan input yang ditolak sebagai `No files found` alih-alih kesalahan, bahkan ketika teks yang dicari ada di file target.

Tiga mode output mengontrol apa yang kembali:

* `files_with_matches`: jalur file saja, tidak ada konten baris. Ini adalah default.
* `content`: baris yang cocok dengan file dan nomor baris. Ketika parameter `offset` tool menunjuk melampaui kecocokan terakhir untuk pola yang memiliki kecocokan, Grep mengembalikan `No entries at this offset`, sehingga Claude memperluas atau mengatur ulang offset alih-alih menyimpulkan pola tidak cocok.
* `count`: jumlah kecocokan per file, diikuti dengan total di semua file yang cocok. Total mencakup setiap kecocokan bahkan ketika parameter `head_limit` atau `offset` tool memotong entri per-file yang terdaftar. Sebelum v2.1.208, total hanya menjumlahkan entri yang terdaftar.

Claude dapat membatasi hasil berdasarkan file dengan parameter `glob`, seperti `**/*.tsx`, atau berdasarkan bahasa dengan parameter `type`, seperti `py` atau `rust`. Secara default, pola cocok dalam satu baris. Claude dapat mengatur `multiline: true` untuk cocok di seluruh batas baris.

Grep menghormati `.gitignore`, jadi file gitignored dilewati. Untuk mencari file gitignored, Claude meneruskan jalurnya secara langsung.

Claude Code memutuskan izin untuk panggilan Grep sebelum memeriksa apakah `path` pencarian ada. Masih menjalankan pemeriksaan izin baca untuk `path` yang hilang di luar [direktori kerja](/docs/id/permissions#working-directories), jadi prompt izin untuk path tidak berarti path ada.

<h2 id="lsp-tool-behavior">
  Perilaku LSP tool
</h2>

LSP tool memberikan Claude intelijen kode dari language server yang sedang berjalan. Setelah setiap pengeditan file, secara otomatis melaporkan kesalahan tipe dan peringatan sehingga Claude dapat memperbaiki masalah tanpa langkah build terpisah. Claude juga dapat memanggilnya secara langsung untuk menavigasi kode:

* Lompat ke definisi simbol
* Temukan semua referensi ke simbol
* Dapatkan informasi tipe pada posisi
* Daftar simbol dalam file
* Cari simbol berdasarkan nama di seluruh workspace
* Temukan implementasi antarmuka
* Lacak hierarki panggilan

Claude Code menjaga tool tetap tidak aktif sampai Anda menginstal [plugin intelijen kode](/docs/id/plugins/code-intelligence) untuk bahasa Anda. Dalam [sesi cloud](/docs/id/claude-code-on-the-web), Claude Code tidak memulai language server plugin, jadi LSP tool tetap tidak aktif di sana. Claude Code mengambil konfigurasi language server dari plugin, dan Anda menginstal binary server sendiri.

Claude Code mengembalikan hasil kesalahan untuk setiap panggilan LSP pada file yang language server-nya tidak dapat dimulai.

<h2 id="monitor-tool">
  Monitor tool
</h2>

Monitor tool memungkinkan Claude mengawasi sesuatu di latar belakang dan bereaksi ketika berubah, tanpa menghentikan percakapan. Minta Claude untuk:

* Tail file log dan tandai kesalahan saat muncul
* Poll PR atau CI job dan laporkan ketika statusnya berubah
* Pantau direktori untuk perubahan file
* Lacak output dari skrip yang sedang berjalan lama yang Anda tunjukkan
* Terhubung ke feed WebSocket dan laporkan setiap pesan saat tiba

Untuk sebagian besar watch, Claude menulis skrip kecil, menjalankannya di latar belakang, dan menerima setiap baris output saat tiba. Untuk server yang sudah mendorong peristiwa, Claude dapat membuka [WebSocket](#websocket-source) daripada menjalankan skrip.

Anda terus bekerja dalam sesi yang sama dan Claude menyela ketika peristiwa tiba.

Setiap watch yang Claude mulai memiliki tenggat waktu: 5 menit secara default, paling lama 30 menit, dan paling lama 10 menit dalam run [non-interaktif](/docs/id/headless) yang diberikan prompt tunggal dengan `-p`.

Pada tenggat waktu, watch berakhir. Claude mendapat satu pemberitahuan, sehingga dapat memulai watch lagi jika masih diperlukan.

Hentikan monitor dengan meminta Claude untuk membatalkannya atau dengan mengakhiri sesi. Ketika Anda menghentikan [subagent](/docs/id/sub-agents) yang memulai monitors, misalnya dari `/tasks`, monitors tersebut berhenti bersamanya.

Ketika Monitor menjalankan perintah, ia menggunakan [aturan izin yang sama seperti Bash](/docs/id/permissions#tool-specific-permission-rules), jadi pola `allow` dan `deny` yang Anda tetapkan untuk Bash berlaku di sini juga. Sementara [auto mode](/docs/id/permission-modes#eliminate-prompts-with-auto-mode) aktif, Claude Code menyisihkan aturan allow yang menyebutkan `Monitor` itu sendiri, bersama dengan [aturan allow luas lainnya yang dihapusnya](/docs/id/permission-modes#how-the-classifier-evaluates-actions), sehingga classifier meninjau perintah Monitor dengan cara yang sama seperti meninjau perintah Bash.

[Sumber WebSocket](#websocket-source) memiliki prompt persetujuan tersendiri, yang juga diputuskan oleh classifier dalam auto mode.

Tool ini tidak tersedia di Amazon Bedrock, Google Cloud's Agent Platform, atau Microsoft Foundry. Ini juga tidak tersedia ketika `DISABLE_TELEMETRY` atau `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC` diatur.

Plugin dapat mendeklarasikan monitors yang dimulai secara otomatis ketika plugin aktif, daripada meminta Claude untuk memulainya. Lihat [plugin monitors](/docs/id/plugins/components#monitors).

<h3 id="websocket-source">
  WebSocket source
</h3>

<Note>
  Sumber WebSocket memerlukan Claude Code v2.1.195 atau lebih baru.
</Note>

Ketika server sudah mendorong peristiwa melalui WebSocket, Claude dapat terhubung langsung ke dalamnya daripada menulis skrip polling. Setiap jenis aktivitas soket menjadi peristiwa atau mengakhiri watch:

* **Pesan teks**: masing-masing menjadi satu peristiwa, bahkan ketika pesan mencakup beberapa baris.
* **Pesan biner**: tidak dilewatkan. Claude menerima baris placeholder seperti `[binary frame, 512 bytes]` sebagai gantinya.
* **Pesan lebih besar dari 1 MiB**: watch berakhir, jadi berlangganan feed yang disaring jika ada.
* **Penutupan soket**: watch berakhir dan Claude menerima kode penutupan.

Watch WebSocket mengambil input `ws` sebagai pengganti `command`, dan satu panggilan Monitor tidak dapat menggabungkan keduanya. Input `ws` memiliki dua bidang:

| Field       | Required | Description                                                                                                                                                  |
| :---------- | :------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `url`       | Yes      | Endpoint untuk terhubung. Harus berupa URL `ws://` atau `wss://` tanpa kredensial tertanam atau spasi, hanya menggunakan karakter ASCII                      |
| `protocols` | No       | Nama subprotokol WebSocket untuk ditawarkan selama handshake. Setiap entri harus berupa token subprotokol yang valid, dan daftar tidak dapat berisi duplikat |

Tenggat waktu `timeout_ms` berlaku untuk watch WebSocket juga: watch berakhir pada tenggat waktu, dan `TaskStop` membatalkannya lebih awal.

Membuka WebSocket meminta persetujuan; dalam [auto mode](/docs/id/permission-modes#eliminate-prompts-with-auto-mode) classifier memutuskan sebagai gantinya. Prompt tidak menawarkan opsi untuk melewati prompt di masa depan untuk host yang sama.

Claude Code menolak URL yang menunjuk ke alamat pribadi, link-local, atau cloud-metadata, termasuk nama host yang menyelesaikan ke salah satu. Ini juga menolak host di `sandbox.network.deniedDomains`, dan ketika [`allowManagedDomainsOnly`](/docs/id/settings-reference#sandbox-network-allowmanageddomainsonly) diatur dalam pengaturan terkelola, host apa pun di luar daftar izin terkelola.

<h2 id="notebookedit-tool-behavior">
  Perilaku NotebookEdit tool
</h2>

NotebookEdit memodifikasi notebook Jupyter satu sel pada satu waktu, menargetkan sel berdasarkan `cell_id` mereka. Ini tidak melakukan penggantian string di seluruh notebook seperti yang dilakukan [Edit](#edit-tool-behavior) pada file biasa.

Tiga mode edit mengontrol apa yang terjadi pada sel target:

* `replace`: timpa sumber sel. Ini adalah default.
* `insert`: tambahkan sel baru setelah target. Tanpa `cell_id`, sel baru masuk di awal notebook. Memerlukan `cell_type` diatur ke `code` atau `markdown`.
* `delete`: hapus sel target.

Aturan izin menggunakan format path `Edit(...)`. Aturan seperti `Edit(notebooks/**)` mencakup panggilan NotebookEdit pada file dalam direktori itu.

<h2 id="powershell-tool">
  Alat PowerShell
</h2>

Alat PowerShell memungkinkan Claude menjalankan perintah PowerShell secara native. Di Windows, ini berarti perintah berjalan di PowerShell alih-alih melalui Git Bash. Bagaimana alat menjadi tersedia tergantung pada platform Anda:

* **Windows tanpa Git Bash**: alat diaktifkan secara otomatis.
* **Windows dengan Git Bash terinstal**: alat aktif secara default untuk akun claude.ai dan Console; atur `CLAUDE_CODE_USE_POWERSHELL_TOOL=1` untuk mengaktifkannya di sesi Amazon Bedrock, Google Cloud's Agent Platform, dan Microsoft Foundry, atau `0` untuk mematikannya.
* **Linux, macOS, dan WSL**: alat bersifat opt-in.

[PreToolUse hooks](/docs/id/hooks#powershell) Anda menerima string perintah alat dalam `tool_input.command`, dengan bidang yang sama seperti alat Bash.

Cocokkan `Bash|PowerShell` dalam hooks yang memeriksa perintah shell; [bagian input hook PowerShell](/docs/id/hooks#powershell) menjelaskan mengapa mencocokkan `Bash` saja tidak cukup.

<h3 id="enable-the-powershell-tool">
  Aktifkan alat PowerShell
</h3>

Atur `CLAUDE_CODE_USE_POWERSHELL_TOOL=1` di lingkungan Anda atau di `settings.json`:

```json theme={null}
{
  "env": {
    "CLAUDE_CODE_USE_POWERSHELL_TOOL": "1"
  }
}
```

Di Windows, atur variabel ke `0` untuk mematikan alat. Di Linux, macOS, dan WSL, alat memerlukan PowerShell 7 atau lebih baru: instal `pwsh` dan pastikan itu ada di `PATH` Anda.

Di Windows, Claude Code secara otomatis mendeteksi `pwsh.exe` untuk PowerShell 7+ dengan fallback ke `powershell.exe` untuk PowerShell 5.1. Ketika alat diaktifkan, Claude memperlakukan PowerShell sebagai shell utama. Alat Bash tetap tersedia untuk skrip POSIX ketika Git Bash terinstal.

Claude Code menjalankan PowerShell dengan `-ExecutionPolicy Bypass` pada cakupan proses saja, sehingga skrip `.ps1` dan impor modul bekerja pada instalasi Windows default tanpa mengubah kebijakan mesin. Bypass cakupan proses tidak mengganti Group Policy `MachinePolicy` atau `UserPolicy`, sehingga kebijakan perusahaan tetap berlaku. Untuk menghormati kebijakan eksekusi efektif mesin sebagai gantinya, atur `CLAUDE_CODE_POWERSHELL_RESPECT_EXECUTION_POLICY=1`.

<h3 id="shell-selection-in-settings-hooks-and-skills">
  Pemilihan shell dalam pengaturan, hooks, dan skills
</h3>

Tiga pengaturan tambahan mengontrol di mana PowerShell digunakan:

* `"defaultShell": "powershell"` dalam [`settings.json`](/docs/id/settings-reference#all-settings): merutekan perintah `!` interaktif melalui PowerShell. Memerlukan alat PowerShell untuk diaktifkan.
* `"shell": "powershell"` pada [command hooks](/docs/id/hooks#command-hook-fields) individual: menjalankan hook tersebut di PowerShell. Hooks menjalankan PowerShell secara langsung, sehingga ini bekerja terlepas dari `CLAUDE_CODE_USE_POWERSHELL_TOOL`.
* `shell: powershell` dalam [skill frontmatter](/docs/id/skills#frontmatter-reference): menjalankan blok `` !`command` `` di PowerShell. Memerlukan alat PowerShell untuk diaktifkan.

Perilaku reset direktori kerja sesi utama yang sama yang dijelaskan di bagian alat Bash berlaku untuk perintah PowerShell, termasuk variabel lingkungan `CLAUDE_BASH_MAINTAIN_PROJECT_WORKING_DIR`.

Mulai dari v2.1.196, kode keluar 1 dari `grep`, `rg`, `egrep`, `fgrep`, `findstr`, dan `git grep` berarti tidak ada kecocokan. Kode keluar 1 dari `git diff` berarti perbedaan ada. Tidak ada hasil yang dilaporkan ke Claude sebagai kegagalan perintah. Untuk `robocopy`, kode keluar 0 hingga 7 adalah hasil informatif, seperti file yang disalin atau file ekstra yang terdeteksi. Kode keluar 8 atau lebih tinggi dihitung sebagai kegagalan.

<h3 id="windows-encoding-and-exit-codes">
  Pengkodean Windows dan kode keluar
</h3>

Di Windows, perilaku pengkodean PowerShell dan kode keluar berikut memerlukan Claude Code v2.1.214 atau lebih baru:

* Pengalihan dengan `>` dan `>>` menulis file UTF-8 di PowerShell 5.1
* Claude Code mengkodekan teks yang disalurkan ke input standar perintah native sebagai UTF-8
* Claude Code menangkap output kesalahan tanpa urutan pelarian ANSI
* Perintah yang proses anak-anaknya menunggu input standar menerima end-of-file alih-alih hang
* Kode keluar 1 dari `where.exe` berarti tidak ada kecocokan, dan dari `fc.exe` dan `diff.exe` berarti file berbeda, jadi ketika perintah menghasilkan output, Claude Code memperlakukan kode keluar tersebut sebagai jawaban negatif yang valid daripada kesalahan perintah. Claude Code masih melaporkan bentuk yang disenyapkan, seperti `where.exe /Q` atau pengalihan ke `$null`, sebagai kegagalan pada kode keluar 1

Sebelum v2.1.214, `>` di PowerShell 5.1 menulis file UTF-16LE, input yang disalurkan non-ASCII tiba sebagai `?`, dan skrip Python bisa crash dengan `UnicodeEncodeError` saat mencetak karakter non-ASCII.

<h3 id="preview-limitations">
  Keterbatasan pratinjau
</h3>

Alat PowerShell memiliki keterbatasan yang diketahui berikut selama pratinjau:

* Profil PowerShell tidak dimuat
* Di Windows, sandboxing tidak didukung

<h2 id="read-tool-behavior">
  Perilaku alat Read
</h2>

Alat Read mengambil jalur file dan mengembalikan isinya dengan nomor baris. Claude diinstruksikan untuk selalu melewatkan jalur absolut.

Secara default, Read mengembalikan file dari awal. Ketika pembacaan seluruh file melebihi batas token, Read mengembalikan halaman pertama dengan pemberitahuan `PARTIAL view` yang memberi tahu Claude berapa banyak file yang diterima dan cara membaca lebih lanjut dengan `offset` dan `limit`. Pembacaan yang melewatkan `offset` atau `limit` eksplisit dan masih melebihi batas token mengembalikan kesalahan.

Pembacaan dengan `limit` eksplisit berhenti segera setelah baris yang dipilih melebihi apa yang dapat ditampung batas token dan mengembalikan kesalahan tanpa memuat sisa rentang. Kesalahan memberi tahu Claude untuk menggunakan `limit` yang lebih kecil, atau untuk mencari konten spesifik dengan [Grep](#grep-tool-behavior) sebagai gantinya ketika satu baris sangat besar. Sebelum v2.1.208, Claude Code memuat seluruh rentang ke dalam memori sebelum menolaknya, jadi membaca file dengan satu baris yang sangat panjang dapat menghabiskan memorinya.

Membaca file kosong mengembalikan pemberitahuan bahwa file ada tetapi isinya kosong, dan `offset` melampaui baris terakhir mengembalikan pemberitahuan yang memberikan jumlah baris file. Sebelum v2.1.208, membaca file kosong mengembalikan pemberitahuan past-the-end sebagai gantinya.

Read menangani beberapa jenis file di luar teks biasa:

* **Gambar**: PNG, JPG, dan format gambar lainnya dikembalikan sebagai konten visual yang dapat dilihat Claude, bukan sebagai byte mentah. Claude Code mengubah ukuran dan mengompresi ulang gambar besar agar sesuai dengan batas ukuran gambar model sebelum mengirimkannya, jadi Claude mungkin melihat versi yang diperkecil dari tangkapan layar besar. Mulai dari v2.1.196, gambar yang masih lebih besar dari 500KB setelah pengubahan ukuran tersebut dikodekan ulang sebagai JPEG dengan kualitas berkurang dengan dimensi pikselnya tidak berubah. Jika Claude melewatkan detail tingkat piksel halus dalam gambar besar, minta untuk memotong wilayah yang diminati terlebih dahulu, misalnya dengan ImageMagick melalui Bash.
* **PDF**: Claude membaca file `.pdf` pendek secara keseluruhan. Untuk PDF yang lebih panjang dari 10 halaman, file tersebut dibaca dalam rentang dengan parameter `pages`, seperti `"1-5"`, hingga 20 halaman sekaligus.
* **Jupyter notebooks**: File `.ipynb` mengembalikan semua sel dengan keluarannya, termasuk kode, markdown, dan visualisasi. Claude Code menolak untuk membaca file notebook di atas 100 MB; kesalahan memberi tahu Claude cara membaca sebagian dari notebook sebagai gantinya, seperti irisan sel, dengan perintah shell.

Read hanya membaca file, bukan direktori. Claude mencantumkan isi direktori dengan perintah shell seperti `ls`.

<h2 id="sendfeedback-tool-behavior">
  Perilaku alat SendFeedback
</h2>

Umpan balik yang disusun Claude adalah laporan umpan balik tentang Claude Code yang ditulis Claude untuk Anda. Ini memerlukan Claude Code v2.1.238 atau lebih baru. Claude Code menyimpan setiap draf di mesin Anda di bawah `~/.claude/feedback/drafts/`, dan tidak ada yang sampai ke Anthropic sampai Anda mengirimnya. Claude menyusun satu dengan alat SendFeedback ketika:

* Alat atau perintah terus gagal
* Tidak dapat membantu dengan sesuatu yang Anda minta
* Anda menunjukkan kesalahan yang dibuat, atau itu menyadarinya
* Anda memintanya untuk mengajukan umpan balik

<h3 id="what-you-see-when-claude-drafts">
  Apa yang Anda lihat ketika Claude menyusun
</h3>

Setelah Claude mengantrekan draf, Anda melihat kartu di atas prompt Anda dengan judul draf. Tekan `1` untuk meninjau draf, tekan `2` dua kali untuk mengirimnya seperti yang ditulis, atau tekan `0` untuk menutupnya. Draf yang ditutup tetap berada dalam antrian Anda. Setelah Anda menutup kartu, Claude Code menanyakan apakah akan mematikan umpan balik yang disusun Claude. Itu berhenti bertanya setelah Anda menolak dua kali.

Secara default, Anda melihat paling banyak tiga kartu dalam sesi; Anthropic dapat menyesuaikan batas itu dari server tanpa rilis. Setelah batas, dan kapan pun Anda menetapkan [`feedbackDrafts`](/docs/id/settings-reference#feedbackdrafts) ke `quiet`, Anda hanya melihat hitungan draf yang antri di footer prompt.

<h3 id="review-and-edit-a-draft">
  Tinjau dan edit draf
</h3>

Jalankan `/feedback` tanpa argumen untuk membuka antrian Anda. Ini mencantumkan setiap draf yang antri dari semua sesi Anda, termasuk draf yang kartunya Anda tutup atau tidak pernah lihat. Pilih draf untuk membukanya untuk ditinjau, di mana Anda dapat:

* Edit judul, area, dan detail
* Atur **Send transcript** ke `yes` atau `no`. Ketika transkrip dari sesi tempat Claude mengantrekan draf masih tersedia, itu dimulai pada `yes`, yang mengirim percakapan itu ke Anthropic; `no` mengirim laporan saja
* Kirim draf, buang, atau biarkan di antrian untuk nanti

Untuk menulis laporan sendiri, tekan `w` untuk dialog umpan balik standar. `/feedback` dengan teks setelahnya, dan `/bug`, buka dialog itu secara langsung.

<h3 id="send-a-draft">
  Kirim draf
</h3>

Ketika Anda mengirim draf, Claude Code mengirimkannya dengan cara yang sama seperti laporan `/feedback`, dengan [retensi](/docs/id/data-usage#feedback-using-the-%2Ffeedback-command) yang sama, dan menghapus draf dari mesin Anda. Ketika Anda mengirim dari kartu, itu menunjukkan `✓ Sent`; ketika Anda mengirim dari antrian, itu ditutup dengan ID tanda terima.

Laporan membawa:

* Judul, area, dan detail Anda
* Info lingkungan, seperti versi Claude Code Anda, sistem operasi, dan model
* ID permintaan API terbaru
* Transkrip percakapan, ketika Anda meninggalkan **Send transcript** pada `yes` di layar tinjauan. Mengirim dari kartu tidak pernah menyertakan transkrip

Claude Code menyimpan direktori kerja Anda dalam draf lokal sehingga dapat menemukan transkrip, dan tidak mengirim direktori.

Dalam [organisasi dengan retensi data nol](/docs/id/zero-data-retention#features-disabled-under-zdr), Claude Code meninggalkan alat, seperti halnya untuk `/feedback`. Jika sesi dalam organisasi seperti itu masih menawarkan alat, draf tetap di mesin Anda, dan pengiriman gagal dengan `Feedback collection is not available for organizations with custom data retention policies.`

<h3 id="discard-or-keep-a-draft">
  Buang atau simpan draf
</h3>

Ketika Anda membuang draf, Claude Code menghapusnya dari mesin Anda. Draf yang Anda tinggalkan di antrian kedaluwarsa setelah 30 hari, atau setelah [`cleanupPeriodDays`](/docs/id/settings-reference#cleanupperioddays) ketika itu lebih pendek. Antrian menyimpan 10 draf di semua sesi Anda, dan ketika Claude mengantrekan yang kesebelas, Claude Code menghapus yang tertua. Ketika Anda menjalankan `/exit` dengan draf dari sesi masih di antrian, Claude Code menanyakan apakah akan meninjau atau membuang sebelum keluar.

<h3 id="turn-claude-drafted-feedback-off">
  Matikan umpan balik yang disusun Claude
</h3>

Atur **Claude-drafted feedback** ke `off` di `/config`, yang menulis pengaturan [`feedbackDrafts`](/docs/id/settings-reference#feedbackdrafts), atau atur [`CLAUDE_CODE_SEND_FEEDBACK=0`](/docs/id/env-vars) untuk satu sesi. Dengan salah satu, Claude tidak dapat mengantrekan draf. Untuk terus menyusun tanpa kartu, atur `feedbackDrafts` ke `quiet` sebagai gantinya. Administrator dapat mengatur `feedbackDrafts` dalam [pengaturan terkelola](/docs/id/managed-settings), yang mengambil alih pengaturan Anda sendiri.

<h3 id="sessions-without-claude-drafted-feedback">
  Sesi tanpa umpan balik yang disusun Claude
</h3>

Claude Code menyertakan alat dalam sesi terminal interaktif di mesin Anda sendiri yang menggunakan Claude API daripada penyedia cloud. Itu meninggalkan alat dari:

* Berjalan `-p` non-interaktif dan sesi [Agent SDK](/docs/id/agent-sdk/overview), yang tidak memiliki layar untuk meninjau antrian
* Sesi cloud seperti [Claude Code di web](/docs/id/claude-code-on-the-web), yang tidak dapat menulis ke antrian di mesin Anda
* Sesi di [Amazon Bedrock](/docs/id/amazon-bedrock), [Claude Platform on AWS](/docs/id/claude-platform-on-aws), [Google Cloud's Agent Platform](/docs/id/google-vertex-ai), atau [Microsoft Foundry](/docs/id/microsoft-foundry)
* Sesi di mana Anda menetapkan [`CLAUDE_CODE_SEND_FEEDBACK=0`](/docs/id/env-vars) atau [`DISABLE_FEEDBACK_COMMAND=1`](/docs/id/env-vars), atur `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC` ke nilai non-kosong apa pun, atau matikan [pengambilan bendera fitur](/docs/id/env-vars#features-that-need-feature-flag-fetching)
* Organisasi yang telah mematikan umpan balik produk, dan [organisasi dengan retensi data nol](/docs/id/zero-data-retention#features-disabled-under-zdr)

<h2 id="task-tool-availability">
  Ketersediaan alat Task
</h2>

Alat-alat pelacakan tugas, `TaskCreate`, `TaskGet`, `TaskUpdate`, `TaskList`, dan `TodoWrite`, tersedia secara default hanya pada model Claude 3.x, Opus 4 hingga 4.7, Sonnet 4 hingga 4.6, dan Haiku 4.5. Di mana pun alat-alat tersebut tersedia, Anda mendapatkan empat alat Task, atau `TodoWrite` saja ketika Anda menetapkan [`CLAUDE_CODE_ENABLE_TASKS=0`](/docs/id/env-vars).

Pada setiap model lainnya, Claude Code menghilangkan alat-alat tersebut kecuali Anda memilih untuk menggunakannya. Hal yang sama berlaku untuk ID model yang Claude Code tidak kenali, seperti nama model khusus yang disajikan melalui [gateway LLM](/docs/id/llm-gateway). Pada model yang lebih baru, Claude melacak pekerjaan multi-langkah tanpa daftar periksa tertulis, dan definisi alat serta pengingat menggunakan konteks. Tanpa alat-alat tersebut, Claude tidak menambahkan apa pun ke [daftar tugas](/docs/id/interactive-mode#task-list) saat bekerja.

Jika Anda ingin menggunakan alat-alat ini pada model yang tidak memilikinya secara default, lakukan salah satu hal berikut:

* Ekspor [`CLAUDE_CODE_ENABLE_TODO_TOOLS=1`](/docs/id/env-vars) sebelum Anda memulai Claude Code, misalnya `CLAUDE_CODE_ENABLE_TODO_TOOLS=1 claude`. Claude Code kemudian menyediakan alat yang sama pada setiap model dan setiap penyedia
* Sebutkan salah satu alat dalam [`--allowedTools`](/docs/id/cli-reference#cli-flags), misalnya `claude --allowedTools TaskCreate`
* Daftarkan alat-alat dalam [`--tools`](/docs/id/cli-reference#cli-flags), yang membatasi alat bawaan sesi ke yang disebutkannya. Sertakan alat yang Anda inginkan bersama dengan alat bawaan lain yang Anda gunakan
* Dalam Agent SDK, opsi [`allowedTools` dan `tools`](/docs/id/agent-sdk/todo-tracking#model-availability) bekerja dengan cara yang sama seperti dua flag

Dalam [sesi latar belakang](/docs/id/agent-view) dan dalam [sesi cloud](/docs/id/claude-code-on-the-web), Claude Code menyediakan alat yang sama pada setiap model, terdaftar atau tidak.

Claude Code memberikan alat kepada subagent hanya ketika sesi Anda memilikinya, bahkan ketika subagent menjalankan model yang berbeda. Anggota tim [tim agen](/docs/id/agent-teams) dalam proses mengikuti sesi Anda dengan cara yang sama, sementara anggota tim dalam [panel terpisahnya sendiri](/docs/id/agent-teams#choose-a-display-mode) berjalan sebagai proses Claude Code terpisah, jadi modelnya sendiri yang memutuskan. Tanpa alat Task, agen berkoordinasi dengan timnya melalui pesan alih-alih [daftar tugas bersama](/docs/id/agent-teams#assign-and-claim-tasks).

Rangkaian default yang dijelaskan di sini berlaku dalam Claude Code v2.1.268 dan yang lebih baru.

<h2 id="webfetch-tool-behavior">
  Perilaku alat WebFetch
</h2>

WebFetch mengambil URL dan prompt yang menjelaskan apa yang akan diekstrak. Alat ini mengambil halaman, mengonversi respons ke Markdown ketika server mengembalikan HTML, dan menjalankan prompt terhadap konten menggunakan model yang kecil dan cepat. Untuk sebagian besar pengambilan, Claude menerima jawaban model tersebut, bukan halaman mentah. Langkah konversi tidak dapat dikonfigurasi.

Ini membuat WebFetch lossy secara desain. Prompt ekstraksi menentukan apa yang sampai ke Claude, jadi hasil yang mengatakan halaman tidak menyebutkan sesuatu mungkin hanya berarti prompt tidak menanyakannya. Minta Claude untuk mengambil lagi dengan prompt yang lebih spesifik, atau gunakan `curl` melalui Bash untuk halaman yang belum diproses.

Beberapa perilaku membentuk respons yang diterima Claude:

* WebFetch menolak `localhost` dan nama host lainnya tanpa titik, seperti nama intranet biasa, sebelum membuat permintaan. [Kesalahan yang dikembalikannya](/docs/id/errors#webfetch-cannot-fetch-localhost) memberitahu Claude untuk menjangkau server lokal dengan `curl` melalui Bash sebagai gantinya.
* URL HTTP secara otomatis ditingkatkan ke HTTPS.
* Halaman besar dipotong ke batas karakter tetap sebelum diproses.
* WebFetch menyimpan cache setiap respons selama 15 menit secara default, jadi pengambilan berulang dari URL yang sama kembali dengan cepat. Pada Claude Code v2.1.233 atau lebih baru, atur [`CLAUDE_CODE_WEBFETCH_CACHE_TTL_MS`](/docs/id/env-vars#variables) untuk mengubah berapa lama WebFetch menyimpan setiap respons.
* Halaman yang belum selesai diunduh dalam lima menit, termasuk pengalihan apa pun yang diikuti WebFetch, gagal dengan kesalahan batas waktu. Pada Claude Code v2.1.268 atau lebih baru, atur [`CLAUDE_CODE_WEBFETCH_DEADLINE_MS`](/docs/id/env-vars#variables) untuk mengubah batas, atau ke `0` untuk menghapusnya.
* Ketika URL dialihkan ke host yang berbeda, WebFetch mengembalikan hasil teks yang menyebutkan URL asli dan target pengalihan alih-alih mengikutinya. Claude kemudian mengambil URL baru dengan panggilan WebFetch kedua.
* Ketika langkah ekstraksi mengalami API yang kelebihan beban, Claude Code mencoba ulang dengan backoff; pengambilan yang masih gagal mengembalikan hasil kesalahan. Sebelum v2.1.212, teks kesalahan API dapat sampai ke Claude seolah-olah itu adalah konten halaman yang diekstrak.

Dalam mode Manual dan `acceptEdits` [permission modes](/docs/id/permission-modes), WebFetch meminta sebelum mengambil, kecuali untuk domain yang [permission rules](/docs/id/permissions#manage-permissions) Anda sudah izinkan atau tolak dan serangkaian domain dokumentasi pra-persetujuan bawaan yang mengambil tanpa prompt. Apa pun yang aturan Anda izinkan, pengambilan juga melewati [WebFetch domain safety check](/docs/id/data-usage#webfetch-domain-safety-check) terlebih dahulu; bagian itu mencakup apa yang dikirim pemeriksaan dan pengaturan yang melewatinya. Prompt menawarkan tiga opsi:

* **Ya**: menyetujui pengambilan ini saja. Panggilan WebFetch berikutnya meminta lagi, bahkan untuk domain yang sama.
* **Ya, dan jangan tanya lagi untuk `<domain>`**: menyetujui pengambilan dan menyimpan aturan izin `WebFetch(domain:...)` untuk domain itu ke `.claude/settings.local.json` untuk repositori itu. Lihat [bagaimana persetujuan yang disimpan bertahan](/docs/id/permissions#permission-system). Ketika organisasi Anda menetapkan [`allowManagedPermissionRulesOnly`](/docs/id/permissions#managed-only-settings), Claude Code menyembunyikan opsi ini.
* **Tidak, dan beri tahu Claude apa yang harus dilakukan berbeda**: menolak pengambilan.

Untuk mengizinkan domain sebelumnya tanpa prompt, tambahkan aturan izin seperti `WebFetch(domain:example.com)`; `WebFetch(domain:*)` mengizinkan setiap domain. Mode `auto` dan `bypassPermissions` [permission modes](/docs/id/permissions#permission-modes) melewati prompt, kecuali untuk domain yang cocok dengan aturan `ask` eksplisit.

Aturan `WebFetch(domain:...)` eksplisit dalam `deny`, `ask`, atau `allow` mengambil alih himpunan pra-persetujuan, jadi Anda dapat memblokir domain pra-persetujuan atau memerlukan prompt untuknya.

WebFetch menetapkan header `User-Agent` yang dimulai dengan `Claude-User`, dan header `Accept` yang lebih menyukai Markdown daripada HTML sehingga server yang mendukung negosiasi konten dapat mengembalikan Markdown secara langsung.

Perintah sandboxed tidak mewarisi himpunan domain dokumentasi pra-persetujuan bawaan WebFetch. Untuk membiarkan perintah sandboxed mencapai domain tanpa prompt, tambahkan domain ke [`allowedDomains`](/docs/id/settings-reference#sandbox-network-alloweddomains) atau izinkan dengan aturan `WebFetch(domain:...)`, yang [sandbox juga hormati](/docs/id/sandboxing#network-isolation). WebFetch tidak pernah membaca daftar allowlist sandbox sebagai gantinya, jadi menambahkan domain ke sandbox atau daftar allowlist jaringan organisasi tidak menghentikan WebFetch dari memintanya.

<h2 id="websearch-tool-behavior">
  Perilaku alat WebSearch
</h2>

WebSearch menjalankan kueri terhadap backend [web search](https://platform.claude.com/docs/en/agents-and-tools/tool-use/web-search-tool) Anthropic dan mengembalikan judul hasil dan URL. Alat ini tidak mengambil halaman hasil. Untuk membaca halaman yang Claude temukan dalam hasil pencarian, alat ini diikuti dengan [WebFetch](#webfetch-tool-behavior).

Alat ini dapat mengeluarkan hingga delapan pencarian backend per panggilan, menyempurnakan pencarian secara internal sebelum mengembalikan hasil. Claude dapat membatasi hasil dengan `allowed_domains` untuk menyertakan hanya host tertentu, atau `blocked_domains` untuk mengecualikannya. Kedua daftar tidak dapat digabungkan dalam satu panggilan.

Ketika permintaan pencarian mengenai API yang kelebihan beban, Claude Code mencoba ulang dengan backoff; panggilan yang masih gagal mengembalikan hasil kesalahan. Sebelum v2.1.212, teks kesalahan API dapat mencapai Claude seolah-olah itu adalah hasil pencarian.

Aturan izin WebSearch tidak memerlukan spesifier. Entri `WebSearch` kosong dalam `allow` atau `deny` adalah satu-satunya bentuk.

Backend pencarian tidak dapat dikonfigurasi. Untuk mencari dengan penyedia yang berbeda, tambahkan [server MCP](/docs/id/mcp) yang mengekspos alat pencarian.

<Note>
  WebSearch tersedia di Claude API dan [Claude Platform on AWS](/docs/id/claude-platform-on-aws). Di Microsoft Foundry, alat ini memerlukan [deployment yang dihosting di Anthropic](https://platform.claude.com/docs/en/build-with-claude/claude-in-microsoft-foundry#hosting-options): deployment yang dihosting di Azure tidak mendukung alat sisi server, jadi panggilan WebSearch gagal. Di Agent Platform Google Cloud, alat ini bekerja dengan Claude 4 dan model yang lebih baru, termasuk Opus, Sonnet, dan Haiku. Amazon Bedrock tidak mengekspos alat web search sisi server.
</Note>

<h3 id="session-search-limit">
  Batas pencarian sesi
</h3>

Sesi dapat melakukan paling banyak 200 panggilan WebSearch, dihitung di seluruh percakapan utama dan setiap [subagent](/docs/id/sub-agents) yang dihasilkannya, jadi pencarian yang dilakukan oleh fan-out penelitian paralel dihitung terhadap batas yang sama. Batas ini memerlukan Claude Code v2.1.212 atau lebih baru. Ketika Claude mencapai batas, panggilan lebih lanjut mengembalikan pemberitahuan yang memberi tahu Claude untuk melanjutkan dengan informasi yang telah dikumpulkannya, bukan kesalahan yang akan mengundang percobaan ulang. Anda tidak melihat pemberitahuan: panggilan yang dibatasi muncul dalam percakapan sebagai pencarian yang tidak melakukan apa pun, dan jika Claude memerlukan lebih banyak pencarian, pemberitahuan memberi tahu Claude untuk meminta Anda menaikkan batas.

Atur variabel lingkungan [`CLAUDE_CODE_MAX_WEB_SEARCHES_PER_SESSION`](/docs/id/env-vars) untuk mengubah batas; variabel ini menerima angka bulat positif, jadi batas dapat dinaikkan tetapi tidak dapat dimatikan. Menjalankan [`/clear`](/docs/id/commands#all-commands) mengatur ulang hitungan. Jika pekerjaan yang masih dapat menghasilkan [subagents](/docs/id/sub-agents) bertahan setelah pembersihan, seperti alur kerja yang sedang berjalan, hitungan akan dibawa ke depan.

<h2 id="write-tool-behavior">
  Perilaku tool Write
</h2>

Tool Write membuat file baru atau menimpa file yang sudah ada dengan konten lengkap yang disediakan. Tool ini tidak menambahkan atau menggabungkan.

Apakah Claude harus membaca file yang sudah ada dalam percakapan saat ini sebelum menimpanya tergantung pada model dan file:

* Claude Opus 4.6, Claude Haiku 4.5, dan model yang lebih lama selalu memerlukan pembacaan, jadi Write ke file yang sudah ada yang belum dibaca gagal dengan kesalahan.
* Model yang lebih baru dapat menimpa file yang tidak pernah mereka baca dalam sesi ini di bawah kondisi yang sama seperti [read-before-edit](#edit-tool-behavior): membacanya tidak akan memerlukan prompt izin dan tool Read tersedia.
* Jupyter notebooks, dan file yang Claude baca hanya sebagian dengan pemberitahuan [`PARTIAL view`](#read-tool-behavior), memerlukan pembacaan pada setiap model.

Batasan ini tidak berlaku untuk file baru. Sebelum v2.1.228, setiap model memerlukan pembacaan sebelum menimpa file yang sudah ada.

Melihat file dengan Bash juga memenuhi persyaratan ini di bawah aturan yang sama yang dijelaskan dalam [Edit tool behavior](#edit-tool-behavior).

Untuk perubahan sebagian pada file yang sudah ada, Claude menggunakan Edit alih-alih Write.

<h2 id="check-which-tools-are-available">
  Periksa tools mana yang tersedia
</h2>

Set tools yang tepat bergantung pada penyedia, platform, dan pengaturan Anda. Untuk memeriksa apa yang dimuat dalam sesi yang sedang berjalan, tanyakan Claude secara langsung:

```text theme={null}
What tools do you have access to?
```

Claude memberikan ringkasan percakapan. Untuk nama tool MCP yang tepat, jalankan `/mcp`.

<Note>
  [advisor tool](/docs/id/advisor) adalah [server tool](https://platform.claude.com/docs/en/agents-and-tools/tool-use/advisor-tool) yang dijalankan API, bukan tool yang diimplementasikan Claude Code. Tool ini tidak memiliki nama yang dapat Anda referensikan dalam aturan izin atau pencocokan hook.
</Note>

<h2 id="see-also">
  Lihat juga
</h2>

* [MCP servers](/docs/id/mcp): tambahkan tools kustom dengan menghubungkan server eksternal
* [Permissions](/docs/id/permissions): sistem izin, sintaks aturan, dan pola khusus tool
* [Subagents](/docs/id/sub-agents): konfigurasi akses tool untuk subagent
* [Hooks](/docs/id/hooks-guide): jalankan perintah kustom sebelum atau sesudah eksekusi tool
