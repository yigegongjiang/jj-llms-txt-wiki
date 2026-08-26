> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Bekerja dengan sesi

> Bagaimana sesi mempertahankan riwayat percakapan agen, dan kapan menggunakan continue, resume, dan fork untuk kembali ke run sebelumnya.

Sesi adalah riwayat percakapan yang dikumpulkan SDK saat agen Anda bekerja. Sesi berisi prompt Anda, setiap pemanggilan alat yang dibuat agen, setiap hasil alat, dan setiap respons. SDK menulisnya ke disk secara otomatis sehingga Anda dapat kembali ke sesi tersebut nanti.

Kembali ke sesi berarti agen memiliki konteks lengkap dari sebelumnya: file yang sudah dibaca, analisis yang sudah dilakukan, keputusan yang sudah dibuat. Anda dapat mengajukan pertanyaan lanjutan, pulih dari gangguan, atau bercabang untuk mencoba pendekatan berbeda.

<Note>
  Sesi mempertahankan **percakapan**, bukan sistem file. Untuk membuat snapshot dan mengembalikan perubahan file yang dibuat agen, gunakan [file checkpointing](/docs/id/agent-sdk/file-checkpointing).
</Note>

Panduan ini mencakup cara memilih pendekatan yang tepat untuk aplikasi Anda, antarmuka SDK yang melacak sesi secara otomatis, cara menangkap ID sesi dan menggunakan `resume` dan `fork` secara manual, dan apa yang perlu diketahui tentang melanjutkan sesi di seluruh host.

<h2 id="choose-an-approach">
  Pilih pendekatan
</h2>

Seberapa banyak penanganan sesi yang Anda butuhkan tergantung pada bentuk aplikasi Anda. Manajemen sesi menjadi penting ketika Anda mengirim beberapa prompt yang harus berbagi konteks. Dalam satu panggilan `query()`, agen sudah mengambil sebanyak putaran yang diperlukan, dan prompt izin serta `AskUserQuestion` [ditangani dalam loop](/docs/id/agent-sdk/user-input) (mereka tidak mengakhiri panggilan).

