> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Observability dengan OpenTelemetry

> Ekspor traces, metrics, dan events dari Agent SDK ke backend observability Anda menggunakan OpenTelemetry.

Ketika Anda menjalankan agents di production, Anda memerlukan visibilitas ke dalam apa yang mereka lakukan:

* tools mana yang mereka panggil
* berapa lama setiap permintaan model memakan waktu
* berapa banyak tokens yang dihabiskan
* di mana kegagalan terjadi

Agent SDK dapat mengekspor data ini sebagai OpenTelemetry traces, metrics, dan log events ke backend apa pun yang menerima OpenTelemetry Protocol (OTLP), seperti Honeycomb, Datadog, Grafana, Langfuse, atau collector yang di-host sendiri.

Panduan ini menjelaskan bagaimana SDK memancarkan telemetry, cara mengonfigurasi ekspor, dan cara menandai dan memfilter data setelah mencapai backend Anda. Untuk membaca penggunaan token dan biaya langsung dari aliran respons SDK alih-alih mengekspor ke backend, lihat [Track cost and usage](/docs/id/agent-sdk/cost-tracking).

<h2 id="how-telemetry-flows-from-the-sdk">
  Bagaimana telemetry mengalir dari SDK
</h2>

Agent SDK menjalankan Claude Code CLI sebagai child process dan berkomunikasi dengannya melalui local pipe. CLI memiliki instrumentasi OpenTelemetry built-in: ia merekam spans di sekitar setiap permintaan model dan eksekusi tool, memancarkan metrics untuk token dan cost counters, dan memancarkan structured log events untuk prompts dan tool results. SDK tidak menghasilkan telemetry sendiri. Sebaliknya, ia melewatkan konfigurasi ke CLI process, dan CLI mengekspor langsung ke collector Anda.

Konfigurasi dilewatkan sebagai environment variables. Secara default, child process mewarisi environment aplikasi Anda, jadi Anda dapat mengonfigurasi telemetry di salah satu dari dua tempat:

* **Process environment:** atur variables di shell, container, atau orchestrator Anda sebelum aplikasi Anda dimulai. Setiap panggilan `query()` mengambilnya secara otomatis tanpa perubahan kode. Ini adalah pendekatan yang direkomendasikan untuk production deployments.
* **Per-call options:** atur variables di `ClaudeAgentOptions.env` (Python) atau `options.env` (TypeScript). Gunakan ini ketika agents yang berbeda dalam proses yang sama memerlukan pengaturan telemetry yang berbeda. Di Python, `env` digabungkan di atas environment yang diwarisi. Di TypeScript, `env` menggantikan environment yang diwarisi sepenuhnya, jadi sertakan `...process.env` dalam object yang Anda lewatkan.

CLI mengekspor tiga sinyal OpenTelemetry independen. Masing-masing memiliki switch enable sendiri dan exporter sendiri, jadi Anda dapat mengaktifkan hanya yang Anda butuhkan.

| Signal     | Apa yang dikandungnya                                                           | Aktifkan dengan                                                     |
| ---------- | ------------------------------------------------------------------------------- | ------------------------------------------------------------------- |
| Metrics    | Counters untuk tokens, cost, sessions, lines of code, dan tool decisions        | `OTEL_METRICS_EXPORTER`                                             |
| Log events | Structured records untuk setiap prompt, API request, API error, dan tool result | `OTEL_LOGS_EXPORTER`                                                |
| Traces     | Spans untuk setiap interaction, model request, tool call, dan hook (beta)       | `OTEL_TRACES_EXPORTER` plus `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA=1` |

