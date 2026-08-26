> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Mulai dengan Claude Code di web

> Jalankan Claude Code di cloud dari browser atau ponsel Anda. Hubungkan repositori GitHub, kirimkan tugas, dan tinjau PR tanpa setup lokal.

<Note>
  Claude Code di web sedang dalam pratinjau penelitian untuk pengguna Pro, Max, dan Team, serta untuk pengguna Enterprise dengan kursi premium atau kursi Chat + Claude Code.
</Note>

Claude Code di web berjalan pada infrastruktur cloud yang dikelola Anthropic alih-alih mesin Anda. Kirimkan tugas dari [claude.ai/code](https://claude.ai/code) di browser Anda atau aplikasi mobile Claude.

Anda memerlukan repositori GitHub untuk [memulai](#connect-github-and-create-an-environment). Claude mengklonnya ke mesin virtual yang terisolasi, membuat perubahan, dan mendorong cabang untuk Anda tinjau. Sesi bertahan di seluruh perangkat, jadi tugas yang Anda mulai di laptop siap ditinjau dari ponsel Anda nanti.

Claude Code di web bekerja dengan baik untuk:

* **Tugas paralel**: jalankan beberapa tugas independen sekaligus, masing-masing dalam sesi dan cabangnya sendiri, tanpa mengelola beberapa worktrees
* **Repo yang tidak Anda miliki secara lokal**: Claude mengklonkan repo segar setiap sesi, jadi Anda tidak perlu memeriksanya
* **Tugas yang tidak memerlukan pengarahan sering**: kirimkan tugas yang terdefinisi dengan baik, lakukan sesuatu yang lain, dan tinjau hasilnya ketika Claude selesai
* **Pertanyaan kode dan eksplorasi**: pahami basis kode atau lacak bagaimana fitur diimplementasikan tanpa checkout lokal

Untuk pekerjaan yang memerlukan konfigurasi lokal, alat, atau lingkungan Anda, menjalankan Claude Code secara lokal atau menggunakan [Remote Control](/docs/id/remote-control) adalah pilihan yang lebih baik.

<h2 id="how-sessions-run">
  Bagaimana sesi berjalan
</h2>

Ketika Anda mengirimkan tugas:

1. **Klonkan dan persiapkan**: repositori Anda diklonkan ke VM yang dikelola Anthropic, dan [skrip setup](/docs/id/claude-code-on-the-web#setup-scripts) Anda berjalan jika dikonfigurasi.
2. **Konfigurasi jaringan**: akses internet diatur berdasarkan [tingkat akses](/docs/id/claude-code-on-the-web#access-levels) lingkungan Anda.
3. **Bekerja**: Claude menganalisis kode, membuat perubahan, menjalankan tes, dan memeriksa pekerjaannya. Anda dapat menonton dan mengarahkan sepanjang waktu, atau pergi dan kembali ketika selesai.
4. **Dorong cabang**: ketika Claude mencapai titik pemberhentian, ia mendorong cabangnya ke GitHub. Anda meninjau diff, meninggalkan komentar inline, membuat PR, atau mengirim pesan lain untuk melanjutkan.

Sesi tidak ditutup ketika cabang didorong. Pembuatan PR dan pengeditan lebih lanjut semuanya terjadi dalam percakapan yang sama.

<h2 id="compare-ways-to-run-claude-code">
  Bandingkan cara menjalankan Claude Code
</h2>

Claude Code berperilaku sama di mana pun. Yang berubah adalah tempat kode dieksekusi dan apakah konfigurasi lokal Anda tersedia. Aplikasi Desktop menawarkan sesi lokal dan cloud, jadi jawaban di bawah ini tergantung pada yang Anda pilih:

|                                        | Di web                                                                                                            | Remote Control                      | Terminal CLI        | Aplikasi Desktop                  |
| :------------------------------------- | :---------------------------------------------------------------------------------------------------------------- | :---------------------------------- | :------------------ | :-------------------------------- |
| **Kode berjalan di**                   | VM cloud Anthropic                                                                                                | Mesin Anda                          | Mesin Anda          | Mesin Anda atau VM cloud          |
| **Anda chat dari**                     | claude.ai atau aplikasi mobile                                                                                    | claude.ai atau aplikasi mobile      | Terminal Anda       | UI Desktop                        |
| **Menggunakan konfigurasi lokal Anda** | Tidak, hanya repo                                                                                                 | Ya                                  | Ya                  | Ya untuk lokal, tidak untuk cloud |
| **Memerlukan GitHub**                  | Ya, atau [bundel repo lokal](/docs/id/claude-code-on-the-web#send-local-repositories-without-github) melalui `--cloud` | Tidak                               | Tidak               | Hanya untuk sesi cloud            |
| **Terus berjalan jika Anda terputus**  | Ya                                                                                                                | Selama terminal tetap terbuka       | Tidak               | Tergantung jenis sesi             |
| **[Mode izin](/docs/id/permission-modes)**  | Terima otomatis editan, Plan                                                                                      | Tanya, Terima otomatis editan, Plan | Semua mode          | Tergantung jenis sesi             |
| **Akses jaringan**                     | Dapat dikonfigurasi per lingkungan                                                                                | Jaringan mesin Anda                 | Jaringan mesin Anda | Tergantung jenis sesi             |

Lihat dokumentasi [terminal quickstart](/docs/id/quickstart), [Aplikasi Desktop](/docs/id/desktop), atau [Remote Control](/docs/id/remote-control) untuk mengaturnya.

<h2 id="connect-github-and-create-an-environment">
  Hubungkan GitHub dan buat lingkungan
</h2>

Setup adalah proses satu kali. Jika Anda sudah menggunakan GitHub CLI, Anda dapat [melakukan ini dari terminal Anda](#connect-from-your-terminal) alih-alih browser.

<Steps>
  <Step title="Kunjungi claude.ai/code">
    Buka [claude.ai/code](https://claude.ai/code) dan masuk dengan akun Anthropic Anda.
  </Step>

  <Step title="Instal Aplikasi Claude GitHub">
    Setelah masuk, claude.ai/code meminta Anda untuk menghubungkan GitHub. Ikuti prompt untuk menginstal Aplikasi Claude GitHub dan memberikan akses ke repositori Anda. Sesi cloud bekerja dengan repositori GitHub yang ada, jadi untuk memulai proyek baru, [buat repositori kosong di GitHub](https://github.com/new) terlebih dahulu.
  </Step>

  <Step title="Buat lingkungan Anda">
    Setelah menghubungkan GitHub, Anda akan diminta untuk membuat lingkungan cloud. Lingkungan mengontrol akses jaringan apa yang dimiliki Claude selama sesi dan apa yang berjalan ketika sesi baru dibuat. Lihat [Alat yang Diinstal](/docs/id/claude-code-on-the-web#installed-tools) untuk apa yang tersedia tanpa konfigurasi apa pun.

    Formulir memiliki bidang-bidang ini:

    * **Nama**: label tampilan. Berguna ketika Anda memiliki beberapa lingkungan untuk proyek atau tingkat akses yang berbeda.
    * **Akses jaringan**: mengontrol apa yang dapat dijangkau sesi di internet. Default, `Trusted`, memungkinkan koneksi ke [registri paket umum](/docs/id/claude-code-on-the-web#default-allowed-domains) seperti npm, PyPI, dan RubyGems sambil memblokir akses internet umum.
    * **Variabel lingkungan**: variabel opsional yang tersedia di setiap sesi, dalam format `.env`. Jangan bungkus nilai dalam tanda kutip, karena tanda kutip disimpan sebagai bagian dari nilai. Ini terlihat oleh siapa pun yang dapat mengedit lingkungan ini.
    * **Skrip setup**: skrip Bash opsional yang berjalan sebelum Claude Code diluncurkan. Gunakan untuk menginstal alat sistem yang tidak disertakan VM cloud, seperti `apt install -y gh`. Hasilnya [di-cache](/docs/id/claude-code-on-the-web#environment-caching), jadi skrip tidak berjalan ulang di setiap sesi. Lihat [Skrip setup](/docs/id/claude-code-on-the-web#setup-scripts) untuk contoh dan tips debugging.

    Untuk proyek pertama, biarkan default dan klik **Buat lingkungan**. Anda dapat [mengeditnya nanti atau membuat lingkungan tambahan](/docs/id/claude-code-on-the-web#configure-your-environment) untuk proyek yang berbeda.
  </Step>
</Steps>

<h3 id="connect-from-your-terminal">
  Hubungkan dari terminal Anda
</h3>

Jika Anda sudah menggunakan GitHub CLI (`gh`), Anda dapat mengatur Claude Code di web tanpa membuka browser. Ini memerlukan [Claude Code CLI](/docs/id/quickstart). `/web-setup` membaca token `gh` lokal Anda, menautkannya ke akun Claude Anda, dan membuat lingkungan cloud default jika Anda tidak memilikinya.

<Note>
  Organisasi dengan [Zero Data Retention](/docs/id/zero-data-retention) yang diaktifkan tidak dapat menggunakan `/web-setup` atau fitur sesi cloud lainnya. Jika GitHub CLI tidak diinstal atau diautentikasi, `/web-setup` membuka alur onboarding browser sebagai gantinya.
</Note>

<Steps>
  <Step title="Autentikasi dengan GitHub CLI">
    Di shell Anda, autentikasi GitHub CLI jika Anda belum melakukannya:

    ```bash theme={null}
    gh auth login
    ```
  </Step>

  <Step title="Masuk ke Claude">
    Di Claude Code CLI, jalankan `/login` untuk masuk dengan akun claude.ai Anda. Lewati langkah ini jika Anda sudah masuk.
  </Step>

  <Step title="Jalankan /web-setup">
    Di Claude Code CLI, jalankan:

    ```text theme={null}
    /web-setup
    ```

    Ini menyinkronkan token `gh` Anda ke akun Claude Anda. Jika Anda belum memiliki lingkungan cloud, `/web-setup` membuat satu dengan akses jaringan Trusted dan tanpa skrip setup. Anda dapat [mengedit lingkungan atau menambahkan variabel](/docs/id/claude-code-on-the-web#configure-your-environment) setelahnya. Setelah `/web-setup` selesai, Anda dapat memulai sesi cloud dari terminal Anda dengan [`--cloud`](/docs/id/claude-code-on-the-web#from-terminal-to-web) atau mengatur tugas berulang dengan [`/schedule`](/docs/id/routines).
  </Step>
</Steps>

<h2 id="start-a-task">
  Mulai tugas
</h2>

Dengan GitHub terhubung dan lingkungan dibuat, Anda siap mengirimkan tugas.

<Steps>
  <Step title="Pilih repositori dan cabang">
    Dari [claude.ai/code](https://claude.ai/code) atau tab Code di aplikasi mobile Claude, klik pemilih repositori di bawah kotak input dan pilih repositori untuk Claude bekerja. Setiap repositori menampilkan pemilih cabang. Ubahnya untuk memulai Claude dari cabang fitur alih-alih default. Anda dapat menambahkan beberapa repositori untuk bekerja di seluruhnya dalam satu sesi.
  </Step>

  <Step title="Pilih mode izin">
    Dropdown mode di sebelah input default ke **Terima otomatis editan**, di mana Claude membuat perubahan dan mendorong cabang tanpa berhenti untuk persetujuan. Beralih ke **Plan mode** jika Anda ingin Claude mengusulkan pendekatan dan menunggu persetujuan Anda sebelum mengedit file. Sesi cloud tidak menawarkan izin Ask atau izin Bypass. Lihat [daftar lengkap mode izin](/docs/id/permission-modes#available-modes) untuk apa yang masing-masing izinkan.
  </Step>

  <Step title="Jelaskan tugas dan kirimkan">
    Ketik deskripsi apa yang Anda inginkan dan tekan Enter. Jadilah spesifik:

    * Beri nama file atau fungsi: "Tambahkan README dengan instruksi setup" atau "Perbaiki tes auth yang gagal di `tests/test_auth.py`" lebih baik daripada "perbaiki tes"
    * Tempel output kesalahan jika Anda memilikinya
    * Jelaskan perilaku yang diharapkan, bukan hanya gejala

    Claude mengklonkan repositori, menjalankan skrip setup Anda jika dikonfigurasi, dan mulai bekerja. Setiap tugas mendapatkan sesi sendiri dan cabangnya sendiri, jadi Anda tidak perlu menunggu satu selesai sebelum memulai yang lain.
  </Step>
</Steps>

<h2 id="pre-fill-sessions">
  Isi sebelumnya sesi
</h2>

Anda dapat mengisi sebelumnya prompt, repositori, dan lingkungan untuk sesi baru dengan menambahkan parameter query ke URL [claude.ai/code](https://claude.ai/code). Gunakan ini untuk membangun integrasi seperti tombol di pelacak masalah Anda yang membuka Claude Code dengan deskripsi masalah sebagai prompt.

| Parameter      | Deskripsi                                                                                                                                                                                          |
| :------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `prompt`       | Teks prompt untuk diisi sebelumnya di kotak input. Alias `q` juga diterima.                                                                                                                        |
| `prompt_url`   | URL untuk mengambil teks prompt dari, untuk prompt yang terlalu panjang untuk disematkan dalam string query. URL harus memungkinkan permintaan lintas asal. Diabaikan ketika `prompt` juga diatur. |
| `repositories` | Daftar slug `owner/repo` yang dipisahkan koma untuk dipilih sebelumnya. Alias `repo` juga diterima.                                                                                                |
| `environment`  | Nama atau ID [lingkungan](#connect-github-and-create-an-environment) untuk dipilih sebelumnya.                                                                                                     |

URL-encode setiap nilai. Contoh di bawah membuka formulir dengan prompt dan repositori yang sudah dipilih:

```text theme={null}
https://claude.ai/code?prompt=Fix%20the%20login%20bug&repositories=acme/webapp
```

<h2 id="review-and-iterate">
  Tinjau dan ulangi
</h2>

Ketika Claude selesai, tinjau perubahan, tinggalkan umpan balik pada baris tertentu, dan terus sampai diff terlihat benar.

<Steps>
  <Step title="Buka tampilan diff">
    Indikator diff menunjukkan baris yang ditambahkan dan dihapus di seluruh sesi, misalnya `+42 -18`. Pilihnya untuk membuka tampilan diff, dengan daftar file di sebelah kiri dan perubahan di sebelah kanan.
  </Step>

  <Step title="Tinggalkan komentar inline">
    Pilih baris apa pun di diff, ketik umpan balik Anda, dan tekan Enter. Komentar antri sampai Anda mengirim pesan berikutnya, kemudian digabungkan dengannya. Claude melihat "di `src/auth.ts:47`, jangan tangkap kesalahan di sini" bersama instruksi utama Anda, jadi Anda tidak harus menjelaskan di mana masalahnya.
  </Step>

  <Step title="Buat permintaan tarik">
    Ketika diff terlihat benar, pilih **Buat PR** di bagian atas tampilan diff. Anda dapat membukanya sebagai PR penuh, draft, atau melompat ke halaman compose GitHub dengan judul dan deskripsi yang dihasilkan.
  </Step>

  <Step title="Terus ulangi setelah PR">
    Sesi tetap aktif setelah PR dibuat. Tempel output kegagalan CI atau komentar pengulas ke chat dan minta Claude untuk mengatasinya. Untuk membuat Claude memantau PR secara otomatis, lihat [Auto-fix pull requests](/docs/id/claude-code-on-the-web#auto-fix-pull-requests).
  </Step>
</Steps>

<h2 id="troubleshoot-setup">
  Troubleshoot setup
</h2>

<h3 id="no-repositories-appear-after-connecting-github">
  Tidak ada repositori yang muncul setelah menghubungkan GitHub
</h3>

Sesi cloud dapat menggunakan repositori apa pun yang dapat dilihat akun GitHub yang terhubung, terlepas dari repositori mana yang Aplikasi Claude GitHub diinstal. Jika repositori hilang, verifikasi akun GitHub yang terhubung memiliki akses ke repositori di GitHub. Jika Anda juga ingin [Auto-fix](/docs/id/claude-code-on-the-web#auto-fix-pull-requests) untuk repositori, instal Aplikasi di atasnya: di github.com, buka **Settings → Applications → Claude → Configure** dan verifikasi repositori terdaftar di bawah **Repository access**. Repositori pribadi memerlukan otorisasi yang sama dengan yang publik.

<h3 id="the-page-only-shows-a-github-login-button">
  Halaman hanya menampilkan tombol login GitHub
</h3>

Sesi cloud memerlukan akun GitHub yang terhubung. Hubungkan melalui alur browser di atas, atau jalankan `/web-setup` dari terminal Anda jika Anda menggunakan GitHub CLI. Jika Anda lebih suka tidak menghubungkan GitHub sama sekali, lihat [Remote Control](/docs/id/remote-control) untuk menjalankan Claude Code di mesin Anda sendiri dan memantaunya dari web.

<h3 id="not-available-for-the-selected-organization">
  "Tidak tersedia untuk organisasi yang dipilih"
</h3>

Organisasi Enterprise mungkin memerlukan Owner untuk mengaktifkan Claude Code di web. Hubungi tim akun Anthropic Anda.

<h3 id="/web-setup-shows-no-commands-match-or-unknown-command">
  `/web-setup` menunjukkan "Tidak ada perintah yang cocok" atau "Perintah Tidak Dikenal"
</h3>

`/web-setup` berjalan di dalam Claude Code CLI, bukan shell Anda. Luncurkan `claude` terlebih dahulu, kemudian ketik `/web-setup` di prompt.

Jika Anda mengetiknya di dalam Claude Code dan menu perintah menunjukkan `Tidak ada perintah yang cocok "/web-setup"`, atau mengirimkannya mengembalikan `Perintah Tidak Dikenal: /web-setup`, perintah tersembunyi karena persyaratan tidak terpenuhi. Penyebabnya biasanya bahwa Anda diautentikasi dengan kunci API atau penyedia pihak ketiga alih-alih langganan claude.ai. Jalankan `/login` untuk masuk dengan akun claude.ai Anda.

<h3 id="could-not-create-a-cloud-environment-or-no-cloud-environment-available-when-using-cloud-or-ultraplan">
  "Tidak dapat membuat lingkungan cloud" atau "Tidak ada lingkungan cloud yang tersedia" saat menggunakan `--cloud` atau ultraplan
</h3>

Fitur sesi jarak jauh membuat lingkungan cloud default secara otomatis jika Anda tidak memilikinya. Jika Anda melihat "Tidak dapat membuat lingkungan cloud", pembuatan otomatis gagal. Jika Anda melihat "Tidak ada lingkungan cloud yang tersedia", CLI Anda mendahului pembuatan otomatis. Dalam kedua kasus, jalankan `/web-setup` di Claude Code CLI untuk membuat satu secara manual, atau kunjungi [claude.ai/code](https://claude.ai/code) dan ikuti langkah **Buat lingkungan Anda** di atas.

<h3 id="setup-script-failed">
  Skrip setup gagal
</h3>

Skrip setup keluar dengan status bukan nol, yang memblokir sesi dari dimulai. Penyebab umum:

* Instalasi paket gagal karena registri tidak ada di [tingkat akses jaringan](/docs/id/claude-code-on-the-web#access-levels) Anda. `Trusted` mencakup sebagian besar manajer paket; `None` memblokir semuanya.
* Skrip mereferensikan file atau jalur yang tidak ada dalam klon segar.
* Perintah yang bekerja secara lokal memerlukan invokasi berbeda di Ubuntu.

Untuk debug, tambahkan `set -x` di bagian atas skrip untuk melihat perintah mana yang gagal. Untuk perintah non-kritis, tambahkan `|| true` sehingga mereka tidak memblokir awal sesi.

<h3 id="new-sessions-hang-or-time-out-during-setup">
  Sesi baru hang atau timeout selama setup
</h3>

Jika sesi baru macet pada langkah skrip setup atau gagal dengan kesalahan kontainer generik sebelum skrip selesai, skrip kemungkinan melebihi anggaran waktu sekitar lima menit untuk membangun [cache lingkungan](/docs/id/claude-code-on-the-web#environment-caching). Langkah berat seperti menarik gambar Docker besar, menyinkronkan pohon dependensi penuh, atau mengunduh bobot model sering mendorong total melampaui batas, terutama ketika mereka berjalan satu demi satu.

Untuk memperbaiki ini, pangkas skrip sehingga dapat diandalkan selesai dalam waktu kurang dari lima menit:

* Jalankan instalasi independen secara paralel dengan `&` dan `wait` akhir alih-alih menjalankannya secara serial.
* Pindahkan unduhan terbesar keluar dari skrip setup dan ke [SessionStart hook](/docs/id/claude-code-on-the-web#setup-scripts-vs-sessionstart-hooks) yang meluncurkannya di latar belakang, sehingga sesi menjadi dapat digunakan saat mereka selesai.
* Hapus tidur ulang panjang dari skrip setup, karena loop ulang yang macet dihitung terhadap anggaran.

<h3 id="session-keeps-running-after-closing-the-tab">
  Sesi terus berjalan setelah menutup tab
</h3>

Ini dirancang demikian. Menutup tab atau menavigasi pergi tidak menghentikan sesi. Ini terus berjalan di latar belakang sampai Claude menyelesaikan tugas saat ini, kemudian menganggur. Dari sidebar, Anda dapat [mengarsipkan sesi](/docs/id/claude-code-on-the-web#archive-sessions) untuk menyembunyikannya dari daftar Anda, atau [menghapusnya](/docs/id/claude-code-on-the-web#delete-sessions) untuk menghapusnya secara permanen.

<h2 id="next-steps">
  Langkah berikutnya
</h2>

Sekarang bahwa Anda dapat mengirimkan dan meninjau tugas, halaman-halaman ini mencakup apa yang akan datang: memulai sesi cloud dari terminal Anda, menjadwalkan pekerjaan berulang, dan memberikan Claude instruksi berdiri.

* [Gunakan Claude Code di web](/docs/id/claude-code-on-the-web): referensi lengkap, termasuk teleportasi sesi ke terminal Anda, skrip setup, variabel lingkungan, dan konfigurasi jaringan
* [Routines](/docs/id/routines): otomatiskan pekerjaan sesuai jadwal, melalui panggilan API, atau sebagai respons terhadap peristiwa GitHub
* [CLAUDE.md](/docs/id/memory): berikan Claude instruksi dan konteks persisten yang dimuat di awal setiap sesi
* Instal aplikasi mobile Claude untuk [iOS](https://apps.apple.com/us/app/claude-by-anthropic/id6473753684) atau [Android](https://play.google.com/store/apps/details?id=com.anthropic.claude) untuk memantau sesi dari ponsel Anda. Dari Claude Code CLI, `/mobile` menampilkan kode QR.
