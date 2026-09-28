> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Panduan kompatibilitas gateway Claude Code

> Jaga gateway LLM tetap kompatibel dengan Claude Code: endpoint yang dipanggilnya, header dan field body yang harus diteruskan, dan apa yang rusak saat dihapus.

Halaman ini mendokumentasikan permintaan yang dikirim Claude Code ke gateway, termasuk endpoint yang dipanggilnya, header dan field body yang harus diteruskan gateway, dan fitur mana yang berhenti berfungsi saat tidak ada. Halaman ini ditulis untuk operator yang mengonfigurasi produk gateway agar bekerja dengan Claude Code.

[Gateway aplikasi Claude](/docs/id/claude-apps-gateway), gateway yang di-host sendiri oleh Anthropic, melayani referensi endpoint-nya sendiri di `GET /protocol`, mencakup endpoint sign-in, inference, managed settings, model discovery, dan telemetry gateway tersebut. Ini adalah dokumen terpisah dari panduan ini.

<Note>
  * Untuk meluncurkan gateway yang ada atau pihak ketiga untuk organisasi Anda, lihat [Meluncurkan gateway LLM](/docs/id/llm-gateway-rollout)
  * Jika Anda adalah pengembang individual yang mengautentikasi Claude Code ke gateway dengan kredensial yang diberikan kepada Anda, lihat [Hubungkan Claude Code ke gateway LLM](/docs/id/llm-gateway-connect)
</Note>

Halaman ini mencakup:

