> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Konfigurasi terminal Anda untuk Claude Code

> Perbaiki Shift+Enter untuk baris baru, dapatkan bel terminal ketika Claude selesai, konfigurasi tmux, cocokkan tema warna, dan aktifkan mode Vim di CLI Claude Code.

Claude Code bekerja di terminal apa pun tanpa konfigurasi. Halaman ini untuk ketika sesuatu yang spesifik tidak berperilaku seperti yang Anda harapkan. Temukan gejala Anda di bawah. Jika semuanya sudah terasa benar, Anda tidak memerlukan halaman ini.

* [Shift+Enter mengirim alih-alih menyisipkan baris baru](#enter-multiline-prompts)
* [Pintasan tombol Option tidak melakukan apa pun di macOS](#enable-option-key-shortcuts-on-macos)
* [Tidak ada suara atau peringatan ketika Claude selesai](#get-a-terminal-bell-or-notification)
* [Anda menjalankan Claude Code di dalam tmux](#configure-tmux)
* [Tampilan berkedip atau scrollback melompat](#switch-to-fullscreen-rendering)
* [Anda ingin kunci Vim di prompt](#edit-prompts-with-vim-keybindings)

Halaman ini tentang membuat terminal Anda mengirimkan sinyal yang tepat ke Claude Code. Untuk mengubah kunci mana yang Claude Code sendiri merespons, lihat [keybindings](/docs/id/keybindings) sebagai gantinya.

<h2 id="enter-multiline-prompts">
  Masukkan prompt multiline
</h2>

Menekan Enter mengirimkan pesan Anda. Untuk menambahkan jeda baris tanpa mengirimkan, tekan Ctrl+J, atau ketik `\` dan kemudian tekan Enter. Keduanya bekerja di setiap terminal tanpa setup.

Di sebagian besar terminal Anda juga dapat menekan Shift+Enter, tetapi dukungan bervariasi menurut emulator terminal:

| Terminal                                                                | Shift+Enter untuk baris baru                           |
| :---------------------------------------------------------------------- | :----------------------------------------------------- |
| Ghostty, Kitty, iTerm2, WezTerm, Warp, Apple Terminal, Windows Terminal | Bekerja tanpa setup                                    |
| VS Code, Cursor, Devin Desktop, Alacritty, Zed                          | Jalankan `/terminal-setup` sekali                      |
| gnome-terminal, JetBrains IDEs seperti PyCharm dan Android Studio       | Tidak tersedia; gunakan Ctrl+J atau `\` kemudian Enter |

Untuk VS Code, Cursor, Devin Desktop, Alacritty, dan Zed, `/terminal-setup` menulis Shift+Enter dan pintasan keyboard lainnya ke dalam file konfigurasi terminal. Binding yang ada dibiarkan tetap ada; jika Anda melihat pesan seperti `VSCode terminal Shift+Enter key binding already configured`, tidak ada perubahan yang dilakukan. Jalankan `/terminal-setup` langsung di terminal host daripada di dalam tmux atau screen, karena perlu menulis ke konfigurasi terminal host.

Di VS Code, Cursor, dan Devin Desktop, `/terminal-setup` juga memperbarui dua pengaturan editor: menetapkan `terminal.integrated.gpuAcceleration` ke `"off"` untuk mencegah teks yang rusak di terminal terintegrasi, dan menetapkan `terminal.integrated.mouseWheelScrollSensitivity` untuk scrolling yang lebih halus dalam [mode fullscreen](/docs/id/fullscreen). Untuk membatalkan perubahan akselerasi GPU, atur kembali ke `"auto"` dan muat ulang jendela editor.

Jika Anda menjalankan di dalam tmux, Shift+Enter juga memerlukan [konfigurasi tmux di bawah](#configure-tmux) bahkan ketika terminal luar mendukungnya.

Untuk mengikat baris baru ke kunci yang berbeda, atau untuk menukar perilaku sehingga Enter menyisipkan baris baru dan Shift+Enter mengirim, petakan tindakan `chat:newline` dan `chat:submit` di [file pintasan keyboard](/docs/id/keybindings) Anda.

<h2 id="enable-option-key-shortcuts-on-macos">
  Aktifkan pintasan tombol Option di macOS
</h2>

Beberapa pintasan Claude Code menggunakan tombol Option, seperti Option+Enter untuk baris baru atau Option+P untuk beralih model. Di macOS, sebagian besar terminal tidak mengirimkan Option sebagai pengubah secara default, jadi pintasan ini tidak melakukan apa pun sampai Anda mengaktifkannya. Pengaturan terminal untuk ini biasanya berlabel "Use Option as Meta Key"; Meta adalah nama Unix historis untuk kunci yang sekarang berlabel Option atau Alt.

<Tabs>
  <Tab title="Apple Terminal">
    Buka Settings → Profiles → Keyboard dan centang "Use Option as Meta Key".

    Jika Anda menerima prompt first-run Claude Code yang menawarkan "Option+Enter untuk baris baru dan visual bell", ini sudah dilakukan. Prompt itu menjalankan `/terminal-setup` untuk Anda, yang mengaktifkan Option sebagai Meta dan mengalihkan audio bell ke flash layar visual di profil Apple Terminal Anda.
  </Tab>

  <Tab title="iTerm2">
    Buka Settings → Profiles → Keys → General dan atur Left Option key dan Right Option key ke "Esc+".

    Menjalankan `/terminal-setup` di iTerm2 mengaktifkan "Applications in terminal may access clipboard" di bawah Settings → General → Selection sehingga perintah `/copy` dapat menulis ke clipboard sistem Anda. Perintah mendeteksi iTerm2 bahkan ketika dijalankan dari dalam tmux. Mulai ulang iTerm2 agar perubahan berlaku.
  </Tab>

  <Tab title="VS Code">
    Tambahkan `"terminal.integrated.macOptionIsMeta": true` ke pengaturan VS Code Anda.
  </Tab>
</Tabs>

Untuk Ghostty, Kitty, dan terminal lainnya, cari pengaturan Option-as-Alt atau Option-as-Meta di file konfigurasi terminal.

<h2 id="get-a-terminal-bell-or-notification">
  Dapatkan bel terminal atau notifikasi
</h2>

Ketika Claude menyelesaikan tugas atau berhenti untuk prompt izin, ia mengirimkan acara notifikasi. Menampilkan ini sebagai bel terminal atau notifikasi desktop memungkinkan Anda beralih ke pekerjaan lain saat tugas panjang berjalan.

Secara default Claude Code mengirimkan notifikasi desktop hanya di Ghostty, Kitty, dan iTerm2. Di terminal lain, atur [`preferredNotifChannel`](/docs/id/settings#available-settings) ke `"terminal_bell"` untuk membunyikan bel terminal sebagai gantinya, atau konfigurasikan [hook Notification](#play-a-sound-with-a-notification-hook) untuk suara khusus atau perintah.

Notifikasi desktop mencapai mesin lokal Anda melalui SSH, jadi sesi jarak jauh masih dapat memperingatkan Anda. Ghostty dan Kitty meneruskannya ke pusat notifikasi OS Anda tanpa setup lebih lanjut. iTerm2 memerlukan Anda untuk mengaktifkan penerusan:

<Steps>
  <Step title="Buka pengaturan notifikasi iTerm2">
    Buka Settings → Profiles → Terminal.
  </Step>

  <Step title="Aktifkan peringatan">
    Centang "Notification Center Alerts", kemudian klik "Filter Alerts" dan aktifkan "Send escape sequence-generated alerts".
  </Step>
</Steps>

Jika notifikasi masih tidak muncul, konfirmasi bahwa aplikasi terminal Anda memiliki izin notifikasi di pengaturan OS Anda, dan jika Anda menjalankan di dalam tmux, [aktifkan passthrough](#configure-tmux).

<h3 id="play-a-sound-with-a-notification-hook">
  Putar suara dengan hook Notification
</h3>

Di terminal apa pun Anda dapat mengonfigurasi [hook Notification](/docs/id/hooks-guide#get-notified-when-claude-needs-input) untuk memutar suara atau menjalankan perintah khusus ketika Claude memerlukan perhatian Anda. Hook berjalan bersama notifikasi desktop daripada menggantinya, jadi terminal yang tidak menerima notifikasi desktop, seperti Warp atau terminal terintegrasi VS Code, dapat menggunakan hook atau mengatur `preferredNotifChannel` ke `"terminal_bell"` sebagai gantinya.

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

Ketika Claude Code berjalan di dalam tmux, dua hal rusak secara default: Shift+Enter mengirim alih-alih menyisipkan baris baru, dan notifikasi desktop dan [progress bar](/docs/id/settings#available-settings) tidak pernah mencapai terminal luar. Tambahkan baris-baris ini ke `~/.tmux.conf`, kemudian jalankan `tmux source-file ~/.tmux.conf` untuk menerapkannya ke server yang berjalan:

```bash ~/.tmux.conf theme={null}
set -g allow-passthrough on
set -s extended-keys on
set -as terminal-features 'xterm*:extkeys'
```

Baris `allow-passthrough` memungkinkan notifikasi dan pembaruan kemajuan mencapai terminal luar alih-alih ditelan oleh tmux. Baris `extended-keys` memungkinkan tmux membedakan Shift+Enter dari Enter biasa sehingga pintasan baris baru bekerja.

<h2 id="match-the-color-theme">
  Cocokkan tema warna
</h2>

Gunakan perintah `/theme`, atau pemilih tema di `/config`, untuk memilih tema Claude Code yang cocok dengan terminal Anda. Memilih opsi auto mendeteksi latar belakang terminal Anda yang terang atau gelap, jadi tema mengikuti perubahan penampilan OS kapan pun terminal Anda melakukannya. Claude Code tidak mengontrol skema warna terminal itu sendiri, yang diatur oleh aplikasi terminal.

Untuk menyesuaikan apa yang muncul di bagian bawah antarmuka, konfigurasikan [baris status khusus](/docs/id/statusline) yang menampilkan model saat ini, direktori kerja, cabang git, atau konteks lainnya.

<h3 id="create-a-custom-theme">
  Buat tema khusus
</h3>

<Note>
  Tema khusus memerlukan Claude Code v2.1.118 atau lebih baru.
</Note>

Selain preset bawaan, `/theme` mencantumkan tema khusus apa pun yang telah Anda tentukan dan tema apa pun yang disumbangkan oleh [plugins](/docs/id/plugins-reference#themes) yang terinstal. Pilih **Tema khusus baru…** di akhir daftar untuk membuat satu secara interaktif: Anda memberi nama tema, kemudian memilih token warna individual untuk ditimpa. Tekan `Ctrl+E` saat tema khusus disorot untuk mengeditnya.

Setiap tema khusus adalah file JSON di `~/.claude/themes/`. Nama file tanpa ekstensi `.json` adalah slug tema, dan memilih tema menyimpan `custom:<slug>` sebagai preferensi tema Anda. File memiliki tiga bidang opsional:

| Bidang      | Tipe   | Deskripsi                                                                                                                                              |
| :---------- | :----- | :----------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`      | string | Label tampilan yang ditampilkan di `/theme`. Defaultnya adalah slug nama file                                                                          |
| `base`      | string | Preset bawaan yang dimulai dari tema: `dark`, `light`, `dark-daltonized`, `light-daltonized`, `dark-ansi`, atau `light-ansi`. Defaultnya adalah `dark` |
| `overrides` | object | Peta nama token warna ke nilai warna. Token yang tidak tercantum di sini jatuh kembali ke preset dasar                                                 |

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

Claude Code memantau `~/.claude/themes/` dan memuat ulang saat file berubah, jadi edit yang dibuat di editor Anda berlaku untuk sesi yang sedang berjalan tanpa restart.

Referensi di bawah mencakup token yang dapat Anda atur di `overrides`. Editor interaktif di `/theme` menampilkan token yang sama dengan pratinjau langsung, ditambah beberapa aksen tujuan tunggal seperti warna layar onboarding yang dihilangkan di sini.

<Accordion title="Referensi token warna">
  Contoh berikut menggabungkan token dari beberapa grup di bawah: aksen merek, batas mode rencana, latar belakang diff, dan latar belakang pesan layar penuh.

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

  | Token         | Kontrol                                                                    |
  | :------------ | :------------------------------------------------------------------------- |
  | `claude`      | Aksen merek utama, digunakan untuk spinner dan label asisten               |
  | `text`        | Teks latar depan default                                                   |
  | `inverseText` | Teks yang digambar di atas latar belakang berwarna, seperti lencana status |
  | `inactive`    | Teks sekunder seperti petunjuk, stempel waktu, dan item yang dinonaktifkan |
  | `subtle`      | Batas samar dan teks sekunder yang dikurangi penekanannya                  |
  | `suggestion`  | Saran pelengkapan otomatis dan sorotan pilihan dalam pemilih               |
  | `permission`  | Batas dialog, termasuk prompt izin dan pemilih                             |
  | `remember`    | Indikator memori dan `CLAUDE.md`                                           |

  <h4 id="status-colors">
    Warna status
  </h4>

  Sinyal keberhasilan, kegagalan, dan status peringatan di seluruh pesan dan indikator.

  | Token     | Kontrol                                              |
  | :-------- | :--------------------------------------------------- |
  | `success` | Pesan kesuksesan dan pemeriksaan yang lulus          |
  | `error`   | Pesan kesalahan dan kegagalan                        |
  | `warning` | Peringatan, pesan hati-hati, dan batas mode otomatis |
  | `merged`  | Status permintaan tarik yang digabungkan             |

  <h4 id="input-box-and-mode-indicators">
    Kotak input dan indikator mode
  </h4>

  Atur warna batas kotak input dan aksen yang ditampilkan saat mode izin atau indikator aktif.

  | Token          | Kontrol                                              |
  | :------------- | :--------------------------------------------------- |
  | `promptBorder` | Batas kotak input dalam mode izin default            |
  | `planMode`     | Aksen dan batas mode rencana                         |
  | `autoAccept`   | Aksen dan batas mode terima-edit                     |
  | `bashBorder`   | Batas kotak input saat memasukkan perintah shell `!` |
  | `ide`          | Indikator koneksi IDE                                |
  | `fastMode`     | Indikator mode cepat                                 |

  <h4 id="diff-rendering">
    Rendering diff
  </h4>

  Warna kode yang ditambahkan dan dihapus dalam edit dan ulasan file.

  | Token               | Kontrol                                                                   |
  | :------------------ | :------------------------------------------------------------------------ |
  | `diffAdded`         | Latar belakang baris yang ditambahkan                                     |
  | `diffRemoved`       | Latar belakang baris yang dihapus                                         |
  | `diffAddedDimmed`   | Latar belakang konteks yang tidak berubah di dekat baris yang ditambahkan |
  | `diffRemovedDimmed` | Latar belakang konteks yang tidak berubah di dekat baris yang dihapus     |
  | `diffAddedWord`     | Sorotan tingkat kata dalam baris yang ditambahkan                         |
  | `diffRemovedWord`   | Sorotan tingkat kata dalam baris yang dihapus                             |

  <h4 id="fullscreen-mode">
    Mode layar penuh
  </h4>

  Terapkan hanya dalam [mode rendering layar penuh](/docs/id/fullscreen), di mana pesan memiliki isian latar belakang.

  | Token                        | Kontrol                                                                |
  | :--------------------------- | :--------------------------------------------------------------------- |
  | `userMessageBackground`      | Latar belakang di balik pesan Anda dalam transkrip                     |
  | `userMessageBackgroundHover` | Latar belakang di balik pesan saat diarahkan atau diperluas            |
  | `messageActionsBackground`   | Latar belakang di balik pesan yang dipilih saat bilah tindakan terbuka |
  | `bashMessageBackgroundColor` | Latar belakang di balik entri perintah shell `!` dalam transkrip       |
  | `memoryBackgroundColor`      | Latar belakang di balik entri memori `#` dalam transkrip               |
  | `selectionBg`                | Latar belakang teks yang dipilih dengan mouse                          |

  <h4 id="usage-meter-and-speaker-labels">
    Meter penggunaan dan label pembicara
  </h4>

  Sesuaikan bilah yang ditampilkan dalam tampilan `/usage` dan label yang membedakan pesan Anda dari Claude.

  | Token              | Kontrol                                       |
  | :----------------- | :-------------------------------------------- |
  | `rate_limit_fill`  | Bagian yang diisi dari meter penggunaan       |
  | `rate_limit_empty` | Bagian yang tidak diisi dari meter penggunaan |
  | `briefLabelYou`    | Warna label `You` pada pesan Anda             |
  | `briefLabelClaude` | Warna label `Claude` pada pesan asisten       |

  <h4 id="shimmer-variants-and-subagent-colors">
    Varian shimmer dan warna subagen
  </h4>

  Beberapa token memiliki varian shimmer berpasangan yang menyediakan warna yang lebih ringan yang digunakan dalam gradien animasi spinner. Timpa shimmer bersama token dasarnya jika animasi terlihat tidak cocok.

  * `claude` dan `claudeShimmer`
  * `warning` dan `warningShimmer`
  * `permission` dan `permissionShimmer`
  * `promptBorder` dan `promptBorderShimmer`
  * `inactive` dan `inactiveShimmer`
  * `fastMode` dan `fastModeShimmer`

  Setiap [subagen](/docs/id/sub-agents) dan tugas paralel ditampilkan dalam salah satu dari delapan warna bernama sehingga Anda dapat membedakannya dalam transkrip. Nama token mengikuti pola `<color>_FOR_SUBAGENTS_ONLY`, di mana `<color>` adalah `red`, `blue`, `green`, `yellow`, `purple`, `orange`, `pink`, atau `cyan`. Timpa ini untuk mengubah tampilan setiap warna bernama. Misalnya, subagen dengan `color: blue` dalam definisinya digambar menggunakan nilai `blue_FOR_SUBAGENTS_ONLY`.

  Kata kunci [`ultrathink`](/docs/id/model-config#use-ultrathink-for-one-off-deep-reasoning) dan [`ultraplan`](/docs/id/ultraplan) dalam input prompt dirender dengan gradien pelangi tujuh warna. Nama token mengikuti pola `rainbow_<color>` dan `rainbow_<color>_shimmer`, di mana `<color>` adalah `red`, `orange`, `yellow`, `green`, `blue`, `indigo`, atau `violet`.
</Accordion>

<h2 id="switch-to-fullscreen-rendering">
  Beralih ke rendering fullscreen
</h2>

Jika tampilan berkedip atau posisi scroll melompat saat Claude sedang bekerja, beralih ke [mode rendering fullscreen](/docs/id/fullscreen). Ini menggambar ke layar terpisah yang terminal cadangkan untuk aplikasi full-screen alih-alih menambahkan ke scrollback normal Anda, yang menjaga penggunaan memori tetap datar dan menambahkan dukungan mouse untuk scrolling dan seleksi. Dalam mode ini Anda scroll dengan mouse atau PageUp di dalam Claude Code daripada dengan scrollback native terminal Anda; lihat [halaman fullscreen](/docs/id/fullscreen#search-and-review-the-conversation) untuk cara mencari dan menyalin.

Jika kedipan adalah satu-satunya masalah dan terminal Anda mendukung synchronized output tetapi tidak terdeteksi otomatis, seperti Emacs `eat`, atur [`CLAUDE_CODE_FORCE_SYNC_OUTPUT=1`](/docs/id/env-vars) untuk menghentikan kedipan tanpa mengubah renderer.

Jalankan `/tui fullscreen` untuk beralih dan simpan preferensi. Percakapan Anda diluncurkan kembali utuh dan sesi mendatang dimulai dalam fullscreen. Anda juga dapat mengatur variabel lingkungan `CLAUDE_CODE_NO_FLICKER` sebelum memulai Claude Code:

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

Ketika Anda menempel lebih dari 10.000 karakter ke dalam prompt, Claude Code menciutkan input ke placeholder `[Pasted text]` sehingga kotak input tetap dapat digunakan. Konten lengkap masih dikirim ke Claude ketika Anda mengirimkan.

Terminal terintegrasi VS Code dapat menjatuhkan karakter dari penempelan sangat besar sebelum mereka mencapai Claude Code, jadi lebih suka alur kerja berbasis file di sana. Untuk input sangat besar seperti seluruh file atau log panjang, tulis konten ke file dan minta Claude untuk membacanya alih-alih menempel. Ini menjaga transkrip percakapan tetap dapat dibaca dan memungkinkan Claude mereferensikan file berdasarkan path di giliran kemudian.

<h2 id="edit-prompts-with-vim-keybindings">
  Edit prompt dengan pintasan keyboard Vim
</h2>

Claude Code mencakup mode pengeditan gaya Vim untuk input prompt. Aktifkan melalui `/config` → Editor mode, atau dengan mengatur [`editorMode`](/docs/id/settings#available-settings) ke `"vim"` di `~/.claude/settings.json`. Atur Editor mode kembali ke `normal` untuk mematikannya.

Mode Vim mendukung subset gerakan dan operator mode NORMAL dan VISUAL, seperti navigasi `hjkl`, seleksi `v`/`V`, dan `d`/`c`/`y` dengan objek teks. Lihat [referensi mode editor Vim](/docs/id/interactive-mode#vim-editor-mode) untuk tabel kunci lengkap.

Gerakan Vim tidak dapat dipetakan ulang melalui file pintasan keyboard. Untuk memetakan urutan mode INSERT dua kunci seperti `jj` ke Escape, atur [`vimInsertModeRemaps`](/docs/id/interactive-mode#remap-insert-mode-key-sequences) dalam pengaturan pengguna Anda.

Menekan Enter masih mengirimkan prompt Anda dalam mode INSERT, tidak seperti Vim standar. Gunakan `o` atau `O` dalam mode NORMAL, atau Ctrl+J, untuk menyisipkan baris baru sebagai gantinya.

<h2 id="related-resources">
  Sumber daya terkait
</h2>

* [Mode interaktif](/docs/id/interactive-mode): referensi pintasan keyboard lengkap dan tabel kunci Vim
* [Keybindings](/docs/id/keybindings): petakan ulang pintasan Claude Code apa pun, termasuk Enter dan Shift+Enter
* [Rendering fullscreen](/docs/id/fullscreen): detail tentang scrolling, pencarian, dan copy dalam mode fullscreen
* [Panduan hooks](/docs/id/hooks-guide): lebih banyak contoh hook Notification untuk Linux dan Windows
* [Troubleshooting](/docs/id/troubleshooting): perbaikan untuk masalah di luar konfigurasi terminal
