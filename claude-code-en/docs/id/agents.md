> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Jalankan agen secara paralel

> Bandingkan cara Claude Code dapat menangani beberapa tugas sekaligus: subagents, agent view, agent teams, dan dynamic workflows.

[Subagents](/docs/id/sub-agents), [agent view](/docs/id/agent-view), [agent teams](/docs/id/agent-teams), dan [dynamic workflows](/docs/id/workflows) masing-masing melakukan paralelisasi pekerjaan dengan cara yang berbeda. Yang tepat tergantung pada apakah Anda ingin tetap berada di setiap percakapan sendiri, menyerahkan tugas dan memeriksa kembali nanti, atau membiarkan Claude mengoordinasikan sekelompok pekerja untuk Anda.

| Pendekatan                         | Apa yang diberikannya                                                                                                                                                                                         | Gunakan ketika                                                                                                                                                                                                                |
| :--------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [Subagents](/docs/id/sub-agents)        | Pekerja delegasi di dalam satu sesi yang melakukan tugas sampingan dalam konteks mereka sendiri dan mengembalikan ringkasan                                                                                   | Tugas sampingan akan membanjiri percakapan utama Anda dengan hasil pencarian, log, atau konten file yang tidak akan Anda referensikan lagi                                                                                    |
| [Agent view](/docs/id/agent-view)       | Satu layar untuk mengirim dan memantau sesi yang berjalan di latar belakang, dibuka dengan `claude agents`. Pratinjau penelitian                                                                              | Anda memiliki beberapa tugas independen dan ingin menyerahkannya, memeriksa status sekilas, dan hanya melangkah ketika salah satunya membutuhkan Anda                                                                         |
| [Agent teams](/docs/id/agent-teams)     | Beberapa sesi terkoordinasi dengan daftar tugas bersama dan pesan antar-agen, dikelola oleh pemimpin. Eksperimental dan dinonaktifkan secara default                                                          | Anda ingin Claude membagi proyek menjadi beberapa bagian, menugaskannya, dan menjaga pekerja tetap tersinkronisasi                                                                                                            |
| [Dynamic workflows](/docs/id/workflows) | Skrip yang menjalankan banyak subagents dan memeriksa silang hasil mereka, untuk pekerjaan yang terlalu besar untuk dikoordinasikan satu putaran sekaligus atau yang memerlukan lebih dari satu kali lintasan | Tugas terlalu besar untuk segelintir subagents, atau Anda ingin temuan diverifikasi satu sama lain: audit seluruh codebase, migrasi 500 file, penelitian yang diperiksa silang, atau rencana yang disusun dari beberapa sudut |

Dalam setiap pendekatan, pekerja adalah sesi Claude. Untuk melibatkan alat yang berbeda, paparkan ke Claude sebagai [MCP server](/docs/id/mcp).

Dua alat lagi mendukung pekerjaan ini tanpa menjadi cara untuk menjalankan agen sendiri:

* [Worktrees](/docs/id/worktrees) memberikan setiap sesi checkout git terpisah, sehingga sesi paralel tidak pernah mengedit file yang sama. Gunakan untuk sesi yang Anda jalankan sendiri. Agent view secara otomatis memindahkan setiap sesi yang dikirim ke worktree-nya sendiri, dan subagents yang Anda hasilkan dapat masing-masing mendapatkan satu juga.
* [`/batch`](/docs/id/commands) adalah [skill](/docs/id/skills) yang membuat Claude membagi satu perubahan besar menjadi 5 hingga 30 subagents terisolasi worktree yang masing-masing membuka pull request. Ini adalah penggunaan subagents dan worktrees yang dikemas, bukan gaya koordinasi terpisah.

Beberapa fitur lain menjalankan Claude tanpa Anda menjalankan setiap langkah, tetapi mereka menyelesaikan masalah yang berbeda daripada membagi pekerjaan di seluruh agen:

