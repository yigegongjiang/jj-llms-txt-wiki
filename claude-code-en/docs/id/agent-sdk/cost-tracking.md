> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Lacak biaya dan penggunaan

> Pelajari cara melacak penggunaan token, memperkirakan biaya, dan mengonfigurasi prompt caching dengan Claude Agent SDK.

Claude Agent SDK menyediakan informasi penggunaan token yang terperinci untuk setiap interaksi dengan Claude. Panduan ini menjelaskan cara melacak penggunaan dengan benar dan memahami pelaporan biaya, terutama ketika menangani penggunaan alat paralel dan percakapan multi-langkah.

Untuk dokumentasi API lengkap, lihat [referensi SDK TypeScript](/docs/id/agent-sdk/typescript) dan [referensi SDK Python](/docs/id/agent-sdk/python).

<Warning>
  Bidang `total_cost_usd` dan `costUSD` adalah perkiraan sisi klien, bukan data penagihan yang berwenang. SDK menghitungnya secara lokal dari tabel harga yang disertakan pada waktu pembuatan, kecuali jika tabel [`modelPricing`](/docs/id/settings-reference#modelpricing) berlaku. Mereka dapat menyimpang dari apa yang sebenarnya Anda tagih ketika:

  * harga berubah
  * versi SDK yang diinstal tidak mengenali model
  * aturan penagihan berlaku yang tidak dapat dimodelkan klien

  Satu aturan penagihan yang dimodelkan SDK adalah [penetapan harga residensi data](https://platform.claude.com/docs/en/about-claude/pricing#data-residency-pricing). Ketika respons `usage` melaporkan `inference_geo: "us"`, SDK mengalikan harga daftar token respons tersebut dengan 1,1. Biaya per-permintaan seperti pencarian web tidak dikalikan. Memerlukan TypeScript Agent SDK v0.3.239 atau lebih baru, atau Python Agent SDK v0.2.144 atau lebih baru.

  Gunakan bidang-bidang ini untuk wawasan pengembangan dan anggaran perkiraan. Untuk penagihan yang berwenang, gunakan [Usage and Cost API](https://platform.claude.com/docs/en/build-with-claude/usage-cost-api) atau halaman Penggunaan di [Claude Console](https://platform.claude.com/usage). Jangan tagih pengguna akhir atau picu keputusan keuangan dari bidang-bidang ini.
</Warning>

<h2 id="understand-token-usage">
  Pahami penggunaan token
</h2>

TypeScript dan Python SDK mengekspos data penggunaan yang sama dengan nama field yang berbeda:

* **TypeScript** menyediakan breakdown token per-step pada setiap pesan asisten (`message.message.id`, `message.message.usage`), biaya per-model melalui `modelUsage` pada pesan hasil, dan total kumulatif pada pesan hasil.
* **Python** menyediakan breakdown token per-step pada setiap pesan asisten sebagai `message.usage` dan `message.message_id`, biaya per-model melalui `model_usage` pada pesan hasil, dan total kumulatif pada pesan hasil sebagai `total_cost_usd`.

Kedua SDK menggunakan model biaya yang sama dan mengekspos granularitas yang sama. Perbedaannya adalah dalam penamaan field dan di mana penggunaan per-step bersarang.

Pelacakan biaya bergantung pada pemahaman tentang bagaimana SDK membatasi data penggunaan:

* **`query()` call:** satu invokasi dari fungsi `query()` SDK. Satu panggilan dapat melibatkan beberapa langkah: Claude merespons, menggunakan tools, mendapatkan hasil, dan merespons lagi. Setiap panggilan menghasilkan satu pesan [`result`](/docs/id/agent-sdk/typescript#sdkresultmessage) di akhir, kecuali dalam [mode input streaming](/docs/id/agent-sdk/streaming-vs-single-mode), di mana satu panggilan `query()` membawa beberapa giliran pengguna dan setiap giliran memancarkan pesan `result` miliknya sendiri.
* **Step:** satu siklus request/response dalam panggilan `query()`. Setiap langkah menghasilkan pesan asisten dengan penggunaan token.
* **Session:** serangkaian panggilan `query()` yang terhubung oleh ID sesi melalui opsi `resume`. Hasil panggilan yang dilanjutkan melaporkan pengeluaran seluruh sesi, bukan hanya panggilan itu sendiri. Lihat [Accumulate costs across multiple calls](#accumulate-costs-across-multiple-calls) untuk cara total dibawa ke depan.

Diagram berikut menunjukkan aliran pesan dari satu panggilan `query()`, dengan penggunaan token dilaporkan pada setiap langkah dan perkiraan kumulatif di akhir:

<img src="https://mintcdn.com/claude-code/ikqp3_70mqIahteV/images/agent-sdk/message-usage-flow.svg?fit=max&auto=format&n=ikqp3_70mqIahteV&q=85&s=68497aee338e01cc745323af7aea378e" className="dark:hidden" alt="Diagram showing a query producing two steps of messages. Step 1 has four assistant messages sharing the same ID and usage (count once), Step 2 has one assistant message with a new ID, and the final result message shows the estimated total_cost_usd." width="760" height="520" data-path="images/agent-sdk/message-usage-flow.svg" />

<img src="https://mintcdn.com/claude-code/_xqph1dUOslCOwsj/images/agent-sdk/message-usage-flow-dark.svg?fit=max&auto=format&n=_xqph1dUOslCOwsj&q=85&s=8ea95085abc0a6b7f55ecef498bd4d14" className="hidden dark:block" alt="Diagram showing a query producing two steps of messages. Step 1 has four assistant messages sharing the same ID and usage (count once), Step 2 has one assistant message with a new ID, and the final result message shows the estimated total_cost_usd." width="760" height="520" data-path="images/agent-sdk/message-usage-flow-dark.svg" />

<Steps>
  <Step title="Setiap langkah menghasilkan pesan asisten">
    Ketika Claude merespons, ia mengirimkan satu atau lebih pesan asisten. Di TypeScript, setiap pesan asisten berisi `BetaMessage` bersarang (diakses melalui `message.message`) dengan `id` dan objek [`usage`](https://platform.claude.com/docs/en/api/messages) dengan hitungan token (`input_tokens`, `output_tokens`). Di Python, dataclass `AssistantMessage` mengekspos data yang sama secara langsung melalui `message.usage` dan `message.message_id`. Ketika Claude menggunakan beberapa tools dalam satu giliran, semua pesan dalam giliran itu berbagi ID yang sama, jadi deduplikasi berdasarkan ID untuk menghindari penghitungan ganda.
  </Step>

  <Step title="Pesan hasil memberikan perkiraan kumulatif">
    Ketika panggilan `query()` selesai, SDK memancarkan pesan hasil dengan `total_cost_usd` dan `usage` kumulatif, diketik sebagai [`SDKResultMessage`](/docs/id/agent-sdk/typescript#sdkresultmessage) di TypeScript dan [`ResultMessage`](/docs/id/agent-sdk/python#resultmessage) di Python. Jika Anda hanya membutuhkan total perkiraan, Anda dapat mengabaikan penggunaan per-step dan membaca nilai tunggal ini.

    Jika Anda membuat beberapa panggilan `query()` independen, setiap hasil hanya mencerminkan biaya panggilan individual itu. Panggilan yang melanjutkan sesi juga menghitung pengeluaran awal sesi.

    Dalam mode input streaming, setiap giliran memancarkan pesan hasil miliknya sendiri. Lihat [Track costs in streaming input mode](#track-costs-in-streaming-input-mode) untuk cara membaca total panggilan dalam mode itu.
  </Step>
</Steps>

<h2 id="track-costs-in-streaming-input-mode">
  Lacak biaya dalam mode input streaming
</h2>

Dalam [mode input streaming](/docs/id/agent-sdk/streaming-vs-single-mode), satu panggilan `query()` membawa beberapa giliran pengguna dan setiap giliran memancarkan pesan hasilnya sendiri. Bidang hasil berbeda dalam cakupan:

* **`usage`**: mencakup hanya giliran itu, dan di dalamnya hanya loop agen utama, bukan subagen apa pun yang dijalankannya.
* **`total_cost_usd` dan `modelUsage`, atau `model_usage` dalam Python**: membawa total berjalan untuk seluruh panggilan sejauh ini, ditambah pengeluaran apa pun yang dipulihkan ketika panggilan melanjutkan sesi.

Dalam panggilan di mana aplikasi Anda tidak pernah mengirim `/clear`, `/reset`, atau `/new`, baca hasil terbaru untuk total panggilan daripada menjumlahkan hasil.

Total berjalan dimulai ulang setiap kali aplikasi Anda mengirim salah satu dari tiga perintah itu, dan di dalam panggilan `query()` tidak ada yang lain yang mengatur ulang mereka. Tiga hasil penting untuk akuntansi Anda:

* **Hasil giliran `/clear` itu sendiri**: mencakup hanya apa yang telah berjalan sejak pengaturan ulang, dan membawa `session_id` baru.
* **Setiap hasil kemudian**: terus menghitung dari pengaturan ulang itu.
* **Hasil terakhir sebelum setiap `/clear`**: menyimpan total untuk giliran sejak pengaturan ulang sebelumnya.

Untuk menghitung total seluruh panggilan, tambahkan hasil terakhir dari sebelum setiap `/clear` ke hasil akhir panggilan. Setiap hasil lainnya, termasuk hasil giliran `/clear` itu sendiri, digantikan oleh hasil yang lebih baru.

Dalam TypeScript, SDK juga memancarkan [`SDKConversationResetMessage`](/docs/id/agent-sdk/typescript#sdkconversationresetmessage) pada setiap pengaturan ulang, sehingga Anda dapat mendeteksi pengaturan ulang dari aliran. Dalam Python, SDK juga memancarkan `ConversationResetMessage`. Sebelum Python SDK v0.2.137, iterator Python menghilangkan pesan itu, jadi pada versi tersebut hitung pengaturan ulang sendiri dari giliran `/clear` yang dikirim aplikasi Anda.

`maxBudgetUsd` (TypeScript) atau `max_budget_usd` (Python) hanya menghitung pengeluaran panggilan itu sendiri: total yang dipulihkan dari sesi yang dilanjutkan tidak dihitung terhadapnya, dan `/clear` memulai anggaran ulang.

<h2 id="get-the-total-cost-of-a-query">
  Dapatkan total biaya dari sebuah query
</h2>

Pesan hasil, yang diketik sebagai [`SDKResultMessage`](/docs/id/agent-sdk/typescript#sdkresultmessage) di TypeScript dan [`ResultMessage`](/docs/id/agent-sdk/python#resultmessage) di Python, menandai akhir dari loop agen untuk panggilan `query()`. Ini mencakup `total_cost_usd`, biaya perkiraan kumulatif di semua langkah dalam panggilan tersebut. Sebuah panggilan yang melanjutkan sesi juga menghitung pengeluaran awal sesi. Dua peringatan berlaku ketika Anda membaca nilainya:

* Di Python bidang ini diketik sebagai opsional, jadi periksa bahwa itu bukan `None` sebelum Anda membacanya.
* Hasil sukses dan kesalahan keduanya membawanya, meskipun hasil akhir dari [kerusakan sesi](#recover-totals-after-a-session-crash) mungkin membawanya dengan nilai nol.

Dalam mode input streaming, baca total panggilan seperti yang dijelaskan dalam [Lacak biaya dalam mode input streaming](#track-costs-in-streaming-input-mode).

Tiga bidang tingkat hasil berbeda dalam apa yang mereka hitung ketika agen menelurkan [subagen](/docs/id/agent-sdk/subagents). Gunakan `modelUsage`, atau `model_usage` di Python, untuk akuntansi token seluruh pohon; bidang `usage` kurang menghitung segera setelah nesting terjadi.

| Bidang                       | Aktivitas Subagen                                                                                                    |
| ---------------------------- | -------------------------------------------------------------------------------------------------------------------- |
| `usage`                      | Dikecualikan. Menghitung hanya loop agen tingkat atas, jadi token yang dikonsumsi di dalam subagen tidak ditambahkan |
| `total_cost_usd`             | Disertakan. Menghitung permintaan subagen bersama loop tingkat atas                                                  |
| `modelUsage` / `model_usage` | Disertakan. Menghitung permintaan subagen bersama loop tingkat atas, dipecah menurut model                           |

Dalam [mode input pesan tunggal](/docs/id/agent-sdk/streaming-vs-single-mode#single-message-input), ketika subagen latar belakang masih berjalan di akhir giliran terakhir, Claude Code menunggu mereka, hingga batas yang dijelaskan dalam [tugas latar belakang saat keluar](/docs/id/headless#background-tasks-at-exit), sebelum memancarkan hasilnya. `total_cost_usd`, `duration_api_ms`, dan `modelUsage` hasil, atau `model_usage` di Python, mencakup pekerjaan yang dilakukan selama penantian tersebut.

Contoh berikut mengulangi aliran pesan dari panggilan `query()` dan mencetak total biaya ketika pesan `result` tiba:

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
          # failure was an error result, the branch above has already run;
          # connection or process failures yield no result message.
          print(f"Session ended with an error: {error}")


  asyncio.run(main())
  ```
</CodeGroup>

Untuk membatasi berapa banyak subagen dapat menambah `total_cost_usd`, atur [batas kedalaman, konkurensi, dan pengeluaran](/docs/id/agent-sdk/subagents#cap-subagent-depth-concurrency-and-spend) pada query.

<h2 id="track-per-step-and-per-model-usage">
  Lacak penggunaan per-langkah dan per-model
</h2>

Contoh-contoh di bagian ini menggunakan nama field TypeScript. Di Python, field yang setara adalah [`AssistantMessage.usage`](/docs/id/agent-sdk/python#assistantmessage) dan `AssistantMessage.message_id` untuk penggunaan per-langkah, dan [`ResultMessage.model_usage`](/docs/id/agent-sdk/python#resultmessage) untuk rincian per-model.

<h3 id="track-per-step-usage">
  Lacak penggunaan per-langkah
</h3>

Setiap pesan asisten berisi `BetaMessage` bersarang (diakses melalui `message.message`) dengan objek `id` dan `usage` yang berisi hitungan token. Ketika Claude menggunakan tools secara paralel, beberapa pesan berbagi `id` yang sama dengan data penggunaan yang identik. Lacak ID mana yang sudah Anda hitung dan lewati duplikat untuk menghindari total yang membengkak.

<Warning>
  Nilai per-langkah yang dideduplikasi akurat untuk token input dan cache. Per-langkah `output_tokens` adalah placeholder, jadi [baca output tokens dari pesan hasil](#read-output-tokens-from-the-result-message).
</Warning>

Contoh berikut mengakumulasi token input di semua langkah, menghitung setiap ID pesan loop utama yang unik hanya sekali dan melewati pesan subagen, serta membaca total output dari pesan hasil, yang mencakup loop utama:

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

const seenIds = new Set<string>();
let totalInputTokens = 0;
let resultOutputTokens = 0;

try {
  for await (const message of query({ prompt: "Summarize this project" })) {
    if (message.type === "assistant" && !message.parent_tool_use_id) {
      const msgId = message.message.id;

      // Parallel tool calls share the same ID, only count once
      if (!seenIds.has(msgId)) {
        seenIds.add(msgId);
        totalInputTokens += message.message.usage.input_tokens;
      }
    }
    if (message.type === "result") {
      // Per-step output_tokens is a placeholder; the result message
      // carries the accumulated output total.
      resultOutputTokens = message.usage.output_tokens;
    }
  }
} catch (error) {
  // A single-shot query() throws after yielding an error result, so the
  // input total below still reflects the steps that ran before the failure.
  console.error(`Session ended with an error: ${error}`);
}

console.log(`Steps: ${seenIds.size}`);
console.log(`Input tokens: ${totalInputTokens}`);
console.log(`Output tokens: ${resultOutputTokens}`);
```

<h3 id="break-down-usage-per-model">
  Rincian penggunaan per model
</h3>

Pesan hasil mencakup [`modelUsage`](/docs/id/agent-sdk/typescript#modelusage), peta nama model ke hitungan token per-model dan biaya. Ini berguna ketika Anda menjalankan beberapa model (misalnya, Haiku untuk subagen dan Opus untuk agen utama) dan ingin melihat ke mana token pergi.

Setiap `costBasis` entri mengatakan tabel harga mana yang menentukan harga permintaan terbaru model itu: `list` untuk harga daftar, `managed` untuk tabel [`modelPricing`](/docs/id/settings-reference#modelpricing), atau `unknown` ketika tidak ada yang cocok dengan ID model. Field memerlukan Claude Code v2.1.246 atau lebih baru.

Contoh berikut menjalankan kueri dan mencetak rincian biaya dan token untuk setiap model yang digunakan:

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
  Akumulasi biaya di seluruh beberapa panggilan
</h2>

Setiap panggilan `query()` mengembalikan `total_cost_usd` pada hasilnya. Cara Anda menggabungkan nilainya tergantung pada apakah panggilan berbagi sesi:

* **Panggilan independen, tanpa opsi `resume` atau `continue`**: setiap hasil hanya mencakup panggilan miliknya sendiri, jadi tambahkan totalnya sendiri, seperti yang dilakukan contoh di bawah ini.
* **Panggilan yang melanjutkan sesi yang sama**: Claude Code menyimpan total sesi ke [transcript](/docs/id/sessions#where-transcripts-are-stored) ketika proses keluar secara normal dan memulihkannya ketika panggilan nanti melanjutkan atau memisahkan sesi. Setiap hasil sudah mencakup pengeluaran awal sesi. Baca hasil terbaru untuk total sesi; menjumlahkan hasil menghitung dua kali pengeluaran yang dipulihkan. Sebelum v2.1.277, sesi yang Anda lanjutkan melalui SDK atau `claude -p` memulai totalnya di nol, jadi hasil setiap panggilan hanya mencakup panggilan itu.

Dalam mode input streaming, baca total setiap panggilan seperti yang dijelaskan dalam [Track costs in streaming input mode](#track-costs-in-streaming-input-mode). Untuk panggilan yang berakhir dalam kerusakan, lihat [Recover totals after a session crash](#recover-totals-after-a-session-crash).

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

<h2 id="handle-errors-caching-and-output-token-counts">
  Menangani kesalahan, caching, dan jumlah token output
</h2>

Untuk pelacakan biaya yang akurat, pertimbangkan jumlah output placeholder pada pesan asisten, token yang dikonsumsi percakapan yang gagal, dan harga token cache.

<h3 id="read-output-tokens-from-the-result-message">
  Baca token output dari pesan hasil
</h3>

Claude Code membangun setiap pesan asisten dari penggunaan yang dilaporkan API ketika respons dimulai, jadi `output_tokens` pesan hanya merupakan jumlah yang dilaporkan API pada `message_start`, sebelum respons dihasilkan. Satu respons API dapat menghasilkan beberapa pesan asisten, dan setiap satu membawa placeholder yang sama.

API melaporkan jumlah output nyata di akhir respons, dan Claude Code menambahkannya ke pesan hasil. Baca token output dari `usage` hasil, atau dari `modelUsage` untuk rincian per-model.

Untuk menonton jumlah output respons tumbuh saat streaming, atur `includePartialMessages`, atau `include_partial_messages` di Python, dan baca `usage` dari setiap acara stream `message_delta`, diketik sebagai [`SDKPartialAssistantMessage`](/docs/id/agent-sdk/typescript#sdkpartialassistantmessage) di TypeScript dan [`StreamEvent`](/docs/id/agent-sdk/python#streamevent) di Python.

<h3 id="track-costs-on-failed-conversations">
  Lacak biaya pada percakapan yang gagal
</h3>

Pesan hasil kesuksesan dan kesalahan keduanya mencakup `usage` dan `total_cost_usd`; di Python kedua bidang diketik sebagai opsional, jadi periksa bahwa mereka bukan `None` sebelum Anda membacanya.

Jika percakapan gagal di tengah jalan, Anda masih mengonsumsi token hingga titik kegagalan. Baca data biaya dari setiap pesan hasil, terlepas dari apakah `subtype`-nya adalah `success` atau salah satu subtipe kesalahan. Pada beberapa hasil kesalahan, `usage` melaporkan lebih sedikit daripada yang dihabiskan panggilan:

* **`error_during_execution` setelah [kerusakan sesi](#recover-totals-after-a-session-crash)**: setiap bidang biaya dapat dinolkan.
* **`error_max_budget_usd`**: `usage` menghilangkan respons yang melampaui anggaran, sementara `total_cost_usd` dan `modelUsage` menyertakannya.

Jika Anda memiliki pilihan, hitung dari `total_cost_usd` atau `modelUsage` daripada `usage`.

<h3 id="recover-totals-after-a-session-crash">
  Pulihkan total setelah kerusakan sesi
</h3>

Ketika proses Claude Code mogok, ia mengeluarkan hasil `error_during_execution` akhir dan keluar, dalam mode input single-shot dan streaming sama-sama. Hasil itu mungkin membawa `usage`, `total_cost_usd`, dan `modelUsage` yang dinolkan, jadi pulihkan total panggilan dari apa yang tiba sebelumnya. Langkah 1 memulihkan total penuh kapan pun hasil sebelumnya ada; fallback di langkah 2 memulihkan hanya token input dan cache loop utama.

1. Gunakan hasil giliran sebelum kerusakan. Dalam mode input streaming, ia menyimpan total berjalan yang dijelaskan dalam [Track costs in streaming input mode](#track-costs-in-streaming-input-mode). Lanjutkan ke langkah 2 sebagai gantinya ketika hasil itu tidak dapat membantu Anda:
   * Panggilan adalah single-shot, jadi tidak ada hasil sebelumnya.
   * Kerusakan terjadi pada giliran pertama.
   * Giliran sebelum kerusakan adalah `/clear` itu sendiri, jadi hasilnya hanya mencakup reset.
2. Jumlahkan `usage` pada pesan asisten sebagai gantinya, menghitung setiap respons API sekali, seperti yang dilakukan contoh [Track per-step usage](#track-per-step-usage). Dalam mode single-shot, jumlahkan semuanya; dalam mode input streaming, jumlahkan yang tiba setelah hasil terakhir. Ini memberi Anda token input dan cache loop utama. Penggunaan subagent tidak dapat dipulihkan dengan cara ini, begitu juga token output atau biaya USD, karena [per-step `output_tokens` adalah placeholder](#read-output-tokens-from-the-result-message).

<h3 id="track-cache-tokens">
  Lacak token cache
</h3>

Agent SDK secara otomatis menggunakan [prompt caching](https://platform.claude.com/docs/en/build-with-claude/prompt-caching) untuk mengurangi biaya pada konten berulang. Anda tidak perlu mengonfigurasi caching sendiri. Objek penggunaan mencakup dua bidang tambahan untuk pelacakan cache:

* `cache_creation_input_tokens`: token yang digunakan untuk membuat entri cache baru (dikenakan biaya pada tingkat lebih tinggi daripada token input standar).
* `cache_read_input_tokens`: token yang dibaca dari entri cache yang ada (dikenakan biaya pada tingkat berkurang).

Lacak ini secara terpisah dari `input_tokens` untuk memahami penghematan caching. Di TypeScript, bidang-bidang ini diketik pada objek [`Usage`](/docs/id/agent-sdk/typescript#usage). Di Python, mereka muncul sebagai kunci dalam dict [`ResultMessage.usage`](/docs/id/agent-sdk/python#resultmessage) (misalnya, `message.usage.get("cache_read_input_tokens", 0)`).

<h3 id="extend-the-prompt-cache-ttl-to-one-hour">
  Perpanjang TTL cache prompt ke satu jam
</h3>

Giliran Anda sendiri jatuh dalam [bucket TTL percakapan utama](/docs/id/prompt-caching#which-ttl-each-request-gets), bersama dengan pembantu yang Claude Code jalankan inline dengan mereka. Permintaan yang Claude Code buat di luar percakapan itu, seperti [subagents](/docs/id/agent-sdk/subagents), memiliki [kontrol TTL terpisah](/docs/id/prompt-caching#choose-the-ttl-yourself).

Entri cache untuk giliran Anda sendiri menggunakan TTL 5 menit secara default ketika Anda mengautentikasi dengan kunci API atau menjalankan pada Amazon Bedrock, Agent Platform Google Cloud, Microsoft Foundry, atau [Claude Platform on AWS](/docs/id/claude-platform-on-aws). Jika beban kerja Anda menjalankan banyak sesi pendek terhadap prompt sistem dan konteks yang sama dengan celah lebih lama dari 5 menit di antara mereka, cache kedaluwarsa di antara sesi dan setiap sesi baru membayar harga input penuh.

Untuk meminta TTL 1 jam pada penulisan cache, atur variabel lingkungan [`ENABLE_PROMPT_CACHING_1H`](/docs/id/env-vars). Anda dapat mengekspornya di lingkungan shell atau container Anda, atau meneruskannya melalui `options.env`.

Contoh berikut mengaktifkan TTL 1 jam untuk agen yang berjalan di Amazon Bedrock. Karena menetapkan `CLAUDE_CODE_USE_BEDROCK`, itu memerlukan kredensial AWS yang berfungsi untuk [Amazon Bedrock](/docs/id/amazon-bedrock); tanpanya kueri gagal.

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

Penulisan cache dengan TTL 1 jam ditagih pada tingkat lebih tinggi daripada penulisan 5 menit, jadi mengaktifkan ini menukar biaya penulisan lebih tinggi untuk lebih banyak pembacaan cache. Lihat [prompt caching pricing](https://platform.claude.com/docs/en/build-with-claude/prompt-caching) untuk detail. Pada langganan Claude dalam penggunaan yang disertakan rencana Anda, Anda mendapatkan TTL 1 jam pada giliran Anda sendiri, dan pada beberapa permintaan pembantu yang Claude Code buat di samping mereka, tanpa menetapkan variabel ini, dan Claude Code menjatuhkan giliran tersebut ke TTL 5 menit setelah Anda menarik pada [usage credits](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans).

`ENABLE_PROMPT_CACHING_1H` meminta TTL 1 jam pada setiap permintaan di kedua bucket. Untuk memilih TTL untuk setiap bucket secara terpisah, gunakan kontrol ini sebagai gantinya. Masing-masing mengambil `5m` atau `1h` dan mengambil prioritas atas `ENABLE_PROMPT_CACHING_1H`:

* Percakapan utama: variabel lingkungan [`CLAUDE_CODE_PROMPT_CACHE_TTL`](/docs/id/env-vars), atau pengaturan [`promptCacheTtl`](/docs/id/settings-reference#promptcachettl)
* Segalanya: variabel lingkungan `CLAUDE_CODE_SUBAGENT_PROMPT_CACHE_TTL`, atau pengaturan [`subagentPromptCacheTtl`](/docs/id/settings-reference#subagentpromptcachettl)

Menetapkan `promptCacheTtl` ke `1h` menjaga cache 1 jam pada percakapan utama sementara Anda menarik pada usage credits. Untuk urutan prioritas lengkap, lihat [choose the TTL yourself](/docs/id/prompt-caching#choose-the-ttl-yourself).

<h2 id="related-documentation">
  Dokumentasi terkait
</h2>

* [Referensi SDK TypeScript](/docs/id/agent-sdk/typescript) - Dokumentasi API lengkap
* [Ikhtisar SDK](/docs/id/agent-sdk/overview) - Memulai dengan SDK
* [Izin SDK](/docs/id/agent-sdk/permissions) - Mengelola izin alat
