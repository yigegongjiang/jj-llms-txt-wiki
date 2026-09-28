> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Gunakan Claude Code dengan pembaca layar

> Atur Claude Code untuk pembaca layar seperti VoiceOver dan NVDA, plus pengaturan untuk pembesar layar, gerakan berkurang, dan tema ramah buta warna.

Claude Code memiliki mode pembaca layar yang menggantikan antarmuka terminal visualnya dengan teks biasa dan linear. Alih-alih kotak, animasi kemajuan, dan penggambaran ulang di tempat, Claude Code mencetak baris berlabel yang dibaca pembaca layar seperti VoiceOver atau NVDA secara berurutan. Anda dapat melakukan percakapan lengkap, menyetujui izin alat, dan meninjau output dari awal hingga akhir.

Mode pembaca layar bersifat opt-in. Jika Anda menggunakan pembesar layar, gerakan berkurang, atau tema ramah buta warna alih-alih pembaca layar, atur `CLAUDE_CODE_ACCESSIBILITY`, `prefersReducedMotion`, atau `theme` dari tabel [Pengaturan aksesibilitas](#accessibility-settings). Mode pembaca layar hanya menyesuaikan antarmuka terminal, jadi Anda tidak membutuhkannya di panel chat ekstensi VS Code. Pada Claude Code v2.1.236 atau lebih baru, ekstensi [mengumumkan aktivitas percakapan ke pembaca layar Anda](/docs/id/vs-code#use-a-screen-reader) di sana tanpa pengaturan apa pun.

<h2 id="turn-on-screen-reader-mode">
  Aktifkan mode pembaca layar
</h2>

Pilih metode yang sesuai dengan seberapa sering Anda menggunakan pembaca layar:

* Untuk satu sesi: jalankan `claude --ax-screen-reader`.
* Untuk sesi yang dimulai dari satu shell: atur variabel lingkungan `CLAUDE_AX_SCREEN_READER` ke `1`. Di Bash atau Zsh, jalankan `export CLAUDE_AX_SCREEN_READER=1`. Di PowerShell, jalankan `$env:CLAUDE_AX_SCREEN_READER = "1"`. Tambahkan baris tersebut ke profil shell Anda untuk mempertahankannya untuk shell di masa depan.
* Untuk setiap sesi di mesin: tambahkan `"axScreenReader": true` ke [file pengaturan](/docs/id/settings) pengguna Anda. Pengaturan ini berlaku di terminal apa pun, termasuk terminal terintegrasi VS Code.

Jika Anda menggabungkan metode, Claude Code menerapkan flag [`--ax-screen-reader`](/docs/id/cli-reference#cli-flags) di atas variabel lingkungan [`CLAUDE_AX_SCREEN_READER`](/docs/id/env-vars#variables), dan variabel di atas pengaturan [`axScreenReader`](/docs/id/settings-reference#axscreenreader).

Jika Anda menggunakan Claude Code melalui SSH, atur variabel lingkungan atau pengaturan pada mesin jarak jauh tempat Claude Code berjalan.

Baris pertama yang dicetak Claude Code mengonfirmasi mode: `[Screen Reader Mode: on via flag]`, `[Screen Reader Mode: on via env]`, atau `[Screen Reader Mode: on via settings]`.

<h2 id="turn-off-screen-reader-mode">
  Matikan mode pembaca layar
</h2>

Balikkan metode apa pun yang mengaktifkan mode: mulai tanpa flag, batalkan pengaturan variabel lingkungan, atau atur `axScreenReader` ke `false`. Jika Anda mengatur `CLAUDE_AX_SCREEN_READER` ke `0`, Claude Code membuat mode tetap mati bahkan ketika pengaturan adalah `true`.

<h2 id="accessibility-settings">
  Pengaturan aksesibilitas
</h2>

Tabel mencantumkan setiap opsi aksesibilitas, apakah Anda menetapkannya sebagai flag, variabel lingkungan, atau pengaturan, dan apa yang diubahnya.

| Opsi                                                                    | Tipe                | Apa yang diubahnya                                                                                                                                                                                                                                                        |
| :---------------------------------------------------------------------- | :------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| [`--ax-screen-reader`](/docs/id/cli-reference#cli-flags)                     | Flag                | Mode pembaca layar untuk satu sesi.                                                                                                                                                                                                                                       |
| [`CLAUDE_AX_SCREEN_READER`](/docs/id/env-vars#variables)                     | Variabel lingkungan | Mode pembaca layar untuk sesi yang dimulai dari shell tempat Anda menetapkannya.                                                                                                                                                                                          |
| [`axScreenReader`](/docs/id/settings-reference#axscreenreader)               | Pengaturan          | Mode pembaca layar untuk setiap sesi ketika `true`.                                                                                                                                                                                                                       |
| [`CLAUDE_AX_STARTUP_QUIET_MS`](/docs/id/env-vars#variables)                  | Variabel lingkungan | Berapa lama Claude Code menunggu setelah baris konfirmasi sebelum menggambar prompt pertama dalam mode pembaca layar. Memerlukan Claude Code v2.1.217 atau lebih baru.                                                                                                    |
| [`CLAUDE_AX_PREPARK_MS`](/docs/id/env-vars#variables)                        | Variabel lingkungan | Berapa lama Claude Code menunggu, dengan kursor di awal baris, sebelum menulis baris baru atau berubah dalam mode pembaca layar. Memerlukan Claude Code v2.1.233 atau lebih baru.                                                                                         |
| [`CLAUDE_CODE_ACCESSIBILITY`](/docs/id/env-vars#variables)                   | Variabel lingkungan | Kursor terminal yang tetap terlihat untuk pembesar layar seperti macOS Zoom ketika Anda menetapkannya ke `1`. Kursor mengikuti tanda sisip input dan, pada Claude Code v2.1.218 atau lebih baru, baris yang disorot dalam menu dan panel seperti `/config` dan `/plugin`. |
| [`prefersReducedMotion`](/docs/id/settings-reference#prefersreducedmotion)   | Pengaturan          | Spinner, shimmer, dan animasi lainnya berkurang atau tidak ada ketika `true`.                                                                                                                                                                                             |
| [`theme`](/docs/id/settings-reference#theme)                                 | Pengaturan          | Warna antarmuka, termasuk tema ramah buta warna `dark-daltonized` dan `light-daltonized`. Anda juga dapat memilih salah satu dengan [`/theme`](/docs/id/commands#all-commands).                                                                                                |
| [`preferredNotifChannel`](/docs/id/settings-reference#preferrednotifchannel) | Pengaturan          | Dengan nilai `"terminal_bell"`, bel terminal di luar mode pembaca layar ketika Claude menunggu Anda.                                                                                                                                                                      |

<h2 id="what-your-screen-reader-hears">
  Apa yang didengar pembaca layar Anda
</h2>

Dalam mode pembaca layar, Claude Code menulis teks datar:

* Tidak ada karakter penggambar kotak untuk chrome antarmuka
* Tidak ada petunjuk berbasis warna saja
* Tidak ada penggambaran ulang konten yang belum berubah. Spinner kemajuan ditampilkan sebagai teks statis
* Tabel dalam balasan Claude dibaca sebagai kalimat `Header: value` alih-alih kisi karakter kotak

Claude Code meninggalkan semua yang dicetak di scrollback terminal Anda, sehingga Anda dapat membaca kembali giliran sebelumnya dengan perintah tinjauan pembaca layar Anda atau pencarian terminal Anda. Claude Code mengabaikan pengaturan [`tui`](/docs/id/settings-reference#tui) dalam mode pembaca layar. Terlepas dari sesi latar belakang terlampir yang tercantum di bawah [Batasan yang diketahui](#known-limitations), ia mencetak teks bergulir alih-alih [rendering layar penuh](/docs/id/fullscreen).

Claude Code juga menunggu di dua titik sehingga pembaca layar Anda dapat mengikuti:

* Setelah Claude Code mencetak baris konfirmasi, ia menunggu 3 detik sebelum menggambar prompt, sehingga pembaca layar Anda dapat menyelesaikan baris. Tekan tombol apa pun untuk mengakhiri penantian. Untuk mengubah panjang penantian, atur [`CLAUDE_AX_STARTUP_QUIET_MS`](/docs/id/env-vars#variables).
* Sebelum Claude Code menulis baris baru atau berubah, seperti petunjuk atau lebih banyak balasan Claude, ia memindahkan kursor ke awal baris dan menunggu 50 milidetik. Pembaca layar Anda kemudian membaca baris dari karakter pertamanya. Karakter yang Anda ketik atau hapus di akhir baris input muncul segera. Untuk mengubah panjang penantian, atur [`CLAUDE_AX_PREPARK_MS`](/docs/id/env-vars#variables).

Setiap pesan dalam transkrip dimulai dengan label yang diumumkan pembaca layar Anda, menamai apa itu: pesan Anda, balasan dan pemikiran Claude, aktivitas alat, kesalahan dan peringatan, serta prompt. Label juga dapat dicari, sehingga Anda dapat melompat antar bagian transkrip dengan mencari scrollback terminal Anda:

| Label                  | Arti                                                                                          |
| :--------------------- | :-------------------------------------------------------------------------------------------- |
| `you:`                 | Pesan Anda                                                                                    |
| `claude:`              | Balasan Claude                                                                                |
| `thinking:`            | Pemikiran Claude                                                                              |
| `tool:`                | Aktivitas alat, seperti pengeditan file atau perintah yang dijalankan                         |
| `tool error:`          | Alat yang gagal                                                                               |
| `error:`               | Kesalahan dalam percakapan, seperti permintaan API yang gagal                                 |
| `warning:`             | Peringatan dari Claude Code, seperti beralih ke model fallback                                |
| `Permission Required:` | Prompt izin menunggu jawaban Anda                                                             |
| `Cost:`                | Ringkasan biaya sesi ketika Claude Code keluar, jika akun Anda [menampilkan biaya](/docs/id/costs) |

Claude Code menjaga kursor terminal pada tanda sisip input, sehingga perintah baca-baris-saat-ini pembaca layar Anda membaca prompt yang Anda edit.

Saat Anda mengetik di akhir baris input, atau tekan `Backspace` di sana, Claude Code hanya menulis karakter yang berubah. Pembaca layar Anda mengulangi hanya karakter tersebut.

Ketika Anda menghapus kata atau baris dengan salah satu [pintasan pengeditan teks](/docs/id/interactive-mode#text-editing), Claude Code mengumumkan teks yang dihapus:

* Menghapus kata dengan `Ctrl+W` atau `Alt+D`, atau dengan `Option+Delete` di macOS atau `Ctrl+Backspace` di Windows
* Menghapus ke awal baris dengan `Ctrl+U` atau `Cmd+Backspace`
* Menghapus ke akhir baris dengan `Ctrl+K`

Ketika Anda mengubah [mode izin](/docs/id/permission-modes) dengan `Shift+Tab`, Claude Code mengumumkan mode izin yang Anda mendarat, seperti `[plan mode on]` atau `[accept edits on]`. Claude Code mencetak pengumuman sekali dan tidak mengulanginya pada penggambaran ulang nanti.

<h3 id="jump-between-turns">
  Melompat antar giliran
</h3>

Claude Code memancarkan penanda integrasi shell OSC 133 di batas giliran, sehingga tombol lompat-ke-prompt-sebelumnya terminal Anda bergerak antar giliran tanpa membaca seluruh transkrip:

* iTerm2: Cmd+Shift+Up
* Terminal VS Code: Ctrl+Up di Windows, Cmd+Up di macOS
* Windows Terminal: tidak ada kunci secara default; ikat tindakan `scrollToMark` dalam pengaturannya
* Kitty dan Ghostty: periksa dokumentasi terminal untuk tombol lompat-ke-prompt-nya

macOS Terminal tidak bertindak atas penanda, dan Claude Code tidak memancarkannya di WezTerm. Di terminal tersebut, cari scrollback untuk label `you:` sebagai gantinya.

<h2 id="answer-menus-and-prompts">
  Menjawab menu dan prompt
</h2>

Dalam mode pembaca layar, menu yang biasanya Anda navigasikan dengan tombol panah, termasuk prompt izin, menjadi daftar bernomor. Claude Code mengumumkan setiap opsi sebagai baris bernomor, kemudian prompt `Enter selection` yang menyebutkan rentang yang valid. Ketik nomor opsi yang Anda inginkan dan tekan Enter.

* Tekan Escape untuk membatalkan menu yang promptnya diakhiri dengan `or Escape to cancel`.
* Jika Anda mengetik nomor yang tidak ada dalam daftar, Claude Code mengumumkan rentang yang valid dan membiarkan Anda mencoba lagi.

Pemilih [`/effort`](/docs/id/model-config#adjust-effort-level), yang merupakan slider di luar mode pembaca layar, menjadi jenis daftar bernomor yang sama.

Prompt ya-atau-tidak meminta jawaban yang diketik alih-alih menu dua opsi. Jawab `y` atau `n` dan tekan Enter. `yes` dan `no` juga berfungsi.

<h2 id="hear-when-claude-code-needs-you">
  Dengarkan ketika Claude Code membutuhkan Anda
</h2>

Dalam mode pembaca layar, Claude Code membunyikan bel terminal ketika membutuhkan perhatian Anda, sehingga Anda tidak perlu terus memeriksa transkrip. Bel berbunyi ketika:

* Claude menyelesaikan balasan
* Prompt atau dialog membutuhkan jawaban Anda, seperti prompt izin
* Alat yang berjalan lebih lama dari 5 detik selesai

Bel adalah peringatan standar terminal Anda. Untuk membisukan, ubah pengaturan bel di aplikasi terminal Anda. Di luar mode pembaca layar, atur [`preferredNotifChannel`](/docs/id/settings-reference#preferrednotifchannel) ke `"terminal_bell"` untuk mendapatkan [bel serupa](/docs/id/terminal-config#get-a-terminal-bell-or-notification) ketika Claude menunggu Anda.

<h2 id="known-limitations">
  Batasan yang diketahui
</h2>

Beberapa perilaku tidak disesuaikan untuk mode pembaca layar:

* Mode pembaca layar tidak aktif secara otomatis ketika pembaca layar sedang berjalan.
* Claude Code tidak mengumumkan perubahan mode izin yang dilakukan dengan cara lain selain bersiklus dengan `Shift+Tab`, seperti memasuki [plan mode](/docs/id/permission-modes#analyze-before-you-edit-with-plan-mode) dari perintah.
* Melampirkan ke [sesi latar belakang](/docs/id/agent-view) dengan `claude attach` atau dari tampilan agen memasuki layar alternatif terminal, yang tidak memiliki scrollback asli. Ini adalah [perilaku yang sama seperti sesi terlampir lainnya](/docs/id/fullscreen). Untuk keluar, tekan Left Arrow pada prompt kosong, atau Ctrl+Z jika dialog memiliki fokus.
* Claude Code mengumumkan biaya dalam ringkasan yang dicetak saat keluar, bukan per giliran.
* Mode pembaca layar tidak mengubah [mode non-interaktif](/docs/id/headless) dengan flag `-p`. Mode non-interaktif sudah menulis teks biasa dan tetap menjadi alternatif untuk scripting.

<h2 id="report-an-issue">
  Laporkan masalah
</h2>

Jika sesuatu tidak berfungsi dengan pembaca layar, pembesar, atau terminal Anda, buka masalah di [pelacak masalah Claude Code](https://github.com/anthropics/claude-code/issues) dan sebutkan teknologi bantu Anda dalam judul. Sertakan sistem operasi, aplikasi terminal, dan nama serta versi teknologi bantu Anda dalam laporan.
