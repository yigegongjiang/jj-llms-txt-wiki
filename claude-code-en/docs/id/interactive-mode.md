> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Mode interaktif

> Referensi lengkap untuk pintasan keyboard, mode input, dan fitur interaktif dalam sesi Claude Code.

<h2 id="keyboard-shortcuts">
  Pintasan keyboard
</h2>

<Note>
  Pintasan keyboard mungkin berbeda menurut platform dan terminal. Dalam [rendering fullscreen](/docs/id/fullscreen), tekan `?` di penampil transkrip untuk melihat pintasan yang tersedia di sana.

  **Pengguna macOS**: Pintasan tombol Option/Alt (`Alt+B`, `Alt+F`, `Alt+Y`, `Alt+M`, `Alt+P`) memerlukan konfigurasi Option sebagai Meta di terminal Anda:

  * **iTerm2**: Settings → Profiles → Keys → General → atur Left/Right Option key ke "Esc+"
  * **Apple Terminal**: Settings → Profiles → Keyboard → centang "Use Option as Meta Key"
  * **VS Code**: atur `"terminal.integrated.macOptionIsMeta": true` dalam pengaturan VS Code

  Lihat [Konfigurasi terminal](/docs/id/terminal-config) untuk detail.
</Note>

<h3 id="general-controls">
  Kontrol umum
</h3>

