> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Simpan sesi ke penyimpanan eksternal

> Cerminkan transkrip sesi Agent SDK ke object store, key-value store, atau database Anda sendiri sehingga host lain dapat melanjutkan sesi Anda.

Secara default, SDK menulis transkrip sesi ke file JSONL di bawah `~/.claude/projects/` pada sistem file lokal. Adaptor `SessionStore` memungkinkan Anda mencerminkan transkrip tersebut ke backend Anda sendiri, seperti object store, key-value store, atau database, sehingga sesi yang dibuat di satu host dapat dilanjutkan di host lain yang menjalankan dari direktori kerja yang cocok.

Alasan umum untuk menggunakan session store:

* **Penerapan multi-host.** Fungsi serverless, pekerja yang diskalakan otomatis, dan runner CI tidak berbagi sistem file. Penyimpanan bersama memungkinkan replika melanjutkan sesi satu sama lain.
* **Daya tahan.** Kontainer lokal bersifat sementara. Penyimpanan eksternal bertahan melalui restart dan redeploy.
* **Kepatuhan dan audit.** Simpan transkrip dalam penyimpanan yang sudah Anda kelola, dengan aturan retensi, enkripsi, dan kontrol akses Anda sendiri.

<h2 id="the-sessionstore-interface">
  Antarmuka `SessionStore`
</h2>

`SessionStore` adalah objek dengan dua metode yang diperlukan, `append` dan `load`, serta empat metode opsional. SDK memanggil `append` untuk menulis entri transkrip selama kueri dan `load` untuk membacanya kembali untuk resume.

<CodeGroup>
  ```typescript TypeScript theme={null}
  // Exported from @anthropic-ai/claude-agent-sdk as
  // SessionStore, SessionKey, SessionStoreEntry, SessionSummaryEntry.

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
    listSessionSummaries?(projectKey: string): Promise<SessionSummaryEntry[]>;
    delete?(key: SessionKey): Promise<void>;
    listSubkeys?(key: {
      projectKey: string;
      sessionId: string;
    }): Promise<string[]>;
  };

  type SessionSummaryEntry = {
    sessionId: string;
    mtime: number;
    data: Record<string, unknown>;
  };
  ```

  ```python Python theme={null}
  # Exported from claude_agent_sdk as
  # SessionStore, SessionKey, SessionStoreEntry, SessionSummaryEntry.

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
      async def list_session_summaries(
          self, project_key: str
      ) -> list[SessionSummaryEntry]: ...
      async def delete(self, key: SessionKey) -> None: ...
      async def list_subkeys(self, key: SessionListSubkeysKey) -> list[str]: ...

  class SessionSummaryEntry(TypedDict):
      session_id: str
      mtime: int
      data: dict[str, Any]
  ```
</CodeGroup>

`SessionKey` mengatasi satu transkrip. `projectKey` adalah pengkodean stabil dan aman sistem file dari direktori kerja, `sessionId` adalah UUID sesi, dan `subpath` diatur ketika entri milik transkrip subagent atau file sidecar daripada percakapan utama.

