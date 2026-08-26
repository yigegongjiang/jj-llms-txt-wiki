> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Kelola banyak agen dengan tampilan agen

> Kirim dan kelola banyak sesi Claude Code dari satu layar. Tampilan agen menunjukkan apa yang dilakukan setiap sesi dan mana yang membutuhkan masukan Anda.

Tampilan agen, dibuka dengan `claude agents`, adalah satu layar untuk semua sesi latar belakang Anda: apa yang sedang berjalan, apa yang membutuhkan masukan Anda, dan apa yang sudah selesai. Kirim sesi baru, pantau keadaan mereka sekilas alih-alih menggulir transkrip, dan campur tangan hanya ketika ada yang membutuhkan Anda. Setiap sesi latar belakang adalah percakapan Claude Code lengkap yang terus berjalan tanpa terminal yang terpasang, sehingga Anda dapat membukanya, membalas, dan pergi kapan saja.

<img src="https://mintcdn.com/claude-code/1B48Qz2Z9hac4SLG/images/agent-view-light.png?fit=max&auto=format&n=1B48Qz2Z9hac4SLG&q=85&s=7a186c96ed47d6700d084d77e786be65" className="dark:hidden" alt="Tampilan agen di terminal: header menunjukkan Claude Code v2.1.140, model, direktori kerja, dan ringkasan jumlah. Sesi dikelompokkan di bawah Membutuhkan masukan, Bekerja, dan Selesai, dengan input pengiriman di bagian bawah dan footer petunjuk keyboard." width="1772" height="780" data-path="images/agent-view-light.png" />

<img src="https://mintcdn.com/claude-code/1B48Qz2Z9hac4SLG/images/agent-view-dark.png?fit=max&auto=format&n=1B48Qz2Z9hac4SLG&q=85&s=a5bed7434bae368faea3a8f023b52aa2" className="hidden dark:block" alt="Tampilan agen di terminal: header menunjukkan Claude Code v2.1.140, model, direktori kerja, dan ringkasan jumlah. Sesi dikelompokkan di bawah Membutuhkan masukan, Bekerja, dan Selesai, dengan input pengiriman di bagian bawah dan footer petunjuk keyboard." width="1772" height="780" data-path="images/agent-view-dark.png" />

Gunakan tampilan agen ketika Anda memiliki beberapa tugas independen yang dapat dikerjakan Claude tanpa Anda menonton setiap langkah. Kirim perbaikan bug, tinjauan permintaan tarik, dan investigasi tes yang tidak stabil sebagai tiga baris, terus bekerja di jendela lain, dan periksa kembali ketika baris menunjukkan bahwa itu membutuhkan Anda atau memiliki hasil.

Ketika Anda ingin bekerja lebih langsung di sesi agen mana pun, lampirkan ke baris untuk memasuki percakapan lengkap.

Untuk membandingkan tampilan agen dengan subagen, tim agen, dan worktrees, lihat [Jalankan agen secara paralel](/docs/id/agents).

<Note>
  Tampilan agen adalah pratinjau penelitian dan memerlukan Claude Code v2.1.139 atau lebih baru. Periksa versi Anda dengan `claude --version`. Antarmuka dan pintasan keyboard mungkin berubah seiring dengan evolusi fitur.
</Note>

Halaman ini mencakup:

