> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Rendering fullscreen

> Aktifkan mode rendering yang lebih halus dan bebas flicker dengan dukungan mouse dan penggunaan memori yang stabil dalam percakapan panjang.

<Note>
  Rendering fullscreen adalah pratinjau penelitian yang bersifat opt-in. Jalankan `/tui fullscreen` untuk beralih dalam percakapan Anda saat ini. Perilaku dapat berubah berdasarkan umpan balik.
</Note>

Rendering fullscreen adalah jalur rendering alternatif untuk Claude Code CLI yang menghilangkan flicker, menjaga penggunaan memori tetap datar dalam percakapan panjang, dan menambahkan dukungan mouse. Ini menggambar antarmuka pada buffer layar alternatif terminal, seperti `vim` atau `htop`, dan hanya merender pesan yang saat ini terlihat. Ini mengurangi jumlah data yang dikirim ke terminal Anda pada setiap pembaruan.

Perbedaannya paling terlihat di emulator terminal di mana throughput rendering adalah hambatan, seperti terminal terintegrasi VS Code, tmux, dan iTerm2. Jika posisi scroll terminal Anda melompat ke atas saat Claude sedang bekerja, atau layar berkedip saat output alat mengalir masuk, mode ini mengatasi masalah tersebut.

<Note>
  Istilah fullscreen menggambarkan bagaimana Claude Code mengambil alih permukaan gambar terminal, seperti yang dilakukan `vim`. Ini tidak ada hubungannya dengan memaksimalkan jendela terminal Anda, dan bekerja pada ukuran jendela apa pun.
</Note>

<h2 id="enable-fullscreen-rendering">
  Aktifkan rendering fullscreen
</h2>

