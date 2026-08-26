> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Menangani persetujuan dan input pengguna

> Tampilkan permintaan persetujuan Claude dan pertanyaan klarifikasi kepada pengguna, kemudian kembalikan keputusan mereka ke SDK.

Saat mengerjakan tugas, Claude kadang-kadang perlu berkonsultasi dengan pengguna. Mungkin perlu izin sebelum menghapus file, atau perlu menanyakan database mana yang akan digunakan untuk proyek baru. Aplikasi Anda perlu menampilkan permintaan ini kepada pengguna sehingga Claude dapat melanjutkan dengan input mereka.

Claude meminta input pengguna dalam dua situasi: ketika membutuhkan **izin untuk menggunakan alat** (seperti menghapus file atau menjalankan perintah), dan ketika memiliki **pertanyaan klarifikasi** (melalui alat `AskUserQuestion`). Keduanya memicu callback `canUseTool` Anda, yang menghentikan eksekusi sampai Anda mengembalikan respons. Ini berbeda dari putaran percakapan normal di mana Claude selesai dan menunggu pesan berikutnya Anda.

Untuk pertanyaan klarifikasi, Claude menghasilkan pertanyaan dan opsi. Peran Anda adalah menyajikannya kepada pengguna dan mengembalikan pilihan mereka. Anda tidak dapat menambahkan pertanyaan Anda sendiri ke alur ini; jika Anda perlu menanyakan sesuatu kepada pengguna, lakukan itu secara terpisah dalam logika aplikasi Anda.

