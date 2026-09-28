> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Buat subagent khusus

> Buat dan gunakan subagent AI khusus di Claude Code untuk alur kerja khusus tugas dan manajemen konteks yang lebih baik.

Subagent adalah asisten AI khusus yang menangani jenis tugas tertentu. Gunakan satu ketika tugas sampingan akan membanjiri percakapan utama Anda dengan hasil pencarian, log, atau konten file yang tidak akan Anda referensikan lagi: subagent melakukan pekerjaan itu dalam konteksnya sendiri dan hanya mengembalikan ringkasan. Tentukan subagent khusus ketika Anda terus menelurkan jenis pekerja yang sama dengan instruksi yang sama.

Setiap subagent berjalan di jendela konteksnya sendiri dengan prompt sistem khusus, akses alat tertentu, dan izin independen. Ketika Claude menemukan tugas yang sesuai dengan deskripsi subagent, Claude mendelegasikan ke subagent tersebut, yang bekerja secara independen dan mengembalikan hasil. Untuk melihat penghematan konteks dalam praktik, [visualisasi jendela konteks](/docs/id/context-window) menjelaskan sesi di mana subagent menangani penelitian di jendela terpisahnya sendiri.

<Note>
  Subagent bekerja dalam satu sesi. Untuk menjalankan banyak sesi independen secara paralel dan memantaunya dari satu tempat, lihat [agen latar belakang](/docs/id/agent-view). Untuk sesi terpisah yang melewatkan pesan satu sama lain, lihat [pesan lintas sesi](/docs/id/cross-session-messaging). Untuk tim yang terkoordinasi dari sesi yang Claude luncurkan dan awasi, lihat [tim agen](/docs/id/agent-teams).
</Note>

Subagent membantu Anda:

* **Mempertahankan konteks** dengan menjaga eksplorasi dan implementasi di luar percakapan utama Anda
* **Menerapkan batasan** dengan membatasi alat mana yang dapat digunakan subagent
* **Menggunakan kembali konfigurasi** di seluruh proyek dengan subagent tingkat pengguna
* **Mengkhususkan perilaku** dengan prompt sistem yang terfokus untuk domain tertentu
* **Mengontrol biaya** dengan merutekan tugas ke model yang lebih cepat dan lebih murah seperti Haiku

Claude menggunakan deskripsi setiap subagent untuk memutuskan kapan mendelegasikan tugas. Ketika Anda membuat subagent, tulis deskripsi yang jelas sehingga Claude tahu kapan menggunakannya.

