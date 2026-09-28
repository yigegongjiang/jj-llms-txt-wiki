> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Perluas Claude dengan skills

> Buat, kelola, dan bagikan skills untuk memperluas kemampuan Claude di Claude Code. Mencakup perintah kustom dan skills bundel.

Skills memperluas apa yang dapat dilakukan Claude. Buat file `SKILL.md` dengan instruksi, dan Claude menambahkannya ke toolkit-nya. Claude menggunakan skills ketika relevan, atau Anda dapat menginvokasinya secara langsung dengan `/skill-name`.

Buat skill ketika Anda terus menempel instruksi yang sama, checklist, atau prosedur multi-langkah ke dalam chat, atau ketika bagian dari CLAUDE.md telah berkembang menjadi prosedur daripada fakta. Tidak seperti konten CLAUDE.md, body skill hanya dimuat ketika digunakan, sehingga materi referensi yang panjang hampir tidak ada biayanya sampai Anda membutuhkannya.

<Note>
  Untuk perintah bawaan seperti `/help` dan `/compact`, dan skills bundel seperti `/debug` dan `/code-review`, lihat [referensi perintah](/docs/id/commands).

  **Perintah kustom telah digabungkan ke dalam skills.** File di `.claude/commands/deploy.md` dan skill di `.claude/skills/deploy/SKILL.md` keduanya membuat `/deploy` dan bekerja dengan cara yang sama. File `.claude/commands/` yang ada tetap berfungsi. Skills menambahkan fitur opsional: direktori untuk file pendukung, frontmatter untuk [mengontrol apakah Anda atau Claude menginvokasinya](#control-who-invokes-a-skill), dan kemampuan bagi Claude untuk memuatnya secara otomatis ketika relevan.
</Note>

Claude Code skills mengikuti standar terbuka [Agent Skills](https://agentskills.io), yang bekerja di berbagai alat AI. Claude Code memperluas standar dengan fitur tambahan seperti [kontrol invokasi](#control-who-invokes-a-skill), [eksekusi subagent](#run-skills-in-a-subagent), dan [injeksi konteks dinamis](#inject-dynamic-context). Lihat [Menggunakan frontmatter skill di luar Claude Code](#using-skill-frontmatter-outside-claude-code) untuk mengetahui bidang frontmatter mana yang merupakan bagian dari standar dan mana yang merupakan ekstensi Claude Code.

<h2 id="bundled-skills">
  Bundled skills
</h2>

Claude Code mencakup serangkaian bundled skills, seperti `/doctor`, `/code-review`, `/batch`, `/debug`, `/loop`, dan `/claude-api`. Bundled skills berbasis prompt: mereka memberikan Claude instruksi terperinci dan membiarkannya mengorkestrasi pekerjaan menggunakan toolsnya. Sebagian besar perintah bawaan malah mengeksekusi logika tetap secara langsung.

Anda menjalankan bundled skill dengan cara yang sama seperti skill lainnya, dengan mengetik `/` diikuti nama skill. Claude menjalankan beberapa bundled skills secara otomatis ketika relevan; yang lain, termasuk `/verify`, hanya berjalan ketika Anda menjalankannya, yang membuat Anda tetap mengendalikan kapan pemeriksaan yang berjalan lebih lama ini menghabiskan waktu dan token.

Sebagian besar bundled skills tersedia di setiap sesi. Beberapa bergantung pada fitur tertentu: `/workflow-authoring`, misalnya, hanya tersedia ketika [dynamic workflows](/docs/id/workflows) diaktifkan.

Untuk mematikan bundled skills, gunakan pengaturan [`disableBundledSkills`](/docs/id/settings-reference#disablebundledskills).

<Note>
  Pemeriksaan setup [`/doctor`](/docs/id/commands#all-commands) tetap dapat diketik ketika `disableBundledSkills` aktif, di Claude Code v2.1.205 dan yang lebih baru. Untuk menyembunyikannya, atur variabel lingkungan `DISABLE_DOCTOR_COMMAND` atau entri [`skillOverrides`](#override-skill-visibility-from-settings) dari `"doctor": "off"`. Sebelum v2.1.205, `/doctor` adalah perintah bawaan daripada bundled skill.
</Note>

Bundled skills terdaftar bersama perintah bawaan dalam [referensi perintah](/docs/id/commands), ditandai **Skill** di kolom Purpose.

<h3 id="run-and-verify-your-app">
  Jalankan dan verifikasi aplikasi Anda
</h3>

Tiga bundled skills bekerja bersama untuk meluncurkan aplikasi Anda dan mengonfirmasi perubahan terhadap aplikasi yang berjalan daripada hanya tes:

| Skill                  | Purpose                                                                                                                                        |
| :--------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------- |
| `/run`                 | Luncurkan dan jalankan aplikasi Anda untuk melihat perubahan bekerja                                                                           |
| `/verify`              | Bangun dan jalankan aplikasi Anda untuk mengonfirmasi perubahan kode melakukan apa yang seharusnya, tanpa kembali ke tes atau pemeriksaan tipe |
| `/run-skill-generator` | Ajarkan `/run` dan `/verify` cara membangun dan meluncurkan proyek Anda                                                                        |

`/run` dan `/verify` bekerja tanpa setup. Mereka menyimpulkan peluncuran dari jenis proyek Anda (CLI, server, TUI, browser-driven) dan dari apa yang ada di README, `package.json`, atau `Makefile` Anda. Inferensi itu menjadi tidak dapat diandalkan untuk proyek yang membutuhkan apa pun di luar peluncuran standar: database, file env, sesi grafis, build multi-langkah.

`/run-skill-generator` merekam resep sebagai gantinya. Ini membuat aplikasi Anda berjalan dari lingkungan yang bersih, menangkap apa yang berhasil (perintah install, variabel env, skrip peluncuran), dan melakukannya sebagai skill per-proyek di `.claude/skills/run-<name>/`. Setelah itu, `/run`, `/verify`, dan agen lainnya di repo mengikuti resep yang direkam daripada menemukannya kembali. Jalankan `/run-skill-generator` sekali per proyek, dan lagi jika proses build atau peluncuran berubah.

`/verify` juga dapat merekam resepnya sendiri. Ketika harus membangun dan menjalankan aplikasi Anda tanpa resep yang direkam, itu menulis apa yang berhasil ke `.claude/skills/verify/SKILL.md` di akar repo, atau di direktori paket yang disentuh dalam monorepo, sehingga run dan agen lain kemudian mengikuti langkah yang sama. Di akar repo, skill yang direkam menggantikan `/verify` bundled. Ini memerlukan Claude Code v2.1.200 atau yang lebih baru.

Claude mengedit file yang direkam hanya ketika itu mengarahkan run dengan salah, seperti perintah yang gagal atau langkah yang hilang, sehingga Anda dapat melakukan commit file tanpa per-session diffs. Sebelum v2.1.205, bundled skill memberi tahu Claude untuk melipat apa pun yang dipelajari run, yang menyebabkan konflik merge yang sering.

<h2 id="getting-started">
  Memulai
</h2>

<h3 id="create-your-first-skill">
  Buat skill pertama Anda
</h3>

Contoh ini membuat skill yang merangkum perubahan yang belum di-commit dalam repositori git Anda dan menandai apa pun yang berisiko. Ini menarik diff langsung ke dalam prompt sebelum Claude membacanya, sehingga respons didasarkan pada pohon kerja aktual Anda daripada apa yang dapat Claude tebak dari file terbuka. Claude memuat skill secara otomatis ketika Anda bertanya tentang perubahan Anda, atau Anda dapat memanggilnya langsung dengan `/summarize-changes`.

<Steps>
  <Step title="Buat direktori skill">
    Buat direktori untuk skill di folder skills pribadi Anda. Skills pribadi tersedia di semua proyek Anda.

    ```bash theme={null}
    mkdir -p ~/.claude/skills/summarize-changes
    ```
  </Step>

  <Step title="Tulis SKILL.md">
    Setiap skill memerlukan file `SKILL.md` dengan dua bagian: frontmatter YAML antara penanda `---` yang memberi tahu Claude kapan menggunakan skill, dan konten markdown dengan instruksi yang diikuti Claude ketika skill berjalan. Nama direktori menjadi perintah yang Anda ketik, dan `description` membantu Claude memutuskan kapan memuat skill secara otomatis.

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

    Baris `` !`git diff HEAD` `` menggunakan [dynamic context injection](#inject-dynamic-context): Claude Code menjalankan perintah dan mengganti baris dengan outputnya sebelum Claude melihat konten skill, sehingga instruksi tiba dengan diff saat ini sudah inline.
  </Step>

  <Step title="Uji skill">
    Buka proyek git, buat edit kecil ke file apa pun, dan mulai Claude Code dengan menjalankan `claude`. Anda dapat menguji skill dengan dua cara.

    **Biarkan Claude memanggilnya secara otomatis** dengan menanyakan sesuatu yang cocok dengan deskripsi:

    ```text theme={null}
    What did I change?
    ```

    **Atau panggilnya langsung** dengan nama skill:

    ```text theme={null}
    /summarize-changes
    ```

    Bagaimanapun, Claude harus merespons dengan ringkasan singkat edit Anda dan daftar risiko.
  </Step>
</Steps>

<h2 id="where-skills-live">
  Pilih tempat skills dimuat
</h2>

Tempat Anda menyimpan skill menentukan sesi mana yang memuatnya. Simpan di bawah direktori home Anda untuk mendapatkannya di setiap proyek, commit ke repositori untuk membagikannya dengan semua orang yang bekerja di sana, atau distribusikan melalui plugin atau managed settings untuk menjangkau seluruh tim.

| Lokasi               | Path                                                                                                             | Dimuat di                                                                                                                                                                                                           |
| :------------------- | :--------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Enterprise           | `.claude/skills/<skill-name>/SKILL.md` di [direktori managed settings](/docs/id/managed-settings#delivery-mechanisms) | Semua pengguna di mesin tempat organisasi Anda menerapkannya                                                                                                                                                        |
| Personal             | `~/.claude/skills/<skill-name>/SKILL.md`                                                                         | Semua proyek Anda di mesin ini, tetapi bukan [sesi Cowork atau cloud](#skills-in-cowork-and-cloud-sessions)                                                                                                         |
| Project              | `.claude/skills/<skill-name>/SKILL.md`                                                                           | Sesi di repositori ini. Commit agar tim Anda juga mendapatkannya                                                                                                                                                    |
| Nested               | `<subdir>/.claude/skills/<skill-name>/SKILL.md`                                                                  | Sesi yang dimulai di atau di bawah `<subdir>`. Sesi yang dimulai di atasnya memuat skill sekali Claude bekerja pada file di sana. Lihat [monorepos dan subdirektori](#discovery-from-parent-and-nested-directories) |
| Additional directory | `.claude/skills/<skill-name>/SKILL.md` di direktori yang Anda lewatkan dengan `--add-dir`                        | Sesi itu. Lihat [direktori di luar proyek](#skills-from-additional-directories)                                                                                                                                     |
| Plugin               | `<plugin>/skills/<skill-name>/SKILL.md`                                                                          | Di mana pun [plugin](/docs/id/plugins/overview) diaktifkan, sebagai `/plugin-name:skill-name`                                                                                                                            |
| claude.ai account    | Skills yang diaktifkan untuk akun claude.ai Anda                                                                 | Sesi Cowork, sesi cloud, dan sesi terminal tempat Anda masuk dengan akun itu. Lihat [Skills yang disinkronkan dari claude.ai](#how-synced-skills-behave)                                                            |

Folder skill juga mengikuti aturan ini:

* **Folder yang di-symlink**: entri `<skill-name>` di lokasi enterprise, personal, atau project dapat berupa symlink ke direktori lain di disk. Claude Code membaca `SKILL.md` dari target dan memuat skill sekali meskipun beberapa lokasi menunjuk ke target yang sama. Plugin skills [menangani symlinks secara berbeda](/docs/id/plugins/host-marketplace#share-files-within-a-marketplace-with-symlinks).
* **Nama yang dicadangkan**: jangan beri nama folder skill `synced`, dalam kapitalisasi apa pun. Claude Code menggunakan `~/.claude/skills/synced/` untuk [skills yang diunduh dari claude.ai](#where-synced-skills-load) dan melewati skill yang Anda buat dengan nama itu di lokasi enterprise, personal, dan project.
* **File perintah**: file Markdown di `.claude/commands/` adalah format yang lebih lama dan masih berfungsi. Ini mendukung [frontmatter](#frontmatter-reference) yang sama kecuali `name` dan `paths`. Untuk menemukan nama yang Anda ketik untuk menginvokasinya, lihat [Bagaimana skill mendapatkan nama perintahnya](#how-a-skill-gets-its-command-name). Lebih suka skill untuk pekerjaan baru, karena skills juga mendukung [file pendukung](#add-supporting-files).
* **Folder skill sebagai plugin**: tambahkan `.claude-plugin/plugin.json` ke folder skill dan itu dimuat sebagai [plugin](/docs/id/plugins/loading#plugins-shared-through-a-repository) bernama `<name>@skills-dir`, sehingga dapat menggabungkan agents, hooks, dan MCP servers. Di `.claude/skills/` proyek, ini memerlukan penerimaan dialog kepercayaan workspace terlebih dahulu.

<h3 id="discovery-from-parent-and-nested-directories">
  Muat skills di monorepos dan subdirektori
</h3>

Claude Code memuat project skills dari `.claude/skills/` di direktori tempat Anda memulainya dan di setiap direktori parent hingga akar repositori, jadi memulai di `packages/frontend/` masih mengambil skills yang ditentukan di root. Ketika Anda [memindahkan sesi dengan `/cd`](/docs/id/permissions#move-the-session-to-another-directory) pada v2.1.246 atau lebih baru, Claude Code menambahkan project skills direktori baru.

Dalam sesi yang berjalan di [git worktree](/docs/id/worktrees) yang tertaut, Claude Code hanya mencari direktori parent hingga akar worktree. Pada Claude Code v2.1.277 atau lebih baru, ketika checkout worktree tidak memiliki direktori `.claude/skills` di rootnya, Claude Code memuat project skills checkout utama sebagai gantinya. Lihat [Apa yang dibagikan worktrees dengan checkout utama](/docs/id/worktrees#what-worktrees-share-with-the-main-checkout).

Skills di direktori `.claude/skills/` di bawah tempat Anda memulai tidak dimuat saat startup. Mereka dimuat pertama kali Claude membaca atau mengedit file di subdirektori itu dan tetap tersedia untuk sisa sesi. Sampai saat itu mereka tidak muncul di menu `/` dan Anda tidak dapat menginvokasinya berdasarkan nama. Untuk memuatnya lebih cepat, jalankan `/add-dir` dengan path subdirektori, yang memerlukan Claude Code v2.1.257 atau lebih baru.

Ketika nested skill berbagi nama dengan skill lain, keduanya tetap tersedia. Dengan skill `deploy` di akar repositori dan skill lain di `apps/web/.claude/skills/`:

* `/deploy` menjalankan skill root. Claude Code juga mencantumkan varian yang memenuhi syarat direktori untuk Claude, dengan instruksi untuk menginvokasi yang direktorinya menyimpan file yang sedang dikerjakan, sehingga nested skill masih berlaku untuk pekerjaan di `apps/web/`.
* `/apps/web:deploy` menjalankan nested skill sendiri. Deskripsinya menamai direktori yang berlaku.

<h3 id="skills-from-additional-directories">
  Muat skills dari direktori di luar proyek
</h3>

Ketika Anda menambahkan direktori dengan `--add-dir` atau `/add-dir`, Claude Code memuat skills di `.claude/skills/` direktori itu, bersama dengan `.claude/commands/` dan `.claude/agents/` nya. Direktori yang Agent SDK tambahkan melalui [`additionalDirectories`](/docs/id/agent-sdk/typescript#options) di TypeScript atau [`add_dirs`](/docs/id/agent-sdk/python#claudeagentoptions) di Python memuat dengan cara yang sama, karena SDK meneruskannya sebagai `--add-dir`. Pengaturan `permissions.additionalDirectories` di `settings.json` memberikan akses file saja dan tidak memuat salah satu dari ini.

Claude Code mengawasi `.claude/skills/` di direktori yang Anda lewatkan dengan `--add-dir` saat peluncuran, seperti yang dijelaskan [Edit a skill during a session](#live-change-detection). Itu tidak mengawasi `.claude/commands/` atau `.claude/agents/` direktori yang ditambahkan, jadi restart sesi setelah mengubah file di sana.

Beban ini bergantung pada [setting source](/docs/id/agent-sdk/claude-code-features#control-filesystem-settings-with-settingsources) `project`, yang aktif secara default. Kebijakan [`strictPluginOnlyCustomization`](/docs/id/settings-reference#strictpluginonlycustomization), [bare mode](/docs/id/headless#start-faster-with-bare-mode), dan [`--safe-mode`](/docs/id/cli-reference#cli-flags) masing-masing membatasinya lebih lanjut, seperti yang dijelaskan halaman-halaman itu. Lihat [Additional directories grant file access, not configuration](/docs/id/permissions#additional-directories-grant-file-access-not-configuration) untuk tabel lengkap apa yang dimuat direktori yang ditambahkan, termasuk `CLAUDE.md` dan pengaturan plugin.

<h3 id="resolve-skills-that-share-a-name">
  Selesaikan skills yang berbagi nama
</h3>

Ketika dua skills berbagi nama, tempat asal masing-masing menentukan yang mana `/name` jalankan. Tabel mencakup lokasi enterprise, personal, project, nested, plugin, dan claude.ai, skills bundel, dan file perintah:

| Nama yang sama di                                                                                    | Yang mana yang berjalan                                                                                                                                                                                                |
| :--------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Dua dari enterprise, personal, dan project                                                           | Enterprise di atas personal, dan personal di atas project. Dengan `deploy` di `~/.claude/skills/` dan `.claude/skills/` proyek, `/deploy` menjalankan yang personal                                                    |
| Salah satu dari lokasi itu dan [bundled skill](#bundled-skills)                                      | Skill Anda menggantikan perintah bundel, tetapi bukan aliasnya. Skill `code-review` proyek menggantikan `/code-review`, dan alias bundel `/review` tidak pernah menjalankan skill Anda                                 |
| Skill dan file di `.claude/commands/`                                                                | Skill itu                                                                                                                                                                                                              |
| Skill root proyek dan nested skill                                                                   | Keduanya dimuat. Lihat [monorepos dan subdirektori](#discovery-from-parent-and-nested-directories)                                                                                                                     |
| Plugin skill dan skill di salah satu lokasi di atas                                                  | Keduanya dimuat, karena plugin skills diberi namespace sebagai `/plugin-name:skill-name`                                                                                                                               |
| Salah satu dari di atas dan skill [disinkronkan dari akun claude.ai Anda](#how-synced-skills-behave) | Skill atau perintah lainnya. Skill yang disinkronkan masih berjalan sebagai `/anthropic-skills:<name>`. Lihat [Ketika nama synced skill cocok dengan perintah lain](#when-a-synced-skill-name-matches-another-command) |

<h3 id="skills-in-cowork-and-cloud-sessions">
  Gunakan skills di sesi Cowork dan cloud
</h3>

Sesi [Cowork](https://claude.com/product/cowork) dan [sesi cloud](/docs/id/cloud-environments#what-carries-over-from-your-setup), termasuk [routines](/docs/id/routines), tidak membaca `~/.claude/skills/` di mesin Anda. Baik sesi Cowork interaktif maupun terjadwal memuat skills yang diaktifkan untuk akun claude.ai Anda, disinkronkan saat startup sesi; kelola dari **Customize** di sidebar Desktop app atau dari pengaturan skills di claude.ai. Sesi cloud juga memuat project skills yang di-commit ke `.claude/skills/` repositori yang dikloning.

Jika skill hanya ada di `~/.claude/skills/` di mesin Anda, Claude Code melaporkan bahwa skill tidak ditemukan ketika [routine](/docs/id/routines) menginvokasinya, karena setiap jalankan routine dimulai sebagai sesi cloud segar. Untuk membuat personal skill tersedia di sesi ini:

* Untuk sesi Cowork dan cloud, aktifkan skill untuk akun claude.ai Anda.
* Untuk sesi cloud, Anda dapat sebagai gantinya commit skill ke `.claude/skills/` repositori. Plugin yang dideklarasikan di `.claude/settings.json` repositori dan plugin yang hanya diaktifkan di pengaturan pengguna Anda [tidak dimuat di sesi cloud](/docs/id/cloud-environments#what-carries-over-from-your-setup).

[Desktop scheduled tasks](/docs/id/desktop-scheduled-tasks) berjalan secara lokal di mesin Anda, jadi mereka memuat `~/.claude/skills/`.

<h3 id="how-synced-skills-behave">
  Skills yang disinkronkan dari claude.ai
</h3>

Bagian ini berlaku untuk Anda jika Anda menggunakan sesi Cowork atau cloud, atau masuk ke Claude Code di terminal Anda dengan akun claude.ai. Di sesi itu, Claude Code memuat skills yang diaktifkan untuk akun claude.ai Anda, tanpa setup di pihak Anda, seperti yang dijelaskan [Where synced skills load](#where-synced-skills-load). Skills itu termasuk yang Anda buat atau aktifkan di pengaturan claude.ai Anda, skills yang disediakan organisasi Anda di sana, dan skills bawaan Anthropic seperti `pdf` dan `xlsx`.

Claude Code mengunduh synced skill dari akun Anda daripada membaca file yang Anda tulis di mesin tempat sesi berjalan, jadi itu menerapkan aturan ke synced skills yang tidak berlaku untuk skills yang Anda simpan di [lokasi skills](#where-skills-live).

<h4 id="where-synced-skills-load">
  Tempat synced skills dimuat
</h4>

Dalam sesi Cowork atau cloud, Claude Code memuat skills yang diaktifkan untuk akun claude.ai Anda, dan [Skills in Cowork and cloud sessions](#skills-in-cowork-and-cloud-sessions) mengatakan bagaimana memilih skills mana yang sesi itu dapatkan.

Di terminal Anda, Claude Code menyinkronkan skills itu di sesi tempat Anda masuk dengan akun claude.ai Anda. Ketika sesi dimulai, Claude Code mengunduh skills akun Anda ke `~/.claude/skills/synced/` di latar belakang, kemudian memeriksa claude.ai untuk perubahan sekitar setiap 10 menit saat sesi berjalan. Ketika pemeriksaan menemukan bahwa skill ditambahkan, diedit, atau dimatikan di claude.ai, Claude Code menambah, memperbarui, atau menghapusnya di sesi yang berjalan tanpa restart. Sinkronisasi di sesi terminal memerlukan Claude Code v2.1.273 atau lebih baru.

Sinkronisasi tidak pernah menunda startup, karena Claude menunggu unduhan skill hanya ketika menginvokasinya. Jalankan [non-interaktif](/docs/id/headless) yang singkat dapat selesai sebelum skill yang baru ditambahkan diunduh, dalam hal ini sesi yang lebih baru mengunduhnya. Untuk membuat jalankan non-interaktif mengunduh skills Anda dan menunggu daftar sebelum menjawab prompt, atur [`CLAUDE_CODE_SYNC_SKILLS`](/docs/id/env-vars#variables) ke `1`.

Claude Code hanya menyinkronkan di sesi yang masuk dengan akun claude.ai Anda dan [mengambil feature flags dari Anthropic](/docs/id/env-vars#features-that-need-feature-flag-fetching). Itu tidak menyinkronkan di sesi ini:

* Sesi yang tidak menggunakan sign-in yang disimpan oleh `/login`, seperti yang mengautentikasi dengan API key, atau yang mana `ANTHROPIC_AUTH_TOKEN`, `CLAUDE_CODE_OAUTH_TOKEN`, atau skrip `apiKeyHelper` menyediakan kredensial
* Sesi yang tidak mengambil feature flags, seperti yang di Amazon Bedrock atau yang mana Anda atur `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`
* Sesi di [bare mode](/docs/id/headless#start-faster-with-bare-mode) atau yang Anda mulai dengan `--safe-mode`
* Sesi tempat managed settings organisasi Anda [mengunci skills ke sumber plugin](/docs/id/settings-reference#strictpluginonlycustomization-skills), atau yang Anda mulai dengan daftar [`--setting-sources`](/docs/id/cli-reference#cli-flags) yang meninggalkan `user`

Jika Anda masuk dengan `/login` selama sesi, restart Claude Code untuk mulai menyinkronkan.

Skills yang sesi sebelumnya sinkronkan tetap di disk. Claude Code memuatnya di sesi yang lebih baru yang masuk ke akun yang sama, bahkan ketika tidak dapat menjangkau claude.ai.

Claude Code mengunduh synced skills dan tidak pernah mengunggahnya. Jika Anda atau Claude mengedit file di bawah `~/.claude/skills/synced/`, perubahan tidak disimpan ke akun claude.ai Anda, dan sinkronisasi yang lebih baru dapat menimpanya atau menghapusnya. Untuk mengubah synced skill, perbarui di claude.ai; sinkronisasi berikutnya mengunduh versi baru.

Untuk melihat skills mana yang disinkronkan, jalankan `/skills`. Menu mencantumnya di bawah `claude.ai sync`.

Beberapa skills Anthropic, seperti `pdf` dan `xlsx`, selalu disinkronkan. Untuk sisanya, aktifkan atau matikan skill di pengaturan skills Anda di claude.ai untuk mengubah apakah itu disinkronkan.

Untuk berhenti menyinkronkan di mesin, atur [`syncClaudeAiSkills`](/docs/id/settings-reference#syncclaudeaiskills) ke `false` di pengaturan pengguna Anda. Claude Code berhenti mengunduh, dan saat berikutnya dimulai itu memindahkan skills yang sudah disinkronkan ke `~/.claude/skills/.trash/` dan tidak lagi memuatnya. Organisasi Anda dapat mematikan sinkronisasi untuk semua orang dengan mematikan Skills di claude.ai. Untuk berhenti menyinkronkan sambil membiarkan Skills aktif, itu dapat mengatur kunci yang sama di [managed settings](/docs/id/managed-settings).

Jika organisasi Anda mematikan Skills di claude.ai, Claude Code menghapus skills yang diunduh dan mereka berhenti dimuat. Skills yang dihapus pindah ke `~/.claude/skills/.trash/`, tempat Anda dapat memulihkan file sampai [retention sweep](/docs/id/claude-directory#cleaned-up-automatically) menghapusnya. Setelah organisasi Anda menghidupkan Skills kembali, Claude Code mengunduh skills yang Anda aktifkan di sinkronisasi berikutnya.

<h4 id="when-a-synced-skill-name-matches-another-command">
  Ketika nama synced skill cocok dengan perintah lain
</h4>

Anda dapat menginvokasi synced skill dengan nama lengkapnya, `/anthropic-skills:<name>`, atau dengan nama pendeknya, `/<name>`. Ketika perintah lain menggunakan nama pendek itu, `/<name>` menjalankan perintah lain, dan synced skill berjalan hanya sebagai `/anthropic-skills:<name>`. Dengan skill `deploy` lokal dan synced `deploy`, `/deploy` menjalankan skill lokal dan `/anthropic-skills:deploy` menjalankan yang disinkronkan. Sebelum v2.1.269, synced skill hanya memiliki nama pendeknya.

Perintah lain dapat berupa salah satu dari ini:

* Perintah bawaan atau [bundled skill](#bundled-skills), termasuk yang tidak tersedia di sesi Anda, misalnya setelah Anda mematikan bundled skills
* Skill di [level lokal apa pun](#where-skills-live) atau file di `.claude/commands/`
* Plugin skill
* [MCP prompt](/docs/id/mcp#use-mcp-prompts-as-commands)

Claude Code memberi label synced skills sehingga Anda dapat mengetahui dari mana mereka berasal. Menu `/skills` dan `/context` mengelompokkan synced skills di bawah `claude.ai sync`, dan menu perintah `/` menandainya sebagai berasal dari claude.ai.

Ketika membandingkan nama, Claude Code mengabaikan huruf besar-kecil, spasi, dan karakter tak terlihat, dan memperlakukan bentuk kompatibilitas seperti huruf lebar dan varian dash sebagai padanan polosnya. Misalnya, synced skill bernama `Commit` dan skill lokal bernama `commit` dihitung sebagai nama yang sama, jadi `/commit` terus menjalankan skill lokal Anda.

Nama yang berbeda hanya dengan huruf yang mirip dari alfabet lain dihitung sebagai nama yang berbeda, dan label `claude.ai sync` adalah cara Anda membedakan keduanya. Pemeriksaan dan label ini memerlukan Claude Code v2.1.228 atau lebih baru.

<h4 id="how-claude-code-handles-the-frontmatter-of-a-synced-skill">
  Bagaimana Claude Code menangani frontmatter dari synced skill
</h4>

Claude Code menerapkan dua aturan ke frontmatter synced skill:

* Claude Code menghormati frontmatter di setiap jenis sesi, jadi hibah `allowed-tools` melalui [permission flow](/docs/id/permissions) normal.
* Claude Code membersihkan teks tampilan yang disediakan skill, seperti deskripsinya. Itu menghapus karakter kontrol, dan dalam teks yang mencapai Claude, seperti deskripsi, itu juga menghindari tanda kurung sudut sehingga teks tidak dapat meniru pemformatan internal Claude Code. Pembersihan ini memerlukan Claude Code v2.1.228 atau lebih baru.

<h4 id="how-claude-code-handles-the-body-of-a-synced-skill">
  Bagaimana Claude Code menangani body dari synced skill
</h4>

Apa yang Claude Code lakukan dengan body synced skill bergantung pada tempat sesi berjalan:

* Dalam sesi cloud, body mempertahankan perilaku yang dimiliki skill lokal, karena sesi berjalan dalam kontainer terisolasi.
* Dalam sesi Cowork di desktop Anda, body mempertahankan perilaku yang dimiliki skill lokal, kecuali Claude Code menggantikan setiap baris perintah `!` dengan placeholder [`disableSkillShellExecution`](#inject-dynamic-context), seperti yang dilakukannya untuk setiap skill yang Anda sediakan di sana.
* Dalam sesi lain di mesin Anda, Claude Code tidak menjalankan perintah [`!`](#inject-dynamic-context), tidak melampirkan file yang referensi `@` beri nama dengan cara yang dilakukannya untuk skill lokal, dan tidak mengganti placeholder `${CLAUDE_PROJECT_DIR}` dan `${CLAUDE_SESSION_ID}`, jadi referensi `@` dan kedua placeholder mencapai Claude sebagai teks literal. Baris perintah `!` mencapai Claude sebagai teks literal juga, atau sebagai placeholder itu ketika `disableSkillShellExecution` aktif. Penanganan ini memerlukan Claude Code v2.1.228 atau lebih baru.

<h3 id="live-change-detection">
  Edit skill selama sesi
</h3>

Claude Code mengawasi direktori skill untuk perubahan file, kecuali di [bare mode](/docs/id/headless#start-faster-with-bare-mode). Ketika Anda menambah, mengedit, atau menghapus skill di bawah `~/.claude/skills/`, project `.claude/skills/`, atau `.claude/skills/` di dalam direktori `--add-dir`, Claude Code mengambil perubahan dalam sesi saat ini, tanpa restart. Jika Anda membuat direktori skills tingkat atas yang tidak ada ketika sesi dimulai, restart Claude Code sehingga dapat mengawasi direktori baru.

Live change detection mencakup teks `SKILL.md` saja. Untuk folder skill yang juga merupakan [plugin](/docs/id/plugins/loading#plugins-shared-through-a-repository), perubahan ke `hooks/`, `.mcp.json`, `agents/`, dan `output-styles/` memerlukan `/reload-plugins` untuk berlaku.

<h3 id="remove-a-skill">
  Hapus skill
</h3>

Bagaimana Anda menghapus skill bergantung pada dari mana asalnya:

* **Personal atau project skill**: hapus direktori skill, `~/.claude/skills/<skill-name>/` atau `.claude/skills/<skill-name>/`. Claude Code [menjatuhkannya dari `/skills` di sesi saat ini](#live-change-detection); konten yang sudah dimuat Claude Code darinya mengikuti [skill content lifecycle](#skill-content-lifecycle).
* **Enterprise skill**: administrator menghapus direktori skill dari `.claude/skills/` di dalam [direktori managed settings](/docs/id/managed-settings#delivery-mechanisms), misalnya `/etc/claude-code/.claude/skills/<skill-name>/` di Linux.
* **Plugin skill**: nonaktifkan atau uninstall plugin yang menyediakannya, dari menu `/plugin` atau dengan `/plugin uninstall <plugin-name>@<marketplace-name>`. Claude Code membongkar skills plugin ketika [perubahan berlaku](/docs/id/plugins/cli-reference#reload-plugins) atau ketika Anda restart.
* **Skill yang disinkronkan dari claude.ai**: matikan skill untuk akun claude.ai Anda, di tempat yang sama Anda [mengaktifkannya](#skills-in-cowork-and-cloud-sessions). Claude Code menghapusnya dari `~/.claude/skills/synced/` saat berikutnya [menyinkronkan skills Anda](#where-synced-skills-load). Jika Anda menghapus direktori dengan tangan sebagai gantinya, sinkronisasi berikutnya mengunduhnya lagi sementara skill tetap diaktifkan di claude.ai.
* **Bundled skill**: atur [`disableBundledSkills`](#bundled-skills) ke `true` untuk mematikan bundled skills, atau atur satu skill ke `"off"` di [`skillOverrides`](#override-skill-visibility-from-settings) untuk menyembunyikannya.

Untuk menyimpan personal atau project skill tetapi menghentikan Claude dari menginvokasinya sendiri, atur [`disable-model-invocation: true`](#control-who-invokes-a-skill) di frontmatter-nya, atau `"user-invocable-only"` di [`skillOverrides`](#override-skill-visibility-from-settings) ketika Anda tidak ingin mengedit file.

<h2 id="configure-skills">
  Konfigurasi skills
</h2>

Skills dikonfigurasi melalui frontmatter YAML di bagian atas `SKILL.md` dan konten markdown yang mengikutinya.

<h3 id="types-of-skill-content">
  Jenis konten skill
</h3>

File skill dapat berisi instruksi apa pun, tetapi memikirkan tentang cara Anda ingin menginvokasinya membantu memandu apa yang harus disertakan:

**Konten referensi** menambahkan pengetahuan yang Claude terapkan pada pekerjaan Anda saat ini. Konvensi, pola, panduan gaya, pengetahuan domain. Konten ini berjalan inline sehingga Claude dapat menggunakannya bersama konteks percakapan Anda.

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

**Konten tugas** memberikan Claude instruksi langkah demi langkah untuk tindakan tertentu, seperti deployment, commit, atau pembuatan kode. Ini sering kali merupakan tindakan yang ingin Anda panggil langsung dengan `/skill-name` daripada membiarkan Claude memutuskan kapan menjalankannya. Tambahkan `disable-model-invocation: true` untuk mencegah Claude memicunya secara otomatis. Contoh di bawah menambahkan `context: fork`, yang menjalankan skill dalam konteks subagent-nya sendiri; lihat [Jalankan skills dalam subagent](#run-skills-in-a-subagent).

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

Jaga isi tubuh tetap ringkas. Setelah skill dimuat, kontennya [tetap dalam konteks di seluruh giliran](#skill-content-lifecycle), jadi setiap baris adalah biaya token berulang. Nyatakan apa yang harus dilakukan daripada menceritakan bagaimana atau mengapa, dan terapkan tes keringkasan yang sama yang akan Anda lakukan untuk [konten CLAUDE.md](/docs/id/best-practices#write-an-effective-claude-md).

<h3 id="frontmatter-reference">
  Referensi frontmatter
</h3>

Konfigurasi skill dengan YAML [frontmatter](/docs/id/glossary#frontmatter) antara penanda `---` di bagian atas `SKILL.md`, dan tulis instruksi skill sebagai Markdown setelah penutup `---`. Nama bidang menggunakan kata-kata huruf kecil yang dipisahkan dengan tanda hubung, kecuali `when_to_use`. File [command](#where-skills-live) di `.claude/commands/` menerima bidang yang sama kecuali `name` dan `paths`. Contoh ini menetapkan empat bidang:

```yaml theme={null}
---
name: my-skill
description: What this skill does
disable-model-invocation: true
allowed-tools: Read Grep
---

Your skill instructions here...
```

Semua bidang bersifat opsional. Hanya `description` yang direkomendasikan sehingga Claude tahu kapan harus menggunakan skill. Nama bidang harus cocok dengan tabel dengan tepat, tanda hubung disertakan: Claude Code mengabaikan bidang yang tidak dikenalinya tanpa melaporkan kesalahan.

Claude Code membaca frontmatter hanya ketika pembukaan `---` adalah baris pertama file. Jika tidak, itu memperlakukan seluruh file, penanda `---` disertakan, sebagai konten skill. Jika YAML antara penanda tidak diuraikan, skill masih dimuat tanpa bidang yang ditetapkan; lihat [Skill tidak memicu](#skill-not-triggering) untuk menemukan dan memperbaiki kesalahan.

Bidang Boolean menerima `yes`, `no`, `on`, `off`, `1`, dan `0` dalam huruf apa pun, selain `true` dan `false`. Sebelum v2.1.218, Claude Code hanya mengenali `true` dan `false`.

| Field                      | Required    | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| :------------------------- | :---------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`                     | No          | Display name shown in skill listings. Defaults to the directory name. See [How a skill gets its command name](#how-a-skill-gets-its-command-name) for how the field interacts with the name you type to invoke the skill.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `description`              | Recommended | What the skill does and when to use it. Claude uses this to decide when to apply the skill. If omitted, uses the first non-empty line of the markdown content. Put the key use case first: the combined `description` and `when_to_use` text is truncated at 1,536 characters in the skill listing to reduce context usage.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `when_to_use`              | No          | Additional context for when Claude should invoke the skill, such as trigger phrases or example requests. Appended to `description` in the skill listing and counts toward the 1,536-character cap.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `argument-hint`            | No          | Hint shown during autocomplete to indicate expected arguments. Example: `[issue-number]` or `[filename] [format]`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `arguments`                | No          | Named positional arguments for [`$name` substitution](#available-string-substitutions) in the skill content. Accepts a space-separated string or a YAML list. Names map to argument positions in order.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `disable-model-invocation` | No          | Set to `true` to prevent Claude from automatically loading this skill. Use for workflows you want to trigger manually with `/name`. Also prevents the skill from being [preloaded into subagents](/docs/id/sub-agents#preload-skills-into-subagents). As of v2.1.196, also prevents the skill from running when a [scheduled task](/docs/id/scheduled-tasks) fires with the skill as its prompt. Default: `false`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `user-invocable`           | No          | Set to `false` when only Claude should invoke the skill: Claude Code hides it from the `/` menu and doesn't run it when you type `/name`. Use for background knowledge users shouldn't invoke directly. Default: `true`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `allowed-tools`            | No          | Tools Claude can use without asking permission during the turn that invokes this skill. The grant clears when you send your next message. Accepts a space- or comma-separated string, or a YAML list. See [Pre-approve tools for a skill](#pre-approve-tools-for-a-skill).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `disallowed-tools`         | No          | Tools removed from Claude's available pool while this skill is active. Use for autonomous skills that should never call certain tools, such as `AskUserQuestion` for a background loop. Accepts a space- or comma-separated string, or a YAML list. The restriction clears when you send your next message. Like deny rules, the field can't remove [`EndConversation`](/docs/id/tools-reference#endconversation-tool-behavior) while any other tool remains.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `model`                    | No          | Model to use when this skill is active. The override applies for the rest of the current turn and isn't saved to settings. The session model resumes when you send your next prompt. Accepts the same values as [`/model`](/docs/id/model-config), or `inherit` to keep the active model. A value excluded by your organization's [`availableModels`](/docs/id/model-config#restrict-model-selection) allowlist isn't used, and the session keeps its current model. In [auto mode](/docs/id/permission-modes#eliminate-prompts-with-auto-mode), and in [plan mode while the classifier reviews commands](/docs/id/permission-modes#analyze-before-you-edit-with-plan-mode), a model that auto mode doesn't support also isn't used, and the session keeps its current model. With `context: fork`, the value sets the [forked subagent's model](#run-skills-in-a-subagent) instead, and an excluded value follows the [same rules as a subagent model override](/docs/id/model-config#restrict-model-selection). |
| `effort`                   | No          | [Effort level](/docs/id/model-config#adjust-effort-level) when this skill is active. Overrides the session effort level. Default: inherits from session. Options: `low`, `medium`, `high`, `xhigh`, `max`; available levels depend on the model.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `context`                  | No          | Set to `fork` to run in a forked subagent context. See [Run skills in a subagent](#run-skills-in-a-subagent).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `agent`                    | No          | Which subagent type to use when `context: fork` is set.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `background`               | No          | Only applies with `context: fork`. Set to `false` to wait for the forked subagent's result in the turn that invoked the skill, instead of [running it in the background](#run-skills-in-a-subagent). Default: `true`. Requires Claude Code v2.1.218 or later.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `hooks`                    | No          | Hooks that Claude Code registers when the skill is invoked and keeps running for the rest of the session. See [Hooks in skills and agents](/docs/id/hooks#hooks-in-skills-and-agents) for the configuration format and the `once` option.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `paths`                    | No          | Glob patterns that limit when this skill is activated. Accepts a comma-separated string or a YAML list. When set, Claude loads the skill automatically only when working with files matching the patterns. Uses the same format as [path-specific rules](/docs/id/memory#path-specific-rules).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `shell`                    | No          | Shell to use for `` !`command` `` and ` ```! ` blocks in this skill. Accepts `bash` (default) or `powershell`. Setting `powershell` runs inline shell commands via PowerShell when the [PowerShell tool](/id/tools-reference#powershell-tool) is enabled: it's on by default on Windows without Git Bash, on by default with Git Bash for claude.ai and Console accounts, and needs `CLAUDE_CODE_USE_POWERSHELL_TOOL=1` in Amazon Bedrock, Google Cloud's Agent Platform, and Microsoft Foundry sessions and on macOS, Linux, and WSL. Set it to `0` to turn the tool off.                                                                                                                                                                                                                                                                                                                                                                                                               |
| `metadata`                 | No          | Free-form YAML map for your own key-value data, such as entitlement or catalog fields, read by your own tooling from `SKILL.md`. Claude Code doesn't act on its contents, and drops a value that isn't a map. Don't reuse frontmatter field names such as `paths` as keys.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `license`                  | No          | License covering the skill. Part of the [Agent Skills](https://agentskills.io) spec; see [Using skill frontmatter outside Claude Code](#using-skill-frontmatter-outside-claude-code). Claude Code accepts the field but doesn't act on it.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `compatibility`            | No          | Environment requirements for the skill, such as intended products or system prerequisites, as defined by the [Agent Skills](https://agentskills.io) spec; see [Using skill frontmatter outside Claude Code](#using-skill-frontmatter-outside-claude-code). Accepts a string of up to 500 characters. Claude Code accepts the field but doesn't act on it.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |

<h4 id="using-skill-frontmatter-outside-claude-code">
  Menggunakan skill frontmatter di luar Claude Code
</h4>

Claude Code menerima setiap bidang dalam tabel di atas. Di luar Claude Code, Anda hanya dapat menggunakan bidang dalam spesifikasi [Agent Skills](https://agentskills.io):

| Distribution path                                                                                                                             | Frontmatter fields you can use                                                 |
| :-------------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------- |
| Claude Code skills at [any level](#where-skills-live), including [plugin](/docs/id/plugins/overview) skills                                        | Every field in the table above                                                 |
| claude.ai skill uploads, the Skills API, and packaging with `package_skill.py` from [anthropics/skills](https://github.com/anthropics/skills) | `name`, `description`, `license`, `compatibility`, `metadata`, `allowed-tools` |

Ketika Anda mengaktifkan skill pribadi untuk akun claude.ai Anda, misalnya untuk menggunakannya dalam [sesi Cowork dan cloud](#skills-in-cowork-and-cloud-sessions) dan rutinitas, Anda mengunggahnya ke claude.ai, jadi aturan yang sama berlaku.

Jika Anda menyertakan bidang apa pun yang tidak diizinkan oleh spesifikasi, pengemasan atau unggahan gagal dengan kesalahan keras daripada mengabaikan bidang:

```
Unexpected key(s) in SKILL.md frontmatter: argument-hint. Allowed properties are: allowed-tools, compatibility, description, license, metadata, name
```

Membatasi frontmatter ke enam bidang spesifikasi menghindari kesalahan kunci yang tidak terduga di atas. Spesifikasi [Agent Skills](https://agentskills.io) dan [persyaratan Skills API](https://docs.claude.com/en/api/skills-guide) mendefinisikan segalanya yang divalidasi oleh jalur tersebut. Fitur isi Claude Code-only, seperti [injeksi konteks dinamis](#inject-dynamic-context), tidak berfungsi dalam obrolan claude.ai atau melalui API. Claude Code menerima keenam bidang, jadi frontmatter yang mengikuti spesifikasi dimuat di Claude Code tanpa perubahan.

<h4 id="how-a-skill-gets-its-command-name">
  Bagaimana skill mendapatkan nama perintahnya
</h4>

Perintah yang Anda ketik untuk menginvokasi skill berasal dari tempat file skill berada dan, untuk skill plugin, juga dari bidang frontmatter `name`. Dalam skill pribadi atau proyek, `name` hanya menetapkan label tampilan yang ditampilkan dalam daftar skill, dan perintah masih berasal dari nama direktori. Dalam skill plugin, `name` menetapkan segmen terakhir dari perintah dan awalan plugin tetap ada.

Tabel di bawah menunjukkan dari mana nama perintah berasal untuk setiap tata letak:

| Skill location                                                                                     | Command name source                                                                                           | Example                                                                                                                                |
| :------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------ | :------------------------------------------------------------------------------------------------------------------------------------- |
| Skill directory under `~/.claude/skills/` or `.claude/skills/`                                     | Directory name                                                                                                | `.claude/skills/deploy-staging/SKILL.md` → `/deploy-staging`                                                                           |
| [Nested](#where-skills-live) `.claude/skills/` directory, when the name clashes with another skill | Subdirectory path relative to the working directory, then the skill directory name                            | `apps/web/.claude/skills/deploy/SKILL.md` → `/apps/web:deploy`                                                                         |
| File under `.claude/commands/`                                                                     | File name without extension                                                                                   | `.claude/commands/deploy.md` → `/deploy`                                                                                               |
| File in a subdirectory of `.claude/commands/`                                                      | Subdirectory path relative to `commands/` with each `/` replaced by `:`, then the file name without extension | `.claude/commands/frontend/component.md` → `/frontend:component`                                                                       |
| Plugin `skills/` subdirectory                                                                      | Frontmatter `name` or the directory name, namespaced by plugin                                                | `my-plugin/skills/review/SKILL.md` → `/my-plugin:review`, or `/my-plugin:fancy` with `name: fancy`                                     |
| Plugin root `SKILL.md`                                                                             | Frontmatter `name`, with the plugin directory name as a fallback                                              | `my-plugin/SKILL.md` with `name: review` → `/my-plugin:review`. See [a single skill at the plugin root](/docs/id/plugins/components#skills) |
| Skill [synced from claude.ai](#how-synced-skills-behave)                                           | The skill's name on your claude.ai account, prefixed with `anthropic-skills:`                                 | Account skill `deploy` → `/anthropic-skills:deploy`, or `/deploy` while no other command uses that name                                |

Dalam skill plugin, frontmatter `name` menggantikan nama direktori dalam segmen terakhir perintah, jadi `my-plugin/skills/review/SKILL.md` dengan `name: fancy` menjadi `/my-plugin:fancy`. Perintah bare `/fancy` juga menginvokasi skill kecuali perintah lain sudah menggunakan nama itu. Jika `name` yang Anda tulis sudah dimulai dengan awalan plugin itu sendiri, Claude Code tidak menambahkan awalan lagi pada v2.1.246 atau lebih baru. Misalnya, `name: my-plugin:fancy` masih menjadi `/my-plugin:fancy`. Dari v2.1.216 hingga v2.1.245, Claude Code menggandakan awalan ketika `name` sudah membawanya.

Dalam [sesi non-interaktif](/docs/id/headless), nama `help` dan `feedback` tidak dicadangkan untuk perintah bawaan khusus terminal mereka, jadi skill plugin dengan salah satu nama tersebut menyimpan perintah bare-nya di sana. Setiap terminal-only built-in lainnya, seperti `/login`, tetap dicadangkan meskipun perintah tidak dapat dijalankan dalam sesi tersebut.

Untuk `SKILL.md` akar plugin, tidak ada direktori skill untuk mengambil nama darinya, jadi `name` menyediakan seluruh segmen terakhir. Tanpa bidang `name`, Claude Code kembali ke nama direktori plugin.

<h4 id="available-string-substitutions">
  Substitusi string yang tersedia
</h4>

Skills mendukung substitusi string untuk nilai dinamis dalam konten skill:

| Variable                | Description                                                                                                                                                                                                                                                                                                 |
| :---------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `$ARGUMENTS`            | All arguments passed when invoking the skill. When no placeholder receives an argument, Claude Code appends them as `ARGUMENTS: <value>`. See [Pass arguments to skills](#pass-arguments-to-skills).                                                                                                        |
| `$ARGUMENTS[N]`         | Access a specific argument by 0-based index, such as `$ARGUMENTS[0]` for the first argument.                                                                                                                                                                                                                |
| `$N`                    | Shorthand for `$ARGUMENTS[N]`, such as `$0` for the first argument or `$1` for the second.                                                                                                                                                                                                                  |
| `$name`                 | Named argument declared in the [`arguments`](#frontmatter-reference) frontmatter list. Names map to positions in order, so with `arguments: [issue, branch]` the placeholder `$issue` expands to the first argument and `$branch` to the second.                                                            |
| `${CLAUDE_SESSION_ID}`  | The current session ID. Useful for logging, creating session-specific files, or correlating skill output with sessions.                                                                                                                                                                                     |
| `${CLAUDE_EFFORT}`      | The current effort level: `low`, `medium`, `high`, `xhigh`, or `max`. Ultracode is not a distinct level and reports as `xhigh`. Use this to adapt skill instructions to the active effort setting.                                                                                                          |
| `${CLAUDE_SKILL_DIR}`   | The directory containing the skill's `SKILL.md` file. For plugin skills, this is the skill's subdirectory within the plugin, not the plugin root. Use this in bash injection commands to reference scripts or files bundled with the skill, regardless of the current working directory.                    |
| `${CLAUDE_PROJECT_DIR}` | The project root directory. This is the same path [hooks](/docs/id/hooks#reference-scripts-by-path) and MCP servers receive as `CLAUDE_PROJECT_DIR`. Use this to reference project-local scripts or files, such as `${CLAUDE_PROJECT_DIR}/.claude/hooks/helper.sh`, independent of where the skill is installed. |
| `${CLAUDE_PLUGIN_ROOT}` | The plugin's installation directory. Substituted only in plugin skills. Use this to reference scripts or files bundled anywhere in the plugin, including resources shared between the plugin's skills. See [plugin environment variables](/docs/id/plugins/manifest-reference#environment-variables).            |
| `${CLAUDE_PLUGIN_DATA}` | The plugin's [persistent data directory](/docs/id/plugins/components#path-variables-and-persistent-data), which survives plugin updates. Substituted only in plugin skills. Use this to reference installed dependencies, generated files, or caches that must outlive an update.                                |

Claude Code menggantikan `${CLAUDE_SKILL_DIR}` dan `${CLAUDE_PROJECT_DIR}` di dua tempat: konten markdown skill, dan aturan Bash dalam frontmatter [`allowed-tools`](#frontmatter-reference). Dalam skill plugin, Claude Code menggantikan `${CLAUDE_PLUGIN_ROOT}` dan `${CLAUDE_PLUGIN_DATA}` di tempat yang sama. Menggunakan variabel yang sama di kedua tempat memungkinkan skill menjalankan skrip bundel tanpa prompt izin. Skill berikut menunjukkan polanya:

```yaml theme={null}
---
name: render-chart
description: Render a chart from a CSV file
allowed-tools: Bash(${CLAUDE_SKILL_DIR}/scripts/render.sh *)
---

Run `${CLAUDE_SKILL_DIR}/scripts/render.sh <csv-file>` to render the chart.
```

Jika skill ini diinstal di `~/.claude/skills/render-chart/`, kedua kemunculan `${CLAUDE_SKILL_DIR}` berkembang ke direktori itu. Aturan `allowed-tools` kemudian cocok dengan perintah yang tepat yang diberitahu skill body kepada Claude untuk dijalankan, jadi skrip berjalan tanpa meminta.

Substitusi `${CLAUDE_PROJECT_DIR}` memerlukan Claude Code v2.1.196 atau lebih baru.

Argumen yang diindeks menggunakan kutipan gaya shell, jadi bungkus nilai multi-kata dalam tanda kutip untuk meneruskannya sebagai argumen tunggal. Misalnya, `/my-skill "hello world" second` membuat `$0` berkembang menjadi `hello world` dan `$1` menjadi `second`. Placeholder `$ARGUMENTS` selalu berkembang ke string argumen lengkap seperti yang diketik.

Placeholder yang diindeks tanpa argumen yang sesuai, seperti `$2` ketika hanya satu argumen yang diteruskan, tetap dalam konten tidak berubah. Placeholder bernama dari frontmatter [`arguments`](#frontmatter-reference) tanpa argumen yang cocok berkembang menjadi string kosong.

Jika Anda meneruskan nilai argumen yang sendiri berisi teks seperti `$1` atau `$ARGUMENTS`, Claude Code menyisipkannya sebagai teks literal dan tidak memperluasnya. Misalnya, jika isi skill berisi `Summarize $0` dan Anda menjalankan `/summarize "$ARGUMENTS from yesterday"`, Claude menerima `Summarize $ARGUMENTS from yesterday`. Claude Code masih menggantikan variabel `${CLAUDE_*}` seperti `${CLAUDE_SKILL_DIR}` setelah menyisipkan argumen.

Untuk menyertakan literal `$` sebelum digit, `ARGUMENTS`, atau nama argumen yang dideklarasikan, seperti `$1.00` dalam prosa, lepaskan dengan garis miring terbalik: `\$1.00`. Garis miring terbalik sebelum `$` lainnya dibiarkan tidak berubah. Hanya satu garis miring terbalik langsung sebelum token yang melepaskan. Garis miring terbalik ganda seperti `\\$1` meninggalkan kedua garis miring terbalik di tempat, dan `$1` masih berkembang ke nilai argumen. Pelarian garis miring terbalik hanya mencakup placeholder argumen ini. Garis miring terbalik tidak mencegah substitusi variabel `${CLAUDE_*}` di mana variabel berlaku.

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

Skills dapat mencakup beberapa file dalam direktorinya. Ini membuat `SKILL.md` fokus pada hal-hal penting sambil membiarkan Claude mengakses materi referensi terperinci hanya saat diperlukan. Dokumen referensi besar, spesifikasi API, atau koleksi contoh tidak perlu dimuat ke dalam konteks setiap kali skill berjalan.

```text theme={null}
my-skill/
├── SKILL.md (required - overview and navigation)
├── reference.md (detailed API docs - loaded when needed)
├── examples.md (usage examples - loaded when needed)
└── scripts/
    └── helper.py (utility script - executed, not loaded)
```

Referensikan file pendukung dari `SKILL.md` sehingga Claude tahu apa yang berisi setiap file dan kapan memuatnya:

```markdown theme={null}
## Additional resources

- For complete API details, see [reference.md](reference.md)
- For usage examples, see [examples.md](examples.md)
```

<Tip>Jaga `SKILL.md` di bawah 500 baris. Pindahkan materi referensi terperinci ke file terpisah.</Tip>

<h3 id="control-who-invokes-a-skill">
  Kontrol siapa yang menginvokasi skill
</h3>

Secara default, baik Anda maupun Claude dapat menginvokasi skill apa pun. Anda dapat mengetik `/skill-name` untuk menginvokasinya secara langsung, dan Claude dapat memuatnya secara otomatis ketika relevan dengan percakapan Anda. Dua bidang frontmatter memungkinkan Anda membatasi ini:

* **`disable-model-invocation: true`**: Hanya Anda yang dapat menginvokasi skill. Gunakan ini untuk alur kerja dengan efek samping atau yang ingin Anda kontrol waktunya, seperti `/commit`, `/deploy`, atau `/send-slack-message`. Anda tidak ingin Claude memutuskan untuk deploy karena kode Anda terlihat siap.

* **`user-invocable: false`**: Hanya Claude yang dapat menginvokasi skill. Gunakan ini untuk pengetahuan latar belakang yang tidak dapat ditindaklanjuti sebagai perintah. Skill `legacy-system-context` menjelaskan cara kerja sistem lama. Claude harus tahu ini ketika relevan, tetapi `/legacy-system-context` bukan tindakan yang bermakna bagi pengguna untuk diambil.

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

Jika Claude mencoba bagaimanapun, Claude Code memblokir panggilan dan menginstruksikan untuk tidak mereproduksi langkah deploy dengan cara lain, jadi harapkan Claude menyarankan menjalankan `/deploy` sendiri.

Berikut adalah bagaimana dua bidang mempengaruhi invokasi dan pemuatan konteks:

| Frontmatter                      | You can invoke | Claude can invoke | When loaded into context                                     |
| :------------------------------- | :------------- | :---------------- | :----------------------------------------------------------- |
| (default)                        | Yes            | Yes               | Description always in context, full skill loads when invoked |
| `disable-model-invocation: true` | Yes            | No                | Description not in context, full skill loads when you invoke |
| `user-invocable: false`          | No             | Yes               | Description always in context, full skill loads when invoked |

<Note>
  Dalam sesi reguler, deskripsi skill dimuat ke dalam konteks sehingga Claude tahu apa yang tersedia, tetapi konten skill penuh hanya dimuat saat diinvokasi. [Subagents dengan skill yang dimuat sebelumnya](/docs/id/sub-agents#preload-skills-into-subagents) bekerja berbeda: konten skill penuh disuntikkan saat startup.
</Note>

<h3 id="skill-content-lifecycle">
  Siklus hidup konten skill
</h3>

Ketika Anda atau Claude menginvokasi skill, konten `SKILL.md` yang dirender memasuki percakapan sebagai pesan tunggal dan tetap ada di seluruh giliran kemudian. Persistensi ini berlaku untuk instruksi skill, bukan izinnya: hibah [`allowed-tools`](#pre-approve-tools-for-a-skill) dihapus ketika Anda mengirim pesan berikutnya. Claude Code tidak membaca ulang file skill pada giliran kemudian, jadi tulis panduan yang harus berlaku sepanjang tugas sebagai instruksi berdiri daripada langkah sekali jalan.

Ketika Claude menginvokasi ulang skill yang konten yang dirender identik dengan salinan yang sudah ada dalam konteks, Claude Code menambahkan catatan singkat bahwa skill sudah dimuat daripada salinan kedua konten. Ketika konten yang dirender berbeda, karena argumen berubah atau perintah [konteks dinamis](#inject-dynamic-context) menghasilkan output baru, Claude Code menambahkan konten penuh lagi.

[Auto-compaction](/docs/id/how-claude-code-works#when-context-fills-up) membawa skill yang diinvokasi maju dalam anggaran token. Ketika percakapan diringkas untuk membebaskan konteks, Claude Code melampirkan kembali invokasi paling baru dari setiap skill setelah ringkasan, menyimpan 5.000 token pertama dari masing-masing. Skill yang dilampirkan kembali berbagi anggaran gabungan 25.000 token. Claude Code mengisi anggaran ini mulai dari skill yang paling baru diinvokasi, jadi skill yang lebih lama dapat dijatuhkan sepenuhnya setelah compaction jika Anda telah menginvokasi banyak dalam satu sesi.

Jika skill tampak berhenti mempengaruhi perilaku setelah respons pertama, konten biasanya masih ada dan model memilih alat atau pendekatan lain. Perkuat deskripsi skill dan instruksi sehingga model terus menyukainya, atau gunakan [hooks](/docs/id/hooks) untuk menegakkan perilaku secara deterministik. Jika skill besar atau Anda menginvokasi beberapa skill lain setelahnya, reinvokasi setelah compaction untuk mengembalikan konten penuh.

<h3 id="pre-approve-tools-for-a-skill">
  Pra-setujui tools untuk skill
</h3>

Bidang `allowed-tools` memberikan izin untuk tools yang terdaftar selama giliran yang menginvokasi skill, sehingga Claude dapat menggunakannya tanpa meminta persetujuan Anda. Hibah dihapus ketika Anda mengirim pesan berikutnya, meskipun konten skill [tetap dalam konteks](#skill-content-lifecycle); menginvokasi skill lagi menerapkannya kembali untuk giliran itu. Ini tidak membatasi tools mana yang tersedia: setiap tool tetap dapat dipanggil, dan [pengaturan izin](/docs/id/permissions) Anda masih mengatur tools yang tidak terdaftar. Untuk pra-setujui tools untuk seluruh sesi daripada satu giliran, tambahkan aturan izin ke pengaturan izin tersebut.

Kepercayaan workspace tidak membatasi bidang ini. Claude Code menerapkan `allowed-tools` skill proyek kapan pun Anda atau Claude menginvokasi skill, termasuk dalam jalankan `-p` dalam folder yang belum pernah Anda percayai. Skill dapat memberikan dirinya akses tool yang luas, jadi tinjau `allowed-tools` skill yang diperiksa ke dalam repositori sebelum Anda menjalankan Claude Code di sana.

Skill ini memungkinkan Claude menjalankan perintah git tanpa persetujuan per-penggunaan kapan pun Anda menginvokasinya:

```yaml theme={null}
---
name: commit
description: Stage and commit the current changes
disable-model-invocation: true
allowed-tools: Bash(git add *) Bash(git commit *) Bash(git status *)
---
```

Untuk menghapus tools dari pool tools yang tersedia Claude saat skill aktif, daftarkan dalam `disallowed-tools` dalam frontmatter skill. Pembatasan dihapus ketika Anda mengirim pesan berikutnya. Seperti aturan deny, bidang tidak dapat menghapus [`EndConversation`](/docs/id/tools-reference#endconversation-tool-behavior) saat tool lain tetap ada. Untuk memblokir tools di semua skills dan prompts, tambahkan aturan deny dalam [pengaturan izin](/docs/id/permissions) Anda.

<h3 id="pass-arguments-to-skills">
  Teruskan argumen ke skills
</h3>

Baik Anda maupun Claude dapat meneruskan argumen saat menginvokasi skill. Argumen tersedia melalui placeholder `$ARGUMENTS`.

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

Ketika Anda menjalankan `/fix-issue 123`, Claude menerima "Fix GitHub issue 123 following our coding standards..."

Jika Anda menginvokasi skill dengan argumen tetapi tidak ada placeholder dalam konten skill yang menerima satu, Claude Code menambahkan `ARGUMENTS: <your input>` ke akhir konten skill sehingga Claude masih melihat apa yang Anda ketik. Placeholder adalah `$ARGUMENTS`, bentuk yang diindeks seperti `$1`, atau argumen bernama. Placeholder yang diindeks tanpa argumen pada posisinya tetap sebagai teks literal dan tidak dihitung sebagai menerima satu. Placeholder bernama dihitung bahkan ketika posisinya tidak memiliki argumen, karena berkembang menjadi string kosong.

Anda juga dapat menumpuk beberapa skills di awal satu pesan. Mengetik `/write-tests /fix-issue 123` memuat kedua skills dan meneruskan teks trailing `123` sebagai `$ARGUMENTS` ke masing-masing. Sebelum v2.1.199, hanya skill pertama yang dimuat dan menerima `/fix-issue 123` sebagai teks argumen literal.

Claude Code memperluas skill pertama ditambah hingga lima lagi yang ditumpuk setelahnya. Ekspansi berhenti pada token pertama yang bukan skill yang dapat diinvokasi pengguna inline, jadi skill yang berjalan sebagai [subagent yang bercabang](#run-skills-in-a-subagent), seperti [`/code-review`](/docs/id/code-review#review-a-diff-locally), atau yang argumennya sendiri mungkin dimulai dengan perintah slash, seperti `/loop`, juga berakhir di sana. Token itu dan segalanya setelahnya menjadi teks argumen untuk setiap skill yang diperluas. `/code-review` berjalan sebagai subagent yang bercabang dari v2.1.218; pada versi sebelumnya itu berjalan inline dan ditumpuk.

Untuk mengakses argumen individual berdasarkan posisi, gunakan `$ARGUMENTS[N]` atau yang lebih pendek `$N`:

```yaml theme={null}
---
name: migrate-component
description: Migrate a component from one language to another
---

Migrate the $ARGUMENTS[0] component from $ARGUMENTS[1] to $ARGUMENTS[2].
Preserve all existing behavior and tests.
```

Menjalankan `/migrate-component SearchBar JavaScript TypeScript` menggantikan `$ARGUMENTS[0]` dengan `SearchBar`, `$ARGUMENTS[1]` dengan `JavaScript`, dan `$ARGUMENTS[2]` dengan `TypeScript`. Skill yang sama menggunakan shorthand `$N`:

```yaml theme={null}
---
name: migrate-component
description: Migrate a component from one language to another
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

Sintaks `` !`<command>` `` menjalankan perintah shell sebelum konten skill dikirim ke Claude. Output perintah menggantikan placeholder, sehingga Claude menerima data aktual, bukan perintah itu sendiri. Claude Code tidak menjalankan perintah ini di mesin Anda ketika skill [disinkronkan dari akun claude.ai Anda](#how-claude-code-handles-the-body-of-a-synced-skill). Pembatasan ini memerlukan Claude Code v2.1.228 atau lebih baru.

Skill ini merangkum pull request dengan mengambil data PR langsung menggunakan GitHub CLI. Perintah `` !`gh pr diff` `` dan perintah lainnya berjalan terlebih dahulu, dan outputnya dimasukkan ke dalam prompt:

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

Substitusi berjalan sekali di atas file asli. Output perintah dimasukkan sebagai teks biasa dan tidak dipindai ulang untuk placeholder `` !`<command>` `` lebih lanjut, sehingga perintah tidak dapat mengeluarkan placeholder untuk pass berikutnya untuk diperluas.

Bentuk inline hanya dikenali ketika `!` muncul di awal baris atau segera setelah whitespace. Jika `!` mengikuti karakter lain, seperti dalam `` KEY=!`cmd` ``, placeholder dibiarkan sebagai teks literal dan perintah tidak berjalan.

Untuk perintah multi-baris, gunakan blok kode yang dibuka dengan ` ```! ` bukan bentuk inline:

````markdown theme={null}
## Environment
```!
node --version
git status --short
```
````

Untuk menonaktifkan perilaku ini untuk skills dan perintah kustom dari pengguna, proyek, plugin, atau sumber [additional-directory](#skills-from-additional-directories), atur `"disableSkillShellExecution": true` dalam [settings](/docs/id/settings). Setiap perintah diganti dengan `[shell command execution disabled by policy]` alih-alih dijalankan. Skills bundel dan terkelola tidak terpengaruh. Pengaturan ini paling berguna dalam [managed settings](/docs/id/managed-settings), di mana pengguna tidak dapat menggantinya.

Claude Code tidak pernah menjalankan perintah ini di mesin Anda ketika perintah muncul dalam skills [disinkronkan dari akun claude.ai Anda](#how-synced-skills-behave), terlepas dari pengaturan ini. Pembatasan ini memerlukan Claude Code v2.1.228 atau lebih baru. [Bagaimana Claude Code menangani isi skill yang disinkronkan](#how-claude-code-handles-the-body-of-a-synced-skill) mengatakan apa yang Claude terima sebagai pengganti perintah dalam setiap jenis sesi.

<Tip>
  Untuk meminta penalaran yang lebih dalam ketika skill berjalan, sertakan `ultrathink` di mana saja dalam konten skill. Lihat [Gunakan ultrathink untuk penalaran mendalam sekali jalan](/docs/id/model-config#use-ultrathink-for-one-off-deep-reasoning).
</Tip>

<h4 id="how-injected-commands-run">
  Bagaimana perintah yang diinjeksi berjalan
</h4>

Claude Code memilih alat yang menjalankan perintah yang diinjeksi skill dari kunci `shell` dalam frontmatter skill dan lingkungan Anda. Setiap kombinasi menjalankan perintah melalui alat Bash atau alat PowerShell, kecuali satu yang gagal dalam invokasi:

* `shell: powershell`, dengan [alat PowerShell](/docs/id/tools-reference#powershell-tool) diaktifkan: perintah berjalan melalui alat PowerShell.
* `shell: bash` ketika bash tidak tersedia: invokasi gagal sebelum perintah apa pun berjalan. Ini terjadi di Windows tanpa Git Bash. Claude Code menampilkan ``Skill <name> requires bash (`shell: bash` in frontmatter) but Git Bash was not found``.
* Kombinasi lainnya: perintah berjalan melalui alat Bash ketika bash tersedia. Ketika tidak, mereka berjalan melalui alat PowerShell.

Kedua alat menjalankan perintah dengan cara yang sama seperti menjalankan perintah shell Claude sendiri. Mereka berbagi direktori kerja, timeout, dan penanganan output:

* **Direktori kerja**: Claude Code menjalankan setiap perintah di direktori kerja shell sesi saat ini. Direktori itu bergerak ketika Claude menjalankan `cd`. Gunakan [`${CLAUDE_SKILL_DIR}` atau `${CLAUDE_PROJECT_DIR}`](#available-string-substitutions) dalam path yang harus diselesaikan dengan cara yang sama setiap kali.
* **stderr**: dengan shell `bash` default, Claude Code menggabungkan stderr ke stdout. Apa pun yang ditulis perintah ke stderr muncul dalam teks yang diinjeksi.
* **Timeout**: setiap perintah berjalan di bawah [timeout](/docs/id/tools-reference#timeout-and-output-limits) default 2 menit alat Bash. Ketika alat Bash [memindahkan perintah yang kedaluwarsa ke latar belakang](/docs/id/tools-reference#background-commands), skill masih dirender. Teks yang diinjeksi melaporkan perpindahan dan menamai tugas latar belakang dan file yang mengumpulkan output perintah. Ketika perintah adalah salah satu yang tidak pernah dilatarbelakangkan oleh alat Bash, Claude Code membunuhnya pada timeout. Kegagalan itu [membatalkan invokasi](#when-an-injected-command-fails).
* **Ukuran output**: output melampaui batas inline alat Bash tiba sebagai jalur file plus pratinjau singkat, bukan teks terpotong. [Output limits](/docs/id/tools-reference#output-limits) mencakup batas dan cara menyesuaikan setiap batas.

Alat PowerShell menerapkan perilaku timeout, backgrounding, dan output-ceiling yang sama pada perintah yang dijalankannya. Lihat bagian [alat PowerShell](/docs/id/tools-reference#powershell-tool) untuk spesifikasinya.

<h4 id="when-an-injected-command-fails">
  Ketika perintah yang diinjeksi gagal
</h4>

Perintah yang gagal membatalkan seluruh invokasi skill, bukan hanya placeholder-nya sendiri. Claude tidak pernah melihat konten skill untuk invokasi itu. Pembatalan menampilkan `Shell command failed for pattern "..."`. Pesan kesalahan mencakup output perintah di bawah `[stderr]`.

Dengan shell `bash` default, kode keluar non-nol apa pun dihitung sebagai kegagalan. Satu pengecualian berlaku: Claude Code memperlakukan kode keluar 1 dari [perintah pencarian dan perbandingan](/docs/id/tools-reference#output-limits) sebagai hasil normal dan menyuntikkan outputnya. Kode keluar 2 atau lebih tinggi gagal bahkan untuk perintah tersebut.

Perintah mana yang mendapatkan pengecualian tergantung pada shell:

* Shell `bash` default: perintah yang tercantum di bawah [Output limits](/docs/id/tools-reference#output-limits)
* `shell: powershell`, ketika alat PowerShell diaktifkan: [set berbeda](/docs/id/tools-reference#shell-selection-in-settings-hooks-and-skills) yang mencakup `grep` dan `git diff` tetapi bukan `find` atau `diff`

Dengan shell `bash` default, tambahkan `|| true` ke perintah lain apa pun yang Anda harapkan keluar non-nol. Skrip pemeriksaan yang keluar 1 ketika menemukan masalah adalah satu contoh.

<h4 id="permission-checks-on-injected-commands">
  Pemeriksaan izin pada perintah yang diinjeksi
</h4>

Perintah yang diinjeksi tidak pernah meminta izin saat skill dirender. Claude Code memeriksa masing-masing terhadap [aturan izin](/docs/id/permissions) Anda terlebih dahulu. Perintah yang cocok dengan aturan deny membatalkan invokasi dengan `Shell command permission check failed for pattern "..."`.

Di luar [mode auto](/docs/id/permission-modes#eliminate-prompts-with-auto-mode), ketika pemeriksaan izin perintah mengembalikan apa pun selain allow, Claude Code membatalkan invokasi dengan kesalahan yang sama. Ini termasuk aturan yang biasanya akan menanyakan Anda. Untuk menjaga perintah yang tidak cocok agar tidak membatalkan di sini, pra-setujui dengan [`allowed-tools`](#pre-approve-tools-for-a-skill). Aturan deny dan ask masih mengganti `allowed-tools`. Lihat [Kelola izin](/docs/id/permissions#manage-permissions).

Dalam mode auto, perintah yang sebaliknya memerlukan persetujuan Anda tidak membatalkan invokasi. Skill dimuat dengan instruksi yang memberi tahu Claude untuk menjalankan perintah terlebih dahulu, dan panggilan Claude sendiri kemudian melalui [pemeriksaan biasa mode auto](/docs/id/permission-modes#how-the-classifier-evaluates-actions). Invokasi masih membatalkan dalam [skill yang di-fork](#run-skills-in-a-subagent) yang menetapkan `agent`, dan dalam sesi di mana Claude tidak memiliki [alat shell yang menjalankan perintah yang diinjeksi](#how-injected-commands-run).

<h3 id="run-skills-in-a-subagent">
  Jalankan skills dalam subagent
</h3>

Tambahkan `context: fork` ke frontmatter Anda ketika Anda ingin skill berjalan dalam isolasi. Claude Code memulai subagent baru dari tipe yang ditetapkan dalam field `agent` dan memberikannya konten skill sebagai promptnya. Subagent tidak melihat riwayat percakapan Anda, sehingga instruksi skill harus berdiri sendiri.

<Note>
  Meskipun namanya, skill dengan `context: fork` tidak berjalan dalam [fork dari percakapan saat ini](/docs/id/sub-agents#fork-the-current-conversation), yang akan memberikan subagent semua yang telah Anda diskusikan sejauh ini. Ketika tugas bergantung pada riwayat itu, fork percakapan alih-alih menggunakan `context: fork`.
</Note>

Subagent yang di-fork berjalan di [latar belakang](/docs/id/sub-agents#run-subagents-in-foreground-or-background): Anda terus bekerja sementara itu berjalan, dan hasilnya tiba dalam percakapan Anda ketika selesai. Atur `background: false` dalam frontmatter untuk menunggu hasil dalam giliran yang menginvokasi skill. Sebelum v2.1.218, skill yang di-fork selalu memblokir giliran sampai selesai.

Claude Code juga menunggu hasil, bahkan ketika skill tidak menetapkan `background: false`, dalam kasus seperti ini:

* Dalam mode non-interaktif, dengan flag `-p` atau Agent SDK
* Ketika Anda menetapkan [`CLAUDE_CODE_DISABLE_BACKGROUND_TASKS`](/docs/id/env-vars) ke `1`, yang juga mematikan semua fitur tugas latar belakang lainnya
* Ketika Anda menginvokasi skill yang di-fork sementara invokasi sebelumnya dari skill yang sama masih berjalan
* Ketika [tugas terjadwal](/docs/id/scheduled-tasks) diaktifkan dengan skill sebagai promptnya

Fork yang di-background juga berjalan dengan [set alat yang lebih sempit yang berlaku untuk subagent latar belakang](/docs/id/sub-agents#run-subagents-in-foreground-or-background): subagent skill adalah tipe agen reguler, jadi pengecualian untuk subagent yang mem-fork percakapan tidak mencakupnya. Jika langkah skill Anda bergantung pada alat di luar set itu, atur `background: false` untuk menjaga set alat penuh.

Skill yang di-fork yang berjalan di latar belakang menerapkan editnya di luar [checkpoints](/docs/id/checkpointing) sesi Anda, jadi `/rewind` tidak membatalkannya; gunakan git untuk mengembalikannya.

<Warning>
  `context: fork` hanya masuk akal untuk skills dengan instruksi eksplisit. Jika skill Anda berisi pedoman seperti "gunakan konvensi API ini" tanpa tugas, subagent menerima pedoman tetapi tidak ada prompt yang dapat ditindaklanjuti, dan kembali tanpa output yang bermakna.
</Warning>

Skills dan [subagents](/docs/id/sub-agents) bekerja bersama dalam dua arah:

| Pendekatan                     | System prompt         | Tugas                 | Juga memuat                                                                                                               |
| :----------------------------- | :-------------------- | :-------------------- | :------------------------------------------------------------------------------------------------------------------------ |
| Skill dengan `context: fork`   | Dari tipe agen        | Konten SKILL.md       | CLAUDE.md, sesuai dengan [startup context](/docs/id/sub-agents#what-loads-at-startup) agen                                     |
| Subagent dengan field `skills` | Isi markdown subagent | Pesan delegasi Claude | Skills yang dimuat sebelumnya + CLAUDE.md, sesuai dengan [startup context](/docs/id/sub-agents#what-loads-at-startup) subagent |

Dengan `context: fork`, Anda menulis tugas dalam skill Anda dan memilih tipe agen untuk menjalankannya. Agen Explore dan Plan bawaan [melewati CLAUDE.md dan status git](/docs/id/sub-agents#what-loads-at-startup) untuk menjaga konteks mereka tetap kecil, jadi skill yang di-fork menggunakan `agent: Explore` hanya melihat konten SKILL.md dan prompt sistem agen sendiri. Untuk kebalikannya, di mana Anda mendefinisikan subagent kustom yang menggunakan skills sebagai materi referensi, lihat [Subagents](/docs/id/sub-agents#preload-skills-into-subagents).

<h4 id="example-research-skill-using-explore-agent">
  Contoh: Skill penelitian menggunakan agen Explore
</h4>

Skill ini menjalankan penelitian dalam agen Explore yang di-fork. Konten skill menjadi tugas, dan agen menyediakan alat read-only yang dioptimalkan untuk eksplorasi codebase:

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

Ketika skill ini berjalan:

1. Konteks terisolasi baru dibuat
2. Subagent menerima konten skill sebagai promptnya (instruksi "Research \$ARGUMENTS thoroughly")
3. Field `agent` menentukan lingkungan eksekusi (model, alat, dan izin)
4. Subagent merangkum hasilnya dan mengembalikannya ke percakapan utama Anda ketika selesai

Field `agent` menentukan konfigurasi subagent mana yang akan digunakan. Opsi mencakup agen bawaan (`Explore`, `Plan`, `general-purpose`) atau subagent kustom apa pun dari `.claude/agents/`. Jika dihilangkan, menggunakan `general-purpose`.

<h3 id="restrict-claude’s-skill-access">
  Batasi akses skill Claude
</h3>

Secara default, Claude dapat menginvokasi skill apa pun yang tidak memiliki `disable-model-invocation: true` yang ditetapkan. Skills yang mendefinisikan `allowed-tools` memberikan Claude akses ke alat tersebut tanpa persetujuan per-penggunaan selama giliran yang menginvokasi skill; hibah dihapus ketika Anda mengirim pesan berikutnya. [Pengaturan izin](/docs/id/permissions) Anda masih mengatur perilaku persetujuan dasar untuk semua alat lainnya. Beberapa perintah bawaan juga tersedia melalui alat Skill, termasuk `/init` dan `/security-review`. Perintah bawaan lainnya seperti `/compact` tidak.

Tiga cara untuk mengontrol skill mana yang dapat diinvokasi Claude:

**Nonaktifkan semua skills** dengan menolak alat Skill dalam `/permissions`:

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

Jika aturan `deny` Anda menamai alias atau nama yang tidak memenuhi syarat daripada nama skill itu sendiri, Claude Code masih memblokir skill: dengan `Skill(review)` itu memblokir bundel `/code-review` melalui alias `/review`-nya, dan dengan `Skill(deploy)` itu memblokir [skill bersarang](#where-skills-live) yang terdaftar sebagai `apps/web:deploy` melalui nama yang tidak memenuhi syaratnya. Sebelum v2.1.260, Claude Code tidak memblokir skill bersarang yang terdaftar di bawah nama yang memenuhi syaratnya ketika aturan deny hanya menamai nama yang tidak memenuhi syarat.

Claude Code mencocokkan aturan `allow` hanya terhadap nama skill itu sendiri dan nama dalam invokasi Claude.

**Sembunyikan skills individual** dengan menambahkan `disable-model-invocation: true` ke frontmatter mereka. Ini menghapus skill dari konteks Claude sepenuhnya.

<Note>
  Dengan `user-invocable: false`, Anda tidak dapat menginvokasi skill, tetapi Claude masih bisa. Untuk menjaga Claude agar tidak menginvokasinya melalui alat Skill, atur `disable-model-invocation: true`.
</Note>

<h3 id="override-skill-visibility-from-settings">
  Ganti visibilitas skill dari pengaturan
</h3>

Pengaturan `skillOverrides` mengontrol visibilitas skill dari [settings](/docs/id/settings) Anda alih-alih frontmatter skill itu sendiri. Gunakan untuk skills yang SKILL.md-nya tidak ingin Anda edit, seperti yang diperiksa ke dalam repo proyek bersama. Menu `/skills` menulisnya untuk Anda: sorot skill dan tekan `Space` untuk mengubah status, lalu `Esc` untuk menyimpan ke `.claude/settings.local.json`.

Setiap kunci adalah nama skill dan setiap nilai adalah salah satu dari empat status:

| Nilai                   | Terdaftar ke Claude | Dalam menu `/` |
| :---------------------- | :------------------ | :------------- |
| `"on"`                  | Nama dan deskripsi  | Ya             |
| `"name-only"`           | Nama saja           | Ya             |
| `"user-invocable-only"` | Tersembunyi         | Ya             |
| `"off"`                 | Tersembunyi         | Tersembunyi    |

Menu `/skills` memberi label status `"user-invocable-only"` `user-only`.

Sejak v2.1.199, `"off"` juga menyembunyikan skill dari daftar perintah yang diiklankan ke klien [Remote Control](/docs/id/remote-control) dan ke pemanggil [Agent SDK](/docs/id/agent-sdk/skills#discover-available-commands), selain menu `/` terminal. Menginvokasi skill tersembunyi dengan nama lengkapnya masih mengembalikan kesalahan `skillOverrides` alih-alih menjalankannya.

Skill yang tidak ada dalam `skillOverrides` diperlakukan sebagai `"on"`. Contoh di bawah ini menciutkan satu skill menjadi namanya dan mematikan yang lain sepenuhnya:

```json theme={null}
{
  "skillOverrides": {
    "legacy-context": "name-only",
    "deploy": "off"
  }
}
```

Beberapa skills bundel memiliki alias, seperti `checkup` untuk `/doctor`. Jika Anda menetapkan entri `skillOverrides` di bawah alias dalam [managed settings](/docs/id/managed-settings) atau dalam file yang Anda lewatkan dengan flag `--settings`, Claude Code menerapkannya ke skill di balik alias. Anda hanya dapat membatasi skill lebih lanjut melalui alias, tidak pernah membuatnya lebih terlihat, dan jika Anda juga menetapkan entri di bawah nama skill itu sendiri dalam managed settings, entri itu memiliki prioritas. Sebelum v2.1.260, Claude Code tidak menerapkan entri di bawah alias ke skill dalam sumber pengaturan apa pun.

Dalam pengaturan pengguna, proyek, dan lokal, Claude Code mencocokkan entri hanya terhadap nama skill. Jika Anda menetapkan entri untuk `review` di sana, itu berlaku untuk skill bernama `review`, bukan bundel `/code-review` melalui alias `/review`-nya.

Plugin skills tidak terpengaruh oleh `skillOverrides`. Kelola mereka melalui `/plugin` sebagai gantinya.

<h3 id="find-unused-skills">
  Temukan skills yang tidak digunakan
</h3>

Setiap skill dalam [daftar skill](#skill-descriptions-are-cut-short) menambah konteks Anda pada setiap giliran, terlepas dari apakah Claude pernah menggunakannya. Jalankan `/skill-doctor` untuk melihat apa yang setiap skill Anda biayai dan seberapa sering digunakan, sehingga Anda dapat memutuskan skill mana yang akan dimatikan. Dalam sesi interaktif, laporan dibuka di tab **Stats** manajer `/plugin`. Dalam [mode non-interaktif](/docs/id/headless) dengan `-p`, Claude Code mencetaknya sebagai teks.

Laporan mencakup skills dalam sesi Anda selain skills bundel dan skills enterprise. Ini menandai skills dalam daftar yang tidak pernah diinvokasi dan mengatakan di mana untuk mematikannya. Dari skills yang diberitahu untuk dimatikan, mulai dengan yang memiliki biaya konteks tertinggi. Laporan juga mencantumkan plugin yang belum Anda gunakan baru-baru ini.

`/skill-doctor` memerlukan Claude Code v2.1.252 atau lebih baru dan tidak tersedia dalam sesi yang melewati [pengambilan flag fitur](/docs/id/env-vars#features-that-need-feature-flag-fetching). Jika Anda menjalankan `/skill-doctor` melalui [Remote Control](/docs/id/remote-control) dari ponsel atau browser Anda, Claude Code menjawab [`Skill usage reports are not available on this connection.`](/docs/id/errors#skill-usage-reports-are-not-available-on-this-connection) sebagai gantinya. Jalankan `/skill-doctor` di terminal pada mesin tempat sesi berjalan.

<h2 id="evaluate-and-iterate-on-a-skill">
  Evaluasi dan iterasi pada sebuah skill
</h2>

Melihat skill terpicu memberitahu Anda bahwa Claude menemukannya, bukan bahwa itu melakukan apa yang Anda maksudkan. Untuk mengetahui skill berfungsi, ukur dua hal secara terpisah: apakah Claude menginvokasinya pada prompt yang seharusnya, dan apakah output cocok dengan apa yang Anda harapkan saat itu terjadi.

Pemeriksaan untuk keduanya adalah perbandingan baseline. Kumpulkan beberapa prompt yang realistis, jalankan masing-masing dalam sesi baru dengan skill tersedia dan lagi dengan itu [dinonaktifkan](#override-skill-visibility-from-settings), dan bandingkan hasilnya. Sesi baru penting karena konteks sisa dari pembuatan skill akan menyembunyikan celah dalam instruksi tertulis.

Dua alat mengotomatisasi perbandingan itu. Untuk skill yang dikirim dalam [plugin](/docs/id/plugins/overview), [`claude plugin eval`](/docs/id/plugin-evals) menjalankan setiap prompt dalam sesi terisolasi dengan dan tanpa plugin, mencetak skornya dengan grader yang Anda tentukan atau yang ditulis untuk Anda, dan keluar dengan non-zero di bawah ambang batas sehingga Anda dapat membatasi CI padanya. Untuk iterasi pada skill tunggal di dalam percakapan Claude Code, plugin skill-creator di bawah menjalankan loop serupa dengan format `evals/evals.json` miliknya sendiri. Dua format tidak dapat dipertukarkan.

<h3 id="run-evals-with-skill-creator">
  Jalankan evals dengan skill-creator
</h3>

Plugin [`skill-creator`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/skill-creator) mengotomatisasi loop perbandingan di dalam Claude Code. Instal dari marketplace resmi:

```text theme={null}
/plugin install skill-creator@claude-plugins-official
```

Jika instalasi gagal, cocokkan pesan yang dilaporkan Claude Code:

* `Marketplace "claude-plugins-official" not found`: tambahkan marketplace dengan `/plugin marketplace add anthropics/claude-plugins-official`, kemudian coba ulang instalasi.
* Plugin [tidak ditemukan di marketplace](/docs/id/plugins/install#install-a-plugin): periksa nama plugin.

Jika ringkasan instalasi melaporkan `Run /reload-plugins to activate.`, Claude Code kemudian menjalankan reload itu untuk Anda. Jika reload memperingatkan bahwa pesan berikutnya Anda akan membaca ulang percakapan, jalankan `/reload-plugins --force` untuk membuat skill plugin tersedia dalam sesi saat ini. Kemudian minta Claude untuk mengevaluasi skill yang ada, misalnya `evaluate my summarize-changes skill with skill-creator`. Plugin memandu Anda melalui penulisan test case dan menjalankan loop:

* **Test cases**: menyimpan prompt, file input, dan perilaku yang diharapkan dalam `evals/evals.json` di dalam direktori skill
* **Isolated runs**: menjalankan [subagent](/docs/id/sub-agents) per test case sehingga setiap run dimulai dengan konteks bersih, dan mencatat jumlah token dan durasi
* **Grading**: memeriksa setiap assertion terhadap output dan menulis pass atau fail dengan bukti ke `grading.json`
* **Benchmark**: mengagregasi pass rate, waktu, dan token untuk with-skill versus without-skill ke dalam `benchmark.json` sehingga Anda dapat membandingkan peningkatan pass-rate terhadap overhead token dan waktu
* **Version comparison**: menjalankan blind A/B antara dua versi skill sehingga Anda dapat mengkonfirmasi edit adalah peningkatan sebelum melakukan commit
* **Description tuning**: menghasilkan prompt should-trigger dan should-not-trigger, mengukur hit rate, dan mengusulkan edit deskripsi saat skill diaktifkan pada permintaan yang salah
* **Review viewer**: membuka laporan HTML di mana Anda memeriksa setiap output dan mencatat umpan balik kualitatif yang dibaca iterasi berikutnya

Untuk format file eval dan alur kerja iterasi lengkap, lihat [Evaluating skill output quality](https://agentskills.io/skill-creation/evaluating-skills) di agentskills.io. Untuk latar belakang pada mode benchmark dan perbandingan, lihat [skill-creator announcement](https://claude.com/blog/improving-skill-creator-test-measure-and-refine-agent-skills).

<h2 id="share-skills">
  Bagikan skills
</h2>

Skills dapat didistribusikan pada berbagai cakupan tergantung pada audiens Anda:

* **Project skills**: Commit `.claude/skills/` ke version control
* **Plugins**: Buat direktori `skills/` di [plugin](/docs/id/plugins/overview) Anda
* **Managed**: Terapkan di seluruh organisasi melalui [managed settings](/docs/id/managed-settings)

<h3 id="generate-visual-output">
  Hasilkan output visual
</h3>

Skills dapat membundel dan menjalankan skrip dalam bahasa apa pun, memberikan Claude kemampuan di luar apa yang mungkin dalam satu prompt. Satu pola adalah menghasilkan output visual: file HTML interaktif yang terbuka di browser Anda untuk menjelajahi data, debugging, atau membuat laporan.

Contoh ini membuat penjelajah codebase: tampilan pohon interaktif di mana Anda dapat memperluas dan menciutkan direktori, melihat ukuran file sekilas, dan mengidentifikasi jenis file berdasarkan warna.

Buat direktori Skill:

```bash theme={null}
mkdir -p ~/.claude/skills/codebase-visualizer/scripts
```

Simpan ini ke `~/.claude/skills/codebase-visualizer/SKILL.md`. Deskripsi memberi tahu Claude kapan harus mengaktifkan Skill ini, dan instruksi memberi tahu Claude untuk menjalankan skrip yang dibundel. Jalur skrip menggunakan [`${CLAUDE_SKILL_DIR}`](#available-string-substitutions) sehingga dapat diselesaikan dengan benar apakah skill dipasang di tingkat personal, project, atau plugin:

````yaml theme={null}
---
name: codebase-visualizer
description: Generate an interactive collapsible tree visualization of your codebase. Use when exploring a new repo, understanding project structure, or identifying large files.
allowed-tools: Bash(python3 *)
---

# Codebase Visualizer

Hasilkan tampilan pohon HTML interaktif yang menunjukkan struktur file proyek Anda dengan direktori yang dapat diciutkan.

## Penggunaan

Jalankan skrip visualisasi dari root proyek Anda:

```bash
python3 ${CLAUDE_SKILL_DIR}/scripts/visualize.py .
```

Ini membuat `codebase-map.html` di direktori saat ini dan membukanya di browser default Anda.

## Apa yang ditampilkan visualisasi

- **Direktori yang dapat diciutkan**: Klik folder untuk memperluas/menciutkan
- **Ukuran file**: Ditampilkan di sebelah setiap file
- **Warna**: Warna berbeda untuk jenis file berbeda
- **Total direktori**: Menunjukkan ukuran agregat setiap folder
````

Simpan ini ke `~/.claude/skills/codebase-visualizer/scripts/visualize.py`. Skrip ini memindai pohon direktori dan menghasilkan file HTML yang mandiri dengan:

* **Sidebar ringkasan** yang menampilkan jumlah file, jumlah direktori, ukuran total, dan jumlah jenis file
* **Bagan batang** yang memecah codebase berdasarkan jenis file (8 teratas berdasarkan ukuran)
* **Pohon yang dapat diciutkan** di mana Anda dapat memperluas dan menciutkan direktori, dengan indikator jenis file berkode warna

Skrip memerlukan Python 3 tetapi hanya menggunakan perpustakaan bawaan, jadi tidak ada paket yang perlu dipasang:

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

Untuk menguji, buka Claude Code di proyek apa pun dan minta "Visualize this codebase." Claude menjalankan skrip, yang mencetak jalur file yang dihasilkan, seperti `Generated /path/to/codebase-map.html`, dan membukanya di browser Anda. Jika Anda bekerja di lingkungan headless di mana tidak ada browser yang terbuka, jalur yang dicetak mengkonfirmasi bahwa skrip berhasil.

Pola ini berfungsi untuk output visual apa pun: grafik dependensi, laporan cakupan pengujian, dokumentasi API, atau visualisasi skema database. Skrip yang dibundel melakukan pekerjaan sementara Claude menangani orkestrasi.

<h2 id="troubleshooting">
  Troubleshooting
</h2>

<h3 id="skill-not-triggering">
  Skill tidak terpicu
</h3>

Jika Claude tidak menggunakan skill Anda saat diharapkan:

1. Periksa deskripsi mencakup kata kunci yang akan secara alami diucapkan pengguna
2. Verifikasi skill muncul di `What skills are available?`
3. Coba rephrase permintaan Anda untuk lebih cocok dengan deskripsi
4. Panggil secara langsung dengan `/skill-name` jika skill dapat diinvokasi oleh pengguna

Jika YAML frontmatter tidak valid, Claude Code memuat badan skill dengan metadata kosong, jadi `/skill-name` tetap berfungsi tetapi Claude tidak dapat mencocokkan terhadap `description` Anda. Jalankan dengan `--debug` untuk melihat error parse.

Jika skill dikirim dalam plugin, Anda dapat mengukur seberapa sering skill tersebut terpicu di seluruh prompt yang realistis daripada memeriksa satu per satu: tulis kasus eval dengan [`tool_used: Skill` grader](/docs/id/plugin-evals#create-your-first-eval-suite) dan jalankan dengan `claude plugin eval` setelah setiap perubahan deskripsi.

Untuk menemukan file `SKILL.md` yang frontmatter-nya tidak parse, jalankan [`claude plugin validate`](/docs/id/plugins/cli-reference#validate-a-directory) pada direktori skills, misalnya `claude plugin validate .claude/skills` untuk project skills atau `claude plugin validate ~/.claude/skills` untuk personal skills. Memerlukan Claude Code v2.1.233 atau lebih baru.

<h3 id="skill-triggers-too-often">
  Skill terpicu terlalu sering
</h3>

Jika Claude menggunakan skill Anda saat Anda tidak menginginkannya:

1. Buat deskripsi lebih spesifik
2. Tambahkan `disable-model-invocation: true` jika Anda hanya menginginkan invokasi manual

<h3 id="skill-descriptions-are-cut-short">
  Deskripsi skill terpotong
</h3>

Claude Code memuat daftar nama skill dan deskripsi ke dalam konteks sehingga Claude tahu apa yang tersedia. Daftar selalu berisi setiap nama skill, tetapi jika Anda memiliki banyak skill, Claude Code mempersingkat deskripsi agar sesuai dengan anggaran karakter daftar, yang dapat menghilangkan kata kunci yang Claude butuhkan untuk mencocokkan permintaan Anda. Anggaran diskalakan pada 1% dari jendela konteks model. Ketika daftar melampaui batas, Claude Code menghapus deskripsi dimulai dengan skill yang Anda panggil paling sedikit, sehingga skill yang Anda gunakan paling banyak mempertahankan teks lengkap mereka.

Jalankan `/doctor` untuk estimasi biaya konteks daftar dan kontributor terbesarnya. Untuk menemukan skill yang layak dimatikan, jalankan [`/skill-doctor`](#find-unused-skills). Ketika daftar melebihi anggarannya, Claude Code juga menulis peringatan ke debug log, terlihat dengan [`--debug`](/docs/id/cli-reference#cli-flags).

Baris Skills di `/context` melaporkan ukuran daftar setelah anggaran diterapkan, sehingga cocok dengan apa yang diterima model. Sebelum v2.1.196, baris menghitung teks lengkap setiap deskripsi dan dapat menunjukkan nilai beberapa kali lebih besar dari anggaran yang dikonfigurasi.

Untuk menaikkan anggaran, atur pengaturan [`skillListingBudgetFraction`](/docs/id/settings-reference#skilllistingbudgetfraction) (misalnya `0.02` = 2%) atau variabel lingkungan `SLASH_COMMAND_TOOL_CHAR_BUDGET` ke jumlah karakter tetap. Untuk membebaskan anggaran untuk skill lain, atur entri prioritas rendah ke `"name-only"` di [`skillOverrides`](#override-skill-visibility-from-settings) sehingga mereka terdaftar tanpa deskripsi. Anda juga dapat memangkas teks `description` dan `when_to_use` di sumber: letakkan kasus penggunaan utama terlebih dahulu, karena teks gabungan setiap entri dibatasi pada 1.536 karakter terlepas dari anggaran. Batas dapat dikonfigurasi dengan [`skillListingMaxDescChars`](/docs/id/settings-reference#skilllistingmaxdescchars).

<h3 id="personal-skills-disappeared">
  Personal skills hilang
</h3>

Jika folder skill yang Anda buat di `~/.claude/skills/` hilang, lihat di `~/.claude/skills/.trash/`. Ketika Claude Code [menyinkronkan skill dari claude.ai](#how-synced-skills-behave), skill tersebut diunduh ke subfolder `synced` terpisah dan tidak memindahkan atau menghapus folder yang Anda buat.

Sebelum v2.1.280, file bernama `manifest.json` di `~/.claude/skills/` menyebabkan Claude Code memindahkan folder skill yang terdaftar dalam file tersebut ke folder dengan stempel waktu di bawah `~/.claude/skills/.trash/`, dan skill tersebut berhenti dimuat.

Untuk memulihkan skill, pindahkan foldernya dari folder dengan stempel waktu kembali ke `~/.claude/skills/`. Lakukan ini sebelum [retention sweep](/docs/id/claude-directory#cleaned-up-automatically) menghapus entri trash, secara default 30 hari setelah dipindahkan ke trash.

<h2 id="related-resources">
  Sumber daya terkait
</h2>

* **[Debug konfigurasi Anda](/docs/id/debug-your-config)**: diagnosis mengapa skill tidak muncul atau tidak terpicu
* **[Mengevaluasi kualitas output skill](https://agentskills.io/skill-creation/evaluating-skills)**: format file eval dan alur kerja iterasi di agentskills.io
* **[Praktik terbaik penulisan skill](https://platform.claude.com/docs/en/agents-and-tools/agent-skills/best-practices)**: panduan penulisan yang berlaku di seluruh produk Claude
* **[Subagents](/docs/id/sub-agents)**: delegasikan tugas ke agen khusus
* **[Plugins](/docs/id/plugins/overview)**: paket dan distribusikan skills dengan ekstensi lainnya
* **[Hooks](/docs/id/hooks)**: otomatisasi workflow di sekitar peristiwa tool
* **[Memory](/docs/id/memory)**: kelola file CLAUDE.md untuk konteks persisten
* **[Commands](/docs/id/commands)**: referensi untuk perintah bawaan dan skills bundel
* **[Permissions](/docs/id/permissions)**: kontrol akses tool dan skill
* **[Claude Tag skills](https://claude.com/docs/claude-tag/admins/skills-repo)**: project skills yang di-commit ke repo juga dimuat ketika repo tersebut digunakan di saluran Claude Tag
