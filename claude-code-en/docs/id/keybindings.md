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
| `Footer`          | Navigasi indikator footer (tugas, tim, diff, artefak)           |
| `MessageSelector` | Pemilihan pesan dialog rewind dan ringkasan                     |
| `DiffDialog`      | Navigasi penampil diff                                          |
| `DiffPanel`       | [Panel diff](/docs/id/interactive-mode#diff-panel) terbuka           |
| `ModelPicker`     | Tingkat upaya pemilih model                                     |
| `EffortSlider`    | Effort slider dibuka oleh `/effort`                             |
| `Select`          | Komponen select/list generik                                    |
| `Plugin`          | Dialog plugin (jelajahi, temukan, kelola)                       |
| `Agents`          | [Tampilan Agent](/docs/id/agent-view) (`claude agents`)              |
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
| `app:exit`             | Ctrl+D    | Keluar dari Claude Code. Tekan dua kali dalam 800ms untuk mengonfirmasi                              |
| `app:redraw`           | (unbound) | Paksa redraw terminal                                                                                |
| `app:toggleTodos`      | Ctrl+T    | Alihkan visibilitas daftar tugas Claude. Ini bukan tampilan background-task [`/tasks`](/docs/id/commands) |
| `app:toggleTranscript` | Ctrl+O    | Alihkan transcript verbose                                                                           |

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

| Tindakan              | Default                           | Deskripsi                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| :-------------------- | :-------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `chat:cancel`         | Escape                            | Batalkan input saat ini                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `chat:clearInput`     | Ctrl+L                            | Paksa redraw layar penuh, mempertahankan input dan percakapan                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `chat:clearScreen`    | Cmd+K                             | Sama dengan `chat:clearInput`. Lihat [Bersihkan percakapan](/docs/id/fullscreen#clear-the-conversation) untuk cara Cmd+K berperilaku di iTerm2 dan Terminal.app                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `chat:killAgents`     | Ctrl+X Ctrl+K                     | Hentikan semua [subagent latar belakang](/docs/id/sub-agents#run-subagents-in-foreground-or-background) yang berjalan dalam sesi ini dan matikan [auto-replies artefak](/docs/id/artifacts#let-claude-reply-to-comments-on-its-own) untuk sisanya                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `chat:cycleMode`      | Shift+Tab\*                       | Mode izin siklus                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `chat:modelPicker`    | Meta+P                            | Buka pemilih model                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `chat:fastMode`       | Meta+O                            | Alihkan mode cepat                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `chat:thinkingToggle` | Meta+T                            | Alihkan pemikiran yang diperluas                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `chat:submit`         | Enter                             | Kirim pesan                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `chat:queueSubmit`    | Ctrl+X Enter                      | Kirim pesan, ditandai untuk menunggu giliran: saat Claude sedang bekerja, Claude Code [mengantrekannya](/docs/id/interactive-mode#queue-messages-while-claude-works) dan tidak pernah mengganggu giliran. Tidak seperti `chat:submit`, ini mengirimkan draft bahkan saat saran autocomplete disorot. Memerlukan v2.1.247 atau lebih baru                                                                                                                                                                                                                                                                                                                                                          |
| `chat:sendNow`        | Ctrl+Enter, Ctrl+X Ctrl+S         | Kirim [pesan antrian Anda](/docs/id/interactive-mode#queue-messages-while-claude-works), dan draft Anda bersama mereka, segera. [Ketika Claude Code mengirim apa yang Anda antrekan](/docs/id/interactive-mode#when-claude-code-sends-what-you-queued) mencakup apa yang terjadi pada giliran Claude yang sedang dikerjakan. Ketika tidak ada yang berjalan, kunci mengirimkan draft, dan dalam [shell mode](/docs/id/interactive-mode#shell-mode-with-prefix) hanya mengantrekan perintah. Terminal yang tidak melaporkan kunci yang diperluas mengirimkan `Ctrl+Enter` sebagai `Enter` biasa, jadi `Ctrl+X Ctrl+S` adalah binding yang berfungsi di terminal apa pun. Memerlukan v2.1.275 atau lebih baru |
| `chat:newline`        | Ctrl+J                            | Sisipkan baris baru tanpa mengirim                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `chat:undo`           | Ctrl+\_, Ctrl+Shift+-             | Batalkan tindakan terakhir                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `chat:externalEditor` | Ctrl+G, Ctrl+X Ctrl+E             | Buka di editor eksternal. [Dispatch input tampilan agen](/docs/id/agent-view#keyboard-shortcuts) juga mengikuti binding keystroke tunggal tindakan ini                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `chat:stash`          | Ctrl+S                            | Simpan prompt saat ini                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `chat:imagePaste`     | Ctrl+V (Alt+V di Windows dan WSL) | Tempel gambar dari clipboard. Di WSL, kedua pintasan terikat secara default                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |

\*Di Windows tanpa mode VT (Node \<24.2.0/\<22.17.0, Bun \<1.2.23), default ke Meta+M.

<h3 id="autocomplete-actions">
  Tindakan autocomplete
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

| Tindakan                | Default     | Deskripsi                                                                                                                                                                                                                                                            |
| :---------------------- | :---------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `confirm:yes`           | Enter       | Konfirmasi tindakan                                                                                                                                                                                                                                                  |
| `confirm:no`            | Escape      | Tolak tindakan                                                                                                                                                                                                                                                       |
| `confirm:previous`      | Up          | Opsi sebelumnya                                                                                                                                                                                                                                                      |
| `confirm:next`          | Down        | Opsi berikutnya                                                                                                                                                                                                                                                      |
| `confirm:nextField`     | Tab         | Bidang berikutnya                                                                                                                                                                                                                                                    |
| `confirm:previousField` | (unbound)   | Bidang sebelumnya                                                                                                                                                                                                                                                    |
| `confirm:toggle`        | Space       | Alihkan pilihan                                                                                                                                                                                                                                                      |
| `confirm:cycleMode`     | Shift+Tab\* | Mode izin siklus. Pada prompt izin file, menutup [bidang komentar](/docs/id/permissions#add-a-comment-when-you-answer-a-permission-prompt) yang terbuka; tanpa bidang terbuka, memilih opsi yang memungkinkan tindakan untuk sisa sesi, ketika prompt menawarkan opsi itu |

\*Di Windows tanpa mode VT (Node \<24.2.0/\<22.17.0, Bun \<1.2.23), default ke Meta+M.

Sebelum v2.1.257, tindakan `confirm:toggleExplanation`, terikat ke `Ctrl+E` secara default, menampilkan penjelasan perintah yang dihasilkan model pada prompt izin Bash dan PowerShell.

Dialog menggunakan `confirm:yes` dan `confirm:no` untuk menerima dan membatalkan bahkan ketika mereka tidak mengajukan pertanyaan ya-atau-tidak. Jika Anda mengikat huruf telanjang seperti `y` atau `n` dalam konteks ini, huruf juga bertindak pada dialog yang tidak pernah menampilkannya sebagai kunci. Dialog yang menampilkan `y` dan `n` sebagai kuncinya membaca huruf-huruf itu sendiri dan tidak memerlukan binding.

Contoh ini mengikat `y` ke `confirm:yes` dan `n` ke `confirm:no`:

```json theme={null}
{
  "bindings": [
    {
      "context": "Confirmation",
      "bindings": {
        "y": "confirm:yes",
        "n": "confirm:no"
      }
    }
  ]
}
```

Dengan binding ini, `y` dan `n` masih mengetik sebagai huruf saat [bidang teks](#text-fields) memiliki fokus.

Sebelum v2.1.280, `y` juga terikat ke `confirm:yes` dan `n` ke `confirm:no` secara default. Jika Anda membuat `keybindings.json` Anda dengan `/keybindings` sebelum v2.1.280, file mencantumkan kedua binding dan mereka tetap berlaku sampai Anda menghapus dua baris itu.

<h3 id="permission-actions">
  Tindakan izin
</h3>

Tindakan yang tersedia dalam konteks `Confirmation` untuk dialog izin:

| Tindakan                 | Default   | Deskripsi                                                                                                 |
| :----------------------- | :-------- | :-------------------------------------------------------------------------------------------------------- |
| `permission:toggleDebug` | (unbound) | Alihkan info debug izin. Default sebelumnya dari Ctrl+D dihapus di v2.1.146 karena mengaburkan `app:exit` |

<h3 id="transcript-actions">
  Tindakan transcript
</h3>

Tindakan yang tersedia dalam konteks `Transcript`:

| Tindakan                   | Default           | Deskripsi                       |
| :------------------------- | :---------------- | :------------------------------ |
| `transcript:toggleShowAll` | Ctrl+E            | Alihkan tampilkan semua konten  |
| `transcript:exit`          | q, Ctrl+C, Escape | Keluar dari tampilan transcript |

`transcript:toggleShowAll` berlaku dalam renderer klasik saja; dalam [rendering fullscreen](/docs/id/fullscreen), penampil transcript tidak menawarkan toggle tampilkan-semua.

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
| `historySearch:cycleScope` | Ctrl+S      | Cakupan siklus: sesi, proyek, di mana-mana |

Default `historySearch:next`, `historySearch:accept`, `historySearch:cancel`, dan `historySearch:execute` berlaku untuk pencarian riwayat inline dalam renderer klasik, yang selalu mencari prompt dari semua proyek. `historySearch:cycleScope` hanya berlaku dalam [rendering fullscreen](/docs/id/fullscreen), di mana `Ctrl+R` membuka dialog pencarian sebagai gantinya dan `Ctrl+S` mengubah cakupannya. Kunci lain dialog sudah diperbaiki dan tidak dapat diikat ulang: `Enter` atau `Tab` menempatkan kecocokan yang disorot dalam input prompt dan `Esc` membatalkan.

<h3 id="task-actions">
  Tindakan tugas
</h3>

Tindakan yang tersedia dalam konteks `Task`:

| Tindakan          | Default               | Deskripsi                                                                          |
| :---------------- | :-------------------- | :--------------------------------------------------------------------------------- |
| `task:background` | Ctrl+B, Ctrl+X Ctrl+B | Tugas latar belakang saat ini. Chord Ctrl+X Ctrl+B menghindari konflik awalan tmux |

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

| Tindakan                | Default           | Deskripsi                                                                                                                                                                                                              |
| :---------------------- | :---------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `footer:next`           | Right             | Item footer berikutnya                                                                                                                                                                                                 |
| `footer:previous`       | Left              | Item footer sebelumnya                                                                                                                                                                                                 |
| `footer:up`             | Up                | Navigasi ke atas di footer (batalkan pilihan di atas)                                                                                                                                                                  |
| `footer:down`           | Down              | Navigasi ke bawah di footer                                                                                                                                                                                            |
| `footer:openSelected`   | Enter             | Buka item footer yang dipilih                                                                                                                                                                                          |
| `footer:clearSelection` | Escape            | Bersihkan pilihan footer                                                                                                                                                                                               |
| `footer:dismiss`        | Backspace, Delete | Tutup tautan [artefak](/docs/id/artifacts) yang dipilih dari footer; artefak yang dipublikasikan itu sendiri tidak terpengaruh. Pada baris footer lainnya, kunci ini tidak berpengaruh. Memerlukan v2.1.217 atau lebih baru |

Saat item footer dipilih, seperti baris di panel agen di bawah prompt, `Enter` membukanya bahkan ketika Anda mengikat ulang `Enter` dalam konteks `Chat` ke `chat:queueSubmit` atau `chat:newline`.

Binding `Chat` pada kunci yang tidak diikat konteks `Footer`, seperti `Shift+Tab` untuk `chat:cycleMode`, terus bekerja saat item dipilih.

<h3 id="message-selector-actions">
  Tindakan pemilih pesan
</h3>

Tindakan yang tersedia dalam konteks `MessageSelector`:

| Tindakan                 | Default                                   | Deskripsi                    |
| :----------------------- | :---------------------------------------- | :--------------------------- |
| `messageSelector:up`     | Up, K, Ctrl+P                             | Pindah ke atas dalam daftar  |
| `messageSelector:down`   | Down, J, Ctrl+N                           | Pindah ke bawah dalam daftar |
| `messageSelector:top`    | Ctrl+Up, Shift+Up, Meta+Up, Shift+K       | Lompat ke atas               |
| `messageSelector:bottom` | Ctrl+Down, Shift+Down, Meta+Down, Shift+J | Lompat ke bawah              |
| `messageSelector:select` | Enter                                     | Pilih pesan                  |

<h3 id="diff-actions">
  Tindakan diff
</h3>

Tindakan yang tersedia dalam konteks `DiffDialog`:

| Tindakan              | Default   | Deskripsi                                                                                                                                                     |
| :-------------------- | :-------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `diff:dismiss`        | Escape    | Tutup penampil diff; dari tampilan detail, kembali ke daftar file sebagai gantinya                                                                            |
| `diff:previousSource` | Left      | Sumber diff sebelumnya                                                                                                                                        |
| `diff:nextSource`     | Right     | Sumber diff berikutnya                                                                                                                                        |
| `diff:previousFile`   | Up, K     | File sebelumnya dalam daftar file; gulir ke atas satu baris dalam tampilan detail                                                                             |
| `diff:nextFile`       | Down, J   | File berikutnya dalam daftar file; gulir ke bawah satu baris dalam tampilan detail                                                                            |
| `diff:viewDetails`    | Enter     | Lihat detail diff                                                                                                                                             |
| `diff:back`           | (unbound) | Kembali dalam penampil diff. Escape melakukan tindakan kembali melalui `diff:dismiss`. Default sebelumnya dari Left dalam tampilan detail dihapus di v2.1.203 |

Tampilan detail diff juga mengikat kunci gaya pager ke [tindakan scroll](#scroll-actions) standar. Binding ini adalah bagian dari konteks `DiffDialog` dan hanya berlaku dalam tampilan detail; default konteks `Scroll` yang tercantum di bawah [Tindakan scroll](#scroll-actions) tidak berubah.

| Tindakan              | Default        | Deskripsi                        |
| :-------------------- | :------------- | :------------------------------- |
| `scroll:pageUp`       | PageUp         | Gulir ke atas setengah viewport  |
| `scroll:pageDown`     | PageDown       | Gulir ke bawah setengah viewport |
| `scroll:fullPageUp`   | Shift+Space, B | Gulir ke atas viewport penuh     |
| `scroll:fullPageDown` | Space          | Gulir ke bawah viewport penuh    |
| `scroll:top`          | G, Home        | Lompat ke atas                   |
| `scroll:bottom`       | Shift+G, End   | Lompat ke bawah                  |

<h3 id="diff-panel-actions">
  Tindakan panel diff
</h3>

Tindakan untuk [panel diff](/docs/id/interactive-mode#diff-panel) yang `/diff` buka dalam rendering fullscreen. `app:cycleDiffBase` berada dalam konteks `DiffPanel`, yang aktif saat panel terbuka; yang lain adalah `Global`. Panel memerlukan Claude Code v2.1.260 atau lebih baru.

| Tindakan                    | Default              | Deskripsi                                                                   |
| :-------------------------- | :------------------- | :-------------------------------------------------------------------------- |
| `app:toggleReplTab`         | (unbound)            | Buka atau tutup panel diff, sama seperti menjalankan `/diff`                |
| `app:cycleDiffBase`         | Ctrl+X B             | Ubah basis perbandingan panel: sesi ini, tidak berkomitmen, kemudian cabang |
| `app:diffFileListUp`        | Ctrl+Up, Meta+Up     | Gulir daftar file panel ke atas saat meluap                                 |
| `app:diffFileListDown`      | Ctrl+Down, Meta+Down | Gulir daftar file panel ke bawah saat meluap                                |
| `app:toggleDiffNoiseFilter` | (unbound)            | Tampilkan atau sembunyikan file uji dan yang dihasilkan dalam panel         |
| `app:toggleDiffPreSession`  | (unbound)            | Perluas atau ciutkan perubahan dari sebelum sesi ini                        |

<h3 id="model-picker-actions">
  Tindakan pemilih model
</h3>

Tindakan yang tersedia dalam konteks `ModelPicker`:

| Tindakan                      | Default | Deskripsi                                    |
| :---------------------------- | :------ | :------------------------------------------- |
| `modelPicker:decreaseEffort`  | Left    | Kurangi tingkat upaya                        |
| `modelPicker:increaseEffort`  | Right   | Tingkatkan tingkat upaya                     |
| `modelPicker:thisSessionOnly` | s       | Terapkan model yang disorot ke sesi ini saja |

<h3 id="effort-slider-actions">
  Tindakan slider upaya
</h3>

Tindakan yang tersedia dalam konteks `EffortSlider`, slider yang terbuka saat Anda menjalankan `/effort` tanpa argumen. Kunci Left, Right, Enter, dan Escape slider tidak dapat diikat ulang.

| Tindakan                       | Default | Deskripsi                                                                                                                            |
| :----------------------------- | :------ | :----------------------------------------------------------------------------------------------------------------------------------- |
| `effortSlider:thisSessionOnly` | s       | Terapkan [tingkat upaya](/docs/id/model-config#adjust-effort-level) yang difokuskan ke sesi ini saja. Memerlukan v2.1.257 atau lebih baru |

<h3 id="select-actions">
  Tindakan pilih
</h3>

Tindakan yang tersedia dalam konteks `Select`:

| Tindakan          | Default         | Deskripsi                         |
| :---------------- | :-------------- | :-------------------------------- |
| `select:next`     | Down, J, Ctrl+N | Opsi berikutnya                   |
| `select:previous` | Up, K, Ctrl+P   | Opsi sebelumnya                   |
| `select:pageUp`   | PageUp          | Pindah ke atas satu halaman opsi  |
| `select:pageDown` | PageDown        | Pindah ke bawah satu halaman opsi |
| `select:first`    | Home            | Opsi pertama                      |
| `select:last`     | End             | Opsi terakhir                     |
| `select:accept`   | Enter           | Terima pilihan                    |
| `select:cancel`   | Escape          | Batalkan pilihan                  |

Claude Code menerapkan binding `select:pageUp`, `select:pageDown`, `select:first`, dan `select:last` Anda dalam menu `/skills`. Dalam sebagian besar daftar lainnya, seperti pemilih `/model`, binding `select:first` dan `select:last` Anda berlaku. PageUp dan PageDown halaman melalui opsi dalam daftar itu terlepas dari binding Anda.

Sebelum v2.1.280, daftar lain itu mengabaikan Home, End, dan binding `select:first` dan `select:last` Anda.

<h3 id="plugin-actions">
  Tindakan plugin
</h3>

Tindakan yang tersedia dalam konteks `Plugin`:

| Tindakan          | Default | Deskripsi                                                                    |
| :---------------- | :------ | :--------------------------------------------------------------------------- |
| `plugin:toggle`   | Space   | Alihkan pilihan plugin                                                       |
| `plugin:install`  | I       | Instal plugin yang dipilih                                                   |
| `plugin:favorite` | F       | Favoritkan plugin yang dipilih sehingga disortir di dekat atas tab Installed |

<h3 id="settings-actions">
  Tindakan pengaturan
</h3>

Tindakan yang tersedia dalam konteks `Settings`. Tindakan `select:accept` dan `confirm:no` digunakan kembali dari konteks [Select](#select-actions) dan [Confirmation](#confirmation-actions) dengan perilaku spesifik Settings: perubahan berlaku untuk setiap pengaturan segera setelah Anda mengubahnya, jadi Escape menutup panel dengan perubahan Anda disimpan daripada menolak.

| Tindakan          | Default      | Deskripsi                                              |
| :---------------- | :----------- | :----------------------------------------------------- |
| `settings:search` | /            | Masuk mode pencarian                                   |
| `settings:retry`  | R            | Coba muat ulang data penggunaan saat terjadi kesalahan |
| `select:accept`   | Enter, Space | Ubah pengaturan yang dipilih atau buka submenu-nya     |
| `confirm:no`      | Escape       | Tutup panel. Perubahan sudah disimpan                  |

<h3 id="agents-actions">
  Tindakan agen
</h3>

Tindakan yang tersedia dalam konteks `Agents`, yang berlaku dalam [tampilan agen](/docs/id/agent-view), dibuka dengan `claude agents`. Memerlukan v2.1.257 atau lebih baru.

| Tindakan            | Default | Deskripsi                                                                                 |
| :------------------ | :------ | :---------------------------------------------------------------------------------------- |
| `agents:switchView` | Ctrl+S  | Alihkan [pengelompokan sesi](/docs/id/agent-view#organize-the-list) antara state dan direktori |
| `agents:togglePin`  | Ctrl+T  | [Pin atau unpin](/docs/id/agent-view#organize-the-list) sesi yang dipilih                      |

Saat tampilan agen terbuka, Claude Code menggunakan binding `Agents` untuk kunci apa pun yang diikat konteks `Agents`, dan mengabaikan binding `Chat` atau `Global` pada kunci yang sama. Misalnya, menekan Ctrl+S dalam tampilan agen mengalihkan pengelompokan sesi daripada memicu default `chat:stash`.

Pintasan editor eksternal dispatch input bukan tindakan `Agents`. Tampilan agen mengikuti binding `chat:externalEditor` konteks `Chat`, Ctrl+G secara default.

Binding api pada keystroke tunggal dalam tampilan agen, jadi chord Ctrl+X Ctrl+E yang terikat ke `chat:externalEditor` tidak membuka editor di sana.

<h3 id="voice-actions">
  Tindakan suara
</h3>

Tindakan yang tersedia dalam konteks `Chat` saat [dikte suara](/docs/id/voice-dictation) diaktifkan:

| Tindakan           | Default | Deskripsi                                               |
| :----------------- | :------ | :------------------------------------------------------ |
| `voice:pushToTalk` | Space   | Dikte prompt. Tahan atau ketuk tergantung mode `/voice` |

<h3 id="scroll-actions">
  Tindakan scroll
</h3>

Tindakan yang tersedia dalam konteks `Scroll` saat [rendering fullscreen](/docs/id/fullscreen) diaktifkan:

| Tindakan                    | Default              | Deskripsi                                                                                                             |
| :-------------------------- | :------------------- | :-------------------------------------------------------------------------------------------------------------------- |
| `scroll:lineUp`             | `wheelup`            | Gulir ke atas satu baris. Pengguliran roda mouse memicu tindakan ini                                                  |
| `scroll:lineDown`           | `wheeldown`          | Gulir ke bawah satu baris. Pengguliran roda mouse memicu tindakan ini                                                 |
| `scroll:pageUp`             | PageUp               | Gulir ke atas setengah tinggi viewport                                                                                |
| `scroll:pageDown`           | PageDown             | Gulir ke bawah setengah tinggi viewport                                                                               |
| `scroll:top`                | Ctrl+Home            | Lompat ke awal percakapan                                                                                             |
| `scroll:bottom`             | Ctrl+End             | Lompat ke pesan terbaru dan aktifkan kembali auto-follow                                                              |
| `scroll:halfPageUp`         | (unbound)            | Gulir ke atas setengah tinggi viewport. Perilaku yang sama dengan `scroll:pageUp`, disediakan untuk rebind gaya vi    |
| `scroll:halfPageDown`       | (unbound)            | Gulir ke bawah setengah tinggi viewport. Perilaku yang sama dengan `scroll:pageDown`, disediakan untuk rebind gaya vi |
| `scroll:fullPageUp`         | (unbound)            | Gulir ke atas tinggi viewport penuh                                                                                   |
| `scroll:fullPageDown`       | (unbound)            | Gulir ke bawah tinggi viewport penuh                                                                                  |
| `selection:copy`            | Ctrl+Shift+C / Cmd+C | Salin teks yang dipilih ke clipboard                                                                                  |
| `selection:clear`           | (unbound)            | Bersihkan pilihan teks aktif. Memerlukan v2.1.234 atau lebih baru                                                     |
| `selection:extendLeft`      | Shift+Left           | Perluas pilihan aktif satu kolom ke kiri                                                                              |
| `selection:extendRight`     | Shift+Right          | Perluas pilihan aktif satu kolom ke kanan                                                                             |
| `selection:extendUp`        | Shift+Up             | Perluas pilihan aktif satu baris ke atas. Menggulir viewport saat pilihan mencapai tepi atas                          |
| `selection:extendDown`      | Shift+Down           | Perluas pilihan aktif satu baris ke bawah. Menggulir viewport saat pilihan mencapai tepi bawah                        |
| `selection:extendLineStart` | Shift+Home           | Perluas pilihan aktif ke awal baris                                                                                   |
| `selection:extendLineEnd`   | Shift+End            | Perluas pilihan aktif ke akhir baris                                                                                  |

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

Claude Code mengurai nama kunci tanpa memperhatikan huruf besar/kecil, jadi `K` adalah binding yang sama dengan `k` dan `ctrl+K` sama dengan `ctrl+k`. Untuk mengikat Shift dan huruf, tulis `shift+k`.

<h3 id="non-us-keyboard-layouts">
  Tata letak keyboard non-US
</h3>

Tulis nama kunci dari pintasan Ctrl sebagai karakter Latin bahkan ketika tata letak keyboard aktif Anda mengetik karakter lain.

Bagaimana Claude Code mencocokkan kunci yang Anda tekan dengan binding tergantung pada jenis tata letak:

* Di bawah tata letak non-Latin seperti Cyrillic, Claude Code mencocokkan pintasan Ctrl berdasarkan posisi kunci tata letak US ketika terminal menggunakan protokol keyboard Kitty dan melaporkan posisi tersebut. Di terminal seperti itu, dengan tata letak Rusia aktif, menekan Ctrl dan tombol fisik W memicu `ctrl+w`. Di terminal yang tidak melaporkan posisi, Claude Code mencocokkan apa pun yang dikirim terminal untuk penekanan kunci: kode kontrol ASCII memicu pintasan Latin, dan penekanan kunci yang tiba sebagai karakter Cyrillic tidak cocok dengan binding apa pun
* Di bawah tata letak yang mengatur ulang huruf Latin, seperti AZERTY, Claude Code mencocokkan huruf yang diketik kunci, jadi menekan Ctrl dan kunci berlabel A memicu `ctrl+a`

Sebelum v2.1.247, menekan pintasan Ctrl di bawah tata letak non-Latin tidak memicu binding-nya di terminal yang menggunakan protokol keyboard Kitty, seperti Ghostty, Kitty, WezTerm, dan iTerm2.

<h3 id="chords">
  Chord
</h3>

Chord adalah urutan keystroke yang dipisahkan oleh spasi:

```text theme={null}
ctrl+k ctrl+s   Tekan Ctrl+K, lepaskan, lalu Ctrl+S
```

Tekan setiap keystroke dalam 3 detik setelah yang sebelumnya. Jika Anda menunggu lebih lama, Claude Code membatalkan chord dan menampilkan pemberitahuan singkat yang mengatakan demikian.

<h3 id="special-keys">
  Tombol khusus
</h3>

* `escape` atau `esc` - Tombol Escape
* `enter` atau `return` - Tombol Enter
* `tab` - Tombol Tab
* `space` - Bilah spasi
* `up`, `down`, `left`, `right` - Tombol panah
* `pageup`, `pagedown` - Tombol Page Up dan Page Down
* `home`, `end` - Tombol Home dan End
* `backspace`, `delete` - Tombol hapus
* `wheelup`, `wheeldown` - Peristiwa scroll roda mouse

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

Claude Code mengikat chord default ini pada awalan `ctrl+x`: `ctrl+x ctrl+k`, `ctrl+x ctrl+e`, `ctrl+x enter`, `ctrl+x ctrl+a`, `ctrl+x ctrl+s`, dan `ctrl+x tab` di `Chat`, `ctrl+x ctrl+b` di `Task`, dan `ctrl+x b` di `DiffPanel`. Chord `ctrl+x enter` memerlukan v2.1.247 atau lebih baru, `ctrl+x b`, `ctrl+x ctrl+a`, dan `ctrl+x tab` memerlukan v2.1.260 atau lebih baru, dan `ctrl+x ctrl+s` memerlukan v2.1.275 atau lebih baru.

Untuk mengklaim kembali `ctrl+x` itu sendiri sebagai binding tombol tunggal, batalkan semuanya:

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
      "context": "DiffPanel",
      "bindings": {
        "ctrl+x b": null
      }
    },
    {
      "context": "Chat",
      "bindings": {
        "ctrl+x ctrl+k": null,
        "ctrl+x ctrl+e": null,
        "ctrl+x enter": null,
        "ctrl+x ctrl+a": null,
        "ctrl+x ctrl+s": null,
        "ctrl+x tab": null,
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

| Pintasan  | Alasan                                                                                                                                                                                                                                                       |
| :-------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Ctrl+C    | Interrupt/cancel yang dikodekan keras                                                                                                                                                                                                                        |
| Ctrl+D    | Exit yang dikodekan keras                                                                                                                                                                                                                                    |
| Ctrl+M    | Claude Code selalu menerimanya sebagai Enter                                                                                                                                                                                                                 |
| Ctrl+\[   | Claude Code selalu menerimanya sebagai Escape. Di terminal yang menggunakan protokol keyboard Kitty, ini memerlukan v2.1.242 atau lebih baru                                                                                                                 |
| Ctrl+I    | Claude Code selalu menerimanya sebagai Tab                                                                                                                                                                                                                   |
| Ctrl+H    | Mengirimkan byte backspace ASCII. [Bagaimana Claude Code membacanya di Windows](/docs/id/terminal-config#fix-backspace-deleting-a-whole-word-on-windows) bergantung pada terminal Anda dan variabel lingkungan [`CLAUDE_CODE_BS_AS_CTRL_BACKSPACE`](/docs/id/env-vars) |
| Caps Lock | Tidak dikirimkan ke aplikasi terminal                                                                                                                                                                                                                        |

<h2 id="terminal-conflicts">
  Konflik terminal
</h2>

Beberapa pintasan mungkin bertentangan dengan multiplexer terminal:

| Pintasan | Konflik                                     |
| :------- | :------------------------------------------ |
| Ctrl+B   | Awalan tmux (tekan dua kali untuk mengirim) |
| Ctrl+A   | Awalan GNU screen                           |
| Ctrl+Z   | Suspend proses Unix (SIGTSTP)               |

<h2 id="text-fields">
  Bidang teks
</h2>

Jika Anda mengikat huruf, digit, atau Spasi yang telah ditentukan, Anda masih dapat mengetik karakter tersebut di bidang teks dalam dialog atau panel. Salah satu bidang tersebut adalah jawaban `Other` untuk pertanyaan yang diajukan Claude. Saat bidang memiliki fokus, tombol yang dapat dicetak yang Anda tekan tanpa Ctrl, Alt, atau Cmd akan masuk ke bidang, dan Claude Code tidak akan mencocokkannya dengan pintasan keyboard Anda.

Kunci-kunci ini masih menjalankan pintasan keyboard mereka saat bidang memiliki fokus:

* Kunci yang tidak mengetik karakter, seperti Enter, Escape, Tab, dan tombol panah
* Tombol apa pun yang ditekan dengan Ctrl, Alt, atau Cmd
* Keystroke kedua dari [chord](#chords) yang sudah sedang berlangsung

Pada prompt utama, Claude Code mencocokkan setiap kunci terhadap konteks aktif, seperti `Chat`, dan mengetik kunci hanya ketika tidak ada pintasan keyboard yang mengambilnya.

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
* Nilai action tidak valid, seperti action yang bukan string atau `null`
* Nama action yang tidak dikenal, seperti typo dari action yang terdaftar. Claude Code melewati binding dan mempertahankan binding default apa pun untuk kunci tersebut. Sebelum v2.1.246, binding dengan nama action yang tidak dikenal secara diam-diam menonaktifkan kunci tersebut
* Konflik pintasan yang dicadangkan
* Binding duplikat dalam konteks yang sama

Claude Code melaporkan peringatan ketika file dimuat dan menulis masing-masing ke log debug. Mulai Claude Code dengan [`--debug`](/docs/id/cli-reference#cli-flags) untuk melihat detailnya.
