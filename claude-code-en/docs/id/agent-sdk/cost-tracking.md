> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Lacak biaya dan penggunaan

> Pelajari cara melacak penggunaan token, memperkirakan biaya, dan mengonfigurasi prompt caching dengan Claude Agent SDK.

Claude Agent SDK menyediakan informasi penggunaan token yang terperinci untuk setiap interaksi dengan Claude. Panduan ini menjelaskan cara melacak penggunaan dengan benar dan memahami pelaporan biaya, terutama ketika menangani penggunaan alat paralel dan percakapan multi-langkah.

Untuk dokumentasi API lengkap, lihat [referensi SDK TypeScript](/docs/id/agent-sdk/typescript) dan [referensi SDK Python](/docs/id/agent-sdk/python).

<Warning>
  Bidang `total_cost_usd` dan `costUSD` adalah perkiraan sisi klien, bukan data penagihan yang berwenang. SDK menghitungnya secara lokal dari tabel harga yang disertakan pada waktu pembuatan, sehingga dapat menyimpang dari apa yang sebenarnya Anda ditagih ketika:

  * harga berubah
  * versi SDK yang diinstal tidak mengenali model
  * aturan penagihan berlaku yang tidak dapat dimodelkan oleh klien

  Gunakan bidang ini untuk wawasan pengembangan dan anggaran perkiraan. Untuk penagihan yang berwenang, gunakan [Usage and Cost API](https://platform.claude.com/docs/en/build-with-claude/usage-cost-api) atau halaman Penggunaan di [Claude Console](https://platform.claude.com/usage). Jangan menagih pengguna akhir atau memicu keputusan keuangan dari bidang ini.
</Warning>

<h2 id="understand-token-usage">
  Pahami penggunaan token
</h2>

SDK TypeScript dan Python mengekspos data penggunaan yang sama dengan nama bidang yang berbeda:

* **TypeScript** menyediakan rincian token per-langkah pada setiap pesan asisten (`message.message.id`, `message.message.usage`), biaya per-model melalui `modelUsage` pada pesan hasil, dan total kumulatif pada pesan hasil.
* **Python** menyediakan rincian token per-langkah pada setiap pesan asisten (`message.usage`, `message.message_id`), biaya per-model melalui `model_usage` pada pesan hasil, dan total yang terakumulasi pada pesan hasil (`total_cost_usd` dan dict `usage`).

Kedua SDK menggunakan model biaya yang sama dan mengekspos granularitas yang sama. Perbedaannya adalah dalam penamaan bidang dan di mana penggunaan per-langkah bersarang.

Pelacakan biaya bergantung pada pemahaman tentang bagaimana SDK membatasi data penggunaan:

* **Panggilan `query()`:** satu invokasi fungsi `query()` SDK. Satu panggilan dapat melibatkan beberapa langkah (Claude merespons, menggunakan alat, mendapatkan hasil, merespons lagi). Setiap panggilan menghasilkan satu pesan [`result`](/docs/id/agent-sdk/typescript#sdkresultmessage) di akhir.
* **Langkah:** satu siklus permintaan/respons dalam panggilan `query()`. Setiap langkah menghasilkan pesan asisten dengan penggunaan token.
* **Sesi:** serangkaian panggilan `query()` yang ditautkan oleh ID sesi (menggunakan opsi `resume`). Setiap panggilan `query()` dalam sesi melaporkan biayanya sendiri secara independen.

Diagram berikut menunjukkan aliran pesan dari satu panggilan `query()`, dengan penggunaan token dilaporkan pada setiap langkah dan perkiraan kumulatif di akhir:

<img src="https://mintcdn.com/claude-code/ikqp3_70mqIahteV/images/agent-sdk/message-usage-flow.svg?fit=max&auto=format&n=ikqp3_70mqIahteV&q=85&s=68497aee338e01cc745323af7aea378e" alt="Diagram menunjukkan query menghasilkan dua langkah pesan. Langkah 1 memiliki empat pesan asisten yang berbagi ID dan penggunaan yang sama (hitung sekali), Langkah 2 memiliki satu pesan asisten dengan ID baru, dan pesan hasil akhir menunjukkan total_cost_usd yang diperkirakan." width="760" height="520" data-path="images/agent-sdk/message-usage-flow.svg" />

<Steps>
  <Step title="Setiap langkah menghasilkan pesan asisten">
    Ketika Claude merespons, ia mengirim satu atau lebih pesan asisten. Di TypeScript, setiap pesan asisten berisi `BetaMessage` bersarang (diakses melalui `message.message`) dengan `id` dan objek [`usage`](https://platform.claude.com/docs/en/api/messages) dengan hitungan token (`input_tokens`, `output_tokens`). Di Python, dataclass `AssistantMessage` mengekspos data yang sama secara langsung melalui `message.usage` dan `message.message_id`. Ketika Claude menggunakan beberapa alat dalam satu giliran, semua pesan dalam giliran itu berbagi ID yang sama, jadi deduplikasi berdasarkan ID untuk menghindari penghitungan ganda.
  </Step>

  <Step title="Pesan hasil memberikan perkiraan kumulatif">
    Ketika panggilan `query()` selesai, SDK memancarkan pesan hasil dengan `total_cost_usd` dan `usage` kumulatif. Ini tersedia di TypeScript ([`SDKResultMessage`](/docs/id/agent-sdk/typescript#sdkresultmessage)) dan Python ([`ResultMessage`](/docs/id/agent-sdk/python#resultmessage)). Jika Anda membuat beberapa panggilan `query()` (misalnya, dalam sesi multi-giliran), setiap hasil hanya mencerminkan biaya panggilan individual itu. Jika Anda hanya membutuhkan total yang diperkirakan, Anda dapat mengabaikan penggunaan per-langkah dan membaca nilai tunggal ini.
  </Step>
</Steps>

<h2 id="get-the-total-cost-of-a-query">
  Dapatkan biaya total dari query
</h2>

Pesan hasil ([TypeScript](/docs/id/agent-sdk/typescript#sdkresultmessage), [Python](/docs/id/agent-sdk/python#resultmessage)) menandai akhir dari loop agen untuk panggilan `query()`. Ini mencakup `total_cost_usd`, biaya perkiraan kumulatif di semua langkah dalam panggilan itu. Ini berfungsi untuk hasil sukses dan kesalahan. Jika Anda menggunakan sesi untuk membuat beberapa panggilan `query()`, setiap hasil hanya mencerminkan biaya panggilan individual itu.

Tiga bidang tingkat hasil berbeda dalam apa yang mereka hitung ketika agen menelurkan [subagen](/docs/id/agent-sdk/subagents). Gunakan `modelUsage`, atau `model_usage` di Python, untuk akuntansi token seluruh pohon; bidang `usage` kurang menghitung segera setelah nesting terjadi.

| Bidang                       | Aktivitas subagen                                                                                                    |
| ---------------------------- | -------------------------------------------------------------------------------------------------------------------- |
| `usage`                      | Dikecualikan. Menghitung hanya loop agen tingkat atas, jadi token yang dikonsumsi di dalam subagen tidak ditambahkan |
| `total_cost_usd`             | Disertakan. Menghitung permintaan subagen bersama loop tingkat atas                                                  |
| `modelUsage` / `model_usage` | Disertakan. Menghitung permintaan subagen bersama loop tingkat atas, dipecah menurut model                           |

Contoh berikut mengulangi aliran pesan dari panggilan `query()` dan mencetak biaya total ketika pesan `result` tiba:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  try {
    for await (const message of query({ prompt: "Summarize this project" })) {
      if (message.type === "result") {
        console.log(`Total cost: $${message.total_cost_usd}`);
      }
    }
  } catch (error) {
    // A single-shot query() throws after yielding an error result. If the
    // failure was an error result, it still carried total_cost_usd and the
    // branch above has already run; connection or process failures yield
    // no result message.
    console.error(`Session ended with an error: ${error}`);
  }
  ```

  ```python Python theme={null}
  from claude_agent_sdk import query, ResultMessage
  import asyncio


  async def main():
      try:
          async for message in query(prompt="Summarize this project"):
              if isinstance(message, ResultMessage):
                  print(f"Total cost: ${message.total_cost_usd or 0}")
      except Exception as error:
          # A single-shot query() raises after yielding an error result. If the
          # failure was an error result, it still carried total_cost_usd and the
          # branch above has already run; connection or process failures yield
          # no result message.
          print(f"Session ended with an error: {error}")


  asyncio.run(main())
  ```
</CodeGroup>

<h2 id="track-per-step-and-per-model-usage">
  Lacak penggunaan per-langkah dan per-model
</h2>

Contoh di bagian ini menggunakan nama bidang TypeScript. Di Python, bidang yang setara adalah [`AssistantMessage.usage`](/docs/id/agent-sdk/python#assistantmessage) dan `AssistantMessage.message_id` untuk penggunaan per-langkah, dan [`ResultMessage.model_usage`](/docs/id/agent-sdk/python#resultmessage) untuk rincian per-model.

<h3 id="track-per-step-usage">
  Lacak penggunaan per-langkah
</h3>

Setiap pesan asisten berisi `BetaMessage` bersarang (diakses melalui `message.message`) dengan `id` dan objek `usage` dengan hitungan token. Ketika Claude menggunakan alat secara paralel, beberapa pesan berbagi `id` yang sama dengan data penggunaan yang identik. Lacak ID mana yang sudah Anda hitung dan lewati duplikat untuk menghindari total yang meningkat.

<Warning>
  Panggilan alat paralel menghasilkan beberapa pesan asisten yang `BetaMessage` bersarangnya berbagi `id` yang sama dan penggunaan yang identik. Selalu deduplikasi berdasarkan ID untuk mendapatkan hitungan token per-langkah yang akurat.
</Warning>

Contoh berikut mengakumulasi token input dan output di semua langkah, menghitung setiap ID pesan unik hanya sekali:

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

const seenIds = new Set<string>();
let totalInputTokens = 0;
let totalOutputTokens = 0;

try {
  for await (const message of query({ prompt: "Summarize this project" })) {
    if (message.type === "assistant") {
      const msgId = message.message.id;

      // Parallel tool calls share the same ID, only count once
      if (!seenIds.has(msgId)) {
        seenIds.add(msgId);
        totalInputTokens += message.message.usage.input_tokens;
        totalOutputTokens += message.message.usage.output_tokens;
      }
    }
  }
} catch (error) {
  // A single-shot query() throws after yielding an error result, so the
  // totals below still reflect the steps that ran before the failure.
  console.error(`Session ended with an error: ${error}`);
}

console.log(`Steps: ${seenIds.size}`);
console.log(`Input tokens: ${totalInputTokens}`);
console.log(`Output tokens: ${totalOutputTokens}`);
```

<h3 id="break-down-usage-per-model">
  Rincian penggunaan per model
</h3>

Pesan hasil mencakup [`modelUsage`](/docs/id/agent-sdk/typescript#modelusage), peta nama model ke hitungan token per-model dan biaya. Ini berguna ketika Anda menjalankan beberapa model (misalnya, Haiku untuk subagen dan Opus untuk agen utama) dan ingin melihat ke mana token pergi.

Contoh berikut menjalankan query dan mencetak rincian biaya dan token untuk setiap model yang digunakan:

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

try {
  for await (const message of query({ prompt: "Summarize this project" })) {
    if (message.type !== "result") continue;

    for (const [modelName, usage] of Object.entries(message.modelUsage)) {
      console.log(`${modelName}: $${usage.costUSD.toFixed(4)}`);
      console.log(`  Input tokens: ${usage.inputTokens}`);
      console.log(`  Output tokens: ${usage.outputTokens}`);
      console.log(`  Cache read: ${usage.cacheReadInputTokens}`);
      console.log(`  Cache creation: ${usage.cacheCreationInputTokens}`);
    }
  }
} catch (error) {
  // A single-shot query() throws after yielding an error result. If the
  // failure was an error result, the per-model breakdown above has already
  // printed; connection or process failures yield no result message.
  console.error(`Session ended with an error: ${error}`);
}
```

<h2 id="accumulate-costs-across-multiple-calls">
  Akumulasi biaya di beberapa panggilan
</h2>

Setiap panggilan `query()` mengembalikan `total_cost_usd`-nya sendiri. SDK tidak menyediakan total tingkat sesi, jadi jika aplikasi Anda membuat beberapa panggilan `query()` (misalnya, dalam sesi multi-giliran atau di seluruh pengguna yang berbeda), akumulasi total sendiri.

Contoh berikut menjalankan dua panggilan `query()` secara berurutan, menambahkan `total_cost_usd` setiap panggilan ke total yang berjalan, dan mencetak biaya per-panggilan dan gabungan:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // Track cumulative cost across multiple query() calls
  let totalSpend = 0;

  const prompts = [
    "Read the files in src/ and summarize the architecture",
    "List all exported functions in src/auth.ts"
  ];

  for (const prompt of prompts) {
    try {
      for await (const message of query({ prompt })) {
        if (message.type === "result") {
          totalSpend += message.total_cost_usd;
          console.log(`This call: $${message.total_cost_usd}`);
        }
      }
    } catch (error) {
      // A single-shot query() throws after yielding an error result. If the
      // failure was an error result, this call's cost was already counted;
      // connection or process failures yield no result message. Continue
      // with the next prompt.
      console.error(`Call failed: ${error}`);
    }
  }

  console.log(`Total spend: $${totalSpend.toFixed(4)}`);
  ```

  ```python Python theme={null}
  from claude_agent_sdk import query, ResultMessage
  import asyncio


  async def main():
      # Track cumulative cost across multiple query() calls
      total_spend = 0.0

      prompts = [
          "Read the files in src/ and summarize the architecture",
          "List all exported functions in src/auth.ts",
      ]

      for prompt in prompts:
          try:
              async for message in query(prompt=prompt):
                  if isinstance(message, ResultMessage):
                      cost = message.total_cost_usd or 0
                      total_spend += cost
                      print(f"This call: ${cost}")
          except Exception as error:
              # A single-shot query() raises after yielding an error result. If
              # the failure was an error result, this call's cost was already
              # counted; connection or process failures yield no result message.
              # Continue with the next prompt.
              print(f"Call failed: {error}")

      print(f"Total spend: ${total_spend:.4f}")


  asyncio.run(main())
  ```
</CodeGroup>

<h2 id="handle-errors-caching-and-token-discrepancies">
  Tangani kesalahan, caching, dan perbedaan token
</h2>

Untuk pelacakan biaya yang akurat, pertimbangkan percakapan yang gagal, harga token cache, dan ketidakkonsistenan pelaporan yang sesekali.

<h3 id="resolve-output-token-discrepancies">
  Selesaikan perbedaan token output
</h3>

Dalam kasus yang jarang terjadi, Anda mungkin mengamati nilai `output_tokens` yang berbeda untuk pesan dengan ID yang sama. Ketika ini terjadi:

1. **Gunakan nilai tertinggi:** pesan terakhir dalam grup biasanya berisi total yang akurat.
2. **Lebih suka pesan hasil:** `total_cost_usd` dalam pesan hasil mencerminkan perkiraan terakumulasi SDK di semua langkah, sehingga lebih dapat diandalkan daripada menjumlahkan nilai per-langkah sendiri. Ini masih merupakan perkiraan dan mungkin berbeda dari tagihan aktual Anda.
3. **Laporkan ketidakkonsistenan:** ajukan masalah di [repositori GitHub Claude Code](https://github.com/anthropics/claude-code/issues).

<h3 id="track-costs-on-failed-conversations">
  Lacak biaya pada percakapan yang gagal
</h3>

Pesan hasil sukses dan kesalahan keduanya mencakup `usage` dan `total_cost_usd`. Jika percakapan gagal di tengah jalan, Anda masih mengonsumsi token hingga titik kegagalan. Selalu baca data biaya dari pesan hasil terlepas dari `subtype`-nya.

<h3 id="track-cache-tokens">
  Lacak token cache
</h3>

Agent SDK secara otomatis menggunakan [prompt caching](https://platform.claude.com/docs/en/build-with-claude/prompt-caching) untuk mengurangi biaya pada konten berulang. Anda tidak perlu mengonfigurasi caching sendiri. Objek penggunaan mencakup dua bidang tambahan untuk pelacakan cache:

* `cache_creation_input_tokens`: token yang digunakan untuk membuat entri cache baru (ditagih dengan tarif lebih tinggi daripada token input standar).
* `cache_read_input_tokens`: token yang dibaca dari entri cache yang ada (ditagih dengan tarif yang dikurangi).

Lacak ini secara terpisah dari `input_tokens` untuk memahami penghematan caching. Di TypeScript, bidang ini diketik pada objek [`Usage`](/docs/id/agent-sdk/typescript#usage). Di Python, mereka muncul sebagai kunci dalam dict [`ResultMessage.usage`](/docs/id/agent-sdk/python#resultmessage) (misalnya, `message.usage.get("cache_read_input_tokens", 0)`).

<h3 id="extend-the-prompt-cache-ttl-to-one-hour">
  Perpanjang TTL cache prompt ke satu jam
</h3>

Entri cache yang ditulis oleh SDK menggunakan TTL 5 menit secara default ketika Anda mengautentikasi dengan kunci API atau menjalankan di Amazon Bedrock, Google Cloud's Agent Platform, atau Microsoft Foundry. Jika beban kerja Anda menjalankan banyak sesi pendek terhadap prompt sistem dan konteks yang sama dengan celah lebih lama dari 5 menit di antara mereka, cache kedaluwarsa di antara sesi dan setiap sesi baru membayar harga input penuh.

Untuk meminta TTL 1 jam pada penulisan cache, atur variabel lingkungan [`ENABLE_PROMPT_CACHING_1H`](/docs/id/env-vars). Anda dapat mengekspornya di lingkungan shell atau kontainer Anda, atau meneruskannya melalui `options.env`.

Contoh berikut mengaktifkan TTL 1 jam untuk agen yang berjalan di Amazon Bedrock:

<CodeGroup>
  ```python Python theme={null}
  from claude_agent_sdk import ClaudeAgentOptions, query
  import asyncio


  async def main():
      options = ClaudeAgentOptions(
          env={
              "CLAUDE_CODE_USE_BEDROCK": "1",
              "ENABLE_PROMPT_CACHING_1H": "1",
          },
      )

      async for message in query(prompt="Summarize this project", options=options):
          print(message)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const options = {
    env: {
      ...process.env,
      CLAUDE_CODE_USE_BEDROCK: "1",
      ENABLE_PROMPT_CACHING_1H: "1",
    },
  };

  for await (const message of query({ prompt: "Summarize this project", options })) {
    console.log(message);
  }
  ```
</CodeGroup>

Penulisan cache dengan TTL 1 jam ditagih dengan tarif lebih tinggi daripada penulisan 5 menit, jadi mengaktifkan ini menukar biaya penulisan lebih tinggi untuk lebih banyak pembacaan cache. Lihat [harga prompt caching](https://platform.claude.com/docs/en/build-with-claude/prompt-caching) untuk detail. Pengguna langganan Claude sudah menerima TTL 1 jam secara otomatis dan tidak perlu mengatur variabel ini.

<h2 id="related-documentation">
  Dokumentasi terkait
</h2>

* [Referensi SDK TypeScript](/docs/id/agent-sdk/typescript) - Dokumentasi API lengkap
* [Ikhtisar SDK](/docs/id/agent-sdk/overview) - Memulai dengan SDK
* [Izin SDK](/docs/id/agent-sdk/permissions) - Mengelola izin alat
