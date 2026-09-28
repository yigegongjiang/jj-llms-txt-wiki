> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Mode interaktif

> Referensi lengkap untuk pintasan keyboard, mode input, dan fitur interaktif dalam sesi Claude Code.

<h2 id="keyboard-shortcuts">
  Pintasan keyboard
</h2>

<Note>
  Pintasan keyboard mungkin berbeda menurut platform dan terminal. Dalam [rendering layar penuh](/docs/id/fullscreen), tekan `?` di penampil transkrip untuk melihat pintasan yang tersedia di sana.

  **Pengguna macOS**: Pintasan tombol Option/Alt (`Alt+B`, `Alt+F`, `Alt+D`, `Alt+Y`, `Alt+P`) memerlukan konfigurasi Option sebagai Meta di terminal Anda. Lihat [Aktifkan pintasan tombol Option di macOS](/docs/id/terminal-config#enable-option-key-shortcuts-on-macos) untuk pengaturan di setiap terminal.
</Note>

<h3 id="general-controls">
  Kontrol umum
</h3>

| Pintasan                                                                                           | Deskripsi                                                                                                                                                                                                                                                                       | Konteks                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| :------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `Ctrl+C`                                                                                           | Interupsi, atau hapus input                                                                                                                                                                                                                                                     | Menghentikan operasi yang sedang berjalan. Jika tidak ada yang berjalan, tekan pertama menghapus input prompt dan tekan kedua keluar dari Claude Code                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `Ctrl+X Ctrl+K`                                                                                    | Hentikan semua [subagen latar belakang](/docs/id/sub-agents#run-subagents-in-foreground-or-background) dalam sesi ini, dan matikan [balasan otomatis artefak](/docs/id/artifacts#let-claude-reply-to-comments-on-its-own) untuk sisanya. Tekan dua kali dalam 3 detik untuk mengonfirmasi | Kontrol subagen                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `Ctrl+D`                                                                                           | Keluar dari sesi Claude Code                                                                                                                                                                                                                                                    | Tekan pertama menampilkan petunjuk konfirmasi dan tekan kedua dalam 800ms keluar. Ketika prompt memiliki teks, `Ctrl+D` menghapus karakter setelah kursor                                                                                                                                                                                                                                                                                                                                                                                                              |
| `Ctrl+G` atau `Ctrl+X Ctrl+E`                                                                      | Buka di editor teks default                                                                                                                                                                                                                                                     | Edit prompt atau respons kustom Anda di editor teks default Anda. `Ctrl+X Ctrl+E` adalah binding readline asli. Aktifkan **Tampilkan respons terakhir di editor eksternal** di `/config` untuk menambahkan balasan sebelumnya Claude sebagai konteks berkomentar `#` di atas prompt Anda; Claude Code menghapus blok komentar saat Anda menyimpan                                                                                                                                                                                                                      |
| `Ctrl+L`                                                                                           | Gambar ulang layar                                                                                                                                                                                                                                                              | Memaksa penggambaran ulang terminal penuh, menjaga input dan riwayat percakapan. Gunakan ini untuk memulihkan jika tampilan menjadi berantakan atau sebagian kosong. Lihat [Hapus percakapan](/docs/id/fullscreen#clear-the-conversation) untuk rendering layar penuh                                                                                                                                                                                                                                                                                                       |
| `Ctrl+O`                                                                                           | Alihkan penampil transkrip                                                                                                                                                                                                                                                      | Menampilkan penggunaan alat terperinci dan eksekusi, dengan stempel waktu dan model yang digunakan pada setiap pesan asisten. Juga memperluas baris yang runtuh secara default, seperti panggilan MCP, ditampilkan sebagai baris `Called slack 3 times` tunggal, dan [pesan dari sesi lain Anda](/docs/id/cross-session-messaging#what-a-message-looks-like), ditampilkan sebagai pratinjau `Message from @<sender>` satu baris                                                                                                                                             |
| `Ctrl+R`                                                                                           | Pencarian riwayat perintah terbalik                                                                                                                                                                                                                                             | Cari melalui perintah sebelumnya secara interaktif                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `Ctrl+V` atau `Cmd+V` (iTerm2) atau `Alt+V` (Windows dan WSL)                                      | Tempel gambar dari clipboard                                                                                                                                                                                                                                                    | Menyisipkan chip `[Image #N]` di kursor sehingga Anda dapat mereferensikannya secara posisional dalam prompt Anda. Di WSL, baik `Ctrl+V` maupun `Alt+V` terikat; gunakan `Alt+V` jika terminal Anda menangkap `Ctrl+V`                                                                                                                                                                                                                                                                                                                                                 |
| `Ctrl+B`                                                                                           | Tugas yang berjalan di latar belakang                                                                                                                                                                                                                                           | Menjalankan perintah Bash dan agen di latar belakang. Pengguna Tmux tekan dua kali                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `Ctrl+T`                                                                                           | Alihkan daftar tugas Claude                                                                                                                                                                                                                                                     | Tampilkan atau sembunyikan [daftar tugas Claude](#task-list) di area status. Ini bukan tampilan tugas latar belakang; gunakan [`/tasks`](/docs/id/commands) untuk melihat shell dan subagen yang berjalan                                                                                                                                                                                                                                                                                                                                                                   |
| `Ctrl+S`                                                                                           | Simpan atau pulihkan prompt                                                                                                                                                                                                                                                     | Dengan teks dalam input, menyimpannya dan menghapus prompt. Ditekan lagi pada prompt kosong, memulihkan teks yang disimpan, posisi kursor, konten yang ditempel, dan mode input, jadi perintah shell `!` yang disimpan [kembali dalam mode shell](#shell-mode-with-prefix)                                                                                                                                                                                                                                                                                             |
| `Ctrl+Z`                                                                                           | Jeda Claude Code                                                                                                                                                                                                                                                                | Unix saja. Menangguhkan proses ke shell Anda; jalankan `fg` untuk melanjutkan                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `Panah Kiri/Kanan`                                                                                 | Siklus melalui tab dialog                                                                                                                                                                                                                                                       | Navigasi antar tab dalam dialog izin dan menu                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `Tab`                                                                                              | Terima saran pelengkapan otomatis, atau tambahkan komentar ke jawaban izin                                                                                                                                                                                                      | Saat saran pelengkapan otomatis ditampilkan dalam input prompt, menerima saran yang dipilih. Pada sebagian besar prompt izin, dengan **Ya** atau **Tidak** fokus, membuka bidang komentar pada opsi itu, dan menekannya lagi menutup bidang. Lihat [tambahkan komentar saat Anda menjawab prompt izin](/docs/id/permissions#add-a-comment-when-you-answer-a-permission-prompt)                                                                                                                                                                                              |
| `Panah Atas/Bawah` atau `Ctrl+P`/`Ctrl+N`                                                          | Pindahkan kursor atau navigasi riwayat perintah                                                                                                                                                                                                                                 | Ketika input mencakup lebih dari satu baris visual, baik dibungkus atau multiline, pertama kali memindahkan kursor dalam prompt. Setelah kursor berada di baris visual pertama atau terakhir, menekan lagi menavigasi riwayat perintah. Sementara Anda memiliki pesan antrian, `Atas` dari baris pertama malah [membawanya kembali](#take-back-what-you-queued)                                                                                                                                                                                                        |
| `Esc`                                                                                              | Interupsi Claude, atau tutup dialog                                                                                                                                                                                                                                             | Hentikan respons atau panggilan alat saat ini di tengah giliran sehingga Anda dapat mengalihkan. Claude menyimpan pekerjaan yang dilakukan sejauh ini. Jika Anda memiliki [pesan antrian](#queue-messages-while-claude-works), Claude Code mengirimnya selanjutnya. Ketika dialog terbuka, `Esc` menutup dialog. Pada prompt izin, `Esc` menolak tindakan, sama seperti [**Tidak** tanpa komentar](/docs/id/permissions#add-a-comment-when-you-answer-a-permission-prompt)                                                                                                  |
| `Esc` + `Esc`                                                                                      | Hapus draf input, atau putar ulang                                                                                                                                                                                                                                              | Ketika input prompt berisi teks, `Esc` ganda menghapusnya dan menyimpan draf ke riwayat sehingga `Atas` mengingatnya. Ketika input kosong, `Esc` ganda membuka [menu putar ulang](/docs/id/checkpointing) untuk memulihkan atau merangkum kode dan percakapan dari titik sebelumnya                                                                                                                                                                                                                                                                                         |
| `Ctrl+Enter` atau `Ctrl+X Ctrl+S`                                                                  | Kirim pesan antrian sekarang                                                                                                                                                                                                                                                    | Mengirim [pesan antrian](#queue-messages-while-claude-works) Anda, dan draf Anda bersama mereka, keluar segera. [Ketika Claude Code mengirim apa yang Anda antrekan](#when-claude-code-sends-what-you-queued) mencakup apa yang terjadi pada giliran yang sedang dikerjakan Claude. Dalam [mode shell](#shell-mode-with-prefix), kunci hanya mengantrekan perintah Anda. Di terminal yang tidak melaporkan kunci diperluas, `Ctrl+Enter` tiba sebagai `Enter` biasa; `Ctrl+X Ctrl+S` bekerja di terminal apa pun. Memerlukan Claude Code v2.1.275 atau yang lebih baru |
| `Shift+Tab`, atau `Alt+M` di Windows ketika runtime Node atau Bun tidak mengaktifkan mode input VT | Mode izin siklus                                                                                                                                                                                                                                                                | Siklus melalui `default` (berlabel Manual dalam indikator mode), `acceptEdits`, `plan`, dan, ketika tersedia, `bypassPermissions` dan kemudian `auto`. Dari `auto`, tekan pertama beralih ke `default`. Lihat [mode izin](/docs/id/permission-modes). Pada prompt izin file, kunci yang sama menutup [bidang komentar](/docs/id/permissions#add-a-comment-when-you-answer-a-permission-prompt) yang terbuka. Tanpa bidang terbuka, ini memilih opsi yang memungkinkan tindakan untuk sisa sesi, ketika prompt menawarkan opsi itu                                                |
| `Option+P` (macOS) atau `Alt+P` (Windows/Linux)                                                    | Model switch                                                                                                                                                                                                                                                                    | Alihkan model tanpa menghapus prompt Anda                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `Option+T` (macOS) atau `Alt+T` (Windows/Linux)                                                    | Alihkan pemikiran diperpanjang                                                                                                                                                                                                                                                  | Aktifkan atau nonaktifkan mode pemikiran diperpanjang. Tidak berpengaruh pada Opus 5.5 atau model Fable, yang selalu menggunakan pemikiran diperpanjang. Bekerja di macOS tanpa mengonfigurasi Option sebagai Meta                                                                                                                                                                                                                                                                                                                                                     |
| `Option+O` (macOS) atau `Alt+O` (Windows/Linux)                                                    | Alihkan mode cepat                                                                                                                                                                                                                                                              | Aktifkan atau nonaktifkan [mode cepat](/docs/id/fast-mode)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |

<h3 id="text-editing">
  Pengeditan teks
</h3>

| Pintasan                     | Deskripsi                                | Konteks                                                                                                                                                                                                                      |
| :--------------------------- | :--------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `Ctrl+A`                     | Pindahkan kursor ke awal baris saat ini  | Dalam input multiline, pindah ke awal baris logis saat ini                                                                                                                                                                   |
| `Ctrl+E`                     | Pindahkan kursor ke akhir baris saat ini | Dalam input multiline, pindah ke akhir baris logis saat ini                                                                                                                                                                  |
| `Ctrl+K`                     | Hapus hingga akhir baris                 | Menyimpan teks yang dihapus untuk ditempel                                                                                                                                                                                   |
| `Ctrl+U`                     | Hapus dari kursor ke awal baris          | Menyimpan teks yang dihapus untuk ditempel. Ulangi untuk menghapus di seluruh baris dalam input multiline. Di macOS, emulator terminal termasuk iTerm2 dan Terminal.app memetakan `Cmd+Backspace` ke pintasan ini            |
| `Ctrl+W`                     | Hapus kembali ke spasi sebelumnya        | Menyimpan teks yang dihapus untuk ditempel. Satu tekan menghapus seluruh jalur atau `--flag=value`. Untuk menghapus hanya kata sebelumnya, tekan `Option+Delete` di macOS atau `Ctrl+Backspace` di Windows                   |
| `Ctrl+Y`                     | Tempel teks yang dihapus                 | Menempel teks yang terakhir Anda hapus dengan salah satu pintasan penghapusan kata atau baris, seperti `Ctrl+K`, `Ctrl+U`, atau `Ctrl+W`                                                                                     |
| `Alt+Y` (setelah `Ctrl+Y`)   | Riwayat tempel siklus                    | Setelah menempel, siklus melalui teks yang dihapus sebelumnya. Memerlukan [Option sebagai Meta](#keyboard-shortcuts) di macOS                                                                                                |
| `Alt+B`                      | Pindahkan kursor kembali satu kata       | Navigasi kata. Memerlukan [Option sebagai Meta](#keyboard-shortcuts) di macOS                                                                                                                                                |
| `Alt+F`                      | Pindahkan kursor maju satu kata          | Pindah ke akhir kata saat ini, atau ke akhir kata berikutnya ketika kursor berada di antara kata-kata. Memerlukan [Option sebagai Meta](#keyboard-shortcuts) di macOS                                                        |
| `Alt+D`                      | Hapus hingga akhir kata                  | Menghapus hingga akhir kata saat ini, atau hingga akhir kata berikutnya ketika kursor berada di antara kata-kata. Menyimpan teks yang dihapus untuk ditempel. Memerlukan [Option sebagai Meta](#keyboard-shortcuts) di macOS |
| `Ctrl+_` atau `Ctrl+Shift+-` | Batalkan pengeditan input terakhir       | Memulihkan teks input sebelumnya dan posisi kursor                                                                                                                                                                           |

<h3 id="make-ctrl-w-delete-back-to-whitespace">
  Batas kata dalam pintasan pengeditan
</h3>

Pintasan kata `Alt+B`, `Alt+F`, `Alt+D`, `Option+Delete`, dan `Ctrl+Backspace` memperlakukan kata sebagai rangkaian huruf dan digit, jadi tanda baca seperti `_`, `.`, dan `/` memisahkan kata-kata. Dengan `src/utils/foo.ts` dalam prompt, tekan berulang `Alt+B` berhenti di awal `ts`, `foo`, `utils`, dan `src`.

`Ctrl+W` berbeda: ini mengabaikan tanda baca dan menghapus kembali ke spasi sebelumnya, jadi satu tekan menghapus semua `src/utils/foo.ts`.

Dalam teks yang ditulis tanpa spasi, seperti Cina atau Jepang, pintasan kata masih bergerak atau menghapus satu kata pada satu waktu.

Konvensi readline ini berlaku di Claude Code v2.1.261 dan yang lebih baru. Pengaturan [`keybindingFlavor`](/docs/id/settings-reference#keybindingflavor) yang mengaktifkannya di versi sebelumnya sudah usang dan tidak berpengaruh.

Anda tidak dapat memetakan ulang pintasan ini dalam [file konfigurasi keybindings](/docs/id/keybindings), yang tidak memiliki tindakan untuk mereka.

<h3 id="theme-and-display">
  Tema dan tampilan
</h3>

| Pintasan | Deskripsi                                  | Konteks                                                                                                                 |
| :------- | :----------------------------------------- | :---------------------------------------------------------------------------------------------------------------------- |
| `Ctrl+T` | Alihkan penyorotan sintaks untuk blok kode | Hanya bekerja di dalam menu pemilih `/theme`. Mengontrol apakah kode dalam respons Claude menggunakan pewarnaan sintaks |

<h3 id="multiline-input">
  Input multiline
</h3>

| Metode         | Pintasan        | Konteks                                                                                                                                                                              |
| :------------- | :-------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Escape cepat   | `\` + `Enter`   | Bekerja di semua terminal                                                                                                                                                            |
| Tombol Option  | `Option+Enter`  | Setelah mengaktifkan [Option sebagai Meta](/docs/id/terminal-config#enable-option-key-shortcuts-on-macos) di macOS                                                                        |
| Shift+Enter    | `Shift+Enter`   | Asli di iTerm2, WezTerm, Ghostty, Kitty, Warp, Apple Terminal, Windows Terminal. Untuk terminal lain, lihat [Masukkan prompt multiline](/docs/id/terminal-config#enter-multiline-prompts) |
| Urutan kontrol | `Ctrl+J`        | Bekerja di terminal apa pun tanpa konfigurasi                                                                                                                                        |
| Mode tempel    | Tempel langsung | Untuk blok kode, log                                                                                                                                                                 |

<h3 id="quick-commands">
  Perintah cepat
</h3>

| Pintasan              | Deskripsi                      | Catatan                                                                                                                                                                                                                                                                                                                                                                                              |
| :-------------------- | :----------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `/` di awal           | Perintah atau skill            | Lihat [perintah](#commands) dan [skills](/docs/id/skills)                                                                                                                                                                                                                                                                                                                                                 |
| `!` di awal           | Mode shell                     | Jalankan perintah secara langsung, tambahkan outputnya ke sesi, dan biarkan Claude meresponsnya                                                                                                                                                                                                                                                                                                      |
| `@`                   | Penyebutan jalur file          | Picu pelengkapan otomatis jalur file. Dalam sesi dengan [cross-session messaging](/docs/id/cross-session-messaging#message-another-session), ketika Anda mengetik setidaknya satu huruf setelah `@`, Claude Code juga menyarankan sesi live lain Anda di mesin ini, sehingga Anda dapat memberi tahu Claude untuk mengirim pesan ke yang Anda pilih. Memerlukan Claude Code v2.1.232 atau yang lebih baru |
| `:`                   | Kode shortcode emoji           | Ketik `:name:` lengkap untuk menyisipkan emoji, atau dua atau lebih karakter untuk saran. Lihat [Kode shortcode emoji](#emoji-shortcodes). Memerlukan Claude Code v2.1.217 atau yang lebih baru                                                                                                                                                                                                      |
| `?` pada input kosong | Alihkan panel bantuan pintasan | Mengetik `?` ketika input sudah berisi teks menyisipkan karakter                                                                                                                                                                                                                                                                                                                                     |

<h3 id="transcript-viewer">
  Penampil transkrip
</h3>

Ketika penampil transkrip terbuka (dialihkan dengan `Ctrl+O`), pintasan ini tersedia. Jalankan `/tui` tanpa argumen untuk memeriksa renderer mana yang aktif. `Ctrl+E` dapat dipetakan ulang melalui [`transcript:toggleShowAll`](/docs/id/keybindings).

| Pintasan             | Deskripsi                                                                                                                                                                                                                    |
| :------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `?`                  | Alihkan panel bantuan pintasan keyboard. Memerlukan [rendering layar penuh](/docs/id/fullscreen)                                                                                                                                  |
| `{` / `}`            | Lompat ke prompt pengguna sebelumnya atau berikutnya, seperti gerakan paragraf vim. Memerlukan [rendering layar penuh](/docs/id/fullscreen)                                                                                       |
| `Ctrl+E`             | Alihkan tampilkan semua konten. Tersedia di renderer klasik saja, bukan di [rendering layar penuh](/docs/id/fullscreen)                                                                                                           |
| `[`                  | Tulis percakapan lengkap ke scrollback asli terminal Anda sehingga `Cmd+F`, mode salinan tmux, dan alat asli lainnya dapat mencarinya. Memerlukan [rendering layar penuh](/docs/id/fullscreen#search-and-review-the-conversation) |
| `v`                  | Tulis percakapan ke file sementara dan buka di `$VISUAL` atau `$EDITOR`. Memerlukan [rendering layar penuh](/docs/id/fullscreen)                                                                                                  |
| `q`, `Ctrl+C`, `Esc` | Keluar dari tampilan transkrip. Ketiganya dapat dipetakan ulang melalui [`transcript:exit`](/docs/id/keybindings)                                                                                                                 |

<h3 id="voice-input">
  Input suara
</h3>

| Pintasan                 | Deskripsi   | Catatan                                                                                                                                                                                              |
| :----------------------- | :---------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Tahan atau ketuk `Space` | Diksi suara | Memerlukan [diksi suara](/docs/id/voice-dictation) diaktifkan. Tahan untuk merekam, atau jalankan `/voice tap` untuk tap-to-toggle. [Dapat dipetakan ulang](/docs/id/voice-dictation#rebind-the-dictation-key) |

<h2 id="commands">
  Perintah
</h2>

Ketik `/` di Claude Code untuk melihat perintah yang tersedia untuk Anda, atau ketik `/` diikuti dengan huruf apa pun untuk memfilter. Menu `/` mencantumkan perintah bawaan, [skills](/docs/id/skills) yang dikemas dan ditulis pengguna, dan perintah yang disumbangkan oleh [plugins](/docs/id/plugins/overview) dan [server MCP](/docs/id/mcp#use-mcp-prompts-as-commands). Tidak semua perintah bawaan terlihat oleh setiap pengguna karena beberapa bergantung pada platform atau paket Anda, dan [beberapa perintah yang tersedia disembunyikan dari menu dengan desain](/docs/id/commands#how-the-command-menu-matches-what-you-type) dan berjalan saat Anda mengetik nama lengkapnya.

Dalam [rendering layar penuh](/docs/id/fullscreen#use-the-mouse), daftar saran perintah `/` dan file `@` juga merespons mouse: mengarahkan kursor menyoroti baris dan mengklik menerimanya.

Lihat [referensi perintah](/docs/id/commands) untuk daftar lengkap perintah yang disertakan dalam Claude Code.

<h3 id="complete-a-command-mid-prompt">
  Selesaikan perintah di tengah-prompt
</h3>

Penyelesaian perintah juga berfungsi di tengah-tengah prompt: ketik `/` setelah spasi, kemudian huruf pertama dari nama, seperti dalam `jalankan tes, kemudian /com`. Hanya perintah yang namanya dimulai dengan huruf-huruf tersebut yang cocok, jadi jalur file seperti `/tmp/notes.md` tidak membuat daftar tetap terbuka. Claude Code menjalankan perintah itu sendiri hanya ketika perintah [memulai pesan Anda](/docs/id/commands).

* **Dalam [rendering layar penuh](/docs/id/fullscreen)**: kecocokan terbuka sebagai daftar saat Anda mengetik, tanpa baris yang disorot, jadi `Enter` masih mengirim prompt Anda seperti yang diketik. Tekan `Tab` untuk menyisipkan kecocokan teratas, atau pilih baris dengan tombol panah dan `Enter`.
* **Di luar layar penuh**: kecocokan teratas yang tersisa muncul sebagai teks hantu di kursor Anda, dengan hitungan seperti `+2` ketika lebih banyak perintah cocok. Tekan `Tab` untuk menyisipkan satu-satunya kecocokan, atau untuk membuka daftar ketika beberapa cocok, kemudian pilih baris dengan tombol panah dan `Enter`.

Di kedua renderer, tekan `Tab` pada `/` kosong di tengah-prompt untuk mencantumkan setiap perintah.

Skill plugin cocok dengan nama barenya juga, jadi `/deploy` menemukan skill bernama `myplugin:deploy-app`. Ketika Anda menyisipkan kecocokan, Claude Code menulis `/myplugin:deploy-app` lengkap.

<h2 id="vim-editor-mode">
  Mode editor Vim
</h2>

Aktifkan pengeditan gaya vim melalui `/config` → Editor mode.

Claude Code menyimpan mode vim dan posisi kursor Anda saat Anda mengalihkan [penampil transkrip](#transcript-viewer) dengan `Ctrl+O` atau membuka dan menutup panel seperti `/config`. Jika Anda meninggalkan prompt dalam mode NORMAL, mode tersebut masih dalam mode NORMAL saat Anda kembali, dengan kursor di tempat Anda meninggalkannya.

<h3 id="mode-switching">
  Pengalihan mode
</h3>

| Perintah            | Tindakan                                                                                                              | Dari mode      |
| :------------------ | :-------------------------------------------------------------------------------------------------------------------- | :------------- |
| `Esc` atau `Ctrl+[` | Masuk mode NORMAL. Di terminal yang menggunakan protokol keyboard Kitty, `Ctrl+[` memerlukan v2.1.242 atau lebih baru | INSERT, VISUAL |
| `i`                 | Sisipkan sebelum kursor                                                                                               | NORMAL         |
| `I`                 | Sisipkan di awal baris                                                                                                | NORMAL         |
| `a`                 | Sisipkan setelah kursor                                                                                               | NORMAL         |
| `A`                 | Sisipkan di akhir baris                                                                                               | NORMAL         |
| `o`                 | Buka baris di bawah                                                                                                   | NORMAL         |
| `O`                 | Buka baris di atas                                                                                                    | NORMAL         |
| `v`                 | Mulai pemilihan visual berdasarkan karakter                                                                           | NORMAL         |
| `V`                 | Mulai pemilihan visual berdasarkan baris                                                                              | NORMAL         |

<h3 id="remap-insert-mode-key-sequences">
  Petakan ulang urutan kunci mode INSERT
</h3>

Pengaturan [`vimInsertModeRemaps`](/docs/id/settings-reference#viminsertmoderemaps) memetakan urutan mode INSERT dua kunci ke Escape, sehingga pemetaan seperti `jj` mengembalikan Anda ke mode NORMAL. Memerlukan Claude Code v2.1.208 atau lebih baru.

Contoh `~/.claude/settings.json` berikut mengaktifkan mode vim dan memetakan `jj` ke Escape:

```json theme={null}
{
  "editorMode": "vim",
  "vimInsertModeRemaps": { "jj": "<Esc>" }
}
```

Setiap kunci adalah tepat dua karakter yang dapat dicetak yang diketik secara berurutan, dan `"<Esc>"` adalah satu-satunya target yang didukung. Entri dengan panjang atau target yang berbeda diabaikan.

Mengetik karakter pertama dari urutan menyisipkannya secara normal. Menekan karakter kedua dalam satu detik menghapus karakter yang tertunda itu dan beralih ke mode NORMAL, meninggalkan tidak ada karakter dalam input Anda. Setelah jendela satu detik, atau jika kunci yang berbeda mengikuti, kedua karakter tetap sebagai teks literal, sehingga Anda masih dapat mengetik kata yang berisi urutan dengan berhenti di antara dua kunci.

Claude Code membaca pengaturan ini dari file pengaturan pengguna Anda, bendera `--settings`, dan [pengaturan terkelola](/docs/id/managed-settings) saja. Entri dalam `.claude/settings.json` atau `.claude/settings.local.json` proyek diabaikan, sehingga repositori yang diperiksa tidak dapat memetakan ulang penekanan tombol Anda.

<h3 id="navigation-normal-mode">
  Navigasi (mode NORMAL)
</h3>

| Perintah        | Tindakan                                                                                                                                                                        |
| :-------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `h`/`j`/`k`/`l` | Pindah kiri/bawah/atas/kanan                                                                                                                                                    |
| `Space`         | Pindah kanan                                                                                                                                                                    |
| `w`             | Kata berikutnya                                                                                                                                                                 |
| `e`             | Akhir kata                                                                                                                                                                      |
| `b`             | Kata sebelumnya                                                                                                                                                                 |
| `0`             | Awal baris                                                                                                                                                                      |
| `$`             | Akhir baris                                                                                                                                                                     |
| `^`             | Karakter non-kosong pertama                                                                                                                                                     |
| `gg`            | Awal input                                                                                                                                                                      |
| `G`             | Akhir input                                                                                                                                                                     |
| `f{char}`       | Lompat ke kemunculan karakter berikutnya                                                                                                                                        |
| `F{char}`       | Lompat ke kemunculan karakter sebelumnya                                                                                                                                        |
| `t{char}`       | Lompat ke tepat sebelum kemunculan karakter berikutnya                                                                                                                          |
| `T{char}`       | Lompat ke tepat setelah kemunculan karakter sebelumnya                                                                                                                          |
| `;`             | Ulangi gerakan f/F/t/T terakhir                                                                                                                                                 |
| `,`             | Ulangi gerakan f/F/t/T terakhir dalam urutan terbalik                                                                                                                           |
| `/`             | Buka pencarian riwayat terbalik, sama dengan `Ctrl+R`. Prompt pencarian kosong menunjukkan petunjuk: tekan `Esc` lalu `i` lalu `/` untuk membuka menu perintah sebagai gantinya |

<Note>
  Dalam mode NORMAL vim, jika kursor berada di awal atau akhir input dan tidak dapat bergerak lebih jauh, `j`/`k` dan `↑`/`↓` menavigasi riwayat perintah sebagai gantinya. `←` pada prompt kosong membuka [tampilan agen](/docs/id/agent-view) dari mode NORMAL serta INSERT; sebelum v2.1.219, `←` pada prompt kosong tidak melakukan apa pun dalam mode NORMAL.
</Note>

<h3 id="editing-normal-mode">
  Pengeditan (mode NORMAL)
</h3>

| Perintah              | Tindakan                                                                                                              |
| :-------------------- | :-------------------------------------------------------------------------------------------------------------------- |
| `x`                   | Hapus karakter                                                                                                        |
| `dd`                  | Hapus baris                                                                                                           |
| `D`                   | Hapus hingga akhir baris                                                                                              |
| `dw`/`de`/`db`        | Hapus kata/hingga akhir/kembali                                                                                       |
| `df{char}`/`dt{char}` | Hapus hingga dan termasuk, atau hingga, kemunculan karakter berikutnya                                                |
| `cc`                  | Ubah baris                                                                                                            |
| `C`                   | Ubah hingga akhir baris                                                                                               |
| `cw`/`ce`/`cb`        | Ubah kata/hingga akhir/kembali                                                                                        |
| `s`                   | Ganti karakter: hapus karakter di bawah kursor dan masuk mode INSERT. Memerlukan Claude Code v2.1.211 atau lebih baru |
| `S`                   | Ganti baris: kosongkan baris dan masuk mode INSERT. Memerlukan Claude Code v2.1.211 atau lebih baru                   |
| `yy`/`Y`              | Yank (salin) baris                                                                                                    |
| `yw`/`ye`/`yb`        | Yank kata/hingga akhir/kembali                                                                                        |
| `p`                   | Tempel setelah kursor                                                                                                 |
| `P`                   | Tempel sebelum kursor                                                                                                 |
| `>>`                  | Indentasi baris                                                                                                       |
| `<<`                  | Kurangi indentasi baris                                                                                               |
| `J`                   | Gabungkan baris                                                                                                       |
| `u`                   | Batalkan                                                                                                              |
| `.`                   | Ulangi perubahan terakhir                                                                                             |

<h3 id="text-objects-normal-mode">
  Objek teks (mode NORMAL)
</h3>

Objek teks bekerja dengan operator seperti `d`, `c`, dan `y`:

| Perintah  | Tindakan                                  |
| :-------- | :---------------------------------------- |
| `iw`/`aw` | Kata dalam/sekitar                        |
| `iW`/`aW` | KATA dalam/sekitar (dibatasi spasi putih) |
| `i"`/`a"` | Dalam/sekitar tanda kutip ganda           |
| `i'`/`a'` | Dalam/sekitar tanda kutip tunggal         |
| `i(`/`a(` | Dalam/sekitar tanda kurung                |
| `i[`/`a[` | Dalam/sekitar tanda kurung siku           |
| `i{`/`a{` | Dalam/sekitar tanda kurung kurawal        |

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

Claude Code menyimpan riwayat prompt yang Anda ketik, dan recall dengan tombol Up-arrow mengambil prompt dari sesi sebelumnya di proyek yang sama:

* Riwayat input disimpan per direktori kerja
* Menjalankan `/clear` memulai sesi baru: recall kemudian menampilkan prompt sesi baru terlebih dahulu, dengan prompt sesi sebelumnya setelahnya. Percakapan sesi sebelumnya disimpan dan dapat dilanjutkan.
* Mengirimkan prompt yang sama dua kali berturut-turut mencatat satu entri riwayat, jadi menekan Up melangkah ke prompt yang berbeda sebelumnya
* Ketika Anda mengingat kembali prompt yang menyertakan teks yang ditempel, Claude Code mengirimkan kembali konten yang ditempel penuh saat Anda mengirimkan ulang. Jika konten telah [dibersihkan](/docs/id/claude-directory#cleaned-up-automatically), Claude Code tidak mengirimkan string literal `[Pasted text #N]`; lihat [Paste large content](/docs/id/terminal-config#paste-large-content) untuk apa yang terjadi pada prompt
* Ekspansi riwayat dengan `!` dinonaktifkan secara default

<h3 id="reverse-search-with-ctrl-r">
  Pencarian terbalik dengan Ctrl+R
</h3>

Tekan `Ctrl+R` untuk mencari secara interaktif melalui riwayat perintah Anda. Dalam [fullscreen rendering](/docs/id/fullscreen), `Ctrl+R` membuka dialog pencarian sebagai gantinya: ketik untuk memfilter, tekan `Up` dan `Down` untuk bergerak melalui kecocokan, dan tekan `Ctrl+S` untuk mengubah cakupan melalui sesi ini, proyek ini, dan semua proyek. Tekan `Enter` atau `Tab` untuk menempatkan kecocokan dalam input prompt, atau `Esc` untuk membatalkan. Langkah-langkah di bawah menjelaskan pencarian inline renderer klasik:

1. **Mulai pencarian**: tekan `Ctrl+R` untuk mengaktifkan pencarian riwayat terbalik
2. **Ketik kueri**: masukkan teks untuk dicari di perintah sebelumnya. Istilah pencarian disorot dalam hasil yang cocok
3. **Navigasi kecocokan**: tekan `Ctrl+R` lagi untuk mengubah melalui kecocokan yang lebih lama
4. **Cakupan pencarian**: pencarian inline selalu mencari prompt dari semua proyek
5. **Terima kecocokan**:
   * Tekan `Tab` atau `Esc` untuk menerima kecocokan saat ini dan lanjutkan pengeditan
   * Tekan `Enter` untuk menerima dan menjalankan perintah segera
6. **Batalkan pencarian**:
   * Tekan `Ctrl+C` untuk membatalkan dan mengembalikan input asli Anda
   * Tekan `Backspace` pada pencarian kosong untuk membatalkan

Pencarian inline memindai riwayat prompt lengkap Anda, terbaru terlebih dahulu, dengan duplikat yang disatukan ke kemunculan terbaru. Dialog fullscreen mencari seluruh riwayat prompt Anda dalam cakupan yang dipilih, terbaru terlebih dahulu, dengan duplikat yang disatukan ke kemunculan terbaru: prompt paling baru muncul segera, dan kecocokan dari prompt yang lebih lama diisi saat Claude Code memuat sisanya. Prompt yang cocok ditampilkan dengan istilah pencarian disorot, sehingga Anda dapat menemukan dan menggunakan kembali input sebelumnya.

Menerima kecocokan atau membatalkan pencarian berlaku segera, bahkan saat Claude Code masih memuat riwayat.

<h2 id="background-bash-commands">
  Perintah Bash latar belakang
</h2>

Claude Code mendukung menjalankan perintah Bash di latar belakang, memungkinkan Anda untuk terus bekerja sementara proses yang berjalan lama dieksekusi.

<h3 id="how-backgrounding-works">
  Cara backgrounding bekerja
</h3>

Ketika Claude Code menjalankan perintah di latar belakang, perintah dijalankan secara asinkron dan segera mengembalikan ID tugas latar belakang. Claude Code dapat merespons prompt baru sementara perintah terus dieksekusi di latar belakang.

Untuk menjalankan perintah di latar belakang, Anda dapat:

* Meminta Claude Code untuk menjalankan perintah di latar belakang
* Tekan `Ctrl+B` untuk memindahkan invokasi alat Bash biasa ke latar belakang. Pengguna Tmux harus menekan `Ctrl+B` dua kali karena kunci awalan tmux.

**Fitur utama:**

* Output ditulis ke file dan Claude dapat mengambilnya menggunakan alat Read
* Tugas latar belakang memiliki ID unik untuk pelacakan dan pengambilan output
* Tugas latar belakang dibersihkan secara otomatis ketika Claude Code keluar. Di macOS dan Linux, ketika Anda menghentikan tugas latar belakang dari [`/tasks`](/docs/id/commands) atau Claude Code menghentikannya saat keluar, proses yang terlepas dari shell tugas, seperti yang dimulai di bawah `setsid` atau `timeout`, juga berhenti
* Jika Anda membuat sesi latar belakang alih-alih keluar, tugas latar belakang Anda terus berjalan di sesi latar belakang. Lihat [membuat sesi latar belakang](/docs/id/agent-view#from-inside-a-session)
* Tugas latar belakang secara otomatis dihentikan jika output melebihi 5GB, dengan catatan di stderr yang menjelaskan alasannya
* Di macOS dan Linux, Claude Code menghentikan tugas latar belakang yang berjalan ketika sistem operasi melaporkan tekanan memori kritis, asalkan sesi telah menganggur selama minimal 30 menit dan tidak ada putaran atau subagen yang berjalan. Memerlukan Claude Code v2.1.193 atau lebih baru
  * [Log debug](/docs/id/debug-your-config) mengatakan mengapa tugas dihentikan, atau mengapa peristiwa tekanan meninggalkan mereka tetap berjalan
  * Atur [`CLAUDE_CODE_DISABLE_BG_SHELL_PRESSURE_REAP`](/docs/id/env-vars) ke `1` untuk mematikan penghentian tekanan memori
* Perintah latar belakang yang dimiliki oleh [subagen](/docs/id/sub-agents) tidak memiliki batas waktu, kecuali bahwa perintah yang dimiliki oleh subagen yang berjalan di latar depan berakhir ketika subagen tersebut memberikan respons finalnya; lihat [Perintah latar belakang](/docs/id/tools-reference#background-commands) dalam referensi alat. Sebelum v2.1.218, baik reap tekanan memori maupun batas 60 menit sebelumnya pada perintah subagen tidak mencakup perintah yang dipindahkan ke latar belakang dengan `Ctrl+B`

Untuk menonaktifkan semua fungsi tugas latar belakang, atur variabel lingkungan `CLAUDE_CODE_DISABLE_BACKGROUND_TASKS` ke `1`. Lihat [Variabel lingkungan](/docs/id/env-vars) untuk detail.

**Perintah yang sering dibuat latar belakang:**

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
* Mendukung pelengkapan jalur file langsung sejak v2.1.193 di semua platform: ketik token yang berisi garis miring ke depan, seperti `./src/` atau `~/`, untuk melihat dropdown file dan direktori yang cocok, kemudian tekan `Tab` untuk menerima. Gunakan garis miring ke depan di Windows juga; dropdown dipicu oleh `/`, bukan `\`
* Keluar dengan `Escape`, `Backspace`, atau `Ctrl+U` pada prompt kosong
* Menempel teks yang dimulai dengan `!` ke prompt kosong secara otomatis memasuki mode shell, cocok dengan perilaku `!` yang diketik

Kecuali sesi Anda adalah salah satu yang tercantum di bawah [mode sandbox ketat](/docs/id/sandboxing#the-unsandboxed-retry-escape-hatch), perintah yang Anda ketik dalam mode shell berjalan di luar [sandbox](/docs/id/sandboxing) bahkan ketika Anda telah mengaktifkan sandboxing, karena sandbox berlaku untuk perintah yang Claude jalankan.

Claude merespons output perintah secara otomatis setelah mendarat di transkrip, sehingga Anda dapat menjalankan `! npm test` dan mendapatkan penjelasan tentang kegagalan tanpa prompt kedua. Respons memiliki biaya yang sama dengan mengirim prompt normal. Untuk mengembalikan perilaku sebelumnya di mana output ditambahkan ke konteks tanpa respons, atur [`respondToBashCommands`](/docs/id/settings-reference#respondtobashcommands) ke `false` dalam `settings.json`. Sebelum v2.1.186, mode shell selalu menambahkan output ke konteks tanpa respons.

<h2 id="queue-messages-while-claude-works">
  Antrekan pesan saat Claude bekerja
</h2>

Ketik pesan dan tekan `Enter` saat Claude sedang bekerja. Claude Code mengantrekan pesan alih-alih mengganggu giliran, dan menampilkan entri yang antri di atas kotak input hingga dikirimkan. Anda dapat mengantrekan perintah `!` [shell](#shell-mode-with-prefix) dan sebagian besar [perintah](/docs/id/commands) dengan cara yang sama, terlepas dari perintah seperti `/status` yang Claude Code jalankan segera setelah Anda mengirimkannya.

Pesan yang dikirim dan antri ditampilkan dalam warna abu-abu hingga Claude mulai merespons, sehingga Anda dapat mengetahui pesan mana yang belum dimulai oleh Claude.

<h3 id="when-claude-code-sends-what-you-queued">
  Kapan Claude Code mengirim apa yang Anda antrekan
</h3>

Kapan entri yang antri mencapai Claude tergantung pada apa yang Anda antrekan.

* Pesan: jika Anda mengantrekan pesan saat Claude menjalankan panggilan alat, Claude Code meneruskannya ke Claude segera setelah panggilan alat tersebut selesai, dalam giliran yang sama. Ketika giliran berakhir dengan pesan yang masih antri, mereka keluar tanpa penekanan tombol lain, dalam urutan yang Anda ketikkan
* Perintah dan perintah shell: Claude Code menahan mereka hingga giliran berakhir, kemudian menjalankannya satu per satu, menjaga urutan yang Anda antrekan

Untuk mengirim apa yang Anda antrekan tanpa menunggu, tekan `Ctrl+Enter`. Pesan antri Anda keluar segera, dengan draft Anda antri di belakangnya jika Anda telah mengetik satu. Memerlukan Claude Code v2.1.275 atau lebih baru.

Jika Anda mengantrekan perintah shell `!` di depan pesan Anda, tombol mengganggu giliran. Jika tidak, apa yang terjadi pada giliran tergantung pada apa yang Claude lakukan saat Anda menekan tombol:

* Menjalankan perintah shell, subagen, atau pekerjaan lain yang dapat dipindahkan ke [latar belakang](#background-bash-commands): pekerjaan itu berpindah ke latar belakang dan terus berjalan, dan Claude membaca pesan Anda dalam giliran yang sama
* Hanya menulis respons, atau menjalankan sesuatu yang tidak dapat dipindahkan ke latar belakang: Claude Code mengganggu giliran dan mengirimkan pesan Anda berikutnya. Sebelum v2.1.281, tombol mengganggu giliran dalam kedua kasus

Dalam [mode shell](#shell-mode-with-prefix), tombol hanya mengantrekan perintah Anda. Di terminal yang tidak melaporkan kunci yang diperluas, `Ctrl+Enter` tiba sebagai `Enter` biasa dan mengantrekan draft alih-alih; `Ctrl+X Ctrl+S` bekerja di terminal apa pun. Kedua tombol adalah pengikatan dari tindakan [`chat:sendNow`](/docs/id/keybindings#chat-actions).

Tekan `Esc` untuk mengganggu giliran tanpa mengirimkan draft Anda. Claude Code menyimpan apa yang Anda antrekan dan mengirimkannya segera.

Claude Code menjalankan beberapa perintah segera setelah Anda mengirimkannya alih-alih mengantrekannya, di antaranya `/model`, `/effort`, dan `/fast`. Masing-masing dari ketiga perintah mengubah pengaturan: model, tingkat upaya, atau mode cepat. Apakah Claude Code menerapkan pengaturan baru ke giliran yang sudah dikerjakan Claude, atau hanya dari giliran Anda berikutnya, berbeda menurut perintah:

* [`/model`](/docs/id/model-config#setting-your-model): setelah Anda mengonfirmasi [peringatan cache](/docs/id/prompt-caching#switching-models), jika Claude Code menampilkannya, Claude Code menerapkan perubahan Anda ke permintaan berikutnya yang dibuat dalam giliran itu
* [`/effort`](/docs/id/model-config#adjust-effort-level): setelah Anda mengonfirmasi [peringatan cache](/docs/id/prompt-caching#changing-effort-level), jika Claude Code menampilkannya, Claude Code menerapkan perubahan Anda ke permintaan berikutnya yang dibuat dalam giliran itu
* [`/fast`](/docs/id/fast-mode#toggle-fast-mode): Claude Code mempertahankan pengaturan mode cepat yang aktif saat giliran dimulai, jadi perubahan kecepatan Anda berlaku dari giliran Anda berikutnya. Jika model saat ini Anda tidak mendukung mode cepat, mengaktifkannya juga [beralih model Anda](/docs/id/prompt-caching#turning-on-fast-mode), dan Claude Code menggunakan model baru dari permintaan berikutnya dalam giliran itu

<h3 id="take-back-what-you-queued">
  Ambil kembali apa yang Anda antrekan
</h3>

Tekan `Up` dari baris pertama kotak input untuk mengambil kembali pesan dan perintah yang antri. Claude Code menghapusnya dari antrian dan menempatkannya di kotak input, satu per baris, di depan teks apa pun yang telah Anda ketik. Edit teks dan tekan `Enter` untuk mengantrekannya lagi sebagai satu entri, atau kosongkan kotak input untuk melepaskannya.

Claude Code mengambil kembali perintah shell yang antri hanya ketika kotak input kosong dan Anda tidak memiliki apa pun yang antri, dan beralih kotak input ke mode shell saat melakukannya. Jika tidak, Claude Code membiarkannya tetap dalam antrian, terdaftar dengan awalan `!` mereka, dan menjalankannya setelah giliran berakhir.

<h2 id="prompt-suggestions">
  Saran prompt
</h2>

Ketika Anda pertama kali membuka sesi, Claude Code menampilkan contoh perintah yang digelapkan dalam input prompt untuk membantu Anda memulai. Ini dipilih dari riwayat git proyek Anda, jadi contohnya mencerminkan file yang telah Anda kerjakan baru-baru ini.

Setelah Claude merespons, Claude Code dapat menyarankan prompt berikutnya berdasarkan riwayat percakapan Anda, seperti langkah lanjutan dari permintaan multi-bagian atau kelanjutan alami dari alur kerja Anda.

* Tekan `Tab` atau `Right arrow` untuk menempatkan saran dalam input prompt, kemudian `Enter` untuk mengirimkan
* Mulai mengetik untuk menolak saran

Claude Code menghasilkan setiap saran prompt berikutnya dengan permintaan latar belakang ke model yang sama yang digunakan sesi Anda. Permintaan dihitung terhadap batas penggunaan rencana Anda atau biaya API Anda. Karena menggunakan kembali prompt cache percakapan, sebagian besar adalah pembacaan cache ditambah beberapa token output, jadi biaya tambahan minimal.

<h3 id="when-claude-code-skips-suggestions">
  Ketika Claude Code melewatkan saran
</h3>

Dalam mode interaktif, Claude Code meninggalkan saran prompt dimatikan secara default dan menyembunyikan toggle **Prompt suggestions** dalam `/config` dalam [sesi yang tidak mengambil feature flags](/docs/id/env-vars#features-that-need-feature-flag-fetching), seperti sesi di penyedia pihak ketiga atau melalui gateway aplikasi Claude, dan dalam [sesi pertama setelah instalasi atau upgrade](/docs/id/env-vars#first-session-after-an-install-or-upgrade) yang flag-nya belum tiba.

Claude Code juga melewatkan saran individual dalam beberapa situasi, termasuk:

* Prompt cache dingin, untuk menghindari biaya yang tidak perlu
* Setelah giliran pertama percakapan, dalam beberapa sesi
* Respons sebelumnya berakhir dengan kesalahan
* Saat Anda berada dalam Plan Mode
* Akun Anda mendekati atau mencapai batas penggunaan. Untuk menjaga saran tetap aktif sampai Anda mencapai batas, atur [`CLAUDE_CODE_ENABLE_PROMPT_SUGGESTION`](/docs/id/env-vars) ke `true`. Sebelum v2.1.238, Claude Code melewatkan saran di dekat batas bahkan dengan variabel diatur ke `true`
* Dalam [agent team](/docs/id/agent-teams), dalam sesi rekan kerja secara default. Sesi pemimpin menampilkan saran

Dalam print mode, Claude Code tidak menghasilkan saran secara default. Lewatkan [`--prompt-suggestions`](/docs/id/cli-reference#cli-flags) dengan `-p "<prompt>" --output-format stream-json --verbose` untuk membuat Claude Code mengeluarkan pesan `prompt_suggestion` setelah setiap giliran yang menghasilkan satu. Generator melewatkan percakapan yang sangat pendek dan prompt cache dingin di sini juga, jadi query `-p` tunggal yang pendek dapat tidak mengeluarkan apa pun.

<h3 id="turn-prompt-suggestions-off">
  Matikan saran prompt
</h3>

Untuk menonaktifkan saran prompt sepenuhnya, gunakan salah satu dari berikut ini:

* Matikan **Prompt suggestions** dalam `/config`
* Atur [`promptSuggestionEnabled`](/docs/id/settings-reference#promptsuggestionenabled) ke `false` dalam file pengaturan Anda
* Atur variabel lingkungan [`CLAUDE_CODE_ENABLE_PROMPT_SUGGESTION`](/docs/id/env-vars) ke `false`, yang mengambil alih pengaturan:
  ```bash theme={null}
  export CLAUDE_CODE_ENABLE_PROMPT_SUGGESTION=false
  ```

Untuk mematikan saran prompt di seluruh organisasi, atur `promptSuggestionEnabled` ke `false` dalam [managed settings](/docs/id/managed-settings). Juga atur `CLAUDE_CODE_ENABLE_PROMPT_SUGGESTION` ke `false` di bawah kunci [`env`](/docs/id/settings-reference#env) yang dikelola sehingga pengguna tidak dapat mengaktifkannya kembali dengan variabel lingkungan mereka sendiri.

<h2 id="emoji-shortcodes">
  Emoji shortcodes
</h2>

Ketik `:` diikuti dengan emoji shortcode dalam input prompt untuk menyisipkan emoji. Memerlukan Claude Code v2.1.217 atau lebih baru.

* Ketik shortcode lengkap seperti `:heart:` dan Claude Code menggantinya dengan ❤️ segera setelah Anda mengetik `:` penutup
* Ketik `:` ditambah setidaknya dua karakter dari nama, seperti `:hea`, untuk membuka popup saran, kemudian tekan `Tab` atau `Enter` untuk menyisipkan emoji yang disorot

Shortcode harus memulai input atau mengikuti spasi, jadi `:` di dalam kata atau URL tidak membuka saran.

Untuk mematikan fitur ini, atur [`emojiCompletionEnabled`](/docs/id/settings-reference#emojicompletionenabled) ke `false` dalam `settings.json`. Ini menonaktifkan popup saran dan penggantian inline.

<h2 id="check-spelling-as-you-type">
  Periksa ejaan saat Anda mengetik
</h2>

Claude Code dapat menggarisbawahi kata-kata yang salah eja dalam input prompt saat Anda mengetik. Ini hanya memeriksa teks dalam kotak input, tidak pernah balasan Claude atau file Anda. Ini juga tidak memeriksa apa pun saat kotak input berada dalam [mode shell](#shell-mode-with-prefix), pencarian riwayat `Ctrl+R`, atau [dikte suara](/docs/id/voice-dictation).

Pemeriksaan ejaan dimatikan secara default, dan Claude Code tidak memeriksa apa pun dalam [mode pembaca layar](/docs/id/accessibility). Memerlukan Claude Code v2.1.235 atau lebih baru.

<h3 id="prerequisites">
  Prasyarat
</h3>

* Instal [aspell](https://github.com/GNUAspell/aspell), [hunspell](https://github.com/hunspell/hunspell), atau [ispell](https://en.wikipedia.org/wiki/Ispell) dan pastikan berada di `PATH` Anda. Claude Code menjalankan yang pertama dari ketiga program yang ditemukannya, dalam urutan itu, di setiap platform, termasuk shim `.cmd` yang diinstal pengelola paket di Windows.
* Untuk memeriksa bahwa program berada di `PATH` Anda, jalankan `aspell --version`, `hunspell --version`, atau `ispell -v` di terminal Anda. Kesalahan "command not found" berarti belum berada di `PATH` Anda.

<h3 id="turn-spell-checking-on-or-off">
  Nyalakan atau matikan pemeriksaan ejaan
</h3>

Claude Code membaca pengaturan [`spellcheck`](/docs/id/settings-reference#spellcheck) dari tiga tempat, dan mengabaikannya di `.claude/settings.json` dan `.claude/settings.local.json` proyek. Nyalakan dari mana pun yang Anda gunakan:

<Tabs>
  <Tab title="Pengaturan pengguna">
    Tambahkan `spellcheck` ke `~/.claude/settings.json`. Ini berlaku di setiap proyek yang Anda buka, seperti sisa [pengaturan pengguna](/docs/id/settings#where-settings-live) Anda:

    ```json theme={null}
    {
      "spellcheck": { "enabled": true }
    }
    ```
  </Tab>

  <Tab title="Baris perintah">
    Simpan `spellcheck` dalam file JSON, seperti `spellcheck.json`:

    ```json theme={null}
    {
      "spellcheck": { "enabled": true }
    }
    ```

    Kemudian teruskan file ke `--settings`. Ini berlaku untuk sesi itu saja:

    ```bash theme={null}
    claude --settings spellcheck.json
    ```
  </Tab>

  <Tab title="Pengaturan terkelola">
    Tambahkan `spellcheck` ke salah satu [sumber pengaturan terkelola](/docs/id/permissions#managed-settings) organisasi Anda. Ini berlaku untuk setiap pengguna yang menerima pengaturan tersebut, dan mereka tidak dapat mematikannya:

    ```json theme={null}
    {
      "spellcheck": { "enabled": true }
    }
    ```
  </Tab>
</Tabs>

Untuk memeriksa bahwa pemeriksaan ejaan aktif, ketik kata yang salah eja dan spasi. Claude Code menggarisbawahi kata tersebut. Jika tidak, lihat [Ketika Claude Code tidak menggarisbawahi apa pun](#when-claude-code-underlines-nothing). Untuk mematikan pemeriksaan ejaan lagi, atur `enabled` ke `false` di tempat yang sama, atau hapus `spellcheck`.

Untuk memilih program mana dari ketiga program yang Claude Code jalankan, kamus mana yang digunakan, atau warna garis bawah, tambahkan salah satu dari bidang ini di sebelah `enabled`, di tempat yang sama:

* `checker`: `aspell`, `hunspell`, atau `ispell`. Claude Code tidak kembali dari pemeriksa yang Anda beri nama, dan memperlakukan nilai apa pun sebagai `auto`.
* `language`: nama kamus dalam bentuk pemeriksa Anda, seperti `en_GB`. Claude Code mengabaikan nilai apa pun yang bukan nama kamus biasa, seperti jalur atau nama dengan spasi, dan pemeriksa menggunakan kamus defaultnya.
* `color`: nama warna seperti `yellow`, atau nilai `#rrggbb`, `#rgb`, `rgb(r,g,b)`, `ansi256(n)`, atau `ansi:<name>`. Claude Code menggunakan warna kesalahan tema Anda secara default dan untuk nilai apa pun yang tidak dikenalinya.

Sebagai contoh, pengaturan `spellcheck` ini menjalankan hunspell dengan kamus `en_GB` dan menggarisbawahi kata-kata dalam warna kuning. Ini bekerja sama di `~/.claude/settings.json`, dalam file yang Anda teruskan ke `--settings`, dan dalam pengaturan terkelola:

```json theme={null}
{
  "spellcheck": {
    "enabled": true,
    "checker": "hunspell",
    "language": "en_GB",
    "color": "yellow"
  }
}
```

Jika lebih dari satu dari tiga tempat memiliki pengaturan `spellcheck`, Claude Code hanya menggunakan satu dari mereka: pengaturan terkelola terlebih dahulu, kemudian `--settings`, kemudian pengaturan pengguna. Ini tidak menggabungkan bidang dari dua tempat. Sebagai contoh, ketika `--settings` menetapkan `spellcheck`, `language` dalam pengaturan pengguna Anda tidak berpengaruh.

<h3 id="what-claude-code-underlines">
  Apa yang Claude Code garisbawahi
</h3>

Tidak lama setelah Anda berhenti mengetik, Claude Code menggarisbawahi kata-kata yang tidak diketahui kamus. Ini membiarkan kata yang masih Anda ketik sendirian sampai Anda melewatinya, dan tidak pernah mengubah teks Anda. Ini juga melewati teks yang terlihat seperti kode:

* Perintah seperti `/help`, penyebutan `@`, URL, jalur file, dan bendera seperti `--verbose`
* Kata-kata dengan digit, garis bawah, atau huruf besar setelah yang pertama, dan teks dalam backtick

Claude Code juga melewati teks Cina, Jepang, Korea, Thai, Lao, Khmer, dan Myanmar.

Claude Code tidak memiliki daftar kata sendiri: sebuah kata salah eja ketika pemeriksa Anda mengatakan demikian. Untuk menghentikan Claude Code dari menggarisbawahi sebuah kata, tambahkan kata tersebut ke kamus pribadi pemeriksa Anda, mengikuti dokumentasi pemeriksa sendiri. Claude Code mengambil kata baru setelah Anda memulainya ulang.

<h3 id="when-claude-code-underlines-nothing">
  Ketika Claude Code tidak menggarisbawahi apa pun
</h3>

Claude Code tidak menggarisbawahi apa pun ketika tidak dapat menjaga pemeriksa tetap berjalan:

* Tidak ada pemeriksa yang diinstal, atau yang Anda beri nama dalam `checker` hilang
* Pemeriksa gagal dua kali berturut-turut, saat startup atau nanti dalam sesi. Claude Code memulainya ulang setelah kegagalan pertama dan berhenti memeriksa setelah yang kedua, sampai Anda memulai ulang Claude Code
* Pemeriksa membutuhkan waktu lebih dari 15 detik untuk menjawab, tiga kali. Setiap kali, Claude Code membiarkan kata-kata yang ditungguinya tidak ditandai; setelah yang ketiga, berhenti memeriksa sampai Anda memulai ulang Claude Code

Untuk mengetahui mana dari ini yang terjadi, mulai `claude --debug` dengan pemeriksaan ejaan aktif dan ketik sebuah kata. Kemudian cari baris `[spellcheck]` dalam log debug di `~/.claude/debug/<session-id>.txt`. Satu baris menamai program yang dimulai Claude Code, atau mencantumkan yang dicarinya dan tidak ditemukan. Baris-baris kemudian mengatakan mengapa berhenti. Kesalahan kamus yang hilang di sana berarti pemeriksa tidak memiliki kamus untuk nilai `language` Anda, atau tidak ada default ketika `language` tidak diatur. Instal satu, atau atur `language` ke kamus yang Anda miliki.

<h2 id="invisible-characters-in-prompts">
  Karakter tak terlihat dalam prompt
</h2>

Teks yang ditempel dapat membawa karakter Unicode yang tidak ditampilkan terminal sama sekali, seperti karakter tag, kontrol bidireksional, dan spasi lebar nol, sehingga prompt dapat berisi teks yang tidak pernah Anda lihat. Untuk mencegah teks yang disalin membawa instruksi yang tidak ditampilkan terminal, Claude Code menghapus karakter-karakter tersebut saat Anda menekan Enter, sebelum mengirim apa pun. Ini membersihkan prompt dan konten dari setiap [referensi teks yang ditempel](/docs/id/terminal-config#paste-large-content) yang disertakan prompt. Claude Code mempertahankan penyambung yang ditulis skrip Persia dan Indik serta pemilih di dalam urutan emoji.

Jika Claude Code menghapus apa pun, Enter tersebut tidak mengirim apa pun. Prompt yang dibersihkan kembali ke kotak input dengan pemberitahuan seperti `Removed 3 invisible characters · review and press Enter to send`, dan menekan Enter lagi mengirim teks seperti yang ditampilkan.

Saat Anda melewatkan prompt di baris perintah, seperti dalam `claude "fix the login bug"`, atau menyalurkannya ke sesi interaktif, Claude Code tidak menunggu Enter kedua. Ini menghapus karakter, menampilkan pemberitahuan, dan mengirim prompt yang dibersihkan. Jika prompt yang dibersihkan dimulai dengan `/`, Claude Code menempatkannya di kotak input agar Anda dapat meninjau dan mengirimnya.

<h2 id="review-changes-with-/diff">
  Tinjau perubahan dengan /diff
</h2>

Jalankan `/diff` untuk melihat perubahan di pohon kerja Anda tanpa meninggalkan Claude Code. Anda melihat pengeditan yang telah dilakukan Claude sejauh ini bersama dengan apa pun yang belum Anda komitkan.

Dalam perubahan yang dibaca `/diff` dari git, submodul muncul sebagai satu entri, dan hanya ketika komit yang ditunjuknya berubah; pengeditan ke file di dalam submodul tidak muncul di sana.

Dalam [rendering fullscreen](/docs/id/fullscreen), `/diff` membuka [panel diff](#diff-panel) di samping percakapan, yang tetap terbuka dan diperbarui saat Anda terus bekerja. Dalam renderer klasik, `/diff` membuka [diff viewer](#diff-viewer) menggantikan prompt, dan Anda menutupnya setelah selesai membaca.

<h3 id="diff-panel">
  Diff panel
</h3>

Panel diff mencantumkan file yang diubah dengan jumlah baris yang ditambahkan dan dihapus, dan menampilkan diff setiap file di bawah daftar. Claude Code menyegarkannya setiap kali Claude mengedit file atau menjalankan perintah shell. Untuk menutupnya, jalankan `/diff` lagi atau klik `✕` di headernya.

Untuk menggunakan panel Anda memerlukan:

* [Rendering fullscreen](/docs/id/fullscreen)
* Repositori git
* Terminal setidaknya 110 kolom lebar
* Claude Code v2.1.260 atau lebih baru

Ketika panel tidak dapat dibuka, `/diff` membuka diff viewer sebagai gantinya atau memberi tahu Anda alasannya.

Panel juga membuka dengan sendirinya setelah Claude mulai mengedit file, jika terminal Anda setidaknya 144 kolom lebar. Setelah Anda membukanya sendiri dengan `/diff`, sesi berikutnya membukanya segera setelah Claude mengedit file di terminal apa pun yang cukup lebar untuk menampungnya. Tutup panel dan panel tetap tertutup, dalam sesi ini dan sesi berikutnya, sampai Anda menjalankan `/diff` lagi.

Saat panel terbuka, Anda dapat:

* **Lompat ke file**: klik barisnya dalam daftar. Gulir panel dengan roda mouse. Ketika daftar file itu sendiri terlalu panjang untuk muat, gulir dengan `Alt+Up` dan `Alt+Down`, atau `Ctrl+Up` dan `Ctrl+Down`.
* **Tanyakan Claude tentang baris tertentu**: pilih baris tersebut di panel dengan mouse. Claude Code melampirkan pilihan ke prompt berikutnya Anda dan menampilkan jumlah baris di samping input sampai Anda mengirimnya.
  * Untuk mengirim prompt tanpa pilihan, pindahkan kursor ke tepat setelah indikator jumlah baris dan tekan `Backspace` untuk menghapusnya. Memerlukan Claude Code v2.1.271 atau lebih baru.
* **Tampilkan file yang ditinggalkan panel**: daftar melewati file uji dan file yang dihasilkan, dan menciutkan perubahan dari sebelum sesi ini menjadi satu baris di bagian bawah. Klik salah satu baris hitungan untuk memperluasnya.
* **Ubah apa yang dibandingkan panel**: tekan `Ctrl+X B` untuk beralih dari perubahan sesi ini, ke perubahan yang tidak dikomitkan sebagai satu daftar, ke semuanya sejak cabang Anda terpisah dari cabang default. Claude Code mengingat pilihan untuk setiap proyek.

Untuk mengikat kunci ke tindakan ini, lihat [Diff panel actions](/docs/id/keybindings#diff-panel-actions).

<h3 id="diff-viewer">
  Diff viewer
</h3>

Diff viewer menggantikan prompt sampai Anda menutupnya. Tampilan **Current** menunjukkan perubahan yang tidak dikomitkan dari git, atau, ketika tidak ada, apa yang ditambahkan cabang Anda di atas cabang default. Viewer juga memiliki tampilan turn untuk setiap prompt setelah Claude mengedit file, menampilkan hanya pengeditan tersebut. Claude Code membangun tampilan turn dari pengeditan file Claude daripada dari git, jadi perubahan yang dilakukan Claude melalui perintah shell hanya muncul di bawah Current.

Gunakan kunci ini di viewer:

* **Left dan Right**: bergerak antara Current dan tampilan turn.
* **Up dan Down**: pilih file.
* **Enter**: buka diff file yang dipilih. Gulir dengan Up dan Down, atau PageUp dan PageDown.
* **Esc**: kembali dari diff file ke daftar, atau tutup viewer dari daftar.

Untuk mengikat ulang kunci ini, lihat [Diff actions](/docs/id/keybindings#diff-actions).

<h2 id="side-questions-with-/btw">
  Pertanyaan sampingan dengan /btw
</h2>

Gunakan `/btw` untuk mengajukan pertanyaan tentang pekerjaan Anda saat ini tanpa menambahkannya ke riwayat percakapan.

```
/btw what was the name of that config file again?
```

Claude menjawab pertanyaan sampingan dari apa yang sudah ada dalam percakapan: pesan Anda, balasannya, dan hasil alat yang telah dikumpulkannya. Anda dapat bertanya tentang kode yang telah dibaca Claude, keputusan yang dibuatnya sebelumnya, atau apa pun lainnya dari sesi tersebut. Pertanyaan sampingan yang lebih baru juga melihat pertanyaan sampingan Anda sebelumnya: Claude Code memutar ulang 20 pertukaran terbaru dengan setiap pertanyaan, sampai Anda menghapusnya. Pertanyaan dan jawaban tidak pernah masuk ke riwayat percakapan. Di terminal, mereka muncul dalam overlay yang dapat ditutup. Terminal menyimpan thread dalam memori: tekan `x` untuk menghapus pertukaran sebelumnya, dan itu akan hilang saat Anda keluar dari Claude Code.

Di [panel chat VS Code extension](/docs/id/vs-code#use-the-prompt-box), `/btw` membuka panel daripada overlay yang dijelaskan bagian ini, dan Anda mengajukan pertanyaan lanjutan langsung di panel. Thread panel bertahan dari reload jendela, sesuai jadwal retensi yang dijelaskan halaman tersebut. Anda memerlukan extension pada v2.1.227 atau lebih baru. Versi extension yang lebih awal tidak menawarkan `/btw`.

* **Tersedia saat Claude sedang bekerja**: Anda dapat menjalankan `/btw` bahkan saat Claude memproses respons. Pertanyaan sampingan berjalan secara independen dan tidak mengganggu giliran utama. Ini melihat semua yang ada dalam percakapan sejauh ini, kecuali balasan yang masih ditulis Claude.
* **Tidak ada akses alat**: pertanyaan sampingan hanya menjawab dari apa yang sudah ada dalam konteks. Claude tidak dapat membaca file, menjalankan perintah, atau mencari saat menjawab pertanyaan sampingan. Jika Claude menulis panggilan alat sebagai teks bagaimanapun, jawaban berakhir dengan catatan bahwa tidak ada yang dieksekusi.
* **Respons tunggal**: tidak ada giliran lanjutan dalam overlay. Untuk melanjutkan thread, ajukan pertanyaan `/btw` lainnya. Untuk melanjutkan dengan akses alat penuh dalam sesi lokal, tekan `f` untuk fork pertanyaan dan jawaban ini ke dalam [subagent latar belakang](/docs/id/sub-agents#fork-the-current-conversation).
* **Biaya rendah**: saat [prompt cache](/docs/id/prompt-caching) percakapan hangat, pertanyaan sampingan biayanya sedikit di luar jawaban itu sendiri.

Lima pertanyaan sampingan sebelumnya terbaru Anda muncul sebagai daftar yang redup di atas jawaban saat ini, dengan hitungan yang lebih lama. Mereka tetap keluar dari riwayat percakapan.

Untuk kembali ke overlay setelah menutupnya, jalankan `/btw` tanpa pertanyaan. Overlay dibuka kembali pada pertukaran paling baru Anda. Sebelum v2.1.212, `/btw` tanpa pertanyaan mencetak pesan penggunaan sebagai gantinya.

Setelah jawaban muncul, overlay menerima kunci-kunci ini.

| Kunci                        | Tindakan                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| :--------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `Space`, `Enter`, `Escape`   | Tutup jawaban dan kembali ke prompt                                                                                                                                                                                                                                                                                                                                                                                                               |
| `Up` / `Down`                | Gulir jawaban                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `Shift+Left` / `Shift+Right` | Langkah antara jawaban ini dan jawaban `/btw` sebelumnya Anda. `Shift+Left` bergerak ke jawaban yang lebih lama dan `Shift+Right` kembali ke yang saat ini. `[` dan `]` melakukan hal yang sama, untuk terminal yang tidak melaporkan `Shift` dengan tombol panah. `Tab` / `Shift+Tab` bersiklus melalui jawaban yang sama. Memerlukan Claude Code v2.1.257 atau lebih baru. Antara v2.1.187 dan v2.1.256, kuncinya adalah `Left` / `Right` biasa |
| `c`                          | Salin jawaban ke clipboard Anda sebagai Markdown mentah. Gunakan ini alih-alih pemilihan mouse, yang menangkap rendering terminal yang dibungkus keras daripada teks sumber                                                                                                                                                                                                                                                                       |
| `f`                          | Mulai [subagent yang di-fork](/docs/id/sub-agents#fork-the-current-conversation) yang mewarisi percakapan induk ditambah pertanyaan dan jawaban ini, sehingga dapat melanjutkan dengan akses alat penuh. Anda tetap dalam sesi saat ini dan menemukan fork di [panel di bawah prompt Anda](/docs/id/sub-agents#observe-and-steer-running-forks). Tersedia hanya dalam sesi lokal                                                                            |
| `x`                          | Hapus daftar pertukaran `/btw` sebelumnya yang ditampilkan di atas jawaban saat ini                                                                                                                                                                                                                                                                                                                                                               |

Dalam [sesi latar belakang](/docs/id/agent-view#attach-to-a-session) yang terpasang, `Left` melepaskan dan mengembalikan Anda ke tampilan agen, bahkan saat jawaban masih tiba. Pertanyaan sampingan terus berjalan saat Anda pergi. Lain kali Anda melampirkan ke sesi, overlay dibuka kembali dengan pertanyaan sampingan, atau dengan jawabannya. Sebelum v2.1.257, `Left` tidak melepaskan di sana.

`/btw` melihat percakapan lengkap Anda tetapi tidak memiliki alat. [Subagent](/docs/id/sub-agents) memiliki alat dan dimulai dari prompt yang diterimanya, atau, untuk [fork](/docs/id/sub-agents#fork-the-current-conversation), dari salinan percakapan ini. Gunakan `/btw` untuk bertanya tentang apa yang sudah diketahui Claude dari sesi ini; gunakan subagent untuk menemukan sesuatu yang baru.

<h2 id="task-list">
  Daftar tugas
</h2>

Daftar tugas adalah checklist to-do Claude: item yang dibuat Claude untuk merencanakan pekerjaan multi-langkah, dengan indikator yang menunjukkan apa yang tertunda, sedang berlangsung, atau selesai. Ini terpisah dari tampilan background-task. Untuk melihat shell yang berjalan dan subagent, gunakan [`/tasks`](/docs/id/commands) sebagai gantinya.

Daftar ini hanya terisi dalam sesi yang memiliki task-tracking tools, yang disediakan Claude Code secara default pada [model Claude 3.x, Opus 4 hingga 4.7, Sonnet 4 hingga 4.6, dan Haiku 4.5](/docs/id/tools-reference#task-tool-availability). Pada model lain apa pun, termasuk ID model yang tidak dikenali Claude Code, daftar tetap kosong kecuali Anda memilih dengan `CLAUDE_CODE_ENABLE_TODO_TOOLS=1` atau salah satu cara lain di bawah [Task tool availability](/docs/id/tools-reference#task-tool-availability). Ketika sesi memiliki tools, daftar tugas bekerja sebagai berikut:

* Tekan `Ctrl+T` untuk mengalihkan tampilan daftar tugas. Tampilan menunjukkan hingga lima tugas sekaligus. Ketika Claude belum membuat item checklist apa pun, toggle tidak memiliki efek yang terlihat karena tidak ada yang ditampilkan
* Jika Anda membiarkan daftar tetap diperluas, Claude Code memulihkan tampilan yang diperluas saat Anda meluncurkan sesi berikutnya yang masih memiliki tugas, seperti dengan `--resume` atau `--continue`. Ketika daftar tugas kosong, Claude Code memulainya dalam keadaan tertutup
* Untuk melihat semua tugas atau menghapusnya, minta Claude secara langsung: "show me all tasks" atau "clear all tasks"
* Tugas bertahan di seluruh pemadatan konteks, membantu Claude tetap terorganisir pada proyek yang lebih besar
* Untuk berbagi daftar tugas di seluruh sesi, atur `CLAUDE_CODE_TASK_LIST_ID` untuk menggunakan direktori bernama di `~/.claude/tasks/`: `CLAUDE_CODE_TASK_LIST_ID=my-project claude`

<h2 id="session-recap">
  Rekap sesi
</h2>

Ketika Anda kembali ke terminal setelah meninggalkannya, Claude Code menampilkan rekap satu baris tentang apa yang terjadi dalam sesi sejauh ini. Rekap dihasilkan di latar belakang setelah setidaknya tiga menit telah berlalu sejak giliran terakhir yang selesai dan terminal tidak fokus, sehingga siap ketika Anda beralih kembali. Rekap hanya muncul setelah sesi memiliki setidaknya tiga giliran, dan tidak pernah dua kali berturut-turut.

Jalankan `/recap` untuk menghasilkan ringkasan sesuai permintaan. Claude Code membatasi rekap otomatis dan output `/recap` hingga 400 karakter. Untuk mematikan rekap otomatis, buka `/config` dan matikan **Session recap**.

Rekap sesi aktif secara default untuk setiap paket dan penyedia. Rekap selalu dilewati dalam mode non-interaktif.

<h2 id="wait-for-a-usage-limit-to-reset">
  Tunggu batas penggunaan untuk direset
</h2>

Ketika [batas penggunaan](/docs/id/errors#youve-hit-your-session-limit) claude.ai menghentikan Claude di tengah tugas, Claude Code menunggu dalam sesi yang terbuka dan melanjutkan tugas secara otomatis setelah batas direset. Lanjutan otomatis aktif secara default dalam sesi interaktif yang masuk dengan langganan claude.ai. Memerlukan Claude Code v2.1.234 atau lebih baru.

Saat Claude Code menunggu, baris di bagian bawah sesi menunjukkan kapan akan melanjutkan:

```text theme={null}
Usage limit reached · continuing automatically at 3:45pm · esc to cancel
```

Jaga sesi tetap terbuka. Apa yang terjadi selanjutnya tergantung pada bagaimana penantian berakhir:

* **Saat reset**: baris membaca `continuing shortly`, kemudian `Usage limit reset · continuing automatically`, dan Claude Code mengirimkan Claude prompt tetap untuk melanjutkan tugas dari tempat berhenti. Ini tidak mengirim ulang pesan terakhir Anda.
* **Setelah komputer Anda tidur**: jika tidur lebih dari sekitar 30 menit dan batas direset saat tidur, baris membaca `Your usage limit has reset · press enter to continue`. Tekan `Enter` untuk melanjutkan. Setelah tidur yang lebih singkat, Claude Code melanjutkan dengan sendirinya.
* **Lebih awal**: ketika Anda selesai menambahkan [kredit penggunaan](/docs/id/costs#add-usage-credits-to-your-subscription) dengan `/usage-credits`, masuk kembali setelah `/upgrade`, atau beralih model dengan `/model` selama penantian, Claude Code memeriksa apakah penggunaan tersedia lagi dan melanjutkan segera jika tersedia. Ini tidak memeriksa setelah upgrade atau pembelian yang Anda lakukan di browser sendiri. Di bawah [`opusplan`](/docs/id/model-config#opusplan-model-setting) dan pengaturan model lainnya yang menjalankan plan mode pada model yang berbeda, Claude Code menunggu reset sebagai gantinya.

Tugas yang dilanjutkan berjalan seperti putaran lainnya. Claude Code masih meminta [izin](/docs/id/permissions) seperti biasa, sehingga tugas dapat berhenti pada prompt saat Anda pergi. Jika mencapai batas lagi, Claude Code mengaktifkan kembali penantian dengan sendirinya paling banyak dua kali berturut-turut, kemudian berhenti dan menampilkan `Automatic continue stopped after repeated usage-limit hits · /rate-limit-options to try again`.

<h3 id="cancel-the-wait">
  Batalkan penantian
</h3>

Tekan `Esc` pada prompt kosong, atau `Ctrl+C`, saat baris ditampilkan, atau jalankan [`/rate-limit-options`](/docs/id/commands#all-commands) dan pilih **Don't continue automatically**. Claude Code mengonfirmasi dengan baris yang dimulai dengan `Automatic continue cancelled`.

Setelah pembatalan, tidak ada yang melanjutkan sampai Anda mengirim prompt atau memilih baris yang dimulai dengan **Wait here, then continue automatically** dari `/rate-limit-options` lagi. Claude Code tidak memulai penantian dengan sendirinya lagi untuk jendela reset itu; jendela reset berikutnya dimulai segar.

Penantian juga berakhir tanpa melanjutkan tugas dalam kasus-kasus ini:

* **Anda mengirim prompt**: Claude Code menjalankan prompt Anda alih-alih menunggu.
* **Anda keluar dari Claude Code**: penantian tidak dimulai ulang saat Anda melanjutkan sesi.
* **Percakapan berubah tangan**: Anda beralih akun dengan `/login`, menghapus atau memutar ulang percakapan, `/resume` sesi lain, menarik satu dengan `/teleport`, meluncurkan ulang dengan `/tui`, atau menyerahkan sesi ke Claude Desktop, sesi latar belakang, atau cloud.
* **Pengaturan mati, atau reset bergerak melampaui 24 jam**: ini hanya mengakhiri penantian yang Claude Code mulai dengan sendirinya. Penantian yang Anda pilih dari `/rate-limit-options` terus menghitung mundur.
* **Kelanjutan diblokir**: hook [`UserPromptSubmit`](/docs/id/hooks#userpromptsubmit) yang memblokir prompt kelanjutan, atau kegagalan sebelum mencapai model, mengakhiri penantian. Claude Code memberi tahu Anda bahwa kelanjutan tidak berjalan. Kirim prompt untuk melanjutkan.

<h3 id="start-a-wait-yourself">
  Mulai penantian sendiri
</h3>

Claude Code tidak memulai penantian dengan sendirinya dalam kasus-kasus ini:

* **Sesi Remote Control dan agent team teammate**: seseorang di terminal itu masih dapat memulai satu.
* **Reset lebih dari 24 jam**: batas mingguan dapat direset berhari-hari.
* **Batas Opus atau Sonnet saat Anda menjalankan model di luar keluarga itu**: putaran berikutnya Anda mungkin tidak mencapai batas itu. [`opusplan`](/docs/id/model-config#opusplan-model-setting) dan pengaturan model lainnya yang menjalankan plan mode pada keluarga terbatas tidak mendapatkan pengecualian ini.

Dalam kasus-kasus itu, dan kapan pun lanjutan otomatis mati, Claude Code membuka menu opsi batas penggunaan sekali per jendela reset ketika Anda mencapai batas di terminal Anda sendiri. Pilih baris yang dimulai dengan **Wait here, then continue automatically** untuk memulai penantian. Dalam sesi [Remote Control](/docs/id/remote-control) atau [agent team](/docs/id/agent-teams) teammate, jalankan `/rate-limit-options` sendiri untuk membuka menu.

Claude Code tidak menawarkan penantian sama sekali dalam kasus-kasus ini:

* **Sesi latar belakang dan `-p` runs**: baris menu tidak tersedia.
* **Kunci API, penyedia cloud, dan penagihan berbasis penggunaan**: penggunaan di sana diukur per permintaan, jadi tidak ada reset untuk ditunggu.
* **[LLM gateway](/docs/id/llm-gateway#subscriptions-and-gateways) tanpa login claude.ai yang disimpan**: Claude Code menawarkan penantian hanya saat login claude.ai yang disimpan adalah kredensial aktif.

<h3 id="turn-automatic-continue-off">
  Matikan lanjutan otomatis
</h3>

Di `/config`, matikan **Continue automatically at usage limit**, atau atur [`autoContinueAtUsageLimit`](/docs/id/settings-reference#autocontinueatusagelimit) ke `false` dalam pengaturan pengguna Anda. `/config autoContinueAtUsageLimit=false` juga berfungsi, termasuk dengan `-p`, tetapi bentuk `key=value` tidak dapat mengaktifkannya kembali, karena pengaturan memberikan eksekusi tanpa pengawasan. File pengaturan mana yang Claude Code baca untuk kunci ini ada dalam [referensi pengaturan](/docs/id/settings-reference#autocontinueatusagelimit).

<h2 id="pr-review-status">
  Status review PR
</h2>

Saat bekerja pada cabang dengan permintaan tarik terbuka, Claude Code menampilkan tautan PR yang dapat diklik di footer, seperti "PR #446". Tautan memiliki garis bawah berwarna yang menunjukkan status review:

* Hijau: disetujui
* Kuning: menunggu review
* Merah: perubahan diminta
* Abu-abu: draft

Badge menghilang setelah permintaan tarik digabungkan atau ditutup.

`Cmd+click` (macOS) atau `Ctrl+click` (Windows/Linux) tautan untuk membuka permintaan tarik di browser Anda.

Status menyegarkan segera setelah `git push`, atau perintah `gh pr` yang mengubah permintaan tarik, seperti `gh pr create` atau `gh pr merge`, berhasil dalam sesi.

Claude Code merender badge sebagai hyperlink bahkan ketika tidak dapat mendeteksi dukungan hyperlink di terminal Anda, yang biasanya terjadi melalui SSH atau di tmux. Atur [`FORCE_HYPERLINK=0`](/docs/id/env-vars) untuk merender badge sebagai teks biasa.

Ketika Anda mengatur [`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`](/docs/id/env-vars), Claude Code tidak memeriksa status permintaan tarik atau permintaan penggabungan.

<Note>
  Status PR untuk repositori GitHub memerlukan token GitHub. Claude Code menemukan satu berdasarkan host remote:

  * **github.com**: `GH_TOKEN` atau `GITHUB_TOKEN`, atau token yang disimpan oleh `gh auth login`. Tanpa satu, footer menampilkan `install gh for PR status` ketika CLI `gh` tidak diinstal, atau `gh auth login for PR status` ketika sudah diinstal
  * **Host GitHub Enterprise yang ditetapkan sebagai `GH_HOST`**: `GH_ENTERPRISE_TOKEN` atau `GITHUB_ENTERPRISE_TOKEN`, atau token yang disimpan oleh `gh auth login --hostname <host>`. Tanpa satu, footer menampilkan petunjuk yang sama
  * **Host GitHub lainnya**: token yang disimpan oleh `gh auth login --hostname <host>`. Tanpa satu, Claude Code tidak menampilkan badge dan tidak ada petunjuk
</Note>

<h3 id="gitlab-merge-requests">
  Permintaan penggabungan GitLab
</h3>

Ketika Anda bekerja pada cabang dengan permintaan penggabungan GitLab terbuka, Claude Code menampilkan badge `MR !N` yang dapat diklik di slot footer yang sebaliknya menampung tautan PR GitHub. `!N` adalah sintaks referensi GitLab sendiri untuk nomor permintaan penggabungan N. Garis bawah berwarna menunjukkan status permintaan penggabungan:

* Hijau: GitLab melaporkan permintaan penggabungan sebagai dapat digabungkan
* Kuning: status terbuka lainnya
* Abu-abu: draft

Badge menghilang setelah permintaan penggabungan digabungkan atau ditutup.

Ini menyegarkan segera setelah `git push`, atau perintah `glab mr` yang mengubah permintaan penggabungan, seperti `glab mr create` atau `glab mr merge`, berhasil dalam sesi.

Untuk mendapatkan badge, Anda memerlukan:

* Claude Code v2.1.234 atau lebih baru
* Remote repositori yang menunjuk ke host GitLab Anda, baik gitlab.com atau instans yang dikelola sendiri
* CLI [`glab`](https://gitlab.com/gitlab-org/cli) di `PATH` Anda, diautentikasi dengan `glab auth login`

Claude Code mengabaikan variabel lingkungan token `glab`, seperti `GITLAB_TOKEN`, ketika memeriksa status, jadi Anda tidak mendapatkan badge dari token yang diekspor saja. Claude Code juga mencari `glab` dan loginnya sekali per sesi, jadi restart Claude Code setelah Anda menginstal `glab` atau menjalankan `glab auth login`.

<h2 id="issue-reference-links">
  Tautan referensi masalah
</h2>

Ketika Claude menyebutkan masalah sebagai `owner/repo#123`, Anda dapat mengklik referensi untuk membukanya, selama terminal Anda mendukung hyperlink. Jika Claude Code tidak mendeteksi dukungan hyperlink di terminal Anda, atur [`FORCE_HYPERLINK`](/docs/id/env-vars) ke `1` untuk mengaktifkan tautan, atau ke `0` untuk menjaga referensi sebagai teks biasa.

Anda hanya mendapatkan tautan untuk bentuk dua bagian `owner/repo#123`. Ini tetap menjadi teks biasa:

* Sebuah `#123` yang berdiri sendiri
* Jalur GitLab bersarang seperti `group/subgroup/project#123`
* Referensi apa pun di dalam rentang kode atau blok kode

Claude Code membangun tautan untuk host repositori yang diidentifikasinya dari git remote Anda, bukan untuk repositori yang dirujuk oleh referensi:

| Host repositori Anda                                                                | Tempat `owner/repo#123` menautkan                    |
| :---------------------------------------------------------------------------------- | :--------------------------------------------------- |
| github.com, host GitHub Enterprise, atau host apa pun yang tidak tercantum di bawah | `https://<host>/owner/repo/issues/123`               |
| gitlab.com                                                                          | `https://gitlab.com/owner/repo/-/issues/123`         |
| bitbucket.org, codeberg.org, atau gitea.com                                         | Tidak ada tautan; referensi tetap menjadi teks biasa |

<h2 id="see-also">
  Lihat juga
</h2>

* [Skills](/docs/id/skills) - Prompt dan alur kerja kustom
* [Checkpointing](/docs/id/checkpointing) - Putar ulang pengeditan Claude dan kembalikan status sebelumnya
* [Referensi CLI](/docs/id/cli-reference) - Bendera dan opsi baris perintah
* [Pengaturan](/docs/id/settings) - Opsi konfigurasi
* [Manajemen memori](/docs/id/memory) - Mengelola file CLAUDE.md
