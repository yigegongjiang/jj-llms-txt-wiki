> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Apa yang baru

> Ringkasan mingguan fitur Claude Code yang penting, dengan cuplikan kode, demo, dan konteks tentang mengapa hal-hal ini penting.

Ringkasan dev mingguan menyoroti fitur yang paling mungkin mengubah cara Anda bekerja. Setiap entri mencakup kode yang dapat dijalankan, demo singkat, dan tautan ke dokumentasi lengkap. Untuk setiap perbaikan bug dan peningkatan kecil, lihat [changelog](/docs/en/changelog).

<Update label="Week 37" description="September 7–11, 2026" tags={["v2.1.263–v2.1.269"]}>
  **`claude plugin eval`**: jalankan plugin Anda terhadap serangkaian kasus uji, beri skor hasilnya, dan bandingkan dengan baseline tanpa plugin. `claude plugin eval init` membuat draf kasus dan grader untuk Anda.

  Juga minggu ini: keluarkan **Claude Code Desktop pane** apa pun ke jendela tersendiri dan doknya kembali nanti; pengaturan **`maxEffortLevel`** membatasi tingkat upaya pada setiap penyedia; dan halaman yang **WebFetch** belum selesai mengunduh dalam lima menit gagal alih-alih menggantung.

  [Baca ringkasan Week 37 →](/docs/id/whats-new/2026-w37)
</Update>

<Update label="Week 36" description="August 31 – September 4, 2026" tags={["v2.1.251–v2.1.261"]}>
  **Claude Fable 5.1**: tersedia di Claude Code dengan jendela konteks 1M-token.

  Juga minggu ini: di paket Pro dan Max, **computer use di aplikasi Desktop** bekerja di latar belakang di macOS saat Anda terus bekerja; dalam rendering fullscreen, **`/diff`** membuka panel langsung di samping percakapan yang menyegarkan saat Claude mengedit; dan **`/skill-doctor`** menunjukkan apa yang setiap skill Anda biayai dalam konteks dan seberapa sering digunakan.

  [Baca ringkasan Week 36 →](/docs/id/whats-new/2026-w36)
</Update>

<Update label="Week 35" description="August 24–28, 2026" tags={["v2.1.240–v2.1.250"]}>
  **Lanjutkan sesi terminal di aplikasi Desktop**: ketik `/resume` di kotak prompt Claude Code Desktop untuk melanjutkan sesi apa pun yang Anda mulai dari CLI, dengan percakapan lengkap dan konteks utuh.

  Juga minggu ini: **umpan balik yang ditulis Claude** membuat Claude menulis laporan umpan balik ketika sesuatu salah dalam sesi, yang Anda tinjau dan kirim dari `/feedback`; **`--restricted`** memulai sesi tanpa alat menjalankan perintah atau pengaturan pengguna dan proyek Anda, untuk harness evaluasi pada mesin bersama; dan pengaturan **`modelPicker`** mengontrol model mana yang daftar pemilih `/model`.

  [Baca ringkasan Week 35 →](/docs/id/whats-new/2026-w35)
</Update>

<Update label="Week 34" description="August 17–21, 2026" tags={["v2.1.234–v2.1.239"]}>
  **`/design`**: pratinjau penelitian yang membawa alur kerja artboard Claude Design ke dalam CLI dan Claude Code Desktop, dibangun di atas artifacts, sehingga Claude membuat artboard yang dapat diedit untuk UI Anda dan mengimplementasikan yang Anda pilih.

  Juga minggu ini: **Concise output style** bawaan membuat Claude memimpin dengan hasil dan melewati preamble; mesin apa pun yang menjalankan `claude remote-control` muncul sebagai **device card** di ponsel Anda sehingga Anda dapat memulai sesi di dalamnya dari tab Code; dan **`ANTHROPIC_DEFAULT_MODEL`** menetapkan model yang dimulai sesi baru.

  [Baca ringkasan Week 34 →](/docs/id/whats-new/2026-w34)
</Update>

