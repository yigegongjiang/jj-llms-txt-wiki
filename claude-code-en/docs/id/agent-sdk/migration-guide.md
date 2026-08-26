> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Migrasi ke Claude Agent SDK

> Panduan untuk migrasi Claude Code TypeScript dan Python SDKs ke Claude Agent SDK

<h2 id="overview">
  Ringkasan
</h2>

Claude Code SDK telah diubah namanya menjadi **Claude Agent SDK** dan dokumentasinya telah diorganisir ulang. Perubahan ini mencerminkan kemampuan SDK yang lebih luas untuk membangun agen AI di luar sekadar tugas pengkodean.

<h2 id="what’s-changed">
  Apa yang Berubah
</h2>

| Aspek                  | Lama                        | Baru                             |
| :--------------------- | :-------------------------- | :------------------------------- |
| **Nama Paket (TS/JS)** | `@anthropic-ai/claude-code` | `@anthropic-ai/claude-agent-sdk` |
| **Paket Python**       | `claude-code-sdk`           | `claude-agent-sdk`               |
| **Lokasi Dokumentasi** | Dokumentasi Claude Code     | API Guide → Bagian Agent SDK     |

<Note>
  **Perubahan Dokumentasi:** Dokumentasi Agent SDK telah dipindahkan dari dokumentasi Claude Code ke API Guide di bawah bagian [Agent SDK](/docs/id/agent-sdk/overview) yang didedikasikan. Dokumentasi Claude Code sekarang fokus pada alat CLI dan fitur otomasi.
</Note>

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

**4. Perbarui dependensi package.json:**

Jika Anda memiliki paket yang terdaftar di `package.json` Anda, perbarui:

Sebelumnya:

```json theme={null}
{
  "dependencies": {
    "@anthropic-ai/claude-code": "^0.0.42"
  }
}
```

Sesudahnya:

```json theme={null}
{
  "dependencies": {
    "@anthropic-ai/claude-agent-sdk": "^0.2.0"
  }
}
```

