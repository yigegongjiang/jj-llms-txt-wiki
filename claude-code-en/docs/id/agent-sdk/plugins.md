> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Plugins dalam SDK

> Muat plugin kustom untuk memperluas Claude Code dengan skills, agen, hooks, dan server MCP melalui Agent SDK

Plugins memungkinkan Anda memperluas Claude Code dengan fungsionalitas kustom yang dapat dibagikan di seluruh proyek. Melalui Agent SDK, Anda dapat secara terprogram memuat plugins dari direktori lokal untuk menambahkan skills, agen, hooks, dan server MCP ke sesi agen Anda.

<h2 id="what-are-plugins">
  Apa itu plugins?
</h2>

Plugins adalah paket ekstensi Claude Code yang dapat mencakup:

* **Skills**: Kemampuan yang dipanggil model yang digunakan Claude secara otonom (juga dapat dipanggil dengan `/skill-name`)
* **Agents**: Subagen khusus untuk tugas-tugas tertentu
* **Hooks**: Penanganan peristiwa yang merespons penggunaan alat dan peristiwa lainnya
* **MCP servers**: Integrasi alat eksternal melalui Model Context Protocol

<Note>
  Direktori `commands/` adalah format warisan. Gunakan `skills/` untuk plugin baru. Claude Code terus mendukung kedua format untuk kompatibilitas mundur.
</Note>

Untuk informasi lengkap tentang struktur plugin dan cara membuat plugins, lihat [Plugins](/docs/id/plugins).

<h2 id="loading-plugins">
  Memuat plugins
</h2>

Muat plugins dengan menyediakan jalur sistem file lokal mereka dalam konfigurasi opsi Anda. Bidang `type` harus `"local"`, satu-satunya nilai yang diterima SDK. Untuk menggunakan plugin yang didistribusikan melalui [marketplace](/docs/id/plugin-marketplaces) atau repositori jarak jauh, unduh terlebih dahulu dan sediakan jalur direktori lokal. SDK mendukung pemuatan beberapa plugins dari lokasi berbeda.

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

* **Jalur relatif**: Diselesaikan relatif terhadap direktori kerja saat ini (misalnya, `"./plugins/my-plugin"`)
* **Jalur absolut**: Jalur sistem file lengkap (misalnya, `"/home/user/plugins/my-plugin"`)

<Note>
  Jalur harus menunjuk ke direktori root plugin: induk dari `skills/`, `agents/`, `hooks/`, `commands/` (legacy), atau `.claude-plugin/`, bukan subdirektori.
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
      // Example: [{ name: "my-plugin", path: "./my-plugin" }]

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
              # Example: [{"name": "my-plugin", "path": "./my-plugin"}]

              # Plugin skills appear with the plugin name as a prefix
              print("Skills:", message.data.get("skills"))
              # Example: ["my-plugin:greet"]

              # Plugin commands use the same prefix, and skills appear here too
              print("Commands:", message.data.get("slash_commands"))
              # Example: ["compact", "context", "my-plugin:custom-command", "my-plugin:greet"]


  asyncio.run(main())
  ```
</CodeGroup>

<h2 id="using-plugin-skills">
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
          prompt="/demo-plugin:greet",  # Use plugin skill with namespace
          options=ClaudeAgentOptions(
              plugins=[{"type": "local", "path": "./plugins/demo-plugin"}]
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
  import * as path from "path";

  async function runWithPlugin() {
    const pluginPath = path.join(__dirname, "plugins", "my-plugin");

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

  from pathlib import Path
  import anyio
  from claude_agent_sdk import (
      AssistantMessage,
      ClaudeAgentOptions,
      SystemMessage,
      TextBlock,
      query,
  )


  async def run_with_plugin():
      """Example using a custom plugin."""
      plugin_path = Path(__file__).parent / "plugins" / "demo-plugin"

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
      anyio.run(run_with_plugin)
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
├── skills/                   # Agent Skills (dipanggil secara otonom atau melalui /skill-name)
│   └── my-skill/
│       └── SKILL.md
├── commands/                 # Legacy: gunakan skills/ sebagai gantinya
│   └── custom-cmd.md
├── agents/                   # Custom agents
│   └── specialist.md
├── hooks/                    # Event handlers
│   └── hooks.json
└── .mcp.json                # Definisi server MCP
```

Untuk informasi terperinci tentang membuat plugins, lihat:

* [Plugins](/docs/id/plugins) - Panduan pengembangan plugin lengkap
* [Plugins reference](/docs/id/plugins-reference) - Spesifikasi teknis dan skema

<h2 id="common-use-cases">
  Kasus penggunaan umum
</h2>

<h3 id="development-and-testing">
  Pengembangan dan pengujian
</h3>

Muat plugins selama pengembangan tanpa menginstalnya secara global:

```typescript theme={null}
plugins: [{ type: "local", path: "./dev-plugins/my-plugin" }];
```

<h3 id="project-specific-extensions">
  Ekstensi khusus proyek
</h3>

Sertakan plugins di repositori proyek Anda untuk konsistensi di seluruh tim:

```typescript theme={null}
plugins: [{ type: "local", path: "./project-plugins/team-workflows" }];
```

<h3 id="multiple-plugin-sources">
  Sumber plugin ganda
</h3>

Gabungkan plugins dari lokasi berbeda:

```typescript theme={null}
plugins: [
  { type: "local", path: "./local-plugin" },
  { type: "local", path: "~/.claude/custom-plugins/shared-plugin" }
];
```

<h2 id="troubleshooting">
  Troubleshooting
</h2>

<h3 id="plugin-not-loading">
  Plugin tidak dimuat
</h3>

Jika plugin Anda tidak muncul dalam pesan init:

1. **Periksa jalurnya**: pastikan jalur menunjuk ke direktori root plugin, induk dari `skills/`, `agents/`, `hooks/`, `commands/` (legacy), atau `.claude-plugin/`
2. **Validasi plugin.json**: jika plugin Anda menyertakan manifest, pastikan memiliki sintaks JSON yang valid
3. **Periksa izin file**: pastikan direktori plugin dapat dibaca

<h3 id="skills-not-appearing">
  Skills tidak muncul
</h3>

Jika plugin skills tidak berfungsi:

1. **Gunakan namespace**: panggil plugin skills sebagai `/plugin-name:skill-name`
2. **Periksa pesan init**: verifikasi bahwa skill muncul di daftar `skills` dengan namespace yang benar
3. **Validasi file skill**: pastikan setiap skill memiliki file `SKILL.md` di subdirektorinya sendiri di bawah `skills/`, misalnya `skills/my-skill/SKILL.md`

<h3 id="path-resolution-issues">
  Masalah resolusi jalur
</h3>

Jika jalur relatif tidak berfungsi:

1. **Periksa direktori kerja**: jalur relatif diselesaikan dari direktori kerja saat ini Anda
2. **Gunakan jalur absolut**: untuk keandalan, pertimbangkan menggunakan jalur absolut
3. **Normalkan jalur**: gunakan utilitas jalur untuk membuat jalur dengan benar

<h2 id="see-also">
  Lihat juga
</h2>

* [Plugins](/docs/id/plugins) - Panduan pengembangan plugin lengkap
* [Plugins reference](/docs/id/plugins-reference) - Spesifikasi teknis
* [Commands](/docs/id/agent-sdk/slash-commands) - Menggunakan commands di SDK
* [Subagents](/docs/id/agent-sdk/subagents) - Bekerja dengan agen khusus
* [Skills](/docs/id/agent-sdk/skills) - Menggunakan Agent Skills
