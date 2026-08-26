> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Simpan sesi ke penyimpanan eksternal

> Cerminkan transkrip sesi ke S3, Redis, atau backend Anda sendiri sehingga host apa pun dapat melanjutkannya.

Secara default, SDK menulis transkrip sesi ke file JSONL di bawah `~/.claude/projects/` pada sistem file lokal. Adaptor `SessionStore` memungkinkan Anda mencerminkan transkrip tersebut ke backend Anda sendiri, seperti S3, Redis, atau database, sehingga sesi yang dibuat di satu host dapat dilanjutkan di host lain.

Alasan umum untuk menggunakan session store:

* **Penerapan multi-host.** Fungsi serverless, pekerja yang diskalakan otomatis, dan runner CI tidak berbagi sistem file. Penyimpanan bersama memungkinkan replika apa pun melanjutkan sesi apa pun.
* **Daya tahan.** Kontainer lokal bersifat sementara. Penyimpanan yang didukung oleh S3 atau database bertahan melalui restart dan redeploy.
* **Kepatuhan dan audit.** Simpan transkrip dalam penyimpanan yang sudah Anda kelola, dengan aturan retensi, enkripsi, dan kontrol akses Anda sendiri.

<h2 id="the-sessionstore-interface">
  Antarmuka `SessionStore`
</h2>

`SessionStore` adalah objek dengan dua metode yang diperlukan, `append` dan `load`, serta tiga metode opsional. SDK memanggil `append` untuk menulis entri transkrip selama kueri dan `load` untuk membacanya kembali untuk resume.

<CodeGroup>
  ```typescript TypeScript theme={null}
  // Exported from @anthropic-ai/claude-agent-sdk as
  // SessionStore, SessionKey, SessionStoreEntry.

  type SessionKey = {
    projectKey: string;
    sessionId: string;
    subpath?: string;
  };

  type SessionStore = {
    // Required
    append(key: SessionKey, entries: SessionStoreEntry[]): Promise<void>;
    load(key: SessionKey): Promise<SessionStoreEntry[] | null>;

    // Optional
    listSessions?(
      projectKey: string,
    ): Promise<Array<{ sessionId: string; mtime: number }>>;
    delete?(key: SessionKey): Promise<void>;
    listSubkeys?(key: {
      projectKey: string;
      sessionId: string;
    }): Promise<string[]>;
  };
  ```

  ```python Python theme={null}
  # Exported from claude_agent_sdk as
  # SessionStore, SessionKey, SessionStoreEntry.

  class SessionKey(TypedDict):
      project_key: str
      session_id: str
      subpath: NotRequired[str]

  class SessionStore(Protocol):
      # Required
      async def append(
          self, key: SessionKey, entries: list[SessionStoreEntry]
      ) -> None: ...
      async def load(self, key: SessionKey) -> list[SessionStoreEntry] | None: ...

      # Optional — omit or raise NotImplementedError
      async def list_sessions(
          self, project_key: str
      ) -> list[SessionStoreListEntry]: ...
      async def delete(self, key: SessionKey) -> None: ...
      async def list_subkeys(self, key: SessionListSubkeysKey) -> list[str]: ...
  ```
</CodeGroup>

`SessionKey` mengatasi satu transkrip. `projectKey` adalah pengkodean stabil dan aman sistem file dari direktori kerja, `sessionId` adalah UUID sesi, dan `subpath` diatur ketika entri milik transkrip subagent atau file sidecar daripada percakapan utama. Perlakukan `subpath` sebagai sufiks kunci yang tidak transparan; ini mengikuti tata letak on-disk, misalnya `subagents/agent-<id>`. Ketika `subpath` tidak ditentukan, kunci merujuk ke transkrip utama.

