> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Buat subagent khusus

> Buat dan gunakan subagent AI khusus di Claude Code untuk alur kerja khusus tugas dan manajemen konteks yang lebih baik.

Subagent adalah asisten AI khusus yang menangani jenis tugas tertentu. Gunakan satu ketika tugas sampingan akan membanjiri percakapan utama Anda dengan hasil pencarian, log, atau konten file yang tidak akan Anda referensikan lagi: subagent melakukan pekerjaan itu dalam konteksnya sendiri dan hanya mengembalikan ringkasan. Tentukan subagent khusus ketika Anda terus menelurkan jenis pekerja yang sama dengan instruksi yang sama.

Setiap subagent berjalan di jendela konteksnya sendiri dengan prompt sistem khusus, akses alat tertentu, dan izin independen. Ketika Claude menemukan tugas yang sesuai dengan deskripsi subagent, Claude mendelegasikan ke subagent tersebut, yang bekerja secara independen dan mengembalikan hasil. Untuk melihat penghematan konteks dalam praktik, [visualisasi jendela konteks](/docs/id/context-window) menjelaskan sesi di mana subagent menangani penelitian di jendela terpisahnya sendiri.

<Note>
  Subagent bekerja dalam satu sesi. Untuk menjalankan banyak sesi independen secara paralel dan memantaunya dari satu tempat, lihat [agen latar belakang](/docs/id/agent-view). Untuk sesi yang berkomunikasi satu sama lain, lihat [tim agen](/docs/id/agent-teams).
</Note>

Subagent membantu Anda:

* **Mempertahankan konteks** dengan menjaga eksplorasi dan implementasi di luar percakapan utama Anda
* **Menerapkan batasan** dengan membatasi alat mana yang dapat digunakan subagent
* **Menggunakan kembali konfigurasi** di seluruh proyek dengan subagent tingkat pengguna
* **Mengkhususkan perilaku** dengan prompt sistem yang terfokus untuk domain tertentu
* **Mengontrol biaya** dengan merutekan tugas ke model yang lebih cepat dan lebih murah seperti Haiku

Claude menggunakan deskripsi setiap subagent untuk memutuskan kapan mendelegasikan tugas. Ketika Anda membuat subagent, tulis deskripsi yang jelas sehingga Claude tahu kapan menggunakannya.

Claude Code mencakup beberapa subagent bawaan seperti Explore, Plan, dan general-purpose. Anda juga dapat membuat subagent khusus untuk menangani tugas tertentu.

<h2 id="built-in-subagents">
  Subagent bawaan
</h2>

Claude Code mencakup subagent bawaan yang Claude gunakan secara otomatis jika sesuai. Masing-masing mewarisi izin percakapan induk dengan pembatasan alat tambahan.

