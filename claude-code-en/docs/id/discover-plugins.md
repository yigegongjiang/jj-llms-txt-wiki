> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Temukan dan instal plugin yang sudah dibuat melalui marketplace

> Temukan dan instal plugin dari marketplace untuk memperluas Claude Code dengan skills, agen, dan kemampuan baru.

Plugin memperluas Claude Code dengan skills, agen, hooks, dan MCP servers. Plugin marketplace adalah katalog yang membantu Anda menemukan dan menginstal ekstensi ini tanpa membuatnya sendiri.

Mencari cara membuat dan mendistribusikan marketplace Anda sendiri? Lihat [Buat dan distribusikan plugin marketplace](/docs/id/plugin-marketplaces).

<h2 id="how-marketplaces-work">
  Cara kerja marketplace
</h2>

Marketplace adalah katalog plugin yang telah dibuat dan dibagikan oleh orang lain. Menggunakan marketplace adalah proses dua langkah:

<Steps>
  <Step title="Tambahkan marketplace">
    Ini mendaftarkan katalog dengan Claude Code sehingga Anda dapat menjelajahi apa yang tersedia. Tidak ada plugin yang diinstal lagi.
  </Step>

  <Step title="Instal plugin individual">
    Jelajahi katalog dan instal plugin yang Anda inginkan.
  </Step>
</Steps>

Anggap saja seperti menambahkan app store: menambahkan toko memberi Anda akses untuk menjelajahi koleksinya, tetapi Anda masih memilih aplikasi mana yang akan diunduh secara individual.

<h2 id="official-anthropic-marketplace">
  Official Anthropic marketplace
</h2>