<Update label="Week 33" description="August 10–14, 2026" tags={["v2.1.225–v2.1.233"]}>
  **Auto-continue setelah batas penggunaan di Desktop**: ketika Anda mencapai batas sesi Anda di Claude Code Desktop, periksa **Auto-continue when limits reset** pada kartu batas dan aplikasi mencoba kembali giliran yang terputus setelah batas direset.

  Juga minggu ini: **fork mode** aktif secara default dalam sesi interaktif, sehingga Claude dapat menyerahkan tugas sampingan kepada subagent yang mewarisi percakapan lengkap; URL permintaan penggabungan **GitLab** bekerja dengan `--worktree` dan tampilan `claude agents`, dan marketplace mengkloning URL `gitlab.com` bare; dan mengetik **`@`** dalam prompt menyebutkan sesi Claude lain berdasarkan nama.

  [Baca ringkasan Week 33 →](/docs/id/whats-new/2026-w33)
</Update>

<Update label="Week 32" description="August 3–7, 2026" tags={["v2.1.220–v2.1.224"]}>
  **Pesan lintas sesi**: di macOS dan Linux, sesi Claude Code Anda sekarang dapat saling berkirim pesan, sehingga Claude meneruskan temuan atau keputusan dari satu sesi ke sesi lain alih-alih Anda menjelaskannya kembali.

  Juga minggu ini: **lingkungan yang di-host sendiri** menjalankan sesi cloud Claude Code pada infrastruktur yang dioperasikan organisasi Anda, dalam beta publik di paket Team dan Enterprise; **auto mode** menjadi mode izin default untuk sesi baru di paket Pro, Max, dan Team mulai 14 Agustus; dan **ekstensi VS Code** mendapatkan tampilan Focus.

  [Baca ringkasan Week 32 →](/docs/id/whats-new/2026-w32)
</Update>

<Update label="Week 30" description="July 20–24, 2026" tags={["v2.1.214–v2.1.219"]}>
  **Claude Opus 5**: model Opus default baru di Claude Code, dengan jendela konteks 1M-token dan fast mode dengan harga \$10/\$50 per MTok.

  Juga minggu ini: **Claude Code Desktop** membuka panel iOS Simulator dalam beta publik sehingga Claude dapat menjalankan aplikasi Anda dan mengetuk melaluinya saat Anda menonton; **plugin Claude Security** menjalankan pemindaian kerentanan multi-agent dari basis kode Anda dan mengubah temuan yang Anda pilih menjadi patch yang Anda terapkan sendiri; dan **`/code-review`** berjalan sebagai subagent latar belakang.

  [Baca ringkasan Week 30 →](/docs/id/whats-new/2026-w30)
</Update>

<Update label="Week 29" description="July 13–17, 2026" tags={["v2.1.207–v2.1.212"]}>
  **Artifacts memanggil konektor MCP Anda**: artifact yang dipublikasikan dapat menarik data langsung dan mengambil tindakan melalui konektor MCP masing-masing penampil ketika mereka membuka halaman, dan minggu ini juga menambahkan tautan berbagi publik, peran editor di Team dan Enterprise, dan artifact yang dibuat dari sesi Claude Tag.

  Juga minggu ini: **screen reader mode** menggantikan antarmuka terminal visual dengan teks biasa dan linier untuk pembaca layar seperti VoiceOver dan NVDA; **`/fork`** menyalin percakapan Anda ke sesi latar belakang baru sementara Anda terus bekerja; dan **auto mode** tidak lagi memerlukan variabel opt-in di Amazon Bedrock, Platform Agent Google Cloud, dan Microsoft Foundry.

  [Baca ringkasan Week 29 →](/docs/id/whats-new/2026-w29)
</Update>

<Update label="Week 28" description="July 6–10, 2026" tags={["v2.1.202–v2.1.206"]}>
  **Browser in-app di Desktop**: Claude Code di desktop mendapatkan browser bawaan, sehingga Claude dapat membuka dokumen, desain, atau situs lain apa pun dan berinteraksi dengan halaman dengan cara yang sama seperti yang dilakukannya dengan pratinjau server dev lokal Anda.

  Juga minggu ini: **`/doctor`** adalah pemeriksaan pengaturan lengkap yang mendiagnosis masalah dan dapat memperbaikinya, dengan `/checkup` sebagai aliasnya; **auto mode** memblokir manipulasi transkrip dan meminta sebelum `rm -rf` pada variabel yang tidak terselesaikan; dan **baris tampilan agent** menunjukkan kata status berwarna dan headline yang ditulis pengklasifikasi.

  [Baca ringkasan Week 28 →](/docs/id/whats-new/2026-w28)
