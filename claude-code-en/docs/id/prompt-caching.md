> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Bagaimana Claude Code menggunakan prompt caching

> Claude Code mengelola prompt caching secara otomatis. Lihat mengapa perubahan model memicu giliran tanpa cache yang lambat, berapa biaya `/compact`, mengapa pengeditan CLAUDE.md tidak berlaku di tengah sesi, dan cara memeriksa tingkat cache hit Anda.

Prompt caching membuat Claude Code lebih cepat dan lebih hemat biaya. Tanpa caching, API akan memproses ulang riwayat lengkap Anda pada setiap giliran. Dengan caching, API menggunakan kembali apa yang sudah diproses, menagih pembacaan ulang pada [tingkat token cache](https://platform.claude.com/docs/en/about-claude/pricing), dan hanya memproses sepenuhnya apa yang berubah.

Claude Code menangani prompt caching untuk Anda, kecuali Anda [menonaktifkannya](#disable-prompt-caching). Masih berguna untuk mengetahui cara kerja prompt caching, karena beberapa tindakan membatalkan cache dan membuat respons berikutnya lebih lambat dan lebih mahal saat membangun kembali. Halaman ini mencakup tindakan mana yang melakukan itu, mengapa beberapa pengaturan menunggu restart untuk diterapkan, dan cara memeriksa kinerja cache ketika penggunaan terlihat tinggi.

<h2 id="how-the-cache-is-organized">
  Bagaimana cache diorganisir
</h2>

Setiap kali Anda mengirim pesan di Claude Code, sistem membuat permintaan API baru. Model tidak mengingat apa pun di antara permintaan, jadi Claude Code mengirim ulang konteks lengkap: prompt sistem, konteks proyek Anda, setiap pesan dan hasil alat sebelumnya, dan pesan baru Anda. Konten baru ditambahkan di akhir, yang berarti sebagian besar dari setiap permintaan identik dengan yang sebelumnya. Prompt caching adalah cara API menghindari pemrosesan ulang bagian yang tidak berubah.

API melakukan cache dengan mencocokkan awal setiap permintaan, yang disebut prefix, terhadap konten yang baru saja diproses. Pada giliran normal, prefix adalah seluruh permintaan sebelumnya dan hanya pertukaran terbaru yang baru. Kecocokan bersifat tepat, jadi perubahan di mana pun dalam prefix menghitung ulang semuanya setelahnya. Tidak ada caching per-file atau per-segment. Lihat [bagaimana prompt caching bekerja](https://platform.claude.com/docs/en/build-with-claude/prompt-caching#how-prompt-caching-works) dalam referensi API untuk mekanisme yang mendasarinya.

<img src="https://mintcdn.com/claude-code/VbDJw--l6T9a9Wvm/images/prompt-caching-prefix.svg?fit=max&auto=format&n=VbDJw--l6T9a9Wvm&q=85&s=f2e8f0b8298a50305fe428ca3f1d1594" className="dark:hidden" alt="Empat giliran ditampilkan sebagai batang horizontal yang berkembang. Permintaan setiap giliran berisi semuanya dari giliran sebelumnya ditambah pertukaran terbaru ditambahkan di akhir. Pada giliran dua dan tiga, prefix yang tidak berubah dibaca dari cache dan hanya pertukaran baru yang diproses. Pada giliran empat, prompt sistem berubah, jadi prefix tidak lagi cocok dan seluruh permintaan diproses ulang dan ditulis." width="720" height="454" data-path="images/prompt-caching-prefix.svg" />

<img src="https://mintcdn.com/claude-code/_xqph1dUOslCOwsj/images/prompt-caching-prefix-dark.svg?fit=max&auto=format&n=_xqph1dUOslCOwsj&q=85&s=297dc1c639f0915cae858d0c4b6f3be5" className="hidden dark:block" alt="Empat giliran ditampilkan sebagai batang horizontal yang berkembang. Permintaan setiap giliran berisi semuanya dari giliran sebelumnya ditambah pertukaran terbaru ditambahkan di akhir. Pada giliran dua dan tiga, prefix yang tidak berubah dibaca dari cache dan hanya pertukaran baru yang diproses. Pada giliran empat, prompt sistem berubah, jadi prefix tidak lagi cocok dan seluruh permintaan diproses ulang dan ditulis." width="720" height="454" data-path="images/prompt-caching-prefix-dark.svg" />

Untuk mendapatkan hasil maksimal dari pencocokan prefix, Claude Code mengurutkan setiap permintaan sehingga konten yang jarang berubah di antara giliran muncul terlebih dahulu:

| Layer          | Konten                                 | Berubah ketika                                      |
| -------------- | -------------------------------------- | --------------------------------------------------- |
| Prompt sistem  | Instruksi inti, definisi alat          | Set definisi alat yang dimuat berubah               |
| Konteks proyek | CLAUDE.md, auto memory, unscoped rules | Sesi dimulai, atau setelah `/clear` atau `/compact` |
| Percakapan     | Pesan Anda, respons Claude, hasil alat | Setiap giliran                                      |

Perubahan pada layer percakapan meninggalkan prompt sistem dan konteks proyek di-cache. Perubahan pada prompt sistem membatalkan semuanya, karena semua konten selanjutnya sekarang berada di belakang prefix yang berbeda. Kolom ketiga memberikan pemicu umum daripada daftar lengkap, dan bagian di bawah mencakup set lengkap.

Aturan pencocokan prefix menjelaskan sebagian besar perilaku di halaman ini. [Plan mode](/docs/id/permission-modes#analyze-before-you-edit-with-plan-mode) dan [skill loading](/docs/id/skills), misalnya, menambahkan instruksi mereka sebagai pesan percakapan, jadi prefix yang di-cache tetap utuh.

Dua pengaturan tidak muncul dalam tabel layer tetapi masih mempengaruhi apa yang tetap di-cache:

* **Model**: setiap model memiliki cache-nya sendiri. Mengganti model menghitung ulang seluruh permintaan bahkan ketika kontennya identik. Lihat [Switching models](#switching-models) di bawah.
* **Effort level**: pada sebagian besar model, setiap effort level memiliki cache-nya sendiri, jadi mengubah effort di tengah sesi menghitung ulang seluruh permintaan. Pada Opus 5.5 dan Fable 5.1 dengan API key atau langganan Claude, cache tetap utuh secara default. Lihat [Changing effort level](#changing-effort-level) di bawah.

<Tip>
  Pilih model dan effort level Anda di awal sesi, kemudian simpan `/compact` untuk istirahat alami di antara tugas. Semakin sedikit perubahan yang Anda buat di tengah tugas, semakin tinggi cache hit rate Anda.
</Tip>

<h3 id="where-the-cache-lives">
  Tempat cache berada
</h3>

Caching terjadi di sisi server, dalam infrastruktur apa pun yang melayani model Anda. Tempat itu tergantung pada cara Anda melakukan autentikasi:

* **API key, langganan Claude, atau [Claude Platform on AWS](/docs/id/claude-platform-on-aws)**: cache berada di infrastruktur Anthropic, diakses melalui [Claude API](https://platform.claude.com/docs)
* **Amazon Bedrock atau Agent Platform Google Cloud**: cache berada di infrastruktur serving penyedia cloud Anda
* **Microsoft Foundry**: tergantung pada [hosting option](https://platform.claude.com/docs/en/build-with-claude/claude-in-microsoft-foundry#hosting-options) deployment. Deployment yang di-host di Azure dilayani di infrastruktur Azure; deployment yang di-host di Anthropic dilayani di infrastruktur Anthropic
* **Custom `ANTHROPIC_BASE_URL` atau [LLM gateway](/docs/id/llm-gateway)**: cache berada di mana pun permintaan Anda diteruskan, dan apakah caching bekerja tergantung pada gateway

Claude Code juga menambahkan konteks sistem di tengah percakapan, seperti pemberitahuan perubahan file, dan menandai blok itu untuk caching di setiap penyedia dan koneksi kecuali Anda menetapkan [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS`](/docs/id/llm-gateway-protocol#disable-pre-release-capabilities), dalam hal ini blok itu dikirim tanpa cache.

Di endpoint penyedia sendiri, Amazon Bedrock dan [Mantle endpoint](/docs/id/amazon-bedrock#use-the-mantle-endpoint)-nya, Agent Platform Google Cloud, dan Microsoft Foundry melakukan cache pada blok dengan cara yang sama seperti Claude API.

Ketika permintaan Anda melewati [LLM gateway](/docs/id/llm-gateway), custom `ANTHROPIC_BASE_URL`, atau cloud provider base-URL override seperti [`ANTHROPIC_BEDROCK_BASE_URL`](/docs/id/env-vars), apa yang tetap di-cache tergantung pada cara gateway menangani [`cache_control` markers](https://platform.claude.com/docs/en/build-with-claude/prompt-caching#explicit-cache-breakpoints) yang dikirim Claude Code:

* **Meneruskannya tanpa perubahan**: blok dan percakapan Anda melakukan cache dengan cara yang sama seperti di endpoint penyedia sendiri.
* **Menolak permintaan yang ditandai dengan error `400` yang menyebutkan `cache_control`**: Claude Code mengirim ulang permintaan dengan marker dipindahkan dari blok dan ke pesan percakapan terakhir Anda, dan menyimpannya di sana untuk sisa percakapan. Blok ditagih sebagai uncached input; percakapan Anda tetap di-cache.
* **Menghapus marker sambil mengembalikan kesuksesan**: seluruh riwayat percakapan Anda ditagih sebagai uncached input di setiap giliran. Gateway yang mengonversi konten sistem bentuk blok ke string biasa menghapus marker dengan cara yang sama.

Untuk apa yang disimpan dan diproses setiap penyedia, lihat [data usage](/docs/id/data-usage). Di mana pun cache berada, entri kedaluwarsa setelah periode tidak aktif, dan [Cache lifetime](#cache-lifetime) di bawah mencakup TTL dan cara memperpanjangnya.

<h2 id="actions-that-invalidate-the-cache">
  Tindakan yang membatalkan cache
</h2>

Tindakan-tindakan ini menyebabkan permintaan berikutnya kehilangan sebagian atau seluruh cache. Anda akan melihat satu putaran yang lebih lambat dan lebih mahal, setelah itu prefiks baru akan di-cache. Sebagian besar dari tindakan ini dapat dihindari di tengah tugas setelah Anda mengetahui bahwa tindakan tersebut memiliki biaya. Pergantian model dapat terasa gratis sampai Anda memperhatikan putaran yang lebih lambat setelahnya.

* [Switching models](#switching-models)
* [Changing effort level](#changing-effort-level)
* [Turning on fast mode](#turning-on-fast-mode)
* [Connecting or disconnecting an MCP server](#connecting-or-disconnecting-an-mcp-server)
* [Enabling or disabling a plugin](#enabling-or-disabling-a-plugin)
* [Denying an entire tool](#denying-an-entire-tool)
* [Compacting the conversation](#compacting-the-conversation)
* [Accumulating many images](#accumulating-many-images)
* [Upgrading Claude Code](#upgrading-claude-code)

<h3 id="switching-models">
  Switching models
</h3>

Setiap model memiliki cache-nya sendiri. Beralih dengan [`/model`](/docs/id/model-config#setting-your-model) berarti permintaan berikutnya membaca seluruh riwayat percakapan tanpa cache hits, meskipun kontennya identik.

Ketika Anda menjalankan `/model` di terminal, Claude Code meminta Anda untuk mengonfirmasi pergantian hanya saat cache masih hangat dan model baru bukan yang menghasilkan respons terakhir. Cache tetap hangat selama satu [cache TTL](#cache-lifetime) setelah Claude Code terakhir mengirim permintaan dalam percakapan ini atau Claude terakhir merespons. Setelah waktu itu berlalu, cache telah kedaluwarsa, jadi Claude Code beralih tanpa bertanya.

Sebelum v2.1.238, Claude Code tidak memeriksa cache TTL dan bertanya bahkan setelah cache telah kedaluwarsa.

Anda juga dapat memerlukan konfirmasi ini atau melewatkannya dengan [PreModelSwitch hook](/docs/id/hooks#premodelswitch-decision-control).

[`opusplan` model setting](/docs/id/model-config#opusplan-model-setting) diselesaikan ke Opus selama plan mode dan Sonnet selama eksekusi, jadi setiap toggle plan-mode adalah pergantian model dan memulai cache segar.

[Automatic model fallback](/docs/id/model-config#automatic-model-fallback) pada model Fable, Opus 5.5, dan Opus 5 juga merupakan pergantian model. Ketika pengklasifikasi keamanan menandai permintaan dalam kategori yang memiliki model fallback, Claude Code menjalankan kembali permintaan pada model tersebut dan sesi berlanjut di sana.

Ketika frontmatter skill atau command menamai [`model`](/docs/id/skills#frontmatter-reference) selain model saat ini sesi, putaran itu juga merupakan pergantian model: permintaan berikutnya membaca seluruh riwayat percakapan tanpa cache hits. Model sesi dilanjutkan pada prompt Anda berikutnya. Skill `context: fork` menetapkan [model subagent yang di-fork](/docs/id/skills#run-skills-in-a-subagent) sebagai gantinya.

<h3 id="changing-effort-level">
  Changing effort level
</h3>

Pada sebagian besar model, mengubah [effort level](/docs/id/model-config#adjust-effort-level) di tengah sesi berarti permintaan berikutnya membaca seluruh riwayat percakapan tanpa cache hits. Saat cache masih hangat, Claude Code meminta Anda untuk mengonfirmasi perubahan terlebih dahulu.

Pada Opus 5.5 dan Fable 5.1 dengan API key atau langganan Claude, mengubah effort menjaga cache, dan Claude Code menerapkan level baru tanpa bertanya. Ini tidak berlaku pada Amazon Bedrock, Google Cloud's Agent Platform, atau [Claude apps gateway](/docs/id/claude-apps-gateway), atau ketika Anda menetapkan [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS`](/docs/id/llm-gateway-protocol#disable-pre-release-capabilities) atau organisasi Anda memiliki konfigurasi HIPAA.

Sebelum v2.1.260, mengubah effort pada Fable 5.1 dengan API key atau langganan Claude juga membatalkan cache.

<h3 id="turning-on-fast-mode">
  Turning on fast mode
</h3>

Mengaktifkan [fast mode](/docs/id/fast-mode) menambahkan header permintaan yang merupakan bagian dari cache key, jadi permintaan pertama yang Claude Code kirim dengan fast mode aktif membaca seluruh riwayat percakapan tanpa cache hits. Claude Code menetapkan header itu sekali ketika putaran dimulai dan menyimpannya untuk seluruh putaran, jadi ketika Anda mengaktifkan fast mode saat Claude sedang bekerja, cache miss dari header terjadi pada permintaan pertama putaran Anda berikutnya. Token input yang tidak di-cache tersebut ditagih dengan [fast mode rates](/docs/id/fast-mode#understand-the-cost-tradeoff), itulah mengapa mengaktifkannya di awal sesi biaya lebih rendah daripada mengaktifkannya jauh ke dalam sesi yang panjang. Jika model saat ini Anda tidak mendukung fast mode, mengaktifkan fast mode juga [beralih model Anda](#switching-models), dan pergantian itu memulai cache segar pada permintaan berikutnya dalam putaran yang sedang berjalan.

Biaya berlaku sekali per percakapan. Setelah putaran fast mode pertama, Claude Code terus mengirim header dan hanya memvariasikan pengaturan kecepatan permintaan, yang bukan bagian dari cache key. Mematikan fast mode, [fallback otomatis ke kecepatan standar](/docs/id/fast-mode#handle-rate-limits) setelah rate limit, dan mengaktifkannya kembali nanti semua menjaga cache. Jika Anda [kehabisan kredit penggunaan](/docs/id/fast-mode#handle-rate-limits) di tengah sesi, Claude Code mencoba kembali setiap permintaan fast mode yang ditolak dengan kecepatan standar dengan cara yang sama, jadi fallback ini juga menjaga cache. `/clear` dan `/compact` mengatur ulang ini, karena mereka membangun kembali cache pada titik-titik tersebut bagaimanapun.

<h3 id="connecting-or-disconnecting-an-mcp-server">
  Connecting or disconnecting an MCP server
</h3>

Definisi tool berada di lapisan system prompt, jadi cache membatalkan ketika set definisi tool dalam permintaan berubah antar putaran. Mengalihkan [advisor tool](/docs/id/advisor) adalah pengecualian: definisinya berada setelah breakpoint cache, jadi mengaktifkan atau menonaktifkan `/advisor` menjaga prefiks yang di-cache tetap utuh. Apakah perubahan [MCP server](/docs/id/mcp) melakukan ini tergantung pada apakah toolnya ditangguhkan oleh [tool search](/docs/id/mcp#scale-with-mcp-tool-search) atau dimuat ke dalam prefiks:

* **Deferred tools**, default pada model yang didukung: server yang terhubung, terputus, atau mengubah daftar toolnya hanya menambahkan konten baru dan tidak mengganggu apa pun yang sudah di-cache.
* **Tools loaded into the prefix**: perubahan apa pun pada mereka membatalkan cache. Ini terjadi ketika [tool search tidak tersedia atau dinonaktifkan](/docs/id/mcp#configure-tool-search), seperti pada model Google Cloud's Agent Platform lebih awal dari generasi Claude 4.5, dengan gateway `ANTHROPIC_BASE_URL` kustom, atau pada [deployment Microsoft Foundry yang dihosting di Azure](https://platform.claude.com/docs/en/build-with-claude/claude-in-microsoft-foundry#hosting-options) setelah Claude Code mendeteksi bahwa deployment menolak tool search. Ini juga terjadi untuk server atau tool yang ditandai [`alwaysLoad`](/docs/id/mcp#exempt-a-server-from-deferral), dan untuk definisi yang disimpan di depan oleh [threshold-based loading](/docs/id/mcp#configure-tool-search).

Ketika tools dimuat ke dalam prefiks, penyebab paling umum dari pembatalan adalah server yang terhubung atau terputus di tengah sesi, yang dapat terjadi tanpa tindakan apa pun dari pihak Anda: proses server stdio keluar, sesi HTTP kedaluwarsa, atau server [reconnects automatically after a transient failure](/docs/id/mcp#automatic-reconnection). Server yang terhubung juga dapat mendorong [dynamic tool update](/docs/id/mcp#dynamic-tool-updates) yang mengubah daftar toolnya.

Mengedit konfigurasi MCP Anda tidak dengan sendirinya mengubah cache. Konfigurasi baru hanya berlaku setelah restart, yaitu ketika server terhubung atau terputus.

<h3 id="enabling-or-disabling-a-plugin">
  Enabling or disabling a plugin
</h3>

Ketika Anda mengaktifkan atau menonaktifkan [plugin](/docs/id/plugins/overview), apa yang perubahan biaya tergantung pada jenis komponen apa yang disediakan plugin. Kasus-kasus di bawah mencakup setiap jenis komponen, kapan Claude Code menerapkan perubahan, dan apa yang terjadi ketika Anda menonaktifkan plugin lagi dalam sesi yang sama.

<h4 id="plugin-components-that-keep-the-cache">
  Plugin components that keep the cache
</h4>

Claude Code tidak pernah membatalkan cache untuk skills, commands, agents, hooks, monitors, atau themes plugin. Ini menambahkan konten mereka setelah percakapan yang ada, jadi permintaan berikutnya membayar untuk konten itu dan masih membaca semuanya sebelumnya dari cache.

<h4 id="plugins-that-provide-mcp-servers">
  Plugins that provide MCP servers
</h4>

Ketika Anda mengaktifkan atau menonaktifkan plugin yang menyediakan [MCP servers](/docs/id/plugins/components#mcp-servers), Claude Code mengikuti aturan yang sama seperti ketika Anda [connect or disconnect an MCP server](#connecting-or-disconnecting-an-mcp-server):

* Jika Claude Code menunda tools server, itu menjaga cache.
* Jika Claude Code memuatnya ke dalam prefiks, permintaan berikutnya membaca ulang seluruh percakapan.

<h4 id="code-intelligence-plugins">
  Code intelligence plugins
</h4>

Ketika Anda mengaktifkan [code intelligence plugin](/docs/id/plugins/code-intelligence), Claude mendapatkan [LSP tool](/docs/id/tools-reference#lsp-tool-behavior).

<h4 id="when-plugin-changes-apply">
  When plugin changes apply
</h4>

Perubahan yang Anda buat di menu `/plugin` melalui [`/reload-plugins`](/docs/id/plugins/cli-reference#reload-plugins), yang Claude Code jalankan untuk Anda ketika Anda menutup menu. Anda membayar biaya, apakah pengumuman yang ditambahkan atau pembacaan ulang penuh, pada putaran pertama setelah perubahan diterapkan. Claude Code juga dapat menerapkan perubahan dengan sendirinya:

* Untuk plugin dengan sumber `command`, Claude Code [can reload the plugin itself](/docs/id/plugins/loading#when-a-command-source-re-runs).
* Ketika Anda [install a plugin from the `/plugin` interface](/docs/id/plugins/install#install-a-plugin), Claude Code dapat mengaktifkannya selama instalasi. Ringkasan instalasi memberi tahu Anda apakah itu dilakukan.
* Ketika Anda [move the session with `/cd`](/docs/id/permissions#move-the-session-to-another-directory) pada v2.1.246 atau lebih baru, Claude Code menerapkan plugin yang diaktifkan pengaturan direktori baru sebagai bagian dari perpindahan, tanpa peringatan pembacaan ulang penuh yang menahan `/reload-plugins`.
* Dalam sesi interaktif, ketika Anda menambah atau menghapus plugin dalam [folder of plugins](/docs/id/plugins/create#load-a-directory-or-archive-for-one-session) yang Anda lewatkan dengan `--plugin-dir`, perubahan diterapkan segera. Jika menerapkannya akan memicu pembacaan ulang penuh, Claude Code menahan perubahan sebagai gantinya dan menampilkan pemberitahuan untuk menjalankan `/reload-plugins`. Memerlukan Claude Code v2.1.265 atau lebih baru.

Ketika `/reload-plugins` berjalan dan reload akan memicu pembacaan ulang penuh, Claude Code menampilkan peringatan dan tidak menerapkan reload. Jalankan `/reload-plugins --force` untuk menerapkannya bagaimanapun.

`/reload-plugins` juga berjalan dalam sesi tanpa terminal interaktif, seperti aplikasi desktop, Agent SDK, dan [non-interactive mode](/docs/id/headless) dengan `-p`, ketika Anda mengetiknya langsung ke sesi. Memerlukan Claude Code v2.1.260 atau lebih baru.

Dalam sesi-sesi itu reload menerapkan semuanya kecuali perubahan MCP server plugin, yang [take effect in your next session](/docs/id/plugins/cli-reference#reload-plugins) dan jadi tidak pernah biaya pembacaan ulang penuh di tengah sesi.

<h4 id="plugins-you-enable-and-then-disable-in-one-session">
  Plugins you enable and then disable in one session
</h4>

Ketika Anda menonaktifkan plugin yang Anda aktifkan sebelumnya dalam sesi, Claude Code mengembalikan bentuk permintaan sebelumnya. Jika prefiks itu masih dalam [cache lifetime](#cache-lifetime)-nya, permintaan berikutnya membaca entri cache yang lebih lama sebagai gantinya dari membangun kembali.

<h3 id="denying-an-entire-tool">
  Denying an entire tool
</h3>

Jika Anda menambahkan nama tool telanjang seperti `Bash` atau `WebFetch` sebagai [deny rule](/docs/id/permissions#manage-permissions), Claude tidak dapat memanggil tool itu dari permintaan Anda berikutnya, apakah Anda menambahkan aturan melalui `/permissions` atau dengan [editing a settings file directly](/docs/id/settings#when-edits-take-effect). Itu termasuk aturan yang Anda tambahkan melalui `/permissions` di tengah putaran.

Ketika [tool search](/docs/id/mcp#scale-with-mcp-tool-search) aktif, yang merupakan default pada model yang didukung, definisi tool permintaan tidak berubah dan prefiks yang di-cache bertahan. Ketika tool search tidak tersedia atau dinonaktifkan, Claude Code menghapus definisi dari permintaan berikutnya, yang membatalkan cache, dan begitu juga menghapus aturan nanti.

Hanya aturan deny yang cocok di posisi nama-tool yang memblokir tool dengan cara ini: nama tool telanjang, bentuk `Bash(*)` yang setara, atau [tool-name glob](/docs/id/permissions#tool-name-wildcards) seperti `"*"`. Glob yang cocok hanya dengan tool MCP, seperti `"mcp__*"`, memblokir tool-tool itu dengan cara yang sama. Aturan deny yang dibatasi seperti `Bash(rm *)`, dan semua aturan allow dan ask, tidak mengubah tool mana yang Claude lihat. Claude Code memeriksanya ketika Claude mencoba panggilan, meninggalkan prefiks tetap utuh.

<h3 id="compacting-the-conversation">
  Compacting the conversation
</h3>

[Compaction](/docs/id/context-window#what-survives-compaction) menggantikan riwayat pesan Anda dengan ringkasan. Dengan desain, ini membatalkan lapisan percakapan, karena permintaan berikutnya memiliki riwayat yang lebih baru dan lebih pendek yang tidak berbagi prefiks dengan yang lama. Claude Code menggunakan kembali lapisan system prompt kecuali percakapan [resumed while keeping a system prompt that would otherwise have changed](#resuming-a-session); dalam hal itu compaction pertama beralih ke prompt saat ini dan lapisan itu membangun kembali sekali. Ini memuat ulang konteks proyek dari disk, yang cache-hits hanya jika CLAUDE.md dan memory tidak berubah sejak sesi dimulai.

Untuk menghasilkan ringkasan, Claude Code mengirim permintaan terpisah dengan system prompt, tools, dan riwayat yang sama seperti percakapan Anda, ditambah instruksi summarization yang ditambahkan sebagai pesan pengguna final. Saat cache hangat, permintaan itu membaca prefiks Anda dari cache, jadi `/compact` di tengah sesi biaya sebagian kecil dari apa yang ukuran konteks sarankan dan menghabiskan sebagian besar waktunya menghasilkan ringkasan.

Setelah istirahat lebih lama dari [cache lifetime](#cache-lifetime), tidak ada cache yang tersisa untuk dibaca, jadi permintaan summarization memproses ulang riwayat penuh sebagai input yang tidak di-cache. Ini adalah mengapa `/compact` biaya paling banyak ketika Anda [resume an old session](/docs/id/sessions#resume-from-a-summary). Dalam kedua kasus hangat dan dingin, putaran setelah compaction membangun kembali cache percakapan hanya untuk ringkasan yang jauh lebih pendek, jadi putaran itu bukan bagian yang lambat.

<Tip>
  Compaction bekerja untuk keuntungan Anda ketika konteks yang Anda buang adalah konten yang tidak lagi Anda butuhkan. Untuk memilih kapan overhead-nya terjadi, jalankan `/compact` pada istirahat alami dalam pekerjaan Anda, seperti antara tugas, daripada menunggu auto-compaction memicu di tengah tugas. Jika Anda telah pergi ke jalur yang ingin Anda abaikan sepenuhnya, [`/rewind`](#rewinding-the-conversation) ke putaran sebelumnya sebagai gantinya. Rewinding memotong kembali ke prefiks yang sudah di-cache, daripada membangun yang baru seperti yang dilakukan compaction.
</Tip>

<h3 id="accumulating-many-images">
  Accumulating many images
</h3>

API membatasi berapa banyak gambar dan PDF yang dapat dibawa setiap permintaan. Untuk angka saat ini, lihat [Request limits](https://platform.claude.com/docs/en/build-with-claude/vision#request-limits) dalam dokumentasi API. Claude Code juga membatasi ukuran total gambar dan PDF dalam permintaan, jadi screenshot besar mencapai batas dengan lebih sedikit gambar daripada yang kecil.

Ketika permintaan berikutnya akan melewati salah satu batas, Claude Code menghapus batch gambar dan PDF tertua dari apa yang dikirimnya, yang membuat ruang untuk lebih banyak sebelum perlu menghapus lagi. Claude tidak lagi dapat melihat gambar yang dihapus. Jika Claude membutuhkan salah satu dari mereka lagi, bagikan lagi.

Menghapus gambar mengubah pesan yang menahannya, jadi permintaan berikutnya memproses ulang percakapan dari yang paling awal dari pesan-pesan itu ke depan. Karena Claude Code menghapus batch sekaligus, Anda melihat satu putaran yang lebih lambat per batch daripada satu dengan setiap screenshot baru.

<h3 id="upgrading-claude-code">
  Upgrading Claude Code
</h3>

Versi Claude Code baru biasanya memperbarui system prompt atau definisi tool, jadi percakapan pertama yang Anda mulai setelah upgrade membangun cache-nya dari atas. [Auto-update](/docs/id/setup#auto-updates) mengunduh versi baru di latar belakang tetapi menerapkannya pada peluncuran berikutnya, tidak pernah di tengah sesi, jadi Anda melihat ini sebagai putaran pertama yang tidak di-cache setelah restart daripada kejutan selama sesi. Atur `DISABLE_AUTOUPDATER=1` untuk mengontrol kapan upgrade diterapkan.

<Note>
  Untuk apa yang biaya untuk melanjutkan percakapan yang Anda mulai sebelum upgrade, lihat [Resuming a session](#resuming-a-session).
</Note>

<h2 id="actions-that-keep-the-cache">
  Tindakan yang mempertahankan cache
</h2>

Tindakan-tindakan ini baik menambahkan ke akhir percakapan atau tidak menyentuh permintaan sama sekali. Beberapa di antaranya, seperti mengedit CLAUDE.md, mempertahankan cache karena alasan yang sama mengapa perubahan tidak mencapai sesi yang sedang berjalan sampai `/clear`, `/compact`, atau restart.

* [Mengedit file di repositori Anda](#editing-files-in-your-repository)
* [Mengedit CLAUDE.md di tengah sesi](#editing-claude-md-mid-session)
* [Mengubah mode izin](#changing-permission-mode)
* [Mengubah gaya output](#changing-output-style)
* [Memanggil skills dan commands](#invoking-skills-and-commands)
* [Menjalankan `/recap`](#running-%2Frecap)
* [Memutar ulang percakapan](#rewinding-the-conversation)
* [Menjalankan subagent](#subagents-and-the-cache)

<h3 id="editing-files-in-your-repository">
  Mengedit file di repositori Anda
</h3>

Konten file memasuki konteks hanya ketika Claude membacanya, dan pembacaan menambahkan ke percakapan. Mengedit file yang sebelumnya dibaca Claude tidak secara retroaktif mengubah pembacaan sebelumnya dalam riwayat. Sebaliknya, Claude Code menambahkan `<system-reminder>` yang mencatat file berubah, dan Claude membacanya kembali jika diperlukan.

<h3 id="editing-claude-md-mid-session">
  Mengedit CLAUDE.md di tengah sesi
</h3>

File CLAUDE.md tingkat project-root dan user-level Anda dibaca sekali pada awal sesi dan disimpan dalam memori. Mengeditnya di tengah sesi tidak membatalkan cache, tetapi edit juga tidak berlaku. Claude terus bekerja dengan versi yang dimuat pada awal sesi. Konten baru dimuat pada `/clear`, `/compact`, atau restart berikutnya.

[File CLAUDE.md bersarang di subdirektori](/docs/id/memory) dan [aturan dengan frontmatter `paths:`](/docs/id/memory#path-specific-rules) dimuat kemudian, ketika Claude pertama kali membaca file yang cocok. Mengedit satu sebelum dimuat memang berlaku. Setelah dimuat, konten adalah bagian dari riwayat percakapan, jadi edit di tengah sesi tidak secara retroaktif mengubahnya.

<h3 id="changing-permission-mode">
  Mengubah mode izin
</h3>

Beralih antara [mode izin](/docs/id/permission-modes), seperti dari Manual ke accept edits, tidak mengubah system prompt atau definisi tool, jadi perubahan mode aman untuk cache. Pengecualiannya adalah plan mode dengan pengaturan model [`opusplan`](/docs/id/model-config#opusplan-model-setting), yang mengalihkan model antara Opus dan Sonnet saat Anda memasuki atau meninggalkan plan mode. Itu membuat toggle mode menjadi [model switch](#switching-models).

<h3 id="changing-output-style">
  Mengubah gaya output
</h3>

Ketika Anda beralih [gaya output](/docs/id/output-styles) di tengah sesi dengan [`/output-style`](/docs/id/output-styles#change-your-output-style), `/config`, atau pengaturan `outputStyle`, Claude menggunakan gaya baru mulai dari pesan Anda berikutnya. Claude Code mengirimkan instruksi gaya baru sebagai pesan dalam percakapan, jadi permintaan itu masih membaca system prompt dan percakapan sebelumnya dari cache.

Sebelum v2.1.251, perubahan gaya di tengah sesi mempertahankan cache tetapi tidak berlaku sampai Anda menjalankan `/clear` atau memulai sesi baru.

<h3 id="invoking-skills-and-commands">
  Memanggil skills dan commands
</h3>

[Skills](/docs/id/skills) dan [commands](/docs/id/commands) menyuntikkan instruksi mereka sebagai pesan pengguna pada titik pemanggilan. Tidak ada yang lebih awal dalam percakapan berubah. Skill atau command yang frontmatter-nya menamai `model` dapat menjadi [model switch](#switching-models) untuk giliran itu.

<h3 id="running-/recap">
  Menjalankan `/recap`
</h3>

[`/recap`](/docs/id/interactive-mode#session-recap) menghasilkan ringkasan untuk ditampilkan di terminal Anda. Tidak seperti `/compact`, ini menambahkan ringkasan sebagai output command daripada mengganti riwayat pesan Anda, jadi prefix yang di-cache tetap utuh.

<h3 id="rewinding-the-conversation">
  Memutar ulang percakapan
</h3>

[`/rewind`](/docs/id/checkpointing) memotong percakapan Anda kembali ke giliran sebelumnya. Riwayat yang tersisa adalah konten yang sama yang dibangun cache darinya pada titik itu, dan system prompt serta lapisan konteks proyek tidak berubah, jadi permintaan berikutnya mencapai entri cache yang lebih awal. Setiap giliran sejak saat itu telah membaca melalui prefix itu, yang menjaga entri tetap hangat bahkan jika giliran asli lebih lama dari TTL.

Memulihkan checkpoint file bersama percakapan tidak memiliki efek terpisah pada cache. Konten file memasuki konteks hanya ketika Claude membacanya, sama seperti [mengedit file di repositori Anda](#editing-files-in-your-repository).

<h2 id="resuming-a-session">
  Melanjutkan sesi
</h2>

Ketika Anda [melanjutkan sesi](/docs/id/sessions#resume-a-session), Claude Code mengirimkan seluruh percakapan lagi, dan permintaan membaca dari cache bagian mana pun dari prefiks yang tidak berubah dan masih dalam [masa pakai cache](#cache-lifetime). Tabel lapisan di bagian atas halaman ini mengatakan apa yang berubah di setiap lapisan.

Prompt sistem akan berubah setelah [upgrade Claude Code](#upgrading-claude-code) atau dengan teks [`--append-system-prompt`](/docs/id/cli-reference#system-prompt-flags) yang berbeda pada resume. Secara default, percakapan yang dilanjutkan mempertahankan prompt sistem yang dimulainya, sehingga riwayatnya masih berada di belakang prompt yang sama, dan perubahan berlaku setelah percakapan dikompakkan atau dalam percakapan baru. [Bendera prompt sistem dalam percakapan yang dilanjutkan](/docs/id/cli-reference#system-prompt-flags-in-resumed-conversations) mencakup kasus di mana Claude Code membangun kembali prompt pada setiap permintaan sebagai gantinya.

<h2 id="cache-lifetime">
  Cache lifetime
</h2>

Prefix yang di-cache kedaluwarsa setelah periode tidak aktif. Setiap permintaan yang mencapai cache mengatur ulang timer, jadi cache tetap hangat selama Anda terus bekerja. Setelah jeda yang cukup lama, permintaan berikutnya menghitung ulang input penuh dan membangun kembali cache, itulah mengapa giliran pertama kembali setelah menjauh dapat terasa jauh lebih lambat.

Pada paket Pro atau Max, ketika Anda melanjutkan sesi besar setelah istirahat lama, Claude Code [menawarkan untuk melanjutkan dari ringkasan](/docs/id/sessions#resume-from-a-summary) sehingga permintaan nanti tidak membawa riwayat penuh.

Time to live (TTL) mengontrol berapa lama jeda yang cache bertahan. API menawarkan dua: TTL lima menit, dan [TTL satu jam](https://platform.claude.com/docs/en/build-with-claude/prompt-caching#1-hour-cache-duration) yang menjaga cache tetap hangat melalui istirahat yang lebih lama tetapi [menagih penulisan cache dengan tarif lebih tinggi](https://platform.claude.com/docs/en/build-with-claude/prompt-caching#pricing). TTL yang lebih lama membantu ketika Anda meninggalkan sesi idle dan kembali ke sana, karena Anda melewati pemrosesan ulang yang dikenakan prefix yang kedaluwarsa. Ini lebih mahal pada ledakan pekerjaan singkat yang tidak pernah idle melampaui lima menit, di mana tarif penulisan yang lebih tinggi berlaku dan masa pakai cache yang lebih lama tidak digunakan.

<h3 id="which-ttl-each-request-gets">
  TTL mana yang diterima setiap permintaan
</h3>

Claude Code memutuskan TTL per permintaan, dan setiap permintaan jatuh dalam salah satu dari dua bucket tetap:

* **Main conversation**: giliran interaktif Anda, run `-p` non-interaktif, dan giliran Agent SDK, ditambah pembantu yang Claude Code jalankan inline dengan mereka
* **Everything else**: permintaan yang Claude Code buat di luar percakapan itu, seperti [subagents](/docs/id/sub-agents), [workflows](/docs/id/workflows), [teammates](/docs/id/agent-teams) dalam proses, fork, compaction, dan judul sesi

Kecuali Anda memilih TTL sendiri, Claude Code meminta TTL satu jam hanya pada langganan Claude dalam penggunaan yang disertakan paket Anda. Di sana ia meminta jam untuk percakapan utama, ditambah serangkaian kecil permintaan pembantu yang Anthropic kontrol di sisi server. Tabel ini memberikan TTL default setiap bucket di bawah kedua jenis penagihan.

| Request bucket    | Claude subscription, within plan usage                                         | Usage credits, API key, or cloud provider |
| ----------------- | ------------------------------------------------------------------------------ | ----------------------------------------- |
| Main conversation | One hour                                                                       | Five minutes                              |
| Everything else   | Five minutes, except the server-controlled helper requests, which get one hour | Five minutes                              |

Setelah Anda melampaui batas penggunaan paket Anda dan Claude Code menggunakan [usage credits](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans), Anda ditagih untuk penggunaan itu, jadi Claude Code menurunkan percakapan utama ke TTL lima menit yang lebih murah. Untuk menjaga TTL satu jam di sana, [pilih TTL sendiri](#choose-the-ttl-yourself).

<h3 id="choose-the-ttl-yourself">
  Pilih TTL sendiri
</h3>

Anda dapat mengatur TTL untuk salah satu bucket. Setiap kontrol mengambil `5m` atau `1h`, dan Claude Code mengabaikan nilai lainnya.

* **Main conversation**: pengaturan [`promptCacheTtl`](/docs/id/settings-reference#promptcachettl), atau variabel lingkungan `CLAUDE_CODE_PROMPT_CACHE_TTL` [environment variable](/docs/id/env-vars)
* **Everything else**: pengaturan [`subagentPromptCacheTtl`](/docs/id/settings-reference#subagentpromptcachettl), atau variabel lingkungan `CLAUDE_CODE_SUBAGENT_PROMPT_CACHE_TTL`

Kedua pengaturan dan kedua variabel lingkungan memerlukan Claude Code v2.1.242 atau lebih baru. Jika Anda masuk dengan API key atau menggunakan penyedia cloud, atur `promptCacheTtl` ke `1h` untuk memberikan percakapan utama cache satu jam. Permintaan di luar itu menjaga default lima menit sampai Anda memilih TTL untuk bucket itu juga.

Ketika lebih dari satu kontrol berlaku, Claude Code mengambil kecocokan pertama dalam urutan ini:

1. `FORCE_PROMPT_CACHING_5M=1`, yang memaksa lima menit untuk kedua bucket
2. Variabel lingkungan bucket
3. Pengaturan bucket
4. Untuk permintaan subagent, nilai `cacheTtl` dalam field frontmatter [`experimental`](/docs/id/sub-agents#supported-frontmatter-fields) subagent, yang memerlukan Claude Code v2.1.248 atau lebih baru. Claude Code mengabaikan `1h` di sana sementara langganan Claude Anda menggunakan usage credits
5. `ENABLE_PROMPT_CACHING_1H=1`, yang meminta satu jam untuk kedua bucket
6. [Default untuk bucket permintaan](#which-ttl-each-request-gets)

Atur `FORCE_PROMPT_CACHING_5M=1` ketika Anda men-debug perilaku cache, membandingkan dua TTL, atau mengganti TTL yang lebih lama yang ditetapkan dalam [managed settings](/docs/id/managed-settings).

Untuk mengonfirmasi TTL mana yang digunakan penulisan cache percakapan utama Anda, jalankan `claude -p "hello" --output-format json` dan baca `usage.cache_creation` dalam hasilnya. Claude Code melaporkan penulisan cache satu jam di bawah `ephemeral_1h_input_tokens` dan penulisan cache lima menit di bawah `ephemeral_5m_input_tokens`.

Melalui gateway LLM yang Anda atur dengan `ANTHROPIC_BASE_URL`, bagian dari permintaan satu jam bepergian dalam header `anthropic-beta`, jadi konfigurasikan gateway untuk [meneruskan header itu tanpa perubahan](/docs/id/llm-gateway-protocol#request-headers). TTL satu jam tidak tersedia melalui [gateway aplikasi Claude](/docs/id/claude-apps-gateway#availability-and-limitations). Di Amazon Bedrock, dukungan prompt caching, panjang prefix yang dapat di-cache minimum, dan ketersediaan TTL satu jam semuanya bervariasi menurut model. Jika hitungan token cache tetap di nol, periksa [model yang didukung, wilayah, dan batas](https://docs.aws.amazon.com/bedrock/latest/userguide/prompt-caching.html#prompt-caching-models) dalam dokumentasi Amazon Bedrock.

<h2 id="cache-scope">
  Cakupan cache
</h2>

Di Claude Code, cache secara efektif dicakup ke satu mesin dan direktori. Setiap percakapan membawa direktori kerja, platform, shell, dan versi OS, dan prompt sistem menamai jalur auto-memory Anda, jadi dua sesi di direktori berbeda membangun prefix berbeda dan melewatkan cache satu sama lain. Itu termasuk worktrees dari repositori yang sama, karena setiap worktree memiliki direktori kerjanya sendiri.

Sesi yang Anda jalankan secara paralel di direktori yang sama membangun prefix yang cocok dan membaca cache satu sama lain. Sesi berurutan berbagi prefix hanya ketika snapshot status git yang diambil pada startup cocok, karena setiap percakapan juga membawa cabang dan commit terbaru dari snapshot itu.

Cache API yang mendasarinya lebih luas. Cache diisolasi di antara organisasi, dan pada beberapa penyedia, [di antara workspace dalam organisasi](https://platform.claude.com/docs/en/build-with-claude/prompt-caching#cache-storage-and-sharing). Dalam batas-batas itu, setiap dua permintaan dengan model dan prefix yang sama membaca cache yang sama. Untuk pemanggil Agent SDK yang menjalankan armada proses otomatis, lihat [improve prompt caching across users and machines](/docs/id/agent-sdk/modifying-system-prompts#improve-prompt-caching-across-users-and-machines) untuk menekan bagian per-mesin dari prompt sistem dan berbagi cache di seluruh mesin.

<h2 id="check-cache-performance">
  Periksa kinerja cache
</h2>

Kinerja cache muncul sebagai dua hitungan token yang dilaporkan API pada setiap respons. Cara paling langsung untuk menontonnya secara langsung adalah [statusline script](/docs/id/statusline) yang membaca objek `current_usage`:

| Field                         | Arti                                                                                                                                               |
| ----------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------- |
| `cache_creation_input_tokens` | Token yang ditulis ke cache pada giliran ini, ditagih dengan tarif penulisan cache                                                                 |
| `cache_read_input_tokens`     | Token yang disajikan dari cache pada giliran ini, ditagih dengan [tarif token cache](/docs/id/about-claude/pricing) model, di bawah tarif input standar |

Rasio baca-ke-kreasi yang tinggi berarti caching berfungsi dengan baik. Jika kreasi tetap tinggi giliran demi giliran, sesuatu berubah dalam prefix Anda. Bagian [actions that invalidate the cache](#actions-that-invalidate-the-cache) mencantumkan penyebab umum.

Untuk ringkasan per-sesi, jalankan `/usage`. Setelah respons pertama percakapan utama, Claude Code menambahkan baris [`Prompt cache (main)`](/docs/id/costs#prompt-cache-statistics) ke blok Sesi, menampilkan rasio hit sesi, jumlah miss, dan apakah cache sedang hangat sekarang. Skrip statusline dapat membaca angka yang sama dari [objek `prompt_cache`](/docs/id/statusline#prompt-cache-fields). Keduanya memerlukan Claude Code v2.1.251 atau lebih baru.

Baris `Prompt cache (main)` juga menamai kemungkinan penyebab miss terakhir ketika Claude Code dapat mengidentifikasinya, misalnya `likely cause: tool definitions changed`. Teks kemungkinan-penyebab memerlukan Claude Code v2.1.260 atau lebih baru.

Untuk visibilitas di seluruh organisasi, exporter OpenTelemetry melaporkan token baca dan kreasi cache per pengguna dan sesi. Lihat [Monitor usage](/docs/id/monitoring-usage) untuk referensi metrik dan atribut acara.

<h2 id="subagents-and-the-cache">
  Subagents dan cache
</h2>

[Subagent](/docs/id/sub-agents) memulai percakapannya sendiri dengan prompt sistem dan set alat-nya sendiri, terpisah dari induk. Permintaan pertamanya tidak membaca cache induk, karena kedua prefix berbeda, dan itu menghangatkan cache-nya sendiri di seluruh giliran-nya. Subagents berada di luar [bucket TTL](#which-ttl-each-request-gets) percakapan utama, jadi mereka mendapatkan lima menit bahkan pada langganan sampai Anda [memilih yang lebih lama](#choose-the-ttl-yourself).

Cache induk tidak terpengaruh. Dari sisi induk, panggilan dan hasil subagent ditambahkan ke percakapan, meninggalkan prefix induk utuh.

[Fork](/docs/id/sub-agents#fork-the-current-conversation), sebaliknya, mewarisi prompt sistem induk, alat, dan riwayat percakapan dengan tepat, jadi permintaan pertamanya membaca cache induk.

Permintaan lain juga dapat membaca prefix yang di-cache oleh permintaan sebelumnya:

* **Session copies**: sesi yang Anda [salin dengan `/fork`](/docs/id/agent-view#copy-the-session-with-%2Ffork) menerima instruksi isolasinya sebagai pesan di akhir percakapan yang disalin, jadi cache yang dibangun oleh percakapan asli tetap utuh.
* **Compaction**: panggilan summarisasi yang dijelaskan dalam [Compacting the conversation](#compacting-the-conversation) menggunakan pendekatan berbagi prefix yang sama.
* **Resumed subagents**: ketika Claude [melanjutkan subagent](/docs/id/sub-agents#resume-subagents), permintaan pertama dari run yang dilanjutkan dapat membaca cache yang di-hangatkan oleh run asli.
* **Workflow fan-outs**: dalam [workflow fan-out](/docs/id/workflows#prompt-caching-in-a-fan-out) dari agen dengan prefix yang sama, Claude Code menahan semua kecuali yang pertama selama hingga 5 detik secara default, jadi permintaan pertama mereka dapat membaca prefix yang di-cache oleh agen pertama.

<h2 id="disable-prompt-caching">
  Nonaktifkan prompt caching
</h2>

Menonaktifkan caching kadang-kadang berguna saat men-debug perilaku caching dengan model atau penyedia tertentu. Untuk mematikannya, atur salah satu variabel lingkungan ini ke `1`:

| Variable                        | Efek                          |
| ------------------------------- | ----------------------------- |
| `DISABLE_PROMPT_CACHING`        | Nonaktifkan untuk semua model |
| `DISABLE_PROMPT_CACHING_HAIKU`  | Nonaktifkan untuk Haiku saja  |
| `DISABLE_PROMPT_CACHING_SONNET` | Nonaktifkan untuk Sonnet saja |
| `DISABLE_PROMPT_CACHING_OPUS`   | Nonaktifkan untuk Opus saja   |
| `DISABLE_PROMPT_CACHING_FABLE`  | Nonaktifkan untuk Fable saja  |

Untuk menetapkan kebijakan caching di seluruh organisasi, masukkan salah satu dari ini atau [TTL variables](#cache-lifetime) dalam blok `env` dari [managed settings](/docs/id/managed-settings). Untuk penggunaan normal, biarkan caching diaktifkan.

<h2 id="related-resources">
  Sumber daya terkait
</h2>

* [Lessons from building Claude Code: Prompt caching is everything](https://claude.com/blog/lessons-from-building-claude-code-prompt-caching-is-everything): alasan desain untuk plan mode, deferred tool loading, dan compaction
* [Explore the context window](/docs/id/context-window): apa yang dimuat ke konteks dan kapan
* [Reduce token usage](/docs/id/costs#reduce-token-usage): strategi di luar caching untuk mengelola ukuran konteks
* [Track and reduce costs](/docs/id/agent-sdk/cost-tracking): pelacakan token cache dan konfigurasi TTL untuk pemanggil Agent SDK
* [Prompt caching](https://platform.claude.com/docs/id/build-with-claude/prompt-caching): mekanisme API yang mendasarinya, breakpoints, dan pricing
