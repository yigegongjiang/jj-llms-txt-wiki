> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Praktik Terbaik untuk Claude Code

> Tips dan pola untuk memaksimalkan Claude Code, dari mengonfigurasi lingkungan Anda hingga menskalakan di seluruh sesi paralel.

Claude Code adalah lingkungan pengkodean yang bersifat agentic. Tidak seperti chatbot yang menjawab pertanyaan dan menunggu, Claude Code dapat membaca file Anda, menjalankan perintah, membuat perubahan, dan bekerja secara mandiri melalui masalah sambil Anda menonton, mengarahkan, atau sepenuhnya menjauh.

Ini mengubah cara Anda bekerja. Alih-alih menulis kode sendiri dan meminta Claude untuk meninjau, Anda menjelaskan apa yang Anda inginkan dan Claude mengetahui cara membangunnya. Claude mengeksplorasi, merencanakan, dan mengimplementasikan.

Namun otonomi ini masih datang dengan kurva pembelajaran. Claude bekerja dalam batasan tertentu yang perlu Anda pahami.

Panduan ini mencakup pola yang telah terbukti efektif di seluruh tim internal Anthropic dan untuk insinyur yang menggunakan Claude Code di berbagai basis kode, bahasa, dan lingkungan. Untuk cara loop agentic bekerja di balik layar, lihat [Cara Claude Code Bekerja](/docs/id/how-claude-code-works).

***

Sebagian besar praktik terbaik didasarkan pada satu batasan: jendela konteks Claude terisi dengan cepat, dan kinerja menurun saat terisi.

Jendela konteks Claude menyimpan seluruh percakapan Anda, termasuk setiap pesan, setiap file yang dibaca Claude, dan setiap output perintah. Namun, ini dapat terisi dengan cepat. Sesi debugging tunggal atau eksplorasi basis kode mungkin menghasilkan dan mengonsumsi puluhan ribu token.

