> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Buat plugins

> Buat plugins kustom untuk memperluas Claude Code dengan skills, agents, hooks, dan MCP servers.

Plugins memungkinkan Anda memperluas Claude Code dengan fungsionalitas kustom yang dapat dibagikan di seluruh proyek dan tim. Panduan ini mencakup pembuatan plugins Anda sendiri dengan skills, agents, hooks, dan MCP servers.

Mencari untuk memasang plugins yang sudah ada? Lihat [Temukan dan pasang plugins](/docs/id/discover-plugins). Untuk spesifikasi teknis lengkap, lihat [Referensi plugins](/docs/id/plugins-reference).

<h2 id="when-to-use-plugins-vs-standalone-configuration">
  Kapan menggunakan plugins vs konfigurasi standalone
</h2>

Claude Code mendukung dua cara untuk menambahkan skills, agents, dan hooks kustom:

| Pendekatan                                                                                      | Nama skill           | Terbaik untuk                                                                                                      |
| :---------------------------------------------------------------------------------------------- | :------------------- | :----------------------------------------------------------------------------------------------------------------- |
| **Standalone** (direktori `.claude/`)                                                           | `/hello`             | Alur kerja pribadi, kustomisasi khusus proyek, eksperimen cepat                                                    |
| **Plugins** (direktori dengan skills, agents, hooks, atau manifes `.claude-plugin/plugin.json`) | `/plugin-name:hello` | Berbagi dengan rekan kerja, distribusi ke komunitas, rilis dengan versi, dapat digunakan kembali di seluruh proyek |

**Gunakan konfigurasi standalone ketika**:

* Anda menyesuaikan Claude Code untuk satu proyek
* Konfigurasi bersifat pribadi dan tidak perlu dibagikan
* Anda bereksperimen dengan skills atau hooks sebelum mengemas mereka
* Anda menginginkan nama skill pendek seperti `/hello` atau `/deploy`

**Gunakan plugins ketika**:

* Anda ingin berbagi fungsionalitas dengan tim atau komunitas Anda
* Anda memerlukan skills/agents yang sama di seluruh beberapa proyek
* Anda menginginkan kontrol versi dan pembaruan mudah untuk ekstensi Anda
* Anda mendistribusikan melalui marketplace
* Anda tidak keberatan dengan skills yang diberi namespace seperti `/my-plugin:hello` (namespace mencegah konflik antara plugins)