**5. Tinjau [perubahan yang merusak](#breaking-changes)**

Buat perubahan kode apa pun yang diperlukan untuk menyelesaikan migrasi.

<h3 id="for-python-projects">
  Untuk Proyek Python
</h3>

**1. Uninstall paket lama:**

```bash theme={null}
pip uninstall claude-code-sdk
```

**2. Install paket baru:**

```bash theme={null}
pip install claude-agent-sdk
```

**3. Perbarui impor Anda:**

Ubah semua impor dari `claude_code_sdk` ke `claude_agent_sdk`:

```python theme={null}
# Sebelumnya
from claude_code_sdk import query, ClaudeCodeOptions

# Sesudahnya
from claude_agent_sdk import query, ClaudeAgentOptions
```

**4. Perbarui nama tipe:**

Ubah `ClaudeCodeOptions` menjadi `ClaudeAgentOptions`:

```python theme={null}
# Sebelumnya
from claude_code_sdk import query, ClaudeCodeOptions

options = ClaudeCodeOptions(model="claude-opus-4-7")

# Sesudahnya
from claude_agent_sdk import query, ClaudeAgentOptions

options = ClaudeAgentOptions(model="claude-opus-4-7")
```

**5. Tinjau [perubahan yang merusak](#breaking-changes)**

Buat perubahan kode apa pun yang diperlukan untuk menyelesaikan migrasi.

<h2 id="breaking-changes">
  Perubahan yang merusak
</h2>

<Warning>
  Untuk meningkatkan isolasi dan konfigurasi eksplisit, Claude Agent SDK v0.1.0 memperkenalkan perubahan yang merusak bagi pengguna yang bermigrasi dari Claude Code SDK. Tinjau bagian ini dengan hati-hati sebelum bermigrasi.
</Warning>

<h3 id="python-claudecodeoptions-renamed-to-claudeagentoptions">
  Python: ClaudeCodeOptions diubah nama menjadi ClaudeAgentOptions
</h3>

**Apa yang berubah:** Tipe Python SDK `ClaudeCodeOptions` telah diubah nama menjadi `ClaudeAgentOptions`.

**Migrasi:**

```python theme={null}
# SEBELUMNYA (claude-code-sdk)
from claude_code_sdk import query, ClaudeCodeOptions

options = ClaudeCodeOptions(model="claude-opus-4-7", permission_mode="acceptEdits")

# SESUDAHNYA (claude-agent-sdk)
from claude_agent_sdk import query, ClaudeAgentOptions

options = ClaudeAgentOptions(model="claude-opus-4-7", permission_mode="acceptEdits")
```

**Mengapa ini berubah:** Nama tipe sekarang cocok dengan branding "Claude Agent SDK" dan memberikan konsistensi di seluruh konvensi penamaan SDK.

<h3 id="system-prompt-no-longer-default">
  Prompt sistem tidak lagi default
</h3>

**Apa yang berubah:** SDK tidak lagi menggunakan prompt sistem Claude Code secara default.

**Migrasi:**

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // SEBELUMNYA (v0.0.x) - Menggunakan prompt sistem Claude Code secara default
  const before = query({ prompt: "Hello" });

  // SESUDAHNYA (v0.1.0) - Menggunakan prompt sistem minimal secara default
  // Untuk mendapatkan perilaku lama, secara eksplisit minta preset Claude Code:
  const presetResult = query({
    prompt: "Hello",
    options: {
      systemPrompt: { type: "preset", preset: "claude_code" }
    }
  });

  // Atau gunakan prompt sistem kustom:
  const customResult = query({
    prompt: "Hello",
    options: {
      systemPrompt: "You are a helpful coding assistant"
    }
  });
  ```

  ```python Python theme={null}
  # SEBELUMNYA (v0.0.x) - Menggunakan prompt sistem Claude Code secara default
  async for message in query(prompt="Hello"):
      print(message)

  # SESUDAHNYA (v0.1.0) - Menggunakan prompt sistem minimal secara default
  # Untuk mendapatkan perilaku lama, secara eksplisit minta preset Claude Code:
  from claude_agent_sdk import query, ClaudeAgentOptions

  async for message in query(
      prompt="Hello",
      options=ClaudeAgentOptions(
          system_prompt={"type": "preset", "preset": "claude_code"}  # Gunakan preset
      ),
  ):
      print(message)

  # Atau gunakan prompt sistem kustom:
  async for message in query(
      prompt="Hello",
      options=ClaudeAgentOptions(system_prompt="You are a helpful coding assistant"),
  ):
      print(message)
  ```
</CodeGroup>

**Mengapa ini berubah:** Memberikan kontrol dan isolasi yang lebih baik untuk aplikasi SDK. Anda sekarang dapat membangun agen dengan perilaku kustom tanpa mewarisi instruksi yang berfokus pada CLI dari Claude Code.

<h3 id="settings-sources-default">
  Default sumber pengaturan
</h3>

Default ini secara singkat diubah di v0.1.0 dan kemudian dikembalikan, jadi tidak ada tindakan migrasi yang diperlukan.

**Perilaku saat ini:** Menghilangkan `settingSources` pada `query()` memuat pengaturan pengguna, proyek, dan sistem file lokal, cocok dengan CLI. Ini termasuk `~/.claude/settings.json`, `.claude/settings.json`, `.claude/settings.local.json`, file CLAUDE.md, dan perintah kustom.

Untuk menjalankan terisolasi dari pengaturan sistem file, teruskan array kosong:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const isolatedResult = query({
    prompt: "Hello",
    options: {
      settingSources: [] // Tidak ada pengaturan sistem file yang dimuat
    }
  });

  // Atau muat hanya sumber tertentu:
  const projectOnlyResult = query({
    prompt: "Hello",
    options: {
      settingSources: ["project"] // Hanya pengaturan proyek
    }
  });
  ```

  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions

  async for message in query(
      prompt="Hello",
      options=ClaudeAgentOptions(setting_sources=[]),  # Tidak ada pengaturan sistem file yang dimuat
  ):
      print(message)

  # Atau muat hanya sumber tertentu:
  async for message in query(
      prompt="Hello",
      options=ClaudeAgentOptions(
          setting_sources=["project"]  # Hanya pengaturan proyek
      ),
  ):
      print(message)
  ```
</CodeGroup>

Isolasi sangat penting untuk pipeline CI/CD, aplikasi yang diterapkan, lingkungan pengujian, dan sistem multi-tenant di mana kustomisasi lokal tidak boleh bocor.

<Note>
  SDK v0.1.0 secara singkat default ke tidak ada pengaturan yang dimuat; ini dikembalikan dalam rilis berikutnya. Python SDK 0.1.59 dan lebih awal memperlakukan daftar kosong sama dengan menghilangkan opsi, jadi upgrade sebelum mengandalkan `setting_sources=[]`. Lihat [Apa yang settingSources tidak kontrol](/docs/id/agent-sdk/claude-code-features#what-settingsources-does-not-control) untuk input yang dibaca bahkan ketika `settingSources` adalah `[]`.
</Note>

<h2 id="why-the-rename">
  Mengapa Pengubahan Nama?
</h2>

Claude Code SDK awalnya dirancang untuk tugas pengkodean, tetapi telah berkembang menjadi kerangka kerja yang kuat untuk membangun semua jenis agen AI. Nama baru "Claude Agent SDK" lebih mencerminkan kemampuannya:

* Membangun agen bisnis (asisten hukum, penasihat keuangan, dukungan pelanggan)
* Membuat agen pengkodean khusus (bot SRE, pengulas keamanan, agen tinjauan kode)
* Mengembangkan agen kustom untuk domain apa pun dengan penggunaan alat, integrasi MCP, dan banyak lagi

<h2 id="getting-help">
  Mendapatkan Bantuan
</h2>

Jika Anda mengalami masalah apa pun selama migrasi:

**Untuk TypeScript/JavaScript:**

1. Periksa bahwa semua impor diperbarui untuk menggunakan `@anthropic-ai/claude-agent-sdk`
2. Verifikasi bahwa package.json Anda memiliki nama paket baru
3. Jalankan `npm install` untuk memastikan dependensi diperbarui

**Untuk Python:**

1. Periksa bahwa semua impor diperbarui untuk menggunakan `claude_agent_sdk`
2. Verifikasi bahwa requirements.txt atau pyproject.toml Anda memiliki nama paket baru
3. Jalankan `pip install claude-agent-sdk` untuk memastikan paket terinstal

<h2 id="next-steps">
  Langkah Berikutnya
</h2>

* Jelajahi [Ringkasan Agent SDK](/docs/id/agent-sdk/overview) untuk mempelajari fitur yang tersedia
* Lihat [Referensi SDK TypeScript](/docs/id/agent-sdk/typescript) untuk dokumentasi API terperinci
* Tinjau [Referensi SDK Python](/docs/id/agent-sdk/python) untuk dokumentasi khusus Python
* Pelajari tentang [Custom Tools](/docs/id/agent-sdk/custom-tools) dan [Integrasi MCP](/docs/id/agent-sdk/mcp)
