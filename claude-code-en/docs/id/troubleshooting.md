> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Troubleshooting

> Perbaiki penggunaan CPU atau memori yang tinggi, hang, thrashing auto-compact, dan masalah pencarian di Claude Code, dan temukan halaman yang tepat untuk masalah lainnya.

Halaman ini mencakup masalah kinerja, stabilitas, dan pencarian setelah Claude Code berjalan. Untuk masalah lainnya, mulai dengan halaman yang sesuai dengan tempat Anda terjebak:

| Gejala                                                                                                                                                  | Buka                                                                                     |
| :------------------------------------------------------------------------------------------------------------------------------------------------------ | :--------------------------------------------------------------------------------------- |
| `command not found`, instalasi gagal, masalah PATH, `EACCES`, kesalahan TLS                                                                             | [Troubleshoot installation and login](/docs/id/troubleshoot-install)                          |
| Pembaruan atau instalasi unduhan gagal dengan `The connection dropped while downloading the update` atau `aborted`                                      | [Error reference](/docs/id/errors#the-connection-dropped-while-downloading-the-update)        |
| Loop login, kesalahan OAuth, `403 Forbidden`, "organization disabled", kredensial Amazon Bedrock, Google Cloud's Agent Platform, atau Microsoft Foundry | [Troubleshoot installation and login](/docs/id/troubleshoot-install#login-and-authentication) |
| Pengaturan tidak diterapkan, hooks tidak berfungsi, server MCP tidak dimuat                                                                             | [Debug your configuration](/docs/id/debug-your-config)                                        |
| Sesi dimulai dalam mode otomatis, atau Claude mengedit file dan menjalankan perintah tanpa bertanya                                                     | [Which mode a session starts in](/docs/id/permission-modes#which-mode-a-session-starts-in)    |
| `API Error: 5xx`, `529 Overloaded`, `429`, kesalahan validasi permintaan                                                                                | [Error reference](/docs/id/errors)                                                            |
| `model not found` atau `you may not have access to it`                                                                                                  | [Error reference](/docs/id/errors#theres-an-issue-with-the-selected-model)                    |
| Ekstensi VS Code tidak terhubung atau tidak mendeteksi Claude                                                                                           | [VS Code integration](/docs/id/vs-code#fix-common-issues)                                     |
| `Claude Code process exited with code 1` di VS Code atau aplikasi SDK                                                                                   | [Error reference](/docs/id/errors#claude-code-process-exited-with-code-n)                     |
| Plugin JetBrains atau IDE tidak terdeteksi                                                                                                              | [JetBrains integration](/docs/id/jetbrains#troubleshooting)                                   |
| CPU atau memori tinggi, respons lambat, hang, pencarian tidak menemukan file                                                                            | [Performance and stability](#performance-and-stability) di bawah                         |

Jika Anda tidak yakin mana yang berlaku, jalankan `/doctor` di dalam Claude Code untuk pemeriksaan otomatis instalasi, pengaturan, ekstensi, dan penggunaan konteks Anda; ini mengusulkan perbaikan yang dapat diterapkan setelah Anda mengonfirmasi. Jika `claude` tidak akan memulai sama sekali, jalankan `claude doctor` dari shell Anda sebagai gantinya. Jalankan `/mcp` untuk memeriksa status server MCP.

<h2 id="performance-and-stability">
  Kinerja dan stabilitas
</h2>

Bagian-bagian ini mencakup masalah yang terkait dengan penggunaan sumber daya, responsivitas, dan perilaku pencarian.

<h3 id="high-cpu-or-memory-usage">
  High CPU or memory usage
</h3>

Claude Code dirancang untuk bekerja dengan sebagian besar lingkungan pengembangan, tetapi dapat mengonsumsi sumber daya signifikan saat memproses codebase besar. Jika Anda mengalami masalah kinerja:

1. Gunakan `/compact` secara teratur untuk mengurangi ukuran konteks. Jika mengembalikan `Not enough messages to compact.`, percakapan memiliki terlalu sedikit putaran untuk diringkas; itu dapat terjadi bahkan dengan konteks penuh ketika satu paste besar mengisinya
2. Tutup dan mulai ulang Claude Code di antara tugas-tugas besar
3. Pertimbangkan menambahkan direktori build besar ke file `.gitignore` Anda
4. Mulai ulang dengan [`claude --safe-mode`](/docs/id/cli-reference#cli-flags) untuk memeriksa apakah plugin, server MCP, atau hook adalah sumbernya. Ini menonaktifkan semua kustomisasi untuk sesi; jika penggunaan turun, lihat [Debug your configuration](/docs/id/debug-your-config#test-against-a-clean-configuration) untuk menemukan yang mana

Jika penggunaan memori heap sesi melampaui 2.5GB, peringatan penggunaan memori kritis muncul. Untuk membebaskan memori, mulai ulang Claude Code dan jalankan [`claude --continue`](/docs/id/cli-reference#cli-flags) untuk melanjutkan percakapan dalam proses baru.

Di luar [fullscreen rendering](/docs/id/fullscreen), menjalankan `/compact` juga membebaskan memori. Peringatan hilang setelah penggunaan memori turun kembali di bawah 2.5GB.

Jika penggunaan memori tetap tinggi setelah langkah-langkah ini, jalankan `/heapdump` untuk menulis dua file ke `~/Desktop`: snapshot heap JavaScript bernama `<session-id>.heapsnapshot` dan rincian memori bernama `<session-id>-diagnostics.json`. Claude Code [menyembunyikan perintah dari menu perintah](/docs/id/commands#how-the-command-menu-matches-what-you-type); ketikkan secara lengkap. Di Linux tanpa folder Desktop, file ditulis ke direktori home Anda.

<Warning>
  File `.heapsnapshot` berisi setiap string dalam proses, termasuk percakapan lengkap Anda dan kredensial. Jangan lampirkan ke masalah publik atau bagikan.
</Warning>

Perintah juga mencetak ringkasan dalam percakapan, menampilkan resident set size, JS heap, array buffers, dan native memory yang tidak terhitung, ditambah indikator kebocoran apa pun yang terdeteksi, seperti tingkat pertumbuhan memori yang tinggi atau jumlah handle terbuka yang tidak biasa tinggi. Ringkasan mengatakan apakah sebagian besar memori berada di JS heap, yang ditangkap snapshot, atau di native memory, yang tidak.

Lakukan salah satu dari dua hal dengan output:

* **Laporkan**: buka [GitHub issue](https://github.com/anthropics/claude-code/issues) dan lampirkan hanya file `-diagnostics.json`, yang membawa statistik di balik ringkasan yang dicetak dan tidak ada konten percakapan atau kredensial
* **Selidiki sendiri**: jika ringkasan mengatakan sebagian besar memori adalah JS heap, buka file `.heapsnapshot` di Chrome DevTools di bawah Memory → Load dan urutkan berdasarkan retained size untuk melihat apa yang menahan memori

Jika ringkasan mengatakan sebagian besar memori adalah native, snapshot tidak dapat menampilkannya; sertakan indikator kebocoran ringkasan dalam laporan Anda sebagai gantinya.

<h3 id="large-tables-are-cut-off-in-the-terminal">
  Large tables are cut off in the terminal
</h3>

Tabel Markdown dengan lebih dari 200 baris merender 200 baris pertamanya diikuti dengan baris `… N more rows not shown`. Hanya tampilan yang dibatasi: tabel lengkap tetap dalam percakapan, dan [`/copy`](/docs/id/commands) menyalin setiap baris. Untuk tabel yang terlalu besar untuk dibaca di terminal, minta Claude untuk menulisnya ke file sebagai gantinya. Sebelum v2.1.208, Claude Code merender setiap baris, jadi melanjutkan sesi yang berisi tabel yang sangat besar dapat terhenti saat merender ulang.

<h3 id="auto-compaction-stops-with-a-thrashing-error">
  Auto-compaction stops with a thrashing error
</h3>

Jika Anda melihat `Autocompact is thrashing: the context refilled to the limit...`, automatic compaction berhasil tetapi file atau output alat segera mengisi ulang jendela konteks beberapa kali berturut-turut. Claude Code berhenti mencoba ulang untuk menghindari pemborosan panggilan API pada loop yang tidak membuat kemajuan.

Untuk pulih:

1. Minta Claude membaca file yang terlalu besar dalam potongan yang lebih kecil, seperti rentang baris tertentu atau fungsi, alih-alih seluruh file
2. Jalankan `/compact` dengan fokus yang menjatuhkan output besar, misalnya `/compact keep only the plan and the diff`
3. Pindahkan pekerjaan file besar ke [subagent](/docs/id/sub-agents) sehingga berjalan di jendela konteks terpisah
4. Jalankan `/clear` jika percakapan sebelumnya tidak lagi diperlukan

<h3 id="command-hangs-or-freezes">
  Command hangs or freezes
</h3>

Jika Claude Code tampak tidak responsif:

1. Tekan Ctrl+C untuk mencoba membatalkan operasi saat ini
2. Jika tidak responsif, Anda mungkin perlu menutup terminal dan memulai ulang

Memulai ulang tidak kehilangan percakapan Anda. Jalankan `claude --resume` di direktori yang sama untuk melanjutkan sesi.

<h3 id="garbled-or-corrupted-text-in-an-editor’s-integrated-terminal">
  Garbled or corrupted text in an editor's integrated terminal
</h3>

Jika karakter ditampilkan sebagai kotak, smear, atau glyph yang salah saat menjalankan Claude Code di terminal terintegrasi VS Code, Cursor, atau Devin Desktop, GPU renderer terminal kemungkinan adalah penyebabnya. Jalankan `/terminal-setup` di dalam Claude Code untuk mengatur `terminal.integrated.gpuAcceleration` ke `"off"`, atau atur secara manual di pengaturan editor Anda dan muat ulang jendela. Lihat [Terminal configuration](/docs/id/terminal-config) untuk pengaturan lain yang ditulis `/terminal-setup`.

<h3 id="mouse-wheel-scrolls-one-line-at-a-time-in-fullscreen-rendering">
  Mouse wheel scrolls one line at a time in fullscreen rendering
</h3>

Dalam [fullscreen rendering](/docs/id/fullscreen), Claude Code menggulir percakapan itu sendiri daripada membiarkannya ke terminal Anda. Jika setiap notch roda bergerak lebih sedikit baris daripada yang Anda inginkan, jalankan `/scroll-speed` untuk menaikkan jumlah baris per notch dan simpan, atau atur variabel lingkungan `CLAUDE_CODE_SCROLL_SPEED`, kecuali di terminal IDE JetBrains, di mana Claude Code menerapkan penanganan scroll sendiri dan tidak ada yang berlaku. Lihat [Mouse wheel scrolling](/docs/id/fullscreen#mouse-wheel-scrolling) untuk nilai yang masing-masing terima.

Untuk bergerak lebih cepat tanpa mengubah kecepatan, tekan `PgUp` dan `PgDn` untuk menggulir setengah layar sekaligus. Untuk mengembalikan scrolling ke scrollback native terminal Anda, jalankan `/tui default` untuk beralih ke renderer klasik.

<h3 id="clipboard-commands-such-as-pbcopy-fail-inside-the-sandbox">
  Clipboard commands such as `pbcopy` fail inside the sandbox
</h3>

Ketika [sandboxing](/docs/id/sandboxing) aktif, utilitas clipboard seperti `pbcopy`, `xclip`, dan `wl-copy` dapat gagal menjangkau clipboard sistem dari dalam perintah Bash yang di-sandbox, meninggalkan clipboard Anda tidak berubah setelah Claude menyalurkan teks ke mereka.

Untuk menempatkan output Claude di clipboard Anda, minta Claude untuk mencetak konten dalam responsnya, kemudian jalankan [`/copy`](/docs/id/commands). `/copy` menulis ke clipboard dari proses Claude Code itu sendiri daripada dari perintah yang di-sandbox, jadi sandboxing tidak membloknya. Ini dapat menyalin satu blok kode alih-alih seluruh respons, dan juga menulis apa yang disalinnya ke file dan mencetak jalurnya, yang memberi Anda fallback ketika penulisan clipboard tidak mencapai terminal Anda, misalnya melalui SSH.

Ketika Claude menyalurkan teks ke salah satu alat ini, menambahkan `pbcopy *`, `wl-copy *`, atau `xclip *` ke [`excludedCommands`](/docs/id/settings-reference#sandbox-excludedcommands) tidak mengeluarkan panggilan itu dari sandbox dengan sendirinya.

<h3 id="copied-text-doesn’t-reach-your-local-clipboard-over-ssh">
  Copied text doesn't reach your local clipboard over SSH
</h3>

Ketika Claude Code berjalan di mesin jarak jauh melalui SSH, Claude Code tidak dapat menjalankan alat clipboard di mesin lokal Anda. Di luar tmux, ketika Anda memilih teks dalam [fullscreen rendering](/docs/id/fullscreen) atau menjalankan `/copy`, Claude Code mengirimkan teks ke terminal Anda sebagai urutan escape OSC 52 sebagai gantinya. Terminal Anda memutuskan apakah akan menempatkannya di clipboard Anda. `/copy` melaporkan `Copied to clipboard` terlepas dari apakah teks tiba, dan di luar tmux pemberitahuan seleksi berbunyi `sent N chars via OSC 52`.

Beberapa terminal tidak bertindak atas OSC 52. iTerm2 mengabaikannya sampai Anda menyalakan **Settings > General > Selection > Applications in terminal may access clipboard**, dan macOS Terminal.app tidak mendukungnya.

Untuk mendapatkan teks tanpa OSC 52:

* Tahan kunci seleksi native terminal Anda sambil menyeret, kemudian salin dengan pintasan biasa terminal Anda, seperti `Cmd+C`. Kuncinya adalah `Fn` di Terminal.app dan `Option` di iTerm2. [Keep native text selection](/docs/id/fullscreen#keep-native-text-selection) mencantumkannya untuk terminal lain.
* Atur [`CLAUDE_CODE_DISABLE_MOUSE=1`](/docs/id/env-vars) di mesin jarak jauh sehingga terminal Anda menangani seleksi untuk seluruh sesi.

<h3 id="search-and-discovery-issues">
  Search and discovery issues
</h3>

Jika Search tool, `@file` mentions, custom agents, atau custom skills tidak menemukan file, binary `ripgrep` bundel mungkin tidak berjalan di sistem Anda. Instal paket `ripgrep` platform Anda dan beri tahu Claude Code untuk menggunakannya sebagai gantinya:

<Tabs>
  <Tab title="macOS">
    ```bash theme={null}
    brew install ripgrep
    ```
  </Tab>

  <Tab title="Ubuntu/Debian">
    ```bash theme={null}
    sudo apt install ripgrep
    ```
  </Tab>

  <Tab title="Alpine">
    ```bash theme={null}
    apk add ripgrep
    ```

    `ripgrep` berada di repositori komunitas Alpine. Jika `apk` melaporkan bahwa paket hilang, lihat [Alpine Linux setup](/docs/id/setup#alpine-linux-and-musl-based-distributions).
  </Tab>

  <Tab title="Arch">
    ```bash theme={null}
    pacman -S ripgrep
    ```
  </Tab>

  <Tab title="Windows">
    ```powershell theme={null}
    winget install BurntSushi.ripgrep.MSVC
    ```
  </Tab>
</Tabs>

Kemudian atur `USE_BUILTIN_RIPGREP` ke `0`, baik di [environment](/docs/id/env-vars) shell Anda atau di blok `env` dari [`settings.json`](/docs/id/settings-reference#all-settings) Anda:

```json theme={null}
{
  "env": {
    "USE_BUILTIN_RIPGREP": "0"
  }
}
```

Untuk mengonfirmasi bahwa perubahan berlaku, jalankan `claude doctor` di terminal Anda dan periksa bahwa baris Search menampilkan jalur ripgrep sistem Anda alih-alih `OK (bundled)`.

<h3 id="slow-or-incomplete-search-results-on-wsl">
  Slow or incomplete search results on WSL
</h3>

Penalti kinerja pembacaan disk saat [bekerja lintas filesystem di WSL](https://learn.microsoft.com/en-us/windows/wsl/filesystems) dapat menghasilkan kecocokan yang lebih sedikit dari yang diharapkan saat menggunakan Claude Code di WSL. Pencarian masih berfungsi, tetapi mengembalikan hasil lebih sedikit daripada di filesystem native.

<Note>
  `claude doctor` menunjukkan Search sebagai OK dalam kasus ini.
</Note>

**Solusi:**

1. **Kirimkan pencarian yang lebih spesifik**: kurangi jumlah file yang dicari dengan menentukan direktori atau jenis file: "Search for JWT validation logic in the auth-service package" atau "Find use of md5 hash in JS files".

2. **Pindahkan proyek ke filesystem Linux**: jika memungkinkan, pastikan proyek Anda berada di filesystem Linux (`/home/`) daripada filesystem Windows (`/mnt/c/`).

3. **Gunakan Windows native sebagai gantinya**: pertimbangkan menjalankan Claude Code secara native di Windows alih-alih melalui WSL, untuk kinerja filesystem yang lebih baik.

<h2 id="get-more-help">
  Dapatkan bantuan lebih lanjut
</h2>

Jika Anda mengalami masalah yang tidak tercakup di sini:

1. Jalankan `/doctor` untuk memeriksa kesehatan instalasi dan `/mcp` untuk memeriksa status server MCP
2. Gunakan perintah `/feedback` dalam Claude Code untuk melaporkan masalah langsung ke Anthropic
3. Periksa [GitHub repository](https://github.com/anthropics/claude-code) untuk masalah yang diketahui
4. Tanyakan Claude secara langsung tentang kemampuan dan fiturnya. Claude memiliki akses bawaan ke dokumentasinya.

Untuk masalah akun, penagihan, atau langganan, hubungi dukungan Anthropic sebagai gantinya: masuk di [claude.ai](https://claude.ai) (Pengguna Console: [platform.claude.com](https://platform.claude.com)), klik inisial Anda di sudut kiri bawah, dan pilih **Dapatkan bantuan**. Lihat [Cara mendapatkan dukungan](https://support.claude.com/en/articles/9015913-how-to-get-support) untuk alur lengkap, termasuk siapa yang dapat menjangkau agen manusia di setiap paket.