<Tip>
  Mulai dengan konfigurasi standalone di `.claude/` untuk iterasi cepat, kemudian [konversi ke plugin](#convert-existing-configurations-to-plugins) ketika Anda siap untuk berbagi.
</Tip>

<h2 id="quickstart">
  Quickstart
</h2>

Quickstart ini memandu Anda melalui pembuatan plugin dengan skill kustom. Anda akan membuat manifest (file konfigurasi yang mendefinisikan plugin Anda), menambahkan skill, dan mengujinya secara lokal menggunakan flag `--plugin-dir`.

<h3 id="prerequisites">
  Prasyarat
</h3>

* Claude Code [diinstal dan diautentikasi](/docs/id/quickstart#step-1-install-claude-code)

<Note>
  Jika Anda tidak melihat perintah `/plugin`, perbarui Claude Code ke versi terbaru. Lihat [Troubleshooting](/docs/id/troubleshooting) untuk instruksi upgrade.
</Note>

<h3 id="create-your-first-plugin">
  Buat plugin pertama Anda
</h3>

<Steps>
  <Step title="Buat direktori plugin">
    Setiap plugin berada di direktorinya sendiri yang berisi skills, agents, atau hooks Anda, secara opsional bersama dengan manifest `.claude-plugin/plugin.json`. Lokasi tidak penting untuk quickstart ini karena Anda akan menunjukkan Claude Code ke direktori dengan `--plugin-dir` di langkah pengujian. Buat di mana saja yang nyaman, seperti folder scratch atau direktori projects:

    ```bash theme={null}
    mkdir my-first-plugin
    ```

    Langkah-langkah yang tersisa berjalan dari direktori induk dan mereferensikan path seperti `my-first-plugin/...` relatif terhadapnya.
  </Step>

  <Step title="Buat manifest plugin">
    File manifest di `.claude-plugin/plugin.json` mendefinisikan identitas plugin Anda: nama, deskripsi, dan versinya. Claude Code menggunakan metadata ini untuk menampilkan plugin Anda di plugin manager.

    Buat direktori `.claude-plugin` di dalam folder plugin Anda:

    ```bash theme={null}
    mkdir my-first-plugin/.claude-plugin
    ```

    Kemudian buat `my-first-plugin/.claude-plugin/plugin.json` dengan konten ini:

    ```json my-first-plugin/.claude-plugin/plugin.json theme={null}
    {
      "name": "my-first-plugin",
      "description": "A greeting plugin to learn the basics",
      "version": "1.0.0",
      "author": {
        "name": "Your Name"
      }
    }
    ```

    | Field         | Tujuan                                                                                                                                                                                                                                                                                         |
    | :------------ | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
    | `name`        | Pengidentifikasi unik dan namespace skill. Skills diawali dengan ini (misalnya, `/my-first-plugin:hello`).                                                                                                                                                                                     |
    | `description` | Ditampilkan di plugin manager saat menjelajahi atau memasang plugins.                                                                                                                                                                                                                          |
    | `version`     | Opsional. Jika diatur, pengguna hanya menerima pembaruan ketika Anda menaikkan field ini. Jika dihilangkan dan plugin Anda didistribusikan melalui git, SHA commit digunakan dan setiap commit dihitung sebagai versi baru. Lihat [manajemen versi](/docs/id/plugins-reference#version-management). |
    | `author`      | Opsional. Membantu untuk atribusi.                                                                                                                                                                                                                                                             |

    Untuk field tambahan seperti `homepage`, `repository`, dan `license`, lihat [skema manifest lengkap](/docs/id/plugins-reference#plugin-manifest-schema).
  </Step>

  <Step title="Tambahkan skill">
    Skills berada di direktori `skills/`. Setiap skill adalah folder yang berisi file `SKILL.md`. Nama folder menjadi nama skill, diawali dengan namespace plugin (`hello/` dalam plugin bernama `my-first-plugin` membuat `/my-first-plugin:hello`).

    Buat direktori skill di folder plugin Anda:

    ```bash theme={null}
    mkdir -p my-first-plugin/skills/hello
    ```

    Kemudian buat `my-first-plugin/skills/hello/SKILL.md` dengan konten ini:

    ```markdown my-first-plugin/skills/hello/SKILL.md theme={null}
    ---
    description: Greet the user with a friendly message
    disable-model-invocation: true
    ---

    Greet the user warmly and ask how you can help them today.
    ```
  </Step>

  <Step title="Uji plugin Anda">
    Jalankan Claude Code dengan flag `--plugin-dir` untuk memuat plugin Anda:

    ```bash theme={null}
    claude --plugin-dir ./my-first-plugin
    ```

    Setelah Claude Code dimulai, coba skill baru Anda:

    ```shell theme={null}
    /my-first-plugin:hello
    ```

    Anda akan melihat Claude merespons dengan salam. Jalankan `/help` untuk melihat skill Anda terdaftar di bawah namespace plugin.

    <Note>
      **Mengapa namespace?** Plugin skills selalu diberi namespace (seperti `/my-first-plugin:hello`) untuk mencegah konflik ketika beberapa plugins memiliki skills dengan nama yang sama.

      Untuk mengubah awalan namespace, perbarui field `name` di `plugin.json`.
    </Note>
  </Step>

  <Step title="Tambahkan argumen skill">
    Buat skill Anda dinamis dengan menerima input pengguna. Placeholder `$ARGUMENTS` menangkap teks apa pun yang disediakan pengguna setelah nama skill.

    Perbarui file `SKILL.md` Anda:

    ```markdown my-first-plugin/skills/hello/SKILL.md theme={null}
    ---
    description: Greet the user with a personalized message
    ---

    # Hello Skill

    Greet the user named "$ARGUMENTS" warmly and ask how you can help them today. Make the greeting personal and encouraging.
    ```

    Jalankan `/reload-plugins` untuk mengambil perubahan, kemudian coba skill dengan nama Anda:

    ```shell theme={null}
    /my-first-plugin:hello Alex
    ```

    Claude akan menyapa Anda dengan nama. Untuk lebih lanjut tentang meneruskan argumen ke skills, lihat [Skills](/docs/id/skills#pass-arguments-to-skills).
  </Step>
</Steps>

Anda telah berhasil membuat dan menguji plugin dengan komponen kunci ini:

* **Plugin manifest** (`.claude-plugin/plugin.json`): menjelaskan metadata plugin Anda
* **Direktori skills** (`skills/`): berisi skills kustom Anda
* **Argumen skill** (`$ARGUMENTS`): menangkap input pengguna untuk perilaku dinamis

<Tip>
  Flag `--plugin-dir` berguna untuk pengembangan dan pengujian. Ketika Anda siap untuk berbagi plugin Anda dengan orang lain, lihat [Buat dan distribusikan marketplace plugin](/docs/id/plugin-marketplaces).
</Tip>

<h2 id="develop-a-plugin-in-your-skills-directory">
  Kembangkan plugin di direktori skills Anda
</h2>

Alih-alih meneruskan `--plugin-dir` pada setiap peluncuran, Anda dapat menyimpan plugin di direktori skills Anda dan membuat Claude Code memuatnya secara otomatis. `claude plugin init` membuat scaffolding untuk satu:

```bash theme={null}
claude plugin init my-tool
```

Ini membuat `~/.claude/skills/my-tool/` dengan manifest `.claude-plugin/plugin.json` dan `SKILL.md` pemula. Pada sesi berikutnya, plugin ini dimuat sebagai `my-tool@skills-dir` tanpa langkah marketplace atau instalasi.

Untuk aturan auto-load, cakupan pribadi vs. proyek, persyaratan workspace-trust, dan cara memperbarui atau menghapus satu, lihat [Skills-directory plugins](/docs/id/plugins-reference#skills-directory-plugins).

<h2 id="plugin-structure-overview">
  Ikhtisar struktur plugin
</h2>

Anda telah membuat plugin dengan skill, tetapi plugins dapat mencakup banyak hal lagi: agents kustom, hooks, MCP servers, LSP servers, dan background monitors.

<Warning>
  **Kesalahan umum**: Jangan letakkan `commands/`, `agents/`, `skills/`, atau `hooks/` di dalam direktori `.claude-plugin/`. Hanya `plugin.json` yang masuk ke dalam `.claude-plugin/`. Semua direktori lainnya harus berada di tingkat root plugin.

  Plugin root adalah direktori plugin individual itu sendiri: yang berisi `.claude-plugin/plugin.json`. Ini tidak pernah `~/.claude/`. Sebagai contoh, Claude Code tidak membaca `.mcp.json` yang ditempatkan di `~/.claude/.mcp.json`.
</Warning>

| Direktori         | Lokasi      | Tujuan                                                                            |
| :---------------- | :---------- | :-------------------------------------------------------------------------------- |
| `.claude-plugin/` | Root plugin | Berisi manifest `plugin.json` (opsional jika komponen menggunakan lokasi default) |
| `skills/`         | Root plugin | Skills sebagai direktori `<name>/SKILL.md`                                        |
| `commands/`       | Root plugin | Skills sebagai file Markdown datar. Gunakan `skills/` untuk plugins baru          |
| `agents/`         | Root plugin | Definisi agent kustom                                                             |
| `hooks/`          | Root plugin | Event handlers di `hooks.json`                                                    |
| `.mcp.json`       | Root plugin | Konfigurasi MCP server                                                            |
| `.lsp.json`       | Root plugin | Konfigurasi LSP server untuk code intelligence                                    |
| `monitors/`       | Root plugin | Konfigurasi background monitor di `monitors.json`                                 |
| `bin/`            | Root plugin | Executable yang ditambahkan ke `PATH` tool Bash saat plugin diaktifkan            |
| `settings.json`   | Root plugin | [Settings](/docs/id/settings) default yang diterapkan ketika plugin diaktifkan         |

Plugin yang mengirimkan tepat satu skill dapat menempatkan `SKILL.md` langsung di root plugin alih-alih membuat direktori `skills/`. Claude Code memuatnya sebagai skill tunggal dan menggunakan field frontmatter `name` untuk nama invokasi. Gunakan tata letak `skills/` untuk plugins yang mungkin berkembang menjadi lebih dari satu skill.

<Note>
  **Langkah berikutnya**: Siap menambahkan lebih banyak fitur? Lompat ke [Kembangkan plugins yang lebih kompleks](#develop-more-complex-plugins) untuk menambahkan agents, hooks, MCP servers, dan LSP servers. Untuk spesifikasi teknis lengkap dari semua komponen plugin, lihat [Referensi plugins](/docs/id/plugins-reference).
</Note>

<h2 id="develop-more-complex-plugins">
  Kembangkan plugins yang lebih kompleks
</h2>

Setelah Anda nyaman dengan plugins dasar, Anda dapat membuat ekstensi yang lebih canggih.

<h3 id="add-skills-to-your-plugin">
  Tambahkan Skills ke plugin Anda
</h3>

Plugins dapat mencakup [Agent Skills](/docs/id/skills) untuk memperluas kemampuan Claude. Skills diinvokasi oleh model: Claude secara otomatis menggunakannya berdasarkan konteks tugas.

Tambahkan direktori `skills/` di root plugin Anda dengan folder Skill yang berisi file `SKILL.md`:

```text theme={null}
my-plugin/
├── .claude-plugin/
│   └── plugin.json
└── skills/
    └── code-review/
        └── SKILL.md
```

Setiap `SKILL.md` berisi frontmatter YAML dan instruksi. Sertakan `description` sehingga Claude tahu kapan menggunakan skill:

```yaml theme={null}
---
description: Reviews code for best practices and potential issues. Use when reviewing code, checking PRs, or analyzing code quality.
---

When reviewing code, check for:
1. Code organization and structure
2. Error handling
3. Security concerns
4. Test coverage
```

Setelah memasang plugin, jalankan `/reload-plugins` untuk memuat Skills. Untuk panduan authoring Skill lengkap termasuk progressive disclosure dan pembatasan tool, lihat [Agent Skills](/docs/id/skills).

<h3 id="add-lsp-servers-to-your-plugin">
  Tambahkan LSP servers ke plugin Anda
</h3>

<Tip>
  Untuk bahasa umum seperti TypeScript, Python, dan Rust, pasang plugin LSP yang sudah dibangun sebelumnya dari marketplace resmi. Buat plugin LSP kustom hanya ketika Anda memerlukan dukungan untuk bahasa yang belum tercakup.
</Tip>

Plugin LSP (Language Server Protocol) memberikan Claude code intelligence real-time. Jika Anda perlu mendukung bahasa yang tidak memiliki plugin LSP resmi, Anda dapat membuat plugin Anda sendiri dengan menambahkan file `.lsp.json` ke plugin Anda:

```json .lsp.json theme={null}
{
  "go": {
    "command": "gopls",
    "args": ["serve"],
    "extensionToLanguage": {
      ".go": "go"
    }
  }
}
```

Pengguna yang memasang plugin Anda harus memiliki binary language server yang diinstal di mesin mereka.

Untuk opsi konfigurasi LSP lengkap, lihat [LSP servers](/docs/id/plugins-reference#lsp-servers).

<h3 id="add-background-monitors-to-your-plugin">
  Tambahkan background monitors ke plugin Anda
</h3>

Background monitors memungkinkan plugin Anda untuk menonton logs, file, atau status eksternal di latar belakang dan memberi tahu Claude saat event tiba. Claude Code memulai setiap monitor secara otomatis ketika plugin aktif, jadi Anda tidak perlu menginstruksikan Claude untuk memulai watch.

Tambahkan file `monitors/monitors.json` di root plugin dengan array entri monitor:

```json monitors/monitors.json theme={null}
[
  {
    "name": "error-log",
    "command": "tail -F ./logs/error.log",
    "description": "Application error log"
  }
]
```

Setiap baris stdout dari `command` dikirimkan ke Claude sebagai notifikasi selama sesi. Untuk skema lengkap, termasuk trigger `when` dan substitusi variabel, lihat [Monitors](/docs/id/plugins-reference#monitors).

<h3 id="ship-default-settings-with-your-plugin">
  Kirim default settings dengan plugin Anda
</h3>

Plugins dapat menyertakan file `settings.json` di root plugin untuk menerapkan konfigurasi default ketika plugin diaktifkan. Saat ini, hanya key `agent` dan `subagentStatusLine` yang didukung.

Mengatur `agent` mengaktifkan salah satu [custom agents](/docs/id/sub-agents) plugin sebagai thread utama, menerapkan system prompt, pembatasan tool, dan modelnya. Ini memungkinkan plugin untuk mengubah perilaku Claude Code secara default ketika diaktifkan.

```json settings.json theme={null}
{
  "agent": "security-reviewer"
}
```

Contoh ini mengaktifkan agent `security-reviewer` yang didefinisikan di direktori `agents/` plugin. Settings dari `settings.json` mengambil prioritas atas `settings` yang dideklarasikan di `plugin.json`. Key yang tidak dikenal diabaikan secara diam-diam.

<h3 id="organize-complex-plugins">
  Organisir plugins kompleks
</h3>

Untuk plugins dengan banyak komponen, organisir struktur direktori Anda berdasarkan fungsionalitas. Untuk layout direktori lengkap dan pola organisasi, lihat [Struktur direktori plugin](/docs/id/plugins-reference#plugin-directory-structure).

<h3 id="test-your-plugins-locally">
  Uji plugins Anda secara lokal
</h3>

Gunakan flag `--plugin-dir` untuk menguji plugins selama pengembangan. Ini memuat plugin Anda secara langsung tanpa memerlukan instalasi.

```bash theme={null}
claude --plugin-dir ./my-plugin
```

Flag juga menerima arsip `.zip` dari direktori plugin, yang memerlukan Claude Code v2.1.128 atau lebih baru.

```bash theme={null}
claude --plugin-dir ./my-plugin.zip
```

Ketika plugin `--plugin-dir` memiliki nama yang sama dengan plugin marketplace yang diinstal, salinan lokal mengambil prioritas untuk sesi itu. Ini memungkinkan Anda menguji perubahan pada plugin yang sudah Anda instal tanpa mencopot pemasangannya terlebih dahulu. Pengecualian adalah plugins yang managed settings force-enable atau force-disable: `--plugin-dir` tidak dapat menimpa pengaturan tersebut.

Saat Anda membuat perubahan pada plugin Anda, jalankan `/reload-plugins` untuk mengambil pembaruan tanpa memulai ulang. Ini memuat ulang plugins, skills, agents, hooks, plugin MCP servers, dan plugin LSP servers. Uji komponen plugin Anda:

* Coba skills Anda dengan `/plugin-name:skill-name`
* Periksa bahwa agents muncul di `/context` di bawah Custom Agents, atau @-mention salah satu dengan nama scoped-nya
* Verifikasi hooks bekerja seperti yang diharapkan

<Tip>
  Anda dapat memuat beberapa plugins sekaligus dengan menentukan flag berkali-kali:

  ```bash theme={null}
  claude --plugin-dir ./plugin-one --plugin-dir ./plugin-two
  ```
</Tip>

Untuk menguji plugin yang sudah dikemas sebagai arsip `.zip` dan dihosting di URL, seperti artefak build CI, gunakan `--plugin-url` sebagai gantinya. Claude Code mengambil arsip saat startup dan memuatnya hanya untuk sesi itu. Jika pengambilan gagal atau arsip tidak valid, Claude Code melaporkan kesalahan pemuatan plugin dan dimulai tanpanya. Pertimbangan [kepercayaan](/docs/id/discover-plugins#security) yang sama berlaku seperti untuk sumber plugin apa pun: hanya arahkan flag ini ke arsip yang Anda kontrol atau percayai.

Untuk memuat beberapa plugins, ulangi flag untuk setiap URL:

```bash theme={null}
claude --plugin-url https://example.com/my-plugin.zip --plugin-url https://example.com/other.zip
```

Atau teruskan URL yang dipisahkan spasi sebagai satu argumen yang dikutip:

```bash theme={null}
claude --plugin-url "https://example.com/my-plugin.zip https://example.com/other.zip"
```

<h3 id="debug-plugin-issues">
  Debug masalah plugin
</h3>

Jika plugin Anda tidak bekerja seperti yang diharapkan:

1. **Periksa struktur**: Pastikan direktori Anda berada di root plugin, bukan di dalam `.claude-plugin/`
2. **Uji komponen secara individual**: Periksa setiap skill, agent, dan hook secara terpisah
3. **Gunakan alat validasi dan debugging**: Lihat [Alat debugging dan pengembangan](/docs/id/plugins-reference#debugging-and-development-tools) untuk perintah CLI dan teknik troubleshooting

<h3 id="share-your-plugins">
  Bagikan plugins Anda
</h3>

Ketika plugin Anda siap untuk dibagikan:

1. **Tambahkan dokumentasi**: Sertakan `README.md` dengan instruksi instalasi dan penggunaan
2. **Pilih strategi versioning**: Tentukan apakah akan menetapkan `version` eksplisit atau mengandalkan SHA commit git. Lihat [manajemen versi](/docs/id/plugins-reference#version-management)
3. **Buat atau gunakan marketplace**: Distribusikan melalui [plugin marketplaces](/docs/id/plugin-marketplaces) untuk instalasi
4. **Uji dengan orang lain**: Minta anggota tim menguji plugin sebelum distribusi yang lebih luas

Setelah plugin Anda berada di marketplace, orang lain dapat memasangnya menggunakan instruksi di [Temukan dan pasang plugins](/docs/id/discover-plugins). Untuk menjaga plugin tetap internal bagi tim Anda, hosting marketplace di [private repository](/docs/id/plugin-marketplaces#private-repositories).

<h3 id="submit-your-plugin-to-the-community-marketplace">
  Kirimkan plugin Anda ke marketplace komunitas
</h3>

Anthropic memelihara dua marketplace publik untuk plugin Claude Code:

* **`claude-plugins-official`**: serangkaian plugin yang dikurasi yang dikelola oleh Anthropic. Terdaftar secara otomatis saat pertama kali Anda memulai Claude Code secara interaktif. Skrip non-interaktif yang berjalan sebelum peluncuran pertama itu harus menambahkannya secara eksplisit dengan `claude plugin marketplace add anthropics/claude-plugins-official`.
* **`claude-community`**: marketplace komunitas publik tempat pengajuan pihak ketiga mendarat setelah review. Pengguna menambahkannya dengan `/plugin marketplace add anthropics/claude-plugins-community` dan memasangnya sebagai `@claude-community`.

Untuk mengirimkan plugin Anda untuk review marketplace komunitas, gunakan salah satu formulir in-app:

* **claude.ai**: [claude.ai/admin-settings/directory/submissions/plugins/new](https://claude.ai/admin-settings/directory/submissions/plugins/new)
* **Console**: [platform.claude.com/plugins/submit](https://platform.claude.com/plugins/submit)

Formulir claude.ai memerlukan organisasi Team atau Enterprise dan akses manajemen direktori; Owners organisasi memiliki akses ini secara default. Penulis individual yang bukan bagian dari organisasi Team atau Enterprise dapat menggunakan formulir Console sebagai gantinya.

Jalankan `claude plugin validate` secara lokal sebelum Anda mengirimkan. Pipeline review menjalankan pemeriksaan yang sama pada setiap pengajuan, bersama dengan screening keamanan otomatis.

Plugin yang disetujui disematkan ke SHA commit tertentu di katalog [`anthropics/claude-plugins-community`](https://github.com/anthropics/claude-plugins-community), dan CI membump pin secara otomatis saat Anda push commit baru ke repository Anda. Katalog publik disinkronkan setiap malam dari pipeline review, jadi dapat ada penundaan antara persetujuan dan plugin Anda muncul di `marketplace.json`. Untuk memeriksa apakah plugin Anda dapat dipasang, cari namanya di [katalog komunitas](https://github.com/anthropics/claude-plugins-community/blob/main/.claude-plugin/marketplace.json).

Marketplace resmi, `claude-plugins-official`, dikurasi secara terpisah. Anthropic memutuskan plugin mana yang akan disertakan atas kebijakannya. Tidak ada proses aplikasi, dan formulir pengajuan tidak menambahkan plugins ke marketplace resmi.

Jika Anthropic mencantumkan plugin Anda di marketplace resmi, CLI Anda dapat meminta pengguna Claude Code untuk memasangnya. Lihat [Rekomendasikan plugin Anda dari CLI Anda](/docs/id/plugin-hints).

<Note>
  Untuk spesifikasi teknis lengkap, teknik debugging, dan strategi distribusi, lihat [Referensi plugins](/docs/id/plugins-reference).
</Note>

<h2 id="convert-existing-configurations-to-plugins">
  Konversi konfigurasi yang ada ke plugins
</h2>

Jika Anda sudah memiliki skills atau hooks di direktori `.claude/` Anda, Anda dapat mengonversinya menjadi plugin untuk berbagi dan distribusi yang lebih mudah.

<h3 id="migration-steps">
  Langkah migrasi
</h3>

<Steps>
  <Step title="Buat struktur plugin">
    Buat direktori plugin baru di root proyek Anda, bersama dengan folder `.claude/` yang sudah ada, sehingga path `cp` relatif di langkah berikutnya dapat diselesaikan:

    ```bash theme={null}
    mkdir -p my-plugin/.claude-plugin
    ```

    Buat file manifest di `my-plugin/.claude-plugin/plugin.json`:

    ```json my-plugin/.claude-plugin/plugin.json theme={null}
    {
      "name": "my-plugin",
      "description": "Migrated from standalone configuration",
      "version": "1.0.0"
    }
    ```
  </Step>

  <Step title="Salin file yang ada">
    Salin konfigurasi yang ada ke direktori plugin:

    ```bash theme={null}
    # Copy commands
    cp -r .claude/commands my-plugin/

    # Copy agents (if any)
    cp -r .claude/agents my-plugin/

    # Copy skills (if any)
    cp -r .claude/skills my-plugin/
    ```
  </Step>

  <Step title="Migrasi hooks">
    Jika Anda memiliki hooks di settings Anda, buat direktori hooks:

    ```bash theme={null}
    mkdir my-plugin/hooks
    ```

    Buat `my-plugin/hooks/hooks.json` dengan konfigurasi hooks Anda. Salin objek `hooks` dari `.claude/settings.json` atau `settings.local.json` Anda, karena formatnya sama. Perintah menerima input hook sebagai JSON di stdin, jadi gunakan `jq` untuk mengekstrak path file:

    ```json my-plugin/hooks/hooks.json theme={null}
    {
      "hooks": {
        "PostToolUse": [
          {
            "matcher": "Write|Edit",
            "hooks": [{ "type": "command", "command": "jq -r '.tool_input.file_path' | xargs npm run lint:fix" }]
          }
        ]
      }
    }
    ```
  </Step>

  <Step title="Uji plugin yang dimigrasikan">
    Muat plugin Anda untuk memverifikasi semuanya berfungsi:

    ```bash theme={null}
    claude --plugin-dir ./my-plugin
    ```

    Uji setiap komponen: jalankan commands Anda, periksa agents muncul di `/context`, dan verifikasi hooks dipicu dengan benar.
  </Step>
</Steps>

<h3 id="what-changes-when-migrating">
  Apa yang berubah saat migrasi
</h3>

| Standalone (`.claude/`)                    | Plugin                               |
| :----------------------------------------- | :----------------------------------- |
| Hanya tersedia di satu proyek              | Dapat dibagikan melalui marketplaces |
| File di `.claude/commands/`                | File di `plugin-name/commands/`      |
| Hooks di `settings.json`                   | Hooks di `hooks/hooks.json`          |
| Harus menyalin secara manual untuk berbagi | Pasang dengan `/plugin install`      |

<Note>
  Setelah migrasi, hapus file asli dari `.claude/` untuk menghindari duplikat. Definisi `.claude/agents/` proyek dan pengguna menggantikan agents plugin dengan nama yang sama, jadi versi plugin hanya berlaku setelah yang asli dihapus. Plugin skills diberi namespace sebagai `/plugin-name:skill-name`, jadi `/skill-name` asli dan salinan plugin keduanya tetap tersedia daripada satu menggantikan yang lain.
</Note>

<h2 id="next-steps">
  Langkah berikutnya
</h2>

Sekarang bahwa Anda memahami sistem plugin Claude Code, berikut adalah jalur yang disarankan untuk tujuan yang berbeda:

<h3 id="for-plugin-users">
  Untuk pengguna plugin
</h3>

* [Temukan dan pasang plugins](/docs/id/discover-plugins): jelajahi marketplaces dan pasang plugins
* [Konfigurasi marketplaces tim](/docs/id/discover-plugins#configure-team-marketplaces): atur plugins tingkat repository untuk tim Anda

<h3 id="for-plugin-developers">
  Untuk pengembang plugin
</h3>

* [Buat dan distribusikan marketplace](/docs/id/plugin-marketplaces): paket dan bagikan plugins Anda
* [Referensi plugins](/docs/id/plugins-reference): spesifikasi teknis lengkap
* Selami lebih dalam komponen plugin spesifik:
  * [Skills](/docs/id/skills): detail pengembangan skill
  * [Subagents](/docs/id/sub-agents): konfigurasi dan kemampuan agent
  * [Hooks](/docs/id/hooks): penanganan event dan otomasi
  * [MCP](/docs/id/mcp): integrasi tool eksternal
