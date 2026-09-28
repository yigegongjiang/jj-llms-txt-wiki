> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Kelola sesi

> Beri nama, lanjutkan, cabang, dan beralih antar percakapan Claude Code. Mencakup `--continue`, `--resume`, `--from-pr`, pemilih `/resume`, penamaan sesi, ekspor transkrip, dan tempat penyimpanan transkrip.

Sesi adalah percakapan yang disimpan yang terikat pada direktori proyek. Claude Code menyimpannya secara lokal saat Anda bekerja, sehingga Anda dapat melanjutkan dari tempat Anda berhenti, membuat cabang untuk mencoba pendekatan berbeda, atau beralih antar tugas.

[Aplikasi desktop](/docs/id/desktop#work-in-parallel-with-sessions), [Claude Code di web](/docs/id/claude-code-on-the-web), dan [ekstensi VS Code](/docs/id/vs-code#resume-past-conversations) masing-masing mempertahankan riwayat sesi mereka sendiri. Halaman ini mencakup CLI.

<h2 id="resume-a-session">
  Lanjutkan sesi
</h2>

Sesi disimpan secara berkelanjutan ke [file transkrip lokal](#export-and-locate-session-data) saat Anda bekerja, sehingga Anda dapat kembali ke satu setelah keluar atau menjalankan `/clear`. Gunakan titik masuk ini:

| Perintah                            | Apa yang dilakukannya                                                                                                         |
| :---------------------------------- | :---------------------------------------------------------------------------------------------------------------------------- |
| `claude --continue`                 | Membuka kembali percakapan terbaru di direktori saat ini                                                                      |
| `claude --resume`                   | Membuka [pemilih sesi](#use-the-session-picker)                                                                               |
| `claude --resume <name>`            | Melanjutkan sesi bernama secara langsung                                                                                      |
| `claude --resume <transcript-path>` | Melanjutkan percakapan yang disimpan dalam file [transkrip](#where-transcripts-are-stored) `.jsonl` di jalur absolut tersebut |
| `claude --from-pr <number>`         | Membuka pemilih sesi yang disaring untuk sesi yang ditautkan ke permintaan tarik tersebut                                     |
| `/resume`                           | Beralih ke percakapan berbeda dari dalam sesi aktif                                                                           |

Claude Code meninggalkan sesi yang dibuat dengan [`claude -p`](/docs/id/headless) atau [Agent SDK](/docs/id/agent-sdk/overview) keluar dari pemilih sesi dan keluar dari `claude --continue`. Anda masih dapat melanjutkan satu dengan meneruskan ID sesinya ke `claude --resume <session-id>`. Dengan `claude --continue`, Claude Code juga melewati [sesi yang prompt pertamanya adalah `/loop`](#where-the-session-picker-looks). Ketika Anda menjalankan [`claude -p --continue`](/docs/id/headless#continue-conversations), Claude Code menyertakan sesi `-p`, SDK, dan `/loop`.

`claude --continue` membuka [sesi latar belakang](/docs/id/agent-view) yang telah selesai, tetapi bukan yang masih berjalan; membuka sesi latar belakang yang selesai memerlukan Claude Code v2.1.257 atau lebih baru. Jika percakapan terbaru Anda adalah sesi yang [Anda pindahkan ke latar belakang](/docs/id/agent-view#send-the-session-to-the-background) dan masih berjalan di sana, Claude Code keluar dengan `Your most recent conversation is running in the background` dan ID sesi tersebut. Lampirkan ke sesi dari [`claude agents`](/docs/id/agent-view#attach-to-a-session), atau jalankan `claude --resume` untuk memilih yang lain.

Anda dapat menjalankan `claude --resume <session-id>` dari direktori mana pun: Claude Code mencari ID di direktori proyek saat ini dan git worktrees-nya terlebih dahulu, kemudian di setiap proyek lain di mesin ini, sehingga menemukan sesi yang dimulai di tempat lain atau dipindahkan dengan [`/cd`](/docs/id/commands). Pencarian lintas proyek menyelesaikan ID hanya ketika tepat satu proyek lain menyimpan transkrip dengan pesan untuknya, sehingga duplikat yang disalin tangan membuat Claude Code melaporkan tidak ditemukan daripada melanjutkan salinan arbitrer. Jika tidak ada sesi yang disimpan cocok dengan ID, Claude Code melaporkan `No conversation found with session ID: <session-id>`. Sebelum v2.1.223, pencarian berhenti di direktori proyek saat ini dan git worktrees-nya, jadi Anda harus melanjutkan dari direktori tempat sesi terakhir bekerja.

<h3 id="what-a-resumed-session-restores">
  Apa yang dipulihkan sesi yang dilanjutkan
</h3>

Sesi yang dilanjutkan memulihkan percakapan bersama dengan status yang disimpan di dalamnya:

* Riwayat percakapan: riwayat lengkap, termasuk panggilan alat dan hasil. Alat yang masih berjalan ketika proses sebelumnya berakhir, misalnya dalam kerusakan, tidak selesai atau berjalan lagi ketika Anda melanjutkan. Claude melihat panggilan yang ditandai sebagai terputus sebelum hasilnya dicatat dan diberitahu untuk memeriksa apakah itu berlaku sebelum menjalankannya lagi, kecuali [`CLAUDE_CODE_RESUME_INTERRUPTED_TURN`](/docs/id/env-vars#variables) diatur. Sebelum v2.1.281, Claude Code menghapus panggilan yang terputus dari percakapan atau menampilkannya kepada Claude sebagai yang Anda interupsi.
* Model: sesi berlanjut pada model yang digunakannya. Model tidak dipulihkan ketika telah pensiun atau tidak diizinkan oleh `availableModels`, ketika bendera `--model` atau variabel lingkungan keluarga `ANTHROPIC_MODEL` memilih satu saat peluncuran, atau pada penyedia yang menggunakan ID penerapan khusus penyedia, seperti [Amazon Bedrock, Google Cloud's Agent Platform, dan Microsoft Foundry](/docs/id/third-party-integrations); lihat [konfigurasi model](/docs/id/model-config#setting-your-model) untuk urutan resolusi.
* Agent: sesi yang dimulai dengan [`--agent`](/docs/id/sub-agents#invoke-subagents-explicitly) atau pengaturan `agent` berlanjut sebagai agen tersebut, mempertahankan pembatasan alat dan model-nya. Teruskan `--agent` saat melanjutkan untuk memilih yang berbeda; untuk prompt sistem dalam kedua kasus, lihat [Bendera prompt sistem dalam percakapan yang dilanjutkan](/docs/id/cli-reference#system-prompt-flags-in-resumed-conversations). Claude Code mencari agen di dua tempat: direktori asli sesi, asalkan Anda telah [mempercayai ruang kerja tersebut](/docs/id/permissions#project-allow-rules-and-workspace-trust), dan kemudian direktori tempat Anda melanjutkan, sehingga agen yang bersifat proyek masih dimuat ketika Anda melanjutkan dari direktori lain. Jika Claude Code tidak menemukan agen di salah satu tempat, sesi dilanjutkan dengan alat default dan menampilkan [peringatan yang menyebutkan agen](/docs/id/errors#session-agent-no-longer-available).
* Mode izin: jika Anda melanjutkan dari terminal dengan `claude --continue`, `claude --resume <session-id>`, atau `claude --resume <name>` ketika nama cocok dengan satu sesi, tanpa `-p`, Claude Code memulihkan mode izin yang ada di sesi, kecuali dalam kasus di [mode izin saat melanjutkan](#permission-mode-on-resume), yang juga mencakup pemilih sesi, `/resume`, dan melanjutkan dengan `claude -p`. Teruskan `--permission-mode` atau `--dangerously-skip-permissions` untuk mengganti mode yang dipulihkan.
* Tujuan aktif: [tujuan](/docs/id/goal#resume-with-an-active-goal) yang masih aktif ketika sesi berakhir terbawa; hitungan giliran, pengatur waktu, dan baseline pengeluaran token-nya disetel ulang.
* Tugas terjadwal: [tugas yang belum kedaluwarsa](/docs/id/scheduled-tasks#limitations) dipulihkan. Tugas Bash latar belakang dan monitor tidak.

Tidak setiap bendera konfigurasi dari peluncuran asli dipulihkan. Jika sesi bergantung pada `--mcp-config`, `--settings`, `--plugin-dir`, `--fallback-model`, atau direktori yang ditambahkan dengan `--add-dir`, teruskan lagi saat Anda melanjutkan; direktori yang ditambahkan pertengahan sesi dengan `/add-dir` juga tidak dipulihkan, meskipun pemilih sesi masih menggunakannya untuk menemukan sesi. File pengaturan standar, seperti `settings.json` dan `settings.local.json`, dibaca ulang saat peluncuran, sehingga konfigurasi yang berada di dalamnya tidak perlu dilewatkan lagi. Untuk `--system-prompt` dan `--append-system-prompt`, lihat [Bendera prompt sistem dalam percakapan yang dilanjutkan](/docs/id/cli-reference#system-prompt-flags-in-resumed-conversations).

<h4 id="permission-mode-on-resume">
  Mode izin saat melanjutkan
</h4>

Mode izin mana yang dimulai Claude Code dalam sesi yang dilanjutkan tergantung pada cara Anda melanjutkan:

* Terminal: `claude --continue`, `claude --resume <session-id>`, atau `claude --resume <name>` ketika nama cocok dengan satu sesi, tanpa `-p`. Claude Code memulihkan mode izin yang ada di sesi, kecuali dalam kasus di tabel. Teruskan `--permission-mode` atau `--dangerously-skip-permissions` untuk mengganti mode yang dipulihkan.
* Non-interaktif: `claude -p --resume` atau `claude -p --continue`. Claude Code memulai jalankan dalam mode izin yang akan dimulai oleh jalankan `claude -p` baru, kecuali sesi yang berakhir dalam mode rencana dilanjutkan dalam mode rencana di bawah [kondisi di bawah](#resume-in-plan-mode-with-p).
* VS Code: panel percakapan ekstensi. Tabel hanya mencakup percakapan yang berakhir dalam mode rencana; untuk sisanya, lihat [lanjutkan percakapan masa lalu](/docs/id/vs-code#resume-past-conversations).
* Pemilih sesi saat peluncuran: sesi yang Anda pilih dari [pemilih sesi](#use-the-session-picker), apakah Anda membukanya dengan `claude --resume` saja, `claude --from-pr`, atau nama yang cocok dengan lebih dari satu sesi. Claude Code tidak memulihkan mode izin yang disimpan. Ini memulai sesi dalam mode izin yang akan dimulai oleh sesi baru dari baris perintah yang sama.
* `/resume` di dalam sesi, dengan atau tanpa argumen: Claude Code tidak memulihkan mode izin yang disimpan. Percakapan yang Anda alihkan berlanjut dalam mode izin yang ada di sesi saat ini.

Memulihkan mode rencana pada jalur non-interaktif dan VS Code memerlukan Claude Code v2.1.246 atau lebih baru. Setiap baris menyebutkan mode izin yang diakhiri sesi, jalur terminal, non-interaktif, dan VS Code mana yang Anda lanjutkan, dan mode izin yang dimulai Claude Code dalam sesi yang dilanjutkan.

| Sesi berakhir dalam | Cara Anda melanjutkan                                                    | Mode izin setelah Anda melanjutkan                                                                                                                                                                                                                                                                                                                                       |
| :------------------ | :----------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `bypassPermissions` | Terminal                                                                 | Mode izin yang akan dimulai oleh sesi baru. Untuk [melewati izin](/docs/id/permission-modes#skip-all-checks-with-bypasspermissions-mode) lagi, aktifkan saat peluncuran dengan salah satu bendera peluncurannya atau `permissions.defaultMode: "bypassPermissions"` dalam [pengaturan pengguna, `--settings`, atau terkelola](/docs/id/settings-reference#permissions-defaultmode) |
| `plan`              | Terminal                                                                 | Mode izin yang akan dimulai oleh sesi baru                                                                                                                                                                                                                                                                                                                               |
| `auto`              | Terminal                                                                 | `auto`, hanya ketika akun Anda masih memenuhi [persyaratan mode otomatis](/docs/id/permission-modes#eliminate-prompts-with-auto-mode)                                                                                                                                                                                                                                         |
| Manual              | Terminal                                                                 | Manual ketika sesi baru akan dimulai dalam mode otomatis dari [default bawaan](/docs/id/permission-modes#which-mode-a-session-starts-in). Ketika `defaultMode` dari file pengaturan [berlaku](/docs/id/permission-modes#which-mode-a-session-starts-in), Claude Code memulai sesi yang dilanjutkan dalam mode tersebut                                                             |
| `plan`              | Non-interaktif, di bawah [kondisi di bawah](#resume-in-plan-mode-with-p) | Mode rencana                                                                                                                                                                                                                                                                                                                                                             |
| Mode apa pun        | Non-interaktif, dalam kasus lain apa pun                                 | Mode izin yang akan dimulai oleh jalankan `claude -p` baru                                                                                                                                                                                                                                                                                                               |
| `plan`              | VS Code                                                                  | Mode rencana, dengan [pengecualian pada halaman VS Code](/docs/id/vs-code#resume-past-conversations)                                                                                                                                                                                                                                                                          |

<h5 id="resume-in-plan-mode-with-p">
  Lanjutkan dalam mode rencana dengan `-p`
</h5>

Jalankan `claude -p --resume` atau `claude -p --continue` dilanjutkan dalam mode rencana hanya ketika keempat kondisi berlaku:

* Anda meneruskan [`--permission-prompt-tool`](/docs/id/cli-reference#cli-flags), sehingga Claude Code dapat menyajikan rencana untuk persetujuan
* Anda tidak meneruskan `--permission-mode` atau `--dangerously-skip-permissions`
* Anda tidak meneruskan `--fork-session`
* Jalankan tidak dimulai melalui [saluran](/docs/id/channels)

<h3 id="resume-from-a-summary">
  Lanjutkan dari ringkasan
</h3>

Pada paket Pro atau Max, ketika Anda melanjutkan sesi yang tidak aktif selama lebih dari sekitar satu jam dan lebih dari 100.000 token, Claude Code memulihkan percakapan dan kemudian membuka dialog sebelum Anda mengirim pesan pertama Anda. [Cache prompt](/docs/id/prompt-caching#cache-lifetime) sesi telah kedaluwarsa pada saat itu, sehingga permintaan berikutnya memproses riwayat lengkap sekali tidak peduli opsi dialog mana yang Anda pilih.

Dialog menawarkan tiga cara untuk melanjutkan sesi. Mereka berbeda dalam berapa banyak percakapan yang masing-masing bawa ke permintaan nanti, yang merupakan pertukaran antara menyimpan setiap detail dan mengirim lebih sedikit token per permintaan:

* **Lanjutkan dari ringkasan**: menjalankan [`/compact`](/docs/id/context-window#what-survives-compaction) segera. Claude Code mengirim satu permintaan peringkasan atas riwayat lengkap, kemudian mengganti riwayat dengan ringkasan, pertukaran terbaru Anda, dan hingga lima file yang baru-baru ini dibaca. Permintaan nanti membawa ringkasan daripada riwayat lengkap.
* **Lanjutkan sesi penuh apa adanya**: memuat percakapan tanpa perubahan. Setelah Anda mengirim pesan pertama Anda, Claude Code memproses ulang dan meng-cache ulang riwayat lengkap, kemudian membacanya kembali dari cache pada permintaan nanti sementara cache tetap hangat.
* **Jangan tanyakan saya lagi**: melanjutkan sesi penuh dan berhenti menampilkan dialog pada semua lanjutan masa depan.

Melanjutkan apa adanya menyimpan setiap detail percakapan yang tersedia, dengan biaya per permintaan yang diskalakan dengan ukuran percakapan. Melanjutkan dari ringkasan biaya lebih sedikit pada setiap permintaan nanti karena membawa ringkasan daripada riwayat lengkap, tetapi apa pun yang ditinggalkan ringkasan tidak lagi dalam konteks Claude. Lihat [mengapa penggunaan meningkat dalam sesi panjang](/docs/id/costs#why-usage-climbs-in-a-long-session) untuk tempat biaya per permintaan itu berasal.

<h3 id="where-the-session-picker-looks">
  Tempat pemilih sesi mencari
</h3>

Claude Code menyimpan sesi per direktori proyek. Secara default pemilih sesi menampilkan:

* Sesi dari worktree saat ini, termasuk [sesi latar belakang](/docs/id/agent-view), yang ditandai `bg` dalam daftar
* Sesi yang dimulai di tempat lain yang menambahkan direktori saat ini dengan `/add-dir`

Gunakan `Ctrl+W` untuk memperluas ke semua worktree repositori atau `Ctrl+A` untuk memperluas ke setiap proyek di mesin ini.

Sesi yang prompt pertamanya adalah perintah [`/loop`](/docs/id/scheduled-tasks#run-a-prompt-repeatedly-with-%2Floop) tidak muncul dalam pemilih, dan `claude --continue` juga melewati mereka. Menjalankan `/loop` nanti dalam percakapan tidak menyembunyikan sesi. Sebelum v2.1.211, jalankan `/loop` awal dalam percakapan menyembunyikan sesi dari pemilih secara permanen.

Memindahkan sesi dengan [`/cd`](/docs/id/commands) memindahkannya ke penyimpanan proyek direktori baru, sehingga muncul dalam pemilih direktori tersebut setelahnya. Sejak v2.1.196, sesi yang dipindahkan tetap keluar dari pemilih direktori lama bahkan setelah kerusakan atau keluar paksa. Pada versi sebelumnya, sesi juga dapat muncul kembali dalam daftar direktori lama setelah keluar yang tidak bersih ketika jalur lama berisi karakter khusus seperti garis bawah.

Ketika Anda memilih sesi dari worktree lain dari repositori yang sama, Claude Code melanjutkannya di tempat; ketika worktree sesi sendiri tidak lagi ada, Claude Code [melanjutkannya dalam direktori saat ini Anda](/docs/id/worktrees#resume-a-worktree-session). Ketika Anda memilih sesi dari proyek yang tidak terkait, Claude Code menyalin perintah `cd` dan resume ke clipboard Anda. Jika direktori proyek tersebut tidak lagi ada, Claude Code melanjutkan sesi dalam direktori saat ini Anda daripada menyalin perintah `cd` yang akan gagal.

Melanjutkan berdasarkan nama diselesaikan di seluruh repositori saat ini dan worktree-nya. Kedua bentuk mencari kecocokan yang tepat dan melanjutkannya secara langsung bahkan jika berada di worktree berbeda:

| Perintah                 | Kecocokan tepat             | Nama ambigu                                                                       |
| :----------------------- | :-------------------------- | :-------------------------------------------------------------------------------- |
| `claude --resume <name>` | Melanjutkan secara langsung | Membuka pemilih sesi dengan nama yang sudah diisi sebagai istilah pencarian       |
| `/resume <name>`         | Melanjutkan secara langsung | Melaporkan kesalahan; jalankan `/resume` tanpa argumen untuk membuka pemilih sesi |

<h2 id="name-your-sessions">
  Beri nama pada sesi Anda
</h2>

Berikan sesi nama deskriptif sehingga mudah ditemukan di pemilih sesi dan dapat dilanjutkan berdasarkan nama. Hal ini paling penting ketika Anda mengerjakan beberapa tugas secara paralel.

| Kapan                               | Cara mengatur nama                                                                                                                                                                |
| :---------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Saat startup                        | `claude -n auth-refactor`                                                                                                                                                         |
| Selama sesi                         | `/rename auth-refactor`. Nama juga muncul di bilah prompt                                                                                                                         |
| Dari pemilih sesi                   | Sorot sesi dan tekan `Ctrl+R`                                                                                                                                                     |
| Saat menerima plan                  | Menerima plan dalam [plan mode](/docs/id/permission-modes#analyze-before-you-edit-with-plan-mode) memberikan sesi judul yang dihasilkan berdasarkan plan kecuali Anda telah menamainya |
| Dari claude.ai atau aplikasi Claude | Ubah nama [sesi Remote Control](/docs/id/remote-control#connect-from-another-device); Claude Code menerapkan nama yang sama di CLI. Memerlukan Claude Code v2.1.221 atau lebih baru    |
| Dari aplikasi desktop               | Ubah nama sesi di [aplikasi desktop](/docs/id/desktop#work-in-parallel-with-sessions)                                                                                                  |

Setelah Anda memberi nama sesi melalui rute CLI atau dari claude.ai, kembali ke sesi tersebut dengan `claude --resume <name>` atau `/resume <name>`; sesi aplikasi desktop dilanjutkan di aplikasi, yang menyimpan riwayat sesinya sendiri. Lihat [Resume a session](#resume-a-session) untuk mengetahui bagaimana perilaku resolusi nama di seluruh worktrees.

Ketika Anda memulai atau melanjutkan sesi interaktif dengan nama yang sudah digunakan oleh sesi live lain di mesin ini, atau mengganti nama sesi menjadi nama seperti itu, Claude Code membiarkan nama dengan sesi yang sudah memilikinya, mengganti nama Anda menjadi varian dengan akhiran dua kata, seperti `auth-refactor-graceful-unicorn`, dan memberitahu Anda. Jalankan `/rename` dengan nama baru jika Anda lebih suka memilih sendiri. Sebelum v2.1.232, kedua sesi menyimpan nama tersebut.

Dalam tiga kasus Claude Code tidak mengganti nama duplikat, sehingga Anda masih dapat melihat dua sesi dengan nama yang sama dalam daftar:

* Tidak memeriksa judul yang dihasilkan AI atau nama tampilan default.
* Tidak memeriksa `--name` dari sesi [background](/docs/id/agent-view#from-your-shell) atau `-p` saat startup.
* Tidak dapat mengganti nama sesi di versi Claude Code yang lebih awal.

Sesi yang tidak Anda beri nama masih mendapatkan dua label yang ditetapkan Claude Code. Hanya judul yang dihasilkan yang berfungsi sebagai handle resume:

* Nama tampilan default: sesi interaktif yang tidak pernah Anda beri nama masih mendapatkan nama tampilan default saat dimulai. Memerlukan Claude Code v2.1.196 atau lebih baru. Default menggabungkan nama direktori kerja dengan akhiran dua karakter, misalnya `my-app-3f`, dan mengidentifikasi sesi dalam daftar sesi yang berjalan, seperti [agent view](/docs/id/agent-view) dan output `claude agents --json`. Default bukan handle resume. Jika Anda meneruskannya ke `claude --resume` atau `/resume`, Claude Code tidak menemukan sesi. Memberi nama sesi menggantikan default dalam daftar tersebut, begitu juga dengan menerima plan.
* Judul yang dihasilkan: jika Anda tidak memberi nama sesi, Claude Code menghasilkan judul sesi untuk itu. Judul adalah ringkasan singkat dari prompt pertama Anda, ditulis oleh permintaan latar belakang ke model kecil/cepat, biasanya model kelas Haiku. Jalankan `claude -p` yang Anda mulai langsung dari shell atau script tidak mendapatkan satu.

  Menerima plan menggantikan judul prompt pertama dengan judul berdasarkan plan. Memberi nama sesi juga menggantinya.

  Anda melihat judul prompt pertama di [pemilih sesi](#use-the-session-picker) dan di bidang [`session_name`](/docs/id/statusline) statusline ketika tidak ada nama yang diatur. Judul plan ditampilkan di tempat yang sama dan juga dalam daftar sesi yang berjalan, di mana ia menggantikan nama tampilan default.

  Anda dapat meneruskan salah satu judul ke `claude --resume` atau `/resume`, dan Claude Code menyelesaikannya dengan cara yang sama seperti nama yang Anda atur.

<h2 id="use-the-session-picker">
  Gunakan pemilih sesi
</h2>

Jalankan `/resume` di dalam sesi, atau `claude --resume` tanpa argumen, untuk membuka pemilih sesi interaktif. Gunakan pintasan keyboard ini untuk menavigasi, mencari, dan memperluas daftar:

| Pintasan                                                    | Tindakan                                                                                                                                                             |
| :---------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `↑` / `↓`                                                   | Navigasi antar sesi                                                                                                                                                  |
| `→` / `←`                                                   | Perluas atau ciutkan sesi yang dikelompokkan                                                                                                                         |
| `Enter`                                                     | Lanjutkan sesi yang disorot                                                                                                                                          |
| `Space`                                                     | Pratinjau konten sesi. `Ctrl+V` juga berfungsi di terminal yang tidak menangkapnya sebagai tempel                                                                    |
| `Ctrl+R`                                                    | Ubah nama sesi yang disorot                                                                                                                                          |
| `/` atau karakter yang dapat dicetak lainnya selain `Space` | Masuk mode pencarian dan filter sesi. Tempel URL permintaan tarik atau gabung GitHub, GitHub Enterprise, GitLab, atau Bitbucket untuk menemukan sesi yang membuatnya |
| `Ctrl+A`                                                    | Tampilkan sesi dari semua proyek di mesin ini. Tekan lagi untuk kembali ke repositori saat ini                                                                       |
| `Ctrl+W`                                                    | Tampilkan sesi dari semua worktrees repositori saat ini. Tekan lagi untuk kembali ke worktree saat ini. Hanya ditampilkan di repositori multi-worktree               |
| `Ctrl+B`                                                    | Filter ke sesi dari cabang git saat ini. Tekan lagi untuk menampilkan semua cabang                                                                                   |
| `Esc`                                                       | Keluar dari pemilih sesi atau mode pencarian                                                                                                                         |

Setiap baris menampilkan nama sesi jika Anda menetapkan satu, jika tidak judul sesi yang dihasilkan AI, ringkasan percakapan, atau prompt pertama, bersama dengan waktu sejak aktivitas terakhir, cabang git, dan ukuran file. Perluas ke semua proyek dengan `Ctrl+A` untuk juga melihat jalur proyek setiap sesi.

Sesi yang dibuat dengan `/branch` atau `--fork-session` mendapatkan ID sesi mereka sendiri dan muncul sebagai baris terpisah. Ketika pemilih menemukan lebih dari satu entri untuk sesi yang sama, sesi tersebut dikelompokkan di bawah satu baris. Tekan `→` untuk memperluas grup.

Jika Claude Code tidak dapat memuat sesi yang Anda pilih dari pemilih `claude --resume`, sesi tersebut mencetak [`Failed to resume the conversation`](/docs/id/errors#failed-to-resume-the-conversation) dengan perintah untuk mencoba lagi, kemudian keluar dengan kode 1. Dari pemilih `/resume` di dalam sesi, Claude Code melaporkan kegagalan dan percakapan saat ini Anda terus berjalan.

<h2 id="branch-a-session">
  Cabang sesi
</h2>

Pencabangan membuat salinan percakapan sejauh ini dan beralih Anda ke dalamnya, meninggalkan yang asli tetap utuh. Gunakan untuk mencoba pendekatan berbeda tanpa kehilangan jalur yang Anda jalani.

Dari dalam sesi, jalankan `/branch` dengan nama opsional:

```text theme={null}
/branch try-streaming-approach
```

Jika Anda menghilangkan nama, Claude Code memberi nama cabang baru setelah prompt pertama dalam percakapan. Mulai dari v2.1.198 ini juga berlaku setelah [compaction](/docs/id/how-claude-code-works#when-context-fills-up); versi sebelumnya kembali ke nama literal `Branched conversation` alih-alih melihat melampaui ringkasan compaction ke prompt pertama asli.

Dari baris perintah, gabungkan `--continue` atau `--resume` dengan `--fork-session`:

```bash theme={null}
claude --continue --fork-session
```

Konfirmasi `/branch` mencetak dua ID sesi: cabang baru yang sekarang Anda masuki dan yang asli. Yang asli tidak berubah di disk dan tetap tersedia di pemilih sesi; kembali ke sana dengan `/resume <original-name>` atau dengan meneruskan ID-nya ke `/resume`.

`/branch` menyalin transkrip dan beralih proses Claude Code yang berjalan untuk menulis ke dalamnya. Perbedaan itu menentukan apa yang diwarisi cabang:

| Keadaan                                                                                                                                                                                      | Setelah `/branch`                                                                                                                                                                                                          |
| :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Riwayat percakapan                                                                                                                                                                           | Disalin ke dalam cabang hingga titik Anda menjalankan `/branch`                                                                                                                                                            |
| Pemberian izin "Izinkan untuk sesi ini"                                                                                                                                                      | Dibawa; cabang berjalan dalam proses yang sama, jadi hibah yang ada masih berlaku. Jika Anda melakukan fork ke proses terpisah dengan `--fork-session`, proses baru dimulai tanpa mereka dan Anda menyetujui ulang di sana |
| [Subagen latar belakang](/docs/id/sub-agents#run-subagents-in-foreground-or-background) dan [perintah Bash latar belakang](/docs/id/interactive-mode#background-bash-commands) yang sedang berlangsung | Terus berjalan. Output mereka muncul di cabang baru yang Anda alihkan, bukan di sesi asli                                                                                                                                  |
| Koneksi [Remote Control](/docs/id/remote-control)                                                                                                                                                 | Tetap terhubung. Ponsel atau browser yang terhubung ke sesi mengikuti Anda ke dalam cabang dan terus menerima pesan baru di sana                                                                                           |

Jika Anda melanjutkan sesi yang sama di dua terminal tanpa melakukan fork, pesan dari keduanya saling bersisipan menjadi satu transkrip. Untuk rewind berbasis checkpoint dalam satu sesi, lihat [Checkpointing](/docs/id/checkpointing).

<h2 id="manage-context-within-a-session">
  Kelola konteks dalam sesi
</h2>

Perintah ini mengontrol apa yang ada di jendela konteks tanpa meninggalkan sesi:

* **`/clear`**: mulai segar dengan konteks kosong. Claude Code menyimpan percakapan sebelumnya; lanjutkan dengan `/resume`, atau, dalam proses Claude Code yang sama, dari [entri sesi sebelumnya di menu rewind](/docs/id/checkpointing#rewind-past-a-cleared-conversation). Tanpa argumen, percakapan baru mempertahankan nama yang Anda tetapkan dengan `--name` atau `/rename`, tetapi bukan judul sesi yang dihasilkan AI. Untuk memberi nama percakapan yang Anda tinggalkan, berikan nama, seperti dalam `/clear release-prep`; percakapan baru kemudian dimulai tanpa nama
* **`/compact [instructions]`**: ganti riwayat dengan ringkasan, secara opsional berfokus pada apa yang Anda tentukan
* **`/context`**: tampilkan apa yang saat ini mengonsumsi konteks

Untuk cara pemadatan berinteraksi dengan CLAUDE.md, skills, dan aturan, lihat [panduan jendela konteks](/docs/id/context-window). Untuk strategi tentang kapan harus menghapus versus memadatkan, lihat [Praktik terbaik](/docs/id/best-practices#manage-your-session).

<h2 id="export-and-locate-session-data">
  Ekspor dan temukan data sesi
</h2>

Jalankan `/export` untuk membuka menu yang memungkinkan Anda menyalin percakapan saat ini ke clipboard atau menyimpannya sebagai file teks biasa, dengan pesan dan output alat dirender sebagai teks yang dapat dibaca. Teruskan nama file untuk melewati menu dan menulis langsung ke file tersebut.

<h3 id="access-conversations-from-scripts">
  Akses percakapan dari skrip
</h3>

`/export` menghasilkan transkrip yang dirender untuk dibaca oleh seseorang. Antarmuka di bawah ini menghasilkan data terstruktur untuk skrip diurai: hasil JSON dari suatu run, jalur ke file transkrip sesi, atau aliran peristiwa langsung. Pilih berdasarkan apa yang memicu skrip:

* **Jalankan Claude sekali dan tangkap hasilnya**: panggil `claude -p` dengan [`--output-format json` atau `stream-json`](/docs/id/headless#get-structured-output) untuk menangkap hasil, ID sesi, penggunaan, dan biaya dari run non-interaktif sebagai JSON terstruktur.
* **Tanyakan pertanyaan ke sesi yang ada**: teruskan ID sesi ke [`claude -p --resume`](/docs/id/headless#continue-conversations) untuk mengirim prompt tindak lanjut, seperti permintaan ringkasan, dan tangkap respons terstruktur.
* **Bereaksi terhadap peristiwa sesi**: baca bidang `transcript_path` yang diterima [hooks](/docs/id/hooks#common-input-fields) dan [perintah status line](/docs/id/statusline#available-data) sebagai input. Hook `SessionEnd` dapat mengarsipkan transkrip ketika sesi berakhir.
* **Sematkan Claude dalam aplikasi TypeScript atau Python**: gunakan [Agent SDK](/docs/id/agent-sdk/overview) untuk menerima setiap pesan secara terprogram.

Contoh di bawah menggunakan antarmuka kedua. Ini mengirim prompt tindak lanjut ke sesi yang ada dan membaca jawaban dengan `jq`:

```bash theme={null}
claude -p --resume <session-id> --output-format json "summarize what we changed" | jq -r '.result'
```

<h3 id="where-transcripts-are-stored">
  Tempat transkrip disimpan
</h3>

Secara default, Claude Code menyimpan transkrip sebagai JSONL di `~/.claude/projects/<project>/<session-id>.jsonl`, di mana `<project>` adalah jalur direktori kerja Anda dengan karakter non-alfanumerik diganti dengan `-`. Untuk direktori kerja yang nama konversinya melebihi 200 karakter, Claude Code memotong nama menjadi 200 karakter dan menambahkan hash dari jalur lengkap, sehingga nama direktori tetap dalam batas sistem file.

Setiap baris adalah objek JSON untuk pesan, penggunaan alat, atau entri metadata. Format entri bersifat internal untuk Claude Code dan berubah antar versi, jadi skrip yang mengurai file ini secara langsung dapat rusak pada rilis apa pun. Untuk membangun data sesi, gunakan `/export` atau [antarmuka skrip](#access-conversations-from-scripts) sebagai gantinya.

Lokasi, retensi, dan perilaku penulisan dapat dikonfigurasi:

| Untuk                                                                                                      | Atur                                                                                        | Di mana                                                      |
| ---------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------- | ------------------------------------------------------------ |
| Pindahkan penyimpanan dari `~/.claude`                                                                     | [`CLAUDE_CONFIG_DIR`](/docs/id/env-vars)                                                         | Variabel lingkungan                                          |
| [Beri nama direktori `<project>` sendiri](#name-the-project-directory-yourself)                            | [`CLAUDE_CODE_PROJECT_DIR_NAME`](/docs/id/env-vars)                                              | Variabel lingkungan                                          |
| Ubah retensi 30 hari                                                                                       | [`cleanupPeriodDays`](/docs/id/settings-reference#cleanupperioddays)                             | `settings.json`                                              |
| Atur batas usia untuk [transkrip Claude Desktop dan Cowork](/docs/id/claude-directory#cleaned-up-automatically) | [`desktopSessionCleanupPeriodDays`](/docs/id/settings-reference#desktopsessioncleanupperioddays) | Pengaturan pengguna, pengaturan terkelola, atau `--settings` |
| Tekan penulisan transkrip di semua mode                                                                    | [`CLAUDE_CODE_SKIP_PROMPT_HISTORY`](/docs/id/env-vars)                                           | Variabel lingkungan                                          |
| Tekan penulisan untuk satu run non-interaktif                                                              | [`--no-session-persistence`](/docs/id/cli-reference)                                             | Flag CLI dengan `claude -p`                                  |

<h3 id="delete-session-data">
  Hapus data sesi
</h3>

Transkrip berusia di bawah [aturan penyapuan retensi](/docs/id/claude-directory#cleaned-up-automatically). Untuk menghapus transkrip proyek dan status terkait lebih cepat, jalankan [`claude project purge`](/docs/id/claude-directory#clear-local-data). Jika Anda menghapus [sesi latar belakang](/docs/id/agent-view) dengan [`claude rm <id>`](/docs/id/agent-view#what-deleting-a-session-removes), transkrip tetap di disk dan tetap tersedia melalui `claude --resume`.

<h3 id="name-the-project-directory-yourself">
  Beri nama direktori proyek sendiri
</h3>

Secara default, Claude Code menurunkan nama `<project>` dari seluruh jalur direktori kerja. Untuk memilih nama sendiri, atur `CLAUDE_CODE_PROJECT_DIR_NAME` bersama dengan `CLAUDE_CONFIG_DIR`. Claude Code kemudian menyimpan transkrip sesi tersebut dan [auto memory](/docs/id/memory#auto-memory) di bawah nama Anda. Ini cocok untuk host yang menyematkan Claude Code dan memberikan setiap sesi direktori konfignya sendiri. Memerlukan Claude Code v2.1.234 atau lebih baru.

Misalnya, peluncuran ini menjaga data penyewa A di bawah `/srv/tenant-a` dan memberi nama direktori proyeknya `work`:

```bash theme={null}
CLAUDE_CONFIG_DIR=/srv/tenant-a CLAUDE_CODE_PROJECT_DIR_NAME=work claude
```

Claude Code menulis transkrip sesi ke `/srv/tenant-a/projects/work/` dan auto memory-nya ke `/srv/tenant-a/projects/work/memory/`, apa pun direktori kerjanya.

Tiga aturan berlaku ketika Anda menetapkannya:

* **Atur `CLAUDE_CONFIG_DIR` juga**: nama tidak bervariasi dengan direktori kerja, jadi di bawah `~/.claude` default, nama tersebut akan menggabungkan transkrip dan auto memory setiap proyek ke dalam satu direktori. Claude Code mengabaikan `CLAUDE_CODE_PROJECT_DIR_NAME` ketika `CLAUDE_CONFIG_DIR` tidak diatur.
* **Gunakan 1-64 huruf, digit, tanda hubung, atau garis bawah**: jangan gunakan nama perangkat Windows seperti `con`. Claude Code mengabaikan nilai apa pun dan menggunakan nama yang diturunkan.
* **Atur di lingkungan shell yang memulai `claude`**: Claude Code membacanya sekali saat startup dari lingkungan tersebut, jadi blok `env` dalam file pengaturan tidak dapat menetapkannya.

Setelah Anda memberi nama direktori proyek direktori konfigurasi, terus luncurkan dengan nama tersebut. Jika Anda memulai Claude Code dengan `CLAUDE_CONFIG_DIR` yang sama tetapi tanpa `CLAUDE_CODE_PROJECT_DIR_NAME`, Claude Code membaca dan menulis direktori yang diturunkan lagi. Sesi yang disimpan di bawah nama Anda tetap di disk: tekan `Ctrl+A` di [pemilih sesi](#use-the-session-picker) untuk mencantumkan sesi dari setiap direktori proyek di bawah direktori konfigurasi tersebut, termasuk yang disematkan, dan apa pun cara Anda meluncurkan, [`claude --resume <session-id>`](#resume-a-session) menemukan sesi yang disimpan di bawah nama apa pun.

<h2 id="see-also">
  Lihat juga
</h2>

Halaman-halaman ini mencakup mekanik sesi dan paralelisme terkait:

* [Worktrees](/docs/id/worktrees): jalankan sesi paralel terisolasi di cabang terpisah
* [Checkpointing](/docs/id/checkpointing): putar ulang kode dan percakapan ke titik sebelumnya
* [Jendela konteks](/docs/id/context-window): apa yang mengisi konteks dan apa yang bertahan dari pemadatan
* [Mode non-interaktif](/docs/id/headless): perilaku sesi di bawah `claude -p`
