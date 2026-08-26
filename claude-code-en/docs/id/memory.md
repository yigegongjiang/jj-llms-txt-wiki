> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Bagaimana Claude mengingat proyek Anda

> Berikan Claude instruksi persisten dengan file CLAUDE.md, dan biarkan Claude mengumpulkan pembelajaran secara otomatis dengan auto memory.

Setiap sesi Claude Code dimulai dengan context window yang segar. Dua mekanisme membawa pengetahuan lintas sesi:

* **File CLAUDE.md**: instruksi yang Anda tulis untuk memberikan Claude konteks persisten
* **Auto memory**: catatan yang Claude tulis sendiri berdasarkan koreksi dan preferensi Anda

Halaman ini mencakup cara untuk:

* [Menulis dan mengorganisir file CLAUDE.md](#claude-md-files)
* [Membatasi aturan ke tipe file tertentu](#organize-rules-with-claude/rules/) dengan `.claude/rules/`
* [Mengonfigurasi auto memory](#auto-memory) agar Claude membuat catatan secara otomatis
* [Troubleshoot](#troubleshoot-memory-issues) ketika instruksi tidak diikuti

<h2 id="claude-md-vs-auto-memory">
  CLAUDE.md vs auto memory
</h2>

Claude Code memiliki dua sistem memori yang saling melengkapi. Keduanya dimuat di awal setiap percakapan. Claude memperlakukan mereka sebagai konteks, bukan konfigurasi yang diberlakukan. Untuk memblokir suatu tindakan terlepas dari apa yang Claude putuskan, gunakan [hook PreToolUse](/docs/id/hooks-guide) sebagai gantinya. Semakin spesifik dan ringkas instruksi Anda, semakin konsisten Claude mengikutinya.

|                           | File CLAUDE.md                                    | Auto memory                                                       |
| :------------------------ | :------------------------------------------------ | :---------------------------------------------------------------- |
| **Siapa yang menulisnya** | Anda                                              | Claude                                                            |
| **Apa yang dikandungnya** | Instruksi dan aturan                              | Pembelajaran dan pola                                             |
| **Cakupan**               | Proyek, pengguna, atau organisasi                 | Per repositori, dibagikan di seluruh worktrees                    |
| **Dimuat ke dalam**       | Setiap sesi                                       | Setiap sesi (200 baris pertama atau 25KB)                         |
| **Gunakan untuk**         | Standar pengkodean, alur kerja, arsitektur proyek | Perintah build, wawasan debugging, preferensi yang Claude temukan |

Gunakan file CLAUDE.md ketika Anda ingin memandu perilaku Claude. Auto memory memungkinkan Claude belajar dari koreksi Anda tanpa usaha manual.

Subagents juga dapat mempertahankan auto memory mereka sendiri. Lihat [konfigurasi subagent](/docs/id/sub-agents#enable-persistent-memory) untuk detail.

<h2 id="claude-md-files">
  File CLAUDE.md
</h2>

File CLAUDE.md adalah file markdown yang memberikan Claude instruksi persisten untuk proyek, alur kerja pribadi Anda, atau seluruh organisasi Anda. Anda menulis file ini dalam teks biasa; Claude membacanya di awal setiap sesi.

<h3 id="when-to-add-to-claude-md">
  Kapan harus menambahkan ke CLAUDE.md
</h3>

Perlakukan CLAUDE.md sebagai tempat Anda menulis apa yang sebaliknya akan Anda jelaskan kembali. Tambahkan ke dalamnya ketika:

* Claude membuat kesalahan yang sama untuk kedua kalinya
* Tinjauan kode menangkap sesuatu yang seharusnya Claude ketahui tentang basis kode ini
* Anda mengetik koreksi atau klarifikasi yang sama ke dalam chat yang Anda ketik di sesi terakhir
* Anggota tim baru akan membutuhkan konteks yang sama untuk produktif

Pertahankan fakta yang seharusnya Claude pegang di setiap sesi: perintah build, konvensi, tata letak proyek, aturan "selalu lakukan X". Jika entri adalah prosedur multi-langkah atau hanya penting untuk satu bagian dari basis kode, pindahkan ke [skill](/docs/id/skills) atau [aturan bersyarat jalur](#organize-rules-with-claude/rules/) sebagai gantinya. [Gambaran umum ekstensi](/docs/id/features-overview#build-your-setup-over-time) mencakup kapan menggunakan setiap mekanisme.

<h3 id="choose-where-to-put-claude-md-files">
  Pilih di mana menempatkan file CLAUDE.md
</h3>

File CLAUDE.md dapat berada di beberapa lokasi, masing-masing dengan cakupan yang berbeda. Tabel di bawah mencantumkan mereka dalam urutan pemuatan, dari cakupan terluas hingga paling spesifik, jadi instruksi proyek muncul dalam konteks setelah instruksi pengguna.

| Cakupan                 | Lokasi                                                                                                                                                                  | Tujuan                                                       | Contoh kasus penggunaan                                                  | Dibagikan dengan                   |
| ----------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------ | ------------------------------------------------------------------------ | ---------------------------------- |
| **Kebijakan terkelola** | • macOS: `/Library/Application Support/ClaudeCode/CLAUDE.md`<br />• Linux dan WSL: `/etc/claude-code/CLAUDE.md`<br />• Windows: `C:\Program Files\ClaudeCode\CLAUDE.md` | Instruksi di seluruh organisasi yang dikelola oleh IT/DevOps | Standar pengkodean perusahaan, kebijakan keamanan, persyaratan kepatuhan | Semua pengguna dalam organisasi    |
| **Instruksi pengguna**  | `~/.claude/CLAUDE.md`                                                                                                                                                   | Preferensi pribadi untuk semua proyek                        | Preferensi gaya kode, pintasan alat pribadi                              | Hanya Anda (semua proyek)          |
| **Instruksi proyek**    | `./CLAUDE.md` atau `./.claude/CLAUDE.md`                                                                                                                                | Instruksi bersama tim untuk proyek                           | Arsitektur proyek, standar pengkodean, alur kerja umum                   | Anggota tim melalui kontrol sumber |
| **Instruksi lokal**     | `./CLAUDE.local.md`                                                                                                                                                     | Preferensi pribadi khusus proyek; tambahkan ke `.gitignore`  | URL sandbox Anda, data test pilihan                                      | Hanya Anda (proyek saat ini)       |

File CLAUDE.md dan CLAUDE.local.md dalam hierarki direktori di atas direktori kerja dimuat sepenuhnya saat peluncuran. File di subdirektori dimuat sesuai permintaan ketika Claude membaca file di direktori tersebut. Lihat [Bagaimana file CLAUDE.md dimuat](#how-claude-md-files-load) untuk urutan resolusi lengkap.

Untuk proyek besar, Anda dapat memecah instruksi menjadi file khusus topik menggunakan [aturan proyek](#organize-rules-with-claude/rules/). Aturan memungkinkan Anda membatasi instruksi ke tipe file atau subdirektori tertentu.

<h3 id="set-up-a-project-claude-md">
  Siapkan CLAUDE.md proyek
</h3>

CLAUDE.md proyek dapat disimpan di `./CLAUDE.md` atau `./.claude/CLAUDE.md`. Buat file ini dan tambahkan instruksi yang berlaku untuk siapa pun yang bekerja pada proyek: perintah build dan test, standar pengkodean, keputusan arsitektur, konvensi penamaan, dan alur kerja umum. Instruksi ini dibagikan dengan tim Anda melalui kontrol versi, jadi fokus pada standar tingkat proyek daripada preferensi pribadi.

<Tip>
  Jalankan `/init` untuk menghasilkan CLAUDE.md awal secara otomatis. Claude menganalisis basis kode Anda dan membuat file dengan perintah build, instruksi test, dan konvensi proyek yang ditemukannya. Jika CLAUDE.md sudah ada, `/init` menyarankan perbaikan daripada menimpanya. Perbaiki dari sana dengan instruksi yang Claude tidak akan temukan sendiri.

  Atur `CLAUDE_CODE_NEW_INIT=1` untuk mengaktifkan alur multi-fase interaktif. `/init` menanyakan artefak mana yang akan diatur: file CLAUDE.md, skills, dan hooks. Kemudian mengeksplorasi basis kode Anda dengan subagent, mengisi celah melalui pertanyaan lanjutan, dan menyajikan proposal yang dapat ditinjau sebelum menulis file apa pun.
</Tip>

<h3 id="write-effective-instructions">
  Tulis instruksi yang efektif
</h3>

File CLAUDE.md dimuat ke dalam context window di awal setiap sesi, mengonsumsi token bersama percakapan Anda. [Visualisasi context window](/docs/id/context-window) menunjukkan di mana CLAUDE.md dimuat relatif terhadap sisa startup context. Karena mereka adalah konteks daripada konfigurasi yang diberlakukan, cara Anda menulis instruksi mempengaruhi seberapa andal Claude mengikutinya. Instruksi yang spesifik, ringkas, dan terstruktur dengan baik bekerja paling baik.

**Ukuran**: targetkan di bawah 200 baris per file CLAUDE.md. File yang lebih panjang mengonsumsi lebih banyak konteks dan mengurangi kepatuhan. Jika instruksi Anda berkembang besar, gunakan [aturan bersyarat jalur](#path-specific-rules) sehingga instruksi hanya dimuat ketika Claude bekerja dengan file yang cocok, menghemat ruang konteks. Anda juga dapat membagi konten menjadi [impor](#import-additional-files) untuk organisasi, meskipun file yang diimpor masih dimuat dan memasuki context window saat peluncuran.

**Struktur**: gunakan header markdown dan bullet untuk mengelompokkan instruksi terkait. Claude memindai struktur dengan cara yang sama seperti pembaca: bagian yang terorganisir lebih mudah diikuti daripada paragraf padat.

**Spesifisitas**: tulis instruksi yang cukup konkret untuk diverifikasi. Misalnya:

* "Gunakan indentasi 2 spasi" daripada "Format kode dengan baik"
* "Jalankan `npm test` sebelum commit" daripada "Uji perubahan Anda"
* "Handler API berada di `src/api/handlers/`" daripada "Jaga file tetap terorganisir"

**Konsistensi**: jika dua aturan saling bertentangan, Claude mungkin memilih satu secara sembarangan. Tinjau file CLAUDE.md Anda, file CLAUDE.md bersarang di subdirektori, dan file [`.claude/rules/`](#organize-rules-with-claude/rules/) secara berkala untuk menghapus instruksi yang ketinggalan zaman atau bertentangan. Dalam monorepo, gunakan [`claudeMdExcludes`](#exclude-specific-claude-md-files) untuk melewati file CLAUDE.md dari tim lain yang tidak relevan dengan pekerjaan Anda.

<h3 id="import-additional-files">
  Impor file tambahan
</h3>

File CLAUDE.md dapat mengimpor file tambahan menggunakan sintaks `@path/to/import`. File yang diimpor diperluas dan dimuat ke dalam konteks saat peluncuran bersama CLAUDE.md yang mereferensikannya.

Jalur relatif dan absolut diizinkan. Jalur relatif diselesaikan relatif terhadap file yang berisi impor, bukan direktori kerja. File yang diimpor dapat secara rekursif mengimpor file lain, dengan kedalaman maksimal empat hop.

Penguraian impor melewati rentang kode Markdown dan blok kode yang dibatasi. Untuk menyebutkan jalur di CLAUDE.md Anda tanpa mengimpornya, bungkus dalam backtick: menulis `` `@README` `` membuat teks tetap literal, sementara `@README` di luar backtick mengimpor file.

Untuk menarik README, package.json, dan panduan alur kerja, referensikan mereka dengan sintaks `@` di mana saja di CLAUDE.md Anda:

```text theme={null}
Lihat @README untuk gambaran umum proyek dan @package.json untuk perintah npm yang tersedia untuk proyek ini.

# Instruksi Tambahan
- alur kerja git @docs/git-instructions.md
```

Untuk preferensi pribadi per-proyek yang tidak ingin Anda periksa ke dalam kontrol versi, buat `CLAUDE.local.md` di root proyek. Itu dimuat bersama `CLAUDE.md` dan diperlakukan dengan cara yang sama. Tambahkan `CLAUDE.local.md` ke `.gitignore` Anda sehingga tidak dikomit; menjalankan `/init` dan memilih opsi pribadi melakukan ini untuk Anda.

Jika Anda bekerja di seluruh beberapa git worktrees dari repositori yang sama, `CLAUDE.local.md` yang diabaikan git hanya ada di worktree tempat Anda membuatnya. Untuk berbagi instruksi pribadi di seluruh worktrees, impor file dari direktori home Anda sebagai gantinya:

```text theme={null}
# Preferensi Individu
- @~/.claude/my-project-instructions.md
```

<Warning>
  Pertama kali Claude Code menemukan impor eksternal dalam proyek, itu menampilkan dialog persetujuan yang mencantumkan file. Jika Anda menolak, impor tetap dinonaktifkan dan dialog tidak muncul lagi.
</Warning>

Untuk pendekatan yang lebih terstruktur untuk mengorganisir instruksi, lihat [`.claude/rules/`](#organize-rules-with-claude/rules/).

<h3 id="agents-md">
  AGENTS.md
</h3>

Claude Code membaca `CLAUDE.md`, bukan `AGENTS.md`. Jika repositori Anda sudah menggunakan `AGENTS.md` untuk agen pengkodean lain, buat `CLAUDE.md` yang mengimpornya sehingga kedua alat membaca instruksi yang sama tanpa menduplikasinya. Anda juga dapat menambahkan instruksi khusus Claude di bawah impor. Claude memuat file yang diimpor saat awal sesi, kemudian menambahkan sisanya:

```markdown CLAUDE.md theme={null}
@AGENTS.md

## Claude Code

Gunakan plan mode untuk perubahan di bawah `src/billing/`.
```

Symlink juga berfungsi jika Anda tidak perlu menambahkan konten khusus Claude:

```bash theme={null}
ln -s AGENTS.md CLAUDE.md
```

Di Windows, membuat symlink memerlukan hak istimewa Administrator atau Developer Mode, jadi gunakan impor `@AGENTS.md` sebagai gantinya.

Menjalankan [`/init`](/docs/id/commands) di repo yang sudah memiliki `AGENTS.md` membacanya dan menggabungkan bagian yang relevan ke dalam `CLAUDE.md` yang dihasilkan. Itu juga membaca konfigurasi alat lain seperti `.cursorrules`, `.devin/rules/`, dan `.windsurfrules`.

<h3 id="how-claude-md-files-load">
  Bagaimana file CLAUDE.md dimuat
</h3>

Claude Code membaca file CLAUDE.md dengan berjalan naik pohon direktori dari direktori kerja saat ini, memeriksa setiap direktori di sepanjang jalan untuk file `CLAUDE.md` dan `CLAUDE.local.md`. Ini berarti jika Anda menjalankan Claude Code di `foo/bar/`, itu memuat instruksi dari `foo/bar/CLAUDE.md`, `foo/CLAUDE.md`, dan file `CLAUDE.local.md` apa pun di sebelahnya.

Semua file yang ditemukan digabungkan ke dalam konteks daripada menimpa satu sama lain. Dalam setiap direktori, konten diurutkan dari root sistem file ke bawah ke direktori kerja Anda. Untuk contoh `foo/bar/`, `foo/CLAUDE.md` muncul dalam konteks sebelum `foo/bar/CLAUDE.md`, jadi instruksi yang lebih dekat dengan tempat Anda meluncurkan Claude dibaca terakhir. Dalam setiap direktori, `CLAUDE.local.md` ditambahkan setelah `CLAUDE.md`, jadi catatan pribadi Anda adalah hal terakhir yang Claude baca di tingkat itu.

Claude juga menemukan file `CLAUDE.md` dan `CLAUDE.local.md` di subdirektori di bawah direktori kerja saat ini. Alih-alih memuatnya saat peluncuran, mereka disertakan ketika Claude membaca file di subdirektori tersebut.

Jika Anda bekerja di monorepo besar di mana file CLAUDE.md tim lain diambil, gunakan [`claudeMdExcludes`](#exclude-specific-claude-md-files) untuk melewatinya. Untuk tata letak lengkap file CLAUDE.md root dan per-direktori serta aturan, lihat [Monorepo dan repo besar](/docs/id/large-codebases).

Komentar HTML tingkat blok (`<!-- maintainer notes -->`) dalam file CLAUDE.md dihapus sebelum konten disuntikkan ke dalam konteks Claude. Gunakan mereka untuk meninggalkan catatan bagi pengelola manusia tanpa menghabiskan token konteks pada mereka. Komentar di dalam blok kode dipertahankan. Ketika Anda membuka file CLAUDE.md secara langsung dengan alat Read, komentar tetap terlihat.

<h4 id="load-from-additional-directories">
  Muat dari direktori tambahan
</h4>

Flag `--add-dir` memberikan Claude akses ke direktori tambahan di luar direktori kerja utama Anda. Secara default, file CLAUDE.md dari direktori ini tidak dimuat.

Untuk juga memuat file memori dari direktori tambahan, atur variabel lingkungan `CLAUDE_CODE_ADDITIONAL_DIRECTORIES_CLAUDE_MD`:

```bash theme={null}
CLAUDE_CODE_ADDITIONAL_DIRECTORIES_CLAUDE_MD=1 claude --add-dir ../shared-config
```

Ini memuat `CLAUDE.md`, `.claude/CLAUDE.md`, `.claude/rules/*.md`, dan `CLAUDE.local.md` dari direktori tambahan. `CLAUDE.local.md` dilewati jika Anda mengecualikan `local` dari [`--setting-sources`](/docs/id/cli-reference).

<h3 id="organize-rules-with-claude/rules/">
  Organisir aturan dengan `.claude/rules/`
</h3>

Untuk proyek yang lebih besar, Anda dapat mengorganisir instruksi menjadi beberapa file menggunakan direktori `.claude/rules/`. Ini membuat instruksi modular dan lebih mudah bagi tim untuk dipertahankan. Aturan juga dapat [dibatasi ke jalur file tertentu](#path-specific-rules), sehingga mereka hanya dimuat ke dalam konteks ketika Claude bekerja dengan file yang cocok, mengurangi kebisingan dan menghemat ruang konteks.

<Note>
  Aturan dimuat ke dalam konteks setiap sesi atau ketika file yang cocok dibuka. Untuk instruksi khusus tugas yang tidak perlu berada dalam konteks sepanjang waktu, gunakan [skills](/docs/id/skills) sebagai gantinya, yang hanya dimuat ketika Anda menginvokasinya atau ketika Claude menentukan mereka relevan dengan prompt Anda.
</Note>

<h4 id="set-up-rules">
  Siapkan aturan
</h4>

Tempatkan file markdown di direktori `.claude/rules/` proyek Anda. Setiap file harus mencakup satu topik, dengan nama file deskriptif seperti `testing.md` atau `api-design.md`. Semua file `.md` ditemukan secara rekursif, jadi Anda dapat mengorganisir aturan ke dalam subdirektori seperti `frontend/` atau `backend/`:

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

<h4 id="path-specific-rules">
  Aturan khusus jalur
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

Aturan tanpa bidang `paths` dimuat tanpa syarat dan berlaku untuk semua file. Aturan bersyarat jalur dipicu ketika Claude membaca file yang cocok dengan pola, bukan pada setiap penggunaan alat. Sejak v2.1.198, pencocokan juga berfungsi ketika Claude mencapai file melalui jalur symlink ke direktori proyek, misalnya dalam checkout yang disymlink.

Gunakan pola glob di bidang `paths` untuk mencocokkan file berdasarkan ekstensi, direktori, atau kombinasi apa pun:

| Pola                   | Cocok dengan                               |
| ---------------------- | ------------------------------------------ |
| `**/*.ts`              | Semua file TypeScript di direktori apa pun |
| `src/**/*`             | Semua file di bawah direktori `src/`       |
| `*.md`                 | File Markdown di root proyek               |
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

Sintaks glob memperlakukan `[` sebagai awal ekspresi bracket seperti `[abc]`. Pola dengan `[` yang tidak dapat dibaca sebagai ekspresi bracket, seperti `photos [2024/**`, tidak valid: itu tidak cocok dengan apa pun, dan pola lain dari aturan terus bekerja. Untuk mencocokkan `[` literal dalam nama file, lepaskan sebagai `photos \[2024/**`. Sebelum v2.1.207, satu pola yang tidak valid membuat alat Read gagal untuk setiap file yang aturan dievaluasi, alih-alih tidak cocok dengan apa pun.

<h4 id="share-rules-across-projects-with-symlinks">
  Bagikan aturan lintas proyek dengan symlinks
</h4>

Direktori `.claude/rules/` mendukung symlinks, jadi Anda dapat mempertahankan set aturan bersama dan menautkannya ke beberapa proyek. Symlinks diselesaikan dan dimuat secara normal, dan symlinks melingkar terdeteksi dan ditangani dengan anggun.

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

Aturan tingkat pengguna dimuat sebelum aturan proyek, memberikan aturan proyek prioritas lebih tinggi.

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
    Gunakan MDM, Group Policy, Ansible, atau alat serupa untuk mendistribusikan file di seluruh mesin pengembang. Lihat [pengaturan terkelola](/docs/id/permissions#managed-settings) untuk opsi konfigurasi di seluruh organisasi lainnya.
  </Step>
</Steps>

Kunci `claudeMd` memungkinkan Anda menempatkan konten CLAUDE.md terkelola langsung di dalam `managed-settings.json` alih-alih menerapkan file terpisah.

**Cakupan**: setiap sesi Claude Code di mesin, di setiap repositori. Untuk panduan khusus repositori, komit CLAUDE.md proyek sebagai gantinya.

**Prioritas**: sama dengan file CLAUDE.md terkelola. Dimuat sebelum CLAUDE.md pengguna dan proyek.

**Di mana itu dihormati**: pengaturan terkelola dan kebijakan saja. Menetapkan `claudeMd` dalam pengaturan pengguna, proyek, atau lokal tidak berpengaruh.

Contoh di bawah menambahkan instruksi perilaku langsung dalam file pengaturan terkelola:

```json theme={null}
{
  "claudeMd": "Always run `make lint` before committing.\nNever push directly to main."
}
```

CLAUDE.md terkelola dan [pengaturan terkelola](/docs/id/settings#settings-files) melayani tujuan yang berbeda. Gunakan pengaturan untuk penegakan teknis dan CLAUDE.md untuk panduan perilaku:

| Kekhawatiran                                    | Konfigurasi dalam                                             |
| :---------------------------------------------- | :------------------------------------------------------------ |
| Blokir alat, perintah, atau jalur file tertentu | Pengaturan terkelola: `permissions.deny`                      |
| Paksakan isolasi sandbox                        | Pengaturan terkelola: `sandbox.enabled`                       |
| Variabel lingkungan dan perutean penyedia API   | Pengaturan terkelola: `env`                                   |
| Metode autentikasi dan kunci organisasi         | Pengaturan terkelola: `forceLoginMethod`, `forceLoginOrgUUID` |
| Panduan gaya kode dan kualitas                  | CLAUDE.md terkelola                                           |
| Pengingat penanganan data dan kepatuhan         | CLAUDE.md terkelola                                           |
| Instruksi perilaku untuk Claude                 | CLAUDE.md terkelola                                           |

Aturan pengaturan diberlakukan oleh klien terlepas dari apa yang Claude putuskan untuk dilakukan. Instruksi CLAUDE.md membentuk perilaku Claude tetapi bukan lapisan penegakan keras.

<h4 id="exclude-specific-claude-md-files">
  Kecualikan file CLAUDE.md tertentu
</h4>

Dalam monorepo besar, file CLAUDE.md leluhur mungkin berisi instruksi yang tidak relevan dengan pekerjaan Anda. Pengaturan `claudeMdExcludes` memungkinkan Anda melewati file tertentu berdasarkan jalur atau pola glob.

Contoh ini mengecualikan CLAUDE.md tingkat atas dan direktori aturan dari folder induk. Tambahkan ke `.claude/settings.local.json` agar pengecualian tetap lokal ke mesin Anda:

```json theme={null}
{
  "claudeMdExcludes": [
    "**/monorepo/CLAUDE.md",
    "/home/user/monorepo/other-team/.claude/rules/**"
  ]
}
```

Pola dicocokkan dengan jalur file absolut menggunakan sintaks glob. Anda dapat mengonfigurasi `claudeMdExcludes` di lapisan [pengaturan](/docs/id/settings#settings-files) apa pun: pengguna, proyek, lokal, atau kebijakan terkelola. Array digabungkan di seluruh lapisan.

File CLAUDE.md kebijakan terkelola tidak dapat dikecualikan. Ini memastikan instruksi di seluruh organisasi selalu berlaku terlepas dari pengaturan individual.

<h2 id="auto-memory">
  Auto memory
</h2>

Auto memory memungkinkan Claude mengumpulkan pengetahuan lintas sesi tanpa Anda menulis apa pun. Claude menyimpan catatan untuk dirinya sendiri saat bekerja: perintah build, wawasan debugging, catatan arsitektur, preferensi gaya kode, dan kebiasaan alur kerja. Claude tidak menyimpan sesuatu setiap sesi. Itu memutuskan apa yang layak diingat berdasarkan apakah informasi akan berguna dalam percakapan masa depan.

<h3 id="enable-or-disable-auto-memory">
  Aktifkan atau nonaktifkan auto memory
</h3>

Auto memory aktif secara default. Untuk mengalihkannya, buka `/memory` dalam sesi dan gunakan toggle auto memory, atau atur `autoMemoryEnabled` dalam pengaturan proyek Anda:

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

Untuk menyimpan auto memory di lokasi berbeda, atur `autoMemoryDirectory` dalam `settings.json` Anda. Itu dibaca dari [cakupan pengaturan](/docs/id/settings#settings-precedence) apa pun: pengguna, proyek, lokal, kebijakan, atau `--settings`.

```json theme={null}
{
  "autoMemoryDirectory": "~/my-custom-memory-dir"
}
```

Nilai harus berupa jalur absolut atau dimulai dengan `~/`. Ketika diatur dalam `.claude/settings.json` atau `.claude/settings.local.json` proyek, nilai dihormati hanya setelah Anda menerima dialog kepercayaan ruang kerja untuk folder itu, gerbang yang sama yang mengatur hooks.

Direktori berisi titik masuk `MEMORY.md` dan file topik opsional:

```text theme={null}
~/.claude/projects/<project>/memory/
├── MEMORY.md          # Indeks ringkas, dimuat ke dalam setiap sesi
├── debugging.md       # Catatan terperinci tentang pola debugging
├── api-conventions.md # Keputusan desain API
└── ...                # File topik lain yang Claude buat
```

`MEMORY.md` bertindak sebagai indeks direktori memori. Claude membaca dan menulis file di direktori ini sepanjang sesi Anda, menggunakan `MEMORY.md` untuk melacak apa yang disimpan di mana.

Auto memory adalah mesin-lokal. Semua worktrees dan subdirektori dalam repositori git yang sama berbagi satu direktori auto memory. File tidak dibagikan di seluruh mesin atau lingkungan cloud.

<h3 id="how-it-works">
  Bagaimana cara kerjanya
</h3>

200 baris pertama `MEMORY.md`, atau 25KB pertama, mana pun yang lebih dulu, dimuat di awal setiap percakapan. Konten di luar batas itu tidak dimuat saat awal sesi. Claude membuat `MEMORY.md` ringkas dengan memindahkan catatan terperinci ke file topik terpisah.

Batas ini hanya berlaku untuk `MEMORY.md`. File CLAUDE.md dimuat sepenuhnya terlepas dari panjangnya, meskipun file yang lebih pendek menghasilkan kepatuhan yang lebih baik.

File topik seperti `debugging.md` atau `patterns.md` tidak dimuat saat startup. Claude membacanya sesuai permintaan menggunakan alat file standarnya ketika membutuhkan informasi.

Claude membaca dan menulis file memori selama sesi Anda. Ketika Anda melihat "Writing memory" atau "Recalled memory" di antarmuka Claude Code, Claude secara aktif memperbarui atau membaca dari `~/.claude/projects/<project>/memory/`.

<h3 id="audit-and-edit-your-memory">
  Audit dan edit memori Anda
</h3>

File auto memory adalah markdown biasa yang dapat Anda edit atau hapus kapan saja. Jalankan [`/memory`](#view-and-edit-with-%2Fmemory) untuk menelusuri dan membuka file memori dari dalam sesi.

<h2 id="view-and-edit-with-/memory">
  Lihat dan edit dengan `/memory`
</h2>

Perintah `/memory` mencantumkan semua file CLAUDE.md, CLAUDE.local.md, dan rules yang dimuat dalam sesi saat ini, memungkinkan Anda mengalihkan auto memory aktif atau mati, dan menyediakan tautan untuk membuka folder auto memory. Pilih file apa pun untuk membukanya di editor Anda.

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

* Jalankan `/memory` untuk memverifikasi file CLAUDE.md dan CLAUDE.local.md Anda dimuat. Jika file tidak terdaftar, Claude tidak dapat melihatnya.
* Periksa bahwa CLAUDE.md yang relevan berada di lokasi yang dimuat untuk sesi Anda (lihat [Choose where to put CLAUDE.md files](#choose-where-to-put-claude-md-files)).
* Buat instruksi lebih spesifik. "Gunakan indentasi 2 spasi" bekerja lebih baik daripada "format kode dengan baik."
* Cari instruksi yang bertentangan di seluruh file CLAUDE.md. Jika dua file memberikan panduan berbeda untuk perilaku yang sama, Claude mungkin memilih satu secara sembarangan.

Jika instruksi adalah sesuatu yang harus berjalan pada titik tertentu, seperti sebelum setiap commit atau setelah setiap pengeditan file, tulislah sebagai [hook](/docs/id/hooks-guide) sebagai gantinya. Hooks dieksekusi sebagai perintah shell pada peristiwa siklus hidup tetap dan berlaku terlepas dari apa yang Claude putuskan untuk lakukan.

Untuk instruksi yang Anda inginkan di tingkat prompt sistem, gunakan [`--append-system-prompt`](/docs/id/cli-reference#system-prompt-flags). Ini harus dilewatkan setiap invokasi, jadi lebih cocok untuk skrip dan otomasi daripada penggunaan interaktif.

<Tip>
  Gunakan [hook `InstructionsLoaded`](/docs/id/hooks#instructionsloaded) untuk mencatat dengan tepat file instruksi mana yang dimuat, kapan mereka dimuat, dan mengapa. Ini berguna untuk men-debug aturan khusus jalur atau file yang dimuat malas di subdirektori.
</Tip>

<h3 id="i-don’t-know-what-auto-memory-saved">
  I don't know what auto memory saved
</h3>

Jalankan `/memory` dan pilih folder auto memory untuk menelusuri apa yang telah disimpan Claude. Semuanya adalah markdown biasa yang dapat Anda baca, edit, atau hapus.

<h3 id="my-claude-md-is-too-large">
  My CLAUDE.md is too large
</h3>

File di atas 200 baris mengonsumsi lebih banyak konteks dan dapat mengurangi kepatuhan. Gunakan [path-scoped rules](#path-specific-rules) untuk memuat instruksi hanya ketika Claude bekerja dengan file yang cocok, atau pangkas konten yang tidak diperlukan dalam setiap sesi. Memisahkan ke dalam [impor `@path`](#import-additional-files) membantu organisasi tetapi tidak mengurangi konteks, karena file yang diimpor dimuat saat peluncuran.

The [`/doctor`](/docs/id/commands#all-commands) checkup proposes trims for a checked-in CLAUDE.md: it cuts content Claude can derive from the codebase, such as directory layouts, dependency lists, and architecture overviews, and keeps pitfalls, rationale, and conventions that differ from tool defaults. The trim check requires Claude Code v2.1.206 or later.

<h3 id="instructions-seem-lost-after-/compact">
  Instructions seem lost after `/compact`
</h3>

CLAUDE.md root proyek bertahan dari pemadatan: setelah `/compact`, Claude membaca ulang dari disk dan menyuntikkannya kembali ke dalam sesi. File CLAUDE.md bersarang di subdirektori tidak disuntikkan kembali secara otomatis; mereka dimuat ulang saat berikutnya Claude membaca file di subdirektori tersebut.

Jika instruksi hilang setelah pemadatan, itu diberikan hanya dalam percakapan atau berada di CLAUDE.md bersarang yang belum dimuat ulang. Tambahkan instruksi percakapan ke CLAUDE.md untuk membuatnya bertahan. Lihat [What survives compaction](/docs/id/context-window#what-survives-compaction) untuk rincian lengkap.

Lihat [Write effective instructions](#write-effective-instructions) untuk panduan tentang ukuran, struktur, dan spesifisitas.

<h2 id="related-resources">
  Sumber daya terkait
</h2>

* [Debug konfigurasi Anda](/docs/id/debug-your-config): diagnosis mengapa CLAUDE.md atau pengaturan tidak berlaku
* [Skills](/docs/id/skills): paket alur kerja yang dapat diulang yang dimuat sesuai permintaan
* [Settings](/docs/id/settings): konfigurasi perilaku Claude Code dengan file pengaturan
* [Memori subagent](/docs/id/sub-agents#enable-persistent-memory): biarkan subagents mempertahankan auto memory mereka sendiri
