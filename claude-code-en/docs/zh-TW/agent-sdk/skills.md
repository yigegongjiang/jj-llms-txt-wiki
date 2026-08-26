> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# SDK 中的 Agent Skills

> 使用 Claude Agent SDK 中的 Agent Skills 擴展 Claude 的專門功能

<h2 id="overview">
  概述
</h2>

Agent Skills 使用專門功能擴展 Claude，Claude 會在相關時自動調用這些功能。Skills 被打包為 `SKILL.md` 文件，包含說明、描述和可選的支持資源。

有關 Skills 的全面信息，包括優勢、架構和編寫指南，請參閱 [Agent Skills 概述](https://platform.claude.com/docs/zh-TW/agents-and-tools/agent-skills/overview)。

<h2 id="how-skills-work-with-the-sdk">
  Skills 如何與 SDK 配合使用
</h2>

使用 Claude Agent SDK 時，Skills 的特點是：

1. **定義為文件系統工件**：在特定目錄（`.claude/skills/`）中創建為 `SKILL.md` 文件
2. **從文件系統加載**：Skills 從由 `settingSources`（TypeScript）或 `setting_sources`（Python）控制的文件系統位置加載
3. **自動發現**：加載文件系統設置後，在啟動時從用戶和項目目錄發現 Skill 元數據；觸發時加載完整內容
4. **由模型調用**：Claude 根據上下文自動選擇何時使用它們
5. **通過 `skills` 選項進行篩選**：發現的 Skills 默認啟用。傳遞 Skill 名稱列表、`"all"` 或 `[]` 來控制會話中可用的 Skills

與子代理不同（可以以編程方式定義），Skills 必須創建為文件系統工件。SDK 不提供用於註冊 Skills 的編程 API。

<Note>
  Skills 通過文件系統設置源發現。使用默認 `query()` 選項時，SDK 加載用戶和項目源，因此 `~/.claude/skills/`、`<cwd>/.claude/skills/` 和 `<cwd>` 的任何父目錄（直到存儲庫根目錄）中的 `.claude/skills/` 中的 Skills 可用。如果您明確設置 `settingSources`，請包含 `'user'` 或 `'project'` 以保持 Skill 發現，或使用 [`plugins` 選項](/docs/zh-TW/agent-sdk/plugins) 從特定路徑加載 Skills。
</Note>

<h2 id="using-skills-with-the-sdk">
  在 SDK 中使用 Skills
</h2>

在 `query()` 上設置 `skills` 選項以控制會話中可用的 Skills。省略時，發現的 Skills 啟用且 Skill 工具可用，與 CLI 行為匹配。傳遞 `"all"` 以啟用每個發現的 Skill，傳遞 Skill 名稱列表以僅啟用那些，或傳遞 `[]` 以禁用全部。設置 `skills` 時，SDK 會自動將 Skill 工具新增至 `allowedTools`。如果您也傳遞明確的 `tools` 列表，請在該列表中包含 `"Skill"`，以便 Claude 可以叫用 skills。

配置後，Claude 自動從檔案系統發現 Skills 並在與使用者請求相關時叫用它們。

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions


  async def main():
      options = ClaudeAgentOptions(
          cwd="/path/to/project",  # Project with .claude/skills/
          setting_sources=["user", "project"],  # Load Skills from filesystem
          skills="all",  # Enable every discovered Skill
          allowed_tools=["Read", "Write", "Bash"],
      )

      async for message in query(
          prompt="Help me process this PDF document", options=options
      ):
          print(message)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Help me process this PDF document",
    options: {
      cwd: "/path/to/project", // Project with .claude/skills/
      settingSources: ["user", "project"], // Load Skills from filesystem
      skills: "all", // Enable every discovered Skill
      allowedTools: ["Read", "Write", "Bash"]
    }
  })) {
    console.log(message);
  }
  ```
</CodeGroup>

要僅啟用特定 Skills，請傳遞它們的名稱。名稱與 `SKILL.md` 中的 `name` 欄位或 Skill 的目錄名稱匹配。對於外掛提供的 Skills，使用 `plugin:skill`。

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(skills=["pdf", "docx"])
  ```

  ```typescript TypeScript theme={null}
  const options = { skills: ["pdf", "docx"] };
  ```
</CodeGroup>

`skills` 選項是內容篩選器，不是沙箱。未列出的 Skills 對模型隱藏並被 Skill 工具拒絕，但它們的檔案仍在磁碟上，可透過 Read 和 Bash 存取。

