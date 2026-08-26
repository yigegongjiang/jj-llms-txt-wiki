> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Jalankan Claude Code di balik peluncur korporat

> Arahkan proses yang dimulai Claude Code dari binernya sendiri, termasuk layanan latar belakang dan setiap sesi tampilan agen, melalui peluncur yang diperlukan dengan CLAUDE_CODE_PROCESS_WRAPPER.

Beberapa organisasi memerlukan setiap proses pada workstation untuk dimulai melalui peluncur wajib. Peluncur menerapkan sandbox, kontrol jaringan, atau injeksi kredensial yang postur keamanan perusahaan bergantung padanya, dan biner yang dimulai tanpanya adalah pelanggaran kebijakan.

`CLAUDE_CODE_PROCESS_WRAPPER` memulai setiap proses yang diluncurkan Claude Code dari binernya sendiri melalui peluncur Anda: layanan latar belakang, setiap sesi yang dihosting dalam [tampilan agen](/docs/id/agent-view), dan peluncuran ulang Claude Code setelah pembaruan. Atur ke jalur absolut peluncur Anda, dan Claude Code menjalankan peluncur dengan perintah Claude Code sebagai argumennya.

Peluncur yang membungkus perintah `claude` pada `PATH` Anda tidak dapat menjangkau proses-proses ini, karena mereka dimulai dari jalur langsung biner tanpa mencari `claude`.

<Note>
  `CLAUDE_CODE_PROCESS_WRAPPER` memerlukan Claude Code v2.1.208 atau lebih baru. Versi sebelumnya mengabaikan variabel dan memulai setiap proses tanpa pembungkus.
</Note>

<h2 id="what-the-launcher-covers">
  Apa yang dicakup peluncur
</h2>

Dengan `CLAUDE_CODE_PROCESS_WRAPPER` diatur, Claude Code memulai masing-masing proses berikut melalui peluncur Anda:

* Layanan latar belakang yang dimulai `claude agents` dan sesi latar belakang sesuai permintaan.
* Host terminal dan sesi Claude Code di dalam setiap baris tampilan agen, termasuk sesi siaga hangat yang disimpan layanan.
* Sesi yang diluncurkan ulang layanan setelah pembaruan atau kerusakan.
* Peluncuran ulang yang dilakukan Claude Code pada dirinya sendiri untuk menyelesaikan pemasangan pembaruan, termasuk tindakan restart-for-update tampilan agen.

Di Windows, variabel diabaikan: kontrak peluncur bergantung pada `exec`, yang tidak didukung Windows. Mesin Windows dengan variabel yang diatur menjalankan setiap proses tanpa pembungkus dan terus bekerja, dan satu-satunya sinyal adalah peringatan dalam [log debug](/docs/id/troubleshooting). Jika kebijakan peluncur Anda mencakup Windows, variabel tidak memuaskannya di sana: hitung mesin Windows sebagai tanpa pembungkus saat Anda merencanakan peluncuran.

<h3 id="processes-that-start-outside-the-launcher">
  Proses yang dimulai di luar peluncur
</h3>

Tiga proses tidak pernah dimulai melalui peluncur:

