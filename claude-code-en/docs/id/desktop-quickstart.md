> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Memulai dengan aplikasi desktop

> Instal Claude Code di desktop dan mulai sesi coding pertama Anda

Aplikasi desktop memberi Anda Claude Code dengan antarmuka grafis yang dirancang untuk menjalankan beberapa sesi berdampingan: sidebar untuk mengelola pekerjaan paralel, tata letak drag-and-drop dengan terminal terintegrasi dan editor file, tinjauan diff visual, pratinjau aplikasi langsung, pemantauan GitHub PR dengan penggabungan otomatis, dan tugas terjadwal. Tidak perlu terminal.

<CardGroup cols={3}>
  <Card title="Unduh untuk macOS" icon="apple" href="https://claude.ai/api/desktop/darwin/universal/dmg/latest/redirect?utm_source=claude_code&utm_medium=docs">
    Build universal untuk Intel dan Apple Silicon
  </Card>

  <Card title="Unduh untuk Windows" icon="windows" href="https://claude.ai/api/desktop/win32/x64/setup/latest/redirect?utm_source=claude_code&utm_medium=docs">
    Untuk prosesor x64
  </Card>

  <Card title="Dapatkan Claude untuk Linux (beta)" icon="linux" href="/docs/id/desktop-linux">
    apt atau .deb untuk Ubuntu dan Debian
  </Card>
</CardGroup>

