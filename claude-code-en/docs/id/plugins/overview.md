> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Ikhtisar plugin

> Pahami apa itu plugin Claude Code, kapan Anda membutuhkannya daripada skill mandiri atau server MCP, dan halaman mana yang harus dibaca untuk memasang atau membuat satu.

Plugin Claude Code adalah direktori skill, agent, hooks, server MCP, atau komponen lainnya yang Claude Code pasang dan muat sebagai satu unit. Sebagian besar plugin berasal dari marketplace, yang merupakan katalog yang mencantumkan plugin dan tempat mengambil masing-masing. Anda juga dapat memuat plugin dari folder yang diberikan seseorang kepada Anda, atau [membangun plugin Anda sendiri](/docs/id/plugins/create).

<Note>
  Jika Anda menggunakan chat claude.ai atau Cowork dan bukan Claude Code, lihat [Plugin di claude.ai dan di Cowork](https://claude.com/docs/plugins/overview).
</Note>

Untuk mencoba plugin sekarang, jalankan `/plugin` dalam sesi terminal Claude Code dan pasang satu dari tab **Discover**, yang mencantumkan plugin dari marketplace resmi Anthropic dan marketplace apa pun yang telah Anda tambahkan. Dari sana:

* [Pasang dan kelola plugin](/docs/id/plugins/install): langkah pemasangan lengkap, cakupan, dan permukaan lainnya
* [Buat plugin](/docs/id/plugins/create): bangun plugin Anda sendiri
* [Tentukan apakah Anda membutuhkan plugin](#decide-whether-you-need-a-plugin): apakah plugin adalah alat yang tepat untuk apa yang Anda inginkan

<h2 id="understand-what-a-plugin-is">
  Pahami apa itu plugin
</h2>

Plugin adalah direktori komponen, biasanya dengan manifes. Manifes, file JSON di `.claude-plugin/plugin.json`, memberikan nama plugin dan dapat menambahkan versi, deskripsi, dan [metadata](/docs/id/plugins/manifest-reference) lainnya. Komponen adalah apa yang ditambahkan plugin ke Claude Code, seperti:

* [**Skills**](/docs/id/plugins/components#skills): instruksi `SKILL.md` yang Claude muat saat relevan, dan yang juga dapat Anda jalankan sebagai perintah
* [**Agents**](/docs/id/plugins/components#agents): definisi subagent yang dapat didelegasikan Claude
* [**Hooks**](/docs/id/plugins/components#hooks): perintah yang dijalankan Claude Code pada titik dalam siklus hidupnya, seperti setelah setiap edit
* [**MCP servers**](/docs/id/plugins/components#mcp-servers): server alat yang terhubung dengan Claude Code saat plugin diaktifkan

Diagram ini menunjukkan plugin bernama `my-plugin` yang menyimpan satu dari masing-masing komponen tersebut, dan apa yang Anda dapatkan dari setiap file setelah plugin dimuat.

<img src="https://mintcdn.com/claude-code/2Q_GtOEovg5qaBem/images/plugin-directory.svg?fit=max&auto=format&n=2Q_GtOEovg5qaBem&q=85&s=f623b64e82713b830e48174f0a922888" className="dark:hidden" alt="Diagram dalam dua kolom yang digabungkan oleh lima panah lurus. Di sebelah kiri, direktori plugin bernama my-plugin, menyimpan manifes di .claude-plugin/plugin.json, skills/review/SKILL.md, agents/reviewer.md, hooks/hooks.json, .mcp.json, dan komponen lainnya. Di sebelah kanan, apa yang diberikan setiap file dalam sesi Anda: manifes menetapkan nama plugin, my-plugin; skill berjalan sebagai /my-plugin:review; file agent adalah subagent yang dapat didelegasikan Claude; file hooks menyimpan hook yang berjalan pada acara siklus hidup; dan .mcp.json menambahkan server MCP yang memberikan alat Claude." width="760" height="336" data-path="images/plugin-directory.svg" />

<img src="https://mintcdn.com/claude-code/2Q_GtOEovg5qaBem/images/plugin-directory-dark.svg?fit=max&auto=format&n=2Q_GtOEovg5qaBem&q=85&s=17ee2bd45b63154fcc148ae1d1f736d8" className="hidden dark:block" alt="Diagram dalam dua kolom yang digabungkan oleh lima panah lurus. Di sebelah kiri, direktori plugin bernama my-plugin, menyimpan manifes di .claude-plugin/plugin.json, skills/review/SKILL.md, agents/reviewer.md, hooks/hooks.json, .mcp.json, dan komponen lainnya. Di sebelah kanan, apa yang diberikan setiap file dalam sesi Anda: manifes menetapkan nama plugin, my-plugin; skill berjalan sebagai /my-plugin:review; file agent adalah subagent yang dapat didelegasikan Claude; file hooks menyimpan hook yang berjalan pada acara siklus hidup; dan .mcp.json menambahkan server MCP yang memberikan alat Claude." width="760" height="336" data-path="images/plugin-directory-dark.svg" />

Untuk setiap jenis komponen yang dapat disimpan plugin, dengan contoh masing-masing, lihat [Plugin components](/docs/id/plugins/components). Untuk melihat di mana setiap bagian berada dalam direktori plugin, gunakan [plugin explorer](/docs/id/plugins/components#explore-the-plugin-directory) di halaman itu.

<h3 id="decide-whether-you-need-a-plugin">
  Tentukan apakah Anda membutuhkan plugin
</h3>

Skills, subagent, hooks, dan server MCP semuanya bekerja sendiri, tanpa plugin. Skill yang Anda simpan di `~/.claude/skills/`, misalnya, tersedia di setiap proyek di mesin Anda. Untuk menyiapkan satu sendiri, lihat [Skills](/docs/id/skills), [Subagents](/docs/id/sub-agents), [Hooks](/docs/id/hooks-guide), atau [MCP](/docs/id/mcp).

Gunakan plugin ketika Anda ingin beberapa skill, subagent, hooks, atau server MCP dikemas sebagai satu unit. Pasang satu untuk mendapatkan setup yang dibangun orang lain, dengan satu perintah dan pembaruan dari marketplace-nya. Buat satu untuk memberikan setup Anda sendiri kepada rekan kerja, pasang di banyak proyek, atau terbitkan rilis yang diberi versi.

<h3 id="what-an-enabled-plugin-adds-to-your-sessions">
  Apa yang ditambahkan plugin yang diaktifkan ke sesi Anda
</h3>

Plugin yang diaktifkan adalah bagian dari setiap sesi, bukan hanya sesi tempat Anda menggunakannya. Itu memiliki beberapa konsekuensi yang patut diketahui sebelum Anda memasang satu:

* **Konteks dan penggunaan**: untuk setiap skill, agent, dan perintah yang [dapat dijalankan Claude sendiri](/docs/id/skills#control-who-invokes-a-skill), nama dan deskripsi ada dalam konteks Claude di setiap giliran sehingga Claude tahu itu ada. Token tersebut dihitung menuju penggunaan Anda dan meninggalkan lebih sedikit ruang di [jendela konteks](/docs/id/context-window) bahkan dalam sesi di mana tidak ada yang dari plugin berjalan. Teks lengkap skill atau agent dimuat hanya saat digunakan. Apa yang ditambahkan server MCP plugin per giliran mengikuti [pencarian alat MCP](/docs/id/mcp#scale-with-mcp-tool-search).
* **Proses**: server MCP yang didefinisikan plugin berjalan bersama setiap sesi tempat plugin diaktifkan, dan hook-nya menyala pada acara mereka.
* **Izin**: apa yang dijalankan plugin, dijalankan sebagai Anda. Lihat [Plugin security and trust](/docs/id/plugins/security) untuk apa yang harus ditinjau terlebih dahulu.

Anda dapat memeriksa jejak plugin di setiap tahap:

* **Sebelum Anda memasang**: buka plugin dari tab **Marketplaces** di `/plugin`. Plugin di marketplace resmi Anthropic menunjukkan perkiraan **Context cost** di sana.
* **Setelah Anda memasang**: [Measure what a plugin costs](/docs/id/plugins/measure#measure-what-a-plugin-costs) menunjukkan cara membaca jejak plugin, dan grup **Not used recently** tab **Installed** mencantumkan plugin yang dapat Anda matikan.
* **Untuk menghentikannya tanpa mencopot**: nonaktifkan plugin dengan `/plugin` atau, di shell Anda, `claude plugin disable`. Lihat [Manage installed plugins](/docs/id/plugins/install#manage-installed-plugins).

<h2 id="get-plugins-from-a-marketplace">
  Dapatkan plugin dari marketplace
</h2>

Marketplace adalah repositori atau direktori dengan file `.claude-plugin/marketplace.json` yang mencantumkan plugin dan tempat mengambil masing-masing. Ini adalah katalog, bukan toko yang dihosting. Anda menambahkan marketplace sekali, kemudian memasang plugin darinya berdasarkan nama, seperti `commit-commands@claude-plugins-official`.

<Note>
  Marketplace plugin bukanlah [Claude Marketplace](https://claude.com/marketplace). Claude Marketplace adalah situs web di claude.com/marketplace tempat Anda menjelajahi plugin, konektor, produk mitra, dan mitra layanan. Ini bukan marketplace yang Anda tambahkan dengan `/plugin marketplace add`.
</Note>

Claude Code menambahkan marketplace resmi Anthropic pertama kali Anda memulai sesi terminal interaktif, kecuali [kebijakan terkelola](/docs/id/plugins/org#allow-the-official-marketplace-and-your-own) memblokir. Claude Code tidak menambahkan marketplace lain sendiri, termasuk marketplace komunitas dan demo Anthropic. Untuk membedakan tiga marketplace Anthropic, baca [Anthropic's marketplaces](/docs/id/plugins/anthropic-marketplaces). Untuk melihat apa yang dicantumkan marketplace resmi, buka tab **Discover** dari `/plugin` dalam sesi atau jelajahi [Claude Marketplace](https://claude.com/marketplace/plugins).

Diagram ini menunjukkan jalur dari marketplace ke sesi Anda. Marketplace mencantumkan plugin, Anda memasang plugin itu, dan Claude Code memuat komponen-komponennya.

<img src="https://mintcdn.com/claude-code/2Q_GtOEovg5qaBem/images/plugins-model.svg?fit=max&auto=format&n=2Q_GtOEovg5qaBem&q=85&s=4196344954b7c2e27fc0bd6a9a1113a1" className="dark:hidden" alt="Diagram jalur marketplace dalam tiga kotak, kiri ke kanan. Marketplace, katalog plugin, mencantumkan plugin. Plugin adalah satu direktori yang dipasang sebagai unit, menyimpan skill, agent, hooks, server MCP, dan komponen lainnya. Anda memasang plugin ke Claude Code, yang memuat komponen-komponennya." width="760" height="252" data-path="images/plugins-model.svg" />

<img src="https://mintcdn.com/claude-code/2Q_GtOEovg5qaBem/images/plugins-model-dark.svg?fit=max&auto=format&n=2Q_GtOEovg5qaBem&q=85&s=f6cdefe1fc05daf3b253d26e9f3f70f6" className="hidden dark:block" alt="Diagram jalur marketplace dalam tiga kotak, kiri ke kanan. Marketplace, katalog plugin, mencantumkan plugin. Plugin adalah satu direktori yang dipasang sebagai unit, menyimpan skill, agent, hooks, server MCP, dan komponen lainnya. Anda memasang plugin ke Claude Code, yang memuat komponen-komponennya." width="760" height="252" data-path="images/plugins-model-dark.svg" />

[Install and manage plugins](/docs/id/plugins/install#install-a-plugin) memiliki langkah pemasangan untuk setiap tempat Anda menjalankan Claude Code. Saat Anda mengembangkan plugin, Anda tidak memerlukan marketplace: muat langsung dari foldernya dengan `--plugin-dir`, seperti yang ditunjukkan [Develop without a marketplace](/docs/id/plugins/create#develop-without-a-marketplace).

<h3 id="make-an-installed-plugin-available-in-your-session">
  Buat plugin yang dipasang tersedia dalam sesi Anda
</h3>

Sebelum plugin yang Anda pasang memberikan Anda skill yang dapat Anda jalankan, plugin harus ada di setiap lapisan ini:

* **Settings**: pengaturan Anda mencantumkan marketplace yang telah Anda tambahkan dan plugin yang diaktifkan.
* **Disk**: `~/.claude/plugins/` menyimpan apa yang telah diambil dan dipasang Claude Code.
* **Session**: plugin dimuat saat startup, atau ketika Anda [reload plugins](/docs/id/plugins/loading#check-which-stage-a-plugin-reached).

Baca [Plugin loading reference](/docs/id/plugins/loading) untuk aturan di setiap lapisan, termasuk file pengaturan mana yang memiliki prioritas dan di mana file berada di disk.

<h2 id="tell-anthropic’s-marketplaces-from-third-party-ones">
  Bedakan marketplace Anthropic dari yang pihak ketiga
</h2>

Nama marketplace menempatkannya di salah satu dari tiga tingkat. Claude Code menerima nama resmi dan komunitas hanya untuk marketplace yang bersumber dari repositori `github.com/anthropics/`:

* **Official**: marketplace dengan salah satu [nama marketplace resmi](/docs/id/plugins/security#official-marketplace-names) Anthropic, termasuk `claude-plugins-official` dan marketplace demo `claude-code-plugins`.
* **Community**: marketplace dengan salah satu nama komunitas Anthropic, seperti `claude-community`. [Identify Anthropic's marketplaces by name](/docs/id/plugins/security#marketplace-tiers) mencantumkan mereka.
* **Third-party**: setiap marketplace lainnya. Marketplace yang diterbitkan rekan kerja atau organisasi Anda adalah pihak ketiga.

Apa pun tingkatnya, plugin yang Anda pasang dapat menjalankan kode dengan hak istimewa pengguna Anda. Baca [Plugin security and trust](/docs/id/plugins/security) untuk cara meninjau plugin sebelum Anda memasangnya.

Melalui [pengaturan terkelola](/docs/id/settings#settings-files), organisasi dapat membuat daftar putih atau memblokir marketplace, memaksa pemasangan plugin, dan mematikan pemuatan hanya-sesi. Baca [Manage plugins for your organization](/docs/id/plugins/org) untuk kontrol tersebut.

<h2 id="understand-install-scopes">
  Pahami cakupan pemasangan
</h2>

Ketika Anda memasang plugin, Anda memilih cakupan, dan cakupan menentukan siapa plugin diaktifkan untuk:

* **User scope**: diaktifkan untuk Anda di setiap proyek di komputer ini
* **Project scope**: diaktifkan untuk semua orang yang bekerja di repositori ini, melalui `.claude/settings.json` yang berkomitmen. Setiap kolaborator masih [memasangnya di mesin mereka sendiri](/docs/id/plugins/loading#enabled-in-project-settings-but-not-installed)
* **Local scope**: diaktifkan untuk Anda di repositori ini saja

Plugin yang Anda pasang di cakupan pengguna di terminal, sesi lokal aplikasi desktop, atau ekstensi VS Code tersedia di dua lainnya di komputer itu, karena ketiganya membaca file pengaturan yang sama. Lihat [Choose an install scope](/docs/id/plugins/install#choose-an-install-scope) untuk cara memilih satu.

Sesi cloud, termasuk satu di browser di claude.ai/code, tidak memuat plugin dalam pengaturan lokal Anda. Untuk langkah pemasangan di terminal, VS Code, dan aplikasi desktop, dan untuk apa yang dimuat sesi cloud, lihat [Install a plugin](/docs/id/plugins/install#install-a-plugin).

<Note>
  Format plugin yang sama juga dipasang di claude.ai dan di Cowork, di mana set komponen yang berbeda dimuat. Untuk permukaan tersebut, lihat [Plugins on claude.ai and in Cowork](https://claude.com/docs/plugins/overview) di claude.com.
</Note>

<h2 id="next-steps">
  Langkah berikutnya
</h2>

Sebagian besar orang mulai dengan memasang plugin dari marketplace resmi Anthropic, yang ditambahkan Claude Code pertama kali Anda memulai sesi terminal interaktif. Jalankan `/plugin` dalam sesi terminal untuk menjelajahinya, atau ikuti [Install and manage plugins](/docs/id/plugins/install), yang juga mencakup aplikasi desktop dan VS Code. Untuk melihat apa yang ada di marketplace itu sebelum Anda membuka Claude Code, jelajahi [Claude Marketplace](https://claude.com/marketplace/plugins) di web.

Untuk membangun plugin Anda sendiri, [Create a plugin](/docs/id/plugins/create) dimulai dengan direktori kosong dan berakhir dengan plugin yang berfungsi.

Setelah Anda memasang atau membangun plugin, halaman-halaman ini mencakup apa yang akan datang:

* **Bagikan apa yang Anda bangun**: [Publish and distribute a plugin](/docs/id/plugins/publish)
* **Periksa apakah itu berfungsi dan digunakan**: [Test plugins with evals](/docs/id/plugin-evals) dan [Measure plugin cost and usage](/docs/id/plugins/measure)
* **Jalankan marketplace untuk tim Anda**: [Create a marketplace](/docs/id/plugins/create-marketplace), kemudian [Host and maintain a marketplace](/docs/id/plugins/host-marketplace)
* **Tetapkan kebijakan plugin untuk organisasi**: [Manage plugins for your organization](/docs/id/plugins/org)
* **Perbaiki masalah**: [Troubleshoot plugins](/docs/id/plugins/troubleshooting)
