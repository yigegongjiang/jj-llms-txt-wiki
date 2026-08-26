> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Sesuaikan pintasan keyboard

> Sesuaikan pintasan keyboard di Claude Code dengan file konfigurasi keybindings.

Claude Code mendukung pintasan keyboard yang dapat disesuaikan. Jalankan `/keybindings` untuk membuat atau membuka file konfigurasi Anda di `~/.claude/keybindings.json`.

<h2 id="configuration-file">
  File konfigurasi
</h2>

File konfigurasi pintasan keyboard adalah objek dengan array `bindings`. Setiap blok menentukan konteks dan peta dari keystroke ke tindakan.

<Note>Perubahan pada file pintasan keyboard secara otomatis terdeteksi dan diterapkan tanpa perlu memulai ulang Claude Code.</Note>

| Field      | Deskripsi                                                   |
| :--------- | :---------------------------------------------------------- |
| `$schema`  | URL JSON Schema opsional untuk penyelesaian otomatis editor |
| `$docs`    | URL dokumentasi opsional                                    |
| `bindings` | Array blok binding berdasarkan konteks                      |

Contoh ini mengikat `Ctrl+E` untuk membuka editor eksternal dalam konteks chat, dan membatalkan ikatan `Ctrl+U`:

```json theme={null}
{
  "$schema": "https://www.schemastore.org/claude-code-keybindings.json",
  "$docs": "https://code.claude.com/docs/id/keybindings",
  "bindings": [
    {
      "context": "Chat",
      "bindings": {
        "ctrl+e": "chat:externalEditor",
        "ctrl+u": null
      }
    }
  ]
}
```

<h2 id="contexts">
  Konteks
</h2>

Setiap blok binding menentukan **konteks** di mana binding berlaku:

| Konteks           | Deskripsi                                                       |
| :---------------- | :-------------------------------------------------------------- |
| `Global`          | Berlaku di mana saja dalam aplikasi                             |
| `Chat`            | Area input chat utama                                           |
| `Autocomplete`    | Menu penyelesaian otomatis terbuka                              |
| `Settings`        | Menu pengaturan                                                 |
| `Confirmation`    | Dialog izin dan konfirmasi                                      |
| `Tabs`            | Komponen navigasi tab                                           |
| `Help`            | Menu bantuan terlihat                                           |
| `Transcript`      | Penampil transkrip                                              |
| `HistorySearch`   | Mode pencarian riwayat (Ctrl+R)                                 |
| `Task`            | Tugas latar belakang sedang berjalan                            |
| `ThemePicker`     | Dialog pemilih tema                                             |
| `Attachments`     | Navigasi lampiran gambar dalam dialog pilih                     |
| `Footer`          | Navigasi indikator footer (tugas, tim, diff)                    |
| `MessageSelector` | Pemilihan pesan dialog rewind dan ringkasan                     |
| `DiffDialog`      | Navigasi penampil diff                                          |
| `ModelPicker`     | Tingkat upaya pemilih model                                     |
| `Select`          | Komponen select/list generik                                    |
| `Plugin`          | Dialog plugin (jelajahi, temukan, kelola)                       |
| `Scroll`          | Pengguliran percakapan dan pemilihan teks dalam mode fullscreen |

Sebelum v2.1.205, konteks `Doctor` dan tindakan `doctor:fix` ada untuk layar diagnostik `/doctor`.

<h2 id="available-actions">
  Tindakan yang tersedia
</h2>

Tindakan mengikuti format `namespace:action`, seperti `chat:submit` untuk mengirim pesan atau `app:toggleTodos` untuk menampilkan daftar tugas. Setiap konteks memiliki tindakan spesifik yang tersedia.

<h3 id="app-actions">
  Tindakan aplikasi
</h3>

Tindakan yang tersedia dalam konteks `Global`:

| Tindakan               | Default   | Deskripsi                                                                                            |
| :--------------------- | :-------- | :--------------------------------------------------------------------------------------------------- |
| `app:interrupt`        | Ctrl+C    | Batalkan operasi saat ini                                                                            |
| `app:exit`             | Ctrl+D    | Keluar dari Claude Code                                                                              |
| `app:redraw`           | (unbound) | Paksa terminal untuk digambar ulang                                                                  |
| `app:toggleTodos`      | Ctrl+T    | Alihkan visibilitas daftar tugas Claude. Ini bukan tampilan background-task [`/tasks`](/docs/id/commands) |
| `app:toggleTranscript` | Ctrl+O    | Alihkan transkrip verbose                                                                            |