* [Format API](#api-formats) dan endpoint yang harus dilayani untuk masing-masing
* [Perilaku klien berdasarkan metode koneksi](#how-the-connection-method-changes-client-behavior): bagaimana ID model, nilai `anthropic-beta`, field permintaan, dan default berbeda antara format dan sign-in gateway aplikasi Claude
* [Header permintaan](#request-headers): mana yang harus mencapai upstream dan mana yang dapat dikonsumsi gateway Anda
* [Header respons](#response-headers): apa yang harus dikembalikan agar deteksi stall, retry, dan tampilan batas penggunaan berfungsi
* Blok [atribusi prompt sistem](#system-prompt-attribution-block) dan bagaimana interaksinya dengan prompt caching
* [Penerusan fitur](#feature-pass-through): apa yang rusak saat header atau field body dihapus
* [Penemuan model](#model-discovery)

Halaman ini menggunakan dua istilah untuk apa yang dilakukan gateway Anda dengan setiap header dan field body:

* **Teruskan tanpa perubahan**: teruskan ke upstream byte-for-byte
* **Konsumsi**: gateway dapat membacanya untuk routing, atribusi, atau tracing dan tidak perlu meneruskannya

Apa pun yang tidak ditandai teruskan tanpa perubahan adalah milik Anda untuk dikonsumsi atau diabaikan.

<h2 id="api-formats">
  Format API
</h2>

Gateway harus mengekspos setidaknya salah satu format API berikut kepada klien Claude Code. Klien memilih format dan menunjukkan Claude Code ke gateway Anda dengan variabel di kolom Selected by tabel di bawah.

Google Cloud's Agent Platform adalah endpoint Claude Google Cloud, sebelumnya Vertex AI; nama variabelnya tetap menggunakan ejaan `VERTEX`.

| Format                                   | Selected by                                                     | Endpoints                                                                                                       | Forward unchanged                                                                                         |
| :--------------------------------------- | :-------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------- |
| Anthropic Messages                       | `ANTHROPIC_BASE_URL`                                            | `/v1/messages`, `/v1/messages/count_tokens` (opsional)                                                          | header permintaan `anthropic-beta` dan `anthropic-version`                                                |
| Amazon Bedrock InvokeModel               | `ANTHROPIC_BEDROCK_BASE_URL` dengan `CLAUDE_CODE_USE_BEDROCK=1` | `/model/{model}/invoke`, `/model/{model}/invoke-with-response-stream`, `/model/{model}/count-tokens` (opsional) | field body permintaan `anthropic_beta` dan `anthropic_version`                                            |
| Google Cloud's Agent Platform rawPredict | `ANTHROPIC_VERTEX_BASE_URL` dengan `CLAUDE_CODE_USE_VERTEX=1`   | `:rawPredict`, `:streamRawPredict`, `count-tokens:rawPredict` (opsional)                                        | header permintaan `anthropic-beta` dan `anthropic-version`, dan field body permintaan `anthropic_version` |

<h3 id="foundry-and-claude-platform-on-aws">
  Foundry dan Claude Platform on AWS
</h3>

Microsoft Foundry dan [Claude Platform on AWS](/docs/id/claude-platform-on-aws) mengimplementasikan format Anthropic Messages. Claude Code merutekan ke mereka melalui variabel mereka sendiri, `ANTHROPIC_FOUNDRY_BASE_URL` dan `ANTHROPIC_AWS_BASE_URL`, tetapi gateway yang berada di depan salah satu dari mereka mengimplementasikan baris Anthropic Messages di atas. Gateway yang berada di depan Claude Platform on AWS juga harus meneruskan header `anthropic-workspace-id`, yang [platform tersebut memerlukan pada setiap permintaan](/docs/id/claude-platform-on-aws).

<h3 id="optional-endpoints-and-startup-traffic">
  Endpoint opsional dan lalu lintas startup
</h3>

Endpoint penghitungan token adalah satu-satunya yang opsional: ketika tidak ada, Claude Code kembali ke perkiraan berbasis karakter dari penggunaan konteks.

Cocokkan pada path, bukan URL lengkap:

* Permintaan inferensi posting ke `/v1/messages?beta=true`
* Metode Google Cloud's Agent Platform menambahkan sufiks ke path model penerbit, seperti dalam `/projects/{project}/locations/{location}/publishers/anthropic/models/{model}:streamRawPredict`

Gateway juga melihat lalu lintas startup best-effort yang dapat ditolak tanpa merusak apa pun. Gateway format Anthropic Messages menerima probe pemanasan koneksi `HEAD /api/hello`, yang Claude Code lewati ketika proxy HTTP atau sertifikat klien dikonfigurasi. Gateway format Amazon Bedrock menerima permintaan `GET /inference-profiles?type=SYSTEM_DEFINED` dan, ketika model yang dikonfigurasi adalah profil inferensi, pencarian `GET /inference-profiles/{profile}`.

Pemeriksaan ketersediaan [fast mode](/docs/id/fast-mode) tidak pernah muncul dalam log gateway: ia memanggil `api.anthropic.com` secara langsung daripada mengikuti `ANTHROPIC_BASE_URL`, jadi pada jaringan yang memblokir egress langsung ke `api.anthropic.com`, fast mode dapat melaporkan kesalahan konektivitas sementara inferensi melalui gateway terus bekerja. [Pemeriksaan keamanan domain WebFetch](/docs/id/data-usage#webfetch-domain-safety-check) juga memanggil `api.anthropic.com` secara langsung. [Gunakan fast mode di belakang proxy dan gateway LLM](/docs/id/fast-mode#use-fast-mode-behind-proxies-and-llm-gateways) mencakup variabel yang memulihkannya.

<h3 id="streaming">
  Streaming
</h3>

Alirkan respons inferensi. Claude Code membaca aliran saat tiba, jadi jika gateway Anda membuffer respons lengkap sebelum meneruskannya, Claude Code macet.

Ketika klien berbicara format Amazon Bedrock, teruskan body respons `InvokeModelWithResponseStream` dan header `Content-Type: application/vnd.amazon.eventstream` tanpa modifikasi, dan jangan konversi aliran ke server-sent events. Lihat [Streaming errors behind a gateway or proxy](/docs/id/amazon-bedrock#streaming-errors-behind-a-gateway-or-proxy).

Teruskan ping keep-alive juga. Pada koneksi melalui `ANTHROPIC_BASE_URL` atau `ANTHROPIC_AWS_BASE_URL`, Claude Code menghitung setiap byte yang gateway Anda teruskan, termasuk event SSE `ping` dan baris komentar, dan membatalkan aliran yang diam selama 300 detik secara default. Ping upstream adalah satu-satunya lalu lintas selama jeda pemikiran panjang, jadi jika gateway Anda menghapus atau membuffer mereka, Claude Code membatalkan aliran selama jeda tersebut; [Automatic retries](/docs/id/errors#automatic-retries) mencakup apa yang dilaporkan aliran yang dibatalkan berdasarkan seberapa jauh respons telah maju. Upstream yang tidak mengirim ping sama sekali, seperti event-stream biner Amazon Bedrock, meninggalkan jeda tersebut tanpa apa pun untuk diteruskan. Ketika menerjemahkan dari upstream seperti itu, keluarkan event `ping` Anda sendiri selama celah senyap. Gateway yang dicapai melalui `ANTHROPIC_BEDROCK_BASE_URL`, `ANTHROPIC_VERTEX_BASE_URL`, atau `ANTHROPIC_FOUNDRY_BASE_URL` tidak dibungkus oleh watchdog tingkat byte ini, bahkan ketika mereka meneruskan format Anthropic Messages; di sana, [timeout idle 5 menit](/docs/id/env-vars) membatalkan aliran senyap sebagai gantinya, dan pada koneksi `ANTHROPIC_BEDROCK_BASE_URL` Anda dapat menambahkan watchdog byte dengan [`CLAUDE_ENABLE_BYTE_WATCHDOG_BEDROCK`](/docs/id/env-vars).

<h3 id="format-mismatch-with-the-upstream">
  Format mismatch dengan upstream
</h3>

Format mana yang digunakan klien menentukan apa yang diterima gateway Anda. Mode kegagalan umum adalah ketidaksesuaian antara format yang dikirim klien ke gateway Anda dan format yang diterima penyedia upstream di belakangnya.

* Ketika klien berbicara format Amazon Bedrock atau Google Cloud's Agent Platform, Claude Code mengirim hanya subset dari set kemampuan penuhnya yang diterima penyedia tersebut
* Ketika klien berbicara format Anthropic Messages, Claude Code mengirim set lengkap, bahkan jika gateway Anda meneruskan ke upstream Amazon Bedrock atau Google Cloud's Agent Platform

Menjembatani perbedaan itu adalah pekerjaan gateway Anda. [Feature pass-through](#feature-pass-through) menjelaskan apa yang rusak ketika tidak.

Jika upstream Anda adalah Amazon Bedrock atau Google Cloud's Agent Platform, Anda dapat menghindari penjembatanan dengan mengekspos format penyedia tersebut sebagai gantinya. [Route to a cloud provider through a gateway](/docs/id/llm-gateway-connect#route-to-a-cloud-provider-through-a-gateway) menunjukkan konfigurasi klien untuk format tersebut.

<h2 id="how-the-connection-method-changes-client-behavior">
  Bagaimana metode koneksi mengubah perilaku klien
</h2>

Cara pengembang terhubung ke gateway Anda menentukan ID model mana, nilai `anthropic-beta` mana, dan bidang permintaan mana yang dikirim Claude Code, serta default mana yang diterapkannya. Gateway Anda melihat salah satu dari tiga perilaku klien:

* **Format Amazon Bedrock atau Agent Platform**: pengembang menetapkan `CLAUDE_CODE_USE_BEDROCK=1` dengan `ANTHROPIC_BEDROCK_BASE_URL`, atau `CLAUDE_CODE_USE_VERTEX=1` dengan `ANTHROPIC_VERTEX_BASE_URL`, menunjuk ke gateway Anda. Claude Code menggunakan ID model, bidang permintaan, dan default penyedia tersebut.
* **Format Anthropic Messages**: pengembang menetapkan `ANTHROPIC_BASE_URL` ke gateway Anda. Claude Code memperlakukan gateway sebagai Claude API dan tidak dapat mengetahui upstream mana yang Anda teruskan.
* **Masuk ke gateway aplikasi Claude**: pengembang masuk ke [gateway aplikasi Claude](/docs/id/claude-apps-gateway). Gateway tersebut berbicara dalam format Anthropic Messages tetapi dapat merutekan ke upstream apa pun, jadi Claude Code hanya mengirimkan nilai `anthropic-beta` dan asumsi kemampuan model yang juga diterima Amazon Bedrock dan Agent Platform.

<h3 id="requests-and-defaults-by-connection-method">
  Permintaan dan default menurut metode koneksi
</h3>

Tabel di bawah membandingkan tiga metode koneksi, satu perilaku per baris. Tabel ini menghilangkan Microsoft Foundry dan Claude Platform di AWS, yang juga menggunakan format Anthropic Messages tetapi yang Claude Code jangkau melalui variabel mereka sendiri. Untuk itu, lihat halaman [Microsoft Foundry](/docs/id/microsoft-foundry) dan [Claude Platform di AWS](/docs/id/claude-platform-on-aws).

| Perilaku                                                                                                                 | Format Amazon Bedrock atau Agent Platform                                                                                                                                                                              | Format Anthropic Messages                                                                                                                                                                       | Masuk ke gateway aplikasi Claude                                                                                         |
| :----------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------- |
| ID model dalam permintaan secara default                                                                                 | Bentuk penyedia, seperti `us.anthropic.claude-opus-4-8` di Amazon Bedrock                                                                                                                                              | ID Anthropic, seperti `claude-opus-4-8`                                                                                                                                                         | ID Anthropic                                                                                                             |
| Nilai `anthropic-beta` yang dikirim                                                                                      | Subset yang diterima Amazon Bedrock dan Agent Platform                                                                                                                                                                 | Set lengkap yang dijelaskan di bawah [feature pass-through](#feature-pass-through), kecuali pengembang menetapkan [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS`](#disable-pre-release-capabilities) | Subset yang diterima Amazon Bedrock dan Agent Platform                                                                   |
| Bidang permintaan untuk ID model yang tidak dikenali Claude Code, seperti alias gateway                                  | Thinking dengan anggaran tetap daripada adaptive reasoning, dan tidak ada bidang effort atau context management                                                                                                        | Semua yang diterima model Claude saat ini di Claude API, termasuk adaptive reasoning, effort, dan context management, yang upstream Amazon Bedrock atau Agent Platform dapat menolak            | Sama dengan format Amazon Bedrock atau Agent Platform                                                                    |
| [TTL prompt cache](/docs/id/prompt-caching#choose-the-ttl-yourself) satu jam ketika pengembang memilih                        | Diminta melalui bidang `ttl` dalam `cache_control`, tanpa nilai beta                                                                                                                                                   | Diminta melalui bidang `ttl` plus nilai `extended-cache-ttl` dalam `anthropic-beta`, yang harus Anda teruskan                                                                                   | Lihat tabel [availability and limitations](/docs/id/claude-apps-gateway#availability-and-limitations) gateway aplikasi Claude |
| Model untuk [background tasks](/docs/id/costs#background-token-usage) kecuali `ANTHROPIC_DEFAULT_HAIKU_MODEL` menetapkan satu | Model Sonnet default, atau model utama setelah satu dipilih, seperti yang dijelaskan halaman [Amazon Bedrock](/docs/id/amazon-bedrock#4-pin-model-versions) dan [Agent Platform](/docs/id/google-vertex-ai#5-pin-model-versions) | Model utama, atau model Haiku default ketika `ANTHROPIC_API_KEY` atau `apiKeyHelper` menyediakan kunci Anthropic Console dan `ANTHROPIC_AUTH_TOKEN` tidak diatur                                | Model utama                                                                                                              |

Untuk fitur yang didukung setiap koneksi dan telemetri yang dikirimnya ke Anthropic secara default, lihat [Feature availability](/docs/id/feature-availability#availability-by-model-provider) dan [Default behaviors by API provider](/docs/id/data-usage#default-behaviors-by-api-provider).

<h3 id="settings-for-unrecognized-model-ids">
  Pengaturan untuk ID model yang tidak dikenali
</h3>

Dua pengaturan sisi klien mengubah apa yang diasumsikan Claude Code untuk ID model yang tidak dikenalinya, terlepas dari metode koneksi mana yang digunakan pengembang:

* **Context window**: Claude Code mengasumsikan 200K, atau 1M ketika ID membawa `[1m]`. Untuk mendeklarasikan jendela sebenarnya, lihat [Correct the window for a gateway or custom model ID](/docs/id/model-config#correct-the-window-for-a-gateway-or-custom-model-id)
* **Capabilities**: untuk memberikan alias gateway kemampuan model di belakangnya, petakan ID Anthropic model tersebut ke alias Anda dengan entri [`modelOverrides`](/docs/id/errors#unrecognized-model-id-on-a-request) dalam pengaturan yang Anda distribusikan. Untuk tempat variabel `ANTHROPIC_DEFAULT_*_MODEL_SUPPORTED_CAPABILITIES` berlaku, lihat [feature pass-through](#feature-pass-through)

<h2 id="request-headers">
  Header permintaan
</h2>

Claude Code menyertakan header ini pada permintaan API. Nama header tidak peka huruf besar-kecil di kawat. Teruskan `anthropic-version` dan `anthropic-beta` tanpa perubahan, ditambah `anthropic-workspace-id` ketika upstream adalah [Claude Platform on AWS](/docs/id/claude-platform-on-aws); sisanya gateway dapat konsumsi untuk routing, atribusi, dan tracing, dan tidak perlu diteruskan.

| Header                          | Deskripsi                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| :------------------------------ | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `Authorization`, `x-api-key`    | Kredensial gateway pengembang, di satu atau kedua header tergantung pada [variabel kredensial](/docs/id/llm-gateway-connect#set-the-credential-variable) yang mereka atur                                                                                                                                                                                                                                                                           |
| `anthropic-version`             | Versi API, saat ini `2023-06-01`. Permintaan format Amazon Bedrock dan Google Cloud's Agent Platform juga membawa field body `anthropic_version`, yang nilainya adalah string dialek penyedia, bukan nilai header ini                                                                                                                                                                                                                          |
| `anthropic-beta`                | Nilai kemampuan yang dipisahkan koma untuk permintaan. Teruskan header secara verbatim; jangan allowlist nilai individual, karena set berubah dengan rilis Claude Code. Ketika pengembang mengautentikasi dengan login claude.ai, yang mungkin ketika `ANTHROPIC_BASE_URL` diatur tanpa variabel kredensial gateway, header ini juga membawa kemampuan OAuth yang diperlukan upstream, dan menghapusnya gagal permintaan tersebut dengan `401` |
| `x-claude-code-session-id`      | Pengidentifikasi unik untuk sesi Claude Code saat ini. Gunakan untuk mengagregasi semua permintaan dari satu sesi tanpa mengurai body permintaan                                                                                                                                                                                                                                                                                               |
| `x-claude-code-agent-id`        | Pengidentifikasi [subagent](/docs/id/sub-agents) yang mengeluarkan permintaan, hadir hanya pada permintaan dari agen yang Claude Code spawn di dalam sesi. Gunakan dengan ID sesi untuk mengatribusikan biaya ke agen paralel                                                                                                                                                                                                                       |
| `x-claude-code-parent-agent-id` | Pengidentifikasi agen yang menspawn agen yang meminta, hadir hanya untuk agen bersarang                                                                                                                                                                                                                                                                                                                                                        |

ID subagent dihasilkan segar untuk setiap spawn. Agen rekan kerja, anggota bernama dari [tim agen](/docs/id/agent-teams), menggunakan kembali ID berbasis nama yang stabil di seluruh reconnections. Dalam kedua kasus ID mengidentifikasi agen, bukan orang atau perangkat, jadi jangan perlakukan header ID agen sebagai pengidentifikasi pengguna.

Jika pengembang Anda menetapkan `ANTHROPIC_CUSTOM_HEADERS`, header tersebut muncul pada permintaan juga.

<h3 id="gateway-hint-headers">
  Header petunjuk gateway
</h3>

Claude Code juga dapat mengirim petunjuk routing: fakta per-permintaan yang dapat digunakan gateway atau router untuk menjadwalkan, cache, atau mengatribusikan permintaan. Memerlukan Claude Code v2.1.273 atau lebih baru.

Apakah permintaan membawanya tergantung pada tempat Claude Code mengirimnya:

* Koneksi langsung ke API Anthropic: dikirim secara default
* URL dasar kustom: nonaktif secara default, karena proxy yang menolak header yang tidak dikenal akan gagal permintaan. Untuk menerimanya, atur [`CLAUDE_CODE_GATEWAY_HINT_HEADERS=1`](/docs/id/env-vars) untuk pengembang Anda, misalnya di blok `env` dari [pengaturan terkelola](/docs/id/managed-settings)
* Backend lainnya, termasuk Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry, dan Claude Platform on AWS: dikirim hanya ketika `CLAUDE_CODE_GATEWAY_HINT_HEADERS=1` diatur

Mengatur `CLAUDE_CODE_GATEWAY_HINT_HEADERS` ke `0` menghentikan header pada setiap koneksi.

Header hanya membawa apa yang baris di bawah daftar: kosakata tetap, nama alat, dan durasi, tidak pernah teks prompt atau konten file. Setiap nilai adalah ASCII yang dapat dicetak.

| Header                              | Deskripsi                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| :---------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `x-claude-code-request-class`       | Jenis permintaan apa ini: `main` untuk giliran percakapan utama, `subagent` untuk giliran [subagent](/docs/id/sub-agents), `workflow` untuk agen yang berjalan di dalam alur kerja, `compaction` untuk permintaan peringkasan yang memadatkan percakapan, atau `auxiliary` untuk permintaan samping seperti judul sesi, pengklasifikasi, dan ringkasan. Dikirim pada setiap permintaan                                                                                                                                                     |
| `x-claude-code-agent-type`          | Jenis subagent yang mengeluarkan permintaan: nama jenis agen bawaan seperti `Explore`, `Plan`, atau `general-purpose`, atau `custom` untuk agen yang ditentukan pengguna, `teammate` untuk anggota [tim agen](/docs/id/agent-teams) yang berjalan dalam proses pemimpin, atau `fork` untuk [fork](/docs/id/sub-agents#fork-the-current-conversation). Hadir hanya pada giliran subagent sendiri; permintaan compaction atau samping subagent menyimpan ID agen tetapi tidak membawa jenis. Nama agen yang dipilih pengguna tidak pernah dikirim |
| `x-claude-code-compaction`          | Hadir pada permintaan yang merangkum percakapan selama [compaction](/docs/id/prompt-caching#compacting-the-conversation). Nilainya mengatakan apa yang memicunya: `auto` ketika jendela konteks mendekati kapasitas, `manual` untuk `/compact`, atau `reactive` ketika API menolak permintaan sebagai terlalu panjang. Tidak ada pada setiap permintaan lainnya                                                                                                                                                                            |
| `x-claude-code-context-compacted`   | Hadir sekali, pada permintaan percakapan utama pertama setelah compaction, dengan nilai yang sama seperti `x-claude-code-compaction`. Awalan percakapan sebelum permintaan ini tidak lagi digunakan, jadi cache yang dikunci padanya dapat dijatuhkan                                                                                                                                                                                                                                                                                 |
| `x-claude-code-prev-tool-durations` | Waktu run terukur dari panggilan alat yang hasilnya dibawa permintaan ini, sebagai `<name>=<ms>;<name>=<ms>`, misalnya `Bash=742;Read=9`. Dikirim pada permintaan berikutnya dari percakapan yang sama setelah batch panggilan alat, dari sesi utama atau subagent                                                                                                                                                                                                                                                                    |

Sebelum mengurai `x-claude-code-prev-tool-durations`, periksa bagaimana Claude Code membangun nilai dan apa yang ditinggalkannya:

* Entri: satu per panggilan alat yang berjalan, dalam urutan hasilnya dikumpulkan, dalam milidetik utuh
* Cap: Claude Code mengirim paling banyak 32 entri dan 4 KB, menyimpan entri pertama
* Encoding: nama alat dikodekan persen, mencakup `%`, `;`, `=`, koma, spasi, dan karakter apa pun di luar ASCII yang dapat dicetak
* Parsing: pisahkan pada `;`, kemudian pada `=`, dan dekode setiap nama
* Ketiadaan: panggilan compaction, permintaan samping, dan permintaan pertama prompt baru tidak membawanya. Jangan baca header yang hilang sebagai giliran yang tidak menjalankan alat
* Waktu: masing-masing mengecualikan prompt izin dan hooks, dan panggilan alat paralel masing-masing melaporkan waktu mereka sendiri, jadi entri tidak menambah hingga celah antara permintaan

<h3 id="forward-as-open-lists">
  Teruskan sebagai daftar terbuka
</h3>

Perlakukan header dan field body sebagai daftar terbuka, bukan daftar tertutup. Claude Code mendapatkan kemampuan di seluruh rilis, dan mereka tiba sebagai nilai `anthropic-beta` baru, field body permintaan baru, dan kadang-kadang header `anthropic-*` atau `x-claude-code-*` baru.

Ketika meneruskan ke upstream format Anthropic, teruskan header permintaan `anthropic-*` dan field body permintaan melalui tanpa perubahan daripada allowlist yang Anda lihat hari ini. Gateway yang disematkan ke daftar yang diamati menghapus header atau field kemampuan berikutnya dan merusaknya pada rilis yang memperkenalkannya.

Pengecualiannya adalah upstream non-Anthropic seperti Amazon Bedrock atau Google Cloud's Agent Platform, di mana menjembatani perbedaan skema adalah pekerjaan gateway; lihat [penerusan fitur](#feature-pass-through).

<h2 id="response-headers">
  Response headers
</h2>

Claude Code membaca response headers ini untuk mendeteksi stalled streams, untuk memutuskan apakah dan kapan harus retry, dan untuk menampilkan usage limits. Tabel ini mencantumkan apa yang harus dikembalikan untuk masing-masing. Juga forward error response bodies tanpa modifikasi, sehingga [capability-rejection recovery](#automatic-retry-and-error-forwarding) Claude Code dapat mencocokkan wording error upstream.

| Header                          | Apa yang harus dikembalikan dan mengapa                                                                                                                                                                                                                                                                                                                                                       |
| :------------------------------ | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `content-type`                  | Return `text/event-stream` pada streamed Anthropic Messages-format responses, dan `application/vnd.amazon.eventstream`, tanpa modifikasi, pada Amazon Bedrock-format responses, di mana [tipe yang berbeda gagal request](/docs/id/amazon-bedrock#streaming-errors-behind-a-gateway-or-proxy). [Streaming](#streaming) mencantumkan koneksi mana yang menjalankan stall detection pada streams ini |
| `retry-after`                   | Return integer seconds daripada HTTP date. Claude Code menunggu setidaknya selama itu sebelum [automatic retry](/docs/id/errors#automatic-retries) berikutnya, dan di luar [`CLAUDE_CODE_RETRY_WATCHDOG`](/docs/id/env-vars) sessions nilai di atas 60 menghentikan retries dan menampilkan error sekaligus                                                                                             |
| `x-should-retry`                | Pass nilai upstream melalui unchanged. Claude Code membaca header ini sebagai satu input ketika memutuskan apakah harus retry failed request: `true` menandai response retryable dan `false` menandai tidak retryable. Untuk retry counts, backoff, dan kegagalan mana yang Claude Code retry, lihat [automatic retries](/docs/id/errors#automatic-retries)                                        |
| `anthropic-ratelimit-unified-*` | Forward nilai upstream unchanged pada setiap response. Claude Code membacanya pada successful responses untuk menampilkan usage terhadap plan limits kepada developers yang signed in dengan claude.ai, dan pada `429` untuk membedakan plan limit atau spend cap dari temporary throttle; lihat [usage limits](/docs/id/errors#usage-limits)                                                      |

<h2 id="system-prompt-attribution-block">
  Blok atribusi prompt sistem
</h2>

Claude Code menambahkan blok atribusi pendek ke prompt sistem yang berisi versi klien dan sidik jari yang berasal dari percakapan. Endpoint `api.anthropic.com` menghapus blok sebelum memproses ketika tiba tidak berubah sebagai blok sistem pertama, jadi tidak mempengaruhi prompt caching pihak pertama. Upstream lain apa pun menerimanya sebagai bagian dari prompt.

Strip bersifat posisional, jadi hanya berfungsi ketika gateway meneruskan array `system` tanpa perubahan. Untuk menjaga blok keluar dari prompt tanpa kehilangan konten sistem lainnya:

* Teruskan array `system` persis seperti yang diterima, menjaga blok tetap pertama: menambahkan blok sistem lain, mengurutkan ulang array, atau mengonversinya menjadi string tunggal mengalahkan strip, dan blok kemudian mencapai model dan kunci cache prompt.
* Jaga blok dalam entri array-nya sendiri: endpoint memperlakukan blok yang digabungkan yang dimulai dengan header atribusi sebagai atribusi sepenuhnya dan menghapus semua yang digabungkan ke dalamnya, termasuk sisa prompt sistem.
* Jika gateway Anda harus membentuk ulang konten sistem, atur [`CLAUDE_CODE_ATTRIBUTION_HEADER=0`](/docs/id/env-vars) sehingga Claude Code menghilangkan blok. Anthropic dan endpoint Claude penyedia cloud membacanya untuk atribusi, jadi hilangkan di klien daripada menghapusnya atau memindahkannya di gateway.

Variabel ada untuk kompatibilitas gateway dan caching pihak ketiga, bukan sebagai kontrol privasi: pada koneksi langsung permintaan lengkap sudah pergi ke Anthropic API bagaimanapun. Ketika keduanya berlaku, Claude Code menjaga blok pada permintaan pengklasifikasi [mode otomatis](/docs/id/permission-modes#eliminate-prompts-with-auto-mode) bahkan ketika Anda menetapkan variabel ke `0`:

* Permintaan pergi ke `api.anthropic.com`, dengan `ANTHROPIC_BASE_URL` tidak diatur atau menamai host itu dan tidak ada penyedia pihak ketiga yang dipilih.
* Kredensial aktif bukan [profil Anthropic atau kredensial federasi](/docs/id/authentication#anthropic-profiles-and-federation-credentials).

Permintaan pengklasifikasi melewati sisa prompt sistem Claude Code, jadi pada permintaan tersebut blok adalah satu-satunya penanda dalam body permintaan yang mengidentifikasinya sebagai lalu lintas Claude Code. Ketika salah satu kondisi gagal, melalui gateway LLM, pada penyedia pihak ketiga, atau dengan profil atau kredensial federasi aktif, menetapkan `0` menghapus blok dari permintaan pengklasifikasi juga. Sebelum v2.1.229, pengecualian ini tidak ada: menetapkan `0` menghapus blok dari permintaan pengklasifikasi tersebut, dan ketika API menolak permintaan yang tidak teridentifikasi, mode otomatis gagal pada setiap tindakan yang dikirimnya ke pengklasifikasi.

Dari Claude Code v2.1.181, blok stabil untuk seumur hidup percakapan ketika permintaan merutekan melalui URL dasar kustom, jadi cache prompt gateway-side yang dikunci pada body permintaan lengkap bekerja tanpa menonaktifkannya, dan penyedia apa pun yang gateway Anda teruskan menerima awalan prompt yang stabil. Sebelum v2.1.181 blok menyertakan token per-permintaan yang mengubah awal prompt sistem pada setiap permintaan. Pada versi tersebut, atur `CLAUDE_CODE_ATTRIBUTION_HEADER=0` ketika gateway Anda melakukan salah satu dari ini:

* Mengimplementasikan cache prompt yang dikunci pada body permintaan.
* Meneruskan permintaan ke penyedia pihak ketiga seperti Amazon Bedrock, Microsoft Foundry, atau Agent Platform Google Cloud, dalam format Anthropic Messages atau format penyedia sendiri, di mana awalan yang berubah mengurangi reuse cache prompt pada penyedia itu.

<h2 id="feature-pass-through">
  Penerusan fitur
</h2>

Claude Code memperlakukan gateway `ANTHROPIC_BASE_URL` sebagai endpoint format Anthropic dan mengirimkannya header beta dan field body permintaan yang dikirimkannya ke `api.anthropic.com`, kecuali set kecil diagnostik dan default yang disediakan untuk koneksi langsung, seperti default streaming alat berbutir halus yang tercakup di bawah. Set tersebut bervariasi menurut rilis, jadi jangan bergantung pada isinya.

Kemampuan yang menambahkan field body memasangkannya dengan header beta, dan pasangan bepergian bersama. Gateway yang menghapus header sambil melewatkan body, atau meneruskan body format Anthropic ke upstream dengan skema berbeda, menghasilkan kesalahan `400` keras; hanya ketika kedua bagian tidak ada bersama-sama fitur mati diam-diam. Gateway yang menulis ulang atau menyunting body permintaan untuk inspeksi konten memecah pasangan dengan cara yang sama seperti penghapusan, jadi inspeksi tanpa memodifikasi. Tabel mencatat di mana fitur menyimpang dari pasangan.

Streaming alat berbutir halus adalah salah satu default koneksi langsung: itu dimatikan secara default setiap kali permintaan merutekan melalui URL dasar kustom, dan gateway menerimanya ketika pengembang menetapkan [`CLAUDE_CODE_ENABLE_FINE_GRAINED_TOOL_STREAMING=1`](/docs/id/env-vars).

| Fitur                                                                                                                                                                                                                                              | Header dan pasangan body                                                                                                                                                                                                             | Gejala ketika rusak                                                                                                                                                                    | Remediasi                                                                                                                               |
| :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------- |
| [Penalaran adaptif](/docs/id/model-config#adjust-effort-level)                                                                                                                                                                                          | Tidak ada header beta. Claude Code mengirim `thinking: {"type": "adaptive"}` untuk Claude 4.6 dan lebih baru, dan memperlakukan nama model yang tidak dikenalinya, seperti alias gateway, sebagai model saat ini yang menerima field | `400` penamaan field `thinking` atau tag `adaptive` ketika build model upstream tidak menerimanya                                                                                      | Tingkatkan upstream. Pada Opus 4.6 dan Sonnet 4.6, pengembang dapat mengatur `CLAUDE_CODE_DISABLE_ADAPTIVE_THINKING=1` sebagai gantinya |
| [Manajemen konteks](https://platform.claude.com/docs/en/build-with-claude/context-editing)                                                                                                                                                         | Header beta manajemen konteks berpasangan dengan field body `context_management`                                                                                                                                                     | `400` dengan `Extra inputs are not permitted`. Umum ketika gateway menerima permintaan format Anthropic tetapi meneruskannya ke Amazon Bedrock                                         | Teruskan keduanya, atau [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1`](/docs/id/env-vars)                                                      |
| [Konteks diperluas](https://platform.claude.com/docs/en/build-with-claude/context-windows#context-window-sizes-by-model) dan [pemikiran interleaved](https://platform.claude.com/docs/en/build-with-claude/extended-thinking#interleaved-thinking) | Hanya header beta, tidak ada field body                                                                                                                                                                                              | Diam-diam tidak tersedia ketika header dihapus; upstream tidak pernah melihat permintaan kemampuan                                                                                     | Teruskan `anthropic-beta` secara verbatim                                                                                               |
| Beta [field alat](https://platform.claude.com/docs/en/agents-and-tools/tool-use/overview)                                                                                                                                                          | Header beta terkait alat berpasangan dengan field skema alat seperti `strict` dan `defer_loading`                                                                                                                                    | `400` penamaan field skema alat yang tidak dikenali ketika body melewati tanpa headernya                                                                                               | Teruskan keduanya, atau [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1`](#disable-pre-release-capabilities)                                 |
| [Upaya](https://platform.claude.com/docs/en/build-with-claude/effort) dan [output terstruktur](https://platform.claude.com/docs/en/build-with-claude/structured-outputs)                                                                           | Field body `output_config` membawa upaya, format output terstruktur, dan pengaturan anggaran tugas; masing-masing berpasangan dengan header betanya sendiri                                                                          | `400` penamaan `output_config`, sering `Extra inputs are not permitted`, pada upstream Amazon Bedrock dan Agent Platform Google Cloud                                                  | Teruskan field dan headernya bersama-sama                                                                                               |
| [Prompt caching](/docs/id/prompt-caching)                                                                                                                                                                                                               | Tidak ada pasangan beta. Claude Code melampirkan penanda `cache_control` ke blok `system` dan ke entri `messages`, termasuk entri `role: "system"` yang ditambahkan mid-conversation                                                 | Tidak ada kesalahan: percakapan ditagih sebagai input uncached pada setiap giliran, terlihat sebagai `input_tokens` tinggi dengan sedikit atau tidak ada aktivitas cache dalam `usage` | Teruskan `cache_control` tidak berubah di mana pun muncul, dan jangan konversi blok-form `system` atau konten pesan ke string biasa     |
| [Penghitungan token](https://platform.claude.com/docs/en/build-with-claude/token-counting)                                                                                                                                                         | Tidak ada pasangan beta; menggunakan endpoint `count_tokens`                                                                                                                                                                         | Tidak ada kesalahan: Claude Code kembali ke perkiraan berbasis karakter, jadi `/context` menampilkan penghitungan perkiraan                                                            | Ekspos endpoint untuk penghitungan token yang tepat                                                                                     |

Variabel `ANTHROPIC_DEFAULT_*_MODEL_SUPPORTED_CAPABILITIES` [](/docs/id/model-config) mendeklarasikan kemampuan model hanya dalam konfigurasi penyedia: `CLAUDE_CODE_USE_BEDROCK`, `CLAUDE_CODE_USE_VERTEX`, `CLAUDE_CODE_USE_FOUNDRY`, dan [`CLAUDE_CODE_USE_MANTLE`](/docs/id/amazon-bedrock#use-the-mantle-endpoint). Mereka tidak memiliki efek di belakang gateway `ANTHROPIC_BASE_URL`.

<h3 id="automatic-retry-and-error-forwarding">
  Retry otomatis dan penerusan kesalahan
</h3>

Apa yang Claude Code lakukan setelah penolakan upstream tergantung pada apa yang ditolak:

* Ketika upstream menolak field `thinking`, pesan sistem mid-conversation, atau penanda `cache_control` pada pesan tersebut, Claude Code mengulangi permintaan dan menonaktifkan kemampuan yang ditolak untuk sisa percakapan
* Ketika upstream menolak [tanda tangan pemikiran](https://platform.claude.com/docs/en/build-with-claude/extended-thinking), termasuk dengan `400` yang pesannya mengatakan blok `bound to a different conversation`, Claude Code menghapus blok pemikiran sebelumnya dari permintaan, mengulangi, dan menjaganya keluar dari setiap permintaan kemudian. Respons baru masih menyertakan pemikiran
* Ketika gateway atau upstream-nya menolak entri [alat advisor](/docs/id/advisor) dalam `tools` sebagai tipe alat yang tidak dikenali, Claude Code mengulangi permintaan sekali tanpa entri tersebut dan nilai `anthropic-beta`-nya. Permintaan kemudian ke URL dasar tersebut meninggalkan advisor keluar sampai Claude Code keluar, dan `/advisor` tidak tersedia untuk pengembang untuk waktu itu. Claude Code mengenali penolakan ini dengan respons `400` atau `422` yang pesannya menyebutkan tipe alat setelah `Input tag`, seperti `Input tag 'advisor_20260301'`. Sebelum v2.1.280, Claude Code tidak mengulangi penolakan ini
* Claude Code tidak mengulangi penolakan manajemen konteks atau field skema alat, jadi kesalahan `400` tersebut mencapai pengembang

Penolakan `bound to a different conversation` berasal dari pemeriksaan [preserved thinking](https://platform.claude.com/docs/en/build-with-claude/preserved-thinking) API, yang gagal ketika konten `system`, `tools`, atau `messages` sebelumnya berbeda dari permintaan yang menghasilkan pemikiran. Gateway yang menulis ulang konten apa pun dari itu dapat menyebabkan penolakan itu sendiri; [Libraries, proxies, and gateways](https://platform.claude.com/docs/en/build-with-claude/preserved-thinking#libraries-proxies-gateways) mencakup apa yang harus dilewatkan tanpa perubahan.

Logika retry cocok dengan kata-kata kesalahan upstream, jadi teruskan body respons kesalahan tanpa modifikasi. Gateway yang membungkus kesalahan upstream dalam amplop miliknya sendiri memecah jalur pemulihan, bahkan ketika mempertahankan kode status, kecuali pesan amplop membawa token `capability_rejected:` yang stabil. [Gateway aplikasi Claude mengganti token tersebut untuk kata-kata kesalahan penyedia cloud](/docs/id/claude-apps-gateway-config#upstream-error-messages), misalnya `capability_rejected: prompt_too_long`.

<h3 id="disable-pre-release-capabilities">
  Nonaktifkan kemampuan pra-rilis
</h3>

`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1` menghentikan Claude Code dari mengirim kemampuan pra-rilis dan field body mereka pada setiap penyedia, termasuk manajemen konteks dan field alat beta. Variabel tidak mempengaruhi penalaran adaptif, yang dipilih oleh model daripada oleh beta. Itu tidak pernah menekan kemampuan OAuth yang diperlukan autentikasi langganan.

Pada Claude Code v2.1.227 atau lebih baru, organisasi Anda dapat menjaga [pencarian alat MCP](/docs/id/mcp#scale-with-mcp-tool-search) tetap aktif di bawah variabel ini melalui [pengaturan terkelola](/docs/id/managed-settings). Apa yang Claude Code kirimkan dengan override tersebut berlaku tergantung pada cara Anda terhubung:

* Pada koneksi langsung, atau melalui gateway yang diatur dengan `ANTHROPIC_BASE_URL`, Claude Code terus mengirim header beta pencarian alat, field alat `defer_loading`, dan blok `tool_reference`, dan menghapus sisanya
* Pada penyedia cloud, atau masuk melalui [gateway aplikasi Claude](/docs/id/claude-apps-gateway), override tidak memiliki efek

Set kemampuan Claude Code mengirim tumbuh di seluruh rilis. Untuk string header beta saat ini, lihat [referensi header beta](https://platform.claude.com/docs/en/api/beta-headers); uji gateway Anda terhadap rilis Claude Code baru daripada menyematkan ke daftar yang diamati.

<h2 id="model-discovery">
  Penemuan model
</h2>

Ketika `ANTHROPIC_BASE_URL` menunjuk ke gateway yang mengekspos format Anthropic Messages, Claude Code dapat menanyakan endpoint `/v1/models` gateway pada startup dan menambahkan model yang dikembalikan ke pemilih `/model`. Jika Anda atau administrator Anda menetapkan `replaceBuiltInOptions` dalam lineup [`modelPicker`](/docs/id/settings-reference#modelpicker), Claude Code menyembunyikan model yang ditemukan dari pemilih.

Pengembang mengaktifkannya dengan menetapkan [`CLAUDE_CODE_ENABLE_GATEWAY_MODEL_DISCOVERY=1`](/docs/id/env-vars), di lingkungan mereka sendiri atau melalui pengaturan terkelola. Penemuan dimatikan secara default sehingga gateway yang didukung oleh kunci API bersama tidak menampilkan setiap model yang dapat diakses kunci kepada setiap pengguna.

<h3 id="when-discovery-runs">
  Ketika penemuan berjalan
</h3>

Penemuan hanya berlaku untuk format Anthropic Messages. Ini tidak berjalan ketika:

* Variabel penyedia `CLAUDE_CODE_USE_*` apa pun diatur, bahkan jika `ANTHROPIC_BASE_URL` juga diatur
* `ANTHROPIC_BASE_URL` tidak diatur atau menunjuk ke `api.anthropic.com`

Penemuan masih berjalan ketika [lalu lintas nonessential dimatikan](/docs/id/llm-gateway-connect#turn-off-traffic-outside-the-gateway-path), karena permintaan hanya pergi ke gateway Anda. Sebelum v2.1.257, penemuan tidak berjalan saat lalu lintas nonessential dimatikan.

<h3 id="request-and-response">
  Permintaan dan respons
</h3>

Permintaan adalah `GET /v1/models?limit=1000` dengan timeout 3 detik secara default, dan pengalihan apa pun diperlakukan sebagai kegagalan sehingga kredensial tidak dapat bocor ke target pengalihan. Gateway yang merespons lebih lambat dari timeout, atau yang mengalihkan `/v1/models`, bahkan `http` ke `https`, gagal penemuan diam-diam; sajikan endpoint langsung di URL dasar yang dikonfigurasi.

Untuk memberikan gateway yang lambat waktu lebih lama, atur [`CLAUDE_CODE_GATEWAY_MODEL_DISCOVERY_TIMEOUT_MS`](/docs/id/env-vars#variables). Variabel memerlukan Claude Code v2.1.269 atau lebih baru.

Claude Code mengirim permintaan penemuan dengan kedua header kredensial di bawah dan menghilangkan header yang nilainya tidak diselesaikan. Mengirim kedua header memerlukan Claude Code v2.1.248 atau lebih baru. Versi sebelumnya mengirim hanya `Authorization` ketika `ANTHROPIC_AUTH_TOKEN` diatur dan hanya `x-api-key` sebaliknya.

* `Authorization`: `ANTHROPIC_AUTH_TOKEN` sebagai token bearer, jika tidak nilai [`apiKeyHelper`](/docs/id/llm-gateway-connect#rotate-credentials-with-apikeyhelper) sebagai token bearer. Dalam hal ini Claude Code menunggu helper mengembalikan sebelum mengirim permintaan.
* `x-api-key`: kunci API yang diselesaikan Claude Code, seperti `ANTHROPIC_API_KEY`. Ketika nilai helper adalah satu-satunya kredensial, header ini juga membawanya, sehingga nilai tiba di kedua header.

Claude Code juga mengirim header apa pun dari `ANTHROPIC_CUSTOM_HEADERS`. Ketika header kustom memiliki nilai non-kosong, Claude Code mengirimnya sebagai pengganti header built-in dengan nama yang sama, mencocokkan nama secara case-insensitive.

Ketika nilai header kredensial tidak diselesaikan, Claude Code melewati penemuan dan menulis baris `[gatewayDiscovery] skipped` ke log debug dari sesi `claude --debug`. Jika Anda menyediakan kredensial hanya melalui `ANTHROPIC_CUSTOM_HEADERS`, Claude Code masih melewati penemuan.

Claude Code membaca `id`, `display_name` opsional, dan `description` opsional dari setiap entri dalam array `data` respons:

```json theme={null}
{
  "data": [
    {
      "id": "claude-sonnet-4-6",
      "display_name": "Claude Sonnet 4.6",
      "description": "Default model for everyday coding tasks"
    },
    { "id": "claude-opus-4-8" }
  ]
}
```

Claude Code menyimpan entri ketika `id` nya berisi `claude` atau `anthropic` di mana saja dalam string, cocok case-insensitive, dan mengabaikan sisanya. ID dengan awalan penyedia seperti `vertex_ai/claude-sonnet-4-6` atau `bedrock/anthropic.claude-sonnet-4-5` melewati filter; ID yang tidak berisi substring apa pun tidak. Sebelum v2.1.223, Claude Code menyimpan entri hanya ketika `id` nya dimulai dengan `claude` atau `anthropic`, yang menyembunyikan ID dengan awalan penyedia.

<h3 id="picker-entries-and-caching">
  Entri pemilih dan caching
</h3>

Pemilih adalah daftar model interaktif yang terbuka ketika pengembang menjalankan `/model` di Claude Code. Setiap entri yang ditemukan menggunakan `display_name` sebagai namanya ketika gateway mengirim satu yang berbeda dari `id`. Jika tidak, entri menampilkan nama model ketika Claude Code [mengenali `id`](/docs/id/model-config#customize-pinned-model-display-and-capabilities), dan `id` ketika tidak. Misalnya, entri dengan `id` `my-gateway-claude-sonnet-4-6` dan tidak ada `display_name` muncul sebagai `Sonnet 4.6`.

Penemuan menambahkan hanya model yang diizinkan oleh pengaturan terkelola [`availableModels`](/docs/id/settings-reference#availablemodels).

Setiap entri juga menampilkan `description` model, runtuh menjadi satu baris. Entri tanpa `description` membaca "From gateway" sebagai gantinya. Sebelum v2.1.257, setiap entri yang ditemukan membaca "From gateway".

ID yang ditemukan tidak mendapatkan barisnya sendiri ketika cocok dengan baris yang sudah ada di pemilih:

* ID yang sama: ID yang ditemukan cocok persis dengan ID baris yang ada, atau dua ID adalah ejaan dari versi [Fable](/docs/id/model-config#work-with-fable) yang sama.
* Model yang sama dengan alias built-in: ketika ID eksplisit yang ditemukan menamai model yang alias built-in saat ini diselesaikan, pemilih menampilkan hanya baris alias. Misalnya, sementara `sonnet` diselesaikan ke `claude-sonnet-5`, `claude-sonnet-5` yang ditemukan runtuh ke baris `sonnet`, dan `claude-sonnet-4-6` yang ditemukan masih mendapatkan barisnya sendiri. Sebelum v2.1.197, Claude Code tidak melipat ID ini ke baris built-in, jadi `claude-sonnet-5` juga mendapatkan barisnya sendiri "From gateway".

Hasil di-cache ke `~/.claude/cache/gateway-models.json`, atau `%USERPROFILE%\.claude\cache\gateway-models.json` di Windows, dan disegarkan pada setiap startup. Jika Anda menetapkan [`CLAUDE_CONFIG_DIR`](/docs/id/env-vars), cache berada di bawah direktori itu sebagai gantinya. Jika permintaan gagal atau gateway tidak mengimplementasikan `/v1/models`, pemilih kembali ke daftar cache dari startup sebelumnya atau ke daftar model built-in. Jika gateway Anda melayani model Claude di bawah alias yang tidak cocok dengan filter penemuan, pengembang dapat menambahkan alias tersebut secara manual dengan variabel [konfigurasi model](/docs/id/model-config).

<h2 id="related-resources">
  Sumber daya terkait
</h2>

Untuk sisa set dokumentasi gateway dan referensi API yang mendasarinya:

* [Ikhtisar gateway](/docs/id/gateways): apa itu gateway dan cara memilih antara gateway aplikasi Claude dan produk lainnya
* [Gateway LLM lainnya](/docs/id/llm-gateway): cara meluncurkan gateway yang dijalankan organisasi Anda dan cara berinteraksinya dengan langganan claude.ai
* [Meluncurkan gateway LLM untuk organisasi Anda](/docs/id/llm-gateway-rollout): daftar periksa admin yang menggunakan panduan ini
* [Menghubungkan Claude Code ke gateway LLM](/docs/id/llm-gateway-connect): konfigurasi per-pengembang dan tabel pemecahan masalah
* [Referensi header beta](https://platform.claude.com/docs/en/api/beta-headers): set nilai `anthropic-beta` saat ini
* [Messages API](https://platform.claude.com/docs/en/api/messages): format API yang diimplementasikan gateway format Anthropic
