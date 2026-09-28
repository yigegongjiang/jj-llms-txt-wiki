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
  SDK menggabungkan biner Claude Code asli untuk platform Anda sebagai dependensi opsional seperti `@anthropic-ai/claude-agent-sdk-darwin-arm64`. Sebagian besar instalasi tidak memerlukan instalasi Claude Code terpisah. Versi SDK melacak versi Claude Code yang dibundel. SDK v0.3.191 menggabungkan Claude Code v2.1.191, jadi fitur di halaman ini yang memerlukan versi Claude Code tertentu memerlukan rilis SDK dengan nomor patch yang sama atau lebih baru. Jika pengelola paket Anda melewatkan dependensi opsional, SDK melempar `Native CLI binary for <platform>-<arch> not found`; setel [`pathToClaudeCodeExecutable`](#options) ke biner `claude` yang diinstal secara terpisah sebagai gantinya.

  Jika pengelola paket Anda tidak menerapkan bidang `libc` npm, seperti yang tidak dilakukan Yarn 1.x, Anda mendapatkan paket platform glibc dan musl di Linux, kira-kira menggandakan ukuran instalasi. Pada Agent SDK v0.2.141 atau lebih baru, SDK masih meluncurkan varian yang benar. Untuk memulihkan ruang dalam gambar kontainer, hapus paket platform yang tidak cocok dengan libc tempat aplikasi Anda berjalan; untuk runtime glibc pada x64, itu adalah `rm -rf node_modules/@anthropic-ai/claude-agent-sdk-linux-x64-musl`. Pada mesin pengembangan penghapusan bersifat sementara, karena Yarn menginstal ulang paket pada perubahan dependensi berikutnya.
</Note>

<h3 id="compile-to-a-single-executable">
  Kompilasi ke executable tunggal
</h3>

Ketika Anda mengompilasi aplikasi Anda menjadi executable file tunggal dengan `bun build --compile`, SDK tidak dapat menyelesaikan biner CLI yang dibundel saat runtime. `require.resolve` tidak berfungsi di dalam filesystem virtual `$bunfs` executable yang dikompilasi, jadi SDK melempar `Native CLI binary for <platform>-<arch> not found`.

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

<h4 id="parameters-2">
  Parameter
</h4>

| Parameter             | Tipe                  | Deskripsi                                                                                                                                                                    |
| :-------------------- | :-------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `options`             | [`Options`](#options) | Objek konfigurasi opsional. Sama dengan parameter `options` ke `query()`                                                                                                     |
| `initializeTimeoutMs` | `number`              | Waktu maksimum dalam milidetik untuk menunggu inisialisasi subprocess. Default ke `60000`. Jika inisialisasi tidak selesai tepat waktu, promise ditolak dengan error timeout |

<h4 id="returns-2">
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
  extras?: { annotations?: ToolAnnotations; searchHint?: string; alwaysLoad?: boolean }
): SdkMcpToolDefinition<Schema>;
```

<h4 id="parameters-3">
  Parameter
</h4>

| Parameter     | Tipe                                                                                                   | Deskripsi                                                                                                                                                                                                                                                                                                                       |
| :------------ | :----------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `name`        | `string`                                                                                               | Nama tool                                                                                                                                                                                                                                                                                                                       |
| `description` | `string`                                                                                               | Deskripsi tentang apa yang dilakukan tool                                                                                                                                                                                                                                                                                       |
| `inputSchema` | `Schema extends AnyZodRawShape`                                                                        | Skema Zod yang mendefinisikan parameter input tool (mendukung Zod 3 dan Zod 4)                                                                                                                                                                                                                                                  |
| `handler`     | `(args, extra) => Promise<`[`CallToolResult`](#calltoolresult)`>`                                      | Fungsi asinkron yang mengeksekusi logika tool                                                                                                                                                                                                                                                                                   |
| `extras`      | `{ annotations?: `[`ToolAnnotations`](#toolannotations)`; searchHint?: string; alwaysLoad?: boolean }` | Anotasi opsional. `annotations` memberikan petunjuk perilaku MCP kepada klien. `searchHint` adalah frasa kemampuan satu baris yang ditampilkan dalam daftar tool yang ditunda ketika [pencarian tool](/docs/id/agent-sdk/tool-search) aktif. `alwaysLoad: true` menjaga skema lengkap tool ini dalam prompt awal daripada menundanya |

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
  instructions?: string;
  tools?: Array<SdkMcpToolDefinition<any>>;
  alwaysLoad?: boolean;
  timeout?: number;
}): McpSdkServerConfigWithInstance;
```

<h4 id="parameters-4">
  Parameter
</h4>

| Parameter              | Tipe                          | Deskripsi                                                                                                                                                                                                                                                                                   |
| :--------------------- | :---------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `options.name`         | `string`                      | Nama server MCP                                                                                                                                                                                                                                                                             |
| `options.version`      | `string`                      | String versi opsional                                                                                                                                                                                                                                                                       |
| `options.instructions` | `string`                      | Instruksi server opsional, dikembalikan dari `initialize` dan ditampilkan ke model sebagai blok instruksi MCP                                                                                                                                                                               |
| `options.tools`        | `Array<SdkMcpToolDefinition>` | Array definisi tool yang dibuat dengan [`tool()`](#tool)                                                                                                                                                                                                                                    |
| `options.alwaysLoad`   | `boolean`                     | Ketika `true`, setiap tool dari server ini tetap dalam prompt awal dan tidak pernah ditunda di belakang [pencarian tool](/docs/id/agent-sdk/tool-search). Menggabungkan dengan `alwaysLoad` per-tool dalam [`tool()`](#tool)                                                                     |
| `options.timeout`      | `number`                      | Timeout dalam milidetik untuk panggilan tool server ini. Claude Code menerapkannya ke server ini sebagai pengganti [`MCP_TOOL_TIMEOUT`](/docs/id/env-vars). Lewatkan seluruh angka minimal 1000. Claude Code mengabaikan nilai lainnya. Memerlukan TypeScript Agent SDK v0.3.248 atau lebih baru |

<h3 id="listsessions">
  `listSessions()`
</h3>

Menemukan dan membuat daftar sesi masa lalu dengan metadata ringan. Filter berdasarkan direktori proyek atau buat daftar sesi di semua proyek.

```typescript theme={null}
function listSessions(options?: ListSessionsOptions): Promise<SDKSessionInfo[]>;
```

<h4 id="parameters-5">
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

<h4 id="example-2">
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

<h4 id="parameters-6">
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

| Properti             | Tipe                    | Deskripsi                                                                                                                                                                                                                                                                         |
| :------------------- | :---------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`               | `"user" \| "assistant"` | Peran pesan                                                                                                                                                                                                                                                                       |
| `uuid`               | `string`                | Pengenal pesan unik                                                                                                                                                                                                                                                               |
| `session_id`         | `string`                | Sesi yang pesan ini milik                                                                                                                                                                                                                                                         |
| `message`            | `unknown`               | Payload pesan mentah dari transkrip                                                                                                                                                                                                                                               |
| `parent_tool_use_id` | `string \| null`        | Untuk pesan subagent, `tool_use_id` dari panggilan tool `Agent` atau `Skill` yang memicu subagent. `null` untuk pesan sesi utama dan sesi yang lebih lama                                                                                                                         |
| `parent_agent_id`    | `string \| null`        | Untuk pesan dari [subagent bersarang](/docs/id/sub-agents#let-subagents-spawn-their-own-subagents), `agentId` dari subagent yang memicunya. `null` untuk pesan sesi utama, pesan dari subagent tingkat atas, dan sesi yang lebih lama. Memerlukan Claude Code v2.1.202 atau lebih baru |

<h4 id="example-3">
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

<h4 id="parameters-7">
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

<h4 id="parameters-8">
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

<h4 id="parameters-9">
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
  Fungsi ini alpha dan API-nya mungkin berubah sebelum stabilisasi.
</Note>

Snapshot berbeda dari apa yang diterapkan sesi live `query()`:

* **`policyHelper`**: `resolveSettings()` membaca sumber MDM, termasuk plist macOS dan Windows HKLM/HKCU, tetapi tidak mengeksekusi subprocess `policyHelper` yang dikonfigurasi admin.
* **Pengaturan yang dikelola server**: `resolveSettings()` tidak mengambil [pengaturan yang dikelola server](/docs/id/server-managed-settings#fetch-and-caching-behavior). Lewatkan mereka sebagai `options.serverManagedSettings` untuk menyertakannya.
* **`defaultMode`**: snapshot mengembalikan `permissions.defaultMode` apa adanya dari setiap tingkat, sehingga dapat mencakup nilai `'auto'` dan `'bypassPermissions'` dari pengaturan proyek dan lokal, yang [sesi live abaikan](/docs/id/permission-modes#which-mode-a-session-starts-in).

```typescript theme={null}
function resolveSettings(
  options?: ResolveSettingsOptions
): Promise<ResolvedSettings>;
```

<h4 id="parameters-10">
  Parameter
</h4>

`resolveSettings()` menerima objek opsi tunggal. Semua field bersifat opsional.

| Parameter                       | Tipe                                  | Default         | Deskripsi                                                                                                                                                                                                                                                                                                                                                |
| :------------------------------ | :------------------------------------ | :-------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `options.cwd`                   | `string`                              | `process.cwd()` | Direktori untuk menyelesaikan pengaturan proyek dan lokal relatif terhadap                                                                                                                                                                                                                                                                               |
| `options.settingSources`        | [`SettingSource`](#settingsource)`[]` | Semua sumber    | Sumber filesystem mana yang akan dimuat. Lewatkan `[]` untuk melewati pengaturan pengguna, proyek, dan lokal. [Kebijakan yang dikelola endpoint](/docs/id/managed-settings#delivery-mechanisms) dimuat dalam semua kasus. `resolveSettings()` menyertakan pengaturan yang dikelola server hanya ketika Anda melewatkan `options.serverManagedSettings`        |
| `options.managedSettings`       | `Settings`                            | `undefined`     | Pengaturan tingkat kebijakan yang disediakan oleh host penyematan. Mengikuti aturan yang sama dengan [`managedSettings` dalam `Options`](#options), kecuali bahwa `resolveSettings()` tidak mengeksekusi [`policyHelper`](/docs/id/settings-reference#policyhelper) yang dikonfigurasi, sehingga snapshot dapat mencakup pengaturan yang dijatuhkan sesi live |
| `options.serverManagedSettings` | `Settings`                            | `undefined`     | Payload pengaturan yang dikelola server dari `/api/claude_code/settings`. Kunci non-pembatasan melewati tanpa filter                                                                                                                                                                                                                                     |

<h4 id="return-type-resolvedsettings">
  Tipe pengembalian: `ResolvedSettings`
</h4>

`resolveSettings()` mengembalikan objek yang menjelaskan pengaturan yang digabungkan dan sumber yang berkontribusi pada setiap kunci.

| Properti     | Tipe                                                | Deskripsi                                                                                         |
| :----------- | :-------------------------------------------------- | :------------------------------------------------------------------------------------------------ |
| `effective`  | `Settings`                                          | Pengaturan yang digabungkan setelah menerapkan semua sumber yang diaktifkan dalam urutan preseden |
| `provenance` | `Partial<Record<keyof Settings, ProvenanceEntry>>`  | Untuk setiap kunci tingkat atas dalam `effective`, sumber mana yang memasok nilai                 |
| `sources`    | `Array<{ source, settings, path?, policyOrigin? }>` | Pengaturan mentah per-sumber, diurutkan dari preseden terendah hingga tertinggi                   |

<h4 id="example-4">
  Contoh
</h4>

Contoh di bawah ini menyelesaikan pengaturan untuk direktori proyek dan mencetak sumber yang mengontrol periode pembersihan. Pada mesin di mana tidak ada file pengaturan yang menetapkan `cleanupPeriodDays`, kedua baris yang dicetak menunjukkan `undefined` untuk nilai, yang merupakan output yang diharapkan daripada error.

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
  Jenis
</h2>

<h3 id="options">
  `Options`
</h3>

Objek konfigurasi untuk fungsi `query()`.

| Properti                          | Jenis                                                                                                                                                                                                          | Default                                          | Deskripsi                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| :-------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `abortController`                 | `AbortController`                                                                                                                                                                                              | `new AbortController()`                          | Pengontrol untuk membatalkan operasi                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `additionalDirectories`           | `string[]`                                                                                                                                                                                                     | `[]`                                             | Direktori tambahan yang dapat diakses Claude. SDK meneruskan setiap entri ke Claude Code sebagai `--add-dir`, jadi dengan pengaturan `project` sumber Claude Code juga [memuat skills, commands, dan subagents direktori](/docs/id/permissions#additional-directories-grant-file-access-not-configuration)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `agent`                           | `string`                                                                                                                                                                                                       | `undefined`                                      | Nama agent untuk thread utama. Agent harus didefinisikan dalam opsi `agents` atau dalam pengaturan                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `agents`                          | `Record<string, [`AgentDefinition`](#agentdefinition)>`                                                                                                                                                        | `undefined`                                      | Tentukan subagents secara terprogram                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `agentProgressSummaries`          | `boolean`                                                                                                                                                                                                      | `false`                                          | Ketika `true`, hasilkan ringkasan kemajuan satu baris untuk subagents dan teruskan pada acara [`task_progress`](#sdktaskprogressmessage) melalui bidang `summary`. Berlaku untuk subagents foreground dan background                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `allowDangerouslySkipPermissions` | `boolean`                                                                                                                                                                                                      | `false`                                          | Aktifkan bypass permissions. Diperlukan saat menggunakan `permissionMode: 'bypassPermissions'`, saat startup atau kemudian melalui `setPermissionMode()`. Lihat [plan mode](/docs/id/agent-sdk/permissions#plan-mode-plan) untuk cara interaksinya dengan `permissionMode: 'plan'`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `allowedTools`                    | `string[]`                                                                                                                                                                                                     | `[]`                                             | Tools untuk auto-approve tanpa prompt. Ini tidak membatasi Claude hanya pada tools ini. Jika Anda menyebutkan salah satu [task-tracking tools](/docs/id/agent-sdk/todo-tracking#model-availability) di sini, Claude Code juga opt-in sesi. Tools lain yang tidak terdaftar jatuh ke `permissionMode` dan `canUseTool`. Gunakan `disallowedTools` untuk memblokir tools. Lihat [Permissions](/docs/id/agent-sdk/permissions#allow-and-deny-rules)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `betas`                           | [`SdkBeta`](#sdkbeta)`[]`                                                                                                                                                                                      | `[]`                                             | Aktifkan fitur beta                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `canUseTool`                      | [`CanUseTool`](#canusetool)                                                                                                                                                                                    | `undefined`                                      | Fungsi permission kustom, dipanggil hanya ketika [permission flow](/docs/id/agent-sdk/permissions#how-permissions-are-evaluated) jatuh ke prompt. Tidak dipanggil untuk panggilan yang di-auto-approve oleh `allowedTools`, allow rules, atau `permissionMode`. Sebuah allow rule tidak pre-approve [actions no mode auto-approves](/docs/id/permission-modes#actions-no-mode-auto-approves). Lihat [`CanUseTool`](#canusetool) untuk detail                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `continue`                        | `boolean`                                                                                                                                                                                                      | `false`                                          | Lanjutkan percakapan terbaru                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `cwd`                             | `string`                                                                                                                                                                                                       | `process.cwd()`                                  | Direktori kerja saat ini                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `debug`                           | `boolean`                                                                                                                                                                                                      | `false`                                          | Aktifkan mode debug untuk proses Claude Code                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `debugFile`                       | `string`                                                                                                                                                                                                       | `undefined`                                      | Tulis debug logs ke path file tertentu. Secara implisit mengaktifkan mode debug                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `disallowedTools`                 | `string[]`                                                                                                                                                                                                     | `[]`                                             | Tools untuk ditolak. Nama bare seperti `"Bash"` menghapus tool dari konteks Claude. Aturan scoped seperti `"Bash(rm *)"` membiarkan tool tersedia dan menolak panggilan yang cocok di setiap permission mode, termasuk `bypassPermissions`, untuk perintah [seperti yang ditulis](/docs/id/permissions#bash-rule-limits). Lihat [Permissions](/docs/id/agent-sdk/permissions#allow-and-deny-rules)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `effort`                          | `'low' \| 'medium' \| 'high' \| 'xhigh' \| 'max'`                                                                                                                                                              | `undefined`                                      | Mengontrol berapa banyak usaha yang Claude keluarkan dalam responsnya. Bekerja dengan adaptive thinking untuk memandu kedalaman thinking. Lihat [adjust the effort level](/docs/id/model-config#adjust-effort-level)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `enableFileCheckpointing`         | `boolean`                                                                                                                                                                                                      | `false`                                          | Aktifkan file change tracking untuk rewinding. Lihat [File checkpointing](/docs/id/agent-sdk/file-checkpointing)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `env`                             | `Record<string, string \| undefined>`                                                                                                                                                                          | `process.env`                                    | Variabel lingkungan. Ketika diatur, ini menggantikan lingkungan subprocess alih-alih merge dengan `process.env`, jadi teruskan `{ ...process.env, YOUR_VAR: 'value' }` untuk menjaga variabel yang diwariskan seperti `PATH`. Lihat [Handle slow or stalled API responses](#handle-slow-or-stalled-api-responses) untuk contoh pola ini, dan [Environment variables](/docs/id/env-vars) untuk variabel yang dibaca CLI yang mendasar. Atur `CLAUDE_AGENT_SDK_CLIENT_APP` untuk mengidentifikasi aplikasi Anda di header User-Agent                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `executable`                      | `'bun' \| 'deno' \| 'node'`                                                                                                                                                                                    | Auto-detected                                    | JavaScript runtime untuk digunakan                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `executableArgs`                  | `string[]`                                                                                                                                                                                                     | `[]`                                             | Argumen untuk diteruskan ke executable                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `extraArgs`                       | `Record<string, string \| null>`                                                                                                                                                                               | `{}`                                             | Argumen tambahan                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `fallbackModel`                   | `string`                                                                                                                                                                                                       | `undefined`                                      | Model untuk digunakan jika model utama gagal. Menerima daftar yang dipisahkan koma. Untuk urutan dan batas, lihat [Fallback model chains](/docs/id/model-config#fallback-model-chains). Untuk panduan, lihat [Choose a model](/docs/id/agent-sdk/configuration#choose-a-model)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `forkSession`                     | `boolean`                                                                                                                                                                                                      | `false`                                          | Ketika melanjutkan dengan `resume`, fork ke session ID baru alih-alih melanjutkan session asli                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `forwardSubagentText`             | `boolean`                                                                                                                                                                                                      | `false`                                          | Teruskan blok teks dan thinking subagent sebagai pesan assistant dan user dengan `parent_tool_use_id` diatur, sehingga konsumen dapat merender transkrip bersarang. Tanpa opsi ini, Claude Code memancarkan blok `tool_use` dan `tool_result` subagent tetapi bukan teks atau thinking. Pesan dari subagents di setiap kedalaman nesting diteruskan pada Claude Code v2.1.219 dan lebih baru; sebelum v2.1.219, hanya pesan dari subagents depth-1 yang muncul. Pesan dari subagents yang forked skill spawn, dan dari nested forked skills, memerlukan v2.1.275 atau lebih baru                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `hooks`                           | `Partial<Record<`[`HookEvent`](#hookevent)`, `[`HookCallbackMatcher`](#hookcallbackmatcher)`[]>>`                                                                                                              | `{}`                                             | Hook callbacks untuk acara                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `includeHookEvents`               | `boolean`                                                                                                                                                                                                      | `false`                                          | Sertakan hook lifecycle events dalam message stream sebagai [`SDKHookStartedMessage`](#sdkhookstartedmessage), [`SDKHookProgressMessage`](#sdkhookprogressmessage), dan [`SDKHookResponseMessage`](#sdkhookresponsemessage). Lifecycle events untuk `SessionStart` dan `Setup` hooks selalu disertakan dan tidak memerlukan opsi ini. Beberapa hook events, seperti `Notification`, `SessionEnd`, `PreCompact`, dan `PostCompact`, tidak pernah menghasilkan `SDKHookStartedMessage`, bahkan dengan opsi ini. Untuk acara tersebut, Claude Code masih memancarkan `SDKHookProgressMessage` saat command hook yang berjalan lebih dari satu detik menghasilkan output, dan memancarkan `SDKHookResponseMessage` hanya ketika hook [yang berjalan di background](/docs/id/hooks#run-hooks-in-the-background) selesai                                                                                                                                                                                                                                                                                                                                                                                                    |
| `includePartialMessages`          | `boolean`                                                                                                                                                                                                      | `false`                                          | Sertakan partial message events                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `loadTimeoutMs`                   | `number`                                                                                                                                                                                                       | `60000`                                          | *Alpha.* Timeout dalam milidetik untuk setiap panggilan `sessionStore.load()` dan `sessionStore.listSubkeys()` selama resume materialization. Jika adapter tidak settle dalam jendela ini, query gagal alih-alih hang. Diabaikan ketika `sessionStore` tidak diatur                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `managedSettings`                 | `Settings`                                                                                                                                                                                                     | `undefined`                                      | Pengaturan policy-tier yang host process Anda sediakan untuk spawned session. Pada mesin dengan managed settings yang di-deploy admin, Claude Code mengabaikan ini kecuali sumber managed priority tertinggi admin menetapkan `parentSettingsBehavior: 'merge'`, dan tidak pernah merge saat [`policyHelper`](/docs/id/settings-reference#policyhelper) menyediakan managed settings. Nilai merged melewati filter restrictive-only; [Restrict parent settings](/docs/id/claude-apps-gateway#restrict-parent-settings) mencakup apa yang filter terima dan kunci `allowManaged*Only`. Host yang menetapkan [`CLAUDE_CODE_PROVIDER_MANAGED_BY_HOST`](/docs/id/env-vars) memiliki tiga kunci yang dibaca langsung dari payload ini: [model configuration](/docs/id/model-config#restrict-model-selection) pada Claude Code v2.1.222 atau lebih baru, [`modelPricing`](/docs/id/settings-reference#modelpricing) ketika tidak ada sumber managed yang menetapkannya pada v2.1.246 atau lebih baru, dan entri `ENABLE_TOOL_SEARCH` env-nya pada v2.1.247 atau lebih baru                                                                                                                                                                      |
| `maxBudgetUsd`                    | `number`                                                                                                                                                                                                       | `undefined`                                      | Hentikan query ketika estimasi biaya sisi klien mencapai nilai USD ini. Hanya menghitung pengeluaran call sendiri; totals yang dipulihkan dari session yang dilanjutkan tidak dihitung. Untuk caveat akurasi dan perilaku reset, lihat [Track cost and usage](/docs/id/agent-sdk/cost-tracking)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `maxThinkingTokens`               | `number`                                                                                                                                                                                                       | `undefined`                                      | *Deprecated:* Gunakan `thinking` sebagai gantinya. Token maksimal untuk proses thinking                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `maxTurns`                        | `number`                                                                                                                                                                                                       | `undefined`                                      | Maksimal agentic turns (tool-use round trips)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `mcpServers`                      | `Record<string, [`McpServerConfig`](#mcpserverconfig)>`                                                                                                                                                        | `{}`                                             | Konfigurasi MCP server                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `model`                           | `string`                                                                                                                                                                                                       | Default dari CLI                                 | Alias model Claude atau nama model lengkap. Lihat [accepted values and provider-specific IDs](/docs/id/model-config#available-models)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `onElicitation`                   | `(request: ElicitationRequest, options: { signal: AbortSignal }) => Promise<ElicitationResult>`                                                                                                                | `undefined`                                      | Callback untuk menangani MCP elicitation requests. Dipanggil ketika MCP server meminta input pengguna dan tidak ada hook yang menanganinya terlebih dahulu. Ketika tidak disediakan, unhandled elicitation requests ditolak secara otomatis                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `outputFormat`                    | `{ type: 'json_schema', schema: JSONSchema }`                                                                                                                                                                  | `undefined`                                      | Tentukan format output untuk hasil agent. Lihat [Structured outputs](/docs/id/agent-sdk/structured-outputs) untuk detail                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `outputStyle`                     | `string`                                                                                                                                                                                                       | `undefined`                                      | Bukan bidang `Options`. Atur `outputStyle` dalam objek [`settings`](/docs/id/settings) inline atau file settings. Lihat [Activate an output style](/docs/id/agent-sdk/modifying-system-prompts#activate-an-output-style)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `pathToClaudeCodeExecutable`      | `string`                                                                                                                                                                                                       | Auto-resolved dari bundled native binary         | Path ke Claude Code executable. Hanya diperlukan jika optional dependencies dilewati selama install atau platform Anda tidak dalam set yang didukung                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `permissionMode`                  | [`PermissionMode`](#permissionmode)                                                                                                                                                                            | `'default'`                                      | Permission mode untuk session                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `permissionPromptToolName`        | `string`                                                                                                                                                                                                       | `undefined`                                      | Nama MCP tool untuk permission prompts                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `permissionPrompts`               | `'host' \| 'none'`                                                                                                                                                                                             | `'host'`                                         | Siapa yang menjawab permission prompts: `'host'` merutekan mereka ke callback [`canUseTool`](#canusetool) Anda atau tool `permissionPromptToolName`, dan `'none'` [menolak panggilan yang akan diprompt](/docs/id/agent-sdk/permissions#how-permissions-are-evaluated). Memerlukan Claude Code v2.1.259 atau lebih baru                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `persistSession`                  | `boolean`                                                                                                                                                                                                      | `true`                                           | Ketika `false`, menonaktifkan session persistence ke disk. Sessions tidak dapat dilanjutkan kemudian                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `planModeInstructions`            | `string`                                                                                                                                                                                                       | `undefined`                                      | Instruksi workflow kustom untuk plan mode. Ketika `permissionMode` adalah `'plan'`, string ini menggantikan badan workflow plan-mode default. CLI masih membungkusnya dengan preamble enforcement read-only dan footer protokol ExitPlanMode                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `plugins`                         | [`SdkPluginConfig`](#sdkpluginconfig)`[]`                                                                                                                                                                      | `[]`                                             | Muat custom plugins dari local paths. Lihat [Plugins](/docs/id/agent-sdk/plugins) untuk detail                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `projectConfigRoot`               | `string`                                                                                                                                                                                                       | `undefined`                                      | Absolute path dari trusted checkout yang `cwd` adalah worktree-nya. Claude Code membaca project settings, `.mcp.json`, dan project's `.claude/` commands, agents, skills, workflows, routines, dan output styles dari direktori ini alih-alih `cwd`, dan menetapkan `CLAUDE_PROJECT_DIR` ke dalamnya. Hooks, helper scripts seperti `apiKeyHelper`, dan stdio MCP servers dimulai dengan direktori ini sebagai working directory mereka. File `CLAUDE.md` dan `.claude/rules/` masih load dari `cwd`. Memerlukan Claude Code v2.1.275 atau lebih baru                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `promptSuggestions`               | `boolean`                                                                                                                                                                                                      | `false`                                          | Aktifkan prompt suggestions. Setelah turn, Claude Code memancarkan pesan `prompt_suggestion` yang membawa predicted next user prompt. Claude Code tidak menghasilkan suggestion untuk beberapa turns, seperti saat akun Anda mendekati atau mencapai usage limit. Lihat [When Claude Code skips suggestions](/docs/id/interactive-mode#when-claude-code-skips-suggestions)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `resume`                          | `string`                                                                                                                                                                                                       | `undefined`                                      | Session ID untuk dilanjutkan                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `resumeDropsTurn`                 | `string`                                                                                                                                                                                                       | `undefined`                                      | Dengan `resumeSessionAt`: prompt UUID dari turn yang truncating resume bermaksud untuk discard. Claude Code menolak resume ketika discarded range berisi apa pun yang tidak dapat dikaitkan dengan turn itu, seperti absorbed queued messages atau task notifications, dan menyebutkan flag `--resume-drops-turn` dalam pesan penolakan. Hanya Agent SDK dan print-mode resumes yang membaca pasangan. Memerlukan Claude Code v2.1.223 atau lebih baru                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `resumeSessionAt`                 | `string`                                                                                                                                                                                                       | `undefined`                                      | Lanjutkan session pada message UUID tertentu                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `sandbox`                         | [`SandboxSettings`](#sandboxsettings)                                                                                                                                                                          | `undefined`                                      | Konfigurasi perilaku sandbox secara terprogram. Lihat [Sandbox settings](#sandboxsettings) untuk detail                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `sessionId`                       | `string`                                                                                                                                                                                                       | Auto-generated                                   | Gunakan UUID tertentu untuk session alih-alih auto-generating satu                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `sessionStore`                    | [`SessionStore`](/docs/id/agent-sdk/session-storage#the-sessionstore-interface)                                                                                                                                     | `undefined`                                      | Mirror session transcripts ke backend eksternal sehingga host lain dapat melanjutkannya. Lihat [Persist sessions to external storage](/docs/id/agent-sdk/session-storage)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `sessionStoreFlush`               | `'batched' \| 'eager'`                                                                                                                                                                                         | `'batched'`                                      | *Alpha.* Flush mode untuk `sessionStore`. Diabaikan ketika `sessionStore` tidak diatur                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `settings`                        | `string \| Settings`                                                                                                                                                                                           | `undefined`                                      | Objek [settings](/docs/id/settings) inline, path file settings, atau string JSON inline. Mengisi layer flag-settings dalam [precedence order](/docs/id/settings#settings-precedence). Ubah saat runtime dengan [`applyFlagSettings()`](#applyflagsettings)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `settingSources`                  | [`SettingSource`](#settingsource)`[]`                                                                                                                                                                          | CLI defaults (all sources)                       | Kontrol filesystem settings mana yang akan dimuat. Teruskan `[]` untuk menonaktifkan user, project, dan local settings. [Endpoint-managed policy](/docs/id/managed-settings#delivery-mechanisms) dimuat terlepas; server-managed settings diambil ketika session mengautentikasi dengan kredensial organisasi pada [eligible configuration](/docs/id/server-managed-settings#platform-availability). Lihat [Use Claude Code features](/docs/id/agent-sdk/claude-code-features#what-settingsources-does-not-control)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `skills`                          | `string[] \| 'all'`                                                                                                                                                                                            | `undefined`                                      | Skills yang tersedia untuk session. Teruskan `'all'` untuk mengaktifkan setiap skill yang ditemukan, atau daftar nama skill. Teruskan nama yang tepat saja. Pada Agent SDK v0.3.221 atau lebih baru, SDK menolak nama yang malformed dan wildcard-form dengan error sebelum memulai proses Claude Code. Ketika diatur, SDK menambahkan Skill tool ke `allowedTools` secara otomatis. Jika Anda juga meneruskan `tools`, sertakan `'Skill'` dalam daftar itu. Lihat [Skills](/docs/id/agent-sdk/skills)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `spawnClaudeCodeProcess`          | `(options: SpawnOptions) => SpawnedProcess`                                                                                                                                                                    | `undefined`                                      | Fungsi kustom untuk spawn proses Claude Code. Gunakan untuk menjalankan Claude Code di VMs, containers, atau remote environments                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `stderr`                          | `(data: string) => void`                                                                                                                                                                                       | `undefined`                                      | Callback untuk output stderr                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `strictMcpConfig`                 | `boolean`                                                                                                                                                                                                      | `false`                                          | Gunakan hanya servers yang diteruskan dalam `mcpServers` dan abaikan project `.mcp.json`, user settings, plugin-provided MCP servers, dan [claude.ai connectors](/docs/id/mcp#use-mcp-servers-from-claude-ai)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `systemPrompt`                    | `string \| string[] \| { type: 'custom'; prompt: string \| string[]; snapshot?: boolean } \| { type: 'preset'; preset: 'claude_code'; append?: string; excludeDynamicSections?: boolean; snapshot?: boolean }` | `undefined` (minimal prompt)                     | Konfigurasi system prompt. Teruskan string untuk custom prompt, atau `{ type: 'preset', preset: 'claude_code' }` untuk menggunakan system prompt Claude Code. Teruskan array strings dengan konstanta `SYSTEM_PROMPT_DYNAMIC_BOUNDARY` yang diekspor antara bagian static dan per-request untuk [cache bagian static dari custom prompt](/docs/id/agent-sdk/modifying-system-prompts#cache-the-static-part-of-a-custom-prompt). Ketika menggunakan bentuk preset object, tambahkan `append` untuk memperluas dengan instruksi tambahan, dan atur `excludeDynamicSections: true` untuk memindahkan per-session context ke first user message untuk [better prompt-cache reuse across machines](/docs/id/agent-sdk/modifying-system-prompts#improve-prompt-caching-across-users-and-machines). Atur `snapshot: false` untuk rebuild prompt pada setiap request alih-alih [reusing prompt yang session catat pada first request-nya](/docs/id/agent-sdk/modifying-system-prompts#change-the-prompt-of-an-existing-session). Untuk mengatur `snapshot` pada custom prompt, teruskan bentuk `{ type: 'custom', prompt }`. Bentuk `{ type: 'custom' }` dan bidang `snapshot` memerlukan TypeScript Agent SDK v0.3.257 atau lebih baru |
| `taskBudget`                      | `{ total: number }`                                                                                                                                                                                            | `undefined`                                      | *Alpha.* API-side task budget dalam tokens. Ketika diatur, model diberitahu budget token sisanya sehingga dapat pace tool use dan wrap up sebelum limit                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `thinking`                        | [`ThinkingConfig`](#thinkingconfig)                                                                                                                                                                            | `{ type: 'adaptive' }` untuk model yang didukung | Mengontrol perilaku thinking/reasoning Claude. Lihat [`ThinkingConfig`](#thinkingconfig) untuk opsi                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `title`                           | `string`                                                                                                                                                                                                       | `undefined`                                      | Display title untuk session. Ketika melanjutkan via `resume` atau `continue`, title session yang dilanjutkan yang persisted mengambil precedence; gunakan [`renameSession()`](#renamesession) untuk retitle session yang ada                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `toolAliases`                     | `Record<string, string>`                                                                                                                                                                                       | `undefined`                                      | Map built-in tool names ke MCP tool names sehingga Claude memanggil implementasi MCP Anda alih-alih built-in. Misalnya, `{ Bash: 'mcp__workspace__bash' }`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `toolConfig`                      | [`ToolConfig`](#toolconfig)                                                                                                                                                                                    | `undefined`                                      | Konfigurasi untuk perilaku built-in tool. Lihat [`ToolConfig`](#toolconfig) untuk detail                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `tools`                           | `string[] \| { type: 'preset'; preset: 'claude_code' }`                                                                                                                                                        | `undefined`                                      | Konfigurasi tool. Teruskan array nama tool atau gunakan preset untuk mendapatkan default tools Claude Code                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |

<h4 id="handle-slow-or-stalled-api-responses">
  Tangani respons API yang lambat atau terhenti
</h4>

CLI subprocess membaca beberapa variabel lingkungan yang mengontrol API timeouts dan stall detection. Teruskan melalui opsi `env`:

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

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

* `API_TIMEOUT_MS`: per-request timeout pada Anthropic client, dalam milidetik. Default `600000`. Berlaku untuk main loop dan semua subagents.
* `CLAUDE_CODE_MAX_RETRIES`: maksimal API retries. Default `10`, capped di `15`. Setiap retry mendapat jendela `API_TIMEOUT_MS` sendiri, jadi worst-case wall time kira-kira `API_TIMEOUT_MS × (CLAUDE_CODE_MAX_RETRIES + 1)` plus backoff. Untuk unattended runs yang perlu menunggu outages yang lebih lama, atur [`CLAUDE_CODE_RETRY_WATCHDOG=1`](/docs/id/errors#tune-retry-behavior): ini retries transient capacity errors tanpa batas dan, pada Claude Code v2.1.199 atau lebih baru, menaikkan default untuk transient errors lainnya ke `300` dan menghapus cap pada variabel ini.
* `CLAUDE_ASYNC_AGENT_STALL_TIMEOUT_MS`: stall watchdog untuk subagents. Saat stream watchdog aktif, default adalah `CLAUDE_STREAM_IDLE_TIMEOUT_MS` plus 5 menit, yang menjadi `600000` kecuali Anda menaikkan variabel itu. Dengan stream watchdog off, default adalah `600000`. Sebelum v2.1.257, default selalu `600000`.

  Timer reset pada setiap stream event. Pada stall, Claude Code membatalkan subagent dan melaporkan stall ke parent. Untuk background subagent, ini juga menandai task failed dan melampirkan partial result apa pun.
* `CLAUDE_ENABLE_STREAM_WATCHDOG` dengan `CLAUDE_STREAM_IDLE_TIMEOUT_MS`: stream watchdog yang membatalkan request ketika headers telah tiba tetapi response body berhenti streaming. Watchdog aktif secara default untuk semua providers; atur `CLAUDE_ENABLE_STREAM_WATCHDOG=0` untuk menonaktifkannya. `CLAUDE_STREAM_IDLE_TIMEOUT_MS` defaults ke `300000` dan diclamped ke minimum itu. Setelah abort, [Automatic retries](/docs/id/errors#automatic-retries) mencakup apa yang Claude Code lakukan, berdasarkan seberapa jauh response telah maju.

  Saat watchdog menunggu response yang gateway di belakang `ANTHROPIC_BASE_URL` tahan terbuka dengan keep-alive pings, host yang menetapkan `includePartialMessages` terus menerima `ping` [stream events](#sdkpartialassistantmessage), jadi baca frames itu sebagai liveness daripada timing session out pada silence. Sebelum v2.1.257, frames berhenti 5 menit setelah last real stream event.

<h3 id="query-object">
  Objek `Query`
</h3>

Interface yang dikembalikan oleh fungsi `query()`.

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
  applyFlagSettings(settings: {
    [K in keyof Settings]?: K extends 'effortLevel'
      ? 'low' | 'medium' | 'high' | 'xhigh' | 'max' | null
      : Settings[K] | null;
  }): Promise<void>;
  updateSettings(
    source: 'localSettings' | 'userSettings',
    settings: Record<string, unknown>,
  ): Promise<void>;
  initializationResult(): Promise<SDKControlInitializeResponse>;
  reinitialize(): Promise<SDKControlInitializeResponse>;
  supportedCommands(): Promise<SlashCommand[]>;
  supportedModels(): Promise<ModelInfo[]>;
  supportedAgents(): Promise<AgentInfo[]>;
  mcpServerStatus(): Promise<McpServerStatus[]>;
  getContextUsage(opts?: {
    detail?: 'summary' | 'full';
  }): Promise<SDKControlGetContextUsageResponse>;
  readFile(
    path: string,
    options?: { maxBytes?: number; encoding?: 'utf-8' | 'base64' }
  ): Promise<SDKControlReadFileResponse | null>;
  reloadSkills(): Promise<SDKControlReloadSkillsResponse>;
  accountInfo(): Promise<AccountInfo>;
  reconnectMcpServer(serverName: string): Promise<void>;
  toggleMcpServer(serverName: string, enabled: boolean): Promise<void>;
  setMcpServers(servers: Record<string, McpServerConfig>): Promise<McpSetServersResult>;
  readMcpResource(serverName: string, uri: string): Promise<SDKControlMcpReadResourceResponse>;
  streamInput(stream: AsyncIterable<SDKUserMessage>): Promise<void>;
  stopTask(taskId: string): Promise<void>;
  close(): void;
}
```

<h4 id="methods">
  Metode
</h4>

| Metode                                 | Deskripsi                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| :------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `interrupt()`                          | Mengganggu query. Hanya tersedia dalam streaming input mode. Ketika CLI mengiklankan kemampuan `interrupt_receipt_v1` dalam [`SDKSystemMessage.capabilities`](#sdksystemmessage), resolves dengan [`SDKControlInterruptResponse`](#sdkcontrolinterruptresponse) yang mencantumkan pesan yang pending ketika interrupt tiba. Resolves `undefined` pada CLIs sebelum v2.1.205                                                                                                                                                      |
| `rewindFiles(userMessageId, options?)` | Mengembalikan files ke state mereka pada user message yang ditentukan. Teruskan `{ dryRun: true }` untuk preview changes. Memerlukan `enableFileCheckpointing: true`. Lihat [File checkpointing](/docs/id/agent-sdk/file-checkpointing)                                                                                                                                                                                                                                                                                               |
| `setPermissionMode()`                  | Mengubah permission mode (hanya tersedia dalam streaming input mode)                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `setModel()`                           | Mengubah model (hanya tersedia dalam streaming input mode). Meneruskan `undefined` atau string `"default"` reset ke [Claude Code's default model](/docs/id/model-config)                                                                                                                                                                                                                                                                                                                                                              |
| `setMaxThinkingTokens()`               | *Deprecated:* Gunakan opsi `thinking` sebagai gantinya. Mengubah maksimal thinking tokens. Meneruskan `null` reset thinking ke session default: mid-session override dihapus, dan thinking tetap off untuk sessions yang memilikinya disabled                                                                                                                                                                                                                                                                                    |
| `applyFlagSettings(settings)`          | Merge settings ke dalam layer flag settings session saat runtime (hanya tersedia dalam streaming input mode). Lihat [`applyFlagSettings()`](#applyflagsettings)                                                                                                                                                                                                                                                                                                                                                                  |
| `updateSettings(source, settings)`     | Menulis satu key yang allowlisted ke file local settings project atau file user settings Anda, sehingga nilai persists untuk sessions kemudian. Lihat [`updateSettings()`](#updatesettings). Memerlukan TypeScript SDK v0.3.257 atau lebih baru, yang bundle Claude Code v2.1.257                                                                                                                                                                                                                                                |
| `initializationResult()`               | Mengembalikan full initialization result termasuk supported commands, models, account info, dan output style configuration                                                                                                                                                                                                                                                                                                                                                                                                       |
| `reinitialize()`                       | Re-sends `initialize` control request ke running CLI dan mengembalikan fresh result alih-alih cached first-connect result. Gunakan setelah transport gap, seperti reattaching ke session setelah disconnect, sehingga pending permission requests mencapai callback `canUseTool` Anda lagi. Buat callback idempotent per request ID, karena request yang response-nya hilang didispatch lagi. Memerlukan Claude Code v2.1.195 atau lebih baru                                                                                    |
| `supportedCommands()`                  | Mengembalikan available commands. Dari Agent SDK v0.3.216 daftar mencerminkan mid-session command changes; lihat [`SDKCommandsChangedMessage`](#sdkcommandschangedmessage)                                                                                                                                                                                                                                                                                                                                                       |
| `supportedModels()`                    | Mengembalikan available models dengan display info                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `supportedAgents()`                    | Mengembalikan available subagents sebagai [`AgentInfo`](#agentinfo)`[]`                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `mcpServerStatus()`                    | Mengembalikan status connected MCP servers sebagai [`McpServerStatus`](#mcpserverstatus)`[]`                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `getContextUsage(opts?)`               | Mengembalikan [`SDKControlGetContextUsageResponse`](#sdkcontrolgetcontextusageresponse) yang memecah session's context window usage berdasarkan kategori, skill, dan tool. Dengan default `detail`, ini adalah data yang sama `/context` tampilkan dalam interactive session. Opsi [`detail`](#sdkcontrolgetcontextusageresponse) memerlukan Agent SDK v0.3.257 atau lebih baru                                                                                                                                                  |
| `readFile(path, options?)`             | Membaca file dari session's filesystem. Claude Code resolves path terhadap `cwd`; [What `readFile()` can read](#what-readfile-can-read) mencantumkan files yang disajikan. Teruskan `{ maxBytes }` untuk mengubah read cap (default 1 MB, ceiling 10 MB) dan `{ encoding: 'base64' }` untuk binary files seperti images. Resolves dengan [`SDKControlReadFileResponse`](#sdkcontrolreadfileresponse), atau `null` pada permission denial, missing file, atau transport error. Memerlukan TypeScript SDK v0.2.121 atau lebih baru |
| `reloadSkills()`                       | Reload skills dari disk, sehingga skills yang Anda tambahkan atau edit mid-session menjadi tersedia untuk running session. Resolves dengan [`SDKControlReloadSkillsResponse`](#sdkcontrolreloadskillsresponse) yang mencantumkan skills yang tersedia setelah reload. Memerlukan Agent SDK v0.3.163 atau lebih baru                                                                                                                                                                                                              |
| `accountInfo()`                        | Mengembalikan account information                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `reconnectMcpServer(serverName)`       | Reconnect MCP server berdasarkan nama. Jika nama juga cocok dengan entri dalam settings file seperti `.mcp.json` atau `~/.claude.json`, Claude Code reconnect server yang Anda konfigurasi melalui [`mcpServers`](#options) atau `setMcpServers()`, bukan settings-file entry. Urutan resolusi itu memerlukan Claude Code v2.1.257 atau lebih baru                                                                                                                                                                               |
| `toggleMcpServer(serverName, enabled)` | Enable atau disable MCP server berdasarkan nama, dengan name resolution yang sama seperti `reconnectMcpServer()`. Menonaktifkan disconnect server                                                                                                                                                                                                                                                                                                                                                                                |
| `setMcpServers(servers)`               | Secara dinamis ganti set MCP servers untuk session ini. Resolves dengan [`McpSetServersResult`](#mcpsetserversresult) yang menyebutkan servers mana yang ditambahkan dan dihapus, dan errors apa pun                                                                                                                                                                                                                                                                                                                             |
| `readMcpResource(serverName, uri)`     | *Alpha.* Membaca satu MCP Apps `ui://` resource dari connected MCP server sehingga aplikasi Anda dapat merender widget tool. Resolves dengan [`SDKControlMcpReadResourceResponse`](#sdkcontrolmcpreadresourceresponse). Memerlukan TypeScript Agent SDK v0.3.280 atau lebih baru                                                                                                                                                                                                                                                 |
| `streamInput(stream)`                  | Stream input messages ke query untuk multi-turn conversations                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `stopTask(taskId)`                     | Stop running background task berdasarkan ID                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `close()`                              | Close query dan terminate underlying process. Secara paksa mengakhiri query dan membersihkan semua resources                                                                                                                                                                                                                                                                                                                                                                                                                     |

<h4 id="applyflagsettings">
  `applyFlagSettings()`
</h4>

Mengubah [settings](/docs/id/settings) pada running session tanpa restart query. Gunakan ketika setting yang tidak memiliki dedicated setter perlu berubah mid-session, seperti tightening `permissions` setelah agent membaca untrusted input. `setModel()` dan `setPermissionMode()` adalah dedicated setters untuk dua key itu; `applyFlagSettings()` adalah bentuk umum yang menerima subset apa pun dari settings keys, dan meneruskan `model` di sini berperilaku sama seperti `setModel()`.

Hanya beberapa keys yang berlaku mid-session:

* **Applied pada next turn**: `effortLevel`, `ultracode`, `permissions`, `hooks`, `skillOverrides`, `fastMode`, `agent`. Switching `agent` juga menerapkan model override dan hooks agent itu pada next turn. System prompt-nya berlaku pada next turn, atau, dalam session yang [reuses recorded system prompt](/docs/id/agent-sdk/modifying-system-prompts#change-the-prompt-of-an-existing-session), sekali session dicompact.
* **Applied selama current turn**: `model`. Jika Anda switch `model` saat Claude bekerja pada turn, response yang Claude sudah generate selesai pada model lama, dan sisa turn, dimulai dengan next call Claude Code buat ke model, menggunakan yang baru. Subagents menjaga model mereka sendiri. Sebelum v2.1.212, mid-turn switch menunggu next turn.
* **Tidak ada efek mid-session**: opsi system prompt. Ini diselesaikan sekali saat startup, jadi running session menjaga nilai asli bahkan meskipun call berhasil. Untuk mengubahnya, mulai session baru.

`effortLevel` menerima nama [effort level](/docs/id/model-config#adjust-effort-level). Ini juga menerima `"ultracode"`, yang meminta `xhigh` effort dengan [ultracode](/docs/id/workflows#let-claude-decide-with-ultracode) on. `applyFlagSettings()` mendeklarasikan `effortLevel` tanpa nilai itu, jadi teruskan `{ ultracode: true }` yang setara dalam TypeScript. Nilai `ultracode` memerlukan Claude Code v2.1.203 atau lebih baru dan diterima hanya oleh `applyFlagSettings()`, bukan oleh key `effortLevel` dalam settings file.

Values ditulis ke layer flag-settings, layer yang sama yang opsi inline `settings` dari `query()` isi saat startup. Ini adalah tier yang sama yang [on-page precedence section](#settings-precedence) sebut programmatic options.

Successive calls shallow-merge top-level keys. Second call dengan `{ permissions: {...} }` menggantikan seluruh objek `permissions` dari prior call daripada deep-merging ke dalamnya.

Untuk clear key yang Anda atur dengan `applyFlagSettings()`, teruskan `null` untuk key itu. Sebagian besar keys kemudian fallback pertama ke nilai yang opsi `settings` dari `query()` atur saat startup, kemudian ke lower-precedence sources. Cleared `model` reset ke [Claude Code's default model](/docs/id/model-config), bahkan ketika settings file menetapkan `model`. Meneruskan `undefined` tidak memiliki efek karena JSON serialization menghapusnya.

Tiga keys selain `model` reset session state alih-alih fallback:

* `effortLevel: null` mengembalikan session ke model's default effort level, bukan ke opsi `effort` dari `query()` atau `effortLevel` dari settings file.
* `agent: null` menjalankan main thread tanpa agent, dimulai dengan next turn, daripada restore opsi `agent` dari `query()` atau `agent` dari settings file. Jika cleared agent telah menerapkan model sendiri, session kembali ke model yang diselesaikan saat startup.
* `ultracode: null` mematikan ultracode, seperti `false` lakukan, daripada restore nilai `ultracode` dari settings file. Session menjaga current effort level-nya, jadi teruskan `effortLevel` dalam call yang sama untuk mengubahnya.

Hanya tersedia dalam streaming input mode, constraint yang sama seperti `setModel()` dan `setPermissionMode()`.

Contoh di bawah switch active model mid-session, kemudian clear override sehingga model reset ke [Claude Code's default model](/docs/id/model-config).

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

const q = query({ prompt: messageStream });

// Override model untuk sisa session
await q.applyFlagSettings({ model: "claude-opus-4-6" });

// Kemudian: clear override; model reset ke Claude Code's default
await q.applyFlagSettings({ model: null });
```

<Note>
  `applyFlagSettings()` adalah TypeScript-only. Python SDK tidak expose method yang setara.
</Note>

<h4 id="updatesettings">
  `updateSettings()`
</h4>

Menulis satu key yang allowlisted ke settings file pada disk, sehingga nilai persists untuk sessions kemudian yang memuat source itu. Setiap source menerima satu key, dengan string value:

* **`"localSettings"`**: menerima `outputStyle` dan merge ke dalam project's local settings file, `.claude/settings.local.json`. Style baru berlaku pada session's next request.
* **`"userSettings"`**: menerima `effortLevel` dan menyimpannya sebagai default [effort level](/docs/id/model-config#adjust-effort-level) untuk session's current model, di bawah [`modelSettings`](/docs/id/settings-reference#modelsettings) dalam user settings file Anda. Meneruskan `max` menulis nothing, karena `max` adalah session-only. Running session menjaga current effort level-nya baik cara, jadi panggil [`applyFlagSettings()`](#applyflagsettings) ketika Anda juga ingin mengubah itu. Source ini memerlukan TypeScript SDK v0.3.277 atau lebih baru, yang bundle Claude Code v2.1.277.

Call menolak ketika request membawa key lain, ketika session berjalan atas remote transport, dan ketika session's [`settingSources`](#options) exclude source yang Anda beri nama. Menghapus key tidak didukung.

<h3 id="warmquery">
  `WarmQuery`
</h3>

Handle yang dikembalikan oleh [`startup()`](#startup). Subprocess sudah spawned dan initialized, jadi memanggil `query()` pada handle ini menulis prompt langsung ke ready process tanpa startup latency.

```typescript theme={null}
interface WarmQuery extends AsyncDisposable {
  query(prompt: string | AsyncIterable<SDKUserMessage>): Query;
  close(): void;
}
```

<h4 id="methods-2">
  Metode
</h4>

| Metode          | Deskripsi                                                                                                                   |
| :-------------- | :-------------------------------------------------------------------------------------------------------------------------- |
| `query(prompt)` | Kirim prompt ke pre-warmed subprocess dan kembalikan [`Query`](#query-object). Dapat hanya dipanggil sekali per `WarmQuery` |
| `close()`       | Close subprocess tanpa mengirim prompt. Gunakan ini untuk discard warm query yang tidak lagi diperlukan                     |

`WarmQuery` mengimplementasikan `AsyncDisposable`, jadi dapat digunakan dengan `await using` untuk automatic cleanup.

<h3 id="sdkcontrolinitializeresponse">
  `SDKControlInitializeResponse`
</h3>

Return type dari `initializationResult()`. Berisi session initialization data.

```typescript theme={null}
type SDKControlInitializeResponse = {
  commands: SlashCommand[];
  agents: AgentInfo[];
  output_style: string;
  available_output_styles: string[];
  models: ModelInfo[];
  account: AccountInfo;
  fast_mode_state?: "off" | "cooldown" | "on";
  fast_mode_disabled_reason?: FastModeDisabledReason;
  hooks_applied?: boolean;
};
```

`hooks_applied` melaporkan apakah Claude Code mendaftarkan `hooks` yang `initialize` request bawa. SDK mengirim request itu sekali ketika session dimulai dan lagi pada setiap panggilan [`reinitialize()`](#query-object). Field memerlukan Agent SDK v0.3.238 atau lebih baru.

Claude Code menghilangkan field ketika request tidak membawa hooks. Ketika request membawa hooks, nilai tergantung pada apakah request adalah session's first initialize dan, untuk yang berulang, pada cara itu mencapai session:

* `true`: Claude Code mendaftarkan hooks. Session's first initialize mengembalikan nilai ini. Repeated initialize yang dikirim melalui CLI's stdin juga mengembalikan `true`. Dalam hal ini hooks dalam new request menggantikan hooks yang didaftarkan sebelumnya.
* `false`: Claude Code mengabaikan hooks. Repeated initialize yang dikirim ke remote session mengembalikan nilai ini, jadi second client yang join session tidak dapat menggantikan hooks yang first client daftarkan.

Sebelum Agent SDK v0.3.238, response tidak pernah membawa field, dan Claude Code mengabaikan `hooks` pada setiap repeated initialize.

Response selalu melaporkan `fast_mode_state`, dan ketika sesuatu memblokir [fast mode](/docs/id/fast-mode), `fast_mode_disabled_reason` membawa reason code bersama dengannya, jadi Anda dapat menjelaskan blocked state alih-alih re-deriving availability. Kedua perilaku memerlukan Claude Code v2.1.219 atau lebih baru. Sebelum v2.1.219, response menghilangkan `fast_mode_state` ketika fast mode tidak tersedia dan tidak pernah membawa reason. Untuk reason codes dan meanings mereka, lihat [`fast_mode_disabled_reason`](#sdkresultmessage) pada result message.

Control-response wrapper untuk successful `initialize` juga membawa array `pending_permission_requests`. Field berada pada response wrapper itu sendiri, bukan dalam payload `SDKControlInitializeResponse` di atas. Setiap entri adalah complete `control_request` message dengan shape `{ type: "control_request", request_id, request }` yang sama yang session stream untuk permission requests saat running.

Array mencantumkan permission requests yang Claude Code process ini telah issued dan belum resolved. SDK membaca array untuk Anda dan mendispatch setiap entri ke callback [`canUseTool`](#canusetool) Anda, redelivery yang sama yang [`reinitialize()`](#query-object) trigger setelah transport gap. Handle repeated request IDs idempotently, karena entri dapat mengulangi request yang callback sudah terima sebelum connection drop.

Array selalu present pada successful `initialize` response dan kosong ketika process ini tidak memiliki unresolved permission request. Memerlukan Claude Code v2.1.268 atau lebih baru. Versi sebelumnya dapat menghilangkan field, jadi jika Anda parse wire protocol sendiri, treat missing field sebagai older CLI daripada sebagai proof bahwa nothing is pending.

<h3 id="sdkcontrolinterruptresponse">
  `SDKControlInterruptResponse`
</h3>

Interrupt receipt: nilai yang [`interrupt()`](#query-object) resolves dengan pada CLI yang mengiklankan kemampuan `interrupt_receipt_v1` dalam [`SDKSystemMessage.capabilities`](#sdksystemmessage). Memerlukan Claude Code v2.1.205 atau lebih baru. Earlier CLIs menjawab interrupt dengan empty success payload, jadi `interrupt()` resolves ke `undefined`.

```typescript theme={null}
type SDKControlInterruptResponse = {
  still_queued: string[];
  cancelled?: string[];
};
```

`still_queued` mencantumkan UUIDs dari user messages yang pending ketika interrupt tiba: messages masih dalam queue, plus messages apa pun yang Claude Code sudah ambil dari queue untuk next turn. Sekali session's first turn telah dimulai, Claude Code memproses listed messages setelah interrupt kecuali Anda cancel mereka terlebih dahulu, dan dapat merge beberapa ke dalam satu turn. Jika Anda interrupt sebelum first turn dimulai, Claude Code membatalkan turn itu segera setelah dimulai, dan listed messages dalam turn itu tidak mendapat response.

Gunakan receipt untuk memutuskan apakah akan resend apa pun. Listed message yang Anda tidak cancel memasuki conversation apakah atau tidak mendapat response, jadi resending-nya mengirimkan ke Claude dua kali.

Interpretasikan list dengan caveats ini:

* Hanya messages yang di-enqueue dengan UUID yang muncul. Array kosong tidak berarti nothing else akan run.
* Hanya main-thread messages yang tercantum. Messages yang ditujukan ke subagent out of scope.
* List dapat include UUIDs yang client Anda tidak pernah kirim, seperti [scheduled task](/docs/id/scheduled-tasks) triggers. Abaikan UUIDs yang Anda tidak kenal alih-alih treat sebagai error.

Client yang drive CLI's control protocol langsung, daripada melalui `interrupt()`, dapat set `cancel_queued: true` pada `interrupt` control request. Claude Code v2.1.219 dan lebih baru mengiklankan support dengan kemampuan `interrupt_cancel_queued_v1` dalam [`SDKSystemMessage.capabilities`](#sdksystemmessage); older CLIs mengabaikan field dan leave queued messages untuk run seperti biasa. Interrupt seperti itu juga cancel setiap message yang akan otherwise tercantum di bawah `still_queued`: receipt mencantumnya di bawah `cancelled` sebagai gantinya, `still_queued` kosong, dan none dari mereka run.

`cancelled` list membawa caveats yang sama seperti `still_queued`. Metode `interrupt()` tidak pernah mengirim `cancel_queued`, jadi receipts yang resolves dengan tidak membawa `cancelled`.

Receipt adalah snapshot yang diambil pada moment interrupt diproses, dan pada clean interrupt tiba sebelum interrupted turn's [`SDKResultMessage`](#sdkresultmessage). Baca receipt daripada inspect queue setelah result itu: loop dimulai next queued turn segera, jadi queue yang Anda inspect setelah result sudah berubah.

<h3 id="sdkcontrolgetcontextusageresponse">
  `SDKControlGetContextUsageResponse`
</h3>

Return type dari [`getContextUsage()`](#query-object). Dengan default `detail`, ini adalah payload yang sama Claude Code render untuk `/context` command dalam interactive session, jadi bersama token counts ini membawa display fields seperti `color` dan `gridRows` yang Claude Code gunakan untuk draw `/context` usage grid.

Argumen `detail` opsional method memilih bagaimana Claude Code menghitung setiap kategori. Dengan default, `'full'`, Claude Code menghitung setiap kategori dengan token-counting API requests. Teruskan `{ detail: 'summary' }` untuk mendapatkan answer dari last response's usage dan local estimates sebagai gantinya. Tidak ada token-count requests keluar, dan per-category numbers adalah approximate. Argumen `detail` memerlukan Agent SDK v0.3.257 atau lebih baru.

Ketika Anda mengirim `/context` sebagai prompt alih-alih memanggil method, Claude Code melampirkan payload [`SDKContextUsage`](#sdkcontextusage) ke bidang `context_usage` dari assistant message yang delivers result. Field itu memerlukan Agent SDK v0.3.232 atau lebih baru.

```typescript theme={null}
type SDKControlGetContextUsageResponse = {
  categories: {
    name: string;
    tokens: number;
    color: string;
    isDeferred?: boolean;
  }[];
  totalTokens: number;
  maxTokens: number;
  rawMaxTokens: number;
  percentage: number;
  gridRows: {
    color: string;
    isFilled: boolean;
    categoryName: string;
    tokens: number;
    percentage: number;
    squareFullness: number;
  }[][];
  model: string;
  memoryFiles: {
    path: string;
    type: string;
    tokens: number;
  }[];
  mcpTools: {
    name: string;
    serverName: string;
    tokens: number;
    isLoaded?: boolean;
  }[];
  deferredBuiltinTools?: {
    name: string;
    tokens: number;
    isLoaded: boolean;
  }[];
  systemTools?: {
    name: string;
    tokens: number;
  }[];
  systemPromptSections?: {
    name: string;
    tokens: number;
  }[];
  agents: {
    agentType: string;
    source: string;
    tokens: number;
  }[];
  slashCommands?: {
    totalCommands: number;
    includedCommands: number;
    tokens: number;
  };
  skills?: {
    totalSkills: number;
    includedSkills: number;
    tokens: number;
    skillFrontmatter: {
      name: string;
      source: string;
      tokens: number;
    }[];
  };
  autoCompactThreshold?: number;
  isAutoCompactEnabled: boolean;
  messageBreakdown?: {
    toolCallTokens: number;
    toolResultTokens: number;
    attachmentTokens: number;
    assistantMessageTokens: number;
    userMessageTokens: number;
    redirectedContextTokens: number;
    unattributedTokens: number;
    toolCallsByType: {
      name: string;
      callTokens: number;
      resultTokens: number;
    }[];
    attachmentsByType: {
      name: string;
      tokens: number;
    }[];
  };
  apiUsage: {
    input_tokens: number;
    output_tokens: number;
    cache_creation_input_tokens: number;
    cache_read_input_tokens: number;
  } | null;
};
```

Baca token attribution dari collection fields:

* `categories` memegang per-category totals.
* `mcpTools` dan `agents` attribute tokens ke individual MCP tools dan subagents.
* `memoryFiles` mencantumkan setiap loaded memory file dengan costnya.
* `skills.skillFrontmatter` attributes skill listing's tokens ke setiap included skill. Per-skill counts mengukur setiap skill's listing entry seperti Claude Code benar-benar kirimkan, yang dapat lebih pendek daripada skill's full frontmatter. Bandingkan `skills.totalSkills` dengan `skills.includedSkills` untuk lihat apakah setiap discovered skill membuat ke listing.

`totalTokens` adalah session's current context usage, dan `maxTokens` adalah window yang usage diukur terhadap. Window itu adalah model's context window, atau lower auto-compaction window ketika satu berlaku. `rawMaxTokens` membawa nilai yang sama seperti `maxTokens`, dan `percentage` adalah `totalTokens` sebagai rounded percentage dari window itu.

Claude Code meninggalkan optional `deferredBuiltinTools`, `systemTools`, dan `systemPromptSections` diagnostics unset, jadi expect mereka absent bahkan meskipun type mendeklarasikan mereka.

<h3 id="sdkcontrolreadfileresponse">
  `SDKControlReadFileResponse`
</h3>

Return type dari [`readFile()`](#query-object).

```typescript theme={null}
type SDKControlReadFileResponse = {
  contents: string;
  absPath: string;
  truncated?: boolean;
  encoding?: 'base64';
};
```

`contents` memegang file text, atau base64 data ketika Anda meminta `encoding: 'base64'`; response's `encoding` field diatur ke `'base64'` dalam hal itu. `absPath` adalah resolved absolute path. `truncated` diatur ketika file lebih panjang daripada `maxBytes` cap dan contents dipotong pada limit itu.

<h4 id="what-readfile-can-read">
  Apa yang `readFile()` dapat baca
</h4>

`readFile()` melayani set files yang lebih sempit daripada Read tool:

* Regular file di dalam salah satu session's working directories, seperti `cwd` dan `additionalDirectories`
* Beberapa file Claude Code sendiri untuk session, seperti tool results

`Read` deny dan ask rules masih memblokir matching path, dan broad `Read` allow rule tidak membuka sisa filesystem ke `readFile()`. Untuk apa pun yang lain call resolves dengan `null`.

<h3 id="sdkcontrolreloadskillsresponse">
  `SDKControlReloadSkillsResponse`
</h3>

Return type dari [`reloadSkills()`](#query-object).

```typescript theme={null}
type SDKControlReloadSkillsResponse = {
  skills: SlashCommand[];
};
```

`skills` mencantumkan skills yang tersedia setelah reload, dalam [`SlashCommand`](#slashcommand) shape yang sama yang `supportedCommands()` kembalikan.

<h3 id="sdkcontrolmcpreadresourceresponse">
  `SDKControlMcpReadResourceResponse`
</h3>

Return type dari [`readMcpResource()`](#query-object), membawa MCP server's `resources/read` result. Memerlukan TypeScript Agent SDK v0.3.280 atau lebih baru.

```typescript theme={null}
type SDKControlMcpReadResourceResponse = {
  contents: {
    uri: string;
    mimeType?: string;
    text?: string;
    blob?: string;
    _meta?: Record<string, unknown>;
  }[];
};
```

Teruskan `readMcpResource()` server name seperti `mcpServerStatus()` laporkan dan `ui://` URI, seperti `ui.resourceUri` yang tool deklarasikan dalam [`_meta`](#mcpserverstatus)-nya. Call menolak untuk URI scheme apa pun yang lain, untuk [SDK MCP server](#createsdkmcpserver) yang aplikasi Anda host sendiri, dan untuk server yang tidak connected. Tersedia ketika init message's [`capabilities`](#sdksystemmessage) include `mcp_read_resource_v1`.

Setiap `contents` entry adalah satu content item seperti server kirimkan. `blob` memegang base64 data untuk binary item, dan `_meta` adalah item's sendiri `_meta`, di mana MCP Apps server menempatkan resource's `ui.csp` dan `ui.permissions`. Contents adalah untrusted third-party HTML, jadi render dalam sandbox.

<h3 id="agentdefinition">
  `AgentDefinition`
</h3>

Konfigurasi untuk subagent yang didefinisikan secara terprogram.

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
  omitClaudeMd?: boolean;
  memory?: "user" | "project" | "local";
  effort?: "low" | "medium" | "high" | "xhigh" | "max" | number;
  permissionMode?: PermissionMode;
  criticalSystemReminder_EXPERIMENTAL?: string;
};
```

| Field                                 | Diperlukan | Deskripsi                                                                                                                                                                                                                                                                                                                                              |
| :------------------------------------ | :--------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `description`                         | Ya         | Natural language description tentang kapan menggunakan agent ini                                                                                                                                                                                                                                                                                       |
| `tools`                               | Tidak      | Array dari allowed tool names. Jika dihilangkan, inherit setiap [tool available to subagents](/docs/id/sub-agents#available-tools). Untuk preload Skills ke dalam agent's context, gunakan field `skills` daripada listing `'Skill'` di sini                                                                                                                |
| `disallowedTools`                     | Tidak      | Array dari tool names untuk secara eksplisit disallow untuk agent ini. MCP server-level patterns juga diterima: `mcp__server` atau `mcp__server__*` menghapus setiap tool dari server itu, dan `mcp__*` menghapus setiap MCP tool dari server apa pun                                                                                                  |
| `prompt`                              | Ya         | System prompt agent                                                                                                                                                                                                                                                                                                                                    |
| `model`                               | Tidak      | Model override untuk agent ini. Menerima alias seperti `'fable'`, `'opus'`, `'sonnet'`, `'haiku'`, `'inherit'`, atau full model ID. `'inherit'` menggunakan main model. Ketika Anda menghilangkannya, Claude Code memilih model dalam [subagent model order](/docs/id/sub-agents#choose-a-model)                                                            |
| `mcpServers`                          | Tidak      | MCP server specifications untuk agent ini                                                                                                                                                                                                                                                                                                              |
| `skills`                              | Tidak      | Array dari skill names untuk preload ke dalam agent context                                                                                                                                                                                                                                                                                            |
| `initialPrompt`                       | Tidak      | Auto-submitted sebagai first user turn ketika agent ini berjalan sebagai main thread agent                                                                                                                                                                                                                                                             |
| `maxTurns`                            | Tidak      | Maksimal agentic turns (API round-trips) sebelum stopping                                                                                                                                                                                                                                                                                              |
| `background`                          | Tidak      | Jalankan agent ini sebagai non-blocking background task ketika invoked                                                                                                                                                                                                                                                                                 |
| `omitClaudeMd`                        | Tidak      | Jalankan agent ini tanpa user, project, dan local CLAUDE.md files ketika berjalan sebagai subagent; managed policy files masih load. Gunakan untuk agents yang mengambil semuanya yang mereka butuhkan dari Agent tool prompt. Diabaikan ketika agent ini berjalan sebagai main thread agent. Memerlukan TypeScript Agent SDK v0.3.271 atau lebih baru |
| `memory`                              | Tidak      | Memory source untuk agent ini: `'user'`, `'project'`, atau `'local'`                                                                                                                                                                                                                                                                                   |
| `effort`                              | Tidak      | Reasoning effort level untuk agent ini. Menerima named level atau integer                                                                                                                                                                                                                                                                              |
| `permissionMode`                      | Tidak      | Permission mode untuk tool execution dalam agent ini. [Subagent inheritance rules](/docs/id/agent-sdk/permissions#available-modes) memutuskan kapan berlaku. Lihat [`PermissionMode`](#permissionmode)                                                                                                                                                      |
| `criticalSystemReminder_EXPERIMENTAL` | Tidak      | Experimental: Critical reminder ditambahkan ke system prompt                                                                                                                                                                                                                                                                                           |

<h3 id="agentmcpserverspec">
  `AgentMcpServerSpec`
</h3>

Menentukan MCP servers yang tersedia untuk subagent. Dapat berupa server name (string yang mereferensikan server dari parent's `mcpServers` config) atau inline server configuration record yang memetakan server names ke configs.

```typescript theme={null}
type AgentMcpServerSpec = string | Record<string, McpServerConfigForProcessTransport>;
```

Di mana `McpServerConfigForProcessTransport` adalah `McpStdioServerConfig | McpSSEServerConfig | McpHttpServerConfig | McpSdkServerConfig`.

<h3 id="settingsource">
  `SettingSource`
</h3>

Mengontrol filesystem-based configuration sources mana yang SDK muat settings dari.

```typescript theme={null}
type SettingSource = "user" | "project" | "local";
```

| Nilai       | Deskripsi                                                                           | Lokasi                        |
| :---------- | :---------------------------------------------------------------------------------- | :---------------------------- |
| `'user'`    | Global user settings                                                                | `~/.claude/settings.json`     |
| `'project'` | Shared project settings (version controlled)                                        | `.claude/settings.json`       |
| `'local'`   | Local project settings, gitignored ketika Claude Code menyimpan setting ke dalamnya | `.claude/settings.local.json` |

<h4 id="default-behavior">
  Perilaku default
</h4>

Ketika `settingSources` dihilangkan atau `undefined`, `query()` memuat filesystem settings yang sama seperti Claude Code CLI: user, project, dan local. Lihat [What settingSources does not control](/docs/id/agent-sdk/claude-code-features#what-settingsources-does-not-control) untuk inputs yang dibaca terlepas dari opsi ini, dan cara menonaktifkannya.

<h4 id="why-use-settingsources">
  Mengapa menggunakan settingSources
</h4>

**Nonaktifkan filesystem settings:**

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

// Jangan muat user, project, atau local settings dari disk
const result = query({
  prompt: "Analyze this code",
  options: { settingSources: [] }
});
```

**Muat hanya specific setting sources:**

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

// Muat hanya project settings, abaikan user dan local
const result = query({
  prompt: "Run CI checks",
  options: {
    settingSources: ["project"] // Hanya .claude/settings.json
  }
});
```

Untuk memuat CLAUDE.md project instructions, sertakan `"project"` dalam `settingSources`. Lihat [Modify system prompts](/docs/id/agent-sdk/modifying-system-prompts#claude-md-files-for-project-level-instructions) untuk cara CLAUDE.md loading berinteraksi dengan system prompt options.

<h4 id="settings-precedence">
  Settings precedence
</h4>

Ketika multiple sources dimuat, settings dimerge dengan precedence ini (tertinggi ke terendah):

1. Local settings (`.claude/settings.local.json`)
2. Project settings (`.claude/settings.json`)
3. User settings (`~/.claude/settings.json`)

Programmatic options seperti `agents`, `allowedTools`, dan `settings` override user, project, dan local filesystem settings. Managed policy settings mengambil precedence atas programmatic options.

<h3 id="permissionmode">
  `PermissionMode`
</h3>

```typescript theme={null}
type PermissionMode =
  | "default" // Perilaku permission standar
  | "acceptEdits" // Auto-accept file edits
  | "bypassPermissions" // Bypass permission checks; explicit ask rules masih prompt
  | "plan" // Planning mode - explore tanpa editing
  | "dontAsk" // Jangan prompt untuk permissions, deny jika tidak pre-approved
  | "auto"; // Model classifier approves atau denies permission prompts
```

<h3 id="canusetool">
  `CanUseTool`
</h3>

Custom permission function type untuk mengontrol tool usage.

Fungsi adalah SDK replacement untuk interactive permission prompt: dipanggil hanya ketika [permission evaluation flow](/docs/id/agent-sdk/permissions#how-permissions-are-evaluated) resolves ke prompt. Tool calls sudah approved oleh entri `allowedTools`, settings allow rule, atau permission mode, seperti `acceptEdits` atau `bypassPermissions`, tidak pernah invoke-nya. Untuk gate setiap tool call, gunakan [`PreToolUse` hook](/docs/id/agent-sdk/hooks) sebagai gantinya.

Sebuah allow rule tidak pre-approve [actions no mode auto-approves](/docs/id/permission-modes#actions-no-mode-auto-approves); lihat [How permissions are evaluated](/docs/id/agent-sdk/permissions#how-permissions-are-evaluated) untuk mana dari mereka yang mencapai callback dan apa yang terjadi dalam `dontAsk` dan `auto` mode.

```typescript theme={null}
type CanUseTool = (
  toolName: string,
  input: Record<string, unknown>,
  options: {
    signal: AbortSignal;
    suggestions?: PermissionUpdate[];
    blockedPath?: string;
    mcpServer?: { name: string; source: string };
    decisionReason?: string;
    toolUseID: string;
    agentID?: string;
    requestId: string;
  }
) => Promise<PermissionResult | null>;
```

| Opsi             | Jenis                                       | Deskripsi                                                                                                                                                                                                                                                                                                    |
| :--------------- | :------------------------------------------ | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `signal`         | `AbortSignal`                               | Signaled jika operasi harus dibatalkan                                                                                                                                                                                                                                                                       |
| `suggestions`    | [`PermissionUpdate`](#permissionupdate)`[]` | Suggested permission updates sehingga user tidak diprompt lagi untuk tool ini. Bash prompts include suggestion dengan `localSettings` [destination](#permissionupdatedestination), jadi returning-nya dalam `updatedPermissions` menulis rule ke `.claude/settings.local.json` dan persists across sessions. |
| `blockedPath`    | `string`                                    | File path yang triggered permission request, jika applicable                                                                                                                                                                                                                                                 |
| `mcpServer`      | `{ name: string; source: string }`          | Untuk tool `mcp__*`, MCP server yang melayaninya dan dari mana definisi server itu berasal, dengan fields dari [`McpServerProvenance`](#mcpserverprovenance). Absent untuk tools lainnya. Memerlukan Agent SDK v0.3.274 atau lebih baru                                                                      |
| `decisionReason` | `string`                                    | Menjelaskan mengapa permission request ini triggered                                                                                                                                                                                                                                                         |
| `toolUseID`      | `string`                                    | Unique identifier untuk specific tool call ini dalam assistant message                                                                                                                                                                                                                                       |
| `agentID`        | `string`                                    | Jika running dalam sub-agent, sub-agent's ID                                                                                                                                                                                                                                                                 |
| `requestId`      | `string`                                    | `control_request` envelope's `request_id`. `control_response` yang aplikasi Anda kirim di luar SDK, seperti signed HTTP POST, harus echo nilai ini sehingga Claude Code process dapat match reply ke request                                                                                                 |

Callback normally resolves request dengan mengembalikan [`PermissionResult`](#permissionresult), yang SDK tulis kembali atas transport-nya sebagai `control_response`. Kembalikan `null` hanya ketika aplikasi Anda sudah mengirim `control_response` untuk request ini atas channel sendiri, echoing `requestId`; SDK kemudian skip menulis response ke transport-nya. Mengembalikan `null` dalam kasus lain apa pun meninggalkan tool call blocked indefinitely, karena tidak ada `control_response` yang pernah dikirim dan permission prompts tidak timeout.

Opsi `requestId` dan nilai return `null` memerlukan Claude Code v2.1.199 atau lebih baru.

<h3 id="permissionresult">
  `PermissionResult`
</h3>

Hasil dari permission check.

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

Konfigurasi untuk perilaku built-in tool.

```typescript theme={null}
type ToolConfig = {
  askUserQuestion?: {
    previewFormat?: "markdown" | "html";
  };
};
```

| Field                           | Jenis                  | Deskripsi                                                                                                                                                                             |
| :------------------------------ | :--------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `askUserQuestion.previewFormat` | `'markdown' \| 'html'` | Opts ke dalam bidang `preview` pada [`AskUserQuestion`](/docs/id/agent-sdk/user-input#question-format) options dan menetapkan content format-nya. Ketika unset, Claude tidak emit previews |

<h3 id="mcpserverconfig">
  `McpServerConfig`
</h3>

Konfigurasi untuk MCP servers.

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
  timeout?: number;
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

Konfigurasi untuk memuat plugins dalam SDK.

```typescript theme={null}
type SdkPluginConfig = {
  type: "local";
  path: string;
  skipMcpDiscovery?: boolean;
};
```

| Field              | Jenis     | Deskripsi                                                                                                                                                                                                       |
| :----------------- | :-------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`             | `'local'` | Harus `'local'` (hanya local plugins yang saat ini didukung)                                                                                                                                                    |
| `path`             | `string`  | Absolute atau relative path ke plugin directory                                                                                                                                                                 |
| `skipMcpDiscovery` | `boolean` | Ketika `true`, SDK memuat skills, hooks, agents, dan commands dari plugin ini tetapi tidak membaca `.mcp.json` atau manifest `mcpServers`-nya. Atur ini ketika aplikasi Anda memiliki plugin's MCP connections. |

**Contoh:**

```typescript theme={null}
plugins: [
  { type: "local", path: "./my-plugin" },
  { type: "local", path: "/absolute/path/to/plugin" }
];
```

Untuk informasi lengkap tentang membuat dan menggunakan plugins, lihat [Plugins](/docs/id/agent-sdk/plugins).

<h2 id="message-types">
  Jenis Pesan
</h2>

<h3 id="sdkmessage">
  `SDKMessage`
</h3>

Tipe union dari semua pesan yang mungkin dikembalikan oleh kueri.

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

<h3 id="sdkassistantmessage">
  `SDKAssistantMessage`
</h3>

Pesan respons asisten.

```typescript theme={null}
type SDKAssistantMessage = {
  type: "assistant";
  uuid: UUID;
  session_id: string;
  message: BetaMessage; // Dari Anthropic SDK
  parent_tool_use_id: string | null;
  error?: SDKAssistantMessageError;
  aborted?: true;
  timestamp?: string;
  context_usage?: SDKContextUsage;
  user_message_uuid?: string;
  user_message_uuids?: string[];
};
```

Bidang `message` adalah [`BetaMessage`](https://platform.claude.com/docs/en/api/messages/create) dari Anthropic SDK. Ini mencakup bidang seperti `id`, `content`, `model`, `stop_reason`, dan `usage`.

`SDKAssistantMessageError` adalah salah satu dari: `'authentication_failed'`, `'oauth_org_not_allowed'`, `'account_on_hold'`, `'billing_error'`, `'rate_limit'`, `'overloaded'`, `'invalid_request'`, `'model_not_found'`, `'server_error'`, `'max_output_tokens'`, `'cloud_credential_error'`, atau `'unknown'`. Empat dari nilai-nilai ini berarti lebih dari yang nama mereka katakan:

* `'model_not_found'`: model yang dipilih tidak ada atau tidak tersedia untuk akun atau deployment Anda
* `'overloaded'`: API mengembalikan 529 karena server mencapai kapasitas, berbeda dengan `'rate_limit'`, yang merupakan 429 terhadap kuota Anda
* `'account_on_hold'`: [akun Anda sedang ditahan](/docs/id/errors#your-account-is-on-hold)
* `'cloud_credential_error'`: Claude Code tidak dapat memperoleh kredensial AWS atau Google Cloud yang dapat digunakan di mesin tempat ia berjalan, sehingga tidak ada permintaan yang mencapai penyedia cloud. Penyebab umum adalah masuk cloud yang kedaluwarsa atau tidak pernah selesai di mesin itu, meskipun layanan kredensial yang singkat tidak dapat dijangkau melaporkan nilai yang sama. Lihat [Tidak dapat memuat kredensial AWS atau Google Cloud](/docs/id/errors#could-not-load-aws-or-google-cloud-credentials). Memerlukan TypeScript Agent SDK v0.3.267 atau lebih baru, yang menggabungkan Claude Code v2.1.267

`aborted` adalah `true` ketika gangguan atau pembatalan memotong pesan asisten sebelum aliran selesai: pesan tidak memiliki `stop_reason` dan konten mungkin berakhir di tengah kata. Bidang ini tidak ada pada pesan yang diselesaikan secara normal. Ini memerlukan Agent SDK v0.3.214 atau lebih baru.

Claude Code menetapkan `user_message_uuid` dan `user_message_uuids` pada pesan asisten pertama giliran, di bawah kondisi dalam [`user_message_uuid`](#user_message_uuid).

`timestamp` adalah waktu ISO 8601 ketika konten pesan selesai dihasilkan pada proses yang menghasilkannya. Nilai berasal dari jam mesin itu, jadi gunakan hanya untuk tampilan dan jangan urutkan pesan berdasarkannya. Satu giliran API dapat menghasilkan beberapa pesan asisten yang berbagi `message.id`, masing-masing dengan `timestamp` sendiri. Ketika bidang tidak ada, kembali ke waktu Anda menerima pesan.

`context_usage` adalah salinan terstruktur dari laporan `/context`, diketik sebagai [`SDKContextUsage`](#sdkcontextusage), dan memerlukan Agent SDK v0.3.232 atau lebih baru. Ketika Anda mengirim `/context` sebagai prompt, Claude Code mengirimkan laporan sebagai pesan asisten yang `message.content` menyimpan tabel markdown, dan melampirkan `context_usage` ke pesan yang sama. Claude Code tidak menetapkan bidang pada pesan asisten lain mana pun, dan versi sebelumnya mengirimkan tabel `/context` tanpanya, jadi baca rincian dari bidang ketika ada dan kembali ke teks markdown ketika tidak.

<h3 id="sdkusermessage">
  `SDKUserMessage`
</h3>

Pesan input pengguna.

```typescript theme={null}
type SDKUserMessage = {
  type: "user";
  uuid?: UUID;
  session_id?: string;
  message: MessageParam; // Dari Anthropic SDK
  pasted_content?: MessageParam["content"][];
  parent_tool_use_id: string | null;
  isSynthetic?: boolean;
  shouldQuery?: boolean;
  tool_use_result?: unknown;
  origin?: SDKMessageOrigin;
  inline_pastes?: string[];
};
```

Atur `pasted_content` untuk mengirim konten yang pengguna tempel ke UI prompt Anda daripada ketik, satu entri per tempel, masing-masing string atau array blok konten. Claude Code menambahkan teks setiap entri setelah teks yang diketik, secara berurutan, dan dapat membungkus setiap tempel dalam tag `<pasted_content>`. Blok selain teks diabaikan, jadi kirim gambar dan dokumen dalam `message.content`. Memerlukan Agent SDK v0.3.277 atau lebih baru.

Atur `shouldQuery` ke `false` untuk menambahkan pesan ke transkrip tanpa memicu giliran asisten. Pesan ditahan dan digabungkan ke pesan pengguna berikutnya yang memicu giliran. Gunakan ini untuk menyuntikkan konteks, seperti output pesan yang Anda jalankan di luar pita, tanpa menghabiskan panggilan model.

Pada pesan yang membawa blok `tool_result`, `tool_use_result` adalah objek output terstruktur alat daripada teks yang dikirim ke model. Bentuknya tergantung pada alat yang dinamai oleh blok `tool_use` yang cocok, jadi bidang diketik `unknown`; bentuk bawaan tercantum di bawah [Jenis Output Alat](#tool-output-types).

Untuk alat `Agent`, `tool_use_result` adalah [`AgentOutput`](#agent-2). Pada hasil `completed`, `content` menyimpan laporan subagen tanpa ID agen dan trailer penggunaan yang Claude Code tambahkan ke teks `tool_result`, jadi render dari `tool_use_result` daripada mengurai teks itu.

Untuk alat MCP yang hasilnya berisi blok `resource_link`, `tool_use_result` adalah objek dengan array `resourceLinks` dari entri [`SDKMcpResourceLink`](#sdkmcpresourcelink). Claude menerima setiap tautan sebagai baris teks dalam blok `tool_result`, jadi baca `resourceLinks` untuk merender file yang dikembalikan server daripada mengurai teks itu. Claude Code menghilangkan `resourceLinks` ketika hasil tidak memiliki tautan dan pada hasil dari subagen, menyimpan paling banyak 50 tautan per hasil, dan berhenti menambahkan tautan setelah array mencapai 64 KiB JSON terserialkan. `resourceLinks` memerlukan Agent SDK v0.3.257 atau lebih baru.

Atur `inline_pastes` untuk memberi tahu Claude Code bagian mana dari `message.content` yang pengguna tempel daripada ketik, satu string per tempel. Teks prompt tetap di tempat pengguna meletakkannya. Claude Code dapat membungkus setiap tempel yang tercantum dalam tag `<pasted_content>` di mana ia berdiri, jadi Claude dapat membedakan materi yang ditempel dari kata-kata pengguna sendiri. Hanya tempel dalam blok teks terakhir prompt yang dibungkus. Memerlukan TypeScript Agent SDK v0.3.280 atau lebih baru.

<h3 id="sdkusermessagereplay">
  `SDKUserMessageReplay`
</h3>

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

Giliran pengguna yang disuntikkan dari luar sesi, yang [`origin`](#sdkmessageorigin) jenisnya adalah `peer` atau `channel`, mencapai aliran sebagai pemutaran ulang apakah itu disampaikan selama giliran aktif atau memulai giliran baru saat sesi menganggur. Sebelum v2.1.207, giliran yang disuntikkan disampaikan saat sesi menganggur tidak menghasilkan pesan pada aliran dan hanya muncul ketika Anda membaca ulang transkrip.

<h3 id="sdkresultmessage">
  `SDKResultMessage`
</h3>

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
      user_message_uuid?: string;
      user_message_uuids?: string[];
      request_sent_wall_ms?: number;
      first_content_frame_ms?: number;
      first_stream_post_ms?: number;
      first_stream_post_ack_ms?: number;
      first_stream_post_wall_ms?: number;
      total_cost_usd: number;
      usage: NonNullableUsage;
      modelUsage: { [modelName: string]: ModelUsage };
      permission_denials: SDKPermissionDenial[];
      queued_turn_count?: number;
      structured_output?: unknown;
      deferred_tool_use?: { id: string; name: string; input: Record<string, unknown> };
      terminal_reason?: TerminalReason;
      fast_mode_state?: FastModeState;
      fast_mode_disabled_reason?: FastModeDisabledReason;
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
      queued_turn_count?: number;
      errors: string[];
      startup_failure_reason?: SDKStartupFailureReason;
      user_message_uuid?: string;
      user_message_uuids?: string[];
      terminal_reason?: TerminalReason;
      fast_mode_state?: FastModeState;
      fast_mode_disabled_reason?: FastModeDisabledReason;
      origin?: SDKMessageOrigin;
    };
```

Beberapa bidang pada hasil membawa detail diagnostik di luar `subtype`:

* `api_error_status`: kode status HTTP dari kesalahan API yang mengakhiri percakapan. Tidak ada atau `null` ketika giliran berakhir tanpa kesalahan API.
* `ttft_ms`: waktu ke token pertama dalam milidetik, diukur ketika pesan asisten lengkap pertama tiba. Hadir hanya pada lengan kesuksesan.
* `ttft_stream_ms`: waktu dalam milidetik hingga acara aliran `message_start` pertama, ketika aliran respons terbuka. Lebih rendah dari `ttft_ms`; celah antara keduanya adalah waktu yang dihabiskan untuk streaming pesan pertama. Hadir hanya pada lengan kesuksesan.
* `user_message_uuid`: `uuid` dari pesan yang Anda kirim yang dijawab giliran ini. Lihat [`user_message_uuid`](#user_message_uuid) untuk hasil mana yang membawanya.
* `user_message_uuids`: `uuid` dari setiap pesan yang Anda kirim yang dijawab Claude Code dalam giliran ini. Lihat [`user_message_uuids`](#user_message_uuids).
* `request_sent_wall_ms`: epoch milidetik di mana Claude Code mengirim permintaan API, untuk bergabung dengan stempel waktu sisi server. Hadir hanya bersama dengan [`user_message_uuid`](#user_message_uuid), pada hasil kesuksesan dengan `is_error` false yang gilirannya mengirim permintaan API.
* `first_content_frame_ms`: waktu dalam milidetik hingga acara aliran `content_block_start` atau `content_block_delta` pertama, menghitung blok pemikiran sebagai konten. Hadir pada lengan kesuksesan hanya, ketika `is_error` adalah false. Memerlukan Agent SDK v0.3.260 atau lebih baru.
* `first_stream_post_ms`, `first_stream_post_ack_ms`, `first_stream_post_wall_ms`: waktu untuk mengunggah acara aliran pertama giliran. Claude Code merekamnya hanya dalam sesi yang dialirkan ke claude.ai, seperti [sesi cloud](/docs/id/claude-code-on-the-web), dan hasil yang `query()` hasilkan tidak membawanya. Memerlukan Agent SDK v0.3.260 atau lebih baru.
* `usage`: loop agen utama saja. Mengecualikan panggilan subagen dan model tambahan, dan per-giliran dalam sesi input streaming. Lebih suka `modelUsage` untuk akuntansi token/biaya.
* `modelUsage`: total per-model untuk setiap panggilan model yang dibuat melalui pipeline kueri selama panggilan `query()` ini, termasuk loop utama, subagen, dan panggilan internal seperti pemadatan dan agen Workflow. Panggilan pembantu di luar pipeline itu, seperti pengklasifikasi izin dan permintaan penghitungan token, dikecualikan. Panggilan yang melanjutkan sesi juga menghitung [total per-model yang dipulihkan dari panggilan sebelumnya sesi](/docs/id/agent-sdk/cost-tracking#accumulate-costs-across-multiple-calls). Dalam sesi input streaming total bersifat kumulatif di seluruh giliran, jadi baca hasil terbaru daripada menjumlahkan di seluruh hasil. Lihat [Lacak biaya dalam mode input streaming](/docs/id/agent-sdk/cost-tracking#track-costs-in-streaming-input-mode) untuk pengaturan ulang dan [Pulihkan total setelah kerusakan sesi](/docs/id/agent-sdk/cost-tracking#recover-totals-after-a-session-crash) untuk hasil yang dinolkan.
* `total_cost_usd`: biaya perkiraan kumulatif dalam USD, mencakup panggilan yang sama dengan `modelUsage` dan pengaturan ulang pada titik yang sama. Panggilan yang melanjutkan sesi juga menghitung [total yang dipulihkan dari panggilan sebelumnya sesi](/docs/id/agent-sdk/cost-tracking#accumulate-costs-across-multiple-calls). Ini adalah perkiraan, bukan pernyataan penagihan. Lihat [Lacak biaya dan penggunaan](/docs/id/agent-sdk/cost-tracking) untuk peringatan akurasi.
* `queued_turn_count`: jumlah pesan yang Anda kirim dengan `origin: { kind: "human" }` yang masih menunggu ketika Claude Code menghasilkan hasil. Lihat [`queued_turn_count`](#queued_turn_count) untuk apa yang `0` dan bidang yang tidak ada katakan.
*

`startup_failure_reason`: mengapa Claude Code menolak untuk memulai, pada hasil `error_during_execution` yang ditulis sebelum keluar pada kegagalan startup yang diketahui. Lihat [`startup_failure_reason`](#startup_failure_reason) untuk nilai-nilai dan kegagalan mana yang membawanya. Memerlukan Agent SDK v0.3.274 atau lebih baru.

* `terminal_reason`: mengapa loop berakhir. Salah satu dari `"completed"`, `"max_turns"`, `"tool_deferred"`, `"aborted_streaming"`, `"aborted_tools"`, `"hook_stopped"`, `"stop_hook_prevented"`, `"background_requested"`, `"blocking_limit"`, `"rapid_refill_breaker"`, `"prompt_too_long"`, `"image_error"`, `"model_error"`, `"api_error"`, `"malformed_tool_use_exhausted"`, `"budget_exhausted"`, `"structured_output_retry_exhausted"`, `"tool_deferred_unavailable"`, atau `"turn_setup_failed"`.
* `fast_mode_state`: salah satu dari `"on"`, `"off"`, atau `"cooldown"`.
* `fast_mode_disabled_reason`: mengapa [fast mode](/docs/id/fast-mode) tidak tersedia sekarang. Tidak ada ketika tidak ada yang memblokir fast mode, meskipun permintaan mungkin masih berjalan dengan kecepatan standar. Selama cooldown setelah batas laju fast mode, Claude Code melaporkan `fast_mode_state: "cooldown"` tanpa kode alasan dan mengaktifkan kembali fast mode ketika cooldown berakhir. Memerlukan Claude Code v2.1.219 atau lebih baru.

Gunakan kode alasan untuk menjelaskan mengapa fast mode mati di UI Anda sendiri daripada menurunkan ketersediaan. Setiap kode menamai pemeriksaan yang memblokir fast mode:

| Kode Alasan            | Arti                                                                                                                                               |
| ---------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------- |
| `free`                 | Akun tidak memiliki langganan berbayar atau kredit penggunaan yang diperlukan fast mode                                                            |
| `preference`           | Organisasi telah menonaktifkan fast mode                                                                                                           |
| `extra_usage_disabled` | Kredit penggunaan dimatikan untuk akun                                                                                                             |
| `network_error`        | [Pemeriksaan ketersediaan](/docs/id/fast-mode#use-fast-mode-behind-proxies-and-llm-gateways) tidak dapat menjangkau `api.anthropic.com`                 |
| `unknown`              | Claude Code tidak dapat menentukan ketersediaan                                                                                                    |
| `not_first_party`      | Sesi menggunakan penyedia selain Anthropic API                                                                                                     |
| `disabled_by_env`      | [`CLAUDE_CODE_DISABLE_FAST_MODE`](/docs/id/env-vars) diatur                                                                                             |
| `model_not_allowed`    | Model Opus fast mode tidak ada dalam daftar izin [`availableModels`](/docs/id/model-config#restrict-model-selection) organisasi                         |
| `sdk_opt_in_required`  | Sesi belum memilih fast mode: teruskan `fastMode: true` dalam opsi [`settings`](#options) atau melalui [`applyFlagSettings()`](#applyflagsettings) |
| `pending`              | Pemeriksaan ketersediaan belum selesai                                                                                                             |

Pasangan bidang yang sama muncul pada [`SDKSystemMessage`](#sdksystemmessage) dan pada [`SDKControlInitializeResponse`](#sdkcontrolinitializeresponse), jadi Anda dapat membaca status fast mode sebelum giliran pertama.

Bidang `origin` meneruskan [`SDKMessageOrigin`](#sdkmessageorigin) dari pesan pengguna yang memicu hasil ini. Ketika SDK menyuntikkan giliran tindak lanjut sintetis, seperti untuk tugas yang selesai, `SDKResultMessage` yang dihasilkan membawa `origin: { kind: "task-notification" }`. Rutinitas yang pemicunya dipecat dan pesan yang diverifikasi server dari sesi lain Anda tiba dengan jenis ini juga, masing-masing dengan `subkind` yang dijelaskan dalam [Subkind notifikasi tugas](#task-notification-subkinds). Periksa `kind` untuk membedakan hasil yang menjawab prompt Anda dari tindak lanjut yang disuntikkan sebelum merutekan atau menekan mereka. Jika aplikasi Anda [mendeklarasikan jalankan terjadwal](#declare-a-scheduled-run), hasil mereka membawa `kind: "task-notification"` juga, jadi jangan tekan pada `kind` saja.

Ketika beberapa penyelesaian tugas latar belakang antri bersama-sama, Claude Code dapat menjawabnya dalam satu giliran daripada satu giliran masing-masing. Setiap penyelesaian masih menghasilkan hasilnya sendiri dengan asal ini. Semua kecuali yang terakhir dari penyelesaian yang Claude Code jawab bersama menghasilkan hasil kosong dengan `num_turns: 0`, secara berurutan, dan hasil yang terakhir membawa giliran yang menjawab mereka semua.

Bidang tidak ada untuk hasil yang dipancarkan sebelum giliran pengguna apa pun, seperti kesalahan startup.

Ketika hook `PreToolUse` mengembalikan `permissionDecision: "defer"`, hasil memiliki `stop_reason: "tool_deferred"` dan `deferred_tool_use` membawa `id`, `name`, dan `input` alat yang tertunda. Baca bidang ini untuk menampilkan permintaan di UI Anda sendiri, kemudian lanjutkan dengan `session_id` yang sama untuk melanjutkan. Lihat [Tunda panggilan alat untuk nanti](/docs/id/hooks#defer-a-tool-call-for-later) untuk perjalanan putaran penuh.

<h4 id="user_message_uuid">
  `user_message_uuid`
</h4>

`uuid` dari [`SDKUserMessage`](#sdkusermessage) yang dijawab giliran, diulang sehingga Anda dapat mencocokkan balasan Claude Code dengan pesan yang Anda kirim. Claude Code mengulangi `uuid` hanya jika Anda menetapkan satu pada pesan. Bidang ini opsional pada `SDKUserMessage`, dan prompt string yang dilewatkan ke `query()` tidak membawa apa pun.

Pesan mana yang dijawab giliran tergantung pada bagaimana giliran dimulai:

* **Pesan reguler yang Anda kirim**, yang berarti tanpa `isSynthetic: true`: giliran menjawab pesan itu untuk seluruh jalannya. Ketika Anda mengirim beberapa pesan berdekatan, Claude Code dapat menggabungkannya menjadi satu giliran, dan bidang kemudian membawa hanya `uuid` pesan terakhir. Untuk mencocokkan balasan dengan salah satu pesan yang digabungkan, gunakan [`user_message_uuids`](#user_message_uuids).
* **Pesan yang Anda kirim dengan `isSynthetic: true`**: giliran menjawab pesan itu pada awalnya. Jika Claude Code mengambil pesan reguler Anda di antara panggilan alat, giliran menjawab pesan yang diambil dari saat itu. Mengulangi `uuid` pesan sintetis memerlukan Agent SDK v0.3.265 atau lebih baru; versi sebelumnya tidak mengulangi apa pun pada giliran sintetis.
* **Prompt yang dihasilkan Claude Code sendiri**, seperti giliran yang melanjutkan pekerjaan terputus setelah sesi dimulai ulang: giliran tidak menjawab pesan Anda pada awalnya dan frame-nya tidak membawa echo. Jika Claude Code mengambil pesan reguler Anda di antara panggilan alat, giliran menjawab pesan itu dari saat itu. Echo pengambilan memerlukan Agent SDK v0.3.265 atau lebih baru; versi sebelumnya tidak mengulangi apa pun pada giliran ini.

Claude Code mengulangi `uuid` pesan yang dijawab pada tiga jenis frame:

* **Hasil**: setiap hasil giliran yang menjawab pesan yang Anda kirim. Setiap hasil seperti itu membawanya pada Agent SDK v0.3.265 atau lebih baru. Sebelum v0.3.265, hasil kesuksesan giliran yang dimulai pesan reguler tidak membawanya ketika giliran tidak mengirim permintaan API atau berakhir dengan panggilan alat yang ditunda. Sebelum v0.3.246, hasil kesalahan juga tidak membawanya, dan sebelum v0.3.216 setiap hasil tidak.
* **Balasan pertama giliran**: [pesan asisten](#sdkassistantmessage) pertama, atau dengan `includePartialMessages` [acara aliran](#sdkpartialassistantmessage) pertama yang `event.type` bukan `ping`, jadi Anda dapat mengikat balasan sebelum hasil tiba. Ketika giliran tidak mengalirkan apa pun, Claude Code menetapkannya pada pesan asisten pertama. Echo balasan pertama memerlukan Agent SDK v0.3.246 atau lebih baru. Ketika pesan yang dijawab giliran berubah di tengah giliran, balasan pertama setelah perubahan membawa bidang juga, pada Agent SDK v0.3.265 atau lebih baru; versi sebelumnya menetapkannya pada satu frame balasan per giliran.
* **Setiap frame [`thinking_tokens`](#sdkthinkingtokensmessage) giliran**: jadi Anda dapat mengatribusikan kemajuan pemikiran ke pesan yang Anda kirim tanpa menunggu balasan pertama giliran. Memerlukan Agent SDK v0.3.260 atau lebih baru.

Claude Code menghilangkan bidang dalam kasus-kasus ini:

* Frame balasan selain balasan pertama itu
* Frame subagen
* Giliran yang tidak menjawab pesan dengan `uuid`: giliran menjawab pesan yang Anda kirim tanpa satu, atau Claude Code memulai giliran sendiri dan tidak mengambil pesan reguler yang memiliki satu
* Hasil yang tidak menjawab pesan apa pun yang Anda kirim, seperti hasil yang dinolkan setelah proses pekerja yang jatuh

<h4 id="user_message_uuids">
  `user_message_uuids`
</h4>

`uuid` dari setiap pesan yang Anda kirim yang dijawab Claude Code dalam giliran ini. Ketika Anda mengirim beberapa pesan berdekatan, Claude Code dapat menggabungkannya menjadi satu giliran, dan `user_message_uuid` kemudian menamai hanya yang terakhir. Untuk mencocokkan balasan dengan salah satu pesan yang digabungkan, cari `uuid` pesan itu di mana pun dalam daftar ini. Memerlukan Agent SDK v0.3.259 atau lebih baru.

Claude Code menetapkan daftar bersama dengan `user_message_uuid` pada setiap frame balasan yang membawa bidang itu dan pada hasil. Untuk set lengkap frame yang membawa `user_message_uuid`, dan versi yang masing-masing memerlukan, lihat [`user_message_uuid`](#user_message_uuid). Daftar selalu berisi `user_message_uuid` dan menyimpan paling banyak 64 entri.

Ketika Claude Code mengambil pesan reguler yang Anda kirim saat giliran berjalan, ia menambahkan `uuid` pesan itu ke daftar hasil.

Ketika balasan pertama atau hasil membawa `user_message_uuid` tanpa daftar, itu berasal dari versi Claude Code sebelumnya, jadi kembali ke bidang tunggal.

<h4 id="queued_turn_count">
  `queued_turn_count`
</h4>

Jumlah pesan yang Anda kirim dengan [`origin: { kind: "human" }`](#sdkmessageorigin) yang masih menunggu dalam antrian perintah ketika Claude Code menghasilkan hasil. Memerlukan Agent SDK v0.3.242 atau lebih baru.

Apa yang `0` dan bidang yang tidak ada katakan:

* **`0`**: Claude Code tidak menghitung pesan yang Anda kirim tanpa `origin` itu, dan tidak menghitung notifikasi tugas, jadi giliran masih dapat mengikuti.
* **Tidak ada**: hasil akhir yang Claude Code pancarkan setelah kerusakan atau kesalahan startup fatal menghilangkan bidang, dan [mungkin membawa total yang dinolkan](/docs/id/agent-sdk/cost-tracking#recover-totals-after-a-session-crash).

<h4 id="startup_failure_reason">
  `startup_failure_reason`
</h4>

Mengapa Claude Code menolak untuk memulai, sehingga aplikasi Anda dapat menawarkan perbaikan daripada percobaan ulang. Claude Code menetapkannya pada hasil `error_during_execution` yang ditulis sebelum keluar pada kegagalan startup yang diketahui. Hasil itu membawa total yang dinolkan, dan array `errors` membawa teks yang sama dengan stderr. Bidang tidak ada pada setiap hasil lainnya. Memerlukan Agent SDK v0.3.274 atau lebih baru.

Atur `CLAUDE_CODE_STARTUP_FAILURE_RESULTS` ke `1` dalam [`env`](#options) untuk menerima hasil ini untuk setiap nilai `SDKStartupFailureReason`. Tanpa variabel itu, Claude Code menulis hasil hanya untuk kegagalan ini, dan sisanya berakhir dengan output stderr, keluar non-nol, dan tidak ada pesan hasil:

* Resume yang Claude Code hentikan karena tidak dapat [mengembalikan sesi ke worktree-nya](/docs/id/worktrees#the-session-resumes-outside-its-worktree), dengan `worktree_unverified` atau `worktree_resume_refused`. Bagian itu mengatakan kesalahan mana yang membawa nilai mana.
* [`continue`](#options) yang ditolak dari percakapan yang dipegang sesi latar belakang, dengan `session_held_by_background`. Untuk [`resume`](#options) yang ditolak dari percakapan seperti itu, Claude Code menulis hasil hanya ketika variabel diatur.

```typescript theme={null}
type SDKStartupFailureReason =
  | "org_pin_api_key_conflict"
  | "org_verify_failed"
  | "org_pin_mismatch"
  | "managed_settings_invalid"
  | "remote_settings_required_unavailable"
  | "gateway_signin_required"
  | "gateway_access_denied"
  | "proxy_invalid"
  | "temp_dir_unusable"
  | "cwd_unavailable"
  | "shell_tool_missing"
  | "session_held_by_background"
  | "worktree_resume_refused"
  | "worktree_unverified"
  | "cli_version_too_old"
  | "bypass_root";
```

Setiap nilai menamai satu penolakan:

| Nilai                                  | Apa yang menghentikan sesi                                                                                                                                                                                              |
| :------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `org_pin_api_key_conflict`             | Pengaturan terkelola [memerlukan masuk gateway first-party atau Cloud](/docs/id/authentication#restrict-login-to-your-organization), dan kunci API Anthropic, token auth, atau `apiKeyHelper` dikonfigurasi sebagai gantinya |
| `org_verify_failed`                    | Organisasi masuk tidak dapat diverifikasi terhadap pin, misalnya karena kegagalan jaringan atau token yang dicabut                                                                                                      |
| `org_pin_mismatch`                     | Masuk milik organisasi yang pin tidak izinkan                                                                                                                                                                           |
| `managed_settings_invalid`             | Pengaturan kebijakan terkelola tidak dapat dibaca, atau pin tidak menamai organisasi                                                                                                                                    |
| `remote_settings_required_unavailable` | Pengaturan terkelola yang organisasi perlukan tidak dapat dimuat                                                                                                                                                        |
| `gateway_signin_required`              | [Gateway cloud](/docs/id/claude-apps-gateway) mengakhiri masuk ini                                                                                                                                                           |
| `gateway_access_denied`                | Permintaan pengaturan terkelola ke gateway cloud kembali dengan 403, yang [tabel pemecahan masalah](/docs/id/claude-apps-gateway-deploy#troubleshooting) gateway mencakup                                                    |
| `proxy_invalid`                        | Pengaturan proxy bukan URL lengkap                                                                                                                                                                                      |
| `temp_dir_unusable`                    | Direktori sementara per-pengguna tidak aman atau tidak dapat dibuat                                                                                                                                                     |
| `cwd_unavailable`                      | Direktori kerja dihapus, dipindahkan, atau tidak dapat dibaca                                                                                                                                                           |
| `shell_tool_missing`                   | Di Windows, tidak ada alat shell yang tersedia: Git Bash hilang, dan PowerShell hilang atau dimatikan dengan `CLAUDE_CODE_USE_POWERSHELL_TOOL`                                                                          |
| `session_held_by_background`           | Percakapan untuk dilanjutkan atau diteruskan berjalan sebagai [sesi latar belakang](/docs/id/agent-view)                                                                                                                     |
| `worktree_resume_refused`              | Worktree sesi gagal pemeriksaan keamanannya, atau resume diluncurkan dari dalamnya. `errors` mengatakan apakah menjalankan resume yang sama lagi berlanjut tanpa worktree                                               |
| `worktree_unverified`                  | Worktree sesi tidak dapat diverifikasi sekarang, dan mencoba lagi mungkin berhasil                                                                                                                                      |
| `cli_version_too_old`                  | Versi Claude Code ini di bawah minimum yang Anthropic perlukan                                                                                                                                                          |
| `bypass_root`                          | Mode izin bypass diminta saat berjalan sebagai root                                                                                                                                                                     |

<h3 id="sdksystemmessage">
  `SDKSystemMessage`
</h3>

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
    source?: string;
  }[];
  model: string;
  permissionMode: PermissionMode;
  slash_commands: string[];
  terminal_slash_commands?: string[];
  output_style: string;
  skills: string[];
  plugins: { name: string; path: string }[];
  fast_mode_state?: FastModeState;
  fast_mode_disabled_reason?: FastModeDisabledReason;
  effort?: "low" | "medium" | "high" | "xhigh" | "max" | null;
  capabilities?: string[];
};
```

`fast_mode_state` melaporkan status [fast mode](/docs/id/fast-mode) sesi. Ketika sesuatu memblokir fast mode, `fast_mode_disabled_reason` menamai pemeriksaan yang memblokirnya; bidang memerlukan Claude Code v2.1.219 atau lebih baru. Untuk kode alasan dan artinya, lihat [`fast_mode_disabled_reason`](#sdkresultmessage) pada pesan hasil.

`terminal_slash_commands` menamai entri dalam `slash_commands` yang antarmukanya terikat ke terminal lokal, seperti `exit`. Anda dapat mengirimnya seperti entri lain dalam `slash_commands`; bidang ada sehingga klien jarak jauh atau mobile dapat menyembunyikannya dari menu perintah. Bidang ada hanya ketika tidak kosong, dan memerlukan Agent SDK v0.3.229 atau lebih baru.

*

`source` pada setiap entri `mcp_servers`: dari mana definisi server berasal, dengan nilai yang sama seperti [`McpServerStatus`](#mcpserverstatus) `source`. Memerlukan Agent SDK v0.3.274 atau lebih baru.

*

`effort`: [tingkat upaya](/docs/id/model-config#adjust-effort-level) yang Claude Code kirim pada permintaan sesi berikutnya, atau `null` ketika tidak mengirim apa pun. Claude Code menetapkan bidang hanya pada pesan init yang dikirimnya ke klien [Remote Control](/docs/id/remote-control), dan menghilangkannya dari pesan init yang dibaca aplikasi Anda. Memerlukan Agent SDK v0.3.234 atau lebih baru.

Array `capabilities` menamai perilaku protokol yang diimplementasikan CLI ini, jadi Anda dapat mendeteksi fitur daripada membandingkan string `claude_code_version`. Ini adalah set terbuka: abaikan nilai yang tidak Anda kenal, dan periksa kemampuan spesifik yang perilakunya Anda andalkan. Bidang memerlukan Claude Code v2.1.205 atau lebih baru dan tidak ada pada CLI sebelumnya.

| Kemampuan                    | Arti                                                                                                                                                                                                                                                                                                                      |
| ---------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `interrupt_receipt_v1`       | [`interrupt()`](#query-object) menyelesaikan dengan penerimaan [`SDKControlInterruptResponse`](#sdkcontrolinterruptresponse) yang mencantumkan pesan yang tertunda ketika gangguan tiba                                                                                                                                   |
| `interrupt_cancel_queued_v1` | Permintaan kontrol `interrupt` menghormati `cancel_queued: true`, membatalkan pesan yang akan dicantumkan penerimaan di bawah `still_queued` dan mencantumnya di bawah `cancelled` sebagai gantinya. Lihat [`SDKControlInterruptResponse`](#sdkcontrolinterruptresponse). Memerlukan Claude Code v2.1.219 atau lebih baru |

<h3 id="sdkpartialassistantmessage">
  `SDKPartialAssistantMessage`
</h3>

Pesan parsial streaming (hanya ketika `includePartialMessages` adalah true). Bidang `parent_tool_use_id` selalu `null`: acara aliran dipancarkan untuk sesi utama saja. Untuk atribusi subagen, gunakan pesan lengkap, yang membawa `parent_tool_use_id`, atau aktifkan [`forwardSubagentText`](#options) untuk menerima teks dan pemikiran subagen sebagai pesan lengkap.

```typescript theme={null}
type SDKPartialAssistantMessage = {
  type: "stream_event";
  event: BetaRawMessageStreamEvent; // Dari Anthropic SDK
  parent_tool_use_id: string | null;
  uuid: UUID;
  session_id: string;
  ttft_ms?: number; // Waktu ke token pertama dalam ms, hadir hanya pada acara message_start
  user_message_uuid?: string;
  user_message_uuids?: string[];
};
```

Claude Code menetapkan `user_message_uuid` dan `user_message_uuids` pada acara aliran non-ping pertama giliran, dan lagi ketika pesan yang dijawab giliran berubah, di bawah kondisi dalam [`user_message_uuid`](#user_message_uuid).

<h3 id="sdkcompactboundarymessage">
  `SDKCompactBoundaryMessage`
</h3>

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

<h3 id="sdkinformationalmessage">
  `SDKInformationalMessage`
</h3>

Spanduk teks generik yang dipancarkan oleh loop. Membawa baris status non-kesalahan, umpan balik hook seperti alasan blokir hook `UserPromptSubmit`, dan output perintah. Pada Claude Code v2.1.227 atau lebih baru, [`systemMessage`](/docs/id/hooks#json-output) hook dapat tiba sebagai pesan ini, dengan setiap baris diawali oleh nama hook, seperti `PostToolUse:Bash says:`. Apakah `systemMessage` hook tiba sebagai pesan ini tergantung pada acara. Setiap [bagian acara](/docs/id/hooks#hook-events) pada halaman hooks mengatakan bagaimana output muncul. Render `content` sebagai plaintext pada `level` yang diberikan.

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

<h3 id="sdkworkershuttingdownmessage">
  `SDKWorkerShuttingDownMessage`
</h3>

Dipancarkan pada pembongkaran pekerja yang anggun sehingga klien jarak jauh dapat menunjukkan mengapa pekerja keluar daripada menunggu timeout detak jantung. `reason` adalah string snake\_case pendek yang ditetapkan oleh CLI host, seperti `"host_exit"` atau `"remote_control_disabled"`. Bertindak atas ini hanya ketika streaming langsung. Sesi yang dilanjutkan memutar ulang instans masa lalu dari pesan ini, jadi abaikan dalam kasus itu.

```typescript theme={null}
type SDKWorkerShuttingDownMessage = {
  type: "system";
  subtype: "worker_shutting_down";
  reason: string;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkplugininstallmessage">
  `SDKPluginInstallMessage`
</h3>

Acara kemajuan instalasi plugin. Dipancarkan ketika [`CLAUDE_CODE_SYNC_PLUGIN_INSTALL`](/docs/id/env-vars) diatur, jadi aplikasi Agent SDK Anda dapat melacak instalasi plugin marketplace sebelum giliran pertama. Status `started` dan `completed` mengurung instalasi keseluruhan. Status `installed` dan `failed` melaporkan marketplace individual dan menyertakan `name`.

```typescript theme={null}
type SDKPluginInstallMessage = {
  type: "system";
  subtype: "plugin_install";
  status: "started" | "installed" | "failed" | "completed";
  name?: string;
  error?: string;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkpermissiondeniedmessage">
  `SDKPermissionDeniedMessage`
</h3>

Acara aliran yang dipancarkan ketika sistem izin menolak panggilan alat tanpa prompt interaktif. Gunakan untuk merender penolakan di UI Anda saat terjadi, daripada hanya mengamati hasil alat `is_error` yang mengikuti. Penolakan mana yang dilaporkan tergantung pada bagaimana jalannya menangani prompt izin:

* **Dengan callback [`canUseTool`](#canusetool) dan [`permissionPrompts: 'host'`](#options) default**: prompt izin pergi ke callback Anda, dan acara ini melaporkan penolakan yang Claude Code putuskan sendiri tanpa memanggilnya.
*

**Tanpa keduanya**: jalankan `-p` telanjang, atau `query()` yang tidak menetapkan `canUseTool` atau `permissionPromptToolName`, menolak panggilan alat apa pun yang akan diminta, dan acara ini melaporkan penolakan itu serta yang Claude Code putuskan sendiri. Sebelum v2.1.223, Claude Code tidak memancarkan acara ini dalam jalankan tanpa callback.

* **Dengan alat prompt MCP**, diatur dengan `permissionPromptToolName` atau flag [`--permission-prompt-tool`](/docs/id/cli-reference#cli-flags), dan `permissionPrompts: 'host'` default: Claude Code tidak memancarkan acara ini sama sekali, bahkan untuk penolakan aturan yang diputuskan sendiri.
*

**Dengan [`permissionPrompts: 'none'`](#options)**: Claude Code menolak panggilan yang akan diminta, bahkan ketika `canUseTool` atau alat prompt MCP juga diatur, dan acara ini melaporkan penolakan itu serta yang Claude Code putuskan sendiri. Memerlukan Claude Code v2.1.259 atau lebih baru.

Dalam setiap konfigurasi, acara ini melewati penolakan apa pun yang diputuskan pada jalur hook `PreToolUse`, apakah hook menolak panggilan sendiri atau aturan deny menimpa keputusan allow atau ask hook. Acara ini juga best-effort: kadang-kadang Claude Code merekam penolakan tanpa memancarkan acara ini, jadi `permission_denials` pada [pesan hasil](#sdkresultmessage) adalah catatan otoritatif.

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

| Bidang                 | Tipe     | Deskripsi                                                                                                                             |
| ---------------------- | -------- | ------------------------------------------------------------------------------------------------------------------------------------- |
| `tool_name`            | `string` | Nama alat yang ditolak                                                                                                                |
| `tool_use_id`          | `string` | ID blok `tool_use` yang dijawab penolakan ini                                                                                         |
| `agent_id`             | `string` | ID subagen ketika panggilan yang ditolak berasal dari dalam subagen. Mencerminkan bidang pada `can_use_tool` untuk perutean sisi host |
| `decision_reason_type` | `string` | Diskriminator untuk komponen yang memutuskan, seperti `"rule"`, `"mode"`, `"classifier"`, atau `"asyncAgent"`                         |
| `decision_reason`      | `string` | Alasan yang dapat dibaca manusia dari komponen yang memutuskan, ketika tersedia                                                       |
| `message`              | `string` | Pesan penolakan yang dikembalikan ke model dalam `tool_result`                                                                        |

<h3 id="sdkpermissiondenial">
  `SDKPermissionDenial`
</h3>

Informasi tentang penggunaan alat yang ditolak.

```typescript theme={null}
type SDKPermissionDenial = {
  tool_name: string;
  tool_use_id: string;
  tool_input: Record<string, unknown>;
};
```

<h3 id="sdkcontextusage">
  `SDKContextUsage`
</h3>

Bentuk terstruktur dari laporan `/context`, dibawa sebagai `context_usage` pada [`SDKAssistantMessage`](#sdkassistantmessage) yang mengirimkan hasil `/context`. Agent SDK v0.3.232 dan lebih baru mengekspor tipe. Tidak seperti [`SDKControlGetContextUsageResponse`](#sdkcontrolgetcontextusageresponse), ia hanya membawa data yang diperlukan untuk merender rincian penggunaan, tanpa bidang tampilan seperti `color` dan `gridRows`.

```typescript theme={null}
type SDKContextUsage = {
  model: string;
  total_tokens: number;
  raw_max_tokens: number;
  percentage: number;
  over_limit?: {
    tokens_over: number;
    kind: "hard_limit" | "compaction_window";
  };
  categories: SDKContextUsageCategory[];
  mcp_tools: {
    name: string;
    server_name: string;
    tokens: number;
  }[];
  memory_files: {
    path: string;
    type: string;
    tokens: number;
  }[];
  agents: {
    agent_type: string;
    source: string;
    tokens: number;
  }[];
  skills?: {
    name: string;
    source: string;
    plugin_name?: string;
    tokens: number;
  }[];
};
```

Tabel mencantumkan apa yang Claude Code masukkan dalam setiap bidang. Bidang dari `model` melalui `over_limit` menggambarkan sesi secara keseluruhan, dan bidang koleksi mengatribusikan token ke item individual.

| Bidang           | Tipe                                                      | Deskripsi                                                                                                                                                                                                                                                                                                             |
| ---------------- | --------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `model`          | `string`                                                  | Model loop utama yang Claude Code hitung penggunaan, bukan subagen                                                                                                                                                                                                                                                    |
| `total_tokens`   | `number`                                                  | Perkiraan Claude Code tentang token yang digunakan. Tidak diklem ke jendela, jadi dapat melebihi `raw_max_tokens` ketika sesi melampaui batas                                                                                                                                                                         |
| `raw_max_tokens` | `number`                                                  | Jendela konteks model, atau [jendela auto-compact](/docs/id/model-config#context-window-and-auto-compaction) yang lebih rendah ketika satu berlaku, seperti yang Anda atur atau batas 200K yang Claude Code terapkan pada beberapa model dengan jendela 1M-token. Claude Code mengukur `total_tokens` terhadap jendela ini |
| `percentage`     | `number`                                                  | `total_tokens` sebagai persentase pembulatan `raw_max_tokens`, jadi dapat melebihi 100 ketika sesi melampaui batas                                                                                                                                                                                                    |
| `over_limit`     | `object`                                                  | Hadir hanya ketika `total_tokens` melebihi `raw_max_tokens`. `tokens_over` adalah jumlah yang berlebihan, dan `kind` mengatakan bagaimana Claude Code menyelesaikan jendela                                                                                                                                           |
| `categories`     | [`SDKContextUsageCategory`](#sdkcontextusagecategory)`[]` | Satu entri per baris rincian penggunaan-per-kategori                                                                                                                                                                                                                                                                  |
| `mcp_tools`      | `object[]`                                                | Token yang diatribusikan ke setiap alat MCP, dengan nama kawatnya, seperti `mcp__linear__create_issue`, dan `server_name`                                                                                                                                                                                             |
| `memory_files`   | `object[]`                                                | Token yang diatribusikan ke setiap file memori yang dimuat, dengan `path` dan label sumber seperti `Project` atau `User` dalam `type`                                                                                                                                                                                 |
| `agents`         | `object[]`                                                | Token yang diatribusikan ke setiap definisi subagen kustom, dengan pengidentifikasi sumber seperti `projectSettings`, `userSettings`, atau `plugin`. Subagen bawaan tidak tercantum                                                                                                                                   |
| `skills`         | `object[]`                                                | Token yang diatribusikan ke setiap skill dalam daftar skill, dengan pengidentifikasi sumber dan, untuk skill plugin, nama plugin dalam `plugin_name`. Tidak ada ketika tidak ada skill yang berkontribusi token                                                                                                       |

`over_limit.kind` merekam bagaimana Claude Code menyelesaikan jendela, bukan apakah API menerima permintaan berikutnya:

* `hard_limit`: jendela adalah apa yang Claude Code percaya menjadi batas model sendiri, melampaui mana API menolak permintaan
* `compaction_window`: jendela adalah jendela kebijakan pemadatan, yang mungkin atau mungkin tidak bertepatan dengan batas model

Claude Code mengembangkan tipe secara aditif, menambahkan data baru sebagai bidang opsional daripada membentuk ulang yang ada. Baca bidang yang Anda ketahui dan abaikan yang tidak Anda kenal.

<h3 id="sdkcontextusagecategory">
  `SDKContextUsageCategory`
</h3>

Satu baris rincian penggunaan-per-kategori `/context`.

```typescript theme={null}
type SDKContextUsageCategory = {
  name: string;
  tokens: number;
  kind: "used" | "free" | "buffer" | "deferred";
};
```

Tabel mencantumkan apa yang Claude Code masukkan dalam setiap bidang baris.

| Bidang   | Tipe     | Deskripsi                                                                                                                               |
| -------- | -------- | --------------------------------------------------------------------------------------------------------------------------------------- |
| `name`   | `string` | Nama tampilan baris seperti `/context` mencetaknya, seperti `Messages`. Klasifikasikan baris berdasarkan `kind`, bukan berdasarkan nama |
| `tokens` | `number` | Hitungan token baris. Baris dapat membawa nol token                                                                                     |
| `kind`   | `string` | Apa yang diwakili baris: `used`, `free`, `buffer`, atau `deferred`                                                                      |

Setiap nilai `kind` mengatakan apa token baris:

* `used`: konten yang menempati jendela konteks
* `free`: jendela yang tersisa
* `buffer`: cadangan pemadatan
* `deferred`: skema alat yang Claude Code tahan di luar jendela dan kecualikan dari perhitungan penggunaan, tercantum untuk kesadaran

<h3 id="sdkmessageorigin">
  `SDKMessageOrigin`
</h3>

Asal usul pesan peran pengguna. Ini muncul sebagai `origin` pada [`SDKUserMessage`](#sdkusermessage) dan diteruskan ke [`SDKResultMessage`](#sdkresultmessage) yang sesuai sehingga Anda dapat mengetahui apa yang memicu giliran tertentu.

```typescript theme={null}
type SDKMessageOrigin =
  | { kind: "human" }
  | { kind: "channel"; server: string }
  | {
      kind: "peer";
      from: string;
      fromMode?: "bypass" | "prompting";
      name?: string;
      fromSession?: string;
      senderTaskId?: string;
      body?: string;
      verifiedPeerPid?: number;
    }
  | {
      kind: "task-notification";
      subkind?: "scheduled-trigger" | "peer-send-message";
      fireReason?: string;
    }
  | { kind: "coordinator" }
  | { kind: "auto-continuation" }
  | { kind: "unclassified" };
```

| `kind`              | Arti                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| ------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `human`             | Input langsung dari pengguna akhir. Jika aplikasi Anda meneruskan apa yang diketik pengguna sebagai pesan pengguna, atur `origin` ke `{ kind: "human" }` secara eksplisit: Claude Code memperlakukan pesan pengguna tanpa `origin` sebagai tidak teratribusi, dan memeriksa yang memerlukan prompt yang diketik manusia, seperti [kata kunci workflow `ultracode`](/docs/id/workflows#ask-for-a-workflow-in-your-prompt), tidak menerimanya. Sebelum v2.1.210, Claude Code memperlakukan `origin` yang tidak ada pada pesan pengguna sebagai input manusia. |
| `channel`           | Pesan tiba di [channel](/docs/id/channels). `server` adalah nama server MCP sumber.                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `peer`              | Pesan dari agen lain: [rekan kerja](/docs/id/agent-teams) dalam proses atau [rekan lintas sesi](/docs/id/cross-session-messaging), sesi Claude Code lain Anda. Lihat [Bidang asal peer](#peer-origin-fields) untuk semantik per-bidang dan model kepercayaan.                                                                                                                                                                                                                                                                                                    |
| `task-notification` | Giliran sintetis yang disuntikkan untuk pengiriman yang tiba tanpa prompt pengguna segar, seperti tugas yang selesai; lihat [`SDKTaskNotificationMessage`](#sdktasknotificationmessage) untuk lengan itu. Prompt yang aplikasi Anda [deklarasikan sebagai jalankan terjadwal](#declare-a-scheduled-run) membawa jenis ini juga. `subkind` opsional menandai apa yang menaikkan notifikasi. Lihat [Subkind notifikasi tugas](#task-notification-subkinds).                                                                                              |
| `coordinator`       | Pesan dari koordinator tim dalam [tim agen](/docs/id/agent-teams).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `auto-continuation` | Giliran sintetis yang disuntikkan ketika sesi berlanjut tanpa input pengguna segar, seperti hasil perintah yang memicu prompt tindak lanjut.                                                                                                                                                                                                                                                                                                                                                                                                           |
| `unclassified`      | Giliran yang disuntikkan yang asal usulnya tidak dapat ditentukan. Memerlukan Claude Code v2.1.223 atau lebih baru. Ketika Claude Code menerima [`SDKUserMessage`](#sdkusermessage) dengan `isSynthetic: true` dan tidak dapat mengklasifikasikannya sebagai `kind` lain, ia menetapkan jenis ini saat pesan tiba dan membingkai giliran ke model sebagai sumber non-pengguna daripada memperlakukannya sebagai input manusia. Aplikasi Anda tidak boleh menetapkan nilai ini.                                                                         |

<h3 id="task-notification-subkinds">
  Subkind notifikasi tugas
</h3>

Ketika Claude Code mengirimkan notifikasi tugas ke sesi, ia menetapkan `subkind` pada `origin` notifikasi jika server Anthropic memverifikasi dari mana notifikasi itu berasal. Ia juga menetapkan `subkind` ketika aplikasi Anda [mendeklarasikan pesan sebagai jalankan terjadwal](#declare-a-scheduled-run) sendiri, yang memerlukan TypeScript Agent SDK v0.3.280 atau lebih baru. `subkind` memerlukan Claude Code v2.1.213 atau lebih baru, dan mengambil salah satu dari dua nilai:

* `scheduled-trigger`: notifikasi adalah prompt tersimpan [rutinitas](/docs/id/routines), disampaikan karena salah satu pemicu rutinitas dipecat: jadwalnya, [pemicu API](/docs/id/routines#add-an-api-trigger), [pemicu GitHub](/docs/id/routines#add-a-github-trigger), atau **Jalankan sekarang**. Prompt yang aplikasi Anda [deklarasikan sebagai jalankan terjadwal](#declare-a-scheduled-run) membawa nilai ini juga. Claude Code membingkai ini ke model sebagai tugas yang ditugaskan sesi, dengan pemberitahuan berbeda dari [pemberitahuan yang dibawa notifikasi tugas lain](#sdktasknotificationmessage).
*

`peer-send-message`: notifikasi adalah pesan yang sesi lain Anda kirim dengan alat `send_message` sisi server yang [Claude Code di web](/docs/id/claude-code-on-the-web) gunakan sesi untuk saling berkirim pesan, bukan [alat `SendMessage` lintas sesi](/docs/id/cross-session-messaging), dan server Anthropic memverifikasi bahwa kedua sesi termasuk dalam grup pribadi sesi yang sama. Memerlukan Claude Code v2.1.224 atau lebih baru. Pengiriman `send_message` yang tidak diverifikasi server dengan cara itu tidak mendapat subkind.

Setiap notifikasi tugas lain tidak memiliki `subkind`. Itu termasuk [aktivitas PR](/docs/id/claude-code-on-the-web#how-claude-responds-to-pr-activity) yang disampaikan ke sesi dan acara latar belakang seperti tugas yang selesai. Pesan dari [alat `SendMessage` lintas sesi](/docs/id/cross-session-messaging) sama sekali bukan notifikasi tugas: apakah mereka berasal dari sesi di mesin yang sama atau melalui server Anthropic dari mesin lain, Claude Code memberi mereka `kind: "peer"` dan [bidang asal peer](#peer-origin-fields).

`fireReason` mengatakan mengapa notifikasi `scheduled-trigger` dipecat, sebagai token huruf kecil pendek seperti `scheduled`, `manual`, `retry`, `catch_up`, atau `api`. Server Anthropic menetapkannya pada pengiriman [rutinitas](/docs/id/routines), dan aplikasi Anda menetapkannya ketika mendeklarasikan jalankan terjadwal. Itu tidak ada ketika tidak ada yang mengirimnya. Memerlukan TypeScript Agent SDK v0.3.280 atau lebih baru.

<h4 id="declare-a-scheduled-run">
  Deklarasikan jalankan terjadwal
</h4>

Jika aplikasi Anda menjalankan prompt sesuai jadwal sendiri, deklarasikan setiap jalankan sehingga Claude Code membingkai giliran ke model sebagai tugas terjadwal daripada sebagai input langsung dari pengguna. Mulai sesi dengan `CLAUDE_CODE_HOST_SCHEDULED_RUN` diatur ke `1` dalam [`env`](#options), kemudian kirim [`SDKUserMessage`](#sdkusermessage) jalankan dengan `origin: { kind: "task-notification", subkind: "scheduled-trigger", fireReason: "scheduled" }` dan tanpa `isSynthetic`. Claude Code mengabaikan deklarasi dalam proses yang dimulai tanpa variabel itu. Ia juga mengabaikannya dalam proses yang lingkungannya membawa [`CLAUDECODE`](/docs/id/env-vars) atau `CLAUDE_CODE_CHILD_SESSION`. Claude Code menyimpan `fireReason` hanya ketika nilainya adalah 1 hingga 32 huruf kecil atau garis bawah. Memerlukan TypeScript Agent SDK v0.3.280 atau lebih baru.

<h3 id="peer-origin-fields">
  Bidang asal peer
</h3>

Asal `peer` mengidentifikasi agen mana yang mengirim pesan: [rekan kerja](/docs/id/agent-teams) dalam proses mengirim ke `main` dengan `SendMessage`, atau [rekan lintas sesi](/docs/id/cross-session-messaging), sesi Claude Code lain Anda. Rekan lintas sesi memerlukan Claude Code v2.1.224 atau lebih baru di macOS dan Linux; lihat [ketersediaan pesan lintas sesi](/docs/id/cross-session-messaging#availability) untuk persyaratan Windows asli. Rekan lintas sesi dapat berjalan di mesin yang sama, atau di [mesin lain Anda](/docs/id/cross-session-messaging#message-sessions-on-other-machines) atau [Claude Code di web](/docs/id/claude-code-on-the-web) ketika pesannya tiba melalui Remote Control. Dua jenis pengirim mengisi bidang secara berbeda:

* `from`: nama rekan kerja, atau alamat pengirim untuk rekan lintas sesi. Untuk [pesan lintas mesin satu arah](/docs/id/cross-session-messaging#message-sessions-on-other-machines), pengirim tidak memiliki alamat balasan dan `from` adalah `"unknown"`. Nilai adalah yang dibuat pengirim; `verifiedPeerPid` adalah identitas yang diverifikasi.
*

`fromMode`: kelas izin sesi pengiriman, `bypass` atau `prompting`, dideklarasikan oleh host yang meneruskan pesan peer antara sesi Anda, seperti [aplikasi desktop](/docs/id/desktop#work-across-sessions). Claude Code membacanya dalam sesi penerima ketika menerapkan [kontrol inbound](/docs/id/cross-session-messaging#control-inbound-messages). Memerlukan Agent SDK v0.3.234 atau lebih baru.

* `senderTaskId`: ID tugas rekan kerja. Tidak ada untuk rekan lintas sesi.
*

`name`: nama tampilan pengirim, dinormalisasi oleh Claude Code: ia menghapus kontrol Unicode, format, surrogate, dan pemisah baris atau paragraf, kemudian memangkas hasil dan membatasinya pada 64 poin kode dengan elipsis. Memerlukan Claude Code v2.1.205 atau lebih baru.

*

`body`: badan pesan yang didekode dengan amplop peer dihapus, byte-tepat dengan apa yang dilihat model. Selalu ada untuk pesan rekan kerja; untuk rekan lintas sesi, ada hanya ketika giliran adalah tepat satu amplop peer yang dibentuk Claude Code. Render `name` dan `body` daripada mengurai ulang teks pesan. Memerlukan Claude Code v2.1.205 atau lebih baru.

*

`fromSession`: ID sesi yang dapat dibuka host pengirim, diatur oleh host pengirim sehingga UI Anda dapat menautkan kembali ke sesi pengiriman. Seperti `from`, itu adalah yang diklaim pengirim: gunakan hanya sebagai target navigasi, dan jangan perlakukan sebagai bukti identitas pengirim. Memerlukan Claude Code v2.1.216 atau lebih baru.

*

`verifiedPeerPid`: ID proses dari proses yang terhubung ke soket pesan lintas sesi sesi ini, diverifikasi oleh kernel dan dibaca dari koneksi itu sendiri, tidak pernah dari muatan. Gunakan itu, bukan `from`, untuk mengidentifikasi pengirim: `from` dapat dipalsukan oleh proses pengguna yang sama. Bidang tidak ada ketika Claude Code tidak dapat memverifikasinya, seperti di Windows atau ingress non-soket, jadi nilai yang tidak ada berarti pengirim tidak diverifikasi. Untuk lalu lintas yang diteruskan itu mengidentifikasi relay daripada penulis pesan, dan ID proses dapat didaur ulang, jadi perlakukan sebagai asal usul daripada token autentikasi. Memerlukan Claude Code v2.1.216 atau lebih baru.

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
  | "UserPromptExpansion"
  | "SessionStart"
  | "SessionEnd"
  | "Stop"
  | "StopFailure"
  | "SubagentStart"
  | "SubagentStop"
  | "PreCompact"
  | "PostCompact"
  | "PreModelSwitch"
  | "PostModelSwitch"
  | "PermissionRequest"
  | "PermissionDenied"
  | "Setup"
  | "TeammateIdle"
  | "TaskCreated"
  | "TaskCompleted"
  | "Elicitation"
  | "ElicitationResult"
  | "ConfigChange"
  | "DirectoryAdded"
  | "WorktreeCreate"
  | "WorktreeRemove"
  | "InstructionsLoaded"
  | "CwdChanged"
  | "FileChanged"
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
  | PermissionDeniedHookInput
  | NotificationHookInput
  | UserPromptSubmitHookInput
  | UserPromptExpansionHookInput
  | SessionStartHookInput
  | SessionEndHookInput
  | StopHookInput
  | StopFailureHookInput
  | SubagentStartHookInput
  | SubagentStopHookInput
  | PreCompactHookInput
  | PostCompactHookInput
  | PreModelSwitchHookInput
  | PostModelSwitchHookInput
  | PermissionRequestHookInput
  | SetupHookInput
  | TeammateIdleHookInput
  | TaskCreatedHookInput
  | TaskCompletedHookInput
  | ElicitationHookInput
  | ElicitationResultHookInput
  | ConfigChangeHookInput
  | InstructionsLoadedHookInput
  | DirectoryAddedHookInput
  | WorktreeCreateHookInput
  | WorktreeRemoveHookInput
  | CwdChangedHookInput
  | FileChangedHookInput
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
  mcp_server?: McpServerProvenance;
};
```

`mcp_server` hadir ketika alat berasal dari server MCP; lihat [`McpServerProvenance`](#mcpserverprovenance). Input `PostToolUse`, `PostToolUseFailure`, `PermissionRequest`, dan `PermissionDenied` membawa bidang yang sama. Bidang ini memerlukan Agent SDK v0.3.274 atau lebih baru.

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
  mcp_server?: McpServerProvenance;
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
  mcp_server?: McpServerProvenance;
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

<h4 id="permissiondeniedhookinput">
  `PermissionDeniedHookInput`
</h4>

```typescript theme={null}
type PermissionDeniedHookInput = BaseHookInput & {
  hook_event_name: "PermissionDenied";
  tool_name: string;
  tool_input: unknown;
  tool_use_id: string;
  reason: string;
  mcp_server?: McpServerProvenance;
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
  session_title?: string;
};
```

<h4 id="userpromptexpansionhookinput">
  `UserPromptExpansionHookInput`
</h4>

```typescript theme={null}
type UserPromptExpansionHookInput = BaseHookInput & {
  hook_event_name: "UserPromptExpansion";
  expansion_type: "slash_command" | "mcp_prompt";
  command_name: string;
  command_args: string;
  command_source?: string;
  prompt: string;
};
```

<h4 id="sessionstarthookinput">
  `SessionStartHookInput`
</h4>

```typescript theme={null}
type SessionStartHookInput = BaseHookInput & {
  hook_event_name: "SessionStart";
  source: "startup" | "resume" | "clear" | "compact" | "fork";
  agent_type?: string;
  model?: string;
  session_title?: string;
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

<h4 id="stopfailurehookinput">
  `StopFailureHookInput`
</h4>

```typescript theme={null}
type StopFailureHookInput = BaseHookInput & {
  hook_event_name: "StopFailure";
  error: SDKAssistantMessageError;
  error_details?: string;
  last_assistant_message?: string;
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

<h4 id="postcompacthookinput">
  `PostCompactHookInput`
</h4>

```typescript theme={null}
type PostCompactHookInput = BaseHookInput & {
  hook_event_name: "PostCompact";
  trigger: "manual" | "auto";
  compact_summary: string;
};
```

<h4 id="premodelswitchhookinput">
  `PreModelSwitchHookInput`
</h4>

Dipicu sebelum perubahan model yang diminta berlaku. `context_tokens` dan bidang setelahnya memperkirakan biaya pengiriman ulang percakapan ke model baru. Untuk deskripsi bidang lengkap dan semantik pemblokiran, lihat [PreModelSwitch](/docs/id/hooks#premodelswitch).

```typescript theme={null}
type PreModelSwitchHookInput = BaseHookInput & {
  hook_event_name: "PreModelSwitch";
  from_model: string;
  to_model: string;
  requested_model: string | null;
  source: "command" | "picker" | "sdk";
  context_tokens: number;
  prompt_cache_warm: boolean;
  cache_ttl: "5m" | "1h";
  estimated_cache_write_usd: number;
  pricing: "configured" | "catalog" | "default";
};
```

<h4 id="postmodelswitchhookinput">
  `PostModelSwitchHookInput`
</h4>

Dipicu setelah model sesi berubah. Ini membawa bidang yang sama dengan `PreModelSwitchHookInput`, dengan dua nilai `source` tambahan. Lihat [PostModelSwitch](/docs/id/hooks#postmodelswitch).

```typescript theme={null}
type PostModelSwitchHookInput = BaseHookInput & {
  hook_event_name: "PostModelSwitch";
  from_model: string;
  to_model: string;
  requested_model: string | null;
  source: "command" | "picker" | "sdk" | "auto" | "resume";
  context_tokens: number;
  prompt_cache_warm: boolean;
  cache_ttl: "5m" | "1h";
  estimated_cache_write_usd: number;
  pricing: "configured" | "catalog" | "default";
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
  mcp_server?: McpServerProvenance;
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

<h4 id="taskcreatedhookinput">
  `TaskCreatedHookInput`
</h4>

```typescript theme={null}
type TaskCreatedHookInput = BaseHookInput & {
  hook_event_name: "TaskCreated";
  task_id: string;
  task_subject: string;
  task_description?: string;
  teammate_name?: string;
  /** @deprecated sejak v2.1.178. Membawa nama tim yang diturunkan dari sesi; akan dihapus. */
  team_name?: string;
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

<h4 id="elicitationhookinput">
  `ElicitationHookInput`
</h4>

```typescript theme={null}
type ElicitationHookInput = BaseHookInput & {
  hook_event_name: "Elicitation";
  mcp_server_name: string;
  message: string;
  mode?: "form" | "url";
  url?: string;
  elicitation_id?: string;
  requested_schema?: Record<string, unknown>;
};
```

<h4 id="elicitationresulthookinput">
  `ElicitationResultHookInput`
</h4>

```typescript theme={null}
type ElicitationResultHookInput = BaseHookInput & {
  hook_event_name: "ElicitationResult";
  mcp_server_name: string;
  elicitation_id?: string;
  mode?: "form" | "url";
  action: "accept" | "decline" | "cancel";
  content?: Record<string, unknown>;
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

<h4 id="instructionsloadedhookinput">
  `InstructionsLoadedHookInput`
</h4>

```typescript theme={null}
type InstructionsLoadedHookInput = BaseHookInput & {
  hook_event_name: "InstructionsLoaded";
  file_path: string;
  memory_type: "User" | "Project" | "Local" | "Managed";
  load_reason:
    | "session_start"
    | "nested_traversal"
    | "path_glob_match"
    | "include"
    | "compact";
  globs?: string[];
  trigger_file_path?: string;
  parent_file_path?: string;
};
```

<h4 id="directoryaddedhookinput">
  `DirectoryAddedHookInput`
</h4>

```typescript theme={null}
type DirectoryAddedHookInput = BaseHookInput & {
  hook_event_name: "DirectoryAdded";
  directory: string;
  source: "slash_command" | "register_repo_root";
};
```

`directory` adalah jalur absolut dari direktori yang ditambahkan. `source` adalah `"slash_command"` ketika `/add-dir` menambahkannya dan `"register_repo_root"` ketika permintaan kontrol SDK melakukannya.

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

<h4 id="cwdchangedhookinput">
  `CwdChangedHookInput`
</h4>

```typescript theme={null}
type CwdChangedHookInput = BaseHookInput & {
  hook_event_name: "CwdChanged";
  old_cwd: string;
  new_cwd: string;
};
```

<h4 id="filechangedhookinput">
  `FileChangedHookInput`
</h4>

```typescript theme={null}
type FileChangedHookInput = BaseHookInput & {
  hook_event_name: "FileChanged";
  file_path: string;
  event: "change" | "add" | "unlink";
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
  /**
   * Urutan escape terminal (misalnya OSC 9 / OSC 777 desktop-notification)
   * untuk Claude Code yang akan dipancarkan atas nama Anda. Hanya notification/title OSCs
   * (0, 1, 2, 9, 99, 777) dan BEL yang diizinkan; nilai yang berisi
   * apa pun yang lain diabaikan secara keseluruhan. Hanya CLI interaktif yang memancarkannya;
   * SDK mengabaikan bidang ini.
   */
  terminalSequence?: string;
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
        sessionTitle?: string;
        /** Ketika decision adalah "block", hilangkan prompt asli dari pesan blok. */
        suppressOriginalPrompt?: boolean;
      }
    | {
        hookEventName: "UserPromptExpansion";
        additionalContext?: string;
      }
    | {
        hookEventName: "SessionStart";
        additionalContext?: string;
        initialUserMessage?: string;
        sessionTitle?: string;
        watchPaths?: string[];
        /**
         * Pindai ulang direktori skill dan command setelah hook SessionStart
         * selesai, sehingga skills yang diinstal oleh hook tersedia di
         * sesi yang sama.
         */
        reloadSkills?: boolean;
      }
    | {
        hookEventName: "Setup";
        additionalContext?: string;
      }
    | {
        hookEventName: "PreModelSwitch";
        /**
         * Kontrak yang sama dengan PreToolUse: "allow" melanjutkan, "deny" membatalkan
         * perubahan, "ask" meminta pengguna untuk mengonfirmasi. Hanya /model dalam sesi
         * interaktif yang menampilkan prompt itu; setiap permukaan lain,
         * termasuk permintaan set_model, memperlakukan "ask" sebagai penolakan.
         */
        permissionDecision?: "allow" | "deny" | "ask";
        permissionDecisionReason?: string;
      }
    | {
        hookEventName: "PostModelSwitch";
        /** Mencapai model dengan permintaan berikutnya yang dilayani model baru. */
        additionalContext?: string;
      }
    | {
        hookEventName: "SubagentStart";
        additionalContext?: string;
      }
    | {
        hookEventName: "PostToolUse";
        additionalContext?: string;
        /**
         * Catatan singkat tentang hasil pemanggilan alat ini untuk pengklasifikasi izin
         * mode otomatis. Dibatasi hingga 2000 karakter, dibagikan di seluruh
         * semua hook yang merespons panggilan yang sama; dihormati pada respons hook sinkron saja.
         * Jangan salin keluaran alat yang tidak dipercaya ke dalamnya.
         */
        classifierContext?: string;
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
        hookEventName: "Stop";
        additionalContext?: string;
      }
    | {
        hookEventName: "SubagentStop";
        additionalContext?: string;
      }
    | {
        hookEventName: "PermissionDenied";
        retry?: boolean;
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
      }
    | {
        hookEventName: "Elicitation";
        action?: "accept" | "decline" | "cancel";
        content?: Record<string, unknown>;
      }
    | {
        hookEventName: "ElicitationResult";
        action?: "accept" | "decline" | "cancel";
        content?: Record<string, unknown>;
      }
    | {
        hookEventName: "CwdChanged";
        watchPaths?: string[];
      }
    | {
        hookEventName: "FileChanged";
        watchPaths?: string[];
      }
    | {
        hookEventName: "WorktreeCreate";
        worktreePath: string;
      }
    | {
        hookEventName: "MessageDisplay";
        /** Teks yang ditampilkan sebagai pengganti delta. Hilangkan (atau kembalikan delta tidak berubah) untuk menampilkan yang asli. */
        displayContent?: string;
      };
};
```

<h2 id="tool-input-types">
  Jenis Input Tool
</h2>

Dokumentasi skema input untuk semua tool Claude Code bawaan. Jenis-jenis ini diekspor dari `@anthropic-ai/claude-agent-sdk` dan dapat digunakan untuk interaksi tool yang aman tipe.

<h3 id="toolinputschemas">
  `ToolInputSchemas`
</h3>

Gabungan jenis input tool yang diekspor dari `@anthropic-ai/claude-agent-sdk`; anggota mencakup:

```typescript theme={null}
type ToolInputSchemas =
  | AgentInput
  | ArtifactInput
  | AskUserQuestionInput
  | BashInput
  | CronCreateInput
  | CronDeleteInput
  | CronListInput
  | EnterPlanModeInput
  | EnterWorktreeInput
  | ExitPlanModeInput
  | ExitWorktreeInput
  | FileEditInput
  | FileReadInput
  | FileWriteInput
  | GlobInput
  | GrepInput
  | ListMcpResourcesInput
  | McpInput
  | MonitorInput
  | NotebookEditInput
  | ProjectsInput
  | PushNotificationInput
  | ReadMcpResourceDirInput
  | ReadMcpResourceInput
  | RefreshMcpToolsInput
  | RemoteTriggerInput
  | ReportFindingsInput
  | ScheduleWakeupInput
  | ShowOnboardingRolePickerInput
  | TaskCreateInput
  | TaskGetInput
  | TaskListInput
  | TaskStopInput
  | TaskUpdateInput
  | TodoWriteInput
  | WebFetchInput
  | WebSearchInput
  | WorkflowInput;
```

<h3 id="agent">
  Agent
</h3>

**Nama tool:** `Agent`. Nama sebelumnya `Task` masih diterima sebagai alias, dan array `tools` dalam pesan init [`SDKSystemMessage`](#sdksystemmessage) saat ini mencantumkan tool ini sebagai `Task` untuk kompatibilitas mundur.

<Note>
  Bidang `mode` sudah usang dan diabaikan pada Claude Code v2.1.212 atau lebih baru. Subagent berjalan dalam mode izin sesi induk atau [`permissionMode`](#agentdefinition) definisinya, dan [aturan pewarisan subagent](/docs/id/agent-sdk/permissions#available-modes) memutuskan yang mana.
</Note>

```typescript theme={null}
type AgentInput = {
  description: string;
  prompt: string;
  subagent_type?: string;
  model?: "sonnet" | "opus" | "haiku" | "fable";
  run_in_background?: boolean;
  name?: string;
  team_name?: string; // Deprecated; ignored
  mode?: "acceptEdits" | "auto" | "bypassPermissions" | "default" | "dontAsk" | "plan"; // Deprecated; ignored. The subagent inheritance rules decide a subagent's permission mode
  isolation?: "worktree" | "remote";
};
```

Meluncurkan agent baru untuk menangani tugas kompleks multi-langkah secara otonom.

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
  answers?: Record<string, string>;
  annotations?: Record<string, { preview?: string; notes?: string }>;
  metadata?: { source?: string };
};
```

Menanyakan pertanyaan klarifikasi kepada pengguna selama eksekusi. Lihat [Menangani persetujuan dan input pengguna](/docs/id/agent-sdk/user-input#handle-clarifying-questions) untuk detail penggunaan.

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

Mengeksekusi perintah Bash dengan timeout opsional dan eksekusi latar belakang. Direktori kerja tetap ada di antara perintah, termasuk perintah yang dijalankan di putaran berikutnya dari sesi multi-putaran; status shell seperti variabel lingkungan yang diekspor tidak. Untuk batas pada perubahan direktori mana yang bertahan, lihat [Apa yang bertahan di antara perintah](/docs/id/tools-reference#what-persists-between-commands).

<h3 id="monitor">
  Monitor
</h3>

**Nama tool:** `Monitor`

```typescript theme={null}
type MonitorInput = {
  description: string;
  timeout_ms: number;
  command?: string;
  ws?: {
    url: string;
    protocols?: string[];
  };
};
```

Menjalankan sumber latar belakang dan mengirimkan setiap peristiwa ke Claude sehingga dapat bereaksi tanpa polling: `command` menjalankan skrip dan memancarkan satu peristiwa per baris stdout, dan `ws` membuka WebSocket dan memancarkan satu peristiwa per frame teks. Berikan tepat satu dari `command` atau `ws`. Sumber `ws` memerlukan Claude Code v2.1.195 atau lebih baru.

`timeout_ms` adalah batas waktu pengawasan dalam milidetik. Ini default ke 300000 dan menerima nilai hingga 3600000. Batas waktu efektif paling banyak 1800000, yaitu 30 menit, jadi nilai yang diterima lebih besar diperpendek ke itu. Pada batas waktu pengawasan berakhir dan Claude menerima satu pemberitahuan sehingga dapat memulai pengawasan baru jika masih membutuhkannya.

Jenis yang diekspor menandai `timeout_ms` sebagai wajib karena skema mengisi default; panggilan yang menghilangkannya memvalidasi.

Ketika Monitor menjalankan perintah, ia mengikuti aturan izin yang sama dengan Bash; pengawasan WebSocket meminta persetujuan secara terpisah. Lihat [referensi tool Monitor](/docs/id/tools-reference#monitor-tool) untuk perilaku dan ketersediaan penyedia.

<h3 id="taskoutput">
  TaskOutput
</h3>

Dihapus di Claude Code v2.1.277, bersama dengan jenis `TaskOutputInput` nya. Sebelumnya mengambil output dari tugas latar belakang yang sedang berjalan atau selesai; Claude membaca file output tugas latar belakang dengan `Read` sebagai gantinya.

Entri `disallowedTools` atau aturan deny yang masih menamai `TaskOutput` diabaikan tanpa peringatan.

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

Membaca file dari sistem file lokal, termasuk teks, gambar, PDF, dan notebook Jupyter. Gunakan `pages` untuk rentang halaman PDF (misalnya, `"1-5"`).

Untuk PDF, Claude menerima konten file di dalam `tool_result` panggilan Read. Pembacaan yang mengembalikan output `pdf` [](#tool-output-types) membawa blok `text` ringkasan diikuti oleh blok `document`. Satu yang mengembalikan output `parts` membawa blok `text` ringkasan diikuti oleh satu blok per halaman yang diekstrak: blok `image`, atau blok `text` yang menamai halaman ketika Claude Code tidak dapat merender sebagai gambar. Sebelum Agent SDK v0.3.242, Claude Code mengirimkan konten file sebagai pesan `user` terpisah setelah hasil tool.

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

Menulis file ke sistem file lokal, menimpa jika ada.

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

Pencocokan pola file cepat yang bekerja dengan ukuran basis kode apa pun.

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
  "-o"?: boolean; // print only the matched parts of each line; requires output_mode: "content"
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

Tool pencarian canggih yang dibangun di atas ripgrep dengan dukungan regex.

<h3 id="taskstop">
  TaskStop
</h3>

**Nama tool:** `TaskStop`

```typescript theme={null}
type TaskStopInput = {
  task_id?: string;
  shell_id?: string; // Deprecated: use task_id
};
```

Menghentikan tugas latar belakang atau shell yang sedang berjalan berdasarkan ID. Mulai dari v2.1.198, `task_id` juga menerima rekan tim tim agent atau agent latar belakang bernama berdasarkan ID atau nama agent.

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
  args?: unknown; // any JSON value; the published typings render this as an object map
  resumeFromRunId?: string;
  title?: string; // ignored; the script's meta block sets the title
  description?: string; // ignored; the script's meta block sets the description
};
```

Menjalankan [workflow dinamis](/docs/id/workflows): skrip yang mengorkestra banyak subagent di latar belakang dan mengembalikan satu hasil terpadu. Tool Workflow tersedia di Agent SDK v0.3.149 dan lebih baru. Setidaknya satu dari `script`, `name`, atau `scriptPath` diperlukan.

| Bidang            | Jenis     | Deskripsi                                                                                                                                                                                                                                                                                                                               |
| ----------------- | --------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `script`          | `string`  | Skrip workflow inline. Harus dimulai dengan `export const meta = { name, description }` sebagai literal, diikuti oleh badan skrip menggunakan `agent()`, `parallel()`, `pipeline()`, dan `phase()`. Array `phases` opsional dalam `meta` mengelompokkan agent di bawah tahap bernama dalam tampilan kemajuan                            |
| `name`            | `string`  | Nama workflow bawaan atau yang disimpan di `.claude/workflows/`. Diselesaikan ke skrip                                                                                                                                                                                                                                                  |
| `scriptPath`      | `string`  | Jalur ke file skrip workflow di disk. Mengambil prioritas atas `script` dan `name`. Claude Code tetap menyimpan skrip setiap invokasi dan mengembalikan jalur dalam hasil, sehingga Anda dapat mengedit file itu dan menginvokasi kembali dengan `scriptPath` yang sama untuk melakukan iterasi                                         |
| `args`            | `unknown` | Nilai input yang diekspos ke skrip sebagai `args` global, untuk workflow bernama yang diparameterisasi seperti pertanyaan penelitian atau daftar jalur file. Lewatkan array dan objek sebagai nilai JSON aktual, bukan sebagai string yang dikodekan JSON                                                                               |
| `resumeFromRunId` | `string`  | ID Jalankan dari invokasi `Workflow` sebelumnya untuk dilanjutkan. Panggilan `agent()` yang selesai dengan input tidak berubah biasanya mengembalikan hasil cache; sisanya berjalan langsung. [Lanjutkan setelah jeda](/docs/id/workflows#resume-after-a-pause) mencakup panggilan selesai mana yang dijalankan kembali. Sesi yang sama saja |
| `title`           | `string`  | Diabaikan; blok `meta` skrip menetapkan judul                                                                                                                                                                                                                                                                                           |
| `description`     | `string`  | Diabaikan; blok `meta` skrip menetapkan deskripsi                                                                                                                                                                                                                                                                                       |

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
  The following tools are available by default only on Claude 3.x models, Opus 4 through 4.7, Sonnet 4 through 4.6, and Haiku 4.5. On every other model, including model IDs Claude Code doesn't recognize, they aren't available unless you opt in:

  * `TodoWrite`
  * `TaskCreate`
  * `TaskGet`
  * `TaskUpdate`
  * `TaskList`

  Wherever the tools are available, Claude Code provides the four Task tools, or `TodoWrite` instead when you set `CLAUDE_CODE_ENABLE_TASKS=0`.

  This default set applies in Claude Code v2.1.268 and later, which the TypeScript Agent SDK bundles from v0.3.268.

  Lihat [Ketersediaan model](/docs/id/agent-sdk/todo-tracking#model-availability) untuk memilih.
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

Mengembalikan snapshot semua tugas dalam daftar saat ini.

<h3 id="exitplanmode">
  ExitPlanMode
</h3>

**Nama tool:** `ExitPlanMode`

```typescript theme={null}
type ExitPlanModeInput = {
  /** Deprecated: no longer used. */
  allowedPrompts?: Array<{
    tool: "Bash";
    prompt: string;
  }>;
  [k: string]: unknown;
};
```

Keluar dari mode rencana. Bidang `allowedPrompts` sudah usang dan diabaikan; Claude Code masih menerimanya sehingga pemanggil dan transkrip yang ada memvalidasi. Sebelum v2.1.205, ia meminta izin Bash berbasis prompt untuk mengimplementasikan rencana.

<h3 id="listmcpresources">
  ListMcpResources
</h3>

**Nama tool:** `ListMcpResourcesTool`

```typescript theme={null}
type ListMcpResourcesInput = {
  server?: string;
};
```

Mencantumkan sumber daya MCP yang tersedia dari server yang terhubung.

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

Membuat dan memasuki worktree git sementara untuk pekerjaan terisolasi. Lewatkan `path` untuk beralih ke worktree yang ada alih-alih membuat yang baru. Pada entri pertama target harus berupa worktree terdaftar dari repositori saat ini atau, di ruang kerja multi-repo, dari repositori yang bersarang di dalamnya; dari dalam sesi worktree harus berada di bawah `.claude/worktrees/` dari repositori sesi. `name` dan `path` saling eksklusif.

<h3 id="exitworktree">
  ExitWorktree
</h3>

**Nama tool:** `ExitWorktree`

```typescript theme={null}
type ExitWorktreeInput = {
  action: "keep" | "remove";
  discard_changes?: boolean;
};
```

Keluar dari worktree git saat ini dan kembali ke direktori kerja asli. Tindakan `keep` meninggalkan worktree dan cabang di disk, sementara `remove` menghapus keduanya. `discard_changes` harus `true` ketika menghapus worktree yang memiliki file yang tidak dikomit atau komit yang tidak digabung.

<h3 id="enterplanmode">
  EnterPlanMode
</h3>

**Nama tool:** `EnterPlanMode`

```typescript theme={null}
type EnterPlanModeInput = {};
```

Memasuki mode rencana, di mana Claude meneliti dan menyajikan rencana sebelum membuat perubahan.

<h3 id="croncreate">
  CronCreate
</h3>

**Nama tool:** `CronCreate`

```typescript theme={null}
type CronCreateInput = {
  cron: string;
  prompt: string;
  recurring?: boolean;
  durable?: boolean;
};
```

Menjadwalkan prompt untuk dijalankan pada jadwal cron 5-bidang dalam waktu lokal. Atur `recurring` ke `false` untuk menembak sekali pada kecocokan berikutnya. Pekerjaan bersifat sesi-scoped secara default: memulai percakapan segar menghapusnya, dan melanjutkan dengan `--resume` atau `--continue` memulihkan pekerjaan yang belum kedaluwarsa. Lihat [Tugas terjadwal](/docs/id/scheduled-tasks).

Mengatur `durable` ke `true` meminta persistensi ke `.claude/scheduled_tasks.json` sehingga pekerjaan bertahan dari restart. Penjadwalan tahan lama tidak tersedia di setiap sesi: ketika tidak, Claude Code menerima `durable: true` tetapi membuat pekerjaan hanya sesi. Baca bidang `durable` output untuk melihat apakah pekerjaan bertahan.

<h3 id="crondelete">
  CronDelete
</h3>

**Nama tool:** `CronDelete`

```typescript theme={null}
type CronDeleteInput = {
  id: string;
};
```

Menghapus pekerjaan cron terjadwal berdasarkan ID yang dikembalikan dari `CronCreate`.

<h3 id="cronlist">
  CronList
</h3>

**Nama tool:** `CronList`

```typescript theme={null}
type CronListInput = {};
```

Mencantumkan pekerjaan cron terjadwal: pekerjaan tahan lama dari `.claude/scheduled_tasks.json` dan pekerjaan hanya sesi dari sesi saat ini.

<h3 id="schedulewakeup">
  ScheduleWakeup
</h3>

**Nama tool:** `ScheduleWakeup`

```typescript theme={null}
type ScheduleWakeupInput = {
  delaySeconds?: number;
  reason?: string;
  prompt?: string;
  noop?: boolean;
  stop?: boolean;
};
```

Menjadwalkan bangun sekali yang menembak prompt yang diberikan setelah penundaan. Tool ini mendukung perintah `/loop` yang berjalan sendiri. Runtime menjepit `delaySeconds` antara 60 dan 3600 detik. Bidang `delaySeconds`, `reason`, `prompt`, dan `noop` diperlukan kecuali `stop` adalah true. `noop: true` melaporkan bangun di mana tidak ada yang berubah. Mengatur `stop: true` membatalkan bangun yang tertunda dan mengakhiri `/loop` yang berjalan sendiri. Bidang `stop` memerlukan Claude Code v2.1.202 atau lebih baru. Lihat [baris ScheduleWakeup dalam referensi tool](/docs/id/tools-reference).

<h3 id="remotetrigger">
  RemoteTrigger
</h3>

**Nama tool:** `RemoteTrigger`

```typescript theme={null}
type RemoteTriggerInput = {
  action:
    | "list"
    | "get"
    | "create"
    | "update"
    | "run"
    | "create_webhook_trigger"
    | "list_runs"
    | "get_run_log";
  trigger_id?: string;
  session_id?: string;
  cursor?: string;
  body?: {
    [k: string]: unknown;
  };
};
```

Mengelola [Rutinitas](/docs/id/routines), jalankan Claude Code terjadwal dan dipicu yang dihosting di cloud. Tool ini mendukung perintah `/schedule`. `trigger_id` diperlukan untuk tindakan `get`, `update`, `run`, dan `list_runs`. `body` diperlukan untuk `create`, `update`, dan `create_webhook_trigger`, dan opsional untuk `run`.

`create_webhook_trigger` melampirkan sumber peristiwa ke rutinitas yang ada, seperti [peristiwa GitHub](/docs/id/routines#add-a-github-trigger) yang menembaknya. `body` menamai sumber, peristiwa, dan rutinitas untuk menembak. Memerlukan Claude Code v2.1.225 atau lebih baru.

`list_runs` mencantumkan jalankan rutinitas baru-baru ini, dan `get_run_log` membaca log satu jalankan. `session_id` menamai jalankan untuk dibaca, dari hasil `list_runs`, dan `cursor` halaman melalui tindakan mana pun hasil. Kedua tindakan memerlukan Claude Code v2.1.227 atau lebih baru.

Tool ini hanya tersedia ketika sesi diautentikasi dengan akun claude.ai pada paket dengan Rutinitas diaktifkan, dan tidak ada ketika kebijakan organisasi Anda menonaktifkan [Claude Code di web](/docs/id/claude-code-on-the-web). Pada Claude Code v2.1.227 atau lebih baru, tool juga tidak ada ketika Pemilik telah [mematikan rutinitas untuk organisasi](/docs/id/routines#routines-are-disabled-by-your-organizations-policy). Sebelum v2.1.227, sesi dengan hanya toggle rutinitas yang dimatikan masih menunjukkan tool, dan server menolak panggilannya.

<h3 id="pushnotification">
  PushNotification
</h3>

**Nama tool:** `PushNotification`

```typescript theme={null}
type PushNotificationInput = {
  message: string;
  status: "proactive";
};
```

Mengirim notifikasi push proaktif kepada pengguna. Jaga `message` di bawah 200 karakter karena sistem operasi mobile memotong teks yang lebih panjang. Lihat [baris PushNotification dalam referensi tool](/docs/id/tools-reference) untuk ketersediaan penyedia; pengiriman push berjalan melalui infrastruktur yang dihosting Anthropic yang tidak dapat diakses dari Amazon Bedrock, Claude Platform di AWS, Platform Agent Google Cloud, atau Microsoft Foundry.

<h3 id="repl">
  REPL
</h3>

Dihapus di v2.1.275. Melalui v2.1.274, tool `REPL` eksperimental dapat dihidupkan dengan `CLAUDE_CODE_REPL=1` dalam [opsi `env`](#options).

<h3 id="reportfindings">
  ReportFindings
</h3>

**Nama tool:** `ReportFindings`

```typescript theme={null}
type ReportFindingsInput = {
  level?: "low" | "medium" | "high" | "xhigh" | "max";
  findings: Array<{
    file: string;
    line?: number;
    summary: string;
    failure_scenario: string;
    short_summary?: string;
    category?: string;
    verdict?: "CONFIRMED" | "PLAUSIBLE";
    outcome?: "fixed" | "skipped" | "no_change_needed";
  }>;
};
```

Melaporkan temuan tinjauan kode sebagai daftar terstruktur sehingga Claude Code dapat merender alih-alih mencetaknya sebagai teks. `level` adalah tingkat upaya yang dijalankan tinjauan. Temuan diurutkan paling parah terlebih dahulu, dengan paling banyak 32 per panggilan, dan array kosong ketika tidak ada yang bertahan. Memerlukan Claude Code v2.1.196 atau lebih baru.

Setiap temuan membawa bidang-bidang ini:

* `file`: jalur relatif repo temuan berada. `line` opsional adalah baris 1-indexed yang ditambatkan.
* `summary`: pernyataan satu kalimat dari cacat. `failure_scenario` menjelaskan input konkret dan status yang mengarah ke output yang salah atau crash.
* `short_summary`: label terkompresi opsional paling banyak 60 karakter untuk tampilan kompak. Memerlukan Claude Code v2.1.212 atau lebih baru.
* `category`: slug kebab-case pendek opsional dari jenis temuan, seperti `correctness` atau `test-coverage`. Memerlukan Claude Code v2.1.199 atau lebih baru.
* `verdict`: ditetapkan ketika lulus verifikasi berjalan; tidak ada pada tinjauan inline-only.
* `outcome`: ditetapkan hanya ketika melaporkan kembali setelah menerapkan perbaikan.

<h3 id="artifact">
  Artifact
</h3>

**Nama tool:** `Artifact`

```typescript theme={null}
type ArtifactInput = {
  action?: "publish" | "list";
  file_path?: string;
  favicon?: string;
  icon?: string;
  limit?: number;
  scope?: "mine" | "shared" | "all";
  title?: string;
  description?: string;
  label?: string;
  url?: string;
  force?: boolean;
  capabilities?: Record<string, unknown>;
  contract?: "latest" | string;
};
```

Menerbitkan file `.html` atau `.md` lokal sebagai halaman artefak yang dihosting, atau mencantumkan artefak yang diterbitkan pengguna. Hilangkan `action` atau lewatkan `"publish"` untuk menerbitkan `file_path`, yang diperlukan untuk tindakan publikasi. Setiap bidang di bawah berlaku untuk publikasi:

* `icon`: satu kata generik pendek untuk ikon tab browser artefak, seperti `chart` atau `map`. Claude menyertakannya pada publikasi pertama dan menghilangkannya pada pembaruan, yang menjaga ikon artefak yang disimpan.
* `favicon`: sudah usang, dan Claude menghilangkannya.
* `title`: menamai halaman yang diterbitkan di tab browser dan galeri ketika file HTML tidak memiliki tag `<title>`.
* `url`: menargetkan artefak yang ada untuk diperbarui di tempat alih-alih membuat yang baru.

`force` adalah penimpa upaya terakhir yang membuang versi yang lebih baru sesi lain diterbitkan. Pada konflik, publikasi yang gagal mengembalikan konten yang lebih baru; Claude menggabungkan perubahannya ke konten itu, atau membaca ulang artefak, dan menerbitkan lagi. Lewatkan `force` hanya ketika pengguna secara eksplisit meminta untuk membuang versi itu.

Lewatkan `"list"` untuk menghitung artefak yang diterbitkan pengguna; hanya `limit` dan `scope` yang dapat menemaninya. `scope` default ke `"mine"`, yang mencantumkan artefak yang dimiliki pengguna; `"shared"` mencantumkan artefak yang dibagikan orang lain kepada pengguna, dan `"all"` mencantumkan keduanya.

* `capabilities`: kemampuan runtime yang digunakan halaman yang diterbitkan, dikunci berdasarkan nama kemampuan, seperti [konektor yang dapat dipanggil halaman](/docs/id/artifacts#pull-live-data-with-mcp-connectors). Layanan artefak memvalidasi deklarasi dan menolak publikasi yang menamai kemampuan yang tidak dapat digunakan akun atau memberikan satu konfigurasi yang tidak valid. Lewatkan `{}` untuk menghapus deklarasi yang disimpan, dan hilangkan bidang pada redeploy untuk menyimpannya. Memerlukan Agent SDK v0.3.235 atau lebih baru.
* `contract`: versi runtime yang dijalankan halaman yang diterbitkan. Hilangkan untuk menyimpan versi artefak saat ini, lewatkan `"latest"` untuk upgrade, atau lewatkan versi tertentu untuk pin atau rollback. Memerlukan Agent SDK v0.3.235 atau lebih baru.

Jenis diekspor, tetapi tool dimatikan secara default dalam sesi Agent SDK. Publikasi juga memerlukan setiap kondisi dalam [tabel ketersediaan artefak](/docs/id/artifacts#availability), yang sesi yang diautentikasi dengan kunci API tidak memenuhi.

<h3 id="projects">
  Projects
</h3>

**Nama tool:** `Projects`

```typescript theme={null}
type ProjectsInput = {
  method:
    | "project_info"
    | "project_read"
    | "project_search"
    | "project_write"
    | "project_delete";
  path?: string;
  content?: string;
  local_path?: string;
  present_to_user?: boolean;
  query?: string;
  n?: number;
};
```

Membaca dan menulis Proyek claude.ai yang dilampirkan ke sesi. Mengirim pada `method`:

* `project_info`: mengembalikan metadata proyek dan daftar doc.
* `project_read`: membaca satu doc berdasarkan `path`.
* `project_search`: menanyakan basis pengetahuan proyek dengan `query`. `n` membatasi hit dan default ke `5`.
* `project_write`: membuat atau mengganti doc di `path` dari tepat satu dari `content`, yang membawa teks inline, atau `local_path`, yang menamai file di dalam direktori kerja. `present_to_user: true` menandai doc yang ditulis sebagai deliverable yang perlu dilihat pengguna.
* `project_delete`: menghapus doc berdasarkan `path`.

<h3 id="readmcpresourcedir">
  ReadMcpResourceDir
</h3>

**Nama tool:** `ReadMcpResourceDirTool`

```typescript theme={null}
type ReadMcpResourceDirInput = {
  server: string;
  uri: string;
};
```

Mencantumkan anak langsung dari sumber daya direktori pada server MCP. Hanya dapat digunakan terhadap server yang telah mendeklarasikan dukungan untuk daftar direktori; daftar tidak rekursif. Daftar direktori tidak diaktifkan di setiap sesi: ketika dimatikan, panggilan mengembalikan daftar `resources` kosong dan bidang `error` melaporkan bahwa daftar direktori tidak diaktifkan.

<h3 id="refreshmcptools">
  RefreshMcpTools
</h3>

**Nama tool:** `RefreshMcpTools`

```typescript theme={null}
type RefreshMcpToolsInput = {
  server?: string; // refresh only this server; omit to refresh all connected servers
};
```

Menanyakan ulang daftar tool server MCP yang terhubung dan menerapkan perubahan apa pun. Jenis diekspor, tetapi Claude Code mendaftarkan tool hanya ketika Anda mengatur `CLAUDE_CODE_ENABLE_REFRESH_MCP_TOOLS=1` dalam [opsi `env`](#options), dan hanya dalam sesi dengan setidaknya satu server MCP. Memerlukan Claude Code v2.1.211 atau lebih baru.

<h3 id="showonboardingrolepicker">
  ShowOnboardingRolePicker
</h3>

**Nama tool:** `ShowOnboardingRolePicker`

```typescript theme={null}
type ShowOnboardingRolePickerInput = {};
```

Merender baris chip pemilih peran yang dapat diklik selama onboarding Cowork sehingga pengguna dapat memilih peran mereka dan mendapatkan plugin yang cocok diinstal. Tidak memerlukan argumen; daftar peran ditentukan oleh klien. Panggilan memblokir sampai pengguna merespons.

<h3 id="mcpinput">
  McpInput
</h3>

**Nama tool:** nama tool MCP dinamis dari bentuk `mcp__<server>__<tool>`

```typescript theme={null}
type McpInput = {
  [k: string]: unknown;
};
```

Argumen tool MCP adalah objek terbuka: setiap server mendefinisikan parameternya sendiri, jadi jenis tidak menempatkan batasan pada nama bidang atau nilai. Konsultasikan skema tool server sendiri untuk bidang yang diterima tool tertentu.

<h2 id="tool-output-types">
  Tipe Output Tool
</h2>

Dokumentasi skema output untuk semua tool Claude Code bawaan. Tipe ini dieksport dari `@anthropic-ai/claude-agent-sdk` dan mewakili data respons aktual yang dikembalikan oleh setiap tool.

<h3 id="tooloutputschemas">
  `ToolOutputSchemas`
</h3>

Union dari tipe output tool yang dieksport dari `@anthropic-ai/claude-agent-sdk`; anggota mencakup:

```typescript theme={null}
type ToolOutputSchemas =
  | AgentOutput
  | ArtifactOutput
  | AskUserQuestionOutput
  | BashOutput
  | CronCreateOutput
  | CronDeleteOutput
  | CronListOutput
  | EnterPlanModeOutput
  | EnterWorktreeOutput
  | ExitPlanModeOutput
  | ExitWorktreeOutput
  | FileEditOutput
  | FileReadOutput
  | FileWriteOutput
  | GlobOutput
  | GrepOutput
  | ListMcpResourcesOutput
  | McpOutput
  | MonitorOutput
  | NotebookEditOutput
  | ProjectsOutput
  | PushNotificationOutput
  | ReadMcpResourceDirOutput
  | ReadMcpResourceOutput
  | RefreshMcpToolsOutput
  | RemoteTriggerOutput
  | ReportFindingsOutput
  | ScheduleWakeupOutput
  | ShowOnboardingRolePickerOutput
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

<h3 id="agent-2">
  Agent
</h3>

**Nama tool:** `Agent`. Nama sebelumnya `Task` masih diterima sebagai alias, dan array `tools` dalam pesan init [`SDKSystemMessage`](#sdksystemmessage) saat ini mencantumkan tool ini sebagai `Task` untuk kompatibilitas mundur.

```typescript theme={null}
type AgentOutput =
  | {
      status: "completed";
      agentId: string;
      agentType?: string;
      content: Array<{ type: "text"; text: string; citations?: unknown[] | null }>;
      resolvedModel?: string;
      modelsUsed?: string[];
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
        output_tokens_details?: {
          thinking_tokens?: number | null;
        } | null;
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
      modelsUsed?: string[];
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

Pada varian `completed`, `resolvedModel` menamai model yang dimulai oleh subagen, yang dapat berbeda dari input `model` yang diminta ketika [`availableModels`](/docs/id/model-config#restrict-model-selection) atau override lainnya berlaku. Field ini memerlukan Claude Code v2.1.174 atau lebih baru. Pada `async_launched`, field ini menamai model yang digunakan ketika tugas berpindah ke latar belakang.

`modelsUsed` mencantumkan model yang digunakan subagen, secara berurutan. Field ini hadir hanya ketika pertukaran tengah-run terjadi, dan model muncul lagi ketika run bertukar kembali ke model tersebut. Pada `async_launched`, daftar mencakup model yang digunakan sebelum backgrounding. Baik `modelsUsed` maupun perilaku backgrounding dari `resolvedModel` memerlukan Claude Code v2.1.212 atau lebih baru.

Jika Claude Code [menyimpan worktree terisolasi subagen](/docs/id/worktrees#isolate-subagents-with-worktrees), `worktreePath` pada hasil `completed` adalah tempat menemukannya. `worktreeBranch` adalah cabangnya, hadir ketika Claude Code membuat worktree dengan git.

Claude Code mengisi `usage` dan `totalTokens` dari permintaan API final subagen, bukan dari seluruh run, jadi `usage.service_tier` adalah string tier layanan yang dilaporkan API pada permintaan tersebut. Ketika ada, `usage.output_tokens_details.thinking_tokens` adalah jumlah token output permintaan tersebut yang merupakan token pemikiran. Field `output_tokens_details` memerlukan TypeScript SDK v0.3.228 atau lebih baru, yang menggabungkan Claude Code v2.1.228.

`usage.output_tokens_details` cocok dengan [`Usage.output_tokens_details`](#usage) dalam arti, dibatasi pada permintaan final tersebut, tetapi setiap levelnya bersifat opsional di sini. Lindungi baik objek maupun field, misalnya `usage.output_tokens_details?.thinking_tokens ?? 0`, daripada membacanya secara langsung.

Sebelum v2.1.207, tipe yang dipublikasikan lebih sempit. Tipe tersebut menghilangkan `worktreePath`, `worktreeBranch`, `citations`, `toolStats.frameCount`, dan field penggunaan `inference_geo`, `speed`, dan `iterations`, dan mengetik `service_tier` sebagai `"standard" | "priority" | "batch"`. Field yang ditandai tipe sebagai opsional dapat tidak ada pada hasil yang dicatat oleh versi sebelumnya.

<h3 id="askuserquestion-2">
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
  annotations?: Record<string, { preview?: string; notes?: string }>;
  afkTimeoutMs?: number;
};
```

Mengembalikan pertanyaan yang diajukan dan jawaban pengguna. `response` diatur ketika pengguna mengetik balasan bentuk bebas alih-alih menjawab pertanyaan terstruktur; ketika ada, Claude menerima "Pengguna merespons: …" alih-alih daftar jawaban per-pertanyaan.

<h3 id="bash-2">
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
  timedOutAfterMs?: number;
  backgroundCwdHint?: string;
  backgroundEndsWithFinalResponse?: true;
  dangerouslyDisableSandbox?: boolean;
  returnCodeInterpretation?: string;
  noOutputExpected?: boolean;
  structuredContent?: unknown[];
  persistedOutputPath?: string;
  persistedOutputSize?: number;
  staleReadFileStateHint?: string;
  ghRateLimitHint?: string;
  gitOperation?: {
    commit?: { sha: string; kind: "committed" | "amended" | "cherry-picked"; branch?: string };
    push?: { branch: string };
    branch?: { ref: string; action: "merged" | "rebased" };
    pr?: {
      number: number;
      url?: string;
      action: "created" | "edited" | "merged" | "commented" | "closed" | "reopened" | "ready" | "draft" | "auto-merge-enabled" | "auto-merge-disabled";
    };
  };
};
```

Field `stdout`, `stderr`, dan `backgroundTaskId` membawa:

| Field              | Apa yang dibawanya                                                                                                     |
| ------------------ | ---------------------------------------------------------------------------------------------------------------------- |
| `stdout`           | Stdout dan stderr perintah, digabungkan menjadi satu aliran yang saling terkait                                        |
| `stderr`           | Pemberitahuan yang ditambahkan tool itu sendiri, seperti pengaturan ulang direktori kerja shell, bukan stderr perintah |
| `backgroundTaskId` | Hadir untuk perintah latar belakang                                                                                    |

`timedOutAfterMs` adalah timeout dalam milidetik, diatur ketika perintah mencapai timeout dan berpindah ke latar belakang daripada dimulai di sana secara eksplisit. `backgroundCwdHint` diatur ketika perintah backgrounded berisi builtin perubahan direktori seperti `cd`, `pushd`, `popd`, atau `chdir`, dan mencatat bahwa direktori kerja sesi tidak berubah. Kedua field memerlukan Claude Code v2.1.210 atau lebih baru.

Ketika subagen yang berjalan di foreground memiliki perintah backgrounded, Claude Code menghentikan perintah ketika subagen tersebut memberikan respons finalnya. Claude Code menetapkan `backgroundEndsWithFinalResponse` ke `true` pada perintah tersebut, dan menghilangkan field ketika perintah bertahan dalam turn, seperti perintah yang dimulai oleh percakapan utama atau oleh subagen latar belakang. Field memerlukan Claude Code v2.1.227 atau lebih baru.

Claude Code menetapkan `gitOperation.commit.branch` ke cabang yang dinamai dalam baris ringkasan commit git, dan menghilangkannya untuk commit yang dibuat pada HEAD terpisah. Field memerlukan Agent SDK v0.3.227 atau lebih baru. Claude Code melaporkan perintah `gh pr reopen` sebagai aksi PR `reopened`, yang memerlukan Agent SDK v0.3.234 atau lebih baru.

<h3 id="monitor-2">
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

<h3 id="edit-2">
  Edit
</h3>

**Nama tool:** `Edit`

```typescript theme={null}
type FileEditOutput = {
  filePath: string;
  oldString: string;
  newString: string;
  originalFile: string | null;
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
    repository?: string | null;
  };
};
```

Mengembalikan diff terstruktur dari operasi edit.

<h3 id="read-2">
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
        /** True ketika pembacaan seluruh file secara otomatis dipaginasi karena melebihi batas token (konten adalah halaman pertama parsial). */
        truncatedByTokenCap?: boolean;
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
      /** Nomor halaman dokumen dari halaman yang diekstrak pertama; memberi label pada gambar halaman dalam konten tool_result. */
      firstPage?: number;
      /** Hanya dalam proses: byte gambar halaman disampaikan sebagai blok gambar dalam konten tool_result dan tidak disimpan pada tool_use_result yang dipancarkan, jadi kunci ini tidak ada di sana. */
      pages?: {
        base64: string;
        mediaType: "image/jpeg" | "image/png" | "image/gif" | "image/webp";
        error?: string;
      }[];
    }
  | {
      type: "file_unchanged";
      file: {
        filePath: string;
      };
      /** Diatur ketika dedup cocok dengan entri yang ditanam startup (CLAUDE.md / memori bersarang) daripada hasil tool_result Read sebelumnya. */
      source?: "seeded";
    };
```

Mengembalikan konten file dalam format yang sesuai dengan tipe file. Didiskriminasikan pada field `type`.

<h3 id="write-2">
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
    repository?: string | null;
  };
  userModified?: boolean;
};
```

Mengembalikan hasil write dengan informasi diff terstruktur. Apa yang dibawa `originalFile` dan `structuredPatch` tergantung pada write:

* Untuk file yang baru dibuat, `originalFile` adalah null dan `structuredPatch` kosong
* Pada overwrite, `originalFile` membawa konten sebelumnya, kecuali ketika konten tersebut lebih besar dari sekitar 10 MB: Claude Code kemudian melewatkan diff dan mengembalikan `originalFile` null dan `structuredPatch` kosong
* `structuredPatch` juga kosong ketika write tidak mengubah apa pun atau diff timeout

<h3 id="glob-2">
  Glob
</h3>

**Nama tool:** `Glob`

```typescript theme={null}
type GlobOutput = {
  durationMs: number;
  numFiles: number;
  filenames: string[];
  truncated: boolean;
  totalMatches?: number;
  countIsComplete?: boolean;
};
```

Mengembalikan jalur file yang cocok dengan pola glob, diurutkan berdasarkan waktu modifikasi.

`totalMatches` dan `countIsComplete` memerlukan Claude Code v2.1.191 atau lebih baru. `totalMatches` melaporkan jumlah file yang cocok sebelum truncation. Ketika `countIsComplete` adalah false, `totalMatches` adalah batas bawah karena pencarian yang mendasar memotong output-nya sendiri.

<h3 id="grep-2">
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
  totalFiles?: number;
  totalLines?: number;
  appliedLimit?: number;
  appliedOffset?: number;
};
```

Mengembalikan hasil pencarian. Bentuknya bervariasi menurut `mode`: daftar file, konten dengan kecocokan, atau hitungan kecocokan. Dalam mode `count`, `numFiles` dan `numMatches` adalah total di seluruh set hasil, bukan slice yang dipaginasi. Sebelum v2.1.208, `head_limit` atau `offset` yang memotong entri yang tercantum juga memotong total tersebut.

`totalFiles` memerlukan Claude Code v2.1.208 atau lebih baru dan melaporkan jumlah total hasil sebelum paginasi `head_limit` dan `offset` dalam mode `files_with_matches`. `totalLines` memerlukan Claude Code v2.1.210 atau lebih baru dan melaporkan jumlah total baris sebelum paginasi dalam mode `content`.

<h3 id="taskstop-2">
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

<h3 id="notebookedit-2">
  NotebookEdit
</h3>

**Nama tool:** `NotebookEdit`

```typescript theme={null}
type NotebookEditOutput = {
  new_source: string;
  old_source?: string;
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

<h3 id="webfetch-2">
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
  artifactRead?: {
    slug: string;
    ver?: string;
    seeded?: false;
  };
};
```

Mengembalikan konten yang diambil dengan status HTTP dan metadata.

`artifactRead` adalah catatan Claude Code sendiri tentang pembacaan artifact, hadir hanya ketika Claude mengambil artifact yang dapat dipublikasikan sesi. Claude Code membacanya kembali ketika sesi dilanjutkan sehingga publikasi kemudian dibangun di atas versi yang tepat; kode Anda tidak perlu bertindak atas hal ini. `slug` menamai artifact, `ver` adalah versi yang dibaca catat dan tidak ada ketika tidak mencatat apa pun, dan `seeded: false` menandai pembacaan yang sumber lengkapnya tidak mencapai Claude. Field `seeded` memerlukan Agent SDK v0.3.239 atau lebih baru.

<h3 id="websearch-2">
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
  searchCount?: number;
};
```

Mengembalikan hasil pencarian dari web.

<h3 id="workflow-2">
  Workflow
</h3>

**Nama tool:** `Workflow`

```typescript theme={null}
type WorkflowOutput = {
  status: "async_launched" | "remote_launched";
  taskId: string;
  taskType?: "local_workflow" | "remote_agent";
  workflowName?: string;
  runId?: string;
  summary?: string;
  transcriptDir?: string;
  scriptPath?: string;
  sessionUrl?: string; // diatur ketika workflow diluncurkan sebagai sesi jarak jauh
  warning?: string;
  error?: string;
};
```

Mengembalikan segera setelah tool menerima invokasi. Hasil akhir tiba kemudian sebagai penyelesaian tugas. Periksa `error` sebelum memperlakukan run sebagai dimulai: skrip yang gagal pemeriksaan sintaksnya mengembalikan `status: "async_launched"` dengan `error` diatur, dan tidak pernah berjalan.

| Field           | Type                                    | Description                                                                                                                                                                 |
| --------------- | --------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `status`        | `"async_launched" \| "remote_launched"` | Tool menerima invokasi. `"async_launched"` untuk run dalam proses, `"remote_launched"` untuk run yang dikirimkan ke sesi jarak jauh alih-alih berjalan dalam proses         |
| `taskId`        | `string`                                | Pengenal tugas latar belakang untuk run                                                                                                                                     |
| `taskType`      | `"local_workflow" \| "remote_agent"`    | Tipe tugas dari tugas latar belakang yang terdaftar, cocok dengan arm `status`                                                                                              |
| `workflowName`  | `string`                                | `meta.name` dari skrip workflow                                                                                                                                             |
| `runId`         | `string`                                | Pengenal workflow run untuk diteruskan sebagai `resumeFromRunId` pada invokasi kemudian. Tidak ada untuk run `remote_launched`, di mana URL sesi cloud adalah handle resume |
| `summary`       | `string`                                | Deskripsi satu baris tentang apa yang dilakukan workflow                                                                                                                    |
| `transcriptDir` | `string`                                | Direktori tempat transkrip subagen ditulis selama eksekusi                                                                                                                  |
| `scriptPath`    | `string`                                | Jalur ke skrip workflow yang disimpan untuk run ini. Edit dan teruskan kembali sebagai `scriptPath` untuk menjalankan ulang tanpa mengirim ulang skrip                      |
| `sessionUrl`    | `string`                                | URL sesi cloud, diatur ketika `status` adalah `"remote_launched"`                                                                                                           |
| `warning`       | `string`                                | Heads-up non-blocking, seperti status git lokal menyimpang dari cabang yang didorong yang akan dikloning sesi cloud                                                         |
| `error`         | `string`                                | Diatur ketika skrip gagal pemeriksaan sintaksnya. Ketika ada, run tidak dimulai meskipun status launched                                                                    |

<h3 id="todowrite-2">
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
  The following tools are available by default only on Claude 3.x models, Opus 4 through 4.7, Sonnet 4 through 4.6, and Haiku 4.5. On every other model, including model IDs Claude Code doesn't recognize, they aren't available unless you opt in:

  * `TodoWrite`
  * `TaskCreate`
  * `TaskGet`
  * `TaskUpdate`
  * `TaskList`

  Wherever the tools are available, Claude Code provides the four Task tools, or `TodoWrite` instead when you set `CLAUDE_CODE_ENABLE_TASKS=0`.

  This default set applies in Claude Code v2.1.268 and later, which the TypeScript Agent SDK bundles from v0.3.268.

  Lihat [Ketersediaan Model](/docs/id/agent-sdk/todo-tracking#model-availability) untuk opt in.
</Note>

<h3 id="taskcreate-2">
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

<h3 id="taskupdate-2">
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

<h3 id="taskget-2">
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

<h3 id="tasklist-2">
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

<h3 id="exitplanmode-2">
  ExitPlanMode
</h3>

**Nama tool:** `ExitPlanMode`

```typescript theme={null}
type ExitPlanModeOutput = {
  plan: string | null;
  isAgent: boolean;
  filePath?: string;
  hasTaskTool?: boolean;
  planWasEdited?: boolean;
  awaitingLeaderApproval?: boolean;
  requestId?: string;
};
```

Mengembalikan status rencana setelah keluar dari mode perencanaan.

<h3 id="listmcpresources-2">
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

<h3 id="readmcpresource-2">
  ReadMcpResource
</h3>

**Nama tool:** `ReadMcpResourceTool`

```typescript theme={null}
type ReadMcpResourceOutput = {
  contents: Array<{
    uri: string;
    mimeType?: string;
    text?: string;
    blobSavedTo?: string;
  }>;
  error?: string;
};
```

Mengembalikan konten sumber daya MCP yang diminta.

<h3 id="enterworktree-2">
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

<h3 id="exitworktree-2">
  ExitWorktree
</h3>

**Nama tool:** `ExitWorktree`

```typescript theme={null}
type ExitWorktreeOutput = {
  action: "keep" | "remove";
  originalCwd: string;
  worktreePath: string;
  worktreeBranch?: string;
  tmuxSessionName?: string;
  discardedFiles?: number;
  discardedCommits?: number;
  message: string;
};
```

Mengembalikan aksi yang diambil dan detail tentang worktree yang keluar.

<h3 id="enterplanmode-2">
  EnterPlanMode
</h3>

**Nama tool:** `EnterPlanMode`

```typescript theme={null}
type EnterPlanModeOutput = {
  message: string;
};
```

Mengembalikan konfirmasi bahwa mode perencanaan telah dimasuki.

<h3 id="croncreate-2">
  CronCreate
</h3>

**Nama tool:** `CronCreate`

```typescript theme={null}
type CronCreateOutput = {
  id: string;
  humanSchedule: string;
  recurring: boolean;
  durable?: boolean; // true ketika disimpan ke .claude/scheduled_tasks.json; false ketika hanya sesi
};
```

Mengembalikan ID pekerjaan dan deskripsi jadwal yang dapat dibaca manusia.

<h3 id="crondelete-2">
  CronDelete
</h3>

**Nama tool:** `CronDelete`

```typescript theme={null}
type CronDeleteOutput = {
  id: string;
};
```

Mengembalikan ID pekerjaan yang dihapus.

<h3 id="cronlist-2">
  CronList
</h3>

**Nama tool:** `CronList`

```typescript theme={null}
type CronListOutput = {
  jobs: {
    id: string;
    cron: string;
    humanSchedule: string;
    prompt: string;
    recurring?: boolean;
    durable?: boolean;
  }[];
};
```

Mengembalikan pekerjaan cron yang dijadwalkan: pekerjaan durable dari `.claude/scheduled_tasks.json` dan pekerjaan hanya-sesi dari sesi saat ini. Pekerjaan hanya-sesi membawa `durable: false`; pekerjaan yang dibaca dari disk menghilangkan field.

<h3 id="schedulewakeup-2">
  ScheduleWakeup
</h3>

**Nama tool:** `ScheduleWakeup`

```typescript theme={null}
type ScheduleWakeupOutput = {
  scheduledFor: number;
  clampedDelaySeconds: number;
  wasClamped: boolean;
  stopped?: boolean;
  cancelledWakeups?: number;
};
```

Mengembalikan kapan wake-up akan diaktifkan sebagai timestamp epoch milidetik, penundaan yang benar-benar digunakan, dan apakah penundaan yang diminta diklem. Field `stopped` adalah `true` ketika panggilan mengakhiri loop dengan `stop: true`. Field ini memerlukan Claude Code v2.1.202 atau lebih baru. Field `cancelledWakeups` menghitung berapa banyak wake-up yang tertunda yang dibatalkan oleh panggilan `stop: true`. Nilai 0 berarti tidak ada yang tertunda, dan cron `/loop` berulang tidak dibatalkan oleh `stop: true`. Field ini memerlukan Claude Code v2.1.206 atau lebih baru.

<h3 id="remotetrigger-2">
  RemoteTrigger
</h3>

**Nama tool:** `RemoteTrigger`

```typescript theme={null}
type RemoteTriggerOutput = {
  status: number;
  json: string;
  summary?: string;
};
```

Mengembalikan status respons API dan body untuk operasi trigger.

<h3 id="pushnotification-2">
  PushNotification
</h3>

**Nama tool:** `PushNotification`

```typescript theme={null}
type PushNotificationOutput = {
  message: string;
  pushSent?: boolean;
  localSent?: boolean;
  disabledReason?: "config_off" | "user_present" | "no_transport";
  sentAt?: string;
};
```

Mengembalikan detail pengiriman, termasuk apakah notifikasi push atau lokal dikirim dan mengapa pengiriman dilewati.

<h3 id="reportfindings-2">
  ReportFindings
</h3>

**Nama tool:** `ReportFindings`

```typescript theme={null}
type ReportFindingsOutput = {
  count: number;
  level?: "low" | "medium" | "high" | "xhigh" | "max";
  findings: Array<{
    file: string;
    line?: number;
    summary: string;
    failure_scenario: string;
    short_summary?: string;
    category?: string;
    verdict?: "CONFIRMED" | "PLAUSIBLE";
    outcome?: "fixed" | "skipped" | "no_change_needed";
  }>;
};
```

Mengembalikan jumlah temuan yang dilaporkan, tingkat upaya yang dijalankan review, dan temuan yang diulang kembali untuk body hasil. Memerlukan Claude Code v2.1.196 atau lebih baru. Field `short_summary` yang diulang kembali memerlukan Claude Code v2.1.212 atau lebih baru.

<h3 id="artifact-2">
  Artifact
</h3>

**Nama tool:** `Artifact`

```typescript theme={null}
type ArtifactOutput =
  | {
      url: string;
      path: string;
      title?: string;
      version?: string;
      capabilities?: unknown;
      stored?: {
        contract: string;
        capabilities?: Record<string, unknown>;
      };
      warnings?: string[];
      contract?: string;
      updated?: boolean;
      liveSubscription?: string;
    }
  | {
      artifacts: Array<{
        title: string;
        url: string;
        updatedAt?: string;
        rel?: "mine" | "shared";
      }>;
      truncated?: boolean;
      scope?: "shared" | "all";
    };
```

Mengembalikan `url` halaman yang dipublikasikan dan `path` lokal yang dipublikasikan untuk aksi publish, dengan `updated` diatur ke true ketika publish menerapkan ulang artifact yang ada, dan `warnings` membawa nasihat apa pun pada waktu publish. Aksi list mengembalikan baris `artifacts` sebagai gantinya, dengan `truncated` diatur ketika lebih banyak artifact ada daripada batas yang diminta. Pada listing yang scopenya bukan `"mine"`, setiap baris membawa `rel` menandai apakah pengguna memiliki artifact atau dibagikan dengan mereka, dan `scope` output mencatat scope non-default mana yang menghasilkan listing; keduanya tidak ada pada listing default.

<h3 id="projects-2">
  Projects
</h3>

**Nama tool:** `Projects`

```typescript theme={null}
type ProjectsOutput =
  | {
      method: "project_info";
      notice?: string;
      name: string;
      description: string;
      instructions: string;
      docs: Array<{ path: string; created_at: string | null }>;
      files?: Array<{
        path: string;
        file_kind: string;
        created_at: string | null;
      }>;
      sync_sources?: Array<{
        type: string | null;
        config: Record<string, unknown>;
      }>;
      knowledge: {
        knowledge_size: number;
        max_knowledge_size: number;
      };
    }
  | {
      method: "project_read";
      notice?: string;
      path: string;
      file_kind?: string;
      content?: string;
      local_file?: string;
      created_at: string | null;
    }
  | {
      method: "project_search";
      notice?: string;
      rag: boolean;
      hits?: Array<{ name?: string; doc_uuid?: string; text?: string }>;
      docs?: string[];
    }
  | {
      method: "project_write";
      notice?: string;
      path: string;
      doc_uuid: string;
      replaced: boolean;
      present_to_user?: boolean;
      local_path?: string;
    }
  | {
      method: "project_delete";
      notice?: string;
      path: string;
      deleted: boolean;
    };
```

Didiskriminasikan pada field `method`, mencerminkan input. `project_read` mengembalikan doc teks kecil inline dalam `content` dan menulis doc yang lebih besar ke jalur `local_file` sebagai gantinya; `project_search` mengembalikan RAG `hits` dengan `rag: true` ketika indeks proyek tersedia dan kembali ke daftar jalur `docs` sebaliknya.

<h3 id="readmcpresourcedir-2">
  ReadMcpResourceDir
</h3>

**Nama tool:** `ReadMcpResourceDirTool`

```typescript theme={null}
type ReadMcpResourceDirOutput = {
  resources: Array<{
    uri: string;
    name: string;
    mimeType?: string;
  }>;
  error?: string;
};
```

Mengembalikan anak langsung dari sumber daya direktori. Subdirektori muncul dengan mimeType `"inode/directory"`; `error` membawa pesan yang dapat dibaca manusia ketika server tidak dapat mencantumkan direktori.

<h3 id="refreshmcptools-2">
  RefreshMcpTools
</h3>

**Nama tool:** `RefreshMcpTools`

```typescript theme={null}
type RefreshMcpToolsOutput = Array<{
  server: string;
  status: "refreshed" | "error" | "not_connected";
  toolCount?: number; // tools sekarang tersedia dari server ini
  added?: string[]; // nama tool yang refresh ini tambahkan
  removed?: string[]; // nama tool yang refresh ini hapus
  error?: string; // mengapa refresh gagal atau server tidak tersedia
}>;
```

Mengembalikan satu entri per server: `refreshed` berarti daftar tool yang di-query ulang diterapkan, `error` berarti re-query gagal dan set tool sebelumnya disimpan, dan `not_connected` berarti server tidak memiliki koneksi live untuk query.

<h3 id="showonboardingrolepicker-2">
  ShowOnboardingRolePicker
</h3>

**Nama tool:** `ShowOnboardingRolePicker`

```typescript theme={null}
type ShowOnboardingRolePickerOutput = {
  role?: string;
  dismissed?: boolean;
};
```

Mengembalikan pilihan pengguna: `role` ketika mereka memilih chip peran atau mengetik satu, dan `dismissed: true` ketika mereka menutup picker. Objek kosong berarti pengguna menyetujui panggilan tanpa memilih peran.

<h3 id="mcpoutput">
  McpOutput
</h3>

**Nama tool:** nama tool MCP dinamis dari bentuk `mcp__<server>__<tool>`

```typescript theme={null}
type McpOutput =
  | string
  | {
      type: string;
      [k: string]: unknown;
    }[]
  | {
      [k: string]: unknown;
    };
```

Hasil tool MCP dikembalikan sebagai string atau array blok konten, tergantung pada server. Cabang objek polos trailing dalam tipe yang dieksport adalah artefak pembuatan skema: SDK tidak mengembalikan objek telanjang, karena output terstruktur server diserialisasi ke string JSON sebelum dikembalikan. Pada runtime nilainya juga dapat `undefined`, meskipun tipe yang dieksport tidak memodelkan ini.

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

Tempat asal kunci API untuk permintaan sesi, dilaporkan sebagai `apiKeySource` pada pesan inisialisasi [`SDKSystemMessage`](#sdksystemmessage).

```typescript theme={null}
type ApiKeySource =
  | "ANTHROPIC_API_KEY"
  | "apiKeyHelper"
  | "/login managed key"
  | "none"
  | "user"
  | "project"
  | "org"
  | "temporary"
  | "oauth";
```

Claude Code melaporkan salah satu dari empat nilai:

| Nilai                | Kunci yang digunakan                                                                                                             |
| -------------------- | -------------------------------------------------------------------------------------------------------------------------------- |
| `ANTHROPIC_API_KEY`  | Kunci dalam variabel lingkungan `ANTHROPIC_API_KEY`                                                                              |
| `apiKeyHelper`       | Kunci yang dikembalikan oleh perintah [`apiKeyHelper`](/docs/id/settings-reference#apikeyhelper) Anda                                 |
| `/login managed key` | Kunci yang disimpan Claude Code ketika Anda masuk dengan akun [Claude Console](/docs/id/authentication#claude-console-authentication) |
| `none`               | Tidak ada kunci API. Sesi mengautentikasi dengan cara lain, seperti login claude.ai, token bearer, atau penyedia cloud           |

Agent SDK v0.3.234 dan yang lebih baru mencantumkan empat nilai ini dalam tipe. Tipe juga menyimpan `user`, `project`, `org`, `temporary`, dan `oauth` sehingga kode yang lebih lama masih dikompilasi, dan Claude Code tidak melaporkannya.

<h3 id="sdkbeta">
  `SdkBeta`
</h3>

Fitur beta yang tersedia yang dapat diaktifkan melalui opsi `betas`. Lihat [Beta headers](https://platform.claude.com/docs/en/api/beta-headers) untuk informasi lebih lanjut.

```typescript theme={null}
type SdkBeta = "context-1m-2025-08-07";
```

<Warning>
  Beta `context-1m-2025-08-07` sudah pensiun sejak 30 April 2026. Melewatkan nilai ini dengan Claude Sonnet 4.5 atau Sonnet 4 tidak berpengaruh, dan permintaan yang melebihi jendela konteks standar 200k-token mengembalikan error. Untuk menggunakan jendela konteks 1M-token, migrasikan ke [Claude Opus 5.5, Claude Opus 5, Claude Sonnet 5, Claude Sonnet 4.6, Claude Opus 4.6, Claude Opus 4.7, atau Claude Opus 4.8](https://platform.claude.com/docs/en/about-claude/models/overview), yang mencakup konteks 1M dengan harga standar tanpa header beta yang diperlukan.
</Warning>

<h3 id="slashcommand">
  `SlashCommand`
</h3>

Informasi tentang perintah yang tersedia.

```typescript theme={null}
type SlashCommand = {
  name: string;
  description: string;
  argumentHint: string;
  aliases?: string[];
  builtin?: boolean;
};
```

`builtin` adalah `true` pada baris ketika perintah adalah milik Claude Code sendiri dan mengetik `/name` menjalankannya. Tidak ada untuk perintah yang didefinisikan oleh pengguna, proyek, plugin, atau server MCP, dan untuk perintah bundel yang salah satu dari [mengganti menurut nama](/docs/id/skills#resolve-skills-that-share-a-name). Memerlukan Agent SDK v0.3.277 atau lebih baru.

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

| Field         | Tipe                  | Deskripsi                                                                                                                                                                                          |
| :------------ | :-------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`        | `string`              | Pengenal tipe agen (misalnya, `"Explore"`, `"general-purpose"`)                                                                                                                                    |
| `description` | `string`              | Deskripsi tentang kapan menggunakan agen ini                                                                                                                                                       |
| `model`       | `string \| undefined` | Model yang digunakan agen ini: alias atau ID model, atau `'inherit'` untuk model parent. Ketika `undefined`, Claude Code memilih model dalam [urutan model subagen](/docs/id/sub-agents#choose-a-model) |

<h3 id="mcpserverprovenance">
  `McpServerProvenance`
</h3>

Server MCP yang melayani tool `mcp__*`, dan tempat asal definisi server itu. Input hook [`PreToolUse`](#pretoolusehookinput), `PostToolUse`, `PostToolUseFailure`, `PermissionRequest`, dan `PermissionDenied` membawanya sebagai `mcp_server`, dan opsi [`CanUseTool`](#canusetool) membawanya sebagai `mcpServer`. Keduanya menghilangkannya untuk tool yang tidak berasal dari server MCP.

```typescript theme={null}
type McpServerProvenance = {
  name: string;
  source: string;
};
```

| Field    | Tipe     | Deskripsi                                                                                                    |
| :------- | :------- | :----------------------------------------------------------------------------------------------------------- |
| `name`   | `string` | Nama yang didaftarkan server, nilai yang sama yang dilaporkan [`mcpServerStatus()`](#query-object) untuk itu |
| `source` | `string` | Tempat asal definisi server: `sdk`, `plugin`, atau cakupan konfigurasi                                       |

`source` mengambil salah satu nilai berikut. Set terbuka, jadi perlakukan nilai yang tidak Anda kenali sebagai sumber yang dikonfigurasi, tidak pernah sebagai `sdk`:

* **`sdk`**: server dalam proses yang didaftarkan aplikasi Anda. Hanya aplikasi host SDK yang dapat mendaftarkan satu, jadi server yang dikonfigurasi tidak pernah melaporkan `sdk`, apa pun namanya.
* **`plugin`**: server yang disediakan [plugin](/docs/id/agent-sdk/plugins). `name` adalah bentuk `plugin:<plugin-name>:<server-name>` yang bersifat scoped seperti yang dijelaskan di bawah [server MCP yang disediakan plugin](/docs/id/mcp#plugin-provided-mcp-servers).
* **Cakupan konfigurasi**: `user`, `project`, `local`, `dynamic`, `managed`, `enterprise`, `claudeai`, atau `agent`. Server `.mcp.json` melaporkan `project`, dan [cakupan instalasi MCP](/docs/id/mcp#mcp-installation-scopes) mendefinisikan `local`, `project`, dan `user`. Server yang diteruskan aplikasi Anda dalam opsi [`mcpServers`](#options), selain server SDK dalam proses, melaporkan `dynamic`.

Dasarkan keputusan kepercayaan pada `source`, bukan pada `name` atau awalan nama tool `mcp__<server>__`. Untuk sumber apa pun selain `sdk`, `name` adalah teks yang tidak dipercaya: escape sebelum ditampilkan.

`McpServerProvenance` dan field yang membawanya memerlukan Agent SDK v0.3.274 atau lebih baru.

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
  source?: string;
  tools?: {
    name: string;
    description?: string;
    annotations?: {
      readOnly?: boolean;
      destructive?: boolean;
      openWorld?: boolean;
    };
    _meta?: Record<string, unknown>;
  }[];
};
```

`source` mengatakan tempat asal definisi server, dengan nilai dan aturan kepercayaan yang sama seperti `source` [`McpServerProvenance`](#mcpserverprovenance). Field memerlukan Agent SDK v0.3.274 atau lebih baru dan tidak ada pada versi sebelumnya.

`_meta` pada entri `tools` membawa anggota MCP Apps dari `_meta` tool itu, sehingga aplikasi Anda dapat menemukan sumber daya `ui://` untuk dirender dengan [`readMcpResource()`](#query-object). Claude Code melewatkan objek `ui` dan string `ui/resourceUri` datar yang sudah usang, dan menahan setiap kunci lainnya. Di dalam `ui`, `resourceUri` adalah string `ui://` dan `visibility` array `"model"` dan `"app"` ketika server menetapkannya, dan anggota lainnya melewati tidak berubah. Claude Code menghapus salah satu kunci ketika nilainya tidak terbentuk dengan baik, dan menghilangkan `_meta` dari tool yang tidak mendeklarasikan keduanya. Field ini hadir hanya ketika [`capabilities`](#sdksystemmessage) pesan inisialisasi mencakup `mcp_tool_ui_meta_v1`, dan memerlukan TypeScript Agent SDK v0.3.280 atau lebih baru.

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
  thinkingTokens?: number;
  cacheReadInputTokens: number;
  cacheCreationInputTokens: number;
  webSearchRequests: number;
  costUSD: number;
  contextWindow: number;
  maxOutputTokens: number;
  canonicalModel?: string;
  provider?: string;
  costBasis?: 'list' | 'managed' | 'unknown';
};
```

`thinkingTokens` menghitung token pemikiran yang dihasilkan model ini. `outputTokens` sudah mencakupnya, jadi jangan menambahkan keduanya. Field ini tidak ada sampai putaran berjalan pada versi Claude Code yang mencatatnya, jadi sesi yang dilanjutkan yang dimulai pada versi sebelumnya melaporkan hitungan sebagian. `thinkingTokens` memerlukan Agent SDK v0.3.257 atau lebih baru.

Field `canonicalModel` dan `provider` memerlukan Claude Code v2.1.218 atau lebih baru. `canonicalModel` adalah ID model kanonik yang digunakan pencarian harga; dapat berbeda dari string model mentah yang menjadi kunci entri, misalnya ketika string itu adalah ID khusus penyedia atau alias.

`provider` menamai backend API yang melayani model, seperti `firstParty`, `bedrock`, `vertex`, `foundry`, `anthropicAws`, `mantle`, atau `gateway`.

`costBasis` menamai tabel harga yang menentukan harga permintaan terbaru model: `list` untuk harga daftar, `managed` untuk tabel [`modelPricing`](/docs/id/settings-reference#modelpricing), atau `unknown` ketika tidak ada yang cocok dengan ID model. Field ini memerlukan Claude Code v2.1.246 atau lebih baru.

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
  output_tokens_details: BetaOutputTokensDetails | null;
};
```

`BetaServerToolUsage`, `BetaIterationsUsage`, dan `BetaOutputTokensDetails` didefinisikan dalam `@anthropic-ai/sdk`.

`output_tokens_details` memecah output yang ditagih menurut kategori. Saat ini membawa satu field, `thinking_tokens: number`, menghitung token output yang dihasilkan model sebagai penalaran internal, termasuk pembatas blok pemikiran. Field `output_tokens_details` memerlukan TypeScript SDK v0.3.228 atau lebih baru, yang menggabungkan Claude Code v2.1.228.

* **Penagihan**: baca pemecahan untuk observabilitas, bukan untuk penagihan. `output_tokens` tetap menjadi total otoritatif, dan `output_tokens - thinking_tokens` mendekati output non-penalaran.
* **Apa yang dihitung**: penalaran mentah yang dihasilkan model, yang dapat lebih panjang dari teks pemikiran yang dikembalikan dalam badan respons. API menghitungnya dengan re-tokenizing teks mentah itu, jadi dapat berbeda dari hitungan generasi eksak model dengan beberapa token.
* **Streaming**: pada pesan asisten yang dialirkan pemecahan ini, seperti `output_tokens`, adalah placeholder `message_start` dan tidak membawa hitungan nyata, jadi bacanya dari pesan hasil `usage` seperti [Baca token output dari pesan hasil](/docs/id/agent-sdk/cost-tracking#read-output-tokens-from-the-result-message) menjelaskan. Pada pesan hasil, `thinking_tokens` membaca `0` ketika model atau penyedia tidak melaporkan pemecahan.
* **Kasus `null`**: `output_tokens_details` sendiri adalah `null` pada pesan asisten yang disintesis Claude Code, seperti pesan kesalahan API.

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

<h3 id="sdkmcpresourcelink">
  `SDKMcpResourceLink`
</h3>

Satu file yang dikembalikan tool MCP dengan referensi. Claude Code membangun setiap entri dari blok `resource_link` dalam hasil tool dan mengirimkan daftar sebagai `resourceLinks` pada [`SDKUserMessage.tool_use_result`](#sdkusermessage), atau sebagai `resource_links` pada [`SDKTaskNotificationMessage`](#sdktasknotificationmessage) ketika panggilan selesai di latar belakang. Memerlukan Agent SDK v0.3.257 atau lebih baru.

```typescript theme={null}
type SDKMcpResourceLink = {
  uri: string;
  name: string;
  title?: string;
  description?: string;
  mimeType?: string;
  size?: number;
  annotations?: Record<string, unknown>;
};
```

Claude Code menghapus blok yang `uri` atau `name` bukan string, dan menghilangkan field opsional yang nilainya bukan dari tipe yang tercantum.

| Field         | Tipe                                   | Deskripsi                                           |
| :------------ | :------------------------------------- | :-------------------------------------------------- |
| `uri`         | `string`                               | URI sumber daya, seperti yang dikembalikan server   |
| `name`        | `string`                               | Nama yang diberikan server ke sumber daya           |
| `title`       | `string \| undefined`                  | Judul tampilan, ketika server menetapkannya         |
| `description` | `string \| undefined`                  | Deskripsi, ketika server menetapkannya              |
| `mimeType`    | `string \| undefined`                  | Tipe MIME, ketika server menetapkannya              |
| `size`        | `number \| undefined`                  | Ukuran dalam byte, ketika server menetapkannya      |
| `annotations` | `Record<string, unknown> \| undefined` | Objek anotasi MCP blok, ketika server menetapkannya |

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

Field `display` opsional mengontrol apakah teks pemikiran dikembalikan `"summarized"` atau `"omitted"`. Pada Claude Opus 4.7 dan yang lebih baru, default API adalah `"omitted"`, jadi atur `"summarized"` untuk menerima konten pemikiran dalam blok `thinking`. Claude Code tidak mengirimkan `display` ke Amazon Bedrock atau Agent Platform Google Cloud, jadi pada penyedia tersebut Opus 4.7 dan yang lebih baru mengembalikan blok `thinking` kosong bahkan ketika Anda menetapkan `display` ke `"summarized"`.

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

Ketika Anda memanggil `setMcpServers()`, Claude Code menerapkan aturan ini:

* **Server yang tidak dinamai panggilan**: Claude Code menjaga server yang disediakan plugin tetap berjalan. Memerlukan Agent SDK v0.3.210 atau lebih baru.
* **Server yang dinamai panggilan**: kecuali untuk server bawaan yang dimulai CLI saat startup, Claude Code mengganti server yang berjalan hanya ketika konfigurasinya berbeda dari yang Anda teruskan.
* **Server bawaan yang dimulai CLI saat startup**: jika panggilan menamai satu, Claude Code menghapus entri itu dan melaporkannya dalam `errors`.

Promise diselesaikan setelah server stdio, HTTP, dan SSE yang baru ditambahkan terhubung atau gagal, jadi tool dari server yang terhubung tersedia pada putaran berikutnya.

`added` mencantumkan server yang ditambahkan atau diganti Claude Code, terlepas dari apakah mereka terhubung. Server yang gagal terhubung muncul di `added` dan `errors`, dengan teks kegagalan di bawah `errors` dan baris `failed` dalam [`mcpServerStatus()`](#methods). Sebelum Claude Code v2.1.257, server yang upaya koneksinya melempar dilaporkan hanya di bawah `errors`.

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
  skippedLinks?: number;
};
```

`skippedLinks` menghitung jalur yang dilacak yang ditolak rewind untuk dipulihkan atau dihapus untuk keamanan link: symlink, hard link, atau file non-reguler lainnya di jalur yang dilacak, direktori parent yang tidak lagi diselesaikan ke tempat yang ditunjuk ketika checkpoint diambil, atau backup yang tidak dapat dibaca dengan aman. Field ini memerlukan Claude Code v2.1.216 atau lebih baru. Panggilan pratinjau dengan `rewindFiles(userMessageId, { dryRun: true })` tidak pernah menetapkannya.

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

Notifikasi ketika tugas latar belakang selesai, gagal, atau dihentikan. Tugas latar belakang mencakup perintah Bash `run_in_background`, watch [Monitor](#monitor), dan subagen latar belakang. Untuk field `ambient`, lihat [`SDKTaskStartedMessage`](#sdktaskstartedmessage), yang mendefinisikannya dan persyaratan versinya.

```typescript theme={null}
type SDKTaskNotificationMessage = {
  type: "system";
  subtype: "task_notification";
  task_id: string;
  tool_use_id?: string;
  status: "completed" | "failed" | "stopped";
  output_file: string;
  summary: string;
  ambient?: boolean;
  usage?: {
    total_tokens: number;
    tool_uses: number;
    duration_ms: number;
  };
  resource_links?: SDKMcpResourceLink[];
  uuid: UUID;
  session_id: string;
};
```

Ketika Claude Code [memindahkan panggilan tool MCP yang panjang ke latar belakang](/docs/id/mcp#automatic-backgrounding-of-long-tool-calls), blok `tool_result` untuk panggilan itu hanya menyimpan placeholder dan hasil nyata panggilan tiba dalam notifikasi ini. Cocokkan notifikasi dengan panggilan menggunakan `tool_use_id`. Pada notifikasi `completed`, `resource_links` mencantumkan file yang dikembalikan tool dengan referensi sebagai entri [`SDKMcpResourceLink`](#sdkmcpresourcelink), dengan batas 50-link dan 64 KiB yang sama seperti [`tool_use_result.resourceLinks`](#sdkusermessage). Claude Code menghilangkan `resource_links` ketika hasil tidak memiliki link dan pada notifikasi untuk tugas yang bukan panggilan tool MCP. `resource_links` memerlukan Agent SDK v0.3.257 atau lebih baru.

Claude Code menambahkan pemberitahuan ke setiap notifikasi tugas yang dikirimnya ke model, kecuali pengiriman yang dicap dengan [subkind `scheduled-trigger`](#task-notification-subkinds), yang membawa framing tugas yang ditugaskan sebagai gantinya. Pemberitahuan menyatakan bahwa tidak ada input manusia yang terjadi, jadi model tidak memperlakukan notifikasi sebagai instruksi atau persetujuan pengguna.

Untuk mendeteksi putaran notifikasi tugas, periksa `origin.kind === "task-notification"` pada [`SDKUserMessage`](#sdkusermessage) atau [`SDKResultMessage`](#sdkresultmessage) daripada mencocokkan teks pemberitahuan. Baca `subkind` dari field yang sama jika Anda perlu tahu apa yang memicunya. Sebelum v2.1.205, Claude Code meninggalkan pemberitahuan pada notifikasi yang tiba saat sesi menganggur.

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
  heartbeat?: boolean;
  subagent_type?: string;
  subagent_retry?: {
    agent_id: string;
    attempt: number;
    max_retries: number;
    retry_delay_ms: number;
    error_status: number | null;
    error_category: string;
  };
  uuid: UUID;
  session_id: string;
};
```

Saat panggilan tool berjalan dalam percakapan utama, Claude Code memancarkan pesan `tool_progress` setiap 30 detik dengan `heartbeat: true`. Setiap detak jantung membawa nama tool dan detik yang telah berlalu, sehingga Anda dapat membedakan panggilan yang berjalan lama dari sesi yang terhenti. Claude Code tidak memancarkan detak jantung untuk panggilan tool di dalam subagen. Field `heartbeat` memerlukan Agent SDK v0.3.214 atau lebih baru. Sebelum v2.1.257, Claude Code tidak memancarkan detak jantung untuk panggilan tool Agent foreground juga.

Pada pesan `tool_progress` untuk tool Agent selain detak jantung, `subagent_type` menamai tipe subagen yang berjalan, seperti `general-purpose`. `subagent_retry` hadir saat subagen itu menunggu backoff kesalahan API, seperti batas laju atau kelebihan beban, dengan satu pesan per upaya retry. Kedua field memerlukan Agent SDK v0.3.214 atau lebih baru.

Untuk merender indikator retry dari `subagent_retry`:

* Lacak indikator menurut `parent_tool_use_id`, yang unik per subagen. `tool_use_id` dibagikan oleh subagen paralel dari satu putaran asisten, jadi melacak menurut itu akan membiarkan pembaruan satu subagen menghapus indikator subagen lain.
* Hapus indikator ketika `tool_progress` yang lebih baru untuk `parent_tool_use_id` yang sama tiba tanpa `subagent_retry` maupun `heartbeat: true`, atau ketika pesan hasil tool tiba. Frame dengan `heartbeat: true` hanya melaporkan keaktifan, jadi pertahankan indikator ketika satu tiba. `attempt` dapat melebihi `max_retries` di bawah retry persisten, jadi jangan menurunkan penghapusan dari penghitung.
* Perlakukan `error_category` sebagai token untuk memilih teks pesan Anda sendiri, bukan sebagai teks tampilan. Nilainya adalah `rate_limit`, `overloaded`, `authentication_failed`, `server_error`, `cloud_credential_error`, dan `unknown`. Tangani nilai yang tidak Anda kenali dengan cara Anda menangani `unknown`, karena rilis yang lebih baru dapat menambahkan nilai.

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

Dipancarkan ketika tugas dimulai. Field `task_type` adalah `"local_bash"` untuk perintah Bash dan watch [Monitor](#monitor), `"local_agent"` untuk subagen, atau `"remote_agent"`.

```typescript theme={null}
type SDKTaskStartedMessage = {
  type: "system";
  subtype: "task_started";
  task_id: string;
  tool_use_id?: string;
  description: string;
  task_type?: string;
  is_backgrounded?: boolean;
  spawn_depth?: number;
  ambient?: boolean;
  uuid: UUID;
  session_id: string;
};
```

`ambient` adalah `true` untuk tugas yang bukan bagian dari pekerjaan sesi, seperti tugas yang dijalankan Claude Code untuk operasinya sendiri. Pemantau pembaruan langsung juga ambient, termasuk pemantau yang diminta pengguna. Kecualikan tugas ambient dari indikator aktivitas. Field ini memerlukan Agent SDK v0.3.247 atau lebih baru.

`ambient` juga muncul pada [`SDKTaskNotificationMessage`](#sdktasknotificationmessage) dan pada entri [`SDKBackgroundTasksChangedMessage`](#sdkbackgroundtaskschangedmessage).

`is_backgrounded` dan `spawn_depth` menjelaskan bagaimana Claude Code memulai tugas. Kedua field memerlukan Agent SDK v0.3.238 atau lebih baru.

* `is_backgrounded`: Claude Code menetapkannya pada tugas `"local_agent"` dan `"local_bash"`. `true` berarti tugas berjalan di latar belakang. `false` berarti tugas berjalan di foreground, dan panggilan tool yang memulainya tetap terblokir sampai tugas selesai atau pindah ke latar belakang.
* `spawn_depth`: Claude Code menetapkannya pada tugas `"local_agent"` saja. Subagen yang dihasilkan thread utama memiliki kedalaman `1`. Subagen yang dihasilkan subagen kedalaman `1` memiliki kedalaman `2`, dan seterusnya.

[Subagen yang dilanjutkan](/docs/id/agent-sdk/subagents#resume-subagents) selalu melaporkan `is_backgrounded: true`, karena Claude Code menjalankan setiap subagen yang dilanjutkan di latar belakang. Ketika tugas foreground pindah ke latar belakang nanti, Claude Code melaporkan nilai `is_backgrounded` baru dalam pesan [`task_updated`](#sdktaskupdatedmessage) daripada mengirimkan `task_started` kedua.

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

Dipancarkan setiap kali set tugas latar belakang yang aktif berubah: tugas dimulai, selesai, dibunuh, agen foreground di-background, atau field `description` atau `ambient` tugas berubah.

Array `tasks` adalah set aktif lengkap. Ganti set yang di-cache dengan setiap payload alih-alih memasangkan acara `task_started` dan `task_notification`, sehingga perubahan keanggotaan berikutnya memperbaiki acara apa pun yang Anda lewatkan.

Pengurutan relatif terhadap acara per-tugas tersebut tidak ditentukan, jadi jangan menghubungkan dua aliran tersebut.

Tidak ada yang dipancarkan saat startup. Atur ulang ke set kosong setiap kali proses CLI sesi dimulai atau dimulai ulang dan biarkan perubahan keanggotaan berikutnya mengisinya kembali.

Ketika Anda mengirimkan permintaan kontrol `initialize` berulang ke sesi yang berjalan, seperti dengan [`reinitialize()`](#query-object) setelah celah transport, Claude Code mengikuti respons dengan snapshot set aktif saat ini, bahkan ketika kosong. Host yang terhubung kembali dengan demikian mempelajari apa yang berjalan tanpa menunggu perubahan keanggotaan berikutnya. Sebelum Agent SDK v0.3.239, Claude Code tidak mengirimkan snapshot setelah `initialize` berulang.

Memerlukan Claude Code v2.1.203 atau lebih baru.

```typescript theme={null}
type SDKBackgroundTasksChangedMessage = {
  type: "system";
  subtype: "background_tasks_changed";
  tasks: {
    task_id: string;
    task_type: string;
    description: string;
    ambient?: boolean;
  }[];
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkthinkingtokensmessage">
  `SDKThinkingTokensMessage`
</h3>

Dipancarkan saat Claude menghasilkan blok pemikiran, termasuk yang diredaksi. `estimated_tokens` adalah estimasi berjalan token pemikiran yang dihasilkan sejauh ini dalam blok saat ini, dan `estimated_tokens_delta` adalah kenaikan yang dibawa frame ini. Gunakan estimasi ini untuk tampilan kemajuan.

Ketika model atau penyedia melaporkan pemecahan, hitungan akhir untuk loop agen tingkat atas adalah [`usage.output_tokens_details.thinking_tokens`](#usage) pesan hasil, yang [tidak termasuk token subagen](/docs/id/agent-sdk/cost-tracking#get-the-total-cost-of-a-query).

Memerlukan Claude Code v2.1.153 atau lebih baru.

```typescript theme={null}
type SDKThinkingTokensMessage = {
  type: "system";
  subtype: "thinking_tokens";
  estimated_tokens: number;
  estimated_tokens_delta: number;
  user_message_uuid?: string;
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

Claude Code tidak memancarkan tipe pesan ini. Ketika Anda mengirimkan perintah seperti `/context` atau `/usage` sebagai prompt, outputnya tiba sebagai [`SDKAssistantMessage`](#sdkassistantmessage).

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

Dipancarkan ketika set perintah yang tersedia berubah di tengah sesi, seperti ketika Claude Code menemukan skills saat agen memasuki subdirektori. Array `commands` adalah daftar lengkap yang diperbarui, jadi ganti daftar perintah yang di-cache dengan payload ini. Memanggil [`supportedCommands()`](#query-object) setelah pesan ini mengembalikan daftar yang diperbarui yang sama, karena metode melacak push terbaru; ini memerlukan Agent SDK v0.3.216 atau lebih baru. Dalam versi SDK yang lebih awal, `supportedCommands()` mengembalikan snapshot yang ditangkap saat inisialisasi dan tidak pernah mencerminkan perubahan di tengah sesi.

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

Dipancarkan setelah putaran ketika [`promptSuggestions`](#options) diaktifkan dan Claude Code menghasilkan saran untuk putaran itu. Berisi prompt pengguna berikutnya yang diprediksi. Untuk putaran yang tidak mendapat saran, lihat [Ketika Claude Code melewatkan saran](/docs/id/interactive-mode#when-claude-code-skips-suggestions).

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

Dipancarkan ketika percakapan sesi diganti tanpa mengakhiri sesi. Dalam panggilan `query()`, hanya `/clear` dan aliasnya yang menghasilkan pesan ini. Pasang transkrip kosong di bawah `new_conversation_id` dan buang judul sesi yang di-cache.

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

`AbortError` adalah satu-satunya kelas error dalam API yang diketik SDK. Kegagalan lainnya, seperti proses Claude Code keluar atau gagal diluncurkan, menolak iterasi pesan dengan error yang tidak membawa kelas SDK untuk dicocokkan. [Troubleshooting](/docs/id/agent-sdk/troubleshooting) mengetik error tersebut menurut pesan, dengan penyebab dan perbaikan untuk masing-masing.

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

| Properti                    | Tipe                                                  | Default     | Deskripsi                                                                                                                                                                                                                                      |
| :-------------------------- | :---------------------------------------------------- | :---------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `enabled`                   | `boolean`                                             | `false`     | Aktifkan mode sandbox untuk eksekusi perintah                                                                                                                                                                                                  |
| `failIfUnavailable`         | `boolean`                                             | `true`      | Berhenti saat startup jika `enabled` adalah `true` tetapi sandbox tidak dapat dimulai. Atur `false` untuk kembali ke eksekusi unsandboxed dengan peringatan di stderr                                                                          |
| `autoAllowBashIfSandboxed`  | `boolean`                                             | `true`      | Auto-approve perintah Bash ketika sandbox diaktifkan                                                                                                                                                                                           |
| `excludedCommands`          | `string[]`                                            | `[]`        | Perintah yang bypass pembatasan sandbox, seperti `['docker *']`. Ini berjalan unsandboxed secara otomatis tanpa keterlibatan model; [`sandbox.excludedCommands`](/docs/id/settings-reference#sandbox-excludedcommands) mencakup kapan entri berlaku |
| `allowUnsandboxedCommands`  | `boolean`                                             | `true`      | Izinkan model untuk meminta menjalankan perintah di luar sandbox. Ketika `true`, model dapat mengatur `dangerouslyDisableSandbox` dalam input tool, yang jatuh kembali ke [sistem izin](#permissions-fallback-for-unsandboxed-commands)        |
| `network`                   | [`SandboxNetworkConfig`](#sandboxnetworkconfig)       | `undefined` | Konfigurasi sandbox spesifik jaringan                                                                                                                                                                                                          |
| `filesystem`                | [`SandboxFilesystemConfig`](#sandboxfilesystemconfig) | `undefined` | Konfigurasi sandbox spesifik filesystem untuk pembatasan baca/tulis                                                                                                                                                                            |
| `ignoreViolations`          | `Record<string, string[]>`                            | `undefined` | Peta substring perintah, atau `*` untuk setiap perintah, ke substring teks pelanggaran untuk diabaikan, seperti `{ "*": ['/etc/hosts'] }`; lihat [`sandbox.ignoreViolations`](/docs/id/settings-reference#sandbox-ignoreviolations)                 |
| `enableWeakerNestedSandbox` | `boolean`                                             | `false`     | Aktifkan sandbox bersarang yang lebih lemah untuk kompatibilitas                                                                                                                                                                               |
| `ripgrep`                   | `{ command: string; args?: string[] }`                | `undefined` | Konfigurasi biner ripgrep kustom untuk lingkungan sandbox                                                                                                                                                                                      |

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
  **Keamanan Unix socket:** Opsi `allowUnixSockets` dapat memberikan akses ke layanan sistem yang menjangkau di luar sandbox. Misalnya, mengizinkan `/var/run/docker.sock` secara efektif memberikan akses sistem host penuh melalui API Docker, melewati isolasi sandbox. Hanya izinkan Unix socket yang benar-benar diperlukan dan pahami implikasi keamanan dari masing-masing.
</Warning>

<h3 id="sandboxnetworkconfig">
  `SandboxNetworkConfig`
</h3>

Konfigurasi spesifik jaringan untuk mode sandbox. Pengaturan ini berlaku untuk perintah Bash sandboxed ketika `enabled` adalah `true` dalam [`SandboxSettings`](#sandboxsettings) induk. Mereka tidak membatasi tool WebFetch, yang menggunakan [aturan izin](/docs/id/permissions#webfetch) sebagai gantinya.

```typescript theme={null}
type SandboxNetworkConfig = {
  allowedDomains?: string[];
  deniedDomains?: string[];
  strictAllowlist?: boolean;
  allowManagedDomainsOnly?: boolean;
  allowLocalBinding?: boolean;
  allowUnixSockets?: string[];
  allowAllUnixSockets?: boolean;
  httpProxyPort?: number;
  socksProxyPort?: number;
};
```

| Properti                  | Tipe       | Default     | Deskripsi                                                                                                                                                                                                                                                                                                                                                                                  |
| :------------------------ | :--------- | :---------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `allowedDomains`          | `string[]` | `[]`        | Nama domain yang dapat diakses proses sandboxed                                                                                                                                                                                                                                                                                                                                            |
| `deniedDomains`           | `string[]` | `[]`        | Nama domain yang tidak dapat diakses proses sandboxed. Mengambil prioritas atas `allowedDomains`                                                                                                                                                                                                                                                                                           |
| `strictAllowlist`         | `boolean`  | `false`     | Tolak akses perintah sandboxed ke host di luar [daftar izin jaringan](/docs/id/sandboxing#network-isolation) alih-alih meminta. Diberlakukan untuk perintah sandboxed saja; tool dalam proses seperti WebFetch tidak dibatasi olehnya. Hanya dihormati dari pengaturan pengguna, terkelola, atau CLI `--settings`; pengaturan proyek diabaikan. Memerlukan Claude Code v2.1.219 atau lebih baru |
| `allowManagedDomainsOnly` | `boolean`  | `false`     | Hanya pengaturan yang dikelola. Ketika diatur dalam [pengaturan yang dikelola](/docs/id/managed-settings), hanya entri `allowedDomains` dan aturan izin `WebFetch(domain:...)` dari pengaturan yang dikelola yang dihormati, dan entri izin dari pengaturan pengguna, proyek, atau lokal diabaikan. Tidak berpengaruh ketika diatur melalui opsi SDK                                            |
| `allowLocalBinding`       | `boolean`  | `false`     | Izinkan proses untuk mengikat ke port lokal (misalnya, untuk dev server)                                                                                                                                                                                                                                                                                                                   |
| `allowUnixSockets`        | `string[]` | `[]`        | Jalur Unix socket yang dapat diakses proses (misalnya, Docker socket)                                                                                                                                                                                                                                                                                                                      |
| `allowAllUnixSockets`     | `boolean`  | `false`     | Izinkan akses ke semua Unix socket                                                                                                                                                                                                                                                                                                                                                         |
| `httpProxyPort`           | `number`   | `undefined` | Port proxy HTTP untuk permintaan jaringan                                                                                                                                                                                                                                                                                                                                                  |
| `socksProxyPort`          | `number`   | `undefined` | Port proxy SOCKS untuk permintaan jaringan                                                                                                                                                                                                                                                                                                                                                 |

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

Ketika `allowUnsandboxedCommands` diaktifkan, model dapat meminta untuk menjalankan perintah di luar sandbox dengan mengatur `dangerouslyDisableSandbox: true` dalam input tool. Permintaan ini jatuh kembali ke sistem izin yang ada, berarti handler `canUseTool` Anda dipanggil, memungkinkan Anda untuk mengimplementasikan logika otorisasi kustom.

Entri `excludedCommands` Anda sebagai gantinya mengambil perintah keluar dari sandbox tanpa keterlibatan model; [`sandbox.excludedCommands`](/docs/id/settings-reference#sandbox-excludedcommands) mencakup kapan entri berlaku.

Dalam contoh di bawah, `isCommandAuthorized` mewakili pemeriksaan otorisasi yang Anda tentukan.

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

<Warning>
  Perintah yang berjalan dengan `dangerouslyDisableSandbox: true` memiliki akses sistem penuh. Pastikan handler `canUseTool` Anda memvalidasi permintaan ini dengan hati-hati.

  Jika `permissionMode` diatur ke `bypassPermissions` dan `allowUnsandboxedCommands` diaktifkan, model dapat secara otonom mengeksekusi perintah di luar sandbox tanpa prompt persetujuan apa pun, terlepas dari [tindakan yang tidak ada mode auto-approve](/docs/id/permission-modes#actions-no-mode-auto-approves). Kombinasi ini secara efektif memungkinkan model untuk melarikan diri dari isolasi sandbox secara diam-diam.
</Warning>

<h2 id="see-also">
  Lihat juga
</h2>

* [Gambaran umum SDK](/docs/id/agent-sdk/overview) - Konsep SDK umum
* [Referensi SDK Python](/docs/id/agent-sdk/python) - Dokumentasi SDK Python
* [Referensi CLI](/docs/id/cli-reference) - Antarmuka baris perintah
* [Alur kerja umum](/docs/id/common-workflows) - Panduan langkah demi langkah