| Pintasan                                                      | Deskripsi                                                                                                                                                                       | Konteks                                                                                                                                                                                                                                                                                                                                                            |
| :------------------------------------------------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `Ctrl+C`                                                      | Interupsi, atau hapus input                                                                                                                                                     | Mengganggu operasi yang sedang berjalan. Jika tidak ada yang berjalan, tekan pertama kali menghapus input prompt dan tekan kedua kali keluar dari Claude Code                                                                                                                                                                                                      |
| `Ctrl+X Ctrl+K`                                               | Matikan semua [subagen latar belakang](/docs/id/sub-agents#run-subagents-in-foreground-or-background) yang berjalan dalam sesi ini. Tekan dua kali dalam 3 detik untuk mengonfirmasi | Kontrol subagen                                                                                                                                                                                                                                                                                                                                                    |
| `Ctrl+D`                                                      | Keluar dari sesi Claude Code                                                                                                                                                    | Sinyal EOF                                                                                                                                                                                                                                                                                                                                                         |
| `Ctrl+G` atau `Ctrl+X Ctrl+E`                                 | Buka di editor teks default                                                                                                                                                     | Edit prompt atau respons kustom Anda di editor teks default. `Ctrl+X Ctrl+E` adalah binding readline-native. Aktifkan Show last response in external editor di `/config` untuk menambahkan respons Claude sebelumnya sebagai konteks berkomentar `#` di atas prompt Anda; blok komentar dihapus saat Anda menyimpan                                                |
| `Ctrl+L`                                                      | Gambar ulang layar                                                                                                                                                              | Memaksa redraw terminal penuh. Input dan riwayat percakapan disimpan. Gunakan ini untuk memulihkan jika tampilan menjadi berantakan atau sebagian kosong                                                                                                                                                                                                           |
| `Ctrl+O`                                                      | Alihkan penampil transkrip                                                                                                                                                      | Menampilkan penggunaan dan eksekusi alat yang terperinci, dengan stempel waktu dan model yang digunakan pada setiap pesan asisten. Juga memperluas panggilan MCP, yang runtuh menjadi satu baris seperti "Called slack 3 times" secara default                                                                                                                     |
| `Ctrl+R`                                                      | Pencarian riwayat perintah terbalik                                                                                                                                             | Cari melalui perintah sebelumnya secara interaktif                                                                                                                                                                                                                                                                                                                 |
| `Ctrl+V` atau `Cmd+V` (iTerm2) atau `Alt+V` (Windows dan WSL) | Tempel gambar dari clipboard                                                                                                                                                    | Menyisipkan chip `[Image #N]` di kursor sehingga Anda dapat mereferensikannya secara posisional dalam prompt Anda. Di WSL, baik `Ctrl+V` maupun `Alt+V` terikat; gunakan `Alt+V` jika terminal Anda menangkap `Ctrl+V`                                                                                                                                             |
| `Ctrl+B`                                                      | Tugas yang berjalan di latar belakang                                                                                                                                           | Menjalankan perintah bash dan agen di latar belakang. Pengguna Tmux tekan dua kali                                                                                                                                                                                                                                                                                 |
| `Ctrl+T`                                                      | Alihkan daftar tugas Claude                                                                                                                                                     | Tampilkan atau sembunyikan [daftar tugas Claude](#task-list) di area status. Ini bukan tampilan tugas latar belakang; gunakan [`/tasks`](/docs/id/commands) untuk melihat shell dan subagen yang berjalan                                                                                                                                                               |
| `Left/Right arrows`                                           | Siklus melalui tab dialog                                                                                                                                                       | Navigasi antar tab dalam dialog izin dan menu                                                                                                                                                                                                                                                                                                                      |
| `Up/Down arrows` atau `Ctrl+P`/`Ctrl+N`                       | Pindahkan kursor atau navigasi riwayat perintah                                                                                                                                 | Ketika input mencakup lebih dari satu baris visual, baik dibungkus atau multiline, pertama-tama memindahkan kursor dalam prompt. Setelah kursor berada di baris visual pertama atau terakhir, menekan lagi menavigasi riwayat perintah. Mulai dari v2.1.169, input single-line yang dibungkus berperilaku sama dengan multiline                                    |
| `Esc`                                                         | Interupsi Claude, atau tutup dialog                                                                                                                                             | Hentikan respons atau panggilan alat saat ini di tengah-tengah giliran sehingga Anda dapat mengalihkan. Claude menyimpan pekerjaan yang telah dilakukan sejauh ini. Ketika dialog seperti prompt izin terbuka, `Esc` menutup dialog daripada mengganggu Claude. Sebelum v2.1.202, `Esc` pada beberapa dialog mengganggu Claude dan membiarkan dialog tetap terbuka |
| `Esc` + `Esc`                                                 | Hapus draft input, atau putar ulang                                                                                                                                             | Ketika input prompt berisi teks, tekan `Esc` dua kali menghapusnya dan menyimpan draft ke riwayat sehingga `Up` dapat mengingatnya. Ketika input kosong, tekan `Esc` dua kali membuka [menu putar ulang](/docs/id/checkpointing) untuk memulihkan atau merangkum kode dan percakapan dari titik sebelumnya                                                              |
| `Shift+Tab` atau `Alt+M` (beberapa konfigurasi)               | Alihkan mode izin                                                                                                                                                               | Beralih antara `default` (berlabel Manual dalam indikator mode), `acceptEdits`, `plan`, dan mode apa pun yang telah Anda aktifkan, seperti `auto` atau `bypassPermissions`. Lihat [permission modes](/docs/id/permission-modes).                                                                                                                                        |
| `Option+P` (macOS) atau `Alt+P` (Windows/Linux)               | Alihkan model                                                                                                                                                                   | Alihkan model tanpa menghapus prompt Anda                                                                                                                                                                                                                                                                                                                          |
| `Option+T` (macOS) atau `Alt+T` (Windows/Linux)               | Alihkan extended thinking                                                                                                                                                       | Aktifkan atau nonaktifkan mode extended thinking. Tidak berpengaruh pada Fable 5, yang selalu menggunakan extended thinking. Mulai dari v2.1.132 pintasan ini berfungsi di macOS tanpa mengonfigurasi Option sebagai Meta                                                                                                                                          |
| `Option+O` (macOS) atau `Alt+O` (Windows/Linux)               | Alihkan mode cepat                                                                                                                                                              | Aktifkan atau nonaktifkan [fast mode](/docs/id/fast-mode)                                                                                                                                                                                                                                                                                                               |

<h3 id="text-editing">
  Pengeditan teks
</h3>

| Pintasan                   | Deskripsi                                | Konteks                                                                                                                                                                                                           |
| :------------------------- | :--------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `Ctrl+A`                   | Pindahkan kursor ke awal baris saat ini  | Dalam input multiline, memindahkan ke awal baris logis saat ini                                                                                                                                                   |
| `Ctrl+E`                   | Pindahkan kursor ke akhir baris saat ini | Dalam input multiline, memindahkan ke akhir baris logis saat ini                                                                                                                                                  |
| `Ctrl+K`                   | Hapus hingga akhir baris                 | Menyimpan teks yang dihapus untuk ditempel                                                                                                                                                                        |
| `Ctrl+U`                   | Hapus dari kursor ke awal baris          | Menyimpan teks yang dihapus untuk ditempel. Ulangi untuk menghapus di seluruh baris dalam input multiline. Di macOS, emulator terminal termasuk iTerm2 dan Terminal.app memetakan `Cmd+Backspace` ke pintasan ini |
| `Ctrl+W`                   | Hapus kata sebelumnya                    | Menyimpan teks yang dihapus untuk ditempel. Di Windows, `Ctrl+Backspace` juga menghapus kata sebelumnya                                                                                                           |
| `Ctrl+Y`                   | Tempel teks yang dihapus                 | Tempel teks yang dihapus dengan `Ctrl+K`, `Ctrl+U`, atau `Ctrl+W`                                                                                                                                                 |
| `Alt+Y` (setelah `Ctrl+Y`) | Siklus riwayat tempel                    | Setelah menempel, siklus melalui teks yang dihapus sebelumnya. Memerlukan [Option as Meta](#keyboard-shortcuts) di macOS                                                                                          |
| `Alt+B`                    | Pindahkan kursor kembali satu kata       | Navigasi kata. Memerlukan [Option as Meta](#keyboard-shortcuts) di macOS                                                                                                                                          |
| `Alt+F`                    | Pindahkan kursor maju satu kata          | Navigasi kata. Memerlukan [Option as Meta](#keyboard-shortcuts) di macOS                                                                                                                                          |

<h3 id="theme-and-display">
  Tema dan tampilan
</h3>

| Pintasan | Deskripsi                                  | Konteks                                                                                                                   |
| :------- | :----------------------------------------- | :------------------------------------------------------------------------------------------------------------------------ |
| `Ctrl+T` | Alihkan penyorotan sintaks untuk blok kode | Hanya berfungsi di dalam menu pemilih `/theme`. Mengontrol apakah kode dalam respons Claude menggunakan pewarnaan sintaks |

<h3 id="multiline-input">
  Input multiline
</h3>

| Metode         | Pintasan        | Konteks                                                                                                  |
| :------------- | :-------------- | :------------------------------------------------------------------------------------------------------- |
| Escape cepat   | `\` + `Enter`   | Berfungsi di semua terminal                                                                              |
| Tombol Option  | `Option+Enter`  | Setelah mengaktifkan [Option as Meta](/docs/id/terminal-config#enable-option-key-shortcuts-on-macos) di macOS |
| Shift+Enter    | `Shift+Enter`   | Bawaan di iTerm2, WezTerm, Ghostty, Kitty, Warp, Apple Terminal, Windows Terminal                        |
| Urutan kontrol | `Ctrl+J`        | Berfungsi di terminal apa pun tanpa konfigurasi                                                          |
| Mode tempel    | Tempel langsung | Untuk blok kode, log                                                                                     |

<Tip>
  Shift+Enter berfungsi tanpa konfigurasi di iTerm2, WezTerm, Ghostty, Kitty, Warp, Apple Terminal, dan Windows Terminal. Untuk VS Code, Cursor, Devin Desktop, Alacritty, dan Zed, jalankan `/terminal-setup` untuk memasang binding.
</Tip>

<h3 id="quick-commands">
  Perintah cepat
</h3>

| Pintasan    | Deskripsi             | Catatan                                                                                               |
| :---------- | :-------------------- | :---------------------------------------------------------------------------------------------------- |
| `/` di awal | Perintah atau skill   | Lihat [perintah](#commands) dan [skills](/docs/id/skills)                                                  |
| `!` di awal | Mode Bash             | Jalankan perintah secara langsung, tambahkan output eksekusi ke sesi, dan biarkan Claude meresponsnya |
| `@`         | Penyebutan jalur file | Picu pelengkapan otomatis jalur file                                                                  |

<h3 id="transcript-viewer">
  Penampil transkrip
</h3>

Ketika penampil transkrip terbuka (dialihkan dengan `Ctrl+O`), pintasan ini tersedia. Dalam [rendering fullscreen](/docs/id/fullscreen), tekan `?` untuk menampilkan panel referensi pintasan keyboard lengkap di dalam penampil. `Ctrl+E` dapat diubah melalui [`transcript:toggleShowAll`](/docs/id/keybindings).

| Pintasan             | Deskripsi                                                                                                                                                                                                                |
| :------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `?`                  | Alihkan panel bantuan pintasan keyboard. Memerlukan [rendering fullscreen](/docs/id/fullscreen)                                                                                                                               |
| `{` / `}`            | Lompat ke prompt pengguna sebelumnya atau berikutnya, seperti gerakan paragraf vim. Memerlukan [rendering fullscreen](/docs/id/fullscreen)                                                                                    |
| `Ctrl+E`             | Alihkan tampilkan semua konten                                                                                                                                                                                           |
| `[`                  | Tulis percakapan lengkap ke scrollback asli terminal Anda sehingga `Cmd+F`, mode copy tmux, dan alat asli lainnya dapat mencarinya. Memerlukan [rendering fullscreen](/docs/id/fullscreen#search-and-review-the-conversation) |
| `v`                  | Tulis percakapan ke file sementara dan buka di `$VISUAL` atau `$EDITOR`. Memerlukan [rendering fullscreen](/docs/id/fullscreen)                                                                                               |
| `q`, `Ctrl+C`, `Esc` | Keluar dari tampilan transkrip. Ketiganya dapat diubah melalui [`transcript:exit`](/docs/id/keybindings)                                                                                                                      |

<h3 id="voice-input">
  Input suara
</h3>

| Pintasan                 | Deskripsi       | Catatan                                                                                                                                                                                               |
| :----------------------- | :-------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Tahan atau ketuk `Space` | Dictation suara | Memerlukan [voice dictation](/docs/id/voice-dictation) untuk diaktifkan. Tahan untuk merekam, atau jalankan `/voice tap` untuk tap-to-toggle. [Dapat diubah](/docs/id/voice-dictation#rebind-the-dictation-key) |

<h2 id="commands">
  Perintah
</h2>

Ketik `/` di Claude Code untuk melihat semua perintah yang tersedia, atau ketik `/` diikuti huruf apa pun untuk memfilter. Menu `/` menampilkan semua yang dapat Anda panggil: perintah bawaan, [skills](/docs/id/skills) bundel dan yang ditulis pengguna, dan perintah yang disumbangkan oleh [plugins](/docs/id/plugins) dan [MCP servers](/docs/id/mcp#use-mcp-prompts-as-commands). Tidak semua perintah bawaan terlihat oleh setiap pengguna karena beberapa bergantung pada platform atau paket Anda.

Dalam [rendering layar penuh](/docs/id/fullscreen#use-the-mouse), daftar saran perintah `/` dan file `@` juga merespons mouse: mengarahkan kursor menyoroti baris dan mengklik menerimanya.

Lihat [referensi perintah](/docs/id/commands) untuk daftar lengkap perintah yang disertakan dalam Claude Code.

<h2 id="vim-editor-mode">
  Mode editor Vim
</h2>

Aktifkan pengeditan gaya vim melalui `/config` → Editor mode.

<h3 id="mode-switching">
  Pengalihan mode
</h3>

| Perintah | Tindakan                                    | Dari mode      |
| :------- | :------------------------------------------ | :------------- |
| `Esc`    | Masuk mode NORMAL                           | INSERT, VISUAL |
| `i`      | Sisipkan sebelum kursor                     | NORMAL         |
| `I`      | Sisipkan di awal baris                      | NORMAL         |
| `a`      | Sisipkan setelah kursor                     | NORMAL         |
| `A`      | Sisipkan di akhir baris                     | NORMAL         |
| `o`      | Buka baris di bawah                         | NORMAL         |
| `O`      | Buka baris di atas                          | NORMAL         |
| `v`      | Mulai pemilihan visual berdasarkan karakter | NORMAL         |
| `V`      | Mulai pemilihan visual berdasarkan baris    | NORMAL         |

<h3 id="remap-insert-mode-key-sequences">
  Pemetaan ulang urutan kunci mode INSERT
</h3>

Pengaturan [`vimInsertModeRemaps`](/docs/id/settings#available-settings) memetakan urutan mode INSERT dua kunci ke Escape, sehingga pemetaan seperti `jj` mengembalikan Anda ke mode NORMAL. Memerlukan Claude Code v2.1.208 atau lebih baru.

Contoh `~/.claude/settings.json` berikut mengaktifkan mode vim dan memetakan `jj` ke Escape:

```json theme={null}
{
  "editorMode": "vim",
  "vimInsertModeRemaps": { "jj": "<Esc>" }
}
```

Setiap kunci adalah tepat dua karakter yang dapat dicetak yang diketik secara berurutan, dan `"<Esc>"` adalah satu-satunya target yang didukung. Entri dengan panjang atau target yang berbeda diabaikan.

Mengetik karakter pertama dari urutan menyisipkannya secara normal. Menekan karakter kedua dalam satu detik menghapus karakter yang tertunda itu dan beralih ke mode NORMAL, meninggalkan tidak ada karakter dalam input Anda. Setelah jendela satu detik, atau jika kunci yang berbeda mengikuti, kedua karakter tetap sebagai teks literal, sehingga Anda masih dapat mengetik kata yang berisi urutan dengan berhenti di antara dua kunci.

Claude Code membaca pengaturan ini dari file pengaturan pengguna Anda, bendera `--settings`, dan [pengaturan yang dikelola](/docs/id/permissions#managed-settings) saja. Entri dalam `.claude/settings.json` atau `.claude/settings.local.json` proyek diabaikan, sehingga repositori yang diperiksa tidak dapat memetakan ulang penekanan tombol Anda.

<h3 id="navigation-normal-mode">
  Navigasi (mode NORMAL)
</h3>

| Perintah        | Tindakan                                                                                                                                                                                              |
| :-------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `h`/`j`/`k`/`l` | Pindah kiri/bawah/atas/kanan                                                                                                                                                                          |
| `Space`         | Pindah ke kanan                                                                                                                                                                                       |
| `w`             | Kata berikutnya                                                                                                                                                                                       |
| `e`             | Akhir kata                                                                                                                                                                                            |
| `b`             | Kata sebelumnya                                                                                                                                                                                       |
| `0`             | Awal baris                                                                                                                                                                                            |
| `$`             | Akhir baris                                                                                                                                                                                           |
| `^`             | Karakter non-blank pertama                                                                                                                                                                            |
| `gg`            | Awal input                                                                                                                                                                                            |
| `G`             | Akhir input                                                                                                                                                                                           |
| `f{char}`       | Lompat ke kemunculan berikutnya dari karakter                                                                                                                                                         |
| `F{char}`       | Lompat ke kemunculan sebelumnya dari karakter                                                                                                                                                         |
| `t{char}`       | Lompat ke tepat sebelum kemunculan berikutnya dari karakter                                                                                                                                           |
| `T{char}`       | Lompat ke tepat setelah kemunculan sebelumnya dari karakter                                                                                                                                           |
| `;`             | Ulangi gerakan f/F/t/T terakhir                                                                                                                                                                       |
| `,`             | Ulangi gerakan f/F/t/T terakhir dalam urutan terbalik                                                                                                                                                 |
| `/`             | Buka pencarian riwayat terbalik, sama seperti `Ctrl+R`. Mulai dari v2.1.191, prompt pencarian kosong menampilkan petunjuk: tekan `Esc` lalu `i` lalu `/` untuk membuka menu perintah sebagai gantinya |

<Note>
  Dalam mode normal vim, jika kursor berada di awal atau akhir input dan tidak dapat bergerak lebih jauh, `j`/`k` dan tombol panah menavigasi riwayat perintah sebagai gantinya.
</Note>

<h3 id="editing-normal-mode">
  Pengeditan (mode NORMAL)
</h3>

| Perintah       | Tindakan                        |
| :------------- | :------------------------------ |
| `x`            | Hapus karakter                  |
| `dd`           | Hapus baris                     |
| `D`            | Hapus hingga akhir baris        |
| `dw`/`de`/`db` | Hapus kata/hingga akhir/kembali |
| `cc`           | Ubah baris                      |
| `C`            | Ubah hingga akhir baris         |
| `cw`/`ce`/`cb` | Ubah kata/hingga akhir/kembali  |
| `yy`/`Y`       | Yank (salin) baris              |
| `yw`/`ye`/`yb` | Yank kata/hingga akhir/kembali  |
| `p`            | Tempel setelah kursor           |
| `P`            | Tempel sebelum kursor           |
| `>>`           | Indentasi baris                 |
| `<<`           | Kurangi indentasi baris         |
| `J`            | Gabungkan baris                 |
| `u`            | Batalkan                        |
| `.`            | Ulangi perubahan terakhir       |

<h3 id="text-objects-normal-mode">
  Objek teks (mode NORMAL)
</h3>

Objek teks bekerja dengan operator seperti `d`, `c`, dan `y`:

| Perintah  | Tindakan                                 |
| :-------- | :--------------------------------------- |
| `iw`/`aw` | Kata dalam/sekitar                       |
| `iW`/`aW` | KATA dalam/sekitar (dibatasi whitespace) |
| `i"`/`a"` | Dalam/sekitar tanda kutip ganda          |
| `i'`/`a'` | Dalam/sekitar tanda kutip tunggal        |
| `i(`/`a(` | Dalam/sekitar tanda kurung               |
| `i[`/`a[` | Dalam/sekitar kurung siku                |
| `i{`/`a{` | Dalam/sekitar kurung kurawal             |

<h3 id="visual-mode">
  Mode visual
</h3>

Tekan `v` untuk pemilihan berdasarkan karakter atau `V` untuk pemilihan berdasarkan baris. Gerakan memperluas pemilihan, dan operator bertindak langsung padanya.

| Perintah         | Tindakan                                                               |
| :--------------- | :--------------------------------------------------------------------- |
| `d`/`x`          | Hapus pemilihan                                                        |
| `y`              | Yank pemilihan                                                         |
| `c`/`s`          | Ubah pemilihan                                                         |
| `p`              | Ganti pemilihan dengan isi register                                    |
| `r{char}`        | Ganti setiap karakter yang dipilih dengan `{char}`                     |
| `~`/`u`/`U`      | Alihkan, huruf kecil, atau huruf besar pemilihan                       |
| `>`/`<`          | Indentasi atau kurangi indentasi baris yang dipilih                    |
| `J`              | Gabungkan baris yang dipilih                                           |
| `o`              | Tukar kursor dan jangkar                                               |
| `iw`/`aw`/`i"`/… | Pilih objek teks                                                       |
| `v`/`V`          | Alihkan antara berdasarkan karakter dan berdasarkan baris, atau keluar |

Mode visual berdasarkan blok dengan `Ctrl+V` tidak didukung.

<h2 id="command-history">
  Riwayat perintah
</h2>

Claude Code mempertahankan riwayat perintah untuk sesi saat ini:

* Riwayat input disimpan per direktori kerja
* Riwayat input direset ketika Anda menjalankan `/clear` untuk memulai sesi baru. Percakapan sesi sebelumnya disimpan dan dapat dilanjutkan.
* Mengirimkan prompt yang sama dua kali berturut-turut mencatat satu entri riwayat, jadi menekan Up melangkah ke prompt berbeda sebelumnya
* Gunakan panah Up/Down untuk menavigasi (lihat pintasan keyboard di atas)
* Ekspansi riwayat dengan `!` dinonaktifkan secara default

<h3 id="reverse-search-with-ctrl-r">
  Pencarian terbalik dengan Ctrl+R
</h3>

Tekan `Ctrl+R` untuk mencari secara interaktif melalui riwayat perintah Anda:

1. **Mulai pencarian**: tekan `Ctrl+R` untuk mengaktifkan pencarian riwayat terbalik
2. **Ketik kueri**: masukkan teks untuk dicari dalam perintah sebelumnya. Istilah pencarian disorot dalam hasil yang cocok
3. **Navigasi kecocokan**: tekan `Ctrl+R` lagi untuk siklus melalui kecocokan yang lebih lama
4. **Ubah cakupan**: pencarian secara default mencakup prompt dari semua proyek. Tekan `Ctrl+S` untuk siklus cakupan melalui sesi ini, proyek ini, dan semua proyek
5. **Terima kecocokan**:
   * Tekan `Tab` atau `Esc` untuk menerima kecocokan saat ini dan lanjutkan pengeditan
   * Tekan `Enter` untuk menerima dan menjalankan perintah segera
6. **Batalkan pencarian**:
   * Tekan `Ctrl+C` untuk membatalkan dan mengembalikan input asli Anda
   * Tekan `Backspace` pada pencarian kosong untuk membatalkan

Pencarian memuat 100 prompt unik terbaru dalam cakupan yang dipilih, dengan duplikat yang disatukan ke kemunculan terbaru. Prompt yang cocok ditampilkan dengan istilah pencarian disorot, sehingga Anda dapat menemukan dan menggunakan kembali input sebelumnya.

Menerima kecocokan atau membatalkan pencarian berlaku segera, bahkan saat Claude Code masih memuat riwayat. Sebelum v2.1.202, menerima atau membatalkan selama pemuatan tersebut dapat melaporkan kesalahan internal.

<h2 id="background-bash-commands">
  Perintah Bash latar belakang
</h2>

Claude Code mendukung menjalankan perintah Bash di latar belakang, memungkinkan Anda untuk terus bekerja sementara proses yang berjalan lama dieksekusi.

<h3 id="how-backgrounding-works">
  Cara backgrounding bekerja
</h3>

Ketika Claude Code menjalankan perintah di latar belakang, ia menjalankan perintah secara asinkron dan segera mengembalikan ID tugas latar belakang. Claude Code dapat merespons prompt baru sementara perintah terus dieksekusi di latar belakang.

Untuk menjalankan perintah di latar belakang, Anda dapat:

* Minta Claude Code untuk menjalankan perintah di latar belakang
* Tekan `Ctrl+B` untuk memindahkan invokasi alat Bash biasa ke latar belakang. Pengguna Tmux harus menekan `Ctrl+B` dua kali karena kunci awalan tmux.

**Fitur utama:**

* Output ditulis ke file dan Claude dapat mengambilnya menggunakan alat Read
* Tugas latar belakang memiliki ID unik untuk pelacakan dan pengambilan output
* Tugas latar belakang dibersihkan secara otomatis ketika Claude Code keluar. Backgrounding sesi alih-alih keluar menyerahkannya ke sesi latar belakang, di mana mereka terus berjalan. Lihat [background a running session](/docs/id/agent-view#from-inside-a-session)
* Tugas latar belakang secara otomatis dihentikan jika output melebihi 5GB, dengan catatan di stderr yang menjelaskan alasannya
* Sejak v2.1.193, di macOS dan Linux, tugas latar belakang yang sedang berjalan dihentikan ketika sistem operasi menandakan tekanan memori, asalkan sesi telah idle selama minimal 30 menit tanpa turn atau subagent yang berjalan. Atur [`CLAUDE_CODE_DISABLE_BG_SHELL_PRESSURE_REAP`](/docs/id/env-vars) ke `1` untuk mematikannya

Untuk menonaktifkan semua fungsionalitas tugas latar belakang, atur variabel lingkungan `CLAUDE_CODE_DISABLE_BACKGROUND_TASKS` ke `1`. Lihat [Environment variables](/docs/id/env-vars) untuk detail.

**Perintah yang sering di-background:**

* Alat build (webpack, vite, make)
* Manajer paket (npm, yarn, pnpm)
* Pelari tes (jest, pytest)
* Server pengembangan
* Proses yang berjalan lama (docker, terraform)

<h3 id="shell-mode-with-prefix">
  Mode shell dengan awalan `!`
</h3>

Jalankan perintah shell secara langsung tanpa melalui Claude dengan menambahkan awalan input Anda dengan `!`:

```bash theme={null}
! npm test
! git status
! ls -la
```

Mode shell:

* Menambahkan perintah dan outputnya ke konteks percakapan
* Menampilkan kemajuan dan output secara real-time
* Mendukung backgrounding `Ctrl+B` yang sama untuk perintah yang berjalan lama
* Tidak memerlukan Claude untuk menginterpretasi atau menyetujui perintah
* Mendukung pelengkapan otomatis berbasis riwayat: ketik perintah parsial dan tekan `Tab` untuk melengkapi dari perintah `!` sebelumnya dalam proyek saat ini
* Mendukung pelengkapan otomatis jalur file langsung sejak v2.1.193 di semua platform: ketik token yang berisi garis miring ke depan, seperti `./src/` atau `~/`, untuk melihat dropdown file dan direktori yang cocok, kemudian tekan `Tab` untuk menerima. Gunakan garis miring ke depan di Windows juga; dropdown dipicu oleh `/`, bukan `\`
* Keluar dengan `Escape`, `Backspace`, atau `Ctrl+U` pada prompt kosong
* Menempel teks yang dimulai dengan `!` ke prompt kosong memasuki mode shell secara otomatis, sesuai dengan perilaku `!` yang diketik

Sejak v2.1.186, Claude merespons output perintah secara otomatis setelah masuk ke transkrip, sehingga Anda dapat menjalankan `! npm test` dan mendapatkan penjelasan tentang kegagalan tanpa prompt kedua. Respons memiliki biaya yang sama dengan mengirim prompt normal. Untuk mengembalikan perilaku sebelumnya di mana output ditambahkan ke konteks tanpa respons, atur [`respondToBashCommands`](/docs/id/settings#available-settings) ke `false` dalam `settings.json`. Sebelum v2.1.186, mode shell selalu menambahkan output ke konteks tanpa respons.

Ini berguna untuk operasi shell cepat sambil mempertahankan konteks percakapan.

<h2 id="prompt-suggestions">
  Saran prompt
</h2>

Ketika Anda pertama kali membuka sesi, perintah contoh yang digelapkan muncul di input prompt untuk membantu Anda memulai. Claude Code memilih ini dari riwayat git proyek Anda, sehingga mencerminkan file yang telah Anda kerjakan baru-baru ini.

Setelah Claude merespons, saran terus muncul berdasarkan riwayat percakapan Anda, seperti langkah lanjutan dari permintaan multi-bagian atau kelanjutan alami dari alur kerja Anda.

* Tekan `Tab` atau `Right arrow` untuk menempatkan saran di input prompt, kemudian `Enter` untuk mengirimkan
* Mulai mengetik untuk menolaknya

Saran berjalan sebagai permintaan latar belakang yang menggunakan kembali cache prompt percakapan induk, sehingga biaya tambahan minimal. Claude Code melewati pembuatan saran ketika cache dingin untuk menghindari biaya yang tidak perlu.

Saran secara otomatis dilewati setelah giliran pertama percakapan dan dalam Plan Mode. Dalam print mode, saran dimatikan secara default. Lewatkan [`--prompt-suggestions`](/docs/id/cli-reference#cli-flags) dengan `--output-format stream-json --verbose` untuk mengeluarkan pesan `prompt_suggestion` setelah setiap giliran.

Untuk menonaktifkan saran prompt sepenuhnya, atur variabel lingkungan atau alihkan pengaturan di `/config`:

```bash theme={null}
export CLAUDE_CODE_ENABLE_PROMPT_SUGGESTION=false
```

<h2 id="side-questions-with-/btw">
  Pertanyaan sampingan dengan /btw
</h2>

Gunakan `/btw` untuk mengajukan pertanyaan cepat tentang pekerjaan saat ini Anda tanpa menambahkan ke riwayat percakapan. Ini berguna ketika Anda menginginkan jawaban cepat tetapi tidak ingin mengacaukan konteks utama atau mengalihkan Claude dari tugas yang berjalan lama.

```
/btw what was the name of that config file again?
```

Pertanyaan sampingan memiliki visibilitas penuh ke percakapan saat ini, sehingga Anda dapat bertanya tentang kode yang telah dibaca Claude, keputusan yang dibuatnya sebelumnya, atau apa pun dari sesi. Pertanyaan dan jawaban bersifat sementara: mereka muncul dalam overlay yang dapat ditutup dan tidak pernah memasuki riwayat percakapan.

* **Tersedia saat Claude sedang bekerja**: Anda dapat menjalankan `/btw` bahkan saat Claude memproses respons. Pertanyaan sampingan berjalan secara independen dan tidak mengganggu giliran utama.
* **Tidak ada akses alat**: pertanyaan sampingan hanya menjawab dari apa yang sudah ada dalam konteks. Claude tidak dapat membaca file, menjalankan perintah, atau mencari saat menjawab pertanyaan sampingan.
* **Respons tunggal**: tidak ada giliran lanjutan dalam overlay. Untuk melanjutkan utas, pisahkan ke dalam sesinya sendiri dengan `f`.
* **Biaya rendah**: pertanyaan sampingan menggunakan kembali cache prompt percakapan induk, sehingga biaya tambahan minimal.

Pertanyaan sampingan sebelumnya dari sesi yang sama muncul sebagai daftar yang redup di atas jawaban saat ini. Mereka tetap keluar dari riwayat percakapan tetapi tetap terlihat dalam overlay sampai Anda menghapusnya.

Setelah jawaban muncul, overlay menerima kunci-kunci ini.

| Kunci                      | Tindakan                                                                                                                                                                                                                                                                        |
| :------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `Space`, `Enter`, `Escape` | Tutup jawaban dan kembali ke prompt                                                                                                                                                                                                                                             |
| `Up` / `Down`              | Gulir jawaban                                                                                                                                                                                                                                                                   |
| `Left` / `Right`           | Langkah di antara jawaban `/btw` Anda dan jawaban sebelumnya dari sesi. `Left` bergerak ke jawaban yang lebih lama dan `Right` kembali ke arah yang saat ini. Memerlukan Claude Code v2.1.187 atau lebih baru                                                                   |
| `c`                        | Salin jawaban ke papan klip Anda sebagai Markdown mentah. Gunakan ini alih-alih pemilihan mouse, yang menangkap rendering terminal yang dibungkus keras daripada teks sumber                                                                                                    |
| `f`                        | Pisahkan ke sesi baru. Pemisahan mewarisi percakapan induk ditambah pertanyaan dan jawaban ini sebagai giliran transkrip nyata, sehingga Anda dapat melanjutkan dengan akses alat penuh. Sesi asli disimpan di bawah [`/resume`](/docs/id/commands). Tersedia hanya dalam sesi lokal |
| `x`                        | Hapus daftar pertukaran `/btw` sebelumnya yang ditampilkan di atas jawaban saat ini                                                                                                                                                                                             |

`/btw` adalah kebalikan dari [subagent](/docs/id/sub-agents): ia melihat percakapan lengkap Anda tetapi tidak memiliki alat, sementara subagent memiliki alat lengkap tetapi dimulai dengan konteks kosong. Gunakan `/btw` untuk bertanya tentang apa yang sudah diketahui Claude dari sesi ini; gunakan subagent untuk menemukan sesuatu yang baru.

<h2 id="task-list">
  Daftar tugas
</h2>

Daftar tugas adalah daftar periksa Claude: item yang dibuat Claude untuk merencanakan pekerjaan multi-langkah, dengan indikator yang menunjukkan apa yang tertunda, sedang berlangsung, atau selesai. Ini terpisah dari tampilan tugas latar belakang. Untuk melihat shell yang berjalan dan subagen, gunakan [`/tasks`](/docs/id/commands) sebagai gantinya.

* Tekan `Ctrl+T` untuk mengalihkan tampilan daftar tugas. Tampilan menampilkan hingga lima tugas sekaligus. Ketika Claude belum membuat item daftar periksa apa pun, toggle tidak memiliki efek visual karena tidak ada yang ditampilkan
* Untuk melihat semua tugas atau menghapusnya, minta Claude secara langsung: "show me all tasks" atau "clear all tasks"
* Tugas bertahan di seluruh pemadatan konteks, membantu Claude tetap terorganisir pada proyek yang lebih besar
* Untuk berbagi daftar tugas di seluruh sesi, atur `CLAUDE_CODE_TASK_LIST_ID` untuk menggunakan direktori bernama di `~/.claude/tasks/`: `CLAUDE_CODE_TASK_LIST_ID=my-project claude`

<h2 id="session-recap">
  Ringkasan sesi
</h2>

Ketika Anda kembali ke terminal setelah pergi, Claude Code menampilkan ringkasan satu baris tentang apa yang terjadi dalam sesi sejauh ini. Ringkasan dihasilkan di latar belakang setelah setidaknya tiga menit telah berlalu sejak giliran terakhir yang selesai dan terminal tidak fokus, sehingga siap ketika Anda beralih kembali. Ringkasan hanya muncul setelah sesi memiliki setidaknya tiga giliran, dan tidak pernah dua kali berturut-turut.

Jalankan `/recap` untuk menghasilkan ringkasan sesuai permintaan. Untuk mematikan ringkasan otomatis, buka `/config` dan nonaktifkan **Session recap**.

Ringkasan sesi aktif secara default untuk setiap paket dan penyedia. Ringkasan selalu dilewati dalam mode non-interaktif.

<h2 id="pr-review-status">
  Status tinjauan PR
</h2>

Ketika bekerja pada cabang dengan permintaan tarik terbuka, Claude Code menampilkan tautan PR yang dapat diklik di footer, seperti "PR #446". Tautan memiliki garis bawah berwarna yang menunjukkan status tinjauan:

* Hijau: disetujui
* Kuning: menunggu tinjauan
* Merah: perubahan diminta
* Abu-abu: draft

Lencana menghilang setelah permintaan tarik digabungkan atau ditutup. `Cmd+click` (macOS) atau `Ctrl+click` (Windows/Linux) tautan untuk membuka permintaan tarik di browser Anda. Status diperbarui setiap 60 detik, dan segera setelah perintah `gh pr` atau `git push` dijalankan dalam sesi.

<Note>
  Status PR memerlukan CLI `gh` untuk diinstal dan diautentikasi (`gh auth login`).
</Note>

<h2 id="see-also">
  Lihat juga
</h2>

* [Skills](/docs/id/skills) - Prompt dan alur kerja kustom
* [Checkpointing](/docs/id/checkpointing) - Putar ulang pengeditan Claude dan kembalikan status sebelumnya
* [Referensi CLI](/docs/id/cli-reference) - Bendera dan opsi baris perintah
* [Pengaturan](/docs/id/settings) - Opsi konfigurasi
* [Manajemen memori](/docs/id/memory) - Mengelola file CLAUDE.md