* Perintah bash latar belakang menjalankan satu perintah shell tanpa memblokir percakapan. Ini tidak menghasilkan agen.
* [Subagent yang di-fork](/docs/id/sub-agents#fork-the-current-conversation) adalah subagent yang mewarisi konteks percakapan lengkap Anda alih-alih memulai dari awal. Ini adalah cara untuk menghasilkan subagent, bukan permukaan terpisah.
* [Routine](/docs/id/routines) menjalankan sesi sesuai jadwal di cloud Anthropic, bukan secara paralel di mesin Anda.

<Note>
  Menjalankan beberapa sesi atau subagents sekaligus mengalikan penggunaan token. Lihat [Costs](/docs/id/costs) untuk detail penggunaan dan batas laju.
</Note>

<h2 id="choose-an-approach">
  Pilih pendekatan
</h2>

Pendekatan yang tepat tergantung pada siapa yang mengoordinasikan pekerjaan, apakah pekerja perlu berkomunikasi, dan apakah mereka mengedit file yang sama:

* **Siapa yang mengoordinasikan pekerjaan?**
  * Claude mendelegasikan dan mengumpulkan hasil di dalam satu percakapan: [subagents](/docs/id/sub-agents)
  * Anda menyerahkan tugas independen dan memeriksa kembali nanti: [agent view](/docs/id/agent-view)
  * Claude merencanakan, menugaskan, dan mengawasi sekelompok pekerja: [agent teams](/docs/id/agent-teams), eksperimental dan dinonaktifkan secara default
  * Skrip memegang rencana alih-alih penilaian giliran demi giliran Claude: [dynamic workflows](/docs/id/workflows). Lihat [bagaimana workflows dibandingkan dengan subagents dan skills](/docs/id/workflows#when-to-use-a-workflow)
* **Apakah pekerja perlu berbicara satu sama lain?** Subagents melaporkan hasil kembali ke percakapan yang menghasilkannya, dan sesi agent view hanya melaporkan kepada Anda. Rekan satu tim dalam agent team berbagi daftar tugas dan saling mengirim pesan secara langsung.
* **Apakah tugas menyentuh file yang sama?** Isolasi pekerjaan dengan [worktrees](/docs/id/worktrees). Subagents dan sesi yang Anda jalankan sendiri dapat masing-masing menggunakan worktree terpisah. Agent teams tidak mengisolasi rekan satu tim dalam worktrees, jadi [partisi pekerjaan](/docs/id/agent-teams#avoid-file-conflicts) sehingga setiap rekan satu tim memiliki set file yang berbeda.

<h2 id="check-on-running-work">
  Periksa pekerjaan yang sedang berjalan
</h2>

Perintah untuk memeriksa pekerjaan yang sedang berjalan tergantung pada pendekatan mana yang Anda gunakan:

* Untuk sesi latar belakang, `claude agents` membuka [agent view](/docs/id/agent-view): satu layar menampilkan setiap sesi, statusnya, dan mana yang membutuhkan input Anda.
* Untuk subagents dalam sesi saat ini, subagents latar belakang bernama muncul dalam typeahead @-mention dengan statusnya. Mulai dari v2.1.198, `/agents` tidak lagi membuka panel; ini mencetak pemberitahuan yang menunjuk ke lokasi file subagent. Untuk [membuat dan mengedit subagents kustom](/docs/id/sub-agents#configure-subagents), tanyakan kepada Claude atau edit file secara langsung. Meskipun nama serupa, `/agents` terpisah dari `claude agents`.
* Untuk apa pun yang berjalan di latar belakang sesi saat ini, `/tasks` mencantumkan setiap item dan memungkinkan Anda memeriksa, melampirkan, atau menghentikannya. Daftar ini juga mencakup subagents yang telah selesai.
* Untuk dynamic workflows, `/workflows` mencantumkan runs yang sedang berjalan dan yang telah selesai, fase yang masing-masing berada di dalamnya, dan berapa banyak agents yang telah selesai.

Untuk tampilan desktop dari semua sesi Anda, lihat [sesi paralel di aplikasi desktop](/docs/id/desktop#work-in-parallel-with-sessions).

<h2 id="learn-more">
  Pelajari lebih lanjut
</h2>

Setiap panduan di bawah mencakup pengaturan dan konfigurasi untuk satu pendekatan:

* [Buat subagents kustom](/docs/id/sub-agents): tentukan spesialis yang dapat digunakan kembali dan kontrol alat mana yang dapat mereka gunakan.
* [Kelola agen dengan agent view](/docs/id/agent-view): kirim sesi, pantau statusnya, dan lampirkan ketika salah satunya membutuhkan Anda.
* [Orkestrasi agent teams](/docs/id/agent-teams): atur pemimpin dan rekan satu tim, tugaskan tugas, dan tinjau pekerjaan mereka.
* [Orkestrasi alur kerja dinamis](/docs/id/workflows): jalankan alur kerja bundel atau biarkan Claude menulis satu yang menjalankan banyak subagents dan memverifikasi temuan mereka satu sama lain.
* [Jalankan sesi paralel dengan worktrees](/docs/id/worktrees): mulai Claude dalam checkout terisolasi, kontrol apa yang disalin, dan bersihkan setelahnya.
