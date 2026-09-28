> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Uji aplikasi iOS di simulator

> Claude Code Desktop membuka aplikasi Anda di pane iOS Simulator ketika Claude membangun, menjalankan, atau memeriksanya, dengan simulator terpisah untuk setiap sesi.

<Note>
  Pane iOS Simulator berada dalam beta publik di Claude Code Desktop di macOS. Tersedia di paket Pro, Max, Team, dan Enterprise, kecuali di organisasi Enterprise yang memiliki konfigurasi HIPAA yang diaktifkan.
</Note>

Pane iOS Simulator menampilkan aplikasi Anda berjalan di Apple iOS Simulator di samping percakapan Anda di Claude Code Desktop. Ketika Claude membangun, memasang, meluncurkan, atau memeriksa aplikasi Anda di simulator, pane terbuka secara otomatis dan melakukan streaming layar perangkat secara langsung. Gunakan untuk menonton Claude menjalankan dan menguji aplikasi Anda, atau ketuk aplikasi sendiri sementara Claude terus bekerja.

Pane simulator mengendalikan simulator secara langsung, jadi tidak memerlukan [computer use](/docs/id/desktop#let-claude-use-your-computer) dan tidak pernah mengambil alih layar Anda atau menyembunyikan jendela lain Anda. Dari CLI, Claude menjangkau iOS Simulator melalui [computer use](/docs/id/computer-use#test-a-simulator-flow) sebagai gantinya, yang mengontrol simulator di layar Anda dengan cara yang sama seperti yang Anda lakukan dengan mouse.

<h2 id="requirements">
  Persyaratan
</h2>

Pane simulator menggunakan alat simulator Apple, yang tidak disertakan dalam aplikasi desktop. Sebelum memulai sesi, pastikan Anda memiliki:

* Claude Desktop v1.24012.0 atau lebih baru
* Mac, karena Apple iOS Simulator hanya berjalan di macOS
* [Xcode](https://developer.apple.com/xcode/) dengan platform iOS terinstal, yang menyediakan perangkat simulator. Jika Xcode belum menampilkan simulator, lihat [Pane simulator mengatakan tidak ada simulator yang ditemukan](#the-simulator-pane-says-no-simulators-were-found)
  * Gunakan Xcode 26.x. Pane belum bekerja dengan Xcode 27, yang menggantikan aplikasi Simulator dengan Device Hub. Jika `xcode-select` menunjuk ke Xcode 27 di Mac Anda, lihat [Pane simulator gagal dengan Xcode 27](#the-simulator-pane-fails-with-xcode-27)

<Note>
  Di halaman ini, "perangkat" mengacu pada iPhone atau iPad yang disimulasikan, salah satu dari perangkat simulator yang sama yang Anda kelola di Xcode di bawah **Window → Devices and Simulators**, bukan perangkat keras fisik.
</Note>

Pane simulator tersedia hanya dalam sesi lokal. Dalam sesi [cloud](/docs/id/desktop#run-long-running-tasks-in-the-cloud) dan [SSH](/docs/id/desktop#ssh-sessions), Claude berjalan di mesin yang tidak dapat menjangkau simulator di Mac Anda.

<h2 id="run-your-app-in-the-simulator">
  Jalankan aplikasi Anda di simulator
</h2>

Anda tidak memerlukan perintah atau pengaturan untuk membuka pane simulator. Claude membukanya ketika menjalankan aplikasi Anda di simulator.

<Steps>
  <Step title="Buka proyek iOS Anda">
    Di Claude Code Desktop, buka tab **Code** dan mulai sesi dengan folder proyek aplikasi Anda sebagai [project folder](/docs/id/desktop#start-a-session). Proyek apa pun yang membangun aplikasi untuk iOS Simulator berfungsi.
  </Step>

  <Step title="Minta Claude untuk menjalankan atau menguji aplikasi">
    Rumuskan tugas seputar menjalankan atau memverifikasi aplikasi. Sebagai contoh:

    ```text theme={null}
    Build the app and run it in the simulator to check the onboarding flow.
    ```
  </Step>

  <Step title="Tonton aplikasi di pane simulator">
    Ketika aplikasi diluncurkan di simulator, pane iOS Simulator terbuka di samping percakapan. Pertama kali Claude menggunakan perangkat, aplikasi desktop meminta Anda untuk mengizinkannya; lihat [Berikan Claude akses ke perangkat](#grant-claude-access-to-a-device). Claude memasang aplikasi, mengetuk melaluinya, dan membaca layar untuk memverifikasi perubahannya sendiri sementara Anda menonton.
  </Step>
</Steps>

Pane simulator terbuka setiap kali Claude meluncurkan aplikasi di simulator, pada titik mana pun dalam sesi. Ketika permintaan Anda adalah tentang melihat aplikasi, misalnya "apakah layar baru terlihat benar?", Claude memulai simulator sebelum mulai bekerja. Setelah Claude memperbaiki bug atau mengubah layar, minta Claude memverifikasi perubahan: meluncurkan ulang aplikasi membuka kembali pane jika tidak terbuka.

Pane simulator menampilkan perangkat mana pun tempat aplikasi benar-benar diluncurkan. Untuk menguji pada perangkat tertentu, sebutkan perangkat itu dalam permintaan Anda, misalnya "jalankan di simulator iPhone SE", dan Claude menargetkan perangkat itu ketika membangun dan meluncurkan.

Perangkat yang Claude boot juga muncul di aplikasi Simulator Apple, dan Claude dapat memasang aplikasi di perangkat yang sudah Anda boot.

Anda juga dapat membuka pane simulator sendiri. Setelah sesi memiliki simulator yang terpasang atau telah mengedit file Swift, menu **Views** di toolbar sesi menampilkan entri **iOS Simulator**. Jika pane belum menampilkan perangkat, klik **Attach simulator**, atau pilih perangkat tertentu dari menu perangkat di sebelahnya; memilih perangkat yang dalam keadaan mati akan mem-boot-nya. Jika Xcode atau simulator-nya hilang, pane menampilkan langkah-langkah setup sebagai gantinya dan mencentangnya saat Anda menyelesaikannya.

<h2 id="control-the-simulator-yourself">
  Kontrol simulator sendiri
</h2>

Pane simulator bersifat interaktif, bukan hanya penampil. Sementara Claude bekerja, atau di antara tugas, Anda dapat:

* Ketuk dan geser dengan mengklik dan menyeret di layar perangkat
* Tekan tombol perangkat keras dengan pintasan keyboard yang sama seperti aplikasi Simulator Apple: **Cmd+Shift+H** untuk Home, **Cmd+L** untuk mengunci, **Cmd+Up Arrow** dan **Cmd+Down Arrow** untuk volume
* Putar perangkat seperempat putaran searah jarum jam dengan tombol putar atau **Cmd+Right Arrow**
* Alihkan perangkat mana yang ditampilkan pane dari menu perangkat, yang mencantumkan versi OS setiap simulator dan apakah itu di-boot
* Simpan tangkapan layar dengan **Cmd+S** atau perekaman layar dengan **Cmd+R**, menggunakan tombol tangkap pane atau pintasan keyboard; file disimpan ke Desktop Anda
* Hentikan streaming perangkat tanpa mematikannya dengan mengklik **Detach simulator**, yang mengembalikan pane ke status **Attach simulator**

Baris di bawah nama perangkat menyetel aliran video dari simulator. Kurangi **Frame rate** atau **Resolution** jika pane membebani Mac Anda, alihkan **Encoding** antara H.264 dan JPEG, atau periksa **FPS** untuk menampilkan frame rate yang diterima pane. Pengaturan ini mengubah cara pane menampilkan perangkat, bukan cara aplikasi berjalan.

Anda dan Claude mengendalikan perangkat yang sama, jadi ketukan Anda mengubah status aplikasi yang Claude lihat. Untuk membuat Claude memeriksa layar tertentu, navigasikan ke sana dengan mengetuk, lalu minta. Sementara Claude mengendalikan perangkat, pane menampilkan lencana **Claude is using this device** di atas layar; tahan mengetuk sampai lencana hilang, sehingga hasilnya mencerminkan aplikasi daripada input Anda.

<h2 id="how-sessions-manage-devices">
  Bagaimana sesi mengelola perangkat
</h2>

Setiap perangkat milik sesi yang meluncurkannya, jadi [parallel sessions](/docs/id/desktop#work-in-parallel-with-sessions) tidak berbagi perangkat: apa yang Anda lihat di pane sesi satu mencerminkan pekerjaan sesi itu, bukan sesi lain. Beralih sesi di sidebar beralih tampilan simulator bersama percakapan, dan beralih kembali melanjutkan perangkat yang sama di mana ia tertinggal. Jika Claude bekerja dengan lebih dari satu perangkat, masing-masing membuka pane-nya sendiri, hingga 4 per sesi.

Claude Code Desktop mematikan simulator yang di-boot-nya sendiri setelah tidak lagi digunakan: ketika Anda keluar dari aplikasi, ketika Anda mengarsipkan sesi, atau 10 menit setelah Anda melepas perangkat dari pane-nya. Perangkat yang Anda boot sendiri, baik dari pane atau di aplikasi Simulator Apple, tidak pernah dimatikan secara otomatis. Untuk mematikan perangkat yang terpasang segera, gunakan tombol shutdown di pane.

<h2 id="grant-claude-access-to-a-device">
  Berikan Claude akses ke perangkat
</h2>

Claude meminta persetujuan Anda sebelum mengontrol perangkat, sementara membangun aplikasi atau membuka URL di dalamnya mengikuti mode izin sesi Anda. Anda atau organisasi Anda juga dapat mematikan akses Claude sepenuhnya.

<h3 id="allow-a-device-the-first-time">
  Izinkan perangkat untuk pertama kalinya
</h3>

Pertama kali Claude menggunakan simulator, aplikasi desktop meminta Anda untuk mengizinkannya. Persetujuan mencakup mengontrol perangkat itu dan mengambil tangkapan layarnya, dan Anda memberikannya sekali per perangkat daripada sekali per sesi. Tangkapan layar Claude dari perangkat dikirim ke Anthropic dan disimpan di bawah pengaturan retensi percakapan normal Anda, jadi jangan masuk ke akun nyata di perangkat yang Claude gunakan.

Setelah Anda mengizinkan perangkat, tindakan Claude di dalamnya, seperti mengetuk, mengetik, meluncurkan aplikasi, dan mengambil tangkapan layar, berjalan tanpa prompt lebih lanjut. Mereka membawa kepercayaan yang sama seperti Anda mengklik di pane, dan mereka hanya menyentuh perangkat yang disimulasikan, jadi pane tidak memerlukan izin macOS Accessibility dan Screen Recording yang diperlukan computer use.

Jika Anda menolak, perangkat masih boot dan pane masih berfungsi untuk ketukan Anda sendiri; hanya akses Claude yang tetap mati. Untuk berubah pikiran nanti, klik **Let Claude use it** di pane.

<h3 id="actions-that-follow-your-permission-mode">
  Tindakan yang mengikuti mode izin Anda
</h3>

Dua tindakan mengikuti [permission mode](/docs/id/permissions#permission-modes) sesi Anda daripada persetujuan satu kali:

* Membuka URL di perangkat, misalnya untuk menguji deep link atau memuat halaman di Safari perangkat, karena URL dapat membawa data keluar dari perangkat.
* Membangun aplikasi, karena `xcodebuild` menjalankan skrip build proyek Anda di Mac Anda. Memeriksa build yang sudah sedang berlangsung tidak memicu prompt.

<h3 id="turn-off-simulator-access">
  Matikan akses simulator
</h3>

Anda dapat mematikan akses simulator Claude di pengaturan aplikasi desktop. Organisasi memiliki dua cara untuk mematikannya untuk semua orang:

* Pengaturan `disableMobileSimulatorTools` [managed setting](/docs/id/desktop#managed-settings) memblokir alat simulator Claude. Pane simulator tetap dapat digunakan untuk ketukan Anda sendiri, dan pengaturan tidak dapat ditimpa dari dalam aplikasi.
* Kunci kebijakan `requireCoworkFullVmSandbox`, yang menjalankan alat Claude di dalam mesin virtual terisolasi daripada di Mac Anda, menonaktifkan pane simulator dan alat simulator Claude sepenuhnya, jadi pane tidak dapat melampirkan perangkat saat diatur.

Claude memberi tahu Anda ketika salah satu berlaku.

<h2 id="limitations">
  Keterbatasan
</h2>

Claude mengendalikan perangkat yang disimulasikan saja dan tidak dapat mengontrol iPhone atau iPad fisik. Untuk menguji di satu, jalankan aplikasi di dalamnya dari Xcode sendiri, lalu jelaskan apa yang Anda lihat atau lampirkan tangkapan layar ke percakapan untuk Claude bekerja darinya.

<h2 id="troubleshooting">
  Troubleshooting
</h2>

<h3 id="the-simulator-pane-doesn’t-open-when-claude-runs-the-app">
  Pane simulator tidak terbuka ketika Claude menjalankan aplikasi
</h3>

Claude mungkin tidak mengenali bahwa Anda ingin menjalankan atau menguji aplikasi, atau alat simulator mungkin hilang. Periksa hal berikut:

* Nyatakan tujuan secara eksplisit, misalnya "jalankan aplikasi di iOS Simulator dan ketuk melalui alur pendaftaran".
* Konfirmasi Xcode dan simulator iOS terinstal dan versi Xcode Anda memenuhi [persyaratan](#requirements).
* Jika organisasi Anda mengelola Claude Code, [alat simulator mungkin dinonaktifkan oleh kebijakan](#turn-off-simulator-access).
* Jika Anda berada di organisasi Enterprise yang memiliki konfigurasi HIPAA diaktifkan, pane simulator tidak tersedia untuk Anda.
* Pane simulator memerlukan Claude Desktop v1.24012.0 atau lebih baru. Buka **Claude → Check for Updates**, lalu restart aplikasi.

<h3 id="the-simulator-pane-says-no-simulators-were-found">
  Pane simulator mengatakan tidak ada simulator yang ditemukan
</h3>

Jika `xcode-select` menunjuk ke Xcode 27, pane dapat melaporkan tidak ada simulator bahkan meskipun perangkat ada; lihat [Pane simulator gagal dengan Xcode 27](#the-simulator-pane-fails-with-xcode-27). Jika tidak, Xcode terinstal tetapi tidak memiliki simulator iOS untuk dicantumkan. Pane simulator menampilkan langkah-langkah setup untuk diikuti dan mencentangnya saat masing-masing selesai. Untuk memasang bagian yang hilang secara manual, unduh runtime simulator iOS dari pengaturan Xcode, atau jalankan `xcodebuild -downloadPlatform iOS`.

<h3 id="the-simulator-pane-fails-with-xcode-27">
  Pane simulator gagal dengan Xcode 27
</h3>

Pane belum bekerja dengan Xcode 27, yang menggantikan aplikasi Simulator dengan Device Hub. Dengan Xcode 27 dipilih, melampirkan perangkat gagal, atau pane melaporkan bahwa tidak ada simulator yang ditemukan meskipun perangkat ada.

Pane menggunakan Xcode apa pun yang `xcode-select` tunjuk. Jika Xcode 27 adalah satu-satunya instalasi Anda, instal Xcode 26.x bersama terlebih dahulu. Kemudian pilih instalasi 26.x berdasarkan jalurnya. Misalnya, jika diinstal sebagai `/Applications/Xcode-26.4.app`:

```bash theme={null}
sudo xcode-select -s /Applications/Xcode-26.4.app
```

Jalankan `xcode-select -p` untuk memeriksa instalasi mana yang dipilih.

<h2 id="see-also">
  Lihat juga
</h2>

* [Computer use in Desktop](/docs/id/desktop#let-claude-use-your-computer): kontrol layar untuk aplikasi tanpa pane khusus
* [Computer use from the CLI](/docs/id/computer-use): bagaimana CLI menjangkau iOS Simulator
* [Work in parallel with sessions](/docs/id/desktop#work-in-parallel-with-sessions): bagaimana sesi mengisolasi perubahan
* [Get started with Claude Code Desktop](/docs/id/desktop-quickstart)
