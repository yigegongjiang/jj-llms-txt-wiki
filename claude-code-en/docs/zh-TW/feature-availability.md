> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 功能可用性

> 比較 Claude Code 功能在 Anthropic 訂閱計畫、Anthropic Console、Amazon Bedrock、AWS 上的 Claude Platform、Google Cloud 的 Agent Platform 和 Microsoft Foundry 中的可用性。

Claude Code CLI 和所有在本地執行的功能在每個提供者上的運作方式完全相同。如需每個提供者的設定說明，請參閱[企業部署概述](/docs/zh-TW/third-party-integrations)。若要直接跳到您的提供者上缺少的功能，請參閱[按提供者摘要](#summary-by-provider)標籤。

在下表中，✓ 表示可用，✗ 表示不可用，「請參閱備註」連結到部分支援的註腳。✓ 後面的限定詞會將可用性縮小到該子集，「管理員啟用」表示該功能處於關閉狀態，直到組織管理員將其開啟。

<h2 id="availability-by-model-provider">
  按模型提供者的可用性
</h2>

您的驗證方式決定了 Claude Code 可以存取哪些功能。如需您的提供者上缺少的單一清單，請參閱[按提供者摘要](#summary-by-provider)標籤。若要在表格中找到您的欄位：

* **Claude 訂閱**：您使用 claude.ai 帳戶登入 Pro、Max、Team 或 Enterprise 計畫
* **Anthropic Console**：您使用 Anthropic API 金鑰進行驗證
* **Amazon Bedrock**：您使用 Amazon Bedrock 模型目錄中的 Claude 模型並設定 `CLAUDE_CODE_USE_BEDROCK`。[Mantle 端點](/docs/zh-TW/amazon-bedrock#use-the-mantle-endpoint)（`CLAUDE_CODE_USE_MANTLE`）由此欄涵蓋
* **AWS 上的 Claude Platform**：您透過 AWS Marketplace 購買了 Claude，但呼叫 Anthropic API，並設定 `CLAUDE_CODE_USE_ANTHROPIC_AWS`
* **Google Cloud's Agent Platform**：由 Google 營運；您設定 `CLAUDE_CODE_USE_VERTEX`
* **Microsoft Foundry**：由 Anthropic 在 Azure 上營運；您設定 `CLAUDE_CODE_USE_FOUNDRY`

<h3 id="features-available-on-every-provider">
  每個提供者上都可用的功能
</h3>

這些在每個提供者上的運作方式完全相同：

* [CLI](/docs/zh-TW/quickstart) 和 [Agent SDK](/docs/zh-TW/agent-sdk/overview)
* [VS Code](/docs/zh-TW/vs-code) 和 [JetBrains](/docs/zh-TW/jetbrains) 擴充功能
* [Subagents](/docs/zh-TW/sub-agents)、[hooks](/docs/zh-TW/hooks-guide)、[commands](/docs/zh-TW/commands) 和 [skills](/docs/zh-TW/skills)
* [CLAUDE.md 記憶](/docs/zh-TW/memory)、[plugins](/docs/zh-TW/plugins) 和 [MCP servers](/docs/zh-TW/mcp)
* [Checkpoints](/docs/zh-TW/checkpointing)、[sandboxing](/docs/zh-TW/sandboxing) 和 [Workflows](/docs/zh-TW/workflows)
* [OpenTelemetry 指標](/docs/zh-TW/monitoring-usage)和[受管設定檔](/docs/zh-TW/settings#settings-files)

這三個功能有提供者特定的差異：

* **MCP servers**：[來自 claude.ai 的連接器](/docs/zh-TW/mcp#use-mcp-servers-from-claude-ai)僅在您的 claude.ai 訂閱是作用中驗證方法時才會載入，而[工具搜尋](/docs/zh-TW/mcp#configure-tool-search)在 Google Cloud's Agent Platform 上預設為關閉，當 `ANTHROPIC_BASE_URL` 指向非第一方主機時也是如此
* **Subagents**：內建的 [Explore subagent](/docs/zh-TW/sub-agents#built-in-subagents) 在 Claude API 上將其繼承的模型上限設為 Opus，在任何其他提供者（包括 AWS 上的 Claude Platform）上直接繼承主要對話的模型
* **[Commands](/docs/zh-TW/commands#all-commands)**：`/design-sync` 和 `/radio` 在 Amazon Bedrock、Google Cloud's Agent Platform、Microsoft Foundry 和 AWS 上的 Claude Platform 上不可用，而 `/voice` 需要 claude.ai 帳戶

<h3 id="features-that-require-a-claude-subscription">
  需要 Claude 訂閱的功能
</h3>

這些需要使用 claude.ai 帳戶登入，無法透過 Anthropic Console API 金鑰或第三方提供者存取：

* [網頁上的 Claude Code](/docs/zh-TW/claude-code-on-the-web)、行動裝置上的 Claude Code 和 [Slack 中的 Claude Code](/docs/zh-TW/slack)
* [Claude Code Desktop](/docs/zh-TW/desktop)
* [Routines](/docs/zh-TW/routines)（`/schedule`）
* [Ultraplan](/docs/zh-TW/ultraplan) 和 [Ultrareview](/docs/zh-TW/ultrareview)
* [Code Review](/docs/zh-TW/code-review)：Team 和 Enterprise 計畫
* [Remote Control](/docs/zh-TW/remote-control)
* [Chrome 擴充功能](/docs/zh-TW/chrome)
* [Computer use](/docs/zh-TW/computer-use)：Pro 和 Max 計畫
* [Artifacts](/docs/zh-TW/artifacts)：Pro、Max、Team 和 Enterprise 計畫
* [Voice dictation](/docs/zh-TW/voice-dictation)

Desktop 是部分例外：[閘道路由可以在應用程式中或由管理員配置](/docs/zh-TW/llm-gateway-connect#desktop-app)、Enterprise 部署可以透過[受管設定](https://claude.com/docs/third-party/claude-desktop/configuration)將 Desktop 路由到 Google Cloud's Agent Platform 或閘道提供者，而[在 3P 上的 Claude Desktop](https://claude.com/docs/third-party/claude-desktop/overview)在 Amazon Bedrock、Google Cloud's Agent Platform、Microsoft Foundry 或自託管 LLM 閘道上執行 Code 標籤。如需這些功能的按計畫可用性，請參閱[按訂閱計畫的可用性](#availability-by-subscription-plan)。

<h3 id="cli-capabilities-that-vary-by-provider">
  按提供者變化的 CLI 功能
</h3>

這些功能在本地 CLI 中運作，但取決於並非每個提供者都公開的伺服器端功能。

<table>
  <thead>
    <tr>
      <th>功能</th>
      <th>Claude 訂閱</th>
      <th>Anthropic Console</th>
      <th>Amazon Bedrock</th>
      <th>AWS 上的 Claude Platform</th>
      <th>Google Cloud's Agent Platform</th>
      <th>Microsoft Foundry</th>
    </tr>
  </thead>

  <tbody>
    <tr>
      <td>[Web search](/docs/zh-TW/tools-reference#websearch-tool-behavior)</td>
      <td>✓</td>
      <td>✓</td>
      <td>✗</td>
      <td>✓</td>
      <td>請參閱備註 <sup><a href="#fn1">1</a></sup></td>
      <td>✓</td>
    </tr>

    <tr>
      <td>[Fast mode](/docs/zh-TW/fast-mode)</td>
      <td>✓</td>
      <td>✓</td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
    </tr>

    <tr>
      <td>[Auto mode](/docs/zh-TW/auto-mode-config)</td>
      <td>✓</td>
      <td>✓</td>
      <td>請參閱備註 <sup><a href="#fn2">2</a></sup></td>
      <td>✓</td>
      <td>請參閱備註 <sup><a href="#fn2">2</a></sup></td>
      <td>請參閱備註 <sup><a href="#fn2">2</a></sup></td>
    </tr>

    <tr>
      <td>[Advisor](/docs/zh-TW/advisor)</td>
      <td>✓</td>
      <td>✓</td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
    </tr>

    <tr>
      <td>[Channels](/docs/zh-TW/channels)</td>
      <td>✓</td>
      <td>✓</td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
    </tr>

    <tr>
      <td>[`/loop` 排程任務](/docs/zh-TW/scheduled-tasks)</td>
      <td>✓</td>
      <td>✓</td>
      <td>請參閱備註 <sup><a href="#fn3">3</a></sup></td>
      <td>請參閱備註 <sup><a href="#fn3">3</a></sup></td>
      <td>請參閱備註 <sup><a href="#fn3">3</a></sup></td>
      <td>請參閱備註 <sup><a href="#fn3">3</a></sup></td>
    </tr>

    <tr>
      <td>[GitHub Actions](/docs/zh-TW/github-actions) 和 [GitLab CI/CD](/docs/zh-TW/gitlab-ci-cd)</td>
      <td>✓</td>
      <td>✓</td>
      <td>✓</td>
      <td>✓</td>
      <td>✓</td>
      <td>✗</td>
    </tr>
  </tbody>
</table>

<h3 id="admin-and-analytics">
  管理和分析
</h3>

組織級控制和使用情況可見性。

<table>
  <thead>
    <tr>
      <th>功能</th>
      <th>Claude 訂閱</th>
      <th>Anthropic Console</th>
      <th>Amazon Bedrock</th>
      <th>AWS 上的 Claude Platform</th>
      <th>Google Cloud's Agent Platform</th>
      <th>Microsoft Foundry</th>
    </tr>
  </thead>

  <tbody>
    <tr>
      <td>[分析儀表板和 API](/docs/zh-TW/analytics)</td>
      <td>✓ (儀表板：Team 和 Enterprise；API：Enterprise)</td>
      <td>✓ <sup><a href="#fn5">5</a></sup></td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
    </tr>

    <tr>
      <td>[伺服器管理的設定](/docs/zh-TW/server-managed-settings)</td>
      <td>✓ (Team 和 Enterprise)</td>
      <td>✓ (Team 和 Enterprise)</td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
    </tr>

    <tr>
      <td>[Zero Data Retention](/docs/zh-TW/zero-data-retention)</td>
      <td>✓ (符合條件的 Enterprise 帳戶)</td>
      <td>✓ (符合條件的帳戶)</td>
      <td>請參閱備註 <sup><a href="#fn4">4</a></sup></td>
      <td>✓ (符合條件的帳戶)</td>
      <td>請參閱備註 <sup><a href="#fn4">4</a></sup></td>
      <td>請參閱備註 <sup><a href="#fn4">4</a></sup></td>
    </tr>
  </tbody>
</table>

<span id="fn1" style={{display: 'block', position: 'relative', top: '-120px'}} /><sup>1</sup> 在 Google Cloud's Agent Platform 上，web search 適用於 Claude 4 模型及更新版本。<br />
<span id="fn2" style={{display: 'block', position: 'relative', top: '-120px'}} /><sup>2</sup> 在這些提供者上，auto mode 僅支援 Claude Sonnet 5、Opus 4.7 和 Opus 4.8。請參閱 [Auto mode 配置](/docs/zh-TW/auto-mode-config)。在 v2.1.158 到 v2.1.206 中，這些提供者上的 auto mode 也需要設定 `CLAUDE_CODE_ENABLE_AUTO_MODE=1`；v2.1.207 移除了此要求。<br />
<span id="fn3" style={{display: 'block', position: 'relative', top: '-120px'}} /><sup>3</sup> 明確的間隔（例如 `/loop every 2 hours`）在每個提供者上都有效。在 Amazon Bedrock、AWS 上的 Claude Platform、Google Cloud's Agent Platform 和 Microsoft Foundry 上，`/loop` 無法選擇自己的間隔或提供預設維護提示，因此沒有間隔的提示每 10 分鐘執行一次，沒有引數的 `/loop` 顯示使用訊息。請參閱[排程任務](/docs/zh-TW/scheduled-tasks)。<br />
<span id="fn4" style={{display: 'block', position: 'relative', top: '-120px'}} /><sup>4</sup> 受您與雲端提供者的協議約束。<br />
<span id="fn5" style={{display: 'block', position: 'relative', top: '-120px'}} /><sup>5</sup> 僅限儀表板和 API。[貢獻指標](/docs/zh-TW/analytics#enable-contribution-metrics)需要 claude.ai Team 或 Enterprise 組織。

<Note>
  如果您透過 [LLM 閘道](/docs/zh-TW/llm-gateway)進行驗證，功能可用性與閘道轉發到的基礎提供者相符。某些僅限 Anthropic 的功能（例如 [Advisor](/docs/zh-TW/advisor)）只有在閘道將請求完整轉發到 Anthropic API 時才能運作。
</Note>

<h3 id="summary-by-provider">
  按提供者摘要
</h3>

每個標籤列出該提供者上不可用或部分支援的功能，以及存在替代方案的地方。未列出的所有功能在 Claude 訂閱上的運作方式相同，除了上述[每個提供者上都可用的功能](#features-available-on-every-provider)中提到的提供者特定差異。在 Amazon Bedrock、Google Cloud's Agent Platform、Microsoft Foundry 和 AWS 上的 Claude Platform 上，向 Anthropic 的錯誤報告和遙測預設為關閉。請參閱[按 API 提供者的預設行為](/docs/zh-TW/data-usage#default-behaviors-by-api-provider)，了解哪些流量仍會到達 Anthropic 以及如何選擇退出。

<Tabs>
  <Tab title="Amazon Bedrock">
    **不可用：** 所有[需要 Claude 訂閱的功能](#features-that-require-a-claude-subscription)，加上 [web search](/docs/zh-TW/tools-reference#websearch-tool-behavior)、[fast mode](/docs/zh-TW/fast-mode)、[Advisor](/docs/zh-TW/advisor)、[Channels](/docs/zh-TW/channels)、[分析儀表板](/docs/zh-TW/analytics)、[伺服器管理的設定](/docs/zh-TW/server-managed-settings) 和 [`/design-sync` 和 `/radio` 命令](/docs/zh-TW/commands#all-commands)。

    **部分支援：**

    * [Desktop](/docs/zh-TW/desktop)：僅透過[在 3P 上的 Claude Desktop](https://claude.com/docs/third-party/claude-desktop/overview)
    * [Auto mode](/docs/zh-TW/auto-mode-config)：Sonnet 5、Opus 4.7 和 Opus 4.8 僅限
    * [`/loop`](/docs/zh-TW/scheduled-tasks)：僅限明確間隔
    * [Zero Data Retention](/docs/zh-TW/zero-data-retention)：受您的 AWS 協議約束

    **替代方案：** 對於排程，使用具有明確間隔的 [`/loop`](/docs/zh-TW/scheduled-tasks) 而不是 `/schedule`。對於雲端工作階段，使用 [GitHub Actions](/docs/zh-TW/github-actions) 或 [GitLab CI/CD](/docs/zh-TW/gitlab-ci-cd)。對於網頁查詢，使用 [WebFetch 工具](/docs/zh-TW/tools-reference#webfetch-tool-behavior)搭配特定 URL。
  </Tab>

  <Tab title="AWS 上的 Claude Platform">
    **不可用：** 所有[需要 Claude 訂閱的功能](#features-that-require-a-claude-subscription)，加上 [fast mode](/docs/zh-TW/fast-mode)、[Advisor](/docs/zh-TW/advisor)、[Channels](/docs/zh-TW/channels)、[分析儀表板](/docs/zh-TW/analytics)、[伺服器管理的設定](/docs/zh-TW/server-managed-settings) 和 [`/design-sync` 和 `/radio` 命令](/docs/zh-TW/commands#all-commands)。

    **在 Amazon Bedrock 不可用的地方可用：** [web search](/docs/zh-TW/tools-reference#websearch-tool-behavior)。

    **部分支援：**

    * [`/loop`](/docs/zh-TW/scheduled-tasks)：僅限明確間隔

    **替代方案：** 對於排程，使用具有明確間隔的 [`/loop`](/docs/zh-TW/scheduled-tasks) 而不是 `/schedule`。對於雲端工作階段，使用 [GitHub Actions](/docs/zh-TW/github-actions) 或 [GitLab CI/CD](/docs/zh-TW/gitlab-ci-cd)。
  </Tab>

  <Tab title="Google Cloud's Agent Platform">
    **不可用：** 所有[需要 Claude 訂閱的功能](#features-that-require-a-claude-subscription)，加上 [fast mode](/docs/zh-TW/fast-mode)、[Advisor](/docs/zh-TW/advisor)、[Channels](/docs/zh-TW/channels)、[分析儀表板](/docs/zh-TW/analytics)、[伺服器管理的設定](/docs/zh-TW/server-managed-settings) 和 [`/design-sync` 和 `/radio` 命令](/docs/zh-TW/commands#all-commands)。

    **部分支援：**

    * [Desktop](/docs/zh-TW/desktop)：透過[受管設定](https://claude.com/docs/third-party/claude-desktop/configuration)或 [Claude Desktop on 3P](https://claude.com/docs/third-party/claude-desktop/overview)
    * [Web search](/docs/zh-TW/tools-reference#websearch-tool-behavior)：Claude 4 模型及更新版本
    * [Auto mode](/docs/zh-TW/auto-mode-config)：Sonnet 5、Opus 4.7 和 Opus 4.8 僅限
    * [`/loop`](/docs/zh-TW/scheduled-tasks)：僅限明確間隔
    * [Zero Data Retention](/docs/zh-TW/zero-data-retention)：受您的 Google Cloud 協議約束

    **替代方案：** 對於排程，使用具有明確間隔的 [`/loop`](/docs/zh-TW/scheduled-tasks) 而不是 `/schedule`。對於雲端工作階段，使用 [GitHub Actions](/docs/zh-TW/github-actions) 或 [GitLab CI/CD](/docs/zh-TW/gitlab-ci-cd)。
  </Tab>

  <Tab title="Microsoft Foundry">
    **不可用：** 所有[需要 Claude 訂閱的功能](#features-that-require-a-claude-subscription)，加上 [fast mode](/docs/zh-TW/fast-mode)、[Advisor](/docs/zh-TW/advisor)、[Channels](/docs/zh-TW/channels)、[GitHub Actions](/docs/zh-TW/github-actions) 和 [GitLab CI/CD](/docs/zh-TW/gitlab-ci-cd)、[分析儀表板](/docs/zh-TW/analytics)、[伺服器管理的設定](/docs/zh-TW/server-managed-settings) 和 [`/design-sync` 和 `/radio` 命令](/docs/zh-TW/commands#all-commands)。

    **部分支援：**

    * [Desktop](/docs/zh-TW/desktop)：僅透過[在 3P 上的 Claude Desktop](https://claude.com/docs/third-party/claude-desktop/overview)
    * [Auto mode](/docs/zh-TW/auto-mode-config)：Sonnet 5、Opus 4.7 和 Opus 4.8 僅限
    * [`/loop`](/docs/zh-TW/scheduled-tasks)：僅限明確間隔
    * [Zero Data Retention](/docs/zh-TW/zero-data-retention)：受您的 Azure 協議約束

    **替代方案：** 對於排程，使用具有明確間隔的 [`/loop`](/docs/zh-TW/scheduled-tasks) 而不是 `/schedule`。
  </Tab>

  <Tab title="Anthropic Console">
    **不可用：** 所有[需要 Claude 訂閱的功能](#features-that-require-a-claude-subscription)。

    [按提供者變化的 CLI 功能](#cli-capabilities-that-vary-by-provider)中的所有功能都可用，當 API 金鑰屬於 Team 或 Enterprise 組織時，[伺服器管理的設定](/docs/zh-TW/server-managed-settings)也可用。
  </Tab>
</Tabs>

<h2 id="availability-by-subscription-plan">
  按訂閱計畫的可用性
</h2>

如果您透過 Amazon Bedrock、Google Cloud 的 Agent Platform、Microsoft Foundry 或 Anthropic Console API 金鑰進行驗證，本節不適用於您。當您使用 claude.ai 帳戶登入時，您的計畫決定了下列功能中哪些可用。

| 功能                                                                          | Pro | Max | Team  | Enterprise                        |
| :-------------------------------------------------------------------------- | :-- | :-- | :---- | :-------------------------------- |
| [網頁上的 Claude Code](/docs/zh-TW/claude-code-on-the-web)                           | ✓   | ✓   | ✓     | ✓ <sup><a href="#fn6">6</a></sup> |
| [Routines](/docs/zh-TW/routines)                                                 | ✓   | ✓   | ✓     | ✓                                 |
| [Remote Control](/docs/zh-TW/remote-control)                                     | ✓   | ✓   | 管理員啟用 | 管理員啟用                             |
| [Channels](/docs/zh-TW/channels)                                                 | ✓   | ✓   | 管理員啟用 | 管理員啟用                             |
| [Computer use](/docs/zh-TW/computer-use)                                         | ✓   | ✓   | ✗     | ✗                                 |
| Dispatch ([Desktop](/docs/zh-TW/desktop#sessions-from-dispatch))                 | ✓   | ✓   | ✗     | ✗                                 |
| [Code Review](/docs/zh-TW/code-review)                                           | ✗   | ✗   | ✓     | ✓                                 |
| [Artifacts](/docs/zh-TW/artifacts)                                               | ✓   | ✓   | ✓     | 管理員啟用                             |
| [分析儀表板和貢獻指標](/docs/zh-TW/analytics)                                              | ✗   | ✗   | ✓     | ✓                                 |
| [Enterprise Analytics API](/docs/zh-TW/analytics#access-data-programmatically)   | ✗   | ✗   | ✗     | ✓                                 |
| [伺服器管理的設定](/docs/zh-TW/server-managed-settings)                                  | ✗   | ✗   | ✓     | ✓                                 |
| [SSO](https://support.claude.com/en/articles/9266767-what-is-the-team-plan) | ✗   | ✗   | ✓     | ✓                                 |
| SCIM                                                                        | ✗   | ✗   | ✗     | ✓                                 |
| [Compliance API](https://platform.claude.com/docs/en/api/compliance)        | ✗   | ✗   | ✗     | ✓                                 |
| [Zero Data Retention](/docs/zh-TW/zero-data-retention)                           | ✗   | ✗   | ✗     | ✓ <sup><a href="#fn7">7</a></sup> |

<span id="fn6" style={{display: 'block', position: 'relative', top: '-120px'}} /><sup>6</sup> 在 Enterprise 上，需要進階座位或 Chat + Claude Code 座位。請參閱[網頁上的 Claude Code](/docs/zh-TW/claude-code-on-the-web)。<br />
<span id="fn7" style={{display: 'block', position: 'relative', top: '-120px'}} /><sup>7</sup> 不包含在標準 Enterprise 計畫中。需要 Anthropic 為符合條件的帳戶進行單獨啟用。請參閱 [Zero Data Retention](/docs/zh-TW/zero-data-retention)。

如需定價和完整計畫比較，請參閱 [Team 計畫](https://support.claude.com/en/articles/9266767-what-is-the-team-plan)和 [Enterprise 計畫](https://support.claude.com/en/articles/9797531-what-is-the-enterprise-plan)。

<h2 id="model-availability">
  模型可用性
</h2>

如需每個提供者和地區可用的 Claude 模型和內容視窗大小，請參閱[模型配置](/docs/zh-TW/model-config)和[模型概述](https://platform.claude.com/docs/en/about-claude/models/overview)。Vision、PDF 輸入和擴展思考是模型功能而非 Claude Code 功能，在提供該模型的每個提供者上都有效。[Prompt caching](/docs/zh-TW/prompt-caching) 在大多數提供者上的運作方式相同；在 Amazon Bedrock 上，支援因模型而異。

<h2 id="related-resources">
  相關資源
</h2>

* [企業部署概述](/docs/zh-TW/third-party-integrations)：比較提供者之間的驗證、計費和地區
* 提供者設定指南：[Amazon Bedrock](/docs/zh-TW/amazon-bedrock)、[AWS 上的 Claude Platform](/docs/zh-TW/claude-platform-on-aws)、[Google Cloud 的 Agent Platform](/docs/zh-TW/google-vertex-ai)、[Microsoft Foundry](/docs/zh-TW/microsoft-foundry)
* [平台和整合](/docs/zh-TW/platforms)：Claude Code 執行的位置，包括 CLI、Desktop、IDE 擴充功能、網頁、行動和 CI/CD
