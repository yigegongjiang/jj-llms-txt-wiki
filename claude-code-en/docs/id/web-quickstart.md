> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Mulai dengan Claude Code di cloud

> Jalankan Claude Code di cloud dari browser atau ponsel Anda. Hubungkan repositori GitHub, kirimkan tugas, dan tinjau PR tanpa setup lokal.

<Note>
  Sesi cloud tersedia di paket Pro, Max, dan Team, serta untuk pengguna Enterprise dengan kursi premium atau kursi Chat + Claude Code.
</Note>

Sesi cloud menjalankan Claude Code pada infrastruktur cloud alih-alih mesin Anda, dikelola Anthropic secara default. Panduan singkat ini memulai satu dari [claude.ai/code](https://claude.ai/code) di browser Anda. Anda juga dapat memulai satu dari aplikasi mobile Claude, aplikasi Desktop, atau terminal Anda dengan `claude --cloud`.

Anda memerlukan repositori GitHub untuk [memulai](#connect-github). Claude mengklonnya ke mesin virtual yang terisolasi, membuat perubahan, dan mendorong cabang untuk Anda tinjau. Sesi bertahan di seluruh perangkat, jadi tugas yang Anda mulai di laptop siap ditinjau dari ponsel Anda nanti.

Sesi cloud bekerja dengan baik untuk:

* **Tugas paralel**: jalankan beberapa tugas independen sekaligus, masing-masing dalam sesi dan cabangnya sendiri, tanpa mengelola beberapa worktrees
* **Repo yang tidak Anda miliki secara lokal**: Claude mengklonkan repo segar setiap sesi, jadi Anda tidak perlu memeriksanya
* **Tugas yang tidak memerlukan pengarahan sering**: kirimkan tugas yang terdefinisi dengan baik, lakukan sesuatu yang lain, dan tinjau hasilnya ketika Claude selesai
* **Pertanyaan kode dan eksplorasi**: pahami basis kode atau lacak bagaimana fitur diimplementasikan tanpa checkout lokal

Untuk pekerjaan yang memerlukan konfigurasi lokal, alat, atau lingkungan Anda, menjalankan Claude Code secara lokal atau menggunakan [Remote Control](/docs/id/remote-control) adalah pilihan yang lebih baik.

<h2 id="how-sessions-run">
  Bagaimana sesi berjalan
</h2>

Langkah-langkah di bawah ini menjelaskan sesi yang dihosting Anthropic. Dalam [lingkungan yang dihosting sendiri](/docs/id/self-hosted-environments), klon dan semuanya setelahnya berjalan pada runner organisasi Anda sendiri, di mana batas jaringan, setup, dan perilaku push dikonfigurasi oleh operator. Ketika Anda mengirimkan tugas:

1. **Klonkan dan persiapkan**: repositori Anda diklonkan ke VM yang dikelola Anthropic, dan [skrip setup](/docs/id/cloud-environments#setup-scripts) Anda berjalan jika dikonfigurasi.
2. **Konfigurasi jaringan**: akses internet diatur berdasarkan [tingkat akses](/docs/id/cloud-environments#access-levels) lingkungan Anda.
3. **Bekerja**: Claude menganalisis kode, membuat perubahan, menjalankan tes, dan memeriksa pekerjaannya. Anda dapat menonton dan mengarahkan sepanjang waktu, atau pergi dan kembali ketika selesai.
4. **Dorong cabang**: ketika Claude mencapai titik pemberhentian, ia mendorong cabangnya ke GitHub. Anda meninjau diff, meninggalkan komentar inline, membuat PR, atau mengirim pesan lain untuk melanjutkan.

Sesi tidak ditutup ketika cabang didorong. Pembuatan PR dan pengeditan lebih lanjut semuanya terjadi dalam percakapan yang sama.

<h2 id="compare-ways-to-run-claude-code">
  Bandingkan cara menjalankan Claude Code
</h2>

Claude Code berperilaku sama di mana pun. Yang berubah adalah tempat sesi berjalan dan apakah konfigurasi lokal Anda tersedia:

|                                        | Sesi cloud                                                                                                        | Sesi lokal                                                                                                                     | Sesi lokal dengan [Remote Control](/docs/id/remote-control)              |
| :------------------------------------- | :---------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------ |
| **Kode berjalan di**                   | Cloud VM, dikelola Anthropic secara default                                                                       | Mesin Anda                                                                                                                     | Mesin Anda                                                          |
| **Anda memulainya dari**               | claude.ai/code, aplikasi Claude mobile, aplikasi Desktop dengan **Cloud** dipilih, atau `claude --cloud`          | Terminal Anda, IDE Anda, atau aplikasi Desktop dengan **Local** dipilih                                                        | Terminal Anda, ekstensi VS Code, atau aplikasi Desktop              |
| **Anda chat dari**                     | claude.ai, aplikasi mobile, atau aplikasi Desktop                                                                 | Tempat Anda memulainya                                                                                                         | claude.ai atau aplikasi mobile, serta tempat Anda memulainya        |
| **Menggunakan konfigurasi lokal Anda** | Tidak, hanya repo                                                                                                 | Ya                                                                                                                             | Ya                                                                  |
| **Memerlukan GitHub**                  | Ya, atau [bundel repo lokal](/docs/id/claude-code-on-the-web#send-local-repositories-without-github) melalui `--cloud` | Tidak                                                                                                                          | Tidak                                                               |
| **Terus berjalan jika Anda terputus**  | Ya                                                                                                                | Tidak                                                                                                                          | Selama sesi tetap terbuka di mesin Anda                             |
| **[Mode izin](/docs/id/permission-modes)**  | Terima editan, Plan, Auto                                                                                         | Semua mode di terminal; lihat [Beralih mode izin](/docs/id/permission-modes#switch-permission-modes) untuk IDE dan aplikasi Desktop | Manual, Terima editan, atau Plan dari claude.ai dan aplikasi mobile |
| **Akses jaringan**                     | Dapat dikonfigurasi per lingkungan                                                                                | Jaringan mesin Anda                                                                                                            | Jaringan mesin Anda                                                 |

Lihat dokumentasi [terminal quickstart](/docs/id/quickstart), [aplikasi Desktop](/docs/id/desktop), atau [Remote Control](/docs/id/remote-control) untuk mengatur sesi lokal.

<h2 id="connect-github">
  Hubungkan GitHub
</h2>

Menghubungkan GitHub adalah langkah satu kali. Jika Anda sudah menggunakan GitHub CLI, Anda dapat [melakukan ini dari terminal Anda](#connect-from-your-terminal) alih-alih browser.

<Note>
  Pada paket Team dan Enterprise, langkah **Sign in with GitHub** hanya berfungsi setelah [Owner](/docs/id/server-managed-settings#access-control) organisasi Claude Anda mengaktifkan konektor GitHub di [**Admin settings > Connectors**](https://claude.ai/admin-settings/connectors). Sampai saat itu, langkah tersebut menampilkan "GitHub access is required for Claude Code on the web" alih-alih tombol sign-in. Setelah konektor aktif, muat ulang [claude.ai/code](https://claude.ai/code) dan mulai lagi dari langkah pertama. Toggle kedua, [Quick web setup](/docs/id/claude-code-on-the-web#github-authentication-options) di [**Admin settings > Claude Code**](https://claude.ai/admin-settings/claude-code), bersifat opsional: dengan toggle aktif, `/web-setup` berfungsi dan onboarding membuat lingkungan untuk anggota.
</Note>

<Steps>
  <Step title="Visit claude.ai/code">
    Buka [claude.ai/code](https://claude.ai/code) dan masuk dengan akun claude.ai Anda.
  </Step>

  <Step title="Sign in with GitHub">
    Setelah Anda masuk, claude.ai/code meminta Anda untuk menghubungkan GitHub. Ikuti prompt, dan claude.ai/code mengirimkan Anda ke halaman otorisasi GitHub. Setujui permintaan otorisasi, dan GitHub mengembalikan Anda ke claude.ai/code. Sesi cloud bekerja dengan repositori GitHub yang ada. Untuk memulai proyek baru, [buat repositori kosong di GitHub](https://github.com/new) terlebih dahulu.

    Dengan koneksi ini, sesi dapat mengkloning repositori publik apa pun, tetapi dapat bekerja di repositori pribadi hanya ketika Claude GitHub App diinstal di dalamnya. [Instal Claude GitHub App](https://github.com/apps/claude/installations/new) di setiap akun GitHub atau organisasi yang repositori pribadinya ingin Anda gunakan. Di organisasi GitHub, pemilik organisasi mungkin perlu menyetujui instalasi. Menginstal Aplikasi juga mengaktifkan [Auto-fix](/docs/id/claude-code-on-the-web#auto-fix-pull-requests), yang memungkinkan Claude merespons kegagalan CI dan komentar review pada pull request di repositori tersebut.

    Jika onboarding meminta Anda untuk menginstal Claude GitHub App pada titik ini dan Anda lebih suka melakukannya nanti, klik **Skip**.
  </Step>

  <Step title="Set up your Default environment">
    [Cloud environment](/docs/id/cloud-environments) adalah konfigurasi yang disimpan yang mengontrol akses jaringan apa yang dimiliki Claude selama sesi dan apa yang berjalan ketika sesi dimulai. Apa yang terjadi setelah Anda menghubungkan GitHub tergantung pada paket Anda:

    * **Pro dan Max**: onboarding membuat lingkungan bernama **Default** untuk Anda.
    * **Team dan Enterprise**: onboarding menampilkan formulir **Create your first cloud environment**. Biarkan nama yang sudah diisi sebelumnya dan akses jaringan tidak berubah dan klik **Create & finish** untuk membuat lingkungan **Default**. Jika Owner telah mengaktifkan [Quick web setup](/docs/id/claude-code-on-the-web#github-authentication-options), onboarding membuat **Default** untuk Anda sebagai gantinya.

    **Default** menggunakan [`Trusted` network access](/docs/id/cloud-environments#access-levels): sesi menjangkau [registri paket umum](/docs/id/cloud-environments#default-allowed-domains) dan domain lain yang diizinkan, dan tidak ada yang lain melalui jaringan sesi. Lihat [Installed tools](/docs/id/cloud-environments#installed-tools) untuk apa yang tersedia tanpa konfigurasi apa pun.

    Untuk proyek pertama, lingkungan **Default** berfungsi apa adanya. Untuk mengubah akses jaringannya, menambahkan variabel lingkungan, atau menjalankan [setup script](/docs/id/cloud-environments#setup-scripts) sebelum sesi dimulai, [edit atau buat lingkungan tambahan](/docs/id/cloud-environments#configure-your-environment).
  </Step>
</Steps>

<h3 id="connect-from-your-terminal">
  Hubungkan dari terminal Anda
</h3>

Jika Anda sudah menggunakan GitHub CLI (`gh`), Anda dapat menghubungkan GitHub untuk sesi cloud dari terminal Anda. Ini memerlukan [Claude Code CLI](/docs/id/quickstart). Pada paket Team dan Enterprise, `/web-setup` tersedia hanya setelah Owner mengaktifkan [Quick web setup](/docs/id/claude-code-on-the-web#github-authentication-options).

Ketika Anda menjalankan `/web-setup`, Claude Code membaca token yang dicetak `gh auth token`, meminta Anda untuk mengonfirmasi, dan mengirimkan token ke Anthropic. Anthropic menyimpannya terenkripsi dengan akun claude.ai Anda, dan sesi cloud Anda menggunakannya untuk akses GitHub sampai Anda [menghapusnya](#remove-the-web-setup-token). Sesi cloud yang Anda mulai sendiri kemudian dapat mengakses repositori apa pun yang dapat diakses token tersebut, tanpa instalasi Claude GitHub App. Thread dalam [proyek](/docs/id/claude-projects#set-up-github-access) masih memerlukan Claude GitHub App.

Jika Anda sudah menghubungkan GitHub di browser, `/web-setup` memperingatkan Anda bahwa melanjutkan menggantikan koneksi tersebut untuk sesi cloud Anda.

<Note>
  Organisasi dengan [Zero Data Retention](/docs/id/zero-data-retention) yang diaktifkan tidak dapat menggunakan `/web-setup` atau fitur sesi cloud lainnya. Jika GitHub CLI tidak diinstal atau tidak diautentikasi, Claude Code membuka alur onboarding browser sebagai gantinya.
</Note>

<Steps>
  <Step title="Authenticate with the GitHub CLI">
    Di shell Anda, autentikasi GitHub CLI jika Anda belum melakukannya:

    ```bash theme={null}
    gh auth login
    ```
  </Step>

  <Step title="Sign in to Claude">
    Di Claude Code CLI, jalankan `/login` untuk masuk dengan akun claude.ai Anda. Lewati langkah ini jika Anda sudah masuk dengan akun claude.ai. Mengautentikasi dengan kunci API tidak dihitung. Untuk memeriksa, jalankan `/status` dan konfirmasi baris **Login method** menampilkan akun claude.ai.
  </Step>

  <Step title="Run /web-setup">
    Di Claude Code CLI, jalankan:

    ```text theme={null}
    /web-setup
    ```

    Konfirmasi prompt untuk mengirimkan token `gh` Anda ke akun Claude Anda. Jika berhasil, Claude Code mencetak `Connected as <your-github-username>` dan membuka [claude.ai/code](https://claude.ai/code) di browser Anda. Jika Anda belum memiliki lingkungan cloud, `/web-setup` membuat satu dengan akses jaringan Trusted dan tanpa setup script. Anda dapat [mengedit lingkungan atau menambahkan variabel](/docs/id/cloud-environments#configure-your-environment) setelahnya. Setelah `/web-setup` selesai, Anda dapat memulai sesi cloud dari terminal Anda dengan [`--cloud`](/docs/id/claude-code-on-the-web#from-terminal-to-cloud) atau mengatur tugas berulang dengan [`/schedule`](/docs/id/routines).
  </Step>
</Steps>

<h4 id="remove-the-web-setup-token">
  Hapus token `/web-setup`
</h4>

Untuk menghapus token dari akun Claude Anda, putuskan GitHub di [claude.ai/customize/connectors](https://claude.ai/customize/connectors). Memutuskan menghapus kredensial GitHub yang digunakan sesi cloud Anda, baik berasal dari browser atau dari `/web-setup`, jadi sesi cloud kehilangan akses GitHub sampai Anda menghubungkan lagi. `gh` lokal Anda tetap masuk, dan token tetap valid di GitHub.

Untuk membatalkan token itu sendiri, cabut di GitHub. Jika Anda masuk ke `gh` melalui browser, token milik entri **GitHub CLI** di bawah [**Settings > Applications > Authorized OAuth Apps**](https://github.com/settings/applications) di GitHub, dan mencabut entri tersebut juga menandatangani GitHub CLI di mesin Anda. Sesi cloud kemudian kehilangan akses GitHub sampai Anda menjalankan `gh auth login` dan `/web-setup` lagi.

<h2 id="start-a-task">
  Mulai tugas
</h2>

Dengan GitHub terhubung dan lingkungan dibuat, Anda siap mengirimkan tugas.

<Steps>
  <Step title="Pilih repositori dan cabang">
    Dari [claude.ai/code](https://claude.ai/code) atau tab Code di aplikasi mobile Claude, klik pemilih repositori di bawah kotak input dan pilih repositori untuk Claude bekerja. Setiap repositori menampilkan pemilih cabang. Ubahnya untuk memulai Claude dari cabang fitur alih-alih default. Anda dapat menambahkan beberapa repositori untuk bekerja di seluruhnya dalam satu sesi.
  </Step>

  <Step title="Pilih mode izin">
    Dropdown mode di sebelah input menunjukkan mode sesi akan berjalan:

    * **Auto**: pengklasifikasi meninjau tindakan Claude alih-alih meminta Anda. Muncul ketika organisasi Anda memungkinkan mode auto dan model yang dipilih mendukungnya
    * **Terima editan**: Claude membuat perubahan dan mendorong cabang tanpa berhenti untuk persetujuan
    * **Plan**: Claude mengusulkan pendekatan dan menunggu Anda menyetujuinya sebelum mengedit file

    Sesi cloud tidak menawarkan izin Manual atau Bypass. Lihat [daftar lengkap mode izin](/docs/id/permission-modes#available-modes) untuk apa yang masing-masing izinkan.
  </Step>

  <Step title="Jelaskan tugas dan kirimkan">
    Ketik deskripsi apa yang Anda inginkan dan tekan Enter. Jadilah spesifik:

    * Beri nama file atau fungsi: "Tambahkan README dengan instruksi setup" atau "Perbaiki tes auth yang gagal di `tests/test_auth.py`" lebih baik daripada "perbaiki tes"
    * Tempel output kesalahan jika Anda memilikinya
    * Jelaskan perilaku yang diharapkan, bukan hanya gejala

    Claude mengklonkan repositori, menjalankan skrip setup Anda jika dikonfigurasi, dan mulai bekerja. Setiap tugas mendapatkan sesi sendiri dan cabangnya sendiri, jadi Anda tidak perlu menunggu satu selesai sebelum memulai yang lain.
  </Step>
</Steps>

<h2 id="pre-fill-sessions">
  Isi sebelumnya sesi
</h2>

Anda dapat mengisi sebelumnya prompt, repositori, dan lingkungan untuk sesi baru dengan menambahkan parameter query ke URL [claude.ai/code](https://claude.ai/code). Gunakan ini untuk membangun integrasi seperti tombol di pelacak masalah Anda yang membuka Claude Code dengan deskripsi masalah sebagai prompt.

| Parameter      | Deskripsi                                                                                                                                                                                          |
| :------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `prompt`       | Teks prompt untuk diisi sebelumnya di kotak input. Alias `q` juga diterima.                                                                                                                        |
| `prompt_url`   | URL untuk mengambil teks prompt dari, untuk prompt yang terlalu panjang untuk disematkan dalam string query. URL harus memungkinkan permintaan lintas asal. Diabaikan ketika `prompt` juga diatur. |
| `repositories` | Daftar slug `owner/repo` yang dipisahkan koma untuk dipilih sebelumnya. Alias `repo` juga diterima.                                                                                                |
| `environment`  | Nama atau ID [lingkungan](#connect-github) untuk dipilih sebelumnya.                                                                                                                               |

URL-encode setiap nilai. Contoh di bawah membuka formulir dengan prompt dan repositori yang sudah dipilih:

```text theme={null}
https://claude.ai/code?prompt=Fix%20the%20login%20bug&repositories=acme/webapp
```

<h2 id="review-and-iterate">
  Tinjau dan ulangi
</h2>

Ketika Claude selesai, tinjau perubahan, tinggalkan umpan balik pada baris tertentu, dan terus sampai diff terlihat benar.

<Steps>
  <Step title="Buka tampilan diff">
    Indikator diff menunjukkan baris yang ditambahkan dan dihapus di seluruh sesi, misalnya `+42 -18`. Pilihnya untuk membuka tampilan diff, dengan daftar file di sebelah kiri dan perubahan di sebelah kanan.

    Diff membandingkan perubahan sesi terhadap cabang dasarnya secara default. Untuk membandingkan terhadap cabang yang berbeda, pilih **Compare against** dan pilih satu.
  </Step>

  <Step title="Tinggalkan komentar inline">
    Pilih baris apa pun di diff, ketik umpan balik Anda, dan tekan Enter. Komentar antri sampai Anda mengirim pesan berikutnya, kemudian digabungkan dengannya. Claude melihat "di `src/auth.ts:47`, jangan tangkap kesalahan di sini" bersama instruksi utama Anda, jadi Anda tidak harus menjelaskan di mana masalahnya.
  </Step>

  <Step title="Buat permintaan tarik">
    Ketika diff terlihat benar, pilih **Create PR** di bagian atas tampilan diff. Anda dapat membukanya sebagai PR penuh, draft, atau melompat ke halaman compose GitHub dengan judul dan deskripsi yang dihasilkan.
  </Step>

  <Step title="Terus ulangi setelah PR">
    Sesi tetap aktif setelah PR dibuat. Tempel output kegagalan CI atau komentar pengulas ke chat dan minta Claude untuk mengatasinya. Untuk membuat Claude memantau PR secara otomatis, lihat [Auto-fix pull requests](/docs/id/claude-code-on-the-web#auto-fix-pull-requests).
  </Step>
</Steps>

<h2 id="troubleshoot-setup">
  Troubleshoot setup
</h2>

<h3 id="no-repositories-appear-after-connecting-github">
  No repositories appear after connecting GitHub
</h3>

If you connected GitHub in the browser, sessions can clone any public repository, but a private repository appears only when the Claude GitHub App is installed on the account or organization that owns it and the installation's repository access includes it. [Install the Claude GitHub App](https://github.com/apps/claude/installations/new) there, or ask an organization owner to install or approve it.

If you connected with `/web-setup`, sessions reach every repository your `gh` token can access. Run `gh repo view OWNER/REPO` in your shell to check that your GitHub CLI login can see the repository, and run `/web-setup` again if you've switched `gh` accounts since connecting.

<h3 id="the-page-only-shows-a-github-login-button">
  The page only shows a GitHub login button
</h3>

Cloud sessions require a connected GitHub account. Connect via the browser flow above, or run `/web-setup` from your terminal if you use the GitHub CLI. If you'd rather not connect GitHub at all, see [Remote Control](/docs/id/remote-control) to run Claude Code on your own machine and monitor it from your browser or phone.

<h3 id="not-available-for-the-selected-organization">
  "Not available for the selected organization"
</h3>

Enterprise organizations may need an Owner to enable cloud sessions. Contact your Anthropic account team.

<h3 id="/web-setup-says-not-signed-in-to-claude">
  `/web-setup` says "Not signed in to Claude"
</h3>

If `/web-setup` responds with "Not signed in to Claude. Run /login first.", the CLI doesn't have a valid claude.ai sign-in. This can also happen when a previous sign-in has expired. Run `/login`, sign in with your claude.ai account, then run `/web-setup` again.

<h3 id="/web-setup-warns-that-your-token-doesn’t-have-the-workflow-scope">
  `/web-setup` warns that your token doesn't have the `workflow` scope
</h3>

If `/web-setup` says your GitHub CLI token doesn't have the `workflow` scope, you can continue, but GitHub can reject some pushes made with that token, such as pushes that change GitHub Actions workflow files. To add the scope, run `gh auth refresh -s workflow` in your shell, then run `/web-setup` again.

<h3 id="web-setup-shows-no-commands-match-or-unknown-command">
  `/web-setup` shows "No commands match" or "Unknown command"
</h3>

`/web-setup` runs inside the Claude Code CLI, not your shell. Launch `claude` first, then type `/web-setup` at the prompt.

If you typed it inside Claude Code and the command menu shows `No commands match "/web-setup"`, or submitting it returns `Unknown command: /web-setup`, the command is hidden because a requirement isn't met. The cause is usually that you're authenticated with an API key or third-party provider instead of a claude.ai subscription. Run `/login` to sign in with your claude.ai account.

On Team and Enterprise plans, the command is hidden by default: the [Quick web setup toggle](/docs/id/claude-code-on-the-web#github-authentication-options) is off until an Owner turns it on. While it's off, [connect GitHub from the browser](#connect-github) instead.

The command is also hidden in two other cases:

* An administrator has disabled cloud sessions for your organization. In this case, submitting `/web-setup` returns [`Cloud sessions are disabled by your organization's policy`](/docs/id/errors#cloud-sessions-are-disabled-by-your-organizations-policy). Before v2.1.268, this case also returned `Unknown command: /web-setup`.
* Your Enterprise organization has [Zero Data Retention](/docs/id/zero-data-retention) enabled, which makes cloud sessions unavailable.

<h3 id="could-not-create-a-cloud-environment-or-no-cloud-environment-available-when-using-cloud">
  "Could not create a cloud environment" or "No cloud environment available" when using `--cloud`
</h3>

Cloud session features create a default cloud environment automatically if you don't have one. If you see "Could not create a cloud environment", automatic creation failed. If you see "No cloud environment available", your CLI predates automatic creation. In either case, run `/web-setup` in the Claude Code CLI, or add an environment from the [environment selector](/docs/id/cloud-environments#configure-your-environment) at [claude.ai/code](https://claude.ai/code).

<h3 id="setup-script-failed">
  Setup script failed
</h3>

The setup script exited with a non-zero status, which blocks the session from starting. Common causes:

* A package install failed because the registry isn't in your [network access level](/docs/id/cloud-environments#access-levels). `Trusted` covers most package managers; `None` blocks them all.
* The script references a file or path that doesn't exist in a fresh clone.
* A command that works locally needs a different invocation on Ubuntu.

To debug, add `set -x` at the top of the script to see which command failed. For non-critical commands, append `|| true` so they don't block session start.

<h3 id="new-sessions-hang-or-time-out-during-setup">
  New sessions hang or time out during setup
</h3>

If new sessions stall on the setup script step or fail with a generic container error before the script finishes, the script is likely exceeding the roughly five-minute time budget for building the [environment cache](/docs/id/cloud-environments#environment-caching). Heavy steps such as pulling large Docker images, syncing full dependency trees, or downloading model weights often push the total over the limit, especially when they run one after another.

To fix this, trim the script so it reliably finishes in under five minutes:

* Run independent installs in parallel with `&` and a final `wait` instead of running them serially.
* Move the largest downloads out of the setup script and into a [SessionStart hook](/docs/id/cloud-environments#setup-scripts-vs-sessionstart-hooks) that launches them in the background, so the session becomes usable while they finish.
* Remove long retry sleeps from the setup script, since a stalled retry loop counts against the budget.

<h3 id="session-keeps-running-after-closing-the-tab">
  Session keeps running after closing the tab
</h3>

This is by design. Closing the tab or navigating away doesn't stop the session. It continues running in the background until Claude finishes the current task, then idles. From the sidebar, you can [archive a session](/docs/id/claude-code-on-the-web#archive-sessions) to hide it from your list, or [delete it](/docs/id/claude-code-on-the-web#delete-sessions) to remove it permanently.

<h2 id="next-steps">
  Langkah berikutnya
</h2>

Sekarang bahwa Anda dapat mengirimkan dan meninjau tugas, halaman-halaman ini mencakup apa yang akan datang: memulai sesi cloud dari terminal Anda, menjadwalkan pekerjaan berulang, dan memberikan Claude instruksi berdiri.

* [Gunakan Claude Code di web](/docs/id/claude-code-on-the-web): referensi lengkap, termasuk teleportasi sesi ke terminal Anda, berbagi sesi, dan perbaikan otomatis permintaan tarik
* [Konfigurasi lingkungan cloud](/docs/id/cloud-environments): tingkat akses jaringan, variabel lingkungan, dan skrip setup untuk sesi cloud
* [Routines](/docs/id/routines): otomatiskan pekerjaan sesuai jadwal, melalui panggilan API, atau sebagai respons terhadap peristiwa GitHub
* [CLAUDE.md](/docs/id/memory): berikan Claude instruksi dan konteks persisten yang dimuat di awal setiap sesi
* Instal aplikasi mobile Claude untuk [iOS](https://apps.apple.com/us/app/claude-by-anthropic/id6473753684) atau [Android](https://play.google.com/store/apps/details?id=com.anthropic.claude) untuk memantau sesi dari ponsel Anda. Dari Claude Code CLI, `/mobile` menampilkan kode QR untuk [claude.ai/mobile](https://claude.ai/mobile) yang membuka toko aplikasi yang tepat untuk ponsel Anda.
