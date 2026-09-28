> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Bagikan output sesi sebagai artifacts

> Artifacts mengubah pekerjaan Claude Code menjadi halaman interaktif langsung di claude.ai yang dapat Anda simpan pribadi, bagikan dengan organisasi Anda, atau publikasikan ke tautan publik.

<Note>
  Artifacts tersedia di paket Pro, Max, Team, dan Enterprise dan memerlukan sesi yang masuk dengan [`/login`](/docs/id/setup#authenticate). Lihat [Availability](#availability) untuk rangkaian lengkap persyaratan.
</Note>

Sebuah [artifact](https://claude.com/features/artifacts) adalah halaman web interaktif langsung yang Claude Code publikasikan dari sesi Anda ke URL pribadi di claude.ai. Anda membukanya di browser, dan halaman tersebut diperbarui di tempat saat sesi berlanjut. Bagikan dari header halaman ketika Anda ingin orang lain melihatnya juga.

<Frame>
  <img src="https://mintcdn.com/claude-code/kaHIYYMIYMYPxQg9/images/artifacts-viewer.png?fit=max&auto=format&n=kaHIYYMIYMYPxQg9&q=85&s=dbfd671cdb0d15f49f808b9e89778fe1" alt="Artifact terbuka di browser di claude.ai/code/artifact. Header viewer menampilkan judul artifact acme-funnel-fix, tombol Share, dan avatar penulis. Menu Share terbuka dengan toggle Always share latest version, pemilih versi yang menunjukkan Sharing version 2, pemilih audiens Everyone at Acme, dan tombol Copy link. Di bawah header, halaman artifact menampilkan dua mockup mobile berdampingan, bagan corong, dan baris kartu metrik." width="2511" height="1890" data-path="images/artifacts-viewer.png" />
</Frame>

<h2 id="when-to-use-an-artifact">
  Kapan menggunakan artifact
</h2>

Gunakan artifact ketika teks terminal bukan medium yang tepat untuk apa yang Claude hasilkan: output yang lebih mudah dilihat dan diinteraksi daripada dibaca baris per baris. Claude membangun halaman dari apa pun yang dapat dijangkau sesi Anda, termasuk basis kode Anda dan data yang ditariknya melalui [alat terhubung Anda](/docs/id/mcp), sehingga halaman dapat menampilkan hal-hal yang memerlukan paragraf untuk dijelaskan. Misalnya, minta Claude untuk:

* Memandu reviewer melalui pull request dengan diff yang diberi anotasi
* Merender dashboard dari data yang sudah ditarik sesi
* Menyusun beberapa opsi desain atau implementasi berdampingan
* Menyimpan timeline investigasi yang terisi saat tugas panjang berjalan
* Mengirim tautan ke rekan kerja alih-alih menempel output ke Slack
* Menerbitkan papan status yang [menarik data segar melalui konektor MCP](#pull-live-data-with-mcp-connectors) setiap kali seseorang membukanya

Lihat [Apa yang dapat Anda bangun](#what-you-can-build) untuk prompt yang cocok dengan ini, dan [Tarik data langsung dengan konektor MCP](#pull-live-data-with-mcp-connectors) untuk prompt papan yang didukung konektor.

<h3 id="what-an-artifact-is-not">
  Apa yang bukan artifact
</h3>

Artifact adalah tangkapan pekerjaan: satu halaman mandiri tanpa backend, sehingga tidak dapat melayani beberapa rute. Untuk alat internal yang dihosting dengan backend, sebarkan di infrastruktur Anda sendiri. Lihat [Batasan halaman](#page-constraints) untuk rangkaian lengkap batasan.

<h2 id="create-an-artifact">
  Buat artefak
</h2>

Claude dapat menerbitkan artefak dengan sendirinya ketika output sesuai untuk halaman, atau Anda dapat memintanya secara langsung. Untuk meminta, sebutkan fitur atau jelaskan output visual yang Anda inginkan dalam bahasa biasa. Kandidat yang baik adalah apa pun yang lebih mudah dilihat daripada dibaca sebagai teks, seperti diff yang diberi anotasi, bagan, atau serangkaian opsi untuk dibandingkan. Prompt di bawah ini adalah dua contoh; lihat [Apa yang dapat Anda buat](#what-you-can-build) untuk pola lainnya.

```text wrap theme={null}
Make an artifact that walks through this PR with the diff annotated inline.
```

```text wrap theme={null}
Build a dashboard artifact of last week's deploy failures by service and keep it updated as you investigate.
```

Kecuali Anda menentukan lokasi, Claude menulis halaman ke file HTML atau Markdown di direktori sementara di luar proyek Anda, kemudian menerbitkannya. Menerbitkan artefak baru melalui [mode izin](/docs/id/permission-modes) sesi Anda:

* **Mode Auto**: pengklasifikasi meninjau penerbitan alih-alih meminta Anda, sehingga Claude dapat menerbitkan halaman tanpa Anda melihat prompt. Mode mana yang dimulai sesi Anda tergantung pada paket Anda; lihat [mode izin awal](/docs/id/permission-modes#eliminate-prompts-with-auto-mode).
* **Mode Manual dan Accept edits**: Claude Code meminta izin; mungkin mengatakan sesuatu seperti `Claude wants to publish deploy-failures.html, uploading it to claude.ai (Anthropic's servers) to host as the page "Deploy failures by service", private to you until you share it`. Pilih **Yes** untuk menerbitkan.

Setelah Anda menyetujui artefak sekali, Claude Code menerbitkannya kembali tanpa bertanya, dan bertanya lagi dalam beberapa kasus, termasuk ketika:

* Claude mendeklarasikan kemampuan runtime untuk halaman, seperti [connector calls](#pull-live-data-with-mcp-connectors) atau [file downloads](#offer-a-file-download)
* Anda telah [membagikannya secara publik](#share-an-artifact)
* Anda telah membagikannya dengan orang-orang tertentu atau organisasi Anda dengan versi terbaru dipilih sebagai versi yang dilihat pemirsa

Setelah penerbitan pertama, Claude mencetak URL, dan browser Anda membuka halaman baru. Jika Anda mengirim prompt melalui [Remote Control](/docs/id/remote-control) dari claude.ai, Claude Desktop, atau aplikasi Claude mobile, tidak ada tab yang terbuka di mesin yang menjalankan sesi. Browser terbuka di sana lain kali Claude menerbitkan artefak dari prompt yang Anda ketik di terminal. Tekan `Ctrl+]` kapan saja untuk membuka kembali artefak terbaru dari sesi Anda.

Claude memilih judul artefak dan emoji, dan keduanya muncul di [galeri artefak](#share-an-artifact) Anda di claude.ai dan di tautan bersama. Claude juga dapat memilih ikon tab browser yang sesuai dengan apa yang ada di halaman, seperti bagan atau kalender. Minta Claude untuk judul, emoji, atau ikon tab tertentu jika Anda menginginkannya.

Untuk menghentikan browser agar tidak membuka secara otomatis ketika artefak baru diterbitkan, atur `CLAUDE_CODE_ARTIFACT_AUTO_OPEN=0` di lingkungan Anda.

Jika Claude merespons bahwa ia tidak dapat menerbitkan, atau menulis file HTML lokal tanpa tautan, alat tidak diaktifkan untuk sesi Anda. Periksa persyaratan [Ketersediaan](#availability).

<h2 id="update-an-artifact">
  Perbarui artefak
</h2>

Minta Claude untuk merevisi halaman, atau biarkan tugas yang berjalan lama menerbitkan ulang saat membuat kemajuan. Claude mengedit file yang mendasarinya dan menerbitkan ulang ke URL yang sama.

```text wrap theme={null}
Tambahkan rincian per-wilayah di bawah bagan ringkasan dan terbitkan ulang.
```

Siapa pun yang membuka halaman akan melihat pembaruan di tempat. Setiap penerbitan menjadi versi, dan dari kontrol **Share** di header halaman Anda dapat memilih versi mana yang dilihat penonton.

Untuk memperbarui artefak dari sesi yang berbeda, berikan Claude URL-nya, atau lampirkan dengan [`/artifacts`](#find-an-artifact-again). Tanpa salah satu dari keduanya, sesi baru membuat artefak baru alih-alih memperbarui yang sudah ada.

```text wrap theme={null}
Perbarui https://claude.ai/code/artifact/5fbea6f3-... dengan angka hari ini.
```

<h2 id="find-an-artifact-again">
  Temukan artefak lagi
</h2>

Jalankan `/artifacts` di Claude Code untuk membuat daftar setiap artefak yang Anda miliki dan setiap artefak yang dibagikan kepada Anda. Pilih satu dan tekan `o` untuk membukanya di browser Anda atau `c` untuk menyalin tautannya. Tekan `Enter` untuk melampirkannya ke sesi saat ini; sebelum v2.1.216, `Enter` membukanya di browser Anda. Claude Code membaca daftar dari akun claude.ai Anda, sehingga berfungsi di sesi baru dan setelah `/clear`, ketika tautan telah keluar dari terminal. Memerlukan Claude Code v2.1.208 atau lebih baru.

<h2 id="share-an-artifact">
  Bagikan artefak
</h2>

Artefak baru hanya terlihat oleh Anda. Untuk membagikannya, buka artefak di browser Anda dan gunakan kontrol **Share** di header halaman. Header juga menautkan ke galeri Anda di [claude.ai/code/artifacts](https://claude.ai/code/artifacts), yang mencantumkan setiap artefak yang telah Anda buat.

Penampil di organisasi Anda dapat melihat siapa yang menerbitkan halaman: pada artefak yang dibagikan dalam organisasi Anda, nama Anda ada di menu judul, dan pada artefak publik itu ada di header halaman untuk penampil yang masuk di organisasi Anda. Penampil yang membuka tautan publik tanpa masuk, atau dari luar organisasi Anda, melihat label `Content is user-generated and unverified.` sebagai gantinya dari nama Anda.

Siapa yang dapat Anda bagikan tergantung pada paket Anda:

* **Dalam organisasi Anda**: pada paket Team dan Enterprise, berikan akses ke orang-orang tertentu di organisasi Anda, atau ke semua orang di dalamnya. Penampil masuk ke claude.ai sebagai anggota organisasi Anda untuk melihat halaman.
* **Secara publik**: bagikan tautan yang dapat dibuka siapa saja di internet, tanpa memerlukan masuk claude.ai. Pada paket Pro dan Max, tautan publik adalah satu-satunya cara untuk membagikan artefak. Pada paket Team dan Enterprise, berbagi publik dimatikan sampai Pemilik [mengaktifkannya untuk organisasi](#control-public-sharing).

<h3 id="let-someone-edit-with-you">
  Biarkan seseorang mengedit bersama Anda
</h3>

Orang-orang yang Anda bagikan adalah penampil secara default: mereka melihat setiap versi yang Anda terbitkan tetapi tidak dapat mengubah halaman. Pada paket Team dan Enterprise, Anda juga dapat menjadikan seseorang sebagai editor. Di dialog berbagi, tambahkan orang dan ubah peran mereka dari **viewer** menjadi **editor**.

Editor menerbitkan versi baru dengan cara yang sama seperti Anda [memperbarui artefak dari sesi lain](#update-an-artifact): mereka memberikan Claude URL artefak, atau melampirkannya dari [`/artifacts`](#find-an-artifact-again), dan Claude menarik konten saat ini dan menerbitkan ulang dengan perubahan mereka. Semua orang dengan halaman terbuka melihat setiap pembaruan secara langsung.

<h2 id="read-an-artifact-shared-with-you">
  Baca artefak yang dibagikan dengan Anda
</h2>

Ketika seseorang membagikan artefak dengan Anda, Anda dapat meminta Claude membacanya: berikan Claude URL-nya, atau lampirkan dari [`/artifacts`](#find-an-artifact-again).

Claude membaca halaman yang ditulis orang lain dengan cara yang sama seperti membaca halaman web dengan [WebFetch](/docs/id/tools-reference#webfetch-tool-behavior): Claude mendapatkan ringkasan tentang apa yang dimintanya daripada halaman mentah, dan ringkasan tersebut melaporkan instruksi yang ditulis ke dalam halaman daripada menyampaikannya. Claude Code juga menyimpan sumber lengkap halaman ke file lokal, yang dapat dibuka Claude ketika memerlukan konten yang tepat, seperti untuk menerbitkan kembali artefak sebagai [editor](#let-someone-edit-with-you).

<h2 id="collect-comments-on-an-artifact">
  Kumpulkan komentar pada artefak
</h2>

Ketika Anda membagikan artefak dalam organisasi Anda, orang-orang yang Anda bagikan dapat meninggalkan komentar di halaman, dan Anda dapat meminta Claude membaca komentar tersebut dan membalasnya. Anda memerlukan Claude Code v2.1.221 atau lebih baru dan paket Team atau Enterprise, karena hanya artefak yang Anda [bagikan dalam organisasi](#share-an-artifact) yang menerima komentar. Claude membaca komentar dalam dua kasus:

* **Anda meminta Claude membacanya**: berikan Claude URL artefak dan minta komentarnya. Claude mencantumkan setiap thread dan menandai komentar dari seseorang yang dapat mengedit artefak yang dikirimkan kepadanya.
* **Seseorang yang dapat mengedit artefak mengirim komentar ke Claude**: dalam thread di halaman, mereka mengirim komentar dengan **Send to Claude**, atau menyebutkan `@claude` di dalamnya. Kedua cara tersebut mengaktifkan thread.

Claude hanya dapat membalas atau menyelesaikan thread yang diaktifkan. Thread lainnya tetap terbuka sampai seseorang menyelesaikannya di halaman. Penonton melihat setiap balasan yang dikaitkan dengan Claude, melalui Anda.

Jika Anda membagikan artefak secara publik, penonton tidak dapat mengomentarinya: halaman mengatakan `Comments aren't available while this Artifact is shared publicly.` Untuk mengubah artefak yang sudah memiliki thread komentar menjadi tautan publik, hapus thread terlebih dahulu.

Untuk meminta komentar sendiri, berikan Claude URL-nya:

```text wrap theme={null}
Read the comments on https://claude.ai/code/artifact/5fbea6f3-... and make the changes the commenters ask for.
```

Jika Claude memberi tahu Anda bahwa ia tidak dapat membaca komentar, periksa versi Anda, sesi Anda, dan pengaturan feature-flag Anda:

* Anda menjalankan Claude Code v2.1.221 atau lebih baru.
* Anda tidak dalam sesi pertama Anda sejak Anda menginstal Claude Code atau meningkatkan dari versi sebelum v2.1.221. Dalam [sesi pertama setelah instalasi atau peningkatan](/docs/id/env-vars#first-session-after-an-install-or-upgrade), Claude mungkin belum dapat membaca komentar; mulai sesi baru dan tanyakan lagi.
* Anda belum mematikan pengambilan feature-flag.

<h3 id="let-claude-reply-to-comments-on-its-own">
  Biarkan Claude membalas komentar dengan sendirinya
</h3>

Setelah sesi Anda menerbitkan artefak, Claude Code memantau artefak tersebut untuk komentar selama sesi berjalan. Ketika seseorang yang dapat mengedit artefak mengirim komentar ke Claude, itu mencapai sesi Anda segera, dan Claude dapat membaca thread dan membalas tanpa Anda meminta.

Anda memerlukan Claude Code v2.1.228 atau lebih baru. Jika Anda mematikan [pengambilan feature-flag](/docs/id/env-vars#features-that-need-feature-flag-fetching), Claude Code tidak memantau komentar.

[Mode izin](/docs/id/permission-modes) Anda menentukan apa yang dilakukan Claude ketika komentar yang dikirim tiba:

* **Claude membalas dengan sendirinya**: ketika mode izin Anda memungkinkan Claude memposting balasan tanpa meminta Anda, Claude membaca thread dan membalas, dan mengedit artefak ketika komentar meminta perubahan. Anda melihat `Auto-replied to comment thread on Artifact: <name>` atau `Auto-edited Artifact: <name> in response to a comment thread`.
* **Claude menunggu Anda**: di luar plan mode, ketika memposting balasan memerlukan persetujuan Anda, Anda melihat `Comments are waiting on Artifact: <name>`. Claude kemudian meminta persetujuan Anda untuk membaca thread, dan lagi untuk memposting balasan.
* **Claude berhenti di plan mode**: Anda melihat `Comments are waiting on Artifact: <name>`, dan Claude tidak membalas sampai Anda meninggalkan plan mode dan memintanya membaca dan membalas.

Claude juga berhenti membalas dengan sendirinya ke artefak setelah menangani 60 komentar yang dikirim atau aktivasi thread pada artefak tersebut dalam satu jam. Anda melihat `Comments are waiting on Artifact: <name>` sekali, dan Claude melanjutkan lagi saat komentar jam itu sudah lama.

Jalankan `/tasks` untuk melihat setiap artefak yang sesi Anda pantau, tercantum sebagai tugas pembaruan langsung. Anda dapat menghentikan Claude dari membalas dengan sendirinya dengan salah satu cara berikut:

* **Tekan Ctrl+C sekali pada prompt idle**: Claude berhenti membalas pada setiap artefak yang sesi Anda pantau. Balasan dimulai lagi setelah Anda mengirim pesan berikutnya.
* **Hentikan tugas di `/tasks`**: Claude berhenti membalas pada artefak tersebut sampai Anda memintanya melanjutkan balasan di sana. Menerbitkan artefak lagi tidak memulai balasan lagi, dan penghentian masih berlaku ketika Anda melanjutkan sesi nanti.
* **Tekan `Ctrl+X Ctrl+K` dua kali dalam 3 detik**: chord yang [menghentikan setiap subagent latar belakang yang berjalan](/docs/id/interactive-mode#general-controls) juga menghentikan Claude dari membalas pada setiap artefak untuk sisa sesi. Meminta Claude melanjutkan balasan tidak membatalkan penghentian ini.

Jika layanan yang mengirimkan komentar menjadi tidak tersedia atau berhenti menjawab, Claude Code terus mencoba untuk terhubung kembali untuk sementara, kemudian berhenti memantau setiap artefak yang sesi Anda pantau.

<h2 id="pull-live-data-with-mcp-connectors">
  Tarik data langsung dengan konektor MCP
</h2>

Sebuah artifact dapat memanggil [konektor MCP](/docs/id/mcp#use-mcp-servers-from-claude-ai) setiap kali seseorang melihatnya, sehingga halaman menampilkan data terkini daripada snapshot dari sesi yang membangunnya. Panggilan konektor dari artifact tersedia di paket Pro, Max, Team, dan Enterprise dan memerlukan Claude Code v2.1.209 atau lebih baru. Pada versi sebelumnya, Claude menerbitkan halaman dengan data apa pun yang dikumpulkan sesi saat membangunnya.

Untuk membuat halaman yang didukung konektor, namai konektor dan data yang Anda inginkan dalam prompt Anda:

```text wrap theme={null}
Build a dashboard artifact of our open pull requests that pulls the live list through my GitHub connector when the page loads.
```

Claude mendeklarasikan konektor mana yang dapat dipanggil halaman sebagai bagian dari penerbitan, dan halaman tidak dapat memanggil konektor di luar deklarasi tersebut. Hanya konektor dari akun claude.ai Anda yang memenuhi syarat: Claude menamainya dalam deklarasi, dan ketika seseorang melihat halaman, setiap panggilan [berjalan melalui koneksi akun penampil sendiri](#how-connector-calls-work-for-viewers) ke konektor tersebut. Server MCP lokal yang Anda konfigurasi di Claude Code, seperti server dari `.mcp.json`, dapat menyediakan data saat Claude membangun halaman, tetapi halaman yang diterbitkan tidak dapat memanggilnya.

Halaman mengambil data saat dimuat dan dapat menyegarkan pada interval atau ketika penampil menggunakan kontrol penyegaran di halaman. Respons disimpan dalam cache di browser penampil, sehingga halaman yang dibuka kembali dirender dari respons yang disimpan dalam cache segera, kemudian diperbarui dengan hasil segar.

<h3 id="how-connector-calls-work-for-viewers">
  Cara kerja panggilan konektor untuk penampil
</h3>

Ketika halaman yang diterbitkan memanggil konektor, panggilan menggunakan akun orang yang melihat halaman, bukan akun orang yang menerbitkannya:

* **Setiap penampil menggunakan konektor mereka sendiri**: panggilan berjalan melalui alat yang terhubung akun penampil, sehingga dua orang membuka dasbor yang sama dapat melihat data berbeda tergantung pada apa yang dapat diakses akun mereka. Halaman tidak pernah melihat kredensial siapa pun; claude.ai membuat panggilan atas nama halaman.
* **Penampil menyetujui akses terlebih dahulu**: claude.ai meminta izin setiap penampil sebelum panggilan konektor pertama halaman. Penampil yang menolak, atau yang belum menghubungkan konektor yang digunakan halaman, masih melihat halaman tanpa bagian langsung-nya.
* **Tindakan juga menggunakan akun penampil**: halaman dapat menawarkan kontrol yang memanggil alat konektor dengan efek samping, seperti memposting pesan atau memperbarui masalah. Tindakan berjalan melalui akun siapa pun yang memilih kontrol.

Ketika Anda berencana untuk berbagi halaman yang didukung konektor, minta Claude untuk menyertakan pesan fallback di setiap bagian langsung yang menamakan konektor yang dibutuhkannya. Penampil yang kehilangan koneksi kemudian melihat apa yang harus dihubungkan daripada bagian kosong.

Artifact yang memanggil konektor tidak dapat dibagikan ke tautan publik di paket apa pun. Di paket Team dan Enterprise, Anda dapat menyimpannya tetap pribadi atau [membagikannya dalam organisasi Anda](#share-an-artifact). Di paket Pro dan Max, di mana tautan publik adalah satu-satunya cara untuk berbagi, artifact yang didukung konektor tetap pribadi untuk Anda.

<h3 id="the-page-shows-no-live-data-for-a-viewer">
  Halaman tidak menampilkan data langsung untuk penampil
</h3>

Ketika halaman yang didukung konektor dirender tetapi bagian langsung-nya tetap kosong untuk seseorang yang Anda bagikan, kerjakan penyebab-penyebab ini:

* **Penampil belum menghubungkan konektor**: konektor adalah per-akun, jadi setiap penampil memerlukan koneksi mereka sendiri ke setiap konektor yang dipanggil halaman. Mereka dapat menambahkan satu di bawah **Settings > Connectors** di claude.ai, kemudian muat ulang halaman.
* **Penampil menolak permintaan izin**: penolakan berlaku untuk sisa pemuatan halaman itu. Memuat ulang halaman membawa permintaan izin kembali.
* **Panggilan konektor dimatikan untuk organisasi**: Pemilik mengontrol [toggle **Enable artifact connectors**](#control-connector-calls-from-artifacts) dalam pengaturan admin.
* **Halaman memanggil nama alat yang tidak diekspos konektor**: bagian yang terpengaruh tetap kosong untuk semua orang, termasuk Anda. Ini dapat terjadi ketika halaman menamakan alat individual di balik konektor gaya gateway yang hanya mengekspos beberapa alat miliknya sendiri. Minta Claude untuk memperbaiki nama alat yang dipanggil halaman dan menerbitkannya lagi.

  Ketika Claude menerbitkan halaman dan alat konektor tersebut tersedia dalam sesi Anda, Claude Code memeriksa nama alat yang dideklarasikan halaman terhadapnya, memperingatkan Claude tentang nama yang tidak cocok, dan menolak penerbitan ketika tidak ada yang cocok. Sebelum v2.1.265, itu menerbitkan halaman tanpa memeriksanya.

<h2 id="offer-a-file-download">
  Tawarkan unduhan file
</h2>

Sebuah artifact dapat menawarkan kepada penampil sebuah file yang dihasilkan halaman, seperti ekspor CSV dari tabel atau PNG dari bagan. Penampil menyimpannya melalui kontrol unduhan di halaman, seperti tombol. Unduhan file adalah kemampuan runtime yang claude.ai aktifkan per akun, jadi Claude memeriksa apakah akun Anda memilikinya sebelum membangun kontrol.

Penampil tidak dapat menyimpan file dari tautan unduhan biasa atau skrip di halaman, karena penampil artifact di claude.ai memblokir unduhan apa pun yang dimulai halaman itu sendiri, termasuk tautan ke URL `data:` atau `blob:`. Jika halaman memiliki tombol unduhan yang dibangun dengan cara itu, minta Claude untuk membangunnya kembali dengan kemampuan unduhan.

Untuk menawarkan file, minta kontrol dan format file dalam prompt Anda:

```text wrap theme={null}
Add a button that downloads this table as a CSV file.
```

Claude mendeklarasikan kemampuan unduhan sebagai bagian dari penerbitan, dengan cara yang sama seperti [mendeklarasikan konektor](#pull-live-data-with-mcp-connectors).

<h2 id="what-you-can-build">
  Apa yang dapat Anda bangun
</h2>

Artifact adalah satu halaman HTML, jadi apa pun yang dapat Anda ekspresikan dalam HTML, CSS, dan JavaScript inline berada dalam cakupan. Pola di bawah ini paling sering muncul.

<h3 id="walk-through-a-change">
  Berjalan melalui perubahan
</h3>

Minta halaman yang merender diff atau perubahan desain dengan anotasi di samping baris yang relevan, sehingga reviewer dapat membaca alasan Anda di samping kode alih-alih merekonstruksinya dari deskripsi.

```text wrap theme={null}
Make an artifact that walks through this PR. Render the diff with margin annotations and color-code findings by severity.
```

<h3 id="compare-alternatives">
  Bandingkan alternatif
</h3>

Minta beberapa varian di satu halaman sehingga Anda dapat mengevaluasinya satu sama lain. Ini berfungsi untuk tata letak, salinan, bentuk API, atau rencana implementasi.

```text wrap theme={null}
Make an artifact with four distinctly different layouts for the settings panel. Vary density and grouping, and lay them out as a grid with a one-line tradeoff under each.
```

<h3 id="tune-with-interactive-controls">
  Sesuaikan dengan kontrol interaktif
</h3>

Minta slider, toggle, atau bidang input yang terikat pada apa pun yang Anda sesuaikan, sehingga Anda dapat menjelajahi nilai secara langsung alih-alih menjelaskannya.

```text wrap theme={null}
Build an artifact with sliders for the easing curve, duration, and delay so I can try values on this transition. Show the animation live as I move them.
```

<h3 id="bring-the-result-back-to-your-session">
  Bawa hasil kembali ke sesi Anda
</h3>

Artifact dapat bertindak sebagai editor ringan untuk keputusan yang kemudian Anda serahkan kembali ke Claude. Minta kontrol ekspor yang menghasilkan teks yang dapat Anda tempel ke terminal, sehingga hasil berinteraksi dengan halaman mengalir kembali ke sesi alih-alih tetap di halaman.

```text wrap theme={null}
Make a triage board artifact with each open issue as a draggable card across Now, Next, Later, and Cut columns. Add a "Copy as prompt" button that gives me the final ordering to paste back here.
```

<h3 id="track-work-in-progress">
  Lacak pekerjaan yang sedang berlangsung
</h3>

Minta Claude untuk menjaga artifact tetap terkini saat tugas panjang berjalan, sehingga siapa pun dengan tautan dapat mengikuti tanpa membaca terminal.

```text wrap theme={null}
Turn this migration plan into a checklist artifact. Check items off as you complete them and add a note for anything you skip.
```

<h2 id="improve-the-visual-design">
  Tingkatkan desain visual
</h2>

Claude menerapkan keterampilan desain bawaan ketika membangun artefak, sehingga halaman mendapatkan palet, tipografi, dan tata letak yang disengaja tanpa permintaan tambahan. Keterampilan tersebut juga mencari sistem desain yang ada di proyek Anda sebelum memilih miliknya sendiri. Design tokens adalah nilai warna, tipografi, dan spasi bernama yang sistem desain Anda gunakan kembali. Untuk menjaga artefak tetap konsisten dengan branding produk Anda, catat di mana Claude dapat menemukannya, seperti [CLAUDE.md](/docs/id/memory) proyek atau file tema di repositori Anda:

```markdown theme={null}
## Design system

- Colors: primary #1a4d8f, accent #f59e0b, surface #f8fafc
- Typography: Inter for body, JetBrains Mono for code
- Spacing: 8px scale, 6px border radius
```

Claude memperlakukan sistem desain Anda sebagai prioritas lebih tinggi daripada pilihannya sendiri, dan prompt Anda sebagai prioritas lebih tinggi daripada keduanya. Judul dan format di atas adalah contoh; daftar warna, font, dan spasi yang jelas apa pun berfungsi.

Untuk tipografi, Claude dapat memuat typeface dari Google Fonts, satu-satunya sumber font eksternal yang dapat dimuat halaman artefak. Claude menginline typeface lain apa pun sebagai `@font-face` data URI dan memberikan setiap typeface tumpukan fallback, sehingga halaman masih ditampilkan jika font tidak dimuat. Untuk menggunakan typeface tertentu, sebutkan dalam prompt atau sistem desain Anda.

<h2 id="draft-a-design-canvas">
  Buat kanvas desain
</h2>

Untuk membuat mockup UI, alur layar, halaman landing, atau poster daripada membangun halaman, jalankan `/design` dengan ringkasan singkat. Claude membuat desain sebagai artboard pada satu kanvas dan menerbitkan kanvas sebagai artefak Design. Ringkasan singkat menamai apa yang ingin Anda gambar:

```text wrap theme={null}
/design a settings screen for a mobile banking app
```

Buka artefak yang diterbitkan di browser desktop untuk meninjau artboard. Pilih elemen pada artboard dan ubah, dan edit Anda disimpan secara otomatis. Anda dapat mengekspor setiap artboard sebagai PNG atau PDF.

`/design` memerlukan sesi di mana [artefak tersedia](#availability) dan Claude Code v2.1.265 atau lebih baru.

<h2 id="page-constraints">
  Batasan halaman
</h2>

Setiap artefak adalah satu halaman yang berdiri sendiri. Claude Code membungkus file yang Anda terbitkan dalam shell dokumen HTML dan melayaninya di bawah Content Security Policy (CSP) yang ketat, yang membentuk apa yang dapat dilakukan halaman.

| Batasan              | Efek                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| :------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Permintaan eksternal | Halaman dapat memuat typeface dari Google Fonts, dan skrip dari [lima host CDN publik](#allowlist-the-viewer-domain): cdnjs, unpkg, Tailwind dan jQuery CDNs, dan jalur yang dipilih di jsDelivr seperti `/npm/`. CSP memblokir setiap gambar eksternal dan semua skrip, stylesheet, dan font eksternal lainnya, dan membiarkan panggilan `fetch`, XHR, dan WebSocket hanya menjangkau asal halaman sendiri dan host Google Fonts. Claude oleh karena itu memuat perpustakaan apa pun yang dibutuhkan halaman dari salah satu CDN tersebut, menginline semua CSS dan JavaScript lainnya, dan menyematkan gambar sebagai data URI. [Panggilan Connector](#pull-live-data-with-mcp-connectors) melewati claude.ai, yang membuat panggilan jaringan itu sendiri. |
| Tidak ada backend    | Artefak adalah halaman statis. Tidak dapat mengautentikasi penampil itu sendiri.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| Unduhan              | Halaman tidak dapat memulai unduhan itu sendiri. Untuk membiarkan penampil menyimpan file yang dihasilkan halaman, Claude mendeklarasikan kemampuan unduhan. Lihat [Tawarkan unduhan file](#offer-a-file-download).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| Halaman tunggal      | Tautan relatif tidak diselesaikan, karena tidak ada yang digunakan bersama halaman. Untuk konten multi-bagian, Claude menggunakan jangkar dalam halaman daripada file terpisah.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| Jenis file sumber    | File yang diterbitkan harus `.html`, `.htm`, atau `.md`, dan harus didekode sebagai UTF-8, atau sebagai UTF-16 little-endian berdasarkan byte-order mark-nya. File Markdown dirender sebagai halaman dokumen bergaya dengan kode yang disintaks-sorot. File yang tidak didekode, atau yang berisi karakter pengganti `U+FFFD`, adalah [ditolak dengan baris dan kolom untuk diperbaiki](/docs/id/errors#the-source-file-is-not-valid-utf-8-text).                                                                                                                                                                                                                                                                                                                  |
| Ukuran yang dirender | Halaman yang dirender harus 16 MiB atau lebih kecil. Gambar tertanam besar adalah penyebab umum ketika penerbitan gagal karena ukuran.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |

Menghasilkan artefak menggunakan token output seperti respons lainnya, dan halaman bergaya lebih intensif token daripada konten yang sama sebagai teks terminal. CSS inline, JavaScript untuk kontrol interaktif, dan terutama gambar yang disematkan sebagai data URI adalah kontributor utama. Untuk mengurangi biaya token artefak:

* Lebih suka SVG, atau HTML dan CSS, untuk diagram daripada gambar raster tertanam
* Hilangkan interaktivitas yang tidak Anda butuhkan
* Buat halaman merangkum dataset besar daripada menginlinenya sepenuhnya

<h2 id="availability">
  Ketersediaan
</h2>

Artifacts memerlukan setiap kondisi di bawah. Ketika salah satu tidak terpenuhi, Claude menulis file HTML lokal atau mengatakan tidak dapat menerbitkan.

| Persyaratan          | Tersedia ketika                                                                                                                                                                                                                                                                                                                                                                                                                            |
| :------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Paket                | Pro, Max, Team, atau Enterprise. Pada paket Pro dan Max, artifacts bersifat pribadi untuk Anda sampai Anda membagikannya, dan tidak ada manajemen admin yang berlaku. Pada paket Team, artifacts diaktifkan secara default. Pada paket Enterprise, pemilik [mengaktifkannya](#manage-artifacts-for-your-organization) di pengaturan admin claude.ai.                                                                                       |
| Autentikasi          | Sesi didukung oleh akun claude.ai: masuk dengan `/login` di CLI atau aplikasi desktop. Sesi Claude Tag masuk melalui identitas agen, jadi tidak ada langkah yang diperlukan di sana. Sesi menggunakan kunci API, [gateway token](/docs/id/llm-gateway), atau kredensial penyedia cloud tidak dapat menerbitkan.                                                                                                                                 |
| Penyedia model       | Anthropic API. Tidak tersedia di [Amazon Bedrock](/docs/id/amazon-bedrock), [Google Cloud's Agent Platform](/docs/id/google-vertex-ai), atau [Microsoft Foundry](/docs/id/microsoft-foundry).                                                                                                                                                                                                                                                             |
| Kebijakan organisasi | Kunci enkripsi yang dikelola pelanggan (CMEK), HIPAA, dan [Zero Data Retention](/docs/id/zero-data-retention) tidak diaktifkan untuk organisasi.                                                                                                                                                                                                                                                                                                |
| Permukaan            | Claude Code CLI, atau aplikasi desktop Claude versi 1.13576.0 atau lebih baru. Sesi [Claude Tag](https://claude.com/docs/claude-tag/overview) juga dapat menerbitkan artifacts ketika Claude Tag dan artifacts keduanya diaktifkan untuk organisasi. Dimatikan secara default di konteks [Agent SDK](/docs/id/agent-sdk/overview), GitHub Action, dan MCP-server, dan ketika [`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`](/docs/id/env-vars) diatur. |

Apakah artifacts diizinkan untuk organisasi Anda berasal dari kebijakan organisasi Anda, yang Claude Code muat dari `api.anthropic.com`. Ketika Claude Code tidak dapat memuat kebijakan, artifacts tidak tersedia. Ketika Anda memintanya, Claude mengatakan alasannya.

Jika proxy, VPN, atau filter web terlibat, minta admin IT Anda untuk membiarkan `api.anthropic.com` melalui. Claude Code terus mencoba ulang di latar belakang, dan artifacts menjadi tersedia setelah kebijakan dimuat dan mengizinkannya.

<h2 id="disable-artifacts">
  Nonaktifkan artifacts
</h2>

Untuk mematikan artifacts untuk sesi Anda sendiri terlepas dari pengaturan organisasi Anda, gunakan salah satu dari:

| Tempat                              | Yang harus dilakukan                                                                                                                     |
| :---------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------- |
| [`/config`](/docs/id/commands)           | Matikan baris **Artifacts**, yang menulis [`"enableArtifact": false`](/docs/id/settings-reference#enableartifact) ke pengaturan pengguna Anda |
| [File pengaturan](/docs/id/settings)     | Atur `"enableArtifact": false`. `"disableArtifact": true` yang sudah usang juga mematikan artifacts                                      |
| [Variabel lingkungan](/docs/id/env-vars) | Atur `CLAUDE_CODE_DISABLE_ARTIFACT=1`                                                                                                    |
| [Aturan izin](/docs/id/permissions)      | Tambahkan `Artifact` ke `permissions.deny`                                                                                               |

Setelah Anda mematikan artifacts dalam file [`--settings`](/docs/id/cli-reference#cli-flags) atau dengan `CLAUDE_CODE_DISABLE_ARTIFACT`, atau administrator Anda mematikannya dalam [pengaturan terkelola](/docs/id/server-managed-settings), tidak ada file pengaturan yang menyalakannya kembali. Sebelum v2.1.242, file yang lebih tinggi dalam [tumpukan prioritas](/docs/id/settings#settings-precedence) dapat menyalakan artifacts kembali bahkan ketika file dengan prioritas lebih rendah menetapkan `"enableArtifact": false`.

Anda juga dapat menetapkan `"enableArtifact": false` dalam `.claude/settings.json` atau `.claude/settings.local.json` proyek untuk mematikan artifacts untuk sesi dalam proyek tersebut. `"enableArtifact": true` di salah satu file tidak menyalakannya kembali. Menghormati kunci dalam pengaturan proyek dan lokal memerlukan Claude Code v2.1.242 atau lebih baru.

Jika Anda menambahkan aturan deny atau ask `WebFetch` tanpa bagian `domain:`, itu tidak mematikan artifacts atau memblokir pembacaan artifact. Aturan [`WebFetch(domain:claude.ai)` dalam `deny` atau `ask` berlaku untuk pembacaan artifact](/docs/id/permissions#allow-or-deny-every-fetch).

<h2 id="manage-artifacts-for-your-organization">
  Kelola artifacts untuk organisasi Anda
</h2>

Pemilik pada paket Team dan Enterprise mengontrol artifacts dari [pengaturan admin claude.ai](https://claude.ai/admin-settings/claude-code). Konten artifact disimpan di infrastruktur yang dioperasikan Anthropic dan hanya terlihat oleh anggota terautentikasi dari organisasi penerbit, kecuali artifact [dibagikan secara publik](#control-public-sharing).

<h3 id="enable-or-disable-artifacts">
  Aktifkan atau nonaktifkan artifacts
</h3>

Untuk mengaktifkan atau menonaktifkan artifacts untuk seluruh organisasi, buka [**Settings > Claude Code > Capabilities**](https://claude.ai/admin-settings/claude-code) dan gunakan toggle **Artifacts**. Pada paket Enterprise dengan kontrol akses berbasis peran, Anda dapat membatasi artifacts ke peran tertentu: buka [**Settings > Roles**](https://claude.ai/admin-settings/roles), edit peran, dan atur izin **Artifacts** di bawah grup **Claude Code**.

<h3 id="control-connector-calls-from-artifacts">
  Kontrol panggilan connector dari artifacts
</h3>

[Panggilan connector dari artifacts](#pull-live-data-with-mcp-connectors) memiliki toggle mereka sendiri, terpisah dari toggle **Artifacts** yang mengaktifkan atau menonaktifkan artifacts. Buka [**Settings > Capabilities**](https://claude.ai/admin-settings/capabilities) dan gunakan toggle **Enable artifact connectors**. Toggle yang sama mengatur panggilan connector dari artifacts yang dibuat dalam percakapan claude.ai, itulah mengapa toggle ini berada di bawah **Settings > Capabilities** daripada **Settings > Claude Code**.

<h3 id="control-public-sharing">
  Kontrol berbagi publik
</h3>

Berbagi publik dimatikan secara default pada paket Team dan Enterprise, jadi anggota dapat berbagi artifacts hanya dalam organisasi sampai Pemilik mengaktifkannya. Untuk memungkinkan anggota menerbitkan artifacts ke tautan publik yang dapat dilihat siapa saja tanpa masuk, buka **Settings > Claude Code > Capabilities** dan aktifkan **External sharing** di bawah toggle **Artifacts**. Menonaktifkannya kembali memblokir akses melalui tautan publik yang ada tanpa mengubah audiens setiap artifact; akses dilanjutkan jika Anda mengaktifkannya kembali.

<h3 id="set-a-retention-policy">
  Atur kebijakan retensi
</h3>

Untuk mengatur berapa lama artifacts disimpan sebelum penghapusan otomatis, buka [**Settings > Data & privacy controls**](https://claude.ai/admin-settings/data-privacy-controls). Anda dapat mengatur periode retensi terpisah untuk artifacts yang masih pribadi untuk penulis mereka dan artifacts yang telah dibagikan.

<h3 id="review-the-audit-log">
  Tinjau log audit
</h3>

Penerbitan, berbagi, dan menghapus artifact masing-masing muncul di log audit organisasi Anda di bawah jenis acara `claude_artifact_*`, keluarga yang sama digunakan untuk artifacts yang dibuat dalam percakapan claude.ai.

<h3 id="allowlist-the-viewer-domain">
  Daftar putih domain viewer
</h3>

Viewer di claude.ai memuat setiap artifact dari asal `*.claudeusercontent.com` yang disandboxkan. Jika organisasi Anda membatasi akses jaringan keluar, tambahkan domain itu ke daftar putih Anda bersama `claude.ai`. Lihat [Network access requirements](/docs/id/network-config#network-access-requirements) untuk daftar lengkap.

Artifact yang memuat typeface dari [Google Fonts](#improve-the-visual-design) juga meminta `fonts.googleapis.com` dan `fonts.gstatic.com`. Kedua host bersifat opsional. Jika Anda memblokir mereka, artifacts akan dirender dalam typeface fallback. Blokir dengan penolakan cepat daripada drop senyap sehingga permintaan font gagal segera daripada menunda render pertama halaman.

Artifacts juga dapat memuat pustaka JavaScript, seperti React atau paket charting, dari `cdnjs.cloudflare.com`, `cdn.jsdelivr.net`, `cdn.tailwindcss.com`, `code.jquery.com`, dan `unpkg.com`, dan dari tidak ada host eksternal lainnya. Jika Anda memblokir host tersebut, bagian dari artifact yang bergantung pada pustaka tidak berfungsi, dan tidak seperti font yang diblokir, pustaka yang diblokir tidak memiliki fallback. Blokir dengan penolakan cepat di sini juga, sehingga permintaan pustaka yang diblokir gagal sekaligus daripada menggantung sampai waktu habis.

<h3 id="list-and-delete-artifacts-with-the-compliance-api">
  Daftar dan hapus artifacts dengan Compliance API
</h3>

[Compliance API](https://docs.claude.com/en/api/compliance) menyediakan endpoint untuk mencantumkan artifacts organisasi Anda, mengambil konten versi tertentu, dan menghapus artifact:

| Metode   | Endpoint                                                            |
| :------- | :------------------------------------------------------------------ |
| `GET`    | `/v1/compliance/code/artifacts`                                     |
| `GET`    | `/v1/compliance/code/artifacts/{artifact_id}/versions/{version_id}` |
| `DELETE` | `/v1/compliance/code/artifacts/{artifact_id}`                       |

Untuk skema permintaan dan respons, lihat [referensi Compliance API](https://docs.claude.com/en/api/compliance/code/artifacts).

<h2 id="related-resources">
  Sumber daya terkait
</h2>

* Jelajahi [pola prompting dan alur kerja](/docs/id/prompt-library) yang berpasangan dengan artifacts
* Ubah prompt artifact yang Anda gunakan kembali menjadi [skill](/docs/id/skills) sehingga Anda dapat memanggilnya sebagai perintah
* [Hubungkan server MCP](/docs/id/mcp) sehingga Claude dapat menarik data langsung ke artifact saat membangun halaman
