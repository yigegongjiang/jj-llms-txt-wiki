> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Siapkan Claude Code di monorepo atau codebase besar

> Konfigurasikan Claude Code untuk monorepos dan codebase pohon tunggal besar dengan file CLAUDE.md bersarang, worktrees sparse, code intelligence, dan skills per-paket sehingga Claude tetap fokus pada kode yang sedang Anda kerjakan.

Codebase besar dapat berupa satu repository dengan jutaan baris atau monorepo dengan banyak paket. Claude Code bekerja pada ukuran apa pun, tetapi seiring dengan pertumbuhan codebase, default yang disesuaikan untuk proyek yang lebih kecil dapat mengisi jendela konteks dengan instruksi dan pembacaan file yang tidak terkait dengan tugas, menghabiskan token dan menurunkan kinerja Claude.

Panduan ini menunjukkan kepada pengembang individual dan tim teknik bagaimana membatasi Claude ke bagian codebase yang disentuh oleh tugas. Setiap bagian mencatat apakah pengaturan bersifat pribadi untuk mesin Anda atau berkomitmen pada repository.

<h2 id="what-this-guide-covers">
  Apa yang panduan ini cakup
</h2>

[Tabel di bawah](#settings-on-this-page) mencantumkan setiap pengaturan dan apa yang dicapainya. [Pohon file setelahnya](#the-example-monorepo) adalah monorepo contoh yang dirujuk oleh setiap sampel kode di halaman ini.

<h3 id="settings-on-this-page">
  Settings on this page
</h3>

Setiap pengaturan di bawah ini independen. Mereka berlapis daripada menggantikan satu sama lain, jadi terapkan mana pun yang sesuai dengan repository Anda. [Pilih di mana memulai Claude](#choose-where-to-start-claude) menentukan di mana file pengaturan Anda berada, jadi baca terlebih dahulu. [Satukan semuanya](#put-it-together) menunjukkan semuanya digabungkan.

| Saya ingin                                                                                           | Gunakan                                                                                   |
| :--------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------- |
| Muat hanya konvensi untuk kode yang Anda sentuh, bukan satu file root yang mencakup setiap subsistem | File [CLAUDE.md](#layer-claude-md-files-by-directory) per-direktori                       |
| Kecualikan file CLAUDE.md untuk paket yang tidak pernah Anda kerjakan                                | [`claudeMdExcludes`](#exclude-irrelevant-claude-md-files)                                 |
| Blokir Claude dari membuka output build, kode yang dihasilkan, dan dependensi yang di-vendor         | Aturan deny [Read](#block-reads-of-generated-and-vendored-code) di `permissions.deny`     |
| Temukan definisi simbol atau pemanggil melalui language server bukan memindai file                   | Plugin [code intelligence](#reduce-file-reads-with-code-intelligence)                     |
| Periksa hanya direktori yang dibutuhkan tugas ketika Claude membuat worktree                         | [`worktree.sparsePaths`](#check-out-only-the-directories-you-need)                        |
| Baca dan edit paket sibling atau repository lain dari sesi yang sama                                 | [`--add-dir`](#grant-access-across-packages-or-repositories) atau `additionalDirectories` |
| Berikan Claude prosedur spesifik untuk satu area yang hanya dimuat ketika relevan                    | [Skills](#add-per-directory-skills) per-direktori                                         |
| Ganti banyak file CLAUDE.md per-direktori dengan satu set konvensi yang semua orang instal           | [Plugin](#centralize-conventions-when-layering-stops-scaling) di marketplace internal     |

<Tip>
  Untuk teknik alur kerja yang menjaga konteks tetap kecil di repository apa pun, seperti [menjalankan eksplorasi di subagent](/docs/id/best-practices#use-subagents-for-investigation) sehingga pembacaan file tetap keluar dari percakapan utama, lihat [Best practices untuk Claude Code](/docs/id/best-practices). Untuk meluncurkan konfigurasi baseline ke setiap pengembang di organisasi Anda, lihat [Siapkan Claude Code untuk organisasi Anda](/docs/id/admin-setup).
</Tip>

<h3 id="the-example-monorepo">
  The example monorepo
</h3>

Contoh di seluruh halaman ini merujuk pada monorepo dengan tiga paket. Pola yang sama bekerja di codebase pohon tunggal besar: di mana contoh menggunakan `packages/api/`, gantikan dengan direktori subsistem Anda sendiri seperti `src/backend/` atau `lib/core/`.

```text theme={null}
monorepo/
  CLAUDE.md                     # instruksi root
  packages/
    api/
      CLAUDE.md                 # instruksi spesifik API
      .claude/skills/
      src/
    web/
      CLAUDE.md                 # instruksi spesifik frontend
      .claude/skills/
      src/
    shared/
      CLAUDE.md                 # instruksi perpustakaan bersama
      src/
```

<h2 id="choose-where-to-start-claude">
  Pilih di mana memulai Claude
</h2>

Di mana Anda meluncurkan `claude` menentukan file mana yang dapat dibaca dan diedit Claude tanpa hibah izin tambahan, file CLAUDE.md mana yang dimuat ke konteks saat startup, dan pengaturan proyek mana yang berlaku.

| Mulai dari      | Akses file                                            | CLAUDE.md dimuat saat peluncuran                                                     | Gunakan ketika                                    |
| :-------------- | :---------------------------------------------------- | :----------------------------------------------------------------------------------- | :------------------------------------------------ |
| Repository root | Setiap file                                           | Root hanya; file subdirektori dimuat sesuai permintaan ketika Claude membaca di sana | Tugas mencakup beberapa paket atau subsistem      |
| Subdirektori    | Subtree itu saja, sampai Anda memberikan lebih banyak | Direktori itu plus setiap ancestor                                                   | Pekerjaan dibatasi pada satu paket atau subsistem |

Pengaturan proyek di `.claude/settings.json` hanya dimuat dari direktori awal Anda dan tidak diwariskan dari direktori induk dengan cara file CLAUDE.md: `.claude/settings.json` di repository root hanya berlaku ketika Anda memulai dari root.

Setiap bagian di bawah menyatakan apakah file pengaturannya berada di repository root atau di subdirektori tempat Anda memulai, dan apakah itu berkomitmen atau disimpan secara lokal.

<h2 id="layer-claude-md-files-by-directory">
  Layer CLAUDE.md files by directory
</h2>

Dalam codebase besar, satu CLAUDE.md di repository root cenderung baik tumbuh untuk mencakup konvensi setiap subsistem, menghabiskan konteks pada instruksi yang tidak terkait dengan tugas saat ini, atau tetap terlalu umum untuk berguna. Membagi instruksi di seluruh file per-direktori berarti Claude memuat aturan repository-wide plus hanya konvensi untuk kode yang sedang Anda kerjakan.

Claude Code memuat setiap file [CLAUDE.md](/docs/id/memory) dari direktori kerja Anda dan setiap direktori induk saat peluncuran, kemudian memuat file setiap subdirektori sesuai permintaan ketika membaca file di sana. File root menetapkan aturan repository-wide dan setiap subdirektori menambahkan miliknya sendiri.

Pemisahan umum adalah dua level:

* **Root `CLAUDE.md`**: instruksi yang berlaku di mana-mana, seperti standar coding, konvensi commit, dan tata letak repository
* **Per-subdirektori `CLAUDE.md`**: konvensi spesifik untuk stack area itu. Dalam monorepo itu satu per paket. Dalam pohon tunggal besar itu satu per subsistem seperti `src/db/` atau `src/api/`

Komitkan file ini ke repository sehingga rekan kerja mewarisinya. Pemilik setiap direktori biasanya memelihara filenya.

Root `CLAUDE.md` mengorientasikan Claude ke struktur repository:

```markdown CLAUDE.md theme={null}
Ini adalah monorepo dengan tiga paket di bawah packages/:

- packages/api: Node.js REST API dengan Express, TypeScript, dan PostgreSQL
- packages/web: Frontend React dengan Vite, TypeScript, dan TailwindCSS
- packages/shared: utilitas TypeScript bersama yang digunakan oleh api dan web

Jalankan perintah dari direktori paket, bukan root monorepo.
Setiap paket memiliki tsconfig.json, package.json, dan test suite-nya sendiri.
```

`CLAUDE.md` setiap subdirektori, di sini `packages/api/CLAUDE.md`, menambahkan konteks spesifik untuk stack area itu:

```markdown packages/api/CLAUDE.md theme={null}
Paket ini adalah server REST API.

- Jalankan tes: `npm test` (menggunakan Vitest)
- Jalankan dev server: `npm run dev` (port 3001)
- Migrasi database: `npm run migrate`
- Variabel lingkungan: salin `.env.example` ke `.env`

Rute API berada di src/routes/. Setiap file rute mengekspor router Express.
Kueri database menggunakan Knex di src/db/. Jangan pernah menulis string SQL mentah di handler rute.
```

Ketika Anda memulai Claude dari `packages/api/`, itu memuat baik `packages/api/CLAUDE.md` dan root `CLAUDE.md`. Claude melihat instruksi lokal bersama aturan repository-wide, tanpa instruksi dari `packages/web/` dalam konteks. Hal yang sama berlaku untuk subdirektori apa pun dalam pohon non-monorepo.

Beberapa cara untuk menjaga file tetap terkini seiring dengan perubahan codebase dan model:

* **Tinjau dalam pull request**: perlakukan edit CLAUDE.md seperti perubahan dokumentasi lainnya sehingga konvensi melacak kode
* **Kunjungi kembali setelah rilis model utama**: instruksi yang mengatasi keterbatasan model yang lebih lama mungkin menjadi overhead setelah model yang lebih baru menangani kasus itu sendiri. Misalnya, aturan yang memaksa refactor file tunggal dapat dihapus setelah keterbatasan hilang
* **Tambahkan hook Stop yang mengusulkan pembaruan**: hook [`Stop`](/docs/id/hooks#stop) menerima jalur ke transkrip sesi ketika Claude selesai merespons, jadi skrip dapat meninjau sesi dan mengusulkan pembaruan CLAUDE.md sementara kesenjangan yang dieksposnya masih segar

Untuk lebih lanjut tentang bagaimana file CLAUDE.md dimuat dan berinteraksi, lihat [Memory dan project instructions](/docs/id/memory).

<h3 id="choose-between-per-directory-claude-md-and-path-scoped-rules">
  Pilih antara CLAUDE.md per-direktori dan aturan path-scoped
</h3>

File `CLAUDE.md` per-direktori dan [aturan path-scoped](/docs/id/memory#path-specific-rules) di bawah `.claude/rules/` keduanya memungkinkan Anda menargetkan instruksi ke bagian pohon. Mereka berbeda dalam di mana file berada dan kapan dimuat.

| Pendekatan                             | Lokasi file                         | Dimuat ketika                                                                                                | Gunakan ketika                                                                                      |
| :------------------------------------- | :---------------------------------- | :----------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------- |
| Per-direktori `CLAUDE.md`              | Di dalam direktori, bersama kodenya | Saat peluncuran ketika dimulai dari direktori itu, atau sesuai permintaan ketika Claude membaca file di sana | Pemilik direktori memelihara konvensi mereka sendiri; instruksi diversi dengan kode                 |
| Aturan path-scoped di `.claude/rules/` | `.claude/` pusat di repo root       | Ketika Claude bekerja dengan file yang cocok dengan glob `paths:` aturan                                     | Anda ingin semua konvensi di satu tempat, atau aturan yang sama berlaku untuk banyak jalur tersebar |

Untuk perbandingan yang juga mencakup skills, lihat [Bandingkan fitur serupa](/docs/id/features-overview#compare-similar-features).

<h3 id="exclude-irrelevant-claude-md-files">
  Exclude irrelevant CLAUDE.md files
</h3>

Ketika Anda memulai Claude dari repository root, `CLAUDE.md` setiap subdirektori dimuat segera setelah Claude membaca file di direktori itu. Pengaturan `claudeMdExcludes` melewati file tertentu berdasarkan jalur atau pola glob sehingga mereka tidak pernah dimuat.

Gunakan ini untuk direktori yang tidak pernah Anda kerjakan, seperti paket tim lain, kode legacy, atau subtree yang di-vendor. Daftar pengecualian bersifat statis, bukan switch per-tugas. Untuk fokus pada satu paket hari ini dan paket lain besok, [mulai Claude dari direktori paket itu](#choose-where-to-start-claude) bukan mengedit pengecualian.

Jika Anda hanya menginginkan pengecualian ini untuk diri sendiri, letakkan pengaturan di `.claude/settings.local.json`. Claude Code mengabaikan file itu ketika membuatnya; karena Anda membuatnya dengan tangan di sini, tambahkan ke gitignore Anda. Pola menggunakan sintaks glob yang cocok dengan jalur file absolut, jadi mulai pola gaya-relatif dengan `**/` untuk cocok di mana saja di pohon. Contoh di bawah mengecualikan paket yang dimiliki oleh tim lain:

```json .claude/settings.local.json theme={null}
{
  "claudeMdExcludes": [
    "**/packages/admin-dashboard/**",
    "**/packages/legacy-*/**"
  ]
}
```

Ini melewati setiap CLAUDE.md dan file rules di bawah paket itu. Root CLAUDE.md dan paket yang Anda kerjakan masih dimuat secara normal.

Pola ini mencakup kasus umum lainnya:

* `"**/packages/*/CLAUDE.md"`: mengecualikan CLAUDE.md setiap paket sambil menjaga root
* `"**/packages/web/**"`: mengecualikan segalanya di bawah paket web, termasuk rules
* `"/home/user/monorepo/legacy/CLAUDE.md"`: mengecualikan satu file tertentu berdasarkan jalur absolut

File CLAUDE.md kebijakan yang dikelola tidak dapat dikecualikan, jadi instruksi organisasi-wide selalu berlaku. Anda dapat mengatur `claudeMdExcludes` di [scope pengaturan](/docs/id/settings#configuration-scopes) apa pun: user, project, local, atau managed. Array menggabung di seluruh scope, jadi tim dapat mengatur default level-proyek sementara individu menambahkan override lokal.

Untuk dokumentasi pengecualian lengkap, lihat [Kecualikan file CLAUDE.md tertentu](/docs/id/memory#exclude-specific-claude-md-files).

<h2 id="reduce-what-claude-reads">
  Kurangi apa yang Claude baca
</h2>

Instruksi hanya bagian dari apa yang berakhir dalam konteks Claude. Pembacaan file adalah biaya lain yang tumbuh dengan codebase. Pengaturan di bawah memblokir pembacaan jalur yang tidak relevan dan menggantikan pemindaian file yang lengkap dengan pencarian language-server.

<h3 id="block-reads-of-generated-and-vendored-code">
  Block reads of generated and vendored code
</h3>

Pencarian konten Claude menghormati `.gitignore` secara default, jadi jalur yang sudah terdaftar di sana, seperti `node_modules/`, `dist/`, dan `build/`, tetap keluar dari hasil pencarian tanpa konfigurasi tambahan.

Untuk jalur yang diperiksa, seperti SDK yang di-vendor atau kode yang dihasilkan berkomitmen, tambahkan aturan deny `Read` di `permissions.deny` untuk memblokir Claude dari membuka file itu bahkan ketika pencarian mencantumnya.

Untuk menerapkan pengecualian ini untuk semua orang yang bekerja di repository, komitkan ke `.claude/settings.json`. Untuk menyimpannya pribadi, gunakan `.claude/settings.local.json` sebagai gantinya. Seperti pengaturan proyek lainnya di halaman ini, file ini hanya dimuat dari direktori awal Anda. Letakkan di repository root jika Anda memulai Claude di sana, atau di `.claude/` setiap paket jika Anda memulai dari subdirektori. Untuk memberlakukan aturan deny yang sama di setiap sesi terlepas dari direktori awal, atur di [managed settings](/docs/id/settings#settings-files), yang tidak dapat ditimpa oleh pengaturan user dan project.

Contoh di bawah memblokir artefak build dan SDK yang di-vendor:

```json .claude/settings.json theme={null}
{
  "permissions": {
    "deny": [
      "Read(./**/dist/**)",
      "Read(./**/build/**)",
      "Read(./**/*.generated.*)",
      "Read(./vendor/**)"
    ]
  }
}
```

Aturan deny mencakup alat file bawaan Claude dan perintah Bash yang dikenali, termasuk `cat`, `head`, `grep`, dan `find`, ketika jalur yang ditolak diteruskan sebagai argumen. Mereka tidak memfilter jalur yang ditolak dari output pencarian rekursif, dan mereka tidak mencakup subprocess arbitrer yang membuka file sendiri. Untuk sintaks pola lengkap, lihat [Aturan izin Read dan Edit](/docs/id/permissions#read-and-edit).

<h3 id="reduce-file-reads-with-code-intelligence">
  Reduce file reads with code intelligence
</h3>

Dalam codebase besar, menemukan di mana simbol didefinisikan atau digunakan dapat menghabiskan banyak pembacaan file dan panggilan grep. [Plugin code intelligence](/docs/id/discover-plugins#code-intelligence) menghubungkan Claude ke language server sehingga dapat melompat ke definisi, menemukan referensi, dan permukaan kesalahan tipe secara langsung bukan memindai pohon.

Marketplace resmi memiliki plugin untuk TypeScript, Python, Go, Rust, dan bahasa umum lainnya. Contoh di bawah menginstal plugin TypeScript:

```shell theme={null}
/plugin install typescript-lsp@claude-plugins-official
```

Untuk mengaktifkan plugin untuk semua orang di repository bukan menginstalnya sendiri, tambahkan ke pengaturan proyek [`enabledPlugins`](/docs/id/settings#plugin-settings).

Plugin code intelligence memerlukan biner language server bahasa di setiap mesin pengembang. Lihat [biner mana yang diperlukan setiap bahasa](/docs/id/discover-plugins#code-intelligence). Menginstal dari marketplace resmi memerlukan akses jaringan ke GitHub, di mana marketplace dihosting. Di jaringan terbatas, [tambahkan marketplace dari host Git internal atau jalur lokal](/docs/id/discover-plugins#add-from-other-git-hosts) sebagai gantinya.

Ini berpasangan baik dengan `claudeMdExcludes` dan aturan `Read` deny di atas. Mereka menjaga konten yang tidak relevan keluar dari konteks, dan code intelligence menjaga Claude dari membaca melalui apa yang tersisa untuk menemukan definisi.

<h2 id="scope-worktrees-and-file-access">
  Scope worktrees dan file access
</h2>

Pengaturan ini mengontrol apa yang ada di disk di worktrees dan direktori mana Claude dapat membaca dan menulis di luar titik awal Anda.

<h3 id="check-out-only-the-directories-you-need">
  Check out only the directories you need
</h3>

Flag `--worktree` memulai sesi di worktree git baru sehingga perubahan tetap terisolasi dari checkout utama Anda. Secara default itu memeriksa seluruh repository. Dalam repository besar, pengaturan `worktree.sparsePaths` menggunakan git sparse-checkout untuk menulis hanya direktori yang terdaftar plus file level-root ke disk, sehingga worktrees dimulai lebih cepat dan menggunakan lebih sedikit ruang.

Jika semua orang yang bekerja di direktori ini memerlukan jalur yang sama, komitkan pengaturan ke `.claude/settings.json`. Untuk menambahkan jalur untuk diri sendiri, gunakan `.claude/settings.local.json`: daftar menggabung di seluruh scope, jadi file lokal dapat menambahkan jalur ke daftar berkomitmen tetapi tidak menghapusnya. Contoh di bawah menunjukkan file berkomitmen:

```json .claude/settings.json theme={null}
{
  "worktree": {
    "sparsePaths": [
      ".claude",
      "packages/api",
      "packages/shared"
    ]
  }
}
```

Ketika Claude membuat worktree, itu memeriksa hanya `.claude/`, `packages/api/`, dan `packages/shared/` bukan pohon penuh. Jalur di `sparsePaths` relatif terhadap repository root, terlepas dari subdirektori mana Anda memulai Claude dari. Jalur direktori apa pun bekerja di sini, bukan hanya root paket.

Ini sangat berguna untuk [isolasi worktree subagent](/docs/id/worktrees#isolate-subagents-with-worktrees). Subagent adalah instance Claude paralel yang dihasilkan untuk subtask, dan masing-masing yang berjalan di worktree mendapat checkout ringan bukan pohon penuh. Semua worktrees dalam sesi berbagi `sparsePaths` yang sama, jadi jika satu subagent memerlukan `packages/api/` dan yang lain memerlukan `packages/web/`, daftarkan keduanya.

Daftarkan direktori di `sparsePaths`, bukan file individual. File level-root seperti `package.json`, `tsconfig.base.json`, dan file lock selalu diperiksa bersama direktori yang Anda daftarkan. Direktori level-root tidak, jadi sertakan `.claude` dalam daftar jika Anda menginginkan `.claude/settings.json`, `.claude/rules/`, atau `.claude/skills/` repository root tersedia di dalam worktree.

Sparse checkout memerlukan git untuk mengaktifkan `extensions.worktreeConfig` di `.git/config` bersama repository saat worktree sparse ada. Claude Code menghapus entri itu setelah worktree terakhir dihapus, tetapi hanya jika Claude Code yang menambahkannya. Itu tidak pernah menghapus nilai yang Anda atur sendiri. Sebelum v2.1.207, entri tetap ada setelah worktree terakhir dihapus, dan alat berbasis go-git seperti `tea` gagal membuka repository sampai Anda menjalankan `git config --unset extensions.worktreeConfig`.

Untuk menghindari duplikasi direktori besar seperti `node_modules` di seluruh worktrees, pasangkan `sparsePaths` dengan `symlinkDirectories` di `.claude/settings.json` yang sama:

```json .claude/settings.json theme={null}
{
  "worktree": {
    "sparsePaths": [
      ".claude",
      "packages/api",
      "packages/shared"
    ],
    "symlinkDirectories": [
      "node_modules"
    ]
  }
}
```

Ini membuat symlink dari `node_modules/` setiap worktree kembali ke salinan repository utama bukan menduplikasinya di disk.

<Note>
  Pengaturan `sparsePaths` dan `symlinkDirectories` dibaca dari direktori awal Anda sebelum worktree dibuat. Setelah pembuatan, direktori kerja sesi adalah worktree root, bukan subdirektori tempat Anda meluncurkan. Pengaturan proyek di dalam worktree oleh karena itu dimuat dari `.claude/settings.json` worktree root, salinan berkomitmen dari file root repository. Letakkan pengaturan lain apa pun yang Anda butuhkan di dalam worktrees, seperti aturan izin atau hooks, di `.claude/settings.json` repository root.
</Note>

Untuk referensi pengaturan worktree lengkap, lihat [Worktree settings](/docs/id/settings#worktree-settings).

<h3 id="grant-access-across-packages-or-repositories">
  Grant access across packages or repositories
</h3>

Bagian ini berlaku ketika Anda memulai Claude dari subdirektori, atau ketika tugas mencakup beberapa checkout. Jika Anda memulai dari repository root dalam pohon besar tunggal, Claude sudah memiliki akses ke setiap file dan Anda dapat melewati ini.

Ketika Anda memulai Claude dari `packages/api/`, itu dapat membaca dan menulis file dalam direktori itu. Jika tugas memerlukan perubahan di seluruh paket, seperti memperbarui tipe bersama yang diimpor oleh `api` dan `web`, Anda perlu memberikan akses ke direktori sibling. Mekanisme yang sama memberikan akses ke repository yang diperiksa secara terpisah.

Pengaturan `additionalDirectories` di `.claude/settings.json` memberikan Claude akses ke direktori di luar direktori kerja. Contoh di bawah memberikan akses ke dua paket sibling:

```json .claude/settings.json theme={null}
{
  "permissions": {
    "additionalDirectories": [
      "../shared",
      "../web"
    ]
  }
}
```

Jalur relatif diselesaikan terhadap direktori tempat Anda memulai Claude. Dengan konfigurasi ini, Claude dapat membaca dan mengedit file di `packages/shared/` dan `packages/web/` sambil bekerja dari `packages/api/`.

Anda juga dapat memberikan akses saat runtime tanpa mengedit pengaturan dengan melewatkan `--add-dir` ketika Anda memulai Claude:

```bash theme={null}
claude --add-dir ../shared
```

Bagaimanapun Anda menambahkan direktori, Claude dapat membaca dan mengedit file di dalamnya. Apakah CLAUDE.md direktori, file `.claude/rules/`, dan skills juga dimuat tergantung pada cara Anda menambahkannya:

| Ditambahkan dengan                        | Memuat CLAUDE.md dan rules                | Memuat skills |
| :---------------------------------------- | :---------------------------------------- | :------------ |
| Pengaturan `additionalDirectories`        | Tidak pernah                              | Tidak pernah  |
| Flag `--add-dir` atau perintah `/add-dir` | Hanya dengan variabel lingkungan di bawah | Ya            |

Untuk memuat file CLAUDE.md dan rules dari direktori yang ditambahkan dengan `--add-dir` atau `/add-dir`, atur variabel lingkungan `CLAUDE_CODE_ADDITIONAL_DIRECTORIES_CLAUDE_MD`:

```bash theme={null}
CLAUDE_CODE_ADDITIONAL_DIRECTORIES_CLAUDE_MD=1 claude --add-dir ../shared
```

Variabel lingkungan tidak berpengaruh pada direktori yang terdaftar dalam pengaturan `additionalDirectories`. Lihat [Muat dari direktori tambahan](/docs/id/memory#load-from-additional-directories) untuk detail.

Untuk direktori sibling yang semua orang di area ini butuhkan, komitkan `additionalDirectories` ke `.claude/settings.json`. Untuk pilihan pribadi atau akses satu kali, gunakan `.claude/settings.local.json` atau lewatkan `--add-dir` saat peluncuran.

<h2 id="add-per-directory-skills">
  Tambahkan skills per-direktori
</h2>

Subdirektori apa pun dapat mendefinisikan [skills](/docs/id/skills) yang dibatasi pada stack-nya sendiri. Skill dimuat sesuai permintaan ketika Claude menentukan itu relevan, jadi tooling spesifik API tidak menghabiskan konteks selama pekerjaan frontend.

Skills hidup di bawah `.claude/skills/` di dalam direktori. Komitkan mereka bersama kode area itu sehingga siapa pun yang mengkloning repository mendapatkannya. Dalam monorepo ini dapat menjadi satu set skills per paket. Dalam codebase pohon tunggal besar itu satu set per subsistem seperti `src/db/.claude/skills/`.

Buat direktori skill di dalam subdirektori:

```bash theme={null}
mkdir -p packages/api/.claude/skills/api-testing
```

Kemudian tulis `SKILL.md` di dalam direktori itu, di sini `packages/api/.claude/skills/api-testing/SKILL.md`. Contoh ini mengajarkan Claude pola pengujian paket API:

```markdown packages/api/.claude/skills/api-testing/SKILL.md theme={null}
---
name: api-testing
description: Pola pengujian untuk paket API. Gunakan saat menulis atau memodifikasi tes di packages/api/.
---

## Struktur tes

Tes berada di `src/__tests__/` mencerminkan struktur direktori `src/`.
Setiap file rute memiliki file `.test.ts` yang sesuai.

## Menjalankan tes

- Semua tes: `npm test`
- File tunggal: `npm test -- src/__tests__/routes/users.test.ts`
- Mode watch: `npm test -- --watch`

## Utilitas tes

- `src/__tests__/helpers/db.ts`: menyediakan `setupTestDb()` dan `teardownTestDb()` untuk tes database
- `src/__tests__/helpers/auth.ts`: menyediakan `createTestUser()` dan `getAuthToken()` untuk endpoint yang diautentikasi

## Pola

- Gunakan `supertest` untuk asersi HTTP, bukan fetch mentah
- Selalu bungkus tes database dalam transaksi yang rollback
- Mock layanan eksternal di `src/__tests__/mocks/`
```

Subdirektori berbeda memegang skills berbeda dengan cara yang sama: `packages/web/.claude/skills/component-patterns/` menjelaskan konvensi komponen frontend bukan pengujian. Ketika Claude bekerja pada file di `packages/api/`, itu memuat skill api-testing. Ketika itu bekerja di `packages/web/`, itu memuat component-patterns sebagai gantinya. Tidak ada skill direktori yang dimuat selama tugas yang lain.

Anda juga dapat membatasi skill berdasarkan pola file bukan penempatan. Field frontmatter [`paths`](/docs/id/skills#frontmatter-reference) mengambil pola glob, dan Claude memuat skill secara otomatis hanya ketika bekerja dengan file yang cocok. Gunakan ini untuk skill yang hidup di `.claude/skills/` repository root tetapi berlaku hanya untuk file tertentu di mana pun mereka muncul, seperti skill migrasi database yang dibatasi pada `**/migrations/**`.

Untuk lebih lanjut tentang membuat dan mengorganisir skills, lihat [Skills](/docs/id/skills).

<h3 id="keep-skills-discoverable">
  Jaga skills tetap dapat ditemukan
</h3>

Dengan skills tersebar di banyak direktori, daftar yang Claude pilih dari dapat tumbuh besar. Claude memilih skill dengan membaca nama dan deskripsi setiap skill yang ditemukan, dan hanya konten skill yang dipilih dimuat penuh ke dalam konteks. Bagian ini mencakup cara menjaga daftar itu tetap kecil dan menulis deskripsi yang bertahan pemendekkan.

Skill mana yang dalam scope tergantung pada di mana Anda memulai Claude:

* **Dari subdirektori seperti `packages/api/`**: skills dari direktori itu, setiap parent hingga repository root, dan level user dan enterprise
* **Dari repository root**: skills dari setiap subdirektori Claude sentuh selama sesi, yang dapat terakumulasi menjadi ratusan
* **Setelah menambahkan sibling dengan [`--add-dir`](#grant-access-across-packages-or-repositories)**: skills sibling itu juga dimuat. Pengaturan `additionalDirectories` memberikan akses file saja dan tidak memuat skills

Nama selalu dimuat, tetapi [deskripsi dipendekkan ketika ada banyak](/docs/id/skills#skill-descriptions-are-cut-short), yang dapat menghapus kata kunci Claude gunakan untuk memutuskan apakah skill berlaku. Jaga deskripsi tetap pendek dan pimpin dengan kata yang permintaan akan berisi, seperti "menulis atau memodifikasi tes di `packages/api/`".

Untuk skills yang banyak direktori bagikan, seperti konvensi PR atau checklist deploy, letakkan di `.claude/skills/` repository root sehingga mereka dimuat dari direktori awal apa pun. Ketika shared skills memerlukan riwayat versi mereka sendiri atau harus bekerja di seluruh repositories, paketkan sebagai [plugin](/docs/id/plugins) sebagai gantinya. Plugin skills menggunakan namespace `plugin-name:skill-name`, jadi mereka tidak pernah bertabrakan dengan skills per-direktori. Tim platform dapat memversi dan memperbarui mereka di satu tempat.

Untuk menemukan skills mana yang tidak digunakan, aktifkan [logs exporter](/docs/id/monitoring-usage) OpenTelemetry dan atur `OTEL_LOG_TOOL_DETAILS=1` sehingga nama skill dicatat verbatim bukan diredaksi. Event [`skill_activated`](/docs/id/monitoring-usage#skill-activated-event) mencatat setiap invokasi dalam atribut `skill.name`-nya, dan `invocation_trigger` mencatat apakah perintah, Claude, atau skill bersarang menginvokasinya, yang memberi tahu Anda apa yang harus dikonsolidasikan atau dihentikan.

<h2 id="centralize-conventions-when-layering-stops-scaling">
  Centralize conventions when layering stops scaling
</h2>

File CLAUDE.md per-direktori dapat menjadi sulit untuk diatur seiring dengan pertumbuhan codebase. Konvensi melayang, file menjadi usang, dan tidak ada yang memiliki root. Menyelesaikan itu biasanya jatuh ke tim yang memelihara setup Claude Code repository bukan ke setiap pengembang yang bekerja di area mereka sendiri.

Pindahkan konvensi dan konten referensi keluar dari CLAUDE.md yang selalu dimuat dan ke mekanisme yang dimuat sesuai permintaan:

* [Skills](/docs/id/skills): materi referensi Claude dimuat hanya ketika relevan dengan tugas
* [Plugins](/docs/id/plugins): bundel terversi dari skills, hooks, dan perintah yang tim platform miliki secara terpusat
* [MCP servers](/docs/id/mcp): jika organisasi Anda sudah menjalankan pencarian kode atau indeks RAG di atas repository, eksposnya sebagai alat MCP sehingga Claude menanyainya bukan membaca file secara langsung

Lihat [pengaturan yang dikelola server atau endpoint](/docs/id/server-managed-settings#choose-between-server-managed-and-endpoint-managed-settings) untuk bagaimana tim platform dapat memberlakukan ini secara terpusat.

<h3 id="recommend-the-right-plugin-at-session-start">
  Recommend the right plugin at session start
</h3>

Setelah konvensi hidup di plugins, rekan kerja yang memulai Claude di bagian pohon yang tidak familiar tidak memiliki sinyal tentang plugin mana yang pemilik area itu pertahankan. Hook [`SessionStart`](/docs/id/hooks#sessionstart) dapat menutup kesenjangan itu, karena apa pun yang hook cetak ke stdout ditambahkan ke konteks Claude sebelum prompt pertama.

Misalnya, Anda dapat menulis skrip yang membaca direktori peluncuran dari [input hook](/docs/id/hooks#common-input-fields), mencarinya dalam peta path-to-plugin yang berkomitmen ke repository, dan mencetak rekomendasi untuk Claude relay dalam balasan pertamanya. Lihat [Otomatisasi tindakan dengan hooks](/docs/id/hooks-guide) untuk menulis dan mendaftarkan hook.

<h2 id="put-it-together">
  Put it together
</h2>

Konfigurasi gabungan di bawah menggunakan tata letak monorepo. File yang sama bekerja untuk subdirektori apa pun dalam pohon tunggal besar. Pengaturan proyek hanya dimuat dari direktori tempat Anda memulai Claude, jadi `.claude/settings.json` setiap subdirektori harus mandiri bukan berlapis pada file root.

Contoh berkomitmen `worktree`, `additionalDirectories`, dan aturan deny `Read` di `.claude/settings.json` sehingga setiap pengembang di `packages/api/` mendapat akses sibling yang sama, jalur sparse, dan pengecualian. File di bawah adalah pengaturan per-area berkomitmen untuk `packages/api/`:

```json packages/api/.claude/settings.json theme={null}
{
  "worktree": {
    "sparsePaths": [
      ".claude",
      "packages/api",
      "packages/shared"
    ],
    "symlinkDirectories": [
      "node_modules"
    ]
  },
  "permissions": {
    "additionalDirectories": [
      "../shared"
    ],
    "deny": [
      "Read(./**/dist/**)",
      "Read(./**/build/**)"
    ]
  }
}
```

Karena sesi ini dimulai dari `packages/api/`, file CLAUDE.md paket sibling sudah keluar dari scope, jadi `claudeMdExcludes` tidak diperlukan di sini. Tambahkan ke `.claude/settings.local.json` repository root sebagai gantinya jika Anda juga memulai sesi dari root.

Entri `additionalDirectories` berlaku ketika Anda memulai Claude dari `packages/api/` secara langsung. Di dalam worktree yang dibuat dari sesi ini, direktori kerja adalah worktree root, jadi file pengaturan ini tidak dimuat. Paket sibling sudah dapat dijangkau di dalam worktree tanpanya, tetapi aturan deny memerlukan salinan kedua di `.claude/settings.json` repository root sehingga sesi worktree mengambilnya, seperti [catatan pengaturan worktree](#check-out-only-the-directories-you-need) menjelaskan:

```json .claude/settings.json theme={null}
{
  "permissions": {
    "deny": [
      "Read(./**/dist/**)",
      "Read(./**/build/**)"
    ]
  }
}
```

Setelah setup, repository memiliki tata letak ini:

```text theme={null}
monorepo/
  CLAUDE.md
  .claude/settings.json                           # aturan deny untuk sesi worktree
  packages/
    api/
      CLAUDE.md
      .claude/settings.json                       # worktree, additionalDirectories, aturan deny
      .claude/skills/api-testing/SKILL.md
    web/
      CLAUDE.md
      .claude/skills/component-patterns/SKILL.md
    shared/
      CLAUDE.md
```

Dengan setup ini, memulai Claude dari `packages/api/`:

* Memuat root CLAUDE.md dan `packages/api/CLAUDE.md`, melewati `packages/web/CLAUDE.md`
* Dapat membaca dan mengedit file di `packages/api/` dan `packages/shared/`
* Melewati pembacaan output build di bawah `dist/` dan `build/` di `packages/api/`
* Memiliki skill api-testing tersedia sesuai permintaan
* Membuat worktrees yang berisi `.claude/`, `packages/api/`, `packages/shared/`, dan file level-root, dengan aturan deny diterapkan di seluruh worktree dari file pengaturan root

<h2 id="scope-and-plan-changes-that-span-packages">
  Scope and plan changes that span packages
</h2>

Konfigurasi di atas mengontrol apa yang Claude lihat. Ketika perubahan tunggal menyentuh beberapa paket, seperti memperbarui tipe bersama bersama dengan setiap situs panggilan yang menggunakannya, bagaimana Anda membatasi dan mengurutkan tugas juga mempengaruhi hasilnya.

Dua teknik membantu menjaga perubahan lintas-paket konsisten:

* **Berikan Claude seluruh perubahan dalam satu sesi**: menyerahkan edit bersama dan situs panggilannya bersama-sama menjaga keputusan di balik setiap edit konsisten, bukan menurunkan mereka per paket
* **Simpan rencana ke file sebelum mengedit**: [rencanakan terlebih dahulu](/docs/id/best-practices#explore-first-then-plan-then-code) dan minta Claude menulis rencana ke file markdown di repository. Sesi lintas-paket panjang [mengompak konteksnya](/docs/id/context-window#what-survives-compaction) sepanjang jalan, dan rencana yang disimpan bertahan di mana riwayat percakapan mungkin tidak

<h2 id="next-steps">
  Langkah berikutnya
</h2>

Setelah konfigurasi ini ada, Anda dapat menyempurnakannya:

* Gunakan [hooks](/docs/id/hooks-guide) untuk menjalankan linter per-direktori atau type-checker setelah Claude mengedit file
* Tinjau [Kelola biaya secara efektif](/docs/id/costs) untuk memahami bagaimana ukuran codebase mempengaruhi penggunaan token dan cara menetapkan batas pengeluaran sebelum rollout yang lebih luas
* Baca [Bagaimana Claude Code bekerja dalam codebase besar](https://claude.com/blog/how-claude-code-works-in-large-codebases-best-practices-and-where-to-start) di blog Claude untuk pola rollout organisasi dan model kepemilikan yang duduk di atas konfigurasi per-repository di halaman ini
