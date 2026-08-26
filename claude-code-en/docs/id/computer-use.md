> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Biarkan Claude menggunakan komputer Anda dari CLI

> Aktifkan computer use di Claude Code CLI sehingga Claude dapat membuka aplikasi, mengklik, mengetik, dan melihat layar Anda di macOS. Uji aplikasi native, debug masalah visual, dan otomatisasi alat GUI-only tanpa meninggalkan terminal Anda.

<Note>
  Computer use adalah research preview di macOS yang memerlukan paket Pro atau Max. Tidak tersedia di paket Team atau Enterprise. Memerlukan sesi interaktif, jadi tidak tersedia dalam mode non-interaktif dengan flag `-p`.
</Note>

Computer use memungkinkan Claude membuka aplikasi, mengontrol layar Anda, dan bekerja di mesin Anda seperti yang Anda lakukan. Dari CLI, Claude dapat mengompilasi aplikasi Swift, meluncurkannya, mengklik setiap tombol, dan mengambil screenshot hasilnya, semuanya dalam percakapan yang sama di mana Claude menulis kodenya.

Halaman ini mencakup cara kerja computer use di CLI. Untuk aplikasi Desktop di macOS atau Windows, lihat [computer use di Desktop](/docs/id/desktop#let-claude-use-your-computer).

<h2 id="what-you-can-do-with-computer-use">
  Apa yang dapat Anda lakukan dengan computer use
</h2>

Computer use menangani tugas yang memerlukan GUI: apa pun yang biasanya harus Anda tinggalkan terminal dan lakukan dengan tangan.

* **Bangun dan validasi aplikasi native**: minta Claude untuk membangun aplikasi menu bar macOS. Claude menulis Swift, mengompilasinya, meluncurkannya, dan mengklik setiap kontrol untuk memverifikasi bahwa itu berfungsi sebelum Anda pernah membukanya.
* **Pengujian UI end-to-end**: arahkan Claude ke aplikasi Electron lokal dan katakan "uji alur onboarding." Claude membuka aplikasi, mengklik melalui pendaftaran, dan mengambil screenshot setiap langkah. Tidak ada konfigurasi Playwright, tidak ada test harness.
* **Debug masalah visual dan tata letak**: beri tahu Claude "modal terpotong di jendela kecil." Claude mengubah ukuran jendela, mereproduksi bug, mengambil screenshot, menambal CSS, dan memverifikasi perbaikan. Claude melihat apa yang Anda lihat.
* **Jalankan alat GUI-only**: berinteraksi dengan alat desain, panel kontrol hardware, iOS Simulator, atau aplikasi proprietary yang tidak memiliki CLI atau API.

<h2 id="when-computer-use-applies">
  Kapan computer use berlaku
</h2>

Claude memiliki beberapa cara untuk berinteraksi dengan aplikasi atau layanan. Computer use adalah yang paling luas dan paling lambat, jadi Claude mencoba alat yang paling presisi terlebih dahulu:

* Jika Anda memiliki [MCP server](/docs/id/mcp) untuk layanan tersebut, Claude menggunakannya.
* Jika tugasnya adalah perintah shell, Claude menggunakan Bash.
* Jika tugasnya adalah pekerjaan browser dan Anda memiliki [Claude di Chrome](/docs/id/chrome) yang diatur, Claude menggunakannya.
* Jika tidak ada yang berlaku, Claude menggunakan computer use.

Kontrol layar dicadangkan untuk hal-hal yang tidak dapat dijangkau oleh yang lain: aplikasi native, simulator, dan alat tanpa API.

<h2 id="enable-computer-use">
  Aktifkan computer use
</h2>

Computer use tersedia sebagai MCP server bawaan yang disebut `computer-use`. Ini dimatikan secara default sampai Anda mengaktifkannya.

<Steps>
  <Step title="Buka menu MCP">
    Dalam sesi Claude Code interaktif, jalankan:

    ```text theme={null}
    /mcp
    ```

    Temukan `computer-use` dalam daftar server. Ini ditampilkan sebagai disabled.
  </Step>

  <Step title="Aktifkan server">
    Pilih `computer-use` dan pilih **Enable**. Pengaturan bertahan per proyek, jadi Anda hanya melakukan ini sekali untuk setiap proyek di mana Anda ingin computer use.
  </Step>

  <Step title="Berikan izin macOS">
    Pertama kali Claude mencoba menggunakan komputer Anda, Anda akan melihat prompt untuk memberikan dua izin macOS:

    * **Accessibility**: memungkinkan Claude mengklik, mengetik, dan menggulir
    * **Screen Recording**: memungkinkan Claude melihat apa yang ada di layar Anda

    Prompt mencakup tautan untuk membuka pane System Settings yang relevan. Berikan keduanya, kemudian pilih **Try again** dalam prompt. macOS mungkin memerlukan Anda untuk memulai ulang Claude Code setelah memberikan Screen Recording.
  </Step>
</Steps>

Setelah setup, minta Claude untuk melakukan sesuatu yang memerlukan GUI:

```text theme={null}
Build the app target, launch it, and click through each tab to make
sure nothing crashes. Screenshot any error states you find.
```

<h2 id="approve-apps-per-session">
  Setujui aplikasi per sesi
</h2>

Mengaktifkan server `computer-use` tidak memberikan Claude akses ke setiap aplikasi di mesin Anda. Pertama kali Claude memerlukan aplikasi tertentu dalam sesi, prompt muncul di terminal Anda menampilkan:

* Aplikasi mana yang ingin Claude kontrol
* Izin tambahan apa pun yang diminta, seperti akses clipboard
* Berapa banyak aplikasi lain yang akan disembunyikan saat Claude bekerja

Pilih **Allow for this session** atau **Deny**. Persetujuan berlaku untuk sesi saat ini. Anda dapat menyetujui beberapa aplikasi sekaligus ketika Claude memintanya bersama-sama.

Aplikasi dengan jangkauan luas menampilkan peringatan tambahan dalam prompt sehingga Anda tahu apa yang disetujui mereka:

| Peringatan                              | Berlaku untuk                                                  |
| :-------------------------------------- | :------------------------------------------------------------- |
| Setara dengan akses shell               | Terminal, iTerm, VS Code, Warp, dan terminal serta IDE lainnya |
| Dapat membaca atau menulis file apa pun | Finder                                                         |
| Dapat mengubah pengaturan sistem        | System Settings                                                |

Aplikasi ini tidak diblokir. Peringatan memungkinkan Anda memutuskan apakah tugas tersebut memerlukan tingkat akses itu.

Tingkat kontrol Claude juga bervariasi menurut kategori aplikasi: browser dan platform perdagangan adalah view-only, terminal dan IDE adalah click-only, dan semuanya mendapatkan kontrol penuh. Lihat [app permissions di Desktop](/docs/id/desktop#app-permissions) untuk rincian tier lengkap.

<h2 id="how-claude-works-on-your-screen">
  Bagaimana Claude bekerja di layar Anda
</h2>

Memahami alurnya membantu Anda mengantisipasi apa yang akan Claude lakukan dan cara untuk campur tangan.

<h3 id="one-session-at-a-time">
  Satu sesi pada satu waktu
</h3>

Computer use menahan kunci machine-wide dari tindakan computer use pertama hingga sesi yang mengambilnya keluar. Mulai dari v2.1.195, menyelesaikan tugas tidak melepaskan kunci; hanya keluar dari sesi yang melakukannya. Jika sesi Claude Code lain sudah menggunakan komputer Anda, upaya baru gagal dengan pesan yang memberi tahu Anda sesi mana yang menahan kunci. Keluar dari sesi itu terlebih dahulu.

<h3 id="apps-are-hidden-while-claude-works">
  Aplikasi disembunyikan saat Claude bekerja
</h3>

Ketika Claude mulai mengontrol layar Anda, aplikasi terlihat lainnya disembunyikan sehingga Claude berinteraksi hanya dengan aplikasi yang disetujui. Jendela terminal Anda tetap terlihat dan dikecualikan dari screenshot, sehingga Anda dapat menonton sesi dan Claude tidak pernah melihat output-nya sendiri.

Ketika Claude menyelesaikan giliran, aplikasi yang disembunyikan dipulihkan secara otomatis.

<h3 id="screenshots-are-downscaled-automatically">
  Screenshot secara otomatis diperkecil
</h3>

Claude Code memperkecil setiap screenshot sebelum mengirimnya ke model. Anda tidak perlu menurunkan resolusi tampilan atau mengubah ukuran jendela pada Retina atau tampilan resolusi tinggi lainnya. MacBook Pro 16-inci pada resolusi Retina asli menangkap pada 3456×2234 dan memperkecil menjadi sekitar 1372×887, mempertahankan rasio aspek.

Tidak ada pengaturan untuk mengubah ukuran target. Jika teks atau kontrol on-screen terlalu kecil untuk Claude baca setelah diperkecil, tingkatkan ukurannya di aplikasi daripada mengubah resolusi tampilan Anda.

<h3 id="stop-at-any-time">
  Hentikan kapan saja
</h3>

Ketika Claude memperoleh kunci, notifikasi macOS muncul: "Claude is using your computer · press Esc to stop." Tekan `Esc` di mana saja untuk membatalkan tindakan saat ini segera, atau tekan `Ctrl+C` di terminal. Bagaimanapun, Claude berhenti, menampilkan kembali aplikasi Anda, dan mengembalikan kontrol kepada Anda. Sesi mempertahankan [kunci computer use](#one-session-at-a-time) hingga keluar.

Notifikasi kedua muncul ketika Claude selesai.

<h2 id="safety-and-the-trust-boundary">
  Keamanan dan batas kepercayaan
</h2>

<Warning>
  Tidak seperti [alat Bash yang di-sandbox](/docs/id/sandboxing), computer use berjalan di desktop aktual Anda dengan akses ke aplikasi yang Anda setujui. Claude memeriksa setiap tindakan dan menandai potensi prompt injection dari konten on-screen, tetapi batas kepercayaan berbeda. Lihat [panduan keamanan computer use](https://support.claude.com/en/articles/14128542) untuk praktik terbaik.
</Warning>

Guardrail bawaan mengurangi risiko tanpa memerlukan konfigurasi:

* **Persetujuan per-aplikasi**: Claude hanya dapat mengontrol aplikasi yang telah Anda setujui dalam sesi saat ini.
* **Peringatan sentinel**: aplikasi yang memberikan akses shell, filesystem, atau pengaturan sistem ditandai sebelum Anda menyetujuinya.
* **Terminal dikecualikan dari screenshot**: Claude tidak pernah melihat jendela terminal Anda, jadi prompt on-screen dalam sesi Anda tidak dapat umpan balik ke model.
* **Escape global**: tombol `Esc` membatalkan computer use dari mana saja, dan penekanan tombol dikonsumsi sehingga prompt injection tidak dapat menggunakannya untuk menutup dialog.
* **File kunci**: hanya satu sesi yang dapat mengontrol mesin Anda pada satu waktu.

<h2 id="example-workflows">
  Contoh alur kerja
</h2>

Contoh-contoh ini menunjukkan cara umum untuk menggabungkan computer use dengan tugas coding.

<h3 id="validate-a-native-build">
  Validasi build native
</h3>

Setelah membuat perubahan pada aplikasi macOS atau iOS, minta Claude untuk mengompilasi dan memverifikasi dalam satu lintasan:

```text theme={null}
Build the MenuBarStats target, launch it, open the preferences window,
and verify the interval slider updates the label. Screenshot the
preferences window when you're done.
```

Claude menjalankan `xcodebuild`, meluncurkan aplikasi, berinteraksi dengan UI, dan melaporkan apa yang ditemukannya.

<h3 id="reproduce-a-layout-bug">
  Reproduksi bug tata letak
</h3>

Ketika bug visual hanya muncul pada ukuran jendela tertentu, biarkan Claude menemukannya:

```text theme={null}
The settings modal clips its footer on narrow windows. Resize the app
window down until you can reproduce it, screenshot the clipped state,
then check the CSS for the modal container.
```

Claude mengubah ukuran jendela, menangkap status yang rusak, dan membaca stylesheet yang relevan.

<h3 id="test-a-simulator-flow">
  Uji alur simulator
</h3>

Jalankan iOS Simulator tanpa menulis XCTest:

```text theme={null}
Open the iOS Simulator, launch the app, tap through the onboarding
screens, and tell me if any screen takes more than a second to load.
```

Claude mengontrol simulator dengan cara yang sama seperti Anda dengan mouse.

<h2 id="differences-from-the-desktop-app">
  Perbedaan dari aplikasi Desktop
</h2>

Permukaan CLI dan Desktop berbagi mesin computer use yang sama, dengan beberapa perbedaan:

| Fitur                        | Desktop                                                     | CLI                               |
| :--------------------------- | :---------------------------------------------------------- | :-------------------------------- |
| Platform                     | macOS dan Windows                                           | macOS saja                        |
| Aktifkan                     | Toggle di **Settings > General** (di bawah **Desktop app**) | Aktifkan `computer-use` di `/mcp` |
| Daftar aplikasi yang ditolak | Dapat dikonfigurasi di Settings                             | Belum tersedia                    |
| Toggle auto-unhide           | Opsional                                                    | Selalu aktif                      |
| Integrasi Dispatch           | Sesi yang dispawn Dispatch dapat menggunakan computer use   | Tidak berlaku                     |

<h2 id="troubleshooting">
  Troubleshooting
</h2>

<h3 id="computer-use-is-in-use-by-another-claude-session">
  "Computer use is in use by another Claude session"
</h3>

Sesi Claude Code lain menahan kunci, yang disimpannya sampai keluar. Keluar dari sesi itu. Jika sesi lain mogok, kunci dilepaskan secara otomatis ketika Claude mendeteksi proses tidak lagi berjalan.

<h3 id="macos-permissions-prompt-keeps-reappearing">
  macOS permissions prompt keeps reappearing
</h3>

macOS kadang-kadang memerlukan restart dari proses yang meminta setelah Anda memberikan Screen Recording. Keluar dari Claude Code sepenuhnya dan mulai sesi baru. Jika prompt terus berlanjut, buka **System Settings > Privacy & Security > Screen Recording** dan konfirmasi aplikasi terminal Anda terdaftar dan diaktifkan.

<h3 id="computer-use-doesn’t-appear-in-/mcp">
  `computer-use` doesn't appear in `/mcp`
</h3>

Server hanya muncul pada setup yang memenuhi syarat. Periksa bahwa:

* Anda berada di macOS. Computer use di CLI tidak tersedia di Linux atau Windows. Di Windows, gunakan [computer use di Desktop](/docs/id/desktop#let-claude-use-your-computer) sebagai gantinya.
* Anda berada di paket Pro atau Max. Jalankan `/status` untuk mengonfirmasi langganan Anda.
* Anda diautentikasi melalui claude.ai. Computer use tidak tersedia dengan penyedia pihak ketiga seperti Amazon Bedrock, Google Cloud's Agent Platform, atau Microsoft Foundry. Jika Anda mengakses Claude secara eksklusif melalui penyedia pihak ketiga, Anda memerlukan akun claude.ai terpisah untuk menggunakan fitur ini.
* Anda berada dalam sesi interaktif. Computer use tidak tersedia dalam mode non-interaktif dengan flag `-p`.

<h2 id="see-also">
  Lihat juga
</h2>

* [Computer use di Desktop](/docs/id/desktop#let-claude-use-your-computer): kemampuan yang sama dengan halaman pengaturan grafis
* [Claude di Chrome](/docs/id/chrome): otomasi browser untuk tugas berbasis web
* [MCP](/docs/id/mcp): hubungkan Claude ke alat dan API terstruktur
* [Sandboxing](/docs/id/sandboxing): bagaimana alat Bash Claude mengisolasi akses filesystem dan jaringan
* [Panduan keamanan computer use](https://support.claude.com/en/articles/14128542): praktik terbaik untuk computer use yang aman
