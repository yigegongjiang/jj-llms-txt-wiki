> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Lingkungan yang di-host sendiri

> Jalankan sesi cloud Claude Code pada infrastruktur yang Anda kontrol: siapkan lingkungan yang di-host sendiri, deploy runner, dan arahkan sesi ke komputasi Anda sendiri.

<Note>
  Lingkungan yang di-host sendiri berada dalam beta publik pada paket Team dan Enterprise dan dinonaktifkan secara default. Lihat [Ketersediaan dan batasan](#availability-and-limitations) untuk jalur pengaktifan dan apa yang dikecualikan.
</Note>

Lingkungan yang di-host sendiri menjalankan sesi cloud Claude Code pada infrastruktur yang dioperasikan organisasi Anda. [Sesi cloud](/docs/id/claude-code-on-the-web) adalah sesi apa pun yang berjalan di tempat selain mesin pengembang: pengembang memulainya dari claude.ai, aplikasi mobile dan desktop, terminal dengan [`claude --cloud`](/docs/id/claude-code-on-the-web#from-terminal-to-cloud), dan [rutinitas terjadwal](/docs/id/routines), dan secara default mereka dijalankan pada infrastruktur Anthropic. Dalam lingkungan yang di-host sendiri, sesi yang sama dijalankan di dalam jaringan Anda, dan pengalaman pengembang sebaliknya sama terlepas dari perbedaan dalam [Ketersediaan dan batasan](#availability-and-limitations) dan [masalah yang diketahui](/docs/id/self-hosted-environments-deploy#known-issues-and-limitations) halaman deploy.

Jika tim Anda tidak menggunakan sesi cloud, tidak ada yang perlu dikonfigurasi di sini: sesi di terminal atau IDE selalu berjalan di mesin pengembang sendiri. Jika Anda ingin menjalankan Claude Code pada mesin yang selalu aktif milik Anda dan menggerakkannya dari perangkat lain, gunakan [Remote Control](/docs/id/remote-control), yang juga tersedia pada paket Pro dan Max. Ketika Anda siap untuk menyiapkan, langsung ke [quickstart](/docs/id/self-hosted-environments-quickstart); untuk meninjau postur keamanan terlebih dahulu, mulai dengan [Deploy ke produksi](/docs/id/self-hosted-environments-deploy). Sisa halaman ini menjelaskan cara kerja self-hosting dan kapan memilihnya.

<h2 id="how-self-hosted-environments-work">
  Cara kerja lingkungan yang di-host sendiri
</h2>

Self-hosting memiliki tiga bagian:

* **Environment**: tujuan bernama yang dapat dikirim sesi cloud. Organisasi Anda membuat environment di pengaturan admin claude.ai, dan masing-masing mengelompokkan serangkaian runner.
* **Runner**: program yang berjalan pada host di dalam jaringan Anda. Runner menjalankan sesi; idenya sama dengan runner CI yang di-host sendiri.
* **Session**: satu tugas Claude Code yang dimulai pengembang.

Ketika pengembang memulai sesi cloud, UI awal sesi menampilkan pemilih environment yang mencantumkan environment yang di-host Anthropic bersama dengan yang telah dibuat organisasi Anda. Jika mereka memilih milik Anda, control plane Anthropic menempatkan sesi pada antrian environment Anda, di mana runner mengklaimnya, mengkloning repositori yang dipilih pengembang, dan memulai proses Claude Code pada host Anda untuk menjalankannya. Runner mengautentikasi ke host git Anda dengan kredensial yang Anda konfigurasi; [Konfigurasi git](/docs/id/self-hosted-environments-deploy#configure-git) mencakup opsinya. Sesi menjangkau layanan internal Anda dari dalam jaringan Anda, dan host git Anda dengan cara yang sama ketika itu internal; lalu lintas ke Anthropic, polling antrian, aliran peristiwa sesi, dan inferensi model, adalah HTTPS keluar ke `api.anthropic.com`, dengan daftar singkat host lebih lanjut yang dapat dijangkau sesi dalam [Persyaratan jaringan](/docs/id/self-hosted-environments-deploy#network-requirements). Anthropic tidak pernah terhubung ke jaringan Anda.

<div style={{maxWidth: "640px", margin: "0 auto"}}>
  <Frame>
    <img src="https://mintcdn.com/claude-code/Y0sJ2uDoOVbOVZrQ/images/self-hosted-network-paths.svg?fit=max&auto=format&n=Y0sJ2uDoOVbOVZrQ&q=85&s=8056103fc1c5564c7f0ef219d260b99d" className="dark:hidden" alt="Diagram arsitektur lingkungan yang di-host sendiri: batas jaringan Anda berisi runner, dua proses sesi Claude Code di dalamnya, dan host git Anda, dengan api.anthropic.com di luar yang menyimpan antrian, aliran sesi, dan inferensi. Runner polling antrian dan menjangkau host git, setiap proses sesi membuka aliran, inferensi, dan koneksi git miliknya sendiri, dan setiap koneksi keluar dari jaringan Anda, tanpa yang masuk." width="680" height="320" data-path="images/self-hosted-network-paths.svg" />

    <img src="https://mintcdn.com/claude-code/Y0sJ2uDoOVbOVZrQ/images/self-hosted-network-paths-dark.svg?fit=max&auto=format&n=Y0sJ2uDoOVbOVZrQ&q=85&s=fec6aef3b0740d80eaf6d6a7000a2233" className="hidden dark:block" alt="Diagram arsitektur lingkungan yang di-host sendiri: batas jaringan Anda berisi runner, dua proses sesi Claude Code di dalamnya, dan host git Anda, dengan api.anthropic.com di luar yang menyimpan antrian, aliran sesi, dan inferensi. Runner polling antrian dan menjangkau host git, setiap proses sesi membuka aliran, inferensi, dan koneksi git miliknya sendiri, dan setiap koneksi keluar dari jaringan Anda, tanpa yang masuk." width="680" height="320" data-path="images/self-hosted-network-paths-dark.svg" />
  </Frame>
</div>

Dua kotak Claude Code dalam diagram adalah proses sesi: satu runner menjalankan dua sesi sekaligus, hingga kapasitas yang dikonfigurasi. Runner melayani satu [owner](#key-concepts) pada satu waktu dan terkunci ke owner itu ketika mengklaim sesi pertamanya, jadi kode yang diperiksa tidak pernah bercampur antara owner; [Runner lifecycle](#runner-lifecycle) mencakup aturannya.

Anda dapat memulai runner sendiri dan membiarkannya berjalan, atau menjalankan [orchestrator autoscaling](/docs/id/self-hosted-environments-configuration#on-demand-runners), proses kedua yang Anda host, yang memulai runner saat sesi antri; setiap runner keluar dengan sendirinya ketika pekerjaannya selesai. Bagaimanapun, Anda menyiapkan environment sekali, dan itu muncul di pemilih pada setiap permukaan yang didukung.

<h2 id="availability-and-limitations">
  Ketersediaan dan batasan
</h2>

Periksa ini sebelum merencanakan peluncuran:

* **Plans**: beta publik untuk organisasi Team dan Enterprise. Lingkungan yang di-host sendiri dinonaktifkan secara default; [Owner](/docs/id/cloud-environments#organization-shared-environments) mengaktifkan **Allow self-hosted environments** pada [halaman admin **Cloud environments**](https://claude.ai/admin-settings/cloud-environments), yang memerlukan [cloud sessions](/docs/id/claude-code-on-the-web) diaktifkan untuk organisasi.
* **Zero Data Retention**: tidak tersedia untuk organisasi dengan [Zero Data Retention](/docs/id/zero-data-retention) diaktifkan.
* **Model inference**: sesi menggunakan Anthropic API, dan inferensi tidak dapat dirutekan melalui [Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry](/docs/id/third-party-integrations), atau [LLM gateway](/docs/id/llm-gateway).
* **Surfaces**: sesi dimulai dari [claude.ai/code](https://claude.ai/code), aplikasi mobile dan desktop, [rutinitas terjadwal](/docs/id/routines), dan terminal, dengan [`claude --cloud`](/docs/id/claude-code-on-the-web#from-terminal-to-cloud) atau [dispatch `--environment`](/docs/id/self-hosted-environments-testing#run-the-test-loop), dapat berjalan di lingkungan yang di-host sendiri. Sesi [Claude Tag](https://claude.com/docs/claude-tag/overview) dapat berjalan di dalamnya juga, tetapi Claude tidak dapat menggunakan [Access bundles](https://claude.com/docs/claude-tag/concepts/glossary#access-bundle) dalam sesi tersebut belum. Sesi [Claude Security](/docs/id/claude-security) dan [Code Review](/docs/id/code-review) belum dirutekan ke mereka. Dukungan untuk dua permukaan itu mengikuti secara terpisah.
* **Repositories**: sesi memeriksa repositori dari GitHub; lihat [opsi autentikasi GitHub](/docs/id/claude-code-on-the-web#github-authentication-options).
* **Billing**: sesi dalam lingkungan yang di-host sendiri menggunakan penggunaan Claude Code organisasi Anda dengan cara yang sama seperti sesi dalam lingkungan yang di-host Anthropic.

<h2 id="why-self-host">
  Mengapa self-host
</h2>

Sebagian besar tim dilayani dengan lebih baik oleh lingkungan yang di-host Anthropic, yang tidak memerlukan infrastruktur untuk dijalankan atau dipertahankan. Self-hosting adalah untuk tim yang persyaratan jaringan, tooling, atau kepatuhan mereka memerlukan menjaga eksekusi sesi pada infrastruktur yang mereka kontrol. Jika itu Anda, rencanakan kepemilikan operasional yang dibawanya: Anda membangun dan mempertahankan citra runner, mengoperasikan fleet, dan mengontrol jaringannya.

Sebagai gantinya, self-hosting memberi Anda akses jaringan, tooling kustom, dan kontrol kepatuhan:

* **Network access**: sesi berjalan di dalam jaringan Anda dan dapat menjangkau layanan internal, database, dan registri tanpa mengeksposnya ke internet publik
* **Custom tooling**: pra-instal compiler, SDK, dan CLI internal dalam citra runner Anda sehingga setiap sesi dimulai siap untuk membangun
* **Compliance**: checkout repositori dan artefak build tetap pada infrastruktur yang Anda kontrol. Konten sesi masih pergi ke `api.anthropic.com` untuk inferensi model.

<h2 id="environments-runners-and-sessions">
  Environment, runner, dan sesi
</h2>

Environment dikelola pada halaman **Cloud environments** di pengaturan admin claude.ai; runner adalah proses yang Anda mulai dan kelola pada infrastruktur Anda sendiri.

<h3 id="key-concepts">
  Konsep kunci
</h3>

Istilah-istilah ini muncul di seluruh halaman self-hosted:

| Istilah            | Apa itu                                                                                                                                                                                                         |
| :----------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Environment        | Grup bernama runner Anda, dibuat di pengaturan claude.ai. Sesi dirutekan ke environment, bukan ke runner individual.                                                                                            |
| Environment secret | Kredensial bersama tunggal yang digunakan runner untuk mengautentikasi dan mendaftar dengan environment. Ditampilkan sekali saat pembuatan environment, berlabel **environment key** di UI admin.               |
| Runner             | Proses jangka panjang yang Anda deploy. Runner mendaftar dengan environment, menerima token runner, dan polling untuk sesi.                                                                                     |
| Session            | Satu tugas Claude Code, dimulai dari claude.ai, aplikasi mobile, atau permukaan Anthropic lain seperti rutinitas terjadwal atau agen. Setiap sesi berjalan sebagai proses Claude Code anak yang dimulai runner. |

Di bidang API, klaim token, dan nama metrik, environment muncul sebagai `pool`, dan ID environment adalah `pool_id`. [Referensi](/docs/id/self-hosted-environments-reference) memetakan dua ejaan, termasuk nama flag `pool` yang sudah usang.

Runner melayani satu owner pada satu waktu. Sesi pertama yang diambil runner mengunci runner ke owner sesi itu, dan runner kemudian menjalankan sesi hanya untuk owner itu, hingga kapasitas yang dikonfigurasi. Siapa owner tergantung pada bagaimana sesi dimulai:

* **Sesi yang dimulai pengguna**: owner adalah akun pengguna itu.
* **Sesi saluran Claude Tag**: Claude menjalankannya tanpa akun pengguna yang terlampir, jadi owner adalah [agen Claude Tag](https://claude.com/docs/claude-tag/concepts/glossary#agent-identity) yang memulai sesi. Setiap sesi saluran yang dimulai agen itu memiliki owner yang sama, siapa pun yang mengirim pesan Slack, jadi runner yang terkunci ke itu melayani sesi yang dimulai orang berbeda ketika Anda menjalankannya pada `--capacity` di atas satu atau dengan `--drain-grace-sec` positif. Runner yang terkunci ke pengguna tidak pernah mengambil ini, dan runner yang terkunci ke agen Claude Tag tidak pernah mengambil sesi pengguna.

Ukuran fleet minimum adalah oleh karena itu jumlah owner yang Anda harapkan aktif sekaligus, menghitung pengguna dan agen Claude Tag.

<h3 id="session-lifecycle">
  Siklus hidup sesi
</h3>

Ketika pengembang memulai sesi dan memilih environment Anda, control plane Anthropic menempatkan sesi pada antrian environment. Dari sana:

1. Runner dengan kapasitas gratis mengklaim sesi dan memegang sewa padanya.
2. Runner mengkloning repositori ke direktori kerjanya dan memulai proses Claude Code anak.
3. Anak streaming peristiwa kembali melalui HTTPS sementara runner terus polling; setiap poll menyegarkan sewa dan berfungsi ganda sebagai detak jantung.
4. Jika runner berhenti polling selama sekitar 60 detik, server memasukkan kembali sesi untuk runner lain.

Runner memberikan setiap permintaan poll 10 detik. Ketika permintaan habis waktu, hilang, atau mendapat respons yang tidak dapat diurai runner, runner terus melayani sesi aktifnya dan mencoba lagi setelah satu atau dua detik daripada menunggu poll terjadwal berikutnya. Misalnya, proxy intersepsi yang menjawab poll dengan halaman miliknya sendiri menghasilkan respons yang tidak dapat diurai runner. Setiap kali permintaan lain gagal dengan salah satu cara itu, runner menggandakan celah sebelum percobaan ulang berikutnya, hingga 20 detik, dan mempersingkat celah kapan pun sewa hampir kedaluwarsa.

<h3 id="runner-lifecycle">
  Siklus hidup runner
</h3>

Sesi pertama yang diambil runner mengunci runner ke owner sesi itu, dan runner menjalankan hingga `--capacity` sesi bersamaan untuk owner itu. Sementara runner memiliki sesi aktif dan belum menerima sinyal shutdown atau mencapai waktu pensiun, runner terus mengklaim pekerjaan antri owner yang terkunci. Apa yang terjadi setelah mereka selesai tergantung pada [`--drain-grace-sec`](/docs/id/self-hosted-environments-reference#runner-cli-flags):

* **Pada default `0`**: runner keluar segera setelah sesi aktifnya selesai, tanpa polling untuk lebih banyak, jadi orchestrator yang Anda deploy di bawahnya, seperti Kubernetes, dapat memulainya ulang dengan disk segar, siap melayani owner apa pun.
* **Pada nilai positif**: runner terus polling antrian owner yang terkunci selama banyak detik itu sebelum keluar.

Siklus hidup ini mengisolasi kode yang diperiksa setiap owner tanpa memerlukan runner untuk menghapus status disk antara owner.

Bagaimana infrastruktur Anda menghentikan runner menentukan apakah Anda memerlukan `--retire-at`. Pembunuhan yang mengirimkan `SIGTERM` tidak memerlukan flag: runner mengalirkan seperti [Shutdown timing](/docs/id/self-hosted-environments-deploy#shutdown-timing) menjelaskan, atau terus melayani sesi yang sudah dipegang ketika Anda menetapkan [`--defer-shutdown-max-min`](/docs/id/self-hosted-environments-deploy#defer-the-drain-past-the-first-signal). Jika infrastruktur Anda malah menghancurkan host pada waktu dinding jam yang diketahui tanpa sinyal, atau dengan periode grace terlalu pendek untuk mengalirkan, seperti batas seumur hidup sandbox atau penarikan spot-instance, teruskan `--retire-at <epoch-seconds>` diatur ke beberapa menit sebelum waktu itu. Pada waktu pensiun:

1. Runner berhenti mengambil pekerjaan baru.
2. Runner melepaskan setiap sesi aktif melalui jalur pelepasan yang sama yang digunakan flag [`--release-idle-session-min`](/docs/id/self-hosted-environments-reference#runner-cli-flags), jadi sesi dilanjutkan pada runner segar ketika pengguna mengirim pesan berikutnya. Kapan runner melepaskan setiap sesi tergantung pada statusnya:
   * Runner melepaskan sesi yang sedang giliran segera setelah giliran itu selesai.
   * Ketika giliran selesai dan meninggalkan tugas latar belakang berjalan, runner menunggu hingga 60 detik untuk mereka, kemudian melepaskan sesi bahkan jika mereka masih berjalan. Jika tugas telah selesai tetapi giliran tindak lanjut yang membaca hasilnya belum berjalan, runner menyimpan sesi sampai giliran itu selesai, dan menunggu tidak lebih lama dari [`SELF_HOSTED_RUNNER_BG_RESULT_GRACE_MS`](/docs/id/self-hosted-environments-reference#environment-variable-only-settings) untuk giliran itu dimulai.
3. Runner keluar 0 setelah semua sesinya dilepaskan.

Giliran yang bertahan lebih lama dari pembunuhan masih hilang; [Shutdown timing](/docs/id/self-hosted-environments-deploy#shutdown-timing) mencakup ukuran margin. Tanpa `--retire-at`, pembunuhan host tanpa sinyal tidak dapat dibedakan dari crash: control plane mencatat pekerja yang hilang daripada pelepasan bersih, dan sesi memasukkan kembali ke runner lain.

<h3 id="network-paths">
  Jalur jaringan
</h3>

Runner dan sesinya membuat beberapa jenis koneksi keluar, dan tidak ada konektivitas masuk dari Anthropic yang diperlukan:

* **Control plane**: runner polling `api.anthropic.com` untuk pekerjaan dan posting peristiwa kemajuan setup dan kegagalan, semua HTTPS keluar. Polling berfungsi ganda sebagai detak jantung runner.
* **SCM connector**: orchestrator opsional [SCM connector](/docs/id/self-hosted-environments-reference#scm-connector-flags) tunnel adalah satu-satunya koneksi WebSocket.
* **Git**: runner mengkloning dari dan mendorong ke host git Anda melalui HTTPS atau SSH, diautentikasi dengan kredensial yang disediakan deployment Anda; [Konfigurasi git](/docs/id/self-hosted-environments-deploy#configure-git) mencakup opsinya, termasuk kredensial yang dicetak per-sesi dan [proxy git Anthropic](/docs/id/self-hosted-environments-deploy#use-the-anthropic-git-proxy), yang merutekan git melalui `api.anthropic.com` sebagai gantinya.
* **Session child**: proses Claude Code anak menyimpan aliran peristiwa sesi ke `api.anthropic.com`, dan membuat panggilan keluar miliknya sendiri untuk inferensi model dan untuk perintah git yang dijalankan selama sesi. Lihat [Persyaratan jaringan](/docs/id/self-hosted-environments-deploy#network-requirements) untuk daftar egress lengkap. [Diagram di atas](#how-self-hosted-environments-work) menunjukkan jalur ini, terlepas dari SCM connector opsional.

Inferensi model menggunakan Anthropic API. Control plane mengirimkan endpoint API ke setiap sesi, dan sesi mengautentikasi dengan token OAuth yang diterbitkan Anthropic, scoped sesi, jadi inferensi tidak dapat dirutekan melalui [Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry](/docs/id/third-party-integrations), atau [LLM gateway](/docs/id/llm-gateway) dalam lingkungan yang di-host sendiri.

Proxy egress korporat didukung. Runner dan [orchestrator autoscaling](/docs/id/self-hosted-environments-configuration#on-demand-runners) opsional menghormati proxy dan variabel mTLS yang dijelaskan dalam [Konfigurasi jaringan](/docs/id/network-config), seperti `HTTPS_PROXY` dan `NO_PROXY`; atur mereka di lingkungan setiap proses. Variabel mencakup panggilan control-plane, [SCM connector](/docs/id/self-hosted-environments-reference#scm-connector-flags) WebSocket orchestrator, dan klon bawaan untuk remote HTTPS, dan sesi mewarisinya dari runner. Streaming sesi menggunakan server-sent events melalui HTTPS, jadi proxy dalam jalur tidak boleh membuffer respons.

Jika proxy Anda juga memerlukan header `Proxy-Authorization`, runner dapat menambahkannya ke setiap koneksi yang dibukanya ke proxy; lihat [Autentikasi ke proxy egress](/docs/id/self-hosted-environments-deploy#authenticate-to-an-egress-proxy).

<h2 id="what-stays-on-your-infrastructure">
  Apa yang tetap pada infrastruktur Anda
</h2>

Checkout repositori, artefak build, rahasia, dan file apa pun yang dibuat atau dimodifikasi sesi tetap pada mesin yang Anda sediakan. Percakapan itu sendiri, termasuk prompt, respons, dan hasil alat, pergi ke `api.anthropic.com` untuk inferensi model, dan Anthropic menyimpan transkrip sesi sehingga Anda dapat melanjutkan sesi dari [permukaan yang didukung](#availability-and-limitations) lain.

Lingkungan yang di-host sendiri memindahkan eksekusi sesi ke dalam jaringan Anda. Control plane tetap di-host Anthropic: orkestrasi sesi, antrian, dan antarmuka claude.ai terus berjalan pada infrastruktur Anthropic.

<h2 id="get-started">
  Memulai
</h2>

Halaman lingkungan yang di-host sendiri diatur oleh apa yang Anda lakukan:

* [Quickstart](/docs/id/self-hosted-environments-quickstart): instal Claude Code, buat environment, mulai runner, dan arahkan sesi pertama Anda
* [Deploy ke produksi](/docs/id/self-hosted-environments-deploy): pengerasan keamanan, egress jaringan, kredensial git, resep Kubernetes dan Compose, masalah yang diketahui, dan troubleshooting
* [Sesuaikan sesi](/docs/id/self-hosted-environments-configuration): skrip wrapper untuk kredensial per-sesi, hook siklus hidup, runner on-demand, server MCP, dan izin
* [Uji end to end](/docs/id/self-hosted-environments-testing): uji smoke CI yang memverifikasi citra runner sebelum Anda mempromosikannya
* [Referensi](/docs/id/self-hosted-environments-reference): setiap flag CLI, variabel lingkungan, metrik, dan endpoint kesehatan
* [Verifikasi identitas sesi](/docs/id/self-hosted-environments-identity): validasi token sesi dari layanan Anda sendiri sebelum memberikan akses
