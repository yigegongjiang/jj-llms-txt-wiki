> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Daftar Todo

> Lacak dan tampilkan todos menggunakan Claude Agent SDK untuk manajemen tugas yang terorganisir

Pelacakan todo menyediakan cara terstruktur untuk mengelola tugas dan menampilkan kemajuan kepada pengguna. Claude Agent SDK mencakup fungsionalitas todo bawaan yang membantu mengorganisir alur kerja yang kompleks dan membuat pengguna tetap terinformasi tentang perkembangan tugas.

<Note>
  Mulai dari TypeScript Agent SDK 0.3.142 dan Claude Code v2.1.142, sesi menggunakan alat Task terstruktur `TaskCreate`, `TaskUpdate`, `TaskGet`, dan `TaskList` sebagai pengganti `TodoWrite`. Python SDK mendapatkan perubahan ini dari Claude Code CLI yang diluncurkannya, bukan dari versi paket Python: pengalihan berlaku setelah CLI tersebut — salinan yang disertakan dalam paket pip, atau yang Anda tunjuk dengan `cli_path` — adalah v2.1.142 atau lebih baru. Lihat [Migrasi ke alat Task](#migrate-to-task-tools) untuk cara memantau perubahan kode. Contoh di halaman ini menetapkan `CLAUDE_CODE_ENABLE_TASKS=0` untuk terus menampilkan `TodoWrite` untuk sesi yang belum bermigrasi.
</Note>

<h3 id="todo-lifecycle">
  Siklus Hidup Todo
</h3>

Todos mengikuti siklus hidup yang dapat diprediksi:

1. **Dibuat** sebagai `pending` ketika tugas diidentifikasi
2. **Diaktifkan** menjadi `in_progress` ketika pekerjaan dimulai
3. **Diselesaikan** ketika tugas selesai dengan sukses
4. **Dihapus** ketika semua tugas dalam grup selesai

<h3 id="when-todos-are-used">
  Kapan Todos Digunakan
</h3>

SDK membuat todos untuk sebagian besar pekerjaan multi-langkah, seperti:

* **Tugas multi-langkah yang kompleks** memerlukan 3 atau lebih tindakan yang berbeda
* **Daftar tugas yang disediakan pengguna** ketika beberapa item disebutkan
* **Operasi non-trivial** yang mendapat manfaat dari pelacakan kemajuan
* **Permintaan eksplisit** ketika pengguna meminta organisasi todo

Mungkin melewatkan todos untuk permintaan yang sangat singkat atau satu langkah.

<h2 id="examples">
  Contoh
</h2>

Sebelum menjalankan contoh-contoh ini, instal Claude Agent SDK dengan mengikuti [quickstart](/docs/id/agent-sdk/quickstart).

Setiap contoh berjalan sampai agen selesai dan menghasilkan pesan hasil akhirnya. Jika sesi mencapai batas giliran terlebih dahulu, pesan hasil tersebut memiliki subtipe `error_max_turns`. Periksa `subtype` untuk mendeteksi penghentian tersebut.

Contoh-contoh ini menggunakan panggilan `query()` single-shot. Setelah menghasilkan hasil `error_max_turns`, `query()` melempar kesalahan yang mencakup `Reached maximum number of turns`. Setiap contoh membungkus loop-nya dalam blok try untuk keluar dengan bersih ketika itu terjadi.

Lihat [Handle the result](/docs/id/agent-sdk/agent-loop#handle-the-result) untuk subtipe hasil.

<h3 id="monitoring-todo-changes">
  Memantau Perubahan Todo
</h3>

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  try {
    for await (const message of query({
      prompt: "Optimize my React app performance and track progress with todos",
      // Re-enable TodoWrite, which this example monitors. Without it, the SDK uses
      // Task tools instead and these tool_use blocks never appear.
      options: { maxTurns: 15, env: { ...process.env, CLAUDE_CODE_ENABLE_TASKS: "0" } }
    })) {
      // Todo updates are reflected in the message stream
      if (message.type === "assistant") {
        for (const block of message.message.content) {
          if (block.type === "tool_use" && block.name === "TodoWrite") {
            const todos = block.input.todos;

            console.log("Todo Status Update:");
            todos.forEach((todo, index) => {
              const status =
                todo.status === "completed" ? "✅" : todo.status === "in_progress" ? "🔧" : "❌";
              console.log(`${index + 1}. ${status} ${todo.content}`);
            });
          }
        }
      }
    }
  } catch (error) {
    // A single-shot query() throws after yielding an error result,
    // such as when the maxTurns limit is hit.
    console.log(`Session ended with an error: ${error}`);
  }
  ```

  ```python Python theme={null}
  import asyncio

  from claude_agent_sdk import query, ClaudeAgentOptions, AssistantMessage, ToolUseBlock


  async def main():
      try:
          async for message in query(
              prompt="Optimize my React app performance and track progress with todos",
              # Re-enable TodoWrite, which this example monitors. Without it, the SDK uses
              # Task tools instead and these tool_use blocks never appear.
              options=ClaudeAgentOptions(max_turns=15, env={"CLAUDE_CODE_ENABLE_TASKS": "0"}),
          ):
              # Todo updates are reflected in the message stream
              if isinstance(message, AssistantMessage):
                  for block in message.content:
                      if isinstance(block, ToolUseBlock) and block.name == "TodoWrite":
                          todos = block.input["todos"]

                          print("Todo Status Update:")
                          for i, todo in enumerate(todos):
                              status = (
   "✅"
   if todo["status"] == "completed"
   else "🔧"
   if todo["status"] == "in_progress"
   else "❌"
                              )
                              print(f"{i + 1}. {status} {todo['content']}")
      except Exception as error:
          # A single-shot query() raises after yielding an error result,
          # such as when the max_turns limit is hit.
          print(f"Session ended with an error: {error}")


  asyncio.run(main())
  ```
</CodeGroup>

<h3 id="real-time-progress-display">
  Tampilan Kemajuan Real-time
</h3>

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  class TodoTracker {
    private todos: any[] = [];

    displayProgress() {
      if (this.todos.length === 0) return;

      const completed = this.todos.filter((t) => t.status === "completed").length;
      const inProgress = this.todos.filter((t) => t.status === "in_progress").length;
      const total = this.todos.length;

      console.log(`\nProgress: ${completed}/${total} completed`);
      console.log(`Currently working on: ${inProgress} task(s)\n`);

      this.todos.forEach((todo, index) => {
        const icon =
          todo.status === "completed" ? "✅" : todo.status === "in_progress" ? "🔧" : "❌";
        const text = todo.status === "in_progress" ? todo.activeForm : todo.content;
        console.log(`${index + 1}. ${icon} ${text}`);
      });
    }

    async trackQuery(prompt: string) {
      try {
        for await (const message of query({
          prompt,
          // Re-enable TodoWrite, which this tracker watches for.
          options: { maxTurns: 20, env: { ...process.env, CLAUDE_CODE_ENABLE_TASKS: "0" } }
        })) {
          if (message.type === "assistant") {
            for (const block of message.message.content) {
              if (block.type === "tool_use" && block.name === "TodoWrite") {
                this.todos = block.input.todos;
                this.displayProgress();
              }
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
  const tracker = new TodoTracker();
  await tracker.trackQuery("Build a complete authentication system with todos");
  ```

  ```python Python theme={null}
  import asyncio

  from claude_agent_sdk import query, ClaudeAgentOptions, AssistantMessage, ToolUseBlock
  from typing import List, Dict


  class TodoTracker:
      def __init__(self):
          self.todos: List[Dict] = []

      def display_progress(self):
          if not self.todos:
              return

          completed = len([t for t in self.todos if t["status"] == "completed"])
          in_progress = len([t for t in self.todos if t["status"] == "in_progress"])
          total = len(self.todos)

          print(f"\nProgress: {completed}/{total} completed")
          print(f"Currently working on: {in_progress} task(s)\n")

          for i, todo in enumerate(self.todos):
              icon = (
                  "✅"
                  if todo["status"] == "completed"
                  else "🔧"
                  if todo["status"] == "in_progress"
                  else "❌"
              )
              text = (
                  todo["activeForm"]
                  if todo["status"] == "in_progress"
                  else todo["content"]
              )
              print(f"{i + 1}. {icon} {text}")

      async def track_query(self, prompt: str):
          try:
              async for message in query(
                  prompt=prompt,
                  # Re-enable TodoWrite, which this tracker watches for.
                  options=ClaudeAgentOptions(max_turns=20, env={"CLAUDE_CODE_ENABLE_TASKS": "0"}),
              ):
                  if isinstance(message, AssistantMessage):
                      for block in message.content:
                          if isinstance(block, ToolUseBlock) and block.name == "TodoWrite":
                              self.todos = block.input["todos"]
                              self.display_progress()
          except Exception as error:
              # A single-shot query() raises after yielding an error result,
              # such as when the max_turns limit is hit.
              print(f"Session ended with an error: {error}")


  # Usage
  async def main():
      tracker = TodoTracker()
      await tracker.track_query("Build a complete authentication system with todos")


  asyncio.run(main())
  ```
</CodeGroup>

<h2 id="migrate-to-task-tools">
  Migrasi ke alat Task
</h2>

Alat Task membagi panggilan `TodoWrite` tunggal menjadi `TaskCreate` untuk setiap item baru dan `TaskUpdate` untuk setiap perubahan status, dengan `TaskList` dan `TaskGet` tersedia untuk model membaca kembali daftar saat ini. Kode pemantauan Anda masih memeriksa blok `tool_use` dalam aliran asisten, tetapi mempertahankan peta yang dikunci berdasarkan ID tugas alih-alih mengganti seluruh daftar pada setiap panggilan. Alat Task adalah default mulai dari TypeScript Agent SDK 0.3.142 dan Claude Code v2.1.142, jadi tidak ada perubahan `options.env` yang diperlukan.

| Dengan `TodoWrite`                                      | Dengan alat Task                                                                                                                                                                                                                                                                                                 |
| ------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Satu panggilan alat menulis ulang array `todos` lengkap | `TaskCreate` menambahkan satu item, `TaskUpdate` menambal satu item berdasarkan `taskId`                                                                                                                                                                                                                         |
| Cocokkan `block.name === "TodoWrite"`                   | Cocokkan `block.name === "TaskCreate"` atau `"TaskUpdate"`                                                                                                                                                                                                                                                       |
| Bentuk item: `{ content, status, activeForm }`          | Input `TaskCreate`: `{ subject, description, activeForm?, metadata? }`. Input `TaskUpdate`: `{ taskId, status?, subject?, description?, activeForm?, addBlocks?, addBlockedBy?, owner?, metadata? }`. `status` adalah `"pending"`, `"in_progress"`, atau `"completed"`; atur `status: "deleted"` untuk menghapus |
| Render `block.input.todos` secara langsung              | Akumulasi item di seluruh panggilan, atau baca snapshot dari hasil alat `TaskList`                                                                                                                                                                                                                               |

ID tugas yang ditugaskan tidak ada dalam input `TaskCreate`. Ini kembali dalam `tool_result` yang cocok sebagai `{ task: { id, subject } }`, jadi tangkap dari blok hasil untuk mengunci peta Anda. Contoh berikut menunjukkan perubahan minimal ke loop [Memantau Perubahan Todo](#monitoring-todo-changes). Ini membaca hanya input `tool_use` dan melewatkan penangkapan ID dari blok `tool_result`. Untuk merender daftar lengkap, pantau hasil alat `TaskList` dalam aliran atau akumulasi hasil `TaskCreate` dan input `TaskUpdate` ke dalam peta.

Input `tool_use` yang dialirkan adalah bentuk mentah yang dipancarkan model. Claude Code memperbaiki beberapa nama kunci yang hampir-tetapi-tidak-benar sebelum eksekusi, memetakan `id` atau `task_id` ke `taskId` dan `active_form` ke `activeForm`, tetapi perbaikan itu tidak tercermin dalam aliran. Baca bidang input `TaskUpdate` secara defensif, seperti yang dilakukan sampel di bawah ini, daripada mengasumsikan nama kanonik selalu ada.

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  try {
    for await (const message of query({
      prompt: "Optimize my React app performance and track progress with todos",
      options: { maxTurns: 15 },
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
              prompt="Optimize my React app performance and track progress with todos",
              options=ClaudeAgentOptions(max_turns=15),
          ):
              if not isinstance(message, AssistantMessage):
                  continue
              for block in message.content:
                  if not isinstance(block, ToolUseBlock):
                      continue
                  if block.name == "TaskCreate":
                      print(f"+ {block.input['subject']}")
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

<h2 id="related-documentation">
  Dokumentasi Terkait
</h2>

* [Referensi TypeScript SDK](/docs/id/agent-sdk/typescript)
* [Referensi Python SDK](/docs/id/agent-sdk/python)
* [Mode Streaming vs Mode Tunggal](/docs/id/agent-sdk/streaming-vs-single-mode)
* [Alat Kustom](/docs/id/agent-sdk/custom-tools)
