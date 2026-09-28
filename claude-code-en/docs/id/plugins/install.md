> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Instal dan kelola plugin

> Instal plugin Claude Code dari marketplace di permukaan apa pun yang Anda gunakan, pilih cakupan instalasi, dan perbarui atau hapus nanti.

Menginstal plugin menambahkan skills, agents, hooks, dan MCP servers-nya ke Claude Code di mesin Anda.

Halaman ini untuk siapa pun yang menggunakan plugin di mesin atau akun mereka sendiri, baik di terminal, aplikasi desktop, IDE, atau sesi cloud: ini mencakup instalasi, memilih cakupan, menambahkan marketplace, dan menjaga plugin tetap terbaru.

<Note>
  Kasus-kasus ini tercakup di halaman lain:

  * **Anda menggunakan claude.ai chat atau Cowork, bukan Claude Code**: lihat [Plugins on claude.ai and in Cowork](https://claude.com/docs/plugins/overview)
  * **Claude Code mencetak error**: temukan di [Troubleshoot plugins](/docs/id/plugins/troubleshooting)
</Note>

Mulai dengan [Instal plugin](#install-a-plugin). Jika seseorang mengirimkan Anda perintah instalasi yang nama `@`-nya bukan `claude-plugins-official`, [tambahkan marketplace itu](#add-a-marketplace) terlebih dahulu.

<h2 id="install-a-plugin">
  Instal plugin
</h2>

Sebagai contoh, bagian ini menginstal [`commit-commands`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/commit-commands) dari [marketplace resmi Anthropic](/docs/id/plugins/anthropic-marketplaces), yang menambahkan perintah untuk commit, push, dan membuka pull request.

Langkah yang sama menginstal plugin lain apa pun: ganti namanya dan nama marketplace-nya di mana pun `commit-commands` dan `claude-plugins-official` muncul. Jika plugin itu berasal dari marketplace yang berbeda, [tambahkan marketplace](#add-a-marketplace) terlebih dahulu.

Pilih tab untuk tempat Anda menjalankan Claude Code.

<Tabs>
  <Tab title="Terminal">
    Mulai Claude Code dengan `claude` di proyek Anda, kemudian:

    <Steps>
      <Step title="Buka detail plugin dengan perintah instalasi">
        Jalankan `/plugin install` dengan nama plugin dan marketplace. Dalam sesi, perintah ini tidak langsung menginstal: ini membuka panel `/plugin` pada detail plugin itu sehingga Anda dapat meninjau dan memilih cakupan terlebih dahulu.

        ```text theme={null}
        /plugin install commit-commands@claude-plugins-official
        ```

        Untuk menjelajahi sebagai gantinya, jalankan `/plugin` tanpa nama plugin: panel membuka pada tab **Discover**, yang mencantumkan plugin dari setiap marketplace yang telah Anda tambahkan, dan Anda dapat mengetik untuk mencari, kemudian tekan **Enter** pada plugin untuk membuka detailnya.
      </Step>

      <Step title="Tinjau apa yang ditambahkan plugin">
        Panel detail menunjukkan deskripsi plugin. Ini juga dapat menunjukkan:

        * **Will install**: perintah, agents, skills, hooks, dan MCP dan LSP servers yang ditambahkan plugin.
        * **Last updated**: ditampilkan untuk plugin di marketplace resmi Anthropic.
        * **Context cost**: untuk plugin di marketplace resmi Anthropic, dua perkiraan token. **Every turn** adalah apa yang ditambahkan plugin ke setiap pesan yang Anda kirim, dan **When invoked** adalah apa yang ditambahkan skills dan agents-nya setelah Claude memuatnya. Perkiraan muncul ketika Anda membuka plugin dengan menamai marketplace-nya, seperti yang dilakukan perintah langkah 1, atau dari tab **Marketplaces**. Panel detail yang Anda capai dari daftar **Discover** tidak menunjukkannya.

        Plugin dari marketplace lokal atau kustom dapat menunjukkan `Components will be discovered at installation` sebagai gantinya.

        Plugin dapat menjalankan hooks dan MCP servers, jadi baca panel sebelum Anda menginstal. Lihat [Plugin security and trust](/docs/id/plugins/security).
      </Step>

      <Step title="Pilih cakupan">
        Pilih salah satu dari tiga opsi instalasi:

        * **Install for you (user scope)**: Anda mendapatkan plugin di setiap proyek di mesin ini
        * **Install for all collaborators on this repository (project scope)**: ini diaktifkan untuk semua orang yang bekerja di repository ini
        * **Install for you, in this repo only (local scope)**: Anda mendapatkannya di repository ini saja

        [Pilih cakupan instalasi](#choose-an-install-scope) mengatakan file pengaturan mana yang ditulis masing-masing dan mana yang berlaku ketika plugin yang sama diatur di lebih dari satu.

        Setelah Anda memilih cakupan, Claude Code menginstal plugin bersama dengan dependensi apa pun yang dideklarasikannya, kemudian mencetak ringkasan instalasi.
      </Step>

      <Step title="Baca ringkasan instalasi">
        Kalimat terakhir ringkasan memberi tahu Anda apakah plugin dapat digunakan dalam sesi ini:

        * **Active now**: `Plugin is now active.` Tidak perlu reload.
        * **Reload needed**: `Run /reload-plugins to activate.` Panel ditutup dan Claude Code menjalankan reload itu untuk Anda. Jika reload akan [membatalkan prompt cache](/docs/id/prompt-caching#enabling-or-disabling-a-plugin), itu memperingatkan dan meninggalkan plugin tertunda sebagai gantinya. Jalankan `/reload-plugins --force` untuk mengaktifkannya bagaimanapun, yang biaya satu permintaan uncached.
        * **Load failed**: `The plugin couldn't be loaded`. Buka tab **Errors** di `/plugin` untuk alasannya, kemudian lihat [After install: plugin not working](/docs/id/plugins/troubleshooting#plugin-installed-but-not-working).
      </Step>

      <Step title="Konfirmasi plugin berfungsi">
        Ketik `/` dan cari skills plugin di bawah namanya, dalam bentuk `/<plugin>:<skill>`. Untuk `commit-commands`, `/commit-commands:commit` muncul. Dua tempat lain juga mencantumkan plugin:

        * Buka tab **Installed** di `/plugin`, yang mencantumkan plugin dengan cakupannya.
        * Di shell Anda, jalankan `claude plugin list`, yang mencetak daftar yang sama dengan baris `Version`, `Scope`, dan `Status`.

        Jika `/commit-commands:commit` tidak muncul, lihat [After install: plugin not working](/docs/id/plugins/troubleshooting#plugin-installed-but-not-working).
      </Step>
    </Steps>

    Menginstal dari marketplace lain apa pun memerlukan satu langkah ekstra terlebih dahulu: [tambahkan marketplace](#add-a-marketplace). Claude Code menambahkan marketplace resmi Anthropic untuk Anda pertama kali Anda memulai sesi terminal interaktif, itulah mengapa contoh melewati langkah itu. Jika Anda menemukan plugin di [claude.com/marketplace](https://claude.com/marketplace), tombol **Claude Code**-nya menyalin perintah instalasi dalam [bentuk shell](#install-from-your-shell)-nya, `claude plugin install <name>@claude-plugins-official`.
  </Tab>

  <Tab title="Desktop app">
    Dalam sesi lokal atau SSH di tab **Code** aplikasi desktop:

    <Steps>
      <Step title="Buka browser plugin">
        Klik tombol **+** di sebelah kotak prompt dan pilih **Plugins**, kemudian **Add plugin**. Browser plugin membuka dengan plugin dari marketplace Anda.
      </Step>

      <Step title="Pilih plugin">
        Temukan `commit-commands` dan pilih.
      </Step>

      <Step title="Pilih cakupan">
        Pilih [cakupan](#choose-an-install-scope): akun pengguna Anda, proyek ini, atau lokal saja.
      </Step>
    </Steps>

    Untuk mengaktifkan, menonaktifkan, atau mencopot nanti, gunakan **+ > Plugins > Manage plugins**. Browser plugin tidak tersedia di sesi cloud aplikasi desktop. Lihat [Install plugins in the desktop app](/docs/id/desktop#install-plugins).
  </Tab>

  <Tab title="VS Code">
    Di panel Claude Code di VS Code:

    <Steps>
      <Step title="Buka Manage plugins">
        Ketik `/plugins` di kotak prompt untuk membuka **Manage plugins**.
      </Step>

      <Step title="Instal plugin">
        Di tab **Plugins**, cari `commit-commands` dan klik **Install**. Jika tab tidak mencantumkan plugin apa pun, tambahkan `anthropics/claude-plugins-official` di tab **Marketplaces** terlebih dahulu.
      </Step>

      <Step title="Pilih cakupan">
        Pilih [cakupan](#choose-an-install-scope): **Install for you**, **Install for this project**, atau **Install locally**.
      </Step>
    </Steps>

    Perubahan Anda berlaku untuk sesi terbuka tanpa restart. Lihat [Manage plugins in VS Code](/docs/id/vs-code#manage-plugins).
  </Tab>

  <Tab title="Cloud session">
    [Sesi cloud](/docs/id/cloud-environments), termasuk [browser di claude.ai/code](/docs/id/claude-code-on-the-web), tidak memiliki browser plugin dan tidak memuat plugin yang Anda instal di mesin Anda sendiri atau yang diaktifkan `.claude/settings.json` repository Anda. Untuk plugin yang didistribusikan organisasi Anda melalui pengaturan terkelola, lihat [Manage plugins for your organization](/docs/id/plugins/org).

    Lihat [bagian mana dari setup Anda yang juga tersedia di sesi cloud](/docs/id/cloud-environments#what-carries-over-from-your-setup) untuk sisa setup Anda.
  </Tab>
</Tabs>

<h3 id="choose-an-install-scope">
  Pilih cakupan instalasi
</h3>

Cakupan instalasi plugin memutuskan siapa yang mendapatkan plugin dan file pengaturan mana yang mencatat sebagai diaktifkan:

* **User scope**: plugin diaktifkan untuk Anda di setiap proyek di mesin ini. Entri masuk di `enabledPlugins` di `~/.claude/settings.json`.
* **Project scope**: plugin diaktifkan untuk semua orang yang bekerja di repository ini. Entri masuk di `.claude/settings.json`, yang Anda commit.
* **Local scope**: plugin diaktifkan untuk Anda di repository ini saja. Entri masuk di `.claude/settings.local.json`.

Beberapa plugin diatur oleh penulis mereka untuk mulai dimatikan, melalui bidang [`defaultEnabled`](/docs/id/plugins/manifest-reference#defaultenabled). Plugin seperti itu diinstal tetapi tetap mati sampai Anda mengaktifkannya dengan `claude plugin enable <name>` di shell Anda, atau dari tab **Installed** dari `/plugin` dalam sesi.

Ketika plugin yang sama diatur di beberapa cakupan, pengaturan lokal menimpa pengaturan proyek, dan pengaturan proyek menimpa pengaturan pengguna. Lihat [Find where a plugin is enabled](/docs/id/plugins/loading#find-where-a-plugin-is-enabled) untuk aturan lengkapnya.

Terminal, sesi lokal aplikasi desktop, dan ekstensi VS Code di satu komputer membaca file pengaturan yang sama, jadi plugin yang Anda instal di cakupan pengguna di salah satu dari mereka tersedia di dua lainnya.

<h3 id="other-places-you-run-claude-code">
  JetBrains, non-interactive runs, dan Agent SDK
</h3>

Beberapa tempat Anda menjalankan Claude Code tidak memiliki browser plugin mereka sendiri:

* **JetBrains IDEs**: plugin JetBrains menjalankan Claude Code di terminal IDE, jadi gunakan langkah-langkah tab **Terminal** di sana.
* **`claude -p` dan non-interactive runs lainnya**: `/plugin` tidak berjalan, dan Claude menjawab `/plugin isn't available in this environment.` Plugin yang sudah Anda instal memang memuat. Instal dan kelola dari shell Anda dengan [perintah `claude plugin`](#install-from-your-shell).
* **Agent SDK**: muat plugin melalui opsi plugin SDK. Lihat [Load plugins in the Agent SDK](/docs/id/agent-sdk/plugins).

Jika Claude Code melaporkan bahwa plugin yang diaktifkan di `.claude/settings.json` repository tidak diinstal, lihat [Enabled in project settings but not installed](/docs/id/plugins/loading#enabled-in-project-settings-but-not-installed).

<Tip>
  Jika Anda adalah penulis plugin yang menguji salinan plugin Anda di disk, mulai Claude Code dari shell Anda dengan `--plugin-dir` untuk memuatnya untuk satu sesi alih-alih menginstalnya. Lihat [Flags that load a plugin for one session](/docs/id/plugins/cli-reference#flags-that-load-a-plugin-for-one-session).
</Tip>

<h3 id="plugins-from-your-claude-ai-account">
  Plugin dari akun claude.ai Anda
</h3>

Akun claude.ai Anda adalah sumber plugin terpisah, bersama dengan marketplace yang Anda instal dari:

* **What arrives**: setiap plugin yang Anda aktifkan untuk akun claude.ai Anda, dan setiap plugin yang organisasi Anda aktifkan untuk anggotanya. Dalam sesi terminal mereka disinkronkan di latar belakang setiap kali Anda memulai Claude Code sambil masuk dengan akun itu; dalam sesi Cowork mereka diunduh ketika sesi dimulai.
* **Where you see them**: di `/plugin` dan `claude plugin list` di bawah ID `<name>@synced`. Anda dapat mematikan satu di cakupan Anda sendiri kecuali organisasi Anda memerlukannya.
* **What doesn't go the other way**: plugin yang Anda instal dengan `/plugin` atau `claude plugin install` tetap di mesin ini dan tidak ditambahkan ke akun claude.ai Anda.

Untuk waktu sinkronisasi, persyaratan sign-in, dan mematikan sinkronisasi, lihat [Plugins synced from claude.ai](/docs/id/plugins/loading#synced-plugins).

<h3 id="install-from-your-shell">
  Instal dari shell Anda
</h3>

Jalankan `claude plugin install` di shell Anda untuk menginstal plugin tanpa memulai sesi Claude Code, misalnya dari skrip setup.

* **Scope**: cakupan pengguna secara default. Lewatkan `--scope project` atau `--scope local` untuk mengubahnya.
* **When the plugins load**: plugin yang diinstalnya memuat lain kali Anda memulai Claude Code, atau ketika Anda menjalankan `/reload-plugins` dalam sesi yang sudah terbuka.
* **The marketplace must be added first**: di mesin di mana tidak ada yang telah membuka sesi Claude Code interaktif, marketplace resmi tidak terdaftar, jadi skrip yang menginstal darinya menjalankan `claude plugin marketplace add anthropics/claude-plugins-official` sebelum instalasi.

```bash theme={null}
claude plugin install formatter@your-org --scope project
```

Perintah mencetak `Successfully installed plugin: formatter@your-org (scope: project)` ketika selesai.

Beberapa plugin menginstal dengan menjalankan perintah yang dinamai marketplace mereka, disebut [sumber `command`](/docs/id/plugins/marketplace-reference#command-plugin-source). Claude Code menunjukkan perintah itu kepada Anda dan meminta Anda menerimanya sebelum berjalan. Skrip tidak memiliki siapa pun untuk menjawab prompt itu, jadi lewatkan `--yes` di sana untuk menerimanya.

Untuk setiap bendera `claude plugin install`, lihat [plugin install](/docs/id/plugins/cli-reference#plugin-install).

<h2 id="add-a-marketplace">
  Tambahkan marketplace
</h2>

Anda hanya memerlukan bagian ini ketika plugin yang Anda inginkan tidak ada di marketplace resmi Anthropic, misalnya satu yang diterbitkan rekan kerja atau satu dari marketplace komunitas Anthropic.

Marketplace adalah katalog plugin, dan Claude Code harus tahu tentang marketplace sebelum Anda dapat menginstal darinya. Anda menambahkan marketplace sekali. Setelah itu, plugin-nya muncul di tab **Discover** dan menginstal dengan `/plugin install <plugin>@<marketplace>` dalam sesi atau `claude plugin install <plugin>@<marketplace>` di shell Anda, di mana `<marketplace>` adalah nama yang didaftarkan marketplace. Untuk melakukan keduanya dalam satu langkah, lihat [Add a marketplace and install in one command](#add-a-marketplace-and-install-in-one-command).

Dalam sesi Claude Code, jalankan `/plugin marketplace add` diikuti oleh sumber marketplace: repository GitHub, repository git di host apa pun, direktori atau file lokal, atau `marketplace.json` yang dihosting.

| Source                     | What you type                                                                                                                                                                                                                                    | Example                                                                                                                                 |
| :------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------- |
| GitHub repository          | `owner/repo`. Tambahkan `#ref` untuk menyematkan branch atau tag.                                                                                                                                                                                | `/plugin marketplace add anthropics/claude-code`, atau `/plugin marketplace add your-org/plugins#v1.2.0` untuk menyematkan tag `v1.2.0` |
| Git repository on any host | URL clone lengkap. Tambahkan `#ref` untuk menyematkan branch atau tag.                                                                                                                                                                           | `/plugin marketplace add https://gitlab.example.com/your-group/your-marketplace.git#v1.0.0`                                             |
| Local directory or file    | Jalur relatif atau absolut ke direktori yang menyimpan `.claude-plugin/marketplace.json`, atau ke file JSON itu sendiri. Mulai jalur relatif dengan `./` atau `../`, karena Claude Code membaca `name/name` telanjang sebagai repository GitHub. | `/plugin marketplace add ./my-marketplace`                                                                                              |
| Hosted `marketplace.json`  | URL `https://`-nya                                                                                                                                                                                                                               | `/plugin marketplace add https://example.com/marketplace.json`                                                                          |

Dari shell Anda, `claude plugin marketplace add` mengambil sumber yang sama.

<Tip>
  `/plugin market` juga berfungsi sebagai bentuk lebih pendek dari `/plugin marketplace`.
</Tip>

Sertakan awalan `https://` pada setiap URL, atau gunakan bentuk `git@host:path` untuk SSH. Jika Anda mengetik `gitlab.example.com/your-group/your-marketplace.git` telanjang, Claude Code membacanya sebagai shorthand GitHub `owner/repo` dan menolaknya.

Ketika perintah berhasil, itu mencetak `Successfully added marketplace: <name>`, dan plugin marketplace muncul di tab **Discover** lain kali Anda membuka `/plugin`, tanpa reload diperlukan. Jika gagal, cocokkan pesan error di [Troubleshoot plugins](/docs/id/plugins/troubleshooting#add-a-marketplace).

<h3 id="add-a-marketplace-and-install-in-one-command">
  Tambahkan marketplace dan instal dalam satu perintah
</h3>

Untuk menginstal plugin dari marketplace yang belum Anda tambahkan, jalankan `/plugin install` dalam sesi Claude Code dan namai sumber marketplace dengan `--marketplace`. Memerlukan Claude Code v2.1.275 atau lebih baru.

```text theme={null}
/plugin install deploy-helper --marketplace your-org/plugins
```

Sumber mengambil [bentuk yang sama seperti `/plugin marketplace add`](#add-a-marketplace), seperti GitHub `owner/repo`, URL git, atau jalur lokal, kecuali bahwa tidak dapat berisi spasi. Berikan nama plugin sendiri, tanpa akhiran `@marketplace`.

Jika Anda belum menambahkan marketplace itu, Claude Code menunjukkan sumber yang diselesaikan dan meminta Anda mengonfirmasi sebelum menambahkannya. Setelah marketplace ditambahkan, detail plugin membuka dan Anda memilih [cakupan instalasi](#install-a-plugin). Jika sumber cocok dengan marketplace yang sudah Anda tambahkan, Claude Code melewati konfirmasi dan membuka detail plugin di marketplace itu.

<h3 id="add-a-private-marketplace">
  Tambahkan marketplace pribadi
</h3>

Marketplace pribadi adalah satu di repository yang Anda butuhkan kredensial untuk clone, di GitHub atau host git apa pun. Anda menambahkannya dengan perintah `/plugin marketplace add` atau `claude plugin marketplace add` yang sama seperti yang publik. Claude Code mengklonnya dengan kredensial git yang sudah ada di mesin Anda dan tidak pernah meminta, jadi setiap cara koneksi memiliki persyaratan:

* **HTTPS**: pembantu kredensial git Anda berlaku, jadi akses yang Anda atur dengan `gh auth login`, Keychain macOS, atau `git-credential-store` berfungsi. Prompt interaktif ditekan, jadi host yang belum pernah Anda autentikasi gagal alih-alih meminta kata sandi.
* **SSH**: host harus sudah ada di file `known_hosts` Anda dan kunci harus berfungsi tanpa prompt passphrase, karena prompt host-fingerprint dan passphrase juga ditekan.
* **GitHub `owner/repo` shorthand**: Claude Code memeriksa apakah kunci SSH Anda mengautentikasi ke `github.com`, kemudian mengklonnya melalui SSH jika ya dan melalui HTTPS jika tidak. Atur [`CLAUDE_CODE_PLUGIN_PREFER_HTTPS=1`](/docs/id/env-vars#variables) untuk melewati pemeriksaan itu dan selalu mengklonnya melalui HTTPS.

Kredensial yang sama berlaku ketika Anda menjalankan `/plugin install`, `/plugin marketplace update`, dan `claude plugin update`.

Di host GitHub Enterprise Server, lihat [Plugin marketplaces on GHES](/docs/id/github-enterprise-server#plugin-marketplaces-on-ghes) untuk kredensial yang dibutuhkan setiap operasi.

Jika organisasi Anda mendaftarkan marketplace untuk Anda melalui pengaturan terkelola, Anda tidak menambahkannya sendiri. Lihat [Pre-install and require plugins](/docs/id/plugins/org#pre-install-and-require-plugins).

<h3 id="add-from-claude-ai">
  Tambahkan marketplace dari claude.ai
</h3>

Dalam sesi terminal di mana [plugin disinkronkan dari akun claude.ai Anda](/docs/id/plugins/loading#synced-plugins), claude.ai juga dapat mencantumkan marketplace plugin untuk Anda, seperti perpustakaan plugin organisasi Anda dan unggahan claude.ai Anda sendiri. Anda menambahkan salah satu dari ini dengan namanya daripada oleh sumber. Menambahkan marketplace dari claude.ai memerlukan Claude Code v2.1.273 atau lebih baru.

Tambahkan marketplace claude.ai dari panel `/plugin` atau dari shell Anda:

* **Inside a session**: jalankan `/plugin` dan buka tab **Marketplaces**, yang mencantumkan marketplace dari claude.ai. Pilih satu di sana untuk menambahkannya.
* **From your shell**: jalankan `claude plugin marketplace list`, yang mencetaknya di bagian `From claude.ai:`. Kemudian jalankan `claude plugin marketplace add` dengan bendera `--claudeai` dan nama yang ditampilkan dalam daftar.

Misalnya, perintah ini menambahkan marketplace bernama `claudeai-organization-library`:

```bash theme={null}
claude plugin marketplace add --claudeai claudeai-organization-library
```

Claude Code mendaftarkan marketplace di bawah nama lokal yang dimulai dengan `claudeai-`, berasal dari nama yang tercantum claude.ai. Misalnya, marketplace yang tercantum sebagai "Organization library" menjadi `claudeai-organization-library`. Instal plugin-nya dengan nama itu, misalnya dengan `claude plugin install <plugin>@claudeai-organization-library`.

Jika Anda keluar, atau masuk ke organisasi claude.ai yang berbeda, marketplace tetap dikonfigurasi tetapi tidak menunjukkan plugin, dan plugin yang sudah Anda instal darinya terus memuat.

Bagian `From claude.ai:` juga dapat mencantumkan marketplace berbasis git yang dibagikan melalui claude.ai, dan itu mencetak sumber untuk masing-masing. Tambahkan mereka dengan sumber itu seperti di [Add a marketplace](#add-a-marketplace), bukan dengan `--claudeai`.

<h2 id="manage-installed-plugins">
  Kelola plugin yang diinstal
</h2>

Tab **Installed** di `/plugin` mencantumkan plugin Anda dengan tindakan untuk mengaktifkan, menonaktifkan, memperbarui, atau mencopot masing-masing. Dalam sesi Claude Code, jalankan `/plugin` dan tekan **Tab** untuk mencapainya, atau jalankan `/plugin enable`, `/plugin disable`, atau `/plugin uninstall` untuk membuka panel dan membuat perubahan itu di sana. Plugin yang dinonaktifkan dikelompokkan di bawah header yang diciutkan di bagian bawah daftar. Gunakan kunci ini di daftar:

* Ketik untuk memfilter berdasarkan nama atau deskripsi.
* Tekan **Space** untuk mengaktifkan atau menonaktifkan plugin yang dipilih, dan **f** untuk menyukainya.
* Tekan **Enter** untuk membuka detail plugin. Menu di sana menawarkan **Disable plugin** atau **Enable plugin**, **Update now**, dan **Uninstall**. Plugin yang mengambil pengaturan juga menawarkan **Configure options**.

Tab juga dapat menunjukkan plugin di cakupan **Managed**. Organisasi Anda menginstal mereka melalui [pengaturan terkelola](/docs/id/settings#settings-files), dan Anda tidak dapat mengaktifkan, menonaktifkan, atau mencopot mereka di sini.

Untuk plugin yang disinkronkan yang organisasi Anda perlukan di claude.ai, lihat [Manage plugins synced from claude.ai](#manage-plugins-synced-from-claude-ai).

Ketika Anda menutup panel `/plugin` dengan perubahan tertunda yang Anda buat di dalamnya, Claude Code menjalankan `/reload-plugins` untuk Anda untuk menerapkannya. Jika reload akan [membatalkan prompt cache](/docs/id/prompt-caching#enabling-or-disabling-a-plugin), itu memperingatkan dan meninggalkan perubahan tertunda sebagai gantinya. Jalankan `/reload-plugins --force` untuk menerapkannya bagaimanapun.

<h3 id="manage-plugins-synced-from-claude-ai">
  Kelola plugin yang disinkronkan dari claude.ai
</h3>

Tab **Installed** di `/plugin` juga mencantumkan [plugin yang disinkronkan dari akun claude.ai Anda](/docs/id/plugins/loading#synced-plugins), dengan `synced` sebagai sumber mereka. Plugin yang disinkronkan muncul dalam sesi terminal di Claude Code v2.1.273 atau lebih baru.

* **Enable or disable**: gunakan tab **Installed**, kecuali organisasi Anda menandai plugin sebagai diperlukan.
* **Remove**: matikan plugin di claude.ai.

Ketika Claude Code menyinkronkan plugin yang ditambahkan, diperbarui, atau dihapus ke sesi interaktif, Anda melihat `Plugins changed. Run /reload-plugins to activate.` Jalankan `/reload-plugins` untuk memuat perubahan dalam sesi itu, atau biarkan untuk lain kali Anda memulai Claude Code.

<h3 id="uninstall-a-plugin-the-project-enables">
  Copot plugin yang diaktifkan proyek
</h3>

Ketika Anda memilih **Uninstall** untuk plugin yang diaktifkan repository ini `.claude/settings.json`, baik dari tab **Installed** atau dengan `/plugin uninstall`, Claude Code menanyakan apakah akan menonaktifkannya untuk Anda atau mencopotnya untuk semua orang:

* **Disable for me**: tekan **y**. Claude Code menulis `false` untuk plugin di `.claude/settings.local.json` Anda dan meninggalkannya diinstal untuk proyek.
* **Uninstall for everyone**: tekan **u**. Claude Code menghapus plugin dari `.claude/settings.json` bersama.

<h3 id="see-what-an-installed-plugin-adds-to-your-sessions">
  Lihat apa yang ditambahkan plugin yang diinstal ke sesi Anda
</h3>

Di shell Anda, jalankan `claude plugin details <name>` untuk plugin yang diinstal. Baris `Always-on` adalah jumlah token yang ditambahkan plugin ke setiap sesi di mana diaktifkan, dan baris per-komponen menunjukkan skill atau agent mana yang berkontribusi paling banyak. Untuk output lengkap dan apa arti setiap angka, lihat [Measure what a plugin costs](/docs/id/plugins/measure#measure-what-a-plugin-costs).

<h3 id="find-plugins-you-no-longer-use">
  Temukan plugin yang tidak lagi Anda gunakan
</h3>

Di tab **Installed** di `/plugin`, plugin yang Anda instal sendiri dan belum digunakan baru-baru ini muncul di bawah header **Not used recently**, dan detail setiap plugin menunjukkan baris **Last used**. Gunakan header itu dan baris itu untuk menemukan plugin yang masih menambahkan startup dan context cost, kemudian nonaktifkan atau copot.

<h3 id="plugins-with-dependencies">
  Plugin dengan dependensi
</h3>

Plugin dapat mendeklarasikan plugin lain yang bergantung padanya. Ketika Anda menginstal, menonaktifkan, atau mencopot plugin seperti itu dari marketplace, Claude Code bertindak pada dependensi itu juga:

* **Install**: Claude Code juga menginstal dan mengaktifkan dependensi yang dideklarasikan plugin di cakupan yang sama. Pesan kesuksesan mencantumnya.
* **Enable**: Claude Code juga mengaktifkan dependensi plugin yang diinstal tetapi dinonaktifkan. Jika dependensi yang dideklarasikan tidak diinstal, enable gagal dan pesan memberi tahu Anda untuk menginstalnya terlebih dahulu.
* **Disable**: ketika plugin yang diaktifkan lain masih membutuhkan yang Anda namai, Claude Code menolak dan mencetak perintah berantai yang menonaktifkan keduanya dalam urutan yang benar.
* **Uninstall**: dependensi yang diinstal otomatis tetap sampai Anda menjalankan `claude plugin prune` di shell Anda; lihat [plugin prune](/docs/id/plugins/cli-reference#plugin-prune).

Jika Anda memuat plugin dengan `--plugin-dir` sebagai gantinya, lihat [Test a plugin and its dependency locally](/docs/id/plugins/dependencies#test-a-plugin-and-its-dependency-locally).

<h3 id="manage-plugins-from-your-shell">
  Kelola plugin dari shell Anda
</h3>

Anda juga dapat mengelola plugin tanpa memulai sesi Claude Code. Di shell Anda, jalankan `claude plugin install`, `enable`, `disable`, atau `uninstall` sebagai perintah terminal biasa; mereka mengubah pengaturan yang sama yang dilakukan panel `/plugin`. Masing-masing mengambil `--scope` untuk menargetkan satu cakupan, dan menggunakan cakupan default ketika Anda menghilangkannya:

* `enable` dan `disable` bertindak pada cakupan paling spesifik yang pengaturannya sudah mencantumkan plugin.
* `install` dan `uninstall` bertindak pada cakupan pengguna.

Misalnya, perintah ini menonaktifkan dan mengaktifkan kembali plugin, kemudian mencopotnya di cakupan proyek:

```bash theme={null}
claude plugin disable formatter@your-org
claude plugin enable formatter@your-org
claude plugin uninstall formatter@your-org --scope project
```

<h2 id="keep-plugins-updated">
  Jaga plugin tetap terbaru
</h2>

Plugin memperbarui secara otomatis ketika marketplace yang mereka berasal dari memiliki auto-update diaktifkan. Setelah sesi dimulai, Claude Code menyegarkan marketplace itu dan memperbarui salinan on-disk plugin yang Anda instal darinya.

Sesi yang berjalan menyimpan versi yang sudah dimuat. Setelah pembaruan, Anda melihat `Plugin updated: <name> · Run /reload-plugins to apply`, dan sesi berikutnya memuat versi baru secara otomatis.

Ini adalah default auto-update untuk setiap jenis marketplace:

* **On by default**: `claude-plugins-official` dan [nama marketplace resmi](/docs/id/plugins/security#official-marketplace-names) lainnya kecuali `knowledge-work-plugins` dan `first-party-plugins`, ditambah [marketplace yang ditambahkan dari claude.ai](#add-from-claude-ai).
* **Off by default**: setiap marketplace lainnya, termasuk marketplace komunitas, marketplace pihak ketiga, dan marketplace pengembangan lokal.

Untuk kapan auto-update berjalan, plugin mana yang dilewati, dan variabel lingkungan yang mematikannya, lihat [When auto-update runs](/docs/id/plugins/loading#when-auto-update-runs).

<h3 id="turn-auto-update-on-or-off-for-a-marketplace">
  Aktifkan atau matikan auto-update untuk marketplace
</h3>

Dalam sesi Claude Code, jalankan `/plugin` dan buka tab **Marketplaces**. Pilih marketplace, kemudian pilih **Enable auto-update** atau **Disable auto-update**.

<h3 id="update-one-plugin-now">
  Perbarui satu plugin sekarang
</h3>

Dalam sesi, buka plugin di tab **Installed** di `/plugin` dan pilih **Update now**, atau di shell Anda jalankan `claude plugin update <plugin>@<marketplace>`.

<h3 id="auto-update-from-a-private-marketplace">
  Auto-update dari marketplace pribadi
</h3>

Untuk marketplace pribadi, lihat [What background auto-update does with credentials](/docs/id/plugins/host-marketplace#what-background-auto-update-does-with-credentials) untuk cara background auto-updates mengautentikasi melalui SSH dan HTTPS, dan [Troubleshoot plugins](/docs/id/plugins/troubleshooting#add-a-marketplace) untuk pesan yang Anda lihat ketika mereka gagal.

<h2 id="manage-marketplaces">
  Kelola marketplace
</h2>

Tab **Marketplaces** di `/plugin` mencantumkan setiap marketplace yang Anda daftarkan, bersama dengan sumbernya. Pilih satu untuk menjelajahi plugin-nya, perbarui daftarnya, aktifkan atau matikan auto-update, atau hapusnya.

Anda juga dapat mencantumkan, memperbarui, dan menghapus marketplace dengan perintah, dari shell Anda atau dalam sesi:

| Action                         | In your shell                             | Inside a session                    |
| :----------------------------- | :---------------------------------------- | :---------------------------------- |
| List marketplaces              | `claude plugin marketplace list`          | `/plugin marketplace list`          |
| Update a marketplace's listing | `claude plugin marketplace update <name>` | `/plugin marketplace update <name>` |
| Remove a marketplace           | `claude plugin marketplace remove <name>` | `/plugin marketplace remove <name>` |

Ketika Anda menghapus marketplace, Claude Code mencopot setiap plugin yang Anda instal darinya dan menghapus entri `enabledPlugins` mereka dari file pengaturan Anda. Tab **Marketplaces** menamai plugin itu sebelum meminta Anda mengonfirmasi.

<h2 id="next-steps">
  Langkah berikutnya
</h2>

* [Anthropic's marketplaces](/docs/id/plugins/anthropic-marketplaces): bagaimana marketplace resmi, komunitas, dan demo berbeda dan di mana untuk menjelajahi masing-masing
* [Plugin loading reference](/docs/id/plugins/loading): mengapa plugin dimuat, tidak dimuat, atau tidak berubah setelah pembaruan
* [Plugin security and trust](/docs/id/plugins/security): apa yang harus ditinjau sebelum Anda menginstal plugin dari marketplace yang tidak Anda kenal
* [Troubleshoot plugins](/docs/id/plugins/troubleshooting): instal dan pesan error marketplace dengan perbaikan mereka
* [Create a plugin](/docs/id/plugins/create): bangun milik Anda sendiri