Untuk daftar lengkap nama metric, nama event, dan attributes, lihat referensi Claude Code [Monitoring](/docs/id/monitoring-usage). Agent SDK memancarkan data yang sama karena menjalankan CLI yang sama. Nama span tercantum dalam [Read agent traces](#read-agent-traces) di bawah.

<h2 id="enable-telemetry-export">
  Aktifkan ekspor telemetry
</h2>

Telemetry dimatikan sampai Anda mengatur `CLAUDE_CODE_ENABLE_TELEMETRY=1` dan memilih setidaknya satu exporter. Konfigurasi paling umum mengirim ketiga sinyal melalui OTLP HTTP ke collector.

Contoh berikut mengatur variables dalam dictionary dan meneruskannya melalui `options.env`. Agent menjalankan satu task, dan CLI mengekspor spans, metrics, dan events ke collector di `collector.example.com` sementara loop mengonsumsi response stream:

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions

  OTEL_ENV = {
      "CLAUDE_CODE_ENABLE_TELEMETRY": "1",
      # Required for traces, which are in beta. Metrics and log events do not need this.
      "CLAUDE_CODE_ENHANCED_TELEMETRY_BETA": "1",
      # Choose an exporter per signal. Use otlp for the SDK; see the Note below.
      "OTEL_TRACES_EXPORTER": "otlp",
      "OTEL_METRICS_EXPORTER": "otlp",
      "OTEL_LOGS_EXPORTER": "otlp",
      # Standard OTLP transport configuration.
      "OTEL_EXPORTER_OTLP_PROTOCOL": "http/protobuf",
      "OTEL_EXPORTER_OTLP_ENDPOINT": "http://collector.example.com:4318",
      "OTEL_EXPORTER_OTLP_HEADERS": "Authorization=Bearer your-token",
  }


  async def main():
      options = ClaudeAgentOptions(env=OTEL_ENV)
      async for message in query(
          prompt="List the files in this directory", options=options
      ):
          print(message)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const otelEnv = {
    CLAUDE_CODE_ENABLE_TELEMETRY: "1",
    // Required for traces, which are in beta. Metrics and log events do not need this.
    CLAUDE_CODE_ENHANCED_TELEMETRY_BETA: "1",
    // Choose an exporter per signal. Use otlp for the SDK; see the Note below.
    OTEL_TRACES_EXPORTER: "otlp",
    OTEL_METRICS_EXPORTER: "otlp",
    OTEL_LOGS_EXPORTER: "otlp",
    // Standard OTLP transport configuration.
    OTEL_EXPORTER_OTLP_PROTOCOL: "http/protobuf",
    OTEL_EXPORTER_OTLP_ENDPOINT: "http://collector.example.com:4318",
    OTEL_EXPORTER_OTLP_HEADERS: "Authorization=Bearer your-token",
  };

  for await (const message of query({
    prompt: "List the files in this directory",
    // env replaces the inherited environment in TypeScript, so spread
    // process.env first to keep PATH, ANTHROPIC_API_KEY, and other variables.
    options: { env: { ...process.env, ...otelEnv } },
  })) {
    console.log(message);
  }
  ```
</CodeGroup>

Karena child process mewarisi environment aplikasi Anda secara default, Anda dapat mencapai hasil yang sama dengan mengekspor variables ini dalam Dockerfile, Kubernetes manifest, atau shell profile dan menghilangkan `options.env` sepenuhnya.

<Note>
  Exporter `console` menulis telemetry ke standard output, yang digunakan SDK sebagai message channel-nya. Jangan atur `console` sebagai nilai exporter saat menjalankan melalui SDK. Untuk menginspeksi telemetry secara lokal, arahkan `OTEL_EXPORTER_OTLP_ENDPOINT` ke collector lokal atau container Jaeger all-in-one sebagai gantinya.
</Note>

<h3 id="flush-telemetry-from-short-lived-calls">
  Flush telemetry dari short-lived calls
</h3>

CLI melakukan batch telemetry dan mengekspor pada interval. Pada clean process exit, ia mencoba untuk flush pending data, tetapi flush dibatasi oleh timeout pendek, jadi spans masih dapat dijatuhkan jika collector lambat merespons. Jika proses Anda dibunuh sebelum CLI shutdown, apa pun yang masih dalam batch buffer hilang. Menurunkan export intervals mengurangi kedua window.

Secara default, metrics mengekspor setiap 60 detik dan traces dan logs mengekspor setiap 5 detik. Contoh berikut mempersingkat ketiga interval sehingga data mencapai collector sementara task pendek masih berjalan:

<CodeGroup>
  ```python Python theme={null}
  OTEL_ENV = {
      # ... exporter configuration from the previous example ...
      "OTEL_METRIC_EXPORT_INTERVAL": "1000",
      "OTEL_LOGS_EXPORT_INTERVAL": "1000",
      "OTEL_TRACES_EXPORT_INTERVAL": "1000",
  }
  ```

  ```typescript TypeScript theme={null}
  const otelEnv = {
    // ... exporter configuration from the previous example ...
    OTEL_METRIC_EXPORT_INTERVAL: "1000",
    OTEL_LOGS_EXPORT_INTERVAL: "1000",
    OTEL_TRACES_EXPORT_INTERVAL: "1000",
  };
  ```
</CodeGroup>

<h2 id="read-agent-traces">
  Baca agent traces
</h2>

Traces memberikan Anda tampilan paling detail dari agent run. Dengan `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA=1` diatur, setiap langkah dari agent loop menjadi span yang dapat Anda inspeksi di backend tracing Anda:

* **`claude_code.interaction`:** membungkus satu turn dari agent loop, dari menerima prompt hingga menghasilkan response.
* **`claude_code.llm_request`:** membungkus setiap panggilan ke Claude API, dengan model name, latency, dan token counts sebagai attributes.
* **`claude_code.tool`:** membungkus setiap tool invocation, dengan child spans untuk permission wait (`claude_code.tool.blocked_on_user`) dan execution itu sendiri (`claude_code.tool.execution`).
* **`claude_code.hook`:** membungkus setiap eksekusi [hook](/docs/id/agent-sdk/hooks). Memerlukan detailed beta tracing (`ENABLE_BETA_TRACING_DETAILED=1` dan `BETA_TRACING_ENDPOINT`) sebagai tambahan untuk variables di atas.

Spans `llm_request`, `tool`, dan `hook` adalah children dari enclosing `claude_code.interaction` span. Ketika agent menghasilkan subagent melalui Task tool, spans `llm_request` dan `tool` subagent bersarang di bawah `claude_code.tool` span parent agent, jadi full delegation chain muncul sebagai satu trace.

Spans membawa atribut `session.id` secara default. Ketika Anda membuat beberapa panggilan `query()` terhadap [session](/docs/id/agent-sdk/sessions) yang sama, filter pada `session.id` di backend Anda untuk melihatnya sebagai satu timeline. Atribut dihilangkan jika `OTEL_METRICS_INCLUDE_SESSION_ID` diatur ke falsy value.

<Note>
  Tracing dalam beta. Nama span dan attributes dapat berubah antar releases. Lihat
  [Traces (beta)](/docs/id/monitoring-usage#traces-beta) dalam referensi Monitoring
  untuk variabel konfigurasi trace exporter.
</Note>

<h2 id="link-traces-to-your-application">
  Hubungkan traces ke aplikasi Anda
</h2>

SDK secara otomatis menyebarkan W3C trace context ke CLI subprocess. Ketika Anda memanggil `query()` sementara OpenTelemetry span aktif di aplikasi Anda, SDK menyuntikkan `TRACEPARENT` dan `TRACESTATE` ke environment child process, dan CLI membacanya sehingga span `claude_code.interaction`-nya menjadi child dari span Anda. Agent run kemudian muncul di dalam trace aplikasi Anda alih-alih sebagai root yang terputus.

Ketika trace-context propagation diaktifkan, CLI juga meneruskan `TRACEPARENT` ke setiap command Bash dan PowerShell yang dijalankannya. Jika command yang diluncurkan melalui Bash tool memancarkan spans OpenTelemetry-nya sendiri, spans tersebut bersarang di bawah span `claude_code.tool.execution` yang membungkus command.

Auto-injection dilewati ketika Anda mengatur `TRACEPARENT` secara eksplisit dalam `options.env`, jadi Anda dapat menyematkan parent context spesifik jika diperlukan. Interactive CLI sessions mengabaikan inbound `TRACEPARENT` sepenuhnya; hanya Agent SDK dan `claude -p` runs yang menghormatinya. Lihat [Traces (beta)](/docs/id/monitoring-usage#traces-beta) dalam referensi Monitoring untuk referensi span dan attribute lengkap.

<h2 id="tag-telemetry-from-your-agent">
  Tag telemetry dari agent Anda
</h2>

Secara default, CLI melaporkan `service.name` sebagai `claude-code`. Jika Anda menjalankan beberapa agents, atau menjalankan SDK bersama services lain yang mengekspor ke collector yang sama, override service name dan tambahkan resource attributes sehingga Anda dapat memfilter berdasarkan agent di backend Anda.

Contoh berikut mengganti nama service dan melampirkan deployment metadata. Nilai-nilai ini diterapkan sebagai OpenTelemetry resource attributes pada setiap span, metric, dan event yang agent pancarkan:

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(
      env={
          # ... exporter configuration ...
          "OTEL_SERVICE_NAME": "support-triage-agent",
          "OTEL_RESOURCE_ATTRIBUTES": "service.version=1.4.0,deployment.environment=production",
      },
  )
  ```

  ```typescript TypeScript theme={null}
  const options = {
    env: {
      ...process.env,
      // ... exporter configuration ...
      OTEL_SERVICE_NAME: "support-triage-agent",
      OTEL_RESOURCE_ATTRIBUTES":
        "service.version=1.4.0,deployment.environment=production",
    },
  };
  ```
</CodeGroup>

<h2 id="attribute-actions-to-your-end-users">
  Atribusikan actions ke end users Anda
</h2>

CLI melampirkan [identity attributes](/docs/id/monitoring-usage#standard-attributes) ke setiap event berdasarkan credential yang digunakan untuk memanggil Anthropic. Ketika Anda membangun aplikasi yang melayani banyak end users dari satu deployment, attributes ini mengidentifikasi credential service Anda, bukan end user atas nama siapa agent bertindak.

Untuk membuat tool calls dan MCP activity dapat diatribusikan ke end users aplikasi Anda, suntikkan end-user identity sebagai resource attributes pada setiap panggilan `query()`. Percent-encode values sebelum menginterpolasikannya, karena `OTEL_RESOURCE_ATTRIBUTES` [reserves commas, spaces, dan equals signs](/docs/id/monitoring-usage#multi-team-organization-support). Contoh berikut melampirkan requesting user dan tenant ke setiap span dan event dari satu request. Ini mengasumsikan objek `request` dari web framework Anda yang membawa user dan tenant IDs:

<CodeGroup>
  ```python Python theme={null}
  from urllib.parse import quote

  options = ClaudeAgentOptions(
      env={
          # ... exporter configuration ...
          "OTEL_RESOURCE_ATTRIBUTES": f"enduser.id={quote(request.user_id)},tenant.id={quote(request.tenant_id)}",
      },
  )
  ```

  ```typescript TypeScript theme={null}
  const options = {
    env: {
      ...process.env,
      // ... exporter configuration ...
      OTEL_RESOURCE_ATTRIBUTES: `enduser.id=${encodeURIComponent(request.userId)},tenant.id=${encodeURIComponent(request.tenantId)}`,
    },
  };
  ```
</CodeGroup>

Dengan end-user identity terlampir, events `tool_decision`, `tool_result`, `mcp_server_connection`, dan `permission_mode_changed`, yang diekspor sebagai log records yang dinamai dengan prefix `claude_code.`, menjadi per-user audit trail yang dapat Anda teruskan ke platform Security Information and Event Management (SIEM). Lihat [Audit security events](/docs/id/monitoring-usage#audit-security-events) dalam referensi Monitoring untuk daftar lengkap security-relevant events dan attributes yang masing-masing membawa.

<h2 id="control-sensitive-data-in-exports">
  Kontrol sensitive data dalam exports
</h2>

Telemetry bersifat structural secara default. Durations, model names, dan tool names dicatat pada setiap span; token counts dicatat ketika underlying API request mengembalikan usage data, jadi spans untuk failed atau aborted requests dapat menghilangkannya. Konten yang dibaca dan ditulis agent Anda tidak dicatat secara default. Variables opt-in ini menambahkan konten ke exported data:

| Variable                  | Menambahkan                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| ------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `OTEL_LOG_USER_PROMPTS=1` | Prompt text pada events `claude_code.user_prompt` dan pada span `claude_code.interaction`                                                                                                                                                                                                                                                                                                                                                                                              |
| `OTEL_LOG_TOOL_DETAILS=1` | Tool input arguments (file paths, shell commands, search patterns) pada events `claude_code.tool_result`                                                                                                                                                                                                                                                                                                                                                                               |
| `OTEL_LOG_TOOL_CONTENT=1` | Full tool input dan output bodies sebagai span events pada `claude_code.tool`, truncated pada 60 KB. Memerlukan [tracing](#read-agent-traces) untuk diaktifkan                                                                                                                                                                                                                                                                                                                         |
| `OTEL_LOG_RAW_API_BODIES` | Full Anthropic Messages API request dan response JSON sebagai log events `claude_code.api_request_body` dan `claude_code.api_response_body`. Atur ke `1` untuk inline bodies truncated pada 60 KB, atau `file:<dir>` untuk untruncated bodies di disk dengan path `body_ref` dalam event. Bodies mencakup entire conversation history dan memiliki extended-thinking content redacted. Mengaktifkan ini menyiratkan consent ke semua yang akan diungkapkan oleh tiga variables di atas |

Biarkan unset kecuali pipeline observability Anda disetujui untuk menyimpan data yang ditangani agent Anda. Lihat [Security and privacy](/docs/id/monitoring-usage#security-and-privacy) dalam referensi Monitoring untuk daftar lengkap attributes dan redaction behavior.

<h2 id="related-documentation">
  Dokumentasi terkait
</h2>

Panduan-panduan ini mencakup topik-topik yang berdekatan untuk monitoring dan deploying agents:

* [Track cost and usage](/docs/id/agent-sdk/cost-tracking): baca token dan cost data dari message stream tanpa backend eksternal.
* [Hosting the Agent SDK](/docs/id/agent-sdk/hosting): deploy agents dalam containers di mana Anda dapat mengatur OpenTelemetry variables pada level environment.
* [Monitoring](/docs/id/monitoring-usage): referensi lengkap untuk setiap environment variable, metric, dan event yang CLI pancarkan.