<h3 id="history-actions">
  Tindakan riwayat
</h3>

Tindakan untuk menavigasi riwayat perintah:

| Tindakan           | Default | Deskripsi               |
| :----------------- | :------ | :---------------------- |
| `history:search`   | Ctrl+R  | Buka pencarian riwayat  |
| `history:previous` | Up      | Item riwayat sebelumnya |
| `history:next`     | Down    | Item riwayat berikutnya |

<h3 id="chat-actions">
  Tindakan chat
</h3>

Tindakan yang tersedia dalam konteks `Chat`:

| Tindakan              | Default                           | Deskripsi                                                                                                                                                                            |
| :-------------------- | :-------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `chat:cancel`         | Escape                            | Batalkan input saat ini                                                                                                                                                              |
| `chat:clearInput`     | Ctrl+L                            | Paksa gambar ulang layar penuh, mempertahankan input. Dalam [rendering fullscreen](/docs/id/fullscreen#clear-the-conversation), tekan dua kali dalam dua detik untuk menjalankan `/clear` |
| `chat:clearScreen`    | Cmd+K                             | Dalam [rendering fullscreen](/docs/id/fullscreen#clear-the-conversation), tekan dua kali dalam dua detik untuk menjalankan `/clear`                                                       |
| `chat:killAgents`     | Ctrl+X Ctrl+K                     | Matikan semua [subagen latar belakang](/docs/id/sub-agents#run-subagents-in-foreground-or-background) dalam sesi ini                                                                      |
| `chat:cycleMode`      | Shift+Tab\*                       | Mode izin siklus                                                                                                                                                                     |
| `chat:modelPicker`    | Meta+P                            | Buka pemilih model                                                                                                                                                                   |
| `chat:fastMode`       | Meta+O                            | Alihkan mode cepat                                                                                                                                                                   |
| `chat:thinkingToggle` | Meta+T                            | Alihkan pemikiran yang diperluas                                                                                                                                                     |
| `chat:submit`         | Enter                             | Kirim pesan                                                                                                                                                                          |
| `chat:newline`        | Ctrl+J                            | Sisipkan baris baru tanpa mengirim                                                                                                                                                   |
| `chat:undo`           | Ctrl+\_, Ctrl+Shift+-             | Batalkan tindakan terakhir                                                                                                                                                           |
| `chat:externalEditor` | Ctrl+G, Ctrl+X Ctrl+E             | Buka di editor eksternal                                                                                                                                                             |
| `chat:stash`          | Ctrl+S                            | Simpan prompt saat ini                                                                                                                                                               |
| `chat:imagePaste`     | Ctrl+V (Alt+V di Windows dan WSL) | Tempel gambar dari clipboard. Di WSL, kedua pintasan keyboard terikat secara default                                                                                                 |

\*Di Windows tanpa mode VT (Node \<24.2.0/\<22.17.0, Bun \<1.2.23), default ke Meta+M.

<h3 id="autocomplete-actions">
  Tindakan penyelesaian otomatis
</h3>

Tindakan yang tersedia dalam konteks `Autocomplete`:

| Tindakan                | Default | Deskripsi        |
| :---------------------- | :------ | :--------------- |
| `autocomplete:accept`   | Tab     | Terima saran     |
| `autocomplete:dismiss`  | Escape  | Tutup menu       |
| `autocomplete:previous` | Up      | Saran sebelumnya |
| `autocomplete:next`     | Down    | Saran berikutnya |

<h3 id="confirmation-actions">
  Tindakan konfirmasi
</h3>

Tindakan yang tersedia dalam konteks `Confirmation`:

| Tindakan                    | Default   | Deskripsi                                                                          |
| :-------------------------- | :-------- | :--------------------------------------------------------------------------------- |
| `confirm:yes`               | Y, Enter  | Konfirmasi tindakan                                                                |
| `confirm:no`                | N, Escape | Tolak tindakan                                                                     |
| `confirm:previous`          | Up        | Opsi sebelumnya                                                                    |
| `confirm:next`              | Down      | Opsi berikutnya                                                                    |
| `confirm:nextField`         | Tab       | Bidang berikutnya                                                                  |
| `confirm:previousField`     | (unbound) | Bidang sebelumnya                                                                  |
| `confirm:toggle`            | Space     | Alihkan pilihan                                                                    |
| `confirm:cycleMode`         | Shift+Tab | Mode izin siklus                                                                   |
| `confirm:toggleExplanation` | Ctrl+E    | Alihkan penjelasan izin yang dihasilkan model pada prompt izin Bash dan PowerShell |

<h3 id="permission-actions">
  Tindakan izin
</h3>

Tindakan yang tersedia dalam konteks `Confirmation` untuk dialog izin:

| Tindakan                 | Default   | Deskripsi                                                                                                    |
| :----------------------- | :-------- | :----------------------------------------------------------------------------------------------------------- |
| `permission:toggleDebug` | (unbound) | Alihkan info debug izin. Default sebelumnya dari Ctrl+D dihapus dalam v2.1.146 karena mengaburkan `app:exit` |

<h3 id="transcript-actions">
  Tindakan transkrip
</h3>

Tindakan yang tersedia dalam konteks `Transcript`:

| Tindakan                   | Default           | Deskripsi                      |
| :------------------------- | :---------------- | :----------------------------- |
| `transcript:toggleShowAll` | Ctrl+E            | Alihkan tampilkan semua konten |
| `transcript:exit`          | q, Ctrl+C, Escape | Keluar dari tampilan transkrip |

<h3 id="history-search-actions">
  Tindakan pencarian riwayat
</h3>

Tindakan yang tersedia dalam konteks `HistorySearch`:

| Tindakan                   | Default     | Deskripsi                                  |
| :------------------------- | :---------- | :----------------------------------------- |
| `historySearch:next`       | Ctrl+R      | Kecocokan berikutnya                       |
| `historySearch:accept`     | Escape, Tab | Terima pilihan                             |
| `historySearch:cancel`     | Ctrl+C      | Batalkan pencarian                         |
| `historySearch:execute`    | Enter       | Jalankan perintah yang dipilih             |
| `historySearch:cycleScope` | Ctrl+S      | Siklus cakupan: sesi, proyek, di mana saja |

<h3 id="task-actions">
  Tindakan tugas
</h3>

Tindakan yang tersedia dalam konteks `Task`:

| Tindakan          | Default               | Deskripsi                                                                                                                  |
| :---------------- | :-------------------- | :------------------------------------------------------------------------------------------------------------------------- |
| `task:background` | Ctrl+B, Ctrl+X Ctrl+B | Tugas latar belakang saat ini. Akord Ctrl+X Ctrl+B memerlukan v2.1.169 atau lebih baru dan menghindari konflik awalan tmux |

<h3 id="theme-actions">
  Tindakan tema
</h3>

Tindakan yang tersedia dalam konteks `ThemePicker`:

| Tindakan                         | Default | Deskripsi                  |
| :------------------------------- | :------ | :------------------------- |
| `theme:toggleSyntaxHighlighting` | Ctrl+T  | Alihkan penyorotan sintaks |

<h3 id="help-actions">
  Tindakan bantuan
</h3>

Tindakan yang tersedia dalam konteks `Help`:

| Tindakan       | Default | Deskripsi          |
| :------------- | :------ | :----------------- |
| `help:dismiss` | Escape  | Tutup menu bantuan |

<h3 id="tabs-actions">
  Tindakan tab
</h3>

Tindakan yang tersedia dalam konteks `Tabs`:

| Tindakan        | Default         | Deskripsi      |
| :-------------- | :-------------- | :------------- |
| `tabs:next`     | Tab, Right      | Tab berikutnya |
| `tabs:previous` | Shift+Tab, Left | Tab sebelumnya |

<h3 id="attachments-actions">
  Tindakan lampiran
</h3>

Tindakan yang tersedia dalam konteks `Attachments`:

| Tindakan               | Default           | Deskripsi                     |
| :--------------------- | :---------------- | :---------------------------- |
| `attachments:next`     | Right             | Lampiran berikutnya           |
| `attachments:previous` | Left              | Lampiran sebelumnya           |
| `attachments:remove`   | Backspace, Delete | Hapus lampiran yang dipilih   |
| `attachments:exit`     | Down, Escape      | Keluar dari navigasi lampiran |

<h3 id="footer-actions">
  Tindakan footer
</h3>

Tindakan yang tersedia dalam konteks `Footer`:

| Tindakan                | Default | Deskripsi                                                |
| :---------------------- | :------ | :------------------------------------------------------- |
| `footer:next`           | Right   | Item footer berikutnya                                   |
| `footer:previous`       | Left    | Item footer sebelumnya                                   |
| `footer:up`             | Up      | Navigasi ke atas dalam footer (batalkan pilihan di atas) |
| `footer:down`           | Down    | Navigasi ke bawah dalam footer                           |
| `footer:openSelected`   | Enter   | Buka item footer yang dipilih                            |
| `footer:clearSelection` | Escape  | Hapus pilihan footer                                     |

<h3 id="message-selector-actions">
  Tindakan pemilih pesan
</h3>

Tindakan yang tersedia dalam konteks `MessageSelector`:

| Tindakan                 | Default                                   | Deskripsi          |
| :----------------------- | :---------------------------------------- | :----------------- |
| `messageSelector:up`     | Up, K, Ctrl+P                             | Naik dalam daftar  |
| `messageSelector:down`   | Down, J, Ctrl+N                           | Turun dalam daftar |
| `messageSelector:top`    | Ctrl+Up, Shift+Up, Meta+Up, Shift+K       | Lompat ke atas     |
| `messageSelector:bottom` | Ctrl+Down, Shift+Down, Meta+Down, Shift+J | Lompat ke bawah    |
| `messageSelector:select` | Enter                                     | Pilih pesan        |

<h3 id="diff-actions">
  Tindakan diff
</h3>

Tindakan yang tersedia dalam konteks `DiffDialog`:

| Tindakan              | Default   | Deskripsi                                                                                                                                                        |
| :-------------------- | :-------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `diff:dismiss`        | Escape    | Tutup penampil diff; dari tampilan detail, kembali ke daftar file sebagai gantinya                                                                               |
| `diff:previousSource` | Left      | Sumber diff sebelumnya                                                                                                                                           |
| `diff:nextSource`     | Right     | Sumber diff berikutnya                                                                                                                                           |
| `diff:previousFile`   | Up, K     | File sebelumnya dalam daftar file; gulir ke atas satu baris dalam tampilan detail                                                                                |
| `diff:nextFile`       | Down, J   | File berikutnya dalam daftar file; gulir ke bawah satu baris dalam tampilan detail                                                                               |
| `diff:viewDetails`    | Enter     | Lihat detail diff                                                                                                                                                |
| `diff:back`           | (unbound) | Kembali dalam penampil diff. Escape melakukan tindakan kembali melalui `diff:dismiss`. Default sebelumnya dari Left dalam tampilan detail dihapus dalam v2.1.203 |

Tampilan detail diff juga mengikat kunci gaya pager ke [tindakan scroll](#scroll-actions) standar. Pengikatan ini adalah bagian dari konteks `DiffDialog` dan hanya berlaku dalam tampilan detail; default konteks `Scroll` yang tercantum di bawah [Tindakan scroll](#scroll-actions) tidak berubah.

| Tindakan              | Default        | Deskripsi                        |
| :-------------------- | :------------- | :------------------------------- |
| `scroll:pageUp`       | PageUp         | Gulir ke atas setengah viewport  |
| `scroll:pageDown`     | PageDown       | Gulir ke bawah setengah viewport |
| `scroll:fullPageUp`   | Shift+Space, B | Gulir ke atas viewport penuh     |
| `scroll:fullPageDown` | Space          | Gulir ke bawah viewport penuh    |
| `scroll:top`          | G, Home        | Lompat ke atas                   |
| `scroll:bottom`       | Shift+G, End   | Lompat ke bawah                  |

<h3 id="model-picker-actions">
  Tindakan pemilih model
</h3>

Tindakan yang tersedia dalam konteks `ModelPicker`:

| Tindakan                      | Default | Deskripsi                                    |
| :---------------------------- | :------ | :------------------------------------------- |
| `modelPicker:decreaseEffort`  | Left    | Kurangi tingkat upaya                        |
| `modelPicker:increaseEffort`  | Right   | Tingkatkan tingkat upaya                     |
| `modelPicker:thisSessionOnly` | s       | Terapkan model yang disorot ke sesi ini saja |

<h3 id="select-actions">
  Tindakan pilih
</h3>

Tindakan yang tersedia dalam konteks `Select`:

| Tindakan          | Default         | Deskripsi        |
| :---------------- | :-------------- | :--------------- |
| `select:next`     | Down, J, Ctrl+N | Opsi berikutnya  |
| `select:previous` | Up, K, Ctrl+P   | Opsi sebelumnya  |
| `select:accept`   | Enter           | Terima pilihan   |
| `select:cancel`   | Escape          | Batalkan pilihan |

<h3 id="plugin-actions">
  Tindakan plugin
</h3>

Tindakan yang tersedia dalam konteks `Plugin`:

| Tindakan          | Default | Deskripsi                                                                                        |
| :---------------- | :------ | :----------------------------------------------------------------------------------------------- |
| `plugin:toggle`   | Space   | Alihkan pemilihan plugin                                                                         |
| `plugin:install`  | I       | Instal plugin yang dipilih                                                                       |
| `plugin:favorite` | F       | Tandai plugin yang dipilih sebagai favorit sehingga diurutkan di dekat bagian atas tab Installed |

<h3 id="settings-actions">
  Tindakan pengaturan
</h3>

Tindakan yang tersedia dalam konteks `Settings`. Tindakan `select:accept` dan `confirm:no` digunakan kembali dari konteks [Select](#select-actions) dan [Confirmation](#confirmation-actions) dengan perilaku khusus Settings: perubahan diterapkan ke setiap pengaturan segera setelah Anda mengubahnya, jadi Escape menutup panel dengan perubahan Anda yang sudah disimpan daripada menolak.

| Tindakan          | Default      | Deskripsi                                              |
| :---------------- | :----------- | :----------------------------------------------------- |
| `settings:search` | /            | Masuk mode pencarian                                   |
| `settings:retry`  | R            | Coba muat ulang data penggunaan saat terjadi kesalahan |
| `select:accept`   | Enter, Space | Ubah pengaturan yang dipilih atau buka submenu-nya     |
| `confirm:no`      | Escape       | Tutup panel. Perubahan sudah disimpan                  |

<h3 id="voice-actions">
  Tindakan suara
</h3>

Tindakan yang tersedia dalam konteks `Chat` ketika [dikte suara](/docs/id/voice-dictation) diaktifkan:

| Tindakan           | Default | Deskripsi                                      |
| :----------------- | :------ | :--------------------------------------------- |
| `voice:pushToTalk` | Space   | Tahan atau ketuk tergantung pada mode `/voice` |

<h3 id="scroll-actions">
  Tindakan scroll
</h3>

Tindakan yang tersedia dalam konteks `Scroll` ketika [rendering fullscreen](/docs/id/fullscreen) diaktifkan:

| Tindakan                    | Default              | Deskripsi                                                                                                             |
| :-------------------------- | :------------------- | :-------------------------------------------------------------------------------------------------------------------- |
| `scroll:lineUp`             | (unbound)            | Gulir ke atas satu baris. Pengguliran roda mouse memicu tindakan ini                                                  |
| `scroll:lineDown`           | (unbound)            | Gulir ke bawah satu baris. Pengguliran roda mouse memicu tindakan ini                                                 |
| `scroll:pageUp`             | PageUp               | Gulir ke atas setengah tinggi viewport                                                                                |
| `scroll:pageDown`           | PageDown             | Gulir ke bawah setengah tinggi viewport                                                                               |
| `scroll:top`                | Ctrl+Home            | Lompat ke awal percakapan                                                                                             |
| `scroll:bottom`             | Ctrl+End             | Lompat ke pesan terbaru dan aktifkan kembali auto-follow                                                              |
| `scroll:halfPageUp`         | (unbound)            | Gulir ke atas setengah tinggi viewport. Perilaku yang sama dengan `scroll:pageUp`, disediakan untuk rebind gaya vi    |
| `scroll:halfPageDown`       | (unbound)            | Gulir ke bawah setengah tinggi viewport. Perilaku yang sama dengan `scroll:pageDown`, disediakan untuk rebind gaya vi |
| `scroll:fullPageUp`         | (unbound)            | Gulir ke atas tinggi viewport penuh                                                                                   |
| `scroll:fullPageDown`       | (unbound)            | Gulir ke bawah tinggi viewport penuh                                                                                  |
| `selection:copy`            | Ctrl+Shift+C / Cmd+C | Salin teks yang dipilih ke clipboard                                                                                  |
| `selection:clear`           | (unbound)            | Hapus pemilihan teks aktif                                                                                            |
| `selection:extendLeft`      | Shift+Left           | Perluas pemilihan aktif satu kolom ke kiri                                                                            |
| `selection:extendRight`     | Shift+Right          | Perluas pemilihan aktif satu kolom ke kanan                                                                           |
| `selection:extendUp`        | Shift+Up             | Perluas pemilihan aktif satu baris ke atas. Menggulir viewport ketika pemilihan mencapai tepi atas                    |
| `selection:extendDown`      | Shift+Down           | Perluas pemilihan aktif satu baris ke bawah. Menggulir viewport ketika pemilihan mencapai tepi bawah                  |
| `selection:extendLineStart` | Shift+Home           | Perluas pemilihan aktif ke awal baris                                                                                 |
| `selection:extendLineEnd`   | Shift+End            | Perluas pemilihan aktif ke akhir baris                                                                                |

<h2 id="keystroke-syntax">
  Sintaks keystroke
</h2>

<h3 id="modifiers">
  Pengubah
</h3>

Gunakan tombol pengubah dengan pemisah `+`:

* `ctrl` atau `control` - Tombol Control
* `shift` - Tombol Shift
* `alt`, `opt`, `option`, atau `meta` - Tombol Alt pada Windows dan Linux, tombol Option pada macOS
* `cmd`, `command`, `super`, atau `win` - Tombol Command pada macOS, tombol Windows pada Windows, tombol Super pada Linux

Grup `cmd` hanya terdeteksi di terminal yang melaporkan pengubah Super, seperti yang mendukung protokol keyboard Kitty atau mode `modifyOtherKeys` xterm. Sebagian besar terminal tidak mengirimnya, jadi gunakan `ctrl` atau `meta` untuk binding yang ingin Anda gunakan di mana saja.

Sebagai contoh:

```text theme={null}
ctrl+k          Ctrl + K
shift+tab       Shift + Tab
meta+p          Option + P pada macOS, Alt + P di tempat lain
ctrl+shift+c    Pengubah ganda
```

<h3 id="uppercase-letters">
  Huruf besar
</h3>

Huruf besar yang berdiri sendiri menyiratkan Shift. Sebagai contoh, `K` setara dengan `shift+k`. Ini berguna untuk binding gaya vim di mana kunci huruf besar dan kecil memiliki arti berbeda.

Huruf besar dengan pengubah (misalnya, `ctrl+K`) diperlakukan sebagai gaya dan **tidak** menyiratkan Shift: `ctrl+K` sama dengan `ctrl+k`.

<h3 id="chords">
  Chord
</h3>

Chord adalah urutan keystroke yang dipisahkan oleh spasi:

```text theme={null}
ctrl+k ctrl+s   Tekan Ctrl+K, lepaskan, lalu Ctrl+S
```

<h3 id="special-keys">
  Tombol khusus
</h3>

* `escape` atau `esc` - Tombol Escape
* `enter` atau `return` - Tombol Enter
* `tab` - Tombol Tab
* `space` - Bilah spasi
* `up`, `down`, `left`, `right` - Tombol panah
* `backspace`, `delete` - Tombol hapus

<h2 id="unbind-default-shortcuts">
  Batalkan pintasan default
</h2>

Atur tindakan ke `null` untuk membatalkan ikatan pintasan default:

```json theme={null}
{
  "bindings": [
    {
      "context": "Chat",
      "bindings": {
        "ctrl+s": null
      }
    }
  ]
}
```

Ini juga berfungsi untuk binding chord. Membatalkan setiap chord yang berbagi awalan membebaskan awalan itu untuk digunakan sebagai binding tombol tunggal. Sebuah chord dalam konteks aktif apa pun menjaga awalannya tetap terlindungi, jadi Anda harus membatalkan setiap chord dalam konteks yang mendefinisikannya.

Keluarga `Ctrl+X` default mencakup dua konteks: `ctrl+x ctrl+k` dan `ctrl+x ctrl+e` di `Chat`, dan `ctrl+x ctrl+b` di `Task`. Untuk mengklaim kembali `ctrl+x` itu sendiri sebagai binding tombol tunggal, batalkan semuanya:

```json theme={null}
{
  "bindings": [
    {
      "context": "Task",
      "bindings": {
        "ctrl+x ctrl+b": null
      }
    },
    {
      "context": "Chat",
      "bindings": {
        "ctrl+x ctrl+k": null,
        "ctrl+x ctrl+e": null,
        "ctrl+x": "chat:newline"
      }
    }
  ]
}
```

Jika Anda membatalkan beberapa tetapi tidak semua chord pada awalan, menekan awalan masih memasuki mode chord-wait untuk binding yang tersisa.

<h2 id="reserved-shortcuts">
  Pintasan yang dicadangkan
</h2>

Pintasan ini tidak dapat diikat ulang:

| Pintasan  | Alasan                                                  |
| :-------- | :------------------------------------------------------ |
| Ctrl+C    | Interrupt/cancel yang dikodekan keras                   |
| Ctrl+D    | Exit yang dikodekan keras                               |
| Ctrl+M    | Identik dengan Enter di terminal (keduanya mengirim CR) |
| Caps Lock | Tidak dikirimkan ke aplikasi terminal                   |

<h2 id="terminal-conflicts">
  Konflik terminal
</h2>

Beberapa pintasan mungkin bertentangan dengan multiplexer terminal:

| Pintasan | Konflik                                     |
| :------- | :------------------------------------------ |
| Ctrl+B   | Awalan tmux (tekan dua kali untuk mengirim) |
| Ctrl+A   | Awalan GNU screen                           |
| Ctrl+Z   | Suspend proses Unix (SIGTSTP)               |

<h2 id="vim-mode-interaction">
  Interaksi mode vim
</h2>

Ketika mode vim diaktifkan melalui `/config` → Editor mode, keybindings dan mode vim beroperasi secara independen:

* **Mode vim** menangani input pada tingkat input teks (gerakan kursor, mode, motions)
* **Keybindings** menangani tindakan pada tingkat komponen (alihkan todos, kirim, dll.)
* Tombol Escape dalam mode vim beralih dari INSERT ke mode NORMAL; itu tidak memicu `chat:cancel`
* Sebagian besar pintasan Ctrl+key melewati mode vim ke sistem keybinding
* Kunci vim tidak dapat dipetakan ulang melalui file keybindings. Untuk memetakan urutan mode INSERT dua kunci seperti `jj` ke Escape, gunakan pengaturan [`vimInsertModeRemaps`](/docs/id/interactive-mode#remap-insert-mode-key-sequences)
* Dalam mode NORMAL vim, `?` menampilkan menu bantuan (perilaku vim)
* Dalam mode NORMAL vim, `/` membuka pencarian riwayat, sama seperti Ctrl+R dalam mode standar

<h2 id="validation">
  Validasi
</h2>

Claude Code memvalidasi pintasan keyboard Anda dan menampilkan peringatan untuk:

* Parse errors (JSON atau struktur tidak valid)
* Nama konteks tidak valid
* Konflik pintasan yang dicadangkan
* Konflik multiplexer terminal
* Binding duplikat dalam konteks yang sama

Claude Code melaporkan peringatan ketika file dimuat dan menulis masing-masing ke log debug. Mulai Claude Code dengan [`--debug`](/docs/id/cli-reference#cli-flags) untuk melihat detailnya.