<h2 id="skill-locations">
  Skill 位置
</h2>

Skills 根據您的 `settingSources`/`setting_sources` 配置從文件系統目錄加載：

* **項目 Skills**（`.claude/skills/`）：通過 git 與您的團隊共享 - 當 `setting_sources` 包含 `"project"` 時加載
* **用戶 Skills**（`~/.claude/skills/`）：跨所有項目的個人 Skills - 當 `setting_sources` 包含 `"user"` 時加載
* **Plugin Skills**：與已安裝的 Claude Code plugins 捆綁

<h2 id="creating-skills">
  創建 Skills
</h2>

Skills 定義為包含具有 YAML frontmatter 和 Markdown 內容的 `SKILL.md` 文件的目錄。`description` 字段確定 Claude 何時調用您的 Skill。

**示例目錄結構**：

```bash theme={null}
.claude/skills/processing-pdfs/
└── SKILL.md
```

有關創建 Skills 的完整指導，包括 SKILL.md 結構、多文件 Skills 和示例，請參閱：

* [Claude Code 中的 Agent Skills](/docs/zh-TW/skills)：包含示例的完整指南
* [Agent Skills 最佳實踐](https://platform.claude.com/docs/en/agents-and-tools/agent-skills/best-practices)：編寫指南和命名約定

<h2 id="tool-restrictions">
  工具限制
</h2>

<Note>
  SKILL.md 中的 `allowed-tools` frontmatter 欄位僅在直接使用 Claude Code CLI 時受支持。**通過 SDK 使用 Skills 時不適用**。

  使用 SDK 時，通過查詢配置中的主 `allowedTools` 選項控制工具存取。
</Note>

要在 SDK 應用程式中控制 Skills 的工具存取，使用 `allowedTools` 預先批准特定工具。沒有 `canUseTool` 回呼時，列表中沒有的任何內容都被拒絕：

<Note>
  假設第一個範例中的匯入陳述式在以下程式碼片段中。
</Note>

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(
      setting_sources=["user", "project"],  # Load Skills from filesystem
      skills="all",
      allowed_tools=["Read", "Grep", "Glob"],
  )

  async for message in query(prompt="Analyze the codebase structure", options=options):
      print(message)
  ```

  ```typescript TypeScript theme={null}
  for await (const message of query({
    prompt: "Analyze the codebase structure",
    options: {
      settingSources: ["user", "project"], // Load Skills from filesystem
      skills: "all",
      allowedTools: ["Read", "Grep", "Glob"],
      permissionMode: "dontAsk" // Deny anything not in allowedTools
    }
  })) {
    console.log(message);
  }
  ```
</CodeGroup>

<h2 id="discovering-available-skills">
  發現可用的 Skills
</h2>

要查看 SDK 應用程序中可用的 Skills，只需詢問 Claude：

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(
      setting_sources=["user", "project"],  # Load Skills from filesystem
      skills="all",
  )

  async for message in query(prompt="What Skills are available?", options=options):
      print(message)
  ```

  ```typescript TypeScript theme={null}
  for await (const message of query({
    prompt: "What Skills are available?",
    options: {
      settingSources: ["user", "project"], // Load Skills from filesystem
      skills: "all"
    }
  })) {
    console.log(message);
  }
  ```
</CodeGroup>

Claude 將根據您當前的工作目錄和已安裝的插件列出可用的 Skills。

<h2 id="testing-skills">
  測試 Skills
</h2>

通過提出與其描述相匹配的問題來測試 Skills：

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(
      cwd="/path/to/project",
      setting_sources=["user", "project"],  # Load Skills from filesystem
      skills="all",
      allowed_tools=["Read", "Bash"],
  )

  async for message in query(prompt="Extract text from invoice.pdf", options=options):
      print(message)
  ```

  ```typescript TypeScript theme={null}
  for await (const message of query({
    prompt: "Extract text from invoice.pdf",
    options: {
      cwd: "/path/to/project",
      settingSources: ["user", "project"], // Load Skills from filesystem
      skills: "all",
      allowedTools: ["Read", "Bash"]
    }
  })) {
    console.log(message);
  }
  ```
</CodeGroup>

如果描述與您的請求相匹配，Claude 會自動調用相關的 Skill。

<h2 id="troubleshooting">
  故障排除
</h2>

<h3 id="skills-not-found">
  找不到 Skills
</h3>

**檢查 settingSources 配置**：Skills 通過 `user` 和 `project` 設置源發現。如果您明確設置 `settingSources`/`setting_sources` 並省略這些源，Skills 不會加載：

<CodeGroup>
  ```python Python theme={null}
  # Skills not loaded: setting_sources excludes user and project
  options = ClaudeAgentOptions(setting_sources=[], skills="all")

  # Skills loaded: user and project sources included
  options = ClaudeAgentOptions(
      setting_sources=["user", "project"],
      skills="all",
  )
  ```

  ```typescript TypeScript theme={null}
  // Skills not loaded: settingSources excludes user and project
  const options = {
    settingSources: [],
    skills: "all"
  };

  // Skills loaded: user and project sources included
  const options = {
    settingSources: ["user", "project"],
    skills: "all"
  };
  ```
</CodeGroup>

有關 `settingSources`/`setting_sources` 的更多詳細信息，請參閱 [TypeScript SDK 參考](/docs/zh-TW/agent-sdk/typescript#settingsource) 或 [Python SDK 參考](/docs/zh-TW/agent-sdk/python#settingsource)。

**檢查工作目錄**：SDK 從 `cwd` 選項中的 `.claude/skills/` 以及每個父目錄直到存儲庫根目錄加載 Skills。確保 `cwd` 指向包含 `.claude/skills/` 的目錄或其下方，在同一存儲庫內：

<CodeGroup>
  ```python Python theme={null}
  # Ensure your cwd points to the directory containing .claude/skills/
  options = ClaudeAgentOptions(
      cwd="/path/to/project",  # .claude/skills/ here or in a parent directory
      setting_sources=["user", "project"],  # Loads skills from these sources
      skills="all",
  )
  ```

  ```typescript TypeScript theme={null}
  // Ensure your cwd points to the directory containing .claude/skills/
  const options = {
    cwd: "/path/to/project", // .claude/skills/ here or in a parent directory
    settingSources: ["user", "project"], // Loads skills from these sources
    skills: "all"
  };
  ```
</CodeGroup>

有關完整模式，請參閱上面的「在 SDK 中使用 Skills」部分。

**驗證文件系統位置**：

```bash theme={null}
# Check project Skills
ls .claude/skills/*/SKILL.md

# Check personal Skills
ls ~/.claude/skills/*/SKILL.md
```

<h3 id="skill-not-being-used">
  Skill 未被使用
</h3>

**檢查 `skills` 選項**：如果您傳遞了 `skills` 列表，確認 Skill 的名稱已包含。傳遞 `[]` 會禁用所有 Skills。

**檢查描述**：確保它具體且包含相關關鍵字。有關編寫有效描述的指導，請參閱 [Agent Skills 最佳實踐](https://platform.claude.com/docs/en/agents-and-tools/agent-skills/best-practices#writing-effective-descriptions)。

<h3 id="additional-troubleshooting">
  其他故障排除
</h3>

有關一般 Skills 故障排除（YAML 語法、調試等），請參閱 [Claude Code Skills 故障排除部分](/docs/zh-TW/skills#troubleshooting)。

<h2 id="related-documentation">
  相關文檔
</h2>

<h3 id="skills-guides">
  Skills 指南
</h3>

* [Claude Code 中的 Agent Skills](/docs/zh-TW/skills)：包含創建、示例和故障排除的完整 Skills 指南
* [Agent Skills 概述](https://platform.claude.com/docs/en/agents-and-tools/agent-skills/overview)：概念概述、優勢和架構
* [Agent Skills 最佳實踐](https://platform.claude.com/docs/en/agents-and-tools/agent-skills/best-practices)：有效 Skills 的編寫指南
* [Agent Skills Cookbook](https://platform.claude.com/cookbook/skills-notebooks-01-skills-introduction)：示例 Skills 和模板

<h3 id="sdk-resources">
  SDK 資源
</h3>

* [SDK 中的子代理](/docs/zh-TW/agent-sdk/subagents)：類似的基於文件系統的代理，具有編程選項
* [SDK 中的斜杠命令](/docs/zh-TW/agent-sdk/slash-commands)：用戶調用的命令
* [SDK 概述](/docs/zh-TW/agent-sdk/overview)：一般 SDK 概念
* [TypeScript SDK 參考](/docs/zh-TW/agent-sdk/typescript)：完整 API 文檔
* [Python SDK 參考](/docs/zh-TW/agent-sdk/python)：完整 API 文檔
