> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Apa yang baru

> Ringkasan mingguan fitur Claude Code yang penting, dengan cuplikan kode, demo, dan konteks tentang mengapa hal-hal ini penting.

Ringkasan dev mingguan menyoroti fitur yang paling mungkin mengubah cara Anda bekerja. Setiap entri mencakup kode yang dapat dijalankan, demo singkat, dan tautan ke dokumentasi lengkap. Untuk setiap perbaikan bug dan peningkatan kecil, lihat [changelog](/docs/id/changelog).

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
