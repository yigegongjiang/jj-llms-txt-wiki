> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Gunakan Claude Code dengan pembaca layar

> Atur Claude Code untuk pembaca layar seperti VoiceOver dan NVDA, plus pengaturan untuk pembesar layar, gerakan berkurang, dan tema ramah buta warna.

Claude Code memiliki mode pembaca layar yang menggantikan antarmuka terminal visualnya dengan teks biasa dan linear. Alih-alih kotak, animasi kemajuan, dan penggambaran ulang di tempat, mode ini mencetak baris berlabel yang dibaca pembaca layar seperti VoiceOver atau NVDA secara berurutan, sehingga Anda dapat melakukan percakapan lengkap, menyetujui izin alat, dan meninjau output dari awal hingga akhir.

Mode pembaca layar bersifat opt-in. Jika Anda menggunakan pembesar layar, gerakan berkurang, atau tema ramah buta warna alih-alih pembaca layar, lihat [Pengaturan aksesibilitas di luar mode pembaca layar](#accessibility-settings-beyond-screen-reader-mode).

<Note>
  Mode pembaca layar memerlukan Claude Code v2.1.181 atau lebih baru. Versi sebelumnya menolak flag `--ax-screen-reader` dengan `error: unknown option '--ax-screen-reader'`.
</Note>

<h2 id="turn-on-screen-reader-mode">
  Aktifkan mode pembaca layar
</h2>

Pilih metode yang sesuai dengan seberapa sering Anda menggunakan pembaca layar:

* Untuk satu sesi: jalankan `claude --ax-screen-reader`.
* Untuk sesi yang dimulai dari satu shell: atur variabel lingkungan `CLAUDE_AX_SCREEN_READER` ke `1`. Di Bash atau Zsh, jalankan `export CLAUDE_AX_SCREEN_READER=1`; di PowerShell, jalankan `$env:CLAUDE_AX_SCREEN_READER = "1"`. Tambahkan baris ke profil shell Anda untuk mencakup setiap shell.
* Untuk setiap sesi di mesin: tambahkan `"axScreenReader": true` ke [file pengaturan](/docs/id/settings) pengguna Anda. Ini mencakup terminal apa pun, termasuk terminal terintegrasi VS Code.

<Note>
  Metode-metode tersebut tercantum dalam urutan prioritas: flag [`--ax-screen-reader`](/docs/id/cli-reference#cli-flags) menggantikan variabel lingkungan [`CLAUDE_AX_SCREEN_READER`](/docs/id/env-vars), yang menggantikan pengaturan [`axScreenReader`](/docs/id/settings#available-settings).
</Note>

Jika Anda menggunakan Claude Code melalui SSH, atur variabel lingkungan atau pengaturan pada mesin jarak jauh tempat Claude Code berjalan.

Ketika mode aktif, hal pertama yang dicetak Claude Code adalah baris konfirmasi yang menyebutkan metode yang mengaktifkannya: `[Screen Reader Mode: on via flag]`, `[Screen Reader Mode: on via env]`, atau `[Screen Reader Mode: on via settings]`. Format penamaan metode memerlukan Claude Code v2.1.206 atau lebih baru. Ketika Claude Code meluncurkan ulang dirinya sendiri, misalnya untuk menyelesaikan pemasangan pembaruan, proses baru mewarisi mode melalui variabel lingkungan `CLAUDE_AX_SCREEN_READER`, sehingga baris konfirmasinya berbunyi `[Screen Reader Mode: on via env]` terlepas dari metode mana yang Anda gunakan.
Versi sebelumnya mencetak `[Accessible screen reader mode: on]`.

<h2 id="turn-off-screen-reader-mode">
  Matikan mode pembaca layar
</h2>

Balikkan metode apa pun yang mengaktifkan mode: mulai tanpa flag, batalkan pengaturan variabel lingkungan, atau atur `axScreenReader` ke `false`. Mengatur `CLAUDE_AX_SCREEN_READER=0` membuat mode tetap mati bahkan ketika pengaturan adalah `true`.

<h2 id="what-your-screen-reader-hears">
  Apa yang didengar pembaca layar Anda
</h2>

Dalam mode pembaca layar, Claude Code menulis teks datar:

* tidak ada karakter penggambar kotak untuk chrome antarmuka
* tidak ada petunjuk warna saja
* tidak ada penggambaran ulang konten yang belum berubah; spinner kemajuan dirender sebagai teks statis
* tabel dalam balasan Claude dibaca sebagai kalimat `Header: value` alih-alih kisi karakter kotak. Memerlukan Claude Code v2.1.198 atau lebih baru; versi sebelumnya menggambar tabel sebagai kisi bahkan dalam mode pembaca layar.

Output terakumulasi dalam scrollback terminal Anda, sehingga Anda dapat membaca kembali giliran sebelumnya dengan perintah tinjauan pembaca layar Anda atau pencarian terminal Anda.

Mode pembaca layar dirender sebagai teks gulir biasa, bahkan jika Anda telah mengaktifkan [rendering layar penuh](/docs/id/fullscreen) dengan [pengaturan `tui`](/docs/id/settings#available-settings); pengaturan tidak berpengaruh saat mode aktif. Sesi latar belakang yang terlampir masih dirender layar penuh; lihat [Batasan yang diketahui](#known-limitations).

Setiap pesan dalam transkrip dimulai dengan label yang diumumkan pembaca layar Anda, menyebutkan apa itu: pesan Anda, balasan Claude, aktivitas alat, kesalahan, dan prompt. Label juga dapat dicari, sehingga Anda dapat melompat antar bagian transkrip dengan mencari scrollback terminal Anda:

| Label                  | Arti                                                                                          |
| :--------------------- | :-------------------------------------------------------------------------------------------- |
| `you:`                 | Pesan Anda                                                                                    |
| `claude:`              | Balasan Claude                                                                                |
| `tool:`                | Aktivitas alat, seperti pengeditan file atau perintah yang dijalankan                         |
| `tool error:`          | Alat yang gagal                                                                               |
| `error:`               | Kesalahan dalam percakapan, seperti permintaan API yang gagal                                 |
| `Permission Required:` | Prompt izin menunggu jawaban Anda                                                             |
| `Cost:`                | Ringkasan biaya sesi ketika Claude Code keluar, jika akun Anda [menampilkan biaya](/docs/id/costs) |

Kursor terminal mengikuti tanda sisip input, sehingga perintah baca-baris-saat-ini pembaca layar Anda menjawab "di mana saya" dengan prompt yang Anda edit.

<h3 id="jump-between-turns">
  Lompat antar giliran
</h3>

Claude Code memancarkan penanda integrasi shell OSC 133 di batas giliran, sehingga kunci lompat-ke-prompt-sebelumnya terminal Anda bergerak antar giliran tanpa membaca seluruh transkrip:

* iTerm2: Cmd+Shift+Up
* Terminal VS Code: Ctrl+Up di Windows, Cmd+Up di macOS
* Windows Terminal: tidak ada kunci secara default; ikat tindakan `scrollToMark` dalam pengaturannya
* Kitty dan Ghostty: periksa dokumentasi terminal untuk kunci lompat-ke-prompt-nya

macOS Terminal tidak bertindak atas penanda, dan Claude Code tidak memancarkannya di WezTerm. Di terminal tersebut, cari scrollback untuk label `you:` sebagai gantinya.

<h2 id="answer-menus-and-prompts">
  Jawab menu dan prompt
</h2>

Dalam mode pembaca layar, menu yang biasanya Anda navigasikan dengan tombol panah, termasuk prompt izin, menjadi daftar bernomor. Setiap opsi diumumkan sebagai baris bernomor, diikuti oleh prompt `Enter selection` yang menyebutkan rentang yang valid. Ketik nomor opsi yang Anda inginkan dan tekan Enter.

* Untuk membatalkan menu yang dapat ditutup: tekan Escape. Promptnya berakhir dengan `or Escape to cancel`.
* Jika Anda mengetik nomor yang tidak ada dalam daftar: Claude Code mengumumkan rentang yang valid dan membiarkan Anda mencoba lagi.

Prompt ya-atau-tidak meminta jawaban yang diketik alih-alih menu dua opsi. Jawab `y` atau `n` dan tekan Enter. `yes` dan `no` juga berfungsi.

<h2 id="hear-when-claude-code-needs-you">
  Dengarkan ketika Claude Code membutuhkan Anda
</h2>

Dalam mode pembaca layar, Claude Code membunyikan bel terminal ketika membutuhkan perhatian Anda, sehingga Anda tidak perlu terus memeriksa transkrip. Bel berbunyi ketika:

* Claude menyelesaikan balasan
* prompt izin muncul
* alat yang berjalan lebih lama dari 5 detik selesai

Bel adalah peringatan standar terminal Anda. Untuk membungkamnya, ubah pengaturan bel di aplikasi terminal Anda. Bel tidak memerlukan mode pembaca layar: di luar mode, atur [`preferredNotifChannel`](/docs/id/settings#available-settings) ke `"terminal_bell"` untuk peringatan serupa ketika Claude menunggu Anda. Lihat [Dapatkan bel terminal atau notifikasi](/docs/id/terminal-config#get-a-terminal-bell-or-notification).

<h2 id="accessibility-settings-beyond-screen-reader-mode">
  Pengaturan aksesibilitas di luar mode pembaca layar
</h2>

Opsi-opsi ini mengatasi kebutuhan aksesibilitas di luar mode pembaca layar. Semuanya bekerja bersama dengannya.

* Variabel lingkungan [`CLAUDE_CODE_ACCESSIBILITY`](/docs/id/env-vars) adalah untuk pembesar layar. Atur `CLAUDE_CODE_ACCESSIBILITY=1` untuk menjaga kursor terminal asli tetap terlihat sehingga pembesar, seperti macOS Zoom, dapat melacak posisi kursor.
* Pengaturan [`prefersReducedMotion`](/docs/id/settings#available-settings) mengurangi atau menonaktifkan spinner, shimmer, dan animasi lainnya tanpa mengubah sisa antarmuka.
* Pengaturan [`theme`](/docs/id/settings#available-settings) memilih warna antarmuka, termasuk tema ramah buta warna `dark-daltonized` dan `light-daltonized`.

<h2 id="known-limitations">
  Batasan yang diketahui
</h2>

Beberapa perilaku tidak disesuaikan untuk mode pembaca layar:

* Mode pembaca layar tidak aktif secara otomatis ketika pembaca layar sedang berjalan.
* Perubahan mode, seperti memasuki [plan mode](/docs/id/permission-modes#analyze-before-you-edit-with-plan-mode), belum diumumkan.
* Melampirkan ke [sesi latar belakang](/docs/id/agent-view) dengan `claude attach` atau dari tampilan agen memasuki layar alternatif terminal, yang tidak memiliki scrollback asli. Ini adalah [perilaku yang sama seperti sesi terlampir lainnya](/docs/id/fullscreen). Untuk keluar, tekan Left Arrow pada prompt kosong, atau Ctrl+Z jika dialog memiliki fokus.
* Claude Code mengumumkan biaya dalam ringkasan yang dicetak saat keluar, bukan per giliran.
* Mode pembaca layar tidak mengubah [mode non-interaktif](/docs/id/headless) dengan flag `-p`. Mode non-interaktif sudah menulis teks biasa dan tetap menjadi alternatif untuk scripting.

<h2 id="report-an-issue">
  Laporkan masalah
</h2>

Jika sesuatu tidak berfungsi dengan pembaca layar, pembesar, atau terminal Anda, buka masalah di [pelacak masalah Claude Code](https://github.com/anthropics/claude-code/issues) dan sebutkan teknologi bantu Anda dalam judul. Sertakan sistem operasi, aplikasi terminal, dan nama serta versi teknologi bantu Anda dalam laporan.

<h2 id="related-resources">
  Sumber daya terkait
</h2>

Halaman-halaman ini menyimpan entri referensi lengkap dan pengaturan terkait untuk apa yang halaman ini bahas:

* [Settings](/docs/id/settings#available-settings): entri `axScreenReader`, `prefersReducedMotion`, `theme`, dan `preferredNotifChannel`
* [Environment variables](/docs/id/env-vars): entri `CLAUDE_AX_SCREEN_READER` dan `CLAUDE_CODE_ACCESSIBILITY`
* [CLI reference](/docs/id/cli-reference#cli-flags): flag `--ax-screen-reader`
* [Terminal configuration](/docs/id/terminal-config): bel, notifikasi, dan tema di luar mode pembaca layar
* [Non-interactive mode](/docs/id/headless): jalankan `claude -p` yang ditulis skrip, yang menulis teks biasa tanpa mode pembaca layar
