> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Lacak todos

> Lacak todos dalam sesi Agent SDK dan tampilkan kemajuan Claude dalam aplikasi Anda dari panggilan alat terstruktur

Claude Code menyediakan [alat pelacakan tugas](/docs/id/tools-reference#task-tool-availability) secara default hanya pada model yang tercantum di bawah [Ketersediaan model](#model-availability). Model yang lebih baru melacak pekerjaan multi-langkah tanpa daftar todo tertulis, jadi pada model tersebut Anda tidak memerlukan apa pun di halaman ini agar Claude dapat menyelesaikan tugas multi-langkah.

Dalam sesi yang memiliki alat pelacakan tugas, Claude menyimpan daftar todo tertulis, memperbarui status setiap item saat bekerja. Anda melihat setiap perubahan dalam aliran pesan sebagai panggilan alat terstruktur. Pilih sesi hanya ketika aplikasi Anda membaca panggilan alat tersebut, baik untuk mencatat aktivitas tugas atau untuk merender tampilan kemajuan sendiri.

<h2 id="model-availability">
  Ketersediaan model
</h2>

<Note>
  The following tools are available by default only on Claude 3.x models, Opus 4 through 4.7, Sonnet 4 through 4.6, and Haiku 4.5. On every other model, including model IDs Claude Code doesn't recognize, they aren't available unless you opt in:

  * `TodoWrite`
  * `TaskCreate`
  * `TaskGet`
  * `TaskUpdate`
  * `TaskList`

  Wherever the tools are available, Claude Code provides the four Task tools, or `TodoWrite` instead when you set `CLAUDE_CODE_ENABLE_TASKS=0`.

  This default set applies in Claude Code v2.1.268 and later, which the TypeScript Agent SDK bundles from v0.3.268.
</Note>

Pada model yang tidak memiliki alat secara default, kecuali Anda memilih sesi, Anda tidak melihat blok `tool_use` untuk mereka dalam aliran pesan. Agent SDK menerapkan default ini melalui biner Claude Code yang disertakannya. Jika Anda menunjuk `pathToClaudeCodeExecutable` (TypeScript) atau `cli_path` (Python) ke instalasi Claude Code Anda sendiri, Anda mendapatkan alat apa pun yang disediakan instalasi tersebut, di bawah default-nya sendiri. Untuk melihat set yang tepat dalam sesi yang berjalan, [periksa alat mana yang tersedia](/docs/id/tools-reference#check-which-tools-are-available). Untuk memilih sesi, lakukan salah satu dari berikut:

* Beri nama salah satu alat dalam opsi [`allowedTools`](/docs/id/agent-sdk/permissions#allow-and-deny-rules) (TypeScript) atau `allowed_tools` (Python)
* Daftar alat dalam opsi `tools`, yang membatasi alat bawaan sesi ke yang dinamainya. Sertakan alat yang Anda inginkan bersama alat bawaan lain yang Anda gunakan
* Atur `CLAUDE_CODE_ENABLE_TODO_TOOLS=1` dalam opsi `env`, seperti yang dilakukan contoh di halaman ini. Di TypeScript, `env` menggantikan lingkungan subproses, jadi sebarkan `...process.env` untuk menyimpan variabel yang diwariskan. Di Python, `env` digabungkan di atas lingkungan yang diwariskan

<h2 id="todo-lifecycle">
  Siklus hidup todo
</h2>

Claude memindahkan setiap todo melalui siklus hidup yang dapat diprediksi:

1. **Dibuat**: Claude menambahkan todo sebagai `pending` ketika mengidentifikasi tugas
2. **Diaktifkan**: Claude menetapkan todo ke `in_progress` ketika memulai pekerjaan
3. **Diselesaikan**: Claude menandainya selesai ketika tugas selesai dengan sukses
4. **Dihapus**: Claude menghapus todo yang tidak lagi dibutuhkan dengan menetapkan `status: "deleted"` dalam panggilan `TaskUpdate`

<h2 id="when-claude-creates-todos">
  Kapan Claude membuat todos
</h2>

Dalam [sesi yang memiliki alat pelacakan tugas](#model-availability), Claude membuat todos untuk sebagian besar pekerjaan multi-langkah, seperti:

* **Tugas multi-langkah yang kompleks** memerlukan tiga atau lebih tindakan yang berbeda
* **Daftar tugas yang disediakan pengguna** ketika beberapa item disebutkan
* **Operasi yang lebih lama** yang mendapat manfaat dari pelacakan kemajuan
* **Permintaan eksplisit** ketika pengguna meminta organisasi todo

Claude dapat melewatkan todos untuk permintaan yang sangat singkat atau satu langkah.

<h2 id="examples">
  Contoh
</h2>

Sebelum menjalankan contoh-contoh ini, instal Claude Agent SDK dengan mengikuti [quickstart](/docs/id/agent-sdk/quickstart). Setiap contoh di halaman ini berbagi pengaturan izin dan perilaku keluar yang sama:

* **Mode izin**: contoh prompt meminta Claude untuk melakukan pekerjaan nyata pada proyek, jadi setiap contoh menetapkan `permissionMode: "acceptEdits"` (TypeScript) atau `permission_mode="acceptEdits"` (Python) untuk menyetujui otomatis pengeditan file yang dihasilkan pekerjaan. Lihat [Mode izin](/docs/id/agent-sdk/permissions#permission-modes) untuk alternatifnya.
* **Batas giliran**: setiap contoh berjalan sampai agen selesai dan menghasilkan pesan hasil akhirnya. Jika sesi mencapai batas giliran terlebih dahulu, pesan hasil tersebut memiliki subtipe `error_max_turns`. Periksa `subtype` untuk mendeteksi penghentian tersebut.
* **Penanganan kesalahan**: contoh-contoh ini menggunakan panggilan `query()` single-shot. Setelah menghasilkan hasil `error_max_turns`, `query()` melempar kesalahan yang mencakup `Reached maximum number of turns`. Setiap contoh membungkus loop-nya dalam blok try untuk keluar dengan bersih ketika itu terjadi. Lihat [Handle the result](/docs/id/agent-sdk/agent-loop#handle-the-result) untuk subtipe hasil.

<Note>
  Pesan sistem tugas, [`SDKTaskNotificationMessage`](/docs/id/agent-sdk/typescript#sdktasknotificationmessage) (TypeScript) atau [`TaskNotificationMessage`](/docs/id/agent-sdk/python#tasknotificationmessage) (Python) di antaranya, melaporkan tugas latar belakang seperti perintah yang dilatarbelakangkan dan subagen. Dalam aliran pesan, Anda melihat aktivitas todo sebagai blok `tool_use` dalam pesan asisten.
</Note>

<h3 id="monitor-todo-changes">
  Memantau perubahan todo
</h3>

Contoh berikut memantau aliran asisten untuk blok `tool_use` `TaskCreate` dan `TaskUpdate` dan mencetak baris `+` dengan subjek setiap tugas baru dan baris pembaruan dengan ID tugas dan status baru setiap perubahan status. Gunakan bentuk ini ketika Anda menginginkan log aktivitas tugas daripada tampilan yang dirender. Baris `+` tidak menyertakan ID yang ditugaskan, jadi log ini tidak dapat mencocokkan pembaruan kembali ke pembuatannya. Untuk mempertahankan korelasi tersebut, tangkap ID seperti yang dilakukan [Display progress in real time](#display-progress-in-real-time).

Input `tool_use` yang dialirkan adalah bentuk mentah yang dipancarkan model. Claude Code memperbaiki beberapa nama kunci yang hampir-tetapi-tidak-benar sebelum eksekusi, memetakan `id` atau `task_id` ke `taskId` dan `active_form` ke `activeForm`, tetapi perbaikan itu tidak tercermin dalam aliran. Baca bidang input `TaskUpdate` secara defensif, seperti yang dilakukan kedua contoh di halaman ini, daripada mengasumsikan nama kanonik selalu ada.

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  try {
    for await (const message of query({
      prompt: "Create a static website with a home page, an about page, and a shared stylesheet, and track progress with todos",
      // Keeps the Task tools on models where Claude Code otherwise doesn't provide them.
      options: { maxTurns: 15, permissionMode: "acceptEdits", env: { ...process.env, CLAUDE_CODE_ENABLE_TODO_TOOLS: "1" } },
    })) {
      if (message.type !== "assistant") continue;
      for (const block of message.message.content) {
        if (block.type !== "tool_use") continue;
        if (block.name === "TaskCreate") {
          const input = block.input as { subject: string };
          console.log(`+ ${input.subject}`);
        } else if (block.name === "TaskUpdate") {
          const input = block.input as {
            taskId?: string;
            id?: string;
            task_id?: string;
            status?: string;
          };
          const taskId = input.taskId ?? input.id ?? input.task_id;
          if (taskId && input.status) console.log(`  ${taskId} -> ${input.status}`);
        }
      }
    }
  } catch (error) {
    // A single-shot query() throws after yielding an error result.
    console.log(`Session ended with an error: ${error}`);
  }
  ```

  ```python Python theme={null}
  import asyncio

  from claude_agent_sdk import query, ClaudeAgentOptions, AssistantMessage, ToolUseBlock

  async def main():
      try:
          async for message in query(
              prompt="Create a static website with a home page, an about page, and a shared stylesheet, and track progress with todos",
              # Keeps the Task tools on models where Claude Code otherwise doesn't provide them.
              options=ClaudeAgentOptions(max_turns=15, permission_mode="acceptEdits", env={"CLAUDE_CODE_ENABLE_TODO_TOOLS": "1"}),
          ):
              if not isinstance(message, AssistantMessage):
                  continue
              for block in message.content:
                  if not isinstance(block, ToolUseBlock):
                      continue
                  if block.name == "TaskCreate":
                      print(f"+ {block.input.get('subject', '')}")
                  elif block.name == "TaskUpdate" and block.input.get("status"):
                      task_id = (
                          block.input.get("taskId")
                          or block.input.get("id")
                          or block.input.get("task_id")
                      )
                      if task_id:
                          print(f"  {task_id} -> {block.input['status']}")
      except Exception as error:
          # A single-shot query() raises after yielding an error result.
          print(f"Session ended with an error: {error}")


  asyncio.run(main())
  ```
</CodeGroup>

<h3 id="display-progress-in-real-time">
  Tampilkan kemajuan secara real-time
</h3>

Contoh berikut memantau aliran asisten untuk blok `tool_use` `TaskCreate` dan `TaskUpdate` dan menyimpan peta tugas yang dikunci berdasarkan ID tugas dalam kelas `TaskTracker`, merender ulang ringkasan kemajuan pada setiap perubahan. Ringkasan menghitung tugas yang diselesaikan dan sedang berlangsung dan menampilkan label `activeForm` setiap item aktif sebagai pengganti `subject`-nya. Gunakan bentuk ini ketika aplikasi Anda mempertahankan tampilan kemajuan daripada mencatat setiap peristiwa.

ID tugas yang ditugaskan tidak ada dalam input `TaskCreate`. Claude Code mengirimkan output terstruktur setiap alat pada pesan pengguna yang membawa blok `tool_result`-nya, dalam bidang `tool_use_result`. Untuk `TaskCreate`, objek itu didokumentasikan untuk TypeScript sebagai `TaskCreateOutput` di bawah [Tool Output Types](/docs/id/agent-sdk/typescript#tool-output-types), dan di Python bidangnya adalah dict biasa dari bentuk yang sama. Pelacak memasangkan setiap blok `tool_result` dengan panggilan `tool_use`-nya berdasarkan `tool_use_id` dan membaca `task.id` dari `tool_use_result` pesan yang dipasangkan. Claude dapat membaca daftar kembali dengan `TaskList` dan detail lengkap satu tugas dengan `TaskGet`.

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  type Task = { subject: string; activeForm?: string; status: string };

  class TaskTracker {
    private tasks = new Map<string, Task>();
    private pendingCreates = new Map<string, { subject: string; activeForm?: string }>();

    displayProgress() {
      if (this.tasks.size === 0) {
        console.log("\nProgress: no open tasks\n");
        return;
      }

      const items = [...this.tasks.values()];
      const completed = items.filter((t) => t.status === "completed").length;
      const inProgress = items.filter((t) => t.status === "in_progress").length;

      console.log(`\nProgress: ${completed}/${this.tasks.size} completed`);
      console.log(`Currently working on: ${inProgress} task(s)\n`);

      for (const [id, task] of this.tasks) {
        const icon =
          task.status === "completed" ? "✅" : task.status === "in_progress" ? "🔧" : "❌";
        const text = task.status === "in_progress" && task.activeForm ? task.activeForm : task.subject;
        console.log(`${id}. ${icon} ${text}`);
      }
    }

    handleToolUse(block: { id: string; name: string; input: unknown }) {
      if (block.name === "TaskCreate") {
        const input = block.input as { subject: string; activeForm?: string; active_form?: string };
        this.pendingCreates.set(block.id, {
          subject: input.subject,
          activeForm: input.activeForm ?? input.active_form,
        });
      } else if (block.name === "TaskUpdate") {
        const input = block.input as {
          taskId?: string;
          id?: string;
          task_id?: string;
          status?: string;
          activeForm?: string;
          active_form?: string;
        };
        const taskId = input.taskId ?? input.id ?? input.task_id;
        if (!taskId) return;
        if (input.status === "deleted") {
          this.tasks.delete(taskId);
          this.displayProgress();
          return;
        }
        const task = this.tasks.get(taskId);
        if (!task) return;
        if (input.status) task.status = input.status;
        const active = input.activeForm ?? input.active_form;
        if (active) task.activeForm = active;
        this.displayProgress();
      }
    }

    handleToolResult(block: { tool_use_id: string; is_error?: boolean }, result: unknown) {
      const create = this.pendingCreates.get(block.tool_use_id);
      if (!create) return;
      this.pendingCreates.delete(block.tool_use_id);
      if (block.is_error) return;
      // The result's user message carries the tool's structured output as
      // tool_use_result; for TaskCreate that's TaskCreateOutput,
      // { task: { id, subject } }.
      const out = result as { task?: { id: string } };
      if (!out?.task?.id) return;
      this.tasks.set(out.task.id, { ...create, status: "pending" });
      this.displayProgress();
    }

    async trackQuery(prompt: string) {
      try {
        for await (const message of query({
          prompt,
          options: { maxTurns: 20, permissionMode: "acceptEdits", env: { ...process.env, CLAUDE_CODE_ENABLE_TODO_TOOLS: "1" } },
        })) {
          if (message.type === "assistant") {
            for (const block of message.message.content) {
              if (block.type === "tool_use") this.handleToolUse(block);
            }
          }
          if (message.type === "user" && Array.isArray(message.message.content)) {
            for (const block of message.message.content) {
              if (block.type === "tool_result") this.handleToolResult(block, message.tool_use_result);
            }
          }
        }
      } catch (error) {
        // A single-shot query() throws after yielding an error result,
        // such as when the maxTurns limit is hit.
        console.log(`Session ended with an error: ${error}`);
      }
    }
  }

  // Usage
  const tracker = new TaskTracker();
  await tracker.trackQuery("Build a complete authentication system with todos");
  ```

  ```python Python theme={null}
  import asyncio

  from claude_agent_sdk import (
      query,
      ClaudeAgentOptions,
      AssistantMessage,
      UserMessage,
      ToolUseBlock,
      ToolResultBlock,
  )


  class TaskTracker:
      def __init__(self):
          self.tasks: dict[str, dict] = {}
          self.pending_creates: dict[str, dict] = {}

      def display_progress(self):
          if not self.tasks:
              print("\nProgress: no open tasks\n")
              return

          completed = len([t for t in self.tasks.values() if t["status"] == "completed"])
          in_progress = len([t for t in self.tasks.values() if t["status"] == "in_progress"])

          print(f"\nProgress: {completed}/{len(self.tasks)} completed")
          print(f"Currently working on: {in_progress} task(s)\n")

          for task_id, task in self.tasks.items():
              icon = (
                  "✅"
                  if task["status"] == "completed"
                  else "🔧"
                  if task["status"] == "in_progress"
                  else "❌"
              )
              text = (
                  task["activeForm"]
                  if task["status"] == "in_progress" and task.get("activeForm")
                  else task["subject"]
              )
              print(f"{task_id}. {icon} {text}")

      def handle_tool_use(self, block: ToolUseBlock):
          if block.name == "TaskCreate":
              self.pending_creates[block.id] = {
                  "subject": block.input.get("subject", ""),
                  "activeForm": block.input.get("activeForm") or block.input.get("active_form"),
              }
          elif block.name == "TaskUpdate":
              task_id = (
                  block.input.get("taskId")
                  or block.input.get("id")
                  or block.input.get("task_id")
              )
              if not task_id:
                  return
              if block.input.get("status") == "deleted":
                  self.tasks.pop(task_id, None)
                  self.display_progress()
                  return
              task = self.tasks.get(task_id)
              if not task:
                  return
              if block.input.get("status"):
                  task["status"] = block.input["status"]
              active = block.input.get("activeForm") or block.input.get("active_form")
              if active:
                  task["activeForm"] = active
              self.display_progress()

      def handle_tool_result(self, block: ToolResultBlock, tool_use_result):
          create = self.pending_creates.pop(block.tool_use_id, None)
          if create is None or block.is_error:
              return
          # The result's user message carries the tool's structured output as
          # tool_use_result; for TaskCreate that's {"task": {"id": ..., "subject": ...}}.
          task = (tool_use_result or {}).get("task") or {}
          if not task.get("id"):
              return
          self.tasks[task["id"]] = {**create, "status": "pending"}
          self.display_progress()

      async def track_query(self, prompt: str):
          try:
              async for message in query(
                  prompt=prompt,
                  options=ClaudeAgentOptions(
                      max_turns=20,
                      permission_mode="acceptEdits",
                      env={"CLAUDE_CODE_ENABLE_TODO_TOOLS": "1"},
                  ),
              ):
                  if isinstance(message, AssistantMessage):
                      for block in message.content:
                          if isinstance(block, ToolUseBlock):
                              self.handle_tool_use(block)
                  if isinstance(message, UserMessage) and isinstance(message.content, list):
                      for block in message.content:
                          if isinstance(block, ToolResultBlock):
                              self.handle_tool_result(block, message.tool_use_result)
          except Exception as error:
              # A single-shot query() raises after yielding an error result,
              # such as when the max_turns limit is hit.
              print(f"Session ended with an error: {error}")


  # Usage
  async def main():
      tracker = TaskTracker()
      await tracker.track_query("Build a complete authentication system with todos")


  asyncio.run(main())
  ```
</CodeGroup>

<h2 id="related-documentation">
  Dokumentasi terkait
</h2>

* [Referensi Agent SDK - TypeScript](/docs/id/agent-sdk/typescript): opsi, tipe, dan skema alat untuk SDK TypeScript, termasuk tipe input dan output alat Task
* [Referensi Agent SDK - Python](/docs/id/agent-sdk/python): opsi, tipe, dan dokumentasi alat untuk SDK Python
* [Streaming Input](/docs/id/agent-sdk/streaming-vs-single-mode): dua mode input, dan kapan menggunakan streaming input daripada panggilan single-shot yang digunakan contoh-contoh ini
* [Berikan Claude alat kustom](/docs/id/agent-sdk/custom-tools): tentukan alat Anda sendiri dengan server MCP in-process SDK