| Metode         | Diperlukan | Dipanggil ketika                                                                                                                                                                                                     |
| :------------- | :--------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `append`       | Ya         | Setelah setiap batch entri transkrip ditulis secara lokal. Entri adalah objek yang aman JSON, satu per baris dalam JSONL lokal.                                                                                      |
| `load`         | Ya         | Sekali sebelum subprocess spawn, ketika `resume` diatur. Kembalikan `null` jika sesi tidak dikenal.                                                                                                                  |
| `listSessions` | Tidak      | Oleh `listSessions({ sessionStore })` dan oleh `query()`/`startup()` dengan `continue: true`. Jika tidak ditentukan, panggilan tersebut melempar.                                                                    |
| `delete`       | Tidak      | Oleh `deleteSession({ sessionStore })`. Menghapus kunci utama (tanpa `subpath`) harus cascade ke semua subkey untuk sesi itu. Jika tidak ditentukan, penghapusan adalah no-op, yang cocok untuk backend append-only. |
| `listSubkeys`  | Tidak      | Selama resume, untuk menemukan transkrip subagent. Jika tidak ditentukan, hanya transkrip utama yang dipulihkan.                                                                                                     |

<h2 id="quick-start">
  Mulai cepat
</h2>

SDK mengirimkan `InMemorySessionStore` untuk pengembangan dan pengujian. Contoh di bawah menjalankan kueri dengan penyimpanan yang terpasang, menangkap ID sesi dari pesan hasil, kemudian melanjutkan dari penyimpanan dalam panggilan `query()` kedua. Panggilan kedua melewatkan instance penyimpanan yang sama ditambah `resume`, sehingga SDK memuat transkrip dari penyimpanan daripada sistem file lokal:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query, InMemorySessionStore } from "@anthropic-ai/claude-agent-sdk";

  const store = new InMemorySessionStore();

  let sessionId: string | undefined;
  for await (const message of query({
    prompt: "List the TypeScript files under src/",
    options: { sessionStore: store },
  })) {
    if (message.type === "result") {
      sessionId = message.session_id;
    }
  }

  // Resume from the store. The agent has full context from the first call.
  for await (const message of query({
    prompt: "Summarize what those files do",
    options: { sessionStore: store, resume: sessionId },
  })) {
    if (message.type === "result" && message.subtype === "success") {
      console.log(message.result);
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import (
      ClaudeAgentOptions,
      InMemorySessionStore,
      ResultMessage,
      query,
  )

  store = InMemorySessionStore()


  async def main():
      session_id = None
      async for message in query(
          prompt="List the Python files under src/",
          options=ClaudeAgentOptions(session_store=store),
      ):
          if isinstance(message, ResultMessage):
              session_id = message.session_id

      # Resume from the store. The agent has full context from the first call.
      async for message in query(
          prompt="Summarize what those files do",
          options=ClaudeAgentOptions(session_store=store, resume=session_id),
      ):
          if isinstance(message, ResultMessage) and message.subtype == "success":
              print(message.result)


  asyncio.run(main())
  ```
</CodeGroup>

Kueri kedua mencetak ringkasan file dari kueri pertama, yang menunjukkan bahwa agen melanjutkan dengan konteks penuh dari penyimpanan.

<h2 id="write-your-own-adapter">
  Tulis adaptor Anda sendiri
</h2>

Implementasikan `append` dan `load` terhadap backend Anda. Tambahkan `listSessions`, `delete`, dan `listSubkeys` jika Anda ingin `listSessions()`, `deleteSession()`, dan subagent resume bekerja terhadap penyimpanan.

Entri yang dilewatkan ke `append` diketik sebagai `SessionStoreEntry` (objek `{ type: string; ... }`). Perlakukan mereka sebagai nilai yang aman JSON yang tidak transparan: simpan dalam urutan dan kembalikan dari `load` dalam urutan yang sama. `load` harus mengembalikan entri yang deep-equal dengan apa yang ditambahkan; serialisasi byte-equal tidak diperlukan, jadi backend seperti Postgres `jsonb` yang mengurutkan ulang kunci objek tidak masalah.

<h2 id="reference-implementations">
  Implementasi referensi
</h2>

Repositori TypeScript SDK mencakup adaptor referensi yang dapat dijalankan untuk S3, Redis, dan Postgres di bawah [`examples/session-stores/`](https://github.com/anthropics/claude-agent-sdk-typescript/tree/main/examples/session-stores). Mereka tidak dipublikasikan ke npm; salin file `src/` yang Anda butuhkan ke proyek Anda dan instal klien backend yang sesuai.

| Adaptor                                                                                                                        | Klien backend        | Model penyimpanan                                                                             |
| :----------------------------------------------------------------------------------------------------------------------------- | :------------------- | :-------------------------------------------------------------------------------------------- |
| [`S3SessionStore`](https://github.com/anthropics/claude-agent-sdk-typescript/tree/main/examples/session-stores/s3)             | `@aws-sdk/client-s3` | Satu file bagian JSONL per `append()`; `load()` mencantumkan, mengurutkan, dan menggabungkan. |
| [`RedisSessionStore`](https://github.com/anthropics/claude-agent-sdk-typescript/tree/main/examples/session-stores/redis)       | `ioredis`            | `RPUSH`/`LRANGE` list per transkrip, ditambah indeks sorted-set sesi.                         |
| [`PostgresSessionStore`](https://github.com/anthropics/claude-agent-sdk-typescript/tree/main/examples/session-stores/postgres) | `pg`                 | Satu baris per entri dalam tabel `jsonb`, diurutkan oleh `BIGSERIAL`.                         |

Setiap adaptor mengambil instance klien yang telah dikonfigurasi sebelumnya, sehingga Anda mengontrol kredensial, TLS, region, dan pooling. Misalnya, dengan S3:

```typescript TypeScript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";
import { S3Client } from "@aws-sdk/client-s3";
import { S3SessionStore } from "./S3SessionStore"; // copied from examples/session-stores/s3

const store = new S3SessionStore({
  bucket: "my-claude-sessions",
  prefix: "transcripts",
  client: new S3Client({ region: "us-east-1" }),
});

for await (const message of query({
  prompt: "Hello!",
  options: { sessionStore: store },
})) {
  if (message.type === "result" && message.subtype === "success") {
    console.log(message.result);
  }
}

// Later, possibly on a different host:
for await (const message of query({
  prompt: "Continue where we left off",
  options: { sessionStore: store, resume: "previous-session-id" },
})) {
  // ...
}
```

<h3 id="validate-your-adapter">
  Validasi adaptor Anda
</h3>

Kedua SDK mengirimkan suite conformance yang menegaskan kontrak perilaku `append`, `load`, dan metode opsional harus memuaskan. Tes untuk metode opsional melewati secara otomatis ketika metode tersebut tidak diimplementasikan.

Di TypeScript, salin [`shared/conformance.ts`](https://github.com/anthropics/claude-agent-sdk-typescript/blob/main/examples/session-stores/shared/conformance.ts) dari direktori contoh ke dalam suite pengujian Anda. Di Python, suite dikirimkan dalam paket:

```python Python theme={null}
import pytest
from claude_agent_sdk.testing import run_session_store_conformance


@pytest.mark.asyncio
async def test_my_store_conformance():
    await run_session_store_conformance(MyRedisStore)
```

<h2 id="behavior-notes">
  Catatan perilaku
</h2>

<h3 id="dual-write-architecture">
  Arsitektur dual-write
</h3>

Penyimpanan adalah cerminan, bukan pengganti. Subprocess Claude Code selalu menulis ke disk lokal terlebih dahulu; SDK kemudian meneruskan setiap batch ke `append()`. Jika Anda ingin salinan lokal bersifat sementara, arahkan `CLAUDE_CONFIG_DIR` ke direktori temp di `options.env`. Karena cerminan bergantung pada penulisan lokal, `sessionStore` tidak dapat digabungkan dengan `persistSession: false`; SDK melempar jika Anda menetapkan keduanya. Ini juga melempar jika digabungkan dengan `enableFileCheckpointing`, karena blob cadangan riwayat file ditulis langsung ke disk lokal dan tidak dicerminkan ke penyimpanan.

<h3 id="mirror-writes-are-best-effort">
  Penulisan cerminan adalah best-effort
</h3>

Jika `append()` menolak, SDK mencoba ulang batch hingga dua kali lagi dengan backoff singkat, untuk maksimal tiga percobaan total. Panggilan yang timeout tidak dicoba ulang, karena panggilan asli mungkin masih mendarat. Jika batch masih gagal, kesalahan dicatat, pesan `{ type: "system", subtype: "mirror_error" }` dipancarkan ke iterator, batch dijatuhkan, dan kueri berlanjut. Transkrip lokal sudah tahan lama di disk, jadi pemadaman penyimpanan tidak mengganggu agen atau kehilangan data secara lokal. Pantau `mirror_error` jika Anda perlu mendeteksi kehilangan data penyimpanan. Karena batch yang dicoba ulang dapat mengirimkan ulang entri yang sudah mendarat, deduplikasi berdasarkan `entry.uuid` dalam implementasi `append()` Anda.

<h3 id="getsessionmessages-returns-the-post-compaction-chain">
  `getSessionMessages` mengembalikan rantai post-compaction
</h3>

`getSessionMessages({ sessionStore })` mengembalikan rantai pesan tertaut yang akan dilihat agen pada resume. Setelah auto-compaction, giliran sebelumnya diganti dengan ringkasan, jadi sesi yang penyimpanannya menyimpan 503 entri mentah dapat mengembalikan 18 pesan dari `getSessionMessages`. Untuk riwayat mentah lengkap, termasuk giliran pre-compaction dan entri metadata, panggil `store.load(key)` secara langsung.

<h3 id="forksession-is-not-a-byte-copy">
  `forkSession` bukan salinan byte
</h3>

`forkSession({ sessionStore })` membaca entri sumber, menulis ulang setiap bidang `sessionId` dan memetakan ulang UUID pesan, kemudian menambahkan entri yang ditransformasi di bawah kunci baru. Salinan tingkat adaptor atau shortcut `CopyObject` akan menghasilkan transkrip yang masih mereferensikan ID sesi lama, jadi SDK tidak menggunakannya.

<h3 id="subagent-transcripts">
  Transkrip subagent
</h3>

Transkrip subagent dicerminkan di bawah `subpath: "subagents/agent-<id>"`. `listSubagents({ sessionStore })` memerlukan adaptor untuk mengimplementasikan `listSubkeys`; `getSubagentMessages({ sessionStore })` menggunakannya ketika tersedia tetapi kembali ke subpath langsung ketika tidak ditentukan. Resume juga memanggil `listSubkeys` untuk memulihkan file subagent; tanpanya, hanya transkrip utama yang dimaterialisasi.

<h3 id="retention">
  Retensi
</h3>

SDK tidak pernah menghapus dari penyimpanan Anda sendiri. Retensi adalah tanggung jawab adaptor: implementasikan TTL, kebijakan lifecycle S3, atau pembersihan terjadwal sesuai dengan persyaratan kepatuhan Anda. Transkrip lokal di bawah `CLAUDE_CONFIG_DIR` disapu secara independen oleh pengaturan `cleanupPeriodDays`.

<h2 id="supported-on">
  Didukung pada
</h2>

Fungsi SDK berikut menerima opsi `sessionStore` dan beroperasi terhadap penyimpanan daripada sistem file lokal ketika disediakan:

* [`query()`](/docs/id/agent-sdk/typescript#query)
* [`startup()`](/docs/id/agent-sdk/typescript#startup)
* [`listSessions()`](/docs/id/agent-sdk/typescript#listsessions)
* [`getSessionInfo()`](/docs/id/agent-sdk/typescript#getsessioninfo)
* [`getSessionMessages()`](/docs/id/agent-sdk/typescript#getsessionmessages)
* [`renameSession()`](/docs/id/agent-sdk/typescript#renamesession)
* [`tagSession()`](/docs/id/agent-sdk/typescript#tagsession)
* [`deleteSession()`](/docs/id/agent-sdk/typescript)
* [`forkSession()`](/docs/id/agent-sdk/typescript)
* [`listSubagents()`](/docs/id/agent-sdk/typescript)
* [`getSubagentMessages()`](/docs/id/agent-sdk/typescript)

<h2 id="related-resources">
  Sumber daya terkait
</h2>

* [Bekerja dengan sesi](/docs/id/agent-sdk/sessions): Lanjutkan, resume, dan fork tanpa penyimpanan kustom
* [Host SDK](/docs/id/agent-sdk/hosting): Pola penerapan untuk lingkungan multi-host
* [TypeScript `Options`](/docs/id/agent-sdk/typescript#options): Referensi opsi lengkap
* [`examples/session-stores/`](https://github.com/anthropics/claude-agent-sdk-typescript/tree/main/examples/session-stores): Adaptor referensi S3, Redis, dan Postgres yang dapat dijalankan
