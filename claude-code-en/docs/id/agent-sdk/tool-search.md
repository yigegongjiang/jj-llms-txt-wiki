> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Skalakan ke banyak tools dengan pencarian tools

> Skalakan agen Anda ke ribuan tools dengan menemukan dan memuat hanya yang diperlukan, sesuai permintaan.

Pencarian tools memungkinkan agen Anda bekerja dengan ratusan atau ribuan tools dengan secara dinamis menemukan dan memuat mereka sesuai permintaan. Alih-alih memuat semua definisi tools ke dalam jendela konteks di awal, agen mencari katalog tools Anda dan memuat hanya tools yang dibutuhkannya.

Pendekatan ini menyelesaikan dua tantangan saat perpustakaan tools berkembang:

* **Efisiensi konteks:** Definisi tools dapat mengonsumsi porsi besar dari jendela konteks (50 tools dapat menggunakan 10-20K tokens), meninggalkan ruang lebih sedikit untuk pekerjaan sebenarnya.
* **Akurasi pemilihan tools:** Akurasi pemilihan tools menurun dengan lebih dari 30-50 tools yang dimuat sekaligus.

<h2 id="how-tool-search-works">
  Cara kerja pencarian tools
</h2>

Pencarian tools aktif secara default, dengan pengecualian yang tercantum dalam [Konfigurasi pencarian tools](#configure-tool-search).

Ketika aktif, definisi tools ditahan dari jendela konteks. Agen menerima ringkasan tools yang tersedia dan mencari yang relevan ketika tugas memerlukan kemampuan yang belum dimuat. Hingga lima tools paling relevan dimuat ke dalam konteks secara default, di mana mereka tetap tersedia untuk giliran berikutnya sampai SDK mengompres pesan tempat agen menemukan mereka. Setelah pemadatan itu, agen mencari tools tersebut lagi ketika mereka membutuhkannya berikutnya.

Pencarian tools menambahkan satu putaran ekstra setiap kali Claude mencari tools, tetapi untuk set tools besar ini diimbangi oleh konteks yang lebih kecil pada setiap giliran. Dengan lebih sedikit dari \~10 tools yang definisinya pas di jendela konteks, memuat semuanya di awal biasanya lebih cepat.

Untuk detail tentang mekanisme API yang mendasarinya, lihat [Pencarian tools dalam API](https://platform.claude.com/docs/id/agents-and-tools/tool-use/tool-search-tool).

<Note>
  Pencarian tools tidak didukung pada penyebaran Microsoft Foundry [yang dihosting di Azure](https://platform.claude.com/docs/en/build-with-claude/claude-in-microsoft-foundry#hosting-options), yang menolaknya di sisi server: SDK mendeteksi penolakan dan memuat definisi tools di awal untuk penyebaran itu. [`ENABLE_TOOL_SEARCH`](#configure-tool-search) tidak dapat mengesampingkan ini, karena penolakan berasal dari penyebaran itu sendiri.
</Note>

<h2 id="configure-tool-search">
  Konfigurasi pencarian tools
</h2>

Pencarian tools aktif secara default. Untuk model pada daftar model yang tidak didukung SDK, SDK memuat definisi tools di awal, dan tidak ada nilai `ENABLE_TOOL_SEARCH` yang mengganti itu. Di Google Cloud's Agent Platform, SDK memutuskan berdasarkan generasi model:

* **Claude Opus 4.5, Sonnet 4.5, Haiku 4.5, dan yang lebih baru**: pencarian tools aktif secara default.
* **Model Agent Platform sebelumnya**: SDK memuat definisi tools di awal, karena stack serving mereka menolak header beta yang diperlukan. `ENABLE_TOOL_SEARCH` tidak dapat mengganti ini.

Sebelum Claude Code v2.1.221, SDK menonaktifkan pencarian tools untuk semua model di Google Cloud's Agent Platform kecuali Anda menetapkan `ENABLE_TOOL_SEARCH`.

SDK juga menonaktifkan pencarian tools ketika `ANTHROPIC_BASE_URL` menunjuk ke host non-first-party, karena sebagian besar proxy tidak meneruskan blok `tool_reference`. Anda dapat mengganti default itu dengan variabel lingkungan `ENABLE_TOOL_SEARCH`:

| Nilai          | Perilaku                                                                                                                                                                                                                                                                                                                                                                                                                      |
| :------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| (tidak diatur) | Pencarian tools aktif. Definisi tools ditunda dan ditemukan sesuai permintaan. Kembali ke pemuatan di awal pada model Google Cloud's Agent Platform yang lebih awal dari generasi Claude 4.5, `ANTHROPIC_BASE_URL` non-first-party, atau deployment Microsoft Foundry yang dihosting di Azure.                                                                                                                                |
| `true`         | Pencarian tools selalu aktif, kecuali pada deployment Microsoft Foundry yang dihosting di Azure, di mana penolakan sisi server masih memaksa pemuatan di awal, dan pada model Google Cloud's Agent Platform yang lebih awal dari generasi Claude 4.5, di mana SDK terus memuat definisi tools di awal. SDK mengirimkan header beta melalui proxy, dan permintaan gagal pada proxy yang tidak mendukung blok `tool_reference`. |
| `auto`         | Menghitung token dalam definisi tools yang dapat ditunda pencarian tools dan membandingkan total terhadap jendela konteks model. Ketika total mencapai 10% dari jendela, pencarian tools diaktifkan. Di bawah itu, SDK memuat setiap definisi tools ke dalam konteks di awal.                                                                                                                                                 |
| `auto:N`       | Sama seperti `auto` dengan persentase kustom. `auto:5` diaktifkan ketika definisi tersebut mencapai 5% dari jendela konteks. Nilai lebih rendah diaktifkan lebih awal.                                                                                                                                                                                                                                                        |
| `false`        | Pencarian tools dimatikan. Semua definisi tools dimuat ke dalam konteks pada setiap giliran.                                                                                                                                                                                                                                                                                                                                  |

Pengaturan [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS`](/docs/id/env-vars) membuat pencarian tools tetap mati. Anda tidak dapat menggantinya dengan menetapkan `ENABLE_TOOL_SEARCH` sendiri. Organisasi Anda dapat membuat pencarian tools tetap aktif melalui [pengaturan terkelola](/docs/id/managed-settings), pada Claude Code v2.1.227 atau lebih baru. [Nonaktifkan kemampuan pra-rilis](/docs/id/llm-gateway-protocol#disable-pre-release-capabilities) mencakup di mana penggantian berlaku dan apa yang dihapus variabel.

Pencarian tools berlaku untuk semua tools terdaftar, baik berasal dari server MCP jarak jauh atau [server MCP SDK kustom](/docs/id/agent-sdk/custom-tools). Ketika Anda menggunakan `auto`, SDK menghitung setiap definisi yang dapat ditunda pencarian tools terhadap satu ambang batas gabungan: setiap tools MCP yang tidak ditandai [`alwaysLoad`](/docs/id/mcp#exempt-a-server-from-deferral), dari server apa pun, ditambah tools bawaan yang dimuat sesuai permintaan. SDK selalu memuat tools bawaan inti seperti Bash, Read, dan Edit di awal dan tidak menghitungnya terhadap ambang batas.

Atur nilai dalam opsi `env` pada `query()`. Dalam TypeScript, `env` menggantikan lingkungan subprocess, jadi sebarkan `...process.env` untuk menjaga variabel yang diwariskan. Dalam Python, `env` digabungkan di atas lingkungan yang diwariskan. Contoh ini terhubung ke server MCP jarak jauh yang mengekspos banyak tools, pra-menyetujui semuanya dengan wildcard, dan menggunakan `auto:5` sehingga pencarian tools diaktifkan ketika definisi yang dapat ditundanya mencapai 5% dari jendela konteks:

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
          ENABLE_TOOL_SEARCH: "auto:5" // Activate tool search when deferrable definitions reach 5% of context
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
              "ENABLE_TOOL_SEARCH": "auto:5"  # Activate tool search when deferrable definitions reach 5% of context
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
* **Dukungan model:** Claude Sonnet 4.5, Claude Haiku 4.5, Claude Opus 4.5, dan model yang lebih baru; lihat [kompatibilitas model dalam dokumentasi API](https://platform.claude.com/docs/id/agents-and-tools/tool-use/tool-search-tool#model-compatibility) untuk daftar terkini. Hal yang sama berlaku di Agent Platform Google Cloud.

<h2 id="related-documentation">
  Dokumentasi terkait
</h2>

* [Pencarian tools dalam API](https://platform.claude.com/docs/id/agents-and-tools/tool-use/tool-search-tool): Dokumentasi API lengkap untuk pencarian tools, termasuk implementasi kustom
* [Hubungkan server MCP](/docs/id/agent-sdk/mcp): Terhubung ke tools eksternal melalui server MCP
* [Tools kustom](/docs/id/agent-sdk/custom-tools): Bangun tools Anda sendiri dengan server MCP SDK
* [Referensi SDK TypeScript](/docs/id/agent-sdk/typescript): Referensi API lengkap
* [Referensi SDK Python](/docs/id/agent-sdk/python): Referensi API lengkap
