> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Pemantauan

> Pelajari cara mengaktifkan dan mengonfigurasi OpenTelemetry untuk Claude Code.

Lacak penggunaan Claude Code, biaya, dan aktivitas alat di seluruh organisasi Anda dengan mengekspor data telemetri melalui OpenTelemetry (OTel). Claude Code mengekspor metrik sebagai data deret waktu melalui protokol metrik standar, acara melalui protokol log/acara, dan secara opsional distributed traces melalui [protokol traces](#traces-beta). Konfigurasikan backend metrik, log, dan traces Anda agar sesuai dengan persyaratan pemantauan Anda.

<h2 id="quick-start">
  Mulai cepat
</h2>

Konfigurasikan OpenTelemetry menggunakan variabel lingkungan:

```bash theme={null}
# 1. Aktifkan telemetri
export CLAUDE_CODE_ENABLE_TELEMETRY=1

# 2. Pilih pengekspor (keduanya bersifat opsional - konfigurasikan hanya yang Anda butuhkan)
export OTEL_METRICS_EXPORTER=otlp       # Opsi: otlp, prometheus, console, none
export OTEL_LOGS_EXPORTER=otlp          # Opsi: otlp, console, none

# 3. Konfigurasikan titik akhir OTLP (untuk pengekspor OTLP)
export OTEL_EXPORTER_OTLP_PROTOCOL=grpc
export OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4317

# 4. Atur autentikasi (jika diperlukan)
export OTEL_EXPORTER_OTLP_HEADERS="Authorization=Bearer your-token"

# 5. Untuk debugging: kurangi interval ekspor
export OTEL_METRIC_EXPORT_INTERVAL=10000  # 10 detik (default: 60000ms)
export OTEL_LOGS_EXPORT_INTERVAL=5000     # 5 detik (default: 5000ms)

# 6. Jalankan Claude Code
claude
```

<Note>
  Interval ekspor default adalah 60 detik untuk metrik dan 5 detik untuk log. Selama pengaturan, Anda mungkin ingin menggunakan interval yang lebih pendek untuk tujuan debugging. Ingat untuk mengatur ulang ini untuk penggunaan produksi.
</Note>

Untuk opsi konfigurasi lengkap, lihat [spesifikasi OpenTelemetry](https://github.com/open-telemetry/opentelemetry-specification/blob/main/specification/protocol/exporter.md#configuration-options).

<h2 id="administrator-configuration">
  Konfigurasi administrator
</h2>

Administrator dapat mengonfigurasi pengaturan OpenTelemetry untuk semua pengguna melalui [file pengaturan terkelola](/docs/id/settings#settings-files). Ini memungkinkan kontrol terpusat pengaturan telemetri di seluruh organisasi. Lihat [prioritas pengaturan](/docs/id/settings#settings-precedence) untuk informasi lebih lanjut tentang bagaimana pengaturan diterapkan.

Contoh konfigurasi pengaturan terkelola:

```json theme={null}
{
  "env": {
    "CLAUDE_CODE_ENABLE_TELEMETRY": "1",
    "OTEL_METRICS_EXPORTER": "otlp",
    "OTEL_LOGS_EXPORTER": "otlp",
    "OTEL_EXPORTER_OTLP_PROTOCOL": "grpc",
    "OTEL_EXPORTER_OTLP_ENDPOINT": "http://collector.example.com:4317",
    "OTEL_EXPORTER_OTLP_HEADERS": "Authorization=Bearer example-token"
  }
}
```

<Note>
  Pengaturan terkelola dapat didistribusikan melalui MDM (Mobile Device Management) atau solusi manajemen perangkat lainnya. Variabel lingkungan yang ditentukan dalam file pengaturan terkelola memiliki prioritas tinggi dan tidak dapat ditimpa oleh pengguna.
</Note>

Claude Code tidak meneruskan variabel lingkungan `OTEL_*` ke subproses yang dihasilkannya, termasuk alat Bash, hooks, server MCP, dan language servers. Aplikasi yang diinstrumentasi OpenTelemetry yang Anda jalankan melalui alat Bash tidak mewarisi titik akhir pengekspor atau header Claude Code, jadi atur variabel tersebut langsung dalam perintah jika aplikasi itu perlu mengekspor telemetrinya sendiri.

<h2 id="configuration-details">
  Detail konfigurasi
</h2>

<h3 id="common-configuration-variables">
  Variabel konfigurasi umum
</h3>

| Variabel Lingkungan                                 | Deskripsi                                                                                                                                                                                                                                                                                                                                                           | Nilai Contoh                                                                                                                           |
| --------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------- |
| `CLAUDE_CODE_ENABLE_TELEMETRY`                      | Mengaktifkan pengumpulan telemetri (diperlukan)                                                                                                                                                                                                                                                                                                                     | `1`                                                                                                                                    |
| `OTEL_METRICS_EXPORTER`                             | Jenis pengekspor metrik, dipisahkan koma. Gunakan `none` untuk menonaktifkan                                                                                                                                                                                                                                                                                        | `console`, `otlp`, `prometheus`, `none`                                                                                                |
| `OTEL_LOGS_EXPORTER`                                | Jenis pengekspor log/acara, dipisahkan koma. Gunakan `none` untuk menonaktifkan                                                                                                                                                                                                                                                                                     | `console`, `otlp`, `none`                                                                                                              |
| `OTEL_EXPORTER_OTLP_PROTOCOL`                       | Protokol untuk pengekspor OTLP, berlaku untuk semua sinyal                                                                                                                                                                                                                                                                                                          | `grpc`, `http/json`, `http/protobuf`                                                                                                   |
| `OTEL_EXPORTER_OTLP_ENDPOINT`                       | Titik akhir pengumpul OTLP untuk semua sinyal                                                                                                                                                                                                                                                                                                                       | `http://localhost:4317`                                                                                                                |
| `OTEL_EXPORTER_OTLP_METRICS_PROTOCOL`               | Protokol untuk metrik, menimpa pengaturan umum                                                                                                                                                                                                                                                                                                                      | `grpc`, `http/json`, `http/protobuf`                                                                                                   |
| `OTEL_EXPORTER_OTLP_METRICS_ENDPOINT`               | Titik akhir metrik OTLP, menimpa pengaturan umum                                                                                                                                                                                                                                                                                                                    | `http://localhost:4318/v1/metrics`                                                                                                     |
| `OTEL_EXPORTER_OTLP_LOGS_PROTOCOL`                  | Protokol untuk log, menimpa pengaturan umum                                                                                                                                                                                                                                                                                                                         | `grpc`, `http/json`, `http/protobuf`                                                                                                   |
| `OTEL_EXPORTER_OTLP_LOGS_ENDPOINT`                  | Titik akhir log OTLP, menimpa pengaturan umum                                                                                                                                                                                                                                                                                                                       | `http://localhost:4318/v1/logs`                                                                                                        |
| `OTEL_EXPORTER_OTLP_HEADERS`                        | Header autentikasi untuk OTLP                                                                                                                                                                                                                                                                                                                                       | `Authorization=Bearer token`                                                                                                           |
| `OTEL_METRIC_EXPORT_INTERVAL`                       | Interval ekspor dalam milidetik (default: 60000)                                                                                                                                                                                                                                                                                                                    | `5000`, `60000`                                                                                                                        |
| `OTEL_LOGS_EXPORT_INTERVAL`                         | Interval ekspor log dalam milidetik (default: 5000)                                                                                                                                                                                                                                                                                                                 | `1000`, `10000`                                                                                                                        |
| `OTEL_LOG_USER_PROMPTS`                             | Aktifkan pencatatan konten prompt pengguna (default: dinonaktifkan)                                                                                                                                                                                                                                                                                                 | `1` untuk mengaktifkan                                                                                                                 |
| `OTEL_LOG_ASSISTANT_RESPONSES`                      | Aktifkan pencatatan teks respons asisten pada acara `assistant_response` (default: dinonaktifkan). Saat tidak diatur, kembali ke nilai `OTEL_LOG_USER_PROMPTS`. Memerlukan Claude Code v2.1.193 atau lebih baru                                                                                                                                                     | `1` untuk mengaktifkan, `0` untuk tetap disunting                                                                                      |
| `OTEL_LOG_TOOL_DETAILS`                             | Aktifkan pencatatan parameter alat dan argumen input dalam acara alat dan atribut span trace: perintah Bash, nama server MCP dan alat, nama skill, nama workflow yang ditulis pengguna, dan input alat. Juga mengaktifkan nama perintah custom, plugin, dan MCP pada acara `user_prompt` (default: dinonaktifkan)                                                   | `1` untuk mengaktifkan                                                                                                                 |
| `OTEL_LOG_TOOL_CONTENT`                             | Aktifkan pencatatan konten input dan output alat dalam acara span (default: dinonaktifkan). Memerlukan [tracing](#traces-beta). Konten dipotong pada 60 KB                                                                                                                                                                                                          | `1` untuk mengaktifkan                                                                                                                 |
| `OTEL_LOG_RAW_API_BODIES`                           | Emit badan permintaan dan respons JSON API Anthropic Messages lengkap sebagai acara log `api_request_body` / `api_response_body` (default: dinonaktifkan). Badan mencakup seluruh riwayat percakapan. Mengaktifkan ini menyiratkan persetujuan untuk semua yang akan diungkapkan oleh `OTEL_LOG_USER_PROMPTS`, `OTEL_LOG_TOOL_DETAILS`, dan `OTEL_LOG_TOOL_CONTENT` | `1` untuk badan inline dipotong pada 60 KB, atau `file:<dir>` untuk badan tidak dipotong di disk dengan pointer `body_ref` dalam acara |
| `OTEL_EXPORTER_OTLP_METRICS_TEMPORALITY_PREFERENCE` | Preferensi temporalitas metrik (default: `delta`). Atur ke `cumulative` jika backend Anda mengharapkan temporalitas kumulatif                                                                                                                                                                                                                                       | `delta`, `cumulative`                                                                                                                  |
| `CLAUDE_CODE_OTEL_HEADERS_HELPER_DEBOUNCE_MS`       | Interval untuk menyegarkan header dinamis (default: 1740000ms / 29 menit)                                                                                                                                                                                                                                                                                           | `900000`                                                                                                                               |

<h3 id="mtls-authentication">
  Autentikasi mTLS
</h3>

Cara Anda mengonfigurasi sertifikat klien untuk pengekspor OTLP tergantung pada protokol OTLP yang digunakan untuk sinyal tersebut, diatur melalui `OTEL_EXPORTER_OTLP_PROTOCOL` atau override per-sinyal. Konfigurasi yang sama berlaku untuk metrik, log, dan traces.

| Protokol                     | Variabel sertifikat klien                                                                                                                                                                           | Percayai CA pengumpul dengan     |
| :--------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------- |
| `http/protobuf`, `http/json` | `CLAUDE_CODE_CLIENT_CERT`, `CLAUDE_CODE_CLIENT_KEY`, dan secara opsional `CLAUDE_CODE_CLIENT_KEY_PASSPHRASE`. Lihat [Konfigurasi jaringan](/docs/id/network-config#mtls-authentication)                  | `NODE_EXTRA_CA_CERTS`            |
| `grpc`                       | `OTEL_EXPORTER_OTLP_CLIENT_KEY` dan `OTEL_EXPORTER_OTLP_CLIENT_CERTIFICATE`, atau varian per-sinyal seperti `OTEL_EXPORTER_OTLP_METRICS_CLIENT_KEY` untuk menggunakan sertifikat berbeda per sinyal | `OTEL_EXPORTER_OTLP_CERTIFICATE` |

Untuk `grpc`, SDK OpenTelemetry membaca variabel OTLP standar secara langsung, jadi konfigurasi yang ada yang menetapkan variabel metrik per-sinyal terus berfungsi.

<h3 id="metrics-cardinality-control">
  Kontrol kardinalitas metrik
</h3>

Variabel lingkungan berikut mengontrol atribut mana yang disertakan dalam metrik untuk mengelola kardinalitas:

| Variabel Lingkungan                        | Deskripsi                                                                             | Nilai Default | Contoh untuk Menonaktifkan |
| ------------------------------------------ | ------------------------------------------------------------------------------------- | ------------- | -------------------------- |
| `OTEL_METRICS_INCLUDE_SESSION_ID`          | Sertakan atribut session.id dalam metrik                                              | `true`        | `false`                    |
| `OTEL_METRICS_INCLUDE_VERSION`             | Sertakan atribut app.version dalam metrik                                             | `false`       | `true`                     |
| `OTEL_METRICS_INCLUDE_ACCOUNT_UUID`        | Sertakan atribut user.account\_uuid dan user.account\_id dalam metrik                 | `true`        | `false`                    |
| `OTEL_METRICS_INCLUDE_ENTRYPOINT`          | Sertakan atribut app.entrypoint dalam metrik                                          | `false`       | `true`                     |
| `OTEL_METRICS_INCLUDE_RESOURCE_ATTRIBUTES` | Sertakan kunci dari `OTEL_RESOURCE_ATTRIBUTES` sebagai atribut pada titik data metrik | `true`        | `false`                    |

Variabel-variabel ini membantu mengontrol kardinalitas metrik, yang mempengaruhi persyaratan penyimpanan dan kinerja kueri di backend metrik Anda. Kardinalitas yang lebih rendah umumnya berarti kinerja yang lebih baik dan biaya penyimpanan yang lebih rendah tetapi data yang kurang granular untuk analisis.

<h3 id="traces-beta">
  Traces (beta)
</h3>

Distributed tracing mengekspor spans yang menghubungkan setiap prompt pengguna ke permintaan API dan eksekusi alat yang dipicunya, sehingga Anda dapat melihat permintaan lengkap sebagai satu trace di backend tracing Anda.

Tracing dimatikan secara default. Untuk mengaktifkannya, atur `CLAUDE_CODE_ENABLE_TELEMETRY=1` dan `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA=1`, kemudian atur `OTEL_TRACES_EXPORTER` untuk memilih tempat spans dikirim. Traces menggunakan kembali [konfigurasi OTLP umum](#common-configuration-variables) untuk titik akhir, protokol, header, dan [mTLS](#mtls-authentication).

| Variabel Lingkungan                   | Deskripsi                                                                          | Nilai Contoh                         |
| ------------------------------------- | ---------------------------------------------------------------------------------- | ------------------------------------ |
| `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA` | Aktifkan span tracing (diperlukan). `ENABLE_ENHANCED_TELEMETRY_BETA` juga diterima | `1`                                  |
| `OTEL_TRACES_EXPORTER`                | Jenis pengekspor traces, dipisahkan koma. Gunakan `none` untuk menonaktifkan       | `console`, `otlp`, `none`            |
| `OTEL_EXPORTER_OTLP_TRACES_PROTOCOL`  | Protokol untuk traces, menimpa `OTEL_EXPORTER_OTLP_PROTOCOL`                       | `grpc`, `http/json`, `http/protobuf` |
| `OTEL_EXPORTER_OTLP_TRACES_ENDPOINT`  | Titik akhir traces OTLP, menimpa `OTEL_EXPORTER_OTLP_ENDPOINT`                     | `http://localhost:4318/v1/traces`    |
| `OTEL_TRACES_EXPORT_INTERVAL`         | Interval ekspor batch span dalam milidetik (default: 5000)                         | `1000`, `10000`                      |

Spans menyunting teks prompt pengguna, detail input alat, dan konten alat secara default. Atur `OTEL_LOG_USER_PROMPTS=1`, `OTEL_LOG_TOOL_DETAILS=1`, dan `OTEL_LOG_TOOL_CONTENT=1` untuk menyertakannya.

Saat tracing aktif, subproses Bash dan PowerShell secara otomatis mewarisi variabel lingkungan `TRACEPARENT` yang berisi konteks trace W3C dari span eksekusi alat yang aktif. Ini memungkinkan subproses apa pun yang membaca `TRACEPARENT` untuk membuat parent spans-nya di bawah trace yang sama, memungkinkan distributed tracing end-to-end melalui skrip dan perintah yang dijalankan Claude.

Saat tracing aktif dan Claude Code terhubung langsung ke API Anthropic, setiap permintaan model membawa header W3C `traceparent` yang diatur ke konteks span `claude_code.llm_request`, dan header `traceresponse` API dicatat sebagai link span. Bersama-sama ini menghubungkan spans sisi klien Claude Code ke trace sisi server melalui perantara yang sesuai. Outbound HTTP MCP requests membawa `traceparent` dengan cara yang sama. Header tidak dikirim ke penyedia pihak ketiga.

Secara default, header `traceparent` pada permintaan model dan HTTP MCP dikirim hanya saat `ANTHROPIC_BASE_URL` tidak diatur atau menunjuk ke API Anthropic, karena beberapa proxy menolak header yang tidak dikenali. Variabel `TRACEPARENT` subproses dikendalikan oleh switch yang sama untuk konsistensi. Jika Anda menjalankan Claude Code melalui proxy `ANTHROPIC_BASE_URL` kustom dan ingin konteks trace dipropagasi, atur `CLAUDE_CODE_PROPAGATE_TRACEPARENT=1`.

Dalam sesi Agent SDK dan non-interaktif yang dimulai dengan `-p`, Claude Code juga membaca `TRACEPARENT` dan `TRACESTATE` dari lingkungannya sendiri saat memulai setiap span interaksi. Ini memungkinkan proses embedding untuk melewatkan konteks trace W3C aktifnya ke dalam subproses sehingga spans Claude Code muncul sebagai anak dari distributed trace pemanggil. Sesi interaktif mengabaikan `TRACEPARENT` inbound untuk menghindari secara tidak sengaja mewarisi nilai ambient dari lingkungan CI atau container.

<h4 id="span-hierarchy">
  Hierarki span
</h4>

Setiap prompt pengguna memulai span root `claude_code.interaction`. Panggilan API, panggilan alat, dan eksekusi hook dicatat sebagai anak-anaknya. Spans alat memiliki dua span anak mereka sendiri: satu untuk waktu yang dihabiskan menunggu keputusan izin dan satu untuk eksekusi itu sendiri. Ketika alat Agent atau alat Task legacy menghasilkan subagent, spans API dan alat subagent bersarang di bawah span `claude_code.tool` induk.

```text theme={null}
claude_code.interaction
├── claude_code.llm_request
├── claude_code.hook                    (memerlukan detailed beta tracing)
└── claude_code.tool
    ├── claude_code.tool.blocked_on_user
    ├── claude_code.tool.execution
    └── (Agent tool) subagent claude_code.llm_request / claude_code.tool spans
```

Dalam sesi Agent SDK dan `claude -p`, `claude_code.interaction` itu sendiri menjadi anak dari span pemanggil saat `TRACEPARENT` diatur dalam lingkungan.

<h4 id="span-attributes">
  Atribut span
</h4>

Setiap span membawa [atribut standar](#standard-attributes) ditambah atribut `span.type` yang cocok dengan namanya. Tabel di bawah mencantumkan atribut tambahan yang diatur pada setiap span. Spans `llm_request`, `tool.execution`, dan `hook` menetapkan status OpenTelemetry `ERROR` saat mereka mencatat kegagalan; span lainnya selalu berakhir dengan status `UNSET`.

**`claude_code.interaction`**

| Atribut                   | Deskripsi                                                  | Gated by                |
| ------------------------- | ---------------------------------------------------------- | ----------------------- |
| `user_prompt`             | Teks prompt. Nilai adalah `<REDACTED>` kecuali gate diatur | `OTEL_LOG_USER_PROMPTS` |
| `user_prompt_length`      | Panjang prompt dalam karakter                              |                         |
| `interaction.sequence`    | Penghitung berbasis 1 dari interaksi dalam sesi ini        |                         |
| `interaction.duration_ms` | Durasi wall-clock dari giliran                             |                         |

**`claude_code.llm_request`**

| Atribut                          | Deskripsi                                                                                                                                                              | Gated by                |
| -------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------- |
| `model`                          | Pengidentifikasi model                                                                                                                                                 |                         |
| `gen_ai.system`                  | Selalu `anthropic`. Konvensi semantik GenAI OpenTelemetry                                                                                                              |                         |
| `gen_ai.request.model`           | Nilai yang sama dengan `model`. Konvensi semantik GenAI OpenTelemetry                                                                                                  |                         |
| `query_source`                   | Subsistem yang mengeluarkan permintaan, seperti `repl_main_thread` atau nama subagent                                                                                  |                         |
| `agent_id`                       | Pengidentifikasi subagent atau rekan kerja yang mengeluarkan permintaan. Tidak ada pada sesi utama                                                                     |                         |
| `parent_agent_id`                | Pengidentifikasi agen yang menghasilkan yang ini. Tidak ada untuk sesi utama dan untuk agen yang dihasilkan langsung darinya                                           |                         |
| `workflow.run_id`                | Pengidentifikasi run dari run alat [Workflow](/docs/id/workflows) yang menghasilkan agen ini, dengan awalan `wf_`. Tidak ada untuk agen yang tidak dihasilkan oleh workflow |                         |
| `workflow.name`                  | Nama workflow yang menghasilkan agen ini. Nama yang ditulis pengguna diganti dengan `custom` kecuali gate diatur                                                       | `OTEL_LOG_TOOL_DETAILS` |
| `speed`                          | `fast` atau `normal`                                                                                                                                                   |                         |
| `llm_request.context`            | `interaction`, `tool`, atau `standalone` tergantung pada span induk                                                                                                    |                         |
| `duration_ms`                    | Durasi wall-clock termasuk retry                                                                                                                                       |                         |
| `ttft_ms`                        | Waktu ke token pertama dalam milidetik                                                                                                                                 |                         |
| `input_tokens`                   | Jumlah token input dari blok penggunaan API                                                                                                                            |                         |
| `output_tokens`                  | Jumlah token output                                                                                                                                                    |                         |
| `cache_read_tokens`              | Token yang dibaca dari prompt cache                                                                                                                                    |                         |
| `cache_creation_tokens`          | Token yang ditulis ke prompt cache                                                                                                                                     |                         |
| `request_id`                     | ID permintaan API Anthropic dari header respons `request-id`                                                                                                           |                         |
| `gen_ai.response.id`             | Nilai yang sama dengan `request_id`. Konvensi semantik GenAI OpenTelemetry                                                                                             |                         |
| `client_request_id`              | `x-client-request-id` yang dihasilkan klien dari upaya terakhir                                                                                                        |                         |
| `attempt`                        | Total upaya yang dilakukan untuk permintaan ini                                                                                                                        |                         |
| `success`                        | `true` atau `false`                                                                                                                                                    |                         |
| `status_code`                    | Kode status HTTP saat permintaan gagal                                                                                                                                 |                         |
| `error`                          | Pesan kesalahan saat permintaan gagal                                                                                                                                  |                         |
| `response.has_tool_call`         | `true` saat respons berisi blok tool-use                                                                                                                               |                         |
| `stop_reason`                    | API response `stop_reason`, seperti `end_turn`, `tool_use`, `max_tokens`, `stop_sequence`, `pause_turn`, atau `refusal`                                                |                         |
| `gen_ai.response.finish_reasons` | Nilai yang sama dengan `stop_reason`, dibungkus dalam array string. Konvensi semantik GenAI OpenTelemetry                                                              |                         |

Setiap upaya retry juga dicatat sebagai acara span `gen_ai.request.attempt` dengan atribut `attempt` dan `client_request_id`.

**`claude_code.tool`**

| Atribut               | Deskripsi                                                                                                                                                                                                                                              | Gated by                |
| --------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ----------------------- |
| `tool_name`           | Nama alat                                                                                                                                                                                                                                              |                         |
| `duration_ms`         | Durasi wall-clock termasuk tunggu izin dan eksekusi                                                                                                                                                                                                    |                         |
| `result_tokens`       | Ukuran token perkiraan dari hasil alat                                                                                                                                                                                                                 |                         |
| `agent_id`            | Pengidentifikasi subagent atau rekan kerja yang menjalankan alat. Tidak ada pada sesi utama                                                                                                                                                            |                         |
| `parent_agent_id`     | Pengidentifikasi agen yang menghasilkan yang ini. Tidak ada untuk sesi utama dan untuk agen yang dihasilkan langsung darinya                                                                                                                           |                         |
| `workflow.run_id`     | Pengidentifikasi run dari run alat Workflow yang menghasilkan agen ini, dengan awalan `wf_`. Tidak ada untuk agen yang tidak dihasilkan oleh workflow                                                                                                  |                         |
| `workflow.name`       | Nama workflow yang menghasilkan agen ini. Nama yang ditulis pengguna diganti dengan `custom` kecuali gate diatur                                                                                                                                       | `OTEL_LOG_TOOL_DETAILS` |
| `tool_use_id`         | ID blok `tool_use` model untuk panggilan ini. Cocok dengan `tool_use_id` pada acara [tool\_result](#tool-result-event) dan [tool\_decision](#tool-decision-event) serta dalam payload hook, sehingga Anda dapat menggabungkan span ke catatan tersebut |                         |
| `gen_ai.tool.call.id` | Nilai yang sama dengan `tool_use_id`. Konvensi semantik GenAI OpenTelemetry                                                                                                                                                                            |                         |
| `file_path`           | Jalur file target untuk alat Read, Edit, dan Write                                                                                                                                                                                                     | `OTEL_LOG_TOOL_DETAILS` |
| `full_command`        | String perintah untuk alat Bash                                                                                                                                                                                                                        | `OTEL_LOG_TOOL_DETAILS` |
| `skill_name`          | Nama skill untuk alat Skill                                                                                                                                                                                                                            | `OTEL_LOG_TOOL_DETAILS` |
| `subagent_type`       | Jenis subagent untuk alat Agent atau alat Task legacy                                                                                                                                                                                                  | `OTEL_LOG_TOOL_DETAILS` |

Saat `OTEL_LOG_TOOL_CONTENT=1`, span ini juga mencatat acara span `tool.output` yang atributnya berisi badan input dan output alat, dipotong pada 60 KB per atribut.

**`claude_code.tool.blocked_on_user`**

| Atribut       | Deskripsi                                                                        | Gated by |
| ------------- | -------------------------------------------------------------------------------- | -------- |
| `duration_ms` | Waktu yang dihabiskan menunggu keputusan izin                                    |          |
| `decision`    | `accept` atau `reject`                                                           |          |
| `source`      | Sumber keputusan, cocok dengan acara [Tool decision event](#tool-decision-event) |          |

**`claude_code.tool.execution`**

| Atribut               | Deskripsi                                                                                                                                                 | Gated by                |
| --------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------- |
| `duration_ms`         | Waktu yang dihabiskan menjalankan badan alat                                                                                                              |                         |
| `tool_use_id`         | Nilai yang sama dengan span `claude_code.tool` induk                                                                                                      |                         |
| `gen_ai.tool.call.id` | Nilai yang sama dengan `tool_use_id`. Konvensi semantik GenAI OpenTelemetry                                                                               |                         |
| `success`             | `true` atau `false`                                                                                                                                       |                         |
| `error`               | String kategori kesalahan saat eksekusi gagal, seperti `Error:ENOENT` atau `ShellError`. Berisi pesan kesalahan lengkap sebagai gantinya saat gate diatur | `OTEL_LOG_TOOL_DETAILS` |

**`claude_code.hook`**

Span ini dipancarkan hanya saat detailed beta tracing aktif, yang memerlukan `ENABLE_BETA_TRACING_DETAILED=1` dan `BETA_TRACING_ENDPOINT` selain konfigurasi pengekspor trace di atas. Dalam sesi CLI interaktif, ini juga memerlukan organisasi Anda untuk berada dalam daftar putih untuk fitur ini. Sesi Agent SDK dan non-interaktif `-p` tidak gated. Ini tidak dipancarkan saat hanya `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA` yang diatur.

| Atribut                  | Deskripsi                                         | Gated by                |
| ------------------------ | ------------------------------------------------- | ----------------------- |
| `hook_event`             | Jenis acara hook, seperti `PreToolUse`            |                         |
| `hook_name`              | Nama hook lengkap, seperti `PreToolUse:Write`     |                         |
| `num_hooks`              | Jumlah perintah hook yang cocok dieksekusi        |                         |
| `hook_definitions`       | Konfigurasi hook yang diserialisasi JSON          | `OTEL_LOG_TOOL_DETAILS` |
| `duration_ms`            | Durasi wall-clock dari semua hook yang cocok      |                         |
| `num_success`            | Jumlah hook yang selesai dengan sukses            |                         |
| `num_blocking`           | Jumlah hook yang mengembalikan keputusan blocking |                         |
| `num_non_blocking_error` | Jumlah hook yang gagal tanpa blocking             |                         |
| `num_cancelled`          | Jumlah hook yang dibatalkan sebelum selesai       |                         |

<Note>
  Atribut tambahan yang mengandung konten seperti `new_context`, `system_prompt_preview`, `user_system_prompt`, `tool_input`, dan `response.model_output` dipancarkan hanya saat detailed beta tracing aktif. Mereka bukan bagian dari skema span yang stabil. `user_system_prompt` juga memerlukan `OTEL_LOG_USER_PROMPTS=1`. Ini membawa hanya teks prompt sistem yang Anda berikan melalui opsi SDK `systemPrompt` atau flag `--system-prompt` dan `--append-system-prompt`, dipotong pada 60 KB, dan dipancarkan sekali per sesi daripada per permintaan.
</Note>

<h3 id="dynamic-headers">
  Header dinamis
</h3>

Untuk lingkungan perusahaan yang memerlukan autentikasi dinamis, Anda dapat mengonfigurasi skrip untuk menghasilkan header secara dinamis. Header dinamis hanya berlaku untuk protokol `http/protobuf` dan `http/json`. Pengekspor `grpc` hanya menggunakan nilai statis `OTEL_EXPORTER_OTLP_HEADERS`.

<h4 id="settings-configuration">
  Konfigurasi pengaturan
</h4>

Tambahkan ke `.claude/settings.json` Anda:

```json theme={null}
{
  "otelHeadersHelper": "/bin/generate_opentelemetry_headers.sh"
}
```

Nilai dapat berupa jalur ke file yang dapat dieksekusi, termasuk jalur yang berisi spasi, atau baris perintah shell dengan argumen. Di Windows, nilai selalu berjalan melalui shell, jadi kutip jalur yang berisi spasi di dalam nilai JSON.

<h4 id="script-requirements">
  Persyaratan skrip
</h4>

Skrip harus menampilkan JSON yang valid dengan pasangan kunci-nilai string yang mewakili header HTTP:

```bash theme={null}
#!/bin/bash
# Contoh: Header ganda
echo "{\"Authorization\": \"Bearer $(get-token.sh)\", \"X-API-Key\": \"$(get-api-key.sh)\"}"
```

Jika pembantu gagal atau mencetak output yang tidak memenuhi persyaratan ini, Claude Code melaporkan kesalahan dalam:

* Output `/status`
* Log debug, saat berjalan dengan [`--debug`](/docs/id/cli-reference#cli-flags) atau setelah menjalankan `/debug` dalam sesi
* stderr, dalam sesi non-interaktif yang dimulai dengan `-p`

<h4 id="refresh-behavior">
  Perilaku penyegaran
</h4>

Skrip pembantu header berjalan saat startup dan secara berkala setelahnya untuk mendukung penyegaran token. Secara default, skrip berjalan setiap 29 menit. Sesuaikan interval dengan variabel lingkungan `CLAUDE_CODE_OTEL_HEADERS_HELPER_DEBOUNCE_MS`.

<h3 id="multi-team-organization-support">
  Dukungan organisasi multi-tim
</h3>

Organisasi dengan beberapa tim atau departemen dapat menambahkan atribut khusus untuk membedakan antara kelompok yang berbeda menggunakan variabel lingkungan `OTEL_RESOURCE_ATTRIBUTES`:

```bash theme={null}
# Tambahkan atribut khusus untuk identifikasi tim
export OTEL_RESOURCE_ATTRIBUTES="department=engineering,team.id=platform,cost_center=eng-123"
```

Atribut khusus ini akan disertakan dalam semua metrik dan acara, memungkinkan Anda untuk:

* Filter metrik berdasarkan tim atau departemen
* Lacak biaya per pusat biaya
* Buat dasbor khusus tim
* Atur peringatan untuk tim tertentu

Claude Code melampirkan nilai-nilai ini sebagai atribut pada setiap titik data metrik dan catatan acara, selain mengirimkannya dalam blok sumber daya OTLP. Karena sebagian besar backend metrik mengekspos atribut titik data sebagai label yang dapat dikueri, Anda dapat mengelompokkan dan memfilter metrik berdasarkan kunci khusus Anda secara langsung. Kunci khusus tidak pernah menimpa [atribut standar](#standard-attributes) seperti `user.id` atau `session.id`: saat kunci bertabrakan, Claude Code mempertahankan nilai built-in.

Setiap kunci khusus menjadi label pada setiap deret metrik, jadi nilai kardinalitas tinggi meningkatkan biaya penyimpanan di backend metrik Anda. Untuk mengirim atribut khusus dalam blok sumber daya saja dan menghilangkannya dari label titik data, atur `OTEL_METRICS_INCLUDE_RESOURCE_ATTRIBUTES=false`. Lihat [Kontrol kardinalitas metrik](#metrics-cardinality-control).

<Warning>
  Variabel lingkungan `OTEL_RESOURCE_ATTRIBUTES` menggunakan pasangan kunci=nilai yang dipisahkan koma dengan persyaratan pemformatan yang ketat:

  * **Tidak ada spasi yang diizinkan**: nilai tidak dapat berisi spasi. Misalnya, `user.organizationName=My Company` tidak valid
  * **Format**: harus berupa pasangan kunci=nilai yang dipisahkan koma: `key1=value1,key2=value2`
  * **Karakter yang diizinkan**: hanya karakter US-ASCII yang tidak termasuk karakter kontrol, spasi, tanda kutip ganda, koma, titik koma, dan garis miring terbalik
  * **Karakter khusus**: karakter di luar rentang yang diizinkan harus dikodekan persen

  Untuk nilai yang memerlukan spasi, gunakan garis bawah atau camelCase sebagai gantinya. Contoh berikut menetapkan `org.name` dengan setiap bentuk:

  ```bash theme={null}
  export OTEL_RESOURCE_ATTRIBUTES="org.name=Johns_Organization"
  export OTEL_RESOURCE_ATTRIBUTES="org.name=JohnsOrganization"
  ```

  Anda dapat mengkodekan persen karakter apa pun, bukan hanya yang dikecualikan. Contoh ini mengkodekan spasi dan apostrof:

  ```bash theme={null}
  export OTEL_RESOURCE_ATTRIBUTES="org.name=John%27s%20Organization"
  ```

  Membungkus nilai dalam tanda kutip tidak menghindari spasi. Misalnya, `org.name="My Company"` menghasilkan nilai literal `"My Company"` dengan tanda kutip disertakan, bukan `My Company`.
</Warning>

<h3 id="example-configurations">
  Konfigurasi contoh
</h3>

Atur variabel lingkungan ini sebelum menjalankan `claude`. Setiap blok menunjukkan konfigurasi lengkap untuk pengekspor atau skenario penerapan yang berbeda:

```bash theme={null}
# Debugging konsol (interval 1 detik)
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_METRICS_EXPORTER=console
export OTEL_METRIC_EXPORT_INTERVAL=1000

# OTLP/gRPC
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_METRICS_EXPORTER=otlp
export OTEL_EXPORTER_OTLP_PROTOCOL=grpc
export OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4317

# Prometheus
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_METRICS_EXPORTER=prometheus

# Pengekspor ganda
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_METRICS_EXPORTER=console,otlp
export OTEL_EXPORTER_OTLP_PROTOCOL=http/json

# Titik akhir/backend berbeda untuk metrik dan log
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_METRICS_EXPORTER=otlp
export OTEL_LOGS_EXPORTER=otlp
export OTEL_EXPORTER_OTLP_METRICS_PROTOCOL=http/protobuf
export OTEL_EXPORTER_OTLP_METRICS_ENDPOINT=http://metrics.example.com:4318
export OTEL_EXPORTER_OTLP_LOGS_PROTOCOL=grpc
export OTEL_EXPORTER_OTLP_LOGS_ENDPOINT=http://logs.example.com:4317

# Hanya metrik (tanpa acara/log)
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_METRICS_EXPORTER=otlp
export OTEL_EXPORTER_OTLP_PROTOCOL=grpc
export OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4317

# Hanya acara/log (tanpa metrik)
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_LOGS_EXPORTER=otlp
export OTEL_EXPORTER_OTLP_PROTOCOL=grpc
export OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4317
```

<h2 id="available-metrics-and-events">
  Metrik dan acara yang tersedia
</h2>

<h3 id="standard-attributes">
  Atribut standar
</h3>

Semua metrik dan acara berbagi atribut standar ini:

| Atribut                               | Deskripsi                                                                                                                                                                                                                                             | Dikendalikan Oleh                                          |
| ------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------- |
| `session.id`                          | Pengidentifikasi sesi unik                                                                                                                                                                                                                            | `OTEL_METRICS_INCLUDE_SESSION_ID` (default: true)          |
| `app.version`                         | Versi Claude Code saat ini                                                                                                                                                                                                                            | `OTEL_METRICS_INCLUDE_VERSION` (default: false)            |
| `app.entrypoint`                      | Bagaimana sesi diluncurkan, seperti `cli`, `sdk-cli`, `sdk-ts`, `sdk-py`, atau `claude-vscode`                                                                                                                                                        | `OTEL_METRICS_INCLUDE_ENTRYPOINT` (default: false)         |
| `organization.id`                     | UUID organisasi (saat diautentikasi)                                                                                                                                                                                                                  | Selalu disertakan saat tersedia                            |
| `user.account_uuid`                   | UUID akun (saat diautentikasi)                                                                                                                                                                                                                        | `OTEL_METRICS_INCLUDE_ACCOUNT_UUID` (default: true)        |
| `user.account_id`                     | ID akun dalam format yang ditandai sesuai dengan API admin Anthropic (saat diautentikasi), seperti `user_01BWBeN28...`                                                                                                                                | `OTEL_METRICS_INCLUDE_ACCOUNT_UUID` (default: true)        |
| `user.id`                             | Pengidentifikasi anonim acak yang dihasilkan pada run pertama dan disimpan di `~/.claude.json`. Tidak mengandung informasi pribadi dan tidak berasal dari akun Claude Anda. Menghapus file menghasilkan nilai yang tidak terkait pada run berikutnya. | Selalu disertakan                                          |
| `user.email`                          | Alamat email pengguna (saat diautentikasi melalui OAuth)                                                                                                                                                                                              | Selalu disertakan saat tersedia                            |
| `terminal.type`                       | Jenis terminal, seperti `iTerm.app`, `vscode`, `cursor`, atau `tmux`                                                                                                                                                                                  | Selalu disertakan saat terdeteksi                          |
| Kunci dari `OTEL_RESOURCE_ATTRIBUTES` | Atribut khusus yang Anda atur, seperti `department` atau `team.id`. Lihat [Dukungan organisasi multi-tim](#multi-team-organization-support)                                                                                                           | `OTEL_METRICS_INCLUDE_RESOURCE_ATTRIBUTES` (default: true) |

Saat Claude Code masuk ke [gateway aplikasi Claude](/docs/id/claude-apps-gateway), CLI memberi stempel ekspor dengan identitas yang diautentikasi dari sesi gateway: `user.id` adalah subjek IdP daripada pengidentifikasi instalasi anonim, `user.email` adalah email yang masuk, dan `user.groups` membawa keanggotaan grup IdP sebagai string yang dipisahkan koma. Setiap ekspor juga membawa `identity.source: gateway-oidc`. Identitas gateway diterapkan terakhir, jadi kunci `user.*` dan `identity.*` yang diatur melalui `OTEL_RESOURCE_ATTRIBUTES` diabaikan pada sesi gateway.

Acara juga menyertakan atribut berikut. Ini tidak pernah dilampirkan pada metrik karena akan menyebabkan kardinalitas tak terbatas:

* `prompt.id`: UUID yang menghubungkan prompt pengguna dengan semua acara berikutnya hingga prompt berikutnya. Lihat [Atribut korelasi acara](#event-correlation-attributes).
* `workspace.host_paths`: direktori ruang kerja host yang dipilih di aplikasi desktop, sebagai array string
* `workflow.run_id`: pengidentifikasi run, dengan awalan `wf_`, pada acara API dan alat yang dipancarkan oleh agen yang termasuk dalam run alat [Workflow](/docs/id/workflows). Memfilter acara berdasarkan satu `workflow.run_id` merekonstruksi permintaan API dan hasil alat run tersebut. Pengidentifikasi mencakup agen yang dihasilkan skrip workflow dan agen apa pun yang dihasilkan agen tersebut pada gilirannya, seperti invokasi skill. Ini cocok dengan pengidentifikasi run yang dilaporkan dalam hasil alat Workflow. Tidak ada pada semua acara lainnya. Memerlukan Claude Code v2.1.202 atau lebih baru
* `workflow.name`: nama workflow, `meta.name` skrip-nya, dipancarkan bersama `workflow.run_id`. Nama workflow built-in muncul verbatim saat run mengeksekusi skrip built-in yang tidak dimodifikasi. Nama yang ditulis pengguna, termasuk salinan yang diedit dari skrip built-in, diganti dengan `custom` kecuali `OTEL_LOG_TOOL_DETAILS=1` diatur. Memerlukan Claude Code v2.1.202 atau lebih baru

<h3 id="metrics">
  Metrik
</h3>

Claude Code mengekspor metrik berikut:

| Nama Metrik                           | Deskripsi                                  | Unit   |
| ------------------------------------- | ------------------------------------------ | ------ |
| `claude_code.session.count`           | Jumlah sesi CLI yang dimulai               | count  |
| `claude_code.lines_of_code.count`     | Jumlah baris kode yang dimodifikasi        | count  |
| `claude_code.pull_request.count`      | Jumlah permintaan tarik yang dibuat        | count  |
| `claude_code.commit.count`            | Jumlah komit git yang dibuat               | count  |
| `claude_code.cost.usage`              | Biaya sesi Claude Code                     | USD    |
| `claude_code.token.usage`             | Jumlah token yang digunakan                | tokens |
| `claude_code.code_edit_tool.decision` | Jumlah keputusan izin alat pengeditan kode | count  |
| `claude_code.active_time.total`       | Total waktu aktif dalam detik              | s      |

<h3 id="metric-details">
  Detail metrik
</h3>

Setiap metrik mencakup atribut standar yang tercantum di atas. Metrik dengan atribut khusus konteks tambahan dicatat di bawah ini.

<h4 id="session-counter">
  Penghitung sesi
</h4>

Ditingkatkan pada awal setiap sesi.

**Atribut**:

* Semua [atribut standar](#standard-attributes)
* `start_type`: Bagaimana sesi dimulai. Salah satu dari `"fresh"`, `"resume"`, `"continue"`, atau `"agents_view"`. Nilai `"agents_view"` mengidentifikasi proses dashboard `claude agents`, antarmuka lokal yang diluncurkan pengguna daripada sesi percakapan. Filter pada nilai ini untuk memisahkan peluncuran proses UI dari sesi percakapan di dasbor Anda.

<h4 id="lines-of-code-counter">
  Penghitung baris kode
</h4>

Ditingkatkan saat kode ditambahkan atau dihapus.

**Atribut**:

* Semua [atribut standar](#standard-attributes)
* `type`: (`"added"`, `"removed"`)
* `model`: Pengidentifikasi model untuk model yang membuat perubahan (misalnya, "claude-sonnet-5")

<h4 id="pull-request-counter">
  Penghitung permintaan tarik
</h4>

Ditingkatkan saat Claude Code membuat permintaan tarik atau merge request melalui perintah shell atau alat MCP.

**Atribut**:

* Semua [atribut standar](#standard-attributes)

<h4 id="commit-counter">
  Penghitung komit
</h4>

Ditingkatkan saat membuat komit git melalui Claude Code.

**Atribut**:

* Semua [atribut standar](#standard-attributes)

<h4 id="cost-counter">
  Penghitung biaya
</h4>

Ditingkatkan setelah setiap permintaan API.

**Atribut**:

* Semua [atribut standar](#standard-attributes)
* `model`: Pengidentifikasi model (misalnya, "claude-sonnet-5")
* `query_source`: Kategori subsistem yang mengeluarkan permintaan. Salah satu dari `"main"`, `"subagent"`, atau `"auxiliary"`
* `speed`: `"fast"` saat permintaan menggunakan mode cepat. Tidak ada sebaliknya
* `effort`: [Tingkat effort](/docs/id/model-config#adjust-effort-level) yang diterapkan pada permintaan: `"low"`, `"medium"`, `"high"`, `"xhigh"`, atau `"max"`. Tidak ada saat model tidak mendukung effort.
* `agent.name`: Jenis subagent yang mengeluarkan permintaan. Nama agen built-in dan agen dari plugin marketplace resmi muncul verbatim. Nama agen yang ditentukan pengguna lainnya diganti dengan `"custom"`. Tidak ada saat permintaan tidak dikeluarkan oleh jenis subagent bernama.
* `skill.name`: Skill aktif untuk permintaan, diatur oleh alat Skill, perintah `/`, atau diwarisi oleh subagent yang dihasilkan. Nama skill built-in, bundled, yang ditentukan pengguna, dan plugin marketplace resmi muncul verbatim. Nama skill plugin pihak ketiga diganti dengan `"third-party"`. Tidak ada saat tidak ada skill yang aktif.
* `plugin.name`: Plugin pemilik saat skill atau subagent aktif disediakan oleh plugin. Nama plugin marketplace resmi muncul verbatim. Nama plugin pihak ketiga diganti dengan `"third-party"`. Tidak ada saat skill atau subagent tidak memiliki plugin pemilik.
* `marketplace.name`: Marketplace tempat plugin pemilik diinstal. Hanya dipancarkan untuk plugin marketplace resmi. Tidak ada sebaliknya.
* `mcp_server.name`: Server MCP yang alatnya berjalan dalam giliran yang menghasilkan permintaan ini. Nama server built-in, claude.ai-proxied, dan official-registry muncul verbatim. Nama server yang dikonfigurasi pengguna diganti dengan `"custom"`. Tidak ada saat tidak ada alat MCP yang berjalan.
* `mcp_tool.name`: Alat MCP yang berjalan dalam giliran yang menghasilkan permintaan ini, dengan redaksi yang sama dengan `mcp_server.name`. Tidak ada saat tidak ada alat MCP yang berjalan.

<h4 id="token-counter">
  Penghitung token
</h4>

Ditingkatkan setelah setiap permintaan API.

**Atribut**:

* Semua [atribut standar](#standard-attributes)
* `type`: (`"input"`, `"output"`, `"cacheRead"`, `"cacheCreation"`)
* `model`: Pengidentifikasi model (misalnya, "claude-sonnet-5")
* `query_source`: Kategori subsistem yang mengeluarkan permintaan. Salah satu dari `"main"`, `"subagent"`, atau `"auxiliary"`
* `speed`: `"fast"` saat permintaan menggunakan mode cepat. Tidak ada sebaliknya
* `effort`: [Tingkat effort](/docs/id/model-config#adjust-effort-level) yang diterapkan pada permintaan. Lihat [Penghitung biaya](#cost-counter) untuk detail.
* `agent.name`, `skill.name`, `plugin.name`, `marketplace.name`, `mcp_server.name`, `mcp_tool.name`: Atribusi skill, plugin, agen, dan MCP untuk permintaan. Lihat [Penghitung biaya](#cost-counter) untuk definisi dan perilaku redaksi.

<h4 id="code-edit-tool-decision-counter">
  Penghitung keputusan alat pengeditan kode
</h4>

Ditingkatkan saat pengguna menerima atau menolak penggunaan alat Edit, Write, atau NotebookEdit.

**Atribut**:

* Semua [atribut standar](#standard-attributes)
* `tool_name`: Nama alat (`"Edit"`, `"Write"`, `"NotebookEdit"`)
* `decision`: Keputusan pengguna (`"accept"`, `"reject"`)
* `source`: Sumber keputusan. Salah satu dari `"config"`, `"hook"`, `"user_permanent"`, `"user_temporary"`, `"user_abort"`, atau `"user_reject"`. Lihat [Acara keputusan alat](#tool-decision-event) untuk mengetahui apa arti setiap nilai.
* `language`: Bahasa pemrograman file yang diedit, seperti `"TypeScript"`, `"Python"`, `"JavaScript"`, atau `"Markdown"`. Mengembalikan `"unknown"` untuk ekstensi file yang tidak dikenali.

<h4 id="active-time-counter">
  Penghitung waktu aktif
</h4>

Melacak waktu aktual yang dihabiskan secara aktif menggunakan Claude Code, tidak termasuk waktu idle. Metrik ini ditingkatkan selama interaksi pengguna, seperti mengetik dan membaca respons, dan selama pemrosesan CLI, seperti eksekusi alat dan pembuatan respons AI.

**Atribut**:

* Semua [atribut standar](#standard-attributes)
* `type`: `"user"` untuk interaksi keyboard, `"cli"` untuk eksekusi alat dan respons AI

<h3 id="events">
  Acara
</h3>

Claude Code mengekspor acara berikut melalui log/acara OpenTelemetry (saat `OTEL_LOGS_EXPORTER` dikonfigurasi):

<h4 id="event-correlation-attributes">
  Atribut korelasi acara
</h4>

Saat pengguna mengirimkan prompt, Claude Code dapat membuat beberapa panggilan API dan menjalankan beberapa alat. Atribut `prompt.id` memungkinkan Anda menghubungkan semua acara tersebut kembali ke prompt tunggal yang memicunya.

| Atribut     | Deskripsi                                                                                                      |
| ----------- | -------------------------------------------------------------------------------------------------------------- |
| `prompt.id` | Pengidentifikasi UUID v4 yang menghubungkan semua acara yang dihasilkan saat memproses prompt pengguna tunggal |

Untuk melacak semua aktivitas yang dipicu oleh prompt tunggal, filter acara Anda berdasarkan nilai `prompt.id` tertentu. Ini mengembalikan acara user\_prompt, acara api\_request apa pun, dan acara tool\_result apa pun yang terjadi saat memproses prompt tersebut.

<Note>
  `prompt.id` sengaja dikecualikan dari metrik karena setiap prompt menghasilkan ID unik, yang akan membuat jumlah deret waktu terus bertambah. Gunakan untuk analisis tingkat acara dan jejak audit saja.
</Note>

<h4 id="user-prompt-event">
  Acara prompt pengguna
</h4>

Dicatat saat pengguna mengirimkan prompt.

**Nama Acara**: `claude_code.user_prompt`

**Atribut**:

* Semua [atribut standar](#standard-attributes)
* `event.name`: `"user_prompt"`
* `event.timestamp`: Stempel waktu ISO 8601
* `event.sequence`: penghitung yang meningkat secara monoton untuk mengurutkan acara dalam sesi
* `prompt_length`: Panjang prompt
* `prompt`: Konten prompt. Diredaksi secara default. Atur `OTEL_LOG_USER_PROMPTS=1` untuk menyertakannya
* `command_name`: Nama perintah saat prompt memanggil satu. Nama perintah built-in dan bundled seperti `compact` atau `debug` dipancarkan apa adanya; alias seperti `reset` dipancarkan sebagai yang diketik daripada nama kanonik. Nama perintah custom, plugin, dan MCP runtuh menjadi `custom` atau `mcp` kecuali `OTEL_LOG_TOOL_DETAILS=1` diatur
* `command_source`: Asal perintah saat ada: `builtin`, `custom`, atau `mcp`. Perintah yang disediakan plugin melaporkan sebagai `custom`

<h4 id="assistant-response-event">
  Acara respons asisten
</h4>

Dicatat setelah setiap permintaan API yang mengembalikan konten teks dari model. Hanya blok teks respons yang disertakan; blok thinking dan blok tool-use dikecualikan. Memerlukan Claude Code v2.1.193 atau lebih baru.

**Nama Acara**: `claude_code.assistant_response`

**Atribut**:

* Semua [atribut standar](#standard-attributes)
* `event.name`: `"assistant_response"`
* `event.timestamp`: Stempel waktu ISO 8601
* `event.sequence`: penghitung yang meningkat secara monoton untuk mengurutkan acara dalam sesi
* `response_length`: Panjang teks respons dalam karakter
* `response`: Teks respons, dipotong pada 60 KB. Diredaksi menjadi `<REDACTED>` secara default. Atur `OTEL_LOG_ASSISTANT_RESPONSES=1` untuk menyertakannya. Saat `OTEL_LOG_ASSISTANT_RESPONSES` tidak diatur, `OTEL_LOG_USER_PROMPTS` mengontrolnya sebagai gantinya, jadi atur `OTEL_LOG_ASSISTANT_RESPONSES=0` untuk menjaga respons diredaksi saat logging prompt aktif
* `model`: Pengidentifikasi model (misalnya, "claude-sonnet-5")
* `request_id`: ID permintaan API Anthropic dari header `request-id` respons. Hadir hanya saat API mengembalikan satu
* `query_source`: Subsistem yang mengeluarkan permintaan, seperti `"repl_main_thread"`, `"compact"`, atau nama subagent

<h4 id="tool-result-event">
  Acara hasil alat
</h4>

Dicatat saat alat menyelesaikan eksekusi. Tidak dipancarkan jika panggilan alat ditolak; lihat [Acara keputusan alat](#tool-decision-event) untuk penolakan.

**Nama Acara**: `claude_code.tool_result`

**Atribut**:

* Semua [atribut standar](#standard-attributes)
* `event.name`: `"tool_result"`
* `event.timestamp`: Stempel waktu ISO 8601
* `event.sequence`: penghitung yang meningkat secara monoton untuk mengurutkan acara dalam sesi
* `tool_name`: Nama alat
* `tool_use_id`: Pengidentifikasi unik untuk invokasi alat ini. Cocok dengan `tool_use_id` yang diteruskan ke hooks, memungkinkan korelasi antara acara OTel dan data yang ditangkap hook.
* `success`: `"true"` atau `"false"`
* `duration_ms`: Waktu eksekusi dalam milidetik
* `error_type`: String kategori kesalahan saat alat gagal, seperti `"Error:ENOENT"` atau `"ShellError"`
* `error` (saat `OTEL_LOG_TOOL_DETAILS=1`): Pesan kesalahan lengkap saat alat gagal
* `decision_type`: Selalu `"accept"`, karena acara ini hanya dipancarkan setelah alat berjalan. Panggilan yang ditolak tidak menghasilkan hasil alat
* `decision_source`: Sumber keputusan izin. Salah satu dari `"config"`, `"hook"`, `"user_permanent"`, atau `"user_temporary"`. Lihat [Acara keputusan alat](#tool-decision-event) untuk mengetahui apa arti setiap nilai. Sumber hanya-tolak `"user_abort"` dan `"user_reject"` tidak pernah muncul pada acara ini.
* `tool_input_size_bytes`: Ukuran input alat yang diserialisasi JSON dalam byte
* `tool_result_size_bytes`: Ukuran hasil alat dalam byte
* `mcp_server_scope`: Pengidentifikasi cakupan server MCP (untuk alat MCP)
* `tool_parameters` (saat `OTEL_LOG_TOOL_DETAILS=1`): String JSON yang berisi parameter khusus alat:
  * Untuk alat Bash: mencakup `bash_command`, `full_command`, `timeout`, `description`, `dangerouslyDisableSandbox`, dan `git_commit_id` (SHA komit, saat perintah `git commit` berhasil)
  * Untuk alat WorkspaceBash: mencakup `bash_command`, `full_command`, `timeout`
  * Untuk alat MCP: mencakup `mcp_server_name`, `mcp_tool_name`
  * Untuk alat Skill: mencakup `skill_name`
  * Untuk alat Agent atau alat Task legacy: mencakup `subagent_type`
* `tool_input` (saat `OTEL_LOG_TOOL_DETAILS=1`): Argumen alat yang diserialisasi JSON. Nilai individual di atas 512 karakter dipotong, dan muatan penuh dibatasi hingga \~4 K karakter. Berlaku untuk semua alat termasuk alat MCP.

<h4 id="api-request-event">
  Acara permintaan API
</h4>

Dicatat untuk setiap permintaan API ke Claude.

**Nama Acara**: `claude_code.api_request`

**Atribut**:

* Semua [atribut standar](#standard-attributes)
* `event.name`: `"api_request"`
* `event.timestamp`: Stempel waktu ISO 8601
* `event.sequence`: penghitung yang meningkat secara monoton untuk mengurutkan acara dalam sesi
* `model`: Model yang digunakan (misalnya, "claude-sonnet-5")
* `cost_usd`: Biaya perkiraan dalam USD
* `duration_ms`: Durasi permintaan dalam milidetik
* `input_tokens`: Jumlah token input
* `output_tokens`: Jumlah token output
* `cache_read_tokens`: Jumlah token yang dibaca dari cache
* `cache_creation_tokens`: Jumlah token yang digunakan untuk pembuatan cache
* `request_id`: ID permintaan API Anthropic dari header `request-id` respons, seperti `"req_011..."`. Hadir hanya saat API mengembalikan satu.
* `speed`: `"fast"` atau `"normal"`, menunjukkan apakah mode cepat aktif
* `query_source`: Subsistem yang mengeluarkan permintaan, seperti `"repl_main_thread"`, `"compact"`, atau nama subagent
* `effort`: [Tingkat effort](/docs/id/model-config#adjust-effort-level) yang diterapkan pada permintaan: `"low"`, `"medium"`, `"high"`, `"xhigh"`, atau `"max"`. Tidak ada saat model tidak mendukung effort.
* `agent.name`, `skill.name`, `plugin.name`, `marketplace.name`, `mcp_server.name`, `mcp_tool.name`: Atribusi skill, plugin, agen, dan MCP untuk permintaan. Lihat [Penghitung biaya](#cost-counter) untuk definisi dan perilaku redaksi.

<h4 id="api-error-event">
  Acara kesalahan API
</h4>

Dicatat saat permintaan API ke Claude gagal.

**Nama Acara**: `claude_code.api_error`

**Atribut**:

* Semua [atribut standar](#standard-attributes)
* `event.name`: `"api_error"`
* `event.timestamp`: Stempel waktu ISO 8601
* `event.sequence`: penghitung yang meningkat secara monoton untuk mengurutkan acara dalam sesi
* `model`: Model yang digunakan (misalnya, "claude-sonnet-5")
* `error`: Pesan kesalahan
* `status_code`: Kode status HTTP sebagai angka. Tidak ada untuk kesalahan non-HTTP seperti kegagalan koneksi.
* `duration_ms`: Durasi permintaan dalam milidetik
* `attempt`: Jumlah total upaya yang dilakukan, termasuk permintaan awal (`1` berarti tidak ada retry yang terjadi)
* `request_id`: ID permintaan API Anthropic dari header `request-id` respons, seperti `"req_011..."`. Hadir hanya saat API mengembalikan satu.
* `speed`: `"fast"` atau `"normal"`, menunjukkan apakah mode cepat aktif
* `query_source`: Subsistem yang mengeluarkan permintaan, seperti `"repl_main_thread"`, `"compact"`, atau nama subagent
* `effort`: [Tingkat effort](/docs/id/model-config#adjust-effort-level) yang diterapkan pada permintaan. Tidak ada saat model tidak mendukung effort.
* `agent.name`, `skill.name`, `plugin.name`, `marketplace.name`, `mcp_server.name`, `mcp_tool.name`: Atribusi skill, plugin, agen, dan MCP untuk permintaan. Lihat [Penghitung biaya](#cost-counter) untuk definisi dan perilaku redaksi.

<h4 id="api-refusal-event">
  Acara penolakan API
</h4>

Dicatat saat permintaan API mengembalikan `stop_reason: "refusal"`. Penolakan tiba pada aliran respons yang berhasil daripada sebagai kesalahan HTTP, jadi acara `api_error` tidak terpicu untuk mereka. Acara ini memungkinkan Anda melacak frekuensi penolakan dan mengelompokkan penolakan berdasarkan atribut yang sama dengan `api_request` dan `api_error`.

**Nama Acara**: `claude_code.api_refusal`

**Atribut**:

* Semua [atribut standar](#standard-attributes)
* `event.name`: `"api_refusal"`
* `event.timestamp`: Stempel waktu ISO 8601
* `event.sequence`: penghitung yang meningkat secara monoton untuk mengurutkan acara dalam sesi
* `model`: Pengidentifikasi model dari permintaan
* `request_id`: ID permintaan API Anthropic dari header `request-id` respons, seperti `"req_011..."`. Hadir hanya saat API mengembalikan satu.
* `query_source`: Subsistem yang mengeluarkan permintaan, seperti `"repl_main_thread"`, `"compact"`, atau nama subagent. Lihat [`api_request`](#api-request-event) untuk definisi.
* `speed`: Baik `"fast"` saat [Mode cepat](/docs/id/fast-mode) aktif, atau `"normal"`
* `attempt`: Nomor upaya retry. Upaya pertama adalah `1`.
* `effort`: [Tingkat effort](/docs/id/model-config#adjust-effort-level) yang diterapkan pada permintaan. Tidak ada saat model tidak mendukung effort.
* `server_fallback_hop`: `true` saat fallback model server-side API sudah mencoba ulang penolakan ini pada model yang berbeda, jadi pengguna tidak melihat penolakan khusus ini. `false` saat permintaan berakhir dalam penolakan. Satu giliran dapat memancarkan acara hop `true` dan acara akhir `false` yang lebih baru saat model fallback juga menolak.
* `has_category`: `true` saat respons API membawa `stop_details.category` dari `"cyber"`, `"bio"`, `"frontier_llm"`, atau `"reasoning_extraction"`. `false` saat respons tidak membawa kategori atau nilai di luar set itu. Tidak ada saat `server_fallback_hop` adalah `true`, karena blok hop tidak membawa `stop_details`.
* `has_explanation`: `true` saat respons API membawa `stop_details.explanation`, sebaliknya `false`. Tidak ada saat `server_fallback_hop` adalah `true`.
* `category`: Nilai `stop_details.category` dari respons API. Salah satu dari `"cyber"`, `"bio"`, `"frontier_llm"`, atau `"reasoning_extraction"`. Hanya ada saat `OTEL_LOG_TOOL_DETAILS=1` diatur dan `has_category` adalah `true`.
* `agent.name`, `skill.name`, `plugin.name`, `marketplace.name`, `mcp_server.name`, `mcp_tool.name`: Atribusi skill, plugin, agen, dan MCP untuk permintaan. Lihat [Penghitung biaya](#cost-counter) untuk definisi dan perilaku redaksi.

<h4 id="api-request-body-event">
  Acara badan permintaan API
</h4>

Dicatat untuk setiap upaya permintaan API saat `OTEL_LOG_RAW_API_BODIES` diatur. Satu acara dipancarkan per upaya, jadi retry dengan parameter yang disesuaikan masing-masing menghasilkan acara mereka sendiri.

**Nama Acara**: `claude_code.api_request_body`

**Atribut**:

* Semua [atribut standar](#standard-attributes)
* `event.name`: `"api_request_body"`
* `event.timestamp`: Stempel waktu ISO 8601
* `event.sequence`: penghitung yang meningkat secara monoton untuk mengurutkan acara dalam sesi
* `body`: Parameter permintaan Messages API yang diserialisasi JSON (system prompt, messages, tools, dll.), dipotong pada 60 KB. Konten extended-thinking dalam giliran asisten sebelumnya diredaksi. Dipancarkan hanya dalam mode inline (`OTEL_LOG_RAW_API_BODIES=1`).
* `body_ref`: Jalur absolut ke file `<dir>/<uuid>.request.json` yang berisi badan yang tidak dipotong. Dipancarkan hanya dalam mode file (`OTEL_LOG_RAW_API_BODIES=file:<dir>`).
* `body_length`: Panjang badan yang tidak dipotong. Byte UTF-8 saat `OTEL_LOG_RAW_API_BODIES=file:<dir>`, atau unit kode UTF-16 saat `=1`
* `body_truncated`: `"true"` saat pemotongan inline terjadi. Tidak ada dalam mode file dan saat tidak ada pemotongan yang terjadi.
* `model`: Pengidentifikasi model dari parameter permintaan
* `query_source`: Subsistem yang mengeluarkan permintaan (misalnya, `"compact"`)

<h4 id="api-response-body-event">
  Acara badan respons API
</h4>

Dicatat untuk setiap respons API yang berhasil saat `OTEL_LOG_RAW_API_BODIES` diatur.

**Nama Acara**: `claude_code.api_response_body`

**Atribut**:

* Semua [atribut standar](#standard-attributes)
* `event.name`: `"api_response_body"`
* `event.timestamp`: Stempel waktu ISO 8601
* `event.sequence`: penghitung yang meningkat secara monoton untuk mengurutkan acara dalam sesi
* `body`: Respons Messages API yang diserialisasi JSON (id, content blocks, usage, stop reason), dipotong pada 60 KB. Konten extended-thinking diredaksi. Dipancarkan hanya dalam mode inline (`OTEL_LOG_RAW_API_BODIES=1`).
* `body_ref`: Jalur absolut ke file `<dir>/<request_id>.response.json` yang berisi badan yang tidak dipotong. Dipancarkan hanya dalam mode file (`OTEL_LOG_RAW_API_BODIES=file:<dir>`).
* `body_length`: Panjang badan yang tidak dipotong. Byte UTF-8 saat `OTEL_LOG_RAW_API_BODIES=file:<dir>`, atau unit kode UTF-16 saat `=1`
* `body_truncated`: `"true"` saat pemotongan inline terjadi. Tidak ada dalam mode file dan saat tidak ada pemotongan yang terjadi.
* `model`: Pengidentifikasi model
* `query_source`: Subsistem yang mengeluarkan permintaan
* `request_id`: ID permintaan API Anthropic dari header `request-id` respons, seperti `"req_011..."`. Hadir hanya saat API mengembalikan satu.

<h4 id="tool-decision-event">
  Acara keputusan alat
</h4>

Dicatat saat keputusan izin alat dibuat (terima/tolak).

**Nama Acara**: `claude_code.tool_decision`

**Atribut**:

* Semua [atribut standar](#standard-attributes)
* `event.name`: `"tool_decision"`
* `event.timestamp`: Stempel waktu ISO 8601
* `event.sequence`: penghitung yang meningkat secara monoton untuk mengurutkan acara dalam sesi
* `tool_name`: Nama alat (misalnya, "Read", "Edit", "Write", "NotebookEdit")
* `tool_use_id`: Pengidentifikasi unik untuk invokasi alat ini. Cocok dengan `tool_use_id` yang diteruskan ke hooks, memungkinkan korelasi antara acara OTel dan data yang ditangkap hook.
* `decision`: Baik `"accept"` atau `"reject"`
* `source`: Sumber keputusan:
  * `"config"`: Diputuskan secara otomatis tanpa diminta, berdasarkan pengaturan proyek, aturan izin dalam pengaturan pribadi pengguna, kebijakan terkelola perusahaan, flag `--allowedTools` atau `--disallowedTools`, mode izin aktif, hibah berskop sesi dari prompt sebelumnya dalam sesi CLI interaktif yang sama, atau karena alat itu aman secara inheren. Acara tidak menunjukkan sumber mana yang cocok.
  * `"hook"`: Hook `PreToolUse` atau `PermissionRequest` mengembalikan keputusan.
  * `"user_permanent"`: Dipancarkan saat pengguna memilih "Ya, dan jangan tanya lagi untuk ..." saat diminta, yang menyimpan aturan izin ke pengaturan pribadi mereka. Dalam CLI interaktif ini dipancarkan hanya untuk pilihan itu sendiri; panggilan nanti yang cocok dengan aturan tersimpan memancarkan `"config"` sebagai gantinya. Dalam sesi Agent SDK atau non-interaktif `-p`, baik pilihan awal maupun kecocokan aturan nanti memancarkan `"user_permanent"`. Diperlakukan sebagai penerimaan.
  * `"user_temporary"`: Dipancarkan saat pengguna memilih "Ya" saat diminta untuk persetujuan satu kali, atau memilih salah satu opsi "... selama sesi ini" pada prompt pengeditan atau pembacaan file. Dalam CLI interaktif ini dipancarkan hanya untuk pilihan itu sendiri; panggilan nanti yang diizinkan oleh hibah berskop sesi itu memancarkan `"config"` sebagai gantinya. Dalam sesi Agent SDK atau non-interaktif `-p`, baik pilihan maupun kecocokan nanti memancarkan `"user_temporary"`. Diperlakukan sebagai penerimaan.
  * `"user_abort"`: Dipancarkan saat pengguna menutup prompt izin tanpa menjawab. Diperlakukan sebagai penolakan.
  * `"user_reject"`: Dipancarkan saat pengguna memilih "Tidak" saat diminta. Dalam CLI interaktif ini dipancarkan hanya untuk pilihan itu sendiri; panggilan yang cocok dengan aturan penolakan dalam pengaturan pribadi mereka memancarkan `"config"` sebagai gantinya. Dalam sesi Agent SDK atau non-interaktif `-p`, panggilan yang cocok dengan aturan penolakan dalam pengaturan pribadi memancarkan `"user_reject"`. Diperlakukan sebagai penolakan.
* `tool_parameters` (saat `OTEL_LOG_TOOL_DETAILS=1`): String JSON yang berisi parameter khusus alat. Bentuk yang sama dengan [Acara hasil alat](#tool-result-event), minus bidang pasca-eksekusi seperti `git_commit_id`. Nilai mungkin berbeda dari `tool_result` untuk panggilan yang diterima jika keputusan izin menulis ulang input alat melalui `updatedInput`. Gunakan atribut ini untuk melihat perintah mana yang ditolak saat `decision` adalah `"reject"`.
  * Untuk alat Bash: mencakup `bash_command`, `full_command`, `timeout`, `description`, `dangerouslyDisableSandbox`
  * Untuk alat WorkspaceBash: mencakup `bash_command`, `full_command`, `timeout`
  * Untuk alat MCP: mencakup `mcp_server_name`, `mcp_tool_name`
  * Untuk alat Skill: mencakup `skill_name`
  * Untuk alat Agent atau alat Task legacy: mencakup `subagent_type`

<h4 id="permission-mode-changed-event">
  Acara mode izin berubah
</h4>

Dicatat saat mode izin berubah, misalnya dari siklus Shift+Tab, keluar dari plan mode, atau pemeriksaan gate mode otomatis.

**Nama Acara**: `claude_code.permission_mode_changed`

**Atribut**:

* Semua [atribut standar](#standard-attributes)
* `event.name`: `"permission_mode_changed"`
* `event.timestamp`: Stempel waktu ISO 8601
* `event.sequence`: penghitung yang meningkat secara monoton untuk mengurutkan acara dalam sesi
* `from_mode`: Mode izin sebelumnya, misalnya `"default"`, `"plan"`, `"acceptEdits"`, `"auto"`, atau `"bypassPermissions"`
* `to_mode`: Mode izin baru
* `trigger`: Apa yang menyebabkan perubahan. Salah satu dari `"shift_tab"`, `"exit_plan_mode"`, `"auto_gate_denied"`, atau `"auto_opt_in"`. Tidak ada saat transisi berasal dari SDK atau bridge

<h4 id="auth-event">
  Acara auth
</h4>

Dicatat saat `/login` atau `/logout` selesai.

**Nama Acara**: `claude_code.auth`

**Atribut**:

* Semua [atribut standar](#standard-attributes)
* `event.name`: `"auth"`
* `event.timestamp`: Stempel waktu ISO 8601
* `event.sequence`: penghitung yang meningkat secara monoton untuk mengurutkan acara dalam sesi
* `action`: `"login"` atau `"logout"`
* `success`: `"true"` atau `"false"`
* `auth_method`: Metode autentikasi, seperti `"oauth"`
* `error_category`: Jenis kesalahan kategori saat tindakan gagal. Pesan kesalahan mentah tidak pernah disertakan
* `status_code`: Kode status HTTP sebagai string saat tindakan gagal dengan kesalahan HTTP

<h4 id="mcp-server-connection-event">
  Acara koneksi server MCP
</h4>

Dicatat saat server MCP terhubung, terputus, atau gagal terhubung.

**Nama Acara**: `claude_code.mcp_server_connection`

**Atribut**:

* Semua [atribut standar](#standard-attributes)
* `event.name`: `"mcp_server_connection"`
* `event.timestamp`: Stempel waktu ISO 8601
* `event.sequence`: penghitung yang meningkat secara monoton untuk mengurutkan acara dalam sesi
* `status`: `"connected"`, `"failed"`, atau `"disconnected"`
* `transport_type`: Transport server, seperti `"stdio"`, `"sse"`, atau `"http"`
* `server_scope`: Cakupan server dikonfigurasi di, seperti `"user"`, `"project"`, atau `"local"`
* `duration_ms`: Durasi upaya koneksi dalam milidetik
* `error_code`: Kode kesalahan saat koneksi gagal
* `is_plugin`: `true` saat server disediakan oleh plugin, `false` sebaliknya
* `plugin_id_hash` (saat `is_plugin` adalah `true`): Hash stabil dari nama plugin dan marketplace, untuk mengelompokkan acara berdasarkan plugin tanpa mengekspos nama
* `plugin.name` (saat `is_plugin` adalah `true`): Nama plugin yang menyediakan server. Untuk plugin pihak ketiga ini adalah string literal `"third-party"` kecuali `OTEL_LOG_TOOL_DETAILS=1`; ini melindungi nama plugin pihak ketiga dari muncul dalam log secara default. Plugin dari sumber Anthropic resmi selalu diidentifikasi berdasarkan nama. Atribut `plugin_id_hash` dan `plugin.name` mengalir ke backend monitoring Anda sendiri dan tidak dikirim ke Anthropic
* `server_name` (saat `OTEL_LOG_TOOL_DETAILS=1`): Nama server yang dikonfigurasi
* `error` (saat `OTEL_LOG_TOOL_DETAILS=1`): Pesan kesalahan lengkap saat koneksi gagal

<h4 id="internal-error-event">
  Acara kesalahan internal
</h4>

Dicatat saat Claude Code menangkap kesalahan internal yang tidak terduga. Hanya nama kelas kesalahan dan kode gaya errno yang dicatat. Pesan kesalahan dan stack trace tidak pernah disertakan. Acara ini tidak dipancarkan saat berjalan terhadap Amazon Bedrock, Google Cloud's Agent Platform, atau Microsoft Foundry, atau saat `DISABLE_ERROR_REPORTING` diatur.

**Nama Acara**: `claude_code.internal_error`

**Atribut**:

* Semua [atribut standar](#standard-attributes)
* `event.name`: `"internal_error"`
* `event.timestamp`: Stempel waktu ISO 8601
* `event.sequence`: penghitung yang meningkat secara monoton untuk mengurutkan acara dalam sesi
* `error_name`: Nama kelas kesalahan, seperti `"TypeError"` atau `"SyntaxError"`
* `error_code`: Kode errno Node.js seperti `"ENOENT"` saat ada pada kesalahan

<h4 id="plugin-installed-event">
  Acara plugin terinstal
</h4>

Dicatat saat plugin selesai menginstal, dari perintah CLI `claude plugin install` dan UI interaktif `/plugin`.

**Nama Acara**: `claude_code.plugin_installed`

**Atribut**:

* Semua [atribut standar](#standard-attributes)
* `event.name`: `"plugin_installed"`
* `event.timestamp`: Stempel waktu ISO 8601
* `event.sequence`: penghitung yang meningkat secara monoton untuk mengurutkan acara dalam sesi
* `marketplace.is_official`: `"true"` jika marketplace adalah marketplace Anthropic resmi, `"false"` sebaliknya
* `install.trigger`: `"cli"` atau `"ui"`
* `plugin.name`: Nama plugin yang diinstal. Untuk marketplace pihak ketiga ini disertakan hanya saat `OTEL_LOG_TOOL_DETAILS=1`
* `plugin.version`: Versi plugin saat dideklarasikan dalam entri marketplace. Untuk marketplace pihak ketiga ini disertakan hanya saat `OTEL_LOG_TOOL_DETAILS=1`
* `marketplace.name`: Marketplace plugin diinstal dari. Untuk marketplace pihak ketiga ini disertakan hanya saat `OTEL_LOG_TOOL_DETAILS=1`

<h4 id="plugin-loaded-event">
  Acara plugin dimuat
</h4>

Dicatat sekali per plugin yang diaktifkan saat startup sesi. Gunakan acara ini untuk menginventarisasi plugin mana yang aktif di seluruh armada Anda, sebagai pelengkap `plugin_installed` yang mencatat tindakan instalasi itu sendiri.

**Nama Acara**: `claude_code.plugin_loaded`

**Atribut**:

* Semua [atribut standar](#standard-attributes)
* `event.name`: `"plugin_loaded"`
* `event.timestamp`: Stempel waktu ISO 8601
* `event.sequence`: penghitung yang meningkat secara monoton untuk mengurutkan acara dalam sesi
* `plugin.name`: nama plugin. Untuk plugin di luar marketplace resmi dan bundel built-in nilainya adalah `"third-party"` kecuali `OTEL_LOG_TOOL_DETAILS=1`
* `marketplace.name`: marketplace tempat plugin diinstal, saat diketahui. Diredaksi menjadi `"third-party"` di bawah kondisi yang sama dengan `plugin.name`
* `plugin.version`: versi dari manifest plugin. Disertakan hanya saat nama tidak diredaksi dan manifest mendeklarasikan versi
* `plugin.scope`: kategori provenance untuk plugin: `"official"`, `"org"`, `"user-local"`, atau `"default-bundle"`
* `enabled_via`: bagaimana plugin menjadi diaktifkan: `"default-enable"`, `"org-policy"`, `"seed-mount"`, atau `"user-install"`
* `plugin_id_hash`: hash deterministik dari nama plugin dan marketplace, dikirim hanya ke pengekspor yang dikonfigurasi. Memungkinkan Anda menghitung berapa banyak plugin pihak ketiga yang berbeda dimuat di seluruh armada Anda tanpa merekam nama mereka
* `has_hooks`: apakah plugin berkontribusi hooks
* `has_mcp`: apakah plugin berkontribusi server MCP
* `host_owned_mcp`: `true` saat host SDK mengelola koneksi MCP plugin ini dan Claude Code melewati pembacaan konfigurasi server MCP plugin, `false` sebaliknya. Memerlukan Claude Code v2.1.172 atau lebih baru
* `skill_path_count`: jumlah direktori skill yang dideklarasikan plugin
* `command_path_count`: jumlah direktori perintah yang dideklarasikan plugin
* `agent_path_count`: jumlah direktori agen yang dideklarasikan plugin
* `safe_mode`: `"true"` saat sesi dimulai dengan [`--safe-mode`](/docs/id/cli-reference), `"false"` sebaliknya. Dalam mode aman acara ini melaporkan inventaris yang dikonfigurasi saja; perintah, skill, hooks, dan server MCP plugin tidak dimuat. Memerlukan Claude Code v2.1.169 atau lebih baru

<h4 id="skill-activated-event">
  Acara skill diaktifkan
</h4>

Dicatat saat skill dipanggil, baik Claude memanggilnya melalui alat Skill atau Anda menjalankannya sebagai perintah `/`.

**Nama Acara**: `claude_code.skill_activated`

**Atribut**:

* Semua [atribut standar](#standard-attributes)
* `event.name`: `"skill_activated"`
* `event.timestamp`: Stempel waktu ISO 8601
* `event.sequence`: penghitung yang meningkat secara monoton untuk mengurutkan acara dalam sesi
* `skill.name`: Nama skill. Untuk skill yang ditentukan pengguna dan plugin pihak ketiga nilainya adalah placeholder `"custom_skill"` kecuali `OTEL_LOG_TOOL_DETAILS=1`
* `invocation_trigger`: Bagaimana skill dipicu (`"user-slash"`, `"claude-proactive"`, atau `"nested-skill"`)
* `skill.source`: Tempat skill dimuat dari (misalnya, `"bundled"`, `"userSettings"`, `"projectSettings"`, `"plugin"`)
* `skill.kind`: `"workflow"` saat skill adalah skill workflow. Tidak ada sebaliknya
* `plugin.name` (saat `OTEL_LOG_TOOL_DETAILS=1` atau plugin dari marketplace resmi): Nama plugin pemilik saat skill disediakan oleh plugin
* `marketplace.name` (saat `OTEL_LOG_TOOL_DETAILS=1` atau plugin dari marketplace resmi): Marketplace plugin pemilik diinstal dari, saat skill disediakan oleh plugin

<h4 id="at-mention-event">
  Acara mention @
</h4>

Dicatat saat Claude Code menyelesaikan mention `@` dalam prompt. Tidak setiap mention memancarkan acara: jalur early-exit seperti penolakan izin, file berukuran besar, lampiran referensi PDF, dan kegagalan listing direktori kembali tanpa logging.

**Nama Acara**: `claude_code.at_mention`

**Atribut**:

* Semua [atribut standar](#standard-attributes)
* `event.name`: `"at_mention"`
* `event.timestamp`: Stempel waktu ISO 8601
* `event.sequence`: penghitung yang meningkat secara monoton untuk mengurutkan acara dalam sesi
* `mention_type`: Jenis mention (`"file"`, `"directory"`, `"agent"`, `"mcp_resource"`)
* `success`: Apakah mention berhasil diselesaikan (`"true"` atau `"false"`)

<h4 id="api-retries-exhausted-event">
  Acara retry API habis
</h4>

Dicatat sekali saat permintaan API gagal setelah lebih dari satu upaya. Dipancarkan bersama acara `api_error` terakhir.

**Nama Acara**: `claude_code.api_retries_exhausted`

**Atribut**:

* Semua [atribut standar](#standard-attributes)
* `event.name`: `"api_retries_exhausted"`
* `event.timestamp`: Stempel waktu ISO 8601
* `event.sequence`: penghitung yang meningkat secara monoton untuk mengurutkan acara dalam sesi
* `model`: Model yang digunakan
* `error`: Pesan kesalahan terakhir
* `status_code`: Kode status HTTP sebagai angka. Tidak ada untuk kesalahan non-HTTP.
* `total_attempts`: Jumlah total upaya yang dilakukan
* `total_retry_duration_ms`: Total waktu wall-clock di semua upaya
* `speed`: `"fast"` atau `"normal"`

<h4 id="hook-registered-event">
  Acara hook terdaftar
</h4>

Dicatat sekali per hook yang dikonfigurasi saat startup sesi. Gunakan acara ini untuk menginventarisasi hook mana yang aktif di seluruh armada Anda, sebagai pelengkap acara `hook_execution_start` dan `hook_execution_complete` per eksekusi.

**Nama Acara**: `claude_code.hook_registered`

**Atribut**:

* Semua [atribut standar](#standard-attributes)
* `event.name`: `"hook_registered"`
* `event.timestamp`: Stempel waktu ISO 8601
* `event.sequence`: penghitung yang meningkat secara monoton untuk mengurutkan acara dalam sesi
* `hook_event`: jenis acara hook, seperti `"PreToolUse"` atau `"PostToolUse"`
* `hook_type`: jenis implementasi hook: `"command"`, `"prompt"`, `"mcp_tool"`, `"http"`, atau `"agent"`
* `hook_source`: tempat hook didefinisikan: `"userSettings"`, `"projectSettings"`, `"localSettings"`, `"flagSettings"`, `"policySettings"`, atau `"pluginHook"`
* `safe_mode`: `"true"` saat sesi dimulai dengan [`--safe-mode`](/docs/id/cli-reference), `"false"` sebaliknya. Memerlukan Claude Code v2.1.169 atau lebih baru
* `hook_matcher` (saat `OTEL_LOG_TOOL_DETAILS=1`): string matcher dari konfigurasi hook, saat satu diatur
* `plugin.name` (saat `hook_source` adalah `"pluginHook"`): nama plugin yang berkontribusi. Untuk plugin di luar marketplace resmi dan bundel built-in nilainya adalah `"third-party"` kecuali `OTEL_LOG_TOOL_DETAILS=1`
* `plugin_id_hash` (saat `hook_source` adalah `"pluginHook"`): hash deterministik dari nama plugin dan marketplace, dikirim hanya ke pengekspor yang dikonfigurasi. Memungkinkan Anda menghitung plugin yang berkontribusi berbeda tanpa merekam nama mereka

<h4 id="hook-execution-start-event">
  Acara mulai eksekusi hook
</h4>

Dicatat saat satu atau lebih hooks mulai dieksekusi untuk acara hook.

**Nama Acara**: `claude_code.hook_execution_start`

**Atribut**:

* Semua [atribut standar](#standard-attributes)
* `event.name`: `"hook_execution_start"`
* `event.timestamp`: Stempel waktu ISO 8601
* `event.sequence`: penghitung yang meningkat secara monoton untuk mengurutkan acara dalam sesi
* `hook_event`: Jenis acara hook, seperti `"PreToolUse"` atau `"PostToolUse"`
* `hook_name`: Nama hook lengkap termasuk matcher, seperti `"PreToolUse:Write"`
* `num_hooks`: Jumlah perintah hook yang cocok
* `managed_only`: `"true"` saat hanya hooks kebijakan terkelola yang diizinkan
* `hook_source`: `"policySettings"` atau `"merged"`
* `safe_mode`: `"true"` saat sesi dimulai dengan [`--safe-mode`](/docs/id/cli-reference), `"false"` sebaliknya. Memerlukan Claude Code v2.1.169 atau lebih baru
* `hook_definitions`: Konfigurasi hook yang diserialisasi JSON. Disertakan hanya saat detailed beta tracing dan `OTEL_LOG_TOOL_DETAILS=1` keduanya diaktifkan

<h4 id="hook-execution-complete-event">
  Acara eksekusi hook selesai
</h4>

Dicatat saat semua hooks untuk acara hook selesai.

**Nama Acara**: `claude_code.hook_execution_complete`

**Atribut**:

* Semua [atribut standar](#standard-attributes)
* `event.name`: `"hook_execution_complete"`
* `event.timestamp`: Stempel waktu ISO 8601
* `event.sequence`: penghitung yang meningkat secara monoton untuk mengurutkan acara dalam sesi
* `hook_event`: Jenis acara hook
* `hook_name`: Nama hook lengkap termasuk matcher
* `num_hooks`: Jumlah perintah hook yang cocok
* `num_success`: Jumlah yang selesai dengan sukses
* `num_blocking`: Jumlah yang mengembalikan keputusan blocking
* `num_non_blocking_error`: Jumlah yang gagal tanpa blocking
* `num_cancelled`: Jumlah dibatalkan sebelum selesai
* `total_duration_ms`: Durasi wall-clock dari semua hooks yang cocok
* `managed_only`: `"true"` saat hanya hooks kebijakan terkelola yang diizinkan
* `hook_source`: `"policySettings"` atau `"merged"`
* `safe_mode`: `"true"` saat sesi dimulai dengan [`--safe-mode`](/docs/id/cli-reference), `"false"` sebaliknya. Memerlukan Claude Code v2.1.169 atau lebih baru
* `hook_definitions`: Konfigurasi hook yang diserialisasi JSON. Disertakan hanya saat detailed beta tracing dan `OTEL_LOG_TOOL_DETAILS=1` keduanya diaktifkan

<h4 id="hook-plugin-metrics-event">
  Acara metrik plugin hook
</h4>

Dicatat saat hook plugin marketplace resmi memancarkan metrik per-invokasi. Hanya plugin yang diinstal dari marketplace Anthropic resmi yang dapat memancarkan ini. Plugin marketplace pihak ketiga dan hook yang dikonfigurasi pengguna tidak memancarkan ke acara ini. Gunakan acara ini untuk memantau perilaku plugin seperti tingkat penemuan, biaya, dan durasi dari stack observabilitas Anda sendiri.

**Nama Acara**: `claude_code.hook_plugin_metrics`

**Atribut**:

* Semua [atribut standar](#standard-attributes)
* `event.name`: `"hook_plugin_metrics"`
* `event.timestamp`: Stempel waktu ISO 8601
* `event.sequence`: penghitung yang meningkat secara monoton untuk mengurutkan acara dalam sesi
* `plugin_id`: pengidentifikasi plugin dalam bentuk `<name>@<marketplace>`
* `hook_event`: jenis acara hook yang memancarkan metrik
* Hingga 20 kunci metrik yang dipancarkan plugin. Nama cocok dengan `^[a-z][a-z0-9_]{0,39}$`. Nilai adalah boolean atau angka.

<h4 id="compaction-event">
  Acara pemadatan
</h4>

Dicatat saat pemadatan percakapan selesai.

**Nama Acara**: `claude_code.compaction`

**Atribut**:

* Semua [atribut standar](#standard-attributes)
* `event.name`: `"compaction"`
* `event.timestamp`: Stempel waktu ISO 8601
* `event.sequence`: penghitung yang meningkat secara monoton untuk mengurutkan acara dalam sesi
* `trigger`: `"auto"` atau `"manual"`
* `success`: `"true"` atau `"false"`
* `duration_ms`: Durasi pemadatan
* `pre_tokens`: Jumlah token perkiraan sebelum pemadatan
* `post_tokens`: Jumlah token perkiraan setelah pemadatan
* `error`: Pesan kesalahan saat pemadatan gagal
* `precompute_reuse`: Hanya diatur saat `trigger` adalah `"manual"`. Auto-compaction dapat menyiapkan ringkasan di latar belakang sebelum jendela konteks penuh, dan atribut ini mencatat apakah `/compact` menggunakan kembali ringkasan yang disiapkan itu. `"hit"` berarti itu digunakan kembali; `"miss_custom_instructions"`, `"miss_hook"`, dan `"miss_not_ready"` memberikan alasan ringkasan segar dihitung sebagai gantinya. Memerlukan Claude Code v2.1.153 atau lebih baru

<h4 id="feedback-survey-event">
  Acara survei umpan balik
</h4>

Dicatat saat survei kualitas sesi ditampilkan atau dijawab. Lihat [Survei kualitas sesi](/docs/id/data-usage#session-quality-surveys) untuk mengetahui apa yang dikumpulkan survei dan cara mengontrolnya.

**Nama Acara**: `claude_code.feedback_survey`

**Atribut**:

* Semua [atribut standar](#standard-attributes)
* `event.name`: `"feedback_survey"`
* `event.timestamp`: Stempel waktu ISO 8601
* `event.sequence`: penghitung yang meningkat secara monoton untuk mengurutkan acara dalam sesi
* `event_type`: Acara siklus hidup survei, misalnya `"appeared"`, `"responded"`, atau `"transcript_prompt_appeared"`
* `appearance_id`: ID unik yang menghubungkan acara yang dipancarkan untuk satu instance survei
* `survey_type`: Survei mana yang menghasilkan acara. `"session"` adalah prompt rating "Bagaimana Claude melakukannya?"
* `response`: Pilihan pengguna pada acara `responded`
* `enabled_via_override`: `true` saat [`CLAUDE_CODE_ENABLE_FEEDBACK_SURVEY_FOR_OTEL`](/docs/id/env-vars) diatur. Dipancarkan sebagai boolean, bukan string. Hadir pada acara survei `session`. Filter pada atribut ini untuk mengkonfirmasi override diterapkan di seluruh armada

<h2 id="interpret-metrics-and-events-data">
  Menafsirkan data metrik dan acara
</h2>

Metrik dan acara yang diekspor mendukung berbagai analisis:

<h3 id="usage-monitoring">
  Pemantauan penggunaan
</h3>

| Metrik                                                        | Peluang Analisis                                                                                                 |
| ------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------- |
| `claude_code.token.usage`                                     | Pecahkan berdasarkan `type` (input/output), pengguna, tim, model, `skill.name`, `plugin.name`, atau `agent.name` |
| `claude_code.session.count`                                   | Lacak adopsi dan keterlibatan dari waktu ke waktu                                                                |
| `claude_code.lines_of_code.count`                             | Ukur produktivitas dengan melacak penambahan dan penghapusan kode, dipecah berdasarkan model                     |
| `claude_code.commit.count` & `claude_code.pull_request.count` | Pahami dampak pada alur kerja pengembangan                                                                       |

<h3 id="cost-monitoring">
  Pemantauan biaya
</h3>

Metrik `claude_code.cost.usage` membantu dengan:

* Melacak tren penggunaan di seluruh tim atau individu
* Mengidentifikasi sesi penggunaan tinggi untuk optimasi
* Atribusi pengeluaran ke skill, plugin, atau jenis subagent tertentu melalui atribut `skill.name`, `plugin.name`, dan `agent.name`

<Note>
  Metrik biaya adalah perkiraan. Untuk data penagihan resmi, lihat penyedia API Anda (Claude Console, Amazon Bedrock, atau Google Cloud's Agent Platform).
</Note>

<h3 id="alerting-and-segmentation">
  Peringatan dan segmentasi
</h3>

Peringatan umum untuk dipertimbangkan:

* Lonjakan biaya
* Konsumsi token yang tidak biasa
* Volume sesi tinggi dari pengguna tertentu

Semua metrik dapat disegmentasikan berdasarkan [atribut standar](#standard-attributes). Atribut `model` tersedia pada `claude_code.token.usage`, `claude_code.cost.usage`, dan dari v2.1.172, `claude_code.lines_of_code.count`.

Rincian per-model dari commit hanya dapat didekati dengan menggabungkan terhadap metrik token atau biaya pada `session.id`, karena satu sesi dapat mencakup beberapa model. Filter sisi token atau biaya ke baris di mana `query_source` adalah `"main"` sehingga permintaan auxiliary dan subagent tidak mengatribusikan commit sesi ke model yang tidak membuatnya.

<h3 id="detect-retry-exhaustion">
  Deteksi kelelahan retry
</h3>

Claude Code mencoba ulang permintaan API yang gagal secara internal dan hanya memancarkan acara `claude_code.api_error` tunggal setelah menyerah, jadi acara itu sendiri adalah sinyal terminal untuk permintaan tersebut. Upaya retry perantara tidak dicatat sebagai acara terpisah.

Atribut `attempt` pada acara mencatat berapa banyak upaya yang dilakukan secara total. `CLAUDE_CODE_MAX_RETRIES` default ke 10 dan dibatasi pada 15; sejak v2.1.199, `CLAUDE_CODE_RETRY_WATCHDOG` menaikkan default dan menghapus batas. Ketika permintaan menghabiskan semua retry pada kesalahan transien, `attempt` sama dengan satu lebih dari batas efektif tersebut: 11 secara default, dan tidak pernah lebih dari 16 kecuali watchdog diatur. Nilai yang lebih rendah menunjukkan kesalahan yang tidak dapat dicoba ulang seperti respons `400`.

Untuk membedakan sesi yang pulih dari sesi yang terhenti, kelompokkan acara berdasarkan `session.id` dan periksa apakah acara `api_request` yang lebih baru ada setelah kesalahan.

<h3 id="event-analysis">
  Analisis acara
</h3>

Data acara memberikan wawasan terperinci tentang interaksi Claude Code:

**Pola Penggunaan Alat**: analisis acara hasil alat untuk mengidentifikasi:

* Alat yang paling sering digunakan
* Tingkat keberhasilan alat
* Waktu eksekusi alat rata-rata
* Pola kesalahan berdasarkan jenis alat

**Pemantauan Kinerja**: lacak durasi permintaan API dan waktu eksekusi alat untuk mengidentifikasi hambatan kinerja.

<h2 id="audit-security-events">
  Audit acara keamanan
</h2>

Acara OpenTelemetry adalah sumber data audit untuk aktivitas Claude Code. Setiap acara membawa atribut identitas yang menghubungkan panggilan alat, aktivitas MCP, dan keputusan izin kembali ke pengguna yang memicunya. Pengekspor log OTLP dapat mengirimkan acara ini ke platform Security Information and Event Management (SIEM) apa pun dengan penerima OTLP, atau ke OpenTelemetry Collector yang meneruskan ke SIEM Anda.

<h3 id="attribute-actions-to-users">
  Atribut tindakan untuk pengguna
</h3>

[Atribut standar](#standard-attributes) pada setiap acara mencakup identitas pengguna yang diautentikasi: `user.email`, `user.account_uuid`, `user.account_id`, dan `organization.id` saat masuk dengan akun Claude, ditambah `user.id` dan `session.id` per-sesi. `user.id` adalah pengidentifikasi berskop instalasi, kecuali pada sesi [Claude apps gateway](/docs/id/claude-apps-gateway), di mana ini adalah subjek IdP dari token yang dikeluarkan gateway.

Panggilan alat MCP, perintah Bash, dan pengeditan file oleh karena itu dikaitkan dengan pengembang yang memulai sesi. Claude Code tidak bertindak di bawah akun layanan terpisah; identitas yang dicatat pada setiap acara adalah akun Claude pengembang itu sendiri, atau identitas IdP pengembang pada sesi [Claude apps gateway](/docs/id/claude-apps-gateway).

Saat Claude Code diautentikasi dengan kunci API langsung, atau terhadap Amazon Bedrock, Google Cloud's Agent Platform, atau Microsoft Foundry, tidak ada akun Claude dalam sesi dan hanya `user.id` dan `session.id` yang diisi. Dalam penerapan ini, lampirkan identitas pengguna sendiri dengan `OTEL_RESOURCE_ATTRIBUTES`, atur per pengguna melalui file [pengaturan terkelola](#administrator-configuration) atau pembungkus peluncuran. Sesi Claude apps gateway tidak memerlukan apa pun dari ini: CLI mencap identitas IdP secara otomatis, seperti yang dijelaskan dalam [Atribut standar](#standard-attributes).

```bash theme={null}
export OTEL_RESOURCE_ATTRIBUTES="enduser.id=jdoe@example.com,enduser.directory_id=S-1-5-21-..."
```

<h3 id="audit-mcp-activity">
  Audit aktivitas MCP
</h3>

Untuk menangkap aktivitas server MCP dengan detail panggilan lengkap, aktifkan pengekspor log dan atur `OTEL_LOG_TOOL_DETAILS=1`. Setiap operasi MCP kemudian menghasilkan acara terstruktur yang membawa nama server, nama alat, dan argumen panggilan bersama atribut identitas standar:

| Acara                   | Apa yang dicatat untuk MCP                                                                                                                                                                             |
| ----------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `mcp_server_connection` | Server terhubung, terputus, dan kegagalan koneksi dengan `server_name`, `transport_type`, `server_scope`, dan detail kesalahan                                                                         |
| `tool_result`           | Setiap panggilan alat MCP dengan `tool_name` dan `mcp_server_scope`, muatan `tool_parameters` yang berisi `mcp_server_name` dan `mcp_tool_name`, dan muatan `tool_input` yang berisi argumen panggilan |
| `tool_decision`         | Apakah panggilan diizinkan atau ditolak, apakah keputusan berasal dari config, hook, atau pengguna, dan muatan `tool_parameters` yang berisi `mcp_server_name` dan `mcp_tool_name`                     |

Tanpa `OTEL_LOG_TOOL_DETAILS`, acara ini menghilangkan detail pengidentifikasi:

* `tool_result`: menyimpan `tool_name` dan `mcp_server_scope`, menghilangkan `mcp_server_name`, `mcp_tool_name`, dan argumen
* `tool_decision`: menyimpan `tool_name`, menghilangkan `tool_parameters`
* `mcp_server_connection`: menghilangkan `server_name` dan pesan kesalahan, tetapi menyimpan `is_plugin`, `plugin_id_hash`, dan `plugin.name`, dengan nama plugin non-Anthropic diredaksi ke literal `"third-party"`, sehingga server yang disediakan plugin tetap dapat dibedakan tanpa pencatatan terperinci

<h3 id="map-security-questions-to-events">
  Peta pertanyaan keamanan ke acara
</h3>

Saat membangun aturan deteksi, cari sinyal yang ingin Anda pantau dan kueri backend Anda untuk acara dan atribut yang sesuai:

| Sinyal                                              | Acara                                                                                      | Atribut Kunci                                                |
| --------------------------------------------------- | ------------------------------------------------------------------------------------------ | ------------------------------------------------------------ |
| Panggilan alat diizinkan atau ditolak, dan oleh apa | `tool_decision`                                                                            | `decision`, `source`, `tool_name`, `tool_parameters`         |
| Eskalasi mode izin                                  | `permission_mode_changed`                                                                  | `from_mode`, `to_mode`, `trigger`                            |
| Hook kebijakan memblokir tindakan                   | `hook_execution_complete`                                                                  | `hook_event`, `num_blocking`                                 |
| Login, logout, dan kegagalan autentikasi            | `auth`                                                                                     | `action`, `success`, `error_category`                        |
| Server MCP terhubung atau gagal                     | `mcp_server_connection`                                                                    | `status`, `server_name`, `is_plugin`, `error_code`           |
| Plugin diinstal dan sumbernya                       | `plugin_installed`                                                                         | `plugin.name`, `marketplace.name`, `marketplace.is_official` |
| Perintah yang dijalankan dan file yang disentuh     | `tool_result` (dieksekusi) atau `tool_decision` (ditolak) dengan `OTEL_LOG_TOOL_DETAILS=1` | `tool_parameters`; `tool_input` (`tool_result` saja)         |

Claude Code memancarkan aliran acara mentah saja. Deteksi anomali, baselining, korelasi lintas sesi, dan peringatan adalah tanggung jawab backend SIEM atau observabilitas Anda.

<h3 id="send-events-to-a-siem">
  Kirim acara ke SIEM
</h3>

Arahkan `OTEL_EXPORTER_OTLP_LOGS_ENDPOINT` ke penerima OTLP SIEM Anda, atau ke OpenTelemetry Collector yang meneruskan ke API ingest asli SIEM Anda. Contoh pengaturan terkelola berikut mengekspor acara saja, dengan detail alat lengkap diaktifkan untuk audit MCP dan Bash:

```json theme={null}
{
  "env": {
    "CLAUDE_CODE_ENABLE_TELEMETRY": "1",
    "OTEL_LOGS_EXPORTER": "otlp",
    "OTEL_LOG_TOOL_DETAILS": "1",
    "OTEL_EXPORTER_OTLP_LOGS_PROTOCOL": "http/protobuf",
    "OTEL_EXPORTER_OTLP_LOGS_ENDPOINT": "https://siem.example.com:4318/v1/logs",
    "OTEL_EXPORTER_OTLP_HEADERS": "Authorization=Bearer your-siem-token"
  }
}
```

<h2 id="backend-considerations">
  Pertimbangan backend
</h2>

Pilihan backend metrik, log, dan traces Anda menentukan jenis analisis yang dapat Anda lakukan:

<h3 id="for-metrics">
  Untuk metrik
</h3>

* **Database deret waktu (misalnya, Prometheus)**: Perhitungan laju, metrik agregat
* **Toko kolumnar (misalnya, ClickHouse)**: Kueri kompleks, analisis pengguna unik
* **Platform observabilitas lengkap (misalnya, Honeycomb, Datadog, Grafana Cloud)**: Kueri lanjutan, visualisasi, peringatan

<h3 id="for-events/logs">
  Untuk acara/log
</h3>

* **Sistem agregasi log (misalnya, Elasticsearch, Loki)**: Pencarian teks lengkap, analisis log
* **Toko kolumnar (misalnya, ClickHouse)**: Analisis acara terstruktur
* **Platform observabilitas lengkap (misalnya, Honeycomb, Datadog, Grafana Cloud)**: Korelasi antara metrik dan acara

<h3 id="for-traces">
  Untuk traces
</h3>

Pilih backend yang mendukung penyimpanan distributed trace dan korelasi span:

* **Sistem distributed tracing (misalnya, Jaeger, Zipkin, Grafana Tempo)**: Visualisasi span, request waterfalls, analisis latensi
* **Platform observabilitas lengkap (misalnya, Honeycomb, Datadog, Grafana Cloud)**: Pencarian trace dan korelasi dengan metrik dan log

Untuk organisasi yang memerlukan metrik Pengguna Aktif Harian/Mingguan/Bulanan (DAU/WAU/MAU), pertimbangkan backend yang mendukung kueri nilai unik yang efisien.

<h2 id="service-information">
  Informasi layanan
</h2>

Semua metrik dan acara diekspor dengan atribut sumber daya berikut:

* `service.name`: `claude-code`
* `service.version`: Versi Claude Code saat ini
* `os.type`: Jenis sistem operasi (misalnya, `linux`, `darwin`, `windows`)
* `os.version`: String versi sistem operasi
* `host.arch`: Arsitektur host (misalnya, `amd64`, `arm64`)
* `wsl.version`: Nomor versi WSL (hanya ada saat berjalan di Windows Subsystem for Linux)
* Nama Meter: `com.anthropic.claude_code`

<h2 id="roi-measurement-resources">
  Sumber daya pengukuran ROI
</h2>

Untuk panduan komprehensif tentang mengukur pengembalian investasi untuk Claude Code, termasuk pengaturan telemetri, analisis biaya, metrik produktivitas, dan pelaporan otomatis, lihat [Panduan Pengukuran ROI Claude Code](https://github.com/anthropics/claude-code-monitoring-guide). Repositori ini menyediakan konfigurasi Docker Compose siap pakai, pengaturan Prometheus dan OpenTelemetry, dan template untuk menghasilkan laporan produktivitas yang terintegrasi dengan alat seperti Linear.

<h2 id="security-and-privacy">
  Keamanan dan privasi
</h2>

* Ekspor OpenTelemetry ke backend Anda adalah opt-in dan memerlukan konfigurasi eksplisit. Untuk telemetri operasional terpisah Anthropic dan cara menonaktifkannya, lihat [Penggunaan data](/docs/id/data-usage#telemetry-services)
* Konten file mentah dan cuplikan kode tidak disertakan dalam metrik atau acara. Trace spans adalah jalur data terpisah: lihat poin `OTEL_LOG_TOOL_CONTENT` di bawah
* Saat diautentikasi melalui OAuth, `user.email` disertakan dalam atribut telemetri. Jika ini menjadi perhatian bagi organisasi Anda, bekerja dengan backend telemetri Anda untuk memfilter atau menyunting bidang ini
* Konten prompt pengguna tidak dikumpulkan secara default. Hanya panjang prompt yang dicatat. Untuk menyertakan konten prompt, atur `OTEL_LOG_USER_PROMPTS=1`
* Teks respons asisten tidak dikumpulkan secara default. Hanya panjang respons yang dicatat. Untuk menyertakan teks respons, atur `OTEL_LOG_ASSISTANT_RESPONSES=1`. Seperti semua data OpenTelemetry dari Claude Code, teks respons dikirim hanya ke titik akhir OTel yang Anda konfigurasikan, tidak pernah ke Anthropic. Ketika variabel ini tidak diatur, `OTEL_LOG_USER_PROMPTS` digunakan sebagai fallback, jadi atur `OTEL_LOG_ASSISTANT_RESPONSES=0` jika Anda menginginkan konten prompt tanpa konten respons
* Argumen input alat dan parameter tidak dicatat secara default. Untuk menyertakannya, atur `OTEL_LOG_TOOL_DETAILS=1`. Data ini dikirim hanya ke titik akhir OTEL yang Anda konfigurasikan, tidak pernah ke Anthropic. Argumen mungkin masih berisi nilai sensitif, jadi konfigurasikan backend telemetri Anda untuk memfilter atau menyunting atribut ini sesuai kebutuhan. Saat diaktifkan:
  * Acara `tool_result` dan `tool_decision` menyertakan atribut `tool_parameters` dengan perintah Bash, nama server MCP dan alat, dan nama skill. Bidang seperti `full_command` dipancarkan tanpa pemotongan
  * Acara `tool_result` juga menyertakan atribut `tool_input` dengan jalur file, URL, pola pencarian, dan argumen lainnya. Nilai individual di atas 512 karakter dipotong dan total dibatasi hingga \~4 K karakter
  * Acara `user_prompt` menyertakan `command_name` verbatim untuk perintah custom, plugin, dan MCP
  * Trace spans menyertakan atribut `tool_input` yang sama dan atribut yang diturunkan dari input seperti `file_path`, dengan pemotongan yang sama dengan `tool_input`
* Konten input dan output alat tidak dicatat dalam trace spans secara default. Untuk menyertakannya, atur `OTEL_LOG_TOOL_CONTENT=1`. Saat diaktifkan, acara span menyertakan konten input dan output alat lengkap dipotong pada 60 KB per span. Ini dapat mencakup konten file mentah dari hasil alat Read dan output perintah Bash. Konfigurasikan backend telemetri Anda untuk memfilter atau menyunting atribut ini sesuai kebutuhan
* Badan permintaan dan respons API Anthropic Messages mentah tidak dicatat secara default. Untuk menyertakannya, atur `OTEL_LOG_RAW_API_BODIES`. Dengan `=1`, setiap panggilan API memancarkan acara log `api_request_body` dan `api_response_body` yang atribut `body`-nya adalah muatan yang diserialisasi JSON, dipotong pada 60 KB. Dengan `=file:<dir>`, badan yang tidak dipotong ditulis ke file `.request.json` dan `.response.json` di bawah direktori tersebut dan acara membawa jalur `body_ref` sebagai gantinya dari badan inline. Kirim direktori dengan pengumpul log atau sidecar daripada melalui aliran telemetri. Dalam kedua mode, badan berisi riwayat percakapan lengkap (system prompt, setiap giliran pengguna dan asisten sebelumnya, hasil alat), jadi mengaktifkan ini menyiratkan persetujuan untuk semua yang akan diungkapkan oleh flag konten `OTEL_LOG_*` lainnya. Konten extended-thinking Claude selalu diredaksi dari badan ini terlepas dari pengaturan lain

<h2 id="monitor-claude-code-on-amazon-bedrock">
  Memantau Claude Code di Amazon Bedrock
</h2>

Untuk panduan pemantauan penggunaan Claude Code terperinci untuk Amazon Bedrock, lihat [Claude Code Monitoring Implementation (Amazon Bedrock)](https://github.com/aws-solutions-library-samples/guidance-for-claude-code-with-amazon-bedrock/blob/main/assets/docs/MONITORING.md).
