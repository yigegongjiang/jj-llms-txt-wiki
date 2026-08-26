> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Bagikan keluaran sesi sebagai artifacts

> Artifacts mengubah pekerjaan Claude Code menjadi halaman interaktif langsung di claude.ai yang dapat Anda simpan secara pribadi, bagikan dengan organisasi Anda, atau terbitkan ke tautan publik.

<Note>
  Artifacts tersedia di paket Pro, Max, Team, dan Enterprise dan memerlukan sesi yang masuk dengan [`/login`](/docs/id/setup#authenticate). Lihat [Availability](#availability) untuk rangkaian lengkap persyaratan.
</Note>

Artifact adalah halaman web interaktif langsung yang Claude Code terbitkan dari sesi Anda ke URL pribadi di claude.ai. Anda membukanya di browser, dan halaman tersebut diperbarui di tempat saat sesi berlanjut. Bagikan dari header halaman ketika Anda ingin orang lain melihatnya juga. Misalnya, gunakan artifact untuk membimbing reviewer melalui pull request dengan diff yang diberi anotasi, membangun dashboard dari data sesi, atau menyimpan timeline investigasi yang terisi saat Claude bekerja.

<Frame>
  <img src="https://mintcdn.com/claude-code/kaHIYYMIYMYPxQg9/images/artifacts-viewer.png?fit=max&auto=format&n=kaHIYYMIYMYPxQg9&q=85&s=dbfd671cdb0d15f49f808b9e89778fe1" alt="Artifact terbuka di browser di claude.ai/code/artifact. Header viewer menampilkan judul artifact acme-funnel-fix, tombol Share, dan avatar penulis. Menu Share terbuka dengan toggle Always share latest version, pemilih versi yang menunjukkan Sharing version 2, pemilih audiens Everyone at Acme, dan tombol Copy link. Di bawah header, halaman artifact menampilkan dua mockup mobile berdampingan, bagan corong, dan baris kartu metrik." width="2511" height="1890" data-path="images/artifacts-viewer.png" />
</Frame>

<h2 id="when-to-use-an-artifact">
  Kapan menggunakan artifact
</h2>

Gunakan artifact ketika teks terminal adalah medium yang salah untuk apa yang Claude hasilkan: output yang lebih mudah dilihat dan berinteraksi daripada dibaca baris demi baris. Claude membangun halaman dari apa pun yang dapat dijangkau sesi Anda, termasuk codebase Anda dan data yang ditariknya melalui [tools yang terhubung](/docs/id/mcp), sehingga halaman dapat menampilkan hal-hal yang memerlukan paragraf untuk dijelaskan. Misalnya, minta Claude untuk:

* Membimbing reviewer melalui pull request dengan diff yang diberi anotasi
* Merender dashboard dari data yang sudah ditarik sesi
* Meletakkan beberapa opsi desain atau implementasi berdampingan
* Menyimpan timeline investigasi yang terisi saat tugas panjang berjalan
* Mengirim tautan ke rekan kerja alih-alih menempel output ke Slack
* Menerbitkan papan status yang [menarik data segar melalui konektor MCP](#pull-live-data-with-mcp-connectors) setiap kali seseorang membukanya

Lihat [What you can build](#what-you-can-build) untuk prompt yang cocok dengan masing-masing, dan [Pull live data with MCP connectors](#pull-live-data-with-mcp-connectors) untuk prompt papan yang didukung konektor.

<h3 id="what-an-artifact-is-not">
  Apa yang bukan artifact
</h3>

Artifact adalah tangkapan pekerjaan, bukan aplikasi. Ini adalah satu halaman yang mandiri tanpa backend, jadi tidak dapat menyimpan input formulir atau melayani beberapa rute, dan satu-satunya jalurnya ke data eksternal ketika seseorang melihatnya adalah [memanggil konektor MCP](#pull-live-data-with-mcp-connectors). Untuk alat internal yang dihosting dengan backend, sebarkan di infrastruktur Anda sendiri. Lihat [Page constraints](#page-constraints) untuk rangkaian lengkap batasan.

<h2 id="create-an-artifact">
  Buat artifact
</h2>

Claude dapat menerbitkan artifact dengan sendirinya ketika output cocok untuk halaman, atau Anda dapat memintanya secara langsung. Untuk meminta, beri nama fitur atau jelaskan output visual yang Anda inginkan dalam bahasa biasa. Kandidat yang baik adalah apa pun yang lebih mudah dilihat daripada dibaca sebagai teks, seperti diff yang diberi anotasi, bagan, atau serangkaian opsi untuk dibandingkan. Prompt di bawah ini adalah dua contoh; lihat [What you can build](#what-you-can-build) untuk pola lainnya.

```text wrap theme={null}
Make an artifact that walks through this PR with the diff annotated inline.
```

```text wrap theme={null}
Build a dashboard artifact of last week's deploy failures by service and keep it updated as you investigate.
```

Claude menulis halaman ke file HTML atau Markdown di proyek Anda, kemudian menerbitkannya. Sebelum menerbitkan artifact baru, Claude Code meminta izin; mungkin mengatakan sesuatu seperti `Claude wants to publish "Deploy failures by service" (deploy-failures.html) to a private page on claude.ai`. Menerbitkan ulang artifact yang sudah Anda setujui tidak akan meminta lagi.

Pilih **Yes** untuk menerbitkan. Claude mencetak URL, dan browser Anda membuka ke halaman baru. Tekan `Ctrl+]` kapan saja untuk membuka kembali artifact terbaru dari terminal.

Claude memilih judul artifact dan emoji untuk ikon tab browsernya. Keduanya muncul di [galeri artifacts](#share-an-artifact) Anda di claude.ai dan di tautan bersama, jadi minta Claude untuk menggunakan judul atau ikon tertentu jika Anda menginginkannya.

Untuk menghentikan browser membuka secara otomatis ketika artifact baru diterbitkan, atur `CLAUDE_CODE_ARTIFACT_AUTO_OPEN=0` di lingkungan Anda.

Jika Claude merespons bahwa tidak dapat menerbitkan, atau menulis file HTML lokal tanpa tautan, tool tidak diaktifkan untuk sesi Anda. Periksa persyaratan [Availability](#availability).

<h2 id="update-an-artifact">
  Perbarui artifact
</h2>

Minta Claude untuk merevisi halaman, atau biarkan tugas yang berjalan lama menerbitkan ulang saat membuat kemajuan. Claude mengedit file yang mendasar dan menerbitkan lagi ke URL yang sama.

```text wrap theme={null}
Add a per-region breakdown below the summary chart and republish.
```

Siapa pun yang membuka halaman melihat pembaruan di tempat. Setiap penerbitan menjadi versi, dan dari kontrol **Share** di header halaman Anda dapat memilih versi mana yang dilihat viewer.

Untuk memperbarui artifact dari sesi yang berbeda, berikan Claude URL artifact dan minta untuk merevisi. Tanpa URL, sesi baru selalu membuat artifact baru daripada memperbarui yang sudah ada.

```text wrap theme={null}
Update https://claude.ai/code/artifact/5fbea6f3-... with today's numbers.
```

<h2 id="share-an-artifact">
  Bagikan artefak
</h2>

Artefak baru hanya terlihat oleh Anda. Untuk membagikannya, buka artefak di browser Anda dan gunakan kontrol **Share** di header halaman. Header menyebutkan Anda sebagai penulis artefak, jadi siapa pun yang Anda bagikan dapat melihat siapa yang menerbitkan halaman. Ini juga menautkan ke galeri Anda di [claude.ai/code/artifacts](https://claude.ai/code/artifacts), yang mencantumkan setiap artefak yang telah Anda buat.

Siapa yang dapat Anda bagikan tergantung pada paket Anda:

* **Dalam organisasi Anda**: pada paket Team dan Enterprise, berikan akses ke orang-orang tertentu di organisasi Anda, atau ke semua orang di dalamnya. Penampil masuk ke claude.ai sebagai anggota organisasi Anda untuk melihat halaman.
* **Secara publik**: bagikan tautan yang dapat dibuka siapa pun di internet, tanpa memerlukan masuk ke claude.ai. Pada paket Pro dan Max, tautan publik adalah satu-satunya cara untuk membagikan artefak. Pada paket Team dan Enterprise, berbagi publik dimatikan sampai Pemilik [mengaktifkannya untuk organisasi](#control-public-sharing).

<h3 id="let-someone-edit-with-you">
  Biarkan seseorang mengedit bersama Anda
</h3>

Orang-orang yang Anda bagikan adalah penampil secara default: mereka melihat setiap versi yang Anda terbitkan tetapi tidak dapat mengubah halaman. Pada paket Team dan Enterprise, Anda juga dapat menjadikan seseorang sebagai editor. Di dialog berbagi, tambahkan orang dan ubah peran mereka dari **viewer** menjadi **editor**.

Editor menerbitkan versi baru dengan cara yang sama seperti Anda [memperbarui artefak dari sesi lain](#update-an-artifact): mereka memberikan URL artefak kepada Claude di sesi mereka sendiri, dan Claude menarik konten saat ini dan menerbitkan ulang dengan perubahan mereka. Semua orang dengan halaman terbuka melihat setiap pembaruan secara langsung.

<h2 id="pull-live-data-with-mcp-connectors">
  Tarik data langsung dengan konektor MCP
</h2>

Sebuah artifact dapat memanggil [konektor MCP](/docs/id/mcp#use-mcp-servers-from-claude-ai) setiap kali seseorang melihatnya, sehingga halaman menampilkan data terkini daripada snapshot dari sesi yang membangunnya. Panggilan konektor dari artifact tersedia di paket Pro, Max, Team, dan Enterprise dan memerlukan Claude Code v2.1.209 atau lebih baru. Pada versi sebelumnya, Claude menerbitkan halaman dengan data apa pun yang dikumpulkan sesi saat membangunnya.

Untuk membuat halaman yang didukung konektor, beri nama konektor dan data yang Anda inginkan dalam prompt Anda:

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

Ketika Anda berencana berbagi halaman yang didukung konektor, minta Claude untuk menyertakan pesan fallback di setiap bagian langsung yang menamai konektor yang dibutuhkannya. Penampil yang kehilangan koneksi kemudian melihat apa yang harus dihubungkan daripada bagian kosong.

Artifact yang memanggil konektor tidak dapat dibagikan ke tautan publik di paket apa pun. Di paket Team dan Enterprise, Anda dapat menyimpannya tetap pribadi atau [membagikannya dalam organisasi Anda](#share-an-artifact). Di paket Pro dan Max, di mana tautan publik adalah satu-satunya cara untuk berbagi, artifact yang didukung konektor tetap pribadi untuk Anda.

<h3 id="the-page-shows-no-live-data-for-a-viewer">
  Halaman tidak menampilkan data langsung untuk penampil
</h3>

Ketika halaman yang didukung konektor dirender tetapi bagian langsung-nya tetap kosong untuk seseorang yang Anda bagikan, kerjakan penyebab-penyebab ini:

* **Penampil belum menghubungkan konektor**: konektor adalah per-akun, jadi setiap penampil memerlukan koneksi mereka sendiri ke setiap konektor yang dipanggil halaman. Mereka dapat menambahkan satu di bawah **Settings > Connectors** di claude.ai, kemudian muat ulang halaman.
* **Penampil menolak permintaan izin**: penolakan berlangsung untuk sisa pemuatan halaman itu. Memuat ulang halaman membawa permintaan izin kembali.
* **Panggilan konektor dimatikan untuk organisasi**: Pemilik mengontrol [toggle **Enable artifact connectors**](#control-connector-calls-from-artifacts) dalam pengaturan admin.

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

Mulai dari Claude Code v2.1.183, Claude menerapkan skill desain bawaan ketika membangun artifact, sehingga halaman mendapatkan palet, tipografi, dan tata letak yang disengaja tanpa prompting tambahan. Skill itu juga mencari sistem desain yang ada di proyek Anda sebelum memilih miliknya sendiri. Untuk menjaga artifacts konsisten dengan branding produk Anda, catat token desain Anda di mana Claude dapat menemukannya, seperti [CLAUDE.md](/docs/id/memory) proyek atau file tema di repositori Anda:

```markdown theme={null}
## Design system

- Colors: primary #1a4d8f, accent #f59e0b, surface #f8fafc
- Typography: Inter for body, JetBrains Mono for code
- Spacing: 8px scale, 6px border radius
```

Claude memperlakukan sistem desain Anda sebagai prioritas lebih tinggi daripada pilihannya sendiri, dan prompt Anda sebagai prioritas lebih tinggi daripada keduanya. Heading dan format di atas adalah contoh; daftar warna, font, dan spasi yang jelas apa pun berfungsi.

<h2 id="page-constraints">
  Batasan halaman
</h2>

Setiap artifact adalah satu halaman yang mandiri. Claude Code membungkus file yang Anda terbitkan dalam shell dokumen HTML dan melayaninya di bawah Content Security Policy (CSP) yang ketat, yang membentuk apa yang dapat dilakukan halaman.

| Batasan                        | Efek                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| :----------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Tidak ada permintaan eksternal | CSP memblokir skrip, stylesheet, font, dan gambar yang dimuat dari host lain, bersama dengan panggilan `fetch`, XHR, dan WebSocket. Claude menginline CSS dan JavaScript dan menyematkan gambar sebagai data URI sehingga halaman dirender tanpa permintaan eksternal apa pun. [Panggilan Connector](#pull-live-data-with-mcp-connectors) adalah pengecualian: halaman menyerahkannya ke claude.ai, yang membuat panggilan jaringan itu sendiri. |
| Tidak ada backend              | Artifact adalah halaman statis. Tidak dapat menyimpan data yang dikirimkan melalui formulir atau mengautentikasi viewer sendiri. Satu-satunya cara untuk mengambil data ketika seseorang melihatnya adalah [memanggil MCP connectors](#pull-live-data-with-mcp-connectors), bukan API miliknya sendiri.                                                                                                                                          |
| Halaman tunggal                | Tautan relatif tidak diselesaikan, karena tidak ada yang disebarkan bersama halaman. Untuk konten multi-bagian, Claude menggunakan jangkar dalam halaman daripada file terpisah.                                                                                                                                                                                                                                                                 |
| Jenis file sumber              | File yang diterbitkan harus `.html`, `.htm`, atau `.md`. File Markdown dirender sebagai HTML bergaya.                                                                                                                                                                                                                                                                                                                                            |
| Ukuran yang dirender           | Halaman yang dirender harus 16 MiB atau lebih kecil. Gambar tertanam besar adalah penyebab umum ketika penerbitan gagal karena ukuran.                                                                                                                                                                                                                                                                                                           |

Menghasilkan artifact menggunakan token output seperti respons lainnya, dan halaman bergaya lebih intensif token daripada konten yang sama sebagai teks terminal. CSS inline, JavaScript untuk kontrol interaktif, dan terutama gambar yang disematkan sebagai data URI adalah kontributor utama. Untuk mengurangi biaya token artifact:

* Lebih suka SVG, atau HTML dan CSS, untuk diagram daripada gambar raster tertanam
* Hilangkan interaktivitas yang tidak Anda butuhkan
* Buat halaman merangkum dataset besar daripada menginlinenya sepenuhnya

<h2 id="availability">
  Ketersediaan
</h2>

Artifacts memerlukan setiap kondisi di bawah. Ketika salah satu tidak terpenuhi, Claude menulis file HTML lokal atau mengatakan tidak dapat menerbitkan.

| Persyaratan          | Tersedia ketika                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| :------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Paket                | Pro, Max, Team, atau Enterprise. Pada paket Pro dan Max, artifacts bersifat pribadi untuk Anda sampai Anda membagikannya, dan tidak ada manajemen admin yang berlaku. Pada paket Team, artifacts diaktifkan secara default. Pada paket Enterprise, pemilik [mengaktifkannya](#manage-artifacts-for-your-organization) di pengaturan admin claude.ai.                                                                                                                     |
| Autentikasi          | Sesi didukung oleh akun claude.ai: masuk dengan `/login` di CLI atau aplikasi desktop. Sesi Claude Tag masuk melalui identitas agen, jadi tidak ada langkah yang diperlukan di sana. Sesi menggunakan kunci API, [gateway token](/docs/id/llm-gateway), atau kredensial penyedia cloud tidak dapat menerbitkan.                                                                                                                                                               |
| Penyedia model       | Anthropic API. Tidak tersedia di [Amazon Bedrock](/docs/id/amazon-bedrock), [Google Cloud's Agent Platform](/docs/id/google-vertex-ai), atau [Microsoft Foundry](/docs/id/microsoft-foundry).                                                                                                                                                                                                                                                                                           |
| Kebijakan organisasi | Kunci enkripsi yang dikelola pelanggan (CMEK), HIPAA, dan [Zero Data Retention](/docs/id/zero-data-retention) tidak diaktifkan untuk organisasi.                                                                                                                                                                                                                                                                                                                              |
| Permukaan            | Claude Code CLI versi 2.1.183 atau lebih baru, atau aplikasi desktop Claude versi 1.13576.0 atau lebih baru. Sesi [Claude Tag](https://claude.com/docs/claude-tag/overview) juga dapat menerbitkan artifacts ketika Claude Tag dan artifacts keduanya diaktifkan untuk organisasi. Dimatikan secara default di konteks [Agent SDK](/docs/id/agent-sdk/overview), GitHub Action, dan MCP-server, dan ketika [`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`](/docs/id/env-vars) diatur. |

<h2 id="disable-artifacts">
  Nonaktifkan artifacts
</h2>

Untuk mematikan artifacts untuk sesi Anda sendiri terlepas dari pengaturan organisasi Anda, gunakan salah satu dari:

| Metode                              | Pengaturan                                 |
| :---------------------------------- | :----------------------------------------- |
| [File pengaturan](/docs/id/settings)     | `"disableArtifact": true`                  |
| [Variabel lingkungan](/docs/id/env-vars) | `CLAUDE_CODE_DISABLE_ARTIFACT=1`           |
| [Aturan izin](/docs/id/permissions)      | Tambahkan `Artifact` ke `permissions.deny` |

<h2 id="manage-artifacts-for-your-organization">
  Kelola artifacts untuk organisasi Anda
</h2>

Pemilik pada paket Team dan Enterprise mengontrol artifacts dari [pengaturan admin claude.ai](https://claude.ai/admin-settings/claude-code). Konten artifact disimpan di infrastruktur yang dioperasikan Anthropic dan hanya terlihat oleh anggota terautentikasi dari organisasi penerbit, kecuali artifact [dibagikan secara publik](#control-public-sharing).

<h3 id="enable-or-disable-artifacts">
  Aktifkan atau nonaktifkan artifacts
</h3>

Untuk mengaktifkan atau menonaktifkan artifacts untuk seluruh organisasi, buka **Settings > Claude Code > Capabilities** dan gunakan toggle **Artifacts**. Pada paket Enterprise dengan kontrol akses berbasis peran, Anda dapat membatasi artifacts ke peran tertentu: buka **Settings > Roles**, edit peran, dan atur izin **Artifacts** di bawah grup **Claude Code**.

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

Untuk mengatur berapa lama artifacts disimpan sebelum penghapusan otomatis, buka **Settings > Data & privacy controls**. Anda dapat mengatur periode retensi terpisah untuk artifacts yang masih pribadi untuk penulis mereka dan artifacts yang telah dibagikan.

<h3 id="review-the-audit-log">
  Tinjau log audit
</h3>

Penerbitan, berbagi, dan menghapus artifact masing-masing muncul di log audit organisasi Anda di bawah jenis acara `claude_artifact_*`, keluarga yang sama digunakan untuk artifacts yang dibuat dalam percakapan claude.ai.

<h3 id="allowlist-the-viewer-domain">
  Daftar putih domain viewer
</h3>

Viewer di claude.ai memuat setiap artifact dari asal `*.claudeusercontent.com` yang disandboxkan. Jika organisasi Anda membatasi akses jaringan keluar, tambahkan domain itu ke daftar putih Anda bersama `claude.ai`. Lihat [Network access requirements](/docs/id/network-config#network-access-requirements) untuk daftar lengkap.

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
