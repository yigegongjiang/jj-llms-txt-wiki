> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Skalakan ke banyak tools dengan pencarian tools

> Skalakan agen Anda ke ribuan tools dengan menemukan dan memuat hanya yang diperlukan, sesuai permintaan.

Pencarian tools memungkinkan agen Anda bekerja dengan ratusan atau ribuan tools dengan secara dinamis menemukan dan memuat mereka sesuai permintaan. Alih-alih memuat semua definisi tools ke dalam jendela konteks di awal, agen mencari katalog tools Anda dan memuat hanya tools yang dibutuhkannya.

Pendekatan ini menyelesaikan dua tantangan saat perpustakaan tools berkembang:

* **Efisiensi konteks:** Definisi tools dapat mengonsumsi porsi besar dari jendela konteks (50 tools dapat menggunakan 10-20K tokens), meninggalkan ruang lebih sedikit untuk pekerjaan sebenarnya.
* **Akurasi pemilihan tools:** Akurasi pemilihan tools menurun dengan lebih dari 30-50 tools yang dimuat sekaligus.

Pencarian tools diaktifkan secara default.

<h2 id="how-tool-search-works">
  Cara kerja pencarian tools
</h2>

Ketika pencarian tools aktif, definisi tools ditahan dari jendela konteks. Agen menerima ringkasan tools yang tersedia dan mencari yang relevan ketika tugas memerlukan kemampuan yang belum dimuat. Hingga lima tools paling relevan dimuat ke dalam konteks secara default, di mana mereka tetap tersedia untuk giliran berikutnya. Jika percakapan cukup panjang sehingga SDK mengompres pesan sebelumnya untuk membebaskan ruang, tools yang sebelumnya ditemukan mungkin dihapus, dan agen mencari lagi sesuai kebutuhan.

Pencarian tools menambahkan satu putaran ekstra pertama kali Claude menemukan tool (langkah pencarian), tetapi untuk set tools besar ini diimbangi oleh konteks yang lebih kecil pada setiap giliran. Dengan lebih sedikit dari \~10 tools, memuat semuanya di awal biasanya lebih cepat.