Karena `projectKey` mengkodekan direktori kerja, resume atau lanjutkan dari toko dari direktori kerja yang cocok dengan run asli. Di TypeScript, jika Anda menetapkan [`CLAUDE_CODE_PROJECT_DIR_NAME`](/docs/id/sessions#name-the-project-directory-yourself) di samping `CLAUDE_CONFIG_DIR` dalam opsi [`env`](/docs/id/agent-sdk/typescript#options) kueri, SDK menentukan kunci entri kueri itu, dan pencarian `resume` dan `continue` miliknya, dengan nama itu sebagai gantinya. Karena pembantu mandiri seperti `listSessions` dan `deleteSession` tidak mengambil `env` dan membaca lingkungan proses, atur `CLAUDE_CONFIG_DIR` dan nama yang sama di lingkungan proses host juga. Memerlukan Agent SDK v0.3.234 atau lebih baru.

Perlakukan `subpath` sebagai sufiks kunci yang tidak transparan; ini mengikuti tata letak on-disk, misalnya `subagents/agent-<id>`. Ketika `subpath` tidak ditentukan, kunci merujuk ke transkrip utama.

| Metode                 | Diperlukan | Dipanggil ketika                                                                                                                                                                                                                                                                                                                   |
| :--------------------- | :--------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `append`               | Ya         | Setelah setiap batch entri transkrip ditulis secara lokal. Entri adalah objek yang aman JSON, satu per baris dalam JSONL lokal.                                                                                                                                                                                                    |
| `load`                 | Ya         | Sebelum subprocess spawn ketika `resume` diatur atau `continue: true` menyelesaikan sesi toko terbaru, dan sekali per sesi ketika listing kembali dari `listSessionSummaries`. Kembalikan `null` jika sesi tidak dikenal.                                                                                                          |
| `listSessions`         | Tidak      | Oleh `listSessions({ sessionStore })` dan oleh `query()`/`startup()` dengan `continue: true`. Jika tidak ditentukan, `continue: true` melempar, dan `listSessions({ sessionStore })` melempar kecuali `listSessionSummaries` diimplementasikan.                                                                                    |
| `listSessionSummaries` | Tidak      | Oleh `listSessions({ sessionStore })` untuk membaca metadata untuk semua sesi dalam satu panggilan. Pertahankan ringkasan di dalam `append`. Jika tidak ditentukan, listing kembali ke `listSessions` ditambah per-sesi `load`.                                                                                                    |
| `delete`               | Tidak      | Oleh `deleteSession({ sessionStore })`. Menghapus kunci utama (tanpa `subpath`) harus cascade ke semua subkey untuk sesi itu dan juga menghapus entri ringkasan sesi, sehingga sesi yang dihapus berhenti muncul di `listSessionSummaries`. Jika tidak ditentukan, penghapusan adalah no-op, yang cocok untuk backend append-only. |
| `listSubkeys`          | Tidak      | Selama resume, untuk menemukan transkrip subagent. Jika tidak ditentukan, hanya transkrip utama yang dipulihkan.                                                                                                                                                                                                                   |

Dalam `SessionSummaryEntry`, `mtime` adalah waktu penulisan penyimpanan sidecar dan harus berbagi sumber jam dengan nilai `mtime` yang dikembalikan `listSessions`. `data` adalah status SDK-owned yang tidak transparan; pertahankan secara verbatim tanpa menginterpretasinya.

Bangun entri dengan memanggil pembantu `foldSessionSummary` yang diekspor, `fold_session_summary` di Python, pada setiap batch di dalam `append`. Lewati batch yang kuncinya memiliki `subpath`; transkrip subagent tidak boleh berkontribusi pada ringkasan sesi utama. Fold tidak pernah menetapkan `mtime`: cap pada waktu persist, melalui argumen `options.mtime` di TypeScript atau dengan menimpa field pada entri yang dikembalikan di Python. Panggilan `append` bersamaan untuk sesi yang sama dapat race pada sidecar, jadi serialisasi read-fold-write dengan transaksi, compare-and-swap, atau per-session lock; fold itu sendiri adalah pure.

Untuk apa yang dilakukan SDK dengan transkrip `load` yang dikembalikan, lihat [Resume dari toko](#resume-from-the-store).

<h2 id="quick-start">
  Mulai cepat
</h2>

SDK mengirimkan `InMemorySessionStore` untuk pengembangan dan pengujian. Contoh di bawah menjalankan kueri dengan penyimpanan yang terpasang, menangkap ID sesi dari pesan hasil, kemudian melanjutkan dari penyimpanan dalam panggilan `query()` kedua. Panggilan kedua melewatkan instance penyimpanan yang sama ditambah `resume`, sehingga SDK memuat transkrip dari penyimpanan daripada sistem file lokal:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query, InMemorySessionStore } from "@anthropic-ai/claude-agent-sdk";

  const store = new InMemorySessionStore();

  let sessionId: string | undefined;
  try {
    for await (const message of query({
      prompt: "List the TypeScript files under src/",
      options: { sessionStore: store },
    })) {
      if (message.type === "result") {
        sessionId = message.session_id;
      }
    }
  } catch (error) {
    // A single-shot query() throws after yielding an error result. If the
    // failure was an error result, sessionId was already captured by the loop
    // above; connection or process failures yield no result message.
    console.error(`Session ended with an error: ${error}`);
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
      try:
          async for message in query(
              prompt="List the Python files under src/",
              options=ClaudeAgentOptions(session_store=store),
          ):
              if isinstance(message, ResultMessage):
                  session_id = message.session_id
      except Exception as error:
          # A single-shot query() raises after yielding an error result. If the
          # failure was an error result, session_id was already captured by the
          # loop above; connection or process failures yield no result message.
          print(f"Session ended with an error: {error}")

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

Implementasikan `append` dan `load` terhadap backend Anda. Tambahkan `listSessions`, `listSessionSummaries`, `delete`, dan `listSubkeys` jika Anda ingin `listSessions()`, pembacaan metadata satu panggilan, `deleteSession()`, dan subagent resume bekerja terhadap penyimpanan.

Entri yang dilewatkan ke `append` diketik sebagai `SessionStoreEntry` (objek `{ type: string; ... }`). Perlakukan mereka sebagai nilai yang aman JSON yang tidak transparan: simpan dalam urutan dan kembalikan dari `load` dalam urutan yang sama. `load` harus mengembalikan entri yang deep-equal dengan apa yang ditambahkan; serialisasi byte-equal tidak diperlukan, jadi backend yang mengurutkan ulang kunci objek, seperti tipe kolom JSON biner, tidak masalah.

<h2 id="reference-implementations">
  Implementasi referensi
</h2>

Kedua repositori SDK mencakup adaptor referensi yang dapat dijalankan di bawah [`examples/session-stores/`](https://github.com/anthropics/claude-agent-sdk-typescript/tree/main/examples/session-stores) dalam TypeScript dan [`examples/session_stores/`](https://github.com/anthropics/claude-agent-sdk-python/tree/main/examples/session_stores) dalam Python. Ada satu adaptor per jenis penyimpanan, dan masing-masing menunjukkan bagaimana `append` dan `load` memetakan ke jenis backend tersebut. Mereka tidak dipublikasikan sebagai paket; salin adaptor untuk jenis yang paling dekat dengan backend Anda ke dalam proyek Anda, instal klien backend Anda, dan sesuaikan.

| Jenis penyimpanan                       | Model penyimpanan                                                                                                         | Adaptor contoh                                                                                                                                                                                                                                             |
| :-------------------------------------- | :------------------------------------------------------------------------------------------------------------------------ | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Object store                            | Satu file bagian per `append()`; `load()` mencantumkan bagian-bagian, mengurutkannya, dan menggabungkannya.               | S3 ([TypeScript](https://github.com/anthropics/claude-agent-sdk-typescript/tree/main/examples/session-stores/s3), [Python](https://github.com/anthropics/claude-agent-sdk-python/blob/main/examples/session_stores/s3_session_store.py))                   |
| Key-value store                         | Satu daftar per transkrip yang `append()` dorong ke dan `load()` baca dalam rentang, ditambah indeks sesi yang diurutkan. | Redis ([TypeScript](https://github.com/anthropics/claude-agent-sdk-typescript/tree/main/examples/session-stores/redis), [Python](https://github.com/anthropics/claude-agent-sdk-python/blob/main/examples/session_stores/redis_session_store.py))          |
| Relational database atau document store | Satu baris atau dokumen per entri, disimpan sebagai JSON dan diurutkan berdasarkan kunci yang ditetapkan saat penyisipan. | Postgres ([TypeScript](https://github.com/anthropics/claude-agent-sdk-typescript/tree/main/examples/session-stores/postgres), [Python](https://github.com/anthropics/claude-agent-sdk-python/blob/main/examples/session_stores/postgres_session_store.py)) |

Setiap adaptor mengambil instance klien yang telah dikonfigurasi sebelumnya, sehingga Anda mengontrol kredensial, TLS, region, dan pooling. Contoh berikut menghubungkan adaptor object-store ke `query()` dan kemudian melanjutkan darinya di host lain:

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

Di TypeScript, salin [`shared/conformance.ts`](https://github.com/anthropics/claude-agent-sdk-typescript/blob/main/examples/session-stores/shared/conformance.ts) dari direktori contoh ke dalam suite pengujian Anda. Di Python, suite dikirimkan dalam paket. Untuk menjalankannya dengan pytest, yang bukan merupakan dependensi SDK, instal pytest terlebih dahulu:

```bash theme={null}
pip install pytest
```

Kemudian teruskan adaptor Anda ke suite dalam file pengujian sebagai factory tanpa argumen, yang `run_session_store_conformance` panggil sekali per kontrak untuk membangun toko yang segar:

```python Python theme={null}
import pytest
from claude_agent_sdk.testing import run_session_store_conformance


@pytest.mark.anyio
async def test_my_store_conformance():
    await run_session_store_conformance(MyRedisStore)
```

Melewatkan kelas `MyRedisStore` itu sendiri, seperti yang dilakukan contoh ini, berfungsi ketika konstruktor tidak mengambil argumen. Untuk adaptor yang mengambil klien yang telah dikonfigurasi sebelumnya, teruskan lambda yang membangun toko sebagai gantinya. Karena kontrak menggunakan kembali kunci sesi yang sama, setiap toko yang dikembalikan factory harus dimulai dengan penyimpanan kosong, jadi buat lambda menyediakan penyimpanan backing terisolasi per panggilan, seperti fake in-memory yang segar, prefix kunci unik, atau database pengujian baru.

<h2 id="behavior-notes">
  Catatan perilaku
</h2>

<h3 id="dual-write-architecture">
  Arsitektur dual-write
</h3>

Subprocess Claude Code selalu menulis setiap batch entri transkrip ke disk lokal terlebih dahulu, dan SDK kemudian meneruskan batch yang sama ke `append()` penyimpanan Anda, sehingga penyimpanan adalah cerminan dari transkrip lokal daripada pengganti untuknya. Salinan mana yang bertahan dari run tergantung pada bagaimana run dimulai:

* **Sesi segar, atau resume ketika penyimpanan tidak memiliki apa pun untuk sesi**: transkrip lokal di bawah direktori konfigurasi Anda bertahan dari run, dan penyimpanan menerima salinan.
* **Run [dilanjutkan dari penyimpanan](#resume-from-the-store)**: salinan lokal dihapus di akhir run, sehingga penyimpanan menyimpan satu-satunya salinan yang tahan lama.

Jika Anda tidak ingin sesi segar meninggalkan transkrip di disk lokal, atur `CLAUDE_CONFIG_DIR` ke direktori temp di `options.env`. Run yang dilanjutkan dari penyimpanan sudah menghapus salinan lokalnya, jadi tidak memerlukan pengaturan seperti itu. Di TypeScript, sebarkan `process.env` ke `env` juga, karena [opsi `env`](/docs/id/agent-sdk/typescript#options) menggantikan lingkungan subprocess.

Jika aplikasi Anda masuk melalui file di direktori konfigurasi, seperti kredensial OAuth atau `apiKeyHelper` di `settings.json` pengguna Anda, salin file-file tersebut ke direktori temp terlebih dahulu, atau atur `ANTHROPIC_API_KEY` di `env` sebagai gantinya. Jika tidak, run gagal dengan `Not logged in`.

Dua opsi bertentangan dengan cerminan, dan SDK melempar pada startup jika Anda menggabungkan salah satu dengan penyimpanan:

* **`persistSession: false`** di TypeScript: mematikan penulisan lokal yang dibangun cerminan. Python SDK tidak memiliki opsi yang setara.
* **File checkpointing**, `enableFileCheckpointing` di TypeScript atau `enable_file_checkpointing` di Python: menulis cadangan file langsung ke disk lokal, dan SDK tidak mencerminkannya ke penyimpanan.

<h3 id="resume-from-the-store">
  Resume dari penyimpanan
</h3>

Ketika Anda melewatkan `resume`, atau `continue: true` di TypeScript atau `continue_conversation=True` di Python, bersama dengan penyimpanan, SDK meminta transkrip dari penyimpanan sebelum ia menelurkan subprocess:

* **`resume`**: SDK meminta sesi yang ID-nya Anda lewatkan.
* **`continue: true`** atau **`continue_conversation=True`**: SDK meminta sesi terbaru penyimpanan.

Ketika penyimpanan mengembalikan transkrip, SDK menulisnya ke direktori konfigurasi sementara, menjalankan subprocess dengan `CLAUDE_CONFIG_DIR` menunjuk ke sana, dan menghapus direktori ketika run berakhir. Transkrip lokal yang run itu tulis dihapus bersama dengannya, itulah mengapa penyimpanan menyimpan satu-satunya salinan yang tahan lama di jalur ini.

SDK juga menyemai direktori sementara dengan file dari direktori konfigurasi nyata Anda. Apa yang disalinnya berbeda menurut bahasa:

* **TypeScript**: kredensial, `.claude.json`, dan `settings.json` pengguna Anda. Dari `settings.json` ia menghilangkan kunci yang berperilaku buruk di bawah direktori konfigurasi sementara: `enabledPlugins`, `extraKnownMarketplaces`, alias [`additionalMarketplaces`](/docs/id/settings-reference#extraknownmarketplaces)-nya, dan `CLAUDE_CONFIG_DIR` apa pun di blok `env` file. Sebelum Agent SDK v0.3.232, SDK tidak menghilangkan alias. Auth yang dikonfigurasi dalam pengaturan, seperti [`apiKeyHelper`](/docs/id/settings-reference#apikeyhelper), bekerja ketika Anda resume dari penyimpanan. Sebelum Agent SDK v0.3.222, TypeScript SDK hanya menyalin kredensial dan `.claude.json`.
* **Python**: kredensial dan `.claude.json` saja, jadi aplikasi yang mengautentikasi melalui `apiKeyHelper` di `settings.json` pengguna Anda gagal dengan `Not logged in` ketika resume dari penyimpanan. `apiKeyHelper` dalam pengaturan terkelola atau proyek masih bekerja, karena Claude Code membaca file-file tersebut dari lokasi yang tidak dipengaruhi oleh `CLAUDE_CONFIG_DIR`.

Ketika penyimpanan tidak memiliki apa pun untuk sesi, SDK berjalan di bawah direktori konfigurasi nyata Anda sebagai gantinya, dan hasilnya tergantung pada opsi mana yang Anda lewatkan:

* **`resume`**: kedua SDK melewatkan ID melalui ke subprocess, yang melanjutkan transkrip lokal persis seperti `resume` tanpa penyimpanan.
* **`continue: true`** di TypeScript: SDK memulai sesi segar.
* **`continue_conversation=True`** di Python: SDK melanjutkan dari sesi lokal terbaru.

<h3 id="mirror-writes-are-best-effort">
  Penulisan cerminan adalah best-effort
</h3>

Jika `append()` menolak, SDK mencoba ulang batch hingga dua kali lagi dengan backoff singkat, untuk maksimal tiga percobaan total. Panggilan yang timeout tidak dicoba ulang, karena panggilan asli mungkin masih mendarat. Jika batch masih gagal, SDK mencatat kesalahan, memancarkan pesan `{ type: "system", subtype: "mirror_error" }` ke iterator, menjatuhkan batch, dan melanjutkan kueri. Karena batch yang dicoba ulang dapat mengirimkan ulang entri yang sudah mendarat, deduplikasi berdasarkan `entry.uuid` dalam implementasi `append()` Anda.

Pemadaman penyimpanan tidak mengganggu agen, karena subprocess menulis lokal terlebih dahulu. Pantau `mirror_error` jika Anda perlu mendeteksi kehilangan data penyimpanan. Pada run [dilanjutkan dari penyimpanan](#resume-from-the-store), batch yang dijatuhkan tidak memiliki salinan yang bertahan setelah run berakhir.

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

SDK tidak pernah menghapus dari penyimpanan Anda sendiri. Retensi adalah tanggung jawab adaptor: gunakan mekanisme kedaluwarsa atau lifecycle backend Anda, atau jalankan pembersihan terjadwal, sesuai dengan persyaratan kepatuhan Anda.

Transkrip lokal di bawah `CLAUDE_CONFIG_DIR` disapu secara independen oleh pengaturan `cleanupPeriodDays`, mengikuti [aturan penyapuan retensi](/docs/id/claude-directory#cleaned-up-automatically). Run [dilanjutkan dari penyimpanan](#resume-from-the-store) tidak meninggalkan transkrip lokal, jadi untuk run tersebut retensi penyimpanan Anda adalah satu-satunya retensi yang ada.

<h2 id="supported-on">
  Didukung pada
</h2>

Fungsi SDK TypeScript berikut menerima opsi `sessionStore` dan beroperasi terhadap penyimpanan daripada sistem file lokal ketika disediakan:

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

Dalam SDK Python, atur `session_store` dalam [`ClaudeAgentOptions`](/docs/id/agent-sdk/python#claudeagentoptions) untuk menjalankan `query()` terhadap penyimpanan. Operasi yang tersisa masing-masing memiliki fungsi Python yang didukung penyimpanan yang mengambil penyimpanan sebagai argumen: `list_sessions_from_store()`, `get_session_info_from_store()`, `get_session_messages_from_store()`, `list_subagents_from_store()`, `get_subagent_messages_from_store()`, `rename_session_via_store()`, `tag_session_via_store()`, `delete_session_via_store()`, dan `fork_session_via_store()`. `startup()` tidak memiliki padanan Python. Fungsi mandiri yang didokumentasikan dalam [referensi SDK Python](/docs/id/agent-sdk/python#functions), seperti `list_sessions()`, membaca file sesi lokal.

<h2 id="related-resources">
  Sumber daya terkait
</h2>

* [Bekerja dengan sesi](/docs/id/agent-sdk/sessions): Lanjutkan, resume, dan fork tanpa penyimpanan kustom
* [Host SDK](/docs/id/agent-sdk/hosting): Pola penerapan untuk lingkungan multi-host
* [TypeScript `Options`](/docs/id/agent-sdk/typescript#options): Referensi opsi lengkap
* [Implementasi referensi](#reference-implementations): Adaptor contoh yang dapat dijalankan untuk penyimpanan objek, penyimpanan kunci-nilai, dan basis data, di kedua repositori SDK
