> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Perluas Claude dengan skills

> Buat, kelola, dan bagikan skills untuk memperluas kemampuan Claude di Claude Code. Termasuk perintah kustom dan skills bundel.

Skills memperluas apa yang dapat dilakukan Claude. Buat file `SKILL.md` dengan instruksi, dan Claude menambahkannya ke toolkit-nya. Claude menggunakan skills saat relevan, atau Anda dapat menginvokasinya secara langsung dengan `/skill-name`.

Buat skill ketika Anda terus menempel instruksi yang sama, checklist, atau prosedur multi-langkah ke dalam chat, atau ketika bagian dari CLAUDE.md telah berkembang menjadi prosedur daripada fakta. Tidak seperti konten CLAUDE.md, badan skill hanya dimuat saat digunakan, jadi materi referensi yang panjang hampir tidak ada biayanya sampai Anda membutuhkannya.

<Note>
  Untuk perintah bawaan seperti `/help` dan `/compact`, dan skills bundel seperti `/debug` dan `/code-review`, lihat [referensi perintah](/docs/id/commands).

  **Perintah kustom telah digabungkan ke dalam skills.** File di `.claude/commands/deploy.md` dan skill di `.claude/skills/deploy/SKILL.md` keduanya membuat `/deploy` dan bekerja dengan cara yang sama. File `.claude/commands/` yang ada tetap berfungsi. Skills menambahkan fitur opsional: direktori untuk file pendukung, frontmatter untuk [mengontrol apakah Anda atau Claude menginvokasinya](#control-who-invokes-a-skill), dan kemampuan bagi Claude untuk memuatnya secara otomatis saat relevan.
</Note>

Skills Claude Code mengikuti standar terbuka [Agent Skills](https://agentskills.io), yang bekerja di berbagai alat AI. Claude Code memperluas standar dengan fitur tambahan seperti [kontrol invokasi](#control-who-invokes-a-skill), [eksekusi subagent](#run-skills-in-a-subagent), dan [injeksi konteks dinamis](#inject-dynamic-context).

<h2 id="bundled-skills">
  Skills bundel
</h2>

Claude Code menyertakan serangkaian skills bundel yang tersedia di setiap sesi kecuali dinonaktifkan dengan pengaturan [`disableBundledSkills`](/docs/id/settings#available-settings), termasuk `/doctor`, `/code-review`, `/batch`, `/debug`, `/loop`, dan `/claude-api`. Tidak seperti sebagian besar perintah bawaan, yang menjalankan logika tetap secara langsung, skills bundel berbasis prompt: mereka memberikan Claude instruksi terperinci dan membiarkannya mengorkestrasi pekerjaan menggunakan tools-nya. Anda menginvokasinya dengan cara yang sama seperti skill lainnya, dengan mengetik `/` diikuti dengan nama skill.

Pengaturan checkup [`/doctor`](/docs/id/commands#all-commands) adalah satu-satunya pengecualian untuk `disableBundledSkills` di Claude Code v2.1.205 dan lebih baru: tetap dapat diketik ketika pengaturan aktif. Untuk menyembunyikannya, atur variabel lingkungan `DISABLE_DOCTOR_COMMAND` atau entri [`skillOverrides`](#override-skill-visibility-from-settings) dari `"doctor": "off"`. Sebelum v2.1.205, `/doctor` adalah perintah bawaan daripada skill bundel.

Skills bundel terdaftar bersama perintah bawaan dalam [referensi perintah](/docs/id/commands), ditandai **Skill** di kolom Tujuan.

<h3 id="run-and-verify-your-app">
  Jalankan dan verifikasi aplikasi Anda
</h3>

Tiga skills bundel bekerja bersama untuk meluncurkan aplikasi Anda dan mengonfirmasi perubahan terhadap aplikasi yang sedang berjalan alih-alih hanya tes:

| Skill                  | Tujuan                                                                                                                                         |
| :--------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------- |
| `/run`                 | Luncurkan dan jalankan aplikasi Anda untuk melihat perubahan bekerja                                                                           |
| `/verify`              | Bangun dan jalankan aplikasi Anda untuk mengonfirmasi perubahan kode melakukan apa yang seharusnya, tanpa kembali ke tes atau pemeriksaan tipe |
| `/run-skill-generator` | Ajarkan `/run` dan `/verify` cara membangun dan meluncurkan proyek Anda                                                                        |

Ketiga skills memerlukan Claude Code v2.1.145 atau lebih baru.

`/run` dan `/verify` bekerja tanpa pengaturan. Mereka menyimpulkan peluncuran dari jenis proyek Anda (CLI, server, TUI, berbasis browser) dan dari apa yang ada di README, `package.json`, atau `Makefile` Anda. Kesimpulan itu menjadi tidak dapat diandalkan untuk proyek yang memerlukan apa pun di luar peluncuran standar: database, file env, sesi grafis, build multi-langkah.

`/run-skill-generator` merekam resep sebagai gantinya. Ini membuat aplikasi Anda berjalan dari lingkungan yang bersih, menangkap apa yang berhasil (perintah instalasi, variabel env, skrip peluncuran), dan melakukannya sebagai skill per-proyek di `.claude/skills/run-<name>/`. Setelah itu, `/run`, `/verify`, dan agen lainnya di repo mengikuti resep yang direkam alih-alih menemukannya kembali. Jalankan `/run-skill-generator` sekali per proyek, dan lagi jika proses build atau peluncuran berubah.

<h2 id="getting-started">
  Memulai
</h2>

<h3 id="create-your-first-skill">
  Buat skill pertama Anda
</h3>

Contoh ini membuat skill yang merangkum perubahan yang belum di-commit dalam repositori git Anda dan menandai apa pun yang berisiko. Ini menarik diff langsung ke dalam prompt sebelum Claude membacanya, sehingga respons didasarkan pada pohon kerja aktual Anda daripada apa yang dapat Claude tebak dari file terbuka. Claude memuat skill secara otomatis saat Anda bertanya tentang perubahan Anda, atau Anda dapat menginvokasinya secara langsung dengan `/summarize-changes`.

<Steps>
  <Step title="Buat direktori skill">
    Buat direktori untuk skill di folder skills pribadi Anda. Skills pribadi tersedia di semua proyek Anda.

    ```bash theme={null}
    mkdir -p ~/.claude/skills/summarize-changes
    ```
  </Step>

  <Step title="Tulis SKILL.md">
    Setiap skill memerlukan file `SKILL.md` dengan dua bagian: frontmatter YAML antara penanda `---` yang memberi tahu Claude kapan menggunakan skill, dan konten markdown dengan instruksi yang diikuti Claude saat skill dijalankan. Nama direktori menjadi perintah yang Anda ketik, dan `description` membantu Claude memutuskan kapan memuatnya secara otomatis.

    Simpan ini ke `~/.claude/skills/summarize-changes/SKILL.md`:

    ```yaml theme={null}
    ---
    description: Summarizes uncommitted changes and flags anything risky. Use when the user asks what changed, wants a commit message, or asks to review their diff.
    ---

    ## Current changes

    !`git diff HEAD`

    ## Instructions

    Summarize the changes above in two or three bullet points, then list any risks you notice such as missing error handling, hardcoded values, or tests that need updating. If the diff is empty, say there are no uncommitted changes.
    ```

    Baris `` !`git diff HEAD` `` menggunakan [injeksi konteks dinamis](#inject-dynamic-context): Claude Code menjalankan perintah dan mengganti baris dengan outputnya sebelum Claude melihat konten skill, sehingga instruksi tiba dengan diff saat ini sudah inline.
  </Step>

  <Step title="Uji skill">
    Buka proyek git, buat edit kecil ke file apa pun, dan mulai Claude Code dengan menjalankan `claude`. Anda dapat menguji skill dengan dua cara.

    **Biarkan Claude menginvokasinya secara otomatis** dengan menanyakan sesuatu yang cocok dengan deskripsi:

    ```text theme={null}
    What did I change?
    ```

    **Atau invokasinya secara langsung** dengan nama skill:

    ```text theme={null}
    /summarize-changes
    ```

    Baik cara apa pun, Claude harus merespons dengan ringkasan singkat edit Anda dan daftar risiko.
  </Step>
</Steps>

<h3 id="where-skills-live">
  Tempat skills berada
</h3>

Tempat Anda menyimpan skill menentukan siapa yang dapat menggunakannya:

| Lokasi     | Path                                                      | Berlaku untuk                     |
| :--------- | :-------------------------------------------------------- | :-------------------------------- |
| Enterprise | Lihat [pengaturan terkelola](/docs/id/settings#settings-files) | Semua pengguna di organisasi Anda |
| Pribadi    | `~/.claude/skills/<skill-name>/SKILL.md`                  | Semua proyek Anda                 |
| Proyek     | `.claude/skills/<skill-name>/SKILL.md`                    | Proyek ini saja                   |
| Plugin     | `<plugin>/skills/<skill-name>/SKILL.md`                   | Tempat plugin diaktifkan          |

Ketika skills berbagi nama yang sama di berbagai level, enterprise menggantikan pribadi, dan pribadi menggantikan proyek. Skill di salah satu level ini juga menggantikan skill bundel dengan nama yang sama. Misalnya, skill `code-review` di `.claude/skills/` proyek Anda menggantikan `/code-review` bundel. Skills plugin menggunakan namespace `plugin-name:skill-name`, jadi mereka tidak dapat bertentangan dengan level lain. Jika Anda memiliki file di `.claude/commands/`, file tersebut bekerja dengan cara yang sama, tetapi jika skill dan perintah berbagi nama yang sama, skill mengambil alih.

Skills juga memuat dari direktori `.claude/skills/` bersarang di bawah direktori kerja Anda. Ketika Claude membaca atau mengedit file di subdirektori, skills dari `.claude/skills/` subdirektori tersebut menjadi tersedia. Ini memungkinkan paket monorepo menyediakan skills mereka sendiri yang berlaku saat bekerja pada paket tersebut, bahkan jika sesi dimulai di akar repo.

Jika skill bersarang berbagi nama dengan skill lain, keduanya tetap tersedia. Misalnya, dengan skill `deploy` di akar proyek dan skill lain di `apps/web/.claude/skills/`:

* Yang bersarang muncul di bawah nama yang memenuhi direktori, `apps/web:deploy`.
* Deskripsinya mengatakan direktori mana yang berlaku.
* Claude memilih varian yang cocok dengan file yang sedang dikerjakan.

Mengetik `/deploy` menjalankan skill akar proyek. Ketik nama yang memenuhi `/apps/web:deploy` untuk menjalankan varian bersarang secara eksplisit.

Ketika Anda atau Claude menginvokasinya dengan nama yang tidak memenuhi, skill akar proyek dimuat, dan Claude Code menambahkan daftar varian yang memenuhi direktori ke kontennya dengan instruksi untuk juga menginvokasinya varian apa pun yang direktorinya menyimpan file yang sedang dikerjakan Claude. Skill bersarang oleh karena itu masih berlaku untuk pekerjaan di direktorinya ketika hanya nama yang tidak memenuhi yang diinvokasinya. Memerlukan Claude Code v2.1.203 atau lebih baru.

Entri `<skill-name>` di lokasi enterprise, pribadi, atau proyek dapat berupa symlink ke direktori di tempat lain di disk. Claude Code mengikuti symlink dan membaca `SKILL.md` dari direktori target, dan jika target yang sama dapat dijangkau dari lebih dari satu lokasi, Claude Code memuat skill sekali. Skills plugin menangani symlink secara berbeda; lihat [Bagikan file dalam marketplace dengan symlinks](/docs/id/plugins-reference#share-files-within-a-marketplace-with-symlinks).

<Note>
  Tambahkan `.claude-plugin/plugin.json` ke folder skill dan itu dimuat sebagai [plugin](/docs/id/plugins-reference#skills-directory-plugins) bernama `<name>@skills-dir`, sehingga dapat menggabungkan agents, hooks, dan server MCP. Di `.claude/skills/` proyek, ini memerlukan menerima dialog kepercayaan workspace terlebih dahulu.
</Note>

<h4 id="live-change-detection">
  Deteksi perubahan langsung
</h4>

Claude Code memantau direktori skill untuk perubahan file. Menambahkan, mengedit, atau menghapus skill di bawah `~/.claude/skills/`, proyek `.claude/skills/`, atau `.claude/skills/` di dalam direktori `--add-dir` berlaku dalam sesi saat ini tanpa memulai ulang. Membuat direktori skills tingkat atas yang tidak ada saat sesi dimulai memerlukan memulai ulang Claude Code sehingga direktori baru dapat dipantau.

<Note>
  Deteksi perubahan langsung mencakup teks `SKILL.md` saja. Untuk folder skill yang juga merupakan [plugin](/docs/id/plugins-reference#skills-directory-plugins), perubahan pada `hooks/`, `.mcp.json`, `agents/`, dan `output-styles/` memerlukan `/reload-plugins` untuk diterapkan.
</Note>

<h4 id="automatic-discovery-from-parent-and-nested-directories">
  Penemuan otomatis dari direktori induk dan bersarang
</h4>

Project skills memuat dari `.claude/skills/` di direktori awal Anda dan di setiap direktori induk hingga akar repositori, jadi memulai Claude di subdirektori masih mengambil skills yang ditentukan di akar. Saat Anda bekerja dengan file di subdirektori di bawah direktori awal Anda, Claude Code juga menemukan skills dari direktori `.claude/skills/` bersarang sesuai permintaan. Misalnya, jika Anda mengedit file di `packages/frontend/`, Claude Code juga mencari skills di `packages/frontend/.claude/skills/`. Ini mendukung pengaturan monorepo di mana paket memiliki skills mereka sendiri.

Setiap skill adalah direktori dengan `SKILL.md` sebagai titik masuk:

```text theme={null}
my-skill/
├── SKILL.md           # Main instructions (required)
├── template.md        # Template for Claude to fill in
├── examples/
│   └── sample.md      # Example output showing expected format
└── scripts/
    └── validate.sh    # Script Claude can execute
```

`SKILL.md` berisi instruksi utama dan diperlukan. File lainnya opsional dan memungkinkan Anda membangun skills yang lebih kuat: template untuk diisi Claude, contoh output yang menunjukkan format yang diharapkan, script yang dapat dijalankan Claude, atau dokumentasi referensi terperinci. Referensikan file pendukung dari `SKILL.md` Anda sehingga Claude tahu apa yang mereka berisi dan kapan memuatnya. Lihat [Tambahkan file pendukung](#add-supporting-files) untuk detail lebih lanjut.

<Note>
  File di `.claude/commands/` masih berfungsi dan mendukung [frontmatter](#frontmatter-reference) yang sama. Skills direkomendasikan karena mendukung fitur tambahan seperti file pendukung.
</Note>

<h4 id="skills-from-additional-directories">
  Skills dari direktori tambahan
</h4>

Bendera `--add-dir` dan perintah `/add-dir` [memberikan akses file](/docs/id/permissions#additional-directories-grant-file-access-not-configuration) daripada penemuan konfigurasi, tetapi skills adalah pengecualian: `.claude/skills/` dalam direktori yang ditambahkan dimuat secara otomatis. Pengecualian ini hanya berlaku untuk `--add-dir` dan `/add-dir`. Pengaturan `permissions.additionalDirectories` di `settings.json` memberikan akses file saja dan tidak memuat skills. Lihat [Deteksi perubahan langsung](#live-change-detection) untuk bagaimana edit diambil selama sesi.

Konfigurasi `.claude/` lainnya seperti perintah dan gaya output tidak dimuat dari direktori tambahan. Lihat [tabel pengecualian](/docs/id/permissions#additional-directories-grant-file-access-not-configuration) untuk daftar lengkap apa yang dimuat dan tidak dimuat, serta cara yang direkomendasikan untuk berbagi konfigurasi di seluruh proyek.

<Note>
  File CLAUDE.md dari direktori `--add-dir` tidak dimuat secara default. Untuk memuatnya, atur `CLAUDE_CODE_ADDITIONAL_DIRECTORIES_CLAUDE_MD=1`. Lihat [Muat dari direktori tambahan](/docs/id/memory#load-from-additional-directories).
</Note>

<h2 id="configure-skills">
  Konfigurasi skills
</h2>

Skills dikonfigurasi melalui frontmatter YAML di bagian atas `SKILL.md` dan konten markdown yang mengikutinya.

<h3 id="types-of-skill-content">
  Jenis konten skill
</h3>

File skill dapat berisi instruksi apa pun, tetapi memikirkan bagaimana Anda ingin menginvokasinya membantu memandu apa yang harus disertakan:

**Konten referensi** menambahkan pengetahuan yang diterapkan Claude pada pekerjaan Anda saat ini. Konvensi, pola, panduan gaya, pengetahuan domain. Konten ini berjalan inline sehingga Claude dapat menggunakannya bersama konteks percakapan Anda.

```yaml theme={null}
---
name: api-conventions
description: API design patterns for this codebase
---

When writing API endpoints:
- Use RESTful naming conventions
- Return consistent error formats
- Include request validation
```

**Konten tugas** memberikan Claude instruksi langkah demi langkah untuk tindakan spesifik, seperti deployment, commit, atau pembuatan kode. Ini sering kali tindakan yang ingin Anda invokasinya secara langsung dengan `/skill-name` daripada membiarkan Claude memutuskan kapan menjalankannya. Tambahkan `disable-model-invocation: true` untuk mencegah Claude memicunya secara otomatis.

```yaml theme={null}
---
name: deploy
description: Deploy the application to production
context: fork
disable-model-invocation: true
---

Deploy the application:
1. Run the test suite
2. Build the application
3. Push to the deployment target
```

`SKILL.md` Anda dapat berisi apa pun, tetapi memikirkan bagaimana Anda ingin skill diinvokasinya (oleh Anda, oleh Claude, atau keduanya) dan di mana Anda ingin menjalankannya (inline atau di subagent) membantu memandu apa yang harus disertakan. Untuk skills kompleks, Anda juga dapat [menambahkan file pendukung](#add-supporting-files) untuk menjaga skill utama tetap fokus.

Jaga isi itu sendiri tetap ringkas. Setelah skill dimuat, kontennya [tetap dalam konteks di seluruh giliran](#skill-content-lifecycle), jadi setiap baris adalah biaya token berulang. Nyatakan apa yang harus dilakukan daripada menceritakan bagaimana atau mengapa, dan terapkan tes keringkasan yang sama yang akan Anda lakukan untuk [konten CLAUDE.md](/docs/id/best-practices#write-an-effective-claude-md).

<h3 id="frontmatter-reference">
  Referensi frontmatter
</h3>

Selain konten markdown, Anda dapat mengonfigurasi perilaku skill menggunakan bidang frontmatter YAML antara penanda `---` di bagian atas file `SKILL.md` Anda:

```yaml theme={null}
---
name: my-skill
description: What this skill does
disable-model-invocation: true
allowed-tools: Read Grep
---

Your skill instructions here...
```

Semua bidang opsional. Hanya `description` yang direkomendasikan sehingga Claude tahu kapan menggunakan skill.

| Bidang                     | Diperlukan       | Deskripsi                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| :------------------------- | :--------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`                     | Tidak            | Nama tampilan yang ditampilkan dalam daftar skill. Default ke nama direktori. Lihat [Bagaimana skill mendapatkan nama perintahnya](#how-a-skill-gets-its-command-name) untuk bagaimana ini berbeda dari nama yang Anda ketik untuk menginvokasinya skill.                                                                                                                                                                                                        |
| `description`              | Direkomendasikan | Apa yang dilakukan skill dan kapan menggunakannya. Claude menggunakan ini untuk memutuskan kapan menerapkan skill. Jika dihilangkan, menggunakan paragraf pertama konten markdown. Depankan kasus penggunaan utama: teks gabungan `description` dan `when_to_use` dipotong pada 1.536 karakter dalam daftar skill untuk mengurangi penggunaan konteks.                                                                                                           |
| `when_to_use`              | Tidak            | Konteks tambahan untuk kapan Claude harus menginvokasinya skill, seperti frasa pemicu atau permintaan contoh. Ditambahkan ke `description` dalam daftar skill dan dihitung terhadap batas 1.536 karakter.                                                                                                                                                                                                                                                        |
| `argument-hint`            | Tidak            | Petunjuk yang ditampilkan selama autocomplete untuk menunjukkan argumen yang diharapkan. Contoh: `[issue-number]` atau `[filename] [format]`.                                                                                                                                                                                                                                                                                                                    |
| `arguments`                | Tidak            | Argumen posisional bernama untuk [substitusi `$name`](#available-string-substitutions) dalam konten skill. Menerima string yang dipisahkan spasi atau daftar YAML. Nama memetakan ke posisi argumen secara berurutan.                                                                                                                                                                                                                                            |
| `disable-model-invocation` | Tidak            | Atur ke `true` untuk mencegah Claude memuat skill ini secara otomatis. Gunakan untuk workflow yang ingin Anda picu secara manual dengan `/name`. Juga mencegah skill dari [dimuat sebelumnya ke dalam subagents](/docs/id/sub-agents#preload-skills-into-subagents). Mulai dari v2.1.196, juga mencegah skill berjalan saat [tugas terjadwal](/docs/id/scheduled-tasks) dijalankan dengan skill sebagai promptnya. Default: `false`.                                       |
| `user-invocable`           | Tidak            | Atur ke `false` untuk menyembunyikan dari menu `/`. Gunakan untuk pengetahuan latar belakang yang tidak boleh diinvokasinya pengguna secara langsung. Default: `true`.                                                                                                                                                                                                                                                                                           |
| `allowed-tools`            | Tidak            | Tools yang dapat digunakan Claude tanpa meminta izin saat skill ini aktif. Menerima string yang dipisahkan spasi atau koma, atau daftar YAML.                                                                                                                                                                                                                                                                                                                    |
| `disallowed-tools`         | Tidak            | Tools yang dihapus dari kumpulan tools yang tersedia Claude saat skill ini aktif. Gunakan untuk skills otonom yang tidak boleh memanggil tools tertentu, seperti `AskUserQuestion` untuk loop latar belakang. Menerima string yang dipisahkan spasi atau koma, atau daftar YAML. Pembatasan dihapus saat Anda mengirim pesan berikutnya.                                                                                                                         |
| `model`                    | Tidak            | Model yang digunakan saat skill ini aktif. Penggantian berlaku untuk sisa giliran saat ini dan tidak disimpan ke pengaturan; model sesi dilanjutkan pada prompt Anda berikutnya. Menerima nilai yang sama seperti [`/model`](/docs/id/model-config), atau `inherit` untuk menjaga model aktif. Nilai yang dikecualikan oleh allowlist [`availableModels`](/docs/id/model-config#restrict-model-selection) organisasi Anda tidak digunakan dan sesi menjaga model saat ini. |
| `effort`                   | Tidak            | [Effort level](/docs/id/model-config#adjust-effort-level) saat skill ini aktif. Mengganti effort level sesi. Default: mewarisi dari sesi. Opsi: `low`, `medium`, `high`, `xhigh`, `max`; level yang tersedia tergantung pada model.                                                                                                                                                                                                                                   |
| `context`                  | Tidak            | Atur ke `fork` untuk menjalankan dalam konteks subagent yang di-fork.                                                                                                                                                                                                                                                                                                                                                                                            |
| `agent`                    | Tidak            | Jenis subagent mana yang digunakan saat `context: fork` diatur.                                                                                                                                                                                                                                                                                                                                                                                                  |
| `hooks`                    | Tidak            | Hooks yang dibatasi pada lifecycle skill ini. Lihat [Hooks dalam skills dan agents](/docs/id/hooks#hooks-in-skills-and-agents) untuk format konfigurasi.                                                                                                                                                                                                                                                                                                              |
| `paths`                    | Tidak            | Pola glob yang membatasi kapan skill ini diaktifkan. Menerima string yang dipisahkan koma atau daftar YAML. Ketika diatur, Claude memuat skill secara otomatis hanya saat bekerja dengan file yang cocok dengan pola. Menggunakan format yang sama seperti [aturan khusus path](/docs/id/memory#path-specific-rules).                                                                                                                                                 |
| `shell`                    | Tidak            | Shell yang digunakan untuk `` !`command` `` dan ` ```! ` blocks dalam skill ini. Menerima `bash` (default) atau `powershell`. Mengatur `powershell` menjalankan perintah shell inline melalui PowerShell di Windows. Memerlukan `CLAUDE_CODE_USE_POWERSHELL_TOOL=1`.                                                                                                                                                                                             |

<h4 id="how-a-skill-gets-its-command-name">
  Bagaimana skill mendapatkan nama perintahnya
</h4>

Perintah yang Anda ketik untuk menginvokasinya skill berasal dari tempat file skill berada. Bidang frontmatter `name` menetapkan label tampilan yang ditampilkan dalam daftar skill dan, kecuali untuk plugin-root `SKILL.md`, tidak mengubah apa yang Anda ketik setelah `/`.

Tabel di bawah menunjukkan dari mana nama perintah berasal untuk setiap tata letak:

| Lokasi skill                                                                                       | Sumber nama perintah                                                               | Contoh                                                                                                                                    |
| :------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------- |
| Direktori skill di bawah `~/.claude/skills/` atau `.claude/skills/`                                | Nama direktori                                                                     | `.claude/skills/deploy-staging/SKILL.md` → `/deploy-staging`                                                                              |
| [Bersarang](#where-skills-live) direktori `.claude/skills/`, ketika nama bentrok dengan skill lain | Jalur subdirektori relatif terhadap direktori kerja, kemudian nama direktori skill | `apps/web/.claude/skills/deploy/SKILL.md` → `/apps/web:deploy`                                                                            |
| File di bawah `.claude/commands/`                                                                  | Nama file tanpa ekstensi                                                           | `.claude/commands/deploy.md` → `/deploy`                                                                                                  |
| Subdirektori `skills/` plugin                                                                      | Nama direktori, diberi namespace oleh plugin                                       | `my-plugin/skills/review/SKILL.md` → `/my-plugin:review`                                                                                  |
| Plugin root `SKILL.md`                                                                             | Frontmatter `name`, dengan nama direktori plugin sebagai fallback                  | `my-plugin/SKILL.md` dengan `name: review` → `/my-plugin:review`. Lihat [Aturan perilaku path](/docs/id/plugins-reference#path-behavior-rules) |

Kasus plugin-root adalah satu-satunya tempat di mana `name` menetapkan nama perintah, karena tidak ada direktori skill untuk mengambilnya. Jika `name` tidak diatur dalam frontmatter, nama direktori plugin digunakan sebagai gantinya.

<h4 id="available-string-substitutions">
  Substitusi string yang tersedia
</h4>

Skills mendukung substitusi string untuk nilai dinamis dalam konten skill:

| Variabel                | Deskripsi                                                                                                                                                                                                                                                                                                                |
| :---------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `$ARGUMENTS`            | Semua argumen yang dilewatkan saat menginvokasinya skill. Jika `$ARGUMENTS` tidak ada dalam konten, argumen ditambahkan sebagai `ARGUMENTS: <value>`.                                                                                                                                                                    |
| `$ARGUMENTS[N]`         | Akses argumen spesifik berdasarkan indeks berbasis 0, seperti `$ARGUMENTS[0]` untuk argumen pertama.                                                                                                                                                                                                                     |
| `$N`                    | Singkat untuk `$ARGUMENTS[N]`, seperti `$0` untuk argumen pertama atau `$1` untuk argumen kedua.                                                                                                                                                                                                                         |
| `$name`                 | Argumen bernama yang dideklarasikan dalam daftar frontmatter [`arguments`](#frontmatter-reference). Nama memetakan ke posisi secara berurutan, jadi dengan `arguments: [issue, branch]` placeholder `$issue` berkembang menjadi argumen pertama dan `$branch` menjadi argumen kedua.                                     |
| `${CLAUDE_SESSION_ID}`  | ID sesi saat ini. Berguna untuk logging, membuat file khusus sesi, atau mengkorelasikan output skill dengan sesi.                                                                                                                                                                                                        |
| `${CLAUDE_EFFORT}`      | Effort level saat ini: `low`, `medium`, `high`, `xhigh`, atau `max`. Ultracode bukan level yang berbeda dan dilaporkan sebagai `xhigh`. Gunakan ini untuk menyesuaikan instruksi skill dengan pengaturan effort aktif.                                                                                                   |
| `${CLAUDE_SKILL_DIR}`   | Direktori yang berisi file `SKILL.md` skill. Untuk skills plugin, ini adalah subdirektori skill dalam plugin, bukan root plugin. Gunakan ini dalam perintah injeksi bash untuk mereferensikan script atau file yang dikemas dengan skill, terlepas dari direktori kerja saat ini.                                        |
| `${CLAUDE_PROJECT_DIR}` | Direktori root proyek. Ini adalah path yang sama yang diterima [hooks](/docs/id/hooks#reference-scripts-by-path) dan server MCP sebagai `CLAUDE_PROJECT_DIR`. Gunakan ini untuk mereferensikan script atau file lokal proyek, seperti `${CLAUDE_PROJECT_DIR}/.claude/hooks/helper.sh`, independen dari tempat skill diinstal. |

Substitusi `${CLAUDE_PROJECT_DIR}` memerlukan Claude Code v2.1.196 atau lebih baru. Ini berlaku untuk body skill dan frontmatter [`allowed-tools`](#frontmatter-reference), jadi aturan izin seperti `Bash(${CLAUDE_PROJECT_DIR}/scripts/lint.sh *)` diselesaikan ke path yang sama yang digunakan body skill.

Argumen yang diindeks menggunakan quoting gaya shell, jadi bungkus nilai multi-kata dalam tanda kutip untuk meneruskannya sebagai argumen tunggal. Misalnya, `/my-skill "hello world" second` membuat `$0` berkembang menjadi `hello world` dan `$1` menjadi `second`. Placeholder `$ARGUMENTS` selalu berkembang menjadi string argumen lengkap seperti yang diketik.

Untuk menyertakan literal `$` sebelum digit, `ARGUMENTS`, atau nama argumen yang dideklarasikan, seperti `$1.00` dalam prosa, escape dengan backslash: `\$1.00`. Backslash sebelum `$` lainnya dibiarkan tidak berubah. Hanya satu backslash langsung sebelum token yang escape-nya. Backslash ganda seperti `\\$1` meninggalkan kedua backslash di tempat, dan `$1` masih berkembang menjadi nilai argumen.

**Contoh menggunakan substitusi:**

```yaml theme={null}
---
name: session-logger
description: Log activity for this session
---

Log the following to logs/${CLAUDE_SESSION_ID}.log:

$ARGUMENTS
```

<h3 id="add-supporting-files">
  Tambahkan file pendukung
</h3>

Skills dapat menyertakan beberapa file di direktorinya. Ini menjaga `SKILL.md` tetap fokus pada hal-hal penting sambil membiarkan Claude mengakses materi referensi terperinci hanya saat diperlukan. Dokumen referensi besar, spesifikasi API, atau koleksi contoh tidak perlu dimuat ke dalam konteks setiap kali skill berjalan.

```text theme={null}
my-skill/
├── SKILL.md (required - overview and navigation)
├── reference.md (detailed API docs - loaded when needed)
├── examples.md (usage examples - loaded when needed)
└── scripts/
    └── helper.py (utility script - executed, not loaded)
```

Referensikan file pendukung dari `SKILL.md` Anda sehingga Claude tahu apa yang berisi setiap file dan kapan memuatnya:

```markdown theme={null}
## Additional resources

- For complete API details, see [reference.md](reference.md)
- For usage examples, see [examples.md](examples.md)
```

<Tip>Jaga `SKILL.md` di bawah 500 baris. Pindahkan materi referensi terperinci ke file terpisah.</Tip>

<h3 id="control-who-invokes-a-skill">
  Kontrol siapa yang menginvokasinya skill
</h3>

Secara default, baik Anda maupun Claude dapat menginvokasinya skill apa pun. Anda dapat mengetik `/skill-name` untuk menginvokasinya secara langsung, dan Claude dapat memuatnya secara otomatis saat relevan dengan percakapan Anda. Dua bidang frontmatter memungkinkan Anda membatasi ini:

* **`disable-model-invocation: true`**: Hanya Anda yang dapat menginvokasinya skill. Gunakan ini untuk workflow dengan efek samping atau yang ingin Anda kontrol waktu, seperti `/commit`, `/deploy`, atau `/send-slack-message`. Anda tidak ingin Claude memutuskan untuk deploy karena kode Anda terlihat siap.

* **`user-invocable: false`**: Hanya Claude yang dapat menginvokasinya skill. Gunakan ini untuk pengetahuan latar belakang yang tidak dapat ditindaklanjuti sebagai perintah. Skill `legacy-system-context` menjelaskan bagaimana sistem lama bekerja. Claude harus tahu ini saat relevan, tetapi `/legacy-system-context` bukan tindakan yang bermakna bagi pengguna untuk diambil.

Contoh ini membuat skill deploy yang hanya dapat Anda picu. Jika Anda menetapkan `disable-model-invocation: true`, Claude tidak dapat menjalankan skill secara otomatis:

```yaml theme={null}
---
name: deploy
description: Deploy the application to production
disable-model-invocation: true
---

Deploy $ARGUMENTS to production:

1. Run the test suite
2. Build the application
3. Push to the deployment target
4. Verify the deployment succeeded
```

Berikut adalah bagaimana dua bidang mempengaruhi invokasi dan pemuatan konteks:

| Frontmatter                      | Anda dapat menginvokasinya | Claude dapat menginvokasinya | Saat dimuat ke dalam konteks                                                |
| :------------------------------- | :------------------------- | :--------------------------- | :-------------------------------------------------------------------------- |
| (default)                        | Ya                         | Ya                           | Deskripsi selalu dalam konteks, skill penuh dimuat saat diinvokasinya       |
| `disable-model-invocation: true` | Ya                         | Tidak                        | Deskripsi tidak dalam konteks, skill penuh dimuat saat Anda menginvokasinya |
| `user-invocable: false`          | Tidak                      | Ya                           | Deskripsi selalu dalam konteks, skill penuh dimuat saat diinvokasinya       |

<Note>
  Dalam sesi reguler, deskripsi skill dimuat ke dalam konteks sehingga Claude tahu apa yang tersedia, tetapi konten skill penuh hanya dimuat saat diinvokasinya. [Subagents dengan skills yang dimuat sebelumnya](/docs/id/sub-agents#preload-skills-into-subagents) bekerja berbeda: konten skill penuh disuntikkan saat startup.
</Note>

<h3 id="skill-content-lifecycle">
  Lifecycle konten skill
</h3>

Saat Anda atau Claude menginvokasinya skill, konten `SKILL.md` yang dirender memasuki percakapan sebagai pesan tunggal dan tetap di sana untuk sisa sesi. Claude Code tidak membaca ulang file skill pada giliran berikutnya, jadi tulis panduan yang harus berlaku sepanjang tugas sebagai instruksi berdiri daripada langkah satu kali.

Ketika Claude menginvokasinya kembali skill yang konten yang dirender identik dengan salinan yang sudah ada dalam konteks, Claude Code menambahkan catatan singkat bahwa skill sudah dimuat daripada salinan kedua dari konten. Ketika konten yang dirender berbeda, karena argumen berubah atau perintah [konteks dinamis](#inject-dynamic-context) menghasilkan output baru, Claude Code menambahkan konten penuh lagi. Sebelum v2.1.202, setiap re-invokasi menambahkan salinan lain dari instruksi skill.

[Auto-compaction](/docs/id/how-claude-code-works#when-context-fills-up) membawa skills yang diinvokasinya maju dalam anggaran token. Ketika percakapan dirangkum untuk membebaskan konteks, Claude Code melampirkan kembali invokasi skill terbaru setelah ringkasan, menjaga 5.000 token pertama dari masing-masing. Skills yang dilampirkan kembali berbagi anggaran gabungan 25.000 token. Claude Code mengisi anggaran ini mulai dari skill yang paling baru diinvokasinya, jadi skills yang lebih lama dapat dijatuhkan sepenuhnya setelah compaction jika Anda telah menginvokasinya banyak dalam satu sesi.

Jika skill tampaknya berhenti mempengaruhi perilaku setelah respons pertama, konten biasanya masih ada dan model memilih tools atau pendekatan lain. Perkuat deskripsi skill dan instruksi sehingga model terus menyukainya, atau gunakan [hooks](/docs/id/hooks) untuk menerapkan perilaku secara deterministik. Jika skill besar atau Anda menginvokasinya beberapa yang lain setelahnya, invokasinya kembali setelah compaction untuk mengembalikan konten penuh.

<h3 id="pre-approve-tools-for-a-skill">
  Pra-setujui tools untuk skill
</h3>

Bidang `allowed-tools` memberikan izin untuk tools yang terdaftar saat skill aktif, sehingga Claude dapat menggunakannya tanpa meminta persetujuan Anda. Ini tidak membatasi tools mana yang tersedia: setiap tool tetap dapat dipanggil, dan [pengaturan izin](/docs/id/permissions) Anda masih mengatur tools yang tidak terdaftar.

Untuk skills yang diperiksa ke dalam direktori `.claude/skills/` proyek, `allowed-tools` berlaku setelah Anda menerima dialog kepercayaan workspace untuk folder itu, sama seperti aturan izin dalam `.claude/settings.json`. Tinjau skills proyek sebelum mempercayai repositori, karena skill dapat memberikan dirinya sendiri akses tools yang luas.

Skill ini memungkinkan Claude menjalankan perintah git tanpa persetujuan per-penggunaan kapan pun Anda menginvokasinya:

```yaml theme={null}
---
name: commit
description: Stage and commit the current changes
disable-model-invocation: true
allowed-tools: Bash(git add *) Bash(git commit *) Bash(git status *)
---
```

Untuk menghapus tools dari kumpulan tools yang tersedia Claude saat skill aktif, daftarkan mereka dalam `disallowed-tools` di frontmatter skill. Pembatasan dihapus saat Anda mengirim pesan berikutnya. Untuk memblokir tools di semua skills dan prompts, tambahkan aturan deny dalam [pengaturan izin](/docs/id/permissions) Anda.

<h3 id="pass-arguments-to-skills">
  Lewatkan argumen ke skills
</h3>

Baik Anda maupun Claude dapat melewatkan argumen saat menginvokasinya skill. Argumen tersedia melalui placeholder `$ARGUMENTS`.

Skill ini memperbaiki masalah GitHub berdasarkan nomor. Placeholder `$ARGUMENTS` diganti dengan apa pun yang mengikuti nama skill:

```yaml theme={null}
---
name: fix-issue
description: Fix a GitHub issue
disable-model-invocation: true
---

Fix GitHub issue $ARGUMENTS following our coding standards.

1. Read the issue description
2. Understand the requirements
3. Implement the fix
4. Write tests
5. Create a commit
```

Saat Anda menjalankan `/fix-issue 123`, Claude menerima "Fix GitHub issue 123 following our coding standards..."

Jika Anda menginvokasinya skill dengan argumen tetapi skill tidak menyertakan `$ARGUMENTS`, Claude Code menambahkan `ARGUMENTS: <your input>` ke akhir konten skill sehingga Claude masih melihat apa yang Anda ketik.

Anda juga dapat menumpuk beberapa skills di awal satu pesan. Mulai dari v2.1.199, mengetik `/code-review /fix-issue 123` memuat kedua skills dan meneruskan teks trailing `123` sebagai `$ARGUMENTS` ke masing-masing. Dalam versi sebelumnya, hanya skill pertama yang dimuat dan menerima `/fix-issue 123` sebagai teks argumen literal.

Claude Code memperluas skill pertama ditambah hingga lima lagi yang ditumpuk setelahnya. Ekspansi berhenti pada token pertama yang bukan skill yang dapat diinvokasinya inline, jadi skill yang berjalan sebagai [subagent yang di-fork](#run-skills-in-a-subagent) atau yang argumennya mungkin sendiri dimulai dengan perintah slash, seperti `/loop`, juga berakhir di sana; token itu dan semuanya setelahnya menjadi teks argumen untuk setiap skill yang diperluas.

Untuk mengakses argumen individual berdasarkan posisi, gunakan `$ARGUMENTS[N]` atau yang lebih pendek `$N`:

```yaml theme={null}
---
name: migrate-component
description: Migrate a component from one framework to another
---

Migrate the $ARGUMENTS[0] component from $ARGUMENTS[1] to $ARGUMENTS[2].
Preserve all existing behavior and tests.
```

Menjalankan `/migrate-component SearchBar React Vue` mengganti `$ARGUMENTS[0]` dengan `SearchBar`, `$ARGUMENTS[1]` dengan `React`, dan `$ARGUMENTS[2]` dengan `Vue`. Skill yang sama menggunakan shorthand `$N`:

```yaml theme={null}
---
name: migrate-component
description: Migrate a component from one framework to another
---

Migrate the $0 component from $1 to $2.
Preserve all existing behavior and tests.
```

<h2 id="advanced-patterns">
  Pola lanjutan
</h2>

<h3 id="inject-dynamic-context">
  Injeksi konteks dinamis
</h3>

Sintaks `` !`<command>` `` menjalankan perintah shell sebelum konten skill dikirim ke Claude. Output perintah mengganti placeholder, sehingga Claude menerima data aktual, bukan perintah itu sendiri.

Skill ini merangkum pull request dengan mengambil data PR langsung dengan GitHub CLI. Perintah `` !`gh pr diff` `` dan lainnya berjalan terlebih dahulu, dan output mereka dimasukkan ke dalam prompt:

```yaml theme={null}
---
name: pr-summary
description: Summarize changes in a pull request
context: fork
agent: Explore
allowed-tools: Bash(gh *)
---

## Pull request context
- PR diff: !`gh pr diff`
- PR comments: !`gh pr view --comments`
- Changed files: !`gh pr diff --name-only`

## Your task
Summarize this pull request...
```

Saat skill ini berjalan:

1. Setiap `` !`<command>` `` dijalankan segera (sebelum Claude melihat apa pun)
2. Output mengganti placeholder dalam konten skill
3. Claude menerima prompt yang sepenuhnya dirender dengan data PR aktual

Ini adalah preprocessing, bukan sesuatu yang dijalankan Claude. Claude hanya melihat hasil akhir.

Substitusi berjalan sekali di atas file asli. Output perintah dimasukkan sebagai teks biasa dan tidak dipindai ulang untuk placeholder `` !`<command>` `` lebih lanjut, jadi perintah tidak dapat mengeluarkan placeholder untuk pass yang lebih lambat untuk diperluas.

Bentuk inline hanya dikenali ketika `!` muncul di awal baris atau segera setelah whitespace. Jika `!` mengikuti karakter lain, seperti dalam `` KEY=!`cmd` ``, placeholder dibiarkan sebagai teks literal dan perintah tidak berjalan.

Untuk perintah multi-baris, gunakan blok kode yang dibuka dengan ` ```! ` sebagai gantinya dari bentuk inline:

````markdown theme={null}
## Environment
```!
node --version
npm --version
git status --short
```
````

Untuk menonaktifkan perilaku ini untuk skills dan perintah kustom dari sumber pengguna, proyek, plugin, atau [direktori tambahan](#skills-from-additional-directories), atur `"disableSkillShellExecution": true` dalam [pengaturan](/docs/id/settings). Setiap perintah diganti dengan `[shell command execution disabled by policy]` sebagai gantinya dijalankan. Skills bundel dan terkelola tidak terpengaruh. Pengaturan ini paling berguna dalam [pengaturan terkelola](/docs/id/permissions#managed-settings), di mana pengguna tidak dapat menimpanya.

<Tip>
  Untuk meminta penalaran yang lebih dalam saat skill berjalan, sertakan `ultrathink` di mana pun dalam konten skill. Lihat [Gunakan ultrathink untuk penalaran mendalam sekali jalan](/docs/id/model-config#use-ultrathink-for-one-off-deep-reasoning).
</Tip>

<h3 id="run-skills-in-a-subagent">
  Jalankan skills dalam subagent
</h3>

Tambahkan `context: fork` ke frontmatter Anda saat Anda ingin skill berjalan dalam isolasi. Konten skill menjadi prompt yang mendorong subagent. Ini tidak akan memiliki akses ke riwayat percakapan Anda.

<Warning>
  `context: fork` hanya masuk akal untuk skills dengan instruksi eksplisit. Jika skill Anda berisi panduan seperti "gunakan konvensi API ini" tanpa tugas, subagent menerima panduan tetapi tidak ada prompt yang dapat ditindaklanjuti, dan kembali tanpa output yang bermakna.
</Warning>

Skills dan [subagents](/docs/id/sub-agents) bekerja bersama dalam dua arah:

| Pendekatan                      | System prompt           | Tugas                 | Juga memuat                                             |
| :------------------------------ | :---------------------- | :-------------------- | :------------------------------------------------------ |
| Skill dengan `context: fork`    | Dari jenis agen         | Konten SKILL.md       | CLAUDE.md, kecuali ketika agen adalah Explore atau Plan |
| Subagent dengan bidang `skills` | Badan markdown subagent | Pesan delegasi Claude | Skills yang dimuat sebelumnya + CLAUDE.md               |

Dengan `context: fork`, Anda menulis tugas dalam skill Anda dan memilih jenis agen untuk menjalankannya. Agen Explore dan Plan bawaan [melewati CLAUDE.md dan git status](/docs/id/sub-agents#what-loads-at-startup) untuk menjaga konteks mereka tetap kecil, jadi skill yang di-fork menggunakan `agent: Explore` hanya melihat konten SKILL.md dan system prompt agen itu sendiri. Untuk kebalikannya, di mana Anda mendefinisikan subagent kustom yang menggunakan skills sebagai materi referensi, lihat [Subagents](/docs/id/sub-agents#preload-skills-into-subagents).

<h4 id="example-research-skill-using-explore-agent">
  Contoh: Skill penelitian menggunakan agen Explore
</h4>

Skill ini menjalankan penelitian dalam agen Explore yang di-fork. Konten skill menjadi tugas, dan agen menyediakan tools baca-saja yang dioptimalkan untuk eksplorasi codebase:

```yaml theme={null}
---
name: deep-research
description: Research a topic thoroughly
context: fork
agent: Explore
---

Research $ARGUMENTS thoroughly:

1. Find relevant files using Glob and Grep
2. Read and analyze the code
3. Summarize findings with specific file references
```

Saat skill ini berjalan:

1. Konteks terisolasi baru dibuat
2. Subagent menerima konten skill sebagai promptnya ("Research \$ARGUMENTS thoroughly...")
3. Bidang `agent` menentukan lingkungan eksekusi (model, tools, dan izin)
4. Hasil dirangkum dan dikembalikan ke percakapan utama Anda

Bidang `agent` menentukan konfigurasi subagent mana yang digunakan. Opsi termasuk agen bawaan (`Explore`, `Plan`, `general-purpose`) atau subagent kustom apa pun dari `.claude/agents/`. Jika dihilangkan, menggunakan `general-purpose`.

<h3 id="restrict-claude’s-skill-access">
  Batasi akses skill Claude
</h3>

Secara default, Claude dapat menginvokasinya skill apa pun yang tidak memiliki `disable-model-invocation: true` diatur. Skills yang mendefinisikan `allowed-tools` memberikan Claude akses ke tools tersebut tanpa persetujuan per-penggunaan saat skill aktif. Pengaturan [izin](/docs/id/permissions) Anda masih mengatur perilaku persetujuan baseline untuk semua tools lainnya. Beberapa perintah bawaan juga tersedia melalui tool Skill, termasuk `/init`, `/review`, dan `/security-review`. Perintah bawaan lainnya seperti `/compact` tidak.

Tiga cara untuk mengontrol skills mana yang dapat diinvokasinya Claude:

**Nonaktifkan semua skills** dengan menolak tool Skill di `/permissions`:

```text theme={null}
# Add to deny rules:
Skill
```

**Izinkan atau tolak skills spesifik** menggunakan [aturan izin](/docs/id/permissions):

```text theme={null}
# Allow only specific skills
Skill(commit)
Skill(review-pr *)

# Deny specific skills
Skill(deploy *)
```

Sintaks izin: `Skill(name)` untuk kecocokan tepat, `Skill(name *)` untuk kecocokan awalan dengan argumen apa pun.

**Sembunyikan skills individual** dengan menambahkan `disable-model-invocation: true` ke frontmatter mereka. Ini menghapus skill dari konteks Claude sepenuhnya.

<Note>
  Bidang `user-invocable` hanya mengontrol visibilitas menu, bukan akses tool Skill. Gunakan `disable-model-invocation: true` untuk memblokir invokasi programatik.
</Note>

<h3 id="override-skill-visibility-from-settings">
  Ganti visibilitas skill dari pengaturan
</h3>

Pengaturan `skillOverrides` mengontrol visibilitas skill dari [pengaturan](/docs/id/settings) Anda sebagai gantinya dari frontmatter skill itu sendiri. Gunakan untuk skills yang SKILL.md-nya Anda tidak ingin edit, seperti yang diperiksa ke dalam repo proyek bersama atau disediakan oleh server MCP. Menu `/skills` menulisnya untuk Anda: sorot skill dan tekan `Space` untuk mengubah status, kemudian `Enter` untuk menyimpan ke `.claude/settings.local.json`.

Setiap kunci adalah nama skill dan setiap nilai adalah salah satu dari empat status:

| Nilai                   | Terdaftar ke Claude | Dalam menu `/` |
| :---------------------- | :------------------ | :------------- |
| `"on"`                  | Nama dan deskripsi  | Ya             |
| `"name-only"`           | Nama saja           | Ya             |
| `"user-invocable-only"` | Tersembunyi         | Ya             |
| `"off"`                 | Tersembunyi         | Tersembunyi    |

Skill yang tidak ada dalam `skillOverrides` diperlakukan sebagai `"on"`. Contoh di bawah ini menciutkan satu skill menjadi namanya dan mematikan yang lain sepenuhnya:

```json theme={null}
{
  "skillOverrides": {
    "legacy-context": "name-only",
    "deploy": "off"
  }
}
```

Plugin skills tidak terpengaruh oleh `skillOverrides`. Kelola yang melalui `/plugin` sebagai gantinya.

<h2 id="evaluate-and-iterate-on-a-skill">
  Evaluasi dan iterasi pada skill
</h2>

Melihat skill terpicu memberi tahu Anda Claude menemukannya, bukan bahwa itu melakukan apa yang Anda maksudkan. Untuk mengetahui skill bekerja, ukur dua hal secara terpisah: apakah Claude menginvokasinya pada prompt yang seharusnya, dan apakah output cocok dengan apa yang Anda harapkan saat itu.

Pemeriksaan untuk keduanya adalah perbandingan baseline. Kumpulkan beberapa prompt realistis, jalankan masing-masing dalam sesi segar dengan skill tersedia dan lagi dengan itu [dinonaktifkan](#override-skill-visibility-from-settings), dan bandingkan hasilnya. Sesi segar penting karena konteks sisa dari penulisan skill akan menyembunyikan celah dalam instruksi tertulis.

<h3 id="run-evals-with-skill-creator">
  Jalankan evals dengan skill-creator
</h3>

Plugin [`skill-creator`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/skill-creator) mengotomatisasi loop perbandingan di dalam Claude Code. Instal dari marketplace resmi:

```text theme={null}
/plugin install skill-creator@claude-plugins-official
```

Jika Claude Code melaporkan bahwa plugin tidak ditemukan di marketplace apa pun, marketplace Anda hilang atau ketinggalan zaman. Jalankan `/plugin marketplace update claude-plugins-official` untuk menyegarkannya, atau `/plugin marketplace add anthropics/claude-plugins-official` jika Anda belum menambahkannya. Kemudian coba instal lagi.

Setelah menginstal, jalankan `/reload-plugins` untuk membuat skills plugin tersedia dalam sesi saat ini. Kemudian minta Claude untuk mengevaluasi skill yang ada, misalnya `evaluate my summarize-changes skill with skill-creator`. Plugin memandu Anda melalui penulisan kasus uji dan menjalankan loop:

* **Kasus uji**: menyimpan prompt, file input, dan perilaku yang diharapkan di `evals/evals.json` di dalam direktori skill
* **Jalankan terisolasi**: menghasilkan [subagent](/docs/id/sub-agents) per kasus uji sehingga setiap jalankan dimulai dengan konteks bersih, dan merekam jumlah token dan durasi
* **Penilaian**: memeriksa setiap pernyataan terhadap output dan menulis lulus atau gagal dengan bukti ke `grading.json`
* **Benchmark**: mengagregasi tingkat lulus, waktu, dan token untuk dengan-skill versus tanpa-skill ke `benchmark.json` sehingga Anda dapat membandingkan peningkatan tingkat lulus terhadap overhead token dan waktu
* **Perbandingan versi**: menjalankan blind A/B antara dua versi skill sehingga Anda dapat mengonfirmasi edit adalah peningkatan sebelum melakukannya
* **Penyesuaian deskripsi**: menghasilkan prompt should-trigger dan should-not-trigger, mengukur tingkat hit, dan mengusulkan edit deskripsi saat skill mengaktifkan pada permintaan yang salah
* **Penampil ulasan**: membuka laporan HTML di mana Anda memeriksa setiap output dan merekam umpan balik kualitatif yang dibaca iterasi berikutnya

Untuk format file eval dan alur kerja iterasi lengkap, lihat [Mengevaluasi kualitas output skill](https://agentskills.io/skill-creation/evaluating-skills) di agentskills.io. Untuk latar belakang pada mode benchmark dan perbandingan, lihat [pengumuman skill-creator](https://claude.com/blog/improving-skill-creator-test-measure-and-refine-agent-skills).

<h2 id="share-skills">
  Bagikan skills
</h2>

Skills dapat didistribusikan pada cakupan berbeda tergantung pada audiens Anda:

* **Skills proyek**: Commit `.claude/skills/` ke version control
* **Plugins**: Buat direktori `skills/` dalam [plugin](/docs/id/plugins) Anda
* **Terkelola**: Terapkan di seluruh organisasi melalui [pengaturan terkelola](/docs/id/settings#settings-files)

<h3 id="generate-visual-output">
  Hasilkan output visual
</h3>

Skills dapat membundel dan menjalankan script dalam bahasa apa pun, memberikan Claude kemampuan di luar apa yang mungkin dalam prompt tunggal. Satu pola yang kuat adalah menghasilkan output visual: file HTML interaktif yang terbuka di browser Anda untuk menjelajahi data, debugging, atau membuat laporan.

Contoh ini membuat penjelajah codebase: tampilan pohon interaktif di mana Anda dapat memperluas dan menciutkan direktori, melihat ukuran file sekilas, dan mengidentifikasi jenis file berdasarkan warna.

Buat direktori Skill:

```bash theme={null}
mkdir -p ~/.claude/skills/codebase-visualizer/scripts
```

Simpan ini ke `~/.claude/skills/codebase-visualizer/SKILL.md`. Deskripsi memberi tahu Claude kapan mengaktifkan Skill ini, dan instruksi memberi tahu Claude untuk menjalankan script yang dikemas. Jalur script menggunakan [`${CLAUDE_SKILL_DIR}`](#available-string-substitutions) sehingga dapat diselesaikan dengan benar apakah skill diinstal pada tingkat personal, proyek, atau plugin:

````yaml theme={null}
---
name: codebase-visualizer
description: Generate an interactive collapsible tree visualization of your codebase. Use when exploring a new repo, understanding project structure, or identifying large files.
allowed-tools: Bash(python3 *)
---

# Codebase Visualizer

Generate an interactive HTML tree view that shows your project's file structure with collapsible directories.

## Usage

Run the visualization script from your project root:

```bash
python3 ${CLAUDE_SKILL_DIR}/scripts/visualize.py .
```

This creates `codebase-map.html` in the current directory and opens it in your default browser.

## What the visualization shows

- **Collapsible directories**: Click folders to expand/collapse
- **File sizes**: Displayed next to each file
- **Colors**: Different colors for different file types
- **Directory totals**: Shows aggregate size of each folder
````

Simpan ini ke `~/.claude/skills/codebase-visualizer/scripts/visualize.py`. Script ini memindai pohon direktori dan menghasilkan file HTML yang mandiri dengan:

* **Sidebar ringkasan** yang menunjukkan jumlah file, jumlah direktori, ukuran total, dan jumlah jenis file
* **Bagan batang** yang memecah codebase berdasarkan jenis file (8 teratas berdasarkan ukuran)
* **Pohon yang dapat diciutkan** di mana Anda dapat memperluas dan menciutkan direktori, dengan indikator jenis file berkode warna

Script memerlukan Python 3 tetapi hanya menggunakan library bawaan, jadi tidak ada paket yang perlu diinstal:

```python expandable theme={null}
#!/usr/bin/env python3
"""Generate an interactive collapsible tree visualization of a codebase."""

import json
import sys
import webbrowser
from html import escape
from pathlib import Path
from collections import Counter

IGNORE = {'.git', 'node_modules', '__pycache__', '.venv', 'venv', 'dist', 'build'}

def scan(path: Path, stats: dict) -> dict:
    result = {"name": path.name, "children": [], "size": 0}
    try:
        for item in sorted(path.iterdir()):
            if item.name in IGNORE or item.name.startswith('.'):
                continue
            if item.is_file():
                size = item.stat().st_size
                ext = item.suffix.lower() or '(no ext)'
                result["children"].append({"name": item.name, "size": size, "ext": ext})
                result["size"] += size
                stats["files"] += 1
                stats["extensions"][ext] += 1
                stats["ext_sizes"][ext] += size
            elif item.is_dir():
                stats["dirs"] += 1
                child = scan(item, stats)
                if child["children"]:
                    result["children"].append(child)
                    result["size"] += child["size"]
    except PermissionError:
        pass
    return result

def generate_html(data: dict, stats: dict, output: Path) -> None:
    ext_sizes = stats["ext_sizes"]
    total_size = sum(ext_sizes.values()) or 1
    sorted_exts = sorted(ext_sizes.items(), key=lambda x: -x[1])[:8]
    colors = {
        '.js': '#f7df1e', '.ts': '#3178c6', '.py': '#3776ab', '.go': '#00add8',
        '.rs': '#dea584', '.rb': '#cc342d', '.css': '#264de4', '.html': '#e34c26',
        '.json': '#6b7280', '.md': '#083fa1', '.yaml': '#cb171e', '.yml': '#cb171e',
        '.mdx': '#083fa1', '.tsx': '#3178c6', '.jsx': '#61dafb', '.sh': '#4eaa25',
    }
    lang_bars = "".join(
        f'<div class="bar-row"><span class="bar-label">{ext}</span>'
        f'<div class="bar" style="width:{(size/total_size)*100}%;background:{colors.get(ext,"#6b7280")}"></div>'
        f'<span class="bar-pct">{(size/total_size)*100:.1f}%</span></div>'
        for ext, size in sorted_exts
    )
    def fmt(b):
        if b < 1024: return f"{b} B"
        if b < 1048576: return f"{b/1024:.1f} KB"
        return f"{b/1048576:.1f} MB"

    html = f'''<!DOCTYPE html>
<html><head>
  <meta charset="utf-8"><title>Codebase Explorer</title>
  <style>
    body {{ font: 14px/1.5 system-ui, sans-serif; margin: 0; background: #1a1a2e; color: #eee; }}
    .container {{ display: flex; height: 100vh; }}
    .sidebar {{ width: 280px; background: #252542; padding: 20px; border-right: 1px solid #3d3d5c; overflow-y: auto; flex-shrink: 0; }}
    .main {{ flex: 1; padding: 20px; overflow-y: auto; }}
    h1 {{ margin: 0 0 10px 0; font-size: 18px; }}
    h2 {{ margin: 20px 0 10px 0; font-size: 14px; color: #888; text-transform: uppercase; }}
    .stat {{ display: flex; justify-content: space-between; padding: 8px 0; border-bottom: 1px solid #3d3d5c; }}
    .stat-value {{ font-weight: bold; }}
    .bar-row {{ display: flex; align-items: center; margin: 6px 0; }}
    .bar-label {{ width: 55px; font-size: 12px; color: #aaa; }}
    .bar {{ height: 18px; border-radius: 3px; }}
    .bar-pct {{ margin-left: 8px; font-size: 12px; color: #666; }}
    .tree {{ list-style: none; padding-left: 20px; }}
    details {{ cursor: pointer; }}
    summary {{ padding: 4px 8px; border-radius: 4px; }}
    summary:hover {{ background: #2d2d44; }}
    .folder {{ color: #ffd700; }}
    .file {{ display: flex; align-items: center; padding: 4px 8px; border-radius: 4px; }}
    .file:hover {{ background: #2d2d44; }}
    .size {{ color: #888; margin-left: auto; font-size: 12px; }}
    .dot {{ width: 8px; height: 8px; border-radius: 50%; margin-right: 8px; }}
  </style>
</head><body>
  <div class="container">
    <div class="sidebar">
      <h1>📊 Summary</h1>
      <div class="stat"><span>Files</span><span class="stat-value">{stats["files"]:,}</span></div>
      <div class="stat"><span>Directories</span><span class="stat-value">{stats["dirs"]:,}</span></div>
      <div class="stat"><span>Total size</span><span class="stat-value">{fmt(data["size"])}</span></div>
      <div class="stat"><span>File types</span><span class="stat-value">{len(stats["extensions"])}</span></div>
      <h2>By file type</h2>
      {lang_bars}
    </div>
    <div class="main">
      <h1>📁 {escape(data["name"])}</h1>
      <ul class="tree" id="root"></ul>
    </div>
  </div>
  <script>
    const data = {json.dumps(data)};
    const colors = {json.dumps(colors)};
    function fmt(b) {{ if (b < 1024) return b + ' B'; if (b < 1048576) return (b/1024).toFixed(1) + ' KB'; return (b/1048576).toFixed(1) + ' MB'; }}
    function esc(s) {{ return s.replace(/[&<>"']/g, c => ({{"&":"&amp;","<":"&lt;",">":"&gt;",'"':"&quot;","'":"&#39;"}}[c])); }}
    function render(node, parent) {{
      if (node.children) {{
        const det = document.createElement('details');
        det.open = parent === document.getElementById('root');
        det.innerHTML = `<summary><span class="folder">📁 ${{esc(node.name)}}</span><span class="size">${{fmt(node.size)}}</span></summary>`;
        const ul = document.createElement('ul'); ul.className = 'tree';
        node.children.sort((a,b) => (b.children?1:0)-(a.children?1:0) || a.name.localeCompare(b.name));
        node.children.forEach(c => render(c, ul));
        det.appendChild(ul);
        const li = document.createElement('li'); li.appendChild(det); parent.appendChild(li);
      }} else {{
        const li = document.createElement('li'); li.className = 'file';
        li.innerHTML = `<span class="dot" style="background:${{colors[node.ext]||'#6b7280'}}"></span>${{esc(node.name)}}<span class="size">${{fmt(node.size)}}</span>`;
        parent.appendChild(li);
      }}
    }}
    data.children.forEach(c => render(c, document.getElementById('root')));
  </script>
</body></html>'''
    output.write_text(html)

if __name__ == '__main__':
    target = Path(sys.argv[1] if len(sys.argv) > 1 else '.').resolve()
    stats = {"files": 0, "dirs": 0, "extensions": Counter(), "ext_sizes": Counter()}
    data = scan(target, stats)
    out = Path('codebase-map.html')
    generate_html(data, stats, out)
    print(f'Generated {out.absolute()}')
    webbrowser.open(f'file://{out.absolute()}')
```

Untuk menguji, buka Claude Code di proyek apa pun dan tanyakan "Visualize this codebase." Claude menjalankan script, menghasilkan `codebase-map.html`, dan membukanya di browser Anda.

Pola ini bekerja untuk output visual apa pun: grafik ketergantungan, laporan cakupan tes, dokumentasi API, atau visualisasi skema database. Script yang dikemas melakukan pekerjaan berat sementara Claude menangani orkestrasi.

<h2 id="troubleshooting">
  Troubleshooting
</h2>

<h3 id="skill-not-triggering">
  Skill tidak terpicu
</h3>

Jika Claude tidak menggunakan skill Anda saat diharapkan:

1. Periksa deskripsi mencakup kata kunci yang akan dikatakan pengguna secara alami
2. Verifikasi skill muncul di `What skills are available?`
3. Coba rephrase permintaan Anda agar lebih cocok dengan deskripsi
4. Invokasinya secara langsung dengan `/skill-name` jika skill dapat diinvokasinya pengguna

Jika frontmatter YAML tidak terbentuk dengan baik, Claude Code memuat badan skill dengan metadata kosong, jadi `/skill-name` masih berfungsi tetapi Claude tidak memiliki `description` untuk dicocokkan. Jalankan dengan `--debug` untuk melihat kesalahan parse.

<h3 id="skill-triggers-too-often">
  Skill terpicu terlalu sering
</h3>

Jika Claude menggunakan skill Anda saat Anda tidak menginginkannya:

1. Buat deskripsi lebih spesifik
2. Tambahkan `disable-model-invocation: true` jika Anda hanya menginginkan invokasi manual

<h3 id="skill-descriptions-are-cut-short">
  Deskripsi skill dipotong pendek
</h3>

Claude Code memuat daftar nama skill dan deskripsi ke dalam konteks sehingga Claude tahu apa yang tersedia. Daftar selalu berisi setiap nama skill, tetapi jika Anda memiliki banyak skills, Claude Code mempersingkat deskripsi agar sesuai dengan anggaran karakter daftar, yang dapat menghilangkan kata kunci yang Claude butuhkan untuk mencocokkan permintaan Anda. Anggaran diskalakan pada 1% dari jendela konteks model. Ketika daftar meluap, Claude Code menghapus deskripsi dimulai dengan skills yang Anda invokasinya paling sedikit, sehingga skills yang Anda gunakan paling banyak tetap mempertahankan teks lengkapnya.

Jalankan `/doctor` untuk estimasi biaya konteks daftar dan kontributor terbesarnya. Ketika daftar melebihi anggarannya, Claude Code juga menulis peringatan ke log debug, terlihat dengan [`--debug`](/docs/id/cli-reference#cli-flags).

Baris Skills di `/context` melaporkan ukuran daftar setelah anggaran diterapkan, sehingga cocok dengan apa yang diterima model. Sebelum v2.1.196, baris menghitung teks lengkap setiap deskripsi dan dapat menampilkan nilai beberapa kali lebih besar dari anggaran yang dikonfigurasi.

Untuk menaikkan anggaran, atur pengaturan [`skillListingBudgetFraction`](/docs/id/settings#available-settings) (misalnya `0.02` = 2%) atau variabel lingkungan `SLASH_COMMAND_TOOL_CHAR_BUDGET` ke jumlah karakter tetap. Untuk membebaskan anggaran bagi skills lainnya, atur entri prioritas rendah ke `"name-only"` di [`skillOverrides`](#override-skill-visibility-from-settings) sehingga mereka terdaftar tanpa deskripsi. Anda juga dapat memangkas teks `description` dan `when_to_use` di sumbernya: depankan kasus penggunaan utama, karena teks gabungan setiap entri dibatasi pada 1.536 karakter terlepas dari anggaran. Batas dapat dikonfigurasi dengan [`skillListingMaxDescChars`](/docs/id/settings#available-settings).

<h2 id="related-resources">
  Sumber daya terkait
</h2>

* **[Debug konfigurasi Anda](/docs/id/debug-your-config)**: diagnosis mengapa skill tidak muncul atau tidak terpicu
* **[Mengevaluasi kualitas output skill](https://agentskills.io/skill-creation/evaluating-skills)**: format file eval dan alur kerja iterasi di agentskills.io
* **[Praktik terbaik penulisan skill](https://platform.claude.com/docs/en/agents-and-tools/agent-skills/best-practices)**: panduan penulisan yang berlaku di seluruh produk Claude
* **[Subagents](/docs/id/sub-agents)**: delegasikan tugas ke agen khusus
* **[Plugins](/docs/id/plugins)**: paket dan distribusikan skills dengan ekstensi lainnya
* **[Hooks](/docs/id/hooks)**: otomatisasi workflow di sekitar peristiwa tool
* **[Memory](/docs/id/memory)**: kelola file CLAUDE.md untuk konteks persisten
* **[Commands](/docs/id/commands)**: referensi untuk perintah bawaan dan skills bundel
* **[Permissions](/docs/id/permissions)**: kontrol akses tool dan skill
* **[Claude Tag skills](https://claude.com/docs/claude-tag/admins/skills-repo)**: project skills yang di-commit ke repo juga dimuat ketika repo tersebut digunakan di saluran Claude Tag
