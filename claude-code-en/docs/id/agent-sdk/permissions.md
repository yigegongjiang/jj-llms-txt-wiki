> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Konfigurasi izin

> Kontrol bagaimana agen Anda menggunakan alat dengan mode izin, hooks, dan aturan allow/deny deklaratif.

Claude Agent SDK menyediakan kontrol izin untuk mengelola bagaimana Claude menggunakan alat. Gunakan mode izin dan aturan untuk menentukan apa yang diizinkan secara otomatis, dan callback [`canUseTool`](/docs/id/agent-sdk/user-input) untuk menangani segalanya di runtime.

<h2 id="how-permissions-are-evaluated">
  Bagaimana izin dievaluasi
</h2>

Ketika Claude meminta alat, SDK memeriksa izin dalam urutan ini:

<Steps>
  <Step title="Hooks">
    Jalankan [hooks](/docs/id/agent-sdk/hooks) terlebih dahulu. Hook dapat menolak panggilan sepenuhnya atau meneruskannya. Hook yang mengembalikan `allow` tidak melewati aturan deny dan ask di bawah; aturan tersebut dievaluasi terlepas dari hasil hook. Hook `PreToolUse` allow juga tidak dapat menyetujui penghapusan `rm` atau `rmdir` yang menargetkan [jalur kritis](/docs/id/permission-modes#critical-paths).
  </Step>

  <Step title="Deny rules">
    Periksa aturan `deny` (dari `disallowed_tools` dan [settings.json](/docs/id/settings-reference#permission-settings)). Jika aturan deny cocok, alat diblokir, bahkan dalam mode `bypassPermissions`. Aturan deny dengan nama bare seperti `Bash` menghapus alat dari konteks Claude sebelum evaluasi ini dimulai, jadi hanya aturan yang dibatasi seperti `Bash(rm *)` yang diperiksa pada langkah ini.
  </Step>

  <Step title="Ask rules">
    Periksa aturan `ask` dari [settings.json](/docs/id/settings-reference#permission-settings). Jika aturan ask cocok, panggilan jatuh melalui callback [`canUseTool`](/docs/id/agent-sdk/user-input) Anda untuk konfirmasi, bahkan dalam mode `bypassPermissions`.

    Alat yang memerlukan interaksi pengguna berperilaku dengan cara yang sama: `AskUserQuestion` dan alat MCP yang servernya menetapkan [`_meta["anthropic/requiresUserInteraction"]`](/docs/id/mcp#require-approval-for-a-specific-tool) selalu jatuh melalui callback, bahkan ketika aturan allow cocok. Dalam mode `dontAsk` kedua kasus ditolak sebagai gantinya, karena mode itu tidak pernah meminta. Anotasi MCP memerlukan Claude Code v2.1.199 atau lebih baru.

    Alat konektor [claude.ai](/docs/id/mcp#organization-controls-on-connector-tools) yang organisasi Anda atur ke `ask` juga meninggalkan alur pada langkah ini. Setiap panggilan jatuh melalui callback, bahkan dalam mode `bypassPermissions` dan bahkan ketika aturan allow cocok. Callback menerima alasan `Your organization requires approval for this tool`. Dalam mode `dontAsk` panggilan ditolak sebagai gantinya, karena mode itu tidak pernah meminta.
  </Step>

  <Step title="Permission mode">
    Terapkan [mode izin](#permission-modes) yang aktif:

    * Dalam mode `bypassPermissions`, Claude Code menyetujui semua yang mencapai langkah ini kecuali penghapusan `rm` dan `rmdir` yang menargetkan [jalur kritis](/docs/id/permission-modes#critical-paths), yang jatuh melalui sebagai gantinya.
    * Dalam mode `acceptEdits`, Claude Code menyetujui operasi file yang tercantum di bawah [Accept edits mode](#accept-edits-mode-acceptedits).
    * Dalam mode `plan`, Claude Code mengirim alat file-edit dan shell-write ke callback `canUseTool` Anda terlepas dari aturan allow, sehingga operasi write tidak dapat disetujui secara otomatis saat merencanakan.
    * Dalam mode lain, permintaan jatuh melalui.
  </Step>

  <Step title="Allow rules">
    Periksa aturan `allow` (dari `allowed_tools` dan settings.json). Jika aturan cocok, alat disetujui. Panggilan yang alat setujui sendiri juga diselesaikan pada langkah ini, tanpa aturan yang diperlukan: misalnya pembacaan file di dalam direktori kerja Anda atau [perintah Bash read-only](/docs/id/permissions#read-only-commands). Penghapusan `rm` dan `rmdir` yang menargetkan [jalur kritis](/docs/id/permission-modes#critical-paths) tidak pernah disetujui oleh aturan allow: mereka mencapai callback Anda dalam mode yang meminta, pergi ke [classifier](/docs/id/permission-modes#eliminate-prompts-with-auto-mode) dalam mode `auto` pada Claude Code v2.1.218 atau lebih baru, dan ditolak dalam mode `dontAsk`.
  </Step>

  <Step title="canUseTool callback">
    Jika tidak diselesaikan oleh salah satu di atas, panggil callback [`canUseTool`](/docs/id/agent-sdk/user-input) Anda untuk keputusan. Dalam mode `dontAsk`, langkah ini dilewati dan alat ditolak.

    Dalam SDK TypeScript, jika Anda menetapkan [`permissionPrompts: 'none'`](/docs/id/agent-sdk/typescript#options), callback Anda tidak dipanggil pada langkah ini. Hook [`PermissionRequest`](/docs/id/hooks#permissionrequest) masih mendapat kesempatan untuk memutuskan, dan jika tidak, Claude Code menolak panggilan. Opsi memerlukan Claude Code v2.1.259 atau lebih baru.
  </Step>
</Steps>

<img src="https://mintcdn.com/claude-code/jYgs7qigNjO1Badj/images/agent-sdk/permissions-flow.svg?fit=max&auto=format&n=jYgs7qigNjO1Badj&q=85&s=c771ad9085b1277d3708027a49c744bc" className="dark:hidden" alt="Diagram dari alur evaluasi izin enam langkah yang cocok dengan langkah-langkah di atas: permintaan alat melewati hooks, deny rules, ask rules, permission mode, allow rules, dan canUseTool. Hooks, deny rules, dan canUseTool dapat merutekan ke Blocked; permission mode bypass, allow rules, dan canUseTool dapat merutekan ke Execute; ask rules merutekan ke canUseTool." width="1180" height="260" data-path="images/agent-sdk/permissions-flow.svg" />

<img src="https://mintcdn.com/claude-code/_xqph1dUOslCOwsj/images/agent-sdk/permissions-flow-dark.svg?fit=max&auto=format&n=_xqph1dUOslCOwsj&q=85&s=e53a91e9059cbf51852b7cedb4dd4251" className="hidden dark:block" alt="Diagram dari alur evaluasi izin enam langkah yang cocok dengan langkah-langkah di atas: permintaan alat melewati hooks, deny rules, ask rules, permission mode, allow rules, dan canUseTool. Hooks, deny rules, dan canUseTool dapat merutekan ke Blocked; permission mode bypass, allow rules, dan canUseTool dapat merutekan ke Execute; ask rules merutekan ke canUseTool." width="1180" height="260" data-path="images/agent-sdk/permissions-flow-dark.svg" />

Jika Anda meneruskan callback `canUseTool` dalam konfigurasi di mana SDK TypeScript mengharapkan urutan evaluasi untuk menyetujui panggilan secara otomatis sebelum callback dikonsultasikan, SDK memancarkan peringatan proses Node.js sekali ketika kueri dibangun. Kode peringatan adalah `CLAUDE_SDK_CAN_USE_TOOL_SHADOWED`. Dua konfigurasi memicunya:

* `permissionMode: 'bypassPermissions'`, yang menyetujui setiap panggilan yang mencapai langkah mode izin terlepas dari [tindakan yang tidak ada mode auto-approve](/docs/id/permission-modes#actions-no-mode-auto-approves)
* Setiap entri `allowedTools` bare seperti `"Read"`, yang menyetujui seluruh alat itu sebelum callback dikonsultasikan, terlepas dari [tindakan yang tidak ada mode auto-approve](/docs/id/permission-modes#actions-no-mode-auto-approves)

Entri dengan spesifier seperti `Bash(ls *)` dan mode `acceptEdits` tidak memicunya, dan aturan allow yang berasal dari file pengaturan tidak terlihat oleh pemeriksaan.

Dengarkan dengan `process.on('warning', ...)` dan cocokkan kode untuk mencatat atau menekannya. Untuk membatasi setiap panggilan alat terlepas dari mode dan aturan, gunakan hook [`PreToolUse`](/docs/id/agent-sdk/hooks) sebagai gantinya.

Halaman ini berfokus pada **aturan allow dan deny** serta **mode izin**. Untuk langkah-langkah lainnya:

* **Hooks:** jalankan kode khusus untuk mengizinkan, menolak, atau memodifikasi permintaan alat. Lihat [Control execution with hooks](/docs/id/agent-sdk/hooks).
* **canUseTool callback:** minta persetujuan pengguna saat runtime, ketika tidak ada langkah sebelumnya yang menyelesaikan panggilan. Lihat [Handle approvals and user input](/docs/id/agent-sdk/user-input).

<h2 id="allow-and-deny-rules">
  Aturan izin dan penolakan
</h2>

`allowed_tools` dan `disallowed_tools` (TypeScript: `allowedTools` / `disallowedTools`) menambahkan entri ke daftar aturan izin dan penolakan dalam alur evaluasi di atas. Jika Anda menyebutkan salah satu dari [alat pelacakan tugas](/docs/id/agent-sdk/todo-tracking#model-availability) dalam `allowed_tools`, Claude Code juga memilih sesi masuk. Alat lain apa pun yang tidak tercantum dalam `allowed_tools` masih tersedia untuk Claude, dan panggilan ke alat tersebut yang memerlukan persetujuan jatuh melalui mode izin. Aturan penolakan berperilaku berbeda tergantung pada apakah mereka menyebutkan alat atau membatasi pola dalam satu.

| Opsi                              | Efek                                                                                                                                                                                                                                                    |
| :-------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `allowed_tools=["Read", "Grep"]`  | `Read` dan `Grep` disetujui secara otomatis. Alat lain yang tidak tercantum di sini masih ada, dan panggilan ke alat tersebut yang memerlukan persetujuan jatuh melalui mode izin dan `canUseTool`.                                                     |
| `disallowed_tools=["Bash"]`       | Definisi alat `Bash` dihapus dari permintaan. Claude tidak melihat alat dan tidak dapat mencobanya.                                                                                                                                                     |
| `disallowed_tools=["Bash(rm *)"]` | `Bash` tetap tersedia. Panggilan yang cocok dengan `rm *` [seperti yang ditulis](/docs/id/permissions#bash-rule-limits) ditolak dalam setiap mode izin, termasuk `bypassPermissions`. Panggilan `Bash` lainnya, termasuk `/bin/rm`, jatuh melalui mode izin. |
| `disallowed_tools=["*"]`          | Setiap definisi alat dihapus dari permintaan. Glob nama alat didukung dalam aturan penolakan: `"*"` cocok dengan setiap alat dan `"mcp__*"` cocok dengan setiap alat MCP di semua server.                                                               |

Aturan izin menerima glob nama alat hanya setelah awalan `mcp__<server>__` literal. Segmen server harus bebas glob sehingga aturan menyebutkan server spesifik yang Anda konfigurasi: `mcp__puppeteer__*` cocok dengan setiap alat dari server `puppeteer`, dan `mcp__github__get_*` cocok dengan alat `get_` miliknya. Entri yang tidak berlabuh seperti `allowed_tools=["*"]` atau `allowed_tools=["mcp__*"]` diabaikan dengan peringatan startup dan tidak menyetujui apa pun secara otomatis.

Aturan berskop untuk `Read` dan `Edit` mengambil pola jalur. Aturan `Edit(path)` mengatur semua alat bawaan yang menulis file, termasuk `Write` dan `NotebookEdit`; aturan `Write(path)` tidak pernah cocok dengan pemeriksaan izin file.

Gunakan `//path` untuk jalur sistem file absolut: aturan penolakan `Edit(//secrets/**)` memblokir penulisan di mana pun di bawah `/secrets` di disk. Dengan garis miring tunggal di depan, `Edit(/secrets/**)` berlabuh di sumber aturan sebagai gantinya. Untuk aturan yang dilewatkan melalui `allowed_tools` atau `disallowed_tools`, itu berarti direktori kerja sesi, sehingga aturan tidak memblokir `/secrets` di disk. Lihat [Aturan Read dan Edit](/docs/id/permissions#read-and-edit) untuk empat bentuk jangkar dan bagaimana aturan dari file pengaturan diselesaikan.

<Warning>
  **Alat yang disetujui secara otomatis tidak pernah mencapai `canUseTool`.** Panggilan alat yang disetujui pada langkah sebelumnya apa pun, oleh `acceptEdits` atau `bypassPermissions`, atau oleh aturan izin, melewati callback `canUseTool` Anda, sehingga pemeriksaan izin yang Anda letakkan di sana diam-diam dilewati untuk alat tersebut. `AskUserQuestion`, alat MCP yang ditandai [`_meta["anthropic/requiresUserInteraction"]`](/docs/id/mcp#require-approval-for-a-specific-tool), alat konektor [organisasi Anda atur ke `ask`](/docs/id/mcp#organization-controls-on-connector-tools), dan penghapusan `rm` dan `rmdir` yang menargetkan [jalur kritis](/docs/id/permission-modes#critical-paths) masih mencapai callback, bahkan ketika aturan izin cocok. Dalam mode `auto`, penghapusan jalur kritis pergi ke [pengklasifikasi](/docs/id/permission-modes#eliminate-prompts-with-auto-mode) alih-alih callback, sementara panggilan lain yang tercantum di sini masih mencapainya; perutean pengklasifikasi memerlukan Claude Code v2.1.218 atau lebih baru. Dalam mode `dontAsk` panggilan ini ditolak sebagai gantinya, tanpa memanggil callback.

  Cakupan tergantung pada bentuk entri: nama telanjang seperti `Read` atau `mcp__github__get_issue` menyetujui setiap panggilan ke alat tersebut terlepas dari pengecualian di atas, sementara aturan berskop seperti `Bash(npm test *)` hanya menyetujui panggilan yang cocok, dan panggilan `Bash` lainnya yang memerlukan persetujuan masih jatuh melalui callback. Untuk pemeriksaan yang harus berjalan pada setiap panggilan alat, gunakan [hook `PreToolUse`](/docs/id/agent-sdk/hooks): hook berjalan sebelum setiap langkah lainnya, dan penolakan hook bahkan berlaku dalam mode `bypassPermissions`.
</Warning>

Untuk agen yang terkunci, pasangkan `allowedTools` dengan `permissionMode: "dontAsk"`:

```typescript theme={null}
const options = {
  allowedTools: ["Read", "Glob", "Grep"],
  permissionMode: "dontAsk"
};
```

Alat yang tercantum disetujui, terlepas dari [tindakan yang tidak ada mode auto-approve](/docs/id/permission-modes#actions-no-mode-auto-approves), dan setiap panggilan lain yang akan meminta ditolak sebagai gantinya. Panggilan yang tidak memerlukan persetujuan dalam mode `default` berjalan apakah atau tidak Anda mencantumnya, seperti [perintah Bash hanya-baca](/docs/id/permissions#read-only-commands), alat seperti `Agent` yang tidak bertanya sebelum menjalankan, dan pembacaan file di dalam direktori kerja Anda. Untuk menempatkan alat di luar jangkauan Claude sepenuhnya, tambahkan nama telanjangnya ke `disallowedTools`.

<Warning>
  **`allowed_tools` tidak membatasi `bypassPermissions`.** `allowed_tools` pra-menyetujui alat yang Anda cantumkan. Alat unlisted lainnya tidak cocok dengan aturan izin apa pun dan jatuh melalui mode izin, di mana `bypassPermissions` menyetujuinya. Mengatur `allowed_tools=["Read"]` bersama dengan `permission_mode="bypassPermissions"` masih menyetujui setiap alat, termasuk `Bash`, `Write`, dan `Edit`. Jika Anda memerlukan `bypassPermissions` tetapi menginginkan alat spesifik diblokir, gunakan `disallowed_tools`.
</Warning>

Anda juga dapat mengonfigurasi aturan izin, penolakan, dan tanya secara deklaratif dalam `.claude/settings.json`. Aturan ini dibaca ketika sumber pengaturan `project` diaktifkan, yang mana untuk opsi `query()` default. Jika Anda mengatur `setting_sources` (TypeScript: `settingSources`) secara eksplisit, sertakan `"project"` agar aturan diterapkan. Lihat [Pengaturan Izin](/docs/id/settings-reference#permission-settings) untuk sintaks aturan.

<h2 id="permission-modes">
  Mode izin
</h2>

Mode izin memberikan kontrol global atas cara Claude menggunakan tools. Anda dapat mengatur mode izin saat memanggil `query()` atau mengubahnya secara dinamis selama sesi streaming.

<h3 id="available-modes">
  Mode yang tersedia
</h3>

SDK mendukung mode izin berikut:

| Mode                | Deskripsi                               | Perilaku Tool                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| :------------------ | :-------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `default`           | Perilaku izin standar                   | Tidak ada persetujuan otomatis berbasis mode; panggilan yang memerlukan persetujuan dan tidak cocok dengan aturan izin memicu callback `canUseTool` Anda                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `dontAsk`           | Tolak alih-alih meminta                 | Setiap panggilan yang sebaliknya akan meminta ditolak. Panggilan yang disetujui oleh `allowed_tools` atau aturan berjalan, begitu juga panggilan yang tidak memerlukan persetujuan dalam mode `default`, seperti pembacaan file di dalam direktori kerja Anda dan panggilan ke `Agent`. Connector tools [organisasi Anda atur ke `ask`](/docs/id/mcp#organization-controls-on-connector-tools) dan tools yang memerlukan interaksi pengguna ditolak bahkan jika Anda telah menyetujuinya sebelumnya, begitu juga penghapusan `rm` dan `rmdir` yang menargetkan [jalur kritis](/docs/id/permission-modes#critical-paths). `canUseTool` tidak pernah dipanggil |
| `acceptEdits`       | Terima otomatis pengeditan file         | Pengeditan file dan [operasi sistem file](#accept-edits-mode-acceptedits) (`mkdir`, `rm`, `mv`, dll.) secara otomatis disetujui                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `bypassPermissions` | Lewati pemeriksaan izin                 | Tools berjalan tanpa prompt izin, kecuali untuk [tindakan yang tidak ada mode auto-approve](/docs/id/permission-modes#actions-no-mode-auto-approves). Gunakan dengan hati-hati                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `plan`              | Mode perencanaan                        | Claude menjelajahi dan merencanakan tanpa mengedit file sumber Anda; pengeditan file tidak pernah auto-approved dan meminta melalui callback `canUseTool` Anda                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `auto`              | Persetujuan yang diklasifikasikan model | Pengklasifikasi model menyetujui atau menolak prompt izin. Lihat [Mode Auto](/docs/id/permission-modes#eliminate-prompts-with-auto-mode) untuk ketersediaan                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |

<Warning>
  **Pewarisan subagent:** Subagent berjalan dalam mode izin sesi induk kecuali Anda mengatur `permissionMode` pada [`AgentDefinition`](/docs/id/agent-sdk/typescript#agentdefinition) dan sesi induk berada dalam mode `default`, `dontAsk`, atau `plan`. Bahkan kemudian, Claude Code tidak pernah menerapkan nilai `"bypassPermissions"`. Subagent berjalan dalam mode `bypassPermissions` hanya ketika sesi induk itu sendiri melakukannya. Pengecualian `bypassPermissions` memerlukan Claude Code v2.1.267 atau lebih baru.

  Subagent mungkin memiliki prompt sistem yang berbeda dan perilaku yang kurang terbatas daripada agen utama Anda, jadi mewarisi `bypassPermissions` memberi mereka akses sistem penuh dan otonom. [Tindakan yang tidak ada mode auto-approve](/docs/id/permission-modes#actions-no-mode-auto-approves) masih berlaku.
</Warning>

<h3 id="set-permission-mode">
  Atur mode izin
</h3>

Anda dapat mengatur mode izin sekali saat memulai query, atau mengubahnya secara dinamis saat sesi aktif.

<Tabs>
  <Tab title="Pada waktu query">
    Lewatkan `permission_mode` (Python) atau `permissionMode` (TypeScript) saat membuat query. Mode ini berlaku untuk seluruh sesi kecuali diubah secara dinamis.

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
    Panggil `set_permission_mode()` (Python) atau `setPermissionMode()` (TypeScript) untuk mengubah mode di tengah sesi. Mode baru berlaku segera untuk semua permintaan tool berikutnya. Ini memungkinkan Anda untuk memulai dengan pembatasan dan melonggarkan izin seiring kepercayaan berkembang, misalnya beralih ke `acceptEdits` setelah meninjau pendekatan awal Claude.

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
  Mode terima pengeditan (`acceptEdits`)
</h4>

Auto-approve operasi file sehingga Claude dapat mengedit kode tanpa meminta. Tools lain (seperti perintah Bash yang bukan operasi sistem file) masih memerlukan izin normal.

**Operasi yang auto-approved:**

* Pengeditan file (tools Edit, Write)
* Perintah sistem file: `mkdir`, `touch`, `rm`, `rmdir`, `mv`, `cp`, `sed`

Keduanya hanya berlaku untuk jalur di dalam direktori kerja atau `additionalDirectories`. Dalam mode `acceptEdits`, Claude Code tidak auto-approve permintaan ketika Claude:

* Bekerja pada jalur di luar cakupan itu
* Menulis ke jalur yang dilindungi
* Menghapus [jalur kritis](/docs/id/permission-modes#critical-paths) dengan `rm` atau `rmdir`

**Gunakan ketika:** Anda mempercayai pengeditan Claude dan menginginkan iterasi yang lebih cepat, seperti selama prototyping atau saat bekerja di direktori terisolasi.

<h4 id="don’t-ask-mode-dontask">
  Mode jangan tanya (`dontAsk`)
</h4>

Mengonversi prompt izin apa pun menjadi penolakan, tanpa memanggil `canUseTool`. Tools yang disetujui sebelumnya oleh `allowed_tools`, aturan izin `settings.json`, atau hook berjalan seperti biasa, begitu juga panggilan yang tidak memerlukan persetujuan dalam mode `default`, seperti pembacaan file di dalam direktori kerja Anda dan panggilan ke `Agent`. Connector tools [organisasi Anda atur ke `ask`](/docs/id/mcp#organization-controls-on-connector-tools), tools yang memerlukan interaksi pengguna, dan penghapusan `rm` dan `rmdir` yang menargetkan [jalur kritis](/docs/id/permission-modes#critical-paths) ditolak bahkan ketika aturan izin cocok. Hook allow `PreToolUse` juga tidak menghapus penghapusan jalur kritis.

**Gunakan ketika:** Anda menginginkan permukaan tool yang tetap dan eksplisit untuk agen headless dan lebih suka penolakan keras daripada ketergantungan diam pada `canUseTool` yang tidak ada.

<h4 id="bypass-permissions-mode-bypasspermissions">
  Mode lewati izin (`bypassPermissions`)
</h4>

Auto-approve penggunaan tool tanpa meminta, kecuali kasus yang tercantum dalam peringatan di bawah. Hook masih dijalankan dan dapat memblokir operasi jika diperlukan. Pada Linux dan macOS, Claude Code menolak untuk memulai dalam mode ini sebagai root atau di bawah `sudo` di luar [sandbox yang diakui](/docs/id/permission-modes#skip-all-checks-with-bypasspermissions-mode), dan query gagal sebelum giliran pertama.

<Warning>
  Gunakan dengan sangat hati-hati. Claude memiliki akses sistem penuh dalam mode ini. Hanya gunakan di lingkungan terkontrol di mana Anda mempercayai semua operasi yang mungkin.

  `allowed_tools` tidak membatasi mode ini. Setiap tool disetujui, bukan hanya yang Anda daftarkan. Kontrol ini masih berlaku:

  * Aturan penolakan, aturan `ask` eksplisit, dan hook dievaluasi sebelum pemeriksaan mode dan masih dapat memblokir tool.
  * Connector tools [organisasi Anda atur ke `ask`](/docs/id/mcp#organization-controls-on-connector-tools), tools yang memerlukan interaksi pengguna, dan penghapusan `rm` dan `rmdir` yang menargetkan [jalur kritis](/docs/id/permission-modes#critical-paths) masih jatuh ke callback `canUseTool` Anda.
  * [Perlindungan pesan lintas sesi](/docs/id/permission-modes#skip-all-checks-with-bypasspermissions-mode) masih berlaku.
</Warning>

<h4 id="plan-mode-plan">
  Mode rencana (`plan`)
</h4>

Claude menjelajahi basis kode dan menghasilkan rencana tanpa mengedit file sumber Anda. Tools read-only berjalan seperti dalam mode izin `default`.

Pengeditan file tidak pernah auto-approved dalam mode rencana, bahkan ketika aturan izin cocok. Mereka meminta melalui callback `canUseTool` Anda sebagai gantinya. Pada Claude Code v2.1.212 atau lebih baru, perintah shell yang memodifikasi file, seperti `touch` dan `rm`, mencapai callback `canUseTool` Anda dengan cara yang sama.

Jika Anda mengatur `allowDangerouslySkipPermissions: true` bersama dengan `permissionMode: 'plan'`, pengeditan file dan perintah shell yang memodifikasi file masih mencapai callback `canUseTool` Anda. Opsi ini memungkinkan Anda untuk beralih ke `bypassPermissions` nanti dengan `setPermissionMode()`.

Claude dapat menggunakan `AskUserQuestion` untuk mengklarifikasi persyaratan sebelum menyelesaikan rencana. Lihat [Tangani persetujuan dan input pengguna](/docs/id/agent-sdk/user-input#handle-clarifying-questions) untuk menangani prompt ini.

**Gunakan ketika:** Anda ingin Claude mengusulkan perubahan tanpa menjalankannya, seperti selama tinjauan kode atau ketika Anda perlu menyetujui perubahan sebelum dibuat.

<h2 id="related-resources">
  Sumber daya terkait
</h2>

Untuk langkah-langkah lain dalam alur evaluasi izin:

* [Menangani persetujuan dan input pengguna](/docs/id/agent-sdk/user-input): prompt persetujuan interaktif dan pertanyaan klarifikasi
* [Panduan hooks](/docs/id/agent-sdk/hooks): jalankan kode khusus pada titik-titik kunci dalam siklus hidup agen
* [Aturan izin](/docs/id/settings-reference#permission-settings): aturan allow/deny deklaratif dalam `settings.json`
