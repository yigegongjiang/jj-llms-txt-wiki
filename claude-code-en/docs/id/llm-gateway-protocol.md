> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Referensi protokol gateway

> Kontrak API antara Claude Code dan gateway LLM: endpoint, header dan field body untuk diteruskan, degradasi fitur ketika field dihapus, header atribusi untuk pelacakan biaya, dan penemuan model.

Halaman ini mendokumentasikan permintaan yang dikirim Claude Code ke gateway, termasuk endpoint yang dipanggilnya, header dan field body yang harus diteruskan gateway, dan fitur mana yang berhenti berfungsi ketika tidak ada. Halaman ini ditulis untuk operator yang mengonfigurasi produk gateway agar bekerja dengan Claude Code.

Gateway [Claude apps gateway](/docs/id/claude-apps-gateway) yang berjalan melayani versi yang dapat dibaca mesin dari kontrak ini di `GET /protocol`, mencakup persyaratan penerusan yang sama ditambah endpoint spesifik Claude apps gateway untuk SSO sign-in, pengiriman managed-settings, dan telemetri. Claude apps gateway berjalan dari binary `claude` yang sama dengan CLI, jadi [Claude apps gateway quickstart](/docs/id/claude-apps-gateway#quickstart) adalah jalur tercepat ke instance yang berjalan dari mana Anda dapat mengambil spesifikasinya.

<Note>
  * Untuk meluncurkan gateway yang sudah ada atau pihak ketiga untuk organisasi Anda, lihat [Meluncurkan gateway LLM](/docs/id/llm-gateway-rollout)
  * Jika Anda adalah pengembang individual yang mengautentikasi Claude Code ke gateway dengan kredensial yang diberikan kepada Anda, lihat [Menghubungkan Claude Code ke gateway LLM](/docs/id/llm-gateway-connect)
</Note>

Halaman ini mencakup:

* [Format API](#api-formats) dan endpoint yang harus disajikan untuk masing-masing
* [Header permintaan](#request-headers): mana yang harus mencapai upstream dan mana yang dapat dikonsumsi gateway Anda
* [Blok atribusi prompt sistem](#system-prompt-attribution-block) dan cara berinteraksinya dengan prompt caching
* [Penerusan fitur](#feature-pass-through): apa yang rusak ketika header atau field body dihapus
* [Penemuan model](#model-discovery)

Halaman ini menggunakan dua istilah untuk apa yang dilakukan gateway Anda dengan setiap header dan field body:

* **Teruskan tanpa perubahan**: teruskan ke upstream byte-for-byte
* **Konsumsi**: gateway dapat membacanya untuk routing, atribusi, atau tracing dan tidak perlu meneruskannya

Apa pun yang tidak ditandai teruskan tanpa perubahan adalah milik Anda untuk dikonsumsi atau diabaikan.

<h2 id="api-formats">
  Format API
</h2>

Gateway harus mengekspos setidaknya satu dari format API berikut ke klien Claude Code. Format mana yang digunakan Claude Code ditentukan oleh konfigurasi klien: variabel di kolom Dipilih oleh tabel di bawah menunjukkan Claude Code ke gateway Anda dalam format tersebut. Google Cloud's Agent Platform adalah endpoint Claude Google Cloud, sebelumnya Vertex AI; nama variabelnya tetap mempertahankan ejaan `VERTEX`.

| Format                                   | Dipilih oleh                                                    | Endpoint                                                                 | Teruskan tanpa perubahan                                                                                  |
| :--------------------------------------- | :-------------------------------------------------------------- | :----------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------- |
| Anthropic Messages                       | `ANTHROPIC_BASE_URL`                                            | `/v1/messages`, `/v1/messages/count_tokens` (opsional)                   | header permintaan `anthropic-beta` dan `anthropic-version`                                                |
| Amazon Bedrock InvokeModel               | `ANTHROPIC_BEDROCK_BASE_URL` dengan `CLAUDE_CODE_USE_BEDROCK=1` | `/model/{model}/invoke`, `/model/{model}/invoke-with-response-stream`    | field body permintaan `anthropic_beta` dan `anthropic_version`                                            |
| Google Cloud's Agent Platform rawPredict | `ANTHROPIC_VERTEX_BASE_URL` dengan `CLAUDE_CODE_USE_VERTEX=1`   | `:rawPredict`, `:streamRawPredict`, `count-tokens:rawPredict` (opsional) | header permintaan `anthropic-beta` dan `anthropic-version`, dan field body permintaan `anthropic_version` |

<h3 id="foundry-and-claude-platform-on-aws">
  Foundry dan Claude Platform on AWS
</h3>

Microsoft Foundry dan [Claude Platform on AWS](/docs/id/claude-platform-on-aws) mengimplementasikan format Anthropic Messages. Claude Code merutekan ke mereka melalui variabel mereka sendiri, `ANTHROPIC_FOUNDRY_BASE_URL` dan `ANTHROPIC_AWS_BASE_URL`, tetapi gateway yang berada di depan salah satu dari mereka mengimplementasikan baris Anthropic Messages di atas. Gateway yang berada di depan Claude Platform on AWS juga harus meneruskan header `anthropic-workspace-id`, yang [platform tersebut memerlukan pada setiap permintaan](/docs/id/claude-platform-on-aws).

<h3 id="optional-endpoints-and-startup-traffic">
  Endpoint opsional dan lalu lintas startup
</h3>

Endpoint penghitungan token adalah satu-satunya yang opsional: ketika tidak ada, Claude Code memperkirakan penggunaan konteks secara lokal. Permintaan inferensi diposting ke `/v1/messages?beta=true`, jadi cocokkan pada path, bukan URL lengkap. Metode Google Cloud's Agent Platform menambahkan sufiks ke path model penerbit, seperti dalam `/projects/{project}/locations/{location}/publishers/anthropic/models/{model}:streamRawPredict`.

Gateway juga melihat lalu lintas startup upaya terbaik yang dapat ditolak tanpa merusak apa pun: probe konektivitas `HEAD /`, dan pada gateway format Amazon Bedrock permintaan `GET /inference-profiles?type=SYSTEM_DEFINED`.

<h3 id="streaming">
  Streaming
</h3>

Respons inferensi harus streaming. Claude Code mengonsumsi server-sent events saat tiba, jadi gateway yang membuffer respons lengkap sebelum meneruskannya menghentikan klien.

<h3 id="format-mismatch-with-the-upstream">
  Ketidakcocokan format dengan upstream
</h3>

Format mana yang digunakan klien menentukan apa yang diterima gateway Anda. Mode kegagalan umum adalah ketidakcocokan antara format yang dikirim klien ke gateway Anda dan format yang diterima penyedia upstream di belakangnya.

* Ketika klien berbicara format Amazon Bedrock atau Google Cloud's Agent Platform, Claude Code mengirim hanya subset dari set kemampuan penuhnya yang diterima penyedia tersebut
* Ketika klien berbicara format Anthropic Messages, Claude Code mengirim set lengkap, bahkan jika gateway Anda meneruskan ke upstream Amazon Bedrock atau Google Cloud's Agent Platform

Menjembatani perbedaan itu adalah pekerjaan gateway Anda. [Penerusan fitur](#feature-pass-through) menjelaskan apa yang rusak ketika tidak ada.

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

<h3 id="forward-as-open-lists">
  Teruskan sebagai daftar terbuka
</h3>

Perlakukan header dan field body sebagai daftar terbuka, bukan daftar tertutup. Claude Code mendapatkan kemampuan di seluruh rilis, dan mereka tiba sebagai nilai `anthropic-beta` baru, field body permintaan baru, dan kadang-kadang header `anthropic-*` atau `x-claude-code-*` baru.

Ketika meneruskan ke upstream format Anthropic, teruskan header permintaan `anthropic-*` dan field body permintaan melalui tanpa perubahan daripada allowlist yang Anda lihat hari ini. Gateway yang disematkan ke daftar yang diamati menghapus header atau field kemampuan berikutnya dan merusaknya pada rilis yang memperkenalkannya.

Pengecualiannya adalah upstream non-Anthropic seperti Amazon Bedrock atau Google Cloud's Agent Platform, di mana menjembatani perbedaan skema adalah pekerjaan gateway; lihat [penerusan fitur](#feature-pass-through).

<h2 id="system-prompt-attribution-block">
  Blok atribusi prompt sistem
</h2>

Claude Code menambahkan blok atribusi pendek ke prompt sistem yang berisi versi klien dan sidik jari yang berasal dari percakapan. Endpoint `api.anthropic.com` menghapus blok sebelum memproses ketika tiba tidak berubah sebagai blok sistem pertama, jadi tidak mempengaruhi prompt caching pihak pertama. Upstream lain apa pun menerimanya sebagai bagian dari prompt.

Strip bersifat posisional, jadi hanya berfungsi ketika gateway meneruskan array `system` tanpa perubahan. Untuk menjaga blok keluar dari prompt tanpa kehilangan konten sistem lainnya:

* Teruskan array `system` persis seperti yang diterima, menjaga blok tetap pertama: menambahkan blok sistem lain, mengurutkan ulang array, atau mengonversinya menjadi string tunggal mengalahkan strip, dan blok kemudian mencapai model dan kunci cache prompt.
* Jaga blok dalam entri array-nya sendiri: endpoint memperlakukan blok yang digabungkan yang dimulai dengan header atribusi sebagai atribusi sepenuhnya dan menghapus semua yang digabungkan ke dalamnya, termasuk sisa prompt sistem.
* Jika gateway Anda harus membentuk ulang konten sistem, atur [`CLAUDE_CODE_ATTRIBUTION_HEADER=0`](/docs/id/env-vars) sehingga Claude Code menghilangkan blok. Anthropic dan endpoint Claude penyedia cloud membacanya untuk atribusi, jadi hilangkan di klien daripada menghapusnya atau memindahkannya di gateway.

Permintaan yang mencapai endpoint tanpa dimodifikasi tidak terpengaruh.

Dari Claude Code v2.1.181, blok stabil untuk seumur hidup percakapan ketika permintaan merutekan melalui URL dasar kustom, jadi cache prompt gateway-side yang dikunci pada body permintaan lengkap bekerja tanpa menonaktifkannya. Sebelum v2.1.181 blok menyertakan token per-permintaan; pada versi tersebut, atur `CLAUDE_CODE_ATTRIBUTION_HEADER=0` jika gateway Anda mengimplementasikan cache seperti itu.

<h2 id="feature-pass-through">
  Penerusan fitur
</h2>

Claude Code memperlakukan gateway `ANTHROPIC_BASE_URL` sebagai endpoint format Anthropic dan mengirimkannya header beta dan field body permintaan yang dikirimkannya ke `api.anthropic.com`, kecuali set kecil diagnostik dan default yang disediakan untuk koneksi langsung, seperti default streaming alat berbutir halus yang tercakup di bawah. Set tersebut bervariasi menurut rilis, jadi jangan bergantung pada isinya.

Kemampuan yang menambahkan field body memasangkannya dengan header beta, dan pasangan bepergian bersama. Gateway yang menghapus header sambil melewatkan body, atau meneruskan body format Anthropic ke upstream dengan skema berbeda, menghasilkan kesalahan `400` keras; hanya ketika kedua bagian tidak ada bersama-sama fitur mati diam-diam. Gateway yang menulis ulang atau menyunting body permintaan untuk inspeksi konten memecah pasangan dengan cara yang sama seperti penghapusan, jadi inspeksi tanpa memodifikasi. Tabel mencatat di mana fitur menyimpang dari pasangan.

Streaming alat berbutir halus adalah salah satu default koneksi langsung: itu dimatikan secara default setiap kali permintaan merutekan melalui URL dasar kustom, dan gateway menerimanya ketika pengembang menetapkan [`CLAUDE_CODE_ENABLE_FINE_GRAINED_TOOL_STREAMING=1`](/docs/id/env-vars).

| Fitur                                                                                                                                                                                                                                              | Header dan pasangan body                                                                                                                                                                                                             | Gejala ketika rusak                                                                                                                            | Remediasi                                                                                                                               |
| :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------- |
| [Penalaran adaptif](/docs/id/model-config#adjust-effort-level)                                                                                                                                                                                          | Tidak ada header beta. Claude Code mengirim `thinking: {"type": "adaptive"}` untuk Claude 4.6 dan lebih baru, dan memperlakukan nama model yang tidak dikenalinya, seperti alias gateway, sebagai model saat ini yang menerima field | `400` penamaan field `thinking` atau tag `adaptive` ketika build model upstream tidak menerimanya                                              | Tingkatkan upstream. Pada Opus 4.6 dan Sonnet 4.6, pengembang dapat mengatur `CLAUDE_CODE_DISABLE_ADAPTIVE_THINKING=1` sebagai gantinya |
| [Manajemen konteks](https://platform.claude.com/docs/en/build-with-claude/context-management)                                                                                                                                                      | Header beta manajemen konteks berpasangan dengan field body `context_management`                                                                                                                                                     | `400` dengan `Extra inputs are not permitted`. Umum ketika gateway menerima permintaan format Anthropic tetapi meneruskannya ke Amazon Bedrock | Teruskan keduanya, atau [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1`](/docs/id/env-vars)                                                      |
| [Konteks diperluas](https://platform.claude.com/docs/en/build-with-claude/context-windows#context-window-sizes-by-model) dan [pemikiran interleaved](https://platform.claude.com/docs/en/build-with-claude/extended-thinking#interleaved-thinking) | Hanya header beta, tidak ada field body                                                                                                                                                                                              | Diam-diam tidak tersedia ketika header dihapus; upstream tidak pernah melihat permintaan kemampuan                                             | Teruskan `anthropic-beta` secara verbatim                                                                                               |
| Beta [field alat](https://platform.claude.com/docs/en/agents-and-tools/tool-use/overview)                                                                                                                                                          | Header beta terkait alat berpasangan dengan field skema alat seperti `strict` dan `defer_loading`                                                                                                                                    | `400` penamaan field skema alat yang tidak dikenali ketika body melewati tanpa headernya                                                       | Teruskan keduanya, atau `CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1`                                                                      |
| [Upaya](https://platform.claude.com/docs/en/build-with-claude/effort) dan [output terstruktur](https://platform.claude.com/docs/en/build-with-claude/structured-outputs)                                                                           | Field body `output_config` membawa upaya, format output terstruktur, dan pengaturan anggaran tugas; masing-masing berpasangan dengan header betanya sendiri                                                                          | `400` penamaan `output_config`, sering `Extra inputs are not permitted`, pada upstream Amazon Bedrock dan Agent Platform Google Cloud          | Teruskan field dan headernya bersama-sama                                                                                               |
| [Penghitungan token](https://platform.claude.com/docs/en/build-with-claude/token-counting)                                                                                                                                                         | Tidak ada pasangan beta; menggunakan endpoint `count_tokens`                                                                                                                                                                         | Claude Code kembali ke estimasi penggunaan konteks secara lokal                                                                                | Ekspos endpoint jika Anda menginginkan hitungan yang tepat                                                                              |

Variabel `ANTHROPIC_DEFAULT_*_MODEL_SUPPORTED_CAPABILITIES` [](/docs/id/model-config) mendeklarasikan kemampuan model hanya dalam konfigurasi penyedia: `CLAUDE_CODE_USE_BEDROCK`, `CLAUDE_CODE_USE_VERTEX`, `CLAUDE_CODE_USE_FOUNDRY`, dan [`CLAUDE_CODE_USE_MANTLE`](/docs/id/amazon-bedrock#use-the-mantle-endpoint). Mereka tidak memiliki efek di belakang gateway `ANTHROPIC_BASE_URL`.

<h3 id="automatic-retry-and-error-forwarding">
  Retry otomatis dan penerusan kesalahan
</h3>

Claude Code retry secara otomatis setelah beberapa penolakan upstream dan menonaktifkan kemampuan yang ditolak untuk sisa percakapan. Penolakan field `thinking`, dari [tanda tangan pemikiran](https://platform.claude.com/docs/en/build-with-claude/extended-thinking), dan dari pesan sistem mid-conversation semuanya pulih dengan cara ini. Penolakan manajemen konteks dan field skema alat tidak retry; kesalahan `400` tersebut mencapai pengembang.

Logika retry cocok dengan kata-kata kesalahan upstream, jadi teruskan body respons kesalahan tanpa modifikasi. Gateway yang membungkus kesalahan upstream dalam amplop miliknya sendiri memecah jalur pemulihan bahkan ketika mempertahankan kode status.

<h3 id="disable-pre-release-capabilities">
  Nonaktifkan kemampuan pra-rilis
</h3>

`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1` menghentikan Claude Code dari mengirim kemampuan pra-rilis dan field body mereka pada setiap penyedia, termasuk manajemen konteks dan field alat beta. Ini tidak mempengaruhi penalaran adaptif, yang dipilih oleh model daripada oleh beta, dan tidak pernah menekan kemampuan OAuth yang diperlukan autentikasi langganan.

Set kemampuan Claude Code mengirim tumbuh di seluruh rilis. Untuk string header beta saat ini, lihat [referensi header beta](https://platform.claude.com/docs/en/api/beta-headers); uji gateway Anda terhadap rilis Claude Code baru daripada menyematkan ke daftar yang diamati.

<h2 id="model-discovery">
  Penemuan model
</h2>

Ketika `ANTHROPIC_BASE_URL` menunjuk ke gateway yang mengekspos format Anthropic Messages, Claude Code dapat menanyakan endpoint `/v1/models` gateway pada startup dan menambahkan model yang dikembalikan ke pemilih `/model`.

Pengembang mengaktifkannya dengan menetapkan [`CLAUDE_CODE_ENABLE_GATEWAY_MODEL_DISCOVERY=1`](/docs/id/env-vars), di lingkungan mereka sendiri atau melalui pengaturan terkelola. Penemuan dimatikan secara default sehingga gateway yang didukung oleh kunci API bersama tidak menampilkan setiap model yang dapat diakses kunci kepada setiap pengguna. Ini memerlukan Claude Code v2.1.129 atau lebih baru.

<h3 id="when-discovery-runs">
  Ketika penemuan berjalan
</h3>

Penemuan hanya berlaku untuk format Anthropic Messages. Ini tidak berjalan ketika:

* Variabel penyedia `CLAUDE_CODE_USE_*` apa pun diatur, bahkan jika `ANTHROPIC_BASE_URL` juga diatur
* `ANTHROPIC_BASE_URL` tidak diatur atau menunjuk ke `api.anthropic.com`
* Lalu lintas nonessential dinonaktifkan, melalui [`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`](/docs/id/env-vars) atau kebijakan organisasi

<h3 id="request-and-response">
  Permintaan dan respons
</h3>

Permintaan adalah `GET /v1/models?limit=1000` dengan timeout 3 detik, dan pengalihan apa pun diperlakukan sebagai kegagalan sehingga kredensial tidak dapat bocor ke target pengalihan. Gateway yang merespons lambat atau mengalihkan `/v1/models`, bahkan `http` ke `https`, gagal penemuan diam-diam; sajikan endpoint langsung di URL dasar yang dikonfigurasi.

Permintaan penemuan mengirim tepat satu header kredensial:

* `ANTHROPIC_AUTH_TOKEN` sebagai token bearer, ketika diatur
* Jika tidak, kunci API yang diselesaikan, termasuk nilai [`apiKeyHelper`](/docs/id/llm-gateway-connect#rotate-credentials-with-apikeyhelper), di header `x-api-key`

Ini berbeda dari permintaan inferensi, yang mengirim nilai helper di kedua header. Gateway yang mengautentikasi `/v1/models` harus menerima `x-api-key` untuk deployment helper. Header apa pun dari `ANTHROPIC_CUSTOM_HEADERS` juga disertakan.

Claude Code membaca `id` dan `display_name` opsional dari setiap entri dalam array `data` respons, dan mengabaikan entri yang `id` tidak dimulai dengan `claude` atau `anthropic`:

```json theme={null}
{
  "data": [
    { "id": "claude-sonnet-4-6", "display_name": "Claude Sonnet 4.6" },
    { "id": "claude-opus-4-8" }
  ]
}
```

<h3 id="picker-entries-and-caching">
  Entri pemilih dan caching
</h3>

Pemilih adalah daftar model interaktif yang terbuka ketika pengembang menjalankan `/model` di Claude Code. Setiap entri yang ditemukan diberi label "Dari gateway" dan menggunakan `display_name` ketika disediakan. Pengaturan terkelola [`availableModels`](/docs/id/settings#available-settings) membatasi apa yang dapat ditambahkan penemuan.

ID yang ditemukan dilewati ketika cocok persis dengan baris yang sudah ada di pemilih, atau ketika ID yang ditemukan dan yang ada keduanya diselesaikan ke [Fable](/docs/id/model-config#work-with-fable-5). Mulai dari Claude Code v2.1.197, ID eksplisit yang ditemukan juga dilipat ke entri built-in ketika keduanya diselesaikan ke model yang sama. Baris built-in dikunci pada alias seperti `sonnet`, jadi ID eksplisit yang ditemukan dari model yang alias saat ini diselesaikan, seperti `claude-sonnet-5`, runtuh ke baris `sonnet`, sementara ID yang alias tidak diselesaikan, seperti `claude-sonnet-4-6`, masih menambahkan baris "Dari gateway" miliknya sendiri di samping entri built-in.

Hasil di-cache ke `~/.claude/cache/gateway-models.json`, atau `%USERPROFILE%\.claude\cache\gateway-models.json` di Windows, dan disegarkan pada setiap startup. Jika permintaan gagal atau gateway tidak mengimplementasikan `/v1/models`, pemilih kembali ke daftar cache dari startup sebelumnya atau ke daftar model built-in. Jika gateway Anda melayani model Claude di bawah alias yang tidak cocok dengan filter penemuan, pengembang dapat menambahkan alias tersebut secara manual dengan variabel [konfigurasi model](/docs/id/model-config).

<h2 id="related-resources">
  Sumber daya terkait
</h2>

Untuk sisa set dokumentasi gateway dan referensi API yang mendasarinya:

* [Ikhtisar gateway](/docs/id/gateways): apa itu gateway dan cara memilih antara gateway aplikasi Claude dan produk lainnya
* [Gateway LLM lainnya](/docs/id/llm-gateway): cara meluncurkan gateway yang dijalankan organisasi Anda dan cara berinteraksinya dengan langganan claude.ai
* [Meluncurkan gateway LLM untuk organisasi Anda](/docs/id/llm-gateway-rollout): daftar periksa admin yang menggunakan kontrak ini
* [Menghubungkan Claude Code ke gateway LLM](/docs/id/llm-gateway-connect): konfigurasi per-pengembang dan tabel pemecahan masalah
* [Referensi header beta](https://platform.claude.com/docs/en/api/beta-headers): set nilai `anthropic-beta` saat ini
* [Messages API](https://platform.claude.com/docs/en/api/messages): format API yang diimplementasikan gateway format Anthropic
