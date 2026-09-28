> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 詞彙表

> Claude Code 術語定義。了解 agentic loop、compaction、CLAUDE.md、hooks、subagents、MCP 和其他核心概念的含義。

本詞彙表定義 Claude Code 術語。每個條目都連結到深入涵蓋該概念的頁面。對於 tokens、temperature 和 RAG 等模型級概念，請參閱[平台詞彙表](https://platform.claude.com/docs/zh-TW/about-claude/glossary)。對於 Claude Desktop 術語（例如 desktop extension、MCPB 和 DXT），請參閱 [Claude 說明中心](https://support.claude.com/)。

<h2 id="a">
  A
</h2>

<h3 id="agents-md">
  AGENTS.md
</h3>

您為 AI 編碼代理編寫的專案指示的 markdown 檔案。如果您的儲存庫有一個且沒有 [CLAUDE.md](#claude-md)，Claude 會將其讀取為您的專案指示，無需您新增第二個檔案。您可以在 `/config` 中變更**專案指示**設定，讓 Claude 同時讀取兩個檔案或僅讀取 `CLAUDE.md`。直接讀取 `AGENTS.md` 需要 Claude Code v2.1.277 或更新版本。在某些會話中 Claude [無法讀取 `AGENTS.md`](/docs/zh-TW/memory#when-agents-md-support-is-unavailable)，因此請改為 [從 `CLAUDE.md` 匯入它](/docs/zh-TW/memory#share-one-file-with-other-coding-tools)。

了解更多：[AGENTS.md](/docs/zh-TW/memory#agents-md)

<h3 id="agent-teams">
  Agent teams
</h3>

由團隊主導協調的多個獨立 Claude Code 會話，具有共享的任務列表和點對點訊息傳遞。與在單個會話內運行且僅向父級報告的 [subagents](#subagent) 不同，隊友各自擁有自己的上下文視窗，您可以直接與任何隊友互動。Agent teams 是實驗性的，預設為停用；請參閱 [Enable agent teams](/docs/zh-TW/agent-teams#enable-agent-teams)。

了解更多：[Run agent teams](/docs/zh-TW/agent-teams)

<h3 id="agentic-coding">
  Agentic coding
</h3>

一種工作流程，其中 AI 可以自主地讀取檔案、執行命令和進行更改，而您可以觀看、重定向或離開，與只能用文字回應的聊天助手相反，您必須自己應用這些文字。Claude Code 是 agentic 的，因為它具有讓它採取行動的 [tools](#tool)，而不僅僅是提供建議。

了解更多：[How Claude Code works](/docs/zh-TW/how-claude-code-works)

<h3 id="agentic-harness">
  Agentic harness
</h3>

將語言模型轉變為能力強大的編碼代理的工具、上下文管理和執行環境。Claude Code 是 harness；Claude 是其中的模型。Harness 提供檔案存取、shell 執行、權限控制、記憶體載入以及將動作鏈接在一起的迴圈。

了解更多：[How Claude Code works](/docs/zh-TW/how-claude-code-works)

<h3 id="agentic-loop">
  Agentic loop
</h3>

Claude 為每項任務執行的循環：收集上下文、採取行動、驗證結果並重複直到完成。每個工具使用都會返回資訊，為下一步提供資訊。您可以隨時中斷迴圈進行重定向。大多數擴展點，包括 [hooks](#hook)、[skills](#skill) 和 [MCP](#mcp-model-context-protocol)，都插入到此迴圈的特定階段。

了解更多：[How Claude Code works](/docs/zh-TW/how-claude-code-works#the-agentic-loop)

<h3 id="artifact">
  Artifact
</h3>

Claude Code 從您的會話發佈到 claude.ai 上私人 URL 的即時互動網頁，因此您可以視覺化查看輸出或與他人共享，而不是閱讀終端文字。當會話重新發佈時，頁面會就地更新。您從 Claude Code 建立的 Artifacts 會出現在與 claude.ai 對話中建立的 artifacts 相同的庫中。共享取決於您的方案：在 Pro 和 Max 上，任何人都可以開啟的公開連結；在 Team 和 Enterprise 上，在您的組織內共享，以及一旦擁有者啟用它們就可以公開連結。

了解更多：[Share session output as artifacts](/docs/zh-TW/artifacts)

<h3 id="auto-memory">
  Auto memory
</h3>

Claude 根據您的更正和偏好為自己編寫的筆記，按 git 儲存庫存儲在 `~/.claude/projects/` 下。同一儲存庫的所有 worktrees 共享一個 auto memory 目錄。`MEMORY.md` 索引的前 200 行或 25 KB 在每個會話開始時載入。Auto memory 是 Claude 編寫的對應物，與您編寫的 [CLAUDE.md](#claude-md) 相對。

了解更多：[Auto memory](/docs/zh-TW/memory#auto-memory)

<h3 id="auto-mode">
  Auto mode
</h3>

一種 [permission mode](#permission-mode)，其中單獨的分類器模型審查動作而不是您，因此 Claude Code 可以在不詢問您的情況下執行大多數動作。Claude Code 仍會在您的明確 ask 規則相符的動作之前詢問您。在 Pro、Max 和 Team 方案上，auto mode 是互動式終端和 VS Code 會話的 [built-in starting permission mode](/docs/zh-TW/permission-modes#which-mode-a-session-starts-in)。分類器會阻止範圍升級、不受信任的基礎設施和 [prompt injection](#prompt-injection)。工具結果會從它看到的內容中移除，因此檔案或網頁中的惡意內容無法直接操縱它。

了解更多：[Eliminate prompts with auto mode](/docs/zh-TW/permission-modes#eliminate-prompts-with-auto-mode)

<h2 id="b">
  B
</h2>

<h3 id="bare-mode">
  Bare mode
</h3>

使用 `--bare`，Claude Code 啟動時不會載入 hooks、skills、自訂命令、subagents、installed plugins、MCP servers、auto memory 或 CLAUDE.md，除了您使用 `--add-dir` 傳遞的目錄中的 skills。建議用於 CI 和指令碼呼叫，其中您需要在每台機器上獲得相同的結果。

了解更多：[使用 bare mode 更快啟動](/docs/zh-TW/headless#start-faster-with-bare-mode)

<h3 id="bundled-skills">
  Bundled skills
</h3>

Claude Code 附帶的基於提示的劇本，例如 `/batch`、`/code-review`、`/debug` 和 `/loop`。與執行固定邏輯的內建命令不同，bundled skills 為 Claude 提供詳細的提示並讓它協調工作，因此它們可以生成代理、讀取檔案並適應您的程式碼庫。

了解更多：[Bundled skills](/docs/zh-TW/skills#bundled-skills)

<h2 id="c">
  C
</h2>

<h3 id="channel">
  Channel
</h3>

一個[MCP 伺服器](#mcp-model-context-protocol)，可將事件推送到您執行中的工作階段，讓 Claude 能夠對您離開終端時發生的事情做出反應。Channel 可以是雙向的：Claude 讀取入站事件並透過同一 Channel 回覆。Telegram、Discord 和 iMessage 已包含在研究預覽中。

深入瞭解：[Channels](/docs/zh-TW/channels)

<h3 id="checkpoint">
  Checkpoint
</h3>

在您傳送開始一個回合的每個提示時建立的還原點。Claude Code 在每次編輯前都會快照檔案，以便 checkpoint 可以還原它們。按 `Esc` 兩次或執行 `/rewind` 以將程式碼、對話或兩者還原到較早的時間點，或從選定的訊息摘要對話的一部分。Checkpoint 會與對話一起儲存，因此已恢復的工作階段仍然可以 `/rewind` 回到它們。它們與 git 分開，不追蹤透過 Bash 工具所做的變更。

深入瞭解：[Checkpointing](/docs/zh-TW/checkpointing)

<h3 id="claude-directory">
  `.claude` 目錄
</h3>

Claude Code 讀取專案範圍設定的目錄：設定、hooks、skills、subagents、rules 和自動記憶。專案在其根目錄有 `.claude/`；您的使用者層級預設值在 `~/.claude/`。

深入瞭解：[The `.claude` directory](/docs/zh-TW/claude-directory)

<h3 id="claude-md">
  CLAUDE.md
</h3>

您為 Claude 撰寫的持久指示的 markdown 檔案，在每個工作階段開始時作為系統提示之後的使用者訊息載入。將專案慣例、架構筆記和「始終執行 X」規則放在此處。專案根目錄 CLAUDE.md 在[壓縮](#compaction)後保留，並在之後從磁碟重新讀取。

您可以在專案範圍的 `./CLAUDE.md` 或 `./.claude/CLAUDE.md`、使用者範圍的 `~/.claude/CLAUDE.md` 或作為組織的[受管原則](#managed-settings)放置 CLAUDE.md。所有發現的檔案都會連接到內容中，而不是相互覆蓋，順序從最廣泛的範圍到最具體的範圍。Claude Code 也可以載入專案的 [AGENTS.md](#agents-md) 檔案，單獨或與 CLAUDE.md 一起。

深入瞭解：[CLAUDE.md files](/docs/zh-TW/memory#claude-md-files)

<h3 id="cloud-session">
  Cloud session
</h3>

一個 Claude Code 工作階段，在您關閉筆記型電腦後仍繼續執行，因為它在雲端基礎設施上執行而不是在您的機器上：預設由 Anthropic 管理，或由您的組織運作的[自託管環境](/docs/zh-TW/self-hosted-environments)。您可以從 claude.ai/code、Claude 行動應用程式、選擇了**雲端**的 Desktop 應用程式、`claude --cloud` 或[例行工作](/docs/zh-TW/routines)啟動一個。在您的終端、IDE 或選擇了**本機**的 Desktop 應用程式中的工作階段是本機工作階段；若要從另一個裝置連接到本機工作階段，請使用[遠端控制](#remote-control)。

深入瞭解：[Use Claude Code in the cloud](/docs/zh-TW/claude-code-on-the-web)

<h3 id="command">
  Command
</h3>

您透過在提示中輸入 `/name` 來叫用的可重複使用指示。內建命令（例如 `/clear`、`/model` 和 `/compact`）控制工作階段。您可以在 `.claude/commands/` 中將自己的命令定義為檔案，或從[外掛程式](#plugin)安裝它們。[Skills](#skill) 是封裝多步驟命令的建議方式。

該詞的另外兩個用途不相關：`claude` CLI 子命令（例如 `claude mcp add`），列在 [CLI 參考](/docs/zh-TW/cli-reference#cli-commands) 中，以及 stdio [MCP 伺服器](#mcp-server)項目的 `command` 欄位，它指定 Claude Code 啟動伺服器時啟動的可執行檔。

深入瞭解：[Commands](/docs/zh-TW/commands) · [Skills](/docs/zh-TW/skills)

<h3 id="compaction">
  Compaction
</h3>

當[內容視窗](#context-window)接近其限制時，自動摘要您的對話。較舊的工具輸出會先清除，然後對話會被摘要。專案根目錄 CLAUDE.md 和自動記憶在壓縮後保留並從磁碟重新載入；僅在對話中給出的指示可能會遺失。執行 `/compact` 以手動觸發，可選擇使用焦點，例如 `/compact focus on the API changes`。

深入瞭解：[What survives compaction](/docs/zh-TW/context-window#what-survives-compaction) · [When context fills up](/docs/zh-TW/how-claude-code-works#when-context-fills-up)

<h3 id="connector">
  Connector
</h3>

添加到您的 claude.ai 帳戶而不是在 Claude Code 中設定的 [MCP 伺服器](#mcp-server)。當您使用該帳戶登入 Claude Code 時，您的連接器會在 `/mcp` 中與您在本地添加的伺服器一起出現。組織也可以佈建連接器並對其設定每個工具的控制。

深入瞭解：[Use MCP servers from claude.ai](/docs/zh-TW/mcp#use-mcp-servers-from-claude-ai)

<h3 id="context-window">
  Context window
</h3>

工作階段的工作記憶，保存對話歷史、檔案內容、命令輸出、CLAUDE.md、自動記憶、已載入的 skills 和系統指示。當您工作時，內容會填滿，直到[壓縮](#compaction)摘要它。執行 `/context` 以查看佔用空間的內容。如需基礎模型概念，請參閱[平台詞彙表](https://platform.claude.com/docs/en/about-claude/glossary#context-window)。

深入瞭解：[Explore the context window](/docs/zh-TW/context-window)

<h2 id="d">
  D
</h2>

<h3 id="dispatch">
  Dispatch
</h3>

一個電話啟動的任務路由器，當您從 Claude 行動應用程式發送編碼任務時，它會在 Desktop 應用程式中生成 Claude Code 會話。您的提示會自動路由到正確的工具。在 Pro 和 Max 計畫上可用。

了解更多：[Sessions from Dispatch](/docs/zh-TW/desktop#sessions-from-dispatch)

<h2 id="e">
  E
</h2>

<h3 id="effort-level">
  Effort level
</h3>

一個設定，控制自適應推理，讓模型決定是否以及在每個步驟上進行多少思考。更高的努力意味著更多的思考 tokens 和更深入的推理；更低的努力更快且更便宜。Effort 在 Fable 模型、Opus 4.6 及更新版本和 Sonnet 4.6 及更新版本上受支援。

了解更多：[Adjust effort level](/docs/zh-TW/model-config#adjust-effort-level)

<h3 id="extended-thinking">
  Extended thinking
</h3>

模型在回應前執行的可見逐步推理。您可以使用 [effort level](#effort-level) 調整它，或在具有固定思考預算的模型上使用 `MAX_THINKING_TOKENS` 限制思考 tokens。思考在終端中以灰色斜體文字顯示。

了解更多：[Use extended thinking](/docs/zh-TW/model-config#extended-thinking)

<h2 id="f">
  F
</h2>

<h3 id="frontmatter">
  Frontmatter
</h3>

位於 Markdown 檔案最頂端的 YAML 設定區塊，介於開頭的 `---` 行和結尾的 `---` 行之間。Skills、subagents、output styles 和 rules 各自從 frontmatter 讀取其設定，例如 skill 的 `description` 或 subagent 的 `tools`，並將結尾 `---` 之後的所有內容視為指示。開頭的 `---` 必須是檔案的第一行。每種檔案類型都接受其自己的一組欄位。

深入瞭解：[Skill frontmatter](/docs/zh-TW/skills#frontmatter-reference)、[Subagent frontmatter](/docs/zh-TW/sub-agents#supported-frontmatter-fields)、[Output style frontmatter](/docs/zh-TW/output-styles#frontmatter)、[Rule frontmatter](/docs/zh-TW/memory#rules-frontmatter-reference)

<h2 id="h">
  H
</h2>

<h3 id="hook">
  Hook
</h3>

一個使用者定義的處理程式，在 Claude Code 生命週期中的特定點自動執行，例如在工具執行前、檔案編輯後或會話開始時。處理程式可以是 shell 命令、HTTP 端點、MCP 工具、LLM 提示或 subagent。Hooks 是確定性的：它們在固定的生命週期點觸發，而不是由模型自行決定。

Hook 配置有三個級別：

* **Hook event**：生命週期點
* **Matcher**：篩選哪些事件觸發它
* **Hook handler**：執行什麼

了解更多：[Get started with hooks](/docs/zh-TW/hooks-guide) · [Hooks reference](/docs/zh-TW/hooks)

<h2 id="m">
  M
</h2>

<h3 id="managed-settings">
  Managed settings
</h3>

由 IT 或 DevOps 在組織範圍內強制執行的設定，透過管理員主控台從 Anthropic 的伺服器傳遞，或部署到 `~/.claude` 外的 OS 級路徑上的裝置。使用者和專案設定無法覆蓋受管設定。伺服器管理的傳遞適用於[符合條件的配置](/docs/zh-TW/server-managed-settings#platform-availability)；請參閱[安全考量](/docs/zh-TW/server-managed-settings#security-considerations)。使用此功能可實現安全策略、合規要求或整個機隊的標準化工具。

了解更多：[Server-managed settings](/docs/zh-TW/server-managed-settings) · [Settings files](/docs/zh-TW/settings#where-settings-live)

<h3 id="mcp-model-context-protocol">
  MCP (Model Context Protocol)
</h3>

一個開放標準，用於將 AI 工具連接到外部資料來源和服務。MCP servers 為 Claude 提供 Slack、Jira、資料庫、瀏覽器和數百個其他整合的新工具。您可以通過 `/mcp` 連接 servers 或將它們添加到 `.mcp.json`。有關協議本身，請參閱[平台詞彙表](https://platform.claude.com/docs/zh-TW/about-claude/glossary#mcp-model-context-protocol)。

了解更多：[Model Context Protocol](/docs/zh-TW/mcp)

<h3 id="mcp-server">
  MCP server
</h3>

一個程式，透過 [MCP](#mcp-model-context-protocol) 為 Claude 提供工具、提示或資源。您可以使用 `claude mcp add` 添加 servers、在 `.mcp.json` 中添加、透過[外掛程式](#plugin)或作為 claude.ai [連接器](#connector)。本地 stdio server 作為一個程序運行，Claude Code 從其設定的 `command` 和 `args` 欄位啟動，這與您在提示符處輸入的[命令](#command)無關。

了解更多：[Model Context Protocol](/docs/zh-TW/mcp)

<h3 id="mcp-tool-search">
  MCP Tool Search
</h3>

一個上下文節省機制，它延遲 MCP 工具架構直到需要時。只有工具名稱和伺服器指示在啟動時載入；Claude 在決定使用特定工具時按需擷取完整架構。這可以防止閒置的 MCP servers 消耗太多上下文。

了解更多：[Scale with MCP Tool Search](/docs/zh-TW/mcp#scale-with-mcp-tool-search)

<h2 id="n">
  N
</h2>

<h3 id="non-interactive-mode">
  Non-interactive mode
</h3>

一種執行單個提示並退出而不進行對話會話的模式，使用 `-p` 或 `--print` 調用。用於 CI、指令碼和管道。除非您傳遞 `--no-session-persistence`，否則執行仍會儲存為可恢復的會話。[Agent SDK](/docs/zh-TW/agent-sdk/overview) 是 Python 和 TypeScript 的等效項。以前稱為 headless mode。

了解更多：[Run Claude Code programmatically](/docs/zh-TW/headless)

<h2 id="o">
  O
</h2>

<h3 id="output-style">
  Output style
</h3>

一個設定，改變 Claude Code 提供給 Claude 的指示，以設定回應行為、語氣或格式。與 [CLAUDE.md](#claude-md) 不同，後者在 Claude Code 的預設指示旁邊新增專案內容，自訂輸出樣式可以取代預設軟體工程指示。

了解更多：[Output styles](/docs/zh-TW/output-styles)

<h2 id="p">
  P
</h2>

<h3 id="permission-mode">
  Permission mode
</h3>

會話的基線批准行為。在 CLI 中使用 `Shift+Tab` 循環或在 VS Code、Desktop 和 claude.ai 中使用模式選擇器。可用的模式是 `default`、`acceptEdits`、`plan`、`auto`、`dontAsk` 和 `bypassPermissions`。

`default` 模式在 CLI 和 VS Code 及 JetBrains 擴充功能中標記為 Manual，Claude Code 接受 `manual` 作為該值的別名。

了解更多：[選擇權限模式](/docs/zh-TW/permission-modes)

<h3 id="permission-rule">
  Permission rule
</h3>

一個設定條目，根據工具名稱和引數模式允許、詢問或拒絕工具調用。規則按 deny→ask→allow 順序評估，首先匹配獲勝。Permission rules 是分層在更廣泛的 [permission mode](#permission-mode) 之上的細粒度控制。

了解更多：[配置權限](/docs/zh-TW/permissions)

<h3 id="plan-mode">
  Plan mode
</h3>

一種 [permission mode](#permission-mode)，其中 Claude 研究並提議更改而不編輯您的原始檔案。它可以讀取、搜索和執行探索命令，然後在觸及任何內容之前提出批准計畫。使用 `/plan` 或按 `Shift+Tab` 進入 plan mode。

了解更多：[使用 plan mode 進行分析後再編輯](/docs/zh-TW/permission-modes#analyze-before-you-edit-with-plan-mode)

<h3 id="plugin">
  Plugin
</h3>

一個 skills、hooks、subagents 和 MCP servers 的捆綁包，打包為單個可安裝單元。Plugin skills 命名為 `plugin-name:skill-name`，以便多個 plugins 共存。通過 [marketplace](/docs/zh-TW/plugins/overview) 在團隊間分發 plugins。

了解更多：[Plugins](/docs/zh-TW/plugins/overview)

<h3 id="project-trust">
  Project trust
</h3>

一個對話框，在 Claude Code 載入其設定之前接受目錄。接受情況按專案目錄保存，除了您的主目錄，其中信任僅在目前工作階段內保持，並在每次啟動時重新出現提示。在您信任目錄之前，Claude Code 會暫不載入其儲存庫提供的某些內容，例如來自 `.claude/settings.json` 的專案允許規則和 marketplaces。[信任資料夾前執行的內容](/docs/zh-TW/permissions#what-runs-before-you-trust-a-folder)列出每種內容，包括 `-p` 工作階段在沒有對話框的情況下執行的內容。

了解更多：[The `.claude` directory](/docs/zh-TW/claude-directory)

<h3 id="prompt-injection">
  Prompt injection
</h3>

嵌入在檔案、網頁或工具結果中的敵對指令，試圖將 Claude 重定向到您從未要求的動作。Claude Code 的防禦包括權限系統、命令黑名單和信任驗證。[Auto mode](#auto-mode) 添加了一個伺服器端探針，掃描工具結果中的可疑內容，以及一個分類器，在去除工具結果後檢查動作，因此注入的文字無法直接操縱它。

了解更多：[防止 prompt injection](/docs/zh-TW/security#protect-against-prompt-injection)

<h2 id="r">
  R
</h2>

<h3 id="remote-control">
  Remote Control
</h3>

一種通過 claude.ai 從您的電話或瀏覽器繼續本地 Claude Code 會話的方式。您的程式碼執行和檔案保留在您的機器上；介面是遠端的。與[雲端會話](/docs/zh-TW/claude-code-on-the-web)不同，後者在雲沙箱中運行。

了解更多：[Remote Control](/docs/zh-TW/remote-control)

<h3 id="rules">
  Rules
</h3>

`.claude/rules/` 中的模組化指令檔案，與 CLAUDE.md 一起載入。規則可以使用 YAML `paths:` frontmatter 進行路徑範圍設定，因此它只在 Claude 讀取匹配檔案時載入，保持上下文精簡直到相關。

了解更多：[Organize rules with `.claude/rules/`](/docs/zh-TW/memory#organize-rules-with-claude/rules/)

<h2 id="s">
  S
</h2>

<h3 id="sandboxing">
  Sandboxing
</h3>

Bash 工具的 OS 級檔案系統和網路隔離。命令在您預先定義的邊界內執行，因此 Claude 可以在其中自由工作，無需每個命令的批准提示。Sandboxing 是與 [permission rules](#permission-rule) 分開的一層。

了解更多：[Sandboxing](/docs/zh-TW/sandboxing)

<h3 id="session">
  Session
</h3>

與您當前目錄相關的對話，具有自己的獨立 [context window](#context-window)。會話可以使用 `claude -c` 恢復、使用 `--fork-session` 分叉以在新會話 ID 下保留歷史記錄，或在終端間並行執行。執行 `/clear` 啟動新會話；前一個會話保持存儲並可通過 `/resume` 獲得。每個會話的記錄存儲在 `~/.claude/projects/` 下。

了解更多：[Work with sessions](/docs/zh-TW/how-claude-code-works#work-with-sessions)

<h3 id="settings-layers">
  Settings layers
</h3>

Claude Code 讀取設定的層級結構，按優先順序從最高到最低：[managed policy](#managed-settings)、命令列引數、`.claude/settings.local.json` 的本地設定、`.claude/settings.json` 的專案設定，然後是 `~/.claude/settings.json` 的使用者設定。陣列跨層級合併；較高層級的標量覆蓋較低層級的。請參閱 [Settings precedence](/docs/zh-TW/settings#settings-precedence)。

了解更多：[Settings files](/docs/zh-TW/settings#where-settings-live)

<h3 id="skill">
  Skill
</h3>

一個 `SKILL.md` 檔案，包含 Claude 添加到其工具包中的指令、知識或工作流程。Claude 在相關時自動載入 skill，或您可以使用 `/skill-name` 直接調用它。Skills 遵循 Agent Skills 開放標準；Claude Code 使用調用控制和 subagent 執行擴展它。

Skills 是自訂命令的推薦後繼者。`.claude/commands/deploy.md` 的檔案和 `.claude/skills/deploy/SKILL.md` 的檔案都會建立 `/deploy` 並以相同方式工作；現有命令檔案繼續工作。

了解更多：[Extend Claude with skills](/docs/zh-TW/skills)

<h3 id="subagent">
  Subagent
</h3>

一個專門的 AI 助手，在自己的上下文視窗中運行，具有自訂系統提示、特定工具存取和獨立權限。它處理委派的任務並向主對話返回摘要。使用 subagents 將大型探索保留在主上下文之外或執行並行研究。Subagent 保持在產生它的會話內。若要在您自己執行的不同會話之間傳遞發現，請使用 [cross-session messaging](/docs/zh-TW/cross-session-messaging)。

內建 subagents 包括 Explore、Plan 和通用目的。

了解更多：[Create custom subagents](/docs/zh-TW/sub-agents)

<h3 id="surface">
  Surface
</h3>

您存取 Claude Code 的任何地方：CLI、VS Code、JetBrains、Desktop 或 claude.ai。所有 surfaces 共享相同的引擎。您機器上的會話讀取您的本地 CLAUDE.md、設定和 skills；[cloud sessions](/docs/zh-TW/cloud-environments#what-carries-over-from-your-setup) 從您儲存庫的全新複製開始，不讀取您機器上的 `~/.claude/`。Slack 和 Chrome 擴展是連接到 surface 的整合，而不是 surfaces 本身。

了解更多：[Platforms and integrations](/docs/zh-TW/platforms)

<h2 id="t">
  T
</h2>

<h3 id="teleport">
  Teleport
</h3>

一個命令 `/teleport`，它將雲 Claude Code 會話拉入您的本地終端。Claude 獲取分支、載入對話歷史並從雲會話的最後狀態恢復。反向方向是 `--cloud`，它將本地任務發送到雲上執行。

了解更多：[從雲到終端](/docs/zh-TW/claude-code-on-the-web#from-cloud-to-terminal)

<h3 id="tool">
  Tool
</h3>

Claude 可以採取的動作：讀取檔案、編輯程式碼、執行 shell 命令、搜索 web、生成 subagent。Tools 是使 Claude Code 成為 agentic 的原因。沒有它們，Claude 只能用文字回應。每個工具使用都會返回一個結果，為 [agentic loop](#agentic-loop) 中 Claude 的下一個決定提供資訊。

了解更多：[Tools available to Claude](/docs/zh-TW/tools-reference)

<h3 id="turn">
  Turn
</h3>

Claude 在一個 [session](#session) 內的一個完整回應。一個 turn 開始於您發送訊息，結束於 Claude 完成回應，中間可能有任意數量的 [tool](#tool) 呼叫。[Stop hooks](#hook) 在每個 turn 結束時觸發。一個 session 由許多 turn 組成，[agentic loop](#agentic-loop) 描述了在一個 turn 內發生的情況。

了解更多：[How Claude Code works](/docs/zh-TW/how-claude-code-works#the-agentic-loop)

<h2 id="v">
  V
</h2>

<h3 id="verification-loop">
  Verification loop
</h3>

一個會話知道工作實際完成而不僅僅是看起來合理的方式。您給 Claude 一個它可以執行的檢查，例如測試套件、構建或螢幕截圖比較，Claude 迭代直到檢查通過，而不是在一次嘗試後停止。驗證迴圈是 [`/goal`](/docs/zh-TW/goal)、無人值守執行和 [dynamic workflows](/docs/zh-TW/workflows) 的先決條件：沒有它，唯一決定代理完成的事情就是代理本身。

了解更多：[Give Claude a way to verify its work](/docs/zh-TW/best-practices#give-claude-a-way-to-verify-its-work)

<h2 id="w">
  W
</h2>

<h3 id="worktree-isolation">
  Worktree isolation
</h3>

一個隔離模式，在 `.claude/worktrees/` 下的單獨 git worktree 中執行 Claude，使用 `-w` 標誌或 subagent 配置中的 `isolation: worktree` 啟用。更改保留在單獨分支的單獨目錄中，因此並行代理不會覆蓋彼此的檔案。

了解更多：[使用 git worktrees 執行並行會話](/docs/zh-TW/worktrees)

***

<h2 id="deprecated-and-renamed-terms">
  已棄用和重新命名的術語
</h2>

這些術語出現在較舊的文件、部落格文章和社群內容中。搜索本網站時使用當前名稱。

| 舊術語                                               | 現在稱為                                          | 備註                                                  |
| ------------------------------------------------- | --------------------------------------------- | --------------------------------------------------- |
| Headless mode                                     | [Non-interactive mode](#non-interactive-mode) | 相同的 `-p` 標誌，相同的行為                                   |
| Web session；「Claude Code on the web」作為任何雲端工作階段的名稱 | [Cloud session](#cloud-session)               | 「Claude Code on the web」現在僅命名 claude.ai/code 的瀏覽器介面 |
| Custom commands                                   | [Skills](#skill)                              | `.claude/commands/` 檔案仍然有效                          |
| Slash commands                                    | Commands                                      | 從產品副本中刪除了「Slash」                                    |