* [Layanan latar belakang yang diinstal](/docs/id/agent-view#the-supervisor-process): `launchd` atau `systemd` memulai proses itu dari file unitnya. `/status` dan `claude daemon status` memperingatkan ketika ini berlaku, dan sesi yang diluncurkan layanan masih dimulai melalui peluncur setelah layanan dimulai ulang dengan variabel dalam pengaturannya.
* Sesi yang Anda mulai sendiri di terminal, yang berjalan bagaimanapun Anda menginvokasinya. Untuk mencakup sesi-sesi ini, letakkan skrip bernama `claude` di direktori sebelumnya pada `PATH` yang menjalankan peluncur Anda dengan biner asli; jangan ganti symlink yang dikelola. Self-spawn tidak berkonsultasi dengan `PATH`, jadi dua peluncur tidak pernah bertumpuk.
* Proses pertama dari deep link `claude-cli://`, yang dimulai handler protokol sistem operasi secara langsung. Semua yang dimulai sesi itu di latar belakang sesudahnya berjalan melalui peluncur. Untuk menutup jalur ini sepenuhnya, [cegah pendaftaran handler](/docs/id/deep-links#registration-and-supported-platforms) dengan pengaturan `disableDeepLinkRegistration`.

<h3 id="helper-process-names-in-process-monitors">
  Nama proses pembantu dalam monitor proses
</h3>

Dengan peluncur yang dikonfigurasi, `ps` dan Activity Monitor menampilkan nama biner versi untuk proses pembantu latar belakang alih-alih label `claude bg-pty-host` dan `claude bg-spare` Claude Code, karena `exec` peluncur membangun ulang daftar argumen. Penamaan ulang adalah efek samping, bukan penyembunyian: proses-proses sebaliknya tidak berubah, dan Claude Code mengidentifikasi proses-prosesnya sendiri berdasarkan jalur biner, tidak pernah berdasarkan nama tampilan.

<h2 id="set-up-the-launcher">
  Atur peluncur
</h2>

<Steps>
  <Step title="Tulis skrip peluncur">
    Buat skrip yang dapat dieksekusi di jalur absolut, seperti `/opt/corp/launcher`. Claude Code menjalankannya dengan perintah Claude Code lengkap sebagai argumennya, dan skrip harus diakhiri dengan memanggil `exec "$@"` sehingga menggantikan dirinya dengan Claude Code:

    ```bash theme={null}
    #!/bin/sh
    # Pengaturan organisasi Anda: masukkan sandbox, terapkan
    # kontrol jaringan, atau injeksikan kredensial.
    exec "$@"
    ```

    Buat dapat dieksekusi dengan `chmod +x`. Bagian pengaturan adalah apa pun yang harus dilakukan peluncur Anda sebelum Claude Code berjalan; [kontrak peluncur](#the-launcher-contract) di bawah mencantumkan aturan yang harus diikuti skrip.

    <Note>
      Jika Anda sebelumnya mengganti symlink `~/.local/bin/claude` dengan peluncur Anda, pulihkan symlink asli dalam perubahan yang sama. Symlink yang diganti membuat sesi pembungkus pertama memulai layanan latar belakang melalui kedua peluncur sekaligus, dan menempatkan instalasi dalam keadaan yang dikelola secara eksternal: `/doctor` melaporkannya, auto-update membiarkan file tetap ada, dan pembersihan versi lama tetap dinonaktifkan sampai installer mengelola jalur itu lagi.
    </Note>
  </Step>

  <Step title="Atur CLAUDE_CODE_PROCESS_WRAPPER dalam pengaturan">
    Atur variabel dalam blok `env` file pengaturan sehingga layanan latar belakang yang terpisah mewarisinya. `export` shell tidak cukup: layanan latar belakang dimulai sesuai permintaan, bertahan lebih lama dari shell Anda, dan tidak pernah membaca ulang profil shell.

    Untuk satu mesin, tambahkan ke `~/.claude/settings.json`. Untuk menerapkannya ke setiap mesin di organisasi Anda, letakkan blok yang sama dalam [pengaturan yang dikelola](/docs/id/permissions#managed-settings):

    ```json theme={null}
    {
      "env": {
        "CLAUDE_CODE_PROCESS_WRAPPER": "/opt/corp/launcher"
      }
    }
    ```

    Ketika lebih dari satu sumber menetapkan variabel, nilai pengaturan yang dikelola menggantikan baik `~/.claude/settings.json` maupun nilai yang diekspor dalam shell, sehingga pengguna tidak dapat mengarahkan self-spawn ke peluncur yang berbeda.

    Pengaturan proyek dan lokal tidak dapat menetapkan variabel ini. File yang dikomitkan ke repositori tidak boleh dapat menempatkan biner di depan setiap proses Claude Code pada mesin, jadi `CLAUDE_CODE_PROCESS_WRAPPER` dalam `.claude/settings.json` atau `.claude/settings.local.json` diabaikan, dengan peringatan dalam [log debug](/docs/id/troubleshooting).
  </Step>

  <Step title="Mulai ulang layanan latar belakang dan sesi Anda">
    Layanan latar belakang yang berjalan dan sesi `claude` apa pun yang terbuka membaca variabel sekali saat startup, jadi mereka terus meluncurkan proses tanpa pembungkus sampai dimulai ulang. Jalankan `claude daemon stop --any` untuk menghentikan layanan sesuai permintaan; perintah berikutnya yang membutuhkannya, seperti `claude agents`, memulai yang dibungkus. [Layanan yang diinstal](/docs/id/agent-view#the-supervisor-process) mengambil `claude daemon stop` tanpa `--any`. Kemudian mulai ulang sesi `claude` terbuka Anda.

    Pada mesin yang tidak dapat Anda mulai ulang dengan tangan, sesi pertama yang dimulai setelah push pengaturan secara otomatis menghentikan layanan sesuai permintaan yang tersisa tanpa pembungkus. Mesin di mana tidak ada sesi baru yang dimulai menyimpan layanannya yang tanpa pembungkus sampai satu dimulai, dan layanan yang diinstal selalu memerlukan restart dalam langkah ini.
  </Step>

  <Step title="Verifikasi">
    Jalankan `/status` dalam sesi: entri Self-exec menunjukkan perintah peluncuran yang diselesaikan dan memperingatkan ketika layanan latar belakang yang berjalan tidak cocok dengannya. `claude daemon status` mencetak informasi yang sama dari shell, termasuk setelah Anda membatalkan pengaturan variabel, ketika `/status` tidak lagi menampilkan entri.
  </Step>
</Steps>

<h2 id="the-launcher-contract">
  Kontrak peluncur
</h2>

Ketika peluncur tidak dapat berjalan, Claude Code menolak untuk memulai proses alih-alih memulainya tanpa pembungkus. Di Windows, [variabel diabaikan](#what-the-launcher-covers) dan proses dimulai tanpa pembungkus. Claude Code memegang skrip ke aturan-aturan ini:

* **Akhiri dengan `exec "$@"`.** Peluncur yang melahirkan anak dan keluar meninggalkan proses Claude Code yatim piatu yang tidak dapat dilacak layanan latar belakang. Tampilan agen menandai sesi seperti itu gagal dengan pesan yang menyebutkan peluncur, dan layanan mengumpulkan apa yang ditinggalkan peluncur.
* **Jangan urutkan ulang, serap, atau tambahkan argumen.** Argumen pertama adalah biner Claude Code dan semuanya setelahnya adalah argv-nya.
* **Lewatkan setiap variabel lingkungan yang diwarisi melalui ke `exec`.** Menambahkan variabel, seperti kredensial yang disuntikkan, baik-baik saja; menjatuhkan yang diwarisi tidak.
  * Token autentikasi per-sesi, pemilihan model dan penyedia, dan `CLAUDE_CODE_PROCESS_WRAPPER` itu sendiri semuanya berjalan di lingkungan yang diwarisi, jadi peluncur yang membangun ulangnya dari daftar izin memecahkan sesi yang dimulainya, dan `/status` melaporkan ketidakcocokan peluncur.
  * Jika peluncur harus memasuki namespace atau sandbox yang mengatur ulang lingkungan, ekspor ulang lingkungan yang diwarisi di dalamnya secara verbatim.
* **Capai `exec` dalam sekitar tiga detik setiap kali peluncur berjalan.** Pengiriman latar belakang dingin menjalankan peluncur dua kali berturut-turut sebelum byte pertama output, jadi lakukan pekerjaan lambat seperti pertukaran single sign-on dengan malas atau dari cache.
  * Peluncur yang berjalan jauh melampaui anggaran diperlakukan sebagai awal yang terhenti dan dimulai ulang.
* **Toleransi untuk diinvokasi dari dalam dirinya sendiri.** Claude Code menerapkan peluncur ke setiap self-spawn bersarang, jadi peluncur yang memperoleh sumber daya eksklusif harus mendeteksi bahwa itu sudah memegangnya.
* **Jangan tulis ke terminal sebelum Claude Code dimulai.** Apa pun yang dicetak sebelum `exec` dilaporkan sebagai penyebab kerusakan jika sesi mati sebelum inisialisasi.

<h3 id="format-of-the-claude_code_process_wrapper-value">
  Format nilai `CLAUDE_CODE_PROCESS_WRAPPER`
</h3>

Untuk sebagian besar peluncur, nilainya hanya jalur absolut skrip, seperti `/opt/corp/launcher`.

Untuk melewatkan argumen peluncur Anda sendiri, tuliskan setelah jalur. Claude Code menguraikan nilai sebagai daftar argumen, bukan perintah shell:

* Whitespace memisahkan token, dan tanda kutip ganda mengelompokkan token yang berisi spasi.
* Nilai yang dimulai dengan `[` dibaca sebagai array string JSON, seperti `["/opt/corp/launcher", "--profile", "cc"]`.
* Sintaks shell tidak berfungsi: tidak ada ekspansi variabel atau globbing, dan operator yang tidak dikutip seperti `;`, `|`, `&`, atau `$(` ditolak sebagai kesalahan konfigurasi daripada diinterpretasikan ulang.

Ketika nilai tidak dapat digunakan, Claude Code menolak untuk memulai proses yang terpengaruh dan [melaporkan alasannya](/docs/id/errors#claude_code_process_wrapper-launcher-errors).

<h2 id="relationship-to-claude_code_shell_prefix">
  Hubungan dengan `CLAUDE_CODE_SHELL_PREFIX`
</h2>

`CLAUDE_CODE_PROCESS_WRAPPER` membungkus proses Claude Code sendiri dan melewatkan perintah melalui sebagai token argv terpisah untuk peluncur ke `exec`. [`CLAUDE_CODE_SHELL_PREFIX`](/docs/id/env-vars) membungkus perintah shell yang dijalankan Claude Code atas nama Anda, seperti panggilan alat Bash, hooks, dan perintah yang memulai server MCP stdio, dan melewatkan masing-masing sebagai string yang dikutip shell tunggal dalam `$1` untuk pembungkus untuk mengevaluasi ulang. Peluncur yang ditulis untuk satu tidak berfungsi sebagai yang lain.

<h2 id="related-resources">
  Sumber daya terkait
</h2>

* [Tampilan agen](/docs/id/agent-view): sesi latar belakang dan proses supervisor yang dicakup peluncur
* [Variabel lingkungan](/docs/id/env-vars): entri referensi `CLAUDE_CODE_PROCESS_WRAPPER`
* [Pengaturan yang dikelola](/docs/id/permissions#managed-settings): berikan blok `env` di seluruh armada
* [Referensi kesalahan peluncur](/docs/id/errors#claude_code_process_wrapper-launcher-errors): pesan penolakan dan cara memulihkan