* [Mulai cepat](#quick-start): berikan Claude tugas untuk dikerjakan di latar belakang, periksa, dan campur tangan ketika diperlukan
* [Pantau sesi dengan tampilan agen](#monitor-sessions-with-agent-view), termasuk ikon status, mengintip dan membalas, melampirkan, mengorganisir, dan pintasan keyboard
* [Kirim agen baru](#dispatch-new-agents) dari tampilan agen, dari dalam sesi, atau dari shell Anda
* [Kelola sesi dari shell](#manage-sessions-from-the-shell) dengan `claude agents`, `claude attach`, dan perintah terkait
* [Bagaimana sesi latar belakang dihosting](#how-background-sessions-are-hosted) oleh proses supervisor

<h2 id="quick-start">
  Mulai cepat
</h2>

Panduan ini mencakup loop tampilan agen inti: kirim tugas, tonton barisnya diperbarui saat Claude bekerja, intip untuk memeriksanya dan balas, serta lampirkan untuk percakapan lengkap. Sesi yang Anda kirim terus berjalan setelah Anda menutup tampilan agen, jadi Anda dapat pergi dan kembali ke sesi tersebut.

<Steps>
  <Step title="Buka tampilan agen">
    Dari shell Anda, jalankan:

    ```bash theme={null}
    claude agents
    ```

    Tampilan agen terbuka dengan input di bagian bawah dan tabel yang terisi saat sesi dimulai. Tekan `Esc` kapan saja untuk kembali ke shell Anda. Sesi Anda terus berjalan saat Anda pergi dan muncul kembali saat Anda membuka tampilan agen berikutnya.
  </Step>

  <Step title="Kirim sesi">
    Ketik prompt yang menjelaskan tugas dan tekan `Enter`. Sesi latar belakang baru dimulai pada tugas tersebut dan muncul sebagai baris yang menunjukkan apakah sedang bekerja, menunggu Anda, atau selesai. Sesi baru menggunakan model yang ditampilkan di header tampilan agen dan [mode izin](#permission-mode-model-and-effort) yang sama seperti yang Anda dapatkan saat menjalankan `claude` di direktori tersebut.

    Setiap prompt yang Anda masukkan di sini memulai sesi baru sendiri. Mengetik prompt lain dan menekan `Enter` meluncurkan sesi kedua bersama yang pertama daripada mengirim tindak lanjut ke sesi tersebut. Anda dapat menjalankan beberapa secara paralel dengan cara ini.

    Setiap sesi menggunakan kuota langganan Anda secara independen, jadi lihat [Batasan](#limitations) sebelum mengirim banyak sekaligus.
  </Step>

  <Step title="Intip dan balas">
    Pilih baris dengan tombol panah dan tekan `Space` untuk membuka panel intip. Panel ini menampilkan output terbaru sesi atau pertanyaan yang sedang ditunggu, bukan transkrip lengkap. Ketik balasan dan tekan `Enter` untuk mengirimnya tanpa meninggalkan tampilan agen.
  </Step>

  <Step title="Lampirkan dan lepaskan">
    Tekan `Enter` atau `→` pada baris untuk melampirkan ketika Anda menginginkan percakapan lengkap. Sesi mengambil alih terminal sebagai sesi Claude Code interaktif penuh. Tekan `←` pada prompt kosong untuk melepaskan dan kembali ke tabel.
  </Step>

  <Step title="Bawa sesi yang ada ke dalam">
    Langkah ini memerlukan sesi yang sedang berjalan. Jika Anda mengikuti langkah-langkah sebelumnya, Anda tidak memiliki sesi yang terbuka di terminal ini, jadi buka sesi `claude` biasa di terminal lain dan kirim pesan terlebih dahulu. Untuk memindahkan sesi yang sudah Anda buka ke tampilan agen, jalankan `/bg` di dalamnya, atau tekan `←` pada prompt kosong untuk mengirimnya ke latar belakang dan membuka tampilan agen dalam satu langkah. Sesi terus berjalan dan muncul sebagai baris bersama yang Anda kirim.
  </Step>
</Steps>

Anda dapat menggunakan `claude agents` sebagai titik masuk utama Anda alih-alih `claude`: kirim setiap tugas dari tampilan agen, lampirkan ketika Anda menginginkan percakapan lengkap, dan tekan `←` untuk kembali ke tabel.

Di dalam sesi `claude` biasa, petunjuk `←` footer prompt menghitung agen latar belakang yang menunggu Anda, seperti `← 2 agents`, dan kembali ke `← for agents` ketika tidak ada yang memerlukan input. Hitungan di atas 99 ditampilkan sebagai `99+`. Hitungan menyegarkan sekitar setiap sepuluh detik saat terminal difokuskan dan segera ketika fokus kembali. Warna berubah sebentar ketika bergerak dan ketika agen selesai, kecuali pengaturan [`prefersReducedMotion`](/docs/id/settings#available-settings) aktif, dan tersembunyi dalam [mode pembaca layar](/docs/id/accessibility). Di [Amazon Bedrock, Google Cloud's Agent Platform, dan Microsoft Foundry](/docs/id/third-party-integrations), petunjuk tetap dalam bentuk `← for agents` biasa tanpa hitungan. Memerlukan Claude Code v2.1.205 atau lebih baru.

<h2 id="monitor-sessions-with-agent-view">
  Pantau sesi dengan tampilan agen
</h2>

Jalankan `claude agents` untuk membuka tampilan agen. Ini mengambil alih terminal penuh dan mencantumkan setiap sesi yang dikelompokkan berdasarkan status, dengan sesi yang disematkan dan yang membutuhkan Anda di bagian atas. Setiap baris menunjukkan nama sesi, aktivitas saat ini, dan usianya, dihitung dari saat sesi dibuat; usia sesi yang selesai membeku pada berapa lama waktu yang dibutuhkan untuk menjalankannya.

Nama tersebut berwarna dengan warna yang ditetapkan oleh [`/color`](/docs/id/commands) dalam sesi itu. Mulai dari v2.1.199, warna terbawa ketika Anda [mengirim sesi ke latar belakang](#from-inside-a-session) dengan `←` atau `/background`.

Secara default, daftar menampilkan setiap sesi latar belakang yang telah Anda mulai, di seluruh semua proyek Anda. Sesi yang bekerja di satu repositori dan sesi lain di worktree berbeda keduanya muncul di sini, terlepas dari direktori mana yang Anda buka tampilan agen dari. Untuk membatasi daftar ke satu proyek, berikan `--cwd`:

```bash theme={null}
claude agents --cwd ~/projects/my-app
```

Ini menampilkan hanya sesi yang dimulai di bawah direktori itu. Sesi yang telah [berpindah ke worktree](#how-file-edits-are-isolated) di bawah `~/projects/my-app/.claude/worktrees/` masih dihitung sebagai milik `~/projects/my-app`.

Sesi interaktif yang Anda buka di terminal lain tidak muncul sampai Anda [mengirimnya ke latar belakang](#from-inside-a-session). [Subagents](/docs/id/sub-agents) dan [teammates](/docs/id/agent-teams) yang sesi hasilkan tidak tercantum sebagai baris terpisah.

```text theme={null}
Disematkan
  ✽ clawd walk cycle          Menggambar bingkai sprite walk-cycle          3m

Siap untuk ditinjau
  ∙ jump physics              Opened PR with collision fix                 #2048  2h

Membutuhkan masukan
  ✻ power-up design           double jump atau wall climb?                    1m

Bekerja
  ✽ collision detection       Menambahkan pemeriksaan swept-AABB ke CollisionSystem   2m
  ✢ playtest level 3          run 12 · all checkpoints cleared           in 4m

Selesai
  ✻ title screen              result: menu, options, and credits done       9m
  ∙ sound effects             result: 14 SFX exported to assets/audio       4h
  … 6 more
```

<h3 id="read-session-state">
  Baca status sesi
</h3>

Setiap baris dimulai dengan ikon yang warna dan animasinya menunjukkan status sesi:

| Status              | Ikon ditampilkan sebagai | Artinya                                                                           |
| :------------------ | :----------------------- | :-------------------------------------------------------------------------------- |
| Bekerja             | Animasi                  | Claude secara aktif menjalankan alat atau menghasilkan respons                    |
| Membutuhkan masukan | Kuning                   | Claude menunggu pertanyaan spesifik atau keputusan izin dari Anda                 |
| Menganggur          | Redup                    | Sesi tidak memiliki apa pun untuk dilakukan dan siap untuk prompt berikutnya Anda |
| Selesai             | Hijau                    | Tugas selesai dengan sukses                                                       |
| Gagal               | Merah                    | Tugas berakhir dengan kesalahan                                                   |
| Dihentikan          | Abu-abu                  | Sesi dihentikan dengan `Ctrl+X` atau `claude stop`                                |

Secara terpisah, bentuk ikon menunjukkan apakah proses yang mendasarinya sedang berjalan:

| Bentuk               | Artinya                                                                                                                       |
| :------------------- | :---------------------------------------------------------------------------------------------------------------------------- |
| `✻` atau animasi `✽` | Proses sesi masih hidup dan merespons segera                                                                                  |
| `∙`                  | Proses telah keluar. Anda masih dapat mengintip, membalas, atau melampirkan, dan Claude memulai ulang dari tempat ia berhenti |
| `✢`                  | Sesi [`/loop`](/docs/id/scheduled-tasks) yang tidur di antara iterasi. Baris menunjukkan jumlah jalannya dan hitungan mundur       |

Label `#N` yang dapat muncul di tepi kanan baris adalah [permintaan tarik yang dibuka sesi](#pull-request-status), bukan bagian dari ikon status.

Judul tab terminal menunjukkan jumlah menunggu-masukan saat tampilan agen terbuka: `2 awaiting input · claude agents` ketika sesi membutuhkan masukan, atau `claude agents` ketika tidak ada.

Mulai dari v2.1.198, saat tampilan agen terbuka, Claude Code juga mengirim notifikasi melalui [saluran notifikasi terminal](/docs/id/terminal-config#get-a-terminal-bell-or-notification) yang dikonfigurasi ketika sesi latar belakang lokal mulai membutuhkan masukan Anda, selesai, atau gagal. Sesi yang berjalan sesuai jadwal, seperti sesi [`/loop`](/docs/id/scheduled-tasks), hanya memberitahu ketika mereka membutuhkan masukan Anda. Notifikasi menggunakan pengaturan [`preferredNotifChannel`](/docs/id/settings#available-settings) yang sama dengan sisa Claude Code dan menjalankan hook [`Notification`](/docs/id/hooks#notification) dengan tipe `agent_needs_input` atau `agent_completed`.

Sesi latar belakang tidak memerlukan terminal apa pun yang terbuka untuk terus bekerja. [Proses supervisor](#the-supervisor-process) terpisah menjalankannya, jadi Anda dapat menutup tampilan agen, menutup shell, atau memulai sesi interaktif baru dan pekerjaan yang dikirim terus berlanjut.

Status sesi bertahan di disk melalui pembaruan otomatis dan restart supervisor. Sesi juga dipertahankan ketika mesin Anda tidur. Proses mereka dilanjutkan saat bangun dan supervisor terhubung kembali ke mereka alih-alih memperlakukan celah waktu sebagai menganggur. Mematikan masih menghentikan sesi yang berjalan; lihat [Sesi menampilkan gagal setelah shutdown](#sessions-show-as-failed-after-shutdown) untuk cara memulihkannya.

Ketika Anda membuka sesi yang telah berhenti merespons, supervisor memulai ulang prosesnya dan sesi melanjutkan respons yang terputus dari tempat ia berhenti. Sesi dapat berakhir dalam status itu ketika mesin tidur saat sedang merespons. Memerlukan Claude Code v2.1.200 atau lebih baru.

<h3 id="row-summaries">
  Ringkasan baris
</h3>

Ringkasan satu baris di setiap baris dihasilkan oleh [model kelas Haiku](/docs/id/model-config) sehingga baris dapat memberi tahu Anda apa yang dilakukan sesi, apa yang dibutuhkannya, atau apa yang dihasilkannya tanpa membuka transkrip. Saat sesi secara aktif bekerja, ringkasan menyegarkan paling banyak sekali setiap 15 detik dari output terbaru sesi itu sendiri tanpa mengirim permintaan model, dan model menulis ringkasan segar ketika setiap giliran berakhir.

Baris yang bekerja menunjukkan apa yang dikatakan sesi sedang dilakukan, dan baris yang diblokir menunjukkan pertanyaan yang diajukannya. Selama giliran yang panjang, model juga menulis ulang ringkasan sekitar sekali semenit, menunggu dua kali lebih lama setelah setiap penulisan ulang hingga empat menit, sehingga baris yang sibuk tidak terus menampilkan ringkasan yang sudah ketinggalan zaman. Teks ringkasan mengisi lebar sisa baris dan hanya terpotong di tepi kanan terminal; buka [panel intip](#peek-and-reply) untuk membaca kalimat yang dipotong tepi. Sebelum v2.1.206, teks dipotong pada 64 kolom terlepas dari lebar terminal.

Ketika daftar [dikelompokkan berdasarkan direktori](#organize-the-list), ringkasan dibuka dengan status sesi sebagai kata berwarna, seperti `Needs input · double jump or wall climb?`. Dalam pengelompokan status default, header grup sudah menamai status, jadi baris hanya menampilkan ringkasan. Sebelum v2.1.205, baris yang dikelompokkan direktori tidak membawa kata status.

Giliran yang seluruh outputnya tidak mengandung huruf atau angka, seperti sesi [`/loop`](/docs/id/scheduled-tasks) yang mencetak simbol tunggal pada iterasi yang tenang, menyimpan ringkasan dan status baris sebelumnya. Sebelum v2.1.205, giliran itu diklasifikasikan ulang dan dapat membalik sesi yang menunggu masukan Anda kembali ke `Bekerja`.

Ringkasan akhir giliran dan setiap penulisan ulang pertengahan giliran adalah satu permintaan kelas Haiku pendek melalui penyedia normal Anda, ditagih dan ditangani di bawah [persyaratan penggunaan data](/docs/id/data-usage) yang sama dengan sesi itu sendiri. Pembaruan 15 detik antara penulisan ulang model menggunakan kembali output sesi itu sendiri dan tidak mengirim permintaan. Pada penyedia pihak ketiga seperti Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry, dan gateway khusus, permintaan kembali ke model utama sesi ketika tidak ada model Haiku yang dikonfigurasi. Atur [`ANTHROPIC_DEFAULT_HAIKU_MODEL`](/docs/id/model-config#environment-variables) untuk memilih model untuk ringkasan ini pada penyedia tersebut.

<h3 id="pull-request-status">
  Status permintaan tarik
</h3>

Ketika sesi membuka permintaan tarik, label `#1234` muncul di tepi kanan baris, tertaut ke permintaan tarik di terminal yang mendukung hyperlink. Label bertahan ketika Anda mengirim tindak lanjut ke sesi, sehingga permintaan tarik tetap terlihat sementara baris kembali ke kemajuan langsung. Sesi latar belakang yang mengisolasi perubahan mereka dalam worktree membuka permintaan tarik ini sendiri; [Bagaimana file edit diisolasi](#how-file-edits-are-isolated) mencakup kapan itu terjadi dan apa yang tidak pernah dilakukan sesi tanpa bertanya.

Sesi yang bekerja pada permintaan tarik yang ada tertaut ke sana dengan cara yang sama. Mengedit, mengomentari, menutup, atau menandai permintaan tarik siap dengan `gh` menautkan permintaan tarik yang dinamai output perintah itu sendiri, jadi perintah `gh` yang output tertangkapnya tidak menamai permintaan tarik tidak membuat tautan; `gh pr merge` adalah kasus umum, karena mencetak hasilnya hanya ke terminal interaktif. Memeriksa permintaan tarik dengan `gh pr checkout`, atau mendorong ke cabang yang memiliki permintaan tarik terbuka, menautkannya dengan mencari cabang itu dengan `gh pr view` sebagai gantinya. Sebelum v2.1.205, hanya permintaan tarik yang dibuat atau diperiksa sesi yang tertaut, dan dorong menautkan satu hanya ketika nama cabang lokal cocok.

Claude Code membaca permintaan tarik dari output perintah lengkap, termasuk bagian yang disimpan ke file ketika output perintah melebihi batas inline. Sebelum v2.1.205, permintaan tarik yang dibuat dalam panggilan Bash yang outputnya melebihi sekitar 30.000 karakter tidak tertaut.

Ketika sesi tertaut ke lebih dari satu permintaan tarik, label menunjukkan hitungan sebagai gantinya, seperti `3 PRs`, berwarna oleh permintaan tarik terbuka yang paling membutuhkan perhatian. Buka [panel intip](#peek-and-reply) untuk melihat semuanya.

Nomor permintaan tarik berwarna berdasarkan statusnya:

| Warna   | Status permintaan tarik                                    |
| :------ | :--------------------------------------------------------- |
| Kuning  | Menunggu pemeriksaan atau tinjauan, atau pemeriksaan gagal |
| Hijau   | Pemeriksaan lulus dan tidak ada tinjauan yang memblokir    |
| Ungu    | Digabungkan                                                |
| Abu-abu | Draf atau ditutup                                          |

Untuk sebagian besar tugas, kolom ini adalah tempat Anda mengambil hasilnya: tinjau dan gabungkan permintaan tarik ketika nomornya berubah menjadi hijau.

<h3 id="peek-and-reply">
  Intip dan balas
</h3>

Tekan `Space` pada baris yang dipilih untuk membuka panel intip. Ini membuka dengan kalimat yang baris potong di tepi terminal, dan kalimat mana itu tergantung pada status sesi:

* Sesi yang menunggu Anda: pertanyaan pasti yang diajukannya, di atas input balasan
* Sesi yang selesai: hasilnya
* Sesi yang bekerja: kalimat status lengkapnya

Permintaan tarik apa pun yang tertaut ke sesi tercantum selanjutnya. Untuk sesi yang menunggu Anda, baris seperti `waiting 3m` di bawahnya menunjukkan berapa lama ia telah menunggu, dan itu adalah satu-satunya waktu yang ditampilkan di panel. Usia di tepi kanan baris adalah angka yang berbeda: itu dihitung dari saat sesi dimulai.

Sebagian besar waktu panel intip cukup dan Anda tidak perlu membuka transkrip lengkap.

Sebelum v2.1.207, setiap intip dibuka dengan kalimat status dan stempel waktu telanjang, dan sesi yang diblokir pertanyaannya muncul di bawahnya dengan awalan stempel waktu yang sama untuk kedua kalinya.

Ketik balasan di panel intip dan tekan `Enter` untuk mengirimnya ke sesi itu. Ketika sesi mengajukan pertanyaan pilihan ganda, panel intip menunjukkan opsi dan Anda dapat menekan tombol angka untuk memilih satu. Untuk sesi terhalang lainnya, tekan `Tab` untuk mengisi input dengan balasan yang disarankan yang dapat Anda edit sebelum mengirim. Awali balasan dengan `!` untuk mengirim perintah Bash sebagai gantinya.

Balasan yang tidak dapat dikirim, karena layanan latar belakang tidak dapat dijangkau atau pengiriman gagal, disimpan dan dikirim ke sesi sebagai prompt berikutnya ketika prosesnya dimulai lagi, dan pesan kesalahan mengatakan balasan disimpan. Balasan dengan awalan `!` tidak disimpan, karena teks yang disimpan akan mencapai sesi sebagai prompt biasa daripada menjalankan sebagai perintah Bash.

Dengan [dikte suara](/docs/id/voice-dictation) diaktifkan, tahan atau ketuk tombol push-to-talk Anda saat input balasan difokuskan untuk mendikte balasan alih-alih mengetiknya. Hal yang sama berlaku dalam input pengiriman di bagian bawah tampilan agen.

Gunakan `↑` dan `↓` untuk mengintip sesi yang berdekatan tanpa menutup panel, atau `→` untuk melampirkan.

<h3 id="attach-to-a-session">
  Lampirkan ke sesi
</h3>

Tekan `Enter` atau `→` pada baris yang dipilih untuk melampirkan. Tampilan agen diganti oleh sesi interaktif lengkap. Ketika Anda melampirkan, Claude memposting ringkasan singkat tentang apa yang terjadi saat Anda pergi.

Saat dilampirkan, sesi berperilaku seperti sesi Claude Code lainnya: [perintah](/docs/id/commands), pintasan keyboard, dan fitur semuanya berfungsi, dengan pengecualian di bawah.

Sesi latar belakang menolak `/install-github-app` dan daftar pengaturan [`/mcp`](/docs/id/mcp), termasuk tindakan autentikasinya, apakah Anda dilampirkan atau membalas dari panel intip. Pesan mengarahkan Anda ke sesi `claude` biasa, dan `/mcp reconnect <server>`, `/mcp enable`, dan `/mcp disable` masih berfungsi.

Sesi yang dilampirkan selalu dirender dalam [mode layar penuh](/docs/id/fullscreen), terlepas dari pengaturan `tui` Anda, karena sesi latar belakang tidak memiliki scrollback terminal untuk ditambahkan. Gulir dengan `PgUp`, `PgDn`, atau roda mouse, dan tekan `Ctrl+O` untuk mode transkrip. Gulir asli terminal Anda dan mode salinan tmux hanya menampilkan viewport saat ini, sama seperti ketika Anda menjalankan aplikasi layar penuh apa pun.

Tekan `←` pada prompt kosong, atau jalankan `/exit`, untuk melepaskan dan kembali ke tampilan agen. Mulai dari v2.1.198, ini berfungsi dengan cara yang sama apakah Anda membuka sesi dari tampilan agen atau dengan `claude attach <id>` dari shell Anda.

`Ctrl+Z` juga melepaskan tetapi kembali ke tempat Anda memulai: tampilan agen jika Anda melampirkan dari sana, atau shell Anda jika Anda menjalankan `claude attach`. Gunakan `Ctrl+Z` ketika dialog memiliki fokus dan tidak merespons `←`.

`Ctrl+C` mempertahankan perilaku interupsi standarnya saat dilampirkan: ini membatalkan respons yang sedang berjalan atau perintah shell `!` daripada melepaskan. Menekan `Ctrl+C` dua kali pada prompt kosong melepaskan, sama seperti di sesi apa pun.

Melepaskan tidak pernah menghentikan sesi latar belakang: `←`, `Ctrl+Z`, `/exit`, dan `Ctrl+C` ganda atau `Ctrl+D` ganda semuanya membiarkannya berjalan. Untuk mengakhiri sesi dari dalamnya, jalankan `/stop`.

Dalam sesi yang berjalan di latar depan, sesi yang Anda mulai di terminal daripada melampirkan dari tampilan agen, menekan `←` pada prompt kosong mengirimnya ke latar belakang dan membuka tampilan agen dengan baris itu dipilih, sehingga Anda dapat beralih sesi tanpa meninggalkan terminal. Tekan tunggal yang sama melepaskan sesi yang dilampirkan.

Jika alat sedang berjalan saat Anda menekan `←`, Claude Code menunggu hingga sekitar sepuluh detik agar alat selesai sebelum mengirim ke latar belakang, dan respons berlanjut dalam sesi latar belakang. Tekan `←` lagi untuk mengirim ke latar belakang segera alih-alih menunggu. Ketika pekerjaan yang sedang berlangsung tidak dapat dibawa ke sesi latar belakang, dialog `Background this session?` muncul terlebih dahulu, sama seperti dengan [`/background`](#from-inside-a-session).

Batas sepuluh detik tidak berlaku saat [subagents](/docs/id/sub-agents) sedang berjalan. Claude Code terus menunggu sehingga pekerjaan mereka terbawa, dan menampilkan pemberitahuan `Still backgrounding after the current tool` saat menunggu; tekan `←` lagi untuk mengirim ke latar belakang tanpa menunggu, yang memulai ulang subagents dari awal. Sebelum v2.1.203, tunggu berakhir setelah sepuluh detik dan subagents yang sedang berjalan dimulai ulang dari awal tanpa peringatan.

Baris dibuat bahkan dari sesi segar tanpa riwayat percakapan, jadi `→` kembali ke sana. Sebelum v2.1.203, tampilan agen menampilkan petunjuk onboarding di bawah baris itu ketika itu adalah satu-satunya.

Anda dapat mematikan pintasan ini dengan pengaturan `leftArrowOpensAgents` di `/config`.

<h3 id="organize-the-list">
  Atur daftar
</h3>

Tampilan agen mengelompokkan sesi sehingga yang membutuhkan masukan berada di atas, dengan `Siap untuk ditinjau` dan `Membutuhkan masukan` di atas `Bekerja` dan `Selesai`. Nama grup ini tidak memetakan satu-ke-satu ke [status](#read-session-state) di atas: sesi bergerak ke `Siap untuk ditinjau` ketika memiliki permintaan tarik terbuka, dan `Selesai` mengumpulkan sesi yang selesai, gagal, dan dihentikan bersama-sama.

Tekan `Ctrl+S` untuk mengelompokkan berdasarkan direktori sebagai gantinya. Pilihan Anda bertahan di seluruh jalankan.

Dalam grup:

* Tekan `Ctrl+T` untuk menyematkan sesi ke atas dan [menjaga proses tetap berjalan](#the-supervisor-process) saat menganggur
* Tekan `Shift+↑` atau `Shift+↓` untuk mengatur ulang sesi
* Tekan `Ctrl+R` untuk mengganti nama sesi
* Tekan `Enter` pada header grup untuk menutupnya

Untuk menghapus sesi dari daftar, tekan `Ctrl+X` untuk menghentikannya dan `Ctrl+X` lagi dalam dua detik untuk menghapusnya. Menekan `Ctrl+X` pada header grup menghapus setiap sesi dalam grup itu setelah konfirmasi.

Menghapus menghapus sesi dari tampilan agen. Jika Claude [membuat worktree](#how-file-edits-are-isolated) untuk sesi, menghapus menghapus worktree itu juga, termasuk perubahan yang tidak dikomitkan di dalamnya, jadi dorong atau komitkan pekerjaan yang ingin Anda simpan terlebih dahulu. Worktree yang Anda buat sendiri dan mulai sesi di dalamnya dibiarkan di tempat. Transkrip percakapan tetap berada di mesin lokal Anda dan tetap tersedia melalui `claude --resume`.

Menghapus tidak pernah menghapus worktree dengan komit yang tidak didorong ke mana pun, atau yang sesi lain yang sedang berjalan klaim atau kunci. Claude Code menyimpan worktree dan sesi, dan footer menamai jalur yang disimpan dan alasannya. Dorong komit, atau tutup sesi lain, lalu hapus lagi.

Menghapus juga menghapus sesi dari [daftar sesi supervisor](#the-supervisor-process), apakah Anda menghapus dengan `Ctrl+X` atau dengan [`claude rm`](#manage-sessions-from-the-shell) dari shell, sehingga penghapusan bertahan di seluruh restart supervisor. Sebelum v2.1.206, menghapus sesi saat supervisor memulai ulang atau tidak dapat dijangkau meninggalkannya dalam daftar itu, dan supervisor berikutnya memulai ulang prosesnya dan menampilkan baris lagi.

Sesi yang selesai yang tidak muat di layar dilipat menjadi baris `… N more`. Kegagalan dan sesi dengan permintaan tarik terbuka selalu tetap terlihat. Grup `Completed` mengisi ruang vertikal yang tersisa setelah grup langsung, dan di terminal pendek header dikompakkan menjadi baris ringkasan tunggal sehingga sesi yang bekerja atau membutuhkan masukan tetap terlihat.

<h3 id="filter-sessions">
  Filter sesi
</h3>

Ketik dalam input pengiriman untuk memfilter alih-alih mengirim:

| Filter                  | Menampilkan                                                                                               |
| :---------------------- | :-------------------------------------------------------------------------------------------------------- |
| `a:<name>`              | Sesi yang menjalankan agen bernama                                                                        |
| `s:<state>`             | Sesi dalam status tertentu, seperti `s:working`. Juga menerima `s:blocked` untuk semua yang menunggu Anda |
| `#<number>` atau URL PR | Sesi yang bekerja pada permintaan tarik itu                                                               |
| URL lainnya             | Sesi yang prompt pertamanya berisi URL itu                                                                |

<h3 id="keyboard-shortcuts">
  Pintasan keyboard
</h3>

Tekan `?` di tampilan agen untuk melihat setiap pintasan dalam konteks. Tabel di bawah merangkumnya.

| Pintasan              | Tindakan                                                                             |
| :-------------------- | :----------------------------------------------------------------------------------- |
| `↑` / `↓`             | Pindah antar baris                                                                   |
| `Enter`               | Lampirkan ke sesi yang dipilih, atau kirim jika ada teks dalam input                 |
| `Space`               | Buka atau tutup panel intip untuk sesi yang dipilih                                  |
| `Shift+Enter`         | Kirim dan lampirkan segera                                                           |
| `→`                   | Lampirkan ke sesi yang dipilih                                                       |
| `Alt+1`..`Alt+9`      | Lampirkan ke sesi 1–9 dalam direktori sesi yang difokuskan                           |
| `Tab`                 | Pada input kosong, telusuri semua subagents. Jika tidak, terapkan saran yang disorot |
| `Ctrl+S`              | Alihkan pengelompokan antara status dan direktori                                    |
| `Ctrl+T`              | Sematkan atau lepas sematkan sesi yang dipilih                                       |
| `Ctrl+R`              | Ganti nama sesi yang dipilih                                                         |
| `Ctrl+G`              | Buka prompt pengiriman di `$VISUAL` atau `$EDITOR` Anda                              |
| `Ctrl+X`              | Hentikan sesi; tekan lagi dalam dua detik untuk menghapusnya                         |
| `Shift+↑` / `Shift+↓` | Atur ulang sesi yang dipilih                                                         |
| `Esc`                 | Tutup panel intip, hapus input, atau keluar                                          |
| `Ctrl+C`              | Hapus input; tekan dua kali untuk keluar                                             |
| `?`                   | Tampilkan semua pintasan                                                             |

<h2 id="dispatch-new-agents">
  Kirim agen baru
</h2>

Anda dapat mengirim sesi latar belakang baru dari tampilan agen, mengirim sesi interaktif yang ada ke latar belakang, atau memulai satu langsung dari shell.

<h3 id="from-agent-view">
  Dari tampilan agen
</h3>

Ketik prompt dalam input di bagian bawah tampilan agen dan tekan `Enter` untuk memulai sesi latar belakang baru. Sesi diberi nama secara otomatis dari prompt; ubah namanya nanti dengan `Ctrl+R`.

Nama yang diterima sesi nanti juga muncul di barisnya, termasuk nama yang Claude turunkan ketika Anda [menerima rencana](/docs/id/permission-modes#review-and-approve-a-plan) dalam sesi itu. Sebelum v2.1.207, sesi latar belakang yang diberi nama dengan menerima rencana menampilkan nama itu dalam `/status` tetapi bukan di baris tampilan agen-nya sampai Anda mengganti namanya sendiri.

Tempel gambar ke dalam prompt untuk menyertakan tangkapan layar atau diagram dengan tugas.

Teks yang ditempel lebih panjang dari 800 karakter atau lebih dari dua baris runtuh menjadi placeholder `[Pasted text #N]` sehingga input tetap pada satu baris; teks lengkap dikirim ketika Anda mengirim. Untuk meninjau atau mengedit teks yang runtuh sebelum mengirim, tempel teks yang sama lagi dan placeholder berkembang kembali ke dalam input. Pengingat `paste again to expand` muncul di bawah input selama beberapa detik setelah tempel pada terminal setidaknya 90 kolom lebar. Sebelum v2.1.207, menempel teks yang sama lagi menambahkan placeholder kedua alih-alih memperluas yang pertama.

Awali atau sebutkan bagian dari prompt untuk mengontrol bagaimana sesi dimulai:

| Input                                 | Efek                                                                                                                                                                   |
| :------------------------------------ | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `<agent-name> <prompt>`               | Jika kata pertama cocok dengan nama [subagent](/docs/id/sub-agents) kustom, subagent itu berjalan sebagai agen utama sesi dengan konfigurasi dari frontmatter-nya           |
| `@<agent-name>`                       | Sebutkan subagent kustom di mana saja dalam prompt untuk menjalankannya sebagai agen utama                                                                             |
| `@<repo>`                             | Sebutkan repositori untuk menjalankan sesi di sana. Lihat [Kirim ke direktori tertentu](#dispatch-to-a-specific-directory) untuk repositori mana yang terdaftar        |
| `/<command>`                          | Sarankan [skills](/docs/id/skills) dan [commands](/docs/id/commands) untuk dikirim sebagai prompt                                                                                |
| `! <command>`                         | Jalankan perintah shell sebagai pekerjaan latar belakang alih-alih memulai sesi Claude. Pekerjaan muncul sebagai baris yang dapat Anda lampirkan, tonton, dan lepaskan |
| `#<number>` atau URL permintaan tarik | Jika sesi sudah bekerja pada PR itu, pilih saja alih-alih mengirim                                                                                                     |
| `Shift+Enter`                         | Kirim dan lampirkan segera ke sesi baru                                                                                                                                |

Serangkaian kecil perintah berjalan dalam tampilan agen itu sendiri alih-alih mengirim:

* `/exit` dan `/quit` menutup tampilan agen
* `/logout` menandatangani Anda keluar
* `/model` menetapkan [model pengiriman](#set-the-model)
* Mulai dari v2.1.198, `/login` membuka dialog masuk sehingga Anda dapat masuk lagi tanpa melampirkan ke sesi

Skills, perintah Anda sendiri, dan built-in yang memperluas prompt seperti `/init` dikirim ke sesi latar belakang baru sebagai prompt pertamanya. Perintah built-in lainnya menampilkan petunjuk `attach to a session to run it` sebagai gantinya. Sebelum v2.1.203, petunjuk menghapus input dan teks yang diketik hilang.

Mengemas tugas berulang sebagai [skill](/docs/id/skills) memungkinkan Anda memulai alur kerja yang sama dari tampilan agen berulang kali tanpa mengetik ulang prompt.

Ketika `@name` yang sama cocok dengan subagent dan repositori saudara, subagent memiliki prioritas. Kecocokan kata pertama tanpa `@` juga berlaku, jadi prompt yang dimulai dengan salah satu nama subagent Anda mengirim subagent itu daripada memperlakukan kata sebagai teks biasa. Gunakan bentuk `@` ketika Anda ingin eksplisit, atau mulai prompt dengan kata berbeda untuk menghindari kecocokan.

<h4 id="dispatch-to-a-specific-directory">
  Kirim ke direktori tertentu
</h4>

Sesi baru berjalan di direktori tempat Anda membuka tampilan agen. Untuk menargetkan direktori berbeda, gunakan salah satu dari ini:

* Buka `claude agents` di direktori itu.
* Buka `claude agents` di direktori induk dan sebutkan repositori anak dengan `@<repo>` dalam prompt. Mengetik `@` mencantumkan target ini:

  * Repositori Git satu level di bawah direktori peluncuran
  * [Git worktrees](/docs/id/worktrees) terdaftar dari repositori yang Anda luncurkan dari yang berada di dalam pohon direktorinya, seperti yang Claude buat di bawah `.claude/worktrees/`, diberi label dengan cabang yang diperiksa. Worktrees yang ditambahkan di luar repositori, seperti dengan `git worktree add ../feature`, tidak terdaftar
  * Direktori apa pun yang sudah memiliki sesi dalam daftar

  Direktori yang namanya berisi spasi tidak terdaftar. Sebelum v2.1.203, worktrees terdaftar tidak terdaftar, jadi pengiriman ke dalamnya berarti menjalankan `claude --bg` dari direktori worktree itu.
* Dari shell, `cd` ke direktori dan jalankan `claude --bg "<prompt>"`.

Ketika tampilan agen dikelompokkan berdasarkan direktori, direktori baris yang disorot menjadi target pengiriman, sehingga Anda dapat menggulir ke grup dan mengirim ke dalamnya tanpa mengetik ulang jalur.

<h3 id="from-inside-a-session">
  Dari dalam sesi
</h3>

Jalankan `/background` atau aliasnya `/bg` untuk memindahkan percakapan saat ini ke sesi latar belakang. Berikan prompt seperti `/bg run the test suite and fix any failures` untuk memberikan satu instruksi lagi terlebih dahulu. Jika Claude sedang merespons ketika Anda menjalankan `/bg`, respons berlanjut dalam sesi latar belakang.

Keluar dari sesi interaktif yang masih memiliki pekerjaan latar belakang yang berjalan, seperti subagent, perintah shell latar belakang, alur kerja, atau [monitor](/docs/id/tools-reference#monitor-tool), menampilkan dialog `Background work is running` alih-alih berhenti segera. Mulai dari v2.1.198 dialog menawarkan `Move to background and exit` bersama `Exit anyway` dan `Stay`. Memilihnya memindahkan sesi ke latar belakang dengan cara yang sama seperti `/background`, kemudian mengembalikan Anda ke shell Anda, sehingga pekerjaan yang dapat dibawa terus berjalan dan sesi muncul dalam tampilan agen. Opsi tidak ditampilkan ketika tampilan agen [dimatikan](#turn-off-agent-view).

Melepaskan dari sesi interaktif memulai proses segar yang dilanjutkan dari percakapan yang disimpan, dan pekerjaan yang sedang berlangsung berpindah ke sesi itu: menjalankan perintah shell latar belakang, subagent yang dilepaskan ke latar belakang, alur kerja dinamis, dan tugas terjadwal yang Anda buat dengan [`/loop`](/docs/id/scheduled-tasks) dibawa ke sesi latar belakang dan terus berjalan di sana. Subagent bergerak bersama dengan semua yang dimulainya, jadi hanya dibawa ketika semua pekerjaan itu dapat berpindah juga, termasuk di Windows. Untuk menghentikan pekerjaan yang sedang berlangsung alih-alih membawanya, atur variabel lingkungan [`CLAUDE_DISABLE_ADOPT=1`](/docs/id/env-vars#variables); Claude Code kemudian meminta Anda untuk mengonfirmasi sebelum melepaskan.

Pekerjaan yang tidak dapat dibawa, seperti [monitor](/docs/id/tools-reference#monitor-tool) yang sedang berjalan, dihentikan. Subagent yang dilepaskan ke latar belakang yang memiliki monitor dihentikan bersama dengannya. Ketika ada pekerjaan seperti itu yang sedang berjalan, Claude Code menampilkan dialog `Background this session?` sehingga Anda dapat mengonfirmasi sebelum dihentikan.

Setelah berada di latar belakang, sesi dapat memulai subagent, monitor, dan perintah latar belakang baru, dan yang tersebut terus berjalan di seluruh detach dan reattach berikutnya.

Bendera konfigurasi dari peluncuran asli dibawa ke sesi yang dilepaskan ke latar belakang, sehingga server MCP, pengaturan, dan model fallback-nya tetap berlaku:

* `--mcp-config` dan `--strict-mcp-config`
* `--settings`
* `--add-dir`
* `--plugin-dir`
* `--fallback-model`
* `--allow-dangerously-skip-permissions`

Direktori yang Anda tambahkan selama sesi dengan [`/add-dir`](/docs/id/permissions#additional-directories-grant-file-access-not-configuration) juga dibawa ke sesi.

Membawa `--allow-dangerously-skip-permissions` melalui membuat `bypassPermissions` dapat dijangkau dalam sesi yang dilepaskan ke latar belakang, tetapi tidak memberikan apa pun yang baru. Mode masih memerlukan penerimaan interaktif satu kali yang sama seperti yang dijelaskan dalam [Mode izin, model, dan upaya](#permission-mode-model-and-effort) sebelum sesi apa pun dapat menggunakannya.

<h3 id="from-your-shell">
  Dari shell Anda
</h3>

Berikan `--bg` atau bentuk panjangnya `--background` untuk memulai sesi yang langsung masuk ke latar belakang:

```bash theme={null}
claude --bg "investigate the flaky SettingsChangeDetector test"
```

Prompt adalah argumen posisional, bukan nilai `-p`. Mulai dari v2.1.198, menggabungkan `--bg` dengan `-p` atau `--print` ditolak dengan kesalahan sebelum sesi apa pun dibuat, karena `--print` tidak pernah memulai sesi interaktif yang `claude agents` lampirkan.

Untuk menjalankan subagent tertentu sebagai agen utama sesi, gabungkan `--bg` dengan `--agent`:

```bash theme={null}
claude --agent code-reviewer --bg "address review comments on PR 1234"
```

Berikan `--name` untuk menetapkan nama tampilan sesi dalam tampilan agen alih-alih yang dibuat secara otomatis:

```bash theme={null}
claude --bg --name "flaky-test-fix" "investigate the flaky SettingsChangeDetector test"
```

Setelah melepaskan ke latar belakang, Claude mencetak ID pendek sesi dan perintah untuk mengelolanya. Ketika layanan yang menghosting sesi latar belakang belum berjalan, `--bg` mungkin pertama kali mencetak `Starting background service…` di atas output ini. Ketika Anda memberikan `--name`, nama muncul setelah ID pendek:

```text theme={null}
backgrounded · 7c5dcf5d · flaky-test-fix
  claude agents             list sessions
  claude attach 7c5dcf5d    open in this terminal
  claude logs 7c5dcf5d      show recent output
  claude stop 7c5dcf5d      stop this session
```

<h4 id="run-a-shell-command">
  Jalankan perintah shell
</h4>

Untuk menjalankan perintah shell sebagai pekerjaan latar belakang alih-alih sesi Claude, ketik `!` sebagai karakter pertama dari input pengiriman tampilan agen. `!` ditampilkan sebagai awalan dan semua yang Anda ketik setelahnya adalah perintah. Contoh berikut mengirim `pytest -x` dari kotak input tampilan agen:

```text theme={null}
! pytest -x
```

Tekan `Enter` untuk memulai pekerjaan. Pekerjaan yang sama juga dapat diluncurkan langsung dari shell Anda dengan `--exec`:

```bash theme={null}
claude --bg --exec 'pytest -x'
```

Perintah berjalan sebagai pekerjaan yang didukung PTY dan muncul sebagai baris dalam tampilan agen, dengan baris output terbaru sebagai statusnya. Pekerjaan shell menjalankan perintah sebagai pengganti Claude, jadi tidak ada model yang dipanggil dan output tidak dikirim ke sesi apa pun.

Untuk melihat output, lampirkan ke baris, tekan `Space` untuk mengintip tanpa melampirkan, atau jalankan `claude logs <id>` dari shell Anda. Output yang ditangkap tetap berada dalam memori dan tidak ditulis ke disk. Baris dan outputnya dibersihkan secara otomatis sekitar lima menit setelah perintah keluar, jadi bacalah sebelum itu jika Anda memerlukan hasilnya.

<h3 id="how-file-edits-are-isolated">
  Bagaimana pengeditan file diisolasi
</h3>

Setiap sesi latar belakang, baik dimulai dari tampilan agen, `/bg`, atau `claude --bg`, dimulai di direktori kerja Anda. Sebelum mengedit file, Claude memindahkan sesi ke [git worktree](/docs/id/worktrees) yang terisolasi di bawah `.claude/worktrees/`, sehingga sesi paralel dapat membaca checkout yang sama tetapi masing-masing menulis ke miliknya sendiri.

Claude melewati worktree ketika:

* Sesi sudah berada di dalam linked git worktree, baik Claude membuatnya di bawah `.claude/worktrees/` atau Anda membuatnya dengan `git worktree add` di tempat lain
* Direktori kerja bukan repositori git dan tidak ada [`WorktreeCreate` hook](/docs/id/hooks#worktreecreate) yang dikonfigurasi
* Penulisan berada di luar direktori kerja

Untuk mematikan isolasi worktree untuk repositori tempat git worktree tidak praktis, atur [`worktree.bgIsolation`](/docs/id/settings#worktree-settings) ke `"none"`. Sesi latar belakang kemudian mengedit salinan kerja Anda secara langsung tanpa pindah ke worktree terlebih dahulu. Tambahkan pengaturan ke `.claude/settings.json` proyek:

```json theme={null}
{
  "worktree": {
    "bgIsolation": "none"
  }
}
```

Di luar repositori git, sesi menulis ke direktori kerja secara langsung dan tidak diisolasi satu sama lain, jadi hindari mengirim sesi paralel yang mengedit file yang sama. Jika Anda menggunakan sistem kontrol versi yang berbeda, konfigurasikan [`WorktreeCreate` hook](/docs/id/worktrees#non-git-version-control) dan Claude mengisolasi pengeditan dengan cara yang sama seperti yang dilakukannya untuk git.

Ketika hook gagal di direktori yang bukan repositori git, sesi melewati isolasi untuk direktori itu dan mengedit direktori kerja di tempat. Di dalam repositori git, penulisan tetap diblokir sampai sesi mengisolasi. Sebelum v2.1.203, sesi latar belakang dalam keadaan itu tidak dapat mengedit file apa pun: setiap penulisan ditolak sampai mengisolasi, dan hook tidak pernah dapat mengisolasi direktori itu.

Menghapus sesi menghapus atau menyimpan worktree yang Claude buat untuk sesi itu, tergantung pada cara Anda menghapusnya dan apa yang dipegang worktree:

* Menghapus dalam tampilan agen dengan `Ctrl+X` dua kali menghapus worktree, termasuk perubahan yang belum dikomit, jadi komit perubahan yang ingin Anda simpan terlebih dahulu.
* Menghapus dari shell dengan [`claude rm`](#manage-sessions-from-the-shell) menyimpan worktree yang memiliki perubahan yang belum dikomit, bersama dengan baris sesinya.
* Tidak ada jalur yang menghapus worktree dengan komit yang tidak didorong ke mana pun: worktree [disimpan bersama dengan sesinya](#organize-the-list) dan output menyebutkan jalur yang disimpan dan alasannya.
* Worktree yang Anda buat sendiri dan mulai sesi di dalamnya dibiarkan di tempat baik cara apa pun.

Untuk menemukan jalur worktree sesi, intip sesi atau lampirkan dan periksa direktori kerjanya.

Sebuah [subagent](/docs/id/sub-agents) yang sesi latar belakang spawn mewarisi direktori kerja sesi, jadi pengeditan filenya mendarat di worktree sesi daripada salinan kerja Anda. Untuk memberikan subagent worktree terpisah sendiri, atur [`isolation: worktree`](/docs/id/sub-agents#supported-frontmatter-fields) dalam frontmatter-nya atau berikan `isolation: "worktree"` saat spawn-nya.

Mulai dari v2.1.198, sesi latar belakang yang mengisolasi perubahan kodenya dalam worktree juga melakukan commit, mendorong cabangnya sendiri, dan membuka draft pull request tanpa berhenti untuk bertanya. Label [`#N`](#pull-request-status) muncul pada barisnya ketika pull request dibuka. Tidak pernah mendorong ke `main` atau `master`, tidak pernah force-push atau merge, dan melewati pull request ketika Anda mengatakan kepadanya untuk tidak membuka satu atau repositori tidak memiliki remote.

Sesi yang mengedit checkout yang tidak mengisolasi dirinya sendiri masih bertanya sebelum melakukan commit atau beralih cabang. Ini berlaku ketika isolasi diatur ke `"none"`, ketika pergerakan worktree gagal, atau ketika sesi dimulai di dalam worktree yang sudah ada.

<h3 id="set-the-model">
  Atur model
</h3>

Nama model yang ditampilkan di header tampilan agen adalah default pengiriman. Sesi baru yang Anda mulai dari input menggunakan model ini, yang berasal dari pengaturan [`model`](/docs/id/settings#available-settings) dalam pengaturan pengguna Anda. Atur dengan memilih model dalam pemilih [`/model`](/docs/id/model-config), atau edit pengaturan secara langsung.

Untuk menimpanya untuk seluruh sesi tampilan agen, berikan `--model` saat membuka tampilan agen. Lihat [Mode izin, model, dan upaya](#permission-mode-model-and-effort).

Untuk mengubah default pengiriman dari dalam tampilan agen, ketik `/model` diikuti dengan nama model dalam input pengiriman dan tekan `Enter`. Header diperbarui untuk menampilkan model itu dengan penanda `(session)`, dan sesi yang Anda kirim setelahnya menggunakannya. Ketik `/model default` untuk menghapus penimpaan dan kembali ke default pengiriman. Penimpaan ini berlangsung untuk sisa dari `claude agents` saat ini dan tidak menulis ke file pengaturan Anda. Contoh berikut mengirim satu sesi pada Opus dan yang berikutnya pada Sonnet:

```text theme={null}
/model opus
refactor auth
/model sonnet
run the test suite
```

Setiap sesi latar belakang dapat berjalan pada model berbeda. Untuk menimpanya untuk satu sesi:

* Dari shell, berikan `--model` dengan `claude --bg`.
* Lampirkan ke sesi yang berjalan dan jalankan `/model` untuk beralih: pilihan dari pemilih, atau `/model <name>` yang diketik, disimpan sebagai default Anda untuk sesi baru kecuali Anda menekan `s` dalam pemilih untuk beralih hanya sesi. Beralih hanya sesi bertahan jika sesi direspawn.
* Kirim [subagent](/docs/id/sub-agents) yang frontmatter-nya menetapkan bidang `model`.

<h3 id="permission-mode-model-and-effort">
  Mode izin, model, dan upaya
</h3>

Sesi latar belakang membaca [pengaturan](/docs/id/settings) dari direktori tempat sesi berjalan, sama seperti jika Anda telah memulai `claude` di sana. Ini mencakup nilai [`env`](/docs/id/settings#available-settings) dalam pengaturan proyek, jadi `ANTHROPIC_MODEL` atau variabel penyedia yang ditetapkan di sana berlaku untuk sesi latar belakang di direktori itu.

Pemilihan penyedia cloud, seperti `CLAUDE_CODE_USE_BEDROCK` atau `CLAUDE_CODE_USE_VERTEX`, dan alias `ANTHROPIC_DEFAULT_*_MODEL` mengikuti shell yang mengirim sesi. Jika Anda mengekspor penimpaan badan permintaan [`CLAUDE_CODE_EXTRA_BODY`](/docs/id/env-vars) dalam shell itu, itu mencapai sesi dengan cara yang sama. Sebelum v2.1.206, pekerja latar belakang mengabaikan `CLAUDE_CODE_EXTRA_BODY` yang diekspor shell.

Jika Anda mengekspor gateway `ANTHROPIC_BASE_URL` dalam shell pengiriman, itu mencapai sesi juga, bersama dengan `ANTHROPIC_CUSTOM_HEADERS`, ketika supervisor berjalan dengan lingkungan gateway yang sama dan sesi berjalan di direktori tempat Anda mengirim atau adalah sesi Anda sendiri yang dilepaskan ke latar belakang dengan `←` atau `/background`. Itu adalah kasus normal ketika shell pertama yang membuka tampilan agen atau mengirim sesi latar belakang adalah shell gateway. Pengiriman ke direktori berbeda dengan `@repo` atau `--cwd` tidak membawa gateway shell; pengaturan [settings](/docs/id/settings) proyek itu menyediakan endpoint. Lihat [proses supervisor](#the-supervisor-process) untuk cara sesi latar belakang bersumber pengaturan penyedia dan kredensial.

[Mode izin](/docs/id/permissions) tergantung pada cara Anda memulai sesi. Melepaskan sesi yang ada dengan `/bg` atau `←` mempertahankan mode izin saat ini, jadi sesi yang Anda alihkan ke `acceptEdits` atau `auto` tetap dalam mode itu setelah detach. Mengirim dari input tampilan agen atau menjalankan `claude --bg` dari shell Anda menggunakan `defaultMode` dari pengaturan direktori itu, atau `permissionMode` dari [frontmatter subagent](/docs/id/sub-agents#supported-frontmatter-fields) yang dikirim.

Mode izin, model, dan upaya yang sesi latar belakang dimulai dengan, bersama dengan [bendera konfigurasi yang dibawanya](#from-inside-a-session), semuanya bertahan ketika supervisor kemudian [menghentikan dan memulai ulang](#the-supervisor-process) prosesnya. Sesi yang Anda luncurkan dengan `claude --bg --dangerously-skip-permissions` atau `claude --bg --permission-mode bypassPermissions` tetap dalam `bypassPermissions` setelah restart itu alih-alih kembali ke `defaultMode` direktori, dan model atau upaya yang Anda ubah di tengah sesi dengan `/model` atau `/effort` disimpan.

Upaya yang sesi ambil dari pengaturan [`effortLevel`](/docs/id/settings#available-settings) daripada dari `--effort` atau `/effort` tidak diperbaiki saat pengiriman: setiap proses yang dimulai untuk sesi membaca pengaturan lagi, jadi mengedit `effortLevel` dalam `settings.json` mencapai sesi yang Anda lepaskan ke latar belakang dengan `←` atau `/bg` dan restart mereka yang lebih baru. Sebelum v2.1.203, melepaskan sesi mencatat upaya yang berasal dari pengaturan seolah-olah Anda telah melewatkan `--effort`, jadi pengeditan `effortLevel` yang lebih baru tidak pernah mencapainya.

Nama yang Anda atur dengan [`/rename`](/docs/id/commands) atau `Ctrl+R` juga bertahan di seluruh restart itu, jadi [`claude --resume <name>`](/docs/id/sessions#name-your-sessions) masih menyelesaikan sesi. Sebelum v2.1.202, restart mengembalikan sesi ke nama yang dikirim dengannya dan nama baru berhenti menyelesaikan.

Untuk menetapkan default untuk setiap sesi yang Anda kirim dari tampilan agen, berikan salah satu dari `--permission-mode`, `--model`, `--effort`, atau `--agent` saat membukanya:

```bash theme={null}
claude agents --permission-mode plan --model opus --effort high
```

`--agent` menetapkan [subagent](/docs/id/sub-agents) yang digunakan ketika prompt pengiriman tidak menyebutkan satu, baik dengan `@name` atau sebagai kata pertama. Ini default ke pengaturan [`agent`](/docs/id/settings#available-settings) jika satu diatur, jika tidak agen catch-all bawaan `claude`. Menyebutkan subagent dalam input pengiriman menimpa keduanya.

`claude agents` juga menerima `--dangerously-skip-permissions` sebagai singkatan untuk `--permission-mode bypassPermissions`, dan `--allow-dangerously-skip-permissions` untuk membuat `bypassPermissions` tersedia dalam siklus `Shift+Tab` setiap sesi yang dikirim tanpa memulai dalam mode itu. Keduanya cocok dengan [bendera CLI tingkat atas](/docs/id/cli-reference).

Default aktif muncul di footer di bawah input pengiriman.

Tanpa bendera ini, sesi menggunakan `defaultMode` dari pengaturan direktori itu atau `permissionMode` dari [frontmatter subagent](/docs/id/sub-agents#supported-frontmatter-fields) yang dikirim, dan model yang ditampilkan di header tampilan agen.

Menggunakan `bypassPermissions` dengan `claude --bg --permission-mode` ditolak sampai Anda telah menerima pengecualian bypass dengan menjalankan `claude --dangerously-skip-permissions` sekali secara interaktif, karena mode itu memungkinkan sesi yang tidak Anda tonton bertindak tanpa persetujuan. Melewatkan `--dangerously-skip-permissions` atau `--permission-mode bypassPermissions` ke `claude agents` menampilkan pengecualian yang sama ketika Anda belum menerimanya sebelumnya, dan menerima menerapkan `bypassPermissions` ke sesi yang Anda luncurkan dari tampilan. Melewatkan `--allow-dangerously-skip-permissions` menampilkan pengecualian yang sama juga, dan menerima membuat `bypassPermissions` tersedia dalam siklus `Shift+Tab` sesi tersebut tanpa memulai di dalamnya.

<h3 id="settings-plugins-and-mcp-servers">
  Pengaturan, plugins, dan server MCP
</h3>

Tampilan agen menerima bendera konfigurasi yang sama dengan `claude` untuk memuat pengaturan, plugins, server MCP, dan direktori tambahan. Setiap bendera berlaku untuk tampilan agen itu sendiri dan diteruskan ke setiap sesi yang Anda kirim darinya, jadi plugin atau server MCP yang Anda muat dengan cara ini tersedia di sesi tersebut juga.

| Bendera                                                                                          | Efek                                                                          |
| :----------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------- |
| [`--settings <file-or-json>`](/docs/id/settings)                                                      | Menimpa pengaturan untuk tampilan agen dan sesi yang dikirim                  |
| [`--add-dir <path>`](/docs/id/permissions#additional-directories-grant-file-access-not-configuration) | Memberikan akses file ke direktori tambahan                                   |
| [`--plugin-dir <path>`](/docs/id/plugins)                                                             | Memuat plugin dari direktori lokal                                            |
| [`--mcp-config <file-or-json>`](/docs/id/mcp)                                                         | Memuat server MCP dari file konfigurasi atau string JSON                      |
| `--strict-mcp-config`                                                                            | Gunakan hanya server MCP dari `--mcp-config`, abaikan konfigurasi MCP lainnya |

Ulangi `--add-dir`, `--plugin-dir`, atau `--mcp-config` sekali per nilai. Bentuk yang dipisahkan spasi, seperti `--add-dir a b c`, tidak didukung dengan `claude agents`.

Contoh berikut membuka tampilan agen dengan penimpaan pengaturan dan satu direktori tambahan:

```bash theme={null}
claude agents --settings ./ci-settings.json --add-dir ../shared-lib
```

<h2 id="manage-sessions-from-the-shell">
  Kelola sesi dari shell
</h2>

Setiap sesi latar belakang memiliki ID pendek yang dapat Anda gunakan dari shell. ID dicetak ketika Anda memulai sesi dengan `claude --bg`, dan ID setiap sesi adalah nama direktorinya di bawah `~/.claude/jobs/`. Perintah-perintah ini berguna untuk scripting atau ketika Anda tidak ingin membuka tampilan agen.

| Perintah                     | Tujuan                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| :--------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `claude agents`              | Buka tampilan agen                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `claude agents --cwd <path>` | Buka tampilan agen yang dibatasi pada sesi yang dimulai di bawah `<path>`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `claude agents --json`       | Cetak sesi aktif sebagai array JSON dan keluar: setiap sesi langsung, ditambah sesi latar belakang yang masih bekerja atau terblokir bahkan ketika prosesnya telah keluar. Tambahkan `--all` untuk juga menyertakan sesi latar belakang yang telah selesai. Setiap entri memiliki `cwd`, `kind`, dan `startedAt`. Entri latar belakang juga memiliki `id`, dapat digunakan dengan `claude attach`/`logs`/`stop`, dan `state`: salah satu dari `working`, `blocked`, `done`, `failed`, atau `stopped`. `pid` dan `status` hanya ada saat proses masih hidup, ditambah `waitingFor` ketika status adalah `waiting`, yang mengatakan apa sesi diblokir, seperti `permission prompt` atau `input needed`; `sessionId` dan `name` muncul ketika diatur. Entri interaktif yang tidak pernah Anda beri nama membawa nama default yang dibangun dari nama direktori kerja ditambah akhiran dua karakter, seperti `my-app-3f`. Gabungkan dengan `--cwd <path>` untuk memfilter |
| `claude attach <id>`         | Lampirkan ke sesi di terminal ini                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `claude logs <id>`           | Cetak output terbaru sesi                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `claude stop <id>`           | Hentikan sesi. Juga menerima `claude kill`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `claude respawn <id>`        | Mulai ulang sesi, baik yang sedang berjalan maupun yang dihentikan, dengan percakapannya tetap utuh, misalnya untuk mengambil biner Claude Code yang telah diperbarui                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `claude respawn --all`       | Mulai ulang setiap sesi yang sedang berjalan, misalnya untuk memindahkan semua sesi ke biner Claude Code yang telah diperbarui sekaligus                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `claude rm <id>`             | Hapus sesi dari daftar. Menghapus worktree yang dibuat Claude untuk sesi jika tidak ada perubahan yang belum di-commit dan tidak ada commit yang tidak didorong ke mana pun; jika tidak, sesi tetap disimpan, dan perintah mencetak jalur worktree dan alasannya sehingga Anda dapat menyelesaikannya dan menjalankan `claude rm` lagi. Membiarkan worktree yang Anda buat sendiri tetap ada. Transkrip percakapan tetap berada di mesin lokal Anda dan tetap tersedia melalui `claude --resume`                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `claude daemon status`       | Cetak status [supervisor](#the-supervisor-process), versi, direktori soket, dan jumlah pekerja                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `claude daemon stop --any`   | Hentikan proses supervisor dan sesi latar belakang yang dihosting. Lewatkan `--keep-workers` untuk membiarkan sesi latar belakang tetap berjalan sehingga supervisor berikutnya dapat terhubung kembali ke sesi tersebut. `claude agents` atau `claude --bg` berikutnya memulai supervisor baru                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |

<h2 id="how-background-sessions-are-hosted">
  Bagaimana sesi latar belakang dihosting
</h2>

Setiap sesi yang tercantum dalam tampilan agen dianggap sebagai sesi latar belakang, terlepas dari apakah Anda saat ini terhubung ke sesinya atau tidak. Sebaliknya, sesi yang dimulai dengan menjalankan `claude` secara langsung terikat pada terminal itu dan berakhir ketika terminal ditutup, kecuali Anda [mengirimnya ke latar belakang](#from-inside-a-session).

<h3 id="the-supervisor-process">
  Proses supervisor
</h3>

Sesi latar belakang dihosting oleh proses supervisor per-pengguna, terpisah dari terminal Anda dan dari tampilan agen. Supervisor dimulai secara otomatis pertama kali Anda mengirim sesi ke latar belakang atau membuka tampilan agen, dan Anda tidak mengelolanya secara langsung.

Ketika pembaruan telah mengganti atau menghapus biner dari mana proses Claude Code yang sedang berjalan diluncurkan, proses itu memulai supervisor dari salinan terinstal lainnya, seperti peluncur `claude` yang terinstal atau versi terbaru di disk.

Supervisor menjaga satu proses worker yang sudah dipanaskan sebelumnya siap sehingga pengiriman dari tampilan agen atau `claude --bg` dimulai tanpa penundaan peluncuran dingin. Ketika Anda mengirim, supervisor menugaskan worker yang sudah dipanaskan sebelumnya ke sesi Anda, menerapkan direktori, pengaturan, dan kredensial sesi itu ke dalamnya, dan kemudian memulai pengganti untuk pengiriman berikutnya. Jika tidak ada worker yang sudah dipanaskan sebelumnya yang sehat tersedia, supervisor meluncurkan proses segar sebagai gantinya.

Supervisor dan sesinya mengautentikasi dengan kredensial yang sama dengan sesi interaktif Anda dan tidak membuat koneksi jaringan tambahan di luar API model. Variabel pemilihan penyedia seperti `CLAUDE_CODE_USE_BEDROCK` dan alias `ANTHROPIC_DEFAULT_*_MODEL` dibaca dari shell yang mengirim setiap sesi dan diterapkan ke workernya.

PATH shell pengiriman diterapkan ke worker dengan cara yang sama, jadi perintah shell yang dijalankan sesi menemukan alat yang sama dengan yang ada di terminal Anda. Sebelum v2.1.203, sesi latar belakang menyimpan `PATH` dari shell yang pertama kali memulai supervisor, jadi alat yang ditambahkan ke `PATH` Anda sejak saat itu bisa hilang, paling sering di Windows.

Sesi latar belakang tidak mewarisi variabel titik akhir gateway seperti `ANTHROPIC_BASE_URL` atau variabel URL dasar yang setara untuk Amazon Bedrock, Google Cloud's Agent Platform, dan Microsoft Foundry dari shell yang memulai supervisor. Tanpa gateway yang diekspor di shell tempat Anda mengirim, sesi menggunakan kredensial yang disimpan Anda dan nilai `env` apa pun dalam [pengaturan](/docs/id/settings) direktori proyek. Untuk mengarahkan setiap sesi dalam proyek ke [gateway LLM](/docs/id/llm-gateway), atur `ANTHROPIC_BASE_URL` dalam blok `env` `settings.json` `.claude/` proyek itu.

Gateway `ANTHROPIC_BASE_URL` yang diekspor di shell tempat Anda mengirim mencapai worker sesi itu, bersama dengan `ANTHROPIC_CUSTOM_HEADERS` dan kredensial yang diekspor bersama dengannya, ketika supervisor dimulai dari lingkungan dengan gateway yang sama. Supervisor menangkap lingkungannya dari shell pertama yang membuka tampilan agen atau mengirim sesi latar belakang, jadi memulai dari shell gateway memberikannya lingkungan itu. Penerusan juga hanya berlaku untuk sesi yang dikirim ke direktori tempat Anda mengirim, atau dikirim ke latar belakang dari sesi Anda sendiri dengan `←` atau `/background`: mengirim ke direktori berbeda dengan `@repo` atau `--cwd` tidak membawa gateway shell, dan blok `env` `settings.json` proyek itu menyediakan titik akhir sebagai gantinya. Ketika lingkungan supervisor membawa gateway berbeda atau tidak ada, worker menyimpan kredensial yang disimpan Anda terhadap titik akhir default alih-alih mencampur kredensial satu lingkungan dengan titik akhir lingkungan lain. Sebelum v2.1.203, `ANTHROPIC_BASE_URL` shell pengiriman dijatuhkan sementara `ANTHROPIC_API_KEY` yang diekspor bersama dengannya disimpan, jadi kunci gateway dikirim ke titik akhir default dan setiap permintaan gagal dengan 401.

Titik akhir yang diteruskan hanya berlaku untuk proses aktif itu dan tidak pernah ditulis ke disk. Ketika supervisor menghentikan sesi idle dan kemudian memulainya kembali, proses yang dimulai ulang membaca titik akhirnya dari pengaturan Anda lagi: dengan `ANTHROPIC_AUTH_TOKEN` gateway itu kembali ke kredensial yang disimpan Anda, dan dengan `ANTHROPIC_API_KEY` yang dikeluarkan gateway itu dapat gagal untuk mengautentikasi sampai gateway diatur dalam pengaturan.

Setiap sesi latar belakang adalah proses Claude Code-nya sendiri, dikelola oleh supervisor daripada terikat pada terminal Anda. Sesi yang secara aktif bekerja, menunggu masukan Anda, atau memiliki terminal yang terpasang membuat prosesnya tetap berjalan. Perintah shell latar belakang yang berjalan, subagen, alur kerja dinamis, atau monitor dihitung sebagai pekerjaan aktif, jadi proses yang berjalan lama seperti server dev membuat sesi tetap hidup.

Setelah sesi selesai dan duduk tanpa lampiran selama sekitar satu jam, supervisor menghentikan prosesnya untuk membebaskan sumber daya. Sesi yang Anda [pin](#organize-the-list) dengan `Ctrl+T` dikecualikan dan membuat prosesnya tetap berjalan saat idle. Transkrip dan status tetap di disk, dan lain kali Anda melampirkan, mengintip, atau membalas sesi yang dihentikan, supervisor memulai proses segar dari tempat ia berhenti. Ketika setiap sesi selesai dan tidak ada terminal yang terhubung, supervisor itu sendiri keluar dan dimulai lagi lain kali Anda membutuhkannya.

Pekerjaan latar belakang yang sesi itu sendiri mulai di tingkat atas diserahkan ketika prosesnya dihentikan, dimulai ulang, atau diperbarui, termasuk di Windows. Proses berikutnya yang dimulai untuk sesi itu mengambilnya kembali:

* Perintah shell latar belakang yang selesai sementara itu dilaporkan sebagai selesai dengan outputnya
* Alur kerja dinamis dilanjutkan dari tempat ia berhenti
* [Subagen latar belakang](/docs/id/sub-agents#run-subagents-in-foreground-or-background) dilanjutkan dari transkrip mereka sendiri

Mulai dari v2.1.198 penyerahan mencakup ketiga-tiganya. Sebelum v2.1.198 itu hanya mencakup perintah shell dan alur kerja, jadi subagen latar belakang berhenti dengan proses dan dilaporkan sebagai gagal pada bangun berikutnya.

Pekerjaan yang statusnya hanya hidup di dalam proses itu sendiri berhenti dengannya alih-alih diserahkan. Itu adalah perintah shell yang dimulai subagen, yang dapat dimulai lagi oleh subagen yang dilanjutkan, dan [monitor](/docs/id/tools-reference#monitor-tool) yang sedang berjalan, yang aliran acaranya tidak dapat dipindahkan ke proses lain.

Menghapus sesi menghentikan semua yang diserahkannya. Untuk menghentikan semua pekerjaan latar belakang sesi dengan proses alih-alih menyerahkannya, atur variabel lingkungan [`CLAUDE_CODE_DISABLE_BG_EXIT_HANDOFF`](/docs/id/env-vars#variables) ke `1`.

Proses yang dimulai ulang menemukan percakapan sesi yang [pindah ke worktree](#how-file-edits-are-isolated) di tengah tugas: ketika transkrip tidak berada di tempat sesi dimulai, Claude Code juga mencari di bawah worktree terdaftar repositori. Sebelum v2.1.207, membuka kembali sesi itu dari tampilan agen setelah prosesnya berhenti dapat menampilkan percakapan kosong dengan hanya prompt aslinya, dengan transkrip masih utuh di disk; membuka sesi lagi pada v2.1.207 atau lebih baru memulihkannya.

Jika sesi yang dimulai ulang kembali menampilkan hanya prompt aslinya karena Claude Code salah membaca transkrip sebagai kosong, transkrip percakapan diganti nama dengan akhiran `.orphaned-` alih-alih dihapus, jadi tetap berada di mesin Anda.

Baris kosong yang tersisa dari menekan `←` yang tidak pernah diberi prompt dihapus sepenuhnya setelah sekitar lima menit sehingga daftar membersihkan dirinya sendiri. Sesi yang dimulai dengan `claude --bg` dan sesi yang menunggu prompt pengaturan seperti dialog kepercayaan tidak dihapus dengan cara ini.

Ketika host kekurangan memori, supervisor menghentikan sesi idle non-pin terlebih dahulu dan menghentikan sesi pin idle hanya jika itu tidak membebaskan apa pun.

Supervisor memantau biner Claude Code yang diinstal di disk dan memulai ulang ke versi baru setelah [auto-updater](/docs/id/setup#auto-updates) reguler menggantinya. Ini adalah pengawasan file lokal, bukan pemeriksaan jaringan. Sesi latar belakang adalah proses terlepas, jadi mereka terus berjalan melalui restart dan supervisor baru terhubung kembali ke mereka. Sesi pin idle juga dimulai ulang di tempat ke versi baru sehingga mengambil pembaruan tanpa Anda melampirkan kembali.

Setelah supervisor baru mengambil alih, itu juga memulai ulang sesi idle yang tersisa ke versi baru, beberapa sekaligus di latar belakang, setelah penundaan singkat yang memungkinkan terminal yang terpasang di seluruh restart untuk terhubung kembali terlebih dahulu. Sesi yang bekerja, menunggu masukan Anda, atau memiliki terminal yang terpasang tidak terganggu; itu berpindah ke versi baru lain kali prosesnya dimulai ulang. Sebelum v2.1.206, supervisor memindahkan hanya beberapa sesi idle per menit ke versi baru, jadi sesi dapat terus menjalankan yang lama selama beberapa waktu setelah pembaruan.

Restart ini hanya pernah memindahkan sesi ke versi yang lebih baru. Supervisor yang menjalankan versi Claude Code yang lebih lama daripada yang dimulai proses sesi dengan meninggalkan proses itu sendiri; sesi terus menjalankan versi yang lebih baru sampai supervisor yang lebih baru mengambil alih.

Menjalankan `claude attach` sementara supervisor memulai ulang sesi, baik untuk pembaruan, kemacetan, atau migrasi, menunggu proses pengganti alih-alih gagal. Baris status seperti `Agent is updating to the new Claude Code…` menamai apa yang ditunggu dan menghitung detik yang telah berlalu, dan perintah terhubung segera setelah sesi siap. Setelah sekitar 60 detik itu berhenti menunggu dan melaporkan kesalahan. Sebelum v2.1.205, `claude attach` berhenti mencoba ulang setelah beberapa detik dan mencetak kesalahan sementara sesi masih dimulai ulang.

<h3 id="where-state-is-stored">
  Tempat status disimpan
</h3>

Status sesi disimpan di bawah direktori konfigurasi Claude Code Anda. Jika Anda menetapkan [`CLAUDE_CONFIG_DIR`](/docs/id/env-vars), supervisor menggunakan direktori itu alih-alih `~/.claude` dan berjalan sebagai instans terpisah dengan sesinya sendiri.

| Jalur                            | Isi                                                                                         |
| :------------------------------- | :------------------------------------------------------------------------------------------ |
| `~/.claude/daemon.log`           | Log supervisor                                                                              |
| `~/.claude/daemon/roster.json`   | Daftar sesi latar belakang yang berjalan, digunakan untuk terhubung kembali setelah restart |
| `~/.claude/jobs/<id>/state.json` | Status per-sesi ditampilkan di tampilan agen                                                |
| `~/.claude/jobs/<id>/tmp/`       | Direktori awal per-sesi. Penulisan di sini tidak meminta izin. Dihapus ketika sesi dihapus  |

Setiap sesi latar belakang memiliki variabel lingkungan `CLAUDE_JOB_DIR` yang diatur ke direktori `~/.claude/jobs/<id>` nya, jadi perintah shell yang dijalankan sesi dapat menulis file sementara ke `$CLAUDE_JOB_DIR/tmp` tanpa bertabrakan dengan sesi paralel.

Untuk memeriksa status ini tanpa membaca file secara langsung, jalankan `claude daemon status`. Ini melaporkan apakah supervisor dapat dijangkau, ID proses dan versinya, direktori soket, dan berapa banyak sesi latar belakang yang aktif.

Perintah ini juga memperingatkan ketika supervisor yang berjalan berada pada versi yang berbeda dari `claude` yang Anda panggil, yang terjadi setelah pembaruan yang belum dimulai ulang oleh supervisor. Peringatan menunjukkan kedua versi dan memberi tahu Anda untuk menjalankan `claude daemon stop --any` untuk mengambil versi baru. Ketika Claude Code diinstal sebagai layanan OS, perintah yang disarankan adalah `claude daemon stop` tanpa flag.

Sesi bertahan melalui ketidaksesuaian versi itu utuh: versi Claude Code yang lebih lama yang memperbarui `state.json` sesi menyimpan bidang yang tidak dikenalinya dan membuat sesi tetap terdaftar. Daftar sesi dalam `roster.json` mengikuti aturan yang sama: versi yang lebih lama yang menulis ulangnya menyimpan bidang yang ditulis versi yang lebih baru, jadi sesi yang dimulai oleh versi yang lebih baru tetap dapat dijangkau dan terus menerima masukan setelah supervisor dimulai ulang. Sebelum v2.1.200, versi yang lebih lama dapat menghapus bidang tersebut saat menulis ulang.

Di Windows, `claude daemon status` menampilkan kesalahan file yang mendasar ketika file kunci pipa daemon terkunci atau tidak dapat dibaca alih-alih melaporkan kegagalan koneksi generik.

<h3 id="turn-off-agent-view">
  Matikan tampilan agen
</h3>

Untuk mematikan agen latar belakang dan tampilan agen sepenuhnya, atur pengaturan `disableAgentView` [setting](/docs/id/settings) ke `true` atau atur variabel lingkungan `CLAUDE_CODE_DISABLE_AGENT_VIEW`. Administrator dapat memberlakukan ini melalui [pengaturan terkelola](/docs/id/permissions#managed-settings).

<h2 id="troubleshooting">
  Troubleshooting
</h2>

<h3 id="claude-agents-lists-subagents-instead-of-opening-agent-view">
  `claude agents` menampilkan subagen alih-alih membuka tampilan agen
</h3>

Jika `claude agents` mencetak hitungan diikuti oleh subagen yang dikonfigurasi dan kemudian keluar, tampilan agen tidak tersedia di lingkungan Anda. Jalankan `claude update` untuk menginstal versi terbaru.

Jika tampilan agen masih tidak terbuka setelah memperbarui, periksa apakah telah [dimatikan](#turn-off-agent-view) oleh pengaturan atau variabel lingkungan.

<h3 id="agent-view-opens-with-no-sessions">
  Tampilan agen terbuka tanpa sesi
</h3>

Sebelum Anda mengirim sesi pertama Anda, tampilan agen menampilkan bagian header kosong dengan deskripsi di bawah masing-masing, ditambah penjelasan satu baris di atas input, sebagai pengganti daftar sesi. Ketik prompt dalam input di bagian bawah dan tekan `Enter` untuk mengirim sesi pertama Anda.

<h3 id="backgrounding-shows-a-background-this-session-dialog">
  Melepaskan menunjukkan dialog `Background this session?`
</h3>

Jika menekan `←` untuk melepaskan sesi saat ini menunjukkan dialog `Background this session?`, sesi memiliki pekerjaan yang sedang berlangsung yang tidak dapat berpindah ke sesi latar belakang, seperti [monitor](/docs/id/tools-reference#monitor-tool) yang sedang berjalan, dan Claude Code tidak akan diam-diam menghentikannya. Dialog menyebutkan pekerjaan yang akan dihentikan dan, secara terpisah, menghitung tugas yang dibawa. Jalankan `/tasks` untuk melihat semua yang sedang berjalan, kemudian konfirmasi untuk melepaskan atau pilih `Stay` untuk membiarkan pekerjaan selesai terlebih dahulu. Lihat [Dari dalam sesi](#from-inside-a-session) untuk jenis tugas mana yang dibawa dan mana yang dihentikan.

<h3 id="prompt-rejected-as-too-short">
  Prompt ditolak karena terlalu pendek
</h3>

Input pengiriman mengharapkan deskripsi tugas, bukan pembuka percakapan. Prompt yang lebih pendek dari empat karakter ditolak dengan petunjuk `Too short` sehingga keystroke yang tersesat tidak memulai sesi. Jelaskan apa yang ingin Anda lakukan sesi, seperti `investigate the flaky checkout test`.

<h3 id="sessions-show-as-failed-after-shutdown">
  Sesi menampilkan sebagai gagal setelah shutdown
</h3>

Mematikan atau memulai ulang mesin Anda menghentikan sesi latar belakang yang sedang berjalan, sehingga sesi tersebut menampilkan sebagai gagal saat Anda berikutnya membuka tampilan agen. Lampirkan, intip, atau balas sesi apa pun dan sesi itu dimulai ulang dari tempat ia berhenti.

Sleep saja tidak menyebabkan hal ini. Sesi dipertahankan di seluruh sleep dan supervisor terhubung kembali ke sesi tersebut saat bangun.

<h3 id="opening-a-session-says-the-conversation-is-already-open">
  Membuka sesi mengatakan percakapan sudah terbuka
</h3>

Membuka baris yang dihentikan yang percakapannya juga dipegang terbuka oleh proses Claude Code non-interaktif yang sedang berjalan lainnya, misalnya pekerja latar belakang untuk percakapan yang sama yang masih sedang berhenti, menampilkan `This conversation is already open in another running Claude session` alih-alih memulai proses baris, karena dua proses tidak dapat menulis ke transkrip yang sama. Balas dalam sesi yang sudah memiliki percakapan terbuka, atau keluar darinya dan buka baris lagi. Balasan yang Anda ketik dengan upaya yang ditolak tidak hilang; itu dikirim saat sesi berikutnya dimulai.

Sebelum v2.1.203, keadaan ini memulai proses kedua. Proses itu keluar dengan kesalahan `currently running as a background agent` dan baris menampilkan sebagai gagal.

<h3 id="a-session-fails-before-starting-with-a-possibly-low-memory-note">
  Sesi gagal sebelum dimulai dengan catatan `possibly low memory`
</h3>

Mulai dari v2.1.199, ketika proses sesi latar belakang keluar sebelum selesai dimulai dan host kekurangan memori, status baris menyebutkan keluar dan menambahkan `possibly low memory — free some up and retry`. Versi sebelumnya hanya menampilkan alasan keluar yang telanjang untuk kegagalan ini.

Catatan ini adalah hipotesis, bukan penyebab yang dikonfirmasi. Claude Code menambahkannya hanya ketika proses keluar diam-diam, tanpa menulis kesalahan dan tanpa dihentikan oleh sinyal, dan host melaporkan memori rendah pada saat itu. Ketika proses menulis kesalahan sebelum keluar, baris menampilkan kesalahan itu sebagai gantinya.

Bebaskan memori di mesin, kemudian lampirkan, intip, atau balas ke baris dan supervisor memulai proses segar untuk sesi. Ketika memori tetap rendah, supervisor juga [menghentikan sesi idle](#the-supervisor-process) untuk membebaskan sumber daya dengan sendirinya.

<h3 id="agent-view-says-the-background-service-did-not-respond">
  Tampilan agen mengatakan layanan latar belakang tidak merespons
</h3>

Jika melampirkan, mengintip, atau `claude logs` melaporkan bahwa layanan latar belakang tidak merespons, proses supervisor kemungkinan besar telah macet. Hentikan dan biarkan `claude agents` berikutnya memulai yang baru. Untuk menjaga sesi latar belakang Anda tetap berjalan melalui restart, berikan `--keep-workers`:

```bash theme={null}
claude daemon stop --any --keep-workers
```

Supervisor baru terhubung kembali ke sesi yang sedang berjalan. Tanpa `--keep-workers`, perintah mengakhiri sesi latar belakang juga. Bendera `--any` mengonfirmasi Anda ingin menghentikan supervisor yang dimulai sesuai permintaan daripada sebagai layanan yang diinstal, yang merupakan default.

Supervisor yang dimulai tetapi tidak dapat menerima koneksi keluar dan melepaskan kuncinya sendiri, jadi `claude agents` berikutnya memulai yang baru tanpa penghentian manual ini. Langkah-langkah di atas berlaku ketika supervisor yang sedang berjalan macet.

Di Windows, jika supervisor tidak merespons permintaan stop, perintah mencetak ID prosesnya. Akhiri proses itu dengan `taskkill /PID <pid>` untuk menyelesaikan pemulihan. Sesi latar belakang masih dipertahankan saat Anda memberikan `--keep-workers`.

<h3 id="dispatch-fails-with-could-not-resolve-authentication-method">
  Pengiriman gagal dengan `Could not resolve authentication method`
</h3>

Jika pengiriman latar belakang gagal dengan `Could not resolve authentication method` sementara sesi interaktif mengautentikasi secara normal, worker yang menerima pengiriman tidak mengambil kredensial. Supervisor menyediakan snapshot kredensial segar saat menugaskan [worker yang sudah dipanaskan sebelumnya](#the-supervisor-process), jadi kesalahan ini berarti tidak ada kredensial yang disimpan tersedia untuk proses supervisor itu sendiri. Konfirmasi Anda telah menjalankan `/login` atau mengonfigurasi kunci API, kemudian hentikan supervisor:

```bash theme={null}
claude daemon stop --any --keep-workers
```

`claude agents` atau `claude --bg` berikutnya memulai supervisor segar yang membaca kredensial yang disimpan Anda. Jika Anda mengautentikasi dengan variabel lingkungan seperti `ANTHROPIC_API_KEY` daripada `/login`, jalankan perintah berikutnya dari shell tempat variabel diatur.

Lihat [referensi kesalahan](/docs/id/errors#could-not-resolve-authentication-method) untuk daftar lengkap penyebab dan perbaikan.

<h3 id="background-sessions-can’t-read-desktop-documents-or-downloads-on-macos">
  Sesi latar belakang tidak dapat membaca Desktop, Documents, atau Downloads di macOS
</h3>

Di macOS, host sesi latar belakang berjalan sebagai prosesnya sendiri dan meminta akses ke folder yang dilindungi secara terpisah dari terminal Anda. Jika sesi latar belakang melaporkan `Operation not permitted` saat membaca `~/Desktop`, `~/Documents`, `~/Downloads`, atau lokasi yang dilindungi lainnya, berikan akses di System Settings di bawah Privacy & Security > Files and Folders, atau aktifkan Full Disk Access untuk entri tersebut.

Dengan installer asli, entri muncul sebagai Claude Code dan pemberian akses tetap ada di seluruh pembaruan. Dengan metode instalasi lain seperti Homebrew atau npm, entri menampilkan jalur biner dan mungkin perlu diberikan akses lagi setelah memperbarui.

<h3 id="background-sessions-can’t-reach-local-network-hosts-on-macos">
  Sesi latar belakang tidak dapat menjangkau host jaringan lokal di macOS
</h3>

Di macOS 15 dan yang lebih baru, sistem memblokir proses dari menjangkau perangkat di jaringan lokal Anda sampai Anda memberikan izin Local Network. Sebelum v2.1.198 host sesi latar belakang tidak pernah meminta izin itu, jadi perintah yang menargetkan alamat LAN gagal dengan `connect: no route to host` meskipun perintah yang sama berfungsi di terminal foreground. Mulai dari v2.1.198, perintah pertama dalam sesi latar belakang yang terhubung ke alamat jaringan lokal memicu prompt izin Local Network macOS untuk Claude Code. Berikan sekali dan perintah tersebut menjangkau host LAN dengan cara yang sama seperti di terminal foreground.

<h3 id="a-session-is-slow-to-respond-after-attaching">
  Sesi lambat merespons setelah melampirkan
</h3>

Setelah sesi selesai dan duduk tanpa lampiran selama sekitar satu jam, supervisor menghentikan prosesnya untuk membebaskan sumber daya. Melampirkan memulai proses segar dari tempat ia berhenti dan beralih ke sesi segera sementara proses dimulai ulang. Sesi yang bekerja, menunggu Anda, atau [disematkan](#organize-the-list) tidak dihentikan dengan cara ini, jadi semat sesi dengan `Ctrl+T` untuk menjaganya tetap responsif.

Saat proses dimulai, layar terakhir dari transkrip sesi ditampilkan dengan catatan `Session is starting` di bawahnya, dan sesi langsung menggantikannya segera setelah siap.

<h3 id="claude/worktrees/-is-filling-up">
  `.claude/worktrees/` penuh
</h3>

Menghapus sesi dalam tampilan agen menghapus worktree yang dibuat Claude untuk sesi tersebut, dan worktree yang tidak dapat dihapus dengan aman [menyimpan baris sesinya](#organize-the-list) sehingga tidak menjadi yatim piatu. `claude rm` menyimpan worktree yang memiliki perubahan yang belum dikomit, dan baris sesinya, dan mencetak jalur yang disimpan. Daftar entri sisa dengan `git worktree list` di direktori proyek dan hapus masing-masing dengan `git worktree remove <path>`. Lihat [Bersihkan worktrees](/docs/id/worktrees#clean-up-worktrees).

<h2 id="limitations">
  Keterbatasan
</h2>

Tampilan agen adalah pratinjau penelitian dengan keterbatasan berikut:

* **Batas laju berlaku**: sesi latar belakang menggunakan kuota langganan Anda sama seperti sesi interaktif, jadi menjalankan sepuluh agen secara paralel menggunakan kuota kira-kira sepuluh kali lebih cepat daripada menjalankan satu.
* **Sesi bersifat lokal**: sesi latar belakang berjalan di mesin Anda. Sesi ini dipertahankan di seluruh tidur tetapi berhenti jika mesin dimatikan.
* **Worktrees yang dibuat Claude dihapus dengan sesi di tampilan agen**: gabungkan perubahan sebelum menghapus sesi yang mengedit file di worktree-nya sendiri. Worktree dengan commit yang tidak didorong ke mana pun disimpan bersama sesi. `claude rm` juga menyimpan worktree yang memiliki perubahan yang belum dilakukan bersama sesinya, dan worktree yang Anda buat sendiri dibiarkan di tempat.

<h2 id="related-resources">
  Sumber daya terkait
</h2>

Untuk cara lain menjalankan Claude secara paralel, lihat:

* [Jalankan agen secara paralel](/docs/id/agents): bandingkan tampilan agen dengan subagents, tim agen, dan worktrees
* [Tim agen](/docs/id/agent-teams): koordinasikan beberapa sesi yang saling berpesan
* [Claude Code di web](/docs/id/claude-code-on-the-web): jalankan sesi di lingkungan cloud yang dikelola alih-alih secara lokal

<h2 id="version-history">
  Riwayat versi
</h2>

Tampilan agen telah berkembang dengan cepat selama pratinjau penelitian. Jika Anda berada di versi Claude Code yang lebih lama, beberapa perilaku di halaman ini mungkin berbeda; khususnya, `claude agents` menolak bendera yang belum didukungnya dengan kesalahan `unknown option`. Tabel di bawah mencantumkan kapan setiap bendera dan perilaku ditambahkan.

| Versi    | Perubahan                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| -------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| v2.1.208 | Melampirkan ke sesi yang prosesnya telah berhenti menampilkan layar terakhir transkripsnya sementara proses dimulai, alih-alih hanya catatan `Session is starting`. Balasan yang tidak dapat dikirimkan karena layanan latar belakang tidak dapat dijangkau atau pengiriman gagal disimpan dan dikirim sebagai prompt berikutnya dari sesi ketika prosesnya dimulai lagi; sebelum rilis ini, balasan yang hilang saat layanan latar belakang tidak dapat dijangkau dibuang. Proses yang binernya sendiri diganti oleh pembaruan masih dapat memulai supervisor, dari peluncur `claude` yang terinstal atau versi terbaru di disk, alih-alih gagal sampai Claude Code dimulai ulang. Supervisor yang menjalankan versi yang lebih lama tidak pernah memulai ulang sesi idle yang dimulai oleh versi yang lebih baru ke binernya sendiri yang lebih lama. Menghapus sesi menghapus worktree-nya bahkan setelah sesi memindahkan worktree ke cabang yang berbeda, dan menjaga worktree tetap bersama dengan baris sesi ketika worktree memiliki commit yang tidak didorong ke mana pun atau sesi lain mengklaimnya, alih-alih menghancurkan commit atau meninggalkan worktree. `/install-github-app` dan daftar pengaturan `/mcp` serta tindakan autentikasinya ditolak dalam sesi latar belakang dengan pesan yang menyebutkan alternatifnya; hanya dalam v2.1.208, pemilih `/model` ditolak dengan cara yang sama dan `/model <name>` yang diketik beralih hanya sesi itu alih-alih juga menyimpan model default Anda. |
| v2.1.207 | Panel intip terbuka dengan kalimat yang dipotong baris, seperti pertanyaan pastinya untuk sesi yang menunggu Anda, dan menampilkan berapa lama sesi yang diblokir telah menunggu sebagai baris `waiting 3m` tunggal alih-alih menambahkan awalan stempel waktu yang sama ke kalimat status dan pertanyaan. Menempel teks yang sama lagi dalam input pengiriman memperluas placeholder `[Pasted text #N]` yang runtuh alih-alih menambahkan yang kedua. Sesi latar belakang yang dinamai dengan menerima rencana menampilkan nama itu pada barisnya. Sesi latar belakang yang pindah ke worktree menyimpan percakapannya ketika prosesnya dimulai ulang dari tampilan agen.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| v2.1.206 | Ringkasan baris mengisi lebar sisa baris dan dipotong hanya di tepi kanan terminal alih-alih pada 64 kolom. Setelah supervisor dimulai ulang ke versi Claude Code baru, ia memulai ulang sesi latar belakang idle yang tersisa ke versi itu di latar belakang alih-alih beberapa per menit. Menghapus sesi dengan `Ctrl+X` atau `claude rm` juga menghapusnya dari daftar sesi supervisor, sehingga baris tidak lagi muncul kembali setelah restart supervisor.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| v2.1.205 | Ringkasan baris menampilkan laporan satu baris sesi itu sendiri, dipotong pada 64 kolom, alih-alih invokasi alat mentah atau hitungan `done/total`; baris yang dikelompokkan direktori terbuka dengan kata status berwarna. Panel intip terbuka dengan kalimat status lengkap dan, untuk sesi yang menunggu Anda, pertanyaan pastinya di atas input balasan. Sesi yang mengedit, mengomentari, menutup, atau menandai permintaan tarik siap dengan `gh` ditautkan ke sana, bukan hanya yang membuat atau checkout permintaan tarik, push menautkan permintaan tarik bahkan ketika nama cabang lokal tidak cocok, dan permintaan tarik yang output perintah pembuatannya melebihi batas inline juga ditautkan. Giliran tanpa teks yang dapat dibaca menyimpan status sesi sebelumnya alih-alih membaliknya kembali ke `Working`. `claude attach` menunggu hingga sekitar 60 detik untuk sesi yang sedang dimulai ulang, dengan baris status yang menyebutkan alasannya, alih-alih gagal.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| v2.1.203 | Gateway `ANTHROPIC_BASE_URL` yang diekspor dalam shell pengiriman mencapai sesi yang dikirim darinya ke direktori yang sama ketika supervisor membagikan gateway environment tersebut, alih-alih dijatuhkan sementara kunci API yang diekspor bersama dengannya tetap disimpan. `PATH` shell pengiriman diterapkan ke setiap worker sesi. Menekan `←` saat subagen sedang berjalan menunggu mereka alih-alih memulai ulang mereka setelah sepuluh detik. Daftar kosong selalu menampilkan header bagian dengan deskripsi di bawah masing-masing. Mengetik `@` dalam input pengiriman juga mencantumkan git worktrees terdaftar dari repositori peluncuran yang berada di dalam pohon direktorinya. Upaya yang diwarisi dari pengaturan `effortLevel` mengikuti pengeditan kemudian ke pengaturan tersebut alih-alih diperbaiki pada pengiriman. Membuka sesi yang dihentikan yang percakapannya sudah terbuka di sesi yang sedang berjalan lainnya ditolak dengan pesan alih-alih gagal pada baris. Perintah yang tidak tersedia dalam tampilan agen meninggalkan teks yang diketik dalam input. Hook `WorktreeCreate` yang gagal di luar repositori git tidak lagi memblokir sesi dari pengeditan file.                                                                                                                                                                                                                                                                                                              |
| v2.1.202 | Nama yang ditetapkan dengan `/rename` atau `Ctrl+R` pada sesi latar belakang tetap ada ketika supervisor menghentikan dan memulai ulang prosesnya, alih-alih kembali ke nama yang dikirim sesi dengan.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| v2.1.200 | Versi Claude Code yang lebih lama yang menulis ulang daftar sesi dalam `roster.json` mempertahankan bidang yang ditulis oleh versi yang lebih baru, sesuai dengan jaminan `state.json` yang ada, sehingga sesi yang dimulai oleh versi yang lebih baru terus menerima input setelah supervisor dimulai ulang. Ketika Anda membuka sesi yang telah berhenti merespons, supervisor memulai ulang prosesnya dan sesi melanjutkan respons yang terputus dari tempat ia berhenti.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| v2.1.199 | Sesi latar belakang yang prosesnya keluar sebelum selesai dimulai pada host dengan memori rendah menampilkan `possibly low memory — free some up and retry` dalam status barisnya alih-alih hanya alasan keluar yang polos. Melepaskan sesi dengan `←` atau `/background` membawa `/color` ke baris baru.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| v2.1.198 | Tampilan agen mengirim notifikasi melalui `preferredNotifChannel` ketika sesi latar belakang memerlukan input, selesai, atau gagal, dan memicu hook `Notification` dengan tipe `agent_needs_input` atau `agent_completed`. `←` dan `/exit` di dalam `claude attach <id>` kembali ke tampilan agen alih-alih keluar ke shell; `Ctrl+Z` kembali ke shell. Sesi latar belakang yang mengisolasi pekerjaannya dalam worktree melakukan commit, mendorong cabang terisolasinya sendiri, tidak pernah `main` atau `master`, dan membuka permintaan tarik draft ketika selesai alih-alih bertanya terlebih dahulu. `/login` berjalan dalam tampilan agen dan membuka dialog masuk. Dialog keluar `Background work is running` menawarkan `Move to background and exit`. Handoff keluar juga mencakup subagen latar belakang, yang melanjutkan dari transkrip mereka pada waktu bangun berikutnya alih-alih dilaporkan sebagai gagal. `claude --bg` dikombinasikan dengan `-p` atau `--print` ditolak dengan kesalahan.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| v2.1.196 | Tekan `←` tunggal mengirim sesi latar depan ke latar belakang; versi sebelumnya memerlukan dua tekan, dengan petunjuk footer dan konfirmasi. `--dangerously-skip-permissions` yang diteruskan ke `claude agents` menampilkan pengecualian bypass alih-alih dijatuhkan diam-diam. Sesi interaktif yang tidak pernah Anda beri nama membawa nama default seperti `my-app-3f` dalam daftar sesi dan `claude agents --json`. Perintah shell latar belakang dan alur kerja dinamis bertahan ketika proses sesi dihentikan, dimulai ulang, atau diperbarui, termasuk di Windows; atur `CLAUDE_CODE_DISABLE_BG_EXIT_HANDOFF=1` untuk mematikan handoff. Transkrip yang salah dibaca sebagai kosong saat restart diganti nama dengan akhiran `.orphaned-` alih-alih dihapus.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| v2.1.195 | Pekerjaan yang sedang berlangsung dibawa ketika Anda melepaskan sesi di Windows juga; atur `CLAUDE_DISABLE_ADOPT=1` untuk menghentikannya sebagai gantinya. Grup `Completed` mengisi ruang vertikal yang tersisa dan header dikompakkan di terminal pendek. Versi Claude Code yang lebih lama tidak lagi menjatuhkan bidang `state.json` sesi yang lebih baru atau menyembunyikan sesi tersebut dari `claude agents`. Melampirkan ke sesi yang dihentikan beralih segera alih-alih menampilkan layar kosong selama lima detik. Supervisor yang tidak dapat menerima koneksi keluar dan melepaskan kuncinya sendiri.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| v2.1.174 | Sesi latar belakang tidak lagi mewarisi variabel titik akhir gateway seperti `ANTHROPIC_BASE_URL` dari shell peluncuran supervisor; supervisor menyediakan snapshot kredensial segar ke worker yang sudah dipanaskan sebelumnya, memperbaiki kesalahan `Could not resolve authentication method` yang palsu.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| v2.1.172 | `/model` dalam input pengiriman menetapkan penimpaan model pengiriman yang dibatasi sesi.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| v2.1.161 | Ringkasan baris menampilkan hitungan `done/total` untuk item kerja paralel; panel intip menyebutkan item kerja paralel yang paling lama berjalan.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| v2.1.157 | `claude agents` menerima `--agent`; sesi yang dikirim menghormati pengaturan `agent`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| v2.1.145 | Dikte suara didukung dalam input balasan panel intip dan input pengiriman.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| v2.1.143 | Pengaturan `worktree.bgIsolation` ditambahkan; `claude agents` menerima `--allow-dangerously-skip-permissions`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| v2.1.142 | `claude agents` menerima `--permission-mode`, `--model`, `--effort`, `--dangerously-skip-permissions`, `--settings`, `--add-dir`, `--plugin-dir`, `--mcp-config`, dan `--strict-mcp-config`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| v2.1.141 | `claude agents` menerima `--cwd` untuk membatasi daftar ke satu proyek.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| v2.1.139 | Tampilan agen diperkenalkan sebagai pratinjau penelitian.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