Ini penting karena kinerja LLM menurun saat konteks terisi. Ketika jendela konteks hampir penuh, Claude mungkin mulai "lupa" instruksi sebelumnya atau membuat lebih banyak kesalahan. Jendela konteks adalah sumber daya paling penting untuk dikelola. Untuk melihat bagaimana sesi terisi dalam praktik, [tonton panduan interaktif](/docs/id/context-window) tentang apa yang dimuat saat startup dan berapa biaya setiap pembacaan file. Lacak penggunaan konteks secara berkelanjutan dengan [baris status khusus](/docs/id/statusline), dan lihat [Kurangi penggunaan token](/docs/id/costs#reduce-token-usage) untuk strategi mengurangi penggunaan token.

***

<h2 id="give-claude-a-way-to-verify-its-work">
  Berikan Claude cara untuk memverifikasi pekerjaannya
</h2>

<Tip>
  Berikan Claude sesuatu yang dapat dijalankannya: tes, build, tangkapan layar untuk dibandingkan. Ini adalah perbedaan antara sesi yang Anda saksikan dan sesi yang dapat Anda tinggalkan.
</Tip>

Claude berhenti ketika pekerjaan terlihat selesai. Tanpa pemeriksaan yang dapat dijalankannya, "terlihat selesai" adalah satu-satunya sinyal yang tersedia, dan Anda menjadi loop verifikasi: setiap kesalahan menunggu Anda untuk menyadarinya. Berikan Claude sesuatu yang menghasilkan lulus atau gagal, dan loop akan menutup dengan sendirinya. Claude melakukan pekerjaan, menjalankan pemeriksaan, membaca hasilnya, dan melakukan iterasi hingga pemeriksaan lulus.

Pemeriksaan adalah apa pun yang mengembalikan sinyal yang dapat dibaca Claude dalam percakapan: rangkaian tes, kode keluar build, linter, skrip yang membedakan output terhadap fixture, atau [tangkapan layar browser](/docs/id/chrome) dibandingkan dengan desain.

| Strategi                                  | Sebelum                                                  | Sesudah                                                                                                                                                                                                               |
| ----------------------------------------- | -------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Sediakan kriteria verifikasi**          | *"implementasikan fungsi yang memvalidasi alamat email"* | *"tulis fungsi validateEmail. contoh kasus uji: [user@example.com](mailto:user@example.com) adalah true, invalid adalah false, [user@.com](mailto:user@.com) adalah false. jalankan tes setelah mengimplementasikan"* |
| **Verifikasi perubahan UI secara visual** | *"buat dashboard terlihat lebih baik"*                   | *"\[tempel tangkapan layar] implementasikan desain ini. ambil tangkapan layar hasilnya dan bandingkan dengan yang asli. daftar perbedaan dan perbaiki"*                                                               |
| **Tangani penyebab akar, bukan gejala**   | *"build gagal"*                                          | *"build gagal dengan kesalahan ini: \[tempel kesalahan]. perbaiki dan verifikasi build berhasil. tangani penyebab akar, jangan tekan kesalahan"*                                                                      |

Setelah pemeriksaan ada, putuskan seberapa ketat pemeriksaan tersebut membatasi penghentian:

* **Dalam satu prompt**: minta Claude menjalankan pemeriksaan dan melakukan iterasi dalam pesan yang sama, seperti dalam tabel di atas.
* **Sepanjang sesi**: atur pemeriksaan sebagai [kondisi `/goal`](/docs/id/goal). Evaluator terpisah memeriksa ulang setelah setiap giliran dan Claude terus bekerja sampai kondisi terpenuhi.
* **Sebagai gerbang deterministik**: [hook Stop](/docs/id/hooks#stop) menjalankan pemeriksaan Anda sebagai skrip dan memblokir giliran dari berakhir sampai lulus. Claude Code menimpa hook dan mengakhiri giliran setelah 8 blok berturut-turut.
* **Dengan pendapat kedua**: [subagent verifikasi](/docs/id/sub-agents) atau [alur kerja dinamis](/docs/id/workflows) yang memeriksa temuannya sendiri memiliki model segar yang mencoba menyangkal hasil, sehingga agen yang melakukan pekerjaan bukan yang menilainya.

Setiap langkah menukar setup untuk perhatian. Versi prompt berfungsi pada tugas apa pun hari ini. Versi `/goal` dan Stop hook adalah yang memungkinkan run tanpa pengawasan selesai dengan benar tanpa Anda.

Biarkan Claude menunjukkan bukti daripada menegaskan kesuksesan: output tes, perintah yang dijalankannya dan apa yang dikembalikannya, atau tangkapan layar hasilnya. Meninjau bukti lebih cepat daripada menjalankan kembali verifikasi sendiri, dan ini berfungsi untuk sesi yang tidak Anda saksikan.

***

<h2 id="explore-first-then-plan-then-code">
  Jelajahi terlebih dahulu, kemudian rencanakan, kemudian kode
</h2>

<Tip>
  Pisahkan penelitian dan perencanaan dari implementasi untuk menghindari menyelesaikan masalah yang salah.
</Tip>

Membiarkan Claude langsung melompat ke pengkodean dapat menghasilkan kode yang menyelesaikan masalah yang salah. Gunakan [plan mode](/docs/id/permission-modes#analyze-before-you-edit-with-plan-mode) untuk memisahkan eksplorasi dari eksekusi.

Alur kerja yang direkomendasikan memiliki empat fase:

<Steps>
  <Step title="Jelajahi">
    Masukkan plan mode. Claude membaca file dan menjawab pertanyaan tanpa membuat perubahan.

    ```txt claude (plan mode) theme={null}
    read /src/auth and understand how we handle sessions and login.
    also look at how we manage environment variables for secrets.
    ```
  </Step>

  <Step title="Rencanakan">
    Minta Claude untuk membuat rencana implementasi terperinci.

    ```txt claude (plan mode) theme={null}
    I want to add Google OAuth. What files need to change?
    What's the session flow? Create a plan.
    ```

    Tekan `Ctrl+G` untuk membuka rencana di editor teks Anda untuk pengeditan langsung sebelum Claude melanjutkan.
  </Step>

  <Step title="Implementasikan">
    Beralih keluar dari plan mode dan biarkan Claude kode, memverifikasi terhadap rencananya.

    ```txt claude (default mode) theme={null}
    implement the OAuth flow from your plan. write tests for the
    callback handler, run the test suite and fix any failures.
    ```
  </Step>

  <Step title="Komit">
    Minta Claude untuk melakukan komit dengan pesan deskriptif dan membuat PR.

    ```txt claude (default mode) theme={null}
    commit with a descriptive message and open a PR
    ```
  </Step>
</Steps>

<Callout>
  Plan mode berguna, tetapi juga menambah overhead.

  Untuk tugas di mana cakupannya jelas dan perbaikannya kecil (seperti memperbaiki typo, menambahkan baris log, atau mengganti nama variabel) minta Claude untuk melakukannya secara langsung.

  Perencanaan paling berguna ketika Anda tidak yakin tentang pendekatannya, ketika perubahan memodifikasi beberapa file, atau ketika Anda tidak terbiasa dengan kode yang dimodifikasi. Jika Anda dapat menjelaskan diff dalam satu kalimat, lewati rencana.
</Callout>

***

<h2 id="provide-specific-context-in-your-prompts">
  Berikan konteks spesifik dalam prompt Anda
</h2>

<Tip>
  Semakin tepat instruksi Anda, semakin sedikit koreksi yang Anda butuhkan.
</Tip>

Claude dapat menyimpulkan niat, tetapi tidak dapat membaca pikiran Anda. Referensikan file spesifik, sebutkan batasan, dan tunjukkan pola contoh.

| Strategi                                                                                         | Sebelum                                              | Sesudah                                                                                                                                                                                                                                                                                                                                                                    |
| ------------------------------------------------------------------------------------------------ | ---------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Batasi tugas.** Tentukan file mana, skenario apa, dan preferensi pengujian.                    | *"tambahkan tes untuk foo.py"*                       | *"tulis tes untuk foo.py yang mencakup kasus tepi di mana pengguna logout. hindari mock."*                                                                                                                                                                                                                                                                                 |
| **Tunjukkan sumber.** Arahkan Claude ke sumber yang dapat menjawab pertanyaan.                   | *"mengapa ExecutionFactory memiliki api yang aneh?"* | *"lihat melalui riwayat git ExecutionFactory dan ringkas bagaimana api-nya menjadi seperti ini"*                                                                                                                                                                                                                                                                           |
| **Referensikan pola yang ada.** Tunjukkan Claude pola dalam basis kode Anda.                     | *"tambahkan widget kalender"*                        | *"lihat bagaimana widget yang ada diimplementasikan di halaman beranda untuk memahami pola. HotDogWidget.php adalah contoh yang baik. ikuti pola untuk mengimplementasikan widget kalender baru yang memungkinkan pengguna memilih bulan dan paginate maju/mundur untuk memilih tahun. bangun dari awal tanpa perpustakaan selain yang sudah digunakan dalam basis kode."* |
| **Jelaskan gejala.** Berikan gejala, lokasi yang mungkin, dan apa "diperbaiki" terlihat seperti. | *"perbaiki bug login"*                               | *"pengguna melaporkan bahwa login gagal setelah timeout sesi. periksa alur auth di src/auth/, terutama penyegaran token. tulis tes yang gagal yang mereproduksi masalah, kemudian perbaiki"*                                                                                                                                                                               |

Prompt yang samar dapat berguna ketika Anda mengeksplorasi dan dapat mengubah arah. Prompt seperti `"apa yang akan Anda tingkatkan dalam file ini?"` dapat mengungkap hal-hal yang tidak akan Anda pikirkan untuk ditanyakan.

<h3 id="provide-rich-content">
  Berikan konten kaya
</h3>

<Tip>
  Gunakan `@` untuk mereferensikan file, tempel tangkapan layar/gambar, atau pipa data secara langsung.
</Tip>

Anda dapat memberikan data kaya kepada Claude dalam beberapa cara:

* **Referensikan file dengan `@`** alih-alih menjelaskan di mana kode berada. Claude membaca file sebelum merespons.
* **Tempel gambar secara langsung**. Salin/tempel atau seret dan lepas gambar ke dalam prompt.
* **Berikan URL** untuk dokumentasi dan referensi API. Gunakan `/permissions` untuk allowlist domain yang sering digunakan.
* **Pipa data** dengan menjalankan `cat error.log | claude` untuk mengirim konten file secara langsung.
* **Biarkan Claude mengambil apa yang dibutuhkan**. Beri tahu Claude untuk menarik konteks sendiri menggunakan perintah Bash, alat MCP, atau dengan membaca file.

***

<h2 id="configure-your-environment">
  Konfigurasi lingkungan Anda
</h2>

Beberapa langkah setup membuat Claude Code jauh lebih efektif di semua sesi Anda. Untuk gambaran lengkap fitur ekstensi dan kapan menggunakan masing-masing, lihat [Perluas Claude Code](/docs/id/features-overview).

<h3 id="write-an-effective-claude-md">
  Tulis CLAUDE.md yang efektif
</h3>

<Tip>
  Jalankan `/init` untuk menghasilkan file CLAUDE.md pemula berdasarkan struktur proyek Anda saat ini, kemudian perbaiki seiring waktu.
</Tip>

CLAUDE.md adalah file khusus yang dibaca Claude di awal setiap percakapan. Sertakan perintah Bash, gaya kode, dan aturan alur kerja. Ini memberikan Claude konteks persisten yang tidak dapat disimpulkan dari kode saja.

Perintah `/init` menganalisis basis kode Anda untuk mendeteksi sistem build, kerangka kerja tes, dan pola kode, memberikan Anda fondasi solid untuk disempurnakan.

Tidak ada format yang diperlukan untuk file CLAUDE.md, tetapi tetap singkat dan mudah dibaca manusia. Sebagai contoh:

```markdown CLAUDE.md theme={null}
# Code style
- Use ES modules (import/export) syntax, not CommonJS (require)
- Destructure imports when possible (eg. import { foo } from 'bar')

# Workflow
- Be sure to typecheck when you're done making a series of code changes
- Prefer running single tests, and not the whole test suite, for performance
```

CLAUDE.md dimuat setiap sesi, jadi hanya sertakan hal-hal yang berlaku secara luas. Untuk pengetahuan domain atau alur kerja yang hanya relevan kadang-kadang, gunakan [skills](/docs/id/skills) sebagai gantinya. Claude memuat mereka sesuai permintaan tanpa membengkak setiap percakapan.

Tetap ringkas. Untuk setiap baris, tanyakan: *"Apakah menghapus ini akan menyebabkan Claude membuat kesalahan?"* Jika tidak, potong. File CLAUDE.md yang membengkak menyebabkan Claude mengabaikan instruksi aktual Anda!

| ✅ Sertakan                                                    | ❌ Kecualikan                                                     |
| ------------------------------------------------------------- | ---------------------------------------------------------------- |
| Perintah Bash yang tidak dapat ditebak Claude                 | Apa pun yang dapat diketahui Claude dengan membaca kode          |
| Aturan gaya kode yang berbeda dari default                    | Konvensi bahasa standar yang sudah diketahui Claude              |
| Instruksi pengujian dan test runner pilihan                   | Dokumentasi API terperinci (tautkan ke dokumen sebagai gantinya) |
| Etiket repositori (penamaan cabang, konvensi PR)              | Informasi yang berubah sering                                    |
| Keputusan arsitektur khusus untuk proyek Anda                 | Penjelasan panjang atau tutorial                                 |
| Keanehan lingkungan pengembang (variabel env yang diperlukan) | Praktik yang jelas sendiri seperti "tulis kode yang bersih"      |
| Gotcha umum atau perilaku yang tidak jelas                    | Deskripsi file demi file dari basis kode                         |

Jika Claude terus melakukan sesuatu yang tidak Anda inginkan meskipun memiliki aturan melawannya, file mungkin terlalu panjang dan aturan hilang. Jika Claude mengajukan pertanyaan yang dijawab di CLAUDE.md, frasenya mungkin ambigu. Perlakukan CLAUDE.md seperti kode: tinjau saat ada yang salah, pangkas secara teratur, dan uji perubahan dengan mengamati apakah perilaku Claude benar-benar bergeser.

Anda dapat menyesuaikan instruksi dengan menambahkan penekanan (misalnya, "PENTING" atau "ANDA HARUS") untuk meningkatkan kepatuhan. Periksa CLAUDE.md ke dalam git sehingga tim Anda dapat berkontribusi. File ini meningkat nilainya seiring waktu.

File CLAUDE.md dapat mengimpor file tambahan menggunakan sintaks `@path/to/import`:

```markdown CLAUDE.md theme={null}
See @README.md for project overview and @package.json for available npm commands.

# Additional Instructions
- Git workflow: @docs/git-instructions.md
- Personal overrides: @~/.claude/my-project-instructions.md
```

Anda dapat menempatkan file CLAUDE.md di beberapa lokasi:

* **Folder home (`~/.claude/CLAUDE.md`)**: berlaku untuk semua sesi Claude
* **Root proyek (`./CLAUDE.md`)**: periksa ke dalam git untuk dibagikan dengan tim Anda
* **Root proyek (`./CLAUDE.local.md`)**: catatan khusus proyek pribadi; tambahkan file ini ke `.gitignore` Anda sehingga tidak dibagikan dengan tim Anda
* **Direktori induk**: berguna untuk monorepo di mana `root/CLAUDE.md` dan `root/foo/CLAUDE.md` ditarik secara otomatis
* **Direktori anak**: Claude menarik file CLAUDE.md anak sesuai permintaan saat bekerja dengan file di direktori tersebut

<h3 id="configure-permissions">
  Konfigurasi izin
</h3>

<Tip>
  Gunakan [auto mode](/docs/id/permission-modes#eliminate-prompts-with-auto-mode) untuk membiarkan classifier menangani persetujuan, `/permissions` untuk allowlist perintah spesifik, atau `/sandbox` untuk isolasi tingkat OS. Masing-masing mengurangi gangguan sambil membuat Anda tetap mengendalikan.
</Tip>

Secara default, Claude Code meminta izin untuk tindakan yang mungkin memodifikasi sistem Anda: penulisan file, perintah Bash, alat MCP, dll. Ini aman tetapi membosankan. Setelah persetujuan kesepuluh Anda tidak benar-benar meninjau lagi, Anda hanya mengklik. Ada tiga cara untuk mengurangi gangguan ini:

* **Auto mode**: model classifier terpisah meninjau perintah dan memblokir hanya apa yang terlihat berisiko: eskalasi cakupan, infrastruktur yang tidak dikenal, atau tindakan yang didorong konten bermusuhan. Terbaik ketika Anda mempercayai arah umum tugas tetapi tidak ingin mengklik setiap langkah
* **Allowlist izin**: izinkan alat spesifik yang Anda tahu aman, seperti `npm run lint` atau `git commit`
* **Sandboxing**: aktifkan isolasi tingkat OS yang membatasi akses sistem file dan jaringan, memungkinkan Claude bekerja lebih bebas dalam batas yang ditentukan

Baca lebih lanjut tentang [permission modes](/docs/id/permission-modes), [permission rules](/docs/id/permissions), dan [sandboxing](/docs/id/sandboxing).

<h3 id="use-cli-tools">
  Gunakan alat CLI
</h3>

<Tip>
  Beri tahu Claude Code untuk menggunakan alat CLI seperti `gh`, `aws`, `gcloud`, dan `sentry-cli` saat berinteraksi dengan layanan eksternal.
</Tip>

Alat CLI adalah cara paling efisien konteks untuk berinteraksi dengan layanan eksternal. Jika Anda menggunakan GitHub, instal CLI `gh`. Claude tahu cara menggunakannya untuk membuat masalah, membuka pull request, dan membaca komentar. Tanpa `gh`, Claude masih dapat menggunakan GitHub API, tetapi permintaan yang tidak diautentikasi sering kali mencapai batas laju.

Claude juga efektif dalam mempelajari alat CLI yang tidak diketahuinya. Coba prompt seperti `Use 'foo-cli-tool --help' to learn about foo tool, then use it to solve A, B, C.`

<h3 id="connect-mcp-servers">
  Hubungkan server MCP
</h3>

<Tip>
  Jalankan `claude mcp add` untuk menghubungkan alat eksternal seperti Notion, Figma, atau database Anda.
</Tip>

Dengan [server MCP](/docs/id/mcp), Anda dapat meminta Claude untuk mengimplementasikan fitur dari pelacak masalah, query database, menganalisis data pemantauan, mengintegrasikan desain dari Figma, dan mengotomatisasi alur kerja.

<h3 id="set-up-hooks">
  Atur hooks
</h3>

<Tip>
  Gunakan hooks untuk tindakan yang harus terjadi setiap kali tanpa pengecualian.
</Tip>

[Hooks](/docs/id/hooks-guide) menjalankan skrip secara otomatis pada titik tertentu dalam alur kerja Claude. Tidak seperti instruksi CLAUDE.md yang bersifat penasihat, hooks bersifat deterministik dan menjamin tindakan terjadi.

Claude dapat menulis hooks untuk Anda. Coba prompt seperti *"Tulis hook yang menjalankan eslint setelah setiap pengeditan file"* atau *"Tulis hook yang memblokir penulisan ke folder migrasi."* Edit `.claude/settings.json` secara langsung untuk mengonfigurasi hooks dengan tangan, dan jalankan `/hooks` untuk menjelajahi apa yang dikonfigurasi.

<h3 id="create-skills">
  Buat skills
</h3>

<Tip>
  Buat file `SKILL.md` di `.claude/skills/` untuk memberikan Claude pengetahuan domain dan alur kerja yang dapat digunakan kembali.
</Tip>

[Skills](/docs/id/skills) memperluas pengetahuan Claude dengan informasi khusus untuk proyek, tim, atau domain Anda. Claude menerapkannya secara otomatis saat relevan, atau Anda dapat menginvokannya secara langsung dengan `/skill-name`.

Buat skill dengan menambahkan direktori dengan `SKILL.md` ke `.claude/skills/`:

```markdown .claude/skills/api-conventions/SKILL.md theme={null}
---
name: api-conventions
description: REST API design conventions for our services
---
# API Conventions
- Use kebab-case for URL paths
- Use camelCase for JSON properties
- Always include pagination for list endpoints
- Version APIs in the URL path (/v1/, /v2/)
```

Skills juga dapat mendefinisikan alur kerja yang dapat digunakan kembali yang Anda panggil secara langsung:

```markdown .claude/skills/fix-issue/SKILL.md theme={null}
---
name: fix-issue
description: Fix a GitHub issue
disable-model-invocation: true
---
Analyze and fix the GitHub issue: $ARGUMENTS.

1. Use `gh issue view` to get the issue details
2. Understand the problem described in the issue
3. Search the codebase for relevant files
4. Implement the necessary changes to fix the issue
5. Write and run tests to verify the fix
6. Ensure code passes linting and type checking
7. Create a descriptive commit message
8. Push and create a PR
```

Jalankan `/fix-issue 1234` untuk menginvokannya. Gunakan `disable-model-invocation: true` untuk alur kerja dengan efek samping yang ingin Anda picu secara manual.

<h3 id="create-custom-subagents">
  Buat subagent khusus
</h3>

<Tip>
  Tentukan asisten khusus di `.claude/agents/` yang dapat didelegasikan Claude untuk tugas terisolasi.
</Tip>

[Subagents](/docs/id/sub-agents) berjalan dalam konteks mereka sendiri dengan set alat yang diizinkan mereka sendiri. Mereka berguna untuk tugas yang membaca banyak file atau memerlukan fokus khusus tanpa mengacaukan percakapan utama Anda.

```markdown .claude/agents/security-reviewer.md theme={null}
---
name: security-reviewer
description: Reviews code for security vulnerabilities
tools: Read, Grep, Glob, Bash
model: opus
---
You are a senior security engineer. Review code for:
- Injection vulnerabilities (SQL, XSS, command injection)
- Authentication and authorization flaws
- Secrets or credentials in code
- Insecure data handling

Provide specific line references and suggested fixes.
```

Beri tahu Claude untuk menggunakan subagent secara eksplisit: *"Gunakan subagent untuk meninjau kode ini untuk masalah keamanan."*

<h3 id="install-plugins">
  Instal plugins
</h3>

<Tip>
  Jalankan `/plugin` untuk menjelajahi marketplace. Plugins menambahkan skills, alat, dan integrasi tanpa konfigurasi.
</Tip>

[Plugins](/docs/id/plugins) menggabungkan skills, hooks, subagents, dan server MCP menjadi satu unit yang dapat diinstal dari komunitas dan Anthropic. Jika Anda bekerja dengan bahasa yang diketik, instal [plugin code intelligence](/docs/id/discover-plugins#code-intelligence) untuk memberikan Claude navigasi simbol presisi dan deteksi kesalahan otomatis setelah pengeditan.

Untuk panduan memilih antara skills, subagents, hooks, dan MCP, lihat [Perluas Claude Code](/docs/id/features-overview#match-features-to-your-goal).

***

<h2 id="communicate-effectively">
  Berkomunikasi secara efektif
</h2>

Cara Anda berkomunikasi dengan Claude Code secara signifikan mempengaruhi kualitas hasil.

<h3 id="ask-codebase-questions">
  Tanyakan pertanyaan basis kode
</h3>

<Tip>
  Tanyakan Claude pertanyaan yang akan Anda tanyakan kepada insinyur senior.
</Tip>

Saat onboarding ke basis kode baru, gunakan Claude Code untuk pembelajaran dan eksplorasi. Anda dapat mengajukan Claude pertanyaan yang sama seperti yang Anda tanyakan kepada insinyur lain:

* Bagaimana cara logging bekerja?
* Bagaimana cara membuat endpoint API baru?
* Apa yang dilakukan `async move { ... }` pada baris 134 dari `foo.rs`?
* Kasus tepi apa yang ditangani `CustomerOnboardingFlowImpl`?
* Mengapa kode ini memanggil `foo()` alih-alih `bar()` pada baris 333?

Menggunakan Claude Code dengan cara ini adalah alur kerja onboarding yang efektif, meningkatkan waktu ramp-up dan mengurangi beban pada insinyur lain. Tidak ada prompt khusus yang diperlukan: tanyakan pertanyaan secara langsung.

<h3 id="let-claude-interview-you">
  Biarkan Claude mewawancarai Anda
</h3>

<Tip>
  Untuk fitur yang lebih besar, biarkan Claude mewawancarai Anda terlebih dahulu. Mulai dengan prompt minimal dan minta Claude untuk mewawancarai Anda menggunakan alat `AskUserQuestion`.
</Tip>

Claude menanyakan tentang hal-hal yang mungkin belum Anda pertimbangkan, termasuk implementasi teknis, UI/UX, kasus tepi, dan trade-off.

```text theme={null}
I want to build [brief description]. Interview me in detail using the AskUserQuestion tool.

Ask about technical implementation, UI/UX, edge cases, concerns, and tradeoffs. Don't ask obvious questions, dig into the hard parts I might not have considered.

Keep interviewing until we've covered everything, then write a complete spec to SPEC.md.
```

Setelah spesifikasi selesai, mulai sesi segar untuk menjalankannya. Sesi baru memiliki konteks bersih yang fokus sepenuhnya pada implementasi, dan Anda memiliki spesifikasi tertulis untuk direferensikan.

Spesifikasi yang paling berguna adalah mandiri: mereka menamai file dan antarmuka yang terlibat, menyatakan apa yang berada di luar cakupan, dan diakhiri dengan langkah verifikasi end-to-end yang membuktikan fitur berfungsi. Waktu yang dihabiskan untuk membuat spesifikasi presisi memberikan hasil lebih banyak daripada waktu yang dihabiskan untuk menonton implementasi.

***

<h2 id="manage-your-session">
  Kelola sesi Anda
</h2>

Percakapan bersifat persisten dan dapat dibalik. Gunakan ini untuk keuntungan Anda!

<h3 id="course-correct-early-and-often">
  Perbaiki arah dengan cepat dan sering
</h3>

<Tip>
  Perbaiki Claude segera setelah Anda melihatnya keluar jalur.
</Tip>

Hasil terbaik datang dari loop umpan balik yang ketat. Meskipun Claude kadang-kadang menyelesaikan masalah dengan sempurna pada upaya pertama, memperbaikinya dengan cepat umumnya menghasilkan solusi yang lebih baik lebih cepat.

* **`Esc`**: hentikan Claude di tengah-tindakan dengan tombol `Esc`. Konteks dipertahankan, jadi Anda dapat mengarahkan kembali.
* **`Esc + Esc` atau `/rewind`**: tekan `Esc` dua kali atau jalankan `/rewind` untuk membuka menu rewind dan mengembalikan percakapan dan status kode sebelumnya, atau ringkas dari pesan yang dipilih.
* **`"Undo that"`**: biarkan Claude mengembalikan perubahannya.
* **`/clear`**: atur ulang konteks antara tugas yang tidak terkait. Sesi panjang dengan konteks yang tidak relevan dapat mengurangi kinerja.

Jika Anda telah memperbaiki Claude lebih dari dua kali pada masalah yang sama dalam satu sesi, konteks penuh dengan pendekatan yang gagal. Jalankan `/clear` dan mulai segar dengan prompt yang lebih spesifik yang menggabungkan apa yang Anda pelajari. Sesi bersih dengan prompt yang lebih baik hampir selalu mengungguli sesi panjang dengan koreksi terakumulasi.

<h3 id="manage-context-aggressively">
  Kelola konteks secara agresif
</h3>

<Tip>
  Jalankan `/clear` antara tugas yang tidak terkait untuk mengatur ulang konteks.
</Tip>

Claude Code secara otomatis mengompaksi riwayat percakapan saat Anda mendekati batas konteks, yang mempertahankan kode dan keputusan penting sambil membebaskan ruang.

Selama sesi panjang, jendela konteks Claude dapat terisi dengan percakapan yang tidak relevan, konten file, dan perintah. Ini dapat mengurangi kinerja dan kadang-kadang mengalihkan Claude.

* Gunakan `/clear` sering antara tugas untuk mengatur ulang jendela konteks sepenuhnya
* Ketika auto compaction dipicu, Claude meringkas apa yang paling penting, termasuk pola kode, status file, dan keputusan kunci
* Untuk kontrol lebih, jalankan `/compact <instructions>`, seperti `/compact Focus on the API changes`
* Untuk mengompaksi hanya bagian dari percakapan, gunakan `Esc + Esc` atau `/rewind`, pilih checkpoint pesan, dan pilih **Summarize from here** atau **Summarize up to here**. Yang pertama mengondensasi pesan dari titik itu maju sambil menjaga konteks awal tetap utuh; yang kedua mengondensasi pesan awal sambil menjaga pesan terbaru tetap lengkap. Lihat [Restore vs. summarize](/docs/id/checkpointing#restore-vs-summarize).
* Sesuaikan perilaku compaction di CLAUDE.md dengan instruksi seperti `"When compacting, always preserve the full list of modified files and any test commands"` untuk memastikan konteks kritis bertahan dari ringkasan
* Untuk pertanyaan cepat yang tidak perlu tetap dalam konteks, gunakan [`/btw`](/docs/id/interactive-mode#side-questions-with-%2Fbtw). Jawabannya muncul dalam overlay yang dapat ditutup dan tidak pernah memasuki riwayat percakapan, jadi Anda dapat memeriksa detail tanpa menumbuhkan konteks.

<h3 id="use-subagents-for-investigation">
  Gunakan subagents untuk investigasi
</h3>

<Tip>
  Delegasikan penelitian dengan `"use subagents to investigate X"`. Mereka mengeksplorasi dalam konteks terpisah, menjaga percakapan utama Anda bersih untuk implementasi.
</Tip>

Karena konteks adalah batasan fundamental Anda, subagents adalah salah satu alat paling kuat yang tersedia. Ketika Claude meneliti basis kode, ia membaca banyak file, semuanya mengonsumsi konteks Anda. Subagents berjalan dalam jendela konteks terpisah dan melaporkan kembali ringkasan:

```text theme={null}
Use subagents to investigate how our authentication system handles token
refresh, and whether we have any existing OAuth utilities I should reuse.
```

Subagent mengeksplorasi basis kode, membaca file yang relevan, dan melaporkan kembali dengan temuan, semuanya tanpa mengacaukan percakapan utama Anda.

Anda juga dapat menggunakan subagents untuk verifikasi setelah Claude mengimplementasikan sesuatu:

```text theme={null}
use a subagent to review this code for edge cases
```

<h3 id="rewind-with-checkpoints">
  Rewind dengan checkpoints
</h3>

<Tip>
  Setiap prompt yang Anda kirim membuat checkpoint. Anda dapat mengembalikan percakapan, kode, atau keduanya ke checkpoint sebelumnya.
</Tip>

Claude secara otomatis membuat snapshot file sebelum setiap perubahan sehingga checkpoint dapat mengembalikannya. Tekan Escape dua kali atau jalankan `/rewind` untuk membuka menu rewind. Anda dapat mengembalikan percakapan saja, mengembalikan kode saja, mengembalikan keduanya, atau meringkas dari pesan yang dipilih. Lihat [Checkpointing](/docs/id/checkpointing) untuk detail.

Alih-alih merencanakan setiap langkah dengan hati-hati, Anda dapat memberi tahu Claude untuk mencoba sesuatu yang berisiko. Jika tidak berhasil, rewind dan coba pendekatan berbeda. Checkpoints bertahan di seluruh sesi, jadi Anda dapat menutup terminal dan masih rewind nanti.

<Warning>
  Checkpoints hanya melacak perubahan yang dibuat melalui alat pengeditan file Claude. Perubahan yang dibuat melalui perintah Bash atau proses eksternal tidak ditangkap. Ini bukan pengganti git.
</Warning>

<h3 id="resume-conversations">
  Lanjutkan percakapan
</h3>

<Tip>
  Beri nama sesi dengan `/rename` dan perlakukan mereka seperti cabang: setiap alur kerja mendapatkan konteks persisten sendiri.
</Tip>

Claude Code menyimpan percakapan secara lokal, jadi ketika tugas mencakup beberapa sesi Anda tidak harus menjelaskan ulang konteksnya. Jalankan `claude --continue` untuk melanjutkan dari sesi terbaru, atau `claude --resume` untuk memilih dari daftar. Berikan sesi nama deskriptif seperti `oauth-migration` sehingga Anda dapat menemukannya nanti. Lihat [Manage sessions](/docs/id/sessions) untuk set lengkap kontrol resume, branch, dan naming.

***

<h2 id="automate-and-scale">
  Otomatisasi dan skalakan
</h2>

Setelah Anda efektif dengan satu Claude, kalikan output Anda dengan sesi paralel, mode non-interaktif, dan pola fan-out.

Semuanya sejauh ini mengasumsikan satu manusia, satu Claude, dan satu percakapan. Tetapi Claude Code skalakan secara horizontal. Teknik di bagian ini menunjukkan bagaimana Anda dapat melakukan lebih banyak.

<h3 id="run-non-interactive-mode">
  Jalankan mode non-interaktif
</h3>

<Tip>
  Gunakan `claude -p "prompt"` di CI, pre-commit hooks, atau skrip. Tambahkan `--output-format stream-json --verbose` untuk output JSON streaming.
</Tip>

Dengan `claude -p "your prompt"`, Anda dapat menjalankan Claude secara non-interaktif, tanpa prompt interaktif. Jalankan masih membuat sesi yang dapat dilanjutkan kecuali Anda melewatkan `--no-session-persistence`. [Mode non-interaktif](/docs/id/headless) adalah cara Anda mengintegrasikan Claude ke dalam pipeline CI, pre-commit hooks, atau alur kerja otomatis apa pun. Format output memungkinkan Anda mengurai hasil secara terprogram: teks biasa, JSON, atau JSON streaming.

```bash theme={null}
# One-off queries
claude -p "Explain what this project does"

# Structured output for scripts
claude -p "List all API endpoints" --output-format json

# Streaming for real-time processing
claude -p "Analyze this log file" --output-format stream-json --verbose
```

<h3 id="run-multiple-claude-sessions">
  Jalankan beberapa sesi Claude
</h3>

<Tip>
  Jalankan beberapa sesi Claude secara paralel untuk mempercepat pengembangan, menjalankan eksperimen terisolasi, atau memulai alur kerja kompleks.
</Tip>

Pilih pendekatan paralel yang sesuai dengan seberapa banyak koordinasi yang ingin Anda lakukan sendiri:

* [Worktrees](/docs/id/worktrees): jalankan sesi CLI terpisah dalam checkout git terisolasi sehingga edit tidak bertabrakan
* [Aplikasi desktop](/docs/id/desktop#work-in-parallel-with-sessions): kelola beberapa sesi lokal secara visual, masing-masing dalam worktree-nya sendiri
* [Claude Code di web](/docs/id/claude-code-on-the-web): jalankan sesi pada infrastruktur cloud yang dikelola Anthropic dalam VM terisolasi
* [Tim agen](/docs/id/agent-teams): koordinasi otomatis dari beberapa sesi dengan tugas bersama, pesan, dan pemimpin tim

Selain paralelisasi pekerjaan, beberapa sesi memungkinkan alur kerja yang berfokus pada kualitas. Konteks segar meningkatkan tinjauan kode karena Claude tidak akan bias terhadap kode yang baru saja ditulisnya.

Sebagai contoh, gunakan pola Writer/Reviewer:

| Sesi A (Penulis)                                                             | Sesi B (Peninjau)                                                                                                                                     |
| ---------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------- |
| `Implementasikan rate limiter untuk endpoint API kami`                       |                                                                                                                                                       |
|                                                                              | `Tinjau implementasi rate limiter di @src/middleware/rateLimiter.ts. Cari kasus tepi, kondisi race, dan konsistensi dengan pola middleware yang ada.` |
| `Berikut adalah umpan balik tinjauan: [output Sesi B]. Tangani masalah ini.` |                                                                                                                                                       |

Anda dapat melakukan sesuatu yang serupa dengan tes: biarkan satu Claude menulis tes, kemudian yang lain menulis kode untuk lulus.

<h3 id="fan-out-across-files">
  Fan out di seluruh file
</h3>

<Tip>
  Loop melalui tugas memanggil `claude -p` untuk masing-masing. Gunakan `--allowedTools` untuk cakupan izin untuk operasi batch.
</Tip>

Untuk migrasi besar atau analisis, Anda dapat mendistribusikan pekerjaan di seluruh banyak invokasi Claude paralel:

<Steps>
  <Step title="Hasilkan daftar tugas">
    Biarkan Claude membuat daftar semua file yang perlu dimigrasikan (misalnya, `list all 2,000 Python files that need migrating`)
  </Step>

  <Step title="Tulis skrip untuk loop melalui daftar">
    ```bash theme={null}
    for file in $(cat files.txt); do
      claude -p "Migrate $file from React to Vue. Return OK or FAIL." \
        --allowedTools "Edit,Bash(git commit *)"
    done
    ```
  </Step>

  <Step title="Uji pada beberapa file, kemudian jalankan dalam skala">
    Perbaiki prompt Anda berdasarkan apa yang salah dengan 2-3 file pertama, kemudian jalankan pada set lengkap. Bendera `--allowedTools` membatasi apa yang dapat dilakukan Claude, yang penting ketika Anda menjalankan tanpa pengawasan.
  </Step>
</Steps>

Anda juga dapat mengintegrasikan Claude ke dalam pipeline pemrosesan/data yang ada:

```bash theme={null}
claude -p "<your prompt>" --output-format json | your_command
```

Gunakan `--verbose` untuk debugging selama pengembangan, dan matikan dalam produksi.

<h3 id="run-autonomously-with-auto-mode">
  Jalankan secara otonom dengan auto mode
</h3>

Untuk eksekusi tanpa gangguan dengan pemeriksaan keamanan latar belakang, gunakan [auto mode](/docs/id/permission-modes#eliminate-prompts-with-auto-mode). Model classifier meninjau perintah sebelum dijalankan, memblokir eskalasi cakupan, infrastruktur yang tidak dikenal, dan tindakan yang didorong konten bermusuhan sambil membiarkan pekerjaan rutin berjalan tanpa prompt.

```bash theme={null}
claude --permission-mode auto -p "fix all lint errors"
```

Untuk run non-interaktif dengan bendera `-p`, auto mode membatalkan jika classifier secara berulang memblokir tindakan, karena tidak ada pengguna untuk kembali. Lihat [kapan auto mode kembali](/docs/id/permission-modes#when-auto-mode-falls-back) untuk ambang batas.

<h3 id="add-an-adversarial-review-step">
  Tambahkan langkah tinjauan adversarial
</h3>

<Tip>
  Sebelum menganggap tugas selesai, biarkan subagent meninjau diff dalam konteks segar dan melaporkan kesenjangan.
</Tip>

Semakin lama Claude bekerja tanpa pengawasan, semakin penting pemeriksaan independen sebelum Anda menghitung pekerjaan sebagai selesai. Peninjau yang berjalan dalam konteks [subagent](/docs/id/sub-agents) segar hanya melihat diff dan kriteria yang Anda berikan, bukan penalaran yang menghasilkan perubahan, sehingga mengevaluasi hasil berdasarkan istilahnya sendiri.

Untuk pemeriksaan kebenaran, jalankan skill [`/code-review`](/docs/id/commands) yang disertakan, yang meninjau diff saat ini untuk bug dalam subagent segar dan mengembalikan temuan ke sesi. Untuk memeriksa diff terhadap rencana Anda, tulis prompt tinjauan sendiri. Beri nama pekerjaan untuk diperiksa, rencana untuk memeriksanya, dan apa yang dihitung sebagai temuan:

```text theme={null}
Gunakan subagent untuk meninjau diff rate limiter terhadap PLAN.md. Periksa bahwa
setiap persyaratan diimplementasikan, kasus tepi yang terdaftar memiliki tes, dan
tidak ada yang di luar cakupan tugas yang berubah. Laporkan kesenjangan, bukan preferensi gaya.
```

Karena peninjau berjalan sebagai subagent, sesi implementasi menerima kesenjangan secara langsung dan dapat memperbaikinya dan meninjau ulang tanpa Anda menyalin temuan antar jendela. Untuk run otonom yang lebih lama, [tim agen](/docs/id/agent-teams) dapat menjaga loop ini berjalan di seluruh banyak tugas sementara Anda spot-check temuan yang dicatat.

<Callout>
  Peninjau yang diminta untuk menemukan kesenjangan biasanya akan melaporkan beberapa, bahkan ketika pekerjaan itu solid, karena itulah yang diminta untuk dilakukan. Mengejar setiap temuan menyebabkan over-engineering: lapisan abstraksi ekstra, kode defensif, dan tes untuk kasus yang tidak dapat terjadi. Beri tahu peninjau untuk menandai hanya kesenjangan yang mempengaruhi kebenaran atau persyaratan yang dinyatakan, dan perlakukan sisanya sebagai opsional.
</Callout>

***

<h2 id="avoid-common-failure-patterns">
  Hindari pola kegagalan umum
</h2>

Ini adalah kesalahan umum. Mengenalinya lebih awal menghemat waktu:

* **Sesi kitchen sink.** Anda mulai dengan satu tugas, kemudian meminta Claude sesuatu yang tidak terkait, kemudian kembali ke tugas pertama. Konteks penuh dengan informasi yang tidak relevan.
  > **Perbaikan**: `/clear` antara tugas yang tidak terkait.
* **Mengoreksi berulang kali.** Claude melakukan sesuatu yang salah, Anda memperbaikinya, masih salah, Anda memperbaiki lagi. Konteks tercemar dengan pendekatan yang gagal.
  > **Perbaikan**: Setelah dua koreksi yang gagal, `/clear` dan tulis prompt awal yang lebih baik menggabungkan apa yang Anda pelajari.
* **CLAUDE.md yang terlalu spesifik.** Jika CLAUDE.md Anda terlalu panjang, Claude mengabaikan setengahnya karena aturan penting hilang dalam kebisingan.
  > **Perbaikan**: Pangkas tanpa ampun. Jika Claude sudah melakukan sesuatu dengan benar tanpa instruksi, hapus atau ubah menjadi hook.
* **Kesenjangan kepercayaan-kemudian-verifikasi.** Claude menghasilkan implementasi yang terlihat masuk akal tetapi tidak menangani kasus tepi.
  > **Perbaikan**: Selalu berikan verifikasi (tes, skrip, tangkapan layar). Jika Anda tidak dapat memverifikasinya, jangan kirimkan.
* **Eksplorasi tak terbatas.** Anda meminta Claude untuk "menyelidiki" sesuatu tanpa membatasinya. Claude membaca ratusan file, mengisi konteks.
  > **Perbaikan**: Batasi investigasi secara sempit atau gunakan subagents sehingga eksplorasi tidak mengonsumsi konteks utama Anda.

***

<h2 id="develop-your-intuition">
  Kembangkan intuisi Anda
</h2>

Pola dalam panduan ini bukan batu loncatan. Mereka adalah titik awal yang bekerja dengan baik secara umum, tetapi mungkin tidak optimal untuk setiap situasi.

Kadang-kadang Anda *harus* membiarkan konteks terakumulasi karena Anda mendalam dalam satu masalah kompleks dan riwayat berharga. Kadang-kadang Anda harus melewati perencanaan dan membiarkan Claude mengetahuinya karena tugas bersifat eksplorasi. Kadang-kadang prompt yang samar adalah tepat karena Anda ingin melihat bagaimana Claude menafsirkan masalah sebelum membatasinya.

Perhatikan apa yang berhasil. Ketika Claude menghasilkan output yang hebat, perhatikan apa yang Anda lakukan: struktur prompt, konteks yang Anda berikan, mode yang Anda gunakan. Ketika Claude berjuang, tanyakan mengapa. Apakah konteksnya terlalu bising? Prompt terlalu samar? Tugas terlalu besar untuk satu pass?

Seiring waktu, Anda akan mengembangkan intuisi yang tidak dapat ditangkap oleh panduan apa pun. Anda akan tahu kapan harus spesifik dan kapan harus terbuka, kapan harus merencanakan dan kapan harus mengeksplorasi, kapan harus menghapus konteks dan kapan harus membiarkannya terakumulasi.

<h2 id="related-resources">
  Sumber daya terkait
</h2>

* [Cara Claude Code Bekerja](/docs/id/how-claude-code-works): loop agentic, alat, dan manajemen konteks
* [Perluas Claude Code](/docs/id/features-overview): skills, hooks, MCP, subagents, dan plugins
* [Alur kerja umum](/docs/id/common-workflows): resep langkah demi langkah untuk debugging, pengujian, PR, dan lainnya
* [CLAUDE.md](/docs/id/memory): simpan konvensi proyek dan konteks persisten
