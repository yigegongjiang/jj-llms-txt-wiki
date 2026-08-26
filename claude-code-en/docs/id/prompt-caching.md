> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Bagaimana Claude Code menggunakan prompt caching

> Claude Code mengelola prompt caching secara otomatis. Lihat mengapa perubahan model memicu giliran tanpa cache yang lambat, berapa biaya `/compact`, mengapa pengeditan CLAUDE.md tidak berlaku di tengah sesi, dan cara memeriksa tingkat cache hit Anda.

Prompt caching membuat Claude Code lebih cepat dan lebih hemat biaya. Tanpa caching, API akan memproses ulang riwayat lengkap Anda pada setiap giliran. Dengan caching, API menggunakan kembali apa yang sudah diproses dan hanya melakukan pekerjaan baru untuk apa yang berubah.

Claude Code menangani prompt caching untuk Anda, kecuali Anda [menonaktifkannya](#disable-prompt-caching). Masih berguna untuk mengetahui cara kerja prompt caching, karena beberapa tindakan membatalkan cache dan membuat respons berikutnya lebih lambat dan lebih mahal saat membangun kembali. Halaman ini mencakup tindakan mana yang melakukan itu, mengapa beberapa pengaturan menunggu restart untuk diterapkan, dan cara memeriksa kinerja cache ketika penggunaan terlihat tinggi.

<h2 id="how-the-cache-is-organized">
  Bagaimana cache diatur
</h2>

Setiap kali Anda mengirim pesan di Claude Code, aplikasi membuat permintaan API baru. Model tidak mengingat apa pun di antara permintaan, jadi Claude Code mengirim ulang konteks lengkap: prompt sistem, konteks proyek Anda, setiap pesan sebelumnya dan hasil alat, serta pesan baru Anda. Konten baru ditambahkan di akhir, yang berarti sebagian besar dari setiap permintaan identik dengan yang sebelumnya. Prompt caching adalah cara API menghindari pemrosesan ulang bagian yang tidak berubah.

API melakukan cache dengan mencocokkan awal setiap permintaan, yang disebut prefix, terhadap konten yang baru-baru ini diproses. Pada giliran normal, prefix adalah seluruh permintaan sebelumnya dan hanya pertukaran terbaru yang baru. Kecocokan bersifat tepat, jadi perubahan di mana pun dalam prefix menghitung ulang semuanya setelahnya. Tidak ada caching per-file atau per-segment. Lihat [cara kerja prompt caching](https://platform.claude.com/docs/id/build-with-claude/prompt-caching#how-prompt-caching-works) dalam referensi API untuk mekanisme yang mendasarinya.

<img src="https://mintcdn.com/claude-code/VbDJw--l6T9a9Wvm/images/prompt-caching-prefix.svg?fit=max&auto=format&n=VbDJw--l6T9a9Wvm&q=85&s=f2e8f0b8298a50305fe428ca3f1d1594" className="dark:hidden" alt="Empat giliran ditampilkan sebagai batang horizontal yang berkembang. Permintaan setiap giliran berisi semuanya dari giliran sebelumnya ditambah pertukaran terbaru ditambahkan di akhir. Pada giliran dua dan tiga, prefix yang tidak berubah dibaca dari cache dan hanya pertukaran baru yang diproses. Pada giliran empat, prompt sistem berubah, jadi prefix tidak lagi cocok dan seluruh permintaan diproses ulang dan ditulis." width="720" height="454" data-path="images/prompt-caching-prefix.svg" />

<img src="https://mintcdn.com/claude-code/_xqph1dUOslCOwsj/images/prompt-caching-prefix-dark.svg?fit=max&auto=format&n=_xqph1dUOslCOwsj&q=85&s=297dc1c639f0915cae858d0c4b6f3be5" className="hidden dark:block" alt="Empat giliran ditampilkan sebagai batang horizontal yang berkembang. Permintaan setiap giliran berisi semuanya dari giliran sebelumnya ditambah pertukaran terbaru ditambahkan di akhir. Pada giliran dua dan tiga, prefix yang tidak berubah dibaca dari cache dan hanya pertukaran baru yang diproses. Pada giliran empat, prompt sistem berubah, jadi prefix tidak lagi cocok dan seluruh permintaan diproses ulang dan ditulis." width="720" height="454" data-path="images/prompt-caching-prefix-dark.svg" />

Untuk mendapatkan hasil maksimal dari pencocokan prefix, Claude Code mengurutkan setiap permintaan sehingga konten yang jarang berubah di antara giliran datang terlebih dahulu:

| Layer          | Konten                                       | Berubah ketika                                                     |
| -------------- | -------------------------------------------- | ------------------------------------------------------------------ |
| Prompt sistem  | Instruksi inti, definisi alat, gaya output   | Set definisi alat yang dimuat berubah, atau Claude Code diperbarui |
| Konteks proyek | CLAUDE.md, auto memory, aturan tanpa cakupan | Sesi dimulai, atau setelah `/clear` atau `/compact`                |
| Percakapan     | Pesan Anda, respons Claude, hasil alat       | Setiap giliran                                                     |

Perubahan pada layer percakapan meninggalkan prompt sistem dan konteks proyek di-cache. Perubahan pada prompt sistem membatalkan semuanya, karena semua konten selanjutnya sekarang berada di belakang prefix yang berbeda. Kolom ketiga memberikan pemicu umum daripada daftar lengkap, dan bagian di bawah mencakup set lengkap, termasuk konten seperti gaya output yang ditetapkan pada awal sesi.

Aturan pencocokan prefix menjelaskan sebagian besar perilaku di halaman ini. [Plan mode](/docs/id/permission-modes#analyze-before-you-edit-with-plan-mode) dan [skill loading](/docs/id/skills), misalnya, menambahkan instruksi mereka sebagai pesan percakapan, jadi prefix yang di-cache tetap utuh.

Dua pengaturan tidak termasuk dalam teks prompt sama sekali, jadi mereka tidak muncul dalam tabel layer, tetapi keduanya adalah bagian dari kunci cache:

* **Model**: setiap model memiliki cache-nya sendiri. Beralih model menghitung ulang seluruh permintaan bahkan ketika kontennya identik. Lihat [Beralih model](#switching-models) di bawah.
* **Effort level**: setiap effort level memiliki cache-nya sendiri untuk model yang sama. Mengubahnya di tengah sesi menghitung ulang seluruh permintaan, dan Claude Code meminta Anda untuk mengonfirmasi sebelum menerapkan perubahan. Lihat [Mengubah effort level](#changing-effort-level) di bawah.

<Tip>
  Pilih model dan effort level Anda di awal sesi, kemudian simpan `/compact` untuk istirahat alami di antara tugas. Semakin sedikit perubahan yang Anda buat di tengah tugas, semakin tinggi tingkat cache hit Anda.
</Tip>

<h3 id="where-the-cache-lives">
  Tempat cache berada
</h3>

Caching terjadi di sisi server, dalam infrastruktur apa pun yang melayani model Anda. Tempat itu tergantung pada cara Anda melakukan autentikasi:

* **API key, langganan Claude, atau [Claude Platform on AWS](/docs/id/claude-platform-on-aws)**: cache berada di infrastruktur Anthropic, diakses melalui [Claude API](https://platform.claude.com/docs)
* **Amazon Bedrock atau Google Cloud's Agent Platform**: cache berada di infrastruktur penyajian penyedia cloud Anda
* **Microsoft Foundry**: permintaan merutekan ke infrastruktur Anthropic
* **Custom `ANTHROPIC_BASE_URL` atau [LLM gateway](/docs/id/llm-gateway)**: cache berada di mana pun permintaan Anda diteruskan, dan apakah caching berfungsi tergantung pada gateway

Untuk apa yang disimpan dan diproses setiap penyedia, lihat [data usage](/docs/id/data-usage). Di mana pun cache berada, entri kedaluwarsa setelah periode tidak aktif, dan [Cache lifetime](#cache-lifetime) di bawah mencakup TTL dan cara memperpanjangnya.

<h2 id="actions-that-invalidate-the-cache">
  Tindakan yang membatalkan cache
</h2>

Tindakan ini menyebabkan permintaan berikutnya kehilangan sebagian atau seluruh cache. Anda melihat satu giliran yang lebih lambat dan lebih mahal, setelah itu prefix baru di-cache. Sebagian besar dapat dihindari di tengah tugas setelah Anda mengetahui biayanya. Perubahan model dapat terasa gratis sampai Anda memperhatikan giliran yang lebih lambat setelahnya.

* [Beralih model](#switching-models)
* [Mengubah tingkat effort](#changing-effort-level)
* [Mengaktifkan fast mode](#turning-on-fast-mode)
* [Menghubungkan atau memutuskan server MCP](#connecting-or-disconnecting-an-mcp-server)
* [Mengaktifkan atau menonaktifkan plugin](#enabling-or-disabling-a-plugin)
* [Menolak seluruh tool](#denying-an-entire-tool)
* [Memadatkan percakapan](#compacting-the-conversation)
* [Meningkatkan Claude Code](#upgrading-claude-code)

<h3 id="switching-models">
  Beralih model
</h3>

Setiap model memiliki cache-nya sendiri. Beralih dengan [`/model`](/docs/id/model-config#setting-your-model) berarti permintaan berikutnya membaca seluruh riwayat percakapan tanpa cache hits, meskipun kontennya identik.

Pengaturan model [`opusplan`](/docs/id/model-config#opusplan-model-setting) diselesaikan ke Opus selama plan mode dan Sonnet selama eksekusi, jadi setiap toggle plan-mode adalah perubahan model dan memulai cache segar.

[Fallback model otomatis](/docs/id/model-config#automatic-model-fallback) pada Fable 5 juga merupakan perubahan model. Ketika pengklasifikasi keamanan menandai permintaan, Claude Code menjalankannya kembali pada model Opus default dan sesi berlanjut di sana.

<h3 id="changing-effort-level">
  Mengubah tingkat effort
</h3>

Cache dikunci oleh [tingkat effort](/docs/id/model-config#adjust-effort-level) serta model, jadi beralih dengan `/effort` berarti permintaan berikutnya membaca seluruh riwayat percakapan tanpa cache hits. Setelah percakapan dimulai, Claude Code menampilkan dialog konfirmasi sebelum menerapkan perubahan effort yang akan membatalkan cache. Perubahan yang diselesaikan ke tingkat yang sama yang sudah berlaku, seperti menetapkan default model secara eksplisit, melewati dialog dan menjaga cache.

<h3 id="turning-on-fast-mode">
  Mengaktifkan fast mode
</h3>

Mengaktifkan [fast mode](/docs/id/fast-mode) menambahkan header permintaan yang merupakan bagian dari cache key, jadi permintaan berikutnya membaca seluruh riwayat percakapan tanpa cache hits. Token input yang tidak di-cache tersebut ditagih dengan [fast mode rates](/docs/id/fast-mode#understand-the-cost-tradeoff), itulah mengapa mengaktifkannya di awal sesi lebih murah daripada mengaktifkannya jauh ke dalam sesi yang panjang. Mengaktifkan fast mode dari model non-Opus juga [beralih model Anda](#switching-models), yang memulai cache segar dengan sendirinya.

Biaya berlaku sekali per percakapan. Setelah giliran fast mode pertama, Claude Code terus mengirim header dan hanya memvariasikan pengaturan kecepatan permintaan, yang bukan bagian dari cache key. Mengaktifkan fast mode, [fallback otomatis ke kecepatan standar](/docs/id/fast-mode#handle-rate-limits) setelah rate limit, dan mengaktifkannya kembali nanti semua menjaga cache. `/clear` dan `/compact` mengatur ulang ini, karena mereka membangun kembali cache di titik-titik tersebut bagaimanapun.

<h3 id="connecting-or-disconnecting-an-mcp-server">
  Menghubungkan atau memutuskan server MCP
</h3>

Definisi alat berada di layer prompt sistem, jadi cache membatalkan ketika set definisi alat dalam permintaan berubah di antara giliran. Mengalihkan [advisor tool](/docs/id/advisor) adalah pengecualian: definisinya berada setelah breakpoint cache, jadi mengaktifkan atau menonaktifkan `/advisor` menjaga prefix yang di-cache tetap utuh. Apakah perubahan [server MCP](/docs/id/mcp) melakukan ini tergantung pada apakah alatnya ditunda oleh [tool search](/docs/id/mcp#scale-with-mcp-tool-search) atau dimuat ke dalam prefix:

* **Alat yang ditunda**, default pada model yang didukung: server yang terhubung, terputus, atau mengubah daftar alatnya hanya menambahkan konten baru dan tidak mengganggu apa pun yang sudah di-cache.
* **Alat yang dimuat ke dalam prefix**: perubahan apa pun pada mereka membatalkan cache. Ini terjadi ketika [tool search tidak tersedia atau dinonaktifkan](/docs/id/mcp#configure-tool-search), seperti pada Google Cloud's Agent Platform atau dengan gateway `ANTHROPIC_BASE_URL` kustom. Ini juga terjadi untuk server atau alat yang ditandai [`alwaysLoad`](/docs/id/mcp#exempt-a-server-from-deferral), dan untuk definisi yang disimpan di depan oleh [threshold-based loading](/docs/id/mcp#configure-tool-search).

Ketika alat dimuat ke dalam prefix, penyebab paling umum dari pembatalan adalah server yang terhubung atau terputus di tengah sesi, yang dapat terjadi tanpa tindakan apa pun dari pihak Anda: proses server stdio keluar, sesi HTTP kedaluwarsa, atau server [reconnects secara otomatis setelah kegagalan sementara](/docs/id/mcp#automatic-reconnection). Server yang terhubung juga dapat mendorong [dynamic tool update](/docs/id/mcp#dynamic-tool-updates) yang mengubah daftar alatnya.

Mengedit konfigurasi MCP Anda tidak dengan sendirinya mengubah cache. Konfigurasi baru berlaku hanya setelah restart, yaitu ketika server terhubung atau terputus.

<h3 id="enabling-or-disabling-a-plugin">
  Mengaktifkan atau menonaktifkan plugin
</h3>

[Plugin](/docs/id/plugins) menggabungkan beberapa jenis komponen, dan biaya perubahan tergantung pada komponen mana yang disediakan plugin. Skills, commands, agents, hooks, LSP servers, monitors, dan themes tidak pernah membatalkan cache: apa pun yang mereka tambahkan ke permintaan ditambahkan setelah percakapan yang ada, jadi permintaan berikutnya membayar untuk konten baru tetapi masih membaca semuanya sebelumnya dari cache.

Pengecualiannya adalah plugin yang menyediakan [server MCP](/docs/id/plugins-reference#mcp-servers). Mengaktifkan atau menonaktifkan satu mengikuti aturan yang sama seperti [menghubungkan atau memutuskan server MCP](#connecting-or-disconnecting-an-mcp-server): cache bertahan ketika alat server ditunda, dan permintaan berikutnya membaca ulang seluruh percakapan ketika mereka dimuat ke dalam prefix.

Perubahan plugin berlaku ketika Anda menjalankan [`/reload-plugins`](/docs/id/discover-plugins#apply-plugin-changes-without-restarting) atau memulai sesi baru. Biaya, baik pengumuman yang ditambahkan atau pembacaan ulang penuh, muncul pada giliran pertama setelah reload, bukan ketika Anda menjalankan `/plugin install`, `/plugin enable`, atau `/plugin disable`. Mulai dari v2.1.163, ketika reload akan memicu pembacaan ulang penuh, `/reload-plugins` menampilkan peringatan dan tidak menerapkan reload. Lewatkan `--force` untuk menerapkan bagaimanapun.

Menonaktifkan plugin yang Anda aktifkan sebelumnya dalam sesi mengembalikan bentuk permintaan sebelumnya. Jika prefix itu masih dalam [cache lifetime](#cache-lifetime)-nya, permintaan berikutnya membaca entri cache yang lebih lama daripada membangun kembali.

<h3 id="denying-an-entire-tool">
  Menolak seluruh tool
</h3>

Menambahkan nama tool yang sederhana seperti `Bash` atau `WebFetch` sebagai [deny rule](/docs/id/permissions#manage-permissions) menghapus tool tersebut dari konteks Claude sepenuhnya. Definisi tool bawaan dimuat ke layer prompt sistem, jadi menambah atau menghapus salah satu aturan ini di tengah sesi membatalkan cache. Perubahan berlaku pada giliran berikutnya baik Anda menambahkannya melalui `/permissions` atau dengan [mengedit file pengaturan secara langsung](/docs/id/settings#when-edits-take-effect).

Hanya deny rule yang cocok di posisi nama-tool yang memiliki efek ini: nama tool yang sederhana, bentuk setara `Bash(*)`, atau [tool-name glob](/docs/id/permissions#tool-name-wildcards) seperti `"*"`. Glob yang cocok hanya dengan MCP tools, seperti `"mcp__*"`, menghapus tools tersebut dengan cara yang sama tetapi menjaga cache tetap utuh ketika tools yang cocok [ditunda](#connecting-or-disconnecting-an-mcp-server), default, karena definisi yang ditunda tidak pernah ada di prefix yang di-cache. Deny rules yang dibatasi seperti `Bash(rm *)`, dan semua allow dan ask rules, tidak mengubah tools mana yang Claude lihat. Claude Code memeriksanya ketika Claude mencoba melakukan panggilan, meninggalkan prefix tetap utuh.

<h3 id="compacting-the-conversation">
  Memadatkan percakapan
</h3>

[Compaction](/docs/id/context-window#what-survives-compaction) menggantikan riwayat pesan Anda dengan ringkasan. Dengan desain, ini membatalkan layer percakapan, karena permintaan berikutnya memiliki riwayat baru yang lebih pendek yang tidak berbagi prefix dengan yang lama. Claude Code menggunakan kembali layer prompt sistem dan memuat ulang konteks proyek dari disk, yang cache-hits hanya jika CLAUDE.md dan memory tidak berubah sejak sesi dimulai.

Untuk menghasilkan ringkasan, Claude Code mengirim permintaan satu kali dengan prompt sistem, alat, dan riwayat yang sama dengan percakapan Anda, ditambah instruksi summarisasi ditambahkan sebagai pesan pengguna akhir. Karena berbagi prefix Anda, permintaan itu membaca cache yang ada daripada memproses ulang riwayat lengkap. Sebagian besar waktu compaction dihabiskan untuk menghasilkan ringkasan, bukan untuk cache miss. Giliran yang mengikuti membangun kembali cache percakapan hanya untuk ringkasan yang jauh lebih pendek, jadi giliran pasca-compaction bukan bagian yang lambat.

<Tip>
  Compaction bekerja untuk keuntungan Anda ketika konteks yang Anda buang adalah konten yang tidak lagi Anda butuhkan. Untuk memilih kapan overhead-nya terjadi, jalankan `/compact` pada istirahat alami dalam pekerjaan Anda, seperti di antara tugas, daripada menunggu auto-compaction memicu di tengah tugas. Jika Anda telah menempuh jalan yang ingin Anda tinggalkan sepenuhnya, [`/rewind`](#rewinding-the-conversation) ke giliran sebelumnya. Rewinding memotong kembali ke prefix yang sudah di-cache, daripada membangun yang baru seperti yang dilakukan compaction.
</Tip>

<h3 id="upgrading-claude-code">
  Meningkatkan Claude Code
</h3>

Versi Claude Code baru biasanya memperbarui prompt sistem atau definisi alat, jadi permintaan pertama setelah upgrade membangun kembali cache dari atas. [Auto-update](/docs/id/setup#auto-updates) mengunduh versi baru di latar belakang tetapi menerapkannya pada peluncuran berikutnya, tidak pernah di tengah sesi, jadi Anda melihat ini sebagai giliran pertama tanpa cache setelah restart daripada kejutan selama sesi. Atur `DISABLE_AUTOUPDATER=1` untuk mengontrol kapan upgrade diterapkan.

<Note>
  [Melanjutkan sesi](/docs/id/sessions#resume-a-session) setelah upgrade memproses ulang seluruh riwayat percakapan tanpa cache hits, karena riwayat sekarang berada di belakang prompt sistem yang berbeda. Biaya diskalakan dengan seberapa lama percakapan yang dilanjutkan, jadi giliran pertama kembali ke sesi panjang dapat menjadi permintaan paling mahal yang Anda kirim.
</Note>

<h2 id="actions-that-keep-the-cache">
  Tindakan yang menjaga cache
</h2>

Tindakan ini baik menambahkan ke akhir percakapan atau tidak menyentuh permintaan sama sekali. Beberapa di antaranya, seperti mengedit CLAUDE.md atau mengubah gaya output, juga mengapa perubahan pengaturan menunggu restart untuk diterapkan.

* [Mengedit file di repositori Anda](#editing-files-in-your-repository)
* [Mengedit CLAUDE.md di tengah sesi](#editing-claude-md-mid-session)
* [Mengubah gaya output](#changing-output-style)
* [Mengubah mode izin](#changing-permission-mode)
* [Memanggil skills dan commands](#invoking-skills-and-commands)
* [Menjalankan `/recap`](#running-%2Frecap)
* [Memutar ulang percakapan](#rewinding-the-conversation)
* [Menelurkan subagent](#subagents-and-the-cache)

<h3 id="editing-files-in-your-repository">
  Mengedit file di repositori Anda
</h3>

Konten file memasuki konteks hanya ketika Claude membacanya, dan pembacaan ditambahkan ke percakapan. Mengedit file yang sebelumnya dibaca Claude tidak secara retroaktif mengubah pembacaan sebelumnya dalam riwayat. Sebaliknya, Claude Code menambahkan `<system-reminder>` mencatat file berubah, dan Claude membacanya kembali jika diperlukan.

<h3 id="editing-claude-md-mid-session">
  Mengedit CLAUDE.md di tengah sesi
</h3>

File CLAUDE.md tingkat akar proyek dan tingkat pengguna dibaca sekali pada awal sesi dan disimpan dalam memori. Mengedit mereka di tengah sesi tidak membatalkan cache, tetapi edit juga tidak berlaku. Claude terus bekerja dengan versi yang dimuat pada awal sesi. Konten baru dimuat pada `/clear`, `/compact`, atau restart berikutnya.

[File CLAUDE.md bersarang di subdirektori](/docs/id/memory) dan [aturan dengan frontmatter `paths:`](/docs/id/memory#path-specific-rules) dimuat nanti, ketika Claude pertama kali membaca file yang cocok. Mengedit satu sebelum dimuat memang berlaku. Setelah dimuat, konten adalah bagian dari riwayat percakapan, jadi edit di tengah sesi tidak secara retroaktif mengubahnya.

<h3 id="changing-output-style">
  Mengubah gaya output
</h3>

[Output style](/docs/id/output-styles) adalah bagian dari prompt sistem, yang Claude Code baca sekali pada awal sesi. Mengubahnya melalui `/config` atau pengaturan `outputStyle` di tengah sesi tidak membatalkan cache, tetapi perubahan juga tidak berlaku. Claude terus menggunakan gaya yang dimuat pada awal sesi. Gaya baru dimuat pada `/clear` atau restart berikutnya.

<h3 id="changing-permission-mode">
  Mengubah mode izin
</h3>

Beralih di antara [permission modes](/docs/id/permission-modes), seperti dari default ke accept edits, tidak mengubah prompt sistem atau definisi alat, jadi perubahan mode aman cache. Pengecualiannya adalah plan mode dengan pengaturan model [`opusplan`](/docs/id/model-config#opusplan-model-setting), yang beralih model di antara Opus dan Sonnet saat Anda memasuki atau meninggalkan plan mode. Itu membuat toggle mode menjadi [model switch](#switching-models).

<h3 id="invoking-skills-and-commands">
  Memanggil skills dan commands
</h3>

[Skills](/docs/id/skills) dan [commands](/docs/id/commands) menyuntikkan instruksi mereka sebagai pesan pengguna pada titik invokasi. Tidak ada yang lebih awal dalam percakapan yang berubah.

<h3 id="running-/recap">
  Menjalankan `/recap`
</h3>

[`/recap`](/docs/id/interactive-mode#session-recap) menghasilkan ringkasan untuk ditampilkan di terminal Anda. Tidak seperti `/compact`, ini menambahkan ringkasan sebagai output perintah daripada menggantikan riwayat pesan Anda, jadi prefix yang di-cache tetap utuh.

<h3 id="rewinding-the-conversation">
  Memutar ulang percakapan
</h3>

[`/rewind`](/docs/id/checkpointing) memotong percakapan Anda kembali ke giliran sebelumnya. Riwayat yang tersisa adalah konten yang sama yang cache dibangun darinya pada saat itu, dan layer prompt sistem dan konteks proyek tidak berubah, jadi permintaan berikutnya mencapai entri cache sebelumnya. Setiap giliran sejak saat itu telah membaca melalui prefix itu, yang menjaga entri tetap hangat bahkan jika giliran asli lebih lama dari TTL.

Memulihkan checkpoint file bersama percakapan tidak memiliki efek terpisah pada cache. Konten file memasuki konteks hanya ketika Claude membacanya, sama seperti [mengedit file di repositori Anda](#editing-files-in-your-repository).

<h2 id="cache-lifetime">
  Cache lifetime
</h2>

Prefix yang di-cache kedaluwarsa setelah periode tidak aktif. Setiap permintaan yang mencapai cache mengatur ulang timer, jadi cache tetap hangat selama Anda terus bekerja. Setelah jeda yang cukup lama, permintaan berikutnya menghitung ulang input penuh dan membangun kembali cache, itulah mengapa giliran pertama kembali setelah menjauh dapat terasa jauh lebih lambat.

Time to live (TTL) mengontrol berapa lama jeda yang cache bertahan. API menawarkan dua: TTL lima menit, dan [TTL satu jam](https://platform.claude.com/docs/id/build-with-claude/prompt-caching#1-hour-cache-duration) yang menjaga cache tetap hangat melalui istirahat yang lebih lama tetapi [menagih penulisan cache dengan tarif lebih tinggi](https://platform.claude.com/docs/id/build-with-claude/prompt-caching#pricing). Claude Code memilih TTL untuk Anda berdasarkan cara Anda melakukan autentikasi, dan Anda dapat menggantinya dengan variabel lingkungan.

<h3 id="on-a-claude-subscription">
  Pada langganan Claude
</h3>

Pada langganan Claude, Claude Code meminta TTL satu jam secara otomatis. Penggunaan disertakan dalam paket Anda daripada ditagih per token, jadi TTL yang lebih lama tidak membebani Anda apa pun dan hanya mempengaruhi berapa lama cache Anda tetap hangat.

Jika Anda telah melampaui batas penggunaan paket Anda dan Claude Code menggunakan [usage credits](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans), Anda ditagih untuk penggunaan itu, jadi Claude Code secara otomatis menurunkan TTL menjadi lima menit.

<h3 id="on-an-api-key-or-third-party-provider">
  Pada API key atau penyedia pihak ketiga
</h3>

Pada API key, Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry, atau Claude Platform on AWS, Anda membayar tarif per-token, jadi TTL tetap pada lima menit yang lebih murah secara default. Untuk memilih [TTL satu jam](https://platform.claude.com/docs/id/build-with-claude/prompt-caching#1-hour-cache-duration), atur `ENABLE_PROMPT_CACHING_1H=1`.

Pada Amazon Bedrock, dukungan prompt caching, panjang prefix yang dapat di-cache minimum, dan ketersediaan TTL satu jam semuanya bervariasi menurut model. Jika hitungan token cache tetap di nol, periksa [model yang didukung, wilayah, dan batas](https://docs.aws.amazon.com/bedrock/latest/userguide/prompt-caching.html#prompt-caching-models) dalam dokumentasi Amazon Bedrock.

<h3 id="override-the-ttl">
  Ganti TTL
</h3>

Atur `FORCE_PROMPT_CACHING_5M=1` untuk memaksa TTL lima menit terlepas dari autentikasi. Ini berguna ketika Anda men-debug perilaku cache, membandingkan dua TTL, atau mengganti `ENABLE_PROMPT_CACHING_1H` yang ditetapkan dalam [managed settings](/docs/id/settings#settings-files).

<h2 id="cache-scope">
  Cakupan cache
</h2>

Di Claude Code, cache secara efektif dicakup ke satu mesin dan direktori. Prompt sistem menyematkan direktori kerja, platform, shell, versi OS, dan jalur auto-memory, jadi dua sesi di direktori berbeda membangun prefix berbeda dan melewatkan cache satu sama lain. Itu termasuk worktrees dari repositori yang sama, karena setiap worktree memiliki direktori kerjanya sendiri.

Sesi yang Anda jalankan secara paralel di direktori yang sama membangun prefix yang cocok dan membaca cache satu sama lain. Sesi berurutan berbagi prefix hanya ketika snapshot status git pada startup cocok, karena prompt sistem juga menangkap cabang dan commit terbaru.

Cache API yang mendasarinya lebih luas. Cache diisolasi di antara organisasi, dan pada beberapa penyedia, [di antara workspace dalam organisasi](https://platform.claude.com/docs/id/build-with-claude/prompt-caching#cache-storage-and-sharing). Dalam batas-batas itu, setiap dua permintaan dengan model dan prefix yang sama membaca cache yang sama. Untuk pemanggil Agent SDK yang menjalankan armada proses otomatis, lihat [improve prompt caching across users and machines](/docs/id/agent-sdk/modifying-system-prompts#improve-prompt-caching-across-users-and-machines) untuk menekan bagian per-mesin dari prompt sistem dan berbagi cache di seluruh mesin.

<h2 id="check-cache-performance">
  Periksa kinerja cache
</h2>

Kinerja cache muncul sebagai dua hitungan token yang dilaporkan API pada setiap respons. Cara paling langsung untuk menontonnya secara langsung adalah [statusline script](/docs/id/statusline) yang membaca objek `current_usage`:

| Field                         | Arti                                                                                                    |
| ----------------------------- | ------------------------------------------------------------------------------------------------------- |
| `cache_creation_input_tokens` | Token yang ditulis ke cache pada giliran ini, ditagih dengan tarif penulisan cache                      |
| `cache_read_input_tokens`     | Token yang disajikan dari cache pada giliran ini, ditagih dengan kira-kira 10% dari tarif input standar |

Rasio baca-ke-kreasi yang tinggi berarti caching berfungsi dengan baik. Jika kreasi tetap tinggi giliran demi giliran, sesuatu berubah dalam prefix Anda. Bagian [actions that invalidate the cache](#actions-that-invalidate-the-cache) mencantumkan penyebab umum.

Untuk visibilitas di seluruh organisasi, exporter OpenTelemetry melaporkan token baca dan kreasi cache per pengguna dan sesi. Lihat [Monitor usage](/docs/id/monitoring-usage) untuk referensi metrik dan atribut acara.

<h2 id="subagents-and-the-cache">
  Subagents dan cache
</h2>

[Subagent](/docs/id/sub-agents) memulai percakapannya sendiri dengan prompt sistem dan set alat-nya sendiri, terpisah dari induk. Ini membangun cache-nya sendiri, dimulai tanpa cache hits pada panggilan pertamanya dan menghangat di seluruh giliran-nya sendiri. Subagents menggunakan TTL lima menit bahkan pada langganan, karena TTL satu jam otomatis berlaku untuk percakapan utama.

Cache induk tidak terpengaruh. Dari sisi induk, panggilan dan hasil subagent ditambahkan ke percakapan, meninggalkan prefix induk utuh.

[Fork](/docs/id/sub-agents#fork-the-current-conversation), sebaliknya, mewarisi prompt sistem induk, alat, dan riwayat percakapan dengan tepat, jadi permintaan pertamanya membaca cache induk. Panggilan summarisasi compaction yang dijelaskan dalam [Compacting the conversation](#compacting-the-conversation) menggunakan pendekatan berbagi prefix yang sama.

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

Untuk menetapkan kebijakan caching di seluruh organisasi, masukkan salah satu dari ini atau [TTL variables](#cache-lifetime) dalam blok `env` dari [managed settings](/docs/id/settings#settings-files). Untuk penggunaan normal, biarkan caching diaktifkan.

<h2 id="related-resources">
  Sumber daya terkait
</h2>

* [Lessons from building Claude Code: Prompt caching is everything](https://claude.com/blog/lessons-from-building-claude-code-prompt-caching-is-everything): alasan desain untuk plan mode, deferred tool loading, dan compaction
* [Explore the context window](/docs/id/context-window): apa yang dimuat ke konteks dan kapan
* [Reduce token usage](/docs/id/costs#reduce-token-usage): strategi di luar caching untuk mengelola ukuran konteks
* [Track and reduce costs](/docs/id/agent-sdk/cost-tracking): pelacakan token cache dan konfigurasi TTL untuk pemanggil Agent SDK
* [Prompt caching](https://platform.claude.com/docs/id/build-with-claude/prompt-caching): mekanisme API yang mendasarinya, breakpoints, dan pricing
