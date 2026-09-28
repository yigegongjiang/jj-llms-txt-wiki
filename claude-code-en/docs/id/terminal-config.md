> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Konfigurasi terminal Anda untuk Claude Code

> Perbaiki Shift+Enter untuk baris baru, dapatkan bel terminal saat Claude selesai, konfigurasi tmux, cocokkan tema warna, dan aktifkan mode Vim di CLI Claude Code.

Claude Code bekerja di terminal apa pun tanpa konfigurasi. Halaman ini untuk ketika sesuatu yang spesifik tidak berperilaku seperti yang Anda harapkan. Temukan gejala Anda di bawah. Jika semuanya sudah terasa benar, Anda tidak memerlukan halaman ini.

* [Shift+Enter mengirimkan alih-alih menyisipkan baris baru](#enter-multiline-prompts)
* [Pintasan tombol Option tidak melakukan apa pun di macOS](#enable-option-key-shortcuts-on-macos)
* [Tidak ada suara atau peringatan saat Claude selesai](#get-a-terminal-bell-or-notification)
* [Anda menjalankan Claude Code di dalam tmux](#configure-tmux)
* [Backspace menghapus seluruh kata di Windows](#fix-backspace-deleting-a-whole-word-on-windows)
* [Tampilan berkedip atau scrollback melompat](#switch-to-fullscreen-rendering)
* [Anda ingin kunci Vim dalam prompt](#edit-prompts-with-vim-keybindings)

Halaman ini tentang membuat terminal Anda mengirimkan sinyal yang tepat ke Claude Code. Untuk mengubah kunci mana yang Claude Code sendiri merespons, lihat [pintasan keyboard](/docs/id/keybindings) sebagai gantinya.

<h2 id="enter-multiline-prompts">
  Masukkan prompt multiline
</h2>

Menekan Enter mengirimkan pesan Anda. Untuk menambahkan jeda baris tanpa mengirimkan, tekan Ctrl+J, atau ketik `\` lalu tekan Enter. Keduanya berfungsi di setiap terminal tanpa setup.

Di sebagian besar terminal Anda juga dapat menekan Shift+Enter, tetapi dukungan bervariasi menurut emulator terminal:

| Terminal                                                                                              | Shift+Enter untuk newline                                              |
| :---------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------- |
| Ghostty, Kitty, iTerm2, WezTerm, Warp, Apple Terminal, Windows Terminal                               | Berfungsi tanpa setup                                                  |
| Terminal lain yang mendukung protokol keyboard kitty, seperti foot dan Alacritty 0.16 atau lebih baru | Berfungsi tanpa setup. Memerlukan Claude Code v2.1.269 atau lebih baru |
| VS Code, Cursor, Devin Desktop, Alacritty sebelum 0.16, Zed                                           | Jalankan `/terminal-setup` sekali                                      |
| gnome-terminal, JetBrains IDEs seperti PyCharm dan Android Studio                                     | Tidak tersedia; gunakan Ctrl+J atau `\` lalu Enter                     |

Untuk VS Code, Cursor, Devin Desktop, Alacritty sebelum 0.16, dan Zed, `/terminal-setup` menulis pintasan keyboard Shift+Enter ke dalam file konfigurasi terminal. Pada run pertama Anda melihat konfirmasi seperti `Installed VSCode terminal Shift+Enter key binding`. Binding yang ada dibiarkan tetap ada; jika Anda melihat pesan seperti `VSCode terminal Shift+Enter key binding already configured`, tidak ada perubahan yang dilakukan. Jalankan `/terminal-setup` langsung di terminal host daripada di dalam tmux atau screen, karena perlu menulis ke konfigurasi terminal host.

Di VS Code, Cursor, dan Devin Desktop, `/terminal-setup` juga memperbarui dua pengaturan editor: mengatur `terminal.integrated.gpuAcceleration` ke `"off"` untuk mencegah teks yang rusak di terminal terintegrasi, dan mengatur `terminal.integrated.mouseWheelScrollSensitivity` untuk scrolling yang lebih halus di [fullscreen mode](/docs/id/fullscreen). Untuk membatalkan perubahan akselerasi GPU, atur kembali ke `"auto"` dan muat ulang jendela editor.

Di Zed, `/terminal-setup` memperbarui `keymap.json` Anda di tempat:

* Jika keymap sudah memiliki binding dan tidak ada satupun yang merupakan Terminal `shift-enter`, Claude Code terlebih dahulu membuat backup ke salinan di direktori yang sama, seperti `keymap.json.1a2b3c4d.bak`, kemudian menggabungkan binding Shift+Enter ke dalam keymap Anda, menjaga pintasan keyboard dan komentar lainnya
* Jika Claude Code tidak dapat membaca atau mengurai keymap, tidak dapat membuat backup, atau tidak dapat memverifikasi hasil yang digabungkan, [file dibiarkan tidak berubah dan blok keybinding dicetak untuk ditambahkan sendiri](/docs/id/errors#terminal-setup-left-your-zed-keymap-unchanged)

Jika Anda menjalankan di dalam tmux, Shift+Enter juga memerlukan [konfigurasi tmux di bawah](#configure-tmux) bahkan ketika terminal luar mendukungnya.

Untuk mengikat newline ke tombol yang berbeda, atau untuk menukar perilaku sehingga Enter menyisipkan newline dan Shift+Enter mengirimkan, petakan tindakan `chat:newline` dan `chat:submit` di [file keybindings](/docs/id/keybindings) Anda.

<h2 id="enable-option-key-shortcuts-on-macos">
  Aktifkan pintasan keyboard Option di macOS
</h2>

Beberapa pintasan keyboard Claude Code menggunakan tombol Option, seperti Option+Enter untuk baris baru atau Option+P untuk beralih model. Di macOS, sebagian besar terminal tidak mengirimkan Option sebagai pengubah secara default, sehingga pintasan ini tidak berfungsi sampai Anda mengaktifkannya. Pengaturan terminal untuk ini biasanya berlabel "Use Option as Meta Key"; Meta adalah nama Unix historis untuk tombol yang sekarang berlabel Option atau Alt.

<Tabs>
  <Tab title="Apple Terminal">
    Buka Settings → Profiles → Keyboard dan centang "Use Option as Meta Key".

    Jika Anda menerima prompt pengaturan terminal first-run Claude Code, ini sudah selesai. Prompt tersebut menjalankan `/terminal-setup` untuk Anda, yang mengaktifkan Option sebagai Meta dan mematikan bel audibel di profil Apple Terminal Anda.

    Dalam [mode pembaca layar](/docs/id/accessibility), `/terminal-setup` membiarkan pengaturan bel tidak berubah sehingga bel terminal tetap audibel. Sebelum v2.1.211, `/terminal-setup` mematikan bel bahkan dalam mode pembaca layar. Jika penjalankan sebelumnya mematikan bel, aktifkan kembali di Settings → Profiles → Advanced → "Audible bell".
  </Tab>

  <Tab title="iTerm2">
    Buka Settings → Profiles → Keys → General dan atur Left Option key dan Right Option key ke "Esc+".

    Menjalankan `/terminal-setup` di iTerm2 mengaktifkan "Applications in terminal may access clipboard" di Settings → General → Selection sehingga perintah `/copy` dapat menulis ke clipboard sistem Anda. Perintah mendeteksi iTerm2 bahkan ketika dijalankan dari dalam tmux. Mulai ulang iTerm2 agar perubahan berlaku.
  </Tab>

  <Tab title="VS Code">
    Tambahkan `"terminal.integrated.macOptionIsMeta": true` ke pengaturan VS Code Anda.
  </Tab>
</Tabs>

Untuk Ghostty, Kitty, dan terminal lainnya, cari pengaturan Option-as-Alt atau Option-as-Meta dalam file konfigurasi terminal.

<h2 id="get-a-terminal-bell-or-notification">
  Dapatkan bel terminal atau notifikasi
</h2>

Ketika Claude menyelesaikan tugas atau berhenti untuk permintaan izin, dan Anda tampaknya jauh dari terminal, Claude Code mengirimkan acara notifikasi. Lihat [kapan setiap jenis notifikasi dikirim](/docs/id/hooks#notification) untuk waktu yang tepat. Menampilkan ini sebagai bel terminal atau notifikasi desktop memungkinkan Anda beralih ke pekerjaan lain saat tugas yang panjang berjalan.

Secara default Claude Code mengirimkan notifikasi desktop hanya di Ghostty, Kitty, dan iTerm2. Di terminal lain, atur [`preferredNotifChannel`](/docs/id/settings-reference#preferrednotifchannel) ke `"terminal_bell"` untuk membunyikan bel terminal sebagai gantinya, atau konfigurasikan [hook Notification](#play-a-sound-with-a-notification-hook) untuk suara atau perintah khusus. Entri pengaturan berikut mengaktifkan bel terminal:

```json ~/.claude/settings.json theme={null}
{
  "preferredNotifChannel": "terminal_bell"
}
```

Notifikasi desktop mencapai mesin lokal Anda melalui SSH, sehingga sesi jarak jauh masih dapat memperingatkan Anda. Ghostty dan Kitty meneruskannya ke pusat notifikasi OS Anda tanpa pengaturan lebih lanjut. iTerm2 memerlukan Anda untuk mengaktifkan penerusan:

<Steps>
  <Step title="Buka pengaturan notifikasi iTerm2">
    Buka Settings → Profiles → Terminal.
  </Step>

  <Step title="Aktifkan peringatan">
    Centang "Notification Center Alerts", kemudian klik "Filter Alerts" dan aktifkan "Send escape sequence-generated alerts".
  </Step>
</Steps>

Jika notifikasi masih tidak muncul, konfirmasikan bahwa aplikasi terminal Anda memiliki izin notifikasi di pengaturan OS Anda, dan jika Anda menjalankan di dalam tmux, [aktifkan passthrough](#configure-tmux).

<h3 id="play-a-sound-with-a-notification-hook">
  Mainkan suara dengan hook Notification
</h3>

Di terminal apa pun Anda dapat mengonfigurasi [hook Notification](/docs/id/hooks-guide#get-notified-when-claude-needs-input) untuk memutar suara atau menjalankan perintah khusus ketika Claude membutuhkan perhatian Anda. Hook berjalan bersama notifikasi bawaan daripada menggantinya, sehingga terminal yang tidak menerima notifikasi desktop, seperti Warp atau terminal terintegrasi VS Code, dapat menggunakan hook atau mengatur `preferredNotifChannel` ke `"terminal_bell"` sebagai gantinya.

Contoh di bawah memutar suara sistem di macOS. Panduan tertaut memiliki perintah notifikasi desktop untuk macOS, Linux, dan Windows.

```json ~/.claude/settings.json theme={null}
{
  "hooks": {
    "Notification": [
      {
        "hooks": [{ "type": "command", "command": "afplay /System/Library/Sounds/Glass.aiff" }]
      }
    ]
  }
}
```

<h2 id="configure-tmux">
  Konfigurasi tmux
</h2>

Ketika Claude Code berjalan di dalam tmux, secara default Shift+Enter mengirimkan alih-alih menyisipkan baris baru, dan notifikasi desktop serta [progress bar](/docs/id/settings-reference#terminalprogressbarenabled) tidak pernah mencapai terminal luar. Tambahkan baris-baris ini ke `~/.tmux.conf`, kemudian jalankan `tmux source-file ~/.tmux.conf` untuk menerapkannya ke server yang sedang berjalan:

```bash ~/.tmux.conf theme={null}
set -g allow-passthrough on
set -s extended-keys on
set -as terminal-features 'xterm*:extkeys'
```

Baris `allow-passthrough` memungkinkan notifikasi dan pembaruan progress mencapai terminal luar alih-alih ditelan oleh tmux. Baris `extended-keys` memungkinkan tmux membedakan Shift+Enter dari Enter biasa sehingga pintasan baris baru berfungsi.

<h2 id="fix-backspace-deleting-a-whole-word-on-windows">
  Perbaiki Backspace menghapus seluruh kata di Windows
</h2>

Di Windows, Claude Code membaca Backspace yang tiba sebagai `^H` sebagai Ctrl+Backspace, yang [menghapus kata sebelumnya](/docs/id/interactive-mode#text-editing), kecuali ketika `TERM_PROGRAM` adalah `mintty` atau `TERM` adalah `cygwin`. Di macOS dan Linux, Claude Code membacanya sebagai Backspace biasa.

Jika setiap kali menekan Backspace menghapus seluruh kata, terminal Anda mengirimkan `^H` untuk Backspace biasa. Atur [`CLAUDE_CODE_BS_AS_CTRL_BACKSPACE=0`](/docs/id/env-vars). Backspace dan Ctrl+H kemudian menghapus satu karakter masing-masing. Jika Ctrl+Backspace hanya menghapus satu karakter di macOS atau Linux karena terminal Anda mengirimkan `^H` untuknya, atur variabel ke `1` sebagai gantinya.

<h2 id="match-the-color-theme">
  Sesuaikan tema warna
</h2>

Gunakan perintah `/theme`, atau pemilih tema di `/config`, untuk memilih tema Claude Code yang sesuai dengan terminal Anda. Memilih opsi auto mendeteksi latar belakang terminal Anda yang terang atau gelap, sehingga tema mengikuti perubahan tampilan OS kapan pun terminal Anda berubah. Claude Code tidak mengontrol skema warna terminal itu sendiri, yang diatur oleh aplikasi terminal.

Untuk menyesuaikan apa yang muncul di bagian bawah antarmuka, konfigurasikan [baris status khusus](/docs/id/statusline) yang menampilkan model saat ini, direktori kerja, cabang git, atau konteks lainnya.

<h3 id="create-a-custom-theme">
  Buat tema khusus
</h3>

Selain preset bawaan, `/theme` mencantumkan tema khusus apa pun yang telah Anda tentukan dan tema apa pun yang disumbangkan oleh [plugins](/docs/id/plugins/components#themes-and-output-styles) yang terinstal. Pilih **New custom theme…** di akhir daftar untuk membuat satu secara interaktif: Anda memberi nama tema, kemudian pilih token warna individual untuk ditimpa. Tekan `Ctrl+E` saat tema khusus disorot untuk mengeditnya.

Setiap tema khusus adalah file JSON di `~/.claude/themes/`. Nama file tanpa ekstensi `.json` adalah slug tema, dan memilih tema menyimpan `custom:<slug>` sebagai preferensi tema Anda. File memiliki tiga bidang opsional:

| Field       | Type   | Description                                                                                                                                     |
| :---------- | :----- | :---------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`      | string | Label tampilan yang ditampilkan di `/theme`. Default ke slug nama file                                                                          |
| `base`      | string | Preset bawaan yang dimulai dari tema: `dark`, `light`, `dark-daltonized`, `light-daltonized`, `dark-ansi`, atau `light-ansi`. Default ke `dark` |
| `overrides` | object | Peta nama token warna ke nilai warna. Token yang tidak tercantum di sini jatuh kembali ke preset dasar                                          |

Nilai warna menerima `#rrggbb`, `#rgb`, `rgb(r,g,b)`, `ansi256(n)`, atau `ansi:<name>` di mana `<name>` adalah salah satu dari 16 nama warna ANSI standar seperti `red` atau `cyanBright`. Token yang tidak dikenal dan nilai warna yang tidak valid diabaikan, jadi kesalahan ketik tidak dapat merusak rendering.

Contoh berikut mendefinisikan tema yang mempertahankan preset gelap tetapi mengubah warna aksen prompt, teks kesalahan, dan teks kesuksesan:

```json ~/.claude/themes/dracula.json theme={null}
{
  "name": "Dracula",
  "base": "dark",
  "overrides": {
    "claude": "#bd93f9",
    "error": "#ff5555",
    "success": "#50fa7b"
  }
}
```

Claude Code memantau `~/.claude/themes/` dan memuat ulang ketika file ditambahkan atau diubah, sehingga edit yang dibuat di editor Anda berlaku untuk sesi yang sedang berjalan tanpa restart. Jika folder `~/.claude/themes/` itu sendiri tidak ada ketika Claude Code dimulai, restart sekali setelah membuat file tema pertama Anda. Setelah itu, perubahan berlaku tanpa restart.

Referensi di bawah mencakup token yang dapat Anda atur di `overrides`. Editor interaktif di `/theme` menampilkan token yang sama dengan pratinjau langsung, ditambah beberapa aksen tujuan tunggal seperti warna layar onboarding yang dihilangkan di sini.

<Accordion title="Color token reference">
  Contoh berikut menggabungkan token dari beberapa grup di bawah: aksen merek, batas mode rencana, latar belakang diff, dan latar belakang pesan.

  ```json ~/.claude/themes/midnight.json theme={null}
  {
    "name": "Midnight",
    "base": "dark",
    "overrides": {
      "claude": "#a78bfa",
      "planMode": "#38bdf8",
      "diffAdded": "#14532d",
      "diffRemoved": "#7f1d1d",
      "userMessageBackground": "#1e1b4b"
    }
  }
  ```

  <h4 id="text-and-accent-colors">
    Warna teks dan aksen
  </h4>

  Kontrol aksen merek utama dan nuansa teks latar depan yang digunakan di seluruh antarmuka.

  | Token         | Controls                                                                   |
  | :------------ | :------------------------------------------------------------------------- |
  | `claude`      | Aksen merek utama, digunakan untuk spinner dan label asisten               |
  | `text`        | Teks latar depan default                                                   |
  | `inverseText` | Teks yang digambar di atas latar belakang berwarna, seperti lencana status |
  | `inactive`    | Teks sekunder seperti petunjuk, stempel waktu, dan item yang dinonaktifkan |
  | `subtle`      | Batas samar dan teks sekunder yang dikurangi penekanannya                  |
  | `suggestion`  | Saran pelengkapan otomatis dan sorotan pilihan di pemilih                  |
  | `permission`  | Batas dialog, termasuk prompt izin dan pemilih                             |
  | `remember`    | Indikator memori dan `CLAUDE.md`                                           |

  <h4 id="status-colors">
    Status colors
  </h4>

  Sinyal keberhasilan, kegagalan, dan status peringatan di seluruh pesan dan indikator.

  | Token     | Controls                                                 |
  | :-------- | :------------------------------------------------------- |
  | `success` | Pesan kesuksesan dan pemeriksaan yang lulus              |
  | `error`   | Pesan kesalahan dan kegagalan                            |
  | `warning` | Peringatan, pesan hati-hati, dan indikator mode otomatis |
  | `merged`  | Status permintaan tarik yang digabungkan                 |

  <h4 id="input-box-and-mode-indicators">
    Input box and mode indicators
  </h4>

  Atur warna batas kotak input dan aksen yang ditampilkan saat mode izin atau indikator aktif.

  | Token          | Controls                                                                                                                                                                      |
  | :------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
  | `promptBorder` | Batas kotak input                                                                                                                                                             |
  | `planMode`     | Aksen Plan mode, pesan plan, dan dialog plan-mode                                                                                                                             |
  | `autoAccept`   | Aksen mode Accept-edits                                                                                                                                                       |
  | `bashBorder`   | Batas kotak input saat memasukkan perintah shell `!`                                                                                                                          |
  | `ide`          | Indikator koneksi IDE                                                                                                                                                         |
  | `fastMode`     | Indikator mode cepat                                                                                                                                                          |
  | `effortUltra`  | Tag `ultracode` pada batas kotak input saat [ultracode](/docs/id/model-config#adjust-effort-level) aktif. Penggantian warna Anda berlaku pada Claude Code v2.1.239 atau lebih baru |

  <h4 id="diff-rendering">
    Diff rendering
  </h4>

  Warna kode yang ditambahkan dan dihapus dalam edit dan ulasan file.

  | Token               | Controls                                                                                               |
  | :------------------ | :----------------------------------------------------------------------------------------------------- |
  | `diffAdded`         | Latar belakang baris yang ditambahkan                                                                  |
  | `diffRemoved`       | Latar belakang baris yang dihapus                                                                      |
  | `diffAddedDimmed`   | Latar belakang baris yang ditambahkan dalam diff yang digelapkan ditampilkan setelah Anda menolak edit |
  | `diffRemovedDimmed` | Latar belakang baris yang dihapus dalam diff yang digelapkan ditampilkan setelah Anda menolak edit     |
  | `diffAddedWord`     | Sorotan tingkat kata dalam baris yang ditambahkan                                                      |
  | `diffRemovedWord`   | Sorotan tingkat kata dalam baris yang dihapus                                                          |

  <h4 id="fullscreen-mode">
    Fullscreen mode
  </h4>

  Claude Code melukis `userMessageBackground`, `bashMessageBackgroundColor`, dan `memoryBackgroundColor` di kedua renderer default dan fullscreen. Ini menggunakan `userMessageBackgroundHover` dan `selectionBg` hanya dalam [mode rendering fullscreen](/docs/id/fullscreen).

  | Token                        | Controls                                                         |
  | :--------------------------- | :--------------------------------------------------------------- |
  | `userMessageBackground`      | Latar belakang di balik pesan Anda dalam transkrip               |
  | `userMessageBackgroundHover` | Latar belakang di balik pesan saat melayang atau diperluas       |
  | `bashMessageBackgroundColor` | Latar belakang di balik entri perintah shell `!` dalam transkrip |
  | `memoryBackgroundColor`      | Latar belakang di balik entri memori `#` dalam transkrip         |
  | `selectionBg`                | Latar belakang teks yang dipilih dengan mouse                    |

  <h4 id="usage-meter-and-speaker-labels">
    Usage meter and speaker labels
  </h4>

  Sesuaikan bilah yang ditampilkan dalam tampilan `/usage` dan label yang membedakan pesan Anda dari Claude.

  | Token              | Controls                                      |
  | :----------------- | :-------------------------------------------- |
  | `rate_limit_fill`  | Bagian yang diisi dari meter penggunaan       |
  | `rate_limit_empty` | Bagian yang tidak diisi dari meter penggunaan |
  | `briefLabelYou`    | Warna label `You` pada pesan Anda             |
  | `briefLabelClaude` | Warna label `Claude` pada pesan asisten       |

  <h4 id="shimmer-variants-and-subagent-colors">
    Shimmer variants and subagent colors
  </h4>

  Beberapa token memiliki varian shimmer berpasangan yang menyediakan warna lebih terang yang digunakan dalam gradien animasi spinner. Ganti shimmer bersama token dasarnya jika animasi terlihat tidak cocok.

  * `claude` dan `claudeShimmer`
  * `warning` dan `warningShimmer`
  * `permission` dan `permissionShimmer`
  * `promptBorder` dan `promptBorderShimmer`
  * `inactive` dan `inactiveShimmer`
  * `fastMode` dan `fastModeShimmer`

  Setiap [subagent](/docs/id/sub-agents) dan tugas paralel ditampilkan dalam salah satu dari delapan warna bernama sehingga Anda dapat membedakannya dalam transkrip. Nama token mengikuti pola `<color>_FOR_SUBAGENTS_ONLY`, di mana `<color>` adalah `red`, `blue`, `green`, `yellow`, `purple`, `orange`, `pink`, atau `cyan`. Ganti ini untuk mengubah tampilan setiap warna bernama. Misalnya, subagent dengan `color: blue` dalam definisinya digambar menggunakan nilai `blue_FOR_SUBAGENTS_ONLY`.

  Claude Code merender kata kunci [`ultrathink`](/docs/id/model-config#use-ultrathink-for-one-off-deep-reasoning) dalam input prompt dengan gradien pelangi tujuh warna. Nama token mengikuti pola `rainbow_<color>` dan `rainbow_<color>_shimmer`, di mana `<color>` adalah `red`, `orange`, `yellow`, `green`, `blue`, `indigo`, atau `violet`.
</Accordion>

<h2 id="switch-to-fullscreen-rendering">
  Beralih ke rendering fullscreen
</h2>

Dalam [mode pembaca layar](/docs/id/accessibility), bagian ini tidak berlaku. Claude Code selalu dirender sebagai teks gulir biasa kecuali dalam [sesi latar belakang](/docs/id/agent-view) yang terlampir, dan jika Anda menjalankan `/tui fullscreen` di sesi lain, Claude Code mencetak penjelasan alih-alih beralih.

Jika tampilan berkedip atau posisi gulir melompat saat Claude sedang bekerja, beralih ke [mode rendering fullscreen](/docs/id/fullscreen). Dalam mode ini Anda menggulir dengan mouse atau PageUp di dalam Claude Code daripada dengan scrollback asli terminal Anda; lihat [halaman fullscreen](/docs/id/fullscreen#search-and-review-the-conversation) untuk cara mencari dan menyalin.

Jika kedipan adalah satu-satunya masalah dan terminal Anda mendukung keluaran tersinkronisasi tetapi tidak terdeteksi otomatis, seperti Emacs `eat`, atur [`CLAUDE_CODE_FORCE_SYNC_OUTPUT=1`](/docs/id/env-vars) untuk menghentikan kedipan tanpa mengubah renderer.

Jalankan `/tui fullscreen` untuk beralih dan simpan preferensi. Percakapan Anda diluncurkan kembali utuh dan sesi mendatang dimulai dalam fullscreen kecuali [awal fullscreen gagal](/docs/id/fullscreen#fullscreen-renderer-didnt-finish-starting). Anda juga dapat mengatur variabel lingkungan `CLAUDE_CODE_NO_FLICKER` sebelum memulai Claude Code:

<CodeGroup>
  ```bash Bash and Zsh theme={null}
  CLAUDE_CODE_NO_FLICKER=1 claude
  ```

  ```powershell PowerShell theme={null}
  $env:CLAUDE_CODE_NO_FLICKER = "1"; claude
  ```

  ```json ~/.claude/settings.json theme={null}
  {
    "env": {
      "CLAUDE_CODE_NO_FLICKER": "1"
    }
  }
  ```
</CodeGroup>

<h2 id="paste-large-content">
  Tempel konten besar
</h2>

Ketika Anda menempel lebih dari 800 karakter atau lebih dari tiga baris ke dalam prompt, Claude Code menciutkan input ke placeholder seperti `[Pasted text #1 +120 lines]` sehingga kotak input tetap dapat digunakan, dan masih mengirimkan konten lengkap ketika Anda mengirimkan. Untuk input yang sangat besar seperti seluruh file atau log panjang, tulis konten ke file dan minta Claude membacanya alih-alih menempel. Transkrip percakapan tetap dapat dibaca dan Claude dapat merujuk ke file berdasarkan jalur di putaran berikutnya. Terminal terintegrasi VS Code juga dapat menjatuhkan karakter dari tempel yang sangat besar sebelum mencapai Claude Code, jadi gunakan file di sana.

Jika tempel membawa [karakter Unicode tak terlihat](/docs/id/interactive-mode#invisible-characters-in-prompts), Claude Code menghapusnya ketika Anda menekan Enter dan menempatkan prompt yang dibersihkan kembali di kotak input untuk Anda kirimkan dengan Enter lain.

<h3 id="how-claude-treats-pasted-text">
  Cara Claude memperlakukan teks yang ditempel
</h3>

Ketika Anda mengirimkan, Claude melihat konten di balik setiap placeholder `[Pasted text #N]` ditandai sebagai teks yang Anda tempel dari tempat lain daripada diketik. Claude diberitahu bahwa tempel dapat berisi instruksi yang tidak Anda tulis, dan untuk mengikuti instruksi di dalamnya hanya di mana pesan yang Anda ketik memintanya. Dalam sesi yang tidak [mengambil flag fitur](/docs/id/env-vars#features-that-need-feature-flag-fetching), tempel tidak ditandai.

<h3 id="delete-and-restore-a-collapsed-paste">
  Hapus dan pulihkan tempel yang diciutkan
</h3>

Ketika Anda menghapus dengan pintasan kata atau baris seperti `Ctrl+W` atau `Ctrl+K`, atau dengan penghapusan vim melalui gerakan `f`/`t` seperti `df]`, dan rentang yang dihapus mencapai dalam placeholder `[Pasted text #N]`, Claude Code menghapus placeholder sepenuhnya. Untuk memulihkannya, tempel penghapusan kembali dengan [`Ctrl+Y`](/docs/id/interactive-mode#text-editing) setelah pintasan kata atau baris, atau dengan [`p` dalam NORMAL mode](/docs/id/interactive-mode#editing-normal-mode) setelah penghapusan vim.

<h3 id="recall-a-prompt-that-had-pasted-text">
  Ingat kembali prompt yang memiliki teks yang ditempel
</h3>

Claude Code menyimpan konten di balik setiap placeholder `[Pasted text #N]` di bawah `~/.claude/paste-cache/`, sehingga ketika Anda mengingat kembali prompt dari [riwayat perintah](/docs/id/interactive-mode#command-history) dan mengirimkannya kembali, konten tempel lengkap dikirimkan lagi, termasuk dalam sesi yang lebih baru.

File cache yang lebih lama dari [`cleanupPeriodDays`](/docs/id/settings-reference#cleanupperioddays) dihapus di bawah [aturan penyapuan retensi](/docs/id/claude-directory#cleaned-up-automatically), sehingga prompt yang diingat kembali dapat mereferensikan teks tempel yang tidak lagi ada. Ketika Anda mengirimkan prompt seperti itu, Claude Code tidak pernah mengirimkan string literal `[Pasted text #N]`, dan menampilkan notifikasi yang menamai tempel yang hilang:

* Dalam prompt biasa dengan teks yang tersisa, Claude Code menghapus placeholder dan mengirimkan teks yang tersisa.
* Dalam perintah [shell mode](/docs/id/interactive-mode#shell-mode-with-prefix) atau perintah `/`, di mana penghapusan akan mengubah apa yang berjalan, dan dalam prompt apa pun penghapusan meninggalkan kosong, Claude Code membatalkan pengiriman dan menyimpan teks asli di input, dengan placeholder masih di dalamnya. Hapus placeholder atau edit perintah, kemudian kirim ulang.

<h2 id="edit-prompts-with-vim-keybindings">
  Edit prompts with Vim keybindings
</h2>

Claude Code mencakup mode editing bergaya Vim untuk input prompt. Aktifkan melalui `/config` → Editor mode, atau dengan mengatur [`editorMode`](/docs/id/settings-reference#editormode) ke `"vim"` dalam `~/.claude/settings.json`. Atur Editor mode kembali ke `normal` untuk mematikannya.

Vim mode mendukung subset dari motions dan operators mode NORMAL dan VISUAL, seperti navigasi `hjkl`, seleksi `v`/`V`, dan `d`/`c`/`y` dengan text objects. Lihat [referensi mode editor Vim](/docs/id/interactive-mode#vim-editor-mode) untuk tabel kunci lengkap.

Motions Vim tidak dapat dipetakan ulang melalui file keybindings. Untuk memetakan urutan mode INSERT dua kunci seperti `jj` ke Escape, atur [`vimInsertModeRemaps`](/docs/id/interactive-mode#remap-insert-mode-key-sequences) dalam pengaturan pengguna Anda.

Menekan Enter masih mengirimkan prompt Anda dalam mode INSERT, tidak seperti Vim standar. Gunakan `o` atau `O` dalam mode NORMAL, atau Ctrl+J, untuk menyisipkan baris baru sebagai gantinya.

<h2 id="related-resources">
  Sumber daya terkait
</h2>

* [Mode interaktif](/docs/id/interactive-mode): referensi pintasan keyboard lengkap dan tabel kunci Vim
* [Keybindings](/docs/id/keybindings): petakan ulang pintasan Claude Code apa pun, termasuk Enter dan Shift+Enter
* [Rendering fullscreen](/docs/id/fullscreen): detail tentang scrolling, pencarian, dan copy dalam mode fullscreen
* [Panduan hooks](/docs/id/hooks-guide): lebih banyak contoh hook Notification untuk Linux dan Windows
* [Troubleshooting](/docs/id/troubleshooting): perbaikan untuk masalah di luar konfigurasi terminal
