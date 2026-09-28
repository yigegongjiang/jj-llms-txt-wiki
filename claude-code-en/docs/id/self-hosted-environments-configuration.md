> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Sesuaikan sesi di lingkungan yang di-host sendiri

> Sesuaikan sesi lingkungan yang di-host sendiri dengan skrip wrapper untuk kredensial per-sesi, hook siklus hidup, dan pemijahan runner sesuai permintaan.

<Note>
  Lingkungan yang di-host sendiri berada dalam beta publik pada paket Team dan Enterprise; seorang [Owner](/docs/id/cloud-environments#organization-shared-environments) mengaktifkannya dengan mengaktifkan **Allow self-hosted environments** di [halaman admin **Cloud environments**](https://claude.ai/admin-settings/cloud-environments). Halaman ini mengasumsikan runner yang berfungsi; lihat [quickstart](/docs/id/self-hosted-environments-quickstart) untuk setup dan [Deploy to production](/docs/id/self-hosted-environments-deploy) untuk resep fleet.
</Note>

Sebuah [lingkungan yang di-host sendiri](/docs/id/self-hosted-environments) menjalankan Claude Code [sesi cloud](/docs/id/claude-code-on-the-web) di infrastruktur Anda sendiri, dieksekusi oleh proses runner yang Anda deploy. Tanpa konfigurasi, runner itu mengkloning repositori sesi, memijahkan Claude Code, dan membersihkan. Halaman ini untuk insinyur platform yang mengoperasikan runner: ini mencakup titik ekstensi untuk ketika default itu tidak sesuai, dari penyediaan kredensial per-sesi hingga mengganti checkout sepenuhnya. Wrapper dan hook berjalan sebagai file yang dapat dieksekusi di host runner, yang merupakan Linux atau macOS, dan contoh di halaman ini mengasumsikan shell POSIX.

Beberapa variabel lingkungan hook di halaman ini masih menggunakan `pool`, seperti `CLAUDE_RUNNER_POOL_ID`; nama flag CLI dan variabel env menggunakan `environment`, seperti `--environment-secret-file`.

<h2 id="wrapper-scripts">
  Skrip wrapper
</h2>

Gunakan skrip wrapper ketika setiap sesi memerlukan setup yang tidak dapat dilakukan runner sendiri: penyediaan kredensial jangka pendek yang dibatasi untuk pembuat sesi, mengekspor rahasia khusus lingkungan, menyiapkan toolchain bahasa, atau menerapkan batas sumber daya di sekitar proses anak. Runner memulai wrapper Anda sebagai pengganti biner Claude Code, sekali per sesi. Akhiri wrapper dengan `exec`-ing ke `$CLAUDE_RUNNER_CLAUDE_BIN`, biner runner sendiri, sehingga sinyal dan kode keluar menyebar dengan benar.

Arahkan `--exec-path`, atau `SELF_HOSTED_RUNNER_EXEC_PATH`, ke wrapper ketika Anda memulai runner:

```bash theme={null}
claude self-hosted-runner --environment-secret-file /etc/claude/environment-secret --exec-path /etc/claude/session-wrapper.sh
```

Runner menetapkan yang berikut di lingkungan wrapper:

| Variabel                            | Deskripsi                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| :---------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `CLAUDE_CODE_SESSION_ACCESS_TOKEN`  | JWT sesi, diawali dengan `sk-ant-cc-`. Klaim `act` mengidentifikasi pembuat sesi, dengan email pembuat dan subjek penyedia identitas upstream ketika permukaan pembuatan merekamnya. Nilainya adalah token pada waktu pemijahan; penyegaran tiba melalui stdin anak, jadi wrapper hanya melihat nilai awal. Lihat [Verify session identity](/docs/id/self-hosted-environments-identity).                                                                                                                                                                                                                                                              |
| `CCR_SESSION_ACCOUNT_EMAIL`         | Email pembuat sesi, pra-ekstrak oleh runner dari klaim `act.email` token tanpa verifikasi tanda tangan. Cocok untuk pelabelan, seperti trailer commit. Ketika email membuka akses penyediaan kredensial, verifikasi token dan baca klaim darinya; lihat [Provision credentials scoped to the session creator](#provision-credentials-scoped-to-the-session-creator). Tidak diatur ketika token tidak membawa email pembuat. Perlakukan sebagai informasi yang dapat diidentifikasi secara pribadi.                                                                                                                                               |
| `CLAUDE_RUNNER_CLIENT_PLATFORM`     | Permukaan klien yang membuat sesi, seperti `web_claude_ai`, `desktop_app`, `ios`, `claude_code_cli`, atau `scheduled_trigger`. Anthropic merekam nilainya sekali saat pembuatan sesi, jadi wrapper dan setiap hook siklus hidup melihat nilai yang sama. Gunakan untuk analitik adopsi dan pelabelan saja, bukan sebagai sinyal otorisasi. Tidak diatur ketika sesi tidak memiliki permukaan yang tercatat atau dikenali, jadi referensikan sebagai `${CLAUDE_RUNNER_CLIENT_PLATFORM:-}` di bawah `set -u`. Memerlukan Claude Code v2.1.229 atau lebih baru.                                                                                     |
| `CLAUDE_RUNNER_CLAUDE_BIN`          | Jalur absolut ke biner Claude Code runner sendiri. Akhiri wrapper Anda dengan `exec "$CLAUDE_RUNNER_CLAUDE_BIN" "$@"` untuk menyerahkan ke biner yang disematkan tanpa hardcoding jalur instalasi.                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `CLAUDE_CODE_REMOTE_SESSION_ID`     | ID sesi dalam bentuk `cse_...` yang ditandai. Ini adalah sesi yang sama yang dilihat [lifecycle hooks](#lifecycle-hooks) sebagai `CLAUDE_RUNNER_SESSION_ID` dalam bentuk `session_...`; variabel UUID cocok di kedua sisi, dan mengganti awalan `cse_` dengan `session_` menghasilkan ID yang ditampilkan di URL sesi.                                                                                                                                                                                                                                                                                                                           |
| `CLAUDE_CODE_REMOTE_SESSION_UUID`   | ID sesi yang sama dalam bentuk UUID kanonik, untuk sistem yang menggunakan UUID sebagai kunci.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `CLAUDE_SESSION_INGRESS_TOKEN_FILE` | Jalur absolut ke file per-sesi yang menyimpan JWT sesi saat ini, tetap segar di seluruh penyegaran token. Subproses shell membacanya untuk header `Authorization` mereka saat mengunduh lampiran yang ditambahkan pengguna ke sesi. `exec` menyimpan variabel secara otomatis; wrapper yang membangun kembali lingkungan anak harus membawa variabel, atau unduhan lampiran berhenti bekerja diam-diam.                                                                                                                                                                                                                                          |
| `CLAUDE_CONFIG_DIR`                 | Direktori konfigurasi Claude per-sesi, ditulis saat awal sesi dari snapshot konfigurasi host runner yang ditangkap runner saat startup; lihat [Permissions and tool approval](#permissions-and-tool-approval). Penulisan di sini terisolasi ke sesi ini. Direktori tetap di bawah `<base-dir>/_sessions/` setelah sesi berakhir kecuali Anda memulai runner dengan [`--remove-session-state`](/docs/id/self-hosted-environments-reference#runner-cli-flags); lihat [Reuse a pre-warmed checkout](/docs/id/self-hosted-environments-deploy#reuse-a-pre-warmed-checkout).                                                                                    |
| `ANTHROPIC_BASE_URL`                | URL dasar API yang akan digunakan anak, dikirimkan oleh bidang kontrol per sesi dan biasanya `https://api.anthropic.com`. Jangan timpa: kredensial inferensi sesi adalah token OAuth yang dikeluarkan Anthropic yang tidak diterima penyedia lain, jadi inferensi di lingkungan yang di-host sendiri tidak dapat dialihkan ke tempat lain.                                                                                                                                                                                                                                                                                                       |
| `CLAUDE_CODE_OAUTH_TOKEN`           | Token akses OAuth jangka pendek yang digunakan anak untuk inferensi model, dibatasi untuk inferensi model dan unggahan file saja, dengan masa pakai sekitar 30 menit. Runner mencetak ulang sebelum kedaluwarsa dan mengirimkan rotasi melalui stdin anak, jadi wrapper yang tidak [menjaga stdin terlampir](#keep-stdin-and-file-descriptor-3-attached) hanya melihat nilai awal. Jangan andalkan daftar IP organisasi Anda untuk membatasi penggunaan token ini: perlakukan sebagai kredensial pembawa yang tetap dapat digunakan selama kira-kira 30 menit jika bocor, dan jangan catat, tulis ke disk, atau teruskan di luar kontainer sesi. |

Wrapper juga mewarisi sisa lingkungan anak yang dikelola, termasuk variabel lingkungan yang disediakan server. `exec` menyebarkan semuanya secara otomatis; jika wrapper Anda memijahkan anak dengan cara lain, teruskan lingkungan lengkap.

<h3 id="keep-stdin-and-file-descriptor-3-attached">
  Jaga stdin dan file descriptor 3 terlampir
</h3>

Stdin anak adalah saluran kontrol runner. Rotasi token dan sinyal akhir sesi tiba di atasnya. Runner juga membuka pipa pada file descriptor 3 dan membaca sinyal aktivitas anak darinya untuk mendorong timeout idle dan startup. `exec "$CLAUDE_RUNNER_CLAUDE_BIN" "$@"` biasa menyimpan keduanya secara otomatis.

Jika wrapper Anda menempatkan anak di latar belakang dengan `&` telanjang, itu memutus stdin anak: sesi terlihat sehat sampai masa pakai token OAuth awal sekitar 30 menit berakhir, kemudian setiap panggilan API gagal dengan `401 authentication_error`. Jika wrapper Anda harus menempatkan anak di latar belakang, misalnya untuk menjaga perangkap pembongkaran tetap hidup, simpan stdin pada file descriptor 4 atau lebih tinggi dan lampirkan kembali secara eksplisit:

```bash theme={null}
exec 4<&0
"$CLAUDE_RUNNER_CLAUDE_BIN" "$@" <&4 4<&- &
CHILD=$!
trap 'teardown' EXIT
wait "$CHILD"
```

Jangan tutup atau gunakan kembali file descriptor 3 di wrapper. Mengalihkan stdout dan stderr anak tidak apa-apa.

<h3 id="provision-credentials-scoped-to-the-session-creator">
  Sediakan kredensial yang dibatasi untuk pembuat sesi
</h3>

Gunakan subperintah `decode-token` untuk membaca klaim dari JWT sesi. Ini membaca token dari argumen, dari `CLAUDE_CODE_SESSION_ACCESS_TOKEN`, atau dari stdin, dalam urutan itu; lihat [Verify the token inside the session](/docs/id/self-hosted-environments-identity#verify-the-token-inside-the-session) untuk apa yang diperiksa. Contoh di bawah ini mendekode identitas pembuat, menukarnya dengan kredensial AWS jangka pendek, dan exec ke Claude Code:

```bash theme={null}
#!/bin/bash
# Kunci pada ID pengguna Anthropic yang stabil dan memerlukan pembuat manusia.
CREATOR_SUB=$("$CLAUDE_RUNNER_CLAUDE_BIN" self-hosted-runner decode-token \
  | jq -re '.act.sub // "" | select(startswith("user:"))') \
  || { echo "decode-token: verification failed or no human creator" >&2; exit 1; }

creds=$(your-sts-helper assume-role --subject "$CREATOR_SUB") \
  || { echo "credential exchange failed" >&2; exit 1; }
eval "$creds"

exec "$CLAUDE_RUNNER_CLAUDE_BIN" "$@"
```

Gunakan `jq -re` daripada `jq -r` ketika klaim yang diekstrak membuka keputusan auth, sehingga klaim yang hilang keluar bukan nol daripada melewatkan string literal `null` ke hilir. Sesi yang dibuat oleh identitas layanan organisasi, seperti sesi bot dan agen, membawa subjek `agent:` daripada `user:`, jadi contoh ini menolak mereka; jika lingkungan Anda melayani sesi tersebut, putuskan secara eksplisit apakah wrapper kembali ke kredensial default untuk mereka daripada keluar. Ketika pertukaran kredensial Anda memerlukan subjek SSO atau email, baca `.act.attested_by.sub` atau `.act.email` dan tangani ketidakhadiran mereka: token membawanya hanya ketika permukaan pembuatan merekamnya, dan [sesi yang dikirim CLI](/docs/id/self-hosted-environments-testing#run-the-test-loop) dapat kekurangan keduanya. Untuk referensi klaim lengkap dan verifikasi dari layanan di luar runner, lihat [Verify session identity](/docs/id/self-hosted-environments-identity).

<h2 id="lifecycle-hooks">
  Lifecycle hooks
</h2>

Lifecycle hooks mengganti tahap pipeline per-sesi runner dengan skrip Anda sendiri. Arahkan runner ke direktori hook dengan `--hooks-dir <path>`, atau `SELF_HOSTED_RUNNER_HOOKS_DIR`. Runner mencari file yang dapat dieksekusi dengan nama yang terkenal; hook apa pun yang tidak ada jatuh ke perilaku bawaan, jadi Anda hanya menulis yang Anda butuhkan. Hook berjalan dengan hak istimewa runner sendiri, dan anak sesi berbagi UID itu, jadi pasang direktori hook hanya-baca, atau panggang ke dalam gambar, sehingga kode sesi tidak dapat memodifikasinya; lihat [bagian hardening](/docs/id/self-hosted-environments-deploy#harden-your-deployment).

Hook ini berbeda dari [Claude Code hooks](/docs/id/hooks), yang berjalan di dalam sesi; lifecycle hooks berjalan di runner, di sekitar sesi.

<h3 id="checkout">
  checkout
</h3>

Berjalan sekali per repositori, sebagai pengganti klon dan fetch bawaan runner. Gunakan hook untuk mengkloning dari cermin read-through, seed pohon kerja dari arsip, atau menerapkan auth git per-sesi. Runner menetapkan:

| Variabel                           | Deskripsi                                                                                                                                                             |
| :--------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `CLAUDE_RUNNER_REPO_URL`           | URL repositori untuk mengkloning, setelah `--git-host-rewrite` dan `--git-ssh-rewrite` apa pun telah diterapkan                                                       |
| `CLAUDE_RUNNER_REPO_REF`           | Revisi untuk checkout: cabang, tag, atau commit SHA seperti yang diminta sesi. Kosong berarti cabang default repositori.                                              |
| `CLAUDE_RUNNER_CHECKOUT_PATH`      | Jalur absolut di mana pohon kerja harus ditinggalkan                                                                                                                  |
| `CLAUDE_RUNNER_SESSION_ID`         | ID sesi dalam bentuk `session_...` yang ditandai, untuk logging dan korelasi                                                                                          |
| `CLAUDE_RUNNER_SESSION_UUID`       | ID sesi yang sama dalam bentuk UUID kanonik                                                                                                                           |
| `CLAUDE_RUNNER_API_BASE_URL`       | URL dasar API Anthropic untuk panggilan yang dibatasi sesi                                                                                                            |
| `CLAUDE_RUNNER_CLIENT_PLATFORM`    | Permukaan klien yang membuat sesi, seperti `web_claude_ai`, `desktop_app`, atau `ios`. Tidak diatur ketika sesi tidak memiliki permukaan yang tercatat atau dikenali. |
| `CLAUDE_CODE_SESSION_ACCESS_TOKEN` | Token akses sesi, untuk panggilan API yang dibatasi sesi                                                                                                              |

Skrip harus meninggalkan pohon kerja di `CLAUDE_RUNNER_CHECKOUT_PATH` yang diperiksa pada revisi yang diminta. Detached HEAD tidak apa-apa; runner membuat cabang kerja sesi di atasnya. Runner memverifikasi jalur berisi `.git` sesudahnya; jika hook Anda mewujudkan sumber non-git seperti Perforce atau tarball yang dibuka, atur `CLAUDE_RUNNER_SKIP_GIT_VERIFY=1` di lingkungan runner untuk melewati pemeriksaan itu. Alur berbasis Git seperti pembuatan cabang kerja dan hasil push memerlukan checkout git, jadi ekspor hasil dari pohon non-git dengan hook [`post-session`](#post-session).

Runner tidak melewatkan kredensial git ke hook. Sebaliknya, cetak kredensial klon per-sesi dari identitas sesi: verifikasi `CLAUDE_CODE_SESSION_ACCESS_TOKEN` dengan perpustakaan JWT standar terhadap titik akhir JWKS di bawah `CLAUDE_RUNNER_API_BASE_URL`, seperti yang dijelaskan dalam [Verify the token from your service](/docs/id/self-hosted-environments-identity#verify-the-token-from-your-service), kemudian buat layanan kredensial Anda mengeluarkan kredensial klon jangka pendek untuk identitas dalam klaim `act` token. `CLAUDE_RUNNER_CLAUDE_BIN` tidak diatur di lingkungan checkout-hook, jadi subperintah `decode-token` tidak tersedia di sini. Kembali ke apa pun autentikasi git yang sudah dimiliki host, seperti agen SSH, pembantu kredensial, atau `.netrc`, juga merupakan pilihan.

Ketika hook keluar bukan nol, atau keluar 0 tanpa meninggalkan checkout yang dapat digunakan di belakang, apa yang dilakukan runner tergantung pada repositori:

* **Repositori yang sesi dorong hasil ke**: runner gagal sesi, dan pada keluar bukan nol permukaan ekor stderr skrip ke pengguna.
* **Repositori yang hanya dibaca sesi**, seperti repositori yang ditambahkan ke sesi yang berjalan: runner mencatat baris `[runner:warn]` dengan detail kegagalan, memposting langkah `Skipped` ke sesi, menghapus apa pun yang ditinggalkan hook di jalur checkout, dan melanjutkan dengan repositori yang tersisa. Ketika runner tidak dapat menghapus jalur segera, itu mencoba penghapusan lagi saat akhir sesi. Jika melewati meninggalkan sesi tanpa repositori sama sekali, runner gagal sesi pula.

Sebelum v2.1.228, runner gagal sesi pada kegagalan hook untuk repositori apa pun, jadi repositori hanya-baca yang tidak dapat dilayani hook gagal sesi lagi pada setiap runner segar yang dilanjutkan sesi.

Runner menghapus jalur checkout setelah sesi berakhir.

<h3 id="post-session">
  post-session
</h3>

Berjalan sekali per sesi, setelah anak Claude Code keluar dan sebelum runner merobohkan ruang kerja. Hook ini adalah satu-satunya kesempatan Anda untuk menyimpan pekerjaan yang tidak dikomit: pada `--capacity` di atas satu, runner menghapus worktree per-sesi tepat setelah hook kembali, dan pada `--capacity 1` [klon kanonik](/docs/id/self-hosted-environments-deploy#reuse-a-pre-warmed-checkout) yang digunakan kembali di-hard-reset ketika sesi berikutnya dimulai, jadi perubahan terlacak yang tidak dikomit tidak bertahan di jalur mana pun. Penggunaan khas adalah mendorong cabang snapshot perubahan yang tidak dikomit, mengarsipkan log, atau memancarkan acara akhir sesi ke sistem Anda sendiri.

Hook menyala pada setiap akhir sesi di mana proses anak dipijahkan, apa pun penyebabnya; nilai `CLAUDE_RUNNER_EXIT_REASON` di bawah menghitung kasus. Itu tidak dapat menyala ketika runner berhenti tiba-tiba, seperti preemption VM atau kehilangan daya; jika Anda memerlukan jaminan terhadap penghentian tiba-tiba, snapshot secara berkala dari dalam sesi dengan hook Claude Code `PostToolUse` sebagai gantinya. Runner menetapkan:

| Variabel                           | Deskripsi                                                                                                                                                                                                              |
| :--------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `CLAUDE_RUNNER_SESSION_ID`         | ID sesi dalam bentuk `session_...` yang ditandai                                                                                                                                                                       |
| `CLAUDE_RUNNER_SESSION_UUID`       | ID sesi yang sama dalam bentuk UUID kanonik                                                                                                                                                                            |
| `CLAUDE_RUNNER_EXIT_REASON`        | Bagaimana sesi berakhir; lihat nilai di bawah tabel                                                                                                                                                                    |
| `CLAUDE_RUNNER_WORKSPACE_PATHS`    | Jalur absolut yang dipisahkan titik dua dari pohon kerja sesi. Kosong untuk sesi tanpa repo.                                                                                                                           |
| `CLAUDE_RUNNER_DEBUG_LOG_PATH`     | Jalur ke log debug sesi, masih di disk saat hook berjalan                                                                                                                                                              |
| `CLAUDE_RUNNER_API_BASE_URL`       | URL dasar API Anthropic untuk panggilan yang dibatasi sesi                                                                                                                                                             |
| `CLAUDE_RUNNER_CLIENT_PLATFORM`    | Permukaan klien yang membuat sesi, seperti `web_claude_ai`, `desktop_app`, atau `ios`. Tidak diatur ketika sesi tidak memiliki permukaan yang tercatat atau dikenali. Memerlukan Claude Code v2.1.229 atau lebih baru. |
| `CLAUDE_CODE_SESSION_ACCESS_TOKEN` | Token akses sesi, untuk panggilan API yang dibatasi sesi                                                                                                                                                               |

`CLAUDE_RUNNER_EXIT_REASON` mengambil salah satu dari empat nilai:

* `completed`: sesi berakhir dengan bersih. Proses Claude Code keluar secara normal, atau sesi diarsipkan atau dihapus saat masih berjalan.
* `failed`: proses Claude Code mogok, atau setup gagal setelah dimulai.
* `interrupted`: runner menghentikan sesi. Itu merilis sesi untuk membebaskan slot, sesi timeout saat startup, server memindahkan sesi dari runner ini, runner sedang mengalirkan, atau sesi melampaui batas [`--kill-session-after-min`](/docs/id/self-hosted-environments-reference#runner-cli-flags) nya.
* `abandoned`: dicadangkan untuk sesi yang diklaim runner lain. Hook saat ini tidak menyala dalam kasus itu.

[Penghitung siklus hidup sesi](/docs/id/self-hosted-environments-reference#session-lifecycle-counter-semantics) menghitung rilis, timeout startup, dan perpindahan server sebagai `completed` daripada `interrupted`, karena runner menyerahkan slot kembali dengan bersih. Harapkan perbedaan itu jika Anda membandingkan penerimaan hook dengan penghitung.

Status keluar hook tidak pernah mempengaruhi hasil sesi; kegagalan dicatat dan diabaikan. Runner menunggu hingga `--post-session-hook-timeout-sec`, 60 detik secara default, pada setiap akhir sesi termasuk shutdown runner. Contoh ini menyimpan pekerjaan yang tidak dikomit ke cabang penyelamatan:

```bash theme={null}
#!/usr/bin/env bash
set -u
IFS=':'
# Konfigurasi pin yang mungkin ditanam sesi di .git/config checkout:
# -c overrides mengalahkan pengaturan lokal repo, memblokir fsmonitor yang ditulis sesi,
# hook-path, dan konfigurasi gpg-program dari mengeksekusi kode dengan hak istimewa hook.
# Repo-local credential.helper, core.sshCommand, dan pushurl
# masih berlaku; jika hook menyimpan kredensial yang tidak dimiliki sesi, pin
# push URL dan helper juga (lihat catatan di bawah skrip).
g() { git -c core.fsmonitor=false -c core.hooksPath=/dev/null \
        -c commit.gpgsign=false "$@"; }
for ws in $CLAUDE_RUNNER_WORKSPACE_PATHS; do
  cd "$ws" 2>/dev/null || continue
  [ -z "$(g status --porcelain 2>/dev/null)" ] && continue
  g add -A
  g commit -q -m "runner snapshot: $CLAUDE_RUNNER_SESSION_ID ($CLAUDE_RUNNER_EXIT_REASON)" || continue
  g push -q origin "HEAD:refs/heads/rescue/$CLAUDE_RUNNER_SESSION_ID" || true
done
```

Hook mendorong dengan kredensial git apa pun yang tersedia di lingkungannya sendiri di host runner. Di bawah [postur tanpa-kredensial-dalam-gambar](/docs/id/self-hosted-environments-deploy#configure-git), termasuk ketika klon bawaan melewati proxy git Anthropic, tidak ada, jadi cetak kredensial push jangka pendek di dalam hook sebelum mendorong: tukarkan token sesi yang diterima hook di `CLAUDE_CODE_SESSION_ACCESS_TOKEN` dengan layanan token Anda sendiri, memverifikasinya seperti [Verify session identity](/docs/id/self-hosted-environments-identity) menjelaskan. Ketika hook menyimpan kredensial yang tidak dimiliki sesi, juga pin di mana itu mendorong: ganti `origin` dengan URL yang disediakan operator dan lewatkan `-c credential.helper=` plus pembantu Anda sendiri, sehingga konfigurasi lokal repo yang ditulis sesi tidak dapat mengalihkan push yang dikreditkan.

<h4 id="hook-timing-when-the-runner-releases-a-session">
  Hook timing when the runner releases a session
</h4>

Sesi yang dirilis dapat dilanjutkan di runner lain. Di runner pada v2.1.236 atau lebih baru, apa yang dilakukan sesi saat rilis memutuskan apakah itu dapat dilanjutkan di runner lain sebelum hook ini selesai:

* **Idle setelah giliran, atau timeout saat startup**: runner menghentikan anak dan menjalankan hook ini hingga selesai. Hanya kemudian itu merilis sesi. Pesan pengguna yang dikirim saat hook berjalan tidak dapat melanjutkan sesi di runner lain sebelum hook selesai.
* **Menunggu pengguna menjawab prompt, seperti prompt izin**: runner merilis sesi terlebih dahulu, kemudian menjalankan hook ini. Pesan pengguna yang dikirim saat hook berjalan dapat melanjutkan sesi di runner lain sebelum hook selesai.

Ini berlaku setiap kali runner merilis sesi: pada timeout idle, pada waktu [`--retire-at`](/docs/id/self-hosted-environments-reference#runner-cli-flags), dan, di runner pada v2.1.260 atau lebih baru, pada batas [`--kill-session-after-min`](/docs/id/self-hosted-environments-reference#runner-cli-flags) sesi. Sesi yang giliran telah berakhir dan yang hanya menyimpan tugas latar belakang dihitung sebagai idle di sini. Sebelum v2.1.236, runner merilis sesi terlebih dahulu kemudian menjalankan hook ini di kedua kasus.

Selama drain `SIGTERM`, runner menyimpan sewa sesi sampai hook selesai; lihat [Shutdown timing](/docs/id/self-hosted-environments-deploy#shutdown-timing).

<h3 id="command">
  command
</h3>

Berjalan sekali per sesi setelah checkout, sebagai pengganti pemijahan anak bawaan. Hook menerima lingkungan yang sama dengan [skrip wrapper](#wrapper-scripts) dan harus `exec` ke `"$CLAUDE_RUNNER_CLAUDE_BIN"` dengan cara yang sama. Gunakan hook `command` untuk menyimpan semua kustomisasi dalam satu direktori hooks; gunakan `--exec-path` ketika wrapper tinggal di tempat lain. Jika `--exec-path` juga diatur, flag mengambil prioritas dan hook `command` diabaikan.

Selalu `exec` biner runner sendiri daripada `claude` yang diselesaikan PATH; jika tidak, Anda mengalahkan [version pinning](/docs/id/self-hosted-environments-deploy#pin-the-version).

<h2 id="on-demand-runners">
  Runner on-demand
</h2>

Alih-alih menjalankan fleet tetap, Anda dapat boot satu runner per sesi. Orchestrator adalah subperintah terpisah yang stateless yang menanyai Anthropic untuk permintaan pemijahan, satu per sesi yang antri tanpa runner tersedia, dan menjalankan hook `spawn-runner` Anda untuk masing-masing. Hook Anda mengirimkan beban kerja ke platform Anda: Kubernetes Job, instans EC2, dispatch Nomad.

Runner on-demand meningkatkan kebersihan kredensial. Pada fleet tetap, rahasia lingkungan tinggal di setiap host runner, yang merupakan host yang sama yang menjalankan sesi pengguna. Dengan orchestrator, rahasia lingkungan tetap hanya di host orchestrator, yang tidak pernah menjalankan kode pengguna; setiap runner yang dipijahkan menerima pesanan kerja sekali pakai yang mendaftarkan tepat satu runner dan kemudian kedaluwarsa.

Untuk memulai orchestrator, lewatkan rahasia lingkungan dan direktori hooks yang berisi skrip `spawn-runner` yang dapat dieksekusi:

```bash theme={null}
claude self-hosted-runner orchestrator \
  --environment-secret-file /etc/claude/environment-secret \
  --hooks-dir /etc/claude/hooks
```

Orchestrator tidak menyimpan status antara polling, jadi Anda dapat menjalankan dua atau lebih replika terhadap lingkungan yang sama untuk ketersediaan. Setiap permintaan pemijahan diklaim server-side oleh tepat satu replika. Semua replika harus menggunakan nilai `--expected-spawn-seconds` yang sama; lihat [kontrak hook](#the-spawn-runner-hook).

<h3 id="the-spawn-runner-hook">
  Hook spawn-runner
</h3>

Orchestrator menjalankan `${hooks-dir}/spawn-runner` sekali per permintaan pemijahan. Hook harus mengirimkan pekerjaan secara asinkron, tanpa menunggu runner boot, dan kembali dalam `--hook-timeout`, 60 detik secara default. Hook menerima:

| Variabel                              | Deskripsi                                                                                                                                                                                                                                                                                                                                            |
| :------------------------------------ | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `CLAUDE_RUNNER_WORK_ORDER_FILE`       | Jalur ke file temp yang berisi JWT pesanan kerja yang ditandatangani yang didaftarkan runner baru. Dihapus setelah hook keluar. Jangan catat isi file.                                                                                                                                                                                               |
| `CLAUDE_RUNNER_ORDER_ID`              | Kunci idempotency yang buram, unik per permintaan pemijahan dan aman untuk nama sumber daya Kubernetes. Gunakan sebagai kunci dedup provisioner Anda.                                                                                                                                                                                                |
| `CLAUDE_RUNNER_SESSION_ID`            | Sesi yang diminta ini untuk. Kosong untuk permintaan pre-warming, yang boot runner standby sebelum sesi spesifik apa pun ketika [`--min-idle`](/docs/id/self-hosted-environments-reference#orchestrator-cli-flags) diatur, jadi jangan asumsikan variabel diatur.                                                                                         |
| `CLAUDE_RUNNER_SESSION_UUID`          | ID sesi yang sama dalam bentuk UUID kanonik. Kosong untuk permintaan pre-warming.                                                                                                                                                                                                                                                                    |
| `CLAUDE_RUNNER_ATTEMPT`               | Berapa banyak permintaan pemijahan yang dimiliki sesi ini. `0` untuk permintaan pre-warming.                                                                                                                                                                                                                                                         |
| `CLAUDE_RUNNER_ORDER_SERVER_TIME`     | Waktu server dari header HTTP `Date` respons polling. Ketika hook memverifikasi `exp` JWT pesanan kerja, bandingkan terhadap nilai ini daripada jam lokal untuk mentoleransi skew. Kosong ketika gateway menghilangkan header.                                                                                                                       |
| `CLAUDE_RUNNER_POOL_ID`               | ID lingkungan yang harus diikuti runner baru, dalam bentuk `ccpool_...`                                                                                                                                                                                                                                                                              |
| `CLAUDE_RUNNER_ACCOUNT_ID`            | ID yang ditandai dari akun yang antri sesi, untuk routing per-akun, kuota, atau chargeback. Kosong ketika tidak tersedia, dan selalu kosong untuk sesi saluran Claude Tag, yang tidak ada akun antri.                                                                                                                                                |
| `CLAUDE_RUNNER_ACCOUNT_EMAIL`         | Email akun yang antri sesi. Kosong ketika tidak tersedia. Perlakukan email sebagai informasi yang dapat diidentifikasi secara pribadi dan jangan catat.                                                                                                                                                                                              |
| `CLAUDE_RUNNER_PRIMARY_REPO_URL`      | URL sumber git pertama sesi, untuk routing ke runner dengan repositori itu pre-warmed. Kosong ketika sesi tidak memiliki sumber git.                                                                                                                                                                                                                 |
| `CLAUDE_RUNNER_PRIMARY_REPO_REVISION` | Revisi sumber git pertama sesi: cabang, SHA, atau tag. Kosong ketika tidak ditentukan.                                                                                                                                                                                                                                                               |
| `CLAUDE_RUNNER_REPO_SOURCES`          | Array JSON dari `{url, revision}` untuk semua sumber git sesi, untuk hook yang route pada repositori sekunder. Kosong ketika tidak ada sumber.                                                                                                                                                                                                       |
| `CLAUDE_RUNNER_CORRELATION_ID`        | ID korelasi yang disediakan saat pembuatan sesi, digemakan kembali sehingga hook dapat memetakan pesanan kerja ini ke permintaan yang membuat sesi. Kosong ketika sesi tidak memiliki satu.                                                                                                                                                          |
| `CLAUDE_RUNNER_CLIENT_PLATFORM`       | Permukaan klien yang membuat sesi, seperti `web_claude_ai`, `desktop_app`, `ios`, atau `scheduled_trigger`, untuk analitik adopsi. Tidak diatur ketika sesi tidak memiliki permukaan yang tercatat atau dikenali, dan untuk permintaan pre-warming; periksa dengan `[ -n "${CLAUDE_RUNNER_CLIENT_PLATFORM:-}" ]`, yang tetap aman di bawah `set -u`. |

Runner yang dipijahkan mendaftarkan dengan pesanan kerja sebagai pengganti rahasia lingkungan:

* **Mulai dengan pesanan kerja**: arahkan [`--environment-secret-file`](/docs/id/self-hosted-environments-reference#runner-cli-flags) ke file yang berisi JWT pesanan kerja, atau atur `SELF_HOSTED_RUNNER_ENVIRONMENT_SECRET` ke nilai JWT.
* **Salin JWT sebelum hook keluar**: orchestrator menghapus file pesanan kerja setelah hook keluar, jadi salin JWT ke dalam beban kerja yang Anda kirimkan, seperti Kubernetes Secret di Job yang dipijahkan, daripada melewatkan jalur file.
* **Gunakan `--capacity 1` di runner yang dipijahkan**: pesanan kerja yang terikat sesi mendaftarkan tepat satu runner yang terikat ke sesi itu, jadi kapasitas lebih tinggi menambah slot yang tidak pernah menerima pekerjaan, dan runner mencatat peringatan saat startup.
* **Pesanan kerja pre-warming mendaftarkan tidak terikat**: runner standby tidak terikat ke sesi dan mengklaim pekerjaan antri seperti runner fleet tetap.

Kontrak memiliki empat aturan yang agnostik provisioner:

1. **Jadilah idempoten pada `CLAUDE_RUNNER_ORDER_ID`.** Pengiriman ulang permintaan yang sama harus memijahkan paling banyak satu runner. Turunkan nama sumber daya deterministik dari ID dan biarkan platform Anda menolak duplikat.
2. **Jangan coba ulang beban kerja.** Satu ID pesanan berarti paling banyak satu beban kerja yang dibuat. Jika runner tidak pernah mendaftar, Anthropic meminta ulang dengan ID pesanan segar setelah `--expected-spawn-seconds`.
3. **Gunakan kontrak kode keluar.** Keluar 0 berarti dikirimkan. Keluar 1 berarti kegagalan yang dapat dicoba ulang; sesi mundur dan ditawarkan kembali. Keluar 2 atau lebih tinggi berarti tidak dapat dicoba ulang; sesi diblokir dari pemijahan lagi sampai [Owner](/docs/id/cloud-environments#organization-shared-environments) memilih **Retry** di atasnya di tab **Activity** lingkungan. Pada keluar bukan nol, ekor stderr hook muncul di sana sebagai alasan kegagalan, jadi tulis kesalahan yang dapat ditindaklanjuti ke stderr dan jangan pernah rahasia. Untuk permintaan pre-warming tidak ada sesi untuk gagal: orchestrator mencatat keluar bukan nol secara lokal saja, dan server meminta ulang pemijahan setelah sewa.
4. **Atur `--expected-spawn-seconds` ke setidaknya waktu boot p99 Anda.** Ini adalah sewa server-side. Semua replika orchestrator harus menggunakan nilai yang sama.

Semua yang ditulis hook ke stdout atau stderr muncul dalam log orchestrator dengan kredensial secara otomatis diredaksi. Jika sesi tetap antri, periksa badan `/healthz` orchestrator untuk hitungan antrian, kemudian buka tab **Activity** lingkungan Anda di [halaman admin **Cloud environments**](https://claude.ai/admin-settings/cloud-environments): perluas sesi yang gagal di sana untuk kesalahan pemijahan, dan pilih **Retry** untuk memintanya ulang.

<h2 id="mcp-servers">
  Server MCP
</h2>

Untuk membuat [server MCP](/docs/id/mcp) tersedia di setiap sesi, tambahkan server tersebut saat waktu pembuatan citra dengan perintah `claude mcp add` yang sama digunakan pada instalasi desktop. Jika runner Anda adalah proses bare daripada kontainer, jalankan perintah yang sama sebagai pengguna runner di host, kemudian restart runner: runner membaca konfigurasi host sekali saat startup. Bendera `--scope user` diperlukan; cakupan lokal default menulis di bawah kunci per-direktori yang tidak disemai runner ke dalam sesi. Misalnya, di Dockerfile Anda:

```dockerfile theme={null}
RUN claude mcp add --scope user sidecar -- /usr/local/bin/mcp-sidecar
RUN claude mcp add --scope user --transport http internal http://mcp-gateway.svc.cluster.local:8080
```

Runner mengambil snapshot konfigurasi host sekali saat startup. Snapshot menangkap kunci `mcpServers` dari `.claude.json` host, yang berada di sebelah daripada di dalam `~/.claude/`, dan runner hanya menyemai kunci itu ke dalam konfigurasi terisolasi setiap sesi; status akun dan riwayat proyek dijatuhkan. Untuk mengonfirmasi bahwa server mencapai sesi, mulai sesi di lingkungan dan minta Claude untuk mencantumkan alat MCP-nya; runner juga mencatat peringatan startup untuk entri yang ditangkap yang `type`-nya tidak dikenali dan menjatuhkan entri, sehingga Anda dapat melihat mengapa server itu hilang dari sesi. Ketika `SELF_HOSTED_RUNNER_HOST_CONFIG_DIR` diatur, runner membaca `.claude.json` dari direktori itu sebagai gantinya, jadi menunjuk variabel ke direktori kosong juga menonaktifkan penyemaian MCP.

Claude Code juga memuat server MCP dari sumber lain:

* File [MCP terkelola](/docs/id/managed-mcp) cakupan enterprise di jalur sistem standarnya: `/etc/claude-code/managed-mcp.json` pada host runner Linux, `/Library/Application Support/ClaudeCode/managed-mcp.json` pada host macOS. Gunakan untuk armada terkunci di mana hanya server yang terdaftar administrator yang dapat dimuat. Lihat [kontrol eksklusif dengan managed-mcp.json](/docs/id/managed-mcp#exclusive-control-with-managed-mcp-json) untuk aturan prioritas. Ketika file ini berada di host runner, Claude Code melewati server MCP yang bidang kontrol Anthropic berikan ke sesi, termasuk konektor claude.ai, dan menamakannya dalam peringatan pada stderr anak sesi, yang dicatat runner pada tingkat log `debug`. Sebelum v2.1.229, sesi tersebut keluar saat startup dengan `You cannot dynamically configure MCP servers when an enterprise MCP config is present`.
* Kunci [`managedMcpServers`](/docs/id/settings-reference#managedmcpservers) dalam [pengaturan terkelola](/docs/id/managed-settings) pada host runner: menyediakan server HTTP dan SSE tanpa mengambil kontrol eksklusif, sehingga server dari sumber lain masih dimuat. Memerlukan Claude Code v2.1.259 atau lebih baru.
* `<repo>/.mcp.json`: cakupan proyek. Komit file ke repositori; server-nya disetujui secara otomatis dalam sesi cloud.

Ketika pengiriman konektor diaktifkan untuk organisasi Anda, bidang kontrol Anthropic mengirimkan konektor yang telah Anda konfigurasi di claude.ai ke sesi yang dibuat secara interaktif melalui konfigurasi MCP yang disediakan server, dirutekan melalui `api.anthropic.com`. Sesi yang dibuat secara terprogram, seperti [pengiriman CLI](/docs/id/self-hosted-environments-testing#run-the-test-loop), tidak menerima pengiriman konektor; berikan mereka server MCP melalui salah satu sumber lain yang tercantum di bagian ini. Token OAuth anak tidak membawa cakupan untuk mengambil konektor secara langsung, jadi anak tidak mencoba pengambilan itu sendiri; pengiriman didorong server.

`settings.json` tidak membawa definisi server MCP, dan tidak ada bidang `mcpServers` tingkat atas dalam skema pengaturan. Dalam pengaturan terkelola, sediakan server dengan kunci [`managedMcpServers`](/docs/id/settings-reference#managedmcpservers) sebagai gantinya.

Sesi mewarisi lingkungan runner, jadi atur [`ENABLE_TOOL_SEARCH`](/docs/id/mcp#scale-with-mcp-tool-search) di sana untuk mengontrol pencarian alat MCP untuk setiap sesi yang dihasilkan runner; halaman MCP mencakup nilainya.

<h2 id="prompt-sessions-to-push-their-work">
  Prompt sesi untuk mendorong pekerjaan mereka
</h2>

Sesi yang di-host Anthropic menjalankan hook [`Stop`](/docs/id/hooks#stop), hook Claude Code yang berjalan ketika Claude selesai merespons, yang mendorong Claude untuk melakukan komit dan mendorong pekerjaannya. Runner tidak memasang satu. Tanpa itu, sesi yang berakhir dengan perubahan yang tidak dikomit meninggalkan pekerjaan itu hanya di disk runner, dan tombol **Create PR** di claude.ai/code tetap tidak aktif sampai cabang ada di remote.

Implementasi referensi di bawah memiliki dua bagian. Gabungkan blok pengaturan ke `~/.claude/settings.json` di host runner, yang ditanam runner ke dalam setiap sesi, dan simpan skrip sebagai `~/.claude/hooks/stop-hook-nudge.sh` di host runner dan buat dapat dieksekusi:

```json theme={null}
{
  "hooks": {
    "Stop": [
      {
        "hooks": [
          {
            "type": "command",
            "timeout": 10,
            "command": "\"$CLAUDE_CONFIG_DIR/hooks/stop-hook-nudge.sh\""
          }
        ]
      }
    ]
  }
}
```

```sh theme={null}
#!/bin/sh
# Implementasi referensi Stop-hook untuk runner yang di-host sendiri.
#
# Mendorong Claude sekali per giliran jika direktori proyek memiliki perubahan
# yang tidak dikomit ATAU komit yang tidak didorong, sehingga pekerjaan tidak
# hilang ketika sesi idle dirilis dan sehingga tombol "Create PR" di claude.ai/code
# menyala.
#
# Tingkat runner (tidak ada perubahan repo): jatuhkan file ini di ~/.claude/hooks/
# di host runner dan gabungkan blok pengaturan Stop-hook yang menyertai ke
# ~/.claude/settings.json — runner menabur keduanya ke dalam setiap sesi.
# Alternatif tingkat repo: komit ke <repo>/.claude/hooks/ dan ubah jalur perintah
# settings.json ke $CLAUDE_PROJECT_DIR/.claude/hooks/.
#
# stdin: payload JSON hook (lihat https://code.claude.com/docs/en/hooks)
# stdout: {"decision":"block","reason":"..."} untuk mendorong, atau tidak ada untuk memungkinkan stop.

# Penjaga re-entry: harness menetapkan stop_hook_active=true ketika menginvokasi
# ulang hook Stop setelah blok. Keluar sehingga kami hanya mendorong sekali per
# giliran. Harness memancarkan JSON kompak (tidak ada spasi setelah titik dua),
# yang pola ini andalkan; gunakan jq jika Anda memerlukan pemeriksaan yang toleran
# terhadap spasi.
in=$(cat)
case "$in" in *'"stop_hook_active":true'*) exit 0 ;; esac

d="$CLAUDE_PROJECT_DIR"

# Bukan repo git → tidak ada yang didorong.
git -C "$d" rev-parse --git-dir >/dev/null 2>&1 || exit 0

# Tidak ada remote → "dorong ke remote" tidak dapat dipenuhi; keluar.
[ -z "$(git -C "$d" remote 2>/dev/null)" ] && exit 0

# Perubahan yang tidak dikomit (staged, unstaged, atau untracked). Kecualikan
# .claude/ sepenuhnya — pengaturan yang ditanam operator dan status runtime
# yang ditulis CLI (kunci penjadwal, worktree, status rutin) tinggal di sana
# dan tidak ada yang merupakan "pekerjaan yang tidak dikomit" yang perlu didorong
# model.
s=$(git -C "$d" status --porcelain -- . ':(exclude).claude/' 2>/dev/null)
if [ -n "$s" ]; then
  printf '{"decision":"block","reason":"There are uncommitted changes in the repository. Please commit and push these changes to the remote branch."}'
  exit 0
fi

# Komit yang tidak didorong. Hitung komit di HEAD yang tidak dapat dijangkau dari
# ref pelacakan remote apa pun atau FETCH_HEAD. Ini bekerja secara seragam untuk:
#   - checkout init+fetch (default runner: hanya FETCH_HEAD ada)
#   - checkout berbasis klon (origin/* ada)
#   - default runner: anak dimulai di cabang hasil sesi, yang dibuat runner
#     setelah checkout
#   - detached HEAD, ketika setup kustom melewati pembuatan cabang itu
# Tanpa titik referensi sama sekali (tidak pernah diambil), tetap diam daripada
# false-positive pada giliran hanya-baca.
base=""
git -C "$d" rev-parse --verify -q FETCH_HEAD >/dev/null && base="FETCH_HEAD"
if [ -z "$base" ] && [ -z "$(git -C "$d" for-each-ref --count=1 refs/remotes/origin 2>/dev/null)" ]; then
  exit 0
fi
# shellcheck disable=SC2086  # $base adalah "" atau "FETCH_HEAD", pemisahan kata yang disengaja
unpushed=$(git -C "$d" rev-list HEAD --not $base --remotes=origin --count 2>/dev/null) || unpushed=0
if [ "$unpushed" -gt 0 ]; then
  branch=$(git -C "$d" symbolic-ref --short -q HEAD)
  if [ -n "$branch" ]; then
    # $branch dipengaruhi penyerang — git-check-ref-format(1) memungkinkan `"`
    # dalam nama ref. `\` dilarang (aturan 10) tetapi lolos pula sebagai pertahanan
    # kedalaman murah.
    # Lolos karakter meta JSON sebelum interpolasi ke payload yang dibangun tangan
    # sehingga cabang seperti x","continue":false tidak dapat menyuntikkan kunci ke
    # JSON output-hook yang diurai harness. $unpushed aman — penjaga -gt di atas
    # menolak apa pun yang bukan integer biasa.
    branch_esc=$(printf '%s' "$branch" | sed 's/\\/\\\\/g; s/"/\\"/g')
    printf '{"decision":"block","reason":"There are %s unpushed commit(s) on branch '\''%s'\''. Please push these changes to the remote repository."}' "$unpushed" "$branch_esc"
  else
    printf '{"decision":"block","reason":"There are %s unpushed commit(s) on a detached HEAD. Please create a branch and push it to the remote repository."}' "$unpushed"
  fi
  exit 0
fi

exit 0
```

Hook mendorong Claude untuk melakukan komit dan mendorong sebelum sesi berakhir, dan tetap diam ketika direktori bukan repositori git atau tidak memiliki remote.

<h2 id="permissions-and-tool-approval">
  Izin dan persetujuan alat
</h2>

Sesi yang di-host sendiri tidak memiliki terminal yang terlampir, jadi prompt izin yang tidak dijawab menghentikan giliran sampai pengguna merespons di UI. Bidang kontrol Anthropic mengirimkan daftar alat setiap sesi dan aturan izin dengan muatan kerja; konfigurasi default pre-menyetujui panggilan alat rutin, termasuk `Bash`, dan sesi cloud [pre-menyetujui edit file terlepas dari mode](/docs/id/permission-modes#switch-permission-modes). Panggilan yang tidak ada yang pre-menyetujui mendorong melalui UI sesi.

<Note>
  Hanya pin mode auto di lingkungan yang kontainer sesinya berjalan dengan [default-deny network egress](/docs/id/self-hosted-environments-deploy#default-deny-egress) dan sisa [bagian hardening](/docs/id/self-hosted-environments-deploy#harden-your-deployment) di tempat. Panggilan alat rutin, termasuk permintaan jaringan `Bash`, berjalan tanpa manusia dalam loop di set alat pre-disetujui default dan dalam mode auto, jadi batas jaringan adalah apa yang membatasi di mana panggilan itu dapat menjangkau.
</Note>

Untuk menjaga prompt ke minimum terlepas dari apa yang dikirimkan bidang kontrol, pin [mode auto](/docs/id/permission-modes#eliminate-prompts-with-auto-mode) dari skrip wrapper atau hook [`command`](#command) Anda. Mode auto memungkinkan sesi berjalan tanpa prompt izin rutin: model pengklasifikasi terpisah meninjau tindakan sebelum mereka berjalan dan memblokir yang ditolaknya, dan aturan ask eksplisit masih memaksa prompt; halaman mode izin mencakup apa yang diperiksa pengklasifikasi. Runner menambahkan flag yang dihitung server sebelum menginvokasi wrapper, dan untuk flag nilai tunggal seperti `--permission-mode` parser menghormati kemunculan terakhir, jadi flag yang Anda tambahkan setelah `"$@"` menimpa nilai yang dikirim server:

```bash theme={null}
#!/bin/bash
exec "$CLAUDE_RUNNER_CLAUDE_BIN" "$@" --permission-mode auto
```

Untuk pre-menyetujui alat spesifik sebagai gantinya, tambahkan `--allowed-tools` dengan aturan Anda, misalnya `--allowed-tools "Bash(bazel *) Bash(yarn *) mcp__internal__*"`. Flag daftar seperti `--allowed-tools` dan `--disallowed-tools` terakumulasi di seluruh kemunculan daripada menimpa, jadi aturan Anda berlaku di atas aturan apa pun yang dikirimkan bidang kontrol. Untuk mempersempit, tambahkan `--disallowed-tools`, yang menolak alat bahkan jika aturan lain memungkinkannya.

<h3 id="how-each-session’s-config-is-assembled">
  Bagaimana konfigurasi setiap sesi dirakit
</h3>

Runner memberikan setiap sesi direktori konfigurasinya sendiri, ditanam dari snapshot `~/.claude/` host yang ditangkap runner sekali saat startup: `settings.json`, `CLAUDE.md`, hooks, agents, commands, dan skills di gambar runner Anda berlaku untuk setiap sesi sebagai baseline tingkat pengguna. Jika Anda mengubah konfigurasi di host yang berjalan, perubahan hanya berlaku setelah Anda memulai ulang runner.

Atur `SELF_HOSTED_RUNNER_HOST_CONFIG_DIR` untuk menabur dari jalur berbeda, atau arahkan ke direktori kosong untuk menonaktifkan penanaman.

`.claude/settings.json` yang dikomit repositori berlapis di atas sebagai pengaturan proyek. Sesi juga membaca [`managed-settings.json`](/docs/id/settings#where-settings-live) dari jalur sistem standar di gambar runner Anda. Apakah kuncinya berlaku bersama [pengaturan yang dikelola server](/docs/id/server-managed-settings) mengikuti [bagaimana Claude Code menggabungkan sumber yang dikelola](/docs/id/managed-settings#how-claude-code-combines-managed-sources): secara default, ketika organisasi Anda mengirimkan kunci yang dikelola server apa pun, sesi mengabaikan file gambar runner terlepas dari [kunci yang dibaca Claude Code dari setiap sumber admin](/docs/id/managed-settings#keys-read-from-every-admin-source), seperti blok `env`, kunci sandbox, jalur biner sandbox, dan `forceRemoteSettingsRefresh`. Lihat [prioritas pengaturan](/docs/id/settings#settings-precedence).

Ketika bidang kontrol Anthropic memasok sesi dengan [Claude Code hooks](/docs/id/hooks), runner memasangnya bersama, bukan di atas, konfigurasi Anda sendiri. Memerlukan Claude Code v2.1.229 atau lebih baru.

* **Di mana mereka mendarat**: runner menulis setiap skrip hook yang disediakan ke subdirektori `hooks/.ccr-launcher/` yang dicadangkan dari direktori konfigurasi sesi dan mendaftarkan skrip dalam file pengaturan terpisah yang dilewatkan ke sesi dengan `--settings`, meninggalkan `settings.json` yang ditanam dan skrip Anda sendiri di `hooks/<name>` tidak tersentuh. Runner membuat ulang subdirektori yang dicadangkan untuk setiap sesi dan tidak menabur konten host di `~/.claude/hooks/.ccr-launcher/` ke dalam sesi.
* **Siapa yang menulisnya**: bidang kontrol mengisinya dari konstanta tetap dalam deployment-nya sendiri, tidak pernah dari input per-sesi atau pihak ketiga.
* **Apa yang masih mengaturnya**: hook yang dikirimkan melalui `--settings` memasuki konfigurasi hook yang digabungkan biasa, bukan tingkat yang dikelola, jadi pengaturan yang dikelola Anda masih berlaku. `disableAllHooks` menonaktifkannya, dan mereka bukan di antara kategori yang [`allowManagedHooksOnly`](/docs/id/settings-reference#allowmanagedhooksonly) tetap dimuat.

<h3 id="repository-committed-permission-rules">
  Aturan izin yang dikomit repositori
</h3>

Jangan letakkan entri `"Edit"`, `"Write"`, atau `"NotebookEdit"` telanjang dalam `permissions.allow` yang dikomit repositori. Aturan alat file telanjang cocok dengan alat terlepas dari jalur, memberikan penulisan di mana saja di host daripada hanya ruang kerja, jadi penjaga confine cakupan penulisan runner menandai sesi; dengan [`--confine-repo-settings enforce`](/docs/id/self-hosted-environments-reference#runner-cli-flags) itu menolak untuk memijahkan sesi daripada mencatat dan melanjutkan. Lihat [bagian hardening](/docs/id/self-hosted-environments-deploy#harden-your-deployment).

Repositori tidak memerlukan aturan alat file sama sekali: sesi cloud [pre-menyetujui edit file terlepas dari mode](/docs/id/permission-modes#switch-permission-modes). Jika Anda melakukan komit aturan, batasi ke ruang kerja, seperti `"Edit(/**)"`; garis miring tunggal di depan relatif terhadap akar proyek, yang merupakan ruang kerja sesi. Aturan alat file telanjang baik-baik saja dalam `settings.json` tingkat host operator, karena file itu tidak dikomit repositori.

`defaultMode` dari `auto` hanya dihormati dari file pengaturan tingkat gambar atau tingkat pengguna, jadi repositori yang diperiksa tidak dapat memberikan dirinya mode auto. Untuk mode mana sesi cloud terima dan sintaks aturan lengkap, lihat [mode izin](/docs/id/permission-modes).

<h2 id="what’s-next">
  Apa selanjutnya
</h2>

* [Reference](/docs/id/self-hosted-environments-reference): setiap flag CLI, variabel lingkungan, dan metrik
* [Verify session identity](/docs/id/self-hosted-environments-identity): validasi token sesi dari layanan di luar runner
