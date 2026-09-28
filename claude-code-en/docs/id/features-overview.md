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
* **[Output styles](/docs/id/output-styles)** menetapkan peran, nada, dan format respons Claude untuk setiap respons dalam sesi
* **[Skills](/docs/id/skills)** menambahkan pengetahuan yang dapat digunakan kembali dan alur kerja yang dapat dipanggil
* **[Code intelligence](/docs/id/tools-reference#lsp-tool-behavior)** menghubungkan Claude ke language server untuk navigasi tingkat simbol dan kesalahan tipe langsung
* **[MCP](/docs/id/mcp)** menghubungkan Claude ke layanan dan alat eksternal
* **[Subagents](/docs/id/sub-agents)** menjalankan loop mereka sendiri dalam konteks terisolasi, mengembalikan ringkasan
* **[Dynamic workflows](/docs/id/workflows)** menjalankan banyak subagents dari skrip yang Claude tulis, mengembalikan satu hasil
* **[Cross-session messaging](/docs/id/cross-session-messaging)** memungkinkan Claude melewatkan pesan dari salah satu sesi Anda ke sesi lain
* **[Hooks](/docs/id/hooks-guide)** menjalankan skrip, permintaan HTTP, panggilan alat MCP, prompt, atau subagent Anda ketika Claude Code mencapai acara siklus hidup
* **[Plugins](/docs/id/plugins/overview)** dan **[marketplaces](/docs/id/plugins/overview)** mengemas dan mendistribusikan fitur-fitur ini

[Skills](/docs/id/skills) adalah ekstensi paling fleksibel. Skill adalah file markdown yang berisi pengetahuan, alur kerja, atau instruksi. Anda dapat memanggil skills dengan perintah seperti `/deploy`, atau Claude dapat memuatnya secara otomatis ketika relevan. Skills dapat berjalan dalam percakapan Anda saat ini atau dalam konteks terisolasi melalui subagents.

<h2 id="match-features-to-your-goal">
  Sesuaikan fitur dengan tujuan Anda
</h2>

Fitur berkisar dari konteks yang selalu aktif yang Claude lihat setiap sesi, hingga kemampuan on-demand yang dapat Anda atau Claude panggil, hingga otomasi latar belakang yang berjalan pada acara tertentu. Tabel di bawah menunjukkan apa yang tersedia dan kapan masing-masing masuk akal.

| Fitur                                                          | Apa yang dilakukannya                                                                    | Kapan menggunakannya                                                                                                                  | Contoh                                                                                                                          |
| -------------------------------------------------------------- | ---------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------- |
| **CLAUDE.md**                                                  | Konteks persisten dimuat setiap percakapan                                               | Konvensi proyek, aturan "selalu lakukan X"                                                                                            | "Gunakan pnpm, bukan npm. Jalankan tes sebelum melakukan commit."                                                               |
| **[Output style](/docs/id/output-styles)**                          | Instruksi yang menetapkan peran, nada, dan format respons Claude untuk seluruh sesi      | Suara, panjang, atau format yang Anda inginkan di setiap respons, atau Claude bekerja sebagai sesuatu selain insinyur perangkat lunak | Gaya Concise bawaan untuk respons yang lebih pendek; gaya kustom yang menjawab setiap pertanyaan dengan diagram terlebih dahulu |
| **Skill**                                                      | Instruksi, pengetahuan, dan alur kerja yang dapat digunakan Claude                       | Konten yang dapat digunakan kembali, dokumen referensi, tugas yang dapat diulang                                                      | `/deploy` menjalankan daftar periksa penyebaran Anda; skill dokumen API dengan pola endpoint                                    |
| **Subagent**                                                   | Konteks eksekusi terisolasi yang mengembalikan hasil ringkasan                           | Isolasi konteks, tugas paralel, pekerja khusus                                                                                        | Tugas penelitian yang membaca banyak file tetapi hanya mengembalikan temuan kunci                                               |
| **[Dynamic workflow](/docs/id/workflows)**                          | Skrip yang ditulis Claude yang menjalankan banyak subagent di latar belakang             | Pekerjaan yang melampaui segelintir subagent, atau temuan yang ingin Anda verifikasi silang                                           | Audit seluruh basis kode, dengan set agen kedua memverifikasi setiap temuan                                                     |
| **[Cross-session messaging](/docs/id/cross-session-messaging)**     | Claude mengirimkan pesan dari salah satu sesi Anda ke sesi lain                          | Sesi yang Anda jalankan sendiri yang membutuhkan temuan satu sama lain di tengah tugas                                                | Satu sesi memperingatkan sesi lain bahwa perubahan yang dilakukannya merusak apa yang sedang dibangun oleh sesi lain            |
| **[Code intelligence](/docs/id/tools-reference#lsp-tool-behavior)** | Navigasi language-server dan diagnostik                                                  | Bahasa yang diketik, basis kode besar di mana grep lambat atau tidak presisi                                                          | Lompat ke definisi simbol alih-alih membaca seluruh file                                                                        |
| **MCP**                                                        | Terhubung ke layanan eksternal                                                           | Data atau tindakan eksternal                                                                                                          | Kueri basis data Anda, posting ke Slack, kontrol browser                                                                        |
| **Hook**                                                       | Skrip, permintaan HTTP, panggilan alat MCP, prompt, atau subagent yang dipicu oleh acara | Otomasi yang harus berjalan pada setiap acara yang cocok                                                                              | Jalankan ESLint setelah setiap pengeditan file                                                                                  |
| **[Artifact](/docs/id/artifacts)**                                  | Publikasikan output sesi sebagai halaman web pribadi yang interaktif                     | Output yang ingin Anda lihat atau bagikan secara visual daripada sebagai teks terminal                                                | Garis waktu insiden yang diperbarui saat Claude menyelidiki                                                                     |

**[Plugins](/docs/id/plugins/overview)** adalah lapisan pengemasan. Plugin menggabungkan skill, hook, subagent, dan server MCP ke dalam satu unit yang dapat diinstal. Skill plugin memiliki namespace (seperti `/my-plugin:review`) sehingga beberapa plugin dapat hidup berdampingan. Gunakan plugin ketika Anda ingin menggunakan kembali pengaturan yang sama di beberapa repositori atau mendistribusikan ke orang lain melalui **[marketplace](/docs/id/plugins/overview)**.

<h3 id="build-your-setup-over-time">
  Bangun pengaturan Anda seiring waktu
</h3>

Anda tidak perlu mengonfigurasi semuanya di muka. Setiap fitur memiliki pemicu yang dapat dikenali, dan sebagian besar tim menambahkannya dalam urutan yang kurang lebih seperti ini:

| Pemicu                                                                                                       | Tambahkan                                                                          |
| :----------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------- |
| Claude mendapatkan konvensi atau perintah yang salah dua kali                                                | Tambahkan ke [CLAUDE.md](/docs/id/memory)                                               |
| Anda terus meminta Claude untuk lebih pendek, menjelaskan lebih banyak, atau menjawab dalam format yang sama | Atur [output style](/docs/id/output-styles)                                             |
| Anda terus mengetik prompt yang sama untuk memulai tugas                                                     | Simpan sebagai [skill](/docs/id/skills) yang dapat dipanggil pengguna                   |
| Anda menempel playbook yang sama atau prosedur multi-langkah ke dalam chat untuk ketiga kalinya              | Tangkap sebagai [skill](/docs/id/skills)                                                |
| Anda terus menyalin data dari tab browser yang Claude tidak bisa lihat                                       | Hubungkan sistem itu sebagai [server MCP](/docs/id/mcp)                                 |
| Claude membaca banyak file untuk menemukan di mana simbol didefinisikan atau digunakan                       | Instal [plugin code intelligence](/docs/id/plugins/code-intelligence) untuk bahasa Anda |
| Tugas sampingan membanjiri percakapan Anda dengan output yang tidak akan Anda referensikan lagi              | Arahkan melalui [subagent](/docs/id/sub-agents)                                         |
| Anda ingin sesuatu terjadi setiap kali tanpa bertanya                                                        | Tulis [hook](/docs/id/hooks-guide)                                                      |
| Repositori kedua membutuhkan pengaturan yang sama                                                            | Paket sebagai [plugin](/docs/id/plugins/overview)                                       |

Pemicu yang sama memberi tahu Anda kapan harus memperbarui apa yang sudah Anda miliki. Kesalahan berulang atau komentar tinjauan berulang adalah pengeditan CLAUDE.md, bukan koreksi sekali jadi dalam chat. Alur kerja yang terus Anda sesuaikan dengan tangan adalah skill yang membutuhkan revisi lain.

<h3 id="compare-similar-features">
  Bandingkan fitur serupa
</h3>

Beberapa fitur dapat terlihat serupa. Untuk panduan yang lebih mendalam tentang memilih di antara mereka, lihat [Steering Claude Code: when to use CLAUDE.md, skills, hooks, and subagents](https://claude.com/blog/steering-claude-code-skills-hooks-rules-subagents-and-more) di blog. Berikut cara membedakannya.

<Tabs>
  <Tab title="Skill vs Subagent">
    Skill dan subagent menyelesaikan masalah yang berbeda:

    * **Skills** adalah konten yang dapat digunakan kembali yang dapat Anda muat ke dalam konteks apa pun
    * **Subagents** adalah pekerja terisolasi yang berjalan terpisah dari percakapan utama Anda

    | Aspek                                           | Skill                                                                | Subagent                                                                         |
    | ----------------------------------------------- | -------------------------------------------------------------------- | -------------------------------------------------------------------------------- |
    | **Apa itu**                                     | Instruksi, pengetahuan, atau alur kerja yang dapat digunakan kembali | Pekerja terisolasi dengan konteksnya sendiri                                     |
    | **Manfaat utama**                               | Bagikan konten di seluruh konteks                                    | Isolasi konteks. Pekerjaan terjadi secara terpisah, hanya ringkasan yang kembali |
    | **Dampak [context window](/docs/id/context-window)** | Menambah jendela utama Anda                                          | Menggunakan jendela terpisah dengan token input dan output-nya sendiri           |
    | **Terbaik untuk**                               | Materi referensi, alur kerja yang dapat dipanggil                    | Tugas yang membaca banyak file, pekerjaan paralel, pekerja khusus                |

    **Skills dapat berupa referensi atau tindakan.** Skill referensi memberikan pengetahuan yang Claude gunakan di seluruh sesi Anda (seperti panduan gaya API Anda). Skill tindakan memberi tahu Claude untuk melakukan sesuatu yang spesifik (seperti `/deploy` yang menjalankan alur kerja penyebaran Anda).

    **Gunakan subagent** ketika Anda membutuhkan isolasi konteks atau ketika jendela konteks Anda penuh. Subagent mungkin membaca puluhan file atau menjalankan pencarian ekstensif, tetapi percakapan utama Anda hanya menerima ringkasan. Karena pekerjaan subagent tidak mengonsumsi konteks utama Anda, ini juga berguna ketika Anda tidak memerlukan pekerjaan perantara untuk tetap terlihat. Subagent kustom dapat memiliki instruksi mereka sendiri dan dapat memuat skill sebelumnya.

    **Mereka dapat menggabungkan.** Subagent dapat memuat skill tertentu (field `skills:`). Skill dapat berjalan dalam konteks terisolasi menggunakan `context: fork`. Lihat [Skills](/docs/id/skills) untuk detail.
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

    **Aturan praktis:** Jaga CLAUDE.md di bawah 200 baris. Jika berkembang, pindahkan konten referensi ke skill atau pisahkan ke file [`.claude/rules/`](/docs/id/memory#organize-rules-with-claude/rules/).
  </Tab>

  <Tab title="CLAUDE.md vs Output style">
    Keduanya memberikan instruksi berdiri kepada Claude. CLAUDE.md membawa apa yang harus diketahui Claude, dan gaya output menetapkan cara Claude merespons.

    | Aspek             | CLAUDE.md                                                  | Output style                                                                                               |
    | ----------------- | ---------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------- |
    | **Memegang**      | Fakta dan aturan tentang proyek Anda                       | Peran, nada, dan format respons                                                                            |
    | **Beralih**       | Selalu dimuat                                              | Satu aktif pada satu waktu; [beralih gaya](/docs/id/output-styles#change-your-output-style) kapan saja Anda mau |
    | **Terbaik untuk** | Perintah build, konvensi, aturan "jangan pernah lakukan X" | Respons yang lebih pendek, penjelasan bersama kode, peran non-teknik                                       |

    **Letakkan di CLAUDE.md** jika itu benar dari proyek apa pun gaya yang Anda gunakan: konvensi pengkodean, perintah build, struktur proyek.

    **Gunakan output style** jika itu tentang respons itu sendiri dan Anda mungkin ingin mematikannya lagi: panjang, format, berapa banyak Claude menjelaskan, atau peran yang berbeda seperti asisten penulisan. Claude Code mencakup [gaya bawaan](/docs/id/output-styles#built-in-output-styles), dan Anda dapat menulis gaya Anda sendiri.

    **Mereka menggabungkan.** CLAUDE.md tetap dimuat gaya mana pun yang Anda pilih. Claude mengikuti keduanya sebagai instruksi, jadi tidak ada yang ditegakkan. Untuk apa pun yang harus terjadi setiap kali, gunakan [hook](/docs/id/hooks-guide).
  </Tab>

  <Tab title="CLAUDE.md vs Rules vs Skills">
    Ketiga-tiganya menyimpan instruksi, tetapi mereka dimuat secara berbeda:

    | Aspek             | CLAUDE.md                        | `.claude/rules/`                                | Skill                                           |
    | ----------------- | -------------------------------- | ----------------------------------------------- | ----------------------------------------------- |
    | **Dimuat**        | Setiap sesi                      | Setiap sesi, atau ketika file yang cocok dibuka | On demand, ketika dipanggil atau relevan        |
    | **Cakupan**       | Seluruh proyek                   | Dapat dibatasi ke jalur file                    | Spesifik tugas                                  |
    | **Terbaik untuk** | Konvensi inti dan perintah build | Panduan spesifik bahasa atau direktori          | Materi referensi, alur kerja yang dapat diulang |

    **Gunakan CLAUDE.md** untuk instruksi yang setiap sesi butuhkan: perintah build, konvensi tes, arsitektur proyek.

    **Gunakan rules** untuk menjaga CLAUDE.md tetap fokus. Rules dengan [frontmatter `paths`](/docs/id/memory#path-specific-rules) hanya dimuat ketika Claude bekerja dengan file yang cocok, menghemat konteks.

    **Gunakan skills** untuk konten yang Claude hanya butuhkan kadang-kadang, seperti dokumentasi API atau daftar periksa penyebaran yang Anda picu dengan `/<name>`.
  </Tab>

  <Tab title="Subagent vs Dynamic workflow">
    Keduanya melakukan pekerjaan di luar percakapan utama Anda. Dengan subagent, Claude memutuskan giliran demi giliran apa yang berjalan selanjutnya. Dalam alur kerja, skrip memutuskan:

    * **Subagents** adalah pekerja yang Claude spawn, masing-masing mengembalikan ringkasan ke percakapan yang memunculkannya
    * **[Dynamic workflows](/docs/id/workflows)** adalah skrip yang ditulis Claude yang menjalankan banyak subagent di latar belakang dan mengembalikan satu hasil

    **Gunakan subagent** ketika Anda membutuhkan pekerja yang cepat dan terfokus: teliti pertanyaan, verifikasi klaim, tinjau file. Subagent melakukan pekerjaan dan mengembalikan ringkasan, jadi percakapan utama Anda tetap bersih. Subagent yang Claude beri nama ketika memunculkannya juga dapat [saling berkirim pesan](/docs/id/sub-agents#what-loads-at-startup).

    **Gunakan dynamic workflow** ketika pekerjaan [melampaui segelintir subagent](/docs/id/workflows#when-to-use-a-workflow), atau ketika Anda ingin temuan diverifikasi silang sebelum Anda melihatnya, seperti audit berbasis kode, migrasi besar, atau rencana yang disusun dari beberapa sudut. Untuk memulai satu, [minta alur kerja dalam prompt Anda](/docs/id/workflows#ask-for-a-workflow-in-your-prompt).

    **Untuk meneruskan temuan dari salah satu sesi Anda ke sesi lain**, minta Claude sesi pertama untuk mengirimkannya. Claude mengirimkannya dengan [cross-session messaging](/docs/id/cross-session-messaging). [Run agents in parallel](/docs/id/agents) membandingkan cara lain untuk menjalankan lebih dari satu Claude sekaligus, termasuk sesi yang Anda serahkan dan periksa kembali nanti.
  </Tab>

  <Tab title="MCP vs Skill">
    MCP menghubungkan Claude ke layanan eksternal. Skills memperluas apa yang Claude ketahui, termasuk cara menggunakan layanan tersebut secara efektif.

    | Aspek           | MCP                                                | Skill                                                                 |
    | --------------- | -------------------------------------------------- | --------------------------------------------------------------------- |
    | **Apa itu**     | Protokol untuk terhubung ke layanan eksternal      | Pengetahuan, alur kerja, dan materi referensi                         |
    | **Menyediakan** | Alat dan akses data                                | Pengetahuan, alur kerja, materi referensi                             |
    | **Contoh**      | Integrasi Slack, kueri basis data, kontrol browser | Daftar periksa tinjauan kode, alur kerja penyebaran, panduan gaya API |

    Ini menyelesaikan masalah yang berbeda dan bekerja dengan baik bersama:

    **MCP** memberikan Claude alat yang dirancang khusus untuk sistem eksternal, dengan koneksi dan autentikasi ditangani oleh server.

    **Skills** memberikan Claude pengetahuan tentang cara menggunakan alat tersebut secara efektif, ditambah alur kerja yang dapat Anda picu dengan `/<name>`. Skill mungkin menyertakan skema basis data tim Anda dan pola kueri, atau alur kerja `/post-to-slack` dengan aturan pemformatan pesan tim Anda.
  </Tab>

  <Tab title="Hook vs Skill">
    Claude Code menjalankan hook pada acara siklus hidup; itu memuat skill ke dalam konteks untuk Claude terapkan.

    | Aspek             | Hook                                                                                | Skill                                                                        |
    | ----------------- | ----------------------------------------------------------------------------------- | ---------------------------------------------------------------------------- |
    | **Berjalan**      | Perintah shell, permintaan HTTP, panggilan alat MCP, prompt LLM, atau subagent      | Instruksi yang Claude baca dan ikuti                                         |
    | **Dipicu oleh**   | [Acara siklus hidup](/docs/id/hooks-guide) seperti `PostToolUse` atau `SessionStart`     | Anda mengetik `/<name>`, atau Claude mencocokkan deskripsi dengan tugas Anda |
    | **Determinisme**  | Selalu menyala pada acaranya; pemicu dijamin                                        | Claude menginterpretasikan instruksi; hasilnya dapat bervariasi              |
    | **Biaya konteks** | Nol kecuali hook mengembalikan output                                               | Deskripsi dimuat setiap sesi; konten penuh dimuat saat digunakan             |
    | **Terbaik untuk** | Linting setelah pengeditan, memblokir perintah yang tidak aman, logging, notifikasi | Alur kerja yang membutuhkan penalaran, materi referensi, tugas multi-langkah |

    **Gunakan hook** ketika tindakan harus terjadi dengan cara yang sama setiap kali dan tidak memerlukan Claude untuk berpikir. Misalnya: format saat menyimpan, tolak `rm -rf /`, posting pesan Slack ketika sesi berakhir.

    **Gunakan skill** ketika Claude harus memutuskan cara menerapkan langkah-langkah, atau ketika kontennya adalah pengetahuan daripada skrip. Misalnya: daftar periksa `/release`, panduan gaya API Anda, playbook debugging.

    **Letakkan guardrail di hook.** Instruksi seperti "jangan pernah edit `.env`" di CLAUDE.md atau skill adalah permintaan, bukan jaminan. Hook `PreToolUse` yang memblokir pengeditan adalah penegakan. Jika aturan harus berlaku setiap kali, buat hook daripada instruksi prompt.

    **Output hook mendarat di konteks.** Hook `PostToolUse` yang menjalankan linter Anda memberi umpan balik hasil sebagai teks yang Claude baca; skill `/fix-lint` memberi tahu Claude cara menyelesaikannya.
  </Tab>
</Tabs>

<h3 id="understand-how-features-layer">
  Pahami bagaimana fitur berlapis
</h3>

Fitur dapat didefinisikan di berbagai tingkat: seluruh pengguna, per-proyek, melalui plugin, atau melalui kebijakan yang dikelola. Anda juga dapat menyarangkan file CLAUDE.md di subdirektori atau menempatkan skill di paket tertentu dari monorepo. Ketika fitur yang sama ada di berbagai tingkat, berikut cara mereka berlapis:

* **File CLAUDE.md** bersifat aditif: semua tingkat berkontribusi konten ke konteks Claude secara bersamaan. File dari direktori kerja Anda dan di atas dimuat saat peluncuran; subdirektori dimuat saat Anda bekerja di dalamnya. Ketika instruksi bertentangan, Claude menggunakan penilaian untuk merekonsiliasi mereka. Lihat [bagaimana file CLAUDE.md dimuat](/docs/id/memory#how-claude-md-files-load).
* **Skills dan subagents** menimpa berdasarkan nama: ketika nama yang sama ada di berbagai tingkat, satu definisi menang berdasarkan prioritas (managed > user > project untuk skill; managed > CLI flag > project > user > plugin untuk subagent). Skill plugin [memiliki namespace](/docs/id/plugins/components#skills) untuk menghindari konflik. Lihat [penemuan skill](/docs/id/skills#resolve-skills-that-share-a-name) dan [cakupan subagent](/docs/id/sub-agents#choose-the-subagent-scope).
* **Server MCP** menimpa berdasarkan nama: lokal > proyek > pengguna. Lihat [cakupan MCP](/docs/id/mcp#scope-hierarchy-and-precedence).
* **Hooks** menggabung: semua hook terdaftar menyala untuk acara yang cocok mereka terlepas dari sumber. Lihat [hooks](/docs/id/hooks-guide).

<h3 id="combine-features">
  Gabungkan fitur
</h3>

Setiap ekstensi menyelesaikan masalah yang berbeda: CLAUDE.md menangani konteks yang selalu aktif, skill menangani pengetahuan on-demand dan alur kerja, MCP menangani koneksi eksternal, subagent menangani isolasi, dan hook menangani otomasi. Pengaturan nyata menggabungkannya berdasarkan alur kerja Anda.

Misalnya, Anda mungkin menggunakan CLAUDE.md untuk konvensi proyek, skill untuk alur kerja penyebaran Anda, MCP untuk terhubung ke basis data Anda, dan hook untuk menjalankan linting setelah setiap pengeditan. Setiap fitur menangani apa yang terbaik.

| Pola                   | Cara kerjanya                                                                                      | Contoh                                                                                            |
| ---------------------- | -------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------- |
| **Skill + MCP**        | MCP menyediakan koneksi; skill mengajarkan Claude cara menggunakannya dengan baik                  | MCP terhubung ke basis data Anda, skill mendokumentasikan skema dan pola kueri Anda               |
| **Skill + Subagent**   | Skill memunculkan subagent untuk pekerjaan paralel                                                 | Skill `/audit` memulai subagent keamanan, kinerja, dan gaya yang bekerja dalam konteks terisolasi |
| **CLAUDE.md + Skills** | CLAUDE.md memegang aturan yang selalu aktif; skill memegang materi referensi yang dimuat on-demand | CLAUDE.md mengatakan "ikuti konvensi API kami," skill berisi panduan gaya API lengkap             |
| **Hook + MCP**         | Hook memicu tindakan eksternal melalui MCP                                                         | Hook pasca-edit mengirim notifikasi Slack ketika Claude memodifikasi file kritis                  |

<h2 id="understand-context-costs">
  Pahami biaya konteks
</h2>

Setiap fitur yang Anda tambahkan mengonsumsi beberapa konteks Claude. Terlalu banyak dapat mengisi jendela konteks Anda, tetapi juga dapat menambah kebisingan yang membuat Claude kurang efektif; skills mungkin tidak dipicu dengan benar, atau Claude mungkin kehilangan jejak konvensi Anda. Memahami trade-off ini membantu Anda membangun setup yang efektif. Untuk tampilan interaktif tentang bagaimana fitur-fitur ini bergabung dalam sesi yang berjalan, lihat [Jelajahi context window](/docs/id/context-window).

<h3 id="context-cost-by-feature">
  Biaya konteks berdasarkan fitur
</h3>

Setiap fitur memiliki strategi pemuatan dan biaya konteks yang berbeda:

| Fitur                 | Kapan dimuat                                 | Apa yang dimuat                                                                                                               | Biaya konteks                                    |
| --------------------- | -------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------ |
| **CLAUDE.md**         | Awal sesi                                    | Konten penuh                                                                                                                  | Setiap permintaan                                |
| **Output styles**     | Awal sesi, dan lagi ketika Anda beralih gaya | Instruksi lengkap gaya aktif; tidak ada untuk gaya Default                                                                    | Setiap permintaan                                |
| **Skills**            | Awal sesi + ketika digunakan                 | Deskripsi di awal, konten penuh ketika digunakan                                                                              | Rendah (deskripsi setiap permintaan)\*           |
| **Server MCP**        | Awal sesi                                    | Nama alat; skema penuh on demand                                                                                              | Rendah sampai alat digunakan                     |
| **Code intelligence** | Setelah pengeditan file dan on demand        | Diagnostik setelah pengeditan; lokasi simbol saat pencarian                                                                   | Rendah; mengurangi pembacaan file di tempat lain |
| **Subagents**         | Ketika dispawn                               | Konteks segar dengan skills yang ditentukan, atau percakapan induk untuk [fork](/docs/id/sub-agents#fork-the-current-conversation) | Terisolasi dari sesi utama                       |
| **Hooks**             | Saat dipicu                                  | Tidak ada (berjalan secara eksternal)                                                                                         | Nol, kecuali hook mengembalikan konteks tambahan |

\*Secara default, deskripsi skill dimuat saat awal sesi sehingga Claude dapat memutuskan kapan menggunakannya. Atur `disable-model-invocation: true` di frontmatter skill untuk menyembunyikannya dari Claude sepenuhnya sampai Anda memanggilnya secara manual. Untuk skill yang tidak Anda tulis, atur [`skillOverrides`](/docs/id/skills#override-skill-visibility-from-settings) di settings untuk melakukan hal yang sama tanpa mengedit filenya.

<h3 id="understand-how-features-load">
  Pahami bagaimana fitur dimuat
</h3>

Setiap fitur dimuat pada titik berbeda dalam sesi Anda. Tab di bawah menjelaskan kapan masing-masing dimuat dan apa yang masuk ke konteks.

<img src="https://mintcdn.com/claude-code/ikqp3_70mqIahteV/images/context-loading.svg?fit=max&auto=format&n=ikqp3_70mqIahteV&q=85&s=aab139e750494a237ae2e0c8f9139b0a" className="dark:hidden" alt="Pemuatan konteks: CLAUDE.md dimuat saat awal sesi dan tetap di setiap permintaan. Nama alat MCP dimuat saat awal dengan skema penuh ditunda sampai digunakan. Skills memuat deskripsi saat awal, konten penuh saat invokasi. Subagents mendapat konteks terisolasi. Hooks berjalan secara eksternal." width="720" height="382" data-path="images/context-loading.svg" />

<img src="https://mintcdn.com/claude-code/_xqph1dUOslCOwsj/images/context-loading-dark.svg?fit=max&auto=format&n=_xqph1dUOslCOwsj&q=85&s=b274089ef9612d9c760bca9838557626" className="hidden dark:block" alt="Pemuatan konteks: CLAUDE.md dimuat saat awal sesi dan tetap di setiap permintaan. Nama alat MCP dimuat saat awal dengan skema penuh ditunda sampai digunakan. Skills memuat deskripsi saat awal, konten penuh saat invokasi. Subagents mendapat konteks terisolasi. Hooks berjalan secara eksternal." width="720" height="382" data-path="images/context-loading-dark.svg" />

<Tabs>
  <Tab title="CLAUDE.md">
    **Kapan:** Awal sesi

    **Apa yang dimuat:** Konten penuh semua file CLAUDE.md (tingkat terkelola, pengguna, dan proyek).

    **Warisan:** Claude membaca file CLAUDE.md dari direktori kerja Anda hingga ke root, dan menemukan yang tersarang di subdirektori saat mengakses file tersebut. Lihat [Bagaimana file CLAUDE.md dimuat](/docs/id/memory#how-claude-md-files-load) untuk detail.

    <Tip>Jaga CLAUDE.md di bawah 200 baris. Pindahkan materi referensi ke skills, yang dimuat on demand. Untuk mendapatkan [proposal trim untuk CLAUDE.md yang diperiksa](/docs/id/memory#my-claude-md-is-too-large), jalankan `/doctor`.</Tip>
  </Tab>

  <Tab title="Skills">
    Skills adalah kemampuan tambahan dalam toolkit Claude. Mereka dapat berupa materi referensi (seperti panduan gaya API) atau alur kerja yang dapat dipanggil yang Anda picu dengan `/<name>` (seperti `/deploy`). Claude Code dilengkapi dengan [skills bundel](/docs/id/commands) seperti `/code-review`, `/batch`, dan `/debug` yang bekerja langsung. Anda juga dapat membuat yang Anda sendiri.

    **Kapan:** Tergantung pada konfigurasi skill. Secara default, deskripsi dimuat saat awal sesi dan konten penuh dimuat ketika digunakan. Untuk skills hanya pengguna (`disable-model-invocation: true`), tidak ada yang dimuat sampai Anda memanggilnya.

    **Apa yang dimuat:** Untuk skills yang dapat dipanggil model, Claude melihat nama dan deskripsi di setiap permintaan. Ketika Anda memanggil skill dengan `/<name>` atau Claude memuatnya secara otomatis, konten penuh dimuat ke percakapan Anda.

    **Bagaimana Claude memilih skills:** Claude mencocokkan tugas Anda terhadap deskripsi skill untuk memutuskan mana yang relevan. Jika deskripsi samar atau tumpang tindih, Claude mungkin memuat skill yang salah atau melewatkan yang akan membantu. Untuk memberi tahu Claude menggunakan skill tertentu, panggilnya dengan `/<name>`. Skills dengan `disable-model-invocation: true` tidak terlihat oleh Claude sampai Anda memanggilnya.

    **Biaya konteks:** Rendah sampai digunakan. Skills hanya pengguna memiliki biaya nol sampai dipanggil.

    **Di subagents:** Skills bekerja berbeda di subagents. Alih-alih pemuatan on-demand, skills yang tercantum di field `skills:` subagent sepenuhnya dimuat sebelumnya ke konteksnya saat peluncuran. Subagents masih dapat menemukan dan memanggil skills proyek, pengguna, dan plugin yang tidak tercantum melalui alat Skill.

    <Tip>Gunakan `disable-model-invocation: true` untuk skills dengan efek samping. Ini menghemat konteks dan memastikan hanya Anda yang memicunya.</Tip>
  </Tab>

  <Tab title="Server MCP">
    **Kapan:** Awal sesi.

    **Apa yang dimuat:** Nama alat dan instruksi server dari server yang terhubung. Skema JSON penuh tetap ditunda sampai Claude memerlukan alat tertentu.

    **Biaya konteks:** [Pencarian alat](/docs/id/mcp#scale-with-mcp-tool-search) diaktifkan secara default, jadi alat MCP idle mengonsumsi konteks minimal.

    <Tip>Jalankan `/mcp` untuk melihat status koneksi setiap server. Jalankan `/context all` untuk melihat berapa banyak token yang digunakan setiap alat MCP yang dimuat. Claude Code [terhubung kembali ke server jarak jauh secara otomatis](/docs/id/mcp#automatic-reconnection) jika mereka terputus, dan Anda dapat memutuskan server yang tidak Anda gunakan secara aktif.</Tip>
  </Tab>

  <Tab title="Code intelligence">
    **Kapan:** Setelah pengeditan file, dan on demand ketika Claude menavigasi kode.

    **Apa yang dimuat:** Kesalahan tipe dan peringatan setelah setiap pengeditan file. Informasi definisi, referensi, dan tipe ketika Claude mencari simbol.

    **Biaya konteks:** Rendah. Pencarian simbol sering menggantikan pembacaan file yang luas, jadi penggunaan konteks bersih dapat turun.

    <Tip>Alat LSP tidak aktif sampai Anda memasang [plugin code intelligence](/docs/id/plugins/code-intelligence) untuk bahasa Anda.</Tip>
  </Tab>

  <Tab title="Subagents">
    **Kapan:** On demand, ketika Anda atau Claude menspawn satu untuk tugas.

    **Apa yang dimuat:** Konteks segar dan terisolasi yang berisi:

    * Prompt sistem agen, bukan prompt sistem Claude Code
    * Konten penuh skills yang tercantum di field `skills:` agen
    * CLAUDE.md dan status git, kecuali agen Explore dan Plan bawaan [menghilangkan keduanya](/docs/id/sub-agents#what-loads-at-startup), dan agen yang definisinya menetapkan [`omitClaudeMd`](/docs/id/sub-agents#supported-frontmatter-fields) melewati file CLAUDE.md pengguna, proyek, dan lokal
    * Apa pun konteks yang agen utama lewatkan dalam prompt

    Untuk [fork](/docs/id/sub-agents#fork-the-current-conversation), Claude Code memuat percakapan induk sejauh ini, prompt sistem, dan alat sebagai gantinya.

    **Biaya konteks:** Terisolasi dari sesi utama.

    <Tip>Gunakan subagents untuk pekerjaan yang tidak memerlukan konteks percakapan penuh Anda. Isolasi mereka mencegah mengembang sesi utama Anda.</Tip>
  </Tab>

  <Tab title="Hooks">
    **Kapan:** Saat dipicu. Claude Code menjalankan hooks pada acara siklus hidup tertentu seperti eksekusi alat, batas sesi, pengajuan prompt, permintaan izin, dan pemadatan. Lihat [Hooks](/docs/id/hooks) untuk daftar lengkap.

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

  <Card title="Dynamic workflows" icon="network" href="/docs/id/workflows">
    Jalankan banyak subagents dari satu skrip
  </Card>

  <Card title="Cross-session messaging" icon="terminal" href="/docs/id/cross-session-messaging">
    Biarkan Claude mengirim pesan ke sesi lain Anda
  </Card>

  <Card title="MCP" icon="plug" href="/docs/id/mcp">
    Hubungkan Claude ke layanan eksternal
  </Card>

  <Card title="Hooks" icon="bolt" href="/docs/id/hooks-guide">
    Otomatisasi tindakan dengan hooks
  </Card>

  <Card title="Plugins" icon="puzzle-piece" href="/docs/id/plugins/overview">
    Bundel dan bagikan set fitur
  </Card>

  <Card title="Marketplaces" icon="store" href="/docs/id/plugins/create-marketplace">
    Host dan distribusikan koleksi plugin
  </Card>
</CardGroup>
