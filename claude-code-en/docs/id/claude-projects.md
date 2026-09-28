> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Biarkan Claude mengoordinasikan pekerjaan berkelanjutan dengan Projects

> Berikan Claude sekumpulan pekerjaan terkait dalam satu percakapan dan biarkan ia mengoordinasikan sesi cloud paralel yang berbagi repositori, instruksi, dan memori.

<Note>
  Projects sedang dalam beta publik di paket Pro dan Max dan diluncurkan secara bertahap, dimulai dengan akun yang telah menggunakan [sesi cloud](/docs/id/claude-code-on-the-web) dan tidak memiliki projects yang ada di claude.ai chat atau Cowork. Mereka belum tersedia di paket Team atau Enterprise. Jika **Projects** tidak muncul di sidebar di [claude.ai/code](https://claude.ai/code) atau di tab Code dari [aplikasi desktop](/docs/id/desktop), peluncuran belum mencapai akun Anda, dan Anda dapat [bergabung dengan daftar tunggu](https://claude.com/form/projects). [Jalankan agen secara paralel](/docs/id/agents) mencantumkan apa yang dapat Anda gunakan sementara itu.
</Note>

Sebuah project adalah satu percakapan berkelanjutan di mana Claude mengoordinasikan aliran pekerjaan terkait untuk Anda. Anda memberi tahu apa yang perlu dilakukan dan ia memulai thread untuk setiap tugas.

Setiap thread biasanya adalah [sesi cloud](/docs/id/claude-code-on-the-web): Claude Code berjalan di cloud daripada di mesin Anda. Ketika sebuah tugas membutuhkan sesuatu yang hanya dimiliki komputer Anda, Anda dapat meminta Claude untuk menjalankan thread itu di komputer Anda sebagai gantinya melalui [Remote Control](/docs/id/remote-control). Thread berjalan secara paralel dan Anda dapat memeriksanya dan mengarahkannya dari ponsel Anda. Thread cloud terus berjalan setelah Anda menutup laptop.

Tanpa project, menjalankan beberapa sesi berarti melakukan koordinasi sendiri: Anda memutuskan apa yang dikerjakan masing-masing, mengulangi latar belakang yang sama di awal masing-masing, dan memeriksa kembali untuk melihat mana yang selesai atau membutuhkan jawaban. Dengan project, Anda malah:

* **Kirim pekerjaan ke satu tempat**: tempel laporan bug, stack trace, atau daftar tugas ke dalam percakapan kapan pun muncul. Claude memulai thread untuk setiap bagian pekerjaan atau meneruskannya ke thread yang sudah bekerja di area itu, dan menjawab pertanyaan cepat di tempat.
* **Atur konteks sekali**: setiap thread baru dimulai dengan instruksi project, jadi aturan yang Anda nyatakan sekali, seperti branch mana yang ditargetkan, mencapai semuanya.
* **Pergi dan kembali ke pekerjaan yang selesai**: ketika Anda kembali satu jam kemudian atau pagi berikutnya, pane **Overview** menunjukkan thread mana yang selesai, pull request mana yang siap untuk ditinjau, dan thread mana yang menunggu jawaban Anda.

Jika Anda sudah tahu pekerjaan yang ingin dijalankan project, langsung ke [Buat project](#create-a-project).

<h2 id="when-to-use-a-project">
  Kapan menggunakan project
</h2>

Project layak dibuat ketika pekerjaan memiliki tujuan yang melampaui satu sesi dan terus menghasilkan tugas. Jenis pekerjaan ini cocok untuk project:

* **Satu tujuan di banyak repositori**: "Bawa setiap layanan ke konfigurasi lint baru." Claude dapat menjalankan thread per repositori, masing-masing dengan pull request sendiri, dan pane [**Overview**](#see-what-needs-you-in-overview) menunjukkan mana yang siap untuk ditinjau.
* **Area yang terus Anda isi**: bug, stack trace, dan permintaan review untuk satu layanan, ditempel ke dalam percakapan saat sampai ke Anda. Jebakan yang Anda beri tahu Claude untuk diingat setelah satu perbaikan ada di [project memory](#give-a-project-standing-context) untuk yang berikutnya.
* **Build atau migrasi yang lebih besar dari satu sesi**: "Bangun apa yang dijelaskan `docs/spec.md`" atau "Pindahkan aplikasi dari ORM yang sudah usang." Pekerjaan terbagi menjadi thread yang masing-masing mengambil bagian, keputusan yang Anda minta Claude untuk diingat di awal mencapai thread yang lebih baru, dan spec berubah serta bug yang Anda temukan selama build masuk ke percakapan yang sama.
* **Pekerjaan yang bukan kode**: folder kontrak atau ekspor tiket dukungan yang terus Anda kembali dengan pertanyaan baru, seperti "temukan sepuluh kesalahan integrasi paling umum di tiket ini." Unggah dokumen alih-alih menambahkan repositori, dan thread memberikan setiap laporan sebagai file di tab [**Library**](#see-what-needs-you-in-overview) project.

Di salah satu dari mereka Anda dapat mengirim batch tugas, beri tahu Claude untuk memulai tanpa meminta Anda mengonfirmasi, pergi, dan temukan thread yang membutuhkan Anda di bawah [**Waiting on you**](#see-what-needs-you-in-overview) ketika Anda kembali, atau minta Claude untuk menjadwalkan bagian pekerjaan sebagai [routine](/docs/id/routines). Jika salah satu ini adalah situasi Anda, [buat project](#create-a-project).

<h3 id="when-something-else-fits-better">
  Kapan sesuatu yang lain lebih cocok
</h3>

Thread bekerja pada repositori GitHub dan pada file, folder, dan folder Google Drive yang Anda unggah ke project, bukan pada file atau alat yang hanya ada di mesin Anda. Jika tugas memerlukan mesin Anda, minta Claude untuk menjalankan threadnya di sana melalui [Remote Control](/docs/id/remote-control). [Limitations](#limitations) mencantumkan apa yang diperlukan. Sesuatu yang lain lebih cocok dalam kasus ini:

* **Satu tugas yang cocok dalam satu sesi**: "Perbaiki tes login yang tidak stabil." Mulai [sesi cloud](/docs/id/claude-code-on-the-web) sendiri.
* **Pekerjaan di mana setiap tugas memerlukan mesin Anda**: database lokal, emulator perangkat, atau API di belakang VPN Anda. Gunakan sesi lokal, atau [agent view](/docs/id/agent-view) untuk menjalankan beberapa sekaligus. Jika pekerjaan hanya memerlukan file lokal, unggah ke project.
* **Satu tugas yang berulang pada jadwal tanpa percakapan di sekitarnya**: "Posting laporan dependensi setiap Senin." Buat [routine](/docs/id/routines) sendiri.
* **Beberapa orang memberi Claude pekerjaan dan mengarahkannya bersama di saluran Slack**: lihat [Claude Tag](https://claude.com/docs/claude-tag/overview).

Project menggunakan batas paket yang sama dengan sesi Claude Code lainnya dan menggunakannya lebih cepat. [Usage and cost](#usage-and-cost) mencakup apa yang menggunakan paket Anda dan cara menjaganya tetap rendah.

<h2 id="how-a-project-is-organized">
  Bagaimana project diorganisir
</h2>

Project adalah satu percakapan koordinasi dengan Claude ditambah thread yang dimulainya untuk melakukan pekerjaan. Ini adalah bagian-bagiannya:

* **Percakapan project**: satu sesi jangka panjang di mana Claude bertindak sebagai koordinator. Ini mengambil apa yang Anda kirim, memutuskan apa yang menjadi thread, dan melacak setiap thread yang dimulainya. Ini melihat apa yang dilaporkan thread kembali, bukan setiap langkah yang mereka ambil.
* **Threads**: para pekerja. Masing-masing adalah sesi terpisah dengan jendela konteks sendiri yang melakukan satu bagian pekerjaan dan melaporkan kembali ke percakapan ketika selesai. Thread cloud bekerja di cabang sendiri dan membuka pull request ketika pekerjaan memanggilnya.
* **Apa yang dimulai setiap cloud thread dengan**:
  * Repositori dan file project, ditambah [instruksi dan memori](#give-a-project-standing-context)
  * `CLAUDE.md` dan skills di [setiap repositori project](#what-threads-pick-up-from-your-repositories), dan di project dengan satu repositori, aturan izin dan hooks repositori itu juga
  * [Connectors](#get-skills-plugins-connectors-and-tools-into-threads) di akun claude.ai Anda
  * [Cloud environment](#choose-an-environment-for-threads) yang menetapkan akses jaringan, variabel lingkungan, kredensial API, dan alat yang terinstal
* **Pane Overview**: di mana Anda [melihat semua thread sekaligus](#see-what-needs-you-in-overview) dan mana yang membutuhkan Anda. Tab lainnya adalah **Library** untuk file yang Anda tambahkan dan file yang dihasilkan thread, **Pull requests** untuk yang dibuka thread, dan **Routines** untuk pekerjaan terjadwal di project.

Cloud threads tidak mengambil apa pun dari setup Claude Code di mesin Anda sendiri. [Dapatkan skills, plugins, connectors, dan tools ke dalam threads](#get-skills-plugins-connectors-and-tools-into-threads) mencakup cara memberikan mereka apa yang mereka butuhkan.

Berikut adalah bagaimana bagian-bagian itu terhubung, dari Anda melalui percakapan ke thread yang melakukan pekerjaan, dengan **Overview** melacak keadaan mereka:

<Frame>
  <img src="https://mintcdn.com/claude-code/e8CLbxM17eD7cAiv/images/claude-projects-overview.svg?fit=max&auto=format&n=e8CLbxM17eD7cAiv&q=85&s=dbf446f69f0bbdb9961d21af207cb93b" className="dark:hidden" alt="Diagram project. Anda menulis di percakapan project, di mana Claude menjawab atau memulai thread. Setiap cloud thread bekerja di cabang dan pull request sendiri. Pane Overview mencantumkan thread berdasarkan keadaan, seperti siap untuk ditinjau, menunggu Anda, dan bekerja." width="600" height="250" data-path="images/claude-projects-overview.svg" />

  <img src="https://mintcdn.com/claude-code/e8CLbxM17eD7cAiv/images/claude-projects-overview-dark.svg?fit=max&auto=format&n=e8CLbxM17eD7cAiv&q=85&s=549a5ba9fea8433729babc37a1f6e9c8" className="hidden dark:block" alt="Diagram project. Anda menulis di percakapan project, di mana Claude menjawab atau memulai thread. Setiap cloud thread bekerja di cabang dan pull request sendiri. Pane Overview mencantumkan thread berdasarkan keadaan, seperti siap untuk ditinjau, menunggu Anda, dan bekerja." width="600" height="250" data-path="images/claude-projects-overview-dark.svg" />
</Frame>

<h2 id="create-a-project">
  Buat project
</h2>

Anda membuat dan menggunakan projects di [claude.ai/code](https://claude.ai/code), di tab Code dari aplikasi desktop, atau di aplikasi mobile Claude untuk [iOS](https://apps.apple.com/us/app/claude-by-anthropic/id6473753684) dan [Android](https://play.google.com/store/apps/details?id=com.anthropic.claude). Di browser dan aplikasi desktop ada dua cara untuk memulai project:

* **Dari awal**, ketika Anda tahu aliran pekerjaan yang ingin dijalankan Claude: buka dialog **New project** dan beri nama. [Mulai project baru dari awal](#start-a-new-project-from-scratch) memandu melalui dialog.
* **Dari sesi cloud yang sudah melakukan pekerjaan**: pilih **Continue as a project** dari menu sesi itu, dan Claude mengusulkan setup project dari apa yang dilakukan sesi. Lihat [Mulai dari sesi cloud yang ada](#start-from-an-existing-cloud-session).

Bagaimanapun, [periksa prasyarat](#check-the-prerequisites) terlebih dahulu.

<h3 id="check-the-prerequisites">
  Periksa prasyarat
</h3>

Sebelum Anda membuat project, periksa paket Anda, setup GitHub Anda, dan apa yang dibutuhkan pekerjaan untuk dijangkau:

* **Paket**: Anda berada di Pro atau Max dan **Projects** ditampilkan di sidebar Anda.
* **GitHub, jika project akan bekerja pada kode**: kode Anda ada di github.com daripada GitHub Enterprise Server, GitLab, atau Bitbucket, akun GitHub yang terhubung memiliki akses push ke kode, dan Claude GitHub App terinstal di atasnya. Jika Anda menghubungkan GitHub dengan [`/web-setup`](/docs/id/web-quickstart#connect-from-your-terminal), token itu memungkinkan sesi cloud lain Anda menjangkau repositori tetapi tidak cukup untuk thread project, yang membutuhkan Claude GitHub App. [Atur akses GitHub](#set-up-github-access) memiliki langkah-langkahnya.
* **Jaringan, kredensial, dan alat**: untuk thread cloud, ini berasal dari [cloud environment](#choose-an-environment-for-threads) project. Environment default sudah menjangkau [registri paket umum](/docs/id/cloud-environments#default-allowed-domains), jadi periksa ini hanya jika pekerjaan membutuhkan domain lain, rahasia, atau alat yang tidak terinstal sebelumnya. Jika pekerjaan membutuhkan server MCP, periksa bahwa itu ditampilkan sebagai terhubung di [connectors claude.ai](https://claude.ai/customize/connectors) Anda.

<h3 id="start-a-new-project-from-scratch">
  Mulai project baru dari awal
</h3>

Memulai project dari awal berarti membuka dialog **New project**, memberi nama aliran pekerjaan, dan secara opsional memberikan tujuan dan repositori serta file yang dikerjakan. Hanya nama yang diperlukan, jadi Anda dapat membuat project terlebih dahulu dan mengisinya saat pekerjaan berkembang.

<Steps>
  <Step title="Buka Projects">
    Di [claude.ai/code](https://claude.ai/code) atau di tab Code dari aplikasi desktop, pilih **Projects** di sidebar kiri, lalu pilih **New project**. Di browser Anda juga dapat langsung ke [claude.ai/code/projects/browse](https://claude.ai/code/projects/browse).
  </Step>

  <Step title="Isi dialog New project">
    Batasi project ke satu aliran pekerjaan yang akan terus Anda tambahkan, seperti semua yang diperlukan untuk menjaga satu API di bawah target latensinya. [Kapan menggunakan project](#when-to-use-a-project) memiliki lebih banyak contoh. Kemudian isi bidang dialog:

    * **Name**: bagaimana project muncul di daftar **Projects**.
    * **Goal** (opsional): satu baris tentang apa yang Anda coba capai, seperti "Pertahankan latensi API p95 di bawah 200 ms". Claude dalam percakapan bekerja menuju itu. Tanpa tujuan, Claude bekerja dari tugas yang Anda kirim, dan Anda dapat menambahkan tujuan nanti di **Project settings > General**.
    * **Context** (opsional): repositori GitHub yang dikerjakan project ini, ditambah file, folder, atau folder Google Drive apa pun yang harus dibaca thread. Klik **Add** untuk masing-masing. Tambahkan repositori yang paling banyak dibutuhkan tugas daripada setiap yang mungkin disentuh pekerjaan; [Tentukan repositori mana yang akan ditambahkan](#decide-which-repositories-to-add) mencakup pilihan, dan Anda dapat menambahkan lebih banyak nanti di **Project settings > Environment**.

    Aturan berdiri tentang cara thread harus bekerja masuk ke [project instructions](#give-a-project-standing-context), yang Anda atur setelah project ada.
  </Step>

  <Step title="Buat project">
    Klik **Create project**. Percakapan project terbuka dengan kotak pesan di bagian bawah, di mana Anda mendeskripsikan pekerjaan untuk Claude.

    Di project pertama Anda, Claude mengambil giliran sendiri segera setelah project dibuat, kecuali Anda mengirim pesan terlebih dahulu. Giliran itu menggunakan paket Anda. Di dalamnya, Claude mungkin:

    * Mulai satu thread yang mengeksplorasi repositori tanpa mengubah apa pun dan mengusulkan langkah berikutnya, jika project memiliki repositori yang dapat dibaca.
    * Posting **Setup recommendations** yang diambil dari sesi cloud terbaru Anda: repositori untuk ditambahkan, routine untuk dibuat, dan thread yang dapat dimulai. Setiap repositori dan routine yang direkomendasikan dimulai dengan diaktifkan. Matikan yang tidak Anda inginkan, lalu klik **Update setup** untuk menambahkan sisanya, atau abaikan rekomendasi dan deskripsikan pekerjaan sendiri.
  </Step>
</Steps>

Project sekarang tercantum di bawah **Projects** di sidebar, dan percakapannya terbuka. [Batch pertama Anda](#your-first-batch) mencakup apa yang harus diatur sebelum Anda mengirimnya pekerjaan.

<h3 id="start-from-an-existing-cloud-session">
  Mulai dari sesi cloud yang ada
</h3>

Jika Anda sudah memiliki sesi cloud yang melakukan pekerjaan yang seharusnya ada di project, buka menu sesi di sidebar dan pilih **Continue as a project** atau **Move to project**:

* **Continue as a project** membuat project baru bernama sesuai sesi dan membukanya. Claude membaca sesi dan memposting **Setup recommendations** dalam percakapan untuk Anda konfirmasi. Sesi asli tetap ada di daftar sesi Anda, dan jika sedang dalam giliran, terus berjalan, jadi hentikan sendiri jika Anda tidak ingin keduanya bekerja sekaligus. Jika Anda menggunakan banner **Set up project** yang dapat muncul di atas kotak pesan sesi cloud, hasilnya sama, kecuali giliran sesi yang sedang berjalan berhenti setelah project terbuka.
* **Move to project** membawa pekerjaan sesi ke project yang ada. Ini memposting pesan di percakapan project itu meminta Claude untuk membaca sesi dan melanjutkan dari mana ia berhenti, dan pekerjaan baru berlanjut di thread project sendiri. Sesi asli tetap ada di daftar sesi Anda, tidak berubah.

<h3 id="set-up-github-access">
  Atur akses GitHub
</h3>

Sebagian besar setup GitHub terjadi sekali, bukan per project. Anda menghubungkan akun GitHub ke Claude sekali, dan Claude GitHub App diinstal sekali per repositori, atau sekali untuk seluruh organisasi GitHub jika Anda memberikan semua repositori. Anda kembali ke langkah-langkah ini ketika Anda menambahkan repositori yang Claude GitHub App belum cover atau satu di organisasi GitHub yang memberlakukan SSO.

<Steps>
  <Step title="Hubungkan akun GitHub Anda">
    Jika Anda belum pernah menggunakan claude.ai/code sebelumnya, kunjungan pertama Anda memandu Anda menghubungkan GitHub; lihat [Hubungkan GitHub](/docs/id/web-quickstart#connect-github). Jika tidak, gunakan salah satu dari [opsi autentikasi GitHub](/docs/id/claude-code-on-the-web#github-authentication-options).
  </Step>

  <Step title="Instal Claude GitHub App di repositori project">
    Instal [Claude GitHub App](https://github.com/apps/claude) dan berikan repositori yang akan digunakan project. Di repositori yang dimiliki organisasi GitHub, hanya pemilik organisasi yang dapat menyelesaikan instalasi; jika Anda bukan, GitHub mengirim permintaan instalasi ke pemilik dan project tidak dapat menggunakan repositori sampai mereka menyetujuinya.
  </Step>

  <Step title="Otorisasi SSO untuk organisasi yang memberlakukannya">
    Jika organisasi GitHub memberlakukan SAML SSO, hubungkan kembali GitHub dan otorisasi aplikasi Claude untuk organisasi itu. Sampai Anda melakukannya, repositori pribadi organisasi itu tidak muncul di dialog **New project** atau **Project settings > Environment**.
  </Step>
</Steps>

Ketika salah satu langkah ini tidak lengkap, dialog **New project** dan halaman project menyebutkan langkah yang hilang dan menautkan ke tempat Anda menyelesaikannya. Selesaikan langkah di sana, lalu klik **Check again** jika dialog menawarkannya. Jika repositori masih hilang dari daftar setelahnya, buka instalasi Claude GitHub App di GitHub, di [github.com/settings/installations](https://github.com/settings/installations) untuk akun pribadi, dan konfirmasi repositori tercantum di bawah **Repository access**. Untuk pesan kesalahan yang dilaporkan thread atau project ketika akses masih salah, lihat [Repository access errors](#repository-access-errors).

<h2 id="work-in-a-project">
  Bekerja di project
</h2>

Berikan Claude pekerjaan melalui percakapan project: tugas satu per satu atau beberapa sekaligus, ditambah pembaruan dan pemikiran longgar saat muncul. Claude merutekan setiap pesan, dan thread melakukan pekerjaan dan melaporkan kembali.

<h3 id="your-first-batch">
  Batch pertama Anda
</h3>

Sebelum Anda mengirim batch pekerjaan project baru, aturlah sehingga thread pertama kembali seperti yang Anda inginkan:

1. [Tulis project instructions](#write-project-instructions): brief yang dimulai setiap thread, seperti cabang mana yang ditargetkan, bagaimana thread memeriksa pekerjaannya, dan apa yang membutuhkan persetujuan Anda.
2. Kirim satu bagian kecil dari pekerjaan nyata, atau mulai salah satu thread yang disarankan Claude, dan buka thread ketika selesai untuk melihat bagaimana ia melaporkan kembali dan apa yang dilakukannya di cabangnya. Jika ia mengasumsikan sesuatu yang salah atau tidak dapat menjangkau apa yang dibutuhkan, [Threads menebak atau terhenti alih-alih bertanya](#threads-guessed-or-stalled-instead-of-asking) mencakup tempat untuk memperbaikinya.
3. Periksa **Thread model** dan **Thread effort** di **Project settings > General**. Project baru menjalankan setiap thread pada Opus dengan effort tinggi, yang menggunakan paket Anda paling cepat; [Pilih model dan biarkan Claude mengelola konteks](#choose-models-and-let-claude-manage-context) mencakup alternatifnya.
4. Minta Claude untuk [mengusulkan thread sebelum memulainya dan menjalankan beberapa sekaligus](#tune-how-claude-runs-a-project), dan turunkan batas tersebut setelah beberapa thread kembali seperti yang Anda inginkan.

<h3 id="send-work-and-read-results">
  Kirim pekerjaan dan baca hasil
</h3>

Claude memutuskan di mana setiap pesan yang Anda kirim dalam percakapan pergi:

* Pertanyaan cepat biasanya mendapat jawaban dalam percakapan.
* Pekerjaan baru masuk ke thread baru atau ke thread yang sudah bekerja di area itu, dan Claude memberi tahu Anda mana. Setiap thread baru muncul di bawah pesan Anda sebagai kartu: kotak dengan judul dan status thread, yang Anda klik untuk membuka thread.
* Beberapa tugas yang tidak terkait dalam satu pesan menjadi thread terpisah.

Jika Claude merutekan sesuatu berbeda dari yang Anda inginkan, katakan saja. [Sesuaikan cara Claude menjalankan project](#tune-how-claude-runs-a-project) mencantumkan hal-hal yang dapat Anda katakan, seperti menggunakan kembali thread yang ada untuk follow-up atau menjawab di tempat alih-alih memulai thread.

Hasil lengkap thread tetap ada di thread, dan Anda membuka kartunya dalam percakapan untuk membacanya. File yang dihasilkan thread juga ada di tab **Library** di **Overview**.

Kadang-kadang Claude mengusulkan thread alih-alih memulainya, dalam daftar **Suggested threads**. Klik panah pada saran untuk memulai thread itu. Ketika beberapa tercantum, tombol di bawah daftar memulai semuanya.

<h3 id="review-a-thread’s-pull-request">
  Tinjau pull request thread
</h3>

Ketika thread mengubah kode, inilah yang dilakukannya kecuali Anda memberi tahu sebaliknya:

* **Branch**: bekerja di cabang baru, dimulai dari cabang default repositori.
* **Pull request**: membuka satu ketika Anda minta, dan dapat membuka satu sendiri untuk perbaikan bug atau perubahan konkret lainnya.
* **Setelah dibuka**: mengawasi pull request dengan [auto-fix](/docs/id/claude-code-on-the-web#auto-fix-pull-requests) diaktifkan, terlepas dari apakah auto-fix aktif untuk sesi cloud lainnya. Ini mendorong perbaikan ketika CI gagal, mengatasi komentar review, dan membalas di thread ketika pemeriksaan lulus dan pull request siap untuk Anda.

Ketika thread telah mendorong cabang atau membuka pull request, kartunya dalam percakapan dapat menampilkan tombol untuk langkah berikutnya:

* **Resolve conflicts**, **Fix CI**, **Address comments**, dan **Merge it** mengirim instruksi itu ke thread sebagai pesan dari Anda, jadi Anda dapat meminta thread sendiri alih-alih menunggu untuk bereaksi terhadap pull request.
* **Review PR** membuka pull request di GitHub.
* **Create PR** muncul ketika thread idle telah mendorong cabang tetapi belum membuka pull request. Mengkliknya membuat pull request dari cabang itu secara langsung daripada mengirim instruksi thread untuk membuka satu.

Untuk mengubah kapan thread membuka pull request, misalnya hanya ketika Anda minta, atau cabang mana yang mereka mulai, katakan saja dalam tugas atau di [project instructions](#write-project-instructions).

<h3 id="see-what-needs-you-in-overview">
  Lihat apa yang membutuhkan Anda di Overview
</h3>

Pane **Overview** di samping percakapan melacak thread project. Sudah terbuka pertama kali Anda membuka project baru. Tombol **Overview** di header project menutup dan membukanya kembali, dan menunjukkan titik ketika thread menunggu Anda.

Di aplikasi desktop, Anda juga mendapatkan notifikasi desktop ketika Claude memposting dalam percakapan, thread mengalami kesalahan, atau thread membutuhkan input Anda, jadi Anda tidak harus membuat project tetap terbuka untuk mengetahuinya. Untuk juga mendapatkan satu setiap kali thread menyelesaikan giliran, atau untuk mematikannya untuk project, pilih **Notifications** di menu sidebar project. Notifikasi ini hanya desktop: di browser, periksa titik pada tombol **Overview**.

Tab **Threads** pane mengelompokkan thread berdasarkan keadaan:

| Grup                 | Apa yang ada di dalamnya                                                                                                                                                                                                                             |
| :------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Ready for review** | Thread yang pull request-nya terbuka dan menunggu review                                                                                                                                                                                             |
| **Waiting on you**   | Thread yang membutuhkan balasan atau persetujuan Anda, atau yang gagal                                                                                                                                                                               |
| **Working**          | Thread yang masih berjalan                                                                                                                                                                                                                           |
| **Landing**          | Thread yang pull request-nya disetujui atau antri untuk merge                                                                                                                                                                                        |
| **Idle**             | Thread yang selesai dan tidak menunggu apa pun                                                                                                                                                                                                       |
| **Resolved**         | Thread yang ditandai selesai: oleh Anda dari menu thread, oleh Claude setelah Anda mengambil langkah terakhir, seperti merge pull request-nya, atau secara otomatis setelah seminggu tanpa aktivitas. Anda dapat membuka kembali dari menu yang sama |

Tab lain pane adalah **Library** untuk file dan folder yang Anda tambahkan dan file yang dihasilkan thread, **Pull requests** setelah thread membuka apa pun, dan **Routines** untuk [routines](/docs/id/routines) yang Claude atur dari project ini.

<h3 id="open-a-thread-when-you-need-control">
  Buka thread ketika Anda membutuhkan kontrol
</h3>

Klik kartu thread dalam percakapan atau barisnya di **Overview** untuk membuka transkrip di pane Overview. Dari sana Anda dapat:

* Baca apa yang dilakukan Claude, langkah demi langkah.
* Kemudi tugas dengan menulis di kotak pesan thread sendiri. Pesan di sana langsung ke thread itu, sementara follow-up dalam percakapan project mencapainya hanya ketika Claude mencocokkan follow-up ke thread itu.
* Jawab prompt izin yang ditunggu thread.
* Interupsi thread dengan **Stop**, yang menggantikan tombol kirim saat thread bekerja, atau dengan menekan Esc.

<h3 id="choose-models-and-let-claude-manage-context">
  Pilih model dan biarkan Claude mengelola konteks
</h3>

Atur model dan effort di **Project settings > General**. Project baru menjalankan Opus di mana-mana, dengan [effort](/docs/id/model-config#adjust-effort-level) tinggi untuk thread dan effort rendah untuk percakapan:

* **Thread model** dan **Thread effort** berlaku untuk thread. Untuk menggunakan model berbeda untuk satu tugas, minta dalam tugas; untuk thread yang sudah berjalan, gunakan pemilih model thread itu.
* **Coordinator model** dan **Coordinator effort** berlaku untuk Claude dalam percakapan project.

Anda tidak mengelola jendela konteks dalam project. Thread compact secara otomatis, dan percakapan bekerja dari pesan terbaru, thread terbaru, dan project memory daripada riwayat lengkapnya, jadi terus berjalan selama project berjalan. Masukkan apa pun yang tidak boleh pernah dijatuhkan ke [project memory](#give-a-project-standing-context). Jika satu thread melampaui konteksnya, itu menunjukkan [Claude kehabisan konteks pada giliran ini](#context-limit).

<h3 id="tune-how-claude-runs-a-project">
  Sesuaikan cara Claude menjalankan project
</h3>

Beri tahu Claude dalam percakapan berapa banyak thread yang harus dijalankan sekaligus, kapan memposting pembaruan, dan kapan membuka pull request. Jika Claude mengoordinasikan dengan cara yang tidak Anda inginkan, katakan saja. Misalnya, Anda dapat mengatakan:

* "Usulkan thread dan tunggu persetujuan saya sebelum memulainya" atau "Mulai ini sekarang tanpa meminta saya untuk mengonfirmasi"
* "Jalankan paling banyak dua thread sekaligus" atau "Gunakan kembali thread yang ada untuk follow-up di area yang sama"
* "Posting pembaruan yang lebih pendek" atau "Hanya posting ketika sesuatu selesai atau terblokir"
* "Berikan saya pembaruan status di setiap thread"
* "Lakukan tugas ini dengan model yang lebih kecil"
* "Jangan buka pull request sampai saya telah melihat rencananya"
* "Beri tahu saya apa yang salah di repositori ini dan jangan perbaiki apa pun dulu", ketika Anda ingin melalui temuan sebelum salah satu menjadi thread
* "Jawab itu di sini alih-alih memulai thread", ketika Claude memulai thread untuk sesuatu yang Anda maksudkan sebagai pertanyaan cepat

Claude menyimpan preferensi seperti ini ke [project memory](#give-a-project-standing-context) sendiri dan mengikutinya di thread yang lebih baru. Ini adalah instruksi yang Claude patuhi, bukan pengaturan yang diberlakukan, jadi batas thread yang Anda berikan dengan cara ini bukan batas keras. Tambahkan satu ke project instructions ketika Anda ingin diucapkan dengan tepat dan diterapkan ke setiap thread dari awal.

<h3 id="unblock-a-thread-waiting-on-approval">
  Buka blokir thread yang menunggu persetujuan
</h3>

Thread berjalan dalam [auto mode](/docs/id/permission-modes#eliminate-prompts-with-auto-mode) ketika model thread mendukungnya, jadi sebagian besar panggilan alat berjalan tanpa meminta Anda. Ketika thread membutuhkan persetujuan Anda, prompt ada di dalam thread itu dan thread menunggu sampai Anda menjawabnya di sana. Memberi tahu Claude dalam percakapan project untuk melanjutkan tidak mencapainya.

Setiap persetujuan mencakup prompt itu, atau sisa thread itu jika Anda memilih opsi yang lebih luas. Untuk membiarkan setiap thread menjalankan perintah tertentu tanpa bertanya, atau untuk memblokir beberapa, tambahkan [permission rules](/docs/id/permissions) ke `.claude/settings.json` repositori. Thread menerapkannya hanya dalam project dengan satu repositori; lihat [Apa yang diambil thread dari repositori Anda](#what-threads-pick-up-from-your-repositories).

<h2 id="give-a-project-standing-context">
  Berikan project konteks berdiri
</h2>

Project memory, project instructions, dan repositori, file, dan environment project membawa konteks di seluruh thread. Anda menetapkan masing-masing sekali.

| Konteks                           | Apa yang dibawanya                                                                                                                                                                                                               | Cara Anda menetapkannya                                                                                                                                                                                       |
| :-------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Project memory                    | Catatan yang disimpan Claude tentang project, seperti persyaratan, keputusan, dan jebakan, disimpan sebagai file. Setiap thread cloud membaca file indeks `MEMORY.md` ketika dimulai dan membuka file lain ketika membutuhkannya | Minta Claude dalam percakapan project atau thread cloud apa pun untuk mengingat persyaratan, keputusan, atau jebakan, atau untuk melupakannya. Baca, edit, dan hapus file di **Project settings > Memory**    |
| Project instructions              | Teks yang dikirim ke setiap thread baru dan ke Claude dalam percakapan project, hingga 16.000 karakter. [Tulis project instructions](#write-project-instructions) mencakup apa yang harus dimasukkan                             | **Project settings > Memory > Project instructions**, atau minta Claude untuk mengubah instruksi                                                                                                              |
| Repositori, file, dan environment | Repositori yang dikloning setiap thread cloud, folder dan file yang dapat dibaca setiap thread di bawah `/mnt/project-files`, dan cloud environment yang dijalankan thread                                                       | Repositori dan environment di **Project settings > Environment**, atau minta Claude dalam percakapan untuk menambahkan repositori ke project. File dan folder dari **Add** di tab **Library** di **Overview** |

**Project settings > Memory** mencantumkan file-file ini di bawah **Auto memory**, karena Claude menulisnya sendiri saat bekerja di project. Mereka terpisah dari [auto memory](/docs/id/memory) yang disimpan Claude Code di mesin Anda, meskipun keduanya menggunakan indeks `MEMORY.md`. Project memory juga terpisah dari file `CLAUDE.md` di repositori project. Setiap thread cloud masih membaca file `CLAUDE.md` itu dari klonnya ketika dimulai, jadi masukkan instruksi tentang repositori di `CLAUDE.md`-nya dan catatan tentang project di project memory.

<h3 id="write-project-instructions">
  Tulis project instructions
</h3>

Project instructions adalah brief yang dimulai setiap thread baru. Klik ikon gear di header project untuk membuka **Project settings**, lalu buka **Memory > Project instructions**. Brief yang berguna mencakup:

* Apa project itu untuk
* Di mana pekerjaan terjadi: repositori mana, cabang mana yang dimulai, cara memberi nama pull request
* Bagaimana thread memeriksa pekerjaannya sendiri sebelum menyebutnya selesai
* Apa yang harus dilakukan ketika sesuatu yang dibutuhkan hilang
* Apa yang membutuhkan persetujuan Anda terlebih dahulu

Misalnya:

```text theme={null}
Project ini menjaga p95 latency untuk API pembayaran di bawah 200 ms: profiling, perbaikan query dan caching, dan upgrade dependensi yang menyertainya, di repositori payments-api.

- Branch dari main dan buka satu draft pull request per thread.
- Sebelum Anda menyebut pekerjaan selesai, jalankan `make test` dan `make lint` dan tempel baris ringkasan dalam pesan terakhir Anda.
- Jika Anda tidak dapat menjangkau sesuatu yang Anda butuhkan, seperti repositori, rahasia, API, atau connector, katakan dengan tepat apa yang hilang dalam pesan pertama Anda dan berhenti. Jangan ganti, mock, atau tebak.
- Jangan merge, force-push, atau ubah konfigurasi CI tanpa bertanya kepada saya di thread.
```

Aturan tentang satu repositori, seperti perintah buildnya, termasuk dalam `CLAUDE.md` repositori itu, yang dibaca setiap thread cloud ketika repositori adalah bagian dari project. Setelah pekerjaan sedang berlangsung, ketika Anda mengoreksi thread, juga beri tahu Claude untuk mengingat koreksi: itu masuk ke [project memory](#give-a-project-standing-context) dan thread yang lebih baru dimulai dengannya.

<h3 id="decide-which-repositories-to-add">
  Tentukan repositori mana yang akan ditambahkan
</h3>

Repositori yang Anda tambahkan ke project datang dengan semua yang ada di dalamnya, kode, `CLAUDE.md`, dan skills, di setiap thread. Repositori yang tidak Anda tambahkan masih dalam jangkauan: thread dapat menambahkan satu ke dirinya sendiri ketika tugasnya membutuhkannya. Sebagian besar project menggunakan keduanya:

* **Tambahkan ke project**, di dialog **New project**, di **Project settings > Environment**, atau dengan meminta Claude dalam percakapan untuk menambahkannya ke project. Setiap thread dari saat itu mengklonnya dan dimulai dengan `CLAUDE.md` dan skills-nya dimuat, terlepas dari apakah tugas menyentuhnya. Pergi dari satu repositori ke beberapa juga mengubah apa yang diambil thread dari `.claude/settings.json` setiap repositori; lihat [Apa yang diambil thread dari repositori Anda](#what-threads-pick-up-from-your-repositories).
* **Biarkan off dan biarkan thread menambahkannya ketika diperlukan.** Thread yang tugasnya membutuhkan repositori yang tidak dimiliki project dapat menambahkannya ke dirinya sendiri, dan catatan dalam thread mengatakan itu ditambahkan ke thread ini saja. Klon terjadi setengah jalan melalui tugas, jadi `CLAUDE.md` dan skills repositori itu tidak ada ketika thread dimulai. Thread berikutnya dimulai tanpanya lagi. Repositori yang ditambahkan dengan cara ini membutuhkan [prasyarat](#check-the-prerequisites) yang sama dengan repositori project: Claude GitHub App diinstal di atasnya dan akses push dari akun GitHub Anda.

Project tidak membutuhkan repositori sama sekali. Thread-nya masih dapat meneliti, menulis dokumen, dan menulis dan menjalankan kode di sandbox mereka sendiri, dan mereka memberikan file ke tab **Library**. Thread di sana juga dapat menambahkan repositori ke dirinya sendiri ketika tugas memanggilnya.

Setelah project memiliki repositori, Claude hanya dapat menambahkan repositori dari pemilik GitHub yang sudah digunakan project, apakah menambahkan satu ke project atau thread menambahkan satu ke dirinya sendiri. Untuk membawa repositori dari pemilik berbeda, tambahkan sendiri di **Project settings > Environment**.

Untuk project yang mencakup banyak repositori, seperti satu fitur dengan kode server, web, mobile, dan desktop, tambahkan satu atau dua repositori yang hampir setiap tugas sentuh dan beri nama yang lain di [project instructions](#write-project-instructions) sehingga Claude tahu di mana sisa kode berada. Thread kemudian dimulai kecil dan menarik repositori lain hanya untuk tugas yang membutuhkannya.

<h3 id="what-threads-pick-up-from-your-repositories">
  Apa yang diambil thread dari repositori Anda
</h3>

Setiap thread cloud mengkloning setiap repositori dalam project dan memuat `CLAUDE.md` dan skills dari semuanya. Permission rules, hooks, dan `env` hanya datang dari `.claude/settings.json` di direktori tempat thread dimulai: di dalam repositori ketika project memiliki satu, dan di atas klon ketika memiliki beberapa, di mana file repositori tidak dibaca untuk mereka.

| Di setiap repositori                                                             | Satu repositori                                                                                                                               | Beberapa repositori                                                               |
| :------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------- |
| `CLAUDE.md`                                                                      | Dimuat ketika thread dimulai                                                                                                                  | Dimuat dari setiap repositori ketika thread dimulai                               |
| Skills, agents, dan commands di bawah `.claude/`                                 | Dimuat                                                                                                                                        | Dimuat dari setiap repositori                                                     |
| Plugins yang diaktifkan di `.claude/settings.json`                               | Tidak dimuat. Tambahkan plugin di **Project settings > Plugins** sebagai gantinya                                                             | Tidak dimuat. Tambahkan plugin di **Project settings > Plugins** sebagai gantinya |
| Permission rules, hooks, dan `env` yang didefinisikan di `.claude/settings.json` | Berlaku untuk thread, kecuali kunci `env` yang [tidak dihormati sesi cloud apa pun](/docs/id/cloud-environments#what-carries-over-from-your-setup) | Tidak berlaku                                                                     |

Dalam project dengan beberapa repositori, setiap klon dilampirkan ke thread sebagai [direktori tambahan](/docs/id/memory#load-from-additional-directories) dengan pemuatan `CLAUDE.md` diaktifkan, itulah mengapa `CLAUDE.md` dan skills setiap repositori dimuat saat mulai meskipun thread dimulai di atasnya. Dalam project dengan beberapa repositori, masukkan aturan berdiri di project instructions dan berikan thread variabel lingkungan melalui [cloud environment](#choose-an-environment-for-threads).

<h3 id="choose-an-environment-for-threads">
  Pilih environment untuk thread
</h3>

Setiap thread baru dimulai dalam [cloud environment](/docs/id/cloud-environments) project. Environment menetapkan domain mana yang dapat dijangkau thread, variabel lingkungan mana yang mereka miliki, kredensial API mana yang ditambahkan ke permintaan mereka, dan apa yang diinstal skrip setup sebelum Claude dimulai. Thread menggunakan environment default yang dihosting Anthropic sampai Anda memilih satu di **Project settings > Environment**.

Jika thread perlu menjangkau API internal atau registri paket pribadi, atau membutuhkan token yang biasanya disimpan mesin Anda, ubah environment daripada project: lihat [Network access](/docs/id/cloud-environments#network-access), [Add API credentials](/docs/id/cloud-environments#add-api-credentials), dan [Setup scripts](/docs/id/cloud-environments#setup-scripts).

<h3 id="get-skills-plugins-connectors-and-tools-into-threads">
  Dapatkan skills, plugins, connectors, dan tools ke dalam threads
</h3>

Thread cloud tidak memiliki skills, server MCP, plugins, dan tools yang diinstal hanya di mesin Anda. Thread yang Claude jalankan di mesin Anda melalui [Remote Control](/docs/id/remote-control) menggunakan apa yang diinstal di sana. Untuk membuat masing-masing tersedia untuk thread cloud:

* Skills, subagents, dan commands: commit ke repositori yang Anda tambahkan ke project, misalnya skill di `.claude/skills/<skill-name>/SKILL.md`. Setiap thread cloud mengkloning setiap repositori dalam project dan memuat `.claude/skills/`, `.claude/agents/`, dan `.claude/commands/` dari masing-masing, jadi skill yang dicommit ke satu repositori tersedia di setiap thread cloud. Thread cloud juga memuat skills yang Anda aktifkan untuk akun claude.ai Anda.
* Plugins: tambahkan di **Project settings > Plugins**; mereka dimuat ke setiap thread cloud baru. Plugins yang dideklarasikan repositori di `.claude/settings.json`-nya [tidak dimuat dalam thread cloud](/docs/id/cloud-environments#what-carries-over-from-your-setup).
* Server MCP: thread cloud mendapatkan alat MCP mereka dari connectors di akun claude.ai Anda, yang merupakan server MCP yang Anda hubungkan sekali di [claude.ai/customize/connectors](https://claude.ai/customize/connectors) atau melalui link **Manage connectors** di **Project settings > Environment**. Setiap thread cloud dapat menggunakan semuanya tanpa setup per-project. Percakapan project itu sendiri tidak memiliki connectors, jadi kirim pekerjaan yang membutuhkan satu sebagai tugas untuk thread cloud. Dalam project dengan satu repositori, thread cloud juga memuat server MCP dari [`.mcp.json`](/docs/id/cloud-environments#what-carries-over-from-your-setup) repositori itu. [Bagaimana connectors mencapai Claude Code](/docs/id/mcp#how-connectors-reach-claude-code) mencantumkan aturan untuk sesi cloud dan pengaturan yang mematikan connectors.
* Command-line tools dan packages: instal di [setup script](/docs/id/cloud-environments#setup-scripts) environment.

Untuk melihat connectors mana yang dimiliki thread cloud yang sedang berjalan di claude.ai/code, buka thread dan pilih **Connectors** dari menu **+** di samping kotak pesannya. Mematikan connector di sana menghapusnya dari thread itu dan menyimpannya sebagai default akun Anda, jadi thread baru dan chat claude.ai dimulai tanpanya sampai Anda menyalakannya kembali. Thread cloud mengambil connector yang Anda tambahkan atau hubungkan kembali setelah pesan berikutnya yang Anda kirimkan.

<h2 id="project-settings-reference">
  Referensi pengaturan project
</h2>

Anda mengubah pengaturan project di claude.ai/code atau di aplikasi desktop, bukan di `settings.json`. Buka **Project settings** dari **Settings** di menu sidebar project atau dari ikon gear di header project.

Pengaturan disimpan saat Anda mengubahnya; bidang teks yang sedang Anda edit, seperti tujuan atau instruksi, menampilkan **Save changes** dan **Discard** sampai Anda meninggalkannya. Perubahan instruksi, repositori, plugins, dan environment di **Project settings** mencapai thread baru, bukan thread yang sudah berjalan.

| Pengaturan                   | Bagian      | Apa yang dikontrolnya                                                                                                                 |
| :--------------------------- | :---------- | :------------------------------------------------------------------------------------------------------------------------------------ |
| Nama, ikon, dan tujuan       | General     | Nama dan ikon project di sidebar, dan tujuan satu barisnya                                                                            |
| Coordinator model dan effort | General     | Model dan [effort level](/docs/id/model-config#adjust-effort-level) untuk Claude dalam percakapan project                                  |
| Thread model dan effort      | General     | Model dan effort level untuk thread                                                                                                   |
| Project instructions         | Memory      | [Aturan berdiri](#give-a-project-standing-context) yang diterima setiap thread baru                                                   |
| Project repositories         | Environment | Repositori yang dikloning thread baru                                                                                                 |
| Cloud environment            | Environment | [Cloud environment](#choose-an-environment-for-threads) yang dijalankan thread baru                                                   |
| Connectors                   | Environment | Link untuk mengelola connectors claude.ai yang didapatkan thread                                                                      |
| Plugins                      | Plugins     | Plugins yang dimuat ke setiap thread baru                                                                                             |
| Usage                        | Usage       | [Penggunaan token](#usage-and-cost) berdasarkan thread dan model                                                                      |
| Memory                       | Memory      | File [memory](#give-a-project-standing-context) project                                                                               |
| Restart Claude               | General     | Memulai ulang percakapan project ketika [Claude berhenti merespons di sana](#claude-hasnt-responded)                                  |
| Pause, Archive, Delete       | General     | Menghentikan, menyembunyikan, atau menghapus project; lihat [Pause, archive, atau delete project](#pause-archive-or-delete-a-project) |

<h3 id="pause-archive-or-delete-a-project">
  Pause, archive, atau delete project
</h3>

Ketiga kontrol ada di bagian bawah **Project settings > General**:

* **Pause**: menghentikan semuanya sekaligus. Setiap thread yang berjalan dan percakapan terinterupsi, thread baru tidak dimulai, routine tidak berjalan, dan project tidak menerima pesan sampai Anda melanjutkannya. Klik **Resume** di tempat yang sama atau di banner di atas kotak pesan project; thread yang dijeda berlanjut ketika Anda mengirim pesan setelahnya.
* **Archive**: menyembunyikan project dari sidebar dan mengarsipkan thread-nya, yang menghentikan thread apa pun yang sedang berjalan atau menonton pull request. Routine dalam project tidak berjalan saat diarsipkan. Untuk membawa project kembali, buka dari halaman Projects dan klik **Unarchive**. Thread-nya tetap diarsipkan sampai Anda membuka arsipnya secara individual dari daftar sesi.
* **Delete**: secara permanen menghapus project bersama dengan thread-nya, memorinya, dan file-nya, dan mematikan routine project. Ini tidak dapat dibatalkan. Cabang dan pull request yang didorong thread ke GitHub tidak terpengaruh.

<h2 id="usage-and-cost">
  Penggunaan dan biaya
</h2>

Penggunaan project dihitung terhadap [batas paket](/docs/id/errors#youve-hit-your-session-limit) yang sama dengan sesi Claude Code lainnya Anda, dan project tidak dapat menghabiskan melampaui batas itu sendiri.

Thread yang mencapai batas paket Anda menunggu dan berlanjut sendiri ketika batas direset, jadi pekerjaan yang Anda tinggalkan berjalan mulai menggunakan jendela penggunaan berikutnya tanpa pesan dari Anda. [Thread mencapai batas penggunaan](#usage-limit-reached) mencakup apa yang Anda lihat, cara menghentikannya, dan satu kasus yang tidak menunggu.

Pekerjaan melampaui batas paket Anda hanya jika Anda telah mengaktifkan [usage credits](/docs/id/costs#add-usage-credits-to-your-subscription) untuk akun Anda. Thread tidak dapat mengaktifkannya untuk Anda.

<h3 id="what-draws-on-your-plan">
  Apa yang menggunakan paket Anda
</h3>

Project menggunakan batas Anda lebih cepat daripada sesi tunggal, dan pada paket Pro khususnya Anda harus mengharapkan mencapai batas Anda lebih cepat pada hari Anda menjalankan satu. Ini adalah bagian dari project yang menggunakan paket Anda:

* **Running threads**: masing-masing adalah sesi penuh, dan beberapa dapat berjalan sekaligus. Tidak ada angka tetap; Claude memulai sebanyak yang diperlukan pekerjaan, dan batas yang Anda [minta](#tune-how-claude-runs-a-project) adalah preferensi daripada batas. Batas yang diberlakukan adalah 200 thread baru per hari di seluruh project Anda.
* **Percakapan**: Claude menggunakan token sendiri membaca apa yang dilaporkan thread dan memutuskan apa yang harus dilakukan selanjutnya.
* **Thread menonton pull request**: thread idle bangun dan menggunakan paket Anda lagi ketika CI gagal atau komentar review tiba di pull request-nya. Untuk menghentikan itu, minta di thread untuk berhenti menonton pull request.

Project tanpa thread yang berjalan, tidak ada pull request yang ditonton, dan tidak ada pesan baru tidak menggunakan paket Anda saat duduk idle, dan project yang diarsipkan juga tidak.

<h3 id="see-and-reduce-a-project’s-usage">
  Lihat dan kurangi penggunaan project
</h3>

Buka **Usage** di **Project settings** untuk melihat penggunaan token berdasarkan thread dan model, dan berapa banyak yang masuk ke percakapan project. Untuk menurunkannya:

* Follow-up yang dirutekan ke thread yang telah idle lebih lama dari [cache lifetime](/docs/id/prompt-caching#cache-lifetime), satu jam di Pro dan Max dalam batas paket Anda, membaca ulang seluruh percakapan thread sebelum melakukan apa pun. Untuk pekerjaan baru, meminta Claude untuk memulai thread segar dapat menggunakan lebih sedikit daripada menghidupkan kembali yang lama dan besar.
* Untuk pekerjaan yang tidak membutuhkan model terbesar, [pilih model yang lebih kecil atau effort level yang lebih rendah](#choose-models-and-let-claude-manage-context) untuk thread, percakapan, atau keduanya.
* Minta Claude dalam percakapan project untuk menjalankan lebih sedikit thread sekaligus, atau untuk menjawab pertanyaan kecil sendiri alih-alih memulai thread.

<h2 id="how-projects-relate-to-other-claude-code-features">
  Bagaimana project berhubungan dengan fitur Claude Code lainnya
</h2>

Beberapa fitur Claude Code memungkinkan lebih dari satu sesi bekerja pada waktu yang sama, jadi menjalankan pekerjaan secara paralel bukan dengan sendirinya apa yang dimaksudkan project. Dalam project, Claude memulai dan melacak sesi alih-alih Anda, dan masing-masing dimulai dari instruksi yang sama. Inilah cara setiap fitur tetangga terhubung ke project:

* **Claude Tag**: [Claude Tag](https://claude.com/docs/claude-tag/overview) adalah Claude di saluran Slack tim Anda, di paket Team dan Enterprise. Siapa pun di saluran dapat memberikan pekerjaan, semua orang di saluran melihat dan mengarahkannya, dan itu menggunakan koneksi yang disiapkan admin untuk saluran itu. Project adalah milik Anda sendiri: Anda satu-satunya yang mengirim pekerjaan atau melihat thread-nya, itu menggunakan akses GitHub dan connectors Anda sendiri, dan itu di Pro dan Max. [Bagaimana Claude Tag berbeda dari Cowork dan Claude Code](https://claude.com/docs/claude-tag/concepts/how-it-works#how-claude-tag-differs-from-cowork-and-claude-code) memiliki perbandingan berdampingan.
* **Cloud sessions**: setiap thread adalah [sesi cloud](/docs/id/claude-code-on-the-web) kecuali Anda meminta Claude untuk menjalankannya di mesin Anda. Bagaimanapun, Claude memulai dan melacaknya alih-alih Anda. Sesi cloud yang Anda mulai sendiri dapat menjadi project atau memberi makan satu melalui [**Continue as a project** atau **Move to project**](#start-from-an-existing-cloud-session).
* **Routines**: ketika Anda meminta pekerjaan terjadwal dalam project, Claude membuat [routine](/docs/id/routines) yang berjalan sebagai thread dalam project itu dan muncul di tab **Routines**-nya. Routine yang Anda buat di luar project terus bekerja sendiri.
* **Local sessions dan agent view**: sesi yang Anda mulai sendiri di terminal, IDE, atau environment lokal aplikasi desktop tidak dapat ditambahkan ke project. Project menjangkau mesin Anda hanya dengan menjalankan thread di sana melalui [Remote Control](/docs/id/remote-control). [Agent view](/docs/id/agent-view) adalah layar untuk melacak beberapa sesi lokal yang Anda mulai sendiri; itu tidak memiliki koordinator.
* **Worktrees**: [worktree](/docs/id/worktrees) memberikan setiap sesi lokal salinan kerjanya sendiri dari repositori sehingga sesi paralel di mesin Anda tidak menimpa satu sama lain. Thread cloud tidak membutuhkannya: setiap thread mengkloning repositorinya ke sandbox cloud sendiri dan bekerja di cabang sendiri.
* **Agent teams**: [agent team](/docs/id/agent-teams) adalah satu sesi yang memulai sesi rekan untuk satu tugas, di mesin Anda atau di dalam sesi cloud, dan berakhir dengan tugas itu.
* **Projects di claude.ai chat dan Cowork**: [pengalaman Projects sebelumnya](https://support.claude.com/en/articles/9517075-what-are-projects), yang mengelompokkan percakapan dan file referensi tanpa thread atau koordinator. Project tersebut terus bekerja seperti yang mereka lakukan hari ini sampai pengalaman yang dirancang ulang mencapai mereka.

[Jalankan agen secara paralel](/docs/id/agents) membandingkan opsi ini berdampingan.

<h2 id="limitations">
  Keterbatasan
</h2>

* Project tersedia di claude.ai/code, di aplikasi desktop, dan di aplikasi mobile Claude, bukan di CLI terminal atau melalui Amazon Bedrock, Google Cloud's Agent Platform, atau Microsoft Foundry. Perintah [`claude project`](/docs/id/cli-reference) CLI, yang mengelola keadaan Claude Code lokal untuk direktori, tidak terkait.
* Thread project adalah [sesi cloud](/docs/id/claude-code-on-the-web), atau sesi di mesin Anda sendiri melalui [Remote Control](/docs/id/remote-control), dengan Anthropic sebagai penyedia model di kedua kasus. [Security](/docs/id/security) dan [Data usage](/docs/id/data-usage) mencakup bagaimana sesi cloud diisolasi dan apa yang disimpan, dan [Connection and security](/docs/id/remote-control#connection-and-security) mencakup bagaimana thread di mesin Anda terhubung dan apa yang disimpan.
* Anda tidak dapat menambahkan sesi yang Anda mulai sendiri di mesin Anda ke project. Untuk membiarkan project menjalankan thread di mesin Anda, hubungkan folder tempat project harus bekerja melalui [Remote Control](/docs/id/remote-control#requirements): aktifkan Remote Control di bawah **Settings > Claude Code** di aplikasi desktop Claude, atau jalankan `claude remote-control` di folder dan biarkan berjalan. Mesin tersebut memerlukan Claude Code v2.1.280 atau lebih baru. Project juga tidak dapat menjalankan thread di mesin Anda saat **Require trusted devices** aktif di pengaturan claude.ai Anda.
* Sandbox thread cloud dijeda antara giliran dan dilanjutkan ketika thread berlanjut. Jika sandbox tidak dapat dilanjutkan, thread berlanjut dari klon segar, jadi perubahan yang tidak dicommit dapat hilang. Pada tugas panjang, minta Claude untuk commit dan push pekerjaan yang sedang berlangsung.
* Project milik satu pengguna. Anda tidak dapat berbagi project atau thread-nya dengan pengguna lain, dan transkrip thread tidak memiliki opsi berbagi yang dimiliki sesi cloud lain. Tidak ada kontrol tingkat organisasi untuk project selama beta.
* Thread milik satu project yang memulainya. Anda tidak dapat memindahkan atau menyalin thread ke project lain, atau memindahkannya keluar untuk berdiri sendiri. [**Move to project**](#start-from-an-existing-cloud-session) hanya berjalan dengan cara lain: itu membawa pekerjaan sesi cloud ke project.

<h2 id="troubleshooting">
  Troubleshooting
</h2>

Untuk prompt setup GitHub di dialog **New project**, lihat [Atur akses GitHub](#set-up-github-access).

<h3 id="a-thread-looks-stuck">
  Thread terlihat terjebak
</h3>

Claude tidak memposting setiap langkah yang diambil thread, jadi thread yang menunjukkan sedang berjalan tanpa pesan baru dalam percakapan project biasanya masih bekerja. Thread cloud baru juga menyediakan [cloud environment](/docs/id/cloud-environments)-nya sebelum Claude dimulai, jadi pembaruan pertamanya membutuhkan waktu. Buka thread untuk membaca transkrip-nya. Jika thread menunggu prompt izin, jawab di sana.

<h3 id="threads-guessed-or-stalled-instead-of-asking">
  Threads menebak atau terhenti alih-alih bertanya
</h3>

Ketika beberapa thread kembali telah mengasumsikan sesuatu yang salah, bekerja di sekitar akses yang hilang, atau berhenti dengan "blocked", penyebabnya biasanya celah yang sama dalam setup project daripada masalah dengan setiap tugas. Urutkan thread mana yang solid sebelum Anda memperbaiki apa pun:

1. Tanya Claude dalam percakapan: "Untuk setiap thread terbuka, daftarkan apa yang Anda minta untuk dilakukan, apa yang diasumsikan atau tidak dapat dijangkau, dan apa yang ditunggu." Claude membaca setiap thread dan menjawab dalam percakapan.
2. Untuk thread yang dimulai dari asumsi yang salah, buka thread dari **Overview** dan tandai resolved dari menunya, atau beri tahu apa yang harus dilakukan sebagai gantinya di kotak pesannya. Cabang dan pull request apa pun tetap di GitHub sampai Anda menghapusnya.
3. Perbaiki celah sekali, di [project instructions](#give-a-project-standing-context) atau [environment](#choose-an-environment-for-threads), lalu kirim satu thread sebelum mengirim sisa pekerjaan lagi sebagai thread baru.

<h3 id="claude-hasnt-responded">
  Claude belum merespons
</h3>

Percakapan project menunjukkan banner "Claude hasn't responded" ketika Claude sedang berjalan tetapi balasannya tidak mencapai project. Klik **Restart Claude** di banner, atau buka **Project settings > General** dan klik **Restart** di baris **Restart Claude**. Claude terhubung kembali ke percakapan; balasan apa pun yang sedang ditulis hilang, dan thread tidak terpengaruh.

<h3 id="repository-access-errors">
  Repository access errors
</h3>

Tiga pesan berarti thread atau project tidak dapat menjangkau salah satu repositorinya. Thread cloud project membutuhkan [prasyarat GitHub](#check-the-prerequisites) bahkan ketika sesi cloud lain Anda mengkloning repositori yang sama tanpa masalah.

* **"Couldn't start the session — Claude doesn't have GitHub access to this project's repository"**, dilaporkan sebelum thread dimulai, ketika Claude GitHub App tidak diinstal di repositori itu, ditangguhkan, atau tidak tertaut ke akun GitHub yang Anda hubungkan.
* **"Unable to access your repository"**, dilaporkan oleh thread ketika klonnya gagal: GitHub menolak klon, repositori tidak ditemukan dengan nama yang dimiliki project, atau cabang yang diminta thread untuk dimulai tidak ada.
* **"Claude can't access"** repositori, ditampilkan ketika Anda menyimpan repositori di dialog **New project** atau **Project settings**. Pesan berlanjut dengan link instalasi dan link reconnect. Gunakan link instalasi jika Claude GitHub App tidak ada di repositori itu, dan link reconnect jika ada, karena GitHub App dapat diinstal di GitHub tanpa tertaut ke akun yang Anda hubungkan ke Claude. Jika pesan mengatakan GitHub App ditangguhkan atau tidak menyertakan repositori ini, ikuti linknya ke GitHub untuk memperbaikinya.

Untuk memperbaiki salah satu dari mereka, klik tombol yang ditawarkan pesan, seperti **Install GitHub App** atau **Select repositories on GitHub**, lalu **Check again**. Ketika blok ada di sisi organisasi GitHub, seperti pemilik yang belum menyetujui aplikasi atau daftar IP allow yang mengecualikan Claude, pesan menunjukkan link **See how to fix** sebagai gantinya. Jika tidak ada tombol, ikuti [Atur akses GitHub](#set-up-github-access), lalu kirim pesan lain untuk mencoba lagi.

<h3 id="usage-limit-reached">
  Thread mencapai batas penggunaan
</h3>

Ketika thread atau percakapan project mencapai batas lima jam atau mingguan paket Anda, itu terus mencoba sendiri dan berlanjut ketika batas direset. Saat menunggu, thread menunjukkan **Service is busy** dengan "Claude is still retrying and will continue automatically." Anda tidak perlu melakukan apa pun agar pekerjaan berlanjut. Jika Anda lebih suka tidak menggunakan jendela penggunaan berikutnya, klik **Stop** di thread, atau [pause project](#pause-archive-or-delete-a-project) untuk menahan setiap thread. Thread yang dimulai routine tidak menunggu: giliran-nya berhenti dengan kesalahan batas, dan Anda mengirim pesan setelah batas direset.

[Usage limit errors](/docs/id/errors#youve-hit-your-session-limit) menjelaskan batas dan kapan mereka direset.

<h3 id="additional-usage-credits-are-required">
  Kredit penggunaan tambahan diperlukan
</h3>

Thread atau percakapan project membuat permintaan yang hanya dicakup paket Anda dengan kredit penggunaan, seperti satu ke model atau ukuran konteks yang tidak disertakan paket Anda, dan kredit penggunaan tidak diaktifkan untuk akun Anda. [Tambahkan kredit penggunaan ke langganan Anda](/docs/id/costs#add-usage-credits-to-your-subscription) mencakup siapa yang dapat mengaktifkannya atau membelinya di setiap paket. Setelah kredit tersedia, kirim pesan lain untuk mencoba lagi.

<h3 id="context-limit">
  Pesan lainnya
</h3>

Pesan ini menyebutkan penyebab mereka sendiri. Tabel memberikan langkah berikutnya untuk masing-masing.

| Pesan                                                                                            | Apa yang harus dilakukan                                                                                                                                                                                                    |
| :----------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| "Unable to connect to repository" dengan "Claude couldn't reach GitHub to fetch your repository" | Tunggu sebentar, lalu kirim pesan lain untuk mencoba lagi                                                                                                                                                                   |
| "Unable to connect to repository" dengan "Claude couldn't access your repository or environment" | Akun GitHub Anda membutuhkan akses push ke repositori, dan environment harus masih ada. Periksa keduanya di **Project settings > Environment**, lalu coba lagi                                                              |
| "Couldn't show the setup proposal"                                                               | Aplikasi yang Anda buka lebih lama dari **Setup recommendations** yang dikirim Claude. Segarkan halaman atau mulai ulang aplikasi desktop, atau minta Claude untuk mengusulkan setup lagi                                   |
| "The project's environment was removed"                                                          | Pilih environment berbeda di **Project settings > Environment**; perubahan berlaku untuk thread baru                                                                                                                        |
| "Setup script failed"                                                                            | Klik **Edit setup script** pada kesalahan, perbaiki skrip di environment, lalu kirim pesan lain. [Setup script failed](/docs/id/web-quickstart#setup-script-failed) mencantumkan penyebab umum                                   |
| "Claude ran out of context on this turn"                                                         | Thread mengisi jendela konteksnya. Jika pesan mengatakan thread berlanjut dalam sesi segar, itu berlanjut sendiri; jika tidak, minta Claude dalam percakapan project untuk memulai thread baru untuk pekerjaan yang tersisa |
| "Reached the turn limit"                                                                         | Thread mencapai batas pada langkah agentic yang [`CLAUDE_CODE_MAX_TURNS`](/docs/id/env-vars) tetapkan. Kirim pesan lain untuk melanjutkan, atau naikkan atau hapus variabel itu di mana pun variabel itu ditetapkan              |

<h2 id="related-resources">
  Sumber daya terkait
</h2>

* [Gunakan Claude Code di cloud](/docs/id/claude-code-on-the-web): bagaimana sesi cloud di balik setiap thread bekerja, termasuk opsi akses GitHub dan auto-fix pada pull request
* [Konfigurasi cloud environments](/docs/id/cloud-environments): ubah apa yang dapat dijangkau thread di jaringan, berikan mereka variabel lingkungan dan kredensial API, dan instal alat dengan skrip setup
* [Otomatisasi pekerjaan dengan routines](/docs/id/routines): jadwal, pemicu, dan manajemen untuk routine, termasuk yang dibuat Claude dari project
* [Kelola beberapa agen dengan agent view](/docs/id/agent-view): jalankan dan lacak beberapa sesi di mesin Anda sendiri ketika pekerjaan membutuhkan alat atau layanan yang hanya dapat dijangkau mesin Anda
* [Projects redesigned: from folder to conversation](https://claude.com/blog/projects-redesigned): pengumuman peluncuran, dengan pemikiran di balik membuat project menjadi percakapan dengan Claude
