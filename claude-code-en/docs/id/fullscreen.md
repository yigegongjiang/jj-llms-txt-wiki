> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Rendering fullscreen

> Aktifkan mode rendering yang lebih halus dan bebas flicker dengan dukungan mouse dan penggunaan memori yang stabil dalam percakapan panjang.

<Note>
  Rendering fullscreen adalah [pratinjau penelitian](#research-preview). Apakah Anda [memulai dalam fullscreen atau di renderer klasik](#fullscreen-by-default) tergantung pada pengaturan Anda. Jalankan `/tui fullscreen` atau `/tui default` untuk beralih dalam percakapan Anda saat ini. Perilaku dapat berubah berdasarkan umpan balik.
</Note>

Rendering fullscreen adalah jalur rendering alternatif untuk Claude Code CLI yang menghilangkan flicker, menjaga penggunaan memori tetap datar dalam percakapan panjang, dan menambahkan dukungan mouse. Ini menggambar antarmuka pada buffer layar alternatif terminal, seperti `vim` atau `htop`, dan hanya merender pesan yang saat ini terlihat. Ini mengurangi jumlah data yang dikirim ke terminal Anda pada setiap pembaruan.

Perbedaannya paling terlihat di emulator terminal di mana throughput rendering adalah hambatan, seperti terminal terintegrasi VS Code, tmux, dan iTerm2. Jika posisi scroll terminal Anda melompat ke atas saat Claude sedang bekerja, atau layar berkedip saat output alat mengalir masuk, mode ini mengatasi masalah tersebut.

<Note>
  Istilah fullscreen menggambarkan bagaimana Claude Code mengambil alih permukaan gambar terminal, seperti yang dilakukan `vim`. Ini tidak ada hubungannya dengan memaksimalkan jendela terminal Anda, dan bekerja pada ukuran jendela apa pun.
</Note>

<h2 id="enable-fullscreen-rendering">
  Aktifkan rendering fullscreen
</h2>

Jalankan `/tui fullscreen` di dalam percakapan Claude Code apa pun. CLI menyimpan pengaturan [`tui`](/docs/id/settings-reference#tui) dan meluncurkan kembali ke fullscreen dengan percakapan Anda tetap utuh, sehingga Anda dapat beralih di tengah sesi tanpa kehilangan konteks. Jalankan `/tui default` untuk beralih kembali ke renderer klasik, atau `/tui` tanpa argumen untuk mencetak renderer mana yang aktif.

Dalam [mode pembaca layar](/docs/id/accessibility), Claude Code selalu menggunakan renderer klasik kecuali dalam [sesi latar belakang](/docs/id/agent-view) yang terpasang, yang masih merender fullscreen. Jika Anda menjalankan `/tui fullscreen` di sesi lain apa pun, Claude Code mencetak penjelasan alih-alih beralih dan tidak mengubah pengaturan `tui` yang disimpan.

Claude Code membawa ini ke sesi yang diluncurkan kembali:

* Percakapan seperti yang muncul di layar. Setelah [`/rewind`](/docs/id/checkpointing#rewind-and-summarize), itu berarti:
  * Jika Anda memutar balik lebih awal dalam sesi, Claude Code meluncurkan kembali dari titik yang diputar balik, bukan dari transkrip yang lebih panjang yang disimpan di disk. Misalnya, jika Anda memutar balik melewati tiga pesan terakhir Anda, sesi yang diluncurkan kembali terbuka tanpanya
  * Jika Anda memutar balik ke sebelum pesan pertama Anda, Claude Code meluncurkan kembali dengan percakapan kosong
* [Mode izin](/docs/id/permission-modes) dan [tingkat upaya](/docs/id/model-config#adjust-effort-level) Anda
* Model yang terakhir Anda pilih dengan [`/model`](/docs/id/model-config#setting-your-model)
* Aturan yang Anda berikan dengan [`--allowed-tools` atau `--disallowed-tools`](/docs/id/cli-reference#cli-flags), dan flag `--agent`, `--agents`, `--append-system-prompt`, dan `--system-prompt-snapshot` Anda

Claude Code menolak untuk meluncurkan kembali jika sesi memiliki pembatasan yang tidak dapat dilewatkan ke proses yang dimulai ulang. Pembatasan yang tidak dapat dilewatkan termasuk:

* Flag peluncuran seperti penggantian [`--system-prompt`](/docs/id/cli-reference#cli-flags), daftar izin [`--tools`](/docs/id/cli-reference#cli-flags), atau [`--setting-sources`](/docs/id/cli-reference#cli-flags)
* Aturan tolak atau tanya yang ditambahkan oleh [pembaruan izin hook atau SDK](/docs/id/hooks#permission-update-entries) untuk sesi ini saja

Dalam hal ini Claude Code mencetak [`Cannot switch renderers in this session`](/docs/id/errors#cannot-switch-renderers-in-this-session) dengan alasannya. Itu tidak beralih atau menyimpan apa pun.

Anda juga dapat mengatur variabel lingkungan `CLAUDE_CODE_NO_FLICKER` sebelum memulai Claude Code:

```bash theme={null}
CLAUDE_CODE_NO_FLICKER=1 claude
```

Untuk cara pengaturan [`tui`](/docs/id/settings-reference#tui) dan variabel digabungkan ketika keduanya diatur, lihat entri pengaturan. Setelah [awal fullscreen yang gagal](#fullscreen-renderer-didnt-finish-starting), Claude Code masih menghormati variabel tetapi bukan pengaturan. Perintah `/tui` menghapus `CLAUDE_CODE_NO_FLICKER` dari proses yang diluncurkan kembali sehingga pengaturan yang ditulisnya berlaku.

<h3 id="fullscreen-by-default">
  Fullscreen secara default
</h3>

[Sesi latar belakang](/docs/id/agent-view) yang terpasang merender fullscreen, dan sesi lain dalam [mode pembaca layar](/docs/id/accessibility) menggunakan renderer klasik. Jika tidak, Claude Code memulai Anda dalam renderer dari baris pertama tabel ini yang cocok dengan pengaturan Anda:

| Situasi Anda                                                                                                                                                                        | Renderer yang Anda mulai         |
| :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------- |
| Anda mengatur [`CLAUDE_CODE_DISABLE_ALTERNATE_SCREEN=1`](/docs/id/env-vars) atau `CLAUDE_CODE_NO_FLICKER=0`                                                                              | Klasik                           |
| Anda mengatur `CLAUDE_CODE_NO_FLICKER=1`                                                                                                                                            | Fullscreen                       |
| Claude Code [mematikan fullscreen setelah awal fullscreen yang gagal](#fullscreen-renderer-didnt-finish-starting) di mesin ini                                                      | Klasik                           |
| Anda berada dalam [mode integrasi `tmux -CC`](#use-with-tmux) iTerm2, atau Anda terhubung melalui SSH ke Claude Code yang berjalan di Windows                                       | Klasik                           |
| Anda menyimpan pengaturan [`tui`](/docs/id/settings-reference#tui)                                                                                                                       | Renderer yang dinamai pengaturan |
| Sesi Anda tidak [mengambil flag fitur dari Anthropic](/docs/id/env-vars#features-that-need-feature-flag-fetching), dan Claude Code telah berhenti menawarkan dialog startup di mesin ini | Klasik                           |
| Sesi Anda tidak mengambil flag fitur dari Anthropic, dan peluncuran Claude Code pertama mesin ini menjalankan v2.1.239 atau lebih baru                                              | Fullscreen                       |
| Sesi Anda mengambil flag fitur dari Anthropic, dan Anda pertama kali menggunakan Claude Code pada atau setelah 6 Mei 2026                                                           | Fullscreen                       |
| Apa pun yang lain                                                                                                                                                                   | Klasik                           |

Sesi yang tidak mengambil flag fitur termasuk yang melalui [Amazon Bedrock](/docs/id/amazon-bedrock), [Platform Agen Google Cloud](/docs/id/google-vertex-ai), atau [Microsoft Foundry](/docs/id/microsoft-foundry), dan yang memiliki telemetri dimatikan.

Jika Anda memulai dalam renderer klasik dan belum menyimpan pengaturan `tui`, Claude Code dapat membuka dialog saat startup yang menawarkan pengalihan:

* Jika Anda menerima, Claude Code meluncurkan kembali dengan cara yang sama seperti `/tui fullscreen`, membawa status sesi yang sama, dan menyimpan pengaturan setelah sesi yang diluncurkan kembali telah [dimulai dengan sukses](#fullscreen-renderer-didnt-finish-starting).
* Jika Anda memilih **Nanti**, Claude Code tidak menawarkan lagi di mesin ini.
* Claude Code berhenti menawarkan setelah menampilkan dialog pada tiga peluncuran, dijawab atau tidak.

<h2 id="what-changes">
  Apa yang berubah
</h2>

Rendering fullscreen mengubah cara CLI menggambar ke terminal Anda. Kotak input tetap berada di bagian bawah layar alih-alih bergerak saat output mengalir masuk. Jika input tetap di tempatnya saat Claude sedang bekerja, rendering fullscreen aktif. Hanya pesan yang terlihat yang disimpan di pohon render, sehingga memori tetap konstan terlepas dari panjang percakapan.

Karena percakapan berada di buffer layar alternatif alih-alih scrollback terminal Anda, beberapa hal bekerja berbeda:

| Sebelumnya                                              | Sekarang                                                                                       | Detail                                                            |
| :------------------------------------------------------ | :--------------------------------------------------------------------------------------------- | :---------------------------------------------------------------- |
| `Cmd+f` atau pencarian tmux untuk menemukan teks        | `Ctrl+o` untuk mode transkrip, kemudian `/` untuk mencari atau `[` untuk menulis ke scrollback | [Cari dan tinjau percakapan](#search-and-review-the-conversation) |
| Klik-dan-seret asli terminal untuk memilih dan menyalin | Pemilihan dalam aplikasi, menyalin secara otomatis saat pelepasan mouse                        | [Gunakan mouse](#use-the-mouse)                                   |
| `Cmd`-klik untuk membuka URL                            | `Cmd`-klik di macOS, `Ctrl`-klik di tempat lain                                                | [Gunakan mouse](#use-the-mouse)                                   |

Jika penangkapan mouse mengganggu alur kerja Anda, Anda dapat [mematikannya](#keep-native-text-selection) sambil mempertahankan rendering bebas flicker.

<h2 id="use-the-mouse">
  Gunakan mouse
</h2>

Rendering fullscreen menangkap peristiwa mouse dan menanganinya di dalam Claude Code:

* **Klik di input prompt** untuk memposisikan kursor Anda di mana saja dalam teks yang Anda ketik.
* **Klik saran dalam daftar perintah `/` atau file `@`** untuk menerimanya. Mengarahkan kursor menyoroti baris di bawah kursor Anda.
* **Klik opsi dalam menu pilih** untuk memilihnya. Ini mencakup prompt izin, `/model`, `/config`, dan dialog lainnya yang menampilkan daftar opsi. Mengarahkan kursor menunjukkan pointer pada baris di bawah kursor Anda.
* **Klik opsi dalam menu multi-pilih** untuk mengalihnya, dan klik tombol kirim untuk mengonfirmasi pilihan Anda. Mengklik baris teks bebas, seperti baris `Other` dalam pertanyaan pilihan ganda, memfokuskan bidang inputnya sehingga Anda dapat mengetik jawaban. Memerlukan Claude Code v2.1.208 atau lebih baru.
* **Klik nilai pengaturan di panel `/config`** untuk mengubahnya, dan gulir daftar pengaturan dengan roda mouse. Memerlukan Claude Code v2.1.271 atau lebih baru.
* **Gulir menu pilih atau multi-pilih dengan roda mouse** ketika memiliki lebih banyak opsi daripada yang ditampilkan sekaligus, seperti daftar `/model` di jendela terminal pendek. Roda menggulir daftar saat pointer berada di atas opsinya. Memerlukan Claude Code v2.1.280 atau lebih baru.
* **Klik hasil tool yang diciutkan** untuk memerluasnya dan melihat output lengkap. Klik lagi untuk menciutkan. Panggilan tool dan hasilnya berkembang bersama. Hanya pesan yang memiliki lebih banyak untuk ditampilkan yang dapat diklik.
  * Mengklik juga memerluaskan output dari perintah shell `!`, baik hasil yang dipotong lebih lama atau baris kemajuan langsung saat perintah berjalan. Memerlukan Claude Code v2.1.257 atau lebih baru.
* **Tahan `Cmd` di macOS, atau `Ctrl` di Linux dan Windows, dan klik URL atau jalur file** untuk membukanya. URL `http://` dan `https://` biasa terbuka di browser Anda, dan jalur file dalam output tool, seperti yang dicetak setelah Edit atau Write, terbuka di aplikasi default Anda. Klik biasa tanpa pengubah tidak membuka tautan, sesuai dengan perilaku terminal asli.
  * Claude Code merender jalur jaringan (UNC), seperti `\\server\share\file.ts`, sebagai teks biasa tanpa tautan, karena membuka jalur jaringan dapat mengirimkan kredensial Windows Anda ke host yang dinamainya.
  * Beberapa terminal macOS meneruskan `Cmd`+klik ke aplikasi yang berjalan alih-alih membuka tautan sendiri, dan protokol mouse terminal tidak memiliki cara untuk mengkodekan kunci `Cmd`, jadi Claude Code menerima klik biasa. Di Ghostty, dan di Warp di macOS, Claude Code mendeteksi ini dan membiarkan klik biasa pada tautan membukanya, dan menahan `Cmd` masih berfungsi.
  * Di terminal terintegrasi VS Code dan terminal berbasis xterm.js serupa, Claude Code menyerahkan kepada penanganan tautan terminal sendiri, yang menggunakan gestur yang sama.
* **Klik dan seret** untuk memilih teks di mana saja dalam percakapan. Klik dua kali memilih kata, sesuai dengan batas kata iTerm2 sehingga jalur file memilih sebagai satu unit. Klik dua kali pada URL memilih seluruh URL, termasuk skema. Klik tiga kali memilih baris.
* **Gulir dengan roda mouse** untuk bergerak melalui percakapan.

Teks yang dipilih disalin ke clipboard Anda secara otomatis saat pelepasan mouse. Untuk mematikannya, alihkan Copy on select di `/config`.

Dengan Copy on select dimatikan, tekan `Ctrl+Shift+c` untuk menyalin secara manual. Di terminal yang mendukung protokol keyboard kitty, seperti kitty, WezTerm, Ghostty, dan iTerm2, `Cmd+c` juga berfungsi. Jika Anda memiliki pilihan aktif, `Ctrl+c` menyalin alih-alih membatalkan.

Dengan pilihan aktif, tahan `Shift` dan tekan tombol panah untuk memperpanjangnya dari keyboard. `Shift+↑` dan `Shift+↓` menggulir viewport saat pilihan mencapai tepi atas atau bawah. `Shift+Home` dan `Shift+End` memperpanjang ke awal atau akhir baris saat ini.

Dalam tampilan prompt normal, apa yang terjadi pada pilihan aktif tergantung pada kunci yang Anda tekan:

* **`Esc`**: Claude Code melakukan tindakan biasa kunci, seperti mengganggu respons yang berjalan atau menutup dialog terbuka, dan pilihan tetap disorot.
* **`PgUp`, `PgDn`, `Ctrl+Home`, `Ctrl+End`, atau `Shift`, `Alt` atau `Option`, atau `Cmd`, `Win`, atau `Super` dengan panah, `Home`, atau kunci `End`**: pilihan tetap.
* **Kunci lainnya, termasuk tombol panah biasa, `Enter`, dan karakter yang diketik**: Claude Code menghapus pilihan.
* **Kunci yang terikat ke [`selection:clear`](/docs/id/keybindings#scroll-actions)**: Claude Code menghapus pilihan, bahkan ketika kunci adalah `Esc` atau kunci lain yang sebaliknya menyimpannya. Tindakan tidak memiliki pengikatan default.

Dalam [mode transkrip](#search-and-review-the-conversation), kunci navigasi dan pencarian yang tercantum di sana juga menyimpan pilihan.

<h2 id="scroll-the-conversation">
  Gulir percakapan
</h2>

Rendering fullscreen menangani pengguliran di dalam aplikasi. Gunakan pintasan ini untuk menavigasi:

| Pintasan        | Tindakan                                                 |
| :-------------- | :------------------------------------------------------- |
| `PgUp` / `PgDn` | Gulir ke atas atau ke bawah setengah layar               |
| `Ctrl+Home`     | Lompat ke awal percakapan                                |
| `Ctrl+End`      | Lompat ke pesan terbaru dan aktifkan kembali auto-follow |
| Roda mouse      | Gulir beberapa baris sekaligus                           |

Anda dapat menggulir kembali ke awal sesi bahkan setelah [compaction](/docs/id/context-window#what-survives-compaction). Claude terus bekerja dari ringkasan compaction, tetapi Claude Code menyimpan setiap pesan sebelumnya dalam scrollback fullscreen di seluruh compaction berulang.

Pada keyboard tanpa tombol `PgUp`, `PgDn`, `Home`, atau `End` khusus, seperti keyboard MacBook, tahan `Fn` dengan tombol panah: `Fn+↑` mengirim `PgUp`, `Fn+↓` mengirim `PgDn`, `Fn+←` mengirim `Home`, dan `Fn+→` mengirim `End`. `Ctrl+Fn+→` tidak menjangkau Claude Code di macOS, jadi keyboard MacBook tidak memiliki chord jump-to-bottom yang berfungsi secara default. Sebagai gantinya, gunakan salah satu opsi ini:

* Klik [tombol jump-to-bottom](#auto-follow).
* Gulir ke bawah dengan roda mouse untuk melanjutkan mengikuti.
* Ikat ulang `scroll:bottom` ke chord yang dapat dikirim keyboard Anda.

Tindakan ini dapat diikat ulang. Lihat [Scroll actions](/docs/id/keybindings#scroll-actions) untuk daftar lengkap nama tindakan, termasuk varian half-page dan full-page yang tidak memiliki binding default.

Saat Anda menggulir ke atas, baris header yang redup di bagian atas percakapan menampilkan prompt terbaru yang telah menggulir di atas tampilan. Klik baris untuk melompat ke prompt tersebut.

<h3 id="auto-follow">
  Auto-follow
</h3>

Menggulir ke atas menghentikan auto-follow sehingga output baru tidak menarik Anda kembali ke bawah. Tombol `Jump to bottom` mengambang di atas tepi bawah transkrip saat Anda menggulir ke atas, dan menampilkan hitungan seperti `3 new messages` ketika output baru tiba. Kliknya, tekan `Ctrl+End`, atau gulir ke bawah untuk melanjutkan mengikuti.

Saat auto-follow dijeda, tampilan juga tetap berada di tempat Anda menggulirnya ketika respons selesai streaming.

Petunjuk keyboard tombol mencerminkan apa yang dapat dikirim keyboard Anda. Di macOS, ini menyarankan untuk mengklik, atau `Fn+↓` untuk menggulir, karena `Ctrl+End` tidak menjangkau Claude Code dari keyboard Mac. Ikat ulang [`scroll:bottom`](/docs/id/keybindings#scroll-actions) dan tombol menampilkan chord Anda di setiap platform.

Pada terminal yang terlalu sempit untuk label lengkap, tombol mempersingkat petunjuk alih-alih membungkus ke baris transkrip di bawahnya.

Untuk mematikan auto-follow sepenuhnya sehingga tampilan tetap berada di tempat Anda meninggalkannya, buka `/config` dan atur Auto-scroll ke off. Dengan auto-scroll dinonaktifkan, tampilan tidak pernah melompat ke bawah dengan sendirinya. Prompt izin dan dialog lainnya yang memerlukan respons tetap bergulir ke dalam tampilan terlepas dari pengaturan ini.

<h3 id="mouse-wheel-scrolling">
  Pengguliran roda mouse
</h3>

Pengguliran roda mouse memerlukan terminal Anda untuk meneruskan peristiwa mouse ke Claude Code. Sebagian besar terminal melakukan ini setiap kali aplikasi memintanya. iTerm2 menjadikannya pengaturan per-profil: jika roda tidak melakukan apa pun tetapi `PgUp` dan `PgDn` berfungsi, buka Settings → Profiles → Terminal dan aktifkan Enable mouse reporting. Pengaturan yang sama juga diperlukan agar click-to-expand dan text selection berfungsi.

Jika pengguliran roda mouse terasa lambat, terminal Anda mungkin mengirim satu peristiwa scroll per takik fisik tanpa pengganda. Beberapa terminal, seperti Ghostty dan iTerm2 dengan scrolling lebih cepat diaktifkan, sudah memperkuat peristiwa roda. Yang lain, termasuk terminal terintegrasi VS Code, mengirim tepat satu peristiwa per takik. Claude Code tidak dapat mendeteksi mana.

Atur `CLAUDE_CODE_SCROLL_SPEED` untuk mengalikan jarak scroll dasar:

```bash theme={null}
export CLAUDE_CODE_SCROLL_SPEED=3
```

Nilai `3` cocok dengan default di `vim` dan aplikasi serupa. Pengaturan menerima nilai positif apa pun hingga 20, termasuk nilai pecahan di bawah 1 seperti `0.25` untuk memperlambat pengguliran trackpad dan roda yang dipercepat di terminal yang sudah memperkuat peristiwa roda.

Untuk menyesuaikan kecepatan scroll secara interaktif, jalankan `/scroll-speed`. Dialog menampilkan penggaris yang dapat Anda gulir saat terbuka sehingga Anda dapat merasakan perubahannya segera. Tekan `←` dan `→` untuk menyesuaikan kecepatan, `r` untuk mengatur ulang ke default yang terdeteksi otomatis, dan `Enter` untuk menyimpan. Dialog melangkah dalam angka bulat hingga 10, dan di terminal yang mendukung kontrol lebih halus, dialog juga menawarkan langkah seperempat hingga 0,25.

Perintah menulis nilai yang sama yang ditetapkan variabel lingkungan `CLAUDE_CODE_SCROLL_SPEED`, disimpan ke `~/.claude/settings.json`. Maksimum dialog adalah 10: jika Anda menetapkan nilai lebih tinggi melalui variabel lingkungan, dialog menampilkan 10, dan menyimpan dari dialog menyimpan 10. Perintah tidak tersedia di terminal IDE JetBrains.

Terpisah dari kecepatan dasar, Claude Code mempercepat laju scroll ketika Anda memutar roda dengan cepat, sehingga putaran cepat mencakup jarak lebih jauh daripada jumlah takik lambat yang sama. Untuk mematikan akselerasi dan mempertahankan laju konstan per takik, atur `wheelScrollAccelerationEnabled` ke `false` di [`settings.json`](/docs/id/settings-reference#all-settings). Pengaturan ini memerlukan Claude Code v2.1.174 atau lebih baru.

<h3 id="scroll-in-the-jetbrains-ide-terminal">
  Gulir di terminal IDE JetBrains
</h3>

Di terminal IDE JetBrains, Claude Code menerapkan penanganan scroll sendiri dan mengabaikan `CLAUDE_CODE_SCROLL_SPEED`. Terminal mengirim peristiwa scroll pada laju yang jauh lebih tinggi daripada emulator lain, sehingga pengganda yang disesuaikan di tempat lain melampaui di sini.

Di 2025.2, terminal juga memiliki bug scroll-wheel yang menghasilkan tombol panah palsu dan peristiwa arah yang salah. Claude Code mendeteksi ini saat runtime dan menguranginya secara otomatis, sehingga pengguliran trackpad dan roda mouse berfungsi tanpa konfigurasi. Untuk pengalaman scroll terbaik, tingkatkan ke 2025.3 atau lebih baru. Claude Code menampilkan petunjuk pertama kali Anda menggulir jika mendeteksi bug.

<h2 id="search-and-review-the-conversation">
  Cari dan tinjau percakapan
</h2>

`Ctrl+o` mengalihkan antara prompt normal dan mode transkrip.

Untuk tampilan yang lebih tenang yang menampilkan hanya prompt terakhir Anda, ringkasan satu baris dari panggilan alat dengan statistik edit diff, dan respons akhir, jalankan `/focus`. Pengaturan ini bertahan di seluruh sesi. Jalankan `/focus` lagi untuk mematikannya.

Mode transkrip mendapatkan navigasi dan pencarian gaya `less`:

| Tombol                                 | Tindakan                                                                                                                             |
| :------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------- |
| `/`                                    | Buka pencarian. Ketik untuk menemukan kecocokan, `Enter` untuk menerima, `Esc` untuk membatalkan dan mengembalikan posisi gulir Anda |
| `n` / `N`                              | Lompat ke kecocokan berikutnya atau sebelumnya. Bekerja setelah Anda menutup bilah pencarian                                         |
| `j` / `k` atau `↑` / `↓`               | Gulir satu baris                                                                                                                     |
| `g` / `G` atau `Home` / `End`          | Lompat ke atas atau bawah                                                                                                            |
| `{` / `}`                              | Lompat ke prompt sebelumnya atau berikutnya                                                                                          |
| `Ctrl+u` / `Ctrl+d`                    | Gulir setengah halaman                                                                                                               |
| `Ctrl+b` / `Ctrl+f` atau `Space` / `b` | Gulir satu halaman penuh                                                                                                             |
| `Ctrl+o`, `Esc`, atau `q`              | Keluar dari mode transkrip dan kembali ke prompt                                                                                     |

`Cmd+f` terminal Anda dan pencarian tmux tidak melihat percakapan karena percakapan tersebut berada di buffer layar alternatif, bukan scrollback asli. Untuk mengembalikan konten ke terminal Anda, tekan `Ctrl+o` untuk memasuki mode transkrip terlebih dahulu, kemudian:

* **`[`**: menulis percakapan lengkap ke buffer scrollback asli terminal Anda, dengan semua output alat diperluas. Percakapan sekarang adalah teks biasa di terminal Anda, jadi `Cmd+f`, mode salinan tmux, dan alat asli lainnya dapat mencari atau memilihnya. Sesi yang panjang mungkin berhenti sejenak saat ini terjadi. Ini berlangsung sampai Anda keluar dari mode transkrip dengan `Esc` atau `q`, yang mengembalikan Anda ke rendering layar penuh. `Ctrl+o` berikutnya dimulai dari awal.
* **`v`**: menulis percakapan ke file sementara dan membukanya di `$VISUAL` atau `$EDITOR`.

<h2 id="watch-your-changes-in-the-diff-panel">
  Tonton perubahan Anda di panel diff
</h2>

Dalam rendering layar penuh, [`/diff`](/docs/id/interactive-mode#review-changes-with-%2Fdiff) membuka panel di samping percakapan daripada penampil yang harus Anda tutup, sehingga Anda dapat menonton perubahan terakumulasi saat Claude bekerja. Di terminal yang lebar, panel juga dapat terbuka dengan sendirinya setelah Claude mulai mengedit file. [Panel diff](/docs/id/interactive-mode#diff-panel) mencakup apa yang ditampilkannya, cara menjaganya tetap tertutup, dan cara mengubah apa yang dibandingkannya.

<h2 id="clear-the-conversation">
  Hapus percakapan
</h2>

Jalankan `/clear` untuk memulai percakapan baru.

Jika tampilan terlihat berantakan atau sebagian kosong, tekan `Ctrl+L` untuk menggambar ulang layar. Penggambaran ulang menjaga percakapan dan input Anda tetap di tempatnya.

`Cmd+K` melakukan hal yang sama dengan `Ctrl+L` ketika terminal Anda meneruskannya ke Claude Code. iTerm2 dan Terminal.app menangani `Cmd+K` sendiri dan menghapus layar mereka sendiri, dan Claude Code mendeteksi layar yang dihapus dan melukis ulang percakapan. Sebelum v2.1.280, dimulai dengan v2.1.260, menekan `Ctrl+L`, atau `Cmd+K` di mana pun mencapai Claude Code, menghapus layar dalam rendering layar penuh. Sebelum v2.1.238, menekan `Ctrl+L` dua kali dalam dua detik menjalankan `/clear`.

<h2 id="use-with-tmux">
  Gunakan dengan tmux
</h2>

Rendering fullscreen berfungsi di dalam tmux, dengan tiga peringatan.

Scrolling roda mouse memerlukan mode mouse tmux. Jika `~/.tmux.conf` Anda belum mengaktifkannya, tambahkan baris ini dan muat ulang konfigurasi Anda:

```bash theme={null}
set -g mouse on
```

Tanpa mode mouse, peristiwa roda pergi ke tmux alih-alih Claude Code. Scrolling keyboard dengan `PgUp` dan `PgDn` berfungsi baik cara. Claude Code mencetak petunjuk satu kali saat startup jika mendeteksi tmux dengan mode mouse mati.

Rendering fullscreen tidak kompatibel dengan mode integrasi tmux iTerm2, yang merupakan mode yang Anda masuki dengan `tmux -CC`. Dalam mode integrasi, iTerm2 merender setiap pane tmux sebagai split native daripada membiarkan tmux menggambar ke terminal. Buffer layar alternatif dan pelacakan mouse tidak berfungsi dengan benar di sana: roda mouse tidak melakukan apa pun, dan double-click dapat merusak status terminal. Jangan aktifkan rendering fullscreen dalam sesi `tmux -CC`. tmux reguler di dalam iTerm2, tanpa `-CC`, berfungsi dengan baik.

Rilis tmux melalui seri 3.6 tidak mengimplementasikan synchronized output, jadi di bawah versi tersebut Anda mungkin melihat lebih banyak flicker selama redraw daripada saat menjalankan Claude Code langsung di terminal Anda. Claude Code menyelidiki terminal untuk dukungan synchronized-output saat startup dan menggunakannya ketika terminal melaporkannya. Jika Anda melihat flicker di bawah tmux, upgrade ke tmux terbaru atau jalankan Claude Code di tab terminal sendiri di luar tmux.

<h2 id="keep-native-text-selection">
  Pertahankan pemilihan teks asli
</h2>

Penangkapan mouse adalah titik gesekan paling umum, terutama melalui SSH atau di dalam tmux. Ketika Claude Code menangkap peristiwa mouse, pemilihan asli terminal Anda yang berhenti saat disalin tidak lagi berfungsi. Pemilihan yang Anda buat dengan klik-dan-seret ada di dalam Claude Code, bukan di buffer pemilihan terminal Anda, jadi mode salin tmux, petunjuk Kitty, dan alat serupa tidak melihatnya.

Claude Code menulis pemilihan ke clipboard sistem Anda, dan jalur yang digunakan tergantung pada pengaturan Anda. Pada sesi lokal, ia menjalankan alat clipboard asli:

* **macOS**: `pbcopy`
* **Linux**: `wl-copy` di Wayland, atau `xclip` atau `xsel` di X11, mana pun yang terinstal. Claude Code menulis baik clipboard maupun pemilihan PRIMARY, jadi tempel tengah-klik berfungsi.
* **Windows dan WSL**: PowerShell `Set-Clipboard`

Di dalam tmux, ia juga menulis ke buffer tempel tmux. Melalui SSH, ia kembali ke urutan escape OSC 52. Di dalam GNU screen, Claude Code menyalin pemilihan panjang ke clipboard juga. Sebelum v2.1.219, jika Anda menyalin pemilihan lebih panjang dari kira-kira 570 karakter, GNU screen mencetak teks base64 ke dalam jendela sebagai gantinya. Claude Code mencetak toast setelah setiap salinan memberi tahu Anda jalur mana yang digunakan.

Beberapa terminal memblokir OSC 52 secara default. iTerm2 memblokirnya sampai Anda mengaktifkan Settings → General → Selection → Applications in terminal may access clipboard; menjalankan [`/terminal-setup`](/docs/id/terminal-config) di iTerm2 mengaktifkan ini untuk Anda.

Untuk pemilihan asli sekali jalan, kunci yang digunakan tergantung pada terminal Anda:

* **Terminal.app**: `Fn`
* **iTerm2**: `Option`
* **VS Code, Cursor, dan Devin Desktop**: `Shift`, atau `Option` di macOS dengan pengaturan `terminal.integrated.macOptionClickForcesSelection` diaktifkan
* **Sebagian besar terminal lainnya**: `Shift`

Tahan kunci itu sambil Anda klik dan seret. Terminal Anda menangani pemilihan itu sendiri alih-alih meneruskannya ke Claude Code, jadi pintasan salin seperti `Cmd+C` bekerja pada apa yang Anda pilih. Claude Code juga menampilkan kunci yang benar dalam petunjuk di layarnya.

Melalui SSH atau di dalam tmux, Claude Code tidak selalu dapat mendeteksi terminal yang Anda hubungkan, jadi petunjuk mencantumkan kunci kandidat sebagai gantinya.

Jika Anda mengandalkan pemilihan asli sepanjang waktu, atur `CLAUDE_CODE_DISABLE_MOUSE=1` untuk keluar dari penangkapan mouse sambil mempertahankan rendering tanpa kedip dan memori datar:

```bash theme={null}
CLAUDE_CODE_NO_FLICKER=1 CLAUDE_CODE_DISABLE_MOUSE=1 claude
```

Dengan penangkapan mouse dinonaktifkan, pengguliran keyboard dengan `PgUp`, `PgDn`, `Ctrl+Home`, dan `Ctrl+End` masih berfungsi, dan terminal Anda menangani pemilihan secara asli. Anda kehilangan klik-ke-posisi-kursor, klik-untuk-memperluas-keluaran-alat, klik URL, dan pengguliran roda di dalam Claude Code.

Untuk mempertahankan pengguliran roda tetapi mematikan penanganan klik, seret, dan hover, atur `CLAUDE_CODE_DISABLE_MOUSE_CLICKS=1` sebagai gantinya. Memerlukan Claude Code v2.1.195 atau lebih baru. `CLAUDE_CODE_DISABLE_MOUSE` memiliki prioritas ketika kedua variabel diatur.

Dengan klik dinonaktifkan, Claude Code masih menangkap mouse, jadi roda dan touchpad menggulir percakapan tetapi klik kiri tidak melakukan apa pun di dalam Claude Code. Anda masih perlu menahan kunci terminal Anda untuk pemilihan klik-dan-seret asli. Klik kanan dan tempel tengah-klik terus berfungsi di terminal yang mendukungnya.

<h2 id="troubleshooting">
  Troubleshooting
</h2>

<h3 id="stale-or-misplaced-text-on-screen">
  Teks basi atau tidak pada tempatnya di layar
</h3>

Rendering fullscreen mengirimkan hanya sel yang berubah antar frame. Beberapa terminal, paling umum Windows Terminal dan host berbasis ConPTY lainnya, menggabungkan penulisan yang diposisikan ini secara tidak benar dan meninggalkan fragmen output sebelumnya di layar sampai Anda mengubah ukuran jendela.

Atur [`CLAUDE_CODE_ALT_SCREEN_FULL_REPAINT=1`](/docs/id/env-vars) untuk mengecat ulang setiap sel pada setiap frame alih-alih mengirimkan pembaruan inkremental.

Di Windows PowerShell:

```powershell theme={null}
$env:CLAUDE_CODE_ALT_SCREEN_FULL_REPAINT = "1"
claude
```

Di macOS atau Linux:

```bash theme={null}
CLAUDE_CODE_ALT_SCREEN_FULL_REPAINT=1 claude
```

Di Windows, Claude Code sudah mengaktifkan pengecatan ulang penuh secara otomatis untuk sesi latar belakang dan [tampilan agen](/docs/id/agent-view), jadi Anda hanya perlu mengatur variabel untuk sesi fullscreen interaktif yang Anda luncurkan secara langsung.

<h3 id="fullscreen-renderer-didnt-finish-starting">
  `Claude Code's fullscreen renderer didn't finish starting last time` muncul saat startup
</h3>

Jika sesi fullscreen di mesin ini mogok sebelum berhasil dimulai, Claude Code memulai sesi berikutnya Anda di renderer klasik dan mencetak salah satu dari dua baris. Sesi telah dimulai dengan berhasil setelah menggambar frame pertamanya dan kemudian tetap aktif selama 10 detik atau Anda mengakhirinya dengan `/exit`, Ctrl+C, atau Ctrl+D. Baris yang Anda lihat memberi tahu Anda apa yang Claude Code lakukan setelah sesi ini:

* Setelah satu kali gagal dimulai, Anda melihat `Claude Code's fullscreen renderer didn't finish starting last time on this machine`. Claude Code mencoba rendering fullscreen lagi di sesi berikutnya yang Anda mulai
* Setelah dua kali gagal dimulai, Anda melihat `Claude Code's fullscreen renderer has repeatedly failed to start on this machine`. Claude Code terus menggunakan renderer klasik sampai Anda memperbarui Claude Code atau menjalankan `/tui fullscreen`, dan tidak mencetak apa pun di sesi-sesi tersebut nanti

Untuk mengonfirmasi bahwa kegagalan dimulai adalah alasan Anda berada di renderer klasik, jalankan `/tui` tanpa argumen. Sementara kegagalan dimulai adalah alasannya, baris `Current renderer` mengatakan demikian.

Untuk tetap menggunakan renderer klasik, jalankan `/tui default`, yang menyimpan pengaturan `tui` tanpa meluncurkan ulang. Untuk mencoba rendering fullscreen lagi, jalankan `/tui fullscreen`. Jika sesi itu juga tidak selesai dimulai, [laporkan masalahnya](#research-preview).

Sebelum v2.1.236, Claude Code terus memulai sesi dalam rendering fullscreen setelah kegagalan dimulai.

<h4 id="how-claude-code-counts-failed-starts">
  Bagaimana Claude Code menghitung kegagalan dimulai
</h4>

* Sesi yang dihitung: hanya sesi yang dimulai dalam rendering fullscreen karena pengaturan `tui` Anda mengatakan demikian, karena Anda menerima [dialog startup](#fullscreen-by-default), atau karena Claude Code memulai Anda di fullscreen secara default
* `CLAUDE_CODE_NO_FLICKER=1`: jika Anda mengaturnya, Claude Code merender sesi itu fullscreen bahkan setelah kegagalan dimulai, dan tidak menghitungnya
* Penghitungan ulang: Claude Code menghitung kegagalan dimulai per versi Claude Code, dan awal fullscreen yang berhasil mengatur ulang penghitungan
* Dialog startup: jika Anda menerima dialog dan sesi yang diluncurkan ulang mogok, Claude Code tidak mencetak baris apa pun dan tidak menampilkan dialog lagi di versi Claude Code ini

<h2 id="research-preview">
  Pratinjau penelitian
</h2>

Rendering fullscreen adalah fitur pratinjau penelitian. Fitur ini telah diuji pada emulator terminal umum, tetapi Anda mungkin mengalami masalah rendering pada terminal yang kurang umum atau konfigurasi yang tidak biasa.

Jika Anda mengalami masalah, jalankan `/feedback` di dalam Claude Code untuk melaporkannya, atau buka issue di [repositori GitHub claude-code](https://github.com/anthropics/claude-code/issues). Sertakan nama dan versi emulator terminal Anda.

Untuk mematikan rendering fullscreen, jalankan `/tui default`, atau batalkan pengaturan `CLAUDE_CODE_NO_FLICKER` jika Anda mengaktifkannya dengan cara itu. Ketika Anda beralih kembali dengan `/tui default`, Claude Code mungkin pertama kali menampilkan prompt feedback opsional yang menanyakan apa yang membuat Anda beralih. Ketik alasan dan tekan `Enter` untuk mengirimnya, atau tekan `Esc` untuk melewatkan. CLI diluncurkan kembali ke renderer klasik bagaimanapun juga. Untuk memaksa renderer klasik terlepas dari pengaturan `tui` yang disimpan, atur `CLAUDE_CODE_DISABLE_ALTERNATE_SCREEN=1`. Renderer klasik menjaga percakapan di scrollback asli terminal Anda sehingga `Cmd+f` dan tmux copy mode berfungsi seperti biasanya.

Sesi latar belakang yang dibuka dari [tampilan agen](/docs/id/agent-view) atau `claude attach` selalu menggunakan rendering fullscreen. Terminal yang melampirkan memasuki buffer layar alternatif untuk menampilkan sesi, dan renderer klasik tidak memiliki scrollback atau penanganan mouse di sana, jadi pengaturan `tui` dan `CLAUDE_CODE_DISABLE_ALTERNATE_SCREEN` tidak berlaku untuk mereka.
