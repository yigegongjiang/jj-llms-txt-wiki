> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Panduan cepat lingkungan yang di-host sendiri

> Siapkan lingkungan yang di-host sendiri pertama Anda: instal Claude Code, buat lingkungan, mulai runner, dan arahkan sesi ke sana.

<Note>
  Lingkungan yang di-host sendiri berada dalam beta publik pada paket Team dan Enterprise; [Ketersediaan dan batasan](/docs/id/self-hosted-environments#availability-and-limitations) mencakup jalur pengaktifan. Halaman ini menjalankan sesi pertama Anda; lihat [Lingkungan yang di-host sendiri](/docs/id/self-hosted-environments) untuk mengetahui apa itu dan [Terapkan ke produksi](/docs/id/self-hosted-environments-deploy) untuk pengerasan dan resep armada.
</Note>

Sebuah [lingkungan yang di-host sendiri](/docs/id/self-hosted-environments) menjalankan Claude Code [sesi cloud](/docs/id/claude-code-on-the-web) pada infrastruktur yang dioperasikan organisasi Anda, dijalankan oleh proses runner yang Anda terapkan. Panduan cepat ini menyiapkan yang pertama, yang terkecil yang berfungsi: satu runner pada satu host, menjalankan satu sesi uji. Ada dua langkah: [buat lingkungan, mulai runner, dan arahkan sesi ke sana](#set-up-an-environment-and-runner), kemudian [kirim pesan ke sesi itu dari terminal Anda](#send-a-follow-up-message-to-a-running-session). Anda akan berpindah antara dua permukaan: claude.ai untuk membuat lingkungan, memeriksa statusnya, dan mengarahkan sesi, dan terminal pada host untuk semua yang dilakukan runner.

Pada akhirnya Anda akan memiliki lingkungan pada [halaman admin **Cloud environments**](https://claude.ai/admin-settings/cloud-environments), runner yang menunggu pekerjaan, dan sesi yang berjalan pada host Anda. Sebelum Anda menghubungkan repositori nyata atau sistem internal, kerjakan [Terapkan ke produksi](/docs/id/self-hosted-environments-deploy), yang mencakup postur keamanan, kontrol egress, kredensial git, dan orkestrasi.

<h2 id="prerequisites">
  Prasyarat
</h2>

<h3 id="organization-and-roles">
  Organisasi dan peran
</h3>

Sisi claude.ai memerlukan:

* **Izinkan lingkungan yang di-host sendiri** diaktifkan oleh [Pemilik](/docs/id/cloud-environments#organization-shared-environments) pada [halaman admin **Cloud environments**](https://claude.ai/admin-settings/cloud-environments); tombol **New** tidak muncul sampai diaktifkan. Jika Anda tidak memiliki peran tersebut, seseorang yang memilikinya dapat membuat lingkungan dan memberikan rahasia kepada Anda; langkah runner dan terminal pada halaman ini tidak memerlukan peran claude.ai, dan di mana langkah memeriksa status di UI admin, baris log runner sendiri memberikan sinyal yang sama.
* Sebuah [koneksi GitHub](/docs/id/claude-code-on-the-web#github-authentication-options) untuk organisasi Anda, sehingga pengembang dapat memilih repositori saat mereka memulai sesi.

<h3 id="host-and-network">
  Host dan jaringan
</h3>

Host runner memerlukan:

* Host atau kontainer Linux atau macOS dengan HTTPS keluar ke `api.anthropic.com`, ke `claude.ai` dan host unduhan yang dialihkan untuk langkah instalasi di bawah, dan ke host git Anda untuk klon; [tabel persyaratan jaringan](/docs/id/self-hosted-environments-deploy#network-requirements) memiliki daftar lengkap. Windows tidak didukung sebagai host runner; jalankan runner dalam kontainer Linux sebagai gantinya. Workstation pengembang tidak terpengaruh, karena sesi dimulai dari claude.ai di browser.
* Jam yang disinkronkan dengan waktu nyata, misalnya dengan NTP. Autentikasi gagal ketika jam lebih dari lima menit mati; lihat [Troubleshooting](/docs/id/self-hosted-environments-deploy#troubleshooting).

<h3 id="software-on-the-runner-host">
  Perangkat lunak pada host runner
</h3>

Instal pada host sebelum Anda memulai:

* **Claude Code v2.1.224 atau lebih baru**, dengan salah satu dari [metode instalasi standar](/docs/id/setup). Runner adalah bagian dari biner `claude` standar, dan versi sebelumnya tidak mengenali subperintah `self-hosted-runner`. Saluran `latest` penginstal asli membawa setiap rilis segera setelah dipublikasikan; saluran `stable`, cask Homebrew `claude-code`, dan repositori apt, dnf, dan apk yang stabil tertinggal sekitar seminggu. Untuk menyematkan versi yang tepat yang dijalankan armada Anda, lihat [Instal versi tertentu](/docs/id/setup#install-a-specific-version). Untuk gambar kontainer, lihat Dockerfile di [Terapkan ke produksi](/docs/id/self-hosted-environments-deploy#build-the-runner-image).
* **Git 2.24 atau lebih baru**. Beberapa opsi git pada halaman terapkan memerlukan versi yang lebih baru; [Konfigurasi git](/docs/id/self-hosted-environments-deploy#configure-git) menyatakan setiap lantai.

Konfirmasi host siap:

```bash theme={null}
claude self-hosted-runner --help
```

Host yang siap mencetak teks penggunaan runner, mencantumkan bendera seperti `--environment-secret-file`. Pada versi yang lebih lama dari 2.1.224, perintah mencetak output `claude --help` umum sebagai gantinya; tingkatkan dengan `claude update` atau instal ulang dari saluran `latest`.

<h2 id="set-up-an-environment-and-runner">
  Siapkan lingkungan dan runner
</h2>

Claude Code mencakup pengaturan terpandu: sesi Claude Code interaktif yang memandu Anda melalui pembuatan lingkungan di UI admin, memulai runner lokal dengan file rahasia yang Anda simpan, mengonfirmasi bahwa runner terdaftar, dan menulis lembar contekan ke `./runner-setup/CHEAT-SHEET.md`. Jalankan pada mesin di mana Anda telah masuk dengan `claude auth login` menggunakan akun yang memiliki peran Pemilik; tidak tersedia dengan kunci API atau penyedia model pihak ketiga. Pada host di mana sesi interaktif tidak mungkin, gunakan langkah manual di bawah sebagai gantinya. Konfirmasi [pemeriksaan versi](#software-on-the-runner-host) lulus terlebih dahulu: pada versi yang lebih lama dari 2.1.224, perintah ini memulai sesi Claude biasa dengan kata-kata sebagai prompt alih-alih pengaturan terpandu. Untuk memulai pengaturan terpandu, jalankan subperintah setup dan ikuti prompt:

```bash theme={null}
claude self-hosted-runner setup
```

Untuk menyiapkan secara manual sebagai gantinya:

<Steps>
  <Step title="Buat lingkungan">
    Buka halaman [**Cloud environments**](https://claude.ai/admin-settings/cloud-environments) di pengaturan admin. Di bawah **Self-hosted environments**, pilih **New**, beri nama lingkungan, dan pilih **Create**. Pada langkah kedua wizard, pilih **Copy environment key** untuk menyalin rahasia lingkungan, yang UI admin beri label kunci lingkungan. claude.ai menampilkan rahasia sekali, dan Anda tidak dapat mengambilnya nanti; itu kedaluwarsa 365 hari setelah pembuatan. ID `ccpool_...` lingkungan tetap terlihat dalam dialog detailnya; Anda akan membutuhkannya untuk pemeriksaan `aud` dalam [verifikasi token](/docs/id/self-hosted-environments-identity) dan untuk mengirim [sesi uji dari CI](/docs/id/self-hosted-environments-testing#run-the-test-loop).

    Jika Anda kehilangan rahasia atau perlu memutar ulang, buat rahasia baru dari tab **Configuration** lingkungan, gulirkan rahasia baru ke runner Anda, kemudian cabut yang lama. Runner yang memegang rahasia yang dicabut gagal polling terautentikasi berikutnya dan keluar, mencatat `poll auth failed`, dan orkestrator Anda memulai ulang dengan rahasia baru.
  </Step>

  <Step title="Mulai runner">
    Buat direktori rahasia. Langkah ini dan berikutnya memerlukan root untuk jalur `/etc/claude`; jalur apa pun yang dapat dibaca proses runner berfungsi, jadi sesuaikan kedua perintah dan nilai `--environment-secret-file` bersama-sama jika Anda menggunakan yang berbeda.

    ```bash theme={null}
    mkdir -p /etc/claude
    ```

    Tulis rahasia lingkungan ke file. Perintah di bawah membaca dari terminal Anda sehingga rahasia tetap keluar dari riwayat shell: tempel nilai yang Anda salin, tekan Enter, kemudian Ctrl-D, dan `umask` subshell membuat file dapat dibaca hanya oleh pemiliknya.

    ```bash theme={null}
    (umask 077 && cat > /etc/claude/environment-secret)
    ```

    Pilih direktori dasar, mengganti `<writable-dir>` dalam perintah runner di bawah dengan jalur absolut yang dapat ditulis atau dibuat oleh runner. Runner membuat direktori saat startup, kemudian memeriksa repositori dan membuat direktori per-sesi di bawahnya. Tanpa `--base-dir` itu menggunakan `/workspace`, yang hanya berfungsi jika direktori itu sudah ada dan dapat ditulis atau Anda memulai runner sebagai root.

    Jika runner tidak dapat membuat atau menulis ke jalur, itu keluar saat startup dengan kesalahan yang menamai direktori alih-alih mendaftar. Lihat [Troubleshooting](/docs/id/self-hosted-environments-deploy#troubleshooting).

    Kemudian mulai runner dengan `--environment-secret-file` dan `--base-dir`. Runner mendaftar dengan lingkungan Anda dan mulai menunggu pekerjaan. Jika runner keluar, mulai ulang dengan tangan. Penerapan produksi menjalankan runner di bawah orkestrator yang memulai ulang runner yang keluar, biasanya dengan filesystem segar per restart; [Gunakan kembali checkout yang sudah hangat](/docs/id/self-hosted-environments-deploy#reuse-a-pre-warmed-checkout) mencakup pengaturan disk persisten yang didukung.

    ```bash theme={null}
    claude self-hosted-runner --environment-secret-file '/etc/claude/environment-secret' --base-dir '<writable-dir>'
    ```
  </Step>

  <Step title="Verifikasi runner muncul">
    Kembali ke [halaman **Cloud environments**](https://claude.ai/admin-settings/cloud-environments). Status lingkungan Anda berubah dari **No runners deployed** menjadi **Healthy** dalam beberapa detik setelah runner dimulai; buka lingkungan dan pilih **Activity** untuk melihat runner itu sendiri.
  </Step>

  <Step title="Arahkan sesi ke lingkungan">
    Mulai sesi di claude.ai/code dan pilih lingkungan Anda dari pemilih lingkungan, di mana lingkungan yang di-host sendiri muncul bersama yang di-host Anthropic. Runner mengklon dengan kredensial git apa pun yang sudah dimiliki host, jadi pilih repositori yang sudah dapat diklon host ini, atau yang publik; opsi kredensial untuk repositori pribadi dalam produksi ada di [Konfigurasi git](/docs/id/self-hosted-environments-deploy#configure-git). Runner yang tersedia berikutnya mengambil sesi yang antri dan mencatat `Picked up session <session-id>` bersama dengan hitungan aktif dan kapasitasnya, sehingga Anda dapat mengonfirmasi dari output runner sendiri host mana yang mengambil sesi. Tonton sesi bekerja dan baca balasan Claude di [claude.ai/code](https://claude.ai/code). Jika sesi duduk antri sebagai gantinya, lihat [Troubleshooting](/docs/id/self-hosted-environments-deploy#troubleshooting).
  </Step>
</Steps>

Runner keluar dengan desain setelah sesi aktifnya selesai; lihat [Runner lifecycle](/docs/id/self-hosted-environments#runner-lifecycle). Untuk produksi, terapkan di bawah orkestrator yang memulai ulang saat keluar. Lihat [Terapkan ke produksi](/docs/id/self-hosted-environments-deploy).

<h2 id="send-a-follow-up-message-to-a-running-session">
  Kirim pesan lanjutan ke sesi yang berjalan
</h2>

Setelah sesi berjalan di lingkungan Anda, kirim lanjutan darinya dari CLI `claude` pada mesin apa pun di mana Anda masuk dengan `claude auth login`; perintah tidak perlu dijalankan dari mesin yang memulai sesi. Perintah memposting satu pesan:

```bash theme={null}
claude -p "your message" --cloud <session-id>
```

Untuk `<session-id>`, berikan ID `session_...` atau `cse_...` telanjang atau URL claude.ai/code sesi. Pengiriman yang berhasil mencetak `Sent to cloud session.` dengan ID sesi dan tautan tampilan. Bentuk ID yang diterima, output JSON, persyaratan akun dan kebijakan, dan referensi kesalahan ada di [Kirim lanjutan dari CLI](/docs/id/claude-code-on-the-web#send-follow-ups-from-the-cli), karena perintah bekerja sama terhadap sesi yang di-host Anthropic.

<h2 id="what’s-next">
  Apa selanjutnya
</h2>

* [Terapkan ke produksi](/docs/id/self-hosted-environments-deploy): keraskan penerapan, kontrol egress, konfigurasi kredensial git, dan jalankan armada di bawah Kubernetes atau Compose
* [Sesuaikan sesi](/docs/id/self-hosted-environments-configuration): skrip wrapper, hook siklus hidup, runner sesuai permintaan, server MCP, dan izin
* [Uji end to end](/docs/id/self-hosted-environments-testing): uji asap CI yang mengirim sesi dan membaca balasan Claude
