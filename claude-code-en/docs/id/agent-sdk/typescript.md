> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Agent SDK reference - TypeScript

> Referensi API lengkap untuk TypeScript Agent SDK, termasuk semua fungsi, tipe, dan antarmuka.

<script src="/docs/components/typescript-sdk-type-links.js" defer />

<h2 id="installation">
  Instalasi
</h2>

```bash theme={null}
npm install @anthropic-ai/claude-agent-sdk
```

<Note>
  SDK menggabungkan biner Claude Code asli untuk platform Anda sebagai dependensi opsional seperti `@anthropic-ai/claude-agent-sdk-darwin-arm64`. Anda tidak perlu menginstal Claude Code secara terpisah. Jika pengelola paket Anda melewatkan dependensi opsional, SDK melempar `Native CLI binary for <platform> not found`; setel [`pathToClaudeCodeExecutable`](#options) ke biner `claude` yang diinstal secara terpisah sebagai gantinya.
</Note>

<h3 id="compile-to-a-single-executable">
  Kompilasi ke executable tunggal
</h3>

Ketika Anda mengompilasi aplikasi Anda menjadi executable file tunggal dengan `bun build --compile`, SDK tidak dapat menyelesaikan biner CLI yang dibundel saat runtime. `require.resolve` tidak berfungsi di dalam filesystem virtual `$bunfs` executable yang dikompilasi, jadi SDK melempar `Native CLI binary for <platform> not found`.

Untuk mengatasi ini, sematkan biner platform sebagai aset file, ekstrak ke path nyata saat startup dengan `extractFromBunfs()`, dan teruskan path tersebut ke [`pathToClaudeCodeExecutable`](#options).

Helper `extractFromBunfs()` memerlukan `@anthropic-ai/claude-agent-sdk` v0.3.144 atau lebih baru. Contoh di bawah ini membangun untuk macOS pada Apple Silicon:

```typescript theme={null}
import binPath from "@anthropic-ai/claude-agent-sdk-darwin-arm64/claude" with { type: "file" };
import { extractFromBunfs } from "@anthropic-ai/claude-agent-sdk/extract";
import { query } from "@anthropic-ai/claude-agent-sdk";

const cliPath = extractFromBunfs(binPath);

for await (const message of query({
  prompt: "Hello",
  options: { pathToClaudeCodeExecutable: cliPath },
})) {
  console.log(message);
}
```

`extractFromBunfs()` menyalin biner yang disematkan keluar dari filesystem virtual executable yang dikompilasi ke direktori temp per-pengguna dan mengembalikan path nyata. Di luar executable yang dikompilasi, ia mengembalikan path input tidak berubah, jadi kode yang sama berjalan dalam pengembangan tanpa modifikasi.

Setiap executable yang dikompilasi menyematkan biner platform tunggal. Cocokkan paket platform dalam impor ke `--target` Anda:

* Untuk cross-compile, instal paket platform yang tidak cocok, misalnya `npm install @anthropic-ai/claude-agent-sdk-linux-x64 --force`.
* Di Windows, subpath biner adalah `claude.exe`, misalnya `@anthropic-ai/claude-agent-sdk-win32-x64/claude.exe`.

<h2 id="functions">
  Fungsi
</h2>

<h3 id="query">
  `query()`
</h3>

Fungsi utama untuk berinteraksi dengan Claude Code. Membuat generator asinkron yang melakukan streaming pesan saat tiba.

```typescript theme={null}
function query({
  prompt,
  options
}: {
  prompt: string | AsyncIterable<SDKUserMessage>;
  options?: Options;
}): Query;
```

<h4 id="parameters">
  Parameter
</h4>

| Parameter | Tipe                                                             | Deskripsi                                                            |
| :-------- | :--------------------------------------------------------------- | :------------------------------------------------------------------- |
| `prompt`  | `string \| AsyncIterable<`[`SDKUserMessage`](#sdkusermessage)`>` | Prompt input sebagai string atau async iterable untuk mode streaming |
| `options` | [`Options`](#options)                                            | Objek konfigurasi opsional (lihat tipe Options di bawah)             |

<h4 id="returns">
  Pengembalian
</h4>

Mengembalikan objek [`Query`](#query-object) yang memperluas `AsyncGenerator<`[`SDKMessage`](#sdkmessage)`, void>` dengan metode tambahan.

<h3 id="startup">
  `startup()`
</h3>

Pra-pemanasan subprocess CLI dengan menspawnya dan menyelesaikan handshake inisialisasi sebelum prompt tersedia. Handle [`WarmQuery`](#warmquery) yang dikembalikan menerima prompt nanti dan menulisnya ke proses yang sudah siap, sehingga panggilan `query()` pertama diselesaikan tanpa membayar biaya spawn dan inisialisasi subprocess secara inline.

```typescript theme={null}
function startup(params?: {
  options?: Options;
  initializeTimeoutMs?: number;
}): Promise<WarmQuery>;
```

<h4 id="parameters-1">
  Parameter
</h4>

| Parameter             | Tipe                  | Deskripsi                                                                                                                                                                    |
| :-------------------- | :-------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `options`             | [`Options`](#options) | Objek konfigurasi opsional. Sama dengan parameter `options` ke `query()`                                                                                                     |
| `initializeTimeoutMs` | `number`              | Waktu maksimum dalam milidetik untuk menunggu inisialisasi subprocess. Default ke `60000`. Jika inisialisasi tidak selesai tepat waktu, promise ditolak dengan error timeout |

<h4 id="returns-1">
  Pengembalian
</h4>

Mengembalikan `Promise<`[`WarmQuery`](#warmquery)`>` yang diselesaikan setelah subprocess telah dispawn dan menyelesaikan handshake inisialisasinya.

<h4 id="example">
  Contoh
</h4>

Panggil `startup()` lebih awal, misalnya saat boot aplikasi, kemudian panggil `.query()` pada handle yang dikembalikan setelah prompt siap. Ini memindahkan spawn subprocess dan inisialisasi keluar dari jalur kritis.

```typescript theme={null}
import { startup } from "@anthropic-ai/claude-agent-sdk";

// Bayar biaya startup di muka
const warm = await startup({ options: { maxTurns: 3 } });

// Nanti, ketika prompt siap, ini langsung
for await (const message of warm.query("What files are here?")) {
  console.log(message);
}
```

<h3 id="tool">
  `tool()`
</h3>

Membuat definisi tool MCP yang aman tipe untuk digunakan dengan server MCP SDK.

```typescript theme={null}
function tool<Schema extends AnyZodRawShape>(
  name: string,
  description: string,
  inputSchema: Schema,
  handler: (args: InferShape<Schema>, extra: unknown) => Promise<CallToolResult>,
  extras?: { annotations?: ToolAnnotations }
): SdkMcpToolDefinition<Schema>;
```

<h4 id="parameters-2">
  Parameter
</h4>

| Parameter     | Tipe                                                              | Deskripsi                                                                      |
| :------------ | :---------------------------------------------------------------- | :----------------------------------------------------------------------------- |
| `name`        | `string`                                                          | Nama tool                                                                      |
| `description` | `string`                                                          | Deskripsi tentang apa yang dilakukan tool                                      |
| `inputSchema` | `Schema extends AnyZodRawShape`                                   | Skema Zod yang mendefinisikan parameter input tool (mendukung Zod 3 dan Zod 4) |
| `handler`     | `(args, extra) => Promise<`[`CallToolResult`](#calltoolresult)`>` | Fungsi asinkron yang mengeksekusi logika tool                                  |
| `extras`      | `{ annotations?: `[`ToolAnnotations`](#toolannotations)` }`       | Anotasi tool MCP opsional yang memberikan petunjuk perilaku kepada klien       |

<h4 id="toolannotations">
  `ToolAnnotations`
</h4>

Dieksport ulang dari `@modelcontextprotocol/sdk/types.js`. Semua field adalah petunjuk opsional; klien tidak boleh mengandalkannya untuk keputusan keamanan.

| Field             | Tipe      | Default     | Deskripsi                                                                                                                                    |
| :---------------- | :-------- | :---------- | :------------------------------------------------------------------------------------------------------------------------------------------- |
| `title`           | `string`  | `undefined` | Judul yang dapat dibaca manusia untuk tool                                                                                                   |
| `readOnlyHint`    | `boolean` | `false`     | Jika `true`, tool tidak memodifikasi lingkungannya                                                                                           |
| `destructiveHint` | `boolean` | `true`      | Jika `true`, tool dapat melakukan pembaruan destruktif (hanya bermakna ketika `readOnlyHint` adalah `false`)                                 |
| `idempotentHint`  | `boolean` | `false`     | Jika `true`, panggilan berulang dengan argumen yang sama tidak memiliki efek tambahan (hanya bermakna ketika `readOnlyHint` adalah `false`)  |
| `openWorldHint`   | `boolean` | `true`      | Jika `true`, tool berinteraksi dengan entitas eksternal (misalnya, pencarian web). Jika `false`, domain tool ditutup (misalnya, tool memori) |

```typescript theme={null}
import { tool } from "@anthropic-ai/claude-agent-sdk";
import { z } from "zod";

const searchTool = tool(
  "search",
  "Search the web",
  { query: z.string() },
  async ({ query }) => {
    return { content: [{ type: "text", text: `Results for: ${query}` }] };
  },
  { annotations: { readOnlyHint: true, openWorldHint: true } }
);
```

<h3 id="createsdkmcpserver">
  `createSdkMcpServer()`
</h3>

Membuat instance server MCP yang berjalan dalam proses yang sama dengan aplikasi Anda.

```typescript theme={null}
function createSdkMcpServer(options: {
  name: string;
  version?: string;
  tools?: Array<SdkMcpToolDefinition<any>>;
}): McpSdkServerConfigWithInstance;
```

<h4 id="parameters-3">
  Parameter
</h4>

| Parameter         | Tipe                          | Deskripsi                                                |
| :---------------- | :---------------------------- | :------------------------------------------------------- |
| `options.name`    | `string`                      | Nama server MCP                                          |
| `options.version` | `string`                      | String versi opsional                                    |
| `options.tools`   | `Array<SdkMcpToolDefinition>` | Array definisi tool yang dibuat dengan [`tool()`](#tool) |

<h3 id="listsessions">
  `listSessions()`
</h3>

Menemukan dan membuat daftar sesi masa lalu dengan metadata ringan. Filter berdasarkan direktori proyek atau buat daftar sesi di semua proyek.

```typescript theme={null}
function listSessions(options?: ListSessionsOptions): Promise<SDKSessionInfo[]>;
```

<h4 id="parameters-4">
  Parameter
</h4>

| Parameter                  | Tipe      | Default     | Deskripsi                                                                                   |
| :------------------------- | :-------- | :---------- | :------------------------------------------------------------------------------------------ |
| `options.dir`              | `string`  | `undefined` | Direktori untuk membuat daftar sesi. Ketika dihilangkan, mengembalikan sesi di semua proyek |
| `options.limit`            | `number`  | `undefined` | Jumlah maksimum sesi yang akan dikembalikan                                                 |
| `options.includeWorktrees` | `boolean` | `true`      | Ketika `dir` berada di dalam repositori git, sertakan sesi dari semua jalur worktree        |

<h4 id="return-type-sdksessioninfo">
  Tipe pengembalian: `SDKSessionInfo`
</h4>

| Properti       | Tipe                  | Deskripsi                                                                             |
| :------------- | :-------------------- | :------------------------------------------------------------------------------------ |
| `sessionId`    | `string`              | Pengenal sesi unik (UUID)                                                             |
| `summary`      | `string`              | Judul tampilan: judul kustom, ringkasan yang dihasilkan otomatis, atau prompt pertama |
| `lastModified` | `number`              | Waktu modifikasi terakhir dalam milidetik sejak epoch                                 |
| `fileSize`     | `number \| undefined` | Ukuran file sesi dalam byte. Hanya diisi untuk penyimpanan JSONL lokal                |
| `customTitle`  | `string \| undefined` | Judul sesi yang ditetapkan pengguna (melalui `/rename`)                               |
| `firstPrompt`  | `string \| undefined` | Prompt pengguna bermakna pertama dalam sesi                                           |
| `gitBranch`    | `string \| undefined` | Cabang Git di akhir sesi                                                              |
| `cwd`          | `string \| undefined` | Direktori kerja untuk sesi                                                            |
| `tag`          | `string \| undefined` | Tag sesi yang ditetapkan pengguna (lihat [`tagSession()`](#tagsession))               |
| `createdAt`    | `number \| undefined` | Waktu pembuatan dalam milidetik sejak epoch, dari timestamp entri pertama             |

<h4 id="example-1">
  Contoh
</h4>

Cetak 10 sesi terbaru untuk proyek. Hasil diurutkan berdasarkan `lastModified` menurun, jadi item pertama adalah yang terbaru. Hilangkan `dir` untuk mencari di semua proyek.

```typescript theme={null}
import { listSessions } from "@anthropic-ai/claude-agent-sdk";

const sessions = await listSessions({ dir: "/path/to/project", limit: 10 });

for (const session of sessions) {
  console.log(`${session.summary} (${session.sessionId})`);
}
```

<h3 id="getsessionmessages">
  `getSessionMessages()`
</h3>

Membaca pesan pengguna dan asisten dari transkrip sesi masa lalu.

```typescript theme={null}
function getSessionMessages(
  sessionId: string,
  options?: GetSessionMessagesOptions
): Promise<SessionMessage[]>;
```

<h4 id="parameters-5">
  Parameter
</h4>

| Parameter        | Tipe     | Default     | Deskripsi                                                                          |
| :--------------- | :------- | :---------- | :--------------------------------------------------------------------------------- |
| `sessionId`      | `string` | required    | UUID sesi untuk dibaca (lihat `listSessions()`)                                    |
| `options.dir`    | `string` | `undefined` | Direktori proyek untuk menemukan sesi. Ketika dihilangkan, mencari di semua proyek |
| `options.limit`  | `number` | `undefined` | Jumlah maksimum pesan yang akan dikembalikan                                       |
| `options.offset` | `number` | `undefined` | Jumlah pesan yang akan dilewati dari awal                                          |

<h4 id="return-type-sessionmessage">
  Tipe pengembalian: `SessionMessage`
</h4>

| Properti             | Tipe                    | Deskripsi                                                                                                                                                                                                                                                        |
| :------------------- | :---------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`               | `"user" \| "assistant"` | Peran pesan                                                                                                                                                                                                                                                      |
| `uuid`               | `string`                | Pengenal pesan unik                                                                                                                                                                                                                                              |
| `session_id`         | `string`                | Sesi yang pesan ini milik                                                                                                                                                                                                                                        |
| `message`            | `unknown`               | Payload pesan mentah dari transkrip                                                                                                                                                                                                                              |
| `parent_tool_use_id` | `string \| null`        | Untuk pesan subagent, `tool_use_id` dari panggilan tool `Agent` yang memicu. `null` untuk pesan sesi utama dan sesi yang lebih lama                                                                                                                              |
| `parent_agent_id`    | `string \| null`        | Untuk pesan dari [subagent bersarang](/docs/id/sub-agents#spawn-nested-subagents), `agentId` dari subagent yang memicunya. `null` untuk pesan sesi utama, pesan dari subagent tingkat atas, dan sesi yang lebih lama. Memerlukan Claude Code v2.1.202 atau lebih baru |

<h4 id="example-2">
  Contoh
</h4>

```typescript theme={null}
import { listSessions, getSessionMessages } from "@anthropic-ai/claude-agent-sdk";

const [latest] = await listSessions({ dir: "/path/to/project", limit: 1 });

if (latest) {
  const messages = await getSessionMessages(latest.sessionId, {
    dir: "/path/to/project",
    limit: 20
  });

  for (const msg of messages) {
    console.log(`[${msg.type}] ${msg.uuid}`);
  }
}
```

<h3 id="getsessioninfo">
  `getSessionInfo()`
</h3>

Membaca metadata untuk sesi tunggal berdasarkan ID tanpa memindai direktori proyek lengkap.

```typescript theme={null}
function getSessionInfo(
  sessionId: string,
  options?: GetSessionInfoOptions
): Promise<SDKSessionInfo | undefined>;
```

<h4 id="parameters-6">
  Parameter
</h4>

| Parameter     | Tipe     | Default     | Deskripsi                                                                     |
| :------------ | :------- | :---------- | :---------------------------------------------------------------------------- |
| `sessionId`   | `string` | required    | UUID sesi yang akan dicari                                                    |
| `options.dir` | `string` | `undefined` | Jalur direktori proyek. Ketika dihilangkan, mencari di semua direktori proyek |

Mengembalikan [`SDKSessionInfo`](#return-type-sdksessioninfo), atau `undefined` jika sesi tidak ditemukan.

<h3 id="renamesession">
  `renameSession()`
</h3>

Mengganti nama sesi dengan menambahkan entri judul kustom. Panggilan berulang aman; judul terbaru menang.

```typescript theme={null}
function renameSession(
  sessionId: string,
  title: string,
  options?: SessionMutationOptions
): Promise<void>;
```

<h4 id="parameters-7">
  Parameter
</h4>

| Parameter     | Tipe     | Default     | Deskripsi                                                                     |
| :------------ | :------- | :---------- | :---------------------------------------------------------------------------- |
| `sessionId`   | `string` | required    | UUID sesi yang akan diganti nama                                              |
| `title`       | `string` | required    | Judul baru. Harus tidak kosong setelah memangkas spasi putih                  |
| `options.dir` | `string` | `undefined` | Jalur direktori proyek. Ketika dihilangkan, mencari di semua direktori proyek |

<h3 id="tagsession">
  `tagSession()`
</h3>

Menandai sesi. Lewatkan `null` untuk menghapus tag. Panggilan berulang aman; tag terbaru menang.

```typescript theme={null}
function tagSession(
  sessionId: string,
  tag: string | null,
  options?: SessionMutationOptions
): Promise<void>;
```

<h4 id="parameters-8">
  Parameter
</h4>

| Parameter     | Tipe             | Default     | Deskripsi                                                                     |
| :------------ | :--------------- | :---------- | :---------------------------------------------------------------------------- |
| `sessionId`   | `string`         | required    | UUID sesi yang akan ditandai                                                  |
| `tag`         | `string \| null` | required    | String tag, atau `null` untuk menghapus                                       |
| `options.dir` | `string`         | `undefined` | Jalur direktori proyek. Ketika dihilangkan, mencari di semua direktori proyek |

<h3 id="resolvesettings">
  `resolveSettings()`
</h3>

Menyelesaikan pengaturan Claude Code yang efektif untuk direktori tertentu menggunakan mesin penggabungan yang sama dengan CLI, tanpa menspawn CLI Claude. Gunakan untuk memeriksa konfigurasi apa yang akan dilihat oleh panggilan `query()` sebelum memanggil satu.

<Note>
  Fungsi ini alpha dan API-nya mungkin berubah sebelum stabilisasi. Fungsi ini membaca sumber MDM, termasuk plist macOS dan Windows HKLM/HKCU, untuk paritas dengan startup CLI, tetapi tidak mengeksekusi subprocess `policyHelper` yang dikonfigurasi admin. Field `permissions.defaultMode` dikembalikan apa adanya dari semua tingkat termasuk pengaturan proyek. Filter kepercayaan yang diterapkan CLI sebelum menghormati mode izin yang meningkat tidak diterapkan.
</Note>

```typescript theme={null}
function resolveSettings(
  options?: ResolveSettingsOptions
): Promise<ResolvedSettings>;
```

<h4 id="parameters-9">
  Parameter
</h4>

`resolveSettings()` menerima objek opsi tunggal. Semua field bersifat opsional.

| Parameter                       | Tipe                                  | Default         | Deskripsi                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| :------------------------------ | :------------------------------------ | :-------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `options.cwd`                   | `string`                              | `process.cwd()` | Direktori untuk menyelesaikan pengaturan proyek dan lokal relatif terhadap                                                                                                                                                                                                                                                                                                                                                                |
| `options.settingSources`        | [`SettingSource`](#settingsource)`[]` | Semua sumber    | Sumber filesystem mana yang akan dimuat. Lewatkan `[]` untuk melewati pengaturan pengguna, proyek, dan lokal. Pengaturan kebijakan terkelola dimuat dalam semua kasus. Pengaturan terkelola server diambil dari `serverManagedSettings` ketika host meneruskannya, atau dibaca dari cache on-disk CLI sebaliknya; snapshot tidak mengambilnya dari jaringan                                                                               |
| `options.managedSettings`       | `Settings`                            | `undefined`     | Pengaturan tingkat kebijakan pembatasan yang disediakan oleh host penyematan. Dijatuhkan secara default ketika tingkat terkelola yang diterapkan admin hadir; digabungkan di bawah tingkat itu ketika [`parentSettingsBehavior`](/docs/id/settings#available-settings) adalah `"merge"`. Kunci non-pembatasan seperti `model` secara diam-diam dijatuhkan sehingga opsi ini dapat memperketat kebijakan terkelola tetapi tidak melonggarkannya |
| `options.serverManagedSettings` | `Settings`                            | `undefined`     | Payload pengaturan terkelola server dari `/api/claude_code/settings`. Kunci non-pembatasan melewati tanpa filter                                                                                                                                                                                                                                                                                                                          |

<h4 id="return-type-resolvedsettings">
  Tipe pengembalian: `ResolvedSettings`
</h4>

`resolveSettings()` mengembalikan objek yang menjelaskan pengaturan yang digabungkan dan sumber yang berkontribusi pada setiap kunci.

| Properti     | Tipe                                                | Deskripsi                                                                                         |
| :----------- | :-------------------------------------------------- | :------------------------------------------------------------------------------------------------ |
| `effective`  | `Settings`                                          | Pengaturan yang digabungkan setelah menerapkan semua sumber yang diaktifkan dalam urutan preseden |
| `provenance` | `Partial<Record<keyof Settings, ProvenanceEntry>>`  | Untuk setiap kunci tingkat atas dalam `effective`, sumber mana yang memasok nilai                 |
| `sources`    | `Array<{ source, settings, path?, policyOrigin? }>` | Pengaturan mentah per-sumber, diurutkan dari preseden terendah hingga tertinggi                   |

<h4 id="example-3">
  Contoh
</h4>

Contoh di bawah ini menyelesaikan pengaturan untuk direktori proyek dan mencetak sumber yang mengontrol periode pembersihan.

```typescript theme={null}
import { resolveSettings } from "@anthropic-ai/claude-agent-sdk";

const { effective, provenance } = await resolveSettings({
  cwd: "/path/to/project",
  settingSources: ["user", "project", "local"],
});

console.log(`Cleanup period: ${effective.cleanupPeriodDays} days`);
console.log(`Set by: ${provenance.cleanupPeriodDays?.source}`);
```

<h2 id="types">
  Tipe
</h2>

<h3 id="options">
  `Options`
</h3>

Objek konfigurasi untuk fungsi `query()`.

| Properti                          | Tipe                                                                                                     | Default                                          | Deskripsi                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| :-------------------------------- | :------------------------------------------------------------------------------------------------------- | :----------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `abortController`                 | `AbortController`                                                                                        | `new AbortController()`                          | Pengontrol untuk membatalkan operasi                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `additionalDirectories`           | `string[]`                                                                                               | `[]`                                             | Direktori tambahan yang dapat diakses Claude                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `agent`                           | `string`                                                                                                 | `undefined`                                      | Nama agen untuk thread utama. Agen harus didefinisikan dalam opsi `agents` atau dalam pengaturan                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `agents`                          | `Record<string, [`AgentDefinition`](#agentdefinition)>`                                                  | `undefined`                                      | Tentukan subagen secara terprogram                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `agentProgressSummaries`          | `boolean`                                                                                                | `false`                                          | Ketika `true`, hasilkan ringkasan kemajuan satu baris untuk subagen dan teruskan pada event [`task_progress`](#sdktaskprogressmessage) melalui field `summary`. Berlaku untuk subagen foreground dan background                                                                                                                                                                                                                                                                                                                                                                                                               |
| `allowDangerouslySkipPermissions` | `boolean`                                                                                                | `false`                                          | Aktifkan bypass izin. Diperlukan saat menggunakan `permissionMode: 'bypassPermissions'`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `allowedTools`                    | `string[]`                                                                                               | `[]`                                             | Tool untuk auto-approve tanpa prompt. Ini tidak membatasi Claude hanya pada tool ini; tool yang tidak terdaftar jatuh ke `permissionMode` dan `canUseTool`. Gunakan `disallowedTools` untuk memblokir tool. Lihat [Izin](/docs/id/agent-sdk/permissions#allow-and-deny-rules)                                                                                                                                                                                                                                                                                                                                                      |
| `betas`                           | [`SdkBeta`](#sdkbeta)`[]`                                                                                | `[]`                                             | Aktifkan fitur beta                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `canUseTool`                      | [`CanUseTool`](#canusetool)                                                                              | `undefined`                                      | Fungsi izin kustom, dipanggil hanya ketika [alur izin](/docs/id/agent-sdk/permissions#how-permissions-are-evaluated) jatuh ke prompt. Tidak dipanggil untuk panggilan yang di-auto-approve oleh `allowedTools`, aturan allow, atau `permissionMode`. `AskUserQuestion`, tool konektor [organisasi Anda atur ke `ask`](/docs/id/mcp#organization-controls-on-connector-tools), dan tool MCP yang ditandai [`requiresUserInteraction`](/docs/id/mcp#require-approval-for-a-specific-tool) mencapainya bahkan jika Anda telah mengizinkannya; dalam mode `dontAsk` ini ditolak sebagai gantinya. Lihat [`CanUseTool`](#canusetool) untuk detail |
| `continue`                        | `boolean`                                                                                                | `false`                                          | Lanjutkan percakapan terbaru                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `cwd`                             | `string`                                                                                                 | `process.cwd()`                                  | Direktori kerja saat ini                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `debug`                           | `boolean`                                                                                                | `false`                                          | Aktifkan mode debug untuk proses Claude Code                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `debugFile`                       | `string`                                                                                                 | `undefined`                                      | Tulis log debug ke jalur file tertentu. Secara implisit mengaktifkan mode debug                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `disallowedTools`                 | `string[]`                                                                                               | `[]`                                             | Tool untuk tolak. Nama bare seperti `"Bash"` menghapus tool dari konteks Claude. Aturan scoped seperti `"Bash(rm *)"` membiarkan tool tersedia dan menolak panggilan yang cocok dalam setiap mode izin, termasuk `bypassPermissions`. Lihat [Izin](/docs/id/agent-sdk/permissions#allow-and-deny-rules)                                                                                                                                                                                                                                                                                                                            |
| `effort`                          | `'low' \| 'medium' \| 'high' \| 'xhigh' \| 'max'`                                                        | Model default                                    | Mengontrol seberapa banyak usaha yang Claude masukkan ke dalam responsnya. Bekerja dengan pemikiran adaptif untuk memandu kedalaman pemikiran. Lihat [sesuaikan tingkat usaha](/docs/id/model-config#adjust-effort-level)                                                                                                                                                                                                                                                                                                                                                                                                          |
| `enableFileCheckpointing`         | `boolean`                                                                                                | `false`                                          | Aktifkan pelacakan perubahan file untuk rewinding. Lihat [File checkpointing](/docs/id/agent-sdk/file-checkpointing)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `env`                             | `Record<string, string \| undefined>`                                                                    | `process.env`                                    | Variabel lingkungan. Ketika diatur, ini mengganti lingkungan subprocess alih-alih menggabungkan dengan `process.env`, jadi lewatkan `{ ...process.env, YOUR_VAR: 'value' }` untuk menyimpan variabel yang diwariskan seperti `PATH`. Lihat [Tangani respons API yang lambat atau terhenti](#handle-slow-or-stalled-api-responses) untuk contoh pola ini, dan [Variabel lingkungan](/docs/id/env-vars) untuk variabel yang dibaca CLI yang mendasarinya. Atur `CLAUDE_AGENT_SDK_CLIENT_APP` untuk mengidentifikasi aplikasi Anda di header User-Agent                                                                               |
| `executable`                      | `'bun' \| 'deno' \| 'node'`                                                                              | Auto-detected                                    | Runtime JavaScript yang akan digunakan                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `executableArgs`                  | `string[]`                                                                                               | `[]`                                             | Argumen untuk diteruskan ke executable                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `extraArgs`                       | `Record<string, string \| null>`                                                                         | `{}`                                             | Argumen tambahan                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `fallbackModel`                   | `string`                                                                                                 | `undefined`                                      | Model yang digunakan jika model utama gagal                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `forkSession`                     | `boolean`                                                                                                | `false`                                          | Saat melanjutkan dengan `resume`, fork ke ID sesi baru alih-alih melanjutkan sesi asli                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `forwardSubagentText`             | `boolean`                                                                                                | `false`                                          | Teruskan blok teks dan pemikiran subagen sebagai pesan asisten dan pengguna dengan `parent_tool_use_id` diatur, sehingga konsumen dapat merender transkrip bersarang. Secara default hanya blok `tool_use` dan `tool_result` dari subagen yang dipancarkan                                                                                                                                                                                                                                                                                                                                                                    |
| `hooks`                           | `Partial<Record<`[`HookEvent`](#hookevent)`, `[`HookCallbackMatcher`](#hookcallbackmatcher)`[]>>`        | `{}`                                             | Callback hook untuk event                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `includeHookEvents`               | `boolean`                                                                                                | `false`                                          | Sertakan event lifecycle hook untuk setiap hook event dalam aliran pesan sebagai [`SDKHookStartedMessage`](#sdkhookstartedmessage), [`SDKHookProgressMessage`](#sdkhookprogressmessage), dan [`SDKHookResponseMessage`](#sdkhookresponsemessage). Event lifecycle untuk hook `SessionStart` dan `Setup` selalu disertakan dan tidak memerlukan opsi ini                                                                                                                                                                                                                                                                       |
| `includePartialMessages`          | `boolean`                                                                                                | `false`                                          | Sertakan event pesan parsial                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `loadTimeoutMs`                   | `number`                                                                                                 | `60000`                                          | *Alpha.* Timeout dalam milidetik untuk setiap panggilan `sessionStore.load()` dan `sessionStore.listSubkeys()` selama materialisasi resume. Jika adapter tidak settle dalam jendela ini, query gagal alih-alih hang. Diabaikan ketika `sessionStore` tidak diatur                                                                                                                                                                                                                                                                                                                                                             |
| `managedSettings`                 | `Settings`                                                                                               | `undefined`                                      | Pengaturan tingkat kebijakan yang disediakan oleh proses parent yang memunculkan. Dijatuhkan ketika tier managed-settings yang dikontrol IT sudah ada di mesin, kecuali admin itu memilih dengan `parentSettingsBehavior: 'merge'`. Disaring ke kunci restrictive-only terlepas dari itu                                                                                                                                                                                                                                                                                                                                      |
| `maxBudgetUsd`                    | `number`                                                                                                 | `undefined`                                      | Hentikan query ketika estimasi biaya sisi klien mencapai nilai USD ini. Dibandingkan dengan estimasi yang sama seperti `total_cost_usd`; lihat [Lacak biaya dan penggunaan](/docs/id/agent-sdk/cost-tracking) untuk peringatan akurasi                                                                                                                                                                                                                                                                                                                                                                                             |
| `maxThinkingTokens`               | `number`                                                                                                 | `undefined`                                      | *Deprecated:* Gunakan `thinking` sebagai gantinya. Token maksimum untuk proses pemikiran                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `maxTurns`                        | `number`                                                                                                 | `undefined`                                      | Putaran agen maksimum (perjalanan round-trip penggunaan tool)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `mcpServers`                      | `Record<string, [`McpServerConfig`](#mcpserverconfig)>`                                                  | `{}`                                             | Konfigurasi server MCP                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `model`                           | `string`                                                                                                 | Default dari CLI                                 | Alias model Claude atau nama model lengkap. Lihat [nilai yang diterima dan ID khusus penyedia](/docs/id/model-config#available-models)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `onElicitation`                   | `(request: ElicitationRequest, options: { signal: AbortSignal }) => Promise<ElicitationResult>`          | `undefined`                                      | Callback untuk menangani permintaan elicitation MCP. Dipanggil ketika server MCP meminta input pengguna dan tidak ada hook yang menanganinya terlebih dahulu. Ketika tidak disediakan, permintaan elicitation yang tidak ditangani secara otomatis ditolak                                                                                                                                                                                                                                                                                                                                                                    |
| `outputFormat`                    | `{ type: 'json_schema', schema: JSONSchema }`                                                            | `undefined`                                      | Tentukan format output untuk hasil agen. Lihat [Output terstruktur](/docs/id/agent-sdk/structured-outputs) untuk detail                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `outputStyle`                     | `string`                                                                                                 | `undefined`                                      | Bukan field `Options`. Atur `outputStyle` dalam objek [`settings`](/docs/id/settings) inline atau file pengaturan sebagai gantinya. Lihat [Aktifkan output style](/docs/id/agent-sdk/modifying-system-prompts#activate-an-output-style)                                                                                                                                                                                                                                                                                                                                                                                                 |
| `pathToClaudeCodeExecutable`      | `string`                                                                                                 | Auto-resolved dari biner asli bundel             | Jalur ke executable Claude Code. Hanya diperlukan jika dependensi opsional dilewati selama instalasi atau platform Anda tidak dalam set yang didukung                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `permissionMode`                  | [`PermissionMode`](#permissionmode)                                                                      | `'default'`                                      | Mode izin untuk sesi                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `permissionPromptToolName`        | `string`                                                                                                 | `undefined`                                      | Nama tool MCP untuk prompt izin                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `persistSession`                  | `boolean`                                                                                                | `true`                                           | Ketika `false`, menonaktifkan persistensi sesi ke disk. Sesi tidak dapat dilanjutkan nanti                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `planModeInstructions`            | `string`                                                                                                 | `undefined`                                      | Instruksi alur kerja kustom untuk mode plan. Ketika `permissionMode` adalah `'plan'`, string ini mengganti badan alur kerja mode plan default. CLI masih membungkusnya dengan preamble penegakan read-only dan footer protokol ExitPlanMode                                                                                                                                                                                                                                                                                                                                                                                   |
| `plugins`                         | [`SdkPluginConfig`](#sdkpluginconfig)`[]`                                                                | `[]`                                             | Muat plugin kustom dari jalur lokal. Lihat [Plugins](/docs/id/agent-sdk/plugins) untuk detail                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `promptSuggestions`               | `boolean`                                                                                                | `false`                                          | Aktifkan saran prompt. Mengirimkan pesan `prompt_suggestion` setelah setiap putaran dengan prompt pengguna berikutnya yang diprediksi                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `resume`                          | `string`                                                                                                 | `undefined`                                      | ID sesi untuk dilanjutkan                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `resumeSessionAt`                 | `string`                                                                                                 | `undefined`                                      | Lanjutkan sesi pada UUID pesan tertentu                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `sandbox`                         | [`SandboxSettings`](#sandboxsettings)                                                                    | `undefined`                                      | Konfigurasi perilaku sandbox secara terprogram. Lihat [Pengaturan sandbox](#sandboxsettings) untuk detail                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `sessionId`                       | `string`                                                                                                 | Auto-generated                                   | Gunakan UUID tertentu untuk sesi alih-alih auto-generate                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `sessionStore`                    | [`SessionStore`](/docs/id/agent-sdk/session-storage#the-sessionstore-interface)                               | `undefined`                                      | Cerminkan transkrip sesi ke backend eksternal sehingga host apa pun dapat melanjutkannya. Lihat [Pertahankan sesi ke penyimpanan eksternal](/docs/id/agent-sdk/session-storage)                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `sessionStoreFlush`               | `'batched' \| 'eager'`                                                                                   | `'batched'`                                      | *Alpha.* Mode flush untuk `sessionStore`. Diabaikan ketika `sessionStore` tidak diatur                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `settings`                        | `string \| Settings`                                                                                     | `undefined`                                      | Objek [pengaturan](/docs/id/settings) inline atau jalur ke file pengaturan. Mengisi lapisan flag-settings dalam [urutan preseden](/docs/id/settings#settings-precedence). Ubah saat runtime dengan [`applyFlagSettings()`](#applyflagsettings)                                                                                                                                                                                                                                                                                                                                                                                          |
| `settingSources`                  | [`SettingSource`](#settingsource)`[]`                                                                    | CLI defaults (all sources)                       | Kontrol pengaturan filesystem mana yang akan dimuat. Lewatkan `[]` untuk menonaktifkan pengaturan pengguna, proyek, dan lokal. Pengaturan kebijakan terkelola dimuat terlepas dari itu; pengaturan yang dikelola server diambil ketika sesi mengautentikasi dengan kredensial organisasi pada [konfigurasi yang memenuhi syarat](/docs/id/server-managed-settings#platform-availability). Lihat [Gunakan fitur Claude Code](/docs/id/agent-sdk/claude-code-features#what-settingsources-does-not-control)                                                                                                                               |
| `skills`                          | `string[] \| 'all'`                                                                                      | `undefined`                                      | Skills yang tersedia untuk sesi. Lewatkan `'all'` untuk mengaktifkan setiap skill yang ditemukan, atau daftar nama skill. Ketika diatur, SDK menambahkan tool Skill ke `allowedTools` secara otomatis. Jika Anda juga meneruskan `tools`, sertakan `'Skill'` dalam daftar itu. Lihat [Skills](/docs/id/agent-sdk/skills)                                                                                                                                                                                                                                                                                                           |
| `spawnClaudeCodeProcess`          | `(options: SpawnOptions) => SpawnedProcess`                                                              | `undefined`                                      | Fungsi kustom untuk spawn proses Claude Code. Gunakan untuk menjalankan Claude Code di VM, kontainer, atau lingkungan jarak jauh                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `stderr`                          | `(data: string) => void`                                                                                 | `undefined`                                      | Callback untuk output stderr                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `strictMcpConfig`                 | `boolean`                                                                                                | `false`                                          | Gunakan hanya server yang diteruskan dalam `mcpServers` dan abaikan `.mcp.json` proyek, pengaturan pengguna, server MCP yang disediakan plugin, dan [konektor claude.ai](/docs/id/mcp#use-mcp-servers-from-claude-ai)                                                                                                                                                                                                                                                                                                                                                                                                              |
| `systemPrompt`                    | `string \| { type: 'preset'; preset: 'claude_code'; append?: string; excludeDynamicSections?: boolean }` | `undefined` (minimal prompt)                     | Konfigurasi prompt sistem. Lewatkan string untuk prompt kustom, atau `{ type: 'preset', preset: 'claude_code' }` untuk menggunakan prompt sistem Claude Code. Saat menggunakan bentuk objek preset, tambahkan `append` untuk memperluas dengan instruksi tambahan, dan atur `excludeDynamicSections: true` untuk memindahkan konteks per-sesi ke pesan pengguna pertama untuk [reuse prompt-cache yang lebih baik di seluruh mesin](/docs/id/agent-sdk/modifying-system-prompts#improve-prompt-caching-across-users-and-machines)                                                                                                  |
| `taskBudget`                      | `{ total: number }`                                                                                      | `undefined`                                      | *Alpha.* Anggaran tugas sisi API dalam token. Ketika diatur, model diberitahu anggaran token sisa yang tersedia sehingga dapat mengatur kecepatan penggunaan tool dan membungkus sebelum batas                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `thinking`                        | [`ThinkingConfig`](#thinkingconfig)                                                                      | `{ type: 'adaptive' }` untuk model yang didukung | Mengontrol perilaku pemikiran/penalaran Claude. Lihat [`ThinkingConfig`](#thinkingconfig) untuk opsi                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `title`                           | `string`                                                                                                 | `undefined`                                      | Judul tampilan untuk sesi. Saat melanjutkan melalui `resume` atau `continue`, judul sesi yang dilanjutkan yang bertahan mengambil preseden; gunakan [`renameSession()`](#renamesession) untuk mengubah judul sesi yang ada                                                                                                                                                                                                                                                                                                                                                                                                    |
| `toolAliases`                     | `Record<string, string>`                                                                                 | `undefined`                                      | Petakan nama tool bawaan ke nama tool MCP sehingga Claude memanggil implementasi MCP Anda sebagai gantinya dari bawaan. Misalnya, `{ Bash: 'mcp__workspace__bash' }`                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `toolConfig`                      | [`ToolConfig`](#toolconfig)                                                                              | `undefined`                                      | Konfigurasi untuk perilaku tool bawaan. Lihat [`ToolConfig`](#toolconfig) untuk detail                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `tools`                           | `string[] \| { type: 'preset'; preset: 'claude_code' }`                                                  | `undefined`                                      | Konfigurasi tool. Lewatkan array nama tool atau gunakan preset untuk mendapatkan tool default Claude Code                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |

<h4 id="handle-slow-or-stalled-api-responses">
  Tangani respons API yang lambat atau terhenti
</h4>

Subprocess CLI membaca beberapa variabel lingkungan yang mengontrol timeout API dan deteksi stall. Lewatkan melalui opsi `env`:

```typescript theme={null}
const result = query({
  prompt: "Analyze this code",
  options: {
    env: {
      ...process.env,
      API_TIMEOUT_MS: "120000",
      CLAUDE_CODE_MAX_RETRIES: "2",
      CLAUDE_ASYNC_AGENT_STALL_TIMEOUT_MS: "120000",
    },
  },
});
```

* `API_TIMEOUT_MS`: timeout per-request pada klien Anthropic, dalam milidetik. Default `600000`. Berlaku untuk loop utama dan semua subagen.
* `CLAUDE_CODE_MAX_RETRIES`: maksimum retry API. Default `10`, dibatasi pada `15`. Setiap retry mendapatkan jendela `API_TIMEOUT_MS` sendiri, jadi waktu dinding terburuk kira-kira `API_TIMEOUT_MS × (CLAUDE_CODE_MAX_RETRIES + 1)` ditambah backoff. Untuk run yang tidak diawasi yang perlu menunggu melalui pemadaman yang lebih lama, atur `CLAUDE_CODE_RETRY_WATCHDOG=1`: itu retry kapasitas error tanpa batas, dan mulai dari Claude Code v2.1.199 menaikkan default untuk error transien lainnya menjadi `300` dan menghapus cap pada variabel ini.
* `CLAUDE_ASYNC_AGENT_STALL_TIMEOUT_MS`: stall watchdog untuk subagen yang diluncurkan dengan `run_in_background`. Default `600000`. Reset pada setiap event stream; pada stall itu membatalkan subagen, menandai tugas gagal, dan menampilkan error ke parent dengan hasil parsial apa pun. Tidak berlaku untuk subagen sinkron.
* `CLAUDE_ENABLE_STREAM_WATCHDOG` dengan `CLAUDE_STREAM_IDLE_TIMEOUT_MS`: membatalkan request ketika header telah tiba tetapi body respons berhenti streaming. Watchdog aktif secara default untuk semua penyedia; atur `CLAUDE_ENABLE_STREAM_WATCHDOG=0` untuk menonaktifkannya. `CLAUDE_STREAM_IDLE_TIMEOUT_MS` default ke `300000` dan diklem ke minimum itu. Request yang dibatalkan melalui jalur retry normal.

<h3 id="query-object">
  Objek `Query`
</h3>

Antarmuka yang dikembalikan oleh fungsi `query()`.

```typescript theme={null}
interface Query extends AsyncGenerator<SDKMessage, void> {
  interrupt(): Promise<SDKControlInterruptResponse | undefined>;
  rewindFiles(
    userMessageId: string,
    options?: { dryRun?: boolean }
  ): Promise<RewindFilesResult>;
  setPermissionMode(mode: PermissionMode): Promise<void>;
  setModel(model?: string): Promise<void>;
  setMaxThinkingTokens(maxThinkingTokens: number | null): Promise<void>;
  applyFlagSettings(settings: { [K in keyof Settings]?: Settings[K] | null }): Promise<void>;
  initializationResult(): Promise<SDKControlInitializeResponse>;
  reinitialize(): Promise<SDKControlInitializeResponse>;
  supportedCommands(): Promise<SlashCommand[]>;
  supportedModels(): Promise<ModelInfo[]>;
  supportedAgents(): Promise<AgentInfo[]>;
  mcpServerStatus(): Promise<McpServerStatus[]>;
  accountInfo(): Promise<AccountInfo>;
  reconnectMcpServer(serverName: string): Promise<void>;
  toggleMcpServer(serverName: string, enabled: boolean): Promise<void>;
  setMcpServers(servers: Record<string, McpServerConfig>): Promise<McpSetServersResult>;
  streamInput(stream: AsyncIterable<SDKUserMessage>): Promise<void>;
  stopTask(taskId: string): Promise<void>;
  close(): void;
}
```

<h4 id="methods">
  Metode
</h4>

| Metode                                 | Deskripsi                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| :------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `interrupt()`                          | Mengganggu query. Hanya tersedia dalam mode input streaming. Ketika CLI mengiklankan kemampuan `interrupt_receipt_v1` dalam [`SDKSystemMessage.capabilities`](#sdksystemmessage), diselesaikan dengan [`SDKControlInterruptResponse`](#sdkcontrolinterruptresponse) yang mencantumkan pesan yang antri yang bertahan dari interrupt. Diselesaikan `undefined` pada CLI sebelum v2.1.205                                                                            |
| `rewindFiles(userMessageId, options?)` | Mengembalikan file ke keadaan mereka pada pesan pengguna yang ditentukan. Lewatkan `{ dryRun: true }` untuk pratinjau perubahan. Memerlukan `enableFileCheckpointing: true`. Lihat [File checkpointing](/docs/id/agent-sdk/file-checkpointing)                                                                                                                                                                                                                          |
| `setPermissionMode()`                  | Mengubah mode izin (hanya tersedia dalam mode input streaming)                                                                                                                                                                                                                                                                                                                                                                                                     |
| `setModel()`                           | Mengubah model (hanya tersedia dalam mode input streaming)                                                                                                                                                                                                                                                                                                                                                                                                         |
| `setMaxThinkingTokens()`               | *Deprecated:* Gunakan opsi `thinking` sebagai gantinya. Mengubah token pemikiran maksimum. Melewatkan `null` mereset pemikiran ke default sesi: override mid-session dihapus, dan pemikiran tetap mati untuk sesi yang memilikinya dinonaktifkan                                                                                                                                                                                                                   |
| `applyFlagSettings(settings)`          | Menggabungkan pengaturan ke dalam lapisan flag settings sesi saat runtime (hanya tersedia dalam mode input streaming). Lihat [`applyFlagSettings()`](#applyflagsettings)                                                                                                                                                                                                                                                                                           |
| `initializationResult()`               | Mengembalikan hasil inisialisasi lengkap termasuk perintah yang didukung, model, info akun, dan konfigurasi gaya output                                                                                                                                                                                                                                                                                                                                            |
| `reinitialize()`                       | Kirim ulang permintaan kontrol `initialize` ke CLI yang sedang berjalan dan kembalikan hasil segar alih-alih hasil first-connect yang di-cache. Gunakan setelah celah transport, seperti reattaching ke sesi setelah disconnect, sehingga permintaan izin yang tertunda mencapai callback `canUseTool` Anda lagi. Buat callback idempotent per request ID, karena permintaan yang responsnya hilang dikirim ulang. Memerlukan Claude Code v2.1.195 atau lebih baru |
| `supportedCommands()`                  | Mengembalikan perintah slash yang tersedia                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `supportedModels()`                    | Mengembalikan model yang tersedia dengan info tampilan                                                                                                                                                                                                                                                                                                                                                                                                             |
| `supportedAgents()`                    | Mengembalikan subagen yang tersedia sebagai [`AgentInfo`](#agentinfo)`[]`                                                                                                                                                                                                                                                                                                                                                                                          |
| `mcpServerStatus()`                    | Mengembalikan status server MCP yang terhubung                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `accountInfo()`                        | Mengembalikan informasi akun                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `reconnectMcpServer(serverName)`       | Sambungkan kembali server MCP berdasarkan nama                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `toggleMcpServer(serverName, enabled)` | Aktifkan atau nonaktifkan server MCP berdasarkan nama                                                                                                                                                                                                                                                                                                                                                                                                              |
| `setMcpServers(servers)`               | Ganti secara dinamis set server MCP untuk sesi ini. Mengembalikan info tentang server mana yang ditambahkan, dihapus, dan error apa pun                                                                                                                                                                                                                                                                                                                            |
| `streamInput(stream)`                  | Alirkan pesan input ke query untuk percakapan multi-putaran                                                                                                                                                                                                                                                                                                                                                                                                        |
| `stopTask(taskId)`                     | Hentikan tugas latar belakang yang sedang berjalan berdasarkan ID                                                                                                                                                                                                                                                                                                                                                                                                  |
| `close()`                              | Tutup query dan hentikan proses yang mendasarinya. Secara paksa mengakhiri query dan membersihkan semua sumber daya                                                                                                                                                                                                                                                                                                                                                |

<h4 id="applyflagsettings">
  `applyFlagSettings()`
</h4>

Mengubah [pengaturan](/docs/id/settings) pada sesi yang sedang berjalan tanpa memulai ulang query. Gunakan ketika pengaturan yang tidak memiliki setter khusus perlu berubah di tengah sesi, seperti memperketat `permissions` setelah agen membaca input yang tidak terpercaya. `setModel()` dan `setPermissionMode()` adalah setter khusus untuk dua kunci itu; `applyFlagSettings()` adalah bentuk umum yang menerima subset kunci pengaturan apa pun, dan melewatkan `model` di sini berperilaku sama seperti `setModel()`.

Hanya beberapa kunci yang berlaku di tengah sesi:

* **Diterapkan pada putaran berikutnya**: `model`, `effortLevel`, `ultracode`, `permissions`, `hooks`, `skillOverrides`, `fastMode`, `agent`. Beralih `agent` juga menerapkan penggantian model agen itu, hooks, dan prompt sistem pada putaran berikutnya.
* **Tidak ada efek di tengah sesi**: opsi prompt sistem. Ini diselesaikan sekali saat startup, jadi sesi yang sedang berjalan menyimpan nilai asli meskipun panggilan berhasil. Untuk mengubahnya, mulai sesi baru.

`effortLevel` menerima nama [tingkat usaha](/docs/id/model-config#adjust-effort-level). Ini juga menerima `"ultracode"`, yang menjalankan sesi pada usaha `xhigh` dan mengaktifkan [ultracode](/docs/id/workflows#let-claude-decide-with-ultracode). Tipe `Settings` mendeklarasikan `effortLevel` tanpa nilai itu, jadi lewatkan `{ ultracode: true }` yang setara dalam TypeScript. Nilai `ultracode` memerlukan Claude Code v2.1.203 atau lebih baru dan diterima hanya oleh `applyFlagSettings()`, bukan oleh kunci `effortLevel` dalam file pengaturan.

Nilai ditulis ke lapisan flag-settings, lapisan yang sama yang opsi `settings` inline dari `query()` isi saat startup. Flag settings duduk di dekat bagian atas [urutan preseden pengaturan](/docs/id/settings#settings-precedence): mereka mengganti pengaturan pengguna, proyek, dan lokal, dan hanya pengaturan kebijakan terkelola yang dapat mengganti mereka. Ini adalah tier yang sama yang [bagian preseden di halaman](#settings-precedence) sebut opsi terprogram.

Panggilan berturut-turut shallow-merge kunci tingkat atas. Panggilan kedua dengan `{ permissions: {...} }` mengganti seluruh objek `permissions` dari panggilan sebelumnya daripada deep-merging ke dalamnya. Untuk menghapus kunci dari lapisan flag dan kembali ke sumber preseden lebih rendah, lewatkan `null` untuk kunci itu. Melewatkan `undefined` tidak memiliki efek karena serialisasi JSON menjatuhkannya.

Hanya tersedia dalam mode input streaming, batasan yang sama seperti `setModel()` dan `setPermissionMode()`.

Contoh di bawah beralih model aktif di tengah sesi, kemudian menghapus override sehingga model kembali ke apa pun yang ditentukan pengaturan pengguna atau proyek.

```typescript theme={null}
const q = query({ prompt: messageStream });

// Override model untuk sisa sesi
await q.applyFlagSettings({ model: "claude-opus-4-6" });

// Nanti: hapus override dan kembali ke pengaturan preseden lebih rendah
await q.applyFlagSettings({ model: null });
```

<Note>
  `applyFlagSettings()` adalah TypeScript-only. SDK Python tidak mengekspos metode setara.
</Note>

<h3 id="warmquery">
  `WarmQuery`
</h3>

Handle yang dikembalikan oleh [`startup()`](#startup). Subprocess sudah dispawn dan diinisialisasi, jadi memanggil `query()` pada handle ini menulis prompt langsung ke proses yang siap tanpa latensi startup.

```typescript theme={null}
interface WarmQuery extends AsyncDisposable {
  query(prompt: string | AsyncIterable<SDKUserMessage>): Query;
  close(): void;
}
```

<h4 id="methods-1">
  Metode
</h4>

| Metode          | Deskripsi                                                                                                                              |
| :-------------- | :------------------------------------------------------------------------------------------------------------------------------------- |
| `query(prompt)` | Kirim prompt ke subprocess yang sudah dipanaskan dan kembalikan [`Query`](#query-object). Hanya dapat dipanggil sekali per `WarmQuery` |
| `close()`       | Tutup subprocess tanpa mengirim prompt. Gunakan ini untuk membuang warm query yang tidak lagi diperlukan                               |

`WarmQuery` mengimplementasikan `AsyncDisposable`, jadi dapat digunakan dengan `await using` untuk pembersihan otomatis.

<h3 id="sdkcontrolinitializeresponse">
  `SDKControlInitializeResponse`
</h3>

Tipe pengembalian dari `initializationResult()`. Berisi data inisialisasi sesi.

```typescript theme={null}
type SDKControlInitializeResponse = {
  commands: SlashCommand[];
  agents: AgentInfo[];
  output_style: string;
  available_output_styles: string[];
  models: ModelInfo[];
  account: AccountInfo;
  fast_mode_state?: "off" | "cooldown" | "on";
};
```

Ketika klien mengirim `initialize` ke sesi yang sudah berjalan, wrapper control-response juga membawa array `pending_permission_requests` opsional. Field berada pada wrapper respons itu sendiri, bukan dalam payload `SDKControlInitializeResponse` di atas. Setiap entri adalah pesan `control_request` lengkap dengan bentuk `{ type: "control_request", request_id, request }` yang sama dengan sesi yang dialirkan untuk permintaan izin saat berjalan.

Ini adalah permintaan yang dikeluarkan sebelum klien terhubung dan masih menunggu balasan. SDK membaca array untuk Anda dan mengirimkan setiap entri ke callback [`canUseTool`](#canusetool) Anda, pengiriman ulang yang sama yang [`reinitialize()`](#query-object) picu setelah celah transport. Tangani ID permintaan berulang secara idempotent, karena entri dapat mengulangi permintaan yang callback sudah terima sebelum koneksi putus.

<h3 id="sdkcontrolinterruptresponse">
  `SDKControlInterruptResponse`
</h3>

Penerimaan interrupt: nilai yang [`interrupt()`](#query-object) diselesaikan dengan pada CLI yang mengiklankan kemampuan `interrupt_receipt_v1` dalam [`SDKSystemMessage.capabilities`](#sdksystemmessage). Memerlukan Claude Code v2.1.205 atau lebih baru. CLI sebelumnya menjawab interrupt dengan payload kesuksesan kosong, jadi `interrupt()` diselesaikan ke `undefined`.

```typescript theme={null}
type SDKControlInterruptResponse = {
  still_queued: string[];
};
```

`still_queued` mencantumkan UUID pesan pengguna yang bertahan dari interrupt: pesan masih dalam antrian, ditambah batch apa pun yang sudah dihapus antrian untuk putaran berikutnya tetapi belum dapat dijangkau oleh abort. Masing-masing berjalan sebagai putaran sendiri setelah interrupt kecuali Anda membatalkannya terlebih dahulu. Gunakan penerimaan untuk memutuskan apakah akan mengirim ulang apa pun; mengirim ulang pesan yang sudah terdaftar menghasilkan putaran duplikat.

Interpretasikan daftar dengan peringatan ini:

* Hanya pesan yang antri dengan UUID muncul. Array kosong tidak berarti tidak ada lagi yang akan berjalan.
* Hanya pesan thread utama yang terdaftar. Pesan yang ditujukan ke subagen di luar jangkauan.
* Daftar dapat mencakup UUID yang klien Anda tidak pernah kirim, seperti pemicu [tugas terjadwal](/docs/id/scheduled-tasks). Abaikan UUID yang tidak Anda kenal alih-alih memperlakukannya sebagai error.

Penerimaan adalah snapshot yang diambil pada saat interrupt diproses, dan pada interrupt yang bersih tiba sebelum [`SDKResultMessage`](#sdkresultmessage) putaran yang terputus. Baca penerimaan daripada memeriksa antrian setelah hasil itu: loop memulai putaran antrian berikutnya segera, jadi antrian yang Anda periksa setelah hasil sudah berubah.

<h3 id="agentdefinition">
  `AgentDefinition`
</h3>

Konfigurasi untuk subagen yang didefinisikan secara terprogram.

```typescript theme={null}
type AgentDefinition = {
  description: string;
  tools?: string[];
  disallowedTools?: string[];
  prompt: string;
  model?: string;
  mcpServers?: AgentMcpServerSpec[];
  skills?: string[];
  initialPrompt?: string;
  maxTurns?: number;
  background?: boolean;
  memory?: "user" | "project" | "local";
  effort?: "low" | "medium" | "high" | "xhigh" | "max" | number;
  permissionMode?: PermissionMode;
  criticalSystemReminder_EXPERIMENTAL?: string;
};
```

| Field                                 | Diperlukan | Deskripsi                                                                                                                                                                                                                                         |
| :------------------------------------ | :--------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `description`                         | Ya         | Deskripsi bahasa alami tentang kapan menggunakan agen ini                                                                                                                                                                                         |
| `tools`                               | Tidak      | Array nama tool yang diizinkan. Jika dihilangkan, mewarisi semua tool dari parent. Untuk preload Skills ke dalam konteks agen, gunakan field `skills` daripada mencantumkan `'Skill'` di sini                                                     |
| `disallowedTools`                     | Tidak      | Array nama tool untuk secara eksplisit tidak izinkan untuk agen ini. Pola tingkat server MCP juga diterima: `mcp__server` atau `mcp__server__*` menghapus setiap tool dari server itu, dan `mcp__*` menghapus setiap tool MCP dari server apa pun |
| `prompt`                              | Ya         | Prompt sistem agen                                                                                                                                                                                                                                |
| `model`                               | Tidak      | Penggantian model untuk agen ini. Menerima alias seperti `'fable'`, `'opus'`, `'sonnet'`, `'haiku'`, `'inherit'`, atau ID model lengkap. Jika dihilangkan atau `'inherit'`, menggunakan model utama                                               |
| `mcpServers`                          | Tidak      | Spesifikasi server MCP untuk agen ini                                                                                                                                                                                                             |
| `skills`                              | Tidak      | Array nama skill untuk preload ke konteks agen                                                                                                                                                                                                    |
| `initialPrompt`                       | Tidak      | Auto-submitted sebagai putaran pengguna pertama ketika agen ini berjalan sebagai agen thread utama                                                                                                                                                |
| `maxTurns`                            | Tidak      | Jumlah maksimum putaran agen (API round-trips) sebelum berhenti                                                                                                                                                                                   |
| `background`                          | Tidak      | Jalankan agen ini sebagai tugas latar belakang non-blocking ketika dipanggil                                                                                                                                                                      |
| `memory`                              | Tidak      | Sumber memori untuk agen ini: `'user'`, `'project'`, atau `'local'`                                                                                                                                                                               |
| `effort`                              | Tidak      | Tingkat usaha penalaran untuk agen ini. Menerima tingkat bernama atau integer                                                                                                                                                                     |
| `permissionMode`                      | Tidak      | Mode izin untuk eksekusi tool dalam agen ini. Lihat [`PermissionMode`](#permissionmode)                                                                                                                                                           |
| `criticalSystemReminder_EXPERIMENTAL` | Tidak      | Eksperimental: Pengingat kritis ditambahkan ke prompt sistem                                                                                                                                                                                      |

<h3 id="agentmcpserverspec">
  `AgentMcpServerSpec`
</h3>

Menentukan server MCP yang tersedia untuk subagen. Dapat berupa nama server (string yang mereferensikan server dari konfigurasi `mcpServers` parent) atau konfigurasi server inline yang merekam nama server ke config.

```typescript theme={null}
type AgentMcpServerSpec = string | Record<string, McpServerConfigForProcessTransport>;
```

Di mana `McpServerConfigForProcessTransport` adalah `McpStdioServerConfig | McpSSEServerConfig | McpHttpServerConfig | McpSdkServerConfig`.

<h3 id="settingsource">
  `SettingSource`
</h3>

Mengontrol sumber konfigurasi berbasis filesystem mana yang dimuat pengaturan SDK.

```typescript theme={null}
type SettingSource = "user" | "project" | "local";
```

| Nilai       | Deskripsi                                          | Lokasi                        |
| :---------- | :------------------------------------------------- | :---------------------------- |
| `'user'`    | Pengaturan pengguna global                         | `~/.claude/settings.json`     |
| `'project'` | Pengaturan proyek bersama (version controlled)     | `.claude/settings.json`       |
| `'local'`   | Pengaturan proyek lokal (tidak version controlled) | `.claude/settings.local.json` |

<h4 id="default-behavior">
  Perilaku default
</h4>

Ketika `settingSources` dihilangkan atau `undefined`, `query()` memuat pengaturan filesystem yang sama seperti CLI Claude Code: pengguna, proyek, dan lokal. Pengaturan kebijakan terkelola dimuat dalam semua kasus; pengaturan yang dikelola server diambil ketika sesi mengautentikasi dengan kredensial organisasi pada [konfigurasi yang memenuhi syarat](/docs/id/server-managed-settings#platform-availability). Lihat [Apa yang tidak dikontrol settingSources](/docs/id/agent-sdk/claude-code-features#what-settingsources-does-not-control) untuk input yang dibaca terlepas dari opsi ini, dan cara menonaktifkannya.

<h4 id="why-use-settingsources">
  Mengapa menggunakan settingSources
</h4>

**Nonaktifkan pengaturan filesystem:**

```typescript theme={null}
// Jangan muat pengaturan pengguna, proyek, atau lokal dari disk
const result = query({
  prompt: "Analyze this code",
  options: { settingSources: [] }
});
```

**Muat semua pengaturan filesystem secara eksplisit:**

```typescript theme={null}
const result = query({
  prompt: "Analyze this code",
  options: {
    settingSources: ["user", "project", "local"] // Muat semua pengaturan
  }
});
```

**Muat hanya sumber pengaturan tertentu:**

```typescript theme={null}
// Muat hanya pengaturan proyek, abaikan pengguna dan lokal
const result = query({
  prompt: "Run CI checks",
  options: {
    settingSources: ["project"] // Hanya .claude/settings.json
  }
});
```

**Lingkungan pengujian dan CI:**

```typescript theme={null}
// Pastikan perilaku konsisten di CI dengan mengecualikan pengaturan lokal
const result = query({
  prompt: "Run tests",
  options: {
    settingSources: ["project"], // Hanya pengaturan bersama tim
    permissionMode: "bypassPermissions"
  }
});
```

**Aplikasi SDK-only:**

```typescript theme={null}
// Tentukan semuanya secara terprogram.
// Lewatkan [] untuk opt out dari sumber pengaturan filesystem.
const result = query({
  prompt: "Review this PR",
  options: {
    settingSources: [],
    agents: {
      /* ... */
    },
    mcpServers: {
      /* ... */
    },
    allowedTools: ["Read", "Grep", "Glob"]
  }
});
```

**Memuat instruksi proyek CLAUDE.md:**

```typescript theme={null}
// Muat pengaturan proyek untuk menyertakan file CLAUDE.md
const result = query({
  prompt: "Add a new feature following project conventions",
  options: {
    systemPrompt: {
      type: "preset",
      preset: "claude_code" // Gunakan prompt sistem Claude Code
    },
    settingSources: ["project"], // Memuat CLAUDE.md dari direktori proyek
    allowedTools: ["Read", "Write", "Edit"]
  }
});
```

<h4 id="settings-precedence">
  Preseden pengaturan
</h4>

Ketika beberapa sumber dimuat, pengaturan digabungkan dengan preseden ini (tertinggi ke terendah):

1. Pengaturan lokal (`.claude/settings.local.json`)
2. Pengaturan proyek (`.claude/settings.json`)
3. Pengaturan pengguna (`~/.claude/settings.json`)

Opsi terprogram seperti `agents`, `allowedTools`, dan `settings` mengganti pengaturan filesystem pengguna, proyek, dan lokal. Pengaturan kebijakan terkelola mengambil preseden atas opsi terprogram.

<h3 id="permissionmode">
  `PermissionMode`
</h3>

```typescript theme={null}
type PermissionMode =
  | "default" // Perilaku izin standar
  | "acceptEdits" // Auto-accept edit file
  | "bypassPermissions" // Bypass pemeriksaan izin; aturan ask eksplisit masih prompt
  | "plan" // Mode perencanaan - jelajahi tanpa mengedit
  | "dontAsk" // Jangan prompt untuk izin, tolak jika tidak pre-approved
  | "auto"; // Gunakan classifier model untuk approve atau deny setiap tool call
```

<h3 id="canusetool">
  `CanUseTool`
</h3>

Tipe fungsi izin kustom untuk mengontrol penggunaan tool.

Fungsi adalah pengganti SDK untuk prompt izin interaktif: dipanggil hanya ketika [alur evaluasi izin](/docs/id/agent-sdk/permissions#how-permissions-are-evaluated) diselesaikan ke prompt. Panggilan tool yang sudah disetujui oleh entri `allowedTools`, aturan allow pengaturan, atau mode izin, seperti `acceptEdits` atau `bypassPermissions`, tidak pernah memanggilnya. Untuk gating setiap tool call, gunakan hook [`PreToolUse`](/docs/id/agent-sdk/hooks) sebagai gantinya.

`AskUserQuestion`, tool MCP yang ditandai [`requiresUserInteraction`](/docs/id/mcp#require-approval-for-a-specific-tool), dan tool konektor [organisasi Anda atur ke `ask`](/docs/id/mcp#organization-controls-on-connector-tools) mencapai fungsi bahkan ketika aturan allow cocok. Dalam mode `dontAsk` panggilan ini ditolak sebagai gantinya, tanpa memanggilnya.

```typescript theme={null}
type CanUseTool = (
  toolName: string,
  input: Record<string, unknown>,
  options: {
    signal: AbortSignal;
    suggestions?: PermissionUpdate[];
    blockedPath?: string;
    decisionReason?: string;
    toolUseID: string;
    agentID?: string;
    requestId: string;
  }
) => Promise<PermissionResult | null>;
```

| Opsi             | Tipe                                        | Deskripsi                                                                                                                                                                                                                                                                                                   |
| :--------------- | :------------------------------------------ | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `signal`         | `AbortSignal`                               | Signaled jika operasi harus dibatalkan                                                                                                                                                                                                                                                                      |
| `suggestions`    | [`PermissionUpdate`](#permissionupdate)`[]` | Saran pembaruan izin sehingga pengguna tidak diprompt lagi untuk tool ini. Prompt Bash menyertakan saran dengan [destination](#permissionupdatedestination) `localSettings`, jadi mengembalikannya dalam `updatedPermissions` menulis aturan ke `.claude/settings.local.json` dan bertahan di seluruh sesi. |
| `blockedPath`    | `string`                                    | Jalur file yang memicu permintaan izin, jika berlaku                                                                                                                                                                                                                                                        |
| `decisionReason` | `string`                                    | Menjelaskan mengapa permintaan izin ini dipicu                                                                                                                                                                                                                                                              |
| `toolUseID`      | `string`                                    | Pengenal unik untuk tool call spesifik ini dalam pesan asisten                                                                                                                                                                                                                                              |
| `agentID`        | `string`                                    | Jika berjalan dalam sub-agen, ID sub-agen                                                                                                                                                                                                                                                                   |
| `requestId`      | `string`                                    | `request_id` dari envelope `control_request`. Sebuah `control_response` yang aplikasi Anda kirim di luar SDK, seperti HTTP POST yang ditandatangani, harus mengulangi nilai ini sehingga proses Claude Code dapat mencocokkan balasan ke permintaan                                                         |

Callback biasanya menyelesaikan permintaan dengan mengembalikan [`PermissionResult`](#permissionresult), yang SDK tulis kembali melalui transportnya sebagai `control_response`. Kembalikan `null` hanya ketika aplikasi Anda sudah mengirim `control_response` untuk permintaan ini melalui saluran sendiri, mengulangi `requestId`; SDK kemudian melewati penulisan respons ke transportnya. Mengembalikan `null` dalam kasus lain meninggalkan tool call terblokir tanpa batas, karena tidak ada `control_response` yang pernah dikirim dan prompt izin tidak timeout.

Opsi `requestId` dan nilai pengembalian `null` memerlukan Claude Code v2.1.199 atau lebih baru.

<h3 id="permissionresult">
  `PermissionResult`
</h3>

Hasil pemeriksaan izin.

```typescript theme={null}
type PermissionResult =
  | {
      behavior: "allow";
      updatedInput?: Record<string, unknown>;
      updatedPermissions?: PermissionUpdate[];
      toolUseID?: string;
    }
  | {
      behavior: "deny";
      message: string;
      interrupt?: boolean;
      toolUseID?: string;
    };
```

<h3 id="toolconfig">
  `ToolConfig`
</h3>

Konfigurasi untuk perilaku tool bawaan.

```typescript theme={null}
type ToolConfig = {
  askUserQuestion?: {
    previewFormat?: "markdown" | "html";
  };
};
```

| Field                           | Tipe                   | Deskripsi                                                                                                                                                                          |
| :------------------------------ | :--------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `askUserQuestion.previewFormat` | `'markdown' \| 'html'` | Opt-in ke field `preview` pada opsi [`AskUserQuestion`](/docs/id/agent-sdk/user-input#question-format) dan atur format kontennya. Ketika tidak diatur, Claude tidak mengirimkan preview |

<h3 id="mcpserverconfig">
  `McpServerConfig`
</h3>

Konfigurasi untuk server MCP.

```typescript theme={null}
type McpServerConfig =
  | McpStdioServerConfig
  | McpSSEServerConfig
  | McpHttpServerConfig
  | McpSdkServerConfigWithInstance;
```

<h4 id="mcpstdioserverconfig">
  `McpStdioServerConfig`
</h4>

```typescript theme={null}
type McpStdioServerConfig = {
  type?: "stdio";
  command: string;
  args?: string[];
  env?: Record<string, string>;
};
```

<h4 id="mcpsseserverconfig">
  `McpSSEServerConfig`
</h4>

```typescript theme={null}
type McpSSEServerConfig = {
  type: "sse";
  url: string;
  headers?: Record<string, string>;
};
```

<h4 id="mcphttpserverconfig">
  `McpHttpServerConfig`
</h4>

```typescript theme={null}
type McpHttpServerConfig = {
  type: "http";
  url: string;
  headers?: Record<string, string>;
};
```

<h4 id="mcpsdkserverconfigwithinstance">
  `McpSdkServerConfigWithInstance`
</h4>

```typescript theme={null}
type McpSdkServerConfigWithInstance = {
  type: "sdk";
  name: string;
  instance: McpServer;
};
```

<h4 id="mcpclaudeaiproxyserverconfig">
  `McpClaudeAIProxyServerConfig`
</h4>

```typescript theme={null}
type McpClaudeAIProxyServerConfig = {
  type: "claudeai-proxy";
  url: string;
  id: string;
};
```

<h3 id="sdkpluginconfig">
  `SdkPluginConfig`
</h3>

Konfigurasi untuk memuat plugin di SDK.

```typescript theme={null}
type SdkPluginConfig = {
  type: "local";
  path: string;
  skipMcpDiscovery?: boolean;
};
```

| Field              | Tipe      | Deskripsi                                                                                                                                                                                                    |
| :----------------- | :-------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`             | `'local'` | Harus `'local'` (hanya plugin lokal yang saat ini didukung)                                                                                                                                                  |
| `path`             | `string`  | Jalur absolut atau relatif ke direktori plugin                                                                                                                                                               |
| `skipMcpDiscovery` | `boolean` | Ketika `true`, SDK memuat skills, hooks, agen, dan perintah dari plugin ini tetapi tidak membaca `.mcp.json` atau manifest `mcpServers` miliknya. Atur ini ketika aplikasi Anda memiliki koneksi MCP plugin. |

**Contoh:**

```typescript theme={null}
plugins: [
  { type: "local", path: "./my-plugin" },
  { type: "local", path: "/absolute/path/to/plugin" }
];
```

Untuk informasi lengkap tentang membuat dan menggunakan plugin, lihat [Plugins](/docs/id/agent-sdk/plugins).

## Tipe Pesan

### `SDKMessage`

Tipe union dari semua pesan yang mungkin dikembalikan oleh query.

```typescript theme={null}
type SDKMessage =
  | SDKAssistantMessage
  | SDKUserMessage
  | SDKUserMessageReplay
  | SDKResultMessage
  | SDKSystemMessage
  | SDKPartialAssistantMessage
  | SDKCompactBoundaryMessage
  | SDKStatusMessage
  | SDKLocalCommandOutputMessage
  | SDKHookStartedMessage
  | SDKHookProgressMessage
  | SDKHookResponseMessage
  | SDKPluginInstallMessage
  | SDKToolProgressMessage
  | SDKAuthStatusMessage
  | SDKTaskNotificationMessage
  | SDKTaskStartedMessage
  | SDKTaskProgressMessage
  | SDKTaskUpdatedMessage
  | SDKBackgroundTasksChangedMessage
  | SDKThinkingTokensMessage
  | SDKSessionStateChangedMessage
  | SDKWorkerShuttingDownMessage
  | SDKCommandsChangedMessage
  | SDKNotificationMessage
  | SDKFilesPersistedEvent
  | SDKToolUseSummaryMessage
  | SDKMemoryRecallMessage
  | SDKRateLimitEvent
  | SDKElicitationCompleteMessage
  | SDKPermissionDeniedMessage
  | SDKPromptSuggestionMessage
  | SDKAPIRetryMessage
  | SDKMirrorErrorMessage
  | SDKInformationalMessage
  | SDKConversationResetMessage;
```

### `SDKAssistantMessage`

Pesan respons asisten.

```typescript theme={null}
type SDKAssistantMessage = {
  type: "assistant";
  uuid: UUID;
  session_id: string;
  message: BetaMessage; // Dari Anthropic SDK
  parent_tool_use_id: string | null;
  error?: SDKAssistantMessageError;
};
```

Field `message` adalah [`BetaMessage`](https://platform.claude.com/docs/id/api/messages/create) dari Anthropic SDK. Ini mencakup field seperti `id`, `content`, `model`, `stop_reason`, dan `usage`.

`SDKAssistantMessageError` adalah salah satu dari: `'authentication_failed'`, `'oauth_org_not_allowed'`, `'billing_error'`, `'rate_limit'`, `'overloaded'`, `'invalid_request'`, `'model_not_found'`, `'server_error'`, `'max_output_tokens'`, atau `'unknown'`. `'model_not_found'` berarti model yang dipilih tidak ada atau tidak tersedia untuk akun atau deployment Anda. `'overloaded'` berarti API mengembalikan 529 karena server mencapai kapasitas, berbeda dengan `'rate_limit'`, yang merupakan 429 terhadap kuota Anda.

### `SDKUserMessage`

Pesan input pengguna.

```typescript theme={null}
type SDKUserMessage = {
  type: "user";
  uuid?: UUID;
  session_id?: string;
  message: MessageParam; // Dari Anthropic SDK
  parent_tool_use_id: string | null;
  isSynthetic?: boolean;
  shouldQuery?: boolean;
  tool_use_result?: unknown;
  origin?: SDKMessageOrigin;
};
```

Atur `shouldQuery` ke `false` untuk menambahkan pesan ke transkrip tanpa memicu putaran asisten. Pesan ditahan dan digabungkan ke pesan pengguna berikutnya yang memicu putaran. Gunakan ini untuk menyuntikkan konteks, seperti output perintah yang Anda jalankan out of band, tanpa menghabiskan panggilan model.

Pada pesan yang membawa blok `tool_result`, `tool_use_result` adalah objek output terstruktur tool daripada teks yang dikirim ke model. Bentuknya tergantung pada tool yang dinamai oleh blok `tool_use` yang cocok, jadi field ini diketik `unknown`; bentuk bawaan tercantum di bawah [Tipe Output Tool](#tool-output-types).

Untuk tool `Agent`, `tool_use_result` adalah [`AgentOutput`](#agent-2). Pada hasil `completed`, `content` menyimpan laporan subagent tanpa ID agen dan trailer penggunaan yang Claude Code tambahkan ke teks `tool_result`, jadi render dari `tool_use_result` daripada mem-parse teks itu.

### `SDKUserMessageReplay`

Pesan pengguna yang diputar ulang dengan UUID yang diperlukan.

```typescript theme={null}
type SDKUserMessageReplay = {
  type: "user";
  uuid: UUID;
  session_id: string;
  message: MessageParam;
  parent_tool_use_id: string | null;
  isSynthetic?: boolean;
  tool_use_result?: unknown;
  origin?: SDKMessageOrigin;
  isReplay: true;
};
```

Putaran pengguna yang disuntikkan dari luar sesi, yang [`origin`](#sdkmessageorigin)-nya adalah `peer` atau `channel`, mencapai aliran sebagai replay apakah itu disampaikan selama putaran aktif atau memulai putaran baru saat sesi idle. Sebelum v2.1.207, putaran yang disuntikkan disampaikan saat sesi idle tidak menghasilkan pesan pada aliran dan hanya muncul ketika Anda membaca ulang transkrip.

### `SDKResultMessage`

Pesan hasil akhir.

```typescript theme={null}
type SDKResultMessage =
  | {
      type: "result";
      subtype: "success";
      uuid: UUID;
      session_id: string;
      duration_ms: number;
      duration_api_ms: number;
      is_error: boolean;
      api_error_status?: number | null;
      num_turns: number;
      result: string;
      stop_reason: string | null;
      ttft_ms?: number;
      ttft_stream_ms?: number;
      total_cost_usd: number;
      usage: NonNullableUsage;
      modelUsage: { [modelName: string]: ModelUsage };
      permission_denials: SDKPermissionDenial[];
      structured_output?: unknown;
      deferred_tool_use?: { id: string; name: string; input: Record<string, unknown> };
      terminal_reason?: TerminalReason;
      fast_mode_state?: FastModeState;
      origin?: SDKMessageOrigin;
    }
  | {
      type: "result";
      subtype:
        | "error_max_turns"
        | "error_during_execution"
        | "error_max_budget_usd"
        | "error_max_structured_output_retries";
      uuid: UUID;
      session_id: string;
      duration_ms: number;
      duration_api_ms: number;
      is_error: boolean;
      num_turns: number;
      stop_reason: string | null;
      total_cost_usd: number;
      usage: NonNullableUsage;
      modelUsage: { [modelName: string]: ModelUsage };
      permission_denials: SDKPermissionDenial[];
      errors: string[];
      terminal_reason?: TerminalReason;
      fast_mode_state?: FastModeState;
      origin?: SDKMessageOrigin;
    };
```

Beberapa field pada hasil membawa detail diagnostik di luar `subtype`:

* `api_error_status`: kode status HTTP dari kesalahan API yang mengakhiri percakapan. Tidak ada atau `null` ketika putaran berakhir tanpa kesalahan API.
* `ttft_ms`: waktu ke token pertama dalam milidetik, diukur ketika pesan asisten pertama yang lengkap tiba. Hadir hanya pada cabang kesuksesan.
* `ttft_stream_ms`: waktu dalam milidetik hingga event aliran `message_start` pertama, ketika aliran respons dibuka. Lebih rendah dari `ttft_ms`; celah antara keduanya adalah waktu yang dihabiskan untuk streaming pesan pertama. Hadir hanya pada cabang kesuksesan.
* `terminal_reason`: mengapa loop berakhir. Salah satu dari `"completed"`, `"max_turns"`, `"tool_deferred"`, `"aborted_streaming"`, `"aborted_tools"`, `"hook_stopped"`, `"stop_hook_prevented"`, `"background_requested"`, `"blocking_limit"`, `"rapid_refill_breaker"`, `"prompt_too_long"`, `"image_error"`, `"model_error"`, `"api_error"`, `"malformed_tool_use_exhausted"`, `"budget_exhausted"`, `"structured_output_retry_exhausted"`, `"tool_deferred_unavailable"`, atau `"turn_setup_failed"`.
* `fast_mode_state`: salah satu dari `"on"`, `"off"`, atau `"cooldown"`.

Field `origin` meneruskan [`SDKMessageOrigin`](#sdkmessageorigin) dari pesan pengguna yang memicu hasil ini. Ketika tugas latar belakang selesai dan SDK menyuntikkan putaran lanjutan sintetis, `SDKResultMessage` yang dihasilkan membawa `origin: { kind: "task-notification" }`. Periksa field ini untuk membedakan hasil yang menjawab prompt Anda dari hasil yang dipancarkan untuk lanjutan tugas latar belakang, sehingga Anda dapat merutekan atau menekan yang terakhir. Field ini tidak ada untuk hasil yang dipancarkan sebelum putaran pengguna apa pun, seperti kesalahan startup.

Ketika hook `PreToolUse` mengembalikan `permissionDecision: "defer"`, hasilnya memiliki `stop_reason: "tool_deferred"` dan `deferred_tool_use` membawa `id`, `name`, dan `input` tool yang tertunda. Baca field ini untuk menampilkan permintaan di UI Anda sendiri, kemudian lanjutkan dengan `session_id` yang sama untuk melanjutkan. Lihat [Defer a tool call for later](/docs/id/hooks#defer-a-tool-call-for-later) untuk perjalanan putaran lengkap.

### `SDKSystemMessage`

Pesan inisialisasi sistem.

```typescript theme={null}
type SDKSystemMessage = {
  type: "system";
  subtype: "init";
  uuid: UUID;
  session_id: string;
  agents?: string[];
  apiKeySource: ApiKeySource;
  betas?: string[];
  claude_code_version: string;
  cwd: string;
  tools: string[];
  mcp_servers: {
    name: string;
    status: string;
  }[];
  model: string;
  permissionMode: PermissionMode;
  slash_commands: string[];
  output_style: string;
  skills: string[];
  plugins: { name: string; path: string }[];
  capabilities?: string[];
};
```

Array `capabilities` menamai perilaku protokol yang diimplementasikan CLI ini, sehingga Anda dapat mendeteksi fitur daripada membandingkan string `claude_code_version`. Ini adalah set terbuka: abaikan nilai yang tidak Anda kenali, dan periksa kemampuan spesifik yang perilakunya Anda andalkan. Field ini memerlukan Claude Code v2.1.205 atau lebih baru dan tidak ada pada CLI yang lebih awal.

| Kemampuan              | Arti                                                                                                                                                                                |
| ---------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `interrupt_receipt_v1` | [`interrupt()`](#query-object) diselesaikan dengan penerimaan [`SDKControlInterruptResponse`](#sdkcontrolinterruptresponse) yang menamai pesan antrian yang bertahan dari interrupt |

### `SDKPartialAssistantMessage`

Pesan parsial streaming (hanya ketika `includePartialMessages` adalah true). Field `parent_tool_use_id` selalu `null`: event aliran dipancarkan untuk sesi utama saja. Untuk atribusi subagent, gunakan pesan lengkap, yang membawa `parent_tool_use_id`, atau aktifkan [`forwardSubagentText`](#options) untuk menerima teks dan pemikiran subagent sebagai pesan lengkap.

```typescript theme={null}
type SDKPartialAssistantMessage = {
  type: "stream_event";
  event: BetaRawMessageStreamEvent; // Dari Anthropic SDK
  parent_tool_use_id: string | null;
  uuid: UUID;
  session_id: string;
  ttft_ms?: number; // Waktu ke token pertama dalam ms, hadir hanya pada event message_start
};
```

### `SDKCompactBoundaryMessage`

Pesan yang menunjukkan batas pemadatan percakapan.

```typescript theme={null}
type SDKCompactBoundaryMessage = {
  type: "system";
  subtype: "compact_boundary";
  uuid: UUID;
  session_id: string;
  compact_metadata: {
    trigger: "manual" | "auto";
    pre_tokens: number;
  };
};
```

### `SDKStatusMessage`

Pesan status aliran.

```typescript theme={null}
type SDKStatusMessage = {
  type: "system";
  subtype: "status";
  status: string;
  uuid: UUID;
  session_id: string;
};
```

### `SDKLocalCommandOutputMessage`

Output perintah lokal.

```typescript theme={null}
type SDKLocalCommandOutputMessage = {
  type: "system";
  subtype: "local_command_output";
  command: string;
  output: string;
  uuid: UUID;
  session_id: string;
};
```

### `SDKHookStartedMessage`

Hook dimulai.

```typescript theme={null}
type SDKHookStartedMessage = {
  type: "system";
  subtype: "hook_started";
  hook_name: string;
  uuid: UUID;
  session_id: string;
};
```

### `SDKHookProgressMessage`

Kemajuan hook.

```typescript theme={null}
type SDKHookProgressMessage = {
  type: "system";
  subtype: "hook_progress";
  hook_name: string;
  progress: string;
  uuid: UUID;
  session_id: string;
};
```

### `SDKHookResponseMessage`

Respons hook.

```typescript theme={null}
type SDKHookResponseMessage = {
  type: "system";
  subtype: "hook_response";
  hook_name: string;
  response: unknown;
  uuid: UUID;
  session_id: string;
};
```

### `SDKToolProgressMessage`

Kemajuan tool.

```typescript theme={null}
type SDKToolProgressMessage = {
  type: "system";
  subtype: "tool_progress";
  tool_name: string;
  progress: string;
  uuid: UUID;
  session_id: string;
};
```

### `SDKAuthStatusMessage`

Status autentikasi.

```typescript theme={null}
type SDKAuthStatusMessage = {
  type: "system";
  subtype: "auth_status";
  status: string;
  uuid: UUID;
  session_id: string;
};
```

### `SDKTaskNotificationMessage`

Notifikasi tugas.

```typescript theme={null}
type SDKTaskNotificationMessage = {
  type: "system";
  subtype: "task_notification";
  task_id: string;
  message: string;
  uuid: UUID;
  session_id: string;
};
```

### `SDKTaskStartedMessage`

Tugas dimulai.

```typescript theme={null}
type SDKTaskStartedMessage = {
  type: "system";
  subtype: "task_started";
  task_id: string;
  uuid: UUID;
  session_id: string;
};
```

### `SDKTaskProgressMessage`

Kemajuan tugas.

```typescript theme={null}
type SDKTaskProgressMessage = {
  type: "system";
  subtype: "task_progress";
  task_id: string;
  progress: string;
  uuid: UUID;
  session_id: string;
};
```

### `SDKTaskUpdatedMessage`

Tugas diperbarui.

```typescript theme={null}
type SDKTaskUpdatedMessage = {
  type: "system";
  subtype: "task_updated";
  task_id: string;
  update: unknown;
  uuid: UUID;
  session_id: string;
};
```

### `SDKBackgroundTasksChangedMessage`

Tugas latar belakang berubah.

```typescript theme={null}
type SDKBackgroundTasksChangedMessage = {
  type: "system";
  subtype: "background_tasks_changed";
  tasks: string[];
  uuid: UUID;
  session_id: string;
};
```

### `SDKThinkingTokensMessage`

Token pemikiran.

```typescript theme={null}
type SDKThinkingTokensMessage = {
  type: "system";
  subtype: "thinking_tokens";
  tokens: number;
  uuid: UUID;
  session_id: string;
};
```

### `SDKSessionStateChangedMessage`

Status sesi berubah.

```typescript theme={null}
type SDKSessionStateChangedMessage = {
  type: "system";
  subtype: "session_state_changed";
  state: string;
  uuid: UUID;
  session_id: string;
};
```

### `SDKWorkerShuttingDownMessage`

Dipancarkan pada pembongkaran worker yang elegan sehingga klien jarak jauh dapat menunjukkan mengapa worker hilang daripada menunggu timeout heartbeat. `reason` adalah string snake\_case pendek yang ditetapkan oleh CLI host, seperti `"host_exit"` atau `"remote_control_disabled"`. Bertindak atas ini hanya ketika streaming langsung. Sesi yang dilanjutkan memutar ulang instance masa lalu dari pesan ini, jadi abaikan dalam kasus itu.

```typescript theme={null}
type SDKWorkerShuttingDownMessage = {
  type: "system";
  subtype: "worker_shutting_down";
  reason: string;
  uuid: UUID;
  session_id: string;
};
```

### `SDKCommandsChangedMessage`

Perintah berubah.

```typescript theme={null}
type SDKCommandsChangedMessage = {
  type: "system";
  subtype: "commands_changed";
  commands: string[];
  uuid: UUID;
  session_id: string;
};
```

### `SDKNotificationMessage`

Notifikasi.

```typescript theme={null}
type SDKNotificationMessage = {
  type: "system";
  subtype: "notification";
  message: string;
  uuid: UUID;
  session_id: string;
};
```

### `SDKFilesPersistedEvent`

Event file yang disimpan.

```typescript theme={null}
type SDKFilesPersistedEvent = {
  type: "system";
  subtype: "files_persisted";
  files: string[];
  uuid: UUID;
  session_id: string;
};
```

### `SDKToolUseSummaryMessage`

Ringkasan penggunaan tool.

```typescript theme={null}
type SDKToolUseSummaryMessage = {
  type: "system";
  subtype: "tool_use_summary";
  summary: string;
  uuid: UUID;
  session_id: string;
};
```

### `SDKMemoryRecallMessage`

Pesan recall memori.

```typescript theme={null}
type SDKMemoryRecallMessage = {
  type: "system";
  subtype: "memory_recall";
  memory: string;
  uuid: UUID;
  session_id: string;
};
```

### `SDKRateLimitEvent`

Event batas laju.

```typescript theme={null}
type SDKRateLimitEvent = {
  type: "system";
  subtype: "rate_limit";
  limit: number;
  uuid: UUID;
  session_id: string;
};
```

### `SDKElicitationCompleteMessage`

Elicitasi selesai.

```typescript theme={null}
type SDKElicitationCompleteMessage = {
  type: "system";
  subtype: "elicitation_complete";
  uuid: UUID;
  session_id: string;
};
```

### `SDKPermissionDeniedMessage`

Event aliran yang dipancarkan ketika sistem izin secara otomatis menolak panggilan tool tanpa prompt interaktif. Gunakan ini untuk merender penolakan di UI Anda saat terjadi, daripada hanya mengamati hasil tool `is_error` yang mengikuti. Jalur tanya interaktif mencapai aplikasi Anda secara terpisah melalui callback [`canUseTool`](#canusetool). Penolakan yang dikeluarkan oleh hook `PreToolUse` tidak dilaporkan melalui event ini.

Event ini memerlukan Claude Code v2.1.136 atau lebih baru.

```typescript theme={null}
type SDKPermissionDeniedMessage = {
  type: "system";
  subtype: "permission_denied";
  tool_name: string;
  tool_use_id: string;
  agent_id?: string;
  decision_reason_type?: string;
  decision_reason?: string;
  message: string;
  uuid: UUID;
  session_id: string;
};
```

| Field                  | Tipe     | Deskripsi                                                                                                                              |
| ---------------------- | -------- | -------------------------------------------------------------------------------------------------------------------------------------- |
| `tool_name`            | `string` | Nama tool yang ditolak                                                                                                                 |
| `tool_use_id`          | `string` | ID dari blok `tool_use` yang dijawab penolakan ini                                                                                     |
| `agent_id`             | `string` | ID subagent ketika panggilan yang ditolak berasal dari dalam subagent. Mencerminkan field pada `can_use_tool` untuk perutean sisi host |
| `decision_reason_type` | `string` | Diskriminator untuk komponen yang memutuskan, seperti `"rule"`, `"mode"`, `"classifier"`, atau `"asyncAgent"`                          |
| `decision_reason`      | `string` | Alasan yang dapat dibaca manusia dari komponen yang memutuskan, ketika tersedia                                                        |
| `message`              | `string` | Pesan penolakan yang dikembalikan ke model dalam `tool_result`                                                                         |

### `SDKPromptSuggestionMessage`

Saran prompt.

```typescript theme={null}
type SDKPromptSuggestionMessage = {
  type: "system";
  subtype: "prompt_suggestion";
  suggestion: string;
  uuid: UUID;
  session_id: string;
};
```

### `SDKAPIRetryMessage`

Percobaan ulang API.

```typescript theme={null}
type SDKAPIRetryMessage = {
  type: "system";
  subtype: "api_retry";
  attempt: number;
  uuid: UUID;
  session_id: string;
};
```

### `SDKMirrorErrorMessage`

Kesalahan mirror.

```typescript theme={null}
type SDKMirrorErrorMessage = {
  type: "system";
  subtype: "mirror_error";
  error: string;
  uuid: UUID;
  session_id: string;
};
```

### `SDKInformationalMessage`

Spanduk teks generik yang dipancarkan oleh loop. Membawa baris status non-error, umpan balik hook seperti alasan blok hook `UserPromptSubmit`, dan output perintah. Render `content` sebagai plaintext pada `level` yang diberikan.

```typescript theme={null}
type SDKInformationalMessage = {
  type: "system";
  subtype: "informational";
  content: string;
  level: "info" | "notice" | "suggestion" | "warning";
  tool_use_id?: string;
  prevent_continuation?: boolean;
  uuid: UUID;
  session_id: string;
};
```

### `SDKConversationResetMessage`

Percakapan direset.

```typescript theme={null}
type SDKConversationResetMessage = {
  type: "system";
  subtype: "conversation_reset";
  uuid: UUID;
  session_id: string;
};
```

### `SDKPermissionDenial`

Informasi tentang penggunaan tool yang ditolak.

```typescript theme={null}
type SDKPermissionDenial = {
  tool_name: string;
  tool_use_id: string;
  tool_input: Record<string, unknown>;
};
```

### `SDKMessageOrigin`

Asal-usul pesan dengan peran pengguna. Ini muncul sebagai `origin` pada [`SDKUserMessage`](#sdkusermessage) dan diteruskan ke [`SDKResultMessage`](#sdkresultmessage) yang sesuai sehingga Anda dapat mengetahui apa yang memicu putaran tertentu.

```typescript theme={null}
type SDKMessageOrigin =
  | { kind: "human" }
  | { kind: "channel"; server: string }
  | {
      kind: "peer";
      from: string;
      name?: string;
      senderTaskId?: string;
      body?: string;
    }
  | { kind: "task-notification" }
  | { kind: "coordinator" }
  | { kind: "auto-continuation" };
```

| `kind`              | Arti                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| ------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `human`             | Input langsung dari pengguna akhir. Pada pesan pengguna, `origin` yang tidak ada juga berarti input manusia.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `channel`           | Pesan yang tiba di [channel](/docs/id/channels). `server` adalah nama server MCP sumber.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `peer`              | Pesan dari agen lain. Untuk [rekan tim](/docs/id/agent-teams) dalam proses yang mengirim ke `main` melalui `SendMessage`, `from` adalah nama rekan tim dan `senderTaskId` adalah ID tugasnya. Untuk peer lintas sesi seperti proses Claude Code lokal lainnya, `from` adalah alamat pengirim dan `senderTaskId` tidak ada. `name` dan `body` memerlukan Claude Code v2.1.205 atau lebih baru. `name` adalah nama tampilan pengirim, dinormalisasi oleh Claude Code: menghapus kontrol Unicode, format, surrogate, dan pemisah baris atau paragraf code points, kemudian memangkas hasilnya dan membatasinya pada 64 code points dengan ellipsis. `body` adalah badan pesan yang didekode dengan amplop peer yang dihapus, byte-exact dengan apa yang dilihat model. Untuk pesan rekan tim `body` selalu ada; untuk peer lintas sesi itu ada hanya ketika putaran adalah tepat satu amplop peer yang dibentuk oleh Claude Code. Render `name` dan `body` daripada mem-parse ulang teks pesan. |
| `task-notification` | Putaran sintetis yang disuntikkan setelah tugas latar belakang selesai. Lihat [`SDKTaskNotificationMessage`](#sdktasknotificationmessage).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `coordinator`       | Pesan dari koordinator tim dalam [tim agen](/docs/id/agent-teams).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `auto-continuation` | Putaran sintetis yang disuntikkan ketika sesi berlanjut tanpa input pengguna baru, seperti hasil perintah yang memicu prompt lanjutan.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |

<h2 id="hook-types">
  Tipe Hook
</h2>

Untuk panduan komprehensif tentang menggunakan hooks dengan contoh dan pola umum, lihat [panduan Hooks](/docs/id/agent-sdk/hooks).

<h3 id="hookevent">
  `HookEvent`
</h3>

Event hook yang tersedia.

```typescript theme={null}
type HookEvent =
  | "PreToolUse"
  | "PostToolUse"
  | "PostToolUseFailure"
  | "PostToolBatch"
  | "Notification"
  | "UserPromptSubmit"
  | "SessionStart"
  | "SessionEnd"
  | "Stop"
  | "SubagentStart"
  | "SubagentStop"
  | "PreCompact"
  | "PermissionRequest"
  | "Setup"
  | "TeammateIdle"
  | "TaskCompleted"
  | "ConfigChange"
  | "WorktreeCreate"
  | "WorktreeRemove"
  | "MessageDisplay";
```

<h3 id="hookcallback">
  `HookCallback`
</h3>

Tipe fungsi callback hook.

```typescript theme={null}
type HookCallback = (
  input: HookInput, // Union dari semua tipe input hook
  toolUseID: string | undefined,
  options: { signal: AbortSignal }
) => Promise<HookJSONOutput>;
```

<h3 id="hookcallbackmatcher">
  `HookCallbackMatcher`
</h3>

Konfigurasi hook dengan matcher opsional.

```typescript theme={null}
interface HookCallbackMatcher {
  matcher?: string;
  hooks: HookCallback[];
  timeout?: number; // Timeout dalam detik untuk semua hook dalam matcher ini
}
```

<h3 id="hookinput">
  `HookInput`
</h3>

Tipe union dari semua tipe input hook.

```typescript theme={null}
type HookInput =
  | PreToolUseHookInput
  | PostToolUseHookInput
  | PostToolUseFailureHookInput
  | PostToolBatchHookInput
  | NotificationHookInput
  | UserPromptSubmitHookInput
  | SessionStartHookInput
  | SessionEndHookInput
  | StopHookInput
  | SubagentStartHookInput
  | SubagentStopHookInput
  | PreCompactHookInput
  | PermissionRequestHookInput
  | SetupHookInput
  | TeammateIdleHookInput
  | TaskCompletedHookInput
  | ConfigChangeHookInput
  | WorktreeCreateHookInput
  | WorktreeRemoveHookInput
  | MessageDisplayHookInput;
```

<h3 id="basehookinput">
  `BaseHookInput`
</h3>

Antarmuka dasar yang diperluas oleh semua tipe input hook.

```typescript theme={null}
type BaseHookInput = {
  session_id: string;
  transcript_path: string;
  cwd: string;
  prompt_id?: string;
  permission_mode?: string;
  effort?: { level: string };
  agent_id?: string;
  agent_type?: string;
};
```

Bidang `prompt_id` adalah UUID yang mengidentifikasi prompt pengguna yang sedang diproses. Ini cocok dengan [atribut `prompt.id` pada acara OpenTelemetry](/docs/id/monitoring-usage#event-correlation-attributes) dan tidak ada sampai input pengguna pertama. Memerlukan Claude Code v2.1.196 atau lebih baru.

<h4 id="pretoolusehookinput">
  `PreToolUseHookInput`
</h4>

```typescript theme={null}
type PreToolUseHookInput = BaseHookInput & {
  hook_event_name: "PreToolUse";
  tool_name: string;
  tool_input: unknown;
  tool_use_id: string;
};
```

<h4 id="posttoolusehookinput">
  `PostToolUseHookInput`
</h4>

```typescript theme={null}
type PostToolUseHookInput = BaseHookInput & {
  hook_event_name: "PostToolUse";
  tool_name: string;
  tool_input: unknown;
  tool_response: unknown;
  tool_use_id: string;
  duration_ms?: number;
};
```

<h4 id="posttoolusefailurehookinput">
  `PostToolUseFailureHookInput`
</h4>

```typescript theme={null}
type PostToolUseFailureHookInput = BaseHookInput & {
  hook_event_name: "PostToolUseFailure";
  tool_name: string;
  tool_input: unknown;
  tool_use_id: string;
  error: string;
  is_interrupt?: boolean;
  duration_ms?: number;
};
```

<h4 id="posttoolbatchhookinput">
  `PostToolBatchHookInput`
</h4>

Dipicu sekali setelah setiap pemanggilan alat dalam batch telah diselesaikan, sebelum permintaan model berikutnya. `tool_response` membawa konten `tool_result` yang diserialisasi yang dilihat model; bentuknya berbeda dari objek `Output` terstruktur dari `PostToolUseHookInput`.

```typescript theme={null}
type PostToolBatchHookInput = BaseHookInput & {
  hook_event_name: "PostToolBatch";
  tool_calls: PostToolBatchToolCall[];
};

type PostToolBatchToolCall = {
  tool_name: string;
  tool_input: unknown;
  tool_use_id: string;
  tool_response?: unknown;
};
```

<h4 id="notificationhookinput">
  `NotificationHookInput`
</h4>

```typescript theme={null}
type NotificationHookInput = BaseHookInput & {
  hook_event_name: "Notification";
  message: string;
  title?: string;
  notification_type: string;
};
```

<h4 id="userpromptsubmithookinput">
  `UserPromptSubmitHookInput`
</h4>

```typescript theme={null}
type UserPromptSubmitHookInput = BaseHookInput & {
  hook_event_name: "UserPromptSubmit";
  prompt: string;
};
```

<h4 id="sessionstarthookinput">
  `SessionStartHookInput`
</h4>

```typescript theme={null}
type SessionStartHookInput = BaseHookInput & {
  hook_event_name: "SessionStart";
  source: "startup" | "resume" | "clear" | "compact";
  agent_type?: string;
  model?: string;
};
```

<h4 id="sessionendhookinput">
  `SessionEndHookInput`
</h4>

```typescript theme={null}
type SessionEndHookInput = BaseHookInput & {
  hook_event_name: "SessionEnd";
  reason: ExitReason; // String dari array EXIT_REASONS
};
```

<h4 id="stophookinput">
  `StopHookInput`
</h4>

```typescript theme={null}
type StopHookInput = BaseHookInput & {
  hook_event_name: "Stop";
  stop_hook_active: boolean;
  last_assistant_message?: string;
  background_tasks?: BackgroundTaskSummary[];
  session_crons?: SessionCronSummary[];
};
```

<h4 id="subagentstarthookinput">
  `SubagentStartHookInput`
</h4>

```typescript theme={null}
type SubagentStartHookInput = BaseHookInput & {
  hook_event_name: "SubagentStart";
  agent_id: string;
  agent_type: string;
};
```

<h4 id="subagentstophookinput">
  `SubagentStopHookInput`
</h4>

```typescript theme={null}
type SubagentStopHookInput = BaseHookInput & {
  hook_event_name: "SubagentStop";
  stop_hook_active: boolean;
  agent_id: string;
  agent_transcript_path: string;
  agent_type: string;
  last_assistant_message?: string;
  background_tasks?: BackgroundTaskSummary[];
  session_crons?: SessionCronSummary[];
};

type BackgroundTaskSummary = {
  id: string;
  type: string;
  status: string;
  description: string;
  command?: string;
  agent_type?: string;
  server?: string;
  tool?: string;
  name?: string;
};

type SessionCronSummary = {
  id: string;
  schedule: string;
  recurring: boolean;
  prompt: string;
};
```

<h4 id="precompacthookinput">
  `PreCompactHookInput`
</h4>

```typescript theme={null}
type PreCompactHookInput = BaseHookInput & {
  hook_event_name: "PreCompact";
  trigger: "manual" | "auto";
  custom_instructions: string | null;
};
```

<h4 id="permissionrequesthookinput">
  `PermissionRequestHookInput`
</h4>

```typescript theme={null}
type PermissionRequestHookInput = BaseHookInput & {
  hook_event_name: "PermissionRequest";
  tool_name: string;
  tool_input: unknown;
  permission_suggestions?: PermissionUpdate[];
};
```

<h4 id="setuphookinput">
  `SetupHookInput`
</h4>

```typescript theme={null}
type SetupHookInput = BaseHookInput & {
  hook_event_name: "Setup";
  trigger: "init" | "maintenance";
};
```

<h4 id="teammateidlehookinput">
  `TeammateIdleHookInput`
</h4>

```typescript theme={null}
type TeammateIdleHookInput = BaseHookInput & {
  hook_event_name: "TeammateIdle";
  teammate_name: string;
  /** @deprecated sejak v2.1.178. Membawa nama tim yang diturunkan dari sesi; akan dihapus. */
  team_name: string;
};
```

<h4 id="taskcompletedhookinput">
  `TaskCompletedHookInput`
</h4>

```typescript theme={null}
type TaskCompletedHookInput = BaseHookInput & {
  hook_event_name: "TaskCompleted";
  task_id: string;
  task_subject: string;
  task_description?: string;
  teammate_name?: string;
  /** @deprecated sejak v2.1.178. Membawa nama tim yang diturunkan dari sesi; akan dihapus. */
  team_name?: string;
};
```

<h4 id="configchangehookinput">
  `ConfigChangeHookInput`
</h4>

```typescript theme={null}
type ConfigChangeHookInput = BaseHookInput & {
  hook_event_name: "ConfigChange";
  source:
    | "user_settings"
    | "project_settings"
    | "local_settings"
    | "policy_settings"
    | "skills";
  file_path?: string;
};
```

<h4 id="worktreecreatehookinput">
  `WorktreeCreateHookInput`
</h4>

```typescript theme={null}
type WorktreeCreateHookInput = BaseHookInput & {
  hook_event_name: "WorktreeCreate";
  name: string;
};
```

<h4 id="worktreeremovehookinput">
  `WorktreeRemoveHookInput`
</h4>

```typescript theme={null}
type WorktreeRemoveHookInput = BaseHookInput & {
  hook_event_name: "WorktreeRemove";
  worktree_path: string;
};
```

<h4 id="messagedisplayhookinput">
  `MessageDisplayHookInput`
</h4>

```typescript theme={null}
type MessageDisplayHookInput = BaseHookInput & {
  hook_event_name: "MessageDisplay";
  turn_id: string;
  message_id: string;
  index: number;
  final: boolean;
  delta: string;
};
```

<h3 id="hookjsonoutput">
  `HookJSONOutput`
</h3>

Nilai pengembalian hook.

```typescript theme={null}
type HookJSONOutput = AsyncHookJSONOutput | SyncHookJSONOutput;
```

<h4 id="asynchookjsonoutput">
  `AsyncHookJSONOutput`
</h4>

```typescript theme={null}
type AsyncHookJSONOutput = {
  async: true;
  asyncTimeout?: number;
};
```

<h4 id="synchookjsonoutput">
  `SyncHookJSONOutput`
</h4>

```typescript theme={null}
type SyncHookJSONOutput = {
  continue?: boolean;
  suppressOutput?: boolean;
  stopReason?: string;
  decision?: "approve" | "block";
  systemMessage?: string;
  reason?: string;
  hookSpecificOutput?:
    | {
        hookEventName: "PreToolUse";
        permissionDecision?: "allow" | "deny" | "ask" | "defer";
        permissionDecisionReason?: string;
        updatedInput?: Record<string, unknown>;
        additionalContext?: string;
      }
    | {
        hookEventName: "UserPromptSubmit";
        additionalContext?: string;
      }
    | {
        hookEventName: "SessionStart";
        additionalContext?: string;
      }
    | {
        hookEventName: "Setup";
        additionalContext?: string;
      }
    | {
        hookEventName: "SubagentStart";
        additionalContext?: string;
      }
    | {
        hookEventName: "PostToolUse";
        additionalContext?: string;
        updatedToolOutput?: unknown;
        /** @deprecated Gunakan `updatedToolOutput`, yang berfungsi untuk semua alat. */
        updatedMCPToolOutput?: unknown;
      }
    | {
        hookEventName: "PostToolUseFailure";
        additionalContext?: string;
      }
    | {
        hookEventName: "PostToolBatch";
        additionalContext?: string;
      }
    | {
        hookEventName: "Notification";
        additionalContext?: string;
      }
    | {
        hookEventName: "PermissionRequest";
        decision:
          | {
              behavior: "allow";
              updatedInput?: Record<string, unknown>;
              updatedPermissions?: PermissionUpdate[];
            }
          | {
              behavior: "deny";
              message?: string;
              interrupt?: boolean;
            };
      };
};
```

<h2 id="tool-input-types">
  Tipe Input Tool
</h2>

Dokumentasi skema input untuk semua tool Claude Code bawaan. Tipe ini dieksport dari `@anthropic-ai/claude-agent-sdk` dan dapat digunakan untuk interaksi tool yang aman tipe.

<h3 id="toolinputschemas">
  `ToolInputSchemas`
</h3>

Union dari semua tipe input tool, dieksport dari `@anthropic-ai/claude-agent-sdk`.

```typescript theme={null}
type ToolInputSchemas =
  | AgentInput
  | AskUserQuestionInput
  | BashInput
  | TaskOutputInput
  | EnterWorktreeInput
  | ExitPlanModeInput
  | FileEditInput
  | FileReadInput
  | FileWriteInput
  | GlobInput
  | GrepInput
  | ListMcpResourcesInput
  | McpInput
  | MonitorInput
  | NotebookEditInput
  | ReadMcpResourceInput
  | SubscribeMcpResourceInput
  | SubscribePollingInput
  | TaskCreateInput
  | TaskGetInput
  | TaskListInput
  | TaskStopInput
  | TaskUpdateInput
  | TodoWriteInput
  | UnsubscribeMcpResourceInput
  | UnsubscribePollingInput
  | WebFetchInput
  | WebSearchInput
  | WorkflowInput;
```

<h3 id="agent">
  Agent
</h3>

**Nama tool:** `Agent` (sebelumnya `Task`, yang masih diterima sebagai alias)

```typescript theme={null}
type AgentInput = {
  description: string;
  prompt: string;
  subagent_type?: string;
  model?: "sonnet" | "opus" | "haiku" | "fable";
  run_in_background?: boolean;
  name?: string;
  mode?: "acceptEdits" | "auto" | "bypassPermissions" | "default" | "dontAsk" | "plan";
  isolation?: "worktree";
};
```

Meluncurkan agen baru untuk menangani tugas kompleks multi-langkah secara otonom.

<h3 id="askuserquestion">
  AskUserQuestion
</h3>

**Nama tool:** `AskUserQuestion`

```typescript theme={null}
type AskUserQuestionInput = {
  questions: Array<{
    question: string;
    header: string;
    options: Array<{ label: string; description: string; preview?: string }>;
    multiSelect: boolean;
  }>;
};
```

Menanyakan pertanyaan klarifikasi kepada pengguna selama eksekusi. Lihat [Tangani persetujuan dan input pengguna](/docs/id/agent-sdk/user-input#handle-clarifying-questions) untuk detail penggunaan.

<h3 id="bash">
  Bash
</h3>

**Nama tool:** `Bash`

```typescript theme={null}
type BashInput = {
  command: string;
  timeout?: number; // milliseconds, max 600000; higher values are clamped to the max
  description?: string;
  run_in_background?: boolean;
  dangerouslyDisableSandbox?: boolean;
};
```

Mengeksekusi perintah bash dalam sesi shell persisten dengan timeout opsional dan eksekusi latar belakang.

<h3 id="monitor">
  Monitor
</h3>

**Nama tool:** `Monitor`

```typescript theme={null}
type MonitorInput = {
  command?: string;
  ws?: {
    url: string;
    protocols?: string[];
  };
  description: string;
  timeout_ms?: number;
  persistent?: boolean;
};
```

Menjalankan sumber latar belakang dan mengirimkan setiap event ke Claude sehingga dapat bereaksi tanpa polling: `command` menjalankan skrip dan mengeluarkan satu event per baris stdout, dan `ws` membuka WebSocket dan mengeluarkan satu event per frame teks. Berikan tepat satu dari `command` atau `ws`. Sumber `ws` memerlukan Claude Code v2.1.195 atau yang lebih baru.

Atur `persistent: true` untuk watch panjang sesi seperti log tails. Ketika Monitor menjalankan perintah, ia mengikuti aturan izin yang sama seperti Bash; watch WebSocket meminta persetujuan secara terpisah. Lihat [referensi tool Monitor](/docs/id/tools-reference#monitor-tool) untuk perilaku dan ketersediaan provider.

<h3 id="taskoutput">
  TaskOutput
</h3>

**Nama tool:** `TaskOutput`

```typescript theme={null}
type TaskOutputInput = {
  task_id: string;
  block: boolean;
  timeout: number;
};
```

Mengambil output dari tugas latar belakang yang sedang berjalan atau selesai.

<h3 id="edit">
  Edit
</h3>

**Nama tool:** `Edit`

```typescript theme={null}
type FileEditInput = {
  file_path: string;
  old_string: string;
  new_string: string;
  replace_all?: boolean;
};
```

Melakukan penggantian string yang tepat dalam file.

<h3 id="read">
  Read
</h3>

**Nama tool:** `Read`

```typescript theme={null}
type FileReadInput = {
  file_path: string;
  offset?: number;
  limit?: number;
  pages?: string;
};
```

Membaca file dari filesystem lokal, termasuk teks, gambar, PDF, dan notebook Jupyter. Gunakan `pages` untuk rentang halaman PDF (misalnya, `"1-5"`).

<h3 id="write">
  Write
</h3>

**Nama tool:** `Write`

```typescript theme={null}
type FileWriteInput = {
  file_path: string;
  content: string;
};
```

Menulis file ke filesystem lokal, menimpa jika ada.

<h3 id="glob">
  Glob
</h3>

**Nama tool:** `Glob`

```typescript theme={null}
type GlobInput = {
  pattern: string;
  path?: string;
};
```

Pencocokan pola file cepat yang bekerja dengan ukuran codebase apa pun.

<h3 id="grep">
  Grep
</h3>

**Nama tool:** `Grep`

```typescript theme={null}
type GrepInput = {
  pattern: string;
  path?: string;
  glob?: string;
  type?: string;
  output_mode?: "content" | "files_with_matches" | "count";
  "-i"?: boolean;
  "-n"?: boolean;
  "-B"?: number;
  "-A"?: number;
  "-C"?: number;
  context?: number;
  head_limit?: number;
  offset?: number;
  multiline?: boolean;
};
```

Tool pencarian yang kuat dibangun di atas ripgrep dengan dukungan regex.

<h3 id="taskstop">
  TaskStop
</h3>

**Nama tool:** `TaskStop`

```typescript theme={null}
type TaskStopInput = {
  task_id?: string;
  shell_id?: string; // Deprecated: gunakan task_id
};
```

Menghentikan tugas latar belakang atau shell yang sedang berjalan berdasarkan ID. Mulai dari v2.1.198, `task_id` juga menerima rekan tim agent atau agen latar belakang bernama berdasarkan ID agen atau nama.

<h3 id="notebookedit">
  NotebookEdit
</h3>

**Nama tool:** `NotebookEdit`

```typescript theme={null}
type NotebookEditInput = {
  notebook_path: string;
  cell_id?: string;
  new_source: string;
  cell_type?: "code" | "markdown";
  edit_mode?: "replace" | "insert" | "delete";
};
```

Mengedit sel dalam file notebook Jupyter.

<h3 id="webfetch">
  WebFetch
</h3>

**Nama tool:** `WebFetch`

```typescript theme={null}
type WebFetchInput = {
  url: string;
  prompt: string;
};
```

Mengambil konten dari URL dan memprosesnya dengan model AI.

<h3 id="websearch">
  WebSearch
</h3>

**Nama tool:** `WebSearch`

```typescript theme={null}
type WebSearchInput = {
  query: string;
  allowed_domains?: string[];
  blocked_domains?: string[];
};
```

Mencari web dan mengembalikan hasil yang diformat.

<h3 id="workflow">
  Workflow
</h3>

**Nama tool:** `Workflow`

```typescript theme={null}
type WorkflowInput = {
  script?: string;
  name?: string;
  scriptPath?: string;
  args?: unknown;
  resumeFromRunId?: string;
};
```

Menjalankan [workflow dinamis](/docs/id/workflows): skrip yang mengorkestra banyak subagen di latar belakang dan mengembalikan satu hasil yang dikonsolidasikan. Tool `Workflow` tersedia di Agent SDK v0.3.149 dan yang lebih baru. Setidaknya satu dari `script`, `name`, atau `scriptPath` diperlukan.

| Field             | Type      | Description                                                                                                                                                                                                                                                                                                 |
| ----------------- | --------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `script`          | `string`  | Skrip workflow inline. Harus dimulai dengan `export const meta = { name, description }` sebagai literal, diikuti oleh badan skrip menggunakan `agent()`, `parallel()`, `pipeline()`, dan `phase()`. Array `phases` opsional dalam `meta` mengelompokkan agen di bawah tahap bernama dalam tampilan kemajuan |
| `name`            | `string`  | Nama workflow bawaan atau yang disimpan di `.claude/workflows/`. Diselesaikan ke skrip                                                                                                                                                                                                                      |
| `scriptPath`      | `string`  | Jalur ke file skrip workflow di disk. Mengambil prioritas atas `script` dan `name`. Setiap invokasi menyimpan skrip dan mengembalikan jalur dalam hasil, sehingga Anda dapat mengedit file itu dan menginvokasi kembali dengan `scriptPath` yang sama untuk melakukan iterasi                               |
| `args`            | `unknown` | Nilai input yang diekspos ke skrip sebagai `args` global, untuk workflow bernama yang diparameterisasi seperti pertanyaan penelitian atau daftar jalur file. Lewatkan array dan objek sebagai nilai JSON aktual, bukan sebagai string yang dikodekan JSON                                                   |
| `resumeFromRunId` | `string`  | Run ID dari invokasi `Workflow` sebelumnya untuk dilanjutkan. Panggilan `agent()` yang selesai dengan input yang tidak berubah mengembalikan hasil yang di-cache; hanya panggilan yang berubah atau baru yang berjalan langsung. Sesi yang sama saja                                                        |

<h3 id="todowrite">
  TodoWrite
</h3>

**Nama tool:** `TodoWrite`

```typescript theme={null}
type TodoWriteInput = {
  todos: Array<{
    content: string;
    status: "pending" | "in_progress" | "completed";
    activeForm: string;
  }>;
};
```

Membuat dan mengelola daftar tugas terstruktur untuk melacak kemajuan.

<Note>
  Mulai dari TypeScript Agent SDK 0.3.142, `TodoWrite` dinonaktifkan secara default. Gunakan `TaskCreate`, `TaskGet`, `TaskUpdate`, dan `TaskList` sebagai gantinya. Lihat [Migrasi ke tool Task](/docs/id/agent-sdk/todo-tracking#migrate-to-task-tools) untuk memperbarui kode pemantauan Anda, atau atur `CLAUDE_CODE_ENABLE_TASKS=0` untuk kembali ke `TodoWrite`.
</Note>

<h3 id="taskcreate">
  TaskCreate
</h3>

**Nama tool:** `TaskCreate`

```typescript theme={null}
type TaskCreateInput = {
  subject: string;
  description: string;
  activeForm?: string;
  metadata?: Record<string, unknown>;
};
```

Membuat satu tugas dan mengembalikan ID yang ditugaskan.

<h3 id="taskupdate">
  TaskUpdate
</h3>

**Nama tool:** `TaskUpdate`

```typescript theme={null}
type TaskUpdateInput = {
  taskId: string;
  status?: "pending" | "in_progress" | "completed" | "deleted";
  subject?: string;
  description?: string;
  activeForm?: string;
  addBlocks?: string[];
  addBlockedBy?: string[];
  owner?: string;
  metadata?: Record<string, unknown>;
};
```

Menambal satu tugas berdasarkan ID. Atur `status` ke `"deleted"` untuk menghapusnya.

<h3 id="taskget">
  TaskGet
</h3>

**Nama tool:** `TaskGet`

```typescript theme={null}
type TaskGetInput = {
  taskId: string;
};
```

Mengembalikan detail lengkap untuk satu tugas, atau `null` ketika ID tidak ditemukan.

<h3 id="tasklist">
  TaskList
</h3>

**Nama tool:** `TaskList`

```typescript theme={null}
type TaskListInput = {};
```

Mengembalikan snapshot dari semua tugas dalam daftar saat ini.

<h3 id="exitplanmode">
  ExitPlanMode
</h3>

**Nama tool:** `ExitPlanMode`

```typescript theme={null}
type ExitPlanModeInput = {
  /** Deprecated: tidak lagi digunakan. */
  allowedPrompts?: Array<{
    tool: "Bash";
    prompt: string;
  }>;
};
```

Keluar dari mode perencanaan. Bidang `allowedPrompts` sudah usang dan diabaikan; Claude Code masih menerimanya sehingga pemanggil dan transkrip yang ada memvalidasi. Sebelum v2.1.205, ia meminta izin Bash berbasis prompt untuk mengimplementasikan rencana.

<h3 id="listmcpresources">
  ListMcpResources
</h3>

**Nama tool:** `ListMcpResourcesTool`

```typescript theme={null}
type ListMcpResourcesInput = {
  server?: string;
};
```

Membuat daftar sumber daya MCP yang tersedia dari server yang terhubung.

<h3 id="readmcpresource">
  ReadMcpResource
</h3>

**Nama tool:** `ReadMcpResourceTool`

```typescript theme={null}
type ReadMcpResourceInput = {
  server: string;
  uri: string;
};
```

Membaca sumber daya MCP tertentu dari server.

<h3 id="enterworktree">
  EnterWorktree
</h3>

**Nama tool:** `EnterWorktree`

```typescript theme={null}
type EnterWorktreeInput = {
  name?: string;
  path?: string;
};
```

Membuat dan memasuki worktree git sementara untuk pekerjaan terisolasi. Lewatkan `path` untuk beralih ke worktree yang ada alih-alih membuat yang baru. Pada entri pertama target harus berupa worktree terdaftar dari repositori saat ini atau, dalam workspace multi-repo, dari repositori yang bersarang di dalamnya; dari dalam sesi worktree harus berada di bawah `.claude/worktrees/` dari repositori sesi. `name` dan `path` saling eksklusif.

<h2 id="tool-output-types">
  Tipe Output Tool
</h2>

Dokumentasi skema output untuk semua tool Claude Code bawaan. Tipe ini dieksport dari `@anthropic-ai/claude-agent-sdk` dan mewakili data respons aktual yang dikembalikan oleh setiap tool.

<h3 id="tooloutputschemas">
  `ToolOutputSchemas`
</h3>

Union dari semua tipe output tool.

```typescript theme={null}
type ToolOutputSchemas =
  | AgentOutput
  | AskUserQuestionOutput
  | BashOutput
  | EnterWorktreeOutput
  | ExitPlanModeOutput
  | FileEditOutput
  | FileReadOutput
  | FileWriteOutput
  | GlobOutput
  | GrepOutput
  | ListMcpResourcesOutput
  | MonitorOutput
  | NotebookEditOutput
  | ReadMcpResourceOutput
  | TaskCreateOutput
  | TaskGetOutput
  | TaskListOutput
  | TaskStopOutput
  | TaskUpdateOutput
  | TodoWriteOutput
  | WebFetchOutput
  | WebSearchOutput
  | WorkflowOutput;
```

<h3 id="agent-1">
  Agent
</h3>

**Nama tool:** `Agent` (sebelumnya `Task`, yang masih diterima sebagai alias)

```typescript theme={null}
type AgentOutput =
  | {
      status: "completed";
      agentId: string;
      agentType?: string;
      content: Array<{ type: "text"; text: string; citations?: unknown[] | null }>;
      resolvedModel?: string;
      totalToolUseCount: number;
      totalDurationMs: number;
      totalTokens: number;
      usage: {
        input_tokens: number;
        output_tokens: number;
        cache_creation_input_tokens: number | null;
        cache_read_input_tokens: number | null;
        server_tool_use: {
          web_search_requests: number;
          web_fetch_requests: number;
        } | null;
        service_tier: string | null;
        cache_creation: {
          ephemeral_1h_input_tokens: number;
          ephemeral_5m_input_tokens: number;
        } | null;
        inference_geo?: string | null;
        speed?: string | null;
        iterations?: unknown;
      };
      toolStats?: {
        readCount: number;
        searchCount: number;
        bashCount: number;
        editFileCount: number;
        linesAdded: number;
        linesRemoved: number;
        otherToolCount: number;
        frameCount?: number;
      };
      prompt: string;
      worktreePath?: string;
      worktreeBranch?: string;
    }
  | {
      status: "async_launched";
      isAsync?: true;
      agentId: string;
      description: string;
      resolvedModel?: string;
      prompt: string;
      outputFile: string;
      canReadOutputFile?: boolean;
    }
  | {
      status: "remote_launched";
      taskId: string;
      sessionUrl: string;
      description: string;
      prompt: string;
      outputFile: string;
    };
```

Mengembalikan hasil dari subagen. Didiskriminasikan pada field `status`: `"completed"` untuk tugas yang selesai, `"async_launched"` untuk tugas latar belakang, dan `"remote_launched"` untuk tugas yang Claude Code kirimkan ke sesi cloud jarak jauh, di mana `sessionUrl` menautkan ke sesi tersebut dan `taskId` mengidentifikasinya.

Field `resolvedModel` pada varian `completed` dan `async_launched` menamai model yang sebenarnya dijalankan oleh subagen, yang dapat berbeda dari input `model` yang diminta ketika [`availableModels`](/docs/id/model-config#restrict-model-selection) atau override lainnya berlaku. Field ini memerlukan Claude Code v2.1.174 atau lebih baru.

Pada varian `completed`, `worktreePath` diatur ketika subagen berjalan di worktree git terisolasi, dan `worktreeBranch` menamai cabang worktree tersebut ketika Claude Code membuatnya. `usage.service_tier` membawa string tier layanan yang dilaporkan API untuk permintaan subagen.

Sebelum v2.1.207, tipe yang dipublikasikan lebih sempit. Tipe tersebut menghilangkan `worktreePath`, `worktreeBranch`, `citations`, `toolStats.frameCount`, dan field penggunaan `inference_geo`, `speed`, dan `iterations`, dan mengetik `service_tier` sebagai `"standard" | "priority" | "batch"`. Field yang ditandai tipe sebagai opsional dapat tidak ada pada hasil yang dicatat oleh versi sebelumnya.

<h3 id="askuserquestion-1">
  AskUserQuestion
</h3>

**Nama tool:** `AskUserQuestion`

```typescript theme={null}
type AskUserQuestionOutput = {
  questions: Array<{
    question: string;
    header: string;
    options: Array<{ label: string; description: string; preview?: string }>;
    multiSelect: boolean;
  }>;
  answers: Record<string, string>;
  response?: string;
};
```

Mengembalikan pertanyaan yang diajukan dan jawaban pengguna. `response` diatur ketika pengguna mengetik balasan bentuk bebas alih-alih menjawab pertanyaan terstruktur; ketika ada, Claude menerima "Pengguna merespons: …" alih-alih daftar jawaban per-pertanyaan.

<h3 id="bash-1">
  Bash
</h3>

**Nama tool:** `Bash`

```typescript theme={null}
type BashOutput = {
  stdout: string;
  stderr: string;
  rawOutputPath?: string;
  interrupted: boolean;
  isImage?: boolean;
  backgroundTaskId?: string;
  backgroundedByUser?: boolean;
  dangerouslyDisableSandbox?: boolean;
  returnCodeInterpretation?: string;
  structuredContent?: unknown[];
  persistedOutputPath?: string;
  persistedOutputSize?: number;
};
```

Mengembalikan output perintah dengan stdout/stderr terpisah. Perintah latar belakang menyertakan `backgroundTaskId`.

<h3 id="monitor-1">
  Monitor
</h3>

**Nama tool:** `Monitor`

```typescript theme={null}
type MonitorOutput = {
  taskId: string;
  timeoutMs: number;
  persistent?: boolean;
};
```

Mengembalikan ID tugas latar belakang untuk monitor yang sedang berjalan. Gunakan ID ini dengan `TaskStop` untuk membatalkan watch lebih awal.

<h3 id="edit-1">
  Edit
</h3>

**Nama tool:** `Edit`

```typescript theme={null}
type FileEditOutput = {
  filePath: string;
  oldString: string;
  newString: string;
  originalFile: string;
  structuredPatch: Array<{
    oldStart: number;
    oldLines: number;
    newStart: number;
    newLines: number;
    lines: string[];
  }>;
  userModified: boolean;
  replaceAll: boolean;
  gitDiff?: {
    filename: string;
    status: "modified" | "added";
    additions: number;
    deletions: number;
    changes: number;
    patch: string;
  };
};
```

Mengembalikan diff terstruktur dari operasi edit.

<h3 id="read-1">
  Read
</h3>

**Nama tool:** `Read`

```typescript theme={null}
type FileReadOutput =
  | {
      type: "text";
      file: {
        filePath: string;
        content: string;
        numLines: number;
        startLine: number;
        totalLines: number;
      };
    }
  | {
      type: "image";
      file: {
        base64: string;
        type: "image/jpeg" | "image/png" | "image/gif" | "image/webp";
        originalSize: number;
        dimensions?: {
          originalWidth?: number;
          originalHeight?: number;
          displayWidth?: number;
          displayHeight?: number;
        };
      };
    }
  | {
      type: "notebook";
      file: {
        filePath: string;
        cells: unknown[];
      };
    }
  | {
      type: "pdf";
      file: {
        filePath: string;
        base64: string;
        originalSize: number;
      };
    }
  | {
      type: "parts";
      file: {
        filePath: string;
        originalSize: number;
        count: number;
        outputDir: string;
      };
    };
```

Mengembalikan konten file dalam format yang sesuai dengan tipe file. Didiskriminasikan pada field `type`.

<h3 id="write-1">
  Write
</h3>

**Nama tool:** `Write`

```typescript theme={null}
type FileWriteOutput = {
  type: "create" | "update";
  filePath: string;
  content: string;
  structuredPatch: Array<{
    oldStart: number;
    oldLines: number;
    newStart: number;
    newLines: number;
    lines: string[];
  }>;
  originalFile: string | null;
  gitDiff?: {
    filename: string;
    status: "modified" | "added";
    additions: number;
    deletions: number;
    changes: number;
    patch: string;
  };
};
```

Mengembalikan hasil write dengan informasi diff terstruktur.

<h3 id="glob-1">
  Glob
</h3>

**Nama tool:** `Glob`

```typescript theme={null}
type GlobOutput = {
  durationMs: number;
  numFiles: number;
  filenames: string[];
  truncated: boolean;
};
```

Mengembalikan jalur file yang cocok dengan pola glob, diurutkan berdasarkan waktu modifikasi.

<h3 id="grep-1">
  Grep
</h3>

**Nama tool:** `Grep`

```typescript theme={null}
type GrepOutput = {
  mode?: "content" | "files_with_matches" | "count";
  numFiles: number;
  filenames: string[];
  content?: string;
  numLines?: number;
  numMatches?: number;
  appliedLimit?: number;
  appliedOffset?: number;
};
```

Mengembalikan hasil pencarian. Bentuknya bervariasi menurut `mode`: daftar file, konten dengan kecocokan, atau hitungan kecocokan.

<h3 id="taskstop-1">
  TaskStop
</h3>

**Nama tool:** `TaskStop`

```typescript theme={null}
type TaskStopOutput = {
  message: string;
  task_id: string;
  task_type: string;
  command?: string;
};
```

Mengembalikan konfirmasi setelah menghentikan tugas latar belakang.

<h3 id="notebookedit-1">
  NotebookEdit
</h3>

**Nama tool:** `NotebookEdit`

```typescript theme={null}
type NotebookEditOutput = {
  new_source: string;
  cell_id?: string;
  cell_type: "code" | "markdown";
  language: string;
  edit_mode: string;
  error?: string;
  notebook_path: string;
  original_file: string;
  updated_file: string;
};
```

Mengembalikan hasil edit notebook dengan konten file asli dan diperbarui.

<h3 id="webfetch-1">
  WebFetch
</h3>

**Nama tool:** `WebFetch`

```typescript theme={null}
type WebFetchOutput = {
  bytes: number;
  code: number;
  codeText: string;
  result: string;
  durationMs: number;
  url: string;
};
```

Mengembalikan konten yang diambil dengan status HTTP dan metadata.

<h3 id="websearch-1">
  WebSearch
</h3>

**Nama tool:** `WebSearch`

```typescript theme={null}
type WebSearchOutput = {
  query: string;
  results: Array<
    | {
        tool_use_id: string;
        content: Array<{ title: string; url: string }>;
      }
    | string
  >;
  durationSeconds: number;
};
```

Mengembalikan hasil pencarian dari web.

<h3 id="workflow-1">
  Workflow
</h3>

**Nama tool:** `Workflow`

```typescript theme={null}
type WorkflowOutput = {
  status: "async_launched";
  taskId: string;
  runId?: string;
  summary?: string;
  transcriptDir?: string;
  scriptPath?: string;
  error?: string;
};
```

Mengembalikan segera setelah tool menerima invokasi. Hasil akhir tiba kemudian sebagai penyelesaian tugas. Periksa `error` sebelum memperlakukan run sebagai dimulai: skrip yang gagal pemeriksaan sintaksnya mengembalikan `status: "async_launched"` dengan `error` diatur, dan tidak pernah berjalan.

| Field           | Type               | Description                                                                                                                                            |
| --------------- | ------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `status`        | `"async_launched"` | Tool menerima invokasi. Ini adalah satu-satunya nilai yang diambil field                                                                               |
| `taskId`        | `string`           | Pengenal tugas latar belakang untuk run                                                                                                                |
| `runId`         | `string`           | Pengenal workflow run untuk diteruskan sebagai `resumeFromRunId` pada invokasi kemudian                                                                |
| `summary`       | `string`           | Deskripsi satu baris tentang apa yang dilakukan workflow                                                                                               |
| `transcriptDir` | `string`           | Direktori tempat transkrip subagen ditulis selama eksekusi                                                                                             |
| `scriptPath`    | `string`           | Jalur ke skrip workflow yang disimpan untuk run ini. Edit dan teruskan kembali sebagai `scriptPath` untuk menjalankan ulang tanpa mengirim ulang skrip |
| `error`         | `string`           | Diatur ketika skrip gagal pemeriksaan sintaksnya. Ketika ada, run tidak dimulai meskipun status `async_launched`                                       |

<h3 id="todowrite-1">
  TodoWrite
</h3>

**Nama tool:** `TodoWrite`

```typescript theme={null}
type TodoWriteOutput = {
  oldTodos: Array<{
    content: string;
    status: "pending" | "in_progress" | "completed";
    activeForm: string;
  }>;
  newTodos: Array<{
    content: string;
    status: "pending" | "in_progress" | "completed";
    activeForm: string;
  }>;
};
```

Mengembalikan daftar tugas sebelumnya dan diperbarui.

<Note>
  Mulai dari TypeScript Agent SDK 0.3.142, `TodoWrite` dinonaktifkan secara default. Gunakan `TaskCreate`, `TaskGet`, `TaskUpdate`, dan `TaskList` sebagai gantinya. Lihat [Migrasi ke tool Task](/docs/id/agent-sdk/todo-tracking#migrate-to-task-tools) untuk memperbarui kode pemantauan Anda, atau atur `CLAUDE_CODE_ENABLE_TASKS=0` untuk kembali ke `TodoWrite`.
</Note>

<h3 id="taskcreate-1">
  TaskCreate
</h3>

**Nama tool:** `TaskCreate`

```typescript theme={null}
type TaskCreateOutput = {
  task: {
    id: string;
    subject: string;
  };
};
```

Mengembalikan tugas yang dibuat dengan ID yang ditetapkan.

<h3 id="taskupdate-1">
  TaskUpdate
</h3>

**Nama tool:** `TaskUpdate`

```typescript theme={null}
type TaskUpdateOutput = {
  success: boolean;
  taskId: string;
  updatedFields: string[];
  error?: string;
  statusChange?: {
    from: string;
    to: string;
  };
};
```

Mengembalikan hasil pembaruan, termasuk field mana yang berubah.

<h3 id="taskget-1">
  TaskGet
</h3>

**Nama tool:** `TaskGet`

```typescript theme={null}
type TaskGetOutput = {
  task: {
    id: string;
    subject: string;
    description: string;
    status: "pending" | "in_progress" | "completed";
    blocks: string[];
    blockedBy: string[];
  } | null;
};
```

Mengembalikan catatan tugas lengkap, atau `null` ketika ID tidak ditemukan.

<h3 id="tasklist-1">
  TaskList
</h3>

**Nama tool:** `TaskList`

```typescript theme={null}
type TaskListOutput = {
  tasks: Array<{
    id: string;
    subject: string;
    status: "pending" | "in_progress" | "completed";
    owner?: string;
    blockedBy: string[];
  }>;
};
```

Mengembalikan snapshot semua tugas dalam daftar saat ini.

<h3 id="exitplanmode-1">
  ExitPlanMode
</h3>

**Nama tool:** `ExitPlanMode`

```typescript theme={null}
type ExitPlanModeOutput = {
  plan: string | null;
  isAgent: boolean;
  filePath?: string;
  hasTaskTool?: boolean;
  awaitingLeaderApproval?: boolean;
  requestId?: string;
};
```

Mengembalikan status rencana setelah keluar dari mode perencanaan.

<h3 id="listmcpresources-1">
  ListMcpResources
</h3>

**Nama tool:** `ListMcpResourcesTool`

```typescript theme={null}
type ListMcpResourcesOutput = Array<{
  uri: string;
  name: string;
  mimeType?: string;
  description?: string;
  server: string;
}>;
```

Mengembalikan array sumber daya MCP yang tersedia.

<h3 id="readmcpresource-1">
  ReadMcpResource
</h3>

**Nama tool:** `ReadMcpResourceTool`

```typescript theme={null}
type ReadMcpResourceOutput = {
  contents: Array<{
    uri: string;
    mimeType?: string;
    text?: string;
  }>;
};
```

Mengembalikan konten sumber daya MCP yang diminta.

<h3 id="enterworktree-1">
  EnterWorktree
</h3>

**Nama tool:** `EnterWorktree`

```typescript theme={null}
type EnterWorktreeOutput = {
  worktreePath: string;
  worktreeBranch?: string;
  message: string;
};
```

Mengembalikan informasi tentang worktree git.

<h2 id="permission-types">
  Tipe Izin
</h2>

<h3 id="permissionupdate">
  `PermissionUpdate`
</h3>

Operasi untuk memperbarui izin.

```typescript theme={null}
type PermissionUpdate =
  | {
      type: "addRules";
      rules: PermissionRuleValue[];
      behavior: PermissionBehavior;
      destination: PermissionUpdateDestination;
    }
  | {
      type: "replaceRules";
      rules: PermissionRuleValue[];
      behavior: PermissionBehavior;
      destination: PermissionUpdateDestination;
    }
  | {
      type: "removeRules";
      rules: PermissionRuleValue[];
      behavior: PermissionBehavior;
      destination: PermissionUpdateDestination;
    }
  | {
      type: "setMode";
      mode: PermissionMode;
      destination: PermissionUpdateDestination;
    }
  | {
      type: "addDirectories";
      directories: string[];
      destination: PermissionUpdateDestination;
    }
  | {
      type: "removeDirectories";
      directories: string[];
      destination: PermissionUpdateDestination;
    };
```

<h3 id="permissionbehavior">
  `PermissionBehavior`
</h3>

```typescript theme={null}
type PermissionBehavior = "allow" | "deny" | "ask";
```

<h3 id="permissionupdatedestination">
  `PermissionUpdateDestination`
</h3>

```typescript theme={null}
type PermissionUpdateDestination =
  | "userSettings" // Pengaturan pengguna global
  | "projectSettings" // Pengaturan proyek per-direktori
  | "localSettings" // Pengaturan proyek lokal
  | "session" // Hanya sesi saat ini
  | "cliArg"; // Argumen CLI
```

<h3 id="permissionrulevalue">
  `PermissionRuleValue`
</h3>

```typescript theme={null}
type PermissionRuleValue = {
  toolName: string;
  ruleContent?: string;
};
```

<h2 id="other-types">
  Tipe Lainnya
</h2>

<h3 id="apikeysource">
  `ApiKeySource`
</h3>

```typescript theme={null}
type ApiKeySource = "user" | "project" | "org" | "temporary" | "oauth";
```

<h3 id="sdkbeta">
  `SdkBeta`
</h3>

Fitur beta yang tersedia yang dapat diaktifkan melalui opsi `betas`. Lihat [Beta headers](https://platform.claude.com/docs/id/api/beta-headers) untuk informasi lebih lanjut.

```typescript theme={null}
type SdkBeta = "context-1m-2025-08-07";
```

<Warning>
  Beta `context-1m-2025-08-07` sudah pensiun sejak 30 April 2026. Melewatkan nilai ini dengan Claude Sonnet 4.5 atau Sonnet 4 tidak berpengaruh, dan permintaan yang melebihi jendela konteks standar 200k-token mengembalikan error. Untuk menggunakan jendela konteks 1M-token, migrasikan ke [Claude Sonnet 5, Claude Sonnet 4.6, Claude Opus 4.6, Claude Opus 4.7, atau Claude Opus 4.8](https://platform.claude.com/docs/id/about-claude/models/overview), yang mencakup konteks 1M dengan harga standar tanpa header beta yang diperlukan.
</Warning>

<h3 id="slashcommand">
  `SlashCommand`
</h3>

Informasi tentang perintah slash yang tersedia.

```typescript theme={null}
type SlashCommand = {
  name: string;
  description: string;
  argumentHint: string;
  aliases?: string[];
};
```

<h3 id="modelinfo">
  `ModelInfo`
</h3>

Informasi tentang model yang tersedia.

```typescript theme={null}
type ModelInfo = {
  value: string;
  resolvedModel?: string;
  displayName: string;
  description: string;
  supportsEffort?: boolean;
  supportedEffortLevels?: ("low" | "medium" | "high" | "xhigh" | "max")[];
  supportsAdaptiveThinking?: boolean;
  supportsFastMode?: boolean;
  supportsAutoMode?: boolean;
};
```

| Field                      | Tipe                                                               | Deskripsi                                                                                                                                                                                                                                                                                                           |
| :------------------------- | :----------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `value`                    | `string`                                                           | Pengenal model untuk diteruskan dalam panggilan API                                                                                                                                                                                                                                                                 |
| `resolvedModel`            | `string \| undefined`                                              | ID model wire kanonik yang diselesaikan oleh `value` entri ini. Entri alias seperti `sonnet` diselesaikan ke ID model eksplisit seperti `claude-sonnet-5`, sehingga host dapat mencocokkan ID model eksplisit yang disimpan terhadap entri alias yang mencakupnya. Memerlukan Claude Code v2.1.197 atau lebih baru. |
| `displayName`              | `string`                                                           | Nama tampilan yang dapat dibaca manusia                                                                                                                                                                                                                                                                             |
| `description`              | `string`                                                           | Deskripsi kemampuan model                                                                                                                                                                                                                                                                                           |
| `supportsEffort`           | `boolean \| undefined`                                             | Apakah model ini mendukung tingkat upaya                                                                                                                                                                                                                                                                            |
| `supportedEffortLevels`    | `("low" \| "medium" \| "high" \| "xhigh" \| "max")[] \| undefined` | Tingkat upaya yang diterima model ini                                                                                                                                                                                                                                                                               |
| `supportsAdaptiveThinking` | `boolean \| undefined`                                             | Apakah model ini mendukung pemikiran adaptif, di mana Claude memutuskan kapan dan berapa banyak untuk berpikir                                                                                                                                                                                                      |
| `supportsFastMode`         | `boolean \| undefined`                                             | Apakah model ini mendukung mode cepat                                                                                                                                                                                                                                                                               |
| `supportsAutoMode`         | `boolean \| undefined`                                             | Apakah model ini mendukung mode otomatis                                                                                                                                                                                                                                                                            |

<h3 id="agentinfo">
  `AgentInfo`
</h3>

Informasi tentang subagen yang tersedia yang dapat dipanggil melalui tool Agent.

```typescript theme={null}
type AgentInfo = {
  name: string;
  description: string;
  model?: string;
};
```

| Field         | Tipe                  | Deskripsi                                                                    |
| :------------ | :-------------------- | :--------------------------------------------------------------------------- |
| `name`        | `string`              | Pengenal tipe agen (misalnya, `"Explore"`, `"general-purpose"`)              |
| `description` | `string`              | Deskripsi tentang kapan menggunakan agen ini                                 |
| `model`       | `string \| undefined` | Alias model yang digunakan agen ini. Jika dihilangkan, mewarisi model parent |

<h3 id="mcpserverstatus">
  `McpServerStatus`
</h3>

Status server MCP yang terhubung.

```typescript theme={null}
type McpServerStatus = {
  name: string;
  status: "connected" | "failed" | "needs-auth" | "pending" | "disabled";
  serverInfo?: {
    name: string;
    version: string;
  };
  error?: string;
  config?: McpServerStatusConfig;
  scope?: string;
  tools?: {
    name: string;
    description?: string;
    annotations?: {
      readOnly?: boolean;
      destructive?: boolean;
      openWorld?: boolean;
    };
  }[];
};
```

<h3 id="mcpserverstatusconfig">
  `McpServerStatusConfig`
</h3>

Konfigurasi server MCP seperti yang dilaporkan oleh `mcpServerStatus()`. Ini adalah union dari semua tipe transport server MCP.

```typescript theme={null}
type McpServerStatusConfig =
  | McpStdioServerConfig
  | McpSSEServerConfig
  | McpHttpServerConfig
  | McpSdkServerConfig
  | McpClaudeAIProxyServerConfig;
```

Lihat [`McpServerConfig`](#mcpserverconfig) untuk detail tentang setiap tipe transport.

<h3 id="accountinfo">
  `AccountInfo`
</h3>

Informasi akun untuk pengguna yang diautentikasi.

```typescript theme={null}
type AccountInfo = {
  email?: string;
  organization?: string;
  subscriptionType?: string;
  tokenSource?: string;
  apiKeySource?: string;
};
```

<h3 id="modelusage">
  `ModelUsage`
</h3>

Statistik penggunaan per-model yang dikembalikan dalam pesan hasil. Nilai `costUSD` adalah estimasi sisi klien. Lihat [Lacak biaya dan penggunaan](/docs/id/agent-sdk/cost-tracking) untuk peringatan penagihan.

```typescript theme={null}
type ModelUsage = {
  inputTokens: number;
  outputTokens: number;
  cacheReadInputTokens: number;
  cacheCreationInputTokens: number;
  webSearchRequests: number;
  costUSD: number;
  contextWindow: number;
  maxOutputTokens: number;
};
```

<h3 id="configscope">
  `ConfigScope`
</h3>

```typescript theme={null}
type ConfigScope = "local" | "user" | "project";
```

<h3 id="nonnullableusage">
  `NonNullableUsage`
</h3>

Versi [`Usage`](#usage) dengan semua field nullable dibuat non-nullable.

```typescript theme={null}
type NonNullableUsage = {
  [K in keyof Usage]: NonNullable<Usage[K]>;
};
```

<h3 id="usage">
  `Usage`
</h3>

Statistik penggunaan token. Ini adalah tipe `BetaUsage` dari `@anthropic-ai/sdk`.

```typescript theme={null}
type Usage = {
  input_tokens: number;
  output_tokens: number;
  cache_creation_input_tokens: number | null;
  cache_read_input_tokens: number | null;
  cache_creation: {
    ephemeral_5m_input_tokens: number;
    ephemeral_1h_input_tokens: number;
  } | null;
  server_tool_use: BetaServerToolUsage | null;
  service_tier: "standard" | "priority" | "batch" | null;
  speed: "standard" | "fast" | null;
  inference_geo: string | null;
  iterations: BetaIterationsUsage | null;
};
```

`BetaServerToolUsage` dan `BetaIterationsUsage` didefinisikan dalam `@anthropic-ai/sdk`.

<h3 id="calltoolresult">
  `CallToolResult`
</h3>

Tipe hasil tool MCP (dari `@modelcontextprotocol/sdk/types.js`). `structuredContent` adalah objek JSON yang dapat dikembalikan bersama `content`, termasuk blok gambar. Lihat [Kembalikan data terstruktur](/docs/id/agent-sdk/custom-tools#return-structured-data).

```typescript theme={null}
type CallToolResult = {
  content: Array<{
    type: "text" | "image" | "audio" | "resource" | "resource_link";
    // Field tambahan bervariasi menurut tipe
  }>;
  structuredContent?: Record<string, unknown>;
  isError?: boolean;
};
```

<h3 id="thinkingconfig">
  `ThinkingConfig`
</h3>

Mengontrol perilaku pemikiran/penalaran Claude. Mengambil preseden atas `maxThinkingTokens` yang sudah usang.

```typescript theme={null}
type ThinkingDisplay = "summarized" | "omitted";

type ThinkingConfig =
  | { type: "adaptive"; display?: ThinkingDisplay } // Model menentukan kapan dan berapa banyak untuk bernalar (Opus 4.6+)
  | { type: "enabled"; budgetTokens?: number; display?: ThinkingDisplay } // Anggaran token pemikiran tetap
  | { type: "disabled" }; // Tidak ada pemikiran yang diperluas
```

Field `display` opsional mengontrol apakah teks pemikiran dikembalikan `"summarized"` atau `"omitted"`. Pada Claude Opus 4.7 dan yang lebih baru, default API adalah `"omitted"`, jadi atur `"summarized"` untuk menerima konten pemikiran dalam blok `thinking`.

<h3 id="spawnedprocess">
  `SpawnedProcess`
</h3>

Antarmuka untuk spawn proses kustom (digunakan dengan opsi `spawnClaudeCodeProcess`). `ChildProcess` sudah memenuhi antarmuka ini.

```typescript theme={null}
interface SpawnedProcess {
  stdin: Writable;
  stdout: Readable;
  readonly killed: boolean;
  readonly exitCode: number | null;
  kill(signal: NodeJS.Signals): boolean;
  on(
    event: "exit",
    listener: (code: number | null, signal: NodeJS.Signals | null) => void
  ): void;
  on(event: "error", listener: (error: Error) => void): void;
  once(
    event: "exit",
    listener: (code: number | null, signal: NodeJS.Signals | null) => void
  ): void;
  once(event: "error", listener: (error: Error) => void): void;
  off(
    event: "exit",
    listener: (code: number | null, signal: NodeJS.Signals | null) => void
  ): void;
  off(event: "error", listener: (error: Error) => void): void;
}
```

<h3 id="spawnoptions">
  `SpawnOptions`
</h3>

Opsi yang diteruskan ke fungsi spawn kustom.

```typescript theme={null}
interface SpawnOptions {
  command: string;
  args: string[];
  cwd?: string;
  env: Record<string, string | undefined>;
  signal: AbortSignal;
}
```

<Note>
  Field `signal` memberi tahu fungsi spawn Anda kapan harus merobohkan proses. Teruskan sebagai opsi `signal` ke `spawn()` Node, atau teruskan ke handler teardown VM atau container Anda.

  Signal ini tidak menyala saat [`Options.abortController`](#options) membatalkan. SDK pertama-tama menutup stdin proses dan menunggu sekitar dua detik sehingga CLI dapat ditutup dengan bersih, kemudian membatalkan signal ini. Untuk bereaksi saat pemanggil membatalkan, dengarkan `Options.abortController.signal` Anda sendiri, yang dapat direferensikan fungsi spawn Anda dari cakupan penutupnya.
</Note>

<h3 id="mcpsetserversresult">
  `McpSetServersResult`
</h3>

Hasil operasi `setMcpServers()`.

```typescript theme={null}
type McpSetServersResult = {
  added: string[];
  removed: string[];
  errors: Record<string, string>;
};
```

<h3 id="rewindfilesresult">
  `RewindFilesResult`
</h3>

Hasil operasi `rewindFiles()`.

```typescript theme={null}
type RewindFilesResult = {
  canRewind: boolean;
  error?: string;
  filesChanged?: string[];
  insertions?: number;
  deletions?: number;
};
```

<h3 id="sdkstatusmessage">
  `SDKStatusMessage`
</h3>

Pesan pembaruan status (misalnya, pemadatan).

```typescript theme={null}
type SDKStatusMessage = {
  type: "system";
  subtype: "status";
  status: "compacting" | null;
  permissionMode?: PermissionMode;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdktasknotificationmessage">
  `SDKTaskNotificationMessage`
</h3>

Notifikasi ketika tugas latar belakang selesai, gagal, atau dihentikan. Tugas latar belakang mencakup perintah Bash `run_in_background`, watch [Monitor](#monitor), dan subagen latar belakang.

```typescript theme={null}
type SDKTaskNotificationMessage = {
  type: "system";
  subtype: "task_notification";
  task_id: string;
  tool_use_id?: string;
  status: "completed" | "failed" | "stopped";
  output_file: string;
  summary: string;
  usage?: {
    total_tokens: number;
    tool_uses: number;
    duration_ms: number;
  };
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdktoolusesummarymessage">
  `SDKToolUseSummaryMessage`
</h3>

Ringkasan penggunaan tool dalam percakapan.

```typescript theme={null}
type SDKToolUseSummaryMessage = {
  type: "tool_use_summary";
  summary: string;
  preceding_tool_use_ids: string[];
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkhookstartedmessage">
  `SDKHookStartedMessage`
</h3>

Dipancarkan ketika hook mulai mengeksekusi.

Claude Code mengirimkan pesan ini, [`SDKHookProgressMessage`](#sdkhookprogressmessage), dan [`SDKHookResponseMessage`](#sdkhookresponsemessage) ke aliran pesan segera, termasuk saat hook `SessionStart` atau `Setup` masih berjalan selama startup sesi. Claude Code v2.1.169 hingga v2.1.203 mengirimkan pesan ini dalam satu batch setelah hook `SessionStart` atau `Setup` selesai; v2.1.204 mengembalikan pengiriman langsung.

```typescript theme={null}
type SDKHookStartedMessage = {
  type: "system";
  subtype: "hook_started";
  hook_id: string;
  hook_name: string;
  hook_event: string;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkhookprogressmessage">
  `SDKHookProgressMessage`
</h3>

Dipancarkan saat hook sedang berjalan, dengan output stdout/stderr.

```typescript theme={null}
type SDKHookProgressMessage = {
  type: "system";
  subtype: "hook_progress";
  hook_id: string;
  hook_name: string;
  hook_event: string;
  stdout: string;
  stderr: string;
  output: string;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkhookresponsemessage">
  `SDKHookResponseMessage`
</h3>

Dipancarkan ketika hook selesai mengeksekusi.

```typescript theme={null}
type SDKHookResponseMessage = {
  type: "system";
  subtype: "hook_response";
  hook_id: string;
  hook_name: string;
  hook_event: string;
  output: string;
  stdout: string;
  stderr: string;
  exit_code?: number;
  outcome: "success" | "error" | "cancelled";
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdktoolprogressmessage">
  `SDKToolProgressMessage`
</h3>

Dipancarkan secara berkala saat tool sedang mengeksekusi untuk menunjukkan kemajuan.

```typescript theme={null}
type SDKToolProgressMessage = {
  type: "tool_progress";
  tool_use_id: string;
  tool_name: string;
  parent_tool_use_id: string | null;
  elapsed_time_seconds: number;
  task_id?: string;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkauthstatusmessage">
  `SDKAuthStatusMessage`
</h3>

Dipancarkan selama alur autentikasi.

```typescript theme={null}
type SDKAuthStatusMessage = {
  type: "auth_status";
  isAuthenticating: boolean;
  output: string[];
  error?: string;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdktaskstartedmessage">
  `SDKTaskStartedMessage`
</h3>

Dipancarkan ketika tugas latar belakang dimulai. Field `task_type` adalah `"local_bash"` untuk perintah Bash latar belakang dan watch [Monitor](#monitor), `"local_agent"` untuk subagen, atau `"remote_agent"`.

```typescript theme={null}
type SDKTaskStartedMessage = {
  type: "system";
  subtype: "task_started";
  task_id: string;
  tool_use_id?: string;
  description: string;
  task_type?: string;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdktaskprogressmessage">
  `SDKTaskProgressMessage`
</h3>

Dipancarkan secara berkala saat subagen atau tugas latar belakang sedang berjalan. Field `summary` diisi hanya ketika [`agentProgressSummaries`](#options) diaktifkan.

```typescript theme={null}
type SDKTaskProgressMessage = {
  type: "system";
  subtype: "task_progress";
  task_id: string;
  tool_use_id?: string;
  description: string;
  subagent_type?: string;
  usage: {
    total_tokens: number;
    tool_uses: number;
    duration_ms: number;
  };
  last_tool_name?: string;
  summary?: string;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdktaskupdatedmessage">
  `SDKTaskUpdatedMessage`
</h3>

Dipancarkan ketika status tugas latar belakang berubah, seperti ketika transisi dari `running` ke `completed`. Gabungkan `patch` ke dalam peta tugas lokal Anda yang dikunci oleh `task_id`. Field `end_time` adalah timestamp epoch Unix dalam milidetik, dapat dibandingkan dengan `Date.now()`.

```typescript theme={null}
type SDKTaskUpdatedMessage = {
  type: "system";
  subtype: "task_updated";
  task_id: string;
  patch: {
    status?: "pending" | "running" | "completed" | "failed" | "killed";
    description?: string;
    end_time?: number;
    total_paused_ms?: number;
    error?: string;
    is_backgrounded?: boolean;
  };
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkbackgroundtaskschangedmessage">
  `SDKBackgroundTasksChangedMessage`
</h3>

Dipancarkan setiap kali set tugas latar belakang yang aktif berubah: tugas dimulai, selesai, dibunuh, atau agen foreground di-background. Array `tasks` adalah set aktif lengkap. Ganti set yang di-cache dengan setiap payload alih-alih memasangkan acara `task_started` dan `task_notification`, sehingga perubahan keanggotaan berikutnya memperbaiki acara apa pun yang Anda lewatkan.

Pengurutan relatif terhadap acara per-tugas tersebut tidak ditentukan, jadi jangan menghubungkan dua aliran tersebut.

Tidak ada yang dipancarkan saat startup. Atur ulang ke set kosong setiap kali proses CLI sesi dimulai atau dimulai ulang dan biarkan perubahan keanggotaan berikutnya mengisinya kembali.

Memerlukan Claude Code v2.1.203 atau lebih baru.

```typescript theme={null}
type SDKBackgroundTasksChangedMessage = {
  type: "system";
  subtype: "background_tasks_changed";
  tasks: {
    task_id: string;
    task_type: string;
    description: string;
  }[];
  uuid: UUID;
  session_id: string;
};
```

### `SDKThinkingTokensMessage`

Dipancarkan saat Claude menghasilkan blok pemikiran, termasuk yang diredaksi, membawa estimasi berjalan token pemikiran yang dihasilkan sejauh ini. `estimated_tokens` adalah total berjalan untuk blok pemikiran saat ini dan `estimated_tokens_delta` adalah kenaikan yang dibawa oleh frame ini. Gunakan untuk tampilan kemajuan. Hitungan akhir untuk loop agen tingkat atas adalah pesan hasil `usage.output_tokens`, yang [tidak termasuk token subagen](/docs/id/agent-sdk/cost-tracking#get-the-total-cost-of-a-query); gunakan [`modelUsage`](#modelusage) untuk akuntansi seluruh pohon.

Memerlukan Claude Code v2.1.153 atau lebih baru.

```typescript theme={null}
type SDKThinkingTokensMessage = {
  type: "system";
  subtype: "thinking_tokens";
  estimated_tokens: number;
  estimated_tokens_delta: number;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkfilespersistedevent">
  `SDKFilesPersistedEvent`
</h3>

Dipancarkan ketika checkpoint file dipersistenkan ke disk.

```typescript theme={null}
type SDKFilesPersistedEvent = {
  type: "system";
  subtype: "files_persisted";
  files: { filename: string; file_id: string }[];
  failed: { filename: string; error: string }[];
  processed_at: string;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkratelimitevent">
  `SDKRateLimitEvent`
</h3>

Dipancarkan ketika sesi mengalami batas laju.

```typescript theme={null}
type SDKRateLimitEvent = {
  type: "rate_limit_event";
  rate_limit_info: {
    status: "allowed" | "allowed_warning" | "rejected";
    resetsAt?: number;
    utilization?: number;
    errorCode?: "credits_required";
    canUserPurchaseCredits?: boolean;
    hasChargeableSavedPaymentMethod?: boolean;
  };
  uuid: UUID;
  session_id: string;
};
```

Ketika `errorCode` adalah `"credits_required"`, penolakan berasal dari langganan claude.ai yang penggunaan yang disertakan sudah habis, dan sesi tidak dapat dilanjutkan sampai pengguna membeli kredit penggunaan. `canUserPurchaseCredits` menunjukkan apakah pengguna yang diautentikasi dapat membeli kredit untuk akun, dan `hasChargeableSavedPaymentMethod` menunjukkan apakah metode pembayaran yang disimpan ada di file. Ketiga field ini tidak ada pada acara batas laju yang bukan penolakan yang diperlukan kredit. Memerlukan Claude Code v2.1.181 atau lebih baru.

<h3 id="sdklocalcommandoutputmessage">
  `SDKLocalCommandOutputMessage`
</h3>

Output dari perintah slash lokal (misalnya, `/voice` atau `/usage`). Ditampilkan sebagai teks gaya asisten dalam transkrip.

```typescript theme={null}
type SDKLocalCommandOutputMessage = {
  type: "system";
  subtype: "local_command_output";
  content: string;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkcommandschangedmessage">
  `SDKCommandsChangedMessage`
</h3>

Dipancarkan ketika set perintah yang tersedia berubah di tengah sesi, seperti ketika skills ditemukan saat agen memasuki subdirektori. Array `commands` adalah daftar lengkap yang diperbarui, jadi ganti daftar perintah yang di-cache dengan payload ini. Memanggil `supportedCommands()` lagi tidak setara: metode itu mengembalikan snapshot yang ditangkap saat inisialisasi dan tidak mencerminkan perubahan di tengah sesi.

```typescript theme={null}
type SDKCommandsChangedMessage = {
  type: "system";
  subtype: "commands_changed";
  commands: SlashCommand[];
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkpromptsuggestionmessage">
  `SDKPromptSuggestionMessage`
</h3>

Dipancarkan setelah setiap putaran ketika `promptSuggestions` diaktifkan. Berisi prompt pengguna berikutnya yang diprediksi.

```typescript theme={null}
type SDKPromptSuggestionMessage = {
  type: "prompt_suggestion";
  suggestion: string;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkconversationresetmessage">
  `SDKConversationResetMessage`
</h3>

Dipancarkan ketika percakapan sesi diganti tanpa mengakhiri sesi, seperti setelah `/clear`, pada exit plan-mode, atau ketika percakapan segar dimulai. Pasang transkrip kosong di bawah `new_conversation_id` dan buang judul sesi yang di-cache.

```typescript theme={null}
type SDKConversationResetMessage = {
  type: "conversation_reset";
  new_conversation_id: UUID;
  uuid: UUID;
  session_id: string;
};
```

Pengetikan yang dipublikasikan SDK mendeklarasikan `SDKConversationResetMessage` dalam Claude Code v2.1.203 dan lebih baru. Sebelum v2.1.203, `SDKMessage` mereferensikan tipe tanpa mendeklarasikannya, jadi penyempitan pada `type === "conversation_reset"` gagal untuk typecheck ketika `skipLibCheck` dinonaktifkan.

<h3 id="aborterror">
  `AbortError`
</h3>

Kelas error kustom untuk operasi abort.

```typescript theme={null}
class AbortError extends Error {}
```

<h2 id="sandbox-configuration">
  Konfigurasi Sandbox
</h2>

<h3 id="sandboxsettings">
  `SandboxSettings`
</h3>

Konfigurasi untuk perilaku sandbox. Gunakan ini untuk mengaktifkan sandboxing perintah dan mengonfigurasi pembatasan jaringan secara terprogram.

```typescript theme={null}
type SandboxSettings = {
  enabled?: boolean;
  failIfUnavailable?: boolean;
  autoAllowBashIfSandboxed?: boolean;
  excludedCommands?: string[];
  allowUnsandboxedCommands?: boolean;
  network?: SandboxNetworkConfig;
  filesystem?: SandboxFilesystemConfig;
  ignoreViolations?: Record<string, string[]>;
  enableWeakerNestedSandbox?: boolean;
  ripgrep?: { command: string; args?: string[] };
};
```

| Properti                    | Tipe                                                  | Default     | Deskripsi                                                                                                                                                                                                                               |
| :-------------------------- | :---------------------------------------------------- | :---------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `enabled`                   | `boolean`                                             | `false`     | Aktifkan mode sandbox untuk eksekusi perintah                                                                                                                                                                                           |
| `failIfUnavailable`         | `boolean`                                             | `true`      | Berhenti saat startup jika `enabled` adalah `true` tetapi sandbox tidak dapat dimulai. Atur `false` untuk kembali ke eksekusi unsandboxed dengan peringatan di stderr                                                                   |
| `autoAllowBashIfSandboxed`  | `boolean`                                             | `true`      | Auto-approve perintah bash ketika sandbox diaktifkan                                                                                                                                                                                    |
| `excludedCommands`          | `string[]`                                            | `[]`        | Perintah yang selalu bypass pembatasan sandbox (misalnya, `['docker']`). Ini berjalan unsandboxed secara otomatis tanpa keterlibatan model                                                                                              |
| `allowUnsandboxedCommands`  | `boolean`                                             | `true`      | Izinkan model untuk meminta menjalankan perintah di luar sandbox. Ketika `true`, model dapat mengatur `dangerouslyDisableSandbox` dalam input tool, yang jatuh kembali ke [sistem izin](#permissions-fallback-for-unsandboxed-commands) |
| `network`                   | [`SandboxNetworkConfig`](#sandboxnetworkconfig)       | `undefined` | Konfigurasi sandbox spesifik jaringan                                                                                                                                                                                                   |
| `filesystem`                | [`SandboxFilesystemConfig`](#sandboxfilesystemconfig) | `undefined` | Konfigurasi sandbox spesifik filesystem untuk pembatasan baca/tulis                                                                                                                                                                     |
| `ignoreViolations`          | `Record<string, string[]>`                            | `undefined` | Peta kategori pelanggaran ke pola untuk diabaikan (misalnya, `{ file: ['/tmp/*'], network: ['localhost'] }`)                                                                                                                            |
| `enableWeakerNestedSandbox` | `boolean`                                             | `false`     | Aktifkan sandbox bersarang yang lebih lemah untuk kompatibilitas                                                                                                                                                                        |
| `ripgrep`                   | `{ command: string; args?: string[] }`                | `undefined` | Konfigurasi biner ripgrep kustom untuk lingkungan sandbox                                                                                                                                                                               |

<Note>
  Sandbox bergantung pada dukungan platform dan, di Linux, alat seperti `bubblewrap` dan `socat`. Ketika `enabled` adalah `true` dan sandbox tidak dapat dimulai, `query()` melaporkan pesan `result` dengan `subtype: "error_during_execution"` dan alasan dalam `errors`. Untuk panggilan `query()` pesan tunggal, SDK melempar setelah menghasilkan hasil kesalahan itu, jadi bungkus loop dalam blok try untuk melanjutkan melewatinya. Lihat [Menangani hasil](/docs/id/agent-sdk/agent-loop#handle-the-result) untuk kontrak kesalahan.

  Untuk menjalankan unsandboxed sebagai gantinya, atur `failIfUnavailable: false`.
</Note>

<h4 id="example-usage">
  Contoh penggunaan
</h4>

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

try {
  for await (const message of query({
    prompt: "Build and test my project",
    options: {
      sandbox: {
        enabled: true,
        autoAllowBashIfSandboxed: true,
        network: {
          allowLocalBinding: true
        }
      }
    }
  })) {
    if ("result" in message) console.log(message.result);
  }
} catch (error) {
  // A single-shot query() throws after yielding an error result,
  // such as when the sandbox can't start (failIfUnavailable defaults to true).
  console.log(`Session ended with an error: ${error}`);
}
```

<Warning>
  **Keamanan Unix socket:** Opsi `allowUnixSockets` dapat memberikan akses ke layanan sistem yang kuat. Misalnya, mengizinkan `/var/run/docker.sock` secara efektif memberikan akses sistem host penuh melalui API Docker, melewati isolasi sandbox. Hanya izinkan Unix socket yang benar-benar diperlukan dan pahami implikasi keamanan dari masing-masing.
</Warning>

<h3 id="sandboxnetworkconfig">
  `SandboxNetworkConfig`
</h3>

Konfigurasi spesifik jaringan untuk mode sandbox. Pengaturan ini berlaku untuk perintah Bash sandboxed ketika `enabled` adalah `true` dalam [`SandboxSettings`](#sandboxsettings) induk. Mereka tidak membatasi tool WebFetch, yang menggunakan [aturan izin](/docs/id/permissions#webfetch) sebagai gantinya.

```typescript theme={null}
type SandboxNetworkConfig = {
  allowedDomains?: string[];
  deniedDomains?: string[];
  allowManagedDomainsOnly?: boolean;
  allowLocalBinding?: boolean;
  allowUnixSockets?: string[];
  allowAllUnixSockets?: boolean;
  httpProxyPort?: number;
  socksProxyPort?: number;
};
```

| Properti                  | Tipe       | Default     | Deskripsi                                                                                                                                                                                                                                                                                                      |
| :------------------------ | :--------- | :---------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `allowedDomains`          | `string[]` | `[]`        | Nama domain yang dapat diakses proses sandboxed                                                                                                                                                                                                                                                                |
| `deniedDomains`           | `string[]` | `[]`        | Nama domain yang tidak dapat diakses proses sandboxed. Mengambil prioritas atas `allowedDomains`                                                                                                                                                                                                               |
| `allowManagedDomainsOnly` | `boolean`  | `false`     | Hanya pengaturan yang dikelola. Ketika diatur dalam [pengaturan yang dikelola](/docs/id/permissions#managed-settings), hanya entri `allowedDomains` dari pengaturan yang dikelola yang dihormati dan entri dari pengaturan pengguna, proyek, atau lokal diabaikan. Tidak berpengaruh ketika diatur melalui opsi SDK |
| `allowLocalBinding`       | `boolean`  | `false`     | Izinkan proses untuk mengikat ke port lokal (misalnya, untuk dev server)                                                                                                                                                                                                                                       |
| `allowUnixSockets`        | `string[]` | `[]`        | Jalur Unix socket yang dapat diakses proses (misalnya, Docker socket)                                                                                                                                                                                                                                          |
| `allowAllUnixSockets`     | `boolean`  | `false`     | Izinkan akses ke semua Unix socket                                                                                                                                                                                                                                                                             |
| `httpProxyPort`           | `number`   | `undefined` | Port proxy HTTP untuk permintaan jaringan                                                                                                                                                                                                                                                                      |
| `socksProxyPort`          | `number`   | `undefined` | Port proxy SOCKS untuk permintaan jaringan                                                                                                                                                                                                                                                                     |

<Note>
  Proxy sandbox bawaan memberlakukan `allowedDomains` berdasarkan nama host yang diminta dan tidak menghentikan atau memeriksa lalu lintas TLS, sehingga teknik seperti [domain fronting](https://en.wikipedia.org/wiki/Domain_fronting) dapat berpotensi melewatinya. Lihat [Batasan keamanan sandboxing](/docs/id/sandboxing#security-limitations) untuk detail dan [Penyebaran aman](/docs/id/agent-sdk/secure-deployment#traffic-forwarding) untuk mengonfigurasi proxy yang menghentikan TLS.
</Note>

<h3 id="sandboxfilesystemconfig">
  `SandboxFilesystemConfig`
</h3>

Konfigurasi spesifik filesystem untuk mode sandbox.

```typescript theme={null}
type SandboxFilesystemConfig = {
  allowWrite?: string[];
  denyWrite?: string[];
  denyRead?: string[];
};
```

| Properti     | Tipe       | Default | Deskripsi                                        |
| :----------- | :--------- | :------ | :----------------------------------------------- |
| `allowWrite` | `string[]` | `[]`    | Pola jalur file untuk mengizinkan akses tulis ke |
| `denyWrite`  | `string[]` | `[]`    | Pola jalur file untuk menolak akses tulis ke     |
| `denyRead`   | `string[]` | `[]`    | Pola jalur file untuk menolak akses baca ke      |

<h3 id="permissions-fallback-for-unsandboxed-commands">
  Fallback Izin untuk Perintah Unsandboxed
</h3>

Ketika `allowUnsandboxedCommands` diaktifkan, model dapat meminta untuk menjalankan perintah di luar sandbox dengan mengatur `dangerouslyDisableSandbox: true` dalam input tool. Permintaan ini jatuh kembali ke sistem izin yang ada, berarti handler `canUseTool` Anda dipanggil, memungkinkan Anda untuk mengimplementasikan logika otorisasi kustom. Dalam contoh di bawah, `isCommandAuthorized` mewakili pemeriksaan otorisasi yang Anda tentukan.

<Note>
  **`excludedCommands` vs `allowUnsandboxedCommands`:**

  * `excludedCommands`: Daftar statis perintah yang selalu bypass sandbox secara otomatis (misalnya, `['docker']`). Model tidak memiliki kontrol atas ini.
  * `allowUnsandboxedCommands`: Biarkan model memutuskan pada runtime apakah akan meminta eksekusi unsandboxed dengan mengatur `dangerouslyDisableSandbox: true` dalam input tool.
</Note>

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

for await (const message of query({
  prompt: "Deploy my application",
  options: {
    sandbox: {
      enabled: true,
      allowUnsandboxedCommands: true // Model dapat meminta eksekusi unsandboxed
    },
    permissionMode: "default",
    canUseTool: async (tool, input) => {
      // Periksa apakah model meminta untuk bypass sandbox
      if (tool === "Bash" && input.dangerouslyDisableSandbox) {
        // Model meminta untuk menjalankan perintah ini di luar sandbox
        console.log(`Unsandboxed command requested: ${input.command}`);

        if (isCommandAuthorized(input.command)) {
          return { behavior: "allow" as const, updatedInput: input };
        }
        return {
          behavior: "deny" as const,
          message: "Command not authorized for unsandboxed execution"
        };
      }
      return { behavior: "allow" as const, updatedInput: input };
    }
  }
})) {
  if ("result" in message) console.log(message.result);
}
```

Pola ini memungkinkan Anda untuk:

* **Audit permintaan model:** Catat ketika model meminta eksekusi unsandboxed
* **Implementasikan allowlist:** Hanya izinkan perintah tertentu untuk berjalan unsandboxed
* **Tambahkan alur persetujuan:** Memerlukan otorisasi eksplisit untuk operasi istimewa

<Warning>
  Perintah yang berjalan dengan `dangerouslyDisableSandbox: true` memiliki akses sistem penuh. Pastikan handler `canUseTool` Anda memvalidasi permintaan ini dengan hati-hati.

  Jika `permissionMode` diatur ke `bypassPermissions` dan `allowUnsandboxedCommands` diaktifkan, model dapat secara otonom mengeksekusi perintah di luar sandbox tanpa prompt persetujuan apa pun (aturan [`ask`](/docs/id/agent-sdk/permissions#how-permissions-are-evaluated) eksplisit masih memaksa satu). Kombinasi ini secara efektif memungkinkan model untuk melarikan diri dari isolasi sandbox secara diam-diam.
</Warning>

<h2 id="see-also">
  Lihat juga
</h2>

* [Gambaran umum SDK](/docs/id/agent-sdk/overview) - Konsep SDK umum
* [Referensi SDK Python](/docs/id/agent-sdk/python) - Dokumentasi SDK Python
* [Referensi CLI](/docs/id/cli-reference) - Antarmuka baris perintah
* [Alur kerja umum](/docs/id/common-workflows) - Panduan langkah demi langkah
