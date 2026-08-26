> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 透過市場探索和安裝預建外掛程式

> 從市場探索和安裝外掛程式，以使用新技能、代理和功能擴展 Claude Code。

外掛程式透過技能、代理、hooks 和 MCP servers 擴展 Claude Code。外掛程式市場是幫助您探索和安裝這些擴展的目錄，無需自己構建它們。

想要建立和分發您自己的市場？請參閱[建立和分發外掛程式市場](/docs/zh-TW/plugin-marketplaces)。

<h2 id="how-marketplaces-work">
  市場如何運作
</h2>

市場是他人建立和共享的外掛程式目錄。使用市場是一個兩步流程：

<Steps>
  <Step title="新增市場">
    這會向 Claude Code 註冊目錄，以便您可以瀏覽可用內容。尚未安裝任何外掛程式。
  </Step>

  <Step title="安裝個別外掛程式">
    瀏覽目錄並安裝您想要的外掛程式。
  </Step>
</Steps>

將其視為新增應用程式商店：新增商店可讓您存取瀏覽其集合，但您仍然可以選擇個別下載哪些應用程式。

<h2 id="official-anthropic-marketplace">
  官方 Anthropic 市場
</h2>

官方 Anthropic 市場 (`claude-plugins-official`) 在您啟動 Claude Code 時自動可用。執行 `/plugin` 並前往 **Discover** 標籤以瀏覽可用內容，或在 [claude.com/plugins](https://claude.com/plugins) 查看目錄。

若要從官方市場安裝外掛程式，請使用 `/plugin install <name>@claude-plugins-official`。例如，若要安裝 GitHub 整合：

```shell theme={null}
/plugin install github@claude-plugins-official
```

如果 Claude Code 報告在任何市場中找不到外掛程式，您的市場可能遺失或已過期。執行 `/plugin marketplace update claude-plugins-official` 以重新整理它，或如果您之前未新增過，執行 `/plugin marketplace add anthropics/claude-plugins-official`。然後重試安裝。

<Note>
  官方市場由 Anthropic 維護，包含由 Anthropic 自行決定。應用內提交表單會將外掛程式新增到[社群市場](#community-marketplace)，而不是官方市場。若要獨立分發外掛程式，請[建立您自己的市場](/docs/zh-TW/plugin-marketplaces)並與使用者共享。
</Note>

官方市場包括多個外掛程式類別：

<h3 id="code-intelligence">
  程式碼智能
</h3>

程式碼智能外掛程式啟用 Claude Code 的內建 LSP 工具，使 Claude 能夠跳轉到定義、尋找參考資料，並在編輯後立即查看類型錯誤。這些外掛程式配置[語言伺服器協議](https://microsoft.github.io/language-server-protocol/)連接，這是為 VS Code 程式碼智能提供動力的相同技術。

這些外掛程式需要在您的系統上安裝語言伺服器二進位檔。如果您已經安裝了語言伺服器，當您開啟專案時，Claude 可能會提示您安裝相應的外掛程式。

| 語言         | 外掛程式                | 所需的二進位檔                      |
| :--------- | :------------------ | :--------------------------- |
| C/C++      | `clangd-lsp`        | `clangd`                     |
| C#         | `csharp-lsp`        | `csharp-ls`                  |
| Go         | `gopls-lsp`         | `gopls`                      |
| Java       | `jdtls-lsp`         | `jdtls`                      |
| Kotlin     | `kotlin-lsp`        | `kotlin-language-server`     |
| Lua        | `lua-lsp`           | `lua-language-server`        |
| PHP        | `php-lsp`           | `intelephense`               |
| Python     | `pyright-lsp`       | `pyright-langserver`         |
| Rust       | `rust-analyzer-lsp` | `rust-analyzer`              |
| Swift      | `swift-lsp`         | `sourcekit-lsp`              |
| TypeScript | `typescript-lsp`    | `typescript-language-server` |

您也可以[為其他語言建立您自己的 LSP 外掛程式](/docs/zh-TW/plugins-reference#lsp-servers)。

<Note>
  如果在安裝外掛程式後在 `/plugin` Errors 標籤中看到 `Executable not found in $PATH`，請從上表安裝所需的二進位檔。
</Note>

<h4 id="what-claude-gains-from-code-intelligence-plugins">
  Claude 從程式碼智能外掛程式獲得的功能
</h4>

安裝程式碼智能外掛程式並且其語言伺服器二進位檔可用後，Claude 獲得兩項功能：

* **自動診斷**：在 Claude 進行每次檔案編輯後，語言伺服器分析變更並自動報告錯誤和警告。Claude 看到類型錯誤、遺漏的匯入和語法問題，無需執行編譯器或 linter。如果 Claude 引入錯誤，它會注意到並在同一輪中修復問題。這不需要超出安裝外掛程式的任何配置。當「找到診斷」指示器出現時，您可以按 **Ctrl+O** 來內聯查看診斷。
* **程式碼導航**：Claude 可以使用語言伺服器跳轉到定義、尋找參考資料、懸停時取得類型資訊、列出符號、尋找實現和追蹤呼叫層次結構。這些操作為 Claude 提供比基於 grep 的搜尋更精確的導航，儘管可用性可能因語言和環境而異。

如果您遇到問題，請參閱[程式碼智能故障排除](#code-intelligence-issues)。

<h3 id="external-integrations">
  外部整合
</h3>

這些外掛程式捆綁預先配置的 [MCP servers](/docs/zh-TW/mcp)，以便您可以連接 Claude 到外部服務，無需手動設定：

* **原始碼控制**：`github`、`gitlab`
* **專案管理**：`atlassian`（Jira/Confluence）、`asana`、`linear`、`notion`
* **設計**：`figma`
* **基礎設施**：`vercel`、`firebase`、`supabase`
* **通訊**：`slack`
* **監控**：`sentry`

<h3 id="automatic-security-review">
  自動安全審查
</h3>

`security-guidance` 外掛程式審查 Claude 進行的每項變更是否存在常見漏洞，並指示 Claude 在同一工作階段中修復發現的問題。請參閱[在 Claude 編寫程式碼時捕捉安全問題](/docs/zh-TW/security-guidance)以了解它檢查的內容以及如何新增專案特定的規則。

<h3 id="development-workflows">
  開發工作流程
</h3>

為常見開發任務新增技能和代理的外掛程式：

* **commit-commands**：Git 提交工作流程，包括提交、推送和 PR 建立
* **pr-review-toolkit**：用於審查拉取請求的專門代理
* **agent-sdk-dev**：使用 Claude Agent SDK 構建的工具
* **plugin-dev**：建立您自己的外掛程式的工具組

<h3 id="output-styles">
  輸出樣式
</h3>

自訂 Claude 的回應方式：

* **explanatory-output-style**：關於實現選擇的教育見解
* **learning-output-style**：用於技能建立的互動式學習模式

<h2 id="community-marketplace">
  社群市場
</h2>

位於 [`anthropics/claude-plugins-community`](https://github.com/anthropics/claude-plugins-community) 的社群市場託管已通過 Anthropic 自動驗證和安全篩選的第三方外掛程式。每個外掛程式都固定到目錄中的特定提交 SHA。與官方市場不同，您需要手動新增它：

```shell theme={null}
/plugin marketplace add anthropics/claude-plugins-community
```

然後使用 `claude-community` 市場名稱從中安裝外掛程式：

```shell theme={null}
/plugin install <plugin-name>@claude-community
```

若要將您自己的外掛程式提交到社群市場，請參閱建立外掛程式指南中的[將您的外掛程式提交到社群市場](/docs/zh-TW/plugins#submit-your-plugin-to-the-community-marketplace)。

<h2 id="try-it-add-the-demo-marketplace">
  試試看：新增演示市場
</h2>

Anthropic 也維護一個[演示外掛程式市場](https://github.com/anthropics/claude-code/tree/main/plugins)（`claude-code-plugins`），其中包含展示外掛程式系統可能性的範例外掛程式。與官方市場不同，您需要手動新增此市場。

<Steps>
  <Step title="新增市場">
    在 Claude Code 中，為 `anthropics/claude-code` 市場執行 `plugin marketplace add` 命令：

    ```shell theme={null}
    /plugin marketplace add anthropics/claude-code
    ```

    這會下載市場目錄並使其外掛程式可供您使用。
  </Step>

  <Step title="瀏覽可用外掛程式">
    執行 `/plugin` 以開啟外掛程式管理器。這會開啟一個標籤式介面，其中有四個標籤，您可以使用 **Tab** 鍵（或 **Shift+Tab** 向後）循環瀏覽：

    * **Discover**：從所有市場瀏覽可用外掛程式
    * **Installed**：檢視和管理已安裝的外掛程式
    * **Marketplaces**：新增、移除或更新已新增的市場
    * **Errors**：檢視任何外掛程式載入錯誤

    前往 **Discover** 標籤以查看您剛新增的市場中的外掛程式。當您的管理員已透過 [`pluginSuggestionMarketplaces`](/docs/zh-TW/settings#available-settings) 受管設定將市場加入允許清單時，標記為與您目前工作目錄相關的外掛程式會釘在頂部，並帶有 **suggested for this directory** 標籤。
  </Step>

  <Step title="安裝外掛程式">
    選擇外掛程式以檢視其詳細資訊。詳細資訊窗格會顯示外掛程式包含的內容及其成本：

    * **Context cost** 估計，讓您可以查看外掛程式每回合會為您的[內容視窗](/docs/zh-TW/features-overview#understand-context-costs)新增多少個 token（Claude Code v2.1.143 及更新版本）
    * 外掛程式的 **Last updated** 日期（v2.1.144 及更新版本）
    * **Will install** 區段，列出外掛程式的命令、代理程式、skills、hooks 和 MCP 及 LSP 伺服器，讓您可以在安裝前檢視它新增的確切內容（v2.1.145 及更新版本）

    選擇安裝範圍：

    * **User scope**：在所有專案中為自己安裝
    * **Project scope**：為此儲存庫上的所有協作者安裝
    * **Local scope**：僅在此儲存庫中為自己安裝

    例如，選擇 **commit-commands**（新增 git 工作流程 skills 的外掛程式）並將其安裝到您的使用者範圍。

    您也可以直接從命令列安裝：

    ```shell theme={null}
    /plugin install commit-commands@claude-code-plugins
    ```

    請參閱[配置範圍](/docs/zh-TW/settings#configuration-scopes)以深入瞭解範圍。
  </Step>

  <Step title="使用您的新外掛程式">
    安裝後，執行 `/reload-plugins` 以啟動外掛程式。外掛程式 skills 由外掛程式名稱命名空間，因此 **commit-commands** 提供 `/commit-commands:commit` 之類的 skills。

    透過對檔案進行變更並執行以下命令來試試看：

    ```shell theme={null}
    /commit-commands:commit
    ```

    這會暫存您的變更、產生提交訊息並建立提交。

    每個外掛程式的工作方式不同。檢查 **Discover** 標籤中的外掛程式詳細資訊以查看它提供的命令和 skills，或造訪其首頁以取得使用指導。
  </Step>
</Steps>

本指南的其餘部分涵蓋了您可以新增市場、安裝外掛程式和管理配置的所有方式。

<h2 id="add-marketplaces">
  新增市場
</h2>

使用 `/plugin marketplace add` 命令從不同來源新增市場。

<Tip>
  **快捷方式**：您可以使用 `/plugin market` 代替 `/plugin marketplace`，以及 `rm` 代替 `remove`。
</Tip>

* **GitHub 儲存庫**：`owner/repo` 格式，例如 `anthropics/claude-code`
* **Git URL**：任何 git 儲存庫 URL，包括 GitLab、Bitbucket 和自託管伺服器
* **本機路徑**：目錄或 `marketplace.json` 檔案的直接路徑
* **遠端 URL**：託管 `marketplace.json` 檔案的直接 URL

<h3 id="add-from-github">
  從 GitHub 新增
</h3>

使用 `owner/repo` 格式新增包含 `.claude-plugin/marketplace.json` 檔案的 GitHub 儲存庫，其中 `owner` 是 GitHub 使用者名稱或組織，`repo` 是儲存庫名稱。

例如，`anthropics/claude-code` 指的是由 `anthropics` 擁有的 `claude-code` 儲存庫：

```shell theme={null}
/plugin marketplace add anthropics/claude-code
```

<h3 id="add-from-other-git-hosts">
  從其他 Git 主機新增
</h3>

透過提供完整 URL 新增任何 git 儲存庫。這適用於任何 Git 主機，包括 GitLab、Bitbucket 和自託管伺服器。包含 `.git` 後綴，以便 Claude Code 複製儲存庫，而不是將 URL 視為託管 `marketplace.json` 檔案的直接連結。

包含 `https://` 前綴。Claude Code v2.1.196 及更新版本會拒絕沒有前綴的主機，例如 `gitlab.com/company/plugins.git`，視其為無效的 GitHub `owner/repo` 簡寫，錯誤訊息會告訴您新增前綴。較早的版本會將其誤讀為 GitHub 儲存庫路徑，並在複製時失敗。

使用 HTTPS：

```shell theme={null}
/plugin marketplace add https://gitlab.com/company/plugins.git
```

使用 SSH：

```shell theme={null}
/plugin marketplace add git@gitlab.com:company/plugins.git
```

若要新增特定分支或標籤，請在 `#` 後面附加 ref：

```shell theme={null}
/plugin marketplace add https://gitlab.com/company/plugins.git#v1.0.0
```

<h3 id="add-from-local-paths">
  從本機路徑新增
</h3>

新增包含 `.claude-plugin/marketplace.json` 檔案的本機目錄：

```shell theme={null}
/plugin marketplace add ./my-marketplace
```

您也可以新增 `marketplace.json` 檔案的直接路徑：

```shell theme={null}
/plugin marketplace add ./path/to/marketplace.json
```

<h3 id="add-from-remote-urls">
  從遠端 URL 新增
</h3>

透過 URL 新增遠端 `marketplace.json` 檔案：

```shell theme={null}
/plugin marketplace add https://example.com/marketplace.json
```

<Note>
  與基於 Git 的市場相比，基於 URL 的市場有一些限制。如果在安裝外掛程式時遇到「找不到路徑」錯誤，請參閱[故障排除](/docs/zh-TW/plugin-marketplaces#plugins-with-relative-paths-fail-in-url-based-marketplaces)。
</Note>

<h2 id="install-plugins">
  安裝外掛程式
</h2>

新增市場後，您可以直接安裝外掛程式：

```shell theme={null}
/plugin install plugin-name@marketplace-name
```

該命令會開啟該外掛程式的詳細資訊，您可以在其中選擇[安裝範圍](/docs/zh-TW/settings#configuration-scopes)。當您執行 `/plugin`、前往 **Discover** 標籤，並在外掛程式上按 **Enter** 時，您會看到相同的選項：

* **User scope**（預設）：在所有專案中為自己安裝
* **Project scope**：為此儲存庫上的所有協作者安裝，這會將外掛程式新增到 `.claude/settings.json`
* **Local scope**：僅在此儲存庫中為自己安裝，不與協作者共享

若要在沒有互動式步驟的情況下安裝，請使用 [`claude plugin install`](/docs/zh-TW/plugins-reference#plugin-install) shell 命令，該命令預設安裝到使用者範圍，除非您傳遞 `--scope`。

您也可能看到具有 **managed** 範圍的外掛程式。這些是由管理員透過[受管設定](/docs/zh-TW/settings#settings-files)安裝的，無法修改。

<Warning>
  在安裝外掛程式之前，請確保您信任它。Anthropic 不控制外掛程式中包含的 MCP servers、檔案或其他軟體，也無法驗證它們是否按預期工作。檢查每個外掛程式的首頁以獲取更多資訊。
</Warning>

<h2 id="manage-installed-plugins">
  管理已安裝的外掛程式
</h2>

執行 `/plugin` 並前往 **Installed** 標籤以檢視、啟用、停用或解除安裝外掛程式。清單按範圍分組，並排序以便您首先看到問題：具有載入錯誤或未解決依賴項的外掛程式出現在頂部，然後是您的最愛，停用的外掛程式摺疊在底部的摺疊標題後面。

從清單中，您可以：

* 按 `f` 以將選定的外掛程式加入最愛或取消加入最愛
* 輸入以按外掛程式名稱或描述篩選
* 按 Enter 以開啟外掛程式的詳細檢視並啟用、停用或解除安裝它

解除安裝專案的 `.claude/settings.json` 啟用的外掛程式會詢問您指的是哪個範圍：僅為您停用它，這會將覆寫寫入您的 `.claude/settings.local.json` 並為專案保留已安裝的外掛程式，或為所有人解除安裝它，這會將其從共用的 `.claude/settings.json` 中移除。需要 Claude Code v2.1.203 或更新版本。在 v2.1.203 之前，對話框僅提供本機停用選項。

詳細檢視會顯示外掛程式貢獻的元件：commands、skills、agents、hooks、MCP servers 和 LSP servers。相同的清單也可從命令列透過 `claude plugin details` 取得。

**Installed** 標籤也會收集您自己安裝但至少兩週內未使用且跨越至少 10 個工作階段的 marketplace 外掛程式，位於 **Not used recently** 標題下。詳細檢視會為每個外掛程式顯示 **Last used** 行。使用這些功能來找出您不再使用但仍在增加啟動和內容成本的外掛程式，然後停用或解除安裝它們。需要 Claude Code v2.1.187 或更新版本。

兩種外掛程式永遠不會列為未使用：

* 您的組織管理的外掛程式或您使用 `--plugin-dir` 載入的外掛程式
* 貢獻 theme、output style、monitor 或 workflow 的外掛程式，因為這些外掛程式提供價值而無需追蹤叫用

當您的組織使用 [`strictKnownMarketplaces`](/docs/zh-TW/settings#strictknownmarketplaces) 限制 marketplaces 時，**Not used recently** 標題和 **Last used** 行都會隱藏。

外掛程式的 [language server](/docs/zh-TW/plugins#add-lsp-servers-to-your-plugin) 在提供診斷或回答程式碼導覽請求時計為已使用，因此其伺服器在您的工作階段中處於活動狀態的 LSP 外掛程式不會列為未使用。在 v2.1.203 之前，無法將語言伺服器活動計為使用，因此貢獻 LSP 伺服器的外掛程式完全豁免於該群組，與 theme 和 output style 外掛程式的方式相同。

計算語言伺服器活動的版本上的第一個工作階段也會重設每個尚未記錄任何使用的 LSP 外掛程式的使用記錄，因此 Claude Code 不會根據在其伺服器活動被追蹤之前記錄的資料將您較早安裝的外掛程式判斷為未使用。在 v2.1.206 之前，該第一個工作階段可能會在 **Not used recently** 下列出主動使用的 LSP 外掛程式並建議檢查它。

當您安裝聲明依賴項的外掛程式時，安裝輸出會列出哪些依賴項與其一起自動安裝。

您也可以使用直接命令管理外掛程式。

列出已安裝的外掛程式而不開啟選單：

```shell theme={null}
/plugin list
```

傳遞 `--enabled` 或 `--disabled` 以僅顯示處於該狀態的外掛程式。

停用外掛程式而不解除安裝：

```shell theme={null}
/plugin disable plugin-name@marketplace-name
```

重新啟用已停用的外掛程式：

```shell theme={null}
/plugin enable plugin-name@marketplace-name
```

在這些識別碼中，`plugin-name` 是 [marketplace 項目](/docs/zh-TW/plugin-marketplaces#plugin-entries) 中外掛程式的 `name`，可能與外掛程式自身 `plugin.json` 中的 `name` 不同。

自 Claude Code v2.1.195 起，`/plugin` 介面中的 **Enable** 和 **Disable** 適用於其兩個名稱不同的外掛程式，`/plugin enable` 和 `/plugin disable` 接受任一名稱。當您在較早版本中停用此類外掛程式時，Claude Code 會報告 `already disabled` 並保持其啟用狀態。

完全移除外掛程式：

```shell theme={null}
/plugin uninstall plugin-name@marketplace-name
```

`--scope` 選項可讓您使用 CLI 命令針對特定範圍：

```shell theme={null}
claude plugin install formatter@your-org --scope project
claude plugin uninstall formatter@your-org --scope project
```

<h3 id="apply-plugin-changes-without-restarting">
  在不重新啟動的情況下套用外掛程式變更
</h3>

當您在工作階段期間安裝、啟用或停用外掛程式時，執行 `/reload-plugins` 以在不重新啟動的情況下啟動所有變更：

```shell theme={null}
/reload-plugins
```

Claude Code 重新載入所有活動外掛程式，並顯示外掛程式、skills、agents、hooks、外掛程式 MCP servers 和外掛程式 LSP servers 的計數。

重新載入在下一個請求時會產生令牌成本：新載入的元件會在附加到對話的內容中宣佈自己，而現有歷史記錄仍然從提示快取讀取。提供 MCP servers 的外掛程式在其工具未被 [tool search](/docs/zh-TW/mcp#scale-with-mcp-tool-search) 延遲時成本更高：該變更會使快取失效，下一個請求會重新讀取整個對話。在該情況下 `/reload-plugins` 會顯示警告並不套用重新載入；傳遞 `--force` 以強制套用。如需詳細資訊，請參閱 [啟用或停用外掛程式](/docs/zh-TW/prompt-caching#enabling-or-disabling-a-plugin)。

<h2 id="manage-marketplaces">
  管理市場
</h2>

您可以透過互動式 `/plugin` 介面或使用 CLI 命令管理市場。

<h3 id="use-the-interactive-interface">
  使用互動式介面
</h3>

執行 `/plugin` 並前往 **Marketplaces** 標籤以：

* 檢視所有已新增的市場及其來源和狀態
* 新增新市場
* 更新市場清單以取得最新外掛程式
* 移除您不再需要的市場

<h3 id="use-cli-commands">
  使用 CLI 命令
</h3>

您也可以使用直接命令管理市場。

列出所有已配置的市場：

```shell theme={null}
/plugin marketplace list
```

從市場重新整理外掛程式清單：

```shell theme={null}
/plugin marketplace update marketplace-name
```

移除市場：

```shell theme={null}
/plugin marketplace remove marketplace-name
```

<Warning>
  移除市場將解除安裝您從中安裝的任何外掛程式。
</Warning>

<h3 id="configure-auto-updates">
  配置自動更新
</h3>

Claude Code 可以在啟動後自動在背景更新市場及其已安裝的外掛程式。為市場啟用自動更新後，Claude Code 會重新整理市場資料並將已安裝的外掛程式更新到其磁碟上的最新版本。

Claude Code 會在您的工作階段開始後檢查市場和外掛程式更新，並隨機延遲最多十分鐘，因此執行中的工作階段會繼續使用它在啟動時載入的版本。如果任何外掛程式已更新，您將看到提示您執行 `/reload-plugins` 的通知，或新版本會在您下次啟動時載入。

透過 UI 為個別市場切換自動更新：

1. 執行 `/plugin` 以開啟外掛程式管理器
2. 選擇 **Marketplaces**
3. 從清單中選擇市場
4. 選擇 **Enable auto-update** 或 **Disable auto-update**

官方 Anthropic 市場預設啟用自動更新。第三方和本機開發市場預設停用自動更新。

管理員也可以在受管設定中的每個 [`extraKnownMarketplaces`](/docs/zh-TW/settings#extraknownmarketplaces) 項目上設定 `"autoUpdate": true`，以為組織市場啟用自動更新，而無需每個使用者都切換它。

若要完全停用 Claude Code 和所有外掛程式的所有自動更新，請設定 `DISABLE_AUTOUPDATER` 環境變數。有關詳細資訊，請參閱[自動更新](/docs/zh-TW/setup#auto-updates)。

若要在停用 Claude Code 自動更新的同時保持外掛程式自動更新啟用，請設定 `FORCE_AUTOUPDATE_PLUGINS=1` 以及 `DISABLE_AUTOUPDATER`：

```bash theme={null}
export DISABLE_AUTOUPDATER=1
export FORCE_AUTOUPDATE_PLUGINS=1
```

當您想要手動管理 Claude Code 更新但仍然接收自動外掛程式更新時，這很有用。

<h2 id="configure-team-marketplaces">
  配置團隊市場
</h2>

團隊管理員可以透過將市場配置新增到 `.claude/settings.json` 來為專案設定自動市場安裝。當團隊成員信任儲存庫資料夾時，Claude Code 會提示他們安裝這些市場和外掛程式。

自 Claude Code v2.1.195 起，此安裝步驟適用於載入外掛程式的每個路徑。只有專案的 `.claude/settings.json` 啟用的外掛程式，且來自外部來源（例如 GitHub 儲存庫或 npm 套件），在團隊成員安裝之前不會載入。在此之前，Claude Code 會將外掛程式報告為未安裝，並顯示要執行的 `claude plugin install` 命令。

將 `extraKnownMarketplaces` 新增到您的專案的 `.claude/settings.json`：

```json theme={null}
{
  "extraKnownMarketplaces": {
    "my-team-tools": {
      "source": {
        "source": "github",
        "repo": "your-org/claude-plugins"
      }
    }
  }
}
```

如需完整配置選項（包括 `extraKnownMarketplaces` 和 `enabledPlugins`），請參閱[外掛程式設定](/docs/zh-TW/settings#plugin-settings)。

<h2 id="security">
  安全性
</h2>

外掛程式和市場是高度受信任的元件，可以使用您的使用者權限在您的機器上執行任意程式碼。僅從您信任的來源安裝外掛程式和新增市場。組織可以使用[受管市場限制](/docs/zh-TW/plugin-marketplaces#managed-marketplace-restrictions)限制使用者可以新增的市場。

<h2 id="troubleshooting">
  故障排除
</h2>

<h3 id="/plugin-command-not-recognized">
  /plugin 命令無法識別
</h3>

如果您看到「未知命令」或 `/plugin` 命令未出現：

1. **檢查您的版本**：執行 `claude --version` 以查看已安裝的內容。
2. **更新 Claude Code**：
   * **Homebrew**：`brew upgrade claude-code`，或如果您安裝了該 cask，執行 `brew upgrade claude-code@latest`
   * **npm**：`npm install -g @anthropic-ai/claude-code@latest`
   * **原生安裝程式**：從[設定](/docs/zh-TW/setup)重新執行安裝命令
3. **重新啟動 Claude Code**：更新後，重新啟動您的終端機並再次執行 `claude`。

<h3 id="common-issues">
  常見問題
</h3>

* **市場未載入**：驗證 URL 是否可存取以及 `.claude-plugin/marketplace.json` 是否存在於路徑中
* **外掛程式安裝失敗**：檢查外掛程式來源 URL 是否可存取以及儲存庫是否為公開，或您是否有存取權
* **安裝後找不到檔案**：外掛程式被複製到快取中，因此參考外掛程式目錄外檔案的路徑將無法運作
* **外掛程式技能未出現**：使用 `rm -rf ~/.claude/plugins/cache` 清除快取，重新啟動 Claude Code，然後重新安裝外掛程式。

如需詳細的故障排除和解決方案，請參閱市場指南中的[故障排除](/docs/zh-TW/plugin-marketplaces#troubleshooting)。如需偵錯工具，請參閱[偵錯和開發工具](/docs/zh-TW/plugins-reference#debugging-and-development-tools)。

<h3 id="code-intelligence-issues">
  程式碼智能問題
</h3>

* **語言伺服器未啟動**：驗證二進位檔已安裝且在您的 `$PATH` 中可用。檢查 `/plugin` Errors 標籤以獲取詳細資訊。
* **高記憶體使用量**：`rust-analyzer` 和 `pyright` 等語言伺服器在大型專案上可能會消耗大量記憶體。如果您遇到記憶體問題，請使用 `/plugin disable <plugin-name>` 停用外掛程式，並改為依賴 Claude 的內建搜尋工具。
* **monorepos 中的誤報診斷**：如果工作區配置不正確，語言伺服器可能會報告內部套件的未解決匯入錯誤。這些不會影響 Claude 編輯程式碼的能力。

<h2 id="next-steps">
  後續步驟
</h2>

* **構建您自己的外掛程式**：請參閱[外掛程式](/docs/zh-TW/plugins)以建立技能、代理和 hooks
* **建立市場**：請參閱[建立外掛程式市場](/docs/zh-TW/plugin-marketplaces)以將外掛程式分發給您的團隊或社群
* **技術參考**：請參閱[外掛程式參考](/docs/zh-TW/plugins-reference)以取得完整規格
