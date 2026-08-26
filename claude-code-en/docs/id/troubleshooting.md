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
| `API Error: 5xx`, `529 Overloaded`, `429`, kesalahan validasi permintaan                                                                                | [Error reference](/docs/id/errors)                                                            |
| `model not found` atau `you may not have access to it`                                                                                                  | [Error reference](/docs/id/errors#theres-an-issue-with-the-selected-model)                    |
| Ekstensi VS Code tidak terhubung atau tidak mendeteksi Claude                                                                                           | [VS Code integration](/docs/id/vs-code#fix-common-issues)                                     |
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

1. Gunakan `/compact` secara teratur untuk mengurangi ukuran konteks
2. Tutup dan mulai ulang Claude Code di antara tugas-tugas besar
3. Pertimbangkan menambahkan direktori build besar ke file `.gitignore` Anda
4. Mulai ulang dengan [`claude --safe-mode`](/docs/id/cli-reference#cli-flags) untuk memeriksa apakah plugin, server MCP, atau hook adalah sumbernya. Ini menonaktifkan semua kustomisasi untuk sesi; jika penggunaan turun, lihat [Debug your configuration](/docs/id/debug-your-config#test-against-a-clean-configuration) untuk menemukan yang mana

Jika penggunaan memori tetap tinggi setelah langkah-langkah ini, jalankan `/heapdump` untuk menulis snapshot heap JavaScript dan rincian memori ke `~/Desktop`. Di Linux tanpa folder Desktop, file ditulis ke direktori home Anda.

Rincian menunjukkan resident set size, JS heap, array buffers, dan native memory yang tidak terhitung, yang membantu mengidentifikasi apakah pertumbuhan ada di objek JavaScript atau di kode native. Untuk memeriksa retainers, buka file `.heapsnapshot` di Chrome DevTools di bawah Memory → Load; rinciannya adalah file yang berakhir dengan `-diagnostics.json`.

<Warning>
  File `.heapsnapshot` berisi setiap string dalam proses. Jangan lampirkan ke masalah publik atau bagikan. Lampirkan hanya file `-diagnostics.json` saat melaporkan masalah memori di [GitHub](https://github.com/anthropics/claude-code/issues). File tersebut berisi statistik memori dan tidak ada konten percakapan atau kredensial.
</Warning>

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

Kemudian atur `USE_BUILTIN_RIPGREP=0` di [environment](/docs/id/env-vars) Anda.

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
