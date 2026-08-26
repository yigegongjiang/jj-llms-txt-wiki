> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 控制組織的 MCP 伺服器存取

> 使用受管配置檔案、允許清單和拒絕清單限制使用者可以新增或連接的 MCP 伺服器。

根據預設，任何執行 Claude Code 的人都可以連接他們選擇的任何 [MCP 伺服器](/docs/zh-TW/mcp)。Anthropic 在將連接器新增到 [Anthropic Directory](https://claude.ai/directory) 之前會根據其 [列表標準](https://claude.com/docs/connectors/building/review-criteria) 審查連接器，但不會對任何 MCP 伺服器進行安全審計或管理。作為管理員，您可以限制在組織中執行的伺服器，從部署固定的已批准集合到完全停用 MCP。

本頁涵蓋如何：

* [選擇符合您需要的控制級別的模式](#choose-a-pattern)
* [使用 `managed-mcp.json` 部署固定伺服器集合](#exclusive-control-with-managed-mcp-json)，包括如何 [完全停用 MCP](#disable-mcp-entirely)
* [使用允許清單和拒絕清單控制伺服器](#policy-based-control-with-allowlists-and-denylists)
* [告訴使用者當限制阻止伺服器時會發生什麼](#how-restrictions-appear-to-users)
* [監控您的組織實際使用的伺服器](#monitor-mcp-usage)

<Note>
  [安全](/docs/zh-TW/security) 頁面涵蓋 MCP 威脅模型以及如何在批准伺服器之前評估它。[決定要強制執行的內容](/docs/zh-TW/admin-setup#decide-what-to-enforce) 涵蓋 MCP 限制以及其他管理控制。
</Note>

<h2 id="choose-a-pattern">
  選擇模式
</h2>

Claude Code 支援一系列限制級別。每個模式使用以下機制中的一個或兩個：`managed-mcp.json` 用於部署固定集合，`allowedMcpServers`/`deniedMcpServers` 用於篩選使用者配置的內容。

| 模式           | 功能                                    | 配置                                                                                            |
| :----------- | :------------------------------------ | :-------------------------------------------------------------------------------------------- |
| **停用 MCP**   | 任何地方都不載入伺服器                           | `managed-mcp.json` 包含空伺服器對應                                                                   |
| **固定部署**     | 每個使用者獲得相同的伺服器，無法新增其他伺服器               | `managed-mcp.json` 包含您想要的伺服器                                                                  |
| **已批准目錄**    | 發佈已批准伺服器的清單；使用者新增他們想要的伺服器，其他所有伺服器都被阻止 | `allowedMcpServers` + `allowManagedMcpServersOnly: true`                                      |
| **僅外掛程式伺服器** | 伺服器只能來自外掛程式；使用者無法新增自己的伺服器             | [`strictPluginOnlyCustomization`](/docs/zh-TW/settings#strictpluginonlycustomization) 包含清單中的 `mcp` |
| **軟允許清單**    | 強制執行允許清單，使用者可以在自己的設定中擴展               | `allowedMcpServers` 不含 `allowManagedMcpServersOnly`                                           |
| **僅拒絕清單**    | 阻止已知的不良伺服器，允許其他所有伺服器                  | `deniedMcpServers`                                                                            |
| **無限制**      | 使用者新增任何內容                             | 不部署任何受管 MCP 配置                                                                                |

<Note>
  Claude Code 沒有內建的 MCP 伺服器登錄表，使用者可以從中瀏覽和安裝。對於已批准目錄模式，在使用者會找到的地方（例如內部 wiki）共享已批准清單及其 `claude mcp add` 命令，或通過 [受管外掛程式市場](/docs/zh-TW/plugin-marketplaces#managed-marketplace-restrictions) 將伺服器作為外掛程式分發，以便使用者可以從 `/plugin` 瀏覽和安裝它們。
</Note>

<h2 id="exclusive-control-with-managed-mcp-json">
  使用 managed-mcp.json 進行獨佔控制
</h2>

如果您部署 `managed-mcp.json` 檔案，Claude Code 只會載入該檔案定義的伺服器。使用者無法新增、修改或使用任何其他 MCP 伺服器，包括外掛程式提供的伺服器。該檔案也會抑制 claude.ai 連接器，除非您 [允許它們與受管集合並存](#allow-claude-ai-connectors-alongside-the-managed-set)。

另外兩個設定可以進一步篩選受管集合：

* `allowedMcpServers` 和 `deniedMcpServers` 也適用於受管伺服器，因此不符合它們的受管伺服器將不會載入。
* 使用者自己的 `deniedMcpServers` 從他們的設定中合併，因此使用者可以為自己阻止受管伺服器。

有關檢查的完整順序，請參閱 [伺服器如何被評估](#how-a-server-is-evaluated)。

`managed-mcp.json` 是一個獨立檔案，因此無法通過 [伺服器受管設定](/docs/zh-TW/server-managed-settings) 傳遞。任何可以寫入具有管理員權限的系統路徑的程序都可以部署它。在大規模部署中，通常通過裝置管理工具進行，例如 macOS 上的 Jamf 或配置檔案、Windows 上的群組原則或 Intune，或 Linux 上您選擇的艦隊管理。Claude Code 在以下路徑之一查找該檔案：

| 平台          | 路徑                                                         |
| :---------- | :--------------------------------------------------------- |
| macOS       | `/Library/Application Support/ClaudeCode/managed-mcp.json` |
| Linux 和 WSL | `/etc/claude-code/managed-mcp.json`                        |
| Windows     | `C:\Program Files\ClaudeCode\managed-mcp.json`             |

該檔案使用與專案 [`.mcp.json`](/docs/zh-TW/mcp#project-scope) 檔案相同的格式：

```json theme={null}
{
  "mcpServers": {
    "github": {
      "type": "http",
      "url": "https://api.githubcopilot.com/mcp/"
    },
    "sentry": {
      "type": "http",
      "url": "https://mcp.sentry.dev/mcp"
    },
    "company-internal": {
      "type": "stdio",
      "command": "/usr/local/bin/company-mcp-server",
      "args": ["--config", "/etc/company/mcp-config.json"],
      "env": {
        "COMPANY_API_URL": "https://internal.example.com"
      }
    }
  }
}
```

<h3 id="authenticate-with-per-user-credentials">
  使用每個使用者的認證進行身份驗證
</h3>

機器上的任何使用者都可以讀取此檔案，因此不要在 `env` 區塊中儲存 API 金鑰或其他認證。改用以下其中一種方式傳遞每個使用者的認證：

* [`${VAR}` 擴展](/docs/zh-TW/mcp#environment-variable-expansion-in-mcp-json) 從每個使用者的環境中讀取機密。
* [OAuth 或每個使用者的標頭](/docs/zh-TW/mcp#authenticate-with-remote-mcp-servers) 以便每個使用者以自己的身份進行身份驗證。
* [`headersHelper`](/docs/zh-TW/mcp#use-dynamic-headers-for-custom-authentication) 在連接時生成認證。

<h3 id="validate-the-configuration">
  驗證配置
</h3>

要確認檔案有效，請在受管機器上執行兩項檢查：

1. `claude mcp list` 只顯示 `managed-mcp.json` 中的伺服器。如果使用者自己的伺服器仍然出現，則檔案未被讀取；檢查路徑和權限。
2. `claude mcp add --transport http test https://example.com/mcp` 失敗，並顯示 `Cannot add MCP server: enterprise MCP configuration is active and has exclusive control over MCP servers`。URL 不需要是真實伺服器，因為原則檢查在聯繫任何內容之前拒絕該命令。

<h3 id="disable-mcp-entirely">
  完全停用 MCP
</h3>

部署包含空伺服器對應的 `managed-mcp.json` 以阻止每個 MCP 伺服器：

```json theme={null}
{
  "mcpServers": {}
}
```

使用者在 `/mcp` 中看不到任何 MCP 伺服器，`claude mcp add` 失敗，並顯示上述企業原則錯誤。使用者之前配置的伺服器在下次啟動會話時停止載入，沒有警告說明原則是原因。

<h3 id="allow-claude-ai-connectors-alongside-the-managed-set">
  允許 claude.ai 連接器與受管集合並存
</h3>

部署 `managed-mcp.json` 預設會抑制 [claude.ai 連接器](/docs/zh-TW/mcp#use-mcp-servers-from-claude-ai)，包括管理員在 claude.ai 管理控制台中為組織配置的連接器。要將這些連接器與 `managed-mcp.json` 中的伺服器一起載入，請在 [受管設定來源](/docs/zh-TW/admin-setup#decide-how-settings-reach-devices) 中設定 `"allowAllClaudeAiMcps": true`。需要 Claude Code v2.1.149 或更新版本。

啟用此設定後，Claude Code 會載入與未部署 `managed-mcp.json` 時相同的 claude.ai 連接器。[允許清單和拒絕清單](#policy-based-control-with-allowlists-and-denylists) 仍然適用於這些連接器，因此您可以使用 `deniedMcpServers` 阻止特定連接器。此設定僅影響 claude.ai 連接器；外掛程式提供的伺服器保持被抑制。

Claude Code 只從管理員控制的原則層級讀取此設定：伺服器受管設定、MDM 部署的 plist 或 HKLM 登錄機碼，或系統 `managed-settings.json` 檔案。將其放在使用者或專案設定中無效，因此使用者無法重新啟用獨佔控制所抑制的連接器。

<h2 id="policy-based-control-with-allowlists-and-denylists">
  使用允許清單和拒絕清單進行基於原則的控制
</h2>

允許清單和拒絕清單篩選允許載入的已配置伺服器。它們不是登錄表：伺服器仍然必須由使用者、外掛程式或 `managed-mcp.json` 新增，然後允許清單或拒絕清單才會應用於它。要將伺服器部署給使用者，請使用 [`managed-mcp.json`](#exclusive-control-with-managed-mcp-json)。兩個清單也會篩選使用 [`--mcp-config` CLI 旗標](/docs/zh-TW/cli-reference#cli-flags) 傳遞的伺服器；`--strict-mcp-config` 限制載入的配置檔案，不會繞過任一清單。

要使允許清單具有權威性，請在 [受管設定來源](/docs/zh-TW/admin-setup#decide-how-settings-reach-devices)（例如伺服器受管設定或已部署的 `managed-settings.json` 檔案）中一起設定 `allowedMcpServers` 和 `allowManagedMcpServersOnly: true`。[將允許清單限制為僅受管設定](#restrict-the-allowlist-to-managed-settings-only) 顯示配置。沒有 `allowManagedMcpServersOnly`，來自每個設定來源的允許清單會合併，包括使用者自己的 `~/.claude/settings.json`，因此使用者可以擴展您的允許清單允許的內容。拒絕清單無論如何都會從每個來源合併。

<Note>
  `allowManagedMcpServersOnly` 與 `allowManagedPermissionRulesOnly` 分開，後者鎖定 [權限規則](/docs/zh-TW/permissions#managed-settings) 只。設定該標誌不會強制執行 MCP 允許清單。
</Note>

<h3 id="match-servers-by-url-command-or-name">
  按 URL、命令或名稱匹配伺服器
</h3>

`allowedMcpServers` 和 `deniedMcpServers` 是條目清單。每個條目是一個物件，具有單一鍵，用於按 URL、命令或名稱識別伺服器：

| 鍵               | 匹配                       | 用於             |
| :-------------- | :----------------------- | :------------- |
| `serverUrl`     | 遠端伺服器 URL，精確或帶有 `*` 萬用字元 | HTTP 和 SSE 伺服器 |
| `serverCommand` | 啟動 stdio 伺服器的確切命令和引數     | Stdio 伺服器      |
| `serverName`    | 使用者指派的標籤。僅精確匹配；萬用字元不展開   | 任一類型，但請參閱下面的警告 |

不設定 `allowedMcpServers` 與將其設定為空陣列不同：

| 設定                  | 未設定（預設）  | 空陣列 `[]` | 已填入       |
| :------------------ | :------- | :------- | :-------- |
| `allowedMcpServers` | 允許所有伺服器  | 不允許任何伺服器 | 僅允許匹配的伺服器 |
| `deniedMcpServers`  | 不阻止任何伺服器 | 不阻止任何伺服器 | 阻止匹配的伺服器  |

請參閱 [受管設定中的無效條目](/docs/zh-TW/settings#invalid-entries-in-managed-settings) 以了解當條目無法通過結構描述驗證時會發生什麼。

<Warning>
  `serverName` 條目（在任一清單中）不是安全控制。名稱是使用者在執行 `claude mcp add` 或編輯配置檔案時指派的標籤，而不是基礎伺服器，因此使用者可以呼叫任何伺服器 `github`。對於 claude.ai 連接器，名稱是 claude.ai 傳回的顯示名稱，可能會變更。要強制執行實際執行的伺服器，請新增 `serverCommand` 或 `serverUrl` 條目。
</Warning>

`serverName` 驗證在兩個清單之間有所不同：

* 在 `deniedMcpServers` 中，`serverName` 接受任何非空字串，因此您可以按顯示名稱阻止 [claude.ai 連接器](/docs/zh-TW/mcp#use-mcp-servers-from-claude-ai)。例如，`{ "serverName": "claude.ai Slack" }` 阻止 Slack 連接器。當您需要拒絕對重新命名具有魯棒性時，或當連接器名稱衝突並獲得 ` (N)` 尾碼時，優先使用 `serverUrl` 條目。
* 在 `allowedMcpServers` 中，`serverName` 限制為字母、數字、連字號和底線。使用 `serverUrl` 來允許列出 claude.ai 連接器。

要關閉所有 claude.ai 連接器，請參閱 [`disableClaudeAiConnectors`](/docs/zh-TW/mcp#disable-claude-ai-connectors)。

<h3 id="how-a-server-is-evaluated">
  伺服器如何被評估
</h3>

在載入伺服器之前，包括來自 `managed-mcp.json` 的伺服器，Claude Code 按順序執行三項檢查：

1. **合併清單。** 來自每個設定來源的允許清單和拒絕清單條目合併為一個允許清單和一個拒絕清單。當 `allowManagedMcpServersOnly` 為 `true` 時，僅保留受管允許清單；拒絕清單始終從每個來源合併。
2. **檢查拒絕清單。** 與任何拒絕清單條目匹配的伺服器（按 URL、命令或名稱）被阻止。沒有任何內容會覆蓋拒絕清單匹配。
3. **檢查允許清單。** 如果 `allowedMcpServers` 未在任何地方設定，通過拒絕清單的每個伺服器都會載入。如果已設定，伺服器必須匹配的內容取決於其類型，如下表所示。

| 伺服器類型          | 在匹配時允許                                                               |
| :------------- | :------------------------------------------------------------------- |
| 遠端（HTTP 或 SSE） | 一個 `serverUrl` 條目。`serverName` 匹配僅在允許清單不包含 `serverUrl` 條目時計數         |
| Stdio          | 一個 `serverCommand` 條目。`serverName` 匹配僅在允許清單不包含 `serverCommand` 條目時計數 |

這些檢查中適用三個匹配規則：

* **命令精確匹配。** 每個引數，按順序。`["npx", "-y", "server"]` 不匹配 `["npx", "server"]` 或 `["npx", "-y", "server", "--flag"]`。
* **`serverCommand` 和 `serverUrl` 值在匹配前展開。** 原則條目和伺服器的配置值都會經過與 `.mcp.json` 相同的 [`${VAR}` 和 `${VAR:-default}` 展開](/docs/zh-TW/mcp#environment-variable-expansion-in-mcp-json)，因此寫成 `["${HOME}/bin/server"]` 的條目會匹配使用相同參考或展開路徑的伺服器配置。在 Windows 上，參考在那裡設定的環境變數，例如 `${USERPROFILE}` 而不是 `${HOME}`。`serverName` 值按字面匹配，永遠不展開。
* **URL 支援 `*` 萬用字元** 在模式中的任何地方，包括方案。主機名匹配不區分大小寫，忽略尾部 FQDN 點，因此 `https://Mcp.Example.com/*` 匹配 `https://mcp.example.com/api`。路徑保持區分大小寫。

| 模式                          | 允許                         |
| :-------------------------- | :------------------------- |
| `https://mcp.example.com/*` | 特定網域上的所有路徑                 |
| `https://mcp.example.com`   | 也允許該網域上的所有路徑。沒有路徑的模式匹配任何路徑 |
| `https://*.example.com/*`   | `example.com` 的任何子網域       |
| `http://localhost:*/*`      | localhost 上的任何連接埠          |
| `*://mcp.example.com/*`     | 任何方案到特定網域                  |

因為 `${VAR}` 展開讀取 Claude Code 自己的程序環境，參考變數的 `serverCommand` 或 `serverUrl` 原則條目會展開為使用者設定的任何值。對於您依賴強制執行的條目，使用字面 URL 和命令。

<h3 id="example-configuration">
  範例配置
</h3>

以下配置設定了硬允許清單和拒絕清單。突出顯示的行改變了清單其餘部分的評估方式，區塊後的標註解釋了每一行：

```json {3,5,11} theme={null}
{
  "allowedMcpServers": [
    { "serverUrl": "https://api.githubcopilot.com/*" },
    { "serverUrl": "https://mcp.sentry.dev/*" },
    { "serverCommand": ["npx", "-y", "@modelcontextprotocol/server-filesystem", "."] },
    { "serverCommand": ["python", "/usr/local/bin/approved-server.py"] },
    { "serverUrl": "https://mcp.example.com/*" },
    { "serverUrl": "https://*.internal.example.com/*" }
  ],
  "deniedMcpServers": [
    { "serverName": "dangerous-server" },
    { "serverCommand": ["npx", "-y", "unapproved-package"] },
    { "serverUrl": "https://*.untrusted.example.com/*" }
  ]
}
```

* **第 3 行**：第一個 `serverUrl` 條目。一旦存在，每個遠端伺服器必須匹配 URL 模式，因此使用者無法通過給它一個允許的名稱來獲得未列出的遠端伺服器。
* **第 5 行**：第一個 `serverCommand` 條目。對 stdio 伺服器有相同的效果，因此每個本地伺服器必須精確匹配列出的命令。
* **第 11 行**：拒絕清單中的 `serverName` 條目。拒絕清單條目始終適用，因此任何名為 `dangerous-server` 的伺服器都被阻止，無論其 URL 或命令如何。

此允許清單中的 `serverName` 條目永遠不會匹配任何內容，因為兩種傳輸類型都已有更嚴格的條目。

下面的手風琴演示了伺服器如何針對其他允許清單和拒絕清單組合進行評估。

<Accordion title="僅 URL 允許清單">
  ```json theme={null}
  {
    "allowedMcpServers": [
      { "serverUrl": "https://mcp.example.com/*" },
      { "serverUrl": "https://*.internal.example.com/*" }
    ]
  }
  ```

  | 伺服器                                                | 結果              |
  | :------------------------------------------------- | :-------------- |
  | `https://mcp.example.com/api` 上的 HTTP 伺服器          | 允許：匹配 URL 模式    |
  | `https://api.internal.example.com/mcp` 上的 HTTP 伺服器 | 允許：匹配萬用字元子網域    |
  | `https://external.example.com/mcp` 上的 HTTP 伺服器     | 阻止：不匹配任何 URL 模式 |
  | 具有任何命令的 Stdio 伺服器                                  | 阻止：沒有名稱或命令條目可匹配 |
</Accordion>

<Accordion title="僅命令允許清單">
  ```json theme={null}
  {
    "allowedMcpServers": [
      { "serverCommand": ["npx", "-y", "approved-package"] }
    ]
  }
  ```

  | 伺服器                                                | 結果           |
  | :------------------------------------------------- | :----------- |
  | 具有 `["npx", "-y", "approved-package"]` 的 Stdio 伺服器 | 允許：匹配命令      |
  | 具有 `["node", "server.js"]` 的 Stdio 伺服器             | 阻止：不匹配命令     |
  | 名為 `my-api` 的 HTTP 伺服器                             | 阻止：沒有名稱條目可匹配 |
</Accordion>

<Accordion title="混合名稱和命令允許清單">
  ```json theme={null}
  {
    "allowedMcpServers": [
      { "serverName": "github" },
      { "serverCommand": ["npx", "-y", "approved-package"] }
    ]
  }
  ```

  | 伺服器                                                                 | 結果                         |
  | :------------------------------------------------------------------ | :------------------------- |
  | 名為 `local-tool` 且具有 `["npx", "-y", "approved-package"]` 的 Stdio 伺服器 | 允許：匹配命令                    |
  | 名為 `local-tool` 且具有 `["node", "server.js"]` 的 Stdio 伺服器             | 阻止：命令條目存在但不匹配              |
  | 名為 `github` 且具有 `["node", "server.js"]` 的 Stdio 伺服器                 | 阻止：stdio 伺服器在命令條目存在時必須匹配命令 |
  | 名為 `github` 的 HTTP 伺服器                                              | 允許：匹配名稱                    |
  | 名為 `other-api` 的 HTTP 伺服器                                           | 阻止：名稱不匹配                   |
</Accordion>

<Accordion title="僅名稱允許清單">
  ```json theme={null}
  {
    "allowedMcpServers": [
      { "serverName": "github" },
      { "serverName": "internal-tool" }
    ]
  }
  ```

  | 伺服器                                   | 結果        |
  | :------------------------------------ | :-------- |
  | 名為 `github` 且具有任何命令的 Stdio 伺服器        | 允許：沒有命令限制 |
  | 名為 `internal-tool` 且具有任何命令的 Stdio 伺服器 | 允許：沒有命令限制 |
  | 名為 `github` 的 HTTP 伺服器                | 允許：匹配名稱   |
  | 任何名為 `other` 的伺服器                     | 阻止：名稱不匹配  |
</Accordion>

<Accordion title="具有拒絕清單覆蓋的允許清單">
  ```json theme={null}
  {
    "allowedMcpServers": [
      { "serverUrl": "https://*.example.com/*" }
    ],
    "deniedMcpServers": [
      { "serverUrl": "https://staging.example.com/*" }
    ]
  }
  ```

  | 伺服器                                           | 結果                       |
  | :-------------------------------------------- | :----------------------- |
  | `https://mcp.example.com/api` 上的 HTTP 伺服器     | 允許：匹配允許清單 URL 模式，無拒絕清單匹配 |
  | `https://staging.example.com/api` 上的 HTTP 伺服器 | 阻止：匹配兩者，但拒絕清單優先          |
  | `https://other.com/mcp` 上的 HTTP 伺服器           | 阻止：不匹配允許清單               |
</Accordion>

<h3 id="restrict-the-allowlist-to-managed-settings-only">
  將允許清單限制為僅受管設定
</h3>

要使受管允許清單成為唯一適用的清單，請在受管設定檔案中設定 `allowManagedMcpServersOnly`：

```json theme={null}
{
  "allowManagedMcpServersOnly": true,
  "allowedMcpServers": [
    { "serverUrl": "https://api.githubcopilot.com/*" },
    { "serverUrl": "https://*.internal.example.com/*" }
  ]
}
```

當 `allowManagedMcpServersOnly` 為 `true` 時，來自使用者、專案和本地設定的允許清單被忽略。拒絕清單仍然從所有來源合併，因此使用者始終可以為自己阻止伺服器。

<h2 id="how-restrictions-appear-to-users">
  限制如何向使用者顯示
</h2>

當限制阻止伺服器時，使用者要麼看到來自 `claude mcp add` 的錯誤，要麼伺服器無聲地停止載入。使用此表格識別這些報告，並在推出變更之前告訴使用者會發生什麼：

| 限制                                           | 使用者看到的內容                                                                                                   |
| :------------------------------------------- | :--------------------------------------------------------------------------------------------------------- |
| `managed-mcp.json` 存在且使用者執行 `claude mcp add` | `Cannot add MCP server: enterprise MCP configuration is active and has exclusive control over MCP servers` |
| 伺服器在拒絕清單上且使用者執行 `claude mcp add`             | `Cannot add MCP server "<name>": server is explicitly blocked by enterprise policy`                        |
| 伺服器不在允許清單上且使用者執行 `claude mcp add`            | `Cannot add MCP server "<name>": not allowed by enterprise policy`                                         |
| 之前配置的伺服器現在被原則阻止                              | 伺服器無聲地從 `/mcp` 和 `claude mcp list` 消失，沒有警告                                                                 |

在最後一種情況下，使用者沒有收到原則是其伺服器消失原因的信號，因此在推出新限制時告訴受影響的使用者哪些伺服器被阻止。

<h2 id="monitor-mcp-usage">
  監控 MCP 使用
</h2>

當 [OpenTelemetry 匯出](/docs/zh-TW/monitoring-usage) 配置時，Claude Code 可以記錄使用者呼叫的 MCP 伺服器和工具。設定 `OTEL_LOG_TOOL_DETAILS=1` 以在工具事件中包含 MCP 伺服器和工具名稱，然後在您的收集器中聚合它們以查看您的使用者實際連接到的伺服器。請參閱 [監控](/docs/zh-TW/monitoring-usage) 以設定匯出器和完整事件架構。

<h2 id="configuration-summary">
  配置摘要
</h2>

本頁涵蓋的每個檔案和設定、它控制的內容以及如何傳遞它：

| 表面                           | 控制的內容                                           | 位置                                                                                                   | 傳遞方式                                                                                                             |
| :--------------------------- | :---------------------------------------------- | :--------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------- |
| `managed-mcp.json`           | 固定伺服器集合、獨佔控制                                    | 系統路徑：`/Library/Application Support/ClaudeCode/`、`/etc/claude-code/` 或 `C:\Program Files\ClaudeCode\` | MDM、GPO、艦隊管理或任何具有管理員權限的程序。無法通過伺服器受管設定設定                                                                          |
| `allowedMcpServers`          | 允許的伺服器允許清單                                      | 任何 [設定檔案](/docs/zh-TW/settings#settings-files)；來自每個來源的條目合併，除非設定了 `allowManagedMcpServersOnly`             | 為了強制執行，[受管設定來源](/docs/zh-TW/admin-setup#decide-how-settings-reach-devices)：伺服器受管設定、`managed-settings.json`、MDM 設定檔或登錄 |
| `deniedMcpServers`           | 被阻止的伺服器拒絕清單                                     | 任何設定檔案；來自每個來源的條目合併                                                                                   | 與 `allowedMcpServers` 相同                                                                                         |
| `allowManagedMcpServersOnly` | 將允許清單鎖定為僅受管來源                                   | 僅受管設定來源；該設定在其他地方無效                                                                                   | 與 `allowedMcpServers` 相同                                                                                         |
| `allowAllClaudeAiMcps`       | 在 `managed-mcp.json` 旁邊載入 claude.ai 連接器，而不是抑制它們 | 僅受管設定來源；該設定在其他地方無效                                                                                   | 與 `allowedMcpServers` 相同                                                                                         |

<h2 id="related-resources">
  相關資源
</h2>

* [決定要強制執行的內容](/docs/zh-TW/admin-setup#decide-what-to-enforce)：MCP 限制以及權限規則、沙箱和其他管理控制
* [通過 MCP 將 Claude Code 連接到工具](/docs/zh-TW/mcp)：完整的 MCP 參考，包括傳輸、範圍和身份驗證
* [設定](/docs/zh-TW/settings)：設定層次結構以及受管設定如何優先
* [伺服器受管設定](/docs/zh-TW/server-managed-settings)：從 Claude.ai 管理控制台傳遞 `allowedMcpServers` 和 `deniedMcpServers`
* [安全](/docs/zh-TW/security)：這些控制防禦的威脅模型
* [Claude Enterprise Administrator Guide](https://claude.com/resources/tutorials/claude-enterprise-administrator-guide)：SSO、SCIM、座位管理和推出劇本