Explore dan Plan melewati file CLAUDE.md Anda dan status git sesi induk untuk menjaga penelitian tetap cepat dan hemat biaya. Setiap subagent bawaan lainnya dan [subagent khusus](#configure-subagents) memuat keduanya. Untuk rincian lengkap tentang apa yang mencapai subagent, lihat [apa yang dimuat saat startup](#what-loads-at-startup).

<Tabs>
  <Tab title="Explore">
    Agen cepat yang dioptimalkan hanya-baca untuk mencari dan menganalisis basis kode.

    * **Model**: mewarisi dari percakapan utama, dibatasi pada Opus di Claude API, jadi Explore tidak pernah berjalan pada model yang lebih mahal daripada yang sudah Anda pilih untuk sesi
    * **Tools**: alat hanya-baca; Write dan Edit ditolak
    * **Purpose**: penemuan file, pencarian kode, eksplorasi basis kode

    Mulai dari v2.1.198, Explore mewarisi model percakapan utama alih-alih selalu berjalan pada Haiku. Di Claude API, model yang diwarisi dibatasi pada Opus: percakapan utama pada tingkat yang lebih tinggi menjalankan Explore pada Opus, dan percakapan utama pada Sonnet atau Haiku menjalankan Explore pada model yang sama. Di penyedia lain apa pun, seperti [Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry, atau Claude Platform on AWS](/docs/id/third-party-integrations), Explore mewarisi model percakapan utama secara langsung.

    [User atau project subagent](#choose-the-subagent-scope) bernama `Explore` menggantikan yang bawaan dan menyimpan bidang `model` miliknya sendiri, jadi tentukan satu dengan `model: haiku` untuk menjaga eksplorasi pada model dengan biaya lebih rendah.

    Claude mendelegasikan ke Explore ketika perlu mencari atau memahami basis kode tanpa membuat perubahan. Ini menjaga hasil eksplorasi di luar konteks percakapan utama Anda.

    Saat memanggil Explore, Claude menentukan tingkat ketelitian: **quick** untuk pencarian yang ditargetkan, **medium** untuk eksplorasi seimbang, atau **very thorough** untuk analisis komprehensif.
  </Tab>

  <Tab title="Plan">
    Agen penelitian yang digunakan selama [plan mode](/docs/id/permission-modes#analyze-before-you-edit-with-plan-mode) untuk mengumpulkan konteks sebelum menyajikan rencana.

    * **Model**: mewarisi dari percakapan utama
    * **Tools**: alat hanya-baca; Write dan Edit ditolak
    * **Purpose**: penelitian basis kode untuk perencanaan

    Ketika Anda dalam plan mode dan Claude perlu memahami basis kode Anda, Claude mendelegasikan penelitian ke subagent Plan sehingga output eksplorasi tetap dalam jendela konteks terpisah sementara percakapan utama tetap hanya-baca.
  </Tab>

  <Tab title="General-purpose">
    Agen yang mampu untuk tugas kompleks multi-langkah yang memerlukan eksplorasi dan tindakan.

    * **Model**: mewarisi dari percakapan utama
    * **Tools**: semua alat
    * **Purpose**: penelitian kompleks, operasi multi-langkah, modifikasi kode

    Claude mendelegasikan ke general-purpose ketika tugas memerlukan eksplorasi dan modifikasi, penalaran kompleks untuk menafsirkan hasil, atau beberapa langkah yang saling bergantung.
  </Tab>

  <Tab title="Other">
    Claude Code mencakup agen pembantu tambahan untuk tugas tertentu. Ini biasanya dipanggil secara otomatis, jadi Anda tidak perlu menggunakannya secara langsung.

    | Agen              | Model  | Kapan Claude menggunakannya                                                  |
    | :---------------- | :----- | :--------------------------------------------------------------------------- |
    | statusline-setup  | Sonnet | Ketika Anda menjalankan `/statusline` untuk mengonfigurasi baris status Anda |
    | claude-code-guide | Haiku  | Ketika Anda mengajukan pertanyaan tentang fitur Claude Code                  |
  </Tab>
</Tabs>

Subagent bawaan terdaftar secara default dalam sesi interaktif. Untuk membatasi mereka:

* Untuk memblokir tipe bawaan tertentu, tambahkan ke `permissions.deny` seperti yang ditunjukkan dalam [Nonaktifkan subagent tertentu](#disable-specific-subagents).
* Untuk mencegah Claude mendelegasikan ke subagent apa pun, tolak alat `Agent` itu sendiri dengan [`permissions.deny`](/docs/id/permissions#tool-specific-permission-rules).
* Untuk menghapus hanya subagent bawaan `Explore` dan `Plan`, atur [`CLAUDE_CODE_DISABLE_EXPLORE_PLAN_AGENTS=1`](/docs/id/env-vars). Claude membaca dan mengeksplorasi file secara langsung alih-alih mendelegasikan ke mereka. Memerlukan Claude Code v2.1.198 atau lebih baru.
* Dalam [mode non-interaktif](/docs/id/headless) dan [Agent SDK](/docs/id/agent-sdk/overview), atur [`CLAUDE_AGENT_SDK_DISABLE_BUILTIN_AGENTS=1`](/docs/id/env-vars) untuk menghapus semua tipe bawaan dan menyediakan hanya milik Anda sendiri.

Selain subagent bawaan ini, Anda dapat membuat subagent Anda sendiri dengan prompt khusus, pembatasan alat, mode izin, hooks, dan skills. Bagian berikut menunjukkan cara memulai dan menyesuaikan subagent.

<h2 id="quickstart-create-your-first-subagent">
  Quickstart: buat subagent pertama Anda
</h2>

Subagent adalah file Markdown dengan frontmatter YAML. Untuk membuat satu, minta Claude menulisnya untuk Anda, atau [tulis file sendiri](#write-subagent-files).

Mulai dari v2.1.198, perintah `/agents` tidak lagi membuka wizard pembuatan interaktif; menjalankannya mencetak pengingat untuk meminta Claude atau mengedit `.claude/agents/` secara langsung. File subagent, bidang frontmatter, dan lokasi `.claude/agents/` dan `~/.claude/agents/` tidak berubah; hanya wizard terminal yang dihapus.

Panduan ini membuat subagent tingkat pengguna yang meninjau kode dan menyarankan perbaikan.

<Steps>
  <Step title="Minta Claude membuat subagent">
    Di Claude Code, jelaskan subagent yang Anda inginkan dan di mana menyimpannya:

    ```text wrap theme={null}
    Create a personal code-improver subagent in ~/.claude/agents/ that scans
    files and suggests improvements for readability, performance, and best
    practices. It should explain each issue, show the current code, and
    provide an improved version. Make it read-only and have it use Sonnet.
    ```

    Claude menulis file dengan `name`, `description`, daftar `tools`, `model`, dan system prompt.
  </Step>

  <Step title="Tinjau file">
    Buka `~/.claude/agents/code-improver.md` dan konfirmasi frontmatter sesuai dengan yang Anda minta. Hasilnya terlihat seperti ini:

    ```markdown theme={null}
    ---
    name: code-improver
    description: Scans files and suggests improvements for readability, performance, and best practices. Use after writing or modifying code.
    tools: Read, Grep, Glob
    model: sonnet
    ---

    You are a code improvement specialist. For each issue you find, explain
    the problem, show the current code, and provide an improved version.
    ```

    Karena file berada di `~/.claude/agents/`, subagent tersedia di setiap proyek di mesin Anda. Untuk membatasi ke satu proyek saja, pindahkan ke direktori `.claude/agents/` proyek tersebut. [Pilih cakupan subagent](#choose-the-subagent-scope) membandingkan keduanya.
  </Step>

  <Step title="Coba">
    Minta Claude mendelegasikan ke subagent baru:

    ```text wrap theme={null}
    Use the code-improver agent to suggest improvements in this project
    ```

    Claude mendelegasikan ke subagent baru Anda, yang memindai basis kode dan mengembalikan saran perbaikan.

    Jika Claude tidak dapat menemukan subagent baru, mulai ulang Claude Code dan coba lagi. Ini terjadi hanya ketika `~/.claude/agents/` tidak ada sebelum sesi dimulai, karena sesi yang berjalan tidak mendeteksi direktori `agents` yang baru dibuat.
  </Step>
</Steps>

Anda sekarang memiliki subagent yang dapat Anda gunakan di proyek apa pun di mesin Anda untuk menganalisis basis kode dan menyarankan perbaikan.

Anda juga dapat menulis file subagent secara manual, mendefinisikannya melalui flag CLI, atau mendistribusikannya melalui plugins. Bagian berikut mencakup semua opsi konfigurasi.

<Note>
  Pada Claude Code v2.1.197 dan lebih awal, `/agents` membuka wizard interaktif dengan tab **Running** yang mencantumkan subagent aktif dan tab **Library** untuk membuat, mengedit, dan menghapusnya.&#x20;
</Note>

<h2 id="configure-subagents">
  Konfigurasi subagent
</h2>

Lokasi file subagent menentukan siapa yang dapat mengaksesnya, dan frontmatter-nya menentukan apa yang dapat dilakukannya. Bagian ini mencakup di mana file subagent berada dan setiap bidang yang didukungnya.

<h3 id="choose-the-subagent-scope">
  Pilih cakupan subagent
</h3>

Simpan file subagent di lokasi berbeda tergantung pada cakupan. Ketika beberapa subagent berbagi nama yang sama, Claude Code menggunakan yang dari lokasi dengan prioritas lebih tinggi.

| Lokasi                     | Cakupan                  | Prioritas     | Cara membuat                                           |
| :------------------------- | :----------------------- | :------------ | :----------------------------------------------------- |
| Pengaturan terkelola       | Seluruh organisasi       | 1 (tertinggi) | Digunakan melalui [pengaturan terkelola](/docs/id/settings) |
| Flag CLI `--agents`        | Sesi saat ini            | 2             | Lewatkan JSON saat meluncurkan Claude Code             |
| `.claude/agents/`          | Proyek saat ini          | 3             | Tanyakan Claude, atau buat file secara manual          |
| `~/.claude/agents/`        | Semua proyek Anda        | 4             | Tanyakan Claude, atau buat file secara manual          |
| Direktori `agents/` plugin | Tempat plugin diaktifkan | 5 (terendah)  | Diinstal dengan [plugins](/docs/id/plugins)                 |

**Subagent proyek** (`.claude/agents/`) ideal untuk subagent khusus untuk basis kode. Periksa mereka ke kontrol versi sehingga tim Anda dapat menggunakannya dan meningkatkannya secara kolaboratif.

Subagent proyek ditemukan dengan berjalan naik dari direktori kerja saat ini, sehingga setiap `.claude/agents/` antara sana dan akar repositori dipindai. Sejak v2.1.178, ketika lebih dari satu direktori bersarang ini mendefinisikan `name` yang sama, Claude Code menggunakan definisi yang paling dekat dengan direktori kerja.

Direktori yang ditambahkan dengan `--add-dir` juga dipindai: folder `.claude/agents/` di dalam direktori yang ditambahkan dimuat bersama subagent proyek. Lihat [Direktori tambahan](/docs/id/permissions#additional-directories-grant-file-access-not-configuration) untuk jenis konfigurasi lain mana yang dimuat dari `--add-dir`. Untuk berbagi subagent di seluruh proyek tanpa `--add-dir`, gunakan `~/.claude/agents/` atau [plugin](/docs/id/plugins).

**Subagent pengguna** (`~/.claude/agents/`) adalah subagent pribadi yang tersedia di semua proyek Anda.

Claude Code memindai `.claude/agents/` dan `~/.claude/agents/` secara rekursif, sehingga Anda dapat mengorganisir definisi ke dalam subfolder seperti `agents/review/` atau `agents/research/`. Jalur subdirektori tidak mempengaruhi cara subagent diidentifikasi atau dipanggil, karena identitas hanya berasal dari bidang frontmatter `name`.

Jaga nilai `name` tetap unik di seluruh pohon: jika dua file dalam satu cakupan mendeklarasikan nama yang sama, Claude Code menyimpan satu dan membuang yang lain tanpa peringatan. Direktori `agents/` plugin juga dipindai secara rekursif. Tidak seperti cakupan proyek dan pengguna, subfolder di dalam direktori `agents/` plugin menjadi bagian dari [pengenal yang dibatasi cakupan](#invoke-subagents-explicitly): file di `agents/review/security.md` dalam plugin `my-plugin` terdaftar sebagai `my-plugin:review:security`.

Pemeriksaan setup [`/doctor`](/docs/id/commands#all-commands) melaporkan file dalam direktori yang sama yang berbagi nama dan menyarankan untuk mengganti nama atau menghapus semua kecuali satu. Sebelum v2.1.205, `/doctor` membuka layar diagnostik yang mencantumkan duplikat dan menunjukkan definisi mana yang aktif.

**Subagent yang ditentukan CLI** dilewatkan sebagai JSON saat meluncurkan Claude Code. Mereka hanya ada untuk sesi itu dan tidak disimpan ke disk, menjadikannya berguna untuk pengujian cepat atau skrip otomasi. Anda dapat mendefinisikan beberapa subagent dalam satu panggilan `--agents`:

<Tabs>
  <Tab title="macOS, Linux, WSL">
    ```bash theme={null}
    claude --agents '{
      "code-reviewer": {
        "description": "Expert code reviewer. Use proactively after code changes.",
        "prompt": "You are a senior code reviewer. Focus on code quality, security, and best practices.",
        "tools": ["Read", "Grep", "Glob", "Bash"],
        "model": "sonnet"
      },
      "debugger": {
        "description": "Debugging specialist for errors and test failures.",
        "prompt": "You are an expert debugger. Analyze errors, identify root causes, and provide fixes."
      }
    }'
    ```
  </Tab>

  <Tab title="Windows PowerShell">
    ```powershell theme={null}
    claude --agents @'
    {
      "code-reviewer": {
        "description": "Expert code reviewer. Use proactively after code changes.",
        "prompt": "You are a senior code reviewer. Focus on code quality, security, and best practices.",
        "tools": ["Read", "Grep", "Glob", "Bash"],
        "model": "sonnet"
      },
      "debugger": {
        "description": "Debugging specialist for errors and test failures.",
        "prompt": "You are an expert debugger. Analyze errors, identify root causes, and provide fixes."
      }
    }
    '@
    ```
  </Tab>
</Tabs>

Flag `--agents` menerima JSON dengan [frontmatter](#supported-frontmatter-fields) yang sama bidang subagent berbasis file: `description`, `prompt`, `tools`, `disallowedTools`, `model`, `permissionMode`, `mcpServers`, `hooks`, `maxTurns`, `skills`, `initialPrompt`, `memory`, `effort`, `background`, `isolation`, dan `color`. Gunakan `prompt` untuk prompt sistem, setara dengan badan markdown dalam subagent berbasis file.

**Subagent terkelola** digunakan oleh administrator organisasi. Tempatkan file markdown dalam `.claude/agents/` di dalam [direktori pengaturan terkelola](/docs/id/settings#settings-files), menggunakan format frontmatter yang sama dengan subagent proyek dan pengguna. Definisi terkelola mengambil alih subagent proyek dan pengguna dengan nama yang sama.

**Subagent plugin** berasal dari [plugins](/docs/id/plugins) yang telah Anda instal. Mereka dimuat bersama subagent khusus Anda dan muncul dalam typeahead @-mention di bawah nama yang dibatasi cakupan mereka. Lihat [referensi komponen plugin](/docs/id/plugins-reference#agents) untuk detail tentang membuat subagent plugin.

<Note>
  Untuk alasan keamanan, subagent plugin tidak mendukung bidang frontmatter `hooks`, `mcpServers`, atau `permissionMode`. Bidang-bidang ini diabaikan saat memuat agen dari plugin. Jika Anda membutuhkannya, salin file agen ke dalam `.claude/agents/` atau `~/.claude/agents/`. Anda juga dapat menambahkan aturan ke [`permissions.allow`](/docs/id/settings#permission-settings) dalam `settings.json` atau `settings.local.json`, tetapi aturan-aturan ini berlaku untuk seluruh sesi, bukan hanya subagent plugin.
</Note>

Definisi subagent dari salah satu cakupan ini juga tersedia untuk [tim agen](/docs/id/agent-teams#use-subagent-definitions-for-teammates): saat menelurkan rekan kerja, Anda dapat mereferensikan jenis subagent dan rekan kerja menggunakan `tools` dan `model`-nya, dengan badan definisi ditambahkan ke prompt sistem rekan kerja sebagai instruksi tambahan. Lihat [tim agen](/docs/id/agent-teams#use-subagent-definitions-for-teammates) untuk bidang frontmatter mana yang berlaku pada jalur itu.

<h3 id="write-subagent-files">
  Tulis file subagent
</h3>

File subagent menggunakan frontmatter YAML untuk konfigurasi, diikuti oleh prompt sistem dalam Markdown:

<Note>
  Claude Code memantau `~/.claude/agents/` dan `.claude/agents/`. Ketika Anda menambah atau mengedit file subagent di disk, atau meminta Claude untuk menulis satu untuk Anda, Claude Code mendeteksi perubahan dalam beberapa detik dan delegasi berikutnya menggunakan definisi yang diperbarui, tanpa perlu restart.

  Dua kasus masih memerlukan restart:

  * Pemantau hanya mencakup direktori yang ada ketika sesi dimulai, jadi setelah membuat file agen pertama cakupan dalam direktori `agents` baru, restart untuk memuatnya.
  * Sesi yang dimulai dengan `--disable-slash-commands` tidak memantau direktori ini sama sekali.
</Note>

```markdown theme={null}
---
name: code-reviewer
description: Reviews code for quality and best practices
tools: Read, Glob, Grep
model: sonnet
---

You are a code reviewer. When invoked, analyze the code and provide
specific, actionable feedback on quality, security, and best practices.
```

Frontmatter mendefinisikan metadata dan konfigurasi subagent. Badan menjadi prompt sistem yang memandu perilaku subagent. Subagent menerima hanya prompt sistem ini (ditambah detail lingkungan dasar seperti direktori kerja), bukan prompt sistem Claude Code lengkap.

Dalam [mode non-interaktif](/docs/id/headless), flag [`--append-subagent-system-prompt`](/docs/id/cli-reference#cli-flags) menambahkan teks yang Anda berikan ke akhir prompt sistem setiap subagent, termasuk subagent bersarang. Memerlukan Claude Code v2.1.205 atau lebih baru.

Subagent dimulai di direktori kerja saat ini percakapan utama. Dalam subagent, perintah `cd` tidak bertahan antara panggilan alat Bash atau PowerShell dan tidak mempengaruhi direktori kerja percakapan utama. Untuk memberikan subagent salinan repositori yang terisolasi sebagai gantinya, atur [`isolation: worktree`](#supported-frontmatter-fields).

Subagent dengan `isolation: worktree` menjalankan perintah Bash dan PowerShell-nya di dalam worktree-nya. Perintah yang direktori kerjanya diselesaikan ke checkout utama Anda, misalnya karena direktori worktree dihapus saat subagent berjalan, gagal dengan kesalahan. Sebelum v2.1.203, perintah seperti itu dapat berjalan di checkout utama.

<h4 id="supported-frontmatter-fields">
  Bidang frontmatter yang didukung
</h4>

Bidang berikut dapat digunakan dalam frontmatter YAML. Hanya `name` dan `description` yang diperlukan.

| Bidang            | Diperlukan | Deskripsi                                                                                                                                                                                                                                                                                                                                                             |
| :---------------- | :--------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`            | Ya         | Pengenal unik menggunakan huruf kecil dan tanda hubung. [Hooks](/docs/id/hooks#subagentstart) menerima nilai ini sebagai `agent_type`. Nama file tidak harus cocok                                                                                                                                                                                                         |
| `description`     | Ya         | Kapan Claude harus mendelegasikan ke subagent ini                                                                                                                                                                                                                                                                                                                     |
| `tools`           | Tidak      | [Alat](#available-tools) yang dapat digunakan subagent. Mewarisi semua alat jika dihilangkan. Jika tidak ada entri dalam daftar yang diselesaikan ke alat, subagent gagal diluncurkan dengan kesalahan yang menyebutkan entri. Untuk memuat Skills ke dalam konteks, gunakan bidang `skills` daripada mencantumkan `Skill` di sini                                    |
| `disallowedTools` | Tidak      | Alat untuk ditolak, dihapus dari daftar yang diwarisi atau ditentukan                                                                                                                                                                                                                                                                                                 |
| `model`           | Tidak      | [Model](#choose-a-model) untuk digunakan: `sonnet`, `opus`, `haiku`, `fable`, ID model lengkap (misalnya, `claude-opus-4-8`), atau `inherit`. Default ke `inherit`                                                                                                                                                                                                    |
| `permissionMode`  | Tidak      | [Mode izin](#permission-modes): `default`, `acceptEdits`, `auto`, `dontAsk`, `bypassPermissions`, `plan`, atau `manual` sebagai alias untuk `default`. Alias `manual` memerlukan Claude Code v2.1.200 atau lebih baru. Diabaikan untuk [subagent plugin](#choose-the-subagent-scope)                                                                                  |
| `maxTurns`        | Tidak      | Jumlah maksimum putaran agentic sebelum subagent berhenti                                                                                                                                                                                                                                                                                                             |
| `skills`          | Tidak      | [Skills](/docs/id/skills) untuk dimuat ke dalam konteks subagent saat startup. Konten skill lengkap disuntikkan, bukan hanya deskripsi. Subagent masih dapat memanggil project, user, dan plugin skills yang tidak tercantum melalui alat Skill                                                                                                                            |
| `mcpServers`      | Tidak      | [MCP servers](/docs/id/mcp) tersedia untuk subagent ini. Setiap entri adalah nama server yang mereferensikan server yang sudah dikonfigurasi (misalnya, `"slack"`) atau definisi inline dengan nama server sebagai kunci dan [konfigurasi MCP server](/docs/id/mcp#installing-mcp-servers) lengkap sebagai nilai. Diabaikan untuk [subagent plugin](#choose-the-subagent-scope) |
| `hooks`           | Tidak      | [Lifecycle hooks](#define-hooks-for-subagents) yang dibatasi pada subagent ini. Diabaikan untuk [subagent plugin](#choose-the-subagent-scope)                                                                                                                                                                                                                         |
| `memory`          | Tidak      | [Cakupan memori persisten](#enable-persistent-memory): `user`, `project`, atau `local`. Memungkinkan pembelajaran lintas sesi                                                                                                                                                                                                                                         |
| `background`      | Tidak      | Atur ke `true` untuk selalu menjalankan subagent ini sebagai [background task](#run-subagents-in-foreground-or-background), bahkan ketika Claude memerlukan hasilnya segera. Ketika tidak diatur, Claude memilih, dan sejak v2.1.198 itu menjalankan subagent di latar belakang secara default                                                                        |
| `effort`          | Tidak      | Tingkat usaha ketika subagent ini aktif. Menimpa tingkat usaha sesi. Default: mewarisi dari sesi. Opsi: `low`, `medium`, `high`, `xhigh`, `max`; tingkat yang tersedia tergantung pada model                                                                                                                                                                          |
| `isolation`       | Tidak      | Atur ke `worktree` untuk menjalankan subagent dalam [git worktree](/docs/id/worktrees) sementara, memberikannya salinan repositori yang terisolasi yang bercabang secara default dari [cabang default](/docs/id/worktrees#choose-the-base-branch) Anda daripada `HEAD` sesi induk. Worktree secara otomatis dibersihkan jika subagent tidak membuat perubahan                   |
| `color`           | Tidak      | Warna tampilan untuk subagent dalam daftar tugas dan transkrip. Menerima `red`, `blue`, `green`, `yellow`, `purple`, `orange`, `pink`, atau `cyan`                                                                                                                                                                                                                    |
| `initialPrompt`   | Tidak      | Auto-submitted sebagai putaran pengguna pertama ketika agen ini berjalan sebagai agen sesi utama (melalui `--agent` atau pengaturan `agent`). [Commands](/docs/id/commands) dan [skills](/docs/id/skills) diproses. Ditambahkan di depan prompt yang disediakan pengguna apa pun                                                                                                |

<h3 id="choose-a-model">
  Pilih model
</h3>

Bidang `model` mengontrol [model AI](/docs/id/model-config) mana yang digunakan subagent:

* **Alias model**: Gunakan salah satu alias yang tersedia: `sonnet`, `opus`, `haiku`, atau `fable`
* **ID model lengkap**: Gunakan ID model lengkap seperti `claude-opus-4-8` atau `claude-sonnet-5`. Menerima nilai yang sama dengan flag `--model`
* **inherit**: Gunakan model yang sama dengan percakapan utama
* **Dihilangkan**: Jika tidak ditentukan, default ke `inherit` (menggunakan model yang sama dengan percakapan utama)

Ketika Claude memanggil subagent, Claude juga dapat melewatkan parameter `model` untuk invokasi spesifik itu. Claude Code menyelesaikan model subagent dalam urutan ini:

1. Variabel lingkungan [`CLAUDE_CODE_SUBAGENT_MODEL`](/docs/id/model-config#environment-variables), jika diatur ke alias model atau ID model
2. Parameter `model` per-invokasi
3. Frontmatter `model` definisi subagent
4. Model percakapan utama

Sejak v2.1.196, mengatur `CLAUDE_CODE_SUBAGENT_MODEL` ke `inherit` sama dengan membiarkannya tidak diatur: resolusi berlanjut dengan parameter `model` per-invokasi, kemudian frontmatter. Dalam versi sebelumnya, `inherit` memaksa subagent ke model percakapan utama dan mengabaikan kedua sumber itu.

Claude Code memeriksa variabel lingkungan, parameter per-invokasi, dan nilai frontmatter terhadap daftar allowlist [`availableModels`](/docs/id/model-config#restrict-model-selection) organisasi Anda. Nilai yang diselesaikan ke model yang dikecualikan dilewati dan subagent berjalan pada model yang diwarisi sebagai gantinya.

Sejak v2.1.198, subagent juga mewarisi konfigurasi [extended thinking](/docs/id/model-config#extended-thinking) percakapan utama: jika thinking aktif dalam sesi Anda, itu aktif untuk subagent, dan jika itu mati, itu tetap mati. Tidak ada pengaturan thinking per-subagent. Sebelum v2.1.198, subagent berjalan dengan extended thinking dinonaktifkan terlepas dari pengaturan percakapan utama.

<h3 id="control-subagent-capabilities">
  Kontrol kemampuan subagent
</h3>

Anda dapat mengontrol apa yang dapat dilakukan subagent melalui akses alat, mode izin, dan aturan bersyarat.

<h4 id="available-tools">
  Alat yang tersedia
</h4>

Subagent mewarisi [alat internal](/docs/id/tools-reference) dan alat MCP yang tersedia dalam percakapan utama secara default. Alat berikut bergantung pada UI percakapan utama atau status sesi dan tidak tersedia untuk subagent, bahkan ketika tercantum dalam bidang `tools`:

* `AskUserQuestion`
* `EnterPlanMode`
* `ExitPlanMode`, kecuali [`permissionMode`](#permission-modes) subagent adalah `plan`
* `ScheduleWakeup`
* `WaitForMcpServers`

Untuk membatasi alat, gunakan bidang `tools` sebagai allowlist atau bidang `disallowedTools` sebagai denylist. Contoh ini menggunakan `tools` untuk secara eksklusif mengizinkan Read, Grep, Glob, dan Bash. Subagent tidak dapat mengedit file, menulis file, atau menggunakan alat MCP apa pun:

```yaml theme={null}
---
name: safe-researcher
description: Research agent with restricted capabilities
tools: Read, Grep, Glob, Bash
---
```

Contoh ini menggunakan `disallowedTools` untuk mewarisi setiap alat dari percakapan utama kecuali Write dan Edit. Subagent menyimpan Bash, alat MCP, dan semuanya yang lain:

```yaml theme={null}
---
name: no-writes
description: Inherits every tool except file writes
disallowedTools: Write, Edit
---
```

Jika keduanya diatur, `disallowedTools` diterapkan terlebih dahulu, kemudian `tools` diselesaikan terhadap kumpulan yang tersisa. Alat yang tercantum di keduanya dihapus.

Ketika tidak ada dalam daftar `tools` yang diselesaikan ke alat, misalnya karena setiap entri salah eja atau menyebutkan alat yang tidak tersedia untuk subagent, Claude Code menolak untuk meluncurkan subagent dan alat Agent mengembalikan kesalahan yang menyebutkan entri yang tidak diselesaikan. Sebelum v2.1.208, subagent itu diluncurkan tanpa alat dan dapat mengembalikan hasil yang kosong atau membingungkan.

Kedua bidang menerima pola tingkat server MCP selain nama alat yang tepat: `mcp__<server>` atau `mcp__<server>__*` memberikan atau menghapus setiap alat dari server bernama. Dalam `disallowedTools`, `mcp__*` juga menghapus setiap alat MCP dari server apa pun. Contoh ini menghapus setiap alat dari server MCP `github` sambil menyimpan alat dari server lain dan setiap alat bawaan:

```yaml theme={null}
---
name: local-only
description: Inherits every tool except those from the github MCP server
disallowedTools: mcp__github
---
```

<h4 id="restrict-which-subagents-can-be-spawned">
  Batasi subagent mana yang dapat dihasilkan
</h4>

Ketika agen berjalan sebagai thread utama dengan `claude --agent`, agen dapat menelurkan subagent menggunakan alat Agent. Untuk membatasi jenis subagent mana yang dapat dihasilkan, gunakan sintaks `Agent(agent_type)` dalam bidang `tools`.

<Note>Dalam versi 2.1.63, alat Task diganti nama menjadi Agent. Referensi `Task(...)` yang ada dalam pengaturan dan definisi agen masih berfungsi sebagai alias.</Note>

```yaml theme={null}
---
name: coordinator
description: Coordinates work across specialized agents
tools: Agent(worker, researcher), Read, Bash
---
```

Ini adalah allowlist: hanya subagent `worker` dan `researcher` yang dapat dihasilkan. Jika agen mencoba menelurkan jenis lain, permintaan gagal dan agen hanya melihat jenis yang diizinkan dalam promptnya. Untuk memblokir agen tertentu sambil mengizinkan semua yang lain, gunakan [`permissions.deny`](#disable-specific-subagents) sebagai gantinya.

Untuk mengizinkan penelur subagent apa pun tanpa pembatasan, gunakan `Agent` tanpa tanda kurung:

```yaml theme={null}
tools: Agent, Read, Bash
```

Jika `Agent` dihilangkan dari daftar `tools` sepenuhnya, agen tidak dapat menelurkan subagent apa pun.

Sintaks allowlist `Agent(agent_type)` hanya berlaku untuk agen yang berjalan sebagai thread utama dengan `claude --agent`. Dalam definisi subagent, mencantumkan `Agent` dalam `tools` memungkinkan subagent itu untuk [menelurkan subagent bersarang](#spawn-nested-subagents), tetapi daftar jenis apa pun di dalam tanda kurung diabaikan.

<h4 id="scope-mcp-servers-to-a-subagent">
  Cakupan MCP servers ke subagent
</h4>

Gunakan bidang `mcpServers` untuk memberikan subagent akses ke [MCP](/docs/id/mcp) servers yang tidak tersedia dalam percakapan utama. Server inline yang ditentukan di sini terhubung saat subagent dimulai dan terputus saat selesai. Referensi string berbagi koneksi sesi induk.

<Note>
  Bidang `mcpServers` berlaku dalam kedua konteks di mana file agen dapat berjalan:

  * Sebagai subagent, dihasilkan melalui alat Agent atau @-mention
  * Sebagai sesi utama, diluncurkan dengan [`--agent`](#invoke-subagents-explicitly) atau pengaturan `agent`

  Ketika agen adalah sesi utama, definisi server inline terhubung saat startup bersama server dari [`.mcp.json`](/docs/id/mcp) dan file pengaturan.
</Note>

Setiap entri dalam daftar adalah definisi server inline atau string yang mereferensikan MCP server yang sudah dikonfigurasi dalam sesi Anda:

```yaml theme={null}
---
name: browser-tester
description: Tests features in a real browser using Playwright
mcpServers:
  # Inline definition: scoped to this subagent only
  - playwright:
      type: stdio
      command: npx
      args: ["-y", "@playwright/mcp@latest"]
  # Reference by name: reuses an already-configured server
  - github
---

Use the Playwright tools to navigate, screenshot, and interact with pages.
```

Definisi inline menggunakan skema yang sama dengan entri server `.mcp.json`, dikunci dengan nama server, dan mendukung jenis `stdio`, `http`, `sse`, dan `ws`.

Untuk menjaga MCP server di luar percakapan utama sepenuhnya dan menghindari deskripsi alatnya mengonsumsi konteks di sana, tentukan secara inline di sini daripada di `.mcp.json`. Subagent mendapatkan alat; percakapan induk tidak.

Sejak v2.1.153, pembatasan MCP yang berlaku untuk sesi utama juga mencakup server yang dideklarasikan dalam frontmatter subagent:

* [`--strict-mcp-config`](/docs/id/cli-reference) dan [`--bare`](/docs/id/cli-reference)
* [Konfigurasi MCP terkelola Enterprise](/docs/id/managed-mcp)
* [`allowedMcpServers` dan `deniedMcpServers` policies](/docs/id/managed-mcp#policy-based-control-with-allowlists-and-denylists)

Ketika salah satu dari ini memblokir server, Claude Code melewatinya dan menampilkan peringatan yang menyebutkan server yang diblokir.

Pembatasan pengaturan terkelola berlaku untuk setiap subagent terlepas dari cara pendefinisiannya. `--strict-mcp-config` tidak memfilter server yang Anda lewatkan secara inline melalui `--agents` atau opsi SDK `agents`, karena itu adalah input pemanggil eksplisit.

<h4 id="permission-modes">
  Mode izin
</h4>

Bidang `permissionMode` mengontrol bagaimana subagent menangani prompt izin. Subagent mewarisi konteks izin dari percakapan utama dan dapat menimpa mode, kecuali ketika mode induk mengambil alih seperti yang dijelaskan di bawah.

| Mode                | Perilaku                                                                                                                                                                                                                                                                                                                                             |
| :------------------ | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `default`           | Pemeriksaan izin standar dengan prompt                                                                                                                                                                                                                                                                                                               |
| `acceptEdits`       | Auto-terima edit file dan perintah sistem file umum untuk jalur di direktori kerja atau `additionalDirectories`                                                                                                                                                                                                                                      |
| `auto`              | [Auto mode](/docs/id/permission-modes#eliminate-prompts-with-auto-mode): pengklasifikasi latar belakang meninjau perintah dan penulisan direktori yang dilindungi                                                                                                                                                                                         |
| `dontAsk`           | Auto-tolak prompt izin. Alat yang secara eksplisit diizinkan masih berfungsi; `AskUserQuestion`, alat konektor [organisasi Anda atur ke `ask`](/docs/id/mcp#organization-controls-on-connector-tools), dan alat MCP yang ditandai [`requiresUserInteraction`](/docs/id/mcp#require-approval-for-a-specific-tool) ditolak bahkan jika Anda telah mengizinkannya |
| `bypassPermissions` | Lewati prompt izin                                                                                                                                                                                                                                                                                                                                   |
| `plan`              | Plan mode (eksplorasi hanya-baca)                                                                                                                                                                                                                                                                                                                    |

<Warning>
  Gunakan `bypassPermissions` dengan hati-hati. Ini melewati prompt izin, memungkinkan subagent untuk menjalankan operasi tanpa persetujuan, termasuk penulisan ke `.git`, `.config/git`, `.claude`, `.vscode`, `.idea`, `.husky`, `.cargo`, `.devcontainer`, `.yarn`, dan `.mvn`.

  Aturan [`ask` eksplisit](/docs/id/permissions#manage-permissions), alat konektor [organisasi Anda atur ke `ask`](/docs/id/mcp#organization-controls-on-connector-tools), alat MCP yang ditandai [`requiresUserInteraction`](/docs/id/mcp#require-approval-for-a-specific-tool), dan penghapusan direktori root dan home seperti `rm -rf /` masih meminta. Lihat [permission modes](/docs/id/permission-modes#skip-all-checks-with-bypasspermissions-mode) untuk detail.
</Warning>

Jika induk menggunakan `bypassPermissions` atau `acceptEdits`, ini mengambil alih dan tidak dapat ditimpa. Jika induk menggunakan [auto mode](/docs/id/permission-modes#eliminate-prompts-with-auto-mode), subagent mewarisi auto mode dan `permissionMode` apa pun dalam frontmatternya diabaikan: pengklasifikasi mengevaluasi panggilan alat subagent dengan aturan blok dan izin yang sama dengan sesi induk.

<h4 id="preload-skills-into-subagents">
  Preload skills ke dalam subagent
</h4>

Gunakan bidang `skills` untuk menyuntikkan konten skill ke dalam konteks subagent saat startup. Ini memberikan subagent pengetahuan domain tanpa memerlukan penemuan dan pemuatan skills selama eksekusi.

```yaml theme={null}
---
name: api-developer
description: Implement API endpoints following team conventions
skills:
  - api-conventions
  - error-handling-patterns
---

Implement API endpoints. Follow the conventions and patterns from the preloaded skills.
```

Konten lengkap setiap skill yang tercantum disuntikkan ke dalam konteks subagent saat startup. Bidang ini mengontrol skill mana yang dimuat sebelumnya, bukan skill mana yang dapat diakses subagent: tanpanya, subagent masih dapat menemukan dan memanggil project, user, dan plugin skills melalui alat Skill selama eksekusi. Untuk mencegah subagent memanggil skills sama sekali, hilangkan `Skill` dari daftar [`tools`](#available-tools) atau tambahkan ke `disallowedTools`.

Anda tidak dapat preload skills yang menetapkan [`disable-model-invocation: true`](/docs/id/skills#control-who-invokes-a-skill), karena preloading menarik dari set skills yang sama yang dapat diinvokasi Claude. Jika skill yang tercantum hilang atau dinonaktifkan, Claude Code melewatinya dan mencatat peringatan ke debug log.

<Note>
  Ini adalah kebalikan dari [menjalankan skill dalam subagent](/docs/id/skills#run-skills-in-a-subagent). Dengan `skills` dalam subagent, subagent mengontrol prompt sistem dan memuat konten skill. Dengan `context: fork` dalam skill, konten skill disuntikkan ke dalam agen yang Anda tentukan. Keduanya menggunakan sistem yang mendasar yang sama.
</Note>

<h4 id="enable-persistent-memory">
  Aktifkan memori persisten
</h4>

Bidang `memory` memberikan subagent direktori persisten yang bertahan di seluruh percakapan. Subagent menggunakan direktori ini untuk membangun pengetahuan seiring waktu, seperti pola basis kode, wawasan debugging, dan keputusan arsitektur.

```yaml theme={null}
---
name: code-reviewer
description: Reviews code for quality and best practices
memory: user
---

You are a code reviewer. As you review code, update your agent memory with
patterns, conventions, and recurring issues you discover.
```

Pilih cakupan berdasarkan seberapa luas memori harus diterapkan:

| Cakupan   | Lokasi                                        | Gunakan ketika                                                                           |
| :-------- | :-------------------------------------------- | :--------------------------------------------------------------------------------------- |
| `user`    | `~/.claude/agent-memory/<name-of-agent>/`     | subagent harus mengingat pembelajaran di seluruh semua proyek                            |
| `project` | `.claude/agent-memory/<name-of-agent>/`       | pengetahuan subagent spesifik proyek dan dapat dibagikan melalui kontrol versi           |
| `local`   | `.claude/agent-memory-local/<name-of-agent>/` | pengetahuan subagent spesifik proyek tetapi tidak boleh diperiksa ke dalam kontrol versi |

Ketika memori diaktifkan:

* Prompt sistem subagent mencakup instruksi untuk membaca dan menulis ke direktori memori.
* Prompt sistem subagent juga mencakup 200 baris pertama atau 25KB dari `MEMORY.md` dalam direktori memori, mana pun yang lebih kecil, dengan instruksi untuk mengkurasi `MEMORY.md` jika melebihi batas itu.
* Alat Read, Write, dan Edit secara otomatis diaktifkan sehingga subagent dapat mengelola file memorinya.

<h5 id="persistent-memory-tips">
  Tips memori persisten
</h5>

* `project` adalah cakupan default yang direkomendasikan. Ini membuat pengetahuan subagent dapat dibagikan melalui kontrol versi.
* Minta subagent untuk berkonsultasi dengan memorinya sebelum memulai pekerjaan: "Review PR ini, dan periksa memori Anda untuk pola yang telah Anda lihat sebelumnya."
* Minta subagent untuk memperbarui memorinya setelah menyelesaikan tugas: "Sekarang setelah Anda selesai, simpan apa yang Anda pelajari ke memori Anda." Seiring waktu, ini membangun basis pengetahuan yang membuat subagent lebih efektif.
* Sertakan instruksi memori langsung dalam file markdown subagent sehingga secara proaktif mempertahankan basis pengetahuannya sendiri:

  ```markdown theme={null}
  Update your agent memory as you discover codepaths, patterns, library
  locations, and key architectural decisions. This builds up institutional
  knowledge across conversations. Write concise notes about what you found
  and where.
  ```

<h4 id="conditional-rules-with-hooks">
  Aturan bersyarat dengan hooks
</h4>

Untuk kontrol yang lebih dinamis atas penggunaan alat, gunakan hooks `PreToolUse` untuk memvalidasi operasi sebelum dijalankan. Ini berguna ketika Anda perlu mengizinkan beberapa operasi alat sambil memblokir yang lain.

Contoh ini membuat subagent yang hanya mengizinkan kueri database hanya-baca. Hook `PreToolUse` menjalankan skrip yang ditentukan dalam `command` sebelum setiap perintah Bash dijalankan:

```yaml theme={null}
---
name: db-reader
description: Execute read-only database queries
tools: Bash
hooks:
  PreToolUse:
    - matcher: "Bash"
      hooks:
        - type: command
          command: "./scripts/validate-readonly-query.sh"
---
```

Claude Code [melewatkan input hook sebagai JSON](/docs/id/hooks#pretooluse-input) melalui stdin ke perintah hook. Skrip validasi membaca JSON ini, mengekstrak perintah Bash, dan [keluar dengan kode 2](/docs/id/hooks#exit-code-2-behavior-per-event) untuk memblokir operasi penulisan:

```bash theme={null}
#!/bin/bash
# ./scripts/validate-readonly-query.sh

INPUT=$(cat)
COMMAND=$(echo "$INPUT" | jq -r '.tool_input.command // empty')

# Block SQL write operations (case-insensitive)
if echo "$COMMAND" | grep -iE '\b(INSERT|UPDATE|DELETE|DROP|CREATE|ALTER|TRUNCATE)\b' > /dev/null; then
  echo "Blocked: Only SELECT queries are allowed" >&2
  exit 2
fi

exit 0
```

Lihat [Hook input](/docs/id/hooks#pretooluse-input) untuk skema input lengkap dan [exit codes](/docs/id/hooks#exit-code-output) untuk bagaimana kode keluar mempengaruhi perilaku. Di Windows, tulis skrip hook dalam PowerShell dan tambahkan `shell: powershell` ke entri hook seperti yang ditunjukkan dalam [menjalankan hooks dalam PowerShell](/docs/id/hooks#windows-powershell-tool).

<h4 id="disable-specific-subagents">
  Nonaktifkan subagent tertentu
</h4>

Anda dapat mencegah Claude menggunakan subagent tertentu dengan menambahkannya ke array `deny` dalam [pengaturan](/docs/id/settings#permission-settings) Anda. Gunakan format `Agent(subagent-name)` di mana `subagent-name` cocok dengan bidang nama subagent.

```json theme={null}
{
  "permissions": {
    "deny": ["Agent(Explore)", "Agent(my-custom-agent)"]
  }
}
```

Ini berfungsi untuk subagent bawaan dan khusus. Anda juga dapat menggunakan flag CLI `--disallowedTools`:

```bash theme={null}
claude --disallowedTools "Agent(Explore)"
```

Lihat [dokumentasi Permissions](/docs/id/permissions#tool-specific-permission-rules) untuk detail lebih lanjut tentang aturan izin.

<h3 id="define-hooks-for-subagents">
  Tentukan hooks untuk subagent
</h3>

Subagent dapat mendefinisikan [hooks](/docs/id/hooks) yang berjalan selama siklus hidup subagent. Ada dua cara untuk mengonfigurasi hooks:

* **Dalam frontmatter subagent**: Tentukan hooks yang hanya berjalan saat subagent tertentu itu aktif
* **Dalam `settings.json`**: Tentukan hooks yang berjalan dalam sesi utama ketika subagent dimulai atau berhenti

<h4 id="hooks-in-subagent-frontmatter">
  Hooks dalam frontmatter subagent
</h4>

Tentukan hooks langsung dalam file markdown subagent. Hooks ini hanya berjalan saat subagent spesifik itu aktif dan dibersihkan saat selesai.

<Note>
  Frontmatter hooks terjadi ketika agen dihasilkan sebagai subagent melalui alat Agent atau @-mention, dan ketika agen berjalan sebagai sesi utama melalui [`--agent`](#invoke-subagents-explicitly) atau pengaturan `agent`. Dalam kasus sesi-utama mereka berjalan bersama hook apa pun yang ditentukan dalam [`settings.json`](/docs/id/hooks).
</Note>

Semua [hook events](/docs/id/hooks#hook-events) didukung. Peristiwa paling umum untuk subagent adalah:

| Peristiwa     | Input Matcher | Kapan itu terjadi                                                   |
| :------------ | :------------ | :------------------------------------------------------------------ |
| `PreToolUse`  | Nama alat     | Sebelum subagent menggunakan alat                                   |
| `PostToolUse` | Nama alat     | Setelah subagent menggunakan alat                                   |
| `Stop`        | (tidak ada)   | Ketika subagent selesai (dikonversi ke `SubagentStop` saat runtime) |

Contoh ini memvalidasi perintah Bash dengan hook `PreToolUse` dan menjalankan linter setelah edit file dengan `PostToolUse`:

```yaml theme={null}
---
name: code-reviewer
description: Review code changes with automatic linting
hooks:
  PreToolUse:
    - matcher: "Bash"
      hooks:
        - type: command
          command: "./scripts/validate-command.sh $TOOL_INPUT"
  PostToolUse:
    - matcher: "Edit|Write"
      hooks:
        - type: command
          command: "./scripts/run-linter.sh"
---
```

Hooks `Stop` dalam frontmatter secara otomatis dikonversi ke peristiwa `SubagentStop`.

<h4 id="project-level-hooks-for-subagent-events">
  Hooks tingkat proyek untuk peristiwa subagent
</h4>

Konfigurasi hooks dalam `settings.json` yang merespons peristiwa siklus hidup subagent dalam sesi utama.

| Peristiwa       | Input Matcher   | Kapan itu terjadi                |
| :-------------- | :-------------- | :------------------------------- |
| `SubagentStart` | Nama jenis agen | Ketika subagent mulai dijalankan |
| `SubagentStop`  | Nama jenis agen | Ketika subagent selesai          |

Kedua peristiwa mendukung matcher untuk menargetkan jenis agen tertentu berdasarkan nama. Nilai matcher adalah `name` frontmatter agen untuk subagent tingkat proyek dan pengguna, atau pengenal yang dibatasi cakupan plugin seperti `my-plugin:db-agent` untuk [subagent plugin](/docs/id/plugins). Nama yang dibatasi cakupan berisi titik dua, sehingga dievaluasi sebagai [ekspresi reguler yang tidak berlabuh](/docs/id/hooks#matcher-patterns); jangkarnya dengan `^` dan `$`, seperti dalam `^my-plugin:db-agent$`, untuk mencocokkan hanya agen itu.

Contoh ini menjalankan skrip setup hanya ketika subagent `db-agent` dimulai, dan skrip cleanup ketika subagent apa pun berhenti:

```json theme={null}
{
  "hooks": {
    "SubagentStart": [
      {
        "matcher": "db-agent",
        "hooks": [
          { "type": "command", "command": "./scripts/setup-db-connection.sh" }
        ]
      }
    ],
    "SubagentStop": [
      {
        "hooks": [
          { "type": "command", "command": "./scripts/cleanup-db-connection.sh" }
        ]
      }
    ]
  }
}
```

Matcher dengan tanda hubung seperti `db-agent` cocok dengan tepat pada Claude Code v2.1.195 atau lebih baru. Pada versi sebelumnya dievaluasi sebagai ekspresi reguler yang tidak berlabuh dan juga terjadi untuk jenis agen apa pun yang berisinya, seperti `prod-db-agent`; jangkarnya sebagai `^db-agent$` pada versi-versi itu.

Lihat [Hooks](/docs/id/hooks) untuk format konfigurasi hook lengkap.

<h2 id="work-with-subagents">
  Bekerja dengan subagent
</h2>

<h3 id="understand-automatic-delegation">
  Pahami delegasi otomatis
</h3>

Claude secara otomatis mendelegasikan tugas berdasarkan deskripsi tugas dalam permintaan Anda, bidang `description` dalam konfigurasi subagent, dan konteks saat ini. Untuk mendorong delegasi proaktif, sertakan frasa seperti "use proactively" dalam bidang deskripsi subagent Anda.

<h3 id="invoke-subagents-explicitly">
  Panggil subagent secara eksplisit
</h3>

Ketika delegasi otomatis tidak cukup, Anda dapat meminta subagent sendiri. Tiga pola meningkat dari saran satu kali ke default sesi-lebar:

* **Bahasa alami**: sebutkan subagent dalam prompt Anda; Claude memutuskan apakah akan mendelegasikan
* **@-mention**: menjamin subagent berjalan untuk satu tugas
* **Sesi-lebar**: seluruh sesi menggunakan prompt sistem subagent, pembatasan alat, dan model melalui flag `--agent` atau pengaturan `agent`

Untuk bahasa alami, tidak ada sintaks khusus. Sebutkan subagent dan Claude biasanya mendelegasikan:

```text wrap theme={null}
Use the test-runner subagent to fix failing tests
Have the code-reviewer subagent look at my recent changes
```

**@-mention subagent.** Ketik `@` dan pilih subagent dari typeahead, dengan cara yang sama Anda @-mention file. Ini memastikan subagent tertentu berjalan daripada meninggalkan pilihan kepada Claude:

```text wrap theme={null}
@"code-reviewer (agent)" look at the auth changes
```

Pesan lengkap Anda masih pergi ke Claude, yang menulis prompt tugas subagent berdasarkan apa yang Anda minta. @-mention mengontrol subagent mana yang Claude panggil, bukan prompt apa yang diterima.

Subagent yang disediakan oleh [plugin](/docs/id/plugins) yang diaktifkan muncul di typeahead dengan nama yang dibatasi, seperti `my-plugin:code-reviewer` atau `my-plugin:review:security` ketika plugin [mengorganisir agen ke dalam subfolder](#choose-the-subagent-scope). Subagent background bernama yang saat ini berjalan dalam sesi juga muncul di typeahead, menunjukkan status mereka di samping nama.

Anda juga dapat mengetik mention secara manual tanpa menggunakan picker: `@agent-<name>` untuk subagent lokal, atau `@agent-` diikuti dengan nama yang dibatasi untuk subagent plugin, misalnya `@agent-my-plugin:code-reviewer`.

**Jalankan seluruh sesi sebagai subagent.** Lewatkan [`--agent <name>`](/docs/id/cli-reference) untuk memulai sesi di mana thread utama itu sendiri mengambil prompt sistem subagent, pembatasan alat, dan model:

```bash theme={null}
claude --agent code-reviewer
```

Prompt sistem subagent menggantikan prompt sistem Claude Code default sepenuhnya, dengan cara yang sama [`--system-prompt`](/docs/id/cli-reference) melakukannya. File `CLAUDE.md` dan memori proyek masih dimuat melalui aliran pesan normal. Nama agen muncul sebagai `@<name>` di header startup sehingga Anda dapat mengonfirmasi itu aktif.

Ini berfungsi dengan subagent bawaan dan khusus, dan pilihan bertahan ketika Anda melanjutkan sesi.

Untuk subagent yang disediakan plugin, Anda dapat melewatkan hanya nama agen dan Claude Code akan menemukannya:

```bash theme={null}
claude --agent security-reviewer
```

Jika beberapa plugin menyediakan agen dengan nama yang sama, lewatkan nama yang dibatasi untuk membedakan:

```bash theme={null}
claude --agent my-plugin:security-reviewer
```

Jika plugin menempatkan agen dalam subfolder dari direktori `agents/` nya, sertakan subfolder dalam nama yang dibatasi, misalnya `claude --agent my-plugin:review:security`.

Untuk menjadikannya default untuk setiap sesi dalam proyek, atur `agent` dalam `.claude/settings.json`:

```json theme={null}
{
  "agent": "code-reviewer"
}
```

Flag CLI menimpa pengaturan jika keduanya ada.

<h3 id="run-subagents-in-foreground-or-background">
  Jalankan subagent di foreground atau background
</h3>

Subagent dapat berjalan di foreground atau background:

* **Subagent foreground** memblokir percakapan utama sampai selesai. Prompt izin dilewatkan kepada Anda saat muncul.
* **Subagent background** berjalan secara bersamaan sementara Anda terus bekerja. Mulai dari v2.1.186, ketika subagent background mencapai panggilan alat yang memerlukan izin, prompt muncul di sesi utama Anda dan menyebutkan subagent yang bertanya. Setujui untuk membiarkan subagent melanjutkan, atau tekan Esc untuk menolak panggilan alat itu saja tanpa menghentikan subagent. Sebelum v2.1.186, subagent background auto-deny setiap panggilan alat yang sebaliknya akan meminta.

Mulai dari v2.1.198, subagent berjalan di background secara default. Claude menjalankan subagent di foreground ketika memerlukan hasil sebelum melanjutkan. Default mengubah di mana subagent berjalan, bukan apa yang diizinkan untuk dilakukan: subagent background masih menampilkan setiap prompt izin di sesi utama Anda. Sebelum v2.1.198, Claude memilih antara foreground dan background berdasarkan tugas.

Anda juga dapat mengarahkan ini sendiri:

* Minta Claude untuk menjalankan tugas di background atau di foreground
* Tekan **Ctrl+B** untuk menempatkan tugas yang sedang berjalan di background

Subagent background yang selesai tetap terdaftar dalam [`/tasks`](/docs/id/commands), ditandai selesai dan diurutkan di bawah pekerjaan yang sedang berjalan, sampai sesi membersihkan daftar tugasnya. Tampilan detailnya tetap terbuka ketika subagent selesai. Subagent yang gagal atau yang Anda hentikan meninggalkan daftar. Sebelum v2.1.208, subagent yang selesai meninggalkan daftar saat itu selesai dan tampilan detailnya ditutup.

Untuk menonaktifkan semua fungsionalitas background task, atur variabel lingkungan `CLAUDE_CODE_DISABLE_BACKGROUND_TASKS` ke `1`. Lihat [Environment variables](/docs/id/env-vars).

Ketika [`CLAUDE_CODE_FORK_SUBAGENT`](#fork-the-current-conversation) diatur ke `1`, setiap spawn subagent berjalan di background dan bidang frontmatter `background` tidak berpengaruh, karena fork mode menghapus parameter `run_in_background` dari alat `Agent`. `CLAUDE_CODE_DISABLE_BACKGROUND_TASKS` mengambil alih fork mode dan menjaga spawn subagent di foreground.

<h3 id="api-errors-in-subagents">
  Kesalahan API dalam subagent
</h3>

Mulai dari v2.1.199, subagent yang berjalan berakhir pada kesalahan API, seperti batas penggunaan atau kesalahan server berulang, melaporkan kegagalan itu kembali ke Claude daripada mengembalikan teks kesalahan seolah-olah itu adalah temuan subagent. Apa yang Claude terima tergantung di mana subagent berjalan:

* **Foreground**: jika batas laju, kelebihan beban, atau kesalahan server memotong subagent yang sudah menghasilkan output teks, alat Agent mengembalikan output parsial itu dengan catatan bahwa subagent dipotong dan tidak menyelesaikan tugasnya. Subagent yang tidak menghasilkan apa pun, atau yang output-nya hanya panggilan alat, gagal dengan [`Agent terminated early due to an API error`](/docs/id/errors#agent-terminated-early-due-to-an-api-error), diikuti oleh detail kesalahan. Dalam v2.1.199, batas laju, kelebihan beban, atau kesalahan server yang memotong bentuk tool-calls-only mengembalikan hasil parsial kosong yang hanya berisi catatan cut-off sebagai gantinya.
* **Background**: subagent ditandai gagal, dan pesan yang Claude terima saat berakhir menyebutkan kesalahan API dan menyertakan output terakhir subagent, jadi pekerjaan parsial tidak hilang.

Setelah kesalahan API yang mendasar hilang, minta Claude untuk mencoba ulang tugas atau [lanjutkan subagent](#resume-subagents).

<h3 id="common-patterns">
  Pola umum
</h3>

<h4 id="isolate-high-volume-operations">
  Isolasi operasi volume tinggi
</h4>

Salah satu penggunaan paling efektif untuk subagent adalah mengisolasi operasi yang menghasilkan jumlah output besar. Menjalankan tes, mengambil dokumentasi, atau memproses file log dapat mengonsumsi konteks yang signifikan. Dengan mendelegasikan ini ke subagent, output verbose tetap dalam konteks subagent sementara hanya ringkasan relevan yang kembali ke percakapan utama Anda.

```text wrap theme={null}
Use a subagent to run the test suite and report only the failing tests with their error messages
```

<h4 id="run-parallel-research">
  Jalankan penelitian paralel
</h4>

Untuk investigasi independen, hasilkan beberapa subagent untuk bekerja secara bersamaan:

```text wrap theme={null}
Research the authentication, database, and API modules in parallel using separate subagents
```

Setiap subagent mengeksplorasi areanya secara independen, kemudian Claude mensintesis temuan. Ini berfungsi terbaik ketika jalur penelitian tidak saling bergantung.

<Warning>
  Ketika subagent selesai, hasil mereka kembali ke percakapan utama Anda. Menjalankan banyak subagent yang masing-masing mengembalikan hasil terperinci dapat mengonsumsi konteks yang signifikan.
</Warning>

Untuk tugas yang memerlukan paralelisme berkelanjutan atau melebihi jendela konteks Anda, [tim agen](/docs/id/agent-teams) memberikan setiap pekerja konteksnya sendiri yang independen.

<h4 id="chain-subagents">
  Rantai subagent
</h4>

Untuk alur kerja multi-langkah, minta Claude untuk menggunakan subagent secara berurutan. Setiap subagent menyelesaikan tugasnya dan mengembalikan hasil ke Claude, yang kemudian melewatkan konteks relevan ke subagent berikutnya.

```text wrap theme={null}
Use the code-reviewer subagent to find performance issues, then use the optimizer subagent to fix them
```

<h3 id="choose-between-subagents-and-main-conversation">
  Pilih antara subagent dan percakapan utama
</h3>

Gunakan **percakapan utama** ketika:

* Tugas memerlukan bolak-balik yang sering atau penyempurnaan iteratif
* Beberapa fase berbagi konteks yang signifikan, seperti perencanaan, implementasi, dan pengujian
* Anda membuat perubahan cepat dan tertarget
* Latensi penting. Subagent dimulai segar dan mungkin memerlukan waktu untuk mengumpulkan konteks

Gunakan **subagent** ketika:

* Tugas menghasilkan output verbose yang Anda tidak butuhkan dalam konteks utama Anda
* Anda ingin menerapkan pembatasan alat atau izin tertentu
* Pekerjaan mandiri dan dapat mengembalikan ringkasan

Pertimbangkan [Skills](/docs/id/skills) sebagai gantinya ketika Anda menginginkan prompt atau alur kerja yang dapat digunakan kembali yang berjalan dalam konteks percakapan utama daripada konteks subagent yang terisolasi.

Untuk pertanyaan cepat tentang sesuatu yang sudah ada dalam percakapan Anda, gunakan [`/btw`](/docs/id/interactive-mode#side-questions-with-%2Fbtw) sebagai gantinya dari subagent. Ini melihat konteks penuh Anda tetapi tidak memiliki akses alat, dan jawabannya dibuang daripada ditambahkan ke riwayat.

<h3 id="spawn-nested-subagents">
  Hasilkan subagent bersarang
</h3>

Mulai dari Claude Code v2.1.172, subagent dapat menghasilkan subagent-nya sendiri. Gunakan ini ketika tugas yang didelegasikan itu sendiri terbagi menjadi subtask paralel, seperti subagent reviewer yang mengirimkan verifier per temuan, sehingga output perantara tidak pernah mencapai percakapan utama Anda. Hanya ringkasan subagent tingkat atas yang kembali kepada Anda.

Subagent bersarang dikonfigurasi dengan cara yang sama seperti subagent tingkat atas dan diselesaikan dari scope yang sama.

Panel subagent di bawah input prompt menunjukkan pohon lengkap: setiap baris menampilkan hitungan `(+N)` keturunan, dan mulai dari v2.1.193, membuka baris menunjukkan saudara dan anak langsung subagent itu dengan jalur kembali ke `main`.

Kedalaman dihitung sebagai jumlah level subagent di bawah percakapan utama, terlepas dari apakah setiap level berjalan di foreground atau background. Subagent pada kedalaman lima tidak menerima alat Agent dan tidak dapat menghasilkan lebih lanjut. Batasnya tetap dan tidak dapat dikonfigurasi.

Mulai dari Claude Code v2.1.187, kedalaman subagent background diperbaiki ketika pertama kali dihasilkan, dan melanjutkan nanti tidak mengubah kedalaman itu. Misalnya, jika percakapan utama Anda menghasilkan subagent A, dan A menghasilkan subagent background B pada kedalaman dua, B masih pada kedalaman dua ketika Anda melanjutkannya langsung dari percakapan utama. Melanjutkan subagent dari konteks yang lebih dangkal tidak membiarkannya menghasilkan level tambahan yang batas kedalaman sudah cegah.

Untuk mencegah subagent tertentu menghasilkan yang lain, hilangkan `Agent` dari daftar `tools` atau tambahkan ke `disallowedTools`.

Fork masih tidak dapat menghasilkan fork lain. Ini dapat menghasilkan jenis subagent lain, dan mereka dihitung menuju batas kedalaman.

<h3 id="manage-subagent-context">
  Kelola konteks subagent
</h3>

<h4 id="what-loads-at-startup">
  Apa yang dimuat saat startup
</h4>

Setiap subagent dimulai dengan jendela konteks yang segar dan terisolasi. Ini tidak melihat riwayat percakapan Anda, skills yang sudah Anda panggil, atau file yang sudah Claude baca. Claude menyusun pesan delegasi yang merangkum tugas, dan subagent bekerja dari sana. Pengecualiannya adalah fork, yang mewarisi percakapan induk daripada memulai segar.

Konteks awal subagent non-fork berisi:

* **Prompt sistem**: prompt agen itu sendiri ditambah detail lingkungan yang Claude Code tambahkan, bukan prompt sistem Claude Code lengkap. Subagent khusus mendefinisikan milik mereka dalam badan markdown atau bidang `prompt`. Agen bawaan memiliki prompt yang telah ditentukan sebelumnya.
* **Pesan tugas**: prompt delegasi yang Claude tulis ketika menyerahkan pekerjaan.
* **CLAUDE.md dan memori**: setiap level dari hierarki memori yang dimuat percakapan utama, termasuk `~/.claude/CLAUDE.md`, aturan proyek, `CLAUDE.local.md`, dan file kebijakan yang dikelola. Agen Explore dan Plan bawaan melewati ini.
* **Status Git**: snapshot yang diambil di awal sesi induk. Tidak ada ketika direktori kerja bukan repositori Git atau ketika `includeGitInstructions` adalah `false`. Explore dan Plan melewatinya terlepas.
* **Skills yang dimuat sebelumnya**: konten lengkap dari skill apa pun yang dinamai dalam bidang `skills` agen. Agen bawaan tidak memuat skills sebelumnya.
* **Daftar saudara**: pengingat sistem yang mencantumkan `main` dan setiap agen bernama lainnya dalam sesi, masing-masing nilai `to` yang valid untuk [`SendMessage`](#resume-subagents). Memerlukan Claude Code v2.1.206 atau lebih baru. Daftar muncul hanya ketika alat subagent mencakup `SendMessage` dan setidaknya satu agen lain memiliki nama, baik Claude menamakannya saat memunculkannya atau berjalan sebagai anggota [tim agen](/docs/id/agent-teams). Ini adalah snapshot yang diambil ketika subagent dimulai, jadi agen yang dinamai nanti tidak muncul.

Explore dan Plan adalah satu-satunya subagent yang menghilangkan CLAUDE.md dan status git. Tidak ada bidang frontmatter atau pengaturan per-agen untuk mengubah agen mana yang melewatinya.

Percakapan utama membaca hasil Explore dan Plan dengan konteks CLAUDE.md penuh, jadi sebagian besar aturan tidak perlu mencapai subagent itu sendiri. Jika aturan harus, seperti "abaikan direktori `vendor/`," nyatakan kembali dalam prompt yang Anda berikan Claude saat mendelegasikan.

<h4 id="resume-subagents">
  Lanjutkan subagent
</h4>

Setiap invokasi subagent membuat instance baru dengan konteks segar. Untuk melanjutkan pekerjaan subagent yang ada daripada memulai dari awal, minta Claude untuk melanjutkannya.

Subagent yang dilanjutkan mempertahankan riwayat percakapan lengkap mereka, termasuk semua panggilan alat sebelumnya, hasil, dan penalaran. Subagent melanjutkan tepat di mana ia berhenti daripada memulai segar.

Ketika subagent selesai, Claude menerima ID agennya. Agen bawaan Explore dan Plan adalah one-shot dan tidak mengembalikan ID agen, jadi mereka tidak dapat dilanjutkan; gunakan `general-purpose` atau subagent khusus ketika Anda perlu melanjutkan pekerjaan.

Claude menggunakan alat `SendMessage` dengan ID agen atau nama agen sebagai bidang `to` untuk melanjutkannya. `SendMessage` tidak memerlukan tim agen untuk diaktifkan; hanya pesan protokol tim terstruktur seperti `shutdown_request` dan `plan_approval_response` yang melakukannya.

Untuk melanjutkan subagent, minta Claude untuk melanjutkan pekerjaan sebelumnya:

```text wrap theme={null}
Use the code-reviewer subagent to review the authentication module
[Agent completes]

Continue that code review and now analyze the authorization logic
[Claude resumes the subagent with full context from previous conversation]
```

Subagent yang dihentikan yang menerima `SendMessage` auto-resume di background tanpa memerlukan invokasi `Agent` baru. Hal yang sama berlaku untuk subagent yang Claude hentikan dengan alat `TaskStop`.

Mulai dari v2.1.191, subagent yang Anda hentikan sendiri, dengan `x` dalam `/tasks` atau permintaan SDK `stop_task`, tidak auto-resume. Panggilan `SendMessage` mengembalikan penolakan yang memberi tahu Claude agen dibatalkan. Ketik ke dalam transkrip subagent itu di panel subagent untuk melanjutkannya sendiri, yang menghapus stop sehingga panggilan `SendMessage` nanti dapat auto-resume lagi.

Melanjutkan memulai run baru dari agen di bawah ID yang sama, jadi subagent yang sudah gagal atau selesai menunjukkan sebagai berjalan lagi dalam daftar tugas dan dalam peristiwa tugas SDK Agent. Sebelum v2.1.205, itu terus menunjukkan status gagal atau selesai sebelumnya sementara run yang dilanjutkan sedang bekerja.

Mulai dari v2.1.199, `SendMessage` memeriksa bahwa nama masih merujuk ke agen yang sama yang dicapai sebelumnya dalam percakapan. Jika agen yang lebih baru telah mengambil nama, seperti agen background yang di-spawn ulang yang menggunakannya kembali, Claude Code menolak pengiriman daripada mengirimkannya ke agen yang salah, dan kesalahan melaporkan agen mana yang sekarang dicapai nama sehingga Claude dapat menargetkan ulang. Untuk mencapai agen sebelumnya sementara masih berjalan, Claude mengalamatkannya dengan ID agen dari hasil spawn. Pemeriksaan dibatasi pada percakapan saat ini dan direset pada `/clear`.

Mulai dari v2.1.198, subagent memperlakukan pesan dari agen yang meluncurkannya sebagai arahan tugas normal, termasuk koreksi kursus mid-task, dan bertindak atas mereka dalam pengaturan izin mereka sendiri. Dua batas masih berlaku terlepas dari siapa yang mengirim pesan: tidak ada pesan dari agen apa pun yang dihitung sebagai persetujuan Anda untuk prompt izin yang tertunda, dan tidak ada pesan agen yang dapat mengubah pengaturan izin subagent, `CLAUDE.md`, atau konfigurasi. Hanya sistem izin atau pesan Anda sendiri yang dapat memberikan persetujuan.

Anda juga dapat meminta Claude untuk ID agen jika Anda ingin mereferensikannya secara eksplisit, atau temukan ID dalam file transkrip di `~/.claude/projects/{project}/{sessionId}/subagents/`. Setiap transkrip disimpan sebagai `agent-{agentId}.jsonl`.

Transkrip subagent bertahan secara independen dari percakapan utama:

* **Pemadatan percakapan utama**: Ketika percakapan utama dipadatkan, transkrip subagent tidak terpengaruh. Mereka disimpan dalam file terpisah.
* **Persistensi sesi**: Transkrip subagent bertahan dalam sesi mereka. Anda dapat melanjutkan subagent setelah memulai ulang Claude Code dengan melanjutkan sesi yang sama.
* **Pembersihan otomatis**: Transkrip dibersihkan berdasarkan pengaturan `cleanupPeriodDays`, yang default ke 30 hari.

<h4 id="auto-compaction">
  Auto-compaction
</h4>

Subagent mendukung pemadatan otomatis menggunakan logika yang sama dengan percakapan utama. Pemadatan dipicu di bawah kondisi yang sama, dan `CLAUDE_AUTOCOMPACT_PCT_OVERRIDE` berlaku untuk subagent juga. Lihat [environment variables](/docs/id/env-vars) untuk kapan override berlaku.

Peristiwa pemadatan dicatat dalam file transkrip subagent:

```json theme={null}
{
  "type": "system",
  "subtype": "compact_boundary",
  "compactMetadata": {
    "trigger": "auto",
    "preTokens": 167189
  }
}
```

Nilai `preTokens` menunjukkan berapa banyak token yang digunakan sebelum pemadatan terjadi.

<h2 id="fork-the-current-conversation">
  Fork percakapan saat ini
</h2>

<Note>
  Subagent yang di-fork memerlukan Claude Code v2.1.117 atau lebih baru. Mulai dari v2.1.161 perintah `/fork` diaktifkan secara default; pada versi sebelumnya memerlukan pengaturan variabel lingkungan [`CLAUDE_CODE_FORK_SUBAGENT`](/docs/id/env-vars) ke `1`. Membiarkan Claude sendiri menelurkan fork bersifat eksperimental dan mungkin berubah di rilis mendatang. Kemampuan ini juga dapat diaktifkan dalam sesi interaktif sebagai bagian dari peluncuran bertahap.
</Note>

Fork adalah subagent yang mewarisi seluruh percakapan sejauh ini daripada memulai segar. Ini menghilangkan isolasi input yang sebaliknya disediakan subagent: fork melihat prompt sistem yang sama, alat, model, dan riwayat pesan sebagai sesi utama, sehingga Anda dapat menyerahkan tugas sampingan tanpa menjelaskan situasinya lagi. Panggilan alat fork sendiri masih tetap keluar dari percakapan Anda dan hanya hasil akhirnya yang kembali, sehingga jendela konteks utama Anda tetap bersih. Gunakan fork ketika subagent bernama memerlukan terlalu banyak latar belakang untuk berguna, atau ketika Anda ingin mencoba beberapa pendekatan secara paralel dari titik awal yang sama.

Untuk mengontrol mode fork terlepas dari peluncuran bertahap, atur [`CLAUDE_CODE_FORK_SUBAGENT`](/docs/id/env-vars) ke `1` untuk mengaktifkannya secara eksplisit atau ke `0` untuk menonaktifkannya. Variabel ini dihormati dalam mode interaktif dan melalui SDK atau `claude -p`.

Mengaktifkan mode fork mengubah Claude Code dalam dua cara:

* Claude dapat menelurkan fork dengan meminta tipe subagent `fork` secara eksplisit. Spawn tanpa tipe subagent masih menggunakan subagent [general-purpose](#built-in-subagents), dan subagent bernama seperti Explore masih menelurkan seperti sebelumnya.
* Setiap spawn subagent berjalan di [background](#run-subagents-in-foreground-or-background), apakah itu fork atau subagent bernama. Atur `CLAUDE_CODE_DISABLE_BACKGROUND_TASKS` ke `1` untuk menjaga spawn tetap sinkron.

Anda dapat memulai fork sendiri dengan `/fork` diikuti oleh direktif, dengan atau tanpa variabel yang ditetapkan. Claude Code memberi nama fork dari kata-kata pertama direktif. Contoh berikut mem-fork percakapan untuk draft kasus uji sementara Anda melanjutkan dengan implementasi dalam sesi utama:

```text wrap theme={null}
/fork draft unit tests for the parser changes so far
```

Fork muncul di panel di bawah prompt Anda dan berjalan di background sementara Anda terus bekerja. Ketika selesai, hasilnya tiba sebagai pesan dalam percakapan utama Anda. Bagian berikutnya mencakup kontrol panel untuk menonton dan mengarahkan fork saat berjalan.

<h3 id="observe-and-steer-running-forks">
  Amati dan arahkan fork yang sedang berjalan
</h3>

Fork yang sedang berjalan muncul di panel di bawah input prompt, dengan satu baris untuk sesi utama dan satu untuk setiap fork. Gunakan kunci ini untuk berinteraksi dengan panel:

| Kunci     | Tindakan                                                          |
| :-------- | :---------------------------------------------------------------- |
| `↑` / `↓` | Pindah antar baris                                                |
| `Enter`   | Buka transkrip fork yang dipilih dan kirimkan pesan tindak lanjut |
| `x`       | Tutup fork yang selesai atau hentikan yang sedang berjalan        |
| `Esc`     | Kembalikan fokus ke input prompt                                  |

Dengan transkrip fork atau subagent terbuka, pesan tindak lanjut dan [skills](/docs/id/skills) pergi ke agen tersebut, tetapi perintah bawaan masih berjalan dalam percakapan utama Anda. Mulai dari v2.1.199, mengetik `/model` atau `/fast` dalam tampilan itu menampilkan pemberitahuan bahwa itu mengubah model percakapan utama atau mode cepat, bukan agen yang dilihat, daripada menjalankannya secara diam-diam.

<h3 id="how-forks-differ-from-named-subagents">
  Bagaimana fork berbeda dari subagent bernama
</h3>

Fork mewarisi segalanya yang dimiliki sesi utama pada saat spawn. Subagent bernama dimulai dari definisinya sendiri.

|                        | Fork                           | Subagent bernama                                                                                           |
| :--------------------- | :----------------------------- | :--------------------------------------------------------------------------------------------------------- |
| Konteks                | Riwayat percakapan lengkap     | Konteks segar dengan prompt yang Anda lewatkan                                                             |
| Prompt sistem dan alat | Sama dengan sesi utama         | Dari file definisi subagent                                                                                |
| Model                  | Sama dengan sesi utama         | Dari bidang `model` subagent                                                                               |
| Izin                   | Prompt muncul di terminal Anda | [Prompt muncul di sesi utama Anda](#run-subagents-in-foreground-or-background) saat berjalan di background |
| Prompt cache           | Dibagikan dengan sesi utama    | Cache terpisah                                                                                             |

Karena prompt sistem fork dan definisi alat identik dengan induk, permintaan pertamanya menggunakan kembali [prompt cache](/docs/id/prompt-caching#subagents-and-the-cache) induk. Ini membuat forking lebih murah daripada menelurkan subagent segar untuk tugas yang memerlukan konteks yang sama.

Ketika Claude menelurkan fork melalui alat Agent, Claude dapat melewatkan `isolation: "worktree"` sehingga edit file fork ditulis ke git worktree terpisah daripada checkout Anda.

<h3 id="limitations">
  Keterbatasan
</h3>

Menetapkan `CLAUDE_CODE_FORK_SUBAGENT=1` mengaktifkan fork mode dalam sesi interaktif, [non-interactive mode](/docs/id/headless), dan Agent SDK; menetapkannya ke `0` menonaktifkan fork mode di mana-mana, termasuk peluncuran server-side apa pun. Fork tidak dapat menelurkan fork lebih lanjut.

<h2 id="example-subagents">
  Contoh subagent
</h2>

Contoh-contoh ini mendemonstrasikan pola efektif untuk membangun subagent. Gunakan mereka sebagai titik awal, atau hasilkan versi yang disesuaikan dengan Claude.

<Tip>
  **Best practices:**

  * **Desain subagent yang terfokus:** setiap subagent harus unggul dalam satu tugas spesifik
  * **Tulis deskripsi terperinci:** Claude menggunakan deskripsi untuk memutuskan kapan mendelegasikan
  * **Batasi akses alat:** berikan hanya izin yang diperlukan untuk keamanan dan fokus
  * **Periksa ke dalam kontrol versi:** bagikan subagent proyek dengan tim Anda
</Tip>

<h3 id="code-reviewer">
  Peninjau kode
</h3>

Subagent hanya-baca yang meninjau kode tanpa memodifikasinya. Contoh ini menunjukkan cara merancang subagent yang terfokus dengan akses alat terbatas yang mengecualikan Edit dan Write, dan prompt terperinci yang menentukan dengan tepat apa yang harus dicari dan cara memformat output.

```markdown theme={null}
---
name: code-reviewer
description: Expert code review specialist. Proactively reviews code for quality, security, and maintainability. Use immediately after writing or modifying code.
tools: Read, Grep, Glob, Bash
model: inherit
---

You are a senior code reviewer ensuring high standards of code quality and security.

When invoked:
1. Run git diff to see recent changes
2. Focus on modified files
3. Begin review immediately

Review checklist:
- Code is clear and readable
- Functions and variables are well-named
- No duplicated code
- Proper error handling
- No exposed secrets or API keys
- Input validation implemented
- Good test coverage
- Performance considerations addressed

Provide feedback organized by priority:
- Critical issues (must fix)
- Warnings (should fix)
- Suggestions (consider improving)

Include specific examples of how to fix issues.
```

<h3 id="debugger">
  Debugger
</h3>

Subagent yang dapat menganalisis dan memperbaiki masalah. Tidak seperti peninjau kode, yang ini mencakup Edit karena memperbaiki bug memerlukan memodifikasi kode. Prompt menyediakan alur kerja yang jelas dari diagnosis ke verifikasi.

```markdown theme={null}
---
name: debugger
description: Debugging specialist for errors, test failures, and unexpected behavior. Use proactively when encountering any issues.
tools: Read, Edit, Bash, Grep, Glob
---

You are an expert debugger specializing in root cause analysis.

When invoked:
1. Capture error message and stack trace
2. Identify reproduction steps
3. Isolate the failure location
4. Implement minimal fix
5. Verify solution works

Debugging process:
- Analyze error messages and logs
- Check recent code changes
- Form and test hypotheses
- Add strategic debug logging
- Inspect variable states

For each issue, provide:
- Root cause explanation
- Evidence supporting the diagnosis
- Specific code fix
- Testing approach
- Prevention recommendations

Focus on fixing the underlying issue, not the symptoms.
```

<h3 id="data-scientist">
  Data scientist
</h3>

Subagent khusus domain untuk pekerjaan analisis data. Contoh ini menunjukkan cara membuat subagent untuk alur kerja khusus di luar tugas pengkodean khas. Ini secara eksplisit menetapkan `model: sonnet` untuk analisis yang lebih mampu.

```markdown theme={null}
---
name: data-scientist
description: Data analysis expert for SQL queries, BigQuery operations, and data insights. Use proactively for data analysis tasks and queries.
tools: Bash, Read, Write
model: sonnet
---

You are a data scientist specializing in SQL and BigQuery analysis.

When invoked:
1. Understand the data analysis requirement
2. Write efficient SQL queries
3. Use BigQuery command line tools (bq) when appropriate
4. Analyze and summarize results
5. Present findings clearly

Key practices:
- Write optimized SQL queries with proper filters
- Use appropriate aggregations and joins
- Include comments explaining complex logic
- Format results for readability
- Provide data-driven recommendations

For each analysis:
- Explain the query approach
- Document any assumptions
- Highlight key findings
- Suggest next steps based on data

Always ensure queries are efficient and cost-effective.
```

<h3 id="database-query-validator">
  Validator kueri database
</h3>

Subagent yang memungkinkan akses Bash tetapi memvalidasi perintah untuk mengizinkan hanya kueri SQL hanya-baca. Contoh ini menunjukkan cara menggunakan hooks `PreToolUse` untuk validasi bersyarat ketika Anda memerlukan kontrol lebih halus daripada bidang `tools`.

```markdown theme={null}
---
name: db-reader
description: Execute read-only database queries. Use when analyzing data or generating reports.
tools: Bash
hooks:
  PreToolUse:
    - matcher: "Bash"
      hooks:
        - type: command
          command: "./scripts/validate-readonly-query.sh"
---

You are a database analyst with read-only access. Execute SELECT queries to answer questions about the data.

When asked to analyze data:
1. Identify which tables contain the relevant data
2. Write efficient SELECT queries with appropriate filters
3. Present results clearly with context

You cannot modify data. If asked to INSERT, UPDATE, DELETE, or modify schema, explain that you only have read access.
```

Claude Code [melewatkan input hook sebagai JSON](/docs/id/hooks#pretooluse-input) melalui stdin ke perintah hook. Skrip validasi membaca JSON ini, mengekstrak perintah yang sedang dijalankan, dan memeriksanya terhadap daftar operasi penulisan SQL. Jika operasi penulisan terdeteksi, skrip [keluar dengan kode 2](/docs/id/hooks#exit-code-2-behavior-per-event) untuk memblokir eksekusi dan mengembalikan pesan kesalahan ke Claude melalui stderr.

Buat skrip validasi di mana saja dalam proyek Anda. Jalur harus cocok dengan bidang `command` dalam konfigurasi hook Anda:

```bash theme={null}
#!/bin/bash
# Blocks SQL write operations, allows SELECT queries

# Read JSON input from stdin
INPUT=$(cat)

# Extract the command field from tool_input using jq
COMMAND=$(echo "$INPUT" | jq -r '.tool_input.command // empty')

if [ -z "$COMMAND" ]; then
  exit 0
fi

# Block write operations (case-insensitive)
if echo "$COMMAND" | grep -iE '\b(INSERT|UPDATE|DELETE|DROP|CREATE|ALTER|TRUNCATE|REPLACE|MERGE)\b' > /dev/null; then
  echo "Blocked: Write operations not allowed. Use SELECT queries only." >&2
  exit 2
fi

exit 0
```

Di macOS dan Linux, buat skrip dapat dieksekusi:

```bash theme={null}
chmod +x ./scripts/validate-readonly-query.sh
```

Di Windows, tulis skrip validasi dalam PowerShell dan tambahkan `shell: powershell` ke entri hook. Lihat [menjalankan hooks dalam PowerShell](/docs/id/hooks#windows-powershell-tool).

Hook menerima JSON melalui stdin dengan perintah Bash dalam `tool_input.command`. Kode keluar 2 memblokir operasi dan mengirimkan pesan kesalahan kembali ke Claude. Lihat [Hooks](/docs/id/hooks#exit-code-output) untuk detail tentang kode keluar dan [Hook input](/docs/id/hooks#pretooluse-input) untuk skema input lengkap.

<h2 id="next-steps">
  Langkah berikutnya
</h2>

Sekarang setelah Anda memahami subagent, jelajahi fitur terkait ini:

* [Distribusikan subagent dengan plugins](/docs/id/plugins) untuk berbagi subagent di seluruh tim atau proyek
* [Jalankan Claude Code secara terprogram](/docs/id/headless) dengan Agent SDK untuk CI/CD dan otomasi
* [Gunakan MCP servers](/docs/id/mcp) untuk memberikan subagent akses ke alat dan data eksternal
