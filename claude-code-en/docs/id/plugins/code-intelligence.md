> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Code intelligence plugins

> Instal plugin language server sehingga Claude melihat kesalahan tipe setelah pengeditan dan menavigasi kode berdasarkan simbol, serta menjawab dialog rekomendasi plugin LSP.

Plugin code intelligence memberikan Claude diagnostik langsung dan go-to-definition yang dimiliki editor Anda, sehingga Claude menangkap kesalahan tipe dan impor yang hilang yang diperkenalkan oleh pengeditan sendirinya sebelum Anda menjalankan build, dan menemukan definisi dan referensi berdasarkan simbol daripada pencarian teks.

Setiap plugin menghubungkan Claude Code ke language server untuk satu bahasa melalui Language Server Protocol (LSP). Anda menginstal plugin dari marketplace resmi Anthropic dan binary language server di mesin Anda.

<Note>
  Plugin code intelligence bekerja dalam sesi terminal. Dalam [sesi cloud](/docs/id/claude-code-on-the-web), Claude Code tidak memulai plugin language servers, jadi Claude tidak mendapatkan diagnostik atau navigasi kode di sana. Untuk menulis plugin language server Anda sendiri, atau untuk menghubungkan language server yang tidak memiliki plugin, lihat [LSP servers in plugin components](/docs/id/plugins/components#lsp-servers).
</Note>

Untuk memulai, temukan bahasa Anda di tabel di bawah [Install a code intelligence plugin](#install-a-code-intelligence-plugin). Plugin dalam tabel itu berasal dari [official plugin marketplace](/docs/id/plugins/anthropic-marketplaces) Anthropic.

Jika Anda sudah melihat dialog **LSP plugin recommendation**, lihat [Accept or dismiss the recommendation dialog](#accept-or-dismiss-the-recommendation-dialog) untuk mengetahui apa yang dilakukan setiap pilihan.

<h2 id="install-a-code-intelligence-plugin">
  Install a code intelligence plugin
</h2>

Plugin code intelligence memberi tahu Claude Code perintah mana yang memulai language server dan ekstensi file mana yang ditanganinya. Ini tidak menyertakan language server. Instal binary language server terlebih dahulu, kemudian plugin, kemudian konfirmasi server dimulai.

<Steps>
  <Step title="Install the language server binary">
    Temukan bahasa Anda di tabel di bawah dan instal binary di barisnya. Jika bahasa Anda tidak terdaftar, lihat [Add a language without an official plugin](#add-a-language-without-an-official-plugin).

    | Language                  | Plugin                                                                                                           | Binary                          |
    | :------------------------ | :--------------------------------------------------------------------------------------------------------------- | :------------------------------ |
    | C/C++                     | [`clangd-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/clangd-lsp)               | `clangd`                        |
    | C#                        | [`csharp-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/csharp-lsp)               | `csharp-ls`                     |
    | Go                        | [`gopls-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/gopls-lsp)                 | `gopls`                         |
    | Java                      | [`jdtls-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/jdtls-lsp)                 | `jdtls`                         |
    | Kotlin                    | [`kotlin-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/kotlin-lsp)               | `kotlin-lsp`                    |
    | Liquid                    | [`liquid-lsp`](https://github.com/Shopify/liquid-skills/tree/main/plugins/liquid-lsp)                            | `shopify`, from the Shopify CLI |
    | Lua                       | [`lua-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/lua-lsp)                     | `lua-language-server`           |
    | PHP                       | [`php-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/php-lsp)                     | `intelephense`                  |
    | Python                    | [`pyright-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/pyright-lsp)             | `pyright-langserver`            |
    | Ruby                      | [`ruby-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/ruby-lsp)                   | `ruby-lsp`                      |
    | Rust                      | [`rust-analyzer-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/rust-analyzer-lsp) | `rust-analyzer`                 |
    | Swift                     | [`swift-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/swift-lsp)                 | `sourcekit-lsp`                 |
    | TypeScript and JavaScript | [`typescript-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/typescript-lsp)       | `typescript-language-server`    |

    Anthropic memelihara setiap plugin dalam tabel kecuali `liquid-lsp`, yang dikelola Shopify dan marketplace resmi mencantumkannya.

    Untuk menemukan perintah yang menginstal binary, ikuti tautan plugin dalam tabel ke README-nya. Untuk TypeScript, perintah itu adalah `npm install -g typescript-language-server typescript`.

    Setelah Anda menginstal binary, konfirmasi bahwa itu ada di `PATH` shell tempat Anda memulai `claude`, misalnya dengan `which typescript-language-server`, atau `Get-Command typescript-language-server` di PowerShell.
  </Step>

  <Step title="Install the plugin">
    Untuk menginstal plugin yang terdaftar untuk bahasa Anda di tabel langkah 1, jalankan `/plugin install` dalam sesi Claude Code, mengganti `typescript-lsp` dengan nama plugin itu:

    ```
    /plugin install typescript-lsp@claude-plugins-official
    ```

    Pesan konfirmasi mengatakan apakah plugin aktif sekarang atau memerlukan `/reload-plugins`. Jika instalasi gagal dengan `Marketplace "claude-plugins-official" not found`, lihat [entri troubleshooting untuk kesalahan itu](/docs/id/plugins/troubleshooting#marketplace-claude-plugins-official-not-found). Untuk mengontrol di mana plugin diinstal, atau untuk menjalankan instalasi dari shell Anda daripada di dalam Claude Code, lihat [Install plugins](/docs/id/plugins/install).
  </Step>

  <Step title="Confirm the server starts">
    Language server dimulai pertama kali Claude mengedit file dengan salah satu ekstensi plugin. Untuk melihatnya bekerja, minta Claude untuk memperkenalkan kesalahan tipe dalam file bahasa itu dan kemudian memperbaikinya. Kemudian periksa percakapan untuk baris diagnostik:

    * **Baris diagnostik muncul**: `Found N new diagnostic issues in M files (ctrl+o to expand)` di bawah edit yang memperkenalkan kesalahan berarti server dimulai.
    * **Tidak ada baris diagnostik yang muncul**: jalankan `/plugin` dan buka tab **Errors**. Baris yang berbunyi `Executable not found in $PATH: "<binary>"` menamai binary yang akan diinstal. Jika tab tidak memiliki baris seperti itu, lihat [Troubleshoot code intelligence](#troubleshoot-code-intelligence).

    Setelah Anda menginstal binary yang hilang, Claude Code mencoba lagi lain kali Claude mengedit file yang cocok. Jika Anda menginstal binary ke direktori yang tidak ada di `PATH` shell tempat Anda memulai `claude`, mulai sesi baru dari shell tempat itu berada.
  </Step>
</Steps>

<h2 id="see-what-claude-gains">
  See what Claude gains
</h2>

Dengan language server yang berjalan, Claude mendapatkan diagnostik dan navigasi kode:

* **Diagnostik setelah pengeditan**: setiap kali Claude mengedit atau menulis file yang ditangani server, Claude mendapatkan kesalahan dan peringatan yang dilaporkan server. Ini melihat kesalahan tipe, impor yang hilang, atau kesalahan sintaks yang diperkenalkannya tanpa menjalankan compiler.
* **Navigasi kode**: Claude mendapatkan alat `LSP` yang mencari simbol melalui server daripada mencari teks untuk mereka. Alat ini hanya baca. Untuk apa yang dapat dicari Claude dengan alat dan bagaimana izin berlaku untuk itu, lihat [LSP tool behavior](/docs/id/tools-reference#lsp-tool-behavior).

<h3 id="read-the-diagnostics-yourself">
  Read the diagnostics yourself
</h3>

Setelah Claude mengedit file yang ditangani server, percakapan hanya menampilkan ringkasan `Found N new diagnostic issues`. Untuk membaca masalahnya sendiri, tekan **Ctrl+O**.

<h2 id="accept-or-dismiss-the-recommendation-dialog">
  Accept or dismiss the recommendation dialog
</h2>

Jika binary language server sudah ada di `PATH` Anda dan plugin yang menggunakannya tidak diinstal, Claude Code menawarkan untuk menginstal plugin untuk Anda dalam dialog berjudul **LSP plugin recommendation**.

<h3 id="when-the-recommendation-dialog-appears">
  When the recommendation dialog appears
</h3>

Dialog **LSP plugin recommendation** dapat muncul setelah Claude mengedit file. Kondisi ini menentukan apakah itu muncul dan plugin mana yang ditawarkan:

* **Plugin cocok dengan file**: salah satu marketplace yang telah Anda tambahkan, atau marketplace resmi yang didaftarkan Claude Code untuk Anda, mencantumkan plugin code intelligence untuk ekstensi file itu, dan binary plugin diinstal.
* **Resmi terlebih dahulu**: ketika lebih dari satu marketplace menawarkan plugin untuk ekstensi, dialog menawarkan plugin marketplace resmi.
* **Sekali per sesi**: dialog muncul paling banyak sekali dalam sesi, untuk file pertama yang cocok Claude edit.
* **Bukan untuk sesi cloud**: dialog tidak pernah muncul ketika terminal Anda terpasang ke sesi cloud, seperti yang Anda mulai dengan [`claude --cloud`](/docs/id/claude-code-on-the-web#from-terminal-to-cloud).

<h3 id="respond-to-the-recommendation-dialog">
  Respond to the recommendation dialog
</h3>

Dialog **LSP plugin recommendation** menamai plugin dan menawarkan pilihan ini:

* **Yes, install**: Claude Code menginstal plugin untuk akun pengguna Anda dan mencetak `<plugin> installed · restart to apply`. Mulai sesi baru untuk memuat server.
* **No, not now**: dialog ditutup, dan sesi yang lebih baru dapat menawarkan plugin lagi. Menekan **Esc** melakukan hal yang sama.
* **Never for this plugin**: dialog berhenti muncul untuk plugin itu dan masih muncul untuk yang lain.
* **Disable all LSP recommendations**: dialog berhenti muncul untuk setiap bahasa.

Jika Anda tidak memilih opsi, Claude Code menutupnya setelah 30 detik dan menghitung itu sebagai diabaikan. Hitungan disimpan di seluruh sesi. Setelah lima dialog yang diabaikan, Claude Code berhenti merekomendasikan plugin, sama seperti jika Anda telah memilih **Disable all LSP recommendations**.

<h3 id="turn-recommendations-back-on">
  Turn recommendations back on
</h3>

Dialog **LSP plugin recommendation** berhenti muncul setelah Anda memilih **Disable all LSP recommendations** atau mengabaikannya lima kali.

* **Dinonaktifkan atau diabaikan lima kali**: untuk mengaktifkannya kembali dalam kedua kasus, hapus kunci `lspRecommendationDisabled` dan `lspRecommendationIgnoredCount` dari `~/.claude.json`, file konfigurasi Claude Code sendiri.
* **Tidak pernah untuk plugin ini**: jika Anda memilih **Never for this plugin** dan menginginkan plugin itu ditawarkan lagi, hapus id `name@marketplace` dari daftar `lspRecommendationNeverPlugins` di file yang sama.

<h2 id="troubleshoot-code-intelligence">
  Troubleshoot code intelligence
</h2>

Halaman troubleshooting plugins mencakup gejala spesifik untuk plugin code intelligence di bawah [Language server doesn't start, uses too much memory, or reports wrong diagnostics](/docs/id/plugins/troubleshooting#language-server-doesnt-start):

* **Language server tidak dimulai**: Anda melihat `Executable not found in $PATH` di tab **Errors** dari `/plugin`, atau Claude tidak pernah melaporkan diagnostik untuk bahasa.
* **Penggunaan memori tinggi**: penggunaan memori meningkat saat server mengindeks proyek.
* **Diagnostik positif palsu dalam monorepo**: diagnostik melaporkan impor sebagai tidak terselesaikan padahal sebenarnya terselesaikan.

<h2 id="add-a-language-without-an-official-plugin">
  Add a language without an official plugin
</h2>

Jika bahasa Anda tidak ada di [tabel plugin resmi](#install-a-code-intelligence-plugin), Anda masih dapat menghubungkan language server.

1. Tulis plugin dengan file `.lsp.json` yang menamai perintah server dan ekstensi file yang ditanganinya.
2. Kemudian muat plugin dengan [`--plugin-dir`](/docs/id/plugins/cli-reference#flags-that-load-a-plugin-for-one-session) atau publikasikan ke marketplace.

Untuk bidang file dan contoh yang dikerjakan, lihat [LSP servers in plugin components](/docs/id/plugins/components#lsp-servers).

<h2 id="next-steps">
  Next steps
</h2>

* [LSP servers in plugin components](/docs/id/plugins/components#lsp-servers): tulis `.lsp.json` untuk language server yang tidak memiliki plugin resmi
* [Install and manage plugins](/docs/id/plugins/install): scopes, updates, dan uninstalling
* [Troubleshoot plugins](/docs/id/plugins/troubleshooting): load errors di luar yang berbasis language-server di halaman ini
* [Find plugins in the official marketplace](/docs/id/plugins/anthropic-marketplaces#find-plugins-in-the-official-marketplace): di mana untuk menjelajahi sisa marketplace resmi
