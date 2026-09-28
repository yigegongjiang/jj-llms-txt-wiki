> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code di Slack

> Delegasikan tugas coding langsung dari workspace Slack Anda. Anthropic sedang menghentikan versi awal ini untuk workspace Team dan Enterprise demi Claude Tag; versi ini tetap menjadi jalur setup pada paket Pro dan Max.

<Warning>
  Halaman ini mendokumentasikan Claude Code di Slack versi awal, yang menjalankan setiap sesi di bawah akun pengguna individual.

  * **Paket Team dan Enterprise:** Anthropic sedang menghentikan versi ini demi [Claude Tag](https://claude.com/product/tag), yang menjalankan @Claude sebagai identitas bersama organisasi Anda dengan akses yang dikonfigurasi admin. Aplikasi Slack dan handle @Claude yang sudah ada tetap ada, dan tim akun Anthropic Anda dapat memberitahu Anda tanggal cutover-nya. [Atur Claude Tag](https://claude.com/docs/claude-tag/overview) untuk workspace baru; untuk memindahkan workspace yang sudah menggunakan versi ini, lihat [Migrasi dari Claude di Slack yang lebih awal](https://claude.com/docs/claude-tag/admins/migrate-from-earlier).
  * **Paket Pro dan Max:** Claude Tag tidak tersedia pada paket individual, jadi halaman ini tetap menjadi jalur setup.
</Warning>

Claude Code di Slack membawa kekuatan Claude Code langsung ke workspace Slack Anda. Ketika Anda menyebutkan `@Claude` dengan tugas coding, Claude secara otomatis mendeteksi niat dan membuat sesi Claude Code cloud, memungkinkan Anda untuk mendelegasikan pekerjaan pengembangan tanpa meninggalkan percakapan tim Anda.

Integrasi ini dibangun di atas aplikasi Claude untuk Slack yang sudah ada tetapi menambahkan perutean cerdas ke sesi Claude Code cloud untuk permintaan yang terkait dengan coding. Setiap sesi berjalan di bawah akun Claude Anda sendiri, menggunakan repositori yang terhubung dan batas rencana Anda.

<h2 id="use-cases">
  Kasus penggunaan
</h2>

* **Investigasi dan perbaikan bug**: Minta Claude untuk menyelidiki dan memperbaiki bug segera setelah dilaporkan di saluran Slack.
* **Review kode cepat dan modifikasi**: Biarkan Claude mengimplementasikan fitur kecil atau refactor kode berdasarkan umpan balik tim.
* **Debugging kolaboratif**: Ketika diskusi tim memberikan konteks penting (misalnya, reproduksi error atau laporan pengguna), Claude dapat menggunakan informasi tersebut untuk menginformasikan pendekatan debugging-nya.
* **Eksekusi tugas paralel**: Mulai tugas coding di Slack sambil melanjutkan pekerjaan lain, menerima notifikasi saat selesai.

<h2 id="prerequisites">
  Prasyarat
</h2>

Sebelum menggunakan Claude Code di Slack, pastikan Anda memiliki hal berikut:

| Persyaratan       | Detail                                                                                                       |
| :---------------- | :----------------------------------------------------------------------------------------------------------- |
| Claude Plan       | Pro, Max, Team, atau Enterprise dengan akses Claude Code (kursi premium atau Chat + Claude Code seats)       |
| Cloud sessions    | [Cloud sessions](/docs/id/claude-code-on-the-web) diaktifkan untuk akun Anda                                      |
| Akun GitHub       | Terhubung di [claude.ai/code](https://claude.ai/code) dengan setidaknya satu repositori yang terauthentikasi |
| Autentikasi Slack | Akun Slack Anda tertaut ke akun Claude Anda melalui aplikasi Claude                                          |

<h2 id="setting-up-claude-code-in-slack">
  Menyiapkan Claude Code di Slack
</h2>

<Steps>
  <Step title="Instal Aplikasi Claude di Slack">
    Administrator workspace harus menginstal aplikasi Claude dari Slack App Marketplace. Kunjungi [Slack App Marketplace](https://slack.com/marketplace/A08SF47R6P4) dan klik "Add to Slack" untuk memulai proses instalasi.
  </Step>

  <Step title="Hubungkan akun Claude Anda">
    Setelah aplikasi diinstal, autentikasi akun Claude individual Anda:

    1. Buka aplikasi Claude di Slack dengan mengklik "Claude" di bagian Apps Anda
    2. Buka tab App Home
    3. Klik "Connect" untuk menghubungkan akun Slack Anda dengan akun Claude Anda
    4. Selesaikan alur autentikasi di browser Anda
  </Step>

  <Step title="Konfigurasi cloud sessions">
    Pastikan cloud sessions dikonfigurasi dengan benar untuk akun Anda:

    * Kunjungi [claude.ai/code](https://claude.ai/code) dan masuk dengan akun yang sama yang Anda hubungkan ke Slack
    * Hubungkan akun GitHub Anda jika belum terhubung
    * Autentikasi setidaknya satu repositori yang ingin Anda gunakan Claude untuk bekerja
  </Step>

  <Step title="Pilih mode perutean Anda">
    Setelah menghubungkan akun Anda, konfigurasi bagaimana Claude menangani pesan Anda di Slack. Buka Claude App Home di Slack untuk menemukan pengaturan **Routing Mode**.

    | Mode            | Perilaku                                                                                                                                                                                                                                                        |
    | :-------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
    | **Code only**   | Claude merutkan semua @mentions ke sesi Claude Code. Terbaik untuk tim yang menggunakan Claude di Slack secara eksklusif untuk tugas pengembangan.                                                                                                              |
    | **Code + Chat** | Claude menganalisis setiap pesan dan secara cerdas merutkan antara Claude Code (untuk tugas coding) dan Claude Chat (untuk penulisan, analisis, dan pertanyaan umum). Terbaik untuk tim yang menginginkan satu titik masuk @Claude untuk semua jenis pekerjaan. |

    <Note>
      Dalam mode Code + Chat, jika Claude merutkan pesan ke Chat tetapi Anda menginginkan sesi coding, Anda dapat mengklik "Retry as Code" untuk membuat sesi Claude Code sebagai gantinya. Demikian pula, jika itu dirutkan ke Code tetapi Anda menginginkan sesi Chat, Anda dapat memilih opsi itu di thread tersebut.
    </Note>
  </Step>

  <Step title="Tambahkan Claude ke saluran">
    Claude tidak secara otomatis ditambahkan ke saluran apa pun setelah instalasi. Untuk menggunakan Claude di saluran, undang dengan mengetik `/invite @Claude` di saluran tersebut. Claude hanya dapat merespons @mentions di saluran tempat itu telah ditambahkan.
  </Step>
</Steps>

<h2 id="how-it-works">
  Cara kerjanya
</h2>

<h3 id="automatic-detection">
  Deteksi otomatis
</h3>

Dalam mode routing Code + Chat, ketika Anda menyebutkan @Claude di saluran atau thread Slack, Claude secara otomatis mendeteksi apakah pesan Anda adalah tugas coding. Tugas coding dikirim ke sesi cloud Claude Code. Apa pun yang lain mendapatkan balasan chat biasa. Dalam mode Code only, setiap @mention dikirim ke Claude Code.

Anda juga dapat secara eksplisit memberi tahu Claude untuk menangani permintaan sebagai tugas coding, bahkan jika itu tidak secara otomatis mendeteksinya.

<Note>
  Claude Code di Slack hanya berfungsi di saluran (publik atau pribadi). Itu tidak berfungsi di pesan langsung (DM).
</Note>

<h3 id="context-gathering">
  Pengumpulan konteks
</h3>

**Dari thread**: Ketika Anda @mention Claude di thread, itu mengumpulkan konteks dari semua pesan di thread tersebut untuk memahami percakapan lengkap.

**Dari saluran**: Ketika disebutkan langsung di saluran, Claude melihat pesan saluran terbaru untuk konteks yang relevan.

Konteks ini membantu Claude memahami masalah, memilih repositori yang sesuai, dan menginformasikan pendekatan terhadap tugas.

<Warning>
  Ketika @Claude dipanggil di Slack, Claude diberi akses ke konteks percakapan untuk lebih memahami permintaan Anda. Claude dapat mengikuti arahan dari pesan lain dalam konteks, jadi pengguna harus memastikan untuk hanya menggunakan Claude dalam percakapan Slack yang terpercaya.
</Warning>

<h3 id="session-flow">
  Alur sesi
</h3>

1. **Inisiasi**: Anda @mention Claude dengan permintaan coding
2. **Deteksi**: Claude menganalisis pesan Anda dan mendeteksi niat coding
3. **Pembuatan sesi**: Sesi Claude Code baru dibuat di claude.ai/code
4. **Pembaruan kemajuan**: Claude memposting pembaruan status ke thread Slack Anda saat pekerjaan berlangsung
5. **Penyelesaian**: Saat selesai, Claude @mention Anda dengan ringkasan dan tombol tindakan
6. **Tinjauan**: Klik "View Session" untuk melihat transkrip lengkap, atau "Create PR" untuk membuka pull request

<h2 id="user-interface-elements">
  Elemen antarmuka pengguna
</h2>

<h3 id="message-actions">
  Tindakan pesan
</h3>

* **View Session**: Membuka sesi Claude Code lengkap di browser Anda di mana Anda dapat melihat semua pekerjaan yang dilakukan, melanjutkan sesi, atau membuat permintaan tambahan.
* **Create PR**: Membuat pull request langsung dari perubahan sesi.
* **Retry as Code**: Jika Claude awalnya merespons sebagai asisten chat tetapi Anda menginginkan sesi coding, klik tombol ini untuk mencoba ulang permintaan sebagai tugas Claude Code.
* **Change Repo**: Memungkinkan Anda untuk memilih repositori yang berbeda jika Claude memilih dengan tidak benar.

<h3 id="repository-selection">
  Pemilihan repositori
</h3>

Claude secara otomatis memilih repositori berdasarkan konteks dari percakapan Slack Anda. Jika beberapa repositori dapat berlaku, Claude dapat menampilkan dropdown yang memungkinkan Anda memilih yang benar.

<h2 id="access-and-permissions">
  Akses dan izin
</h2>

<h3 id="user-level-access">
  Akses tingkat pengguna
</h3>

| Jenis Akses             | Persyaratan                                                                          |
| :---------------------- | :----------------------------------------------------------------------------------- |
| Sesi Claude Code        | Setiap pengguna menjalankan sesi di bawah akun Claude mereka sendiri                 |
| Penggunaan & Batas Laju | Sesi dihitung terhadap batas rencana pengguna individual                             |
| Akses Repositori        | Pengguna hanya dapat mengakses repositori yang telah mereka hubungkan secara pribadi |
| Riwayat Sesi            | Sesi muncul di riwayat Claude Code Anda di claude.ai/code                            |

<h3 id="workspace-level-access">
  Akses tingkat workspace
</h3>

Administrator workspace Slack mengontrol apakah aplikasi Claude tersedia di workspace mereka:

| Kontrol                    | Deskripsi                                                                                                                 |
| :------------------------- | :------------------------------------------------------------------------------------------------------------------------ |
| Instalasi aplikasi         | Admin workspace memutuskan apakah akan menginstal aplikasi Claude dari Slack App Marketplace                              |
| Distribusi Enterprise Grid | Untuk organisasi Enterprise Grid, admin organisasi dapat mengontrol workspace mana yang memiliki akses ke aplikasi Claude |
| Penghapusan aplikasi       | Menghapus aplikasi dari workspace segera mencabut akses untuk semua pengguna di workspace tersebut                        |

<h3 id="channel-based-access-control">
  Kontrol akses berbasis saluran
</h3>

Menginstal aplikasi tidak menambahkan Claude ke saluran apa pun. Claude merespons @mentions hanya di saluran tempat itu telah ditambahkan; undang dengan `/invite @Claude`. Ini bekerja di saluran publik dan pribadi. Admin dapat mengontrol siapa yang menggunakan Claude Code dengan mengelola saluran mana Claude diundang dan siapa yang memiliki akses ke saluran tersebut. Ini menambahkan lapisan kontrol akses di luar izin tingkat workspace.

<h2 id="what’s-accessible-where">
  Apa yang dapat diakses di mana
</h2>

**Di Slack**: Anda akan melihat pembaruan status, ringkasan penyelesaian, dan tombol tindakan. Transkrip lengkap disimpan dan selalu dapat diakses.

**Di claude.ai/code**: Sesi Claude Code lengkap dengan riwayat percakapan lengkap, semua perubahan kode, dan operasi file. Sesi tetap berada di riwayat Claude Code Anda di [claude.ai/code](https://claude.ai/code), di mana Anda dapat melanjutkan sesi sebelumnya, mereferensikannya, atau membuat pull request.

Untuk akun Enterprise dan Team, sesi yang dibuat dari Claude di Slack secara otomatis terlihat oleh organisasi. Lihat [cloud session sharing](/docs/id/claude-code-on-the-web#share-sessions) untuk detail lebih lanjut.

<h2 id="best-practices">
  Praktik terbaik
</h2>

<h3 id="writing-effective-requests">
  Menulis permintaan yang efektif
</h3>

* **Jadilah spesifik**: Sertakan nama file, nama fungsi, atau pesan error ketika relevan.
* **Berikan konteks**: Sebutkan repositori atau proyek jika tidak jelas dari percakapan.
* **Tentukan kesuksesan**: Jelaskan seperti apa "selesai"—haruskah Claude menulis tes? Memperbarui dokumentasi? Membuat PR?
* **Gunakan thread**: Balas di thread saat membahas bug atau fitur sehingga Claude dapat mengumpulkan konteks lengkap.

<h3 id="when-to-use-slack-vs-web">
  Kapan menggunakan Slack vs. web
</h3>

**Gunakan Slack ketika**: Konteks sudah ada dalam diskusi Slack, Anda ingin memulai tugas secara asinkron, atau Anda berkolaborasi dengan rekan tim yang membutuhkan visibilitas.

**Gunakan web secara langsung ketika**: Anda perlu mengunggah file, menginginkan interaksi real-time selama pengembangan, atau bekerja pada tugas yang lebih panjang dan kompleks.

<h2 id="troubleshooting">
  Pemecahan masalah
</h2>

<h3 id="claude-code-is-not-enabled-for-your-account">
  "Claude Code tidak diaktifkan untuk akun Anda"
</h3>

Kesalahan ini berarti akun Claude Anda belum memiliki lingkungan cloud. Masuk ke [claude.ai/code](https://claude.ai/code) sekali dengan akun yang sama yang Anda hubungkan ke Slack dan selesaikan [onboarding web](/docs/id/web-quickstart#connect-github), yang membuat lingkungan cloud default Anda atau meminta Anda untuk membuatnya. Kesalahan hilang pada penyebutan Anda berikutnya. Setiap pengguna harus melakukan ini secara individual.

<h3 id="sessions-not-starting">
  Sesi tidak dimulai
</h3>

1. Verifikasi akun Claude Anda terhubung di Claude App Home
2. Periksa bahwa sesi cloud diaktifkan untuk akun Anda
3. Pastikan Anda memiliki setidaknya satu repositori GitHub yang terhubung ke Claude Code

<h3 id="sessions-from-a-claude-tag-channel-fail-to-start">
  Sesi dari saluran Claude Tag gagal dimulai
</h3>

Entri ini berlaku untuk ruang kerja yang menggunakan [Claude Tag](https://claude.com/docs/claude-tag/overview), di mana Claude bekerja di saluran sebagai identitas bersama organisasi Anda, bukan sebagai akun anggota mana pun. Jika Anda membuat lingkungan cloud saluran di [claude.ai/code](https://claude.ai/code), itu milik akun pribadi Anda, dan Claude tidak dapat memulai sesi saluran di lingkungan pribadi. Claude Code gagal sesi segera, dan mencoba lagi tidak membantu.

Jika Anda adalah Pemilik dan lingkungannya adalah milik Anda sendiri, [bagikan dengan organisasi](/docs/id/cloud-environments#organization-shared-environments) dari pemilih lingkungan. Jika tidak, Pemilik membuat ulang sebagai lingkungan bersama organisasi dari halaman **Cloud environments** di [pengaturan admin](https://claude.ai/admin-settings).

Anda dapat menerapkannya dengan dua cara:

* Tetapkan sebagai default organisasi di [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code).
* [Tetapkan di saluran](https://claude.com/docs/claude-tag/admins/troubleshooting#channel-sessions-use-the-wrong-environment-or-can%E2%80%99t-find-one) di pengaturan admin Claude Tag.

Jika Anda bukan Pemilik, kirimkan entri ini ke salah satu.

<h3 id="repository-not-showing">
  Repositori tidak ditampilkan
</h3>

1. Hubungkan repositori di [claude.ai/code](https://claude.ai/code)
2. Verifikasi izin GitHub Anda untuk repositori tersebut
3. Coba putuskan dan hubungkan kembali akun GitHub Anda

<h3 id="wrong-repository-selected">
  Repositori yang salah dipilih
</h3>

1. Klik tombol "Change Repo" untuk memilih repositori yang berbeda
2. Sertakan nama repositori dalam permintaan Anda untuk pemilihan yang lebih akurat

<h3 id="authentication-errors">
  Kesalahan autentikasi
</h3>

1. Putuskan dan hubungkan kembali akun Claude Anda di App Home
2. Pastikan Anda masuk ke akun Claude yang benar di browser Anda
3. Periksa bahwa rencana Claude Anda mencakup akses Claude Code

<h2 id="current-limitations">
  Keterbatasan saat ini
</h2>

* **GitHub saja**: repositori harus berada di GitHub.
* **Satu PR sekaligus**: setiap sesi dapat membuat satu pull request.
* **Akses sesi cloud diperlukan**: pengguna memerlukan akses ke [sesi cloud](/docs/id/claude-code-on-the-web); tanpa itu, Claude merespons dengan respons chat standar.

<h2 id="related-resources">
  Sumber daya terkait
</h2>

<CardGroup>
  <Card title="Claude Code di cloud" icon="cloud" href="/docs/id/claude-code-on-the-web">
    Pelajari lebih lanjut tentang sesi cloud
  </Card>

  <Card title="Claude untuk Slack" icon="slack" href="https://claude.com/claude-and-slack">
    Dokumentasi Claude untuk Slack umum
  </Card>

  <Card title="Claude Tag" icon="users" href="https://claude.com/docs/claude-tag/overview">
    @Claude yang dikelola organisasi di Slack dengan akses yang dikonfigurasi admin
  </Card>

  <Card title="Slack App Marketplace" icon="store" href="https://slack.com/marketplace/A08SF47R6P4">
    Instal aplikasi Claude dari Slack Marketplace
  </Card>

  <Card title="Claude Help Center" icon="circle-question" href="https://support.claude.com">
    Dapatkan dukungan tambahan
  </Card>
</CardGroup>
