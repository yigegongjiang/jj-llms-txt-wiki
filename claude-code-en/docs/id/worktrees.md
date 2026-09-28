> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Jalankan sesi paralel dengan worktrees

> Isolasi sesi Claude Code paralel dalam git worktrees terpisah sehingga perubahan tidak bertabrakan. Mencakup flag `--worktree`, isolasi subagent, `.worktreeinclude`, pembersihan, dan hook VCS non-git.

[git worktree](https://git-scm.com/docs/git-worktree) adalah direktori kerja terpisah dengan file dan cabang sendiri, berbagi riwayat repositori dan remote yang sama dengan checkout utama Anda. Menjalankan setiap sesi Claude Code dalam worktree-nya sendiri berarti edit dalam satu sesi tidak akan pernah menyentuh file di sesi lain, sehingga satu sesi dapat membangun fitur sementara sesi kedua memperbaiki bug.

<Note>
  Worktrees memerlukan repositori git; untuk sistem kontrol versi lainnya, [konfigurasikan hook untuk menggantikan logika git](#non-git-version-control). Di [aplikasi desktop](/docs/id/desktop#work-in-parallel-with-sessions), pilih opsi **worktree** saat Anda memulai sesi untuk memberikannya worktree-nya sendiri.
</Note>

Worktrees adalah salah satu dari beberapa cara untuk menjalankan Claude secara paralel. Mereka mengisolasi edit file. [Subagents](/docs/id/sub-agents) membagi pekerjaan di dalam satu sesi, dan [cross-session messaging](/docs/id/cross-session-messaging) memungkinkan Claude melewatkan temuan antar sesi di worktrees Anda. Lihat [Run agents in parallel](/docs/id/agents) untuk membandingkan pendekatan, atau lompat ke [Isolate subagents with worktrees](#isolate-subagents-with-worktrees) untuk menggunakan worktrees dan subagents bersama-sama.

Sebagian besar sesi hanya memerlukan dua bagian pertama: [mulai Claude dalam worktree](#start-claude-in-a-worktree), kemudian [bersihkan saat Anda keluar](#clean-up-worktrees). Kembali ke sisa halaman saat Anda perlu [melanjutkan sesi](#resume-a-worktree-session), [mengubah cara worktrees dibuat](#customize-worktree-creation), atau [debug kegagalan](#troubleshooting).

<h2 id="start-claude-in-a-worktree">
  Mulai Claude dalam worktree
</h2>

Lewatkan `--worktree` atau `-w` dengan nama untuk membuat worktree terisolasi dan memulai Claude di dalamnya. Secara default, worktree dibuat di bawah `.claude/worktrees/<name>/` di root repositori Anda, pada cabang baru bernama `worktree-<name>`:

```bash theme={null}
claude --worktree feature-auth
```

Jalankan perintah lagi dengan nama berbeda di terminal lain untuk memulai sesi terisolasi kedua. Jika Anda menghilangkan nama, Claude menghasilkan satu seperti `bright-running-fox`.

Jalankan interaktif memerlukan [workspace trust](/docs/id/security): jika Anda belum menjalankan Claude di direktori sebelumnya, jalankan `claude` sekali di sana untuk menerima dialog kepercayaan, atau `--worktree` keluar dengan kesalahan yang meminta Anda untuk melakukannya. Jalankan non-interaktif dengan `-p` melewati pemeriksaan kepercayaan, jadi `claude -p --worktree` melanjutkan tanpanya.

<Tip>
  Tambahkan `.claude/worktrees/` ke `.gitignore` Anda sehingga konten worktree tidak muncul sebagai file yang tidak dilacak dalam checkout utama Anda.
</Tip>

<h3 id="set-up-the-worktree-environment">
  Atur lingkungan worktree
</h3>

Worktree adalah checkout segar, jadi inisialisasi lingkungan pengembangan Anda di sana: minta Claude untuk menginstal dependensi, atau jalankan setup proyek Anda sendiri di direktori worktree di bawah `.claude/worktrees/`. Untuk membawa file yang diabaikan git seperti `.env` ke setiap worktree baru secara otomatis, tambahkan file [`.worktreeinclude`](#copy-gitignored-files-into-worktrees).

<h3 id="ask-claude-to-create-a-worktree">
  Minta Claude untuk membuat worktree
</h3>

Anda juga dapat meminta Claude untuk "bekerja dalam worktree" selama sesi, dan itu membuat satu dengan tool [`EnterWorktree`](/docs/id/tools-reference). Setelah berada dalam worktree, Claude dapat beralih langsung ke worktree lain di bawah `.claude/worktrees/` dengan memanggil `EnterWorktree` dengan jalur target; worktree sebelumnya tetap berada di disk tanpa disentuh.

Saat Claude memasuki jalur di luar direktori `.claude/worktrees/` repositori, Claude Code meminta persetujuan Anda terlebih dahulu, karena perpindahan mengambil direktori kerja sesi, akses tulis, dan konfigurasi proyek seperti `CLAUDE.md` dan settings ke lokasi tersebut. Aturan [izin](/docs/id/permissions) `EnterWorktree` atau memilih "jangan tanya lagi" tidak menekan prompt ini; hanya mode `bypassPermissions` yang melewatinya. Sebelum v2.1.206, Claude dapat memasuki jalur worktree yang ada tanpa bertanya.

<Note>
  **Jalur hook tidak mengikuti worktree.** Setelah Claude memasuki worktree, Claude Code menyimpan `${CLAUDE_PROJECT_DIR}` dalam [hooks](/docs/id/hooks#reference-scripts-by-path) Anda di mana itu berada dan melewatkan jalur worktree kepada mereka dengan cara berbeda:

  * **`${CLAUDE_PROJECT_DIR}` tetap di tempat**: itu masih menunjuk ke root proyek di mana sesi dimulai, jadi perintah hook seperti `${CLAUDE_PROJECT_DIR}/.claude/hooks/check-style.sh` masih menjalankan skrip dalam checkout utama.
  * **`cwd` mengikuti Claude**: field `cwd` dalam [input JSON](/docs/id/hooks#common-input-fields) hook adalah root worktree, dan itu bergerak lagi saat Claude menjalankan `cd`. Bacanya saat hook memerlukan jalur worktree.
</Note>

<h2 id="clean-up-worktrees">
  Bersihkan worktrees
</h2>

Saat Anda keluar dari sesi worktree interaktif, Claude memeriksa worktree untuk pekerjaan yang akan dihapus oleh penghapusan: file yang diubah atau tidak dilacak, pekerjaan yang tidak dikomitkan di dalam submodul yang diperiksa, dan commit baru.

* **Worktree bersih**: untuk sesi tanpa nama, Claude menghapus worktree dan cabangnya secara otomatis. Sesi [bernama](/docs/id/sessions#name-your-sessions) meminta Anda terlebih dahulu sehingga Anda dapat menyimpan worktree untuk nanti
* **Worktree memiliki pekerjaan di dalamnya**: Claude meminta Anda untuk menyimpan atau menghapus worktree. Menyimpan mempertahankan direktori dan cabang sehingga Anda dapat kembali nanti. Menghapus menghapus direktori worktree dan cabangnya, bersama dengan semua pekerjaan di dalamnya
* **Status worktree tidak dapat diverifikasi**: ketika Claude Code tidak dapat menghitung perubahan worktree atau tidak dapat memeriksa checkout submodulnya, Claude meminta Anda daripada menghapus worktree secara otomatis. Prompt tersebut menyebutkan apa yang tidak dapat diperiksa

Jalankan non-interaktif dengan `-p` tidak memiliki prompt keluar, jadi Claude tidak membersihkan worktrees mereka, dan Claude Code meninggalkan kunci yang diambilnya pada setiap satu saat pembuatan di tempat sampai [sapuan kunci basi](#clean-up-subagent-and-background-session-worktrees) sesi nanti melepaskannya. Untuk menghapus satu, jalankan `git worktree remove`; jika git menolak karena worktree terkunci, jalankan `git worktree unlock` padanya terlebih dahulu.

Di Windows, menghapus worktree tidak menghapus file di luar itu. Jika folder di dalam worktree adalah tautan ke tempat lain, seperti junction NTFS atau symlink direktori, Claude Code menghapus hanya tautan dan menyimpan folder yang ditunjuknya. Sebelum v2.1.205, menghapus worktree dengan tautan bersarang di subdirektori dapat menghapus folder yang ditunjuknya.

<h2 id="resume-a-worktree-session">
  Lanjutkan sesi worktree
</h2>

Saat Anda melanjutkan sesi yang berada di dalam worktree, Claude Code mengembalikan sesi ke worktree itu. Ini berlaku untuk resume interaktif, untuk `--continue` dan `--resume` dalam [mode non-interaktif](/docs/id/headless) dengan `-p`, dan untuk Agent SDK. Kembali di dalam worktree, Claude masih dapat keluar darinya dengan tool [`ExitWorktree`](/docs/id/tools-reference).

Sebelum mengembalikan sesi ke worktree-nya, Claude Code memverifikasi bahwa worktree masih merupakan checkout terpisah dari yang utama, dan menolak untuk memasuki kembali worktree yang gagal pemeriksaan. Untuk git worktree, pemeriksaan membaca metadata git-nya. Worktree tanpa metadata git, seperti yang dibuat hook [`WorktreeCreate`](#non-git-version-control), dapat lulus pemeriksaan; kasus yang Claude Code masih tolak tercantum dengan pemulihan mereka di bawah [Claude Code refuses to use a worktree](#claude-code-refuses-to-use-a-worktree). Untuk pesan dan cara memulihkan dari masing-masing, lihat [The session resumes outside its worktree](#the-session-resumes-outside-its-worktree).

Di mana Anda meluncurkan, dan bagaimana Anda melanjutkan, mengubah apa yang Claude Code masuki kembali:

* **Direktori peluncuran**: lanjutkan dari checkout utama atau direktori lain dari repositori. Claude Code memasuki kembali worktree yang dibuat dengan git di bawah `.claude/worktrees/` bahkan saat Anda meluncurkan dari dalamnya. Saat Anda meluncurkan dari dalam worktree lain, Claude Code memasuki kembali hanya jika dapat menjaminnya dari sana: worktree yang merupakan repositorinya sendiri, satu tanpa metadata git, atau peluncuran dari subdirektori worktree yang Anda buat dengan `git worktree add` menolak, jadi luncurkan dari checkout utama.
* **`--fork-session`**: sesi yang di-fork dimulai di direktori tempat Anda meluncurkan Claude, dan Claude Code meninggalkan worktree sesi asli tanpa disentuh.
* **Worktree yang dihapus**: jika direktori worktree tidak lagi ada, Claude Code melanjutkan sesi di direktori tempat Anda meluncurkan Claude. Itu memberi tahu Anda worktree hilang dan menghapus pengikatan worktree sesi.

<Note>
  Sebelum v2.1.212, resume non-interaktif tetap berada di direktori awal dan `ExitWorktree` melaporkan bahwa tidak ada sesi worktree aktif untuk keluar.
</Note>

Saat Claude memasuki atau keluar dari worktree yang dibuat Claude Code dengan git, transkrip mengikuti: Claude Code merekam sesi di bawah direktori kerja baru sesi, dengan cara yang sama seperti [`/cd`](/docs/id/commands) melakukannya, jadi `/desktop` dan `--resume` menemukannya di sana. Keluar memindahkannya kembali dengan cara yang sama. Worktree yang dibuat oleh hook [`WorktreeCreate`](#non-git-version-control) menyimpan transkrip di direktori peluncuran. Memerlukan Claude Code v2.1.198 atau lebih baru.

<h2 id="how-claude-code-enforces-isolation">
  Bagaimana Claude Code memberlakukan isolasi
</h2>

Saat sesi terisolasi dalam worktree, Claude Code memblokir panggilan tool yang pemeriksaan di bawah tentukan. Aturan yang sama berlaku apakah Anda memulai sesi dengan `--worktree`, Claude memasuki worktree dengan `EnterWorktree`, atau Anda melanjutkan sesi worktree.

Penegakan yang sama mencakup setiap subagent yang Claude hasilkan dari sesi terisolasi. Itu berlaku apakah sesi interaktif atau berjalan di [latar belakang](/docs/id/agent-view#how-file-edits-are-isolated). [Subagents yang berjalan dalam worktree mereka sendiri](#isolate-subagents-with-worktrees) membawa pemeriksaan yang sama. Riwayat versi mereka berada di bawah [Write subagent files](/docs/id/sub-agents#write-subagent-files).

Claude Code menerapkan empat pemeriksaan:

* **Edit file**: Claude Code memblokir `Edit`, `Write`, atau `NotebookEdit` yang menargetkan jalur dalam checkout utama.
* **Direktori kerja perintah**: Claude Code memblokir perintah Bash, PowerShell, atau Monitor yang direktori kerjanya diselesaikan ke checkout utama, atau yang direktori kerjanya tidak dapat diverifikasi tetap di luar itu.
* **Pengalihan git**: Claude Code memblokir perintah Bash atau Monitor yang mengalihkan git ke checkout utama. Pengalihan dapat datang melalui `git -C`, `--git-dir`, variabel `GIT_DIR` atau `GIT_WORK_TREE`, atau `cd` ke checkout utama sebelum menjalankan git.
* **Bentuk perintah**: Claude Code memblokir perintah Bash atau Monitor saat tidak dapat memverifikasi dari teks perintah bahwa git apa pun yang dijalankan perintah tetap berada di dalam worktree. Itu terjadi, misalnya, saat nama perintah dihitung saat runtime, saat sintaks tidak dapat diuraikan, atau saat ekspansi seperti `${!name}` atau `${ command; }` dapat menjalankan perintah yang teks tidak jelaskan. Claude Code memberi tahu Claude cara menulis ulang perintah yang ditolak, seperti membaginya menjadi perintah biasa yang terpisah. Anda tidak dapat mematikan pemeriksaan ini.

Pemeriksaan berlaku untuk repositori tempat Anda meluncurkan Claude Code. Mereka juga mencakup checkout utama yang ditautkan worktree ditautkan darinya. Untuk perintah PowerShell, Claude Code hanya menerapkan pemeriksaan direktori kerja.

Claude melihat setiap penolakan sebagai kesalahan tool yang menamai worktree dan mengatakan cara melanjutkan. Untuk perintah yang ditolak, lihat [apa arti pesan penolakan dan cara menghapusnya](/docs/id/errors#command-blocked-by-the-worktree-isolation-checks).

<h2 id="isolate-subagents-with-worktrees">
  Isolasi subagents dengan worktrees
</h2>

Subagents dapat berjalan dalam worktrees mereka sendiri sehingga edit paralel tidak bertabrakan. Minta Claude untuk "gunakan worktrees untuk agen Anda", atau buat isolasi permanen untuk [subagent kustom](/docs/id/sub-agents#supported-frontmatter-fields) dengan menambahkan `isolation: worktree` ke frontmatter-nya.

Subagent ini di `.claude/agents/` selalu berjalan dalam worktree-nya sendiri:

```markdown theme={null}
---
name: refactorer
description: Applies mechanical refactors across many files
isolation: worktree
---

Apply the requested refactor across every affected file, then run the tests
and report the results.
```

Setiap subagent mendapatkan worktree sementara yang Claude Code hapus secara otomatis saat subagent selesai tanpa perubahan; worktree dengan perubahan tetap berada di disk sampai [sapuan berkala di bawah](#clean-up-subagent-and-background-session-worktrees) dapat menghapusnya tanpa kehilangan pekerjaan.

Worktrees subagent menggunakan [cabang dasar](#choose-the-base-branch) yang sama dengan `--worktree`, jadi mereka membuat cabang dari cabang default repositori Anda kecuali `worktree.baseRef` diatur ke `"head"`.

<h3 id="clean-up-subagent-and-background-session-worktrees">
  Bersihkan subagent dan worktrees sesi latar belakang
</h3>

Claude Code menjalankan sapuan berkala yang menghapus worktrees yang dibuat Claude untuk subagents dan [sesi latar belakang](/docs/id/agent-view#how-file-edits-are-isolated) setelah mereka lebih tua dari pengaturan [`cleanupPeriodDays`](/docs/id/settings-reference#cleanupperioddays) Anda, mengikuti [aturan sapuan retensi](/docs/id/claude-directory#cleaned-up-automatically).

Saat Anda [mengirim ke latar belakang](/docs/id/agent-view#send-the-session-to-the-background) sesi `--worktree`, worktree-nya menjadi worktree sesi latar belakang yang dapat dihapus sapuan. Sapuan meninggalkan worktree di tempat dalam kasus ini:

* Worktree masih menyimpan pekerjaan: file yang diubah atau tidak dilacak, atau commit yang belum didorong.
* Submodul yang diperiksa dalam worktree menyimpan file yang diubah atau tidak dilacak, atau Claude Code tidak dapat memeriksa submodul worktree. Pemeriksaan ini memerlukan Claude Code v2.1.274 atau lebih baru.
* Salah satu dari [empat kasus yang juga memblokir pembuatan worktree](#git-lfs-content-is-missing-from-a-worktree-claude-code-created) berlaku: Claude Code tidak dapat menentukan driver filter mana yang didefinisikan konfigurasi repositori, atau menemukan pengaturan di sana yang tidak dapat dimatikan.
* Worktree milik sesi `--worktree` yang belum Anda kirim ke latar belakang, apa pun usianya.
* Anda membuat worktree sendiri dengan `git worktree add`, bahkan jika Anda kemudian menjalankan sesi `--worktree <name>` di dalamnya dan mengirimnya ke latar belakang.

Claude Code menulis penanda ke metadata git setiap worktree yang dibuat dengan git, dan sapuan menyimpan worktree apa pun tanpa satu, termasuk worktree yang dibuat hook [`WorktreeCreate`](#non-git-version-control). Sebelum v2.1.246, sapuan tidak memeriksa penanda, dan dapat menghapus worktree yang Anda buat sendiri saat catatan sesi latar belakang lama menunjuk padanya.

Saat agen berjalan, Claude Code menyimpan `git worktree lock` pada worktree-nya sehingga pembersihan bersamaan tidak dapat menghapusnya, dan melepaskan kunci saat agen selesai. Claude Code menyimpan kunci yang sama pada worktree yang dibuat untuk sesi yang dikirim ke latar belakang saat sesi berjalan, jadi sapuan meninggalkan worktree di tempat dan `git worktree remove` menolak untuk menghapusnya.

Sapuan juga melepaskan kunci yang Claude Code atur untuk sesi yang proses-nya telah keluar, jadi sesi latar belakang yang terbunuh tidak meninggalkan worktree-nya terkunci secara permanen. Sapuan tidak pernah melepaskan kunci yang Anda atur sendiri dengan `git worktree lock`. Sebelum v2.1.210, kunci yang ditinggalkan sesi yang terbunuh tetap di tempat sampai Anda menjalankan `git worktree unlock`.

Untuk membersihkan worktree yang disimpan sapuan, jalankan `git worktree remove`, tambahkan `--force` jika worktree memiliki perubahan yang belum dikomit atau file yang tidak dilacak. Jika git menolak karena worktree terkunci, jalankan `git worktree unlock` padanya terlebih dahulu.

<h2 id="customize-worktree-creation">
  Sesuaikan pembuatan worktree
</h2>

Default Claude Code untuk membuat worktrees mencakup sebagian besar sesi: itu membuat mereka di bawah `.claude/worktrees/`, membuat cabang mereka dari cabang default repositori Anda, dan hanya checkout file yang dilacak. Opsi di bagian ini mengubah default tersebut.

<h3 id="choose-the-base-branch">
  Pilih cabang dasar
</h3>

Worktrees baru membuat cabang dari cabang default repositori, jadi sebagian besar sesi tidak memerlukan pengaturan ini. Atur `worktree.baseRef` dalam [settings](/docs/id/settings-reference#worktree) untuk membuat cabang dari pekerjaan saat ini Anda. Pengaturan menerima dua nilai:

* `"fresh"` (default): membuat cabang dari cabang default repositori di remote, biasanya `main`, jadi worktree dimulai dari pohon bersih yang cocok dengan remote.
* `"head"`: membuat cabang dari `HEAD` lokal saat ini Anda, jadi worktree membawa commit yang belum didorong dan status cabang fitur Anda. Gunakan ini saat mengisolasi subagents yang perlu beroperasi pada pekerjaan yang sedang berlangsung. Di dalam worktree, `"head"` diselesaikan ke `HEAD` worktree itu, bukan checkout utama.

Anda tidak dapat mengatur `worktree.baseRef` ke nama cabang. Untuk memulai worktree dari cabang yang ada tertentu, [buat dengan git secara langsung](#manage-worktrees-manually).

Untuk dasar `"fresh"`, Claude Code menyimpan `origin/HEAD` saat ini: saat repositori belum diambil dalam 24 jam terakhir, itu mengambil cabang default, dibatasi pada lima detik, dan menggunakan ref yang disimpan secara lokal jika pengambilan gagal. Jika tidak ada remote yang dikonfigurasi, atau `origin/HEAD` tidak disimpan secara lokal dan tidak dapat diambil, worktree kembali ke `HEAD` lokal saat ini Anda. Sebelum v2.1.208, worktree segar menggunakan apa pun yang sudah disimpan secara lokal di `origin/HEAD`.

Contoh ini membuat setiap worktree baru membuat cabang dari pekerjaan saat ini Anda:

```json theme={null}
{
  "worktree": {
    "baseRef": "head"
  }
}
```

<h3 id="branch-from-a-pull-request">
  Buat cabang dari pull request
</h3>

Untuk membuat cabang dari pull request atau merge request tertentu, lewatkan `--worktree` nomor dengan awalan `#`, URL pull request GitHub, atau URL merge request GitLab seperti `https://gitlab.com/group/repo/-/merge_requests/123`. Claude Code mengambil commit kepala perubahan itu dari `origin` dan membuat worktree di `.claude/worktrees/pr-<number>`. Kutip argumen sehingga shell Anda tidak memperlakukan `#` sebagai awal komentar:

```bash theme={null}
claude --worktree "#1234"
```

Claude Code hanya membaca nomor dari URL. Itu selalu mengambil dari remote `origin` repositori Anda, dan memilih jalur pengambilan berdasarkan host `origin`:

* **github.com**: mengambil `pull/<number>/head`
* **gitlab.com**: mengambil `merge-requests/<number>/head`
* **GitHub Enterprise, GitLab yang dikelola sendiri, atau host lainnya**: mencoba `pull/<number>/head` terlebih dahulu, kemudian `merge-requests/<number>/head`

Sebelum v2.1.233, Claude Code hanya menerima `#<number>` dan URL pull request gaya GitHub untuk `--worktree`, dan selalu mengambil `pull/<number>/head`.

<h3 id="copy-gitignored-files-into-worktrees">
  Salin file yang diabaikan git ke dalam worktrees
</h3>

Worktree adalah checkout segar, jadi file yang tidak dilacak seperti `.env` atau `.env.local` dari repositori utama Anda tidak ada. Untuk menyalinnya secara otomatis saat Claude membuat worktree, tambahkan file `.worktreeinclude` ke root proyek Anda.

File menggunakan sintaks `.gitignore`. Hanya file yang cocok dengan pola dan juga diabaikan git yang disalin, jadi file yang dilacak tidak pernah diduplikasi.

Jika Anda menulis pola yang dimulai dengan `**/` dan file yang Anda inginkan berada di dalam direktori yang diabaikan git secara keseluruhan, Claude Code menyalinnya hanya saat direktori itu sendiri cocok dengan pola, atau saat nama pertama setelah `**/` adalah salah satu nama dalam jalur direktori. Misalnya, jika Anda menulis `**/.claude/skills/*.md`, nama pertama itu adalah `.claude`, jadi Claude Code menyalin file yang cocok dari direktori `.claude/` yang diabaikan. Untuk menyalin file dari direktori yang diabaikan yang pola `**/` tidak jangkau, beri nama direktori dalam pola: tulis `vendor/**/config.json` daripada `**/config.json`. Sebelum v2.1.239, Claude Code menyalin file dari direktori yang sepenuhnya diabaikan untuk pola `**/` hanya saat direktori itu sendiri cocok dengan pola.

`.worktreeinclude` ini menyalin dua file env dan konfigurasi rahasia ke setiap worktree baru:

```text .worktreeinclude theme={null}
.env
.env.local
config/secrets.json
```

Ini berlaku untuk setiap worktree yang dibuat Claude Code dengan git: worktrees `--worktree`, [worktrees subagent](#isolate-subagents-with-worktrees), dan sesi paralel dalam [aplikasi desktop](/docs/id/desktop#work-in-parallel-with-sessions). Dengan hook [`WorktreeCreate`](#non-git-version-control), salin file di dalam skrip hook.

<h3 id="reuse-a-worktree-name">
  Gunakan kembali nama worktree
</h3>

Melewatkan `--worktree` nama yang direktorinya sudah ada membuka worktree yang ada itu daripada membuat yang baru.

Dengan `"fresh"` [dasar](#choose-the-base-branch) default, worktree yang dibuka kembali disetel ulang ke cabang default repositori daripada melanjutkan di ujung lamanya saat semua hal berikut berlaku:

* Tidak memiliki perubahan yang belum dikomit atau file yang tidak dilacak.
* Masih berada di cabang yang dibuat Claude Code untuk itu.
* Tidak memiliki commit sendiri, atau pull request atau merge request-nya digabungkan dan cabang remote-nya dihapus.

Claude Code mendeteksi kasus yang digabungkan dari keadaan git saja: cabang remote yang didorong worktree tidak lagi ada, dan setiap commit dalam worktree sudah berada di cabang default.

Dalam setiap kasus lain, Claude Code membuka kembali worktree di ujung lamanya:

* Worktree gagal salah satu kondisi.
* Claude Code tidak dapat memverifikasi keadaan worktree.
* `worktree.baseRef` adalah `"head"`.
* Nama adalah referensi pull request atau merge request.

Sebelum v2.1.208, saat Anda menggunakan kembali nama, Claude Code selalu membuka kembali worktree lama di ujung lamanya.

<h3 id="replace-worktree-creation-with-a-hook">
  Ganti pembuatan worktree dengan hook
</h3>

Konfigurasikan hook [`WorktreeCreate`](/docs/id/hooks#worktreecreate) untuk menggantikan logika `git worktree` default sepenuhnya, termasuk menempatkan worktrees di tempat lain selain `.claude/worktrees/`. Untuk contoh lengkap, lihat [Non-git version control](#non-git-version-control).

<h2 id="what-worktrees-share-with-the-main-checkout">
  Apa yang dibagikan worktrees dengan checkout utama
</h2>

Worktree mendapatkan file dan cabang sendiri, tetapi berbagi hal-hal berikut dengan checkout utama:

* **Direktori `.git` repositori**: perintah git dalam worktree menulis ke direktori `.git` bersama repositori utama, dan [sandboxing](/docs/id/sandboxing#filesystem-isolation) memungkinkan penulisan tersebut, jadi perintah seperti `git commit` bekerja dari dalam worktree dengan sandbox diaktifkan.
* **Plugins**: plugin yang diinstal pada [cakupan proyek](/docs/id/plugins/loading#find-where-a-plugin-is-enabled) dari checkout utama juga dimuat dalam worktrees dari repositori yang sama, jadi Anda tidak perlu menginstal ulang per worktree. Memerlukan Claude Code v2.1.200 atau lebih baru.
* **Persetujuan izin**: memilih "Ya, dan jangan tanya lagi" untuk perintah Bash dalam sesi worktree menyimpan aturan ke `.claude/settings.local.json` checkout utama, jadi itu berlaku dalam checkout utama dan di setiap worktree lain dari repositori, dan itu bertahan penghapusan worktree. Di Windows dan dalam kasus lain di mana Claude Code [tidak menggunakan root repositori](/docs/id/settings#where-claude-code-looks-for-each-file), aturan tetap dengan worktree itu. Sebelum v2.1.211, persetujuan yang diberikan dalam worktree disimpan di dalam worktree itu, tidak berlaku di tempat lain, dan hilang saat worktree dihapus. Lihat [di mana persetujuan disimpan](/docs/id/permissions#permission-system).
* **Skills, agents, dan commands yang tidak dilacak**: ketika checkout worktree tidak memiliki direktori `.claude/skills` di akarnya, misalnya karena `.claude/skills` Anda diabaikan oleh git, Claude Code memuat [project skills](/docs/id/skills#where-skills-live) checkout utama dalam sesi worktree. Dalam worktree dengan direktori `.claude/skills` sendiri, hanya salinan itu yang dimuat.

  Pembacaan yang sama mencakup `.claude/agents` dan `.claude/commands`. Untuk skills, pembacaan memerlukan Claude Code v2.1.277 atau lebih baru.

Semua ini berlaku apakah Anda membuat worktree dengan `--worktree`, dengan `git worktree add`, atau melalui [aplikasi desktop](/docs/id/desktop#work-in-parallel-with-sessions).

<h2 id="manage-worktrees-manually">
  Kelola worktrees secara manual
</h2>

Buat worktrees dengan Git secara langsung saat Anda perlu checkout cabang yang ada tertentu atau menempatkan worktree di luar repositori.

Buat worktree pada cabang baru:

```bash theme={null}
git worktree add ../project-feature-a -b feature-a
```

Buat worktree dari cabang yang ada, ganti `fix-issue-456` dengan cabang yang sudah ada dalam repositori Anda:

```bash theme={null}
git worktree add ../project-bugfix fix-issue-456
```

Mulai Claude dalam worktree:

```bash theme={null}
cd ../project-feature-a
claude
```

Daftar worktrees Anda:

```bash theme={null}
git worktree list
```

Hapus satu saat Anda selesai dengannya:

```bash theme={null}
git worktree remove ../project-feature-a
```

Lihat [dokumentasi Git worktree](https://git-scm.com/docs/git-worktree) untuk referensi perintah lengkap.

<h2 id="non-git-version-control">
  Non-git version control
</h2>

Isolasi worktree menggunakan git secara default. Untuk SVN, Perforce, Mercurial, atau sistem lainnya, konfigurasikan hook [`WorktreeCreate` dan `WorktreeRemove`](/docs/id/hooks#worktreecreate) untuk menyediakan logika pembuatan dan pembersihan kustom. Karena hook menggantikan perilaku git default, [`.worktreeinclude`](#copy-gitignored-files-into-worktrees) tidak diproses saat Anda menggunakan `--worktree`. Salin file konfigurasi lokal apa pun di dalam skrip hook Anda.

Hook `WorktreeCreate` ini membaca nama worktree dari JSON pada stdin dengan `jq`, checkout salinan kerja SVN segar, dan mencetak jalur direktori sehingga Claude Code dapat menggunakannya sebagai direktori kerja sesi. Tambahkan konfigurasi ke [`settings.json`](/docs/id/settings#where-settings-live) Anda:

```json theme={null}
{
  "hooks": {
    "WorktreeCreate": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "bash -c 'NAME=$(jq -r .name); DIR=\"$HOME/.claude/worktrees/$NAME\"; svn checkout https://svn.example.com/repo/trunk \"$DIR\" >&2 && echo \"$DIR\"'"
          }
        ]
      }
    ]
  }
}
```

Pasangkan dengan hook `WorktreeRemove` untuk membersihkan saat sesi berakhir. Lihat [referensi hooks](/docs/id/hooks#worktreecreate) untuk skema input dan contoh penghapusan.

Hook `WorktreeCreate` juga memungkinkan Anda menjalankan [`/batch`](/docs/id/commands#all-commands) di luar repositori git. Setiap subagent `/batch` kemudian menerbitkan perubahannya dengan perintah kontrol versi proyek Anda dan, ketika tidak dapat membuka pull request, melaporkan apa yang diterbitkannya. Menjalankan `/batch` di luar repositori git memerlukan Claude Code v2.1.281 atau lebih baru.

<h2 id="troubleshooting">
  Troubleshooting
</h2>

Claude Code melaporkan kesalahan di bawah saat membuat worktree, memasuki satu saat startup, atau mengembalikan sesi yang dilanjutkan ke satu.

<h3 id="claude-code-can’t-enter-the-worktree-at-startup">
  Claude Code tidak dapat memasuki worktree saat startup
</h3>

Saat Claude Code tidak dapat memasuki direktori worktree saat startup, itu mencetak kesalahan yang menamai jalur dan keluar dengan kode 1. Ini dapat terjadi saat hook [`WorktreeCreate`](/docs/id/hooks#worktreecreate) mencetak sesuatu selain direktori yang dibuat, atau saat direktori dihapus setelah diatur.

<h3 id="worktree-creation-fails-on-a-symlinked-path">
  Pembuatan worktree gagal pada jalur symlinked
</h3>

Claude Code menolak untuk membuat worktree saat `.claude`, `.claude/worktrees`, atau direktori worktree itu sendiri adalah symlink, dan kesalahan menamai jalur symlinked. Hapus symlink dan coba lagi. Sebelum v2.1.212, jika repositori sudah berisi symlink yang dikomit di salah satu jalur itu, pembuatan worktree mengikutinya dan dapat membuat file di luar repositori.

<h3 id="git-lfs-content-is-missing-from-a-worktree-claude-code-created">
  File Git LFS adalah file pointer dalam worktree yang dibuat Claude Code
</h3>

Jika Anda menyiapkan [Git LFS](https://git-lfs.com) dengan `git lfs install --local`, worktree yang dibuat Claude Code berisi file pointer LFS daripada file nyata. Flag `--local` menulis filter LFS ke `.git/config` repositori itu sendiri daripada konfigurasi git global Anda. `git lfs install` biasa menulis ke konfigurasi global Anda dan tidak terpengaruh. Hal yang sama berlaku untuk [driver filter](https://git-scm.com/docs/gitattributes) lainnya yang didefinisikan dalam konfigurasi repositori itu sendiri.

Claude Code melewati driver filter repositori itu sendiri saat membuat worktree karena driver filter adalah perintah shell, dan apa pun yang dapat menulis ke repositori, termasuk Claude, dapat menempatkan satu di sana. Sebelum v2.1.247, Claude Code menjalankan driver tersebut selama pembuatan worktree.

Untuk mendapatkan file nyata, jalankan `git lfs pull` di dalam worktree.

Dalam empat kasus langka, Claude Code tidak membuat worktree sama sekali: itu tidak dapat mengatakan driver filter mana yang didefinisikan konfigurasi repositori, atau itu menemukan pengaturan di sana yang tidak dapat dimatikan. Cocokkan kesalahan dengan perbaikannya:

* **`Could not read the repository git config to neutralize filter drivers`**: Claude Code tidak dapat membaca `.git/config` repositori, misalnya karena izinnya. Perbaiki itu dan coba lagi.
* **`The repository git config defines a filter driver whose name cannot be neutralized (contains "=" or a newline)`**: ganti nama atau hapus driver filter itu dalam `.git/config` dan coba lagi.
* **`The repository git config has a conditional include (includeIf)`**: pindahkan pengaturan yang `includeIf` dalam `.git/config` tarik langsung ke dalam file itu, hapus `includeIf`, dan coba lagi. `includeIf` dalam konfigurasi git global Anda tidak memicu ini.
* **`Git was not run: the repository's own git config sets <key>`**: pesan menamai kunci yang menunjuk Git LFS ke program untuk dijalankan, seperti `lfs.customtransfer.<name>.path` atau `lfs.standalonetransferagent`. Jika pengaturan itu milik Anda, pindahkan ke konfigurasi git global Anda. Jika Anda tidak mengenalinya, hapus dari konfigurasi git repositori, karena alat atau checkout yang Anda tidak percayai mungkin telah menulisnya. Coba lagi setelah kunci hilang dari konfigurasi repositori.

<h3 id="claude-code-refuses-to-use-a-worktree">
  Claude Code menolak untuk menggunakan worktree
</h3>

Kesalahan yang dimulai dengan `Refusing to use <path> as an isolation worktree` berarti Claude Code memeriksa identitas git direktori sebelum mengadopsinya sebagai checkout terisolasi sesi atau subagent, dan menolaknya. Pemeriksaan berjalan apakah Claude Code membuat worktree, memasuki yang ada, atau menggunakan kembali dari jalankan sebelumnya.

Dalam sebagian besar kasus sisa pesan mengatakan metadata git direktori diselesaikan ke checkout utama: misalnya, file `.git`-nya menunjuk ke direktori `.git` repositori itu sendiri, atau git menyelesaikan pohon kerjanya ke checkout utama melalui pengalihan `core.worktree`. Dari direktori seperti itu, perintah git biasa seperti `git reset --hard` akan bertindak pada checkout utama daripada worktree. Claude Code juga menolak saat direktori memiliki entri `.git` yang tidak dapat dibaca, daripada menganggap worktree aman.

Direktori dengan tidak ada metadata git sama sekali, seperti yang dibuat hook [`WorktreeCreate`](#non-git-version-control) Anda, lulus pemeriksaan hanya saat tidak ada repositori git yang berisi itu. Jika hook membuat direktori di dalam repositori, git menyelesaikannya ke checkout repositori itu dan Claude Code menolaknya dengan pesan `git resolves its working tree to`, jadi buat hook membuat direktorinya di luar repositori apa pun.

Claude Code meninggalkan direktori yang ditolak di tempat, karena mungkin menyimpan pekerjaan. Cocokkan pesan dengan pemulihan-nya, apakah itu mengikuti `Refusing to use <path>` atau muncul dalam [pesan resume](#the-session-resumes-outside-its-worktree); beberapa akhiran hanya terjadi dalam pesan resume:

* **Mengatakan `launch from the parent checkout` atau `Run the resume from the project checkout`**: Anda meluncurkan Claude Code dari dalam worktree. Luncurkan dari checkout utama; worktree tidak memerlukan rekreasi.
* **Mengatakan `it cannot be resumed or re-entered`**: tidak ada dalam sesi ini yang menjamin worktree dari tempat Anda meluncurkan. Buat ulang; direktori dan pekerjaan-nya tetap berada di disk untuk pemulihan manual, dan saat worktree memiliki checkout induk, melanjutkan dari sana juga berfungsi.
* **Mengatakan `it contains the protected checkout`**: direktori yang ditolak adalah induk checkout utama Anda, seperti direktori home Anda. Jangan hapus. Ubah jalur worktree, seperti jalur yang dikembalikan hook `WorktreeCreate` Anda atau target `EnterWorktree`, sehingga worktree tidak berisi checkout.
* **Mengatakan `the protected checkout <path> has a .git entry that could not be examined` atau `has git metadata that could not be resolved`**: masalahnya adalah metadata git checkout utama, bukan worktree. Jangan hapus worktree, dan abaikan saran pesan untuk membuat ulangnya, yang tidak berlaku untuk dua akhiran ini. Perbaiki checkout utama, misalnya masalah izin atau penolakan git `dubious ownership` pada `.git`-nya, dan coba lagi.
* **Mengatakan `its recorded path has a network spelling`**: Claude Code tidak pernah melanjutkan ke worktree di jalur jaringan. Buat ulang worktree di jalur lokal.
* **Akhiran lainnya**: pesan menamai masalah dan perbaikannya, seperti menghapus pengalihan `core.worktree` atau membuat ulang worktree; ikuti. Sebelum menghapus direktori yang pesannya mengatakan identitas git tidak dapat diverifikasi, tangani penyebab yang dinamai terlebih dahulu, misalnya symlink dalam jalur worktree atau git itu sendiri gagal berjalan, karena direktori mungkin sehat. Saat Anda membuat ulang, selamatkan perubahan apa pun yang Anda butuhkan dari direktori lama terlebih dahulu; itu tetap berada di disk.

<h3 id="the-session-resumes-outside-its-worktree">
  Sesi melanjutkan di luar worktree-nya
</h3>

Saat Anda melanjutkan sesi secara interaktif dan Claude Code tidak dapat mengembalikannya ke worktree-nya, Claude Code mengatakan demikian dengan salah satu pesan di bawah. Saat Claude Code menghapus pengikatan worktree, itu mencatat penghapusan dalam transkrip sesi. Jika Anda [menekan penulisan transkrip](/docs/id/sessions#where-transcripts-are-stored), pesan mengatakan sebagai gantinya bahwa pengikatan tidak dapat dihapus dan bahwa Claude Code akan memeriksa kembali worktree pada resume yang lebih baru.

| Pesan dimulai dengan                              | Apa yang terjadi dan apa yang harus dilakukan                                                                                                                                                                                                                                                                                                                                                                                               |
| :------------------------------------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `Your worktree <path> no longer exists`           | Direktori worktree dihapus. Sesi berlanjut di direktori saat ini tanpa isolasi, dan Claude Code menghapus pengikatan worktree. Tidak ada tindakan yang diperlukan.                                                                                                                                                                                                                                                                          |
| `Could not verify your worktree <path> this time` | Claude Code tidak dapat memverifikasi worktree, biasanya karena alasan sementara; pengikatan disimpan, dan sesi berlanjut di direktori saat ini tanpa isolasi. Lanjutkan lagi untuk mencoba ulang; jika terus terjadi, masuki worktree dalam sesi baru dan cocokkan pesan penolakan di bawah [Claude Code refuses to use a worktree](#claude-code-refuses-to-use-a-worktree), yang dapat menamai metadata checkout utama daripada worktree. |
| `Did not re-enter your worktree <path>`           | Claude Code menolak pengikatan worktree sebagai tidak aman; itu menghapus pengikatan dan sesi berlanjut tanpa isolasi. Pesan mencakup penolakan spesifik: cocokkan di bawah [Claude Code refuses to use a worktree](#claude-code-refuses-to-use-a-worktree), karena perbaikannya adalah rekreasi untuk beberapa penolakan dan perubahan jalur untuk yang lain.                                                                              |
| `Could not re-enter your worktree <path>`         | Claude Code tidak dapat menjamin worktree dari tempat Anda meluncurkan, paling umum karena Anda meluncurkan dari dalamnya; pengikatan disimpan. Sisa pesan menamai perbaikannya; cocokkan di bawah [Claude Code refuses to use a worktree](#claude-code-refuses-to-use-a-worktree).                                                                                                                                                         |

Dalam [mode non-interaktif](/docs/id/headless) dengan `-p`, dan pada resume yang [Agent SDK](/docs/id/agent-sdk/sessions) jalankan, Claude Code menghentikan resume dengan kesalahan stderr untuk setiap penolakan kecuali worktree yang hilang, daripada melanjutkan tanpa isolasi.

Dengan `--output-format stream-json`, penolakan juga tiba di stdout sebagai pesan `result` dengan subtipe `error_during_execution` yang array `errors`-nya membawa teks yang sama, jadi aplikasi Agent SDK menerima alasan daripada hanya exit non-nol. Sebelum v2.1.260, penolakan resume worktree tidak menghasilkan pesan `result`.

Pesan mengambil bentuk berbeda dari pesan interaktif dalam tabel:

* `Error: cannot resume into worktree <path>: ...This session was not started.` untuk penolakan yang ditunjukkan tabel sebagai `Did not re-enter`. Claude Code menghapus pengikatan worktree sebelum keluar, dan kesalahan mengatakan demikian; saat berikutnya Anda melanjutkan percakapan, sesi berlanjut di direktori saat ini tanpa isolasi worktree. Sebelum v2.1.260, Claude Code tidak menulis pengikatan yang dihapus, jadi setiap percobaan ulang resume yang sama gagal dengan kesalahan yang sama.

  Jika Anda [menekan penulisan transkrip](/docs/id/sessions#where-transcripts-are-stored), penghapusan tidak dapat disimpan. Kesalahan kemudian mengatakan perintah yang sama akan ditolak lagi, dan menamai `--fork-session` dan memulai percakapan baru sebagai cara untuk melanjutkan tanpa worktree.
* `Error: could not verify worktree <path> for this resume, so the resume was aborted...` untuk `Could not verify`
* `Error: ...The worktree binding is kept.` untuk `Could not re-enter`
* `Notice: the worktree <path> for this session no longer exists...` untuk worktree yang hilang; Claude Code mencetaknya dan melanjutkan sesi, seperti resume interaktif

Akhiran penolakan yang tertanam dalam setiap kesalahan dibagikan dengan pemberitahuan interaktif, jadi itu masih cocok dengan entri-nya di bawah [Claude Code refuses to use a worktree](#claude-code-refuses-to-use-a-worktree).

Dalam hasil stream-json, [`startup_failure_reason`](/docs/id/agent-sdk/typescript#startup_failure_reason) adalah `worktree_unverified` untuk kesalahan `could not verify worktree` dan `worktree_resume_refused` untuk kesalahan `cannot resume into worktree` dan `The worktree binding is kept`. Aplikasi dapat membuat cabang di atasnya daripada mencocokkan teks kesalahan. Sebelum v2.1.274, hasil tidak membawa field `startup_failure_reason`.

<h2 id="see-also">
  Lihat juga
</h2>

Worktrees menangani isolasi file. Halaman terkait di bawah mencakup pendelegasian pekerjaan ke checkout terisolasi itu, melewatkan temuan di antara mereka, dan beralih antar sesi yang Anda buat:

* [Subagents](/docs/id/sub-agents): delegasikan pekerjaan ke agen terisolasi dalam sesi
* [Cross-session messaging](/docs/id/cross-session-messaging): biarkan sesi dalam worktrees Anda melewatkan temuan satu sama lain
* [Agent teams](/docs/id/agent-teams): koordinasikan beberapa sesi Claude secara otomatis
* [Manage sessions](/docs/id/sessions): beri nama, lanjutkan, dan beralih antar percakapan
* [Desktop parallel sessions](/docs/id/desktop#work-in-parallel-with-sessions): sesi yang didukung worktree dalam aplikasi desktop