Untuk detail tentang mekanisme API yang mendasarinya, lihat [Pencarian tools dalam API](https://platform.claude.com/docs/id/agents-and-tools/tool-use/tool-search-tool).

<Note>
  Pencarian tools didukung pada Claude Sonnet 4.5, Claude Haiku 4.5, Claude Opus 4.5, dan model yang lebih baru; lihat [kompatibilitas model dalam dokumentasi API](https://platform.claude.com/docs/id/agents-and-tools/tool-use/tool-search-tool#model-compatibility) untuk daftar terkini. Di Agent Platform Google Cloud, model yang didukung minimum adalah Claude Sonnet 4.5 dan Claude Opus 4.5.
</Note>

<h2 id="configure-tool-search">
  Konfigurasi pencarian tools
</h2>

Pencarian tools aktif secara default. Ini dinonaktifkan secara default di Google Cloud's Agent Platform, di mana didukung untuk Claude Sonnet 4.5 dan lebih baru serta Claude Opus 4.5 dan lebih baru. Ini juga dinonaktifkan ketika `ANTHROPIC_BASE_URL` menunjuk ke host non-first-party, karena sebagian besar proxy tidak meneruskan blok `tool_reference`. Anda dapat mengganti salah satu default dengan variabel lingkungan `ENABLE_TOOL_SEARCH`:

| Nilai          | Perilaku                                                                                                                                                                                                                                                                             |
| :------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| (tidak diatur) | Pencarian tools aktif. Definisi tools ditunda dan ditemukan sesuai permintaan. Kembali ke pemuatan di awal di Google Cloud's Agent Platform atau `ANTHROPIC_BASE_URL` non-first-party.                                                                                               |
| `true`         | Pencarian tools selalu aktif. SDK mengirimkan header beta bahkan di Google Cloud's Agent Platform dan melalui proxy. Permintaan gagal pada model Google Cloud's Agent Platform lebih awal dari Sonnet 4.5 atau Opus 4.5, atau pada proxy yang tidak mendukung blok `tool_reference`. |
| `auto`         | Memeriksa jumlah token gabungan dari semua definisi tools terhadap jendela konteks model. Jika melebihi 10%, pencarian tools diaktifkan. Jika di bawah 10%, semua tools dimuat ke dalam konteks secara normal.                                                                       |
| `auto:N`       | Sama seperti `auto` dengan persentase kustom. `auto:5` diaktifkan ketika definisi tools melebihi 5% dari jendela konteks. Nilai lebih rendah diaktifkan lebih awal.                                                                                                                  |
| `false`        | Pencarian tools dimatikan. Semua definisi tools dimuat ke dalam konteks pada setiap giliran.                                                                                                                                                                                         |

Pengaturan [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS`](/docs/id/env-vars) membuat pencarian tools tetap mati, dan `ENABLE_TOOL_SEARCH` tidak dapat menggantinya. Variabel ini menghapus header beta yang diperlukan oleh definisi tools `defer_loading` dan blok konten `tool_reference`.

Pencarian tools berlaku untuk semua tools terdaftar, baik berasal dari server MCP jarak jauh atau [server MCP SDK kustom](/docs/id/agent-sdk/custom-tools). Saat menggunakan `auto`, ambang batas didasarkan pada ukuran gabungan semua definisi tools di semua server.

Atur nilai dalam opsi `env` pada `query()`. Dalam TypeScript, `env` menggantikan lingkungan subprocess, jadi sebarkan `...process.env` untuk menjaga variabel yang diwariskan. Dalam Python, `env` digabungkan di atas lingkungan yang diwariskan. Contoh ini terhubung ke server MCP jarak jauh yang mengekspos banyak tools, pra-menyetujui semuanya dengan wildcard, dan menggunakan `auto:5` sehingga pencarian tools diaktifkan ketika definisi mereka melebihi 5% dari jendela konteks:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  try {
    for await (const message of query({
      prompt: "Find and run the appropriate database query",
      options: {
        mcpServers: {
          "enterprise-tools": {
            // Connect to a remote MCP server
            type: "http",
            url: "https://tools.example.com/mcp"
          }
        },
        allowedTools: ["mcp__enterprise-tools__*"], // Wildcard pre-approves all tools from this server
        env: {
          ...process.env, // env replaces the subprocess environment, so keep inherited variables
          ENABLE_TOOL_SEARCH: "auto:5" // Activate tool search when tools exceed 5% of context
        }
      }
    })) {
      if (message.type === "result" && message.subtype === "success") {
        console.log(message.result);
      }
    }
  } catch (error) {
    // A single-shot query() throws after yielding an error result
    console.log(`Session ended with an error: ${error}`);
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage


  async def main():
      options = ClaudeAgentOptions(
          mcp_servers={
              "enterprise-tools": {
                  "type": "http",
                  "url": "https://tools.example.com/mcp",
              }
          },
          allowed_tools=[
              "mcp__enterprise-tools__*"
          ],  # Wildcard pre-approves all tools from this server
          env={
              "ENABLE_TOOL_SEARCH": "auto:5"  # Activate tool search when tools exceed 5% of context
          },
      )

      try:
          async for message in query(
              prompt="Find and run the appropriate database query",
              options=options,
          ):
              if isinstance(message, ResultMessage) and message.subtype == "success":
                  print(message.result)
      except Exception as error:
          # A single-shot query() raises after yielding an error result
          print(f"Session ended with an error: {error}")


  asyncio.run(main())
  ```
</CodeGroup>

Untuk menjalankan contoh ini, ganti `https://tools.example.com/mcp` dengan URL server MCP Anda sendiri. Jika berhasil, teks hasil akan dicetak ke konsol.

Karena ini adalah panggilan `query()` single-shot, SDK akan melempar setelah menghasilkan hasil kesalahan, jadi contoh membungkus loop dalam blok try. Untuk melihat mengapa jalankan gagal, periksa `subtype` pesan hasil, seperti `error_during_execution`, di dalam loop. Untuk informasi lebih lanjut tentang pesan hasil, lihat [Menangani hasil](/docs/id/agent-sdk/agent-loop#handle-the-result).

Mengatur `ENABLE_TOOL_SEARCH` ke `"false"` menonaktifkan pencarian tools dan memuat semua definisi tools ke dalam konteks pada setiap giliran. Ini menghilangkan putaran pencarian, yang dapat lebih cepat ketika set tools kecil (lebih sedikit dari \~10 tools) dan definisi cocok dengan nyaman di jendela konteks.

<h2 id="optimize-tool-discovery">
  Optimalkan penemuan tools
</h2>

Mekanisme pencarian mencocokkan kueri terhadap nama dan deskripsi tools. Nama seperti `search_slack_messages` muncul untuk berbagai permintaan daripada `query_slack`. Deskripsi dengan kata kunci spesifik ("Cari pesan Slack berdasarkan kata kunci, saluran, atau rentang tanggal") cocok dengan lebih banyak kueri daripada yang generik ("Kueri Slack").

Anda juga dapat menambahkan bagian prompt sistem yang mencantumkan kategori tools yang tersedia. Ini memberikan agen konteks tentang jenis tools apa yang tersedia untuk dicari. Teruskan teks melalui opsi `systemPrompt` di TypeScript atau `system_prompt` di Python, menggunakan preset `claude_code` dengan `append`, yang menambahkan teks Anda ke prompt preset daripada menggantinya:

<CodeGroup>
  ```typescript TypeScript theme={null}
  options: {
    systemPrompt: {
      type: "preset",
      preset: "claude_code",
      append: "You can search for tools to interact with Slack, GitHub, and Jira."
    }
  }
  ```

  ```python Python theme={null}
  options = ClaudeAgentOptions(
      system_prompt={
          "type": "preset",
          "preset": "claude_code",
          "append": "You can search for tools to interact with Slack, GitHub, and Jira.",
      }
  )
  ```
</CodeGroup>

Untuk rangkaian lengkap opsi prompt sistem, lihat [Memodifikasi prompt sistem](/docs/id/agent-sdk/modifying-system-prompts).

<h2 id="limits">
  Batas
</h2>

* **Tools maksimum:** 10.000 tools dalam katalog Anda
* **Hasil pencarian:** mengembalikan hingga lima tools paling relevan per pencarian secara default
* **Dukungan model:** Claude Sonnet 4.5, Claude Haiku 4.5, Claude Opus 4.5, dan model yang lebih baru; lihat [kompatibilitas model dalam dokumentasi API](https://platform.claude.com/docs/id/agents-and-tools/tool-use/tool-search-tool#model-compatibility) untuk daftar terkini. Di Agent Platform Google Cloud, Claude Sonnet 4.5 dan yang lebih baru serta Claude Opus 4.5 dan yang lebih baru.

<h2 id="related-documentation">
  Dokumentasi terkait
</h2>

* [Pencarian tools dalam API](https://platform.claude.com/docs/id/agents-and-tools/tool-use/tool-search-tool): Dokumentasi API lengkap untuk pencarian tools, termasuk implementasi kustom
* [Hubungkan server MCP](/docs/id/agent-sdk/mcp): Terhubung ke tools eksternal melalui server MCP
* [Tools kustom](/docs/id/agent-sdk/custom-tools): Bangun tools Anda sendiri dengan server MCP SDK
* [Referensi SDK TypeScript](/docs/id/agent-sdk/typescript): Referensi API lengkap
* [Referensi SDK Python](/docs/id/agent-sdk/python): Referensi API lengkap
