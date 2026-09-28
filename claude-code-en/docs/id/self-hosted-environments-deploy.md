> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Terapkan lingkungan yang di-host sendiri ke produksi

> Jalankan runner yang di-host sendiri dalam produksi: pengerasan keamanan, kontrol egress jaringan, kredensial git, resep Kubernetes dan Compose, serta pemecahan masalah.

<Note>
  Lingkungan yang di-host sendiri berada dalam beta publik pada paket Team dan Enterprise; [Ketersediaan dan batasan](/docs/id/self-hosted-environments#availability-and-limitations) mencakup jalur pengaktifan. Halaman ini mencakup menjalankan armada dalam produksi; lihat [quickstart](/docs/id/self-hosted-environments-quickstart) untuk runner dan sesi pertama Anda.
</Note>

Sebuah [lingkungan yang di-host sendiri](/docs/id/self-hosted-environments) menjalankan [sesi cloud](/docs/id/claude-code-on-the-web) Claude Code pada runner yang Anda terapkan di dalam jaringan Anda, dan dalam produksi sesi-sesi tersebut menjalankan kode yang diarahkan model atas nama semua orang yang dapat mengirim sesi ke lingkungan. Halaman ini untuk operator yang membawa lingkungan yang berfungsi ke produksi. Ini berfungsi melalui penerapan secara berurutan: apa yang harus dikunci sebelum menghubungkan sistem nyata, egress yang dibutuhkan armada, bagaimana sesi mengautentikasi ke host git Anda, resep penerapan itu sendiri, dan apa yang harus diperiksa ketika sesi berperilaku tidak normal.

<h2 id="harden-your-deployment">
  Keraskan penerapan Anda
</h2>

Runner yang di-host sendiri menjalankan kode arbitrer yang diarahkan model pada infrastruktur Anda atas nama semua orang yang dapat mengirim sesi ke lingkungannya. Itu adalah anggota organisasi Anthropic Anda, dan siapa pun yang dapat memulai sesi saluran [Claude Tag](https://claude.com/docs/claude-tag/overview) dalam cakupan yang diarahkan Pemilik ke lingkungan. Kerjakan setiap item sebelum Anda menghubungkan lingkungan ke sistem produksi:

* **Kontainer ephemeral per-sesi**: jalankan setiap proses runner dalam kontainer atau VM segar yang dihancurkan ketika proses keluar, dengan `--capacity 1` dan `--drain-grace-sec 0` default sehingga setiap kontainer melayani tepat satu sesi. Pada kapasitas yang lebih tinggi, atau dengan grace drain positif, satu kontainer melayani beberapa sesi dari [pemilik terkunci](/docs/id/self-hosted-environments#key-concepts) yang sama; lihat [Runner lifecycle](/docs/id/self-hosted-environments#runner-lifecycle). Jangan gunakan kembali filesystem antara restart runner, kecuali dalam pengaturan [pre-warmed checkout](#reuse-a-pre-warmed-checkout) yang disengaja, dan tidak pernah di seluruh pemilik.
* **Tidak ada kredensial luas dalam gambar**: jangan sertakan kunci SSH jangka panjang, kredensial penyedia cloud, atau token akses pribadi yang memberikan lebih dari yang dibutuhkan sesi. Mint kredensial yang digunakan selama sesi, seperti push atau token API, per sesi dari [script wrapper](/docs/id/self-hosted-environments-configuration#wrapper-scripts) Anda. Untuk klon awal, yang terjadi sebelum wrapper berjalan, gunakan [`checkout` lifecycle hook](/docs/id/self-hosted-environments-configuration#checkout) atau [`--use-anthropic-git-proxy`](#use-the-anthropic-git-proxy); lihat [Konfigurasi git](#configure-git).
* **Simpan rahasia lingkungan dari host yang menjalankan sesi**: rahasia lingkungan dapat mendaftarkan runner dan mengambil sesi apa pun yang antri di lingkungan. Pada armada tetap, itu hidup di setiap host runner, di mana kode sesi apa pun dapat membaca file rahasia. Lebih suka [runner on-demand](/docs/id/self-hosted-environments-configuration#on-demand-runners), di mana rahasia tetap di host orchestrator, yang tidak pernah menjalankan kode pengguna, dan setiap runner menerima perintah kerja sekali pakai yang mendaftarkan tepat satu runner. Pada armada tetap, perlakukan file environment-secret sebagai dapat dibaca oleh setiap sesi dan putar rahasia setelah kompromi sesi yang dicurigai.
* **Egress jaringan default-deny**: batasi lalu lintas keluar kontainer runner dan sesi di batas jaringan Anda sendiri di setiap lingkungan; [Default-deny egress](#default-deny-egress) mencakup apa yang harus diizinkan dan mengapa.
* **IAM host least-privilege**: identitas komputasi yang terpasang ke host runner, seperti profil instans atau akun layanan node, harus memberikan hanya apa yang dibutuhkan runner itu sendiri. Sesi harus mendapatkan kredensial mereka sendiri melalui script wrapper Anda daripada mewarisi identitas host.
* **Blokir endpoint metadata cloud dari sesi**: menjaga sesi dari identitas host memerlukan pemblokiran akses mereka ke endpoint metadata cloud, dan kebijakan egress tingkat subnet tidak mencegat lalu lintas metadata link-local, jadi blokir di kontainer itu sendiri:

  * IMDSv2 dengan batas hop satu
  * GKE Workload Identity dengan penyembunyian metadata
  * Penolakan eksplisit untuk `169.254.169.254` dalam namespace jaringan kontainer sesi

  Blokir berlaku untuk script wrapper dan lifecycle hooks Anda juga, karena mereka berbagi kontainer. Autentikasi pertukaran token apa pun dengan [session JWT](/docs/id/self-hosted-environments-identity) terhadap layanan token Anda sendiri melalui egress yang diizinkan, atau gunakan identitas web berbasis file seperti IAM Roles for Service Accounts (IRSA) di Amazon EKS.
* **Isolasi filesystem per-runner**: setiap proses runner mendapatkan direktori kerjanya sendiri yang tidak dapat dibaca atau ditulis oleh proses lain di host. Buat `--hooks-dir`, script wrapper, dan `~/.claude/` host read-only untuk sesi, baik built-in ke gambar atau dipasang read-only.
* **Dispatch tidak memiliki kontrol akses per-lingkungan**: anggota organisasi Anthropic Anda dapat mengirim sesi ke salah satu lingkungannya. Jika Pemilik [merutekan saluran Claude Tag ke lingkungan](/docs/id/cloud-environments#set-the-environment-a-claude-tag-channel-uses), siapa pun yang [pengaturan akses Claude Tag](https://claude.com/docs/claude-tag/admins/restrict-access#restrict-who-can-use-claude) mengakui dapat memulai sesi saluran yang berjalan di sana. Secara default itu adalah siapa pun di ruang kerja Slack yang terhubung, dengan atau tanpa akun Claude. Perlakukan setiap host runner sebagai dapat dijangkau untuk eksekusi kode oleh semua orang yang dapat mengirim ke sana, dan tempatkan di host runner hanya data dan kredensial yang semua orang tersebut diizinkan untuk membaca. [`--lock-to-account`](/docs/id/self-hosted-environments-reference#runner-cli-flags) membatasi sesi akun mana yang dijalankan host tertentu, tetapi tidak mempersempit siapa yang dapat mengirim ke lingkungan. Untuk membuat lingkungan yang di-host sendiri satu-satunya opsi picker, [Pemilik](/docs/id/cloud-environments#organization-shared-environments) dapat menyembunyikan lingkungan yang di-host Anthropic untuk seluruh organisasi dari halaman [**Cloud environments**](https://claude.ai/admin-settings/cloud-environments).
* **Terapkan penjaga pengaturan repo**: pilih mode penjaga dengan [`--confine-repo-settings`](/docs/id/self-hosted-environments-reference#runner-cli-flags). Default `warn` mencatat pelanggaran dan masih menghasilkan sesi, `enforce` menolak sesi, dan `off` menonaktifkan pemindaian. Runner memindai pengaturan yang berkomitmen di setiap repositori untuk:

  * Hibah yang diselesaikan di luar ruang kerja sesi itu sendiri: entri `additionalDirectories`, aturan `Edit`, `Write`, atau `NotebookEdit` dalam `permissions.allow`, atau entri `sandbox.filesystem.allowWrite` atau `allowRead`
  * Blok `env` yang tidak kosong
  * Penggantian postur operator seperti `sandbox.enabled: false`

  Penjaga berjalan terlepas dari [`--trust-workspace`](/docs/id/self-hosted-environments-reference#runner-cli-flags), dan tidak mencakup hook repositori, `.mcp.json`, atau aturan Bash; lihat [Permissions and tool approval](/docs/id/self-hosted-environments-configuration#permissions-and-tool-approval) untuk tempat hibah tersebut berada.

<Note>
  Daftar IP allowlist organisasi Anda tidak mencakup lalu lintas runner yang di-host sendiri secara default. Jangan andalkan itu sebagai kontrol jaringan untuk lalu lintas runner atau sesi; terapkan default-deny egress di batas jaringan Anda sendiri, dan hubungi tim akun Anthropic Anda jika Anda menginginkan penegakan allowlist IP untuk organisasi Anda.
</Note>

<h2 id="network-requirements">
  Persyaratan jaringan
</h2>

Runner dan anak-anak sesi yang dihasilkannya membuat koneksi keluar ke host di bawah. Batasi egress kontainer sesi ke host ini dan layanan internal spesifik yang perlu dijangkau sesi; [Default-deny egress](#default-deny-egress) mencakup cara dan alasannya.

Host-host ini selalu diperlukan:

| Host                                                                 | Port                                    | Digunakan untuk                                                                                                                                                                                                                                                                                                                                                                   |
| :------------------------------------------------------------------- | :-------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `api.anthropic.com`                                                  | 443, HTTPS; WSS untuk konektor SCM saja | Bidang kontrol runner dan streaming sesi, inferensi model, flag fitur, analitik produk, pengambilan kunci [JWKS](/docs/id/self-hosted-environments-identity), penandatanganan komit, proxy git ketika `--use-anthropic-git-proxy` diatur, dan terowongan [SCM connector](/docs/id/self-hosted-environments-reference#scm-connector-flags) orchestrator ketika `--scm-connector-host` diatur |
| Host git Anda, seperti `github.com` atau host GitHub Enterprise Anda | 443 atau 22                             | Kloning dan push repositori. Tidak diperlukan jika runner menggunakan `--use-anthropic-git-proxy`, yang merutekan lalu lintas git melalui `api.anthropic.com`.                                                                                                                                                                                                                    |

Apakah host-host ini diperlukan tergantung pada konfigurasi Anda:

| Host                                 | Port | Ketika diperlukan                                                                                                                                                                                                                                                                                 |
| :----------------------------------- | :--- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `downloads.claude.ai`                | 443  | Pada waktu instalasi, ketika Anda menginstal atau memperbarui Claude Code di host dengan installer asli; script `install.sh` itu sendiri disajikan dari `claude.ai`. Pada waktu runtime sesi, hanya ketika sesi menginstal plugin dari marketplace Anthropic resmi.                               |
| `storage.googleapis.com`             | 443  | Pada waktu runtime sesi, untuk hitungan instalasi plugin dan metadata yang ditampilkan dalam `/plugin`.                                                                                                                                                                                           |
| `code.claude.com` dan `claude.com`   | 443  | Pencarian dokumentasi oleh agen claude-code-guide built-in dan permintaan WebFetch yang telah disetujui sebelumnya selama sesi. Memblokir host-host ini hanya mempengaruhi pencarian dokumentasi.                                                                                                 |
| `*.frame.claudeusercontent.com`      | 443  | Hanya ketika [alat Artifact](/docs/id/artifacts#availability) tersedia untuk sesi di organisasi Anda; default bervariasi menurut paket, sesuai tabel ketersediaan di sana. Atur `CLAUDE_CODE_DISABLE_ARTIFACT=1` di runner untuk menjaga alat tetap dinonaktifkan terlepas dari pengaturan organisasi. |
| `registry.npmjs.org`                 | 443  | Ketika sesi menginstal plugin, baik untuk mengambil paket plugin sumber npm maupun untuk menginstal dependensi Node.js plugin, atau ketika server MCP yang diluncurkan `npx` berjalan                                                                                                             |
| `http-intake.logs.us5.datadoghq.com` | 443  | Metrik operasional Anthropic. Hanya ketika `CLAUDE_CODE_BYOC_ENABLE_DATADOG=1` diatur; dimatikan secara default di lingkungan yang di-host sendiri.                                                                                                                                               |
| `browser-intake-us5-datadoghq.com`   | 443  | Unggahan laporan kesalahan Anthropic, dikirim hanya ketika [pelaporan kesalahan](/docs/id/data-usage#telemetry-services) diaktifkan untuk akun sesi. Ditekan oleh `DISABLE_ERROR_REPORTING=1` atau `DISABLE_TELEMETRY=1`.                                                                              |

Runner tidak menjangkau `statsig.anthropic.com`, `*.sentry.io`, `claude.ai`, atau `platform.claude.com`. Host-host ini muncul dalam beberapa daftar periksa jaringan enterprise yang lebih lama, tetapi Anda tidak perlu mengizinkan daftar mereka untuk lalu lintas runner atau sesi: pengambilan flag fitur pergi ke `api.anthropic.com`, dan runner mengautentikasi dengan rahasia lingkungan daripada OAuth interaktif. Dua alur sisi host memang menjangkau `claude.ai`, jadi jalankan dari host yang egress-nya memungkinkannya daripada memperluas egress kontainer sesi: installer satu baris mengambil `install.sh` dari `claude.ai` pada waktu instalasi, dan `claude auth login` interaktif, yang [guided setup](/docs/id/self-hosted-environments-quickstart#set-up-an-environment-and-runner), mode signed-in `doctor`, dan [CI dispatch](/docs/id/self-hosted-environments-testing#authenticate-from-ci) gunakan, masuk melalui `claude.ai`, `claude.com`, dan `platform.claude.com`. `mcp-proxy.anthropic.com` juga tidak diperlukan: sesi yang di-host sendiri tidak menggunakannya, dan pengiriman konektor claude.ai organisasi Anda ke sesi, ketika diaktifkan untuk organisasi Anda, merutekan melalui `api.anthropic.com`. Lihat [MCP servers](/docs/id/self-hosted-environments-configuration#mcp-servers).

<h3 id="default-deny-egress">
  Default-deny egress
</h3>

Terapkan kontainer runner dan sesi dalam segmen jaringan atau namespace yang lalu lintas keluarnya dibatasi ke host dalam [tabel persyaratan jaringan](#network-requirements), host git Anda, dan layanan internal spesifik yang perlu dijangkau sesi. Produk tidak dapat memverifikasi atau menegakkan ini, jadi terapkan di batas jaringan Anda sendiri di setiap lingkungan. Kode sesi diarahkan model dan dapat mencoba koneksi ke host arbitrer; default-deny egress di lapisan jaringan membatasi di mana upaya tersebut dapat mendarat. Ini berlaku terlepas dari mode izin: set alat yang telah disetujui sebelumnya sudah mencakup `Bash`, jadi egress shell berjalan tanpa prompt bahkan tanpa [mode auto](/docs/id/self-hosted-environments-configuration#permissions-and-tool-approval).

Untuk detail tentang telemetri mana yang dipancarkan setiap sesi dan cara mematikannya, lihat [Telemetry](/docs/id/self-hosted-environments-reference#telemetry).

<h3 id="authenticate-to-an-egress-proxy">
  Autentikasi ke proxy egress
</h3>

Beberapa proxy egress korporat memerlukan header `Proxy-Authorization` pada setiap koneksi. Token dalam header itu sering berputar terlalu cepat untuk ditulis ke URL proxy yang Anda atur dalam `HTTPS_PROXY`. Atur `HTTPS_PROXY` atau `HTTP_PROXY` ke URL proxy Anda seperti biasa, kemudian atur `--proxy-authorization-command` atau `--proxy-authorization-file` untuk memberi tahu runner di mana membaca nilai header. Kedua flag memerlukan Claude Code v2.1.238 atau lebih baru.

<h4 id="choose-where-the-proxy-authorization-value-comes-from">
  Pilih dari mana nilai `Proxy-Authorization` berasal
</h4>

Pilih flag yang cocok dengan cara Anda menghasilkan token `Proxy-Authorization`:

* **[`--proxy-authorization-command <command>`](/docs/id/self-hosted-environments-reference#runner-cli-flags)**: pilih ini untuk token yang Anda hasilkan sesuai permintaan. Runner menjalankan perintah shell dan menggunakan stdout yang dipangkas sebagai nilai header, misalnya `Bearer <token>`.
* **[`--proxy-authorization-file <path>`](/docs/id/self-hosted-environments-reference#runner-cli-flags)**: pilih ini untuk token yang proses lain putar di tempat. Runner membaca file dan menggunakan isinya yang dipangkas sebagai nilai header.

<h4 id="configurations-the-runner-refuses-to-start-with">
  Konfigurasi yang menolak runner untuk dimulai dengan
</h4>

Setiap flag juga memiliki bentuk variabel lingkungan, tercantum di sebelahnya dalam [referensi flag CLI runner](/docs/id/self-hosted-environments-reference#runner-cli-flags). Sebelum runner menghubungi proxy atau bidang kontrol Anda, itu memeriksa flag dan variabelnya, dan menolak untuk memulai dalam tiga kasus:

* **Kedua flag diatur**: satu flag ditambah variabel lingkungan flag lainnya dihitung sebagai pengaturan keduanya.
* **Tidak ada URL proxy**: baik `HTTPS_PROXY` maupun `HTTP_PROXY` tidak memegang URL `http://` atau `https://`. Runner membaca kedua variabel dalam huruf besar atau kecil, dan tidak berkonsultasi dengan `ALL_PROXY`.
* **Flag apa pun diteruskan ke subperintah orchestrator**: `self-hosted-runner orchestrator` tidak menerima flag atau variabel lingkungannya. Teruskan flag ke setiap runner yang dimulai orchestrator.

<h4 id="what-the-runner-changes-while-a-proxy-authorization-flag-is-set">
  Apa yang diubah runner saat flag proxy-authorization diatur
</h4>

Dengan flag apa pun yang diatur, runner memulai pendengar miliknya sendiri dan mengirim lalu lintas proxy dari dirinya sendiri, lifecycle hooks-nya, dan sesi-sesinya melalui pendengar itu. Pendengar menambahkan header `Proxy-Authorization` dalam perjalanan ke proxy Anda.

* **Pendengar**: pendengar adalah proxy forward di `127.0.0.1`. Runner memulai pendengar sebelum mendaftar dengan bidang kontrol, dan keluar pada startup jika pendengar tidak dapat dimulai.
* **Variabel proxy**: runner menulis ulang mana pun dari `HTTPS_PROXY` dan `HTTP_PROXY` yang Anda atur sehingga menunjuk ke pendengar. Nilai yang ditulis ulang itu menjangkau runner itu sendiri, lifecycle hooks-nya, dan setiap sesi yang dijalankannya.
* **Rotasi token**: token yang diputar berlaku tanpa restart. Untuk setiap koneksi yang dibuka pendengar ke proxy Anda, runner menjalankan perintah Anda atau membaca file Anda lagi dan menambahkan hasilnya sebagai header.
* **Lingkungan sesi**: sesi menjangkau proxy Anda hanya melalui pendengar. Dalam lingkungan setiap sesi, runner menghapus `ALL_PROXY`, menghapus ejaan apa pun dari `HTTPS_PROXY` atau `HTTP_PROXY` yang tidak Anda atur, dan menyematkan `NO_PROXY` ke nilai runner itu sendiri.
* **Log**: runner tidak pernah mencatat nilai header.

<h2 id="configure-git">
  Konfigurasi git
</h2>

Runner mengelola checkout repositori tetapi tidak mengonfigurasi identitas git atau kredensial secara default. Anda mengontrol gambar dan lingkungan proses runner, jadi Anda mengontrol konfigurasi git. Pilih salah satu dari dua pendekatan:

* **Biarkan runner mengonfigurasi git**: mulai runner dengan `--configure-git` untuk memilikinya menulis identitas yang sama dan konfigurasi penandatanganan komit yang digunakan sesi yang di-host Anthropic
* **Kirim konfigurasi git dalam gambar Anda**: atur identitas dan kredensial push sendiri, misalnya untuk berkomitmen di bawah identitas bot Anda sendiri

Lantai versi Git di host runner: [`--configure-git`](#let-the-runner-configure-git) penandatanganan komit SSH memerlukan Git 2.34 atau lebih baru, [`--use-anthropic-git-proxy`](#use-the-anthropic-git-proxy) memerlukan 2.32 atau lebih baru, dan melanjutkan sesi dari cabang yang didorong oleh [`--push-outcome-on-release`](/docs/id/self-hosted-environments-reference#runner-cli-flags) memerlukan 2.29 atau lebih baru. Git 2.24 cukup jika Anda menghilangkan ketiganya dan mengelola identitas git sendiri.

<h3 id="let-the-runner-configure-git">
  Biarkan runner mengonfigurasi git
</h3>

Mulai runner dengan `--configure-git`, atau atur `SELF_HOSTED_RUNNER_CONFIGURE_GIT=1`, untuk memilikinya menulis konfigurasi git global pada startup:

* `user.name = Claude` dan `user.email = noreply@anthropic.com`, cocok dengan sesi yang di-host Anthropic
* Penandatanganan komit dan tag format SSH, dirutekan melalui shim yang dikelola runner yang menandatangani setiap komit melalui layanan penandatanganan Anthropic menggunakan kredensial sesi itu sendiri. Tanda tangan dapat diverifikasi di GitHub terhadap kunci penandatanganan SSH yang dipublikasikan Anthropic.
* `push.negotiate = true`, jadi git menanyakan host git Anda komit mana yang sudah dimilikinya sebelum mengemas push. Memerlukan Claude Code v2.1.257 atau lebih baru.
* `core.hooksPath` menunjuk ke direktori hooks yang dikelola runner. Hook `commit-msg` dan `prepare-commit-msg`-nya menambahkan trailer `Co-authored-by:` untuk pembuat sesi ke setiap komit, dibangun dari email dalam [`CCR_SESSION_ACCOUNT_EMAIL`](/docs/id/self-hosted-environments-configuration#wrapper-scripts) dan dihilangkan ketika variabel itu tidak diatur. Jika gambar Anda sudah menetapkan `core.hooksPath`, runner membiarkan pengaturan Anda tetap, melewati instalasi hook ini, dan mencetak peringatan `[runner:git]`.

Penandatanganan komit memerlukan git 2.34 atau lebih baru; runner memeriksa pada startup dan keluar dengan kesalahan jika git Anda lebih lama. Flag ini tidak mengonfigurasi kredensial push, yang masih Anda sediakan dalam gambar.

<h3 id="ship-git-config-in-your-image">
  Kirim konfigurasi git dalam gambar Anda
</h3>

Identitas git diperlukan untuk komit apa pun. Aturnya di seluruh sistem dalam Dockerfile Anda sehingga konfigurasi berlaku terlepas dari pengguna mana yang menjalankan proses runner:

```dockerfile theme={null}
RUN git config --system user.name "Claude" && \
    git config --system user.email "noreply@anthropic.com"
```

Tanpa identitas, `git commit` gagal dengan `Please tell me who you are` dan sesi tidak dapat membuat kemajuan. Anda dapat menggunakan identitas bot Anda sendiri; runner tidak mengganti nilai-nilai ini.

Jangan memanggang kredensial push jangka panjang atau berskop luas ke gambar runner bersama: kredensial dalam gambar tersedia untuk setiap sesi yang dijalankan gambar, siapa pun yang memulainya. Sebagai gantinya, mint token jangka pendek, least-scoped per sesi dari [script wrapper](/docs/id/self-hosted-environments-configuration#wrapper-scripts) Anda, menggunakan identitas pembuat sesi yang didekode dari session JWT. Pasangkan dengan kontainer per-sesi ephemeral, yang memerlukan `--capacity 1`, jadi tidak ada kredensial yang melampaui sesi yang memintnya; lihat [bagian pengerasan](#harden-your-deployment).

Jika Anda harus mengonfigurasi kredensial push di tingkat gambar, misalnya untuk kunci deploy read-only, cakupkan seerat mungkin yang diizinkan host git Anda:

* Kunci deploy SSH terbatas pada satu repositori dengan rewrite `url.<base>.insteadOf`
* `credential.helper` yang mengembalikan token berskop minimal
* `GIT_SSH_COMMAND` menunjuk ke kunci berskop sempit

Mekanisme apa pun yang Anda konfigurasi harus bekerja tanpa prompt, karena klon built-in runner dan fetch menonaktifkan prompt yang git, SSH, dan Git Credential Manager akan menampilkan:

* Runner menetapkan `GIT_TERMINAL_PROMPT=0`, jadi git tidak meminta nama pengguna atau kata sandi.
* Runner menjalankan SSH dengan `BatchMode=yes`, ditambahkan ke `GIT_SSH_COMMAND` Anda jika Anda menetapkan satu, jadi SSH tidak meminta frasa sandi atau konfirmasi host.
* Runner menetapkan `GCM_INTERACTIVE=never`, jadi Git Credential Manager tidak membuka dialog sign-in.
* Runner menghapus `core.askPass`, jadi jika Anda menggunakan helper askpass, aturnya melalui variabel lingkungan `GIT_ASKPASS` sebagai gantinya.

Jika host git Anda menolak kredensial, atau Anda tidak mengonfigurasi satu, runner mencoba beberapa kali dan kemudian gagal persiapan repositori ketika repositori adalah yang didorong sesi untuk hasil. Untuk repositori yang hanya dibaca sesi, [Troubleshooting](#troubleshooting) mencakup kapan runner melewatinya. Runner tidak meneruskan pengaturan ini ke lingkungan sesi.

Jika direktori checkout dimiliki oleh uid berbeda dari proses runner, git menolak untuk beroperasi pada mereka; tambahkan `safe.directory`:

```dockerfile theme={null}
RUN git config --system --add safe.directory '*'
```

<h3 id="use-the-anthropic-git-proxy">
  Gunakan proxy git Anthropic
</h3>

Mulai runner dengan `--use-anthropic-git-proxy`, atau atur `CLAUDE_RUNNER_USE_GIT_PROXY=1`, untuk memilikinya klon melalui proxy git Anthropic, diautentikasi dengan token jangka pendek sesi itu sendiri. Untuk sesi pengguna biasa, proxy menggunakan token OAuth GitHub atau GitHub Enterprise yang disimpan untuk pembuat sesi; untuk sesi bot dan agen, itu menggunakan token instalasi GitHub App organisasi Anda. Bagaimanapun, gambar runner tidak memerlukan kredensial git sama sekali: tidak ada kunci SSH, tidak ada credential helper, tidak ada `.netrc`. Ini adalah jalur auth yang sama yang digunakan lingkungan yang di-host Anthropic.

Proxy memerlukan `--capacity 1` karena URL proxy adalah per-sesi, dan git 2.32 atau lebih baru karena git yang lebih lama mengabaikan mekanisme konfigurasi yang digunakan proxy untuk mengisolasi sesi satu sama lain. Runner menolak untuk memulai jika salah satu persyaratan tidak terpenuhi. Karena proxy mengambil dari sisi Anthropic, host git Anda harus dapat dijangkau dari infrastruktur Anthropic, persyaratan yang sama yang dimiliki sesi yang di-host Anthropic; untuk host git yang hanya dapat dirutekan di dalam jaringan Anda, gunakan [`checkout` lifecycle hook](/docs/id/self-hosted-environments-configuration#checkout). Setiap proses runner menangani satu sesi pada satu waktu, jadi jalankan lebih banyak replika untuk paralelisme. Ketika proxy diaktifkan, `--git-host-rewrite` dan `--git-ssh-rewrite` tidak berpengaruh: URL proxy menunjuk ke `api.anthropic.com`, bukan host git Anda.

Runner juga melaporkan opt-in ke Anthropic ketika mendaftar, mencetak `Registering as opted in to Anthropic-managed git (--use-anthropic-git-proxy)` pada startup. Melaporkan opt-in memerlukan Claude Code v2.1.267 atau lebih baru, dan versi sebelumnya menerima flag tanpa melaporkannya atau mencetak baris itu. Setiap sesi pada runner yang telah opt-in kemudian menggunakan baik git yang dikelola Anthropic atau URL proxy per-sesi. Ketika sesi menggunakan URL proxy per-sesi, runner mencatat satu baris `[runner:warn]` mengatakan demikian.

<h3 id="rewrite-git-urls-for-private-networks">
  Tulis ulang URL git untuk jaringan pribadi
</h3>

URL repositori tiba dari bidang kontrol sebagai HTTPS, dengan nama host host git Anda; untuk GitHub Enterprise, itu adalah nama host yang Anda konfigurasi untuk [integrasi GitHub Enterprise](/docs/id/github-enterprise-server) dalam pengaturan admin Claude Code di claude.ai. Dua flag yang dapat diulang menulis ulang URL tersebut sebelum klon:

* `--git-host-rewrite <from>=<to>`: untuk split-horizon DNS, di mana Anthropic menjangkau host git Anda melalui nama host eksternal tetapi runner harus menggunakan yang internal
* `--git-ssh-rewrite <host>`: untuk host git yang hanya menerima SSH, menulis ulang `https://<host>/owner/repo` ke `git@<host>:owner/repo`

Penulisan ulang host berjalan terlebih dahulu, jadi daftar nama host internal dalam `--git-ssh-rewrite` jika Anda memerlukan keduanya. Untuk kontrol penuh atas checkout, gunakan [`checkout` lifecycle hook](/docs/id/self-hosted-environments-configuration#checkout).

<h2 id="build-the-runner-image">
  Bangun gambar runner
</h2>

Anthropic tidak menerbitkan gambar runner yang telah dibangun sebelumnya. Bangun milik Anda sendiri di sekitar biner `claude`, berlapis dengan toolchain apa pun yang dibutuhkan repositori Anda: runtime bahasa, compiler, package manager, dan sidecar [MCP](/docs/id/mcp).

Resep di bawah menggunakan `--capacity 4`, jadi satu kontainer melayani hingga empat sesi bersamaan dari pemilik terkunci yang sama. Itu tidak memberikan isolasi kontainer per-sesi dalam [bagian pengerasan](#harden-your-deployment): sebelum menghubungkan lingkungan ke sistem produksi, jalankan resep pada `--capacity 1` dengan satu kontainer per sesi, atau gunakan [runner on-demand](/docs/id/self-hosted-environments-configuration#on-demand-runners), yang juga menjaga rahasia lingkungan dari host yang menjalankan sesi.

Dockerfile ini adalah titik awal minimal:

```dockerfile theme={null}
FROM debian:bookworm-slim
ARG CLAUDE_CODE_VERSION
RUN apt-get update && apt-get install -y --no-install-recommends git curl ca-certificates openssh-client \
 && rm -rf /var/lib/apt/lists/*
RUN curl -fsSL "https://downloads.claude.ai/claude-code-releases/${CLAUDE_CODE_VERSION:?set with --build-arg CLAUDE_CODE_VERSION}/linux-x64/claude" \
      -o /usr/local/bin/claude && chmod +x /usr/local/bin/claude
RUN git config --system user.name "Claude" \
 && git config --system user.email "noreply@anthropic.com" \
 && git config --system --add safe.directory '*'
ENTRYPOINT ["claude"]
```

Tukar `linux-x64` untuk `linux-arm64` jika node Anda adalah ARM, atau untuk `linux-x64-musl` atau `linux-arm64-musl` pada gambar berbasis musl seperti Alpine; lihat [Alpine Linux setup](/docs/id/setup#alpine-linux-and-musl-based-distributions) untuk paket tambahan yang dibutuhkan gambar musl. URL adalah lokasi rilis Claude Code standar, jadi Anda dapat memverifikasi biner yang diunduh terhadap manifes yang ditandatangani rilis seperti yang dijelaskan dalam [Binary integrity and code signing](/docs/id/setup#binary-integrity-and-code-signing). Bangun gambar dengan Claude Code versi 2.1.224 atau lebih baru, kemudian dorong ke registri Anda dan referensikan dalam resep di bawah:

```bash theme={null}
docker build --build-arg CLAUDE_CODE_VERSION=2.1.267 -t <your-registry>/claude-runner:latest .
```

<h2 id="size-cpu-and-memory-for-sessions">
  Ukuran CPU dan memori untuk sesi
</h2>

Ukuran kontainer atau host runner untuk sesi yang dijalankannya daripada untuk proses runner. Runner itu sendiri menanyakan pekerjaan, menyiapkan checkout setiap sesi, menjalankan [lifecycle hooks](/docs/id/self-hosted-environments-configuration#lifecycle-hooks) Anda, dan memulai serta mengawasi proses sesi. Beban datang dari sesi: masing-masing adalah proses Claude Code ditambah apa pun yang dimulainya, seperti build, test suite, instalasi paket, dan [MCP server](/docs/id/mcp).

Untuk satu sesi, mulai dengan nilai-nilai berikut, dinyatakan sebagai permintaan dan batas Kubernetes atau setara platform Anda, dan perlakukan sebagai titik awal daripada persyaratan:

* **Memori**: permintaan dan batas 4 GiB masing-masing, yang memenuhi minimum 4 GB dalam [persyaratan sistem](/docs/id/setup#system-requirements) Claude Code. Jaga keduanya tetap sama sehingga penjadwal memperhitungkan memori penuh kontainer. Ketika kontainer mencapai batas memorinya, kernel membunuh proses di dalamnya, yang dapat mengakhiri sesi di tengah-tugas.
* **CPU**: permintaan 2 CPU dan batas 4 CPU, jadi sesi dapat meledak di atas permintaan selama build. Kernel membatasi kontainer pada batas CPU-nya daripada membunuh proses di dalamnya, jadi sesi pada batas berjalan lebih lambat tetapi terus berjalan.

Dalam spec kontainer Kubernetes, atur nilai awal tersebut dengan blok `resources` berikut:

```yaml theme={null}
resources:
  requests:
    cpu: "2"
    memory: 4Gi
  limits:
    cpu: "4"
    memory: 4Gi
```

Build dan test biasanya merupakan bagian terbesar dan paling variabel dari beban sesi, jadi jalankan build representatif repositori Anda, ukur puncak CPU dan memorinya, dan naikkan nilai awal apa pun yang tidak meninggalkan ruang untuk proses Claude Code di atas puncak itu.

Runner menggunakan `--capacity` untuk membatasi berapa banyak sesi yang dijalankannya sekaligus. Itu tidak membagi CPU atau memori di antara mereka, jadi sesi pada runner berbagi CPU dan memori kontainer. Untuk membatasi bagian satu sesi, terapkan batas dari [script wrapper](/docs/id/self-hosted-environments-configuration#wrapper-scripts) Anda. Apa yang harus diberikan satu kontainer oleh karena itu tergantung pada berapa banyak sesi yang dilayaninya sekaligus:

* **Satu sesi per runner**: berikan setiap kontainer nilai satu sesi. Gunakan ukuran ini pada `--capacity 1`, yang [bagian pengerasan](#harden-your-deployment) rekomendasikan, dan untuk [runner on-demand](/docs/id/self-hosted-environments-configuration#on-demand-runners), di mana Anda menetapkan nilai pada workload yang [`spawn-runner` hook](/docs/id/self-hosted-environments-configuration#the-spawn-runner-hook) Anda kirimkan, seperti template pod Job Kubernetes.
* **Beberapa sesi per runner**: pada `--capacity` di atas satu, kalikan nilai satu sesi dengan kapasitas, karena hingga banyak sesi dapat berjalan dalam kontainer sekaligus. Resep [Kubernetes](#kubernetes) dan [Docker Compose](#docker-compose) menjalankan `--capacity 4` tanpa batas CPU atau memori, jadi tambahkan batas yang diukur untuk kapasitas yang Anda jalankan.

<h2 id="kubernetes">
  Kubernetes
</h2>

Runner melayani `GET /healthz` di port 8080 secara default, dapat dikonfigurasi dengan `--health-port`, jadi probe Kubernetes bekerja tanpa setup tambahan. Endpoint mengembalikan `200` kapan pun proses hidup, jadi probe di bawah mendeteksi proses mati, bukan yang terjebak; untuk menangkap runner yang berhenti polling, beri peringatan pada seri `last_poll_age_seconds` dari [`/metrics`](/docs/id/self-hosted-environments-reference#prometheus-metrics). Deployment di bawah memasang rahasia lingkungan dari Secret Kubernetes, menunjuk probe liveness dan readiness ke `/healthz`, dan menetapkan periode grace terminasi 90 detik. Lihat [Shutdown timing](#shutdown-timing) untuk alasan periode grace penting.

Manifest tidak menetapkan `resources` CPU atau memori pada kontainer runner. Tambahkan blok yang diukur untuk kapasitas yang Anda jalankan, seperti [Size CPU and memory for sessions](#size-cpu-and-memory-for-sessions) jelaskan.

```yaml theme={null}
apiVersion: apps/v1
kind: Deployment
metadata:
  name: claude-runner
  namespace: claude-runners
spec:
  replicas: 3
  selector:
    matchLabels:
      app: claude-runner
  template:
    metadata:
      labels:
        app: claude-runner
        app.kubernetes.io/part-of: claude-code-self-hosted-runner
    spec:
      terminationGracePeriodSeconds: 90
      containers:
        - name: runner
          image: <your-registry>/claude-runner:latest
          args:
            - self-hosted-runner
            - --environment-secret-file
            - /etc/claude/environment-secret
            - --capacity
            - "4"
          volumeMounts:
            - name: environment-secret
              mountPath: /etc/claude
              readOnly: true
          ports:
            - name: health
              containerPort: 8080
          readinessProbe:
            httpGet:
              path: /healthz
              port: 8080
            initialDelaySeconds: 5
            periodSeconds: 10
          livenessProbe:
            httpGet:
              path: /healthz
              port: 8080
            initialDelaySeconds: 30
            periodSeconds: 30
      volumes:
        - name: environment-secret
          secret:
            secretName: claude-runner-environment-secret
```

Deployment di atas hidup dalam namespace `claude-runners`. Buat namespace terlebih dahulu:

```bash theme={null}
kubectl create namespace claude-runners
```

Buat Secret pendukung dari file lokal yang memegang nilai yang Anda salin dalam langkah [**Copy environment key**](/docs/id/self-hosted-environments-quickstart#set-up-an-environment-and-runner) UI admin, jadi rahasia tidak pernah muncul dalam riwayat shell Anda. Jalankan `(umask 077 && cat > ./environment-secret)`, tempel rahasia, tekan Enter, kemudian Ctrl-D. Kemudian buat Secret dan hapus file:

```bash theme={null}
kubectl create secret generic claude-runner-environment-secret -n claude-runners --from-file=environment-secret=./environment-secret
```

<h2 id="docker-compose">
  Docker Compose
</h2>

Layanan Compose di bawah memulai ulang runner kapan pun keluar, yang mencakup crash dan exit normal setelah draining. Kebijakan restart Docker memulai ulang kontainer yang sama dengan lapisan yang dapat ditulisnya utuh, jadi runner kembali pada filesystem yang digunakan kembali daripada yang segar yang [postur pengerasan](#harden-your-deployment) rekomendasikan; gunakan resep ini untuk evaluasi, dan untuk produksi baik buat ulang kontainer per run atau gunakan orchestrator yang melakukannya.

```yaml theme={null}
services:
  claude-runner:
    image: <your-registry>/claude-runner:latest
    command:
      - self-hosted-runner
      - --environment-secret-file
      - /run/secrets/environment-secret
      - --capacity
      - "4"
    secrets:
      - environment-secret
    restart: always
    stop_grace_period: 90s

secrets:
  environment-secret:
    file: ./environment-secret
```

<h2 id="shutdown-timing">
  Waktu shutdown
</h2>

Pada `SIGTERM`, runner berhenti menerima pekerjaan baru dan, kecuali Anda menetapkan [`--defer-shutdown-max-min`](#defer-the-drain-past-the-first-signal), menunggu hingga `--drain-wait-sec`, nol secara default, agar turn yang sedang berjalan selesai, menghentikan pohon proses setiap sesi, dan menjalankan hook lifecycle [`post-session`](/docs/id/self-hosted-environments-configuration#post-session). Pohon proses tersebut mencakup perintah yang masih dijalankan Claude dalam sesi.

Jalur drain penuh memerlukan hingga `--session-stop-grace-sec` + `--drain-wait-sec` + `--post-session-hook-timeout-sec`, ditambah 15 detik overhead tetap untuk pembersihan proses, ditambah 30 detik lagi ketika [`--push-outcome-on-release`](/docs/id/self-hosted-environments-reference#runner-cli-flags) diatur. Itu adalah 80 detik pada default, dan runner mencatat totalnya saat startup. Sesi drain secara paralel di bawah satu anggaran ini, jadi totalnya tidak bertambah dengan `--capacity`.

Pada `--drain-wait-sec 0` default, restart bergulir mengganggu turn yang sedang berjalan; setiap sesi dilanjutkan di runner lain, kehilangan pekerjaan yang tidak didorong seperti yang dijelaskan di bawah [Known issues](#additional-limitations). Atur `--drain-wait-sec`, dan naikkan periode grace untuk mencocokkan, untuk membiarkan turn selesai terlebih dahulu.

Sepanjang seluruh jalur itu, runner terus melakukan heartbeat ke control plane pada kapasitas nol, sehingga lease sesi tidak kedaluwarsa dan tidak dikembalikan ke runner lain sementara hook `post-session` masih menulis pekerjaan yang belum dikomit. Heartbeat berhenti tepat sebelum runner deregisters.

Berikan runner setidaknya total yang dicatat saat startup sebelum host menghentikannya. Di mana Anda menetapkan itu tergantung pada cara host Anda berhenti:

* **Dengan periode grace `SIGTERM`**: atur `terminationGracePeriodSeconds` pada Kubernetes, `stop_grace_period` pada Docker Compose, atau setara orchestrator Anda ke setidaknya total itu. Default Kubernetes 30 detik lebih pendek dari jalur drain runner, jadi Kubernetes menghentikan pod sebelum runner selesai draining.
* **Dengan [`--retire-at`](/docs/id/self-hosted-environments-reference#runner-cli-flags)**: ukuran margin antara waktu retire dan waktu stop host untuk mencakup turn tipikal, ditambah background-task hold yang dijelaskan [Runner lifecycle](/docs/id/self-hosted-environments#runner-lifecycle), ditambah total yang sama. Hitung waktu retire di setiap peluncuran, misalnya `date +%s` ditambah lifetime yang dimaksudkan runner.
* **Dengan [`--defer-shutdown-max-min`](#defer-the-drain-past-the-first-signal)**: tambahkan dua bagian lagi ke total jalur drain. Yang pertama adalah menit yang Anda konfigurasi. Yang kedua adalah post-release grace yang dijelaskan [Defer the drain past the first signal](#defer-the-drain-past-the-first-signal), 75 detik pada default. Dengan flag diatur, runner juga mencetak angka gabungan saat startup, setelah total jalur drain.

<h3 id="defer-the-drain-past-the-first-signal">
  Defer the drain past the first signal
</h3>

Atur [`--defer-shutdown-max-min <n>`](/docs/id/self-hosted-environments-reference#runner-cli-flags) jika Anda ingin runner yang sedang Anda restart terus melayani sesi yang dipegang selama hingga `n` menit, alih-alih mendrainnya pada sinyal pertama. Pada `SIGTERM` atau `SIGINT` pertama, runner berhenti menerima pekerjaan baru dan terus melayani sesi yang dipegang. Ini terus polling sehingga control plane tidak mengantrekan kembali sesi tersebut. Memerlukan Claude Code v2.1.238 atau lebih baru.

<h4 id="what-happens-to-the-sessions-the-runner-holds-after-the-first-signal">
  Apa yang terjadi pada sesi yang dipegang runner setelah sinyal pertama
</h4>

Dalam dua tahap pertama yang mengikuti sinyal, runner melepaskan sesi, dan sesi yang dirilis dilanjutkan di runner segar ketika pengguna mereka mengirim pesan berikutnya. Menghitung dari sinyal pertama, runner bergerak melalui tiga tahap:

* **Untuk `n` menit pertama**: runner melayani sesinya secara normal dan terus memberlakukan `--startup-timeout-min` dan `--kill-session-after-min`. Jika Anda juga menetapkan [`--release-idle-session-min`](/docs/id/self-hosted-environments-reference#runner-cli-flags), runner melepaskan sesi apa pun yang penggunanya telah idle selama itu; tanpanya, sesi idle tetap di runner.
* **Ketika `n` menit habis**: runner melepaskan setiap sesi yang masih dipegang, idle atau tidak. Runner menunggu turn sesi mid-turn berakhir, dan hingga 60 detik lagi untuk background tasks turn, sebelum melepaskan sesi itu.
* **Ketika post-release grace habis**: runner mendrainkan sesi apa pun yang masih dipegang, dan control plane mengantrekan kembali setiap sesi yang didrain ke runner lain segera. Post-release grace dimulai ketika `n` menit habis dan 75 detik pada default. Jika Anda menetapkan `--drain-wait-sec` di atas 60 detik, post-release grace adalah `--drain-wait-sec` ditambah 15 detik sebagai gantinya.

Di tahap mana pun, runner keluar 0 segera setelah tidak memegang sesi. Sinyal kedua memotong tahap pendek: runner mendrainkan segera, seperti yang dilakukan pada sinyal pertama tanpa `--defer-shutdown-max-min`. Setelah drain sedang berlangsung, sinyal berikutnya force-exits runner. Itu berlaku apakah sinyal kedua atau post-release grace yang habis memulai drain.

<h4 id="size-the-stop-timeout">
  Ukuran stop timeout
</h4>

Berikan stop timeout host Anda setidaknya jumlah dari tiga bagian: `n` menit yang Anda konfigurasi, post-release grace, dan jalur drain penuh yang dijelaskan [Shutdown timing](#shutdown-timing). Dengan pengaturan default post-release grace adalah 75 detik dan jalur drain adalah 80 detik, jadi izinkan `n` menit ditambah 155 detik. Runner mencetak jumlah ini saat startup kapan pun `--defer-shutdown-max-min` diatur.

Jika stop timeout habis sebelum runner selesai, host membunuh runner. Sesi yang masih dipegang tidak mendapatkan hook `post-session`. Runner tidak deregisters, dan control plane mengantrekan kembali sesi sekitar satu menit kemudian. Jika Anda tidak dapat memberikan stop timeout jumlah itu, biarkan `--defer-shutdown-max-min` tidak diatur sehingga runner mendrainkan pada sinyal pertama sebagai gantinya.

<h3 id="what-reaches-a-running-post-session-hook">
  Apa yang mencapai hook post-session yang sedang berjalan
</h3>

Hook `post-session` dan setiap anak sesi Claude masing-masing berjalan dalam kelompok proses POSIX mereka sendiri, terpisah dari runner, jadi mekanisme stop mencapainya secara berbeda:

* **`SIGTERM` sementara runner sudah draining**: force-exits runner segera, melewati apa pun yang tersisa dari jalur drain. Tanpa [`--defer-shutdown-max-min`](#defer-the-drain-past-the-first-signal), itu adalah `SIGTERM` kedua yang diterima runner. Tidak ada yang menandai hook `post-session` mid-run, jadi di host bare di mana proses init mengadopsi orphans, itu selesai sendiri, tetapi tanpa pengawasan: anggaran timeout-nya tidak lagi berlaku, dan penulisan ke pipa log tertutup dapat membunuhnya dengan `SIGPIPE`, jadi hook yang perlu bertahan dari forced exit di sana harus mengarahkan ulang output-nya sendiri ke file. Dalam resep container di halaman ini runner adalah PID 1 container dan keluarnya mengakhiri container, dan di bawah `KillMode=control-group` default systemd, kill cgroup-wide mencapai hook juga, seperti yang dijelaskan entri **Cgroup-wide kills**; di keduanya, perlakukan forced exit sebagai fatal untuk hook dan andalkan periode grace sebagai gantinya.
* **Sinyal process-group-wide**, seperti `kill -- -<pid>` dalam skrip wrapper, shell job control, atau watchdog group-wide: mencapai runner dan subprocess mid-`checkout`-hook, yang tetap group-attached dengan sengaja, tetapi bukan hook `post-session` mid-run atau anak sesi.
* **Cgroup-wide kills**, seperti `KillMode=control-group` default systemd atau `SIGKILL` yang dikirimkan Kubernetes ke seluruh container ketika `terminationGracePeriodSeconds` kedaluwarsa: mencapai segalanya, termasuk hook. Isolasi process-group tidak melindungi terhadap ini, itulah mengapa periode grace harus mencakup jalur drain penuh.
* **Timeout hook sendiri**: ketika hook melebihi `--post-session-hook-timeout-sec`, runner mengirim `SIGTERM` ke seluruh kelompok proses hook, kemudian `SIGKILL` dua detik kemudian, jadi worker yang hook fork, seperti tar, rsync, atau git, berakhir dengan shell wrapper alih-alih bertahan sebagai orphan. Pengawasan runner berakhir setelah stdio hook ditutup: worker yang mengarahkan ulang output-nya sendiri ke file dan bertahan melampaui tahap `SIGTERM` melampaui jangkauan runner.

Ketika drain dimulai, dan lagi pada forced exit, runner mencatat berapa banyak hook `post-session` yang masih berjalan, jadi Anda dapat membedakan drain yang tenang dari yang mid-snapshot.

<h2 id="keep-the-base-directory-and-capacity-identical-across-runners">
  Jaga direktori dasar dan kapasitas identik di seluruh runner
</h2>

Jika runner mati mid-sesi, server antri ulang sesi dan runner lain dalam lingkungan mengambilnya. Runner itu menurunkan jalur checkout dari `--base-dir` dan `--capacity` miliknya sendiri: `--capacity 1` checkout langsung di bawah `--base-dir`, dan `--capacity` di atas `1` menggunakan worktree per-sesi. Ketika runner dalam lingkungan yang sama menggunakan nilai berbeda untuk flag apa pun, direktori kerja sesi yang dilanjutkan berubah, dan jalur absolut yang agen catat sebelumnya, dalam edit, panggilan alat, atau catatan miliknya sendiri, menunjuk ke lokasi yang tidak lagi ada.

Gunakan `--base-dir` dan `--capacity` yang sama pada setiap runner dalam lingkungan, dan jangan gunakan nilai per-host seperti ID instans atau nama host.

Direktori dasar default ke `/workspace`, dengan pengecualian baris referensi [`--base-dir`](/docs/id/self-hosted-environments-reference#runner-cli-flags) catat. Runner memerlukan akses tulis ke sana. Pada startup, sebelum mendaftar, runner membuat direktori dan mengkonfirmasi dapat menulis ke sana, dan keluar dengan `cannot create or write to base directory` ketika tidak bisa. Runner yang dimulai sebagai root membuat `/workspace` default itu sendiri. Untuk runner non-root, buat direktori dan berikan kepemilikan pengguna runner sebelum memulai runner, atau arahkan `--base-dir` ke direktori yang pengguna itu sudah miliki.

<h2 id="reuse-a-pre-warmed-checkout">
  Gunakan kembali checkout yang telah dipanaskan sebelumnya
</h2>

Untuk repositori besar, klon dapat mendominasi startup sesi. Pada `--capacity 1` tanpa [`checkout` hook](/docs/id/self-hosted-environments-configuration#checkout), runner menyimpan satu klon kanonik per repositori di `<base-dir>/<repo-owner>/<repo>` dan menggunakannya kembali di seluruh sesi: itu mengambil ref yang diminta, melepaskan `HEAD`, dan reset keras ke sana, yang hampir instan ketika sedikit yang berubah. Untuk melewati klon dingin, sediakan klon dalam salah satu dari dua cara:

* **Klon dalam gambar**: bangun klon ke gambar runner Anda di jalur itu. Setiap kontainer segar kemudian dimulai dengan klon hangat tanpa menggunakan kembali disk.
* **Klon pada volume persisten**: pada runner yang Anda pre-lock ke akun satu pengguna dengan [`--lock-to-account`](/docs/id/self-hosted-environments-reference#runner-cli-flags), arahkan `--base-dir` ke volume persisten, jadi disk hanya pernah melayani akun itu. Runner yang pre-locked tidak pernah mengambil sesi saluran Claude Tag, jadi opsi ini tidak berlaku untuk runner yang melayani mereka.

Apa jalur penggunaan kembali lakukan dan tidak jamin:

* **Bentuk klon apa pun bekerja**: klon penuh, shallow, atau single-branch di jalur digunakan apa adanya. Runner tidak pernah melewatkan `--depth` ketika mengambil ke klon yang ada, jadi pre-warm penuh menyimpan riwayat penuhnya dan yang shallow tetap shallow. `CLAUDE_RUNNER_FETCH_DEPTH` (`full`, `0`, atau angka; default 50) mengontrol hanya klon dingin yang dibuat runner ketika tidak ada klon yang ada.
* **Perubahan terlacak reset, file tidak terlacak bertahan**: setiap sesi dimulai dari reset keras yang menghapus modifikasi sesi sebelumnya, tetapi runner tidak pernah menjalankan `git clean`, jadi file tidak terlacak dari sesi sebelumnya pemilik terkunci tetap di pohon.
* **Direktori per-sesi juga bertahan**: di samping checkout, runner membuat entri per-sesi di bawah `<base-dir>/_sessions/` untuk setiap sesi yang dijalankannya. Direktori konfigurasi Claude sesi menyimpan salinan lokal transkrip percakapan. Di sebelahnya duduk file yang diunggah sesi, ketika sesi memiliki apa pun. Direktori sesi duduk di sana juga: itu menyimpan worktree per-sesi apa pun dan checkout hook `checkout` sementara sesi berjalan, dan itu menyimpan apa pun yang Claude tulis di dalamnya.

  Secara default runner meninggalkan ini di tempat ketika sesi berakhir, jadi pada disk yang melampaui proses runner mereka menumpuk. Setiap sesi berjalan sebagai pengguna runner sendiri, jadi sesi kemudian apa pun yang disk layani dapat membacanya. Jika Anda menyimpan `--base-dir` persisten, ukuran volume untuk pertumbuhan itu. Hal yang sama berlaku untuk setup apa pun yang memulai ulang runner pada filesystem yang sama, termasuk resep [Docker Compose](#docker-compose).
* **Dengan `--remove-session-state`, direktori per-sesi tidak bertahan**: mulai runner dengan [`--remove-session-state`](/docs/id/self-hosted-environments-reference#runner-cli-flags) untuk memilikinya menghapus direktori per-sesi setiap sesi saat sesi berakhir. Penghapusan adalah best-effort: direktori tetap ketika runner dibunuh sebelum cleanup-nya berjalan. Klon kanonik dan file yang ditulis sesi di tempat lain di host, seperti direktori sementara, tetap terlepas.
* **Dengan proxy git, reset menjadi checkout**: dengan [`--use-anthropic-git-proxy`](#use-the-anthropic-git-proxy), runner membersihkan `.git/` klon sebelum setiap sesi, menyimpan object store, refs, dan shallow state tetapi menghapus index, jadi setiap sesi membayar checkout working-tree penuh daripada reset hampir instan; itu masih tidak pernah re-clone. Pre-warm submodule tidak didukung di bawah proxy.
* **Klon panjang tidak memerlukan workaround**: runner membatasi setiap operasi git dengan watchdog no-progress 120 detik dan hard cap 30 menit, bukan timeout flat, jadi klon dingin lambat yang terus melaporkan kemajuan selesai.

<h2 id="pin-the-version">
  Sematkan versi
</h2>

Proses Claude Code anak setiap sesi menjalankan biner runner itu sendiri, dan runner mematikan auto-update di dalam sesi yang dihasilkannya, jadi setiap sesi menjalankan versi yang Anda instal di host atau bangun ke gambar. Update tingkat host berlaku waktu berikutnya runner dimulai.

* **Untuk menyimpan armada pada satu versi**: bangun gambar dengan versi yang disematkan, atau pada host telanjang instal versi spesifik dan [nonaktifkan auto-update](/docs/id/setup#disable-auto-updates)
* **Untuk upgrade**: instal versi yang lebih baru atau bangun ulang gambar, kemudian mulai ulang runner
* **Plugin**: marketplace plugin tidak auto-update juga; atur `FORCE_AUTOUPDATE_PLUGINS=1` dalam lingkungan runner untuk membiarkan plugin auto-update sementara biner tetap disematkan

<h2 id="scale-the-fleet">
  Skala armada
</h2>

Orchestrator Anda memutuskan kapan menambah atau menghapus runner. Karena [kunci one-owner-per-runner](/docs/id/self-hosted-environments#runner-lifecycle), jumlah replika minimum adalah jumlah pengguna dan agen Claude Tag yang Anda harapkan aktif bersamaan; `--capacity` mengontrol paralelisme dalam sesi satu pemilik, bukan di seluruh pemilik.

Dua pendekatan penskalaan tersedia:

* **Armada tetap**: jalankan set replika runner statis dan skala pada [metrik Prometheus](/docs/id/self-hosted-environments-reference#prometheus-metrics) yang dilayani setiap runner
* **Runner on-demand**: jalankan subperintah `claude self-hosted-runner orchestrator`, yang menanyakan Anthropic untuk sesi yang antri tanpa runner tersedia dan memanggil hook `spawn-runner` Anda untuk boot satu per sesi. Lihat [On-demand runners](/docs/id/self-hosted-environments-configuration#on-demand-runners).

<h2 id="known-issues-and-limitations">
  Masalah dan batasan yang diketahui
</h2>

Berikut adalah batasan dalam rilis ini, dengan workaround di mana ada.

<h3 id="connector-traffic-leaves-your-network">
  Lalu lintas konektor meninggalkan jaringan Anda
</h3>

Anthropic memanggil alat konektor dari infrastruktur miliknya sendiri daripada dari runner Anda. Alat konektor adalah konektor claude.ai, seperti GitHub, Slack, dan Linear. Ketika Claude menggunakan konektor dalam sesi yang di-host sendiri, lalu lintas itu melewati `api.anthropic.com` daripada berasal dari dalam batas jaringan Anda.

Untuk menjaga konektor keluar dari sesi yang di-host sendiri, saring dengan [pengaturan kebijakan `allowedMcpServers` dan `deniedMcpServers`](/docs/id/managed-mcp#policy-based-control-with-allowlists-and-denylists). Claude Code menerapkan pengaturan ini ke konektor yang dikirimkan Anthropic serta ke server yang Anda seed dari host runner dan server yang ditambahkan pengguna, jadi jika Anda terapkan allowlist untuk server lain, Claude Code memblokir konektor yang dikirimkan juga. Untuk menjaga konektor tersedia bersama allowlist berbasis URL, tambahkan entri yang cocok dengan jalur proxy Anthropic untuk konektor yang dikirimkan:

* `https://api.anthropic.com/v2/ccr-sessions/*`
* `https://api.anthropic.com/v1/code/sessions/*`
* `https://api.anthropic.com/v1/code/mcp/*`

Jika lalu lintas alat harus tetap di dalam jaringan, jalankan alat setara sebagai server MCP lokal pada gambar runner. Lihat [MCP servers](/docs/id/self-hosted-environments-configuration#mcp-servers).

<h3 id="some-sessions-don’t-count-as-idle">
  Beberapa sesi tidak dihitung sebagai menganggur
</h3>

Sesi yang memegang tugas latar belakang yang tidak pernah selesai tidak dihitung sebagai menganggur, jadi `--release-idle-session-min` tidak akan melepaskan slot sesi itu. Sesi yang menunggu persetujuan yang diminta dari dalam panggilan alat yang berjalan juga tidak dihitung sebagai menganggur. Selalu atur `--kill-session-after-min` bersama sebagai backstop keras sehingga tidak ada sesi yang dapat memegang slot tanpa batas.

`--kill-session-after-min` adalah backstop untuk sesi yang melarikan diri. Pada runner pada v2.1.260 atau lebih baru, sesi yang mencapai batas tidak dihentikan begitu saja. Runner memberikan jendela grace, 15 menit secara default, yang dapat Anda ubah dengan [`SELF_HOSTED_RUNNER_MAX_LIFETIME_GRACE_MS`](/docs/id/self-hosted-environments-reference#environment-variable-only-settings):

* Jika sesi menunggu penggunanya, runner melepaskannya. Jika putarannya telah berakhir dan hanya memegang tugas latar belakang, runner menunggu hingga 60 detik untuk tugas-tugas itu selesai dan kemudian melepaskannya. Sesi dilanjutkan ketika penggunanya mengirim pesan berikutnya.
* Jika putaran masih berjalan, runner menunggu putaran selesai, atau untuk sesi berikutnya menunggu penggunanya, dan kemudian melepaskannya.
* Jika sesi masih di runner ketika jendela grace berakhir, runner menghentikannya, dan pekerjaan putaran yang berjalan hilang. Putaran menunggu persetujuan yang diminta dari dalam panggilan alat yang berjalan adalah satu cara sesi melampaui jendela.

Sesi yang dilepaskan dilanjutkan dari klon segar, jadi pekerjaan yang tidak didorong hilang bagaimanapun; lihat [Resumed sessions lose unpushed work](#additional-limitations). Sebelum v2.1.260, runner menghentikan setiap sesi pada batas, setelah menunggu paling banyak jendela grace untuk putaran yang berjalan selesai.

Atur flag di atas sesi terpanjang yang diharapkan, seperti `--kill-session-after-min 480` untuk 8 jam. Untuk membebaskan slot dari percakapan yang menjadi menganggur, gunakan `--release-idle-session-min`.

<h3 id="additional-limitations">
  Batasan tambahan
</h3>

* **Sesi yang dilanjutkan kehilangan pekerjaan yang tidak didorong**: ketika sesi dilepaskan atau runner-nya dimulai ulang, dan pengguna mengirim pesan lain, sesi dilanjutkan pada runner segar yang klon repositori lagi dari cabang awalnya, jadi pekerjaan yang tidak didorong sesi hilang. Atur [`--push-outcome-on-release`](/docs/id/self-hosted-environments-reference#runner-cli-flags) untuk memiliki runner membuat push best-effort dari cabang hasil sesi sebelum melepaskan, jadi sesi yang dilanjutkan dimulai dari komit tersebut; ini menyimpan pekerjaan yang berkomitmen, bukan working tree yang kotor. Sebelum mengaktifkannya, batasi siapa yang dapat mendorong ke ref `claude/*` pada remote sumber, misalnya dengan ruleset cabang: pada resume, runner mengambil cabang yang sebelumnya didorong tanpa memverifikasi siapa yang mendorongnya, jadi siapa pun dengan akses push ke ref tersebut dapat menempatkan konten ke dalam workspace yang dilanjutkan. Runner juga membuang konfigurasi per-sesi pada resume, berarti direktori konfigurasi Claude sesi dan status shell apa pun yang ditulis sesi; `--push-outcome-on-release` tidak mencakup itu.
* **Repositori pribadi tidak dapat ditambahkan mid-sesi**: repositori yang ditambahkan ke sesi setelah dimulai tidak diklon dengan kredensial pada runner yang di-host sendiri, jadi penambahan gagal. Pilih setiap repositori yang dibutuhkan sesi ketika Anda membuatnya.
* **Beberapa konektor tidak muncul dalam sesi yang di-host sendiri**: konektor yang belum Anda hubungkan dalam pengaturan claude.ai tidak terdaftar dalam sesi yang di-host sendiri, dan sesi tidak akan meminta Anda untuk menghubungkannya. Hubungkan dalam Pengaturan terlebih dahulu, kemudian mulai sesi segar. Menambahkan konektor ke sesi yang sudah berjalan juga tidak membuat alat-alatnya tersedia untuk Claude; mulai sesi segar untuk mengambil konektor yang baru ditambahkan.

<h3 id="report-an-issue">
  Laporkan masalah
</h3>

Untuk masalah dengan lingkungan yang di-host sendiri, hubungi tim akun Anthropic Anda.

<h2 id="troubleshooting">
  Troubleshooting
</h2>

Untuk diagnosis terpandu, jalankan subperintah doctor pada host runner. Subperintah doctor memulai sesi Claude Code interaktif dengan log dan status runner yang terlampir. Masuk dengan `claude auth login` pada host tersebut terlebih dahulu sehingga sesi dapat menanyakan lingkungan Anda, runner-nya, dan sesi-sesi yang antri. Tanpa masuk tersebut, misalnya ketika host melakukan autentikasi dengan kunci API, itu terbatas pada endpoint kesehatan lokal, metrik, dan log runner, dan itu membaca log hanya jika Anda memulai runner dengan `--log-file`.

```bash theme={null}
claude self-hosted-runner doctor
```

Masalah umum:

* **Runner tidak muncul di lingkungan**: konfirmasi bahwa host dapat menjangkau `api.anthropic.com` melalui HTTPS, rahasia lingkungan saat ini, dan jam host berada dalam lima menit dari waktu nyata; skew yang lebih besar menyebabkan autentikasi gagal. Log runner `[runner:fatal]` dengan alasan penolakan pada kegagalan auth.
* **Runner keluar saat startup dengan `cannot create or write to base directory`**: runner tidak dapat membuat atau menulis ke `--base-dir`, yang secara default adalah `/workspace`. Perbaiki kepemilikan direktori atau arahkan `--base-dir` ke jalur yang dapat ditulis, seperti dijelaskan dalam [Keep the base directory and capacity identical across runners](#keep-the-base-directory-and-capacity-identical-across-runners). Jika runner malah mencatat `[runner:fatal]` mengatakan pemeriksaan direktori dasar habis waktu, direktori berada pada mount NFS atau CSI yang tergantung. Periksa kesehatan mount daripada izin. Runner mencetak kedua kegagalan startup ini ke stderr sebelum membuka `--log-file`, jadi cari di terminal atau log kontainer platform Anda daripada file log. Sebelum v2.1.225, runner tidak memeriksa direktori dasar saat startup, dan misconfiguration ini gagal sesi setelah pickup sebagai gantinya.
* **Sesi tetap antri**: setiap runner online dapat dikunci ke pemilik yang berbeda. Periksa `claude_code_self_hosted_runner_locked_account` [metrik](/docs/id/self-hosted-environments-reference#prometheus-metrics) setiap runner atau bidang `locked_account` dari baris log `[runner:health]`-nya untuk melihat siapa yang memegangnya. Keduanya menunjukkan email pemilik hanya setelah runner telah dikeluarkan token sesi yang membawa klaim `act.email`, yang sesi agen Claude Tag tidak pernah lakukan. Tanpa klaim, runner tidak memancarkan seri `locked_account` dan mencatat `locked_account=yes`, yang memberi tahu Anda bahwa runner terkunci tetapi tidak ke pemilik mana. Tambahkan replika, atau tunggu runner yang ada untuk mengalirkan dan memulai ulang. Jika lingkungan menggunakan runner on-demand, periksa orchestrator sebagai gantinya; lihat [On-demand runners](/docs/id/self-hosted-environments-configuration#on-demand-runners).
* **Sesi gagal segera setelah pickup**: buka sesi di claude.ai/code untuk melihat kesalahan. Penyebab paling umum adalah [kredensial git](#configure-git) yang hilang dalam gambar runner dan alat build yang tidak diinstal. Direktori dasar yang tidak dapat ditulis menghentikan runner saat startup daripada gagal sesi. Lihat entri **Runner keluar saat startup dengan `cannot create or write to base directory`** dalam daftar ini.
* **Sesi tidak dapat menjangkau jaringan melalui proxy egress yang mengautentikasi**: ketika sumber yang Anda atur dengan [`--proxy-authorization-command` atau `--proxy-authorization-file`](#authenticate-to-an-egress-proxy) gagal, habis waktu setelah 30 detik, atau menghasilkan nilai kosong, runner menjawab koneksi itu `502 Bad Gateway` dan mencatat alasannya. Runner menyunting stderr perintah dalam log itu dan tidak pernah mencatat nilai header. Dengan `--proxy-authorization-command`, jalankan perintah sendiri pada host untuk mengonfirmasi bahwa itu mencetak seluruh nilai header pada stdout. Jika runner malah keluar saat startup dengan `could not start the proxy-authorization listener`, itu tidak dapat membuka pendengar loopback-nya.
* **Runner mencatat baris `Poll failed` yang berisi `rejecting the malformed poll response`**: runner menerima respons work-poll yang badan-nya bukan JSON yang diharapkan antrian, paling sering karena sesuatu antara runner dan `api.anthropic.com`, seperti proxy intersepsi atau portal captive, menjawab dengan halaman sendiri. Runner menolak respons, menghitung di bawah jenis `transport` dari [metrik](/docs/id/self-hosted-environments-reference#prometheus-metrics) `claude_code_self_hosted_runner_poll_errors_total`, dan mencoba ulang pada jadwal poll-gagal yang dijelaskan dalam [Session lifecycle](/docs/id/self-hosted-environments#session-lifecycle). Runner terus melayani sesi live-nya. Konfigurasikan proxy untuk melewatkan respons dari `api.anthropic.com` tanpa perubahan. Sebelum v2.1.246, runner membaca respons seperti itu sebagai antrian kerja kosong, yang dapat mengakhiri sesi live-nya atau membuatnya keluar.
* **Cabang sesi tidak lagi ada di remote**: untuk sumber git yang hanya dibaca sesi, runner melewati sumber itu dan melanjutkan pada sumber yang tersisa. Untuk sumber yang sesi dorong hasil ke, cabang yang dihapus, biasanya karena digabungkan dan dihapus otomatis, gagal sesi dengan kesalahan yang menamai repositori dan cabang dan meminta Anda untuk mengembalikan cabang dan mencoba ulang. Runner gagal sesi dengan kesalahan yang sama ketika melewati akan meninggalkannya tanpa repositori sama sekali. Sebelum v2.1.228, sesi seperti itu dimulai di direktori kosong.
* **Sesi dimulai tanpa salah satu repositorinya**: pada runner tanpa hook [`checkout`](/docs/id/self-hosted-environments-configuration#checkout), host git dapat menolak pemeriksaan akses runner untuk repositori yang hanya dibaca sesi. Runner kemudian melewati repositori itu, mencatat baris `[runner:warn] could not access context source` yang menamai penolakan, dan memulai sesi pada sumber yang tersisa.

  Runner hanya melewati penolakan yang jelas: host menjawab bahwa repositori tidak ditemukan, git tidak menemukan kredensial untuk host, atau autentikasi gagal. Kegagalan jaringan, timeout, atau HTTP `403` masih gagal memulai sesi, begitu juga penolakan untuk repositori yang sesi dorong hasil ke. Runner masih gagal sesi yang melewati akan meninggalkannya tanpa repositori sama sekali. Dengan [`--use-anthropic-git-proxy`](#use-the-anthropic-git-proxy), runner hanya melewati repositori yang proxy git sendiri tolak.

  Pemeriksaan akses berjalan lagi setiap kali sesi dimulai pada runner, jadi setelah identitas git runner memiliki akses baca, awal berikutnya mengklona repositori. Sebelum v2.1.274, masing-masing penolakan ini gagal memulai sesi.
* **Sesi membutuhkan waktu berapa menit untuk dimulai**: klon awal biasanya mendominasi. Tonton [metrik](/docs/id/self-hosted-environments-reference#prometheus-metrics) `claude_code_self_hosted_runner_session_init_duration_seconds` untuk mengonfirmasi, dan potong klon dengan [pre-warmed checkout](#reuse-a-pre-warmed-checkout) atau `CLAUDE_RUNNER_FETCH_DEPTH` yang lebih kecil.
* **Turns gagal dengan 401**: setiap sesi mengautentikasi panggilan model dengan [`CLAUDE_CODE_OAUTH_TOKEN`](/docs/id/self-hosted-environments-configuration#wrapper-scripts) jangka pendek yang runner ambil dari Anthropic dan putar di atas stdin sesi. Ketika turn berakhir dengan 401 atau 403 dari API model, runner mengambil token segar dan meneruskannya ke sesi. Turn yang gagal tidak dicoba ulang.

  Ketika pengambilan gagal, runner mencatat baris `inference_token refresh failed` yang mengatakan kapan itu akan mencoba ulang, dan itu terus mencoba ulang selama sesi berjalan.

  Jika setiap panggilan mulai gagal sekitar 30 menit ke dalam sesi, skrip wrapper mungkin telah memutuskan stdin sesi, jadi rotasi token tidak dapat menjangkaunya; lihat [Keep stdin and file descriptor 3 attached](/docs/id/self-hosted-environments-configuration#keep-stdin-and-file-descriptor-3-attached).

  Sebelum v2.1.274, runner berhenti mencoba ulang pengambilan yang gagal setelah beberapa upaya dan menunggu yang dijadwalkan berikutnya. Turn yang gagal tidak memicu pengambilan, jadi setiap turn gagal dengan 401 sampai pengambilan yang dijadwalkan berikutnya.
* **Pod dibunuh di tengah-drain**: naikkan `terminationGracePeriodSeconds` ke setidaknya nilai yang dicatat runner saat startup. Lihat [Shutdown timing](#shutdown-timing).

Setelah logging diinisialisasi, runner menulis log lifecycle-nya, termasuk baris `[runner:fatal]`, ke stdout, dan output debug ke stderr, semuanya sebagai baris teks biasa daripada JSON. Kegagalan startup yang dijelaskan dalam entri troubleshooting di atas mencetak ke stderr sebelum titik itu. Tangkap kedua aliran dengan `--log-file`, yang juga memungkinkan `self-hosted-runner doctor` untuk mengekornya, atau dengan pengumpulan log platform Anda.

Setiap proses anak sesi menulis log debug terpisah. Pada kegagalan runner menampilkan ekor log bersama sesi di claude.ai/code. Kecuali Anda memulai runner dengan [`--remove-session-state`](/docs/id/self-hosted-environments-reference#runner-cli-flags), itu juga menyimpan log sesi yang gagal di disk dan mencetak jalurnya dalam log runner.

<h2 id="what’s-next">
  Apa selanjutnya
</h2>

* [Sesuaikan sesi](/docs/id/self-hosted-environments-configuration): script wrapper, lifecycle hook, runner on-demand, server MCP, dan izin
* [Uji end to end](/docs/id/self-hosted-environments-testing): verifikasi gambar runner baru dari CI sebelum mempromosikannya
* [Referensi](/docs/id/self-hosted-environments-reference): setiap flag CLI, variabel lingkungan, dan metrik
