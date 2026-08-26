> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# TypeScript SDK V2 session API (dihapus)

> Referensi untuk API sesi SDK Agent TypeScript V2 yang dihapus, dengan pola send/stream berbasis sesi untuk percakapan multi-turn.

<Warning>
  API sesi V2 tidak lagi didukung. TypeScript Agent SDK 0.3.142 menghapus `unstable_v2_createSession`, `unstable_v2_resumeSession`, `unstable_v2_prompt`, dan tipe `SDKSession` serta `SDKSessionOptions`.

  Untuk bermigrasi, gunakan [API `query()`](/docs/id/agent-sdk/typescript) dan [opsi sesi](/docs/id/agent-sdk/sessions) yang diterimanya. Lewatkan `AsyncIterable<SDKUserMessage>` untuk percakapan multi-turn, atau `options.resume` untuk melanjutkan sesi yang disimpan. Halaman ini disimpan untuk referensi jika Anda mempertahankan kode pada Agent SDK 0.2.x atau lebih awal.
</Warning>

V2 adalah API sesi eksperimental yang menghilangkan kebutuhan untuk async generators dan koordinasi yield. Alih-alih mengelola status generator di seluruh turn, setiap turn adalah siklus `send()`/`stream()` terpisah. Permukaan API berkurang menjadi tiga konsep:

* `createSession()` / `resumeSession()`: Mulai atau lanjutkan percakapan
* `session.send()`: Kirim pesan
* `session.stream()`: Dapatkan respons

<h2 id="installation">
  Instalasi
</h2>

Agent SDK 0.2.x adalah versi terakhir yang menyertakan antarmuka V2. Versi paket melompat dari 0.2.x langsung ke 0.3.142, jadi versi penghapusan di atas dan pin instalasi di bawah menggambarkan batas yang sama. Untuk menginstal rilis terakhir yang kompatibel dengan V2, pin versi mayor dan minor:

```bash theme={null}
npm install @anthropic-ai/claude-agent-sdk@0.2
```

<Note>
  SDK menggabungkan binary Claude Code asli untuk platform Anda sebagai dependensi opsional, jadi Anda tidak perlu menginstal Claude Code secara terpisah.
</Note>

<h2 id="quick-start">
  Mulai cepat
</h2>

<h3 id="one-shot-prompt">
  Prompt sekali jalan
</h3>

Untuk kueri single-turn sederhana di mana Anda tidak perlu mempertahankan sesi, gunakan `unstable_v2_prompt()`. Contoh ini mengirim pertanyaan matematika dan mencatat jawabannya:

```typescript theme={null}
import { unstable_v2_prompt } from "@anthropic-ai/claude-agent-sdk";

const result = await unstable_v2_prompt("What is 2 + 2?", {
  model: "claude-opus-4-7"
});
if (result.subtype === "success") {
  console.log(result.result);
}
```

<details>
  <summary>Lihat operasi yang sama di V1</summary>

  ```typescript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const q = query({
    prompt: "What is 2 + 2?",
    options: { model: "claude-opus-4-7" }
  });

  for await (const msg of q) {
    if (msg.type === "result" && msg.subtype === "success") {
      console.log(msg.result);
    }
  }
  ```
</details>

<h3 id="basic-session">
  Sesi dasar
</h3>

Untuk interaksi di luar prompt tunggal, buat sesi. V2 memisahkan pengiriman dan streaming menjadi langkah-langkah yang berbeda:

* `send()` mengirimkan pesan Anda
* `stream()` mengalirkan respons kembali

Pemisahan eksplisit ini memudahkan untuk menambahkan logika antar turn (seperti memproses respons sebelum mengirim tindak lanjut).

