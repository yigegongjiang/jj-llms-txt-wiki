> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Bagaimana Claude mengingat proyek Anda

> Berikan Claude instruksi persisten dengan file CLAUDE.md atau AGENTS.md, dan biarkan Claude mengumpulkan pembelajaran secara otomatis dengan auto memory.

Setiap sesi Claude Code dimulai dengan context window yang segar. Dua mekanisme membawa pengetahuan lintas sesi:

* **File CLAUDE.md**: instruksi yang Anda tulis untuk memberikan Claude konteks persisten. Claude juga dapat membaca file [`AGENTS.md`](#agents-md) repositori, sendiri atau bersama CLAUDE.md
* **Auto memory**: catatan yang Claude tulis sendiri berdasarkan koreksi dan preferensi Anda

Halaman ini mencakup cara untuk:

* [Menulis dan mengorganisir file CLAUDE.md](#claude-md-files)
* [Menggunakan AGENTS.md yang sudah ada](#agents-md) sebagai instruksi proyek Anda, sendiri atau bersama CLAUDE.md
* [Membatasi aturan ke tipe file tertentu](#organize-rules-with-claude/rules/) dengan `.claude/rules/`
* [Mengonfigurasi auto memory](#auto-memory) agar Claude membuat catatan secara otomatis
* [Troubleshoot](#troubleshoot-memory-issues) ketika instruksi tidak diikuti

<h2 id="claude-md-vs-auto-memory">
  CLAUDE.md vs auto memory
</h2>

Claude Code memiliki dua sistem memori yang saling melengkapi. Keduanya dimuat di awal setiap percakapan. Claude memperlakukan mereka sebagai konteks, bukan konfigurasi yang diberlakukan. Untuk memblokir suatu tindakan terlepas dari apa yang Claude putuskan, gunakan [hook PreToolUse](/docs/id/hooks-guide) sebagai gantinya. Semakin spesifik dan ringkas instruksi Anda, semakin konsisten Claude mengikutinya.

|                           | File CLAUDE.md                                    | Auto memory                                                                                                         |
| :------------------------ | :------------------------------------------------ | :------------------------------------------------------------------------------------------------------------------ |
| **Siapa yang menulisnya** | Anda                                              | Claude                                                                                                              |
| **Apa yang dikandungnya** | Instruksi dan aturan                              | Pembelajaran dan pola                                                                                               |
| **Cakupan**               | Proyek, pengguna, atau organisasi                 | Per repositori, dibagikan di seluruh worktrees                                                                      |
| **Dimuat ke dalam**       | Setiap sesi                                       | Setiap sesi (200 baris pertama atau 25KB)                                                                           |
| **Gunakan untuk**         | Standar pengkodean, alur kerja, arsitektur proyek | Preferensi Anda, koreksi yang Anda berikan kepada Claude, konteks proyek yang Claude tidak dapat turunkan dari kode |

Gunakan file CLAUDE.md ketika Anda ingin memandu perilaku Claude. Auto memory memungkinkan Claude belajar dari koreksi Anda tanpa usaha manual.

Subagents juga dapat mempertahankan auto memory mereka sendiri. Lihat [konfigurasi subagent](/docs/id/sub-agents#enable-persistent-memory) untuk detail.

<h2 id="claude-md-files">
  File CLAUDE.md
</h2>

File CLAUDE.md adalah file markdown yang memberikan instruksi persisten kepada Claude untuk proyek, alur kerja pribadi Anda, atau seluruh organisasi Anda. Anda menulis file ini dalam teks biasa; Claude membacanya di awal setiap sesi. Jika repositori Anda menggunakan `AGENTS.md` sebagai gantinya, lihat [AGENTS.md](#agents-md).

<h3 id="when-to-add-to-claude-md">
  Kapan menambahkan ke CLAUDE.md
</h3>

Perlakukan CLAUDE.md sebagai tempat Anda menuliskan apa yang sebaliknya akan Anda jelaskan kembali. Tambahkan ke dalamnya ketika:

* Claude membuat kesalahan yang sama untuk kedua kalinya
* Tinjauan kode menangkap sesuatu yang seharusnya Claude ketahui tentang basis kode ini
* Anda mengetik koreksi atau klarifikasi yang sama ke dalam chat yang Anda ketik di sesi terakhir
* Anggota tim baru akan membutuhkan konteks yang sama untuk produktif

Pertahankan itu untuk fakta yang harus Claude pegang di setiap sesi: perintah build, konvensi, tata letak proyek, aturan "selalu lakukan X". Jika entri adalah prosedur multi-langkah atau hanya penting untuk satu bagian dari basis kode, pindahkan ke [skill](/docs/id/skills) atau [aturan dengan cakupan path](#organize-rules-with-claude/rules/) sebagai gantinya. [Ringkasan ekstensi](/docs/id/features-overview#build-your-setup-over-time) mencakup kapan menggunakan setiap mekanisme.

<h3 id="choose-where-to-put-claude-md-files">
  Pilih di mana menempatkan file CLAUDE.md
</h3>

File CLAUDE.md dapat berada di beberapa lokasi, masing-masing dengan cakupan berbeda. Tabel di bawah mencantumnya dalam urutan pemuatan, dari cakupan terluas hingga paling spesifik, sehingga instruksi proyek muncul dalam konteks setelah instruksi pengguna.

| Cakupan                 | Lokasi                                                                                                                                                                  | Tujuan                                                       | Contoh kasus penggunaan                                                  | Dibagikan dengan                   |
| ----------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------ | ------------------------------------------------------------------------ | ---------------------------------- |
| **Kebijakan terkelola** | • macOS: `/Library/Application Support/ClaudeCode/CLAUDE.md`<br />• Linux dan WSL: `/etc/claude-code/CLAUDE.md`<br />• Windows: `C:\Program Files\ClaudeCode\CLAUDE.md` | Instruksi di seluruh organisasi yang dikelola oleh IT/DevOps | Standar pengkodean perusahaan, kebijakan keamanan, persyaratan kepatuhan | Semua pengguna dalam organisasi    |
| **Instruksi pengguna**  | `~/.claude/CLAUDE.md`                                                                                                                                                   | Preferensi pribadi untuk semua proyek                        | Preferensi gaya kode, pintasan alat pribadi                              | Hanya Anda (semua proyek)          |
| **Instruksi proyek**    | `./CLAUDE.md` atau `./.claude/CLAUDE.md`. Lihat [AGENTS.md](#agents-md) untuk kapan `./AGENTS.md` dimuat sebagai gantinya atau bersama dengannya                        | Instruksi bersama tim untuk proyek                           | Arsitektur proyek, standar pengkodean, alur kerja umum                   | Anggota tim melalui kontrol sumber |
| **Instruksi lokal**     | `./CLAUDE.local.md`                                                                                                                                                     | Preferensi pribadi khusus proyek; tambahkan ke `.gitignore`  | URL sandbox Anda, data uji pilihan                                       | Hanya Anda (proyek saat ini)       |

File CLAUDE.md dan CLAUDE.local.md dalam hierarki direktori di atas direktori kerja dimuat saat peluncuran. File di subdirektori dimuat sesuai permintaan ketika Claude membaca file di direktori tersebut. Lihat [Bagaimana file CLAUDE.md dimuat](#how-claude-md-files-load) untuk urutan resolusi lengkap.

Untuk proyek besar, Anda dapat memecah instruksi menjadi file khusus topik menggunakan [aturan proyek](#organize-rules-with-claude/rules/). Aturan memungkinkan Anda membatasi instruksi ke jenis file atau subdirektori tertentu.

<h3 id="set-up-a-project-claude-md">
  Siapkan CLAUDE.md proyek
</h3>

CLAUDE.md proyek dapat disimpan di `./CLAUDE.md` atau `./.claude/CLAUDE.md`. Buat file ini dan tambahkan instruksi yang berlaku untuk siapa pun yang bekerja pada proyek: perintah build dan test, standar pengkodean, keputusan arsitektur, konvensi penamaan, dan alur kerja umum. Instruksi ini dibagikan dengan tim Anda melalui kontrol versi, jadi fokus pada standar tingkat proyek daripada preferensi pribadi. Untuk mengonfirmasi file dimuat, jalankan `/context` dalam sesi dan periksa daftar di bawah **Memory files**.

<Tip>
  Jalankan `/init` untuk menghasilkan CLAUDE.md awal secara otomatis. Claude menganalisis basis kode Anda dan membuat file dengan perintah build, instruksi test, dan konvensi proyek yang ditemukannya. Jika CLAUDE.md sudah ada, `/init` menyarankan perbaikan daripada menimpanya. Perbaiki dari sana dengan instruksi yang Claude tidak akan temukan sendiri.

  Untuk alur multi-fase interaktif sebagai gantinya, atur variabel lingkungan `CLAUDE_CODE_NEW_INIT` ke `1` sebelum Anda menjalankan `/init`. Atur di shell Anda atau di blok `env` file pengaturan, seperti yang ditunjukkan dalam [Atur variabel lingkungan](/docs/id/env-vars#set-environment-variables). Dengan itu diatur, `/init` menanyakan artefak mana yang akan diatur: file CLAUDE.md, skills, dan hooks. Kemudian mengeksplorasi basis kode Anda dengan subagent, mengisi celah melalui pertanyaan lanjutan, dan menyajikan proposal yang dapat ditinjau sebelum menulis file apa pun. Variabel hanya mengubah cara `/init` berjalan, jadi Anda dapat membiarkannya diatur.
</Tip>

<h3 id="write-effective-instructions">
  Tulis instruksi yang efektif
</h3>

File CLAUDE.md dimuat ke jendela konteks di awal setiap sesi, mengonsumsi token bersama percakapan Anda. [Visualisasi jendela konteks](/docs/id/context-window) menunjukkan di mana CLAUDE.md dimuat relatif terhadap sisa konteks startup. Karena mereka adalah konteks daripada konfigurasi yang ditegakkan, cara Anda menulis instruksi mempengaruhi seberapa andal Claude mengikutinya. Instruksi yang spesifik, ringkas, dan terstruktur dengan baik bekerja paling baik.

**Ukuran**: targetkan di bawah 200 baris per file CLAUDE.md. File yang lebih panjang mengonsumsi lebih banyak konteks dan mengurangi kepatuhan. Jika instruksi Anda berkembang besar, gunakan [aturan dengan cakupan path](#path-specific-rules) sehingga instruksi hanya dimuat ketika Claude bekerja dengan file yang cocok. Anda juga dapat membagi konten menjadi [impor](#import-additional-files) untuk organisasi, meskipun file yang diimpor masih dimuat dan memasuki jendela konteks saat peluncuran.

**Struktur**: gunakan header markdown dan bullet untuk mengelompokkan instruksi terkait. Claude memindai struktur dengan cara yang sama seperti pembaca: bagian yang terorganisir lebih mudah diikuti daripada paragraf padat.

**Spesifisitas**: tulis instruksi yang cukup konkret untuk diverifikasi. Sebagai contoh:

* "Gunakan indentasi 2 spasi" daripada "Format kode dengan benar"
* "Jalankan `npm test` sebelum commit" daripada "Uji perubahan Anda"
* "Handler API berada di `src/api/handlers/`" daripada "Jaga file tetap terorganisir"

**Konsistensi**: jika dua aturan saling bertentangan, Claude mungkin memilih satu secara sewenang-wenang. Tinjau file CLAUDE.md Anda, file CLAUDE.md bersarang di subdirektori, dan [`.claude/rules/`](#organize-rules-with-claude/rules/) secara berkala untuk menghapus instruksi yang ketinggalan zaman atau bertentangan. Dalam monorepo, gunakan [`claudeMdExcludes`](#exclude-specific-claude-md-files) untuk melewati file CLAUDE.md dari tim lain yang tidak relevan dengan pekerjaan Anda.

<h3 id="import-additional-files">
  Impor file tambahan
</h3>

File CLAUDE.md dapat mengimpor file tambahan menggunakan sintaks `@path/to/import`. File yang diimpor diperluas dan dimuat ke konteks saat peluncuran bersama CLAUDE.md yang mereferensikannya.

Jalur relatif dan absolut diizinkan. Jalur relatif diselesaikan relatif terhadap file yang berisi impor, bukan direktori kerja. File yang diimpor dapat secara rekursif mengimpor file lain, dengan kedalaman maksimal empat hop.

Penguraian impor melewati rentang kode Markdown dan blok kode yang dibatasi. Untuk menyebutkan jalur di CLAUDE.md Anda tanpa mengimpornya, bungkus dalam backtick: menulis `` `@README` `` membuat teks literal, sementara `@README` di luar backtick mengimpor file.

Untuk menarik README, package.json, dan panduan alur kerja, referensikan dengan sintaks `@` di mana saja di CLAUDE.md Anda:

```text theme={null}
Lihat @README untuk ringkasan proyek dan @package.json untuk perintah npm yang tersedia untuk proyek ini.

# Instruksi Tambahan
- alur kerja git @docs/git-instructions.md
```

Untuk preferensi pribadi per-proyek yang tidak boleh diperiksa ke kontrol versi, buat `CLAUDE.local.md` di akar proyek. Ini dimuat bersama `CLAUDE.md` dan diperlakukan dengan cara yang sama. Tambahkan `CLAUDE.local.md` ke `.gitignore` Anda sehingga tidak dikomit. Dengan `CLAUDE_CODE_NEW_INIT=1` diatur, menjalankan `/init` dan memilih opsi pribadi melakukan ini untuk Anda.

Jika Anda bekerja di beberapa git worktrees dari repositori yang sama, `CLAUDE.local.md` yang diabaikan git hanya ada di worktree tempat Anda membuatnya. Untuk berbagi instruksi pribadi di seluruh worktrees, impor file dari direktori home Anda sebagai gantinya:

```text theme={null}
# Preferensi Individu
- @~/.claude/my-project-instructions.md
```

<Warning>
  Impor dalam file memori tingkat proyek bersifat eksternal ketika jalurnya diselesaikan di luar direktori kerja Anda, seperti impor direktori home di atas. Pertama kali Claude Code menemukan impor eksternal dalam proyek, ia menampilkan dialog persetujuan yang mencantumkan file. Jika Anda menolak, impor tetap dinonaktifkan dan dialog tidak muncul lagi.

  Claude Code menampilkan dialog untuk melindungi Anda dari file yang orang lain komit ke proyek bersama. File memori tingkat pengguna, seperti `~/.claude/CLAUDE.md` dan `~/.claude/rules/`, adalah file yang Anda tulis sendiri. Kecuali dalam sesi [Cowork](https://claude.com/product/cowork) di desktop Anda, Claude Code memuat impor mereka tanpa dialog dan mempercayai mereka seperti sisa konfigurasi pribadi Anda.

  Dalam sesi Cowork di desktop Anda, Claude Code melewati impor apa pun dalam file tingkat pengguna yang diselesaikan ke jalur di luar direktori kerja sesi dan memuat sisa file. Dalam sesi tersebut juga melewati `~/.claude/CLAUDE.md` yang merupakan symlink atau hard link, dan direktori `~/.claude/rules/` atau file aturan yang disymlink yang menunjuk di luar direktori kerja.
</Warning>

<h3 id="how-claude-md-files-load">
  Bagaimana file CLAUDE.md dimuat
</h3>

Claude Code memuat `CLAUDE.md` dan `CLAUDE.local.md` dari direktori kerja saat ini dan setiap direktori di atasnya. Jalankan Claude Code di `foo/bar/` dan itu memuat instruksi dari `foo/bar/CLAUDE.md`, `foo/CLAUDE.md`, dan file `CLAUDE.local.md` apa pun di sampingnya.

Semua file yang ditemukan digabungkan ke dalam konteks daripada menimpa satu sama lain. Di seluruh pohon direktori, konten diurutkan dari akar sistem file ke bawah ke direktori kerja Anda. Untuk contoh `foo/bar/`, `foo/CLAUDE.md` muncul dalam konteks sebelum `foo/bar/CLAUDE.md`, jadi instruksi lebih dekat ke tempat Anda meluncurkan Claude dibaca terakhir. Dalam setiap direktori, `CLAUDE.local.md` ditambahkan setelah `CLAUDE.md`, jadi catatan pribadi Anda adalah hal terakhir yang Claude baca di tingkat itu.

Claude juga menemukan file `CLAUDE.md` dan `CLAUDE.local.md` di subdirektori di bawah direktori kerja saat ini. Daripada memuat mereka saat peluncuran, mereka disertakan ketika Claude membaca file di subdirektori tersebut.

Jika Anda bekerja di monorepo besar di mana file CLAUDE.md tim lain diambil, gunakan [`claudeMdExcludes`](#exclude-specific-claude-md-files) untuk melewatinya. Untuk tata letak lengkap file CLAUDE.md akar dan per-direktori serta aturan, lihat [Monorepo dan repositori besar](/docs/id/large-codebases).

Komentar HTML tingkat blok (`<!-- maintainer notes -->`) dalam file CLAUDE.md dilepas sebelum konten disuntikkan ke konteks Claude. Gunakan mereka untuk meninggalkan catatan untuk pengelola manusia tanpa menghabiskan token konteks pada mereka. Komentar di dalam blok kode dipertahankan. Ketika Anda membuka file CLAUDE.md langsung dengan alat Read, komentar tetap terlihat.

<h4 id="load-from-additional-directories">
  Muat dari direktori tambahan
</h4>

Bendera `--add-dir` memberikan Claude akses ke direktori tambahan di luar direktori kerja utama Anda. Secara default, file CLAUDE.md dari direktori ini tidak dimuat.

Untuk juga memuat file memori dari direktori tambahan, atur variabel lingkungan `CLAUDE_CODE_ADDITIONAL_DIRECTORIES_CLAUDE_MD`:

```bash theme={null}
CLAUDE_CODE_ADDITIONAL_DIRECTORIES_CLAUDE_MD=1 claude --add-dir ../shared-config
```

Bentuk inline menetapkan variabel untuk peluncuran itu saja di Bash atau Zsh. Untuk menjaganya tetap aktif untuk setiap sesi, tambahkan ke blok `env` di `~/.claude/settings.json` seperti yang ditunjukkan dalam [Atur variabel lingkungan](/docs/id/env-vars#set-environment-variables).

Ini memuat `CLAUDE.md`, `.claude/CLAUDE.md`, `.claude/rules/*.md`, dan `CLAUDE.local.md` dari direktori tambahan. `CLAUDE.local.md` dilewati jika Anda mengecualikan `local` dari [`--setting-sources`](/docs/id/cli-reference).

<h3 id="organize-rules-with-claude/rules/">
  Atur aturan dengan `.claude/rules/`
</h3>

Untuk proyek yang lebih besar, Anda dapat mengatur instruksi menjadi beberapa file menggunakan direktori `.claude/rules/`. Ini membuat instruksi modular dan lebih mudah bagi tim untuk dipertahankan. Aturan juga dapat [dibatasi ke jalur file tertentu](#path-specific-rules), sehingga mereka hanya dimuat ke konteks ketika Claude bekerja dengan file yang cocok, mengurangi kebisingan dan menghemat ruang konteks.

<Note>
  Aturan dimuat ke konteks setiap sesi atau ketika file yang cocok dibuka. Untuk instruksi khusus tugas yang tidak perlu berada dalam konteks sepanjang waktu, gunakan [skills](/docs/id/skills) sebagai gantinya, yang hanya dimuat ketika Anda menginvokasinya atau ketika Claude menentukan mereka relevan dengan prompt Anda.
</Note>

<h4 id="set-up-rules">
  Siapkan aturan
</h4>

Tempatkan file markdown di direktori `.claude/rules/` proyek Anda. Setiap file harus mencakup satu topik, dengan nama file deskriptif seperti `testing.md` atau `api-design.md`. Semua file `.md` ditemukan secara rekursif, sehingga Anda dapat mengatur aturan ke dalam subdirektori seperti `frontend/` atau `backend/`:

```text theme={null}
your-project/
├── .claude/
│   ├── CLAUDE.md           # Instruksi proyek utama
│   └── rules/
│       ├── code-style.md   # Pedoman gaya kode
│       ├── testing.md      # Konvensi pengujian
│       └── security.md     # Persyaratan keamanan
```

Aturan tanpa [frontmatter `paths`](#path-specific-rules) dimuat saat peluncuran dengan prioritas yang sama seperti `.claude/CLAUDE.md`.

Aturan proyek dilewati jika Anda mengecualikan `project` dari [`--setting-sources`](/docs/id/cli-reference). Sebelum v2.1.211, aturan yang dimuat sesuai permintaan, termasuk aturan dengan cakupan path dan aturan di direktori `.claude/rules/` bersarang, dimuat bahkan ketika `project` dikecualikan.

<h4 id="path-specific-rules">
  Aturan khusus path
</h4>

Aturan dapat dibatasi ke file tertentu menggunakan frontmatter YAML dengan bidang `paths`. Aturan bersyarat ini hanya berlaku ketika Claude bekerja dengan file yang cocok dengan pola yang ditentukan.

```markdown theme={null}
---
paths:
  - "src/api/**/*.ts"
---

# Aturan Pengembangan API

- Semua endpoint API harus menyertakan validasi input
- Gunakan format respons kesalahan standar
- Sertakan komentar dokumentasi OpenAPI
```

Aturan tanpa bidang `paths` dimuat tanpa syarat dan berlaku untuk semua file. Aturan dengan cakupan path dipicu ketika Claude membaca file yang cocok dengan pola, bukan pada setiap penggunaan alat. Mulai dari v2.1.198, pencocokan juga berfungsi ketika Claude mencapai file melalui jalur symlink ke direktori proyek, misalnya dalam checkout yang disymlink.

Gunakan pola glob dalam bidang `paths` untuk mencocokkan file berdasarkan ekstensi, direktori, atau kombinasi apa pun:

| Pola                   | Cocok dengan                               |
| ---------------------- | ------------------------------------------ |
| `**/*.ts`              | Semua file TypeScript di direktori apa pun |
| `src/**/*`             | Semua file di bawah direktori `src/`       |
| `*.md`                 | File Markdown di akar proyek               |
| `src/components/*.tsx` | Komponen React di direktori tertentu       |

Anda dapat menentukan beberapa pola dan menggunakan ekspansi brace untuk mencocokkan beberapa ekstensi dalam satu pola:

```markdown theme={null}
---
paths:
  - "src/**/*.{ts,tsx}"
  - "lib/**/*.ts"
  - "tests/**/*.test.ts"
---
```

Setiap grup brace mengalikan jumlah pola yang diperluas: `src/*.{ts,tsx}` diperluas menjadi dua pola, dan `{a,b}/{c,d}/*.{ts,tsx}` menjadi delapan. Untuk menjaga ekspansi terbatas, seluruh daftar `paths` aturan berbagi satu anggaran 1.000 pola yang diperluas dan 4 MiB, dan pola tanpa brace tidak dihitung terhadapnya.

Claude Code menggunakan pola apa pun yang akan melampaui anggaran yang tidak diperluas, dan brace literal mereka tidak cocok dengan file apa pun. Sebelum v2.1.217, nilai `paths` dengan banyak grup brace menghentikan atau menghancurkan CLI saat startup.

Sintaks Glob memperlakukan `[` sebagai awal ekspresi bracket seperti `[abc]`. Pola dengan `[` yang tidak dapat dibaca sebagai ekspresi bracket, seperti `photos [2024/**`, tidak valid: itu tidak cocok dengan file apa pun, dan pola lain aturan terus bekerja. Untuk mencocokkan `[` literal dalam nama file, lepaskan sebagai `photos \[2024/**`. Sebelum v2.1.207, satu pola tidak valid membuat alat Read gagal untuk setiap file aturan dievaluasi terhadap, daripada tidak cocok dengan apa pun.

<h4 id="rules-frontmatter-reference">
  Referensi frontmatter aturan
</h4>

Konfigurasi aturan dengan [frontmatter](/docs/id/glossary#frontmatter) YAML antara penanda `---` di bagian atas file. `paths` adalah satu-satunya bidang yang Claude Code baca dari aturan; bidang lain diabaikan tanpa kesalahan. Claude Code menghapus frontmatter sebelum memuat aturan ke dalam konteks.

| Bidang  | Diperlukan | Deskripsi                                                                                                                         |
| :------ | :--------- | :-------------------------------------------------------------------------------------------------------------------------------- |
| `paths` | Tidak      | Pola glob yang [membatasi aturan ke file yang cocok](#path-specific-rules). Menerima daftar YAML atau string yang dipisahkan koma |

Jika YAML antara penanda tidak diuraikan, Claude Code mengabaikan frontmatter dan memuat aturan seolah-olah tidak memiliki `paths`. Jalankan `claude --debug` untuk melihat kesalahan penguraian.

<h4 id="share-rules-across-projects-with-symlinks">
  Bagikan aturan di seluruh proyek dengan symlink
</h4>

Direktori `.claude/rules/` mendukung symlink, sehingga Anda dapat mempertahankan set aturan bersama dan menautkannya ke beberapa proyek. Symlink melingkar terdeteksi dan ditangani dengan baik.

Claude Code memperlakukan symlink yang targetnya berada di luar direktori kerja Anda seperti [impor eksternal](#import-additional-files). Aturan yang ditautkan tidak dimuat sampai Anda menyetujui impor eksternal untuk proyek, dan setelah itu hanya yang tanpa [bidang `paths`](#path-specific-rules) yang dimuat. Claude Code meminta persetujuan itu hanya ketika file memori proyek mengimpor file di luar direktori kerja dengan `@path`, bukan untuk symlink saja. Untuk memuat aturan bersama tanpa persetujuan itu, simpan di [`~/.claude/rules/`](#user-level-rules), di mana mereka berlaku untuk setiap proyek di mesin Anda.

Contoh ini menautkan direktori bersama dan file individual:

```bash theme={null}
ln -s ~/shared-claude-rules .claude/rules/shared
ln -s ~/company-standards/security.md .claude/rules/security.md
```

<h4 id="user-level-rules">
  Aturan tingkat pengguna
</h4>

Aturan pribadi di `~/.claude/rules/` berlaku untuk setiap proyek di mesin Anda. Gunakan untuk preferensi yang bukan khusus proyek:

```text theme={null}
~/.claude/rules/
├── preferences.md    # Preferensi pengkodean pribadi Anda
└── workflows.md      # Alur kerja pilihan Anda
```

Claude Code memuat aturan tingkat pengguna sebelum aturan proyek, jadi aturan proyek muncul lebih lambat dalam konteks Claude daripada aturan pengguna. Tidak ada set yang menimpa yang lain: jika aturan pengguna dan aturan proyek bertentangan, Claude mungkin mengikuti salah satu, jadi jaga keduanya konsisten.

<h3 id="manage-claude-md-for-large-teams">
  Kelola CLAUDE.md untuk tim besar
</h3>

Untuk organisasi yang menerapkan Claude Code di seluruh tim, Anda dapat memusatkan instruksi dan mengontrol file CLAUDE.md mana yang dimuat.

<h4 id="deploy-organization-wide-claude-md">
  Terapkan CLAUDE.md di seluruh organisasi
</h4>

Organisasi dapat menerapkan CLAUDE.md yang dikelola secara terpusat yang berlaku untuk semua pengguna di mesin. File ini tidak dapat dikecualikan oleh pengaturan individual.

<Steps>
  <Step title="Buat file di lokasi kebijakan terkelola">
    * macOS: `/Library/Application Support/ClaudeCode/CLAUDE.md`
    * Linux dan WSL: `/etc/claude-code/CLAUDE.md`
    * Windows: `C:\Program Files\ClaudeCode\CLAUDE.md`
  </Step>

  <Step title="Terapkan dengan sistem manajemen konfigurasi Anda">
    Gunakan MDM, Group Policy, Ansible, atau alat serupa untuk mendistribusikan file di seluruh mesin pengembang. Lihat [pengaturan terkelola](/docs/id/managed-settings) untuk opsi konfigurasi di seluruh organisasi lainnya.
  </Step>
</Steps>

Kunci `claudeMd` memungkinkan Anda menempatkan konten CLAUDE.md terkelola langsung di dalam `managed-settings.json` daripada menerapkan file terpisah.

**Cakupan**: setiap sesi Claude Code di mesin, di setiap repositori. Untuk panduan khusus repositori, komit CLAUDE.md proyek sebagai gantinya.

**Prioritas**: sama dengan file CLAUDE.md terkelola. Dimuat sebelum CLAUDE.md pengguna dan proyek.

**Di mana itu dihormati**: pengaturan terkelola dan kebijakan saja. Menetapkan `claudeMd` dalam pengaturan pengguna, proyek, atau lokal tidak berpengaruh.

Contoh di bawah menambahkan instruksi perilaku langsung dalam file pengaturan terkelola:

```json theme={null}
{
  "claudeMd": "Selalu jalankan `make lint` sebelum commit.\nJangan pernah push langsung ke main."
}
```

CLAUDE.md terkelola dan [pengaturan terkelola](/docs/id/managed-settings) melayani tujuan berbeda. Gunakan pengaturan untuk penegakan teknis dan CLAUDE.md untuk panduan perilaku:

| Kekhawatiran                                    | Konfigurasi di                                                |
| :---------------------------------------------- | :------------------------------------------------------------ |
| Blokir alat, perintah, atau jalur file tertentu | Pengaturan terkelola: `permissions.deny`                      |
| Terapkan isolasi sandbox                        | Pengaturan terkelola: `sandbox.enabled`                       |
| Variabel lingkungan dan perutean penyedia API   | Pengaturan terkelola: `env`                                   |
| Metode login dan pembatasan organisasi          | Pengaturan terkelola: `forceLoginMethod`, `forceLoginOrgUUID` |
| Pedoman gaya kode dan kualitas                  | CLAUDE.md terkelola                                           |
| Pengingat penanganan data dan kepatuhan         | CLAUDE.md terkelola                                           |
| Instruksi perilaku untuk Claude                 | CLAUDE.md terkelola                                           |

Aturan pengaturan ditegakkan oleh klien terlepas dari apa yang Claude putuskan untuk dilakukan. Instruksi CLAUDE.md membentuk perilaku Claude tetapi bukan lapisan penegakan keras.

<h4 id="exclude-specific-claude-md-files">
  Kecualikan file CLAUDE.md tertentu
</h4>

Dalam monorepo besar, file CLAUDE.md leluhur mungkin berisi instruksi yang tidak relevan dengan pekerjaan Anda. Pengaturan `claudeMdExcludes` memungkinkan Anda melewati file tertentu berdasarkan jalur atau pola glob.

Contoh ini mengecualikan CLAUDE.md tingkat atas dan direktori aturan dari folder induk. Tambahkan ke `.claude/settings.local.json` sehingga pengecualian tetap lokal ke mesin Anda:

```json theme={null}
{
  "claudeMdExcludes": [
    "**/monorepo/CLAUDE.md",
    "/home/user/monorepo/other-team/.claude/rules/**"
  ]
}
```

Pola dicocokkan dengan jalur file absolut menggunakan sintaks glob. Anda dapat mengonfigurasi `claudeMdExcludes` di [lapisan pengaturan](/docs/id/settings#where-settings-live) apa pun: pengguna, proyek, lokal, atau kebijakan terkelola. Array digabungkan di seluruh lapisan.

Untuk mengecualikan file aturan yang Anda jangkau melalui [symlink](#share-rules-across-projects-with-symlinks), apakah file atau direktorinya adalah tautan, tulis pola terhadap jalur mana pun: jalur file di bawah `.claude/rules/` atau target tautannya. Pola yang cocok dengan jalur mana pun mengecualikan file. Sebelum v2.1.239, hanya pola yang cocok dengan target tautan yang mengecualikan file.

File CLAUDE.md kebijakan terkelola tidak dapat dikecualikan. Ini memastikan instruksi di seluruh organisasi selalu berlaku terlepas dari pengaturan individual.

<h2 id="agents-md">
  AGENTS.md
</h2>

Claude Code dapat membaca [`AGENTS.md`](/docs/id/glossary#agents-md) sebagai instruksi proyek Anda, sehingga repositori yang sudah diatur untuk agen pengkodean lain berfungsi tanpa menambahkan `CLAUDE.md`, impor, atau pengaturan. Tabel ini menunjukkan apa yang Claude baca secara default untuk setiap kombinasi file instruksi di repositori Anda:

| Repositori Anda memiliki                                                                              | Claude membaca                                                |
| :---------------------------------------------------------------------------------------------------- | :------------------------------------------------------------ |
| `AGENTS.md`, dan tidak ada `CLAUDE.md` atau `CLAUDE.local.md` di direktori kerja Anda atau di atasnya | `AGENTS.md` Anda                                              |
| `AGENTS.md` dan `CLAUDE.md` atau `CLAUDE.local.md` di direktori kerja Anda atau di atasnya            | File `CLAUDE.md` Anda saja                                    |
| `CLAUDE.md` yang sudah [mengimpor `AGENTS.md`](#share-one-file-with-other-coding-tools)               | `CLAUDE.md` Anda, dengan `AGENTS.md` disertakan melalui impor |

Untuk mengubah default, misalnya untuk membuat Claude selalu membaca kedua file, membaca hanya `CLAUDE.md`, atau membaca hanya instruksi yang dikelola organisasi Anda, [ubah pengaturan **Project instructions**](#choose-which-instruction-files-load).

<Note>
  Membaca `AGENTS.md` secara langsung memerlukan Claude Code v2.1.277 atau lebih baru. Dalam beberapa sesi Claude [tidak dapat membaca `AGENTS.md`](#when-agents-md-support-is-unavailable), jadi [impor dari `CLAUDE.md`](#share-one-file-with-other-coding-tools) di sana sebagai gantinya.
</Note>

<h3 id="when-claude-code-reads-agents-md">
  When Claude Code reads AGENTS.md
</h3>

Secara default, Claude membaca `AGENTS.md` hanya ketika Anda tidak memiliki `CLAUDE.md` di direktori kerja Anda atau di atasnya. Berikut adalah file Anda mana yang dihitung untuk pemeriksaan itu:

* **Dihitung, sehingga Claude membacanya alih-alih `AGENTS.md`**: `CLAUDE.md`, `.claude/CLAUDE.md`, atau `CLAUDE.local.md` di direktori kerja Anda atau direktori apa pun di atasnya
* **Tidak dihitung, dan terus dimuat bersama `AGENTS.md`**: `~/.claude/CLAUDE.md` Anda, `CLAUDE.md` yang dikelola organisasi Anda, dan file `.claude/rules/`

Ketika tidak ada yang dihitung, berikut adalah apa yang Claude baca dan bagaimana Anda dapat mengetahuinya:

* **Pada awal sesi**: setiap `AGENTS.md` dan `.claude/AGENTS.md` di direktori kerja Anda dan direktori di atasnya. Dalam sesi interaktif Anda melihat baris seperti `no CLAUDE.md found; AGENTS.md loaded: /home/you/repo/AGENTS.md` dalam percakapan
* **Saat Claude bekerja di subdirektori**: `AGENTS.md` subdirektori, ketika Claude membuka file di sana dengan alat Read dan subdirektori itu tidak memiliki salah satu dari tiga file `CLAUDE.md` miliknya sendiri
* **Di dalam setiap `AGENTS.md`**: impor [`@path`](#import-additional-files) diperluas, pola [`claudeMdExcludes`](#exclude-specific-claude-md-files) berlaku, dan subagen yang [melewati instruksi proyek](/docs/id/sub-agents#what-loads-at-startup) melewati file-file ini juga
* **Tidak dibaca**: `AGENTS.local.md`, `AGENTS.override.md`, atau apa pun di bawah direktori `.agents/`

<Note>
  Karena `CLAUDE.local.md` dihitung, menambahkan satu untuk menyimpan instruksi pribadi Anda yang tidak berkomitmen dalam proyek yang bergantung pada `AGENTS.md` menghentikan Claude dari membaca `AGENTS.md` untuk Anda. Untuk menyimpan `CLAUDE.local.md` Anda dan tetap membuat Claude membaca `AGENTS.md`, atur **Project instructions** ke [`claude-md-and-agents-md`](#choose-which-instruction-files-load).
</Note>

<h3 id="choose-which-instruction-files-load">
  Choose which instruction files load
</h3>

Untuk mengubah file mana yang Claude baca, ketik `/config` dalam sesi Claude Code untuk membuka panel pengaturan, kemudian atur **Project instructions** ke salah satu nilai berikut:

| Value                     | What Claude reads                                                                                                                                                                                                                                                                                                                                                       |
| :------------------------ | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `claude-md-or-agents-md`  | File `CLAUDE.md` Anda, atau file `AGENTS.md` Anda ketika Anda tidak memiliki `CLAUDE.md` atau `CLAUDE.local.md` di direktori kerja Anda atau di atasnya. Ini adalah default                                                                                                                                                                                             |
| `claude-md-and-agents-md` | File `CLAUDE.md` dan `AGENTS.md` Anda bersama-sama, file `CLAUDE.md` setiap direktori terlebih dahulu dan `AGENTS.md` nya setelahnya. Claude Code melewati `AGENTS.md` yang sudah dimuat, sehingga yang diimpor atau disimlink oleh `CLAUDE.md` Anda tidak dibaca dua kali                                                                                              |
| `claude-md`               | File `CLAUDE.md` Anda saja                                                                                                                                                                                                                                                                                                                                              |
| `managed-only`            | Hanya `CLAUDE.md` yang dikelola organisasi Anda dan [auto memory](#auto-memory) saat peluncuran. File `CLAUDE.md` proyek, lokal, dan pengguna Anda, file `.claude/rules/` Anda, dan setiap `AGENTS.md` dikecualikan. File `CLAUDE.md` dan `.claude/rules/` subdirektori, dan [path-scoped rules](#path-specific-rules), tetap dimuat ketika Claude membaca file di sana |

Anda juga dapat mengatur nilai dalam file pengaturan alih-alih `/config`. Tambahkan di bawah ID plugin `agents-md` bawaan dalam [`pluginConfigs`](/docs/id/settings-reference#pluginconfigs), di `~/.claude/settings.json`, file `--settings`, atau [managed settings](/docs/id/managed-settings). Claude Code mengabaikannya dalam file pengaturan proyek dan lokal. Contoh ini membuat Claude membaca kedua file:

```json settings.json theme={null}
{
  "pluginConfigs": {
    "agents-md@builtin": {
      "options": { "instructionFiles": "claude-md-and-agents-md" }
    }
  }
}
```

Perubahan Anda berlaku dari pesan berikutnya yang Anda kirim dan di setiap sesi baru.

<h3 id="when-agents-md-support-is-unavailable">
  When AGENTS.md support is unavailable
</h3>

Dalam sesi ini Claude membaca file `CLAUDE.md` saja, dan **Project instructions** tidak muncul di panel pengaturan `/config`:

* Anda menggunakan versi Claude Code sebelum v2.1.277
* Anda menonaktifkan plugin `agents-md` bawaan di `/plugin`
* Dalam beberapa kasus, ini adalah [sesi pertama Anda setelah Anda meningkatkan](/docs/id/env-vars#first-session-after-an-install-or-upgrade) dari v2.1.276 atau lebih awal. Claude membaca `AGENTS.md` dari sesi berikutnya Anda

Sebelum v2.1.281, beberapa sesi, seperti yang ada di Amazon Bedrock atau dengan telemetri dinonaktifkan, membaca file `CLAUDE.md` saja. Pada versi tersebut, perbarui Claude Code. Untuk memberikan Claude `AGENTS.md` Anda dalam sesi apa pun dari ini, [impor dari `CLAUDE.md`](#share-one-file-with-other-coding-tools).

<h3 id="where-agents-md-differs-from-claude-md">
  Where AGENTS.md differs from CLAUDE.md
</h3>

`AGENTS.md` yang Claude baca melalui pengaturan **Project instructions** berbeda dari `CLAUDE.md` di tempat-tempat ini:

|                                                                                                                                                  | `CLAUDE.md`                                                                      | `AGENTS.md` dibaca melalui pengaturan                                                                  |
| :----------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------- |
| Hook [`InstructionsLoaded`](/docs/id/hooks#instructionsloaded)                                                                                        | Aktif                                                                            | Tidak aktif. Mereka aktif seperti biasa untuk `AGENTS.md` yang diimpor atau disimlink oleh `CLAUDE.md` |
| Direktori yang Anda tambahkan dengan `--add-dir` saat [`CLAUDE_CODE_ADDITIONAL_DIRECTORIES_CLAUDE_MD`](#load-from-additional-directories) diatur | `CLAUDE.md` mereka dimuat                                                        | `AGENTS.md` mereka tidak dimuat                                                                        |
| Impor `@path` dari file di luar direktori kerja Anda                                                                                             | Claude Code meminta Anda menyetujui [external imports](#import-additional-files) | Dimuat hanya jika Anda sudah menyetujui impor eksternal untuk proyek ini, tanpa prompt                 |

<h3 id="remove-an-earlier-agents-md-workaround">
  Remove an earlier AGENTS.md workaround
</h3>

Jika Anda menyiapkan Claude Code untuk membaca `AGENTS.md` sebelum melakukannya sendiri, berikut adalah apa yang harus dilakukan dengan setiap pengaturan umum:

* **`CLAUDE.md` yang berisi `@AGENTS.md`**: Anda dapat meninggalkannya. Menyimpan impor tidak pernah membuat Claude membaca `AGENTS.md` dua kali, nilai **Project instructions** apa pun yang Anda gunakan. Hapus `CLAUDE.md` jika tidak menyimpan apa pun yang lain, atau simpan jika beberapa sesi Anda [tidak dapat memuat `AGENTS.md` secara langsung](#when-agents-md-support-is-unavailable).
* **`CLAUDE.md` yang memberi tahu Claude dalam kata-kata untuk membaca `AGENTS.md`**: Claude melihat `AGENTS.md` hanya jika memutuskan untuk membuka file. Hapus `CLAUDE.md` sehingga Claude membaca `AGENTS.md` secara langsung, atau ganti kalimat dengan impor `@AGENTS.md`.
* **`CLAUDE.md` yang symlink ke `AGENTS.md`**: tidak ada, atau hapus symlink. Bagaimanapun Claude membaca konten sekali.
* **Hook `SessionStart` yang mencetak `AGENTS.md`**: hapus itu. Setelah Claude membaca `AGENTS.md` secara langsung, hook menambahkan salinan kedua ke konteks.

<h3 id="share-one-file-with-other-coding-tools">
  Share one file with other coding tools
</h3>

Ketika Claude tidak membaca `AGENTS.md` Anda secara langsung, Anda masih dapat menyimpannya sebagai satu file yang dibagikan setiap alat dengan menempatkan impor `@AGENTS.md` dalam `CLAUDE.md` di sebelahnya. Lakukan ini ketika proyek Anda juga memiliki `CLAUDE.md`, ketika Anda telah mengatur **Project instructions** ke `claude-md`, atau dalam sesi yang [tidak dapat memuat `AGENTS.md`](#when-agents-md-support-is-unavailable). Tambahkan instruksi khusus Claude apa pun di bawah impor, dan Claude membaca file yang diimpor terlebih dahulu, kemudian sisanya:

```markdown CLAUDE.md theme={null}
@AGENTS.md

## Claude Code

Use plan mode for changes under `src/billing/`.
```

Jika Anda tidak memerlukan konten khusus Claude, symlink juga berfungsi:

```bash theme={null}
ln -s AGENTS.md CLAUDE.md
```

Perintah tidak mencetak output pada kesuksesan. Sebelum Anda memilih symlink daripada impor, periksa batasan ini:

* **Pengeditan**: Claude membaca `CLAUDE.md` melalui tautan, tetapi alat Edit dan Write [menolak untuk menulis melalui symlink](/docs/id/errors#refusing-after-a-symlink-changed), dan penolakan mengarahkan Claude untuk mengedit target tautan, `AGENTS.md`, sebagai gantinya
* **Windows**: jika Anda atau siapa pun yang mengkloning repositori bekerja di Windows, gunakan impor `@AGENTS.md` sebagai gantinya. Membuat symlink di sana memerlukan hak istimewa Administrator atau Mode Pengembang, dan Git memeriksa symlink yang berkomitmen sebagai file teks biasa kecuali `core.symlinks` diaktifkan, yang meninggalkan klon itu dengan `CLAUDE.md` satu baris sebagai pengganti instruksi Anda

Dengan salah satu pendekatan, jalankan `/context` dalam sesi berikutnya Anda dan konfirmasi `CLAUDE.md` muncul di bawah **Memory files**.

<h3 id="migrate-instructions-from-other-tools">
  Migrate instructions from other tools
</h3>

Menjalankan [`/init`](/docs/id/commands) membaca file instruksi alat lain dan menggabungkan bagian yang relevan ke dalam `CLAUDE.md` yang dihasilkan:

* Aturan Cursor di `.cursor/rules/` atau `.cursorrules`
* Aturan Copilot di `.github/copilot-instructions.md`
* Dengan `CLAUDE_CODE_NEW_INIT=1` diatur: `AGENTS.md`, `.devin/rules/`, `.windsurf/rules/` atau `.windsurfrules`, dan `.clinerules`

Anda juga dapat menjalankan [`/import`](/docs/id/commands) untuk membawa konfigurasi agen pengkodean yang didukung ke Claude Code, yang menambahkan salinan satu kali file instruksi seperti `AGENTS.md` ke `CLAUDE.md` yang cocok dan membawa server MCP, perintah, subagen, dan skills. Memerlukan Claude Code v2.1.213 atau lebih baru.

<h2 id="auto-memory">
  Auto memory
</h2>

Auto memory memungkinkan Claude mengumpulkan pengetahuan lintas sesi tanpa Anda menulis apa pun. Saat bekerja, Claude menyimpan empat jenis catatan untuk dirinya sendiri. Claude mencatat jenisnya sebagai bidang `type` dalam frontmatter file memori:

* `user`: peran Anda, keahlian, dan preferensi kerja
* `feedback`: koreksi yang Anda berikan kepada Claude dan pendekatan yang Anda konfirmasi
* `project`: pekerjaan yang sedang berlangsung, tenggat waktu, dan keputusan yang tidak dapat Claude turunkan dari kode atau riwayat git
* `reference`: tempat menemukan informasi di luar proyek, seperti pelacak masalah atau dasbor

Claude melewati apa pun yang dapat diturunkan dari basis kode, seperti arsitektur, jalur file, atau perbaikan debugging. Itu juga melewati apa pun yang sudah dikatakan file CLAUDE.md Anda.

Claude tidak menyimpan sesuatu setiap sesi. Itu memutuskan apa yang layak diingat berdasarkan apakah informasi akan berguna dalam percakapan masa depan.

<h3 id="enable-or-disable-auto-memory">
  Aktifkan atau nonaktifkan auto memory
</h3>

Auto memory aktif secara default. Untuk mengalihkannya, buka `/memory` dalam sesi dan gunakan toggle auto memory, yang menyimpan `autoMemoryEnabled` ke pengaturan pengguna Anda di `~/.claude/settings.json`. Untuk mematikannya untuk satu proyek, atur `autoMemoryEnabled` dalam pengaturan proyek itu:

```json theme={null}
{
  "autoMemoryEnabled": false
}
```

Untuk menonaktifkan auto memory melalui variabel lingkungan, atur `CLAUDE_CODE_DISABLE_AUTO_MEMORY=1`.

<h3 id="storage-location">
  Lokasi penyimpanan
</h3>

Setiap proyek mendapatkan direktori memori sendiri di `~/.claude/projects/<project>/memory/`. Jalur `<project>` berasal dari repositori git, jadi semua worktrees dan subdirektori dalam repo yang sama berbagi satu direktori auto memory. Di luar repo git, root proyek digunakan sebagai gantinya.

Jika Anda menetapkan [`CLAUDE_CODE_PROJECT_DIR_NAME`](/docs/id/sessions#name-the-project-directory-yourself) di samping `CLAUDE_CONFIG_DIR`, Claude Code menggunakan nama itu sebagai direktori `<project>` di bawah `<config dir>/projects/` sebagai gantinya, di mana pun repositori apa pun yang Anda luncurkan, jadi proyek yang diluncurkan dengan direktori konfigurasi itu berbagi satu direktori auto memory. Memerlukan Claude Code v2.1.234 atau lebih baru.

Untuk menyimpan auto memory di lokasi berbeda, atur `autoMemoryDirectory` dalam `settings.json` Anda. Itu dibaca dari [cakupan pengaturan](/docs/id/settings#settings-precedence) apa pun: pengguna, proyek, lokal, kebijakan, atau `--settings`.

```json theme={null}
{
  "autoMemoryDirectory": "~/my-custom-memory-dir"
}
```

Nilai harus berupa jalur absolut atau dimulai dengan `~/`.

Ketika Anda menetapkannya dalam `.claude/settings.json` atau `.claude/settings.local.json` proyek, Claude Code menghormatinya di bawah [aturan kepercayaan ruang kerja yang sama dengan hooks dalam file pengaturan](/docs/id/permissions#what-runs-before-you-trust-a-folder). Sementara [`permissions.blockReadsOutsideWorkingDirectories`](/docs/id/settings-reference#permissions-blockreadsoutsideworkingdirectories) aktif, Claude Code tidak memuat auto memory dari direktori yang dipilih oleh [file pengaturan yang disediakan repositori](/docs/id/permissions#when-your-local-settings-file-needs-trust) dan tidak menyimpan apa pun ke dalamnya, di mana pun direktori itu berada.

Direktori berisi indeks `MEMORY.md` dan satu file topik per memori:

```text theme={null}
~/.claude/projects/<project>/memory/
├── MEMORY.md           # Indeks, satu baris per memori, dimuat ke dalam setiap sesi
├── user_role.md        # Satu memori
├── feedback_testing.md # Satu memori
└── ...                 # File topik lainnya yang Claude buat
```

`MEMORY.md` bertindak sebagai indeks direktori memori. Claude membaca dan menulis file di direktori ini sepanjang sesi Anda, menggunakan `MEMORY.md` untuk melacak apa yang disimpan di mana.

Auto memory adalah mesin-lokal. Semua worktrees dan subdirektori dalam repositori git yang sama berbagi satu direktori auto memory. File tidak dibagikan di seluruh mesin atau lingkungan cloud.

Claude Code menghapus transkrip sesi lama setelah periode retensi [`cleanupPeriodDays`](/docs/id/settings-reference#cleanupperioddays), tetapi mengecualikan file memori di direktori memori dari [pembersihan retensi](/docs/id/claude-directory#cleaned-up-automatically) itu. `MEMORY.md` dan file topik tetap ada sampai Anda atau Claude mengedit atau menghapusnya.

<h3 id="how-it-works">
  Bagaimana cara kerjanya
</h3>

200 baris pertama `MEMORY.md`, atau 25KB pertama, mana pun yang lebih dulu, dimuat di awal setiap percakapan. Konten di luar batas itu tidak dimuat saat awal sesi. Claude membuat `MEMORY.md` ringkas dengan memindahkan catatan terperinci ke file topik terpisah.

Setelah Claude menulis ke `MEMORY.md`, Claude Code mengukur file terhadap batas baca 200 baris dan 25KB. Jika file mendekati batas, Claude Code mengingatkan Claude untuk mempersingkatnya: simpan satu baris per entri, pindahkan detail ke file topik, dan gabungkan atau lepaskan entri yang sudah usang. Jika file melampaui batas, penulisan masih berhasil, tetapi Claude Code mengembalikan [kesalahan yang memberi tahu Claude untuk menulis ulang indeks](/docs/id/errors#memory-index-is-over-its-read-limit), karena semua yang melampaui batas dijatuhkan pada beban berikutnya.

Batas ini hanya berlaku untuk `MEMORY.md`. Claude Code memuat file CLAUDE.md hingga 4 MiB sepenuhnya dan melewati file yang lebih besar. File yang lebih pendek menghasilkan kepatuhan yang lebih baik.

Claude Code tidak memuat file topik seperti `user_role.md` atau `feedback_testing.md` saat startup. Claude membacanya sesuai permintaan menggunakan alat file standarnya ketika membutuhkan informasi.

Auto memory percakapan utama tidak dimuat ke dalam [subagents](/docs/id/sub-agents#what-loads-at-startup); pengecualiannya adalah [fork](/docs/id/sub-agents#fork-the-current-conversation), yang mewarisi percakapan induk dan prompt sistem. Auto memory subagent sendiri, diaktifkan dengan bidang `memory` subagent, adalah direktori terpisah.

Claude membaca dan menulis file memori selama sesi Anda. Ketika Anda melihat pesan seperti "Saved 2 memories" atau "Recalled 2 memories" di antarmuka Claude Code, Claude secara aktif memperbarui atau membaca dari `~/.claude/projects/<project>/memory/`.

Ketika Claude menulis file memori yang dimulai dengan frontmatter YAML, Claude Code mencatat waktu penulisan dalam bidang frontmatter `modified` sebagai stempel waktu ISO 8601. Stempel waktu menunjukkan seberapa terkini faktanya, baik untuk Anda maupun untuk Claude ketika membacanya kembali. File apa pun yang memiliki frontmatter mendapatkan bidang saat Claude menulis ulang, termasuk file yang dibuat di versi sebelumnya; Claude Code tidak pernah menambahkan frontmatter ke file yang tidak memilikinya. Bidang `modified` memerlukan Claude Code v2.1.214 atau lebih baru.

<h3 id="audit-and-edit-your-memory">
  Audit dan edit memori Anda
</h3>

File auto memory adalah markdown biasa yang dapat Anda edit atau hapus kapan saja. Jalankan [`/memory`](#view-and-edit-with-%2Fmemory) untuk menelusuri dan membuka file memori dari dalam sesi.

<h2 id="view-and-edit-with-/memory">
  Lihat dan edit dengan `/memory`
</h2>

Perintah `/memory` mencantumkan file CLAUDE.md, CLAUDE.local.md, dan file memory lainnya di seluruh cakupan pengguna dan proyek, termasuk entri CLAUDE.md pengguna dan proyek untuk file yang belum ada. Ini juga memungkinkan Anda mengalihkan auto memory aktif atau mati dan menyediakan opsi untuk membuka folder auto memory. Pilih file apa pun untuk membukanya di editor Anda; memilih file yang belum ada akan membuatnya terlebih dahulu. Untuk memeriksa file mana yang benar-benar dimuat ke dalam sesi saat ini, jalankan `/context`.

Editor GUI seperti VS Code membuka file di jendela terpisah, dan Anda dapat terus menggunakan sesi saat file terbuka. Sebelum v2.1.216, `/memory` menunggu Anda menutup file sebelum merespons. Editor terminal seperti Vim mengambil alih terminal sampai Anda keluar.

Ketika Anda meminta Claude untuk mengingat sesuatu, seperti "selalu gunakan pnpm, bukan npm" atau "ingat bahwa tes API memerlukan instans Redis lokal," Claude menyimpannya ke auto memory. Untuk menambahkan instruksi ke CLAUDE.md sebagai gantinya, minta Claude secara langsung, seperti "tambahkan ini ke CLAUDE.md," atau edit file sendiri melalui `/memory`.

<h2 id="troubleshoot-memory-issues">
  Troubleshoot memory issues
</h2>

Ini adalah masalah paling umum dengan CLAUDE.md dan auto memory, bersama dengan langkah-langkah untuk men-debug mereka.

<h3 id="claude-isn’t-following-my-claude-md">
  Claude isn't following my CLAUDE.md
</h3>

Konten CLAUDE.md disampaikan sebagai pesan pengguna setelah prompt sistem, bukan sebagai bagian dari prompt sistem itu sendiri. Claude membacanya dan mencoba mengikutinya, tetapi tidak ada jaminan kepatuhan ketat, terutama untuk instruksi yang samar atau bertentangan.

Untuk men-debug:

* Jalankan `/context` dan periksa daftar di bawah **Memory files** untuk memverifikasi file CLAUDE.md dan CLAUDE.local.md Anda dimuat. Jika file `CLAUDE.md` tidak ada di sana, Claude tidak dapat melihatnya. Gunakan `/memory` untuk membuka dan mengedit file.
* Periksa bahwa CLAUDE.md yang relevan berada di lokasi yang dimuat untuk sesi Anda (lihat [Choose where to put CLAUDE.md files](#choose-where-to-put-claude-md-files)).
* Buat instruksi lebih spesifik. "Gunakan indentasi 2 spasi" bekerja lebih baik daripada "format kode dengan baik."
* Cari instruksi yang bertentangan di seluruh file CLAUDE.md. Jika dua file memberikan panduan berbeda untuk perilaku yang sama, Claude mungkin memilih satu secara sembarangan.

Jika instruksi adalah sesuatu yang harus berjalan pada titik tertentu, seperti sebelum setiap commit atau setelah setiap pengeditan file, tulislah sebagai [hook](/docs/id/hooks-guide) sebagai gantinya. Hooks dieksekusi sebagai perintah shell pada peristiwa siklus hidup tetap dan berlaku terlepas dari apa yang Claude putuskan untuk lakukan.

Untuk instruksi yang Anda inginkan di tingkat prompt sistem, gunakan [`--append-system-prompt`](/docs/id/cli-reference#system-prompt-flags). Anda meneruskannya saat peluncuran, jadi lebih cocok untuk skrip dan otomasi daripada penggunaan interaktif. Untuk cara kerjanya saat Anda melanjutkan percakapan, lihat [System prompt flags in resumed conversations](/docs/id/cli-reference#system-prompt-flags-in-resumed-conversations).

<Tip>
  Gunakan [hook `InstructionsLoaded`](/docs/id/hooks#instructionsloaded) untuk mencatat dengan tepat file instruksi mana yang dimuat, kapan mereka dimuat, dan mengapa. Ini berguna untuk men-debug aturan khusus jalur atau file yang dimuat malas di subdirektori.
</Tip>

<h3 id="my-agents-md-isn’t-loading">
  My AGENTS.md isn't loading
</h3>

Jika repositori Anda memiliki `AGENTS.md` dan Claude tampaknya tidak tahu apa yang dikatakannya, penyebab biasanya adalah `CLAUDE.md` di suatu tempat di jalur proyek. Secara default Claude membaca `AGENTS.md` hanya ketika Anda tidak memiliki `CLAUDE.md` atau `CLAUDE.local.md` di direktori kerja Anda atau di atasnya. Periksa ini secara berurutan:

1. Cari `CLAUDE.md`, `.claude/CLAUDE.md`, atau `CLAUDE.local.md` di direktori kerja Anda atau direktori apa pun di atasnya, selain `~/.claude/CLAUDE.md`. Jika Anda menemukan satu, Claude membacanya sebagai gantinya dari `AGENTS.md` kecuali Anda menetapkan **Project instructions** ke `claude-md-and-agents-md`.
2. Jalankan `claude --version` dan konfirmasi v2.1.277 atau lebih baru. Sebelum v2.1.281, beberapa sesi, seperti sesi di Amazon Bedrock atau dengan telemetri dinonaktifkan, [tidak dapat memuat `AGENTS.md`](#when-agents-md-support-is-unavailable) juga, jadi pada versi tersebut perbarui ke v2.1.281 atau lebih baru.
3. Ketik `/config` dalam sesi Anda untuk membuka panel pengaturan dan konfirmasi **Project instructions** tidak diatur ke `claude-md` atau `managed-only`. Jika Anda tidak melihat pengaturan di sana sama sekali, sesi Anda adalah sesi yang [tidak dapat memuat `AGENTS.md`](#when-agents-md-support-is-unavailable).

Untuk memeriksa apakah Claude membaca `AGENTS.md` Anda, jalankan `/memory` dan cari jalurnya dalam daftar.

Sebelum v2.1.280, `/memory` dan `/context` tidak mencantumkan `AGENTS.md` yang Claude baca secara langsung. Pada versi tersebut, tanyakan kepada Claude apa yang dikatakan instruksi proyeknya sebagai gantinya.

Jika Anda ingin menyimpan `CLAUDE.md` yang Anda temukan, atau sesi Anda tidak dapat memuat `AGENTS.md`, [tambahkan `CLAUDE.md` di sebelah `AGENTS.md` Anda yang mengimpornya](#share-one-file-with-other-coding-tools).

<h3 id="i-don’t-know-what-auto-memory-saved">
  I don't know what auto memory saved
</h3>

Jalankan `/memory` dan pilih folder auto memory untuk menelusuri apa yang telah disimpan Claude. Semuanya adalah markdown biasa yang dapat Anda baca, edit, atau hapus.

<h3 id="my-claude-md-is-too-large">
  My CLAUDE.md is too large
</h3>

File di atas 200 baris mengonsumsi lebih banyak konteks dan dapat mengurangi kepatuhan. Claude Code melewati file di atas 4 MiB. Gunakan [path-scoped rules](#path-specific-rules) untuk memuat instruksi hanya ketika Claude bekerja dengan file yang cocok, atau pangkas konten yang tidak diperlukan dalam setiap sesi. Memisahkan ke dalam [impor `@path`](#import-additional-files) membantu organisasi tetapi tidak mengurangi konteks, karena file yang diimpor dimuat saat peluncuran.

[`/doctor`](/docs/id/commands#all-commands) checkup mengusulkan pemangkasan untuk CLAUDE.md yang diperiksa: ia memotong konten yang dapat Claude turunkan dari codebase, seperti tata letak direktori, daftar dependensi, dan ikhtisar arsitektur, dan menyimpan jebakan, rasional, dan konvensi yang berbeda dari default alat. Pemeriksaan pemangkasan memerlukan Claude Code v2.1.206 atau lebih baru.

<h3 id="instructions-seem-lost-after-/compact">
  Instructions seem lost after `/compact`
</h3>

CLAUDE.md root proyek bertahan dari pemadatan: setelah `/compact`, Claude membaca ulang dari disk dan menyuntikkannya kembali ke dalam sesi. File CLAUDE.md bersarang di subdirektori dan aturan dengan [frontmatter `paths:`](#path-specific-rules) dimuat ulang saat Claude membaca file yang mereka terapkan.

Jika instruksi hilang setelah pemadatan, itu diberikan hanya dalam percakapan, berada di CLAUDE.md bersarang yang belum dimuat ulang, atau merupakan aturan yang dibatasi jalur yang belum cocok dengan file sejak itu. Tambahkan instruksi percakapan ke CLAUDE.md untuk membuatnya bertahan. Lihat [What survives compaction](/docs/id/context-window#what-survives-compaction) untuk rincian lengkap.

Lihat [Write effective instructions](#write-effective-instructions) untuk panduan tentang ukuran, struktur, dan spesifisitas.

<h2 id="related-resources">
  Sumber daya terkait
</h2>

* [Debug konfigurasi Anda](/docs/id/debug-your-config): diagnosis mengapa CLAUDE.md atau pengaturan tidak berlaku
* [Skills](/docs/id/skills): paket alur kerja yang dapat diulang yang dimuat sesuai permintaan
* [Settings](/docs/id/settings): konfigurasi perilaku Claude Code dengan file pengaturan
* [Memori subagent](/docs/id/sub-agents#enable-persistent-memory): biarkan subagents mempertahankan auto memory mereka sendiri
