> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Konfigurasi izin

> Kontrol bagaimana agen Anda menggunakan alat dengan mode izin, hooks, dan aturan allow/deny deklaratif.

Claude Agent SDK menyediakan kontrol izin untuk mengelola bagaimana Claude menggunakan alat. Gunakan mode izin dan aturan untuk menentukan apa yang diizinkan secara otomatis, dan callback [`canUseTool`](/docs/id/agent-sdk/user-input) untuk menangani segalanya di runtime.

<Note>
  Halaman ini mencakup mode izin dan aturan. Untuk membangun alur persetujuan interaktif di mana pengguna menyetujui atau menolak permintaan alat di runtime, lihat [Tangani persetujuan dan input pengguna](/docs/id/agent-sdk/user-input).
</Note>

<h2 id="how-permissions-are-evaluated">
  Bagaimana izin dievaluasi
</h2>

Ketika Claude meminta alat, SDK memeriksa izin dalam urutan ini:

<Steps>
  <Step title="Hooks">
    Jalankan [hooks](/docs/id/agent-sdk/hooks) terlebih dahulu. Hook dapat menolak panggilan sepenuhnya atau meneruskannya. Hook yang mengembalikan `allow` tidak melewati aturan deny dan ask di bawah; aturan tersebut dievaluasi terlepas dari hasil hook.
  </Step>

  <Step title="Deny rules">
    Periksa aturan `deny` (dari `disallowed_tools` dan [settings.json](/docs/id/settings#permission-settings)). Jika aturan deny cocok, alat diblokir, bahkan dalam mode `bypassPermissions`. Aturan deny dengan nama bare seperti `Bash` menghapus alat dari konteks Claude sebelum evaluasi ini dimulai, jadi hanya aturan berscopе seperti `Bash(rm *)` yang diperiksa pada langkah ini.
  </Step>

  <Step title="Ask rules">
    Periksa aturan `ask` dari [settings.json](/docs/id/settings#permission-settings). Jika aturan ask cocok, panggilan jatuh melalui callback [`canUseTool`](/docs/id/agent-sdk/user-input) Anda untuk konfirmasi, bahkan dalam mode `bypassPermissions`.

    Alat yang memerlukan interaksi pengguna berperilaku dengan cara yang sama: `AskUserQuestion` dan alat MCP yang servernya menetapkan [`_meta["anthropic/requiresUserInteraction"]`](/docs/id/mcp#require-approval-for-a-specific-tool) selalu jatuh melalui callback, bahkan ketika aturan allow cocok. Dalam mode `dontAsk` kedua kasus ditolak sebagai gantinya, karena mode itu tidak pernah meminta. Anotasi MCP memerlukan Claude Code v2.1.199 atau lebih baru.

    Alat konektor [claude.ai](/docs/id/mcp#organization-controls-on-connector-tools) yang organisasi Anda telah atur ke `ask` juga meninggalkan alur pada langkah ini. Setiap panggilan jatuh melalui callback, bahkan dalam mode `bypassPermissions` dan bahkan ketika aturan allow cocok. Callback menerima alasan `Organisasi Anda memerlukan persetujuan untuk alat ini`. Dalam mode `dontAsk` panggilan ditolak sebagai gantinya, karena mode itu tidak pernah meminta.
  </Step>

  <Step title="Permission mode">
    Terapkan [mode izin](#permission-modes) yang aktif. `bypassPermissions` menyetujui semua yang mencapai langkah ini. `acceptEdits` menyetujui operasi file. `plan` merutekan alat file-edit dan shell-write ke callback `canUseTool` Anda terlepas dari aturan allow, jadi operasi write tidak dapat disetujui secara otomatis saat merencanakan. Mode lain jatuh melalui.
  </Step>

  <Step title="Allow rules">
    Periksa aturan `allow` (dari `allowed_tools` dan settings.json). Jika aturan cocok, alat disetujui.
  </Step>

  <Step title="canUseTool callback">
    Jika tidak diselesaikan oleh salah satu di atas, panggil callback [`canUseTool`](/docs/id/agent-sdk/user-input) Anda untuk keputusan. Dalam mode `dontAsk`, langkah ini dilewati dan alat ditolak.
  </Step>
</Steps>

<img src="https://mintcdn.com/claude-code/jYgs7qigNjO1Badj/images/agent-sdk/permissions-flow.svg?fit=max&auto=format&n=jYgs7qigNjO1Badj&q=85&s=c771ad9085b1277d3708027a49c744bc" alt="Diagram alur evaluasi izin enam langkah yang sesuai dengan langkah-langkah di atas: permintaan alat melewati hooks, aturan deny, aturan ask, mode izin, aturan allow, dan canUseTool. Hooks, aturan deny, dan canUseTool dapat merutekan ke Blocked; bypass mode izin, aturan allow, dan canUseTool dapat merutekan ke Execute; aturan ask merutekan ke canUseTool." width="1180" height="260" data-path="images/agent-sdk/permissions-flow.svg" />

Mulai dari v2.1.198, jika Anda meneruskan callback `canUseTool` yang urutan evaluasi ini tidak pernah dapat mencapai, SDK TypeScript mengeluarkan peringatan proses Node.js sekali ketika kueri dibangun. Kode peringatan adalah `CLAUDE_SDK_CAN_USE_TOOL_SHADOWED`. Dua konfigurasi memicunya:

* `permissionMode: 'bypassPermissions'`, yang secara otomatis menyetujui setiap panggilan yang mencapai langkah mode izin
* Setiap entri `allowedTools` bare seperti `"Read"`, yang secara otomatis menyetujui seluruh alat itu sebelum callback dikonsultasikan

Entri dengan specifier seperti `Bash(ls *)` dan mode `acceptEdits` tidak memicunya, dan aturan allow yang berasal dari file pengaturan tidak terlihat oleh pemeriksaan.

Dengarkan dengan `process.on('warning', ...)` dan cocokkan kode untuk mencatat atau menekannya. Untuk membatasi setiap panggilan alat terlepas dari mode dan aturan, gunakan hook [`PreToolUse`](/docs/id/agent-sdk/hooks) sebagai gantinya.

Halaman ini berfokus pada **aturan allow dan deny** serta **mode izin**. Untuk langkah lainnya:

* **Hooks:** jalankan kode khusus untuk mengizinkan, menolak, atau memodifikasi permintaan alat. Lihat [Kontrol eksekusi dengan hooks](/docs/id/agent-sdk/hooks).
* **canUseTool callback:** minta persetujuan pengguna di runtime, ketika tidak ada langkah sebelumnya yang menyelesaikan panggilan. Lihat [Tangani persetujuan dan input pengguna](/docs/id/agent-sdk/user-input).

<h2 id="allow-and-deny-rules">
  Aturan allow dan deny
</h2>

`allowed_tools` dan `disallowed_tools` (TypeScript: `allowedTools` / `disallowedTools`) menambahkan entri ke daftar aturan allow dan deny dalam alur evaluasi di atas. Aturan allow hanya mempengaruhi persetujuan: alat yang tidak tercantum dalam `allowed_tools` masih tersedia untuk Claude dan jatuh melalui mode izin. Aturan deny berperilaku berbeda tergantung pada apakah mereka menamai alat atau membatasi pola dalam satu alat.

| Opsi                              | Efek                                                                                                                                                                                 |
| :-------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `allowed_tools=["Read", "Grep"]`  | `Read` dan `Grep` disetujui secara otomatis. Alat yang tidak tercantum di sini masih ada dan jatuh melalui mode izin dan `canUseTool`.                                               |
| `disallowed_tools=["Bash"]`       | Definisi alat `Bash` dihapus dari permintaan. Claude tidak melihat alat dan tidak dapat mencobanya.                                                                                  |
| `disallowed_tools=["Bash(rm *)"]` | `Bash` tetap tersedia. Panggilan yang cocok dengan `rm *` ditolak di setiap mode izin, termasuk `bypassPermissions`. Panggilan `Bash` lainnya jatuh melalui mode izin.               |
| `disallowed_tools=["*"]`          | Setiap definisi alat dihapus dari permintaan. Glob nama-alat didukung dalam aturan deny: `"*"` cocok dengan setiap alat dan `"mcp__*"` cocok dengan setiap alat MCP di semua server. |

Aturan allow menerima glob nama-alat hanya setelah awalan literal `mcp__<server>__`. Segmen server harus bebas glob sehingga aturan menamai server spesifik yang Anda konfigurasi: `mcp__puppeteer__*` cocok dengan setiap alat dari server `puppeteer`, dan `mcp__github__get_*` cocok dengan alat `get_` miliknya. Entri yang tidak berlabuh seperti `allowed_tools=["*"]` atau `allowed_tools=["mcp__*"]` diabaikan dengan peringatan startup dan tidak menyetujui apa pun secara otomatis.

Aturan yang dibatasi untuk `Read` dan `Edit` mengambil pola jalur. Aturan `Edit(path)` mengatur semua alat bawaan yang menulis file, termasuk `Write` dan `NotebookEdit`; aturan `Write(path)` tidak pernah cocok dengan pemeriksaan izin file.

Gunakan `//path` untuk jalur sistem file absolut: aturan deny dari `Edit(//secrets/**)` memblokir penulisan di mana pun di bawah `/secrets` di disk. Dengan garis miring tunggal di depan, `Edit(/secrets/**)` berlabuh di sumber aturan sebagai gantinya. Untuk aturan yang dilewatkan melalui `allowed_tools` atau `disallowed_tools`, itu berarti direktori kerja sesi, sehingga aturan tidak memblokir `/secrets` di disk. Lihat [Aturan Read dan Edit](/docs/id/permissions#read-and-edit) untuk empat bentuk jangkar dan bagaimana aturan dari file pengaturan diselesaikan.

<Warning>
  **Alat yang disetujui otomatis tidak pernah mencapai `canUseTool`.** Panggilan alat yang disetujui pada langkah sebelumnya apa pun, oleh `acceptEdits` atau `bypassPermissions`, atau oleh aturan allow, melewati callback `canUseTool` Anda, sehingga pemeriksaan izin yang Anda letakkan di sana secara diam-diam dilewati untuk alat tersebut. `AskUserQuestion`, alat MCP yang ditandai [`_meta["anthropic/requiresUserInteraction"]`](/docs/id/mcp#require-approval-for-a-specific-tool), dan alat konektor [yang organisasi Anda atur ke `ask`](/docs/id/mcp#organization-controls-on-connector-tools) masih mencapai callback, bahkan ketika aturan allow cocok.

  Cakupan tergantung pada bentuk entri: nama bare seperti `Read` atau `mcp__github__get_issue` menyetujui secara otomatis setiap panggilan ke alat tersebut, sementara aturan yang dibatasi seperti `Bash(ls *)` hanya menyetujui panggilan yang cocok dan panggilan `Bash` lainnya masih jatuh melalui callback. Untuk pemeriksaan yang harus berjalan pada setiap panggilan alat, gunakan hook [`PreToolUse`](/docs/id/agent-sdk/hooks): hook berjalan sebelum setiap langkah lainnya, dan penolakan hook berlaku bahkan dalam mode `bypassPermissions`.
</Warning>

Untuk agen yang terkunci, pasangkan `allowedTools` dengan `permissionMode: "dontAsk"`. Alat yang tercantum disetujui, terlepas dari alat yang selalu diminta dalam Peringatan di atas; apa pun yang lain ditolak sepenuhnya daripada meminta:

```typescript theme={null}
const options = {
  allowedTools: ["Read", "Glob", "Grep"],
  permissionMode: "dontAsk"
};
```

<Warning>
  **`allowed_tools` tidak membatasi `bypassPermissions`.** `allowed_tools` hanya pra-menyetujui alat yang Anda cantumkan. Alat yang tidak tercantum tidak cocok dengan aturan allow apa pun dan jatuh melalui mode izin, di mana `bypassPermissions` menyetujuinya. Menetapkan `allowed_tools=["Read"]` bersama dengan `permission_mode="bypassPermissions"` masih menyetujui setiap alat, termasuk `Bash`, `Write`, dan `Edit`. Jika Anda memerlukan `bypassPermissions` tetapi ingin alat tertentu diblokir, gunakan `disallowed_tools`.
</Warning>

Anda juga dapat mengonfigurasi aturan allow, deny, dan ask secara deklaratif di `.claude/settings.json`. Aturan ini dibaca ketika sumber pengaturan `project` diaktifkan, yang merupakan default untuk opsi `query()`. Jika Anda menetapkan `setting_sources` (TypeScript: `settingSources`) secara eksplisit, sertakan `"project"` agar aturan diterapkan. Lihat [Pengaturan izin](/docs/id/settings#permission-settings) untuk sintaks aturan.

<h2 id="permission-modes">
  Mode izin
</h2>

Mode izin memberikan kontrol global atas bagaimana Claude menggunakan alat. Anda dapat menetapkan mode izin saat memanggil `query()` atau mengubahnya secara dinamis selama sesi streaming.

<h3 id="available-modes">
  Mode yang tersedia
</h3>

SDK mendukung mode izin ini:

| Mode                | Deskripsi                               | Perilaku alat                                                                                                                                                                                                                                                                                                |
| :------------------ | :-------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `default`           | Perilaku izin standar                   | Tidak ada persetujuan otomatis; alat yang tidak cocok memicu callback `canUseTool` Anda                                                                                                                                                                                                                      |
| `dontAsk`           | Tolak daripada meminta                  | Apa pun yang tidak pra-disetujui oleh `allowed_tools` atau aturan ditolak; alat konektor [organisasi Anda atur ke `ask`](/docs/id/mcp#organization-controls-on-connector-tools) dan alat yang memerlukan interaksi pengguna ditolak bahkan jika Anda telah pra-menyetujuinya. `canUseTool` tidak pernah dipanggil |
| `acceptEdits`       | Terima otomatis edit file               | Edit file dan [operasi sistem file](#accept-edits-mode-acceptedits) (`mkdir`, `rm`, `mv`, dll.) disetujui secara otomatis                                                                                                                                                                                    |
| `bypassPermissions` | Lewati pemeriksaan izin                 | Alat berjalan tanpa prompt izin, kecuali aturan [`ask`](#how-permissions-are-evaluated) eksplisit cocok, alat konektor [organisasi Anda atur ke `ask`](/docs/id/mcp#organization-controls-on-connector-tools), dan alat yang memerlukan interaksi pengguna (gunakan dengan hati-hati)                             |
| `plan`              | Mode perencanaan                        | Claude menjelajahi dan merencanakan tanpa mengedit file sumber Anda; edit file tidak pernah disetujui secara otomatis dan diminta melalui callback `canUseTool` Anda                                                                                                                                         |
| `auto`              | Persetujuan yang diklasifikasikan model | Pengklasifikasi model menyetujui atau menolak setiap panggilan alat. Lihat [Mode Auto](/docs/id/permission-modes#eliminate-prompts-with-auto-mode) untuk ketersediaan                                                                                                                                             |

<Warning>
  **Warisan subagen:** Ketika induk menggunakan `bypassPermissions`, `acceptEdits`, atau `auto`, semua subagen mewarisi mode tersebut dan tidak dapat ditimpa per subagen. Subagen mungkin memiliki prompt sistem yang berbeda dan perilaku yang kurang terbatas daripada agen utama Anda, jadi mewarisi `bypassPermissions` memberikan mereka akses sistem penuh dan otonom. Aturan [`ask`](#how-permissions-are-evaluated) eksplisit, alat konektor [organisasi Anda atur ke `ask`](/docs/id/mcp#organization-controls-on-connector-tools), dan alat yang memerlukan interaksi pengguna masih memaksa prompt.
</Warning>

<h3 id="set-permission-mode">
  Tetapkan mode izin
</h3>

Anda dapat menetapkan mode izin sekali saat memulai kueri, atau mengubahnya secara dinamis saat sesi aktif.

<Tabs>
  <Tab title="Pada waktu kueri">
    Teruskan `permission_mode` (Python) atau `permissionMode` (TypeScript) saat membuat kueri. Mode ini berlaku untuk seluruh sesi kecuali diubah secara dinamis.

    <CodeGroup>
      ```python Python theme={null}
      import asyncio
      from claude_agent_sdk import query, ClaudeAgentOptions


      async def main():
          async for message in query(
              prompt="Help me refactor this code",
              options=ClaudeAgentOptions(
                  permission_mode="default",  # Set the mode here
              ),
          ):
              if hasattr(message, "result"):
                  print(message.result)


      asyncio.run(main())
      ```

      ```typescript TypeScript theme={null}
      import { query } from "@anthropic-ai/claude-agent-sdk";

      async function main() {
        for await (const message of query({
          prompt: "Help me refactor this code",
          options: {
            permissionMode: "default" // Set the mode here
          }
        })) {
          if ("result" in message) {
            console.log(message.result);
          }
        }
      }

      main();
      ```
    </CodeGroup>
  </Tab>

  <Tab title="Selama streaming">
    Panggil `set_permission_mode()` (Python) atau `setPermissionMode()` (TypeScript) untuk mengubah mode di tengah sesi. Mode baru berlaku segera untuk semua permintaan alat berikutnya. Ini memungkinkan Anda memulai dengan pembatasan dan melonggarkan izin seiring kepercayaan berkembang, misalnya beralih ke `acceptEdits` setelah meninjau pendekatan awal Claude.

    <CodeGroup>
      ```python Python theme={null}
      import asyncio
      from claude_agent_sdk import ClaudeSDKClient, ClaudeAgentOptions


      async def main():
          async with ClaudeSDKClient(
              options=ClaudeAgentOptions(
                  permission_mode="default",  # Start in default mode
              )
          ) as client:
              await client.query("Help me refactor this code")

              # Change mode dynamically mid-session
              await client.set_permission_mode("acceptEdits")

              # Process messages with the new permission mode
              async for message in client.receive_response():
                  if hasattr(message, "result"):
                      print(message.result)


      asyncio.run(main())
      ```

      ```typescript TypeScript theme={null}
      import { query } from "@anthropic-ai/claude-agent-sdk";

      async function main() {
        const q = query({
          prompt: "Help me refactor this code",
          options: {
            permissionMode: "default" // Start in default mode
          }
        });

        // Change mode dynamically mid-session
        await q.setPermissionMode("acceptEdits");

        // Process messages with the new permission mode
        for await (const message of q) {
          if ("result" in message) {
            console.log(message.result);
          }
        }
      }

      main();
      ```
    </CodeGroup>
  </Tab>
</Tabs>

<h3 id="mode-details">
  Detail mode
</h3>

<h4 id="accept-edits-mode-acceptedits">
  Mode terima edit (`acceptEdits`)
</h4>

Menyetujui operasi file secara otomatis sehingga Claude dapat mengedit kode tanpa meminta. Alat lain (seperti perintah Bash yang bukan operasi sistem file) masih memerlukan izin normal.

**Operasi yang disetujui secara otomatis:**

* Edit file (alat Edit, Write)
* Perintah sistem file: `mkdir`, `touch`, `rm`, `rmdir`, `mv`, `cp`, `sed`

Keduanya hanya berlaku untuk jalur di dalam direktori kerja atau `additionalDirectories`. Jalur di luar cakupan itu dan penulisan ke jalur yang dilindungi masih meminta.

**Gunakan ketika:** Anda mempercayai edit Claude dan menginginkan iterasi yang lebih cepat, seperti selama prototyping atau saat bekerja di direktori terisolasi.

<h4 id="don’t-ask-mode-dontask">
  Mode jangan tanya (`dontAsk`)
</h4>

Mengonversi prompt izin apa pun menjadi penolakan. Alat yang pra-disetujui oleh `allowed_tools`, aturan allow `settings.json`, atau hook berjalan normal. Alat konektor [organisasi Anda atur ke `ask`](/docs/id/mcp#organization-controls-on-connector-tools) dan alat yang memerlukan interaksi pengguna ditolak bahkan ketika aturan allow cocok. Segalanya ditolak tanpa memanggil `canUseTool`.

**Gunakan ketika:** Anda menginginkan permukaan alat yang tetap dan eksplisit untuk agen headless dan lebih suka penolakan keras daripada ketergantungan diam pada `canUseTool` yang tidak ada.

<h4 id="bypass-permissions-mode-bypasspermissions">
  Mode lewati izin (`bypassPermissions`)
</h4>

Menyetujui semua penggunaan alat secara otomatis tanpa prompt. Hooks masih dijalankan dan dapat memblokir operasi jika diperlukan.

<Warning>
  Gunakan dengan sangat hati-hati. Claude memiliki akses sistem penuh dalam mode ini. Hanya gunakan di lingkungan terkontrol di mana Anda mempercayai semua operasi yang mungkin.

  `allowed_tools` tidak membatasi mode ini. Setiap alat disetujui, bukan hanya yang Anda cantumkan. Aturan deny (`disallowed_tools`), aturan `ask` eksplisit, dan hooks dievaluasi sebelum pemeriksaan mode dan masih dapat memblokir alat. Alat konektor [organisasi Anda atur ke `ask`](/docs/id/mcp#organization-controls-on-connector-tools) dan alat yang memerlukan interaksi pengguna masih jatuh melalui callback `canUseTool` Anda.
</Warning>

<h4 id="plan-mode-plan">
  Mode rencana (`plan`)
</h4>

Claude menjelajahi basis kode dan menghasilkan rencana tanpa mengedit file sumber Anda. Alat baca saja berjalan seperti dalam mode default. Edit file tidak pernah disetujui secara otomatis dalam mode rencana, bahkan ketika aturan allow cocok. Mereka diminta melalui callback `canUseTool` Anda sebagai gantinya. Claude dapat menggunakan `AskUserQuestion` untuk mengklarifikasi persyaratan sebelum menyelesaikan rencana. Lihat [Tangani persetujuan dan input pengguna](/docs/id/agent-sdk/user-input#handle-clarifying-questions) untuk menangani prompt ini.

**Gunakan ketika:** Anda ingin Claude mengusulkan perubahan tanpa menjalankannya, seperti selama tinjauan kode atau ketika Anda perlu menyetujui perubahan sebelum dibuat.

<h2 id="related-resources">
  Sumber daya terkait
</h2>

Untuk langkah lain dalam alur evaluasi izin:

* [Tangani persetujuan dan input pengguna](/docs/id/agent-sdk/user-input): prompt persetujuan interaktif dan pertanyaan klarifikasi
* [Panduan hooks](/docs/id/agent-sdk/hooks): jalankan kode khusus di titik kunci dalam siklus hidup agen
* [Aturan izin](/docs/id/settings#permission-settings): aturan allow/deny deklaratif di `settings.json`
