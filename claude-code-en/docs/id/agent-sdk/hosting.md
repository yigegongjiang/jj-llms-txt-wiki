> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Hosting the Agent SDK

> Menerapkan Agent SDK dalam produksi: arsitektur subprocess, persistensi sesi, penskalaan, observabilitas, dan isolasi multi-tenant untuk Docker, Kubernetes, dan penyedia sandbox.

Agent SDK menjalankan dan mengawasi subprocess `claude` CLI yang memiliki shell, direktori kerja, dan file sesi di disk. Menghosting ini tidak seperti menghosting pembungkus API stateless. Setiap agen yang berjalan adalah proses yang berumur panjang yang terikat pada status lokal, yang membentuk cara Anda mengalokasikan sumber daya, mempertahankan sesi, dan menskalakan di seluruh tenant.

Halaman ini mencakup self-hosting pada infrastruktur Anda sendiri: pahami [model subprocess](#the-subprocess-model), [pilih pola sesi](#choose-a-session-pattern), [sediakan kontainer](#provision-the-container), dan [tangani masalah produksi](#handle-production-concerns) seperti persistensi, observabilitas, autentikasi, dan isolasi multi-tenant. Untuk Dockerfile yang dapat digunakan dan manifes Kubernetes, lihat [hosting cookbook](https://github.com/anthropics/claude-cookbooks/tree/main/claude_agent_sdk/hosting).

Jika Anda tidak memerlukan kontrol infrastruktur, isolasi khusus, atau data plane Anda sendiri, pertimbangkan [Managed Agents](https://platform.claude.com/docs/id/managed-agents/overview) sebagai gantinya: REST API yang dihosting di mana Anthropic menjalankan agen dan sandbox, sehingga aplikasi Anda mengirim peristiwa dan streaming kembali hasil tanpa infrastruktur hosting untuk dioperasikan.

<Info>
  Untuk pengerasan keamanan di luar sandboxing dasar, termasuk kontrol jaringan, manajemen kredensial, dan opsi isolasi, lihat [Secure Deployment](/docs/id/agent-sdk/secure-deployment).
</Info>

<h2 id="the-subprocess-model">
  Model subprocess
</h2>

Setiap keputusan hosting di halaman ini mengikuti dari cara SDK menjalankan agen. Ketika kode Anda memanggil `query()`, SDK menjalankan proses CLI `claude` terpisah dan berkomunikasi dengannya melalui stdio. Subprocess tersebut memiliki shell, direktori kerja, dan transkrip sesi JSONL di disk lokal.

<img src="https://mintcdn.com/claude-code/ikqp3_70mqIahteV/images/agent-sdk/hosting-subprocess.svg?fit=max&auto=format&n=ikqp3_70mqIahteV&q=85&s=9dac857ca9d3b1410c3734900c386004" alt="Request flow: client to your app, which spawns a claude CLI subprocess over stdio inside the container; the subprocess writes to local disk and calls api.anthropic.com over HTTPS" width="920" height="220" data-path="images/agent-sdk/hosting-subprocess.svg" />

Satu sesi agen memetakan ke satu subprocess. Menjalankan N sesi bersamaan berarti N subprocess, masing-masing dengan pohon proses dan file transkrip sendiri. Secara default mereka semua mewarisi direktori kerja aplikasi Anda, jadi teruskan `cwd` pada setiap panggilan `query()` ketika sesi memerlukan sistem file terpisah:

<CodeGroup>
  ```typescript TypeScript theme={null}
  query({ prompt, options: { cwd: "/work/session-a" } })
  ```

  ```python Python theme={null}
  query(prompt=prompt, options=ClaudeAgentOptions(cwd="/work/session-a"))
  ```
</CodeGroup>

<h3 id="state-that-lives-on-local-disk">
  State yang berada di disk lokal
</h3>

Tiga jenis state agen berada di sistem file kontainer secara default. Tidak satupun dari mereka bertahan dari restart kontainer, scale-down, atau perpindahan ke node yang berbeda.

| State                       | Lokasi Default                                                                             |
| --------------------------- | ------------------------------------------------------------------------------------------ |
| Session transcripts         | `~/.claude/projects/`, atau direktori `projects/` di bawah `CLAUDE_CONFIG_DIR` jika diatur |
| `CLAUDE.md` memory files    | `~/.claude/CLAUDE.md` untuk user tier dan direktori kerja sesi untuk project tier          |
| Working-directory artifacts | Direktori kerja sesi                                                                       |

Untuk mempertahankan transkrip di seluruh host, konfigurasikan adaptor [`SessionStore`](/docs/id/agent-sdk/session-storage). Memory files dan artefak direktori kerja lainnya memerlukan strategi penyimpanan mereka sendiri, seperti volume yang dipasang atau sinkronisasi object-store.

Untuk cara sesi, resumption, dan forking bekerja di tingkat API, lihat [Sessions](/docs/id/agent-sdk/sessions).

<h2 id="choose-a-session-pattern">
  Pilih pola sesi
</h2>

Empat pola ini mencakup siklus hidup sesi: berapa lama kontainer hidup relatif terhadap sesi yang dilayaninya. Untuk tempat kontainer berjalan, [panduan hosting](https://github.com/anthropics/claude-cookbooks/blob/main/claude_agent_sdk/07_Hosting_the_agent.ipynb) memiliki [kode yang dapat digunakan](https://github.com/anthropics/claude-cookbooks/tree/main/claude_agent_sdk/hosting) untuk Docker lokal, Modal, dan Kubernetes. Pilih pola sesi di sini dan target penerapan dari panduan.

<h3 id="ephemeral-sessions">
  Sesi Ephemeral
</h3>

Buat kontainer untuk setiap tugas pengguna dan hancurkan ketika tugas selesai. Terbaik untuk tugas sekali jalan. Pengguna mungkin masih berinteraksi dengan AI saat tugas sedang diselesaikan, tetapi setelah selesai kontainer dihancurkan.

Contoh beban kerja termasuk investigasi dan perbaikan bug, ekstraksi faktur dan tanda terima, terjemahan dokumen, dan transformasi media.

Kontainer menjalankan entrypoint satu kali yang memanggil SDK dan keluar. Contoh di bawah menunjukkan versi TypeScript minimal. Simpan sebagai `entrypoint.mts` atau atur `"type": "module"` di `package.json` sehingga `await` tingkat atas tersedia.

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

const prompt = process.env.TASK_PROMPT!;
for await (const message of query({ prompt, options: { maxTurns: 20 } })) {
  console.log(message);
}
```

<h3 id="long-running-sessions">
  Sesi berjalan lama
</h3>

Jalankan instans kontainer persisten, sering kali menghosting beberapa proses SDK per kontainer, untuk melayani pekerjaan yang sedang berlangsung. Terbaik untuk agen yang mengambil tindakan otonom, melayani konten, atau menangani aliran pesan volume tinggi.

Contoh beban kerja termasuk agen email yang menyeleksi dan merespons surat masuk, pembuat situs yang menghosting situs yang dapat diedit per pengguna melalui port kontainer, dan chatbot yang menangani lalu lintas berkelanjutan dari platform seperti Slack.

Kontainer mengekspos endpoint HTTP atau WebSocket dan memetakan setiap sesi aktif ke kueri yang berumur panjang dan subproses di baliknya. Di TypeScript, gunakan [`streamInput()`](/docs/id/agent-sdk/typescript#query-object) untuk menambahkan giliran ke sesi aktif dan [`startup()`](/docs/id/agent-sdk/typescript#startup) untuk pra-pemanasan subproses sebelum lalu lintas masuk. Di Python, gunakan [`ClaudeSDKClient`](/docs/id/agent-sdk/python#claudesdkclient) untuk menjaga sesi tetap terbuka di seluruh giliran. Ukuran kontainer sehingga dapat menampung jumlah maksimum sesi bersamaan dalam memori.

<h3 id="hybrid-sessions">
  Sesi Hybrid
</h3>

Kontainer ephemeral yang terhidrasi dari [`SessionStore`](/docs/id/agent-sdk/session-storage) saat startup dan mempertahankan pembaruan kembali. Terbaik untuk sesi yang mencakup banyak interaksi tetapi menganggur di antara mereka. Kontainer berhenti selama periode menganggur dan kembali hidup ketika pengguna kembali.

Contoh beban kerja termasuk manajer proyek pribadi dengan check-in intermiten, penelitian mendalam yang dijeda dan dilanjutkan selama berjam-jam, dan agen dukungan pelanggan yang memuat riwayat tiket di seluruh interaksi.

Sesuaikan waktu tunggu idle penyedia Anda dengan seberapa sering Anda mengharapkan pengguna kembali. Mematikan kontainer tanpa `SessionStore` yang dikonfigurasi kehilangan transkrip dengannya, jadi penyimpanan diperlukan untuk pola ini, bukan opsional.

Pola ini bergantung pada melanjutkan sesi berdasarkan ID dengan penyimpanan bersama yang terlampir:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query, type SessionStore } from "@anthropic-ai/claude-agent-sdk";

  declare const userInput: string;
  declare const sessionId: string;          // looked up from your database by user
  declare const sessionStore: SessionStore; // S3, Redis, Postgres, or your own adapter

  for await (const message of query({
    prompt: userInput,
    options: { resume: sessionId, sessionStore },
  })) {
    // ...
  }
  ```

  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions

  async for message in query(
      prompt=user_input,
      options=ClaudeAgentOptions(
          resume=session_id,            # looked up from your database by user
          session_store=session_store,  # S3, Redis, Postgres, or your own adapter
      ),
  ):
      ...
  ```
</CodeGroup>

Lihat [Penyimpanan sesi](/docs/id/agent-sdk/session-storage) untuk antarmuka `SessionStore` lengkap dan adaptor referensi.

<h3 id="multi-agent-container">
  Kontainer multi-agen
</h3>

Jalankan beberapa subproses SDK di dalam satu kontainer. Terbaik untuk agen yang harus berkolaborasi erat, misalnya simulasi multi-agen di mana agen berinteraksi satu sama lain di lingkungan bersama.

Berikan setiap agen direktori kerjanya sendiri sehingga mereka tidak menimpa file satu sama lain, dan isolasi pemuatan pengaturan sehingga file `CLAUDE.md` per-agen tidak bocor di seluruh agen. Lihat [Isolasi multi-penyewa](#multi-tenant-isolation) untuk opsi spesifik.

<h2 id="provision-the-container">
  Menyediakan kontainer
</h2>

<h3 id="container-based-sandboxing">
  Sandboxing berbasis kontainer
</h3>

Jalankan SDK di dalam kontainer bersandbox untuk isolasi proses, batasan sumber daya, kontrol jaringan, dan sistem file yang bersifat sementara. Beberapa penyedia mengkhususkan diri dalam lingkungan kontainer bersandbox yang sesuai dengan model Agent SDK.

Pertanyaan yang harus dijawab saat memilih penyedia:

* **Siapa yang menjalankan sandbox**: penyedia sandbox-as-a-service mengoperasikan infrastruktur untuk Anda, sementara opsi self-hosted memberikan perangkat lunak untuk dijalankan di server Anda sendiri.
* **Latensi cold-start**: berapa lama dari "buat sandbox" hingga "siap menerima permintaan pertama." Pola ephemeral memerlukan start sub-detik. Pola long-running dapat mentoleransi lebih banyak.
* **Penyimpanan persisten**: apakah penyedia menawarkan volume tahan lama atau hanya disk ephemeral. Pola hybrid memerlukan penyimpanan tahan lama di suatu tempat, baik di dalam sandbox atau di sampingnya.
* **Model penetapan harga**: per-detik, per-permintaan, atau penagihan per jam tetap. Penetapan harga per-detik cocok untuk beban kerja ephemeral yang bersifat bursty. Per jam cocok untuk sesi long-running.
* **Jaringan**: dukungan untuk aturan egress khusus, proxy outbound, dan peering VPC pribadi untuk lingkungan yang diatur.

Penyedia untuk dievaluasi:

* [Modal Sandbox](https://modal.com/docs/guide/sandbox), dengan [demo implementation](https://modal.com/docs/examples/claude-slack-gif-creator)
* [Cloudflare Sandboxes](https://github.com/cloudflare/sandbox-sdk)
* [Daytona](https://www.daytona.io/)
* [E2B](https://e2b.dev/)
* [Fly Machines](https://fly.io/docs/machines/)
* [Vercel Sandbox](https://vercel.com/docs/functions/sandbox)

Untuk opsi self-hosted seperti Docker, gVisor, dan Firecracker, dan konfigurasi isolasi terperinci, lihat [Isolation Technologies](/docs/id/agent-sdk/secure-deployment#isolation-technologies).

<h3 id="runtime-dependencies">
  Dependensi runtime
</h3>

Kontainer hanya memerlukan runtime bahasa SDK Anda:

* Python 3.10+ untuk Python SDK, atau Node.js 18+ untuk TypeScript SDK
* Kedua paket SDK menggabungkan binary Claude Code asli untuk platform host, jadi tidak perlu instalasi Claude Code atau Node.js terpisah untuk CLI yang dihasilkan

Binary yang digabungkan disematkan ke versi paket SDK, jadi memperbarui SDK adalah cara Anda memperbarui CLI. SDK mengikuti semver: ambil rilis patch secara berkelanjutan dan tinjau changelog [TypeScript](https://github.com/anthropics/claude-agent-sdk-typescript/blob/main/CHANGELOG.md) atau [Python](https://github.com/anthropics/claude-agent-sdk-python/blob/main/CHANGELOG.md) sebelum mengambil minor.

<h3 id="resources">
  Sumber daya
</h3>

1 GiB RAM, 5 GiB disk, dan 1 CPU per agent adalah titik awal yang wajar untuk instance yang baru dimulai. Penggunaan memori tumbuh dengan panjang sesi dan aktivitas tool, jadi ukuran untuk panjang sesi dan concurrency yang benar-benar Anda butuhkan daripada baseline idle. Lihat [Scaling and concurrency](#scaling-and-concurrency) untuk cara menghitung agent per host.

<h3 id="network">
  Jaringan
</h3>

SDK memerlukan outbound HTTPS ke `api.anthropic.com`, atau ke endpoint regional penyedia Anda saat berjalan di Amazon Bedrock atau Google Cloud's Agent Platform. Jika agent Anda menggunakan [MCP servers](/docs/id/agent-sdk/mcp) atau tool eksternal, mereka memerlukan akses outbound ke endpoint tersebut juga. Untuk production, arahkan traffic outbound melalui proxy egress yang memberlakukan allowlist domain, menyuntikkan kredensial, dan mencatat permintaan. Lihat [Secure Deployment](/docs/id/agent-sdk/secure-deployment) untuk pola lengkapnya.

Untuk traffic inbound, ekspos port HTTP atau WebSocket di kontainer. Aplikasi Anda menangani permintaan klien di port tersebut dan memanggil SDK secara internal; subprocess itu sendiri tidak mendengarkan di jaringan.

<h2 id="handle-production-concerns">
  Menangani kekhawatiran produksi
</h2>

Kerjakan keputusan-keputusan ini sebelum mengirimkan agen yang di-host sendiri.

<h3 id="session-and-state-persistence">
  Persistensi sesi dan status
</h3>

Disk lokal default hilang saat restart, scale-down, atau perpindahan ke node yang berbeda. Untuk sesi apa pun yang diharapkan pengguna untuk dilanjutkan, cerminkan transkrip ke penyimpanan yang tahan lama dengan adaptor [`SessionStore`](/docs/id/agent-sdk/session-storage). Lihat [Implementasi referensi](/docs/id/agent-sdk/session-storage#reference-implementations) untuk adaptor S3, Redis, dan Postgres serta suite kepatuhan untuk milik Anda sendiri.

Tiga hal yang perlu diketahui tentang perilaku `SessionStore`:

* **Transkrip saja**: `SessionStore` mencerminkan transkrip, bukan file memori `CLAUDE.md` atau artefak direktori kerja lainnya. Pasang volume bersama atau sinkronkan yang lain secara terpisah.
* **Cermin, bukan penggantian**: subprocess menulis ke disk lokal terlebih dahulu, dan penyimpanan menerima salinan setiap batch. Penulisan lokal tetap berwenang.
* **Pesan `mirror_error`**: batch yang ditolak penyimpanan dikirim hingga tiga kali total, dengan backoff singkat sebelum setiap percobaan ulang; panggilan yang habis waktu tidak dicoba ulang. Jika batch masih gagal, SDK melepasnya, memancarkan pesan `{ type: "system", subtype: "mirror_error" }`, dan melanjutkan kueri. Beri peringatan pada ini jika daya tahan penyimpanan penting.

<h3 id="observability">
  Observabilitas
</h3>

Agen Agent SDK adalah proses yang berumur panjang yang menjalankan panggilan alat di banyak putaran API. Tanpa telemetri, Anda tidak dapat melihat alat mana yang berjalan, berapa lama waktu yang dibutuhkan, atau di mana sesi terhenti.

SDK mewarisi konfigurasi OpenTelemetry dari lingkungan. Atur variabel lingkungan OTEL di tingkat kontainer atau orchestrator sehingga setiap panggilan `query()` mengekspor span, metrik, dan peristiwa log ke kolektor Anda. Contoh di bawah ini mengaktifkan ekspor OTLP untuk ketiga sinyal. `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA` hanya diperlukan untuk jejak; abaikan jika Anda hanya mengekspor metrik dan log.

```bash title=".env' theme={null}
CLAUDE_CODE_ENABLE_TELEMETRY=1
CLAUDE_CODE_ENHANCED_TELEMETRY_BETA=1
OTEL_TRACES_EXPORTER=otlp
OTEL_METRICS_EXPORTER=otlp
OTEL_LOGS_EXPORTER=otlp
OTEL_EXPORTER_OTLP_PROTOCOL=http/protobuf
OTEL_EXPORTER_OTLP_ENDPOINT=http://collector.example.com:4318
```

Teks prompt dan input alat tidak disertakan dalam ekspor secara default. Lihat [Kontrol data sensitif dalam ekspor](/docs/id/agent-sdk/observability#control-sensitive-data-in-exports) untuk flag opt-in, dan [Observabilitas](/docs/id/agent-sdk/observability) untuk katalog sinyal lengkap.

<h3 id="auth-and-secrets">
  Otentikasi dan rahasia
</h3>

Tiga kekhawatiran otentikasi penting pada waktu hosting:

* **API Anthropic**: subprocess membaca `ANTHROPIC_API_KEY` dari lingkungannya. Suplai dari manajer rahasia Anda, atau atur `ANTHROPIC_BASE_URL` untuk merutekan panggilan model melalui proxy yang menyuntikkan kunci di luar kontainer. Lihat [Manajemen kredensial](/docs/id/agent-sdk/secure-deployment#credential-management) untuk pola proxy dan [Ikhtisar SDK](/docs/id/agent-sdk/overview#get-started) untuk metode otentikasi yang didukung.
* **Inbound**: letakkan otentikasi di gateway di depan kontainer agen. Agen harus menerima permintaan yang telah diautentikasi sebelumnya dan tidak boleh menjadi komponen yang memvalidasi token pengguna.
* **Alat outbound**: jaga kredensial alat keluar dari lingkungan agen. Rutekan panggilan outbound melalui proxy yang menyuntikkan kunci API setelah permintaan meninggalkan kontainer. Agen membuat panggilan; proxy menambahkan kredensial.

<h3 id="scaling-and-concurrency">
  Penskalaan dan konkurensi
</h3>

Setiap sesi berjalan di subprocess-nya sendiri, jadi konkurensi pada host dibatasi oleh berapa banyak subprocess yang dapat ditampung RAM-nya.

Ukuran setiap host dengan rumus ini:

```text theme={null}
agents per host = (host RAM - overhead) / (per-session RAM ceiling)
```

Ukur batas RAM per-sesi dengan menjalankan sesi representatif ke panjang target Anda di bawah beban alat yang diharapkan dan mencatat RSS puncak. Titik awal 1 GiB dalam [Sumber Daya](#resources) adalah lantai, bukan batas.

Perutean horizontal-scale tergantung pada pola Anda. Untuk sesi yang berjalan lama, di mana kontainer menampung banyak sesi, jalankan pool kontainer di belakang load balancer dan pin setiap sesi ke satu kontainer menggunakan consistent hashing pada `sessionId`. Sesi yang disematkan terus mengenai kontainer yang sama, dan oleh karena itu subprocess yang sama yang berjalan, sampai dikeluarkan atau kontainer dimulai ulang.

Fanout besar dari [subagen](/docs/id/agent-sdk/subagents) bersamaan dari sesi tunggal dapat mencapai batas laju API. Pecahkan pekerjaan menjadi batch yang lebih kecil daripada mengeluarkan satu dispatch yang lebar.

<h3 id="cost">
  Biaya
</h3>

Biaya token Anthropic biasanya mendominasi biaya infrastruktur kontainer dengan urutan besarnya atau lebih. Kontainer yang disediakan secara minimal berjalan kira-kira \$0,05 per jam, sementara sesi agen panjang tunggal dapat menghabiskan dolar dalam token. Lihat [Pelacakan biaya](/docs/id/agent-sdk/cost-tracking) untuk akuntansi token per-sesi.

<h3 id="multi-tenant-isolation">
  Isolasi multi-tenant
</h3>

Perilaku SDK default membaca pengaturan dan file memori `CLAUDE.md` dari sistem file. Dalam kontainer bersama yang melayani beberapa tenant, file-file tersebut dapat membocorkan konteks satu tenant ke sesi tenant lain.

Untuk mengisolasi tenant di dalam kontainer bersama:

* Lewatkan `settingSources: []` dalam TypeScript atau `setting_sources=[]` dalam Python sehingga tidak ada pengaturan sistem file yang dimuat.
* Atur `CLAUDE_CODE_DISABLE_AUTO_MEMORY=1` dalam `env`. [Memori otomatis](/docs/id/memory#auto-memory) di `~/.claude/projects/<project>/memory/` dimuat ke prompt sistem terlepas dari `settingSources`. Lihat [Apa yang settingSources tidak kontrol](/docs/id/agent-sdk/claude-code-features#what-settingsources-does-not-control) untuk input lain yang dimuat tanpa syarat.
* Arahkan `CLAUDE_CONFIG_DIR` ke direktori per-tenant sehingga tenant tidak berbagi konfigurasi global `~/.claude.json`.
* Gunakan direktori kerja per-tenant. Lewatkan `cwd` secara eksplisit pada setiap panggilan `query()`.
* Terapkan aturan egress per-tenant di proxy Anda, seperti IP outbound yang berbeda, kredensial, atau daftar allowlist domain, sehingga tenant yang dikompromikan tidak dapat mengeksfiltrasikan data melalui kebijakan outbound tenant lain.

Contoh di bawah ini menerapkan empat opsi tingkat SDK bersama-sama. Bangun `tenantDir` dan `configDir` sehingga setiap tenant mendapatkan jalur yang tidak dapat dibaca tenant lain. Dalam TypeScript, `env` menggantikan lingkungan subprocess, jadi sebarkan `...process.env` untuk menjaga variabel yang diwarisi seperti `PATH` dan `ANTHROPIC_API_KEY`. Dalam Python, `env` digabungkan di atas lingkungan yang diwarisi.

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  declare const prompt: string;
  declare const tenantDir: string;
  declare const configDir: string;

  for await (const message of query({
    prompt,
    options: {
      cwd: tenantDir,
      settingSources: [],
      env: {
        ...process.env,
        CLAUDE_CONFIG_DIR: configDir,
        CLAUDE_CODE_DISABLE_AUTO_MEMORY: "1",
      },
    },
  })) {
    // ...
  }
  ```

  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions

  async for message in query(
      prompt=prompt,
      options=ClaudeAgentOptions(
          cwd=tenant_dir,
          setting_sources=[],
          env={
              "CLAUDE_CONFIG_DIR": config_dir,
              "CLAUDE_CODE_DISABLE_AUTO_MEMORY": "1",
          },
      ),
  ):
      ...
  ```
</CodeGroup>

Untuk kontrol jaringan per-tenant, lihat [Penerapan Aman](/docs/id/agent-sdk/secure-deployment).

<h2 id="known-limitations">
  Keterbatasan yang Diketahui
</h2>

Rencanakan di sekitar ini dalam desain penyebaran Anda.

| Keterbatasan                                           | Apa yang harus dilakukan                                                                                                                                                                                                                                                                                             |
| ------------------------------------------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Tidak ada timeout sesi tingkat atas                    | Sesi tidak akan timeout dengan sendirinya. Atur `maxTurns` dalam `Options` untuk membatasi berapa banyak putaran penggunaan alat yang diambil agen sebelum berhenti.                                                                                                                                                 |
| Pertumbuhan memori selama sesi panjang                 | Batasi panjang sesi atau daur ulang subproses secara berkala. Lihat [Penskalaan dan konkurensi](#scaling-and-concurrency).                                                                                                                                                                                           |
| Fanout subagen paralel besar dapat mencapai batas laju | Pecah pekerjaan menjadi batch yang lebih kecil daripada mengeluarkan satu pengiriman yang luas.                                                                                                                                                                                                                      |
| Tidak ada batas waktu dinding per subagen              | Batasi setiap [subagen](/docs/id/agent-sdk/subagents) dengan `maxTurns` dalam `AgentDefinition`-nya. Hanya untuk subagen latar belakang, `CLAUDE_ASYNC_AGENT_STALL_TIMEOUT_MS` menetapkan watchdog stall yang aktif ketika subagen `run_in_background` berhenti menghasilkan output; ini bukan batas waktu runtime total. |

<h2 id="next-steps">
  Langkah Berikutnya
</h2>

* [Hosting cookbook](https://github.com/anthropics/claude-cookbooks/blob/main/claude_agent_sdk/07_Hosting_the_agent.ipynb): panduan notebook dengan [kode yang dapat digunakan](https://github.com/anthropics/claude-cookbooks/tree/main/claude_agent_sdk/hosting) untuk Docker, Modal, dan Kubernetes.
* [Session storage](/docs/id/agent-sdk/session-storage): pertahankan transkrip di seluruh host dengan adaptor `SessionStore`.
* [Observability](/docs/id/agent-sdk/observability): ekspor jejak OTEL, metrik, dan log ke kolektor Anda.
* [Secure deployment](/docs/id/agent-sdk/secure-deployment): kontrol jaringan, manajemen kredensial, dan pengerasan isolasi.
* [Cost tracking](/docs/id/agent-sdk/cost-tracking): akuntansi token dan biaya per-sesi.
