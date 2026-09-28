> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Plugins dalam SDK

> Muat plugin kustom untuk memperluas Claude Code dengan skills, agen, hooks, dan server MCP melalui Agent SDK

Plugins memungkinkan Anda memperluas Claude Code dengan fungsionalitas kustom yang dapat dibagikan di seluruh proyek. Melalui Agent SDK, Anda dapat secara terprogram memuat plugins dari direktori lokal untuk menambahkan kemampuan ke sesi agen Anda. Sebuah plugin dapat mencakup:

* **Skills**: kemampuan yang Claude panggil secara otonom ketika relevan. Anda juga dapat menjalankan plugin skill secara langsung dengan `/plugin-name:skill-name`.
* **Agents**: subagen khusus untuk tugas-tugas tertentu
* **Hooks**: penanganan peristiwa yang merespons penggunaan alat dan peristiwa lainnya
* **MCP servers**: integrasi alat eksternal melalui Model Context Protocol

Untuk informasi lengkap tentang struktur plugin dan cara membuat plugins, lihat [Plugins](/docs/id/plugins/overview).

<h2 id="loading-plugins">
  Memuat plugins
</h2>

Muat plugins dengan menyediakan jalur sistem file lokal mereka dalam konfigurasi opsi Anda. Bidang `type` harus `"local"`, satu-satunya nilai yang diterima SDK. SDK mendukung pemuatan beberapa plugins dari lokasi berbeda.