Untuk Windows ARM64, unduh [penginstal ARM64](https://claude.ai/api/desktop/win32/arm64/setup/latest/redirect?utm_source=claude_code\&utm_medium=docs). Di Linux, instal dengan apt; lihat [Claude Desktop di Linux](/docs/id/desktop-linux).

<Note>
  Claude Code memerlukan [langganan Pro, Max, Team, atau Enterprise](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=desktop_quickstart_pricing).
</Note>

Halaman ini memandu Anda melalui instalasi aplikasi dan memulai sesi pertama Anda. Jika Anda sudah siap, lihat [Gunakan Claude Code Desktop](/docs/id/desktop) untuk referensi lengkap.

Aplikasi desktop memiliki tiga tab:

* **Chat**: Percakapan umum tanpa akses file, mirip dengan claude.ai.
* **Cowork**: Agen latar belakang otonom yang bekerja pada tugas di mesin virtual bersandbox dengan lingkungannya sendiri, berjalan secara independen saat Anda melakukan pekerjaan lain. Sesi Cowork on-device menjalankan VM di komputer Anda; sesi Cowork jarak jauh berjalan di VM yang dikelola Anthropic sebagai gantinya.
* **Code**: Asisten coding interaktif dengan akses langsung ke file lokal Anda. Tergantung pada mode izin, Anda menyetujui setiap perubahan saat Claude mengusulkannya atau meninjau perubahan setelah Claude membuatnya.

Chat dan Cowork tercakup dalam [Claude Help Center](https://support.claude.com/); menginstal dan menerapkan aplikasi desktop tercakup dalam [artikel dukungan Claude Desktop](https://support.claude.com/en/collections/16163169-claude-desktop). Halaman ini berfokus pada tab **Code**.

<h2 id="install">
  Instal
</h2>

<Steps>
  <Step title="Instal dan masuk">
    Di macOS dan Windows, unduh installer dari tautan di atas dan jalankan. Di Linux, ikuti langkah-langkah instalasi di [Claude Desktop di Linux](/docs/id/desktop-linux). Luncurkan Claude dari folder Aplikasi Anda di macOS, menu Start di Windows, atau peluncur aplikasi Anda di Linux, kemudian masuk dengan akun Anthropic Anda.
  </Step>

  <Step title="Buka tab Code">
    Klik tab **Code** di bagian atas tengah. Jika mengklik Code meminta Anda untuk upgrade, Anda perlu [berlangganan paket berbayar](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=desktop_quickstart_upgrade) terlebih dahulu. Jika meminta Anda untuk masuk online, selesaikan masuk dan mulai ulang aplikasi. Jika Anda melihat kesalahan 403, lihat [pemecahan masalah autentikasi](/docs/id/desktop#403-or-authentication-errors-in-the-code-tab).
  </Step>
</Steps>

Aplikasi desktop menyertakan Claude Code. Anda tidak perlu menginstal Node.js atau CLI secara terpisah. Untuk menggunakan `claude` dari terminal, instal CLI secara terpisah. Lihat [Memulai dengan CLI](/docs/id/quickstart).

<h2 id="start-your-first-session">
  Mulai sesi pertama Anda
</h2>

Dengan tab Code terbuka, pilih proyek dan berikan Claude sesuatu untuk dikerjakan.

<Steps>
  <Step title="Pilih lingkungan dan folder">
    Pilih **Local** untuk menjalankan Claude di mesin Anda menggunakan file Anda secara langsung. Klik **Select folder** dan pilih direktori proyek Anda.

    <Tip>
      Mulai dengan proyek kecil yang Anda kenal dengan baik. Ini adalah cara tercepat untuk melihat apa yang dapat dilakukan Claude Code.
    </Tip>

    Anda juga dapat memilih:

    * **Cloud**: Jalankan sesi di cloud yang terus berlanjut bahkan jika Anda menutup aplikasi. Lihat [Use Claude Code in the cloud](/docs/id/claude-code-on-the-web) untuk mengetahui cara kerja sesi cloud.
    * **SSH**: Terhubung ke mesin jarak jauh melalui SSH, seperti server Anda sendiri, cloud VM, atau dev container. Desktop secara otomatis menginstal Claude Code di mesin jarak jauh saat pertama kali Anda terhubung.
    * **WSL** (Windows): Jalankan sesi di dalam [distribusi WSL 2](/docs/id/desktop-wsl); Claude Code, tools, dan git berjalan di sisi Linux dengan path asli.
  </Step>

  <Step title="Pilih model">
    Pilih model dari dropdown di sebelah tombol kirim. Lihat [models](/docs/id/model-config#available-models) untuk perbandingan model yang tersedia. Anda dapat mengubah model nanti dari dropdown yang sama.
  </Step>

  <Step title="Beri tahu Claude apa yang harus dilakukan">
    Ketik apa yang ingin Anda lakukan Claude:

    * `Find a TODO comment and fix it`
    * `Add tests for the main function`
    * `Create a CLAUDE.md with instructions for this codebase`

    Sebuah [session](/docs/id/desktop#work-in-parallel-with-sessions) adalah percakapan dengan Claude tentang kode Anda. Setiap sesi melacak konteks dan perubahannya sendiri.
  </Step>

  <Step title="Tinjau dan terima perubahan">
    Apa yang terjadi selanjutnya tergantung pada [permission mode](/docs/id/desktop#choose-a-permission-mode) yang ditampilkan di selector di sebelah tombol kirim:

    * **Auto or Accept edits**: Claude menerapkan perubahan filenya, dan indikator seperti `+12 -1` muncul sehingga Anda dapat meninjau perubahan tersebut di tampilan diff
    * **Manual**: Claude mengusulkan setiap perubahan dan menunggu persetujuan Anda sebelum menerapkannya. File Anda tidak dimodifikasi sampai Anda menerima, dan jika Anda menolak perubahan, Claude menanyakan bagaimana Anda ingin melanjutkan

    Dalam mode Manual, Anda akan melihat:

    1. Sebuah [diff view](/docs/id/desktop#review-changes-with-diff-view) yang menunjukkan dengan tepat apa yang akan berubah di setiap file
    2. Tombol Accept/Reject untuk menyetujui atau menolak setiap perubahan
    3. Pembaruan real-time saat Claude menyelesaikan permintaan Anda
  </Step>
</Steps>

<h2 id="now-what">
  Apa selanjutnya?
</h2>

Anda telah membuat edit pertama Anda. Untuk referensi lengkap tentang semua yang dapat dilakukan Desktop, lihat [Use Claude Code Desktop](/docs/id/desktop). Berikut adalah beberapa hal yang dapat Anda coba selanjutnya.

**Interupsi dan arahkan.** Anda dapat mengarahkan ulang Claude kapan saja. Klik tombol stop untuk menghentikan segera, atau ketik koreksi dan tekan **Enter** untuk mengirimnya tanpa menghentikan tindakan yang sedang berjalan. Bagaimanapun, Anda tidak perlu menunggu hingga selesai atau memulai dari awal.

**Berikan Claude lebih banyak konteks.** Ketik `@filename` di kotak prompt untuk menarik file tertentu ke dalam percakapan, lampirkan gambar dan PDF menggunakan tombol lampiran, atau seret dan lepas file langsung ke prompt. Semakin banyak konteks yang dimiliki Claude, semakin baik hasilnya. Lihat [Add files and context](/docs/id/desktop#add-files-and-context-to-prompts).

**Gunakan skills untuk tugas yang dapat diulang.** Ketik `/` atau klik **+** → **Slash commands** untuk menjelajahi [built-in commands](/docs/id/commands), [custom skills](/docs/id/skills), dan plugin skills. Skills adalah prompt yang dapat digunakan kembali yang dapat Anda panggil kapan pun Anda membutuhkannya, seperti daftar periksa tinjauan kode atau langkah-langkah penerapan.

**Tinjau perubahan sebelum melakukan commit.** Setelah Claude mengedit file, indikator `+12 -1` muncul. Klik untuk membuka [diff view](/docs/id/desktop#review-changes-with-diff-view), tinjau modifikasi file demi file, dan beri komentar pada baris tertentu. Claude membaca komentar Anda dan merevisi. Klik **Review code** untuk membuat Claude mengevaluasi diff itu sendiri dan meninggalkan saran inline.

**Sesuaikan seberapa banyak kontrol yang Anda miliki.** [permission mode](/docs/id/desktop#choose-a-permission-mode) Anda menetapkan seberapa banyak yang dapat dilakukan Claude tanpa meminta persetujuan:

* **Auto**: pengklasifikasi meninjau tindakan di latar belakang dan memblokir yang berisiko alih-alih menanyakan Anda.
* **Manual**: Claude bertanya sebelum mengedit file atau menjalankan perintah.
* **Accept edits**: Claude secara otomatis menerima pengeditan file untuk iterasi yang lebih cepat.
* **Plan**: Claude mengusulkan pendekatan tanpa mengedit file apa pun, yang berguna sebelum refaktor besar.

**Tambahkan plugin untuk kemampuan lebih.** Klik tombol **+** di sebelah kotak prompt dan pilih **Plugins** untuk menjelajahi dan menginstal [plugins](/docs/id/desktop#install-plugins) yang menambahkan skills, agents, MCP servers, dan lainnya.

**Atur ruang kerja Anda.** Seret pane chat, diff, terminal, file, dan browser ke tata letak apa pun yang Anda inginkan. Buka terminal dengan **Ctrl+\`** untuk menjalankan perintah bersama sesi Anda, atau klik jalur file untuk membukanya di pane file. Lihat [Arrange your workspace](/docs/id/desktop#arrange-your-workspace).

**Pratinjau aplikasi Anda.** Ketika Anda menjalankan dev server Anda di desktop, aplikasi Anda terbuka di pane Browser, yang juga dapat [membuka situs eksternal](/docs/id/desktop#browse-external-sites). Claude dapat melihat aplikasi yang sedang berjalan, menguji endpoint, memeriksa log, dan melakukan iterasi pada apa yang dilihatnya. Lihat [Preview your app](/docs/id/desktop#preview-your-app).

**Lacak permintaan pull Anda.** Setelah membuka PR, Claude Code memantau hasil pemeriksaan CI dan dapat secara otomatis memperbaiki kegagalan atau menggabungkan PR setelah semua pemeriksaan lulus. Lihat [Monitor pull request status](/docs/id/desktop#monitor-pull-request-status).

**Letakkan Claude sesuai jadwal.** Atur [scheduled tasks](/docs/id/desktop-scheduled-tasks) untuk menjalankan Claude secara otomatis secara berulang: tinjauan kode harian setiap pagi, audit dependensi mingguan, atau briefing yang menarik dari alat yang terhubung Anda.

**Skalakan ketika Anda siap.** Buka [parallel sessions](/docs/id/desktop#work-in-parallel-with-sessions) dari sidebar untuk mengerjakan beberapa tugas sekaligus, masing-masing di worktree Git-nya sendiri, dan buka [tasks pane](/docs/id/desktop#watch-background-tasks) untuk menonton subagents dan perintah latar belakang yang sedang dijalankan sesi. Buka [side chat](/docs/id/desktop#ask-a-side-question-without-derailing-the-session) untuk mengajukan pertanyaan tanpa mengganggu thread utama. Kirim [long-running work to the cloud](/docs/id/desktop#run-long-running-tasks-in-the-cloud) sehingga terus berjalan bahkan jika Anda menutup aplikasi, atau [continue a session on the web or in your IDE](/docs/id/desktop#continue-in-another-surface) jika tugas memakan waktu lebih lama dari yang diharapkan. [Connect external tools](/docs/id/desktop#extend-claude-code) seperti GitHub, Slack, dan Linear untuk menyatukan alur kerja Anda.

<h2 id="what’s-next">
  Apa selanjutnya
</h2>

* [Gunakan Claude Code Desktop](/docs/id/desktop): mode izin, sesi paralel, tampilan diff, konektor, dan konfigurasi enterprise
* [Datang dari CLI?](/docs/id/desktop#coming-from-the-cli): jalankan Desktop dan CLI pada proyek yang sama, dan bandingkan fitur, setara flag, dan apa yang tidak tersedia di Desktop
* [Troubleshooting](/docs/id/desktop#troubleshooting): solusi untuk kesalahan umum dan masalah setup
* [Best practices](/docs/id/best-practices): tips untuk menulis prompt yang efektif dan mendapatkan hasil maksimal dari Claude Code
* [Common workflows](/docs/id/common-workflows): tutorial untuk debugging, refactoring, testing, dan lainnya
