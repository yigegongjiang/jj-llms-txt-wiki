> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Dapatkan output terstruktur dari agen

> Kembalikan JSON yang divalidasi dari alur kerja agen menggunakan JSON Schema, Zod, atau Pydantic. Dapatkan data terstruktur yang aman tipe setelah penggunaan alat multi-putaran.

Output terstruktur memungkinkan Anda menentukan bentuk data yang tepat yang ingin Anda kembalikan dari agen. Agen dapat menggunakan alat apa pun yang diperlukan untuk menyelesaikan tugas, dan Anda tetap mendapatkan JSON yang divalidasi sesuai dengan skema Anda di akhir. Tentukan [JSON Schema](https://json-schema.org/understanding-json-schema/about) untuk struktur yang Anda butuhkan, dan SDK memvalidasi output terhadapnya, meminta ulang jika tidak cocok. Jika validasi tidak berhasil dalam batas percobaan ulang, hasilnya adalah kesalahan bukan data terstruktur; lihat [Penanganan kesalahan](#error-handling).

Untuk keamanan tipe penuh, gunakan [Zod](#type-safe-schemas-with-zod-and-pydantic) (TypeScript) atau [Pydantic](#type-safe-schemas-with-zod-and-pydantic) (Python) untuk menentukan skema Anda dan dapatkan objek yang sangat diketik kembali.

<h2 id="why-structured-outputs">
  Mengapa output terstruktur?
</h2>

Agen mengembalikan teks bentuk bebas secara default, yang berfungsi untuk obrolan tetapi tidak ketika Anda perlu menggunakan output secara terprogram. Output terstruktur memberi Anda data yang diketik yang dapat Anda teruskan langsung ke logika aplikasi, database, atau komponen UI Anda.

Pertimbangkan aplikasi resep di mana agen mencari web dan membawa kembali resep. Tanpa output terstruktur, Anda mendapatkan teks bentuk bebas yang perlu Anda parsing sendiri. Dengan output terstruktur, Anda menentukan bentuk yang Anda inginkan dan mendapatkan data yang diketik yang dapat Anda gunakan langsung di aplikasi Anda.

<AccordionGroup>
  <Accordion title="Tanpa output terstruktur">
    ```text theme={null}
    Berikut adalah resep kue cokelat chip klasik!

    **Kue Cokelat Chip**
    Waktu persiapan: 15 menit | Waktu memasak: 10 menit

    Bahan-bahan:
    - 2 1/4 cangkir tepung serbaguna
    - 1 cangkir mentega, dilunakkan
    ...
    ```

    Untuk menggunakan ini di aplikasi Anda, Anda perlu mengurai judul, mengonversi "15 menit" menjadi angka, memisahkan bahan dari instruksi, dan menangani pemformatan yang tidak konsisten di seluruh respons.
  </Accordion>

  <Accordion title="Dengan output terstruktur">
    ```json theme={null}
    {
      "name": "Kue Cokelat Chip",
      "prep_time_minutes": 15,
      "cook_time_minutes": 10,
      "ingredients": [
        { "item": "tepung serbaguna", "amount": 2.25, "unit": "cangkir" },
        { "item": "mentega, dilunakkan", "amount": 1, "unit": "cangkir" }
        // ...
      ],
      "steps": ["Panaskan oven hingga 375°F", "Kocok mentega dan gula" /* ... */]
    }
    ```

    Data yang diketik yang dapat Anda gunakan langsung di UI Anda.
  </Accordion>
</AccordionGroup>

<h2 id="quick-start">
  Mulai cepat
</h2>

Untuk menggunakan output terstruktur, tentukan [JSON Schema](https://json-schema.org/understanding-json-schema/about) yang menggambarkan bentuk data yang Anda inginkan, kemudian teruskan ke `query()` melalui opsi `outputFormat` (TypeScript) atau opsi `output_format` (Python). Ketika agen selesai, pesan hasil mencakup bidang `structured_output` dengan data yang divalidasi sesuai dengan skema Anda.

Contoh di bawah ini meminta agen untuk meneliti Anthropic dan mengembalikan nama perusahaan, tahun didirikan, dan kantor pusat sebagai output terstruktur.

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // Tentukan bentuk data yang ingin Anda kembalikan
  const schema = {
    type: "object",
    properties: {
      company_name: { type: "string" },
      founded_year: { type: "number" },
      headquarters: { type: "string" }
    },
    required: ["company_name"]
  };

  for await (const message of query({
    prompt: "Teliti Anthropic dan berikan informasi perusahaan utama",
    options: {
      outputFormat: {
        type: "json_schema",
        schema: schema
      }
    }
  })) {
    // Pesan hasil berisi structured_output dengan data yang divalidasi
    if (message.type === "result" && message.subtype === "success" && message.structured_output) {
      console.log(message.structured_output);
      // { company_name: "Anthropic", founded_year: 2021, headquarters: "San Francisco, CA" }
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage

  // Tentukan bentuk data yang ingin Anda kembalikan
  schema = {
      "type": "object",
      "properties": {
          "company_name": {"type": "string"},
          "founded_year": {"type": "number"},
          "headquarters": {"type": "string"},
      },
      "required": ["company_name"],
  }


  async def main():
      async for message in query(
          prompt="Teliti Anthropic dan berikan informasi perusahaan utama",
          options=ClaudeAgentOptions(
              output_format={"type": "json_schema", "schema": schema}
          ),
      ):
          // Pesan hasil berisi structured_output dengan data yang divalidasi
          if isinstance(message, ResultMessage) and message.structured_output:
              print(message.structured_output)
              // {'company_name': 'Anthropic', 'founded_year': 2021, 'headquarters': 'San Francisco, CA'}


  asyncio.run(main())
  ```
</CodeGroup>

<h2 id="type-safe-schemas-with-zod-and-pydantic">
  Skema yang aman tipe dengan Zod dan Pydantic
</h2>

Alih-alih menulis JSON Schema dengan tangan, Anda dapat menggunakan [Zod](https://zod.dev/) (TypeScript) atau [Pydantic](https://docs.pydantic.dev/latest/) (Python) untuk menentukan skema Anda. Perpustakaan ini menghasilkan JSON Schema untuk Anda dan memungkinkan Anda mengurai respons menjadi objek yang sepenuhnya diketik yang dapat Anda gunakan di seluruh basis kode Anda dengan pelengkapan otomatis dan pemeriksaan tipe.

Contoh di bawah ini menentukan skema untuk rencana implementasi fitur dengan ringkasan, daftar langkah (masing-masing dengan tingkat kompleksitas), dan risiko potensial. Agen merencanakan fitur dan mengembalikan objek `FeaturePlan` yang diketik. Anda kemudian dapat mengakses properti seperti `plan.summary` dan mengulangi `plan.steps` dengan keamanan tipe penuh.

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { z } from "zod";
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // Tentukan skema dengan Zod
  const FeaturePlan = z.object({
    feature_name: z.string(),
    summary: z.string(),
    steps: z.array(
      z.object({
        step_number: z.number(),
        description: z.string(),
        estimated_complexity: z.enum(["low", "medium", "high"])
      })
    ),
    risks: z.array(z.string())
  });

  type FeaturePlan = z.infer<typeof FeaturePlan>;

  // Konversi ke JSON Schema
  const schema = z.toJSONSchema(FeaturePlan);

  // Gunakan dalam query
  for await (const message of query({
    prompt:
      "Rencanakan cara menambahkan dukungan mode gelap ke aplikasi React. Pecahkan menjadi langkah-langkah implementasi.",
    options: {
      outputFormat: {
        type: "json_schema",
        schema: schema
      }
    }
  })) {
    if (message.type === "result" && message.subtype === "success" && message.structured_output) {
      // Validasi dan dapatkan hasil yang sepenuhnya diketik
      const parsed = FeaturePlan.safeParse(message.structured_output);
      if (parsed.success) {
        const plan: FeaturePlan = parsed.data;
        console.log(`Fitur: ${plan.feature_name}`);
        console.log(`Ringkasan: ${plan.summary}`);
        plan.steps.forEach((step) => {
          console.log(`${step.step_number}. [${step.estimated_complexity}] ${step.description}`);
        });
      }
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  from pydantic import BaseModel
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage


  class Step(BaseModel):
      step_number: int
      description: str
      estimated_complexity: str  # 'low', 'medium', 'high'


  class FeaturePlan(BaseModel):
      feature_name: str
      summary: str
      steps: list[Step]
      risks: list[str]


  async def main():
      async for message in query(
          prompt="Rencanakan cara menambahkan dukungan mode gelap ke aplikasi React. Pecahkan menjadi langkah-langkah implementasi.",
          options=ClaudeAgentOptions(
              output_format={
                  "type": "json_schema",
                  "schema": FeaturePlan.model_json_schema(),
              }
          ),
      ):
          if isinstance(message, ResultMessage) and message.structured_output:
              # Validasi dan dapatkan hasil yang sepenuhnya diketik
              plan = FeaturePlan.model_validate(message.structured_output)
              print(f"Fitur: {plan.feature_name}")
              print(f"Ringkasan: {plan.summary}")
              for step in plan.steps:
                  print(
                      f"{step.step_number}. [{step.estimated_complexity}] {step.description}"
                  )


  asyncio.run(main())
  ```
</CodeGroup>

**Manfaat:**

* Inferensi tipe penuh (TypeScript) dan petunjuk tipe (Python)
* Validasi runtime dengan `safeParse()` atau `model_validate()`
* Pesan kesalahan yang lebih baik
* Skema yang dapat dikomposisi dan dapat digunakan kembali

<h2 id="output-format-configuration">
  Konfigurasi format output
</h2>

Opsi `outputFormat` (TypeScript) atau `output_format` (Python) menerima objek dengan:

* `type`: Atur ke `"json_schema"` untuk output terstruktur
* `schema`: Objek [JSON Schema](https://json-schema.org/understanding-json-schema/about) yang menentukan struktur output Anda. Anda dapat menghasilkan ini dari skema Zod dengan `z.toJSONSchema()` atau model Pydantic dengan `.model_json_schema()`

SDK mendukung fitur JSON Schema standar termasuk semua tipe dasar (object, array, string, number, boolean, null), `enum`, `const`, `required`, objek bersarang, dan definisi `$ref`. Untuk daftar lengkap fitur yang didukung dan batasan, lihat [Batasan JSON Schema](https://platform.claude.com/docs/id/build-with-claude/structured-outputs#json-schema-limitations).

Skema yang bukan JSON Schema yang valid gagal dijalankan saat startup dengan kesalahan yang menyebutkan masalahnya. Sebelum v2.1.205, skema yang tidak valid secara diam-diam diabaikan dan agen mengembalikan teks yang tidak terstruktur.

Kata kunci `format`, seperti `"format": "email"`, diterima sebagai anotasi dan tidak diberlakukan oleh validator SDK. Sebelum v2.1.205, skema apa pun yang berisi `format` dianggap tidak valid.

<h2 id="example-todo-tracking-agent">
  Contoh: agen pelacakan TODO
</h2>

Contoh ini menunjukkan bagaimana output terstruktur bekerja dengan penggunaan alat multi-langkah. Agen perlu menemukan komentar TODO dalam basis kode, kemudian mencari informasi git blame untuk masing-masing. Agen secara mandiri memutuskan alat mana yang akan digunakan (Grep untuk mencari, Bash untuk menjalankan perintah git) dan menggabungkan hasil menjadi respons terstruktur tunggal.

Skema mencakup bidang opsional (`author` dan `date`) karena informasi git blame mungkin tidak tersedia untuk semua file. Agen mengisi apa yang dapat ditemukannya dan menghilangkan sisanya.

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // Tentukan struktur untuk ekstraksi TODO
  const todoSchema = {
    type: "object",
    properties: {
      todos: {
        type: "array",
        items: {
          type: "object",
          properties: {
            text: { type: "string" },
            file: { type: "string" },
            line: { type: "number" },
            author: { type: "string" },
            date: { type: "string" }
          },
          required: ["text", "file", "line"]
        }
      },
      total_count: { type: "number" }
    },
    required: ["todos", "total_count"]
  };

  // Agen menggunakan Grep untuk menemukan TODO, Bash untuk mendapatkan informasi git blame
  for await (const message of query({
    prompt: "Temukan semua komentar TODO dalam basis kode ini dan identifikasi siapa yang menambahkannya",
    options: {
      outputFormat: {
        type: "json_schema",
        schema: todoSchema
      }
    }
  })) {
    if (message.type === "result" && message.subtype === "success" && message.structured_output) {
      const data = message.structured_output as { total_count: number; todos: Array<{ file: string; line: number; text: string; author?: string; date?: string }> };
      console.log(`Ditemukan ${data.total_count} TODO`);
      data.todos.forEach((todo) => {
        console.log(`${todo.file}:${todo.line} - ${todo.text}`);
        if (todo.author) {
          console.log(`  Ditambahkan oleh ${todo.author} pada ${todo.date}`);
        }
      });
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage

  # Tentukan struktur untuk ekstraksi TODO
  todo_schema = {
      "type": "object",
      "properties": {
          "todos": {
              "type": "array",
              "items": {
                  "type": "object",
                  "properties": {
                      "text": {"type": "string"},
                      "file": {"type": "string"},
                      "line": {"type": "number"},
                      "author": {"type": "string"},
                      "date": {"type": "string"},
                  },
                  "required": ["text", "file", "line"],
              },
          },
          "total_count": {"type": "number"},
      },
      "required": ["todos", "total_count"],
  }


  async def main():
      # Agen menggunakan Grep untuk menemukan TODO, Bash untuk mendapatkan informasi git blame
      async for message in query(
          prompt="Temukan semua komentar TODO dalam basis kode ini dan identifikasi siapa yang menambahkannya",
          options=ClaudeAgentOptions(
              output_format={"type": "json_schema", "schema": todo_schema}
          ),
      ):
          if isinstance(message, ResultMessage) and message.structured_output:
              data = message.structured_output
              print(f"Ditemukan {data['total_count']} TODO")
              for todo in data["todos"]:
                  print(f"{todo['file']}:{todo['line']} - {todo['text']}")
                  if "author" in todo:
                      print(f"  Ditambahkan oleh {todo['author']} pada {todo['date']}")


  asyncio.run(main())
  ```
</CodeGroup>

<h2 id="error-handling">
  Penanganan kesalahan
</h2>

Pembuatan output terstruktur dapat gagal ketika agen tidak dapat menghasilkan JSON yang valid sesuai dengan skema Anda. Ini biasanya terjadi ketika skema terlalu kompleks untuk tugas, tugas itu sendiri ambigu, atau agen mencapai batas percobaan ulangnya mencoba memperbaiki kesalahan validasi. Ini juga dapat terjadi tanpa ada kegagalan validasi: [fallback model](/docs/id/model-config#automatic-model-fallback) dapat menarik kembali output yang sudah selesai di tengah aliran, dan jika tidak ada percobaan ulang yang menggantinya, jalannya berakhir dengan kesalahan yang sama. Periksa bidang `errors` pada pesan hasil untuk membedakan dua penyebab sebelum men-debug skema Anda.

Ketika kesalahan terjadi, pesan hasil memiliki `subtype` yang menunjukkan apa yang salah:

| Subtype                               | Arti                                                                                                                                                         |
| ------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `success`                             | Output dihasilkan dan divalidasi dengan berhasil                                                                                                             |
| `error_max_structured_output_retries` | Tidak ada output yang valid yang bertahan setelah beberapa percobaan (kegagalan validasi, atau penarikan fallback model tanpa percobaan ulang yang berhasil) |

Contoh di bawah ini memeriksa bidang `subtype` untuk menentukan apakah output dihasilkan dengan berhasil atau jika Anda perlu menangani kegagalan:

<CodeGroup>
  ```typescript TypeScript theme={null}
  for await (const msg of query({
    prompt: "Extract contact info from the document",
    options: {
      outputFormat: {
        type: "json_schema",
        schema: contactSchema
      }
    }
  })) {
    if (msg.type === "result") {
      if (msg.subtype === "success" && msg.structured_output) {
        // Gunakan output yang divalidasi
        console.log(msg.structured_output);
      } else if (msg.subtype === "error_max_structured_output_retries") {
        // Tangani kegagalan - coba ulang dengan prompt yang lebih sederhana, kembali ke yang tidak terstruktur, dll.
        console.error("Tidak dapat menghasilkan output yang valid");
      }
    }
  }
  ```

  ```python Python theme={null}
  async for message in query(
      prompt="Extract contact info from the document",
      options=ClaudeAgentOptions(
          output_format={"type": "json_schema", "schema": contact_schema}
      ),
  ):
      if isinstance(message, ResultMessage):
          if message.subtype == "success" and message.structured_output:
              # Gunakan output yang divalidasi
              print(message.structured_output)
          elif message.subtype == "error_max_structured_output_retries":
              # Tangani kegagalan
              print("Tidak dapat menghasilkan output yang valid")
  ```
</CodeGroup>

**Tips untuk menghindari kesalahan:**

* **Jaga skema tetap fokus.** Skema yang bersarang dalam dengan banyak bidang yang diperlukan lebih sulit untuk dipenuhi. Mulai sederhana dan tambahkan kompleksitas sesuai kebutuhan.
* **Cocokkan skema dengan tugas.** Jika tugas mungkin tidak memiliki semua informasi yang diperlukan skema Anda, buat bidang tersebut opsional.
* **Gunakan prompt yang jelas.** Prompt yang ambigu membuat lebih sulit bagi agen untuk mengetahui output apa yang harus dihasilkan.

<h2 id="related-resources">
  Sumber daya terkait
</h2>

* [Dokumentasi JSON Schema](https://json-schema.org/): pelajari sintaks JSON Schema untuk menentukan skema kompleks dengan objek bersarang, array, enum, dan batasan validasi
* [API Structured Outputs](https://platform.claude.com/docs/en/build-with-claude/structured-outputs): gunakan output terstruktur dengan Claude API secara langsung untuk permintaan satu putaran tanpa penggunaan alat
* [Custom tools](/docs/id/agent-sdk/custom-tools): berikan agen Anda alat khusus untuk dipanggil selama eksekusi sebelum mengembalikan output terstruktur
