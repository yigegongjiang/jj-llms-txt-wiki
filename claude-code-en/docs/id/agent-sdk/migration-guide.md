> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Migrasi ke Claude Agent SDK

> Panduan untuk migrasi Claude Code TypeScript dan Python SDKs ke Claude Agent SDK

<h2 id="overview">
  Ikhtisar
</h2>

Claude Code SDK telah diubah namanya menjadi **Claude Agent SDK** dan dokumentasinya telah diorganisir ulang. Perubahan ini mencerminkan kemampuan SDK yang lebih luas untuk membangun agen AI di luar sekadar tugas pengkodean.

Bermigrasi dari OpenAI Agents SDK? [Resep migrasi OpenAI Agents SDK](https://platform.claude.com/cookbook/claude-agent-sdk-04-migrating-from-openai-agents-sdk) memetakan setiap primitif ke Claude Agent SDK melalui satu contoh yang telah dikerjakan.

<h2 id="what’s-changed">
  Apa yang Berubah
</h2>

| Aspek                  | Lama                        | Baru                                                                             |
| :--------------------- | :-------------------------- | :------------------------------------------------------------------------------- |
| **Nama Paket (TS/JS)** | `@anthropic-ai/claude-code` | `@anthropic-ai/claude-agent-sdk`                                                 |
| **Paket Python**       | `claude-code-sdk`           | `claude-agent-sdk`                                                               |
| **Lokasi Dokumentasi** | Claude Code docs            | Claude Code docs → bagian [Agent SDK](/docs/id/agent-sdk/overview) yang didedikasikan |

<h2 id="migration-steps">
  Langkah-Langkah Migrasi
</h2>

<h3 id="for-typescript/javascript-projects">
  Untuk Proyek TypeScript/JavaScript
</h3>

**1. Uninstall paket lama:**

```bash theme={null}
npm uninstall @anthropic-ai/claude-code
```

**2. Install paket baru:**

```bash theme={null}
npm install @anthropic-ai/claude-agent-sdk
```

**3. Perbarui impor Anda:**

Ubah semua impor dari `@anthropic-ai/claude-code` ke `@anthropic-ai/claude-agent-sdk`:

```typescript theme={null}
// Sebelumnya
import { query, tool, createSdkMcpServer } from "@anthropic-ai/claude-code";

// Sesudahnya
import { query, tool, createSdkMcpServer } from "@anthropic-ai/claude-agent-sdk";
```

**4. Perbarui package.json:**

Jika `@anthropic-ai/claude-code` masih tercantum dalam `package.json` Anda, gantikan dengan `@anthropic-ai/claude-agent-sdk` dan perbarui juga rentang versinya, misalnya dari `"^0.0.42"` menjadi `"^0.3.0"`.

**5. Tinjau [perubahan yang merusak](#breaking-changes)**

Buat perubahan kode apa pun yang diperlukan untuk menyelesaikan migrasi.

<h3 id="for-python-projects">
  Untuk Proyek Python
</h3>

**1. Uninstall paket lama:**

```bash theme={null}
pip uninstall -y claude-code-sdk
```

Jika paket lama tidak terinstal, pip mencetak `WARNING: Skipping claude-code-sdk as it is not installed.` Itu adalah hal yang diharapkan dan Anda dapat melanjutkan ke langkah berikutnya.

**2. Install paket baru:**

```bash theme={null}
pip install claude-agent-sdk
```

Jika `claude-code-sdk` tercantum dalam `requirements.txt` atau `pyproject.toml` Anda, gantikan dengan `claude-agent-sdk`.

**3. Perbarui impor Anda:**

Ubah semua impor dari `claude_code_sdk` ke `claude_agent_sdk`:

```python theme={null}
# Sebelumnya
from claude_code_sdk import query, ClaudeCodeOptions

# Sesudahnya
from claude_agent_sdk import query, ClaudeAgentOptions
```

**4. Tinjau [perubahan yang merusak](#breaking-changes)**

Buat perubahan kode apa pun yang diperlukan untuk menyelesaikan migrasi.

<h2 id="breaking-changes">
  Perubahan yang merusak kompatibilitas
</h2>

<Warning>
  Untuk meningkatkan isolasi dan konfigurasi eksplisit, Claude Agent SDK v0.1.0 memperkenalkan perubahan yang merusak kompatibilitas bagi pengguna yang bermigrasi dari Claude Code SDK.
</Warning>

<h3 id="python-claudecodeoptions-renamed-to-claudeagentoptions">
  Python: ClaudeCodeOptions diganti nama menjadi ClaudeAgentOptions
</h3>

**Apa yang berubah:** Tipe SDK Python `ClaudeCodeOptions` telah diganti nama menjadi `ClaudeAgentOptions`.

**Migrasi:**

```python theme={null}
# SEBELUMNYA (claude-code-sdk)
from claude_code_sdk import query, ClaudeCodeOptions

options = ClaudeCodeOptions(model="claude-opus-4-7", permission_mode="acceptEdits")

# SESUDAHNYA (claude-agent-sdk)
from claude_agent_sdk import query, ClaudeAgentOptions

options = ClaudeAgentOptions(model="claude-opus-4-7", permission_mode="acceptEdits")
```

<h3 id="system-prompt-no-longer-default">
  System prompt tidak lagi default
</h3>

**Apa yang berubah:** SDK tidak lagi menggunakan system prompt Claude Code secara default.

**Migrasi:**

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // SEBELUMNYA (v0.0.x) - Menggunakan system prompt Claude Code secara default
  const before = query({ prompt: "Hello" });

  // SESUDAHNYA (v0.1.0) - Menggunakan system prompt minimal secara default
  // Untuk mendapatkan perilaku lama, secara eksplisit minta preset Claude Code:
  const presetResult = query({
    prompt: "Hello",
    options: {
      systemPrompt: { type: "preset", preset: "claude_code" }
    }
  });

  // Atau gunakan system prompt kustom:
  const customResult = query({
    prompt: "Hello",
    options: {
      systemPrompt: "You are a helpful coding assistant"
    }
  });
  ```

  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions
  import asyncio


  async def main():
      # SEBELUMNYA (v0.0.x) - Menggunakan system prompt Claude Code secara default
      async for message in query(prompt="Hello"):
          print(message)

      # SESUDAHNYA (v0.1.0) - Menggunakan system prompt minimal secara default
      # Untuk mendapatkan perilaku lama, secara eksplisit minta preset Claude Code:
      async for message in query(
          prompt="Hello",
          options=ClaudeAgentOptions(
              system_prompt={"type": "preset", "preset": "claude_code"}  # Gunakan preset
          ),
      ):
          print(message)

      # Atau gunakan system prompt kustom:
      async for message in query(
          prompt="Hello",
          options=ClaudeAgentOptions(system_prompt="You are a helpful coding assistant"),
      ):
          print(message)


  asyncio.run(main())
  ```
</CodeGroup>

<h3 id="settings-sources-default">
  Default sumber pengaturan
</h3>

Default ini secara singkat diubah di v0.1.0 untuk tidak memuat pengaturan filesystem dan kemudian dikembalikan, jadi tidak ada tindakan migrasi yang diperlukan.

**Perilaku saat ini:** Menghilangkan `settingSources` pada `query()` memuat pengaturan pengguna, proyek, dan filesystem lokal, sesuai dengan CLI. Ini mencakup `~/.claude/settings.json`, `.claude/settings.json`, `.claude/settings.local.json`, file CLAUDE.md, dan perintah kustom.

Untuk menjalankan terisolasi dari pengaturan filesystem, teruskan `settingSources: []`, atau `setting_sources=[]` di Python. Lihat [Control filesystem settings with settingSources](/docs/id/agent-sdk/claude-code-features#control-filesystem-settings-with-settingsources) untuk mengetahui apa yang dimuat setiap sumber.

Isolasi sangat penting untuk pipeline CI/CD, aplikasi yang diterapkan, lingkungan pengujian, dan sistem multi-tenant di mana kustomisasi lokal tidak boleh bocor.

<Note>
  Python SDK 0.1.59 dan lebih awal memperlakukan daftar kosong sama dengan menghilangkan opsi, jadi tingkatkan sebelum mengandalkan `setting_sources=[]`. Lihat [What settingSources does not control](/docs/id/agent-sdk/claude-code-features#what-settingsources-does-not-control) untuk input yang dibaca bahkan ketika `settingSources` adalah `[]`.
</Note>

<h2 id="next-steps">
  Langkah Berikutnya
</h2>

* Jelajahi [Ringkasan Agent SDK](/docs/id/agent-sdk/overview) untuk mempelajari fitur yang tersedia
* Lihat [Referensi SDK TypeScript](/docs/id/agent-sdk/typescript) untuk dokumentasi API terperinci
* Tinjau [Referensi SDK Python](/docs/id/agent-sdk/python) untuk dokumentasi khusus Python
* Pelajari tentang [Custom Tools](/docs/id/agent-sdk/custom-tools) dan [Integrasi MCP](/docs/id/agent-sdk/mcp)
