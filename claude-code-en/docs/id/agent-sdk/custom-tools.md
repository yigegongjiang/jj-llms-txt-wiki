> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Berikan Claude alat kustom

> Tentukan alat kustom dengan server MCP dalam proses SDK Agent sehingga Claude dapat memanggil fungsi Anda, mengakses API Anda, dan melakukan operasi khusus domain.

Alat kustom memperluas SDK Agent dengan memungkinkan Anda menentukan fungsi Anda sendiri yang dapat dipanggil Claude selama percakapan. Menggunakan server MCP dalam proses SDK, Anda dapat memberikan Claude akses ke database, API eksternal, logika khusus domain, atau kemampuan lain yang dibutuhkan aplikasi Anda.

<h2 id="quick-reference">
  Referensi cepat
</h2>

| Apa yang ingin Anda lakukan                   | Lakukan ini                                                                                                                                                                                                     |
| :-------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Menentukan alat                               | Gunakan [`@tool`](/docs/id/agent-sdk/python#tool) (Python) atau [`tool()`](/docs/id/agent-sdk/typescript#tool) (TypeScript) dengan nama, deskripsi, skema, dan penangan. Lihat [Buat alat kustom](#create-a-custom-tool). |
| Mendaftarkan alat dengan Claude               | Bungkus dalam `create_sdk_mcp_server` / `createSdkMcpServer` dan teruskan ke `mcpServers` dalam `query()`. Lihat [Panggil alat kustom](#call-a-custom-tool).                                                    |
| Pra-setujui alat                              | Tambahkan ke alat yang diizinkan Anda. Lihat [Konfigurasi alat yang diizinkan](#configure-allowed-tools).                                                                                                       |
| Hapus alat bawaan dari konteks Claude         | Teruskan array `tools` yang hanya mencantumkan bawaan yang Anda inginkan. Lihat [Konfigurasi alat yang diizinkan](#configure-allowed-tools).                                                                    |
| Biarkan Claude memanggil alat secara paralel  | Atur `readOnlyHint: true` pada alat tanpa efek samping. Lihat [Tambahkan anotasi alat](#add-tool-annotations).                                                                                                  |
| Kontrol pesan kesalahan yang dibaca Claude    | Kembalikan `isError: true` untuk menyusun pesan alih-alih menampilkan pengecualian mentah. Lihat [Tangani kesalahan](#handle-errors).                                                                           |
| Kembalikan gambar atau file                   | Gunakan blok `image` atau `resource` dalam array konten. Lihat [Kembalikan gambar dan sumber daya](#return-images-and-resources).                                                                               |
| Kembalikan hasil JSON yang dapat dibaca mesin | Atur `structuredContent` pada hasilnya. Lihat [Kembalikan data terstruktur](#return-structured-data).                                                                                                           |
| Skalakan ke banyak alat                       | Gunakan [pencarian alat](/docs/id/agent-sdk/tool-search) untuk memuat alat sesuai permintaan.                                                                                                                        |

<h2 id="create-a-custom-tool">
  Buat alat kustom
</h2>

Alat didefinisikan oleh empat bagian, dilewatkan sebagai argumen ke helper [`tool()`](/docs/id/agent-sdk/typescript#tool) di TypeScript atau dekorator [`@tool`](/docs/id/agent-sdk/python#tool) di Python:

* **Nama:** pengidentifikasi unik yang digunakan Claude untuk memanggil alat.
* **Deskripsi:** apa yang dilakukan alat. Claude membaca ini untuk memutuskan kapan memanggilnya.
* **Skema input:** argumen yang harus disediakan Claude. Di TypeScript ini selalu [skema Zod](https://zod.dev/), dan `args` handler diketik darinya secara otomatis. Di Python ini adalah dict yang memetakan nama ke tipe, seperti `{"latitude": float}`, yang dikonversi SDK ke JSON Schema untuk Anda. Dekorator Python juga menerima dict [JSON Schema](https://json-schema.org/understanding-json-schema/about) lengkap secara langsung ketika Anda membutuhkan enums, rentang, bidang opsional, atau objek bersarang.
* **Handler:** fungsi async yang berjalan ketika Claude memanggil alat. Ini menerima argumen yang divalidasi dan harus mengembalikan objek dengan:
  * `content` (diperlukan): array blok hasil, masing-masing dengan `type` dari `"text"`, `"image"`, `"audio"`, `"resource"`, atau `"resource_link"`. Lihat [Return images and resources](#return-images-and-resources) untuk blok non-teks.
  * `structuredContent` (opsional): objek JSON yang menyimpan hasil sebagai data yang dapat dibaca mesin, dikembalikan bersama `content`. Lihat [Return structured data](#return-structured-data).
  * `isError` (opsional): atur ke `true` untuk menandakan kegagalan alat sehingga Claude dapat bereaksi terhadapnya. Lihat [Handle errors](#handle-errors).

Setelah mendefinisikan alat, bungkus dalam server dengan [`createSdkMcpServer`](/docs/id/agent-sdk/typescript#createsdkmcpserver) (TypeScript) atau [`create_sdk_mcp_server`](/docs/id/agent-sdk/python#create_sdk_mcp_server) (Python). Server berjalan in-process di dalam aplikasi Anda, bukan sebagai proses terpisah.

<h3 id="weather-tool-example">
  Contoh alat cuaca
</h3>

Contoh ini mendefinisikan alat `get_temperature` dan membungkusnya dalam server MCP. Ini hanya mengatur alat; untuk meneruskannya ke `query` dan menjalankannya, lihat [Call a custom tool](#call-a-custom-tool) di bawah.

<CodeGroup>
  ```python Python theme={null}
  from typing import Any
  import httpx
  from claude_agent_sdk import tool, create_sdk_mcp_server


  # Define a tool: name, description, input schema, handler
  @tool(
      "get_temperature",
      "Get the current temperature at a location",
      {"latitude": float, "longitude": float},
  )
  async def get_temperature(args: dict[str, Any]) -> dict[str, Any]:
      async with httpx.AsyncClient() as client:
          response = await client.get(
              "https://api.open-meteo.com/v1/forecast",
              params={
                  "latitude": args["latitude"],
                  "longitude": args["longitude"],
                  "current": "temperature_2m",
                  "temperature_unit": "fahrenheit",
              },
          )
          data = response.json()

      # Return a content array - Claude sees this as the tool result
      return {
          "content": [
              {
                  "type": "text",
                  "text": f"Temperature: {data['current']['temperature_2m']}°F",
              }
          ]
      }


  # Wrap the tool in an in-process MCP server
  weather_server = create_sdk_mcp_server(
      name="weather",
      version="1.0.0",
      tools=[get_temperature],
  )
  ```

  ```typescript TypeScript theme={null}
  import { tool, createSdkMcpServer } from "@anthropic-ai/claude-agent-sdk";
  import { z } from "zod";

  // Define a tool: name, description, input schema, handler
  const getTemperature = tool(
    "get_temperature",
    "Get the current temperature at a location",
    {
      latitude: z.number().describe("Latitude coordinate"), // .describe() adds a field description Claude sees
      longitude: z.number().describe("Longitude coordinate")
    },
    async (args) => {
      // args is typed from the schema: { latitude: number; longitude: number }
      const response = await fetch(
        `https://api.open-meteo.com/v1/forecast?latitude=${args.latitude}&longitude=${args.longitude}&current=temperature_2m&temperature_unit=fahrenheit`
      );
      const data: any = await response.json();

      // Return a content array - Claude sees this as the tool result
      return {
        content: [{ type: "text", text: `Temperature: ${data.current.temperature_2m}°F` }]
      };
    }
  );

  // Wrap the tool in an in-process MCP server
  const weatherServer = createSdkMcpServer({
    name: "weather",
    version: "1.0.0",
    tools: [getTemperature]
  });
  ```
</CodeGroup>

Lihat referensi TypeScript [`tool()`](/docs/id/agent-sdk/typescript#tool) atau referensi Python [`@tool`](/docs/id/agent-sdk/python#tool) untuk detail parameter lengkap, termasuk format input JSON Schema dan struktur nilai pengembalian.

<Tip>
  Untuk membuat parameter opsional: di TypeScript, tambahkan `.default()` ke bidang Zod. Di Python, skema dict memperlakukan setiap kunci sebagai diperlukan, jadi tinggalkan parameter dari skema, sebutkan dalam string deskripsi, dan baca dengan `args.get()` di handler. Alat [`get_precipitation_chance` di bawah](#add-more-tools) menunjukkan kedua pola.
</Tip>

<h3 id="call-a-custom-tool">
  Panggil alat kustom
</h3>

Teruskan server MCP yang Anda buat ke `query` melalui opsi `mcpServers`. Kunci dalam `mcpServers` menjadi segmen `{server_name}` dalam nama lengkap setiap alat: `mcp__{server_name}__{tool_name}`. Daftarkan nama itu dalam `allowedTools` sehingga alat berjalan tanpa prompt izin.

Potongan ini menggunakan kembali `weatherServer` dari [contoh alat cuaca](#weather-tool-example) untuk menanyakan Claude tentang cuaca di lokasi tertentu.

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage


  async def main():
      options = ClaudeAgentOptions(
          mcp_servers={"weather": weather_server},
          allowed_tools=["mcp__weather__get_temperature"],
      )

      async for message in query(
          prompt="What's the temperature in San Francisco?",
          options=options,
      ):
          # ResultMessage is the final message after all tool calls complete
          if isinstance(message, ResultMessage) and message.subtype == "success":
              print(message.result)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "What's the temperature in San Francisco?",
    options: {
      mcpServers: { weather: weatherServer },
      allowedTools: ["mcp__weather__get_temperature"]
    }
  })) {
    // "result" is the final message after all tool calls complete
    if (message.type === "result" && message.subtype === "success") {
      console.log(message.result);
    }
  }
  ```
</CodeGroup>

Gabungkan potongan ini dengan definisi alat dan server dari [contoh alat cuaca](#weather-tool-example) dalam satu file, kemudian jalankan dengan `python weather.py` untuk Python atau `npx tsx weather.ts` untuk TypeScript. Claude memanggil `get_temperature` dan skrip mencetak jawaban satu baris dengan suhu saat ini di San Francisco.

<h3 id="add-more-tools">
  Tambahkan lebih banyak alat
</h3>

Server menampung sebanyak alat yang Anda daftarkan dalam array `tools`-nya. Dengan lebih dari satu alat di server, Anda dapat mendaftarkan masing-masing dalam `allowedTools` secara individual atau menggunakan wildcard `mcp__weather__*` untuk mencakup setiap alat yang diekspos server.

Contoh di bawah mendefinisikan alat kedua, `get_precipitation_chance`, dan mengganti definisi `weatherServer` dari [contoh alat cuaca](#weather-tool-example) dengan yang mencantumkan kedua alat dalam array.

<CodeGroup>
  ```python Python theme={null}
  # Define a second tool for the same server
  @tool(
      "get_precipitation_chance",
      "Get the hourly precipitation probability for a location. "
      "Optionally pass 'hours' (1-24) to control how many hours to return.",
      {"latitude": float, "longitude": float},
  )
  async def get_precipitation_chance(args: dict[str, Any]) -> dict[str, Any]:
      # 'hours' isn't in the schema - read it with .get() to make it optional
      hours = args.get("hours", 12)
      async with httpx.AsyncClient() as client:
          response = await client.get(
              "https://api.open-meteo.com/v1/forecast",
              params={
                  "latitude": args["latitude"],
                  "longitude": args["longitude"],
                  "hourly": "precipitation_probability",
                  "forecast_days": 1,
              },
          )
          data = response.json()
      chances = data["hourly"]["precipitation_probability"][:hours]

      return {
          "content": [
              {
                  "type": "text",
                  "text": f"Next {hours} hours: {'%, '.join(map(str, chances))}%",
              }
          ]
      }


  # Rebuild the server with both tools in the array
  weather_server = create_sdk_mcp_server(
      name="weather",
      version="1.0.0",
      tools=[get_temperature, get_precipitation_chance],
  )
  ```

  ```typescript TypeScript theme={null}
  // Define a second tool for the same server
  const getPrecipitationChance = tool(
    "get_precipitation_chance",
    "Get the hourly precipitation probability for a location",
    {
      latitude: z.number(),
      longitude: z.number(),
      hours: z
        .number()
        .int()
        .min(1)
        .max(24)
        .default(12) // .default() makes the parameter optional
        .describe("How many hours of forecast to return")
    },
    async (args) => {
      const response = await fetch(
        `https://api.open-meteo.com/v1/forecast?latitude=${args.latitude}&longitude=${args.longitude}&hourly=precipitation_probability&forecast_days=1`
      );
      const data: any = await response.json();
      const chances = data.hourly.precipitation_probability.slice(0, args.hours);

      return {
        content: [{ type: "text", text: `Next ${args.hours} hours: ${chances.join("%, ")}%` }]
      };
    }
  );

  // Rebuild the server with both tools in the array
  const weatherServer = createSdkMcpServer({
    name: "weather",
    version: "1.0.0",
    tools: [getTemperature, getPrecipitationChance]
  });
  ```
</CodeGroup>

[Tool search](/docs/id/agent-sdk/tool-search) aktif secara default dan menunda alat SDK MCP: Claude melihat nama setiap alat dalam daftar kompak dan memuat skema lengkapnya sesuai permintaan. Dengan pencarian alat dinonaktifkan, setiap alat dalam array ini mengonsumsi ruang jendela konteks pada setiap giliran. Di TypeScript, teruskan `alwaysLoad: true` dalam argumen `extras` dari [`tool()`](/docs/id/agent-sdk/typescript#tool) atau dalam opsi [`createSdkMcpServer()`](/docs/id/agent-sdk/typescript#createsdkmcpserver) untuk menjaga skema lengkap alat dalam prompt awal.

<h3 id="add-tool-annotations">
  Tambahkan anotasi alat
</h3>

[Anotasi alat](https://modelcontextprotocol.io/docs/concepts/tools#tool-annotations) adalah metadata opsional yang menjelaskan bagaimana alat berperilaku. Teruskan sebagai argumen kelima ke helper `tool()` di TypeScript atau melalui argumen kata kunci `annotations` untuk dekorator `@tool` di Python. Semua bidang petunjuk adalah Boolean.

| Bidang            | Default | Arti                                                                                                                        |
| :---------------- | :------ | :-------------------------------------------------------------------------------------------------------------------------- |
| `readOnlyHint`    | `false` | Alat tidak memodifikasi lingkungannya. Mengontrol apakah alat dapat dipanggil secara paralel dengan alat baca-saja lainnya. |
| `destructiveHint` | `true`  | Alat dapat melakukan pembaruan destruktif. Hanya informatif.                                                                |
| `idempotentHint`  | `false` | Panggilan berulang dengan argumen yang sama tidak memiliki efek tambahan. Hanya informatif.                                 |
| `openWorldHint`   | `true`  | Alat menjangkau sistem di luar proses Anda. Hanya informatif.                                                               |

Anotasi adalah metadata, bukan penegakan. Alat yang ditandai `readOnlyHint: true` masih dapat menulis ke disk jika itulah yang dilakukan handler. Jaga anotasi tetap akurat dengan handler.

Contoh ini menambahkan `readOnlyHint` ke alat `get_temperature` dari [contoh alat cuaca](#weather-tool-example).

<CodeGroup>
  ```python Python theme={null}
  from claude_agent_sdk import tool, ToolAnnotations


  @tool(
      "get_temperature",
      "Get the current temperature at a location",
      {"latitude": float, "longitude": float},
      annotations=ToolAnnotations(
          readOnlyHint=True
      ),  # Lets Claude batch this with other read-only calls
  )
  async def get_temperature(args):
      return {"content": [{"type": "text", "text": "..."}]}
  ```

  ```typescript TypeScript theme={null}
  import { tool } from "@anthropic-ai/claude-agent-sdk";
  import { z } from "zod";

  tool(
    "get_temperature",
    "Get the current temperature at a location",
    { latitude: z.number(), longitude: z.number() },
    async (args) => ({ content: [{ type: "text", text: `...` }] }),
    { annotations: { readOnlyHint: true } } // Lets Claude batch this with other read-only calls
  );
  ```
</CodeGroup>

Lihat `ToolAnnotations` dalam referensi [TypeScript](/docs/id/agent-sdk/typescript#toolannotations) atau [Python](/docs/id/agent-sdk/python#toolannotations).

<h2 id="control-tool-access">
  Kontrol akses tool
</h2>

Contoh [weather tool](#weather-tool-example) mendaftarkan server dan mencantumkan tools dalam `allowedTools`. Bagian ini mencakup cara membatasi akses ketika Anda memiliki beberapa tools atau ingin membatasi built-ins. Untuk cara nama tool dibangun, lihat [Call a custom tool](#call-a-custom-tool).

<h3 id="configure-allowed-tools">
  Konfigurasi tools yang diizinkan
</h3>

Opsi `tools` dan daftar allowed/disallowed mempengaruhi dua lapisan: availability, yang mengontrol apakah tool muncul dalam konteks Claude, dan permission, yang mengontrol apakah panggilan disetujui setelah Claude mencobanya. `tools` dan entri `disallowedTools` dengan nama bare mengubah availability. `allowedTools` dan aturan `disallowedTools` yang scoped mengubah permission. Jika Anda menyebutkan salah satu dari [task-tracking tools](/docs/id/agent-sdk/todo-tracking#model-availability) dalam `allowedTools`, Claude Code juga memilih sesi masuk.

| Opsi                      | Lapisan      | Efek                                                                                                                                                                                                                                                                                |
| :------------------------ | :----------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `tools: ["Read", "Grep"]` | Availability | Hanya built-ins yang tercantum berada dalam konteks Claude. Built-ins yang tidak tercantum dihapus. Tools MCP tidak terpengaruh.                                                                                                                                                    |
| `tools: []`               | Availability | Semua built-ins dihapus. Claude hanya dapat menggunakan tools MCP Anda.                                                                                                                                                                                                             |
| allowed tools             | Permission   | Tools yang tercantum berjalan tanpa prompt permission. Tools lain yang tidak tercantum tetap tersedia; panggilan melalui [permission flow](/docs/id/agent-sdk/permissions).                                                                                                              |
| disallowed tools          | Keduanya     | Nama tool bare seperti `"Bash"` menghapus tool dari konteks Claude, sama seperti menghilangkannya dari `tools`. Aturan scoped seperti `"Bash(rm *)"` membiarkan tool dalam konteks dan menolak hanya panggilan yang cocok [seperti yang ditulis](/docs/id/permissions#bash-rule-limits). |

Untuk menghapus built-in sepenuhnya, hilangkan dari `tools` atau cantumkan nama barenya dalam `disallowedTools` (Python: `disallowed_tools`); keduanya membuat tool tetap keluar dari konteks sehingga Claude tidak pernah mencobanya. Aturan `disallowedTools` yang scoped memblokir panggilan yang cocok tetapi membiarkan tool tetap terlihat, jadi Claude mungkin membuang giliran mencobanya. Lihat [Configure permissions](/docs/id/agent-sdk/permissions) untuk urutan evaluasi lengkap.

<h2 id="handle-errors">
  Menangani kesalahan
</h2>

Kesalahan handler tidak menghentikan loop agen. Server MCP in-process SDK menangkap pengecualian yang tidak tertangani dan mengembalikannya sebagai hasil kesalahan, jadi cara Anda melaporkan kesalahan menentukan apa yang Claude baca, bukan apakah kueri gagal:

| Apa yang terjadi                                                                                 | Hasil                                                                                                                                                                                      |
| :----------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Handler melempar pengecualian yang tidak tertangani                                              | Server MCP mengonversinya menjadi hasil kesalahan yang membawa pesan pengecualian mentah. Claude melihat pesan itu, dan loop agen berlanjut.                                               |
| Handler menangkap kesalahan dan mengembalikan `isError: true` (TS) / `"is_error": True` (Python) | Claude melihat pesan yang Anda susun. Anda dapat menambahkan konteks yang kurang dari pengecualian mentah, seperti permintaan mana yang gagal atau apa yang harus dicoba sebagai gantinya. |

Dalam kedua kasus Claude dapat mencoba kembali, mencoba alat yang berbeda, atau menjelaskan kegagalan. Tangkap kesalahan sendiri ketika pesan pengecualian mentah tidak cukup bagi Claude untuk bertindak.

Contoh di bawah ini menangkap dua jenis kegagalan di dalam handler dan menyusun pesan kesalahan yang Claude baca. Status HTTP non-200 ditangkap dari respons dan dikembalikan sebagai hasil kesalahan. Kesalahan jaringan atau JSON yang tidak valid ditangkap oleh `try/except` (Python) atau `try/catch` (TypeScript) di sekitarnya dan juga dikembalikan sebagai hasil kesalahan. Dalam kedua kasus Claude menerima pesan yang menjelaskan kegagalan alih-alih string pengecualian kosong.

<CodeGroup>
  ```python Python theme={null}
  import json
  import httpx
  from typing import Any
  from claude_agent_sdk import tool


  @tool(
      "fetch_data",
      "Fetch data from an API",
      {"endpoint": str},  # Simple schema
  )
  async def fetch_data(args: dict[str, Any]) -> dict[str, Any]:
      try:
          async with httpx.AsyncClient() as client:
              response = await client.get(args["endpoint"])
              if response.status_code != 200:
                  # Return the failure as a tool result so Claude can react to it.
                  # is_error marks this as a failed call rather than odd-looking data.
                  return {
                      "content": [
                          {
                              "type": "text",
                              "text": f"API error: {response.status_code} {response.reason_phrase}",
                          }
                      ],
                      "is_error": True,
                  }

              data = response.json()
              return {"content": [{"type": "text", "text": json.dumps(data, indent=2)}]}
      except Exception as e:
          # Composes the message Claude reads. An uncaught exception would
          # reach Claude as the raw str(e) with no context.
          return {
              "content": [{"type": "text", "text": f"Failed to fetch data: {str(e)}"}],
              "is_error": True,
          }
  ```

  ```typescript TypeScript theme={null}
  import { tool } from "@anthropic-ai/claude-agent-sdk";
  import { z } from "zod";

  tool(
    "fetch_data",
    "Fetch data from an API",
    {
      endpoint: z.string().url().describe("API endpoint URL")
    },
    async (args) => {
      try {
        const response = await fetch(args.endpoint);

        if (!response.ok) {
          // Return the failure as a tool result so Claude can react to it.
          // isError marks this as a failed call rather than odd-looking data.
          return {
            content: [
              {
                type: "text",
                text: `API error: ${response.status} ${response.statusText}`
              }
            ],
            isError: true
          };
        }

        const data = await response.json();
        return {
          content: [
            {
              type: "text",
              text: JSON.stringify(data, null, 2)
            }
          ]
        };
      } catch (error) {
        // Composes the message Claude reads. An uncaught throw would
        // reach Claude as the raw error message with no context.
        return {
          content: [
            {
              type: "text",
              text: `Failed to fetch data: ${error instanceof Error ? error.message : String(error)}`
            }
          ],
          isError: true
        };
      }
    }
  );
  ```
</CodeGroup>

<h2 id="return-images-and-resources">
  Mengembalikan gambar dan sumber daya
</h2>

Array `content` dalam hasil tool menerima blok `text`, `image`, `audio`, `resource`, dan `resource_link`. Anda dapat mencampurnya dalam respons yang sama. Di TypeScript, SDK menyimpan blok audio ke disk dan Claude menerima blok teks dengan jalur file yang disimpan; di Python, SDK menghapus blok audio dari hasil tool dan mencatat peringatan.

Claude menerima setiap blok tautan sumber daya sebagai blok teks yang berisi nama tautan, URI, dan deskripsi. Di TypeScript, aplikasi Anda juga menerima tautan itu sendiri sebagai [`resourceLinks`](/docs/id/agent-sdk/typescript#sdkmcpresourcelink) pada `tool_use_result` pesan pengguna; di Python, SDK meratakan mereka ke teks sebelum CLI melihat hasilnya, jadi kunci Python [`resourceLinks`](/docs/id/agent-sdk/python#usermessage) tidak pernah diproduksi untuk tool dalam proses.

<h3 id="images">
  Gambar
</h3>

Blok gambar membawa byte gambar secara inline, dikodekan sebagai base64. Tidak ada bidang URL. Untuk mengembalikan gambar yang berada di URL, ambil dalam handler, baca byte respons, dan kodekan base64 sebelum mengembalikan. Hasilnya diproses sebagai input visual.

| Field      | Type      | Notes                                                                                 |
| :--------- | :-------- | :------------------------------------------------------------------------------------ |
| `type`     | `"image"` |                                                                                       |
| `data`     | `string`  | Byte yang dikodekan Base64. Base64 mentah saja, tanpa awalan `data:image/...;base64,` |
| `mimeType` | `string`  | Diperlukan. Misalnya `image/png`, `image/jpeg`, `image/webp`, `image/gif`             |

<CodeGroup>
  ```python Python theme={null}
  import base64
  import httpx
  from claude_agent_sdk import tool


  # Define a tool that fetches an image from a URL and returns it to Claude
  @tool("fetch_image", "Fetch an image from a URL and return it to Claude", {"url": str})
  async def fetch_image(args):
      async with httpx.AsyncClient() as client:  # Fetch the image bytes
          response = await client.get(args["url"])

      return {
          "content": [
              {
                  "type": "image",
                  "data": base64.b64encode(response.content).decode(
                      "ascii"
                  ),  # Base64-encode the raw bytes
                  "mimeType": response.headers.get(
                      "content-type", "image/png"
                  ),  # Read MIME type from the response
              }
          ]
      }
  ```

  ```typescript TypeScript theme={null}
  import { tool } from "@anthropic-ai/claude-agent-sdk";
  import { z } from "zod";

  tool(
    "fetch_image",
    "Fetch an image from a URL and return it to Claude",
    {
      url: z.string().url()
    },
    async (args) => {
      const response = await fetch(args.url); // Fetch the image bytes
      const buffer = Buffer.from(await response.arrayBuffer()); // Read into a Buffer for base64 encoding
      const mimeType = response.headers.get("content-type") ?? "image/png";

      return {
        content: [
          {
            type: "image",
            data: buffer.toString("base64"), // Base64-encode the raw bytes
            mimeType
          }
        ]
      };
    }
  );
  ```
</CodeGroup>

<h3 id="resources">
  Sumber Daya
</h3>

Blok sumber daya menyematkan sepotong konten yang diidentifikasi oleh URI. URI adalah label untuk Claude untuk referensi; konten aktual berada di bidang `text` atau `blob` blok. Gunakan ini ketika tool Anda menghasilkan sesuatu yang masuk akal untuk ditangani berdasarkan nama nanti, seperti file yang dihasilkan atau catatan dari sistem eksternal.

| Field               | Type         | Notes                                                                                                                                      |
| :------------------ | :----------- | :----------------------------------------------------------------------------------------------------------------------------------------- |
| `type`              | `"resource"` |                                                                                                                                            |
| `resource.uri`      | `string`     | Pengenal untuk konten. Skema URI apa pun                                                                                                   |
| `resource.text`     | `string`     | Konten, jika berupa teks. Berikan ini atau `blob`, bukan keduanya                                                                          |
| `resource.blob`     | `string`     | Konten yang dikodekan base64, jika biner. Hanya TypeScript: SDK Python menghapus sumber daya biner dari hasil tool dan mencatat peringatan |
| `resource.mimeType` | `string`     | Opsional                                                                                                                                   |

Contoh ini menunjukkan blok sumber daya yang dikembalikan dari dalam handler tool. URI `file:///tmp/report.md` adalah label yang dapat direferensikan Claude nanti; SDK tidak membaca dari jalur tersebut.

<CodeGroup>
  ```typescript TypeScript theme={null}
  return {
    content: [
      {
        type: "resource",
        resource: {
          uri: "file:///tmp/report.md", // Label for Claude to reference, not a path the SDK reads
          mimeType: "text/markdown",
          text: "# Report\n..." // The actual content, inline
        }
      }
    ]
  };
  ```

  ```python Python theme={null}
  return {
      "content": [
          {
              "type": "resource",
              "resource": {
                  "uri": "file:///tmp/report.md",  # Label for Claude to reference, not a path the SDK reads
                  "mimeType": "text/markdown",
                  "text": "# Report\n...",  # The actual content, inline
              },
          }
      ]
  }
  ```
</CodeGroup>

Bentuk blok ini berasal dari tipe MCP `CallToolResult`. Lihat [spesifikasi MCP](https://modelcontextprotocol.io/specification/2025-06-18/server/tools#tool-result) untuk definisi lengkapnya.

<h2 id="return-structured-data">
  Mengembalikan data terstruktur
</h2>

`structuredContent` adalah objek JSON opsional pada hasil, terpisah dari array `content`. Gunakan untuk mengembalikan nilai mentah yang dapat dibaca Claude sebagai bidang yang tepat daripada menguraikannya dari string teks atau gambar.

Ketika `structuredContent` diatur, Claude menerima JSON ditambah blok gambar atau sumber daya apa pun dari `content`. Blok teks dalam `content` tidak diteruskan, karena diasumsikan menduplikasi data terstruktur. Contoh di bawah merender bagan sebagai blok gambar dan mengembalikan titik data di baliknya dalam `structuredContent` dari penangan yang sama. Dalam cuplikan, `chartPngBuffer` adalah `Buffer` yang menyimpan byte PNG yang dirender.

```typescript TypeScript theme={null}
return {
  content: [
    {
      type: "image",
      data: chartPngBuffer.toString("base64"),
      mimeType: "image/png"
    }
  ],
  structuredContent: {
    series: "temperature_2m",
    unit: "fahrenheit",
    points: [62.1, 63.4, 65.0, 64.2]
  }
};
```

<Note>
  Dekorator Python `@tool` hanya meneruskan `content` dan `is_error` dari dict pengembalian penangan. Untuk mengembalikan `structuredContent` dari Python, jalankan [server MCP mandiri](/docs/id/agent-sdk/mcp) daripada server SDK dalam proses.
</Note>

<h2 id="example-unit-converter">
  Contoh: konverter satuan
</h2>

Alat ini mengonversi nilai antara satuan panjang, suhu, dan berat. Pengguna dapat bertanya "konversi 100 kilometer ke mil" atau "berapa 72°F dalam Celsius," dan Claude memilih jenis satuan dan satuan yang tepat dari permintaan.

Ini mendemonstrasikan dua pola:

* **Enum schemas:** `unit_type` dibatasi pada sekumpulan nilai tetap. Di TypeScript, gunakan `z.enum()`. Di Python, dict schema tidak mendukung enums, jadi JSON Schema dict lengkap diperlukan.
* **Unsupported input handling:** ketika pasangan konversi tidak ditemukan, handler mengembalikan `isError: true` sehingga Claude dapat memberitahu pengguna apa yang salah daripada memperlakukan kegagalan sebagai hasil normal.

<CodeGroup>
  ```python Python theme={null}
  from typing import Any
  from claude_agent_sdk import tool, create_sdk_mcp_server


  # z.enum() in TypeScript becomes an "enum" constraint in JSON Schema.
  # The dict schema has no equivalent, so full JSON Schema is required.
  @tool(
      "convert_units",
      "Convert a value from one unit to another",
      {
          "type": "object",
          "properties": {
              "unit_type": {
                  "type": "string",
                  "enum": ["length", "temperature", "weight"],
                  "description": "Category of unit",
              },
              "from_unit": {
                  "type": "string",
                  "description": "Unit to convert from, e.g. kilometers, fahrenheit, pounds",
              },
              "to_unit": {"type": "string", "description": "Unit to convert to"},
              "value": {"type": "number", "description": "Value to convert"},
          },
          "required": ["unit_type", "from_unit", "to_unit", "value"],
      },
  )
  async def convert_units(args: dict[str, Any]) -> dict[str, Any]:
      conversions = {
          "length": {
              "kilometers_to_miles": lambda v: v * 0.621371,
              "miles_to_kilometers": lambda v: v * 1.60934,
              "meters_to_feet": lambda v: v * 3.28084,
              "feet_to_meters": lambda v: v * 0.3048,
          },
          "temperature": {
              "celsius_to_fahrenheit": lambda v: (v * 9) / 5 + 32,
              "fahrenheit_to_celsius": lambda v: (v - 32) * 5 / 9,
              "celsius_to_kelvin": lambda v: v + 273.15,
              "kelvin_to_celsius": lambda v: v - 273.15,
          },
          "weight": {
              "kilograms_to_pounds": lambda v: v * 2.20462,
              "pounds_to_kilograms": lambda v: v * 0.453592,
              "grams_to_ounces": lambda v: v * 0.035274,
              "ounces_to_grams": lambda v: v * 28.3495,
          },
      }

      key = f"{args['from_unit']}_to_{args['to_unit']}"
      fn = conversions.get(args["unit_type"], {}).get(key)

      if not fn:
          return {
              "content": [
                  {
                      "type": "text",
                      "text": f"Unsupported conversion: {args['from_unit']} to {args['to_unit']}",
                  }
              ],
              "is_error": True,
          }

      result = fn(args["value"])
      return {
          "content": [
              {
                  "type": "text",
                  "text": f"{args['value']} {args['from_unit']} = {result:.4f} {args['to_unit']}",
              }
          ]
      }


  converter_server = create_sdk_mcp_server(
      name="converter",
      version="1.0.0",
      tools=[convert_units],
  )
  ```

  ```typescript TypeScript theme={null}
  import { tool, createSdkMcpServer } from "@anthropic-ai/claude-agent-sdk";
  import { z } from "zod";

  const convert = tool(
    "convert_units",
    "Convert a value from one unit to another",
    {
      unit_type: z.enum(["length", "temperature", "weight"]).describe("Category of unit"),
      from_unit: z
        .string()
        .describe("Unit to convert from, e.g. kilometers, fahrenheit, pounds"),
      to_unit: z.string().describe("Unit to convert to"),
      value: z.number().describe("Value to convert")
    },
    async (args) => {
      type Conversions = Record<string, Record<string, (v: number) => number>>;

      const conversions: Conversions = {
        length: {
          kilometers_to_miles: (v) => v * 0.621371,
          miles_to_kilometers: (v) => v * 1.60934,
          meters_to_feet: (v) => v * 3.28084,
          feet_to_meters: (v) => v * 0.3048
        },
        temperature: {
          celsius_to_fahrenheit: (v) => (v * 9) / 5 + 32,
          fahrenheit_to_celsius: (v) => ((v - 32) * 5) / 9,
          celsius_to_kelvin: (v) => v + 273.15,
          kelvin_to_celsius: (v) => v - 273.15
        },
        weight: {
          kilograms_to_pounds: (v) => v * 2.20462,
          pounds_to_kilograms: (v) => v * 0.453592,
          grams_to_ounces: (v) => v * 0.035274,
          ounces_to_grams: (v) => v * 28.3495
        }
      };

      const key = `${args.from_unit}_to_${args.to_unit}`;
      const fn = conversions[args.unit_type]?.[key];

      if (!fn) {
        return {
          content: [
            {
              type: "text",
              text: `Unsupported conversion: ${args.from_unit} to ${args.to_unit}`
            }
          ],
          isError: true
        };
      }

      const result = fn(args.value);
      return {
        content: [
          {
            type: "text",
            text: `${args.value} ${args.from_unit} = ${result.toFixed(4)} ${args.to_unit}`
          }
        ]
      };
    }
  );

  const converterServer = createSdkMcpServer({
    name: "converter",
    version: "1.0.0",
    tools: [convert]
  });
  ```
</CodeGroup>

Setelah server didefinisikan, teruskan ke `query` dengan cara yang sama seperti contoh cuaca. Contoh ini mengirimkan tiga prompt berbeda dalam loop untuk menunjukkan alat yang sama menangani jenis satuan yang berbeda. Untuk setiap respons, ia memeriksa objek `AssistantMessage` (yang berisi panggilan alat yang dibuat Claude selama giliran itu) dan mencetak setiap `ToolUseBlock` sebelum mencetak teks `ResultMessage` akhir. Ini memungkinkan Anda melihat kapan Claude menggunakan alat versus menjawab dari pengetahuannya sendiri.

Karena [tool search](/docs/id/agent-sdk/tool-search) aktif secara default, output juga dapat mencakup panggilan `ToolSearch` saat Claude memuat skema alat yang ditunda.

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import (
      query,
      ClaudeAgentOptions,
      ResultMessage,
      AssistantMessage,
      ToolUseBlock,
  )


  async def main():
      options = ClaudeAgentOptions(
          mcp_servers={"converter": converter_server},
          allowed_tools=["mcp__converter__convert_units"],
      )

      prompts = [
          "Convert 100 kilometers to miles.",
          "What is 72°F in Celsius?",
          "How many pounds is 5 kilograms?",
      ]

      for prompt in prompts:
          try:
              async for message in query(prompt=prompt, options=options):
                  if isinstance(message, AssistantMessage):
                      for block in message.content:
                          if isinstance(block, ToolUseBlock):
                              print(f"[tool call] {block.name}({block.input})")
                  elif isinstance(message, ResultMessage) and message.subtype == "success":
                      print(f"Q: {prompt}\nA: {message.result}\n")
          except Exception as error:
              # A single-shot query() raises after yielding an error result. Only success
              # results are printed above, so handle the failure here and continue with
              # the next prompt.
              print(f"Call failed: {error}")


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const prompts = [
    "Convert 100 kilometers to miles.",
    "What is 72°F in Celsius?",
    "How many pounds is 5 kilograms?"
  ];

  for (const prompt of prompts) {
    try {
      for await (const message of query({
        prompt,
        options: {
          mcpServers: { converter: converterServer },
          allowedTools: ["mcp__converter__convert_units"]
        }
      })) {
        if (message.type === "assistant") {
          for (const block of message.message.content) {
            if (block.type === "tool_use") {
              console.log(`[tool call] ${block.name}`, block.input);
            }
          }
        } else if (message.type === "result" && message.subtype === "success") {
          console.log(`Q: ${prompt}\nA: ${message.result}\n`);
        }
      }
    } catch (error) {
      // A single-shot query() throws after yielding an error result. Only success
      // results are logged above, so handle the failure here and continue with
      // the next prompt.
      console.error(`Call failed: ${error}`);
    }
  }
  ```
</CodeGroup>

<h2 id="next-steps">
  Langkah Berikutnya
</h2>

Anda dapat menggabungkan pola-pola di halaman ini dalam server yang sama: satu server dapat menampung alat database, alat gateway API, dan renderer gambar bersama-sama.

Dari sini:

* Jika server Anda berkembang menjadi puluhan alat, lihat [pencarian alat](/docs/id/agent-sdk/tool-search) untuk menunda pemuatan hingga Claude membutuhkannya.
* Untuk terhubung ke server MCP eksternal (filesystem, GitHub, Slack) alih-alih membangun server Anda sendiri, lihat [Hubungkan server MCP](/docs/id/agent-sdk/mcp).
* Untuk mengontrol alat mana yang berjalan secara otomatis versus memerlukan persetujuan, lihat [Konfigurasi izin](/docs/id/agent-sdk/permissions).