Contoh di bawah membuat sesi, mengirim "Hello!" ke Claude, dan mencetak respons teks. Ini menggunakan [`await using`](https://www.typescriptlang.org/docs/handbook/release-notes/typescript-5-2.html#using-declarations-and-explicit-resource-management) (TypeScript 5.2+) untuk secara otomatis menutup sesi ketika blok keluar. Anda juga dapat memanggil `session.close()` secara manual.

```typescript theme={null}
import { unstable_v2_createSession } from "@anthropic-ai/claude-agent-sdk";

await using session = unstable_v2_createSession({
  model: "claude-opus-4-7"
});

await session.send("Hello!");
for await (const msg of session.stream()) {
  // Filter for assistant messages to get human-readable output
  if (msg.type === "assistant") {
    const text = msg.message.content
      .filter((block) => block.type === "text")
      .map((block) => block.text)
      .join("");
    console.log(text);
  }
}
```

<details>
  <summary>Lihat operasi yang sama di V1</summary>

  Di V1, aliran input dan output melalui generator async tunggal. Untuk prompt dasar ini terlihat serupa, tetapi menambahkan logika multi-turn memerlukan restrukturisasi untuk menggunakan generator input.

  ```typescript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const q = query({
    prompt: "Hello!",
    options: { model: "claude-opus-4-7" }
  });

  for await (const msg of q) {
    if (msg.type === "assistant") {
      const text = msg.message.content
        .filter((block) => block.type === "text")
        .map((block) => block.text)
        .join("");
      console.log(text);
    }
  }
  ```
</details>

<h3 id="multi-turn-conversation">
  Percakapan multi-turn
</h3>

Sesi mempertahankan konteks di seluruh pertukaran berganda. Untuk melanjutkan percakapan, panggil `send()` lagi pada sesi yang sama. Claude mengingat turn sebelumnya.

Contoh ini mengajukan pertanyaan matematika, kemudian mengajukan tindak lanjut yang mereferensikan jawaban sebelumnya:

```typescript theme={null}
import { unstable_v2_createSession } from "@anthropic-ai/claude-agent-sdk";

await using session = unstable_v2_createSession({
  model: "claude-opus-4-7"
});

// Turn 1
await session.send("What is 5 + 3?");
for await (const msg of session.stream()) {
  // Filter for assistant messages to get human-readable output
  if (msg.type === "assistant") {
    const text = msg.message.content
      .filter((block) => block.type === "text")
      .map((block) => block.text)
      .join("");
    console.log(text);
  }
}

// Turn 2
await session.send("Multiply that by 2");
for await (const msg of session.stream()) {
  if (msg.type === "assistant") {
    const text = msg.message.content
      .filter((block) => block.type === "text")
      .map((block) => block.text)
      .join("");
    console.log(text);
  }
}
```

<details>
  <summary>Lihat operasi yang sama di V1</summary>

  ```typescript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // Must create an async iterable to feed messages
  async function* createInputStream() {
    yield {
      type: "user",
      session_id: "",
      message: { role: "user", content: [{ type: "text", text: "What is 5 + 3?" }] },
      parent_tool_use_id: null
    };
    // Must coordinate when to yield next message
    yield {
      type: "user",
      session_id: "",
      message: { role: "user", content: [{ type: "text", text: "Multiply by 2" }] },
      parent_tool_use_id: null
    };
  }

  const q = query({
    prompt: createInputStream(),
    options: { model: "claude-opus-4-7" }
  });

  for await (const msg of q) {
    if (msg.type === "assistant") {
      const text = msg.message.content
        .filter((block) => block.type === "text")
        .map((block) => block.text)
        .join("");
      console.log(text);
    }
  }
  ```
</details>

<h3 id="session-resume">
  Lanjutkan sesi
</h3>

Jika Anda memiliki ID sesi dari interaksi sebelumnya, Anda dapat melanjutkannya nanti. Ini berguna untuk alur kerja yang berjalan lama atau ketika Anda perlu mempertahankan percakapan di seluruh restart aplikasi.

Contoh ini membuat sesi, menyimpan ID-nya, menutupnya, kemudian melanjutkan percakapan:

```typescript theme={null}
import {
  unstable_v2_createSession,
  unstable_v2_resumeSession,
  type SDKMessage
} from "@anthropic-ai/claude-agent-sdk";

// Helper to extract text from assistant messages
function getAssistantText(msg: SDKMessage): string | null {
  if (msg.type !== "assistant") return null;
  return msg.message.content
    .filter((block) => block.type === "text")
    .map((block) => block.text)
    .join("");
}

// Create initial session and have a conversation
const session = unstable_v2_createSession({
  model: "claude-opus-4-7"
});

await session.send("Remember this number: 42");

// Get the session ID from any received message
let sessionId: string | undefined;
for await (const msg of session.stream()) {
  sessionId = msg.session_id;
  const text = getAssistantText(msg);
  if (text) console.log("Initial response:", text);
}

console.log("Session ID:", sessionId);
session.close();

// Later: resume the session using the stored ID
await using resumedSession = unstable_v2_resumeSession(sessionId!, {
  model: "claude-opus-4-7"
});

await resumedSession.send("What number did I ask you to remember?");
for await (const msg of resumedSession.stream()) {
  const text = getAssistantText(msg);
  if (text) console.log("Resumed response:", text);
}
```

<details>
  <summary>Lihat operasi yang sama di V1</summary>

  ```typescript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // Create initial session
  const initialQuery = query({
    prompt: "Remember this number: 42",
    options: { model: "claude-opus-4-7" }
  });

  // Get session ID from any message
  let sessionId: string | undefined;
  for await (const msg of initialQuery) {
    sessionId = msg.session_id;
    if (msg.type === "assistant") {
      const text = msg.message.content
        .filter((block) => block.type === "text")
        .map((block) => block.text)
        .join("");
      console.log("Initial response:", text);
    }
  }

  console.log("Session ID:", sessionId);

  // Later: resume the session
  const resumedQuery = query({
    prompt: "What number did I ask you to remember?",
    options: {
      model: "claude-opus-4-7",
      resume: sessionId
    }
  });

  for await (const msg of resumedQuery) {
    if (msg.type === "assistant") {
      const text = msg.message.content
        .filter((block) => block.type === "text")
        .map((block) => block.text)
        .join("");
      console.log("Resumed response:", text);
    }
  }
  ```
</details>

<h3 id="cleanup">
  Pembersihan
</h3>

Sesi dapat ditutup secara manual atau otomatis menggunakan [`await using`](https://www.typescriptlang.org/docs/handbook/release-notes/typescript-5-2.html#using-declarations-and-explicit-resource-management), fitur TypeScript 5.2+ untuk pembersihan sumber daya otomatis. Jika Anda menggunakan versi TypeScript yang lebih lama atau mengalami masalah kompatibilitas, gunakan pembersihan manual sebagai gantinya.

**Pembersihan otomatis (TypeScript 5.2+):**

```typescript theme={null}
import { unstable_v2_createSession } from "@anthropic-ai/claude-agent-sdk";

await using session = unstable_v2_createSession({
  model: "claude-opus-4-7"
});
// Session closes automatically when the block exits
```

**Pembersihan manual:**

```typescript theme={null}
import { unstable_v2_createSession } from "@anthropic-ai/claude-agent-sdk";

const session = unstable_v2_createSession({
  model: "claude-opus-4-7"
});
// ... use the session ...
session.close();
```

<h2 id="api-reference">
  Referensi API
</h2>

<h3 id="unstable_v2_createsession">
  `unstable_v2_createSession()`
</h3>

Membuat sesi baru untuk percakapan multi-turn.

```typescript theme={null}
function unstable_v2_createSession(options: {
  model: string;
  // Additional options supported
}): SDKSession;
```

<h3 id="unstable_v2_resumesession">
  `unstable_v2_resumeSession()`
</h3>

Melanjutkan sesi yang ada berdasarkan ID.

```typescript theme={null}
function unstable_v2_resumeSession(
  sessionId: string,
  options: {
    model: string;
    // Additional options supported
  }
): SDKSession;
```

<h3 id="unstable_v2_prompt">
  `unstable_v2_prompt()`
</h3>

Fungsi kenyamanan sekali jalan untuk kueri single-turn.

```typescript theme={null}
function unstable_v2_prompt(
  prompt: string,
  options: {
    model: string;
    // Additional options supported
  }
): Promise<SDKResultMessage>;
```

<h3 id="sdksession-interface">
  Antarmuka SDKSession
</h3>

```typescript theme={null}
interface SDKSession {
  readonly sessionId: string;
  send(message: string | SDKUserMessage): Promise<void>;
  stream(): AsyncGenerator<SDKMessage, void>;
  close(): void;
}
```

<h2 id="feature-availability">
  Ketersediaan fitur
</h2>

API sesi V2 tidak mendukung setiap fitur V1. Berikut ini memerlukan [SDK V1](/docs/id/agent-sdk/typescript):

* Session forking (opsi `forkSession`)
* Beberapa pola input streaming lanjutan

<h2 id="see-also">
  Lihat juga
</h2>

* [Referensi TypeScript SDK (V1)](/docs/id/agent-sdk/typescript) - Dokumentasi SDK V1 lengkap
* [Gambaran umum SDK](/docs/id/agent-sdk/overview) - Konsep SDK umum
* [Contoh V2 di GitHub](https://github.com/anthropics/claude-agent-sdk-demos/tree/main/hello-world-v2) - Contoh kode yang berfungsi
