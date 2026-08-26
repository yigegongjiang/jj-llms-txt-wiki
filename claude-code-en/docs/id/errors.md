> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Referensi kesalahan

> Cari pesan kesalahan runtime Claude Code dengan penjelasan arti dan cara memperbaikinya.

Halaman ini mencantumkan kesalahan runtime yang ditampilkan Claude Code dan cara memulihkan dari masing-masing, ditambah apa yang harus diperiksa ketika respons tampak tidak normal tanpa kesalahan. Untuk kesalahan instalasi seperti `command not found` atau kegagalan TLS selama penyiapan, lihat [Troubleshoot installation and login](/docs/id/troubleshoot-install).

Kesalahan dan perintah pemulihan ini berlaku di seluruh CLI, [aplikasi Desktop](/docs/id/desktop), dan [Claude Code di web](/docs/id/claude-code-on-the-web), karena ketiganya membungkus CLI Claude Code yang sama. Untuk masalah khusus permukaan, lihat bagian troubleshooting di halaman permukaan tersebut.

<Note>
  Claude Code memanggil Claude API untuk respons model, jadi sebagian besar kesalahan runtime memetakan ke kode kesalahan API yang mendasar. Halaman ini mencakup apa arti setiap kesalahan di dalam Claude Code dan cara memulihkan. Untuk definisi kode status HTTP mentah, lihat [referensi kesalahan Platform Claude](https://platform.claude.com/docs/en/api/errors).
</Note>

<h2 id="find-your-error">
  Temukan kesalahan Anda
</h2>

Cocokkan pesan yang Anda lihat di terminal dengan bagian di bawah ini.

| Pesan                                                                                              | Bagian                                                                                                                          |
| :------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------ |
| `API Error: 500 Internal server error`                                                             | [Kesalahan server](#api-error-500-internal-server-error)                                                                        |
| `API Error: Repeated 529 Overloaded errors`                                                        | [Kesalahan server](#api-error-repeated-529-overloaded-errors)                                                                   |
| `Request timed out`                                                                                | [Kesalahan server](#request-timed-out), atau [Jaringan](#unable-to-connect-to-api) jika pesan menyebutkan koneksi internet Anda |
| `Server error mid-response. The response above may be incomplete.`                                 | [Kesalahan server](#the-response-above-may-be-incomplete)                                                                       |
| `Connection closed mid-response` / `Response stalled mid-stream`                                   | [Kesalahan server](#the-response-above-may-be-incomplete)                                                                       |
| `<model> is temporarily unavailable, so auto mode cannot determine the safety of...`               | [Kesalahan server](#auto-mode-cannot-determine-the-safety-of-an-action)                                                         |
| `Auto mode could not evaluate this action and is blocking it for safety`                           | [Kesalahan server](#auto-mode-cannot-determine-the-safety-of-an-action)                                                         |
| `Auto mode classifier transcript exceeded context window`                                          | [Kesalahan server](#auto-mode-cannot-determine-the-safety-of-an-action)                                                         |
| `Agent terminated early due to an API error`                                                       | [Kesalahan server](#agent-terminated-early-due-to-an-api-error)                                                                 |
| `You've hit your session limit` / `You've hit your weekly limit`                                   | [Batas penggunaan](#youve-hit-your-session-limit)                                                                               |
| `Usage credits required for 1M context`                                                            | [Batas penggunaan](#usage-credits-required-for-1m-context)                                                                      |
| `Server is temporarily limiting requests`                                                          | [Batas penggunaan](#server-is-temporarily-limiting-requests)                                                                    |
| `Request rejected (429)`                                                                           | [Batas penggunaan](#request-rejected-429)                                                                                       |
| `Credit balance is too low`                                                                        | [Batas penggunaan](#credit-balance-is-too-low)                                                                                  |
| `Not logged in · Please run /login`                                                                | [Autentikasi](#not-logged-in)                                                                                                   |
| `Could not resolve authentication method`                                                          | [Autentikasi](#could-not-resolve-authentication-method)                                                                         |
| `Invalid API key`                                                                                  | [Autentikasi](#invalid-api-key)                                                                                                 |
| `Your apiKeyHelper script is failing`                                                              | [Autentikasi](#your-apikeyhelper-script-is-failing)                                                                             |
| `This organization has been disabled`                                                              | [Autentikasi](#this-organization-has-been-disabled)                                                                             |
| `Your organization has disabled API key authentication`                                            | [Autentikasi](#your-organization-has-disabled-api-key-authentication)                                                           |
| `Your organization has disabled Claude subscription access`                                        | [Autentikasi](#your-organization-has-disabled-claude-subscription-access)                                                       |
| `Routines are disabled by your organization's policy`                                              | [Autentikasi](#routines-are-disabled-by-your-organizations-policy)                                                              |
| `Remote Control is only available when using Claude via api.anthropic.com`                         | [Autentikasi](#remote-control-requires-the-anthropic-api)                                                                       |
| `OAuth token revoked` / `OAuth token has expired`                                                  | [Autentikasi](#oauth-token-revoked-or-expired)                                                                                  |
| `Login expired · Please run /login`                                                                | [Autentikasi](#login-expired)                                                                                                   |
| `Failed to authenticate: OAuth session expired and could not be refreshed`                         | [Autentikasi](#login-expired)                                                                                                   |
| `does not meet scope requirement user:profile`                                                     | [Autentikasi](#oauth-scope-requirement)                                                                                         |
| `AWS credentials expired or invalid`                                                               | [Autentikasi](#aws-credentials-expired-or-invalid)                                                                              |
| `AWS authentication failed`                                                                        | [Autentikasi](#aws-authentication-failed)                                                                                       |
| `AWS default-chain credential resolve timed out`                                                   | [Autentikasi](#aws-default-chain-credential-resolve-timed-out)                                                                  |
| `Unable to connect to API`                                                                         | [Jaringan](#unable-to-connect-to-api)                                                                                           |
| `Waiting for API response · will retry in`                                                         | [Percobaan ulang otomatis](#automatic-retries), atau [Jaringan](#unable-to-connect-to-api) jika terus berlanjut                 |
| `Bedrock streaming response has content-type "..."; expected "application/vnd.amazon.eventstream"` | [Jaringan](#bedrock-streaming-response-has-an-unexpected-content-type)                                                          |
| `SSL certificate verification failed`                                                              | [Jaringan](#ssl-certificate-errors)                                                                                             |
| `SSL certificate error (...)` during login or startup                                              | [Jaringan](#ssl-certificate-errors)                                                                                             |
| `403` with `x-deny-reason: host_not_allowed` in a cloud or routine session                         | [Jaringan](#host-not-allowed-in-a-cloud-session)                                                                                |
| `Couldn't reconnect to your Remote Control session`                                                | [Jaringan](#couldnt-reconnect-to-your-remote-control-session)                                                                   |
| `Prompt is too long`                                                                               | [Kesalahan permintaan](#prompt-is-too-long)                                                                                     |
| `Error during compaction: Conversation too long`                                                   | [Kesalahan permintaan](#error-during-compaction-conversation-too-long)                                                          |
| `Request too large`                                                                                | [Kesalahan permintaan](#request-too-large)                                                                                      |
| `Image was too large`                                                                              | [Kesalahan permintaan](#image-was-too-large)                                                                                    |
| `Unable to resize image`                                                                           | [Kesalahan permintaan](#unable-to-resize-image)                                                                                 |
| `PDF too large` / `PDF is password protected`                                                      | [Kesalahan permintaan](#pdf-errors)                                                                                             |
| `Extra inputs are not permitted`                                                                   | [Kesalahan permintaan](#extra-inputs-are-not-permitted)                                                                         |
| `There's an issue with the selected model`                                                         | [Kesalahan permintaan](#theres-an-issue-with-the-selected-model)                                                                |
| `Model ... is not a recognized model id`                                                           | [Kesalahan permintaan](#model-is-not-a-recognized-model-id)                                                                     |
| `Claude Opus is not available with the Claude Pro plan`                                            | [Kesalahan permintaan](#claude-opus-is-not-available-with-the-claude-pro-plan)                                                  |
| `Model ... is restricted by your organization's settings`                                          | [Kesalahan permintaan](#model-is-restricted-by-your-organizations-settings)                                                     |
| `thinking.type.enabled is not supported for this model`                                            | [Kesalahan permintaan](#thinking-type-enabled-is-not-supported-for-this-model)                                                  |
| `max_tokens must be greater than thinking.budget_tokens`                                           | [Kesalahan permintaan](#thinking-budget-exceeds-output-limit)                                                                   |
| `API Error: 400 due to tool use concurrency issues`                                                | [Kesalahan permintaan](#tool-use-or-thinking-block-mismatch)                                                                    |
| `Claude Code is unable to respond to this request, which appears to violate our Usage Policy`      | [Kesalahan permintaan](#usage-policy-refusal)                                                                                   |
| `<model> has safety measures that flagged this message for a cybersecurity topic`                  | [Kesalahan permintaan](#safety-measures-flagged-a-cybersecurity-topic)                                                          |
| `Installation was killed before it could finish (exit code 137)`                                   | [Kesalahan instalasi](#installation-was-killed-before-it-could-finish)                                                          |
| `The connection dropped while downloading the update`                                              | [Kesalahan instalasi](#the-connection-dropped-while-downloading-the-update)                                                     |
| `Download timed out: exceeded the total deadline`                                                  | [Kesalahan instalasi](#the-connection-dropped-while-downloading-the-update)                                                     |
| `--bg and --print conflict`                                                                        | [Kesalahan baris perintah](#command-line-errors)                                                                                |
| `Error: --json-schema is not a valid JSON Schema`                                                  | [Kesalahan baris perintah](#command-line-errors)                                                                                |
| `Could not import <server>: <reason>`                                                              | [Kesalahan baris perintah](#could-not-import-a-server-from-claude-desktop)                                                      |
| `Error: MCP tool <name> (passed via --permission-prompt-tool) not found`                           | [Kesalahan baris perintah](#mcp-permission-prompt-tool-not-found)                                                               |
| `Marketplace "<name>" is registered from an untrusted source`                                      | [Kesalahan plugin](#marketplace-is-registered-from-an-untrusted-source)                                                         |
| `references ${user_config.*} in a shell-form command`                                              | [Kesalahan plugin](#plugin-command-references-user-config)                                                                      |
| `Monitor "<name>" from plugin <plugin> references ${user_config.*} in its command`                 | [Kesalahan plugin](#plugin-command-references-user-config)                                                                      |
| `headersHelper for MCP server '<name>' references ${user_config.*}`                                | [Kesalahan plugin](#plugin-command-references-user-config)                                                                      |
| `would be spawned with zero tools — refusing`                                                      | [Kesalahan alat](#agent-would-be-spawned-with-zero-tools)                                                                       |
| `File is covered by a Read deny rule in your permission settings`                                  | [Kesalahan alat](#file-is-covered-by-a-read-deny-rule)                                                                          |
| `Can't open MCP settings in a background session`                                                  | [Kesalahan sesi latar belakang](#commands-refused-in-a-background-session)                                                      |
| `CLAUDE_CODE_PROCESS_WRAPPER: launcher ...`                                                        | [Kesalahan sesi latar belakang](#claude_code_process_wrapper-launcher-errors)                                                   |
| `Ignoring N permissions.allow entries from ... this workspace has not been trusted`                | [Peringatan konfigurasi](#workspace-has-not-been-trusted)                                                                       |
| Responses seem lower quality than usual                                                            | [Kualitas respons](#responses-seem-lower-quality-than-usual)                                                                    |

<h2 id="automatic-retries">
  Percobaan ulang otomatis
</h2>

Claude Code mencoba ulang kegagalan transien sebelum menampilkan kesalahan kepada Anda. Kesalahan server, respons kelebihan beban, waktu tunggu permintaan, throttle 429 sementara, dan koneksi yang terputus semuanya dicoba ulang hingga 10 kali dengan backoff eksponensial. Mulai dari v2.1.198, ini mencakup koneksi yang terputus di tengah respons sebelum output yang terlihat telah dialirkan: Claude Code mengeluarkan kembali permintaan dengan backoff yang sama dan giliran berlanjut daripada berhenti dengan kesalahan koneksi. Mulai dari v2.1.199, throttle 429 sementara yang tidak membawa header kuota paket Anda juga dicoba ulang ketika Anda masuk dengan langganan claude.ai; versi sebelumnya hanya mencoba ulang untuk autentikasi kunci API dan Enterprise.

Beberapa kelas kegagalan tidak dicoba ulang, karena percobaan ulang tidak dapat berhasil:

* Mulai dari v2.1.199, kegagalan validasi sertifikat TLS, seperti proxy yang menginspeksi TLS, bundel `NODE_EXTRA_CA_CERTS` yang hilang, atau sertifikat yang kedaluwarsa, gagal pada percobaan pertama sehingga perbaikan muncul segera daripada setelah anggaran percobaan ulang penuh. Lihat [Kesalahan sertifikat SSL](#ssl-certificate-errors). Kondisi TLS transien seperti timeout handshake masih mencoba ulang.
* Mulai dari v2.1.199, kesalahan server yang tiba setelah Claude telah dialirkan output yang terlihat menjaga respons parsial dan menambahkan [pemberitahuan respons tidak lengkap](#the-response-above-may-be-incomplete) daripada mencoba ulang, karena menjalankan kembali permintaan dapat menjalankan alat yang sama dua kali. Versi sebelumnya membuang output parsial dan melaporkan giliran sebagai kesalahan.
* Respons streaming [Amazon Bedrock dengan tipe konten yang tidak terduga](#bedrock-streaming-response-has-an-unexpected-content-type) gagal pada percobaan pertama, karena gateway atau proxy yang menulis ulang respons akan menulis ulang percobaan ulang dengan cara yang sama. Memerlukan Claude Code v2.1.208 atau lebih baru.

Saat mencoba ulang, spinner menampilkan hitungan mundur `Retrying in Ns · attempt x/y` setelah label kesalahan. Label menyebutkan alasan spesifik dari percobaan pertama untuk kegagalan yang dapat Anda tindaklanjuti segera: jaringan tidak aktif, handshake TLS gagal, atau Anda mencapai batas laju. Untuk kesalahan lainnya, itu berbunyi `API error` pada awalnya. Mulai dari v2.1.198, itu beralih ke alasan spesifik dari percobaan ketiga, atau pada percobaan terakhir ketika `CLAUDE_CODE_MAX_RETRIES` memungkinkan lebih sedikit dari tiga; versi sebelumnya hanya beralih pada percobaan terakhir.

Mulai dari v2.1.198, tip spinner biasa ditekan selama percobaan ulang. Setelah alasan kesalahan terungkap, jika kegagalan adalah kelebihan beban 529, baris di bawah hitungan mundur juga menyebutkan di mana memeriksa status layanan: `status.claude.com` di API Anthropic, atau host penyedia atau gateway yang disebutkan dalam pesan pada konfigurasi lain.

Jika tidak ada data yang tiba di aliran respons selama 20 detik sementara permintaan masih tertunda, spinner menampilkan `Waiting for API response · will retry in … · check your network` sebelum percobaan ulang apa pun dimulai. Permintaan belum gagal: hitungan mundur berjalan ke titik di mana Claude Code membatalkan koneksi yang macet dan mencoba ulang, sehingga banner hilang dengan sendirinya setelah data dilanjutkan atau percobaan ulang berhasil. Mulai dari v2.1.185 ambang batasnya adalah 20 detik; versi sebelumnya menampilkan banner setelah 10 detik dengan wording yang berbeda. Jika muncul kembali pada setiap percobaan, perlakukan sebagai [masalah jaringan](#unable-to-connect-to-api).

Ketika Anda melihat salah satu kesalahan di halaman ini, percobaan ulang tersebut telah habis, kecuali jika itu termasuk kelas yang tidak dicoba ulang, seperti kegagalan validasi sertifikat. Anda dapat menyesuaikan perilaku dengan variabel lingkungan ini:

| Variabel                                     | Default | Efek                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| :------------------------------------------- | :------ | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [`CLAUDE_CODE_MAX_RETRIES`](/docs/id/env-vars)    | 10      | Jumlah percobaan ulang. Dibatasi pada 15 mulai dari v2.1.186; mulai dari v2.1.199 `CLAUDE_CODE_RETRY_WATCHDOG` menaikkan default dan menghapus batas. Turunkan untuk menampilkan kegagalan lebih cepat dalam skrip.                                                                                                                                                                                                                                                                                                    |
| [`CLAUDE_CODE_RETRY_WATCHDOG`](/docs/id/env-vars) | unset   | Atur ke `1` dalam sesi tanpa pengawasan seperti pekerjaan CI untuk mencoba ulang kesalahan kapasitas `429` dan `529` tanpa batas daripada gagal setelah percobaan `CLAUDE_CODE_MAX_RETRIES`. Mulai dari v2.1.199, itu juga menaikkan jumlah percobaan ulang default untuk kesalahan transien lainnya, seperti kesalahan server, timeout, dan koneksi yang terputus, menjadi 300, kira-kira tiga jam backoff, dan menghapus batas 15 pada `CLAUDE_CODE_MAX_RETRIES` jika Anda menetapkan variabel itu secara eksplisit. |
| [`API_TIMEOUT_MS`](/docs/id/env-vars)             | 600000  | Waktu tunggu per permintaan dalam milidetik. Naikkan untuk jaringan lambat atau proxy.                                                                                                                                                                                                                                                                                                                                                                                                                                 |

<h2 id="server-errors">
  Kesalahan server
</h2>

Kesalahan ini berasal dari penyedia inferensi daripada akun atau permintaan Anda. Pada Anthropic API itu berarti infrastruktur Anthropic. Pada Amazon Bedrock, Agent Platform Google Cloud, Microsoft Foundry, atau gateway khusus itu berarti infrastruktur penyedia tersebut.

<h3 id="api-error-500-internal-server-error">
  API Error: 500 Internal server error
</h3>

Claude Code menampilkan kode status dan pesan kesalahan API untuk respons 5xx apa pun. Contoh di bawah menunjukkan respons 500 pada Anthropic API:

```text theme={null}
API Error: 500 Internal server error. This is a server-side issue, usually temporary — try again in a moment. If it persists, check https://status.claude.com.
```

Kalimat terakhir menyebutkan tempat untuk memeriksa kesehatan layanan dan bervariasi menurut penyedia. Konfigurasi Amazon Bedrock, Agent Platform Google Cloud, dan Microsoft Foundry menyebutkan status layanan penyedia tersebut. `ANTHROPIC_BASE_URL` khusus menyebutkan host gateway.

Ini menunjukkan kegagalan yang tidak terduga di dalam API. Ini tidak disebabkan oleh prompt, pengaturan, atau akun Anda.

**Yang harus dilakukan:**

* Periksa [status.claude.com](https://status.claude.com), atau halaman status penyedia yang disebutkan dalam pesan, untuk insiden aktif
* Tunggu satu menit, kemudian kirim pesan Anda lagi. Pesan asli Anda masih ada dalam percakapan, jadi untuk prompt yang panjang Anda dapat mengetik `try again` daripada menempel seluruh hal tersebut.
* Jika kesalahan berlanjut tanpa ada insiden yang diposting, jalankan `/feedback` sehingga Anthropic dapat menyelidiki dengan detail permintaan Anda. Lihat [Report an error](#report-an-error) jika `/feedback` tidak tersedia di lingkungan Anda.

<h3 id="api-error-repeated-529-overloaded-errors">
  API Error: Repeated 529 Overloaded errors
</h3>

API sementara mencapai kapasitas di semua pengguna. Claude Code telah mencoba ulang beberapa kali sebelum menampilkan pesan ini:

```text theme={null}
API Error: Repeated 529 Overloaded errors. The API is at capacity — this is usually temporary. Try again in a moment. If it persists, check https://status.claude.com.
```

Kalimat terakhir bervariasi menurut penyedia dengan cara yang sama seperti kesalahan 500 di atas.

529 bukan batas penggunaan Anda dan tidak dihitung terhadap kuota Anda.

**Yang harus dilakukan:**

* Periksa [status.claude.com](https://status.claude.com), atau halaman status penyedia yang disebutkan dalam pesan, untuk pemberitahuan kapasitas
* Coba lagi dalam beberapa menit
* Jalankan `/model` dan beralih ke model yang berbeda untuk terus bekerja, karena kapasitas dilacak per model. Claude Code meminta Anda untuk melakukan ini ketika satu model mengalami beban yang sangat tinggi, misalnya `Opus is experiencing high load, please use /model to switch to Sonnet`.

<h3 id="request-timed-out">
  Request timed out
</h3>

API tidak merespons sebelum batas waktu koneksi.

```text theme={null}
Request timed out
```

Ini dapat terjadi selama periode beban tinggi atau ketika model menghasilkan respons yang sangat besar. Waktu tunggu permintaan default adalah 10 menit.

**Yang harus dilakukan:**

* Coba ulang permintaan
* Untuk tugas yang berjalan lama, pecah pekerjaan menjadi prompt yang lebih kecil
* Jika jaringan lambat atau proxy adalah penyebabnya, naikkan `API_TIMEOUT_MS` seperti yang dijelaskan dalam [Automatic retries](#automatic-retries)
* Jika waktu tunggu sering terjadi dan jaringan Anda sehat, lihat [Network and connection errors](#network-and-connection-errors) di bawah

<h3 id="the-response-above-may-be-incomplete">
  The response above may be incomplete
</h3>

Respons streaming gagal setelah Claude telah menghasilkan output yang terlihat. Mengirim ulang permintaan dapat menjalankan panggilan alat yang sama dua kali, jadi Claude Code menyimpan apa yang sudah streaming dan menambahkan pemberitahuan ini sebagai gantinya dari membuang giliran. Varian mana yang Anda lihat menyebutkan penyebabnya:

```text theme={null}
API Error: Server error mid-response. The response above may be incomplete.
API Error: Connection closed mid-response. The response above may be incomplete.
API Error: Response stalled mid-stream. The response above may be incomplete.
```

* `Server error mid-response`: kesalahan server overloaded atau 5xx mid-stream. Varian ini memerlukan Claude Code v2.1.199 atau lebih baru; sebelumnya kasus itu membuang output parsial dan melaporkan seluruh giliran sebagai kesalahan.
* `Connection closed mid-response`: koneksi terputus.
* `Response stalled mid-stream`: aliran berhenti mengirim data.

**Yang harus dilakukan:**

* Baca respons yang streaming. Tidak ada yang hilang, tetapi kalimat atau panggilan alat terakhir mungkin hilang.
* Balas dengan `continue` untuk membuat Claude melanjutkan dari tempat ia berhenti
* Jika kesalahan yang sama muncul sebelum output yang terlihat, Claude Code mencoba ulang permintaan daripada menyelesaikannya. Lihat [Automatic retries](#automatic-retries).

<h3 id="auto-mode-cannot-determine-the-safety-of-an-action">
  Auto mode cannot determine the safety of an action
</h3>

Model yang digunakan [auto mode](/docs/id/permission-modes#eliminate-prompts-with-auto-mode) untuk mengklasifikasikan tindakan tidak dapat menghasilkan keputusan, jadi auto mode tidak menyetujui tindakan secara otomatis. Pesan yang Anda lihat tergantung pada alasan pengklasifikasi gagal.

Pembacaan, pencarian, dan pengeditan di dalam direktori kerja Anda melewati pengklasifikasi, jadi mereka terus bekerja dalam semua kasus ini.

Ketika model pengklasifikasi kelebihan beban:

```text theme={null}
<model> is temporarily unavailable, so auto mode cannot determine the safety of <tool> right now. Wait briefly and then try this action again.
```

**Yang harus dilakukan:**

* Coba ulang setelah beberapa detik; Claude melihat pesan yang sama dan biasanya mencoba ulang sendiri
* Jika percobaan ulang terus gagal, lanjutkan dengan tugas baca-saja dan kembali ke tindakan yang diblokir nanti
* Ini bersifat sementara dan tidak terkait dengan [auto mode eligibility](/docs/id/permission-modes#eliminate-prompts-with-auto-mode); Anda tidak perlu mengubah pengaturan

Ketika pengklasifikasi mengembalikan respons yang tidak dapat diuraikan:

```text theme={null}
Auto mode could not evaluate this action and is blocking it for safety — run with --debug for details
```

**Yang harus dilakukan:**

* Coba ulang tindakan; ini biasanya berhasil pada percobaan berikutnya
* Jalankan `claude --debug` dan ulangi tindakan untuk melihat respons pengklasifikasi yang mendasar dalam log debug

Ketika pemeriksaan keamanan API terpisah memblokir permintaan pengklasifikasi karena konten percakapan sebelumnya:

```text theme={null}
Auto mode could not evaluate this action and is blocking it for safety — a safety check separate from auto mode blocked this request because of earlier conversation content — it isn't about the action itself — run with --debug for details
```

**Yang harus dilakukan:**

* Ini bukan keputusan tentang tindakan Anda. Konten yang sudah ada dalam percakapan Anda memicu filter keamanan pada API ketika auto mode mengirim percakapan ke pengklasifikasi
* Mencoba ulang tidak akan membantu; konten percakapan yang sama akan memicu filter lagi
* Beralih ke [permission mode](/docs/id/permission-modes) yang berbeda sehingga Anda dapat menyetujui tindakan ketika diminta, atau mulai percakapan baru tanpa konten pemicu

Ketika percakapan telah tumbuh lebih besar dari jendela konteks pengklasifikasi:

```text theme={null}
Auto mode classifier transcript exceeded context window — falling back to manual approval (try /compact to reduce conversation size)
```

Dalam sesi interaktif, auto mode kembali ke prompt izin normal untuk tindakan itu sehingga Anda dapat menyetujui atau menolaknya secara manual. Dalam [non-interactive mode](/docs/id/headless) jalankan dibatalkan karena transkrip hanya tumbuh dan mencoba ulang tidak dapat berhasil.

**Yang harus dilakukan:**

* Setujui atau tolak tindakan dalam prompt yang muncul
* Jalankan `/compact` untuk mengurangi ukuran percakapan sehingga tindakan berikutnya sesuai dengan jendela pengklasifikasi lagi

<h3 id="agent-terminated-early-due-to-an-api-error">
  Agent terminated early due to an API error
</h3>

Permintaan API [subagent](/docs/id/sub-agents) gagal secara terminal, misalnya karena batas penggunaan tercapai atau percobaan ulang untuk kesalahan server habis, jadi subagent berhenti sebelum menyelesaikan tugasnya. Pesan ini memerlukan Claude Code v2.1.199 atau lebih baru; sebelumnya teks kesalahan API dikembalikan ke Claude seolah-olah itu adalah hasil subagent.

```text theme={null}
Agent terminated early due to an API error: <error detail>
```

**Yang harus dilakukan:**

* Cocokkan detail kesalahan setelah titik dua dengan bagiannya sendiri di halaman ini, seperti [Usage limits](#usage-limits) atau [Server errors](#server-errors), dan ikuti langkah-langkah bagian itu
* Setelah kesalahan yang mendasar hilang, minta Claude untuk mencoba ulang tugas atau [resume the subagent](/docs/id/sub-agents#resume-subagents)

Ketika batas laju, overload, atau kesalahan server mengganggu subagent latar depan yang sudah menghasilkan output teks, Claude menerima output parsial itu ditandai sebagai tidak lengkap daripada kesalahan ini. Subagent yang satu-satunya output adalah panggilan alat juga mendapatkan kesalahan ini; dalam v2.1.199 bentuk itu mengembalikan hasil parsial kosong sebagai gantinya. Lihat [API errors in subagents](/docs/id/sub-agents#api-errors-in-subagents).

<h2 id="usage-limits">
  Batas penggunaan
</h2>

Kesalahan ini berarti kuota yang terikat pada akun atau paket Anda telah tercapai. Kesalahan ini berbeda dari [kesalahan server](#server-errors), yang mempengaruhi semua orang.

<h3 id="youve-hit-your-session-limit">
  Anda telah mencapai batas sesi
</h3>

Paket langganan mencakup tunjangan penggunaan bergulir. Ketika habis, Anda akan melihat salah satu pesan berikut:

```text theme={null}
You've hit your session limit · resets 3:45pm
You've hit your weekly limit · resets Mon 12:00am
You've hit your Opus limit · resets 3:45pm
```

Claude Code memblokir permintaan lebih lanjut hingga waktu reset yang ditunjukkan dalam pesan. Batas sesi dan mingguan dibagikan di semua model, jadi beralih model tidak memulihkan akses. Batas Opus hanya berlaku untuk permintaan Opus, jadi beralih ke model lain dengan `/model` membuat Anda tetap bekerja.

Penggunaan dihitung terhadap tunjangan sesi dan mingguan pada saat yang sama. Satu ledakan aktivitas berat, seperti fanout alur kerja besar, dapat menghabiskan tunjangan mingguan sebelum jendela sesi direset.

**Yang harus dilakukan:**

* Tunggu waktu reset yang ditunjukkan dalam kesalahan
* Untuk batas Opus, jalankan `/model` dan beralih ke model lain untuk terus bekerja
* Jalankan `/usage` untuk melihat batas paket dan kapan mereka direset
* Jalankan `/usage-credits` untuk membeli penggunaan tambahan pada Pro dan Max, atau untuk memintanya dari admin Anda pada Team dan Enterprise. Lihat [usage credits untuk paket berbayar](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) untuk cara penagihan ini.
* Untuk meningkatkan paket Anda untuk batas dasar yang lebih tinggi, lihat [claude.com/pricing](https://claude.com/pricing)

Untuk memantau tunjangan sisa Anda sebelum mencapai batas, tambahkan bidang `rate_limits` ke [baris status kustom](/docs/id/statusline#rate-limit-usage), atau di aplikasi Desktop klik [cincin penggunaan](/docs/id/desktop#check-usage) di sebelah pemilih model.

<h3 id="usage-credits-required-for-1m-context">
  Kredit penggunaan diperlukan untuk konteks 1M
</h3>

Model yang dipilih menggunakan jendela konteks diperpanjang 1M-token, dan paket Anda hanya mencakupnya melalui kredit penggunaan.

```text theme={null}
API Error: Usage credits required for 1M context · run /usage-credits to turn them on, or /model to switch to standard context
```

Ini adalah pemeriksaan hak, bukan kelelahan kuota. Ini terjadi bahkan ketika tunjangan sesi dan mingguan Anda memiliki kapasitas yang tersisa. Lihat [Extended context](/docs/id/model-config#extended-context) untuk paket mana yang mencakup konteks 1M secara langsung dan mana yang memerlukan kredit penggunaan.

Ketika kesalahan ini muncul di tengah percakapan karena konteks tumbuh melampaui 200K token, Claude Code secara otomatis memadatkan percakapan kembali di bawah batas konteks standar dan menjaga sesi pada batas itu setelahnya, jadi tidak ada tindakan yang diperlukan. Pada versi sebelum v2.1.172, kesalahan berulang pada setiap permintaan berikutnya termasuk `/compact`; jalankan `/clear` pada versi tersebut untuk pulih. Langkah-langkah di bawah berlaku ketika Anda secara eksplisit memilih model `[1m]`.

**Yang harus dilakukan:**

* Jalankan `/model` dan pilih varian tanpa akhiran `[1m]` untuk kembali ke jendela konteks standar
* Jalankan `/usage-credits` untuk mengaktifkan penagihan terukur untuk varian 1M pada Pro dan Max, atau untuk memintanya dari admin Anda pada Team dan Enterprise
* Jika kesalahan berlanjut setelah `/model`, ID model 1M mungkin diatur di tempat lain. Lihat [There's an issue with the selected model](#theres-an-issue-with-the-selected-model) untuk lokasi konfigurasi yang harus diperiksa dalam urutan prioritas.
* Untuk menghapus varian 1M dari pemilih model sepenuhnya, atur [`CLAUDE_CODE_DISABLE_1M_CONTEXT=1`](/docs/id/env-vars)

<h3 id="server-is-temporarily-limiting-requests">
  Server sedang membatasi permintaan sementara
</h3>

API menerapkan throttle berumur pendek yang tidak terkait dengan kuota paket Anda.

```text theme={null}
API Error: Server is temporarily limiting requests (not your usage limit)
```

Claude Code membedakan ini dari batas paket Anda dengan tidak adanya header kuota terpadu yang dibawa respons batas nyata. Mulai dari v2.1.199 ini [dicoba ulang secara otomatis](#automatic-retries) dengan backoff sebelum ditampilkan, cara apa pun Anda melakukan autentikasi. Pada versi sebelumnya, sesi yang masuk dengan langganan claude.ai gagal giliran pada kemunculan pertama; hanya API key dan Enterprise sign-ins yang mencoba ulangnya.

**Yang harus dilakukan:**

* Tunggu sebentar dan coba lagi
* Periksa [status.claude.com](https://status.claude.com) jika berlanjut

<h3 id="request-rejected-429">
  Permintaan ditolak (429)
</h3>

Anda telah mencapai batas laju yang dikonfigurasi untuk kunci API, proyek Amazon Bedrock, atau proyek Google Cloud Anda.

```text theme={null}
API Error: Request rejected (429) · this may be a temporary capacity issue. If it persists, check https://status.claude.com.
```

Kalimat trailing menunjukkan di mana memeriksa kesehatan layanan dan bervariasi menurut penyedia. Amazon Bedrock, Agent Platform Google Cloud, dan konfigurasi Microsoft Foundry menyebutkan status layanan penyedia itu sebagai gantinya dari halaman status Anthropic. `ANTHROPIC_BASE_URL` kustom menyebutkan host gateway.

**Yang harus dilakukan:**

* Jalankan `/status` dan konfirmasi kredensial aktif adalah yang Anda harapkan. `ANTHROPIC_API_KEY` yang tersesat di lingkungan Anda dapat merutekan permintaan melalui kunci tingkat rendah alih-alih langganan Anda.
* Periksa konsol penyedia Anda untuk batas aktif dan minta tingkat yang lebih tinggi jika diperlukan
* Untuk kunci API Anthropic, lihat [referensi batas laju](https://platform.claude.com/docs/en/api/rate-limits) untuk cara kerja tingkat dan cara menetapkan batas per-workspace
* Kurangi concurrency: turunkan [`CLAUDE_CODE_MAX_TOOL_USE_CONCURRENCY`](/docs/id/env-vars), hindari menjalankan banyak subagen paralel, atau beralih ke model yang lebih kecil dengan `/model` untuk run skrip volume tinggi

<h3 id="credit-balance-is-too-low">
  Saldo kredit terlalu rendah
</h3>

Organisasi Console Anda telah kehabisan kredit prabayar.

```text theme={null}
Credit balance is too low
```

**Yang harus dilakukan:**

* Tambahkan kredit di [platform.claude.com/settings/billing](https://platform.claude.com/settings/billing), dan pertimbangkan untuk mengaktifkan auto-reload di sana sehingga saldo terisi ulang sebelum mencapai nol
* Beralih ke autentikasi langganan dengan `/login` jika Anda memiliki paket Pro, Max, Team, atau Enterprise
* Atur batas pengeluaran per-workspace di Console untuk mencegah satu proyek menguras saldo org. Lihat [Manage costs effectively](/docs/id/costs).

<h2 id="authentication-errors">
  Kesalahan autentikasi
</h2>

Kesalahan ini berarti Claude Code tidak dapat membuktikan identitas Anda kepada API. Jalankan `/status` kapan saja untuk melihat kredensial mana yang saat ini aktif.

<h3 id="not-logged-in">
  Belum masuk
</h3>

Tidak ada kredensial yang valid tersedia untuk sesi ini.

```text theme={null}
Not logged in · Please run /login
```

**Yang harus dilakukan:**

* Jalankan `/login` untuk autentikasi dengan langganan Claude Anda atau akun Console
* Jika Anda mengharapkan variabel lingkungan untuk mengautentikasi Anda, konfirmasi bahwa `ANTHROPIC_API_KEY` diatur dan diekspor di shell tempat Anda meluncurkan `claude`
* Untuk CI atau otomasi di mana login interaktif tidak mungkin, konfigurasikan skrip [`apiKeyHelper`](/docs/id/settings#available-settings) yang mengambil kunci saat startup
* Lihat [Prioritas autentikasi](/docs/id/authentication#authentication-precedence) untuk memahami kredensial mana yang digunakan Claude Code ketika beberapa tersedia

Jika Anda diminta untuk masuk berulang kali, lihat [Belum masuk atau token kedaluwarsa](/docs/id/troubleshoot-install#not-logged-in-or-token-expired) untuk perbaikan jam sistem dan Keychain macOS.

<h3 id="could-not-resolve-authentication-method">
  Tidak dapat menyelesaikan metode autentikasi
</h3>

Sesi mencapai klien API tanpa kredensial apa pun. Ini muncul di [sesi latar belakang](/docs/id/agent-view), sesi cloud, dan konteks Agent SDK di mana pemeriksaan login interaktif tidak berjalan sebelum permintaan pertama.

```text theme={null}
Could not resolve authentication method. Expected one of apiKey, authToken, credentials, config, or profile to be set. Or for one of the "X-Api-Key" or "Authorization" headers to be explicitly omitted
```

Sebelum v2.1.174, sesi latar belakang atau cloud yang ditugaskan ke pekerja yang sudah diinisialisasi sebelumnya yang menganggur dapat gagal dengan cara ini bahkan ketika kredensial yang valid dikonfigurasi. Tingkatkan untuk memulihkan. Pada versi saat ini, kesalahan berarti tidak ada kredensial yang tersedia untuk proses pekerja.

**Yang harus dilakukan:**

* Tingkatkan ke v2.1.174 atau lebih baru jika ini muncul di sesi latar belakang atau cloud dan kredensial Anda sudah dikonfigurasi
* Konfirmasi bahwa `ANTHROPIC_API_KEY`, `CLAUDE_CODE_OAUTH_TOKEN`, atau kredensial penyedia cloud Anda diatur di lingkungan yang meluncurkan pekerja, bukan hanya di shell interaktif Anda
* Untuk Agent SDK, lihat [pengaturan autentikasi](/docs/id/agent-sdk/overview#get-started)
* Jalankan `/status` dalam sesi interaktif di lingkungan yang sama untuk mengonfirmasi sumber kredensial mana yang diselesaikan

<h3 id="invalid-api-key">
  Kunci API tidak valid
</h3>

Variabel lingkungan `ANTHROPIC_API_KEY` atau skrip `apiKeyHelper` mengembalikan kunci yang ditolak API.

```text theme={null}
Invalid API key · Fix external API key
```

**Yang harus dilakukan:**

* Periksa kesalahan ketik dan konfirmasi bahwa kunci belum dicabut di [Console](https://platform.claude.com/settings/keys)
* Jalankan `env | grep ANTHROPIC` di shell yang sama. Alat seperti direnv, plugin shell dotenv, dan terminal IDE dapat memuat kunci yang sudah usang dari file `.env` di proyek Anda tanpa Anda mengaturnya secara eksplisit.
* Batalkan pengaturan `ANTHROPIC_API_KEY` dan jalankan `/login` untuk menggunakan autentikasi langganan sebagai gantinya
* Jika kunci berasal dari skrip [`apiKeyHelper`](/docs/id/settings#available-settings), jalankan skrip secara langsung untuk mengonfirmasi bahwa skrip mencetak kunci yang valid di stdout
* Jalankan `/status` untuk mengonfirmasi sumber kredensial mana yang sebenarnya digunakan Claude Code

<h3 id="your-apikeyhelper-script-is-failing">
  Skrip apiKeyHelper Anda gagal
</h3>

Perintah yang dikonfigurasi dalam pengaturan [`apiKeyHelper`](/docs/id/settings#available-settings) keluar dengan kesalahan, habis waktu, atau tidak mencetak apa pun ke stdout. Tanpa kunci dari skrip, permintaan mencapai API dengan kredensial placeholder, dan API menolaknya dengan `401`.

```text theme={null}
Your apiKeyHelper script is failing · This usually means you need to re-authenticate with your provider · Run /status to see the script's error output
```

Claude Code menjalankan kembali skrip dan mencoba ulang permintaan hingga dua kali lagi sebelum menampilkan pesan ini, jadi kegagalan muncul dalam tiga upaya. Sebelum v2.1.208, Claude Code menghabiskan [anggaran percobaan ulang](#automatic-retries) penuh mengirim ulang permintaan dengan kredensial placeholder dan kemudian melaporkan kesalahan autentikasi `401` generik daripada kegagalan skrip.

Menjalankan `/login` tidak membantu di sini: output helper [mengambil prioritas](/docs/id/authentication#authentication-precedence) atas login yang disimpan selama pengaturan ada.

**Yang harus dilakukan:**

* Jalankan perintah yang dikonfigurasi dalam `apiKeyHelper` secara langsung di shell Anda untuk mereproduksi kegagalan
* Jika perintah melaporkan sesi yang kedaluwarsa, autentikasi ulang dengan penyedia kredensial Anda, misalnya dengan masuk kembali ke SSO atau vault rahasia Anda
* Perbaiki perintah sehingga mencetak kunci ke stdout dan keluar dengan kode 0. Lihat [putar kredensial dengan apiKeyHelper](/docs/id/llm-gateway-connect#rotate-credentials-with-apikeyhelper) untuk pengaturan yang berfungsi.
* Jalankan `/status` untuk mengonfirmasi bahwa `apiKeyHelper` adalah sumber kredensial aktif. Setiap kali perintah gagal, kode keluar dan output kesalahannya muncul di panel `Cloud authentication` di terminal.

<h3 id="this-organization-has-been-disabled">
  Organisasi ini telah dinonaktifkan
</h3>

Kunci `ANTHROPIC_API_KEY` yang sudah usang dari organisasi Console yang dinonaktifkan menimpa login langganan Anda.

```text theme={null}
Your ANTHROPIC_API_KEY belongs to a disabled organization · Unset the environment variable to use your other credentials
API Error: 400 ... This organization has been disabled.
```

Variabel lingkungan memiliki prioritas lebih tinggi daripada `/login`, jadi kunci yang diekspor di profil shell Anda atau dimuat dari file `.env` digunakan bahkan ketika Anda memiliki langganan Pro atau Max yang berfungsi. Dalam mode non-interaktif (`-p`), kunci selalu digunakan saat ada.

**Yang harus dilakukan:**

* Batalkan pengaturan `ANTHROPIC_API_KEY` di shell saat ini dan hapus dari profil shell Anda, kemudian luncurkan kembali `claude`
* Jalankan `/status` setelahnya untuk mengonfirmasi bahwa kredensial aktif adalah langganan Anda
* Jika tidak ada variabel lingkungan yang diatur dan kesalahan berlanjut, organisasi yang dinonaktifkan adalah organisasi yang terikat pada `/login` Anda. Hubungi dukungan atau masuk dengan akun yang berbeda.

<h3 id="your-organization-has-disabled-api-key-authentication">
  Organisasi Anda telah menonaktifkan autentikasi kunci API
</h3>

Pesan ini memerlukan Claude Code v2.1.169 atau lebih baru. Admin organisasi Console Anda telah mematikan autentikasi kunci API, jadi API menolak kunci yang dikirim Claude Code. Petunjuk pemulihan setelah `·` bervariasi tergantung di mana kunci berasal:

```text theme={null}
Your organization has disabled API key authentication · Run /login to sign in with your claude.ai account
Your organization has disabled API key authentication · Unset ANTHROPIC_API_KEY to use your claude.ai account instead
Your organization has disabled API key authentication · Unset ANTHROPIC_API_KEY and run /login to sign in with your claude.ai account
Your organization has disabled API key authentication · Unset the apiKeyHelper setting and run /login to sign in with your claude.ai account
```

Variabel lingkungan dan `apiKeyHelper` memiliki prioritas lebih tinggi daripada `/login`, jadi menjalankan `/login` saja tidak membantu sementara salah satu masih menyuplai kunci. Lihat [Prioritas autentikasi](/docs/id/authentication#authentication-precedence).

**Yang harus dilakukan:**

* Jika pesan menyebutkan `ANTHROPIC_API_KEY`, batalkan pengaturannya di shell saat ini dan hapus dari profil shell Anda atau file `.env`, kemudian luncurkan kembali `claude`
* Jika pesan menyebutkan `apiKeyHelper`, hapus pengaturan [`apiKeyHelper`](/docs/id/settings#available-settings) dari `settings.json` Anda
* Jalankan `/login` untuk masuk dengan akun claude.ai Anda
* Jalankan `/status` setelahnya untuk mengonfirmasi bahwa kredensial aktif adalah langganan Anda daripada kunci API
* Jika Anda memerlukan autentikasi kunci API untuk otomasi, minta admin organisasi Anda untuk mengaktifkannya kembali di Console

<h3 id="your-organization-has-disabled-claude-subscription-access">
  Organisasi Anda telah menonaktifkan akses langganan Claude
</h3>

Organisasi Claude Anda tidak mengizinkan masuk ke Claude Code dengan login langganan. Menjalankan `/login` lagi dengan akun yang sama mengembalikan kesalahan yang sama.

```text theme={null}
Your organization has disabled Claude subscription access for Claude Code · Use an Anthropic API key instead, or ask your admin to enable access
```

Ini adalah pengaturan organisasi sisi server, jadi tidak dapat ditimpa dari pengaturan lokal, variabel lingkungan, atau bendera CLI.

Agent SDK dan mode non-interaktif `-p` menampilkan ini sebagai kode kesalahan `oauth_org_not_allowed`.

**Yang harus dilakukan:**

* Minta admin Anda untuk mengaktifkan akses Claude Code untuk organisasi Anda
* Autentikasi dengan kunci API Console daripada langganan Anda. Lihat [autentikasi Claude Console](/docs/id/authentication#claude-console-authentication) untuk pengaturan.
* Jika Anda adalah admin dan tidak melihat opsi untuk mengaktifkan akses, hubungi [dukungan Anthropic](https://support.claude.com)

<h3 id="routines-are-disabled-by-your-organizations-policy">
  Rutinitas dinonaktifkan oleh kebijakan organisasi Anda
</h3>

Pemilik di organisasi Tim atau Enterprise Anda telah mematikan rutinitas di tingkat organisasi. Kesalahan muncul ketika Anda mencoba membuat atau menjalankan rutinitas, termasuk dari `/schedule` dan UI [Rutinitas](/docs/id/routines) di claude.ai/code.

```text theme={null}
Routines are disabled by your organization's policy.
```

Ini adalah pengaturan sisi server, jadi tidak dapat ditimpa dari pengaturan lokal, variabel lingkungan, atau bendera CLI.

**Yang harus dilakukan:**

* Minta Pemilik di organisasi Anda untuk mengaktifkan toggle **Rutinitas** di [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code)
* Untuk pekerjaan terjadwal sekali pakai yang tidak memerlukan rutinitas tingkat organisasi, lihat [tugas terjadwal](/docs/id/scheduled-tasks)

<h3 id="remote-control-requires-the-anthropic-api">
  Remote Control memerlukan API Anthropic
</h3>

Sesi tidak berbicara langsung ke API Anthropic, jadi tidak ada backend claude.ai untuk [Remote Control](/docs/id/remote-control) untuk dipasangkan.

```text theme={null}
Remote Control is only available when using Claude via api.anthropic.com.
```

Ini muncul di Amazon Bedrock, Platform Agen Google Cloud, dan Microsoft Foundry. Mulai dari v2.1.196, ini juga muncul ketika [`ANTHROPIC_BASE_URL`](/docs/id/env-vars) menunjuk ke host selain `api.anthropic.com`, seperti [gateway LLM](/docs/id/llm-gateway) atau proxy, bahkan ketika Anda masuk dengan claude.ai.

**Yang harus dilakukan:**

* Batalkan pengaturan `ANTHROPIC_BASE_URL` dan mulai ulang sesi, atau mulai Remote Control dari sesi yang berbicara langsung ke API Anthropic
* Untuk pesan startup Remote Control ini dan lainnya, lihat [Troubleshoot Remote Control](/docs/id/remote-control#troubleshooting)

<h3 id="oauth-token-revoked-or-expired">
  Token OAuth dicabut atau kedaluwarsa
</h3>

Login yang disimpan Anda tidak lagi valid. Token yang dicabut berarti Anda keluar di mana-mana atau admin menghapus akses; token yang kedaluwarsa berarti penyegaran otomatis gagal di tengah sesi.

Kedua pesan melaporkan penolakan yang dikembalikan API untuk permintaan yang dikirim Claude Code. Ketika login yang disimpan sudah dihapus setelah penyegaran yang gagal, Anda melihat [Login kedaluwarsa](#login-expired) sebagai gantinya.

```text theme={null}
OAuth token revoked · Please run /login
OAuth token has expired · Please run /login
API Error: 401 ... authentication_error
```

**Yang harus dilakukan:**

* Jalankan `/login` untuk masuk lagi
* Jika kesalahan kembali dalam sesi yang sama setelah autentikasi ulang, jalankan `/logout` terlebih dahulu untuk sepenuhnya menghapus token yang disimpan, kemudian `/login`
* Untuk prompt login berulang di seluruh peluncuran, lihat pemeriksaan jam sistem dan Keychain macOS di [Troubleshooting](/docs/id/troubleshoot-install#not-logged-in-or-token-expired)
* Untuk kegagalan lainnya termasuk `403 Forbidden` dan masalah browser OAuth, lihat [Login dan autentikasi](/docs/id/troubleshoot-install#login-and-authentication)

<h3 id="login-expired">
  Login kedaluwarsa
</h3>

Claude Code mencoba memperbarui login claude.ai atau Claude Console yang disimpan dan layanan OAuth menolak token penyegaran yang disimpan, jadi Claude Code menghapus kredensial yang disimpan. Setelah itu, setiap permintaan berhenti secara lokal sebelum mencapai API, karena hanya `/login` yang dapat membuat kredensial baru. Sebelum v2.1.206, Claude Code mengirim permintaan bagaimanapun dengan kredensial apa pun yang tersisa di lingkungan, dan setiap model kemudian gagal dengan [Ada masalah dengan model yang dipilih](#theres-an-issue-with-the-selected-model) atau 401 daripada prompt untuk masuk.

```text theme={null}
Login expired · Please run /login
```

Dalam [mode non-interaktif](/docs/id/headless) (`-p`) dan [Agent SDK](/docs/id/agent-sdk/overview), pesan berbunyi sebagai berikut, dan kode kesalahan terstruktur adalah `authentication_failed`:

```text theme={null}
Failed to authenticate: OAuth session expired and could not be refreshed
```

Ini bukan keadaan yang sama dengan [Token OAuth dicabut atau kedaluwarsa](#oauth-token-revoked-or-expired). Pesan tersebut melaporkan 401 yang dikembalikan API. Claude Code sendiri menghasilkan `Login expired` untuk login yang sudah gagal diperbaharui, jadi tidak mengirim permintaan.

Sesi yang diautentikasi dengan kunci API, [`CLAUDE_CODE_OAUTH_TOKEN`](/docs/id/env-vars), atau penyedia pihak ketiga tidak menggunakan login yang disimpan dan tidak pernah melihat pesan ini.

**Yang harus dilakukan:**

* Jalankan `/login` untuk masuk lagi. Mencoba ulang tanpa masuk menunjukkan pesan yang sama pada setiap permintaan.
* Dalam mode non-interaktif, jalankan `claude` di lingkungan yang sama, selesaikan `/login`, kemudian jalankan kembali perintah Anda. Untuk otomasi yang tidak dapat masuk secara interaktif, autentikasi dengan `ANTHROPIC_API_KEY` atau [buat token jangka panjang dengan `claude setup-token`](/docs/id/authentication#generate-a-long-lived-token).
* Jika masuk terus gagal, lihat [Login dan autentikasi](/docs/id/troubleshoot-install#login-and-authentication)

<h3 id="oauth-scope-requirement">
  Persyaratan cakupan OAuth
</h3>

Token yang disimpan mendahului cakupan izin yang diperlukan fitur yang lebih baru. Anda paling sering melihat ini dari `/usage` dan indikator penggunaan baris status:

```text theme={null}
OAuth token does not meet scope requirement: user:profile
```

**Yang harus dilakukan:**

* Jalankan `/login` untuk mendapatkan token baru dengan cakupan saat ini. Anda tidak perlu keluar terlebih dahulu.

<h3 id="aws-credentials-expired-or-invalid">
  Kredensial AWS kedaluwarsa atau tidak valid
</h3>

Pesan ini memerlukan Claude Code v2.1.198 atau lebih baru dan hanya muncul ketika [`awsAuthRefresh`](/docs/id/amazon-bedrock#advanced-credential-configuration) diatur dalam file pengaturan Anda. Token sesi AWS Anda kedaluwarsa atau ditolak, dan penyegaran otomatis yang sudah dijalankan Claude Code tidak menghasilkan kredensial yang diterima API. Ini muncul pada 401 dari [Claude Platform on AWS](/docs/id/claude-platform-on-aws) atau [endpoint Mantle](/docs/id/amazon-bedrock#use-the-mantle-endpoint), yang merupakan cara penyedia tersebut melaporkan token keamanan yang kedaluwarsa.

Petunjuk tindakan di tengah menyebutkan perintah `awsAuthRefresh` dari pengaturan Anda, jadi bervariasi. Bagian yang stabil adalah `AWS credentials expired or invalid` di awal:

```text theme={null}
AWS credentials expired or invalid · run /login and select "Claude Platform on AWS · refresh credentials", or run `aws sso login --profile myprofile` in another terminal · API Error: 401 ...
```

Tanpa `awsAuthRefresh` yang dikonfigurasi, 401 yang sama menunjukkan pesan generik `Please run /login` sebagai gantinya, yang tidak dapat menyegarkan kredensial AWS.

**Yang harus dilakukan:**

* Jalankan perintah `awsAuthRefresh` yang disebutkan dalam pesan, seperti `aws sso login --profile myprofile`, di terminal lain dan selesaikan masuk browser, kemudian coba lagi
* Dalam sesi interaktif, jalankan `/login`, pilih **platform pihak ketiga**, kemudian pilih **Claude Platform on AWS · refresh credentials** di bawah **Menggunakan platform pihak ketiga** untuk menjalankan perintah yang sama tanpa memulai ulang Claude Code. Lihat [Konfigurasi kredensial AWS](/docs/id/claude-platform-on-aws#1-configure-aws-credentials)
* Jika kesalahan berulang setelah perintah penyegaran berhasil, konfirmasi identitas valid di luar Claude Code dengan `aws sts get-caller-identity` di shell dan profil yang sama

<h3 id="aws-authentication-failed">
  Autentikasi AWS gagal
</h3>

Pesan ini memerlukan Claude Code v2.1.198 atau lebih baru dan hanya muncul ketika [`awsAuthRefresh`](/docs/id/amazon-bedrock#advanced-credential-configuration) diatur dalam file pengaturan Anda. Penyedia AWS Anda mengembalikan 403, atau [Amazon Bedrock](/docs/id/amazon-bedrock) mengembalikan 401.

Claude Code tidak dapat mengetahui penyebab mana yang Anda alami. Amazon Bedrock melaporkan token keamanan yang kedaluwarsa sebagai 403, tetapi 403 juga merupakan cara melaporkan penolakan otorisasi, seperti `AccessDeniedException` dari izin IAM yang hilang atau model yang tidak diaktifkan untuk akun Anda.

401 dari Amazon Bedrock juga mendarat di sini daripada di bawah [Kredensial AWS kedaluwarsa atau tidak valid](#aws-credentials-expired-or-invalid), karena Amazon Bedrock tidak melaporkan token yang kedaluwarsa sebagai 401. 401 dari endpoint tersebut biasanya berasal dari sesuatu yang lain di jalur permintaan, seperti proxy perusahaan.

Penyegaran kredensial memperbaiki token yang kedaluwarsa dan tidak dapat memperbaiki penyebab lainnya, jadi pesan menawarkan keduanya:

```text theme={null}
AWS authentication failed · run /login and select "Claude Platform on AWS · refresh credentials", or run `aws sso login --profile myprofile` in another terminal · if credentials are current, check AWS permissions and model access · API Error: 403 ...
```

Petunjuk tindakan di tengah menyebutkan perintah `awsAuthRefresh` dari pengaturan Anda, jadi bervariasi. Bagian yang stabil adalah `AWS authentication failed` di awal.

**Yang harus dilakukan:**

* Jalankan perintah `awsAuthRefresh` yang disebutkan dalam pesan, atau `aws sso login`, untuk berjaga-jaga jika kredensial yang kedaluwarsa adalah penyebabnya
* Jika kredensial Anda saat ini, konfirmasi izin IAM di [Konfigurasi IAM](/docs/id/amazon-bedrock#iam-configuration) terlampir pada identitas yang Anda gunakan dan bahwa model yang dipilih diaktifkan untuk akun dan wilayah Anda
* Jalankan `aws sts get-caller-identity` untuk mengonfirmasi identitas mana yang digunakan permintaan Anda; `AWS_PROFILE` yang sudah usang atau profil default adalah penyebab umum ketidakcocokan izin

<h3 id="aws-default-chain-credential-resolve-timed-out">
  Penyelesaian kredensial rantai default AWS habis waktu
</h3>

Rantai penyedia kredensial default AWS tidak menghasilkan kredensial dalam 60 detik, jadi Claude Code menghentikan penyelesaian dan gagal permintaan. Kegagalan adalah penyelesaian kredensial lokal: permintaan tidak pernah mencapai [Amazon Bedrock](/docs/id/amazon-bedrock), [Claude Platform on AWS](/docs/id/claude-platform-on-aws), atau [endpoint Mantle](/docs/id/amazon-bedrock#use-the-mantle-endpoint). Claude Code menghapus [cache kredensial](/docs/id/amazon-bedrock#credential-caching-and-resolution-timeout) dan mencoba ulang sebelum kesalahan ini muncul, jadi pada saat Anda melihatnya rantai telah macet pada upaya berulang.

```text theme={null}
API Error: AWS default-chain credential resolve timed out
```

Penyebab umum adalah perintah `credential_process` di profil AWS Anda yang menunggu input yang tidak dapat diterima, dan kontainer atau VM yang layanan metadata instans (IMDS) tidak pernah menjawab probe rantai. Sebelum v2.1.207, rantai yang macet membiarkan permintaan menunggu tanpa batas waktu daripada gagal dengan pesan ini.

**Yang harus dilakukan:**

* Jalankan `aws sts get-caller-identity` di shell yang sama dengan `AWS_PROFILE` yang sama. Jika juga hang, perbaiki profil; perintah `credential_process` yang meminta secara interaktif adalah penyebab umum.
* Selesaikan langkah masuk sebelum memulai Claude Code, misalnya `aws sso login --profile myprofile`, sehingga rantai diselesaikan dari cache SSO lokal daripada menunggu alur browser
* Jika rantai Anda menjalankan masuk interaktif yang secara sah memerlukan lebih dari 60 detik, seperti SSO dengan MFA melalui wrapper seperti `aws-vault`, naikkan batas dalam milidetik dengan [`CLAUDE_CODE_AWS_CHAIN_RESOLVE_TIMEOUT_MS`](/docs/id/env-vars)

<h2 id="network-and-connection-errors">
  Kesalahan jaringan dan koneksi
</h2>

Kesalahan ini berarti permintaan jaringan dari Claude Code gagal mencapai tujuannya, atau sesuatu antara Claude Code dan API mengubah respons dalam perjalanannya kembali. Biasanya berasal dari jaringan lokal Anda, proxy, atau firewall, atau dari kebijakan jaringan lingkungan cloud.

<h3 id="unable-to-connect-to-api">
  Tidak dapat terhubung ke API
</h3>

Koneksi TCP ke API gagal atau tidak pernah selesai.

```text theme={null}
Unable to connect to API. Check your internet connection
Unable to connect to API (ECONNREFUSED)
Unable to connect to API (ECONNRESET)
Unable to connect to API (ETIMEDOUT)
fetch failed
Request timed out. Check your internet connection and proxy settings
```

Penyebab umum termasuk tidak ada akses internet, VPN yang memblokir `api.anthropic.com`, atau proxy perusahaan yang diperlukan tetapi tidak dikonfigurasi.

**Yang harus dilakukan:**

* Konfirmasi Anda dapat menjangkau host API dari shell yang sama dengan menjalankan `curl -I https://api.anthropic.com`. Di Windows PowerShell gunakan `curl.exe -I https://api.anthropic.com` sehingga alias `Invoke-WebRequest` bawaan tidak digunakan.
* Jika Anda berada di belakang proxy perusahaan, atur `HTTPS_PROXY` sebelum meluncurkan Claude Code dan lihat [Konfigurasi jaringan](/docs/id/network-config)
* Jika Anda merutekan melalui gateway LLM atau relay, atur [`ANTHROPIC_BASE_URL`](/docs/id/env-vars) ke alamatnya. Lihat [Hubungkan Claude Code ke gateway LLM](/docs/id/llm-gateway-connect) untuk pengaturan.
* Pastikan firewall Anda memungkinkan host yang tercantum dalam [Persyaratan akses jaringan](/docs/id/network-config#network-access-requirements)
* Kegagalan intermiten [diulang secara otomatis](#automatic-retries); kegagalan persisten menunjukkan masalah jaringan lokal

Jika `curl` berhasil tetapi Claude Code masih gagal, penyebabnya biasanya sesuatu antara runtime dan jaringan daripada jaringan itu sendiri:

* Di Linux dan WSL, periksa `/etc/resolv.conf` untuk nameserver yang tidak dapat dijangkau. WSL khususnya dapat mewarisi resolver yang rusak dari host.
* Di macOS, klien VPN yang terputus atau dihapus dapat meninggalkan antarmuka tunnel atau aturan routing. Periksa `ifconfig` untuk antarmuka `utun` yang sudah usang dan hapus ekstensi jaringan VPN di System Settings.
* Docker Desktop dan runtime container serupa dapat mengintersepsi lalu lintas keluar. Tutup mereka dan coba lagi untuk mengesampingkan ini.

<h3 id="bedrock-streaming-response-has-an-unexpected-content-type">
  Bedrock streaming response memiliki content-type yang tidak terduga
</h3>

Gateway atau proxy antara Claude Code dan [Amazon Bedrock](/docs/id/amazon-bedrock) mengubah badan respons streaming atau header `Content-Type`-nya. Amazon Bedrock melakukan streaming respons sebagai `application/vnd.amazon.eventstream`, dan Claude Code menolak respons streaming yang berhasil yang melaporkan content-type berbeda daripada mendekode badan yang tidak dapat dibacanya. Permintaan tidak diulang.

```text theme={null}
Bedrock streaming response has content-type "text/event-stream"; expected "application/vnd.amazon.eventstream". A gateway or proxy between Claude Code and Bedrock is likely transforming the response body — Bedrock's binary event-stream format must be passed through unmodified. Set CLAUDE_CODE_DISABLE_BEDROCK_CONTENT_TYPE_GUARD=1 to suppress this check while the gateway is being fixed.
```

Sebelum v2.1.208, konfigurasi yang sama muncul sebagai `API Error: Truncated event message received` setelah seluruh respons telah di-buffer.

**Yang harus dilakukan:**

* Konfigurasikan gateway untuk melewatkan badan respons `InvokeModelWithResponseStream` dan header `Content-Type`-nya tanpa diubah. Perantara yang memancarkan kembali aliran sebagai server-sent events adalah penyebab umum.
* Jika gateway hanya menulis ulang header dan melewatkan badan biner tanpa diubah, atur [`CLAUDE_CODE_DISABLE_BEDROCK_CONTENT_TYPE_GUARD=1`](/docs/id/env-vars) untuk melewati pemeriksaan sampai gateway diperbaiki. Lihat [Streaming errors behind a gateway or proxy](/docs/id/amazon-bedrock#streaming-errors-behind-a-gateway-or-proxy).

<h3 id="ssl-certificate-errors">
  Kesalahan sertifikat SSL
</h3>

Proxy atau perangkat keamanan di jaringan Anda mengintersepsi lalu lintas TLS dengan sertifikatnya sendiri, dan Claude Code tidak mempercayainya.

```text theme={null}
Unable to connect to API: SSL certificate verification failed. Check your proxy or corporate SSL certificates
Unable to connect to API: Self-signed certificate detected
```

Mulai dari v2.1.199, kegagalan validasi sertifikat tidak diulang, jadi kesalahan ini muncul pada percobaan pertama daripada setelah [anggaran retry](#automatic-retries) penuh. Versi sebelumnya menghabiskan beberapa menit untuk mencoba ulang sebelum menampilkannya. Kondisi TLS sementara, seperti timeout handshake, masih mencoba ulang.

Selama `/login` dan pemeriksaan konektivitas startup, kegagalan yang sama dilaporkan dengan kode OpenSSL dan perbaikan inline:

```text theme={null}
SSL certificate error (UNABLE_TO_GET_ISSUER_CERT_LOCALLY). If you are behind a corporate proxy or TLS-intercepting firewall, set NODE_EXTRA_CA_CERTS to your CA bundle path, or ask IT to allowlist *.anthropic.com. Run `claude doctor` for details.
```

**Yang harus dilakukan:**

* Ekspor bundle CA organisasi Anda dan arahkan Claude Code ke sana dengan `NODE_EXTRA_CA_CERTS=/path/to/ca-bundle.pem`
* Lihat [Konfigurasi jaringan](/docs/id/network-config#custom-ca-certificates) untuk instruksi pengaturan lengkap
* Jangan atur `NODE_TLS_REJECT_UNAUTHORIZED=0`, yang menonaktifkan validasi sertifikat sepenuhnya

<h3 id="host-not-allowed-in-a-cloud-session">
  Host tidak diizinkan dalam sesi cloud
</h3>

Permintaan HTTP keluar dari sesi cloud atau routine diblokir oleh kebijakan jaringan lingkungan.

```text theme={null}
HTTP 403
x-deny-reason: host_not_allowed
```

Anda juga dapat melihat sertifikat TLS yang tidak cocok dengan sertifikat asli tujuan. Lingkungan cloud merutekan lalu lintas keluar melalui proxy yang memberlakukan kebijakan jaringan, jadi sertifikat yang tidak cocok berarti proxy mengakhiri koneksi, bukan tujuan.

Ini bukan masalah jaringan sisi klien. Sesi cloud dan [routines](/docs/id/routines) berjalan di dalam lingkungan sandbox yang lalu lintas keluarnya disaring ke daftar izin lingkungan. Lingkungan **Default** menggunakan akses **Trusted**, yang memungkinkan [daftar izin default](/docs/id/claude-code-on-the-web#default-allowed-domains) dari registri paket, API penyedia cloud, registri container, dan domain pengembangan umum tetapi memblokir semuanya.

**Yang harus dilakukan:**

* Buka routine untuk diedit, atau mulai sesi cloud. Pilih ikon cloud yang menampilkan nama lingkungan Anda, seperti **Default**, untuk membuka pemilih. Arahkan ke lingkungan Anda dan klik ikon pengaturan.
* Dalam dialog **Update cloud environment**, ubah **Network access** dari **Trusted** ke **Custom**, kemudian tambahkan domain yang diblokir ke **Allowed domains**. Masukkan satu domain per baris. Centang **Also include default list of common package managers** untuk menyimpan [daftar izin default](/docs/id/claude-code-on-the-web#default-allowed-domains) bersama domain kustom Anda. Pilih **Full** sebagai gantinya jika Anda menginginkan akses tanpa batas.
* Klik **Save changes**. Jalankan berikutnya menggunakan daftar izin yang diperbarui.

Lihat [Network access](/docs/id/claude-code-on-the-web#network-access) untuk tingkat akses dan daftar izin default. Sesi CLI lokal tidak terpengaruh oleh kebijakan ini.

<h3 id="couldnt-reconnect-to-your-remote-control-session">
  Tidak dapat terhubung kembali ke sesi Remote Control Anda
</h3>

```text theme={null}
Couldn't reconnect to your Remote Control session. Retry, or start a fresh session without --resume.
```

Melanjutkan dengan `claude --resume` atau `claude --continue` terhubung kembali ke sesi [Remote Control](/docs/id/remote-control) yang dicatat dalam percakapan itu. Pesan ini berarti reconnection gagal karena alasan yang mungkin bersifat sementara, seperti gangguan jaringan atau kesalahan server, jadi Claude Code tidak dapat mengkonfirmasi apakah sesi remote masih ada. Sesi lokal Anda terus berjalan tanpa Remote Control.

**Yang harus dilakukan:**

* Jalankan `/remote-control` untuk mencoba ulang koneksi
* Mulai Claude Code tanpa `--resume` untuk membuat sesi Remote Control baru
* Untuk pesan startup Remote Control lainnya, lihat [Troubleshoot Remote Control](/docs/id/remote-control#troubleshooting)

Anda tidak akan melihat pesan ini ketika server mengkonfirmasi sesi sebelumnya tidak lagi ada; Claude Code membuat yang baru dalam hal itu. Sebelum v2.1.200, kegagalan reconnection apa pun membuat sesi Remote Control baru, yang meninggalkan sesi ekstra dalam daftar sesi di claude.ai/code.

<h2 id="request-errors">
  Kesalahan permintaan
</h2>

Kesalahan ini berkaitan dengan konten permintaan Anda. Sebagian besar kembali dari API setelah menolak permintaan; beberapa diproduksi secara lokal oleh Claude Code sebelum permintaan apa pun dikirim.

<h3 id="prompt-is-too-long">
  Prompt terlalu panjang
</h3>

Percakapan ditambah file terlampir melebihi jendela konteks model.

```text theme={null}
Prompt is too long
```

**Yang harus dilakukan:**

* Jalankan `/compact` untuk merangkum giliran sebelumnya dan membebaskan ruang, atau `/clear` untuk memulai dari awal
* Jalankan `/context` untuk melihat rincian apa yang mengonsumsi jendela: prompt sistem, alat, file memori, dan pesan
* Nonaktifkan server MCP yang tidak Anda gunakan dengan `/mcp disable <name>` untuk menghapus definisi alat mereka dari konteks
* Pangkas file memori `CLAUDE.md` yang besar, atau pindahkan instruksi ke [aturan yang dibatasi jalur](/docs/id/memory#path-specific-rules) yang dimuat hanya ketika relevan
* Subagen mewarisi setiap definisi alat MCP dari sesi induk, yang dapat mengisi jendela konteks mereka sebelum giliran pertama. Nonaktifkan server MCP yang tidak Anda gunakan sebelum menelurkan subagen.
* Auto-compact diaktifkan secara default dan biasanya mencegah kesalahan ini. Jika Anda telah menetapkan [`DISABLE_AUTO_COMPACT`](/docs/id/env-vars), aktifkan kembali atau jalankan `/compact` secara manual sebelum jendela penuh.

Lihat [Jelajahi jendela konteks](/docs/id/context-window) untuk tampilan interaktif tentang bagaimana konteks terisi.

<h3 id="error-during-compaction-conversation-too-long">
  Kesalahan selama pemadatan: Percakapan terlalu panjang
</h3>

`/compact` itu sendiri gagal karena tidak ada cukup konteks gratis untuk menampung ringkasan yang dihasilkannya.

```text theme={null}
Error during compaction: Conversation too long. Press esc twice to go up a few messages and try again.
```

Ini dapat terjadi ketika jendela sudah penuh pada saat auto-compact dipicu, atau ketika Anda menjalankan `/compact` setelah melihat `Prompt is too long`.

**Yang harus dilakukan:**

* Tekan Esc dua kali untuk membuka daftar pesan dan mundur beberapa giliran. Ini menghilangkan pesan terbaru dari konteks. Kemudian jalankan `/compact` lagi.
* Jika mundur tidak membebaskan cukup ruang, jalankan `/clear` untuk memulai sesi baru. Percakapan sebelumnya Anda disimpan dan dapat dibuka kembali dengan `/resume`.

<h3 id="request-too-large">
  Permintaan terlalu besar
</h3>

Badan permintaan mentah melebihi batas byte API sebelum tokenisasi, biasanya karena file atau lampiran besar yang ditempel.

```text theme={null}
Request too large (max 30 MB). Double press esc to go back and remove or shrink the attached content.
```

Ini adalah batas ukuran pada permintaan HTTP, terpisah dari [batas jendela konteks](#prompt-is-too-long).

**Yang harus dilakukan:**

* Tekan Esc dua kali dan mundur melewati giliran yang menambahkan konten yang terlalu besar
* Referensikan file besar berdasarkan jalur alih-alih menempel konten mereka, sehingga Claude dapat membacanya dalam potongan
* Untuk gambar, lihat [Gambar terlalu besar](#image-was-too-large) di bawah

<h3 id="image-was-too-large">
  Gambar terlalu besar
</h3>

Gambar yang ditempel atau dilampirkan melebihi batas ukuran atau dimensi API.

```text theme={null}
Image was too large. Double press esc to go back and try again with a smaller image.
API Error: 400 ... image dimensions exceed max allowed size
```

Claude Code mengganti gambar yang tidak dapat diproses dengan placeholder teks dan mencoba lagi, sehingga pesan berikutnya berhasil. Pada versi sebelum 2.1.142, gambar yang ditempel dapat tetap berada dalam percakapan dan mengulangi kesalahan yang sama pada setiap pesan berikutnya. Untuk pulih pada versi tersebut, tekan Esc dua kali dan mundur melewati giliran tempat gambar ditambahkan.

**Yang harus dilakukan:**

* Ubah ukuran gambar sebelum menempel. API menerima gambar hingga 8000 piksel di tepi terpanjang untuk satu gambar, atau 2000 piksel ketika banyak gambar berada dalam konteks.
* Ambil tangkapan layar yang lebih ketat dari wilayah yang relevan alih-alih layar penuh

<h3 id="unable-to-resize-image">
  Tidak dapat mengubah ukuran gambar
</h3>

Claude Code tidak dapat mengurangi skala gambar yang dilampirkan sebelum mengirimnya ke API.

```text theme={null}
Unable to resize image — image processing is unavailable and dimensions could not be read from the file header. Please convert the image to PNG, JPEG, GIF, or WebP.
Unable to resize image — dimensions exceed the 2000x2000px limit and image processing failed. Please resize the image to reduce its pixel dimensions.
Unable to resize image (… raw, … base64). The image exceeds the … API limit and compression failed. Please resize the image manually or use a smaller image.
Unable to resize image — could not verify image dimensions are within the 2000x2000px API limit.
```

Claude Code biasanya mengubah ukuran gambar besar secara otomatis. Kesalahan ini berarti pemroses gambar asli gagal dimuat atau mengembalikan kesalahan, sehingga gambar tidak dapat diubah ukurannya agar sesuai dengan batas API.

**Yang harus dilakukan:**

* Jika pesan meminta Anda untuk mengonversi gambar, konversikan ke PNG, JPEG, GIF, atau WebP dan lampirkan lagi. Claude Code dapat memverifikasi dimensi untuk format ini tanpa pemroses gambar.
* Jika pesan melaporkan batas dimensi atau ukuran, ubah ukuran atau kompres ulang gambar di bawah batas tersebut sebelum melampirkan.

<h3 id="pdf-errors">
  Kesalahan PDF
</h3>

PDF yang Anda lampirkan tidak dapat diproses.

```text theme={null}
PDF too large (max 100 pages, 32 MB). Try splitting it or extracting text first.
PDF is password protected. Try removing protection or extracting text first.
The PDF file was not valid. Try converting to a different format first.
```

**Yang harus dilakukan:**

* Untuk PDF yang terlalu besar, minta Claude untuk membaca rentang halaman dengan alat Read alih-alih melampirkan seluruh file, atau ekstrak teks dengan alat seperti `pdftotext` dan referensikan file output berdasarkan jalur
* Untuk PDF yang dilindungi atau tidak valid, hapus kata sandi atau ekspor ulang file dari aplikasi sumbernya, kemudian coba lagi

<h3 id="extra-inputs-are-not-permitted">
  Input tambahan tidak diizinkan
</h3>

Proxy atau gateway LLM antara Claude Code dan API menghapus header permintaan `anthropic-beta`, sehingga API menolak bidang yang bergantung padanya.

```text theme={null}
API Error: 400 ... Extra inputs are not permitted ... context_management
API Error: 400 ... Extra inputs are not permitted ... tools.0.custom.input_examples
API Error: 400 ... Unexpected value(s) for the `anthropic-beta` header
```

Claude Code mengirim bidang khusus beta seperti `context_management`, `effort`, dan `input_examples` alat bersama dengan header `anthropic-beta` yang mengaktifkannya. Ketika gateway meneruskan badan tetapi menghapus header, API melihat bidang yang tidak dikenalinya.

**Yang harus dilakukan:**

* Konfigurasikan gateway Anda untuk meneruskan header `anthropic-beta`. Lihat [feature pass-through](/docs/id/llm-gateway-protocol#feature-pass-through) untuk apa yang harus diteruskan gateway.
* Sebagai fallback, atur [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1`](/docs/id/env-vars) sebelum meluncurkan. Ini menonaktifkan fitur yang memerlukan header beta sehingga permintaan berhasil melalui gateway yang tidak dapat meneruskannya.

<h3 id="theres-an-issue-with-the-selected-model">
  Ada masalah dengan model yang dipilih
</h3>

Nama model yang dikonfigurasi tidak dikenali atau akun Anda tidak memiliki akses ke model tersebut. Mulai dari v2.1.160 petunjuk trailing, ditampilkan di sini dalam bentuk interaktifnya, bervariasi menurut permukaan.

```text theme={null}
There's an issue with the selected model (claude-...). It may not exist or you may not have access to it. Run /model to pick a different model.
```

**Yang harus dilakukan:**

* **CLI Interaktif**: jalankan `/model` untuk memilih dari model yang tersedia untuk akun Anda.
* **Mode non-interaktif (`-p`)**: teruskan `--model` dengan alias atau ID yang valid, atau atur [`ANTHROPIC_MODEL`](/docs/id/env-vars). Teks kesalahan menunjukkan `Run --model` di permukaan ini.
* **Agent SDK**: teks kesalahan menghilangkan petunjuk karena model diatur secara terprogram. Atur [`model` pada `Options`](/docs/id/agent-sdk/typescript#options) di TypeScript atau [`ClaudeAgentOptions(model=...)`](/docs/id/agent-sdk/python#claudeagentoptions) di Python, dan tangani kesalahan terstruktur `model_not_found` untuk menampilkan pengambil ulang atau pemilih model Anda sendiri.
* Gunakan alias seperti `sonnet` atau `opus` alih-alih ID versi lengkap. Alias diselesaikan ke default yang dipertahankan sehingga tidak menjadi usang. Lihat [Konfigurasi model](/docs/id/model-config).
* Jika model yang salah terus kembali di CLI, ID yang usang diatur di suatu tempat. Periksa dalam [urutan prioritas](/docs/id/model-config#setting-your-model): flag `--model`, variabel lingkungan `ANTHROPIC_MODEL`, kemudian bidang `model` di `.claude/settings.local.json`, `.claude/settings.json` proyek Anda, dan `~/.claude/settings.json`. Hapus nilai yang usang dan Claude Code kembali ke default akun Anda.
* Claude Code melaporkan login claude.ai yang kedaluwarsa sebagai [Login expired](#login-expired), bukan sebagai kesalahan ini. Sebelum v2.1.206, login yang kedaluwarsa yang tidak lagi dapat disegarkan gagal di setiap model dengan kesalahan ini; jalankan `/login` jika Anda melihat itu di versi yang lebih lama.
* Untuk penyebaran Google Cloud's Agent Platform, lihat [Pemecahan masalah Google Cloud's Agent Platform](/docs/id/google-vertex-ai#troubleshooting).

<h3 id="model-is-not-a-recognized-model-id">
  Model bukan ID model yang dikenali
</h3>

String model yang Anda teruskan ke sakelar model bukan alias model, ID model yang diketahui versi Claude Code ini, atau ID yang dimulai dengan `claude-`. Penyebab umum adalah kesalahan ketik dalam ID, nama tampilan seperti `Sonnet 5` di mana ID `claude-sonnet-5` diharapkan, atau alias yang hanya dikenali versi Claude Code yang lebih baru. Claude Code menolak sakelar segera. Sebelum v2.1.200, Claude Code menyimpan string dan gagal pada permintaan berikutnya dengan [Ada masalah dengan model yang dipilih](#theres-an-issue-with-the-selected-model).

```text theme={null}
Model "claud-sonnet-5" is not a recognized model id. Did you mean 'claude-sonnet-5'?
```

Petunjuk trailing menamai alias atau ID model yang paling cocok. Ketika tidak ada yang cukup dekat, itu berbunyi `Run /model to see available models.` sebagai gantinya.

Claude Code menghasilkan kesalahan ini secara lokal pada saat sakelar diminta, sebelum permintaan API apa pun dibuat. Ini berlaku ketika model diatur melalui metode [Agent SDK](/docs/id/agent-sdk/typescript) `setModel()` atau oleh aplikasi seperti [Desktop app](/docs/id/desktop) yang menjalankan CLI Claude Code untuk Anda.

**Yang harus dilakukan:**

* Jalankan `/model` tanpa argumen untuk membuka pemilih dan pilih dari model yang tersedia untuk akun Anda, kemudian teruskan alias atau ID yang ditampilkan di sana
* Jika Anda menggunakan alias yang didukung versi Claude Code yang lebih baru, jalankan `claude update`. ID lengkap yang dimulai dengan `claude-` melewati pemeriksaan ini bahkan ketika model lebih baru dari versi Claude Code Anda, jadi upgrade tidak diperlukan untuk itu.
* Model yang disimpan sebelum v2.1.200 tidak diperbaiki oleh pemeriksaan ini. Jika nilai yang usang terus kembali, hapus dari lokasi yang tercantum di bawah [Ada masalah dengan model yang dipilih](#theres-an-issue-with-the-selected-model).
* Pemeriksaan hanya berjalan di Anthropic API. Di Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry, [Claude Platform on AWS](/docs/id/claude-platform-on-aws), dan di belakang [LLM gateway](/docs/id/llm-gateway) atau `ANTHROPIC_BASE_URL` kustom, penyedia atau gateway Anda menentukan nama model, jadi Claude Code menerima string apa pun dan meneruskannya.

<h3 id="claude-opus-is-not-available-with-the-claude-pro-plan">
  Claude Opus tidak tersedia dengan paket Claude Pro
</h3>

Paket langganan aktif Anda tidak mencakup model yang Anda pilih.

```text theme={null}
Claude Opus is not available with the Claude Pro plan · Select a different model in /model
```

**Yang harus dilakukan:**

* Jalankan `/model` dan pilih model yang disertakan paket Anda
* Jika Anda baru-baru ini meningkatkan paket dan masih melihat ini, jalankan `/logout` kemudian `/login`. Token yang disimpan mencerminkan paket Anda pada saat Anda masuk, jadi upgrade di web tidak berlaku dalam sesi yang ada sampai Anda mengautentikasi ulang.
* Lihat [claude.com/pricing](https://claude.com/pricing) untuk model mana yang disertakan setiap paket

<h3 id="model-is-restricted-by-your-organizations-settings">
  Model dibatasi oleh pengaturan organisasi Anda
</h3>

Admin organisasi Anda telah menonaktifkan model ini di konsol admin claude.ai, atau dikecualikan oleh daftar izin [`availableModels`](/docs/id/model-config#restrict-model-selection) dalam pengaturan terkelola. Ketika model yang dibatasi diatur dengan `--model`, `ANTHROPIC_MODEL`, atau pengaturan `model`, Claude Code mengganti model yang diizinkan dan melanjutkan. Mengetik `/model <name>` untuk model yang dibatasi ditolak dengan `Run /model to choose a different model.` dan sesi menyimpan model saat ini.

```text theme={null}
Model "claude-opus-4-8" is restricted by your organization's settings. Using claude-sonnet-4-6 instead.
```

Claude Code memperlakukan alias keluarga model, salah satu dari `opus`, `sonnet`, `haiku`, atau `fable`, sebagai permintaan untuk keluarga itu daripada versi terbarunya. Di Anthropic API dan di [Claude Platform on AWS](/docs/id/claude-platform-on-aws), alias keluarga yang dibatasi diselesaikan ke versi terbaru keluarga yang diizinkan organisasi Anda dan daftar izin `availableModels`, dan pemberitahuan substitusi menamai versi itu. Claude Code menolak `/model <alias>` hanya ketika setiap versi keluarga dibatasi. Sebelum v2.1.205, alias keluarga diganti atau ditolak berdasarkan versi terbarunya saja, bahkan ketika versi keluarga yang lebih lama diizinkan.

**Yang harus dilakukan:**

* Jalankan `/model` untuk memilih dari model yang diizinkan organisasi Anda. Model yang dibatasi disembunyikan dari pemilih.
* Jika model yang dibatasi diatur di `--model`, `ANTHROPIC_MODEL`, atau bidang `model` file pengaturan, hapus atau perbarui nilai itu sehingga pemberitahuan tidak berulang pada setiap peluncuran
* Jika Anda memerlukan akses ke model yang dibatasi, minta admin organisasi Anda untuk mengaktifkannya. Lihat [Pembatasan model organisasi](/docs/id/model-config#organization-model-restrictions).

<h3 id="thinking-type-enabled-is-not-supported-for-this-model">
  thinking.type.enabled tidak didukung untuk model ini
</h3>

Versi Claude Code Anda lebih lama dari minimum untuk Sonnet 5, Opus 4.8, atau Opus 4.7. CLI mengirim konfigurasi pemikiran yang model tidak lagi terima.

```text theme={null}
API Error: 400 ... "thinking.type.enabled" is not supported for this model. Use "thinking.type.adaptive" and "output_config.effort" to control thinking behavior.
```

**Yang harus dilakukan:**

* Jalankan `claude update` dan restart Claude Code. Opus 4.7 memerlukan v2.1.111 atau lebih baru. Opus 4.8 memerlukan v2.1.154 atau lebih baru. Sonnet 5 memerlukan v2.1.197 atau lebih baru
* Jika Anda tidak dapat upgrade, jalankan `/model` dan pilih Opus 4.6 atau Sonnet 4.6 sebagai gantinya
* Jika Anda mengalami ini di [Agent SDK](/docs/id/agent-sdk/overview), upgrade paket SDK sebagai gantinya. Opus 4.8 memerlukan TypeScript SDK v0.3.154 atau lebih baru dan Python SDK v0.2.88 atau lebih baru. Sonnet 5 memerlukan TypeScript SDK v0.3.197 atau lebih baru

<h3 id="thinking-budget-exceeds-output-limit">
  Anggaran pemikiran melebihi batas output
</h3>

Anggaran pemikiran yang diperpanjang yang dikonfigurasi melebihi panjang respons maksimum, sehingga tidak ada ruang yang tersisa untuk jawaban sebenarnya.

```text theme={null}
API Error: 400 ... max_tokens must be greater than thinking.budget_tokens
```

Claude Code menyesuaikan nilai-nilai ini secara otomatis di Anthropic API. Anda biasanya melihat kesalahan ini di Amazon Bedrock atau Google Cloud's Agent Platform ketika [`MAX_THINKING_TOKENS`](/docs/id/env-vars) diatur lebih tinggi dari batas output penyedia, atau ketika mode rencana menaikkan anggaran pemikiran.

**Yang harus dilakukan:**

* Turunkan `MAX_THINKING_TOKENS`, atau naikkan [`CLAUDE_CODE_MAX_OUTPUT_TOKENS`](/docs/id/env-vars) di atas anggaran pemikiran
* Lihat [Extended thinking](/docs/id/model-config#extended-thinking) untuk bagaimana anggaran berinteraksi dengan panjang output

<h3 id="tool-use-or-thinking-block-mismatch">
  Ketidakcocokan blok penggunaan alat atau pemikiran
</h3>

Riwayat percakapan mencapai API dalam keadaan tidak konsisten, biasanya setelah panggilan alat terputus atau giliran diedit di tengah aliran.

```text theme={null}
API Error: 400 due to tool use concurrency issues. Run /rewind to recover the conversation.
API Error: 400 ... unexpected `tool_use_id` found in `tool_result` blocks
API Error: 400 ... thinking blocks ... cannot be modified
```

Ketiga varian berarti hal yang sama: urutan blok `tool_use`, `tool_result`, dan `thinking` dalam riwayat tidak lagi cocok dengan apa yang diharapkan API.

**Yang harus dilakukan:**

* Jika Anda menggunakan Opus 4.7 atau Opus 4.8, jalankan `claude update` terlebih dahulu. Versi sebelum v2.1.156 dapat memicu kesalahan ini selama penggunaan alat normal, dan `/rewind` tidak menghapusnya.
* Jalankan `/rewind`, atau tekan Esc dua kali, untuk mundur ke checkpoint sebelum giliran yang rusak dan lanjutkan dari sana. Lihat [Checkpointing](/docs/id/checkpointing) untuk bagaimana checkpoint dibuat dan dipulihkan.

<h3 id="usage-policy-refusal">
  Penolakan Kebijakan Penggunaan
</h3>

API menolak untuk merespons karena konten dalam percakapan memicu pemeriksaan [Kebijakan Penggunaan](https://www.anthropic.com/legal/aup). Pesan mencakup ID Permintaan yang dapat Anda kutip untuk mendukung jika Anda percaya penolakan tidak benar.

```text theme={null}
API Error: Claude Code is unable to respond to this request, which appears to violate our Usage Policy (https://www.anthropic.com/legal/aup). Please double press esc to edit your last message or start a new session for Claude Code to assist with a different task.
```

Pemeriksaan mengevaluasi percakapan lengkap, bukan hanya prompt terbaru Anda, jadi mengirim pesan baru dalam sesi yang sama biasanya memicu penolakan yang sama. Hal yang sama berlaku setelah keluar dan membuka kembali sesi dengan `--continue` atau `--resume`, karena transkrip di disk masih berisi konten pemicu. Di [Amazon Bedrock](/docs/id/amazon-bedrock), [Google Cloud's Agent Platform](/docs/id/google-vertex-ai), dan [Microsoft Foundry](/docs/id/microsoft-foundry), pesan ini juga mencakup permintaan yang ditandai oleh langkah-langkah keselamatan model sebagai topik keamanan siber. Lihat [Langkah-langkah keselamatan menandai topik keamanan siber](#safety-measures-flagged-a-cybersecurity-topic).

**Yang harus dilakukan:**

* Tekan Esc dua kali atau jalankan `/rewind` untuk mundur ke checkpoint sebelum giliran yang memicu penolakan, kemudian rephrase atau ambil pendekatan berbeda. Lihat [Checkpointing](/docs/id/checkpointing).
* Jika Anda tidak dapat mengidentifikasi giliran mana yang menyebabkannya, jalankan `/clear` untuk memulai percakapan baru dalam proyek yang sama. Percakapan sebelumnya Anda disimpan di disk dan tetap tersedia di `/resume`.
* Dalam [mode non-interaktif](/docs/id/headless) (`-p`), di mana rewind tidak tersedia, coba lagi dengan prompt yang diucapkan ulang dalam sesi baru tanpa `--continue`. Pemeriksaan kebijakan bervariasi menurut model, jadi beralih ke model berbeda dengan `--model` juga dapat menyelesaikan penolakan dalam beberapa kasus.

<h3 id="safety-measures-flagged-a-cybersecurity-topic">
  Langkah-langkah keselamatan menandai topik keamanan siber
</h3>

Langkah-langkah keselamatan model menandai konten dalam percakapan sebagai topik keamanan siber. Pesan menamai model yang menandai permintaan:

```text theme={null}
API Error: Opus 4.8 has safety measures that flagged this message for a cybersecurity topic. To learn about the Cyber Verification Program and apply for access, visit our help center: https://support.claude.com/en/articles/14604842-real-time-cyber-safeguards-on-claude.

If you were not engaging in a cybersecurity topic, please send feedback via /feedback.
```

Pesan menghubungkan ke [Program Verifikasi Siber](https://support.claude.com/en/articles/14604842-real-time-cyber-safeguards-on-claude), yang memberikan akses untuk pekerjaan keamanan siber yang sah. Penjaga itu sendiri adalah server-side dan mendahului v2.1.203; rilis ini hanya mengubah kata-kata pesan dan halaman yang ditautkannya.

Apa yang Anda lihat tergantung pada penyedia dan mode Anda:

* Di [Amazon Bedrock](/docs/id/amazon-bedrock), [Google Cloud's Agent Platform](/docs/id/google-vertex-ai), dan [Microsoft Foundry](/docs/id/microsoft-foundry), bendera keamanan siber menghasilkan pesan [Penolakan Kebijakan Penggunaan](#usage-policy-refusal) sebagai gantinya.
* [Mode non-interaktif](/docs/id/headless) menghilangkan kalimat `/feedback`.

Sebelum v2.1.203, pesan berbunyi `<model>'s safeguards flagged this message for a cybersecurity topic. If your work requires this access, you can apply for an exemption:` diikuti oleh tautan formulir pengecualian.

**Yang harus dilakukan:**

* Jika pekerjaan Anda memerlukan konten ini, ajukan permohonan akses melalui [Program Verifikasi Siber](https://support.claude.com/en/articles/14604842-real-time-cyber-safeguards-on-claude)
* Jika permintaan Anda bukan tentang topik keamanan siber, jalankan `/feedback` untuk melaporkan positif palsu
* Untuk terus bekerja dalam sesi yang sama, tekan Esc dua kali atau jalankan `/rewind` untuk mundur ke checkpoint sebelum giliran yang memicu bendera, kemudian ambil pendekatan berbeda. Lihat [Checkpointing](/docs/id/checkpointing).

<h2 id="installation-errors">
  Kesalahan instalasi
</h2>

Kesalahan ini muncul saat menginstal atau memperbarui Claude Code, dari [skrip instalasi](/docs/id/setup#install-claude-code), `claude install`, atau `claude update`. Untuk masalah `command not found`, PATH, izin, dan TLS selama pengaturan, lihat [Troubleshoot installation and login](/docs/id/troubleshoot-install).

<h3 id="installation-was-killed-before-it-could-finish">
  Installation was killed before it could finish
</h3>

Skrip instalasi melaporkan ketika langkah `claude install` dihentikan oleh sinyal. Di Linux, kode keluar 137 berarti proses menerima SIGKILL, dan pada host dengan memori rendah itu biasanya pembunuh out-of-memory (OOM) kernel. Skrip mencetak penjelasan ini dan keluar dengan kode 137:

```text theme={null}
Installation was killed before it could finish (exit code 137). This usually means the system ran out of memory.
Claude Code needs roughly 512MB of free memory to install. Free up memory, then run this script again.
```

Untuk sinyal fatal lainnya, dan untuk kode keluar 137 di macOS, skrip mencetak `Installation was killed before it could finish (exit code <N>)` dengan kode keluar aktual dan menghilangkan penjelasan kehabisan memori. Pesan berasal dari skrip instalasi yang digunakan macOS dan Linux, yang juga mencakup instalasi di dalam WSL; skrip instalasi Windows asli tidak pernah mencetaknya. Sebelum v2.1.200, skrip keluar hanya dengan baris `Killed` shell yang kosong.

**Yang harus dilakukan:**

* Hentikan proses lain untuk membebaskan memori, kemudian jalankan kembali penginstal
* Tambahkan ruang swap atau pindah ke instans yang lebih besar. Lihat [Install killed on low-memory Linux servers](/docs/id/troubleshoot-install#install-killed-on-low-memory-linux-servers) untuk perintah file swap.

<h3 id="the-connection-dropped-while-downloading-the-update">
  The connection dropped while downloading the update
</h3>

Koneksi ke server unduhan ditutup saat `claude install`, `claude update`, atau [automatic updater](/docs/id/setup#auto-updates) mengambil biner Claude Code, dan pengulangan tidak berhasil. Claude Code mencoba ulang unduhan ketika koneksi putus, transfer macet, atau file yang diunduh gagal checksumnya, hingga tiga percobaan total. Kesalahan HTTP yang selesai, seperti 404, tidak dicoba ulang karena server sudah menjawab. Sebelum v2.1.202, koneksi yang putus tunggal gagal mengunduh segera dengan kesalahan kosong `aborted` alih-alih mencoba ulang.

```text theme={null}
The connection dropped while downloading the update (attempt 3/3: aborted). Check your network — proxies sometimes cut off large downloads.
```

Teks dalam tanda kurung menyebutkan percobaan mana yang gagal dan kesalahan jaringan yang mendasarinya. `claude update` mendahului pesan dengan `Error: Failed to install native update` di stderr.

Unduhan yang tetap terhubung tetapi tidak selesai dalam 10 menit gagal dengan `Download timed out: exceeded the total deadline` sebagai gantinya. Claude Code tidak mencoba ulang unduhan yang habis waktu, karena koneksi yang terlalu lambat untuk selesai dalam batas waktu tidak akan selesai pada pengulangan segera. Langkah-langkah di bawah berlaku untuk kedua pesan. Sebelum v2.1.205, batas waktu 10 menit yang sama dilaporkan sebagai `timeout of 600000ms exceeded` klien HTTP generik.

Penyebab umum adalah proxy atau gateway yang menutup transfer panjang sebelum selesai. Biner Claude Code adalah unduhan besar, jadi batas koneksi proxy yang tidak pernah mempengaruhi lalu lintas API normal masih dapat mengganggu.

**Yang harus dilakukan:**

* Jalankan `claude update` lagi. Pada jaringan yang sehat, unduhan biasanya berhasil pada run berikutnya. Untuk pesan yang habis waktu, jalankan lagi dari jaringan yang lebih cepat atau kurang dibatasi.
* Jika jaringan Anda memerlukan proxy, atur `HTTPS_PROXY` sebelum menjalankan penginstal atau `claude update`. Lihat [Check network connectivity](/docs/id/troubleshoot-install#check-network-connectivity).
* Jika proxy perusahaan terus menutup transfer, minta tim jaringan Anda untuk mengizinkan unduhan lengkap dari `downloads.claude.ai`. Lihat [Network access requirements](/docs/id/network-config#network-access-requirements).
* Jalankan `claude doctor` dari shell Anda untuk diagnostik instalasi

<h2 id="command-line-errors">
  Kesalahan baris perintah
</h2>

Kesalahan ini berasal dari baris perintah `claude` dan subperintahnya. Claude Code mencetaknya sebelum menjalankan prompt Anda atau mengirim permintaan API apa pun.

<h3 id="conflict-between-bg-and-print">
  Konflik antara --bg dan --print
</h3>

Pesan ini memerlukan Claude Code v2.1.198 atau lebih baru. Anda menggabungkan `--bg` dengan `-p` atau `--print` dalam invokasi `claude` yang sama. `--bg` memulai [sesi latar belakang](/docs/id/agent-view#from-your-shell) yang kemudian Anda lampirkan dengan `claude agents`, sementara `--print` berjalan [non-interaktif](/docs/id/headless) dan tidak pernah memulai sesi interaktif yang `claude agents` lampirkan. Sebelum v2.1.198, kombinasi ini secara diam-diam membuat pekerjaan latar belakang yang tidak pernah dapat dilampirkan.

```text theme={null}
--bg and --print conflict: --print never starts the interactive session that `claude agents` attaches to, so the job would be unattachable. The prompt is the positional — drop --print: `claude --bg '<task>'`.
```

**Yang harus dilakukan:**

* Hapus `-p` atau `--print`. `--bg` mengambil prompt sebagai argumen posisionalnya, jadi `claude --bg "<task>"` adalah perintah lengkapnya. Lihat [Dispatch new agents from your shell](/docs/id/agent-view#from-your-shell).
* Untuk menjalankan prompt secara non-interaktif dan mencetak hasilnya alih-alih membuat sesi latar belakang, hapus `--bg` dan jalankan `claude -p "<task>"`

<h3 id="the-json-schema-value-is-not-a-valid-json-schema">
  Nilai --json-schema bukan JSON Schema yang valid
</h3>

Skema yang Anda teruskan ke [`--json-schema`](/docs/id/cli-reference#cli-flags) dalam [mode non-interaktif](/docs/id/headless#get-structured-output) gagal kompilasi JSON Schema, jadi `claude` keluar dengan kode 1 alih-alih menjalankan prompt. Sebelum v2.1.205, skema yang tidak valid menghasilkan output tidak terstruktur tanpa kesalahan, dan skema apa pun yang menggunakan kata kunci `format` dianggap tidak valid.

```text theme={null}
Error: --json-schema is not a valid JSON Schema: data/type must be equal to one of the allowed values
```

Teks setelah titik dua kedua adalah diagnostik validator dan menyebutkan kata kunci atau lokasi yang gagal. Skema yang menggunakan kata kunci `format`, seperti `"format": "email"`, valid: Claude Code menerima `format` sebagai anotasi dan tidak memberlakukannya.

Claude Code menjalankan dua pemeriksaan sebelum kompilasi skema: ia menolak nilai yang tidak dapat diurai JSON dengan `Error: --json-schema is not valid JSON`, dan JSON yang valid tetapi bukan objek dengan `Error: --json-schema must be a JSON object`.

**Yang harus dilakukan:**

* Perbaiki bagian skema yang diagnostik sebutkan, kemudian jalankan kembali perintahnya
* Jika diagnostiknya adalah `schema too large`, kurangi nesting skema dan penggunaan ulang `$ref`
* Lihat [Get structured output](/docs/id/headless#get-structured-output) untuk skema dan perintah yang berfungsi

<h3 id="could-not-import-a-server-from-claude-desktop">
  Tidak dapat mengimpor server dari Claude Desktop
</h3>

Claude Code tidak dapat menambahkan salah satu server yang Anda pilih dalam `claude mcp add-from-claude-desktop`. Perintah masih mengimpor server yang dipilih lainnya dan mencetak satu baris per server yang tidak dapat ditambahkan. Sebelum v2.1.205, server pertama yang gagal menghentikan impor dan tidak ada server yang dipilih yang ditambahkan.

```text theme={null}
Could not import my server: Invalid name my server. Names can only contain letters, numbers, hyphens, and underscores.
```

Teks setelah nama server adalah alasannya. Yang paling umum adalah pemeriksaan nama: Claude Desktop memungkinkan karakter dalam nama server, seperti spasi dan titik, yang `claude mcp` batasi hanya untuk huruf, angka, tanda hubung, dan garis bawah. Alasan lain termasuk konfigurasi server yang gagal validasi dan server yang diblokir oleh [kebijakan MCP](/docs/id/managed-mcp) organisasi Anda.

**Yang harus dilakukan:**

* Ubah nama server di `claude_desktop_config.json` untuk hanya menggunakan huruf, angka, tanda hubung, dan garis bawah, kemudian jalankan `claude mcp add-from-claude-desktop` lagi
* Tambahkan server itu secara langsung dengan `claude mcp add` atau `claude mcp add-json` dengan nama yang valid. Lihat [Import MCP servers from Claude Desktop](/docs/id/mcp#import-mcp-servers-from-claude-desktop).

<h3 id="mcp-permission-prompt-tool-not-found">
  Alat prompt izin MCP tidak ditemukan
</h3>

Alat yang Anda teruskan ke [`--permission-prompt-tool`](/docs/id/cli-reference#cli-flags) tidak ada di antara alat MCP yang terhubung ketika jalankan pertama kali memerlukan keputusan izin, baik karena servernya tidak pernah terhubung atau karena tidak ada server yang terhubung yang mengekspos alat dengan nama itu. Claude Code masih mengirim prompt Anda: jalankan [non-interaktif](/docs/id/headless) keluar dengan kesalahan ini, dan kode keluar 1, pada panggilan alat pertama yang memerlukan persetujuan, jadi tidak menghasilkan jawaban meskipun permintaan telah dibuat. Sebelum prompt pertama, Claude Code menunggu hingga batas waktu koneksi per-server 30 detik yang ditetapkan oleh [`MCP_TIMEOUT`](/docs/id/env-vars) untuk server itu terhubung. Sebelum v2.1.206, startup tidak menunggu server selesai terhubung, jadi server yang dimulai lambat tetapi sehat menghasilkan kesalahan ini juga.

```text theme={null}
Error: MCP tool mcp__permissions__approve (passed via --permission-prompt-tool) not found. Available MCP tools: none
```

Daftar setelah `Available MCP tools:` menyebutkan alat MCP yang terhubung ketika penantian berakhir.

**Yang harus dilakukan:**

* Periksa bahwa server dimulai dan tetap terhubung: jalankan `claude mcp list` di direktori yang sama dan konfirmasi server terdaftar sebagai terhubung
* Konfirmkan nama alat cocok dengan nama `mcp__<server>__<tool>` yang server ekspos
* Jika server memerlukan lebih dari 30 detik untuk dimulai, naikkan [`MCP_TIMEOUT`](/docs/id/env-vars)

<h2 id="plugin-errors">
  Kesalahan plugin
</h2>

Kesalahan ini berasal dari konfigurasi [plugin](/docs/id/plugins) dan [marketplace](/docs/id/plugin-marketplaces). Untuk masalah plugin yang tidak menghasilkan salah satu pesan di halaman ini, seperti URL marketplace yang tidak memuat atau plugin yang terpasang tetapi tidak muncul, lihat [Troubleshooting plugin](/docs/id/discover-plugins#troubleshooting).

<h3 id="marketplace-is-registered-from-an-untrusted-source">
  Marketplace terdaftar dari sumber yang tidak terpercaya
</h3>

Marketplace terdaftar dengan nama yang [dicadangkan untuk marketplace resmi Anthropic](/docs/id/plugin-marketplaces#marketplace-schema), tetapi sumber terdaftarnya bukan repositori GitHub `anthropics`. Claude Code memeriksa ulang nama yang dicadangkan setiap kali memuat atau menyegarkan marketplace, sehingga marketplace dan plugin yang dipasang darinya berhenti memuat. Sebelum v2.1.205, nama hanya diperiksa ketika marketplace ditambahkan, sehingga entri yang terdaftar sebelum namanya dicadangkan terus memuat.

```text theme={null}
Marketplace "claude-community" is registered from an untrusted source: The name 'claude-community' is reserved for official Anthropic marketplaces. Only repositories from 'github.com/anthropics/' can use this name. To fix it, remove the marketplace and re-add it from the official source.
```

**Yang harus dilakukan:**

* Jalankan `claude plugin marketplace remove <name>`, kemudian tambahkan marketplace lagi dari repositori resmi `github.com/anthropics`
* Jika Anda menerbitkan marketplace pihak ketiga yang menggunakan nama sebelum nama tersebut dicadangkan, ubah namanya dan minta pengguna untuk menambahkannya kembali dari sumber Anda
* Lihat daftar nama yang dicadangkan di bawah [Marketplace schema](/docs/id/plugin-marketplaces#marketplace-schema)

<h3 id="plugin-command-references-user-config">
  Plugin command references user\_config in a shell command
</h3>

Hook plugin, [monitor](/docs/id/plugins-reference#monitors), atau perintah MCP [`headersHelper`](/docs/id/mcp#use-dynamic-headers-for-custom-authentication) mereferensikan opsi `${user_config.KEY}` [plugin](/docs/id/plugins-reference#user-configuration), dan string yang disubstitusi akan diteruskan ke shell. Nilai yang dikonfigurasi berisi `$(...)`, backtick, atau `;` akan berjalan sebagai kode di sana, jadi Claude Code menolak untuk memulai komponen alih-alih mensubstitusi nilai. Pemeriksaan berjalan pada template perintah, sehingga kesalahan muncul bahkan ketika tidak ada nilai yang dikonfigurasi. Sebelum v2.1.207, nilai disubstitusi ke dalam perintah shell.

Redaksi tergantung pada permukaan mana yang mereferensikan opsi. Hook bentuk shell melaporkan:

```text theme={null}
Hook from plugin formatter@acme-tools references ${user_config.*} in a shell-form command. The substituted value would be re-parsed by the shell. Use exec form instead — {"command": "<executable>", "args": ["${user_config.KEY}", ...]} — or read $CLAUDE_PLUGIN_OPTION_<KEY> from the hook's environment. Command: ./scripts/notify.sh ${user_config.webhook_url}
```

Monitor melaporkan:

```text theme={null}
Monitor "deploy-status" from plugin deploy-tools references ${user_config.*} in its command. The substituted value would be passed to a shell. Monitor commands cannot safely reference ${user_config.*}; have the monitor script read the value from a config file or prompt instead.
```

MCP `headersHelper` melaporkan:

```text theme={null}
headersHelper for MCP server 'internal-api' references ${user_config.*}. The substituted value would be passed to a shell; read the value inside the helper script instead (e.g. from an env var set in the server's "env" block).
```

**Yang harus dilakukan:**

* Untuk hook, tambahkan array `args` sehingga berjalan dalam [exec form](/docs/id/hooks#exec-form-and-shell-form), di mana setiap `${user_config.KEY}` menjadi satu argumen tanpa shell di antaranya. Atau lepaskan referensi dan baca variabel lingkungan `$CLAUDE_PLUGIN_OPTION_<KEY>` di dalam skrip
* Untuk monitor, lepaskan referensi dan buat skrip monitor membaca nilai dari file konfigurasi
* Untuk `headersHelper`, pindahkan `${user_config.KEY}` ke dalam bidang `headers` server, yang tidak diurai shell, atau baca nilai di dalam skrip helper

<h2 id="tool-errors">
  Kesalahan Tool
</h2>

Kesalahan ini berasal dari tool bawaan Claude yang menolak input. Claude memperbaiki sebagian besar kesalahan tool secara otomatis; dua kesalahan di bawah ini memerlukan perubahan dari Anda, karena berasal dari definisi subagent atau aturan izin yang Anda kontrol.

<h3 id="agent-would-be-spawned-with-zero-tools">
  Agent would be spawned with zero tools
</h3>

Tidak ada yang ada dalam [daftar `tools` subagent](/docs/id/sub-agents#supported-frontmatter-fields) yang terselesaikan menjadi tool, jadi Claude Code menolak untuk meluncurkan subagent daripada memulai yang tidak dapat bertindak. Pesan mengelompokkan entri berdasarkan alasan mereka tidak terselesaikan: bukan tool yang dikenali, tool yang tidak tersedia untuk subagent, atau dikenali tetapi tidak cocok dengan tool apa pun dalam sesi saat ini. Menghilangkan bidang `tools` tidak pernah memicu penolakan ini. Pola server MCP seperti `mcp__github__*` tidak dikecualikan: ketika tidak ada tool yang terhubung berasal dari server itu, peluncuran ditolak dengan pola dalam grup yang tidak cocok dengan apa pun. Sebelum v2.1.208, subagent diluncurkan tanpa tool dan mengembalikan hasil yang kosong atau membingungkan.

```text theme={null}
Agent 'code-reviewer' would be spawned with zero tools — refusing. Its tools list resolved to nothing: unrecognized [Grpe]. Fix the agent's tools frontmatter or pass a different subagent_type.
```

**Yang harus dilakukan:**

* Perbaiki setiap entri yang dinamai kesalahan terhadap [tool yang tersedia untuk subagent](/docs/id/sub-agents#available-tools)
* Hapus entri untuk tool yang tidak dimiliki sesi, seperti tool MCP dari server yang tidak terhubung
* Untuk memberikan subagent setiap tool yang dimiliki induk, hapus bidang `tools` daripada membuat daftar tool

<h3 id="file-is-covered-by-a-read-deny-rule">
  File is covered by a Read deny rule
</h3>

Tool Edit dipanggil pada jalur yang cocok dengan [aturan deny `Read`](/docs/id/permissions#read-and-edit), termasuk membuat file baru di jalur itu. Pengeditan menulis ulang konten yang harus dapat dibaca Claude kembali, jadi panggilan ditolak sebelum akses file apa pun. Aturan memblokir tool Edit saja: Write dan NotebookEdit tidak tercakup oleh aturan deny `Read`. Sebelum v2.1.208, hanya aturan deny `Edit` yang memblokir pengeditan, dan aturan deny `Read` saja tidak.

```text theme={null}
File is covered by a Read deny rule in your permission settings and cannot be edited.
```

**Yang harus dilakukan:**

* Jika Claude harus dapat mengedit file, hapus atau persempit aturan deny `Read` di `/permissions` atau di [settings](/docs/id/settings#permission-settings)
* Jika file harus tetap tidak tersentuh, pertahankan aturan dan tambahkan aturan deny `Edit` untuk jalur yang sama sehingga tool Write dan NotebookEdit juga diblokir

<h2 id="background-session-errors">
  Kesalahan sesi latar belakang
</h2>

[Sesi latar belakang](/docs/id/agent-view) berjalan tanpa terminal interaktif mereka sendiri, jadi perintah yang membutuhkan satu berperilaku berbeda di sana. Pesan-pesan ini muncul dalam transkrip sesi latar belakang, dalam tampilan agen atau setelah melampirkan.

<h3 id="commands-refused-in-a-background-session">
  Perintah yang ditolak dalam sesi latar belakang
</h3>

Perintah yang membuka dialog interaktif ditolak dalam sesi latar belakang dengan pesan yang menyebutkan formulir yang berfungsi di sana atau memberi tahu Anda untuk menjalankan perintah dari terminal biasa. `/install-github-app`, daftar pengaturan `/mcp`, dan tindakan autentikasi dalam menu server MCP semuanya ditolak dengan cara ini. Sebelum v2.1.208, mereka membuka dialog mereka di dalam sesi latar belakang.
Dalam v2.1.208 saja, pemilih `/model` juga ditolak dalam sesi latar belakang, dan `/upgrade` mencetak URL upgrade alih-alih membuka browser.

Redaksi menyebutkan perintah yang ditolak. Daftar pengaturan `/mcp` melaporkan:

```text theme={null}
Can't open MCP settings in a background session — use `/mcp enable|disable|reconnect <server>` to steer, or run /mcp from an interactive terminal to authenticate.
```

**Yang harus dilakukan:**

* Gunakan formulir yang disebutkan pesan, seperti `/mcp reconnect <server>`, `/mcp enable`, atau `/mcp disable`
* Untuk alur masuk dan otorisasi, jalankan perintah dari sesi `claude` biasa dalam terminal

<h3 id="claude_code_process_wrapper-launcher-errors">
  Kesalahan peluncur CLAUDE\_CODE\_PROCESS\_WRAPPER
</h3>

[`CLAUDE_CODE_PROCESS_WRAPPER`](/docs/id/corporate-launcher) diatur, dan nilainya tidak dapat digunakan, jadi Claude Code menolak untuk memulai proses yang terpengaruh daripada menjalankannya tanpa peluncur. Masalah konfigurasi dilaporkan dengan pesan yang dimulai dengan nama variabel dan menyatakan alasannya, misalnya:

```text theme={null}
CLAUDE_CODE_PROCESS_WRAPPER: launcher `/opt/corp/launcher` is not an executable regular file
```

Peluncur yang dimulai tetapi keluar tanpa mengganti dirinya dengan Claude Code gagal dalam sesi yang sedang dimulai, dan baris sesi dalam tampilan agen melaporkan bahwa peluncur `must exec, not daemonize`, diikuti oleh apa pun yang dicetak peluncur. Sesi yang tidak dapat dimulai atau mencapai layanan latar belakang karena peluncur melaporkan masalah peluncur sebagai alasan di dalam `Couldn't reach the background service (...)`.

**Yang harus dilakukan:**

* Atur variabel ke jalur absolut dari executable yang diakhiri dengan memanggil `exec "$@"`. Lihat [kontrak peluncur](/docs/id/corporate-launcher#the-launcher-contract) untuk kontrak lengkap
* Periksa `/status`, yang menunjukkan perintah peluncuran yang diselesaikan dalam entri Self-exec-nya dan memperingatkan ketika layanan latar belakang yang berjalan tidak cocok, atau jalankan `claude daemon status` dari shell
* Setelah memperbaiki nilai dalam blok `env` dari [pengaturan](/docs/id/corporate-launcher#set-up-the-launcher), mulai ulang layanan latar belakang dengan `claude daemon stop --any` sehingga pengiriman berikutnya memulai yang dibungkus

<h2 id="configuration-warnings">
  Peringatan Konfigurasi
</h2>

Claude Code menulis pesan-pesan ini ke stderr saat startup daripada menampilkan kesalahan dalam percakapan. Mereka melaporkan konfigurasi yang dibaca tetapi tidak diterapkan.

<h3 id="workspace-has-not-been-trusted">
  Workspace belum dipercaya
</h3>

Claude Code menemukan aturan `permissions.allow` atau entri `permissions.additionalDirectories` dalam `.claude/settings.json` atau `.claude/settings.local.json` proyek dan tidak menerapkannya, karena [aturan allow dari pengaturan proyek memerlukan kepercayaan workspace](/docs/id/permissions#project-allow-rules-and-workspace-trust). Jumlah, nama pengaturan, dan file yang dinamai dalam pesan bervariasi dengan konfigurasi Anda. Aturan `deny` dan `ask` tidak terpengaruh.

```text theme={null}
Ignoring 2 permissions.allow entries from .claude/settings.local.json: this workspace has not been trusted. Run Claude Code interactively here once and accept the trust dialog, or set projects["/Users/you/project"].hasTrustDialogAccepted: true in /Users/you/.claude.json.
```

**Yang harus dilakukan:**

* Jalankan `claude` di direktori dan terima dialog kepercayaan. Dialog muncul bahkan ketika direktori induk sudah dipercaya, mencantumkan aturan yang ditahan, dan memungkinkan Anda menolak dan terus bekerja tanpanya. Sebelum v2.1.200, tidak ada dialog yang muncul dalam situasi itu, jadi langkah ini tidak dapat diselesaikan di sana.
* Dalam [mode non-interaktif](/docs/id/headless) dengan `-p` tidak ada dialog yang ditampilkan. Atur entri `hasTrustDialogAccepted` dalam `~/.claude.json` menggunakan kunci `projects` yang tepat yang dicetak pesan.
* Jika pesan menyebutkan `.claude/settings.local.json` dan Anda memulai Claude Code di luar repositori git atau di direktori home Anda, perbarui ke v2.1.200 atau lebih baru. Versi 2.1.196 hingga 2.1.199 memperlakukan `.claude/settings.local.json` Anda sendiri sebagai yang disediakan repositori di workspace tersebut. Pada v2.1.207 dan lebih baru, pembaruan tidak cukup di luar repositori git jika Anda belum mempercayai folder: menentukan bahwa folder tidak berada di dalam repositori menjalankan git, dan Claude Code menjalankan pemeriksaan itu hanya setelah Anda menerima dialog kepercayaan, jadi gunakan langkah pertama. Direktori home Anda dan [configuration home](/docs/id/permissions#project-allow-rules-and-workspace-trust) lainnya dikecualikan dan tidak menunggu dialog. Lihat [Project allow rules and workspace trust](/docs/id/permissions#project-allow-rules-and-workspace-trust).

<h2 id="responses-seem-lower-quality-than-usual">
  Respons tampak berkualitas lebih rendah dari biasanya
</h2>

Jika jawaban Claude tampak kurang mampu dari yang Anda harapkan tetapi tidak ada kesalahan yang ditampilkan, penyebabnya biasanya adalah status percakapan daripada model itu sendiri. Claude Code tidak secara diam-diam mengubah versi model. Ini dapat beralih ke model fallback dalam tiga kasus spesifik:

* [`--fallback-model`](/docs/id/cli-reference#cli-flags) yang dikonfigurasi mengambil alih setelah kesalahan ketersediaan, hanya untuk giliran itu, dengan pemberitahuan dalam transkrip
* Pemeriksaan startup Amazon Bedrock atau Google Cloud's Agent Platform menemukan model default Anda tidak tersedia
* [Fallback model otomatis](/docs/id/model-config#automatic-model-fallback) pada Fable 5 memindahkan sesi ke model Opus default dan menampilkan pemberitahuan dalam transkrip

Pemeriksaan Model selection di bawah menangkap kasus kedua dan ketiga; kasus pertama muncul sebagai pemberitahuan transkrip daripada perubahan `/model`. [Konfigurasi model](/docs/id/model-config) menjelaskan kapan setiap fallback berlaku.

Periksa ini terlebih dahulu:

* **Model selection**: jalankan `/model` untuk mengonfirmasi Anda berada di model yang Anda harapkan. Pilihan `/model` sebelumnya atau variabel lingkungan `ANTHROPIC_MODEL` mungkin membuat Anda berada di model yang lebih kecil dari yang Anda maksudkan.
* **Effort level**: jalankan `/effort` untuk memeriksa tingkat reasoning saat ini dan naikkan untuk debugging atau pekerjaan desain yang sulit. Default bervariasi menurut model, jadi periksa sebelum mengasumsikan Anda di bawah maksimum. Lihat [Adjust effort level](/docs/id/model-config#adjust-effort-level) untuk default per-model dan pintasan `ultrathink`.
* **Context pressure**: jalankan `/context` untuk melihat seberapa penuh jendela tersebut. Jika mendekati kapasitas, jalankan `/compact` pada titik alami atau `/clear` untuk memulai dari awal. Lihat [Explore the context window](/docs/id/context-window) untuk cara auto-compact mempengaruhi giliran sebelumnya.
* **Stale instructions**: file `CLAUDE.md` yang besar atau ketinggalan zaman dan definisi alat MCP mengonsumsi konteks dan dapat mengarahkan respons. Pemeriksaan `/doctor` menandai file memori yang berukuran besar dan ekstensi yang tidak digunakan, dan `/context` menampilkan penggunaan token alat MCP. Sebelum v2.1.205, `/doctor` membuka layar diagnostik yang menandai file memori yang berukuran besar dan definisi subagent.

Ketika respons salah, rewinding biasanya bekerja lebih baik daripada membalas dengan koreksi. Tekan Esc dua kali atau jalankan `/rewind` untuk mundur ke sebelum giliran yang buruk, kemudian rephrase prompt dengan spesifik lebih lanjut. Mengoreksi dalam-thread menjaga upaya yang salah dalam konteks, yang dapat menambatkan jawaban kemudian ke dalamnya. Lihat [Checkpointing](/docs/id/checkpointing).

Jika kualitas masih tampak tidak sesuai setelah memeriksa di atas, jalankan `/feedback` dan jelaskan apa yang Anda harapkan versus apa yang Anda dapatkan. Feedback yang dikirimkan dengan cara ini mencakup transkrip percakapan, yang merupakan cara tercepat bagi Anthropic untuk mendiagnosis regresi nyata. Lihat [Report an error](#report-an-error) jika `/feedback` tidak tersedia di lingkungan Anda.

Jika Claude memperingatkan tentang injeksi prompt yang dicurigai, atau menolak permintaan karena injeksi yang dicurigai, dan teks yang dinamai peringatan adalah konteks yang ditambahkan Claude Code ke percakapan secara otomatis daripada konten file atau web, jalankan `claude update` dan coba lagi. Jika peringatan berulang setelah memperbarui, [laporkan](/docs/id/#report-an-error) daripada menempel konten yang ditandai kembali ke dalam prompt. Sebelum v2.1.201, Sonnet 5 menolak beberapa permintaan dengan cara yang sama.

<h2 id="report-an-error">
  Laporkan kesalahan
</h2>

Untuk kesalahan dari komponen yang tidak tercakup di halaman ini, lihat panduan yang relevan:

* Server MCP gagal terhubung atau autentikasi: [MCP](/docs/id/mcp)
* Skrip hook gagal atau memblokir alat: [Debug hooks](/docs/id/hooks#debug-hooks)
* Izin ditolak atau kesalahan sistem file selama instalasi: [Troubleshoot installation and login](/docs/id/troubleshoot-install)

Jika kesalahan tidak tercantum di sini atau perbaikan yang disarankan tidak membantu:

* Jalankan `/feedback` di dalam Claude Code untuk mengirimkan transkrip dan deskripsi ke Anthropic. Perintah ini juga menawarkan untuk membuka masalah GitHub yang sudah diisi sebelumnya. Pengiriman ke Anthropic memerlukan [authentication](/docs/id/authentication). Di Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry, dan penyedia pihak ketiga lainnya, atau ketika tidak ada kredensial Anthropic yang dikonfigurasi, `/feedback` menyimpan arsip lokal yang dapat Anda kirimkan ke perwakilan akun Anthropic Anda.
* Jalankan `claude doctor` dari shell Anda untuk diagnostik sistem file read-only dari instalasi Anda, atau jalankan pemeriksaan `/doctor` di dalam Claude Code untuk menemukan dan memperbaiki masalah pengaturan
* Periksa [status.claude.com](https://status.claude.com) untuk insiden aktif
* Cari [existing issues](https://github.com/anthropics/claude-code/issues) di GitHub
