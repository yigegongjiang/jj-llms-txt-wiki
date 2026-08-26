> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Perluas Claude Code

> Pahami kapan menggunakan CLAUDE.md, Skills, subagents, hooks, MCP, dan plugins.

Claude Code menggabungkan model yang bernalar tentang kode Anda dengan [alat bawaan](/docs/id/how-claude-code-works#tools) untuk operasi file, pencarian, eksekusi, dan akses web. Alat bawaan mencakup sebagian besar tugas pengkodean. Panduan ini mencakup lapisan ekstensi: fitur yang Anda tambahkan untuk menyesuaikan apa yang Claude ketahui, menghubungkannya ke layanan eksternal, dan mengotomatisasi alur kerja.

<Note>
  Untuk cara loop agentic inti bekerja, lihat [Cara Claude Code Bekerja](/docs/id/how-claude-code-works).
</Note>

**Baru di Claude Code?** Mulai dengan [CLAUDE.md](/docs/id/memory) untuk konvensi proyek, kemudian tambahkan ekstensi lain [saat pemicu spesifik muncul](#build-your-setup-over-time).

<h2 id="overview">
  Ikhtisar
</h2>

Ekstensi terhubung ke bagian berbeda dari loop agentic:

* **[CLAUDE.md](/docs/id/memory)** menambahkan konteks persisten yang Claude lihat setiap sesi
* **[Skills](/docs/id/skills)** menambahkan pengetahuan yang dapat digunakan kembali dan alur kerja yang dapat dipanggil
* **[Code intelligence](/docs/id/tools-reference#lsp-tool-behavior)** menghubungkan Claude ke language server untuk navigasi tingkat simbol dan kesalahan tipe langsung
* **[MCP](/docs/id/mcp)** menghubungkan Claude ke layanan dan alat eksternal
* **[Subagents](/docs/id/sub-agents)** menjalankan loop mereka sendiri dalam konteks terisolasi, mengembalikan ringkasan
* **[Agent teams](/docs/id/agent-teams)** mengoordinasikan beberapa sesi independen dengan tugas bersama dan pesan peer-to-peer
* **[Hooks](/docs/id/hooks-guide)** berjalan pada acara siklus hidup dan dapat menjalankan skrip, permintaan HTTP, prompt, atau subagent
* **[Plugins](/docs/id/plugins)** dan **[marketplaces](/docs/id/plugin-marketplaces)** mengemas dan mendistribusikan fitur-fitur ini

[Skills](/docs/id/skills) adalah ekstensi paling fleksibel. Skill adalah file markdown yang berisi pengetahuan, alur kerja, atau instruksi. Anda dapat memanggil skills dengan perintah seperti `/deploy`, atau Claude dapat memuatnya secara otomatis ketika relevan. Skills dapat berjalan dalam percakapan Anda saat ini atau dalam konteks terisolasi melalui subagents.

<h2 id="match-features-to-your-goal">
  Cocokkan fitur dengan tujuan Anda
</h2>

Fitur berkisar dari konteks yang selalu aktif yang Claude lihat setiap sesi, hingga kemampuan on-demand yang dapat Anda atau Claude panggil, hingga otomasi latar belakang yang berjalan pada acara tertentu. Tabel di bawah menunjukkan apa yang tersedia dan kapan masing-masing masuk akal.

| Fitur                                                          | Apa yang dilakukannya                                                | Kapan menggunakannya                                                                   | Contoh                                                                                       |
| -------------------------------------------------------------- | -------------------------------------------------------------------- | -------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------- |
| **CLAUDE.md**                                                  | Konteks persisten dimuat setiap percakapan                           | Konvensi proyek, aturan "selalu lakukan X"                                             | "Gunakan pnpm, bukan npm. Jalankan tes sebelum commit."                                      |
| **Skill**                                                      | Instruksi, pengetahuan, dan alur kerja yang dapat digunakan Claude   | Konten yang dapat digunakan kembali, dokumen referensi, tugas yang dapat diulang       | `/deploy` menjalankan daftar periksa deployment Anda; skill dokumen API dengan pola endpoint |
| **Subagent**                                                   | Konteks eksekusi terisolasi yang mengembalikan hasil ringkasan       | Isolasi konteks, tugas paralel, pekerja khusus                                         | Tugas penelitian yang membaca banyak file tetapi hanya mengembalikan temuan kunci            |
| **[Agent teams](/docs/id/agent-teams)**                             | Mengoordinasikan beberapa sesi Claude Code independen                | Penelitian paralel, pengembangan fitur baru, debugging dengan hipotesis bersaing       | Spawn reviewer untuk memeriksa keamanan, performa, dan tes secara bersamaan                  |
| **[Code intelligence](/docs/id/tools-reference#lsp-tool-behavior)** | Navigasi language-server dan diagnostik                              | Bahasa yang diketik, basis kode besar di mana grep lambat atau tidak presisi           | Lompat ke definisi simbol daripada membaca seluruh file                                      |
| **MCP**                                                        | Terhubung ke layanan eksternal                                       | Data atau tindakan eksternal                                                           | Kueri database Anda, posting ke Slack, kontrol browser                                       |
| **Hook**                                                       | Skrip, permintaan HTTP, prompt, atau subagent yang dipicu oleh acara | Otomasi yang harus berjalan pada setiap acara yang cocok                               | Jalankan ESLint setelah setiap edit file                                                     |
| **[Artifact](/docs/id/artifacts)**                                  | Publikasikan output sesi sebagai halaman web pribadi yang interaktif | Output yang ingin Anda lihat atau bagikan secara visual daripada sebagai teks terminal | Garis waktu insiden yang diperbarui saat Claude menyelidiki                                  |

**[Plugins](/docs/id/plugins)** adalah lapisan pengemasan. Plugin menggabungkan skills, hooks, subagents, dan MCP servers menjadi satu unit yang dapat diinstal. Plugin skills memiliki namespace (seperti `/my-plugin:review`) sehingga beberapa plugin dapat hidup berdampingan. Gunakan plugins ketika Anda ingin menggunakan kembali setup yang sama di beberapa repositori atau mendistribusikan ke orang lain melalui **[marketplace](/docs/id/plugin-marketplaces)**.

<h3 id="build-your-setup-over-time">
  Bangun setup Anda seiring waktu
</h3>

Anda tidak perlu mengonfigurasi semuanya di awal. Setiap fitur memiliki pemicu yang dapat dikenali, dan sebagian besar tim menambahkannya dalam urutan yang kira-kira seperti ini:

| Pemicu                                                                                          | Tambahkan                                                                                   |
| :---------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------ |
| Claude mendapatkan konvensi atau perintah yang salah dua kali                                   | Tambahkan ke [CLAUDE.md](/docs/id/memory)                                                        |
| Anda terus mengetik prompt yang sama untuk memulai tugas                                        | Simpan sebagai [skill](/docs/id/skills) yang dapat dipanggil pengguna                            |
| Anda menempel playbook yang sama atau prosedur multi-langkah ke chat untuk ketiga kalinya       | Tangkap sebagai [skill](/docs/id/skills)                                                         |
| Anda terus menyalin data dari tab browser yang Claude tidak bisa lihat                          | Hubungkan sistem itu sebagai [server MCP](/docs/id/mcp)                                          |
| Claude membaca banyak file untuk menemukan di mana simbol didefinisikan atau digunakan          | Instal [plugin code intelligence](/docs/id/discover-plugins#code-intelligence) untuk bahasa Anda |
| Tugas sampingan membanjiri percakapan Anda dengan output yang tidak akan Anda referensikan lagi | Arahkan melalui [subagent](/docs/id/sub-agents)                                                  |
| Anda ingin sesuatu terjadi setiap kali tanpa bertanya                                           | Tulis [hook](/docs/id/hooks-guide)                                                               |
| Repositori kedua membutuhkan setup yang sama                                                    | Paket sebagai [plugin](/docs/id/plugins)                                                         |

Pemicu yang sama memberi tahu Anda kapan harus memperbarui apa yang sudah Anda miliki. Kesalahan berulang atau komentar tinjauan berulang adalah edit CLAUDE.md, bukan koreksi satu kali dalam chat. Alur kerja yang terus Anda sesuaikan dengan tangan adalah skill yang memerlukan revisi lain.

<h3 id="compare-similar-features">
  Bandingkan fitur serupa
</h3>

Beberapa fitur dapat terlihat serupa. Untuk panduan yang lebih mendalam tentang memilih di antara mereka, lihat [Steering Claude Code: when to use CLAUDE.md, skills, hooks, and subagents](https://claude.com/blog/steering-claude-code-skills-hooks-rules-subagents-and-more) di blog. Berikut cara membedakannya.

<Tabs>
  <Tab title="Skill vs Subagent">
    Skills dan subagents menyelesaikan masalah yang berbeda:

    * **Skills** adalah konten yang dapat digunakan kembali yang dapat Anda muat ke konteks apa pun
    * **Subagents** adalah pekerja terisolasi yang berjalan terpisah dari percakapan utama Anda

    | Aspek                                           | Skill                                                                | Subagent                                                                         |
    | ----------------------------------------------- | -------------------------------------------------------------------- | -------------------------------------------------------------------------------- |
    | **Apa itu**                                     | Instruksi, pengetahuan, atau alur kerja yang dapat digunakan kembali | Pekerja terisolasi dengan konteksnya sendiri                                     |
    | **Manfaat utama**                               | Bagikan konten di seluruh konteks                                    | Isolasi konteks. Pekerjaan terjadi secara terpisah, hanya ringkasan yang kembali |
    | **Dampak [context window](/docs/id/context-window)** | Menambah jendela utama Anda                                          | Menggunakan jendela terpisah dengan token input dan output sendiri               |
    | **Terbaik untuk**                               | Materi referensi, alur kerja yang dapat dipanggil                    | Tugas yang membaca banyak file, pekerjaan paralel, pekerja khusus                |

    **Skills dapat berupa referensi atau tindakan.** Skills referensi memberikan pengetahuan yang Claude gunakan sepanjang sesi Anda (seperti panduan gaya API Anda). Skills tindakan memberi tahu Claude untuk melakukan sesuatu yang spesifik (seperti `/deploy` yang menjalankan alur kerja deployment Anda).

    **Gunakan subagent** ketika Anda membutuhkan isolasi konteks atau ketika jendela konteks Anda penuh. Subagent mungkin membaca puluhan file atau menjalankan pencarian ekstensif, tetapi percakapan utama Anda hanya menerima ringkasan. Karena pekerjaan subagent tidak mengonsumsi konteks utama Anda, ini juga berguna ketika Anda tidak memerlukan pekerjaan perantara untuk tetap terlihat. Subagents kustom dapat memiliki instruksi mereka sendiri dan dapat memuat skills sebelumnya.

    **Mereka dapat digabungkan.** Subagent dapat memuat skills tertentu sebelumnya (field `skills:`). Skill dapat berjalan dalam konteks terisolasi menggunakan `context: fork`. Lihat [Skills](/docs/id/skills) untuk detail.
  </Tab>

  <Tab title="CLAUDE.md vs Skill">
    Keduanya menyimpan instruksi, tetapi mereka dimuat secara berbeda dan melayani tujuan yang berbeda.

    | Aspek                       | CLAUDE.md                    | Skill                                             |
    | --------------------------- | ---------------------------- | ------------------------------------------------- |
    | **Dimuat**                  | Setiap sesi, secara otomatis | On demand                                         |
    | **Dapat menyertakan file**  | Ya, dengan impor `@path`     | Ya, dengan impor `@path`                          |
    | **Dapat memicu alur kerja** | Tidak                        | Ya, dengan `/<name>`                              |
    | **Terbaik untuk**           | Aturan "selalu lakukan X"    | Materi referensi, alur kerja yang dapat dipanggil |

    **Letakkan di CLAUDE.md** jika Claude harus selalu mengetahuinya: konvensi pengkodean, perintah build, struktur proyek, aturan "jangan pernah lakukan X".

    **Letakkan di skill** jika itu materi referensi yang Claude butuhkan kadang-kadang (dokumen API, panduan gaya) atau alur kerja yang Anda picu dengan `/<name>` (deploy, review, release).

    **Aturan praktis:** Jaga CLAUDE.md di bawah 200 baris. Jika berkembang, pindahkan konten referensi ke skills atau pisahkan ke file [`.claude/rules/`](/docs/id/memory#organize-rules-with-claude%2Frules%2F).
  </Tab>

  <Tab title="CLAUDE.md vs Rules vs Skills">
    Ketiganya menyimpan instruksi, tetapi mereka dimuat secara berbeda:

    | Aspek             | CLAUDE.md                        | `.claude/rules/`                                | Skill                                           |
    | ----------------- | -------------------------------- | ----------------------------------------------- | ----------------------------------------------- |
    | **Dimuat**        | Setiap sesi                      | Setiap sesi, atau ketika file yang cocok dibuka | On demand, ketika dipanggil atau relevan        |
    | **Cakupan**       | Seluruh proyek                   | Dapat dibatasi ke jalur file                    | Spesifik tugas                                  |
    | **Terbaik untuk** | Konvensi inti dan perintah build | Panduan spesifik bahasa atau direktori          | Materi referensi, alur kerja yang dapat diulang |

    **Gunakan CLAUDE.md** untuk instruksi yang setiap sesi butuhkan: perintah build, konvensi tes, arsitektur proyek.

    **Gunakan rules** untuk menjaga CLAUDE.md tetap fokus. Rules dengan [`paths` frontmatter](/docs/id/memory#path-specific-rules) hanya dimuat ketika Claude bekerja dengan file yang cocok, menghemat konteks.

    **Gunakan skills** untuk konten yang Claude hanya butuhkan kadang-kadang, seperti dokumentasi API atau daftar periksa deployment yang Anda picu dengan `/<name>`.
  </Tab>

  <Tab title="Subagent vs Agent team">
    Keduanya melakukan paralelisasi pekerjaan, tetapi mereka secara arsitektur berbeda:

    * **Subagents** berjalan di dalam sesi Anda dan melaporkan hasil kembali ke konteks utama Anda
    * **Agent teams** adalah sesi Claude Code independen yang berkomunikasi satu sama lain

    | Aspek             | Subagent                                               | Agent team                                                      |
    | ----------------- | ------------------------------------------------------ | --------------------------------------------------------------- |
    | **Konteks**       | Jendela konteks sendiri; hasil kembali ke pemanggil    | Jendela konteks sendiri; sepenuhnya independen                  |
    | **Komunikasi**    | Melaporkan hasil kembali ke agen utama saja            | Rekan kerja saling mengirim pesan secara langsung               |
    | **Koordinasi**    | Agen utama mengelola semua pekerjaan                   | Daftar tugas bersama dengan koordinasi diri                     |
    | **Terbaik untuk** | Tugas terfokus di mana hanya hasil yang penting        | Pekerjaan kompleks yang memerlukan diskusi dan kolaborasi       |
    | **Biaya token**   | Lebih rendah: hasil diringkas kembali ke konteks utama | Lebih tinggi: setiap rekan kerja adalah instans Claude terpisah |

    **Gunakan subagent** ketika Anda membutuhkan pekerja cepat dan terfokus: teliti pertanyaan, verifikasi klaim, tinjau file. Subagent melakukan pekerjaan dan mengembalikan ringkasan. Percakapan utama Anda tetap bersih.

    **Gunakan agent team** ketika rekan kerja perlu berbagi temuan, menantang satu sama lain, dan berkoordinasi secara independen. Agent teams terbaik untuk penelitian dengan hipotesis bersaing, tinjauan kode paralel, dan pengembangan fitur baru di mana setiap rekan kerja memiliki bagian terpisah.

    **Titik transisi:** Jika Anda menjalankan subagents paralel tetapi mencapai batas konteks, atau jika subagents Anda perlu berkomunikasi satu sama lain, agent teams adalah langkah alami berikutnya.

    <Note>
      Agent teams bersifat eksperimental dan dinonaktifkan secara default. Lihat [agent teams](/docs/id/agent-teams) untuk setup dan batasan saat ini.
    </Note>
  </Tab>

  <Tab title="MCP vs Skill">
    MCP menghubungkan Claude ke layanan eksternal. Skills memperluas apa yang Claude ketahui, termasuk cara menggunakan layanan tersebut secara efektif.

    | Aspek           | MCP                                              | Skill                                                             |
    | --------------- | ------------------------------------------------ | ----------------------------------------------------------------- |
    | **Apa itu**     | Protokol untuk terhubung ke layanan eksternal    | Pengetahuan, alur kerja, dan materi referensi                     |
    | **Menyediakan** | Akses alat dan data                              | Pengetahuan, alur kerja, materi referensi                         |
    | **Contoh**      | Integrasi Slack, kueri database, kontrol browser | Daftar periksa tinjauan kode, alur kerja deploy, panduan gaya API |

    Ini menyelesaikan masalah yang berbeda dan bekerja dengan baik bersama:

    **MCP** memberi Claude alat yang dirancang khusus untuk sistem eksternal, dengan koneksi dan autentikasi ditangani oleh server.

    **Skills** memberi Claude pengetahuan tentang cara menggunakan alat tersebut secara efektif, ditambah alur kerja yang dapat Anda picu dengan `/<name>`. Skill mungkin menyertakan skema database tim Anda dan pola kueri, atau alur kerja `/post-to-slack` dengan aturan pemformatan pesan tim Anda.

    Contoh: Server MCP menghubungkan Claude ke database Anda. Skill mengajarkan Claude model data Anda, pola kueri umum, dan tabel mana yang digunakan untuk tugas berbeda.
  </Tab>

  <Tab title="Hook vs Skill">
    Hook berjalan pada acara siklus hidup; skill dimuat ke dalam konteks untuk Claude terapkan.

    | Aspek             | Hook                                                                                  | Skill                                                                        |
    | ----------------- | ------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------- |
    | **Berjalan**      | Perintah shell, permintaan HTTP, prompt LLM, atau subagent                            | Instruksi yang Claude baca dan ikuti                                         |
    | **Dipicu oleh**   | [Acara siklus hidup](/docs/id/hooks#hook-events) seperti `PostToolUse` atau `SessionStart` | Anda mengetik `/<name>`, atau Claude mencocokkan deskripsi dengan tugas Anda |
    | **Determinisme**  | Selalu berjalan pada acaranya; pemicu dijamin                                         | Claude menginterpretasikan instruksi; hasil dapat bervariasi                 |
    | **Biaya konteks** | Nol kecuali hook mengembalikan output                                                 | Deskripsi dimuat setiap sesi; konten penuh dimuat saat digunakan             |
    | **Terbaik untuk** | Linting setelah edit, memblokir perintah yang tidak aman, logging, notifikasi         | Alur kerja yang memerlukan penalaran, materi referensi, tugas multi-langkah  |

    **Gunakan hook** ketika tindakan harus terjadi dengan cara yang sama setiap kali dan tidak perlu Claude berpikir. Misalnya: format saat simpan, tolak `rm -rf /`, posting pesan Slack ketika sesi berakhir.

    **Gunakan skill** ketika Claude harus memutuskan cara menerapkan langkah-langkahnya, atau ketika kontennya adalah pengetahuan daripada skrip. Misalnya: daftar periksa `/release`, panduan gaya API Anda, playbook debugging.

    **Letakkan guardrails di hooks.** Instruksi seperti "jangan pernah edit `.env`" di CLAUDE.md atau skill adalah permintaan, bukan jaminan. Hook `PreToolUse` yang memblokir edit adalah penegakan. Jika aturan harus berlaku setiap kali, buat hook daripada instruksi prompt.

    **Output hook mendarat di konteks.** Hook `PostToolUse` yang menjalankan linter Anda memberi umpan balik hasil sebagai teks yang Claude baca; skill `/fix-lint` memberi tahu Claude cara menyelesaikannya.
  </Tab>
</Tabs>

<h3 id="understand-how-features-layer">
  Pahami bagaimana fitur berlapis
</h3>

Fitur dapat didefinisikan di beberapa tingkat: seluruh pengguna, per-proyek, melalui plugins, atau melalui kebijakan terkelola. Anda juga dapat menyarangkan file CLAUDE.md di subdirektori atau menempatkan skills di paket tertentu dari monorepo. Ketika fitur yang sama ada di beberapa tingkat, berikut cara mereka berlapis:

* **File CLAUDE.md** bersifat aditif: semua tingkat berkontribusi konten ke konteks Claude secara bersamaan. File dari direktori kerja Anda dan di atas dimuat saat peluncuran; subdirektori dimuat saat Anda bekerja di dalamnya. Ketika instruksi bertentangan, Claude menggunakan penilaian untuk merekonsiliasi mereka, dengan instruksi yang lebih spesifik biasanya mengambil alih. Lihat [bagaimana file CLAUDE.md dimuat](/docs/id/memory#how-claude-md-files-load).
* **Skills dan subagents** menimpa berdasarkan nama: ketika nama yang sama ada di beberapa tingkat, satu definisi menang berdasarkan prioritas (terkelola > pengguna > proyek untuk skills; terkelola > bendera CLI > proyek > pengguna > plugin untuk subagents). Plugin skills adalah [namespaced](/docs/id/plugins#add-skills-to-your-plugin) untuk menghindari konflik. Lihat [penemuan skill](/docs/id/skills#where-skills-live) dan [cakupan subagent](/docs/id/sub-agents#choose-the-subagent-scope).
* **Server MCP** menimpa berdasarkan nama: lokal > proyek > pengguna. Lihat [cakupan MCP](/docs/id/mcp#scope-hierarchy-and-precedence).
* **Hooks** bergabung: semua hook terdaftar berjalan untuk acara pencocokan mereka terlepas dari sumber. Lihat [hooks](/docs/id/hooks-guide).

<h3 id="combine-features">
  Gabungkan fitur
</h3>

Setiap ekstensi menyelesaikan masalah yang berbeda: CLAUDE.md menangani konteks yang selalu aktif, skills menangani pengetahuan dan alur kerja on-demand, MCP menangani koneksi eksternal, subagents menangani isolasi, dan hooks menangani otomasi. Setup nyata menggabungkan mereka berdasarkan alur kerja Anda.

Misalnya, Anda mungkin menggunakan CLAUDE.md untuk konvensi proyek, skill untuk alur kerja deployment Anda, MCP untuk terhubung ke database Anda, dan hook untuk menjalankan linting setelah setiap edit. Setiap fitur menangani apa yang terbaik.

| Pola                   | Cara kerjanya                                                                                         | Contoh                                                                                              |
| ---------------------- | ----------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------- |
| **Skill + MCP**        | MCP menyediakan koneksi; skill mengajarkan Claude cara menggunakannya dengan baik                     | MCP terhubung ke database Anda, skill mendokumentasikan skema dan pola kueri Anda                   |
| **Skill + Subagent**   | Skill menspawn subagents untuk pekerjaan paralel                                                      | Skill `/audit` memulai subagents keamanan, performa, dan gaya yang bekerja dalam konteks terisolasi |
| **CLAUDE.md + Skills** | CLAUDE.md menyimpan aturan yang selalu aktif; skills menyimpan materi referensi yang dimuat on-demand | CLAUDE.md mengatakan "ikuti konvensi API kami," skill berisi panduan gaya API lengkap               |
| **Hook + MCP**         | Hook memicu tindakan eksternal melalui MCP                                                            | Hook pasca-edit mengirim notifikasi Slack ketika Claude memodifikasi file kritis                    |

<h2 id="understand-context-costs">
  Pahami biaya konteks
</h2>

Setiap fitur yang Anda tambahkan mengonsumsi beberapa konteks Claude. Terlalu banyak dapat mengisi jendela konteks Anda, tetapi juga dapat menambah kebisingan yang membuat Claude kurang efektif; skills mungkin tidak dipicu dengan benar, atau Claude mungkin kehilangan jejak konvensi Anda. Memahami trade-off ini membantu Anda membangun setup yang efektif. Untuk tampilan interaktif tentang bagaimana fitur-fitur ini bergabung dalam sesi yang berjalan, lihat [Jelajahi context window](/docs/id/context-window).

<h3 id="context-cost-by-feature">
  Biaya konteks berdasarkan fitur
</h3>

Setiap fitur memiliki strategi pemuatan dan biaya konteks yang berbeda:

| Fitur                 | Kapan dimuat                          | Apa yang dimuat                                             | Biaya konteks                                    |
| --------------------- | ------------------------------------- | ----------------------------------------------------------- | ------------------------------------------------ |
| **CLAUDE.md**         | Awal sesi                             | Konten penuh                                                | Setiap permintaan                                |
| **Skills**            | Awal sesi + ketika digunakan          | Deskripsi di awal, konten penuh ketika digunakan            | Rendah (deskripsi setiap permintaan)\*           |
| **Server MCP**        | Awal sesi                             | Nama alat; skema penuh on demand                            | Rendah sampai alat digunakan                     |
| **Code intelligence** | Setelah pengeditan file dan on demand | Diagnostik setelah pengeditan; lokasi simbol saat pencarian | Rendah; mengurangi pembacaan file di tempat lain |
| **Subagents**         | Ketika dispawn                        | Konteks segar dengan skills yang ditentukan                 | Terisolasi dari sesi utama                       |
| **Hooks**             | Saat dipicu                           | Tidak ada (berjalan secara eksternal)                       | Nol, kecuali hook mengembalikan konteks tambahan |

\*Secara default, deskripsi skill dimuat saat awal sesi sehingga Claude dapat memutuskan kapan menggunakannya. Atur `disable-model-invocation: true` di frontmatter skill untuk menyembunyikannya dari Claude sepenuhnya sampai Anda memanggilnya secara manual. Ini mengurangi biaya konteks menjadi nol untuk skills yang hanya Anda picu sendiri. Untuk skill yang tidak Anda tulis, atur [`skillOverrides`](/docs/id/skills#override-skill-visibility-from-settings) di settings untuk melakukan hal yang sama tanpa mengedit filenya.

<h3 id="understand-how-features-load">
  Pahami bagaimana fitur dimuat
</h3>

Setiap fitur dimuat pada titik berbeda dalam sesi Anda. Tab di bawah menjelaskan kapan masing-masing dimuat dan apa yang masuk ke konteks.

<img src="https://mintcdn.com/claude-code/ikqp3_70mqIahteV/images/context-loading.svg?fit=max&auto=format&n=ikqp3_70mqIahteV&q=85&s=aab139e750494a237ae2e0c8f9139b0a" alt="Pemuatan konteks: CLAUDE.md dimuat saat awal sesi dan tetap di setiap permintaan. Nama alat MCP dimuat saat awal dengan skema penuh ditunda sampai digunakan. Skills memuat deskripsi saat awal, konten penuh saat invokasi. Subagents mendapat konteks terisolasi. Hooks berjalan secara eksternal." width="720" height="382" data-path="images/context-loading.svg" />

<Tabs>
  <Tab title="CLAUDE.md">
    **Kapan:** Awal sesi

    **Apa yang dimuat:** Konten penuh semua file CLAUDE.md (tingkat terkelola, pengguna, dan proyek).

    **Warisan:** Claude membaca file CLAUDE.md dari direktori kerja Anda hingga ke root, dan menemukan yang tersarang di subdirektori saat mengakses file tersebut. Lihat [Bagaimana file CLAUDE.md dimuat](/docs/id/memory#how-claude-md-files-load) untuk detail.

    <Tip>Jaga CLAUDE.md di bawah 200 baris. Pindahkan materi referensi ke skills, yang dimuat on-demand.</Tip>
  </Tab>

  <Tab title="Skills">
    Skills adalah kemampuan tambahan dalam toolkit Claude. Mereka dapat berupa materi referensi (seperti panduan gaya API) atau alur kerja yang dapat dipanggil yang Anda picu dengan `/<name>` (seperti `/deploy`). Claude Code dilengkapi dengan [skills bundel](/docs/id/commands) seperti `/code-review`, `/batch`, dan `/debug` yang bekerja langsung. Anda juga dapat membuat yang Anda sendiri. Claude menggunakan skills ketika sesuai, atau Anda dapat memanggil satu secara langsung.

    **Kapan:** Tergantung pada konfigurasi skill. Secara default, deskripsi dimuat saat awal sesi dan konten penuh dimuat ketika digunakan. Untuk skills hanya pengguna (`disable-model-invocation: true`), tidak ada yang dimuat sampai Anda memanggilnya.

    **Apa yang dimuat:** Untuk skills yang dapat dipanggil model, Claude melihat nama dan deskripsi di setiap permintaan. Ketika Anda memanggil skill dengan `/<name>` atau Claude memuatnya secara otomatis, konten penuh dimuat ke percakapan Anda.

    **Bagaimana Claude memilih skills:** Claude mencocokkan tugas Anda terhadap deskripsi skill untuk memutuskan mana yang relevan. Jika deskripsi samar atau tumpang tindih, Claude mungkin memuat skill yang salah atau melewatkan yang akan membantu. Untuk memberi tahu Claude menggunakan skill tertentu, panggilnya dengan `/<name>`. Skills dengan `disable-model-invocation: true` tidak terlihat oleh Claude sampai Anda memanggilnya.

    **Biaya konteks:** Rendah sampai digunakan. Skills hanya pengguna memiliki biaya nol sampai dipanggil.

    **Di subagents:** Skills bekerja berbeda di subagents. Alih-alih pemuatan on-demand, skills yang tercantum di field `skills:` subagent sepenuhnya dimuat sebelumnya ke konteksnya saat peluncuran. Subagents masih dapat menemukan dan memanggil skills proyek, pengguna, dan plugin yang tidak tercantum melalui alat Skill.

    <Tip>Gunakan `disable-model-invocation: true` untuk skills dengan efek samping. Ini menghemat konteks dan memastikan hanya Anda yang memicunya.</Tip>
  </Tab>

  <Tab title="Server MCP">
    **Kapan:** Awal sesi.

    **Apa yang dimuat:** Nama alat dari server yang terhubung. Skema JSON penuh tetap ditunda sampai Claude memerlukan alat tertentu.

    **Biaya konteks:** [Pencarian alat](/docs/id/mcp#scale-with-mcp-tool-search) diaktifkan secara default, jadi alat MCP idle mengonsumsi konteks minimal.

    <Tip>Jalankan `/mcp` untuk melihat status koneksi dan biaya token per server. Claude Code [terhubung kembali ke server jarak jauh secara otomatis](/docs/id/mcp#automatic-reconnection) jika mereka terputus, dan Anda dapat memutuskan server yang tidak Anda gunakan secara aktif.</Tip>
  </Tab>

  <Tab title="Code intelligence">
    **Kapan:** Setelah pengeditan file, dan on demand ketika Claude menavigasi kode.

    **Apa yang dimuat:** Kesalahan tipe dan peringatan setelah setiap pengeditan file. Informasi definisi, referensi, dan tipe ketika Claude mencari simbol.

    **Biaya konteks:** Rendah. Pencarian simbol sering menggantikan pembacaan file yang luas, jadi penggunaan konteks bersih dapat turun.

    <Tip>Alat LSP tidak aktif sampai Anda memasang [plugin code intelligence](/docs/id/discover-plugins#code-intelligence) untuk bahasa Anda.</Tip>
  </Tab>

  <Tab title="Subagents">
    **Kapan:** On demand, ketika Anda atau Claude menspawn satu untuk tugas.

    **Apa yang dimuat:** Konteks segar dan terisolasi yang berisi:

    * Prompt sistem agen, bukan prompt sistem Claude Code lengkap
    * Konten penuh skills yang tercantum di field `skills:` agen
    * CLAUDE.md dan status git, kecuali agen Explore dan Plan bawaan [menghilangkan keduanya](/docs/id/sub-agents#what-loads-at-startup)
    * Apa pun konteks yang agen utama lewatkan dalam prompt

    **Biaya konteks:** Terisolasi dari sesi utama. Subagents tidak mewarisi riwayat percakapan Anda atau skills yang dipanggil.

    <Tip>Gunakan subagents untuk pekerjaan yang tidak memerlukan konteks percakapan penuh Anda. Isolasi mereka mencegah mengembang sesi utama Anda.</Tip>
  </Tab>

  <Tab title="Hooks">
    **Kapan:** Saat dipicu. Hooks berjalan pada acara siklus hidup tertentu seperti eksekusi alat, batas sesi, pengajuan prompt, permintaan izin, dan pemadatan. Lihat [Hooks](/docs/id/hooks) untuk daftar lengkap.

    **Apa yang dimuat:** Tidak ada secara default. Hooks berjalan di luar percakapan utama.

    **Biaya konteks:** Nol, kecuali hook mengembalikan output yang ditambahkan sebagai pesan ke percakapan Anda.

    <Tip>Hooks ideal untuk efek samping (linting, logging) yang tidak perlu mempengaruhi konteks Claude.</Tip>
  </Tab>
</Tabs>

<h2 id="learn-more">
  Pelajari lebih lanjut
</h2>

Setiap fitur memiliki panduan sendiri dengan instruksi setup, contoh, dan opsi konfigurasi.

<CardGroup cols={2}>
  <Card title="CLAUDE.md" icon="file-lines" href="/docs/id/memory">
    Simpan konteks proyek, konvensi, dan instruksi
  </Card>

  <Card title="Skills" icon="brain" href="/docs/id/skills">
    Berikan Claude keahlian domain dan alur kerja yang dapat digunakan kembali
  </Card>

  <Card title="Subagents" icon="users" href="/docs/id/sub-agents">
    Alihkan pekerjaan ke konteks terisolasi
  </Card>

  <Card title="Agent teams" icon="network" href="/docs/id/agent-teams">
    Koordinasikan beberapa sesi yang bekerja secara paralel
  </Card>

  <Card title="MCP" icon="plug" href="/docs/id/mcp">
    Hubungkan Claude ke layanan eksternal
  </Card>

  <Card title="Hooks" icon="bolt" href="/docs/id/hooks-guide">
    Otomatisasi alur kerja dengan hooks
  </Card>

  <Card title="Plugins" icon="puzzle-piece" href="/docs/id/plugins">
    Bundel dan bagikan set fitur
  </Card>

  <Card title="Marketplaces" icon="store" href="/docs/id/plugin-marketplaces">
    Host dan distribusikan koleksi plugin
  </Card>
</CardGroup>