</Update>

<Update label="Week 27" description="June 29 – July 3, 2026" tags={["v2.1.195–v2.1.201"]}>
  **Claude Sonnet 5**: model default baru untuk kursi langganan Pro, Team Standard, dan Enterprise, dengan coding tingkat atas dan penggunaan tool dengan harga Sonnet, jendela konteks 1M-token asli, dan adaptive thinking diaktifkan secara default.

  Juga minggu ini: **Claude di Chrome** tersedia secara umum di semua paket Anthropic langsung; **subagent berjalan di latar belakang secara default** sehingga Claude terus bekerja saat mereka berjalan; **Claude Desktop di Linux** mendarat dalam beta di Ubuntu dan Debian; dan **`/radio`** menyetel ke Claude FM lo-fi radio.

  [Baca ringkasan Week 27 →](/docs/id/whats-new/2026-w27)
</Update>

<Update label="Week 26" description="June 22–26, 2026" tags={["v2.1.185–v2.1.193"]}>
  **`claude mcp login`**: autentikasi server MCP yang dikonfigurasi dari shell Anda alih-alih menu `/mcp` interaktif, dan hapus kredensial yang disimpannya nanti dengan `claude mcp logout`.

  Juga minggu ini: **shell mode merespons output perintah** (`! npm test` mendapat penjelasan tanpa prompt kedua); **`/rewind`** dapat melanjutkan percakapan dari sebelum `/clear` dijalankan; dan **subagent latar belakang** sekarang menampilkan prompt izin di sesi utama alih-alih auto-denying.

  [Baca ringkasan Week 26 →](/docs/id/whats-new/2026-w26)
</Update>

<Update label="Week 25" description="June 15–19, 2026" tags={["v2.1.178–v2.1.183"]}>
  **Artifacts**: ubah output sesi menjadi halaman langsung yang dapat dibagikan di claude.ai yang diperbarui di tempat saat sesi bekerja, sekarang dalam beta di paket Team dan Enterprise.

  Juga minggu ini: **aturan deny dan ask cocok dengan parameter tool** dengan `Tool(param:value)`, misalnya `Agent(model:opus)`; **`/config key=value`** menetapkan pengaturan apa pun dari prompt, dalam mode `-p`, dan dari Remote Control; dan **auto mode memblokir perintah git yang merusak** ketika Anda tidak meminta untuk membuang pekerjaan lokal.

  [Baca ringkasan Week 25 →](/docs/id/whats-new/2026-w25)
</Update>

<Update label="Week 24" description="June 8–12, 2026" tags={["v2.1.166–v2.1.176"]}>
  **`/cd`**: pindahkan sesi saat ini ke direktori kerja baru di tengah percakapan tanpa membangun kembali cache prompt.

  Juga minggu ini: **sub-agent dapat menelurkan sub-agent mereka sendiri** (rantai latar belakang dibatasi pada lima level dalam); **`--safe-mode`** memulai Claude Code dengan semua kustomisasi dinonaktifkan untuk pemecahan masalah; dan **`fallbackModel`** mengonfigurasi hingga tiga model fallback yang dicoba secara berurutan.

  [Baca ringkasan Week 24 →](/docs/id/whats-new/2026-w24)
</Update>

<Update label="Week 23" description="June 1–5, 2026" tags={["v2.1.158–v2.1.165"]}>
  **Auto mode di Amazon Bedrock, Google Cloud's Agent Platform, dan Microsoft Foundry**: auto mode sekarang tersedia di penyedia pihak ketiga untuk Opus 4.7 dan Opus 4.8, menggantikan prompt izin dengan pemeriksaan keamanan latar belakang.

  Juga minggu ini: **pengeditan otomatis yang lebih aman** meminta persetujuan sebelum menulis file yang dapat menjalankan kode dalam mode `acceptEdits`; **`/plugin list`** mencetak plugin terinstal Anda secara inline; dan **persyaratan versi** memungkinkan penerapan terkelola untuk memerlukan rentang versi Claude Code yang disetujui.

  [Baca ringkasan Week 23 →](/docs/id/whats-new/2026-w23)
