> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Jalankan sesi paralel dengan worktrees

> Isolasi sesi Claude Code paralel dalam git worktrees terpisah sehingga perubahan tidak bertabrakan. Mencakup flag `--worktree`, isolasi subagent, `.worktreeinclude`, pembersihan, dan hook VCS non-git.

[git worktree](https://git-scm.com/docs/git-worktree) adalah direktori kerja terpisah dengan file dan cabang sendiri, berbagi riwayat repositori dan remote yang sama dengan checkout utama Anda. Menjalankan setiap sesi Claude Code dalam worktree-nya sendiri berarti edit dalam satu sesi tidak akan pernah menyentuh file di sesi lain, sehingga Anda dapat memiliki Claude membangun fitur di satu terminal sambil memperbaiki bug di terminal kedua.

Halaman ini mencakup isolasi worktree di CLI. Semuanya di bawah mengasumsikan repositori git. Untuk sistem kontrol versi lainnya, lihat [Non-git version control](#non-git-version-control). [Aplikasi desktop](/docs/id/desktop#work-in-parallel-with-sessions) membuat worktree untuk setiap sesi baru secara otomatis.

Worktrees adalah salah satu dari beberapa cara untuk menjalankan Claude secara paralel. Mereka mengisolasi edit file, sementara [subagents](/docs/id/sub-agents) dan [agent teams](/docs/id/agent-teams) mengoordinasikan pekerjaan itu sendiri. Lihat [Run agents in parallel](/docs/id/agents) untuk membandingkan pendekatan, atau lompat ke [Isolate subagents with worktrees](#isolate-subagents-with-worktrees) untuk menggunakan worktrees dan subagents bersama-sama.

<h2 id="start-claude-in-a-worktree">
  Mulai Claude dalam worktree
</h2>

Lewatkan `--worktree` atau `-w` untuk membuat worktree terisolasi dan memulai Claude di dalamnya. Secara default, worktree dibuat di bawah `.claude/worktrees/<value>/` di root repositori Anda, pada cabang baru bernama `worktree-<value>`:

```bash theme={null}
claude --worktree feature-auth
```

Untuk menempatkan worktrees di tempat lain, konfigurasikan hook [`WorktreeCreate`](#non-git-version-control). Jalankan perintah lagi dengan nama berbeda di terminal lain untuk memulai sesi terisolasi kedua:

```bash theme={null}
claude --worktree bugfix-123
```

Jika Anda menghilangkan nama, Claude menghasilkan satu seperti `bright-running-fox`:

```bash theme={null}
claude --worktree
```

Anda juga dapat meminta Claude untuk "bekerja dalam worktree" selama sesi, dan itu akan membuat satu dengan tool [`EnterWorktree`](/docs/id/tools-reference). Setelah berada dalam worktree, Claude dapat beralih langsung ke worktree lain di bawah `.claude/worktrees/` dengan memanggil `EnterWorktree` dengan jalur target. Worktree sebelumnya tetap berada di disk tanpa disentuh.

Memasuki jalur di luar direktori `.claude/worktrees/` repositori meminta persetujuan Anda terlebih dahulu, karena itu memindahkan direktori kerja sesi, akses tulis, dan konfigurasi proyek seperti `CLAUDE.md` dan settings ke lokasi tersebut. Aturan [izin](/docs/id/permissions) `EnterWorktree` atau memilih "jangan tanya lagi" tidak menekan prompt ini; hanya mode `bypassPermissions` yang melewatinya. Sebelum v2.1.206, Claude dapat memasuki jalur worktree yang ada tanpa bertanya.

Mulai dari v2.1.198, memasuki atau keluar dari worktree juga memindahkan transkrip sesi ke penyimpanan proyek direktori tersebut, dengan cara yang sama seperti [`/cd`](/docs/id/commands) melakukannya, sehingga `/desktop` dan `--resume` menemukan sesi di sana setelahnya. Worktrees yang dibuat oleh hook [`WorktreeCreate`](#non-git-version-control) dikecualikan dan menyimpan transkrip di direktori peluncuran.

Worktrees bekerja dengan [sandboxing](/docs/id/sandboxing#filesystem-isolation) diaktifkan: sandbox memungkinkan penulisan ke direktori `.git` bersama repositori utama sehingga perintah seperti `git commit` dapat memperbarui refs dan indeks dari dalam linked worktree.

Sebelum menggunakan `--worktree` secara interaktif di direktori untuk pertama kalinya, terima dialog kepercayaan workspace dengan menjalankan `claude` sekali di direktori tersebut. Jika kepercayaan belum diterima, `--worktree` keluar dengan kesalahan dan meminta Anda untuk menjalankan `claude` di direktori terlebih dahulu. Jalankan non-interaktif dengan `-p` melewati [pemeriksaan kepercayaan](/docs/id/security), jadi `claude -p --worktree` melanjutkan tanpanya.

Jika Claude Code tidak dapat memasuki direktori worktree saat startup, misalnya karena hook [`WorktreeCreate`](/docs/id/hooks#worktreecreate) mencetak sesuatu selain direktori yang dibuat, atau karena direktori dihapus setelah diatur, Claude Code mencetak kesalahan yang menamai jalur dan keluar dengan kode 1. Sebelum v2.1.205, ini menghancurkan sesi, dan dengan `-p` itu terhenti selama sekitar 30 detik sebelum keluar dengan kode 0.

Plugin yang diinstal pada [cakupan proyek](/docs/id/plugins-reference#plugin-installation-scopes) dari checkout utama juga dimuat dalam worktrees dari repositori yang sama, sehingga Anda tidak perlu menginstal ulang per worktree. Ini berlaku apakah Anda membuat worktree dengan `--worktree` atau dengan `git worktree add`. Memerlukan Claude Code v2.1.200 atau lebih baru.

<Tip>
  Tambahkan `.claude/worktrees/` ke `.gitignore` Anda sehingga konten worktree tidak muncul sebagai file yang tidak dilacak dalam checkout utama Anda.
</Tip>

<h3 id="choose-the-base-branch">
  Pilih cabang dasar
</h3>

Worktrees bercabang dari cabang default repositori Anda, `origin/HEAD`, sehingga mereka dimulai dari pohon bersih yang cocok dengan remote. Ketika tidak ada yang mengambil repositori dalam 24 jam terakhir, Claude Code menyegarkan `origin/HEAD` dengan pengambilan cabang default, dibatasi pada lima detik, dan menggunakan ref yang disimpan secara lokal jika pengambilan gagal. Jika tidak ada remote yang dikonfigurasi, atau `origin/HEAD` tidak disimpan secara lokal dan tidak dapat diambil, worktree kembali ke `HEAD` lokal Anda saat ini.

Penyegaran memerlukan Claude Code v2.1.208 atau lebih baru; sebelum itu, worktree baru menggunakan apa pun yang sudah disimpan secara lokal di `origin/HEAD`.

Untuk selalu bercabang dari `HEAD` lokal, atur `worktree.baseRef` ke `"head"` dalam [settings](/docs/id/settings#worktree-settings). Mengatur `baseRef` ke `"head"` membuat worktree baru membawa commit yang belum didorong dan status cabang fitur Anda, yang berguna saat mengisolasi subagent yang perlu beroperasi pada pekerjaan yang sedang berlangsung. Ketika sesi berjalan di dalam linked worktree, `"head"` diselesaikan ke `HEAD` worktree tersebut, bukan checkout utama. Pengaturan hanya menerima `"fresh"` atau `"head"`, bukan git refs arbitrer:

```json theme={null}
{
  "worktree": {
    "baseRef": "head"
  }
}
```

Untuk bercabang dari pull request tertentu, lewatkan nomor PR dengan awalan `#`, atau URL pull request GitHub lengkap. Claude Code mengambil `pull/<number>/head` dari `origin` dan membuat worktree di `.claude/worktrees/pr-<number>`:

```bash theme={null}
claude --worktree "#1234"
```

Untuk kontrol penuh atas cara pembuatan worktrees, konfigurasikan hook [`WorktreeCreate`](/docs/id/hooks#worktreecreate), yang menggantikan logika `git worktree` default sepenuhnya.

<h3 id="reuse-a-worktree-name">
  Gunakan kembali nama worktree
</h3>

Menggunakan kembali nama worktree yang direktorinya sudah ada melanjutkan worktree tersebut.

Worktree yang dilanjutkan disetel ulang ke [base saat ini](#choose-the-base-branch) alih-alih melanjutkan di ujung lamanya ketika semua hal berikut berlaku:

* Tidak memiliki perubahan yang belum dikomit atau file yang tidak dilacak.
* Masih berada di cabang yang dibuat Claude Code untuk itu.
* Tidak pernah berkomit, atau pull request-nya digabungkan dan cabang remote-nya dihapus.

Sebelum v2.1.208, nama yang digunakan kembali selalu melanjutkan worktree lama di ujung lamanya.

<h2 id="copy-gitignored-files-into-worktrees">
  Salin file yang diabaikan git ke dalam worktrees
</h2>

Worktree adalah checkout segar, jadi file yang tidak dilacak seperti `.env` atau `.env.local` dari repositori utama Anda tidak ada. Untuk menyalinnya secara otomatis saat Claude membuat worktree, tambahkan file `.worktreeinclude` ke root proyek Anda.

File menggunakan sintaks `.gitignore`. Hanya file yang cocok dengan pola dan juga diabaikan git yang disalin, sehingga file yang dilacak tidak pernah diduplikasi.

`.worktreeinclude` ini menyalin dua file env dan konfigurasi rahasia ke setiap worktree baru:

```text .worktreeinclude theme={null}
.env
.env.local
config/secrets.json
```

Ini berlaku untuk worktrees yang dibuat dengan `--worktree`, [subagent worktrees](#isolate-subagents-with-worktrees), dan sesi paralel dalam [aplikasi desktop](/docs/id/desktop#work-in-parallel-with-sessions).

<h2 id="isolate-subagents-with-worktrees">
  Isolasi subagents dengan worktrees
</h2>

Subagents dapat berjalan dalam worktrees mereka sendiri sehingga edit paralel tidak bertabrakan. Minta Claude untuk "gunakan worktrees untuk agen Anda", atau atur secara permanen pada [subagent kustom](/docs/id/sub-agents#supported-frontmatter-fields) dengan menambahkan `isolation: worktree` ke frontmatter. Setiap subagent mendapatkan worktree sementara yang dihapus secara otomatis saat subagent selesai tanpa perubahan.

Worktrees subagent menggunakan [base branch](#choose-the-base-branch) yang sama dengan `--worktree`, jadi mereka membuat cabang dari branch default repositori Anda kecuali `worktree.baseRef` diatur ke `"head"`.

<h2 id="clean-up-worktrees">
  Bersihkan worktrees
</h2>

Saat Anda keluar dari sesi worktree, pembersihan tergantung pada apakah Anda membuat perubahan:

* **Tidak ada perubahan yang belum dilakukan, tidak ada file yang tidak dilacak, dan tidak ada commit baru**: worktree dan cabangnya dihapus secara otomatis. Jika sesi memiliki [nama](/docs/id/sessions#name-your-sessions), Claude malah meminta sehingga Anda dapat menyimpan worktree untuk nanti
* **Perubahan yang belum dilakukan, file yang tidak dilacak, atau commit baru ada**: Claude meminta Anda untuk menyimpan atau menghapus worktree. Menyimpan mempertahankan direktori dan cabang sehingga Anda dapat kembali nanti. Menghapus menghapus direktori worktree dan cabangnya, membuang semua perubahan yang belum dilakukan, file yang tidak dilacak, dan commit
* **Jalankan non-interaktif**: worktrees yang dibuat dengan `--worktree` bersama dengan `-p` tidak dibersihkan secara otomatis karena tidak ada prompt keluar. Hapus dengan `git worktree remove`

Worktrees yang Claude buat untuk subagent dan [sesi latar belakang](/docs/id/agent-view#how-file-edits-are-isolated) dihapus secara otomatis setelah mereka lebih tua dari pengaturan [`cleanupPeriodDays`](/docs/id/settings#available-settings) Anda, asalkan mereka tidak memiliki perubahan yang belum dilakukan, tidak ada file yang tidak dilacak, dan tidak ada commit yang belum didorong. Worktrees yang Anda buat dengan `--worktree` tidak pernah dihapus oleh sapuan ini.

Saat agen sedang berjalan, Claude menjalankan `git worktree lock` pada worktreenya sehingga pembersihan bersamaan tidak dapat menghapusnya. Kunci dilepaskan ketika agen selesai. Untuk membersihkan worktree yang disimpan oleh sapuan, jalankan `git worktree remove`, tambahkan `--force` jika worktree memiliki perubahan yang belum dilakukan atau file yang tidak dilacak.

Di Windows, sebelum menghapus worktree, Claude Code menghapus sambungan NTFS atau symlink direktori apa pun pada kedalaman apa pun di dalamnya sebagai entri tautan, sehingga menghapus worktree tidak menghapus file yang ditunjuk tautan. Sebelum v2.1.205, Claude Code hanya menghapus tautan tingkat atas sebagai entri tautan, dan menghapus worktree dengan sambungan yang bersarang di subdirektori dapat menghapus isi direktori yang ditunjuk tautan di luar worktree.

<h2 id="manage-worktrees-manually">
  Kelola worktrees secara manual
</h2>

Untuk kontrol penuh atas lokasi worktree dan konfigurasi cabang, buat worktrees dengan Git secara langsung. Ini berguna saat Anda perlu checkout cabang yang ada tertentu atau menempatkan worktree di luar repositori.

Buat worktree pada cabang baru:

```bash theme={null}
git worktree add ../project-feature-a -b feature-a
```

Buat worktree dari cabang yang ada:

```bash theme={null}
git worktree add ../project-bugfix bugfix-123
```

Mulai Claude dalam worktree:

```bash theme={null}
cd ../project-feature-a && claude
```

Daftar worktrees Anda:

```bash theme={null}
git worktree list
```

Hapus satu saat Anda selesai dengannya:

```bash theme={null}
git worktree remove ../project-feature-a
```

Lihat [dokumentasi Git worktree](https://git-scm.com/docs/git-worktree) untuk referensi perintah lengkap. Ingat untuk menginisialisasi lingkungan pengembangan Anda di setiap worktree baru: instal dependensi, atur lingkungan virtual, atau jalankan apa pun yang diperlukan setup proyek Anda.

<h2 id="non-git-version-control">
  Non-git version control
</h2>

Isolasi worktree menggunakan git secara default. Untuk SVN, Perforce, Mercurial, atau sistem lainnya, konfigurasikan hook [`WorktreeCreate` dan `WorktreeRemove`](/docs/id/hooks#worktreecreate) untuk menyediakan logika pembuatan dan pembersihan kustom. Karena hook menggantikan perilaku git default, [`.worktreeinclude`](#copy-gitignored-files-into-worktrees) tidak diproses saat Anda menggunakan `--worktree`. Salin file konfigurasi lokal apa pun di dalam skrip hook Anda.

Hook `WorktreeCreate` ini membaca nama worktree dari stdin, checkout salinan kerja SVN segar, dan mencetak jalur direktori sehingga Claude Code dapat menggunakannya sebagai direktori kerja sesi:

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

<h2 id="see-also">
  Lihat juga
</h2>

Worktrees menangani isolasi file. Halaman terkait di bawah mencakup pendelegasian pekerjaan ke checkout terisolasi tersebut dan beralih antar sesi yang Anda buat:

* [Subagents](/docs/id/sub-agents): delegasikan pekerjaan ke agen terisolasi dalam sesi
* [Agent teams](/docs/id/agent-teams): koordinasikan beberapa sesi Claude secara otomatis
* [Manage sessions](/docs/id/sessions): beri nama, lanjutkan, dan beralih antar percakapan
* [Desktop parallel sessions](/docs/id/desktop#work-in-parallel-with-sessions): sesi yang didukung worktree dalam aplikasi desktop