Jalankan `/tui fullscreen` di dalam percakapan Claude Code apa pun. CLI menyimpan pengaturan [`tui`](/docs/id/settings#available-settings) dan meluncurkan kembali ke fullscreen dengan percakapan Anda tetap utuh, sehingga Anda dapat beralih di tengah sesi tanpa kehilangan konteks. Jalankan `/tui default` untuk beralih kembali ke renderer klasik, atau `/tui` tanpa argumen untuk mencetak renderer mana yang aktif.

Sesi yang diluncurkan kembali mempertahankan percakapan seperti yang muncul di layar. Jika Anda menjalankan [`/rewind`](/docs/id/checkpointing#rewind-and-summarize) sebelumnya dalam sesi, peluncuran kembali dilanjutkan dari titik yang telah diputar balik daripada transkrip yang lebih panjang yang disimpan di disk. Sebelum v2.1.207, beralih renderer setelah rewind memulihkan percakapan yang telah dihapus oleh rewind.

Anda juga dapat mengatur variabel lingkungan `CLAUDE_CODE_NO_FLICKER` sebelum memulai Claude Code:

```bash theme={null}
CLAUDE_CODE_NO_FLICKER=1 claude
```

Pengaturan `tui` dan variabel lingkungan adalah setara. Perintah `/tui` menghapus `CLAUDE_CODE_NO_FLICKER` dari proses yang diluncurkan kembali sehingga pengaturan yang ditulisnya berlaku.

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
* **Klik opsi dalam menu pilih** untuk memilihnya. Ini mencakup prompt izin, `/model`, `/config`, dan dialog lainnya yang menampilkan daftar opsi. Mengarahkan kursor menunjukkan pointer pada baris di bawah kursor Anda. Memerlukan Claude Code v2.1.187 atau lebih baru.
* **Klik opsi dalam menu multi-pilih** untuk mengalihkannya, dan klik tombol kirim untuk mengonfirmasi pilihan Anda. Mengklik baris teks bebas, seperti baris `Other` dalam pertanyaan pilihan ganda, memfokuskan bidang inputnya sehingga Anda dapat mengetik jawaban. Memerlukan Claude Code v2.1.208 atau lebih baru.
* **Klik hasil alat yang diciutkan** untuk memperluasnya dan melihat output lengkap. Klik lagi untuk menciutkan. Panggilan alat dan hasilnya berkembang bersama. Hanya pesan yang memiliki lebih banyak untuk ditampilkan yang dapat diklik.
* **Tahan `Cmd` di macOS, atau `Ctrl` di Linux dan Windows, dan klik URL atau jalur file** untuk membukanya. Jalur file dalam output alat, seperti yang dicetak setelah Edit atau Write, terbuka di aplikasi default Anda. URL `http://` dan `https://` biasa terbuka di browser Anda. Mulai dari v2.1.181, klik biasa tanpa menahan `Cmd` atau `Ctrl` tidak lagi membuka tautan, sesuai dengan perilaku terminal asli. Beberapa terminal macOS meneruskan `Cmd`+click ke aplikasi yang berjalan alih-alih membuka tautan sendiri, dan protokol mouse terminal tidak memiliki cara untuk mengenkode kunci `Cmd`, jadi Claude Code menerimanya sebagai klik biasa. Di Ghostty, dan mulai dari v2.1.198 di Warp di macOS, Claude Code mendeteksi ini dan membiarkan klik biasa pada tautan membukanya, dan menahan `Cmd` masih berfungsi. Di terminal terintegrasi VS Code dan terminal berbasis xterm.js serupa, Claude Code menunda ke penanganan tautan terminal sendiri, yang menggunakan gestur yang sama.
* **Klik dan seret** untuk memilih teks di mana saja dalam percakapan. Klik ganda memilih kata, mencocokkan batas kata iTerm2 sehingga jalur file memilih sebagai satu unit. Mulai dari v2.1.198, klik ganda pada URL memilih seluruh URL, termasuk skema. Klik tiga kali memilih baris.
* **Gulir dengan roda mouse** untuk bergerak melalui percakapan.

Teks yang dipilih disalin ke clipboard Anda secara otomatis saat pelepasan mouse. Untuk mematikan ini, alihkan Copy on select di `/config`.

Dengan Copy on select dimatikan, tekan `Ctrl+Shift+c` untuk menyalin secara manual. Di terminal yang mendukung protokol keyboard kitty, seperti kitty, WezTerm, Ghostty, dan iTerm2, `Cmd+c` juga berfungsi. Jika Anda memiliki pemilihan aktif, `Ctrl+c` menyalin alih-alih membatalkan.

Dengan pemilihan aktif, tahan `Shift` dan tekan tombol panah untuk memperluas dari keyboard. `Shift+↑` dan `Shift+↓` menggulir viewport saat pemilihan mencapai tepi atas atau bawah. `Shift+Home` dan `Shift+End` memperluas ke awal atau akhir baris saat ini.

<h2 id="scroll-the-conversation">
  Gulir percakapan
</h2>

Rendering fullscreen menangani scrolling di dalam aplikasi. Gunakan pintasan ini untuk menavigasi:

| Pintasan        | Tindakan                                                 |
| :-------------- | :------------------------------------------------------- |
| `PgUp` / `PgDn` | Gulir naik atau turun setengah layar                     |
| `Ctrl+Home`     | Lompat ke awal percakapan                                |
| `Ctrl+End`      | Lompat ke pesan terbaru dan aktifkan kembali auto-follow |
| Roda mouse      | Gulir beberapa baris sekaligus                           |

Pada keyboard tanpa tombol `PgUp`, `PgDn`, `Home`, atau `End` khusus, seperti keyboard MacBook, tahan `Fn` dengan tombol panah: `Fn+↑` mengirim `PgUp`, `Fn+↓` mengirim `PgDn`, `Fn+←` mengirim `Home`, dan `Fn+→` mengirim `End`. `Ctrl+Fn+→` tidak menjangkau Claude Code di macOS, jadi keyboard MacBook tidak memiliki chord lompat-ke-bawah yang berfungsi secara default. Sebagai gantinya, gunakan salah satu opsi ini:

* Klik [tombol jump-to-bottom](#auto-follow).
* Gulir ke bawah dengan roda mouse untuk melanjutkan mengikuti.
* Ikat ulang `scroll:bottom` ke chord yang dapat dikirim keyboard Anda.

Tindakan ini dapat diikat ulang. Lihat [Scroll actions](/docs/id/keybindings#scroll-actions) untuk daftar lengkap nama tindakan, termasuk varian setengah halaman dan halaman penuh yang tidak memiliki pengikatan default.

<h3 id="auto-follow">
  Auto-follow
</h3>

Menggulir naik menjeda auto-follow sehingga output baru tidak menarik Anda kembali ke bawah. Tombol `Jump to bottom` mengapung di atas tepi bawah transkrip saat Anda menggulir naik, dan menampilkan hitungan seperti `3 new messages` ketika output baru tiba. Klik tombol tersebut, tekan `Ctrl+End`, atau gulir ke bawah untuk melanjutkan mengikuti.

Saat auto-follow dijeda, tampilan juga tetap di mana Anda menggulirnya ketika respons selesai streaming. Sebelum v2.1.207, tampilan dapat melompat di atas awal jawaban ketika respons panjang selesai streaming.

Petunjuk keyboard tombol mencerminkan apa yang dapat dikirim keyboard Anda. Di macOS, tombol menyarankan untuk mengklik, atau `Fn+↓` untuk menggulir, karena `Ctrl+End` tidak menjangkau Claude Code dari keyboard Mac. Ikat ulang [`scroll:bottom`](/docs/id/keybindings#scroll-actions) dan tombol menampilkan chord Anda di setiap platform. Sebelum v2.1.206, tombol menyarankan `Ctrl+End` di macOS.

Pada terminal yang terlalu sempit untuk label lengkap, tombol memendekkan petunjuk alih-alih membungkus ke baris transkrip di bawahnya. Sebelum v2.1.206, label panjang dapat membungkus di atas transkrip.

Untuk mematikan auto-follow sepenuhnya sehingga tampilan tetap di mana Anda meninggalkannya, buka `/config` dan atur Auto-scroll ke off. Dengan auto-scroll dinonaktifkan, tampilan tidak pernah melompat ke bawah dengan sendirinya. Prompt izin dan dialog lainnya yang memerlukan respons masih menggulir ke tampilan terlepas dari pengaturan ini.

<h3 id="mouse-wheel-scrolling">
  Scrolling roda mouse
</h3>

Scrolling roda mouse memerlukan terminal Anda untuk meneruskan peristiwa mouse ke Claude Code. Sebagian besar terminal melakukan ini setiap kali aplikasi memintanya. iTerm2 menjadikannya pengaturan per-profil: jika roda tidak melakukan apa pun tetapi `PgUp` dan `PgDn` berfungsi, buka Settings → Profiles → Terminal dan aktifkan Enable mouse reporting. Pengaturan yang sama juga diperlukan untuk klik-untuk-memperluas dan pemilihan teks agar berfungsi.

Jika scrolling roda mouse terasa lambat, terminal Anda mungkin mengirim satu peristiwa scroll per takik fisik tanpa pengganda. Beberapa terminal, seperti Ghostty dan iTerm2 dengan scrolling lebih cepat diaktifkan, sudah memperkuat peristiwa roda. Yang lain, termasuk terminal terintegrasi VS Code, mengirim tepat satu peristiwa per takik. Claude Code tidak dapat mendeteksi mana.

Atur `CLAUDE_CODE_SCROLL_SPEED` untuk mengalikan jarak scroll dasar:

```bash theme={null}
export CLAUDE_CODE_SCROLL_SPEED=3
```

Nilai `3` cocok dengan default di `vim` dan aplikasi serupa. Pengaturan menerima nilai dari 1 hingga 20, dan nilai fraksional di bawah 1 seperti `0.5` untuk memperlambat scrolling trackpad dan roda yang dipercepat di terminal yang sudah memperkuat peristiwa roda.

Untuk menyesuaikan kecepatan scroll secara interaktif, jalankan `/scroll-speed`. Dialog menampilkan penggaris yang dapat Anda gulir saat terbuka sehingga Anda dapat merasakan perubahan segera. Tekan `←` dan `→` untuk menyesuaikan, `r` untuk mengatur ulang ke default yang terdeteksi otomatis, dan `Enter` untuk menyimpan.

Perintah menulis nilai yang sama yang ditetapkan variabel lingkungan `CLAUDE_CODE_SCROLL_SPEED`, disimpan ke `~/.claude/settings.json`. Perintah tidak tersedia di terminal IDE JetBrains.

Terpisah dari kecepatan dasar, Claude Code mempercepat laju scroll ketika Anda memutar roda dengan cepat, sehingga putaran cepat mencakup jarak lebih jauh daripada jumlah takik lambat yang sama. Untuk mematikan akselerasi dan mempertahankan laju konstan per takik, atur `wheelScrollAccelerationEnabled` ke `false` di [`settings.json`](/docs/id/settings#available-settings). Pengaturan ini memerlukan Claude Code v2.1.174 atau lebih baru.

<h3 id="scroll-in-the-jetbrains-ide-terminal">
  Scroll di terminal IDE JetBrains
</h3>

Di terminal IDE JetBrains, Claude Code menerapkan penanganan scroll-nya sendiri dan mengabaikan `CLAUDE_CODE_SCROLL_SPEED`. Terminal mengirim peristiwa scroll pada tingkat yang jauh lebih tinggi daripada emulator lain, sehingga pengganda yang disesuaikan di tempat lain melampaui di sini.

Di 2025.2, terminal juga memiliki bug scroll-wheel yang menghasilkan tombol panah palsu dan peristiwa arah yang salah. Claude Code mendeteksi ini pada waktu runtime dan menguranginya secara otomatis, sehingga scrolling trackpad dan roda mouse berfungsi tanpa konfigurasi. Untuk pengalaman scroll terbaik, tingkatkan ke 2025.3 atau lebih baru. Claude Code menampilkan petunjuk pertama kali Anda scroll jika mendeteksi bug.

<h2 id="search-and-review-the-conversation">
  Cari dan tinjau percakapan
</h2>

`Ctrl+o` mengalihkan antara prompt normal dan mode transkrip.

Untuk tampilan yang lebih tenang yang menampilkan hanya prompt terakhir Anda, ringkasan satu baris panggilan alat dengan diffstat edit, dan respons akhir, jalankan `/focus`. Pengaturan bertahan di seluruh sesi. Jalankan `/focus` lagi untuk mematikannya.

Mode transkrip mendapatkan navigasi dan pencarian gaya `less`:

| Kunci                                  | Tindakan                                                                                                                              |
| :------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------ |
| `/`                                    | Buka pencarian. Ketik untuk menemukan kecocokan, `Enter` untuk menerima, `Esc` untuk membatalkan dan mengembalikan posisi scroll Anda |
| `n` / `N`                              | Lompat ke kecocokan berikutnya atau sebelumnya. Bekerja setelah Anda menutup bilah pencarian                                          |
| `j` / `k` atau `↑` / `↓`               | Gulir satu baris                                                                                                                      |
| `g` / `G` atau `Home` / `End`          | Lompat ke atas atau bawah                                                                                                             |
| `Ctrl+u` / `Ctrl+d`                    | Gulir setengah halaman                                                                                                                |
| `Ctrl+b` / `Ctrl+f` atau `Space` / `b` | Gulir halaman penuh                                                                                                                   |
| `Ctrl+o`, `Esc`, atau `q`              | Keluar dari mode transkrip dan kembali ke prompt                                                                                      |

`Cmd+f` dan pencarian tmux terminal Anda tidak melihat percakapan karena berada di buffer layar alternatif, bukan scrollback asli. Untuk mengembalikan konten ke terminal Anda, tekan `Ctrl+o` untuk memasuki mode transkrip terlebih dahulu, kemudian:

* **`[`**: menulis percakapan lengkap ke buffer scrollback asli terminal Anda, dengan semua output alat diperluas. Percakapan sekarang merupakan teks biasa di terminal Anda, sehingga `Cmd+f`, mode salinan tmux, dan alat asli lainnya dapat mencari atau memilihnya. Sesi panjang mungkin berhenti sejenak saat ini terjadi. Ini berlangsung sampai Anda keluar dari mode transkrip dengan `Esc` atau `q`, yang mengembalikan Anda ke rendering fullscreen. `Ctrl+o` berikutnya dimulai segar.
* **`v`**: menulis percakapan ke file sementara dan membukanya di `$VISUAL` atau `$EDITOR`.

Tekan `Esc` atau `q` untuk kembali ke prompt.

<h2 id="clear-the-conversation">
  Hapus percakapan
</h2>

Tekan `Ctrl+L` dua kali dalam dua detik untuk menjalankan `/clear` dan memulai percakapan baru. Penekanan pertama menggambar ulang layar dan menampilkan petunjuk; penekanan kedua menghapus percakapan. Di macOS, menekan ganda `Cmd+K` juga menjalankan `/clear`.

<h2 id="use-with-tmux">
  Gunakan dengan tmux
</h2>

Rendering fullscreen bekerja di dalam tmux, dengan tiga peringatan.

Scrolling roda mouse memerlukan mode mouse tmux. Jika `~/.tmux.conf` Anda belum mengaktifkannya, tambahkan baris ini dan muat ulang konfigurasi Anda:

```bash theme={null}
set -g mouse on
```

Tanpa mode mouse, peristiwa roda pergi ke tmux alih-alih Claude Code. Scrolling keyboard dengan `PgUp` dan `PgDn` bekerja baik cara. Claude Code mencetak petunjuk satu kali saat startup jika mendeteksi tmux dengan mode mouse dimatikan.

Rendering fullscreen tidak kompatibel dengan mode integrasi tmux iTerm2, yang merupakan mode yang Anda masuki dengan `tmux -CC`. Dalam mode integrasi, iTerm2 merender setiap panel tmux sebagai pemisah asli daripada membiarkan tmux menggambar ke terminal. Buffer layar alternatif dan pelacakan mouse tidak bekerja dengan benar di sana: roda mouse tidak melakukan apa pun, dan klik ganda dapat merusak status terminal. Jangan aktifkan rendering fullscreen di sesi `tmux -CC`. Tmux reguler di dalam iTerm2, tanpa `-CC`, bekerja dengan baik.

Tidak setiap versi tmux menerapkan output yang disinkronkan dari aplikasi, jadi Anda mungkin melihat lebih banyak flicker selama redraw di bawah tmux dibandingkan saat menjalankan Claude Code langsung di terminal Anda. Jika flicker terlihat jelas, terutama melalui SSH, tingkatkan ke tmux terbaru atau jalankan Claude Code di tab terminal terpisahnya sendiri di luar tmux. Periksa versi tmux Anda dengan `tmux -V`.

Claude Code menghidupkan output yang disinkronkan secara otomatis ketika mendeteksi tmux 3.4 atau lebih baru dari variabel `TERM_PROGRAM_VERSION`, dan kembali ke pertanyaan terminal secara langsung untuk dukungan output yang disinkronkan ketika versi tidak dapat ditentukan. Apakah redraw benar-benar menjadi atomik tergantung pada versi tmux Anda menghormati output yang disinkronkan; jika Anda masih melihat flicker di bawah tmux 3.4 atau lebih baru, tingkatkan ke tmux terbaru. Deteksi ini memerlukan Claude Code v2.1.200 atau lebih baru.

<h2 id="keep-native-text-selection">
  Pertahankan pemilihan teks asli
</h2>

Penangkapan mouse adalah titik gesekan paling umum, terutama melalui SSH atau di dalam tmux. Ketika Claude Code menangkap peristiwa mouse, copy-on-select asli terminal Anda berhenti bekerja. Pemilihan yang Anda buat dengan klik-dan-seret ada di dalam Claude Code, bukan di buffer pemilihan terminal Anda, sehingga mode salinan tmux, petunjuk Kitty, dan alat serupa tidak melihatnya.

Claude Code menulis pemilihan ke clipboard sistem Anda, dan jalur yang digunakan tergantung pada pengaturan Anda. Pada sesi lokal, ia menjalankan alat clipboard asli:

* **macOS**: `pbcopy`
* **Linux**: `wl-copy` di Wayland, atau `xclip` atau `xsel` di X11, mana pun yang terinstal. Claude Code menulis baik clipboard maupun pemilihan PRIMARY, sehingga paste tengah-klik berfungsi.
* **Windows dan WSL**: PowerShell `Set-Clipboard`

Di dalam tmux, ia juga menulis ke buffer pasta tmux. Melalui SSH, ia kembali ke urutan escape OSC 52. Claude Code mencetak toast setelah setiap salinan memberi tahu Anda jalur mana yang digunakan.

Beberapa terminal memblokir OSC 52 secara default. iTerm2 memblokirnya sampai Anda mengaktifkan Settings → General → Selection → Applications in terminal may access clipboard; menjalankan [`/terminal-setup`](/docs/id/terminal-config) di iTerm2 mengaktifkan ini untuk Anda.

Untuk pemilihan asli sekali pakai, kunci yang digunakan tergantung pada terminal Anda:

* **Terminal.app**: `Fn`
* **iTerm2**: `Option`
* **VS Code, Cursor, dan Devin Desktop**: `Shift`, atau `Option` di macOS dengan pengaturan `terminal.integrated.macOptionClickForcesSelection` diaktifkan
* **Sebagian besar terminal lainnya**: `Shift`

Tahan kunci itu sambil Anda klik dan seret. Terminal Anda menangani pemilihan itu sendiri alih-alih meneruskannya ke Claude Code, sehingga pintasan salinan seperti `Cmd+C` bekerja pada apa yang Anda pilih. Claude Code juga menampilkan kunci yang benar dalam petunjuk on-screen-nya.

Melalui SSH atau di dalam tmux, Claude Code tidak selalu dapat mendeteksi terminal yang Anda hubungkan, jadi petunjuk mencantumkan kunci kandidat sebagai gantinya.

Jika Anda mengandalkan pemilihan asli sepanjang waktu, atur `CLAUDE_CODE_DISABLE_MOUSE=1` untuk keluar dari penangkapan mouse sambil mempertahankan rendering bebas flicker dan memori datar:

```bash theme={null}
CLAUDE_CODE_NO_FLICKER=1 CLAUDE_CODE_DISABLE_MOUSE=1 claude
```

Dengan penangkapan mouse dinonaktifkan, scrolling keyboard dengan `PgUp`, `PgDn`, `Ctrl+Home`, dan `Ctrl+End` masih berfungsi, dan terminal Anda menangani pemilihan secara asli. Anda kehilangan klik-untuk-memposisikan-kursor, klik-untuk-memperluas-output-alat, klik-URL, dan scrolling roda di dalam Claude Code.

Untuk mempertahankan scrolling roda tetapi mematikan penanganan klik, seret, dan hover, atur `CLAUDE_CODE_DISABLE_MOUSE_CLICKS=1` sebagai gantinya. Memerlukan Claude Code v2.1.195 atau lebih baru. `CLAUDE_CODE_DISABLE_MOUSE` memiliki prioritas ketika kedua variabel diatur.

Dengan klik dinonaktifkan, Claude Code masih menangkap mouse, sehingga roda dan touchpad menggulir percakapan tetapi klik kiri tidak melakukan apa pun di dalam Claude Code. Anda masih perlu menahan kunci terminal Anda untuk pemilihan klik-dan-seret asli. Klik kanan dan paste tengah-klik terus berfungsi di terminal yang mendukungnya.

<h2 id="troubleshooting">
  Troubleshooting
</h2>

<h3 id="stale-or-misplaced-text-on-screen">
  Teks basi atau tidak sesuai tempat di layar
</h3>

Rendering fullscreen mengirimkan hanya sel yang berubah antar frame. Beberapa terminal, paling umum Windows Terminal dan host berbasis ConPTY lainnya, menggabungkan penulisan yang diposisikan ini secara tidak benar dan meninggalkan fragmen output sebelumnya di layar hingga Anda mengubah ukuran jendela.

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

Di Windows, Claude Code sudah mengaktifkan full repaint secara otomatis untuk sesi latar belakang dan [agent view](/docs/id/agent-view), jadi Anda hanya perlu mengatur variabel untuk sesi fullscreen interaktif yang Anda luncurkan secara langsung.

<h2 id="research-preview">
  Pratinjau penelitian
</h2>

Rendering fullscreen adalah fitur pratinjau penelitian. Ini telah diuji pada emulator terminal umum, tetapi Anda mungkin mengalami masalah rendering pada terminal yang kurang umum atau konfigurasi yang tidak biasa.

Jika Anda mengalami masalah, jalankan `/feedback` di dalam Claude Code untuk melaporkannya, atau buka masalah di [repositori GitHub claude-code](https://github.com/anthropics/claude-code/issues). Sertakan nama dan versi emulator terminal Anda.

Untuk mematikan rendering fullscreen, jalankan `/tui default`, atau batalkan pengaturan `CLAUDE_CODE_NO_FLICKER` jika Anda mengaktifkannya dengan cara itu. Untuk memaksa renderer klasik terlepas dari pengaturan `tui` yang disimpan, atur `CLAUDE_CODE_DISABLE_ALTERNATE_SCREEN=1`. Renderer klasik menjaga percakapan dalam scrollback asli terminal Anda sehingga `Cmd+f` dan mode copy tmux berfungsi seperti biasanya.

Sesi latar belakang yang dibuka dari [tampilan agen](/docs/id/agent-view) atau `claude attach` selalu menggunakan rendering fullscreen. Terminal yang melampirkan memasuki buffer layar alternatif untuk menampilkan sesi, dan renderer klasik tidak memiliki scrollback atau penanganan mouse di sana, jadi pengaturan `tui` dan `CLAUDE_CODE_DISABLE_ALTERNATE_SCREEN` tidak berlaku untuk mereka.