Deskripsi tersebut menggunakan konteks, jadi tetap singkat. Ketika deskripsi gabungan subagent Anda, kecuali yang bawaan, melebihi 15.000 token, Claude Code menampilkan [peringatan saat startup dengan jumlah token total](/docs/id/errors#agent-descriptions-are-over-the-15000-token-limit). Potong bidang `description` subagent Anda, dan pindahkan detail ke prompt sistem setiap subagent, yang hanya dimuat ketika subagent itu berjalan.

<h2 id="built-in-subagents">
  Subagent bawaan
</h2>

Claude Code mencakup subagent bawaan yang Claude gunakan secara otomatis jika sesuai. Masing-masing mewarisi izin percakapan induk; sebagian besar berjalan dengan set alat yang terbatas.

Explore dan Plan melewati file CLAUDE.md Anda dan status git sesi induk untuk menjaga penelitian tetap cepat dan hemat biaya. Setiap subagent bawaan lainnya dan [subagent khusus](#configure-subagents) memuat keduanya, kecuali definisinya menetapkan bidang [`omitClaudeMd`](#supported-frontmatter-fields) untuk melewati file CLAUDE.md pengguna, proyek, dan lokal. Untuk rincian lengkap tentang apa yang mencapai subagent, lihat [apa yang dimuat saat startup](#what-loads-at-startup).

<Tabs>
  <Tab title="Explore">
    Agen cepat yang dioptimalkan hanya-baca untuk mencari dan menganalisis basis kode.

    * **Model**: mewarisi dari percakapan utama, dibatasi pada Opus di Claude API, jadi Explore tidak pernah berjalan pada model yang lebih mahal daripada yang sudah Anda pilih untuk sesi, kecuali Anda menetapkan `CLAUDE_CODE_SUBAGENT_MODEL` dan [memaksanya ke setiap subagent](#run-every-subagent-on-one-model)
    * **Tools**: alat hanya-baca; Write dan Edit ditolak
    * **Purpose**: penemuan file, pencarian kode, eksplorasi basis kode

    Mulai dari v2.1.198, Explore mewarisi model percakapan utama alih-alih selalu berjalan pada Haiku. Di Claude API, model yang diwarisi dibatasi pada Opus: percakapan utama pada tingkat yang lebih tinggi menjalankan Explore pada Opus, dan percakapan utama pada Sonnet atau Haiku menjalankan Explore pada model yang sama. Di penyedia lain apa pun, seperti [Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry, atau Claude Platform on AWS](/docs/id/third-party-integrations), Explore mewarisi model percakapan utama secara langsung.

    [User atau project subagent](#choose-the-subagent-scope) bernama `Explore` menggantikan yang bawaan dan menyimpan bidang `model` miliknya sendiri, jadi tentukan satu dengan `model: haiku` untuk menjaga eksplorasi pada model dengan biaya lebih rendah.

    Claude mendelegasikan ke Explore ketika perlu mencari atau memahami basis kode tanpa membuat perubahan. Ini menjaga hasil eksplorasi di luar konteks percakapan utama Anda.

    Saat memanggil Explore, Claude menentukan tingkat ketelitian: **quick** untuk pencarian yang ditargetkan, **medium** untuk eksplorasi seimbang, atau **very thorough** untuk analisis komprehensif.
  </Tab>

  <Tab title="Plan">
    Agen penelitian yang digunakan selama [plan mode](/docs/id/permission-modes#analyze-before-you-edit-with-plan-mode) untuk mengumpulkan konteks sebelum menyajikan rencana.

    * **Model**: mewarisi dari percakapan utama, kecuali Anda menetapkan `CLAUDE_CODE_SUBAGENT_MODEL` dan [memaksanya ke setiap subagent](#run-every-subagent-on-one-model)
    * **Tools**: alat hanya-baca; Write dan Edit ditolak
    * **Purpose**: penelitian basis kode untuk perencanaan

    Ketika Anda dalam plan mode dan Claude perlu memahami basis kode Anda, Claude mendelegasikan penelitian ke subagent Plan sehingga output eksplorasi tetap dalam jendela konteks terpisah sementara percakapan utama tetap hanya-baca.
  </Tab>

  <Tab title="General-purpose">
    Agen yang mampu untuk tugas kompleks multi-langkah yang memerlukan eksplorasi dan tindakan.

    * **Model**: model [`CLAUDE_CODE_SUBAGENT_MODEL`](#choose-a-model) jika Anda menetapkan satu dan tidak ada yang menetapkan model dengan cara lain, jika tidak model percakapan utama; [Pilih model](#choose-a-model) menyatakan urutan lengkap, dan [Jalankan setiap subagent pada satu model](#run-every-subagent-on-one-model) menunjukkan cara membuat variabel menggantikan sumber-sumber tersebut
    * **Tools**: setiap alat [tersedia untuk subagent](#available-tools)
    * **Purpose**: penelitian kompleks, operasi multi-langkah, modifikasi kode

    Claude mendelegasikan ke general-purpose ketika tugas memerlukan eksplorasi dan modifikasi, penalaran kompleks untuk menafsirkan hasil, atau beberapa langkah yang saling bergantung.
  </Tab>

  <Tab title="Other">
    Claude Code mencakup agen pembantu tambahan untuk tugas tertentu. Ini biasanya dipanggil secara otomatis, jadi Anda tidak perlu menggunakannya secara langsung.

    | Agen              | Model                                                                                                              | Kapan Claude menggunakannya                                                                                                                                                                                                                                                                                                        |
    | :---------------- | :----------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
    | claude            | Tidak ada miliknya sendiri; mengikuti [urutan model](#choose-a-model) ketika Claude menelurkannya sebagai subagent | Ketika tugas tidak sesuai dengan agen yang lebih khusus. Catch-all dengan setiap alat [tersedia untuk subagent](#available-tools). Juga agen default untuk [sesi latar belakang](/docs/id/agent-view) yang dikirim; [mode izin mana yang dimulainya](/docs/id/agent-view#permission-mode-model-and-effort) tergantung pada cara sesi dimulai |
    | statusline-setup  | Sonnet                                                                                                             | Ketika Anda menjalankan `/statusline` untuk mengonfigurasi baris status Anda                                                                                                                                                                                                                                                       |
    | claude-code-guide | Haiku                                                                                                              | Ketika Anda mengajukan pertanyaan tentang fitur Claude Code                                                                                                                                                                                                                                                                        |
  </Tab>
</Tabs>

Subagent bawaan terdaftar secara default dalam sesi interaktif. Untuk membatasi mereka:

* Untuk memblokir tipe bawaan tertentu, tambahkan ke `permissions.deny` seperti yang ditunjukkan dalam [Nonaktifkan subagent tertentu](#disable-specific-subagents).
* Untuk mencegah Claude mendelegasikan ke subagent apa pun, tolak alat `Agent` itu sendiri dengan [`permissions.deny`](/docs/id/permissions#tool-specific-permission-rules).
* Untuk menghapus hanya subagent bawaan `Explore` dan `Plan`, atur [`CLAUDE_CODE_DISABLE_EXPLORE_PLAN_AGENTS=1`](/docs/id/env-vars). Claude membaca dan mengeksplorasi file secara langsung alih-alih mendelegasikan ke mereka. Memerlukan Claude Code v2.1.198 atau lebih baru.
* Dalam [mode non-interaktif](/docs/id/headless) dan [Agent SDK](/docs/id/agent-sdk/overview), atur [`CLAUDE_AGENT_SDK_DISABLE_BUILTIN_AGENTS=1`](/docs/id/env-vars) untuk menghapus semua tipe bawaan dan menyediakan hanya milik Anda sendiri.

Panggilan alat Agent yang menghilangkan `subagent_type` gagal dengan [`subagent_type is required`](/docs/id/errors#subagent-type-is-required) ketika sesi tidak memiliki subagent `general-purpose` untuk kembali.

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

    Claude mendelegasikan ke subagent baru Anda, yang memindai basis kode dan mengembalikan saran perbaikan. Dalam transkrip, delegasi muncul sebagai baris pemanggilan alat yang menunjukkan nama subagent diikuti oleh deskripsi tugas singkat, seperti `code-improver(Suggest code improvements)`.

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
| Direktori `agents/` plugin | Tempat plugin diaktifkan | 5 (terendah)  | Diinstal dengan [plugins](/docs/id/plugins/overview)        |

**Subagent proyek** (`.claude/agents/`) ideal untuk subagent khusus untuk basis kode. Periksa mereka ke kontrol versi sehingga tim Anda dapat menggunakannya dan meningkatkannya secara kolaboratif.

Subagent proyek ditemukan dengan berjalan naik dari direktori kerja saat ini, sehingga setiap `.claude/agents/` antara sana dan akar repositori dipindai. Ketika lebih dari satu direktori bersarang ini mendefinisikan `name` yang sama, Claude Code menggunakan definisi yang paling dekat dengan direktori kerja.

Ketika Anda menambahkan direktori dengan `--add-dir` atau `/add-dir`, Claude Code juga memuat folder `.claude/agents/` miliknya, bersama dengan subagent proyek Anda. Lihat [Direktori tambahan](/docs/id/permissions#additional-directories-grant-file-access-not-configuration) untuk jenis konfigurasi lain mana yang dimuat dari `--add-dir`. Untuk berbagi subagent di seluruh proyek tanpa `--add-dir`, gunakan `~/.claude/agents/` atau [plugin](/docs/id/plugins/overview).

**Subagent pengguna** (`~/.claude/agents/`) adalah subagent pribadi yang tersedia di semua proyek Anda.

Claude Code memindai `.claude/agents/` dan `~/.claude/agents/` secara rekursif, sehingga Anda dapat mengorganisir definisi ke dalam subfolder seperti `agents/review/` atau `agents/research/`. Jalur subdirektori tidak mempengaruhi cara subagent diidentifikasi atau dipanggil, karena identitas hanya berasal dari bidang frontmatter `name`.

Jaga nilai `name` tetap unik di seluruh pohon: jika dua file dalam direktori `.claude/agents/` yang sama, termasuk subfolder-nya, mendeklarasikan nama yang sama, Claude Code memuat hanya satu dari mereka, dipilih berdasarkan urutan pembacaan sistem file daripada prioritas yang terdokumentasi. Di seluruh direktori proyek bersarang, definisi yang paling dekat dengan direktori kerja menang, seperti dijelaskan di atas. Pemeriksaan setup [`/doctor`](/docs/id/commands#all-commands) melaporkan file dalam direktori yang sama yang berbagi nama dan menyarankan untuk mengganti nama atau menghapus semua kecuali satu. Sebelum v2.1.205, `/doctor` membuka layar diagnostik yang mencantumkan duplikat dan menunjukkan definisi mana yang aktif.

Direktori `agents/` plugin juga dipindai secara rekursif. Tidak seperti cakupan proyek dan pengguna, subfolder di dalam direktori `agents/` plugin menjadi bagian dari [pengenal yang dibatasi cakupan](#invoke-subagents-explicitly): file di `agents/review/security.md` dalam plugin `my-plugin` terdaftar sebagai `my-plugin:review:security`.

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

Flag `--agents` menerima JSON dengan bidang `prompt` ditambah bidang [frontmatter](#supported-frontmatter-fields) ini: `description`, `tools`, `disallowedTools`, `model`, `permissionMode`, `mcpServers`, `hooks`, `maxTurns`, `skills`, `initialPrompt`, `memory`, `effort`, `background`, `omitClaudeMd`, dan `isolation`. Gunakan `prompt` untuk prompt sistem, setara dengan badan markdown dalam subagent berbasis file. `color` dan `experimental` tidak diterima di sini dan diabaikan daripada ditolak.

Setiap kunci tingkat atas dalam JSON adalah nama agen. Jangan mulai nama dengan `-`.

Untuk apa yang Claude Code lakukan dengan nilai yang tidak dapat dimuat, dan flag serta variabel lingkungan yang melewati pemeriksaan itu, lihat [`Invalid --agents configuration`](/docs/id/errors#invalid-agents-configuration).

**Subagent terkelola** digunakan oleh administrator organisasi. Tempatkan file markdown dalam `.claude/agents/` di dalam [direktori pengaturan terkelola](/docs/id/managed-settings#delivery-mechanisms), menggunakan format frontmatter yang sama dengan subagent proyek dan pengguna. Definisi terkelola mengambil alih subagent proyek dan pengguna dengan nama yang sama.

**Subagent plugin** berasal dari [plugins](/docs/id/plugins/overview) yang telah Anda instal. Mereka dimuat secara otomatis bersama subagent khusus Anda dan muncul dalam typeahead @-mention di bawah nama yang dibatasi cakupan mereka. Lihat [referensi komponen plugin](/docs/id/plugins/components#agents) untuk detail tentang membuat subagent plugin.

<Note>
  Untuk alasan keamanan, subagent plugin tidak mendukung bidang frontmatter `hooks`, `mcpServers`, atau `permissionMode`. Bidang-bidang ini diabaikan saat memuat agen dari plugin. Jika Anda membutuhkannya, salin file agen ke dalam `.claude/agents/` atau `~/.claude/agents/`. Anda juga dapat menambahkan aturan ke [`permissions.allow`](/docs/id/settings-reference#permissions-allow) dalam `settings.json` atau `settings.local.json`, tetapi aturan-aturan ini berlaku untuk seluruh sesi, bukan hanya subagent plugin.
</Note>

Definisi subagent dari salah satu cakupan ini juga tersedia untuk [tim agen](/docs/id/agent-teams#use-subagent-definitions-for-teammates): saat menelurkan rekan kerja, Anda dapat mereferensikan jenis subagent, dan Claude Code menerapkan bagian dari definisi itu ke rekan kerja. Lihat [tim agen](/docs/id/agent-teams#use-subagent-definitions-for-teammates) untuk bagian mana yang berlaku di setiap mode tampilan.

<h3 id="write-subagent-files">
  Tulis file subagent
</h3>

File subagent menggunakan frontmatter YAML untuk konfigurasi, diikuti oleh prompt sistem dalam Markdown:

<Note>
  Claude Code memantau `~/.claude/agents/` dan `.claude/agents/`. Ketika Anda menambah atau mengedit file subagent di disk, atau meminta Claude untuk menulis satu untuk Anda, Claude Code mendeteksi perubahan dalam beberapa detik dan delegasi berikutnya menggunakan definisi yang diperbarui, tanpa perlu restart.

  Tiga kasus masih memerlukan restart:

  * Pemantau hanya mencakup direktori yang ada ketika sesi dimulai, jadi setelah membuat file agen pertama cakupan dalam direktori `agents` baru, restart untuk memuatnya.
  * Claude Code tidak memantau `.claude/agents/` di dalam direktori yang ditambahkan dengan `--add-dir` atau `/add-dir`, jadi setelah menambah atau mengedit subagent di sana, restart untuk memuat perubahan.
  * Sesi yang dimulai dengan `--disable-slash-commands` tidak memantau direktori ini sama sekali.
</Note>

```markdown .claude/agents/code-reviewer.md theme={null}
---
name: code-reviewer
description: Reviews code for quality and best practices
tools: Read, Glob, Grep
model: sonnet
---

You are a code reviewer. When invoked, analyze the code and provide
specific, actionable feedback on quality, security, and best practices.
```

Frontmatter mendefinisikan metadata dan konfigurasi subagent. Badan menjadi prompt sistem yang memandu perilaku subagent. Subagent menerima hanya prompt sistem ini ditambah detail lingkungan dasar seperti direktori kerja, bukan prompt sistem Claude Code lengkap.

Dalam [mode non-interaktif](/docs/id/headless), lewatkan [`--append-subagent-system-prompt`](/docs/id/cli-reference#cli-flags) untuk menambahkan teks Anda ke akhir prompt sistem setiap subagent, subagent bersarang termasuk, terlepas dari [subagent yang bercabang](#fork-the-current-conversation), yang menggunakan kembali prompt percakapan itu sendiri. Memerlukan Claude Code v2.1.205 atau lebih baru. Jika teks Anda terlalu panjang untuk dilewatkan di baris perintah, simpan ke file dan lewatkan jalur dengan `--append-subagent-system-prompt-file` sebagai gantinya. Flag file memerlukan Claude Code v2.1.261 atau lebih baru.

Subagent dimulai di direktori kerja saat ini percakapan utama. Dalam subagent, perintah `cd` tidak bertahan antara panggilan alat Bash atau PowerShell dan tidak mempengaruhi direktori kerja percakapan utama. Untuk memberikan subagent salinan repositori yang terisolasi sebagai gantinya, atur [`isolation: worktree`](#supported-frontmatter-fields).

Subagent dengan `isolation: worktree` menjalankan perintah Bash dan PowerShell-nya di dalam worktree-nya. Perintah yang direktori kerjanya diselesaikan ke checkout utama Anda, misalnya karena direktori worktree dihapus saat subagent berjalan, gagal dengan kesalahan. Sebelum v2.1.203, perintah seperti itu dapat berjalan di checkout utama.

Pemeriksaan direktori kerja ini mencakup seluruh repositori yang berisi direktori tempat Anda meluncurkan Claude Code. Ketika sesi Anda berjalan dalam [worktree](/docs/id/worktrees) yang ditautkan sendiri, pemeriksaan juga mencakup checkout utama yang worktree itu ditautkan darinya. Sebelum v2.1.210, pemeriksaan hanya mencakup direktori peluncuran itu sendiri. Perintah yang direktori kerjanya diselesaikan di tempat lain dalam repositori yang sama, seperti akar repositori ketika Anda meluncurkan Claude Code dari subdirektori monorepo, berjalan di sana sebagai gantinya dari gagal.

Untuk perintah Bash, Claude Code juga memeriksa perintah itu sendiri dalam dua cara:

* Ini memblokir perintah yang mengarahkan git ke checkout utama.
* Ini menolak perintah ketika tidak dapat memverifikasi dari teks perintah bahwa git apa pun yang dijalankan perintah tetap di dalam worktree, misalnya ketika nama perintah dihitung saat runtime.

Vektor pengalihan dan aturan bentuk tercantum di bawah [Bagaimana Claude Code memberlakukan isolasi](/docs/id/worktrees#how-claude-code-enforces-isolation). Perintah PowerShell hanya mendapatkan pemeriksaan direktori kerja.

Perintah [Monitor](/docs/id/tools-reference#monitor-tool) melalui pemeriksaan direktori kerja dan konten perintah yang sama dengan perintah Bash.

Ketika percakapan utama itu sendiri berjalan terisolasi dalam worktree, Claude Code menerapkan pemeriksaan yang sama ke sesi dan ke setiap subagent yang dihasilkannya, termasuk subagent tanpa `isolation: worktree`; lihat [Bagaimana Claude Code memberlakukan isolasi](/docs/id/worktrees#how-claude-code-enforces-isolation).

<h3 id="supported-frontmatter-fields">
  Referensi frontmatter
</h3>

Konfigurasi subagent dengan frontmatter YAML [frontmatter](/docs/id/glossary#frontmatter) antara penanda `---` di bagian atas file-nya, dan tulis prompt sistem-nya sebagai Markdown setelah penutup `---`. Hanya `name` dan `description` yang diperlukan.

Nama bidang multi-kata menggunakan camelCase, seperti `maxTurns` dan `disallowedTools`, dan harus cocok dengan tabel dengan tepat: Claude Code mengabaikan bidang yang tidak dikenalinya tanpa melaporkan kesalahan. Untuk mengetahui mengapa file subagent tidak dimuat, lihat [File subagent yang Claude Code lewati](#subagent-files-claude-code-skips).

| Bidang            | Diperlukan | Deskripsi                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| :---------------- | :--------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`            | Ya         | Pengenal unik, seperti `code-reviewer` atau `reviewer-v2`. [Hooks](/docs/id/hooks#subagentstart) menerima nilai ini sebagai `agent_type`. Nama file tidak harus cocok. Nama tidak dapat berisi `:`, yang dicadangkan untuk [pengenal yang dibatasi cakupan plugin](/docs/id/plugins/overview) seperti `my-plugin:reviewer`. Claude Code tidak memuat file yang namanya berisi satu dan mencatat kesalahan ke debug log. Sebelum v2.1.218, nama seperti itu diterima                                                       |
| `description`     | Ya         | Kapan Claude harus mendelegasikan ke subagent ini                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `tools`           | Tidak      | [Alat](#available-tools) yang dapat digunakan subagent, sebagai string yang dipisahkan koma seperti `Read, Grep, Bash` atau daftar YAML. Mewarisi setiap alat yang tersedia untuk subagent jika dihilangkan. Jika tidak ada entri dalam daftar yang diselesaikan ke alat, subagent biasanya [gagal diluncurkan](/docs/id/errors#agent-would-be-spawned-with-zero-tools) dengan kesalahan yang menyebutkan entri. Untuk memuat Skills ke dalam konteks, gunakan bidang `skills` daripada mencantumkan `Skill` di sini |
| `disallowedTools` | Tidak      | Alat untuk ditolak, dihapus dari daftar yang diwarisi atau ditentukan. Format yang sama dengan `tools`. Entri dengan penentu, seperti `Bash(git push *)`, masih [menghapus seluruh alat](#available-tools)                                                                                                                                                                                                                                                                                                      |
| `model`           | Tidak      | [Model](#choose-a-model) untuk digunakan: `sonnet`, `opus`, `haiku`, `fable`, ID model lengkap seperti `claude-opus-5-5`, atau `inherit`. Ketika Anda menghilangkannya, Claude Code memilih model dalam [urutan model subagent](#choose-a-model)                                                                                                                                                                                                                                                                |
| `permissionMode`  | Tidak      | [Mode izin](#permission-modes): `default`, `acceptEdits`, `auto`, `dontAsk`, `bypassPermissions`, `plan`, atau `manual` sebagai alias untuk `default`. Alias `manual` memerlukan Claude Code v2.1.200 atau lebih baru. Diabaikan untuk [subagent plugin](#choose-the-subagent-scope)                                                                                                                                                                                                                            |
| `maxTurns`        | Tidak      | Jumlah maksimum putaran agentic sebelum subagent berhenti. Ketika subagent mencapai batas, Claude Code mengembalikan outputnya ditandai sebagai parsial, dan Claude dapat [melanjutkannya](#resume-subagents) untuk terus. Penandaan parsial memerlukan Claude Code v2.1.246 atau lebih baru                                                                                                                                                                                                                    |
| `skills`          | Tidak      | [Skills](/docs/id/skills) untuk dimuat sebelumnya ke dalam konteks subagent saat startup. Konten skill lengkap disuntikkan, bukan hanya deskripsi. Subagent masih dapat memanggil project, user, dan plugin skills yang tidak tercantum melalui alat Skill                                                                                                                                                                                                                                                           |
| `mcpServers`      | Tidak      | [MCP servers](/docs/id/mcp) tersedia untuk subagent ini. Setiap entri adalah nama server yang mereferensikan server yang sudah dikonfigurasi (misalnya, `"slack"`) atau definisi inline dengan nama server sebagai kunci dan [konfigurasi MCP server](/docs/id/mcp#installing-mcp-servers) lengkap sebagai nilai. Diabaikan untuk [subagent plugin](#choose-the-subagent-scope)                                                                                                                                           |
| `hooks`           | Tidak      | [Lifecycle hooks](#define-hooks-for-subagents) yang dibatasi pada subagent ini. Diabaikan untuk [subagent plugin](#choose-the-subagent-scope)                                                                                                                                                                                                                                                                                                                                                                   |
| `memory`          | Tidak      | [Cakupan memori persisten](#enable-persistent-memory): `user`, `project`, atau `local`. Memungkinkan pembelajaran lintas sesi                                                                                                                                                                                                                                                                                                                                                                                   |
| `background`      | Tidak      | Atur ke `true` untuk menjaga subagent ini di latar belakang bahkan ketika Claude meminta untuk menjalankannya di latar depan. Ketika [fork mode](#turn-fork-mode-on-or-off) aktif, Claude Code sudah menjalankan subagent yang Claude hasilkan [di latar belakang](#run-subagents-in-foreground-or-background)                                                                                                                                                                                                  |
| `omitClaudeMd`    | Tidak      | Atur ke `true` untuk meluncurkan subagent ini tanpa file user, project, dan local CLAUDE.md; [file kebijakan terkelola](/docs/id/memory#how-claude-md-files-load) masih dimuat, kecuali untuk [subagent terkelola](#choose-the-subagent-scope). Gunakan untuk subagent yang mengambil semua yang mereka butuhkan dari [prompt delegasi](#what-loads-at-startup). Diabaikan ketika agen berjalan sebagai agen sesi utama melalui `--agent` atau pengaturan `agent`. Memerlukan Claude Code v2.1.271 atau lebih baru   |
| `effort`          | Tidak      | Tingkat usaha ketika subagent ini aktif. Menimpa tingkat usaha sesi. Default: mewarisi dari sesi. Opsi: `low`, `medium`, `high`, `xhigh`, `max`; tingkat yang tersedia tergantung pada model                                                                                                                                                                                                                                                                                                                    |
| `isolation`       | Tidak      | Atur ke `worktree` untuk menjalankan subagent dalam [git worktree](/docs/id/worktrees) sementara, memberikannya salinan repositori yang terisolasi yang bercabang secara default dari [cabang default](/docs/id/worktrees#choose-the-base-branch) Anda daripada `HEAD` sesi induk. Worktree secara otomatis dibersihkan jika subagent tidak membuat perubahan                                                                                                                                                             |
| `color`           | Tidak      | Warna tampilan untuk subagent dalam daftar tugas dan transkrip. Menerima `red`, `blue`, `green`, `yellow`, `purple`, `orange`, `pink`, atau `cyan`                                                                                                                                                                                                                                                                                                                                                              |
| `initialPrompt`   | Tidak      | Auto-submitted sebagai putaran pengguna pertama ketika agen ini berjalan sebagai agen sesi utama (melalui `--agent` atau pengaturan `agent`). [Commands](/docs/id/commands) dan [skills](/docs/id/skills) diproses. Ditambahkan di depan prompt yang disediakan pengguna apa pun. Diabaikan untuk [subagent plugin](#choose-the-subagent-scope)                                                                                                                                                                           |
| `experimental`    | Tidak      | Peta opsi eksperimental. Atur kunci `cacheTtl`-nya ke `5m` atau `1h` untuk memilih [lifetime cache prompt](/docs/id/prompt-caching#choose-the-ttl-yourself) untuk permintaan subagent ini, di tempat frontmatter dalam [prioritas lifetime cache](/docs/id/prompt-caching#choose-the-ttl-yourself). Claude Code mengabaikan nilai lain apa pun, mengabaikan `1h` saat langganan Claude Anda menggunakan kredit penggunaan, dan membaca bidang hanya dari file subagent. Memerlukan Claude Code v2.1.248 atau lebih baru   |

Tulis `cacheTtl` di dalam peta `experimental`, bukan di tingkat atas frontmatter.

```yaml theme={null}
---
name: repo-auditor
description: Audits a large repository and reports what it finds
experimental:
  cacheTtl: 1h
---
```

<h4 id="subagent-files-claude-code-skips">
  File subagent yang Claude Code lewati
</h4>

Claude Code melewati file dalam direktori `agents` proyek, pengguna, atau terkelola, atau dalam satu di bawah direktori yang Anda tambahkan dengan `--add-dir`, tanpa melaporkannya dalam sesi, ketika frontmatter memiliki salah satu masalah ini:

* **Tidak ada `name`**: Claude Code memperlakukan file sebagai dokumentasi yang disimpan di samping agen Anda.
* **Pembukaan `---` yang bukan baris pertama file**: Claude Code membaca file sebagai tidak memiliki frontmatter dan memperlakukannya sebagai dokumentasi.
* **`name` yang dimulai dengan `-` atau berisi `:`**: Claude Code melewati file dan menulis kesalahan ke debug log. Lihat baris `name` dalam tabel di atas.
* **`name` tetapi tidak ada `description`**: Claude Code melewati file dan menulis alasannya ke debug log.
* **YAML yang tidak diuraikan**: Claude Code membaca tidak ada bidang dari file, melewatinya, dan menulis kesalahan penguraian ke debug log.

Untuk melihat debug log, jalankan Claude Code dengan `--debug`.

[Subagent plugin](/docs/id/plugins/components#agents) yang frontmatter-nya tidak memiliki `name` atau tidak diuraikan masih dimuat, di bawah nama file-nya.

<h5 id="check-an-agents-directory-before-a-session">
  Periksa direktori `agents` sebelum sesi
</h5>

Untuk menemukan file dalam direktori `agents` yang frontmatter-nya tidak diuraikan, jalankan `claude plugin validate` terhadap direktori, misalnya `.claude/agents` atau `~/.claude/agents`. Claude Code memeriksa hanya [direktori yang Anda beri nama](/docs/id/plugins/cli-reference#validate-a-directory), dan tidak menandai file yang frontmatter-nya diuraikan tetapi tidak memiliki `name`. Memerlukan Claude Code v2.1.233 atau lebih baru.

<h3 id="choose-a-model">
  Pilih model
</h3>

Bidang `model` mengontrol model mana yang digunakan subagent:

* **Alias model**: gunakan salah satu alias yang tersedia: `sonnet`, `opus`, `haiku`, atau `fable`
* **ID model lengkap**: gunakan ID model lengkap seperti `claude-opus-5-5` atau `claude-sonnet-5`. Menerima nilai yang sama dengan flag `--model`
* **inherit**: gunakan model yang sama dengan percakapan utama

Ketika Claude memanggil subagent, Claude juga dapat melewatkan parameter `model` untuk invokasi spesifik itu. Claude Code menyelesaikan model subagent dalam urutan ini:

1. Parameter `model` per-invokasi
2. Frontmatter `model` definisi subagent, di mana `inherit` memilih model percakapan utama
3. Variabel lingkungan [`CLAUDE_CODE_SUBAGENT_MODEL`](/docs/id/model-config#environment-variables), ketika Anda mengaturnya ke alias model atau ID model
4. Model percakapan utama

Dalam dua kasus, alias keluarga seperti `opus` dalam parameter per-invokasi atau frontmatter diselesaikan ke model percakapan utama sebagai gantinya dari [versi yang ditunjuk alias](/docs/id/model-config#model-aliases):

* **Model percakapan utama milik keluarga itu**: subagent berjalan pada model yang tepat percakapan utama, termasuk apa pun `[1m]` suffix, sehingga mendapatkan [jendela konteks diperpanjang](/docs/id/model-config#extended-context) yang sama dengan percakapan utama.
* **Claude Code tidak dapat mengetahui keluarga model percakapan utama, pada [penyedia selain API Anthropic](/docs/id/third-party-integrations)**: ini dapat terjadi dengan [application inference profile ARN](/docs/id/amazon-bedrock#iam-configuration) pada Amazon Bedrock yang Claude Code belum diselesaikan ke model pendukung. Kasus ini hanya mencakup alias `opus`, dan tidak berlaku ketika Anda mengatur [`ANTHROPIC_DEFAULT_OPUS_MODEL`](/docs/id/model-config#environment-variables), karena `opus` kemudian diselesaikan ke model yang Anda atur.

Alias dalam `CLAUDE_CODE_SUBAGENT_MODEL` selalu diselesaikan ke versi yang ditunjuk alias, bahkan ketika itu menyebutkan keluarga percakapan utama.

Mengatur `CLAUDE_CODE_SUBAGENT_MODEL` dengan sendirinya tidak mengubah model yang dijalankan subagent Explore dan Plan bawaan. Untuk mengubahnya, lihat [Jalankan setiap subagent pada satu model](#run-every-subagent-on-one-model).

Sebelum v2.1.251, `CLAUDE_CODE_SUBAGENT_MODEL` datang pertama dalam urutan ini dan menimpa parameter per-invokasi dan frontmatter, termasuk `model: inherit`.

Mengatur variabel ke `inherit` sama dengan membiarkannya tidak diatur. Sebelum v2.1.196, nilai itu memaksa subagent ke model percakapan utama dan mengabaikan sumber lain.

Claude Code memeriksa parameter per-invokasi, frontmatter, dan nilai variabel lingkungan terhadap daftar allowlist [`availableModels`](/docs/id/model-config#restrict-model-selection) organisasi Anda. Untuk nilai yang diblokir, Claude Code mengganti model lain:

* Ketika nilai yang diblokir adalah alias keluarga seperti `opus`, Claude Code menjalankan subagent pada versi terbaru keluarga itu yang daftar allowlist izinkan, mengikuti [aturan substitusi dan cakupan penyedia](/docs/id/model-config#restrict-model-selection) yang sama seperti `/model`. Sebelum v2.1.222, Claude Code menjalankan subagent pada model yang diwarisi untuk alias keluarga yang diblokir juga.
* Untuk nilai yang diblokir lainnya, pada penyedia di mana substitusi itu tidak beroperasi, atau ketika daftar allowlist tidak mengizinkan versi keluarga apa pun, Claude Code menjalankan subagent pada model yang diwarisi sebagai gantinya. Jika Anda mengatur `CLAUDE_CODE_SUBAGENT_MODEL`, Claude Code mencoba model itu terlebih dahulu, di bawah aturan yang sama ini.

Dalam sesi interaktif, Claude Code menampilkan peringatan yang menyebutkan model yang diminta dan model yang dijalankan subagent, untuk substitusi apa pun.

Untuk memeriksa model mana yang dijalankan subagent, jalankan [`/tasks`](/docs/id/commands). Claude Code menyebutkan model pada baris subagent, dan menambahkan [tingkat usaha](/docs/id/model-config#adjust-effort-level) ketika definisi subagent, atau skill yang dihasilkannya, menetapkan [`effort`](#supported-frontmatter-fields). Memerlukan Claude Code v2.1.242 atau lebih baru.

Parameter `model` per-invokasi juga berlaku ketika subagent [dilanjutkan atau dikirim pesan lanjutan](#resume-subagents), sehingga subagent tetap pada model itu. Sebelum v2.1.211, melanjutkan menghapus nilai per-invokasi dan subagent kembali ke bidang `model` definisinya atau, tanpanya, model percakapan utama.

Sejak v2.1.198, subagent juga mewarisi konfigurasi [extended thinking](/docs/id/model-config#extended-thinking) percakapan utama: jika thinking aktif dalam sesi Anda, itu aktif untuk subagent, dan jika itu mati, itu tetap mati. Tidak ada pengaturan thinking per-subagent. Sebelum v2.1.198, subagent berjalan dengan extended thinking dinonaktifkan terlepas dari pengaturan percakapan utama.

<h4 id="run-every-subagent-on-one-model">
  Jalankan setiap subagent pada satu model
</h4>

`CLAUDE_CODE_SUBAGENT_MODEL` adalah default, jadi definisi subagent atau model yang Claude lewatkan masih mengambil alih darinya. Untuk menerapkan satu model ke setiap subagent, [rekan kerja](/docs/id/agent-teams#specify-teammates-and-models), dan [agen workflow](/docs/id/workflows), juga atur `CLAUDE_CODE_SUBAGENT_MODEL_FORCE` ke `1`. Memerlukan Claude Code v2.1.257 atau lebih baru.

* Jika Anda mengatur kedua variabel, subagent berjalan pada model dalam `CLAUDE_CODE_SUBAGENT_MODEL`.
* Jika Anda hanya mengatur `CLAUDE_CODE_SUBAGENT_MODEL_FORCE`, subagent berjalan pada model percakapan utama.

Misalnya, untuk menjalankan setiap subagent pada Haiku, atur kedua variabel dalam blok `env` [file pengaturan](/docs/id/settings):

```json theme={null}
{
  "env": {
    "CLAUDE_CODE_SUBAGENT_MODEL": "haiku",
    "CLAUDE_CODE_SUBAGENT_MODEL_FORCE": "1"
  }
}
```

Untuk memeriksa bahwa pengaturan berlaku, jalankan [`/tasks`](/docs/id/commands) saat subagent berjalan. Baris subagent menunjukkan model yang dijalankannya.

Sementara `CLAUDE_CODE_SUBAGENT_MODEL_FORCE` [aktif](/docs/id/env-vars), Claude Code mengabaikan bidang `model` dari setiap definisi subagent, termasuk subagent Explore dan Plan bawaan, dan Claude tidak dapat melewatkan model ketika memulai subagent. Dua jenis subagent masih berjalan pada model percakapan utama:

* [Fork](#fork-the-current-conversation)
* [Skill yang berjalan dalam subagent](/docs/id/skills#run-skills-in-a-subagent) dengan `model: inherit`

Ketika Anda hanya mengatur `CLAUDE_CODE_SUBAGENT_MODEL_FORCE`, subagent Explore bawaan menyimpan [batas modelnya](#built-in-subagents).

<h3 id="control-subagent-capabilities">
  Kontrol kemampuan subagent
</h3>

Anda dapat mengontrol apa yang dapat dilakukan subagent melalui akses alat, mode izin, dan aturan bersyarat.

<h4 id="available-tools">
  Alat yang tersedia
</h4>

Subagent mewarisi [alat bawaan](/docs/id/tools-reference) dan alat MCP yang tersedia dalam percakapan utama, dipersempit oleh dua filter: yang pertama menghapus daftar singkat alat dari setiap subagent, dan yang kedua mengurangi set alat bawaan untuk subagent yang berjalan di [latar belakang](#run-subagents-in-foreground-or-background), yang merupakan default. Di macOS, Linux, dan WSL, subagent juga dapat menerima alat Glob dan Grep ketika percakapan utama tidak memilikinya, seperti dijelaskan di bawah [Perilaku alat Glob](/docs/id/tools-reference#glob-tool-behavior). [Fork](#fork-the-current-conversation) melewati kedua filter dan menerima kumpulan alat percakapan utama yang tepat. Filter pertama menghapus alat ini, bahkan ketika tercantum dalam bidang `tools`:

* `Agent`, ketika subagent berada di [batas kedalaman](#let-subagents-spawn-their-own-subagents); dalam [fork](#fork-the-current-conversation) alat tetap tercantum tetapi mengembalikan kesalahan sebagai gantinya dari peneluran
* `AskUserQuestion`
* `EndConversation`, yang hanya dapat mengakhiri percakapan utama; lihat [Perilaku alat EndConversation](/docs/id/tools-reference#endconversation-tool-behavior)
* `EnterPlanMode`
* `ExitPlanMode`, kecuali [`permissionMode`](#permission-modes) subagent adalah `plan`
* `ScheduleWakeup`
* `WaitForMcpServers`
* `Workflow`

Filter kedua berlaku untuk subagent yang berjalan di latar belakang. Terlepas dari `Agent` dan `ExitPlanMode`, yang mengikuti kondisi filter pertama di mana pun subagent berjalan, subagent latar belakang menyimpan setiap alat MCP tetapi hanya alat bawaan ini: `Read`, `Grep`, `Glob`, `LSP`, `Bash`, `PowerShell`, `Edit`, `Write`, `NotebookEdit`, `WebFetch`, `WebSearch`, `TodoWrite`, `Skill`, `ToolSearch`, `EnterWorktree`, `ExitWorktree`, `Monitor`, `TaskStop`, `SendMessage`, dan `Artifact`, plus [`SubagentHandback`](/docs/id/tools-reference) untuk subagent yang melaporkan melaluinya. Claude Code menghapus setiap alat bawaan lainnya dari subagent latar belakang, baik yang diwarisi atau tercantum dalam bidang `tools`, sehingga definisi yang sama dapat diselesaikan ke alat berbeda di latar depan dan latar belakang. Penghapusan tidak melaporkan kesalahan kecuali itu meninggalkan daftar `tools` [diselesaikan ke tidak ada apa-apa](/docs/id/errors#agent-would-be-spawned-with-zero-tools).

Sebelum v2.1.280, subagent latar belakang tidak dapat menggunakan `LSP`.

[`ListAgents`](/docs/id/cross-session-messaging) mengikuti filter ini seperti alat bawaan apa pun: subagent latar depan mewarisnya dalam sesi di mana cross-session messaging diaktifkan, dan subagent latar belakang tidak menyimpannya.

Rekan kerja dalam [tim agen](/docs/id/agent-teams) juga menyimpan alat tugas dan alat cron: `TaskCreate`, `TaskGet`, `TaskList`, `TaskUpdate`, `CronCreate`, `CronDelete`, dan `CronList`.

Dalam [sesi tanpa alat Task](/docs/id/tools-reference#task-tool-availability), Claude Code tidak menyediakan alat tugas ke subagent juga, bahkan ketika subagent menjalankan model berbeda. Rekan kerja dalam proses mengikuti sesi Anda dengan cara yang sama, sementara rekan kerja dalam [split pane](/docs/id/agent-teams#choose-a-display-mode) miliknya sendiri berjalan sebagai proses Claude Code terpisah, jadi modelnya sendiri yang memutuskan.

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
description: Inherits the available tools except file writes
disallowedTools: Write, Edit
---
```

Jika keduanya diatur, `disallowedTools` diterapkan terlebih dahulu, kemudian `tools` diselesaikan terhadap kumpulan yang tersisa. Alat yang tercantum di keduanya dihapus.

Ketika tidak ada dalam daftar `tools` yang diselesaikan ke alat, misalnya karena setiap entri salah eja atau menyebutkan alat yang tidak tersedia untuk subagent, Claude Code biasanya menolak untuk meluncurkan subagent dan alat Agent mengembalikan kesalahan yang menyebutkan entri yang tidak diselesaikan; lihat [Agent would be spawned with zero tools](/docs/id/errors#agent-would-be-spawned-with-zero-tools) untuk pesan dan cara memperbaiki setiap entri. Sebelum v2.1.208, subagent itu diluncurkan tanpa alat dan dapat mengembalikan hasil yang kosong atau membingungkan.

Kedua bidang menerima pola tingkat server MCP selain nama alat yang tepat: `mcp__<server>` atau `mcp__<server>__*` memberikan atau menghapus setiap alat dari server bernama. Dalam `disallowedTools`, `mcp__*` juga menghapus setiap alat MCP dari server apa pun. Contoh ini menghapus setiap alat dari server MCP `github` sambil menyimpan alat dari server lain dan alat bawaan dalam kumpulannya:

```yaml theme={null}
---
name: local-only
description: Inherits every tool except those from the github MCP server
disallowedTools: mcp__github
---
```

Entri `disallowedTools` dengan penentu, seperti `Bash(git push *)`, masih menghapus seluruh alat dari subagent, bukan hanya perintah yang cocok. Untuk menyimpan Bash dan memblokir perintah tertentu, tambahkan [aturan penolakan Bash](/docs/id/permissions#bash) seperti `Bash(git push *)` ke `permissions.deny` dalam pengaturan Anda. Aturan berlaku untuk percakapan utama dan subagent.

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

Untuk mengizinkan peneluran subagent apa pun tanpa pembatasan, gunakan `Agent` tanpa tanda kurung:

```yaml theme={null}
tools: Agent, Read, Bash
```

Jika Anda menghilangkan `Agent` dari daftar `tools` sepenuhnya, agen tidak dapat menelurkan subagent apa pun dengan alat Agent.

Sintaks allowlist `Agent(agent_type)` hanya berlaku untuk agen yang berjalan sebagai thread utama dengan `claude --agent`. Dalam definisi subagent, mencantumkan `Agent` dalam `tools` memungkinkan subagent itu untuk menelurkan subagent bersarang sementara [batas kedalaman](#let-subagents-spawn-their-own-subagents) mengizinkannya, tetapi daftar jenis apa pun di dalam tanda kurung diabaikan.

<h4 id="scope-mcp-servers-to-a-subagent">
  Cakupan MCP servers ke subagent
</h4>

Gunakan bidang `mcpServers` untuk memberikan subagent akses ke [MCP](/docs/id/mcp) servers yang tidak tersedia dalam percakapan utama. Server inline yang ditentukan di sini terhubung ketika subagent dimulai, tunduk pada [aturan kepercayaan untuk folder file agen](#inline-server-trust), dan terputus ketika selesai. Referensi string berbagi koneksi sesi induk.

<Note>
  Bidang `mcpServers` berlaku dalam kedua konteks di mana file agen dapat berjalan:

  * Sebagai subagent, dihasilkan melalui alat Agent atau @-mention
  * Sebagai sesi utama, diluncurkan dengan [`--agent`](#invoke-subagents-explicitly) atau pengaturan `agent`

  Ketika agen adalah sesi utama, definisi server inline terhubung saat startup bersama server dari [`.mcp.json`](/docs/id/mcp) dan file pengaturan, di bawah [aturan kepercayaan yang sama untuk folder file agen](#inline-server-trust). Dalam `/mcp`, server jarak jauh (HTTP atau SSE) yang telah Anda gunakan sebelumnya dapat menampilkan [status `cached`](/docs/id/mcp#managing-your-servers) sebagai gantinya; Claude Code menghubungkannya ketika Claude pertama kali memanggil salah satu alatnya.
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

<span id="inline-server-trust" />Claude Code memuat server inline dari file agen dalam direktori `.claude/agents/` proyek Anda, atau dalam direktori `.claude/agents/` direktori `--add-dir`, hanya setelah Anda [mempercayai folder tempat file agen berasal](/docs/id/permissions#what-runs-before-you-trust-a-folder). Sebelum v2.1.238, Claude Code memuat server ini tanpa memeriksa kepercayaan.

* **Kepercayaan yang tidak dihitung**: kepercayaan folder induk, dan kepercayaan otomatis yang sesi `-p` atau SDK dapatkan untuk [hooks dalam file pengaturan](/docs/id/permissions#what-runs-before-you-trust-a-folder)
* **Sampai saat itu**: Claude Code melewati setiap server inline dalam file agen itu dan menulis kunci `projects["<path>"].hasTrustDialogAccepted` yang tepat untuk `~/.claude.json` ke debug log
* **Direktori `--add-dir`**: direktori di luar repositori workspace terpercaya Anda memerlukan entri kepercayaan sendiri, karena file `.claude/agents/` miliknya tidak mewarisi kepercayaan workspace Anda

Claude Code memuat dua jenis server tanpa memeriksa kepercayaan untuk folder tempat file agen berasal:

* Nama yang mereferensikan server yang sudah Anda konfigurasi
* Server inline dalam file agen dari `~/.claude/agents/`, dalam satu yang Anda lewatkan dengan `--agents` atau opsi SDK `agents`, atau dalam satu yang pengaturan terkelola sediakan

Pembatasan MCP yang berlaku untuk sesi utama juga mencakup server yang dideklarasikan dalam frontmatter subagent:

* [`--strict-mcp-config`](/docs/id/cli-reference) dan [`--bare`](/docs/id/cli-reference)
* [Konfigurasi MCP terkelola Enterprise](/docs/id/managed-mcp)
* [`allowedMcpServers` dan `deniedMcpServers` policies](/docs/id/managed-mcp#policy-based-control-with-allowlists-and-denylists)

Ketika salah satu dari ini memblokir server, Claude Code melewatinya dan menampilkan peringatan yang menyebutkan server yang diblokir.

Pembatasan pengaturan terkelola berlaku untuk setiap subagent terlepas dari cara pendefinisiannya. `--strict-mcp-config` tidak memfilter server yang Anda lewatkan secara inline melalui `--agents` atau opsi SDK `agents`, karena itu adalah input pemanggil eksplisit.

<h4 id="permission-modes">
  Mode izin
</h4>

Atur `permissionMode` untuk memilih mode izin yang dijalankan subagent. Gunakan nilai konfigurasi mode, jadi mode Manual adalah `default`. Jika Anda membiarkannya tidak diatur, subagent mewarisi mode percakapan utama, yang dimulai sebagai [auto mode](/docs/id/permission-modes#eliminate-prompts-with-auto-mode) pada paket Pro, Max, dan Team kecuali pengaturan Anda atau organisasi Anda mengubahnya.

Mode izin percakapan utama memutuskan apakah Claude Code menggunakan nilai yang Anda atur:

* Ketika percakapan utama berada dalam `bypassPermissions`, `acceptEdits`, atau [auto mode](/docs/id/permission-modes#eliminate-prompts-with-auto-mode), subagent berjalan dalam mode yang sama dan Claude Code mengabaikan `permissionMode` yang Anda atur. Di bawah auto mode, pengklasifikasi mengevaluasi panggilan alat subagent dengan aturan blok dan izin percakapan utama. Ketika subagent selesai, pengklasifikasi juga meninjau pekerjaannya dan laporannya yang final sebelum laporan disampaikan, seperti [Bagaimana auto mode menangani subagent](/docs/id/permission-modes#eliminate-prompts-with-auto-mode) menjelaskan.
* Ketika percakapan utama berada dalam mode `default`, `dontAsk`, atau `plan`, subagent berjalan dalam mode izin yang Anda atur, kecuali `bypassPermissions`. Subagent yang mendeklarasikan `bypassPermissions` menyimpan mode percakapan utama sebagai gantinya. Pengecualian `bypassPermissions` memerlukan Claude Code v2.1.267 atau lebih baru.

`permissionMode` menerima nilai ini, dan `manual` sebagai alias untuk `default`:

| Mode                | Perilaku                                                                                                                                                                                                                                                                                                                                                                                                    |
| :------------------ | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `default`           | Mode manual: meminta izin                                                                                                                                                                                                                                                                                                                                                                                   |
| `acceptEdits`       | Auto-terima edit file dan perintah sistem file umum untuk jalur di direktori kerja atau `additionalDirectories`                                                                                                                                                                                                                                                                                             |
| `auto`              | [Auto mode](/docs/id/permission-modes#eliminate-prompts-with-auto-mode): pengklasifikasi latar belakang meninjau perintah dan penulisan direktori yang dilindungi                                                                                                                                                                                                                                                |
| `dontAsk`           | Auto-tolak prompt izin. Alat yang secara eksplisit diizinkan masih berfungsi; `AskUserQuestion`, alat MCP yang ditandai [`requiresUserInteraction`](/docs/id/mcp#require-approval-for-a-specific-tool), dan alat konektor [organisasi Anda atur ke `ask`](/docs/id/mcp#organization-controls-on-connector-tools) dalam sesi di mana pengaturan itu mencapai Claude Code ditolak bahkan jika Anda telah mengizinkannya |
| `bypassPermissions` | [Lewati prompt izin](/docs/id/permission-modes#skip-all-checks-with-bypasspermissions-mode). Subagent berjalan dalam mode ini hanya ketika percakapan utama melakukannya                                                                                                                                                                                                                                         |
| `plan`              | Plan mode (eksplorasi hanya-baca)                                                                                                                                                                                                                                                                                                                                                                           |

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

Anda tidak dapat preload skills yang menetapkan [`disable-model-invocation: true`](/docs/id/skills#control-who-invokes-a-skill), karena preloading menarik dari set skills yang sama yang dapat diinvokasi Claude. Ini termasuk skill `/verify` bundel: hanya Anda yang dapat menjalankannya, jadi itu juga tidak dapat dimuat sebelumnya.

Jika skill yang tercantum hilang atau dinonaktifkan, misalnya oleh kebijakan organisasi Anda, Claude Code melewatinya dan mencatat peringatan ke debug log.

<Note>
  Ini adalah kebalikan dari [menjalankan skill dalam subagent](/docs/id/skills#run-skills-in-a-subagent). Dengan `skills` dalam subagent, subagent mengontrol prompt sistem dan memuat konten skill. Dengan `context: fork` dalam skill, konten skill disuntikkan ke dalam agen yang Anda tentukan. Dalam kedua kasus subagent dimulai tanpa riwayat percakapan Anda.
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

Memori subagent adalah bagian dari [auto memory](/docs/id/memory#auto-memory): jika Anda mematikan auto memory, dengan pengaturan `autoMemoryEnabled` atau `CLAUDE_CODE_DISABLE_AUTO_MEMORY`, bidang `memory` tidak berpengaruh dan subagent diluncurkan tanpa instruksi memori atau akses alat memori yang dijelaskan di bawah.

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

Di macOS dan Linux, buat skrip dapat dieksekusi, atau hook gagal sebagai gantinya dari memblokir apa pun:

```bash theme={null}
chmod +x ./scripts/validate-readonly-query.sh
```

Untuk menguji aturan, minta subagent untuk menjalankan pernyataan `UPDATE`: skrip keluar dengan kode 2, Claude Code memblokir perintah, dan subagent melihat pesan `Blocked: Only SELECT queries are allowed`.

Lihat [Hook input](/docs/id/hooks#pretooluse-input) untuk skema input lengkap dan [exit codes](/docs/id/hooks#exit-code-output) untuk bagaimana kode keluar mempengaruhi perilaku. Di Windows, tulis skrip hook dalam PowerShell dan tambahkan `shell: powershell` ke entri hook seperti yang ditunjukkan dalam [menjalankan hooks dalam PowerShell](/docs/id/hooks#windows-powershell-tool).

<h4 id="disable-specific-subagents">
  Nonaktifkan subagent tertentu
</h4>

Anda dapat mencegah Claude menggunakan subagent tertentu dengan menambahkannya ke array `deny` dalam [pengaturan](/docs/id/settings-reference#permission-settings) Anda. Gunakan format `Agent(subagent-name)` di mana `subagent-name` cocok dengan bidang nama subagent.

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

* **Dalam frontmatter subagent**: tentukan hooks yang hanya berjalan saat subagent tertentu itu aktif
* **Dalam `settings.json`**: tentukan hooks tingkat sesi yang juga terjadi di dalam subagent. Peristiwa alat seperti `PreToolUse` dan `PostToolUse` terjadi untuk panggilan alat subagent dengan cara yang sama seperti dalam percakapan utama, dan `SubagentStart` dan `SubagentStop` terjadi ketika subagent dimulai atau selesai

Hooks dari [file pengaturan, pengaturan kebijakan terkelola, dan plugins](/docs/id/hooks#hook-locations) semuanya berlaku di dalam subagent, jadi hook `PreToolUse` dalam `settings.json` juga berjalan sebelum setiap alat yang digunakan subagent.

<h4 id="hooks-in-subagent-frontmatter">
  Hooks dalam frontmatter subagent
</h4>

Tentukan hooks langsung dalam file markdown subagent. Hooks ini hanya berjalan saat subagent spesifik itu aktif dan dibersihkan saat selesai.

<Note>
  Frontmatter hooks terjadi ketika agen dihasilkan sebagai subagent melalui alat Agent atau @-mention, dan ketika agen berjalan sebagai sesi utama melalui [`--agent`](#invoke-subagents-explicitly) atau pengaturan `agent`. Dalam kasus sesi-utama mereka berjalan bersama hook apa pun yang ditentukan dalam [`settings.json`](/docs/id/hooks).
</Note>

Untuk membiarkan hooks frontmatter subagent tingkat proyek berjalan, terima [dialog kepercayaan workspace](/docs/id/permissions#project-allow-rules-and-workspace-trust) untuk folder yang berisi file agen. Hooks dari subagent tingkat pengguna dalam `~/.claude/agents/` dan dari definisi yang Anda lewatkan dengan `--agents` berjalan tanpa langkah ini. Jika Anda menambahkan folder dengan `--add-dir` dari luar repositori workspace terpercaya Anda, percayai folder itu secara terpisah: hooks `.claude/agents/` miliknya tidak mewarisi kepercayaan workspace Anda.

Sampai Anda mempercayai folder, subagent masih berjalan, tetapi Claude Code melewati hooks frontmatter-nya dan mencatat kesalahan ke debug log yang menjelaskan cara mempercayai folder. Ini adalah aturan yang lebih ketat daripada aturan untuk hooks dalam file pengaturan: mempercayai folder induk tidak cukup, dan sesi `-p` tidak dihitung sebagai terpercaya. [What runs before you trust a folder](/docs/id/permissions#what-runs-before-you-trust-a-folder) membandingkan keduanya. Sebelum v2.1.218, hooks frontmatter dapat berjalan dari folder yang belum Anda percayai, termasuk dalam sesi non-interaktif.

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

Ketika agen dipanggil sebagai subagent, hooks `Stop` dalam frontmatter secara otomatis dikonversi ke peristiwa `SubagentStop`.

<h4 id="project-level-hooks-for-subagent-events">
  Hooks tingkat proyek untuk peristiwa subagent
</h4>

Konfigurasi hooks dalam `settings.json` yang merespons peristiwa siklus hidup subagent dalam sesi utama.

| Peristiwa       | Input Matcher   | Kapan itu terjadi                |
| :-------------- | :-------------- | :------------------------------- |
| `SubagentStart` | Nama jenis agen | Ketika subagent mulai dijalankan |
| `SubagentStop`  | Nama jenis agen | Ketika subagent selesai          |

Kedua peristiwa mendukung matcher untuk menargetkan jenis agen tertentu berdasarkan nama. Nilai matcher adalah `name` frontmatter agen untuk subagent tingkat proyek dan pengguna, atau pengenal yang dibatasi cakupan plugin seperti `my-plugin:db-agent` untuk [subagent plugin](/docs/id/plugins/components#agents). Nama yang dibatasi cakupan berisi titik dua, sehingga dievaluasi sebagai [ekspresi reguler yang tidak berlabuh](/docs/id/hooks#matcher-patterns); jangkarnya dengan `^` dan `$`, seperti dalam `^my-plugin:db-agent$`, untuk mencocokkan hanya agen itu.

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

Jaga deskripsi tetap singkat: Claude Code menampilkan peringatan startup ketika deskripsi gabungan subagent Anda melampaui [batas 15.000 token](/docs/id/errors#agent-descriptions-are-over-the-15000-token-limit), dan masih memuat setiap subagent.

Jika subagent dikirim dalam [plugin](/docs/id/plugins/overview), Anda dapat mengukur seberapa andal Claude mendelegasikan ke dalamnya di seluruh prompt realistis daripada memeriksa satu per satu: [`claude plugin eval`](/docs/id/plugin-evals) menjalankan setiap prompt dengan dan tanpa plugin dan menilai hasilnya.

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

Subagent yang disediakan oleh [plugin](/docs/id/plugins/overview) yang diaktifkan muncul di typeahead dengan nama yang dibatasi, seperti `my-plugin:code-reviewer` atau `my-plugin:review:security` ketika plugin [mengorganisir agen ke dalam subfolder](#choose-the-subagent-scope). Subagent background bernama yang saat ini berjalan dalam sesi juga muncul di typeahead, menunjukkan status mereka di samping nama.

Anda juga dapat mengetik mention secara manual tanpa menggunakan picker: `@agent-<name>` untuk subagent lokal, atau `@agent-` diikuti dengan nama yang dibatasi untuk subagent plugin, misalnya `@agent-my-plugin:code-reviewer`. Saat Anda mengetik formulir ini, typeahead menampilkan kecocokan file daripada agen. Penyebutan agen masih diselesaikan saat Anda mengirimkan.

**Jalankan seluruh sesi sebagai subagent.** Lewatkan [`--agent <name>`](/docs/id/cli-reference) untuk memulai sesi di mana thread utama itu sendiri mengambil prompt sistem subagent, pembatasan alat, dan model:

```bash theme={null}
claude --agent code-reviewer
```

Prompt sistem subagent menggantikan prompt sistem Claude Code default sepenuhnya, dengan cara yang sama [`--system-prompt`](/docs/id/cli-reference) melakukannya. File `CLAUDE.md` dan memori proyek masih dimuat melalui aliran pesan normal, bahkan ketika definisi agen menetapkan [`omitClaudeMd`](#supported-frontmatter-fields).

Nama agen muncul sebagai `@<name>` di header startup sehingga Anda dapat mengonfirmasi itu aktif.

Ini berfungsi dengan subagent bawaan dan khusus, dan pilihan bertahan ketika Anda melanjutkan sesi: Claude Code memulihkan pembatasan alat dan model agen bersama dengan percakapan. Jika agen tidak lagi ada saat Anda melanjutkan, sesi berlanjut dengan alat default dan menampilkan [peringatan yang menyebutkan agen](/docs/id/errors#session-agent-no-longer-available). Untuk prompt sistem dalam kedua kasus, lihat [Bendera prompt sistem dalam percakapan yang dilanjutkan](/docs/id/cli-reference#system-prompt-flags-in-resumed-conversations).

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
* **Subagent background** berjalan secara bersamaan sementara Anda terus bekerja. Ketika subagent background mencapai panggilan alat yang memerlukan izin, Claude Code menampilkan prompt di sesi utama Anda dan menyebutkan subagent yang bertanya. Setujui untuk membiarkan subagent melanjutkan, atau tekan Esc untuk menolak panggilan alat itu saja tanpa menghentikan subagent.

Untuk setiap subagent yang Claude hasilkan dengan alat Agent, Claude Code memilih foreground atau background dari kasus pertama yang berlaku:

* Jika anggota [tim agen](/docs/id/agent-teams#limitations) dalam proses yang menghasilkan subagent, Claude Code menjalankannya di foreground. Claude Code menolak dengan kesalahan untuk menghasilkan subagent anggota tim yang definisinya menetapkan [`background: true`](#supported-frontmatter-fields). Di mana [fork mode](#turn-fork-mode-on-or-off) mati dan Anda belum [mematikan background tasks](/docs/id/env-vars), Claude Code juga menolak dengan kesalahan ketika anggota tim menetapkan `run_in_background: true`.
* Jika Anda menetapkan [`CLAUDE_CODE_DISABLE_BACKGROUND_TASKS`](/docs/id/env-vars) ke `1`, Claude Code menjalankan subagent di foreground, dalam setiap jenis sesi dan apakah fork mode aktif atau tidak.
* Di mana [fork mode](#turn-fork-mode-on-or-off) aktif, seperti yang terjadi secara default dalam sesi interaktif, Claude Code menjalankan subagent di background, subagent fork dan non-fork sama-sama, dan Claude tidak dapat meminta foreground.
* Di mana fork mode mati, Claude menjalankan subagent di background secara default dan di foreground ketika memerlukan hasil sebelum melanjutkan. Fork mode mati dalam [mode non-interaktif](/docs/id/headless) dengan `-p` dan dalam Agent SDK kecuali Anda mengaktifkannya. Untuk menjaga subagent tertentu di background bahkan ketika Claude menginginkan hasil, atur bidang frontmatter [`background`](#supported-frontmatter-fields) ke `true`.

Untuk skill dengan `context: fork`, Claude Code mengikuti aturan dalam [Jalankan skills dalam subagent](/docs/id/skills#run-skills-in-a-subagent) sebagai gantinya, apakah fork mode aktif atau tidak.

Subagent background berjalan dengan [set alat bawaan yang lebih kecil](#available-tools) daripada subagent foreground, kecuali untuk fork percakapan dan subagent foreground yang [dilanjutkan](#resume-subagents).

Subagent background menampilkan setiap prompt izin di sesi utama Anda. Ketika Anda menjawab salah satu prompt tersebut dengan pilihan yang berlangsung melampaui panggilan alat itu, seperti hibah yang berlangsung untuk sisa sesi, Claude Code menerapkan jawaban Anda ke seluruh sesi, termasuk percakapan utama Anda.

Subagent background dapat meninggalkan perintah [Bash atau PowerShell](/docs/id/tools-reference#background-commands) background [berjalan melampaui akhir giliran](/docs/id/interactive-mode#how-backgrounding-works). Ketika perintah itu berakhir, Claude Code mengirim notifikasi ke subagent.

Hasil subagent background mencapai Claude sebagai notifikasi penyelesaian dalam giliran yang lebih baru. Claude menunggu notifikasi itu sebelum melaporkan hasil subagent, dan jika Anda bertanya tentang kemajuan terlebih dahulu, itu melaporkan bahwa subagent masih berjalan. Sebelum v2.1.211, Claude kadang melaporkan hasil untuk subagent background yang belum selesai.

Anda juga dapat mengarahkan ini sendiri:

* Di mana fork mode mati, minta Claude untuk menjalankan tugas di background atau di foreground
* Tekan **Ctrl+B** untuk menempatkan tugas yang sedang berjalan di background

Claude Code menghapus baris subagent background dari panel subagent di bawah input prompt dengan salah satu dari dua cara, tergantung bagaimana subagent berakhir:

* Ketika subagent selesai dengan sukses, Claude Code menghapus barisnya segera dan, kecuali dalam [mode pembaca layar](/docs/id/accessibility), menampilkan `/tasks to see subagents` di footer selama 30 detik. Selama 30 detik itu, jalankan [`/tasks`](/docs/id/commands) dan tekan `Enter` pada subagent untuk membuka transkrip. Sebelum v2.1.232, Claude Code menyimpan baris selama 30 detik setelah subagent selesai, sama seperti yang gagal, dan tidak menampilkan petunjuk footer.
* Ketika subagent gagal atau Anda menghentikannya, Claude Code menyimpan barisnya selama 30 detik. Untuk menghapus baris lebih cepat, pilih dan tekan `x`.

Subagent background yang selesai tetap terdaftar dalam [`/tasks`](/docs/id/commands), ditandai selesai dan diurutkan di bawah pekerjaan yang sedang berjalan, selama 30 detik yang sama dengan petunjuk footer. Tampilan detailnya tetap terbuka ketika subagent selesai. Subagent yang gagal atau yang Anda hentikan meninggalkan daftar. Sebelum v2.1.208, subagent yang selesai meninggalkan daftar saat itu selesai dan tampilan detailnya ditutup.

<h3 id="subagent-names">
  Nama subagent
</h3>

Claude dapat memberi subagent nama dengan melewatkan parameter `name` pada panggilan alat Agent, dan dapat melakukannya sendiri, tanpa bertanya kepada Anda terlebih dahulu. Nama membuat subagent dapat dialamatkan: Claude dapat [mengirim pesan atau melanjutkannya berdasarkan nama](#resume-subagents) setelah selesai.

Dalam sesi interaktif dengan [tim agen](/docs/id/agent-teams) diaktifkan, subagent yang Claude hasilkan dari percakapan utama dengan `name` diluncurkan sebagai anggota tim sebagai gantinya, kecuali panggilan adalah [fork](#fork-the-current-conversation) atau melewatkan `isolation` pada panggilan itu sendiri. Nilai `isolation` dalam frontmatter subagent tidak mencegahnya, dan anggota tim kemudian berjalan di direktori kerja sesi utama. Lihat [Bagaimana Claude memulai tim agen](/docs/id/agent-teams#how-claude-starts-agent-teams).

<h3 id="api-errors-in-subagents">
  Kesalahan API dalam subagent
</h3>

Ketika sesuatu [memotong respons subagent di tengah-aliran](/docs/id/errors#the-response-above-may-be-incomplete), dan respons parsial berisi teks tetapi tidak ada panggilan alat, Claude Code meminta subagent untuk melanjutkan daripada mengakhiri run. Ini terjadi dalam sesi interaktif juga. Run berakhir pada kesalahan hanya setelah kelanjutan itu habis.

Mulai dari v2.1.199, subagent yang run-nya berakhir pada kesalahan API, seperti batas penggunaan atau kesalahan server berulang, melaporkan kegagalan itu kembali ke Claude daripada mengembalikan teks kesalahan seolah-olah itu adalah temuan subagent. Apa yang Claude terima tergantung di mana subagent berjalan:

* **Foreground**: jika batas laju, kelebihan beban, atau kesalahan server memotong subagent yang sudah menghasilkan output teks, alat Agent mengembalikan output parsial itu dengan catatan bahwa subagent dipotong dan tidak menyelesaikan tugasnya. Subagent yang tidak menghasilkan apa pun, atau yang output-nya hanya panggilan alat, gagal dengan [`Agent terminated early due to an API error`](/docs/id/errors#agent-terminated-early-due-to-an-api-error), diikuti oleh detail kesalahan. Dalam v2.1.199, batas laju, kelebihan beban, atau kesalahan server yang memotong bentuk tool-calls-only mengembalikan hasil parsial kosong yang hanya berisi catatan cut-off sebagai gantinya.
* **Background**: subagent ditandai gagal, dan pesan yang Claude terima saat berakhir menyebutkan kesalahan API dan menyertakan output terakhir subagent, jadi pekerjaan parsial tidak hilang.

Ketika Anda mengonfigurasi [rantai model fallback](/docs/id/model-config#fallback-model-chains) dan subagent mengalami kegagalan yang dicakup rantai, seperti model-nya tidak tersedia, Claude Code beralih subagent ke model pertama dalam rantai yang menerima permintaan. Subagent terus bekerja daripada berakhir pada kesalahan.

Setelah kesalahan API yang mendasar hilang, minta Claude untuk mencoba ulang tugas atau [lanjutkan subagent](#resume-subagents).

<h3 id="subagent-output-scanning">
  Pemindaian output subagent
</h3>

Claude Code memindai laporan akhir setiap subagent sebelum Claude membacanya. Subagent mungkin telah membaca file, halaman web, atau output perintah yang tidak pernah Anda tinjau, dan teks dari sumber tersebut dapat membawa instruksi yang ditujukan ke percakapan utama. Pemindaian tidak pernah menghapus atau menulis ulang apa pun; itu membuat dua jenis perubahan yang mungkin Anda perhatikan dalam laporan:

* **Penyisipan backslash**: pemindaian menyisipkan backslash ke dalam teks yang meniru output Claude Code itu sendiri, seperti tag `<system-reminder>` atau baris yang dimulai dengan `Human:` atau `Assistant:`, sehingga peniruan dibaca sebagai teks biasa daripada disalahartikan sebagai bagian dari percakapan.
* **Baris penanda**: pemindaian menambahkan baris yang dimulai dengan `[harness: subagent output matched instruction-shaped pattern(s):` ketika laporan meniru tag seperti `<system-reminder>` atau menyebutkan pengaturan izin seperti `bypassPermissions` atau `--dangerously-skip-permissions`. Penyebutan pengaturan izin mendapatkan baris penanda, tetapi teks itu sendiri tetap seperti yang ditulis.

Pemindaian tidak menilai apakah konten berbahaya, dan itu tidak mengubah apa yang dapat dilakukan instruksi dalam laporan: panggilan alat yang dilaporkan mengarahkan Claude untuk membuat masih melalui [pemeriksaan izin](/docs/id/permissions) sesi dan [sandboxing](/docs/id/sandboxing). Ini bukan pengganti untuk [membatasi apa yang dapat dijangkau subagent](#control-subagent-capabilities).

Laporan yang kembali ke Claude sebagai hasil subagent juga tiba di bawah header yang menandainya sebagai output subagent. Header menyatakan bahwa instruksi atau klaim persetujuan di dalam laporan adalah kata-kata subagent dan tidak membawa otoritas dari Anda.

Laporan [subagent background](#run-subagents-in-foreground-or-background) tiba di dalam notifikasi penyelesaian, yang ditandai sebagai peristiwa otomatis daripada pesan dari Anda.

<Note>
  Pemindaian output subagent memerlukan Claude Code v2.1.210 atau lebih baru.
</Note>

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

Untuk pekerjaan yang perlu terus berjalan secara paralel atau tidak akan muat dalam satu jendela konteks, jalankan dalam [sesi terpisah](/docs/id/agents) dan biarkan Claude [meneruskan temuan di antara mereka](/docs/id/cross-session-messaging).

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
* Latensi penting. Subagent yang bukan [fork](#fork-the-current-conversation) dimulai segar dan mungkin memerlukan waktu untuk mengumpulkan konteks

Gunakan **subagent** ketika:

* Tugas menghasilkan output verbose yang Anda tidak butuhkan dalam konteks utama Anda
* Anda ingin menerapkan pembatasan alat atau izin tertentu
* Pekerjaan mandiri dan dapat mengembalikan ringkasan

Pertimbangkan [Skills](/docs/id/skills) sebagai gantinya ketika Anda menginginkan prompt atau alur kerja yang dapat digunakan kembali yang berjalan dalam konteks percakapan utama daripada konteks subagent yang terisolasi.

Untuk pertanyaan tentang sesuatu yang sudah ada dalam percakapan Anda, gunakan [`/btw`](/docs/id/interactive-mode#side-questions-with-%2Fbtw) sebagai gantinya dari subagent. Ini melihat konteks penuh Anda tetapi tidak memiliki akses alat, dan jawabannya tidak ditambahkan ke riwayat.

<h3 id="let-subagents-spawn-their-own-subagents">
  Biarkan subagent menghasilkan subagent mereka sendiri
</h3>

Secara default, subagent dapat menghasilkan subagent-nya sendiri, hingga tiga lapisan di bawah percakapan utama. Pada batas kedalaman, Claude Code menahan alat `Agent` dari setiap subagent kecuali [fork](#fork-the-current-conversation), jadi subagent pada batas melakukan pekerjaan yang didelegasikan itu sendiri dan mengembalikan satu ringkasan. Fork pada batas menyimpan `Agent` dalam daftar alat yang diwariskan, tetapi alat mengembalikan kesalahan daripada menghasilkan.

Subagent bersarang cocok untuk tugas yang didelegasikan yang itu sendiri terbagi menjadi subtask paralel, seperti subagent reviewer yang mengirimkan verifier per temuan. Dalam sesi interaktif, hanya ringkasan subagent tingkat atas yang kembali kepada Anda dan output perantara tetap keluar dari percakapan utama Anda: subagent yang meluncurkan subagent background menunggu hasil mereka sebelum selesai. Dalam [mode non-interaktif](/docs/id/headless) dan Agent SDK, subagent peluncur tidak menunggu, jadi subagent background bersarang yang selesai setelah peluncurnya telah berakhir melaporkan ke percakapan utama Anda sebagai gantinya.

Untuk mengubah batas, atur [`CLAUDE_CODE_MAX_SUBAGENT_SPAWN_DEPTH`](/docs/id/env-vars) ke jumlah lapisan subagent yang Anda inginkan di bawah percakapan utama Anda. Misalnya, entri ini dalam [`settings.json`](/docs/id/settings) membatasi nesting pada dua lapisan:

```json theme={null}
{
  "env": {
    "CLAUDE_CODE_MAX_SUBAGENT_SPAWN_DEPTH": "2"
  }
}
```

Dengan nilai ini, subagent Anda dapat mendelegasikan ke lapisan kedua mereka sendiri, dan lapisan kedua itu tidak dapat mendelegasikan lebih lanjut. Atur `1` untuk mematikan nesting.

Subagent bersarang dikonfigurasi dengan cara yang sama seperti subagent tingkat atas dan diselesaikan dari [scope](#choose-the-subagent-scope) yang sama. Untuk menjaga satu subagent agar tidak menghasilkan sementara nesting aktif, seperti reviewer yang harus tetap read-only, hilangkan `Agent` dari daftar [`tools`](#available-tools) atau tambahkan ke `disallowedTools`.

Dalam terminal, Claude Code menampilkan subagent bersarang sebagai pohon dalam panel subagent di bawah input prompt dan menandai setiap baris yang masih memiliki keturunan dalam panel dengan hitungan `(+N)` mereka. Buka baris untuk melihat saudara dan anak langsung subagent itu dengan jalur kembali ke `main`.

<Note>
  Versi sebelumnya menggunakan default yang berbeda:

  * **v2.1.172 hingga v2.1.216**: subagent dapat bersarang secara default, hingga lima lapisan dalam, dan batas tidak dapat diubah.
  * **v2.1.217 hingga v2.1.218**: batas default ke satu, jadi subagent tidak dapat menghasilkan miliknya sendiri kecuali Anda menaikkannya; v2.1.219 menaikkan default ke tiga.
</Note>

<h3 id="concurrent-subagent-limit">
  Batas subagent bersamaan
</h3>

Dua batas mengontrol penggunaan subagent, masing-masing dengan variabelnya sendiri: yang ini menghentikan Claude dari menghasilkan lebih banyak subagent sementara terlalu banyak berjalan, dan [batas kedalaman](#let-subagents-spawn-their-own-subagents) membatasi seberapa dalam subagent bersarang. Tidak ada batas pada jumlah total subagent yang dapat Claude hasilkan selama sesi.

Secara default, ketika 20 subagent berjalan dalam sesi, menghasilkan yang lain dengan alat Agent gagal dengan `Concurrent subagent limit reached`, dan kesalahan memberi tahu Claude untuk tidak mencoba ulang. Menghasilkan berhasil lagi ketika hitungan yang berjalan turun di bawah batas. Untuk mengubah batas, atur [`CLAUDE_CODE_MAX_CONCURRENT_SUBAGENTS`](/docs/id/env-vars) ke bilangan bulat positif apa pun. Sesi dengan [ultracode](/docs/id/model-config#adjust-effort-level) aktif dikecualikan: batas tidak diterapkan di sana. Memerlukan Claude Code v2.1.217 atau lebih baru.

Batas hanya memblokir subagent yang Claude hasilkan dengan alat Agent, tetapi run lain menempati slot yang sama:

* Fork dalam sesi yang Anda mulai dengan [`/subtask`](#fork-the-current-conversation) menempati slot saat berjalan dan tidak pernah diblokir oleh batas.
* [Melanjutkan subagent](#resume-subagents) yang sudah selesai menempati slot segar tanpa memeriksa batas, jadi resume dapat mendorong hitungan yang berjalan melampaui batas.

Agen yang fitur lain jalankan, seperti agen [workflow](/docs/id/workflows) dan anggota [tim agen](/docs/id/agent-teams), mengikuti batas mereka sendiri sebagai gantinya.

<h3 id="manage-subagent-context">
  Kelola konteks subagent
</h3>

<h4 id="what-loads-at-startup">
  Apa yang dimuat saat startup
</h4>

Setiap subagent dimulai dengan jendela konteks yang segar dan terisolasi. Ini tidak melihat riwayat percakapan Anda, skills yang sudah Anda panggil, atau file yang sudah Claude baca. Claude menyusun pesan delegasi yang merangkum tugas, dan subagent bekerja dari sana. Pengecualiannya adalah [fork](#fork-the-current-conversation), yang mewarisi percakapan induk daripada memulai segar.

Konteks awal subagent non-fork berisi:

* **Prompt sistem**: prompt agen itu sendiri ditambah detail lingkungan yang Claude Code tambahkan, bukan prompt sistem Claude Code. Subagent khusus mendefinisikan milik mereka dalam [badan markdown](#write-subagent-files) atau bidang `prompt`. Agen bawaan memiliki prompt yang telah ditentukan sebelumnya.
* **Pesan tugas**: prompt delegasi yang Claude tulis saat menyerahkan pekerjaan.
* **File CLAUDE.md**: setiap level dari [hierarki CLAUDE.md](/docs/id/memory#how-claude-md-files-load) yang dimuat percakapan utama, termasuk `~/.claude/CLAUDE.md`, aturan proyek, `CLAUDE.local.md`, file kebijakan yang dikelola, dan file [`AGENTS.md`](/docs/id/memory#agents-md) apa pun yang dimuat sebagai instruksi proyek. Agen Explore dan Plan bawaan melewati ini. Subagent yang definisinya menetapkan [`omitClaudeMd`](#supported-frontmatter-fields) hanya memuat file kebijakan yang dikelola, atau tidak ada sama sekali ketika definisi berasal dari [pengaturan yang dikelola](#choose-the-subagent-scope).
* **Status Git**: snapshot yang diambil di awal sesi induk. Tidak ada ketika direktori kerja bukan repositori Git atau ketika [`includeGitInstructions`](/docs/id/settings-reference#includegitinstructions) adalah `false`. Explore dan Plan melewatinya terlepas.
* **Skills yang dimuat sebelumnya**: konten lengkap dari skill apa pun yang dinamai dalam bidang [`skills`](#preload-skills-into-subagents) agen. Agen bawaan tidak memuat skills sebelumnya.
* **Daftar saudara**: pengingat sistem yang mencantumkan `main` dan setiap agen bernama lainnya dalam sesi, masing-masing nilai `to` yang valid untuk [`SendMessage`](#resume-subagents). Memerlukan Claude Code v2.1.206 atau lebih baru. Daftar muncul hanya ketika alat subagent mencakup `SendMessage` dan setidaknya satu agen lain memiliki nama, baik Claude menamakannya saat memunculkannya atau berjalan sebagai anggota [tim agen](/docs/id/agent-teams). Ini adalah snapshot yang diambil ketika subagent dimulai, jadi agen yang dinamai nanti tidak muncul.

Untuk meluncurkan salah satu subagent Anda sendiri tanpa file CLAUDE.md pengguna, proyek, dan lokal, atur [`omitClaudeMd: true`](#supported-frontmatter-fields) dalam frontmatter atau `--agents` JSON.

Percakapan utama masih memiliki CLAUDE.md penuh Anda saat membaca hasil subagent ini, jadi sebagian besar aturan tidak perlu mencapai subagent itu sendiri. Jika aturan harus, seperti "abaikan direktori `vendor/`," nyatakan kembali dalam prompt yang Anda berikan Claude saat mendelegasikan.

Anda tidak dapat mengubah subagent mana yang menerima status git. Hanya Explore dan Plan yang melewatinya.

Beberapa status percakapan utama tidak pernah mencapai subagent non-fork:

* **Gaya output**: subagent menjalankan prompt sistemnya sendiri, jadi [gaya output](/docs/id/output-styles) Anda tidak membentuk responsnya, kecuali dalam [fork](#fork-the-current-conversation).
* **Memori otomatis**: [memori otomatis](/docs/id/memory#auto-memory) percakapan utama tidak dimuat. Untuk memberi subagent memori persisten miliknya sendiri, gunakan bidang [`memory`](#enable-persistent-memory).
* **Ukuran jendela konteks**: jendela konteks subagent diukur oleh modelnya sendiri, bukan induk. Mendelegasikan ke model dengan jendela yang lebih kecil memberikan subagent itu jendela yang lebih kecil.

<h4 id="resume-subagents">
  Lanjutkan subagent
</h4>

Setiap invokasi subagent membuat instance baru daripada melanjutkan yang sebelumnya. Untuk melanjutkan pekerjaan subagent yang ada daripada memulai dari awal, minta Claude untuk melanjutkannya.

Subagent yang dilanjutkan mempertahankan riwayat percakapan lengkap mereka, termasuk semua panggilan alat sebelumnya, hasil, dan penalaran. Jika subagent menghasilkan [subagent background miliknya sendiri](#let-subagents-spawn-their-own-subagents), riwayat itu mencakup hasil yang mereka berikan saat berjalan. Subagent melanjutkan tepat di mana ia berhenti daripada memulai segar.

* Ketika subagent selesai, Claude menerima ID agennya.
* Agen bawaan Explore dan Plan adalah one-shot dan tidak mengembalikan ID agen, jadi Claude tidak dapat melanjutkan mereka. Gunakan `general-purpose` atau subagent khusus ketika Anda perlu melanjutkan pekerjaan.
* Ketika subagent berhenti pada batas [`maxTurns`](#supported-frontmatter-fields), Claude Code menandai output yang dikembalikan sebagai parsial. Untuk subagent yang mengembalikan ID agen, Claude Code juga mencatat dalam hasil bahwa Claude dapat mengirim pesan ke subagent untuk melanjutkan dari tempat ia berhenti.

Claude menggunakan alat `SendMessage` dengan ID agen atau nama agen sebagai bidang `to` untuk melanjutkannya. `SendMessage` tidak memerlukan [tim agen](/docs/id/agent-teams) untuk diaktifkan; hanya pesan protokol tim terstruktur seperti `shutdown_request` dan `plan_approval_response` yang melakukannya. Melampaui subagent dan rekan tim, dalam sesi di mana cross-session messaging diaktifkan, Claude dapat menggunakan alat yang sama untuk mengirim pesan [sesi Claude Code Anda yang lain](/docs/id/cross-session-messaging), di mesin ini atau [melampaui](/docs/id/cross-session-messaging#message-sessions-on-other-machines).

Untuk melanjutkan subagent, minta Claude untuk melanjutkan pekerjaan sebelumnya:

```text wrap theme={null}
Use the code-reviewer subagent to review the authentication module
[Agent completes]

Continue that code review and now analyze the authorization logic
[Claude resumes the subagent with full context from previous conversation]
```

Ketika Claude mengirim pesan subagent yang selesai dengan alat `SendMessage`, subagent melanjutkan di background tanpa invokasi `Agent` baru. Hal yang sama berlaku untuk subagent yang Claude hentikan dengan alat `TaskStop`, setelah run yang dihentikan telah keluar. Run yang dilanjutkan menyimpan [set alat dari tempat subagent pertama kali berjalan](#run-subagents-in-foreground-or-background) dan dapat terus membaca [prompt cache yang dihangatkan run asli](/docs/id/prompt-caching#subagents-and-the-cache).

Subagent yang memiliki alat `SendMessage` dapat mengirim pesan itu juga. Dalam sesi interaktif, agen yang dilanjutkan kemudian melaporkan kembali ke subagent yang melanjutkannya, bukan ke percakapan utama Anda. Subagent itu menunggu hasil sebelum menyelesaikan pekerjaan miliknya sendiri. Ketika subagent mengirim pesan ke agen yang dilaporkannya, seperti peluncurnya sendiri, Claude Code melanjutkan agen itu tanpa mengarahkan ulang hasilnya.

Subagent yang Anda hentikan sendiri, dengan `x` dalam `/tasks` atau permintaan SDK `stop_task`, tidak auto-resume. Jika Claude mengirimnya pesan, pesan ditolak dan Claude diberitahu agen dibatalkan.

Sementara [baris subagent itu masih dalam panel subagent](#run-subagents-in-foreground-or-background), ketik ke dalam transkrip untuk melanjutkannya sendiri. Setelah itu, pesan dari Claude dapat auto-resume lagi.

Melanjutkan memulai run baru dari agen di bawah ID yang sama, jadi subagent yang sudah gagal atau selesai menunjukkan sebagai berjalan lagi dalam daftar tugas dan dalam peristiwa tugas SDK Agent. Sebelum v2.1.205, itu terus menunjukkan status gagal atau selesai sebelumnya sementara run yang dilanjutkan sedang bekerja.

Mulai dari v2.1.199, `SendMessage` memeriksa bahwa nama masih merujuk ke agen yang sama yang dicapai sebelumnya dalam percakapan. Jika agen yang lebih baru telah mengambil nama, seperti agen background yang di-spawn ulang yang menggunakannya kembali, Claude Code menolak pengiriman daripada mengirimkannya ke agen yang salah, dan kesalahan melaporkan agen mana yang sekarang dicapai nama sehingga Claude dapat menargetkan ulang. Untuk mencapai agen sebelumnya sementara masih berjalan, Claude mengalamatkannya dengan ID agen yang diterima saat menghasilkan agen itu. Pemeriksaan dibatasi pada percakapan saat ini dan direset pada `/clear`.

Mulai dari v2.1.198, subagent memperlakukan pesan dari agen yang meluncurkannya sebagai arahan tugas normal, termasuk koreksi kursus mid-task, dan bertindak atas mereka dalam pengaturan izin mereka sendiri. Dua batas masih berlaku terlepas dari siapa yang mengirim pesan: tidak ada pesan dari agen apa pun yang dihitung sebagai persetujuan Anda untuk prompt izin yang tertunda, dan tidak ada pesan agen yang dapat mengubah pengaturan izin subagent, `CLAUDE.md`, atau konfigurasi. Hanya sistem izin atau pesan Anda sendiri yang dapat memberikan persetujuan.

Anda juga dapat meminta Claude untuk ID agen jika Anda ingin mereferensikannya secara eksplisit, atau temukan ID dalam file transkrip di `~/.claude/projects/{project}/{sessionId}/subagents/`. Setiap transkrip disimpan sebagai `agent-{agentId}.jsonl`.

Transkrip subagent bertahan secara independen dari percakapan utama:

* **Pemadatan percakapan utama**: ketika percakapan utama dipadatkan, transkrip subagent tidak terpengaruh. Mereka disimpan dalam file terpisah.
* **Persistensi sesi**: transkrip subagent bertahan dalam sesi mereka. Anda dapat [melanjutkan subagent](#resume-subagents) setelah memulai ulang Claude Code dengan melanjutkan sesi yang sama.
* **Pembersihan otomatis**: Claude Code menghapus transkrip subagent setelah periode retensi `cleanupPeriodDays`, 30 hari secara default, mengikuti [aturan sweep retensi](/docs/id/claude-directory#cleaned-up-automatically).

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
  Jalankan subagent yang di-fork dengan `/subtask`, yang memerlukan Claude Code v2.1.212 atau lebih baru. Ketika [tampilan agent dimatikan](/docs/id/agent-view#turn-off-agent-view), `/subtask` tidak tersedia dan `/fork` memulai subagent yang di-fork sebagai gantinya; jika tidak `/fork` menyalin seluruh sesi ke [sesi latar belakang](/docs/id/agent-view#from-inside-a-session) baru.
</Note>

Fork adalah subagent yang mewarisi seluruh percakapan sejauh ini daripada memulai segar. Ini menghilangkan isolasi input yang sebaliknya disediakan subagent: fork melihat prompt sistem yang sama, alat, model, dan riwayat pesan sebagai sesi utama, sehingga Anda dapat menyerahkan tugas sampingan tanpa menjelaskan situasinya lagi. Panggilan alat fork sendiri masih tetap keluar dari percakapan Anda dan hanya hasil akhirnya yang kembali, sehingga jendela konteks utama Anda tetap bersih. Gunakan fork ketika subagent lain mana pun memerlukan terlalu banyak latar belakang untuk berguna, atau ketika Anda ingin mencoba beberapa pendekatan secara paralel dari titik awal yang sama.

Claude memulai fork dengan meminta tipe subagent `fork` melalui alat Agent. Anda mengontrol apakah itu dapat dengan [mode fork](#turn-fork-mode-on-or-off), yang diaktifkan secara default dalam sesi interaktif.

Anda dapat memulai fork sendiri dengan `/subtask` diikuti oleh tugas, terlepas dari apakah mode fork diaktifkan atau tidak. Pada v2.1.161 hingga v2.1.211 perintahnya adalah `/fork`. Claude Code memberi nama fork dari kata-kata pertama tugas. Contoh berikut mem-fork percakapan untuk draft kasus uji sementara Anda melanjutkan dengan implementasi dalam sesi utama:

```text wrap theme={null}
/subtask draft unit tests for the parser changes so far
```

Fork muncul di panel di bawah prompt Anda dan berjalan di latar belakang sementara Anda terus bekerja. Ketika selesai, hasilnya tiba sebagai pesan dalam percakapan utama Anda. Bagian berikutnya mencakup kontrol panel untuk menonton dan mengarahkan fork saat berjalan.

<h3 id="observe-and-steer-running-forks">
  Amati dan arahkan fork yang sedang berjalan
</h3>

Fork yang sedang berjalan muncul di panel di bawah input prompt, dengan satu baris untuk sesi utama dan satu untuk setiap fork.

Ketika fork selesai dengan sukses, Claude Code menghapus barisnya. Claude Code menyimpan baris fork yang gagal atau yang Anda hentikan selama 30 detik, [sama seperti untuk subagent latar belakang lainnya](#run-subagents-in-foreground-or-background). Sebelum v2.1.232, Claude Code juga menyimpan baris fork yang selesai selama 30 detik.

Gunakan kunci ini untuk berinteraksi dengan panel:

| Kunci     | Tindakan                                                                                                                                                                                                                            |
| :-------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `↑` / `↓` | Pindah antar baris                                                                                                                                                                                                                  |
| `Enter`   | Buka transkrip fork yang dipilih dan kirimkan pesan tindak lanjut                                                                                                                                                                   |
| `x`       | Hentikan fork yang dipilih jika sedang berjalan, atau tutup barisnya jika tidak lagi berjalan. Pada baris sesi utama, atau pada baris fork yang transkrinya Anda buka dengan `Enter`, `x` mengetik ke dalam prompt sebagai gantinya |
| `Esc`     | Kembalikan fokus ke input prompt                                                                                                                                                                                                    |

Dengan transkrip fork atau subagent terbuka, pesan tindak lanjut dan [skills](/docs/id/skills) pergi ke agen tersebut, tetapi perintah bawaan masih berjalan dalam percakapan utama Anda. Mulai dari v2.1.199, mengetik `/model` atau `/fast` dalam tampilan itu menampilkan pemberitahuan bahwa itu mengubah model percakapan utama atau mode cepat, bukan agen yang dilihat, daripada menjalankannya secara diam-diam.

<h3 id="how-forks-differ-from-other-subagents">
  Bagaimana fork berbeda dari subagent lainnya
</h3>

Fork mewarisi segalanya yang dimiliki sesi utama pada saat spawn. Subagent lainnya dimulai segar dari definisinya.

|                        | Fork                           | Subagent non-fork                                                                                              |
| :--------------------- | :----------------------------- | :------------------------------------------------------------------------------------------------------------- |
| Konteks                | Riwayat percakapan lengkap     | Konteks segar dengan prompt yang Anda lewatkan                                                                 |
| Prompt sistem dan alat | Sama dengan sesi utama         | Dari [file definisi](#write-subagent-files) subagent, [disaring untuk run latar belakang](#available-tools)    |
| Model                  | Sama dengan sesi utama         | Dari bidang `model` subagent                                                                                   |
| Izin                   | Prompt muncul di terminal Anda | [Prompt muncul di sesi utama Anda](#run-subagents-in-foreground-or-background) saat berjalan di latar belakang |
| Prompt cache           | Dibagikan dengan sesi utama    | Cache terpisah                                                                                                 |

Karena prompt sistem fork dan definisi alat identik dengan induk, permintaan pertamanya menggunakan kembali [prompt cache](/docs/id/prompt-caching#subagents-and-the-cache) induk. Ini membuat forking lebih murah daripada menelurkan subagent segar untuk tugas yang memerlukan konteks yang sama.

Ketika Claude menelurkan fork melalui alat Agent, Claude dapat melewatkan `isolation: "worktree"` sehingga edit file fork ditulis ke git worktree terpisah daripada checkout Anda. Fork tidak dapat menelurkan fork lebih lanjut.

<h3 id="turn-fork-mode-on-or-off">
  Aktifkan atau nonaktifkan mode fork
</h3>

Claude Code mengaktifkan mode fork secara default dalam sesi interaktif dan membiarkannya dimatikan secara default dalam [mode non-interaktif](/docs/id/headless) dengan `-p` dan dalam Agent SDK. Default interaktif memerlukan Claude Code v2.1.232 atau lebih baru. Pada versi sebelumnya, atur `CLAUDE_CODE_FORK_SUBAGENT` ke `1` untuk mengaktifkan mode fork.

Anda dapat mengetahui mode fork diaktifkan dari cara Claude Code menangani alat Agent:

* Claude dapat menelurkan fork dengan meminta tipe subagent `fork`. Ketika Claude tidak meminta tipe, Claude mendapatkan subagent [general-purpose](#built-in-subagents), jika sesi masih memiliki tipe tersebut. Subagent yang di-spawn dari definisi, seperti Explore, bekerja seperti biasanya.
* Claude Code menjalankan subagent yang Claude spawn di latar belakang, fork dan subagent non-fork sama-sama, terlepas dari [kasus yang tetap di latar depan](#run-subagents-in-foreground-or-background). Claude Code juga menghapus parameter `run_in_background` alat Agent, sehingga Claude tidak dapat meminta latar depan.

Atur variabel lingkungan [`CLAUDE_CODE_FORK_SUBAGENT`](/docs/id/env-vars) untuk mengganti default:

* `1` mengaktifkan mode fork dalam mode non-interaktif dan Agent SDK juga
* `0` menonaktifkan mode fork dalam setiap jenis sesi

Untuk menjaga mode fork tetap aktif tetapi menghentikan Claude dari menelurkan fork, [tolak tipe subagent `fork`](#disable-specific-subagents) dengan aturan `Agent(fork)`. Claude Code masih menjalankan subagent yang Claude spawn di latar belakang, terlepas dari [kasus yang sama yang tetap di latar depan](#run-subagents-in-foreground-or-background).

<h2 id="example-subagents">
  Contoh subagent
</h2>

Contoh-contoh ini mendemonstrasikan pola efektif untuk membangun subagent. Gunakan mereka sebagai titik awal, atau hasilkan versi yang disesuaikan dengan Claude.

<Tip>
  **Best practices:**

  * **Desain subagent yang terfokus:** setiap subagent harus unggul dalam satu tugas spesifik
  * **Tulis deskripsi yang menonjolkan satu subagent:** Claude menggunakan deskripsi untuk memutuskan kapan mendelegasikan. Buat setiap deskripsi cukup spesifik untuk merutekan ke subagent yang tepat, dan pertahankan set gabungan dalam [anggaran deskripsi 15.000-token](#understand-automatic-delegation)
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

Prompt sistem memberitahu subagent untuk menolak permintaan penulisan, jadi hook adalah backstop: jika subagent mencoba penulisan bagaimanapun, Claude Code memblokir perintah dan subagent melihat pesan `Blocked: Write operations not allowed. Use SELECT queries only.`.

<h2 id="next-steps">
  Langkah berikutnya
</h2>

Sekarang setelah Anda memahami subagent, jelajahi fitur terkait ini:

* [Distribusikan subagent dengan plugins](/docs/id/plugins/components#agents) untuk berbagi subagent di seluruh tim atau proyek
* [Jalankan Claude Code secara terprogram](/docs/id/headless) dengan Agent SDK untuk CI/CD dan otomasi
* [Gunakan MCP servers](/docs/id/mcp) untuk memberikan subagent akses ke alat dan data eksternal