Callback dapat tetap tertunda tanpa batas waktu. Eksekusi tetap dijeda sampai callback Anda kembali, dan SDK hanya membatalkan tunggu ketika kueri itu sendiri dibatalkan. Jika pengguna mungkin membutuhkan waktu lebih lama untuk merespons daripada yang dapat ditahan proses Anda, kembalikan keputusan [`defer` hook](/docs/id/hooks#defer-a-tool-call-for-later), yang memungkinkan proses keluar dan dilanjutkan nanti dari sesi yang disimpan.

Panduan ini menunjukkan cara mendeteksi setiap jenis permintaan dan merespons dengan tepat.

<h2 id="detect-when-claude-needs-input">
  Deteksi ketika Claude membutuhkan input
</h2>

Berikan callback `canUseTool` dalam opsi kueri Anda. Callback dipicu setiap kali Claude membutuhkan input pengguna, menerima nama alat dan input sebagai argumen:

<CodeGroup>
  ```python Python theme={null}
  async def handle_tool_request(tool_name, input_data, context):
      # Minta pengguna dan kembalikan izin atau tolak
      ...


  options = ClaudeAgentOptions(can_use_tool=handle_tool_request)
  ```

  ```typescript TypeScript theme={null}
  async function handleToolRequest(toolName, input, options) {
    // options includes { signal: AbortSignal, suggestions?: PermissionUpdate[] }
    // Minta pengguna dan kembalikan izin atau tolak
  }

  const options = { canUseTool: handleToolRequest };
  ```
</CodeGroup>

Callback dipicu dalam dua kasus:

1. **Alat membutuhkan persetujuan**: Claude ingin menggunakan alat yang tidak disetujui secara otomatis oleh [aturan izin](/docs/id/agent-sdk/permissions) atau mode izin. Periksa `tool_name` untuk alat (misalnya, `"Bash"`, `"Write"`).
2. **Claude mengajukan pertanyaan**: Claude memanggil alat `AskUserQuestion`. Periksa apakah `tool_name == "AskUserQuestion"` untuk menanganinya secara berbeda. Jika Anda menentukan array `tools`, sertakan `AskUserQuestion` agar ini berfungsi. Lihat [Menangani pertanyaan klarifikasi](#handle-clarifying-questions) untuk detail.

<Warning>
  **Callback tidak pernah dipicu untuk alat yang disetujui secara otomatis.** Setiap persetujuan sebelumnya dalam [alur evaluasi izin](/docs/id/agent-sdk/permissions#how-permissions-are-evaluated), aturan izin atau mode seperti `acceptEdits` atau `bypassPermissions`, menyelesaikan panggilan sebelum `canUseTool` dikonsultasikan. Jika Anda mencantumkan alat secara langsung dalam `allowed_tools`, pemeriksaan `canUseTool` untuk alat tersebut tidak pernah berjalan kecuali aturan ask atau mode `plan` mengarahkan panggilan kembali ke prompt. Untuk logika yang harus diterapkan ke setiap panggilan alat, gunakan [hook `PreToolUse`](/docs/id/agent-sdk/hooks), yang dijalankan sebelum sisa alur dan dapat mengizinkan, menolak, atau memodifikasi permintaan.

  `AskUserQuestion`, alat MCP yang ditandai [`requiresUserInteraction`](/docs/id/mcp#require-approval-for-a-specific-tool), dan alat konektor [yang organisasi Anda atur ke `ask`](/docs/id/mcp#organization-controls-on-connector-tools) mencapai callback bahkan ketika aturan izin cocok. Dalam mode `dontAsk` panggilan ini ditolak sebagai gantinya, tanpa memanggil callback.
</Warning>

Anda juga dapat menggunakan [hook `PermissionRequest`](/docs/id/agent-sdk/hooks#available-hooks) untuk mengirim notifikasi eksternal (Slack, email, push) ketika Claude menunggu persetujuan.

<h2 id="handle-tool-approval-requests">
  Menangani permintaan persetujuan alat
</h2>

Setelah Anda melewatkan callback `canUseTool` dalam opsi kueri Anda, callback dipicu ketika Claude ingin menggunakan alat yang tidak disetujui secara otomatis. Callback Anda menerima tiga argumen:

| Argumen                             | Deskripsi                                                                                                                                                                                                                                                                                                                                                         |
| ----------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `toolName`                          | Nama alat yang ingin digunakan Claude (misalnya, `"Bash"`, `"Write"`, `"Edit"`)                                                                                                                                                                                                                                                                                   |
| `input`                             | Parameter yang diteruskan Claude ke alat. Isi bervariasi menurut alat.                                                                                                                                                                                                                                                                                            |
| `options` (TS) / `context` (Python) | Konteks tambahan termasuk `suggestions` opsional (entri `PermissionUpdate` yang diusulkan untuk menghindari permintaan ulang) dan sinyal pembatalan. Di TypeScript, `signal` adalah `AbortSignal`; di Python, bidang sinyal dicadangkan untuk penggunaan di masa depan. Lihat [`ToolPermissionContext`](/docs/id/agent-sdk/python#toolpermissioncontext) untuk Python. |

Objek `input` berisi parameter khusus alat. Contoh umum:

| Alat    | Bidang input                            |
| ------- | --------------------------------------- |
| `Bash`  | `command`, `description`, `timeout`     |
| `Write` | `file_path`, `content`                  |
| `Edit`  | `file_path`, `old_string`, `new_string` |
| `Read`  | `file_path`, `offset`, `limit`          |

Lihat referensi SDK untuk skema input lengkap: [Python](/docs/id/agent-sdk/python#tool-input%2Foutput-types) | [TypeScript](/docs/id/agent-sdk/typescript#tool-input-types).

Anda dapat menampilkan informasi ini kepada pengguna sehingga mereka dapat memutuskan apakah akan mengizinkan atau menolak tindakan, kemudian kembalikan respons yang sesuai.

Contoh berikut meminta Claude untuk membuat dan menghapus file uji. Ketika Claude mencoba setiap operasi, callback mencetak permintaan alat ke terminal dan meminta persetujuan y/n.

<CodeGroup>
  ```python Python theme={null}
  import asyncio

  from claude_agent_sdk import ClaudeAgentOptions, ResultMessage, query
  from claude_agent_sdk.types import (
      HookMatcher,
      PermissionResultAllow,
      PermissionResultDeny,
      ToolPermissionContext,
  )


  async def can_use_tool(
      tool_name: str, input_data: dict, context: ToolPermissionContext
  ) -> PermissionResultAllow | PermissionResultDeny:
      # Tampilkan permintaan alat
      print(f"\nTool: {tool_name}")
      if tool_name == "Bash":
          print(f"Command: {input_data.get('command')}")
          if input_data.get("description"):
              print(f"Description: {input_data.get('description')}")
      else:
          print(f"Input: {input_data}")

      # Dapatkan persetujuan pengguna
      response = input("Allow this action? (y/n): ")

      # Kembalikan izin atau tolak berdasarkan respons pengguna
      if response.lower() == "y":
          # Izinkan: alat dijalankan dengan input asli (atau dimodifikasi)
          return PermissionResultAllow(updated_input=input_data)
      else:
          # Tolak: alat tidak dijalankan, Claude melihat pesan
          return PermissionResultDeny(message="User denied this action")


  # Solusi yang diperlukan: hook dummy menjaga aliran tetap terbuka untuk can_use_tool
  async def dummy_hook(input_data, tool_use_id, context):
      return {"continue_": True}


  async def prompt_stream():
      yield {
          "type": "user",
          "message": {
              "role": "user",
              "content": "Create a test file in /tmp and then delete it",
          },
      }


  async def main():
      async for message in query(
          prompt=prompt_stream(),
          options=ClaudeAgentOptions(
              can_use_tool=can_use_tool,
              hooks={"PreToolUse": [HookMatcher(matcher=None, hooks=[dummy_hook])]},
          ),
      ):
          if isinstance(message, ResultMessage) and message.subtype == "success":
              print(message.result)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";
  import * as readline from "readline";

  // Helper untuk meminta input pengguna di terminal
  function prompt(question: string): Promise<string> {
    const rl = readline.createInterface({
      input: process.stdin,
      output: process.stdout
    });
    return new Promise((resolve) =>
      rl.question(question, (answer) => {
        rl.close();
        resolve(answer);
      })
    );
  }

  for await (const message of query({
    prompt: "Create a test file in /tmp and then delete it",
    options: {
      canUseTool: async (toolName, input) => {
        // Tampilkan permintaan alat
        console.log(`\nTool: ${toolName}`);
        if (toolName === "Bash") {
          console.log(`Command: ${input.command}`);
          if (input.description) console.log(`Description: ${input.description}`);
        } else {
          console.log(`Input: ${JSON.stringify(input, null, 2)}`);
        }

        // Dapatkan persetujuan pengguna
        const response = await prompt("Allow this action? (y/n): ");

        // Kembalikan izin atau tolak berdasarkan respons pengguna
        if (response.toLowerCase() === "y") {
          // Izinkan: alat dijalankan dengan input asli (atau dimodifikasi)
          return { behavior: "allow", updatedInput: input };
        } else {
          // Tolak: alat tidak dijalankan, Claude melihat pesan
          return { behavior: "deny", message: "User denied this action" };
        }
      }
    }
  })) {
    if ("result" in message) console.log(message.result);
  }
  ```
</CodeGroup>

<Note>
  Di Python, `can_use_tool` memerlukan [mode streaming](/docs/id/agent-sdk/streaming-vs-single-mode). Ketika Anda melewatkan aliran pesan terbatas melalui `query(prompt=generator)` atau `ClaudeSDKClient.connect(prompt=async_iterable)`, SDK menutup aliran input setelah pesan terakhir, sebelum callback izin dapat dipanggil, kecuali hook terdaftar atau server MCP dalam proses menjaganya tetap terbuka. Contoh di atas menjaganya tetap terbuka dengan hook `PreToolUse` yang mengembalikan `{"continue_": True}`. Menghubungkan tanpa prompt dan mengirim pesan melalui `ClaudeSDKClient.query()` menjaga aliran tetap terbuka dengan sendirinya dan tidak memerlukan hook.
</Note>

Contoh ini menggunakan alur `y/n` di mana input apa pun selain `y` diperlakukan sebagai penolakan. Dalam praktik, Anda mungkin membangun UI yang lebih kaya yang memungkinkan pengguna memodifikasi permintaan, memberikan umpan balik, atau mengarahkan Claude sepenuhnya. Lihat [Merespons permintaan alat](#respond-to-tool-requests) untuk semua cara Anda dapat merespons.

<h3 id="respond-to-tool-requests">
  Merespons permintaan alat
</h3>

Callback Anda mengembalikan salah satu dari dua jenis respons:

| Respons     | Python                                     | TypeScript                            |
| ----------- | ------------------------------------------ | ------------------------------------- |
| **Izinkan** | `PermissionResultAllow(updated_input=...)` | `{ behavior: "allow", updatedInput }` |
| **Tolak**   | `PermissionResultDeny(message=...)`        | `{ behavior: "deny", message }`       |

Saat mengizinkan, alat berjalan dengan input yang diminta Claude kecuali Anda mengembalikan input yang dimodifikasi, `updatedInput` di TypeScript atau `updated_input` di Python. Sebelum v2.1.207, Claude Code menolak hasil izin yang menghilangkan `updatedInput` dan menolak panggilan alat dengan kesalahan validasi.

Saat menolak, berikan pesan yang menjelaskan alasannya. Claude melihat pesan ini dan mungkin menyesuaikan pendekatannya.

<CodeGroup>
  ```python Python theme={null}
  from claude_agent_sdk.types import PermissionResultAllow, PermissionResultDeny

  # Izinkan alat untuk dijalankan
  return PermissionResultAllow(updated_input=input_data)

  # Blokir alat
  return PermissionResultDeny(message="User rejected this action")
  ```

  ```typescript TypeScript theme={null}
  // Izinkan alat untuk dijalankan
  return { behavior: "allow", updatedInput: input };

  // Blokir alat
  return { behavior: "deny", message: "User rejected this action" };
  ```
</CodeGroup>

Selain mengizinkan atau menolak, Anda dapat memodifikasi input alat atau memberikan konteks yang membantu Claude menyesuaikan pendekatannya:

* **Setujui**: biarkan alat dijalankan seperti yang diminta Claude
* **Setujui dengan perubahan**: modifikasi input sebelum eksekusi (misalnya, sanitasi jalur, tambahkan batasan)
* **Setujui dan ingat**: kembalikan aturan izin yang disarankan sehingga panggilan yang cocok melewati prompt berikutnya
* **Tolak**: blokir alat dan beri tahu Claude mengapa
* **Sarankan alternatif**: blokir tetapi arahkan Claude ke arah yang diinginkan pengguna
* **Alihkan sepenuhnya**: gunakan [input streaming](/docs/id/agent-sdk/streaming-vs-single-mode) untuk mengirim Claude instruksi yang sepenuhnya baru

<Tabs>
  <Tab title="Setujui">
    Pengguna menyetujui tindakan apa adanya. Teruskan `input` dari callback Anda tanpa perubahan dan alat dijalankan persis seperti yang diminta Claude.

    <CodeGroup>
      ```python Python theme={null}
      async def can_use_tool(tool_name, input_data, context):
          print(f"Claude wants to use {tool_name}")
          approved = await ask_user("Allow this action?")

          if approved:
              return PermissionResultAllow(updated_input=input_data)
          return PermissionResultDeny(message="User declined")
      ```

      ```typescript TypeScript theme={null}
      canUseTool: async (toolName, input) => {
        console.log(`Claude wants to use ${toolName}`);
        const approved = await askUser("Allow this action?");

        if (approved) {
          return { behavior: "allow", updatedInput: input };
        }
        return { behavior: "deny", message: "User declined" };
      };
      ```
    </CodeGroup>
  </Tab>

  <Tab title="Setujui dengan perubahan">
    Pengguna menyetujui tetapi ingin memodifikasi permintaan terlebih dahulu. Anda dapat mengubah input sebelum alat dijalankan. Claude melihat hasilnya tetapi tidak diberitahu Anda mengubah apa pun. Berguna untuk sanitasi parameter, menambahkan batasan, atau membatasi akses.

    <CodeGroup>
      ```python Python theme={null}
      async def can_use_tool(tool_name, input_data, context):
          if tool_name == "Bash":
              # Pengguna menyetujui, tetapi batasi semua perintah ke sandbox
              sandboxed_input = {**input_data}
              sandboxed_input["command"] = input_data["command"].replace(
                  "/tmp", "/tmp/sandbox"
              )
              return PermissionResultAllow(updated_input=sandboxed_input)
          return PermissionResultAllow(updated_input=input_data)
      ```

      ```typescript TypeScript theme={null}
      canUseTool: async (toolName, input) => {
        if (toolName === "Bash") {
          // Pengguna menyetujui, tetapi batasi semua perintah ke sandbox
          const sandboxedInput = {
            ...input,
            command: input.command.replace("/tmp", "/tmp/sandbox")
          };
          return { behavior: "allow", updatedInput: sandboxedInput };
        }
        return { behavior: "allow", updatedInput: input };
      };
      ```
    </CodeGroup>
  </Tab>

  <Tab title="Setujui dan ingat">
    Pengguna menyetujui dan tidak ingin ditanya lagi untuk jenis panggilan ini. Argumen callback ketiga membawa `suggestions`, sebuah array entri [`PermissionUpdate`](/docs/id/agent-sdk/typescript#permissionupdate) yang sudah jadi. Kembalikan satu di `updatedPermissions` untuk menerapkannya. Saran dengan tujuan `localSettings` menulis aturan ke `.claude/settings.local.json` sehingga sesi mendatang melewati prompt untuk panggilan yang cocok.

    Contoh Python memerlukan `claude-agent-sdk` 0.1.80 atau lebih baru.

    <CodeGroup>
      ```python Python theme={null}
      async def can_use_tool(tool_name, input_data, context):
          choice = await ask_user(f"Allow {tool_name}?", ["once", "always", "no"])

          if choice == "always":
              persist = [
                  s for s in context.suggestions if s.destination == "localSettings"
              ]
              return PermissionResultAllow(
                  updated_input=input_data, updated_permissions=persist
              )
          if choice == "once":
              return PermissionResultAllow(updated_input=input_data)
          return PermissionResultDeny(message="User declined")
      ```

      ```typescript TypeScript theme={null}
      canUseTool: async (toolName, input, { suggestions = [] }) => {
        const choice = await askUser(`Allow ${toolName}?`, ["once", "always", "no"]);

        if (choice === "always") {
          const persist = suggestions.filter(
            (s) => s.destination === "localSettings"
          );
          return {
            behavior: "allow",
            updatedInput: input,
            updatedPermissions: persist
          };
        }
        if (choice === "once") {
          return { behavior: "allow", updatedInput: input };
        }
        return { behavior: "deny", message: "User declined" };
      };
      ```
    </CodeGroup>
  </Tab>

  <Tab title="Tolak">
    Pengguna tidak menginginkan tindakan ini terjadi. Blokir alat dan berikan pesan yang menjelaskan alasannya. Claude melihat pesan ini dan mungkin mencoba pendekatan yang berbeda.

    <CodeGroup>
      ```python Python theme={null}
      async def can_use_tool(tool_name, input_data, context):
          approved = await ask_user(f"Allow {tool_name}?")

          if not approved:
              return PermissionResultDeny(message="User rejected this action")
          return PermissionResultAllow(updated_input=input_data)
      ```

      ```typescript TypeScript theme={null}
      canUseTool: async (toolName, input) => {
        const approved = await askUser(`Allow ${toolName}?`);

        if (!approved) {
          return {
            behavior: "deny",
            message: "User rejected this action"
          };
        }
        return { behavior: "allow", updatedInput: input };
      };
      ```
    </CodeGroup>
  </Tab>

  <Tab title="Sarankan alternatif">
    Pengguna tidak menginginkan tindakan spesifik ini, tetapi memiliki ide yang berbeda. Blokir alat dan sertakan panduan dalam pesan Anda. Claude akan membaca ini dan memutuskan cara melanjutkan berdasarkan umpan balik Anda.

    <CodeGroup>
      ```python Python theme={null}
      async def can_use_tool(tool_name, input_data, context):
          if tool_name == "Bash" and "rm" in input_data.get("command", ""):
              # Pengguna tidak ingin menghapus, sarankan pengarsipan sebagai gantinya
              return PermissionResultDeny(
                  message="User doesn't want to delete files. They asked if you could compress them into an archive instead."
              )
          return PermissionResultAllow(updated_input=input_data)
      ```

      ```typescript TypeScript theme={null}
      canUseTool: async (toolName, input) => {
        if (toolName === "Bash" && input.command.includes("rm")) {
          // Pengguna tidak ingin menghapus, sarankan pengarsipan sebagai gantinya
          return {
            behavior: "deny",
            message:
              "User doesn't want to delete files. They asked if you could compress them into an archive instead."
          };
        }
        return { behavior: "allow", updatedInput: input };
      };
      ```
    </CodeGroup>
  </Tab>

  <Tab title="Alihkan sepenuhnya">
    Untuk perubahan arah yang lengkap (bukan hanya dorongan), gunakan [input streaming](/docs/id/agent-sdk/streaming-vs-single-mode) untuk mengirim Claude instruksi baru secara langsung. Ini melewati permintaan alat saat ini dan memberikan Claude instruksi yang sepenuhnya baru untuk diikuti.
  </Tab>
</Tabs>

<h2 id="handle-clarifying-questions">
  Menangani pertanyaan klarifikasi
</h2>

Ketika Claude membutuhkan lebih banyak arahan tentang tugas dengan beberapa pendekatan yang valid, Claude memanggil alat `AskUserQuestion`. Ini memicu callback `canUseTool` Anda dengan `toolName` diatur ke `AskUserQuestion`. Input berisi pertanyaan Claude sebagai opsi pilihan ganda, yang Anda tampilkan kepada pengguna dan kembalikan pilihan mereka.

<Tip>
  Pertanyaan klarifikasi sangat umum dalam [mode `plan`](/docs/id/agent-sdk/permissions#plan-mode-plan), di mana Claude menjelajahi basis kode dan mengajukan pertanyaan sebelum mengusulkan rencana. Ini membuat mode plan ideal untuk alur kerja interaktif di mana Anda ingin Claude mengumpulkan persyaratan sebelum membuat perubahan.
</Tip>

Langkah-langkah berikut menunjukkan cara menangani pertanyaan klarifikasi:

<Steps>
  <Step title="Berikan callback canUseTool">
    Berikan callback `canUseTool` dalam opsi kueri Anda. Secara default, `AskUserQuestion` tersedia. Jika Anda menentukan array `tools` untuk membatasi kemampuan Claude (misalnya, agen read-only dengan hanya `Read`, `Glob`, dan `Grep`), sertakan `AskUserQuestion` dalam array itu. Jika tidak, Claude tidak akan dapat mengajukan pertanyaan klarifikasi:

    <CodeGroup>
      ```python Python theme={null}
      async for message in query(
          prompt="Analyze this codebase",
          options=ClaudeAgentOptions(
              # Sertakan AskUserQuestion dalam daftar alat Anda
              tools=["Read", "Glob", "Grep", "AskUserQuestion"],
              can_use_tool=can_use_tool,
          ),
      ):
          print(message)
      ```

      ```typescript TypeScript theme={null}
      for await (const message of query({
        prompt: "Analyze this codebase",
        options: {
          // Sertakan AskUserQuestion dalam daftar alat Anda
          tools: ["Read", "Glob", "Grep", "AskUserQuestion"],
          canUseTool: async (toolName, input) => {
            // Tangani pertanyaan klarifikasi di sini
          }
        }
      })) {
        console.log(message);
      }
      ```
    </CodeGroup>
  </Step>

  <Step title="Deteksi AskUserQuestion">
    Dalam callback Anda, periksa apakah `toolName` sama dengan `AskUserQuestion` untuk menanganinya secara berbeda dari alat lain:

    <CodeGroup>
      ```python Python theme={null}
      async def can_use_tool(tool_name: str, input_data: dict, context):
          if tool_name == "AskUserQuestion":
              # Implementasi Anda untuk mengumpulkan jawaban dari pengguna
              return await handle_clarifying_questions(input_data)
          # Tangani alat lain secara normal
          return await prompt_for_approval(tool_name, input_data)
      ```

      ```typescript TypeScript theme={null}
      canUseTool: async (toolName, input) => {
        if (toolName === "AskUserQuestion") {
          // Implementasi Anda untuk mengumpulkan jawaban dari pengguna
          return handleClarifyingQuestions(input);
        }
        // Tangani alat lain secara normal
        return promptForApproval(toolName, input);
      };
      ```
    </CodeGroup>
  </Step>

  <Step title="Analisis input pertanyaan">
    Input berisi pertanyaan Claude dalam array `questions`. Setiap pertanyaan memiliki `question` (teks untuk ditampilkan), `options` (pilihan), dan `multiSelect` (apakah beberapa pilihan diizinkan):

    ```json theme={null}
    {
      "questions": [
        {
          "question": "How should I format the output?",
          "header": "Format",
          "options": [
            { "label": "Summary", "description": "Brief overview" },
            { "label": "Detailed", "description": "Full explanation" }
          ],
          "multiSelect": false
        },
        {
          "question": "Which sections should I include?",
          "header": "Sections",
          "options": [
            { "label": "Introduction", "description": "Opening context" },
            { "label": "Conclusion", "description": "Final summary" }
          ],
          "multiSelect": true
        }
      ]
    }
    ```

    Lihat [Format pertanyaan](#question-format) untuk deskripsi bidang lengkap.
  </Step>

  <Step title="Kumpulkan jawaban dari pengguna">
    Presentasikan pertanyaan kepada pengguna dan kumpulkan pilihan mereka. Cara Anda melakukan ini tergantung pada aplikasi Anda: prompt terminal, formulir web, dialog seluler, dll.
  </Step>

  <Step title="Kembalikan jawaban ke Claude">
    Bangun objek `answers` sebagai catatan di mana setiap kunci adalah teks `question` dan setiap nilai adalah `label` opsi yang dipilih:

    | Dari objek pertanyaan                                             | Gunakan sebagai |
    | ----------------------------------------------------------------- | --------------- |
    | Bidang `question` (misalnya, `"How should I format the output?"`) | Kunci           |
    | Bidang `label` opsi yang dipilih (misalnya, `"Summary"`)          | Nilai           |

    Untuk pertanyaan multi-pilih, berikan array label atau gabungkan dengan `", "`. Jika Anda [mendukung input teks bebas](#support-free-text-input), gunakan teks kustom pengguna sebagai nilai.

    <CodeGroup>
      ```python Python theme={null}
      return PermissionResultAllow(
          updated_input={
              "questions": input_data.get("questions", []),
              "answers": {
                  "How should I format the output?": "Summary",
                  "Which sections should I include?": ["Introduction", "Conclusion"],
              },
          }
      )
      ```

      ```typescript TypeScript theme={null}
      return {
        behavior: "allow",
        updatedInput: {
          questions: input.questions,
          answers: {
            "How should I format the output?": "Summary",
            "Which sections should I include?": "Introduction, Conclusion"
          }
        }
      };
      ```
    </CodeGroup>
  </Step>
</Steps>

<h3 id="question-format">
  Format pertanyaan
</h3>

Input berisi pertanyaan yang dihasilkan Claude dalam array `questions`. Setiap pertanyaan memiliki bidang-bidang ini:

| Bidang        | Deskripsi                                                                                                                                                |
| ------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `question`    | Teks pertanyaan lengkap untuk ditampilkan                                                                                                                |
| `header`      | Label pendek untuk pertanyaan (maks 12 karakter)                                                                                                         |
| `options`     | Array 2-4 pilihan, masing-masing dengan `label` dan `description`. TypeScript: secara opsional `preview` (lihat [di bawah](#option-previews-typescript)) |
| `multiSelect` | Jika `true`, pengguna dapat memilih beberapa opsi                                                                                                        |

Struktur yang diterima callback Anda:

```json theme={null}
{
  "questions": [
    {
      "question": "How should I format the output?",
      "header": "Format",
      "options": [
        { "label": "Summary", "description": "Brief overview of key points" },
        { "label": "Detailed", "description": "Full explanation with examples" }
      ],
      "multiSelect": false
    }
  ]
}
```

<h4 id="option-previews-typescript">
  Pratinjau opsi (TypeScript)
</h4>

`toolConfig.askUserQuestion.previewFormat` menambahkan bidang `preview` ke setiap opsi sehingga aplikasi Anda dapat menampilkan mockup visual di samping label. Tanpa pengaturan ini, Claude tidak menghasilkan pratinjau dan bidang tidak ada.

| `previewFormat`        | `preview` berisi                                                                                             |
| :--------------------- | :----------------------------------------------------------------------------------------------------------- |
| tidak diatur (default) | Bidang tidak ada. Claude tidak menghasilkan pratinjau.                                                       |
| `"markdown"`           | ASCII art dan blok kode yang dibatasi                                                                        |
| `"html"`               | Fragmen `<div>` bergaya (SDK menolak `<script>`, `<style>`, dan `<!DOCTYPE>` sebelum callback Anda berjalan) |

Format berlaku untuk semua pertanyaan dalam sesi. Claude menyertakan `preview` pada opsi di mana perbandingan visual membantu (pilihan tata letak, skema warna) dan menghilangkannya di mana tidak akan membantu (konfirmasi ya/tidak, pilihan hanya teks). Periksa `undefined` sebelum merender.

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

for await (const message of query({
  prompt: "Help me choose a card layout",
  options: {
    toolConfig: {
      askUserQuestion: { previewFormat: "html" }
    },
    canUseTool: async (toolName, input) => {
      // input.questions[].options[].preview adalah string HTML atau undefined
      return { behavior: "allow", updatedInput: input };
    }
  }
})) {
  // ...
}
```

Opsi dengan pratinjau HTML:

```json theme={null}
{
  "label": "Compact",
  "description": "Title and metric value only",
  "preview": "<div style=\"padding:12px;border:1px solid #ddd;border-radius:8px\"><div style=\"font-size:12px;color:#666\">Active users</div><div style=\"font-size:28px;font-weight:600\">1,284</div></div>"
}
```

<h3 id="response-format">
  Format respons
</h3>

Kembalikan objek `answers` yang memetakan bidang `question` setiap pertanyaan ke `label` opsi yang dipilih:

| Bidang      | Deskripsi                                                                                 |
| ----------- | ----------------------------------------------------------------------------------------- |
| `questions` | Teruskan array pertanyaan asli (diperlukan untuk pemrosesan alat)                         |
| `answers`   | Objek di mana kunci adalah teks pertanyaan dan nilai adalah label yang dipilih            |
| `response`  | Respons freeform opsional yang diketik pengguna alih-alih menjawab pertanyaan terstruktur |

Untuk pertanyaan multi-pilih, berikan array label atau gabungkan dengan `", "`. Untuk input teks bebas per-pertanyaan seperti opsi "Other", masukkan teks pengguna dalam `answers[question]` seperti yang ditunjukkan dalam [Dukung input teks bebas](#support-free-text-input). Atur `response` hanya ketika UI Anda memungkinkan pengguna untuk menutup kartu pertanyaan dan mengetik respons umum yang bukan jawaban untuk pertanyaan spesifik apa pun. Ketika `response` diatur, Claude menerima "The user responded: …" alih-alih daftar jawaban per-pertanyaan.

```json theme={null}
{
  "questions": [
    // ...
  ],
  "answers": {
    "How should I format the output?": "Summary",
    "Which sections should I include?": ["Introduction", "Conclusion"]
  }
}
```

<h4 id="support-free-text-input">
  Dukung input teks bebas
</h4>

Opsi yang telah ditentukan Claude tidak akan selalu mencakup apa yang diinginkan pengguna. Untuk memungkinkan pengguna mengetik jawaban mereka sendiri:

* Tampilkan pilihan "Other" tambahan setelah opsi Claude yang menerima input teks
* Gunakan teks kustom pengguna sebagai nilai jawaban (bukan kata "Other")

Lihat [contoh lengkap](#complete-example) di bawah untuk implementasi lengkap.

<h3 id="complete-example">
  Contoh lengkap
</h3>

Claude mengajukan pertanyaan klarifikasi ketika membutuhkan input pengguna untuk melanjutkan. Misalnya, ketika diminta membantu memutuskan tech stack untuk aplikasi seluler, Claude mungkin menanyakan tentang cross-platform vs native, preferensi backend, atau platform target. Pertanyaan-pertanyaan ini membantu Claude membuat keputusan yang sesuai dengan preferensi pengguna daripada menebak.

Contoh ini menangani pertanyaan-pertanyaan tersebut dalam aplikasi terminal. Berikut yang terjadi di setiap langkah:

1. **Arahkan permintaan**: Callback `canUseTool` memeriksa apakah nama alat adalah `"AskUserQuestion"` dan mengarahkan ke handler khusus
2. **Tampilkan pertanyaan**: Handler melakukan loop melalui array `questions` dan mencetak setiap pertanyaan dengan opsi bernomor
3. **Kumpulkan input**: Pengguna dapat memasukkan angka untuk memilih opsi, atau mengetik teks bebas langsung (misalnya, "jquery", "i don't know")
4. **Peta jawaban**: Kode memeriksa apakah input adalah numerik (menggunakan label opsi) atau teks bebas (menggunakan teks langsung)
5. **Kembalikan ke Claude**: Respons mencakup array `questions` asli dan pemetaan `answers`

Simpan versi TypeScript sebagai `ask.ts` dan jalankan dengan `npx tsx ask.ts`, atau simpan versi Python sebagai `ask.py` dan jalankan dengan `python ask.py`.

<CodeGroup>
  ```python Python theme={null}
  import asyncio

  from claude_agent_sdk import ClaudeAgentOptions, ResultMessage, query
  from claude_agent_sdk.types import HookMatcher, PermissionResultAllow


  def parse_response(response: str, options: list) -> str:
      """Analisis input pengguna sebagai nomor opsi atau teks bebas."""
      try:
          indices = [int(s.strip()) - 1 for s in response.split(",")]
          labels = [options[i]["label"] for i in indices if 0 <= i < len(options)]
          return ", ".join(labels) if labels else response
      except ValueError:
          return response


  async def handle_ask_user_question(input_data: dict) -> PermissionResultAllow:
      """Tampilkan pertanyaan Claude dan kumpulkan jawaban pengguna."""
      answers = {}

      for q in input_data.get("questions", []):
          print(f"\n{q['header']}: {q['question']}")

          options = q["options"]
          for i, opt in enumerate(options):
              print(f"  {i + 1}. {opt['label']} - {opt['description']}")
          if q.get("multiSelect"):
              print("  (Enter numbers separated by commas, or type your own answer)")
          else:
              print("  (Enter a number, or type your own answer)")

          response = input("Your choice: ").strip()
          answers[q["question"]] = parse_response(response, options)

      return PermissionResultAllow(
          updated_input={
              "questions": input_data.get("questions", []),
              "answers": answers,
          }
      )


  async def can_use_tool(
      tool_name: str, input_data: dict, context
  ) -> PermissionResultAllow:
      # Arahkan AskUserQuestion ke handler pertanyaan kami
      if tool_name == "AskUserQuestion":
          return await handle_ask_user_question(input_data)
      # Setujui secara otomatis alat lain untuk contoh ini
      return PermissionResultAllow(updated_input=input_data)


  async def prompt_stream():
      yield {
          "type": "user",
          "message": {
              "role": "user",
              "content": "Help me decide on the tech stack for a new mobile app",
          },
      }


  # Solusi yang diperlukan: hook dummy menjaga aliran tetap terbuka untuk can_use_tool
  async def dummy_hook(input_data, tool_use_id, context):
      return {"continue_": True}


  async def main():
      async for message in query(
          prompt=prompt_stream(),
          options=ClaudeAgentOptions(
              can_use_tool=can_use_tool,
              hooks={"PreToolUse": [HookMatcher(matcher=None, hooks=[dummy_hook])]},
          ),
      ):
          if isinstance(message, ResultMessage) and message.subtype == "success":
              print(message.result)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";
  import * as readline from "readline/promises";

  // Helper untuk meminta input pengguna di terminal
  async function prompt(question: string): Promise<string> {
    const rl = readline.createInterface({ input: process.stdin, output: process.stdout });
    const answer = await rl.question(question);
    rl.close();
    return answer;
  }

  // Analisis input pengguna sebagai nomor opsi atau teks bebas
  function parseResponse(response: string, options: any[]): string {
    const indices = response.split(",").map((s) => parseInt(s.trim()) - 1);
    const labels = indices
      .filter((i) => !isNaN(i) && i >= 0 && i < options.length)
      .map((i) => options[i].label);
    return labels.length > 0 ? labels.join(", ") : response;
  }

  // Tampilkan pertanyaan Claude dan kumpulkan jawaban pengguna
  async function handleAskUserQuestion(input: any) {
    const answers: Record<string, string> = {};

    for (const q of input.questions) {
      console.log(`\n${q.header}: ${q.question}`);

      const options = q.options;
      options.forEach((opt: any, i: number) => {
        console.log(`  ${i + 1}. ${opt.label} - ${opt.description}`);
      });
      if (q.multiSelect) {
        console.log("  (Enter numbers separated by commas, or type your own answer)");
      } else {
        console.log("  (Enter a number, or type your own answer)");
      }

      const response = (await prompt("Your choice: ")).trim();
      answers[q.question] = parseResponse(response, options);
    }

    // Kembalikan jawaban ke Claude (harus menyertakan pertanyaan asli)
    return {
      behavior: "allow",
      updatedInput: { questions: input.questions, answers }
    };
  }

  async function main() {
    for await (const message of query({
      prompt: "Help me decide on the tech stack for a new mobile app",
      options: {
        canUseTool: async (toolName, input) => {
          // Arahkan AskUserQuestion ke handler pertanyaan kami
          if (toolName === "AskUserQuestion") {
            return handleAskUserQuestion(input);
          }
          // Setujui secara otomatis alat lain untuk contoh ini
          return { behavior: "allow", updatedInput: input };
        }
      }
    })) {
      if ("result" in message) console.log(message.result);
    }
  }

  main();
  ```
</CodeGroup>

<h2 id="limitations">
  Keterbatasan
</h2>

* **Subagents**: `AskUserQuestion` saat ini tidak tersedia di subagents yang dihasilkan melalui alat Agent
* **Batas pertanyaan**: setiap panggilan `AskUserQuestion` mendukung 1-4 pertanyaan dengan 2-4 opsi masing-masing

<h2 id="other-ways-to-get-user-input">
  Cara lain untuk mendapatkan input pengguna
</h2>

Callback `canUseTool` dan alat `AskUserQuestion` mencakup sebagian besar skenario persetujuan dan klarifikasi, tetapi SDK menawarkan cara lain untuk mendapatkan input dari pengguna:

<h3 id="streaming-input">
  Input streaming
</h3>

Gunakan [input streaming](/docs/id/agent-sdk/streaming-vs-single-mode) ketika Anda perlu:

* **Mengganggu agen di tengah-tugas**: kirim sinyal pembatalan atau ubah arah saat Claude sedang bekerja
* **Memberikan konteks tambahan**: tambahkan informasi yang dibutuhkan Claude tanpa menunggu untuk ditanya
* **Membangun antarmuka obrolan**: biarkan pengguna mengirim pesan lanjutan selama operasi yang berjalan lama

Input streaming ideal untuk UI percakapan di mana pengguna berinteraksi dengan agen sepanjang eksekusi, bukan hanya di titik persetujuan.

<h3 id="custom-tools">
  Alat kustom
</h3>

Gunakan [alat kustom](/docs/id/agent-sdk/custom-tools) ketika Anda perlu:

* **Mengumpulkan input terstruktur**: bangun formulir, wizard, atau alur kerja multi-langkah yang melampaui format pilihan ganda `AskUserQuestion`
* **Mengintegrasikan sistem persetujuan eksternal**: terhubung ke platform tiket, alur kerja, atau persetujuan yang ada
* **Menerapkan interaksi khusus domain**: buat alat yang disesuaikan dengan kebutuhan aplikasi Anda, seperti antarmuka tinjauan kode atau daftar periksa penyebaran

Alat kustom memberi Anda kontrol penuh atas interaksi, tetapi memerlukan lebih banyak pekerjaan implementasi daripada menggunakan callback `canUseTool` bawaan.

<h2 id="related-resources">
  Sumber daya terkait
</h2>

* [Konfigurasi izin](/docs/id/agent-sdk/permissions): atur mode dan aturan izin
* [Kontrol eksekusi dengan hooks](/docs/id/agent-sdk/hooks): jalankan kode kustom di titik-titik kunci dalam siklus hidup agen
* [Referensi TypeScript SDK](/docs/id/agent-sdk/typescript#canusetool): dokumentasi API canUseTool lengkap