Official Anthropic marketplace (`claude-plugins-official`) secara otomatis tersedia saat Anda memulai Claude Code. Jalankan `/plugin` dan buka tab **Discover** untuk menjelajahi apa yang tersedia, atau lihat katalog di [claude.com/plugins](https://claude.com/plugins).

Untuk menginstal plugin dari official marketplace, gunakan `/plugin install <name>@claude-plugins-official`. Misalnya, untuk menginstal integrasi GitHub:

```shell theme={null}
/plugin install github@claude-plugins-official
```

Jika Claude Code melaporkan bahwa plugin tidak ditemukan di marketplace mana pun, marketplace Anda mungkin hilang atau ketinggalan zaman. Jalankan `/plugin marketplace update claude-plugins-official` untuk menyegarkannya, atau `/plugin marketplace add anthropics/claude-plugins-official` jika Anda belum menambahkannya sebelumnya. Kemudian coba instal lagi.

<Note>
  Official marketplace dikurasi oleh Anthropic, dan penyertaan adalah atas kebijakan Anthropic. Formulir pengajuan dalam aplikasi menambahkan plugin ke [community marketplace](#community-marketplace), bukan yang resmi. Untuk mendistribusikan plugin secara independen, [buat marketplace Anda sendiri](/docs/id/plugin-marketplaces) dan bagikan dengan pengguna.
</Note>

Official marketplace mencakup beberapa kategori plugin:

<h3 id="code-intelligence">
  Code intelligence
</h3>

Plugin code intelligence mengaktifkan alat LSP bawaan Claude Code, memberikan Claude kemampuan untuk melompat ke definisi, menemukan referensi, dan melihat kesalahan tipe segera setelah edit. Plugin ini mengonfigurasi koneksi [Language Server Protocol](https://microsoft.github.io/language-server-protocol/), teknologi yang sama yang mendukung code intelligence VS Code.

Plugin ini memerlukan binary language server untuk diinstal di sistem Anda. Jika Anda sudah memiliki language server yang diinstal, Claude mungkin akan meminta Anda untuk menginstal plugin yang sesuai saat Anda membuka proyek.

| Language   | Plugin              | Binary required              |
| :--------- | :------------------ | :--------------------------- |
| C/C++      | `clangd-lsp`        | `clangd`                     |
| C#         | `csharp-lsp`        | `csharp-ls`                  |
| Go         | `gopls-lsp`         | `gopls`                      |
| Java       | `jdtls-lsp`         | `jdtls`                      |
| Kotlin     | `kotlin-lsp`        | `kotlin-language-server`     |
| Lua        | `lua-lsp`           | `lua-language-server`        |
| PHP        | `php-lsp`           | `intelephense`               |
| Python     | `pyright-lsp`       | `pyright-langserver`         |
| Rust       | `rust-analyzer-lsp` | `rust-analyzer`              |
| Swift      | `swift-lsp`         | `sourcekit-lsp`              |
| TypeScript | `typescript-lsp`    | `typescript-language-server` |

Anda juga dapat [membuat plugin LSP Anda sendiri](/docs/id/plugins-reference#lsp-servers) untuk bahasa lain.

<Note>
  Jika Anda melihat `Executable not found in $PATH` di tab `/plugin` Errors setelah menginstal plugin, instal binary yang diperlukan dari tabel di atas.
</Note>

<h4 id="what-claude-gains-from-code-intelligence-plugins">
  Apa yang Claude dapatkan dari plugin code intelligence
</h4>

Setelah plugin code intelligence diinstal dan binary language server-nya tersedia, Claude mendapatkan dua kemampuan:

* **Automatic diagnostics**: setelah setiap edit file yang dilakukan Claude, language server menganalisis perubahan dan melaporkan kesalahan dan peringatan secara otomatis. Claude melihat kesalahan tipe, impor yang hilang, dan masalah sintaks tanpa perlu menjalankan compiler atau linter. Jika Claude memperkenalkan kesalahan, itu akan menyadari dan memperbaiki masalah dalam giliran yang sama. Ini tidak memerlukan konfigurasi apa pun selain menginstal plugin. Anda dapat melihat diagnostik secara inline dengan menekan **Ctrl+O** saat indikator "diagnostics found" muncul.
* **Code navigation**: Claude dapat menggunakan language server untuk melompat ke definisi, menemukan referensi, mendapatkan informasi tipe saat hover, membuat daftar simbol, menemukan implementasi, dan melacak hierarki panggilan. Operasi ini memberikan Claude navigasi yang lebih presisi daripada pencarian berbasis grep, meskipun ketersediaan mungkin berbeda menurut bahasa dan lingkungan.

Jika Anda mengalami masalah, lihat [Code intelligence troubleshooting](#code-intelligence-issues).

<h3 id="external-integrations">
  External integrations
</h3>

Plugin ini menggabungkan [MCP servers](/docs/id/mcp) yang sudah dikonfigurasi sebelumnya sehingga Anda dapat menghubungkan Claude ke layanan eksternal tanpa setup manual:

* **Source control**: `github`, `gitlab`
* **Project management**: `atlassian` (Jira/Confluence), `asana`, `linear`, `notion`
* **Design**: `figma`
* **Infrastructure**: `vercel`, `firebase`, `supabase`
* **Communication**: `slack`
* **Monitoring**: `sentry`

<h3 id="automatic-security-review">
  Automatic security review
</h3>

Plugin `security-guidance` meninjau setiap perubahan yang dilakukan Claude untuk kerentanan umum dan menginstruksikan Claude untuk memperbaiki apa yang ditemukannya dalam sesi yang sama. Lihat [Catch security issues as Claude writes code](/docs/id/security-guidance) untuk apa yang diperiksa dan cara menambahkan aturan khusus proyek.

<h3 id="development-workflows">
  Development workflows
</h3>

Plugin yang menambahkan skills dan agen untuk tugas pengembangan umum:

* **commit-commands**: Git commit workflows termasuk commit, push, dan pembuatan PR
* **pr-review-toolkit**: Agen khusus untuk meninjau pull request
* **agent-sdk-dev**: Tools untuk membangun dengan Claude Agent SDK
* **plugin-dev**: Toolkit untuk membuat plugin Anda sendiri

<h3 id="output-styles">
  Output styles
</h3>

Sesuaikan cara Claude merespons:

* **explanatory-output-style**: Wawasan edukatif tentang pilihan implementasi
* **learning-output-style**: Mode pembelajaran interaktif untuk membangun skill

<h2 id="community-marketplace">
  Community marketplace
</h2>

Community marketplace di [`anthropics/claude-plugins-community`](https://github.com/anthropics/claude-plugins-community) menampung plugin pihak ketiga yang telah lulus validasi otomatis Anthropic dan penyaringan keamanan. Setiap plugin disematkan ke SHA commit tertentu dalam katalog. Tidak seperti official marketplace, Anda menambahkannya secara manual:

```shell theme={null}
/plugin marketplace add anthropics/claude-plugins-community
```

Kemudian instal plugin darinya menggunakan nama marketplace `claude-community`:

```shell theme={null}
/plugin install <plugin-name>@claude-community
```

Untuk mengirimkan plugin Anda sendiri ke community marketplace, lihat [Kirimkan plugin Anda ke community marketplace](/docs/id/plugins#submit-your-plugin-to-the-community-marketplace) dalam panduan create-plugins.

<h2 id="try-it-add-the-demo-marketplace">
  Coba: tambahkan demo marketplace
</h2>

Anthropic juga memelihara [demo plugins marketplace](https://github.com/anthropics/claude-code/tree/main/plugins) (`claude-code-plugins`) dengan plugin contoh yang menunjukkan apa yang mungkin dengan sistem plugin. Tidak seperti official marketplace, Anda perlu menambahkan ini secara manual.

<Steps>
  <Step title="Tambahkan marketplace">
    Dari dalam Claude Code, jalankan perintah `plugin marketplace add` untuk marketplace `anthropics/claude-code`:

    ```shell theme={null}
    /plugin marketplace add anthropics/claude-code
    ```

    Ini mengunduh katalog marketplace dan membuat plugin-nya tersedia untuk Anda.
  </Step>

  <Step title="Jelajahi plugin yang tersedia">
    Jalankan `/plugin` untuk membuka plugin manager. Ini membuka antarmuka bertab dengan empat tab yang dapat Anda siklus menggunakan **Tab**, atau **Shift+Tab** untuk mundur:

    * **Discover**: jelajahi plugin yang tersedia dari semua marketplace Anda
    * **Installed**: lihat dan kelola plugin yang diinstal
    * **Marketplaces**: tambah, hapus, atau perbarui marketplace yang ditambahkan
    * **Errors**: lihat kesalahan pemuatan plugin apa pun

    Buka tab **Discover** untuk melihat plugin dari marketplace yang baru saja Anda tambahkan. Ketika administrator Anda telah memasukkan marketplace ke dalam daftar putih melalui pengaturan terkelola [`pluginSuggestionMarketplaces`](/docs/id/settings#available-settings), plugin yang ditandai sebagai relevan dengan direktori kerja Anda saat ini disematkan di bagian atas dengan label **suggested for this directory**.
  </Step>

  <Step title="Instal plugin">
    Pilih plugin untuk melihat detailnya. Pane detail menunjukkan apa yang berisi plugin dan biayanya:

    * Estimasi **Context cost** sehingga Anda dapat melihat berapa banyak token yang akan ditambahkan plugin ke [context window](/docs/id/features-overview#understand-context-costs) Anda setiap putaran (Claude Code v2.1.143 dan yang lebih baru)
    * Tanggal **Last updated** plugin (v2.1.144 dan yang lebih baru)
    * Bagian **Will install** yang mencantumkan perintah, agen, skills, hooks, dan server MCP dan LSP plugin, sehingga Anda dapat meninjau dengan tepat apa yang ditambahkan sebelum menginstal (v2.1.145 dan yang lebih baru)

    Pilih cakupan instalasi:

    * **User scope**: instal untuk diri sendiri di semua proyek
    * **Project scope**: instal untuk semua kolaborator di repositori ini
    * **Local scope**: instal untuk diri sendiri di repositori ini saja

    Misalnya, pilih **commit-commands**, plugin yang menambahkan skills alur kerja git, dan instal ke cakupan pengguna Anda.

    Anda juga dapat menginstal langsung dari baris perintah:

    ```shell theme={null}
    /plugin install commit-commands@claude-code-plugins
    ```

    Lihat [Configuration scopes](/docs/id/settings#configuration-scopes) untuk mempelajari lebih lanjut tentang cakupan.
  </Step>

  <Step title="Gunakan plugin baru Anda">
    Setelah menginstal, jalankan `/reload-plugins` untuk mengaktifkan plugin. Skills plugin diberi namespace oleh nama plugin, jadi **commit-commands** menyediakan skills seperti `/commit-commands:commit`.

    Coba dengan membuat perubahan pada file dan menjalankan:

    ```shell theme={null}
    /commit-commands:commit
    ```

    Ini menampilkan perubahan Anda, menghasilkan pesan commit, dan membuat commit.

    Setiap plugin bekerja berbeda. Periksa detail plugin di tab **Discover** untuk melihat perintah dan skills yang disediakan, atau kunjungi homepage-nya untuk panduan penggunaan.
  </Step>
</Steps>

Sisa panduan ini mencakup semua cara Anda dapat menambahkan marketplace, menginstal plugin, dan mengelola konfigurasi Anda.

<h2 id="add-marketplaces">
  Tambahkan marketplace
</h2>

Gunakan perintah `/plugin marketplace add` untuk menambahkan marketplace dari sumber yang berbeda.

<Tip>
  **Shortcuts**: Anda dapat menggunakan `/plugin market` sebagai ganti `/plugin marketplace`, dan `rm` sebagai ganti `remove`.
</Tip>

* **GitHub repositories**: format `owner/repo`, misalnya `anthropics/claude-code`
* **Git URLs**: URL repositori git apa pun, termasuk GitLab, Bitbucket, dan server self-hosted
* **Local paths**: direktori atau jalur langsung ke file `marketplace.json`
* **Remote URLs**: URL langsung ke file `marketplace.json` yang dihosting

<h3 id="add-from-github">
  Tambahkan dari GitHub
</h3>

Tambahkan repositori GitHub yang berisi file `.claude-plugin/marketplace.json` menggunakan format `owner/repo`, di mana `owner` adalah nama pengguna atau organisasi GitHub dan `repo` adalah nama repositori.

Misalnya, `anthropics/claude-code` merujuk ke repositori `claude-code` yang dimiliki oleh `anthropics`:

```shell theme={null}
/plugin marketplace add anthropics/claude-code
```

<h3 id="add-from-other-git-hosts">
  Tambahkan dari host Git lainnya
</h3>

Tambahkan repositori git apa pun dengan memberikan URL lengkap. Ini bekerja dengan host Git apa pun, termasuk GitLab, Bitbucket, dan server self-hosted. Sertakan akhiran `.git` sehingga Claude Code mengkloning repositori daripada memperlakukan URL sebagai tautan langsung ke file `marketplace.json` yang dihosting.

Sertakan awalan `https://` juga. Claude Code v2.1.196 dan yang lebih baru menolak host yang diketik tanpa itu, seperti `gitlab.com/company/plugins.git`, sebagai shorthand `owner/repo` GitHub yang tidak valid, dan kesalahan memberi tahu Anda untuk menambahkan awalan. Versi sebelumnya salah membacanya sebagai jalur repositori GitHub dan gagal saat waktu kloning.

Menggunakan HTTPS:

```shell theme={null}
/plugin marketplace add https://gitlab.com/company/plugins.git
```

Menggunakan SSH:

```shell theme={null}
/plugin marketplace add git@gitlab.com:company/plugins.git
```

Untuk menambahkan cabang atau tag tertentu, tambahkan `#` diikuti oleh ref:

```shell theme={null}
/plugin marketplace add https://gitlab.com/company/plugins.git#v1.0.0
```

<h3 id="add-from-local-paths">
  Tambahkan dari jalur lokal
</h3>

Tambahkan direktori lokal yang berisi file `.claude-plugin/marketplace.json`:

```shell theme={null}
/plugin marketplace add ./my-marketplace
```

Anda juga dapat menambahkan jalur langsung ke file `marketplace.json`:

```shell theme={null}
/plugin marketplace add ./path/to/marketplace.json
```

<h3 id="add-from-remote-urls">
  Tambahkan dari URL jarak jauh
</h3>

Tambahkan file `marketplace.json` jarak jauh melalui URL:

```shell theme={null}
/plugin marketplace add https://example.com/marketplace.json
```

<Note>
  Marketplace berbasis URL memiliki beberapa keterbatasan dibandingkan dengan marketplace berbasis Git. Jika Anda mengalami kesalahan "path not found" saat menginstal plugin, lihat [Troubleshooting](/docs/id/plugin-marketplaces#plugins-with-relative-paths-fail-in-url-based-marketplaces).
</Note>

<h2 id="install-plugins">
  Instal plugin
</h2>

Setelah Anda menambahkan marketplace, Anda dapat menginstal plugin secara langsung:

```shell theme={null}
/plugin install plugin-name@marketplace-name
```

Perintah membuka detail plugin tersebut, di mana Anda memilih [cakupan instalasi](/docs/id/settings#configuration-scopes). Anda melihat pilihan yang sama ketika Anda menjalankan `/plugin`, pergi ke tab **Discover**, dan menekan **Enter** pada plugin:

* **User scope** (default): instal untuk diri sendiri di semua proyek
* **Project scope**: instal untuk semua kolaborator di repositori ini, yang menambahkan plugin ke `.claude/settings.json`
* **Local scope**: instal untuk diri sendiri di repositori ini saja, tidak dibagikan dengan kolaborator

Untuk menginstal tanpa langkah interaktif, gunakan perintah shell [`claude plugin install`](/docs/id/plugins-reference#plugin-install), yang menginstal ke cakupan pengguna kecuali Anda melewatkan `--scope`.

Anda juga dapat melihat plugin dengan cakupan **managed**. Ini diinstal oleh administrator melalui [managed settings](/docs/id/settings#settings-files) dan tidak dapat dimodifikasi.

<Warning>
  Pastikan Anda mempercayai plugin sebelum menginstalnya. Anthropic tidak mengontrol apa yang MCP servers, file, atau perangkat lunak lain yang disertakan dalam plugin dan tidak dapat memverifikasi bahwa mereka bekerja seperti yang dimaksudkan. Periksa homepage setiap plugin untuk informasi lebih lanjut.
</Warning>

<h2 id="manage-installed-plugins">
  Kelola plugin yang diinstal
</h2>

Jalankan `/plugin` dan buka tab **Installed** untuk melihat, mengaktifkan, menonaktifkan, atau menghapus plugin Anda. Daftar dikelompokkan menurut cakupan dan diurutkan sehingga Anda melihat masalah terlebih dahulu: plugin dengan kesalahan pemuatan atau dependensi yang tidak terselesaikan muncul di bagian atas, diikuti oleh favorit Anda, dengan plugin yang dinonaktifkan dilipat di belakang header yang runtuh di bagian bawah.

Dari daftar Anda dapat:

* tekan `f` untuk menandai atau menghapus tanda favorit pada plugin yang dipilih
* ketik untuk memfilter berdasarkan nama atau deskripsi plugin
* tekan Enter untuk membuka tampilan detail plugin dan mengaktifkan, menonaktifkan, atau menghapusnya

Menghapus plugin yang diaktifkan oleh `.claude/settings.json` proyek menanyakan cakupan mana yang Anda maksud: nonaktifkan untuk Anda saja, yang menulis penggantian ke `.claude/settings.local.json` Anda dan membiarkan plugin tetap diinstal untuk proyek, atau hapus untuk semua orang, yang menghapusnya dari `.claude/settings.json` bersama. Memerlukan Claude Code v2.1.203 atau lebih baru. Sebelum v2.1.203, dialog hanya menawarkan nonaktifkan lokal.

Tampilan detail menunjukkan komponen yang disumbangkan plugin: perintah, skills, agen, hooks, server MCP, dan server LSP. Inventaris yang sama tersedia dari baris perintah dengan `claude plugin details`.

Tab **Installed** juga mengumpulkan plugin marketplace yang Anda instal sendiri tetapi belum digunakan dalam setidaknya dua minggu, dalam rentang setidaknya 10 sesi, di bawah header **Not used recently**. Tampilan detail menunjukkan baris **Last used** untuk setiap plugin. Gunakan ini untuk menemukan plugin yang masih menambah biaya startup dan konteks meskipun Anda tidak lagi menggunakannya, kemudian nonaktifkan atau hapus. Memerlukan Claude Code v2.1.187 atau lebih baru.

Dua jenis plugin tidak pernah dicantumkan sebagai tidak digunakan:

* plugin yang organisasi Anda kelola atau yang Anda muat dengan `--plugin-dir`
* plugin yang menyumbangkan tema, gaya output, monitor, atau workflow, karena mereka memberikan nilai tanpa invokasi untuk dilacak

Header **Not used recently** dan baris **Last used** keduanya disembunyikan ketika organisasi Anda membatasi marketplace dengan [`strictKnownMarketplaces`](/docs/id/settings#strictknownmarketplaces).

[Language server](/docs/id/plugins#add-lsp-servers-to-your-plugin) plugin dihitung sebagai digunakan ketika ia memberikan diagnostik atau menjawab permintaan navigasi kode, jadi plugin LSP yang servernya aktif dalam sesi Anda tidak dicantumkan sebagai tidak digunakan. Sebelum v2.1.203, aktivitas language server tidak dapat dihitung sebagai penggunaan, jadi plugin yang menyumbangkan server LSP dikecualikan dari grup sepenuhnya, dengan cara yang sama seperti plugin tema dan gaya output masih.

Sesi pertama pada versi yang menghitung aktivitas language server juga mengatur ulang catatan penggunaan setiap plugin LSP yang belum mencatat penggunaan apa pun, jadi Claude Code tidak menilai plugin yang Anda instal sebelumnya sebagai tidak digunakan berdasarkan data yang dicatat sebelum aktivitas servernya dilacak. Sebelum v2.1.206, sesi pertama itu dapat mencantumkan plugin LSP yang digunakan secara aktif di bawah **Not used recently** dan menyarankan untuk meninjau.

Saat Anda menginstal plugin yang mendeklarasikan dependensi, output instalasi mencantumkan dependensi mana yang diinstal secara otomatis bersama dengannya.

Anda juga dapat mengelola plugin dengan perintah langsung.

Daftar plugin yang diinstal tanpa membuka menu:

```shell theme={null}
/plugin list
```

Lewatkan `--enabled` atau `--disabled` untuk menampilkan hanya plugin dalam status tersebut.

Nonaktifkan plugin tanpa menghapusnya:

```shell theme={null}
/plugin disable plugin-name@marketplace-name
```

Aktifkan kembali plugin yang dinonaktifkan:

```shell theme={null}
/plugin enable plugin-name@marketplace-name
```

Dalam pengenal ini, `plugin-name` adalah `name` plugin dalam [entri marketplace](/docs/id/plugin-marketplaces#plugin-entries), yang dapat berbeda dari `name` dalam `plugin.json` plugin itu sendiri.

Mulai dari Claude Code v2.1.195, **Enable** dan **Disable** dalam antarmuka `/plugin` bekerja untuk plugin yang dua namanya berbeda, dan `/plugin enable` serta `/plugin disable` menerima nama apa pun. Ketika Anda menonaktifkan plugin seperti itu dalam versi sebelumnya, Claude Code melaporkan `already disabled` dan membiarkannya tetap diaktifkan.

Hapus plugin sepenuhnya:

```shell theme={null}
/plugin uninstall plugin-name@marketplace-name
```

Opsi `--scope` memungkinkan Anda menargetkan cakupan tertentu dengan perintah CLI:

```shell theme={null}
claude plugin install formatter@your-org --scope project
claude plugin uninstall formatter@your-org --scope project
```

<h3 id="apply-plugin-changes-without-restarting">
  Terapkan perubahan plugin tanpa restart
</h3>

Saat Anda menginstal, mengaktifkan, atau menonaktifkan plugin selama sesi, jalankan `/reload-plugins` untuk mengambil semua perubahan tanpa restart:

```shell theme={null}
/reload-plugins
```

Claude Code memuat ulang semua plugin aktif dan menampilkan hitungan untuk plugin, skills, agen, hooks, server MCP plugin, dan server LSP plugin.

Memuat ulang memiliki biaya token pada permintaan berikutnya: komponen yang baru dimuat mengumumkan diri mereka dalam konten yang ditambahkan ke percakapan, sementara riwayat yang ada masih membaca dari prompt cache. Plugin yang menyediakan server MCP memerlukan biaya lebih ketika alatnya tidak ditangguhkan oleh [pencarian alat](/docs/id/mcp#scale-with-mcp-tool-search): perubahan membatalkan cache dan permintaan berikutnya membaca ulang seluruh percakapan. Dalam hal itu `/reload-plugins` menampilkan peringatan dan tidak menerapkan reload; lewatkan `--force` untuk menerapkan bagaimanapun. Lihat [mengaktifkan atau menonaktifkan plugin](/docs/id/prompt-caching#enabling-or-disabling-a-plugin) untuk detail.

<h2 id="manage-marketplaces">
  Kelola marketplace
</h2>

Anda dapat mengelola marketplace melalui antarmuka `/plugin` interaktif atau dengan perintah CLI.

<h3 id="use-the-interactive-interface">
  Gunakan antarmuka interaktif
</h3>

Jalankan `/plugin` dan buka tab **Marketplaces** untuk:

* Lihat semua marketplace yang ditambahkan dengan sumber dan statusnya
* Tambahkan marketplace baru
* Perbarui daftar marketplace untuk mengambil plugin terbaru
* Hapus marketplace yang tidak lagi Anda butuhkan

<h3 id="use-cli-commands">
  Gunakan perintah CLI
</h3>

Anda juga dapat mengelola marketplace dengan perintah langsung.

Daftar semua marketplace yang dikonfigurasi:

```shell theme={null}
/plugin marketplace list
```

Segarkan daftar plugin dari marketplace:

```shell theme={null}
/plugin marketplace update marketplace-name
```

Hapus marketplace:

```shell theme={null}
/plugin marketplace remove marketplace-name
```

<Warning>
  Menghapus marketplace akan menghapus instalasi plugin apa pun yang Anda instal darinya.
</Warning>

<h3 id="configure-auto-updates">
  Konfigurasi auto-updates
</h3>

Claude Code dapat secara otomatis memperbarui marketplace dan plugin yang diinstal di latar belakang setelah startup. Saat auto-update diaktifkan untuk marketplace, Claude Code menyegarkan data marketplace dan memperbarui plugin yang diinstal ke versi terbaru mereka di disk.

Claude Code memeriksa pembaruan marketplace dan plugin setelah sesi Anda dimulai, dengan penundaan acak hingga sepuluh menit, sehingga sesi yang berjalan terus menggunakan versi yang dimuat saat peluncuran. Jika ada plugin yang diperbarui, Anda akan melihat notifikasi yang meminta Anda untuk menjalankan `/reload-plugins`, atau versi baru dimuat saat peluncuran berikutnya.

Alihkan auto-update untuk marketplace individual melalui UI:

1. Jalankan `/plugin` untuk membuka plugin manager
2. Pilih **Marketplaces**
3. Pilih marketplace dari daftar
4. Pilih **Enable auto-update** atau **Disable auto-update**

Marketplace resmi Anthropic memiliki auto-update diaktifkan secara default. Marketplace pihak ketiga dan pengembangan lokal memiliki auto-update dinonaktifkan secara default.

Administrator juga dapat mengatur `"autoUpdate": true` pada setiap entri [`extraKnownMarketplaces`](/docs/id/settings#extraknownmarketplaces) dalam pengaturan terkelola untuk mengaktifkan auto-update untuk marketplace organisasi tanpa memerlukan setiap pengguna untuk mengalihkannya.

Untuk menonaktifkan semua pembaruan otomatis sepenuhnya untuk Claude Code dan semua plugin, atur variabel lingkungan `DISABLE_AUTOUPDATER`. Lihat [Auto updates](/docs/id/setup#auto-updates) untuk detail.

Untuk menjaga plugin auto-updates tetap diaktifkan sambil menonaktifkan Claude Code auto-updates, atur `FORCE_AUTOUPDATE_PLUGINS=1` bersama dengan `DISABLE_AUTOUPDATER`:

```bash theme={null}
export DISABLE_AUTOUPDATER=1
export FORCE_AUTOUPDATE_PLUGINS=1
```

Ini berguna saat Anda ingin mengelola pembaruan Claude Code secara manual tetapi masih menerima pembaruan plugin otomatis.

<h2 id="configure-team-marketplaces">
  Konfigurasi team marketplace
</h2>

Admin tim dapat menyiapkan instalasi marketplace otomatis untuk proyek dengan menambahkan konfigurasi marketplace ke `.claude/settings.json`. Saat anggota tim mempercayai folder repositori, Claude Code meminta mereka untuk menginstal marketplace dan plugin ini.

Mulai dari Claude Code v2.1.195, langkah instalasi ini berlaku pada setiap path yang memuat plugin. Plugin yang hanya diaktifkan oleh `.claude/settings.json` proyek dan berasal dari sumber eksternal seperti repositori GitHub atau paket npm, tidak akan dimuat sampai anggota tim menginstalnya. Sampai saat itu, Claude Code melaporkan plugin sebagai tidak terinstal dan menampilkan perintah `claude plugin install` untuk dijalankan.

Tambahkan `extraKnownMarketplaces` ke `.claude/settings.json` proyek Anda:

```json theme={null}
{
  "extraKnownMarketplaces": {
    "my-team-tools": {
      "source": {
        "source": "github",
        "repo": "your-org/claude-plugins"
      }
    }
  }
}
```

Untuk opsi konfigurasi lengkap termasuk `extraKnownMarketplaces` dan `enabledPlugins`, lihat [Plugin settings](/docs/id/settings#plugin-settings).

<h2 id="security">
  Security
</h2>

Plugin dan marketplace adalah komponen yang sangat dipercaya yang dapat menjalankan kode arbitrer di mesin Anda dengan hak istimewa pengguna Anda. Hanya instal plugin dan tambahkan marketplace dari sumber yang Anda percayai. Organisasi dapat membatasi marketplace mana yang diizinkan pengguna untuk ditambahkan menggunakan [managed marketplace restrictions](/docs/id/plugin-marketplaces#managed-marketplace-restrictions).

<h2 id="troubleshooting">
  Troubleshooting
</h2>

<h3 id="/plugin-command-not-recognized">
  /plugin command not recognized
</h3>

Jika Anda melihat "unknown command" atau perintah `/plugin` tidak muncul:

1. **Periksa versi Anda**: Jalankan `claude --version` untuk melihat apa yang diinstal.
2. **Perbarui Claude Code**:
   * **Homebrew**: `brew upgrade claude-code`, atau `brew upgrade claude-code@latest` jika Anda menginstal cask itu
   * **npm**: `npm install -g @anthropic-ai/claude-code@latest`
   * **Native installer**: Jalankan kembali perintah install dari [Setup](/docs/id/setup)
3. **Restart Claude Code**: Setelah memperbarui, restart terminal Anda dan jalankan `claude` lagi.

<h3 id="common-issues">
  Common issues
</h3>

* **Marketplace not loading**: Verifikasi URL dapat diakses dan bahwa `.claude-plugin/marketplace.json` ada di jalur
* **Plugin installation failures**: Periksa bahwa URL sumber plugin dapat diakses dan repositori bersifat publik, atau bahwa Anda memiliki akses ke repositori tersebut
* **Files not found after installation**: Plugin disalin ke cache, jadi jalur yang mereferensikan file di luar direktori plugin tidak akan berfungsi
* **Plugin skills not appearing**: Hapus cache dengan `rm -rf ~/.claude/plugins/cache`, restart Claude Code, dan instal ulang plugin.

Untuk troubleshooting terperinci dengan solusi, lihat [Troubleshooting](/docs/id/plugin-marketplaces#troubleshooting) dalam panduan marketplace. Untuk tools debugging, lihat [Debugging and development tools](/docs/id/plugins-reference#debugging-and-development-tools).

<h3 id="code-intelligence-issues">
  Code intelligence issues
</h3>

* **Language server not starting**: Verifikasi binary diinstal dan tersedia di `$PATH` Anda. Periksa tab `/plugin` Errors untuk detail.
* **High memory usage**: Language server seperti `rust-analyzer` dan `pyright` dapat mengonsumsi memori signifikan pada proyek besar. Jika Anda mengalami masalah memori, nonaktifkan plugin dengan `/plugin disable <plugin-name>` dan andalkan tools pencarian bawaan Claude sebagai gantinya.
* **False positive diagnostics in monorepos**: Language server mungkin melaporkan kesalahan impor yang tidak terselesaikan untuk paket internal jika workspace tidak dikonfigurasi dengan benar. Ini tidak mempengaruhi kemampuan Claude untuk mengedit kode.

<h2 id="next-steps">
  Langkah selanjutnya
</h2>

* **Buat plugin Anda sendiri**: lihat [Plugins](/docs/id/plugins) untuk membuat skills, agen, dan hooks
* **Buat marketplace**: lihat [Create a plugin marketplace](/docs/id/plugin-marketplaces) untuk mendistribusikan plugin ke tim atau komunitas Anda
* **Referensi teknis**: lihat [Plugins reference](/docs/id/plugins-reference) untuk spesifikasi lengkap