| Apa yang Anda bangun                                                   | Apa yang digunakan                                                                                                                                    |
| :--------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------- |
| Tugas sekali jalan: prompt tunggal, tanpa lanjutan                     | Tidak ada yang ekstra. Satu panggilan `query()` menanganinya.                                                                                         |
| Obrolan multi-putaran dalam satu proses                                | [`ClaudeSDKClient` (Python) atau `continue: true` (TypeScript)](#automatic-session-management). SDK melacak sesi untuk Anda tanpa penanganan ID.      |
| Lanjutkan dari mana Anda berhenti setelah restart proses               | `continue_conversation=True` (Python) / `continue: true` (TypeScript). Melanjutkan sesi terbaru di direktori, tidak perlu ID.                         |
| Lanjutkan sesi masa lalu tertentu (bukan yang terbaru)                 | Tangkap ID sesi dan teruskan ke `resume`.                                                                                                             |
| Coba pendekatan alternatif tanpa kehilangan yang asli                  | Fork sesi.                                                                                                                                            |
| Tugas stateless, tidak ingin apa pun ditulis ke disk (TypeScript saja) | Atur [`persistSession: false`](/docs/id/agent-sdk/typescript#options). Sesi hanya ada dalam memori untuk durasi panggilan. Python selalu bertahan ke disk. |

<h3 id="continue-resume-and-fork">
  Continue, resume, dan fork
</h3>

Continue, resume, dan fork adalah bidang opsi yang Anda atur pada `query()` ([`ClaudeAgentOptions`](/docs/id/agent-sdk/python#claudeagentoptions) di Python, [`Options`](/docs/id/agent-sdk/typescript#options) di TypeScript).

**Continue** dan **resume** keduanya mengambil sesi yang ada dan menambahkannya. Perbedaannya adalah cara mereka menemukan sesi tersebut:

* **Continue** menemukan sesi terbaru di direktori saat ini. Anda tidak melacak apa pun. Bekerja dengan baik ketika aplikasi Anda menjalankan satu percakapan pada satu waktu.
* **Resume** mengambil ID sesi tertentu. Anda melacak ID. Diperlukan ketika Anda memiliki beberapa sesi (misalnya, satu per pengguna dalam aplikasi multi-pengguna) atau ingin kembali ke sesi yang bukan yang terbaru.

**Fork** berbeda: fork membuat sesi baru yang dimulai dengan salinan riwayat asli. Asli tetap tidak berubah. Gunakan fork untuk mencoba arah berbeda sambil mempertahankan opsi untuk kembali.

<h2 id="automatic-session-management">
  Manajemen sesi otomatis
</h2>

Kedua SDK menawarkan antarmuka yang melacak status sesi untuk Anda di seluruh panggilan, sehingga Anda tidak perlu melewatkan ID secara manual. Gunakan ini untuk percakapan multi-putaran dalam satu proses.

<h3 id="python-claudesdkclient">
  Python: `ClaudeSDKClient`
</h3>

[`ClaudeSDKClient`](/docs/id/agent-sdk/python#claudesdkclient) menangani ID sesi secara internal. Setiap panggilan ke `client.query()` secara otomatis melanjutkan sesi yang sama. Panggil [`client.receive_response()`](/docs/id/agent-sdk/python#claudesdkclient) untuk mengulangi pesan untuk kueri saat ini. Gunakan klien sebagai manajer konteks async sehingga penyiapan dan pembongkaran koneksi ditangani untuk Anda, atau panggil `connect()` dan `disconnect()` secara manual.

Contoh ini menjalankan dua kueri terhadap `client` yang sama. Yang pertama meminta agen untuk menganalisis modul; yang kedua memintanya untuk refactor modul tersebut. Karena kedua panggilan melalui instance klien yang sama, kueri kedua memiliki konteks lengkap dari yang pertama tanpa `resume` atau ID sesi eksplisit:

```python Python theme={null}
import asyncio
from claude_agent_sdk import (
    ClaudeSDKClient,
    ClaudeAgentOptions,
    AssistantMessage,
    ResultMessage,
    TextBlock,
)


def print_response(message):
    """Print only the human-readable parts of a message."""
    if isinstance(message, AssistantMessage):
        for block in message.content:
            if isinstance(block, TextBlock):
                print(block.text)
    elif isinstance(message, ResultMessage):
        cost = (
            f"${message.total_cost_usd:.4f}"
            if message.total_cost_usd is not None
            else "N/A"
        )
        print(f"[done: {message.subtype}, cost: {cost}]")


async def main():
    options = ClaudeAgentOptions(
        allowed_tools=["Read", "Edit", "Glob", "Grep"],
    )

    async with ClaudeSDKClient(options=options) as client:
        # First query: client captures the session ID internally
        await client.query("Analyze the auth module")
        async for message in client.receive_response():
            print_response(message)

        # Second query: automatically continues the same session
        await client.query("Now refactor it to use JWT")
        async for message in client.receive_response():
            print_response(message)


asyncio.run(main())
```

Lihat [referensi Python SDK](/docs/id/agent-sdk/python#choosing-between-query-and-claudesdkclient) untuk detail tentang kapan menggunakan `ClaudeSDKClient` versus fungsi `query()` mandiri.

<h3 id="typescript-continue-true">
  TypeScript: `continue: true`
</h3>

SDK TypeScript tidak memiliki objek klien yang memegang sesi seperti `ClaudeSDKClient` Python. Sebagai gantinya, teruskan `continue: true` pada setiap panggilan `query()` berikutnya dan SDK mengambil sesi terbaru di direktori saat ini. Tidak perlu pelacakan ID.

Contoh ini membuat dua panggilan `query()` terpisah. Yang pertama membuat sesi segar; yang kedua menetapkan `continue: true`, yang memberitahu SDK untuk menemukan dan melanjutkan sesi terbaru di disk. Agen memiliki konteks lengkap dari panggilan pertama:

```typescript TypeScript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

// First query: creates a new session
for await (const message of query({
  prompt: "Analyze the auth module",
  options: { allowedTools: ["Read", "Glob", "Grep"] }
})) {
  if (message.type === "result" && message.subtype === "success") {
    console.log(message.result);
  }
}

// Second query: continue: true resumes the most recent session
for await (const message of query({
  prompt: "Now refactor it to use JWT",
  options: {
    continue: true,
    allowedTools: ["Read", "Edit", "Write", "Glob", "Grep"]
  }
})) {
  if (message.type === "result" && message.subtype === "success") {
    console.log(message.result);
  }
}
```

<Note>
  [API sesi V2](/docs/id/agent-sdk/typescript-v2-preview) eksperimental, yang menyediakan `createSession()` dengan pola `send` / `stream`, sudah dihapus di TypeScript Agent SDK 0.3.142. Gunakan fungsi `query()` dan opsi sesi yang dijelaskan di halaman ini sebagai gantinya.
</Note>

<h2 id="use-session-options-with-query">
  Gunakan opsi sesi dengan `query()`
</h2>

<h3 id="capture-the-session-id">
  Tangkap ID sesi
</h3>

Resume dan fork memerlukan ID sesi. Bacanya dari bidang `session_id` pada pesan hasil ([`ResultMessage`](/docs/id/agent-sdk/python#resultmessage) di Python, [`SDKResultMessage`](/docs/id/agent-sdk/typescript#sdkresultmessage) di TypeScript), yang ada di setiap hasil terlepas dari kesuksesan atau kesalahan. Di TypeScript ID juga tersedia lebih awal sebagai bidang langsung pada `SystemMessage` init; di Python itu bersarang di dalam `SystemMessage.data`.

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage


  async def main():
      session_id = None

      async for message in query(
          prompt="Analyze the auth module and suggest improvements",
          options=ClaudeAgentOptions(
              allowed_tools=["Read", "Glob", "Grep"],
          ),
      ):
          if isinstance(message, ResultMessage):
              session_id = message.session_id
              if message.subtype == "success":
                  print(message.result)

      print(f"Session ID: {session_id}")
      return session_id


  session_id = asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  let sessionId: string | undefined;

  for await (const message of query({
    prompt: "Analyze the auth module and suggest improvements",
    options: { allowedTools: ["Read", "Glob", "Grep"] }
  })) {
    if (message.type === "result") {
      sessionId = message.session_id;
      if (message.subtype === "success") {
        console.log(message.result);
      }
    }
  }

  console.log(`Session ID: ${sessionId}`);
  ```
</CodeGroup>

<h3 id="resume-by-id">
  Lanjutkan berdasarkan ID
</h3>

Teruskan ID sesi ke `resume` untuk kembali ke sesi tertentu tersebut. Agen mengambil dengan konteks lengkap dari mana pun sesi berakhir. Alasan umum untuk melanjutkan:

* **Lanjutkan tugas yang selesai.** Agen sudah menganalisis sesuatu; sekarang Anda ingin itu bertindak atas analisis tersebut tanpa membaca ulang file.
* **Pulih dari batas.** Run pertama berakhir dengan `error_max_turns` atau `error_max_budget_usd` (lihat [Tangani hasil](/docs/id/agent-sdk/agent-loop#handle-the-result)); lanjutkan dengan batas yang lebih tinggi.
* **Mulai ulang proses Anda.** Anda menangkap ID sebelum shutdown dan ingin memulihkan percakapan.

Contoh ini melanjutkan sesi dari [Tangkap ID sesi](#capture-the-session-id) dengan prompt lanjutan. Karena Anda melanjutkan, agen sudah memiliki analisis sebelumnya dalam konteks:

<CodeGroup>
  ```python Python theme={null}
  # Earlier session analyzed the code; now build on that analysis
  async for message in query(
      prompt="Now implement the refactoring you suggested",
      options=ClaudeAgentOptions(
          resume=session_id,
          allowed_tools=["Read", "Edit", "Write", "Glob", "Grep"],
      ),
  ):
      if isinstance(message, ResultMessage) and message.subtype == "success":
          print(message.result)
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const sessionId = "..."; // The ID you captured in the previous example

  // Earlier session analyzed the code; now build on that analysis
  for await (const message of query({
    prompt: "Now implement the refactoring you suggested",
    options: {
      resume: sessionId,
      allowedTools: ["Read", "Edit", "Write", "Glob", "Grep"]
    }
  })) {
    if (message.type === "result" && message.subtype === "success") {
      console.log(message.result);
    }
  }
  ```
</CodeGroup>

Anda seharusnya melihat respons yang dibangun atas analisis sebelumnya alih-alih memulai dari awal. Itu mengkonfirmasi bahwa agen melanjutkan sesi dengan konteks sebelumnya tetap utuh.

<Tip>
  Jika panggilan `resume` mengembalikan sesi segar alih-alih riwayat yang diharapkan, penyebab paling umum adalah `cwd` yang tidak cocok. Sesi disimpan di bawah `~/.claude/projects/<encoded-cwd>/*.jsonl`, atau di bawah `$CLAUDE_CONFIG_DIR/projects/<encoded-cwd>/*.jsonl` jika Anda menetapkan variabel lingkungan `CLAUDE_CONFIG_DIR`, di mana `<encoded-cwd>` adalah direktori kerja absolut dengan setiap karakter non-alfanumerik diganti dengan `-` (jadi `/Users/me/proj` menjadi `-Users-me-proj`). Jika panggilan resume Anda berjalan dari direktori berbeda, SDK mencari di tempat yang salah. File sesi juga perlu ada di mesin saat ini.
</Tip>

Untuk melanjutkan sesi di seluruh mesin atau di lingkungan serverless, cerminkan transkrip ke penyimpanan bersama dengan adaptor [`SessionStore`](/docs/id/agent-sdk/session-storage).

<h3 id="fork-to-explore-alternatives">
  Fork untuk menjelajahi alternatif
</h3>

Forking membuat sesi baru yang dimulai dengan salinan riwayat asli tetapi menyimpang dari titik itu. Fork mendapatkan ID sesi sendiri; ID asli dan riwayat tetap tidak berubah. Anda berakhir dengan dua sesi independen yang dapat Anda lanjutkan secara terpisah.

<Note>
  Forking membuat cabang riwayat percakapan, bukan sistem file. Jika agen yang di-fork mengedit file, perubahan tersebut nyata dan terlihat oleh sesi apa pun yang bekerja di direktori yang sama. Untuk membuat cabang dan mengembalikan perubahan file, gunakan [file checkpointing](/docs/id/agent-sdk/file-checkpointing).
</Note>

Contoh ini dibangun di atas [Tangkap ID sesi](#capture-the-session-id): Anda sudah menganalisis modul auth di `session_id` dan ingin menjelajahi OAuth2 tanpa kehilangan utas yang berfokus pada JWT. Blok pertama fork sesi dan menangkap ID fork (`forked_id`); blok kedua melanjutkan `session_id` asli untuk melanjutkan jalur yang berfokus pada JWT. Anda sekarang memiliki dua ID sesi yang menunjuk ke dua riwayat terpisah:

<CodeGroup>
  ```python Python theme={null}
  # Fork: branch from session_id into a new session
  forked_id = None
  async for message in query(
      prompt="Instead of JWT, outline how OAuth2 would work for the auth module",
      options=ClaudeAgentOptions(
          resume=session_id,
          fork_session=True,
          max_turns=5,
      ),
  ):
      if isinstance(message, ResultMessage):
          forked_id = message.session_id  # The fork's ID, distinct from session_id
          if message.subtype == "success":
              print(message.result)

  print(f"Forked session: {forked_id}")

  # Original session is untouched; resuming it continues the JWT thread
  async for message in query(
      prompt="Continue with the JWT approach",
      options=ClaudeAgentOptions(resume=session_id),
  ):
      if isinstance(message, ResultMessage) and message.subtype == "success":
          print(message.result)
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const sessionId = "..."; // The ID you captured in the previous example

  // Fork: branch from sessionId into a new session
  let forkedId: string | undefined;

  for await (const message of query({
    prompt: "Instead of JWT, outline how OAuth2 would work for the auth module",
    options: {
      resume: sessionId,
      forkSession: true,
      maxTurns: 5
    }
  })) {
    if (message.type === "system" && message.subtype === "init") {
      forkedId = message.session_id; // The fork's ID, distinct from sessionId
    }
    if (message.type === "result" && message.subtype === "success") {
      console.log(message.result);
    }
  }

  console.log(`Forked session: ${forkedId}`);

  // Original session is untouched; resuming it continues the JWT thread
  for await (const message of query({
    prompt: "Continue with the JWT approach",
    options: { resume: sessionId }
  })) {
    if (message.type === "result" && message.subtype === "success") {
      console.log(message.result);
    }
  }
  ```
</CodeGroup>

Anda seharusnya melihat bahwa `forkedId` berbeda dari ID sesi asli. Melanjutkan sesi asli masih melanjutkan utas JWT, yang mengkonfirmasi bahwa fork tidak memodifikasi riwayat asli.

<h2 id="resume-across-hosts">
  Lanjutkan di seluruh host
</h2>

File sesi bersifat lokal untuk mesin yang membuatnya. Untuk melanjutkan sesi di host berbeda (pekerja CI, kontainer ephemeral, serverless), Anda memiliki dua opsi:

* **Pindahkan file sesi.** Pertahankan `~/.claude/projects/<encoded-cwd>/<session-id>.jsonl` dari run pertama dan pulihkan ke jalur yang sama di host baru sebelum memanggil `resume`. `cwd` harus cocok.
* **Jangan andalkan resume sesi.** Tangkap hasil yang Anda butuhkan (output analisis, keputusan, diff file) sebagai status aplikasi dan teruskan ke prompt sesi segar. Ini sering lebih kuat daripada mengirim file transkrip.

Kedua SDK mengekspos fungsi untuk menghitung sesi di disk dan membaca pesan mereka: [`listSessions()`](/docs/id/agent-sdk/typescript#listsessions) dan [`getSessionMessages()`](/docs/id/agent-sdk/typescript#getsessionmessages) di TypeScript, [`list_sessions()`](/docs/id/agent-sdk/python#list_sessions) dan [`get_session_messages()`](/docs/id/agent-sdk/python#get_session_messages) di Python. Gunakan mereka untuk membangun pemilih sesi kustom, logika pembersihan, atau penampil transkrip.

Kedua SDK juga mengekspos fungsi untuk mencari dan mengubah sesi individual: [`get_session_info()`](/docs/id/agent-sdk/python#get_session_info), [`rename_session()`](/docs/id/agent-sdk/python#rename_session), dan [`tag_session()`](/docs/id/agent-sdk/python#tag_session) di Python, dan [`getSessionInfo()`](/docs/id/agent-sdk/typescript#getsessioninfo), [`renameSession()`](/docs/id/agent-sdk/typescript#renamesession), dan [`tagSession()`](/docs/id/agent-sdk/typescript#tagsession) di TypeScript. Gunakan mereka untuk mengorganisir sesi berdasarkan tag atau memberi mereka judul yang dapat dibaca manusia.

<h2 id="related-resources">
  Sumber daya terkait
</h2>

* [Bagaimana loop agen bekerja](/docs/id/agent-sdk/agent-loop): Pahami putaran, pesan, dan akumulasi konteks dalam sesi
* [File checkpointing](/docs/id/agent-sdk/file-checkpointing): Lacak dan kembalikan perubahan file di seluruh sesi
* [Python `ClaudeAgentOptions`](/docs/id/agent-sdk/python#claudeagentoptions): Referensi opsi sesi lengkap untuk Python
* [TypeScript `Options`](/docs/id/agent-sdk/typescript#options): Referensi opsi sesi lengkap untuk TypeScript