</Update>

<Update label="Week 22" description="May 25–29, 2026" tags={["v2.1.150–v2.1.157"]}>
  **Claude Opus 4.8**: model default baru untuk Max, Team Premium, Enterprise pay-as-you-go, dan akun Anthropic API, dengan upaya tinggi secara default dan `/effort xhigh` untuk tugas-tugas tersulit.

  Juga minggu ini: **dynamic workflows** mengorkestrasi puluhan hingga ratusan subagent dari skrip yang ditulis Claude; **security-guidance plugin** meninjau perubahan Claude untuk kerentanan saat bekerja; dan **fast mode** berjalan di Opus 4.8 dengan harga \$10/\$50 per MTok.

  [Baca ringkasan Week 22 →](/docs/id/whats-new/2026-w22)
</Update>

<Update label="Week 21" description="May 18–22, 2026" tags={["v2.1.143–v2.1.149"]}>
  **Auto mode di paket Pro**: auto mode sekarang berjalan di akun Pro dan mendukung Sonnet 4.6 bersama Opus, menggantikan prompt izin dengan pemeriksaan keamanan latar belakang.

  Juga minggu ini: **`/usage`** memecah apa yang mendorong batas paket Anda berdasarkan skill, subagent, plugin, dan server MCP; perintah **`/code-review`** baru melaporkan bug kebenaran; dan **background sessions** muncul di `/resume` dan tetap hidup saat disematkan.

  [Baca ringkasan Week 21 →](/docs/id/whats-new/2026-w21)
</Update>

<Update label="Week 20" description="May 11–15, 2026" tags={["v2.1.139–v2.1.142"]}>
  **Tampilan agent**: `claude agents` membuka satu layar untuk setiap sesi Claude Code, menunjukkan apa yang sedang berjalan, apa yang menunggu Anda, dan apa yang sudah selesai.

  Juga minggu ini: **`/goal`** membuat Claude terus bekerja di seluruh giliran sampai kondisi penyelesaian terpenuhi; **fast mode** sekarang berjalan di Opus 4.7 secara default; dan **menu Rewind** dapat mengompresi konteks sebelumnya dengan "Summarize up to here".

  [Baca ringkasan Week 20 →](/docs/id/whats-new/2026-w20)
</Update>

<Update label="Week 19" description="May 4–8, 2026" tags={["v2.1.128–v2.1.136"]}>
  **Plugin dimuat dari arsip `.zip` dan URL**: `--plugin-dir` sekarang menerima file `.zip`, dan `--plugin-url` mengambil arsip plugin untuk sesi saat ini.

  Juga minggu ini: **`worktree.baseRef`** memilih apakah worktree baru bercabang dari default jarak jauh atau `HEAD` lokal; **aturan hard deny mode otomatis** memblokir tindakan tanpa syarat terlepas dari pengecualian izin; dan **hooks melihat tingkat upaya aktif** melalui `effort.level` dan `$CLAUDE_EFFORT`.

  [Baca ringkasan Week 19 →](/docs/id/whats-new/2026-w19)
</Update>

<Update label="Week 18" description="April 27 – May 1, 2026" tags={["v2.1.120–v2.1.126"]}>
  **Windows tanpa Git Bash**: Git untuk Windows tidak lagi diperlukan, dan Claude Code menggunakan PowerShell sebagai alat shell ketika Bash tidak ada.

  Juga minggu ini: **`claude ultrareview`** membawa tinjauan kode cloud ke CI dan skrip; **`claude project purge`** membersihkan status lokal untuk proyek; dan menempel **URL PR ke `/resume`** menemukan sesi yang membuatnya.

  [Baca ringkasan Week 18 →](/docs/id/whats-new/2026-w18)
