> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Panduan Cepat

> Mulai dengan Agent SDK Python atau TypeScript untuk membangun agen AI yang bekerja secara mandiri

Gunakan Agent SDK untuk membangun agen AI yang membaca kode Anda, menemukan bug, dan memperbaikinya, semuanya tanpa intervensi manual.

**Yang akan Anda lakukan:**

1. Menyiapkan proyek dengan Agent SDK
2. Membuat file dengan beberapa kode yang berisi bug
3. Menjalankan agen yang menemukan dan memperbaiki bug secara otomatis

<h2 id="prerequisites">
  Prasyarat
</h2>

* **Node.js 18+** atau **Python 3.10+**
* Akun **Anthropic** ([daftar di sini](https://platform.claude.com/))

<h2 id="setup">
  Penyiapan
</h2>

<Steps>
  <Step title="Buat folder proyek">
    Buat direktori baru untuk panduan cepat ini:

    ```bash theme={null}
    mkdir my-agent
    cd my-agent
    ```

    Untuk proyek Anda sendiri, Anda dapat menjalankan SDK dari folder apa pun; SDK akan memiliki akses ke file di direktori tersebut dan subdirektorinya secara default.
  </Step>

  <Step title="Instal SDK">
    Instal paket Agent SDK untuk bahasa Anda:

    <Tabs>
      <Tab title="TypeScript (proyek baru)">
        ```bash theme={null}
        npm init -y
        npm pkg set type=module
        npm install @anthropic-ai/claude-agent-sdk
        npm install --save-dev tsx
        ```

        Mengatur `"type": "module"` di `package.json` memungkinkan skrip agen Anda menggunakan `await` tingkat atas, dan [tsx](https://tsx.is) menjalankan file TypeScript secara langsung.
      </Tab>

      <Tab title="TypeScript (proyek yang ada)">
        ```bash theme={null}
        npm install @anthropic-ai/claude-agent-sdk
        npm install --save-dev tsx
        ```

        [tsx](https://tsx.is) menjalankan file TypeScript secara langsung. Jika proyek Anda menggunakan CommonJS, beri nama skrip agen Anda `agent.mts` alih-alih `agent.ts`. Ekstensi `.mts` membuat tsx memperlakukan file sebagai modul ES, sehingga `await` tingkat atas berfungsi tanpa mengonversi seluruh proyek Anda ke modul ES. Gunakan `agent.mts` sebagai pengganti `agent.ts` dalam langkah buat dan jalankan nanti dalam panduan cepat ini.
      </Tab>

      <Tab title="Python (uv)">
        [uv](https://docs.astral.sh/uv/) adalah pengelola paket Python yang cepat yang menangani lingkungan virtual secara otomatis:

        ```bash theme={null}
        uv init
        uv add claude-agent-sdk
        ```
      </Tab>

      <Tab title="Python (pip)">
        Buat dan aktifkan lingkungan virtual, kemudian instal paket.

        Di macOS atau Linux:

        ```bash theme={null}
        python3 -m venv .venv
        source .venv/bin/activate
        pip install claude-agent-sdk
        ```

        Di Windows:

        ```powershell theme={null}
        py -m venv .venv
        .venv\Scripts\Activate.ps1
        pip install claude-agent-sdk
        ```

        Jika PowerShell memblokir `Activate.ps1` dengan kesalahan kebijakan eksekusi, jalankan `Set-ExecutionPolicy -Scope Process RemoteSigned` terlebih dahulu.
      </Tab>
    </Tabs>

    <Note>
      TypeScript SDK menggabungkan biner Claude Code asli untuk platform Anda sebagai dependensi opsional, jadi Anda tidak perlu menginstal Claude Code secara terpisah.
    </Note>
  </Step>

  <Step title="Atur kunci API Anda">
    Dapatkan kunci API dari [Claude Console](https://platform.claude.com/), kemudian atur sebagai variabel lingkungan di shell tempat Anda akan menjalankan agen Anda:

    <Tabs>
      <Tab title="macOS / Linux">
        ```bash theme={null}
        export ANTHROPIC_API_KEY=your-api-key
        ```
      </Tab>

      <Tab title="Windows (PowerShell)">
        ```powershell theme={null}
        $env:ANTHROPIC_API_KEY = "your-api-key"
        ```
      </Tab>
    </Tabs>

    SDK membaca kunci dari lingkungan proses yang menjalankan agen Anda; SDK tidak memuat file `.env` secara otomatis. Jika Anda menyimpan kunci di file `.env`, muat sendiri, misalnya dengan paket `dotenv`, sebelum memanggil SDK.

    SDK juga mendukung autentikasi melalui penyedia API pihak ketiga:

    * **Amazon Bedrock**: atur variabel lingkungan `CLAUDE_CODE_USE_BEDROCK=1` dan konfigurasikan kredensial AWS
    * **Claude Platform on AWS**: atur `CLAUDE_CODE_USE_ANTHROPIC_AWS=1` dan `ANTHROPIC_AWS_WORKSPACE_ID`, kemudian konfigurasikan kredensial AWS
    * **Google Cloud's Agent Platform**: atur variabel lingkungan `CLAUDE_CODE_USE_VERTEX=1` dan konfigurasikan kredensial Google Cloud
    * **Microsoft Azure**: atur variabel lingkungan `CLAUDE_CODE_USE_FOUNDRY=1` dan konfigurasikan kredensial Azure

    Lihat panduan penyiapan untuk [Amazon Bedrock](/docs/id/amazon-bedrock), [Claude Platform on AWS](/docs/id/claude-platform-on-aws), [Google Cloud's Agent Platform](/docs/id/google-vertex-ai), atau [Microsoft Foundry](/docs/id/microsoft-foundry) untuk detail selengkapnya.

    <Note>
      Kecuali telah disetujui sebelumnya, Anthropic tidak mengizinkan pengembang pihak ketiga untuk menawarkan login claude.ai atau batas laju untuk produk mereka, termasuk agen yang dibangun di Agent SDK Claude. Silakan gunakan metode autentikasi kunci API yang dijelaskan dalam dokumen ini.
    </Note>
  </Step>
</Steps>

<h2 id="create-a-buggy-file">
  Buat file dengan bug
</h2>

Panduan cepat ini memandu Anda melalui pembuatan agen yang dapat menemukan dan memperbaiki bug dalam kode. Pertama, Anda memerlukan file dengan beberapa bug yang disengaja untuk diperbaiki oleh agen. Buat `utils.py` di direktori `my-agent` dan tempel kode berikut:

```python theme={null}
def calculate_average(numbers):
    total = 0
    for num in numbers:
        total += num
    return total / len(numbers)


def get_user_name(user):
    return user["name"].upper()
```

Kode ini memiliki dua bug:

1. `calculate_average([])` mogok dengan pembagian oleh nol
2. `get_user_name(None)` mogok dengan TypeError

<h2 id="build-an-agent-that-finds-and-fixes-bugs">
  Bangun agen yang menemukan dan memperbaiki bug
</h2>

Buat `agent.py` jika Anda menggunakan Python SDK, atau `agent.ts` untuk TypeScript. Gunakan `agent.mts` sebagai gantinya jika proyek yang ada Anda menggunakan CommonJS:

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, AssistantMessage, ResultMessage


  async def main():
      # Agentic loop: streams messages as Claude works
      async for message in query(
          prompt="Review utils.py for bugs that would cause crashes. Fix any issues you find.",
          options=ClaudeAgentOptions(
              allowed_tools=["Read", "Edit", "Glob"],  # Auto-approve these tools
              permission_mode="acceptEdits",  # Auto-approve file edits
          ),
      ):
          # Print human-readable output
          if isinstance(message, AssistantMessage):
              for block in message.content:
                  if hasattr(block, "text"):
                      print(block.text)  # Claude's reasoning
                  elif hasattr(block, "name"):
                      print(f"Tool: {block.name}")  # Tool being called
          elif isinstance(message, ResultMessage):
              print(f"Done: {message.subtype}")  # Final result


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // Agentic loop: streams messages as Claude works
  for await (const message of query({
    prompt: "Review utils.py for bugs that would cause crashes. Fix any issues you find.",
    options: {
      allowedTools: ["Read", "Edit", "Glob"], // Auto-approve these tools
      permissionMode: "acceptEdits" // Auto-approve file edits
    }
  })) {
    // Print human-readable output
    if (message.type === "assistant" && message.message?.content) {
      for (const block of message.message.content) {
        if ("text" in block) {
          console.log(block.text); // Claude's reasoning
        } else if ("name" in block) {
          console.log(`Tool: ${block.name}`); // Tool being called
        }
      }
    } else if (message.type === "result") {
      console.log(`Done: ${message.subtype}`); // Final result
    }
  }
  ```
</CodeGroup>

Kode ini memiliki tiga bagian utama:

1. **`query`**: titik masuk utama yang membuat loop agentic. Ini mengembalikan iterator async, jadi Anda menggunakan `async for` untuk streaming pesan saat Claude bekerja. Lihat API lengkap di referensi SDK [Python](/docs/id/agent-sdk/python#query) atau [TypeScript](/docs/id/agent-sdk/typescript#query).

2. **`prompt`**: apa yang ingin Anda lakukan Claude. Claude mengetahui alat mana yang digunakan berdasarkan tugas.

3. **`options`**: konfigurasi untuk agen. Contoh ini menggunakan `allowedTools` untuk pra-persetujuan `Read`, `Edit`, dan `Glob`, dan `permissionMode: "acceptEdits"` untuk auto-persetujuan perubahan file. Opsi lainnya termasuk `systemPrompt`, `mcpServers`, dan lainnya. Lihat semua opsi untuk [Python](/docs/id/agent-sdk/python#claudeagentoptions) atau [TypeScript](/docs/id/agent-sdk/typescript#options).

Loop `async for` terus berjalan saat Claude berpikir, memanggil alat, mengamati hasil, dan memutuskan apa yang harus dilakukan selanjutnya. Setiap iterasi menghasilkan pesan: penalaran Claude, panggilan alat, hasil alat, atau hasil akhir. SDK menangani orkestrasi (eksekusi alat, manajemen konteks, percobaan ulang) sehingga Anda hanya mengonsumsi aliran. Loop berakhir ketika Claude menyelesaikan tugas atau mengalami kesalahan.

Penanganan pesan di dalam loop memfilter output yang dapat dibaca manusia. Tanpa penyaringan, Anda akan melihat objek pesan mentah termasuk inisialisasi sistem dan status internal, yang berguna untuk debugging tetapi berisik sebaliknya.

<Note>
  Contoh ini menggunakan streaming untuk menampilkan kemajuan secara real-time. Jika Anda tidak memerlukan output langsung (misalnya, untuk pekerjaan latar belakang atau pipeline CI), Anda dapat mengumpulkan semua pesan sekaligus. Lihat [Streaming vs. single-turn mode](/docs/id/agent-sdk/streaming-vs-single-mode) untuk detail selengkapnya.
</Note>

<h3 id="run-your-agent">
  Jalankan agen Anda
</h3>

Agen Anda siap. Jalankan dengan perintah berikut:

<Tabs>
  <Tab title="TypeScript">
    ```bash theme={null}
    npx tsx agent.ts
    ```

    Jika Anda memberi nama skrip Anda `agent.mts`, jalankan `npx tsx agent.mts` sebagai gantinya.
  </Tab>

  <Tab title="Python (uv)">
    ```bash theme={null}
    uv run agent.py
    ```
  </Tab>

  <Tab title="Python (pip)">
    Dengan lingkungan virtual Anda masih diaktifkan:

    ```bash theme={null}
    python agent.py
    ```
  </Tab>
</Tabs>

Saat bekerja, agen mencetak penalarannya dan setiap alat yang dipanggilnya, diakhiri dengan `Done: success`. Setelah menjalankan, periksa `utils.py`. Anda akan melihat kode defensif yang menangani daftar kosong dan pengguna null. Agen Anda secara mandiri:

1. **Membaca** `utils.py` untuk memahami kode
2. **Menganalisis** logika dan mengidentifikasi kasus tepi yang akan mogok
3. **Mengedit** file untuk menambahkan penanganan kesalahan yang tepat

Inilah yang membuat Agent SDK berbeda: Claude menjalankan alat secara langsung alih-alih meminta Anda untuk mengimplementasikannya.

<Note>
  Jika Anda melihat "API key not found", pastikan Anda telah menetapkan variabel lingkungan `ANTHROPIC_API_KEY` di shell tempat Anda menjalankan agen Anda. SDK tidak memuat file `.env` secara otomatis. Lihat [panduan pemecahan masalah lengkap](/docs/id/troubleshooting) untuk bantuan lebih lanjut.
</Note>

<h3 id="try-other-prompts">
  Coba prompt lain
</h3>

Sekarang agen Anda sudah diatur, coba beberapa prompt berbeda:

* `"Add docstrings to all functions in utils.py"`
* `"Add type hints to all functions in utils.py"`
* `"Create a README.md documenting the functions in utils.py"`

<h3 id="customize-your-agent">
  Sesuaikan agen Anda
</h3>

Anda dapat mengubah perilaku agen dengan mengubah opsi. Berikut adalah beberapa contoh:

**Tambahkan kemampuan pencarian web:**

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(
      allowed_tools=["Read", "Edit", "Glob", "WebSearch"], permission_mode="acceptEdits"
  )
  ```

  ```typescript TypeScript hidelines={1,-1} theme={null}
  const _ = {
    options: {
      allowedTools: ["Read", "Edit", "Glob", "WebSearch"],
      permissionMode: "acceptEdits"
    }
  };
  ```
</CodeGroup>

**Berikan Claude prompt sistem kustom:**

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(
      allowed_tools=["Read", "Edit", "Glob"],
      permission_mode="acceptEdits",
      system_prompt="You are a senior Python developer. Always follow PEP 8 style guidelines.",
  )
  ```

  ```typescript TypeScript hidelines={1,-1} theme={null}
  const _ = {
    options: {
      allowedTools: ["Read", "Edit", "Glob"],
      permissionMode: "acceptEdits",
      systemPrompt: "You are a senior Python developer. Always follow PEP 8 style guidelines."
    }
  };
  ```
</CodeGroup>

**Jalankan perintah di terminal:**

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(
      allowed_tools=["Read", "Edit", "Glob", "Bash"], permission_mode="acceptEdits"
  )
  ```

  ```typescript TypeScript hidelines={1,-1} theme={null}
  const _ = {
    options: {
      allowedTools: ["Read", "Edit", "Glob", "Bash"],
      permissionMode: "acceptEdits"
    }
  };
  ```
</CodeGroup>

Dengan `Bash` diaktifkan, coba: `"Write unit tests for utils.py, run them, and fix any failures"`

<h2 id="key-concepts">
  Konsep kunci
</h2>

**Tools** mengontrol apa yang dapat dilakukan agen Anda:

| Tools                                  | Apa yang dapat dilakukan agen |
| -------------------------------------- | ----------------------------- |
| `Read`, `Glob`, `Grep`                 | Analisis hanya-baca           |
| `Read`, `Edit`, `Glob`                 | Analisis dan modifikasi kode  |
| `Read`, `Edit`, `Bash`, `Glob`, `Grep` | Otomasi penuh                 |

**Permission modes** mengontrol berapa banyak pengawasan manusia yang Anda inginkan:

| Mode                | Perilaku                                                                                                                                                                                                                                                                                                 | Kasus penggunaan                                     |
| ------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------- |
| `acceptEdits`       | Auto-persetujuan pengeditan file dan perintah sistem file umum, meminta tindakan lain                                                                                                                                                                                                                    | Alur kerja pengembangan terpercaya                   |
| `plan`              | Menjalankan alat hanya-baca; pengeditan file tidak pernah auto-disetujui dan mencapai callback `canUseTool` Anda                                                                                                                                                                                         | Menentukan cakupan tugas sebelum menyetujui eksekusi |
| `dontAsk`           | Menolak apa pun yang tidak ada di `allowedTools`; connector tools [organisasi Anda atur ke `ask`](/docs/id/mcp#organization-controls-on-connector-tools) dan tools yang memerlukan interaksi pengguna ditolak bahkan jika Anda telah mencantumkannya                                                          | Agen headless terkunci                               |
| `auto`              | Pengklasifikasi model menyetujui atau menolak setiap panggilan alat                                                                                                                                                                                                                                      | Agen otonom dengan penjaga keamanan                  |
| `bypassPermissions` | Menjalankan setiap alat tanpa prompt, kecuali tools yang cocok dengan aturan [`ask`](/docs/id/agent-sdk/permissions#how-permissions-are-evaluated) eksplisit, connector tools [organisasi Anda atur ke `ask`](/docs/id/mcp#organization-controls-on-connector-tools), dan tools yang memerlukan interaksi pengguna | CI sandboxed, lingkungan yang sepenuhnya terpercaya  |
| `default`           | Memerlukan callback `canUseTool` untuk menangani persetujuan                                                                                                                                                                                                                                             | Alur persetujuan kustom                              |

Contoh di atas menggunakan mode `acceptEdits`, yang auto-persetujuan operasi file sehingga agen dapat berjalan tanpa prompt interaktif. Jika Anda ingin meminta pengguna untuk persetujuan, gunakan mode `default` dan sediakan callback [`canUseTool`](/docs/id/agent-sdk/user-input) yang mengumpulkan input pengguna. Untuk kontrol lebih lanjut, lihat [Permissions](/docs/id/agent-sdk/permissions).

<h2 id="next-steps">
  Langkah berikutnya
</h2>

Sekarang Anda telah membuat agen pertama Anda, pelajari cara memperluas kemampuannya dan menyesuaikannya dengan kasus penggunaan Anda:

* **[Permissions](/docs/id/agent-sdk/permissions)**: kontrol apa yang dapat dilakukan agen Anda dan kapan memerlukan persetujuan
* **[Hooks](/docs/id/agent-sdk/hooks)**: jalankan kode kustom sebelum atau sesudah panggilan alat
* **[Sessions](/docs/id/agent-sdk/sessions)**: bangun agen multi-turn yang mempertahankan konteks
* **[MCP servers](/docs/id/agent-sdk/mcp)**: terhubung ke database, browser, API, dan sistem eksternal lainnya
* **[Hosting](/docs/id/agent-sdk/hosting)**: sebarkan agen ke Docker, cloud, dan CI/CD
* **[Example agents](https://github.com/anthropics/claude-agent-sdk-demos)**: lihat contoh lengkap: asisten email, agen penelitian, dan lainnya