Untuk menggunakan plugin yang didistribusikan melalui [marketplace](/docs/id/plugins/overview) atau repositori jarak jauh, unduh terlebih dahulu dan sediakan jalur direktori lokal. Untuk tata letak direktori yang dibutuhkan plugin, lihat [referensi struktur Plugin](#plugin-structure-reference) di bawah.

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Hello",
    options: {
      plugins: [
        { type: "local", path: "./my-plugin" },
        { type: "local", path: "/absolute/path/to/another-plugin" }
      ]
    }
  })) {
    // Plugin commands, agents, and other features are now available
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions


  async def main():
      async for message in query(
          prompt="Hello",
          options=ClaudeAgentOptions(
              plugins=[
                  {"type": "local", "path": "./my-plugin"},
                  {"type": "local", "path": "/absolute/path/to/another-plugin"},
              ]
          ),
      ):
          # Plugin commands, agents, and other features are now available
          pass


  asyncio.run(main())
  ```
</CodeGroup>

<h3 id="path-specifications">
  Spesifikasi jalur
</h3>

Jalur plugin dapat berupa:

* **Jalur relatif**: diselesaikan relatif terhadap opsi `cwd` (misalnya, `"./plugins/my-plugin"`)
* **Jalur absolut**: jalur sistem file lengkap (misalnya, `"/home/user/plugins/my-plugin"`)

<Note>
  Jalur harus menunjuk ke direktori root plugin: induk dari `skills/`, `agents/`, `hooks/`, `commands/`, atau `.claude-plugin/`.
</Note>

<h2 id="verifying-plugin-installation">
  Memverifikasi instalasi plugin
</h2>

Ketika plugins dimuat dengan berhasil, mereka muncul dalam pesan inisialisasi sistem. Anda dapat memverifikasi bahwa plugins Anda tersedia:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Hello",
    options: {
      plugins: [{ type: "local", path: "./my-plugin" }]
    }
  })) {
    if (message.type === "system" && message.subtype === "init") {
      // Check loaded plugins
      console.log("Plugins:", message.plugins);
      // Example: [{ name: "my-plugin", path: "/absolute/path/to/my-plugin" }]

      // Plugin skills appear with the plugin name as a prefix
      console.log("Skills:", message.skills);
      // Example: ["my-plugin:greet"]

      // Plugin commands use the same prefix, and skills appear here too
      console.log("Commands:", message.slash_commands);
      // Example: ["compact", "context", "my-plugin:custom-command", "my-plugin:greet"]
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, SystemMessage


  async def main():
      async for message in query(
          prompt="Hello",
          options=ClaudeAgentOptions(
              plugins=[{"type": "local", "path": "./my-plugin"}]
          ),
      ):
          if isinstance(message, SystemMessage) and message.subtype == "init":
              # Check loaded plugins
              print("Plugins:", message.data.get("plugins"))
              # Example: [{"name": "my-plugin", "path": "/absolute/path/to/my-plugin"}]

              # Plugin skills appear with the plugin name as a prefix
              print("Skills:", message.data.get("skills"))
              # Example: ["my-plugin:greet"]

              # Plugin commands use the same prefix, and skills appear here too
              print("Commands:", message.data.get("slash_commands"))
              # Example: ["compact", "context", "my-plugin:custom-command", "my-plugin:greet"]


  asyncio.run(main())
  ```
</CodeGroup>

<h2 id="use-plugin-skills">
  Menggunakan plugin skills
</h2>

Skills dari plugins secara otomatis diberi namespace dengan nama plugin untuk menghindari konflik. Untuk menjalankan satu secara langsung, kirimkan `/plugin-name:skill-name` sebagai prompt.

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // Load a plugin with a custom /greet skill
  for await (const message of query({
    prompt: "/my-plugin:greet", // Use plugin skill with namespace
    options: {
      plugins: [{ type: "local", path: "./my-plugin" }]
    }
  })) {
    // Claude executes the custom greeting skill from the plugin
    if (message.type === "assistant") {
      console.log(message.message.content);
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, AssistantMessage, TextBlock


  async def main():
      # Load a plugin with a custom /greet skill
      async for message in query(
          prompt="/my-plugin:greet",  # Use plugin skill with namespace
          options=ClaudeAgentOptions(
              plugins=[{"type": "local", "path": "./my-plugin"}]
          ),
      ):
          # Claude executes the custom greeting skill from the plugin
          if isinstance(message, AssistantMessage):
              for block in message.content:
                  if isinstance(block, TextBlock):
                      print(f"Claude: {block.text}")


  asyncio.run(main())
  ```
</CodeGroup>

<Note>
  Jika Anda menginstal plugin melalui CLI (misalnya, `/plugin install my-plugin@marketplace`), Anda masih dapat menggunakannya di SDK dengan menyediakan jalur instalasinya. Periksa `~/.claude/plugins/` untuk plugins yang diinstal CLI.
</Note>

<h2 id="complete-example">
  Contoh lengkap
</h2>

Berikut adalah contoh lengkap yang mendemonstrasikan pemuatan dan penggunaan plugin:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";
  import { fileURLToPath } from "node:url";

  async function runWithPlugin() {
    const pluginPath = fileURLToPath(new URL("./plugins/my-plugin", import.meta.url));

    console.log("Loading plugin from:", pluginPath);

    for await (const message of query({
      prompt: "What custom commands do you have available?",
      options: {
        plugins: [{ type: "local", path: pluginPath }],
        maxTurns: 3
      }
    })) {
      if (message.type === "system" && message.subtype === "init") {
        console.log("Loaded plugins:", message.plugins);
        console.log("Available skills:", message.skills);
        console.log("Available commands:", message.slash_commands);
      }

      if (message.type === "assistant") {
        console.log("Assistant:", message.message.content);
      }
    }
  }

  runWithPlugin().catch(console.error);
  ```

  ```python Python theme={null}
  #!/usr/bin/env python3
  """Example demonstrating how to use plugins with the Agent SDK."""

  import asyncio
  from pathlib import Path

  from claude_agent_sdk import (
      AssistantMessage,
      ClaudeAgentOptions,
      SystemMessage,
      TextBlock,
      query,
  )


  async def run_with_plugin():
      """Example using a custom plugin."""
      plugin_path = Path(__file__).parent / "plugins" / "my-plugin"

      print(f"Loading plugin from: {plugin_path}")

      options = ClaudeAgentOptions(
          plugins=[{"type": "local", "path": str(plugin_path)}],
          max_turns=3,
      )

      async for message in query(
          prompt="What custom commands do you have available?", options=options
      ):
          if isinstance(message, SystemMessage) and message.subtype == "init":
              print(f"Loaded plugins: {message.data.get('plugins')}")
              print(f"Available skills: {message.data.get('skills')}")
              print(f"Available commands: {message.data.get('slash_commands')}")

          if isinstance(message, AssistantMessage):
              for block in message.content:
                  if isinstance(block, TextBlock):
                      print(f"Assistant: {block.text}")


  if __name__ == "__main__":
      asyncio.run(run_with_plugin())
  ```
</CodeGroup>

<h2 id="plugin-structure-reference">
  Referensi struktur plugin
</h2>

Direktori plugin biasanya berisi file manifest `.claude-plugin/plugin.json`. Manifest bersifat opsional. Ketika dihilangkan, Claude Code secara otomatis menemukan komponen dari tata letak direktori. Direktori dapat mencakup:

```text theme={null}
my-plugin/
├── .claude-plugin/
│   └── plugin.json          # Plugin manifest (opsional, komponen ditemukan secara otomatis tanpanya)
├── skills/                   # Agent Skills (dipanggil secara otonom atau melalui /plugin-name:skill-name)
│   └── my-skill/
│       └── SKILL.md
├── commands/                 # Skills sebagai file .md datar
│   └── custom-cmd.md
├── agents/                   # Custom agents
│   └── specialist.md
├── hooks/                    # Event handlers
│   └── hooks.json
└── .mcp.json                # Definisi server MCP
```

<Note>
  Direktori `commands/` menyimpan skills sebagai file Markdown datar. Gunakan `skills/` untuk plugin baru. Claude Code mendukung kedua lokasi.
</Note>

<h2 id="multiple-plugin-sources">
  Sumber plugin ganda
</h2>

Gabungkan plugins dari lokasi berbeda:

```typescript theme={null}
import * as os from "node:os";
import * as path from "node:path";

plugins: [
  { type: "local", path: "./local-plugin" },
  {
    type: "local",
    path: path.join(os.homedir(), ".claude", "custom-plugins", "shared-plugin")
  }
];
```

<Note>
  SDK tidak memperluas jalur tilde seperti `~/plugins`. Jika jalur plugin tidak ada, SDK melewati plugin tersebut dan sesi berlanjut, jadi periksa daftar `plugins` dalam pesan init untuk mengonfirmasi setiap plugin dimuat.
</Note>

<h2 id="troubleshooting">
  Troubleshooting
</h2>

<h3 id="plugin-not-loading">
  Plugin tidak dimuat
</h3>

Jika plugin Anda tidak muncul dalam pesan init:

1. **Periksa jalurnya**: pastikan jalur menunjuk ke direktori root plugin, induk dari `skills/`, `agents/`, `hooks/`, `commands/`, atau `.claude-plugin/`
2. **Validasi plugin.json**: jika plugin Anda menyertakan manifest, pastikan memiliki sintaks JSON yang valid
3. **Periksa izin file**: pastikan direktori plugin dapat dibaca
4. **Konfirmasi direktori ada**: SDK melewati jalur yang tidak ada, dan plugin tidak muncul dalam daftar `plugins` pesan init

<h3 id="skills-not-appearing">
  Skills tidak muncul
</h3>

Jika plugin skills tidak berfungsi:

1. **Gunakan namespace**: panggil plugin skills sebagai `/plugin-name:skill-name`
2. **Periksa pesan init**: verifikasi bahwa skill muncul di daftar `skills` dengan namespace yang benar
3. **Validasi file skill**: pastikan setiap skill memiliki file `SKILL.md` di subdirektorinya sendiri di bawah `skills/`, misalnya `skills/my-skill/SKILL.md`

<h2 id="see-also">
  Lihat juga
</h2>

* [Plugins](/docs/id/plugins/overview) - Panduan pengembangan plugin lengkap
* [Plugins reference](/docs/id/plugins/manifest-reference) - Spesifikasi teknis
* [Commands](/docs/id/agent-sdk/skills#dispatch-commands-by-name) - Mengirimkan commands di SDK
* [Subagents](/docs/id/agent-sdk/subagents) - Bekerja dengan agen khusus
* [Skills](/docs/id/agent-sdk/skills) - Menggunakan Agent Skills
