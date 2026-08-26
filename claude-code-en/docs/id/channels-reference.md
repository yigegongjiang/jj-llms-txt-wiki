> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Referensi Channels

> Bangun server MCP yang mendorong webhooks, alerts, dan pesan chat ke dalam sesi Claude Code. Referensi untuk kontrak channel: deklarasi kemampuan, event notifikasi, tools balasan, gating pengirim, dan relay izin.

<Note>
  Channels berada dalam [research preview](/docs/id/channels#research-preview). Organisasi Team dan Enterprise harus [secara eksplisit mengaktifkannya](/docs/id/channels#enterprise-controls).
</Note>

Sebuah channel adalah server MCP yang mendorong event ke dalam sesi Claude Code sehingga Claude dapat bereaksi terhadap hal-hal yang terjadi di luar terminal.

Anda dapat membangun channel satu arah atau dua arah. Channel satu arah meneruskan alerts, webhooks, atau event monitoring untuk Claude bertindak. Channel dua arah seperti chat bridges juga [mengekspos tool balasan](#expose-a-reply-tool) sehingga Claude dapat mengirim pesan kembali. Sebuah channel dengan jalur pengirim terpercaya juga dapat memilih untuk [relay permission prompts](#relay-permission-prompts) sehingga Anda dapat menyetujui atau menolak penggunaan tool dari jarak jauh.

Halaman ini mencakup:

* [Overview](#overview): bagaimana channels bekerja
* [Yang Anda butuhkan](#what-you-need): persyaratan dan langkah-langkah umum
* [Contoh: bangun penerima webhook](#example-build-a-webhook-receiver): panduan satu arah minimal
* [Server options](#server-options): field konstruktor
* [Notification format](#notification-format): payload event dan perilaku pengiriman
* [Expose a reply tool](#expose-a-reply-tool): biarkan Claude mengirim pesan kembali
* [Gate inbound messages](#gate-inbound-messages): pemeriksaan pengirim untuk mencegah prompt injection
* [Relay permission prompts](#relay-permission-prompts): teruskan permission prompts ke channels jarak jauh

Untuk menggunakan channel yang sudah ada daripada membangun satu, lihat [Channels](/docs/id/channels). Telegram, Discord, iMessage, dan fakechat disertakan dalam research preview.

<h2 id="overview">
  Overview
</h2>

Sebuah channel adalah server [MCP](https://modelcontextprotocol.io) yang berjalan di mesin yang sama dengan Claude Code. Claude Code menjalankannya sebagai subprocess dan berkomunikasi melalui stdio. Server channel Anda adalah jembatan antara sistem eksternal dan sesi Claude Code:

* **Chat platforms** (Telegram, Discord): plugin Anda berjalan secara lokal dan polling API platform untuk pesan baru. Ketika seseorang DM bot Anda, plugin menerima pesan dan meneruskannya ke Claude. Tidak ada URL untuk diekspos.
* **Webhooks** (CI, monitoring): server Anda mendengarkan pada port HTTP lokal. Sistem eksternal POST ke port tersebut, dan server Anda mendorong payload ke Claude.

<img src="https://mintcdn.com/claude-code/9FG0ZKj9uKYiHmbi/images/channel-architecture.svg?fit=max&auto=format&n=9FG0ZKj9uKYiHmbi&q=85&s=9a037b7da80184ae49015c0256b21a1f" alt="Diagram arsitektur menunjukkan sistem eksternal terhubung ke server channel lokal Anda, yang berkomunikasi dengan Claude Code melalui stdio" width="600" height="220" data-path="images/channel-architecture.svg" />

<h2 id="what-you-need">
  Yang Anda butuhkan
</h2>

Satu-satunya persyaratan keras adalah paket [`@modelcontextprotocol/sdk`](https://www.npmjs.com/package/@modelcontextprotocol/sdk) dan runtime yang kompatibel dengan Node.js. [Bun](https://bun.sh), [Node](https://nodejs.org), dan [Deno](https://deno.com) semuanya bekerja. Plugin pra-bangun dalam research preview menggunakan Bun, tetapi channel Anda tidak harus.

Server Anda perlu:

1. Mendeklarasikan kemampuan `claude/channel` sehingga Claude Code mendaftarkan pendengar notifikasi
2. Memancarkan event `notifications/claude/channel` ketika sesuatu terjadi
3. Terhubung melalui [stdio transport](https://modelcontextprotocol.io/docs/concepts/transports#standard-io) (Claude Code menjalankan server Anda sebagai subprocess)

Bagian [Server options](#server-options) dan [Notification format](#notification-format) mencakup masing-masing secara detail. Lihat [Contoh: bangun penerima webhook](#example-build-a-webhook-receiver) untuk panduan lengkap.

Selama research preview, custom channels tidak ada di [approved allowlist](/docs/id/channels#supported-channels). Gunakan `--dangerously-load-development-channels` untuk menguji secara lokal. Lihat [Test during the research preview](#test-during-the-research-preview) untuk detail.

<h2 id="example-build-a-webhook-receiver">
  Contoh: bangun penerima webhook
</h2>

Panduan ini membangun server file tunggal yang mendengarkan permintaan HTTP dan meneruskannya ke sesi Claude Code Anda. Pada akhirnya, apa pun yang dapat mengirim HTTP POST, seperti pipeline CI, alert monitoring, atau perintah `curl`, dapat mendorong event ke Claude.

Contoh ini menggunakan [Bun](https://bun.sh) sebagai runtime untuk server HTTP bawaan dan dukungan TypeScript. Anda dapat menggunakan [Node](https://nodejs.org) atau [Deno](https://deno.com) sebagai gantinya; satu-satunya persyaratan adalah [MCP SDK](https://www.npmjs.com/package/@modelcontextprotocol/sdk).

<Steps>
  <Step title="Buat proyek">
    Buat direktori baru dan instal MCP SDK:

    ```bash theme={null}
    mkdir webhook-channel && cd webhook-channel
    bun add @modelcontextprotocol/sdk
    ```
  </Step>

  <Step title="Tulis server channel">
    Buat file bernama `webhook.ts`. Ini adalah seluruh server channel Anda: terhubung ke Claude Code melalui stdio, dan mendengarkan POST HTTP pada port 8788. Ketika permintaan tiba, mendorong body ke Claude sebagai channel event.

    ```ts title="webhook.ts" theme={null}
    #!/usr/bin/env bun
    import { Server } from '@modelcontextprotocol/sdk/server/index.js'
    import { StdioServerTransport } from '@modelcontextprotocol/sdk/server/stdio.js'

    // Buat server MCP dan deklarasikan sebagai channel
    const mcp = new Server(
      { name: 'webhook', version: '0.0.1' },
      {
        // kunci ini adalah yang membuatnya menjadi channel — Claude Code mendaftarkan pendengar untuknya
        capabilities: { experimental: { 'claude/channel': {} } },
        // ditambahkan ke system prompt Claude sehingga tahu cara menangani event ini
        instructions: 'Events dari webhook channel tiba sebagai <channel source="webhook" ...>. Mereka satu arah: baca dan bertindak, tidak ada balasan yang diharapkan.',
      },
    )

    // Terhubung ke Claude Code melalui stdio (Claude Code menjalankan proses ini)
    await mcp.connect(new StdioServerTransport())

    // Mulai server HTTP yang meneruskan setiap POST ke Claude
    Bun.serve({
      port: 8788,  // port terbuka apa pun berfungsi
      // localhost-only: tidak ada yang di luar mesin ini dapat POST
      hostname: '127.0.0.1',
      async fetch(req) {
        const body = await req.text()
        await mcp.notification({
          method: 'notifications/claude/channel',
          params: {
            content: body,  // menjadi body dari tag <channel>
            // setiap kunci menjadi atribut tag, misalnya <channel path="/" method="POST">
            meta: { path: new URL(req.url).pathname, method: req.method },
          },
        })
        return new Response('ok')
      },
    })
    ```

    File melakukan tiga hal secara berurutan:

    * **Server configuration**: membuat server MCP dengan `claude/channel` dalam capabilities, yang memberi tahu Claude Code ini adalah channel. String [`instructions`](#server-options) masuk ke system prompt Claude: beri tahu Claude event apa yang diharapkan, apakah akan membalas, dan bagaimana cara merutekan balasan jika harus.
    * **Stdio connection**: terhubung ke Claude Code melalui stdin/stdout. Ini standar untuk server [MCP](https://modelcontextprotocol.io/docs/concepts/transports#standard-io) apa pun: Claude Code menjalankannya sebagai subprocess.
    * **HTTP listener**: memulai server web lokal pada port 8788. Setiap body POST diteruskan ke Claude sebagai channel event melalui `mcp.notification()`. `content` menjadi body event, dan setiap entry `meta` menjadi atribut pada tag `<channel>`. Pendengar memerlukan akses ke instance `mcp`, jadi berjalan dalam proses yang sama. Anda dapat membaginya menjadi modul terpisah untuk proyek yang lebih besar.
  </Step>

  <Step title="Daftarkan server Anda dengan Claude Code">
    Tambahkan server ke konfigurasi MCP Anda sehingga Claude Code tahu cara memulainya. Untuk `.mcp.json` tingkat proyek di direktori yang sama, gunakan jalur relatif. Untuk konfigurasi tingkat pengguna di `~/.claude.json`, gunakan jalur absolut lengkap sehingga server dapat ditemukan dari proyek apa pun:

    ```json title=".mcp.json" theme={null}
    {
      "mcpServers": {
        "webhook": { "command": "bun", "args": ["./webhook.ts"] }
      }
    }
    ```

    Claude Code membaca konfigurasi MCP Anda saat startup dan menjalankan setiap server sebagai subprocess.
  </Step>

  <Step title="Uji">
    Selama research preview, custom channels tidak ada di allowlist, jadi mulai Claude Code dengan flag pengembangan:

    ```bash theme={null}
    claude --dangerously-load-development-channels server:webhook
    ```

    Kali pertama Anda memulai sesi dalam proyek ini, Claude Code meminta persetujuan sebelum menggunakan server baru dari `.mcp.json`. Dialog melaporkan "New MCP server found in this project: webhook". Pilih **Use this MCP server** untuk melanjutkan.

    Ketika Claude Code dimulai, membaca konfigurasi MCP Anda, menjalankan `webhook.ts` Anda sebagai subprocess, dan pendengar HTTP dimulai secara otomatis pada port yang Anda konfigurasi (8788 dalam contoh ini). Anda tidak perlu menjalankan server sendiri.

    Pemberitahuan redup di bawah banner startup mengonfirmasi channel terdaftar: `Channels (experimental) messages from server:webhook inject directly in this session · restart without --dangerously-load-development-channels to stop`.

    Jika Anda melihat "blocked by org policy," admin organisasi Anda perlu [mengaktifkan channels](/docs/id/channels#enterprise-controls) terlebih dahulu.

    Di terminal terpisah, simulasikan webhook dengan mengirim HTTP POST dengan pesan ke server Anda. Contoh ini mengirim alert kegagalan CI ke port 8788 (atau port apa pun yang Anda konfigurasi):

    ```bash theme={null}
    curl -X POST localhost:8788 -d "build failed on main: https://ci.example.com/run/1234"
    ```

    Payload tiba di sesi Claude Code Anda sebagai tag `<channel>`:

    ```text theme={null}
    <channel source="webhook" path="/" method="POST">build failed on main: https://ci.example.com/run/1234</channel>
    ```

    Di terminal Claude Code Anda, Anda akan melihat Claude menerima pesan dan mulai merespons: membaca file, menjalankan perintah, atau apa pun yang diminta pesan. Ini adalah channel satu arah, jadi Claude bertindak dalam sesi Anda tetapi tidak mengirim apa pun kembali melalui webhook. Untuk menambahkan balasan, lihat [Expose a reply tool](#expose-a-reply-tool).

    Jika event tidak tiba, diagnosis tergantung pada apa yang dikembalikan `curl`:

    * **`curl` berhasil tetapi tidak ada yang mencapai Claude**: jalankan `/mcp` dalam sesi Anda untuk memeriksa status server. "Failed to connect" biasanya berarti kesalahan dependensi atau impor dalam file server Anda; periksa debug log di `~/.claude/debug/<session-id>.txt` untuk jejak stderr.
    * **`curl` gagal dengan "connection refused"**: port tidak terikat lagi atau proses basi dari run sebelumnya memegangnya. `lsof -i :<port>` menunjukkan apa yang mendengarkan; `kill` proses basi sebelum memulai ulang sesi Anda.
  </Step>
</Steps>

[Server fakechat](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins/fakechat) memperluas pola ini dengan UI web, lampiran file, dan tool balasan untuk chat dua arah.

<h2 id="test-during-the-research-preview">
  Uji selama research preview
</h2>

Selama research preview, setiap channel harus ada di [approved allowlist](/docs/id/channels#research-preview) untuk mendaftar. Flag pengembangan melewati allowlist untuk entri spesifik setelah prompt konfirmasi. Contoh ini menunjukkan kedua tipe entri:

```bash theme={null}
# Menguji plugin yang sedang Anda kembangkan
claude --dangerously-load-development-channels plugin:yourplugin@yourmarketplace

# Menguji server .mcp.json telanjang (belum ada wrapper plugin)
claude --dangerously-load-development-channels server:webhook
```

Bypass per-entri. Menggabungkan flag ini dengan `--channels` tidak memperluas bypass ke entri `--channels`. Selama research preview, approved allowlist dikurasi oleh Anthropic, jadi channel Anda tetap pada flag pengembangan saat Anda membangun dan menguji.

<Note>
  Flag ini melewati allowlist saja. Kebijakan organisasi `channelsEnabled` masih berlaku. Jangan gunakan untuk menjalankan channels dari sumber yang tidak terpercaya.
</Note>

<h2 id="server-options">
  Server options
</h2>

Sebuah channel menetapkan opsi ini dalam konstruktor [`Server`](https://modelcontextprotocol.io/docs/concepts/servers). Field `instructions` dan `capabilities.tools` adalah [MCP standar](https://modelcontextprotocol.io/docs/concepts/servers); `capabilities.experimental['claude/channel']` dan `capabilities.experimental['claude/channel/permission']` adalah penambahan spesifik channel:

| Field                                                    | Type     | Description                                                                                                                                                                                                                                                                                                          |
| :------------------------------------------------------- | :------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `capabilities.experimental['claude/channel']`            | `object` | Diperlukan. Selalu `{}`. Kehadiran mendaftarkan pendengar notifikasi.                                                                                                                                                                                                                                                |
| `capabilities.experimental['claude/channel/permission']` | `object` | Opsional. Selalu `{}`. Mendeklarasikan bahwa channel ini dapat menerima permission relay requests. Ketika dideklarasikan, Claude Code meneruskan permission prompts ke channel Anda sehingga Anda dapat menyetujui atau menolak mereka dari jarak jauh. Lihat [Relay permission prompts](#relay-permission-prompts). |
| `capabilities.tools`                                     | `object` | Dua arah saja. Selalu `{}`. Kemampuan tool MCP standar. Lihat [Expose a reply tool](#expose-a-reply-tool).                                                                                                                                                                                                           |
| `instructions`                                           | `string` | Direkomendasikan. Ditambahkan ke system prompt Claude. Beri tahu Claude event apa yang diharapkan, apa arti atribut tag `<channel>`, apakah akan membalas, dan jika demikian tool mana yang digunakan dan atribut mana yang diteruskan kembali (seperti `chat_id`).                                                  |

Untuk membuat channel satu arah, hilangkan `capabilities.tools`. Contoh ini menunjukkan setup dua arah dengan kemampuan channel, tools, dan instructions yang ditetapkan:

```ts theme={null}
import { Server } from '@modelcontextprotocol/sdk/server/index.js'

const mcp = new Server(
  { name: 'your-channel', version: '0.0.1' },
  {
    capabilities: {
      experimental: { 'claude/channel': {} },  // mendaftarkan pendengar channel
      tools: {},  // hilangkan untuk channels satu arah
    },
    // ditambahkan ke system prompt Claude sehingga tahu cara menangani event Anda
    instructions: 'Messages tiba sebagai <channel source="your-channel" ...>. Balas dengan tool balasan.',
  },
)
```

Untuk mendorong event, panggil `mcp.notification()` dengan method `notifications/claude/channel`. Params ada di bagian berikutnya.

<h2 id="notification-format">
  Notification format
</h2>

Server Anda memancarkan `notifications/claude/channel` dengan dua params:

| Field     | Type                     | Description                                                                                                                                                                                                                                                                |
| :-------- | :----------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `content` | `string`                 | Body event. Dikirimkan sebagai body dari tag `<channel>`.                                                                                                                                                                                                                  |
| `meta`    | `Record<string, string>` | Opsional. Setiap entry menjadi atribut pada tag `<channel>` untuk konteks routing seperti chat ID, nama pengirim, atau severity alert. Kunci harus identifier: huruf, digit, dan underscore saja. Kunci yang berisi hyphen atau karakter lain secara diam-diam dijatuhkan. |

Server Anda mendorong event dengan memanggil `mcp.notification()` pada instance `Server`. Contoh ini mendorong alert kegagalan CI dengan dua kunci meta:

```ts theme={null}
await mcp.notification({
  method: 'notifications/claude/channel',
  params: {
    content: 'build failed on main: https://ci.example.com/run/1234',
    meta: { severity: 'high', run_id: '1234' },
  },
})
```

Event tiba dalam konteks Claude dibungkus dalam tag `<channel>`. Atribut `source` ditetapkan secara otomatis dari nama server yang dikonfigurasi:

```text theme={null}
<channel source="your-channel" severity="high" run_id="1234">
build failed on main: https://ci.example.com/run/1234
</channel>
```

Notifikasi tidak diakui. `await` pada `mcp.notification()` diselesaikan ketika pesan ditulis ke transport, bukan ketika Claude telah memprosesnya. Jika sesi belum memuat server Anda sebagai channel, atau kebijakan organisasi membloknya, event dijatuhkan diam-diam tanpa error dikembalikan ke server Anda.

Jika Anda memerlukan konfirmasi pengiriman, lacak state event dalam server Anda dan ekspos [reply tool](#expose-a-reply-tool) yang dapat dipanggil Claude untuk melaporkan status kembali.

Event antri ke dalam sesi dan diproses secara berurutan. Jika beberapa notifikasi tiba saat Claude sibuk, mereka dikirimkan bersama pada giliran berikutnya dan Claude menangani mereka sebagai grup. Untuk memproses aliran event independen secara bersamaan, jalankan sesi terpisah.

<h2 id="expose-a-reply-tool">
  Expose a reply tool
</h2>

Jika channel Anda dua arah, seperti chat bridge daripada alert forwarder, ekspos [tool MCP](https://modelcontextprotocol.io/docs/concepts/tools) standar yang dapat dipanggil Claude untuk mengirim pesan kembali. Tidak ada yang spesifik channel tentang registrasi tool. Tool balasan memiliki tiga komponen:

1. Entry `tools: {}` dalam capabilities konstruktor `Server` Anda sehingga Claude Code menemukan tool
2. Tool handlers yang mendefinisikan schema tool dan mengimplementasikan logika pengiriman
3. String `instructions` dalam konstruktor `Server` Anda yang memberi tahu Claude kapan dan bagaimana memanggil tool

Untuk menambahkan ini ke [penerima webhook di atas](#example-build-a-webhook-receiver):

<Steps>
  <Step title="Aktifkan penemuan tool">
    Dalam konstruktor `Server` Anda di `webhook.ts`, tambahkan `tools: {}` ke capabilities sehingga Claude Code tahu server Anda menawarkan tools:

    ```ts theme={null}
    capabilities: {
      experimental: { 'claude/channel': {} },
      tools: {},  // mengaktifkan penemuan tool
    },
    ```
  </Step>

  <Step title="Daftarkan tool balasan">
    Tambahkan berikut ke `webhook.ts`. `import` masuk di bagian atas file dengan import lainnya; dua handlers masuk antara konstruktor `Server` dan `mcp.connect()`. Ini mendaftarkan tool `reply` yang dapat dipanggil Claude dengan `chat_id` dan `text`:

    ```ts theme={null}
    // Tambahkan import ini di bagian atas webhook.ts
    import { ListToolsRequestSchema, CallToolRequestSchema } from '@modelcontextprotocol/sdk/types.js'

    // Claude menanyakan ini saat startup untuk menemukan tool apa yang ditawarkan server Anda
    mcp.setRequestHandler(ListToolsRequestSchema, async () => ({
      tools: [{
        name: 'reply',
        description: 'Kirim pesan kembali melalui channel ini',
        // inputSchema memberi tahu Claude argumen apa yang harus diteruskan
        inputSchema: {
          type: 'object',
          properties: {
            chat_id: { type: 'string', description: 'Percakapan untuk membalas' },
            text: { type: 'string', description: 'Pesan untuk dikirim' },
          },
          required: ['chat_id', 'text'],
        },
      }],
    }))

    // Claude memanggil ini ketika ingin menjalankan tool
    mcp.setRequestHandler(CallToolRequestSchema, async req => {
      if (req.params.name === 'reply') {
        const { chat_id, text } = req.params.arguments as { chat_id: string; text: string }
        // send() adalah outbound Anda: POST ke platform chat Anda, atau untuk pengujian lokal
        // broadcast SSE yang ditunjukkan dalam contoh lengkap di bawah.
        send(`Reply to ${chat_id}: ${text}`)
        return { content: [{ type: 'text', text: 'sent' }] }
      }
      throw new Error(`unknown tool: ${req.params.name}`)
    })
    ```
  </Step>

  <Step title="Perbarui instructions">
    Perbarui string `instructions` dalam konstruktor `Server` Anda sehingga Claude tahu merutekan balasan kembali melalui tool. Contoh ini memberi tahu Claude untuk melewatkan `chat_id` dari tag inbound:

    ```ts theme={null}
    instructions: 'Messages tiba sebagai <channel source="webhook" chat_id="...">. Balas dengan tool balasan, melewatkan chat_id dari tag.'
    ```
  </Step>
</Steps>

Berikut adalah `webhook.ts` lengkap dengan dukungan dua arah. Balasan outbound mengalir melalui `GET /events` menggunakan [Server-Sent Events](https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events) (SSE), jadi `curl -N localhost:8788/events` dapat menontonnya secara langsung; chat inbound tiba di `POST /`:

```ts title="Full webhook.ts with reply tool' expandable theme={null}
#!/usr/bin/env bun
import { Server } from '@modelcontextprotocol/sdk/server/index.js'
import { StdioServerTransport } from '@modelcontextprotocol/sdk/server/stdio.js'
import { ListToolsRequestSchema, CallToolRequestSchema } from '@modelcontextprotocol/sdk/types.js'

// --- Outbound: tulis ke pendengar curl -N apa pun di /events ---
// Bridge nyata akan POST ke platform chat Anda sebagai gantinya.
const listeners = new Set<(chunk: string) => void>()
function send(text: string) {
  const chunk = text.split('\n').map(l => `data: ${l}\n`).join('') + '\n'
  for (const emit of listeners) emit(chunk)
}

const mcp = new Server(
  { name: 'webhook', version: '0.0.1' },
  {
    capabilities: {
      experimental: { 'claude/channel': {} },
      tools: {},
    },
    instructions: 'Messages tiba sebagai <channel source="webhook" chat_id="...">. Balas dengan tool balasan, melewatkan chat_id dari tag.',
  },
)

mcp.setRequestHandler(ListToolsRequestSchema, async () => ({
  tools: [{
    name: 'reply',
    description: 'Kirim pesan kembali melalui channel ini',
    inputSchema: {
      type: 'object',
      properties: {
        chat_id: { type: 'string', description: 'Percakapan untuk membalas' },
        text: { type: 'string', description: 'Pesan untuk dikirim' },
      },
      required: ['chat_id', 'text'],
    },
  }],
}))

mcp.setRequestHandler(CallToolRequestSchema, async req => {
  if (req.params.name === 'reply') {
    const { chat_id, text } = req.params.arguments as { chat_id: string; text: string }
    send(`Reply to ${chat_id}: ${text}`)
    return { content: [{ type: 'text', text: 'sent' }] }
  }
  throw new Error(`unknown tool: ${req.params.name}`)
})

await mcp.connect(new StdioServerTransport())

let nextId = 1
Bun.serve({
  port: 8788,
  hostname: '127.0.0.1',
  idleTimeout: 0,  // jangan tutup aliran SSE idle
  async fetch(req) {
    const url = new URL(req.url)

    // GET /events: aliran SSE sehingga curl -N dapat menonton balasan Claude secara langsung
    if (req.method === 'GET' && url.pathname === '/events') {
      const stream = new ReadableStream({
        start(ctrl) {
          ctrl.enqueue(': connected\n\n')  // sehingga curl menunjukkan sesuatu segera
          const emit = (chunk: string) => ctrl.enqueue(chunk)
          listeners.add(emit)
          req.signal.addEventListener('abort', () => listeners.delete(emit))
        },
      })
      return new Response(stream, {
        headers: { 'Content-Type': 'text/event-stream', 'Cache-Control': 'no-cache' },
      })
    }

    // POST: teruskan ke Claude sebagai channel event
    const body = await req.text()
    const chat_id = String(nextId++)
    await mcp.notification({
      method: 'notifications/claude/channel',
      params: {
        content: body,
        meta: { chat_id, path: url.pathname, method: req.method },
      },
    })
    return new Response('ok')
  },
})
```

[Server fakechat](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins/fakechat) menunjukkan contoh yang lebih lengkap dengan lampiran file dan pengeditan pesan.

<h2 id="gate-inbound-messages">
  Gate inbound messages
</h2>

Channel tanpa gate adalah vektor prompt injection. Siapa pun yang dapat menjangkau endpoint Anda dapat menempatkan teks di depan Claude. Channel yang mendengarkan platform chat atau endpoint publik memerlukan pemeriksaan pengirim nyata sebelum memancarkan apa pun.

Periksa pengirim terhadap allowlist sebelum memanggil `mcp.notification()`. Contoh ini menjatuhkan pesan apa pun dari pengirim yang tidak ada dalam set:

```ts theme={null}
const allowed = new Set(loadAllowlist())  // dari access.json Anda atau setara

// di dalam handler pesan Anda, sebelum memancarkan:
if (!allowed.has(message.from.id)) {  // pengirim, bukan ruangan
  return  // jatuhkan diam-diam
}
await mcp.notification({ ... })
```

Gate pada identitas pengirim, bukan identitas chat atau ruangan: `message.from.id` dalam contoh, bukan `message.chat.id`. Dalam chat grup, ini berbeda, dan gating pada ruangan akan membiarkan siapa pun dalam grup yang diizinkan menyuntikkan pesan ke dalam sesi.

Channel [Telegram](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins/telegram) dan [Discord](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins/discord) gate pada allowlist pengirim dengan cara yang sama. Mereka bootstrap list dengan pairing: pengguna DM bot, bot membalas dengan kode pairing, pengguna menyetujuinya dalam sesi Claude Code mereka, dan ID platform mereka ditambahkan. Lihat implementasi mana pun untuk alur pairing lengkap. Channel [iMessage](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins/imessage) mengambil pendekatan berbeda: mendeteksi alamat pengguna sendiri dari database Messages saat startup dan membiarkan mereka melewati secara otomatis, dengan pengirim lain ditambahkan oleh handle.

<h2 id="relay-permission-prompts">
  Relay permission prompts
</h2>

Ketika Claude memanggil tool yang memerlukan persetujuan, dialog terminal lokal terbuka dan sesi menunggu. Channel dua arah dapat memilih untuk menerima prompt yang sama secara paralel dan meneruskannya kepada Anda di perangkat lain. Keduanya tetap aktif: Anda dapat menjawab di terminal atau di ponsel, dan Claude Code menerapkan jawaban mana pun yang tiba terlebih dahulu dan menutup yang lain.

Relay mencakup persetujuan penggunaan tool seperti `Bash`, `Write`, dan `Edit`. Dialog kepercayaan proyek dan persetujuan server MCP tidak relay; hanya muncul di terminal lokal.

<h3 id="how-relay-works">
  Bagaimana relay bekerja
</h3>

Ketika permission prompt terbuka, loop relay memiliki empat langkah:

1. Claude Code menghasilkan ID permintaan pendek dan memberi tahu server Anda
2. Server Anda meneruskan prompt dan ID ke aplikasi chat Anda
3. Pengguna jarak jauh membalas dengan ya atau tidak dan ID itu
4. Handler inbound Anda menguraikan balasan menjadi verdict, dan Claude Code menerapkannya hanya jika ID cocok dengan permintaan terbuka

Dialog terminal lokal tetap terbuka melalui semua ini. Jika seseorang di terminal menjawab sebelum verdict jarak jauh tiba, jawaban itu diterapkan sebagai gantinya dan permintaan jarak jauh yang tertunda dijatuhkan.

<img src="https://mintcdn.com/claude-code/9FG0ZKj9uKYiHmbi/images/channel-permission-relay.svg?fit=max&auto=format&n=9FG0ZKj9uKYiHmbi&q=85&s=97d57f128f0da55f105ab1e3a7e10240" alt="Diagram urutan: Claude Code mengirim notifikasi permission_request ke server channel, server memformat dan mengirim prompt ke aplikasi chat, manusia membalas dengan verdict, dan server menguraikan balasan itu menjadi notifikasi permission kembali ke Claude Code" width="600" height="230" data-path="images/channel-permission-relay.svg" />

<h3 id="permission-request-fields">
  Permission request fields
</h3>

Notifikasi outbound dari Claude Code adalah `notifications/claude/channel/permission_request`. Seperti [channel notification](#notification-format), transport adalah MCP standar tetapi method dan schema adalah ekstensi Claude Code. Objek `params` memiliki empat field string yang server Anda format ke dalam prompt outgoing:

| Field           | Description                                                                                                                                                                                                                                                                                                                                                                                          |
| --------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `request_id`    | Lima huruf kecil diambil dari `a`-`z` tanpa `l`, jadi tidak pernah dibaca sebagai `1` atau `I` ketika diketik di ponsel. Sertakan dalam prompt outgoing Anda sehingga dapat diulang dalam balasan. Claude Code hanya menerima verdict yang membawa ID yang dikeluarkannya. Dialog terminal lokal tidak menampilkan ID ini, jadi handler outbound Anda adalah satu-satunya cara untuk mempelajarinya. |
| `tool_name`     | Nama tool yang ingin dipanggil Claude, misalnya `Bash` atau `Write`.                                                                                                                                                                                                                                                                                                                                 |
| `description`   | Ringkasan yang dapat dibaca manusia tentang apa yang dilakukan panggilan tool spesifik ini, teks yang sama yang ditampilkan dialog terminal lokal. Untuk panggilan Bash ini adalah deskripsi Claude tentang perintah, atau perintah itu sendiri jika tidak ada yang diberikan.                                                                                                                       |
| `input_preview` | Argumen tool sebagai string JSON, dipotong menjadi 200 karakter. Untuk Bash ini adalah perintah; untuk Write ini adalah jalur file dan awalan konten. Hilangkan dari prompt Anda jika Anda hanya memiliki ruang untuk pesan satu baris. Server Anda memutuskan apa yang akan ditampilkan.                                                                                                            |

Verdict yang dikirim server Anda kembali adalah `notifications/claude/channel/permission` dengan dua field: `request_id` mengulangi ID di atas, dan `behavior` diatur ke `'allow'` atau `'deny'`. Allow membiarkan panggilan tool melanjutkan; deny menolaknya, sama seperti menjawab No dalam dialog lokal. Tidak ada verdict yang mempengaruhi panggilan masa depan.

<h3 id="add-relay-to-a-chat-bridge">
  Tambahkan relay ke chat bridge
</h3>

Menambahkan permission relay ke channel dua arah memerlukan tiga komponen:

1. Entry `claude/channel/permission: {}` di bawah capabilities `experimental` dalam konstruktor `Server` Anda sehingga Claude Code tahu untuk meneruskan prompts
2. Notification handler untuk `notifications/claude/channel/permission_request` yang memformat prompt dan mengirimnya melalui API platform Anda
3. Pemeriksaan dalam handler pesan inbound Anda yang mengenali `yes <id>` atau `no <id>` dan memancarkan notifikasi `notifications/claude/channel/permission` verdict sebagai gantinya dari meneruskan teks ke Claude

Hanya deklarasikan kemampuan jika channel Anda [mengautentikasi pengirim](#gate-inbound-messages), karena siapa pun yang dapat membalas melalui channel Anda dapat menyetujui atau menolak penggunaan tool dalam sesi Anda.

Untuk menambahkan ini ke chat bridge dua arah seperti yang dirakit dalam [Expose a reply tool](#expose-a-reply-tool):

<Steps>
  <Step title="Deklarasikan kemampuan permission">
    Dalam konstruktor `Server` Anda, tambahkan `claude/channel/permission: {}` bersama `claude/channel` di bawah `experimental`:

    ```ts theme={null}
    capabilities: {
      experimental: {
        'claude/channel': {},
        'claude/channel/permission': {},  // opt in ke permission relay
      },
      tools: {},
    },
    ```
  </Step>

  <Step title="Tangani permintaan masuk">
    Daftarkan notification handler antara konstruktor `Server` Anda dan `mcp.connect()`. Claude Code memanggilnya dengan [empat request fields](#permission-request-fields) ketika dialog permission terbuka. Handler Anda memformat prompt untuk platform Anda dan menyertakan instruksi untuk membalas dengan ID:

    ```ts theme={null}
    import { z } from 'zod'

    // setNotificationHandler merutekan oleh z.literal pada field method,
    // jadi schema ini adalah validator dan kunci dispatch
    const PermissionRequestSchema = z.object({
      method: z.literal('notifications/claude/channel/permission_request'),
      params: z.object({
        request_id: z.string(),     // lima huruf kecil, sertakan verbatim dalam prompt Anda
        tool_name: z.string(),      // misalnya "Bash", "Write"
        description: z.string(),    // ringkasan yang dapat dibaca manusia tentang panggilan ini
        input_preview: z.string(),  // argumen tool sebagai JSON, dipotong menjadi ~200 karakter
      }),
    })

    mcp.setNotificationHandler(PermissionRequestSchema, async ({ params }) => {
      // send() adalah outbound Anda: POST ke platform chat Anda, atau untuk pengujian lokal
      // broadcast SSE yang ditunjukkan dalam contoh lengkap di bawah.
      send(
        `Claude ingin menjalankan ${params.tool_name}: ${params.description}\n\n` +
        // ID dalam instruksi adalah apa yang diuraikan handler inbound Anda dalam Langkah 3
        `Balas "yes ${params.request_id}" atau "no ${params.request_id}"`,
      )
    })
    ```
  </Step>

  <Step title="Intersep verdict dalam handler inbound Anda">
    Handler inbound Anda adalah loop atau callback yang menerima pesan dari platform Anda: tempat yang sama di mana Anda [gate pada pengirim](#gate-inbound-messages) dan memancarkan `notifications/claude/channel` untuk meneruskan chat ke Claude. Tambahkan pemeriksaan sebelum panggilan chat-forwarding yang mengenali format verdict dan memancarkan notifikasi permission sebagai gantinya.

    Regex cocok dengan format ID yang dihasilkan Claude Code: lima huruf, tidak pernah `l`. Flag `/i` mentoleransi autocorrect ponsel memanfaatkan balasan; hurufkan ID yang ditangkap sebelum mengirimnya kembali.

    ```ts theme={null}
    // cocok dengan "y abcde", "yes abcde", "n abcde", "no abcde"
    // [a-km-z] adalah alfabet ID yang digunakan Claude Code (huruf kecil, lewati 'l')
    // /i mentoleransi autocorrect ponsel; hurufkan capture sebelum mengirim
    const PERMISSION_REPLY_RE = /^\s*(y|yes|n|no)\s+([a-km-z]{5})\s*$/i

    async function onInbound(message: PlatformMessage) {
      if (!allowed.has(message.from.id)) return  // gate pada pengirim terlebih dahulu

      const m = PERMISSION_REPLY_RE.exec(message.text)
      if (m) {
        // m[1] adalah kata verdict, m[2] adalah ID permintaan
        // mancarkan notifikasi verdict kembali ke Claude Code daripada chat
        await mcp.notification({
          method: 'notifications/claude/channel/permission',
          params: {
            request_id: m[2].toLowerCase(),  // normalisasi jika ada autocorrect caps
            behavior: m[1].toLowerCase().startsWith('y') ? 'allow' : 'deny',
          },
        })
        return  // ditangani sebagai verdict, jangan juga teruskan sebagai chat
      }

      // tidak cocok format verdict: jatuh melalui ke jalur chat normal
      await mcp.notification({
        method: 'notifications/claude/channel',
        params: { content: message.text, meta: { chat_id: String(message.chat.id) } },
      })
    }
    ```
  </Step>
</Steps>

Claude Code juga menjaga dialog terminal lokal tetap terbuka, jadi Anda dapat menjawab di tempat mana pun, dan jawaban pertama yang tiba diterapkan. Balasan jarak jauh yang tidak cocok dengan format yang diharapkan dengan tepat gagal dalam salah satu dari dua cara, dan dalam kedua kasus dialog tetap terbuka:

* **Format berbeda**: regex handler inbound Anda gagal cocok, jadi teks seperti `approve it` atau `yes` tanpa ID jatuh melalui sebagai pesan normal ke Claude.
* **Format benar, ID salah**: server Anda memancarkan verdict, tetapi Claude Code tidak menemukan permintaan terbuka dengan ID itu dan menjatuhkannya diam-diam.

<h3 id="full-example">
  Contoh lengkap
</h3>

`webhook.ts` yang dirakit di bawah menggabungkan ketiga ekstensi dari halaman ini: tool balasan, sender gating, dan permission relay. Jika Anda memulai di sini, Anda juga akan memerlukan [project setup dan entry `.mcp.json`](#example-build-a-webhook-receiver) dari panduan awal.

Untuk membuat kedua arah dapat diuji dari curl, pendengar HTTP melayani dua jalur:

* **`GET /events`**: memegang aliran SSE terbuka dan mendorong setiap pesan outbound sebagai baris `data:`, jadi `curl -N` dapat menonton balasan Claude dan permission prompts tiba secara langsung.
* **`POST /`**: sisi inbound, handler yang sama seperti sebelumnya, sekarang dengan pemeriksaan format verdict disisipkan sebelum cabang chat-forward.

```ts title="Full webhook.ts with permission relay' expandable theme={null}
#!/usr/bin/env bun
import { Server } from '@modelcontextprotocol/sdk/server/index.js'
import { StdioServerTransport } from '@modelcontextprotocol/sdk/server/stdio.js'
import { ListToolsRequestSchema, CallToolRequestSchema } from '@modelcontextprotocol/sdk/types.js'
import { z } from 'zod'

// --- Outbound: tulis ke pendengar curl -N apa pun di /events ---
// Bridge nyata akan POST ke platform chat Anda sebagai gantinya.
const listeners = new Set<(chunk: string) => void>()
function send(text: string) {
  const chunk = text.split('\n').map(l => `data: ${l}\n`).join('') + '\n'
  for (const emit of listeners) emit(chunk)
}

// Allowlist pengirim. Untuk panduan lokal kami mempercayai nilai header X-Sender tunggal
// "dev"; bridge nyata akan memeriksa ID pengguna platform.
const allowed = new Set(['dev'])

const mcp = new Server(
  { name: 'webhook', version: '0.0.1' },
  {
    capabilities: {
      experimental: {
        'claude/channel': {},
        'claude/channel/permission': {},  // opt in ke permission relay
      },
      tools: {},
    },
    instructions:
      'Messages tiba sebagai <channel source="webhook" chat_id="...">. ' +
      'Balas dengan tool balasan, melewatkan chat_id dari tag.',
  },
)

// --- reply tool: Claude memanggil ini untuk mengirim pesan kembali ---
mcp.setRequestHandler(ListToolsRequestSchema, async () => ({
  tools: [{
    name: 'reply',
    description: 'Kirim pesan kembali melalui channel ini',
    inputSchema: {
      type: 'object',
      properties: {
        chat_id: { type: 'string', description: 'Percakapan untuk membalas' },
        text: { type: 'string', description: 'Pesan untuk dikirim' },
      },
      required: ['chat_id', 'text'],
    },
  }],
}))

mcp.setRequestHandler(CallToolRequestSchema, async req => {
  if (req.params.name === 'reply') {
    const { chat_id, text } = req.params.arguments as { chat_id: string; text: string }
    send(`Reply to ${chat_id}: ${text}`)
    return { content: [{ type: 'text', text: 'sent' }] }
  }
  throw new Error(`unknown tool: ${req.params.name}`)
})

// --- permission relay: Claude Code (bukan Claude) memanggil ini ketika dialog terbuka
const PermissionRequestSchema = z.object({
  method: z.literal('notifications/claude/channel/permission_request'),
  params: z.object({
    request_id: z.string(),
    tool_name: z.string(),
    description: z.string(),
    input_preview: z.string(),
  }),
})

mcp.setNotificationHandler(PermissionRequestSchema, async ({ params }) => {
  send(
    `Claude ingin menjalankan ${params.tool_name}: ${params.description}\n\n` +
    `Balas "yes ${params.request_id}" atau "no ${params.request_id}"`,
  )
})

await mcp.connect(new StdioServerTransport())

// --- HTTP pada :8788: GET /events mengalirkan outbound, POST merutekan inbound ---
const PERMISSION_REPLY_RE = /^\s*(y|yes|n|no)\s+([a-km-z]{5})\s*$/i
let nextId = 1

Bun.serve({
  port: 8788,
  hostname: '127.0.0.1',
  idleTimeout: 0,  // jangan tutup aliran SSE idle
  async fetch(req) {
    const url = new URL(req.url)

    // GET /events: aliran SSE sehingga curl -N dapat menonton balasan dan prompts secara langsung
    if (req.method === 'GET' && url.pathname === '/events') {
      const stream = new ReadableStream({
        start(ctrl) {
          ctrl.enqueue(': connected\n\n')  // sehingga curl menunjukkan sesuatu segera
          const emit = (chunk: string) => ctrl.enqueue(chunk)
          listeners.add(emit)
          req.signal.addEventListener('abort', () => listeners.delete(emit))
        },
      })
      return new Response(stream, {
        headers: { 'Content-Type': 'text/event-stream', 'Cache-Control': 'no-cache' },
      })
    }

    // semuanya lainnya adalah inbound: gate pada pengirim terlebih dahulu
    const body = await req.text()
    const sender = req.headers.get('X-Sender') ?? ''
    if (!allowed.has(sender)) return new Response('forbidden', { status: 403 })

    // periksa format verdict sebelum memperlakukan sebagai chat
    const m = PERMISSION_REPLY_RE.exec(body)
    if (m) {
      await mcp.notification({
        method: 'notifications/claude/channel/permission',
        params: {
          request_id: m[2].toLowerCase(),
          behavior: m[1].toLowerCase().startsWith('y') ? 'allow' : 'deny',
        },
      })
      return new Response('verdict recorded')
    }

    // chat normal: teruskan ke Claude sebagai channel event
    const chat_id = String(nextId++)
    await mcp.notification({
      method: 'notifications/claude/channel',
      params: { content: body, meta: { chat_id, path: url.pathname } },
    })
    return new Response('ok')
  },
})
```

Uji jalur verdict dalam tiga terminal. Yang pertama adalah sesi Claude Code Anda, dimulai dengan [development flag](#test-during-the-research-preview) sehingga menjalankan `webhook.ts`:

```bash theme={null}
claude --dangerously-load-development-channels server:webhook
```

Di yang kedua, alirkan sisi outbound sehingga Anda dapat melihat balasan Claude dan permission prompts apa pun saat mereka menyala:

```bash theme={null}
curl -N localhost:8788/events
```

Di yang ketiga, kirim pesan yang akan membuat Claude mencoba menjalankan perintah:

```bash theme={null}
curl -d "list the files in this directory" -H "X-Sender: dev" localhost:8788
```

Listing file adalah read-only, jadi Claude menjalankannya tanpa persetujuan. Dialog permission terbuka ketika Claude memanggil tool `reply` untuk mengirim jawabannya kembali. Dialog lokal terbuka di terminal Claude Code Anda, dan sebentar kemudian prompt untuk `mcp__webhook__reply` muncul dalam aliran `/events`, termasuk ID lima huruf. Setujui dari sisi jarak jauh:

```bash theme={null}
curl -d "yes <id>" -H "X-Sender: dev" localhost:8788
```

Dialog lokal menutup, tool `reply` berjalan, dan balasan Claude mendarat dalam aliran.

Tiga bagian spesifik channel dalam file ini:

* **Capabilities** dalam konstruktor `Server`: `claude/channel` mendaftarkan pendengar notifikasi, `claude/channel/permission` opt in ke permission relay, `tools` membiarkan Claude menemukan tool balasan.
* **Outbound paths**: handler tool `reply` adalah apa yang dipanggil Claude untuk respons percakapan; handler notifikasi `PermissionRequestSchema` adalah apa yang dipanggil Claude Code ketika dialog permission terbuka. Keduanya memanggil `send()` untuk broadcast melalui `/events`, tetapi dipicu oleh bagian berbeda dari sistem.
* **HTTP handler**: `GET /events` memegang aliran SSE terbuka sehingga curl dapat menonton outbound secara langsung; `POST` adalah inbound, gated pada header `X-Sender`. Body `yes <id>` atau `no <id>` masuk ke Claude Code sebagai notifikasi verdict dan tidak pernah mencapai Claude; apa pun lainnya diteruskan ke Claude sebagai channel event.

<h2 id="package-as-a-plugin">
  Paket sebagai plugin
</h2>

Untuk membuat channel Anda dapat diinstal dan dibagikan, bungkus dalam [plugin](/docs/id/plugins) dan publikasikan ke [marketplace](/docs/id/plugin-marketplaces). Pengguna menginstalnya dengan `/plugin install`, kemudian mengaktifkannya per sesi dengan `--channels plugin:<name>@<marketplace>`.

Channel yang dipublikasikan ke marketplace Anda sendiri masih memerlukan `--dangerously-load-development-channels` untuk berjalan, karena tidak ada di [approved allowlist](/docs/id/channels#supported-channels). Allowlist default adalah plugin channel di `claude-plugins-official`, yang dikurasi Anthropic sesuai kebijaksanaannya. [Formulir pengajuan in-app](/docs/id/plugins#submit-your-plugin-to-the-community-marketplace) menambahkan plugin ke community marketplace, yang tidak ada di allowlist channel.

Jika Anda bekerja dengan kontak partner Anthropic, hubungi mereka untuk mengoordinasikan listing official-marketplace. Pada rencana Team dan Enterprise, admin dapat sebagai gantinya menyertakan plugin Anda dalam daftar [`allowedChannelPlugins`](/docs/id/channels#restrict-which-channel-plugins-can-run) organisasi, yang menggantikan allowlist Anthropic default.

<h2 id="see-also">
  Lihat juga
</h2>

* [Channels](/docs/id/channels) untuk menginstal dan menggunakan Telegram, Discord, iMessage, atau demo fakechat, dan untuk mengaktifkan channels untuk Team atau Enterprise org
* [Working channel implementations](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins) untuk kode server lengkap dengan alur pairing, tools balasan, dan lampiran file
* [MCP](/docs/id/mcp) untuk protokol dasar yang diimplementasikan server channel
* [Plugins](/docs/id/plugins) untuk mengemas channel Anda sehingga pengguna dapat menginstalnya dengan `/plugin install`