</Update>

<Update label="Week 17" description="April 20–24, 2026" tags={["v2.1.114–v2.1.119"]}>
  **`/ultrareview`** dibuka sebagai pratinjau penelitian publik: armada agen pemburu bug berjalan di cloud dan temuan kembali ke CLI atau Desktop Anda secara otomatis.

  Juga minggu ini: **session recap** menunjukkan kepada Anda apa yang terjadi saat terminal tidak fokus; **custom themes** memungkinkan Anda membangun dan mengirimkan palet warna dari `/theme` atau plugin; dan **Claude Code di web** mendapat desain ulang dengan sidebar sesi baru dan tata letak drag-and-drop.

  [Baca ringkasan Week 17 →](/docs/id/whats-new/2026-w17)
</Update>

<Update label="Week 16" description="April 13–17, 2026" tags={["v2.1.105–v2.1.113"]}>
  **Claude Opus 4.7** hadir sebagai default baru di Max dan Team Premium, dengan tingkat upaya `xhigh` baru yang merupakan pengaturan yang direkomendasikan untuk sebagian besar pekerjaan coding dan slider `/effort` interaktif untuk menyesuaikannya.

  Juga minggu ini: **Routines** di Claude Code di web menjalankan agen cloud templated dari jadwal, acara GitHub, atau panggilan API; **notifikasi push mobile** mengirim ping ke ponsel Anda ketika tugas panjang selesai atau Claude membutuhkan Anda; `/usage` menunjukkan apa yang mendorong batas Anda; dan CLI bergerak ke biner asli.

  [Baca ringkasan Week 16 →](/docs/id/whats-new/2026-w16)
</Update>

<Update label="Week 15" description="April 6–10, 2026" tags={["v2.1.92–v2.1.101"]}>
  **Ultraplan** memasuki pratinjau awal: buat rencana di cloud dari CLI Anda, tinjau dan beri komentar di editor web, kemudian jalankan secara jarak jauh atau tarik kembali secara lokal. Jalankan pertama kali sekarang secara otomatis membuat lingkungan cloud untuk Anda.

  Juga minggu ini: alat **Monitor** mengalirkan acara latar belakang ke dalam percakapan sehingga Claude dapat mengikuti log dan bereaksi secara langsung, `/loop` menyesuaikan diri sendiri saat Anda menghilangkan interval, `/team-onboarding` mengemas pengaturan Anda menjadi panduan yang dapat diputar ulang, dan `/autofix-pr` mengaktifkan perbaikan otomatis PR dari terminal Anda.

  [Baca ringkasan Week 15 →](/docs/id/whats-new/2026-w15)
</Update>

<Update label="Week 14" description="March 30 – April 3, 2026" tags={["v2.1.86–v2.1.91"]}>
  **Computer use** hadir ke CLI dalam pratinjau penelitian: Claude dapat membuka aplikasi asli, mengklik melalui UI, dan memverifikasi perubahan dari terminal Anda. Terbaik untuk menutup loop pada hal-hal yang hanya GUI yang dapat verifikasi.

  Juga minggu ini: pelajaran interaktif `/powerup`, rendering alt-screen bebas flicker, override ukuran hasil MCP per-tool hingga 500K, dan executable plugin di `PATH` alat Bash.

  [Baca ringkasan Week 14 →](/docs/id/whats-new/2026-w14)
</Update>

<Update label="Week 13" description="March 23–27, 2026" tags={["v2.1.83–v2.1.85"]}>
  **Auto mode** hadir dalam pratinjau penelitian: pengklasifikasi menangani prompt izin Anda sehingga tindakan aman berjalan tanpa gangguan dan yang berisiko diblokir. Jalan tengah antara menyetujui semuanya dan `--dangerously-skip-permissions`.

  Juga minggu ini: computer use di aplikasi Desktop, perbaikan otomatis PR di Web, pencarian transkrip dengan `/`, alat PowerShell asli untuk Windows, dan hook `if` bersyarat.

  [Baca ringkasan Week 13 →](/docs/id/whats-new/2026-w13)
</Update>
